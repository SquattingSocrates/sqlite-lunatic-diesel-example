use diesel::prelude::*;
use sqlite_lunatic_example::{
    establish_connection, models,
    schema::posts::dsl::{posts, published},
};
use std::env::args;

fn main() {
    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = connection
        .transaction::<_, diesel::result::Error, _>(|connection| {
            diesel::update(posts.find(id))
                .set(published.eq(true))
                .execute(connection)?;

            posts.find(id).first::<models::Post>(connection)
        })
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}
