use std::{
    collections::HashMap,
    env::{current_dir, set_current_dir},
    fs::{self, create_dir, File},
    io::{self, Write},
    path::Path,
};

use confparse::Conf;
use decert_scheduler::schedule;
use itertools::Itertools;

use crate::artifacts::compile_entry_cpp;

fn write_input_port(port_name: &str, ports_hpp: &mut File) -> io::Result<()> {
    let input_port_snippet = include_str!("../cpp_snippets/input_port.cpp");
    ports_hpp
        .write(format!("\n{}\n", input_port_snippet.replace("NAME", &port_name)).as_bytes())?;

    Ok(())
}

fn write_output_port(port_name: &str, ports_hpp: &mut File) -> io::Result<()> {
    let output_port_snippet = include_str!("../cpp_snippets/output_port.cpp");
    ports_hpp
        .write(format!("\n{}\n", output_port_snippet.replace("NAME", &port_name)).as_bytes())?;

    Ok(())
}

fn write_sensor(sensor_name: &str, ports_hpp: &mut File) -> io::Result<()> {
    let sensor_snippet = include_str!("../cpp_snippets/sensor.cpp");
    ports_hpp.write(format!("\n{}\n", sensor_snippet.replace("NAME", &sensor_name)).as_bytes())?;

    Ok(())
}

pub fn create_project(project_name: &str) -> std::io::Result<()> {
    let path_dir = Path::new(project_name);
    fs::create_dir(path_dir)?;
    File::create(path_dir.join("sensors.json"))?;
    Ok(())
}

pub fn add_obc(id: u32) -> io::Result<()> {
    create_dir(format!("obc{id}"))?;
    create_dir(format!("obc{id}/entry"))?;
    create_dir(format!("obc{id}/lib"))?;
    File::create(format!("obc{id}/ports.hpp"))?;

    let mut port_file = File::create(format!("obc{id}/ports.hpp"))?;
    port_file.write(
        b"// not to be touched by user\n// will be regenerated to ensure correctness on each build",
    )?;
    File::create(format!("obc{id}/tasks.conf"))?;
    Ok(())
}

pub fn update_tasks() -> Result<Conf, String> {
    let conf = confparse::get_conf("tasks.conf")?;

    println!("{:?}", conf);

    let mut ports_hpp = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("ports.hpp")
        .map_err(|e| e.to_string())?;

    ports_hpp.write(
        b"// not to be touched by user\n// will be regenerated to ensure correctness on each build\n",
    ).map_err(|e| e.to_string())?;

    for inports in conf.inports.iter() {
        write_input_port(&inports, &mut ports_hpp).map_err(|e| e.to_string())?;
    }
    for outports in conf.outports.iter() {
        write_output_port(&outports, &mut ports_hpp).map_err(|e| e.to_string())?;
    }

    let sensors = conf.tasks.iter().flat_map(|x| x.args.clone()).unique();

    for sensor in sensors {
        println!("{:?}", sensor);
        write_sensor(&sensor, &mut ports_hpp).map_err(|e| e.to_string())?;
    }

    let task_snippet = include_str!("../cpp_snippets/task.cpp");

    for task in &conf.tasks {
        let mut file = match File::create_new(format!("entry/{}.cpp", task.name)) {
            Ok(x) => x,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.to_string()),
        };

        let task_code = task_snippet.replace("TASKNAME", &task.name).replace(
            "ARGS",
            &task
                .args
                .iter()
                .filter(|f| f.len() > 0)
                .map(|x| {
                    let first3lower = x[..3].to_lowercase();
                    format!("{x} {first3lower}")
                })
                .collect::<Vec<String>>()
                .join(", "),
        );

        file.write(task_code.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    for file in Path::new("entry").read_dir().map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?.path();
        let filename = file.file_stem().unwrap_or_default();
        let filename = filename.to_str().ok_or("Filename is not valid UTF-8")?;
        if !conf.tasks.iter().any(|f| &*f.name == filename) {
            Err(format!("'{filename}' is not the name of any task."))?
        }
    }

    Ok(conf)
}

fn precompilation() -> io::Result<HashMap<u32, Conf>> {
    let is_root = Path::new("./sensors.json").exists();
    if !is_root {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Not in project's root directory",
        ));
    }

    let mut obc_ids = Vec::new();

    for dir in Path::new(".").read_dir()? {
        let dir = dir?;
        let path = dir.path();
        if path.is_dir() {
            let Some(last_component_osstr) = path.components().last() else {
                continue;
            };
            let Some(last_component) = last_component_osstr.as_os_str().to_str() else {
                continue;
            };
            let Some(obc_id_str) = last_component.strip_prefix("obc") else {
                println!("Non obc folder found: {path:?}");
                continue;
            };

            let obc_id = obc_id_str
                .parse::<u32>()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            obc_ids.push(obc_id);
        }
    }

    let root_dir = current_dir()?;

    let mut topology = HashMap::new();

    for obc_id in obc_ids {
        set_current_dir(root_dir.join(Path::new(&format!("obc{obc_id}/"))))?;
        let conf = update_tasks().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        topology.insert(obc_id, conf);
    }

    set_current_dir(root_dir)?;

    Ok(topology)
}

pub fn compile() -> Result<(), String> {
    let topology = precompilation().map_err(|e| e.to_string())?;
    let sensors = schedule(&topology)?;

    // creating class strings for each sensors and ports in Vec:sensors
    let mut sensor_impl: HashMap<String, String> = HashMap::new(); // sensor_name: implementation
    // let mut port_impl: HashMap<String, String> = HashMap::new(); // port_name: implementation
    let mut port2obc: HashMap<String, u32> = HashMap::new(); // port_name:OBC which declares it as out port 

    // sensors
    let sensor_impl_snippet = include_str!("../cpp_snippets/sensor_impl.cpp");
    for (id, sensor) in sensors.sensors.iter().enumerate() {
        let mut sensor_code = sensor_impl_snippet.to_string();

        sensor_code = sensor_code.replace("{NAME}", &sensor.name);
        sensor_code = sensor_code.replace("{ST}", &sensor.from);
        sensor_code = sensor_code.replace("{ET}", &sensor.to);
        sensor_code = sensor_code.replace("{ID}", &id.to_string());

        sensor_impl.insert(sensor.name.to_string(), sensor_code);
    }

    // ports
    for (obc_id, conf) in &topology {
        for port in &conf.outports {
            if port2obc.contains_key(&port.to_string()) {
                return Err(format!(
                    "Two OBCs cannot have the same output port: {obc_id} and {}",
                    port2obc[&port.to_string()]
                ));
            }
            // Insert the port if it doesn't exist to track it
            port2obc.insert(port.to_string(), obc_id.clone());
        }
    }

    // ports implementations
    let port_impl_snippet = include_str!("../cpp_snippets/port_impl.cpp");
    let port_impl: HashMap<String, String> = port2obc.iter().map(|(port_name, obc_id)| {
        let mut port_code = port_impl_snippet.to_string();

        port_code = port_code.replace("{NAME}", &port_name);
        port_code = port_code.replace("{ID}", &obc_id.to_string());
        
        (port_name.clone(), port_code)
    }).collect();


    // creates ports.cpp for each obc file
    let root_dir = current_dir().map_err(|e| e.to_string())?;

    for (obc_id, conf) in &topology {
        let mut ports_cpp = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(root_dir.join(format!("obc{obc_id}")).join("ports.cpp"))
            .map_err(|e| e.to_string())?;

        let mut ports_used = conf.outports.clone();
        ports_used.append(&mut conf.inports.clone());
        let sensors_used = conf.tasks.iter().flat_map(|x| x.args.clone()).unique();
        for sensor_name in sensors_used {
            let Some(implementation) = sensor_impl.get(&*sensor_name) else {
                Err(format!(
                    "Sensor used : {sensor_name} is not defined in sensor.json"
                ))?
            };
            ports_cpp.write(implementation.as_bytes()).map_err(|e| e.to_string()) ?;
        }

        for port_name in ports_used {
            let Some(implementation) = port_impl.get(&*port_name) else {
                Err(format!(
                    "Port used : {port_name} is not defined in sensor.json"
                ))?
            };
            ports_cpp.write(implementation.as_bytes()).map_err(|e| e.to_string()) ?;
        }
    }

    for (obc_id, _) in &topology {
        compile_entry_cpp(*obc_id);
    }
    Ok(())
}
