FROM rust:1.67.1 AS builder
RUN apt-get update \
    && apt-get install -y musl-tools \
    && rm -rf /var/lib/apt/lists

ARG TASK_URL="https://github.com/go-task/task/releases/download/v3.20.0/task_linux_amd64.tar.gz"
RUN curl -Lo /tmp/task.tgz "${TASK_URL}" \
    && tar xvzOf /tmp/task.tgz task > /bin/task \
    && rm -f /tmp/task.tgz \
    && chmod +x /bin/task

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo install trunk --version 0.16.0
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo install sqlx-cli --version 0.6.2

FROM builder AS builder-base

WORKDIR /project
COPY Cargo.toml Cargo.lock Taskfile.yml Trunk.toml ./
COPY common/ ./common/
COPY backend/ ./backend/
COPY frontend/ ./frontend/

FROM builder-base AS backend-builder

RUN --mount=type=cache,target=/usr/local/cargo/registry task backend:build -- --target x86_64-unknown-linux-musl

FROM builder-base AS frontend-builder

RUN --mount=type=cache,target=/usr/local/cargo/registry task frontend:build

FROM scratch AS backend

COPY --from=backend-builder /project/target/x86_64-unknown-linux-musl/release/backend /backend
CMD ["/backend"]

FROM nginx:1.22-alpine AS frontend

COPY docker/nginx/server.conf /etc/nginx/conf.d/default.conf
COPY --from=frontend-builder /project/dist/ /srv/frontend
