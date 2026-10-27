//! Shader library and raster programs.
use crate::Result;
use crate::device::Device;
use std::collections::BTreeMap;
use std::ffi::CString;
use std::sync::Arc;
use super::interface::ReflectedShader;
use super::interface::ShaderInterfacePlan;
use super::layout::PipelineLayout;
use super::layout::invalid_shader;
use super::pipeline::ComputePipelineDesc;
use super::pipeline::VulkanPipeline;
use super::pipeline_cache::PipelineCache;
use super::raster::RasterPipelineDesc;
use super::reflection::validate_compute_reflection;

/// 按资产键选择真实 shader，自动生产布局与计算管线；不假定文件名就是 country-core pass 键。
pub struct ShaderLibrary {
    pub cache: PipelineCache,
    shaders: BTreeMap<String, ReflectedShader>,
    layouts: BTreeMap<String, Arc<PipelineLayout>>,
    compiled_rasters:
        BTreeMap<(String, String), Arc<shader_pipeline::compiled::CompiledShaderPipeline>>,
    compiled_computes: BTreeMap<String, Arc<shader_pipeline::compiled::CompiledShaderPipeline>>,
}

pub struct RasterProgram {
    pub vertex: ReflectedShader,
    pub fragment: ReflectedShader,
    pub layout: Arc<PipelineLayout>,
}

impl RasterProgram {
    /// 图形固定状态仍由材质/附件消费者提供；shader/entry/layout 则由资产选择生产。
    pub fn pipeline_desc(&self, mut state: RasterPipelineDesc) -> Result<RasterPipelineDesc> {
        state.vertex = self.vertex.module.clone();
        state.fragment = Some(self.fragment.module.clone());
        state.layout = self.layout.clone();
        state.entry =
            CString::new(self.vertex.entry.entry_point.as_str()).map_err(|_| invalid_shader())?;
        state.fragment_entry = Some(
            CString::new(self.fragment.entry.entry_point.as_str()).map_err(|_| invalid_shader())?,
        );
        Ok(state)
    }
}

impl ShaderLibrary {
    pub fn new(device: Arc<Device>) -> Result<Self> {
        Ok(Self {
            cache: PipelineCache::new(device, &[])?,
            shaders: BTreeMap::new(),
            layouts: BTreeMap::new(),
            compiled_rasters: BTreeMap::new(),
            compiled_computes: BTreeMap::new(),
        })
    }
    pub fn register(&mut self, key: String, shader: ReflectedShader) -> Result<()> {
        if !Arc::ptr_eq(&self.cache.device, &shader.module.device) {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        self.layouts.clear();
        self.cache.invalidate();
        self.compiled_computes.remove(&key);
        self.compiled_rasters
            .retain(|(vertex, fragment), _| vertex != &key && fragment != &key);
        self.shaders.insert(key, shader);
        Ok(())
    }
    pub fn create_compute_pipeline(
        &mut self,
        key: &str,
        samplers: &BTreeMap<String, Arc<crate::resource::sampler::Sampler>>,
    ) -> Result<Arc<VulkanPipeline>> {
        let runtime_device_scalar = self
            .cache
            .device
            .max_descriptor_set_update_after_bind_sampled_images();
        use ash::vk::Handle;
        if samplers
            .values()
            .any(|sampler| !Arc::ptr_eq(&self.cache.device, &sampler.device))
        {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let shader = self.shaders.get(key).ok_or_else(invalid_shader)?;
        if shader.entry.stage != "GLCompute" {
            return Err(invalid_shader());
        }
        // sampler 句柄属于布局键；修改 sampler 配置不能复用旧 layout。
        let sampler_map = samplers
            .iter()
            .map(|(name, sampler)| (name.clone(), sampler.handle.as_raw()))
            .collect::<BTreeMap<_, _>>();
        let mut by_name = BTreeMap::new();
        for descriptor in &shader.entry.descriptors {
            if descriptor.descriptor_type == "Sampler" {
                let handle = sampler_map
                    .get(&descriptor.name)
                    .copied()
                    .ok_or_else(|| crate::Error::invalid_usage("着色器资源、接口或设备不匹配"))?;
                by_name.insert(descriptor.name.clone(), handle);
            }
        }
        let plan = if let Some(compiled) = self.compiled_computes.get(key) {
            // Compute reflection checks: no stage inputs, known local size.
            validate_compute_reflection(&compiled.reflection)?;
            ShaderInterfacePlan::from_compiled_pipeline(compiled, &by_name, runtime_device_scalar)?
        } else {
            ShaderInterfacePlan::from_reflection(
                std::slice::from_ref(&shader.entry),
                &by_name,
                runtime_device_scalar,
            )?
        };
        let layout_key = serde_json::to_string(&(key, &plan.sets, runtime_device_scalar))
            .map_err(|error| crate::Error::invalid_usage_with("无法编码计算管线布局键", error))?;
        let layout = match self.layouts.get(&layout_key) {
            Some(layout) => layout.clone(),
            None => {
                let retained = samplers.values().cloned().collect::<Vec<_>>();
                let layout = plan.create_pipeline_layout(self.cache.device.clone(), &retained)?;
                self.layouts.insert(layout_key, layout.clone());
                layout
            }
        };
        VulkanPipeline::create_compute_pipeline(
            &self.cache,
            ComputePipelineDesc {
                shader: shader.module.clone(),
                entry: CString::new(shader.entry.entry_point.as_str())
                    .map_err(|_| invalid_shader())?,
                layout,
            },
        )
    }
    pub fn invalidate(&mut self, key: &str) {
        self.shaders.remove(key);
        self.compiled_computes.remove(key);
        self.compiled_rasters
            .retain(|(vertex, fragment), _| vertex != key && fragment != key);
        self.layouts.clear();
        self.cache.invalidate();
    }
    pub fn prepare_raster_program(
        &mut self,
        vertex_key: &str,
        fragment_key: &str,
        samplers: &BTreeMap<String, Arc<crate::resource::sampler::Sampler>>,
    ) -> Result<RasterProgram> {
        self.prepare_raster_program_with_runtime_limit(vertex_key, fragment_key, samplers, u32::MAX)
    }
    /// Bound runtime arrays to the consumer's real resolver capacity. The
    /// device limit and Vulkan layout-support checks remain authoritative.
    pub fn prepare_raster_program_with_runtime_limit(
        &mut self,
        vertex_key: &str,
        fragment_key: &str,
        samplers: &BTreeMap<String, Arc<crate::resource::sampler::Sampler>>,
        max_runtime_descriptors: u32,
    ) -> Result<RasterProgram> {
        if max_runtime_descriptors == 0 {
            return Err(crate::Error::invalid_usage(
                "runtime descriptor capacity must be nonzero",
            ));
        }
        let runtime_device_scalar = self
            .cache
            .device
            .max_descriptor_set_update_after_bind_sampled_images()
            .min(max_runtime_descriptors.saturating_add(32));
        use ash::vk::Handle;
        if samplers
            .values()
            .any(|sampler| !Arc::ptr_eq(&self.cache.device, &sampler.device))
        {
            return Err(crate::Error::invalid_usage("着色器资源、接口或设备不匹配"));
        }
        let vertex = self
            .shaders
            .get(vertex_key)
            .cloned()
            .ok_or_else(invalid_shader)?;
        let fragment = self
            .shaders
            .get(fragment_key)
            .cloned()
            .ok_or_else(invalid_shader)?;
        if vertex.entry.stage != "Vertex" || fragment.entry.stage != "Fragment" {
            return Err(invalid_shader());
        }
        let sampler_map = samplers
            .iter()
            .map(|(name, sampler)| (name.clone(), sampler.handle.as_raw()))
            .collect();
        let plan = if let Some(compiled) = self
            .compiled_rasters
            .get(&(vertex_key.to_owned(), fragment_key.to_owned()))
        {
            ShaderInterfacePlan::from_compiled_pipeline(
                compiled,
                &sampler_map,
                runtime_device_scalar,
            )?
        } else {
            ShaderInterfacePlan::from_reflection(
                &[vertex.entry.clone(), fragment.entry.clone()],
                &sampler_map,
                runtime_device_scalar,
            )?
        };
        let key = serde_json::to_string(&(
            vertex_key,
            fragment_key,
            &plan.sets,
            runtime_device_scalar,
        ))
        .map_err(|error| crate::Error::invalid_usage_with("无法编码图形管线布局键", error))?;
        let layout = match self.layouts.get(&key) {
            Some(layout) => layout.clone(),
            None => {
                let retained = samplers.values().cloned().collect::<Vec<_>>();
                let layout = plan.create_pipeline_layout(self.cache.device.clone(), &retained)?;
                self.layouts.insert(key, layout.clone());
                layout
            }
        };
        Ok(RasterProgram {
            vertex,
            fragment,
            layout,
        })
    }
}

impl ShaderLibrary {
    /// 资产owner提供选择键；原存储字段保留至实际layout生产。
    pub fn register_compiled_compute(&mut self, key: String, bytes: &[u8]) -> Result<()> {
        use shader_pipeline::compiled::{CompiledShaderPipeline, ShaderStage};
        let compiled = CompiledShaderPipeline::decode(bytes).map_err(|_| invalid_shader())?;
        if compiled.stages.len() != 1 || compiled.stages[0].stage != ShaderStage::Compute {
            return Err(invalid_shader());
        }
        let shader =
            ReflectedShader::from_compiled_pipeline_stage(self.cache.device.clone(), &compiled, 0)?;
        self.register(key.clone(), shader)?;
        self.compiled_computes.insert(key, Arc::new(compiled));
        Ok(())
    }
    /// 两个stage创建成功后一起注册；fragment失败不破坏旧资产/缓存。
    pub fn register_compiled_raster(
        &mut self,
        vertex_key: String,
        fragment_key: String,
        bytes: &[u8],
    ) -> Result<()> {
        use shader_pipeline::compiled::{CompiledShaderPipeline, ShaderStage};
        let compiled = CompiledShaderPipeline::decode(bytes).map_err(|_| invalid_shader())?;
        if vertex_key == fragment_key || compiled.stages.len() != 2 {
            return Err(invalid_shader());
        }
        let vertex_index = compiled
            .stages
            .iter()
            .position(|stage| stage.stage == ShaderStage::Vertex)
            .ok_or_else(invalid_shader)?;
        let fragment_index = compiled
            .stages
            .iter()
            .position(|stage| stage.stage == ShaderStage::Fragment)
            .ok_or_else(invalid_shader)?;
        let vertex = ReflectedShader::from_compiled_pipeline_stage(
            self.cache.device.clone(),
            &compiled,
            vertex_index,
        )?;
        let fragment = ReflectedShader::from_compiled_pipeline_stage(
            self.cache.device.clone(),
            &compiled,
            fragment_index,
        )?;
        self.register(vertex_key.clone(), vertex)?;
        self.register(fragment_key.clone(), fragment)?;
        self.compiled_rasters
            .insert((vertex_key, fragment_key), Arc::new(compiled));
        Ok(())
    }
}
