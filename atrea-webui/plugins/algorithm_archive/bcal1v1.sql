WITH incomming_love (channel,score) AS (
    SELECT t1.channel AS channel, COUNT(t1.channel) AS score
    FROM (
        SELECT r1.channel AS channel, r1.lc AS lc
        FROM (
            SELECT rc1.target AS channel, rc1.raider AS lc
            FROM raid_connections rc1
            INNER JOIN liked_channels lc1 ON lc1.name = rc1.raider
            UNION
            SELECT sc2.target AS channel, sc2.author AS lc
            FROM shoutout_connections sc2
            INNER JOIN liked_channels lc2 ON lc2.name = sc2.author
        ) r1
        GROUP BY channel,lc
    ) t1
    GROUP BY t1.channel
), outgoing_love (channel,score) AS (
    SELECT t1.channel AS channel, COUNT(t1.channel) AS score
    FROM (
        SELECT r2.channel AS channel, r2.lc AS lc
        FROM (
            SELECT rc1.target AS channel, rc1.raider AS lc
            FROM raid_connections rc1
            INNER JOIN liked_channels lc1 ON lc1.name = rc1.raider
            UNION
            SELECT sc2.author AS channel, sc2.target AS lc
            FROM shoutout_connections sc2
            INNER JOIN liked_channels lc2 ON lc2.name = sc2.target
        ) r2 GROUP BY channel,lc
    ) t1 GROUP BY t1.channel
)
SELECT t2.channel AS channel, SUM(t2.score) * 1.0 AS score
FROM (
    SELECT channel, score FROM incomming_love
    UNION
    SELECT channel, score FROM outgoing_love
) t2
WHERE t2.channel NOT IN (SELECT name FROM liked_channels)
GROUP BY t2.channel


ORDER BY score DESC
LIMIT ? OFFSET ?;

