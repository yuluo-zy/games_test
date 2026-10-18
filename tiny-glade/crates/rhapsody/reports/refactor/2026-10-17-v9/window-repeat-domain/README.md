# 重复窗口脚本与关闭实账领域验证

使用保存的 `window_lab`，SHA256 `c780a6a94a476830db808b2f900e8d3830ae1dbd6a4ceefac3e94ed167b88f06`。`build-manifest.json` 绑定源指纹、离线锁定构建命令与构建前后稳定状态；对应 Window CPU 21 项和严格 Clippy 通过。原生运行使用 Apple M3 / MoltenVK、Khronos 与同步验证。

| 场景 | 实际结果 |
| --- | --- |
| fixture | 360 presented / 360 verified；第 300 帧与关闭前均为 3 个图资源、2 个命令及 2 个命令池 |
| raster-repeat | 900 presented / 900 verified；3 次错误编辑拒绝、3 次帧边界发布、3 轮完成 |
| wsi-repeat | 900 presented / 900 verified；368 次真实 resize、73 次完整最小化/恢复、3 个完整批次；事件环保留 140 条，累计淘汰 374 条 |

三次运行退出码均为 0，验证层 error 为 0，呈现关闭完成且保留 WSI 代际为 0。释放 Lab 与设备所有者后，已登记资源数量、资源字节、已知 CPU 存储和总预算均为 0。应用读回图保存在各场景目录；它们不代表屏幕截图或键盘检查。

`runs.json` 保存两次重复运行的完整原生命令。fixture 使用同一二进制和相同 Vulkan 环境，参数为 `--scene fixture --frames 360 --fps 240 --gpu-timing on --require-validation`，报告、读回及缓存指向 `fixture/`。

本构建早于随后实际 Session 的访问表、回执池、资源投影及提交临时存储改动。这些结果只证明该保存快照中的重复脚本和关闭行为，不能替代最终源码矩阵、30 分钟长跑或性能门。磁盘缓存收益也不能由这三次功能运行推断。
