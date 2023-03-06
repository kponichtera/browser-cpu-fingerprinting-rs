# Development

## Prerequisites

* [Docker](https://www.docker.com/) with Docker Compose plugin (checked with version 23.0.1)
* [Task](https://taskfile.dev/) (checked with version 3.20.0)
* [Rust](https://www.rust-lang.org/) toolchain (checked with version 1.67.1)
  * WASM toolchain target (run `rustup target add wasm32-unknown-unknown`)
  * [Trunk](https://trunkrs.dev/) (checked with version 0.16.0)
  * [SQLx CLI](https://crates.io/crates/sqlx-cli) (checked with version 0.6.2)

## Running backend locally

1. Start required Docker containers with `task dev:compose:up`
2. Setup local development database with `task backend:migrations:reset`.
   You can use the same command later to remove
   all the data from the database and start fresh.
3. Build and run backend with `task backend:run`.
   API will be available on http://localhost:8080.

## Running frontend locally

1. Build and run frontend with `task frontend:run`.
   It will be served on http://localhost:8000.
   Page will automatically refresh when there are changes in the code.

## Working with migrations

### Adding new database migrations

1. Run `task backend:migrations:add -- abcdefg`
   to create new migration called *abcdefg* in the `backend/migrations` catalogue.
2. Add desired queries and DDLs to generated SQL file.
3. Apply the migrations with `task backend:migrations:run`.
   Migrations will be also automatically applied when the backend application is rebuilt and restarted.

To check the state of migrations, execute `task backend:migrations:info`.

### Regenerating query metadata

In order to allow SQLx to validate queries without having database connection during build time, 
use `task backend:migrations:generate_metadata` task while all migrations are applied and the local database is running.