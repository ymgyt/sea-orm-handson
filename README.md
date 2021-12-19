# SeaORM Hands On

## Prepare

```console
# install task runner 
carogo install cargo-make

# load environment variables
# or source .envrc
direnv allow
```

## Usage

```console
# start postgres database
cargo make up

# create tables
cargo make migrate

# generate sea orm entities
cargo make seaorm_generate_entity

# login to postgres
cargo make db_login
```
