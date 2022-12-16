use super::schema::{post_topics, posts, topics, users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Identifiable, Associations, AsChangeset)]
#[diesel(belongs_to(User))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Debug, Identifiable, AsChangeset)]
pub struct Topic {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Debug, Identifiable, AsChangeset)]
#[diesel(primary_key(id))]
pub struct PostTopic {
    pub id: i32,
    pub topic_id: i32,
    pub post_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = post_topics)]
pub struct NewPostTopic {
    pub topic_id: i32,
    pub post_id: i32,
}
