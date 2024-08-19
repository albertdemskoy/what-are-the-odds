-- Your SQL goes here
CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  sport_id INT NOT NULL,
  home_team TEXT NOT NULL,
  away_team TEXT NOT NULL,
  commence_time TIMESTAMP NOT NULL,
  UNIQUE (sport_id, home_team, away_team, commence_time),
  CONSTRAINT fk_sport FOREIGN KEY(sport_id) REFERENCES sports(id)
)
