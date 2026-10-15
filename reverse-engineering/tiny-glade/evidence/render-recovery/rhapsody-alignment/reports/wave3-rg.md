# rg 域第三批（闭合原版未读项）

图例：✅原版对齐[证据地址] / ⚠️候选 / ❌读不出及原因。工具：`/tmp/dis.py`（objdump 反汇编 `tiny-glade.rne`）、PDB 公共符号表
（`render-architecture/public-symbols.json`）、`country-core-audit/data/procs.tsv`；新增分析脚本在 `/tmp/w3_*.py`（只读）。

## 工作包 1：`validate_read_access` —— ✅ 原版就是空操作

* PDB 公共符号**有**这两个函数（之前误判为"无符号"）：`<Texture as Resource>::validate_read_access`（`…h83678d6fcc0889d6`）与
  `<Buffer as Resource>::validate_read_access`（`…hfc3e91621b7af5dd`），在 `public-symbols.json` 里 section 1 offset 240512，
  VA = **0x14003bb80**，两者 ICF 折叠到同一地址。
* 0x14003bb80 的机器码只有 `c3`（ret）+ int3 填充（`dis.py 0x14003bb80`）。即函数体为空，没有断言、没有 panic 文案。
  所以"在内联展开里找检查"的前提不成立：根本没有检查，调用点保留 `call 0x14003bb80`（例如 `get_or_create_temporal<&str>` 0x14096c674 处
  的一次调用，被 ICF 后与其他空函数共址），不是被优化掉的断言。
* `rg/resource.rs::validate_read_access` 保持空操作并把证据写进文档注释；`pass_api.rs`/`pass_bindings.rs` 里既有的读访问校验
  （种类/代际检查）是游戏之外的重建加固，不是它的对应物，未改。测试：`tests/rg_wave3.rs::validate_read_access_is_a_noop_like_the_original_ret`。
* ⚠️ 附注：同一地址被许多空函数共用，不能把"调用 0x14003bb80"一律读成 validate_read_access。

## 工作包 2：`PassPgoStats::extend` / `reset` —— ✅ 找到并纠正

* `extend` 的内联体在 `record_batch`（0x1402c1d40）里（串行与并行两条路径各一份，Ghidra 行 298-305 / 608-614）：对块返回的每个 16 字节
  `PassPgoDuration {name:u64, d:u64}` 依次 ① `FUN_1402ca150(current, name, d)` ② `current.total@0x30 += d` ③ `current.max@0x38 = max(max, d)`。
  参数：`param_7 = executor+0x88`（current，被写）、`param_8 = executor+0x48`（previous，只读，`duration_ns`/总和/最大值取自它）——`execute_compiled` 0x1400fa190 行 459-460。
* `FUN_1402ca150`（0x1402ca150）是 `hashbrown::HashMap::insert`：命中桶时 `0x1402ca255: mov rdx,[r15-8]`（取旧值）… `0x1402ca2a1: mov [r15],rsi`（**写新值，覆盖**），
  未命中走插入。因此**同名后者覆盖前者，不是累加**；而 `total` 是普通 `add`（不饱和）、`max` 是 `max`。
* 之前实现把耗时表按名累加（`*entry += ns`）并用 `saturating_add`，与机器码不符。已改：
  `record` = insert 覆盖 + `wrapping_add` + `max`；`extend` 改为接收 `IntoIterator<Item = PassPgoDuration>`（原版没有"并两个 Stats"的操作，旧的 `extend(&Stats)` 是候选），执行器两处合并点改为 `pgo.current.extend(durations)`。
* `reset`：`end_frame`（0x140bec400）里整块 `+0x48 ↔ +0x88`（含种子，4 对 qword，0x40 字节）后清空新 `+0x88`：len（`+0xa0`）非 0 时控制字节 `memset 0xff`（长度 mask+1+0x11）、`len=0`、重算 `growth_left`
  （`(mask+1)&!7 - (mask+1)>>3`，mask<8 时 = mask）；**无论 len 是否为 0 都把 `+0xb8/+0xc0`（即 total/max）清零**。与现 `reset` 一致。
* 预算 clamp：`record_batch` 行 507-517 `lo=total>>4, hi=total>>1`，`budget = hi; if max<hi {budget=max}; if max<lo {budget=lo}`，与 `max.clamp(lo,hi)` 一致（未改）。
* 测试：`tests/parallel_chunks.rs::stats_extend_overwrites_per_name_but_accumulates_total_and_max`（新）、`tests/rg_frame_round2.rs`（改为覆盖语义）、`tests/rg_wave3.rs::pgo_current_is_cleared_but_seed_free_and_extend_matches_insert_semantics`（含回绕）。
* ⚠️ `counts`（`call_count`）是候选自加表，原 0x40 字节结构里没有，保留。

## 工作包 3：`ResourceExportHandle<Texture>` 与 `compile` 返回布局 —— ✅ 布局 / ⚠️ 语义

读三个实例（0x1401647b0、0x140161cd0、0x140166510；`param_1` = sret 输出指针，`param_2` = `&mut RenderGraphState`）的成功/失败尾部：

| 实例 | 成功路径 | 失败路径 |
|---|---|---|
| 展示图 0x140166510（Export=`()`） | `memcpy(out, graph, 0x218)`（行 580），无尾部写入 → `Ok` 主体 0x218 | `*out = 0x8000000000000000` |
| 加载图 0x1401647b0 / 世界图 0x140161cd0（Export=`ResourceExportHandle<Texture>`） | `memcpy(out, graph, 0x218); *(u32*)(out + 0x218) = h`（`puVar35[0x43]`） → 整体 0x220 | `*out = 0x8000000000000000` |

* ✅ `ResourceExportHandle<Texture>` 是 **4 字节 u32**，紧跟在 `CompiledRenderGraph`（0x218）之后；旧文档"=0x218B / 是 CompiledRenderGraph 包装"不对。
* ⚠️ 语义：`h` 取自导出前那份纹理句柄记录的首个 u32（0x1401647b0 行 760 `uVar23 = *(u32*)slot`，随后才 `FUN_140bd6380(slot, …, 0x22)`=`Texture::export`），
  即该纹理在图里的逻辑资源下标；Rust 侧的类型名/字段名读不出。
* ✅ `Err` = 小值优化：只写首 qword = i64::MIN（`CompiledRenderGraph` 首字段有此 niche）。失败条件 = `ResourceRegistry::populate_resources`（0x140be7ba0）返回非 0（行 988：`cVar22 != 0`）；
  失败分支做 `RenderGraph::on_retire`（0x140be9420）→ 把图拷回 `state`（0x90 字节）→ 写 niche → drop registry。**Err 路径不写任何载荷**，CompileError 没有可读的字段——这正是 OOM 降分辨率只靠"返回 Err"触发的原因。
  其余出错点（`FUN_1428d9310` 等）都是 panic，不是 Err。
* `CompiledRenderGraph::new`（0x1402c4280）逐段拷贝验证了 0x218 的段划分（`+0x00` 0x90、`+0x90` 0x18、`+0xa8` 0x108、`+0x1b0` 0x50、`+0x200` 0x18）。
* 落码（`rg/compiled.rs`）：`RawVecLayout`、`CompiledRenderGraphLayout`（`repr(C)`，0x218）、`ResourceExportHandle<T>`（`repr(transparent)` u32）、`TextureKind`、
  `CompileOkLayout<Export>`（`{graph, export}`；`()` 时 0x218，句柄时 0x220）、`COMPILE_ERR_NICHE`/`compile_result_is_err`。3 个偏移/大小测试。
  各段内部字段仍只有字节块级证据，段体用 `[u8; N]`；现 `graph::CompiledGraph` 仍是语义候选，不与之一一对应。

## 工作包 4：59 个 pass 调用点的方法清单 —— ✅（结构性结论）

见 `reports/rg-pass-methods.md`（新）。核心发现：**原版没有"每个 pass 一个回调体"**。所有 pass 都经 `PassBuilder::render<PassResources, closure>` 装箱，闭包类型只有 6 种共享回调体
（D 计算分发 0x1415fc050→0x14169af70、R 光栅绘制 0x140b649e0→0x141697ab0、F 清缓冲 0x140904990、C 清颜色图像 0x140904a30、P 缓冲拷贝 0x140904ac0、Z 重置间接参数 0x140a880a0）；
各 pass 的差别在闭包环境。逐体反汇编得到每种回调体的 RenderPassApi 方法 + 设备函数槽，再按各 pass 构图函数（深度≤5）里的装箱点推出 59 行。
* 设备槽偏移 = `0x78 + 8×ash::DeviceFnV1_*` 字段序号（`tests/rg_wave3.rs::original_device_slots_equal_ash_table_offsets` 用 `offset_of!` 证明全部相等）；`+0x4b8/+0x4c0` = `DeviceFnV1_2` 前两项 `cmd_draw_indirect_count`/`cmd_draw_indexed_indirect_count`；`+0x898/+0x8a0` = `begin_label/end_label`。
* `RenderPassApi` 在原版只有 11 个命令类符号函数（begin/end_render_pass、set_default_view_and_scissor、set_stencil_ref、set_line_width、bind_vertex_buffer_by_binding_point、bind_index_buffer、push_constants、copy_buffer、fill_buffer、clear_color_image）；
  draw*/dispatch* **没有独立符号**，内联在 D/R 回调体里。`rg/pass_api.rs` 已全部具备，**没有缺方法**；`draw_call_count`/`begin_attachments` 是候选扩展，保留。
* 新增 `pass_api::original_slots`（24 个槽常量 + `slot(&Command)`，无通配 match，增删 `Command` 编译期强制同步）。
* ❌/⚠️：静态直接调用图，不含尾调用与动态分派里再装箱的闭包；R 的具体 `vkCmdDraw*` 种类由运行时绘制记录决定，未逐 pass 核对；加载图/展示图闭包内入口不在 59 行内。

## 工作包 5：resource key —— ⚠️ 判断，不改

`rg/` 里剩余的 `BTreeMap<String,…>`：
* `pass_bindings::BoundPassBindings.bindings`、`PassBuilder` 的 `bindings`、`binding.rs` 的 `named_slots`：键是**着色器反射的描述符名**。原版在 pass 构建时把名字解析成
  `shader_pipeline::reflection::BindingIndex`（`bind_descriptor_set<…IntoIter<(BindingIndex, PassResourceSlot)>…>` 0x1416a6010 / 0x1416a5380 的泛型实参即证据），名字表 `(BindingIndex, String)` 只用于解析与报错。
  现重建把解析推迟到 `descriptor_writes`（按 `ReflectedEntry.descriptors[].name` 查），属于合理的重建边界；改成整数键要同时改 `binding.rs`（不归本批）与 `pass_builder.rs` 的公开 API，且会让声明端失去可读名，故不改。
* `vulkan.rs` 的 `ResourceResolver`/`specifications`（按名字登记）：图内资源在原版是逻辑 u32 句柄，名字属于主机侧便利层；该文件不归本批。
* "u64 TypeId 查表"（`Components::get_resource_id`）是 ECS 世界侧（bevy），不在 rg；时态键已是 `Ustr`（与原 `hash_one<RandomState, Ustr>` 0x1412e8120 一致）。
* 结论：无需改动；没有发现 `rg/` 内应按整数/TypeId 取代的字符串键。

## 工作包 6：换角度重读

* **`TemporalRenderGraphRecorder +0xc8..`** —— ✅ 读出（`get_or_create_temporal` 0x14096c310 / 0x1404f6850 的字段使用 + `get_and_flip_parity` 0x140beced0 反推）：
  Recorder 总长 0x110：`+0x00` RenderGraph(0xa8)、**`+0xa8` = `&GraphicsDevice`**（`[rdi+0xa8]` 作 `GraphicsDevice::create_buffer` 第 2 参，`export_temporal` 不拷它）、
  `+0xb0..0x110` 内嵌 `TemporalRenderGraphState`（0x60）：表 1 temporal 资源（ctrl `+0xb0`、mask `+0xb8`、growth `+0xc0`、items `+0xc8`、种子 `+0xd0/+0xd8`）与**表 2 奇偶翻转表**（ctrl `+0xe0`…items `+0xf8`、种子 `+0x100/+0x108`；`get_and_flip_parity(recorder+0xb0, key)` 体内读 `state+0x48/+0x50`）。
  旧文档"+0xd8..+0x108 含义未读出"即第二张表。`export_temporal` 输出是去掉设备引用的 `(RenderGraph, State)` 元组，所以输出里表整体前移 8 字节（旧文档的 `+0xa8` 控制字节是输出基准）。落成 `temporal::original_layout`（含测试）。
* **Record 0x68B** —— ✅ 部分：`+0x00` 键 Ustr、`+0x08` 状态 u32（0 纹理 / 1 缓冲 / 2 已导出；`get_or_create_temporal` 命中要求 `==2` 否则报错并改写为 1）、`+0x0c` 起负载（字节 `+0x0c` 为 `import_unowned` 的初始访问参数）、`+0x30` 导出载体（`import_unowned` 第 2 参）、
  **`+0x66` 必须 == 7（缓冲变体的类型标记）**、命中后写回 20 字节 `ImportedBuffer` 到 `+0x0c..0x20`；返回 `Result` 0x20 字节 `{ImportedBuffer(0x14), tag u8 @0x18}`：0 新建 / 1 复用 / 2 错误（String 载荷）。❌ `+0x24..0x30`、`+0x67` 字段名仍未读出。
* **`ResourceSyncGroup::can_be_concurrent_with`** —— ✅ 无组级独立版本：机器码里只有 0x140bece70（`PassResourceAccessType::can_be_concurrent_with`）与 0x140bece60（`is_equivalent`）；`is_write`=0x140c10e10 `add cl,0xd8; cmp cl,0x10; setb`（访问码 0x28..0x37 为写）；
  逻辑：都不是写 → `access` 相等；否则 `access` 相等 && `group` 相等 && `group != 0`。现 `resource_rules::can_be_concurrent_with` 与之逐条一致，无需改。
* **OOM 降分辨率全链** —— ✅（rhapsody 侧）：compile 失败的唯一来源是 `populate_resources`（0x140be7ba0）返回非 0；Err 不带载荷（见工作包 3）；`on_retire`+图回退到 `state` 后由 country-core 的 `recompile_at_reduced_scale`（已实现，见上轮）处理。加载图路径无重试（上轮已证）。

## 改动文件（均在归属范围内）

* `crates/rhapsody/src/rg/executor.rs`（PassPgoStats record/extend、两处合并点）
* `crates/rhapsody/src/rg/compiled.rs`（compile 返回布局类型 + 测试）
* `crates/rhapsody/src/rg/pass_api.rs`（`original_slots`）
* `crates/rhapsody/src/rg/resource.rs`（`validate_read_access` 文档证据）
* `crates/rhapsody/src/rg/temporal.rs`（`original_layout` 字段级 + 测试）
* `crates/rhapsody/tests/parallel_chunks.rs`、`tests/rg_frame_round2.rs`（PGO 语义）
* 新增 `crates/rhapsody/tests/rg_wave3.rs`（4 个）；`reports/rg-pass-methods.md`、本报告。

## 新增/修改测试

`rg_wave3.rs` 4 个；`compiled::compile_layout_tests` 3 个；`temporal` 布局测试扩充；`parallel_chunks` 1 个改写；`rg_frame_round2` 1 个改写。

## 遗留

* `ResourceExportHandle<Texture>` 的 Rust 类型名/字段名；Record `+0x24..0x30`/`+0x67`；59 个 pass 里 R 回调体的具体 draw 种类（运行时数据）；加载图/展示图闭包入口清单。
* `PassPgoStats.counts` 候选表。
* 未接线：`CompileOkLayout` 等 `repr(C)` 类型只是布局描述，不参与运行（现 `graph::CompiledGraph` 是语义候选）。
