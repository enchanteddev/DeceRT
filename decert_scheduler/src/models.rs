use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Sensor {
    pub sensor_type: String,
    pub sensor_name: String,
}

impl Sensor {
    fn from_name(name: &str, sensors_map: &HashMap<String, Sensor> ) -> Sensor {
        sensors_map.get(name).expect("Sensor not found").clone()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Task {
    pub fn_identifier: String,
    pub call_time_ms: u64,
    pub args: Vec<Sensor>,
    pub satisfies: Option<Vec<String>>,
    pub requires: Option<Vec<String>>,
}

impl Hash for Task {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fn_identifier.hash(state);
    }
}

impl Task {
    fn new(
        fn_identifier: String,
        call_time_ms: u64,
        args: Vec<String>,
        satisfies: Option<Vec<String>>,
        requires: Option<Vec<String>>,
        sensors_map: &HashMap<String, Sensor>,
    ) -> Self {
        let resolved_args = args
            .into_iter()
            .map(|name| Sensor::from_name(&name, sensors_map))
            .collect();
        Task {
            fn_identifier,
            call_time_ms,
            args: resolved_args,
            satisfies,
            requires,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OBCInfo {
    pub cpu_id: u32,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Topology {
    pub cpus: Vec<OBCInfo>,
}

pub fn load_sensors<P: AsRef<Path>>(path: P) -> HashMap<String, Arc<Sensor>> {
    let data = fs::read_to_string(path).expect("Failed to read sensors.json");
    let sensor_list: Vec<Sensor> = serde_json::from_str(&data).expect("Invalid JSON format");
    sensor_list
        .into_iter()
        .map(|sensor| (sensor.sensor_name.clone(), Arc::new(sensor)))
        .collect()
}
