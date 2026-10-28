# crates/tg-text/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B FontTable与ShapeStream/RawShapeStream CPU配置→lookup/scaled、builder→resolver→原u32 words子链；lib.rs仅模块glue。

## 接线与验证

接线：加入shared-foundation workspace，仅FontTable/shape CPU子链；没有renderer成功stub。

既有6tests/51原append+21原metrics；独立review未发现CPU代码/性能/造轮子缺陷。原PDB String::deserialize→RON deserialize_string链纠正旧同址标签误读；当前仅注释/README修正，旧证明hash保留。候选请求/接受集合/panic/分配复用探针完成；完整text/glyph/GPU与原serde未验。

batch-02（2026-10-13）：Ghidra shared + radare2 RNE 反编译 TextRenderer/TextFragment/font_table 族；修正 rect 签名（pos/extent/opacity/shapes，颜色固定 u32::MAX 经 scale_u32_packed_srgba_color 缩放）、set_internal_scaled_metrics 签名（renderer/font_size/line_height + NaN 守卫早退）、scaled 操作数序；cosmic-text/etagere 接线经 consume/upload/get_glyph 反编译确认；登记 prepare Action 0x22 break、bounds[1] 来源、replace_editor 返回 void、render getter 契约差异。behavior 6/6 + text_pipeline 9 过（2 Windows 专属字体测试加非 Windows ignore），clippy 过。证据：[batch-02 BUSINESS_RULES](../../evidence/small-libs-batch-02-2026-10-13/tg-text/BUSINESS_RULES.md)、[delivery](../../evidence/small-libs-batch-02-2026-10-13/tg-text/delivery.json)、[native-oracle](../../evidence/small-libs-batch-02-2026-10-13/tg-text/native-oracle.json)。

完成轮（2026-10-14）：set_text 入队动作改 BufferEnd（原 tag 0x14）、prepare bounds[1] 改 scaled line_height、replace_editor 改 void + take_editor 用 mem::replace、0x22 break 分类为超范围无效标签（防御守卫，行为等价）；cosmic::Action 确认 32 字节/扁平标签 0x0-0x21。35 测试过（含 integrity symlink/权限探针），clippy 过。
完成度判定（2026-10-14）：库内业务函数全部实现（TextFragment 17/TextRenderer 11/FontTable/ShapeStream），核心算法反编译验证；保留项——简单 setter 未逐一 fresh 反编译复核、render 返回类型为重建扩展、TextRenderer::new 装配细节、依赖/工具链边界、消费者接线与 A/C。
## 已恢复的函数和类型范围

- FontTable::get/from_ron、FontTableEntry::scaled/default_scale
- ShapeStream builder/is_empty/encode_no_images
- RawShapeStream::new/clear/append；encode和Image::with_scaling业务规则合并入append

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| tg_text::font_table::FontTable::get | [reconstruction/shared-foundation/crates/tg-text/src/font_table.rs](../../reconstruction/shared-foundation/crates/tg-text/src/font_table.rs)：`get` | B：原缺失None规则静态对照；std HashMap复用 | scoped_business_comparison：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |
| tg_text::font_table::FontTableEntry::scaled | [reconstruction/shared-foundation/crates/tg-text/src/font_table.rs](../../reconstruction/shared-foundation/crates/tg-text/src/font_table.rs)：`scaled` | B：21原RNE指标输入逐bits零差异；恢复接口非原ABI | scoped_business_comparison：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |
| tg_text::shape_stream::RawShapeStream::append | [reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs](../../reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs)：`append` | B：51原RNE words/resolver顺序对照；预分配、不执行allocator增长/GPU | scoped_business_comparison：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |
| tg_text::shape_stream::RawShapeStream::encode | [reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs](../../reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs)：`append` | B：原inline私有word规则折入append三分支；非words accessor/独立序列化 | scoped_business_comparison：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |
| tg_text::shape_stream::ShapeStream::encode_no_images | [reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs](../../reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs)：`encode_no_images` | B：indexed允许、named panic静态/候选核对；原panic未执行 | scoped_business_comparison：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |

## 明确缺口

- TextFragment/TextRenderer未恢复
- cosmic-text原fork/etagere字形与atlas策略、GPU upload/toolbar/rhapsody消费者
- 完整原serde运行/unknown-field/错误payload
- named image纹理resolver原生产者
- A/C行为验收

下一闭包：沿真实TextFragment consumers恢复attrs/metrics/prepare并复用原cosmic-text/etagere

## 证据

- [evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md)
- [evidence/small-libs-batch-01/tg-text/native-oracle.json](../../evidence/small-libs-batch-01/tg-text/native-oracle.json)
- [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)
- [evidence/small-libs-batch-01/integration/dependency-validation.json](../../evidence/small-libs-batch-01/integration/dependency-validation.json)
- [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json)
- [evidence/small-libs-batch-01/tg-text/interface-map.json](../../evidence/small-libs-batch-01/tg-text/interface-map.json)
- [evidence/small-libs-review-2026-10-13/tg-text/REVIEW.md](../../evidence/small-libs-review-2026-10-13/tg-text/REVIEW.md)
- [evidence/small-libs-review-2026-10-13/tg-text/findings.json](../../evidence/small-libs-review-2026-10-13/tg-text/findings.json)
- [evidence/small-libs-review-2026-10-13/integration/summary.json](../../evidence/small-libs-review-2026-10-13/integration/summary.json)
- [evidence/small-libs-review-2026-10-13/integration/ERRATA.md](../../evidence/small-libs-review-2026-10-13/integration/ERRATA.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/tg-text/src/font_table.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-text/src/font_table.rs](../../reconstruction/shared-foundation/crates/tg-text/src/font_table.rs)：original_pdb_rules_and_scoped_native_or_candidate_comparison | tg_text::font_table::FontTable::get（scoped_business_comparison；业务等价以限定证据为准）<br>tg_text::font_table::FontTableEntry::scaled（scoped_business_comparison；业务等价以限定证据为准） | candidate_present；字体表lookup/scaled配置子域，未恢复字形/renderer。：[evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>tg_text::font_table::FontTable::get：原缺失None规则静态对照；std HashMap复用：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>tg_text::font_table::FontTableEntry::scaled：21原RNE指标输入逐bits零差异；恢复接口非原ABI：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |
| `crates/tg-text/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/tg-text/src/shape_stream.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs](../../reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs)：original_pdb_rules_and_scoped_native_or_candidate_comparison<br>[skeleton/task-index/work/TG-aa951cb1d534b93d33/abc-tools-52/crates/tg-text/src/shape_stream.rs](../../skeleton/task-index/work/TG-aa951cb1d534b93d33/abc-tools-52/crates/tg-text/src/shape_stream.rs)：explicit_candidate_delivery_mapping | tg_text::shape_stream::ShapeStream::is_empty（implemented_unverified；业务等价以限定证据为准）<br>tg_text::shape_stream::RawShapeStream::append（scoped_business_comparison；业务等价以限定证据为准）<br>tg_text::shape_stream::RawShapeStream::encode（scoped_business_comparison；业务等价以限定证据为准）<br>tg_text::shape_stream::ShapeStream::encode_no_images（scoped_business_comparison；业务等价以限定证据为准） | candidate_present；builder与raw words CPU子域，未接原GPU/真实resolver生产者。：[evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>tg_text::shape_stream::RawShapeStream::append：51原RNE words/resolver顺序对照；预分配、不执行allocator增长/GPU：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>tg_text::shape_stream::RawShapeStream::encode：原inline私有word规则折入append三分支；非words accessor/独立序列化：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>tg_text::shape_stream::ShapeStream::encode_no_images：indexed允许、named panic静态/候选核对；原panic未执行：[evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-text/delivery.json](../../evidence/small-libs-batch-01/tg-text/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/tg-text/src/color.rs

[reconstruction/shared-foundation/crates/tg-text/src/color.rs](../../reconstruction/shared-foundation/crates/tg-text/src/color.rs)

SHA256：`c007593bb617adf43eb01320db2bfd5816e53e6c7a97b06cad14deadd7128b6d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `scale_u32_packed_srgba_color` | 3 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-text/src/font_table.rs

[reconstruction/shared-foundation/crates/tg-text/src/font_table.rs](../../reconstruction/shared-foundation/crates/tg-text/src/font_table.rs)

SHA256：`1b3c199efb935642ed847e38492c445e177946d3007718c66e81974dc76068b8`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `default_scale` | 21 | 未由扫描证明；以证据限定范围为准 |
| `get` | 31 | 未由扫描证明；以证据限定范围为准 |
| `from_ron` | 34 | 未由扫描证明；以证据限定范围为准 |
| `scaled` | 48 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-text/src/fragment.rs

[reconstruction/shared-foundation/crates/tg-text/src/fragment.rs](../../reconstruction/shared-foundation/crates/tg-text/src/fragment.rs)

SHA256：`336e2087e8930ec81fa03b3a87756d02d0e9fe9dd0036511eba4825d766b21e6`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 27 | 未由扫描证明；以证据限定范围为准 |
| `with_font_table` | 48 | 未由扫描证明；以证据限定范围为准 |
| `set_font_role` | 62 | 未由扫描证明；以证据限定范围为准 |
| `editor` | 73 | 未由扫描证明；以证据限定范围为准 |
| `editor_mut` | 76 | 未由扫描证明；以证据限定范围为准 |
| `replace_editor` | 80 | 未由扫描证明；以证据限定范围为准 |
| `take_editor` | 84 | 未由扫描证明；以证据限定范围为准 |
| `text` | 89 | 未由扫描证明；以证据限定范围为准 |
| `set_text` | 98 | 未由扫描证明；以证据限定范围为准 |
| `attrs` | 123 | 未由扫描证明；以证据限定范围为准 |
| `apply_buffer_attrs` | 132 | 未由扫描证明；以证据限定范围为准 |
| `set_family` | 141 | 未由扫描证明；以证据限定范围为准 |
| `set_weight` | 148 | 未由扫描证明；以证据限定范围为准 |
| `set_metrics` | 154 | 未由扫描证明；以证据限定范围为准 |
| `set_internal_scaled_metrics` | 161 | 未由扫描证明；以证据限定范围为准 |
| `set_max_width` | 174 | 未由扫描证明；以证据限定范围为准 |
| `set_align` | 180 | 未由扫描证明；以证据限定范围为准 |
| `clear_align` | 183 | 未由扫描证明；以证据限定范围为准 |
| `set_cursor_visible` | 186 | 未由扫描证明；以证据限定范围为准 |
| `bounds` | 192 | 未由扫描证明；以证据限定范围为准 |
| `editor_action` | 195 | 未由扫描证明；以证据限定范围为准 |
| `pending_actions` | 198 | 未由扫描证明；以证据限定范围为准 |
| `prepare` | 201 | 未由扫描证明；以证据限定范围为准 |
| `generate_primitives` | 227 | 未由扫描证明；以证据限定范围为准 |
| `primitives` | 315 | 未由扫描证明；以证据限定范围为准 |
| `layout_for_render` | 318 | 未由扫描证明；以证据限定范围为准 |
| `render_in_box` | 333 | 未由扫描证明；以证据限定范围为准 |
| `render` | 355 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-text/src/gpu.rs

[reconstruction/shared-foundation/crates/tg-text/src/gpu.rs](../../reconstruction/shared-foundation/crates/tg-text/src/gpu.rs)

SHA256：`fd1ec515b1900042c58ea684886ffdfbb6798d246f751c1c4cebf4ecf1d39fd0`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `sampled_2d` | 50 | 未由扫描证明；以证据限定范围为准 |
| `new` | 59 | 未由扫描证明；以证据限定范围为准 |
| `data_buffer` | 154 | 未由扫描证明；以证据限定范围为准 |
| `render_offscreen` | 176 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-text/src/lib.rs

[reconstruction/shared-foundation/crates/tg-text/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-text/src/lib.rs)

SHA256：`29f8bf83bc718f5425f1ffad5ee01d5dde57407a724ee0f43678b0475013fe97`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/shared-foundation/crates/tg-text/src/renderer.rs

[reconstruction/shared-foundation/crates/tg-text/src/renderer.rs](../../reconstruction/shared-foundation/crates/tg-text/src/renderer.rs)

SHA256：`a53fa6e5d36108c84f97157562445451d7c1ab014e553f89e573094d967b9e46`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `rect` | 22 | 未由扫描证明；以证据限定范围为准 |
| `primitive_bytes` | 57 | 未由扫描证明；以证据限定范围为准 |
| `consume` | 67 | 未由扫描证明；以证据限定范围为准 |
| `consume_with` | 73 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 78 | 未由扫描证明；以证据限定范围为准 |
| `default` | 122 | 未由扫描证明；以证据限定范围为准 |
| `init_font_system` | 126 | 未由扫描证明；以证据限定范围为准 |
| `new` | 136 | 未由扫描证明；以证据限定范围为准 |
| `with_atlas_size` | 140 | 未由扫描证明；以证据限定范围为准 |
| `load_font_from_file` | 158 | 未由扫描证明；以证据限定范围为准 |
| `load_font_data` | 161 | 未由扫描证明；以证据限定范围为准 |
| `font_faces` | 164 | 未由扫描证明；以证据限定范围为准 |
| `pixels_per_point` | 167 | 未由扫描证明；以证据限定范围为准 |
| `atlas` | 170 | 未由扫描证明；以证据限定范围为准 |
| `set_pixels_per_point` | 178 | 未由扫描证明；以证据限定范围为准 |
| `reset_glyph_cache` | 185 | 未由扫描证明；以证据限定范围为准 |
| `invalidate_atlas` | 188 | 未由扫描证明；以证据限定范围为准 |
| `begin_frame` | 196 | 未由扫描证明；以证据限定范围为准 |
| `cache_len` | 199 | 未由扫描证明；以证据限定范围为准 |
| `pending_uploads` | 202 | 未由扫描证明；以证据限定范围为准 |
| `validate_glyph_cache_allocs` | 205 | 未由扫描证明；以证据限定范围为准 |
| `allocate_atlas_space` | 220 | 未由扫描证明；以证据限定范围为准 |
| `get_glyph` | 241 | 未由扫描证明；以证据限定范围为准 |
| `upload_pending_glyphs` | 306 | 未由扫描证明；以证据限定范围为准 |
| `rect` | 320 | 未由扫描证明；以证据限定范围为准 |
| `rect_with_opacity` | 332 | 未由扫描证明；以证据限定范围为准 |
| `shaped_rect` | 346 | 未由扫描证明；以证据限定范围为准 |
| `append_primitives` | 358 | 未由扫描证明；以证据限定范围为准 |
| `render` | 367 | 未由扫描证明；以证据限定范围为准 |
| `consume` | 370 | 未由扫描证明；以证据限定范围为准 |
| `consume_with` | 375 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs

[reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs](../../reconstruction/shared-foundation/crates/tg-text/src/shape_stream.rs)

SHA256：`bad65b0e5e6bbfda8417cf0823b6e65a75eef135bb1e2d1484f59fbf26649e84`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from` | 18 | 未由扫描证明；以证据限定范围为准 |
| `from` | 23 | 未由扫描证明；以证据限定范围为准 |
| `from` | 28 | 未由扫描证明；以证据限定范围为准 |
| `deserialize` | 36 | 未由扫描证明；以证据限定范围为准 |
| `new` | 56 | 未由扫描证明；以证据限定范围为准 |
| `fill_color` | 59 | 未由扫描证明；以证据限定范围为准 |
| `image` | 63 | 未由扫描证明；以证据限定范围为准 |
| `corner_radius` | 70 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 74 | 未由扫描证明；以证据限定范围为准 |
| `encode_no_images` | 77 | 未由扫描证明；以证据限定范围为准 |
| `from_ron` | 84 | 未由扫描证明；以证据限定范围为准 |
| `new` | 93 | 未由扫描证明；以证据限定范围为准 |
| `words` | 96 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 99 | 未由扫描证明；以证据限定范围为准 |
| `append` | 103 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-aa951cb1d534b93d33/abc-tools-52/crates/tg-text/src/shape_stream.rs

[skeleton/task-index/work/TG-aa951cb1d534b93d33/abc-tools-52/crates/tg-text/src/shape_stream.rs](../../skeleton/task-index/work/TG-aa951cb1d534b93d33/abc-tools-52/crates/tg-text/src/shape_stream.rs)

SHA256：`e3a5b1f6d4e403a2875ca4a50846cb6f256df41384b6cab93465e032bbcb9329`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_empty` | 15 | 未由扫描证明；以证据限定范围为准 |
