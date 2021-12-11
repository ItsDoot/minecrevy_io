use std::io;
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};
use once_cell::sync::OnceCell;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Returns the exact byte size of `v` if it were encoded as a VarInt.
pub fn var_i32_byte_length(v: i32) -> usize {
    fn byte_lengths() -> [usize; 33] {
        let mut arr = [0; 33];
        for i in 0..33 {
            arr[i] = ((31.0 - (i as f64 - 1.0)) / 7.0).ceil() as usize;
        }
        arr[32] = 1;
        arr
    }
    static BYTE_LENGTHS: OnceCell<[usize; 33]> = OnceCell::new();

    BYTE_LENGTHS.get_or_init(byte_lengths)[v.leading_zeros() as usize]
}

/// Extends [`Read`] with methods for reading Minecraft protocol data types. (For `std::io`.)
///
/// # Examples
///
/// Read a variable-length-encoded 32 bit integer from a [`Read`]:
///
/// ```rust
/// use std::io::Cursor;
/// use minecrevy_io::ReadMcExt;
///
/// let mut rdr = Cursor::new(vec![0xDD, 0xC7, 0x01]);
/// assert_eq!(25565, rdr.read_var_i32().unwrap());
/// ```
pub trait ReadMcExt: Read {
    /// Reads a variable-length encoded 32 bit integer (`VarInt`) from the underlying reader.
    ///
    /// The 7 least significant bits are used to encode the value and the most significant bit
    /// indicates whether there's another byte after it for the next part of the number.
    /// VarInts are effectively little endian, and are at most 5 bytes long.
    ///
    /// # Examples
    ///
    /// Read a variable-length encoded 32 bit integer from a [`Read`]:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use minecrevy_io::ReadMcExt;
    ///
    /// let mut rdr = Cursor::new(vec![0xDD, 0xC7, 0x01]);
    /// assert_eq!(25565, rdr.read_var_i32().unwrap());
    /// ```
    fn read_var_i32(&mut self) -> io::Result<i32> {
        let mut value: i32 = 0;
        let mut length: u8 = 0;
        loop {
            let byte: u8 = self.read_u8()?;
            value |= i32::from(byte & 0x7F) << (length * 7);

            length += 1;
            if length > 5 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "VarInt is too big"));
            }

            if value & 0x80 != 0x80 {
                break;
            }
        }
        return Ok(value);
    }

    /// Reads a VarInt where it is to be used as a non-negative length.
    fn read_var_i32_length(&mut self) -> io::Result<usize> {
        let length = self.read_var_i32()?;
        usize::try_from(length)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid VarInt length"))
    }
}

/// Extends [`Write`] with methods for writing Minecraft protocol data types. (For `std::io`.)
///
/// # Examples
///
/// Writes a variable-length encoded 32 bit integer to a [`Write`]:
///
/// ```rust
/// use minecrevy_io::WriteMcExt;
///
/// let mut wrt = Vec::new();
/// wrt.write_var_i32(25565).unwrap();
/// assert_eq!(vec![0xDD, 0xC7, 0x01], wrt);
/// ```
pub trait WriteMcExt: Write {
    /// Writes a variable-length encoded 32 bit integer (`VarInt`) to the underlying writer.
    ///
    /// # Examples
    ///
    /// Writes a variable-length encoded 32 bit integer to a [`Write`]:
    ///
    /// ```rust
    /// use minecrevy_io::WriteMcExt;
    ///
    /// let mut wrt = Vec::new();
    /// wrt.write_var_i32(25565).unwrap();
    /// assert_eq!(vec![0xDD, 0xC7, 0x01], wrt);
    /// ```
    fn write_var_i32(&mut self, v: i32) -> io::Result<()> {
        let mut v = v as u32;
        loop {
            if v & !0x7F == 0 {
                self.write_u8(v as u8)?;
            }

            self.write_u8(((v & 0x7F) | 0x80) as u8)?;
            v >>= 7;
        }
    }

    fn write_var_i32_length(&mut self, length: usize) -> io::Result<()> {
        let length = i32::try_from(length)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid VarInt length"))?;
        self.write_var_i32(length)
    }
}

impl<T: Read> ReadMcExt for T {}

impl<T: Write> WriteMcExt for T {}

/// Extends [`AsyncRead`] with methods for reading Minecraft protocol data types asynchronously.
/// (For `tokio::io`.)
#[async_trait::async_trait]
pub trait AsyncReadMcExt: AsyncRead + Unpin {
    async fn read_var_i32(&mut self) -> io::Result<i32> {
        let mut value: i32 = 0;
        let mut length: u8 = 0;
        loop {
            let byte: u8 = self.read_u8().await?;
            value |= i32::from(byte & 0x7F) << (length * 7);

            length += 1;
            if length > 5 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "VarInt is too big"));
            }

            if value & 0x80 != 0x80 {
                break;
            }
        }
        return Ok(value);
    }

    async fn read_var_i32_length(&mut self) -> io::Result<usize> {
        let length = self.read_var_i32().await?;
        usize::try_from(length)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid var i32 length"))
    }
}

/// Extends [`AsyncWrite`] with methods for writing Minecraft protocol data types asynchronously.
/// (For `tokio::io`.)
#[async_trait::async_trait]
pub trait AsyncWriteMcExt: AsyncWrite + Unpin {
    async fn write_var_i32(&mut self, v: i32) -> io::Result<()> {
        let mut v = v as u32;
        loop {
            if v & !0x7F == 0 {
                self.write_u8(v as u8).await?;
            }

            self.write_u8(((v & 0x7F) | 0x80) as u8).await?;
            v >>= 7;
        }
    }
}

impl<T: AsyncRead + Unpin> AsyncReadMcExt for T {}

impl<T: AsyncWrite + Unpin> AsyncWriteMcExt for T {}
