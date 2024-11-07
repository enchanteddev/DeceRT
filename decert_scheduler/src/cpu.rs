use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    sync::Arc,
};

use confparse::Task;

#[derive(Clone, Debug)]
pub struct CPU {
    pub id: u32,
    tasks: Vec<Task>,
    runnable_tasks: BinaryHeap<(i32, Task)>,
    completed_tasks: HashSet<Task>,
    satisfied: HashSet<Arc<str>>,
}

impl CPU {
    pub fn new(id: u32, tasks: Vec<Task>) -> Self {
        let mut runnable_tasks = BinaryHeap::new();
        for task in &tasks {
            if task.requires.is_empty() {
                runnable_tasks.push((-(task.cycles as i32), task.clone()));
            }
        }
        CPU {
            id,
            tasks,
            runnable_tasks,
            completed_tasks: HashSet::new(),
            satisfied: HashSet::new(),
        }
    }

    pub fn get_task(&mut self) -> Option<Task> {
        self.runnable_tasks.pop().map(|(_, task)| task)
    }

    pub fn task_complete(&mut self, task: &Task) {
        self.completed_tasks.insert(task.clone());
        for cond in &task.satisfies {
            self.satisfied.insert(cond.clone());
        }
    }

    pub fn reset(&mut self) {
        self.runnable_tasks.clear();
        for task in &self.tasks {
            if self.completed_tasks.contains(task) {
                continue;
            }
            if task.requires.iter().all(|req| self.satisfied.contains(req)) {
                self.runnable_tasks
                    .push((-(task.cycles as i32), task.clone()));
            }
        }
    }
}

pub fn get_next_tasks(unutilised_cpus: &HashSet<u32>, cpus: &mut HashMap<u32, CPU>) -> HashMap<u32, Option<Task>> {
    /* returns a HashMap of next tasks for the given list of cpus*/
    unutilised_cpus.into_iter()
        .filter_map(|id| cpus.get_mut(&id).map(|f| (*id, f.get_task())))
        .collect()
}
