use std::{
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

        let mut tasks_string = "".to_string();
        for task in self.tasks.clone() {
            match task {
                CodeTask::FunctionCall(t) => {
                    tasks_string += &format!(
                        "runTasks({}, {} ,{})",
                        t.fn_identifier,
                        t.args.join(", "),
                        t.cycles
                    );
                }
                CodeTask::Delay(t) => {
                    tasks_string += &format!("delay({})", t.call_time_ms);
                }
            }
        }
        let entry_snippet = include_str!("../../cpp_snippets/obc.cpp");
        let final_code = entry_snippet.replace("__TASKS__", &tasks_string);

        create_dir_all(&path);

        let mut entry_cpp = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(path.join("entry.cpp"))
            .map_err(|e| e.to_string())?;
        entry_cpp
            .write(final_code.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
