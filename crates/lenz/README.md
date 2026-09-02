# Lenz

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/nixonyh/lenz#license)
[![Crates.io](https://img.shields.io/crates/v/lenz.svg)](https://crates.io/crates/lenz)
[![Docs](https://docs.rs/lenz/badge.svg)](https://docs.rs/lenz/latest/lenz/)
[![CI](https://github.com/nixonyh/lenz/workflows/CI/badge.svg)](https://github.com/nixonyh/lenz/actions)
[![Discord](https://img.shields.io/discord/442334985471655946.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/Mhnyp6VYEQ)

**Lenz** turns a path to a deeply nested field into a reusable handle
you pass around and use later to read or write it. Paths live in the
type system, checked at compile time, zero-cost at runtime.

## Example

```rust
use lenz::{Cursor, Lenz, Tagged};

// A tag type. `lenz` only carries it; a downstream crate attaches the
// meaning (say, "how to write this field to a backend").
pub struct SetText;

#[derive(Lenz)]
pub struct Label {
    #[lenz(tag = SetText)]
    pub text: String,
    pub size: u32,
}

#[derive(Lenz)]
pub struct Button {
    pub label: Label,
    pub icon: Option<Label>,
    #[lenz(ignore)]
    pub pressed: bool,
}

let mut button = Button {
    label: Label { text: "Save".into(), size: 14 },
    icon: None,
    pressed: false,
};

// Walk from the root to a nested field. Each `.field()` adds a hop;
// the `Option` on `icon` is just another link.
let text = Button::cursor().label().text();

// `.accessor()` ends the walk: two `fn` pointers, `Copy`, no alloc.
let acc = text.accessor();
assert_eq!(acc.get(&button).map(String::as_str), Some("Save"));
*acc.get_mut(&mut button).unwrap() = "Saved".into();

// A walk through an absent `Option` returns `None`, never panics.
let icon_text = Button::cursor().icon().text().accessor();
assert_eq!(icon_text.get(&button), None);

// `.hops()` lists the links; `.key()` names the whole walk.
assert_eq!(Button::cursor().label().text().hops().len(), 2);

// `text` is tagged, so its path carries `SetText` as its `Tag`.
fn writes_text<P: Tagged<Tag = SetText>>(_: Cursor<P>) {}
writes_text(Button::cursor().label().text());
```

## How it works

`#[derive(Lenz)]` generates, for each field: a zero-sized `FieldPath`
marker and a `{Struct}Cursor` method that walks to it.
`Struct::cursor()` starts a walk at the root; each `.field()` call
chains another link, and the `B::Source == A::Target` bound rejects a
mismatched join at compile time.

End a walk one of these ways:

- **`.accessor()`**: collapses the path into an `Accessor<S, T>` - two
  `fn` pointers, `Copy`, no allocation. `get` / `get_mut` return an
  `Option`, `None` if any `Option` along the way was absent, so a
  caller never has to know which links were optional.
- **`.key()`**: one `FieldId` naming the whole walk, distinct from any
  single hop's id, so `top.text` and `bottom.text` key separately.
- **`.hops()`**: a `FieldId` per link, the route a patch follows.

A `FieldId` wraps the path type's own `TypeId`, so an id can only come
from a real path.

## Attributes

- **`#[lenz(ignore)]`**: the field gets no marker and no cursor
  method, so nothing can name a path to it.
- **`#[lenz(tag = <path>)]`**: the field's marker also implements
  `Tagged`, with `<path>` as the associated `Tag` type. `lenz` never
  inspects it, and a `Chain` carries the tag of its last hop, so a
  composed walk exposes the tag of the field it ends on.
- **`#[lenz(crate = <path>)]`**: on the struct, points the generated
  code at a re-exported `lenz`, for a struct emitted by another
  crate's macro:

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
