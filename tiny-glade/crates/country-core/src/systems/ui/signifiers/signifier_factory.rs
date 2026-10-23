//! 指示符拥有工厂与实体构建器，保留既有 Arc 闭包工厂接口。
//!
//! B PDB486 的 new274B/build3840B 全范围原字节支撑网格五分支、
//! 动画/scale/pass、严格资源查找和 TransientMesh 生命周期。实体身份
//! 属于 signifier_create；本构建器不生成假 SignifierId。

use std::sync::Arc;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::*;

use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::asset_libraries::shader_library::AssetShaderLibrary;

use super::animation::SignifierAnimation;

/// 原生唤醒签名（O：vtable 0x142ae70d0 / 0x142efa150 的 call 槽）。
///
/// 记录 `{name: String, inner: Arc<dyn Fn(..) -> Vec<Entity> + Send + Sync>}` 的
/// 第 4/5 qword 就是该胖指针的 `{data, vtable}`；消费端 `signifier_create`
/// （0x140b2cfa0）在去重后调用 vtable 槽唤醒它。对应 `Arc::drop_slow` 单态
/// 0x14064f9d0（`dyn Fn(tuple<ref AssetMeshLibrary, ref AssetShaderLibrary,
/// ref_mut Commands>, Output = Vec<Entity>) + Send + Sync`）。
pub type SignifierClosure = dyn Fn(&AssetMeshLibrary, &AssetShaderLibrary, &mut Commands<'_, '_>) -> Vec<Entity>
    + Send
    + Sync;

/// `SignifierFactory` — 40 B 指示符记录容器（O 0x14089d8b0）。
///
/// 布局（O，fresh）：`{name_len@+0, name_ptr@+8, name_len@+0x10, &inner@+0x18,
/// marker@+0x20}`。前 24 B 是 `String`（`{len, ptr, len}`；`new` 把 `&str` 拷贝成
/// 自有分配），后 16 B 是 `Arc<dyn Fn(..) -> Vec<Entity>>` 的 `{data, vtable}`。
///
/// crate 侧把闭包所有权保留在 [`Self::closure`]（不在 40 B 记录内），使
/// `write_factory`/`wake` 无需从裸 vtable 指针重建类型；裸 `inner`/`marker` 字段
/// 仍与原生 5-qword 视图逐位对应（`new` 的旧调用点使用）。
#[derive(Default)]
pub struct SignifierFactory {
    /// @+0：载荷/名字字节长度。
    pub name_len: usize,
    /// @+8：拷贝后的载荷指针。
    pub name_ptr: *mut u8,
    /// @+0x10：长度副本。
    pub name_len_dup: usize,
    /// @+0x18：内部记录/消费端指针。
    pub inner: *const u8,
    /// @+0x20：closure 标记/vtable。
    pub marker: u64,
    /// crate 侧闭包槽（非 40 B 记录）：`from_closure` 时持有真实唤醒闭包。
    pub closure: Option<Arc<SignifierClosure>>,
}

impl Clone for SignifierFactory {
    /// 深拷贝载荷（避免派生 `Clone` 对裸指针 + `Drop` 的双重释放），并共享闭包
    /// （原生记录经 `Arc` 克隆，引用计数 +1）。
    fn clone(&self) -> Self {
        // Native Option<&str> distinguishes None from Some(""). Preserve it
        // across String/closure cloning rather than normalizing both to [].
        let mut cloned = if self.name_ptr.is_null() {
            let mut empty = Self::default();
            empty.inner = self.inner;
            empty.marker = self.marker;
            empty
        } else {
            Self::new(self.payload(), self.inner, self.marker)
        };
        cloned.closure = self.closure.clone();
        cloned
    }
}

impl std::fmt::Debug for SignifierFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignifierFactory")
            .field("name_len", &self.name_len)
            .field("name_ptr", &self.name_ptr)
            .field("name_len_dup", &self.name_len_dup)
            .field("inner", &self.inner)
            .field("marker", &self.marker)
            .field("has_closure", &self.has_closure())
            .finish()
    }
}

// SAFETY: 原始类型把载荷指针置于共享记录；crate 内以 `Box<[u8]>` 持有实际字节，指针字段
// 仅用于布局对齐（本类型不直接 deref 载荷）。供 `SignifierEntityBuilder` 与
// `UiSignifierStream` 消费面使用。
unsafe impl Send for SignifierFactory {}
unsafe impl Sync for SignifierFactory {}

impl SignifierFactory {
    /// `SignifierFactory::new`（0x14089d8b0，O）：`memcpy` 载荷字节到新建分配，
    /// 写 40B `{len, ptr, len, &inner, marker}`。载荷以 `Box<[u8]>` 持有（分配由本类型
    /// 接管，与原版 `rust_alloc` + 拷贝等价）；`name_ptr` 指向该盒。
    pub fn new(payload: &[u8], inner: *const u8, marker: u64) -> Self {
        let boxed: Box<[u8]> = payload.into();
        let ptr = Box::into_raw(boxed) as *mut u8;
        let len = payload.len();
        Self {
            name_len: len,
            name_ptr: ptr,
            name_len_dup: len,
            inner,
            marker,
            closure: None,
        }
    }

    /// 从真实闭包构造原生记录（`{name String, inner Arc<dyn Fn..>}`，40 B）。
    ///
    /// 名字仍按 `new` 的语义拷贝为自有字节串；闭包所有权进入 crate 侧
    /// [`Self::closure`] 槽，供 [`Self::wake`] 唤醒。原生两处记录构造
    /// （`DebugLine::show` 0x140b04f70 与 glade-border 0x141be0e73）都经
    /// `SignifierFactory::new` 的等价内联序列写同样的 5 qword。
    pub fn from_closure(name: &str, closure: Arc<SignifierClosure>) -> Self {
        let mut factory = Self::new(name.as_bytes(), std::ptr::null(), 0);
        factory.closure = Some(closure);
        factory
    }

    /// 是否携带可唤醒闭包（原生闭包记录 vs crate 旧 raw 5-qword 视图）。
    pub fn has_closure(&self) -> bool {
        self.closure.is_some()
    }

    /// 名字载荷（`from_closure`/`new` 均存字节串；非 UTF-8 返回 None）。
    pub fn name(&self) -> Option<&str> {
        if self.name_ptr.is_null() {
            None
        } else {
            Some(std::str::from_utf8(self.payload()).expect("native signifier name is UTF-8"))
        }
    }

    /// 唤醒闭包（O：`signifier_create` 0x140b2cfa0 的 vtable 调用槽；
    /// 参数 `(&AssetMeshLibrary, &AssetShaderLibrary, &mut Commands)`，返回
    /// `Vec<Entity>`）。无闭包的 raw 记录返回空列表——旧 crate 调用点不参与唤醒。
    pub fn wake(
        &self,
        mesh_library: &AssetMeshLibrary,
        shader_library: &AssetShaderLibrary,
        commands: &mut Commands<'_, '_>,
    ) -> Vec<Entity> {
        match &self.closure {
            Some(closure) => closure(mesh_library, shader_library, commands),
            None => Vec::new(),
        }
    }

    /// 载荷长度（O：@+0）。
    pub fn len(&self) -> usize {
        self.name_len
    }

    /// 载荷字节（I：`{len,ptr,len}` 记录的安全读面）。
    pub fn payload(&self) -> &[u8] {
        if self.name_ptr.is_null() {
            return &[];
        }
        // SAFETY: 指针来自 `new` 的 `Box<[u8]>`，长度记录在 `name_len`。
        unsafe { std::slice::from_raw_parts(self.name_ptr, self.name_len) }
    }

    /// 载荷是否为空（O：`len == 0`）。
    pub fn is_empty(&self) -> bool {
        self.name_len == 0
    }
}

impl Drop for SignifierFactory {
    /// 释放 `new` 里 `Box::into_raw` 的载荷盒（与原版 `rust_dealloc` 对应）。
    fn drop(&mut self) {
        if !self.name_ptr.is_null() {
            // SAFETY: 指针来自 `Box::into_raw`，类型与长度以 `name_len` 记录。
            unsafe {
                drop(Box::from_raw(std::slice::from_raw_parts_mut(
                    self.name_ptr,
                    self.name_len,
                )));
            }
            self.name_ptr = std::ptr::null_mut();
        }
    }
}

/// 原作 SignifierMesh 的五种网格来源；全部数据由正常 Rust 所有者管理。
pub enum SignifierMesh {
    Named(String),
    Empty,
    Points(Vec<glam::Vec3>),
    PointsWithColors {
        points: Vec<glam::Vec3>,
        colors: Vec<glam::Vec3>,
    },
    Generated(Box<dyn Fn() -> Vec<glam::Vec3> + Send + Sync>),
}

impl std::fmt::Debug for SignifierMesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Named(name) => f.debug_tuple("Named").field(name).finish(),
            Self::Empty => f.write_str("Empty"),
            Self::Points(points) => f.debug_tuple("Points").field(points).finish(),
            Self::PointsWithColors { points, colors } => f
                .debug_struct("PointsWithColors")
                .field("points", points)
                .field("colors", colors)
                .finish(),
            Self::Generated(_) => f.write_str("Generated(..)"),
        }
    }
}

/// B 14089d9b0 / 14089dd50：解析真实网格、着色器并建立可回收的绘制实体。
/// 使用原五分支的安全表示，不把应用对象伪装成可跨生命周期解引用的 qword。
#[derive(Debug)]
pub struct SignifierEntityBuilder {
    mesh: SignifierMesh,
    rotation: glam::Quat,
    position: glam::Vec3,
    shader: String,
    animation: SignifierAnimation,
    explicit_scale: Option<f32>,
    minimum_lifespan: Option<f32>,
    parent_to_volume_origin: bool,
    wireframe: bool,
    ui_pass: bool,
}

impl SignifierEntityBuilder {
    pub fn new(mesh: SignifierMesh) -> Self {
        Self {
            mesh,
            rotation: glam::Quat::IDENTITY,
            position: glam::Vec3::ZERO,
            shader: "vertex_color_shader".to_owned(),
            animation: SignifierAnimation {
                head: 1u64.to_le_bytes(),
                flag_8: 0,
                flag_9: 1,
                value: 1.0,
                duration: 0.2,
                t: 0.0,
                eased: 0.0,
                ..Default::default()
            },
            explicit_scale: None,
            minimum_lifespan: None,
            parent_to_volume_origin: false,
            wireframe: false,
            ui_pass: true,
        }
    }

    pub fn from_mesh_name(name: impl Into<String>) -> Self {
        Self::new(SignifierMesh::Named(name.into()))
    }

    pub fn from_points(points: Vec<glam::Vec3>) -> Self {
        Self::new(SignifierMesh::Points(points))
    }

    pub fn position(mut self, position: glam::Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn rotation(mut self, rotation: glam::Quat) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.explicit_scale = Some(scale);
        self
    }

    pub fn animation_duration(mut self, duration: f32) -> Self {
        self.animation.duration = duration;
        self
    }

    pub fn custom_shader(mut self, name: &str) -> Self {
        self.shader = name.to_owned();
        self
    }

    pub fn shader_name(&self) -> &str {
        &self.shader
    }

    /// 原作写 pass ^ 1，并仅在该字节为零时插入 UiPass。
    pub fn set_ui_pass(mut self, pass: u8) -> Self {
        self.ui_pass = (pass ^ 1) == 0;
        self
    }

    pub fn mark_as_debug(mut self) -> Self {
        self.wireframe = true;
        self
    }

    pub fn disable_default_animation(mut self) -> Self {
        self.animation.flag_8 = 1;
        self
    }

    pub fn minimum_lifespan(mut self, lifespan: f32) -> Self {
        self.minimum_lifespan = Some(lifespan);
        self
    }

    pub fn parent_to_widget(mut self) -> Self {
        self.parent_to_volume_origin = true;
        self
    }

    pub fn animation_easing(mut self, easing: ::utils::easing_functions::EasingFunction) -> Self {
        use ::utils::easing_functions::EasingFunction as E;
        let (tag, value): (u32, f32) = match easing {
            E::CircularInOut => (0, 0.0),
            E::CubicInOut => (1, 0.0),
            E::SineInOut => (2, 0.0),
            E::Bias(value) => (3, value),
            E::ElasticInOut => (4, 0.0),
            E::BackOut => (5, 0.0),
            E::CubicOut => (6, 0.0),
        };
        self.animation.head[..4].copy_from_slice(&tag.to_le_bytes());
        self.animation.head[4..].copy_from_slice(&value.to_le_bytes());
        self
    }

    pub fn build(
        mut self,
        meshes: &AssetMeshLibrary,
        shaders: &AssetShaderLibrary,
        commands: &mut Commands,
    ) -> Entity {
        use super::{
            animation::SignifierParentToVolumeOrigin,
            signifier_mark_for_delete::SignifierMinimumLifespan,
        };
        use crate::{
            asset_libraries::{mesh_library::MeshAsset, transient_mesh::TransientMesh},
            components::drawable::DrawableMeshBundle,
            render::mesh::{ATTR_COLOR, Mesh},
            systems::render_loop::draw_passes::draw_lists::{
                DontCastShadow, UiPass, WireframePass,
            },
            utils::{components::transform::Transform, geo::mesh_from_pts},
        };

        let (mesh, transient) = match self.mesh {
            SignifierMesh::Named(name) => (meshes.get_handle_by_name(&name), false),
            SignifierMesh::Empty => (
                meshes.add(MeshAsset::new(Mesh::new("signifier empty"), None)),
                true,
            ),
            SignifierMesh::Points(points) => (
                meshes.add(MeshAsset::new(
                    mesh_from_pts("signifier custom", &points),
                    None,
                )),
                true,
            ),
            SignifierMesh::PointsWithColors { points, colors } => {
                let mut mesh = mesh_from_pts("signifier custom w colors", &points);
                mesh.set_attribute(ATTR_COLOR, colors);
                (meshes.add(MeshAsset::new(mesh, None)), true)
            }
            SignifierMesh::Generated(make_points) => {
                let points = make_points();
                (
                    meshes.add(MeshAsset::new(
                        mesh_from_pts("signifier custom fn", &points),
                        None,
                    )),
                    true,
                )
            }
        };
        // 原顺序为网格解析 → 预留实体 → 着色器查找；缺资源仍失败。
        let entity = commands.spawn_empty().id();
        let scale = self.explicit_scale.unwrap_or(1.0);
        let initial_scale = if self.animation.flag_8 == 0 {
            self.animation.value = scale;
            0.0
        } else {
            scale
        };
        let shader = shaders.get_raster_handle_by_name(&self.shader).unwrap();
        commands.entity(entity).insert((
            DrawableMeshBundle {
                mesh,
                shader,
                transform: Transform {
                    rotation: self.rotation,
                    translation: self.position,
                    scale: glam::Vec3::splat(initial_scale),
                },
            },
            self.animation,
        ));
        if self.wireframe {
            commands.entity(entity).insert(WireframePass);
        } else if self.ui_pass {
            commands.entity(entity).insert(UiPass);
        }
        commands.entity(entity).insert(DontCastShadow);
        if let Some(min_lifespan) = self.minimum_lifespan {
            commands.entity(entity).insert(SignifierMinimumLifespan {
                t: 0.0,
                min_lifespan,
            });
        }
        if self.parent_to_volume_origin {
            commands
                .entity(entity)
                .insert(SignifierParentToVolumeOrigin);
        }
        if transient {
            commands
                .entity(entity)
                .insert(TransientMesh::new_with_registry(
                    mesh,
                    meshes.transient_meshes(),
                ));
        }
        // 名字哈希的身份由 signifier_create 为工厂返回的实体赋予。
        entity
    }
}

#[cfg(test)]
mod complete_builder_tests {
    use super::super::signifier_create::SignifierId;
    use super::*;
    use crate::{
        asset_libraries::{
            mesh_library::MeshAsset,
            transient_mesh::{TransientMesh, delete_dropped_transient_meshes},
            vao_library::AssetVAOLibrary,
        },
        render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
        systems::render_loop::draw_passes::draw_lists::{DontCastShadow, UiPass, WireframePass},
        utils::components::transform::Transform,
    };
    use bevy_ecs::world::CommandQueue;
    use glam::{Quat, Vec3};

    fn libraries() -> (AssetMeshLibrary, AssetShaderLibrary) {
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(Mesh::new("named"), Some("brush".to_owned())));
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(1),
            ),
            false,
        );
        (meshes, shaders)
    }

    fn build(
        builder: SignifierEntityBuilder,
        meshes: &AssetMeshLibrary,
        shaders: &AssetShaderLibrary,
        world: &mut World,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let entity = builder.build(meshes, shaders, &mut Commands::new(&mut queue, world));
        queue.apply(world);
        entity
    }

    #[test]
    fn named_mesh_keeps_library_ownership_and_original_animation_defaults() {
        let (meshes, shaders) = libraries();
        let mut world = World::new();
        let rotation = Quat::from_rotation_y(0.7);
        let position = Vec3::new(1.0, 2.0, 3.0);
        let entity = build(
            SignifierEntityBuilder::from_mesh_name("brush")
                .animation_duration(0.0)
                .rotation(rotation)
                .position(position)
                .scale(1.8),
            &meshes,
            &shaders,
            &mut world,
        );
        let transform = world.get::<Transform>(entity).unwrap();
        assert_eq!(transform.rotation, rotation);
        assert_eq!(transform.translation, position);
        assert_eq!(transform.scale, Vec3::ZERO);
        assert_eq!(
            *world
                .get::<crate::asset_libraries::Handle<Mesh>>(entity)
                .unwrap(),
            meshes.get_handle_by_name("brush"),
        );
        assert!(world.get::<UiPass>(entity).is_some());
        assert!(world.get::<DontCastShadow>(entity).is_some());
        assert!(world.get::<TransientMesh>(entity).is_none());
        assert!(world.get::<SignifierId>(entity).is_none());
        let mut animation = world.get_mut::<SignifierAnimation>(entity).unwrap();
        assert_eq!(animation.head, 1u64.to_le_bytes());
        assert_eq!(animation.flag_9, 1);
        animation.advance(0.0, false);
        assert_eq!(
            (animation.t, animation.eased, animation.value),
            (1.0, 1.0, 1.8)
        );
    }

    #[test]
    fn all_generated_mesh_branches_keep_and_reclaim_their_actual_receipt() {
        let (mut meshes, shaders) = libraries();
        let mut vaos = AssetVAOLibrary::new();
        let mut world = World::new();
        let points = vec![Vec3::ZERO, Vec3::X];
        let sources = [
            SignifierMesh::Empty,
            SignifierMesh::Points(points.clone()),
            SignifierMesh::PointsWithColors {
                points: points.clone(),
                colors: vec![Vec3::Y, Vec3::Z],
            },
            SignifierMesh::Generated(Box::new(move || points.clone())),
        ];
        for (source, label) in sources.into_iter().zip([
            "signifier empty",
            "signifier custom",
            "signifier custom w colors",
            "signifier custom fn",
        ]) {
            let entity = build(
                SignifierEntityBuilder::new(source)
                    .disable_default_animation()
                    .scale(2.0)
                    .set_ui_pass(0),
                &meshes,
                &shaders,
                &mut world,
            );
            let receipt = world.get::<TransientMesh>(entity).unwrap();
            let handle = receipt.mesh_handle();
            assert_eq!(meshes.get(handle).unwrap().label, label);
            let mesh = meshes.get(handle).unwrap();
            if label == "signifier empty" {
                assert!(mesh.indices.is_empty());
                assert!(mesh.attributes.is_empty());
            } else {
                assert_eq!(mesh.positions(), (&[0, 1][..], &[Vec3::ZERO, Vec3::X][..]));
                let crate::render::mesh::VertexAttributeValues::Float3(colors) =
                    &mesh.attributes[crate::render::mesh::ATTR_COLOR]
                else {
                    panic!("expected original color channel")
                };
                let expected = if label == "signifier custom w colors" {
                    vec![Vec3::Y, Vec3::Z]
                } else {
                    vec![Vec3::ONE, Vec3::ONE]
                };
                assert_eq!(colors, &expected);
            }

            assert_eq!(
                *world
                    .get::<crate::asset_libraries::Handle<Mesh>>(entity)
                    .unwrap(),
                handle
            );
            assert_eq!(
                world.get::<Transform>(entity).unwrap().scale,
                Vec3::splat(2.0)
            );
            assert!(world.get::<UiPass>(entity).is_none());
            assert!(world.get::<DontCastShadow>(entity).is_some());
            world.despawn(entity);
            assert_eq!(
                meshes.transient_meshes().lock().unwrap().last(),
                Some(&handle)
            );
            delete_dropped_transient_meshes(&TestDevice(0), &mut meshes, &mut vaos);
            assert!(meshes.get(handle).is_none());
        }
        assert!(meshes.get(meshes.get_handle_by_name("brush")).is_some());
    }

    #[test]
    fn wireframe_precedes_ui_and_optional_lifespan_and_parent_are_real_components() {
        let (meshes, shaders) = libraries();
        let mut world = World::new();
        let entity = build(
            SignifierEntityBuilder::from_mesh_name("brush")
                .mark_as_debug()
                .minimum_lifespan(0.7)
                .parent_to_widget(),
            &meshes,
            &shaders,
            &mut world,
        );
        assert!(world.get::<WireframePass>(entity).is_some());
        assert!(world.get::<UiPass>(entity).is_none());
        assert!(
            world
                .get::<super::super::animation::SignifierParentToVolumeOrigin>(entity)
                .is_some()
        );
        let lifespan = world
            .get::<super::super::signifier_mark_for_delete::SignifierMinimumLifespan>(entity)
            .unwrap();
        assert_eq!((lifespan.t, lifespan.min_lifespan), (0.0, 0.7));
    }

    #[test]
    fn generated_callback_runs_once_and_is_dropped_after_its_mesh_is_registered() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct DropCount(Arc<AtomicUsize>);
        impl Drop for DropCount {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }
        let calls = Arc::new(AtomicUsize::new(0));
        let drops = Arc::new(AtomicUsize::new(0));
        let closure_calls = Arc::clone(&calls);
        let capture = DropCount(Arc::clone(&drops));
        let source = SignifierMesh::Generated(Box::new(move || {
            let _ = &capture;
            closure_calls.fetch_add(1, Ordering::Relaxed);
            vec![Vec3::ZERO, Vec3::X]
        }));
        let (meshes, shaders) = libraries();
        build(
            SignifierEntityBuilder::new(source),
            &meshes,
            &shaders,
            &mut World::new(),
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn missing_mesh_fails_before_reservation_and_missing_shader_fails_after_reservation() {
        for (mesh_name, expected_reserved) in [("missing", 0), ("brush", 1)] {
            let (meshes, _) = libraries();
            let shaders = AssetShaderLibrary::new();
            let mut world = World::new();
            let mut queue = CommandQueue::default();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                SignifierEntityBuilder::from_mesh_name(mesh_name).build(
                    &meshes,
                    &shaders,
                    &mut Commands::new(&mut queue, &world),
                );
            }));
            assert!(result.is_err());
            assert_eq!(world.entities().used_count(), expected_reserved);
            queue.apply(&mut world);
        }
    }
}
