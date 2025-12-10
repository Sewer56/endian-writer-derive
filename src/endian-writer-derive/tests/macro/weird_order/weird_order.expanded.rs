use endian_writer_derive::EndianWritable;
struct WeirdOrder {
    c: u8,
    b: u16,
    a: u32,
}
use endian_writer::*;
impl HasSize for WeirdOrder {
    const SIZE: usize = 0 + <u8 as HasSize>::SIZE + <u16 as HasSize>::SIZE
        + <u32 as HasSize>::SIZE;
}
impl EndianWritableAt for WeirdOrder {
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        let c = self.c;
        writer.write_at(&c, offset);
        let b = self.b;
        writer.write_at(&b, offset + 0 + <u8 as HasSize>::SIZE as isize);
        let a = self.a;
        writer
            .write_at(
                &a,
                offset + 0 + <u8 as HasSize>::SIZE as isize
                    + <u16 as HasSize>::SIZE as isize,
            );
    }
}
impl EndianReadableAt for WeirdOrder {
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let c = <u8 as EndianReadableAt>::read_at(reader, offset);
        let b = <u16 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <u8 as HasSize>::SIZE as isize,
        );
        let a = <u32 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <u8 as HasSize>::SIZE as isize + <u16 as HasSize>::SIZE as isize,
        );
        Self { c, b, a }
    }
}
