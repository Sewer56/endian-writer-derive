# endian-writer-derive

[![Crates.io](https://img.shields.io/crates/v/endian-writer-derive.svg)](https://crates.io/crates/endian-writer-derive)
[![Docs.rs](https://docs.rs/endian-writer-derive/badge.svg)](https://docs.rs/endian-writer-derive)
[![CI](https://github.com/Sewer56/endian-writer-derive/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/endian-writer-derive/actions)

## About

Procedural Macros for [endian-writer] crate (version 2.X.X).

This is a procedural macro for automatically deriving [EndianWritableAt], [EndianReadableAt],
and [HasSize] traits for structs whose members already implement the above traits.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
endian-writer-derive = "0.1"
endian-writer = "2.1"
```

## Usage

### Basic Example

The following Rust code:

```rust
use endian_writer_derive::EndianWritable;

#[derive(EndianWritable)]
#[repr(C)]
struct MyStruct {
    a: u32,
    b: u16,
    c: u8,
}
```

Expands to:

```rust
use endian_writer::*;

#[repr(C)]
struct MyStruct {
    a: u32,
    b: u16,
    c: u8,
}

impl HasSize for MyStruct {
    const SIZE: usize = <u32 as HasSize>::SIZE + <u16 as HasSize>::SIZE + <u8 as HasSize>::SIZE;
}

impl EndianWritableAt for MyStruct {
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        let a = self.a;
        writer.write_at(&a, offset);
        let b = self.b;
        writer.write_at(&b, offset + <u32 as HasSize>::SIZE as isize);
        let c = self.c;
        writer.write_at(
            &c,
            offset + <u32 as HasSize>::SIZE as isize + <u16 as HasSize>::SIZE as isize,
        );
    }
}

impl EndianReadableAt for MyStruct {
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let a = <u32 as EndianReadableAt>::read_at(reader, offset);
        let b = <u16 as EndianReadableAt>::read_at(
            reader,
            offset + <u32 as HasSize>::SIZE as isize,
        );
        let c = <u8 as EndianReadableAt>::read_at(
            reader,
            offset + <u32 as HasSize>::SIZE as isize + <u16 as HasSize>::SIZE as isize,
        );
        Self { a, b, c }
    }
}
```

This code is equivalent to what a human would write, in terms of functionality, at no overhead.

### Code Generation Behaviour

Fields are written to the output ***in the order they are declared in the source code***.

Consider the following struct:

```rust
use endian_writer_derive::EndianWritable;

#[derive(EndianWritable)]
struct WeirdOrder {
    c: u8,
    b: u16,
    a: u32,
}
```

The struct will be written in the order of `u8`, `u16`, and lastly `u32`.
On the other hand, Rust may rearrange the fields as `u32`, `u16`, and lastly `u8`.

In other words, the derive macro treats the code as if the struct is `#[repr(C, packed(1))]`.
If padding is desired, the user must manually create it in the struct.

## Requirements

- The macro can only be applied to structs with named fields.
- All fields within the struct must implement `HasSize`, `EndianWritableAt`, and `EndianReadableAt`.
- The struct should have a deterministic layout, typically enforced using `#[repr(C)]`.

## Developer Manual

For step-by-step development guidance, see the [Developer Manual](https://reloaded-project.github.io/reloaded-templates-rust/manual/).

## Contributing

We welcome contributions! See the [Contributing Guide](https://reloaded-project.github.io/reloaded-templates-rust/manual/#contributing) for details.

## License

Licensed under [MIT](./LICENSE).

[endian-writer]: https://crates.io/crates/endian-writer
[EndianWritableAt]: https://docs.rs/endian-writer/2.1.0/endian_writer/traits/trait.EndianWritableAt.html
[EndianReadableAt]: https://docs.rs/endian-writer/2.1.0/endian_writer/traits/trait.EndianReadableAt.html
[HasSize]: https://docs.rs/endian-writer/2.1.0/endian_writer/traits/trait.HasSize.html
