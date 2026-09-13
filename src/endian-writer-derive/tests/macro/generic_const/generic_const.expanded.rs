use endian_writer_derive::EndianWritable;
#[repr(C)]
struct WithConst<T, const N: usize> {
    inner: T,
    bytes: [u8; N],
    tag: u32,
}
use endian_writer::*;
impl<T: HasSize + EndianWritableAt + EndianReadableAt, const N: usize> HasSize
for WithConst<T, N> {
    const SIZE: usize = 0 + <T as HasSize>::SIZE + <[u8; N] as HasSize>::SIZE
        + <u32 as HasSize>::SIZE;
}
impl<T: HasSize + EndianWritableAt + EndianReadableAt, const N: usize> EndianWritableAt
for WithConst<T, N> {
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        writer.write_at(&self.inner, offset);
        writer.write_at(&self.bytes, offset + 0 + <T as HasSize>::SIZE as isize);
        writer
            .write_at(
                &self.tag,
                offset + 0 + <T as HasSize>::SIZE as isize
                    + <[u8; N] as HasSize>::SIZE as isize,
            );
    }
}
impl<T: HasSize + EndianWritableAt + EndianReadableAt, const N: usize> EndianReadableAt
for WithConst<T, N> {
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let inner = <T as EndianReadableAt>::read_at(reader, offset);
        let bytes = <[u8; N] as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <T as HasSize>::SIZE as isize,
        );
        let tag = <u32 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <T as HasSize>::SIZE as isize
                + <[u8; N] as HasSize>::SIZE as isize,
        );
        Self { inner, bytes, tag }
    }
}
