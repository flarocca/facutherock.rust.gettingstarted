struct SendEmailCommand {
    to: String,
    subject: String,
    body: String
}

impl SendEmailCommand {
    fn new(to: String, subject: String, body: String) -> SendEmailCommand {
        SendEmailCommand {
            to,
            subject,
            body
        }
    }
}

impl Command for SendEmailCommand {}