mod internal {
    use godot::classes::{FileAccess, DirAccess, file_access};
    use godot::builtin::GString;
    use godot::global::Error;
    use godot::prelude::{Gd, godot_error};
    use serde::{Serialize, Deserialize};

    pub struct MultiClientDataSaver {

    }

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
        }

        pub fn load_data() -> Option<MultiClientData>
        {
            Self::create_structure_if_missing();
            None
        }

        fn create_structure_if_missing() -> Option<MultiClientData> {
            if !DirAccess::dir_exists_absolute(GString::from(SETTINGS_DIRECTORY_PATH)) {
                let error: Error = DirAccess::make_dir_absolute(GString::from(SETTINGS_DIRECTORY_PATH));

                if error != Error::OK
                {
                    godot_error!("Could not create setting dir {:?} at path {:?}", error, SETTINGS_DIRECTORY_PATH);
                    return None;
                }
            }

            if !FileAccess::file_exists(GString::from(SETTINGS_FILE_FULL_PATH))
            {
                let default_data: MultiClientData = MultiClientData::default();
                Self::save_data(&default_data);
                return Some(default_data);
            }

            None
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct MultiClientData {
        clients_count: i64,
        cmd_line: String,
        run_from_main: bool,
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
    }
}

use godot::classes::{EditorPlugin, IEditorPlugin};
use godot::prelude::*;
use crate::multi_client_plugin::internal::MultiClientDataSaver;

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

