CREATE TYPE res_mode AS ENUM ('fixup', 'vx', 'fx');

CREATE TABLE modes (
    guild_id VARCHAR(20),
    user_id VARCHAR(20),
    mode res_mode,
    PRIMARY KEY (guild_id, user_id)
);
