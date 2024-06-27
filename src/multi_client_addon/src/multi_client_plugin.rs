use godot::classes::{EditorPlugin, IEditorPlugin};
use godot::prelude::*;
use crate::multi_client_data::MultiClientDataSaver;

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
struct MultiClientPlugin {
    base: Base<EditorPlugin>
}

#[godot_api]
impl IEditorPlugin for MultiClientPlugin {
    fn enter_tree(&mut self) {
        MultiClientDataSaver::load_data();
    }
}

