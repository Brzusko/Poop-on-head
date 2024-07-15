use std::collections::VecDeque;
use godot::classes::Os;
use godot::prelude::{Gd, GString, PackedStringArray};

struct Process {
    pid: i32,
}

impl Process {
    pub fn new(pid: i32) -> Self {
        Self {
            pid
        }
    }
}

pub struct ClientRunner {
    process_container: VecDeque<Process>,
    os_singleton: Gd<Os>,
}

impl ClientRunner {
    pub fn new() -> Self {
        Self {
            process_container: VecDeque::new(),
            os_singleton: Os::singleton(),
        }
    }

    pub fn run_process(&mut self) {
        if !self.os_singleton.has_feature(GString::from("editor"))
        {
            return;
        }
        // TODO: add parsing for arguments later
        // let mut args: PackedStringArray = PackedStringArray::new();
        // args.resize(2);
        // args[0] = GString::from("arg1");
        // args[1] = GString::from("arg2");
        
        if self.os_singleton.has_feature(GString::from("macos")) {
            let path: GString;
            {
                path = self.os_singleton.get_executable_path();
            }
            
            let pid: i32 = self.os_singleton.create_process(path, PackedStringArray::new());
            
            if pid == -1 {
                return;
            }
            
            let process: Process = Process::new(pid);
            self.process_container.push_back(process);
            return;
        }
        
        let pid: i32 = self.os_singleton.create_instance(PackedStringArray::new());
        if  pid == -1 {
            return;
        }
        let process: Process = Process::new(pid);
        self.process_container.push_back(process);
    }

    pub fn kill_all_process_collection(&mut self) {
        while let Some(process)  = self.process_container.pop_back() {
            self.os_singleton.kill(process.pid);
        }
    }
}