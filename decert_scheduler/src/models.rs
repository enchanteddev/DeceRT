use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Sensor {
    pub sensor_name: String,
    pub from_address: String,
    pub to_address: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task (confparse::Task);


impl Deref for Task {
    type Target = confparse::Task;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Task {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl Hash for Task {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}

impl Task {
    pub fn new(task: confparse::Task) -> Task {
        Task(task)
    }
}

#[derive(Deserialize, Serialize)]
struct SensorJson {
    sensors: Vec<Sensor>,
    ports: Vec<String>,
}

pub fn load_sensors<P: AsRef<Path>>(path: P) -> HashMap<String, Arc<Sensor>> {
    let data = fs::read_to_string(path).expect("Failed to read sensors.json");
    let sensorjson: SensorJson = serde_json::from_str(&data).expect("Invalid JSON format");
    let sensor_list = sensorjson.sensors;
    sensor_list
        .into_iter()
        .map(|sensor| (sensor.sensor_name.clone(), Arc::new(sensor)))
        .collect()
}
