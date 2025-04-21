use std::{fs::File, io::Read, path::PathBuf};

use crate::Settings;

const HELP: &str = "
atrea collector

USAGE:
    atrea-collector [flags]

FLAGS:
    -c, --channel-list-file PATH    location of the channel-list (newline seperated list)
    -j, --log-joins                 log join infos
    -s, --log-shoutouts             log shoutouts
        --min-raidsize      NUMBER  minimum size of raids to follow (default: 3)
        --max-raidsize      NUMBER  maximum size of raids to follow (default: infinite)
        --stdout-sent               debug-log sent things via stdout
        --stdout-recieved           debug-log recieved things via stdout
";

pub fn main() {
    let (settings, channel_list_file) = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse arguments: {}", e);
            eprintln!("{HELP}");
            std::process::exit(1);
        }
    };
    let channel_list: Vec<String> = match read_channel_list_file(channel_list_file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read channel list file: {}", e);
            std::process::exit(1);
        }
    };
    #[cfg(feature = "irc")]
    match crate::irc::main(settings, channel_list) {
        Ok(..) => {}
        Err(e) => eprintln!("ERR: {}", e),
    };
    #[cfg(feature = "eventsub")]
    match crate::event_sub::main(settings, channel_list) {
        Ok(..) => {}
        Err(e) => eprintln!("ERR: {}", e),
    };
}

fn read_channel_list_file(channel_list_file: PathBuf) -> Result<Vec<String>, std::io::Error> {
    let mut f: File = File::open(channel_list_file)?;
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf.lines().map(String::from).collect::<Vec<String>>())
}

fn parse_args() -> Result<(Settings, PathBuf), pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        println!("{HELP}");
        std::process::exit(0);
    }
    let settings = Settings {
        log_joins: pargs.contains(["-j", "--log-joins"]),
        log_shoutouts: pargs.contains(["-s", "--log-shoutouts"]),
        stdout_log_sent: pargs.contains("--stdout-sent"),
        stdout_log_recieved: pargs.contains("--stdout-recieved"),
        min_raidsize_to_follow: pargs.opt_value_from_str("--min-raidsize")?.unwrap_or(3),
        max_raidsize_to_follow: pargs
            .opt_value_from_str("--max-raidsize")?
            .unwrap_or(usize::MAX),
    };
    let channel_list_file: PathBuf = pargs
        .opt_value_from_os_str(["-c", "--channel-list-file"], parse_path)?
        .unwrap_or_else(|| PathBuf::from("./liked_channels.json"));

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!(
            "Unsupported arguments provided: {}",
            remaining
                .into_iter()
                .map(|i| -> String { i.into_string().expect("non-unicode arguments recieved") })
                .collect::<Vec<String>>()
                .join(", ")
        );
        eprintln!("{HELP}");
        std::process::exit(1);
    }

    Ok((settings, channel_list_file))
}

fn parse_path(s: &std::ffi::OsStr) -> Result<PathBuf, &'static str> {
    Ok(s.into())
}
