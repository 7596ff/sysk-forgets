BEGIN;

CREATE TABLE IF NOT EXISTS items (
    title TEXT,
    pub_date INTEGER,
    itunes_author TEXT,
    itunes_image TEXT,
    itunes_subtitle TEXT,
    itunes_summary TEXT,
    content TEXT,
    itunes_duration TEXT,
    guid TEXT PRIMARY KEY,
    enclosure TEXT
);

CREATE TABLE IF NOT EXISTS mentioned_items (
    mentioned_title TEXT,
    mentioned_guid TEXT,
    contained_episode TEXT,
    contained_guid TEXT PRIMARY KEY,
    pub_date INTEGER
);

COMMIT;
