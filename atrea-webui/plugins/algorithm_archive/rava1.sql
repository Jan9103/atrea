SELECT cl.channel AS channel,
    (
        (COALESCE(favs_raided.score, 0) * 10)
        + (COALESCE(favs_raid_count.score, 0) * 1)
        + (COALESCE(raid_by_favs_count.score, 0) * 1.5)
        + (COALESCE(vixen_score.score, 0) * 1)
    ) AS score
FROM (
    SELECT DISTINCT raider AS channel
    FROM raid_connections
    WHERE channel NOT IN (SELECT name FROM liked_channels)
) cl

-- how many of the favorites did they raid?
LEFT JOIN (
    SELECT rc.raider AS channel, COUNT(rc.raider) as score
    FROM raid_connections rc
    INNER JOIN liked_channels lc ON lc.name=rc.target
    GROUP BY rc.raider
) favs_raided ON cl.channel=favs_raided.channel

-- how often do they raid favorites?
-- TODO: weight based on total raids in the favorite and to preevnt scewing over time
LEFT JOIN (
    SELECT rc.raider AS channel, SUM(raid_count) AS score
    FROM raid_connections rc
    INNER JOIN liked_channels lc ON lc.name=rc.target
    GROUP BY rc.raider
) favs_raid_count ON cl.channel=favs_raid_count.channel

-- how often do they get raided by a favorite?
-- TOOD: weight to prevent scewing over time
LEFT JOIN (
    SELECT rc.target AS channel, SUM(raid_count) AS score
    FROM raid_connections rc
    INNER JOIN liked_channels lc ON lc.name=rc.raider
    GROUP BY rc.target
) raid_by_favs_count ON cl.channel=raid_by_favs_count.channel

-- shared viewers
LEFT JOIN (
    WITH vixens (viewer, target, score) AS (
        SELECT
            jc.viewer AS viewer,
            jc.target AS channel,
            -- jc.count AS count,
            join_weights.score AS score
        FROM join_counts jc
        INNER JOIN (
            SELECT
                jc.target AS channel,
                -- SUM(jc.count) AS total_joins,
                AVG(jc.count) AS avg_joins,
                (100 / SUM(jc.count)) AS score
            FROM join_counts jc
            WHERE jc.count > 2 -- exclude one time joiners from average (scam bots, not interrested, etc)
            GROUP BY jc.target
        ) join_weights ON join_weights.channel = jc.target
        WHERE jc.count > (join_weights.avg_joins * 0.75) -- reduce noise
    )
    SELECT
        v.target AS channel,
        mv.score + v.score AS score
    FROM vixens v
    INNER JOIN (
        SELECT v.viewer AS viewer, SUM(v.score) AS score
        FROM vixens v
        INNER JOIN (
            SELECT name FROM liked_channels
        ) lc ON v.target = lc.name
        GROUP BY v.viewer
    ) mv ON mv.viewer = v.viewer  -- mv = mutual vixens
    WHERE v.target NOT IN (SELECT name FROM liked_channels)
    GROUP BY v.target
) vixen_score ON cl.channel = vixen_score.channel

ORDER BY score DESC
LIMIT ? OFFSET ?;
