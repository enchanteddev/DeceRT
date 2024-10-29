use std::sync::Arc;
mod parse;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: Arc<str>,
    pub args: Vec<Arc<str>>,
    pub requires: Vec<Arc<str>>,
    pub satisfies: Vec<Arc<str>>,
    pub cycles: u16
}

#[derive(Debug, Clone)]
pub struct Conf {
    pub inports: Vec<Arc<str>>,
    pub outports: Vec<Arc<str>>,
    pub initial: Vec<Arc<str>>,
    pub tasks: Vec<Task>
}


pub fn get_conf(path: &str) -> Result<Conf, String> {
    todo!()
}