use rocket::{self, get, post};
use rocket_contrib::json::Json;
use crate::connection::*;
use diesel::prelude::*;
use crate::schema::links;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::iter;

#[get("/links")]
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