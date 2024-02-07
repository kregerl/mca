use serde::{Deserialize, Serialize};

use crate::{vec::{Vec3I, Vec3D, Vec2F, Vec2I}, uuid::Uuid};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityChunk {
    data_version: i32,
    entities: Vec<Entity>,
    position: Vec2I,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemTag {
    damage: Option<i32>,
    unbreakable: Option<bool>,
    can_destroy: Vec<String>,
    custom_model_data: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "Count")]
    count: i8,
    #[serde(rename = "Slot")]
    slot: Option<i8>,
    id: String,
    tag: Option<ItemTag>
}

// TODO: Add the other fields.
#[derive(Debug, Deserialize, Serialize)]
pub struct Brain {
    memories: Option<Memories>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValueCompound<T> {
    value: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TTLCompound<T> {
    value: bool,
    ttl: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmptyTTLCompound<T> {
    ttl: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoolCompound;

#[derive(Debug, Deserialize, Serialize)]
pub struct Home {
    value: HomeValue 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HomeValue {
    dimension: String,
    pos: Vec3I,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Memories {
    #[serde(rename = "minecraft:admiring_disabled")]
    admiring_disabled: Option<TTLCompound<i64>>,
    #[serde(rename = "minecraft:admiring_item")]
    admiring_item: Option<TTLCompound<i64>>,
    #[serde(rename = "minecraft:angry_at")]
    angry_at: Option<TTLCompound<i64>>,
    #[serde(rename = "minecraft:dig_cooldown")]
    dig_cooldown: Option<ValueCompound<i64>>,
    #[serde(rename = "minecraft:gaze_cooldown_ticks")]
    gaze_cooldown_ticks: Option<ValueCompound<i32>>,
    #[serde(rename = "minecraft:golem_detected_recently")]
    golem_detected_recently: Option<TTLCompound<i32>>,
    #[serde(rename = "minecraft:has_hunting_cooldown")]
    has_hunting_cooldown: Option<TTLCompound<i32>>,
    #[serde(rename = "minecraft:home")]
    home: Option<Home>,
    #[serde(rename = "minecraft:hunted_recently")]
    hunted_recently: Option<TTLCompound<i32>>,
    #[serde(rename = "minecraft:is_emerging")]
    is_emerging: Option<BoolCompound>,
    #[serde(rename = "minecraft:is_in_water")]
    is_in_water: Option<BoolCompound>,
    #[serde(rename = "minecraft:is_pregnant")]
    is_pregnant: Option<BoolCompound>,
    #[serde(rename = "minecraft:is_sniffing")]
    is_sniffing: Option<BoolCompound>,
    #[serde(rename = "minecraft:is_tempted")]
    is_tempted: Option<ValueCompound<bool>>,
    #[serde(rename = "minecraft:item_pickup_cooldown_ticks")]
    item_pickup_cooldown_ticks: Option<ValueCompound<i32>>,
    #[serde(rename = "minecraft:job_site")]
    job_site: Option<Home>,
    #[serde(rename = "minecraft:last_slept")]
    last_slept: Option<i64>,
    #[serde(rename = "minecraft:last_woken")]
    last_woken: Option<i64>,
    #[serde(rename = "minecraft:last_worked_at_poi")]
    last_worked_at_poi: Option<i64>,
    #[serde(rename = "minecraft:liked_noteblock")]
    liked_noteblock: Option<Home>,
    #[serde(rename = "minecraft:liked_noteblock_cooldown_ticks")]
    liked_noteblock_cooldown_ticks: Option<ValueCompound<i32>>,
    #[serde(rename = "minecraft:liked_player")]
    liked_player: Option<ValueCompound<Uuid>>,
    #[serde(rename = "minecraft:long_jump_cooling_down")]
    long_jump_cooling_down: Option<ValueCompound<i32>>,
    #[serde(rename = "minecraft:meeting_point")]
    meeting_point: Option<Home>,
    #[serde(rename = "minecraft:play_dead_ticks")]
    play_dead_ticks: Option<ValueCompound<i32>>,
    #[serde(rename = "minecraft:potential_job_site")]
    potential_job_site: Option<Home>,
    #[serde(rename = "minecraft:ram_cooldown_ticks")]
    ram_cooldown_ticks: Option<ValueCompound<i32>>,
    #[serde(rename = "minecraft:recent_projectiles")]
    recent_projectiles: Option<EmptyTTLCompound<i64>>,
    #[serde(rename = "minecraft:roar_sound_cooldown")]
    roar_sound_cooldown: Option<EmptyTTLCompound<i64>>,
    #[serde(rename = "minecraft:roar_sound_delay")]
    roar_sound_delay: Option<EmptyTTLCompound<i64>>,
    #[serde(rename = "minecraft:sniff_cooldown")]
    sniff_cooldown: Option<EmptyTTLCompound<i64>>,
    #[serde(rename = "minecraft:sniffer_explored_positions")]
    sniffer_explored_positions: Option<Vec3I>,
    #[serde(rename = "minecraft:temptation_cooldown_ticks")]
    temptation_cooldown_ticks: Option<i32>,
    #[serde(rename = "minecraft:touch_cooldown")]
    touch_cooldown: Option<EmptyTTLCompound<i64>>,
    #[serde(rename = "minecraft:universal_anger")]
    universal_anger: Option<TTLCompound<i64>>,
    #[serde(rename = "minecraft:vibration_cooldown")]
    vibration_cooldown: Option<EmptyTTLCompound<i64>>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttributeModifiers {
    amount: f64,
    name: String,
    operation: i32,
    uuid: Uuid
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    base: f64,
    name: String,
    modifiers: Option<Vec<AttributeModifiers>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HandItem;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Entity {
    air: i16,
    custom_name: Option<String>,
    custom_name_visible: Option<bool>,
    fall_distance: f32,
    fire: i16,
    glowing: Option<bool>,
    has_visual_fire: Option<bool>,
    #[serde(rename = "id")]
    id: String,
    invulnerable: bool,
    motion: Vec<f64>,
    no_gravity: Option<bool>,
    on_ground: bool,
    passengers: Option<Vec<Entity>>,
    portal_cooldown: i32,
    pos: Vec3D,
    rotation: Vec2F,
    silent: Option<bool>,
    ticks_frozen: Option<i32>,
    #[serde(rename = "UUID")]
    uuid: Uuid,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct HiddenEffect;

#[derive(Debug, Deserialize, Serialize)]
pub struct PotionEffect {
    ambient: bool,
    amplifier: i8,
    duration: i32,
    hidden_effect: Option<HiddenEffect>,
    id: i32,
    show_icon: bool,
    show_particles: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Leash {
    #[serde(rename = "UUID")]
    uuid: Option<Uuid>,
    x: Option<i32>,
    y: Option<i32>,
    z: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MobEntity {
    absorption_amount: f32,
    #[serde(rename = "active_effects")]
    active_effects: Vec<PotionEffect>,
    armor_drop_chances: Vec<[f32; 4]>,
    armor_items: Vec<Item>,
    attributes: Vec<Attribute>,
    brain: Brain,
    can_pickup_loot: bool, 
    death_loot_table: Option<String>,
    death_loot_table_seed :Option<i64>,
    death_time: i16,
    fall_flying: i8,
    health: f32,
    hurt_by_timestamp: i32,
    hurt_time: i16,
    hand_drop_chances: Vec<f32>,
    hand_items: Vec<Item>,
    leash: Option<Leash>,
    left_handed: bool,
    #[serde(rename = "NoAI")]
    no_ai: bool,
    persistence_required: bool,
    sleeping_x: i32,
    sleeping_y: i32,
    sleeping_z: i32,
    // The 'Team" tag is actually not part of the NBT data of a mob, but instead used when spawning it, 
    // so it cannot be tested for. It makes the mob instantly join the scoreboard team with that name.
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AngryEntityTags {
    anger_time: i32,
    angry_at: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CanBreedTags {
    age: i32,
    forced_age: i32,
    in_love: i32,
    love_cause: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CanBeTamedTags {
    owner: Uuid,
    sitting: bool,    
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HorseTags {
    bred: bool,
    eating_hatstack: bool,
    owner: Uuid,
    saddle_item: Option<Item>,
    tame: bool,
    temper: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ZombieTags {
    can_break_doors: bool,
    drowned_conversion_time: i32,
    in_water_time: i32,
    is_baby: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CanBeInRaidTags {
    can_join_raid: bool, 
    patrol_leader: bool,
    patrolling: bool,
    patrol_target: Vec3I,
    raid_id: i32,
    wave: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Gossip {
    value: i32,
    target: Uuid,
    #[serde(rename = "type")]
    typ: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Offers {
    recipes: Vec<Recipe>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    buy: Item,
    buy_b: Item,
    demand: i32,
    max_uses: i32,
    price_multiplier: f32,
    reward_exp: bool,
    sell: Item,
    special_price: i32,
    uses: i32,
    xp: i32,
}

#[derive(Debug, Deserialize, Serialize)]
enum VillagerLevel {
    Novice = 1,
    Apprentice = 2,
    Journeyman = 3,
    Expert = 4,
    Master = 5,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VillagerData {
    level: VillagerLevel,
    profession: String,
    #[serde(rename = "type")]
    typ: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VillagerTags {
    gossips: Vec<Gossip>,
    offers: Offers,
    villager_data: VillagerData,
    xp: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProjectileTags {
    has_been_shot: bool,
    left_owner: Option<bool>,
    owner: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrowTags {
    crit: bool,
    damage: f64,
    // inBlockState
    in_ground: bool,
    life: i16,
    pickup: i8,
    #[serde(rename = "PierceLevel")]
    pierce_level: i8,
    shake: i8,
    #[serde(rename = "ShotFromCrossbow")]
    shot_from_crossbow: bool,
    #[serde(rename = "SoundEvent")]
    sound_event: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FireballTags {
    power: Vec3D
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoatTags {
    #[serde(rename = "Type")]
    typ: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerEntityTags {
    items: Vec<Item>,
    loot_table: Option<String>,
    loot_table_seed: Option<i64>,
}   

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecartTags {
    custom_display_tile: Option<bool>,
    display_offset: Option<i32>,
    // DisplayState
}   

#[derive(Debug, Deserialize, Serialize)]
pub struct SpawnPotential<T> {
    weight: i32,
    data: SpawnPotentialData<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpawnPotentialData<T> {
    entity: T,
    custom_spawn_rules: CustomSpawnRules,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomSpawnRules {
    block_light_limit: i32,
    sky_light_limit: i32,
}

// FIXME: Maybe remove the 'T' from this struct since theres not really a way to know the entity type
// before tring to deserialize the struct. Replace with Entity?
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpawnerTags<T> {
    delay: i16,
    max_nearby_entities: i16,
    max_spawn_delay: i16,
    min_spaw_delay: i16,
    required_player_percentage: i16,
    spawn_count: i16,
    spawn_data: T,
    spawn_potentials: Option<Vec<SpawnPotential<T>>>,
    spawn_range: i16
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HangableTags {
    facing: i8,
    tile_x: i32,
    tile_y: i32,
    tile_z: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockEntity {
    id: String,
    keep_packed: Option<bool>,
    x: i32,
    y: i32,
    z: i32,
}