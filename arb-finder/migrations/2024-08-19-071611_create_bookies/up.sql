-- Your SQL goes here
CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  book_key TEXT NOT NULL,
  book_title TEXT NOT NULL,
  region TEXT NOT NULL,
  is_exchange BOOLEAN NOT NULL DEFAULT FALSE,
  UNIQUE (book_key, region)
)
