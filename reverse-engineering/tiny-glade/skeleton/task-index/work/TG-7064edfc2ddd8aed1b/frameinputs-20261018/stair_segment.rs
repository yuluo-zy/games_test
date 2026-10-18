//! Candidate addendum to canonical stairs_state, not a replacement graph.
use crate::resources::walls::decorator::DecoratorId;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StairSegment {
    a: DecoratorId,
    b: DecoratorId,
}
impl StairSegment {
    /// B @140901260, alias shared with StairGraphEdge::new: unsigned min/max.
    pub fn new(a: DecoratorId, b: DecoratorId) -> Self {
        Self {
            a: DecoratorId(a.0.min(b.0)),
            b: DecoratorId(a.0.max(b.0)),
        }
    }
    pub fn nodes(self) -> (DecoratorId, DecoratorId) {
        (self.a, self.b)
    }
}
impl From<StairSegment> for StairGraphEdge {
    /// Native prepare hashes the same two endpoint words as StairGraphEdge.
    fn from(segment: StairSegment) -> Self {
        Self::new(segment.a.0, segment.b.0)
    }
}
// Same original resource identity; no wrapper/new map is inserted.
impl bevy_ecs::resource::Resource for TransientStairEdgeIdAllocator {}
const _: () = assert!(core::mem::size_of::<StairSegment>() == 8);
