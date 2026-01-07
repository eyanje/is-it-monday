# Is it Monday? back end

## Running

To start the back end, install [Rust and Cargo](https://rust-lang.org/tools/install/), then run
```bash
cargo run
```

### Environment variables

The back end expects the following variables to be defined in the environment,
else it will not start.

- `DATABASE_PATH`: a path to the libSQL/SQLite database.
- `HOST`: the host and port to listen on.
- `CLEAN_BEFORE`: clean all database entries older than `CLEAN_BEFORE` seconds.
- `CLEAN_TIMEOUT`: wait `CLEAN_TIMEOUT` seconds before cleaning again.
- `ALLOW_ORIGINS`: CORS-allowed origins, delimited by spaces.

## Docker image

To build the Docker image for the back end, run

```bash
docker build . --tag is-it-monday-backend
```

Be aware that the Docker image must be stopped externally using `docker
container kill`.
```bash
docker container kill <CONTAINER_NAME>
```
