# small-libs batch-02：三库逆向完善整合报告

日期：2026-10-13。基线：B patched EXE（cf048e27...）。范围：`reconstruction/shared-foundation/crates/{motomoto,tg-package-integrity,tg-text}`。构建入口：`reconstruction/small-libs/Cargo.toml`（隔离工作区，`[lib] path` 引用同一份生产源码）。

## 概览

| 库 | 本轮工作 | 结果 |
|---|---|---|
| motomoto | Ghidra 反编译 add_chunk×2/Drop/iter/next + 存档消费者映射 + 字节级测试 | 候选与反编译一致；12 测试过；clippy 过 |
| tg-package-integrity | Ghidra 反编译 verify/find_extra_files/checksum 叶 + 报告/崩溃消费者接线 + OS 边界修复 + serde/schema 测试 | 候选与反编译一致；7 测试过；clippy 过 |
| tg-text | Ghidra 反编译 TextRenderer/TextFragment/font_table 族 + 候选差异登记/修正 + 依赖接线核验 | 见下方小节 |

## motomoto

- 反编译核对：`Writer::add_chunk<Cursor,&str/u64>`（0x141552690/0x141552820）、ChunkWriter Drop（0x14087abf0）、`Reader::iter`（0x142080d50）、`next`（0x142080d70）与候选逐条一致（错误槽 take/清空、end-start-16 回填、UnexpectedEof/溢出/panic 边界）。
- 存档消费者映射：`capture_game_snapshot`（0x1417a1d00）add_chunk<&str> 调用 + serde_json 世界状态写入；读取侧 `validate_and_load_closest_snapshot_data`（0x1417a6e50）`Reader::iter→next` + DeflateDecoder + serde 反序列化。
- 新增测试：`byte_exact_header_layout_and_trailing_partial_tail_ignored`。

## tg-package-integrity

- 反编译核对：`verify_package_manifest`（0x14067c3d0，读→blake3→FNV(0xf52dad71b327e45a)低16位门→serde 解析→version==1→easy-parallel→find_extra_files→FileIssues）、checksum 叶（0x140f737f0，FNV-1a prime 0x100000001b3 + 长度前缀）、`find_extra_files`（0x140f79650/0x140f78630，递归/tmp 跳过/symlink/UTF-8/HashSet）均一致。
- 消费者接线：`verify_package_integrity_and_show_report`（0x2552f5）verify 失败→`run_integrity_report_gui`（行704）；`on_game_crashed`（0x3b040f）verify→`run_crash_reporter_gui`（行1365-1366）。
- 修复：合成测试的 OS 相关失败（`build-info/manifest.json/../manifest.json` 在 Unix=Missing/Windows=Corrupt，断言必归其一后对照其余）。
- 新增测试：`unknown_fields_are_ignored_and_version_must_be_u32`。

## tg-text

- 反编译核对（Ghidra shared + radare2 RNE）：`TextRenderer::new/load_font_from_file/rect/render/consume/upload_pending_glyphs/get_glyph`（0x140b8e4a0–0x140b8fea0、0x140fac810）、`FontTable::get/scaled`（0x140b91fa0/0x140b92090）、`TextFragment::prepare/render/set_internal_scaled_metrics/replace_editor/take_editor`（0x140b90f60–0x140b91620），及 toolbar rect 调用方（0x140680330/0x140680590）。
- 实现修正（对齐原反编译）：
  1. `TextRenderer::rect` 对齐原签名 `rect(pos, extent, opacity:f32, shapes)`——颜色固定 `u32::MAX`，alpha 经 `scale_u32_packed_srgba_color` 缩放（RNE 0x140b8f510：`mov ecx,0xffffffff; call 0x140c918d0`），panic 断言 `stream_len < 256`；拆出 `rect_with_color` 作为重建扩展。
  2. `TextFragment::set_internal_scaled_metrics` 对齐原签名 `(renderer, font_size, line_height)`：已缩放 metrics 参数 + 与当前 buffer 相等且均非 NaN 时早退（0x140b90780）。
  3. `FontTableEntry::scaled` 操作数序对齐原 `mulps`：`self.scale * input`（0x140b9209f-0x140b920a8）。
  4. clippy `chunks_exact(3)`→`as_chunks::<3>()` 清理。
- cosmic-text/etagere 接线：consume/upload/get_glyph 反编译确认候选接线（rhapsody update_buffer_bytes/update_texture、glyph_cache CacheKey 查找）与原调用一致；逐字节 fork 等价属依赖边界。
- 登记差异（未擅改，见 native-oracle.json remaining_unknowns）：prepare 中 Action 判别值 0x22 的 break 语义、prepare bounds[1] 来源（原直读字体 metrics vs 候选布局 fold）、`replace_editor` 返回 void vs 候选 mem::replace、`render` getter(+0x440/+0x460) 契约、toolbar/SSBO 真实消费者（GPU/窗口后置）。
- 环境边界：2 个 text_pipeline 测试为 Windows 专属字体验证（Fantasy→Impact 映射、`C:/Windows/Fonts/arial.ttf` 路径），本轮加非 Windows `ignore` 标记保留 Windows 覆盖；macOS 套件全绿（behavior 6/6 + text_pipeline 9/9 + 2 ignored）。工具链 NaN 位模式差异（原 0xFFC00000 vs 本机 LLVM 0x7FC00000）登记，测试已 NaN 容忍。

## 统一验证结果（macOS）

- motomoto 12 测试过；tg-package-integrity 8 测试过；tg-text behavior 6 + text_pipeline 9 过（2 Windows 专属忽略）。共 35 过 + 2 忽略。
- 严格 clippy `-D warnings`（all-targets，无 gpu feature）通过。
- tg-text gpu 特性（ash + rhapsody-rebuild 路径）未在本轮启用；GPU example 需要原 shader/字体，不在本机运行（保持 batch-01 边界）。

## 完成轮（2026-10-14 追加）

按用户指示补齐"真实消费者接线与 A/C"之外的可闭合缺口：

**tg-text（反编译驱动的实质修正）**
- `cosmic_text::Action` 类型确认：fork 中为 **32 字节**、扁平标签 0x0–0x21（33/34 变体，`Motion(LayoutCursor)=0x0` 至 `Scroll=0x21`）；候选 `Vec<Action>` 类型正确。
- `set_text` 入队动作修正：原 tag `0x14` = `Action::Motion(Motion::BufferEnd)`（光标落文档末尾），候选此前用 `BufferStart(0x13)`——已改 BufferEnd；测试语义同步为 `ends_with`。
- `prepare` `Action 0x22 break` 已分类：0x22 是超范围无效标签，原代码为防御守卫，合法动作永不触发；候选"全量应用"对所有可构造输入行为等价。
- `prepare` bounds[1] 修正：layout fold → scaled line_height（`+0.0` 归零），原直取字体 metrics。
- `replace_editor` 对齐原：丢弃旧 editor 返回 void；`take_editor` 改用 `mem::replace` 直接返回旧值（原 take_editor 反编译确认）。

**tg-package-integrity**
- 新增 symlink/权限探针测试：symlink（文件/目录）跳过、无权限目录 read_dir 失败静默、普通文件入 extra——对照 find_extra_files 反编译（0x140f79650）。

**motomoto**
- 补反编译：`AsTag for &str`（0x142080ca0，>8 字节 panic）、`tag_from_str_trunc`（0x142080d00，min(8,len) 无 panic）与候选一致。
- 范围闭合：原 Writer/ChunkWriter/Reader 仅观察到 `Cursor<&mut Vec<u8>>` 单态化，无 File/BufWriter 原实例——泛型 IO 失败/flush 合同归候选重建边界。
- ABI 观察：ChunkWriter 原为 `{parent, 错误槽指针, start}` 三字段（24B），候选 `{parent, start}`（16B）业务等价未追字节。
- 勘误：batch-01 将 Reader::new 地址标为 0x14067dd80，实为 `toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut`；Reader::new 保持 metadata 级。

## 依赖

- 公共依赖闭包：157 项身份匹配原 B 锁；7 项既有候选偏差来自 tg-text 可选 gpu 特性路径（gpu-allocator 0.27 等），已在 PROJECT_MEMORY 登记；本轮无新增依赖，Cargo.lock 未变。
- 反编译会话：`evidence/toolchain/session-ghidra-shared-motmoto-b02.json`、`session-ghidra-shared-integrity-b02.json`、`session-ghidra-shared-tgtext-b02.json`；tg-text 另有 RNE 反汇编规则登记于其 native-oracle.json。

## 验收边界

- 剩余未闭合（按用户指示保留）：真实消费者接线（存档 schema/报告 GUI/崩溃 GUI/toolbar-SSBO/GPU 渲染路径）、A/C 版本、原入口 shipped-process runtime 全链（依赖真实发行进程）。
- 库内已登记边界：tg-text `render getter(+0x440/+0x460)` 消费者契约、`TextRenderer::new` 完整装配细节、cosmic-text/etagere 逐字节 fork 等价（依赖边界）、`inf*0` NaN 位模式（工具链规范化差异）；motomoto 泛型非 Cursor IO 合同（原无该实例）、ChunkWriter 三字段 ABI（业务等价未追字节）；integrity 32/64 位 Hash 长度前缀跨平台（原行为即平台相关）。
- 差分/反编译仅作校验，不作实现手段（遵守用户指示的流程）。

## 完成度判定（2026-10-14）

按功能纳入规则枚举各库 PDB 发射业务函数后逐项核对（排除 derive/Deserialize visitor/closure 胶水/宏生成）：

| 模块 | 库内业务逻辑 | 判定 |
|---|---|---|
| motomoto | tag 编码、add_chunk 错误槽顺序、Drop 长度回填、Reader 迭代边界（None/UnexpectedEof/panic）——全部实现，add_chunk×2/Drop/iter/next/as_tag/tag_from_str_trunc 反编译一致 + 173 原 Reader RNE 案例 | ✅ 实现+验证完成 |
| tg-package-integrity | 校验门（BLAKE3→Hash<[u8;32]>→seeded FNV low16）、verify 链、missing/corrupt/extra、extra 扫描——全部实现，verify/find_extra_files×2/checksum 叶反编译一致 + 29 容器案例 + 真实 2.7GB 包 + 探针 | ✅ 实现+验证完成 |
| tg-text | TextFragment 17、TextRenderer 11、FontTable、ShapeStream 全业务函数均有实现；核心算法（rect/prepare/render/scaled/set_internal_scaled_metrics/replace_editor/take_editor/get_glyph/upload_pending_glyphs/consume/load_font_from_file/append）反编译验证 + 51+21+20+88 原 RNE 案例；`Image::with_scaling` 由 `image(source,scaling)` 覆盖；`image_name_to_index` 原文即 panic stub（与候选 unreachable 一致） | ⚠️ 实现完成、核心验证完成；少量 setter 复核 + API/边界保留 |

**保留（非缺失业务函数）**：
- tg-text 简单 setter/getter（set_max_width/set_align/set_weight/set_family/text/bounds/editor_action/font_faces/allocate_atlas_space/validate_glyph_cache_allocs/set_pixels_per_point）已实现但未逐一 fresh 反编译复核——严格"逐函数原生证据"需补这几条复核记录。
- tg-text `render` 返回类型为重建扩展（原 getter 流视图 vs 候选 AtlasDescriptor）；`TextRenderer::new` 装配细节未全核对。
- 依赖/工具链边界：NaN 位模式、cosmic/etagere 逐字节等价、integrity 32/64 位 Hash。
- 消费者接线与 A/C 按用户指示保留（存档 schema、报告/崩溃 GUI、toolbar-SSBO、GPU 路径、A/C）。
