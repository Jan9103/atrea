use crate::Settings;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct LogStreams {
    raid_csv: File,
    join_csv: Option<File>,
    shoutout_csv: Option<File>,
}

impl LogStreams {
    pub fn init(settings: &Settings) -> Result<Self, std::io::Error> {
        let mut res = Self {
            raid_csv: File::create_new(PathBuf::from(".").join(
                format!("raids-starting-{time}.csv",
                            time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                                ),
            ))?,
            join_csv: if settings.log_joins {
                Some(File::create_new(PathBuf::from(".").join(
                    format!("joins-starting-{time}.csv",
                            time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                                ),
                ))?)
            } else {
                None
            },
            shoutout_csv: if settings.log_shoutouts {
                Some(File::create_new(PathBuf::from(".").join(
                    format!("shoutouts-starting-{time}.csv",
                            time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                                ),
                ))?)
            } else {
                None
            },
        };
        res.raid_csv
            .write_all("timestamp,raider,target,size\n".as_bytes())?;
        if let Some(ref mut join_csv) = res.join_csv {
            join_csv.write_all("timestamp,chatter,target\n".as_bytes())?;
        };
        if let Some(ref mut shoutout_csv) = res.shoutout_csv {
            shoutout_csv.write_all("timestamp,author,target\n".as_bytes())?;
        };
        Ok(res)
    }

    pub fn log_raid(
        &mut self,
        raider: &str,
        target: &str,
        size: usize,
    ) -> Result<(), std::io::Error> {
        self.raid_csv.write_all(
            format!(
                "{time},{raider},{target},{size}\n",
                time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )
            .as_bytes(),
        )?;
        self.raid_csv.flush()?;
        Ok(())
    }

    pub fn log_join(&mut self, chatter: &str, room: &str) -> Result<(), std::io::Error> {
        if let Some(ref mut join_csv) = self.join_csv {
            join_csv.write_all(
                format!(
                    "{time},{chatter},{room}\n",
                    time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                )
                .as_bytes(),
            )?;
            join_csv.flush()?;
        }
        Ok(())
    }

    pub fn log_shoutout(&mut self, author: &str, target: &str) -> Result<(), std::io::Error> {
        if let Some(ref mut shoutout_csv) = self.shoutout_csv {
            shoutout_csv.write_all(
                format!(
                    "{time},{author},{target}\n",
                    time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                )
                .as_bytes(),
            )?;
            shoutout_csv.flush()?;
        }
        Ok(())
    }
}
