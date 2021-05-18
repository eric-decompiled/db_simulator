## Dependencies
- PostgreSQL (or docker)
- rust

To run an emphemeral postgreSQL instance with docker you can use:
`docker pull postgres`
`docker run --rm --name dev-postgres -e POSTGRES_PASSWORD=topsecret -p 5432:5432 postgres`

If you choose to use different settings either set `$DATABASE_URL`, or update the `.env` file with the connection string.

When testing on a mac the app would erorr with `FATAL role postgres does not exist`. This was from trying to connect to a postgres installed by brew and not the docker container. `brew services stop postgresql` was able to resolve this issue.

If you don't have rust installed, you can use [rustup](https://rustup.rs/) to get it.

## Running
By default a type file is expected at `./type_mapping.json`. This can be configured with `$TYPE_FILE`. This file is only parsed on start up. For changes to be reflected in `writer` it will need to be restarted. `reader` will need to be restarted to detect new event types.

## Programs
  - Writer: `cargo run --bin writer`
  - Reader `cargo run --bin reader`

## Type Mapping
The following values are supported for "type_mapping" fields:
- `int`
- `bigint`
- `text`
- `boolean`
- `timestamp`

## Notes
All events are stored in a table `events`, which is indexed on (time, type), all other data is stored in `body` as JSONB. There is also a unique `fingerprint` column whose contents are derived from a  `sha256::digest` of the event before inserting it. 

The few tests are integration style tests, they depend on having access to a DB and will insert events into it.

### `time`:
`reader` queries for events within an interval, since `time` is powering our index it is stored in a dedicated SQL column and not the body. `timestamp` values are supported in the payload with different names, albeit they will be stored in rust secs + nanos since epoch format. All events will be assigned a `time` value even if its absent from the mapping. `time` will be sometime within the last week.