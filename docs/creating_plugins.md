# Creating plugins

## File Structure

A plugin consists of multiple files in a directory.

The only required file is a `meta.json`.

All other files have to be placed next to it.

## Base meta.json

```json
{
  "description": "Some description",
  "category": "design"
}
```

`description` is the text shown below the name in the `plugins` page.

`category` is for sorting it in the plugins page.
It can theoretically be anything, but it is recommended to use one of:
* `design` (changing colors, rounding profile pictures, etc)
* `integration` (add data from external services, link to them, etc)
* `algorithm` (adding algorithms)
* `other`

## Adding a Algorithm

### `meta.json`:

```json
{
  "algorithms": [
    {
      "name": "algorithm_name",
      "description": "A variant of algorthm x finding smaller streamers",
      "used_data": "raids shoutouts",
      "is_primary": true
    }
  ]
}
```

* **name:** The name of the algorithm
* **description:** A description
* **used_data:** A space separated list of data used by this algorithm.
  * Available options: `raids`, `joins`, `shoutouts`
* **is_primary:** Should this be treated as a primary algorithm?
  * Primary algorithms show up in the navigation. Other algorithms only in the algorithm list.

### SQL file

ATREA uses the [sqlite flavour of SQL](https://sqlite.org/lang.html).  
You can find a tutorial [here](https://www.sqlitetutorial.net/sqlite-cheat-sheet/).  
And might want to take inspiration from [the official algorithms](https://github.com/Jan9103/atrea/tree/main/atrea-webui/src/sql/recs) and [algorithm archive](https://github.com/Jan9103/atrea/tree/main/atrea-webui/plugins/algorithm_archive).

The algorithm should be stored in a file called `<algorithm_name>.sql` (if it is called `bob1` the file should be `bob1.sql`).

The algorithm gets provided 2 parameters:
1. `LIMIT`
2. `OFFSET`

It should also be ordered by score (highest score first).

This can easily be fulfilled by just adding the following to the end:
```sql
ORDER BY score DESC
LIMIT ? OFFSET ?
```

The query result should return 2 columns:
1. (STRING) the channel being evaluated (called `channel` and being the first column)
2. (FLOAT) the score given to the channel (called `score` and being the second column)

So a extremely basic algorithm would be:  
(this counts how many liked channels have raided a channel)

```sql
SELECT
  r.target AS channel,
  COUNT(*) * 1.0 AS score  -- * 1.0 turns the INT into a FLOAT
FROM raid_connections r
WHERE r.raider IN (SELECT name FROM liked_channels)
GROUP BY r.target
ORDER BY score DESC
LIMIT ? OFFSET ?
```

## Changing the UI

It is possible to inject `js` or `css` into the [base files](https://github.com/Jan9103/atrea/tree/main/atrea-webui/src/html).  
For this add a file called `<original_html_name>.<js or css>` (example: `index.css` for injecting `css` into `index.html` OR `box_channel.js` to inject `js` into `box_channel.html`).

If a plugin with similar functionality exists it might save you a lot of work by just copying and adjusting ("forking") it.  
You can find the plugin code [here](https://github.com/Jan9103/atrea/tree/main/atrea-webui/plugins).

### Settings

`meta.json`:
```json
{
  "settings": {
    "setting_name": "default_value"
  }
}
```

Settings are always strings. If you need something else: convert it at runtime.

You can access the setting values using `@!setting_name!@` in the code.  
The value gets escaped as a string, but delivered without quotes. This way your editor should highlight (mis-)use properly.  
This will just be replaced before getting sent to the browser.

Example usage:

```js
let columns = int("@!column_count!@");
my_node.innerText = "Hello @!user_name!@";
```

### ATREA's internal communication (opening windows, etc)

ATREA uses internal [message channels](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel).  
You can find the reader (and thus command list) [here](https://github.com/Jan9103/atrea/blob/main/atrea-webui/src/html/index_mailbox.js).  

All messages are `json` records with a `action` field containing the base action.

`./atrea.js` ([code](https://github.com/Jan9103/atrea/blob/main/atrea-webui/src/html/atrea.js)) also contains helper functions.

### Creating elements

* wait for the DOM - otherwise you might get random errors:
  * `libs/xeact.js`: `r(()=>{YOUR CODE HERE})`
* `document.getElementById(id)` (`libs/xeact.js`: `g`)
* `document.createElement(node_type_name)` (`atrea.js`: `n`)
* `document.createTextNode(text)` (`libs/xeact.js`: `t`)
* delete all child nodes: `libs/xeact.js`: `x(node)`

### Handling errors

ATREA has custom functions to show the error to the user:

* `atrea.js`: `cr` (**C**heck fetch **R**esponse)
  * This assumes every non-200 response to be a unexpected error. No idea why JS usually has no checks whatsoever.
  * `fetch("foo").then(cr).WHATEVER_YOU_USUALLY_DO`
* `atrea.js`: `ce` (**C**atch **E**rror handler)
  * `.catch(ce)` (mainly for `fetch[..].catch(ce)`)
* `atrea.js`: `show_error`
  * `try { something } catch (error) {show_error(error);}`
