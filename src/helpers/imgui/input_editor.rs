use std::num::IntErrorKind;

use imgui::{InputText, Ui};

pub struct InputUsize {
    hint: String,
    auto_select_all: Option<bool>,
    allow_tab_input: Option<bool>,
    no_horizontal_scroll: Option<bool>,
    always_insert_mode: Option<bool>,
    read_only: Option<bool>,
    password: Option<bool>,
    no_undo_redo: Option<bool>,
}

impl InputUsize {
    pub fn new() -> Self {
        Self {
            hint: String::default(),
            auto_select_all: None,
            allow_tab_input: None,
            no_horizontal_scroll: None,
            always_insert_mode: None,
            read_only: None,
            password: None,
            no_undo_redo: None,
        }
    }

    // pub fn hint(mut self, text: &str) -> Self {
    //     self.hint = text.to_string();
    //     self
    // }

    pub fn auto_select_all(mut self, value: bool) -> Self {
        self.auto_select_all = Some(value);
        self
    }

    // pub fn allow_tab_input(mut self, value: bool) -> Self {
    //     self.allow_tab_input = Some(value);
    //     self
    // }

    // pub fn no_horizontal_scroll(mut self, value: bool) -> Self {
    //     self.no_horizontal_scroll = Some(value);
    //     self
    // }

    // pub fn always_insert_mode(mut self, value: bool) -> Self {
    //     self.always_insert_mode = Some(value);
    //     self
    // }

    // pub fn read_only(mut self, value: bool) -> Self {
    //     self.read_only = Some(value);
    //     self
    // }

    // pub fn password(mut self, value: bool) -> Self {
    //     self.password = Some(value);
    //     self
    // }

    // pub fn no_undo_redo(mut self, value: bool) -> Self {
    //     self.no_undo_redo = Some(value);
    //     self
    // }

    pub fn build(self, ui: &Ui, label: &str, value: &mut usize) -> bool {
        let mut value_str = value.to_string();

        let mut input_text = InputText::new(ui, label, &mut value_str)
            .chars_decimal(true)
            .chars_noblank(true);
        if self.hint != String::default() {
            input_text = input_text.hint(&self.hint);
        }
        if let Some(auto_select_all) = self.auto_select_all {
            input_text = input_text.auto_select_all(auto_select_all);
        }
        if let Some(allow_tab_input) = self.allow_tab_input {
            input_text = input_text.allow_tab_input(allow_tab_input);
        }
        if let Some(no_horizontal_scroll) = self.no_horizontal_scroll {
            input_text = input_text.no_horizontal_scroll(no_horizontal_scroll);
        }
        if let Some(always_insert_mode) = self.always_insert_mode {
            input_text = input_text.always_insert_mode(always_insert_mode);
        }
        if let Some(read_only) = self.read_only {
            input_text = input_text.read_only(read_only);
        }
        if let Some(password) = self.password {
            input_text = input_text.password(password);
        }
        if let Some(no_undo_redo) = self.no_undo_redo {
            input_text = input_text.no_undo_redo(no_undo_redo);
        }

        let edited = input_text.build();

        if edited {
            *value = match value_str.parse() {
                Ok(value) => value,
                Err(err) => match err.kind() {
                    IntErrorKind::Empty => 0,
                    IntErrorKind::InvalidDigit => *value,
                    IntErrorKind::PosOverflow => usize::MAX,
                    IntErrorKind::NegOverflow => usize::MIN,
                    IntErrorKind::Zero => 0,
                    _ => *value,
                },
            };
        }

        edited
    }
}
