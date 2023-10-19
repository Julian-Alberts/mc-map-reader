use crate::file::{FileItemRead, FileItemWrite};
use async_std::io::{Read, Write};
use async_trait::async_trait;

pub struct RegionInventories {
    pub inventories: Vec<Inventory>,
}

pub struct Inventory {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub items: Vec<Item>,
}

pub struct Item {
    pub group_id: u64,
    pub count: u64,
}

#[async_trait]
impl<R> FileItemRead<R> for RegionInventories
where
    R: Read + Unpin + Send + Sync,
{
    async fn read(data: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            inventories: FileItemRead::read(data).await?,
        })
    }
}

#[async_trait]
impl<W> FileItemWrite<W> for RegionInventories
where
    W: Write + Unpin + Send + Sync,
{
    async fn write(&self, data: &mut W) -> std::io::Result<()> {
        self.inventories.write(data).await
    }
}

#[async_trait]
impl<R> FileItemRead<R> for Inventory
where
    R: Read + Unpin + Send + Sync,
{
    async fn read(data: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            x: FileItemRead::read(data).await?,
            y: FileItemRead::read(data).await?,
            z: FileItemRead::read(data).await?,
            items: FileItemRead::read(data).await?,
        })
    }
}

#[async_trait]
impl<W> FileItemWrite<W> for Inventory
where
    W: Write + Unpin + Send + Sync,
{
    async fn write(&self, data: &mut W) -> std::io::Result<()> {
        self.x.write(data).await?;
        self.y.write(data).await?;
        self.z.write(data).await?;
        self.items.write(data).await?;
        Ok(())
    }
}

#[async_trait]
impl<R> FileItemRead<R> for Item
where
    R: Read + Unpin + Send + Sync,
{
    async fn read(data: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            group_id: FileItemRead::read(data).await?,
            count: FileItemRead::read(data).await?,
        })
    }
}

#[async_trait]
impl<W> FileItemWrite<W> for Item
where
    W: Write + Unpin + Send + Sync,
{
    async fn write(&self, data: &mut W) -> std::io::Result<()> {
        self.group_id.write(data).await?;
        self.count.write(data).await?;
        Ok(())
    }
}
