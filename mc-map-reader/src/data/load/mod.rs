macro_rules! mod_try_from_tag {
    ({
        $($name: ty:
            $(
                [$($(if feature = $feature:literal)? $key:literal => $setter:ident test($nbt_input_value:expr => $prop:ident = $test_value:expr),)*]
                $(? [$($(if feature = $error_feature:literal)? $data_type:ty,)*])?
            )?
            $(
                $build_fn:ident $(? [$($type:ident,)*])?
            )?
        ,)*
    }) => {
        $(
            try_from_tag!($name =>
                $(
                    [$(
                        $key: $setter $(feature = $feature)?
                    ,)*]
                    $(
                        ?[ $($(if feature = $error_feature)? $data_type,)* ]
                    )?
                )?
                $(
                    $build_fn $(? [$($type,)*])?
                )?
            );
        )*

        #[allow(non_snake_case)]
        #[cfg(test)]
        mod macro_tests {
            use super::*;
            paste::paste!{
            $($(
                #[test]
                fn [<test_ $name>]() {
                    let tag = crate::nbt::Tag::Compound(std::collections::HashMap::from_iter(
                        [$(
                            ($key.to_string(), $nbt_input_value.into())
                        ,)*]
                    ));
                    let actual = $name::try_from(tag);
                    let expected = $name {$(
                        $prop: $test_value,
                    )*};
                    assert_eq!(actual, Ok(expected));
                }
            )?)*
            }
        }
    };
}

macro_rules! try_from_tag {
    ($name:ty => [$(
        $key:literal: $setter:ident $(feature = $feature:literal)?,
    )*] $(? [ $($(if feature = $error_feature:literal)? $data_type:ty,)* ])? ) => {
        paste::paste!{
        try_from_tag!(error $name => [[< $name Builder >], $($($data_type $(=> feature = $error_feature)?,)*)?]);
        try_from_tag!(other_impls $name);
        }
        impl TryFrom<std::collections::HashMap<String, $crate::nbt::Tag>> for $name {
            type Error = paste::paste!([< $name Error >]);
            fn try_from(mut nbt_data: std::collections::HashMap<String, $crate::nbt::Tag>) -> Result<Self, Self::Error> {
                type Builder = paste::paste!([<$name Builder>]);
                let mut builder = Builder::default();
                add_data_to_builder!(builder, nbt_data => [
                    $(
                        $key: $setter $(feature = $feature)?,
                    )*
                ]);
                let b = builder
                    .try_build()?;
                Ok(b)
            }
        }
        impl TryFrom<$crate::nbt::Tag> for $name {
            type Error = paste::paste! { [< $name Error >] };
            fn try_from(nbt_data: $crate::nbt::Tag) -> Result<Self, Self::Error> {
                let nbt_data = nbt_data.get_as_map()?;
                Self::try_from(nbt_data)
            }
        }
    };
    ($name:ty => $build_fn:ident $(? [$($type:ident,)*])?) => {
        paste::paste! {
        impl TryFrom<HashMap<String, Tag>> for $name {
            type Error = [< $name Error >];
            fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
                type Builder = paste::paste!([<$name Builder>]);
                let mut builder = Builder::default();
                let result:Result<_, [< $name Error >]> = $build_fn(&mut builder, nbt_data);
                result?;
                let b = builder
                    .try_build()
                    .map_err([< $name Error >]::from)?;
                Ok(b)
            }
        }
        impl TryFrom<Tag> for $name {
            type Error = [< $name Error >];
            fn try_from(nbt_data: Tag) -> Result<Self, Self::Error> {
                let nbt_data = nbt_data.get_as_map()?;
                Self::try_from(nbt_data)
            }
        }
        }
        paste::paste!{
            try_from_tag!(error $name => [[< $name Builder >], $($($type,)*)?]);
            try_from_tag!(other_impls $name);
        }
    };
    (enum $name:ty => $build_fn:ident $(? [$($type:ident,)*])?) => {
        paste::paste! {
        impl TryFrom<HashMap<String, Tag>> for $name {
            type Error = [< $name Error >];
            fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
                let result = $build_fn(nbt_data)?;
                Ok(result)
            }
        }
        impl TryFrom<Tag> for $name {
            type Error = [< $name Error >];
            fn try_from(nbt_data: Tag) -> Result<Self, Self::Error> {
                let nbt_data = nbt_data.get_as_map()?;
                Self::try_from(nbt_data)
            }
        }
        }
        paste::paste!{
            try_from_tag!(error $name => [$($($type,)*)?]);
            try_from_tag!(other_impls $name);
        }
    };
    (error $name:ty => [$($error:ty $(=> feature = $feature:literal)?,)*]) => {
        paste::paste! {
            #[derive(Debug, thiserror::Error, PartialEq)]
            #[doc = "Error type for"]
            #[doc = stringify!($name)]
            pub enum [< $name Error >] {
                #[error(transparent)]
                /// An NBT error occurred
                Nbt(#[from] crate::nbt::Error),
                $(
                    $(#[cfg(feature = $feature)])?
                    /// An error occurred while parsing a field occurred
                    #[error(transparent)]
                    $error(#[from] [< $error Error >])
                ),*
            }
        }
    };
    (other_impls $name: ty) => {
        paste::paste!{
        impl $crate::nbt::NbtData for $name {
            type BuildError = [< $name Error >];
        }
        }
    }
}

macro_rules! add_data_to_builder {
    ($builder:ident, $nbt:ident => [$(
        $key:literal: $setter:ident $(feature = $feature:literal)?,
    )*]) => {
        $(
            $(#[cfg(feature = $feature)])?
            {
                if let Some(value) = $nbt.remove($key) {
                    $builder.$setter(value.try_into()?)
                }
            }
        )*
    };
}

#[cfg(feature = "block_entity")]
pub mod block_entity;
#[cfg(feature = "region_file")]
pub mod chunk;
pub mod dimension;
pub mod entity;
pub mod file_format;
pub mod item;
