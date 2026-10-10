//! Authoritative architectural intent. No Bevy, rendering handles, or OS services.
pub mod context;
pub mod strokes;
pub mod terrain;
use garden_geometry::Rect;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    num::NonZeroU64,
};

macro_rules! id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(NonZeroU64);
        impl $name {
            pub fn new(value: u64) -> Option<Self> {
                NonZeroU64::new(value).map(Self)
            }
            pub fn get(self) -> u64 {
                self.0.get()
            }
        }
    };
}
id!(BuildingId);
id!(BlockId);

pub const MAX_BLOCKS: usize = 3;
pub const MAX_STORIES: u8 = 4;
pub const SUPPORT_MARGIN: f64 = 0.15;
pub const MIN_STORY_HEIGHT: f64 = 2.2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Roof {
    Gabled,
    Hipped,
    Flat,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facade {
    Stone,
    Plaster,
    Timber,
}

/// Resolved automatic count is persisted intent, not re-derived on load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stories {
    Auto { resolved: u8 },
    Locked(u8),
}
impl Stories {
    pub fn count(self) -> u8 {
        match self {
            Self::Auto { resolved } => resolved,
            Self::Locked(n) => n,
        }
    }
    pub fn resized(self, height: f64) -> Self {
        match self {
            Self::Locked(_) => self,
            Self::Auto { resolved } => {
                let mut n = resolved;
                // A 0.2m dead band around each half-story threshold prevents chatter.
                while n < MAX_STORIES && height > (f64::from(n) + 0.5) * 3.0 + 0.2 {
                    n += 1;
                }
                while n > 1
                    && (height < (f64::from(n) - 0.5) * 3.0 - 0.2
                        || height < f64::from(n) * MIN_STORY_HEIGHT)
                {
                    n -= 1;
                }
                Self::Auto { resolved: n }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Placement {
    pub x: f64,
    pub z: f64,
    pub elevation: f64,
    pub yaw: f64,
}

/// Untrusted input DTO: importers and UI must pass through Building::try_new.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockDraft {
    pub id: BlockId,
    pub parent: Option<BlockId>,
    pub footprint: Rect,
    pub height: f64,
    pub stories: Stories,
    pub roof_intent: Roof,
    pub facade: Facade,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingDraft {
    pub id: BuildingId,
    pub placement: Placement,
    pub blocks: Vec<BlockDraft>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Building {
    draft: BuildingDraft,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainError(pub &'static str);
impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for DomainError {}

#[derive(Debug, Clone)]
pub enum BuildingEdit {
    Resize {
        block: BlockId,
        footprint: Rect,
        height: f64,
    },
    SetRoof {
        block: BlockId,
        roof: Roof,
    },
    SetStories {
        block: BlockId,
        stories: Stories,
    },
    SetFacade {
        block: BlockId,
        facade: Facade,
    },
    AddBlock(BlockDraft),
    RemoveBlock(BlockId),
    /// Absolute local position; descendants keep their relative placement.
    TranslateBlock {
        block: BlockId,
        x: f64,
        z: f64,
    },
    Move(Placement),
}

impl Building {
    pub fn try_new(draft: BuildingDraft) -> Result<Self, DomainError> {
        validate(&draft)?;
        Ok(Self { draft })
    }
    pub fn id(&self) -> BuildingId {
        self.draft.id
    }
    pub fn placement(&self) -> Placement {
        self.draft.placement
    }
    pub fn blocks(&self) -> &[BlockDraft] {
        &self.draft.blocks
    }
    pub fn to_draft(&self) -> BuildingDraft {
        self.draft.clone()
    }
    pub fn block(&self, id: BlockId) -> Option<&BlockDraft> {
        self.blocks().iter().find(|b| b.id == id)
    }

    /// Transactional aggregate edit: rejection leaves the original unchanged.
    pub fn edited(&self, edit: BuildingEdit) -> Result<Self, DomainError> {
        let mut draft = self.to_draft();
        match edit {
            BuildingEdit::AddBlock(block) => draft.blocks.push(block),
            BuildingEdit::Move(placement) => draft.placement = placement,
            BuildingEdit::TranslateBlock { block, x, z } => {
                let original = self.block(block).ok_or(DomainError("unknown block"))?;
                let dx = x - original.footprint.x;
                let dz = z - original.footprint.z;
                let moved = descendants(&draft.blocks, block);
                for b in &mut draft.blocks {
                    if moved.contains(&b.id) {
                        b.footprint.x += dx;
                        b.footprint.z += dz;
                    }
                }
            }
            BuildingEdit::RemoveBlock(id) => {
                if self.block(id).is_none() {
                    return Err(DomainError("unknown block"));
                }
                let removed = descendants(&draft.blocks, id);
                draft.blocks.retain(|b| !removed.contains(&b.id));
            }
            edit => {
                let id = match &edit {
                    BuildingEdit::Resize { block, .. }
                    | BuildingEdit::SetRoof { block, .. }
                    | BuildingEdit::SetStories { block, .. }
                    | BuildingEdit::SetFacade { block, .. } => *block,
                    _ => unreachable!(),
                };
                let block = draft
                    .blocks
                    .iter_mut()
                    .find(|b| b.id == id)
                    .ok_or(DomainError("unknown block"))?;
                match edit {
                    BuildingEdit::Resize {
                        footprint, height, ..
                    } => {
                        block.footprint = footprint;
                        block.height = height;
                        block.stories = block.stories.resized(height);
                    }
                    BuildingEdit::SetRoof { roof, .. } => block.roof_intent = roof,
                    BuildingEdit::SetStories { stories, .. } => block.stories = stories,
                    BuildingEdit::SetFacade { facade, .. } => block.facade = facade,
                    _ => unreachable!(),
                }
            }
        }
        Self::try_new(draft)
    }
}

fn descendants(blocks: &[BlockDraft], root: BlockId) -> BTreeSet<BlockId> {
    let mut ids = BTreeSet::from([root]);
    loop {
        let count = ids.len();
        for b in blocks {
            if b.parent.is_some_and(|id| ids.contains(&id)) {
                ids.insert(b.id);
            }
        }
        if count == ids.len() {
            return ids;
        }
    }
}

fn validate(d: &BuildingDraft) -> Result<(), DomainError> {
    let reject = |s| Err(DomainError(s));
    let p = d.placement;
    if ![p.x, p.z, p.elevation, p.yaw]
        .into_iter()
        .all(f64::is_finite)
    {
        return reject("non-finite placement");
    }
    if d.blocks.is_empty() || d.blocks.len() > MAX_BLOCKS {
        return reject("building requires 1..=3 blocks");
    }
    let by_id: BTreeMap<_, _> = d.blocks.iter().map(|b| (b.id, b)).collect();
    if by_id.len() != d.blocks.len() {
        return reject("duplicate block id");
    }
    if d.blocks.iter().filter(|b| b.parent.is_none()).count() != 1 {
        return reject("exactly one root required");
    }
    for block in &d.blocks {
        let n = block.stories.count();
        if !block.footprint.valid()
            || !(1.2..=100.0).contains(&block.footprint.width)
            || !(1.2..=100.0).contains(&block.footprint.depth)
        {
            return reject("invalid footprint");
        }
        if !(1..=MAX_STORIES).contains(&n)
            || !block.height.is_finite()
            || block.height < f64::from(n) * MIN_STORY_HEIGHT
            || block.height > 16.0
        {
            return reject("invalid story height/count");
        }
        if let Some(parent) = block.parent {
            let base = by_id
                .get(&parent)
                .ok_or(DomainError("missing support parent"))?;
            if !base.footprint.contains(block.footprint, SUPPORT_MARGIN) {
                return reject("upper block outside support margin");
            }
        }
        let mut visited = BTreeSet::new();
        let mut cursor = Some(block.id);
        let mut total = 0u8;
        while let Some(id) = cursor {
            if !visited.insert(id) {
                return reject("support cycle");
            }
            let ancestor = by_id.get(&id).ok_or(DomainError("missing parent"))?;
            total += ancestor.stories.count();
            cursor = ancestor.parent;
        }
        if total > MAX_STORIES {
            return reject("support chain exceeds four stories");
        }
    }
    for (i, a) in d.blocks.iter().enumerate() {
        for b in &d.blocks[i + 1..] {
            if a.parent == b.parent && a.footprint.intersects(b.footprint) {
                return reject("siblings overlap");
            }
        }
    }
    Ok(())
}

/// Small reusable fixture for examples/tests; not a shipping asset or game preset.
pub fn sample_building(id: BuildingId, width: f64, height: f64) -> BuildingDraft {
    BuildingDraft {
        id,
        placement: Placement::default(),
        blocks: vec![BlockDraft {
            id: BlockId::new(1).unwrap(),
            parent: None,
            footprint: Rect {
                x: 0.0,
                z: 0.0,
                width,
                depth: 6.0,
            },
            height,
            stories: Stories::Auto { resolved: 1 }.resized(height),
            roof_intent: Roof::Gabled,
            facade: Facade::Plaster,
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn translating_a_supported_block_moves_its_descendants_atomically() {
        let mut d = draft();
        let mut middle = child();
        middle.footprint.width = 6.;
        middle.footprint.depth = 4.;
        let mut top = child();
        top.id = BlockId::new(3).unwrap();
        top.parent = Some(middle.id);
        top.footprint = Rect {
            x: 2.,
            z: 2.,
            width: 2.,
            depth: 2.,
        };
        d.blocks.extend([middle, top]);
        let original = Building::try_new(d).unwrap();
        let moved = original
            .edited(BuildingEdit::TranslateBlock {
                block: BlockId::new(2).unwrap(),
                x: 1.5,
                z: 1.2,
            })
            .unwrap();
        assert_eq!(moved.blocks()[0], original.blocks()[0]);
        assert_eq!(
            moved.block(BlockId::new(3).unwrap()).unwrap().footprint.x,
            2.5
        );
        assert!((moved.block(BlockId::new(3).unwrap()).unwrap().footprint.z - 2.2).abs() < 1e-8);
        assert!(
            original
                .edited(BuildingEdit::TranslateBlock {
                    block: BlockId::new(2).unwrap(),
                    x: 8.,
                    z: 1.
                })
                .is_err()
        );
        assert!(
            original
                .edited(BuildingEdit::TranslateBlock {
                    block: BlockId::new(2).unwrap(),
                    x: f64::NAN,
                    z: 1.
                })
                .is_err()
        );
        assert_eq!(
            original
                .block(BlockId::new(2).unwrap())
                .unwrap()
                .footprint
                .x,
            1.
        );
    }
    #[test]
    fn three_level_support_tree_removes_subtree_without_mutating_original() {
        let mut d = draft();
        let mut middle = child();
        middle.footprint.width = 6.0;
        middle.footprint.depth = 4.0;
        let mut top = child();
        top.id = BlockId::new(3).unwrap();
        top.parent = Some(middle.id);
        top.footprint = Rect {
            x: 2.0,
            z: 2.0,
            width: 2.0,
            depth: 2.0,
        };
        d.blocks.extend([middle, top]);
        let building = Building::try_new(d).unwrap();
        let removed = building
            .edited(BuildingEdit::RemoveBlock(BlockId::new(2).unwrap()))
            .unwrap();
        assert_eq!(removed.blocks().len(), 1);
        assert_eq!(building.blocks().len(), 3);
    }
    fn draft() -> BuildingDraft {
        sample_building(BuildingId::new(1).unwrap(), 10.0, 3.0)
    }
    fn child() -> BlockDraft {
        BlockDraft {
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
        }
    }
    #[test]
    fn automatic_stories_have_hysteresis() {
        let one = Stories::Auto { resolved: 1 };
        assert_eq!(one.resized(4.6).count(), 1);
        assert_eq!(one.resized(4.8).count(), 2);
        assert_eq!(one.resized(4.8).resized(4.4).count(), 2);
        assert_eq!(one.resized(4.8).resized(4.2).count(), 1);
        assert_eq!(one.resized(4.8).resized(4.35).count(), 1);
        for n in 1..=4 {
            assert_eq!(one.resized(f64::from(n) * 3.0).count(), n);
        }
    }
    #[test]
    fn rejects_invalid_values_and_duplicate_ids() {
        let mut d = draft();
        d.blocks[0].height = f64::NAN;
        assert!(Building::try_new(d).is_err());
        let mut d = draft();
        d.blocks.push(d.blocks[0].clone());
        assert!(Building::try_new(d).is_err());
        let mut d = draft();
        d.blocks[0].stories = Stories::Locked(0);
        assert!(Building::try_new(d).is_err());
    }
    #[test]
    fn support_and_edit_are_atomic() {
        let original = Building::try_new(draft()).unwrap();
        let stacked = original.edited(BuildingEdit::AddBlock(child())).unwrap();
        let bad = BuildingEdit::Resize {
            block: BlockId::new(1).unwrap(),
            footprint: Rect {
                width: 3.0,
                ..original.blocks()[0].footprint
            },
            height: 3.0,
        };
        assert!(stacked.edited(bad).is_err());
        assert_eq!(stacked.blocks()[0].footprint.width, 10.0);
        assert_eq!(
            stacked
                .edited(BuildingEdit::RemoveBlock(BlockId::new(2).unwrap()))
                .unwrap(),
            original
        );
    }
    #[test]
    fn rejects_overlapping_siblings_and_excess_stories() {
        let mut d = draft();
        d.blocks.push(child());
        let mut c = child();
        c.id = BlockId::new(3).unwrap();
        d.blocks.push(c);
        assert!(Building::try_new(d).is_err());
        let mut d = sample_building(BuildingId::new(1).unwrap(), 10.0, 12.0);
        d.blocks.push(child());
        assert!(Building::try_new(d).is_err());
    }
}
