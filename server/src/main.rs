#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod connection;
mod schema;
mod api;

use rocket::{self, get, routes};
use diesel::prelude::*;

use connection::*;
use api::handler::*;
use rocket::response::Redirect;
use schema::links;


#[get("/<path>")]
fn redirect(conn: DbConn, path: String) -> Redirect {
    let result: Vec<String> = links::table
        .select(links::columns::redirect)
        .filter(links::columns::path.eq(&path))
        .limit(1)
        .load(&*conn)
        .unwrap();
    Redirect::to(format!("{}", result[0]))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![redirect, index])
        .mount("/api/", routes![get_links, create_link])
        .launch();
}