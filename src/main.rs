use clipboard::{ClipboardContext, ClipboardProvider};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{message::Mailbox, Address, Message, SmtpTransport, Transport};
use std::error::Error;
use std::fs::File;
use std::process::Command;
use termion::color;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the informations about the mail server, username and password.
    // Default is from the password-store.
    let smtp_server = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("smtp.gmail.com"));
    let smtp_username = std::env::args().nth(2).unwrap_or_else(|| {
        Command::new("pass")
            .arg("show")
            .arg("google/mailaddress")
            .arg("-c")
            .output()
            .expect("Failed to execute pass command.");

        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
        if let Ok(contents) = clipboard.get_contents() {
            String::from(contents.trim())
        } else {
            println!("Cannot obtain email address from the clipboard. Clipboard empty.");
            String::from("no email address given")
        }
    });
    // Note that if you use googlemail, you need to create an app-password.
    // https://security.google.com/settings/security/apppasswords
    // Maybe you need to enable 2FA before.
    let smtp_password = std::env::args().nth(3).unwrap_or_else(|| {
        Command::new("pass")
            .arg("show")
            .arg("google/rust-mailer")
            .arg("-c")
            .output()
            .expect("Failed to execute pass command.");

        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
        if let Ok(contents) = clipboard.get_contents() {
            String::from(contents.trim())
        } else {
            println!("Cannot obtain password from the clipboard. Clipboard empty.");
            String::from("no password given")
        }
    });
    let filename_receivers = std::env::args()
        .nth(4)
        .unwrap_or_else(|| String::from("./contacts/test.csv"));

    // Parse the sender.
    println!("Try to parse given email address {}", smtp_username);
    let sender = smtp_username.parse::<Address>()?;

    // Load the list of the recipients.
    let file = File::open(filename_receivers)?;
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    for result in reader.records() {
        let record = result?;
        let salutation = &record[0].trim();
        let email_address = &record[1].trim();

        // Compose the email.
        let email_subject = "Subject";
        let email_body = format!("{},\n\nThis is the body of your email.", salutation);

        // Parse the receiver.
        let receiver;
        if let Ok(_receiver) = email_address.parse::<Address>() {
            receiver = _receiver;
        } else {
            println!(
                "{}Cannot send email to {}. Check the email address.{}",
                color::Fg(color::Red),
                email_address,
                color::Fg(color::Black)
            );
            continue;
        }

        // Create the mail with sender and receiver.
        let email = Message::builder()
            .from(Mailbox::new(None, sender.clone()))
            .to(Mailbox::new(None, receiver))
            .subject(email_subject)
            .body(email_body)?;

        // Create the credentials.
        let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());
        let mailer = SmtpTransport::relay(smtp_server.as_str())?
            .credentials(creds)
            .build();

        // Send the mail.
        match mailer.send(&email) {
            Ok(_) => println!(
                "{}Email sent to {}{}",
                color::Fg(color::Green),
                email_address,
                color::Fg(color::Black)
            ),
            Err(e) => eprintln!(
                "{}Error sending email to {}: {}{}",
                color::Fg(color::Red),
                email_address,
                e,
                color::Fg(color::Black)
            ),
        }
    }

    println!("\nDone.");
    Ok(())
}
