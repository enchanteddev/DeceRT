use crate::models::Task;
use crate::scheduler::DictBitMask;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

pub struct CPU {
    pub id: i32,
    pub tasks: Vec<Task>,
    pub current_tasks: Vec<Task>,
    pub satisfied: HashMap<String, bool>,
}

impl CPU {
    pub fn new(id: i32, tasks: Vec<Task>) -> Self {
        let mut cpu = CPU {
            id,
            tasks,
            current_tasks: Vec::new(),
            satisfied: HashMap::new(),
        };
        cpu.update_task_list();
        cpu
    }

    pub fn update_task_list(&mut self) {
        /*
           Updates the currentTasks according to the satisfied tasks
           called in init and when updating the requirements in requirement_satisfied function
        */
        let mut to_remove = Vec::new();

        for task in &self.tasks {
            if task.requires.as_ref().map_or(true, |req| {
                req.iter().all(|r| *self.satisfied.get(r).unwrap_or(&false))
            }) {
                self.current_tasks.push(task.clone());
                to_remove.push(task.clone());
            }
        }

        self.tasks.retain(|task| !to_remove.contains(task));
    }

    pub fn get_next_first_task(&mut self, sensor_bit_mask: &DictBitMask) -> Option<Task> {
        /*
           Checks the sensor_bit_mask and returns the next task which could be executed
           on basis current statisfied requirements and free sensor.
           return Null if can't find any
           time complexity -> O()
        */

        let mut to_remove = Vec::new();
        let mut to_return: Option<Task> = None;

        for task in &self.current_tasks {
            if task
                .args
                .iter()
                .all(|sensor| !sensor_bit_mask.get(&sensor.sensor_name))
            {
                to_remove.push(task.clone());
                to_return = Some(task.clone());
                break;
            }
        }

        self.current_tasks.retain(|task| !to_remove.contains(task));
        to_return
    }

    pub fn requirement_satisfied(&mut self, req: &str) {
        /*
        scheduler sends message that this requirement has been satisfied
         */
        self.satisfied.insert(req.to_string(), true);
        self.update_task_list();
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nCPUID : {}\nsatisfied requirements : {:?}\nremaining tasks: \n{}\ntasks in queue: \n{}",
            self.id,
            self.satisfied,
            self.tasks.iter().map(|task| format!("{:?}", task)).collect::<Vec<_>>().join("\n"),
            self.current_tasks.iter().map(|task| format!("{:?}", task)).collect::<Vec<_>>().join("\n"))
    }
}

pub fn get_next_tasks(
    cpus: &mut HashMap<String, Arc<RefCell<CPU>>>,
    cpus_bit_mask: &DictBitMask,
    sensor_bit_mask: &DictBitMask,
) -> HashMap<i32, Option<Task>> {
    let mut unutilized_cpus = HashMap::new();

    for (cpuname, _) in &cpus_bit_mask.sensors {
        let is_in_use = cpus_bit_mask.get(cpuname);
        if is_in_use {
            continue;
        }

        if let Some(cpu) = cpus.get(cpuname) {
            let next_task = cpu.borrow_mut().get_next_first_task(sensor_bit_mask);
            unutilized_cpus.insert(cpu.borrow().id, next_task);
        }
    }

    unutilized_cpus
}
