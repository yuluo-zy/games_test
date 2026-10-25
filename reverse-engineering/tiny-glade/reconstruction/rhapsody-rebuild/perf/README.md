# 性能追踪

真实 GPU 测量入口：Windows 下运行 `powershell -File scripts/measure-hardware-gpu.ps1`；原 blur 单独计时用 `cargo run --locked --release --example blur_gpu_perf`。日志、设备信息、源码哈希和汇总写入 `evidence/hardware-gpu-<时间>/`，正确性失败仍保留测量并返回失败状态。

与通用游戏引擎的比较、帧预算和计时限制见 [真实 GPU 性能分析](HARDWARE_ANALYSIS.md)。

```bash
# 本机无显卡时用 lavapipe（软件 Vulkan）跑 GPU 场景；无设备时 GPU 场景自动跳过
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json

cargo bench --bench perf                      # 运行，结果写 perf/last.json（不入库）
cargo bench --bench perf -- --check           # 与 baseline.json 比较，计数器回退则退出码 1
cargo bench --bench perf -- --save-baseline   # 把本次结果存为新基线（有意优化/变更后再做）
```

## 指标分两层

| 层 | 内容 | 用途 |
|---|---|---|
| `counters`（确定性） | `idle_waits` 整设备 idle 次数；`queue_submits` 提交次数；`memory_allocations` `vkAllocateMemory` 次数 | **硬门禁**：任何一项高于基线即失败 |
| `ns_per_op`（有噪声） | 单操作耗时，取多次运行中位数 | **参考**：超过基线 2 倍仅警告 |

另有 `cargo test --test perf_guards` 把最重要的两条不变量固化进常规测试：
帧循环不出现整设备 idle、每帧恰好一次提交；分配统计不退化为二次复杂度。

## 场景

| 名称 | 衡量什么 |
|---|---|
| `cpu/graph_run/passes=N` | 构图→编译→录制→提交→回收的 CPU 全链路，看每 pass 耗时是否随规模上升 |
| `cpu/alloc_accounting/live=N` | 分配统计登记/释放，看每操作耗时是否随存活数上升（二次复杂度探针） |
| `gpu/buffer_create_drop` / `texture_create_drop` | 资源创建/销毁：`memory_allocations` 与 `idle_waits` |
| `gpu/texture_upload` | setup 命令缓冲 + 暂存上传：提交数与 idle 数 |
| `gpu/frame_loop/passes=N` | 多帧会话：每帧提交数、idle 数、耗时 |

## 文件

- `baseline.json`：当前（优化后）基线，入库。
- `baseline_before_optimization.json`：2026-10-13 优化前的数据，保留作对照。
- `last.json`：最近一次运行结果，生成物，已忽略。

## 注意

- lavapipe 是 CPU 软件实现：计数器可信，**绝对耗时不能外推到真显卡**。换到真机/不同驱动后应重新
  `--save-baseline`，不要和别的机器的基线直接比较耗时。
- 新增场景请同时输出计数器；只有耗时、没有计数器的场景无法作为回归门禁。
