use endian_writer_derive::EndianWritable;
#[repr(C)]
struct Simple {
    a: u32,
    b: u16,
    c: u8,
}
use endian_writer::*;
impl HasSize for Simple {
    const SIZE: usize = 0 + <u32 as HasSize>::SIZE + <u16 as HasSize>::SIZE
        + <u8 as HasSize>::SIZE;
}
impl EndianWritableAt for Simple {
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        let a = self.a;
        writer.write_at(&a, offset);
        let b = self.b;
        writer.write_at(&b, offset + 0 + <u32 as HasSize>::SIZE as isize);
        let c = self.c;
        writer
            .write_at(
                &c,
                offset + 0 + <u32 as HasSize>::SIZE as isize
                    + <u16 as HasSize>::SIZE as isize,
            );
    }
}
impl EndianReadableAt for Simple {
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let a = <u32 as EndianReadableAt>::read_at(reader, offset);
        let b = <u16 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <u32 as HasSize>::SIZE as isize,
        );
        let c = <u8 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <u32 as HasSize>::SIZE as isize
                + <u16 as HasSize>::SIZE as isize,
        );
        Self { a, b, c }
    }
}
