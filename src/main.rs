#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod connection;
mod schema;
mod api;
mod models;

use rocket::{self, get, routes};
use diesel::prelude::*;

use connection::*;
use api::handler::*;
use rocket::response::Redirect;
use schema::links;
use rocket_contrib::serve::{StaticFiles};


#[get("/<path>", rank = 2)]
fn redirect(conn: DbConn, path: String) -> Option<Redirect> {
    let result: Vec<String> = links::table
        .select(links::columns::redirect)
        .filter(links::columns::path.eq(&path))
        .limit(1)
        .load(&*conn)
        .unwrap();
    if result.len() > 0 {
        Some(Redirect::to(format!("{}", result[0])))
    } else {
        None
    }

}
#[get("/", rank = 3)]
fn index() -> Redirect {
    Redirect::to("/ui")
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index, redirect])
        .mount("/ui", StaticFiles::from("static").rank(1))
        .mount("/api/", routes![get_links, create_link])
        .launch();
}