# flash-cards

`flash-cards` is written in Rust. It uses `seed` for frontend and `actix` for
backend.

# Getting started

```bash
git clone https://github.com/knarkzel/flash-cards
```

# Running backend

```bash
cd flash-cards/backend
cargo run
```

For automatic rebuilding, do `cargo install cargo-watch`, then do `cargo watch
run`.

## Resources

- [actix](https://actix.rs/) for documentation.
	- [receiving data via forms](https://github.com/actix/examples/blob/master/forms/form/src/main.rs)
- [rustqlite](https://github.com/rusqlite/rusqlite) for database in Sqlite.
	- [example 1](https://github.com/knarkzel/elixir/blob/master/src/migrations.rs)
	- [example 2](https://github.com/knarkzel/elixir/blob/master/src/routes/index.rs)

# Running frontend

Make sure `cargo-make` is installed (`cargo install cargo-make`).

```bash
cd flash-cards/frontend
cargo make build
cargo make serve
```

You can also run `cargo make watch` to automatically rebuild the project on
file changes. 

## Resources

- [seed](https://seed-rs.org/) for documentation. 
- [chess-randomizer](https://github.com/knarkzel/chess-randomizer/blob/main/src/lib.rs), demonstrates fetching and state.
