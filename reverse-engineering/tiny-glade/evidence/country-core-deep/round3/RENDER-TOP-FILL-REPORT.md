# RENDER-TOP-FILL-REPORT — render 顶层 decomp 补齐（2026-10-15）

render 顶层章节 550+ O 行的证据列原为文档/目录引用（`B_*.md`、`wp*`、`render-loop\decomp`），结论正确但不可独立复验。
本次将 `round3/render-top-job.json`（702 业务地址 + 25 dangling = 727）全部批量反编译，NotFound 手解，交叉核对后
就地改写章节证据列指向具体 decomp 文件。

## 1. 批量反编译

- 命令：`python scripts/run-render-top-fill.py tg_rtf`（4 分片并发 RE-MCP，dbid `tg_rtf_c0..c3`）
- 结果：**650/727 成功**落盘 `evidence/country-core-deep/render-top/decomp/decomp-<va>.txt`，**77 NotFound**。
- 补齐数：**650 份反编译** + **77 份 NotFound 记录**（含 1 个补充反编译 0x140a35240 见 §5）。
- 落盘：`evidence/country-core-deep/render-top/decomp/`（727 文件，大小 69 B–66.9 KB）。
- 汇总：`evidence/country-core-deep/render-top/fill-summary.json`、`fill-report.json`；NotFound 清单存
  `round3/render-top-notfound.json`。

## 2. NotFound 手解（58/77 有章节行，全部 hex 手解）

77 个 NotFound 中 **58 个有对应章节行**，全部用 `rne-read.py --hex` 手解并写入章节证据列（注记「hex 手解」）。
按族归纳：

| 族 | 地址 | hex 结论 |
|---|---|---|
| gpu_world `type_name` | 0x141572130..0x141572260 | `lea rax,[rip+typename]; mov edx,len; ret`（fat ptr，串已证实为 Rust 类型名，如 RoofTileSsbo/WrenInstanceData…） |
| gpu_world `as_any` | 0x141572280..0x1415723b0 | `mov rax,rcx; lea rdx,[rip+vtable]; ret` |
| gpu_world `gpu_flush`/`rg_handle`/`import_to_rg`/`export_from_rg`/`copy` | 0x1415723b0..0x141572400 | `add rcx,0x18; jmp <泛型实现>` 转发 thunk |
| atmosphere `SunSkyColors::lerp` | 0x140a97650 | 12×f32 `self*(1-t)+other*t`（mulps×2+addps，1-t 先算，87B） |
| frame_constants `should_reset_temporal` | 0x140976880 | `cmp dword[rcx+0x80],0; sete al; ret`（11B，+0x80 时间性重置标志） |
| mod `from_preference_order` | 0x140987cb0 | `mov rax,rcx; cmp rdx,2; jb`（len<2 早退）→ len/2 轮迭代偏好序 |
| render_config | 0x140968ad0 resolution、0x140968bb0 TGSR 缩放表查、0x14003ae40 visit_str 跳表 | 字段访问器 / 枚举→f32 表 / serde 派生跳表 |
| instance_data | 0x14098f970 new_string、0x140990af0 clone_behavior | 构造器 / `lea rdx,[str]; mov eax,2; ret` |
| trimesh `index_count` | 0x14129dbc0 | `movzx eax,word[rcx+8]; ret`（u16 索引数） |
| framebuffer_binding | 0x1409092f0 | 判别读 [rcx]==1→移位取指针 |
| pass_resource_access | 0x1408ef170 / 0x1409248f0 | `mov rcx,[rcx]; add rcx,N; jmp` 转发 thunk |
| 其它小函数 | 0x140a97650 等 | 见章节行「hex 手解」注记 |

**19 个 NotFound 无章节行**（job 内/ dangling 引用，未对应任何 O 行）不做章节注记：
0x1412e6650/0x1412e6670/0x1412e6640/0x140911230/0x140911240/0x140911260、
0x14092e5f0/0x14092e620/0x14092e640/0x14092e680、0x1409a8e10/0x1409a9110、
0x140a97860、0x1409149d0、0x140985f30、0x14083c240、0x140005c50、0x1408ef180。

## 3. 交叉核对（逐章 2-3 个代表函数，~60 个核对点 + 58 个 hex 手解）

对落盘 decomp 抽查代表函数，比对章节 O 结论与实际反编译：

- **一致**：atmosphere serialize/lerp、directional look_at/set_direction_towards、frame_constants、
  mesh new/arrayed_indices/add_color/set_attribute、mod、nani_draw（TypeId 0x662978458699a5ef、
  PASS_VARIANT_TABLE 立即数 0x302010000、binding hash 0xcdf9c36a6a9c8c80、RG handle 0x2106412d631790c6、
  MAX_DRAWS_PER_LIST 表 0x142aab288、cmd_slot*20）、gpu_world visit_str（RoofTile/Velocity/WoodenPlank/
  GardenPlank/TreeTrunk 等变体串）、register_required_components（bevy 胶水）、buffers（跳表 0x142aa15a8/1618 +
  0x1b0B memset）、instance_data、mesh_atlas add（Vertex_Position 15 字符键查）、systems curved（kind 0x18、
  +0x40 byte / +0x44 u32、HalftimberColorIds 串）/stone_floor（Roof 0x58B/Floor 0x28B）/maintain、trimesh
  builder（meshlet `param_3/0xc0`、0x40 remap -1 初始化）、vao build_vao、pass_resources new_from（7 个 TypeId，
  B 可选缺→0）、shader_uri parse（':'/'/'/'.' rfind + unwrap panic）、draw_stream new（+0x40=0x100000003、
  blend 1.0、HashMap +0x70）、uniform_binder new（+0xd8==0→None 标签 0xa）/write_*（4 守卫 + 偏移写 +
  slice 越界 panic）、compute/draw/bind_* 系列、shader Compute/Raster 构造等。**均一致**。
- **decomp 受限（一致性未受影响）**：buffers 两函数 Ghidra 跳表不可恢复（`Could not recover jumptable`），
  算法细节依赖 B-doc，但地址/缓冲尺寸/跳表地址与章节一致。
- **load_texture（0x140917860）小注记**：magic 0x48502183 已直读证实；puffin scope 名串与
  「load_compiled_texture_image」措辞未逐字绑定（decomp 引用 UNK_142aacc4e 区做路径/日志拼接；
  串本体 0x142aad64 存在）。章节行已加注。

### 不一致（修正清单）

1. **shader.rs.md 行 0x140002370**（is_exclusive/clone_behavior 桩）：章节原写 `mov rax,rcx; ret 类 5 B 桩`，
   实际 hex 为 `xor eax,eax; ret`（3B，返回 0/false）。已修正为 `xor eax,eax; ret 类 3 B 桩`，注记
   「decomp 补齐交叉核对修正（hex 手解）」。mesh_atlas.rs.md 同址行本已写「3B」正确，仅补 hex 证据。
2. **load_texture.rs.md 行 0x140917860**：puffin scope 名称措辞加注（见上，非实质差异）。

## 4. 章节证据列更新（就地）

- 将 `chapters/render/**`（37 章）含地址行的证据列改写为具体反编译引用（共 **642 行**有地址）：
  - **530 行** → `render-top/decomp/decomp-<va>.txt（原 <旧引用>）`
  - **107 行** → `hex 手解 <va>（rne-read --hex 本批；原 <旧引用>）`
  - **2 行** → 既有其它域 decomp（render_config 0x140914840 → expansion-b2；view_constants 0x140b18f60 → backend-misc）
  - **3 行** 仍文档引用（其中 1 行为布局表 `0x8000000000000000` 哨兵值非地址，误扫；真实缺项 2 行见 §6）。
  - 布局表/非业务行保持原样（函数名作证据的 5/7 列行）。
- 注：多列行（作用列含 `|` 的 9/10/11 列行）证据列定位为 `cols[-2]`，7 列行地址在作用列——均已正确处理
  （曾因按 `cols[6]` 固定列改写，7 个多列行被误写，已恢复并重按 `cols[-2]` 重写，复核 0 残留）。
- 已验证：37 文件行数 vs HEAD 零变化（无行丢失）；`**O**` 计数不变。

## 5. 额外补充

- `mesh.rs.md` `set_attribute<VertexAttributeValues>`（0x140a35240，687B）不在原 702 job 内，补跑单个
  RE-MCP 反编译落盘 `decomp/decomp-0x140a35240.txt` 并更新证据列（hash 查找→原位替换/克隆插入；尾部
  slice.rs 分配 panic）。

## 6. 仍缺项

- **2 个有地址的章节行证据列仍为文档引用**（地址不在 render-top job 内）：
  - `sparse_instance_buffer.rs.md` `SparseInstanceBufferWriter::push<T>`（0x1412e4450，wp2 合同）
  - `ssbo.rs.md` `BufferWriteError`（0x1402b15c0，render-gap §1 #9）
- 布局表行证据列多为函数名引用（`ShadowedBuffer::new / get_mut` 等），非 decomp 文件，未逐一改写（非审计 O 行）。
- 19 个无章节行的 NotFound 地址未手解（job-only/dangling，不影响章节 O 结论）。
- 部分大函数（nani_draw 之外）未逐字节比对 B-doc 细节，仅核对关键标记；B-doc 与 decomp 无冲突。

## 交付物

- `evidence/country-core-deep/render-top/decomp/`（727 反编译文件，含补跑 0x140a35240）
- `round3/RENDER-TOP-FILL-REPORT.md`（本文件）
- 章节证据列更新（`chapters/render/**`，37 文件就地）
- 中间产物：`round3/render-top-notfound.json`、`round3/render-chapter-inventory.json`、
  `round3/render-crosscheck-targets.json`、`render-top/{fill-summary,fill-report,chunk-*-report,ext-report}.json`
