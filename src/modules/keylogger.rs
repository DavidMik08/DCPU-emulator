use std::collections::VecDeque;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct KeyboardController {
    fifo: VecDeque<u8>,
}

impl KeyboardController {
    fn new() -> Self {
        Self {
            fifo: VecDeque::with_capacity(16),
        }
    }

    fn push_key(&mut self, ascii: u8, pressed: bool) {
        let value = if pressed { ascii | 0x80 } else { ascii & 0x7F };
        if self.fifo.len() < 16 {
            self.fifo.push_back(value);
        }
    }

    fn read_data(&mut self) -> Option<u8> {
        self.fifo.pop_front()
    }

    fn status(&self) -> bool {
        !self.fifo.is_empty()
    }
}

fn key_to_ascii(key: VirtualKeyCode) -> Option<u8> {
    match key {
        VirtualKeyCode::A => Some(b'A'),
        VirtualKeyCode::B => Some(b'B'),
        VirtualKeyCode::C => Some(b'C'),
        VirtualKeyCode::D => Some(b'D'),
        VirtualKeyCode::E => Some(b'E'),
        VirtualKeyCode::F => Some(b'F'),
        VirtualKeyCode::G => Some(b'G'),
        VirtualKeyCode::H => Some(b'H'),
        VirtualKeyCode::I => Some(b'I'),
        VirtualKeyCode::J => Some(b'J'),
        VirtualKeyCode::K => Some(b'K'),
        VirtualKeyCode::L => Some(b'L'),
        VirtualKeyCode::M => Some(b'M'),
        VirtualKeyCode::N => Some(b'N'),
        VirtualKeyCode::O => Some(b'O'),
        VirtualKeyCode::P => Some(b'P'),
        VirtualKeyCode::Q => Some(b'Q'),
        VirtualKeyCode::R => Some(b'R'),
        VirtualKeyCode::S => Some(b'S'),
        VirtualKeyCode::T => Some(b'T'),
        VirtualKeyCode::U => Some(b'U'),
        VirtualKeyCode::V => Some(b'V'),
        VirtualKeyCode::W => Some(b'W'),
        VirtualKeyCode::X => Some(b'X'),
        VirtualKeyCode::Y => Some(b'Y'),
        VirtualKeyCode::Z => Some(b'Z'),
        VirtualKeyCode::Space => Some(b' '),
        VirtualKeyCode::Return => Some(b'\n'),
        VirtualKeyCode::Key1 => Some(b'1'),
        VirtualKeyCode::Key2 => Some(b'2'),
        VirtualKeyCode::Key3 => Some(b'3'),
        VirtualKeyCode::Key4 => Some(b'4'),
        VirtualKeyCode::Key5 => Some(b'5'),
        VirtualKeyCode::Key6 => Some(b'6'),
        VirtualKeyCode::Key7 => Some(b'7'),
        VirtualKeyCode::Key8 => Some(b'8'),
        VirtualKeyCode::Key9 => Some(b'9'),
        VirtualKeyCode::Key0 => Some(b'0'),
        _ => None,
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .with_title("8-bit Keyboard Input")
        .build(&event_loop)
        .unwrap();

    let mut kb = KeyboardController::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(key),
                            state,
                            ..
                        },
                    ..
                } => {
                    if let Some(ascii) = key_to_ascii(key) {
                        kb.push_key(ascii, state == ElementState::Pressed);
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                while kb.status() {
                    if let Some(value) = kb.read_data() {
                        println!("Keyboard event: 0x{:02X}", value);
                    }
                }
            }
            _ => {}
        }
    });
}

