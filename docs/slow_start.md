# Slow Start

This tutorial expects you to be using linux. If you are not install [gitbash](https://git-scm.com/downloads/win) and use that.

If you are not familiar with bash you might need some help from a fried who does.

It is also expected for you to use common-sense.  
So if it says `cd /where_ever/you/want/to/store/it` you will have to adjust it.

If you are in a hurry or are experienced use the [quick start](./quick_start.md) and come back here
if you need more info on something specific.

## Target environment / Intended setup

ATREA is split into 2 (+1) parts in order to optimize one specific environment / use-case:
One low-powered device (raspberry pi?) running 24/7 and one high-powered device (your desktop?).

You can run it outside of this, but you will see that it is optimized for that.

The `atrea-collector`, which collects the needed data by running 24/7, is written to be
as efficient as possible.
Due to this targeted efficiency its output-format is `csv` and not a full sqlite-database.

The `atrea-webui` is written to make the browsing experience as good as possible and be easy to edit.  
Thus it uses `sqlite` (a full blown database) with caches as storage format.

To convert from the `csv` to the `sqlite` there is the `atrea-converter`.
The converter has grown a bit over time and also handles things like fetching extra metadata from twitch.

So the target environment is:
* 1 low-powered server (raspberry pi?)
  * runs `atrea-collector` 24/7
* 1 high-powered device (your desktop?)
  * runs `atrea-converter` and `atrea-webui` on demand
* some sort of file-transfer between the two
  * [scp](https://en.wikipedia.org/wiki/Secure_copy_protocol)
  * [ftp](https://en.wikipedia.org/wiki/File_Transfer_Protocol)
  * [rsync](https://en.wikipedia.org/wiki/Rsync)
  * [sshfs](https://en.wikipedia.org/wiki/SSHFS)
  * [nas](https://en.wikipedia.org/wiki/Network-attached_storage)
  * etc


## Setting up `atrea-collector`

### General (for all options)

#### Build flags

Choosing the backend:
* `irc` (default): uses [twitch's irc-server](https://dev.twitch.tv/docs/chat/irc/)
  * less resource usage
* `eventsub` (WIP): uses [twitch's eventsub-api](https://dev.twitch.tv/docs/eventsub/)
  * will probably have more complete data (proper shoutout dection, outgoing raids, etc)
  * will maybe result in more data-collection-types
  * does not yet work

### Native

Runtime system dependencies:
* openssl

Build-time dependencies:
* cargo (<https://rustup.sh>)
* git

Download:

```sh
cd /where_ever/you/want/to/store/it

git clone --depth 1 https://github.com/jan9103/atrea
mkdir atrea/collected_data  # could be anywhere, but for simplicity ill keep it here during this tutorial
cd atrea/atrea-collector
cargo build --release

# resulting binary-location:
# /where_ever/you/want/to/store/it/atrea/atrea-collector/target/release/atrea-collector
#
# you can symlink it into your PATH if you want or just use the entire path whenever referenced.
# system-wide:
chmod a+rw /where_ever/you/want/to/store/it/atrea/atrea-collector/target/release/atrea-collector
ln -s /where_ever/you/want/to/store/it/atrea/atrea-collector/target/release/atrea-collector /usr/local/bin/
# user only:
ln -s /where_ever/you/want/to/store/it/atrea/atrea-collector/target/release/atrea-collector "$HOME/.local/bin/"
```

Updating:

```sh
cd /where_ever/you/want/to/store/it/atrea/atrea-collector

git pull
cargo build --release
```

### Nixos (untested)

```nix
{ lib, fetchFromGitHub, rustPlatform, openssl, pkg-config, nix-update-script}:
let version = "0.5"; in  # <-- replace
rustPlatform.buildRustPackage {
  pname = "atrea-collector";
  inherit version;
  src = fetchFromGitHub {
    owner = "jan9103";
    repo = "atrea-collector";
    rev = version;
    hash = lib.fakeHash;  # <-- replace
  } + "/atrea-collector";
  cargoHash = lib.fakeHash;  # <-- replace
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
  passthru.updateScript = nix-update-script { };
  meta = with lib; {
    description = "atrea-collector";
    homepage = "https://github.com/jan9103/atrea";
    license = licenses.agpl3Only;
    mainProgram = "atrea-collector";
  };
}
```


## How `atrea-collector` operates (abridged)

It first connects to the twitch server and "joins" (collects data from) the channels listed in the channel-list-file.

When someone raids into a joined channel atrea-collector will join the raider.  
This is configurable, expandable, etc - but that is part of the usage chapter.

Long version: [how it works](./how_it_works.md)


## Running `atrea-collector`

you can run `<binary> --help` to get a quick overview.

The collector is intended to run 24/7 and be restarted from time to time.  
So i would recommend using something like a systemd-service.  
But it is still important to understand the basics first to enable you to configure it.

### Usage

**Environmental variables:** ([what is that?](https://en.wikipedia.org/wiki/Environment_variable))
* `PWD` (current directory): This is where the collected data will be stored

**Flags:** ([what is that?](https://en.wikipedia.org/wiki/Command-line_interface#Command-line_option))
* `--channel-list-file <PATH>` (`-c <PATH>`): see below
* `--log-joins` (`-j`): Enables the logging of join-messages from chatters.
  * This is a lot of data (especially for bigger streamers) with low impact-per-storage-byte.
  * `atrea-collector` is extremely efficient, so you will probably not notice it, but this is extra cpu/.. usage.
  * If you target small streamers i would recommend to turn it on.
* `--log-shoutouts` (`-s`): Enables the logging of shoutouts.
  * Sadly this is based on parsing `!so` and `!shoutout` from the streamer, thus:
    * atrea will have to parse all chat messages (though it has suprising little cpu/.. impact)
    * the collected data is faulty by design (no idea if they shout-out twitch, twitter, etc; false shoutouts; etc)
  * Not many algorithms use this (yet), but it isn't much disk-space per day.
* `--follow-shoutouts`: When enabled it will follow shoutouts the same way as it follows raids.
  * It is sadly impossible to filter big streamers.
  * It is often less related content than with raids (a lot of shoutouts are artists, etc).
* `--min-raidsize <NUMBER>`: Minimum raid-size to follow.
  * Default: 3
  * Why?
    * Trolls sometimes solo-raid.
    * You can find gold, but often small streamers are new.
    * If you target big streamers you probably don't want to waste resources on "tiny" streamer, which raid with 5 people into a 5k viwer channel.
* `--max-raidsize <NUMBER>`: Maximum raid-size to follow.
  * Default: [usize::MAX](https://doc.rust-lang.org/std/primitive.usize.html#associatedconstant.MAX) (essentially infinite, since `atrea-collector` will crash if this is exceeded)
  * This is intended for searching "small" streamers.
    * Sometimes 100k streamers raid 50 streamers and it trickles up/down (10 -> 20 -> 100 -> 500 -> 10k).
    * Big streamers raid big streamers, so once one is joined all will be joined quite quickly.
    * Especially with `-j` big streamers have a big impact on cpu-usage and disk. (10 viewers vs 100k viwers => 10_000x)
      * It is still manageable, but why waste resources on useless data?
* `--stdout-sent`: Log sent commands to stdout (mainly for debugging)
* `--stdout-recieved`: Log recieved commands to stdout (mainly for debugging and extremely spammy)

**channel-list-file:**  
A newline seperated list of channels, which should be starting point for the data-collection.  
This should be channels you like.  
From my experience a size of 20-100 is ideal, but this also depends on stream-frequency, etc.

The channels should be written as their [login name](https://dev.twitch.tv/docs/api/reference).  
You can get the login-name by just looking at the URL:  
if the URL is `https://www.twitch.tv/princessxen` the login is `princessxen`.

Example:
```
princessxen
brodieongames
dove_7
```

### Restarting

Since we experience exponential growth it makes sense to restart the collector semi-frequently.  
The growth speed is not constant since it can take some time to escape a friendgroup, etc, but you
should calculate with it always speeding up its growth.

And outside of the resource-usage point: The data tends to become less accurate over time:
* A darksouls streamer might raid a valheim streamer, which might raid a minecraft streamer, which might raid a pokemon streamer, etc.
* Same for attributes like "funny", "lgbtq-friendly", etc.

But you also need data and level-2, level-3, etc data is still interresting.
* You probably already know your streamers immediate friendgroup.
* Streamers around one or two corners are often still very simiar.

The restart-frequency is up to you:
* Smaller streamers stream (and thus raid/..) less frequently and are less demanding.
  * `--max-raidsize` can help significantly
* A smaller channel-list-file takes longer.

I personally restart it once per week since that net is thrown far enough for me, but even a month
is still at the bottom of htop if you have `--max-raidsize 1000` or similar.

### When is it finished?

Never.  
Streamers always find new friends, raid random people in the same category, etc.  
And even if we have every streamer on the platform: new ones get created.

**When does it start to become useful?**

Technically after the first raid happens.  
And it once again depends on your use-case.  
I would say after 100 raids (`wc -l /where_ever/you/want/to/store/it/atrea/collected_data/raids-*.csv` outputs the current count)
it should have found a few streamers you don't know, but might enjoy.  
From there on it is recessive growth, but 1k raids will obviously have better results, etc.

**When does it stagnate?**

Depends on your use-case, streamers, and taste.  
And there will always be random raids into a new friendgroup, etc.  
But it tends to significantly slow down after a month for me (7days, 200 max-size, etc).

If you add new streamers to the channel-list-file or decrease the restart-frequency it will
obviously expand the potential targets and thus speed up again.

### Compression

The data produced by `atrea-collector` compresses extremely well since 90% of it is constantly repeated channel-logins.

You can theoretically use any compression, but `atrea` "supports" a few and makes using them with `atrea-converter` easier:
* no compression
* [gzip](https://en.wikipedia.org/wiki/Gzip) (fastest; my choice)
* [bzip2](https://en.wikipedia.org/wiki/Bzip2)
* [xz](https://en.wikipedia.org/wiki/XZ_Utils) (best compression)

[benchmarks](https://www.rootusers.com/gzip-vs-bzip2-vs-xz-performance-comparison/)

### Automation

#### Systemd

`/etc/systemd/system/atrea-collector.service`:

```ini
[Unit]
After=network.target
Description=atrea-collector

[Service]
Type=simple
WorkingDirectory=/where_ever/you/want/to/store/it/atrea/colleted_data
ExecStart=/where_ever/you/want/to/store/it/atrea/atrea-collector/target/release/atrea-collector -j -s

# You can run this as any user/group and i would recommend giving it its own for security.
Group=root
User=root
RestrictSUIDGID=true

Restart=always
RestartSec=30
# automatically restart every 7 days:
RuntimeMaxSec=7d

[Install]
WantedBy=multi-user.target
```

#### Nixos

```nix
{pkgs, ...}: let
  base_dir = "/where_ever/you/want/to/store/it/atrea";
in {
  # we create a atrea specific user here.
  # thus you will have to `chown` (or `chmod a+rw`) the `collected_data` directory
  # and `chmod a+rx /where_ever/you/want/to/store/it/atrea/atrea-collector/target/release/atrea-collector`
  #
  # alternatively you can just set the user and group to `root` or your user below
  # and delete this section
  users.users.atreacollector = {
    group = "atreacollector";
    isSystemUser = true;
  };
  users.groups.atreacollector = {};

  environment.systemPackages = with pkgs; [
    openssh
    # not required at runtime, but useful:
    cargo git gzip
  ];

  systemd.services."trt" = {
    description = "trt";
    after = ["network.target"];
    wantedBy = ["multi-user.target"];
    serviceConfig = {
      Type = "simple";
      WorkingDirectory = "${base_dir}/collected_data";
      # if you used nixos to build the package use "${pkg.atrea-collector}/bin/atrea-collector" as binary instead.
      #
      # adjust the arguments to your liking
      ExecStart = "${base_dir}/atrea-collector/target/release/atrea-collector -j -s";

      Restart = "always";
      RestartSec = 30;
      RuntimeMaxSec = "7d";  # restart periodically

      User  = "atreacollector";
      Group = "atreacollector";
      RestrictSUIDGID = "true";
    };
  };
}
```


## Setting up `atrea-converter`

You will be running `atrea-converter` once before running `atrea-webui` and it uses as much cpu as it can.  
So it makes sense to install and run it on your high-powered device.

### Native

Atrea converter is a script and thus you will have to install the language it is written in: [nu](https://www.nushell.sh/book/installation.html)  
If you choose to compile `nu` yourself keep the `sqlite` feature enabled.

If you use the same device as for `atrea-collector` you can use the same atrea download: `/where_ever/you/want/to/store/it/atrea/atrea-converter/convert.nu`.

Otherwise:

```sh
cd /where_ever/you/want/to/store/it
git clone --depth 1 https://github.com/jan9103/atrea
mkdir area/collected_data
```

Update:

```sh
cd /where_ever/you/want/to/store/it/atrea
git pull
```

## Getting twitch tokens

`atrea-converter` requires a twitch token in order to fetch extra information about streamers, such as:
* profile picture
* description
* display name
* type (affiliate, etc)
* creation date

[tutorial](https://dev.twitch.tv/docs/authentication/register-app/).

You might want to write the values down somewhere, so that you don't have to go through that again.  
If you don't mind the security risk you can also add them to your `~/.bashrc` or `$nu.config-path`.


## Running `atrea-converter`

### Transferring data between devices

If you run the suggested environment and have the `atrea-collector` on a seperate device
you will have to somehow get the data over.
If not just skip this.

Here are a few options:

**sshfs:**

This requires the collector-device to be running a [ssh](https://en.wikipedia.org/wiki/Secure_Shell)-server
and your converter-device to install [sshfs](https://en.wikipedia.org/wiki/SSHFS).

sshfs will make all remote files accessible until you restart by live-sending their contents over the net when requested.

```sh
# on converter-device

# where is the server (collector_device)? (ip, domain, or config-name)
server_address="192.168.178.21"
# where is the `collected_data` directory on the collector-device?
remote_path="/where_ever/you/want/to/store/it/atrea/collected_data"
# where should it be located on the converter-device?
local_path="/where_ever/you/want/to/store/it/atrea/collected_data"

sshfs "$server_address:$remote_path" "$local_path"
```

**scp:**

This requires the collector-device to be running a [ssh](https://en.wikipedia.org/wiki/Secure_Shell)-server
and your converter-device to install [scp](https://en.wikipedia.org/wiki/Secure_copy_protocol).

```sh
# on converter-device

# where is the server (collector_device)? (ip, domain, or config-name)
server_address="192.168.178.21"
# where is the `collected_data` directory on the collector-device?
remote_path="/where_ever/you/want/to/store/it/atrea/collected_data"
# where should it be located on the converter-device?
local_path="/where_ever/you/want/to/store/it/atrea/collected_data"

scp "$server_address:$remote_path/*" "$local_path"
```

**ftp:**

This requires the collector-device to be running a [ftp](https://en.wikipedia.org/w/index.php?title=Special%3ASearch&search=File+Transfer+Protocol&wprov=acrw1_0)-server
and your converter-device to install some kind of ftp client.

Example ftp-clients:
* [filezilla](https://filezilla-project.org/) (ol reliable graphical)
* graphical file-managers with any update after 1980 like [dolphin](https://apps.kde.org/dolphin/) or [pcmanfm](https://en.wikipedia.org/wiki/PCMan_File_Manager)
* universal downloaders like [cURL](https://en.wikipedia.org/wiki/CURL) (usually preinstalled on linux), [wget](https://en.wikipedia.org/wiki/Wget) (usually preinstalled on linux), or [aria2](https://github.com/aria2/aria2)
* dedicated ftp-clients like [NcFTP](https://en.wikipedia.org/wiki/NcFTP)
* [etc](https://en.wikipedia.org/wiki/Comparison_of_FTP_client_software)


### Usage

The script is located at `/where_ever/you/want/to/store/it/atrea/atrea-converter/convert.nu`.

`nu convert.nu --help` works.

**Environmental variables:** ([what is that?](https://en.wikipedia.org/wiki/Environment_variable))

* `TWITCH_CLIENT_ID` and `TWITCH_CLIENT_SECRET`

**Flags:** ([what is that?](https://en.wikipedia.org/wiki/Command-line_interface#Command-line_option))

* `--collected-data-dir <PATH>`
  * default: `./collected_data`.
  * the directory where `atrea-collector` spewed its `csv` files, or the directory you copied it to.
  * only `.csv`, `.csv.bz2`, `.csv.gz`, and `.csv.xz` will be used.
* `--output-sqlite <PATH>`: Where should the final `sqlite` file be stored?
  * default: `./atrea_db.sqlite`.
* `--liked-channel-file <PATH>`: see below
  * default: `./liked_channels.txt`.

**liked-channel-file:**

This uses the same format as the `atrea-collector` `channel-list-file`.

You can use the same file, but you can also use diffrent ones.  
Thus you can use the same collector for multiple people by just concatinating their `liked-channel-file`s.  
But it can also be used to improve results if you think a channel is a good starting point, but not a channel you like.

It is recommended to only include channels from the `channel-list-file` in the `liked-channel-file`.

**example:**

```sh
cd /where_ever/you/want/to/store/it/atrea
export TWITCH_CLIENT_ID="123"
export TWITCH_CLIENT_SECRET="123"
nu atrea-converter/convert.nu --collected-data-dir ./collected_data --output-sqlite ./atrea-webui/atrea_db.sqlite --liked-channel-file ./liked_channels.txt
```


## Use of the data with other software

Both `csv` and `sqlite` are universal formats, which can be used by many programs.

* If you know some `sql` or want to look at the raw data [dbeaver](https://dbeaver.io/) is a nice option.
* Nu(-shell) is great for exploring or converting either.
* It should be possible to convert it for software like [gephi](https://github.com/gephi/gephi).


## Setting up `atrea-webui`

### Native

This will recycle the atrea download used by `atrea-converter` since it is on the same device.

Build dependencies:
* `cargo` (<https://rustup.sh>)

```sh
cd /where_ever/you/want/to/store/it/atrea/atrea-web
cargo build --release

# resulting binary location:
# /where_ever/you/want/to/store/it/atrea/atrea-web/target/release/atrea-web
```

Updating:

```sh
cd /where_ever/you/want/to/store/it/atrea/atrea-web
git pull  # if you updated `atrea-converter` first this is a duplicate
cargo build --release
```


## Running `atrea-webui`

`atrea-webui` is buit uppon [rocket-rs](https://rocket.rs/) and thus missing a proper interface, etc.

### Usage

**Environmental variables:** ([what is that?](https://en.wikipedia.org/wiki/Environment_variable))

* `PWD` (current directory).
  * recommendation: `/where_ever/you/want/to/store/it/atrea/atrea-web`
  * the sqlite file generated by `atrea-converter` has to be located at `$PWD/atrea_db.sqlite`.
  * if you want to use plugins they will have to be located at `$PWD/plugins`.
    * official plugins are at `/where_ever/you/want/to/store/it/atrea/atrea-web/plugins`.
  * since settings are stored at `$PWD/atrea_settings_db.sqlite` it is recommended to stick to one location.
* `ROCKET_PORT`: On which port should it serve?
  * default: `8000`
* `ROCKET_ADDRESS`: On which address should it server?
  * default: `0.0.0.0` (everywhere)


## Using `atrea-webui`

You can access it with your browser at the port and address you specified.  
If you use the defaults and are on the same device you can simply use `http://127.0.0.1:8000`.  
If it is on another device run `ip addr` on the webui-device to get its ip and then use `http://<ip address>:<port>`

Just open the navigation and click through things.

### Concepts

* `algorithm`: A algorithm is one way to process the data and generate a list of recommendations.
* `plugin`: A plugin is something outside of the binary, which you can enable to change or add something, such as:
  * design changes.
  * extra algorithms.
  * integrations with external services, such as [ganymede](https://github.com/Zibbp/ganymede).
  * links to external services.
* `box` / `window` / `winbox`: The floating boxes in the webui.
* `channel`: A twitch account in the context of beeing a streamer.
* `viewer`: A twitch account in the context of beeing a viewer.
* `force-graph`: A interactive [Force-directed graph drawing](https://en.wikipedia.org/wiki/Force-directed_graph_drawing).
* `sql` / `sqlite`: The database solution used ([sqlite](https://en.wikipedia.org/wiki/SQLite)) and the language used to interact with it ([sql](https://en.wikipedia.org/wiki/SQL)).

### API (aka use from other software)

This is not yet properly documented, but atrea-webui offers a json-API, which could be used by 3rd party services or UIs.
