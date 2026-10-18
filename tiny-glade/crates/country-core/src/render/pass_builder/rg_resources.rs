//! rg_resources.rs — RG（渲染图）资源创建接口（B 阶段 i_api 新增）。
//!
//! 来源：d_main（`mei_pass` 的 "mei temp"、`final_post` 的 "msaa color out"、水/冰的 RG 纹理）、c1/c2/c3
//! （`create_raw_resource`、`get_or_create_temporal[_pair]`、`BufferDesc::new_gpu_only`、`TextureDesc::new_2d/storage/sampled`）
//! 各自在自己的文件里造了 `RgFactory` / `PassFrame` / `FramePasses` 接缝；这里给出**唯一**一份，由 [`PassHost`]
//! （`draw_stream.rs`）经 [`PassHost::rg_resources`](crate::render::pass_builder::draw_stream::PassHost::rg_resources) 暴露。
//!
//! 机器码调用点（O）：
//!  * `rhapsody::rg::RenderGraph::create_raw_resource`（0x140be94d0）：参数 `(&mut RenderGraph, name: &str, desc: impl Into<resource::Desc>)`，
//!    `TextureDesc` 与 `BufferDesc` 两种来源（`rg::resource::From<TextureDesc>` / `From<BufferDesc>`）；返回句柄，资源号在 `+0`（纹理，36 B：
//!    `{id: u32, desc: TextureDesc}`）或 `+0x10`（缓冲，见 `compute_pass_builder::RgBuffer`）。
//!  * `rhapsody::rg::temporal::get_or_create_temporal<PassResources, &str>`（0x1404f6e50）与 `get_or_create_temporal_pair`（0x1404f7450）：
//!    `Result<_, E>`，调用点一律 `Result::unwrap`。
//!  * 返回结构的布局取自 `render_world.rs` 的核对（单张历史：`RgTexture` + `+0x24` 标志字节，2 = `Err` niche；pair：当前帧 + 上一帧
//!    各 36 B + 末尾 1 B 标志），字段含义 U/I 同该文件。
//!
//! 实现者是 rhapsody 的 `RenderGraph`；本文件只定义 trait 与结果类型，不发明行为。

use crate::render::pass_builder::compute_pass_builder::RgBuffer;
use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::render::ssbo::rhapsody_boundary::BufferDesc;
use crate::rhapsody_types::TextureDesc;

/// Preserves the concrete backend source so callers can classify OOM/device loss.
pub type RgError = anyhow::Error;

/// Recover driver categories across the game-side error envelope.
pub(crate) fn into_backend_error(error: RgError) -> rhapsody::Error {
    match error.downcast::<rhapsody::Error>() {
        Ok(error) => error,
        Err(error) => rhapsody::Error::InvalidUsage {
            what: "named graph resource".into(),
            source: Some(error.into()),
        },
    }
}

/// `get_or_create_temporal`（纹理）的成功结果：纹理 + 历史有效字节。
/// B `0x1404f702e` 缓存命中写 1，`0x1404f72d6` 新建写 0；2 是 Err niche。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TemporalTexture {
    pub texture: RgTexture,
    pub flag: u8,
}

/// `get_or_create_temporal_pair` 的成功结果：当前帧 / 上一帧各一张 + 历史有效标志。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TemporalTexturePair {
    pub current: RgTexture,
    pub previous: RgTexture,
    /// Historical field name retained for compatibility. True means valid
    /// history, as consumed by SSGI/ReSTIR/TAA, not a newly allocated texture.
    pub fresh: bool,
}

/// `get_or_create_temporal`（缓冲）的成功结果（c2：`get_or_create_temporal(name, BufferDesc::new_gpu_only(size, usage))`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TemporalBuffer {
    pub buffer: RgBuffer,
    pub flag: u8,
}

/// RG 资源创建面（`RenderGraph` + `temporal`）。
pub trait RgResources {
    /// `RenderGraph::create_raw_resource(name, TextureDesc)`（0x140be94d0）：返回 `{id, desc}`（desc 原样回传）。
    fn create_raw_texture(&mut self, name: &str, desc: &TextureDesc) -> RgTexture;
    /// `RenderGraph::create_raw_resource(name, BufferDesc)`：返回缓冲资源。
    fn create_raw_buffer(&mut self, name: &str, desc: &BufferDesc) -> RgBuffer;
    /// `get_or_create_temporal(name, TextureDesc)`（0x1404f6e50）。
    fn get_or_create_temporal_texture(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexture, RgError>;
    /// `get_or_create_temporal_pair(name, TextureDesc)`（0x1404f7450）。
    fn get_or_create_temporal_texture_pair(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexturePair, RgError>;
    /// `get_or_create_temporal(name, BufferDesc)`。
    fn get_or_create_temporal_buffer(
        &mut self,
        name: &str,
        desc: &BufferDesc,
    ) -> Result<TemporalBuffer, RgError>;
}

/// 测试/离线用的记录型实现（资源号自增；temporal 按名字复用）。真实实现由 rhapsody 的 `RenderGraph` 提供。
#[derive(Debug, Default)]
#[cfg(test)]
pub struct RecordingRg {
    /// `(名字, 创建的是不是纹理)`，按调用顺序。
    pub created: Vec<(String, RgKind)>,
    /// temporal 名字 → 资源号（pair 占连续两个）。
    pub temporal: std::collections::HashMap<String, u32>,
    pub next_id: u32,
}

/// [`RecordingRg`] 记录的资源种类。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(test)]
pub enum RgKind {
    Texture(TextureDesc),
    Buffer(BufferDesc),
}

#[cfg(test)]
impl RecordingRg {
    fn fresh(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

#[cfg(test)]
impl RgResources for RecordingRg {
    fn create_raw_texture(&mut self, name: &str, desc: &TextureDesc) -> RgTexture {
        self.created.push((name.to_owned(), RgKind::Texture(*desc)));
        RgTexture {
            resource: self.fresh(),
            desc: *desc,
        }
    }
    fn create_raw_buffer(&mut self, name: &str, desc: &BufferDesc) -> RgBuffer {
        self.created.push((name.to_owned(), RgKind::Buffer(*desc)));
        RgBuffer {
            resource: self.fresh(),
        }
    }
    fn get_or_create_temporal_texture(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexture, RgError> {
        let known = self.temporal.get(name).copied();
        let id = match known {
            Some(id) => id,
            None => {
                let id = self.fresh();
                self.temporal.insert(name.to_owned(), id);
                id
            }
        };
        Ok(TemporalTexture {
            texture: RgTexture {
                resource: id,
                desc: *desc,
            },
            flag: known.is_some() as u8,
        })
    }
    fn get_or_create_temporal_texture_pair(
        &mut self,
        name: &str,
        desc: &TextureDesc,
    ) -> Result<TemporalTexturePair, RgError> {
        let known = self.temporal.get(name).copied();
        let id = match known {
            Some(id) => id,
            None => {
                let id = self.fresh();
                let _prev = self.fresh();
                self.temporal.insert(name.to_owned(), id);
                id
            }
        };
        Ok(TemporalTexturePair {
            current: RgTexture {
                resource: id,
                desc: *desc,
            },
            previous: RgTexture {
                resource: id + 1,
                desc: *desc,
            },
            fresh: known.is_some(),
        })
    }
    fn get_or_create_temporal_buffer(
        &mut self,
        name: &str,
        _desc: &BufferDesc,
    ) -> Result<TemporalBuffer, RgError> {
        let known = self.temporal.get(name).copied();
        let id = match known {
            Some(id) => id,
            None => {
                let id = self.fresh();
                self.temporal.insert(name.to_owned(), id);
                id
            }
        };
        Ok(TemporalBuffer {
            buffer: RgBuffer { resource: id },
            flag: known.is_some() as u8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_resources_get_distinct_ids_and_keep_their_desc() {
        let mut rg = RecordingRg::default();
        let d = TextureDesc::new_2d([640, 360], 0x4c).sampled().storage();
        let a = rg.create_raw_texture("ssgi", &d);
        let b = rg.create_raw_buffer("subtotal_sum_buf", &BufferDesc::new_gpu_only(64, 0x20));
        assert_eq!((a.resource, a.desc), (0, d));
        assert_eq!(b.resource, 1);
        assert_eq!(rg.created.len(), 2);
        assert!(d.storage && d.sampled && d.format == 0x4c && d.extent_2d() == [640, 360]);
    }

    #[test]
    fn temporal_resources_are_reused_by_name_and_flag_valid_history() {
        let mut rg = RecordingRg::default();
        let d = TextureDesc::new_2d([8, 8], 9);
        let first = rg.get_or_create_temporal_texture("h", &d).unwrap();
        let again = rg.get_or_create_temporal_texture("h", &d).unwrap();
        assert_eq!((first.flag, again.flag), (0, 1));
        assert_eq!(first.texture.resource, again.texture.resource);
        let p = rg
            .get_or_create_temporal_texture_pair("ssgi_history", &d)
            .unwrap();
        assert!(!p.fresh);
        assert_eq!(p.previous.resource, p.current.resource + 1);
        assert!(
            rg.get_or_create_temporal_texture_pair("ssgi_history", &d)
                .unwrap()
                .fresh
        );
    }

    #[test]
    fn named_backend_errors_keep_categories_through_context() {
        for error in [
            rhapsody::Error::DeviceLost,
            rhapsody::Error::OutOfHostMemory,
            rhapsody::Error::OutOfDeviceMemory,
            rhapsody::Error::Timeout,
            rhapsody::Error::unsupported("missing capability"),
        ] {
            let kind = error.kind();
            let message = error.to_string();
            let recovered =
                into_backend_error(anyhow::Error::new(error).context("history allocation"));
            assert_eq!(recovered.kind(), kind);
            assert_eq!(recovered.to_string(), message);
        }
        let error = into_backend_error(anyhow::Error::msg("wrong resource type"));
        assert!(matches!(
            error,
            rhapsody::Error::InvalidUsage {
                source: Some(_),
                ..
            }
        ));
    }
}
