use std::sync::Arc;
mod parse;


struct Task {
    name: Arc<str>,
    args: Vec<Arc<str>>,
    requires: Vec<Arc<str>>,
    satisfies: Vec<Arc<str>>,
    cycles: u16
}


struct Conf {
    inports: Vec<Arc<str>>,
    outports: Vec<Arc<str>>,
    initial: Vec<Arc<str>>,
    tasks: Vec<Task>
}


