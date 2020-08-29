use crate::schema::links;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize)]
pub(crate) struct Link {
    id: i32,
    redirect: String,
    path: String
}

#[derive(Insertable, Deserialize)]
#[table_name="links"]
pub(crate) struct NewLink {
    redirect: String
}