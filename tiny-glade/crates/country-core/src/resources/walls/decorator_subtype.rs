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
use utils::geometry::{Rectangle2d, Rectangle3d};

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

/// B DecoratorLibKey：{subtype, variant} 复合键。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DecoratorLibKey {
    pub subtype: u8,
    pub variant: u8,
}

impl DecoratorLibKey {
    /// 0x140b25c90（O；PDB 名与 impl$1::get::closure$0 两参交换同址）：构造。
    pub fn new(subtype: u8, variant: u8) -> Self {
        Self { subtype, variant }
    }
}

/// B DecoratorCollision：装饰碰撞体（Rectangle3d + parry3d cuboid + IsoTransform）。
/// native-physics 碰撞体在可选的 `native-physics` feature 域（库外），此处承载
/// 几何记录。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorCollision {
    /// 三维包围矩形（footprint + y 界）。
    pub bounds: Rectangle3d,
    /// I：墙空间尺寸（dims_wall_space）。
    pub dims: Vec2,
}

impl DecoratorCollision {
    /// 0x140b25ca0（O）：空碰撞体（Rectangle3d + parry3d cuboid + IsoTransform
    /// 三连结构全零初始化 0x50+ 字节）。parry3d 部分为可选 feature 域。
    pub fn empty() -> Self {
        Self {
            bounds: Rectangle3d {
                footprint: Rectangle2d::new(Vec2::ZERO, Vec2::ZERO, Vec2::X),
                bottom: 0.0,
                top: 0.0,
            },
            dims: Vec2::ZERO,
        }
    }

    /// 行 118（内联，I）：墙空间尺寸。
    pub fn dims_wall_space(&self) -> Vec2 {
        self.dims
    }
}

/// Legacy host record retained for unmigrated bbox/catalog callers.
///
/// This is not native DerivedDecoratorInfo: the original is a borrowed view of
/// the subtype and mesh libraries. The unique backing map and four mesh tables
/// still need their actual producer/schema migration; do not treat this record
/// or an empty library as proof that those assets are loaded.
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

    /// 行 285（内联，I）：墙空间尺寸（bbx 差值）。
    pub fn dims_wall_space(&self) -> Vec2 {
        Vec2::new(
            self.bbx_max.x - self.bbx_min.x,
            self.bbx_max.z - self.bbx_min.z,
        )
    }

    /// 0x141fdc7b0（O，13KB）：bbx_ls（库空间包围盒）。
    pub fn bbx_ls(&self) -> (glam::Vec3, glam::Vec3) {
        (self.bbx_min, self.bbx_max)
    }

    /// 行 298（内联，I，经 impl$6 尾跳转发）：碰撞网格。U：网格数据待取证。
    pub fn collision_mesh(&self) -> DecoratorCollision {
        DecoratorCollision {
            bounds: Rectangle3d::new(
                (self.bbx_min + self.bbx_max) * 0.5,
                self.bbx_max - self.bbx_min,
                Vec2::X,
            ),
            dims: self.dims_wall_space(),
        }
    }

    /// 行 309（内联，I）：交互网格。U：网格数据待取证。
    pub fn interaction_mesh(&self) -> DecoratorCollision {
        self.collision_mesh()
    }
}

impl IntoDecoratorSubtype for LegacyDerivedDecoratorInfoRecord {
    /// impl$6（0x140833ba0）：尾跳转发（取 subtype）。
    fn into_decorator_subtype(self) -> DecoratorSubtype {
        self.subtype
    }
}

/// Unique current subtype owner, still using legacy host record payloads.
/// Native ID→DecoratorSubtype representation migration remains with its producer.
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct DerivedDecoratorSubtypes {
    map: HashMap<u32, Arc<LegacyDerivedDecoratorInfoRecord>>,
}

impl DerivedDecoratorSubtypes {
    /// 0x140b25b60（O）：查询，未命中 panic。
    pub fn get(&self, id: u32) -> Arc<LegacyDerivedDecoratorInfoRecord> {
        self.map.get(&id).cloned().unwrap_or_else(|| {
            panic!("The decorator subtype library doesn't contain subtype {}", id)
        })
    }

    /// 插入（供网格库维护；原为消费方注册，0x140b25b60 对面）。
    pub fn insert(&mut self, id: u32, info: Arc<LegacyDerivedDecoratorInfoRecord>) {
        self.map.insert(id, info);
    }
}

/// B DecoratorMeshLibrary：4 表（map1@+0x0 collision、map2@+0x30 interaction、
/// map3@+0x60 outline、map4@+0x90 collision_outline；insert 0x140b262a0 克隆后插
/// 4 表）。outline 表在 decorator_outline.rs 专项（本文件外）；此处以同一
/// LegacyDerivedDecoratorInfoRecord 记录承载并登记该边界。作为资源注册（SystemParam 消费）。
/// These legacy values are not genuine native collision/outline payloads.
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct DecoratorMeshLibrary {
    /// map1@+0x0：碰撞表。
    pub collision: HashMap<DecoratorLibKey, Arc<LegacyDerivedDecoratorInfoRecord>>,
    /// map2@+0x30：交互表。
    pub interaction: HashMap<DecoratorLibKey, Arc<LegacyDerivedDecoratorInfoRecord>>,
    /// map3@+0x60：描边表（专项类型在 decorator_outline.rs）。
    pub outline: HashMap<DecoratorLibKey, Arc<LegacyDerivedDecoratorInfoRecord>>,
    /// map4@+0x90：碰撞描边表。
    pub collision_outline: HashMap<DecoratorLibKey, Arc<LegacyDerivedDecoratorInfoRecord>>,
}

impl DecoratorMeshLibrary {
    /// 0x140b25ef0（O）：collision 与 interaction 插同一值。
    pub fn insert_collision_and_interaction_as_same(
        &mut self,
        key: DecoratorLibKey,
        info: Arc<LegacyDerivedDecoratorInfoRecord>,
    ) {
        self.collision.insert(key, info.clone());
        self.interaction.insert(key, info);
    }

    /// 0x140b262a0（O）：Arc 克隆后插 4 表。
    pub fn insert(&mut self, key: u32, info: Arc<LegacyDerivedDecoratorInfoRecord>) {
        let lib_key = DecoratorLibKey::new(key as u8, 0);
        self.insert_collision_and_interaction_as_same(lib_key, info.clone());
        self.outline.insert(lib_key, info.clone());
        self.collision_outline.insert(lib_key, info);
    }

    /// 0x140b26690（O）：map3@+0x60 描边查询。
    pub fn get_outline(&self, key: DecoratorLibKey) -> Option<Arc<LegacyDerivedDecoratorInfoRecord>> {
        self.outline.get(&key).cloned()
    }

    /// 0x140b267f0（O）：map4@+0x90 碰撞描边查询。
    pub fn get_collision_outline(&self, key: DecoratorLibKey) -> Option<Arc<LegacyDerivedDecoratorInfoRecord>> {
        self.collision_outline.get(&key).cloned()
    }

    /// 行 223（内联）：碰撞 bbx（库空间）。
    pub fn get_collision_bbx_ls(&self, key: DecoratorLibKey) -> Option<(glam::Vec3, glam::Vec3)> {
        self.collision.get(&key).map(|i| i.bbx_ls())
    }

    /// 0x141fdc7b0（O）：dims_wall_space（bbx 差值）。未命中 fmt panic
    /// "The decorator mesh library doesn't contain "（0x142fcaa58，file
    /// decorator_subtype.rs 0x142fcaa68）。
    pub fn dims_wall_space(&self, key: DecoratorLibKey) -> Vec2 {
        self.collision
            .get(&key)
            .map(|i| i.dims_wall_space())
            .unwrap_or_else(|| {
                panic!(
                    "The decorator mesh library doesn't contain {}",
                    key.subtype
                )
            })
    }

    /// 行 249（内联，I）：碰撞网格。
    pub fn collision_mesh(&self, key: DecoratorLibKey) -> Option<DecoratorCollision> {
        self.collision.get(&key).map(|i| i.collision_mesh())
    }

    /// 行 258（内联，I）：交互网格。
    pub fn interaction_mesh(&self, key: DecoratorLibKey) -> Option<DecoratorCollision> {
        self.interaction.get(&key).map(|i| i.interaction_mesh())
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
/// The backing map still contains legacy host records during the explicit
/// producer migration; no second subtype table or mesh library is introduced.
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
/// clone an Arc, cache a second subtype map, or claim the legacy collision and
/// outline table payloads are restored.
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
    /// Borrow directly from the unique stored entry during the explicit legacy
    /// map migration. Do not obtain a temporary cloned Arc through get(id).
    pub fn try_get_subtype(&self, id: DecoratorId) -> Option<&'a DecoratorSubtype> {
        self.subtypes.map.get(&id.0).map(|entry| &entry.subtype)
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

    /// The real second owner; its current legacy payload migration is separate.
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

    #[test]
    fn mesh_library_insert_and_query_4_tables() {
        let mut lib = DecoratorMeshLibrary::default();
        let info = Arc::new(LegacyDerivedDecoratorInfoRecord::new(
            DecoratorSubtype(1),
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::new(1.0, 2.0, 3.0),
        ));
        let key = DecoratorLibKey::new(1, 0);
        lib.insert(1, info);

        assert!(lib.get_outline(key).is_some());
        assert!(lib.get_collision_outline(key).is_some());
        assert_eq!(lib.get_collision_bbx_ls(key).unwrap().1.z, 3.0);
        assert_eq!(lib.dims_wall_space(key), Vec2::new(1.0, 3.0));
        assert!(lib.collision_mesh(key).is_some());
        assert!(lib.interaction_mesh(key).is_some());
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
    fn into_decorator_subtype_forwarding() {
        assert_eq!(DecoratorSubtype(5).into_decorator_subtype(), DecoratorSubtype(5));
        let d = LegacyDerivedDecoratorInfoRecord::new(DecoratorSubtype(6), glam::Vec3::ZERO, glam::Vec3::ONE);
        assert_eq!(d.into_decorator_subtype(), DecoratorSubtype(6));
    }
    #[test]
    fn derived_view_borrows_existing_entry_without_cloning_arc() {
        let mut subtypes = DerivedDecoratorSubtypes::default();
        let mesh_lib = DecoratorMeshLibrary::default();
        let entry = Arc::new(LegacyDerivedDecoratorInfoRecord::new(
            DecoratorSubtype(20), glam::Vec3::ZERO, glam::Vec3::ONE,
        ));
        subtypes.insert(7, entry.clone());
        let owners = Arc::strong_count(&entry);
        let view = DerivedDecoratorInfoView::new(&subtypes, &mesh_lib);
        assert!(core::ptr::eq(view.subtype_library(), &subtypes));
        assert!(core::ptr::eq(view.mesh_library(), &mesh_lib));
        assert!(core::ptr::eq(view.try_get_subtype(DecoratorId(7)).unwrap(), &entry.subtype));
        assert_eq!(view.subtype(DecoratorId(7)), DecoratorSubtype(20));
        assert_eq!(Arc::strong_count(&entry), owners);
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
        world.resource_mut::<DerivedDecoratorSubtypes>().insert(7, Arc::new(
            LegacyDerivedDecoratorInfoRecord::new(DecoratorSubtype(5), glam::Vec3::ZERO, glam::Vec3::ONE),
        ));
        let resources = state.get(&world);
        assert_eq!(resources.as_view().subtype(DecoratorId(7)), DecoratorSubtype(5));
    }

}

pub mod placement;
