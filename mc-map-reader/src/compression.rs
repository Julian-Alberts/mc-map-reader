use std::io::Read;

use thiserror::Error;

pub fn decompress(data: &[u8], compression: &Compression) -> Result<Vec<u8>, Error> {
    
    let decompressed = match compression {
        Compression::GZip => {
            yazi::decompress(data, yazi::Format::Zlib).map_err(|e| Error::GZip(e))?.0
        },
        Compression::Zlib => {
            let mut decompressed = Vec::new();
            compress::zlib::Decoder::new(data).read_to_end(&mut decompressed).map_err(|e| Error::Zlib(e))?;
            decompressed
        }
        Compression::Uncompressed => unimplemented!(),
        Compression::Other => unimplemented!(),
    };
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

#[derive(Debug, Error)]
pub enum Error {
    #[error("GZip error: {0:#?}")]
    GZip(yazi::Error),
    #[error(transparent)]
    Zlib(std::io::Error),
}
