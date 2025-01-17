use std::process::Command;

pub struct ProcessManager {
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    child_processes: Vec<std::process::Child>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            child_processes: Vec::new(),
        }
    }

    pub fn launch_process(&mut self, program: &str, args: &[&str]) -> std::io::Result<()> {
        #[cfg(target_os = "windows")]
        {
            let child = Command::new(program)
                .args(args)
                .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
                .spawn()?;
            self.child_processes.push(child);
        }

        #[cfg(target_os = "linux")]
        {
            let child = Command::new(program).args(args).spawn()?;
            self.child_processes.push(child);
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg("-a")
                .arg(program)
                .args(args)
                .spawn()?;
        }

        Ok(())
    }

    pub fn cleanup(&mut self) {
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        {
            for child in &mut self.child_processes {
                let _ = child.kill();
            }
            self.child_processes.clear();
        }
    }
}
