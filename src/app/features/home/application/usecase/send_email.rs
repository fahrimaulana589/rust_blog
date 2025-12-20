use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, Mailbox};

#[derive(Clone)]
pub struct Execute {}

impl Execute {
    pub fn new() -> Self {
        Self {}
    }

    pub fn send(&self) -> Result<(), String> {
        let email = Message::builder()
            .from(Mailbox::new(None, "no-reply@example.com".parse().unwrap()))
            .to("user@test.com".parse().unwrap())
            .subject("Email HTML")
            .header(header::ContentType::TEXT_HTML)
            .body(String::from(
                "<h1>Hello</h1><p>Email HTML dari <b>Rust</b></p>",
            ))
            .unwrap();

        let mailer = SmtpTransport::builder_dangerous("localhost")
            .port(1025)
            .build();

        mailer.send(&email).unwrap();

        Ok(())
    }
}
