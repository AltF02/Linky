use rocket::{self, get, post};
use rocket_contrib::json::Json;
use crate::connection::*;
use crate::models::*;
use diesel::prelude::*;
use crate::schema::links;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::iter;


#[get("/links", rank = 4)]
pub(crate) fn get_links(conn: DbConn) -> Json<Vec<Link>> {
    let links = links::table
        .order(links::columns::id.desc())
        .load::<Link>(&*conn)
        .unwrap();
    Json(links)
}

#[post("/new", data = "<new_link>")]
pub(crate) fn create_link(conn: DbConn, new_link: Json<NewLink>) -> Json<Link> {
    let mut rng = thread_rng();
    let rand_path: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(8)
        .collect();

    let result = diesel::insert_into(links::table)
        .values((&*new_link, links::columns::path.eq(rand_path)))
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

#[get("/links/<id>", rank = 2)]
pub(crate) fn get_link_by_id(conn: DbConn, id: i32) -> Json<Vec<Link>> {
    let link = links::table
        .filter(links::columns::id.eq(id))
        .load(&*conn)
        .unwrap();
    Json(link)
}

#[get("/links/<url>", rank = 3)]
pub(crate) fn get_link_by_url(conn: DbConn, url: String) -> Json<Vec<Link>> {
    let links = links::table
        .filter(links::columns::redirect.eq(url))
        .load(&*conn)
        .unwrap();

    Json(links)
}