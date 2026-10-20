//! `create_rectangle::get_building_and_roof_colors` (0x140a94880) — the
//! building/roof colour selection policy shared by the circle and rectangle
//! creation systems.
//!
//! Fresh evidence (2026-10-17, `ghidra --worker shared`):
//! - decomp/disasm `planning/impl-walls-colors/decomp/0x140a94880-*`;
//! - call sites `create_rectangle` 0x140a93790 (0x140a93c8f) and
//!   `create_circle` 0x14097e560 (0x14097eaea).
//!
//! ## Output (sret `in_RCX`, caller buffer is 0x50 B; 0x1c B written)
//!
//! Disasm stores: `[out+0x18]=EAX`, `[out+0x11]=RAX`, `[out+0]=EBP`,
//! `[out+4]=R8D`, `[out+0xc]=R13B`, `[out+8]=R14D`, `[out+0x10]=BL`:
//!
//! | offset | field | source |
//! |---|---|---|
//! | +0x00 | building/wall colour id (u32) | `settings+0x1c`, else `wall_colors[prev]` |
//! | +0x04 | roof colour id (u32) | `settings+0x20`, else roof component `+0x50` |
//! | +0x08 | halftimber colour id (u32) | `settings+0x24`, else `halftimber_colors[prev]` |
//! | +0x0c | wall style (u8) | `settings+0x42`, else previous wall style |
//! | +0x10 | roof appearance[0] = component `+0x40` (flag2) | 0 default |
//! | +0x11..0x1c | appearance[1..12] = component `+0x41..0x4c` | zero default |
//!
//! This is exactly [`SelectedCreationAppearance`] (`wall_color_id`,
//! `roof_color_id`, `timber_color_id`, `wall_style`, `roof_appearance[12]`), so
//! that existing type contract is reused rather than duplicated.
//!
//! ## Native inputs
//!
//! | ABI | native | recovered model |
//! |---|---|---|
//! | RCX | sret `&mut [u8;0x50]` | return value |
//! | RDX | previous-option discriminant (bit 0 only) | `has_previous: bool` |
//! | R8 | previous `GlobalId` (u64 == wall id) | `previous: u64` |
//! | R9 | `&Vec<u32>` wall colour table | `wall_colors: &[u32]` |
//! | stack+0x28 | `&Vec<u32>` halftimber colour table | `halftimber_colors: &[u32]` |
//! | stack+0x30 | `&PublicWalls` (map A @+0, map B @+0x30) | `&impl PublicWallColorSource` |
//! | stack+0x38 | `&InnerWalls` dense roof component tables | `&RoofColorComponentTables` |
//! | stack+0x40 | `&GladeColorCustomization` defaults | `&GladeColorCustomization` |
//!
//! `settings+0x1c`/`+0x20`/`+0x24`/`+0x42` are `building`/`roof`/`halftimber`
//! and `building_wall_style` in [`GladeColorCustomization`].
//!
//! ## Control flow (fresh)
//!
//! 1. Load the four defaults; `if (tag & 1) == 0` emit defaults.
//! 2. Map A (`PublicWalls` records, stride 0x2a8) probe for `previous`.
//!    Miss → emit defaults. Hit → `point_count < 2 || !(length > 0.0)` is
//!    `core::panicking::panic()` (UCOMISS `JBE` also catches NaN).
//!    `shape_tag > 1` (`[record+0x260]`, shape params tag) → emit defaults.
//! 3. `building_color = wall_colors[previous]` and
//!    `halftimber_color = halftimber_colors[previous]`, both with
//!    `panic_bounds_check` on the vector length.
//! 4. Map B (`PublicWalls` roofs, 16 B entries) probe: miss → step 6.
//!    Hit value is an entity `u64`; `value >> 32 == 0` → step 6.
//! 5. Dense roof component chain (record `0x14`; generation check; presence
//!    bitset `(idx>>6,idx&0x3f)`; `0x48` chunk; `~slot*0x30`; leaf `0x58`).
//!    Success → roof colour id `+0x50`, appearance `+0x40..0x4c`.
//!    Every deep failure falls through to step 6 (never panics).
//! 6. Re-probe map A for the previous wall style; miss →
//!    `core::option::unwrap_failed()`.
//!
//! ## Representation notes / seams
//!
//! - `PublicWalls` is a HashMap abstraction in this crate; the native map probes
//!   are provided by [`PublicWallColorSource`], implemented for `PublicWalls`
//!   via `get_unchecked` (raw record, matching the native unvalidated probe) and
//!   `roof_entity` (entity bits, generation in the high 32 bits).
//! - The dense tables live inside native `InnerWalls` (`+0x1a8`, bitset
//!   `+0x28/+0x38`, `+0x110` slot) and are **not** modelled by the recovered
//!   `InnerWalls` (walls group in flight). [`RoofColorComponentTables`] is the
//!   recovered close-out input; its producer is registered as a seam.
//! - Native reads inside the dense chain are unchecked raw loads. The recovered
//!   model is bounds-checked and routes an out-of-range read to the same "miss"
//!   path; for valid tables behaviour is identical (safe superset registered).

use crate::resources::glade_color_customization::GladeColorCustomization;
use crate::resources::walls::{WallId, public_walls::PublicWalls};
use crate::systems::wall::shapes::authoring::SelectedCreationAppearance;

/// One map A (`PublicWalls` records) row as consumed by the colour policy.
/// Native record stride 0x2a8; fields read at `+0x18`, `+0x38`, `+0x260`,
/// `+0x2a1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PublicWallColorHost {
    /// Native `+0x18` simplified Curve2 point count.
    pub point_count: u64,
    /// Native `+0x38` simplified Curve2 length.
    pub curve_length: f32,
    /// Native `+0x260` shape-parameters tag (`> 1` means no colour derivation).
    pub shape_tag: u32,
    /// Native `+0x2a1` wall style byte.
    pub wall_style: u8,
}

/// Native `PublicWalls` lookups used by the colour policy: map A records and
/// map B roof entities (16 B entries, key `GlobalId` → `Entity` bits).
pub trait PublicWallColorSource {
    /// Unvalidated map A probe. `None` on a missing row; validation lives in the
    /// caller so the native panic order is preserved.
    fn color_host(&self, wall: u64) -> Option<PublicWallColorHost>;
    /// Map B probe returning packed entity bits (`index | generation << 32`).
    fn roof_entity_bits(&self, wall: u64) -> Option<u64>;
}

impl PublicWallColorSource for PublicWalls {
    fn color_host(&self, wall: u64) -> Option<PublicWallColorHost> {
        let state = self.get_unchecked(WallId(wall))?;
        let curve = state.simplified_curve_2d();
        Some(PublicWallColorHost {
            point_count: curve.points.len() as u64,
            curve_length: curve.length,
            shape_tag: state.authority.source().shape.tag(),
            wall_style: state.authority.source().style,
        })
    }

    fn roof_entity_bits(&self, wall: u64) -> Option<u64> {
        self.roof_entity(WallId(wall))
            .map(|entity| entity.to_bits())
    }
}

/// Native dense roof-component record, stride 0x14 (`records` vector at the
/// store's `+0x8`/`+0x10`). Offsets: generation `+0x0`, presence bit `+0x4`,
/// unobserved `+0x8`, chunk key `+0xc`, component index `+0x10`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoofSparseRecord {
    pub generation: u32,
    pub bit_index: u32,
    /// Native `+0x8`; no consumer observed in this function.
    pub unobserved_8: u32,
    pub chunk_key: u32,
    pub element_index: u32,
}

/// Native roof presence bitset (`+0x28` words pointer, `+0x38` capacity) plus
/// the `+0x110` slot index used by the chunk reverse table.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoofPresenceBitset {
    /// Word array (native `+0x28`).
    pub words: Vec<u64>,
    /// Number of addressable bits (native `+0x38`); `bit_index >= bit_len` is a
    /// miss, not a panic.
    pub bit_len: usize,
    /// Native `+0x110` index into the selected chunk's `reverse` table.
    pub slot: u64,
}

/// One `0x30`-stride chunk entry (native array pointer at chunk `+0x18`).
/// Only `+0x10` (pointer to the `0x58`-stride colour array) is consumed.
#[derive(Clone, Debug, Default)]
pub struct RoofChunkEntry {
    /// Native `+0x0..0x10`; not consumed here.
    pub unobserved_0: [u8; 0x10],
    /// Native `+0x10` pointer to the `0x58`-stride colour array. The recovered
    /// model stores the array inline (representation difference only).
    pub colors: Vec<RoofColorEntry>,
}

/// One `0x58`-stride roof colour component. Only the appearance block
/// (`+0x40..0x4c`) and colour id (`+0x50`) are consumed.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoofColorEntry {
    /// Native `+0x0..0x40`; not consumed here.
    pub unobserved_0: [u8; 0x40],
    /// Native `+0x40..0x4c`: `flag2` + the 11-byte appearance payload.
    pub appearance: [u8; 12],
    /// Native `+0x4c..0x50`; not consumed here.
    pub unobserved_4c: [u8; 4],
    /// Native `+0x50` roof colour id.
    pub color_id: u32,
    /// Native `+0x54..0x58`; not consumed here.
    pub unobserved_54: [u8; 4],
}

impl Default for RoofColorEntry {
    fn default() -> Self {
        Self {
            unobserved_0: [0; 0x40],
            appearance: [0; 12],
            unobserved_4c: [0; 4],
            color_id: 0,
            unobserved_54: [0; 4],
        }
    }
}

impl RoofColorEntry {
    /// Native `+0x30` roof pitch `height_01` (fresh `create_rectangle`
    /// 0x140a93790, `MOVSS XMM0,xmmword [RCX+RAX+0x30]`). The old-roof
    /// inheritance replaces the rectangle height when `pitch <= 0.1`.
    pub fn pitch_01(&self) -> f32 {
        f32::from_le_bytes(self.unobserved_0[0x30..0x34].try_into().unwrap())
    }

    /// Native `+0x3d` roof style byte (fresh `create_rectangle` 0x140a93790,
    /// `MOVZX EBX,byte [RCX+RAX+0x3d]`).
    pub fn style(&self) -> u8 {
        self.unobserved_0[0x3d]
    }
}

/// One `0x48`-stride chunk. Consumers read the `+0x18` entry array and the
/// `+0x38` reverse-index array.
#[derive(Clone, Debug, Default)]
pub struct RoofChunk {
    /// Native `+0x0..0x18`; not consumed here.
    pub unobserved_0: [u8; 0x18],
    /// Native `+0x18` pointer to the `0x30`-stride entry array.
    pub entries: Vec<RoofChunkEntry>,
    /// Native `+0x20..0x38`; not consumed here.
    pub unobserved_20: [u8; 0x18],
    /// Native `+0x38` pointer to the reverse-index array.
    pub reverse: Vec<u64>,
    /// Native `+0x40..0x48`; not consumed here.
    pub unobserved_40: [u8; 8],
}

/// Recovered input for the native dense tables embedded in `InnerWalls`
/// (`+0x8` record vector, `+0x1a8` chunk table, `[0]` presence bitset).
///
/// Seam: the recovered `InnerWalls` does not own these tables; a producer that
/// fills this resource from the real roof/component storage is outstanding.
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct RoofColorComponentTables {
    pub presence: RoofPresenceBitset,
    pub records: Vec<RoofSparseRecord>,
    pub chunks: Vec<RoofChunk>,
}

impl RoofColorComponentTables {
    /// `map B value → record (+0x14) → generation → bitset gate → chunk (+0x48)
    /// → reverse slot → ~slot*0x30 entry → +0x10 leaf array → +index*0x58`.
    /// Every failure is a miss; the native function never panics here.
    pub fn lookup(&self, entity_bits: u64) -> Option<&RoofColorEntry> {
        let index = (entity_bits & 0xffff_ffff) as usize;
        let generation = (entity_bits >> 32) as u32;
        let record = self.records.get(index)?;
        if record.generation != generation {
            return None;
        }
        let bit = record.bit_index as usize;
        if bit >= self.presence.bit_len {
            return None;
        }
        // Native `BT word, bit` takes the bit offset modulo 64.
        let word = *self.presence.words.get(bit >> 6)?;
        if (word >> (bit & 0x3f)) & 1 == 0 {
            return None;
        }
        // Native chunk / reverse / leaf reads are unchecked; the recovered
        // model routes an out-of-range read to the same miss path.
        let chunk = self.chunks.get(record.chunk_key as usize)?;
        let reverse = *chunk.reverse.get(self.presence.slot as usize)?;
        let entry = chunk.entries.get((!reverse) as usize)?;
        entry.colors.get(record.element_index as usize)
    }
}

/// `0x140a94880 get_building_and_roof_colors`.
///
/// Native panic sites preserved: map A validation (`core::panicking::panic`),
/// colour-vector bounds (`panic_bounds_check`), and the final map A re-probe
/// (`core::option::unwrap_failed`).
///
/// Divergence-free on branch conditions; the only representation differences
/// are the safe-superset bounds routing inside the dense chain (see module doc).
pub fn get_building_and_roof_colors(
    has_previous: bool,
    previous: u64,
    wall_colors: &[u32],
    halftimber_colors: &[u32],
    public: &impl PublicWallColorSource,
    tables: &RoofColorComponentTables,
    settings: &GladeColorCustomization,
) -> SelectedCreationAppearance {
    let mut selected = SelectedCreationAppearance {
        wall_color_id: settings.building.0,
        roof_color_id: settings.roof.0,
        timber_color_id: settings.halftimber.0,
        wall_style: settings.building_wall_style,
        roof_appearance: [0; 12],
    };

    if !has_previous {
        return selected;
    }
    let Some(host) = public.color_host(previous) else {
        return selected;
    };
    // Native 0x140a94c7f / 0x140a94c97: `core::panicking::panic()`.
    if host.point_count < 2 || !(host.curve_length > 0.0) {
        panic!("assertion failed: w.simplified_curve_2d.is_valid()");
    }
    if host.shape_tag > 1 {
        // Native restores the default c1 and emits with flag2 = 0.
        return selected;
    }

    // Native 0x140a94caf / 0x140a94cbe: `panic_bounds_check`.
    selected.wall_color_id = wall_colors[previous as usize];
    selected.timber_color_id = halftimber_colors[previous as usize];

    if let Some(entity_bits) = public.roof_entity_bits(previous) {
        // Native skips empty map B and treats generation 0 as a miss.
        if entity_bits >> 32 != 0 {
            if let Some(component) = tables.lookup(entity_bits) {
                selected.roof_color_id = component.color_id;
                selected.roof_appearance = component.appearance;
            }
        }
    }

    // Native 0x140a94c73: `core::option::unwrap_failed()`. The map A row was
    // present above, so this only triggers on a concurrently mutated store.
    let host = public
        .color_host(previous)
        .expect("called `Option::unwrap()` on a `None` value");
    selected.wall_style = host.wall_style;
    selected
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::color_id::ColorId;

    #[derive(Default)]
    struct TestWalls {
        hosts: std::collections::HashMap<u64, PublicWallColorHost>,
        roofs: std::collections::HashMap<u64, u64>,
    }

    impl PublicWallColorSource for TestWalls {
        fn color_host(&self, wall: u64) -> Option<PublicWallColorHost> {
            self.hosts.get(&wall).copied()
        }
        fn roof_entity_bits(&self, wall: u64) -> Option<u64> {
            self.roofs.get(&wall).copied()
        }
    }

    fn host(tag: u32, style: u8) -> PublicWallColorHost {
        PublicWallColorHost {
            point_count: 4,
            curve_length: 2.0,
            shape_tag: tag,
            wall_style: style,
        }
    }

    fn settings() -> GladeColorCustomization {
        GladeColorCustomization {
            building: ColorId(11),
            roof: ColorId(12),
            halftimber: ColorId(13),
            building_wall_style: 14,
            ..Default::default()
        }
    }

    fn tables_with(entity_bits: u64, color: u32, appearance: [u8; 12]) -> RoofColorComponentTables {
        let record = RoofSparseRecord {
            generation: (entity_bits >> 32) as u32,
            bit_index: 3,
            unobserved_8: 0,
            chunk_key: 1,
            element_index: 2,
        };
        RoofColorComponentTables {
            presence: RoofPresenceBitset {
                words: vec![1 << 3],
                bit_len: 64,
                slot: 1,
            },
            records: vec![record],
            chunks: vec![
                RoofChunk::default(),
                RoofChunk {
                    entries: vec![RoofChunkEntry {
                        unobserved_0: [0; 0x10],
                        colors: vec![
                            RoofColorEntry::default(),
                            RoofColorEntry::default(),
                            RoofColorEntry {
                                appearance,
                                color_id: color,
                                ..Default::default()
                            },
                        ],
                    }],
                    // ~1 == u64::MAX - 1: native indexes the entry array with
                    // the bitwise complement, so store the complemented index.
                    reverse: vec![0, !0u64],
                    ..Default::default()
                },
            ],
        }
    }

    #[test]
    fn no_previous_building_uses_all_four_defaults() {
        let out = get_building_and_roof_colors(
            false,
            7,
            &[],
            &[],
            &TestWalls::default(),
            &RoofColorComponentTables::default(),
            &settings(),
        );
        assert_eq!(out.wall_color_id, 11);
        assert_eq!(out.roof_color_id, 12);
        assert_eq!(out.timber_color_id, 13);
        assert_eq!(out.wall_style, 14);
        assert_eq!(out.roof_appearance, [0; 12]);
    }

    #[test]
    fn missing_previous_host_uses_defaults() {
        let out = get_building_and_roof_colors(
            true,
            7,
            &[],
            &[],
            &TestWalls::default(),
            &RoofColorComponentTables::default(),
            &settings(),
        );
        assert_eq!((out.wall_color_id, out.timber_color_id), (11, 13));
        assert_eq!(out.wall_style, 14);
    }

    #[test]
    fn shape_tag_above_one_skips_colour_tables() {
        let mut walls = TestWalls::default();
        walls.hosts.insert(7, host(2, 9));
        let out = get_building_and_roof_colors(
            true,
            7,
            &[],
            &[],
            &walls,
            &RoofColorComponentTables::default(),
            &settings(),
        );
        // Native returns the defaults without touching either colour vector.
        assert_eq!((out.wall_color_id, out.timber_color_id), (11, 13));
        assert_eq!(out.roof_color_id, 12);
        assert_eq!(out.wall_style, 14);
    }

    #[test]
    fn map_a_hit_inherits_colours_and_style_when_map_b_misses() {
        let mut walls = TestWalls::default();
        walls.hosts.insert(2, host(1, 4));
        let out = get_building_and_roof_colors(
            true,
            2,
            &[0, 0, 21, 22],
            &[0, 0, 31, 32],
            &walls,
            &RoofColorComponentTables::default(),
            &settings(),
        );
        assert_eq!(out.wall_color_id, 21);
        assert_eq!(out.timber_color_id, 31);
        assert_eq!(out.roof_color_id, 12);
        assert_eq!(out.wall_style, 4);
        assert_eq!(out.roof_appearance, [0; 12]);
    }

    #[test]
    fn roof_component_overrides_roof_colour_and_appearance() {
        let mut walls = TestWalls::default();
        walls.hosts.insert(2, host(1, 4));
        let entity = (1u64 << 32) | 0;
        walls.roofs.insert(2, entity);
        let appearance = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let tables = tables_with(entity, 91, appearance);
        let out = get_building_and_roof_colors(
            true,
            2,
            &[0, 0, 21, 22],
            &[0, 0, 31, 32],
            &walls,
            &tables,
            &settings(),
        );
        assert_eq!(out.roof_color_id, 91);
        assert_eq!(out.roof_appearance, appearance);
        // Style still comes from the map A re-probe.
        assert_eq!(out.wall_style, 4);
    }

    #[test]
    fn generation_zero_and_deep_misses_fall_back_to_style_inheritance() {
        let mut walls = TestWalls::default();
        walls.hosts.insert(2, host(1, 4));
        walls.roofs.insert(2, 0); // generation 0 → miss
        let out = get_building_and_roof_colors(
            true,
            2,
            &[0, 0, 21],
            &[0, 0, 31],
            &walls,
            &RoofColorComponentTables::default(),
            &settings(),
        );
        assert_eq!(out.roof_color_id, 12);
        assert_eq!(out.wall_style, 4);

        // Bitset gate cleared → miss.
        let entity = (1u64 << 32) | 0;
        let mut tables = tables_with(entity, 91, [9; 12]);
        tables.presence.words[0] = 0;
        let out = get_building_and_roof_colors(
            true,
            2,
            &[0, 0, 21],
            &[0, 0, 31],
            &walls,
            &tables,
            &settings(),
        );
        assert_eq!(out.roof_color_id, 12);
        assert_eq!(out.roof_appearance, [0; 12]);

        // Generation mismatch → miss.
        let mut tables = tables_with(entity, 91, [9; 12]);
        tables.records[0].generation = 2;
        let out = get_building_and_roof_colors(
            true,
            2,
            &[0, 0, 21],
            &[0, 0, 31],
            &walls,
            &tables,
            &settings(),
        );
        assert_eq!(out.roof_color_id, 12);
    }

    #[test]
    #[should_panic]
    fn invalid_previous_wall_panics() {
        let mut walls = TestWalls::default();
        walls.hosts.insert(
            2,
            PublicWallColorHost {
                point_count: 1,
                curve_length: 2.0,
                shape_tag: 1,
                wall_style: 0,
            },
        );
        let _ = get_building_and_roof_colors(
            true,
            2,
            &[0, 0, 21],
            &[0, 0, 31],
            &walls,
            &RoofColorComponentTables::default(),
            &settings(),
        );
    }

    #[test]
    #[should_panic]
    fn nan_previous_wall_length_panics() {
        let mut walls = TestWalls::default();
        walls.hosts.insert(
            2,
            PublicWallColorHost {
                point_count: 4,
                curve_length: f32::NAN,
                shape_tag: 1,
                wall_style: 0,
            },
        );
        let _ = get_building_and_roof_colors(
            true,
            2,
            &[0, 0, 21],
            &[0, 0, 31],
            &walls,
            &RoofColorComponentTables::default(),
            &settings(),
        );
    }

    #[test]
    #[should_panic]
    fn colour_table_bounds_panic() {
        let mut walls = TestWalls::default();
        walls.hosts.insert(2, host(1, 4));
        let _ = get_building_and_roof_colors(
            true,
            2,
            &[0, 0],
            &[0, 0, 31],
            &walls,
            &RoofColorComponentTables::default(),
            &settings(),
        );
    }
}
