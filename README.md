# Crawly

A Simple feed reader implementation using rust and Vue.js.

## Building

Rust components produce two binaries, one webserver to be used by web client, and a crawler that will periodically update stale feeds.

To build both the binaries run

```sh
$ cargo build
```

To build specific binaries run either

```sh
$ cargo build --bin crawly # builds web server

$ cargo build --bin crawler # builds the crawler
```

## Running

To run either the webserver or the crawler create a `.env` file with following contents

```
APP_DATABASE_URL=postgres://<user>:<password>@<host>/<db name>
APP_PORT=9000
```

## Postgres

Prior to running the webserver or the crawler create the required Postgres Schema using the SQL script in `server/init.sql`
