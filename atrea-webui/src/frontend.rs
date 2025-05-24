use std::borrow::Cow;

use rocket::{
    http::{ContentType, Header, Status},
    response::content,
};
use rocket_db_pools::sqlx::{self, Row};

use crate::AtreaSettingsDb;

const EXTRA_HEAD_MARKER: &str = "<!--HEAD-->";

#[derive(rocket::Responder)]
pub struct AFeR<T> {
    inner: T,
    content_type: ContentType,
    cache_control: Header<'static>,
}
impl<T> AFeR<T> {
    const DEFAULT_CACHE_CONTROL: &'static str = "max-age=3600"; // 1hour
    pub fn new(body: T, content_type: ContentType) -> Self {
        Self {
            inner: body,
            content_type,
            cache_control: Header {
                name: rocket::http::uncased::Uncased::new("Cache-Control"),
                value: Cow::from(Self::DEFAULT_CACHE_CONTROL),
            },
        }
    }
}

#[get("/")]
pub async fn get_slash() -> rocket::response::Redirect {
    rocket::response::Redirect::to(uri!("./index.html"))
}

// ########
// # HTML #
// ########

#[get("/box_channel.html")]
pub async fn get_html_box_channel(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_channel").await?;
    let html: String =
        include_str!("html/box_channel.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_viewer.html")]
pub async fn get_html_box_viewer(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_viewer").await?;
    let html: String =
        include_str!("html/box_viewer.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_help_credits.html")]
pub async fn get_html_box_help_credits(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_help_credits").await?;
    let html: String =
        include_str!("html/box_help_credits.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_help_recs.html")]
pub async fn get_html_box_help_recs(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_help_recs").await?;
    let html: String =
        include_str!("html/box_help_recs.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_known_viewers.html")]
pub async fn get_html_box_known_viewers(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_known_viewers").await?;
    let html: String =
        include_str!("html/box_known_viewers.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_nav.html")]
pub async fn get_html_box_nav(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_nav").await?;
    let html: String =
        include_str!("html/box_nav.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_plugins.html")]
pub async fn get_html_box_plugins(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_plugins").await?;
    let html: String =
        include_str!("html/box_plugins.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_raw.html")]
pub async fn get_html_box_raw(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_raw").await?;
    let html: String =
        include_str!("html/box_raw.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_recs.html")]
pub async fn get_html_box_recs(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_recs").await?;
    let html: String =
        include_str!("html/box_recs.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[deprecated]
#[get("/box_rel_graph.html")]
pub async fn get_html_box_rel_graph(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_rel_graph").await?;
    let html: String =
        include_str!("html/box_rel_graph.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_graph_v2.html")]
pub async fn get_html_box_graph_v2(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_graph_v2").await?;
    let html: String =
        include_str!("html/box_graph_v2.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_graph_v2_control.html")]
pub async fn get_html_box_graph_v2_control(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_graph_v2_control").await?;
    let html: String = include_str!("html/box_graph_v2_control.html")
        .replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/index.html")]
pub async fn get_html_index(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "index").await?;
    let html: String = include_str!("html/index.html")
        .replace(EXTRA_HEAD_MARKER, extra_head.as_str())
        .replace(
            "<!--VERSION-->",
            option_env!("CARGO_PKG_VERSION").unwrap_or("(unknown version)"),
        );
    Ok(content::RawHtml(html))
}

// ######
// # JS #
// ######

#[get("/atrea.js")]
pub async fn get_js_atrea() -> AFeR<&'static [u8]> {
    AFeR::new(include_bytes!("html/atrea.js"), ContentType::JavaScript)
}

#[get("/box_channel.js")]
pub async fn get_js_box_channel() -> AFeR<&'static [u8]> {
    AFeR::new(
        include_bytes!("html/box_channel.js"),
        ContentType::JavaScript,
    )
}
#[get("/box_nav.js")]
pub async fn get_js_box_nav() -> AFeR<&'static [u8]> {
    AFeR::new(include_bytes!("html/box_nav.js"), ContentType::JavaScript)
}
#[get("/box_recs.js")]
pub async fn get_js_box_recs() -> AFeR<&'static [u8]> {
    AFeR::new(include_bytes!("html/box_recs.js"), ContentType::JavaScript)
}
#[get("/index_mailbox.js")]
pub async fn get_js_index_mailbox() -> AFeR<&'static [u8]> {
    AFeR::new(
        include_bytes!("html/index_mailbox.js"),
        ContentType::JavaScript,
    )
}

// libs

#[get("/libs/force-graph.min.js")]
pub async fn get_js_lib_force_graph() -> AFeR<&'static [u8]> {
    AFeR::new(
        include_bytes!("html/libs/force-graph.min.js"),
        ContentType::JavaScript,
    )
}
#[get("/libs/winbox.min.js")]
pub async fn get_js_lib_winbox() -> AFeR<&'static [u8]> {
    AFeR::new(
        include_bytes!("html/libs/winbox.min.js"),
        ContentType::JavaScript,
    )
}
#[get("/libs/xeact.js")]
pub async fn get_js_lib_xeact() -> AFeR<&'static [u8]> {
    AFeR::new(
        include_bytes!("html/libs/xeact.js"),
        ContentType::JavaScript,
    )
}

// #######
// # CSS #
// #######

#[get("/biglist.css")]
pub async fn get_css_biglist() -> AFeR<&'static [u8]> {
    AFeR::new(include_bytes!("html/biglist.css"), ContentType::CSS)
}
#[get("/box_style.css")]
pub async fn get_css_box_style() -> AFeR<&'static [u8]> {
    AFeR::new(include_bytes!("html/box_style.css"), ContentType::CSS)
}
#[get("/style.css")]
pub async fn get_css_style() -> AFeR<&'static [u8]> {
    AFeR::new(include_bytes!("html/style.css"), ContentType::CSS)
}

// #######
// # SVG #
// #######

#[get("/loading_image.svg")]
pub async fn get_svg_loading() -> AFeR<&'static [u8]> {
    AFeR::new(include_bytes!("html/loading_image.svg"), ContentType::SVG)
}
#[get("/logo.svg")]
pub async fn get_svg_logo() -> AFeR<&'static [u8]> {
    AFeR::new(include_bytes!("html/logo.svg"), ContentType::SVG)
}
#[get("/twitch_glitch_logo.svg")]
pub async fn get_svg_twitch_glitch() -> AFeR<&'static [u8]> {
    AFeR::new(
        include_bytes!("html/twitch_glitch_logo.svg"),
        ContentType::SVG,
    )
}

// ###########
// # UTILITY #
// ###########

async fn get_extra_head(
    mut db: rocket_db_pools::Connection<AtreaSettingsDb>,
    base_name: &'static str,
) -> Result<String, Status> {
    let mut result = String::new();

    let css_name: String = format!("{base_name}.css");
    match sqlx::query(
        r#"
        SELECT DISTINCT pl.plugin_name FROM plugins pl
        JOIN plugin_files pf ON pl.plugin_name = pf.plugin_name
        WHERE pl.enabled == TRUE
        AND file_name == ?;
        "#,
    )
    .bind(&css_name)
    .fetch_all(&mut **db)
    .await
    {
        Ok(res) => {
            for plugin in res.iter() {
                let plugin_name: String = plugin.get(0);
                result.push_str(
                    format!(
                        r#"<link rel="stylesheet" href="get_css_plugin?plugin_name={plugin_name}&css_name={css_name}">"#
                    )
                    .as_str(),
                );
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };
    let js_name: String = format!("{base_name}.js");
    match sqlx::query(
        r#"
        SELECT DISTINCT pl.plugin_name FROM plugins pl
        JOIN plugin_files pf ON pl.plugin_name = pf.plugin_name
        WHERE pl.enabled == TRUE
        AND file_name == ?;
        "#,
    )
    .bind(&js_name)
    .fetch_all(&mut **db)
    .await
    {
        Ok(res) => {
            for plugin in res.iter() {
                let plugin_name: String = plugin.get(0);
                result.push_str(
                    format!(
                        r#"<script src="get_js_plugin?plugin_name={plugin_name}&js_name={js_name}" type="module"></script>"#
                    )
                    .as_str(),
                );
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(Status::InternalServerError);
        }
    };

    Ok(result)
}
