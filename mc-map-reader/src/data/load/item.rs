use std::collections::HashMap;

use crate::{data::item::*, nbt::Tag};

try_from_tag!(Item => [
    "Count": set_count,
    "id": set_id,
    "tag": set_tag,
]);
try_from_tag!(ItemWithSlot => parse_item_with_slot ? [ Item, ]);
fn parse_item_with_slot(
    builder: &mut ItemWithSlotBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), ItemWithSlotError> {
    add_data_to_builder!(builder, nbt_data => [
        "Slot": set_slot,
    ]);
    builder.set_item(nbt_data.try_into()?);
    Ok(())
}