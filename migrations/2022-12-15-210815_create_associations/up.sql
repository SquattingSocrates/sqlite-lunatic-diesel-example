CREATE TABLE users (
  id INTEGER NOT NULL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0,
  user_id INTEGER NOT NULL,

  FOREIGN KEY (user_id)
    REFERENCES users (id)
);

CREATE TABLE topics (
  id INTEGER NOT NULL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE post_topics (
  id INTEGER NOT NULL PRIMARY KEY,
  post_id INTEGER NOT NULL,
  topic_id INTEGER NOT NULL,

  FOREIGN KEY (post_id)
    REFERENCES posts (id),

  FOREIGN KEY (topic_id)
    REFERENCES topics (id)
);