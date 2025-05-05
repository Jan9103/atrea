use core::str;
use openssl::ssl::{SslConnector, SslMethod, SslStream};
use std::{
    collections::HashMap,
    io::prelude::{Read, Write},
    net::TcpStream,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{LogStreams, MyError, Settings};

const TWITCH_IRC_SERVER: &str = "irc.chat.twitch.tv:6697";
const TWITCH_IRC_DOMAIN: &str = "irc.chat.twitch.tv";
const BOT_NAME: &str = "justinfan007";
const BOT_PASS: &str = "oauth:hi";

fn init_anon_irc(
    settings: &Settings,
    channels: &Vec<String>,
) -> Result<SslStream<TcpStream>, MyError> {
    let tcp_stream = TcpStream::connect(TWITCH_IRC_SERVER.to_string())?;
    let ssl_connector: SslConnector = SslConnector::builder(SslMethod::tls_client())?.build();
    let mut stream: SslStream<TcpStream> = ssl_connector.connect(TWITCH_IRC_DOMAIN, tcp_stream)?;
    send_irc_cmd(settings, &mut stream, "PASS", BOT_PASS.into())?;
    send_irc_cmd(settings, &mut stream, "NICK", BOT_NAME.into())?;
    send_irc_cmd(
        settings,
        &mut stream,
        "CAP REQ",
        ":twitch.tv/commands".into(),
    )?;
    send_irc_cmd(
        settings,
        &mut stream,
        "CAP REQ",
        ":twitch.tv/membership".into(),
    )?;
    send_irc_cmd(settings, &mut stream, "CAP REQ", ":twitch.tv/tags".into())?;
    for channel_name in channels {
        send_irc_cmd(settings, &mut stream, "JOIN", format!("#{channel_name}"))?;
    }
    Ok(stream)
}

fn send_irc_cmd(
    settings: &Settings,
    //stream: &mut SslStream<TcpStream>,
    stream: &mut dyn Write,
    cmd: &str,
    msg: String,
) -> Result<(), MyError> {
    let mut s = cmd.to_string();
    s.push(' ');
    s.push_str(&msg);
    if settings.stdout_log_sent {
        match cmd {
            "PONG" => {}
            "PASS" => {
                println!(
                    "{} >>> PASS [CENSORED]",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                );
            }
            _ => {
                println!(
                    "{} >>> {}",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    s.trim()
                );
            }
        }
    }
    s.push_str("\r\n");
    stream.write_all(s.as_bytes())?;
    Ok(())
}

fn recieve_irc(
    settings: &Settings,
    stream: &mut SslStream<TcpStream>,
    logs: &mut LogStreams,
) -> Result<(), MyError> {
    let mut buf: Vec<u8>;
    let mut tmp = [1];
    loop {
        buf = Vec::new();
        loop {
            stream.read_exact(&mut tmp)?;
            match tmp[0] {
                0x1 => continue, // heading
                0xD => continue, // CR
                0xA => break,    // LF (end of irc command)
                _ => buf.push(tmp[0]),
            }
        }
        let res_str: &str =
            str::from_utf8(&buf[..]).expect("Twitch sent a non-utf8 string via irc");
        if res_str.is_empty() {
            continue;
        }
        match handle_incomming_irc(settings, stream, res_str, logs)? {
            IrcMessageHandleResult::Ok => {}
            IrcMessageHandleResult::RestartIrc => {
                send_irc_cmd(settings, stream, "QUIT", "".to_string())?;
                return Ok(());
            }
        }
    }
}

enum IrcMessageHandleResult {
    Ok,
    RestartIrc,
}

fn handle_incomming_irc(
    settings: &Settings,
    stream: &mut SslStream<TcpStream>,
    msg: &str,
    logs: &mut LogStreams,
) -> Result<IrcMessageHandleResult, MyError> {
    if settings.stdout_log_recieved {
        println!(
            "{} <<< {}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            msg
        );
    }
    let mut args: Vec<&str> = msg.trim().split(' ').collect();
    let tags: HashMap<&str, &str> = if args.first().unwrap_or(&"").starts_with("@") {
        args.remove(0)
            .strip_prefix("@")
            .unwrap_or("")
            .split(";")
            .map(|item| item.split_once("=").unwrap_or(("", "")))
            .collect()
    } else {
        HashMap::new()
    };
    let author: &str = if args.first().unwrap_or(&"").starts_with(":") {
        args.remove(0)
            .strip_prefix(":")
            .unwrap_or("")
            .split_once("!")
            .unwrap_or(("NO_AUTHOR", ""))
            .0
    } else {
        ""
    };
    if args.is_empty() {
        eprintln!(
            "ERR (irc.handle_incomming_message): missing index irc_cmd: {}",
            msg
        );
        return Ok(IrcMessageHandleResult::Ok);
    };
    let irc_cmd: &str = args.remove(0);
    // if irc_cmd != "PING" {
    //     println!(
    //         "{} <<< {}",
    //         SystemTime::now()
    //             .duration_since(UNIX_EPOCH)
    //             .unwrap()
    //             .as_secs(),
    //         msg,
    //     );
    // }
    match irc_cmd {
        "PING" => send_irc_cmd(
            settings,
            stream,
            "PONG",
            args.first().unwrap_or(&"").to_string(),
        )?,
        "PRIVMSG" => {
            if !settings.log_shoutouts {
                return Ok(IrcMessageHandleResult::Ok);
            }

            if tags.get("mod") == Some(&"1")
                || tags.get("badges").unwrap_or(&"").starts_with("broadcaster")
            {
                let room: &str = args.remove(0).strip_prefix("#").unwrap_or("NO_ROOM");
                args[0] = args.first().unwrap_or(&"").strip_prefix(":").unwrap_or("");
                match *args.first().unwrap_or(&"") {
                    "!so" | "!shoutout" => {
                        if let Some(target) = args.get(1) {
                            // the login is just the display name in lowercase in 99% of cases
                            let target = target.to_lowercase();
                            // only allow valid usernames (some shoutout by twitter-url, name, (at)twitter_handle, etc)
                            if target.chars().all(|c| c.is_ascii_alphanumeric()) && target.len() > 3
                            {
                                logs.log_shoutout(room, target.as_str())?;
                                if settings.follow_shoutouts {
                                    send_irc_cmd(settings, stream, "JOIN", format!("#{}", target))?;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        "RECONNECT" => {
            return Ok(IrcMessageHandleResult::RestartIrc);
        }
        "JOIN" => {
            if author != BOT_NAME && settings.log_joins {
                let room: &str = args.remove(0).strip_prefix("#").unwrap_or("NO_ROOM");
                logs.log_join(author, room)?;
            }
        }
        "USERNOTICE" => {
            let room: &str = args.remove(0).strip_prefix("#").unwrap_or("NO_ROOM");
            if matches!(tags.get("msg-id"), Some(&"raid")) {
                let raid_size = if let Some(vc) = tags.get("msg-param-viewerCount") {
                    (*vc).parse::<usize>().unwrap_or(0)
                } else {
                    0
                };
                if let Some(raider) = tags.get("login") {
                    println!("RAID: {} -> {} ({})", raider, room, raid_size);
                    if settings.max_raidsize_to_follow >= raid_size
                        && raid_size >= settings.min_raidsize_to_follow
                    {
                        send_irc_cmd(settings, stream, "JOIN", format!("#{}", raider))?;
                    }
                    logs.log_raid(raider, room, raid_size)?;
                } else {
                    dbg!(tags);
                    panic!("NO RAIDER IN RAID");
                }
            }
            // TODO:
        }
        _ => {}
    };
    Ok(IrcMessageHandleResult::Ok)
}

pub fn main(settings: Settings, channels: Vec<String>) -> Result<(), MyError> {
    let mut logs: LogStreams = LogStreams::init(&settings)?;
    loop {
        let mut stream = init_anon_irc(&settings, &channels)?;
        recieve_irc(&settings, &mut stream, &mut logs)?;
    }
}
