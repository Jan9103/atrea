-- this file is here for documentation purposes

CREATE TABLE [liked_channels] (
    `name` TEXT
);

CREATE TABLE [raids] (
    `timestamp` DATETIME,
    `raider` TEXT,
    `target` TEXT,
    `size` INTEGER
);

CREATE TABLE [joins] (
    `timestamp` DATETIME,
    `viewer` TEXT,
    `target` TEXT
);

-- a cache for improving query-speed (yes im aware all data in here is redundant)
CREATE TABLE [raid_connections] (
    `raider` TEXT,
    `target` TEXT,
    `total_viewers` INTEGER,  -- grand total of viewers sent over
    `raid_count` INTEGER,
    `average_raid_size` INTEGER
);

-- data from the twitch-api
CREATE TABLE [channel_info] (
    `id` INTEGER,
    `login` STRING,  -- this is used in all the other tables (since IRC does not always include the id)
    `display_name` STRING,
    `broadcaster_type` STRING,
    `description` STRING,
    `profile_image_url` STRING,
    `created_at` DATETIME
);
