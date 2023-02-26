use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::{
    nbt::{List, Tag, Array},
    nbt_data::{chunk::BlockState, entity::Entity},
};

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BlockEntity {
    id: String,
    #[builder({default false})]
    keep_packed: bool,
    x: i32,
    y: i32,
    z: i32,
    entity_type: BlockEntityType,
}

#[derive(Debug)]
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

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Banner {
    custom_name: Option<String>,
    patterns: List<BannerPattern>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BannerPattern {
    color: i32,
    pattern: String,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Barrel {
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Item {
    id: String,
    tag: Option<HashMap<String, Tag>>,
    count: i8,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct ItemWithSlot {
    slot: i8,
    item: Item,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Beacon {
    custom_name: Option<String>,
    lock: Option<String>,
    levels: i32,
    primary: i32,
    secondary: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Beehive {
    bees: List<BeeInHive>,
    flower_pos: FlowerPos,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BeeInHive {
    entity_data: Entity,
    min_occupation_ticks: i32,
    ticks_in_hive: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct FlowerPos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BlastFurnace {
    burn_time: i16,
    cook_time: i16,
    cook_time_total: i16,
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BrewingStand {
    brew_time: i16,
    custom_name: Option<String>,
    fuel: i8,
    items: List<ItemWithSlot>,
    lock: Option<String>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Campfire {
    cooking_times: Array<i32>,
    cooking_total_times: Array<i32>,
    items: List<ItemWithSlot>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct ChiseledBookshelf {
    items: List<ItemWithSlot>,
    last_interacted_slot: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Chest {
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Comparator {
    output_signal: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct CommandBlock {
    auto: bool,
    command: String,
    condition_met: bool,
    custom_name: Option<String>,
    last_execution: i64,
    last_output: String,
    powered: bool,
    success_count: i32,
    track_output: bool,
    update_last_execution: bool,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Conduit {
    target: Array<i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Dispenser {
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Dropper {
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct EnchantingTable {
    custom_name: Option<String>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct EndGateway {
    age: i64,
    exact_teleport: bool,
    exit_portal: ExitPortal,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct ExitPortal {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Furnace {
    burn_time: i16,
    cook_time: i16,
    cook_time_total: i16,
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Hopper {
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
    transfer_cooldown: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Jigsaw {
    final_state: String,
    joint: String,
    name: String,
    pool: String,
    target: String,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Jukebox {
    is_playing: bool,
    record_item: Item,
    record_start_tick: i64,
    tick_count: i64,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Lectern {
    book: Item,
    page: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct MobSpawner {
    spawner: Spawner,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Spawner {
    delay: i16,
    max_nearby_entities: i16,
    max_spawn_delay: i16,
    min_spawn_delay: i16,
    required_player_range: i16,
    spawn_count: i16,
    spawn_data: HashMap<String, Tag>,
    spawn_potentials: List<PotentialSpawn>,
    spawn_range: i16,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct PotentialSpawn {
    weight: i32,
    data: HashMap<String, Tag>,
    custom_spawn_rules: Option<CustomSpawnRules>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct CustomSpawnRules {
    block_light_limit: i32,
    sky_light_limit: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Piston {
    block_state: BlockState,
    extending: bool,
    facing: i32,
    progress: f32,
    source: bool,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct ShulkerBox {
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Sign {
    glowing_text: bool,
    color: String,
    text1: String,
    text2: String,
    text3: String,
    text4: String,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Skull {
    note_block_sound: Option<String>,
    extra_type: String,
    skull_owner: Option<SkullOwner>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct SkullOwner {
    id: Array<i32>,
    name: Option<String>,
    properties: List<SkullOwnerProperties>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct SkullOwnerProperties {
    textures: List<SkullOwnerTextures>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct SkullOwnerTextures {
    value: String,
    signature: Option<String>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Smoker {
    burn_time: i16,
    cook_time: i16,
    cook_time_total: i16,
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct SoulCampfire {
    cooking_times: Array<i32>,
    cooking_total_times: Array<i32>,
    items: List<ItemWithSlot>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct StructureBlock {
    author: String,
    ignore_entities: bool,
    integrity: f32,
    metadata: String,
    mirror: String,
    mode: String,
    name: String,
    pos_x: i32,
    pos_y: i32,
    pos_z: i32,
    powered: bool,
    rotation: String,
    seed: i64,
    show_bounding_box: bool,
    size_x: i32,
    size_y: i32,
    size_z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct TrappedChest {
    custom_name: Option<String>,
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

macro_rules! impl_IBE_for_builder {
    ($ty:ty) => {
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
    };
}

macro_rules! impl_CBEB_for_builder {
    ($ty:ty) => {
        impl CookingBlockEntityBuilder for $ty {
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
        }
    };
}

impl_IBE_for_builder!(BarrelBuilder);
impl_IBE_for_builder!(ChestBuilder);
impl_IBE_for_builder!(DispenserBuilder);
impl_IBE_for_builder!(DropperBuilder);
impl_IBE_for_builder!(HopperBuilder);
impl_IBE_for_builder!(ShulkerBoxBuilder);
impl_IBE_for_builder!(TrappedChestBuilder);
impl_CBEB_for_builder!(BlastFurnaceBuilder);
impl_CBEB_for_builder!(FurnaceBuilder);
impl_CBEB_for_builder!(SmokerBuilder);

pub trait InventoryBlockEntityBuilder {
    fn set_custom_name(&mut self, custom_name: String);
    fn set_items(&mut self, items: List<ItemWithSlot>);
    fn set_lock(&mut self, lock: String);
    fn set_loot_table(&mut self, loot_table: String);
    fn set_loot_table_seed(&mut self, loot_table_seed: i64);
}

pub trait CookingBlockEntityBuilder {
    fn set_burn_time(&mut self, burn_time: i16);
    fn set_cook_time(&mut self, cook_time: i16);
    fn set_cook_time_total(&mut self, cook_time_total: i16);
    fn set_custom_name(&mut self, custom_name: String);
    fn set_items(&mut self, items: List<ItemWithSlot>);
    fn set_lock(&mut self, lock: String);
    fn set_recipes_used(&mut self, recipes_used: HashMap<String, i32>);
}
