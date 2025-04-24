#!/bin/sh
docker run \
  --rm \
  -v ..:/mnt \
  -w /mnt/atrea-converter \
  ghcr.io/nushell/nushell:0.103.0-alpine \
  nu convert.nu \
    --collected-data-dir ../collected_data \
    --output-sqlite ../atrea-webui/atrea_db.sqlite \
    --liked-channel-file ../liked_channels.txt
