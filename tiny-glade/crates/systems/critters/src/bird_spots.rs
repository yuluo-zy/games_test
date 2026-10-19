//! Original B BirdSpot lifetime: create_spots (140f86340), Drop (14089a380).
//! The manager owns the returned-record queue strongly; each point set owns
//! only a Weak reference, so dropping points after their manager is safe.
use bevy_ecs::{component::Component, entity::Entity, resource::Resource};
use glam::Vec3;
use std::sync::{Arc, Mutex, Weak};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BirdSpotId(pub u32);

/// Native 24-byte record: Option<Entity>, world position, monotonically assigned id.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BirdSpot {
    pub occupant: Option<Entity>,
    pub pos: Vec3,
    pub id: BirdSpotId,
}

#[derive(Component, Debug, Default)]
pub struct BirdSpots {
    pub spots: Box<[BirdSpot]>,
    returned: Weak<Mutex<Vec<BirdSpot>>>,
}
impl Drop for BirdSpots {
    fn drop(&mut self) {
        if let Some(returned) = self.returned.upgrade() {
            // Move every entire record, including the occupant and id. Native
            // parking_lot mutexes do not poison; recover std's lock accordingly.
            let mut returned = returned.lock().unwrap_or_else(|e| e.into_inner());
            for spot in std::mem::take(&mut self.spots).into_vec() { returned.push(spot); }
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct BirdSpotManager {
    next_id: u32,
    returned: Arc<Mutex<Vec<BirdSpot>>>,
}
impl BirdSpotManager {
    /// B 140f86330: return old u32, wrapping increment by one.
    pub fn gen_id(&mut self) -> BirdSpotId {
        let old = self.next_id;
        self.next_id = old.wrapping_add(1);
        BirdSpotId(old)
    }
    pub fn create_spots(&mut self, spots: Vec<BirdSpot>) -> BirdSpots {
        BirdSpots {
            spots: spots.into_boxed_slice(),
            returned: Arc::downgrade(&self.returned),
        }
    }
    /// Original generic recycle drains under the queue mutex; the actual game
    /// callback owns the entity-generation/query/state predicates.
    pub fn recycle(&mut self, mut callback: impl FnMut(BirdSpot)) {
        let mut returned = self.returned.lock().unwrap_or_else(|e| e.into_inner());
        for spot in returned.drain(..) {
            callback(spot);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ids_wrap_and_whole_records_return_only_when_points_drop() {
        let mut manager = BirdSpotManager::default();
        manager.next_id = u32::MAX;
        assert_eq!(manager.gen_id(), BirdSpotId(u32::MAX));
        assert_eq!(manager.gen_id(), BirdSpotId(0));
        let expected = BirdSpot {
            occupant: Some(Entity::from_raw(19)),
            pos: Vec3::new(-0.0, 3.0, 5.0),
            id: BirdSpotId(6),
        };
        let points = manager.create_spots(vec![expected]);
        manager.recycle(|_| panic!("point set still owns its record"));
        drop(points);
        let mut got = Vec::new();
        manager.recycle(|p| got.push(p));
        assert_eq!(got, [expected]);
        manager.recycle(|_| panic!("record recycled twice"));
    }
    #[test]
    fn weak_lifetime_allows_manager_to_drop_first() {
        let points = BirdSpotManager::default().create_spots(vec![BirdSpot::default()]);
        drop(points);
    }
}
