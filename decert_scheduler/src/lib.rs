use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use codewriter::CodeWriter;
use serde_json::from_str;
use crate::models::load_sensors;
use crate::models::Task;
use crate::scheduler::{DictBitMask, schedule};
use crate::cpu::{CPU, get_next_tasks};
use crate::codewriter::FunctionCall;
mod scheduler;
mod models;
mod cpu;
mod codewriter;


fn main() {
    // Load topology from JSON file
    let topology_json = std::fs::read_to_string("topology.json").expect("Failed to read topology.json");
    let topology: Topology = from_str(&topology_json).expect("Failed to parse topology");
    
    let sensor_path = "";

    // Initialize DictBitMask for sensors and CPUs
    let sensors = load_sensors(sensor_path);
    let sensor_keys: Vec<String> = sensors.keys().cloned().collect();
    let mut sbm = DictBitMask::new(sensor_keys);
    let cpu_keys: Vec<String> = topology.cpus.iter().map(|cpu| cpu.cpu_id.clone()).collect();
    let mut cpus_bit_mask = DictBitMask::new(cpu_keys.clone());

    // Create CPU instances
    let mut cpus: HashMap<String, CPU> = topology
        .cpus
        .iter()
        .map(|cpu| (cpu.cpu_id.clone(), CPU::new(cpu.cpu_id.clone(), cpu.tasks.clone())))
        .collect();

    // Initialize sensor_tasks map
    let mut sensor_tasks: HashMap<String, Vec<Task>> = sensors
        .keys()
        .map(|s| (s.clone(), Vec::new()))
        .collect();

    // Initial scheduling
    let mut next_tasks = get_next_tasks(&cpus, &cpus_bit_mask, &sbm);
    let mut cpu_scheduled_tasks: HashMap<String, Option<Task>> = HashMap::new();
    let mut output = Vec::new();

    while !next_tasks.is_empty() {
        let mut new_task_added = false;
        let task2cpu: HashMap<String, String> = next_tasks
            .iter()
            .filter_map(|(cpuname, task)| 
            task.as_ref().map(|t| (t.fn_identifier.clone(), cpuname.clone().to_string())))
            .collect();

        for (cpuname, task) in &next_tasks {
            if task.is_none() {
                continue;
            }
            new_task_added = true;
            for sensor in &task.as_ref().unwrap().args {
                sensor_tasks
                    .get_mut(&sensor.sensor_name)
                    .expect("Sensor not found")
                    .push(task.clone().unwrap());
            }
        }

        if !new_task_added {
            break;
        }

        output = schedule(&sbm, &cpus_bit_mask, &sensor_tasks);
        for task in &output {
            if let Some(cpuname) = task2cpu.get(&task.fn_identifier) {
                cpu_scheduled_tasks.insert(cpuname.clone(), Some(task.clone()));
                cpus_bit_mask.set(cpuname, true);
            }
        }
        next_tasks = get_next_tasks(&cpus, &cpus_bit_mask, &sbm);
    }

    // CodeWriter and time tracking
    let mut cpu_cw: HashMap<String, CodeWriter> = cpu_keys
        .iter()
        .map(|cpu_id| (cpu_id.clone(), CodeWriter::new(format!("CPU{}", cpu_id))))
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
                fn_identifier: task.as_ref().unwrap().fn_identifier.clone(),
                call_time_ms: task.as_ref().unwrap().call_time_ms,
                args: task.as_ref().unwrap().args.iter().map(|a| a.sensor_name.clone()).collect(),
            };
            cpu_writer.append(function_call, time);
        }
    }

    // Commit all CPU writers
    for (_, cpu_writer) in &cpu_cw {
        cpu_writer.commit(PathBuf::from("."));
    }
}
