struct GodotProcess
{
    pid: i64,
}

struct ClientRunner
{
    godot_process_list: Vec<GodotProcess>
}

impl ClientRunner {
    pub fn new() -> Self
    {
        Self {
            godot_process_list: vec![]
        }
    }
}