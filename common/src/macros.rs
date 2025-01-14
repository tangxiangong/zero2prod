#[macro_export]
macro_rules! impl_error_from_extract_rejection {
    ($($rejection:ty),+ $(,)?) => {
        $(
            impl From<$rejection> for AppError {
                fn from(error: $rejection) -> Self {
                    Self {
                        status_code: error.status(),
                        message: error.to_string(),
                    }
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_error_from_server_error {
    ($($error:ty),+ $(,)?) => {
        $(
            impl From<$error> for AppError {
                fn from(e: $error) -> Self {
                    Self {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: e.to_string(),
                    }
                }
            }
        )+
    };
}
