use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
    path::PathBuf,
    sync::Arc,
};

use codewriter::{CodeTask, CodeWriter, FunctionCall};
use confparse::{Conf, Task};
use cpu::{get_next_tasks, CPU};
use scheduler::{task_schedule, BitMap};
use serde::Deserialize;
mod codewriter;
mod cpu;
mod scheduler;

#[derive(Deserialize, Clone)]
pub struct Sensors {
    pub name: Arc<str>,
    pub from: Arc<str>,
    pub to: Arc<str>,
}

#[derive(Deserialize)]
pub struct SensorJson {
    pub sensors: Vec<Sensors>,
    pub ports: Vec<String>,
}

fn read_sensors() -> Result<SensorJson, String> {
    let data = read_to_string("./sensors.json").map_err(|e| e.to_string())?;
    let sensorjson: SensorJson = serde_json::from_str(&data)
        .map_err(|e| e.to_string())
        .map_err(|e| e.to_string())?;
    Ok(sensorjson)
}

pub fn schedule(topology: &HashMap<u32, Conf>) -> Result<SensorJson, String> {
    let sensorjson = read_sensors()?;
    let sensors = sensorjson.sensors.clone();
    let mut cpus: HashMap<u32, CPU> = topology
        .iter()
        .map(|(cpu_id, conf)| {
            (
                *cpu_id,
                CPU::new(*cpu_id, conf.tasks.clone(), conf.initial.clone()),
            )
        })
        .collect();

    let sensors_to_int: HashMap<Arc<str>, u8> = sensors
        .iter()
        .enumerate()
        .map(|(loc, sensor)| (sensor.name.clone(), loc as u8))
        .collect(); // gives a map from sensor name to its location in sensors vector

    let mut sensor_bitmap = BitMap::new(); // sensor bit map

    let mut cpu_codewriter: HashMap<u32, CodeWriter> = cpus
        .iter()
        .map(|(id, _)| (*id, CodeWriter::new()))
        .collect(); // codewriter for each cpu

    // let mut task2cpus: HashMap<Task, u32> = HashMap::new(); // task -> cpu_id
    let mut scheduled_tasks: BinaryHeap<(i32, Task)> = BinaryHeap::new(); // currently scheduled tasks
    let mut next_tasks: Vec<(Task, u8)>; // stores the next set of tasks to be scheduled
    let mut pending_tasks: HashMap<Task, u8> = HashMap::new(); // stores the waits of tasks which failed to get scheduled
    let mut unutilized_cpus: HashSet<u32> = cpus.keys().fold(HashSet::new(), |mut acc, x| {
        acc.insert(*x);
        acc
    }); // all cpus which are unutilized for this cycle of scheduling
    let mut time = 0;

    loop {
        println!("Cycle {}", time);
        loop {
            let mut next_tasks_with_runnable_tasks_left: Vec<_> = get_next_tasks(&unutilized_cpus, &mut cpus)
                .into_iter()
                .filter_map(|(cpu_id, task)| {
                    let Some((task, runnable_tasks_left)) = task else {
                        return None;
                    };
                    if pending_tasks.contains_key(&task) {
                        Some((runnable_tasks_left, (task.clone(), pending_tasks[&task] as u8)))
                    } else {
                        // initially set task-wt to 1
                        Some((runnable_tasks_left, (task, 1 as u8)))
                    }
                })
                .collect();
            next_tasks_with_runnable_tasks_left.sort();

            next_tasks = next_tasks_with_runnable_tasks_left.into_iter().map(|(_, x)| x).collect();
            println!("next_tasks: {:?}", next_tasks);

            if next_tasks.is_empty() {
                // reset and continue
                cpus.iter_mut().for_each(|(id, cpu)| {
                    cpu.reset();
                });
                unutilized_cpus.iter().for_each(|id| {
                    let Some(codewriter) = cpu_codewriter.get_mut(id) else {
                        return;
                    };
                    codewriter.start_delay(time);
                });
                break;
            }

            // pushed newly scheduled tasks into scheduled tasks
            // println!("sensors_to_int: {:?}", sensors_to_int);
            let task_currently_scheduled =
                task_schedule(&next_tasks, &sensors_to_int, sensor_bitmap);
            // println!("task_currently_scheduled: {:?}", task_currently_scheduled);
            for task in &task_currently_scheduled {
                task.args
                    .iter()
                    .for_each(|sensor| sensor_bitmap.set(sensors_to_int[sensor], true));
                // remove the cpu of these tasks from unutilized
                unutilized_cpus.remove(&task.obc_id);
                scheduled_tasks.push((-(task.cycles as i32), task.clone()));
            }

            // append the rest of the tasks to pending tasks
            next_tasks.iter().for_each(|(task, weight)| {
                if !task_currently_scheduled.contains(&(task.clone())) {
                    pending_tasks.insert(task.clone(), weight + 1);
                }
            });
            println!("task_currently_scheduled: {:?}", task_currently_scheduled);
            println!("unutilized_cpus: {:?}", unutilized_cpus);
        }
        // if empty then schduling completed
        if scheduled_tasks.is_empty() {
            break;
        }

        let mut curr_cycle = 0;
        // set requirements satisfied
        while let Some(task_entry) = scheduled_tasks.pop() {
            println!("task_entry: {:?}", task_entry);
            let task_cpu = cpus
                .get_mut(&task_entry.1.obc_id)
                .expect("Did not find CPU for id. Impossible!");
            let curr_task = task_entry.1;
            println!("curr_task: {:?}", curr_task);
            // task2cpus.remove(&curr_task); // freeing space
            unutilized_cpus.insert(task_cpu.id); // added this cpu to unutilized

            // println!("unutilized_cpus: {:?}", unutilized_cpus);
            curr_cycle = curr_task.cycles;

            // free up the sensors
            curr_task
                .args
                .iter()
                .for_each(|sensor| sensor_bitmap.set(sensors_to_int[sensor], false));
            task_cpu.task_complete(&curr_task);
            task_cpu.reset();
            let Some(codewriter) = cpu_codewriter.get_mut(&task_cpu.id) else {
                Err("Did not found the codewriter for this cpu")?
            };
            // this function written to the code writer
            codewriter.append(
                CodeTask::FunctionCall(FunctionCall {
                    fn_identifier: curr_task.name.clone(),
                    cycles: curr_task.cycles,
                    args: curr_task.args.clone(),
                }),
                time.into(),
            );

            // break no more tasks are finishing at this timestamp
            if scheduled_tasks.peek().is_some() {
                if scheduled_tasks.peek().unwrap().1.cycles != curr_cycle {
                    break;
                }
            }
        }
        time += curr_cycle as i32;
    }

    for (id, cpu_cw) in cpu_codewriter.iter_mut() {
        cpu_cw.commit(PathBuf::from(format!("./obc{id}")), time)?;
    }
    Ok(sensorjson)
}
