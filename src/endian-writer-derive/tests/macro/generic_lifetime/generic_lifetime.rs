use endian_writer_derive::EndianWritable;

use std::marker::PhantomData;

use endian_writer::{
    EndianReadableAt, EndianReader, EndianWritableAt, EndianWriter, HasSize,
};

// A struct cannot declare a lifetime no field uses (E0392), so the lifetime is
// anchored by this zero-sized stand-in, which stays serializable for the derive.
struct Borrowed<'a> {
    marker: PhantomData<&'a u32>,
}

impl HasSize for Borrowed<'_> {
    const SIZE: usize = 0;
}

impl EndianWritableAt for Borrowed<'_> {
    unsafe fn write_at<W: EndianWriter>(&self, _writer: &mut W, _offset: isize) {}
}

impl EndianReadableAt for Borrowed<'_> {
    unsafe fn read_at<R: EndianReader>(_reader: &mut R, _offset: isize) -> Self {
        Borrowed { marker: PhantomData }
    }
}

// The derive must still carry `'a` through the impl generics.
#[derive(EndianWritable)]
#[repr(C)]
struct WithLifetime<'a, T> {
    inner: T,
    tag: u8,
    borrow: Borrowed<'a>,
}
