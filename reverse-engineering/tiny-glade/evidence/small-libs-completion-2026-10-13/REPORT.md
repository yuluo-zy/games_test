# 三个基础库开发与独立验收

本提交按用户要求立即发布当前源码及有界证据。三个库的生产实现已开发，最后统一回归与扩展 GPU 用例正在执行；未完成检查不标通过。游戏消费者接线未纳入本轮。

- motomoto：真实 File/BufWriter 与故障合同，共11测试已通过；核心算法未改。原 flush 泛型独立运行仍有限定。
- tg-package-integrity：原 verify 隔离容器29例零差异；真实2.70GB包流式读取两次完整结果一致；峰值1.39GiB降至9.4–9.7MiB。旧耗时26.56秒、新103.55秒和warm85.71秒；环境/cache不同，不能宣称速度改善。最后合成测试改动待统一复验。
- tg-text：编辑/排版/fallback/cache/atlas/原48B协议、opacity和center/extent包装、候选ECS及GPU已实现。原rect20组与opacity88组、17CPU测试对应证据见 `../tg-text-full-2026-10-13`。实际Intel UHD630原shader完成空帧、glyph/solid哨兵、冷暖缓存、圆角/图片、scale和错误路径；扩展四种图片缩放/重建恢复待最终统一结果。

发布入口：`../../reconstruction/small-libs`，直接引用相同生产源码。GPU完整输入及原文件保持本地，只发布方法/输入哈希/结果。原代码隔离容器验证与原发行进程验收分别记录；Bevy fork及已有rhapsody allocator候选偏差未抹去。
