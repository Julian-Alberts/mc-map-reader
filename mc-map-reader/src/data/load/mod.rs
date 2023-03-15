macro_rules! try_from_tag_for_module {
    ($({$name:ident => $([$(
        $key:literal $(as $ty:ty)?: $setter:ident,
    )*])? $($build_fn:ident)? }),*) => {
        $(paste::paste! {
            try_from_tag!($name, [< $name Builder >], Error => $([$(
                $key $(as $ty)?: $setter,
            )*])? $($build_fn)?);
        })*
    };
}

macro_rules! try_from_tag {
    ($name:ty, $builder:ty => [$(
        $key:literal $(as $ty:ty)?: $setter:ident $(feature = $feature:literal)?,
    )*]) => {
        try_from_tag!(error $name => [$($($ty,)?)*]);
        try_from_tag!(other_impls $name);
        impl TryFrom<std::collections::HashMap<String, $crate::nbt::Tag>> for $name {
            type Error = paste::paste!([< $name Error >]);
            fn try_from(mut nbt_data: std::collections::HashMap<String, $crate::nbt::Tag>) -> Result<Self, Self::Error> {
                type Builder = $builder;
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
    ($name:ty, $builder:ty => $build_fn:ident [$($type:ident,)*]) => {
        paste::paste! {
        impl TryFrom<HashMap<String, Tag>> for $name {
            type Error = [< $name Error >];
            fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
                let mut builder = $builder::default();
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
        try_from_tag!(error $name => [$($type,)*]);
        try_from_tag!(other_impls $name);
    };
    (error $name:ty => [$($error:ty,)*]) => {
        paste::paste! {
            #[derive(Debug, thiserror::Error)]
            pub enum [< $name Error >] {
                #[error(transparent)]
                Nbt(#[from] crate::nbt::Error),
                #[error(transparent)]
                [< $name Builder >](#[from] [< $name BuilderError >]),
                $(
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
        /*paste::paste!{
        impl TryFrom<$crate::nbt::Tag> for $crate::nbt::List<$name> {
            type Error = [< $name Error >];
            fn try_from(tag: $crate::nbt::Tag) -> Result<Self, Self::Error> {
                let i = tag
                    .get_as_list()?
                    .take()
                    .into_iter()
                    .map($name::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(i.into())
            }
        }

        impl TryFrom<$crate::nbt::Tag> for std::collections::HashMap<String, $name> {
            type Error = [< $name Error >];
            fn try_from(tag: $crate::nbt::Tag) -> Result<Self, Self::Error> {
                tag.get_as_map()?.into_iter().map(|(k,v)| (k, v.try_into())).collect::<Result<std::collections::HashMap<_,_>,_>>()
            }
        }
        }*/
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
pub mod chunk;
mod dimension;
pub mod entity;
pub mod file_format;
