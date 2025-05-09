WITH l2 (login) AS (
  SELECT rc1.raider AS login
  FROM raid_connections rc1
  WHERE rc1.target == ?
  GROUP BY rc1.raider
  UNION
  SELECT rc2.target AS login
  FROM raid_connections rc2
  WHERE rc2.raider == ?
  GROUP BY rc2.target
)
SELECT
  JSON_OBJECT(
  	'source', rc.raider,
  	'target', rc.target,
  	'raidcount', rc.raid_count,
  	'avg_size', rc.average_raid_size
  )
FROM raid_connections rc
WHERE rc.raider IN (SELECT login FROM l2) OR rc.target IN (SELECT login FROM l2);
