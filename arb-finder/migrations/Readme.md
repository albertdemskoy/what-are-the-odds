classDiagram
Odds : market_key
Odds : price
Odds : line?
Odds : timestamp
Odds : outcome_key

Event : home_team
Event : away_team
Event : start_time
Event --> Odds : has many

Sport : key
Sport --> Event : has many
Sport : category
Sport : title
Sport : description

Bookie : name
Bookie : key
Bookie : region
Bookie --> Odds : has many
