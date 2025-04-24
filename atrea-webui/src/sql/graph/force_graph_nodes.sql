SELECT
  JSON_OBJECT(
  	'name', du.name,
  	'id', du.name,
  	--'val', SQRT(COALESCE(rs.avg_size, 0)) / 2.0
    'val', IIF(du.name in (SELECT name FROM liked_channels), 5, 1),
    'color', IIF(du.name in (SELECT name FROM liked_channels), "#f00", "#aa0")
  )
FROM (
  --SELECT DISTINCT name
  --FROM (
  --  SELECT DISTINCT rc.raider AS name
  --  FROM raid_connections rc
  --  JOIN (
  --    SELECT rc2.target AS name
  --    FROM raid_connections rc2
  --  ) rc2 ON rc.raider = rc2.name
  --)
  SELECT f.raider AS name FROM (
    SELECT rc1.raider
    FROM raid_connections rc1
    GROUP BY rc1.raider
    UNION
    SELECT target
    FROM raid_connections rc2
    GROUP BY rc2.target
  ) f
) du
LEFT JOIN (
  SELECT
    rc.raider AS raider,
    AVG(rc.total_viewers) AS avg_size
  FROM raid_connections rc
  GROUP BY rc.raider
) rs ON du.name = rs.raider;
