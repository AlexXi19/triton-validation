#[macro_export]
macro_rules! unwrap_or_return_error {
    ($value:expr, $error:expr, $message:expr) => {
        match $value {
            Ok(v) => v,
            Err(_) => return $error.body($message),
        }
    };
}
