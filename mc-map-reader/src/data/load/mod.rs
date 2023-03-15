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
    ($name:ident, $builder:ident => [$(
        $key:literal $(as $ty:ty)?: $setter:ident $(feature = $feature:literal)?,
    )*]) => {
        try_from_tag!(error $name => [$($($ty,)?)*]);
        try_from_tag!(other_impls $name);
        impl TryFrom<std::collections::HashMap<String, $crate::nbt::Tag>> for $name {
            type Error = paste::paste!([< $name Error >]);
            fn try_from(mut nbt_data: std::collections::HashMap<String, $crate::nbt::Tag>) -> Result<Self, Self::Error> {
                let mut builder = $builder::default();
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
        impl TryFrom<Tag> for $name {
            type Error = paste::paste! { [< $name Error >] };
            fn try_from(nbt_data: Tag) -> Result<Self, Self::Error> {
                let nbt_data = nbt_data.get_as_map()?;
                Self::try_from(nbt_data)
            }
        }
    };
    ($name:ident, $builder:ty => $build_fn:ident [$($type:ident,)*]) => {
        paste::paste! {
        impl TryFrom<HashMap<String, Tag>> for $name {
            type Error = [< $name Error >];
            fn try_from(nbt_data: HashMap<String, Tag>) -> Result<Self, Self::Error> {
                let mut builder = $builder::default();
                let r:Result<(), [< $name Error >]> = $build_fn(&mut builder, nbt_data);
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
    (error $name:ident => [$($error:ty,)*]) => {
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
    (other_impls $name: ident) => {
        paste::paste!{
        impl TryFrom<Tag> for $crate::nbt::List<$name> {
            type Error = [< $name Error >];
            fn try_from(tag: Tag) -> Result<Self, Self::Error> {
                let i = tag
                    .get_as_list()?
                    .take()
                    .into_iter()
                    .map($name::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(i.into())
            }
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
pub mod chunk;
pub mod entity;
//pub mod file_format;
