//! B `resources/walls/decorator_subtype.rs` — 装饰子类型网格库（原 49 函数：业务 20）。
//!
//! 证据：`chapters/resources/walls/decorator_subtype.rs.md`（方法面 O）+ `wall/decomp/*`。
//! 记号：O=机器码证实 / I=推断 / U=未恢复。
//!
//! `queue`/`apply`/`new_archetype` 0x14003bb80 = `ret` 空函数（SystemParam 派生
//! 胶水的默认 no-op）；`validate_param` 0x14026c6a0 为 bevy 派生胶水（检查
//! "::subtypes"/"::mesh_lib::subtypes" 资源存在），均不计独立业务。

use std::collections::HashMap;
use std::sync::Arc;

use bevy_ecs::system::{Res, SystemParam};

use super::decorator::DecoratorId;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use utils::geometry::{Rectangle3d, intersect::Triangle};
use crate::systems::collision::world::RaycastMeshData;

pub use super::decorator_outline::{DecoratorCollisionOutline, DecoratorOutline};

/// B DecoratorSubtype：装饰子类型（u8；空=0x1a）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DecoratorSubtype(pub u8);

impl DecoratorSubtype {
    /// 空子类型值（chapter：空=0x1a）。
    pub const EMPTY: u8 = 0x1a;

    /// 0x140002370（O）：迭代 first（xor eax,eax → 返回 0；edit_arrow 泛型
    /// 实例）。I：迭代器语义待取证。
    pub fn first() -> Self {
        Self(0)
    }

    /// 0x140b25b50（O，fresh disasm 2026-10-17）：**位掩码成员测试**
    /// `mov ecx,0x91008; bt ecx,eax; setc al`——子类型 ∈ {bit3,bit12,bit16,bit19}
    /// （0x91008 = 2³|2¹²|2¹⁶|2¹⁹）才可上角。Rust 此前 `== 3`（restorer 近似）与
    /// 机器码不符——已重建为位测试。
    pub fn can_be_on_corner(self) -> bool {
        // x86 `bt ecx, eax` 位索引按 32 取模；掩码 0x91008。
        ((0x91008u32 >> (self.0 as u32 % 32)) & 1) != 0
    }
}

/// B 子类型→子类型转换（impl$6 0x140833ba0 尾跳转发 / impl$7 0x140833250 字节
/// 直通，O 结构）。
pub trait IntoDecoratorSubtype {
    fn into_decorator_subtype(self) -> DecoratorSubtype;
}

impl IntoDecoratorSubtype for DecoratorSubtype {
    /// impl$7（0x140833250）：`mov eax,ecx` 字节直通。
    fn into_decorator_subtype(self) -> DecoratorSubtype {
        self
    }
}

/// 装饰网格逻辑键：subtype与rank；保留现variant字段名称作为rank。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DecoratorLibKey {
    pub subtype: u8,
    pub variant: u8,
}

impl DecoratorLibKey {
    /// 0x140b25c90（O；PDB 名与 impl$1::get::closure$0 两参交换同址）：构造。
    pub fn new(subtype: u8, rank: u8) -> Self {
        Self { subtype, variant: rank }
    }
}

/// 原装饰碰撞记录：拥有三角形、共享真实物理网格，并保留原Rectangle3d。
/// 三角形clone会复制缓冲，mesh只增加同一Arc的引用；此值不能Copy。
#[derive(Clone)]
pub struct DecoratorCollision {
    pub triangles: Vec<Triangle>,
    pub mesh: Arc<RaycastMeshData>,
    pub bounds: Rectangle3d,
}

impl std::fmt::Debug for DecoratorCollision {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DecoratorCollision")
            .field("triangles", &self.triangles)
            .field("mesh_hash", &self.mesh.hash)
            .field("bounds", &self.bounds)
            .finish()
    }
}

impl DecoratorCollision {
    /// 原140b25ca0构造真实零cuboid、空三角形和单位bounds；不是默认无shape记录。
    #[cfg(feature = "native-physics")]
    pub fn empty() -> Self {
        Self {
            triangles: Vec::new(),
            mesh: Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::ZERO, glam::Vec3::ZERO)),
            bounds: Rectangle3d::new(glam::Vec3::ZERO, glam::Vec3::ONE, Vec2::X),
        }
    }
}

/// 尚待调用方迁移的旧bbox元数据；不在canonical四表或ID表中存储。
/// 原DerivedDecoratorInfo是两个真实owner的借用视图，不能用本记录替代。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LegacyDerivedDecoratorInfoRecord {
    /// 子类型。
    pub subtype: DecoratorSubtype,
    /// I：bbx min（库空间）。
    pub bbx_min: glam::Vec3,
    /// I：bbx max。
    pub bbx_max: glam::Vec3,
    /// I：备用子类型（try_get_subtype 返回；U：精确字段待取证）。
    pub alt_subtype: Option<DecoratorSubtype>,
}

/// Compatibility name for callers awaiting the real catalog/schema migration.
#[deprecated(note = "legacy bbox record; canonical ID lookup is DerivedDecoratorInfoView")]
pub type DerivedDecoratorInfo = LegacyDerivedDecoratorInfoRecord;

impl LegacyDerivedDecoratorInfoRecord {
    /// 行 273（内联，I）：构造。
    pub fn new(subtype: DecoratorSubtype, bbx_min: glam::Vec3, bbx_max: glam::Vec3) -> Self {
        Self {
            subtype,
            bbx_min,
            bbx_max,
            alt_subtype: None,
        }
    }

    /// Legacy record field access, not native required ID lookup.
    pub fn subtype(&self) -> DecoratorSubtype {
        self.subtype
    }

    /// 旧宿主bbox差值方言；不是尚未绑定完整signature的原dims_wall_space。
    pub fn dims_wall_space(&self) -> Vec2 {
        Vec2::new(
            self.bbx_max.x - self.bbx_min.x,
            self.bbx_max.z - self.bbx_min.z,
        )
    }

    /// 旧宿主min/max元数据，不声称复现原13KB网格范围算法。
    pub fn bbx_ls(&self) -> (glam::Vec3, glam::Vec3) {
        (self.bbx_min, self.bbx_max)
    }

    // 此legacy元数据没有真实mesh owner，不能从bbox伪造canonical碰撞值。
}

impl IntoDecoratorSubtype for LegacyDerivedDecoratorInfoRecord {
    /// impl$6（0x140833ba0）：尾跳转发（取 subtype）。
    fn into_decorator_subtype(self) -> DecoratorSubtype {
        self.subtype
    }
}

/// 原装饰ID到子类型的唯一权威表，payload直接是DecoratorSubtype。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct DerivedDecoratorSubtypes {
    map: HashMap<u32, DecoratorSubtype>,
}

impl DerivedDecoratorSubtypes {
    /// 原140b25b60：不存在的实际ID必须失败，不能退回空子类型。
    pub fn get(&self, id: u32) -> DecoratorSubtype {
        self.map.get(&id).copied().unwrap_or_else(|| {
            panic!("The decorator subtype library doesn't contain subtype {}", id)
        })
    }

    /// 原producer清同一map并保留allocation/hasher，删除上一帧不再存在的ID。
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// 返回真实旧值，供原producer检测重复ID；第二遍可显式忽略覆盖结果。
    pub fn insert(&mut self, id: u32, subtype: DecoratorSubtype) -> Option<DecoratorSubtype> {
        self.map.insert(id, subtype)
    }
}

/// 原四张拥有真实值的网格表；默认空表不表示资产加载完成。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct DecoratorMeshLibrary {
    pub collision: HashMap<DecoratorLibKey, DecoratorCollision>,
    pub interaction: HashMap<DecoratorLibKey, DecoratorCollision>,
    pub outline: HashMap<DecoratorLibKey, DecoratorOutline>,
    pub collision_outline: HashMap<DecoratorLibKey, DecoratorCollisionOutline>,
}

impl DecoratorMeshLibrary {
    /// 原140b25ef0：先复制三角形及共享Arc，再逐表插入并释放被替换的旧值。
    pub fn insert_collision_and_interaction_as_same(
        &mut self,
        key: DecoratorLibKey,
        collision: DecoratorCollision,
        outline: DecoratorOutline,
        collision_outline: DecoratorCollisionOutline,
    ) {
        let collision_copy = collision.clone();
        drop(self.collision.insert(key, collision_copy));
        drop(self.interaction.insert(key, collision));
        drop(self.outline.insert(key, outline));
        drop(self.collision_outline.insert(key, collision_outline));
    }

    /// 原140b262a0保留第一次collision复制与最终释放顺序，不合并成事务或move。
    pub fn insert(
        &mut self,
        key: DecoratorLibKey,
        collision: DecoratorCollision,
        interaction: DecoratorCollision,
        outline: DecoratorOutline,
        collision_outline: DecoratorCollisionOutline,
    ) {
        let collision_copy = collision.clone();
        drop(self.collision.insert(key, collision_copy));
        drop(self.interaction.insert(key, interaction));
        drop(self.outline.insert(key, outline));
        drop(self.collision_outline.insert(key, collision_outline));
        drop(collision);
    }

    /// 现安全host适配：借用同一碰撞表，不逐装饰物或逐帧复制Vec。
    pub fn collision_record(&self, key: DecoratorLibKey) -> &DecoratorCollision {
        self.collision.get(&key).unwrap_or_else(|| Self::missing_key(key))
    }

    pub fn interaction_record(&self, key: DecoratorLibKey) -> &DecoratorCollision {
        self.interaction.get(&key).unwrap_or_else(|| Self::missing_key(key))
    }

    /// 原140b26690返回表内值地址；显式None载荷与缺键是不同情况。
    pub fn get_outline(&self, key: DecoratorLibKey) -> &DecoratorOutline {
        self.outline.get(&key).unwrap_or_else(|| Self::missing_key(key))
    }

    /// 原140b267f0同样保留required失败，不生成默认描边。
    pub fn get_collision_outline(&self, key: DecoratorLibKey) -> &DecoratorCollisionOutline {
        self.collision_outline.get(&key).unwrap_or_else(|| Self::missing_key(key))
    }

    fn missing_key(key: DecoratorLibKey) -> ! {
        panic!(
            "The decorator mesh library doesn't contain ({}, {})",
            key.subtype, key.variant,
        )
    }
}

/// Legacy record conversion only; native From140b26a70 is the two-owner view below.
impl From<&LegacyDerivedDecoratorInfoRecord> for DecoratorSubtype {
    fn from(info: &LegacyDerivedDecoratorInfoRecord) -> Self {
        info.subtype
    }
}

/// Actual unique resource owners used by native DerivedDecoratorInfo.
///
/// The SystemParam replaces the old registration-only DecoratorSubtypes shell.
/// 同一张原subtype表与同一mesh library，不复制或缓存另一份registry。
#[derive(SystemParam)]
pub struct DerivedDecoratorInfoResources<'w> {
    subtypes: Res<'w, DerivedDecoratorSubtypes>,
    mesh_lib: Res<'w, DecoratorMeshLibrary>,
}

impl DerivedDecoratorInfoResources<'_> {
    pub fn as_view(&self) -> DerivedDecoratorInfoView<'_> {
        DerivedDecoratorInfoView::new(&self.subtypes, &self.mesh_lib)
    }
}

/// Compatibility alias for the former registration-only SystemParam name.
#[deprecated(note = "use DerivedDecoratorInfoResources and borrow its actual two-owner view")]
pub type DecoratorSubtypes<'w> = DerivedDecoratorInfoResources<'w>;

/// Borrowed native DerivedDecoratorInfo owner pair (From140b26a70/8B).
///
/// This lifetime-safe view shares the existing canonical resources. It does not
/// clone an Arc or cache a second subtype map; queries borrow the actual owners.
#[derive(Clone, Copy, Debug)]
pub struct DerivedDecoratorInfoView<'a> {
    subtypes: &'a DerivedDecoratorSubtypes,
    mesh_lib: &'a DecoratorMeshLibrary,
}

impl<'a> DerivedDecoratorInfoView<'a> {
    pub const fn new(
        subtypes: &'a DerivedDecoratorSubtypes,
        mesh_lib: &'a DecoratorMeshLibrary,
    ) -> Self {
        Self { subtypes, mesh_lib }
    }

    /// Native231B lookup returns None for a missing actual decorator ID.
    ///
    /// 直接借用唯一ID表的真实值，不经临时Arc或影子记录。
    pub fn try_get_subtype(&self, id: DecoratorId) -> Option<&'a DecoratorSubtype> {
        self.subtypes.map.get(&id.0)
    }

    /// Native8B required lookup preserves the missing-ID failure.
    pub fn subtype(&self, id: DecoratorId) -> DecoratorSubtype {
        *self.try_get_subtype(id).unwrap_or_else(|| {
            panic!("The decorator subtype library doesn't contain subtype {}", id.0)
        })
    }

    pub fn subtype_library(&self) -> &'a DerivedDecoratorSubtypes {
        self.subtypes
    }

    /// 借出同一四表资源，消费者按实际key读取真实mesh与bounds。
    pub fn mesh_library(&self) -> &'a DecoratorMeshLibrary {
        self.mesh_lib
    }
}

impl<'a, 'w> From<&'a DerivedDecoratorInfoResources<'w>> for DerivedDecoratorInfoView<'a> {
    fn from(resources: &'a DerivedDecoratorInfoResources<'w>) -> Self {
        resources.as_view()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtype_corner_predicate() {
        assert!(DecoratorSubtype(3).can_be_on_corner());
        assert!(!DecoratorSubtype(0).can_be_on_corner());
        assert!(!DecoratorSubtype(0x1a).can_be_on_corner());
        assert_eq!(DecoratorSubtype::first(), DecoratorSubtype(0));
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn mesh_library_insert_and_query_4_tables() {
        let mut lib = DecoratorMeshLibrary::default();
        let key = DecoratorLibKey::new(1, 4);
        lib.insert_collision_and_interaction_as_same(
            key, DecoratorCollision::empty(),
            DecoratorOutline::None, DecoratorCollisionOutline::None,
        );
        assert!(matches!(lib.get_outline(key), DecoratorOutline::None));
        assert!(matches!(lib.get_collision_outline(key), DecoratorCollisionOutline::None));
        assert!(Arc::ptr_eq(&lib.collision_record(key).mesh, &lib.interaction_record(key).mesh));
        assert_eq!(lib.collision_record(key).bounds.footprint.size, Vec2::ONE);
    }

    #[test]
    fn derived_subtypes_get_panics_on_missing() {
        let lib = DerivedDecoratorSubtypes::default();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = lib.get(99);
        }))
        .is_err());
    }

    #[test]
    fn id_map_clear_retains_capacity_and_insert_reports_the_replaced_value() {
        let mut subtypes = DerivedDecoratorSubtypes::default();
        for id in 0..64 {
            assert_eq!(subtypes.insert(id, DecoratorSubtype(24)), None);
        }
        let capacity = subtypes.map.capacity();
        assert_eq!(subtypes.insert(7, DecoratorSubtype(25)), Some(DecoratorSubtype(24)));
        subtypes.clear();
        assert!(subtypes.map.is_empty());
        assert_eq!(subtypes.map.capacity(), capacity);
        assert_eq!(subtypes.insert(7, DecoratorSubtype(19)), None);
        assert_eq!(subtypes.get(7), DecoratorSubtype(19));
        assert_eq!(subtypes.map.capacity(), capacity);
    }

    #[test]
    fn into_decorator_subtype_forwarding() {
        assert_eq!(DecoratorSubtype(5).into_decorator_subtype(), DecoratorSubtype(5));
        let d = LegacyDerivedDecoratorInfoRecord::new(DecoratorSubtype(6), glam::Vec3::ZERO, glam::Vec3::ONE);
        assert_eq!(d.into_decorator_subtype(), DecoratorSubtype(6));
    }
    #[test]
    fn derived_view_borrows_existing_entry_without_cloning_arc() {
        let mut subtypes = DerivedDecoratorSubtypes::default();
        let mesh_lib = DecoratorMeshLibrary::default();
        assert_eq!(subtypes.insert(7, DecoratorSubtype(20)), None);
        let view = DerivedDecoratorInfoView::new(&subtypes, &mesh_lib);
        assert!(core::ptr::eq(view.subtype_library(), &subtypes));
        assert!(core::ptr::eq(view.mesh_library(), &mesh_lib));
        assert!(core::ptr::eq(view.try_get_subtype(DecoratorId(7)).unwrap(), subtypes.map.get(&7).unwrap()));
        assert_eq!(view.subtype(DecoratorId(7)), DecoratorSubtype(20));
        assert_eq!(view.try_get_subtype(DecoratorId(8)), None);
    }

    #[test]
    fn derived_view_required_missing_id_is_observable() {
        let subtypes = DerivedDecoratorSubtypes::default();
        let mesh_lib = DecoratorMeshLibrary::default();
        let view = DerivedDecoratorInfoView::new(&subtypes, &mesh_lib);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            view.subtype(DecoratorId(99));
        })).is_err());
    }

    #[test]
    fn system_param_view_reuses_actual_world_owners_and_sees_next_producer_write() {
        use bevy_ecs::{system::SystemState, world::World};
        let mut world = World::new();
        world.insert_resource(DerivedDecoratorSubtypes::default());
        world.insert_resource(DecoratorMeshLibrary::default());
        let mut state = SystemState::<DerivedDecoratorInfoResources>::new(&mut world);
        {
            let resources = state.get(&world);
            let view = DerivedDecoratorInfoView::from(&resources);
            assert!(core::ptr::eq(view.subtype_library(), world.resource::<DerivedDecoratorSubtypes>()));
            assert!(core::ptr::eq(view.mesh_library(), world.resource::<DecoratorMeshLibrary>()));
            assert_eq!(view.try_get_subtype(DecoratorId(7)), None);
        }
        assert_eq!(world.resource_mut::<DerivedDecoratorSubtypes>().insert(7, DecoratorSubtype(5)), None);
        let resources = state.get(&world);
        assert_eq!(resources.as_view().subtype(DecoratorId(7)), DecoratorSubtype(5));
    }

}

pub mod placement;


#[cfg(all(test, feature = "native-physics"))]
mod canonical_collision_regressions {
    use super::*;
    use glam::Vec3;

    fn collision(size: Vec3) -> DecoratorCollision {
        DecoratorCollision {
            triangles: vec![Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y)],
            mesh: Arc::new(RaycastMeshData::new_cuboid_at(size, Vec3::ZERO)),
            bounds: Rectangle3d::new(Vec3::ZERO, size, Vec2::X),
        }
    }

    #[test]
    fn collision_clone_copies_triangles_and_shares_the_actual_mesh() {
        let original = collision(Vec3::new(2.0, 3.0, 4.0));
        let cloned = original.clone();
        assert_eq!(original.triangles, cloned.triangles);
        assert_ne!(original.triangles.as_ptr(), cloned.triangles.as_ptr());
        assert!(Arc::ptr_eq(&original.mesh, &cloned.mesh));
        assert_eq!(original.bounds, cloned.bounds);
        assert!(original.mesh.shape_ref().unwrap().as_cuboid().is_some());
    }

    #[test]
    fn shared_insertion_keeps_distinct_vecs_and_releases_all_replaced_owners() {
        let mut lib = DecoratorMeshLibrary::default();
        let key = DecoratorLibKey::new(3, 2);
        let first = collision(Vec3::ONE);
        let previous_mesh = Arc::downgrade(&first.mesh);
        lib.insert_collision_and_interaction_as_same(
            key, first, DecoratorOutline::Lines(vec![Vec3::ZERO, Vec3::X]),
            DecoratorCollisionOutline::Sphere(2.0),
        );
        let c = lib.collision_record(key);
        let i = lib.interaction_record(key);
        assert_ne!(c.triangles.as_ptr(), i.triangles.as_ptr());
        assert!(Arc::ptr_eq(&c.mesh, &i.mesh));
        assert_eq!(Arc::strong_count(&c.mesh), 2);
        let replacement = collision(Vec3::splat(2.0));
        lib.insert_collision_and_interaction_as_same(
            key, replacement, DecoratorOutline::None, DecoratorCollisionOutline::None,
        );
        assert!(previous_mesh.upgrade().is_none());
        assert!(matches!(lib.get_outline(key), DecoratorOutline::None));
        assert!(matches!(lib.get_collision_outline(key), DecoratorCollisionOutline::None));
    }

    #[test]
    fn four_table_insert_preserves_different_collision_and_interaction() {
        let mut lib = DecoratorMeshLibrary::default();
        let key = DecoratorLibKey::new(7, 5);
        let collision_value = collision(Vec3::new(2.0, 3.0, 4.0));
        let collision_probe = collision_value.mesh.clone();
        let interaction = collision(Vec3::new(5.0, 6.0, 7.0));
        let interaction_probe = interaction.mesh.clone();
        let moved_triangle_address = interaction.triangles.as_ptr();
        lib.insert(key, collision_value, interaction, DecoratorOutline::Mesh("decorators/window".into()),
            DecoratorCollisionOutline::Lines(vec![Vec3::ZERO, Vec3::Y]));
        let c = lib.collision_record(key);
        let i = lib.interaction_record(key);
        assert!(Arc::ptr_eq(&c.mesh, &collision_probe));
        assert!(Arc::ptr_eq(&i.mesh, &interaction_probe));
        assert!(!Arc::ptr_eq(&c.mesh, &i.mesh));
        assert_eq!(Arc::strong_count(&collision_probe), 2);
        assert_eq!(Arc::strong_count(&interaction_probe), 2);
        assert_eq!(i.triangles.as_ptr(), moved_triangle_address);
        assert_eq!(c.bounds.footprint.size.x, 2.0);
        assert_eq!(c.bounds.top - c.bounds.bottom, 3.0);
        assert_eq!(i.bounds.footprint.size.x, 5.0);
        assert_eq!(i.bounds.top - i.bounds.bottom, 6.0);
    }

    #[test]
    fn exact_rank_missing_is_failure_while_stored_none_is_valid() {
        use std::panic::{catch_unwind, AssertUnwindSafe};
        let mut lib = DecoratorMeshLibrary::default();
        let key = DecoratorLibKey::new(4, 1);
        lib.insert_collision_and_interaction_as_same(
            key, collision(Vec3::ONE), DecoratorOutline::None, DecoratorCollisionOutline::None,
        );
        assert!(core::ptr::eq(lib.get_outline(key), lib.outline.get(&key).unwrap()));
        assert!(core::ptr::eq(lib.get_collision_outline(key), lib.collision_outline.get(&key).unwrap()));
        let wrong_rank = DecoratorLibKey::new(4, 0);
        assert!(catch_unwind(AssertUnwindSafe(|| lib.get_outline(wrong_rank))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| lib.get_collision_outline(wrong_rank))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| lib.collision_record(wrong_rank))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| lib.interaction_record(wrong_rank))).is_err());
    }

    #[test]
    fn empty_is_a_real_zero_cuboid_with_unit_bounds() {
        let empty = DecoratorCollision::empty();
        assert!(empty.triangles.is_empty());
        let cuboid = empty.mesh.shape_ref().unwrap().as_cuboid().unwrap();
        assert_eq!(cuboid.half_extents.x, 0.0);
        assert_eq!(cuboid.half_extents.y, 0.0);
        assert_eq!(cuboid.half_extents.z, 0.0);
        assert_eq!(empty.bounds, Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X));
        assert_eq!(empty.mesh.local_transform().translation, Vec3::ZERO);
    }
}
