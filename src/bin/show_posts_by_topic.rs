use diesel::prelude::*;
use lunatic::Mailbox;
use sqlite_lunatic_example::{
    models::{Post, PostTopic, Topic},
    *,
};

#[lunatic::main]
fn main(_: Mailbox<()>) {
    use sqlite_lunatic_example::schema::{
        post_topics::dsl as post_topics_dsl, posts, posts::dsl as post_dsl,
        topics::dsl as topics_dsl,
    };

    let connection = &mut establish_connection();
    let topic_name = get_input("What topic/tag of posts do you want to view?");

    let posts = topics_dsl::topics
        .inner_join(post_topics_dsl::post_topics.inner_join(post_dsl::posts))
        .filter(topics_dsl::name.eq(&topic_name))
        .select(posts::all_columns)
        .load::<Post>(connection)
        .expect(format!("Error loading posts matching topic {}", topic_name).as_str());

    println!("Displaying {} matching posts", posts.len());
    for post in posts {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
