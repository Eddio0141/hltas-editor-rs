pub mod fps;
pub mod frametime;

use hltas::{types::Button, HLTAS};

pub fn hltas_to_str(hltas: &HLTAS) -> String {
    let mut file_u8: Vec<u8> = Vec::new();
    hltas.to_writer(&mut file_u8).unwrap();

    if let Ok(content) = String::from_utf8(file_u8) {
        return content;
    }
    // shouldn't be possible for this to happen
    panic!("unable to convert hltas to string");
}

pub fn button_to_str(button: &Button) -> &str {
    match button {
        Button::Forward => "forward",
        Button::ForwardLeft => "forward left",
        Button::Left => "left",
        Button::BackLeft => "back left",
        Button::Back => "back",
        Button::BackRight => "back right",
        Button::Right => "right",
        Button::ForwardRight => "forward right",
    }
}
