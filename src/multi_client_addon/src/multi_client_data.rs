use std::fs::File;
use godot::classes::{FileAccess, DirAccess, file_access, Json};
use godot::builtin::GString;
use godot::global::Error;
use godot::prelude::{Gd, godot_error};
use serde::{Serialize, Deserialize};

pub struct MultiClientDataSaver;
static SETTINGS_DIRECTORY_PATH: &str = "user://multi_client_tool";
static SETTINGS_FILE_FULL_PATH: &str = "user://multi_client_tool/settings.json";

impl MultiClientDataSaver {
    pub fn save_data(data_to_save: &MultiClientData)
    {
        let file: Option<Gd<FileAccess>> = FileAccess::open(
            GString::from(SETTINGS_FILE_FULL_PATH),
            file_access::ModeFlags::WRITE);

        if file.is_none()
        {
            godot_error!("Could not write settings file, error: {:?}, at path {:?}", FileAccess::get_open_error(), SETTINGS_FILE_FULL_PATH);
            return;
        }

        let serialized_data = serde_json::to_string(data_to_save).unwrap();
        let mut unwrapped_file: Gd<FileAccess> = file.unwrap();

        unwrapped_file.store_string(GString::from(serialized_data));
        unwrapped_file.close();
    }

    pub fn load_data() -> Option<MultiClientData>
    {
        if !Self::create_structure_if_missing()
        {
            return None;
        }

        let file: Option<Gd<FileAccess>> = FileAccess::open(GString::from(SETTINGS_FILE_FULL_PATH), file_access::ModeFlags::READ);

        if file.is_none()
        {
            let error: Error = FileAccess::get_open_error();
            godot_error!("Could not open settings file, error: {:?}", error);

            return None;
        }

        let unwrapped_file: Gd<FileAccess> = file.unwrap();
        let file_content = unwrapped_file.get_as_text();

        if file_content.is_empty()
        {
            return None;
        }

        let result = serde_json::from_str::<MultiClientData>(file_content.to_string().as_str());

        match result {
            Ok(data) => { Some(data) }
            Err(error) => {
                godot_error!("Settings parsing error: {:?}", error);
                None
            }
        }
    }

    fn create_structure_if_missing() -> bool {
        if !DirAccess::dir_exists_absolute(GString::from(SETTINGS_DIRECTORY_PATH)) {
            let error: Error = DirAccess::make_dir_absolute(GString::from(SETTINGS_DIRECTORY_PATH));

            if error != Error::OK
            {
                godot_error!("Could not create setting dir {:?} at path {:?}", error, SETTINGS_DIRECTORY_PATH);
                return false;
            }
        }

        if !FileAccess::file_exists(GString::from(SETTINGS_FILE_FULL_PATH))
        {
            let default_data: MultiClientData = MultiClientData::default();
            Self::save_data(&default_data);
            return true;
        }

        true
    }
}

#[derive(Serialize, Deserialize)]
pub struct MultiClientData {
    pub clients_count: i64,
    cmd_line: String,
    pub run_from_main: bool,
}

impl MultiClientData
{
    pub fn default() -> Self
    {
        Self
        {
            clients_count: 0,
            cmd_line: String::from(""),
            run_from_main: false
        }
    }

    pub fn cmd_line_text(&self) -> GString
    {
        GString::from(self.cmd_line.to_string())
    }

    pub fn assign_cmd_line_text(&mut self, text: GString)
    {
        self.cmd_line = text.to_string();
    }
}
