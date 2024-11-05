use std::{collections::{BinaryHeap, HashMap}, sync::Arc};

use confparse::{Conf, Task};
use cpu::{get_next_tasks, CPU};
use scheduler::{task_schedule, BitMap};
mod cpu;
mod codewriter;
mod scheduler;

struct Sensors {
    name: String
}


fn get_sensors()-> Vec<Sensors>{
    /* reads sensor.json and creates a Vector of Sensors */
    todo!()
}

pub fn schedule(topology:HashMap<u32,Conf>) -> Result<(), String> {

    let cpus: HashMap<u32, CPU> = topology.iter().map(|entry|
    {
        (*entry.0,CPU::new(*entry.0, entry.1.tasks.clone()))
    }
    ).collect();

    let sensors = get_sensors();


    let sensors_to_int: HashMap<Arc<str>, u8> = sensors.iter().enumerate().map(|(loc, sensor)| {
        (sensor.name.into(), loc as u8)
    } ).collect(); // gives a map from sensor name to its location in sensors vector
    let sensor_bitmap = BitMap{
        
    };
    let mut task2cpus: HashMap<Task,u32> = HashMap::new(); // task -> cpu_id
    let mut scheduled_tasks:BinaryHeap<(i32,Task)> = BinaryHeap::new(); // currently scheduled tasks
    let mut next_tasks: Vec<(Task, u8)>; // stores the next set of tasks to be scheduled
    let mut pending_tasks: HashMap<Task, u8> = HashMap::new(); // stores the waits of tasks which failed to get scheduled
    let mut unutilized_cpus: HashMap<u32, CPU> = cpus.clone(); // all cpus which are unutilized for this cycle of scheduling

    loop {
        next_tasks = get_next_tasks(unutilized_cpus.clone()).iter().filter_map(|entry|
        {
            entry.1.as_ref().map(|t| {
                task2cpus.insert(t.clone(), *entry.0);
                if pending_tasks.contains_key(t) {
                    (t.clone(), pending_tasks[t] as u8)
                } else {
                    (t.clone(), 1 as u8)
                }
            })
        }).collect();

        
        task_schedule(&next_tasks, sensors_to_int, sensors_used);
        

        // if empty then schduling completed
        if scheduled_tasks.is_empty() {
            break;
        }

        // set requirements satisfied
        let Some(task_entry) = scheduled_tasks.pop() else {
            Err("Something bad occured ")?
        };
        let mut task_cpu = cpus[&task2cpus[&task_entry.1]].clone();
        task2cpus.remove(&task_entry.1); // freeing space
        task_cpu.task_complete(task_entry.1);

    };
    Ok(())
}