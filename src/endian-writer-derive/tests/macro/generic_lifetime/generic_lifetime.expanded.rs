use endian_writer_derive::EndianWritable;
use std::marker::PhantomData;
use endian_writer::{
    EndianReadableAt, EndianReader, EndianWritableAt, EndianWriter, HasSize,
};
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
#[repr(C)]
struct WithLifetime<'a, T> {
    inner: T,
    tag: u8,
    borrow: Borrowed<'a>,
}
use endian_writer::*;
impl<'a, T: HasSize + EndianWritableAt + EndianReadableAt> HasSize
for WithLifetime<'a, T> {
    const SIZE: usize = 0 + <T as HasSize>::SIZE + <u8 as HasSize>::SIZE
        + <Borrowed<'a> as HasSize>::SIZE;
}
impl<'a, T: HasSize + EndianWritableAt + EndianReadableAt> EndianWritableAt
for WithLifetime<'a, T> {
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        writer.write_at(&self.inner, offset);
        writer.write_at(&self.tag, offset + 0 + <T as HasSize>::SIZE as isize);
        writer
            .write_at(
                &self.borrow,
                offset + 0 + <T as HasSize>::SIZE as isize
                    + <u8 as HasSize>::SIZE as isize,
            );
    }
}
impl<'a, T: HasSize + EndianWritableAt + EndianReadableAt> EndianReadableAt
for WithLifetime<'a, T> {
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let inner = <T as EndianReadableAt>::read_at(reader, offset);
        let tag = <u8 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <T as HasSize>::SIZE as isize,
        );
        let borrow = <Borrowed<
            'a,
        > as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <T as HasSize>::SIZE as isize + <u8 as HasSize>::SIZE as isize,
        );
        Self { inner, tag, borrow }
    }
}
