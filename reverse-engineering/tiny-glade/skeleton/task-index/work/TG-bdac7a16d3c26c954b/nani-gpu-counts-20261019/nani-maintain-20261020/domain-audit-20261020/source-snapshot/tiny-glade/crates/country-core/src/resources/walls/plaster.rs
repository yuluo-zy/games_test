//! 石膏资源族（walls/plaster.rs）。
//!
//! 原 B `resources/walls/plaster.rs`（PDB 实体 `Plaster`/`AnimatedPlasterPeel`/
//! `CreatePlasterCmd`/`DeletePlasterCmd`/`RecalculateHalfTimberMeshCmd`/
//! `RecalculatePlasterCmd`/`RecalculateWoodenTurretCmd`；`system_wall_constructor`
//! 的 plaster_systems 消费）。
use bevy_ecs::{component::Component, event::Event, resource::Resource};

use crate::{Gap, resources::walls::WallId};

/// 石膏墙（O：ECS `&Plaster`；布局 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Plaster(pub Gap);

/// Native producer1411fcad0 inserts WallId→PlasterPeelAmount; Nani
/// consumer14207b290 reads its f32, falling back to the actual wall's peel.
#[repr(transparent)]
#[derive(Clone,Copy,Debug,Default,PartialEq)]
pub struct PlasterPeelAmount(pub f32);

#[derive(Resource,Clone,Debug,Default)]
pub struct AnimatedPlasterPeel(pub std::collections::HashMap<WallId,PlasterPeelAmount>);
impl AnimatedPlasterPeel {
    /// Native clear_canvas140a7b102..157 clears the actual RawTable and keeps
    /// its allocation. No fabricated default wall/peel entry is inserted.
    pub fn clear(&mut self) {self.0.clear();}
    pub fn get(&self,wall:WallId)->Option<f32> {self.0.get(&wall).map(|amount|amount.0)}
}

#[cfg(test)]
mod animated_peel_owner_tests {
    use super::*;
    #[test]
    fn canvas_clear_uses_real_nonempty_amount_owner_and_keeps_allocation() {
        let mut peel=AnimatedPlasterPeel::default();
        peel.0.insert(WallId(7),PlasterPeelAmount(0.625));
        peel.0.insert(WallId(9),PlasterPeelAmount(f32::from_bits(0x7fc01234)));
        assert_eq!(peel.get(WallId(7)),Some(0.625));
        assert_eq!(peel.get(WallId(9)).unwrap().to_bits(),0x7fc01234);
        let capacity=peel.0.capacity();peel.clear();
        assert!(peel.0.is_empty());assert_eq!(peel.0.capacity(),capacity);
        assert_eq!(peel.get(WallId(7)),None);
    }
}

/// 创建石膏命令（O 名）。
#[derive(Event, Clone, Copy, Debug)]
pub struct CreatePlasterCmd(pub WallId);

/// 删除石膏命令（O 名）。
#[derive(Event, Clone, Copy, Debug)]
pub struct DeletePlasterCmd(pub WallId);

/// 石膏重算命令（O 名）。
#[derive(Event, Clone, Copy, Debug)]
pub struct RecalculatePlasterCmd(pub WallId);
