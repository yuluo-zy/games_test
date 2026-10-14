//! `grading_lut.rs` — 调色 LUT：`.cube` 文件 → 3D 纹理（`GradingLut::load_cube`，O 0x140b01050）。
//!
//! 只收已恢复的函数：解析逻辑在 [`super::grading_lut_cube`]，这里是“读文件 → 解析 → 建纹理”的设备接线。
//! 未恢复的部分**不放占位**：`ColorGradingState::flattened`（0x140b00b20）、`GradingLut::load_unwrapped`
//! （0x140b00bc0，PNG 解码）、`GradingLuts::load`（0x140b01df0，依赖未反编译的闭包 0x140ae5bb0）、
//! `lut_name_to_asset_name` / `asset_name`（inline 宿主，消费方 `final_post` 已自带 `grading_lut:` 前缀）。
//! 证据与偏离见 `reverse-engineering/tiny-glade/evidence/country-core-audit/R1_grading_lut.md`。

use std::path::Path;

use crate::render::load_texture::boundary::{Texture, TextureDesc, TextureDevice};

pub use super::grading_lut_cube::{CubeLut, parse_cube, parse_rgb};
use super::grading_lut_cube::{TEXTURE_FORMAT, TEXTURE_NAME};

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
    /// O：0x140b01050。读文件 → [`parse_cube`] → `TextureDesc::new_3d([size; 3], 0x7b).sampled()` →
    /// `create_texture("grading lut", desc, texels 字节)`。
    ///
    /// 失败路径（O）：读文件失败 → 该 `io::Error`；解析失败 → [`parse_cube`] 的错误；`create_texture`
    /// 失败 → `Result::unwrap` panic（原版如此，不转成 Err）。`name` 是调用方给的 LUT 名，成功时移入结果。
    pub fn load_cube<D: TextureDevice>(
        device: &D,
        path: &Path,
        name: String,
    ) -> anyhow::Result<GradingLut> {
        let text = std::fs::read_to_string(path)?;
        let cube = parse_cube(&text)?;
        let extent = [cube.size as u32; 3];
        let desc = TextureDesc::new_3d(extent, TEXTURE_FORMAT).sampled();
        let texture = device
            .create_texture(TEXTURE_NAME, &desc, &cube.texel_bytes())
            .unwrap();
        Ok(GradingLut {
            name,
            size: extent[0],
            texture,
        })
    }
}

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
        fn create_texture(
            &self,
            name: &str,
            desc: &TextureDesc,
            data: &[u8],
        ) -> Result<Texture, BackendError> {
            if self.fail {
                return Err(BackendError("boom".into()));
            }
            self.seen
                .borrow_mut()
                .push((name.to_owned(), *desc, data.to_vec()));
            Ok(Texture {
                id: 7,
                width: desc.extent[0],
                height: desc.extent[1],
                format: desc.format,
            })
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
        let dev = Recorder {
            fail: false,
            seen: RefCell::new(Vec::new()),
        };
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
        let dev = Recorder {
            fail: false,
            seen: RefCell::new(Vec::new()),
        };
        let p = std::env::temp_dir().join("grading_lut_definitely_missing.cube");
        assert!(GradingLut::load_cube(&dev, &p, "x".into()).is_err());
        assert!(dev.seen.borrow().is_empty());
    }

    #[test]
    fn load_cube_parse_error_is_err_before_any_texture() {
        let path = tmp_cube("bad", "LUT_1D_SIZE 4\n");
        let dev = Recorder {
            fail: false,
            seen: RefCell::new(Vec::new()),
        };
        let e = GradingLut::load_cube(&dev, &path, "x".into()).unwrap_err();
        let _ = std::fs::remove_file(&path);
        assert_eq!(e.to_string(), "1D LUTs are not supported");
        assert!(dev.seen.borrow().is_empty());
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn load_cube_create_texture_failure_panics_like_the_original() {
        let path = tmp_cube("panic", "LUT_3D_SIZE 1\n0 0 0\n");
        let dev = Recorder {
            fail: true,
            seen: RefCell::new(Vec::new()),
        };
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
