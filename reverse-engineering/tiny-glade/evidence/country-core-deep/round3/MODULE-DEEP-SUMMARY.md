# country-core 功能模块深挖程度汇总（2026-10-15）

数据：SOURCE_MAP.json（523 文件 / 3106 业务函数）+ chapters（537 章）+ 各域 ALGORITHMS.md + round3/MODULE-DEEP-DIG.md 台账。
标注：`算法级`=章节已含"实现方式（算法级）"节；O/I/U = 章节结论分级；待证 = 无结论行。

## 总览

| 模块 | 章 | 业务函数 | 算法节 | O | I | U | 待证 | 深挖程度 |
|---|---|---|---|---|---|---|---|---|
| render（render/ + render_loop） | 120 | 957 | 120 | 701 | 27 | 0 | 1 | ✅ 算法级全量 + 审查 |
| wall | 83 | 431 | 82 | 312 | 80 | 38 | 1 | ✅ 算法级全量（6 子任务并行） |
| asset_libraries | 12 | 387 | 0* | — | — | — | — | ✅ 算法级（ALGORITHMS.md） |
| collision-eco | 35 | 217 | 29 | 209 | 3 | 1 | 0 | ✅ 算法级全量 |
| audio-misc | 52 | 177 | 51 | 23 | 0 | 1 | 0 | ✅ 算法级全量 |
| ui-input | 35 | 170 | 35 | 165 | 3 | 1 | 0 | ✅ 算法级全量 |
| history | 27 | 115 | 27 | 120 | 10 | 0 | 0 | ✅ 算法级全量 |
| terrain-water | 37 | 106 | 36 | 104 | 1 | 0 | 0 | ✅ 算法级全量 |
| session-save | 16 | 103 | 16 | 97 | 1 | 5 | 0 | ✅ 算法级全量 |
| roofs-rt | 17 | 81 | 12 | 77 | 3 | 0 | 0 | ✅ 算法级 |
| components | 3 | 78 | 3 | 0* | — | — | — | ✅ 算法级（ALGORITHMS.md） |
| camera-time | 20 | 78 | 20 | 72 | 6 | 0 | 0 | ✅ 算法级全量 |
| utils | 13 | 72 | 5 | 3* | — | — | — | ✅ 算法级（ALGORITHMS.md） |
| startup | 36 | 61 | 36 | 0* | — | — | — | ✅ 算法级（ALGORITHMS.md） |
| input | 12 | 38 | 8 | 10* | — | — | — | ✅ 算法级（ALGORITHMS.md） |
| backend-misc | 14 | 22 | 14 | 22 | 0 | 0 | 0 | ✅ 算法级全量 |
| geometry | 4 | 13 | 4 | 0* | — | — | — | ✅ 算法级（ALGORITHMS.md） |

* 注：asset_libraries/components/utils/startup/input/geometry 六模块的结论集中记录在各自 ALGORITHMS.md（章节行表格简略，算法文档详尽）。

## 各模块深挖成果摘要

### render（最大模块：957 业务函数）
- **115/120 章算法级**（render 顶层 37 + render_loop 78；5 章为纯胶水/空壳补实）
- 帧编排链图 W1-W54 + 条件真值表；pass 声明→执行链；dispatch 参数表；nani 实例化链
- 关键闭合：render_world 54-pass、build_draw_lists 87 名/85 叶分发表、water/ice surface 完整接线
- 遗留：Flower03Ssbo NAME、PassResources::new_from(&World) typed gap、各 *Ssbo 字段布局

### wall（431 业务函数，6 子任务并行）
- 83 章全量算法级 + wall/ALGORITHMS.md 835 行（三大流总图 + 40+ 常量）
- 关键闭合：WallSpatialHash 全族、ShapeParameters 28B 位 0 判别、snapping 全阈值、DecoratorStorage 五集合、高度钳制数值全解
- 7 项关键纠错（calc_wall_bend 末端段、structure_outline AppMode 门控等）
- 遗留 U 38（含 35 胶水）：publish 逐字段复制映射、Cell 中心浮点、简化曲线布局

### 六新模块（asset_libraries/components/utils/startup/input/geometry）
- 各 ALGORITHMS.md 全量（FrozenMap 语义、load_json_as_mesh 全流程、组件 required 映射 57 项、startup 入库链、动作集/录制环、instanced_wall 上传）
- 未探究闭合 2/3 完成（utils+startup ✅、components ✅；asset_libraries/input+geometry 待收）

### 其余域（均算法级全量）
- collision-eco：flush 链/legacy_spherecast 裁决/ivy grower 10 相
- audio-misc：MusicPlayer 四状态机/Steam 22 块
- ui-input：mode_manager 跳表/ui_edit_focus 24 项表
- history：history_process 42KB 全文/回放族
- session-save：save_thread 主循环/mark_dirty 4KB 位图
- camera-time：rotate_yaw_pitch/zoom/camera_snapshot
- terrain-water：WorldRaster 写/上传链/脏位图修正
- roofs-rt：BVH/屋顶常量更正
- backend-misc：flush_screen_log 环/prepare_frame_constants

## 质量保障

- O 抽查两轮：round1 40 条（80%）+ round2 225 条（95.1%），修正 93 行/30 文件
- 5 组 review 未探究清单（MERGED.md）
- 12 份 ALGORITHMS.md 全部落盘

## 未闭合项（全局）

1. 证据引用卫生：65 条章节 evid 悬空引用 + wall 137/469 行引 NotFound 文件（需批量整修）
2. 跨域裁决：MainActionSet 双形态、Events fork 双布局、InputFrame 偏移、transient_meshes（已裁决）、FrozenMap insert 集成
3. 剩余 U 卡点：SubworldIndex 槽名、Flower03Ssbo NAME、EscMenuState 变体名、PhotoMode→CameraMode 映射等（各 CLOSURE 已列）
