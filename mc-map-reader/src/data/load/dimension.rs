use crate::data::dimension::*;

try_from_tag!(Dimension => [
    "ultrawarm": set_ultrawarm,
    "natural": set_natural,
    "coordinate_scale": set_coordinate_scale,
    "has_skylight": set_has_skylight,
    "has_ceiling": set_has_ceiling,
    "ambient_light": set_ambient_light,
    "fixed_time": set_fixed_time,
    "monster_spawn_block_light_limit": set_monster_spawn_block_light_limit,
    "piglin_safe": set_piglin_safe,
    "bed_works": set_bed_works,
    "respawn_anchor_works": set_respawn_anchor_works,
    "has_raids": set_has_raids,
    "logical_height": set_logical_height,
    "min_y": set_min_y,
    "height": set_height,
    "infiniburn": set_infiniburn,
    "effects": set_effects,
]);
