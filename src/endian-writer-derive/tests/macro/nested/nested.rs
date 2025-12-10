use endian_writer_derive::EndianWritable;
#[derive(EndianWritable, Copy, Clone)]
#[repr(C)]
struct Inner {
    x: u16,
    y: u32,
}

#[derive(EndianWritable)]
#[repr(C)]
struct Outer {
    a: u8,
    b: Inner,
    c: u64,
}
