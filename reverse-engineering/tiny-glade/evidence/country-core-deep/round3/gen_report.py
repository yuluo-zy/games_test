# -*- coding: utf-8 -*-
"""Generate round3/O-AUDIT-ROUND2.md from merged verdicts."""
import io, json, os
from collections import Counter, defaultdict

OUT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3"
merged = json.load(open(OUT + "/oa2-merged-verdicts.json", encoding="utf-8"))

# reclassify 0x14097d280: evidence file found at reconstruction/country-core-render/wp20
for r in merged:
    if r["addr"] == "0x14097d280":
        r["verdict"] = "一致"
        r["detail"] = "一致（round2 补证）：证据文件实位于 reconstruction/country-core-render/wp20/analysis/0x14097d280_move_gizmo_and_trigger_vol.txt（11KB 反编译，与该址 PDB 吻合）；worker 因在 evidence/country-core-deep 下找不到而标证据不足，路径属 reconstruction 子目录"

def fn_short(fn):
    s = fn.strip("`")
    parts = s.split("::")
    return parts[-1] if parts else s

by_dom = defaultdict(list)
for r in merged:
    by_dom[r["domain"]].append(r)
for d in by_dom:
    by_dom[d].sort(key=lambda x: x["addr"])

buf = io.StringIO()
buf.write("# O 结论抽查复查 · 第二轮（round3 round2，2026-10-15）\n\n")
buf.write("""> 任务来源：第一轮（seed 42）只抽查 40 条 / 1923 条 O 结论，一致率 80%。本轮按域比例补查
> 225 条（?? 顶层 70 含 10 条非 render 分层补抽；terrain-water 因第一轮未覆盖且 coverage 最低补抽 10 条），
> 固定种子 20261015，排除第一轮 O-sample-42.json 全部已查地址。本轮同时复核并纠正第一轮
> "0x14003bb80 簇身份错位"结论（关键方法发现 #2 二阶纠错）。

## 方法与范围

- **O 行清单**：章节表行（逆向结论列以 O 或 **O** 开头；**O** 行 claim 取作用列）。域归属用
  scripts/chapter-audit.py 的 DOMAINS 映射（与 AUDIT stats 口径一致）。
- **抽样**：seed=20261015，每域均匀不放回抽；?? 域 60 条均匀抽 + 10 条非 render 顶层子域分层补抽
  （components/startup/utils/geometry/顶层——第一轮未覆盖；render/** 占 ?? 池约 66%，均匀抽会淹没它们）；
  terrain-water 补抽 10 条（seed 20261015 续抽，排除 round1+round2 已查地址）。
  清单落盘 `round3/O-sample-round2.json`（215 条）+ `round3/O-sample-round2-terrain-supp.json`（10 条）。
- **验证链**：章节行 → 域 decomp/ 既有反编译（先排除 `{"error":...}` 坏文件）→ `rne-read.py` 原字节/常量/串手解
  → PDB 身份（CU 过程表 + **S_PUB32 public-symbols.json 双查**）→ MCP 补反编译（dbid tg_oa2w1..w9）。
- **分级**：一致 / 不一致（代码写 X 证据为 Y）/ 证据不足。9 个 worker 分片 + 主 shard 交叉验证。

## 本轮关键方法发现（先读）

1. **S_PUB32 是 ICF 折叠区的权威符号表**：CU 过程表（load_procs）每地址只留 1 个代表符号，用它断言
   "该址唯一符号"会系统性误判。实测折叠区符号数：0x140002370=1757、0x14003bb80=437、0x14067dd80=141、
   0x140949170=4、0x14092e6a0=2、0x14092e6c0=2。行内 fn 与 pdb_exact 不同时，必须查 S_PUB32 该地址
   是否含行内 fn 符号；有且机器码吻合 → 一致（ICF 折叠注记）。
2. **第一轮 9 行 0x14003bb80 修正本身被推翻（二阶纠错）**：round1 判"该址唯一符号=Plugin::finish；
   原身份未证实"。S_PUB32 实测该址含 draw_path PathRastersResources / slider3d Slider3d /
   steam SteamOverlayOpenResources 的 SystemParam apply/new_archetype/queue 精确符号 →
   原"SystemParam 胶水"身份成立（机器码 1B ret = 无状态胶水 no-op）。9 行已回改（就地修正清单）。
3. **wp15/wp20 证据目录在 reconstruction/country-core-render/ 下，不在 evidence/**：多处章节 evid
   引用 wp15/analysis/xxx.c、wp20/analysis/xxx.txt 在 evidence 下找不到；实际文件在 reconstruction 子目录。
   本轮 2 条证据不足因此补证为一致（0x14097d280；0x14089d9b0 的行为部分）。
4. **"折叠（NotFound）"滥用**：decomp 落盘为 NotFound 不代表函数不存在——0x14092df10（26B）、
   0x1409a90c0（70B）、0x140aabdc0（5B）等均为真实小函数（Ghidra 未建函数），被章节误标"折叠单行"。
5. **反编译截断伪影**：Ghidra 跳表恢复失败会产生截断文本（0x141444c70 "316B 跳板"实为 10.4KB 函数），
   不能按截断长度断言函数大小；跳表基址必须按 lea disp 化验算。
6. **数量断言（"N callers"）无 xrefs 证据不成立**（延续 round1 rotation 规则）：signifiers 一章
   9 处 callers 数量断言中本轮抽到的 "35 callers"（0x14089d9b0）无 xrefs 支撑，已删除。
""")

buf.write("## 逐条结果（225 条）\n\n")
for d in ["??", "wall", "collision-eco", "ui-input", "terrain-water", "history", "render-loop",
          "session-save", "roofs-rt", "camera-time", "audio-misc", "backend-misc"]:
    items = by_dom.get(d, [])
    if not items:
        continue
    buf.write(f"### {d}（{len(items)} 条）\n\n")
    buf.write("| 文件 | 函数 | VA | 判定 |\n|---|---|---|---|\n")
    for r in items:
        rel = r["rel"].replace("/", "·")
        v = r["verdict"]
        mark = {"一致": "✓", "不一致": "✗", "证据不足": "△"}.get(v, "")
        buf.write(f"| {rel[:38]} | {fn_short(r['fn'])[:46]} | {r['addr']} | {mark} {v} |\n")
    buf.write("\n")
    # detail block for non-一致 items
    for r in items:
        if r["verdict"] != "一致":
            buf.write(f"- **{r['addr']}** `{fn_short(r['fn'])}`（{r['rel']}）：{r['detail'][:600]}\n\n")

buf.write("""
## 统计

| 域 | 抽查 | 一致 | 不一致 | 证据不足 |
|---|---|---|---|---|
""")
tot = Counter(r["verdict"] for r in merged)
for d in ["??", "wall", "collision-eco", "ui-input", "terrain-water", "history", "render-loop",
          "session-save", "roofs-rt", "camera-time", "audio-misc", "backend-misc"]:
    c = Counter(r["verdict"] for r in by_dom[d])
    buf.write(f"| {d} | {len(by_dom[d])} | {c['一致']} | {c.get('不一致',0)} | {c.get('证据不足',0)} |\n")
buf.write(f"| **合计** | **{len(merged)}** | **{tot['一致']}** | **{tot.get('不一致',0)}** | **{tot.get('证据不足',0)}** |\n")
buf.write(f"""
一致率 **{tot['一致']/len(merged)*100:.1f}%**（{tot['一致']}/{len(merged)}），较第一轮 80% 显著提高；仍存
{tot.get('不一致',0)} 条不一致与 {tot.get('证据不足',0)} 条证据不足（其中 0x140af8c50 为 O(部分) 主动降级）。
""")

buf.write("""

## 错误模式归类（10 条不一致 + 1 条证据不足，另 1 条补证为一致）

1. **常量/地址错**（0x14090df90 GameConfig::upgrade）：f32 比较常量地址错——claim 写 0x14202018e（0.5?），
   objdump 证实为 1.0@0x142925904 与 0.125@0x142aab384；首指令为 `test byte[+0x88],1`（非 cmp）。
2. **行为断言被机器码推翻**（0x140839560 impl$97::from）：claim"写判别字节 +0x11=8"，机器码 16B 仅拷贝
   payload {16B@+0, u32@+0x10} 无判别写入（对照 impl$96 0x140839550 确有 `c6 41 11 07`）。
3. **"折叠（NotFound）"证伪**（0x14092df10 into_drag_state、0x1409a90c0 on_drag_release）：decomp NotFound
   被当"折叠"证据；实为 Ghidra 未建函数的真实小函数（26B/70B），机器码为条件函数/结构复制。
4. **数量断言无证据**（0x14089d9b0 "35 callers"）：延续 round1 rotation 模式；行为（默认值序列）经
   wp15 反编译证实，数量断言删除。
5. **evid 错引相邻函数**（0x140af6170 replay_edit）：evid 引用 decomp-0x140af5b30.txt（相邻函数），
   本址自有 decomp 存在；作用列是复制粘贴的描述。
6. **截断反编译误当函数大小**（0x141444c70）："316B 跳板"实为 Ghidra 跳表恢复失败截断，函数约 10.4KB；
   跳表基址 0x142c5f2fc（非 0x142c5f248，该址属另一单态）。
7. **尾调目标错**（0x1416286d0）：行内 0x1414433xx，e9 rel32 化验算=0x1414423e0。
8. **串归属错**（0x14171f7f0 flush_deferred_and_sync）：错误日志串的 xref 在 save_thread 主循环
   0x141720680，不在本函数。
9. **方向/条件反**（0x1409e9640 sample_at_world_xz）：debug 门 `*0x14362c308 == 0` 才走 +0x201 分支，
   claim 写"≠0"。
10. **O(部分) 不当完整 O**（0x140af8c50 Toolbar::new）：行为断言未核，0x140af9000 实为函数体内 +0x1b0
    （非独立函数）；行内自带"待核 I"注记，本轮维持证据不足/降级。
11. **证据路径漂移**（0x14097d280）：wp20 证据在 reconstruction/country-core-render/ 下，worker 在
    evidence 下找不到标证据不足；补证后一致。

## 就地修正清单（round3 round2 复查修正，合计 ~80 行）

| 文件 | 行/条目 | 修正 |
|---|---|---|
| systems/paths/draw_path.rs.md | L12/15/16（3 行） | 0x14003bb80 回改：S_PUB32 含 PathRastersResources SystemParam apply/new_archetype/queue → 身份成立（round1 "唯一符号"结论作废） |
| systems/ui/resources/slider3d.rs.md | L11/14/15（3 行） | 同上（Slider3d SystemParam） |
| systems/steam/mod.rs.md | L19/21/22（3 行） | 同上（SteamOverlayOpenResources SystemParam） |
| game_config.rs.md | L12（1 行） | 常量地址 0x14202018e→1.0@0x142925904/0.125@0x142aab384；test byte |
| resources/walls/decorator.rs.md | L47（1 行） | impl$97::from 判别写入证伪 → 16B payload 拷贝 |
| resources/walls/inner_walls.rs.md | L18（1 行） | max_id 误标 → remove_all |
| resources/walls/decorator_archivist.rs.md | L26（1 行） | cull_allow_any 误标 → cull_allow_wall |
| systems/ui/resources/ui_state.rs.md | L27/28（2 行） | "折叠（NotFound）"证伪 → 26B 条件函数；语义未证实注记 |
| systems/ui/resources/widget_params.rs.md | L25（1 行） | "折叠"证伪 → 70B 结构复制 |
| systems/ui_move_shapes.rs.md | L12（1 行） | 证据路径补证（reconstruction/wp20） |
| resources/toolbar.rs.md | L11（1 行） | 0x140af9000=体内+0x1b0；O(部分) 降级注记 |
| systems/ui/signifiers/mod.rs.md | L18（1 行） | "35 callers" 删除（无 xrefs）；行为经 wp15 证实 |
| systems/history/history_process.rs.md | L11-14+L17（5 行） | 316B 跳板→10.4KB；跳表基址按单态；replay_edit evid 纠错 |
| systems/history/replay_path.rs.md | L3+L11（2 行） | 尾调目标 0x1414433xx→0x1414423e0 |
| systems/save/save_thread.rs.md | L14（1 行） | 错误日志串归属改主循环 0x141720680 |
| resources/history/mod.rs.md | L109（1 行） | filter_map 谓词不内联进 16B 迭代器构造 |
| render/pass_builder/draw_builder.rs.md | 43 行 evid | 0x14162c290（函数址）→ "WallColorIds" 类型名串段 @0x14293ff6b |
| render/pass_builder/shader_uri.rs.md | 4 行 guess | source_hash 元 &i32→&self |
| render/mesh.rs.md | L50（1 行） | append 尺寸 170B→570B（PDB） |
| render/render_config.rs.md | L13（1 行） | "字符串枚举"→"字段名匹配"（DisplayConfig） |
| render/raster_single.rs.md | L12（1 行） | 补 MCP 反编译证据（oa2-out/w2-decomp-1409d00a0.txt） |
| resources/terrain_height.rs.md | L11（1 行） | debug 门方向 ≠0→==0 |

## 佐证文件

- `round3/O-sample-round2.json`（215 条抽样清单）、`round3/O-sample-round2-terrain-supp.json`（10 条）
- `round3/oa2-pdb-identity.json`（225 址 PDB 身份：CU 精确符号/owner/S_PUB32）
- `round3/oa2-merged-verdicts.json`（逐条判定合并）
- `round3/oa2-chunks/*.json`、`round3/oa2-results/w1..w9.json`（9 worker 验证明细）
- `round3/oa2-out/`（MCP 补反编译）、`round3/oa2-decomp-index.json`（decomp 存在性索引）
- `round3/dis_game_config.py`、`tools/country-core-audit/dis.py`（objdump 反汇编复核）
""")

io.open(OUT + "/O-AUDIT-ROUND2.md", "w", encoding="utf-8").write(buf.getvalue())
print("written", len(buf.getvalue()))
