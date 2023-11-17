use clipboard::{ClipboardContext, ClipboardProvider};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{message::Mailbox, Address, Message, SmtpTransport, Transport};
use std::error::Error;
use std::fs::File;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the informations about the mail server, username and password.
    // Default is from the password-store.
    let smtp_server = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("smtp.gmail.com"));
    let smtp_username = std::env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("timschwarzbrunn2@gmail.com"));
    // Note that if you use googlemail, you need to create an app-password.
    // https://security.google.com/settings/security/apppasswords
    // Maybe you need to enable 2FA before.
    let smtp_password = std::env::args().nth(3).unwrap_or_else(|| {
        Command::new("pass")
            .arg("show")
            .arg("google/mailer")
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

    // Load the list of the recipients.
    let file = File::open("./contacts/test.csv")?;
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    for result in reader.records() {
        let record = result?;
        let salutation = &record[0].trim();
        let email_address = &record[1].trim();

        // Compose the email.
        let email_subject = "Buchreihe XYZ";
        let email_body = format!("{},\n\nThis is the body of your email.", salutation);

        // Send the email.
        let email = Message::builder()
            .from(Mailbox::new(None, smtp_username.parse::<Address>()?))
            .to(Mailbox::new(None, email_address.parse::<Address>()?))
            .subject(email_subject)
            .body(email_body)?;

        let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());
        let mailer = SmtpTransport::relay(smtp_server.as_str())?
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent to {}", email_address),
            Err(e) => eprintln!("Error sending email to {}: {}", email_address, e),
        }
    }

    Ok(())
}
