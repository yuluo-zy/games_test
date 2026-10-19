//! 每帧维护资产：回收已删除对象、重载变更、构建并刷新 VAO。
use super::{
    Handle,
    color_library::{ColorLibrarySsbo, GladeTheme, try_update_color_library_in},
    configwatch::ConfigWatch,
    mesh_library::AssetMeshLibrary,
    meshwatch::MeshWatch,
    ssbo_library::AssetSsboLibrary,
    texture_library::{AssetTextureLibrary, TextureLibraryDevice},
    texture_watch::TextureWatch,
    transient_mesh::delete_dropped_transient_meshes,
    vao_library::AssetVAOLibrary,
};
use crate::{
    name_hash::NameHash,
    render::{mesh::Mesh, ssbo::rhapsody_boundary::GraphicsDevice, vao::Vao},
};
use bevy_ecs::prelude::*;
use std::{path::PathBuf, sync::Arc};

/// 宿主注入真实资源表。缺少设备时保留队列，不制造更新成功。
#[derive(Resource)]
pub struct AssetLibraryRuntime {
    pub(crate) buffers: Arc<dyn GraphicsDevice + Send + Sync>,
    pub(crate) textures: Arc<dyn TextureLibraryDevice + Send + Sync>,
    pub assets_root: PathBuf,
    pub compiled_texture_root: PathBuf,
    pub theme: GladeTheme,
}
impl AssetLibraryRuntime {
    /// Borrow the same physical buffer device used by library maintenance.
    /// External game systems may flush their typed sparse owners without
    /// constructing a second device or taking ownership of its tables.
    pub fn buffer_device(&self) -> &(dyn GraphicsDevice + Send + Sync) {
        self.buffers.as_ref()
    }
    pub fn new(
        buffers: Arc<dyn GraphicsDevice + Send + Sync>,
        textures: Arc<dyn TextureLibraryDevice + Send + Sync>,
    ) -> Self {
        Self {
            buffers,
            textures,
            assets_root: "assets".into(),
            compiled_texture_root: crate::render::load_texture::COMPILED_TEXTURE_DIR.into(),
            theme: GladeTheme::Summer,
        }
    }
}

/// 所有维护系统共享顺序；宿主在 PostUpdate 调用，完成后才读取 draw 数据。
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AssetMaintenance {
    Reclaim,
    Reload,
    Rebuild,
    Geometry,
}

pub fn add_asset_library_systems(schedule: &mut Schedule) {
    schedule.configure_sets(
        (
            AssetMaintenance::Reclaim,
            AssetMaintenance::Reload,
            AssetMaintenance::Rebuild,
            AssetMaintenance::Geometry,
        )
            .chain(),
    );
    schedule.add_systems((
        reclaim_transient_meshes.in_set(AssetMaintenance::Reclaim),
        (reload_meshes, reload_textures, reload_colors).in_set(AssetMaintenance::Reload),
        rebuild_mesh_vaos.in_set(AssetMaintenance::Rebuild),
        crate::geometry::runtime::sync_wall_bricks.in_set(AssetMaintenance::Geometry),
    ));
}

fn reclaim_transient_meshes(
    runtime: Option<Res<AssetLibraryRuntime>>,
    meshes: Option<ResMut<AssetMeshLibrary>>,
    vaos: Option<ResMut<AssetVAOLibrary>>,
) {
    let (Some(runtime), Some(mut meshes), Some(mut vaos)) = (runtime, meshes, vaos) else {
        return;
    };
    delete_dropped_transient_meshes(runtime.buffers.as_ref(), &mut meshes, &mut vaos);
    // 普通资产也能由编辑器或业务代码直接删除，不依赖实体上存在 TransientMesh。
    for mesh in meshes.drain_removed_meshes() {
        vaos.remove(runtime.buffers.as_ref(), mesh);
    }
}
fn reload_meshes(watch: Option<Res<MeshWatch>>, meshes: Option<ResMut<AssetMeshLibrary>>) {
    let (Some(watch), Some(mut meshes)) = (watch, meshes) else {
        return;
    };
    let paths: Vec<_> = watch.changed_paths().lock().unwrap().drain().collect();
    for path in paths {
        if let Err(error) = meshes.try_reload_mesh_src(&path) {
            log::error!("网格热重载失败，保留旧资产 {}：{error:#}", path.display());
        }
    }
}
fn reload_colors(
    runtime: Option<Res<AssetLibraryRuntime>>,
    watch: Option<Res<ConfigWatch>>,
    ssbos: Option<Res<AssetSsboLibrary>>,
    palette: Option<ResMut<ColorLibrarySsbo>>,
) {
    let (Some(runtime), Some(watch), Some(ssbos), Some(mut palette)) =
        (runtime, watch, ssbos, palette)
    else {
        return;
    };
    if watch.take_pending().is_none() {
        return;
    }
    if let Err(error) = try_update_color_library_in(
        &runtime.assets_root,
        runtime.theme,
        &ssbos,
        runtime.buffers.as_ref(),
        &mut palette,
    ) {
        log::error!("颜色热重载失败，保留旧调色板：{error:#}");
    }
}
fn reload_textures(
    runtime: Option<Res<AssetLibraryRuntime>>,
    watch: Option<Res<TextureWatch>>,
    textures: Option<ResMut<AssetTextureLibrary>>,
) {
    let (Some(runtime), Some(watch), Some(mut textures)) = (runtime, watch, textures) else {
        return;
    };
    let names: Vec<_> = watch.changed().lock().unwrap().drain().collect();
    for name in names {
        let result = (|| -> anyhow::Result<()> {
            let path = runtime
                .compiled_texture_root
                .join(name.as_str())
                .with_extension(crate::render::load_texture::COMPILED_TEXTURE_EXT);
            let file = crate::render::load_texture::AlignedFile::read(&path)?;
            let image = crate::render::load_texture::try_parse_compiled_image(file.bytes(), &path)?;
            let texture =
                runtime
                    .textures
                    .create_texture(name.as_str(), &image.desc, image.data)?;
            match textures.replace(NameHash::compute(name.as_str()), texture.clone()) {
                Ok(previous) => runtime.textures.release_texture(&previous),
                Err(error) => {
                    runtime.textures.release_texture(&texture);
                    return Err(error.into());
                }
            }
            Ok(())
        })();
        if let Err(error) = result {
            log::error!("纹理热重载失败，保留旧资产 {name}：{error:#}");
        }
    }
}
/// 实体、CPU 网格身份和已经发布的 VAO 身份。
type MeshVaoComponents = (Entity, &'static Handle<Mesh>, Option<&'static Handle<Vao>>);

fn rebuild_mesh_vaos(
    runtime: Option<Res<AssetLibraryRuntime>>,
    meshes: Option<ResMut<AssetMeshLibrary>>,
    vaos: Option<ResMut<AssetVAOLibrary>>,
    mesh_entities: Query<MeshVaoComponents>,
    mut commands: Commands,
) {
    let (Some(runtime), Some(mut meshes), Some(mut vaos)) = (runtime, meshes, vaos) else {
        return;
    };
    let mut changed = std::mem::take(&mut meshes.modified_meshes);
    changed.sort_unstable();
    changed.dedup();
    changed.retain(|mesh| {
        if meshes.get(*mesh).is_none() {
            return false;
        }
        if !vaos.has_vao(*mesh) {
            return false;
        }
        match vaos.rebuild_vao(runtime.buffers.as_ref(), &meshes, *mesh) {
            Ok(()) => false,
            Err(error) => {
                log::error!("VAO 重建失败，将重试 {mesh:?}：{error:#}");
                true
            }
        }
    });
    meshes.modified_meshes = changed;
    for (entity, mesh, current_vao) in &mesh_entities {
        let usable = meshes
            .get(*mesh)
            .is_some_and(|mesh| !mesh.indices.is_empty());
        if !usable {
            vaos.remove(runtime.buffers.as_ref(), *mesh);
            if current_vao.is_some() {
                commands.entity(entity).remove::<Handle<Vao>>();
            }
            continue;
        }
        match vaos.add(runtime.buffers.as_ref(), &meshes, *mesh) {
            Ok(vao) if current_vao.copied() != Some(vao) => {
                commands.entity(entity).insert(vao);
            }
            Ok(_) => {}
            Err(error) => log::error!("VAO 创建失败，将重试 {mesh:?}：{error:#}"),
        }
    }
}
