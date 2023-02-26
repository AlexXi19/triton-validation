#[macro_export]
macro_rules! unwrap_or_return_error {
    ($value:expr, $error:expr, $message:expr) => {
        match $value {
            Ok(v) => v,
            Err(_) => return $error.body($message),
        }
    };
}

#[macro_export]
macro_rules! unwrap_with_nack {
    ($expr:expr, $channel:expr, $delivery_tag:expr) => {
        match $expr {
            None => {
                $channel
                    .nack_to_dlq($delivery_tag)
                    .await
                    .unwrap_or_else(|e| {
                        error!("Error nacking message to DLQ: {:?}", e);
                    });
                error!(Err(err.into()));
            }
            Some(val) => val,
        }
    };
    ($expr:expr, $channel:expr, $delivery_tag:expr, $error:expr) => {
        match $expr {
            Err(_) => {
                $channel
                    .nack_to_dlq($delivery_tag)
                    .await
                    .unwrap_or_else(|e| {
                        error!("Error nacking message to DLQ: {:?}", e);
                    });
                error!($error);
                continue;
            }
            Ok(val) => val,
        }
    };
}
