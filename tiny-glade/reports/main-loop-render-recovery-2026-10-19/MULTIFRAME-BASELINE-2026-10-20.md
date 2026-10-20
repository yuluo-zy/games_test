# 多帧渲染稳态验证与性能基线（2026-10-20）

测试：`cargo test -p tiny-glade --features native-audio --test original_gameplay_chain shipped_session`
（新增 `shipped_session_steady_state_multi_frame`）。环境：lavapipe（软件光栅化），debug 构建（无调试符号），
Khronos 校验层开启，480x270，shipped session（`TG_MULTIFRAME_OUT` 指向 `multiframe/` 保存 PNG 与明细）。

## 多帧入口

- `production_renderer::render_headless_gameplay_frame(bridge, world)`：不经窗口/交换链，复用生产路径同一序列
  （`build_draw_lists` -> `refresh_draw_vaos` -> `build_frame(with_main_prelude(build_full_world_stage))` ->
  prewarm -> `compile_built` -> `execute_compiled` -> 截图 `submitted` -> `take_exported_texture`）。
  与窗口路径差别：没有 egui 后端、没有 resolve/present 图；帧间节流按 `frames_in_flight` 等最旧在途帧（等价 `acquire_swapchain_image`）。
- 测试流程：加载 shipped session -> `warm_up_gameplay`（工作线程首帧）-> `accept_gameplay_warmup` -> 30 帧循环：
  每帧 `App::update`、MenuBackground 截图请求（第 2/3/4/5/10/20/30 帧）、渲染、`finish_screenshot_capture`、
  `RenderTemporalState::frame_render_returned`（与 `GameFrameDriver` 一致）。
- `world_graph::WorldHistoryObservation`：`build_world` 记录本帧 TAA/TGSR 颜色历史对的 `history_valid`（只读观测）。
- rhapsody 只加只读统计：`PassPgoStats::iter()`（每 pass CPU 录制耗时，取自 executor 回调计时），
  `bindless` 耗尽错误带描述符类别。

## 断言结果（全部通过，30 帧）

| 项 | 结果 |
|---|---|
| 第 2 帧起 `frame_counter` 递增且 != 0 | 通过（帧 i 的计数 = i） |
| 第 2 帧起 TAA 颜色历史有效 | 通过（29/29 帧 `history_valid`） |
| 计划缓存命中 | **0/28 命中**，见下文“未闭合” |
| 计划缓存未命中原因 | 仅 Cold(1)、GraphStructure、AccessSemantics；Descriptor/Sharing/QueuePolicy/Shader*/DeviceGeneration 均为 0 |
| 校验层 VUID | 0（日志无 VUID/Validation Error） |
| bindless 描述符不泄漏 | shader 堆 live 稳定 5333~5341（两帧在途），峰值不增长；纹理堆 live 恒定 |
| 动态常量环 | pages 稳定 3，page_reuses 递增（帧间回收） |
| 设备内存稳态不增长 | **不通过（已知缺陷，未闭合）**：+46 MB/23 帧 |
| 帧间稳定 | 相机不动。帧 20->30：平均亮度差 1.74、平均绝对差 2.54、|d|>16 像素占 0.45%；前 5 帧因曝光适应/TAA 收敛波动 2~7 |

亮度序列（第 2/3/4/5/10/20/30 帧均值）：114.7、116.9、113.6、115.2、111.7、104.9、103.2，之后收敛（20->30 差 1.7）。
下降趋势在前 20 帧，后段稳定，与曝光/历史收敛一致；是否即眼适应曲线尚未用原版对照确认。

## 性能基线（帧 2~29，中位数 / P95，毫秒）

| 指标 | 中位数 | P95 | 范围 |
|---|---|---|---|
| App update | 24.6 | 30.0 | 22.6-30.7 |
| 声明（build_frame） | 227 | 274 | 177-296 |
| 编译（prewarm+compile_built；ExecutionStats.compile_ns） | 349 / 345 | 379 / 375 | 339-386 |
| 录制（ExecutionStats.record_ns） | 195 | 243 | 167-247 |
| 提交（submit_ns） | 7569 | 10965 | 5198-11181 |
| CPU 合计（update+声明+compile_ns+record_ns，不含提交） | 806 | 853 | |
| 整帧（含提交） | 8501 | 11938 | |

**submit_ns 在 lavapipe 上无参考意义**：软件光栅化在提交调用内同步执行 GPU 命令，5~11 s 全是 CPU 光栅化。
指标看 CPU 声明/编译/录制：稳态约 0.8 s/帧（首帧 record 387 ms、声明 262 ms、compile 379 ms；稳态录制反而更低，因为首帧含冷启动）。
单帧 `commands_recorded`=1，barrier 160 条/3270 次转换，batches 8~9（并行 batch 7~8，chunks 1）。

每 pass CPU 录制耗时（最后一帧，完整表见 `multiframe/pass-record-last-frame.txt`；同名 pass 取最后一次，括号内为调用次数）：

| pass | ms |
|---|---|
| Shadows（x3） | 92.7 |
| G-buffer | 87.8 |
| G-buffer OC/2 | 45.4 |
| Apply deferred lighting | 8.7 |
| Forward | 6.2 |
| G-buffer emissive OC/2 | 6.0 |
| Grass object push pass | 5.3 |
| G-buffer velocity | 4.7 |
| Nani fill unsorted instance indirection（x2） | 4.6 |
| Pre-TAA transparency | 3.7 |

与首帧结论一致：Shadows、G-buffer 是录制大头（raster 每 draw 构建参数块）。Nani 各 pass 稳态 3~5 ms，
首帧 32 ms 的 fill indirection 是冷启动（管线/缓存预热）而不是稳态成本。

## 发现并修复的缺陷

1. **第 3 帧起崩溃：动态常量页的 native 范围表耗尽**（`等待超时`，`range_state.rs` `max_cells`=4096）。
   根因：页内每次主机写入都使 revision 递增，相邻单元不会合并；页被 bump 分配追加或回收复用时从不重置，
   单页一帧约 700+ 次写入，数帧后 cells 超限。修复（rhapsody `raw/dynamic_constants.rs`）：
   回收页时用 `discard_idle_range_state` 重置范围表，页内分配数封顶 1000（每写入至多 2 单元，留一半余量）。
2. **第 3~22 帧起 bindless 描述符表耗尽**（`bindless descriptor table is full`，先是 StorageBuffer 后是 SampledImage）。
   根因：宿主按单帧实测需求(存储 4096/采样 2048)定容，但租约保留到该帧 GPU 完成，`frames_in_flight`=2 时两帧共约 5340 个描述符。
   修复（`render_host.rs`）：存储缓冲需求 16384、采样图像 8192（仍按设备上限收敛）。
3. **warm-up 在负载下超时**：首帧 pipeline prewarm 上限 10 s 与 GPU 完成等待 10 s 过短（生产路径已是 180 s）。
   改为同一常量 180 s / 120 s（上限而非目标）。

## 未闭合

- **计划缓存永不命中**（0/28）。根因已定位：`GraphStructure`。`PlanKey` 含每个 pass 访问的 buffer 范围；
  帧常量、uniform 上传、BDA 上传、cpu-upload 等 pass 声明的是动态常量页内 bump 分配的 `Buffer{offset,size}`，
  偏移每帧不同（例如 “Visual terrain height” 访问 resource 60 的 offset 35200/7712/42912…），整张图结构因此每帧都不同；
  pass 数、资源数、导出数、描述符、访问语义每帧一致。`compile_ns`≈345 ms/帧本可省去。
  尝试把这些访问改为整页 `AccessRange::Whole`：被 rhapsody `command_buffer.rs` 的
  “private upload read overlaps a declared or opaque native access” 拒绝（同页还有未声明的私有上传读），已回退。
  可行方向：(a) rhapsody 计划键对“全图只读的导入 buffer”规范化范围（需同步 `sync_plan::RangeProgram` 在实例化阶段取实际范围，改动大）；
  (b) 帧常量/上传 chunk 改用每帧固定偏移的专用页，使范围跨帧稳定。均未实施，需要 rhapsody 层设计决定。
- **设备内存逐帧增长**：分配记录对比（`multiframe/allocations-frame05.txt` vs `allocations-frame29.txt`）显示每帧多一张
  2088960 B 的图像（计数 7 -> 31，与 4x MSAA 480x270 RGBA8 目标大小一致；1080p 下约 30 MB/帧），`wait_submits_done`、
  `clear_idle_resource_pool`、`wait_idle` 后不释放，即有 Arc 持有者未放。已排除：池停放（parked=0）、alias set（4 个且不涨）、
  截图捕获（仅活动请求）、命令 owner（完成后释放）。持有者未找到；`TG_MULTIFRAME_STRICT_MEMORY=1` 时测试会对该增长断言失败。
  其余分配（15155200 B 的 7 -> 9、1044480 B 的 10 -> 13 等）也有小幅增长，疑似同源。
- 亮度前 20 帧下降 10%（114.7 -> 103.2）的来源（曝光适应 vs 其他）未用原版对照确认。

## 2026-10-20 修复结果（缺陷 1、缺陷 2）

- **设备内存逐帧增长（已闭合）**：持有者是 rhapsody `AttachmentCache`（`rg/attachments.rs`，raster_host 的 `RasterShared.attachments`，容量 256）。
  每帧导出并移交 owner 的 `msaa color out` 都是新图像，其渲染附件视图（`TextureView` 持有 `Arc<Texture>`）留在缓存里，
  直到缓存饱和才淘汰（诊断：按创建点统计，仅该图像逐帧存活，其余 alias/池资源稳定；`wait_idle` 不释放因缓存是强引用）。
  修复：新增视图时先释放“纹理只被缓存自己的视图持有”的条目（外部/在途命令仍持有则保留）。前后：+46219264 B/23 帧 -> 0（帧 12 -> 28 反而 -518400 B）。
- **计划缓存 0/28 -> 26/28 命中（已闭合）**：动态常量页此前在 GPU 已完成时继续在页内追加，chunk 偏移取决于 GPU 完成时机。
  现在页一旦被提交使用即封存，只在使用完成后从偏移 0 重置，帧内 chunk 偏移逐帧一致；`AccessRange::Buffer` 仍是计划键的一部分，
  键相同即声明相同，命中与未命中行为等价（`complete_plan_miss_reasons...`/`borrowed_comparison_matches...` 已覆盖范围偏移进入键）。
  剩余 2 次结构性未命中来自时序纹理对的奇偶翻转与截图 pass（plan 变体 4 个，第 6 帧起每帧命中）；compile_ns 约 345 ms -> 约 11 ms。
