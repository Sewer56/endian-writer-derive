use endian_writer_derive::EndianWritable;
#[repr(C)]
struct WithLifetime<'a, T> {
    inner: T,
    tag: u8,
}
use endian_writer::*;
impl<'a, T: HasSize + EndianWritableAt + EndianReadableAt> HasSize
for WithLifetime<'a, T> {
    const SIZE: usize = 0 + <T as HasSize>::SIZE + <u8 as HasSize>::SIZE;
}
impl<'a, T: HasSize + EndianWritableAt + EndianReadableAt> EndianWritableAt
for WithLifetime<'a, T> {
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        writer.write_at(&self.inner, offset);
        writer.write_at(&self.tag, offset + 0 + <T as HasSize>::SIZE as isize);
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
        Self { inner, tag }
    }
}
