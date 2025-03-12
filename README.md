# Building WEB API with RUST

# Initialize Rust Project

```

cargo init server
```

## How to Run

```
cargo build
```
cargo run
```

## Test API Requests

Using Curl
```
curl http://127.0.0.1:8080/       # Hello World
curl -X POST http://127.0.0.1:8080/create  # created
curl -X PUT http://127.0.0.1:8080/update   # updated
curl -X DELETE http://127.0.0.1:8080/delete  # deleted
```