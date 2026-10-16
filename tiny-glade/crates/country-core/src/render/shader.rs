//! Shader 程序的加载 / 缓存层（layer A1，游戏自有部分）。
//!
//! 真正的 shader 编译器（`shader_pipeline`、`shader_variants`）和 GPU 设备（`rhapsody`）都是外部库，
//! 这里只复刻原函数的**调用顺序、所有权、失败处理与数据布局**，库本身以 `boundary` 占位。
//!
//! 证据（O，逐条核对机器码，见 `evidence/country-core-audit/L0_NOTES.md`）：
//!  * `ComputeShaderProgram::new`            0x140937e10（1061 B）
//!  * `RasterShaderProgram::new_draw`         0x140938240（359 B）
//!  * `compile_shader_variants`               0x1409383b0（1870 B）
//!  * `RasterShaderProgramVariant::get_or_generate` 0x140938b00（2186 B）
//!
//! 与旧版的主要差异：
//!  1. `ComputeShaderProgram::new` 的参数是 `(name: &str, desc: ComputePipelineDesc(0x48 B, 按值), device)`；
//!     加载编译产物失败 → `Result::unwrap` panic（shader.rs:72）；随后 **trace** 级日志
//!     `"Creating pipeline {name}"`（target `country_core::render::shader`，旧版写成 debug）；
//!     `Arc::new` 后调用 `create_compute_pipeline`，其结果不再检查。布局：desc @0、pipeline(0x48 B) @0x48、
//!     name(&str) @0x90，共 0xa0。
//!  2. `compile_shader_variants(desc) -> Result<[RasterShaderProgramVariant; 2], anyhow::Error>`：
//!     `get_raster_shader_variants` 返回**两份** desc；先 `load_compiled(第一份)`，失败直接返回 Err；
//!     再 `load_compiled(第二份)`，失败时丢弃第一份并返回 Err。成功则每份编译产物各配一个空的变体缓存。
//!  3. `RasterShaderProgram`（0xd8 B）= `{ desc @0(0x48), name &str @0x48, variants [2]×0x40 @0x58 }`；
//!     `new_draw` 对 `compile_shader_variants` 失败 `unwrap`（shader.rs:100）。第二个形参未被使用。
//!  4. `RasterShaderProgramVariant`（0x40 B）= `{ compiled: Arc<Compiled> @0, cache: RwLock<HashMap<Key, Box<RasterPipeline>>> @8 }`。
//!     `get_or_generate(&self, device, key)`：读锁查表（命中即返回）→ 写锁 `entry`（并发下可能已被他人填入，同样返回现有项）→
//!     空位时 `Arc` 强引用 +1、`create_raster_pipeline` 失败 `unwrap`（shader.rs:190）、`Box` 后插入。
//!     命中时作为参数传入的 `key` 被丢弃（所有权在调用者）。返回指向 `Box` 内部的引用——表项永不删除。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

/// 外部库占位：`shader_pipeline` / `shader_variants` / `rhapsody::raw::device`。
pub mod boundary {
    use std::sync::Arc;

    pub use crate::rhapsody_types::{
        CompiledShaderPipeline, ComputePipeline, Error, RasterPipeline, RasterPipelineDesc,
        RasterPipelineKey,
    };

    /// `shader_pipeline::pipeline_desc::ComputePipelineDesc`（0x48 B，见 `shader_uri`）。
    pub use crate::render::pass_builder::shader_uri::ComputePipelineDesc;

    /// 外部库在这里被压成一个 trait，方法 = 原函数的外部调用点。
    pub trait ShaderBackend {
        /// `ShaderCompilerArtifacts::load_compiled<ComputePipelineDesc>`（O 0x1409e3c50）。
        fn load_compiled_compute(
            &self,
            desc: &ComputePipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error>;
        /// `ShaderCompilerArtifacts::load_compiled<RasterPipelineDesc>`（O 0x1409e4160）。
        fn load_compiled_raster(
            &self,
            desc: &RasterPipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error>;
        /// `shader_variants::get_raster_shader_variants`（O 0x1417388a0）：两份 desc。
        fn raster_shader_variants(&self, desc: &RasterPipelineDesc) -> [RasterPipelineDesc; 2];
        /// `GraphicsDevice::create_compute_pipeline`（O 0x1416a4b20；结果不检查）。
        fn create_compute_pipeline(
            &self,
            name: &str,
            compiled: Arc<CompiledShaderPipeline>,
        ) -> ComputePipeline;
        /// `GraphicsDevice::create_raster_pipeline`（O 0x1416a3720；调用点 `Result::unwrap`）。
        fn create_raster_pipeline(
            &self,
            compiled: Arc<CompiledShaderPipeline>,
            key: &RasterPipelineKey,
        ) -> Result<RasterPipeline, Error>;
    }
}

use boundary::*;

/// `ComputeShaderProgram`（0xa0 B）。
pub struct ComputeShaderProgram {
    /// +0x00。
    pub desc: ComputePipelineDesc,
    /// +0x48。
    pub pipeline: ComputePipeline,
    /// +0x90。
    pub name: &'static str,
}

impl ComputeShaderProgram {
    /// O 0x140937e10。
    pub fn new(name: &'static str, desc: ComputePipelineDesc, device: &dyn ShaderBackend) -> Self {
        // shader.rs:72 —— `load_compiled(&desc).unwrap()`。
        let compiled = device
            .load_compiled_compute(&desc)
            .expect("called `Result::unwrap()` on an `Err` value");
        // 日志级别立即数 5 = Trace；行 74。
        log::trace!(target: "country_core::render::shader", "Creating pipeline {}", name);
        let pipeline = device.create_compute_pipeline(name, Arc::new(compiled));
        Self {
            desc,
            pipeline,
            name,
        }
    }
}

/// `RasterShaderProgramVariant`（0x40 B）：一份编译产物 + 它派生出的管线缓存。
pub struct RasterShaderProgramVariant {
    /// +0x00。
    pub compiled: Arc<CompiledShaderPipeline>,
    /// +0x08（锁状态）、+0x10（表）、+0x30（SipHash 密钥）。
    cache: RwLock<HashMap<RasterPipelineKey, Box<RasterPipeline>>>,
}

impl RasterShaderProgramVariant {
    fn new(compiled: Arc<CompiledShaderPipeline>) -> Self {
        Self {
            compiled,
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// 已缓存的管线数量（诊断用，I）。
    pub fn cached(&self) -> usize {
        self.cache.read().len()
    }

    /// O 0x140938b00。见模块注释第 4 点。
    pub fn get_or_generate(
        &self,
        device: &dyn ShaderBackend,
        key: RasterPipelineKey,
    ) -> &RasterPipeline {
        if let Some(hit) = self.cache.read().get(&key) {
            return unsafe { &*(&**hit as *const RasterPipeline) };
        }
        let mut map = self.cache.write();
        let slot = match map.entry(key) {
            std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
            std::collections::hash_map::Entry::Vacant(v) => {
                let compiled = Arc::clone(&self.compiled);
                // shader.rs:190 —— `create_raster_pipeline(..).unwrap()`。
                let pipeline = device
                    .create_raster_pipeline(compiled, v.key())
                    .expect("called `Result::unwrap()` on an `Err` value");
                v.insert(Box::new(pipeline))
            }
        };
        // 表项是 `Box`，永不删除、搬动哈希表不会移动其内容，故可在解锁后继续引用。
        unsafe { &*(&**slot as *const RasterPipeline) }
    }
}

/// `compile_shader_variants`（O 0x1409383b0）。
pub fn compile_shader_variants(
    desc: &RasterPipelineDesc,
    device: &dyn ShaderBackend,
) -> Result<[RasterShaderProgramVariant; 2], Error> {
    let [d0, d1] = device.raster_shader_variants(desc);
    let c0 = Arc::new(device.load_compiled_raster(&d0)?);
    let c1 = Arc::new(device.load_compiled_raster(&d1)?);
    Ok([
        RasterShaderProgramVariant::new(c0),
        RasterShaderProgramVariant::new(c1),
    ])
}

/// `RasterShaderProgram`（0xd8 B）。
pub struct RasterShaderProgram {
    /// +0x00。
    pub desc: RasterPipelineDesc,
    /// +0x48。
    pub name: &'static str,
    /// +0x58，两个 0x40 B 的变体。
    pub variants: [RasterShaderProgramVariant; 2],
}

impl RasterShaderProgram {
    /// `new_draw`（O 0x140938240）：编译两个变体；失败 `unwrap`（shader.rs:100）。
    pub fn new_draw(
        name: &'static str,
        desc: RasterPipelineDesc,
        device: &dyn ShaderBackend,
    ) -> Self {
        let variants = compile_shader_variants(&desc, device)
            .expect("called `Result::unwrap()` on an `Err` value");
        Self {
            desc,
            name,
            variants,
        }
    }
}

/// 全 crate 唯一的设备 `rhapsody_contract::GraphicsDevice` 实现 `ShaderBackend`。
/// D 组接入（WIRING §2.2/§3.3）：`load_compiled_compute`/`load_compiled_raster` 经
/// `shader_pipeline::compiled::CompiledShaderPipeline::load_compiled` 真实加载发行 .bin，再 `encode()`
/// 回游戏侧字节容器。管线创建（`create_compute_pipeline`/`create_raster_pipeline`）与
/// `raster_shader_variants` 仍是登记缺口（游戏侧 `ComputePipeline`/`RasterPipeline` 占位装不下
/// `Arc<VulkanPipeline>`，WIRING §4.9；不补造、不假成功）。
#[cfg(not(test))]
impl ShaderBackend for crate::rhapsody_contract::GraphicsDevice {
    fn load_compiled_compute(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<CompiledShaderPipeline, Error> {
        use crate::rhapsody_types::{
            RhapsodyCompiledShaderPipeline, RhapsodyComputePipelineDesc, ShaderDefines,
        };
        // 游戏侧 desc（shader_uri，`{path, entry, defines}`）→ 真实 `pipeline_desc::ComputePipelineDesc`，
        // `compiled_file_path()` 由真实库按 `{path}_{entry}_{defines.artifact_hash()}.bin` 计算（O：0x1409e3ab0）。
        let real = RhapsodyComputePipelineDesc {
            source_path: desc.path.clone(),
            entry_point: desc.entry.clone(),
            defines: ShaderDefines(desc.defines.clone()),
        };
        let compiled = RhapsodyCompiledShaderPipeline::load_compiled(real.compiled_file_path())
            .map_err(anyhow::Error::msg)?;
        let bytes = compiled
            .encode()
            .map_err(|error| anyhow::anyhow!("compiled shader encode failed: {error:?}"))?;
        Ok(CompiledShaderPipeline(bytes))
    }
    fn load_compiled_raster(
        &self,
        desc: &RasterPipelineDesc,
    ) -> Result<CompiledShaderPipeline, Error> {
        use crate::rhapsody_types::{
            RhapsodyCompiledShaderPipeline, RhapsodyRasterPipelineDesc, ShaderDefines,
        };
        // 真实 raster 文件键只用 source_path + defines（无入口名，pipeline_desc.rs 注释）。
        // 游戏侧 `entry` 对应编译容器各 stage 的入口字符串（0x141477105），不参与文件键。
        let mut real = RhapsodyRasterPipelineDesc::new_vertex_fragment(desc.path.clone());
        real.defines = ShaderDefines(desc.defines.clone());
        let compiled = RhapsodyCompiledShaderPipeline::load_compiled(real.compiled_file_path())
            .map_err(anyhow::Error::msg)?;
        let bytes = compiled
            .encode()
            .map_err(|error| anyhow::anyhow!("compiled shader encode failed: {error:?}"))?;
        Ok(CompiledShaderPipeline(bytes))
    }
    fn raster_shader_variants(&self, _desc: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
        // 登记缺口 §4.9：shader-pipeline 无 `shader_variants` 模块；原 O 0x1417388a0
        // `get_raster_shader_variants`（两份 desc）待取证。不补造、不假成功。
        panic!("registered gap (WIRING §4.9): shader_variants::get_raster_shader_variants (O 0x1417388a0) is not recovered")
    }
    fn create_compute_pipeline(
        &self,
        _name: &str,
        _compiled: Arc<CompiledShaderPipeline>,
    ) -> ComputePipeline {
        // 登记缺口 §2.2：真实路径 = `ShaderLibrary::register_compiled_compute(key, bytes)` →
        // `create_compute_pipeline(key, samplers) -> Arc<VulkanPipeline>`（raw/shader.rs:1220）；
        // 游戏侧 `ComputePipeline { id: u64 }` 装不下 `Arc<VulkanPipeline>`（§4.8 类型缝）。
        // ShaderLibrary 需持有实例 + sampler 表，未恢复。不补造、不假成功。
        panic!("registered gap (WIRING §2.2): create_compute_pipeline needs ShaderLibrary instance; game-side ComputePipeline cannot hold Arc<VulkanPipeline> (O 0x1416a4b20)")
    }
    fn create_raster_pipeline(
        &self,
        _compiled: Arc<CompiledShaderPipeline>,
        _key: &RasterPipelineKey,
    ) -> Result<RasterPipeline, Error> {
        // 登记缺口 §2.2：真实路径 = `ShaderLibrary::prepare_raster_program` →
        // `RasterProgram::pipeline_desc` → `RasterPipelineOpts::apply_to` → `create_raster_pipeline`
        // （raw/shader.rs:1299/1182/392/460）；游戏侧 `RasterPipeline { id, compiled }` 装不下
        // `Arc<VulkanPipeline>`（§4.8 类型缝）。不补造、不假成功。
        Err(anyhow::anyhow!(
            "registered gap (WIRING §2.2): create_raster_pipeline needs ShaderLibrary instance; game-side RasterPipeline cannot hold Arc<VulkanPipeline> (O 0x1416a3720)"
        ))
    }
}

#[cfg(test)]
impl ShaderBackend for crate::rhapsody_contract::GraphicsDevice {
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
    ) -> ComputePipeline {
        ComputePipeline { id: self.0 }
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::RefCell;

    #[derive(Default)]
    struct Mock {
        log: RefCell<Vec<String>>,
        fail_load: RefCell<Option<String>>,
        fail_pipeline: bool,
    }

    impl ShaderBackend for Mock {
        fn load_compiled_compute(
            &self,
            d: &ComputePipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error> {
            self.log.borrow_mut().push(format!("load_c {}", d.path));
            if self.fail_load.borrow().is_some() {
                return Err(anyhow::anyhow!("boom"));
            }
            Ok(CompiledShaderPipeline(d.path.clone().into_bytes()))
        }
        fn load_compiled_raster(
            &self,
            d: &RasterPipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error> {
            self.log.borrow_mut().push(format!("load_r {}", d.path));
            if self.fail_load.borrow().as_deref() == Some(d.path.as_str()) {
                return Err(anyhow::anyhow!("boom"));
            }
            Ok(CompiledShaderPipeline(d.path.clone().into_bytes()))
        }
        fn raster_shader_variants(&self, d: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
            let mk = |s: &str| RasterPipelineDesc {
                path: format!("{}.{s}", d.path),
                ..d.clone()
            };
            [mk("a"), mk("b")]
        }
        fn create_compute_pipeline(
            &self,
            name: &str,
            _c: Arc<CompiledShaderPipeline>,
        ) -> ComputePipeline {
            self.log.borrow_mut().push(format!("create_c {name}"));
            ComputePipeline { id: 1 }
        }
        fn create_raster_pipeline(
            &self,
            c: Arc<CompiledShaderPipeline>,
            k: &RasterPipelineKey,
        ) -> Result<RasterPipeline, Error> {
            self.log
                .borrow_mut()
                .push(format!("create_r {}", k.selector));
            if self.fail_pipeline {
                return Err(anyhow::anyhow!("no pipeline"));
            }
            Ok(RasterPipeline {
                id: k.selector as u64,
                compiled: c,
            })
        }
    }

    fn cdesc() -> ComputePipelineDesc {
        ComputePipelineDesc {
            path: "/x.cs.hlsl".into(),
            entry: "cs_main".into(),
            defines: vec![],
        }
    }
    fn rdesc() -> RasterPipelineDesc {
        RasterPipelineDesc {
            path: "/mesh".into(),
            entry: "vs".into(),
            defines: vec![],
        }
    }

    #[test]
    fn compute_program_loads_then_creates() {
        let m = Mock::default();
        let p = ComputeShaderProgram::new("blur", cdesc(), &m);
        assert_eq!(*m.log.borrow(), ["load_c /x.cs.hlsl", "create_c blur"]);
        assert_eq!(
            (p.name, p.pipeline.id, p.desc.entry.as_str()),
            ("blur", 1, "cs_main")
        );
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn compute_program_panics_when_artifact_missing() {
        let m = Mock::default();
        *m.fail_load.borrow_mut() = Some("x".into());
        ComputeShaderProgram::new("blur", cdesc(), &m);
    }

    #[test]
    fn variants_load_in_order_and_start_empty() {
        let m = Mock::default();
        let v = compile_shader_variants(&rdesc(), &m).unwrap();
        assert_eq!(*m.log.borrow(), ["load_r /mesh.a", "load_r /mesh.b"]);
        assert_eq!((v[0].cached(), v[1].cached()), (0, 0));
        assert_eq!(v[1].compiled.0, b"/mesh.b");
    }

    #[test]
    fn second_variant_failure_is_an_error_not_a_panic() {
        let m = Mock::default();
        *m.fail_load.borrow_mut() = Some("/mesh.b".into());
        assert!(compile_shader_variants(&rdesc(), &m).is_err());
        assert_eq!(m.log.borrow().len(), 2); // 第一份已加载（随后被丢弃）
    }

    #[test]
    fn get_or_generate_caches_per_key() {
        let m = Mock::default();
        let v = compile_shader_variants(&rdesc(), &m).unwrap();
        m.log.borrow_mut().clear();
        let k1 = RasterPipelineKey {
            handle: 9,
            selector: 1,
        };
        let a = v[0].get_or_generate(&m, k1) as *const RasterPipeline;
        let b = v[0].get_or_generate(&m, k1) as *const RasterPipeline;
        assert_eq!(a, b);
        assert_eq!(*m.log.borrow(), ["create_r 1"]); // 第二次命中，不再创建
        let c = v[0].get_or_generate(
            &m,
            RasterPipelineKey {
                handle: 9,
                selector: 2,
            },
        );
        assert_eq!(c.id, 2);
        assert_eq!(v[0].cached(), 2);
        assert_eq!(v[1].cached(), 0); // 两个变体的缓存互相独立
        assert!(Arc::ptr_eq(&c.compiled, &v[0].compiled)); // 强引用来自本变体
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn pipeline_creation_failure_panics() {
        let m = Mock {
            fail_pipeline: true,
            ..Default::default()
        };
        let v = compile_shader_variants(&rdesc(), &m).unwrap();
        v[0].get_or_generate(
            &m,
            RasterPipelineKey {
                handle: 1,
                selector: 1,
            },
        );
    }

    #[test]
    fn new_draw_keeps_desc_name_and_two_variants() {
        let m = Mock::default();
        let p = RasterShaderProgram::new_draw("mesh", rdesc(), &m);
        assert_eq!(
            (p.name, p.desc.path.as_str(), p.variants.len()),
            ("mesh", "/mesh", 2)
        );
    }

    #[test]
    fn contract_device_is_a_shader_backend() {
        let dev = crate::rhapsody_contract::GraphicsDevice(3);
        let v = compile_shader_variants(&rdesc(), &dev).unwrap();
        let p = v[0].get_or_generate(
            &dev,
            RasterPipelineKey {
                handle: 1,
                selector: 5,
            },
        );
        assert_eq!(p.id, 5);
        let c = ComputeShaderProgram::new("c", cdesc(), &dev);
        assert_eq!(c.pipeline.id, 3);
    }
}
