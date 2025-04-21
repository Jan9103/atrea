/// settings changable via CLI
#[derive(Copy, Clone)]
pub struct Settings {
    pub log_joins: bool,
    pub log_shoutouts: bool,
    pub stdout_log_sent: bool,
    pub stdout_log_recieved: bool,
    pub min_raidsize_to_follow: usize,
    pub max_raidsize_to_follow: usize,
}
