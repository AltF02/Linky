use crate::schema::links;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};

#[database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
pub(crate) struct Link {
    id: i32,
    redirect: String,
    path: String
}

#[derive(Insertable, Deserialize)]
#[table_name="links"]
pub struct NewLink {
    redirect: String
}

