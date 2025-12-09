use colored::{ColoredString, Colorize};
use std::io::Write;
use std::{fs::OpenOptions, sync::Mutex};

pub mod macros;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[derive(Clone, Copy)]
pub enum Protocol {
    Http,
    WebSocket,
    Custom,
}

impl Protocol {
    pub fn style(&self) -> ColoredString {
        match self {
            Protocol::Http => "HTTP".on_blue().white().bold(),
            Protocol::WebSocket => "WS".on_blue().white().bold(),
            Protocol::Custom => "CSTM".on_yellow().black().bold(),
        }
    }
}

pub enum Channel {
    Stdout,
    Stderr,
    File(String),
}

pub struct Logger {
    channel: Channel,
}

lazy_static::lazy_static! {
    static ref GLOBAL_LOGGER: Mutex<Logger> = Mutex::new(Logger::new(Channel::Stdout));
}

impl Logger {
    pub fn new(channel: Channel) -> Self {
        Logger { channel }
    }

    pub fn set_channel(channel: Channel) {
        let mut logger = GLOBAL_LOGGER.lock().unwrap();
        logger.channel = channel;
    }

    fn dispatch(log: &str) {
        let logger = GLOBAL_LOGGER.lock().unwrap();
        match &logger.channel {
            Channel::Stdout => println!("{log}"),
            Channel::Stderr => eprintln!("{log}"),
            Channel::File(path) => {
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                    writeln!(file, "{log}").ok();
                }
            }
        }
    }

    pub fn log(protocol: Protocol, sender: &str, msg: &str) {
        let formated = format!(
            "{} {}: {}",
            protocol.style(),
            format!("[{}]", sender.bold()),
            msg
        );
        Self::dispatch(&formated);
    }

    pub fn slog(author: &str, msg: &str) {
        let formated = format!("{} {}", author.on_purple().white().bold(), msg);

        Self::dispatch(&formated);
    }
}
