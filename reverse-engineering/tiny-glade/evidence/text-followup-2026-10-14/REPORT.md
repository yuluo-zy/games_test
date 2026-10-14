# tg-text 续轮恢复，2026-10-14

范围仅 B Windows x86_64 的游戏文本策略，当前源码为 `D:/game/tiny-glade/crates/tg-text`。未分析用户 demo，未重写 cosmic-text、swash、etagere、Bevy 或 std。当前是行为恢复源码，不是作者原始源码。

## 本轮可审阅实现

- `renderer.rs::render` 从仅返回 atlas descriptor 改为 `(&AtlasDescriptor, u64)`：借用同一 atlas descriptor，并报告最近一次 consume 的图元数。原 `0x140b8f670` 15 字节机器码返回 `RAX=self+0x460`、`RDX=[self+0x440]`，`0x140fac810` 的 consume 开头将 pending primitive len 写入 +0x440。pending 流新增/清空不会立即改变此快照；下一次空 consume 才将其置零。
- `consume_with` 在调用 closure 前保存 count。若回调 unwind，pending primitives/shapes 保留、frame 不推进；成功后清流再推进 frame。`consume` 作为返回拥有 RenderBatch 的 CPU 适配同样保存 count。原完整 consume 还在清流后执行 GPU glyph upload，本 CPU 适配仍由调用方单独调 `upload_pending_glyphs`，没有声称原 GPU 上传链完整等价。
- `fragment.rs::set_internal_scaled_metrics` 原 `0x140b90780` 在 metrics 变化后移走原 lines，调用原 cosmic `Buffer::new_empty`，再用 `Editor::new` 重建。此前候选 `Buffer::set_metrics` 错误保留编辑器 cursor/selection、尺寸/scroll/wrap、auto-indent/tab-width/change 状态。现在复用原依赖 API 恢复重建顺序；文本、行属性和 fragment 的独立 actions 队列保留。相等且有序（非 NaN）时早退。
- scale 变化清 atlas/cache/pending 时保留已请求的 overflow 增长；原 setter 没有写 +0x4d4 flag，不能取消下一次 upload 的倍增。
- 删除 SubpixelMask 的 RGB3 分组 + 最大通道合成 alpha。原 upload 内联策略 `0x14180585e` 对 tag0 alpha mask 展开 `0xffffff | alpha<<24`，非0内容直接传原 bytes 并校验 `width*height*4`。原 swash 0.1.16 的 SubpixelMask/Color 均为 RGBA32；候选现原样保留。当前 cosmic fork `src/swash.rs` 固定 `Format::Alpha`，正常 glyph 链不会产生 SubpixelMask，此分支修正来自静态原码/依赖合同，不声称已跑原 Subpixel runtime。

## 逐项证据与剩余合同

| 项目 | 原观察 | 本轮结论 |
|---|---|---|
| new `0x140b8e4a0`，PDB 3771 字节 | locale 获取失败用 `en-US`；空 fontdb 及 Times New Roman/Arial/Comic Sans MS/Impact/Courier New；FontSystem、etagere 1024×1024；原 native texture 创建尺寸 0x400×0x400、格式0x2b、标名11字节；向 +0x460..0x490 写 native Texture 56字节；+0x440 count=0，+0x4c8/+0x4cc 初值各1，+0x4d0=1.0，overflow=0 | CPU 初始化已有实现并 fresh 复核；native rhapsody Texture/全局设备对象和日志/错误所有权仍缺，constructor未执行。不能把默认CPU constructor当原全部装配 |
| render `0x140b8f670` | 借用 Texture + consume count，getter 无副作用 | 原寄存器执行6组通过；CPU borrowed descriptor+count 已恢复；真实 `&Texture` 为 GPU 适配缺口 |
| consume `0x140fac810`，PDB 759字节 | 先保存count，再依序上传 primitive/shape；成功清两流→upload_pending→32-bit frame+1 | CPU快照、unwind顺序已修，rhapsody上传/失败panic全链未运行 |
| scaled_metrics `0x140b90780`，PDB1280字节 | 双分量有序相等早退；移动lines、新空Buffer、重建Editor | 真实依赖重建已修；零/NaN/极端metrics的原完整入口未执行。原 Buffer::new_empty 约束 line_height!=0，与旧 set_metrics 的 font_size!=0 边界不同 |
| set_family `0x140b90c80`，PDB 544字节 | 输入是角色byte，与+self+0x255比较；经 Arc<FontTable> 的角色查找取得 family与scale，apply attrs，然后对+0x240/+0x244两分量缩放并调用scaled_metrics | 当前 `set_family(String)` 是独立名称适配，`set_font_role` 是显式传表适配。原 shared Arc表、角色状态/lookup失败合同仍待恢复，未伪称同签名等价 |
| set_weight `0x140b90ea0` | u16 weight相同早退，否则更新属性并尾调 apply_buffer_attrs | 现候选策略相符；fresh反编译/原29字节机器码已核；并未执行原复杂属性闭包 |
| set_max_width `0x140b90ec0` | 有序相等早退；changed/NaN保存width并置buffer.redraw，+0/-0相等 | 原inline-owned Buffer路径7组运行通过；候选合同测试覆盖相等/变化/NaN |
| set_align `0x140b90f40` | 仅将byte写入+0x256，不置redraw | 原byte store6组运行通过；候选保持相同dirty策略。clear_align为候选辅助入口 |
| set_pixels_per_point `0x140b8f3f0` | 有序相等早退；NaN进入失效。保存scale，复制当前u32 frame(+0x4c8)到失效stamp(+0x4cc)，清pending image bytes/glyph map、etagere clear、usage条目置零；不写overflowflag，不重建Swash context | 清理范围核实；overflow保留已修。候选generation/frame为u64、失效用递增generation并同帧强制失效；原 u32 rollover和原同帧调度仍是明确差异/未知，未静默宣称等价 |

新 metrics/rebuild 的 generation=0 仍是候选强制失效机制，不是原 Editor 的 Rust 内存布局。SwashCache context 重建仅发生候选 overflow upload，与 scale setter 清理分开。

## 身份、工具结果与验收

原发行 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；持久 Windows Ghidra worker `text-followup-20261014` 绑定 B patched EXE `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。PDB `module624.txt` / `module1185.txt` 的来源与 procedure size 复用上一轮证据；本轮对上述范围的原字节 SHA 重新验证。`fresh-result-corrected.json` 七函数、`closure-result.json` 三函数均 `ok:true`，脚本成功/不超时且响应完整；私有 upload dispatch 在 std iteration 单态化函数内，仅恢复游戏 content 调用策略，未恢复 std iterator。

`save-result.json` 保存成功；正常停止后以同worker重开，`reopen-result.json` 再核目标SHA一致且 getter/scaled_metrics/upload-fold 的 body分别15/701/797字节，证实范围准备持久保存。`stop-final.json` 已正常停止本worker，未停止其他任务。

`fresh-result.json` 是首次 `prepare_functions` 错用 `functions` key 的失败，保留为诊断；后来按现场 schema 改成 `targets`，七项不超过20。没有将初轮失败记通过。Ghidra 保留 void() 占位并标未验证Rust ABI；new/scaled_metrics 的反编译 body受异常分支/共享代码影响可能小于PDB范围，完整 RNE 反汇编补充核对，不以反编译body长度证明覆盖。

`native_contract.py` 执行原 getter6组、set_align6组、max_width7组，共19组全通过，详情 `native-contract.json`。未启动游戏、未调用原 constructor、GPU、native TextFragment metrics重建/FontTable Arc 路径。原7个目标 raw范围 SHA验证通过；上传fold额外范围依据原 PE RUNTIME_FUNCTION（804字节），记录在 `closure-query.json`。

当前 `cargo test --locked --offline -p tg-text --all-targets` 23项通过：既有17项 + 新2项unit/4项合同。第一次新增metrics fixture未载字体而失败（`no default font found`），补发行 Ubuntu font fixture后通过；未改产品字体初始化。最初workspace锁拒绝是并发root锁更新时的观察，本agent未写Cargo.lock。最终联合 Clippy/gpu check及文件SHA由集成owner记录，不以编译成功替代原等价。

剩余：原 shared FontTable/role模型、native Texture/toolbar/SSBO消费者、u32 frame rollover与同帧失效顺序、完整metrics异常输入/完整glyph upload原入口及A/C。GPU本轮不执行。原数据库/发行文件未覆盖；worker准备的函数范围仅保存于独立Windows副本，不声称同步旧Mac或原基线数据库。
