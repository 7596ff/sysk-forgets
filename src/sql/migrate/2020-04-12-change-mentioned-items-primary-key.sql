BEGIN;

CREATE TABLE IF NOT EXISTS mentioned_items_copy (
    mentioned_title TEXT,
    mentioned_guid TEXT,
    contained_episode TEXT,
    contained_guid TEXT,
    pub_date INTEGER PRIMARY KEY
);

INSERT INTO mentioned_items_copy (
    mentioned_title, mentioned_guid, contained_episode, contained_guid, pub_date
) SELECT 
    mentioned_title, mentioned_guid, contained_episode, contained_guid, pub_date
FROM mentioned_items;

DROP TABLE mentioned_items;
ALTER TABLE mentioned_items_copy RENAME TO mentioned_items;

COMMIT;
