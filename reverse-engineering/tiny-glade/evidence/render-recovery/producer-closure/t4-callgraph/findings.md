# T4 取证：全镜像直接调用图

- 目标：PE `workspace/input/tiny-glade.exe`（SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`）。
- 方法：capstone 逐函数有界解码（**绝不裸扫 `.text`**）+ 四源函数边界并集 + gap 二次回收。
- 产物：`t4-callgraph/{scan_calls_full.py, country_core_subgraph.py, analyze_coverage.py, compare_repro.py, callgraph-full.json(42.9 MB), country-core-subgraph.json(418 KB), repro-capstone-rhapsody.json, repro-objdump/, compare-report.json, full-scan.meta.json, country-core-subgraph.console.txt}`。
- **未改动任何受保护文件**（workgraph 文件 mtime 仍为 11:17）。

## 0. 父级复核

- `callgraph-full.json` = **42,905,380 字节（42.9 MB）** ✓
- `country-core-subgraph.json` = 417,998 字节，结构 `{keys, reverse, country_core_degree}`；`keys["0x1404e8010"]` = `{"label":"tiny_glade::render_loop::render", "known_end":"0x1404ea7cf", "instruction_count":1780, "direct_callees":[…]}` ✓
- **`compare-report.json` 的决定性内容**：

```json
{"A_vs_B": {"direct_diff": [], "ext_diff": [], "ind_diff": []},
 "A_vs_C": {"direct_diff": [], "ext_diff": [], "ind_diff": []},
 "counts": {"A": {"fns":450,"direct":4382,"ext":26,"ind":138},
            "B": {"fns":450,"direct":4382,"ext":26,"ind":138},
            "C": {"fns":450,"direct":4382,"ext":26,"ind":138}}}
```
⇒ **三种独立方法（已提交 / objdump 重跑 / capstone 自研）边集完全一致，零差异。**

## 1. 门禁复现（一致性）

用原生成脚本 `scripts/render-recovery/build_rhapsody_workgraph.py`（`--output` 改指本目录）重跑，产出与已提交的 `workgraph/calls.json` / `functions.json` / `summary.json` **逐字节 IDENTICAL**。

再用自研 capstone PE 扫描器对同一 450 函数复现：

| | 函数 | direct | external_jumps | indirect |
|---|---:|---:|---:|---:|
| A（已提交） | 450 | 4,382 | 26 | 138 |
| B（objdump 重跑） | 450 | 4,382 | 26 | 138 |
| C（capstone 自研） | 450 | 4,382 | 26 | 138 |

**0 处边差异。** `summary.direct_call_sites = 4408 = 4382(direct) + 26(tail-jump)`。
唯一差异是 `instruction_count`（objdump 把裸 `lock f0` 前缀单列成一条指令多计，**与边无关**）。

## 2. 全镜像统计（观察）

- **函数起点 126,209**（`.pdata` 116,269 + rhapsody 450 + country_core 1,829 + `.text` 公共符号 71,646，按起点去重）。
- **直接 `call rel32` = 419,936**
- **`call [reg]` 间接 = 20,840**
- **`jmp rel32` 尾跳 = 18,015**（**分开单列，不混入 direct**）

**解析率**：**419,822（99.97%）** 落在函数起点；114 落在已知范围内但非起点；**0 落在所有已知函数之外**。
方法：先用四源范围界定，再对「仅作为被调目标但无独立边界」的 **1,272** 个未解析站点做二次回收（338 个叶子入口，45,423 条指令），**使外部目标归零**。
`.text` 字节覆盖 **93.91% → 约 94.3%**。

## 3. country_core 层调用图（观察）

**1,829 条渲染符号 → 1,147 个唯一 VA**（ICF 别名）。

层内边：**cc→cc 2,494**；**cc→外 10,678**；**外→cc 10,203**；cc 间接 203；cc 尾跳 649。

**关键链**：

| 节点 | 被调者数 | 调用者 |
|---|---:|---|
| `render`(`0x1404e8010`) | **118**（含 `RenderGraphState::compile`、`DrawLists::new/build/sort`、`Swapchain`/`ResourcePool`） | **仅 2**（`__rust_begin_short_backtrace` 拼板 + 一个未命名入口 ⇒ 帧渲染入口） |
| `compile`(`0x140161cd0`) | **152** | `render` × 2 |
| `render_world`(`0x1404ec110`) | **99** | `compile` × 1 |

`render_world` **直接调用主要 pass**：`upload_world_raster`、`compute_water`、`rasterize_terrain_stroke`、`displace_rocky_terrain`、`read_terrain_data`、`rasterize_buildings`、`compute_garden`、`path_pebbles`、`leaf_particles`、`roof_tiles_gravity`、`stairs`、`gpu_generate_draw_lists`、`shadow_pass`、`calc_terrain_lighting_texture` 等。

⇒ **链：`render → compile → render_world → {compute/draw passes}`** —— **与 C1（顺序）和 T5（t5 说 begin_frame 的调用者不在 450 集内）三方互证。**

## 4. 假阳性控制（观察）

- **仅解码有界函数体**（四源并集），**不裸扫数据区**；
- 已知 objdump 前缀伪指令差异；
- gap 回收的越界风险**实测极小**（仅 +10 条 direct，抽查 `0x142571530` 为真实 SIMD 例程）；
- **ICF 同址多名字保留**（不合并语义）；
- **`.pdata` 仅用于边界，非当引用**（**这正是 T5 纠正的那一点**）。

## 5. 仍不可得

1. **间接 `call [reg]` / vtable 目标**：全镜像 **20,840 处（4.7%）**；**cc 层 203** —— 无法解析（见 T5：**0.35% 可解**）。
2. **约 6% `.text` 不属于任何有界函数**（填充 + 仅被间接调用的叶子），其内部调用**未计入**。
3. **静态边 ≠ 运行时执行**（分支 / 内联 / 间接派发可绕过）。

## 6. 结论

| 问题 | 答案 |
|---|---|
| 直接调用图能抠出来吗？ | **能，且已抠出** —— 全镜像 419,936 条，99.97% 解析到函数起点 |
| 可复现吗？ | **可** —— 三法零差异 |
| country_core 层有了吗？ | **有了** —— 1,147 个唯一 VA，层内 2,494 条边 |
| 间接调用呢？ | **基本不能**（20,840 处，静态只解 0.35%） |
| 覆盖缺口 | 约 6% `.text` 无边界；间接目标不可解 |

⇒ **"调用图"这个结构性障碍，一半已经解决**（直接边）；另一半（间接）**静态无解**，需动态证据。
