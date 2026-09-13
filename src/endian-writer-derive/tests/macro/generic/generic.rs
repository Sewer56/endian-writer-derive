use endian_writer_derive::EndianWritable;

#[derive(EndianWritable)]
#[repr(C)]
struct Generic<T> {
    inner: T,
    tag: u32,
}
