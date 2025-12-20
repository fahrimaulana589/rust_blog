use crate::config::Config;
use crate::app::features::auth::domain::entity::User;
use lettre::message::{Mailbox, header};
use lettre::{Message, SmtpTransport, Transport};


#[derive(Clone)]
pub struct Email {
    pub config: Config,
}

impl Email {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Email {
    pub fn send_test_email(&self) -> Result<String, String> {
        let email = Message::builder()
            .from(Mailbox::new(None, self.config.smtp_from.parse().unwrap()))
            .to("user@test.com".parse().unwrap())
            .subject("Email HTML")
            .header(header::ContentType::TEXT_HTML)
            .body(String::from(
                "<h1>Hello</h1><p>Email HTML dari <b>Rust</b></p>",
            ))
            .unwrap();
        self.send_email(email)
    }

    pub fn send_email_to_user(&self, user: User, subject: String, body: String) -> Result<String, String> {
        let email = Message::builder()
            .from(Mailbox::new(None, self.config.smtp_from.parse().unwrap()))
            .to(user.email.parse().unwrap())
            .subject(subject)
            .header(header::ContentType::TEXT_HTML)
            .body(String::from(
                body,
            ))
            .unwrap();
        self.send_email(email)
    }

    fn send_email(&self,email: Message) -> Result<String, String> {
        let mailer = SmtpTransport::builder_dangerous(self.config.smtp_host.clone())
            .port(self.config.smtp_port as u16)
            .build();

        mailer.send(&email).unwrap();
        Ok("Email sent successfully".to_string())
    }
}
