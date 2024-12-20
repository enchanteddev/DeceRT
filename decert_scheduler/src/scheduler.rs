use std::{collections::HashMap, fmt::Debug, sync::Arc};

use confparse::Task;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitMap {
    map: u128,
}

impl Debug for BitMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hm = self.iter().fold(HashMap::new(), |mut hm, i| {
            hm.insert(i, self.get(i));
            hm
        });
        write!(f, "BitMap {{ {:?} }} val: {:?}", hm, self.map)
    }
}

impl BitMap {
    pub fn new() -> Self {
        BitMap { map: 0 }
    }
    
    pub fn get(&self, index: u8) -> bool {
        self.map & (1 << index) == (1 << index)
    }

    pub fn set(&mut self, index: u8, val: bool) {
        // println!("index: {:?}, val: {:?}", index, val);
        self.map &= !(1 << index);
        self.map |= (val as u128) << index;
    }

    pub fn conflict(&self, other: &Self) -> bool {
        self.map & other.map == 1
    }

    pub fn conflict_u64(&self, other_value: u128) -> bool {
        self.map & other_value == 1
    }

    pub fn combine(&self, other: &Self) -> Self {
        BitMap {
            map: self.map & other.map,
        }
    }

    pub fn combine_u64(&self, other_value: u128) -> Self {
        BitMap {
            map: self.map & other_value,
        }
    }

    pub fn is_filled(&self, index: u8) -> bool {
        self.map & ((1 << (index + 1)) - 1) == ((1 << (index + 1)) - 1)
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        (0..128).filter(move |&i| self.get(i))
    }

    pub fn get_map(&self) -> u128 {
        self.map
    }
}

pub fn task_schedule(
    tasks: &Vec<(Task, u8)>,
    sensors_to_int: &HashMap<Arc<str>, u8>,
    sensors_used: BitMap,
) -> Vec<Task> {
    task_schedule_rec(tasks, 0, sensors_to_int, sensors_used)
        .iter()
        .map(|i|{
            tasks[i as usize].0.clone()  
        })
        .collect()
}

fn task_schedule_rec(
    tasks: &Vec<(Task, u8)>,
    index: u8,
    sensors_to_int: &HashMap<Arc<str>, u8>,
    sensors_used: BitMap,
) -> BitMap {
    println!("index: {:?}, sensors_used: {:?}, sensors_to_int: {:?}", index, sensors_used, sensors_to_int);
    if sensors_used.is_filled(sensors_to_int.len() as u8) {
        return BitMap { map: 0 };
    }
    let Some((task, _)) = tasks.get(index as usize) else {
        return BitMap { map: 0 };
    };
    // to check if the task is runnable
    if task.args.iter().any(|f| sensors_used.get(sensors_to_int[f])) {
        return BitMap { map: 0 };
    }
    let mut s = sensors_used.clone();
    task.args.iter().for_each(|sensor| {
        let Some(i) = sensors_to_int.get(sensor) else {
            return;
        };
        s.set(*i, true);
    });
    println!("task: {:?}", task);
    println!("s: {:?}", s);

    let mut task_marked = task_schedule_rec(tasks, index + 1, sensors_to_int, s);
    task_marked.set(index, true);
    println!("task_marked_before: {:?}", task_marked);

    let s = sensors_used.clone();
    let task_unmarked = task_schedule_rec(tasks, index + 1, sensors_to_int, s);

    let tml = task_marked.iter().fold(0, |acc, x| {
        acc + tasks.get(x as usize).map(|f| f.1).unwrap_or(0)
    });
    let tuml = task_unmarked.iter().fold(0, |acc, x| {
        acc + tasks.get(x as usize).map(|f| f.1).unwrap_or(0)
    });

    println!("task_marked: {:?}", task_marked);
    println!("task_unmarked: {:?}", task_unmarked);
    if tml >= tuml {
        task_marked
    } else {
        task_unmarked
    }
}
