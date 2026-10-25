# rhapsody rg 域 审查：未补全 / 未探究清单

- agent：`rhapsody-rg-review`。日期：2026-10-15。只读审查，不改任何源码与既有文档。
- 审查对象：`domains/rg/ALGORITHMS.md`（304 行，agent=rhapsody-rg）vs 骨架清单 `domains/rg.json`（198 条目 = 107 compiled_procedure + 90 inline_instance + 1 inlinee_declaration；57 有地址）。
- 证据：`domains/rg/decomp/`（61 文件 = 51 真实反编译 + 11 NotFound 桩）、共享 `rhapsody/decomp/*.c`、`../CALL-CHAIN.md`、rebuild `reconstruction/rhapsody-rebuild/src/rg/**`。
- 标注沿用：**O**=机器码/PDB 实测、**I**=推断、**U**=未知；✓=已分析 / △=表面 / ✗=未覆盖。

## 0. 总体核对结果（先给结论）

| 核对项 | 结果 |
|---|---|
| 107 个 compiled 函数逐项核对 | ✓ ~86 / △ ~14 / ✗ **7**（详见 §1.1） |
| 57 个有地址函数是否全进 ALGORITHMS.md | **是**（0 缺失，地址级已程序化比对） |
| 51 个真实反编译结论是否落实为 O | **是**（51/51 均为 O；无"反编译了但结论仍 I/U"的地址） |
| 11 个 Ghidra NotFound（82B 桩） | **全部只有 I/U 结论**，无一有原字节证据（详见 §2） |
| 业务链断点 / 算法 U | 屏障计算、OOM 降分辨率、池回收闭环、compile ABI、时态状态机（详见 §3） |
| rebuild 对照 11 缺是否逐项消化 | 名义上全在 §3 有行，但 8 项仍是"半"未闭合；并发现 1 处**错误结论**（§4.2） |

**本审查最重要发现（纠错）**：ALGORITHMS.md §3 与 `round3/MODULE-DEEP-DIG.md` 登记的
**"`image_access_mask_to_usage_flags` rebuild 数值表与原 0x140bd49d0 全错位"是错的**——
我逐输入→输出核对 12 个映射对，rebuild（`reconstruction/rhapsody-rebuild/src/raw/resource_usage.rs`）
用 ash `ImageUsageFlags` 常量（TRANSFER_SRC=1 / TRANSFER_DST=2 / SAMPLED=4 / STORAGE=8 /
COLOR_ATTACHMENT=0x10 / DEPTH_STENCIL_ATTACHMENT=0x20）的**数值输出与原机器码完全一致**：
原 `0x20→4、0x40→8、0x80/0x100/0x180→0x10、0x200/0x400/0x600→0x20、0x220→0x24、0x800→1、0x1000→2、0x18000→8`
⇔ rebuild `0x20→SAMPLED(4)、0x40|0x18000→STORAGE(8)、0x80|0x100|0x180→COLOR_ATTACHMENT(0x10)、
0x200|0x400|0x600→DEPTH_STENCIL_ATTACHMENT(0x20)、0x220→DS|SAMPLED(0x24)、0x800→TRANSFER_SRC(1)、0x1000→TRANSFER_DST(2)`。
除结构上（原单值分支 vs rebuild `0x40|0x18000` 合并臂、panic 形式）外**数值零差异**。
**"高优先修正项"应作废**：不要在下一轮去"修正"本来就正确的 rebuild 表。详见 §4.2。

---

## 1. 未覆盖函数表

### 1.1 compiled_procedure 完全无分析行（✗，7 个）

这些在骨架里是 `compiled_procedure`（真实编译实体），但 ALGORITHMS.md §2 无对应行，§1.1 也只提名字或根本不提。

| 函数 | 骨架文件:行 | 地址 | 卡点 | 建议补法 |
|---|---|---|---|---|
| `import_export::impl$2::into_sampled_rg_texture` | import_export.rs:182 | —（无 va） | **唯一业务型 compiled 缺口**。只在 §1.1 文件职责表出现一次名字，无 role/verdict。rebuild 有候选 `resource.rs::Ref::into_sampled_rg_texture`（157 行，转发 bind_view），但原函数地址未定位、与 `resource::Ref::into_sampled_rg_texture` 是否同函数未定 | 在 `resolved-calls.json` / `0x1404e8010` / `render_world` 调用面 grep 调用点做 xref 定位真实地址；或对照 rebuild Ref 方法与 CALL-CHAIN §3.1 rg::resource 行推定语义 |
| `recorder::impl$12::fmt` | recorder.rs:176 | — | 无行。可能是某类型 Debug/fmt 派生 | 低优先；xref 看谁调 fmt；多数情况属 derive 可复用 |
| `recorder::impl$4::default` | recorder.rs:182 | — | 无行。Default 派生 | 低优先；按类型合同（Default）处理 |
| `resource::impl$4::from` | resource.rs:238 | — | 无行。From 转换 | 低优先；xref 判目标类型 |
| `resource::impl$5::from` | resource.rs:244 | — | 无行。From 转换 | 低优先 |
| `resource::impl$39::fmt` | resource.rs:271 | — | 无行。Debug fmt | 低优先 |
| `resource_registry::impl$2::fmt` | resource_registry.rs:41 | — | 无行。Debug fmt（Registry 布局已有 O 数据，可顺带定） | 低优先；结合 §1.2 布局一起 |

### 1.2 inline_instance / closure 未覆盖（价值分级）

业务相关、建议补：
- **`ResourceSyncGroup::can_be_concurrent_with`**（recorder.rs:197，inline）——未分析行。与 `PassResourceAccessType::can_be_concurrent_with`（0x140bece70，已 O）并列的组级判定，语义应相近但未核对。建议：xref 找调用点，对照 0x140bece70 分支结构手解。
- **`resource::Ref::raw`**（resource.rs:344，inline）——未分析。trivial（返逻辑号），可并入 §2.9 句柄族。
- **`resource_info::ResourceInfo::allocate`**（resource_info.rs:55，inline）+ `allocate::closure$0`——未分析。`calculate` 内按图资源数扩容的分配策略，涉及 usage 表与 access 记录初始化，是生命周期折算的前置步，建议在补 §3-链 时一并解。

派生/胶水类（价值低，列全供核对，不需单独深挖）：`binding::find_resource_access_type::closure$0/$1`、`rg_binding_to_descriptor_binding::closure$1/$2`、`compiled::new::closure$0`、`record_batch::closure$0/$2/$3`、`executor::execute_compiled::closure$0`、`submit_pending_cbs::closure$0`、`pass_api::begin_render_pass::closure$0/$1`、`resource_registry::into_exports::closure$0/$1`、`temporal::get_or_create_temporal::closure$0`×2、`graph::impl$12::default`、`executor::impl$16::default`、`resource_info::impl$7/impl$17::eq`、`impl$31::partial_cmp`、`resource_pool::impl$1::default`、`resource_registry::impl$5::default`、`recorder::impl$14::fmt` 等（凡无地址 inline 均未进表，属 derive/fmt/Default 合同，可整批归为"复用 derive"不逐个登记）。

---

## 2. 表面分析表（有行但 role/算法/行为空泛，或缺机器码证据）

全部 11 个 Ghidra NotFound 地址（82B 错误桩，`{"error":"No function at ...","error_type":"NotFound"}`）在 ALGORITHMS.md 里只有 I/U 结论，无原字节证据。PLAYBOOK §3.1 明言"NotFound 文件不能作为 O 证据"——这些是下一轮手解（rne-read / rne-disasm）的主目标。

| 函数 | 地址 | 现状 verdict / 行文 | 卡点 | 建议补法 |
|---|---|---|---|---|
| `BatchCompiler::new` | 0x140be95e0（NF） | I"编译实例存在；全零" | 无机器码；execute_compiled 调用点已知 | rne-read 手解（短函数，应可解出零初始化/字段布局） |
| `pass_access_range_indices` | 0x140be9690（NF） | U"编译实例（Pass 访问范围索引）" | **用途本身未定** | 先 xref 找调用方（疑似 compiled.rs 内 pass 访问范围换算，服务于 add_pre_pass_barriers），再按调用点反推参数面；rne-read 补字节 |
| `PassPgoNameId::from_name` | 0x14067dd80（NF） | U"原 hash 算法 U（rebuild FNV-1a 是候选）" | **影响 PGO 分块正确性**；hash 算法未核对 | rne-read 手解该短函数（hashbrown hash_one 或自写 FNV/xxh，比对 rebuild executor.rs FNV-1a） |
| `PassPgoDuration::new` | 0x140be9c70（NF） | I"编译实例存在" | 无内容 | rne-read；duration 采样（QueryPool 写回）可经 PassPgoFrames 结构推 |
| `RenderPassApi::end_render_pass` | 0x1416f7850（NF） | I"编译实例" | 无内容；begin 已 O | rne-read；应为一两处 vtable 调用（对照 begin 的 +0x400 系） |
| `RenderPassApi::set_line_width` | 0x1416f7960（NF） | I"device+0xad9 门控" | **门控证据来自 rebuild 非原机器码** | rne-read 验证 +0xad9 字节门控与 vtable+? 调用 |
| `PassResourceRef::resource_index` | 0x140bece40（NF） | I"取资源号" | 无内容 | rne-read；与 ResourceHandle::raw 同款取号 |
| `ResourceSyncGroup::new` | 0x140bece50（NF） | I"AtomicU32 fetch_add 从 2 起（access_ledger 重建，O/I）" | 非原机器码，重建推断 | rne-read 验证 fetch_add 初值（2 或 1）与递增原子 |
| `ResourceSyncGroup::is_equivalent` | 0x140bece60（NF） | I"u32 相等" | 无内容 | rne-read；cmp/jz 直解 |
| `PassResourceAccessType::new` | 0x140b25c90（NF） | I"构造" | 无内容 | rne-read |
| `ResourceHandle::raw` | 0x140833250（NF） | I"取序号" | 无内容；rebuild 注释"原 raw 入口直接返回 ECX" | rne-read（几字节，可确证） |
| `GraphExecutor::wait_submits_done` | —（inline） | I"设备等待" | 无机器码 | 与 wait_gpu_queue_trim 0x140bea680 对照，应为其子集 |
| `PassPgoStats::reset` / `extend` | —（inline） | I"清表+total/max；并集" | end_frame 交换已 O，但 reset/extend 数据流未直证 | 从 end_frame 0x140bec400 段与 PassPgoFrames 交换代码反推 |
| `CompiledRenderGraph::compute_batches` | —（inline） | I"new 内 0x1402bf880 对应段" | 分块边界/批次生成规则未解 | **0x1402bf880 已见调用点未反编译**——补反编译该地址（FUN_1402bf880），解批次切分 |
| `CompiledRenderGraph::add_pre_pass_barriers` | —（inline） | I"编译实例" | 批前屏障生成算法未解（见 §3-3） | 在 record_batch 0x1402c1d40 前段找其内联体 |
| `CompiledRenderGraph::gather_batch_preamble_barriers` | —（inline） | I"批首部屏障收集" | 同上 | 同上 |
| `PassBuilder::add_resource_access` | —（无地址） | O/I"access_ledger 规则…不兼容 panic" | 引用"0x1409dca90/0x140533b10 区"非单地址 | 把规则来源地址钉死（哪个单态化实例），补至少一个实例反编译 |
| `RenderPassApi::find_resource_access_type` | —（无地址） | I"扫描当前 pass 访问，未声明 panic" | 无机器码 | 在 begin_render_pass / record_batch 内联段找其调用体 |
| `validate_read_access`（impl$0/impl$1） | —（无地址） | I"编译实例（rebuild：空操作）" | **rebuild 断言"两个实例均只有 RET"是候选判断** | 找单态化地址 rne-read 验证是否真为 no-op |
| `import_export` 4 个有地址方法 | 0x140bd60a0 / 0x140bd6380 / 0x140bd63f0 / 0x140bd4fc0 | O（经 resolved-calls） | §2.9 只给一行合并；**impl$0=Buffer?/impl$1=Texture? 归属带问号**（U卡点#9） | 见 §3-9；xref 各地址调用方的句柄类型判别 |

另：§2.7 行文把 pass_builder `render` 记"6 实例"、随后列 7 个闭包名并把 `clear_rg_buffer/clear_rg_color_image`（实为 recorder impl$1::render 的实例）混入，与骨架数（pass_builder compiled 5 + inline 泛型 1；recorder compiled 7）不一致——文档自身计数口误，非功能缺口，登记待修。

---

## 3. 业务逻辑缺口（处理链断点 / 算法 U）

### 3.1 compile→record_batch→execute→temporal→resource 链的断点清单

| # | 链段 | 现状 | 断点描述 |
|---|---|---|---|
| 1 | **OOM 降分辨率编译路径** | 仅 §3 一行"render 0x1404e8010 0x1404e8a3d..0x1404e9516 重试，panic 串 0x1429ebc17 族。CALL-CHAIN 差距#1 未闭合" | **identified 但完全未消化**。CALL-CHAIN §4#1 已给出素材：`min_extent`、`[32,32]` 兜底、写 2 到 TypeId 资源、0x1404e9376 常量。降分辨率后重编译的触发条件（CompileError 载荷）、32x32 资源写入、重试次数/上限全 U。这是 compile 语义里最有业务价值的缺口 |
| 2 | **compile 参数面与 RenderGraphState 布局** | U卡点#1/#2 | `RenderGraphState` 完整字段名与布局 U；`Option<RenderGraph>` tag（i64::MIN）已见但 swapchain_ctx/closure 捕获布局未解；`ResourceExportHandle<Texture>` Rust 定义 U（=CompiledRenderGraph 0x218B 包装？） |
| 3 | **屏障计算核心** | `add_pre_pass_barriers`/`gather_batch_preamble_barriers`/`compute_batches`/`record_postamble` 中 3 个 I | barrier 判定逻辑（对哪些资源、何时、image layout / src+dst stage / access mask 怎么算）未从机器码解。record_batch 的 barrier **调用点**已 O（0x140bd66a0/0x140bd6810/0x140bee030），但"**为何发**"的决策算法没挖。**SynchronizationScope 0x50B（@CompiledRenderGraph+0x1b0，0x140bd6650 构造）字段布局 U**。`compute_batches` 指向 FUN_1402bf880 未反编译（见 §2） |
| 4 | **资源池回收闭环** | take/insert/clear 已 O | **`ResourcePool::insert_*` 的调用方未接上**。谁在退役时把纹理/缓冲还回池？`ExecutedRenderGraph::retire`（0x140be9d00）只跑回调+回收导出，未见回池；`ResourcePool::clear` 在 render 0x6d23 被调。take→create→…→insert 的完整生命周期链中"insert 侧"（可能经 `raw/managed` 或 device 回收）断在 rg 域外，需 xref 0x140be6d70/0x140be7320 调用方确认 |
| 5 | **record_batch 分块预算细节** | budget=clamp(max,total/16,total/2) 贪心已 O | 贪心累加的精确停止条件、块边界与 pass 数关系、串/并行切换字节（U卡点#5：execute_compiled 内 batch+4 与 0x02 比较）模糊；`PassPgoNameId::from_name` hash U（见 §2）直接影响预算按 pass 归属 |
| 6 | **时态状态机** | 命中/未命中/desc 不匹配三支已 O | Record 0x68B 中 `-0x58..-0x40` 字段（desc/token/access 拼装，export_temporal 反编译已见两 qword 未定名）；state 0→2 转移已见但"何时判定 Exported 后允许同帧二次请求 / error 路径（tag=2）完整语义"未收口（U卡点#10）；**`TemporalRenderGraphRecorder` 完整字段（+0xc8..+0x108）U** |
| 7 | **CompiledRenderGraph +0x128 per-resource 表** | U卡点#7 | +0x128 0x18-stride 表与 ResourceInfo::lifetimes（0x18 同 stride）是否同份/何时复制 U；+0x200 待发初始过渡 Vec 由 FUN_1402bf880（compute_batches）产出，其与 ResourceInfo lifetime events 的关系未理清（ALGORITHMS §2.3 说"由 ResourceInfo 生命周期事件构建"但反编译只见 `FUN_1402bf880` 输出直写 +0x200，表述存疑） |
| 8 | **`last_known_frame_duration_ns` 语义错位** | 已证实返 bool（0x140beaae0 阈值比较） | PDB 名 vs 行为错位已登记，但 rebuild 无对应方法；PGO 查询缓冲读取 + device+0x658+0x3a8 浮点系数 + 0x140be0590 收尾函数（域外）语义 U（U卡点#3/#13） |
| 9 | **import_export impl 归属** | U卡点#9 | `impl$0`/`impl$1` 哪个是 Buffer/Texture、`export_temporal` 里 Buffer 导出 0x140bd5120 的 trait 归属 U；`into_sampled_rg_texture` 未覆盖（§1.1） |
| 10 | **PassPgo 数据流** | end_frame 前后帧交换已 O | 哪个 pass 的 duration 进哪一帧、extend/reset 时序（谁调）未直证 |

### 3.2 明确"还是 U"的算法项汇总

pass_access_range_indices 用途、PassPgoNameId::from_name hash、SynchronizationScope 布局、CompiledRenderGraph+0x128 表关系、execute_compiled 批量状态字节、ResourceRegistry::resource byte@2 语义、begin_render_pass 附件缓存键（RenderPassDesc）与 RenderPassBinding 0x44B 布局、TemporalRenderGraphRecorder 完整字段、import_export impl 归属、ResourceExportHandle 定义、CompileError 载荷（§4 补全优先级已归并）。

---

## 4. rebuild 对照缺口

### 4.1 11 缺逐项消化状态（任务指定核对）

| # | 缺项 | §3 行 | 消化状态 |
|---|---|---|---|
| 1 | `RenderGraphState::compile` 三实例 | **半** | 未闭合：rebuild `compile_frame` 无三个游戏闭包实例、无 `ResourceExportHandle` 返回、Result 语义 vs 原 OOM 降级未建模（差距#1 仍开） |
| 2 | `ResourceExportHandle<Texture>` | 半 | 0x218B 编译图对象已实，但 Rust 类型定义 U（U卡点#2） |
| 3 | `TemporalRenderGraphRecorder` + trait | 半 | rebuild 只有 `TemporalRenderGraphState`，无 trait 化。**但本审查核查：rebuild temporal.rs 行为其实高度对齐**（Ustr 键✓、TEMPORAL_EXPORT_ACCESS=0x37✓、:0/:1 后缀✓、export_temporal✓、parity✓）——差在类型/命名而非行为。另发现**未登记差异**：原复用路径用 `Texture::import_unowned`（0x140bd60a0），rebuild 用 `import_owned`——所有权语义（谁持有物理 token）不一致，见 §3.1-9 |
| 4 | `ImportExportToRenderGraph` trait | 半 | rebuild 无 trait、无 export 两套；impl 归属 U |
| 5 | `from_owned_rg_resource` | 半 | rebuild 无该方法；render R9 调用点 0x140bd63f0 已定位，方法体未解 |
| 6 | `RenderGraphRecorder` | ✗名/半行为 | 原对象=`RenderGraphRecord<PassResources>`（drop_in_place 实证），rebuild `Recorder` 同名不同物 |
| 7 | OOM 降分辨率 | 半（identified） | **未消化**，见 §3.1-1 |
| 8 | FnOnce 签名 | 半 | 原 `FnOnce(u64)->bool` vs rebuild `FnMut(PassContext)->bool` 已确认（CALL-CHAIN §4#2） |
| 9 | `image_access_mask_to_usage_flags` | **✗（结论错误）** | 见 §4.2 ——应**撤下"高优修正"** |
| 10 | `buffer_access_mask_to_usage_flags` | ✓ | 逐位一致（0x140bd4b00 vs rebuild 位运算）✓ |
| 11 | temporal 键 Ustr | 半 | rebuild 已用 Ustr ✓；country-core `ustr_contract::intern` 返回 0（差距#10 未闭合） |

### 4.2 本审查纠错（最重要）：image_access_mask_to_usage_flags "全错位" 结论错误

- 原机器码 0x140bd49d0（decomp 全文 68 行已复核）：`0x20→4、0x40→8、0x80/0x100/0x180→0x10、0x200/0x400/0x600→0x20、0x220→0x24、0x800→1、0x1000→2、0x18000→8`，未命中走 `swi(3)` panic。
- rebuild `rhapsody-rebuild/src/raw/resource_usage.rs`：`0x20→ImageUsageFlags::SAMPLED、0x40|0x18000→STORAGE、0x80|0x100|0x180→COLOR_ATTACHMENT、0x200|0x400|0x600→DEPTH_STENCIL_ATTACHMENT、0x220→DS|SAMPLED、0x800→TRANSFER_SRC、0x1000→TRANSFER_DST`。
- ash `ImageUsageFlags` 数值：TRANSFER_SRC=1、TRANSFER_DST=2、SAMPLED=4、STORAGE=8、COLOR_ATTACHMENT=0x10、DEPTH_STENCIL_ATTACHMENT=0x20。
- **12/12 映射对数值相等**。rebuild 只是"单值匹配 + 常量命名"，无一处输出错位。
- 影响：ALGORITHMS.md §3 的"✗ 高优先修正项"、§3 核对结论"发现新 ✗"、`round3/MODULE-DEEP-DIG.md` rhapsody-rg 登记行同一说法——**三处都应作废**，否则集成轮会被误导去"修"本来正确的代码。

### 4.3 对照表本身遗漏的原↔rebuild 项

- `import_export::impl$2::into_sampled_rg_texture`（原 compiled）↔ rebuild `resource.rs::Ref::into_sampled_rg_texture`（157 行）——§3 无行；归属（是 import_export 的 impl 还是 Ref 方法）U。
- `ResourceSyncGroup::can_be_concurrent_with`（原 inline，未分析）↔ rebuild（access_ledger/资源规则）——§3 无行。
- `validate_read_access` ×2 ↔ rebuild no-op 断言——§3 无行。
- §3 行"`recorder::RenderGraphRecorder / RenderGraphRecord<PassResources>`"判定"✗名/半行为"是准确的，但未提 rebuild `Recorder` 的三个方法（record_batch/plan_chunked_batch/finish_chunked_batch）与原 execute_compiled 批量路径的逐点对应关系。

---

## 5. 补全优先级排序（下一轮最值得深挖）

| 序 | 项 | 理由 | 具体动作 |
|---|---|---|---|
| P0 | **撤消 `image_access_mask_to_usage_flags` 误判** | 防止集成轮修正已对代码；本审查已给出逐值证据 | 在 ALGORITHMS.md §3 与 MODULE-DEEP-DIG.md 登记处更正（下轮写入口径），无需改 rebuild |
| P1 | **OOM 降分辨率编译路径** | 唯一"identified 未消化"的高业务价值链；compile Result 语义的钥匙 | 反编译 render 0x1404e8010 的 0x1404e8a3d..0x1404e9516 重试段 + rne-read 常量 0x1404e9376 / panic 串族 0x1429ebc17；定 min_extent/32x32/TypeId 写入 |
| P2 | **11 个 NF 函数 rne-read 手解** | 全部只有 I/U，缺原字节 | 按 §2 表逐地址 rne-read/rne-disasm，优先 `PassPgoNameId::from_name`（hash）、`pass_access_range_indices`（用途）、`BatchCompiler::new`（编译链）、`ResourceHandle::raw`/`end_render_pass`/`set_line_width`（短函数） |
| P3 | **屏障计算核心 + SynchronizationScope 0x50B** | 分块/屏障是 rg 域核心算法，当前 3/4 为 I | 补反编译 `FUN_1402bf880`（compute_batches）+ 解 0x140bd6650 的 SynchronizationScope 构造；在 record_batch 0x1402c1d40 前段抠 add_pre_pass_barriers/gather_batch_preamble_barriers 内联体 |
| P4 | **资源池回收闭环** | 生命周期链"谁 insert 回池"断点 | xref 0x140be6d70/0x140be7320（insert_texture/insert_buffer）调用方；确认退役→回池路径与 raw/managed 交界 |
| P5 | **compile ABI + RenderGraphState / ResourceExportHandle 类型** | U卡点#1/#2，compile 三实例参数面 | 从 0x140161cd0.c（148KB 已反编译）前段参数寄存器与状态字段 +0x08..0x90 反推；xref 0x140161cd0 调用点定 ResourceExportHandle 消费 |
| P6 | **时态状态机收口** | Record 0x68B 字段 + recorder 布局 U | export_temporal 反编译已给 `-0x58/-0x50/-0x48/-0x40` 字段证据，逐字段定名；xref 0x1404f6e50 二次请求路径 |
| P7 | **import_export impl 归属 + into_sampled_rg_texture** | 归属带问号 + 唯一 compiled 未覆盖业务函数 | xref 0x140bd60a0/0x140bd6380/0x140bd63f0/0x140bd4fc0 调用方的句柄类型；定位 into_sampled_rg_texture 地址 |
| P8 | **rebuild 行为差异登记** | temporal 复用 import_unowned vs rebuild import_owned；FnOnce vs FnMut | 补进下轮 ALGORITHMS/对照文档，不涉本轮 |
| P9 | **低优先**：7 个 fmt/default/from 派生 + 派生 closure 族 | 类型合同即可覆盖 | 按 derive/serde 合同整批归并，不逐函数深挖 |

**完成度评估**：ALGORITHMS.md 对"有地址、能反编译"的函数覆盖度很高（51/51 真实反编译全部落实 O，57/57 地址全进文档，0 个"反编译了仍 I/U"），说明 rhapsody-rg 主体工作扎实。缺口集中在三类：① 11 个 NF（全 I/U）；② 屏障/分块/池回收/OOM 四条算法链只到"调用点 O、决策逻辑 I"；③ rebuild 对照有 1 处结论性错误（image_access_mask）。按 P0→P7 补即可把"完成度不高"的观感拉齐到业务逻辑层。
