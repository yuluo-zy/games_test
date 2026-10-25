# asset-pipe → asset：B 业务恢复与复验

本轮先闭合小包 asset-pipe 的 B 纹理流水线，再修复其 asset 配置/扫描/归档前置条件。采用自己的 Rust 包装与控制流，外部解码、滤波、压缩、Serde/RON/RKYV 算法仍直接调用原作锁定依赖。

## 范围与源码结构

三版联合库存不能当作 B 的源码分母。asset-pipe 的 B 只有 `lib.rs` / `texture.rs`；C 新增 `mip_flood.rs` / `store.rs` / `texture_processing.rs`，本轮没有拿 B 的地址推断它们。asset 的 B 是 9 个路径：lib、inheritable、prefab、prefab/config、prefab/scan、texture/blob、texture/config、texture/desc、texture/scan。另加的 `asset/src/texture.rs` 只是合成模块入口；C 的 foliage、mesh、legacy_mod、texture 原文件仍是独立缺口。

源码位于 `reconstruction/shared-foundation/crates/{asset-pipe,asset}/`。逐文件函数、原作地址、证据和验证范围见 `delivery.json`。B 持久 worker 为 `foundation-assets-small-first`，项目是 Windows B seed 的独立副本，没有合并历史 Mac overlay；原型仍未冒充真实 Rust ABI。

## 规则、实现、差异修复

| 原作实际规则 | 当前实现 | 对照、差异及修复 | 复验 |
|---|---|---|---|
| texture 默认 compression 判别值为 0，即 Color | `CompressionMethod::default()` 为 Color | 旧草稿错误选择 None；依据 B root scanner 的零初始化改正 | 缺失 compression 的扫描继承与编译 fixture |
| BC7 先检查所有 alpha 是否为 255，再选择 basic preset | 复用原 Intel fork 的 opaque_basic / alpha_basic | 旧草稿使用 slow；B 常量是 refine=2、skip=12/8/0或4、skip_mode2=true，已修复 | 压缩策略 fixture；原常量与依赖 preset 字段核对 |
| sRGB 在 float 工作空间做 EOTF，只处理 RGB；输出做 OETF，alpha 不变 | 原 image 0.25.1 的 RGBA32F/8 转换，私有 transfer 函数 | EOTF 的 NaN 比较按原无序分支使用 `x > threshold`；OETF 用原有 `<=` 分支 | 原 64B OETF 指令 4364 案例，零差异 |
| 压缩资格依据解码后源图两维都至少 4；可压缩图尺寸向 4 对齐 | 原 codec/filter + 私有尺寸策略 | 不能先根据 resize 后尺寸决定压缩；小图 resize 变大仍不压缩 | 3×6→9×10、5×6→8×8 fixture |
| 所有 resize 使用 CatmullRom；mip 由 base 直接采样，边缘重复至块大小 | `next_level` 调原 image，结果明确物化成 RGBA8 | sRGB 的 mip helper 也先变成 RGBA8，这是原代码行为，未擅自改为更理想的全程 float | mip 字节数/边缘填充/transfer fixture；原调用点及 helper 指令对照 |
| DDS 保留输入数据/尺寸/mip，忽略普通图处理 options；只复制 get_data(0) | 原 ddsfile 0.5.1 + 私有格式白名单 | 原数组层数可大于 1，但复制数据仍只有 layer zero；保留此规则。Legacy DXT1/DXT5 固定 sRGB | 6 DXGI / 2 legacy 格式、layer-zero 与拒绝不支持格式 fixture |
| 数组索引必须连续 0..N−1；各层 header 必须一致，数据按 layer-major 拼接 | 扫描器 + `assemble_array` | 非连续文件索引报错；格式/尺寸/mip 不一致报错 | 连续/缺层/混合 header fixtures |
| cache 依据完整 desc、源 path mtime、magic+version；不检查 payload | `cache_is_current` / per-asset 编译 | 有效八字节 header 即可命中，即使 payload 缺失；版本不匹配重建 | 实际临时 PNG→输出→header-only命中→旧版本重建闭环 |
| 元数据坏/缺失时重建；名称排序；任务全部完成后收集结果并写 meta/list | 原 easy-parallel 3.2.0 的 `Parallel::each(...).run()` | 已替换先前顺序执行候选，原调用给每个 descriptor 一个任务，没有 threadcount/chunks 参数；复用库 spawn/join，不手写线程池。错误或 panic 不写 meta/list | 成功编译、错误任务仍完成其他输出、panic先join再传播、只读 list 388 名称 |
| Inheritable Option 接受裸值、Some、None、unit；枚举名字必须保留 | 原 RON RawValue + typed decode、IMPLICIT_SOME；unit 经原 RON parser 识别 | 原 RON fork 用 serde Content type_name 精确识别。Rust 1.97 多出 `<'_>`，旧 untagged 草稿丢失 Code 标识；薄适配修复，依赖源码未改 | 裸 Code / Some(Code) / None / () / 坏 enum fixtures；25 发行 prefab |
| prefab scale/albedo 使用 RON 三元素序列 | 通过 Serde 的 Vec<f32> 边界检查后转 `[f32;3]` | 旧草稿普通数组 Deserialize 错要求 tuple；已按发行输入修复。原 native 类型名仍不作已证实声明 | 发行输入解析、长度错误拒绝、继承覆盖 fixture |

原 texture 文件 magic 为 `0x48502183`，pipeline version 为 8，归档 root 28 字节。使用原 RKYV fork 的 `archive_attr(repr(C), derive(bytecheck::CheckBytes))`，修复的是依赖 API/格式兼容，不是游戏算法。388 个发行文件（311,679,016 字节）全部经验证解档并逐字节回写相同，包含普通图、9 个数组和 3D LUT。字段类型/作者 enum 名称、原 native 布局仍没有因此被证明。

## 验证边界与剩余工作

- `cargo test -p asset -p asset-pipe --locked --offline`：当前 19 个测试通过，其中 388 个发行纹理、25 个 prefab / 28 个依赖文件、扫描/继承/流水线契约与正常/错误/panic 编译闭环，以及新继承/名称借用/具体默认值合同。
- `cargo clippy -p asset -p asset-pipe --all-targets --locked --offline -- -D warnings` 与定向 fmt 通过。
- `gamma-verification.json`：仅 OETF 私有 64B 原指令差分；公共 powf 调原 Windows UCRT，未重实现库数学。
- `bounded-report.json`：B SHA 校验后限定准备/批量查询 9 个私有函数成功。脚本/工具调用不是业务验收。
- 原 `assets-src/textures` 只有 16 个 RON 配置，没有 PNG/JPG/DDS 原输入。曾尝试真实源图黄金重编译，发现扫描零图后停止，没有改原发行目录。完整 image pipeline / compression 输出对发行 payload 的重编译差分未完成；不把 archive roundtrip 计成重编译一致。
- prefab 原字段类型、所有 unresolved-default 与坏输入错误/副作用路径、原日志/CLI 标志、country-core 热重载消费者整合仍待核对。任务调度已复用原 easy-parallel 的 each→run。
- 接口库存见 `interface-coverage.json` / `interface-coverage.md`：包括 B11 文件的 inlinee 声明，不只使用 emitted procedures 下界；私有接口、closure、typed schema 规则和库/派生胶水分别列出。
- A / C 的独立二进制、锁文件身份和新文件业务要另立工具项目与证据；本轮完整模块验收继续为 false。

第二批补 11 个对应 API 实现，其中 5 个此前未触及；统一接口台账为 61 个已观察接口、42 个已分析/有实现、19 个尚未触及。浅继承四种纹理实例的全部有效联合标签组合 160,000 组原指令差分为零差异。原 country-core 运行时仍未接线；泛型 concrete_default 的 indexed owner/bounds 未因此补造。详见 [interface-batch-02/DELIVERY.md](interface-batch-02/DELIVERY.md)。
