CREATE TABLE entries(
  id SERIAL PRIMARY KEY,
  location VARCHAR NOT NULL,
  utc_date TEXT NOT NULL,
  temperature FLOAT NOT NULL
)