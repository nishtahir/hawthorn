# Hawthorn

ELO as a service.

## Development Setup

In order to update the schema and perform database migrations you need the Diesel cli tool installed

```
cargo install diesel_cli --no-default-features --features "sqlite"
```

Make sure you have the `rustfmt-preview` component installed.

```sh
$ rustup component add rustfmt-preview
```

