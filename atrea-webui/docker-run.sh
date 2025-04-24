#!/bin/sh
docker run \
  --rm \
  -v atrea_db.sqlite:/mnt/atrea_db.sqlite \
  -w /mnt/collected_data \
  -e ROCKET_PORT=8000 \
  atrea-webui:latest
