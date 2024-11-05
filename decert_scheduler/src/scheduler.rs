use std::collections::{HashMap, HashSet};
use crate::models::Task;

pub struct DictBitMask {
    pub sensors: HashMap<String, usize>, // Map of sensor name to its index in the mask
    pub mask: u64,                       // 64-bit integer to represent the bitmask
}

impl DictBitMask {
    pub fn new(sensors: Vec<String>) -> Self {
        let sensor_map = sensors
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect();
        DictBitMask {
            sensors: sensor_map,
            mask: 0,
        }
    }

    pub fn conflict(&self, other: &DictBitMask) -> bool {
        self.mask & other.mask != 0
    }

    pub fn clone(&self) -> Self {
        DictBitMask {
            sensors: self.sensors.clone(),
            mask: self.mask,
        }
    }

    pub fn set(&mut self, sensor: &str, value: bool) {
        if let Some(&sensor_index) = self.sensors.get(sensor) {
            if value {
                self.mask |= 1 << sensor_index;
            } else {
                self.mask &= !(1 << sensor_index);
            }
        } else {
            panic!("Sensor {} not found", sensor);
        }
    }

    pub fn get(&self, sensor: &str) -> bool {
        if let Some(&sensor_index) = self.sensors.get(sensor) {
            (self.mask & (1 << sensor_index)) != 0
        } else {
            panic!("Sensor {} not found", sensor);
        }
    }

    pub fn to_string(&self) -> String {
        let dict_of_set: HashMap<String, bool> = self
            .sensors
            .keys()
            .map(|k| (k.clone(), self.get(k)))
            .collect();
        format!("{:?}", dict_of_set)
    }
}

// Check if all sensors required by the task are available in the DictBitMask
pub fn is_sensor_available(task: &Task, sbm: &DictBitMask) -> bool {
    task.args.iter().all(|sensor| !sbm.get(&sensor))
}

// Set or clear sensors in the DictBitMask according to the task’s requirements
pub fn set_sensors(task: &Task, sbm: &mut DictBitMask, value: bool) {
    for sensor in &task.args {
        sbm.set(&sensor, value);
    }
}

pub fn schedule(
    sbm: &DictBitMask,
    cpus: &DictBitMask,
    sensor_tasks: &HashMap<String, Vec<Task>>,
) -> Vec<Task> {
    let mut current_sbm = sbm.clone();
    let mut tasks: HashSet<Task> = HashSet::new();

    for _ in 0..sensor_tasks.len() {
        for (sensor, task_list) in sensor_tasks {
            if current_sbm.get(sensor) {
                continue;
            }

            // Find the first available task that can use the sensor
            let first_available_task = task_list
                .iter()
                .position(|task| is_sensor_available(task, &current_sbm));

            // If no available task was found, continue to the next sensor
            if let Some(first_task_index) = first_available_task {
                // Remove conflicting tasks and clear their sensors
                let mut to_remove = Vec::new();
                for task in &tasks {
                    if task.args.iter().any(|s| *s == (*sensor.clone()).into()) {
                        if &task_list[first_task_index] != task {
                            to_remove.push(task.clone());
                            set_sensors(task, &mut current_sbm, false);
                        }
                    }
                }
                for t in to_remove {
                    tasks.remove(&t);
                }

                // Add the new task and mark the sensor as used
                let task = &task_list[first_task_index];
                tasks.insert(task.clone());
                current_sbm.set(sensor, true);
            }
        }
    }

    tasks.into_iter().collect()
}
