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
WHERE v.target NOT IN (SELECT name FROM liked_channels)
GROUP BY v.target
ORDER BY score DESC
LIMIT ? OFFSET ?;
