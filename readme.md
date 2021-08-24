# First web developed by Rust & Rocket

### Install Diesel CLI
```
cargo install diesel_cli --no-default-features --features postgres
```

### Setup Diesel
```
#[for dev]
export DATABASE_URL=postgres://rocket:rocket@localhost/rocket
diesel setup

diesel migration generate create_post_table
diesel migration run
```