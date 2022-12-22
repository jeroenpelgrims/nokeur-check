use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env::var;
mod history;
mod rates;

fn main() {
    dotenv::dotenv().ok();

    let threshold = std::env::var("THRESHOLD_RATE")
        .expect("THRESHOLD_RATE must be set.")
        .parse::<f64>()
        .expect("Can't parse threshold rate");
    let last = history::last();
    let new = rates::get_nokeur();

    match last {
        None => history::add(new),
        Some(last) => {
            println!("Last rate: {}, new rate: {}", last, new);
            if (last > threshold && new < threshold) || (last < threshold && new < last) {
                history::add(new);
                send_email(new);
            } else if last < threshold && new > threshold {
                history::add(new);
            }
        }
    }
}

fn send_email(nokeur: f64) {
    let smtp_server = var("SMTP_SERVER").expect("Couldn't read smtp server");
    let smtp_user = var("SMTP_USER").expect("Couldn't read smtp user");
    let smtp_password = var("SMTP_PASSWORD").expect("Couldn't read smtp password");
    let smtp_sender = var("SMTP_SENDER").expect("Couldn't read sender");
    let smtp_receiver = var("SMTP_RECEIVER").expect("Couldn't read receiver");
    let email = Message::builder()
        .from(smtp_sender.parse().unwrap())
        .to(smtp_receiver.parse().unwrap())
        .subject(format!("EUR/NOK exchange rate is {}", nokeur))
        .body(String::from("See subject"))
        .unwrap();
    let creds = Credentials::new(smtp_user, smtp_password);
    let mailer = SmtpTransport::relay(smtp_server.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
