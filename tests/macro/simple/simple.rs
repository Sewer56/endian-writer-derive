use endian_writer_derive::EndianWritable;

#[derive(EndianWritable)]
#[repr(C)]
struct Simple {
    a: u32,
    b: u16,
    c: u8,
}
