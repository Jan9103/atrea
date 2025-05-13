use std::path::PathBuf;

use crate::{json_escape_string, AtreaDb, AtreaSettingsDb};

use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket_db_pools::{sqlx, sqlx::sqlite::SqliteRow, sqlx::Row};

struct Algorithm {
    pub name: &'static str,
    pub sql: &'static str,
    pub description: &'static str,
    pub used_data: &'static [&'static str],
    pub is_primary: bool,
}
impl Algorithm {
    pub fn api_json(&self) -> String {
        format!(
            r#"{{"name": "{}", "description": "{}", "used_data": [{}], "is_primary": {}}}"#,
            json_escape_string(self.name),
            json_escape_string(self.description),
            self.used_data
                .iter()
                .map(|i| { format!(r#""{}""#, json_escape_string(i)) })
                .collect::<Vec<String>>()
                .join(","),
            if self.is_primary { "true" } else { "false" },
        )
    }
}

const AVAILABLE_GENERAL_ALGORITHMS: &[Algorithm] = &[
    Algorithm {
        name: "brta2",
        sql: include_str!("sql/recs/brta2.sql"),
        description: "Basic raid trace analysis v2 (improved handling of big datasets). Mainly finds friendgroups.",
        used_data: &["raids"],
        is_primary: true,
    },
    Algorithm {
        name: "bsc",
        sql: include_str!("sql/recs/bsc.sql"),
        description: "Basic shoutout connections.",
        used_data: &["shoutouts"],
        is_primary: false,
    },
    Algorithm {
        name: "bsv",
        sql: include_str!("sql/recs/bsv.sql"),
        description: "Basic shared viewers. Heavily favors big streamers.",
        used_data: &["joins"],
        is_primary: false,
    },
    Algorithm {
        name: "rava3",
        sql: include_str!("sql/recs/rava3.sql"),
        description: "Raid and viewer analysis v3 (vixen2 + brta2). Can be pretty slow.",
        used_data: &["raids", "joins"],
        is_primary: true,
    },
    Algorithm {
        name: "vixen2",
        sql: include_str!("sql/recs/vixen2.sql"),
        description: "Weighted shared viewer analysis. Finds similar channels around a few corners.",
        used_data: &["joins"],
        is_primary: false,
    },
];

#[get("/api/recs/algorithms/general")]
pub async fn list_general_algorithms(
    mut db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<RawJson<String>, Status> {
    let mut plugin_algos: Vec<String> = match sqlx::query(
        "SELECT pa.algorithm_name, pa.algorithm_description, pa.is_primary, pa.used_data FROM plugin_algorithms pa INNER JOIN plugins p ON p.plugin_name = pa.plugin_name WHERE p.enabled == true;",
    )
    .fetch_all(&mut **db)
    .await
    {
        Ok(res) => {
            res.into_iter().map(|sql_row| {
                let alg_name: &str = sql_row.get(0);
                let alg_desc: &str = sql_row.get(1);
                let is_primary: bool = sql_row.get(2);
                let used_data: &str = sql_row.get(3);
                format!(
                    r#"{{"name": "{}", "description": "{}", "is_primary": {}, "used_data": [{}]}}"#,
                    json_escape_string(alg_name),
                    json_escape_string(alg_desc),
                    if is_primary {"true"} else {"false"},
                    used_data.split_ascii_whitespace().map(|i| format!("\"{}\"", json_escape_string(i))).collect::<Vec<String>>().join(",")
                )
            }).collect::<Vec<String>>()
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };

    let mut all_algos: Vec<String> = AVAILABLE_GENERAL_ALGORITHMS
        .iter()
        .map(|i| i.api_json())
        .collect::<Vec<String>>();
    all_algos.append(&mut plugin_algos);

    Ok(RawJson(format!("[{}]", all_algos.join(","))))
}

#[get("/api/recs/general/<algorithm>?<offset>&<limit>")]
pub async fn get_general(
    db: rocket_db_pools::Connection<AtreaDb>,
    mut settings_db: rocket_db_pools::Connection<AtreaSettingsDb>,
    offset: Option<u32>,
    limit: Option<u32>,
    algorithm: &str,
) -> Result<RawJson<String>, Status> {
    if let Some(alg) = AVAILABLE_GENERAL_ALGORITHMS
        .iter()
        .find(|i| i.name == algorithm)
    {
        return for_query(alg.sql, db, offset, limit).await;
    }
    match sqlx::query(
        "SELECT algorithm_name, plugin_name FROM plugin_algorithms WHERE algorithm_name == ?;",
    )
    .bind(algorithm)
    .fetch_all(&mut **settings_db)
    .await
    {
        Ok(res) => match res.into_iter().next() {
            Some(col) => {
                let algo_name: &str = col.get(0);
                let plugin_name: &str = col.get(1);
                let file_path: PathBuf = PathBuf::from("plugins")
                    .join(plugin_name)
                    .join(format!("{algo_name}.sql"));
                if !file_path.is_file() {
                    eprintln!(
                        "Invalid plugin algorithm: {} of {} is missing its sql file",
                        algo_name, plugin_name
                    );
                    return Err(Status::InternalServerError);
                }
                match std::fs::read_to_string(file_path) {
                    Ok(v) => {
                        return for_query(v.as_str(), db, offset, limit).await;
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        Err(Status::InternalServerError)
                    }
                }
            }
            None => Err(Status::BadRequest),
        },
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/api/recs/general/liked_channels?<offset>&<limit>")]
pub async fn get_liked_channels(
    db: rocket_db_pools::Connection<AtreaDb>,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<RawJson<String>, Status> {
    for_query(
        include_str!("sql/recs/liked_channels.sql"),
        db,
        offset,
        limit,
    )
    .await
}

/// query has to:
///   return channel (str) and score (f64)
///   accept 2 parameters: 1. limit 2. offset
async fn for_query(
    query: &str,
    mut db: rocket_db_pools::Connection<AtreaDb>,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<RawJson<String>, Status> {
    let res: Vec<SqliteRow> = match sqlx::query(query)
        .bind(limit.unwrap_or(20))
        .bind(offset.unwrap_or(0))
        .fetch_all(&mut **db)
        .await
    {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };
    Ok(RawJson(format!(
        "[{}]",
        res.into_iter()
            .map(|r| -> String {
                let channel: &str = r.get("channel");
                let score: f64 = r.get("score");
                format!(
                    r#"{{"channel": "{}", "score": {}}}"#,
                    json_escape_string(channel),
                    score
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    )))
}
