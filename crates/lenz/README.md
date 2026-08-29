# Lenz

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/nixonyh/lenz#license)
[![Crates.io](https://img.shields.io/crates/v/lenz.svg)](https://crates.io/crates/lenz)
[![Docs](https://docs.rs/lenz/badge.svg)](https://docs.rs/lenz/latest/lenz/)
[![CI](https://github.com/nixonyh/lenz/workflows/CI/badge.svg)](https://github.com/nixonyh/lenz/actions)
[![Discord](https://img.shields.io/discord/442334985471655946.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/Mhnyp6VYEQ)

**Lenz** builds typed, composable paths to the fields of a struct. A
path is a *type*, not a value: zero-sized, collapsing to a pair of
plain function pointers, with a hop through an `Option` field just
another link rather than a separate kind of path.

## Quick Start

```rust
use lenz::Lenz;

#[derive(Lenz)]
pub struct Card {
    pub header: Header,
}

#[derive(Lenz)]
pub struct Header {
    pub badge: Option<Badge>,
}

#[derive(Lenz)]
pub struct Badge {
    pub icon: Icon,
}

#[derive(Lenz)]
pub struct Icon {
    pub size: u32,
}

let card = Card {
    header: Header {
        badge: Some(Badge { icon: Icon { size: 12 } }),
    },
};

// Walk from the root to a nested field.
let size = Card::cursor().header().badge().icon().size().accessor();

assert_eq!(size.get(&card), Some(&12));
```

## How it works

`#[derive(Lenz)]` generates, for each field: a zero-sized `FieldPath`
marker and a `{Struct}Cursor` method that walks to it.
`Struct::cursor()` starts a walk at the root; each `.field()` call
chains another link, and the `B::Source == A::Target` bound rejects a
mismatched join at compile time.

End a walk one of three ways:

- **`.accessor()`** collapses the path into an `Accessor<S, T>`: two
  `fn` pointers, `Copy`, no allocation. `get` / `get_mut` return an
  `Option`, `None` if any `Option` along the way was absent - so a
  caller never has to know which links were optional.
- **`.key()`** returns one `FieldId` naming the whole walk, distinct
  from any single hop's id, so `top.text` and `bottom.text` key
  separately.
- **`.hops()`** returns a `FieldId` per link, the route a patch
  follows.

A `FieldId` wraps the path type's own `TypeId`, so an id can only come
from a real path.

## Attributes

### `#[lenz(ignore)]`

A field marked `#[lenz(ignore)]` gets no marker and no cursor method,
so nothing can name a path to it.

```rust
use lenz::Lenz;

#[derive(Lenz)]
pub struct Row {
    pub label: String,
    #[lenz(ignore)]
    pub cache: Vec<u8>,
}

// `Row::cursor().label()` exists; there is no `.cache()`.
let _ = Row::cursor().label().accessor();
```

### `#[lenz(crate = <path>)]`

By default the generated code finds the `lenz` crate by name. A struct
built by a macro in another crate that only re-exports `lenz` can
point it elsewhere:

```text
#[derive(Lenz)]
#[lenz(crate = ::my_framework::lenz)]
pub struct Generated { /* ... */ }
```

## `no_std`

`lenz` is `#![no_std]`; it uses `alloc` for the `Vec` that
[`Cursor::hops`] returns.

## License

`lenz` is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.
