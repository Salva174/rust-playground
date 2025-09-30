use std::io::{Read, Stdin};

pub enum InputEvent {
    Up,
    Down,
    Right,
    Left,
    Enter,
    Exit,
    Back,
    Unknown {
        input: Vec<u8>
    },
}

pub fn read_input(stdin: &mut Stdin, buffer: & mut [u8]) -> Result<InputEvent, Box<dyn std::error::Error>> {
    let size = stdin.read(buffer)?;
    let input = &buffer[..size];

    match input {
        &[3] => {
            Ok(InputEvent::Exit)
        }
        &[13] => {
            Ok(InputEvent::Enter)
        }
        &[27, 91, 65] => {
            Ok(InputEvent::Up)
        }
        &[27, 91, 66] => {
            Ok(InputEvent::Down)
        }
        &[27, 91, 67] => {
            Ok(InputEvent::Right)
        }
        &[27, 91, 68] => {
            Ok(InputEvent::Left)
        }
        &[127] => {
            Ok(InputEvent::Back)
        }
        _ => {
            Ok(InputEvent::Unknown {
                input: Vec::from(input)
            })
        }
    }
}