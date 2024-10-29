use std::env;

use cli::create_project;

mod cli;
fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() == 1){
        // error
    }
    let project_name = &args[1];
    create_project(project_name.clone()).expect("Wrong Input format");
}
