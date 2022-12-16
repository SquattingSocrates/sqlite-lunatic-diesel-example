use diesel::{prelude::*, QueryDsl, SqliteConnection, TextExpressionMethods};
use lunatic::{
    process::{AbstractProcess, ProcessRef, Request, RequestHandler, StartProcess},
    Mailbox,
};
use serde::{Deserialize, Serialize};
use sqlite_lunatic_example::{
    create_post, establish_connection, get_input,
    models::{self, NewPost},
    schema::users,
};
use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Serialize, Deserialize)]
struct UserGuard;

struct UserConnection {
    cache: HashMap<String, models::User>,
    connection: SqliteConnection,
}

impl AbstractProcess for UserGuard {
    type Arg = ();
    type State = UserConnection;

    fn init(_: ProcessRef<Self>, _start: ()) -> Self::State {
        Self::State {
            cache: HashMap::new(),
            connection: establish_connection(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserName(String);

impl RequestHandler<UserName> for UserGuard {
    type Response = Option<i32>;

    fn handle(state: &mut Self::State, UserName(user_name): UserName) -> Self::Response {
        if let Some(user) = state.cache.get(&user_name) {
            return Some(user.id);
        }
        match users::dsl::users
            .filter(users::dsl::name.like(&user_name))
            .first::<models::User>(&mut state.connection)
        {
            Ok(user) => {
                let user_id = user.id;
                state.cache.insert(user_name, user);
                Some(user_id)
            }
            Err(_) => None,
        }
    }
}

trait UserGuardHandler {
    fn lookup_user(&mut self, user_name: String) -> Option<i32>;
}

impl UserGuardHandler for ProcessRef<UserGuard> {
    fn lookup_user(&mut self, user_name: String) -> Option<i32> {
        let user_name = user_name.trim().to_string();
        let request = UserName(user_name);
        self.request(request)
    }
}

#[lunatic::main]
fn main(_: Mailbox<()>) {
    let mut user_guard = UserGuard::start_link((), None);
    let connection = &mut establish_connection();
    let user_name = get_input("What is your username?");

    let user_id = user_guard
        .lookup_user(user_name)
        .unwrap_or_else(|| panic!("Failed to find user by such name"));

    // get user by username
    let title = get_input("What would you like your title to be?");
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();
    // let body = body.replace("'", "''");

    let new_post = NewPost {
        title,
        body: &body,
        user_id,
    };
    let _ = create_post(connection, new_post);
    println!("\nSaved draft {}", title);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
