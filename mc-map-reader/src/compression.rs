use std::io::Read;

pub fn decompress(data: &[u8], compression: &Compression) -> Result<Vec<u8>, Error> {
    let mut decompressed = Vec::new();
    match compression {
        Compression::GZip => libflate::gzip::Decoder::new(data)?.read_to_end(&mut decompressed),
        Compression::Zlib => compress::zlib::Decoder::new(data).read_to_end(&mut decompressed),
        Compression::Uncompressed => unimplemented!(),
        Compression::Other => unimplemented!(),
    }?;
    Ok(decompressed)
}

pub enum Compression {
    GZip = 1,
    Zlib = 2,
    Uncompressed = 3,
    Other,
}

impl From<u8> for Compression {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::GZip,
            2 => Self::Zlib,
            3 => Self::Uncompressed,
            _ => Self::Other,
        }
    }
}

pub type Error = std::io::Error;
