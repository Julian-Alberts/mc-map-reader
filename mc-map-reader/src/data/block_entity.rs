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

use super::load::item::ItemWithSlotError;

#[derive(Debug, Builder, Clone)]
pub struct BlockEntity {
    pub id: String,
    #[builder({default false})]
    pub keep_packed: bool,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub entity_type: BlockEntityType,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Builder, Clone)]
pub struct Banner {
    pub custom_name: Option<String>,
    pub patterns: Option<List<BannerPattern>>,
}

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct BannerPattern {
    pub color: i32,
    pub pattern: String,
}

#[derive(Debug, Builder, Clone)]
pub struct Barrel {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Clone)]
pub struct Beacon {
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub levels: i32,
    pub primary: i32,
    pub secondary: i32,
}

#[derive(Debug, Builder, Clone)]
pub struct Beehive {
    pub bees: Option<List<BeeInHive>>,
    pub flower_pos: Option<FlowerPos>,
}

#[derive(Debug, Builder, Clone)]
pub struct BeeInHive {
    pub entity_data: Entity,
    pub min_occupation_ticks: i32,
    pub ticks_in_hive: i32,
}

#[derive(Debug, Builder, Clone)]
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

#[derive(Debug, Builder, Clone)]
pub struct BrewingStand {
    pub brew_time: i16,
    pub custom_name: Option<String>,
    pub fuel: i8,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
}

#[derive(Debug, Builder, Clone)]
pub struct Campfire {
    pub cooking_times: Array<i32>,
    pub cooking_total_times: Array<i32>,
    pub items: Option<List<ItemWithSlot>>,
}

#[derive(Debug, Builder, Clone)]
pub struct ChiseledBookshelf {
    pub items: Option<List<ItemWithSlot>>,
    pub last_interacted_slot: i32,
}

#[derive(Debug, Builder, Clone)]
pub struct Chest {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Clone)]
pub struct Comparator {
    pub output_signal: i32,
}

#[derive(Debug, Builder, Clone)]
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

#[derive(Debug, Builder, Clone)]
pub struct Conduit {
    pub target: Array<i32>,
}

#[derive(Debug, Builder, Clone)]
pub struct Dispenser {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Clone)]
pub struct Dropper {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}
#[derive(Debug, Builder, Clone)]
pub struct EnchantingTable {
    pub custom_name: Option<String>,
}

#[derive(Debug, Builder, Clone)]
pub struct EndGateway {
    pub age: i64,
    pub exact_teleport: bool,
    pub exit_portal: ExitPortal,
}

#[derive(Debug, Builder, Clone)]
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

#[derive(Debug, Builder, Clone)]
pub struct Hopper {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
    #[builder({default 0})]
    pub transfer_cooldown: i32,
}

#[derive(Debug, Builder, Clone)]
pub struct Jigsaw {
    pub final_state: String,
    pub joint: String,
    pub name: String,
    pub pool: String,
    pub target: String,
}

#[derive(Debug, Builder, Clone)]
pub struct Jukebox {
    pub is_playing: bool,
    pub record_item: Item,
    pub record_start_tick: i64,
    pub tick_count: i64,
}

#[derive(Debug, Builder, Clone)]
pub struct Lectern {
    pub book: Option<Item>,
    pub page: Option<i32>,
}

#[derive(Debug, Builder, Clone)]
pub struct MobSpawner {
    pub spawner: Spawner,
}

#[derive(Debug, Builder, Clone)]
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

#[derive(Debug, Builder, Clone)]
pub struct PotentialSpawn {
    pub weight: i32,
    pub data: HashMap<String, Tag>,
    pub custom_spawn_rules: Option<CustomSpawnRules>,
}

#[derive(Debug, Builder, Clone)]
pub struct CustomSpawnRules {
    pub block_light_limit: i32,
    pub sky_light_limit: i32,
}

#[derive(Debug, Builder, Clone)]
pub struct Piston {
    pub block_state: PistonBlockState,
    pub extending: bool,
    pub facing: i32,
    pub progress: f32,
    pub source: bool,
}

#[derive(Debug, Builder, Clone)]
pub struct PistonBlockState {
    pub name: String,
    pub properties: HashMap<String, Tag>,
}

#[derive(Debug, Builder, Clone)]
pub struct ShulkerBox {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Clone)]
pub struct Sign {
    pub glowing_text: bool,
    pub color: String,
    pub text1: String,
    pub text2: String,
    pub text3: String,
    pub text4: String,
}

#[derive(Debug, Builder, Clone)]
pub struct Skull {
    pub note_block_sound: Option<String>,
    pub extra_type: Option<String>,
    pub skull_owner: Option<SkullOwner>,
}

#[derive(Debug, Builder, Clone)]
pub struct SkullOwner {
    pub id: Array<i32>,
    pub name: Option<String>,
    pub properties: Option<List<SkullOwnerProperties>>,
}

#[derive(Debug, Builder, Clone)]
pub struct SkullOwnerProperties {
    pub textures: Option<List<SkullOwnerTextures>>,
}

#[derive(Debug, Builder, Clone)]
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

#[derive(Debug, Builder, Clone)]
pub struct SoulCampfire {
    pub cooking_times: Array<i32>,
    pub cooking_total_times: Array<i32>,
    pub items: Option<List<ItemWithSlot>>,
}

#[derive(Debug, Builder, Clone)]
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

#[derive(Debug, Builder, Clone)]
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
            type Target = paste::paste! {[< $res >]};

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
pub trait InventoryBlockEntityBuilder {
    fn set_custom_name(&mut self, custom_name: String);
    fn set_items(&mut self, items: List<ItemWithSlot>);
    fn set_lock(&mut self, lock: String);
    fn set_loot_table(&mut self, loot_table: String);
    fn set_loot_table_seed(&mut self, loot_table_seed: i64);
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
    Self::CookingBlockError: From<crate::nbt::Error> + From<ItemWithSlotError>,
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
