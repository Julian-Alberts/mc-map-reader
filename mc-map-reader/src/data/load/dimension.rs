use crate::data::dimension::*;

mod_try_from_tag!(Dimension: [
    "type" => set_dimension_type test(crate::nbt::Tag::String("test".to_string()) => dimension_type = "test".to_string()),
    "generator" => set_generator test(crate::nbt::Tag::Compound(std::collections::HashMap::new()) => generator = std::collections::HashMap::new(); DimensionBuilderError::UnsetGenerator),
],);
