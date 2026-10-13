//! `boundary` — game-owned render/core types owned by OTHER work-packages (wp2/
//! wp4/wp7), surfaced here as the game-side boundary my WP16 systems reference.
//!
//! These are NOT re-recovered: they already live in the integrating
//! `country-core` crate (`render::mesh::Mesh`, `render::vao::Vao`,
//! `asset_libraries::mesh_library::AssetMeshLibrary`, `render::frame_constants::FrameConstants`,
//! `render::view_constants::ViewConstants`, `resources::world_raster::WorldRaster`,
//! `render::sparse_instance_buffer::SparseInstanceBufferWriter`, …). The
//! integration owner replaces this module's facades with the real types; every
//! field here is a typed gap unless marked O.

#![allow(dead_code)]

use std::collections::HashMap;

use crate::typed_gap::Gap;

/// 网格（wp4 `render::mesh::Mesh`；O：顶点/法线/索引 + 属性表）。
#[derive(Clone, Debug)]
pub struct Mesh {
    pub name: alloc::string::String,
    pub vertices: Vec<glam::Vec3>,
    pub normals: Vec<glam::Vec3>,
    pub indices: Vec<u32>,
    pub _attributes: Gap,
}

/// 顶点属性绑定（wp4；U）。
#[derive(Clone, Debug)]
pub struct VaoBinding(pub Gap);

/// 顶点缓冲对象（wp4 `render::vao::Vao`；O：GPU buffer + 绑定表）。
#[derive(Clone, Debug)]
pub struct Vao {
    pub bindings: Vec<VaoBinding>,
    pub _gap: Gap,
}

/// 网格注册表（wp4 `AssetMeshLibrary`；O：`FrozenMap<Handle<Mesh>, Box<Mesh>>`）。
#[derive(Clone, Debug, Default)]
pub struct AssetMeshLibrary(pub HashMap<u64, Mesh>);

impl AssetMeshLibrary {
    pub fn get_mut(&mut self, _handle: u64) -> Option<&mut Mesh> {
        None
    }
    pub fn has_vao(&self, _handle: u64) -> bool {
        false
    }
}

/// 纹理注册表（wp4 `AssetTextureLibrary`；O：按名注册的纹理）。
#[derive(Clone, Debug, Default)]
pub struct AssetTextureLibrary(pub HashMap<alloc::string::String, Gap>);

/// 帧常量（wp4 `render::frame_constants::FrameConstants`；O：帧序号/视口/Jitter）。
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameConstants {
    pub frame_index: u32,
    pub _gap: Gap,
}

/// 视口常量（wp4 `render::view_constants::ViewConstants`；O：pixel offset 设置）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ViewConstants {
    pub _gap: Gap,
}

/// 渲染配置（wp4 `RenderConfig`；configwatch 热载目标）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderConfig(pub Gap);

/// 世界栅格（wp7 `resources::world_raster::WorldRaster<N>`；路径/水/花园栅格）。
#[derive(Clone, Debug, Default)]
pub struct WorldRaster<const N: usize> {
    /// 数据（O：u8 网格；类型 N=512 栅格）。
    pub data: Vec<u8>,
    pub _gap: Gap,
}

/// 稀疏实例缓冲写入器（wp2 `SparseInstanceBufferWriter<T>`；instanced_wall 上传）。
pub struct SparseInstanceBufferWriter<T> {
    pub _marker: core::marker::PhantomData<fn() -> T>,
    pub _lock: Gap,
}

impl<T> SparseInstanceBufferWriter<T> {
    pub fn clear(&mut self) {}
    pub fn push(&mut self, _instance: Gap) {}
    pub fn gpu_flush(&mut self) {}
}
