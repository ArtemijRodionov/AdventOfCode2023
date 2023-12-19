#[macro_export]
macro_rules! dbg {
    ($($rest:tt)*) => {
        if std::env::var("NODBG").is_err() {
            std::dbg!($($rest)*);
        }
    }
}
