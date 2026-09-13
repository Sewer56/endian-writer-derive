use endian_writer_derive::EndianWritable;
#[repr(C)]
struct Multi<T, U>
where
    U: Copy,
{
    first: T,
    second: U,
    count: u16,
}
use endian_writer::*;
impl<
    T: HasSize + EndianWritableAt + EndianReadableAt,
    U: HasSize + EndianWritableAt + EndianReadableAt,
> HasSize for Multi<T, U>
where
    U: Copy,
{
    const SIZE: usize = 0 + <T as HasSize>::SIZE + <U as HasSize>::SIZE
        + <u16 as HasSize>::SIZE;
}
impl<
    T: HasSize + EndianWritableAt + EndianReadableAt,
    U: HasSize + EndianWritableAt + EndianReadableAt,
> EndianWritableAt for Multi<T, U>
where
    U: Copy,
{
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        writer.write_at(&self.first, offset);
        writer.write_at(&self.second, offset + 0 + <T as HasSize>::SIZE as isize);
        writer
            .write_at(
                &self.count,
                offset + 0 + <T as HasSize>::SIZE as isize
                    + <U as HasSize>::SIZE as isize,
            );
    }
}
impl<
    T: HasSize + EndianWritableAt + EndianReadableAt,
    U: HasSize + EndianWritableAt + EndianReadableAt,
> EndianReadableAt for Multi<T, U>
where
    U: Copy,
{
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let first = <T as EndianReadableAt>::read_at(reader, offset);
        let second = <U as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <T as HasSize>::SIZE as isize,
        );
        let count = <u16 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <T as HasSize>::SIZE as isize + <U as HasSize>::SIZE as isize,
        );
        Self { first, second, count }
    }
}
