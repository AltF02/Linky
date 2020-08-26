#![feature(proc_macro_hygiene, decl_macro)]

mod redirect;
mod api;
mod connection;

use rocket::*;
use rocket::{
    response::{Redirect, content},
    http::RawStr,
};
use rocket_contrib::serve::{StaticFiles};

use redirect::static_rocket_route_info_for_redirect;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![redirect, index])
        .launch();
}
