use pc_keyboard::layouts::Us104Key;
use pc_keyboard::HandleControl::Ignore;
use pc_keyboard::{DecodedKey, KeyCode, KeyEvent, KeyState, Keyboard, ScancodeSet1};
use spin::{Lazy, Mutex};
use x86_64::instructions::interrupts::without_interrupts;

use crate::interrupt::interrupt_handler::STDIN_BUFFER;
use crate::renderer::text_renderer::TEXT_RENDERER;
use crate::{print, serial_print};

pub fn execute(scancode: u8) {
    static KEYBOARD: Lazy<Mutex<Keyboard<Us104Key, ScancodeSet1>>> = Lazy::new(|| {
        let keyboard = Keyboard::new(ScancodeSet1::new(), Us104Key, Ignore);
        Mutex::new(keyboard)
    });

    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(event)) = keyboard.add_byte(scancode) {
        // println!("Event tirgger: {:?}", event);
        match event {
            KeyEvent {
                code: KeyCode::Delete,
                state: KeyState::Down,
            } => {}
            KeyEvent {
                code: KeyCode::Backspace,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXT_RENDERER.get() {
                    without_interrupts(|| writer.lock().cursor_left())
                }
            }
            KeyEvent {
                code: KeyCode::ArrowLeft,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXT_RENDERER.get() {
                    without_interrupts(|| writer.lock().cursor_left())
                }
            }
            KeyEvent {
                code: KeyCode::ArrowRight,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXT_RENDERER.get() {
                    without_interrupts(|| writer.lock().cursor_right())
                }
            }
            KeyEvent {
                code: KeyCode::ArrowUp,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXT_RENDERER.get() {
                    without_interrupts(|| writer.lock().cursor_up())
                }
            }
            KeyEvent {
                code: KeyCode::ArrowDown,
                state: KeyState::Down,
            } => {
                if let Some(writer) = TEXT_RENDERER.get() {
                    without_interrupts(|| writer.lock().cursor_down())
                }
            }
            _ => {
                if let Some(key) = keyboard.process_keyevent(event) {
                    // println!("Key: {:?}", key);
                    match key {
                        DecodedKey::Unicode(character) => {
                            if character.is_ascii() {
                                print!("{:}", character);
                                serial_print!("{:}", character);
                                STDIN_BUFFER.lock().push_back(character as u8);
                            } else {
                                print!("{:?}", key);
                                serial_print!("{:?}", key);
                            }
                        }
                        DecodedKey::RawKey(key) => {
                            // print!("{:?}", key);
                            serial_print!("{:?}", key);
                        }
                    }
                }
            }
        }
    }
}
