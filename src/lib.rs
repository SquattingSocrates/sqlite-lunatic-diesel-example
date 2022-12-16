pub mod models;
pub mod schema;

use diesel::{prelude::*, SqliteConnection};
use dotenvy::dotenv;
use std::{env, io::stdin};

use models::NewPost;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut SqliteConnection, new_post: NewPost) -> usize {
    use crate::schema::posts;

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}

pub fn get_input<S: Into<String>>(message: S) -> String {
    println!("{}", message.into());
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
