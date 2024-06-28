use godot::prelude::*;
use godot::classes::{VBoxContainer, IVBoxContainer, CheckBox, LineEdit, Button};
use godot::global::Error;
use crate::multi_client_data::MultiClientData;

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
struct MainToolWindow
{
    base: Base<VBoxContainer>,
    #[export]
    cmd_line: Option<Gd<LineEdit>>,
    #[export]
    clients_line: Option<Gd<LineEdit>>,
    #[export]
    run_from_main_checkbox: Option<Gd<CheckBox>>,
    #[export]
    run_game_button: Option<Gd<Button>>
}

#[godot_api]
impl IVBoxContainer for MainToolWindow
{
    fn init(base: Base<VBoxContainer>) -> Self {
        Self {
            base,
            cmd_line: None,
            clients_line: None,
            run_from_main_checkbox: None,
            run_game_button: None,
        }
    }

    fn ready(&mut self) {
        if !self.run_game_button.is_none()
        {
            let callable: Callable = self.base_mut().callable(StringName::from("connect_button_pressed"));
            let btn: &mut Gd<Button> = self.run_game_button.as_mut().unwrap();
            let error: Error = btn.connect(StringName::from("pressed"), callable);

            if error != Error::OK
            {
                godot_print!("Could not connect signal to button, error: {:?}", error);
            }
        }
    }
}

impl MainToolWindow
{
    pub fn assign_data(&mut self, data: MultiClientData)
    {
        if !self.cmd_line.is_none()
        {

        }

        if !self.clients_line.is_none()
        {

        }

        if !self.run_from_main_checkbox.is_none()
        {

        }
    }

    pub fn retrieve_data() -> MultiClientData
    {
        MultiClientData::default()
    }
}

#[godot_api]
impl MainToolWindow
{
    #[func]
    pub fn connect_button_pressed(&mut self)
    {
        godot_print!("Connect");
    }
}