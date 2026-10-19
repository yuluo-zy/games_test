//! `grading_lut.rs` — 调色 LUT：`.cube` / 展平 PNG → 3D 纹理（`load_cube` O 0x140b01050、
//! `load_unwrapped` O 0x140b00bc0、`GradingLuts::load` O 0x140b01df0，均 fresh 反编译 2026-10-17）。
//!
//! 解析逻辑在 [`super::grading_lut_cube`]，这里是“读文件 → 解析/解码 → 建纹理”的设备接线。
//! `ColorGradingState::flattened`（0x140b00b20）已实现在 [`super::image_formation`]。
//!
//! # `load_unwrapped`（fresh disasm 0x140b00bc0 订正）
//! 原版签名是 `(device, name: String) -> GradingLut`（**不是** `(device, path, name) -> Result`）：
//!  - 函数体内 `format_inner`（0x140d79f90）用常量片段 `"assets/luts/"`（0x142ae6bf6, len 12）+
//!    一个参数 + `".png"`（0x142ae6c02, len 4）造出路径；随后 `Reader::open_impl`（0x140ba3a10）
//!    直接吃这个格式化 String（`RCX=out, RDX=ptr, R8=len`）。
//!  - `decode<BufReader<File>>` 0x14091bbb0 与 `open` 的失败路径都走 `unwrap_failed`（panic），
//!    不是 `?`；尾部 `MOV RAX,RSI; RET` 只写 0x60 B 的 `GradingLut`，无 Result 判别子。
//!  - extent 三轴固定 0x10、格式 0x25（`MOV R8D,0x25` = VK_FORMAT_R8G8B8A8_UNORM），名
//!    `"grading lut"`（0x142ae6c70, len 0xb）；`create_texture` 失败同 `unwrap` panic。
//!
//! # `GradingLuts::load`（fresh 0x140b01df0 + 闭包 0x140ae5bb0）
//! `read_dir(...).unwrap()` → 每条 `DirEntry`（闭包内 `entry.unwrap()`）→ `path/extension/file_stem`
//! → `"cube"`（len 4）走 [`GradingLut::load_cube`]（Err → `log::error!("Failed parsing LUT {}: {}")`
//! 并跳过）；`"png"`（len 3）走 [`GradingLut::load_unwrapped`]（原版对其 Result 无错误分支，即
//! unwrap）；其它扩展名 → `log::warn!("Unsupported LUT: {}")` 并跳过；非 UTF-8 扩展名/文件名静默跳过。
//! 返回值是 **两个并行 Vec**：元素 0x28（`String` 名 + 3×u32 尺寸）与元素 0x38（纹理记录，
//! 见 [`TextureRecord`]），闭包每次成功向两者各压一条。Rust 侧以 [`GradingLutMeta`] +
//! [`TextureRecord`] 两个 Vec 表出。
//!
//! `asset_name`（inline 宿主 lut_gallery@78）依赖 `Option::map_or_else` + `lut_name_to_asset_name`；
//! helper [`asset_name`] 按该形状落地（缺省空串 I）。`lut_name_to_asset_name` 前缀 `grading_lut:`（O）。
//! 证据与偏离见 `reverse-engineering/tiny-glade/evidence/country-core-audit/R1_grading_lut.md`
//! 与本轮 `reports/interaction/ui_mode/ui-restorer-remainder-REMAINDER.md`。

use std::path::Path;

use crate::render::load_texture::boundary::{Texture, TextureDesc, TextureDevice};

pub use super::grading_lut_cube::{CubeLut, parse_cube, parse_rgb};
use super::grading_lut_cube::{TEXTURE_FORMAT, TEXTURE_NAME};

/// 展平 LUT 的固定目录（O：`load_unwrapped` 的 `format!` 常量片段 0x142ae6bf6, len 12 = `"assets/luts/"`）。
pub const UNWRAPPED_DIR: &str = "assets/luts/";
/// 展平 LUT 的扩展名（O：0x142ae6c02, len 4 = `".png"`）。
pub const UNWRAPPED_EXT: &str = ".png";

/// 展平 LUT 的固定边长（O：`load_unwrapped` 0x140b00bc0 的 `new_3d` extent 三轴同 `0x10`，`MOV R14,0x1000000010`
/// + `[RBP+0x58]=0x10`）。展平 PNG 为 16³ LUT 的 RGBA8 像素。
pub const UNWRAPPED_SIZE: u32 = 16;
/// 展平 LUT 纹理格式（O：`load_unwrapped` 的 `new_3d` 第三参 `MOV R8D,0x25` = `VK_FORMAT_R8G8B8A8_UNORM`）。
pub const UNWRAPPED_TEXTURE_FORMAT: u32 = 0x25;

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

    /// O：0x140b00bc0。`assets/luts/{name}.png` → PNG 解码 → RGBA8 → 固定 16³ 3D 纹理。
    ///
    /// fresh disasm 顺序（O）：`format!("assets/luts/{}.png", name)`（0x140d79f90）→
    /// `image::io::Reader::open_impl`（0x140ba3a10）→ `Reader::decode<BufReader<File>>`（0x14091bbb0）→
    /// `DynamicImage::to_rgba8`（0x140bad8d0）→ 释放原图（0x140b00010）→
    /// `TextureDesc::new_3d([16; 3], 0x25)`（0x140bd5310）→ `.sampled()`（0x140bd5370）→
    /// `create_texture("grading lut", …, data)`（0x1416a6700）。
    ///
    /// 失败路径（O）：open / decode / create_texture 全部是 `unwrap` panic（函数尾 `MOV RAX,RSI; RET`
    /// 只返回 0x60 B 的 [`GradingLut`]，无错误判别子）。`name` 由调用方移入结果。
    pub fn load_unwrapped<D: TextureDevice>(device: &D, name: String) -> GradingLut {
        let path = format!("{UNWRAPPED_DIR}{name}{UNWRAPPED_EXT}");
        Self::load_unwrapped_from_path(device, Path::new(&path), name)
    }

    /// [`GradingLut::load_unwrapped`] 的路径参数化内核（同一解码链；用于测试与显式路径接线）。
    pub fn load_unwrapped_from_path<D: TextureDevice>(
        device: &D,
        path: &Path,
        name: String,
    ) -> GradingLut {
        let reader = image::io::Reader::open(path).unwrap();
        let rgba = reader.decode().unwrap().to_rgba8();
        let desc = TextureDesc::new_3d([UNWRAPPED_SIZE; 3], UNWRAPPED_TEXTURE_FORMAT).sampled();
        let texture = device
            .create_texture(TEXTURE_NAME, &desc, rgba.as_raw())
            .unwrap();
        GradingLut {
            name,
            size: UNWRAPPED_SIZE,
            texture,
        }
    }
}

/// `GradingLuts` 第一个并行 Vec 的元素（O：stride 0x28 = `GradingLut` 前缀 0x28 B：
/// `name: String`（0x18）+ 3×`u32` 尺寸（0x18/0x1c/0x20）+ 4 B 对齐）。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GradingLutMeta {
    /// LUT 名（O：`GradingLut +0x00` String；`load_cube` 里来自文件名 stem）。
    pub name: String,
    /// 三维边长（O：`GradingLut +0x18/+0x1c/+0x20` 三份同值）。
    pub extent: [u32; 3],
}

/// `GradingLut` 元数据前缀记录大小（O：`GradingLuts::load` 0x140b01df0 第一个并行 Vec 的
/// 元素 stride = 0x28；= name String 0x18 + 3×u32 尺寸 0xc + 4 B 对齐）。
pub const META_RECORD_SIZE: usize = 0x28;
/// `GradingLut` 纹理记录大小（O：第二个并行 Vec 的 stride = **0x38**；
/// `load_cube` 0x140b01050 `MOVUPS [RDX+0x28..0x60]` 从 `create_texture` 的 sret 输出
/// 原样搬 0x38 B）。
pub const TEXTURE_RECORD_SIZE: usize = 0x38;

/// 原生纹理记录（O：`GradingLut +0x28..0x60`，0x38 B，是 `create_texture` 的返回值）。
///
/// fresh 只锚定 **大小 0x38**（`GradingLuts::load` 的 stride 与 `load_cube` 的 0x38 字节搬移）；
/// 记录内部字段属于 `rhapsody::resource::Texture`，本 crate 只有 0x18 语义模型
/// （`id/width/height/format`）。`_gap` 保留未锚定尾部，不编造字段。
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TextureRecord {
    /// 语义纹理句柄（本 crate `Texture` 模型，0x18 B）。
    pub texture: Texture,
    /// 0x18..0x38：未锚定纹理记录字节（fresh 未给出字段语义）。
    pub _gap: [u8; 0x20],
}

impl TextureRecord {
    /// 包装 `create_texture` 的返回语义模型（未锚定尾部清零）。
    pub fn new(texture: Texture) -> Self {
        Self {
            texture,
            _gap: [0; 0x20],
        }
    }
}

/// 调色 LUT 集合（O：`GradingLuts`，0x30 B = 两个并行 Vec：stride 0x28 的元数据 + stride 0x38 的纹理）。
///
/// fresh 0x140b01df0 的闭包每成功解析一条 LUT 就同时向两个 Vec 各压一条（`load` 里两段
/// `grow_one` + 独立长度计数器）；原生纹理记录 0x38 B（[`TextureRecord`]）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct GradingLuts {
    /// stride 0x28：`{name String, extent [u32;3]}`。
    pub metas: Vec<GradingLutMeta>,
    /// stride 0x38：纹理记录（[`TextureRecord`]）。
    pub textures: Vec<TextureRecord>,
}

impl GradingLuts {
    /// O：0x140b01df0。扫描目录，按扩展名加载 LUT，成功的条目成对推入两个 Vec。
    ///
    /// 分支（fresh，逐条）：
    /// - `read_dir(directory)` 失败 → `unwrap` panic（0x140b01df0 里 `local_398 == 2` 走 `unwrap_failed`）；
    /// - 每条 `entry` 失败 → 闭包内 `unwrap_failed`；
    /// - `extension`/`file_stem` 缺失或非 UTF-8 → 静默跳过；
    /// - `"cube"` → [`GradingLut::load_cube`]；`Err` → `log::error!(target:
    ///   "country_core::resources::grading_lut", "Failed parsing LUT {}: {}", path, err)` 并跳过；
    /// - `"png"` → [`GradingLut::load_unwrapped`]（原版成功路径直接搬 0x60 B，无错误分支；
    ///   这里 `.unwrap()` 复现其 panic 语义）；
    /// - 其它 → `log::warn!(target: …, "Unsupported LUT: {}", path)` 并跳过。
    pub fn load<D: TextureDevice>(device: &D, directory: &Path) -> GradingLuts {
        let mut luts = GradingLuts::default();
        for entry in std::fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let (Some(ext), Some(stem)) = (
                path.extension().and_then(|e| e.to_str()),
                path.file_stem().and_then(|s| s.to_str()),
            ) else {
                continue;
            };
            match ext {
                "cube" => match GradingLut::load_cube(device, &path, stem.to_owned()) {
                    Ok(lut) => luts.push(lut),
                    Err(err) => log::error!(
                        target: "country_core::resources::grading_lut",
                        "Failed parsing LUT {}: {}",
                        path.display(),
                        err
                    ),
                },
                "png" => {
                    let lut = GradingLut::load_unwrapped_from_path(device, &path, stem.to_owned());
                    luts.push(lut);
                }
                _ => log::warn!(
                    target: "country_core::resources::grading_lut",
                    "Unsupported LUT: {}",
                    path.display()
                ),
            }
        }
        luts
    }

    /// 把一条 [`GradingLut`] 拆成两个并行 Vec 的尾部条目（O：闭包每次成功各压一条）。
    pub fn push(&mut self, lut: GradingLut) {
        let GradingLut {
            name,
            size,
            texture,
        } = lut;
        self.metas.push(GradingLutMeta {
            name,
            extent: [size; 3],
        });
        self.textures.push(TextureRecord::new(texture));
    }

    /// 已加载 LUT 数（两个 Vec 同步增长；O）。
    pub fn len(&self) -> usize {
        self.metas.len()
    }

    /// 无 LUT。
    pub fn is_empty(&self) -> bool {
        self.metas.is_empty()
    }
}

/// `lut_name_to_asset_name`（O inline 宿主：`final_post@82` / `lut_gallery@82`，PDB inline 依赖
/// `format!` + `map_or_else`）：LUT 名 → 资产名，前缀 `grading_lut:`（O：`final_post` 消费方
/// 已自带该前缀，见 LOGIC-UI-REMAINDER）。签名 I（inline 无独立符号，落为自由 helper）。
pub fn lut_name_to_asset_name(lut_name: &str) -> String {
    format!("grading_lut:{lut_name}")
}

/// `asset_name`（O inline 宿主 `lut_gallery@78` / `final_post`）：PDB 内联依赖
/// `Option::map_or_else` + [`lut_name_to_asset_name`]。`None` 时返回空串（缺省值 I：内联体里
/// 缺省分支未被单独锚定，登记）。
pub fn asset_name(lut_name: Option<&str>) -> String {
    lut_name.map_or_else(String::new, lut_name_to_asset_name)
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
                return Err(BackendError::OutOfDeviceMemory);
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

    #[test]
    fn load_unwrapped_decodes_png_into_fixed_16_cube_texture() {
        let dev = Recorder {
            fail: false,
            seen: RefCell::new(Vec::new()),
        };
        let path =
            std::env::temp_dir().join(format!("grading_lut_unwrapped_{}.png", std::process::id()));
        let img = image::RgbaImage::from_raw(16, 16, vec![0u8; 16 * 16 * 4]).unwrap();
        img.write_to(
            &mut std::fs::File::create(&path).unwrap(),
            image::ImageFormat::Png,
        )
        .unwrap();

        let lut = GradingLut::load_unwrapped_from_path(&dev, &path, "warm".into());
        let _ = std::fs::remove_file(&path);

        assert_eq!((lut.name.as_str(), lut.size), ("warm", UNWRAPPED_SIZE));
        let seen = dev.seen.borrow();
        let (name, desc, data) = &seen[0];
        assert_eq!(name, "grading lut");
        assert_eq!(desc.extent, [UNWRAPPED_SIZE; 3]);
        assert_eq!(desc.format, UNWRAPPED_TEXTURE_FORMAT);
        assert!(desc.sampled);
        assert_eq!(data.len(), 16 * 16 * 4);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn load_unwrapped_missing_file_panics_like_the_original() {
        let dev = Recorder {
            fail: false,
            seen: RefCell::new(Vec::new()),
        };
        let p = std::env::temp_dir().join("grading_lut_unwrapped_definitely_missing.png");
        let _ = GradingLut::load_unwrapped_from_path(&dev, &p, "x".into());
    }

    #[test]
    fn lut_name_to_asset_name_prepends_grading_lut_prefix() {
        assert_eq!(lut_name_to_asset_name("warm"), "grading_lut:warm");
        assert_eq!(lut_name_to_asset_name(""), "grading_lut:");
    }

    #[test]
    fn asset_name_maps_optional_name() {
        assert_eq!(asset_name(Some("warm")), "grading_lut:warm");
        assert_eq!(asset_name(None), "");
    }

    #[test]
    fn texture_record_matches_native_stride() {
        // O：`GradingLuts::load` 0x140b01df0 的两个并行 Vec stride = 0x28 / 0x38。
        assert_eq!(core::mem::size_of::<TextureRecord>(), TEXTURE_RECORD_SIZE);
        assert_eq!(META_RECORD_SIZE, 0x28);
        assert_eq!(TEXTURE_RECORD_SIZE, 0x38);
    }

    #[test]
    fn grading_luts_load_scans_directory_by_extension() {
        let dir = std::env::temp_dir().join(format!("grading_luts_dir_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("tone.cube"), "LUT_3D_SIZE 2\n0 0 0\n1 1 1\n").unwrap();
        std::fs::write(dir.join("ignored.txt"), "not a lut").unwrap();
        std::fs::write(dir.join("noext"), "x").unwrap();
        let dev = Recorder {
            fail: false,
            seen: RefCell::new(Vec::new()),
        };
        let luts = GradingLuts::load(&dev, &dir);
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(luts.len(), 1);
        assert_eq!(luts.textures.len(), 1);
        assert_eq!(luts.metas[0].name, "tone");
        assert_eq!(luts.metas[0].extent, [2, 2, 2]);
        // 只对 .cube 建了纹理；.txt/.noext 跳过。
        assert_eq!(dev.seen.borrow().len(), 1);
    }

    struct Remove(std::path::PathBuf);
    impl Drop for Remove {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
}
