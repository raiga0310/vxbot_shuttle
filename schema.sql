DROP TABLE IF EXISTS modes;

CREATE TABLE modes (
    guild_id VARCHAR(20),
    user_id VARCHAR(20),
    mode VARCHAR(5),
    PRIMARY KEY (guild_id, user_id)
);
