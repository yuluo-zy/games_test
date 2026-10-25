//! 窗灯共享组件和唯一状态资源。
//! B 版真实消费者使用 usize 键与 u32 装饰物命令编号。
//! 这里采用安全命名字段，不声明原 Rust 或 Bevy 的内存 ABI。
use bevy_ecs::{component::Component, resource::Resource};
use std::collections::HashMap;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowLight {
    pub key: usize,
    pub decorator_id: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LightInfo {
    pub brightness: f32,
    pub timer: f32,
    pub actual_on: u8,
    pub target_on: u8,
}

impl LightInfo {
    /// 原插入分支明确写入四个字段，其中计时器初值为零。
    pub fn new(lights_on: bool) -> Self {
        Self { brightness: 0.0, timer: 0.0,
            actual_on: lights_on as u8, target_on: lights_on as u8 }
    }
}

#[derive(Resource, Clone, Debug, Default)]
pub struct GlobalWindowLightsControl {
    pub lights: HashMap<usize, LightInfo>,
    pub lights_on: bool,
}
