use std::num::IntErrorKind;

use imgui::{InputText, Ui};

pub fn input_usize(ui: &Ui, label: &str, value: &mut usize) -> bool {
    let mut value_str = value.to_string();
    let edited = InputText::new(ui, label, &mut value_str)
        .chars_decimal(true)
        .chars_noblank(true)
        .build();

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
