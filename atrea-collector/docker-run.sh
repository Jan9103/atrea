#!/bin/sh
docker run \
  --rm \
  -v ..:/mnt \
  -w /mnt/collected_data \
  atrea-collector:latest \
    --channel-list-file /mnt/liked_channels.txt \
    --log-joins \
    --log-shoutouts
