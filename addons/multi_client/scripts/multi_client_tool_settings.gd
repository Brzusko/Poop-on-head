extends Object;
class_name MultiClientToolSettings;

const _settings_dir_path: String = "user://multi_client_tool";
const _settings_file_name: String = "settings.json";
const _settings_file_path: String = _settings_dir_path + "/" + _settings_file_name;

func load_settings() -> MultiClientSaveData:
	var loaded_save_data: MultiClientSaveData = MultiClientSaveData.new();

	if DirAccess.dir_exists_absolute(_settings_dir_path) && FileAccess.file_exists(_settings_file_path):
		var file: FileAccess = FileAccess.open(_settings_file_path, FileAccess.READ);

		if !file:
			printerr("Could not open save file located in: %s, with error: %s" % [_settings_file_path, FileAccess.get_open_error()]);
			return loaded_save_data;

		var file_content: Dictionary = JSON.parse_string(file.get_as_text());

		if !file_content:
			printerr("Could not parse file located in %s" % _settings_file_path);
			return loaded_save_data;
		
		loaded_save_data = MultiClientSaveData.from_dict(file_content);
		return loaded_save_data;

	__generate_save_file();
	return loaded_save_data;
	
func __generate_save_file() -> void:
	if !DirAccess.dir_exists_absolute(_settings_dir_path):
		var result: int = DirAccess.make_dir_absolute(_settings_dir_path);

		if result != OK:
			printerr("Could not create save directory at path: %s" % _settings_dir_path);
			return;
	
	if !FileAccess.file_exists(_settings_file_path):
		var empty_data: MultiClientSaveData = MultiClientSaveData.new();
		save_data(empty_data);


func save_data(data_to_save: MultiClientSaveData) -> bool:
	var file_content: String = JSON.stringify(data_to_save.to_dict());
	var file: FileAccess = FileAccess.open(_settings_file_path, FileAccess.WRITE);

	if !file:
		printerr("Could not write save file located in: %s, with error: %s" % [_settings_file_path, FileAccess.get_open_error()]);
		return false;

	file.store_string(file_content);

	return true;
