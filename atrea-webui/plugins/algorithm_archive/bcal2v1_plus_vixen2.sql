WITH level1 (channel,score) AS (
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
    SELECT t2.channel AS channel, SUM(t2.score) AS score
    FROM (
        SELECT channel, score FROM incomming_love
        UNION
        SELECT channel, score FROM outgoing_love
    ) t2
    WHERE t2.channel NOT IN (SELECT name FROM liked_channels)
    GROUP BY t2.channel
), incomming_love_l2 (channel,score) AS (
    SELECT t4.channel AS channel, SUM(t4.score) AS score
    FROM (
        SELECT r1.channel AS channel, r1.lc AS lc, r1.score AS score
        FROM (
            SELECT rc1.target AS channel, rc1.raider AS lc, l11.score AS score
            FROM raid_connections rc1
            INNER JOIN level1 l11 ON rc1.raider = l11.channel
            UNION
            SELECT sc2.target AS channel, sc2.author AS lc, l12.score AS score
            FROM shoutout_connections sc2
            INNER JOIN level1 l12 ON sc2.author = l12.channel
        ) r1
        GROUP BY channel,lc
    ) t4
    GROUP BY t4.channel
), outgoing_love_l2 (channel,score) AS (
    SELECT t4.channel AS channel, SUM(t4.score) AS score
    FROM (
        SELECT r1.channel AS channel, r1.lc AS lc, r1.score AS score
        FROM (
            SELECT rc1.raider AS channel, rc1.target AS lc, l11.score AS score
            FROM raid_connections rc1
            INNER JOIN level1 l11 ON rc1.target = l11.channel
            UNION
            SELECT sc2.author AS channel, sc2.target AS lc, l12.score AS score
            FROM shoutout_connections sc2
            INNER JOIN level1 l12 ON sc2.target = l12.channel
        ) r1
        GROUP BY channel,lc
    ) t4
    GROUP BY t4.channel
), vixen2 (channel,score) AS (
    WITH vixens (viewer, target, score) AS (
        SELECT
            jc.viewer AS viewer,
            jc.target AS channel,
            IIF(join_weights.score > 1.0, join_weights.score, 1.0) AS score
        FROM join_counts jc
        INNER JOIN (
            SELECT
                jc.target AS channel,
                AVG(jc.count) AS avg_joins,
                (100.0 / SUM(jc.count)) AS score
            FROM join_counts jc
            WHERE jc.count > 2 -- exclude one time joiners from average (scam bots, not interrested, etc)
            GROUP BY jc.target
        ) join_weights ON join_weights.channel = jc.target
        WHERE jc.count > (join_weights.avg_joins * 0.50) -- reduce noise
    )
    SELECT
        v.target AS channel,
        SUM(mv.score) AS score
    FROM vixens v
    INNER JOIN (
        SELECT v.viewer AS viewer, SUM(v.score) AS score
        FROM vixens v
        INNER JOIN (
            SELECT name FROM liked_channels
        ) lc ON v.target = lc.name
        GROUP BY v.viewer
    ) mv ON mv.viewer = v.viewer  -- mv = mutual vixens
    GROUP BY v.target
)

SELECT t3.channel AS channel, SUM(t3.score) + (SUM(v2.score) * 0.01) AS score
FROM (
    SELECT channel, score * 1.0 AS score FROM incomming_love_l2
    UNION
    SELECT channel, score * 1.0 AS score FROM outgoing_love_l2
    UNION
    SELECT channel, score * 2.0 AS score FROM level1
) t3
LEFT JOIN vixen2 v2 ON v2.channel = t3.channel
WHERE t3.channel NOT IN (SELECT name FROM liked_channels)
GROUP BY t3.channel

ORDER BY score DESC
LIMIT ? OFFSET ?;
