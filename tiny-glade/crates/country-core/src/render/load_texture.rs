//! 编译纹理文件加载。边界检查和整数转换失败通过 Result 返回，供启动与热重载共用。
//! 二进制格式来源保留在项目证据，运行期不依赖 Rust 对象布局或指针对齐。

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
        /// 释放同一宿主创建的纹理身份；GPU 使用由托管资源延迟回收。
        fn release_texture(&self, _texture: &Texture) {}
        fn create_texture(
            &self,
            name: &str,
            desc: &TextureDesc,
            data: &[u8],
        ) -> Result<Texture, BackendError>;
    }
}

/// 全 crate 唯一的设备 `rhapsody_contract::GraphicsDevice` 实现纹理创建。
/// D 组接入（WIRING §2.1/§3.3）：真实路径 = `GraphicsDevice::create_texture::<&[u8]>(&real_desc, name,
/// data) -> Result<ManagedTexture>`（device_api.rs:151）。游戏侧 0x38B `Texture { id, width, height,
/// format }` 装不下 `ManagedTexture`（**登记缺口 §4.8**，同 ssbo 的 40B Buffer 缝）。此实现记录日志并
/// 返回未接错误，不补造、不假成功。`crate::rhapsody_types::convert_texture_desc` 是真实 desc 换算。
#[cfg(not(test))]
impl boundary::TextureDevice for crate::rhapsody_contract::GraphicsDevice {
    fn create_texture(
        &self,
        name: &str,
        desc: &TextureDesc,
        _data: &[u8],
    ) -> Result<Texture, BackendError> {
        log::warn!(
            target: "country_core::render::load_texture",
            "create_texture({name}): registered gap (WIRING §4.8), game-side Texture cannot hold ManagedTexture; real desc = {:?}",
            crate::rhapsody_types::convert_texture_desc(desc)
        );
        Err(BackendError::unsupported(
            "Texture backend is not connected (registered gap WIRING §4.8)",
        ))
    }
}

#[cfg(test)]
impl boundary::TextureDevice for crate::rhapsody_contract::GraphicsDevice {
    fn create_texture(
        &self,
        _name: &str,
        desc: &TextureDesc,
        _data: &[u8],
    ) -> Result<Texture, BackendError> {
        // 常开 `TextureLibraryDevice: TextureDevice` 超 trait 要求 rhapsody_contract 类型实现
        // 本 trait；CPU 测试不构造该类型（统一用 `render::test_device::TestDevice`），id 取 0。
        Ok(Texture {
            id: 0,
            width: desc.extent[0],
            height: desc.extent[1],
            format: 0,
        })
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
    try_parse_compiled_image(bytes, shown).expect("编译纹理解析失败")
}

/// 热重载使用 Result，截断文件、非法偏移和尺寸不会中断游戏循环。
pub fn try_parse_compiled_image<'a>(
    bytes: &'a [u8],
    shown: &Path,
) -> anyhow::Result<CompiledImage<'a>> {
    anyhow::ensure!(
        bytes.len() >= 8 + FOOTER,
        "纹理文件不完整：{}",
        shown.display()
    );
    let magic = u32::from_le_bytes(bytes[0..4].try_into()?);
    let version = u32::from_le_bytes(bytes[4..8].try_into()?);
    anyhow::ensure!(
        magic == MAGIC,
        "Image {} has an incorrect magic identifier",
        shown.display()
    );
    anyhow::ensure!(
        version == VERSION,
        "Image {} has incorrect version",
        shown.display()
    );
    let footer_start = bytes.len() - FOOTER;
    let footer = &bytes[footer_start..];
    let read_u32 =
        |offset: usize| u32::from_le_bytes(footer[offset..offset + 4].try_into().unwrap());
    let read_u16 =
        |offset: usize| u16::from_le_bytes(footer[offset..offset + 2].try_into().unwrap());
    let relative_offset = read_u32(0) as i32;
    let data_length = read_u32(4) as usize;
    let extent = [read_u32(8), read_u32(12), read_u32(16)];
    let mip_levels = u8::try_from(read_u16(20))?;
    anyhow::ensure!(
        extent.iter().all(|dimension| *dimension > 0) && mip_levels > 0,
        "纹理尺寸或 mip 数为零"
    );
    let start = footer_start
        .checked_add_signed(relative_offset as isize)
        .ok_or_else(|| anyhow::anyhow!("纹理数据偏移越界"))?;
    let end = start
        .checked_add(data_length)
        .ok_or_else(|| anyhow::anyhow!("纹理数据长度溢出"))?;
    anyhow::ensure!(
        start >= 8 && end <= footer_start,
        "纹理数据与文件头/尾部重叠"
    );
    Ok(CompiledImage {
        desc: TextureDesc::new(extent, footer[25], footer[24])
            .sampled()
            .mip_levels(mip_levels)
            .array_elements(read_u16(22)),
        data: &bytes[start..end],
    })
}

/// `Path::new(dir).join(name).with_extension("texture")`（O）。
pub fn compiled_texture_path(name: &str) -> PathBuf {
    Path::new(COMPILED_TEXTURE_DIR)
        .join(name)
        .with_extension(COMPILED_TEXTURE_EXT)
}

/// 文件字节拥有者。保留调用接口，解析器不再要求额外的对齐拷贝。
pub struct AlignedFile {
    bytes: Vec<u8>,
}
impl AlignedFile {
    pub fn read(path: &Path) -> std::io::Result<Self> {
        std::fs::read(path).map(|bytes| Self { bytes })
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// O 0x140917860。
pub fn load_compiled_texture_image(device: &dyn TextureDevice, name: &str) -> Texture {
    let path = compiled_texture_path(name);
    log::trace!(target: "country_core::render::load_texture", "Loading compiled image {} from {}", name, path.display());
    let file = AlignedFile::read(&path)
        .unwrap_or_else(|e| panic!("Error opening {}: {}", path.display(), e));
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
        unsafe { core::slice::from_raw_parts_mut(w.as_mut_ptr() as *mut u8, v.len()) }
            .copy_from_slice(v);
        (w, v.len())
    }
    fn view(w: &(Vec<u32>, usize)) -> &[u8] {
        unsafe { core::slice::from_raw_parts(w.0.as_ptr() as *const u8, w.1) }
    }

    #[test]
    fn contract_device_creates_texture_from_desc() {
        let dev = crate::render::test_device::TestDevice(7);
        let t = load_via(&dev, TextureDesc::new([64, 32, 1], 0, 0));
        assert_eq!((t.id, t.width, t.height), (7, 64, 32));
    }
    fn load_via(d: &dyn boundary::TextureDevice, desc: TextureDesc) -> Texture {
        d.create_texture("x", &desc, &[]).unwrap()
    }

    #[test]
    fn path_has_dir_and_texture_extension() {
        assert_eq!(
            compiled_texture_path("grass.png"),
            Path::new("compiled-assets/textures/grass.texture")
        );
        assert_eq!(
            compiled_texture_path("a/b"),
            Path::new("compiled-assets/textures/a/b.texture")
        );
    }

    #[test]
    fn parses_footer_fields_and_data_slice() {
        let file = build(&[1, 2, 3, 4, 5, 6, 7, 8], [64, 32, 1], 7, 2, 11, 22);
        let w = aligned(&file);
        let img = parse_compiled_image(view(&w), Path::new("x"));
        assert_eq!(img.data, &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            img.desc,
            TextureDesc {
                extent: [64, 32, 1],
                kind: 22,
                format: 11,
                sampled: true,
                mip_levels: 7,
                array_elements: 2,
                ..Default::default()
            }
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
