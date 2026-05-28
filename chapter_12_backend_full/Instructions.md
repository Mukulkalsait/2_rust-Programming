

# to avoide the docker contaienr compiletime database checking error we use this aproch with query_as!() macros.

## Generate query cache

cargo sqlx prepare

## Then build with offline mode
cargo build --release --features "sqlx/offline"
