        SELECT *
        FROM (
            SELECT *,
                   ROW_NUMBER() OVER (PARTITION BY (book_id, outcome, market_type) ORDER BY offered_at DESC) AS rn
            FROM odds_offering
            WHERE event_id = 1
        ) subquery
        WHERE rn = 1
		