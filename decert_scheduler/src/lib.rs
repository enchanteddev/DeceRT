use crate::codewriter::FunctionCall;
use crate::cpu::{get_next_tasks, CPU};
use crate::models::load_sensors;
use crate::models::Task;
use crate::scheduler::{schedule, DictBitMask};
use codewriter::CodeWriter;
use confparse::Conf;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
mod codewriter;
mod cpu;
mod models;
mod scheduler;

pub fn scheduler(topology: HashMap<u32, Conf>) -> Result<(), String> {
    let sensor_path = "";

    // Initialize DictBitMask for sensors and CPUs
    let sensors = load_sensors(sensor_path);
    let sensor_keys: Vec<String> = sensors.keys().cloned().collect();
    let sbm = DictBitMask::new(sensor_keys);
    let cpu_keys: Vec<String> = topology.keys().map(|id| format!("CPU{}", id)).collect();
    let mut cpus_bit_mask = DictBitMask::new(cpu_keys.clone());

    // Create CPU instances
    let mut cpus: HashMap<String, Arc<RefCell<CPU>>> = topology
        .iter()
        .map(|(cpuid, conf)| {
            (
                format!("CPU{}", cpuid),
                Arc::new(RefCell::new(CPU::new(
                    *cpuid as i32,
                    conf.tasks.iter().map(|t| Task::new(t.clone())).collect(),
                ))),
            )
        })
        .collect();

    // Initialize sensor_tasks map
    let mut sensor_tasks: HashMap<String, Vec<Task>> =
        sensors.keys().map(|s| (s.clone(), Vec::new())).collect();

    // Initial scheduling
    let mut next_tasks = get_next_tasks(&mut cpus, &cpus_bit_mask, &sbm);
    let mut cpu_scheduled_tasks: HashMap<String, Option<Task>> = HashMap::new();

    while !next_tasks.is_empty() {
        let mut new_task_added = false;
        let task2cpu: HashMap<Arc<str>, String> = next_tasks
            .iter()
            .filter_map(|(cpuname, task)| {
                task.as_ref()
                    .map(|t| (t.name.clone(), cpuname.clone().to_string()))
            })
            .collect();

        for (_, task) in &next_tasks {
            if task.is_none() {
                continue;
            }
            new_task_added = true;
            for sensor in &task.as_ref().unwrap().args {
                sensor_tasks
                    .get_mut(&sensor.to_string())
                    .expect("Sensor not found")
                    .push(task.clone().unwrap());
            }
        }

        if !new_task_added {
            break;
        }

        let output = schedule(&sbm, &cpus_bit_mask, &sensor_tasks);
        for task in &output {
            if let Some(cpuname) = task2cpu.get(&task.name) {
                cpu_scheduled_tasks.insert(cpuname.clone(), Some(task.clone()));
                cpus_bit_mask.set(cpuname, true);
            }
        }
        next_tasks = get_next_tasks(&mut cpus, &cpus_bit_mask, &sbm);
    }

    // CodeWriter and time tracking
    let mut cpu_cw: HashMap<String, CodeWriter> = cpu_keys
        .iter()
        .map(|cpu_id| (cpu_id.clone(), CodeWriter::new()))
        .collect();

    let time = 0;
    println!("{:?}", cpu_scheduled_tasks);
    for cpuname in cpus.keys() {
        let task = cpu_scheduled_tasks.get(cpuname).unwrap_or(&None);
        let cpu_writer = cpu_cw.get_mut(cpuname).unwrap();
        if task.is_none() {
            cpu_writer.start_delay(time);
        } else {
            let function_call = FunctionCall {
                fn_identifier: task.as_ref().unwrap().name.clone(),
                cycles: task.as_ref().unwrap().cycles,
                args: task
                    .as_ref()
                    .unwrap()
                    .args
                    .iter()
                    .map(|a| a.clone())
                    .collect(),
            };
            cpu_writer.append(codewriter::CodeTask::FunctionCall(function_call), time);
        }
    }

    // Commit all CPU writers
    for (_, cpu_writer) in cpu_cw.iter_mut() {
        cpu_writer.commit(PathBuf::from("."))?;
    }
    Ok(())
}
