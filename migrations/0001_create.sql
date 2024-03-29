CREATE TABLE IF NOT EXISTS boards (
    id INTEGER PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    lists_order JSON DEFAULT "[]" NOT NULL
);


CREATE TABLE IF NOT EXISTS lists (
    id INTEGER PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    cards_order JSON DEFAULT "[]" NOT NULL
);

CREATE TABLE IF NOT EXISTS cards (
    id INTEGER PRIMARY KEY NOT NULL,
    title VARCHAR(250) NOT NULL,
    list_id INTEGER NOT NULL,
    FOREIGN KEY (list_id) REFERENCES lists (id)
);
