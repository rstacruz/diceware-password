extern crate clipboard;

use clipboard::{ClipboardContext, ClipboardProvider};
use std::boxed::Box;
use std::io::Write;
use std::process::{Command, Stdio};

/// Copies a string to a clipboard.
pub fn copy_to_clipboard(value: &str) {
    match copy_via_xsel(value) {
        Err(_) => match copy_via_clipboard_rs(value) {
            Err(_) => (),
            Ok(_) => (),
        },
        Ok(_) => (),
    };
}

/// Copy via the 'xsel' utility, if present.
/// This is preferrable to `clipboard` or `x11-clipboard` because it won't clear the clipboard
/// after the program exits.
pub fn copy_via_xsel(value: &str) -> Result<(), std::io::Error> {
    let proc = Command::new("xsel")
        .stdin(Stdio::piped())
        .arg("-b")
        .spawn()?;

    proc.stdin.unwrap().write_all(value.as_bytes())
}

/// Copy via the 'xsel' utility, if present.
pub fn copy_via_clipboard_rs(value: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(value.to_owned())
}
