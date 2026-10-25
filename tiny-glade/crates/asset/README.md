# asset

`asset` 定义 prefab 和纹理的配置模型、目录扫描规则、继承合并及纹理归档格式，为资产编译器和运行时消费者提供共同的数据合同。

本包负责把文件配置解析成资产描述，不解码图片、不生成 mip、不压缩纹理，也不创建渲染实例或上传 GPU 资源。图片处理与编译入口见 [`asset-pipe`](../asset-pipe/README.md)。

## 模块结构

| 模块 | 职责 |
| --- | --- |
| `inheritable` | 字段状态、浅层替换、对象与映射继承，以及可选对象的默认补齐。 |
| `prefab` | 拥有型 `PrefabName` 和零分配借用视图 `PrefabNameRef`。 |
| `prefab::config` | 部件、渲染、RT、音频配置及合并后的必填字段检查。 |
| `prefab::scan` | 递归加载 prefab、应用目录默认配置、记录文件依赖。 |
| `texture::config` | 尚待继承的纹理选项及目录级 `TextureGroupConfig`。 |
| `texture::desc` | 已解析的纹理选项、单图/数组描述、叶源遍历和逻辑名称检查。 |
| `texture::scan` | 读取纹理目录、合并选项、构造普通纹理和有序数组描述。 |
| `texture::blob` | 带 magic/version 的 rkyv 归档序列化、结构校验和加载。 |

## 数据模型

### Prefab

`PrefabConfig.parts` 按部件名保存 `PrefabPartConfig`。每个部件包含删除标记 `delete`、渲染配置 `render`、可选 RT 配置 `rt` 和可选音频配置 `audio`。

| 配置 | 字段与要求 |
| --- | --- |
| `render` | `mesh`、`shader` 为必填字符串；`mesh_processor` 默认为 `None`，启用时使用 `Code(name: "...")`。 |
| `rt` | 默认禁用；启用后 `mesh` 必填，目前枚举支持 `Aabb`；`mesh_scale` 和三个季节的 albedo 默认为 `[1.0, 1.0, 1.0]`；`semantic` 默认为 `None`，还支持 `Canopy`。 |
| `audio` | 默认禁用；启用后 `trigger.shape`、`trigger.kind` 必填，shape 支持 `Aabb`；`trigger.shape_scale` 默认为 `[1.0, 1.0, 1.0]`。 |
| `delete` | 默认为 `false`；为 `true` 时表达部件删除，不要求提供其他部件数据。 |

`PrefabName` 保存 `Arc<str>`，克隆共享字符串存储；`PrefabNameRef` 可直接借用 `&str`。两者的比较和哈希使用原始 UTF-8 内容，不做大小写转换或路径规范化。

扫描返回 `PrefabScan`：`prefabs` 保存已继承、已验证的配置；`dependencies.by_file` 从默认文件和 prefab 文件映射到依赖它们的 prefab 名称。

### 纹理

`TextureOpts` 是已解析的处理选项；`InheritableTextureOpts` 是对应的配置输入，可携带尚未继承的字段。

| 选项 | RON 值 | 根默认值 |
| --- | --- | --- |
| `resize` | `None` 或 `Some((宽, 高))` | `None` |
| `gamma` | `Linear`、`sRGB` | `Linear` |
| `mips` | `None`、`Full`、`Limited(数量)` | `None` |
| `compression` | `Color`、`Grayscale`、`None` | `Color` |

`TextureAssetDesc` 包含源 `path`、已解析的 `opts` 和 `layers`。`layers` 为空时表示单个源文件；非空时表示数组，层的顺序就是数组顺序。`source_files()` 返回全部叶源路径，`visit_source_files()` 以相同顺序遍历，避免为递归中间层分配 Vec。

`TextureAssetBlob` 包含字节数据 `data`、三维尺寸 `dimensions`、`mips`、`layers` 和数值标签 `format`、`kind`。文件前 8 字节是小端 magic `0x48502183` 和版本 `8`，随后为固定字段布局的 rkyv 归档。

## 目录与 RON 配置

### Prefab 目录

```text
prefabs/
  default.ron
  house.ron
  garden/
    default.ron
    bench.ron
```

扫描器将 `house.ron` 命名为 `house`，将 `garden/bench.ron` 命名为 `garden/bench`。每层 `default.ron` 都是该层及子目录的默认配置，不作为独立 prefab 输出。`.asset-filter` 文件被忽略；其他非小写 `.ron` 文件返回错误。文件名必须可表示为 UTF-8。

根 `default.ron` 示例：

```ron
(
    parts: {
        "body": (
            render: (mesh: "default_mesh", shader: "default_shader"),
        ),
    },
)
```

`house.ron` 示例：

```ron
(
    parts: {
        "body": (
            render: (
                mesh: "house_mesh",
                shader: "wood_shader",
                mesh_processor: Code(name: "house_processor"),
            ),
            rt: Some((mesh: Aabb, semantic: Canopy)),
            audio: Some((trigger: (shape: Aabb, kind: "Wood"))),
        ),
    },
)
```

Prefab 三分量向量使用 RON 序列，例如 `mesh_scale: [1.0, 2.0, 1.0]`；纹理的 resize 使用二元组。

### 纹理目录

```text
textures/
  assets.ron
  wood.png
  stone.jpg
  detail/
    moss.dds
  array_leaves/
    assets.ron
    0.png
    1.png
```

普通文件去扩展名得到逻辑名称，例如 `wood`、`detail/moss`；`array_leaves` 生成名为 `leaves` 的数组。数组层必须连续命名为 `0`、`1`、`2`……，空数组目录忽略。普通纹理与数组共享名称空间，精确重名返回错误。

只接受小写 `.png`、`.jpg`、`.dds`；忽略 `.txt` 和 `.asset-filter`，读取 `assets.ron` 作为配置，其余文件返回错误。最终逻辑名称不得含空路径组件、`.`、`..`、反斜杠或冒号。

根 `assets.ron` 示例：

```ron
(
    default: (gamma: sRGB, mips: Full, compression: Color),
    assets: {
        "wood": (resize: Some((512, 512))),
        "stone": (compression: None),
    },
)
```

`assets` 的键只匹配同目录普通源文件的 stem，不匹配子目录名称或数组名称，也不会向子目录传播。`default` 则向子目录传播。

数组各层的共同选项写入 `array_leaves/assets.ron`：

```ron
(
    default: (resize: Some((128, 128)), gamma: sRGB, mips: Full),
)
```

此处的 `default` 应用到 `0.png`、`1.png` 的叶描述；也可在该文件的 `assets` 中以 `"0"`、`"1"` 为键配置单层。数组容器描述的 `opts` 来自父目录默认值；`asset-pipe` 处理数组时使用各叶描述的选项，再验证各层是否可组装。

## 继承与业务流程

`Inheritable<T>` 区分三个状态：`Inherit` 表示未指定、需要继承；`MissingDefault` 表示必填字段没有值或默认；`Value(T)` 表示显式值。`Default` 创建的是 `Inherit`，不是已解析的业务默认值。

继承以子配置优先：只将 `Inherit` 替换为父值，`MissingDefault` 保持未解决状态；显式原子值保持不变。对象按字段合并，HashMap 保留子键并补入父键；`inherit_shallow` 将整个字段视为原子值。

可选字段接受裸值、`Some(...)`、`None`；`()` 也表示显式 `None`。省略字段会继承，显式 `None` 则禁用继承来的对象。启用的 RT/audio 对象在合并后补齐自身默认值，必填字段仍需由配置提供。

Prefab 扫描先累计祖先和当前目录的默认配置，再合并每个叶文件，调用 `PrefabConfig::validate()`，最后写入名称和文件依赖。目录默认配置可以不完整；最终未删除部件的必填字段必须已解决。

纹理扫描先将当前目录的 `default` 与父目录默认值合并，再把普通文件的专用选项叠加到当前默认值，解析成 `TextureOpts`。遇到数组目录时递归收集层，以数字顺序组装描述；扫描阶段只检查命名与配置，不读取图片像素。

## 公开 API

| 用途 | 入口 |
| --- | --- |
| 字段合并与读取 | `Inherit::inherit_from`、可选对象补齐钩子 `finish_inheritance`、`Inheritable::inherit_shallow`、`try_as_concrete`、`try_into_concrete`。 |
| Prefab 名称与查找 | `PrefabName::new`、`PrefabNameRef::from_str_ref`、两种名称的 `as_str`、`PrefabName::as_name_ref`；HashMap 可用借用名称查找。 |
| Prefab 加载 | `prefab::scan::load_prefab_config` 仅解析单文件；`PrefabConfig::validate` 检查合并结果；`prefab::scan::scan_asset_tree` 执行完整扫描。 |
| Prefab 默认配置 | `PrefabPartConfig::resolved_defaults` 及 render/RT/audio/trigger 类型的 `concrete_default`；补齐可选值，保留必填字段的 `MissingDefault`。 |
| Serde 适配 | `deserialize_inheritable`、`deserialize_inheritable_option`，供配置字段自定义反序列化使用。 |
| Prefab 依赖 | `PrefabScan`、`PrefabDependencies::add_file_to_prefab_dependency`；重复注册按插入顺序保留。 |
| 纹理配置 | `InheritableTextureOpts::concrete_default`、`from_concrete`、`try_into_concrete`。 |
| 纹理扫描与依赖 | `texture::scan::scan_asset_tree`、`TextureAssetDesc::source_files`、`visit_source_files`、`validate_logical_name`。 |
| 纹理归档 | `TextureAssetBlob::to_file_bytes`、`load`、`validate_file`。 |

两个扫描模块也公开 `scan_asset_tree_walker`，供需要显式前缀、父配置和外部结果容器的调用方使用。一般使用 `scan_asset_tree`，由入口负责初始化根默认值。

## 错误语义

扫描、配置验证及归档操作通过 `anyhow::Result` 返回错误。配置错误附带文件、部件或字段上下文；纹理重名错误列出两个源路径。`try_as_concrete` 和 `try_into_concrete` 返回 `UnresolvedField`，区分待继承和缺少必填默认。

`as_concrete`、`into_concrete` 及 `InheritableTextureOpts::into_concrete` 要求字段已经解决，否则 panic。处理文件输入时优先使用 fallible API。

Prefab walker 成功或失败都会恢复传入依赖栈的原长度。两个 walker 失败时，其外部结果容器可能已有部分条目；调用方应丢弃失败扫描的结果。`load_prefab_config` 不自动继承或验证。

归档读取检查 magic、版本和 rkyv 结构；支持任意对齐的输入切片，对齐合适时直接借用归档，不合适时复制到对齐缓冲。`load` 返回拥有自身数据的 blob；`validate_file` 不反序列化像素。两者都不验证 GPU 标签组合、纹理尺寸与像素字节数是否一致。

## 使用示例

从具有上述目录结构的根目录读取已解析资产：

```rust
use asset::{
    prefab::{PrefabNameRef, scan as prefab_scan},
    texture::scan as texture_scan,
};
use std::path::Path;

fn inspect_assets(root: &Path) -> anyhow::Result<()> {
    let prefabs = prefab_scan::scan_asset_tree(&root.join("prefabs"))?;
    if let Some(house) = prefabs.prefabs.get(PrefabNameRef::from_str_ref("house")) {
        house.validate()?;
    }

    let textures = texture_scan::scan_asset_tree(&root.join("textures"))?;
    for (name, descriptor) in &textures {
        descriptor.visit_source_files(&mut |source| {
            println!("{} <- {}", name, source.display());
        });
    }
    Ok(())
}
```

## 开发与验证

在 `games_test/tiny-glade` workspace 根执行：

```sh
cargo fmt -p asset -- --check
cargo clippy --offline -p asset --all-targets --no-deps -- -D warnings
cargo test --offline -p asset --all-targets
```

测试覆盖字段继承、显式禁用、必填字段诊断、名称冲突、依赖栈恢复、数组编号约束、叶源遍历和归档对齐/损坏输入。发行资源回归还读取原 prefab 与 388 份纹理归档并检查字节一致 roundtrip；这些测试需要 [`tests/_shipped_root.rs`](tests/_shipped_root.rs) 指定的资源目录。`--offline` 要求依赖已缓存。

## 支持与限制

本包的数据验证止于配置字段、目录规则及归档结构；不检查 mesh/shader/音频标识是否存在，不验证 GPU 格式语义，也不保证各数组层的尺寸和格式一致。这些检查由具体编译器或资源消费者承担。描述类型可表达嵌套层结构，`asset-pipe` 的数组组装只接受单层 2D 输入。

扫描结果使用 HashMap，目录遍历及映射迭代没有排序保证；数组层和叶源遍历顺序明确。逻辑名称检查也不等于目标文件系统的大小写或平台保留名检查。归档 wire 布局不等同于 Rust native ABI，读取成功不能替代完整游戏加载验证。

## 来源与审查记录

- [源码审查记录](../../reports/foundation/2026-10-17-foundation-rereview/README.md)。
- [导入来源与哈希记录](../../provenance/foundation-audit.json)。
- [原恢复源码](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/asset)与[业务合同来源](../../../reverse-engineering/tiny-glade/docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)。
