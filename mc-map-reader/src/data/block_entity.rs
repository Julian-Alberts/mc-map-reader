use std::collections::HashMap;

use jbe::Builder;

use crate::{
    data::{
        entity::Entity,
        item::{Item, ItemWithSlot},
        load::block_entity::*,
    },
    nbt::{Array, List, Tag},
};

use super::{load::item::ItemWithSlotError, FieldError};

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct BlockEntity {
    pub id: String,
    #[builder({default: false})]
    pub keep_packed: bool,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub entity_type: BlockEntityType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockEntityType {
    Banner(Banner),
    Barrel(Barrel),
    Beacon(Beacon),
    Bed,
    Beehive(Beehive),
    Bell,
    BlastFurnace(BlastFurnace),
    BrewingStand(BrewingStand),
    Campfire(Campfire),
    ChiseledBookshelf(ChiseledBookshelf),
    Chest(Chest),
    Comparator(Comparator),
    CommandBlock(CommandBlock),
    Conduit(Conduit),
    DaylightDetector,
    Dispenser(Dispenser),
    Dropper(Dropper),
    EnchantingTable(EnchantingTable),
    EnderChest,
    EndGateway(EndGateway),
    EndPortal,
    Furnace(Furnace),
    Hopper(Hopper),
    Jigsaw(Jigsaw),
    Jukebox(Jukebox),
    Lectern(Lectern),
    MobSpawner(MobSpawner),
    Piston(Piston),
    ShulkerBox(ShulkerBox),
    Sign(Sign),
    Skull(Skull),
    Smoker(Smoker),
    SoulCampfire(SoulCampfire),
    StructureBlock(StructureBlock),
    TrappedChest(TrappedChest),
    Other(HashMap<String, Tag>),
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Banner {
    pub custom_name: Option<String>,
    pub patterns: Option<List<BannerPattern>>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct BannerPattern {
    pub color: i32,
    pub pattern: String,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Barrel {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Beacon {
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub levels: i32,
    pub primary: i32,
    pub secondary: i32,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Beehive {
    pub bees: Option<List<BeeInHive>>,
    pub flower_pos: Option<FlowerPos>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct BeeInHive {
    pub entity_data: Entity,
    pub min_occupation_ticks: i32,
    pub ticks_in_hive: i32,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct FlowerPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct BlastFurnace {
    pub burn_time: i16,
    pub cook_time: i16,
    pub cook_time_total: i16,
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct BrewingStand {
    pub brew_time: i16,
    pub custom_name: Option<String>,
    pub fuel: i8,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Campfire {
    pub cooking_times: Array<i32>,
    pub cooking_total_times: Array<i32>,
    pub items: Option<List<ItemWithSlot>>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct ChiseledBookshelf {
    pub items: Option<List<ItemWithSlot>>,
    pub last_interacted_slot: i32,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Chest {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Comparator {
    pub output_signal: i32,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct CommandBlock {
    pub auto: bool,
    pub command: String,
    pub condition_met: bool,
    pub custom_name: Option<String>,
    pub last_execution: i64,
    pub last_output: String,
    pub powered: bool,
    pub success_count: i32,
    pub track_output: bool,
    pub update_last_execution: bool,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Conduit {
    pub target: Array<i32>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Dispenser {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Dropper {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}
#[derive(Debug, Builder, Clone, PartialEq)]
pub struct EnchantingTable {
    pub custom_name: Option<String>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct EndGateway {
    pub age: i64,
    pub exact_teleport: bool,
    pub exit_portal: ExitPortal,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct ExitPortal {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Furnace {
    pub burn_time: i16,
    pub cook_time: i16,
    pub cook_time_total: i16,
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Hopper {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
    #[builder({default: 0})]
    pub transfer_cooldown: i32,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Jigsaw {
    pub final_state: String,
    pub joint: String,
    pub name: String,
    pub pool: String,
    pub target: String,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Jukebox {
    pub is_playing: bool,
    pub record_item: Item,
    pub record_start_tick: i64,
    pub tick_count: i64,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Lectern {
    pub book: Option<Item>,
    pub page: Option<i32>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct MobSpawner {
    pub spawner: Spawner,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Spawner {
    pub delay: i16,
    pub max_nearby_entities: i16,
    pub max_spawn_delay: i16,
    pub min_spawn_delay: i16,
    pub required_player_range: i16,
    pub spawn_count: i16,
    pub spawn_data: HashMap<String, Tag>,
    pub spawn_potentials: Option<List<PotentialSpawn>>,
    pub spawn_range: i16,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct PotentialSpawn {
    pub weight: i32,
    pub data: HashMap<String, Tag>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct CustomSpawnRules {
    pub block_light_limit: i32,
    pub sky_light_limit: i32,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Piston {
    pub block_state: PistonBlockState,
    pub extending: bool,
    pub facing: i32,
    pub progress: f32,
    pub source: bool,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct PistonBlockState {
    pub name: String,
    pub properties: HashMap<String, Tag>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct ShulkerBox {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Sign {
    pub glowing_text: bool,
    pub color: String,
    pub text1: String,
    pub text2: String,
    pub text3: String,
    pub text4: String,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Skull {
    pub note_block_sound: Option<String>,
    pub extra_type: Option<String>,
    pub skull_owner: Option<SkullOwner>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct SkullOwner {
    pub id: Array<i32>,
    pub name: Option<String>,
    pub properties: Option<List<SkullOwnerProperties>>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct SkullOwnerProperties {
    pub textures: Option<List<SkullOwnerTextures>>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct SkullOwnerTextures {
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Smoker {
    pub burn_time: i16,
    pub cook_time: i16,
    pub cook_time_total: i16,
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct SoulCampfire {
    pub cooking_times: Array<i32>,
    pub cooking_total_times: Array<i32>,
    pub items: Option<List<ItemWithSlot>>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct StructureBlock {
    pub author: String,
    pub ignore_entities: bool,
    pub integrity: f32,
    pub metadata: String,
    pub mirror: String,
    pub mode: String,
    pub name: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub powered: bool,
    pub rotation: String,
    pub seed: i64,
    pub show_bounding_box: bool,
    pub size_x: i32,
    pub size_y: i32,
    pub size_z: i32,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct TrappedChest {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

macro_rules! impl_IBE_for_builder {
    ($ty:ty, $res:ty) => {
        impl InventoryBlockEntityBuilder for $ty {
            type InventoryBlockError = paste::paste! { [< $res Error >] };
            type Target = $res;

            fn set_custom_name(&mut self, custom_name: String) {
                self.set_custom_name(custom_name)
            }

            fn set_items(&mut self, items: List<ItemWithSlot>) {
                self.set_items(items)
            }

            fn set_lock(&mut self, lock: String) {
                self.set_lock(lock)
            }

            fn set_loot_table(&mut self, loot_table: String) {
                self.set_loot_table(loot_table)
            }

            fn set_loot_table_seed(&mut self, loot_table_seed: i64) {
                self.set_loot_table_seed(loot_table_seed)
            }
            fn try_build(self) -> Result<Self::Target, Self::InventoryBlockError> {
                let res = self.try_build()?;
                Ok(res)
            }
        }

        impl InventoryBlock for $res {
            fn custom_name(&self) -> Option<&String> {
                self.custom_name.as_ref()
            }
            fn items(&self) -> Option<&List<ItemWithSlot>> {
                self.items.as_ref()
            }
            fn lock(&self) -> Option<&String> {
                self.lock.as_ref()
            }
            fn loot_table(&self) -> Option<&String> {
                self.loot_table.as_ref()
            }
            fn loot_table_seed(&self) -> Option<i64> {
                self.loot_table_seed
            }
        }
    };
}

macro_rules! impl_CBEB_for_builder {
    ($ty:ty, $res:ty) => {
        impl CookingBlockEntityBuilder for $ty {
            type CookingBlockError = paste::paste! { [< $res Error >] };
            type Target = $res;

            fn set_burn_time(&mut self, burn_time: i16) {
                self.set_burn_time(burn_time)
            }

            fn set_cook_time(&mut self, cook_time: i16) {
                self.set_cook_time(cook_time)
            }

            fn set_cook_time_total(&mut self, cook_time_total: i16) {
                self.set_cook_time_total(cook_time_total)
            }

            fn set_custom_name(&mut self, custom_name: String) {
                self.set_custom_name(custom_name)
            }

            fn set_items(&mut self, items: List<ItemWithSlot>) {
                self.set_items(items)
            }

            fn set_lock(&mut self, lock: String) {
                self.set_lock(lock)
            }

            fn set_recipes_used(&mut self, recipes_used: HashMap<String, i32>) {
                self.set_recipes_used(recipes_used)
            }

            fn try_build(self) -> Result<Self::Target, Self::CookingBlockError> {
                let res = self.try_build()?;
                Ok(res)
            }
        }

        impl CookingBlockEntity for $res {
            fn burn_time(&self) -> i16 {
                self.burn_time
            }
            fn cook_time(&self) -> i16 {
                self.cook_time
            }
            fn cook_time_total(&self) -> i16 {
                self.cook_time_total
            }
            fn custom_name(&self) -> Option<&String> {
                self.custom_name.as_ref()
            }
            fn items(&self) -> Option<&List<ItemWithSlot>> {
                self.items.as_ref()
            }
            fn lock(&self) -> Option<&String> {
                self.lock.as_ref()
            }
            fn recipes_used(&self) -> &HashMap<String, i32> {
                &self.recipes_used
            }
        }
    };
}

impl_IBE_for_builder!(BarrelBuilder, Barrel);
impl_IBE_for_builder!(ChestBuilder, Chest);
impl_IBE_for_builder!(DispenserBuilder, Dispenser);
impl_IBE_for_builder!(DropperBuilder, Dropper);
impl_IBE_for_builder!(HopperBuilder, Hopper);
impl_IBE_for_builder!(ShulkerBoxBuilder, ShulkerBox);
impl_IBE_for_builder!(TrappedChestBuilder, TrappedChest);
impl_CBEB_for_builder!(BlastFurnaceBuilder, BlastFurnace);
impl_CBEB_for_builder!(FurnaceBuilder, Furnace);
impl_CBEB_for_builder!(SmokerBuilder, Smoker);

pub trait InventoryBlock {
    fn custom_name(&self) -> Option<&String>;
    fn items(&self) -> Option<&List<ItemWithSlot>>;
    fn lock(&self) -> Option<&String>;
    fn loot_table(&self) -> Option<&String>;
    fn loot_table_seed(&self) -> Option<i64>;
}
pub trait InventoryBlockEntityBuilder
where
    Self::InventoryBlockError:
        From<FieldError<crate::nbt::Error>> + From<FieldError<ItemWithSlotError>>,
{
    type InventoryBlockError;
    type Target;
    fn set_custom_name(&mut self, custom_name: String);
    fn set_items(&mut self, items: List<ItemWithSlot>);
    fn set_lock(&mut self, lock: String);
    fn set_loot_table(&mut self, loot_table: String);
    fn set_loot_table_seed(&mut self, loot_table_seed: i64);
    fn try_build(self) -> Result<Self::Target, Self::InventoryBlockError>;
}

pub trait CookingBlockEntity {
    fn burn_time(&self) -> i16;
    fn cook_time(&self) -> i16;
    fn cook_time_total(&self) -> i16;
    fn custom_name(&self) -> Option<&String>;
    fn items(&self) -> Option<&List<ItemWithSlot>>;
    fn lock(&self) -> Option<&String>;
    fn recipes_used(&self) -> &HashMap<String, i32>;
}
pub trait CookingBlockEntityBuilder
where
    Self::CookingBlockError:
        From<FieldError<crate::nbt::Error>> + From<FieldError<ItemWithSlotError>>,
    Self::Target: CookingBlockEntity,
{
    type CookingBlockError;
    type Target;
    fn set_burn_time(&mut self, burn_time: i16);
    fn set_cook_time(&mut self, cook_time: i16);
    fn set_cook_time_total(&mut self, cook_time_total: i16);
    fn set_custom_name(&mut self, custom_name: String);
    fn set_items(&mut self, items: List<ItemWithSlot>);
    fn set_lock(&mut self, lock: String);
    fn set_recipes_used(&mut self, recipes_used: HashMap<String, i32>);
    fn try_build(self) -> Result<Self::Target, Self::CookingBlockError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_inventory_block_entity<B>(builder: &mut B)
    where
        B: InventoryBlockEntityBuilder,
    {
        builder.set_custom_name("test".to_string());
        builder.set_items(List::from(vec![]));
        builder.set_lock("test".to_string());
        builder.set_loot_table("test".to_string());
        builder.set_loot_table_seed(1);
    }

    fn assert_inventory_block_entity(block: &dyn InventoryBlock) {
        assert_eq!(block.custom_name(), Some(&"test".to_string()));
        assert_eq!(block.items(), Some(&List::from(vec![])));
        assert_eq!(block.lock(), Some(&"test".to_string()));
        assert_eq!(block.loot_table(), Some(&"test".to_string()));
        assert_eq!(block.loot_table_seed(), Some(1));
    }

    fn test_cooking_block_entity<B>(builder: &mut B)
    where
        B: CookingBlockEntityBuilder,
    {
        builder.set_burn_time(1);
        builder.set_cook_time(1);
        builder.set_cook_time_total(1);
        builder.set_custom_name("test".to_string());
        builder.set_items(List::from(vec![]));
        builder.set_lock("test".to_string());
        builder.set_recipes_used(HashMap::new());
    }

    fn assert_cooking_block_entity(block: &dyn CookingBlockEntity) {
        assert_eq!(block.burn_time(), 1);
        assert_eq!(block.cook_time(), 1);
        assert_eq!(block.cook_time_total(), 1);
        assert_eq!(block.custom_name(), Some(&"test".to_string()));
        assert_eq!(block.items(), Some(&List::from(vec![])));
        assert_eq!(block.lock(), Some(&"test".to_string()));
        assert_eq!(block.recipes_used(), &HashMap::new());
    }

    #[test]
    fn test_barrel() {
        let mut builder = BarrelBuilder::default();
        test_inventory_block_entity(&mut builder);
        let barrel =
            InventoryBlockEntityBuilder::try_build(builder).expect("Error building barrel");
        assert_inventory_block_entity(&barrel);
    }

    #[test]
    fn test_chest() {
        let mut builder = ChestBuilder::default();
        test_inventory_block_entity(&mut builder);
        let chest = InventoryBlockEntityBuilder::try_build(builder).expect("Error building chest");
        assert_inventory_block_entity(&chest);
    }

    #[test]
    fn test_dispenser() {
        let mut builder = DispenserBuilder::default();
        test_inventory_block_entity(&mut builder);
        let dispenser =
            InventoryBlockEntityBuilder::try_build(builder).expect("Error building dispenser");
        assert_inventory_block_entity(&dispenser);
    }

    #[test]
    fn test_dropper() {
        let mut builder = DropperBuilder::default();
        test_inventory_block_entity(&mut builder);
        let dropper =
            InventoryBlockEntityBuilder::try_build(builder).expect("Error building dropper");
        assert_inventory_block_entity(&dropper);
    }

    #[test]
    fn test_hopper() {
        let mut builder = HopperBuilder::default();
        test_inventory_block_entity(&mut builder);
        let hopper =
            InventoryBlockEntityBuilder::try_build(builder).expect("Error building hopper");
        assert_inventory_block_entity(&hopper);
    }

    #[test]
    fn test_shulker_box() {
        let mut builder = ShulkerBoxBuilder::default();
        test_inventory_block_entity(&mut builder);
        let shulker_box =
            InventoryBlockEntityBuilder::try_build(builder).expect("Error building shulker box");
        assert_inventory_block_entity(&shulker_box);
    }

    #[test]
    fn test_trapped_chest() {
        let mut builder = TrappedChestBuilder::default();
        test_inventory_block_entity(&mut builder);
        let trapped_chest =
            InventoryBlockEntityBuilder::try_build(builder).expect("Error building trapped chest");
        assert_inventory_block_entity(&trapped_chest);
    }

    #[test]
    fn test_blast_furnace() {
        let mut builder = BlastFurnaceBuilder::default();
        test_cooking_block_entity(&mut builder);
        let blast_furnace =
            CookingBlockEntityBuilder::try_build(builder).expect("Error building blast furnace");
        assert_cooking_block_entity(&blast_furnace);
    }

    #[test]
    fn test_furnace() {
        let mut builder = FurnaceBuilder::default();
        test_cooking_block_entity(&mut builder);
        let furnace =
            CookingBlockEntityBuilder::try_build(builder).expect("Error building furnace");
        assert_cooking_block_entity(&furnace);
    }

    #[test]
    fn test_smoker() {
        let mut builder = SmokerBuilder::default();
        test_cooking_block_entity(&mut builder);
        let smoker = CookingBlockEntityBuilder::try_build(builder).expect("Error building smoker");
        assert_cooking_block_entity(&smoker);
    }
}
