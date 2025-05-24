use rocket::{http::Status, response::content::RawJson};
use rocket_db_pools::{sqlx, sqlx::Row};

use crate::AtreaDb;

#[get("/api/graph/all/nodes")]
pub async fn get_all_nodes(
    mut db: rocket_db_pools::Connection<AtreaDb>,
) -> Result<RawJson<String>, Status> {
    match sqlx::query(include_str!("./sql/graph/force_graph_nodes.sql"))
        .fetch_all(&mut **db)
        .await
    {
        Ok(res) => Ok(RawJson(format!(
            "[{}]",
            res.into_iter()
                .map(|r| r.get(0))
                .collect::<Vec<String>>()
                .join(",")
        ))),
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/api/graph/all/links")]
pub async fn get_all_links(
    mut db: rocket_db_pools::Connection<AtreaDb>,
) -> Result<RawJson<String>, Status> {
    match sqlx::query(include_str!("./sql/graph/force_graph_links.sql"))
        .fetch_all(&mut **db)
        .await
    {
        Ok(res) => Ok(RawJson(format!(
            "[{}]",
            res.into_iter()
                .map(|r| r.get(0))
                .collect::<Vec<String>>()
                .join(",")
        ))),
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/api/graph/neighbour/nodes/<login>/<depth>")]
pub async fn get_neightbour_nodes(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    login: &str,
    depth: usize,
) -> Result<RawJson<String>, Status> {
    let sql_query = sql_query_to_gather_neighbour_nodes(depth);
    match sqlx::query(sql_query.as_str())
        .bind(login)
        .bind(login)
        .bind(login)
        .bind(login)
        .fetch_all(&mut **db)
        .await
    {
        Ok(res) => Ok(RawJson(format!(
            "[{}]",
            res.into_iter()
                .map(|r| r.get(0))
                .collect::<Vec<String>>()
                .join(",")
        ))),
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/api/graph/neighbour/links/<login>/<depth>")]
pub async fn get_neightbour_links(
    mut db: rocket_db_pools::Connection<AtreaDb>,
    login: &str,
    depth: usize,
) -> Result<RawJson<String>, Status> {
    let sql_query = sql_query_to_gather_neighbour_links(depth);
    match sqlx::query(sql_query.as_str())
        .bind(login)
        .bind(login)
        .fetch_all(&mut **db)
        .await
    {
        Ok(res) => Ok(RawJson(format!(
            "[{}]",
            res.into_iter()
                .map(|r| r.get(0))
                .collect::<Vec<String>>()
                .join(",")
        ))),
        Err(err) => {
            eprintln!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}

/// you will have to insert:
/// 1. center node login
/// 2. center node login
fn sql_query_to_gather_neighbour_links(depth: usize) -> String {
    const CORE_SQL: &str = r#"
        SELECT DISTINCT login FROM (
            SELECT rc0.raider AS login
            FROM raid_connections rc0
            WHERE rc0.raider == ?
            UNION
            SELECT rc1.target AS login
            FROM raid_connections rc1
            WHERE rc1.target == ?
        )
    "#;
    const LEVEL_SQL: &str = r#"
        WITH l[[lvl]] (login) AS (
            [[CHILD]]
        ), n[[lvl]] (raider, target) AS (
            SELECT rc.raider, rc.target
            FROM raid_connections rc
            WHERE rc.raider IN (SELECT login FROM l[[lvl]]) OR rc.target IN (SELECT login FROM l[[lvl]])
        )
        SELECT DISTINCT login FROM (
            SELECT na.raider AS login
            FROM n[[lvl]] na
            UNION
            SELECT nb.target AS login
            FROM n[[lvl]] nb
        )
    "#;
    const SHELL_SQL: &str = r#"
        WITH sc (login) AS (
            [[CHILD]]
        )
        SELECT JSON_OBJECT(
            'source', src.raider,
            'target', src.target,
            'raidcount', src.raid_count,
            'avg_size', src.average_raid_size
        )
        FROM raid_connections src
        WHERE src.raider IN (SELECT login FROM sc) OR src.target IN (SELECT login FROM sc);
    "#;

    let mut current_state: String = String::from(CORE_SQL);

    for i in 1..depth {
        current_state = LEVEL_SQL
            .replace("[[lvl]]", &format!("{i}"))
            .replace("[[CHILD]]", &current_state);
    }

    SHELL_SQL.replace("[[CHILD]]", &current_state)
}

/// you will have to insert:
/// 1. center node login
/// 2. center node login
/// 3. center node login
/// 4. center node login
fn sql_query_to_gather_neighbour_nodes(depth: usize) -> String {
    const CORE_SQL: &str = r#"
        SELECT DISTINCT login FROM (
            SELECT rc0.raider AS login
            FROM raid_connections rc0
            WHERE rc0.raider == ?
            GROUP BY rc0.raider
            UNION
            SELECT rc0.target AS login
            FROM raid_connections rc0
            WHERE rc0.target == ?
            GROUP BY rc0.target
            UNION
            SELECT rc1.raider AS login
            FROM raid_connections rc1
            WHERE rc1.target == ?
            GROUP BY rc1.raider
            UNION
            SELECT rc2.target AS login
            FROM raid_connections rc2
            WHERE rc2.raider == ?
            GROUP BY rc2.target
        )
    "#;
    const LEVEL_SQL: &str = r#"
        WITH l[[lvl]] (login) AS (
            [[CHILD]]
        )
        SELECT DISTINCT login FROM (
            SELECT r1.login AS login
            FROM l[[lvl]] r1
            UNION
            SELECT rc1.raider AS login
            FROM raid_connections rc1
            WHERE rc1.target IN (SELECT login FROM l[[lvl]])
            GROUP BY rc1.raider
            UNION
            SELECT rc2.target AS login
            FROM raid_connections rc2
            WHERE rc2.raider IN (SELECT login FROM l[[lvl]])
            GROUP BY rc2.target
        )
    "#;
    const SHELL_SQL: &str = r##"
        SELECT JSON_OBJECT(
            'name', c.login,
            'id', c.login,
            'val', IIF(c.login in (SELECT name FROM liked_channels), 5, 1),
            'color', IIF(c.login in (SELECT name FROM liked_channels), "#f00", "#aa0")
        )
        FROM (
            [[CHILD]]
        ) c;
    "##;

    let mut current_query: String = String::from(CORE_SQL);

    for i in 1..depth {
        current_query = LEVEL_SQL
            .replace("[[lvl]]", &format!("{i}"))
            .replace("[[CHILD]]", &current_query);
    }

    SHELL_SQL.replace("[[CHILD]]", &current_query)
}
