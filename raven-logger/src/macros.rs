#[macro_export]
macro_rules! log {
    ($protocol: expr, $sender:expr, $($arg:tt)*) => {
        $crate::Logger::log($protocol, $sender, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! slog {
    ($author: expr, $($arg:tt)*) => {
        $crate::Logger::slog($author, &format!($($arg)*))
    };
}
