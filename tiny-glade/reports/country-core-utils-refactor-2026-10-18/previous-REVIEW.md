# utils 源码正确性审查（2026-10-18）

审查者：当前 Codex 主会话。范围：本目录全部 12 个 Rust 文件，以及共享索引/采样/Transform、Mesh、资产库、指示符构建器、读档和部分实际消费者。业务说明见 [README.md](previous-README.md)，可重复反例与源码指纹见 [报告](../country-core-utils-review-2026-10-18/README.md)。

结论：**changes_required（当前源码健壮性范围）**。确认 2 个边界问题；另外记录 2 项业务接线缺口及过期注释。此次未修改业务实现，不是冻结候选的独立交付验收，也没有原二进制运行差分；原作等价性仍需补证（needs_evidence），不输出 passed 或 verified。

## F1 · P2 / major：保守栅格遍历在整数上界回绕

位置：[conservative_line_raster.rs:68](../../crates/country-core/src/utils/conservative_line_raster.rs#L68) 的闭区间行遍历，以及 [第 79 行](../../crates/country-core/src/utils/conservative_line_raster.rs#L79) 的列推进。

输入 `conservative_plot_line((0,3e9),(0,3e9),0,callback)` 是一个有限坐标的退化点。主轴坐标转换为 `i32::MAX`，首格发出后，`col.wrapping_add(1)` 回到 `i32::MIN`。因为仍小于终点，代码继续枚举约 2^32 个主轴坐标，最终才可能退出；它不是严格的无限循环，但对同步编辑路径已等同长时间卡死。

对偶反例 `((3e9,0),(3e9,0),0)` 的次轴上下界都是 `i32::MAX`；`hi.wrapping_add(1)` 变成负值，整个闭区间被跳过，没有任何回调。

影响：`DirtyTiles` 通过回调丢弃网格外格子，但丢弃发生在枚举以后，不能避免超长遍历。其公开笔画入口没有在调用前验证坐标有限性/范围。正常约 ±65 世界范围没有触发此反例；目前未证明正常游戏操作能产生这种极端数据。

证据：[raster-probe.rs](../country-core-utils-review-2026-10-18/raster-probe.rs) 直接包含本次源码，回调第 4 次主动中止并断言 `MAX → MIN → MIN+1 → MIN+2`，不会等待数十亿次遍历；另一测试证明次轴漏格。[日志](../country-core-utils-review-2026-10-18/raster-probe.log) 包括原有 8 测试与 2 个新增反例。

建议：在首列处理后先检查是否已到终列；使用不会计算 `hi+1` 的闭区间迭代。对 DirtyTiles 的有界用途再验证输入，或采用经验证的有界裁剪入口，防止枚举海量网格外格。裁剪需要保留半径与边界覆盖语义。此为当前源码健壮性问题，本轮没有证明原版是否也保留同样行为；任何改变应登记相对原版的策略差异。

## F2 · P2 / major：热重载成功发布不兼容的位置属性

位置：[load_json.rs:63](../../crates/country-core/src/utils/load_json.rs#L63) 的属性校验与发布；消费路径为 `AssetMeshLibrary::try_reload_mesh_src` → `Mesh::positions_without_indices` / `positions`。

下面的文件通过了当前所有检查：

```json
{"indices":{"buffer":[0]},"attributes":["Vertex_Position"],
 "Vertex_Position":{"type":["float",2],"buffer":[[1,2]]}}
```

`float2` 是受支持的通用属性类型，长度也覆盖索引，因此加载返回 `Ok`。资产库随即覆盖原先有效的 Mesh；后续位置访问强制要求 `Float3`，触发 `incorrect attribute type` panic。热重载的“失败保留旧资产”机制不能保护这种语义错误，因为加载器将其视为成功。缺失位置属性也没有在加载层拒绝。

证据：[probes.rs](../country-core-utils-review-2026-10-18/probes.rs) 的首个测试创建有效资产、写入上述文件、调用真实热重载入口，并捕获读取替换后位置的 panic。完整 crate 构建被目录外输入接口错误阻断；随后直接编译同一组当前实现文件，47 项测试通过（含 2 个探针），输入指纹在运行前后一致，详见报告与 source-probes.json。相关源码是 [Mesh 类型访问](../../crates/country-core/src/render/mesh.rs#L198) 和 [资产替换入口](../../crates/country-core/src/asset_libraries/mesh_library.rs#L162)。

建议：为需要位置的网格消费者在资产发布前校验 `Vertex_Position: Float3`，并按其用途校验 UV/法线等保留语义；不应把所有自定义属性都限制成固定格式。应返回 `Err` 让热重载保留原资产。通用 Mesh 若允许无位置数据，应由具体消费方声明所需合同，而不是无条件禁止所有此类 Mesh。

这是当前消费者合同的健壮性缺口，**不据此认定偏离原作**。历史原算法说明主要核对属性种类与最大索引，未提供原版按保留名字检查类型的正面证据。

## 接线缺口（与上述两个问题分开）

### G1：采样开关没有生产写入者

[float_grid.rs](../../crates/country-core/src/utils/float_grid.rs) 再导出的 `CENTERED_PATH` 与共享 crate 是同一对象；委托本身没有重复状态问题。`LoadVersionGates` 也已实现版本阈值与写入方法。

但本轮全工作区 Rust 源码检索仅找到该写入方法的测试调用，实际 `load_game_session` 只返回 gates，`load_game_system` 没有应用它。若宿主未另行设置，新版存档仍走初始 false 的旧采样路径，产生半格偏移/边界行为差别。应在加载世界数据前写入同一共享开关，并核验切换新旧会话的顺序；这属于加载链缺口，不能只改本目录转发文件。

依据：[save_file.rs:246](../../crates/country-core/src/resources/save_file.rs#L246)、[load_game.rs:610](../../crates/country-core/src/systems/save/load_game.rs#L610)、[load_game_system](../../crates/country-core/src/systems/save/load_game.rs#L643)、共享 `utils/src/float_grid.rs`。本轮采用源码调用检索，未运行完整读档场景。

### G2：调试请求尚未闭合到可绘制实体

[DebugLine::show](../../crates/country-core/src/utils/debug_viz.rs#L298) 的闭包保留 mesh/shader 库参数但不使用；[SignifierEntityBuilder::build](../../crates/country-core/src/systems/ui/signifiers/signifier_factory.rs#L418) 仅生成实体及部分指示符组件，没有解析网格/shader、插入完整 drawable/pass。球和三维矩形也没有 `show` 实现。

因此已有 `show_packs_the_record_and_wakes_the_dashed_line_closure` 测试只覆盖入流、唤醒和拓扑/虚线参数。不能用该测试通过作为绘制成功证据。这是代码已经明确登记的恢复边界，本轮没有将其算作新发现回归。

## 当前读到的正确合同

- DirtyTiles 的清空、OR 合并、快照清空、行优先遍历和 StateDelta 委托互相一致。矩形上界 63 且半开的规则虽然不直观，但代码和现有合同一致，本轮不判为 off-by-one 错误。
- IndexCondenser 对重复 ID 拒绝插入；解析与移除委托同一个共享实现，缓存提示会修复。必须由消费方同步交换并行数组。
- ChildWall 检查插值结果与 NaN；不要求分割参数或结果顺序属于当前接口合同，不能直接新增排序/clamp 改变语义。
- AudioLoop 首次 stop 记时、严格超过阈值才停止、播放请求取消计时、失败不推进 cursor 的控制流一致。空集合/任意修改公开 cursor 不受保护，应由构造和调用方维持前提。
- Resource 和事件 helper 直接使用 Bevy API，避免手写容器/事件计数逻辑。drain/last 都是破坏性消费；需要多 reader 的系统应使用 EventReader。
- Transform 默认单位缩放、forward=-Z、TRS 和共享 look-at/比较的调用一致。没有把这些依赖委托计作独立恢复算法。

以上只表示静态审阅范围内未见其他确定错误，不替代完整原版验证。

## 注释与验证限制

- `load_json.rs:80` 仍写“读文件/解析失败仍 panic”，当前两个 `?` 实际返回 `Err`。这与现行热重载策略一致，旧注释和旧原版差分结论须分开使用。
- `audio_utils.rs` 顶部仍说未做 `instantiate_loop`；当前实现已在 `audio_resources.rs:497`，不能继续列为完全缺失。
- `geo.rs`、投影/浮点函数和 debug 构造器的大量 O 地址来自历史恢复注释，本轮没有重新查询对应原始字节。`compute_affine` 的 Rust 返回类型为 `Mat4`，本文不把它的名字或布局注释当作已验证的原 ABI。
- 没有本轮冻结 manifest、完整原生证据链和原版进程运行结果，因此不能给出正式 `passed_scoped`、全目录完成或 B 等价结论。
