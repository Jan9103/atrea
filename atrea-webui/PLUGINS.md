# Plugins

## File structure

```
plugin_name
|- meta.json
|- index.css
|- box_channel.js
```

* each plugin is a single directory.
* each plugin needs a `meta.json` file.
* to add `js` code to a `html` file name it `<html_name>.js`
* to add `css` to a `html` file name it `<html_name>.css`

## `meta.json`

```json
{
  "description": "My first plugin",
  "category": "design",
  "settings": {
    "my_setting": "default_value"
  },
  "algorithms": [
    {"name": "foo", "description": "my_algorithm", "used_data": "raids", "is_primary": true}
  ]
}
```

* the `meta.json` has to contain a `description` string (which can be empty).
* the optional `settings` key contains a record.
  * the record-key is the setting name (which can be any string, but it is recommended to keep it `^[a-zA-Z0-9-_ ]+$`).
  * the record-value is the default setting-value.
    * values can only be strings. if you need something else parse it at read-time.
  * `"@!my_setting!@"` will be replaced with the value of `my_setting` (example: `"default_value"`) within your `js` before beeing sent to the browser.
* the optional `category` is a string.
  * common keys: `design`, `integration`, `algorithm`, `other`
* the optional `algorithms` is a list of records:
  * `name`: string
    * the algorithm has to be stored in `<name>.sql`
  * `description`: string
  * `used_data`: string: space-seperated list.
    * values: `raids`, `joins`, `shoutouts`
  * `is_primary`: bool: should this be included in the nav-list?
