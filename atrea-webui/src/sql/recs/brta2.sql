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
  COALESCE(irs.score, 0)
    + COALESCE(ors.score, 0)
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
ORDER BY score DESC
LIMIT ? OFFSET ?;
