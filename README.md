# `lunatic-diesel-sqlite`

Diesel's `Getting Started` guide using SQLite with the lunatic runtime

```diff
- Please note that you need to have sqlite as well as the lunatic VM installed on your machine in order to use the diesel CLI
```

## Usage

```
$ echo "DATABASE_URL=file:test.db" > .env
$ diesel migration run

$ cargo run --bin show_posts

$ cargo run --bin write_post
# write your post

$ cargo run --bin publish_post 1

$ cargo run --bin show_posts
# your post will be printed here

# Delete post with given title
$ cargo run --bin delete_post "title of post to delete"

$ cargo run --bin show_posts
# observe that no posts are shown
```