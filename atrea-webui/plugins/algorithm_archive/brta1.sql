-- basic raid trace algorithm v1

-- i use DISTINCT, ON, and WHERE since sql sucks and produces duplicates (even with all active)

SELECT cl.channel AS channel,
    (
        (COALESCE(favs_raided.score, 0) * 10)
        + (COALESCE(favs_raid_count.score, 0) * 1)
        + (COALESCE(raid_by_favs_count.score, 0) * 1.5)
    ) AS score
FROM (
    SELECT DISTINCT raider AS channel
    FROM raid_connections
    WHERE channel NOT IN (SELECT name FROM liked_channels)
) cl
LEFT JOIN (
    SELECT rc.raider AS channel, COUNT(rc.raider) as score
    FROM raid_connections rc
    INNER JOIN liked_channels lc ON lc.name=rc.target
    GROUP BY rc.raider
) favs_raided ON cl.channel=favs_raided.channel
LEFT JOIN (
    SELECT rc.raider AS channel, SUM(raid_count) AS score
    FROM raid_connections rc
    INNER JOIN liked_channels lc ON lc.name=rc.target
    GROUP BY rc.raider
) favs_raid_count ON cl.channel=favs_raid_count.channel
LEFT JOIN (
    SELECT rc.target AS channel, SUM(raid_count) AS score
    FROM raid_connections rc
    INNER JOIN liked_channels lc ON lc.name=rc.raider
    GROUP BY rc.target
) raid_by_favs_count ON cl.channel=raid_by_favs_count.channel
ORDER BY score DESC
LIMIT ? OFFSET ?;
