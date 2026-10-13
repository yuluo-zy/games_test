# tg-text：原作 B 版业务规则（batch-02 复核）

状态：TextRenderer/TextFragment 策略经本轮 RNE 反汇编复核后对齐原签名；behavior 6/6 与 text_pipeline 9/11 通过。2 个 text_pipeline 失败为环境相关（macOS 字体映射 / Windows 专属字体路径），非本轮回归。toolbar/SSBO 真实消费者与 GPU 完整验收仍后置。

## 来源和证据身份

- 原锁 tg-text 0.1.0 local；PDB 路径 /home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/lib.rs（renderer/fragment 均合并在 lib.rs，与候选分文件不同仅为组织差异）。
- 反编译：Ghidra shared worker（B patched EXE cf048e27...）；机器码：radare2（RNE f507a6ce...）。worker=tgtext-r2（只读）。

## Observed（本轮新确认/修正）

| 原规则 | 依据 | 实现与对照 |
|---|---|---|
| **TextRenderer::rect 签名 = rect(pos, extent, opacity:f32, shapes)**：无独立 color 参数；颜色固定 0xffffffff，alpha 经 utils::scale_u32_packed_srgba_color 缩放 | RNE 0x140b8f510：`mov ecx,0xffffffff; movaps xmm1,xmm3; call 0x140c918d0`；toolbar 调用方 0x140680330 传 opacity(0.5f) 与 shapes | 候选 rect 已改为 opacity 语义；新增 rect_with_color(extension) 保留显式颜色 |
| rect 断言 `stream_len < 256`；panic 消息 "assertion failed: stream_len < 256"，源 crates/tg-text/src/lib.rs | RNE 0x140b8f625 引字符串 | 候选 assert 保留 |
| rect stream 字段 = (当前 shape 词数 << 8) \| shapes.len；primitive 48B（pos/uv/color/stream/padding 布局） | RNE 0x140b8f57a shl r12d,8; or r12d,edi；0x140b8f597-0x140b8f5c1 字段写入 | 候选一致；候选额外断言 start<=0x00ff_ffff 为防御扩展 |
| **TextFragment::set_internal_scaled_metrics 签名 = (renderer, font_size, line_height)**：参数为已缩放 metrics；与当前 buffer 相等且均非 NaN 时早退；变化时重建 buffer | RNE 0x140b90780：参数比较 + NaN 守卫 + Buffer::new_empty/行复制 | 候选已改为带参+早退；重建用 set_metrics（等价候选实现，非逐行复制） |
| **FontTableEntry::scaled 操作数顺序 = self.scale * input（scale 第一操作数，mulps）**；family 复制 16B；两分量无 clamp | RNE 0x140b92093-0x140b920a8：movups family; movsldup scale; mulps scale,input | 候选已改 self.scale * font_size / self.scale * line_height |
| 原 scaled 对 inf*0 产生 0xFFC00000(-QNaN)；本机 LLVM mulss/mulps 规范化 0x7FC00000(+QNaN) | fixture metric case 16 期望 0xFFC00000；本机实测 0x7FC00000 | 两者均 quiet NaN，测试已 NaN 容忍；登记工具链边界 |
| prepare：actions 排空后逐条 editor.action()，但**遇到 Action 判别值 0x22 时 break 停止** | RNE 0x140b90f60：`if (*piVar2 == 0x22) break;` | 候选全量应用；差异待分类，未改 |
| prepare：bounds[0](+0x238) 布局闭包；bounds[1](+0x23c)= 字体 metrics line_height（直读 fVar1） | RNE 0x140b90f60：pfVar4 按角色选 +0x154/+0x11c/+300，fVar1=*pfVar4 | 候选 bounds[1] 用 layout_runs fold（line_top+line_height）；差异已登记，未改 |
| render：generation 检查（+0x24c < renderer atlas generation 时重新生成 primitives），generate_primitives 内联 | RNE 0x140b91120 | 候选结构一致（prepare+layout+generate_primitives+append_primitives） |
| replace_editor：drop_in_place 旧 editor + memcpy 新值(0x1a0)；返回 void | RNE 0x140b906d0 | 候选 mem::replace 返回旧值——重建扩展；原丢弃 |
| take_editor：创建空 buffer，旧 editor 复制返回，原槽置空 | RNE 0x140b91620 | 候选一致 |
| TextRenderer::render（0x140b8f670）是 getter：返回 (u64, &self+0x460) 流视图 | RNE 0x140b8f670：`mov rdx,[rcx+0x440]; lea rax,[rcx+0x460]; ret` | 候选 render()->AtlasDescriptor 为重建扩展，差异已登记 |
| FontTable::get：HashMap(RandomState) 按 FontFamily 哈希查找 | RNE 0x140b91fa0 | 候选一致 |
| consume：primitives/shape 词数快照+0x440，rhapsody 上传（结果 0xb 校验），reset +0x420/+0x438，upload_pending_glyphs，帧计数+0x4c8 | RNE 0x140fac810 | 候选 consume/RenderBatch 语义一致（GPU 为 rhapsody 接线） |

## 依赖与接线

- cosmic-text fork 0.11.2（PounceLight rev 3da2a14，features std+swash）与 etagere 0.2.10 为原锁版本；consume/upload/get_glyph 反编译确认候选接线与原调用一致。逐字节 fork 等价属依赖边界，未验收。
- GPU 上传/字形 atlas 经 rhapsody update_buffer_bytes/update_texture；本机不跑真实 Vulkan。

## Inferred / reconstruction choices

- rect_with_color / rect(pos,extent,opacity) 的拆分是本候选对齐原签名后的选择；原无 rect_with_color。
- set_internal_scaled_metrics 重建用 cosmic-text set_metrics 而非原 Buffer::new_empty+行复制（语义等价候选）。
- bounds[1]、Action 0x22、replace_editor 返回、render getter 契约四项保留为待分类差异，未擅自改写。
