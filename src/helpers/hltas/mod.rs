pub mod fps;
pub mod frametime;

use hltas::{
    types::{Button, Line},
    HLTAS,
};

pub fn hltas_to_str(hltas: &HLTAS) -> String {
    let mut file_u8: Vec<u8> = Vec::new();
    hltas.to_writer(&mut file_u8).unwrap();

    if let Ok(content) = String::from_utf8(file_u8) {
        return content;
    }
    // shouldn't be possible for this to happen
    panic!("unable to convert hltas to string");
}

pub fn lines_to_str(lines: Vec<Line>) -> String {
    // make a dummy hltas
    let hltas = HLTAS {
        lines,
        ..Default::default()
    };

    let hltas = hltas_to_str(&hltas);

    let header_lines = "version 1\nframes\n";

    // just in case the hltas default format changes, I assert this
    if let Some(index) = hltas.find(header_lines) {
        assert_eq!(index, 0, "assertion failed with {} == 0, probably HLTAS default format changed and function `lines_to_str` needs an update.", index);
    } else {
        panic!("No match found for version 1\\nframes\\n. Probably HLTAS default format changed and function `lines_to_str` needs an update.");
    }

    hltas[header_lines.len()..hltas.len() - 1].to_string()
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
