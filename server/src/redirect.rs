use rocket::*;
use rocket::http::RawStr;

#[get("/<path>")]
pub(crate) fn redirect(path: &RawStr) -> String {
    format!("Redirect to: {}!", path.as_str())
}
