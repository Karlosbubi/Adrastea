-- Your SQL goes here
CREATE TABLE IF NOT EXISTS articles (
                    hash TEXT PRIMARY KEY,
                    title TEXT,
                    version INTEGER NOT NULL,
                    path TEXT NOT NULL,
                    feed INTEGER NOT NULL,
                    url TEXT NOT NULL
                )