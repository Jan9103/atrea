CREATE TABLE IF NOT EXISTS plugins (
  plugin_name TEXT,
  description TEXT,
  category TEXT,
  enabled INTEGER  -- boolean
);

CREATE TABLE IF NOT EXISTS plugin_files (
  file_name TEXT,  -- example: "box_recs.js"
  plugin_name TEXT
);

CREATE TABLE IF NOT EXISTS plugin_settings (
  plugin_name TEXT,
  setting_key TEXT,
  setting_value TEXT,
  setting_default TEXT
);

CREATE TABLE IF NOT EXISTS plugin_algorithms (
  plugin_name TEXT,
  algorithm_description TEXT,
  algorithm_name TEXT,
  is_primary INT,
  used_data TEXT  -- space-seperated
);
