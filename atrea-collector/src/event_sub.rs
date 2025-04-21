use crate::MyError;
use openssl::ssl::{SslConnector, SslMethod, SslStream};
use std::net::TcpStream;
use tungstenite::{connect, protocol::Message};

//const TWITCH_WWS_DOMAIN: &str = "eventsub.wss.twitch.tv";
//const TWITCH_WWS_PORT: usize = 443;
const TWITCH_WWS_URL: &str = "wss://eventsub.wss.twitch.tv/ws?keepalive_timeout_seconds=30";

pub fn init_connection() -> Result<(), MyError> {
    //let tcp_stream = TcpStream::connect(format!("{TWITCH_WWS_DOMAIN}:{TWITCH_WWS_PORT}"))?;
    //let ssl_connector: SslConnector = SslConnector::builder(SslMethod::tls_client())?.build();
    //let mut stream: SslStream<TcpStream> = ssl_connector.connect(TWITCH_WWS_DOMAIN, tcp_stream)?;

    //let url: Url = Url::parse(TWITCH_WWS_URL)?;
    let (mut socket, response) = tungstenite::client::connect(TWITCH_WWS_URL)?;
    // TODO: ssl
    // TODO: implement

    //socket.send()

    todo!()
}

pub fn main(settings: Settings, channels: Vec<String>) -> Result<(), MyError> {
    todo!()
}
