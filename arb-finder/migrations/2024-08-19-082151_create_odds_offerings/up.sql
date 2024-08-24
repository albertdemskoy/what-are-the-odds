-- Your SQL goes here
CREATE TABLE odds_offering (
  id SERIAL PRIMARY KEY,
  event_id INT NOT NULL,
  book_id INT NOT NULL,
  offered_at TIMESTAMPTZ NOT NULL,
  outcome TEXT NOT NULL,
  market_type TEXT NOT NULL,
  offered_line NUMERIC(5,2),
  offered_odds NUMERIC(5,2) NOT NULL,
  CONSTRAINT fk_event FOREIGN KEY(event_id) REFERENCES events(id),
  CONSTRAINT fk_book FOREIGN KEY(book_id) REFERENCES books(id)
)
