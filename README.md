# mailer
Send personalized mails automatically to a list of people.

## Usage

`cargo run -- <smtp-server> <smtp-username> <smtp-password> <path-to-csv>`

Example:

`cargo run -- smtp.gmail.com your-own-email@googlemail.com 1234567890 ./contacts/contact-list.csv`

## Notes

When you are using googlemail, you will need to create an [app-password](https://security.google.com/settings/security/apppasswords) and use this password instead of your normal password. If you cannot create one, you may first activate 2FA within your account.

## To-Do

- Make the subject and body of the mail customizable through files or the command line.
