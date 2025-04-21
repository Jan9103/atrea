-- parameters: limit, offset
SELECT viewer, COUNT(*) AS score
FROM join_counts jc
INNER JOIN liked_channels lc ON lc.name = jc.target
WHERE count > 2
GROUP BY viewer
ORDER BY score DESC
LIMIT ? OFFSET ?;
