use diesel::{prelude::*, SqliteConnection};
use sqlite_lunatic_example::{
    establish_connection, get_input,
    models::{self, NewPostTopic, PostTopic},
    schema::{post_topics, posts::dsl::posts, topics::dsl as topics_dsl},
};
use std::env::args;

pub fn tag_post(
    mut conn: SqliteConnection,
    post_id: i32,
    topic: String,
) -> Result<(), diesel::result::Error> {
    let topic_id = if let Ok(topic) = topics_dsl::topics
        .filter(topics_dsl::name.eq(&topic))
        .first::<models::Topic>(&mut conn)
    {
        topic.id
    } else {
        diesel::insert_into(topics_dsl::topics)
            .values(topics_dsl::name.eq(&topic))
            .returning(topics_dsl::id)
            .get_result::<i32>(&mut conn)?
    };
    println!("GOT TOPIC_ID {}", topic_id);

    let post_topic = NewPostTopic { post_id, topic_id };
    println!("INSERTING POST_TOPIC {:?}", post_topic);
    diesel::insert_into(post_topics::table)
        .values(&post_topic)
        .execute(&mut conn)
        .expect("Error saving new post");

    Ok(())
}

fn main() {
    let post_id = args()
        .nth(1)
        .expect("tag_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let mut connection = establish_connection();

    let post = posts
        .find(post_id)
        .first::<models::Post>(&mut connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", post_id));

    let topic_name = get_input(format!(
        "What topic/tag do you want to assign to the post {}?",
        post.title
    ));

    tag_post(connection, post_id, topic_name.clone()).expect("should have tagged post");

    println!("Tagged post {} with topic {}", post.title, topic_name);
}
