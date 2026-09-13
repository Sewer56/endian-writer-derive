use endian_writer_derive::EndianWritable;

#[derive(EndianWritable)]
#[repr(C)]
struct Multi<T, U>
where
    U: Copy,
{
    first: T,
    second: U,
    count: u16,
}
