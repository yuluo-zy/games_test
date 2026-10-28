# GPU pass 级性能测试方案（2026-10-27）

状态：**Windows headless Runner 已接入，并完成 static/many_draws 隔离+统计试跑**。
完整三模式、两场景、三次重复矩阵和同机原作对照仍未执行，本文不宣布验收通过。

## 1. 为什么需要这份方案

仓库里已有的 CPU 证据（[当前版本 p95](current-version-p95-20261027.md)）只覆盖 owner 线程：静态场景渲染阶段 CPU p95 12.6120ms、整帧 owner p95 15.7644ms，不含 GPU 等待。

用户在实体机上另行提供过两份数据，**均未入库，本文不把它们当作可复现证据**：

- 一份 GPU 诊断称静态场景 GPU command 合计约 148ms、与墙钟 151ms 接近，因此该机上帧率由 GPU 限制。
- 一份 pass 级 GPU 时间戳（静态 60 预热 / 120 测量帧，many_draws 200 预热 / 120 测量帧，两轮 `valid=true`）排出 G-buffer、Depth of field、G-buffer OC/2、Shadows 为最贵 pass；many_draws 相对 static 的恶化几乎全部来自 G-buffer（+159ms）和 Shadows（+43ms）。

这些数据有两个盲区，本方案专门针对它们：

1. **span 互相重叠。** 起始时间戳写在 `TOP_OF_PIPE`（`rhapsody/src/rg/profiling/mod.rs`），不受屏障约束，一个 pass 的 span 可能包含前一个 pass 的尾部。固定成本 pass（DOF、OC/2）的排名因此不能直接当作各自的开销。
2. **不知道是几何受限还是像素受限。** 条目数不代表几何量：墙砖、植被、石子、草都是 GPU 生成的间接 draw，一条命令可带大量实例。

另一个必须正视的前提：`status.json` 记录的测试 GPU 是 **Intel UHD Graphics 630**，而原作最低配置是 GTX 1060 / RX 580 级别（见 `reports/runtime/2026-10-21-render-perf-architecture/ANALYSIS.md`）。在没有同机原作对照之前，无法区分"原作在这台机器上本来就这么慢"与"还原版比原作慢"。

## 2. 诊断工具（已实现）

两个诊断建立在已有的 pass 时间戳之上，默认全部关闭，关闭时不产生任何 GPU 命令。

| 开关 | 作用 | 代价 |
| --- | --- | --- |
| `TG_GPU_TIMING=1` | 每个 pass 一对时间戳（已有功能） | 小 |
| `TG_GPU_ISOLATE=1` | 每个 pass 的第一个时间戳前插入 `ALL_COMMANDS` 执行屏障，span 只含该 pass 自己的工作 | 帧被串行化，各 span 之和大于生产帧 |
| `TG_GPU_STATS=1` | 每个 pass 一个 pipeline-statistics 查询：输入顶点、输入图元、顶点着色器调用、裁剪调用与图元、片元调用、compute 调用 | 部分驱动上改变时序 |

`TG_GPU_ISOLATE` 与 `TG_GPU_STATS` 都依赖 pass spans，否则配置被拒绝。库层对应 `GpuTimingConfig::{pass_spans, isolate_passes, pass_statistics}`，样本字段 `GpuCommandTiming::statistics`。

Windows headless controller 现在提供 `--pass-spans`、`--isolate`、`--stats`，
分别设置 `TG_GPU_PASS_SPANS`、`TG_GPU_ISOLATE`、`TG_GPU_STATS`。夹具直接配置
`GpuTimingConfig`，不依赖被 headless 夹具禁止的 `TG_GPU_TIMING`。

输出：窗口日志中的 `gpu_spans …` 行；headless session 还保存每个原始 pass span
及其可选 pipeline statistics，并校验 frame/receipt 关联、容量跳过、重复 pass index
和统计缺失。

限制：

- 统计查询只在同时支持 graphics 与 compute 的队列族上记录；其他队列上的 pass 没有统计。
- 需要设备支持 `pipelineStatisticsQuery`；设备创建时只要驱动支持就会启用，不支持时配置返回 unsupported。
- 各厂商对片元调用的计数口径（helper invocation、MSAA 样本、early-z 之后）不完全一致，只用于同机同驱动内的相对比较。
- 隔离模式测得的是**独占成本**，用于给 pass 排序，**不能当作生产帧耗时**。

## 3. 测量矩阵

固定条件（与现有四场景验收一致）：Release、mimalloc、1280×720、world scale 1.0、保留原内部对齐尺寸与画质、关闭 vsync 与帧率上限、不加载 Vulkan validation layer、记录 GPU 型号与驱动版本。

场景：`static`、`many_draws`（draw-list 条目 ≥1,000，当前夹具为 1522 条、480 个克隆）。两者都是固定输入，保证两种模式下 GPU 看到同一份工作。

每个场景依次运行三种模式，每种至少 3 次重复，预热 200 帧、测量 120 帧以上：

| 模式 | 环境变量 | 用途 |
| --- | --- | --- |
| A 基线 | `TG_GPU_TIMING=1` | 与实体机已有数据对照，确认复现 |
| B 隔离 | `TG_GPU_TIMING=1 TG_GPU_ISOLATE=1` | 每个 pass 的独占耗时 |
| C 隔离加统计 | `TG_GPU_TIMING=1 TG_GPU_ISOLATE=1 TG_GPU_STATS=1` | 顶点数、片元数、compute 调用数 |

运行有效性（任一不满足则该次作废，保留日志，不入统计）：

- 时间戳与统计无丢失、跳过或容量不足（`capacity_skips == 0`，`pending` 归零）。
- 进程正常退出，无 validation 之外的设备错误。
- 三次重复之间同一 pass 的均值离散度需要报告；离散过大时不下结论。

## 4. 判读规则

对每个 pass，在模式 B 取独占耗时 `t`，在模式 C 取 `V`（输入顶点）、`F`（片元调用）、`C`（compute 调用）。

| 观察 | 判断 | 方向 |
| --- | --- | --- |
| `F / (1280×720)` 明显大于 1（例如 G-buffer 达到数倍） | 像素受限或 overdraw 高 | 按深度从前往后排序不透明物体；深度相等处需要逐像素验证 |
| `V` 很大而 `F` 不大，且 `t` 随 many_draws 的 `V` 线性增长 | 几何受限 | 保守的视锥与 HZB 剔除、按级联独立剔除阴影；这些需要保证不漏画 |
| `C` 很大 | compute 受限 | 单独分析该 dispatch |
| `t` 在模式 A 很大、在模式 B 小很多 | 原先的 span 含有前序 pass 的尾部 | 该 pass 不是真实瓶颈，排名以模式 B 为准 |
| 全部 pass 的模式 B 之和远大于模式 A 的整帧 GPU 时间 | 生产帧里 pass 之间有显著重叠 | 优化单个 pass 的收益会被重叠吸收一部分，需要整帧验证 |

几条不要直接下结论的情形：

- "条目数少所以不是几何问题"不成立，必须看 `V`。
- "G-buffer OC/2 只有少量条目所以可以跳过"不成立：OC/2 是两阶段遮挡剔除的第二阶段，补画本帧新暴露的物体，跳过会漏画。
- "Final blit 是多余拷贝"不成立：它是原作的 `final_post`，做 LUT 调色并写入 4×MSAA 目标。

## 5. 同机原作对照

这是决定后续方向的实验，优先级高于任何渲染架构改动。

1. 在同一台机器、同一分辨率与画质、同一相机和场景下运行原版 Tiny Glade。
2. 用 PresentMon 记录帧时间；用 PIX 或 RenderDoc 抓同一帧，逐 pass 记录 GPU 耗时。
3. 建立 pass 名对照表：我们的 pass 名与原作抓帧中的事件名一一对应，无法对应的单独列出。

决策：

- 原作整体也只有个位数 FPS：问题在硬件与画质档位，属于产品决策（换验收硬件或做画质分级），不是还原缺陷。
- 原作整体明显更快：逐 pass 对比，**只修比原作慢的 pass**，这是回归而不是架构问题。
- 与原作持平的 pass：继续优化就等于偏离原作，必须先由产品决定是否接受画质变化。

## 6. 与像素验收的关系

当前验收包含严格的水/冰逐像素比对。优化按能否保持输出分两类：

- **可以保证逐位一致**：空输入不声明 pass；输入未变且输出为持久资源时跳过 pass；不透明物体按深度从前往后排序（需逐像素验证深度相等处）；严格保守的按级联独立剔除。
- **改变像素，需要产品确认**：半分辨率的 DOF、体积光、漫反射；GI 或反射降到 30Hz/15Hz；阴影历史缓存与远级联降频；LOD。

TAA 抖动、草、树叶、水面与粒子每帧都在变，因此"相机不动就复用 DOF/体积光/GI 结果"也不是逐位一致。

## 7. 产出物与完成标准

产出物（原始日志与 `.perf` 下的文件保持被 git 忽略，只提交摘要）：

- 每个场景与模式的窗口日志摘要，含设备、驱动、二进制 SHA256、源码提交。
- 一张 pass 表：独占耗时、`V`、`F`、`C`、`F/像素`，按独占耗时降序。
- 同机原作对照表与结论（属于哪一种决策）。

完成标准：三种模式在两个场景上都有有效数据；同机原作对照已完成；每个被列为瓶颈的 pass 都有"几何、像素、compute 或重叠"的归因。在此之前不改渲染架构。

## 8. 已验证与未验证

已验证（Linux，lavapipe 软件光栅，HEAD `f960b524`）：

- rhapsody 全量测试 927 通过、3 失败、12 忽略。3 个失败（`crates_domain` 的 DXGI 测试、`texture_view_usage` 的 2 项，后者需要 Khronos 验证层）在父提交 `446a67be` 上同样失败，与本改动无关。
- 新增测试 `rhapsody/tests/gpu_pass_diagnostics.rs` 通过：光栅 pass 画 2 次三角形得到输入顶点 6、图元 2；compute pass 派发 3 个 workgroup、本地大小 4×2×1 得到 24 次调用；隔离模式下结果相同。
- 窗口汇总类 `GpuSpanWindow` 单独编译并通过 3 个测试。

Windows 实体机增量验证：

- `9ac7eac74` 修复 headless GPU 诊断夹具未初始化
  `isolate_passes/pass_statistics` 的编译错误，Release 编译检查通过。
- `ec3ea62da` 将三模式接入 Windows headless controller；控制器 29 tests 通过。
- 冻结 `ec3ea62da` Release EXE，SHA256
  `e4fe726fe4949bab3c645197db15760307ac03bd440dd2877b9ecc291246f44d`。
- `many_draws` 200 预热 / 120 clean 帧的 isolate+statistics 单轮
  `valid=true`、exit 0、无 pending/capacity skip，保存 109 个命名 pass。
  session SHA256
  `da687b35962c6da63cb2f9758fd37284f60946a21b47fa3b3be0e40d87791510`。
- 单轮隔离成本均值合计 394.270ms，仅用于 pass 排序。最贵为 G-buffer 219.821ms；
  输入顶点 6,243.2k、输入图元 2,081.1k、片元调用 26.90M，即 29.19 次片元调用/720p 像素。
- Depth of field 22.598ms（942.1k compute calls）、G-buffer OC/2 21.412ms、
  三个 Shadows 合计 59.997ms、Deferred lighting 8.982ms。
- `static` 同配置单轮同样有效；隔离成本均值合计 154.692ms。G-buffer 32.779ms，
  统计为输入顶点 1,121.5k、输入图元 373.8k、片元调用 3.16M、3.43 次片元/像素。
  Depth of field 21.355ms、G-buffer OC/2 19.852ms、三个 Shadows 合计
  13.382ms、Deferred lighting 8.542ms。session SHA256
  `ab29eb37e50394bd0f1be3c2bb02916da1fb55b0a12c875401f85115d90f0176`。

仍未验证：

- lavapipe 顺序执行 pass，看不出隔离模式与普通模式的差异，只证明机制可用。
- static、三次重复离散度、统计开销和厂商口径对照尚未完成。
- 同机原作对照尚未执行。

## 9. 复现与自检命令

```bash
# Linux 离屏自检（lavapipe）
export PATH=$HOME/.cargo/bin:$PATH
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
cd tiny-glade
cargo test -p rhapsody --all-features --test gpu_pass_diagnostics
cargo test -p rhapsody --all-features --test gpu_pass_diagnostics -- --ignored --nocapture   # 打印报告

# Windows 实体机：隔离 + 统计（headless controller）
python -B tools/perf_gpu_diagnostic.py `
  --build-json .perf/headless-builds/BUILD/build.json `
  --output .perf/gpu-diagnostics/RUN `
  --scene many_draws --warmup 200 --samples 120 `
  --capacity 200000 --pass-spans --isolate --stats
```

## 10. 预设对照（Performance / Balanced / Quality）

`evidence-gpu-pass-20261027/` 的两次测量使用 `quality` 预设（不开上采样，DoF 开），
所以 TGSR 三个 pass 与内部分辨率缩小后的各 pass 耗时都没有数据。本节补测。

### 预设定义

夹具环境变量 `TG_PERF_QUALITY` 与 controller 参数 `--quality` 对应游戏内 `QUALITY_SETTINGS`
（`gui/settings.rs`）：

| `--quality` | upscaling_mode | upscaling_preset（内部分辨率比例） | dof_enable | 说明 |
| --- | --- | --- | --- | --- |
| `quality`（默认） | Off | Quality | true | 与已有 evidence 一致 |
| `balanced` | Tgsr | Quality（约 0.58~0.67，以 `render_config.rs` 为准） | true | TGSR 加 DoF |
| `performance` | Tgsr | Performance（约 0.5） | false | TGSR，无 DoF |

`UltraQuality` 需要 world scale 1.5 和自适应分辨率路径，不在本对照范围。
`run.json` 的 `settings.quality_preset` 记录实际使用的预设；没有该字段的旧 run 视为 `quality`。

### 步骤

每个预设、每个场景各跑一次（static 与 many_draws），其余参数与已有 evidence 相同：

```powershell
foreach ($q in 'quality','balanced','performance') {
  foreach ($scene in 'static','many_draws') {
    python -B tools/perf_gpu_diagnostic.py `
      --build-json .perf/headless-builds/BUILD/build.json `
      --output .perf/gpu-diagnostics/$q-$scene `
      --scene $scene --warmup 200 --samples 120 --capacity 200000 `
      --pass-spans --isolate --stats --quality $q
  }
}
# 每个 session.json 生成排名
python docs/performance/evidence-gpu-pass-20261027/analyze_gpu_passes.py `
  .perf/gpu-diagnostics/performance-static/session.json --output performance-static-ranking.json
```

必须使用同一个冻结 EXE。`quality` 一组是重复测量，用来对照已有 evidence 确认可复现。

### 要回答的问题与判读

| 问题 | 看什么 | 判读 |
| --- | --- | --- |
| TGSR 自身开销 | `TGSR blur lum` + `TGSR reproject` + `TGSR` 三个 pass 之和，对比 `quality` 组的 `TAA`（4.10ms） | 之和小于约 5ms 则不值得改 `tgsr.cs.hlsl` |
| 内部分辨率缩小能省多少 | DOF、Apply deferred lighting、Volumetrics、Apply screen shadows、Trace GI、Reproject GI 在各预设下的耗时比，对照像素比 | 接近像素比说明像素绑定；不缩说明受固定开销或几何限制 |
| 几何绑定的 pass 是否缩小 | G-buffer、G-buffer OC/2、Shadows 的顶点数与耗时 | 顶点数应不变；耗时若显著下降说明 G-buffer 有片元绑定部分，可回答"顶点还是片元绑定" |
| 总帧时间 | isolated pass total mean | 仅作预设间相对比较，不是生产帧延迟 |
| 画质代价 | 另用 `tools/compare_surface_pixels.py` 对各预设出图 | 本文不判定画质，只给性能侧数据 |

### 完成标准

- 6 组 run 都 `valid`，Vulkan 校验无诊断，`settings.quality_preset` 与目录名一致。
- `quality` 组的 isolated total 与已有 evidence 的 154.7ms 偏差在 run-to-run 离散度内（先看 p95 与已有两次的差）。
- 三个 TGSR pass 在 `balanced`、`performance` 两组中出现，在 `quality` 组中不出现；否则说明预设没有生效，该组作废。
- 结果写入新的 `evidence-gpu-pass-<date>/`，不修改已有 evidence 目录。

### 状态

`TG_PERF_QUALITY` 的夹具实现（`tests/cpu_measurement/mod.rs`）未在 Linux 编译验证
（tiny-glade 在本机无法构建）；controller 与其单元测试已在本机通过。首次运行请先看
`fixture.log` 中的 `perf quality preset: ...` 行。
