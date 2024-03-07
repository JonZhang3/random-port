# random-port

> Get an available port.

## Install

```sh
cargo add random-port
```

## Usage

```rust
use random_port::PortPicker;

let port: u16 = PortPicker::new().pick().unwrap();
```

## API

### `PortPicker`

#### `pick()`

Returns a `Result<u16, Error>` for a available port.

### `port_range(RangeInclusive)`

Specifies the range of ports to check. Must be in the range `1024..=65535`. E.g. `port_range(1024..=65535)`.

### `execlude(HashSet<u16>)`/`execlude_add(u16)`

Specifies the ports to exclude.

### `protocol(Protocol)`

Specifies the protocol to check, Default is `Protocol::All`. Can be either `Protocol::Tcp`, `Protocol::Udp` or `Protocol::All`.

### `host(String)`

Specifies the host to check. Can be either an Ipv4 or Ipv6 address.

If not specified, will checks availability on all local addresses defined in the system.

### `random(bool)`

Specifies whether to pick a random port from the range.

If not specified, will pick the first available port from the range.
