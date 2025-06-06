# ATREA

A unofficial 3rd party alternate [twitch](https://twitch.tv) recommendation algorithm for nerds.

Why for nerds? because its "hard" to run and works best with some sort of home-lab.  
It is also not viable to host it for the masses thanks to [rate limits](https://dev.twitch.tv/docs/chat/#rate-limits) (and thus not being designed for it).

![screenshot](https://jan9103.github.io/atrea/media/00.avif)

## Docs

Getting started
* [quick start](./docs/quick_start.md)
* [slow start](./docs/slow_start.md)

Contributing / Extending:
* [plugin creation guide](./docs/creating_plugins.md)

## Project status

* Currently it is in alpha (all the base-functionality exists).
* Since i have everything i personally need my focus has shifted (for now).

## Feature Highlights

* Multiple recommendation algorithms
* Channel view (raids, shoutouts, viewers you might know, etc)
* Force graphs for connections
* Plugin system + A few official plugins:
  * More algorithms
  * Integration with things like [ganymede](https://github.com/Zibbp/ganymede), [pronouns.alejo.io](https://pr.alejo.io/), [twitchtracker.com](https://twitchtracker.com), etc
  * Design adjustments

### Planned things

* Easier installation
* UI improvements (more links, nicer looking, more "load more", loading symbols, etc)
* Collect more data (subscribers, VIPs, MODs, etc)
* More / Better Algorithms
* Stabilisation of API
* More caching, Incremental conversions, etc
* Etc

There is a bunch more ideas and details, but i don't want to nail that down.

## Legal

**Credits:**

* [credits.html](atrea-webui/src/html/box_help_credits.html)

**Disclaimer:**

This application is not affiliated with or endorsed by Twitch Interactive (a subsidiary of Amazon.com, Inc.) or any other entity associated with the Twitch platform.
It is a third-party tool created as a hobby project and for informational purposes only.

**Important Note:**

* This application uses publicly available data from Twitch's public API, which is subject to change.
* Any trademarks, logos, or branding used within this application are owned by their respective entities (including Twitch Interactive) and are used solely for identification and information purposes.

Your use of this application is at your own risk. The author(s) disclaim any liability or responsibility for any damages resulting from the use of this application.

**This tool is not intended to:**

* Interact with private user data
* Engage in copyright infringement
* Provide unauthorized access to restricted Twitch features
