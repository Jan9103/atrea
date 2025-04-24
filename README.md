# ATREA

A unofficial 3rd party alternate [twitch](https://twitch.tv) recommendation algorythm for nerds.

Why for nerds? because its "hard" to run and works best with some sort of homelab.  
It is also not viable to host it for the masses thanks to [rate limits](https://dev.twitch.tv/docs/chat/#rate-limits).

![screenshot](https://jan9103.github.io/atrea/media/00.avif)

## Project status

* Currently it is in alpha (the base-functionality exists in a buggy state).
* Due to the problems with the concept and the early stage of the project i might abandon it.
* Development has been pretty slow so far.

## How does it work?

It consists of 3 parts:
* atrea-collector:
  * has to run in the background (on a server) for a few days.
  * collects information from twitch about raids, shoutouts, shared viewers, etc.
  * extremely optimized in order to use as few resources as possible.
  * how it operates:
    * it initially joins all liked channels and reads their chat, raids, etc.
    * it joins any channel raiding a joined channel (within parameters).
      * yes this is recursive and exponential growth, but i haven't encountered any problems so far when i restart it once per week.
      * sadly twitch only informs about incomming and not outgoing raids.
* atrea-converter:
  * transforms raw data into a usable format (sqlite).
  * fetches extra-information, such as profile pictures, channel descriptions, etc.
  * caches some expensive calculations.
  * why is this an extra step?
    * i can't reliable run anything on `atrea-webui` startup.
    * `nu` makes this way easier than `rust`.
    * `atrea-collector` should be as lightweight as possible
      * sqlite can use multiple gb of ram, etc (and csv takes almost nothing).
      * sqlite, etc are pretty huge dependencies.
      * it has to run some things after shutdown (mainly the caching).
* atrea-webui:
  * a webserver for you to look at in the end.
  * runs the actual recommendation algorithms.

## Usage

### Getting the binaries

For now you will have to compile them yourself.

If you are on windows use either `nu` or `git-bash` as your shell.

1. install rust ([via rustup](https://rustup.rs/))
1. install `git` (it should be available for all package-managers)
1. install `nu` ([tutorial](https://www.nushell.sh/book/installation.html))
1. open a terminal and run the following commands (in order):
  1. `git clone https://github.com/jan9103/atrea`
  1. `cd atrea/atrea-webui`
  1. `cargo build --release`
  1. `cd ../atrea-webui`
  1. `cargo build --release`

You can now find the binaries at:
* `your home directory/atrea/atrea-collector/target/release/atrea-collector`
* `your home directory/atrea/atrea-converter/convert.nu` (this one is a script)
* `your home directory/atrea/atrea-webui/target/release/atrea-webui`

### Global configuration

You should first define a list of channels you like in a file.  
Each line should contain the `login` of one channel.  
You can find the `login` by opening their channel and looking at the url.  
Example: the login for the url `https://www.twitch.tv/princessxen` is `princessxen`

Example liked channels file contents:
```
princessxen
brodieongames
dove_7
```

### Collecting data

Usage:
```
atrea collector

USAGE:
    atrea-collector [flags]

FLAGS:
    -c, --channel-list-file PATH    location of the channel-list (newline seperated list)
    -j, --log-joins                 log join infos
    -s, --log-shoutouts             log shoutouts
        --min-raidsize      NUMBER  minimum size of raids to follow (default: 3)
        --max-raidsize      NUMBER  maximum size of raids to follow (default: infinite)
        --stdout-sent               debug-log sent things via stdout
        --stdout-recieved           debug-log recieved things via stdout
```

Example: `atrea-collector -c ./liked_channels.txt --log-joins --log-shoutouts`

Explaining each argument:
* `--channel-list-file`: A filepath to the file we created in the `global configuration` section.
* `--log-joins`: Enable viewer-join tracking (disabled by default).
  * This generates a lot of data
    * About 150 times as much as the raid tracing for me
    * It is still manageable (especially if you specify a `--max-raidsize`) and usually stays in the `kb/week` area.
* `--log-shoutouts`: Enable shoutout tracking (disabled by default)
  * This tracks `!so` and `!shoutout` messages from moderators
  * It is usually not much output data, but more cpu usage since the bot now has to read chat.
* `--min-raidsize`: Change the minimum size of a raid for it to be followed.
  * i have it set to 3 since i prefer smaller channels with sub 100 viewers.
  * if you prefer 1k+ channels you might want to set it too 100 or similar.
* `--max-raidsize`: Change the maximum size of a raid for it to be followed.
  * by default there is (essentially) no limit.
  * if you like me prefer small streams you can save a lot of resourced by settign it to 200 or something and thus don't end up logging the chats of `vedal987`, `cdawgva`, and other giga-streamers.
* `--stdout-sent` and `--stdout-recieved`: these are ment for developers.


How to run it:
* Dont restart it to often.
  * Revers-raid data usually takes one or two days to start popping up.
* Restart it regularly:
  * I restart mine once a week, but once a month might be better.
  * It experiences exponential growth (in data collected, cpu usage, and network transfer amount).
    * I have let it run for over a month on a raspberry-pi-4 without issues, but you will hit a wall at some point.
  * The longer it runs the more data gets caught and the useless data portion grows faster than the useful portion.
* When does it have enough data?
  * Depends on your settings, goal, and channels (raid-frequency, stream-frequency, size, etc).
  * I would say after a week you should have a decent dataset in most cases.
  * If your channels are very active (in total) you might already have a decent result after a day.
* When is it finished?
  * The data will always improve, but after 3months it will probably appear almost frozen.
  * If you found new channels you can add them to the list.
  * Streamers find new friends all the time, but it might take some time to sicker through to here.
* Feel free to compress the csv-results of finished executions.
  * supported: no compression, `gz`, `xz`, and `bz2`



### Converting data

```
Required environmental variables:
* TWITCH_CLIENT_ID and TWITCH_CLIENT_SECRET ( https://dev.twitch.tv/docs/authentication/register-app/ )

Usage:
  > convert.nu {flags}

Flags:
  --collected-data-dir <path> (default: './collected_data')
  --output-sqlite <path> (default: './atrea_db.sqlite')
  --liked-channel-file <path> (default: './liked_channels.txt')
  -h, --help: Display the help message for this command
```

Explanation of flags:
* `--collected-data-dir`: path to the directory containing the data collected by `atrea-collector` (a bunch of `.csv` files; usually the directory you executed it in)
* `--output-sqlite`: path to where it should store the `sqlite` file containing the result.
* `--liked-channel-file`: path to the file we set up in the `global configuration` chapter.

Explanation of env vars:
* You can get the values for `TWITCH_CLIENT_ID` and `TWITCH_CLIENT_SECRET` at <https://dev.twitch.tv/console> ([tutorial](https://dev.twitch.tv/docs/authentication/register-app/))
  * These get used to fetch extra information, such as profile-pictures from the twitch-API

Warning:
* Im not sure if the webui survives a live-replacement of its sqlite file. so just stop it beforehand.
* This never happens automatically. you will have to rerun it to update the data seen in the webui.



### Displaying the data (aka webui)

* The sqlite file from the conversion step has to be at `./atrea_db.sqlite` ([rocket](https://rocket.rs/guide) seems to not allow dynamic selection..)

Environmental variables:

* `ROCKET_ADDRESS` (default is for me `0.0.0.0`): The address to bind to
* `ROCKET_PORT` (default is for me `8000`): The port to use

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
