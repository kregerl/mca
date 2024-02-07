use crate::{arrays::FixedSizeArray, bb::BoundingBox, entities::BlockEntity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[test]
fn test_region() {
    use crate::parse_mca;
    let filename = "region/r.0.0.mca";
    println!("{:#?}", parse_mca::<RegionChunk>(filename).unwrap());
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegionChunk {
    data_version: i32,
    #[serde(rename = "xPos")]
    x_pos: i32,
    #[serde(rename = "zPos")]
    z_pos: i32,
    #[serde(rename = "yPos")]
    y_pos: i32,
    status: String,
    last_update: i64,
    #[serde(rename = "sections")]
    sections: Vec<Section>,
    #[serde(rename = "block_entities")]
    block_entities: Option<Vec<BlockEntity>>,
    carving_masks: Option<CarvingMasks>,
    heightmaps: HeightMaps,
    // Lights,
    #[serde(rename = "fluid_ticks")]
    fluid_ticks: Vec<TileTicks>,
    #[serde(rename = "block_ticks")]
    block_ticks: Vec<TileTicks>,
    inhabited_time: i64,
    // A List of 24 (number of sections in the height) TAG_Lists that store positions
    // of blocks that need to receive an update when a proto-chunk turns into a full chunk
    post_processing: FixedSizeArray<Vec<i16>, 24>,
    #[serde(rename = "structures")]
    structures: Structures,
}

#[derive(Debug, Deserialize, Serialize)]
struct Section {
    #[serde(rename = "Y")]
    y: i8,
    block_states: SectionData<BlockStatePalette>,
    biomes: SectionData<String>,
    #[serde(rename = "BlockLight")]
    block_light: Option<FixedSizeArray<i8, 2048>>,
    #[serde(rename = "SkyLight")]
    sky_light: Option<FixedSizeArray<i8, 2048>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SectionData<T> {
    palette: Vec<T>,
    data: Option<WrappedLongVec>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WrappedLongVec(#[serde(serialize_with = "nbt::long_array")] Vec<i64>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct BlockStatePalette {
    name: String,
    properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct CarvingMasks {
    air: Option<WrappedLongVec>,
    liquid: Option<WrappedLongVec>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct HeightMaps {
    motion_blocking: Option<WrappedLongVec>,
    motion_blocking_no_leaves: Option<WrappedLongVec>,
    ocean_floor: Option<WrappedLongVec>,
    ocean_floor_wg: Option<WrappedLongVec>,
    world_surface: Option<WrappedLongVec>,
    world_surface_wg: Option<WrappedLongVec>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TileTicks {
    i: String,
    p: i32,
    t: i32,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Structures {
    #[serde(rename = "References")]
    references: HashMap<String, WrappedLongVec>,
    starts: HashMap<String, StructureStart>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct StructureStart {
    children: Vec<StructurePiece>,
    chunk_x: Option<i32>,
    chunk_z: Option<i32>,
    #[serde(rename = "id")]
    id: String,
    processed: Option<Vec<ProcessedChunk>>,
    valid: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct ProcessedChunk {
    x: i32,
    z: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct StructurePiece {
    #[serde(rename = "BB")]
    bounding_box: BoundingBox,
    // Valid values are WARM and COLD.
    biome_type: Option<String>,
    // (Village "ViSmH") Hut roof type.
    #[serde(rename = "C")]
    village_roof_type: Option<i8>,
    // (Village "ViF" and "ViDF") Crop in the farm plot
    #[serde(rename = "CA")]
    ca: Option<BlockStatePalette>,
    // (Village "ViF" and "ViDF") Crop in the farm plot
    #[serde(rename = "CB")]
    cb: Option<BlockStatePalette>,
    // (Village "ViDF") Crop in the farm plot
    #[serde(rename = "CC")]
    cc: Option<BlockStatePalette>,
    // (Village "ViDF") Crop in the farm plot
    #[serde(rename = "CD")]
    cd: Option<BlockStatePalette>,
    chest: Option<bool>,
    // (Mineshaft "MSCrossing") Indicates the "incoming" direction for the crossing.
    d: Option<i32>,
    // (Temples and huts) Depth of the structure (X/Z).
    depth: Option<i32>,
    // (Mineshaft "MSRoom") List of exits from the room.
    entrances: Option<Vec<BoundingBox>>,
    // (Stronghold) The type of door at the entry to this piece.
    entry_door: Option<String>,
    // Appears to be some sort of measure of how far this piece is from the start.
    gd: Option<i32>,
    // (Desert temple) Whether chest was placed.
    #[serde(rename = "hasPlacedChest0")]
    has_palced_chest_0: Option<bool>,
    #[serde(rename = "hasPlacedChest1")]
    has_palced_chest_1: Option<bool>,
    #[serde(rename = "hasPlacedChest2")]
    has_palced_chest_2: Option<bool>,
    #[serde(rename = "hasPlacedChest3")]
    has_palced_chest_3: Option<bool>,
    height: Option<i32>,
    // (Temples, huts and villages) Y level the structure was moved to in
    // order to place it on the surface, or -1 if it hasn't been moved yet
    h_pos: Option<i32>,
    // (Mineshaft "MSCorridor") Whether the corridor has a cave spider monster spawner.
    #[serde(rename = "hps")]
    hps: Option<bool>,
    // (Mineshaft "MSCorridor") Whether the corridor has rails.
    #[serde(rename = "hr")]
    hr: Option<bool>,
    #[serde(rename = "id")]
    id: String,
    // The integrity of this structure (only used by ocean ruins).
    #[serde(rename = "integrity")]
    integrity: Option<f32>,
    // If this ocean ruin is big.
    #[serde(rename = "isLarge")]
    is_large: Option<bool>,
    #[serde(rename = "junctions")]
    junctions: Option<Vec<VillageJunction>>,
    left: Option<bool>,
    #[serde(rename = "leftHigh")]
    left_high: Option<bool>,
    #[serde(rename = "leftLow")]
    left_low: Option<bool>,
    // (Village "ViSR") Length of the road piece
    length: Option<i32>,
    mob: Option<bool>,
    num: Option<i32>,
    // Likely orientation of the structure piece.
    #[serde(rename = "O")]
    orientation: i32,
    #[serde(rename = "placedHiddenChest")]
    placed_hidden_chest: Option<bool>,
    #[serde(rename = "placedMainChest")]
    placed_main_chest: Option<bool>,
    #[serde(rename = "palcedTrap1")]
    placed_trap_1: Option<bool>,
    #[serde(rename = "palcedTrap2")]
    placed_trap_2: Option<bool>,
    pos_x: Option<i32>,
    pos_y: Option<i32>,
    pos_z: Option<i32>,
    right: Option<bool>,
    #[serde(rename = "rightHigh")]
    right_high: Option<bool>,
    #[serde(rename = "rightLow")]
    right_low: Option<bool>,
    // Rotation of ocean ruins and shipwrecks. Valid values are
    // COUNTERCLOCKWISE_90, NONE, CLOCKWISE_90, and CLOCKWISE_180.
    rot: Option<String>,
    // (Mineshaft "MSCorridor") Whether the corridor has cobwebs.
    #[serde(rename = "sc")]
    sc: Option<bool>,
    seed: Option<i32>,
    source: Option<bool>,
    steps: Option<i32>,
    // Table: 0 is no table, 1 and 2 place it on either side of the hut.
    #[serde(rename = "T")]
    table: Option<i32>,
    tall: Option<bool>,
    template: Option<String>,
    terrace: Option<bool>,
    #[serde(rename = "tf")]
    two_floors: Option<bool>,
    #[serde(rename = "TPX")]
    tpx: Option<i32>,
    #[serde(rename = "TPY")]
    tpy: Option<i32>,
    #[serde(rename = "TPZ")]
    tpz: Option<i32>,
    // type: TODO: i8 if village or i32 is stronghold
    #[serde(rename = "VCount")]
    village_count: Option<i32>,
    width: Option<i32>,
    witch: Option<bool>,
    zombie: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
struct VillageJunction {
    source_x: i32,
    source_ground_y: i32,
    source_z: i32,
    delta_y: i32,
    // terrain_matching or rigid
    dest_proj: String,
}
