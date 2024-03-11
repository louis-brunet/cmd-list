use std::process::{Child, Stdio};

pub trait OutputFormatter {
    fn format_all(&self, complete_stdout: &[u8]);
}

#[derive(Debug)]
pub enum CommandRunnerError {
    ChildProcessSpawnFailed,
    NoChildStdoutError,
}

#[derive(Debug)]
pub struct CommandRunner {}

impl CommandRunner {
    pub fn run_command(shell: &str, cmd: &str, args: &str) -> Result<Child, CommandRunnerError> {
        let concatenated = format!("{} {}", cmd, args);
        let mut cmd = std::process::Command::new(shell);
        cmd.arg("-c")
            .arg(concatenated)
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|_err| CommandRunnerError::ChildProcessSpawnFailed)
    }
}
