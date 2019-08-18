extern crate clipboard;
use clipboard::{ClipboardContext, ClipboardProvider};

/// Copies a string to a clipboard.
pub fn copy_to_clipboard(value: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(value.to_owned()).unwrap();
}
