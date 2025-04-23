SELECT name AS channel, 0.0 AS score
FROM liked_channels
ORDER BY name ASC
LIMIT ? OFFSET ?;
