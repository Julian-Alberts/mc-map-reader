macro_rules! add_data_to_builder {
    ($builder:ident, $nbt:ident => [$(
        $key:literal: $setter:ident
    ),*]) => {
        $(
            if let Some(value) = $nbt.remove($key) {
                $builder.$setter(value.try_into()?)
            }
        )*
    };
}

pub mod block_entity;
pub mod chunk;
pub mod entity;
