pub enum MyError {
    IoErr(std::io::Error),
    SslErrStack(openssl::error::ErrorStack),
    SslHandshakeTcpError(openssl::ssl::HandshakeError<std::net::TcpStream>),
    #[cfg(feature = "eventsub")]
    TungsteniteError(tungstenite::Error),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::IoErr(error) => error.fmt(f),
            MyError::SslErrStack(error_stack) => error_stack.fmt(f),
            MyError::SslHandshakeTcpError(handshake_error) => handshake_error.fmt(f),
            #[cfg(feature = "eventsub")]
            MyError::TungsteniteError(error) => error.fmt(f),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::IoErr(value)
    }
}

impl From<openssl::error::ErrorStack> for MyError {
    fn from(value: openssl::error::ErrorStack) -> Self {
        Self::SslErrStack(value)
    }
}

impl From<openssl::ssl::HandshakeError<std::net::TcpStream>> for MyError {
    fn from(value: openssl::ssl::HandshakeError<std::net::TcpStream>) -> Self {
        Self::SslHandshakeTcpError(value)
    }
}

#[cfg(feature = "eventsub")]
impl From<tungstenite::Error> for MyError {
    fn from(value: tungstenite::Error) -> Self {
        Self::TungsteniteError(value)
    }
}
