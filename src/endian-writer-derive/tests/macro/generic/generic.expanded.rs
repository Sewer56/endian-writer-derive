use endian_writer_derive::EndianWritable;
#[repr(C)]
struct Generic<T> {
    inner: T,
    tag: u32,
}
use endian_writer::*;
impl<T: HasSize + EndianWritableAt + EndianReadableAt> HasSize for Generic<T> {
    const SIZE: usize = 0 + <T as HasSize>::SIZE + <u32 as HasSize>::SIZE;
}
impl<T: HasSize + EndianWritableAt + EndianReadableAt> EndianWritableAt for Generic<T> {
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        writer.write_at(&self.inner, offset);
        writer.write_at(&self.tag, offset + 0 + <T as HasSize>::SIZE as isize);
    }
}
impl<T: HasSize + EndianWritableAt + EndianReadableAt> EndianReadableAt for Generic<T> {
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let inner = <T as EndianReadableAt>::read_at(reader, offset);
        let tag = <u32 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <T as HasSize>::SIZE as isize,
        );
        Self { inner, tag }
    }
}
