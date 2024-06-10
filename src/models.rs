use std::collections::HashMap;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio, ExitStatus};


struct Process {
    name: String,
    path: String,
    arguments: HashMap<String, String>,
    monitored: bool
}

impl Process {
    fn launch(self) -> ExitStatus {
        let mut p = Command::new(self.path).spawn().expect("Spawn failed");
        p.wait().expect("Wait failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launch_simple_no_args_sucess() {
        let p = Process {
            name: String::from("one"),
            path: String::from("/usr/bin/true"),
            arguments: HashMap::new(),
            monitored: false
        };
        assert!(p.launch().success());
    }

    #[test]
    fn launch_simple_no_args_fail() {
       let p = Process {
            name: String::from("one"),
            path: String::from("/usr/bin/false"),
            arguments: HashMap::new(),
            monitored: false
        };
        assert_ne!(p.launch().success(), true);
    }
}