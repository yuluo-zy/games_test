//! `load_compiled_texture_image` — 加载预编译的纹理文件（layer A1）。
//!
//! 证据（O，逐条核对 0x140917860，3178 B，见 `evidence/country-core-audit/L0_NOTES.md`）：
//!  * 路径：`Path::new("compiled-assets/textures").join(name).with_extension("texture")`。
//!    旧版把它写成 `with_extension("bin")` 且不加目录，是错的。
//!  * 日志：`trace!("Loading compiled image {} from {}", name, path.display())`（级别立即数 5，target
//!    `country_core::render::load_texture`）。
//!  * 打开失败 → `panic!("Error opening {}: {}", path.display(), io_error)`；`file_len` 或 mmap 失败 →
//!    `.expect("image mmap")`。文件用 `memmap2` 只读映射。
//!  * 校验顺序：长度 ≥ 8（否则 `&data[..8]` 越界 panic）→ **`ptr % 4 == 0`**（assert_eq，不是 8）→
//!    `u32 @0 == 0x48502183`，消息 `"Image {path} has an incorrect magic identifier"` → `u32 @4 == 8`，消息
//!    `"Image {path} has incorrect version"`。旧版的 "mip 计数 < 0xff"、"长度 %8" 等检查并不存在。
//!  * 尾部 0x1c 字节的描述（相对文件末尾）：
//!      −0x1c i32 数据相对偏移（相对**该字段自身地址**）  −0x18 u32 数据长度
//!      −0x14 u32×3 extent(w,h,d)   −0x08 u16 mip 数   −0x06 u16 数组层数
//!      −0x04 u8（`TextureDesc::new` 第 4 参）   −0x03 u8（第 3 参）   末 2 字节未读取
//!    `TextureDesc::new(extent, b(−3), a(−4)).sampled().mip_levels(u8::try_from(mips).unwrap()).array_elements(layers)`。
//!  * `create_texture(device, Ustr::from(name), desc, data)`；Err → `Result::unwrap` panic。返回 0x38 B 的 `Texture`。
//!  * puffin 作用域 "load_compiled_texture_image"（库行为，省略）。
//!
//! 偏离：原版对 `len < 0x1c` 或偏移越界直接读裸指针（UB），这里改为带边界检查的 panic。

use std::path::{Path, PathBuf};

/// 预编译纹理的目录与扩展名（O：字符串常量 `compiled-assets/textures` / `texture`）。
pub const COMPILED_TEXTURE_DIR: &str = "compiled-assets/textures";
pub const COMPILED_TEXTURE_EXT: &str = "texture";
pub const MAGIC: u32 = 0x4850_2183;
pub const VERSION: u32 = 8;
const FOOTER: usize = 0x1c;

/// 外部库占位：类型来自 `crate::rhapsody_types`（全 crate 唯一一份），设备 trait 在此。
pub mod boundary {
    pub use crate::rhapsody_types::{BackendError, Texture, TextureDesc};

    pub trait TextureDevice {
        fn create_texture(&self, name: &str, desc: &TextureDesc, data: &[u8]) -> Result<Texture, BackendError>;
    }
}

/// 全 crate 唯一的设备 `rhapsody_contract::GraphicsDevice` 实现纹理创建（占位：不触 GPU，
/// 用文件里的 extent 回填 `Texture`）。
impl boundary::TextureDevice for crate::rhapsody_contract::GraphicsDevice {
    fn create_texture(&self, _name: &str, desc: &TextureDesc, _data: &[u8]) -> Result<Texture, BackendError> {
        Ok(Texture { id: self.0, width: desc.extent[0], height: desc.extent[1], format: 0 })
    }
}

use boundary::*;

/// 解析结果：描述 + 像素数据切片。
#[derive(Debug, PartialEq, Eq)]
pub struct CompiledImage<'a> {
    pub desc: TextureDesc,
    pub data: &'a [u8],
}

/// 校验并解析映射后的文件内容（O：`ptr % 4`、魔数、版本、尾部字段）。`shown` 仅用于 panic 消息中的路径。
pub fn parse_compiled_image<'a>(bytes: &'a [u8], shown: &Path) -> CompiledImage<'a> {
    let len = bytes.len();
    let _ = &bytes[..8];
    assert_eq!((bytes.as_ptr() as usize) % 4, 0);
    let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
    assert_eq!(magic, MAGIC, "Image {} has an incorrect magic identifier", shown.display());
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    assert_eq!(version, VERSION, "Image {} has incorrect version", shown.display());

    let foot = &bytes[len - FOOTER..];
    let u32_at = |o: usize| u32::from_le_bytes(foot[o..o + 4].try_into().unwrap());
    let u16_at = |o: usize| u16::from_le_bytes(foot[o..o + 2].try_into().unwrap());
    let rel = u32_at(0) as i32 as isize;
    let data_len = u32_at(4) as usize;
    let extent = [u32_at(8), u32_at(12), u32_at(16)];
    let mips = u16_at(20);
    let layers = u16_at(22);
    let (arg4, arg3) = (foot[24], foot[25]);

    let desc = TextureDesc::new(extent, arg3, arg4)
        .sampled()
        .mip_levels(u8::try_from(mips).unwrap())
        .array_elements(layers);
    // 数据偏移相对“尾部块起点”（= len − 0x1c）。
    let start = usize::try_from((len - FOOTER) as isize + rel).expect("image data offset out of range");
    let data = &bytes[start..start + data_len];
    CompiledImage { desc, data }
}

/// `Path::new(dir).join(name).with_extension("texture")`（O）。
pub fn compiled_texture_path(name: &str) -> PathBuf {
    Path::new(COMPILED_TEXTURE_DIR).join(name).with_extension(COMPILED_TEXTURE_EXT)
}

/// 4 字节对齐的文件缓冲（`memmap2` 映射天然页对齐；这里用 u32 存储保证同样的对齐）。
pub struct AlignedFile {
    words: Vec<u32>,
    len: usize,
}
impl AlignedFile {
    pub fn read(path: &Path) -> std::io::Result<Self> {
        let raw = std::fs::read(path)?;
        let mut words = vec![0u32; raw.len().div_ceil(4)];
        let bytes = unsafe { core::slice::from_raw_parts_mut(words.as_mut_ptr() as *mut u8, words.len() * 4) };
        bytes[..raw.len()].copy_from_slice(&raw);
        Ok(Self { words, len: raw.len() })
    }
    pub fn bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.words.as_ptr() as *const u8, self.len) }
    }
}

/// O 0x140917860。
pub fn load_compiled_texture_image(device: &dyn TextureDevice, name: &str) -> Texture {
    let path = compiled_texture_path(name);
    log::trace!(target: "country_core::render::load_texture", "Loading compiled image {} from {}", name, path.display());
    let file = AlignedFile::read(&path).unwrap_or_else(|e| panic!("Error opening {}: {}", path.display(), e));
    let img = parse_compiled_image(file.bytes(), &path);
    device
        .create_texture(name, &img.desc, img.data)
        .expect("called `Result::unwrap()` on an `Err` value")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 造一个合法文件：头 8 B + 数据 + 0x1c 尾部。
    fn build(data: &[u8], extent: [u32; 3], mips: u16, layers: u16, a4: u8, a3: u8) -> Vec<u8> {
        let mut f = Vec::new();
        f.extend_from_slice(&MAGIC.to_le_bytes());
        f.extend_from_slice(&VERSION.to_le_bytes());
        let data_off = f.len();
        f.extend_from_slice(data);
        while f.len() % 4 != 0 {
            f.push(0);
        }
        let foot_at = f.len();
        f.extend_from_slice(&((data_off as i32) - (foot_at as i32)).to_le_bytes());
        f.extend_from_slice(&(data.len() as u32).to_le_bytes());
        for e in extent {
            f.extend_from_slice(&e.to_le_bytes());
        }
        f.extend_from_slice(&mips.to_le_bytes());
        f.extend_from_slice(&layers.to_le_bytes());
        f.extend_from_slice(&[a4, a3, 0, 0]);
        assert_eq!(f.len() - foot_at, FOOTER);
        f
    }

    fn aligned(v: &[u8]) -> (Vec<u32>, usize) {
        let mut w = vec![0u32; v.len().div_ceil(4)];
        unsafe { core::slice::from_raw_parts_mut(w.as_mut_ptr() as *mut u8, v.len()) }.copy_from_slice(v);
        (w, v.len())
    }
    fn view(w: &(Vec<u32>, usize)) -> &[u8] {
        unsafe { core::slice::from_raw_parts(w.0.as_ptr() as *const u8, w.1) }
    }

    #[test]
    fn contract_device_creates_texture_from_desc() {
        let dev = crate::rhapsody_contract::GraphicsDevice(7);
        let t = load_via(&dev, TextureDesc::new([64, 32, 1], 0, 0));
        assert_eq!((t.id, t.width, t.height), (7, 64, 32));
    }
    fn load_via(d: &dyn boundary::TextureDevice, desc: TextureDesc) -> Texture {
        d.create_texture("x", &desc, &[]).unwrap()
    }

    #[test]
    fn path_has_dir_and_texture_extension() {
        assert_eq!(compiled_texture_path("grass.png"), Path::new("compiled-assets/textures/grass.texture"));
        assert_eq!(compiled_texture_path("a/b"), Path::new("compiled-assets/textures/a/b.texture"));
    }

    #[test]
    fn parses_footer_fields_and_data_slice() {
        let file = build(&[1, 2, 3, 4, 5, 6, 7, 8], [64, 32, 1], 7, 2, 11, 22);
        let w = aligned(&file);
        let img = parse_compiled_image(view(&w), Path::new("x"));
        assert_eq!(img.data, &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            img.desc,
            TextureDesc { extent: [64, 32, 1], arg3: 22, arg4: 11, sampled: true, mip_levels: 7, array_elements: 2, ..Default::default() }
        );
    }

    #[test]
    #[should_panic(expected = "Image q has an incorrect magic identifier")]
    fn bad_magic_panics_with_path() {
        let mut file = build(&[0; 4], [1, 1, 1], 1, 1, 0, 0);
        file[0] ^= 0xff;
        let w = aligned(&file);
        parse_compiled_image(view(&w), Path::new("q"));
    }

    #[test]
    #[should_panic(expected = "Image q has incorrect version")]
    fn bad_version_panics_with_path() {
        let mut file = build(&[0; 4], [1, 1, 1], 1, 1, 0, 0);
        file[4] = 9;
        let w = aligned(&file);
        parse_compiled_image(view(&w), Path::new("q"));
    }

    #[test]
    #[should_panic]
    fn mip_count_over_255_panics() {
        let file = build(&[0; 4], [1, 1, 1], 256, 1, 0, 0);
        let w = aligned(&file);
        parse_compiled_image(view(&w), Path::new("q"));
    }

    #[test]
    #[should_panic]
    fn shorter_than_header_panics() {
        let w = aligned(&[1, 2, 3, 4]);
        parse_compiled_image(view(&w), Path::new("q"));
    }

    #[test]
    #[should_panic]
    fn misaligned_pointer_panics() {
        let file = build(&[0; 4], [1, 1, 1], 1, 1, 0, 0);
        let w = aligned(&file);
        // 从偏移 1 开始：指针 % 4 == 1。
        parse_compiled_image(&view(&w)[1..], Path::new("q"));
    }
}
