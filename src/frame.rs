use std::io;
use std::io::{Cursor, Read, Seek, Write};
use std::num::NonZeroUsize;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{AsyncReadMcExt, ReadMcExt, var_i32_byte_length, WriteMcExt};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct PacketFrame {
    pub(crate) content_len: Option<NonZeroUsize>,
    pub(crate) id: i32,
    pub(crate) body: Vec<u8>,
}

impl PacketFrame {
    pub async fn read<R: AsyncRead + Unpin + Send>(mut reader: R) -> io::Result<Self> {
        let content_len = reader.read_var_i32_length().await?;
        let mut content = Vec::with_capacity(content_len);
        reader.read_exact(&mut content).await?;

        Self::read_content(Cursor::new(content), content_len)
    }

    pub fn read_sync<R: Read>(mut reader: R) -> io::Result<Self> {
        let content_len = reader.read_var_i32_length()?;
        let mut content = Vec::with_capacity(content_len);
        reader.read_exact(&mut content)?;

        Self::read_content(Cursor::new(content), content_len)
    }

    fn read_content<R: Read>(mut reader: R, content_len: usize) -> io::Result<Self> {
        let id = reader.read_var_i32()?;
        let mut body = Vec::new();
        reader.read_to_end(&mut body)?;

        Ok(Self {
            content_len: NonZeroUsize::new(content_len),
            id,
            body,
        })
    }

    pub async fn write<W: AsyncWrite + Unpin>(&self, mut writer: W) -> io::Result<()> {
        let mut bytes = Vec::<u8>::with_capacity(self.len());
        self.write_sync(&mut bytes)?;
        writer.write_all(&bytes).await?;
        Ok(())
    }

    pub fn write_sync<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_var_i32(self.content_len() as i32)?;
        writer.write_var_i32(self.id)?;
        writer.write_all(&self.body)?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        let content_len = self.content_len();
        var_i32_byte_length(content_len as i32) + content_len
    }

    pub fn content_len(&self) -> usize {
        self.content_len
            .map(|len| len.get())
            .unwrap_or_else(|| var_i32_byte_length(self.id) + self.body.len())
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn body(&self) -> impl Read + Seek + '_ {
        Cursor::new(&self.body)
    }
}

pub trait FromPacketFrame: Sized {
    fn from(frame: &PacketFrame) -> Self;
}

pub trait ToPacketFrame: Sized {
    fn into(&self) -> PacketFrame;
}
