use endian_writer_derive::EndianWritable;
#[repr(C)]
struct Inner {
    x: u16,
    y: u32,
}
use endian_writer::*;
impl HasSize for Inner {
    const SIZE: usize = 0 + <u16 as HasSize>::SIZE + <u32 as HasSize>::SIZE;
}
impl EndianWritableAt for Inner {
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        let x = self.x;
        writer.write_at(&x, offset);
        let y = self.y;
        writer.write_at(&y, offset + 0 + <u16 as HasSize>::SIZE as isize);
    }
}
impl EndianReadableAt for Inner {
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let x = <u16 as EndianReadableAt>::read_at(reader, offset);
        let y = <u32 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <u16 as HasSize>::SIZE as isize,
        );
        Self { x, y }
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Inner {}
#[automatically_derived]
impl ::core::clone::Clone for Inner {
    #[inline]
    fn clone(&self) -> Inner {
        let _: ::core::clone::AssertParamIsClone<u16>;
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}
#[repr(C)]
struct Outer {
    a: u8,
    b: Inner,
    c: u64,
}
use endian_writer::*;
impl HasSize for Outer {
    const SIZE: usize = 0 + <u8 as HasSize>::SIZE + <Inner as HasSize>::SIZE
        + <u64 as HasSize>::SIZE;
}
impl EndianWritableAt for Outer {
    unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
        let a = self.a;
        writer.write_at(&a, offset);
        let b = self.b;
        writer.write_at(&b, offset + 0 + <u8 as HasSize>::SIZE as isize);
        let c = self.c;
        writer
            .write_at(
                &c,
                offset + 0 + <u8 as HasSize>::SIZE as isize
                    + <Inner as HasSize>::SIZE as isize,
            );
    }
}
impl EndianReadableAt for Outer {
    unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
        let a = <u8 as EndianReadableAt>::read_at(reader, offset);
        let b = <Inner as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <u8 as HasSize>::SIZE as isize,
        );
        let c = <u64 as EndianReadableAt>::read_at(
            reader,
            offset + 0 + <u8 as HasSize>::SIZE as isize
                + <Inner as HasSize>::SIZE as isize,
        );
        Self { a, b, c }
    }
}
