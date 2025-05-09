WITH l2 (login) AS (
  SELECT rc0.raider AS login
  FROM raid_connections rc0
  WHERE rc0.raider == ?
  GROUP BY rc0.raider
  UNION
  SELECT rc1.raider AS login
  FROM raid_connections rc1
  WHERE rc1.target == ?
  GROUP BY rc1.raider
  UNION
  SELECT rc2.target AS login
  FROM raid_connections rc2
  WHERE rc2.raider == ?
  GROUP BY rc2.target
), l3 (login) AS (
  SELECT DISTINCT login FROM (
    SELECT l21.login AS login
    FROM l2 l21
    UNION
    SELECT l22.login AS login
    FROM l2 l22
    UNION
    SELECT rc1.raider AS login
    FROM raid_connections rc1
    WHERE rc1.target IN (SELECT login FROM l2)
    GROUP BY rc1.raider
    UNION
    SELECT rc2.target AS login
    FROM raid_connections rc2
    WHERE rc2.raider IN (SELECT login FROM l2)
    GROUP BY rc2.target
  )
)
SELECT
  JSON_OBJECT(
    'name', du.login,
    'id', du.login,
    'val', 1
  )
FROM l3 du;
