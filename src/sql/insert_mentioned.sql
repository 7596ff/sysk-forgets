INSERT OR REPLACE INTO mentioned_items (
    mentioned_title,
    mentioned_guid,
    contained_episode,
    contained_guid,
    pub_date
) VALUES (
    ?1,
    ?2,
    ?3,
    ?4,
    ?5
);
