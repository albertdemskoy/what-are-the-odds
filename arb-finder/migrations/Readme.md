classDiagram
OddsOffering : market_type
OddsOffering : price
OddsOffering : line?
OddsOffering : timestamp
OddsOffering : outcome_key

Event : home_team
Event : away_team
Event : start_time
Event : UNIQUE(home_team, away_team, start_time, sport)
Event --> OddsOffering : has many

Sport : key
Sport : UNIQUE(key)
Sport --> Event : has many
Sport : category
Sport : title
Sport : description

Book : name
Book : key
Book : UNIQUE(key)
Book : region
Book : is_exchange
Book --> OddsOffering : has many
