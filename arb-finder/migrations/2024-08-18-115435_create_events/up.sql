-- Your SQL goes here
CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  sport_id INT NOT NULL,
  home_team TEXT NOT NULL,
  away_team TEXT NOT NULL,
  commence_time TIMESTAMP NOT NULL,
  CONSTRAINT fk_sport FOREIGN KEY(sport_id) REFERENCES sports(id)
)
