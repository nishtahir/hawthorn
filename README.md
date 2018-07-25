# Hawthorn

[![Travis Build Status](https://img.shields.io/travis/nishtahir/hawthorn.svg)](https://travis-ci.org/nishtahir/hawthorn)
[![Docker Build Status](https://img.shields.io/docker/build/nishtahir/hawthorn.svg)](https://hub.docker.com/r/nishtahir/hawthorn/)

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

## Building

Local builds can simply use cargo with the optional release flag. The project does require a version of rust nightly
which is specified in the `rust-toolchain` file.

```
cargo build --release
```

However a docker based build is available

```
docker build .
```

## Deployment

The easiest way to deploy is using docker by pulling `nishtahir/hawthorn`. You will have to provide a path to your 
database using the `DATABASE_URL` variable. You can configure the log level, port, etc using rockets' config 
options 

```
docker run -d -e DATABASE_URL=/data/production-db.sqlite \
    -e ROCKET_ENV=production \
    -e ROCKET_LOG=normal \
    -e ROCKET_ADDRESS=0.0.0.0 \
    -e ROCKET_PORT=80  \
    -p 4000:80 \
    -v [volume]:/data/ \
    nishtahir/hawthorn
```

> Note that the image is based on alpine which does not set a `localhost` variable which
> Rocket uses by default in development mode. As a result, you should pass a `ROCKET_ADDRESS` as
> part of your deployment. 

The database can be prepared using diesel's migration tool. 

```
diesel migration run
```