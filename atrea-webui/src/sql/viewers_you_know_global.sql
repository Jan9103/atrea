-- parameters: limit, offset
WITH join_counts (viewer, target, count) AS (
    SELECT viewer, target, COUNT(*) AS count
    FROM joins
    GROUP BY viewer, target
)
SELECT viewer, COUNT(*) AS score
FROM join_counts jc
INNER JOIN liked_channels lc ON lc.name = jc.target
WHERE count > 2
GROUP BY viewer
ORDER BY score DESC
LIMIT ? OFFSET ?;
