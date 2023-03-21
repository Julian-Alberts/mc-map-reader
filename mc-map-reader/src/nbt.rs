use std::{collections::HashMap, ops::Deref, vec::IntoIter};

use thiserror::Error;

macro_rules! tags {
    ($({
        id: $id:literal,
        tag_type: $tag_type:ident,
        $(
            payload: $ty:ty,
            converter: $converter:ident,
            getter: $getter:ident,
        )?
        description: $description:literal
    }),*) => {
        #[derive(Debug, Clone, PartialEq)]
        /// Tags are used to store data in the NBT format.
        pub enum Tag {
            $(
                #[doc=$description]
                $tag_type $(($ty))?
            ),*
        }

        impl Tag {
            fn new(id: u8, data: &[u8], offset: &mut usize) -> Result<Tag, Error> {
                let tag = match id {
                    $($id => Self::$tag_type$(($converter(data, offset)?))?,)*
                    other => {
                        log::error!("Unknown tag id: {}", other);
                        return Err(Error::UnknownTagId(other))
                    }
                };
                Ok(tag)
            }
            #[allow(unused_variables)]
            fn get_id(&self) -> u8 {
                match self {
                    $(Self::$tag_type$(($converter))? => $id),*
                }
            }

            $($(
            /// Returns the value of the tag if it is of the correct type.
            pub fn $getter(self) -> Result<$ty, Error> {
                if let Self::$tag_type(v) = self {
                    Ok(v)
                } else {
                    log::error!("Tried to get {} from tag of type {}", stringify!($ty), self.get_id());
                    Err(Error::InvalidValue)
                }
            }
            )?)*
        }

        $($(
        impl NbtData for $ty {
            type BuildError = Error;
        }
        impl TryFrom<Tag> for $ty {
            type Error = Error;
            fn try_from(value: Tag) -> Result<$ty, Self::Error> {
                if let Tag::$tag_type(v) = value {
                    Ok(v)
                } else {
                    Err(Error::InvalidValue)
                }
            }
        }
        )?)*
    };
}

// TODO add test
/// All possible NBT data types must implement this trait.
/// Most of the time this is done by macros.
pub trait NbtData: TryFrom<Tag, Error = Self::BuildError>
where
    Self::BuildError: From<Error>,
{
    /// The error type that is returned when building the data type using TryFrom<Tag> fails.
    type BuildError;
}

// TODO add test
impl<T> TryFrom<Tag> for List<T>
where
    T: NbtData,
{
    type Error = T::BuildError;
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        let values = value
            .get_as_list()?
            .0
            .into_iter()
            .map(T::try_from)
            .collect::<Result<_, _>>()?;
        Ok(values)
    }
}

// TODO add test
impl<T> NbtData for HashMap<String, T>
where
    T: NbtData,
{
    type BuildError = T::BuildError;
}

// TODO add test
impl<T> TryFrom<Tag> for HashMap<String, T>
where
    T: NbtData,
{
    type Error = T::BuildError;
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        let values = value
            .get_as_map()?
            .into_iter()
            .map(|(k, v)| T::try_from(v).map(|v| (k, v)))
            .collect::<Result<_, _>>()?;
        Ok(values)
    }
}

// TODO add test
impl TryFrom<Tag> for bool {
    type Error = Error;
    fn try_from(value: Tag) -> Result<bool, Self::Error> {
        match value {
            Tag::Byte(1) => Ok(true),
            Tag::Byte(_) => Ok(false),
            _ => Err(Error::InvalidValue),
        }
    }
}

// TODO add test
impl<T> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

// TODO add test
impl<T> IntoIterator for List<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// TODO add test
impl<A> FromIterator<A> for List<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

tags![
{
    id: 0,
    tag_type: End,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 1,
    tag_type: Byte,
    payload: i8,
    converter: convert_to_i8,
    getter: get_as_i8,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 2,
    tag_type: Short,
    payload: i16,
    converter: convert_to_i16,
    getter: get_as_i16,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 3,
    tag_type: Int,
    payload: i32,
    converter: convert_to_i32,
    getter: get_as_i32,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 4,
    tag_type: Long,
    payload: i64,
    converter: convert_to_i64,
    getter: get_as_i64,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 5,
    tag_type: Float,
    payload: f32,
    converter: convert_to_f32,
    getter: get_as_f32,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 6,
    tag_type: Double,
    payload: f64,
    converter: convert_to_f64,
    getter: get_as_f64,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 7,
    tag_type: ByteArray,
    payload: Array<i8>,
    converter: convert_to_i8_array,
    getter: get_as_i8_array,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 8,
    tag_type: String,
    payload: String,
    converter: convert_to_string,
    getter: get_as_string,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 9,
    tag_type: List,
    payload: List<Tag>,
    converter: convert_to_list,
    getter: get_as_list,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 10,
    tag_type: Compound,
    payload: HashMap<String, Tag>,
    converter: convert_to_map,
    getter: get_as_map,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 11,
    tag_type: IntArray,
    payload: Array<i32>,
    converter: convert_to_32_array,
    getter: get_as_i32_array,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
},
{
    id: 12,
    tag_type: LongArray,
    payload: Array<i64>,
    converter: convert_to_i64_array,
    getter: get_as_i64_array,
    description: "Used to mark the end of compound tags. This tag does not have a name, so it is only ever a single byte 0. It may also be the type of empty List tags."
}
];

/// A NBT Array of a specific type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array<T>(Vec<T>);

/// A NBT List of a specific type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<T>(Vec<T>);

// TODO Add test
impl<T> List<T> {
    /// Get the inner vector.
    pub fn take(self) -> Vec<T> {
        self.0
    }
    /// Get an iterator over the data.
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.0.iter()
    }
}

/// A generic error type which represents all possible errors that can occur when parsing NBT.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// The given tag ID is not valid.
    #[error("Unknown Tag ID: {0}")]
    UnknownTagId(u8),
    /// The given value is not valid.
    #[error("Invalid Value")]
    InvalidValue,
}

// TODO Add test
impl<T> Deref for Array<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO Add test
impl<T> Deref for List<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO Add test
/// Parse a NBT tag from a byte slice.
pub fn parse(data: &[u8]) -> Result<Tag, Error> {
    match data[0] {
        10 => Tag::new(10, data, &mut 3),
        out => panic!("{out}"),
    }
}

fn convert_to_i8(data: &[u8], offset: &mut usize) -> Result<i8, Error> {
    let result = data[*offset] as i8;
    *offset += 1;
    Ok(result)
}

fn convert_to_i16(data: &[u8], offset: &mut usize) -> Result<i16, Error> {
    let result = i16::from_be_bytes([data[*offset], data[*offset + 1]]);
    *offset += 2;
    Ok(result)
}

fn convert_to_i32(data: &[u8], offset: &mut usize) -> Result<i32, Error> {
    let result = i32::from_be_bytes([
        data[*offset],
        data[*offset + 1],
        data[*offset + 2],
        data[*offset + 3],
    ]);
    *offset += 4;
    Ok(result)
}

fn convert_to_i64(data: &[u8], offset: &mut usize) -> Result<i64, Error> {
    let result = i64::from_be_bytes([
        data[*offset],
        data[*offset + 1],
        data[*offset + 2],
        data[*offset + 3],
        data[*offset + 4],
        data[*offset + 5],
        data[*offset + 6],
        data[*offset + 7],
    ]);
    *offset += 8;
    Ok(result)
}

//TODO Add test
fn convert_to_f32(data: &[u8], offset: &mut usize) -> Result<f32, Error> {
    let result = f32::from_be_bytes([
        data[*offset],
        data[*offset + 1],
        data[*offset + 2],
        data[*offset + 3],
    ]);
    *offset += 4;
    Ok(result)
}

//TODO Add test
fn convert_to_f64(data: &[u8], offset: &mut usize) -> Result<f64, Error> {
    let result = f64::from_be_bytes([
        data[*offset],
        data[*offset + 1],
        data[*offset + 2],
        data[*offset + 3],
        data[*offset + 4],
        data[*offset + 5],
        data[*offset + 6],
        data[*offset + 7],
    ]);
    *offset += 8;
    Ok(result)
}

fn convert_to_i8_array(data: &[u8], offset: &mut usize) -> Result<Array<i8>, Error> {
    let len = convert_to_i32(data, offset)? as usize;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(convert_to_i8(data, offset)?)
    }
    Ok(Array(result))
}

fn convert_to_string(data: &[u8], offset: &mut usize) -> Result<String, Error> {
    let len = convert_to_i16(data, offset)? as usize;
    let str_data = data[*offset..len + *offset].to_vec();
    *offset += len;
    String::from_utf8(str_data).or(Err(Error::InvalidValue))
}

fn convert_to_list(data: &[u8], offset: &mut usize) -> Result<List<Tag>, Error> {
    let item_type = convert_to_i8(data, offset)? as u8;
    let len = convert_to_i32(data, offset)? as usize;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(Tag::new(item_type, data, offset)?);
    }
    Ok(List(result))
}

fn convert_to_map(data: &[u8], offset: &mut usize) -> Result<HashMap<String, Tag>, Error> {
    let mut map = HashMap::new();

    while data.len() > *offset {
        let value_type = convert_to_i8(data, offset)? as u8;
        if value_type == Tag::End.get_id() {
            break;
        }
        let key = convert_to_string(data, offset)?;
        let tag = Tag::new(value_type, data, offset)?;
        map.insert(key, tag);
    }
    Ok(map)
}

fn convert_to_32_array(data: &[u8], offset: &mut usize) -> Result<Array<i32>, Error> {
    let len = convert_to_i32(data, offset)? as usize;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(convert_to_i32(data, offset)?)
    }
    Ok(Array(result))
}

fn convert_to_i64_array(data: &[u8], offset: &mut usize) -> Result<Array<i64>, Error> {
    let len = convert_to_i32(data, offset)? as usize;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(convert_to_i64(data, offset)?)
    }
    Ok(Array(result))
}

#[cfg(test)]
mod tests {

    use test_case::test_case;
    use super::Tag;

    #[test_case(&[10], 0 => 10; "Single byte array")]
    #[test_case(&[1,2,3,4,5,6,7], 0 => 1; "Multi byte array")]
    #[test_case(&[1,2,3,4,5,6,7], 3 => 4; "Offset in array")]
    fn test_convert_to_i8(data: &[u8], mut offset: usize) -> i8 {
        let orig_offset = offset;
        let result = super::convert_to_i8(data, &mut offset).unwrap();
        assert_eq!(offset, orig_offset + 1);
        result
    }

    #[test_case(&[0, 10], 0 => 10; "Single value array")]
    #[test_case(&[0, 1, 0, 2, 0, 3, 0, 4], 0 => 1; "Multi value array")]
    #[test_case(&[0, 1, 0, 2, 0, 3, 0, 4], 2 => 2; "Offset in array")]
    #[test_case(&[0, 1, 0, 2, 0, 3, 0, 4], 5 => 768; "Big value")]
    #[test_case(&[0, 1, 0, 2, 0, 3, 3, 4], 5 => 771; "Multi byte value")]
    fn test_convert_to_i16(data: &[u8], mut offset: usize) -> i16 {
        let orig_offset = offset;
        let result = super::convert_to_i16(data, &mut offset).unwrap();
        assert_eq!(offset, orig_offset + 2);
        result
    }

    #[test_case(&[0, 0, 0, 10], 0 => 10; "Single value array")]
    #[test_case(&[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4], 0 => 1; "Multi value array")]
    #[test_case(&[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4], 4 => 2; "Offset in array")]
    #[test_case(&[1, 1, 1, 1], 0 => 0b1_0000_0001_0000_0001_0000_0001; "Big value")]
    fn test_convert_to_i32(data: &[u8], mut offset: usize) -> i32 {
        let orig_offset = offset;
        let result = super::convert_to_i32(data, &mut offset).unwrap();
        assert_eq!(offset, orig_offset + 4);
        result
    }
    #[test_case(&[0, 0, 0, 0, 0, 0, 0, 10], 0 => 10; "Single value array")]
    #[test_case(&[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 4], 4 => 3; "Offset in array")]
    #[test_case(&[1, 1, 1, 1, 1, 1, 1, 1], 0 => 0b1_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001; "Big value")]
    fn test_convert_to_i64(data: &[u8], mut offset: usize) -> i64 {
        let orig_offset = offset;
        let result = super::convert_to_i64(data, &mut offset).unwrap();
        assert_eq!(offset, orig_offset + 8);
        result
    }

    #[test_case(&[0, 0, 0, 1, 1], 0 => vec![1]; "Single value array")]
    #[test_case(&[0, 0, 0, 4, 1, 2, 3, 4], 0 => vec![1,2,3,4]; "Multi value array")]
    fn test_convert_to_i8_array(data: &[u8], mut offset: usize) -> Vec<i8> {
        let orig_offset = offset;
        let result = super::convert_to_i8_array(data, &mut offset).unwrap();
        assert_eq!(offset, orig_offset + 4 + result.0.len());
        result.0
    }

    #[test]
    fn test_convert_to_string() {
        let data = &[0, 5, b'H', b'e', b'l', b'l', b'o'];
        let mut offset = 0;
        let result = super::convert_to_string(data, &mut offset).unwrap();
        assert_eq!(offset, 7);
        assert_eq!(result, "Hello");
    }

    #[test_case(&[1, 0, 0, 0, 1, 1], 0 => vec![Tag::Byte(1)]; "Single value")]
    #[test_case(&[1, 0, 0, 0, 2, 1, 255], 0 => vec![Tag::Byte(1), Tag::Byte(-1)]; "Multi value")]
    fn test_convert_to_list(data: &[u8], mut offset: usize) -> Vec<Tag> {
        let orig_offset = offset;
        let result = super::convert_to_list(data, &mut offset).unwrap();
        assert_eq!(offset, orig_offset + 5 + result.0.len());
        result.0
    }

    #[test_case(&[0], 0 => Vec::<(String, Tag)>::new(); "Empty map")]
    #[test_case(&[1, 0, 1, b'A', 1, 0], 0 => vec![("A".to_string(), Tag::Byte(1))]; "Single value in map")]
    #[test_case(&[1, 0, 1, b'A', 1, 8, 0, 2, b'B', b'B', 0, 4, b'A', b'B', b'C', b'D', 0], 0 => vec![("A".to_string(), Tag::Byte(1)), ("BB".to_string(), Tag::String("ABCD".to_string()))]; "Multi value in map")]
    fn test_convert_to_compound(data: &[u8], mut offset: usize) -> Vec<(String, Tag)> {
        let mut result = super::convert_to_map(data, &mut offset).unwrap()
            .into_iter().collect::<Vec<_>>();
        result.sort_by(|a, b| a.0.cmp(&b.0));
        result
    }

    
}
