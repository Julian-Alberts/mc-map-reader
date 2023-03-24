use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::{
    data::{entity::Entity, item::{Item, ItemWithSlot}, load::block_entity::*},
    nbt::{Array, List, Tag},
};

use super::load::item::ItemWithSlotError;

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct BlockEntity {
    #[get = "pub"]
    pub id: String,
    #[get_copy = "pub"]
    #[builder({default false})]
    pub keep_packed: bool,
    #[get_copy = "pub"]
    pub x: i32,
    #[get_copy = "pub"]
    pub y: i32,
    #[get_copy = "pub"]
    pub z: i32,
    #[get = "pub"]
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

#[derive(Debug, Builder, Getters, Clone)]
pub struct Banner {
    pub custom_name: Option<String>,
    pub patterns: Option<List<BannerPattern>>,
}


#[derive(Debug, Builder, Getters, CopyGetters, Clone, PartialEq)]
pub struct BannerPattern {
    #[get_copy = "pub"]
    pub color: i32,
    #[get = "pub"]
    pub pattern: String,
}

#[derive(Debug, Builder, Getters, Clone)]
pub struct Barrel {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Beacon {
    #[get = "pub"]
    pub custom_name: Option<String>,
    #[get = "pub"]
    pub lock: Option<String>,
    #[get_copy = "pub"]
    pub levels: i32,
    #[get_copy = "pub"]
    pub primary: i32,
    #[get_copy = "pub"]
    pub secondary: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Beehive {
    pub bees: Option<List<BeeInHive>>,
    pub flower_pos: Option<FlowerPos>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct BeeInHive {
    #[get = "pub"]
    pub entity_data: Entity,
    #[get_copy = "pub"]
    pub min_occupation_ticks: i32,
    #[get_copy = "pub"]
    pub ticks_in_hive: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
#[get_copy = "pub"]
pub struct FlowerPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone, PartialEq)]
pub struct BlastFurnace {
    #[get_copy = "pub"]
    pub burn_time: i16,
    #[get_copy = "pub"]
    pub cook_time: i16,
    #[get_copy = "pub"]
    pub cook_time_total: i16,
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    #[get = "pub"]
    pub recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct BrewingStand {
    #[get_copy = "pub"]
    pub brew_time: i16,
    pub custom_name: Option<String>,
    #[get_copy = "pub"]
    pub fuel: i8,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Campfire {
    #[get = "pub"]
    pub cooking_times: Array<i32>,
    #[get = "pub"]
    pub cooking_total_times: Array<i32>,
    pub items: Option<List<ItemWithSlot>>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct ChiseledBookshelf {
    pub items: Option<List<ItemWithSlot>>,
    #[get_copy = "pub"]
    pub last_interacted_slot: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Chest {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    #[get_copy = "pub"]
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Comparator {
    #[get_copy = "pub"]
    pub output_signal: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct CommandBlock {
    #[get_copy = "pub"]
    pub auto: bool,
    #[get = "pub"]
    pub command: String,
    #[get_copy = "pub"]
    pub condition_met: bool,
    pub custom_name: Option<String>,
    #[get_copy = "pub"]
    pub last_execution: i64,
    #[get = "pub"]
    pub last_output: String,
    #[get_copy = "pub"]
    pub powered: bool,
    #[get_copy = "pub"]
    pub success_count: i32,
    #[get_copy = "pub"]
    pub track_output: bool,
    #[get_copy = "pub"]
    pub update_last_execution: bool,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Conduit {
    #[get = "pub"]
    pub target: Array<i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Dispenser {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    #[get_copy = "pub"]
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Dropper {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    #[get_copy = "pub"]
    pub loot_table_seed: Option<i64>,
}
#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct EnchantingTable {
    pub custom_name: Option<String>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct EndGateway {
    #[get_copy = "pub"]
    pub age: i64,
    #[get_copy = "pub"]
    pub exact_teleport: bool,
    #[get = "pub"]
    pub exit_portal: ExitPortal,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct ExitPortal {
    #[get_copy = "pub"]
    pub x: i32,
    #[get_copy = "pub"]
    pub y: i32,
    #[get_copy = "pub"]
    pub z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone, PartialEq)]
pub struct Furnace {
    #[get_copy = "pub"]
    pub burn_time: i16,
    #[get_copy = "pub"]
    pub cook_time: i16,
    #[get_copy = "pub"]
    pub cook_time_total: i16,
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    #[get = "pub"]
    pub recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Hopper {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    #[get_copy = "pub"]
    pub loot_table_seed: Option<i64>,
    #[get_copy = "pub"]
    #[builder({default 0})]
    pub transfer_cooldown: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Jigsaw {
    #[get = "pub"]
    pub final_state: String,
    #[get = "pub"]
    pub joint: String,
    #[get = "pub"]
    pub name: String,
    #[get = "pub"]
    pub pool: String,
    #[get = "pub"]
    pub target: String,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Jukebox {
    #[get_copy = "pub"]
    pub is_playing: bool,
    #[get = "pub"]
    pub record_item: Item,
    #[get_copy = "pub"]
    pub record_start_tick: i64,
    #[get_copy = "pub"]
    pub tick_count: i64,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Lectern {
    pub book: Option<Item>,
    pub page: Option<i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct MobSpawner {
    #[get = "pub"]
    pub spawner: Spawner,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Spawner {
    #[get_copy = "pub"]
    pub delay: i16,
    #[get_copy = "pub"]
    pub max_nearby_entities: i16,
    #[get_copy = "pub"]
    pub max_spawn_delay: i16,
    #[get_copy = "pub"]
    pub min_spawn_delay: i16,
    #[get_copy = "pub"]
    pub required_player_range: i16,
    #[get_copy = "pub"]
    pub spawn_count: i16,
    #[get = "pub"]
    pub spawn_data: HashMap<String, Tag>,
    pub spawn_potentials: Option<List<PotentialSpawn>>,
    #[get_copy = "pub"]
    pub spawn_range: i16,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct PotentialSpawn {
    #[get_copy = "pub"]
    pub weight: i32,
    #[get = "pub"]
    pub data: HashMap<String, Tag>,
    pub custom_spawn_rules: Option<CustomSpawnRules>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct CustomSpawnRules {
    #[get_copy = "pub"]
    pub block_light_limit: i32,
    #[get_copy = "pub"]
    pub sky_light_limit: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Piston {
    #[get = "pub"]
    pub block_state: PistonBlockState,
    #[get_copy = "pub"]
    pub extending: bool,
    #[get_copy = "pub"]
    pub facing: i32,
    #[get_copy = "pub"]
    pub progress: f32,
    #[get_copy = "pub"]
    pub source: bool,
}

#[derive(Debug, Builder, Getters, Clone)]
#[get = "pub"]
pub struct PistonBlockState {
    pub name: String,
    pub properties: HashMap<String, Tag>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct ShulkerBox {
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    pub loot_table: Option<String>,
    #[get_copy = "pub"]
    pub loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Sign {
    #[get_copy = "pub"]
    glowing_text: bool,
    #[get = "pub"]
    color: String,
    #[get = "pub"]
    text1: String,
    #[get = "pub"]
    text2: String,
    #[get = "pub"]
    text3: String,
    #[get = "pub"]
    text4: String,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Skull {
    note_block_sound: Option<String>,
    extra_type: Option<String>,
    skull_owner: Option<SkullOwner>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct SkullOwner {
    #[get = "pub"]
    id: Array<i32>,
    name: Option<String>,
    #[get = "pub"]
    properties: Option<List<SkullOwnerProperties>>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct SkullOwnerProperties {
    #[get = "pub"]
    textures: Option<List<SkullOwnerTextures>>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct SkullOwnerTextures {
    #[get = "pub"]
    value: String,
    signature: Option<String>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone, PartialEq)]
pub struct Smoker {
    #[get_copy = "pub"]
    pub burn_time: i16,
    #[get_copy = "pub"]
    pub cook_time: i16,
    #[get_copy = "pub"]
    pub cook_time_total: i16,
    pub custom_name: Option<String>,
    pub items: Option<List<ItemWithSlot>>,
    pub lock: Option<String>,
    #[get = "pub"]
    pub recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct SoulCampfire {
    #[get = "pub"]
    cooking_times: Array<i32>,
    #[get = "pub"]
    cooking_total_times: Array<i32>,
    items: Option<List<ItemWithSlot>>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct StructureBlock {
    #[get = "pub"]
    author: String,
    #[get_copy = "pub"]
    ignore_entities: bool,
    #[get_copy = "pub"]
    integrity: f32,
    #[get = "pub"]
    metadata: String,
    #[get = "pub"]
    mirror: String,
    #[get = "pub"]
    mode: String,
    #[get = "pub"]
    name: String,
    #[get_copy = "pub"]
    pos_x: i32,
    #[get_copy = "pub"]
    pos_y: i32,
    #[get_copy = "pub"]
    pos_z: i32,
    #[get_copy = "pub"]
    powered: bool,
    #[get = "pub"]
    rotation: String,
    #[get_copy = "pub"]
    seed: i64,
    #[get_copy = "pub"]
    show_bounding_box: bool,
    #[get_copy = "pub"]
    size_x: i32,
    #[get_copy = "pub"]
    size_y: i32,
    #[get_copy = "pub"]
    size_z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct TrappedChest {
    custom_name: Option<String>,
    items: Option<List<ItemWithSlot>>,
    lock: Option<String>,
    loot_table: Option<String>,
    #[get_copy = "pub"]
    loot_table_seed: Option<i64>,
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
            type CookingBlockError = paste::paste!{ [< $res Error >] };
            type Target = paste::paste!{[< $res >]};

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
