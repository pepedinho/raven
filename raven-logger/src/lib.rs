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
            Protocol::WebSocket => "WS".on_bright_cyan().white().bold(),
            Protocol::Custom => "CSTM".on_yellow().black().bold(),
        }
    }
}

pub enum Channel {
    Stdout,
    Stderr,
    File(String),
}

pub enum MatchError {
    MethodMismatch {
        expected: String,
        found: String,
    },
    QueryMismatch {
        key: String,
        expected: String,
        found: String,
    },
    HeaderMismatch {
        key: String,
        expected: String,
        found: String,
    },
    BodyMismatch {
        expected: String,
        found: String,
    },
}

impl MatchError {
    pub fn label(&self) -> &'static str {
        match self {
            MatchError::MethodMismatch { .. } => "METHOD",
            MatchError::QueryMismatch { .. } => "QUERY",
            MatchError::HeaderMismatch { .. } => "HEADER",
            MatchError::BodyMismatch { .. } => "BODY",
        }
    }

    pub fn diff_lines(&self) -> Vec<String> {
        match self {
            MatchError::MethodMismatch { expected, found } => vec![
                format!("expected: {}", expected.green()),
                format!("found   : {}", found.red()),
            ],

            MatchError::QueryMismatch {
                key,
                expected,
                found,
            } => vec![
                format!("expected: {} = {}", key, expected.green()),
                format!(
                    "found   : {}",
                    if found.is_empty() {
                        "<missing>".red()
                    } else {
                        found.red()
                    }
                ),
            ],

            MatchError::HeaderMismatch {
                key,
                expected,
                found,
            } => vec![
                format!("expected: {} = {}", key, expected.green()),
                format!(
                    "found   : {}",
                    if found.is_empty() {
                        "<missing>".red()
                    } else {
                        found.red()
                    }
                ),
            ],

            MatchError::BodyMismatch { expected, found } => vec![
                format!("expected: {}", expected.green()),
                format!("found   : {}", found.red()),
            ],
        }
    }
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
            "{:^8} {:<20}: {}",
            protocol.style(),
            format!("[{}]", sender.bold()),
            msg
        );
        Self::dispatch(&formated);
    }

    pub fn slog(author: &str, msg: &str) {
        let formated = format!("{:^8} {}", author.on_purple().white().bold(), msg);

        Self::dispatch(&formated);
    }

    pub fn mismatch(protocol: Protocol, sender: &str, kind: &MatchError) {
        let header = format!(
            "{:^8} {:<20} {}",
            protocol.style(),
            format!("[{}]", sender.bold()),
            kind.label().on_red().white().bold()
        );

        let header_width = strip_ainsi(&header).len() - 1;

        let indent = " ".repeat(header_width);

        let diff_lines = kind.diff_lines();

        let mut out = String::new();
        out.push_str(&header);
        out.push('\n');

        for (i, line) in diff_lines.iter().enumerate() {
            let branch = if i + 1 == diff_lines.len() {
                "└── "
            } else {
                "├── "
            };

            out.push_str(&indent);
            out.push_str(branch);
            out.push_str(&format!("{}", line.bold()));
            out.push('\n');
        }

        Self::dispatch(out.trim_end());
    }
}

fn strip_ainsi(s: &str) -> String {
    let mut res = String::new();
    let mut chars = s.chars();

    let iter = chars.by_ref();
    while let Some(c) = iter.next() {
        if c == '\x1b' {
            for c in iter.by_ref() {
                if c == 'm' {
                    break;
                }
            }
        } else {
            res.push(c);
        }
    }
    res
}
