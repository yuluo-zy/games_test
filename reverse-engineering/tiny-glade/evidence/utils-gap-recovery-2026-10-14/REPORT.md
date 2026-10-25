# utils 剩余接口逆向与消费者合并

2026-10-14，交付目录 `D:/game/tiny-glade/crates/utils`。研究原 B 发行 RNE/PDB；未研究 tiny-garden，未改旧 shared-foundation 研究源码。所有依赖继续复用。

后续修正：本报告为当时源码快照。当前样条/Curve/投影增量见 [B 深挖报告](../utils-deep-followup-2026-10-14/REPORT.md)。下文 `pointer/length` 表述过强：四字节 `mov rax,rcx; ret` 只证明指针返回，RDX 未被改动不证明长度或 slice 接收者。原借用类型仍未知；后续已单独证明有所有权 Curve 的 XZ/12→8 投影。旧 90 项验收不自动覆盖后续源码。

## 已恢复的源码与消费链

`IndexCondenser::insert` 的真实内联身份是 IPI `0x74ee`，两个宿主为 module 523 `OwnedGpuWorldSubset::extend_no_data`（`0x140992280`）和 module 1622 `GpuWorld::create_instances`（`0x14156d0d0`）。这两处都先查询 key：重复时返回到调用者 next_id 重试，不改数组或映射；缺失时取旧数组长度，push ID，再写 HashMap 的 ID→index，交给后续 Condex/InstanceKey 消费。

共享 utils 实现 `insert -> Option<Condex<K>>`、保留缓存 get/clear、补 `remove_index` 返回旧稠密下标。`K` 是安全键适配，原发射实例的数据 key 为 u32；`Option` 表示原成功/失败分支，不是恢复了 PDB 缺失的 Rust ABI。已有 bool remove 兼容接口委托 remove_index。

country-core 的 IndexCondenser 改为委托共享 utils，仅保留类型标记和 `(id, &mut hint)` ↔ Cell 适配。实际 `OwnedGpuWorldSubset::allocate` 使用共享 insert 的失败分支继续递增 next_id，update/remove 通过共享缓存修复/返回下标同步载荷数组。已有消费者测试覆盖 ID 回绕撞车、容量拒绝、载荷 swap-remove 和旧 key 缓存修复；这是真实恢复源码接线，使用 MockDevice 的 CPU 测试不计原完整 GPU 进程验收。

FloatGridCells/固定 512 网格适配迁入共享 utils，country-core 原名称重导出；`RasterBufferRef<512>` 继续按 u8/255 提供格点。显式模式入口与全局模式入口共用原 FloatGridSampler 算法，保留 legacy 上边界 frac=2 的外推、centered 半格偏移、NaN 路径和 lerp 运算顺序。

## 原证据及差分范围

- [pdb-scan.json](pdb-scan.json)：新扫描原 PDB 全部 4894 模块，按 LF_FUNC_ID 的完整定义 scope 关联 PROC/INLINESITE；原 SHA 绑定。扫描脚本不算游戏源码交付。
- [mcp-result.json](mcp-result.json)、[修正后的搜索](mcp-search-corrected-result.json)：Windows 官方 SDK→ReVa/Ghidra 独立持久 worker `utils-gap-20261014`，patched B SHA `cf048e27…`。两处 insert 区段 DB 字节与发行 RNE 相同；未修改/合并原数据库。首次搜索漏设 regex 且 shell 展开造成错误 pattern，其零结果不作语义证据，后续按单独名称修正。
- [index-native-fixtures.json](index-native-fixtures.json)：发行 RNE SHA `f507a6ce…`。两处原 insert 内联 52 输入；完整 `0x140a19f50` remove 入口 78 输入。比较数组顺序、全部映射、map_len、结果与更新 cache。std/hashbrown 哈希、查找、插入、移除执行原指令；输入提供原布局表和充足容量，仅继承分配/内存库桥，避免增长，不覆盖分配失败或巨大索引。insert 重复分支在原 caller 重试入口停止，不宣称完整 producer/callback 执行。
- [grid-native-fixtures.json](grid-native-fixtures.json)：发行 `0x140a14df0` 的 byte-grid 原入口 100 输入，两模式各 50。非对称非线性格点、不同 X/Z extent、上下边界、NaN/∞/零 extent，输出逐位相等；没有替换业务算法或注入预期输出。验证只覆盖该 512×512 单态化，其余网格仍待验收。

采样开关的 public symbol 是 `utils::float_grid::USE_FLOAT_GRID_SAMPLING_FIX`。原机器码先读指针槽 `0x14362c308`，再读其指向的字节 `0x143638fac`。旧 country-core 合同把槽误称字节；已纠正。第一版验证器也错写槽，mode=true 的输入无效，首例 candidate=1056536954 / oracle=1036568777；[旧无效样例](grid-invalid-pointer-fixtures.json) 保留，绝不作算法结论。修正后重新执行并更新包内 fixture。

[Ghidra xrefs](flag-xref-result.json) 未记录两地址引用，不能解释为没有调用者。[原 .text RIP 扫描](flag-raw-references.json) 找到 6 处指针槽读取，邻近机器码为采样分支；仍未定位写入者。直接/间接、运行时计算引用的边界已登记。

## 仍无足够证据的接口

| 接口 | 独立过程/内联宿主 | 处理 |
|---|---|---|
| WallSpaceCoord::bound/is_bounded/to_affine3 | 全 PDB 扫描无匹配；有 IPI 声明 `0xaefe/0xaefd/0xaef7` | 保持未实现，不由缺记录断言未编译或死代码 |
| Spline2D::find_segment_and_dt/position_at_t/tangent_at_t | 无匹配；有声明 `0xaf37/0xaf56/0xaf57` | 构造/局部求值保留，不从 3D 镜像补假算法 |
| Vec3SliceView2 投影轴/元素 stride | 原 as_vec2 `0x14067dd80` 只有 4 字节 pointer/length 返回 | 保留安全借用传输，不添加猜测 get/iter/紧凑 Vec2 slice |

这些未知方法已从正式包移除，没有 experimental-reconstruction 特性。B 的负搜索只限记录范围；A/C、真实全部游戏消费者和原完整进程运行本轮未验收。

## 验收与数值边界

最终状态以 [validation.json](validation.json) 和各阶段日志为准。Debug/Release 分开记录；country-core 1142 项 CPU 测试通过；utils/country-core all-targets 严格 Clippy 通过。

Windows Release 首次原圆形样例相差 4 ULP（1059277456 vs 1059277460），失败记录保存在 [原 Release 日志](release-before-runtime-input.log)。只将样例 ingress 加 black_box，要求实际运行浮点/CRT 路径；所有原输入 bits、预期 bits 和 Windows 逐位断言不变，生产圆形算法未改。Debug/Release 最终各 90 项通过（87 unit + 3 integration），无失败/忽略。这支持编译期求值影响的解释，但未逐项核对 LLVM IR 中间值；不证明任意常量表达式都复制原 CRT，也不将非 Windows 的容差验收升级为逐位等价。

锁文件检查的全 workspace 公共依赖 502/503 身份匹配；唯一既有例外 gpu-allocator 0.27.0（原 local 0.26.0）在渲染路径，未由本轮新增。首次 `--locked` 因并行工程补依赖失败，离线更新只补现有 theme-fetcher 的 log 边；之后 peer 更新 workspace 锁已分别绑定到实际阶段输入，不覆盖或声称锁无变化。utils 本轮未新增生产/测试依赖。

源码、Cargo 输入和原证据 SHA 登记在 `tiny-glade/provenance/utils-gap-audit.json`，历史导入/旧报告不自动覆盖本轮代码。库整体及游戏完整功能验收仍为 partial。
