use std::env::args;

use diesel::prelude::*;
use lunatic::Mailbox;
use sqlite_lunatic_example::{models::Post, *};

// for testing setting of custom allocator
// via host function set_custom_guest_allocator
#[no_mangle]
pub fn __test_set_guest_allocator(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    println!("CUSTOM ALLOCATOR USED for {} bytes", len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

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
