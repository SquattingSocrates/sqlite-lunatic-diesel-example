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

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(connection)
        .unwrap();

    let post = posts.find(id).first::<models::Post>(connection);
    println!("TRYING TO FIND {:?}", post);
    // .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    // println!("Published post {}", post.title);
}
