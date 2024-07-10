use godot::prelude::*;
use godot::classes::{ILineEdit, LineEdit, RegEx, RegExMatch};
use godot::global::Error;
use std::any::type_name;
use godot::obj::BaseMut;

#[derive(GodotClass)]
#[class(base=LineEdit)]
pub struct NumericLineEdit
{
    base: Base<LineEdit>,
    reg_ex: Gd<RegEx>,
    value_cache: GString,
}

#[godot_api]
impl ILineEdit for NumericLineEdit
{
    fn init(base: Base<LineEdit>) -> Self
    {
        Self {
            base,
            reg_ex: RegEx::create_from_string(GString::from("")).unwrap(),
            value_cache: GString::new(),
        }
    }

    fn ready(&mut self) {
        let error: Error = self.reg_ex.compile(GString::from("^[0-9]+$"));

        if error != Error::OK
        {
            godot_error!("{:?} could not compile RegEx, error: {:?}", type_name::<LineEdit>(), error);
            return;
        }

        let callable: Callable = self.base_mut().callable(StringName::from("value_changed"));
        self.base_mut().connect(StringName::from("text_changed"), callable);
    }
}

#[godot_api]
impl NumericLineEdit {

    #[func]
    pub fn value_changed(&mut self, new_text: GString)
    {
        let result: Option<Gd<RegExMatch>> = self.reg_ex.search(new_text.clone());

        if result.is_some()
        {
            self.value_cache = new_text;
            return;
        }

        let value_cache: GString = self.value_cache.clone();

        let mut base_api: BaseMut<NumericLineEdit> = self.base_mut();
        let caret_pos:i32 = base_api.get_caret_column();
        base_api.set_text(value_cache);
        base_api.set_caret_column(caret_pos);
    }
    
    pub fn get_value_as_int(&self) -> i64 {
        let value = self.base().get_text();
        if value.is_empty() {
            return 0;
        }
        let value_as_rust_string = value.to_string();
        let parsed_value = value_as_rust_string.parse::<i64>();
        match parsed_value {
            Ok(val) => { val }
            Err(_) => { 0 }
        }
    }
}