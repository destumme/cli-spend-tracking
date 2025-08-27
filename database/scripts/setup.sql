--ATTACH DATABASE 'spending.db' AS spending;
CREATE TABLE IF NOT EXISTS spending.transactions(
    uuid TEXT PRIMARY KEY,
    ignored INTEGER,
    account TEXT,
    date TEXT,
    summary TEXT,
    amount INTEGER,
    category TEXT,
    notes BLOB
);
CREATE TABLE if NOT EXISTS spending.categories(
    uuid TEXT PRIMARY KEY,
    name TEXT,
    date_added TEXT
);