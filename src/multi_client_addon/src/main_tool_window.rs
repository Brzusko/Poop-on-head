use std::num::ParseIntError;
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

#[godot_api]
impl MainToolWindow
{
    #[func]
    pub fn connect_button_pressed(&mut self)
    {
        godot_print!("Connect");
    }
}

impl MainToolWindow
{
    pub fn assign_data(&mut self, data: &MultiClientData)
    {
        if let Some(mut cmd_line) = self.cmd_line.as_mut()
        {
            let text: GString = data.cmd_line_text();
            cmd_line.set_text(text);
        }

        if let Some(mut clients_count_line_edit) = self.clients_line.as_mut()
        {
            let clients_count: i64 = data.clients_count;
            clients_count_line_edit.set_text(GString::from(clients_count.to_string()));
        }

        if let Some(mut run_from_main_checkbox) = self.run_from_main_checkbox.as_mut()
        {
            let run_from_main: bool = data.run_from_main;
            run_from_main_checkbox.set_pressed(run_from_main);
        }
    }

    pub fn retrieve_data(&mut self) -> MultiClientData
    {
        let mut data = MultiClientData::default();

        if let Some(mut cmd_line) = self.cmd_line.as_mut()
        {
            data.assign_cmd_line_text(cmd_line.get_text())
        }

        if let Some(mut clients_count_line_edit) = self.clients_line.as_mut()
        {
            let clients: Result<i64, ParseIntError> = String::from(clients_count_line_edit.get_text().to_string()).parse();

            if let Ok(parsed_clients) = clients
            {
                data.clients_count = parsed_clients;
            }
        }

        if let Some(mut run_from_main_checkbox) = self.run_from_main_checkbox.as_mut()
        {
            data.run_from_main = run_from_main_checkbox.is_pressed();
        }

        data
    }
}