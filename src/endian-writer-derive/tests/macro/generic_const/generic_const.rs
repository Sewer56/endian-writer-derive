use endian_writer_derive::EndianWritable;

// `N` is a const parameter, not a type parameter; the derive must carry it
// through every emitted impl without adding trait bounds to it.
#[derive(EndianWritable)]
#[repr(C)]
struct WithConst<T, const N: usize> {
    inner: T,
    bytes: [u8; N],
    tag: u32,
}
