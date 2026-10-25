# utils 顶层 22 文件独立代码审查

范围仅 `utils/src/*.rs`，排除 `lib.rs` 及 components/curve/geometry；按原作 B 的函数接口和已有原指令证据复核。当前代码可编译，但**业务功能尚不完善**：191 个已观察原接口中，99 个有局部业务分析，92 个未分析，88 个没有映射实现。未分析项包含仍待确认的 trait/schema 方法，不能把它们全部称为已证实缺失的游戏 API，也不能用测试通过率替代接口覆盖率。

本轮修复 2 项，确认 1 项原作也存在的输入风险，记录 2 组重要功能缺口和 1 组尚未实测的性能风险。逐文件接口、缺口、位置和当前 SHA 在 [review.json](review.json)，真实命令和 22 个源码 SHA 在 [validation.json](validation.json)。历史差分报告保持原样，两个修改文件的历史源码 SHA 已过期。

## 已修复

| ID | 类型 / 级别 | 原接口与问题 | 修复与验证 |
|---|---|---|---|
| UB-R02 | 依赖重复实现 / P2 | Spline 的 knot 生产者仍手写向量减法、dot 和 sqrt；合同却称向量数学复用 glam | 改用原 glam 0.29.3 的 Vec2/Vec3 subtraction + length；保留游戏弦长顺序与 scalar spline solver。44 个原构造快照、120 个位置及 120 个切线查询的既有回归通过，门限仍为原 1e-5 |
| UB-R03 | 性能缺陷 / P2 | SelectiveDeserialize 为每个忽略的 map key 构造 String，仅用于比较字段名 | 通过 Serde visitor 直接比较 borrowed key，保留 deserialize_string、UTF-8 形式、字段顺序及吞错/default 业务。50,000 个 ignored keys 上，分配从 50,003 次减至 1 次；载荷相同。12 个既有策略案例及 escaped key + late-error 回归通过 |

分配探针 [selector-allocation-probe.rs](selector-allocation-probe.rs) 直接链接 Cargo 选中的原 serde 1.0.200 / serde_json 1.0.85 和当前 utils。旧候选保存在 [selective-before.rs](selective-before.rs)。这里比较的是候选优化前后，**没有宣称完成原生 Save-GC visitor 对照**。单次 debug 耗时仅为本机佐证，不是游戏帧率测试。

## 待解决

| ID | 类型 / 级别 | 可复现问题与边界 | 后续必须补的证据 |
|---|---|---|---|
| UB-R04 | 原作共有输入风险 / P1 | Rdp::new_with_opts：负 epsilon 且没有可用 split 时，split_offset=0，反复排入同一区间，输出增长且没有进展。三点共线、epsilon=-1、spacing=0 可触发 | 原入口 0x141fe0b90 在 3,000,000 指令预算内也未返回；两组正 epsilon / 重复点控制案例正常返回。先确认真实消费者是否保证 epsilon 非负，再裁决安全 adapter 是否应拒绝该输入，不能误报为恢复偏差 |
| UB-R05 | 功能不完整 / P1 | Vec3SliceAs2::as_vec2 只能返回原 Vec3 storage，无法读取或迭代投影后的 Vec2 | 四字节原入口只证明 pointer / length 不变。须取得原调用方的返回类型、轴及 stride；禁止将 stride 12 的 Vec3 storage 强转为 stride 8 的 Vec2 slice |
| UB-R06 | 功能不完整 / P1 | WallSpaceCoord 的 bound/new/offset/affine、WallSpaceRegion::intersects、LoopSlice/map_slice、Rdp new/stitch、Spline2D segment selection 等尚无已确认实现 | [逐接口清单](../../shared-foundation-2026-10-13/utils-base/interface-coverage.md)。原 Time/SystemParam、terrain/grid、save snapshot payload 及完整消费者仍未接线 |
| UB-R07 | 性能风险 / P2，未实测 | RDP 最坏情况反复扫描可达 O(n²)；Spline 有多个 O(n) 临时 buffer 和重复 prefix accumulation；local_time 每次读锁 | 先补真实调用量、控制点规模和热路径 profile。当前没有证据表明这些已造成游戏性能故障，不作臆测优化 |

[rdp-boundary-probe.json](rdp-boundary-probe.json) 记录原字节身份、预算和停止 PC。预算耗尽不等于形式化证明死循环；候选递推及原 PC 支持风险判断，真实调用前置条件未知。没有运行无界候选进程。

## 每个文件的复核结论

| 原文件 | 已实现的主要业务 | 完整性 / 重点缺口 | 性能与依赖检查 |
|---|---|---|---|
| angle_range.rs | 环绕与 range clamp | wrap_angle、构造边界、NaN/tie 域不完整 | 私有角度策略；使用 std 浮点运算，无通用数学库替身 |
| camera.rs | reverse-Z projection、body/eye helper、XZ rotation、Lens 初值 | body trait、TAA jitter、through、Lens smoothing / 资源生产者不完整 | Mat/Quat/normalize 已复用 glam；固定规模，无额外堆分配 |
| circular_f32.rs | wrap、distance、shortest lerp | project_unbounded、完整 serde/schema 及 tie/NaN 域未验收 | 自有环形策略，Serde 复用；没有另写序列化器 |
| comparative_space.rs | circular project / subtract / distance | 其他原 impl owner、lerp_clamp、region classification 尚未确认 | 单步 wrap 业务保留，不能根据名称改成多圈最短距离 |
| easing_functions.rs | 枚举映射与 easing 分支 | shoulder 等原 helper 接口、非有限和超范围域不完整 | 私有组合公式；使用 std transcendental，不另写数学库 |
| file.rs | 日期、backup path 和 copy | PathTail、IO failure / logging / non-UTF8 行为未全面验收 | std/time 复用；文件复制无需自写 buffer |
| float_grid.rs | texel-center fix 与 legacy 两种 bilinear policy | 原存储类型、flags、dimensions/load consumer 缺失；不能从 ByteFloatGrid adapter 推断全 grid | 固定 4 次 load，当前无已确认性能故障 |
| halver.rs | f32 / array / glam 类型转 f16 | 所有原实例及生产者未验收 | 直接 half 2.4.1，未见 half 转换轮子 |
| index_condenser.rs | stale cache repair、get/remove/clear | insert/default 的真实 key producer 及游戏消费者缺失 | Vec + HashMap 复用；缓存快路 O(1)，共享 Cell 更新不需要假容器实现 |
| local_time.rs | offset cache refresh、UTC fallback、conversion | now_appox_local 和平台刷新 workflow 未验收 | time + std lock 复用，锁成本待真实调用频率确认 |
| loop_range.rs | 原 signed remainder、bounds、subslice | LoopSlice/map_slice 及 next/back owner 行为未单独验收 | std Iterator adapter；ExactSize 默认 len 已正确，不需重写 |
| name_hash.rs | 原 xxh3 及 debug label | 部分 trait / 实际 name producer 未验收 | 直接原 xxhash-rust，无 hash 轮子 |
| natural_cubic.rs | open / periodic solve、构造及 3D evaluate | 2D segment selection、退化/重复/非有限点与完整周期压力未验收 | 已清除 knot vector math 轮子；私有 scalar solver 为 O(n) |
| rdp.rs | 原 split / spacing / force / shortcut | new/stitch/into_indices 和负 epsilon 前置条件缺失 | 私有变体需保留；最坏 O(n²)，并非可随意替换的标准库调用 |
| rgb9e5.rs | 原 rounding / shared-exponent packing | clamp/log2 helpers 与全部非有限域未逐接口验收 | 原自有 packing policy；不把 ISA NaN 操作序当任意 f32 clamp |
| rng.rs | 原游戏 seed affine mix | typed producer 的所有 downstream caller 尚未接线 | fastrand 1.8.0 完成生成 / shuffle / distribution，无 RNG 轮子 |
| selective_deserialize.rs | History→snapshots、吞错/default 与 partial-success | 原完整 visitor / snapshot 类型未验收 | 已消除每 key 分配；decode/escaping/UTF-8 仍交给 Serde |
| subtractive_1d_boolean.rs | endpoint policy 与 subtract | sequential cuts、非有限/反向范围、所有 old flags 未全面验收 | std Vec 复用；有 O(ranges) 扫描和输出分配，真实规模未知 |
| swizzlers.rs | 四个坐标 adapter，borrowed transport | as_vec2 projection consumer 缺失 | glam 构造；未引入 unsafe slice cast 或 Vec2 repack |
| threshold_range.rs | 连续 low/high runs 与 callback policy | NaN 和消费者 singleton retention 未完整对照 | O(n) 单次扫描、无 heap allocation |
| timer.rs | warmup + delta 上限 + 4 samples filter | 原 Bevy Time / Local / SystemParam 完整接线缺失 | std VecDeque 复用；没有自写 Bevy scheduler |
| wall_space.rs | 距离/坐标、curve bridge、tangent 和 extension | bound/offset/affine/region 及真实墙消费者大量缺口 | 复用 CurveSampling 与 glam；没有复制曲线最近点算法 |

## 验证与依赖结论

当前 `cargo test -p utils --locked --offline` 和 `cargo clippy -p utils --locked --offline --all-targets -- -D warnings` 均通过。完整 utils 测试包含 peer 的 geometry/curve，不能用这个数量计本 owner 的原接口。30 个 utils 可达外部依赖的 version / source / checksum 都与原 Cargo.lock 匹配。

本轮最初将 LoopRange inherent len 在 reversed subslice 时返回 usize::MAX 判断成 ExactSizeIterator 缺陷，复核后排除：原 helper 本来就允许这个结果，std trait 默认 len 使用 size_hint，已返回 0。临时 override / 冗余测试在交付前撤回，源行为未改。对应 UB-R01 在机器报告中明确记为 dismissed，不计修复成果。

没有发现其他已确认的 RNG、f16、hash、JSON parser、容器或 Bevy 替身。此结论只覆盖上述 22 文件；不代表所有历史恢复代码已审完。原样条 scalar solver、RDP 变体、RGB9E5 和游戏策略有私有归属证据，保留它们不属于重复实现通用依赖。
