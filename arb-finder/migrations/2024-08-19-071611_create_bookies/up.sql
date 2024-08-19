-- Your SQL goes here
CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  book_key TEXT NOT NULL,
  book_title TEXT NOT NULL,
  region TEXT NOT NULL,
  UNIQUE (book_key),
  is_exchange BOOLEAN DEFAULT FALSE
)
