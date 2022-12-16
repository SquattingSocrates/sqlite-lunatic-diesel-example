use diesel::prelude::*;
use lunatic::Mailbox;
use sqlite_lunatic_example::{models::Post, *};

#[lunatic::main]
fn main(_: Mailbox<()>) {
    use sqlite_lunatic_example::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
