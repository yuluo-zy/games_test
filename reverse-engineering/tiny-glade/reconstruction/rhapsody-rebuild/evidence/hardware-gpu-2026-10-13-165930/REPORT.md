# rhapsody 真实 GPU 性能测试

硬件：Intel(R) UHD Graphics 630；驱动 31.0.101.2141。release 优化构建，原锁定依赖。

现有基准：1 轮预热 + 3 轮，每轮内部取 3–7 次中位数；下表为各轮中位数的中位数。原 blur：每分辨率 2 次预热 + 15 次 GPU 时间戳采样。

## 原 blur shader GPU 时间戳

| 分辨率 | GPU 区间中位 ms | P95 ms |
|---|---:|---:|
| 64×64 | 0.352 | 7.892 |
| 256×256 | 4.591 | 12.266 |
| 512×512 | 19.710 | 38.003 |
| 1024×1024 | 67.561 | 95.094 |

区间为现有 profiler 的 TOP_OF_PIPE→BOTTOM_OF_PIPE，围住 bind/dispatch；可能包含前序上传依赖等待。每次重新建图、资源和管线。P95 用 nearest-rank，15 个样本时等于最大值。不是 shader ALU 活跃时间，也不是完整游戏帧/FPS。

## CPU / 驱动 / 端到端开销

| 场景 | 三轮中位 |
|---|---:|
| 帧循环 1 pass | 0.1674 ms/frame |
| 帧循环 8 passes | 0.2051 ms/frame |
| 重回调串行录制 | 4.1505 ms/frame |
| 重回调原作分块策略 | 1.8643 ms/frame |
| 重回调候选 balanced 策略 | 1.5986 ms/frame |
| 512 buffers 专用分配 | 0.0508 ms/buffer |
| 512 buffers 子分配 | 0.0053 ms/buffer |
| 64×64 RGBA8 上传 | 0.6022 ms/upload |

重回调原策略相对串行加速 2.23 倍；每 pass 人为 spin 50 µs，64 passes，不代表真实游戏 shader 性能。
零 spin 回调原策略/串行耗时比 4.09，并行调度开销在轻回调下明显。

三轮计数器门禁通过，计数稳定；帧循环 32 帧提交 32 次、整设备 idle 0 次、资源登记 2 次。512 buffers 专用分配存活 512 个 memory blocks，子分配 1 个。memory_allocations 是逻辑分配登记，不是 vkAllocateMemory 次数。未覆盖 lavapipe 基线。

## 正确性与限制

正确性测试状态：失败，未修改原测试阈值。日志解析结果：`{"passed": false, "log": "correctness.log", "cause": "unresolved; shader trigonometry/sampling boundary differences are a hypothesis", "pixels_compared": 2116, "loop_counts": {"outer": 31, "inner": 9}, "median_relative_error": 0.0, "maximum_relative_error": 0.011276185, "required_maximum_relative_error": 0.01}`。sin/cos 与采样取整边界差异仅为待核实假设；本轮不宣称原因已查明。

新增计时入口检查所有像素 alpha 覆盖与 64 内部样本；采样点中位误差保持 <1e-4，最大误差是否小于 1% 单独记录，允许失败标志保留性能数据。这不替代原全图正确性测试。

只测当前候选 rhapsody；没有完整原作帧对照、present/FPS、温度/频率/利用率采样或 validation layer 检查。保留原始日志、设备信息、源码 SHA256 与逐样本结果。

复现：`powershell -File scripts/measure-hardware-gpu.ps1`；原 blur GPU 单独运行：`cargo run --locked --release --example blur_gpu_perf`。

数据：[summary.json](summary.json)、[blur-gpu-perf.json](blur-gpu-perf.json)、[correctness.log](correctness.log)、`perf-1/2/3.json`。

## 收尾源码变动

归档时检查到 1 个已有源码文件哈希变化（逐项见 summary.json）。收尾 Clippy：未通过；pipeline_probe：未通过。详见对应日志；实测结果只对应记录的源码版本，不能作为后续变更的当前验收。
