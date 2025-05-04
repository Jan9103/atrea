use std/log

# https://twitchinsights.net/bots
# https://streamscharts.com/tools/bots
const KNOWN_GLOBAL_BOTS: list<string> = [
  "anthropologydept"
  "blerp"
  "botbandera"
  "botrixofficial"
  "botrixoficial"
  "commanderroot"
  "creatisbot"
  "dizel_478"
  "fossabot"
  "frostytoolsdotcom"
  "fyow"
  "jeetbot"
  "kofistreambot"
  "kumabotdefender"
  "lurxx"
  "moobot"
  "mr_gionny99"
  "nightbot"
  "overlayexpert"
  "own3d"
  "playwithviewersbot"
  "pokemoncommunitygame"
  "sery_bot"
  "soundalerts"
  "streamelements"
  "streamlabs"
  "streamstickers"
  "streamstickers"
  "tangiabot"
  "tangiabot"
  "trackerggbot"
  "wizebot"
  "wzbot"

  "justinfan007"  # this bot itself
]

# Required environmental variables:
# * TWITCH_CLIENT_ID and TWITCH_CLIENT_SECRET ( https://dev.twitch.tv/docs/authentication/register-app/ )
def main [
  --collected-data-dir: path = "./collected_data"
  --output-sqlite: path = "./atrea_db.sqlite"
  --liked-channel-file: path = "./liked_channels.txt"
]: nothing -> nothing {
  $env.NU_LOG_FORMAT = ($env.NU_LOG_FORMAT? | default "%ANSI_START%%DATE%|%LEVEL%|%MSG%%ANSI_STOP%")
  $env.NU_LOG_LEVEL = ($env.NU_LOG_LEVEL? | default "info")
  $env.NU_LOG_DATE_FORMAT = ($env.NU_LOG_DATE_FORMAT | default "%FT%H:%M:%S")
  let output_sqlite: path = ($output_sqlite | path expand)
  let collected_data_dir: path = ($collected_data_dir | path expand)
  let liked_channels: list<string> = (open --raw $liked_channel_file | lines)

  log debug 'Validating inputs..'
  #if ($liked_channels | describe) != 'list<string>' {
  #  log error $'invalid liked-channel-list in ($liked_channel_file) \(expected a list of strings, got ($liked_channels | describe))'
  #  exit 1
  #}
  if ("TWITCH_CLIENT_ID" not-in $env) or ("TWITCH_CLIENT_SECRET" not-in $env) {
    log error "Missing a environmental variable (rerun with '--help' for more info)"
    exit 1
  }

  log info 'Savig liked channels to sqlite..'
  stor create -t liked_channels --columns {"name": str}
  $liked_channels
  | each {|i| {"name": $i} }
  | stor insert -t liked_channels | null

  csv_to_sqlite $collected_data_dir
  calculate_raid_connections
  calculate_shoutout_connections
  preprocess_joins
  get_channel_data

  stor export -f $output_sqlite | null
}

def csv_to_sqlite [
  collected_data_dir: path
]: nothing -> nothing {
  cd $collected_data_dir

  log info "Converting joins to sqlite.."
  stor create -t joins --columns {"timestamp": datetime, "viewer": str, "target": str}
  glob 'joins-*.csv*'
  | par-each {|file|
    log info $"  Converting ($file | path basename) to sqlite.."
    smart_csv_cat $file
    | rename timestamp viewer target
    | where viewer not-in $KNOWN_GLOBAL_BOTS
    | stor insert -t joins | null
  } | null

  log info "Converting raids to sqlite.."
  stor create -t raids --columns {"timestamp": datetime, "raider": str, "target": str, "size": int}
  glob 'raids-*.csv*'
  | par-each {|file|
    log info $"  Converting ($file | path basename) to sqlite.."
    smart_csv_cat $file
    | stor insert -t raids | null
  } | null

  log info "Converting shoutouts to sqlite.."
  stor create -t shoutouts --columns {"timestamp": datetime, "author": str, "target": str}
  glob 'shoutouts-*.csv*'
  | par-each {|file|
    log info $"  Converting ($file | path basename) to sqlite.."
    smart_csv_cat $file
    | stor insert -t shoutouts | null
  } | null
}

def smart_csv_cat [
  file: path
]: nothing -> table {
  match ($file | path parse | get extension) {
    "csv" => { open --raw $file }
    "gz" => { ^gzip -kdc $file }
    "xz" => { ^xz -kdc $file }
    "bz2" => { ^bz2 -kdc $file }
    _ => { error make {msg: $"Unknown file extension in collected-data directory: ($file)"} }
  }
  | from csv
  | update timestamp {|i| $i.timestamp | into datetime -f %s }
}

def calculate_raid_connections []: nothing -> nothing {
  log info "Generating raid-connection cache.."
  stor create -t raid_connections --columns {"raider": str, "target": str, "total_viewers": int, "raid_count": int, "average_raid_size": int} | null
  (stor open).raids.raider
  | uniq
  | par-each {|raider|
    for target in ((stor open).raids | where raider == $raider | get target | uniq) {
      let raids = ((stor open).raids | where raider == $raider and target == $target)
      {
        "raider": $raider, "target": $target,
        "total_viewers": ($raids.size | math sum)
        "raid_count": ($raids | length)
        "average_raid_size": ($raids.size | math avg | math round)
      } | stor insert -t raid_connections | null
    }
  } | null
}

def calculate_shoutout_connections []: nothing -> nothing {
  log info "Generating shoutout-connection cache.."
  stor create -t shoutout_connections --columns {"author": str, "target": str, "shoutout_count": int} | null
  (stor open).shoutouts.author
  | uniq
  | par-each {|author|
    for target in ((stor open).shoutouts | where author == $author | get target | uniq) {
      let raids = ((stor open).shoutouts | where author == $author and target == $target)
      {
        "author": $author, "target": $target,
        "shoutout_count": ($raids | length)
      } | stor insert -t shoutout_connections | null
    }
  } | null
}

def get_channel_data []: nothing -> nothing {
  log info "Fetching channel details via twitch api.."
  stor create -t channel_info --columns {
    "id": int
    "login": str
    "display_name": str
    "broadcaster_type": str
    "description": str
    "profile_image_url": str
    "created_at": datetime
  }

  log info "  Logging into twitch.."
  let access_token: string = (
    ""
    | http post $'https://id.twitch.tv/oauth2/token?grant_type=client_credentials&client_id=($env.TWITCH_CLIENT_ID)&client_secret=($env.TWITCH_CLIENT_SECRET)'
    | get access_token
  )
  let headers: list<string> = [
    "Authorization" $"Bearer ($access_token)"
    "Client-ID" $env.TWITCH_CLIENT_ID
  ]

  log info '  Getting channel info..'
  for chunk in (
    # no idea why, but using `queryr db` is broken (the data still shows up in the for-loop, but dosn't get saved)
    #stor open
    #| query db 'SELECT DISTINCT name FROM (SELECT DISTINCT (r.raider || r.target) AS name FROM raids r UNION SELECT DISTINCT lc.name FROM liked_channels lc)'
    #| get name
    (stor open).raid_connections.raider
    | append ((stor open).raid_connections.target)
    | append ((stor open).liked_channels.name)
    | append ((stor open).shoutout_connections.target)
    | uniq
    | url encode -a
    | each {|i| $'login=($i)'}
    | chunks 100  # limit of channels you can check per request
  ) {
    if ($chunk | is-empty) { continue }
    let res = (
      http get -H $headers $'https://api.twitch.tv/helix/users?($chunk | str join "&")'
      | get data
      | update created_at {|i| $i.created_at | into datetime -f %+ }
      | update id {|i| $i.id | into int}
      | select id login display_name broadcaster_type description profile_image_url created_at
    )
    $res | stor insert -t channel_info
  }

  log info '  Cleaning up twitch access token..'
  "" | http post -H $headers $'https://id.twitch.tv/oauth2/revoke?client_id=($env.TWITCH_CLIENT_ID)&token=($access_token)'

  null
}

def preprocess_joins [] {
  log info 'Preprocessing join messages..'
  stor create -t join_counts --columns {
    "viewer": str
    "target": str
    "count": int
  }

  for channel in (stor open | query db 'SELECT DISTINCT target FROM joins').target {
    stor open
    | query db 'SELECT viewer, target, COUNT(*) AS count FROM joins WHERE target = ? GROUP BY viewer' -p [$channel]
    | stor insert -t join_counts
  }
}
