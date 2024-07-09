use std::collections::VecDeque;
use godot::classes::Os;
use godot::engine::EditorPlugin;
use godot::prelude::{Gd, GString, PackedStringArray};

struct Process {
    pid: i32,
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
        let mut args: PackedStringArray = PackedStringArray::new();
        args.resize(2);
        args[0] = GString::from("arg1");
        args[1] = GString::from("arg2");
        self.os_singleton.create_instance(PackedStringArray::new());
    }

    pub fn kill_all_process_collection(&mut self) {
        while let Some(process)  = self.process_container.pop_back() {
            self.os_singleton.kill(process.pid);
        }
    }
}