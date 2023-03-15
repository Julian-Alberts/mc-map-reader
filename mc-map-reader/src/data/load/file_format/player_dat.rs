use std::collections::HashMap;

use crate::{data::file_format::player_dat::*, nbt::Tag};
use crate::data::load::entity::MobError;

try_from_tag![
    Player => parse_player ? [
        Mob,
    ]
];

fn parse_player(
    builder: &mut PlayerBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), PlayerError> {
    builder.set_mob(nbt_data.try_into()?);
    Ok(())
}
