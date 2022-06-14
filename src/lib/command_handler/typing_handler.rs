use tfc::{Context, Key, KeyboardContext, UnicodeKeyboardContext};
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
    let mut context = Context::new();

    if context.is_ok() {
        let mut ctx = context.unwrap();

        let chars = text.chars();
        let mut index_to_skip: Option<usize> = None;

        for (index, c) in chars.enumerate() {
            if index_to_skip.is_some() && index == index_to_skip.unwrap() {
                index_to_skip = None;
                continue;
            }

            if c == '\\' && text.len() > (index + 1) {
                if text.chars().nth(index + 1).unwrap() == 'n' {
                    ctx.key_click(Key::ReturnOrEnter);
                    index_to_skip = Some(index + 1);
                    continue;
                }
            }
            ctx.unicode_char(c);
        }
    }
}
