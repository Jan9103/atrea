-- how many liked channels have shouted x out
-- sometimes shoutouts are accidental (wrong), etc. so a second shoutout from the same liked_channel adds 0.5 score

SELECT
	target AS channel,
	SUM(CASE sc.shoutout_count WHEN 0 THEN 0.0 WHEN 1 THEN 1.0 ELSE 1.5 END) AS score
FROM shoutout_connections sc 
INNER JOIN liked_channels lc ON lc.name = sc.author
GROUP BY sc.target
ORDER BY score DESC
LIMIT 20 OFFSET 0;
