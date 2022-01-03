pub mod fps;
pub mod frametime;

use std::num::NonZeroU32;

use hltas::{
    types::{Button, FrameBulk, Line},
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
    if lines.is_empty() {
        return String::new();
    }

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

pub fn str_to_lines(lines: &str) -> Option<Vec<Line>> {
    let version1_text = "version 1";

    if lines.len() < version1_text.len() {
        return None;
    }

    let lines = if lines[..version1_text.len()].starts_with(version1_text) {
        // parse as whole hltas file
        lines.to_string()
    } else {
        // parse as hltas lines
        format!("version 1\nframes\n{}", lines)
    };

    match HLTAS::from_str(&lines) {
        Ok(hltas) => Some(hltas.lines),
        Err(_) => None,
    }
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

pub fn empty_framebulk(frametime: &str, frame_count: NonZeroU32) -> FrameBulk {
    FrameBulk {
        auto_actions: hltas::types::AutoActions {
            movement: None,
            leave_ground_action: None,
            jump_bug: None,
            duck_before_collision: None,
            duck_before_ground: None,
            duck_when_jump: None,
        },
        movement_keys: hltas::types::MovementKeys {
            forward: false,
            left: false,
            right: false,
            back: false,
            up: false,
            down: false,
        },
        action_keys: hltas::types::ActionKeys {
            jump: false,
            duck: false,
            use_: false,
            attack_1: false,
            attack_2: false,
            reload: false,
        },
        frame_time: frametime.to_string(),
        pitch: None,
        frame_count,
        console_command: None,
    }
}
