use crate::data::dimension::*;

try_from_tag!(Dimension => [
    "type": set_dimension_type,
    "generator": set_generator,
]);
