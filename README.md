# pfrs
[![Crate](https://img.shields.io/crates/v/pfrs.svg)](https://crates.io/crates/pfrs)

A cross-platform solution dedicated to finding an available dynamic port.

## Usage
```rust
use pfrs::*;

fn main() {
    let port = find_open_port().unwrap();
}
```