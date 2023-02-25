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
        #[derive(Debug)]
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
                    other => return Err(Error::UnknownTagId(other))
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
            pub fn $getter(self) -> Result<$ty, Error> {
                if let Self::$tag_type(v) = self {
                    Ok(v)
                } else {
                    Err(Error::InvalidValue)
                }
            }
            )?)*
        }

        $($(
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

impl <T> TryFrom<HashMap<String, Tag>> for HashMap<String, T> 
    where T: TryFrom<Tag, Error = crate::nbt::Error>
{
    type Error = crate::nbt::Error;
    fn try_from(map: HashMap<String, Tag>) -> Result<Self, Self::Error> {
        
    }
}

impl<T> TryFrom<Tag> for List<T>
where
    T: TryFrom<Tag, Error = Error>,
{
    type Error = Error;
    fn try_from(tag: Tag) -> Result<Self, Self::Error> {
        let i = tag
            .get_as_list()?
            .take()
            .into_iter()
            .map(T::try_from)
            .collect::<Result<_, _>>()?;
        Ok(List(i))
    }
}

impl<T> IntoIterator for List<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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

#[derive(Debug)]
pub struct Array<T>(Vec<T>);
#[derive(Debug)]
pub struct List<T>(Vec<T>);

pub struct Compound<T>(HashMap<String, T>);

impl<T> List<T> {
    pub fn take(self) -> Vec<T> {
        self.0
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown Tag ID: {0}")]
    UnknownTagId(u8),
    #[error("Invalid Value")]
    InvalidValue,
    #[error(transparent)]
    MissingData(#[from] crate::nbt_data::chunk::MissingData),
    #[error(transparent)]
    ChunkStatus(#[from] crate::nbt_data::chunk::ChunkStatusError),
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

pub fn parse(data: &[u8]) -> Result<Tag, Error> {
    match data[0] {
        10 => Tag::new(10, data, &mut 3),
        _ => panic!(),
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
    let str_data = data[*offset..len + *offset].iter().map(|i| *i).collect();
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
