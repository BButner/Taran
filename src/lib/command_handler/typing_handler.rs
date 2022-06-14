use enigo::{Enigo, Key, KeyboardControllable};
use crate::lib::config::Typing;

enum TypingMode {
    // Windows
    KeyDownUp,
    // macOS
    KeySequence,
}

pub fn handle_typing_macro(cmd: &Typing) {
    type_macro(&cmd.text);
}

pub fn type_macro(text: &String) {
    let mode: TypingMode = if cfg!(target_os = "windows") {
        TypingMode::KeyDownUp
    } else {
        TypingMode::KeySequence
    };

    let mut enigo = Enigo::new();

    match mode {
        TypingMode::KeySequence => enigo.key_sequence(text),
        TypingMode::KeyDownUp => {
            for c in text.chars() {
                if c.is_uppercase() { enigo.key_down(Key::Shift) }

                let lowercase = c.to_lowercase().nth(0).unwrap();

                enigo.key_down(Key::Layout(lowercase));
                enigo.key_up(Key::Layout(lowercase));
                if c.is_uppercase() { enigo.key_up(Key::Shift) }
            }
        }
    }
}
