//! Canonical B clutter mesh metadata, owned by the actual asset loader.
//! B RNE: helper 1408b8e90/730B; metadata writes 1422c1cc0/928B.
//! See the identity-bound scene-clutter-producer-20261020 evidence package.

use bevy_ecs::{component::Component, resource::Resource};
use glam::{Vec2, Vec3};
use utils::{geometry::rectangle::Rectangle3d, name_hash::NameHash};

use crate::{asset_libraries::Handle, components::drawable::NaniSolidTexture, render::mesh::Mesh};

// Preserve the existing serde/tag identity; do not create a second enum.
pub use crate::systems::mode_manager::ClutterMeshTy;

/// The real loader's metadata. None is only the original demo-omitted asset.
#[derive(Clone, Debug)]
pub struct ClutterMesh {
    pub bounds: Rectangle3d,
    pub mesh_name: &'static str,
    pub mesh: Option<Handle<Mesh>>,
    pub name_hash: NameHash,
    pub radius_xz: f32,
    /// Original +0x44. Its business meaning is not inferred from a number.
    pub native_group: u8,
    /// Original +0x45, the actual decorator builder selector writer.
    pub solid_texture: Option<NaniSolidTexture>,
    /// Original +0x46 and +0x47; preserve these flags for their real consumers.
    pub native_flag_46: bool,
    pub native_flag_47: bool,
}

impl ClutterMesh {
    /// Original closure's explicit demo omission path: no mesh registration,
    /// zero-sized bounds, real name hash, radius=1, and unchanged metadata flags.
    pub fn omitted_in_demo(
        mesh_name: &'static str,
        native_group: u8,
        solid_texture: Option<NaniSolidTexture>,
        native_flag_46: bool,
        native_flag_47: bool,
    ) -> Self {
        Self {
            bounds: Rectangle3d::new(Vec3::ZERO, Vec3::ZERO, Vec2::X),
            mesh_name,
            mesh: None,
            name_hash: NameHash::compute(mesh_name),
            radius_xz: 1.0,
            native_group,
            solid_texture,
            native_flag_46,
            native_flag_47,
        }
    }
}

/// Fixed slots follow original +0x18..+0x1c8. Type29 uses the first flowerbed.
/// There is no successful empty/default resource.
#[derive(Resource, Clone, Debug)]
pub struct ClutterMeshes {
    fixed: [ClutterMesh; 55],
    window_flowerbeds: Vec<ClutterMesh>,
}

impl ClutterMeshes {
    pub fn from_parts(fixed: [ClutterMesh; 55], window_flowerbeds: Vec<ClutterMesh>) -> Self {
        Self {
            fixed,
            window_flowerbeds,
        }
    }

    pub fn mesh(&self, ty: ClutterMeshTy) -> &ClutterMesh {
        let tag = ty as usize;
        if tag == 29 {
            self.window_flowerbeds
                .first()
                .expect("ClutterMeshes window flowerbeds are empty")
        } else {
            &self.fixed[if tag < 29 { tag } else { tag - 1 }]
        }
    }

    pub fn window_flowerbeds(&self) -> &[ClutterMesh] {
        &self.window_flowerbeds
    }
}

/// The original helper's three outputs. The generator takes its actual selector
/// from mesh.solid_texture, while other consumers may use this tuple's selector.
#[derive(Clone, Copy, Debug)]
pub struct ClutterMeshTyData<'a> {
    pub mesh: &'a ClutterMesh,
    pub nani_name: &'static str,
    pub solid_texture: Option<NaniSolidTexture>,
}

pub fn decode_solid_texture(code: u8) -> Option<NaniSolidTexture> {
    if code == 9 {
        None
    } else {
        Some(NaniSolidTexture::try_from(code).expect("invalid native clutter solid selector"))
    }
}

pub fn clutter_mesh_ty_data(ty: ClutterMeshTy, meshes: &ClutterMeshes) -> ClutterMeshTyData<'_> {
    // Every entry was followed through the original relative jump table.
    const SELECTOR: [u8; 56] = [
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 4, 4, 4, 4, 5, 9, 4, 4, 4, 4, 6, 4, 9, 4, 4, 4, 4, 9, 9, 9,
        9, 9, 9, 9, 3, 0, 1, 7, 8, 7, 4, 4, 2, 4, 4, 9, 4, 4, 4, 4, 4, 9, 9, 9, 4, 4,
    ];
    let code = SELECTOR[ty as usize];
    let nani_name = match ty as u8 {
        39 | 40 | 41 | 43 => "nani_clutter_SolidAlbedoMapEmissive",
        _ if code == 9 => "nani_solid_vertex_color",
        _ => "nani_clutter_SolidAlbedoMap",
    };
    ClutterMeshTyData {
        mesh: meshes.mesh(ty),
        nani_name,
        solid_texture: decode_solid_texture(code),
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClutterMarkedForDeletion;

/// `DecoratorAutoClutterChildren`（原版 `country_core::resources::clutter_meshes`；
/// `ClutterPlugin::build` 在 0x1406527c2 以空 `HashMap`+`RandomState`（0x30 B）插入）。
///
/// 机器码证据：`ui_convert_autoclutter_to_decoclutter`（0x14099ca60）的 FunctionSystem 参数面为
/// `(ResMut<DeletedAutoClutter>, ResMut<DecoratorAutoClutterChildren>, Query<(Entity, &AutoClutter,
/// &AutoClutterConvertableToDecoClutter, &Transform, ..)>)`，函数体对 `HashMap<DecoratorId,
/// Vec<DisplayChildAutoClutterInfo>>` 做 `grow_one<DisplayChildAutoClutterInfo>`（步长 0x60）。
/// 即此前本 crate 以 `AutoClutterParentMap` 近似的 parent→children 映射，现取原名与原模块。
/// 项保存真实网格元数据、变换及动画进度；不保存native指针或人工子实体镜像。
/// ui_convert每帧清空并从真实ECS Query重建，生命周期与实体世代保持一致。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorAutoClutterChildren(
    pub hashbrown::HashMap<u32, Vec<crate::systems::ui_clutter::ui_autoclutter::DisplayChildAutoClutterInfo>, std::hash::RandomState>,
);

impl DecoratorAutoClutterChildren {
    /// 维护一条记录（已有键 push，新键新建 Vec）。
    pub fn maintain(&mut self, parent: u32, info: crate::systems::ui_clutter::ui_autoclutter::DisplayChildAutoClutterInfo) {
        self.0.entry(parent).or_default().push(info);
    }
    pub fn parent_of(&self, parent: u32) -> Option<&Vec<crate::systems::ui_clutter::ui_autoclutter::DisplayChildAutoClutterInfo>> {
        self.0.get(&parent)
    }
}
