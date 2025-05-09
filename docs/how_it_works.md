# How it works

[back](./index.md)

## ATREA collector

The collector connects to twitch's [IRC](https://en.wikipedia.org/wiki/IRC) server and logs interesting data to [csv](https://en.wikipedia.org/wiki/Comma-separated_values) files.  
The idea is to keep this as efficient as possible in order to make it ATREA easy to deploy on a overloaded raspberry-pi.

Algorithm:
1. Join the IRC
1. Join `liked_channels`
1. Receive messages
  * If it is a raid: log raid and join raiding channel (recursion)
    * Sadly twitch does not inform us about outgoing raids, only incoming
  * If it is a message: check for shoutouts (-> log + join)
  * If it is a join: log

There currently is a plan to move from IRC to [eventsub](https://dev.twitch.tv/docs/eventsub/).


## ATREA converter

`csv` is extremely efficient at being written, but not ideal (or compatible) for reading.  
So the converter converts it to [sqlite3](https://sqlite.org/index.html) and caches a few values.


## ATREA webui (backend aka API)

The webui is build upon the [rocket](https://rocket.rs/) framework, which handles routing, db-connections, etc.  
It serves:
* The frontend (html, css, js, svg)
  * It edits served `html` files to include a import for the plugin-code
* Database query results
  * Sending the whole database to the frontend would be extremely inefficient
  * Examples:
    * Run a recommendation algorithm
    * Get channel details
    * Get raid logs or statistics
    * Generate structure for the force-graph.

## ATREA webui (frontend)

`index.html`:
* The "background" image
* Manages the boxes (opening new ones, etc)
  * controlable by boxes via [message channels](https://developer.mozilla.org/en-US/docs/Web/API/Channel_Messaging_API)
* Build upon [winbox](https://nextapps-de.github.io/winbox/)

Everything else (including the navigation) are separate webpage embedded into `index.html` via [iframes](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe).

Most boxes consist of basic html-structure and a js fetch, which then build the rest of the page based on the API response.

---

[back](./index.md)
