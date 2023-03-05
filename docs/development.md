# Development

## Prerequisites

* [Docker](https://www.docker.com/) with Docker Compose plugin
* [Task](https://taskfile.dev/)
* [Rust](https://www.rust-lang.org/) toolchain
  * [Trunk](https://trunkrs.dev/)

## Running backend locally

1. Start required Docker containers with `task dev:compose:up`
2. Build and run backend with `task backend:run`.
   API will be available on http://localhost:8080.

## Running frontend locally

1. Build and run frontend with `task frontend:run`.
   It will be served on http://localhost:8000.
   Page will automatically refresh when there are changes in the code.
