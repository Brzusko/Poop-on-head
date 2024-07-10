use std::ops::Deref;
use std::rc::Rc;
use std::sync::Mutex;
use godot::prelude::*;
use godot::classes::{EditorPlugin, IEditorPlugin, PackedScene};
use godot::engine::{Control, Window};
use crate::main_tool_window::MainToolWindow;

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
struct MultiClientPlugin {
    base: Base<EditorPlugin>,
    main_tool_window: Option<Gd<MainToolWindow>>
}

#[godot_api]
impl IEditorPlugin for MultiClientPlugin {
    fn enter_tree(&mut self) {
        let path: GString = GString::from("res://addons/multi_client/scenes/ui/tool_window.tscn");
        let packed_scene_unwrapped: Result<Gd<PackedScene>, IoError> = try_load::<PackedScene>(path);
        
        if packed_scene_unwrapped.is_err() {
            return;
        }
        
        let packed_scene: Gd<PackedScene> = packed_scene_unwrapped.unwrap();
        let loaded_node: Option<Gd<Node>> = packed_scene.instantiate();
        
        if loaded_node.is_none() {
            return;
        }
        
        let window_candidate = loaded_node.unwrap().try_cast::<MainToolWindow>();
        
        if window_candidate.is_err()
        {
            window_candidate.err().unwrap().queue_free();
            return;
        }
        
        let window_scene = window_candidate.unwrap();
        self.main_tool_window = Some(window_scene.clone());
        self.base_mut().add_control_to_bottom_panel(window_scene.upcast(), GString::from("Multi Client Runner"));
    }

    fn exit_tree(&mut self) {
        if self.main_tool_window.is_none() {
            return;
        }
        
        let control = self.main_tool_window.clone().unwrap().upcast::<Control>();
        self.base_mut().remove_control_from_bottom_panel(control);
    }
}