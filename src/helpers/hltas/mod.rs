pub mod fps;

use hltas::HLTAS;

pub fn hltas_to_str(hltas: &HLTAS) -> String {
    let mut file_u8: Vec<u8> = Vec::new();
    hltas.to_writer(&mut file_u8).unwrap();

    if let Ok(content) = String::from_utf8(file_u8) {
        return content;
    }
    // shouldn't be possible for this to happen
    panic!("unable to convert hltas to string");
}
