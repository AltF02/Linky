#![feature(proc_macro_hygiene, decl_macro)]

mod redirect;
mod api;

use rocket::*;
use rocket::{
    response::{Redirect, content},
    http::RawStr,
};
use rocket_contrib::serve::{StaticFiles};
use rocket_contrib::databases::postgres;

use redirect::static_rocket_route_info_for_redirect;

fn main() {
    rocket::ignite()
        .mount("/", routes![redirect])
        .launch();
}
