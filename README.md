# Rust API


## Install

```shell
make setup
```

## Database

Project uses Diesel as an ORM 

```shell
# up migration run
diesel migration run
# down migration run
diesel migration down
# re-test up migration 
diesel migration redo
# generate new migration files
diesel migration generate description_of_migration
```