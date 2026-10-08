//! Deterministic layout and mesh compilation, independent of engine assets.
pub mod mesh;
use garden_domain::{BlockId, Building, BuildingId, Facade, Placement, Roof};
use garden_geometry::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Face {
    Front,
    Back,
    Left,
    Right,
}

/// Stable semantic identity. Positions and dimensions are deliberately excluded.
/// `slot` counts from the center outwards; renderer can match old/new parts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartKey {
    pub building: BuildingId,
    pub block: BlockId,
    pub face: Face,
    pub story: u8,
    pub slot: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowPlacement {
    pub key: PartKey,
    pub along_wall: f64,
    pub elevation: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockLayout {
    pub block: BlockId,
    pub footprint: Rect,
    pub base_elevation: f64,
    pub height: f64,
    pub stories: u8,
    pub facade: Facade,
    pub effective_roof: Roof,
    pub terraces: Vec<Rect>,
    pub windows: Vec<WindowPlacement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingLayout {
    pub building: BuildingId,
    pub placement: Placement,
    pub blocks: Vec<BlockLayout>,
}

/// Current P0 layout rules are fixed, versioned together with the compiler.
/// A future configurable rule catalog must be captured into each async request.
pub const RULES_REVISION: u64 = 1;

pub fn compile(building: &Building) -> BuildingLayout {
    let mut blocks = building.blocks().iter().collect::<Vec<_>>();
    blocks.sort_by_key(|b| b.id);
    let blocks = blocks
        .into_iter()
        .map(|block| {
            let mut base = 0.0;
            let mut parent = block.parent;
            while let Some(id) = parent {
                let support = building.block(id).expect("validated parent");
                base += support.height;
                parent = support.parent;
            }
            let mut children = building
                .blocks()
                .iter()
                .filter(|b| b.parent == Some(block.id))
                .collect::<Vec<_>>();
            children.sort_by_key(|b| b.id);
            let effective_roof = if children.is_empty() {
                block.roof_intent
            } else {
                Roof::Flat
            };
            let terraces = if children.is_empty() {
                Vec::new()
            } else {
                let mut pieces = vec![block.footprint];
                for child in &children {
                    pieces = pieces
                        .into_iter()
                        .flat_map(|r| r.subtract(child.footprint))
                        .collect();
                }
                pieces
            };
            let mut windows = Vec::new();
            let stories = block.stories.count();
            let story_height = block.height / f64::from(stories);
            for face in [Face::Front, Face::Back, Face::Left, Face::Right] {
                let length = match face {
                    Face::Front | Face::Back => block.footprint.width,
                    _ => block.footprint.depth,
                };
                let bays = ((length - 0.8) / 2.1).floor().max(1.0) as u16;
                let spacing = (length - 0.8) / f64::from(bays);
                let mut centers = (0..bays)
                    .map(|i| 0.4 + (f64::from(i) + 0.5) * spacing)
                    .collect::<Vec<_>>();
                centers.sort_by(|a, b| {
                    (a - length / 2.0)
                        .abs()
                        .total_cmp(&(b - length / 2.0).abs())
                        .then(a.total_cmp(b))
                });
                for story in 0..stories {
                    for (slot, &along_wall) in centers.iter().enumerate() {
                        windows.push(WindowPlacement {
                            key: PartKey {
                                building: building.id(),
                                block: block.id,
                                face,
                                story,
                                slot: slot as u16,
                            },
                            along_wall,
                            elevation: base + (f64::from(story) + 0.5) * story_height,
                            width: 0.9f64.min(spacing * 0.7),
                            height: 1.2f64.min(story_height * 0.6),
                        });
                    }
                }
            }
            BlockLayout {
                block: block.id,
                footprint: block.footprint,
                base_elevation: base,
                height: block.height,
                stories,
                facade: block.facade,
                effective_roof,
                terraces,
                windows,
            }
        })
        .collect();
    BuildingLayout {
        building: building.id(),
        placement: building.placement(),
        blocks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::{BlockDraft, BuildingEdit, Stories, sample_building};
    fn building(w: f64, h: f64) -> Building {
        Building::try_new(sample_building(BuildingId::new(1).unwrap(), w, h)).unwrap()
    }
    #[test]
    fn resizing_changes_bays_and_stories_not_window_scale() {
        for (width, expected) in [(3.0, 1), (6.0, 2), (10.0, 4)] {
            let layout = compile(&building(width, 3.0));
            let front = layout.blocks[0]
                .windows
                .iter()
                .filter(|w| w.key.face == Face::Front)
                .collect::<Vec<_>>();
            assert_eq!(front.len(), expected);
            assert!(front.iter().all(|w| w.width <= 0.9));
        }
        for n in 1..=4 {
            let layout = compile(&building(10.0, f64::from(n) * 3.0));
            assert_eq!(layout.blocks[0].stories, n);
            assert_eq!(layout.blocks[0].windows.len(), usize::from(n) * 12);
        }
    }
    #[test]
    fn stacking_changes_roof_terrace_and_elevation_then_restores_intent() {
        let base = building(10.0, 3.0);
        let child = BlockDraft {
            id: BlockId::new(2).unwrap(),
            parent: Some(BlockId::new(1).unwrap()),
            footprint: Rect {
                x: 1.0,
                z: 1.0,
                width: 4.0,
                depth: 3.0,
            },
            height: 3.0,
            stories: Stories::Locked(1),
            roof_intent: Roof::Hipped,
            facade: Facade::Stone,
        };
        let stacked = base.edited(BuildingEdit::AddBlock(child)).unwrap();
        let layout = compile(&stacked);
        assert_eq!(layout.blocks[0].effective_roof, Roof::Flat);
        assert_eq!(
            layout.blocks[0]
                .terraces
                .iter()
                .map(|r| r.area())
                .sum::<f64>(),
            48.0
        );
        assert_eq!(layout.blocks[1].base_elevation, 3.0);
        assert_eq!(stacked.blocks()[0].roof_intent, Roof::Gabled);
        let restored = stacked
            .edited(BuildingEdit::RemoveBlock(BlockId::new(2).unwrap()))
            .unwrap();
        assert_eq!(compile(&restored), compile(&base));
    }
    #[test]
    fn compiler_is_deterministic_with_stable_part_keys() {
        let a = compile(&building(6.0, 3.0));
        assert_eq!(a, compile(&building(6.0, 3.0)));
        let b = compile(&building(7.0, 3.0));
        assert_eq!(
            a.blocks[0]
                .windows
                .iter()
                .map(|w| w.key)
                .collect::<Vec<_>>(),
            b.blocks[0]
                .windows
                .iter()
                .map(|w| w.key)
                .collect::<Vec<_>>()
        );
    }
}
