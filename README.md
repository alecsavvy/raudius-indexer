# raudius-indexer

An implementation of audius-protocol's indexing logic rewritten in rust.

## how to run

Configure your .env file like so
```
RPC_GATEWAY=
ENTITY_MANAGER_ADDRESS=
DB_CONNECTION_STRING="mongodb://root:pass@0.0.0.0:27017"
SERVER_ADDR="127.0.0.1:3000"
HASH_SALT=
HASH_MIN_LENGTH=
```

Make sure your db, contracts, and gateway are online and then simply
```
cargo run
```
