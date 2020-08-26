/*use crate::connection::DbConn;
use rocket_contrib::json::Json;
use crate::api::Link;
use crate::schema::links::dsl::links;
use diesel::result::Error;
use rocket::http::Status;
use rocket::*;
use rocket::http::RawStr;
use diesel::{sql_query, RunQueryDsl};

#[get("/")]
fn all (connection: DbConn) -> &'static str{
    let test = sql_query("SELECT * FROM linky.links")
        .get_results(&connection);
    println!("{}", test);
    "Yeet"
} */
