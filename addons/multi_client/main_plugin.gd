@tool
extends EditorPlugin;
class_name MultiClientPlugin;

const _main_plugin_scene_path: StringName = "";
var _main_control_node: Control = null;
var _is_initialized: bool = false;

func _enter_tree():
	__initialize_plugin();
		
	 
func _exit_tree():
	if !_is_initialized:
		return;
		
	__dispose_plugin();
	
	
func __initialize_plugin() -> void:
	var plugin_packed_scene: PackedScene = load(_main_plugin_scene_path);
	
	if !plugin_packed_scene:
		printerr("Could not load plugin scene, path: %s" % _main_plugin_scene_path);
		return;
		
	


func __dispose_plugin() -> void:
	pass;

