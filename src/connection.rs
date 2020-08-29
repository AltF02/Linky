use rocket_contrib::databases::{database, diesel::PgConnection};

#[database("postgres")]
pub(crate) struct DbConn(PgConnection);


