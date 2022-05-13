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
                enigo.key_down(Key::Layout(c));
                enigo.key_up(Key::Layout(c));
            }
        }
    }
}
