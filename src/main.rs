use std::process::exit;

mod cli;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: decert <command> [options]");
        exit(0);
    }
    match args[1].as_str() {
        "create-project" => {
            let Some(project_name) = args.get(2) else {
                println!("Usage: decert create-project <name>");
                exit(1)
            };
            match cli::create_project(project_name) {
                Ok(_) => println!("Project created"),
                Err(e) => println!("Error: {}", e),
            };
        }
        "add-obc" => {
            let Ok(id) = args[2].parse::<u32>() else {
                println!("Usage: decert add-obc <id>");
                exit(2);
            };
            match cli::add_obc(id) {
                Ok(_) => println!("New OBC created"),
                Err(e) => println!("Error: {}", e),
            };
        },
        "update-tasks" => {
            match cli::update_tasks() {
                Ok(_) => println!("Tasks updated"),
                Err(e) => println!("Error: {}", e),
            };
        }
        _ => println!("Unknown command"),
    }
}
