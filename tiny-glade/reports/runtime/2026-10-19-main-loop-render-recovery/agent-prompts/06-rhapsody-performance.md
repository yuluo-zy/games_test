# 底层正确性与实测性能

先读取同目录 COMMON.md。本文件覆盖此前创建6名子agent的所有指令；你现在是单一执行者。

目标：为真实游戏主链修复当前refactored rhapsody的具体缺陷，并保持/改善实测性能；此任务合并此前performance_team职责，不唤醒其成员。
输入：当前rhapsody OPTIMIZATIONS/REFACTOR_PROGRESS、最近Git diff、bridge/frame/raster host实际调用者；已有M3/WindowLab测量须核source输入绑定，摘要缺测量不等于从未测过。
写面：rhapsody及游戏host适配的有证必要API候选；如与02/03同文件冲突只给root必要hunk，不双写。
正确性：graph compile/record/submit错误原子性，queue ownership/acquire/present，descriptor/reflection/BDA与view范围，temporal/named同owner交接、transient alias lease和retire、PresentPolicy及内容初始化合同。尊重用户已修borrowed-adopt预检/graph_id cleanup，不退回旧实现。
性能：独立找实际热点：CPU构图、每帧分配/锁/无谓驱动查询、compile/prewarm/wait、descriptor/VAO cache和上传、VRAM增长。只针对有据问题交最窄修复；给可复用现有探针/统一测量命令，root执行。优化前后同场景同输入，至少报告CPU/分配/等待及GPU完成边界，不许以功能减少提速。
验收：受影响语义测试/真实GPU当前路径、多帧缓存命中和正确失效、在飞资源安全回收、无稳定增长；不能据单次耗时声称性能达标。
