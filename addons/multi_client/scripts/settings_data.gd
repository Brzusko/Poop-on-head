extends RefCounted;
class_name MultiClientSaveData;

const dict_keys: Array[String] = [
	"cmd_string",
	"client_count",
	"start_from_main_scene",
];

var cmd_string: String = "": set = _cmd_string_setter;

func _cmd_string_setter(value: String) -> void:
	self.set_meta("cmd_string", value);
	cmd_string = value;

var client_count: int = 0: set = _client_count_setter;

func _client_count_setter(value: int) -> void:
	if value < 0:
		return;
	
	self.set_meta("client_count", value);
	client_count = value;

var start_from_main_scene: bool = false: set = _start_from_main_scene;

func _start_from_main_scene(value: bool) -> void:
	self.set_meta("start_from_main_scene", value);
	start_from_main_scene = value;
 
func _init():
	for key: String in dict_keys:
		self.set_meta(key, self.get(key));

func to_dict() -> Dictionary:
	var new_dict: Dictionary = {};

	for key: String in dict_keys:
		if !self.has_meta(key):
			continue;
		
		new_dict[key] = self.get_meta(key);

	return new_dict;

static func from_dict(data_to_parse: Dictionary) -> MultiClientSaveData:
	var save_data: MultiClientSaveData = MultiClientSaveData.new();

	# for now, tool does not support nested data in file like arrays/dictionaries
	for key: String in data_to_parse.keys():
		if !save_data.has_meta(key):
			continue;
		
		save_data.set(key, data_to_parse[key]);
	
	return save_data;
