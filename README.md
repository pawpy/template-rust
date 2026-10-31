# Rust starter

A minimal web server in Rust with [Axum](https://github.com/tokio-rs/axum) and Tokio.

## Run it locally

```sh
cargo run
```

Then open http://localhost:8080. `GET /health` answers `ok` for health checks.

## Deploying

| Step | Command |
| --- | --- |
| Build | cargo build --release |
| Start | ./target/release/app |

## Environment variables

| Variable | Meaning |
| --- | --- |
| `PORT` | Port to listen on. Default: 8080. |

## License

MIT
