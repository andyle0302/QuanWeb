pub mod users;
pub mod blogs;

pub use users::{User, Role};
pub use blogs::{DocFormat, RawBlogPost, DetailedBlogPost, BlogCategory};

#[derive(Debug, serde::Serialize, serde::Deserialize, edgedb_derive::Queryable)]
pub struct MinimalObject {
    pub id: uuid::Uuid,
}
