# M01 圆形编辑：原算法、交互、历史与实体生命周期核验

本轮在既有 B 目录内替换早期五参数事件适配，恢复半径编辑、旋转、动态圆弧/箭头及圆墙装饰消费。不是以函数数量、旧统计或旧 fixture 认定完成。共享调度与资源注册由中央 owner 修改；本文只记录本链证据和边界。

## 原目录与函数对应

编辑前实际检查了 `reverse-engineering/tiny-glade/skeleton/B/crates`。下列五个生产文件均沿用原路径，没有新增生产目录或换回旧渲染框架。

| 原 B 文件/符号 | 原 VA 或归属 | 当前业务落点 |
| --- | --- | --- |
| `country-core/src/systems/wall/shapes/circle/ui_edit_circle.rs::ui_edit_circle` | `14165d460`, PDB module 1640, 原贡献 12691B | 原 13 组、30 展开 SystemParam；注册、选中/未选中、抓取、投影、拖动、释放 |
| 同文件 `get_trigger_vol` | PDB inlinee `12418`，原行607 | 未选中入口从真实 WallEntities/Query 取 WallTriggerVolume，并调用原编辑参数 get_tris；无效体生成 Empty |
| 同文件 `update_circle_radius` | PDB inline，原 UI 物理函数内 | phase4 的投影误差、上下限、时间平滑、边界、变更阈值、权威 curve/enclosure、音频、树与 pins |
| 同文件 `finish_circle_edit` | PDB inlinee `1241d`，原行463 | 释放分支的阈值、归档恢复、墙/复合墙装饰重贴、Concurrent MorphPremadeShape、提示与羊检查 |
| 同文件 `show_full_circle_signifier` 及两层闭包 | `141660880`, `141660b30` | 拥有名称与闭包的生成网格 factory，完整圆采样0.5、指定高度与线宽 |
| 同文件 focused arrow 闭包 | `141660600` | 原资产 `brush_arrow_flat_single_foward`、旋转/偏移、scale1.5、duration0 |
| `circle/ui_rotate_circle.rs::ui_rotate_circle`、闭包 | `1409ac910`, PDB module527, 6847B；factory `1409ae3d0` | 原16组、21展开参数；投影轮廓左端、盒拾取、旋转、屋顶/装饰事件、历史、回位插值与音频 |
| `circle/signifiers/edit_arrow.rs` 系统与闭包 | `140b35120` | 活动资源驱动 `flat_arrow` factory，scale2、duration0；箭头和圆弧实体同步变换 |
| `circle/signifiers/edit_line.rs` 系统与闭包 | `140a64a40` | 半径+0.23、局部u=0.75、半弧宽0.01→0.1、采样0.2、duration0.3、真实可变网格 |
| `resources/walls/wall_trigger_volume.rs::get_tris` | `1408e80f0` | 原 quad 裁剪、端点高度、法线偏移、特殊顶面分支和三角绕序 |
| `systems/decorator/src/decorator_on_freehand_wall_edited.rs::decorator_on_circle_edited` | `14081c060`, 690B | 同一真实 OnCircleRotated 身份；墙/复合墙坐标更新及 modified，Roof 跳过，其余非法分支 panic |

PDB 的 default、Bevy 注册/clone 派生符号由 Rust Default/Component 派生和真实 ECS 组件注册承担，不恢复编译器内部容器算法。三个 inline 业务 helper 的原逻辑明确对应上表相位，没有以同名空函数伪装恢复。

## 业务与数据流

1. `Editing(None)` 从真实墙三角体注册原 Widget；`Editing(Wall/Roof)` 以半径圆周 SphereCloud 注册聚焦 Widget。相机变化只抑制四种 UI 声音，不关闭历史。
2. 真实 picker 与 ui_update 产生 hover/3/4/释放字节。开始抓取保存首次圆与屏幕投影误差，归档 Wall+Terrain 并开放原剔除许可。
3. 半径拖动投影到固定径向射线，限幅至原位模式 `0x3f666667..8`，按 GameTime delta、速率10平滑，复用原边界扫描。超过 `delta*0.5` 才更新权威轮廓/形状；材料音、建筑音、树隐藏及 pins 使用各自原字段。
4. 释放半径变化超过0.01才重贴装饰并记录一组 Concurrent MorphPremadeShape；其余恢复 Wall/Terrain 归档。树释放通知仍发出。原 UCOMISS/JBE 的无序比较也进入恢复分支。
5. 旋转使用公共墙原 `+0x40` 地形映射轮廓，逐点进行两级相机矩阵投影，不能替换成世界X极值或加工后的路径 `+0x88`。原盒参数3.5×0.5×1.5、径向+1.0再+0.3完整保留。
6. 旋转开始归档 Wall；拖动更新权威圆和原采样轮廓，有屋顶时发 RecalculateRoofCmd(kind2)。每个 held 和 release 都发 OnCircleRotated，实际装饰 consumer 直接借权威 Curve2 更新 anchor，避免逐装饰 clone/重采样。释放记录历史；角手柄回位持续发34，终点发一次35。
7. 未选中半径 hover 的 AnimatedCircleSignifierResource 使用末尾 Option<bool>，None只表示不续租。动态箭头/弧采用真实 factory 和资产；弧每帧写入网格并保留退出后的反向动画，随后 despawn 和 TransientMesh 队列释放。

## 身份与独立原指令验证

- 原 RNE：`B版本内容/tiny-glade.rne`，SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 对照修补 EXE：SHA256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`（实际原始指令 oracle 只运行上面的 RNE）。
- B PDB：`2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
- `m01_circle_native.py` 映射有 SHA 断言的 PE，直接运行 x64 原指令。仅桥接分配/释放/memcpy/panic终止；不把恢复后的 Rust 当 oracle。
- `m01-circle-trigger-native.json`：288组完整 get_tris 调用，涵盖两种特殊标记、apply、完整/局部/反向/空/越界 u 范围、法线选项及不同端高；48组按原程序 panic，其余全部三角位置逐 bit 对比。
- `m01-circle-trigger-edge-native.json`：24组原完整 get_tris 调用，补可选法线缺席/零/非零、NaN/无穷法线与NaN端高。有限值和NaN payload严格比较；仅aarch64允许 `inf*0` 新生indefinite NaN为正`7fc00000`、原x86为负`ffc00000`的已知ISA符号差，不对NaN结果做有限值回退。
- `m01-circle-rotate-gizmo-native.json`：24组原 `1409acc74` 至真实 box 调用的参数捕获，变化轮廓、中心、窗口宽度及矩阵投影，四元数/位置/scale共10个浮点逐 bit 对比。
- `m01-circle-radius-policy-native.json`：原 MAXSS/MINSS 13组含正负零、无穷、qNaN/sNaN，释放 UCOMISS/JBE 30组，逐位/分支比较。
- 完整原 UI 机器码：`m01-circle-edit-original.asm`、`m01-circle-rotate-original.asm`。

## 工具失真与可复用经验

1. **先看 PDB 物理贡献，再看 decompiler 的 body。** 原 rotate 6847B，旧 Ghidra body 截短并含 halt_baddata，不能拿截断 C 认定缺少分支。独立 worker `m01-circle-20261026` 校验6847B代码 SHA `baaa2fd49bbfd5afcc11742530dca68e5642e6cfc1612ba5a138f53fbbf56f17`，setBody 时减去其他函数已拥有地址，随后恢复 switch override。共享库没有被修改。
2. `1409ad07b` 的原四项 signed-relative 跳表位于 `142abe0a8`，目标 `1409ad64f/1409ad07d/1409ad1a3/1409ad467`，分别对应2/3/4/释放。完整脚本与修复后 C 已保存 `m01-circle-flow-repair-job.json`、`m01-circle-rotate-repaired.c`。
3. **ICF 别名不等于事件身份。** 原 `141327750` 可误标为材料音，实际 caller 写48B EventInstance，其中 payload 为 Entity+WallId+kind2，是 RecalculateRoofCmd。按写布局与真实系统参数核身份。
4. **公共 map 桶含8B WallId 前缀。** 字段基址必须扣除前缀；`+284` 已由共享 publication 写 constrained_extra，不能误报漏写后改成 authority.height；`+280` 是 elevation，pins scalar 顺序为 bottom=elevation、top=max_y。
5. **同是圆曲线也有不同采样。** 本链权威与完整圆0.5，局部动画弧0.2；不得通用替换。`+0.23`、u=.75、弧半宽.1是三个独立量。
6. **工厂 duration 不是最短寿命。** 原 builder +0x78是 duration；箭头0、圆弧.3。原 factory 名、闭包及两个组件必须保留，不能写五qword数字记录。
7. **矩阵、SSE 无序和操作数次序须直接对照。** MAXSS/MINSS右操作数NaN、UCOMISS/JBE unordered都影响业务。新增指令差分确实发现并修复释放误提交。
8. **测试也遵守历史写门与索引。** finish_recording 会把 DecoratorStorage 标为 immutable；每帧必须包含 production 的 mark_mutable 前缀，模拟下游装饰变更放在同帧 start之后/finish之前，不能在两帧之间绕开该门。恢复测试还必须维护 DecoratorToStorage 索引，不能只往桶map插入。此前两处失败均为夹具缺失真实合同，已补齐原调度与索引。

## 验证命令和验收范围

工作目录均为 `/Users/yuluo/Desktop/game/tiny-glade`，共用既有 target，`CARGO_INCREMENTAL=0`。

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_circle_native.py
CARGO_INCREMENTAL=0 cargo test -p country-core --lib circle --offline -q
CARGO_INCREMENTAL=0 cargo test -p country-core --lib circle --features native-physics --offline -q
CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib circle --offline -q
git diff --check -- crates/country-core/src/systems/wall/shapes/circle crates/country-core/src/resources/walls/wall_trigger_volume.rs crates/systems/decorator/src/decorator_on_freehand_wall_edited.rs
```

原指令输出：288组/48 panic、24组非有限触发体、24组旋转、13组限幅、30组释放分支均成功生成。country-core circle过滤的40项全部通过（含原三角未聚焦 hover、Wall/Terrain 归档恢复及非有限触发体），日志`m01-circle-core-tests.log`。native-physics特征同一过滤40项也全部通过，最终进一步覆盖6.1弧度起始、真实拖动跨2π、撤销/重做以及回位音，日志`m01-circle-native-physics-tests.log`。system-decorator circle过滤9项通过，包括共享事件真实 World 混合 Wall/Compound/Roof/缺桶验证，日志`m01-circle-decorator-tests.log`。定向diff检查通过。

真实 ECS 验证包括半径picker→多帧3/4/释放→权威publication→历史提交→原WallReplayCommand撤销/重做；旋转camera_mouse_ray→picker→UI→跨2π规范角域→屋顶及装饰事件→历史撤销/重做→回位34/35声音；箭头/弧真实资产创建、更新、0.3秒反向动画、despawn、TransientMesh队列及资产删除。未进行用户暂缓的OS鼠标键盘输入；本子任务不以CPU资产创建冒充实际GPU画面验收。

## 必须保留的共享待核边界

原 selected 但非hover 分支 `14165e1c9..14165e24c` 确实向第6组的全局 UiSignifierStream（参数+0x20，存frame+0x178）调用 show_full_circle_signifier；helper尾部直接 AppendOnlyVec.push，不是 WidgetContext 附着，未见producer guard。共享 signifier_create 在 UiState.native_tag==0时又确实不take stream。已交中央 owner 调查原清理或交换owner；不得给 circle 人为添加无hover guard，也不得把当前共享流滞留误称完成稳定性。该边界独立于上述几何/交互/历史正确性结果。
