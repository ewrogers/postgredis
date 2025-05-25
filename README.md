# postgedis

Redis server using Postgres as a backend, written in Rust.

Inspired by [Fireship's Video](https://www.youtube.com/watch?v=3JW732GrMdg).

## Architecture

This project implements a single-threaded event loop system using Tokio's
`current_thread` runtime. This approach provides efficient handling of
concurrent connections without the complexity of multi-threading.

### Event Loop System

- Uses Tokio's async runtime for non-blocking I/O operations
- Single-threaded execution model for predictable behavior
- Handles multiple client connections concurrently through `async`/`await`

### Client Commands

- Implements RESP (Redis Serialization Protocol) for command parsing
- Supports basic Redis-compatible commands
- Asynchronous command processing within the event loop

### Server Commands

- Internal command representation for request handling
- Supports responses like PONG, Error, and generic RESP responses
- Converts server commands to RESP format for client communication
