use rocket::{
    http::{ContentType, Status},
    response::content,
};
use rocket_db_pools::sqlx::{self, Row};

use crate::AtreaSettingsDb;

const EXTRA_HEAD_MARKER: &str = "<!--HEAD-->";

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
#[get("/box_recs.html")]
pub async fn get_html_box_recs(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_recs").await?;
    let html: String =
        include_str!("html/box_recs.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/box_rel_graph.html")]
pub async fn get_html_box_rel_graph(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "box_rel_graph").await?;
    let html: String =
        include_str!("html/box_rel_graph.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}
#[get("/index.html")]
pub async fn get_html_index(
    db: rocket_db_pools::Connection<AtreaSettingsDb>,
) -> Result<content::RawHtml<String>, Status> {
    let extra_head: String = get_extra_head(db, "index").await?;
    let html: String =
        include_str!("html/index.html").replace(EXTRA_HEAD_MARKER, extra_head.as_str());
    Ok(content::RawHtml(html))
}

// ######
// # JS #
// ######

#[get("/box_channel.js")]
pub async fn get_js_box_channel() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/box_channel.js"))
}
#[get("/box_nav.js")]
pub async fn get_js_box_nav() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/box_nav.js"))
}
#[get("/box_recs.js")]
pub async fn get_js_box_recs() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/box_recs.js"))
}
#[get("/index_mailbox.js")]
pub async fn get_js_index_mailbox() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/index_mailbox.js"))
}

// libs

#[get("/libs/force-graph.min.js")]
pub async fn get_js_lib_force_graph() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/libs/force-graph.min.js"))
}
#[get("/libs/winbox.min.js")]
pub async fn get_js_lib_winbox() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/libs/winbox.min.js"))
}
#[get("/libs/xeact.js")]
pub async fn get_js_lib_xeact() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/libs/xeact.js"))
}

// #######
// # CSS #
// #######

#[get("/biglist.css")]
pub async fn get_css_biglist() -> content::RawCss<&'static [u8]> {
    content::RawCss(include_bytes!("html/biglist.css"))
}
#[get("/box_style.css")]
pub async fn get_css_box_style() -> content::RawCss<&'static [u8]> {
    content::RawCss(include_bytes!("html/box_style.css"))
}
#[get("/style.css")]
pub async fn get_css_style() -> content::RawCss<&'static [u8]> {
    content::RawCss(include_bytes!("html/style.css"))
}

// #######
// # SVG #
// #######

#[get("/loading_image.svg")]
pub async fn get_svg_loading() -> RawSvg<&'static [u8]> {
    RawSvg(include_bytes!("html/loading_image.svg"))
}
#[get("/twitch_glitch_logo.svg")]
pub async fn get_svg_twitch_glitch() -> RawSvg<&'static [u8]> {
    RawSvg(include_bytes!("html/twitch_glitch_logo.svg"))
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
                        r#"<link rel="stylesheet" href="api/plugins/get_css/{plugin_name}/{css_name}">"#
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
                        r#"<script src="api/plugins/get_js/{plugin_name}/{js_name}" type="module">"#
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

pub struct RawSvg<R>(pub R);
impl<'r, 'o: 'r, R: rocket::response::Responder<'r, 'o>> rocket::response::Responder<'r, 'o>
    for RawSvg<R>
{
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        (ContentType::SVG, self.0).respond_to(request)
    }
}
