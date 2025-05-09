# Quick Start

[back](./index.md)

Currently ATREA consists of 3 seperate applications.  
The quick-start is a example usage without much explanation and expects you to know bash.

## setup

* [cargo download](https://rustup.rs)

```sh
cd ~
git clone --depth 1 https://github.com/jan9103/atrea
cd atrea
mkdir collected_data
vim liked_channels.txt  # one channel login-name per line

cd atrea-collector
cargo build --releaes
cd ../atrea-webui
cargo build --release
```

## atrea collector

Collects the raw data needed.
Run this on your server 24/7 and restart from time to time (maybe every week).

```sh
cd ~/atrea/collected_data
../atrea-collector/target/release/atrea-collector \
  --channel-list-file ../liked_channels.txt \
  --log-joins \
  --log-shoutouts \
  --max-raidsize 300
```

## atrea converter

Convert the data for the webui.  
Stop the webui while running this.  

* [nu download](https://nushell.sh)
* [get twitch credentials](https://dev.twitch.tv/docs/authentication/register-app/)

```sh
cd ~/atrea
rm -f atrea-webui/atrea_db.sqlite
export TWITCH_CLIENT_ID="123example123"
export TWITCH_CLIENT_SECRET="123example123"
nu atrea-converter/convert.nu \
  --collected-data-dir collected_data \
  --output-sqlite atrea-webui/atrea_db.sqlite \
  --liked-channel-file liked_channels.txt
```

## atrea webui

A Webserver for displaying the converted data.

```sh
cd ~/atrea/atrea-webui
export ROCKET_PORT=8000
export ROCKET_ADDRESS=0.0.0.0
./target/release/atrea-webui
```

---

[back](./index.md)
