use rocket::{http::ContentType, response::content};

#[get("/")]
pub async fn get_slash() -> rocket::response::Redirect {
    rocket::response::Redirect::to(uri!("./index.html"))
}

// ########
// # HTML #
// ########

#[get("/box_channel.html")]
pub async fn get_html_box_channel() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/box_channel.html"))
}
#[get("/box_help_credits.html")]
pub async fn get_html_box_help_credits() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/box_help_credits.html"))
}
#[get("/box_help_recs.html")]
pub async fn get_html_box_help_recs() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/box_help_recs.html"))
}
#[get("/box_known_viewers.html")]
pub async fn get_html_box_known_viewers() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/box_known_viewers.html"))
}
#[get("/box_recs.html")]
pub async fn get_html_box_recs() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/box_recs.html"))
}
#[get("/index.html")]
pub async fn get_html_index() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/index.html"))
}

// ######
// # JS #
// ######

#[get("/box_channel.js")]
pub async fn get_js_box_channel() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/box_channel.js"))
}
#[get("/box_recs.js")]
pub async fn get_js_box_recs() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/box_recs.js"))
}
#[get("/index_mailbox.js")]
pub async fn get_js_index_mailbox() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/index_mailbox.js"))
}
#[get("/index_nav.js")]
pub async fn get_js_index_nav() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/index_nav.js"))
}

// libs

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
#[get("/index_style.css")]
pub async fn get_css_index_style() -> content::RawCss<&'static [u8]> {
    content::RawCss(include_bytes!("html/index_style.css"))
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

pub struct RawSvg<R>(pub R);
impl<'r, 'o: 'r, R: rocket::response::Responder<'r, 'o>> rocket::response::Responder<'r, 'o>
    for RawSvg<R>
{
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        (ContentType::SVG, self.0).respond_to(request)
    }
}
