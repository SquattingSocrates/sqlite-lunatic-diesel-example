use diesel::{prelude::*, QueryDsl, SqliteConnection, TextExpressionMethods};
use lunatic::{
    abstract_process,
    process::{AbstractProcess, ProcessRef, Request, RequestHandler, StartProcess},
    Mailbox,
};
use serde::{Deserialize, Serialize};
use sqlite_lunatic_example::{
    create_post, establish_connection, get_input,
    models::{self, NewPost},
    schema::topics::dsl as topics_dsl,
};
use std::{
    collections::HashMap,
    io::{stdin, Read},
};

struct SqliteProcess {
    connection: SqliteConnection,
}

#[abstract_process]
impl AbstractProcess for SqliteProcess {
    // type Arg = ();
    // type State = UserConnection;

    #[init]
    fn init(_: ProcessRef<Self>, _start: ()) -> Self {
        Self {
            // cache: HashMap::new(),
            connection: establish_connection(),
        }
    }

    #[handle_request]
    fn create_topic(&mut self, topic: String) -> i32 {
        if let Ok(topic) = topics_dsl::topics
            .filter(topics_dsl::name.eq(&topic))
            .first::<models::Topic>(&mut self.connection)
        {
            topic.id
        } else {
            diesel::insert_into(topics_dsl::topics)
                .values(topics_dsl::name.eq(&topic))
                .returning(topics_dsl::id)
                .get_result::<i32>(&mut self.connection)
                .expect("failed to insert new topic")
        }
    }
}

#[lunatic::main]
fn main(_: Mailbox<()>) {
    let procs: Vec<ProcessRef<SqliteProcess>> = (0..3)
        .map(|_| SqliteProcess::start_link((), None))
        .collect();

    for (idx, proc) in procs.iter().enumerate() {
        let user_id = proc.create_topic(format!("test{}", idx));
        println!("GOT THIS TOPIC {:?} | {:?}", idx, user_id);
    }
    // let user_id_1 = second_process.create_topic("test2");
    // let user_id_2 = third_process.create_topic(user_name);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
