WITH raid_weighting (raider, raid_count) AS (
  SELECT
    rc.raider,
    SUM(rc.raid_count) * 1.0 AS raid_count
   FROM raid_connections rc
   GROUP BY rc.raider
),
raid_scores (raider, target, score) AS (
  SELECT
  	rc.raider,
  	rc.target,
  	((rc.raid_count / rw.raid_count) * 10) AS score
  FROM raid_connections rc
  INNER JOIN raid_weighting rw ON rw.raider = rc.raider
)
SELECT
  c.channel AS channel,
  COALESCE(irs.score, 0.0)
    + COALESCE(ors.score, 0.0)
    + COALESCE(vs.score, 0.0)
    AS score
FROM (
  SELECT DISTINCT raider AS channel
  FROM raid_connections rc
  WHERE channel NOT IN (SELECT name FROM liked_channels)
) c
LEFT JOIN (
  SELECT
    rs.target AS channel,
    (
      SUM(rs.score)
      + (COUNT(*) * 10.0)
    ) AS score
   FROM raid_scores rs
   INNER JOIN liked_channels lc ON lc.name = rs.raider
   GROUP BY rs.target
) irs ON c.channel = irs.channel
LEFT JOIN (
  SELECT
    rs.raider AS channel,
    (
      (SUM(rs.score) * 0.25)
      + (COUNT(*) * 2.5)
    ) AS score
  FROM raid_scores rs
  INNER JOIN liked_channels lc ON lc.name = rs.target
  GROUP BY rs.raider
) ors ON c.channel = ors.channel
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
                (100.0 / SUM(jc.count)) AS score
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
) vs ON c.channel = vs.channel
ORDER BY score DESC
LIMIT ? OFFSET ?;
