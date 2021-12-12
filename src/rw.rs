///! Traits for reading/writing Minecraft data types (not packets themselves).

use std::io;
use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub use minecrevy_io_derive::McRead;
use crate::ReadMcExt;

pub trait McRead: Sized {
    fn read<R: Read>(reader: R) -> io::Result<Self>;

    fn read_into<R: Read>(&mut self, reader: R) -> io::Result<()> {
        *self = Self::read(reader)?;
        Ok(())
    }
}

macro_rules! mcread_impl {
    ($($ty:ty => $fn:expr),+) => {
        $(
        impl McRead for $ty {
            fn read<R: Read>(mut reader: R) -> io::Result<Self> {
                $fn(&mut reader)
            }
        }
        )+
    };
}

mcread_impl!(
    u8   => ReadBytesExt::read_u8,
    u16  => ReadBytesExt::read_u16::<BigEndian>,
    u32  => ReadBytesExt::read_u32::<BigEndian>,
    u64  => ReadBytesExt::read_u64::<BigEndian>,
    u128 => ReadBytesExt::read_u128::<BigEndian>,
    i8   => ReadBytesExt::read_i8,
    i16  => ReadBytesExt::read_i16::<BigEndian>,
    i32  => ReadBytesExt::read_i32::<BigEndian>,
    i64  => ReadBytesExt::read_i64::<BigEndian>,
    i128 => ReadBytesExt::read_i128::<BigEndian>,
    f32  => ReadBytesExt::read_f32::<BigEndian>,
    f64  => ReadBytesExt::read_f64::<BigEndian>
);

impl McRead for bool {
    fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        Ok(reader.read_u8()? != 0x00)
    }
}

impl McRead for String {
    fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let len = reader.read_var_i32_length()?;
    }
}

pub trait McWrite {
    fn write<W: Write>(&self, writer: W) -> io::Result<()>;
}

macro_rules! mcwrite_impl {
    ($($ty:ty => $fn:expr),+) => {
        $(
        impl McWrite for $ty {
            fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
                $fn(&mut writer, *self)
            }
        }
        )+
    };
}

mcwrite_impl!(
    u8   => WriteBytesExt::write_u8,
    u16  => WriteBytesExt::write_u16::<BigEndian>,
    u32  => WriteBytesExt::write_u32::<BigEndian>,
    u64  => WriteBytesExt::write_u64::<BigEndian>,
    u128 => WriteBytesExt::write_u128::<BigEndian>,
    i8   => WriteBytesExt::write_i8,
    i16  => WriteBytesExt::write_i16::<BigEndian>,
    i32  => WriteBytesExt::write_i32::<BigEndian>,
    i64  => WriteBytesExt::write_i64::<BigEndian>,
    i128 => WriteBytesExt::write_i128::<BigEndian>,
    f32  => WriteBytesExt::write_f32::<BigEndian>,
    f64  => WriteBytesExt::write_f64::<BigEndian>
);

impl McWrite for bool {
    fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_u8(if *self { 0x01 } else { 0x00 })
    }
}
