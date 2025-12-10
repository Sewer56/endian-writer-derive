use endian_writer_derive::EndianWritable;

#[derive(EndianWritable)]
struct WeirdOrder {
    c: u8,
    b: u16,
    a: u32,
}
