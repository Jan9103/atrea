use rocket::{http::ContentType, response::content};

#[get("/")]
pub async fn get_slash() -> rocket::response::Redirect {
    rocket::response::Redirect::to(uri!("./index.html"))
}
#[get("/index.html")]
pub async fn get_index() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/index.html"))
}
#[get("/style.css")]
pub async fn get_style() -> content::RawCss<&'static [u8]> {
    content::RawCss(include_bytes!("html/style.css"))
}
#[get("/channel_view_iframe.html")]
pub async fn get_channel_view_iframe() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/channel_view_iframe.html"))
}
#[get("/algorithm_overview_iframe.html")]
pub async fn get_algorithm_overview_iframe() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/algorithm_overview_iframe.html"))
}
// #[get("/channel_view.js")]
// pub async fn get_channel_view_js() -> content::RawJavaScript<&'static [u8]> {
//     content::RawJavaScript(include_bytes!("html/channel_view.js"))
// }
#[get("/recommend_channels.html")]
pub async fn get_recommend_channels() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/recommend_channels.html"))
}
#[get("/recommend_channels.js")]
pub async fn get_recommend_channels_js() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/recommend_channels.js"))
}
#[get("/xeact.js")]
pub async fn get_xeact() -> content::RawJavaScript<&'static [u8]> {
    content::RawJavaScript(include_bytes!("html/xeact.js"))
}
#[get("/credits.html")]
pub async fn get_credits() -> content::RawHtml<&'static [u8]> {
    content::RawHtml(include_bytes!("html/credits.html"))
}
#[get("/loading_image.svg")]
pub async fn get_loading_image() -> RawSvg<&'static [u8]> {
    RawSvg(include_bytes!("html/loading_image.svg"))
}
#[get("/twitch_glitch_logo.svg")]
pub async fn get_twitch_glitch_logo() -> RawSvg<&'static [u8]> {
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
