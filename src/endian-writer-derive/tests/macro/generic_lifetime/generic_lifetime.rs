use endian_writer_derive::EndianWritable;

// `'a` can't appear in any field; the derive must still carry it through.
#[derive(EndianWritable)]
#[repr(C)]
struct WithLifetime<'a, T> {
    inner: T,
    tag: u8,
}
