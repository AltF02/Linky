#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod redirect;
mod api;
mod connection;
mod schema;

use rocket::*;
use rocket::{
    response::{Redirect, content},
    http::RawStr,
};
use rocket_contrib::serve::{StaticFiles};

use redirect::static_rocket_route_info_for_redirect;
use crate::connection::DbConn;
use diesel::{Connection, sql_query, RunQueryDsl};

#[get("/")]
fn index(connection: DbConn) -> &'static str {
    let test = sql_query("SELECT * FROM linky.links")
        .get_result(&*connection);
    println!("{:?}", test);
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![redirect, index])
        .launch();
}
