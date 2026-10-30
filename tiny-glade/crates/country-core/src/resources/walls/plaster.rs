//! 石膏资源族（walls/plaster.rs）。
//!
//! 原 B `resources/walls/plaster.rs`（PDB 实体 `Plaster`/`AnimatedPlasterPeel`/
//! `CreatePlasterCmd`/`DeletePlasterCmd`/`RecalculateHalfTimberMeshCmd`/
//! `RecalculatePlasterCmd`/`RecalculateWoodenTurretCmd`；`system_wall_constructor`
//! 的 plaster_systems 消费）。
use bevy_ecs::{component::Component, event::Event, resource::Resource};

use crate::resources::walls::WallId;

/// B 的 40 字节石膏实例记录。前 32 字节的字段语义仍待网格生产者核定；
/// 按原位保留全部位型，末尾 WallId 供 Nani 与剥落资源索引。
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Plaster {
    pub words: [u32; 8],
    pub wall: WallId,
}

impl Plaster {
    /// 供已有 Nani 打包逻辑读取，不改变浮点或地址相关字的位型。
    pub fn nani_view(self) -> crate::render::nani::instance_updates::PlasterView {
        let mut words = [0; 10];
        words[..8].copy_from_slice(&self.words);
        words[8] = self.wall.0 as u32;
        words[9] = (self.wall.0 >> 32) as u32;
        crate::render::nani::instance_updates::PlasterView::from_words(words)
    }
}

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

    #[test]
    fn plaster_record_keeps_all_words_and_wall_identity_for_nani() {
        let p = Plaster { words: [1, 2, 3, 4, 5, 6, 7, 8], wall: WallId(0x1122_3344_5566_7788) };
        assert_eq!(std::mem::size_of::<Plaster>(), 40);
        let view = p.nani_view();
        assert_eq!(view.head, [1, 2, 3, 4]);
        assert_eq!(view.qword_10, 5 | (6u64 << 32));
        assert_eq!((view.dword_18, view.dword_1c, view.wall_id), (7, 8, p.wall.0));
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

/// 半木重建：B reader 按 64B 逐项复制到 CurvedHalfTimberInstanceData。
#[derive(Event, Clone, Debug)]
pub struct RecalculateHalfTimberMeshCmd {
    pub instances: Vec<crate::render::nani::instance_data::CurvedHalfTimberInstanceData>,
    pub wall: WallId,
}

/// 木塔半木重建；空实例列表是原零高度分支的有效结果。
#[derive(Event, Clone, Debug)]
pub struct RecalculateWoodenTurretCmd {
    pub instances: Vec<crate::render::nani::instance_data::CurvedHalfTimberInstanceData>,
    pub wall: WallId,
}
