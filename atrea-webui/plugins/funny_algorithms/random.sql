SELECT
    rc.raider AS channel,
    1.0 AS score
FROM raid_connections rc
WHERE rc.raider NOT IN (SELECT name FROM liked_channels)
ORDER BY RANDOM()
LIMIT ? OFFSET ?;
