# endian-writer-derive

[![Crates.io](https://img.shields.io/crates/v/endian-writer-derive.svg)](https://crates.io/crates/endian-writer-derive)
[![Docs.rs](https://docs.rs/endian-writer-derive/badge.svg)](https://docs.rs/endian-writer-derive)
[![CI](https://github.com/Sewer56/endian-writer-derive/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/endian-writer-derive/actions)

## About

Procedural Macros for [endian-writer] crate.

This is a procedural macro for automatically deriving [EndianWritableAt], [EndianReadableAt],
and [HasSize] traits for structs whose members already implement the above traits.

### Example

The following piece of Rust code.

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
struct Simple {
    a: u32,
    b: u16,
    c: u8,
}
impl HasSize for Simple {
    const SIZE: usize = <u32 as HasSize>::SIZE + <u16 as HasSize>::SIZE + <u8 as HasSize>::SIZE;
}
impl EndianWritableAt for Simple {
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
impl EndianReadableAt for Simple {
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let a = <u32 as EndianReadableAt>::read_at(reader, offset);
        let b =
            <u16 as EndianReadableAt>::read_at(reader, offset + <u32 as HasSize>::SIZE as isize);
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

Consider the following struct

```rust
#[derive(EndianWritable)]
struct WeirdOrder {
    c: u8,
    b: u16,
    a: u32,
}
```

The struct will be written in the order of `u8`, `u16` and lastly `u32`.
On the other hand, Rust may rearrange the fields as `u32`, `u16` and lastly `u8`.

In other words, the derive macro treats the code as if the struct is `#[repr(C, packed(1))]`.
If padding is desired, the user must manually create it in the struct.

## Development

For information on how to work with this codebase, see [README-DEV.MD](README-DEV.MD).

## License

Licensed under [MIT](./LICENSE).  

[Learn more about Reloaded's general choice of licensing for projects.][reloaded-license].  

[codecov]: https://about.codecov.io/
[crates-io-key]: https://crates.io/settings/tokens
[endian-writer]: https://crates.io/crates/endian-writer
[EndianWritableAt]: https://docs.rs/endian-writer/2.1.0/endian_writer/traits/trait.EndianWritableAt.html
[EndianReadableAt]: https://docs.rs/endian-writer/2.1.0/endian_writer/traits/trait.EndianReadableAt.html
[HasSize]: https://docs.rs/endian-writer/2.1.0/endian_writer/traits/trait.HasSize.html
[nuget-key]: https://www.nuget.org/account/apikeys
[reloaded-license]: https://reloaded-project.github.io/Reloaded.MkDocsMaterial.Themes.R2/Pages/license/