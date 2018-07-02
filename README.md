# Hawthorn

[![Build Status](https://travis-ci.org/nishtahir/hawthorn.svg?branch=master)](https://travis-ci.org/nishtahir/hawthorn)

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
