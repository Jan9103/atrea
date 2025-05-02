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
  "settings": {
    "my_setting": "default_value"
  }
}
```

* the `meta.json` has to contain a `description` string (which can be empty).
* the `settings` key contains a record.
  * the record-key is the setting name (which can be any string, but it is recommended to keep it `^[a-zA-Z0-9-_ ]+$`).
  * the record-value is the default setting-value.
    * values can only be strings. if you need something else parse it at read-time.
  * `"@!my_setting!@"` will be replaced with the value of `my_setting` (example: `"default_value"`) within your `js` before beeing sent to the browser.
