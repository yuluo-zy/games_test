//! Original B enum/serde identities recovered from serializers and native equality.
//! AppMode: 0x140afc7c0, 0x140afde20; EditSubject: 0x140afc4a0;
//! DecoratorType: 0x140a478f0; ClutterMeshTy names: 0x142cda7c8 (56 entries).
use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum WallShape {
    Freeform = 0,
    Circle = 1,
    Rectangle = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EraserLayer {
    All = 0,
    Wall = 1,
    Path = 2,
    Bulldoze = 3,
    Water = 4,
    Garden = 5,
    Tree = 6,
}

/// The six editing variants occupy AppMode's native tags 0..=5.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EditSubject {
    #[default]
    None,
    Wall(u64),
    Roof(u64),
    DecoClutter {
        decorator_id: u32,
    },
    StairPoint {
        decorator_id: u32,
    },
    Terrain(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ClutterMeshTy {
    #[serde(rename = "plant_pot_v1")]
    PlantPotV1 = 0,
    #[serde(rename = "ladder")]
    Ladder = 1,
    #[serde(rename = "wheel")]
    Wheel = 2,
    #[serde(rename = "pitchfork")]
    Pitchfork = 3,
    #[serde(rename = "shovel")]
    Shovel = 4,
    #[serde(rename = "box_w_stuff")]
    BoxWStuff = 5,
    #[serde(rename = "watering_can")]
    WateringCan = 6,
    #[serde(rename = "firewood")]
    Firewood = 7,
    #[serde(rename = "bucket")]
    Bucket = 8,
    #[serde(rename = "birdnest")]
    Birdnest = 9,
    #[serde(rename = "bench")]
    Bench = 10,
    #[serde(rename = "bench_w_basket")]
    BenchWBasket = 11,
    #[serde(rename = "bench_short")]
    BenchShort = 12,
    #[serde(rename = "barrel_w_plantpot")]
    BarrelWPlantpot = 13,
    #[serde(rename = "haystack")]
    Haystack = 14,
    #[serde(rename = "plant_pot_v2")]
    PlantPotV2 = 15,
    #[serde(rename = "birdhouse")]
    Birdhouse = 16,
    #[serde(rename = "basket")]
    Basket = 17,
    #[serde(rename = "basket_w_food")]
    BasketWFood = 18,
    #[serde(rename = "barrel_w_food_basket")]
    BarrelWFoodBasket = 19,
    #[serde(rename = "potato_sack")]
    PotatoSack = 20,
    #[serde(rename = "road_sign")]
    RoadSign = 21,
    #[serde(rename = "sunflower")]
    Sunflower = 22,
    #[serde(rename = "cart")]
    Cart = 23,
    #[serde(rename = "cart2")]
    Cart2 = 24,
    #[serde(rename = "crates_w_tools")]
    CratesWTools = 25,
    #[serde(rename = "crates_w_flowers")]
    CratesWFlowers = 26,
    #[serde(rename = "window_flowerpot")]
    WindowFlowerpot = 27,
    #[serde(rename = "window_shutter")]
    WindowShutter = 28,
    #[serde(rename = "window_flowerbeds")]
    WindowFlowerbeds = 29,
    #[serde(rename = "reed")]
    Reed = 30,
    #[serde(rename = "lilypad")]
    Lilypad = 31,
    #[serde(rename = "fallen_leaves")]
    FallenLeaves = 32,
    #[serde(rename = "fallen_petals")]
    FallenPetals = 33,
    #[serde(rename = "olden_rock")]
    OldenRock = 34,
    #[serde(rename = "olden_tree_log")]
    OldenTreeLog = 35,
    #[serde(rename = "olden_tree_stump")]
    OldenTreeStump = 36,
    #[serde(rename = "pumpkin")]
    Pumpkin = 37,
    #[serde(rename = "pumpkin2")]
    Pumpkin2 = 38,
    #[serde(rename = "pumpkin_w_face")]
    PumpkinWFace = 39,
    #[serde(rename = "cart_w_pumpkins")]
    CartWPumpkins = 40,
    #[serde(rename = "crates_w_pumpkins")]
    CratesWPumpkins = 41,
    #[serde(rename = "olden_tree_stump_autumn")]
    OldenTreeStumpAutumn = 42,
    #[serde(rename = "barrel_w_candles")]
    BarrelWCandles = 43,
    #[serde(rename = "anvil")]
    Anvil = 44,
    #[serde(rename = "fabric_rolls")]
    FabricRolls = 45,
    #[serde(rename = "stall_bread")]
    StallBread = 46,
    #[serde(rename = "stall_veggies")]
    StallVeggies = 47,
    #[serde(rename = "crate_fish")]
    CrateFish = 48,
    #[serde(rename = "rack_brooms")]
    RackBrooms = 49,
    #[serde(rename = "flowerbox")]
    Flowerbox = 50,
    #[serde(rename = "door_bell")]
    DoorBell = 51,
    #[serde(rename = "door_krans")]
    DoorKrans = 52,
    #[serde(rename = "door_krans_autumn")]
    DoorKransAutumn = 53,
    #[serde(rename = "cart_w_pumpkins_regular")]
    CartWPumpkinsRegular = 54,
    #[serde(rename = "crates_w_pumpkins_regular")]
    CratesWPumpkinsRegular = 55,
}
impl ClutterMeshTy {
    pub fn from_native_tag(tag: u8) -> Option<Self> {
        let all: [ClutterMeshTy; 56] = [
            Self::PlantPotV1,
            Self::Ladder,
            Self::Wheel,
            Self::Pitchfork,
            Self::Shovel,
            Self::BoxWStuff,
            Self::WateringCan,
            Self::Firewood,
            Self::Bucket,
            Self::Birdnest,
            Self::Bench,
            Self::BenchWBasket,
            Self::BenchShort,
            Self::BarrelWPlantpot,
            Self::Haystack,
            Self::PlantPotV2,
            Self::Birdhouse,
            Self::Basket,
            Self::BasketWFood,
            Self::BarrelWFoodBasket,
            Self::PotatoSack,
            Self::RoadSign,
            Self::Sunflower,
            Self::Cart,
            Self::Cart2,
            Self::CratesWTools,
            Self::CratesWFlowers,
            Self::WindowFlowerpot,
            Self::WindowShutter,
            Self::WindowFlowerbeds,
            Self::Reed,
            Self::Lilypad,
            Self::FallenLeaves,
            Self::FallenPetals,
            Self::OldenRock,
            Self::OldenTreeLog,
            Self::OldenTreeStump,
            Self::Pumpkin,
            Self::Pumpkin2,
            Self::PumpkinWFace,
            Self::CartWPumpkins,
            Self::CratesWPumpkins,
            Self::OldenTreeStumpAutumn,
            Self::BarrelWCandles,
            Self::Anvil,
            Self::FabricRolls,
            Self::StallBread,
            Self::StallVeggies,
            Self::CrateFish,
            Self::RackBrooms,
            Self::Flowerbox,
            Self::DoorBell,
            Self::DoorKrans,
            Self::DoorKransAutumn,
            Self::CartWPumpkinsRegular,
            Self::CratesWPumpkinsRegular,
        ];
        all.get(tag as usize).copied()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DecoratorType {
    CottageWindow,
    GothicWindow,
    ArrowSlit,
    Chimney,
    Lantern,
    Flag,
    Clutter { ty: ClutterMeshTy },
    Stairs,
}
impl DecoratorType {
    pub const fn native_tag(self) -> u8 {
        match self {
            Self::CottageWindow => 56,
            Self::GothicWindow => 57,
            Self::ArrowSlit => 58,
            Self::Chimney => 59,
            Self::Lantern => 60,
            Self::Flag => 61,
            Self::Clutter { ty } => ty as u8,
            Self::Stairs => 63,
        }
    }
    pub fn from_native_tag(tag: u8) -> Option<Self> {
        Some(match tag {
            56 => Self::CottageWindow,
            57 => Self::GothicWindow,
            58 => Self::ArrowSlit,
            59 => Self::Chimney,
            60 => Self::Lantern,
            61 => Self::Flag,
            63 => Self::Stairs,
            n => Self::Clutter {
                ty: ClutterMeshTy::from_native_tag(n)?,
            },
        })
    }
}

/// All sixteen original variants. Native Rust enum layout uses the six
/// EditSubject discriminants as the first six tags; this API does not assume
/// the rebuilding compiler reproduces that private ABI.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppMode {
    Editing(EditSubject),
    Wall(WallShape),
    Path,
    Cobblestone(u64),
    Eraser(EraserLayer),
    Terrain,
    Decorator(DecoratorType),
    Color,
    Replay,
    LoadGame,
    Water,
    Tree,
    Garden,
    Fences,
    Photomode,
    CloneClutter,
}
impl Default for AppMode {
    fn default() -> Self {
        Self::Editing(EditSubject::None)
    }
}
impl AppMode {
    pub const fn native_tag(self) -> u32 {
        match self {
            Self::Editing(EditSubject::None) => 0,
            Self::Editing(EditSubject::Wall(_)) => 1,
            Self::Editing(EditSubject::Roof(_)) => 2,
            Self::Editing(EditSubject::DecoClutter { .. }) => 3,
            Self::Editing(EditSubject::StairPoint { .. }) => 4,
            Self::Editing(EditSubject::Terrain(_)) => 5,
            Self::Wall(_) => 6,
            Self::Path => 7,
            Self::Cobblestone(_) => 8,
            Self::Eraser(_) => 9,
            Self::Terrain => 10,
            Self::Decorator(_) => 11,
            Self::Color => 12,
            Self::Replay => 13,
            Self::LoadGame => 14,
            Self::Water => 15,
            Self::Tree => 16,
            Self::Garden => 17,
            Self::Fences => 18,
            Self::Photomode => 19,
            Self::CloneClutter => 20,
        }
    }
    /// 0x140a86b40: original mask 0x3cfe indexed by normalized outer tag.
    pub const fn is_tool_selected(self) -> bool {
        matches!(
            self,
            Self::Wall(_)
                | Self::Path
                | Self::Cobblestone(_)
                | Self::Eraser(_)
                | Self::Terrain
                | Self::Decorator(_)
                | Self::Color
                | Self::Water
                | Self::Tree
                | Self::Garden
                | Self::Fences
        )
    }
}
