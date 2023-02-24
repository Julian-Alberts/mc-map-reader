use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::{
    nbt::Tag,
    nbt_data::{chunk::BlockState, entity::Entity},
};

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct BlockEntity {
    id: String,
    keep_packed: bool,
    x: i32,
    y: i32,
    z: i32,
    entity_type: BlockEntityType,
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
    Skull(Skull),
    Smoker(Smoker),
    StructureBlock(StructureBlock),
    TrappedChest(TrappedChest),
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Banner {
    custom_name: Option<String>,
    patterns: Vec<BannerPattern>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct BannerPattern {
    color: i32,
    pattern: String,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Barrel {
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Item {
    id: String,
    tag: Option<HashMap<String, Tag>>,
    count: i8,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct ItemWithSlot {
    slot: i8,
    item: Item,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Beacon {
    custom_name: Option<String>,
    lock: Option<String>,
    levels: i32,
    primary: i32,
    secondary: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Beehive {
    bees: Vec<BeeInHive>,
    flower_pos: FlowerPos,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct BeeInHive {
    entity_data: Entity,
    min_occupation_ticks: i32,
    ticks_in_hive: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct FlowerPos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct BlastFurnace {
    burn_time: i16,
    cook_time: i16,
    cook_time_total: i16,
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct BrewingStand {
    brew_time: i16,
    custom_name: Option<String>,
    fuel: i8,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Campfire {
    cooking_times: Vec<i32>,
    cooking_total_times: Vec<i32>,
    items: Vec<ItemWithSlot>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct ChiseledBookshelf {
    items: Vec<ItemWithSlot>,
    last_interacted_slot: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Chest {
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Comparator {
    output_signal: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
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

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Conduit {
    target: Vec<i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Dispenser {
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Dropper {
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct EnchantingTable {
    custom_name: Option<String>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct EndGateway {
    age: i64,
    exact_teleport: bool,
    exit_portal: ExitPortal,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct ExitPortal {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Furnace {
    burn_time: i16,
    cook_time: i16,
    cook_time_total: i16,
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Hopper {
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
    transfer_cooldown: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Jigsaw {
    final_state: String,
    join: String,
    name: String,
    pool: String,
    target: String,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Jukebox {
    is_playing: bool,
    record_item: Item,
    record_start_tick: i64,
    tick_count: i64,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Lectern {
    book: Item,
    page: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct MobSpawner {
    spawner: Spawner,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Spawner {
    delay: i16,
    max_nearby_entities: i16,
    max_spawn_delay: i16,
    min_spawn_delay: i16,
    required_player_range: i16,
    spawn_count: i16,
    spawn_data: HashMap<String, Tag>,
    spawn_potentials: Vec<PotentialSpawn>,
    spawn_range: i16,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct PotentialSpawn {
    weight: i32,
    data: HashMap<String, Tag>,
    custom_spawn_rules: Option<CustomSpawnRules>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct CustomSpawnRules {
    block_light_limit: i32,
    sky_light_limit: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Piston {
    block_state: BlockState,
    extending: bool,
    facing: i32,
    progress: f32,
    source: bool,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct ShulkerBox {
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Sign {
    glowing_text: bool,
    color: String,
    text1: String,
    text2: String,
    text3: String,
    text4: String,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Skull {
    note_block_sound: Option<String>,
    extra_type: String,
    skull_owner: Option<SkullOwner>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct SkullOwner {
    id: Vec<i32>,
    name: Option<String>,
    properties: Vec<SkullOwnerProperties>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct SkullOwnerProperties {
    textures: Vec<SkullOwnerTextures>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct SkullOwnerTextures {
    value: String,
    signature: Option<String>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct Smoker {
    burn_time: i16,
    cook_time: i16,
    cook_time_total: i16,
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    recipes_used: HashMap<String, i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct SoulCampfire {
    cooking_times: Vec<i32>,
    cooking_total_times: Vec<i32>,
    items: Vec<ItemWithSlot>,
}

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
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

#[derive(Debug, Builder, Getters, CopyGetters, Clone)]
pub struct TrappedChest {
    custom_name: Option<String>,
    items: Vec<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}
