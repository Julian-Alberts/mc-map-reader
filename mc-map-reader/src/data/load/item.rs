use std::collections::HashMap;

use crate::{data::{item::*, FieldError}, nbt::Tag};

mod_try_from_tag!({
    Item: [
        "Count" => set_count test(10_i8 => count = 10),
        "id" => set_id test("test_id".to_string() => id = "test_id".to_string()),
        "tag" => set_tag test(HashMap::new() => tag = Some(HashMap::new())),
    ],
    ItemWithSlot: parse_item_with_slot ? [ Item, ],
});
fn parse_item_with_slot(
    builder: &mut ItemWithSlotBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), ItemWithSlotError> {
    add_data_to_builder!(builder, nbt_data => [
        "Slot": set_slot,
    ]);
    builder.set_item(nbt_data.try_into().map_err(|e| FieldError::new("<internal> item",e))?);
    Ok(())
}
