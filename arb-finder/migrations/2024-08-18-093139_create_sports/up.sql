-- Your SQL goes here
CREATE TABLE sports (
  id SERIAL PRIMARY KEY,
  category VARCHAR NOT NULL,
  sport_key VARCHAR NOT NULL,
  title varchar NOT NULL
)