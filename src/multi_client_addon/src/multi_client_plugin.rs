use godot::classes::{EditorPlugin, IEditorPlugin};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
struct MultiClientPlugin {
    base: Base<EditorPlugin>
}

#[godot_api]
impl IEditorPlugin for MultiClientPlugin {

}

mod internal {
    struct MultiClientDataSaver {}
    impl MultiClientDataSaver {}

    struct MultiClientData {}
}
