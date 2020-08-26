use super::schema::links;


pub mod post;
pub mod handler;

#[derive(Queryable, AsChangeset)]
#[table_name = "links"]
pub struct Link {
    pub id: i64,
    pub redirect: String,
    pub path: String,
}
