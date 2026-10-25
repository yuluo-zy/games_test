# render/load_texture.rs

> 文件作用：**编译纹理镜像加载器**——`load_compiled_texture_image(path, …) -> Result<Texture, _>`：拼编译镜像路径 →
> mmap 读文件 → 头校验（对齐长度、magic、mip 数）→ `TextureDesc` + `create_texture`。返回 0x40 B
> `rhapsody::raw::texture::Texture`（复用）。证据：`reconstruction/country-core-render/wp4/contracts/
> view_frame_light_atmos_texture.md`（O，2323 B 全量反编译）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `load_compiled_texture_image` | 0x140917860 | 10 | 猜测：加载纹理；证实：编译镜像读取+校验+创建 | **O**（2323 B）：1. puffin scope `"load_compiled_texture_image"`（串 0x142aad64 已证实存在；decomp 引用 UNK_142aacc4e 区拼路径/日志）；2. `Path::to_path_buf` + join + `with_extension`（编译镜像路径）；3. debug 日志；`std::fs::OpenOptions::open`（io 错误 → panic）、`memmap2` mmap + file_len；4. 头校验：对齐长度、magic `0x48502183`（decomp 0x140917b66 直读证实）、字段 +4 == 8、尾部 mips u16 < 0xff；5. `TextureDesc::new().sampled().mip_levels(..).array_elements(..)`；`ustr::Ustr::from(path)`；`GraphicsDevice::create_texture<&[u8]>`（reuse）unwrap；6. 返回 0x40 B `Texture` | render-top/decomp/decomp-0x140917860.txt（原 wp4） |

## 派生胶水（1 项：closure）

- 创建/校验闭包（无独立符号；I）。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `load_compiled_texture_image`（0x140917860）—— 编译镜像加载链
处理链（O，2323B）：
1. puffin scope `"load_compiled_texture_image"`；
2. `Path::to_path_buf + join + with_extension`（编译镜像路径拼接）；
3. debug 日志；`std::fs::OpenOptions::open`（io 错误 → panic）、`memmap2` mmap + file_len；
4. **头校验**：对齐长度、magic `0x48502183`、字段 +4 == 8、尾部 mips u16 < 0xff；
5. `TextureDesc::new().sampled().mip_levels(..).array_elements(..)`；`ustr::Ustr::from(path)`（temporal 键同用 Ustr）；
6. `GraphicsDevice::create_texture<&[u8]>`（reuse）unwrap → 返回 0x40B `Texture`。
- 消费链：`texture_library`/`AssetTextureLibrary` 入库（导入的 RG 资源经 `Texture as ImportExportToRenderGraph`）；编译镜像头字段语义（len-0x14/len-0xc/len-8）字节级 O 但语义 U。

## 纠错/差异

- 编译镜像头字段语义（len-0x14 / len-0xc / len-8）O 字节级但语义 U（wp4 注）；magic 0x48502183 已证实。
- reconstruction `load_texture.rs`（227 B）已按合同实现，无差异。
