use async_std::io::{Read, Write};

pub mod data_types;
pub mod region_inventories;

#[async_trait::async_trait]
pub trait FileItemWrite<W>: Sized
where
    W: Write + Unpin + Send + Sync,
{
    async fn write(&self, write: &mut W) -> std::io::Result<()>;
}

#[async_trait::async_trait]
pub trait FileItemRead<R>: Sized
where
    R: Read + Unpin + Send + Sync,
{
    async fn read(read: &mut R) -> std::io::Result<Self>;
}
