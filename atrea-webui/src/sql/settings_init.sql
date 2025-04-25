CREATE TABLE IF NOT EXISTS plugins (
  plugin_name TEXT,
  description TEXT,
  enabled INTEGER  -- boolean
);

CREATE TABLE IF NOT EXISTS plugin_files (
  file_name TEXT,  -- example: "box_recs.js"
  plugin_name TEXT
);
