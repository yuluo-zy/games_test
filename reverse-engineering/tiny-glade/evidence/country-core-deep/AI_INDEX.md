# Tiny Glade country-core 逆向证据总索引（AI 可读）

> 用途：给后续 AI 会话/agent 快速定位逆向内容与查看方式。本文件是**导航入口**，
> 不含细节——细节在各指向文件中。更新：2026-10-15。

## 0. 一句话定位

对 Tiny Glade 原作（B 版，`tiny-glade.rne` sha256 `f507a6ce…`）`country-core` crate
（`src/systems/**` `src/resources/**` `render/**` 等 17 功能模块、523 文件、3106 业务函数）
做了**行为级逆向**：每个函数的作用、算法、数据结构、常量、调用链，全部沉淀为
**章节（chapters）+ 域级文档（DIGEST/ALGORITHMS）+ 反编译证据（decomp）**。
证据全部机器码可复验（~3200 份反编译落盘 + 两轮 O 抽查一致率 95%）。

## 1. 目录树（evidence/country-core-deep/）

```
country-core-deep/
├── INDEX.md                ★ 中央索引（领域状态/冲突登记/跨域闭合）— 先读这个
├── PLAYBOOK.md             ★ 挖掘手册（工具入口/Rust 逆向关键事实/纪律）— AI 作业前必读
├── OVERVIEW-2026-10-14.md  状况总览（工作背景/方法/资产结构）
├── MODULE_MAP.md           ★ 功能模块地图（523 文件 → 每文件作用/业务函数数/状态）
├── SOURCE_MAP.json         ★ 源码地图（420 文件/4027 函数：名称/地址/kind/源码行）
│   ├── SOURCE_MAP-render.json / -extra / -expansion / EXPANSION（扩展覆盖）
├── chapters/               ★ 539 章 = 每文件一章节（文件作用 → 业务函数表 → 算法级 → 胶水）
│   └── chapters-expansion/（顶层模块补充）
├── <域>/                   ★ 每域一个目录，含：
│   ├── DIGEST.md          域级权威文档（布局表/O 事实/panic 串/合同纠错/跨域线索）
│   ├── HIGHLIGHTS.md      10 条最重要发现
│   ├── ALGORITHMS.md      算法级汇总（处理链图/逐函数算法/常量表/状态机）— 21 份
│   ├── CLOSURE-*.md       未探究项闭合报告（round3 起）
│   └── decomp/            该域反编译落盘（decomp-<va>.txt）
├── round2/                 第二轮审查：<域>-REVIEW.md（逐文件"代码写X证据为Y"）
│   └── FIX-LIST.{md,json} 统一修正清单（277 条：OK/FIX/WRONG/PARTIAL/GAP/STUB/LATER）
├── round3/                 第三轮：
│   ├── MODULE-DEEP-DIG.md  ★ 全轮台账（每任务交付/要点/纠错）— 最新状态看这里
│   ├── MODULE-DEEP-SUMMARY.md 模块深挖程度汇总表
│   ├── O-AUDIT-REVIEW.md   O 抽查第一轮（40 条，80% 一致）
│   ├── O-AUDIT-ROUND2.md   O 抽查第二轮（225 条，95.1% 一致）
│   ├── RENDER-U-CLOSURE.md 渲染 U 项闭合（9 闭合/2 卡点）
│   ├── REVIEW-UNEXPLORED/ 5 组审查"未探究清单"（groupA..E + MERGED.md）
│   ├── RENDER-TOP-FILL-REPORT.md  render 顶层 702 反编译补齐报告
│   ├── DANGLING-FILL-REPORT.md   25 条悬空引用核实报告
│   └── oa2-results/        O 抽查分片（w1..w9.json）
├── rhapsody/               core↔rhapsody 调用链调查（CALL-CHAIN.md + decomp）
├── cross-domain/           跨域闭合证据（photo_mode↔CameraMode 等）
└── tools-extra/            补充工具（disasm-va.py 等）
```

**17 个功能域目录**：wall（最大 426 decomp）/ render-loop / render-top / collision-eco /
ui-input / audio-misc / terrain-water / history / session-save / camera-time / roofs-rt /
backend-misc / asset-libraries / input / components-geometry / startup / utils（+utils-input-config、expansion-b1/b2）。

## 2. 从问题到答案的查找路径

### 想找：某个函数/地址是干什么的
1. `SOURCE_MAP.json` → 按地址/函数名查（含 kind：business/glue/inline/dep、源码行号）
2. 函数所在文件 → `chapters/<路径>.md` 对应章节 → 该函数的"作用/逆向结论/证据"行
3. 需要算法细节 → 同域 `ALGORITHMS.md` 或 `DIGEST.md`
4. 需要反编译原文 → `<域>/decomp/decomp-<va>.txt`

### 想找：某功能模块的全貌
- `MODULE_MAP.md`（每个顶层目录/子目录的文件清单+作用+业务函数数）
- 该域 `DIGEST.md`（权威事实）+ `ALGORITHMS.md`（算法级）+ `HIGHLIGHTS.md`（要点）

### 想找：当前还有哪些没挖/待裁决
- `round3/MODULE-DEEP-DIG.md`（台账，最新）
- `round3/REVIEW-UNEXPLORED/MERGED.md`（5 组合并未探究清单）
- 各域 `CLOSURE-*.md`（卡点表）
- `INDEX.md`（跨域冲突登记 + 未决疑问）

### 想找：现有 reconstruction 代码哪里与机器码不符
- `round2/<域>-REVIEW.md`（"代码写 X，证据为 Y"逐条）
- `round2/FIX-LIST.md`（统一修正清单）

### 想找：某个依赖/渲染框架怎么调
- `rhapsody/CALL-CHAIN.md`（core↔rhapsody 30 入口对照）
- `MODULE_MAP.md` render 节

## 3. 工具（scripts/）

| 工具 | 作用 | 用法 |
|---|---|---|
| `scripts/rne-read.py` | 原字节/字符串直读 | `--hex <va> <n>` / `--fat <va>`（&str）/ `--unks <decomp文件>`（批量解串） |
| `scripts/rne-disasm.py` | capstone 反汇编小函数 | `rne-disasm.py <va>` |
| `scripts/run-decomp-batch.py` | 批量反编译（域分片并发） | `run-decomp-batch.py <domain> <dbid> [--chunks N] [--lock]` |
| `scripts/chapter-audit.py` | 章节结论质量审计（O/I/U/empty） | 直接运行 |
| `scripts/run-render-top-fill.py` | render 顶层批量补齐 | `run-render-top-fill.py <dbid>` |
| `scripts/mcp-job.mjs` | RE-MCP 定点调用 | `mcp-job.mjs re_mcp_ghidra <job> <out>` |
| `tools/country-core-audit/comp_register_map.py` | TypeId 批量扫描 | — |

**MCP 后端**：RE-MCP（预分析 116,270 函数，`program=tiny-glade.exe`），dbid 各 agent 专属。
**权威符号表**：`evidence/render-architecture/public-symbols.json`（S_PUB32，10.1 万条——ICF 折叠区以此为准）。

## 4. 关键逆向事实（复用于后续作业，详见 PLAYBOOK）

1. **游戏 Vec = cap-first**：`{cap@+0x00, ptr@+0x08, len@+0x10}`（与 std 不同）
2. **i64::MIN 哨兵** = Option 判别；enum niche 判别还原 = `x - 0x7fffffffffffffff`
3. **Arc 克隆** = `LOCK; inc`；Arc drop = `LOCK; dec` 归零释放
4. **Bevy ResMut** = tick 回写（`param[2] = param+0x1c`）
5. **S_PUB32 只含 pub 项**；私有函数查骨架 CU"编译实例 RVAs"或 .rdata CU 函数边界表
6. **PDB 名与行为会错位**——机器码为准
7. **索引层级**：v1（错位）→ v2（S_PUB32 pub）→ v3（合并，当前权威）
8. **构建环境**：`/home/h3/code/pouncelight/country-slice-private`（Pounce Light，Linux 交叉编译 Windows）
9. **全局变量族**：0x14362c420=panic 门控、0x14362c3a8=日志级别、0x14362c011=楼梯校验开关（v>0x1a）、0x14362c308=CENTERED_PATH（v>0x19）、0x143634d90=demo 开关

## 5. 权威数据源

| 文件 | 内容 |
|---|---|
| `tracking/original-Cargo.lock` | 原版 901 依赖锁 |
| `skeleton/B/crates/country-core/src/**` | B 版 PDB 骨架（函数符号/源码行/编译实例 RVA） |
| `reconstruction/country-core/src/**` | 重建源码（当前按 FIX-LIST 待修） |
| `reconstruction/country-core-render/wp*/contracts/*.md` | 各工作包合同（部分已被新证据纠正） |
| `evidence/country-core-audit/B_*.md` | B 阶段渲染审计（20 份，与新证据互证） |

## 6. 当前状态一句话

- country-core 内部：3106 业务函数全部有结论，候选 U/空行归零，证据机器码可复验
- 遗留：~136 条低价值 I 行（薄闭包/泛型内联）、~8 条跨域裁决项（MainActionSet 双形态、Events fork、FrozenMap insert 等）、**下层依赖 crate**（frozen/toolbar/shader-*/rhapsody 11 入口）未深挖
- 实时台账：`round3/MODULE-DEEP-DIG.md`（每轮任务/交付/纠错追加）
