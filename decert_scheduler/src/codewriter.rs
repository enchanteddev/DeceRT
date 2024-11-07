use std::{
    collections::HashMap,
    fs::{self, create_dir_all},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub fn_identifier: Arc<str>,
    pub cycles: u16,
    pub args: Vec<Arc<str>>,
}

#[derive(Debug, Clone)]
pub struct Delay {
    pub call_time_ms: i32,
}

#[derive(Debug, Clone)]
pub enum CodeTask {
    FunctionCall(FunctionCall),
    Delay(Delay),
}

fn write_wrapper_fn(name: &str, args: Vec<Arc<str>>) -> String {
    let wrapper = include_str!("../../cpp_snippets/task_wrapper.cpp");
    let extractor = include_str!("../../cpp_snippets/args_extract.cpp");
    let arg_var_names: Vec<String> = args
        .iter()
        .map(|arg| format!("var_{}", arg.to_lowercase()))
        .collect();

    let args_extracted = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            extractor
                .replace("{SENSORNAME}", arg)
                .replace("{SVARNAME}", arg_var_names[i].as_str())
        })
        .fold(String::new(), |acc, x| acc + &x);

    wrapper
        .replace("{TASKNAME}", name)
        .replace("{EXTRACTARGS}", &args_extracted)
        .replace("{ARGS}", arg_var_names.join(", ").as_str())
}

fn write_args_array(
    name: &str,
    args: Vec<Arc<str>>,
    arg_vars: HashMap<Arc<str>, String>,
) -> String {
    format!(
        "void* args_{name}[] = {{ {} }};",
        args.iter()
            .filter_map(|f| arg_vars.get(f).map(|f| format!("(void*) {f}")))
            .collect::<Vec<String>>()
            .join(", ")
    )
}

fn write_run_task(
    name: &str,
    args: Vec<Arc<str>>,
    arg_vars: HashMap<Arc<str>, String>,
    delay: u32,
) -> String {
    format!(
        "{}\nrun_task(wrapper_{name}, args_{name}, {delay});",
        write_args_array(name, args, arg_vars)
    )
}

#[derive(Debug, Clone)]
pub struct CodeWriter {
    pub tasks: Vec<CodeTask>,
    pub delayed_at: Option<i32>,
}

impl CodeWriter {
    pub fn new() -> CodeWriter {
        CodeWriter {
            tasks: vec![],
            delayed_at: None,
        }
    }
    pub fn append(&mut self, task: CodeTask, current_time_ms: i32) {
        /*May add a delay if a delay was started previously */
        match self.delayed_at {
            Some(t) => {
                assert!(t < current_time_ms);
                self.tasks.push(CodeTask::Delay(Delay {
                    call_time_ms: current_time_ms - t,
                }));
                self.delayed_at = None;
            }
            None => {}
        }
        self.tasks.push(task);
    }
    pub fn start_delay(&mut self, current_time_ms: i32) {
        /*
        Starts a delay, it is safe to call this function multiple times.
        Delay will be counted from the first call to this function.
         */
        match self.delayed_at {
            Some(_) => {}
            None => {
                self.delayed_at = Some(current_time_ms);
            }
        }
    }

    pub fn commit(&mut self, path: PathBuf) -> Result<(), String> {
        /*
        Commits the code to the file.
        Starts writing from the beginning, hence better to run this only at the end
         */
        let task_wrappers = self
            .tasks
            .iter()
            .filter_map(|f| {
                if let CodeTask::FunctionCall(t) = f {
                    Some(write_wrapper_fn(&t.fn_identifier, t.args.clone()))
                } else {
                    None
                }
            })
            .fold(String::new(), |acc, x| acc + &x);

        let all_args: Vec<Arc<str>> = self
            .tasks
            .iter()
            .filter_map(|f| {
                if let CodeTask::FunctionCall(t) = f {
                    Some(t.args.clone())
                } else {
                    None
                }
            })
            .flat_map(|f| f)
            .collect();

        let arg_vars: HashMap<Arc<str>, String> = all_args
            .iter()
            .map(|f| (f.clone(), format!("var_{}", f.to_lowercase())))
            .collect();

        let inits = all_args
            .iter()
            .map(|f| format!("{}* var_{} = new {}();\n\t", f, arg_vars[f], f))
            .fold(String::new(), |acc, x| acc + &x);

        let mut tasks_string = "".to_string();
        for task in self.tasks.clone() {
            match task {
                CodeTask::FunctionCall(t) => {
                    tasks_string += &write_run_task(
                        &t.fn_identifier,
                        t.args.clone(),
                        arg_vars.clone(),
                        t.cycles as u32,
                    );
                    //&format!(
                    //     "runTask({}, {} ,{});\n\t\t",
                    //     t.fn_identifier,
                    //     t.args.join(", "),
                    //     t.cycles
                    // );
                }
                CodeTask::Delay(t) => {
                    tasks_string += &format!("delay({});\n", t.call_time_ms);
                }
            }
        }
        let entry_snippet = include_str!("../../cpp_snippets/entry.cpp");
        let final_code = entry_snippet.replace("{TASKS}", &tasks_string);

        create_dir_all(&path).map_err(|e| e.to_string())?;

        let mut entry_cpp = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.join("entry.cpp"))
            .map_err(|e| e.to_string())?;
        entry_cpp
            .write(final_code.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
