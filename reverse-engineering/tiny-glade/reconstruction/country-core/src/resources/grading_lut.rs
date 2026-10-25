//! `grading_lut.rs` — color grading LUT resources (3D LUT textures).
//!
//! Business logic (contracts/lut.md §2):
//!   - `ColorGradingState`: current grading selection (name + 4 values);
//!     `flattened` 0x140b00b20 feeds `final_post` and `prepare_frame_constants`.
//!   - `GradingLut`: single 3D LUT. `load_unwrapped` 0x140b00bc0 (unwrapped PNG →
//!     RGBA8 → 3D texture); `load_cube` 0x140b01050 (full `.cube` parser: TITLE /
//!     LUT_3D_SIZE / DOMAIN_MIN|MAX / data lines → RGB9E5 pack → 3D texture);
//!     `parse_rgb` 0x140b01cc0 (3 f32 per line).
//!   - `GradingLuts::load` 0x140b01df0: readdir over the LUT directory, per-file
//!     closure load.
//!   - `LutGalleryState`: gallery state resource (inserted by lut_gallery::build,
//!     default layout O).

use std::path::Path;

use crate::render::load_texture::boundary::{Texture, TextureDesc, TextureDevice};
use crate::typed_gap::Gap;

pub use super::grading_lut_cube::{parse_cube, parse_rgb, CubeLut};
use super::grading_lut_cube::{TEXTURE_FORMAT, TEXTURE_NAME};

/// 当前颜色分级选择（O 名称）。布局（O 偏移，语义 I）：
/// +0x00 String 名、+0x18..+0x24 4×f32、+0x28 byte 已命名标志。
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ColorGradingState {
    /// +0x00 LUT 名称（String {ptr,len,cap}，默认 `"default"`）。
    pub name: Gap,
    /// +0x18..+0x24 分级值（默认 {1.0,0.0,1.0,1.0}；语义 U：exposure/contrast/…）。
    pub values: [f32; 4],
    /// +0x28 已命名标志。
    pub named: u8,
}

impl ColorGradingState {
    /// O：0x140b00b20。扁平化输出（name + 4 值 + byte）；被 `final_post@50` 与
    /// `prepare_frame_constants@50` inline 消费。
    pub fn flattened(&self) -> FlattenedGrading {
        unimplemented!()
    }
}

/// `ColorGradingState::flattened` 输出（O：{String, 4×f32, byte}；布局 U）。
pub struct FlattenedGrading(pub Gap);

/// 单条 3D LUT（O 名称）。布局（O，`load_cube` 写序）：+0x00 String 名（调用方传入）、
/// +0x18/+0x1c/+0x20 边长 ×3（u32，同值）、+0x28..+0x58 3D 纹理句柄区。
#[derive(Clone, Debug)]
pub struct GradingLut {
    /// +0x00 LUT 名（`load_cube` 的 `param_5`，原样移入）。
    pub name: String,
    /// +0x18 边长（`size as u32`；+0x1c/+0x20 为同值，这里只存一份）。
    pub size: u32,
    /// +0x28..+0x58 3D 纹理（`create_texture` 的返回值）。
    pub texture: Texture,
}

impl GradingLut {
    /// O：0x140b00bc0。展平 PNG → RGBA8 → 3D 纹理（`new_3d + sampled +
    /// create_texture<&[u8]>`）。返回 Result。
    pub fn load_unwrapped(_path: &Gap) -> Result<GradingLut, Gap> {
        unimplemented!()
    }

    /// O：0x140b01050。读文件 → [`parse_cube`]（`.cube` 解析，逻辑见 `grading_lut_cube.rs`）→
    /// `TextureDesc::new_3d([size; 3], 0x7b).sampled()` → `create_texture("grading lut", desc, texels 字节)`。
    ///
    /// 失败路径（O）：读文件失败 → 该 `io::Error`；解析失败 → [`parse_cube`] 的错误；`create_texture`
    /// 失败 → `Result::unwrap` panic（原版如此，不转成 Err）。`name` 是调用方给的 LUT 名，成功时移入结果。
    pub fn load_cube<D: TextureDevice>(device: &D, path: &Path, name: String) -> anyhow::Result<GradingLut> {
        let text = std::fs::read_to_string(path)?;
        let cube = parse_cube(&text)?;
        let extent = [cube.size as u32; 3];
        let desc = TextureDesc::new_3d(extent, TEXTURE_FORMAT).sampled();
        let texture = device.create_texture(TEXTURE_NAME, &desc, &cube.texel_bytes()).unwrap();
        Ok(GradingLut { name, size: extent[0], texture })
    }

    /// O（inline 宿主）：`lut_name_to_asset_name`（final_post@82/lut_gallery@82）。
    pub fn lut_name_to_asset_name(&self, _lut_name: &str) -> Gap {
        unimplemented!()
    }

    /// O（inline 宿主）：`asset_name`（lut_gallery@78）。
    pub fn asset_name(&self) -> &Gap {
        unimplemented!()
    }
}

/// 分级 LUT 集合（O 名称）。布局（O 偏移）：+0x00 Vec<stride 0x28>、
/// +0x18 Vec<stride 0x38>。
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct GradingLuts {
    /// +0x00 目录条目 Vec A（stride 0x28）。
    pub _list_a: Gap,
    /// +0x18 目录条目 Vec B（stride 0x38）。
    pub _list_b: Gap,
}

impl GradingLuts {
    /// O：0x140b01df0。`readdir` LUT 目录 + 每 `DirEntry` 调 `impl$3::load::closure_env$0`
    ///（load_cube/load_unwrapped，I）→ 两 Vec。
    pub fn load(_dir: &Gap) -> Result<GradingLuts, Gap> {
        unimplemented!()
    }
}

/// LUT 画廊状态：真实类型在 `systems::render_loop::lut_gallery`（O 16 B：状态 Arc + 预览槽）。
/// 旧的 0x40 B 全零占位已删除，这里只 re-export（`lib.rs` 的 `consume::core` 仍经此路径取用）。
pub use crate::systems::render_loop::lut_gallery::LutGalleryState;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::render::load_texture::boundary::BackendError;

    /// 记录 `create_texture` 的入参；可配置成失败。
    struct Recorder {
        fail: bool,
        seen: RefCell<Vec<(String, TextureDesc, Vec<u8>)>>,
    }
    impl TextureDevice for Recorder {
        fn create_texture(&self, name: &str, desc: &TextureDesc, data: &[u8]) -> Result<Texture, BackendError> {
            if self.fail {
                return Err(BackendError("boom".into()));
            }
            self.seen.borrow_mut().push((name.to_owned(), *desc, data.to_vec()));
            Ok(Texture { id: 7, width: desc.extent[0], height: desc.extent[1], format: desc.format })
        }
    }

    fn tmp_cube(tag: &str, body: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!("grading_lut_{}_{tag}.cube", std::process::id()));
        std::fs::write(&p, body).unwrap();
        p
    }

    #[test]
    fn load_cube_builds_sampled_3d_e5b9g9r9_texture() {
        let path = tmp_cube("ok", "LUT_3D_SIZE 2\n0 0 0\n1 1 1\n");
        let dev = Recorder { fail: false, seen: RefCell::new(Vec::new()) };
        let lut = GradingLut::load_cube(&dev, &path, "warm".to_owned()).unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!((lut.name.as_str(), lut.size), ("warm", 2));
        assert_eq!(lut.texture.id, 7);
        let seen = dev.seen.borrow();
        let (name, desc, data) = &seen[0];
        assert_eq!(name, "grading lut");
        assert_eq!(desc.extent, [2, 2, 2]);
        assert_eq!(desc.format, 0x7b);
        assert!(desc.sampled);
        assert_eq!(data.len(), 8 * 4);
    }

    #[test]
    fn load_cube_missing_file_is_err_before_any_texture() {
        let dev = Recorder { fail: false, seen: RefCell::new(Vec::new()) };
        let p = std::env::temp_dir().join("grading_lut_definitely_missing.cube");
        assert!(GradingLut::load_cube(&dev, &p, "x".into()).is_err());
        assert!(dev.seen.borrow().is_empty());
    }

    #[test]
    fn load_cube_parse_error_is_err_before_any_texture() {
        let path = tmp_cube("bad", "LUT_1D_SIZE 4\n");
        let dev = Recorder { fail: false, seen: RefCell::new(Vec::new()) };
        let e = GradingLut::load_cube(&dev, &path, "x".into()).unwrap_err();
        let _ = std::fs::remove_file(&path);
        assert_eq!(e.to_string(), "1D LUTs are not supported");
        assert!(dev.seen.borrow().is_empty());
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn load_cube_create_texture_failure_panics_like_the_original() {
        let path = tmp_cube("panic", "LUT_3D_SIZE 1\n0 0 0\n");
        let dev = Recorder { fail: true, seen: RefCell::new(Vec::new()) };
        let _guard = Remove(path.clone());
        let _ = GradingLut::load_cube(&dev, &path, "x".into());
    }

    struct Remove(std::path::PathBuf);
    impl Drop for Remove {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
}
