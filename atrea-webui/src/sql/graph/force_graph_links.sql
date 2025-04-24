SELECT
  JSON_OBJECT(
  	'source', rc.raider,
  	'target', rc.target,
  	'raidcount', rc.raid_count,
  	'avg_size', rc.average_raid_size
  )
FROM raid_connections rc;
