use std::io::Read;

/// Decompresses the given data using the given compression.
pub fn decompress(data: &[u8], compression: &Compression) -> Result<Vec<u8>, Error> {
    let mut decompressed = Vec::new();
    match compression {
        Compression::GZip => libflate::gzip::Decoder::new(data)?.read_to_end(&mut decompressed),
        Compression::Zlib => compress::zlib::Decoder::new(data).read_to_end(&mut decompressed),
        Compression::Uncompressed => return Ok(data.iter().map(|v| *v).collect::<Vec<_>>()),
        Compression::Other => unimplemented!("Only GZip, ZLib and Uncompressed are supported"),
    }?;
    Ok(decompressed)
}

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use std::io::Write;
    use test_case::test_case;

    use super::Compression;

    #[test_case(1, Compression::GZip; "GZip")]
    #[test_case(2, Compression::Zlib; "ZLib")]
    #[test_case(3, Compression::Uncompressed; "Uncompressed")]
    #[test_case(4, Compression::Other; "Other_4")]
    #[test_case(5, Compression::Other; "Other_5")]
    #[test_case(0, Compression::Other; "Other_0")]
    fn compression_from(value: u8, expected: Compression) {
        assert_eq!(expected, value.into())
    }

    #[test]
    #[should_panic(expected = "Only GZip, ZLib and Uncompressed are supported")]
    fn compression_other_unimplemented() {
        super::decompress(&[], &Compression::Other).unwrap();
    }

    #[test]
    fn decompress_uncompressed() {
        let result = super::decompress(&[1,2,3,4,5,6,7,8,9,10], &Compression::Uncompressed).unwrap();
        let expected = vec![1,2,3,4,5,6,7,8,9,10];
        assert_eq!(result, expected);
    }

    #[test]
    fn decompress_gzip() {
        let mut encoded = Vec::new();
        let mut encoder = libflate::gzip::Encoder::new(&mut encoded).unwrap();
        encoder.write_all(b"Hello World").unwrap();
        encoder.finish().unwrap();
        assert!(encoded.len() > 0);
        
        let decoded = super::decompress(&encoded, &Compression::GZip).unwrap();
        
        assert_eq!(decoded.as_slice(), b"Hello World");
    }

    #[test]
    fn decompress_zlib() {
        let mut encoded = Vec::new();
        let mut encoder = libflate::zlib::Encoder::new(&mut encoded).unwrap();
        encoder.write_all(b"Hello World").unwrap();
        encoder.finish().unwrap();
        assert!(encoded.len() > 0);
        
        let decoded = super::decompress(&encoded, &Compression::Zlib).unwrap();
        
        assert_eq!(decoded.as_slice(), b"Hello World");
    }

    #[test]
    fn decompress_invalid() {
        let res = super::decompress(&[1,2,3,4,5,6,7,8,9,10], &Compression::GZip);
        assert!(res.is_err());
    }

}
