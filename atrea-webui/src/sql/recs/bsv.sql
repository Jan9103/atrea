SELECT target AS channel, SUM(viewer_score.score) * 1.0 AS score
FROM join_counts jc
INNER JOIN (
    SELECT viewer, COUNT(*) AS score
    FROM join_counts jc
    INNER JOIN liked_channels lc ON lc.name = jc.target
    WHERE count > 3
    GROUP BY viewer
) viewer_score ON jc.viewer = viewer_score.viewer
WHERE jc.count > 3
AND target NOT IN (SELECT name FROM liked_channels)
GROUP BY target
ORDER BY score DESC
LIMIT ? OFFSET ?;
