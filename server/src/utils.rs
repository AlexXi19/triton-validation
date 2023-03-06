#[macro_export]
macro_rules! unwrap_or_return_error {
    ($value:expr, $error:expr, $message:expr) => {
        match $value {
            Ok(v) => v,
            Err(e) => {
                return {
                    let err = format!("{}: {:?}", $message, e);
                    error!(err);
                    $error.body(err)
                }
            }
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
            Err(e) => {
                $channel
                    .nack_to_dlq($delivery_tag)
                    .await
                    .unwrap_or_else(|e| {
                        error!("Error nacking message to DLQ: {:?}", e);
                    });
                error!("{:?}: {:?}", $error, e);
                continue;
            }
            Ok(val) => val,
        }
    };
}
