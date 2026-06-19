use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use log::debug;
use thiserror::Error;

const TMPFS_PATH: &str = "/tmp/shine";
const DIST_XML_FILE: &str = "ui.xml";
const DIST_FILE_VARIABLE: &str = "XML";


#[derive(Error, Debug)]
pub enum ShellError {
    #[error("Failed to create resource : {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to exec `{command}`: {source}")]
    ExecutionFailed {
        command: String,
        source: std::io::Error,
    },

    #[error("Execution of `{command}` return exit code: {code}")]
    NonZeroExitCode { command: String, code: i32 },

    #[error("Failed to read shell output: {0}")]
    OutputReadingError(String),
}

#[derive(Debug)]
pub struct Shell {
    child: Child,
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
    dist_dir: PathBuf,
    pub dist_file: PathBuf,
}

impl Shell {
    pub fn new() -> Result<Self, ShellError> {
        let dist_dir = PathBuf::from(TMPFS_PATH);
        let dist_file = dist_dir.join(DIST_XML_FILE);

        fs::create_dir_all(&dist_dir)?;

        let mut child = Command::new("bash")
            .args(["--noprofile", "--norc"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        Ok(Shell {
            stdin: child.stdin.take().unwrap(),
            stdout: BufReader::new(child.stdout.take().unwrap()),
            child,
            dist_dir,
            dist_file,
        })
    }

    pub fn init_env(&mut self) -> Result<(), ShellError> {
        debug!("\n═══ Initialising env ═══");

        self.exec(format!(
            "export {}={}",
            DIST_FILE_VARIABLE,
            self.dist_file.to_string_lossy()
        ))?;

        self.exec(format!("mkdir -p '{}'", self.dist_dir.to_string_lossy()))?;

        Ok(())
    }

    pub fn exec(&mut self, cmd: String) -> Result<(), ShellError> {
        debug!("executing : {cmd}");

        let end_delimiter = "END-OF-EXEC";

        let cmd_full = format!(r#"{} 2>&1; echo "{}:$?";"#, cmd, end_delimiter);

        writeln!(self.stdin, "{cmd_full}").map_err(|e| ShellError::ExecutionFailed {
            command: cmd.clone(),
            source: e,
        })?;
        loop {
            let mut line = String::new();
            self.stdout
                .read_line(&mut line)
                .map_err(|e| ShellError::OutputReadingError(e.to_string()))?;
            if line.contains(end_delimiter) {
                let code = line
                    .split(':')
                    .last()
                    .unwrap_or_default()
                    .trim()
                    .parse()
                    .unwrap_or(-1);
                if code != 0 {
                    return Err(ShellError::NonZeroExitCode {
                        command: cmd.clone(),
                        code,
                    });
                }
                break;
            }
            print!("  {line}");
        }

        Ok(())
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = fs::remove_dir_all(&self.dist_dir);
    }
}
