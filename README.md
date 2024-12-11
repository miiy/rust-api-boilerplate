# rust-api-boilerplate

Rust RESTFul API boilerplate with Actix Web, SQLx, redis-rs

# Required

- MySQL
- Redis

## Features

- Modular design
- Http server: [Actix Web](https://github.com/actix/actix-web)
- CORS [actix-cors](https://github.com/actix/actix-extras/tree/master/actix-cors)
- Time: [time-rs](https://github.com/time-rs/time)
- Database: MySQL [SQLx](https://github.com/launchbadge/sqlx)
- Cache: Redis [redis-rs](https://github.com/redis-rs/redis-rs)
- Json: [serde](https://github.com/serde-rs/serde)
- Config: [config-rs](https://github.com/rust-cli/config-rs)

## Project structure

The project adopts a modular architectural design.

```text
├── config        // app config files
├── docs          // documents
├── migrations    // database migrations files
├── src
│ ├── middleware  // actix middleware
│ ├── server      // api modules
│ │ ├── auth      // auth module
│ │ │ ├── dto.rs        // request and response struct
│ │ │ ├── error.rs      // error
│ │ │ ├── handler.rs    // handler
│ │ │ ├── mod.rs
│ │ │ ├── model.rs      // model
│ │ │ ├── repository.rs // repository
│ │ │ ├── route.rs      // module routes
│ │ │ └── service.rs    // service logic
│ │ ├── index     // index module
│ │ ├── user      // user module
│ │ ├── post      // post module
│ │ ├── profile   // profile module
│ │ ├── error.rs  // api error
│ │ ├── mod.rs
│ │ └── route.rs  // api routes
│ ├── config.rs   // app config
│ ├── db.rs
│ ├── error.rs
│ ├── jwt.rs
│ ├── lib.rs
│ ├── main.rs
│ └── pagination.rs
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

## Modules

Auth: register, login

User: profile

Post: create, update, delete, detail, list


## Getting Started

create database

```sql
CREATE DATABASE rust_api DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

migrate

```bash
cargo install sqlx-cli
export DATABASE_URL=mysql://root:123456@localhost/rust_api
sqlx migrate run
```

edit config

config/*.yaml

## Running

```bash
cargo run
```
