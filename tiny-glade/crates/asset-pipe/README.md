# asset-pipe

离线纹理编译库。将图片、DDS 和纹理数组转换为游戏使用的 `.texture` 文件，维护源文件指纹、编译缓存和纹理名称索引。

本包依赖 [asset](../asset/README.md) 提供配置继承、目录扫描、纹理描述和归档类型；图片解码、重采样及块压缩由 `image`、`ddsfile`、`intel_tex_2` 实现。本包不负责 GPU 上传、资源生命周期或游戏中的纹理绑定。

## 模块结构

所有公开入口由 `asset_pipe::texture` 导出，内部模块按处理阶段组织。

| 文件 | 职责 |
| --- | --- |
| `src/texture/image.rs` | 颜色空间转换、透明度滤波、尺寸调整、mip、边缘填充及 BC 编码 |
| `src/texture/dds.rs` | DDS 格式分派、维度和层数验证、完整 payload 导入 |
| `src/texture/array.rs` | 单图与数组分派、逐层处理和布局一致性验证 |
| `src/texture/layout.rs` | 受检的像素缓冲区大小和纹理 mip 字节布局计算 |
| `src/texture/cache.rs` | 源文件指纹、编译元数据及缓存有效性判断 |
| `src/texture/compile.rs` | 编译入口、任务调度、名称预检和索引发布 |
| `src/texture/output.rs` | 同目录临时文件及完整文件替换 |

## 输入与输出

项目入口使用以下目录约定：

```text
<root>/
├── assets-src/textures/
│   ├── assets.ron
│   ├── brick.png
│   └── array_roof/
│       ├── 0.png
│       └── 1.png
├── compiled-assets/textures/
│   ├── brick.texture
│   ├── roof.texture
│   └── list.ron
└── tmp/
    └── textures.meta.ron
```

源目录的配置继承、支持扩展名和数组命名规则见 [asset 的纹理扫描接口](../asset/README.md)。例如，以下 `assets.ron` 为当前目录及子目录指定颜色和 mip 策略：

```ron
(
    default: (
        gamma: sRGB,
        mips: Full,
        compression: Color,
    ),
    assets: {
        "mask": (gamma: Linear, compression: Grayscale),
    },
)
```

名称不含源文件扩展名，使用 `/` 分隔。编译入口拒绝空组件、`.`、`..`、反斜杠和冒号。输出沿用替换末尾扩展名的约定：`brick` 和 `brick.detail` 都会映射到 `brick.texture`，因此不能同时存在。批量编译和只读索引还会拒绝大小写及 Unicode 规范等价的输出名，避免跨平台文件覆盖。

| 输出 | 数据内容 |
| --- | --- |
| `<name>.texture` | `asset::texture::blob::TextureAssetBlob` 的版本化归档 |
| `list.ron` | 按逻辑名排序的 `Vec<RelativePathBuf>` |
| `textures.meta.ron` | 名称到编译元数据的映射，用于下次增量编译 |

## 公开接口

### 编译与加载

| 接口 | 行为与返回值 |
| --- | --- |
| `compile_texture_assets(root)` | 扫描并编译整个源目录，返回名称列表和 `TextureAssetLibraryMeta`，成功后发布索引 |
| `compile_texture_asset(name, descriptor, output_root, previous)` | 编译或复用单个纹理，返回新元数据；不更新项目索引 |
| `compile_or_load_texture_assets(root, allow_compile)` | 允许编译且源目录存在时执行批量编译，否则读取并验证现有名称索引 |
| `cache_is_current(descriptor, mtime, sources, previous, bytes)` | 判断完整输出文件字节与先前元数据是否构成有效缓存 |

`compile_or_load_texture_assets` 编译新项目时不要求已有 `list.ron`。只读路径不会检查每个索引条目的输出是否存在或可被 GPU 消费；需要加载纹理的调用方继续负责文件读取和资源创建。

### 图像与数组处理

| 接口 | 行为 |
| --- | --- |
| `process_texture_asset(descriptor)` | 按 descriptor 分派普通图片、DDS 或目录数组，不写文件 |
| `process_regular_image(descriptor)` | 从 PNG/JPEG 路径解码并应用纹理选项 |
| `process_decoded_image(image, options)` | 处理调用方已解码的 `DynamicImage` |
| `process_dds_image(path)` / `process_dds(dds)` | 直接导入支持的 DDS 数据，不应用普通图片选项 |
| `assemble_array(layers)` | 校验并拼接单层 2D blob，返回层优先排列的数组 blob |
| `mip_count(dimensions, chain)` | 计算 mip 数量，拒绝零尺寸和零级限制 |
| `pad_edges(image, dimensions)` | 重复底边和右边像素，返回 `Result<RgbaImage>`；目标不能缩小源图 |

### 编码辅助接口

| 接口 | 行为 |
| --- | --- |
| `srgb_eotf(value)` / `srgb_oetf(value)` | sRGB 与线性 RGB 的标量传递函数，不处理 alpha |
| `compress(image, mode)` | 压缩已按 4×4 块对齐的 RGBA8 图像；模式 0/1/2 分别为 BC5/BC4/BC7 |
| `block_bytes(mode)` | 返回上述压缩模式的每块字节数：16/8/16 |
| `bc7_settings(opaque)` | 根据是否完全不透明选择 BC7 编码质量预设 |

低层 `compress` 的 BC4 输入取 R 通道，BC5 输入取 R 和 A 通道。普通 `CompressionMethod` 只选择 BC7、BC4 或无压缩，不选择 BC5。

## 业务流程

### 普通图片

1. 验证源尺寸和目标尺寸。浮点输入必须有限；颜色变换或滤波中出现非有限结果时返回错误。
2. 根据**源图宽高**决定是否压缩：仅当宽高都大于 3，且 `compression != None` 时启用。放大一个小源图不会改变这项选择。
3. 选择 `resize` 或源尺寸作为目标。压缩目标向上对齐到 4 的倍数，base 图像重采样到该最终尺寸；未压缩目标不做块对齐。
4. 建立处理空间。sRGB 的 RGB 转入线性浮点，alpha 不经过 gamma 转换；linear 的 16 位和浮点源保留浮点精度，8 位源可继续使用原 RGBA8 缓冲区。
5. 需要滤波时先预乘 alpha，滤波后反预乘。含透明度的图像使用非负 Triangle 权重，全不透明图像使用 Catmull–Rom。归档仍保存直通 alpha（straight alpha）。
6. 根据最终 base 尺寸确定 mip 数；每级从 base 独立采样，压缩 mip 的块边界用右边和底边像素填充。
7. 在最终编码时转回 sRGB 或量化为 8 位，并输出原始 RGBA、BC7 或 BC4 数据。sRGB 灰度块压缩不受支持，会返回错误。

`MipChain::None` 只生成 base；`Full` 生成至最大边为 1；`Limited(n)` 最多生成 n 级，并受完整链长度约束。普通图像的输出 kind 为 2D、层数为 1。

### DDS 与数组

DDS 导入保留所有声明层及其 mip，按层优先顺序储存。支持的 DXGI 格式为 `R9G9B9E5_SharedExp`、`BC1_UNorm_sRGB`、`BC3_UNorm`、`BC3_UNorm_sRGB`、`BC5_UNorm`、`BC5_SNorm`；legacy 格式支持 DXT1、DXT5 和 ATI2。legacy DXT1/DXT5 分别沿用 BC1/BC3 sRGB 输出格式。

DDS 的 resize、gamma、mip 和 compression 选项不参与处理。导入验证非零尺寸、维度约束、有效 mip 数、层数上限及 payload 字节数，忽略 payload 末尾不属于纹理的多余字节。不支持 cubemap、3D array 和白名单之外的格式。

目录数组逐层调用相同的格式分派。每层必须是单层 2D，尺寸、mip 数和格式相同，且数据字节数符合布局。嵌套数组、3D 纹理或已有多层 DDS 不能作为数组的一层。各层通过验证后按顺序拼接，输出 kind 为 2D array。

### 批量编译与缓存

每个叶源文件记录路径、修改时间、长度和 xxh3 内容哈希。缓存命中必须同时满足：

- descriptor、源时间和所有叶依赖指纹相同；
- `pipeline_revision` 与当前处理规则一致；
- 输出文件的 xxh3 哈希匹配 `output_hash`；
- 完整文件通过 magic/version 及 rkyv 结构校验。

损坏、不可读或不匹配的缓存按未命中处理；源文件错误和新输出发布错误会返回给调用方。旧 metadata 可以通过缺省字段读取，但不能命中当前缓存，需要重新编译。文件格式版本由 `asset` 管理，当前为 8；本包处理规则版本为 2，两者分别管理数据布局和内容生成策略。

`TextureAssetMeta` 包含 `desc`、`src_mtime`、`sources`、`pipeline_revision` 和可选 `output_hash`；`TextureAssetLibraryMeta.assets` 按逻辑名保存这些记录。`cache_is_current` 接收完整文件，不能只传 8 字节文件头。

## 并发与错误语义

批量编译使用有限任务队列，工作线程数为可用逻辑 CPU 数、任务数和 8 的最小值。队列锁只保护领取任务，图像处理和文件 I/O 在锁外进行。所有已分配任务结束后才返回普通任务错误，多个错误按逻辑名排序选择，不依赖线程完成顺序。

每个纹理及索引先写同目录的独占临时文件，关闭句柄后 rename；错误路径清理临时文件，避免以半写文件覆盖已有成品。只有纹理任务全部成功后才依次发布 metadata 和名称索引。

批次不构成多文件事务：任务失败时可能已经更新部分纹理；索引发布阶段失败时，也可能已有一个索引更新。调用方收到错误后可以重试，由缓存校验恢复。调用方应串行编译同一 root，不应假定并发编译具有事务隔离。文件替换不提供断电持久化保证。

## 使用示例

编译或读取项目索引：

```rust,no_run
use anyhow::Result;
use asset_pipe::texture::compile_or_load_texture_assets;
use std::path::Path;

fn main() -> Result<()> {
    let names = compile_or_load_texture_assets(Path::new("project"), true)?;
    for name in names {
        println!("{name}");
    }
    Ok(())
}
```

在内存中处理图像并生成归档：

```rust
use anyhow::Result;
use asset::texture::{blob::TextureAssetBlob, desc::{CompressionMethod, Gamma, MipChain, TextureOpts}};
use asset_pipe::texture::process_decoded_image;
use image::{DynamicImage, Rgba, RgbaImage};

fn main() -> Result<()> {
    let source = RgbaImage::from_pixel(4, 4, Rgba([128, 64, 32, 255]));
    let options = TextureOpts {
        resize: None,
        gamma: Gamma::Srgb,
        mips: MipChain::Full,
        compression: CompressionMethod::None,
    };
    let texture = process_decoded_image(DynamicImage::ImageRgba8(source), &options)?;
    assert_eq!(texture.dimensions, [4, 4, 1]);
    assert_eq!(texture.mips, 3);
    let bytes = texture.to_file_bytes()?;
    TextureAssetBlob::validate_file(&bytes)?;
    Ok(())
}
```

## 性能与限制

已有 linear RGBA8 图像可直接借给 BC7；无 mip 的未压缩 linear 输出接管原像素 Vec。数组按层处理，避免同时保留全部中间层；正常对齐的归档校验借用输入，不反序列化像素。

完整输出读取和哈希是缓存有效性保证的一部分，热缓存仍有与输出大小相关的 I/O 和计算成本。有限工作线程数不等于进程内存预算，大图和长数组仍受实际可用内存约束。

## 开发与验证

在 workspace 根目录执行：

```sh
cargo test --offline -p asset-pipe --all-targets
cargo clippy --offline -p asset-pipe --all-targets --no-deps -- -D warnings
cargo run --offline -p asset-pipe --example benchmark_pipeline
```

测试覆盖像素与 mip 策略、透明边缘、非有限输入、DDS 和数组布局、名称预检、缓存失效、错误任务收敛、临时文件清理和整图分配回归。`benchmark_pipeline` 生成独立临时项目，用于观察冷编译和热缓存耗时，不作为受控吞吐量基准。

当前验证环境为 macOS；Windows/Linux 真机、GPU 上传和完整游戏消费不在已验证范围内。

## 相关资料

- [asset 模块文档](../asset/README.md)：源目录、配置继承与 `.texture` 归档定义。
- [源码来源审计](../../provenance/foundation-audit.json)：导入来源及历史快照。
- [重构报告](../../reports/foundation-refactor-2026-10-17/README.md)、[完整复审报告](../../reports/foundation-rereview-2026-10-17/README.md)：变更历史、验证日志和性能观测。
