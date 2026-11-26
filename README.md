# draw-rs

Real-time collaborative drawing app written in Rust.

The server is built on Gotham and upgrades requests to WebSockets. Drawing events are broadcast to every connected client, which renders the shared canvas. It listens on `127.0.0.1:7878`.

## Run

```bash
cargo run --release
```

Open http://127.0.0.1:7878/ in two browsers to see the shared canvas.

## Layout

- `src/main.rs` — HTTP handler and WebSocket connection loop
- `src/ws.rs` — WebSocket handshake and framing
- `static/` — browser client

## License

MIT
