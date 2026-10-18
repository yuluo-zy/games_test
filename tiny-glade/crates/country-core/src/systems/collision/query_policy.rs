//! Original17-subworld query dispatch policy, implemented without validation.
//! Actual intersection operations belong to Rapier/Parry. Geometry/source maps
//! for selected subworlds are required; missing ones are never assumed empty.
use crate::resources::walls::publication_fields::WallId;
#[derive(Clone, Copy, Debug)]
pub struct CollisionRay {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RaycastGeoSourceProjection(pub [u8; 24]);
impl RaycastGeoSourceProjection {
    pub fn tag(self) -> u8 {
        self.0[0]
    }
    pub fn platform_stair(wall: WallId) -> Self {
        let mut bytes = [0; 24];
        bytes[0] = 7;
        bytes[8..16].copy_from_slice(&wall.0.to_le_bytes());
        Self(bytes)
    }
}
#[derive(Clone, Debug)]
pub struct SubworldRayHit<H> {
    pub handle: H,
    pub source: RaycastGeoSourceProjection,
    pub time_of_impact: f32,
    pub normal: Option<[f32; 3]>,
}
#[derive(Clone, Debug)]
pub struct CollisionRayHit<H> {
    pub subworld: u8,
    pub hit: SubworldRayHit<H>,
    pub point: [f32; 3],
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CollisionQueryGap {
    MissingSubworld(u8),
    MissingMeshSource,
    UnrecoveredPredicate,
}
/// The caller must resolve actual source/predicate policy and actual dependency
/// collider sets. This is an executable interface, not supplied hit injection.
pub trait CollisionSubworldQueries {
    type Handle;
    fn cast_ray(
        &self,
        subworld: u8,
        ray: CollisionRay,
        maximum_time_of_impact: f32,
        with_normal: bool,
        excluded_wall: Option<WallId>,
    ) -> Result<Option<SubworldRayHit<Self::Handle>>, CollisionQueryGap>;
}
pub fn selected_subworlds(mask: u32) -> impl Iterator<Item = u8> {
    (0u8..17).filter(move |index| mask & (1u32 << *index) != 0)
}
/// Both original dispatchers walk selected subworlds in ascending order, pass
/// the latest maximum TOI into each dependency query, and overwrite on a hit.
/// Consequently equal-distance hits from later subworlds replace earlier ones.
/// Both wrappers use solid=true; backend implementation must preserve it.
pub fn query_collision_world<Q: CollisionSubworldQueries>(
    world: &Q,
    ray: CollisionRay,
    mut maximum_time_of_impact: f32,
    subworld_mask: u32,
    with_normal: bool,
    excluded_wall: Option<WallId>,
) -> Result<Option<CollisionRayHit<Q::Handle>>, CollisionQueryGap> {
    let mut nearest = None;
    for subworld in selected_subworlds(subworld_mask) {
        if let Some(hit) = world.cast_ray(
            subworld,
            ray,
            maximum_time_of_impact,
            with_normal,
            excluded_wall,
        )? {
            maximum_time_of_impact = hit.time_of_impact;
            let point = std::array::from_fn(|i| {
                let distance = ray.direction[i] * hit.time_of_impact;
                ray.origin[i] + distance
            });
            nearest = Some(CollisionRayHit {
                subworld,
                hit,
                point,
            });
        }
    }
    Ok(nearest)
}
