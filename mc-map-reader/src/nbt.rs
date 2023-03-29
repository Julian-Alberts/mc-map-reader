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
        impl From<$ty> for Tag {
            fn from(value: $ty) -> Self {
                Self::$tag_type(value)
            }
        }
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

/// All possible NBT data types must implement this trait.
/// Most of the time this is done by macros.
pub trait NbtData: TryFrom<Tag, Error = Self::BuildError>
where
    Self::BuildError: From<Error>,
{
    /// The error type that is returned when building the data type using TryFrom<Tag> fails.
    type BuildError;
}

impl<T> TryFrom<Tag> for List<T>
where
    T: NbtData,
{
    type Error = T::BuildError;
    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        let values = value
            .get_as_list()?
            .take()
            .into_iter()
            .map(T::try_from)
            .collect::<Result<_, _>>()?;
        Ok(values)
    }
}

impl<T> NbtData for HashMap<String, T>
where
    T: NbtData,
{
    type BuildError = T::BuildError;
}

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

impl<T> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> IntoIterator for List<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<A> FromIterator<A> for Array<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

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

impl<T> Deref for Array<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for List<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Parse a NBT tag from a byte slice.
pub fn parse(data: &[u8]) -> Result<Tag, Error> {
    match data[0] {
        10 => Tag::new(10, data, &mut 3),
        _ => Err(Error::InvalidValue),
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
    use std::collections::HashMap;

    use super::{Array, Error, List, Tag};
    use test_case::test_case;

    #[test_case(0, &[] => (Ok(Tag::End), 0); "End tag")]
    #[test_case(1, &[10] => (Ok(Tag::Byte(10)), 1); "Byte tag")]
    #[test_case(2, &[0, 10] => (Ok(Tag::Short(10)), 2); "Short tag")]
    #[test_case(3, &[0, 0, 0, 10] => (Ok(Tag::Int(10)), 4); "Int tag")]
    #[test_case(4, &[0, 0, 0, 0, 0, 0, 0, 10] => (Ok(Tag::Long(10)), 8); "Long tag")]
    #[test_case(5, (42.0f32).to_be_bytes().as_slice() => (Ok(Tag::Float(42.0)), 4); "Float tag")]
    #[test_case(6, (42.0f64).to_be_bytes().as_slice() => (Ok(Tag::Double(42.0)), 8); "Double tag")]
    #[test_case(7, &[0, 0, 0, 2, 1, 2] => (Ok(Tag::ByteArray(Array(vec![1, 2]))), 6); "Byte array tag")]
    #[test_case(8, &[0, 5, b'H', b'e', b'l', b'l', b'o'] => (Ok(Tag::String("Hello".to_owned())), 7); "String tag")]
    #[test_case(9, &[1, 0, 0, 0, 3, 1, 2, 3] => (Ok(Tag::List(List(vec![Tag::Byte(1), Tag::Byte(2), Tag::Byte(3)]))), 8); "List tag")]
    #[test_case(
        10, &[1, 0, 1, b'A', 32, 8, 0, 1, b'B', 0, 3, b'B', b'i', b't', 0] =>
        (Ok(Tag::Compound(HashMap::from_iter(vec![("A".to_owned(), Tag::Byte(32)), ("B".to_owned(), Tag::String("Bit".to_owned()))].into_iter()))), 15);
        "Map tag"
    )]
    #[test_case(11, &[0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 2] => (Ok(Tag::IntArray(Array(vec![1, 2]))), 12); "Int array tag")]
    #[test_case(12, &[0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2] => (Ok(Tag::LongArray(Array(vec![1, 2]))), 20); "Long array tag")]
    #[test_case(13, &[] => (Err(Error::UnknownTagId(13)), 0); "Unknown tag id")]
    fn test_new_tag(id: u8, data: &[u8]) -> (Result<Tag, Error>, usize) {
        let mut offset = 0;
        (Tag::new(id, data, &mut offset), offset)
    }

    #[test_case(Tag::End => 0; "End tag")]
    #[test_case(Tag::Byte(10) => 1; "Byte tag")]
    #[test_case(Tag::Short(10) => 2; "Short tag")]
    #[test_case(Tag::Int(10) => 3; "Int tag")]
    #[test_case(Tag::Long(10) => 4; "Long tag")]
    #[test_case(Tag::Float(10.0) => 5; "Float tag")]
    #[test_case(Tag::Double(10.0) => 6; "Double tag")]
    #[test_case(Tag::ByteArray(Array(vec![1, 2])) => 7; "Byte array tag")]
    #[test_case(Tag::String("Hello".to_owned()) => 8; "String tag")]
    #[test_case(Tag::List(List(vec![Tag::Byte(1), Tag::Byte(2), Tag::Byte(3)])) => 9; "List tag")]
    #[test_case(Tag::Compound(HashMap::from_iter(vec![("A".to_owned(), Tag::Byte(32)), ("B".to_owned(), Tag::String("Bit".to_owned()))].into_iter())) => 10; "Map tag")]
    #[test_case(Tag::IntArray(Array(vec![1, 2])) => 11; "Int array tag")]
    #[test_case(Tag::LongArray(Array(vec![1, 2])) => 12; "Long array tag")]
    fn test_get_id_from_tag(tag: Tag) -> u8 {
        tag.get_id()
    }

    #[test_case(Tag::List(List(vec![Tag::Byte(10), Tag::Byte(20), Tag::Byte(30)])) => Ok(List(vec![10, 20, 30])); "List of bytes")]
    #[test_case(Tag::Byte(10) => Err(Error::InvalidValue); "Not a list")]
    #[test_case(Tag::List(List(vec![Tag::Byte(10), Tag::Int(20), Tag::Byte(30)])) => Err(Error::InvalidValue); "Wrong data type")]
    fn test_try_into_list(list: Tag) -> Result<List<i8>, super::Error> {
        list.try_into()
    }

    #[test_case(
        Tag::Compound(HashMap::from_iter([("A".to_owned(), Tag::Byte(10)), ("B".to_owned(), Tag::Byte(20)), ("C".to_owned(), Tag::Byte(30))].into_iter())) =>
        Ok(HashMap::from_iter(vec![("A".to_string(), 10), ("B".to_string(), 20), ("C".to_string(), 30)].into_iter()));
        "Map of bytes"
    )]
    #[test_case(Tag::Byte(10) => Err(Error::InvalidValue); "Not a map")]
    #[test_case(
        Tag::Compound(HashMap::from_iter([("A".to_owned(), Tag::Byte(10)), ("B".to_owned(), Tag::Int(20)), ("C".to_owned(), Tag::Byte(30))].into_iter())) =>
        Err(Error::InvalidValue);
        "Mixed map"
    )]
    fn test_try_into_map(map: Tag) -> Result<HashMap<String, i8>, super::Error> {
        map.try_into()
    }

    #[test_case(Tag::Byte(1) => Ok(true); "Byte true")]
    #[test_case(Tag::Byte(0) => Ok(false); "Byte false")]
    #[test_case(Tag::Int(1) => Err(Error::InvalidValue); "Invalid")]
    fn test_try_to_bool(tag: Tag) -> Result<bool, super::Error> {
        tag.try_into()
    }

    #[test_case(vec![10] => List(vec![10]); "Single byte vector")]
    #[test_case(vec![1,2,3,4,5,6,7] => List(vec![1,2,3,4,5,6,7]); "Multi byte vector")]
    fn test_list_from_vec(vec: Vec<u8>) -> List<u8> {
        vec.into()
    }

    #[test]
    fn test_list_into_iter() {
        let list = List(vec![1, 2, 3, 4, 5, 6, 7]);
        let iter = list.into_iter();
        assert_eq!(iter.count(), 7);
    }

    #[test]
    fn test_list_from_iter() {
        let list: List<u8> = vec![1, 2, 3, 4, 5, 6, 7].into_iter().collect();
        assert_eq!(list, List(vec![1, 2, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn test_take_inner_of_list() {
        let list = List(vec![1, 2, 3, 4, 5, 6, 7]);
        let inner: Vec<u8> = list.take();
        assert_eq!(inner, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_list_iter() {
        let list = List(vec![1, 2, 3, 4, 5, 6, 7]);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_dref_array() {
        let array = Array(vec![1, 2, 3, 4, 5, 6, 7]);
        let inner = &*array;
        assert_eq!(inner, &vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_dref_list() {
        let list = List(vec![1, 2, 3, 4, 5, 6, 7]);
        let inner = &*list;
        assert_eq!(inner, &vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test_case(&[8] => Err(Error::InvalidValue); "Unexpected type")]
    #[test_case(&[10, 0, 0, 8, 0, 1, b'a', 0, 5, b'H', b'e', b'l', b'l', b'o', 1, 0, 1, b'b', 10, 0] => Ok(Tag::Compound(HashMap::from_iter([
        ("a".to_owned(), Tag::String("Hello".to_owned())),
        ("b".to_owned(), Tag::Byte(10))
    ]))); "Single byte array")]
    fn test_parse(data: &[u8]) -> Result<Tag, Error> {
        super::parse(data)
    }

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

    #[test_case(42.0, 0 => 42.0; "42")]
    #[test_case(0.815, 0 => 0.815; "815")]
    #[test_case(0.0, 0 => 0.0; "Single value array")]
    fn test_convert_f32(data: f32, mut offset: usize) -> f32 {
        let orig_offset = offset;
        let data = data.to_be_bytes();
        let result = super::convert_to_f32(data.as_slice(), &mut offset).unwrap();
        assert_eq!(offset, orig_offset + 4);
        result
    }

    #[test_case(42.0, 0 => 42.0; "42")]
    #[test_case(0.815, 0 => 0.815; "815")]
    #[test_case(0.0, 0 => 0.0; "Single value array")]
    fn test_convert_f64(data: f64, mut offset: usize) -> f64 {
        let orig_offset = offset;
        let data = data.to_be_bytes();
        let result = super::convert_to_f64(data.as_slice(), &mut offset).unwrap();
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
        let mut result = super::convert_to_map(data, &mut offset)
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        result.sort_by(|a, b| a.0.cmp(&b.0));
        result
    }
}
