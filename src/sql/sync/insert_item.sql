INSERT OR REPLACE INTO items (
    title,
    pub_date,
    itunes_author,
    itunes_image,
    itunes_subtitle,
    itunes_summary,
    content,
    itunes_duration,
    guid,
    enclosure
) VALUES (
    ?1,
    ?2,
    ?3,
    ?4,
    ?5,
    ?6,
    ?7,
    ?8,
    ?9,
    ?10
);
