use std::io::Result;
use async_std::io::{Read, ReadExt, Write, WriteExt};
use async_trait::async_trait;
use crate::file::{FileItemRead, FileItemWrite};

#[async_trait]
impl <R> FileItemRead<R> for i32
    where R: Read + Unpin + Send + Sync
{
    async fn read(data: &mut R) -> Result<Self> {
        let mut bytes = [0; std::mem::size_of::<i32>()];
        data.read_exact(&mut bytes).await?;
        Ok(i32::from_be_bytes(bytes))
    }
}

#[async_trait]
impl <W> FileItemWrite<W> for i32
    where W: Write + Unpin + Send + Sync
{
    async fn write(&self, data: &mut W) -> Result<()> {
        data.write_all(self.to_be_bytes().as_slice()).await
    }
}

#[async_trait]
impl <R: Read> FileItemRead<R> for u32
    where R: Read + Unpin + Send + Sync
{
    async fn read(data: &mut R) -> Result<Self> {
        let mut bytes = [0; 4];
        data.read_exact(&mut bytes).await?;
        let self_ = u32::from_be_bytes(bytes);
        Ok(self_)
    }
}

#[async_trait]
impl <W> FileItemWrite<W> for u32
    where W: Write + Unpin + Send + Sync
{
    async fn write(&self, data: &mut W) -> Result<()> {
        data.write_all(&self.to_be_bytes()).await
    }
}

#[async_trait]
impl <R, T> FileItemRead<R> for Vec<T>
where
    T: FileItemRead<R> + Send + Sync,
    R: Read + Unpin + Send + Sync,
{
    async fn read(data: &mut R) -> Result<Self> {
        let len = u32::read(data).await?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::read(data).await?);
        }
        Ok(vec)
    }
}

#[async_trait]
impl <W, T> FileItemWrite<W> for Vec<T>
    where
        T: FileItemWrite<W> + Send + Sync,
        W: Write + Unpin + Send + Sync,
{
    async fn write(&self, data: &mut W) -> Result<()> {
        (self.len() as u32).write(data).await?;
        for item in self {
            item.write(data).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::file::{Write, Read};

    mod i32 {
        use crate::file::{FileItemRead, FileItemWrite};
        use super::*;

        #[async_std::test]
        async fn i32_serialize() {
            let mut vec = Vec::new();
            10_i32.write(&mut vec).await;
            assert_eq!(&[0, 0, 0, 10], vec.as_slice());
        }

        #[async_std::test]
        async fn i32_from_slice() {
            let mut slice = [0u8, 0, 0, 10].as_slice();
            assert_eq!(10_i32, i32::read(&mut slice).await.expect("Unexpected Error"));
        }

        #[async_std::test]
        async fn i32_serialize_into_none_empty() {
            let mut vec = Vec::from_iter([5, 5, 5, 5]);
            10_i32.write(&mut vec).await;
            assert_eq!(&[5, 5, 5, 5, 0, 0, 0, 10], vec.as_slice());
        }

        #[async_std::test]
        async fn i32_from_long_slice() {
            let mut slice = [0u8, 0, 0, 10, 123, 43, 54].as_slice();
            assert_eq!(10_i32, i32::read(&mut slice).await.expect("Unexpected Error"));
            assert_eq!(slice.len(), 3);
        }

        #[async_std::test]
        async fn i32_from_slice_error() {
            let mut slice = [0u8, 0, 0].as_slice();
            assert!(i32::read(&mut slice).await.is_err());
        }

        #[async_std::test]
        async fn i32_serialize_neg_value() {
            let mut vec = Vec::new();
            (-1_i32).write(&mut vec).await;
            assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF], vec.as_slice());
        }

        #[async_std::test]
        async fn i32_from_slice_neg_value() {
            let mut slice = [0xFF, 0xFF, 0xFF, 0xFF].as_slice();
            assert_eq!(-1, i32::read(&mut slice).await.expect("Unexpected Error"));
        }
    }
    mod u32 {
        use crate::file::{FileItemRead, FileItemWrite};

        #[async_std::test]
        async fn u32_serialize() {
            let mut vec = Vec::new();
            10_u32.write(&mut vec).await;
            assert_eq!(&[0, 0, 0, 10], vec.as_slice());
        }

        #[async_std::test]
        async fn u32_from_slice() {
            let mut slice = [0u8, 0, 0, 10].as_slice();
            assert_eq!(10_u32, u32::read(&mut slice).await.expect("Unexpected Error"));
        }

        #[async_std::test]
        async fn u32_serialize_into_none_empty() {
            let mut vec = Vec::from_iter([5, 5, 5, 5]);
            10_u32.write(&mut vec).await;
            assert_eq!(&[5, 5, 5, 5, 0, 0, 0, 10], vec.as_slice());
        }

        #[async_std::test]
        async fn u32_from_long_slice() {
            let mut slice = [0u8, 0, 0, 10, 123, 43, 54].as_slice();
            assert_eq!(10_u32, u32::read(&mut slice).await.expect("Unexpected Error"));
            assert_eq!(slice.len(), 3);
        }

        #[async_std::test]
        async fn u32_from_slice_error() {
            let mut slice = [0u8, 0, 0].as_slice();
            assert!(u32::read(&mut slice).await.is_err());
        }
    }
    mod vec {
        use crate::file::{FileItemRead, FileItemWrite};

        #[async_std::test]
        async fn serialize_empty_vec() {
            let vec: Vec<i32> = Vec::new();
            let mut res_vec = Vec::new();

            vec.write(&mut res_vec).await;
            assert_eq!(&[0,0,0,0], res_vec.as_slice());
            assert_eq!(vec, Vec::<i32>::read(&mut res_vec.as_slice()).await.expect("Unexpected Error"));
        }

        #[async_std::test]
        async fn serialize_vec_with_one_item() {
            let vec = Vec::from_iter([0x01FF]);
            let mut res_vec = Vec::new();
            vec.write(&mut res_vec).await;
            assert_eq!(&[0,0,0,1,0,0,0x01,0xFF], res_vec.as_slice());
            assert_eq!(vec, Vec::<i32>::read(&mut res_vec.as_slice()).await.expect("Unexpected Error"));
        }

        #[async_std::test]
        async fn serialize_vec_with_multiple_items() {
            let vec = Vec::from_iter([1,2,3,4]);
            let mut res_vec = Vec::new();
            vec.write(&mut res_vec).await.expect("Unexpected Error");
            assert_eq!(&[0,0,0,4, 0,0,0,1, 0,0,0,2, 0,0,0,3, 0,0,0,4], res_vec.as_slice());
            assert_eq!(vec, Vec::<i32>::read(&mut res_vec.as_slice()).await.expect("Unexpected Error"));
        }
    }
}
