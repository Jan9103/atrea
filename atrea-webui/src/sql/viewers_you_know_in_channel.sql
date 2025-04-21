-- parameters: channel, limit, offset
SELECT jc.viewer AS channel, viewer_score.score AS shared_channel_count
FROM join_counts jc
INNER JOIN (
    SELECT viewer, COUNT(*) AS score
    FROM join_counts jc
    INNER JOIN liked_channels lc ON lc.name = jc.target
    WHERE count > 2
    GROUP BY viewer
) viewer_score ON jc.viewer = viewer_score.viewer
WHERE jc.count > 2
AND target = ?
ORDER BY score DESC
LIMIT ? OFFSET ?;
