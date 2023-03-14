macro_rules! try_from_tag_for_module {
    ($({$name:ident => $([$(
        $key:literal: $setter:ident
    ),*])? $($build_fn:ident)? }),*) => {
        $(paste::paste! {
            try_from_tag!($name, [< $name Builder >], Error => $([$(
                $key: $setter
            ),*])? $($build_fn)?);
        })*

        #[derive(Debug, thiserror::Error, PartialEq, Eq)]
        pub enum Error {
            $(
                #[error(transparent)]
                $name(#[from] paste::paste! { [< $name BuilderError >]}),
            )*
        }
    };
}

macro_rules! try_from_tag {
    ($name:ident, $builder:ident, $error:ty => [$(
        $key:literal: $setter:ident
    ),*]) => {
        try_from_tag!(from_tag $name);
        impl TryFrom<std::collections::HashMap<String, $crate::nbt::Tag>> for $name {
            type Error = $error;
            fn try_from(mut nbt_data: std::collections::HashMap<String, $crate::nbt::Tag>) -> Result<Self, Self::Error> {
                let mut builder = $builder::default();
                add_data_to_builder!(builder, nbt_data => [
                    $(
                        $key: $setter
                    ),*
                ]);
                let b = builder
                    .try_build()
                    .map_err($error::from)
                    .map_err(MissingData::from)?;
                Ok(b)
            }
        }
    };
    ($name:ident, $builder:ident, $error:ty => $build_fn:ident) => {
        try_from_tag!(from_tag $name);
        impl TryFrom<HashMap<String, Tag>> for $name {
            type Error = $error;
            fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
                let mut builder = $builder::default();
                $build_fn(&mut builder, nbt_data)?;
                let b = builder
                    .try_build()
                    .map_err($error::from)
                    .map_err(MissingData::from)?;
                Ok(b)
            }
        }
    };
    (from_tag $name:ident) => {
        impl TryFrom<Tag> for $name {
            type Error = crate::nbt::Error;
            fn try_from(nbt_data: Tag) -> Result<Self, Self::Error> {
                let nbt_data = nbt_data.get_as_map()?;
                Self::try_from(nbt_data)
            }
        }
    };
}

macro_rules! add_data_to_builder {
    ($builder:ident, $nbt:ident => [$(
        $key:literal: $setter:ident $(feature = $feature:literal)?
    ),*]) => {
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
pub mod chunk;
pub mod entity;
pub mod file_format;
