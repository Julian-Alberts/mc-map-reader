use std::collections::HashMap;

use crate::{data::file_format::player_dat::*, nbt::Tag};

try_from_tag_for_module![{
    Player => parse_player
}];

fn parse_player(
    builder: &mut PlayerBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), crate::nbt::Error> {
    Ok(())
}
