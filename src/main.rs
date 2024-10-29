use std::process::exit;

mod cli;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: decert <command> [options]");
        exit(0);
    }
    match args[1].as_str() {
        "add-obc" => {
            let Ok(id) = args[2].parse::<u32>() else {
                println!("Usage: decert add-obc <id>");
                exit(0);
            };
            match cli::add_obc(id) {
                Ok(_) => println!("New OBC created"),
                Err(e) => println!("Error: {}", e),
            };
        },
        _ => println!("Unknown command"),
    }
}
