//! 共享测试占位设备。
//!
//! `rhapsody_contract::GraphicsDevice` 已由 `pub u64` 占位改为包装真实
//! `rhapsody::device::Device`（WIRING §1），CPU 测试无法构造真实 `Arc<Device>`。本类型以
//! u64 序列号承载原 tuple 占位的测试语义（`id`/句柄即序列号），并实现各 render 层设备 trait。
//! `rhapsody_contract::GraphicsDevice` 自身的 `#[cfg(test)]` trait 实现保留（供
//! `texture_library.rs` 的常开 `TextureLibraryDevice: TextureDevice` 超 trait 编译），但 CPU 测试
//! 统一用本类型构造设备值。

use core::ops::Range;
use std::sync::Arc;

use crate::asset_libraries::texture_library::{
    DescriptorImageUpdate, ImageView, TextureLibraryDevice, TextureUpdate,
};
use crate::render::load_texture::boundary::{self, BackendError, Texture, TextureDesc};
use crate::render::shader::boundary::{
    CompiledShaderPipeline, ComputePipeline, ComputePipelineDesc, Error, RasterPipeline,
    RasterPipelineDesc, RasterPipelineKey, ShaderBackend,
};
use crate::render::ssbo::rhapsody_boundary::{self, Buffer, BufferDesc};
use crate::rhapsody_contract::DescriptorSet;

/// 测试占位设备（u64 = 原 `GraphicsDevice(pub u64)` 占位的序列号；真实设备不可达）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TestDevice(pub u64);

impl rhapsody_boundary::GraphicsDevice for TestDevice {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        _name: &str,
        _data: Option<&[u8]>,
    ) -> Result<Buffer, rhapsody_boundary::BackendError> {
        Ok(Buffer {
            byte_size: desc.size as u64,
            ..Buffer::default()
        })
    }

    fn update_buffer_bytes(
        &self,
        _buffer: &Buffer,
        _data: &[u8],
        _ranges: &[Range<usize>],
        _offset: usize,
    ) -> Result<(), rhapsody_boundary::BackendError> {
        Ok(())
    }

    /// 占位：没有 GPU，持久映射区以泄漏的零页代替（16 B 对齐，长度 = `desc.size`）。
    fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> {
        let words = (buffer.byte_size as usize).div_ceil(16);
        let page: &'static mut [u128] = Box::leak(vec![0u128; words].into_boxed_slice());
        // SAFETY: `u128` 切片按字节重解释；长度截到 `byte_size`，对齐 16 ≥ 任意 `StorageElement` 的对齐。
        Some(unsafe {
            core::slice::from_raw_parts_mut(page.as_mut_ptr() as *mut u8, buffer.byte_size as usize)
        })
    }
}

impl boundary::TextureDevice for TestDevice {
    fn create_texture(
        &self,
        _name: &str,
        desc: &TextureDesc,
        _data: &[u8],
    ) -> Result<Texture, BackendError> {
        Ok(Texture {
            id: self.0,
            width: desc.extent[0],
            height: desc.extent[1],
            format: 0,
        })
    }
}

impl TextureLibraryDevice for TestDevice {
    fn create_persistent_descriptor_set(&self, _name: &str) -> Result<DescriptorSet, BackendError> {
        Ok(DescriptorSet::default())
    }
    fn update_persistent_descriptor_set(
        &self,
        _: &DescriptorSet,
        _: &[DescriptorImageUpdate],
    ) -> Result<(), BackendError> {
        Ok(())
    }
    fn get_vk_image_view(&self, texture: &Texture, _mip: u32) -> Result<ImageView, BackendError> {
        Ok(ImageView(texture.id))
    }
    fn update_texture(
        &self,
        _texture: &Texture,
        _update: &TextureUpdate,
    ) -> Result<(), BackendError> {
        Ok(())
    }
}

impl ShaderBackend for TestDevice {
    fn load_compiled_compute(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<CompiledShaderPipeline, Error> {
        Ok(CompiledShaderPipeline(desc.path.as_bytes().to_vec()))
    }
    fn load_compiled_raster(
        &self,
        desc: &RasterPipelineDesc,
    ) -> Result<CompiledShaderPipeline, Error> {
        Ok(CompiledShaderPipeline(desc.path.as_bytes().to_vec()))
    }
    fn raster_shader_variants(&self, desc: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
        [desc.clone(), desc.clone()]
    }
    fn create_compute_pipeline(
        &self,
        _name: &str,
        _compiled: Arc<CompiledShaderPipeline>,
    ) -> rhapsody::Result<ComputePipeline> {
        Ok(ComputePipeline { id: self.0 })
    }
    fn create_raster_pipeline(
        &self,
        compiled: Arc<CompiledShaderPipeline>,
        key: &RasterPipelineKey,
    ) -> Result<RasterPipeline, Error> {
        Ok(RasterPipeline {
            id: key.selector as u64,
            compiled,
        })
    }
}

/// CPU-only readback readiness adapter. It never claims real GPU completion;
/// production hazard tests live in the native BufferHost suite.
#[derive(Default)]
pub(crate) struct TestReadbackAccess {
    ready: std::sync::atomic::AtomicBool,
    reads: std::sync::atomic::AtomicUsize,
}
impl TestReadbackAccess {
    pub(crate) fn set_ready(&self, ready: bool) {
        self.ready.store(ready, std::sync::atomic::Ordering::Release);
    }
    pub(crate) fn reads(&self) -> usize {
        self.reads.load(std::sync::atomic::Ordering::Acquire)
    }
}
impl rhapsody_boundary::MappedUploadWriter for TestReadbackAccess {
    fn write(&self, _: usize, _: &[u8]) -> Result<(), rhapsody_boundary::BackendError> {
        Err(rhapsody_boundary::BackendError::unsupported("test readback has no native upload"))
    }
    fn with_write(&self, _: usize, _: usize, edit: &mut dyn FnMut()) -> Result<(), rhapsody_boundary::BackendError> {
        edit();
        Ok(())
    }
    fn try_read(&self, _: usize, _: usize, read: &mut dyn FnMut()) -> Result<bool, rhapsody_boundary::BackendError> {
        let ready = self.ready.load(std::sync::atomic::Ordering::Acquire);
        if ready {
            self.reads.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            read();
        }
        Ok(ready)
    }
}
