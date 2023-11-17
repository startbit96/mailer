use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let smtp_server = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("smtp.gmail.com"));
    let smtp_username = std::env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("timschwarzbrunn2@googlemail.com"));
    let smtp_password = std::env::args().nth(3).unwrap_or_else(|| {
        Command::new("pass")
            .arg("show")
            .arg("google/mail")
            .arg("-c")
            .output()
            .expect("Failed to execute pass command.");

        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
        if let Ok(contents) = clipboard.get_contents() {
            String::from(contents)
        } else {
            println!("Cannot obtain password from the clipboard. Clipboard empty.");
            String::from("no password given")
        }
    });

    Ok(())
}
