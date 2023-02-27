use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use jbe::Builder;

use crate::{
    nbt::{Array, List, Tag},
    nbt_data::{chunk::BlockState, entity::Entity},
};

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BlockEntity {
    #[get = "pub"]
    id: String,
    #[get_copy = "pub"]
    #[builder({default false})]
    keep_packed: bool,
    #[get_copy = "pub"]
    x: i32,
    #[get_copy = "pub"]
    y: i32,
    #[get_copy = "pub"]
    z: i32,
    #[get = "pub"]
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

#[derive(Debug, Builder, Getters)]
#[get = "pub"]
pub struct Banner {
    custom_name: Option<String>,
    #[get = "pub"]
    patterns: List<BannerPattern>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BannerPattern {
    #[get_copy = "pub"]
    color: i32,
    #[get = "pub"]
    pattern: String,
}

#[derive(Debug, Builder, Getters)]
pub struct Barrel {
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}

impl Barrel {
    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }

    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn loot_table(&self) -> Option<&String> {
        self.loot_table.as_ref()
    }

    pub fn loot_table_seed(&self) -> Option<i64> {
        self.loot_table_seed
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Item {
    #[get = "pub"]
    id: String,
    tag: Option<HashMap<String, Tag>>,
    #[get_copy = "pub"]
    count: i8,
}

impl Item {
    pub fn tag(&self) -> Option<&HashMap<String, Tag>> {
        self.tag.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct ItemWithSlot {
    #[get_copy = "pub"]
    slot: i8,
    #[get = "pub"]
    item: Item,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Beacon {
    #[get = "pub"]
    custom_name: Option<String>,
    #[get = "pub"]
    lock: Option<String>,
    #[get_copy = "pub"]
    levels: i32,
    #[get_copy = "pub"]
    primary: i32,
    #[get_copy = "pub"]
    secondary: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
#[get = "pub"]
pub struct Beehive {
    bees: List<BeeInHive>,
    flower_pos: FlowerPos,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BeeInHive {
    #[get = "pub"]
    entity_data: Entity,
    #[get_copy = "pub"]
    min_occupation_ticks: i32,
    #[get_copy = "pub"]
    ticks_in_hive: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
#[get_copy = "pub"]
pub struct FlowerPos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BlastFurnace {
    #[get_copy = "pub"]
    burn_time: i16,
    #[get_copy = "pub"]
    cook_time: i16,
    #[get_copy = "pub"]
    cook_time_total: i16,
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    #[get = "pub"]
    recipes_used: HashMap<String, i32>,
}

impl BlastFurnace {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct BrewingStand {
    #[get_copy = "pub"]
    brew_time: i16,
    custom_name: Option<String>,
    #[get_copy = "pub"]
    fuel: i8,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
}

impl BrewingStand {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Campfire {
    #[get = "pub"]
    cooking_times: Array<i32>,
    #[get = "pub"]
    cooking_total_times: Array<i32>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct ChiseledBookshelf {
    #[get = "pub"]
    items: List<ItemWithSlot>,
    #[get_copy = "pub"]
    last_interacted_slot: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Chest {
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    #[get_copy = "pub"]
    loot_table_seed: Option<i64>,
}

impl Chest {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }

    pub fn loot_table(&self) -> Option<&String> {
        self.loot_table.as_ref()
    }

}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Comparator {
    #[get_copy = "pub"]
    output_signal: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct CommandBlock {
    #[get_copy = "pub"]
    auto: bool,
    #[get = "pub"]
    command: String,
    #[get_copy = "pub"]
    condition_met: bool,
    custom_name: Option<String>,
    #[get_copy = "pub"]
    last_execution: i64,
    #[get = "pub"]
    last_output: String,
    #[get_copy = "pub"]
    powered: bool,
    #[get_copy = "pub"]
    success_count: i32,
    #[get_copy = "pub"]
    track_output: bool,
    #[get_copy = "pub"]
    update_last_execution: bool,
}

impl CommandBlock {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Conduit {
    #[get = "pub"]
    target: Array<i32>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Dispenser {
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    #[get_copy = "pub"]
    loot_table_seed: Option<i64>,
}

impl Dispenser {
    pub fn loot_table(&self) -> Option<&String> {
        self.loot_table.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }

    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Dropper {
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    #[get_copy = "pub"]
    loot_table_seed: Option<i64>,
}

impl Dropper {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }

    pub fn loot_table(&self) -> Option<&String> {
        self.loot_table.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct EnchantingTable {
    custom_name: Option<String>,
}

impl EnchantingTable {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct EndGateway {
    #[get_copy = "pub"]
    age: i64,
    #[get_copy = "pub"]
    exact_teleport: bool,
    #[get = "pub"]
    exit_portal: ExitPortal,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct ExitPortal {
    #[get_copy = "pub"]
    x: i32,
    #[get_copy = "pub"]
    y: i32,
    #[get_copy = "pub"]
    z: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Furnace {
    #[get_copy = "pub"]
    burn_time: i16,
    #[get_copy = "pub"]
    cook_time: i16,
    #[get_copy = "pub"]
    cook_time_total: i16,
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    #[get = "pub"]
    recipes_used: HashMap<String, i32>,
}

impl Furnace {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Hopper {
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    #[get_copy = "pub"]
    loot_table_seed: Option<i64>,
    #[get_copy = "pub"]
    transfer_cooldown: i32,
}

impl Hopper {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn loot_table(&self) -> Option<&String> {
        self.loot_table.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Jigsaw {
    #[get = "pub"]
    final_state: String,
    #[get = "pub"]
    joint: String,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    pool: String,
    #[get = "pub"]
    target: String,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Jukebox {
    #[get_copy = "pub"]
    is_playing: bool,
    #[get = "pub"]
    record_item: Item,
    #[get_copy = "pub"]
    record_start_tick: i64,
    #[get_copy = "pub"]
    tick_count: i64,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Lectern {
    #[get = "pub"]
    book: Item,
    #[get_copy = "pub"]
    page: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct MobSpawner {
    #[get = "pub"]
    spawner: Spawner,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Spawner {
    #[get_copy = "pub"]
    delay: i16,
    #[get_copy = "pub"]
    max_nearby_entities: i16,
    #[get_copy = "pub"]
    max_spawn_delay: i16,
    #[get_copy = "pub"]
    min_spawn_delay: i16,
    #[get_copy = "pub"]
    required_player_range: i16,
    #[get_copy = "pub"]
    spawn_count: i16,
    #[get = "pub"]
    spawn_data: HashMap<String, Tag>,
    #[get = "pub"]
    spawn_potentials: List<PotentialSpawn>,
    #[get_copy = "pub"]
    spawn_range: i16,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct PotentialSpawn {
    #[get_copy = "pub"]
    weight: i32,
    #[get = "pub"]
    data: HashMap<String, Tag>,
    custom_spawn_rules: Option<CustomSpawnRules>,
}

impl PotentialSpawn {
    pub fn custom_spawn_rules(&self) -> Option<&CustomSpawnRules> {
        self.custom_spawn_rules.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct CustomSpawnRules {
    #[get_copy = "pub"]
    block_light_limit: i32,
    #[get_copy = "pub"]
    sky_light_limit: i32,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Piston {
    #[get = "pub"]
    block_state: BlockState,
    #[get_copy = "pub"]
    extending: bool,
    #[get_copy = "pub"]
    facing: i32,
    #[get_copy = "pub"]
    progress: f32,
    #[get_copy = "pub"]
    source: bool,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct ShulkerBox {
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    #[get_copy = "pub"]
    loot_table_seed: Option<i64>,
}

impl ShulkerBox {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }

    pub fn loot_table(&self) -> Option<&String> {
        self.loot_table.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
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

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Skull {
    note_block_sound: Option<String>,
    #[get = "pub"]
    extra_type: String,
    skull_owner: Option<SkullOwner>,
}

impl Skull {
    pub fn note_block_sound(&self) -> Option<&String> {
        self.note_block_sound.as_ref()
    }

    pub fn skull_owner(&self) -> Option<&SkullOwner> {
        self.skull_owner.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct SkullOwner {
    #[get = "pub"]
    id: Array<i32>,
    name: Option<String>,
    #[get = "pub"]
    properties: List<SkullOwnerProperties>,
}

impl SkullOwner {
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct SkullOwnerProperties {
    #[get = "pub"]
    textures: List<SkullOwnerTextures>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct SkullOwnerTextures {
    #[get = "pub"]
    value: String,
    signature: Option<String>,
}

impl SkullOwnerTextures {
    pub fn signature(&self) -> Option<&String> {
        self.signature.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct Smoker {
    #[get_copy = "pub"]
    burn_time: i16,
    #[get_copy = "pub"]
    cook_time: i16,
    #[get_copy = "pub"]
    cook_time_total: i16,
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    #[get = "pub"]
    recipes_used: HashMap<String, i32>,
}

impl Smoker {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }
}

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct SoulCampfire {
    #[get = "pub"]
    cooking_times: Array<i32>,
    #[get = "pub"]
    cooking_total_times: Array<i32>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
}

#[derive(Debug, Builder, Getters, CopyGetters)]
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

#[derive(Debug, Builder, Getters, CopyGetters)]
pub struct TrappedChest {
    custom_name: Option<String>,
    #[get = "pub"]
    items: List<ItemWithSlot>,
    lock: Option<String>,
    loot_table: Option<String>,
    #[get_copy = "pub"]
    loot_table_seed: Option<i64>,
}

impl TrappedChest {
    pub fn custom_name(&self) -> Option<&String> {
        self.custom_name.as_ref()
    }

    pub fn loot_table(&self) -> Option<&String> {
        self.loot_table.as_ref()
    }

    pub fn lock(&self) -> Option<&String> {
        self.lock.as_ref()
    }
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
