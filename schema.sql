CREATE TABLE modes (
    guild_id VARCHAR(20),
    user_id VARCHAR(20),
    mode ENUM ("fixup", "fx", "vx"),
    PRIMARY KEY (guild_id, user_id)
)
