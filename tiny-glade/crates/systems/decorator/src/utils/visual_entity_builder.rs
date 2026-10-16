//! B `utils/visual_entity_builder.rs` — 装饰视觉实体构建器（D1 消费契约）。
//!
//! 方法面（PDB 编译单元元数据，O 行号）：new@54、mark_as_preview@84、
//! has_birdspot@89、color_id@99、lantern_light@104、custom_shader@126、
//! albedo_map@131、clothline@147、smoke、flag_roof_cloth、flag_wall_cloth、
//! mark_as_window_glass、`build<DecoratorPersistentVisualState>` /
//! `build<DecoratorTransientVisualState>`（6 个编译实例；Ghidra DB 未建函数，
//! 实参面 = 调用点 + build 实例名，O：build 向状态表 add_decorator_visual_entity）。
//!
//! 状态表对接（O，D3 证据）：瞬态表 = `HashMap<u32, Vec<Entity>>`（条目 0x28B）；
//! 持久表 = `HashMap<u32, PersistentVisualEntry>`（0x20B 值，sealed 标志 +
//! 3×u64 载荷）。实体组件集 U（build 反编译缺失）：D1 以 core 既有组件
//! （Transform + ColorId）与本地标记组件承载，注册于 RESTORE.md §视觉实体组件集。

use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_ecs::world::World;
use country_core::resources::color_id::ColorId;
use glam::{Quat, Vec3};
use utils::components::transform::Transform;

use crate::decorator_visual_state::{
    DecoratorPersistentVisualState, DecoratorTransientVisualState, PersistentVisualEntry,
};

/// 装饰视觉变换组件（D1 本地包裹 utils::Transform；U：原组件身份待取证）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, PartialEq)]
pub struct DecoratorVisualTransform(pub Transform);

/// 窗口玻璃标记组件（U：原组件身份待取证；D1 本地标记）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct WindowGlassMarker;

/// 预览标记组件（mark_as_preview，U：原组件身份待取证）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct DecoratorPreviewMarker;

/// 鸟巢标记组件（has_birdspot，U）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct BirdspotMarker;

/// 自定义着色器 id 组件（custom_shader，U）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoratorShaderId(pub u32);

/// 灯组件（lantern_light，U：光参数载荷待取证）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct LanternLight;

/// 烟组件（smoke，U）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct DecoratorSmoke;

/// 布组件（clothline / flag_roof_cloth / flag_wall_cloth，U：载荷待取证）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DecoratorCloth(pub u8);

/// B WindowVisualEntityBuilder：窗口系装饰实体构建器（new@54 行，61 内联实例）。
/// 字段集 U（build 反编译缺失）；D1 按调用点消费面承载。
#[derive(Clone, Debug, Default)]
pub struct WindowVisualEntityBuilder {
    /// 生成位置。
    pub pos: Vec3,
    /// 生成旋转。
    pub rot: Quat,
    /// 装饰 id。
    pub decorator_id: u32,
    /// 颜色（color_id）。
    pub color: Option<ColorId>,
    /// 着色器（custom_shader）。
    pub shader: Option<u32>,
    /// 反照率贴图（albedo_map，U：资产句柄载荷）。
    pub albedo: Option<u64>,
    /// 预览（mark_as_preview）。
    pub preview: bool,
    /// 窗口玻璃（mark_as_window_glass）。
    pub window_glass: bool,
    /// 布（clothline / flag_roof_cloth / flag_wall_cloth）。
    pub cloth: Option<u8>,
    /// 鸟巢（has_birdspot）。
    pub birdspot: bool,
    /// 灯（lantern_light）。
    pub lantern: bool,
    /// 烟（smoke）。
    pub smoke: bool,
}

impl WindowVisualEntityBuilder {
    /// new（行 54，O 结构）：以位置/旋转/id 构建。
    pub fn new(pos: Vec3, rot: Quat, decorator_id: u32) -> Self {
        Self {
            pos,
            rot,
            decorator_id,
            ..Self::default()
        }
    }

    /// mark_as_preview（行 84）。
    pub fn mark_as_preview(&mut self) {
        self.preview = true;
    }

    /// has_birdspot（行 89）。
    pub fn has_birdspot(&mut self) {
        self.birdspot = true;
    }

    /// color_id（行 99）。
    pub fn color_id(&mut self, color: ColorId) {
        self.color = Some(color);
    }

    /// lantern_light（行 104）。
    pub fn lantern_light(&mut self) {
        self.lantern = true;
    }

    /// custom_shader（行 126）。
    pub fn custom_shader(&mut self, shader: u32) {
        self.shader = Some(shader);
    }

    /// albedo_map（行 131）。
    pub fn albedo_map(&mut self, albedo: u64) {
        self.albedo = Some(albedo);
    }

    /// clothline（行 147）：布（载荷 U）。
    pub fn clothline(&mut self) {
        self.cloth = Some(0);
    }

    /// flag_roof_cloth：屋顶旗布（载荷 U）。
    pub fn flag_roof_cloth(&mut self) {
        self.cloth = Some(1);
    }

    /// flag_wall_cloth：墙旗布（载荷 U）。
    pub fn flag_wall_cloth(&mut self) {
        self.cloth = Some(2);
    }

    /// mark_as_window_glass（行 415 前）。
    pub fn mark_as_window_glass(&mut self) {
        self.window_glass = true;
    }

    /// smoke。
    pub fn smoke(&mut self) {
        self.smoke = true;
    }

    /// build<DecoratorTransientVisualState>：瞬态表消费的实体生成
    /// （组件集 U，D1 承载）。
    pub fn build_transient(
        &self,
        world: &mut World,
        state: &mut DecoratorTransientVisualState,
    ) -> Entity {
        let entity = self.spawn(world);
        // 瞬态表无独立 add 方法（D3 契约：HashMap<u32, Vec<Entity>>，or_default push）。
        state.entities.entry(self.decorator_id).or_default().push(entity);
        entity
    }

    /// build<DecoratorPersistentVisualState>：持久表消费的实体生成。
    /// 载荷 I：payload[0] = 实体位（build 反编译缺失；seal/remove 以 id 寻址）。
    pub fn build_persistent(
        &self,
        world: &mut World,
        state: &mut DecoratorPersistentVisualState,
    ) -> Entity {
        let entity = self.spawn(world);
        state.add_decorator_visual_entity(
            self.decorator_id,
            PersistentVisualEntry {
                sealed: 0,
                payload: [entity.to_bits(), 0, 0],
            },
        );
        entity
    }

    /// 实体生成（组件集 U：Transform + ColorId + 标记组件；U 项见文件头）。
    fn spawn(&self, world: &mut World) -> Entity {
        let mut entity = world.spawn(DecoratorVisualTransform(Transform {
            t: self.pos,
            r: self.rot,
            s: Vec3::ONE,
        }));
        if let Some(color) = self.color {
            entity.insert(color);
        }
        if self.preview {
            entity.insert(DecoratorPreviewMarker);
        }
        if self.window_glass {
            entity.insert(WindowGlassMarker);
        }
        if self.birdspot {
            entity.insert(BirdspotMarker);
        }
        if let Some(shader) = self.shader {
            entity.insert(DecoratorShaderId(shader));
        }
        if self.lantern {
            entity.insert(LanternLight);
        }
        if self.smoke {
            entity.insert(DecoratorSmoke);
        }
        if let Some(cloth) = self.cloth {
            entity.insert(DecoratorCloth(cloth));
        }
        entity.id()
    }

    /// Commands 版本（build 消费面等价；D1 接线）。
    pub fn build_with_commands(
        &self,
        commands: &mut Commands,
        state: &mut DecoratorTransientVisualState,
    ) -> Entity {
        let mut entity = commands.spawn(DecoratorVisualTransform(Transform {
            t: self.pos,
            r: self.rot,
            s: Vec3::ONE,
        }));
        if let Some(color) = self.color {
            entity.insert(color);
        }
        if self.preview {
            entity.insert(DecoratorPreviewMarker);
        }
        if self.window_glass {
            entity.insert(WindowGlassMarker);
        }
        if let Some(shader) = self.shader {
            entity.insert(DecoratorShaderId(shader));
        }
        if self.cloth.is_some() {
            entity.insert(DecoratorCloth(self.cloth.unwrap_or(0)));
        }
        let entity = entity.id();
        state.entities.entry(self.decorator_id).or_default().push(entity);
        entity
    }
}
