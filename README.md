# postgedis

Redis server using Postgres as a backend, written in Rust.

Inspired by [Fireship's Video](https://www.youtube.com/watch?v=3JW732GrMdg).

## Why does this exist?

Most modern applications already rely on PostgreSQL as their primary database.
Instead of running a separate Redis instance that requires additional
infrastructure, monitoring, and maintenance, Postgedis allows you to leverage
your existing Postgres deployment as a Redis-compatible server.

Key benefits include:

- **Zero Additional Infrastructure**: Uses your existing Postgres database
  as a key-value backend
- **Advanced Features**: Inherits Postgres' robust functionality and extensions
- **Pub/Sub Layer**: Provides Redis-compatible publish/subscribe functionality
- **High Performance**: Written in Rust for optimal speed and memory efficiency
- **Small Footprint**: Minimal resource overhead as it's just a thin
  compatibility layer
- **Familiar Interface**: Uses the standard Redis protocol, working with
  existing Redis clients
- **Data Consistency**: Single source of truth for both your application data
  and caching layer

This approach simplifies your stack while providing the benefits of both Redis
and Postgres in a single, efficient package.

Given it uses Postgres, even more extended functionality and commands can be
implemented that go beyond the normal Redis capabilities.

## Implemented Commands

### Key-Value Commands

| Command | Status |
|---------|--------|
| GET     | ❌      |
| SET     | ❌      |
| DEL     | ❌      |
| EXISTS  | ❌      |
| EXPIRE  | ❌      |
| TTL     | ❌      |

### List Commands

| Command | Status |
|---------|--------|
| LPUSH   | ❌      |
| RPUSH   | ❌      |
| LPOP    | ❌      |
| RPOP    | ❌      |
| LLEN    | ❌      |
| LRANGE  | ❌      |

### Set Commands

| Command   | Status |
|-----------|--------|
| SADD      | ❌      |
| SREM      | ❌      |
| SMEMBERS  | ❌      |
| SISMEMBER | ❌      |
| SCARD     | ❌      |

### Hash Commands

| Command | Status |
|---------|--------|
| HSET    | ❌      |
| HGET    | ❌      |
| HDEL    | ❌      |
| HGETALL | ❌      |
| HEXISTS | ❌      |

### Sorted Set Commands

| Command | Status |
|---------|--------|
| ZADD    | ❌      |
| ZREM    | ❌      |
| ZRANGE  | ❌      |
| ZRANK   | ❌      |
| ZCARD   | ❌      |

### Other Commands

| Command | Status |
|---------|--------|
| PING    | ❌      |
