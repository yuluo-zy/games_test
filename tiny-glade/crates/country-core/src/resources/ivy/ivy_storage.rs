//! B 0x140869460..0x140869941: owned polyline, root and spawn state.
use bevy_ecs::prelude::Resource;
use glam::Vec3;
use perchance::PerchanceContext;
use std::collections::{HashMap, VecDeque};

pub type IvyId = u64;
pub type IvyPointId = u64;

/// Native point stride 40. The +32 scalar is animated branch thickness, not
/// reserved padding; add_point initializes it to zero.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IvyPoint {
    pub id: IvyPointId,
    pub position: Vec3,
    pub direction: Vec3,
    pub thickness: f32,
}

/// Native +68 discriminant 1 holds two Vec3s at +6c/+78. B 0x14094b1e8
/// interpolates them into the current tail point; 0x14094bf85 initializes both.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IvyGrowthSegment {
    pub from: Vec3,
    pub to: Vec3,
}

#[derive(Clone, Debug)]
pub struct Ivy {
    pub points: VecDeque<IvyPoint>,
    pub id: IvyId,
    /// Original +28 root copy remains separate from the mutable deque front.
    pub root: IvyPoint,
    pub maximum_point_count: usize,
    pub next_point_id: IvyPointId,
    /// Original +60 is forwarded unchanged to leaf +18.
    pub source_60: u64,
    pub pending_growth: Option<IvyGrowthSegment>,
    /// Native +84, interpolation progress for the pending segment.
    pub growth_progress: f32,
    /// Native +88: the per-ivy growth cooldown. The grower decrements it by
    /// `time.delta` while it exceeds 0.001 (0x1429cd060) and skips the ivy
    /// (0x14094aea9..0x14094adf5); `spawn_ivy` seeds it from the scalar range.
    pub growth_delay: f32,
    pub flag_8c: u8,
}

impl Ivy {
    pub fn add_point(&mut self, position: Vec3, direction: Vec3) -> IvyPointId {
        self.next_point_id = self.next_point_id.wrapping_add(1);
        let id = self.next_point_id;
        self.points.push_back(IvyPoint {
            id,
            position,
            direction,
            thickness: 0.0,
        });
        id
    }
    pub fn point_count(&self) -> usize {
        self.points.len()
    }
    /// Native empty VecDeque unwrap is a panic, not an optional last point.
    pub fn last_point(&self) -> &IvyPoint {
        self.points.back().expect("ivy has no last point")
    }
    pub fn last_segment_point_ids(&self) -> (IvyPointId, IvyPointId) {
        let n = self.points.len();
        assert!(n >= 2, "not enough ivy points");
        (self.points[n - 2].id, self.points[n - 1].id)
    }
    /// ID lookup scans logical deque order; IDs cease to be indices after a
    /// root or tail removal. B leaf animation uses precisely this lookup.
    pub fn get_point_id(&self, id: IvyPointId) -> Option<&IvyPoint> {
        self.points.iter().find(|point| point.id == id)
    }
    pub fn set_last_point_position(&mut self, position: Vec3) {
        self.points
            .back_mut()
            .expect("ivy has no last point")
            .position = position;
    }
    pub fn remove_last_point(&mut self) -> Option<IvyPoint> {
        self.points.pop_back()
    }
    pub fn remove_root(&mut self) -> Option<IvyPoint> {
        self.points.pop_front()
    }
}

#[derive(Resource, Clone)]
pub struct IvyStorage {
    ivies: HashMap<IvyId, Ivy>,
    rng: PerchanceContext,
    next_ivy_id: IvyId,
    /// Native new() byte at +48 is 1. The grower's guard phase (0x14094c09f)
    /// toggles bit0 each frame and runs the growth closure only while the bit
    /// is set, so growth advances every other frame.
    pub flag_48: u8,
}
impl Default for IvyStorage {
    fn default() -> Self {
        Self::new()
    }
}
impl IvyStorage {
    /// B insert_ivy_resources writes RNG state [1,0] = perchance seed 0,
    /// counter 0 and final byte 1. The dependency owns the RNG algorithm.
    pub fn new() -> Self {
        Self::with_seed(0)
    }
    pub fn with_seed(seed: u128) -> Self {
        Self {
            ivies: HashMap::new(),
            rng: PerchanceContext::new(seed),
            next_ivy_id: 0,
            flag_48: 1,
        }
    }
    pub fn next_id(&self) -> IvyId {
        self.next_ivy_id
    }
    pub(crate) fn rng_mut(&mut self) -> &mut PerchanceContext {
        &mut self.rng
    }
    /// Native RNG call order is count then float. Range end is exclusive.
    /// Root point ID is 0; next_point_id is initialized to 1, so add_point's
    /// increment produces 2. The map uses its old counter before incrementing.
    pub fn spawn_ivy(
        &mut self,
        position: Vec3,
        direction: Vec3,
        scalar_range: std::ops::Range<f32>,
        count_range: std::ops::Range<usize>,
        source_60: u64,
    ) -> IvyId {
        assert!(
            count_range.start <= count_range.end,
            "invalid ivy point-count range"
        );
        let maximum_point_count = self
            .rng
            .usize_less_than(count_range.end - count_range.start)
            + count_range.start;
        // Native +88 growth cooldown: the grower waits out this many seconds
        // (0x14094adf5: `delay -= delta` while `delay > 0.001`) before the ivy
        // starts growing.
        let growth_delay =
            (scalar_range.end - scalar_range.start) * self.rng.uniform_f32() + scalar_range.start;
        let root = IvyPoint {
            id: 0,
            position,
            direction,
            thickness: 0.0,
        };
        let id = self.next_ivy_id;
        self.ivies.insert(
            id,
            Ivy {
                points: VecDeque::from([root]),
                id,
                root,
                maximum_point_count,
                next_point_id: 1,
                source_60,
                pending_growth: None,
                growth_progress: 0.0,
                growth_delay,
                flag_8c: 0,
            },
        );
        self.next_ivy_id = self.next_ivy_id.wrapping_add(1);
        id
    }
    pub fn iter(&self) -> impl Iterator<Item = (&IvyId, &Ivy)> {
        self.ivies.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&IvyId, &mut Ivy)> {
        self.ivies.iter_mut()
    }
    pub fn roots(&self) -> impl Iterator<Item = Vec3> + '_ {
        self.ivies.values().map(|ivy| ivy.root.position)
    }
    pub fn get(&self, id: IvyId) -> Option<&Ivy> {
        self.ivies.get(&id)
    }
    pub fn get_mut(&mut self, id: IvyId) -> Option<&mut Ivy> {
        self.ivies.get_mut(&id)
    }
    pub fn remove(&mut self, id: IvyId) -> Option<Ivy> {
        self.ivies.remove(&id)
    }
    pub fn total_segment_count(&self) -> usize {
        self.ivies
            .values()
            .map(|ivy| ivy.points.len().saturating_sub(1))
            .sum()
    }
    /// Total stored points across every ivy. The native spawner budget
    /// (`total_point_count + 0x13 < 0x9c40`, 0x14094d09b..a7) and the
    /// per-frame 0x46 (70) point cap (0x14094db6e..b9e) compare this count.
    pub fn total_point_count(&self) -> usize {
        self.ivies.values().map(|ivy| ivy.points.len()).sum()
    }
}
