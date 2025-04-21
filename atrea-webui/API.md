# API documentation

## `/api/raids`

source-code: `src/api_raids.rs`

Get a list of induvidual raids:
* `/api/raids/from/<name>/to/<name>`
* `/api/raids/from/<name>`
* `/api/raids/to/<name>`

Get statistics:
* `/api/raids/from/<name>/to/<name>/stats` (returns a single item, not a list)
* `/api/raids/from/<name>/stats`
* `/api/raids/to/<name>/stats`

## `/api/recs`

source-code: `src/api_recs.rs`

### General purpose algorithms: `/api/recs/general/<algorithm>`

Parameters:
* offset
* limit

Algorithm | Description
--------- | -----------
`brta1` | Basic raid trace algorithm v1 (raided (by) favorites)
`bsv` | Basic shared viewers (count how many viewers someone shares with liked channels)
`rava1` | Raid and viewer analysis v1 (vixen1 + brta1)
`vixen1` | Weighted shared viewer analysis
