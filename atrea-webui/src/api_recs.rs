use crate::{json_escape_string, AtreaDb};

use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket_db_pools::{sqlx, sqlx::sqlite::SqliteRow, sqlx::Row};

struct Algorithm {
    pub name: &'static str,
    pub sql: &'static str,
    pub description: &'static str,
    pub used_data: &'static [&'static str],
}
impl Algorithm {
    pub fn api_json(&self) -> String {
        format!(
            r#"{{"name": "{}", "description": "{}", "used_data": [{}]}}"#,
            json_escape_string(self.name),
            json_escape_string(self.description),
            self.used_data
                .iter()
                .map(|i| { format!(r#""{}""#, json_escape_string(i)) })
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

const AVAILABLE_GENERAL_ALGORITHMS: &[Algorithm] = &[
    Algorithm {
        name: "brta1",
        sql: include_str!("sql/recs/brta1.sql"),
        description: "Basic raid trace analysis v1. Mainly finds friendgroups.",
        used_data: &["raids"],
    },
    Algorithm {
        name: "brta2",
        sql: include_str!("sql/recs/brta2.sql"),
        description: "Basic raid trace analysis v2 (improved handling of big datasets). Mainly finds friendgroups.",
        used_data: &["raids"],
    },
    Algorithm {
        name: "bsc",
        sql: include_str!("sql/recs/bsc.sql"),
        description: "Basic shoutout connections.",
        used_data: &["shoutouts"],
    },
    Algorithm {
        name: "bsv",
        sql: include_str!("sql/recs/bsv.sql"),
        description: "Basic shared viewers. Heavily favors big streamers.",
        used_data: &["joins"],
    },
    Algorithm {
        name: "rava1",
        sql: include_str!("sql/recs/rava1.sql"),
        description: "Raid and viewer analysis v1 (vixen1 + brta1). Can be pretty slow and tends to find ones you never heard of.",
        used_data: &["raids", "joins"],
    },
    Algorithm {
        name: "rava2",
        sql: include_str!("sql/recs/rava2.sql"),
        description: "Raid and viewer analysis v2 (vixen1 + brta2). Can be pretty slow and tends to find ones you never heard of.",
        used_data: &["raids", "joins"],
    },
    Algorithm {
        name: "vixen1",
        sql: include_str!("sql/recs/vixen1.sql"),
        description: "Weighted shared viewer analysis. Finds loosely similar streamers.",
        used_data: &["joins"],
    },
];

#[get("/api/recs/raided_most_liked_channels?<offset>&<limit>")]
pub async fn raided_most_liked_channels(
    db: rocket_db_pools::Connection<AtreaDb>,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<RawJson<String>, Status> {
    for_query("SELECT rc.raider AS channel, COUNT(rc.raider) * 1.0 AS score FROM raid_connections rc INNER JOIN liked_channels lc ON lc.name == rc.target GROUP BY rc.raider ORDER BY score DESC LIMIT ? OFFSET ?",db , offset, limit).await
}

#[get("/api/recs/algorithms/general")]
pub async fn list_general_algorithms() -> RawJson<String> {
    RawJson(format!(
        "[{}]",
        AVAILABLE_GENERAL_ALGORITHMS
            .iter()
            .map(|i| i.api_json())
            .collect::<Vec<String>>()
            .join(",")
    ))
}

#[get("/api/recs/general/<algorithm>?<offset>&<limit>")]
pub async fn get_general(
    db: rocket_db_pools::Connection<AtreaDb>,
    offset: Option<u32>,
    limit: Option<u32>,
    algorithm: &str,
) -> Result<RawJson<String>, Status> {
    let algorithm: &Algorithm = match AVAILABLE_GENERAL_ALGORITHMS
        .iter()
        .find(|i| i.name == algorithm)
    {
        Some(alg) => alg,
        None => {
            return Err(Status::BadRequest);
        }
    };
    for_query(algorithm.sql, db, offset, limit).await
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
    query: &'static str,
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
