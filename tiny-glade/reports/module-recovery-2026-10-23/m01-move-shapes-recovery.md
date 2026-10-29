# M01 预制建筑移动与共用移动手柄（2026-10-26）

## 范围与原版身份

恢复位置保持 B 目录：`crates/country-core/src/systems/ui_move_shapes.rs`。之前生产目录缺少这个原 owner。本轮保留现有手工重构的资源、glam、Bevy、装饰归档、历史、碰撞与指示符实现，没有另建平行游戏对象。

原发布 RNE SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
分析 EXE SHA256：`cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。
PDB SHA256：`2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
每个原函数的 PDB 字节数及 RNE 函数字节 SHA 见 `m01-move-native-identity.json`。

| 原函数 | VA | PDB 字节数 | 当前归属 |
|---|---|---:|---|
| ui_move_shapes | 14097a060 | 12822 | 本文件实际 Bevy system |
| move_gizmo_and_trigger_vol | 14097d280 | 2618 | 本文件共用 typed helper |
| 手柄地面虚线闭包 | 14097dcc0 | 1488 | helper 持有的真实 SignifierFactory |
| 手柄填充/描边闭包 | 14097e290 | 709 | helper 持有的真实 SignifierFactory |
| Rapier::raycast_simple | 1408ce180 | 697 | 现 collision/world.rs 同一个查询核心 |
| raycast_simple predicate | 1408ca9a0 | 153 | 查询核心的真实 registry 过滤闭包 |

## 反编译修复与证据边界

旧 `decomp-0x14097a060.txt` 和共享库的新读取均把错误跳表分支延伸进 create_circle 等邻函数。Ghidra 甚至报告 body 仅 3213 个离散字节，C 却超过 100KB；这种输出不能证明业务边界。

复用自由墙 owner 已验证的修复方法，在独立 ReVa worker `m01-move-20261026` 中执行：核对原函数字节 SHA；按 PDB `97a060..97d276` 重建 body 并剔除既有邻函数；从原 PE 相对表 `142ab7514` 读取四项；在间接跳转 `14097ac59` 写 JumpTable override。四目标分别为 `14097c31e`、`14097ac5b`、`14097ae9d`、`14097af78`。修复后的 C 已保存为 `m01-move-shapes-repaired.c`，任务是 `m01-move-shapes-repair-job.json`。未修改 shared 或其他 worker。正常停止专属backend后已重开核验：body10677、四case0/1/2/3、manual override均仍存在，见 `m01-move-worker-reopen.json`；结束时再次正常关闭自己的backend释放内存。

重建的 Ghidra body 有 10677 个归属字节；12822 是 PDB 物理过程跨度，含异常处理与已单独归属的函数片段。两者不能互换作为业务覆盖率。

## 真实输入、算法与副作用

主系统保留原 15 组 SystemParam、30 项资源/事件。现宿主额外用同组 `Option<Res<DemoModeEnabled>>` 承接原进程全局门禁，与输入和边界共用一个 owner；无配置时沿用既有 B 发布入口的 false。

模式只接受聚焦 Wall/Roof；查验公开墙并拒绝非预制形状。支持移除过的建筑注册 `add_support_{wall}` 球体，位置为 `max(elevation, terrain)-2`，点击恢复支持，记录 ChangeSupports 并发原墙体 UI 音效。此入口没有外部位置事件或模拟点击代理。

共用手柄八参数为：`position, orientation, actions, ui, wall, published_state, ground_y, stream`，返回真实 `UiWidgetContext`。自由墙 owner 直接复用该函数。

手柄缩放按源 Curve2 长度：`t=clamp((length-.5)/2.5)`，先计算 `t*t`，再算 `(3-(t+t))*(t*t)`，最后 `(1-smooth)*.5+smooth`。保留 SSE NaN/负零行为。拾取盒是**宽深 1.5*scale、固定高度 .5**；不能把 XMM2 的 .5 误读成第三个相同尺寸。拾取盒不随悬停放大；显示网格活动时放大 1.2，网格 scale 再乘 1.5。

注册 `move_shape{wall}` 的真实 TriangleMesh，按 camera_rotate_mode 的按键状态控制四声音槽。elevation>0 时创建地面竖虚线及真实 `DashedLineLength`。填充与描边使用原 `move_shape`/`move_shape_outline` 两个具名资源、原 rotation/position、零时长动画与不同 UI pass；资源缺失交由现 builder 报错，不生成空网格替代。

拖拽开始时归档墙和地面装饰及活碰撞载荷，设置剔除许可，保存墙附着装饰的原位置相对建筑中心差值、初始形状、抓取平面高度和射线抓取差。移动帧从权威 InnerWalls 取完整源，悬空时投影相机射线到原抓取平面，地面建筑取实际 CursorTerrainRaycast。

屋顶吸附使用原 `0x202` 槽掩码、向下射线、排除当前 WallId、被命中建筑的闭区间竖直相交，再复用原 snap_to_roof_ridge。点/线段分支都有完整指示线；吸附新进入才发声音。禁用吸附用现真实 Modifiers，不新建简化输入。

每移动帧先发布原区域 pins 与树隐藏请求，再校验完整形状边界。矩形谷地复用原20步形状插值搜索；圆形谷地保留圆半径裁剪，以及矩形面对目标边的两端点沿同一单位方向求圆交、取最小退出距离、100速率连续平滑。越界时发布 display/feedback，应用原约束后的结果，且保留原本帧不发移动音频的门。

写回前发布真正 `OnShapeMoved {affected,wall,offset}`，然后一次性更新完整 WallState。圆重新离散参数为 `.5`，矩形复用自身算法；height/elevation/arch/style/support 等其他源数据保留，metadata69 读取真实共用 demo policy 写0/5。

拖拽结束发布最后装饰移动事件、羊检查、MorphPremadeShape 复合历史和 finished 树请求。音频只在实际 AudioCursorMoved 到来时读取，speed 原 inverse_lerp(0,250,speed) 再保留原 SSE 范围门；ShapeIntersections 只依据真实配对 key 决定额外建筑音频，不伪造碰撞。

## 两个关键原指令纠错

### 高度不能从最近的 terrain call 猜测

入口 `97a966 sample_at_world_xz` → `97a96b xmm12` 是手柄的真实 ground_y，经 `97ac03` 传 helper。然而 held 分支的 `97b70b movss -0x1c(%rcx,%r10),%xmm12` 重新读取权威源 +54，覆盖该地形值。最后 `97c2c2` 写完整新 WallState+54 的是**原建筑 height**，不会把建筑压成地形高度。判断必须贯穿 CFG 追非易失 XMM 的覆盖。

### 移动与历史重放的圆采样本来不同

移动 `97c237` 读 `142925870=0x3f000000(.5)`，`97c23f` 调 Circle2d::as_curve2；原重放 `140ac01ff` 读 `1429cd068=0x3e4ccccd(.2)`，`140ac0207` 调同函数。现 replay `Shape::as_curve2(.2)` 保留了原更密离散；不能为了让测试曲线点数相等而改成 .5。验收比较业务形状与高度，并保留这项原分支差异。

## 碰撞接口修复

原移动调用 `97c462..97c496` 是 sret、world、ray、Some排除墙、wallId、MAX_TOI、mask0x202。原 callee 将排除载荷装入 predicate 环境，原 predicate 常量 `0x107` 只包含 kind0/1/2/8；对应 source+8 WallId 相等才过滤。其他类型保留。过滤前仍走真实 mesh receipt/generation 校验。

当前 collision/world.rs 新增 `raycast_simple_excluding_wall_id(...,WallId)`，复用原有17槽 Rapier 核心，恢复 predicate，并为 None 调用同一个核心。无 native-physics 的该建筑吸附入口显式失败，不以假未命中掩盖生产缺能力。

范围外遗留：旧 `raycast_simple_exclude_wall_id(...,SubworldColliderHandle)` 兼容入口仍被 wall-constructor/rectangle_pillars 的旧签名使用。其历史签名把 WallId 错建成物理句柄，需要建筑/支撑 owner 独立核原并迁移；M01 新链不再走该兼容入口。不能据本轮判整个支撑业务已还原。

## 验证状态

1. `m01-move-gizmo-native.py` 在原 RNE 执行 `14097d2c4..14097d326` 的实际 SSE。146个边界/随机/NaN/负零向量已落 JSON；从生产源码直接提取的 Rust 函数逐 bits 全部一致。
2. 已加入真实 World、真实 TriangleMesh registry、hover_intersection、ui_update、真实 digital 输入及 history start/finish 的多帧测试，覆盖圆/矩形、固定拾取盒高度、权威源移动、墙高保留、装饰归档释放、羊/树事件、MorphPremadeShape、真实 replay undo/redo 与退出捕获 Arc 释放。
3. 真实 native-physics collider 测试已通过：17个source kind各做 None/排除同墙/排除其他墙三次查询，共51个真实物理查询；与原153B predicate的51个实际native执行分支交叉验证。原过滤集合严格为0/1/2/8。
4. 最终 `CARGO_INCREMENTAL=0 cargo test -p country-core --features native-physics --lib --offline ui_move_shapes::tests -- --nocapture`：3 passed /0 failed /0 ignored。包含真实恢复支撑按钮、悬空手柄三工厂实体、UiPass/scale/虚线长度，以及真正mark→反向动画→despawn→TransientMesh登记→mesh/VAO清理。最后一次修改还去掉先clone旧曲线再覆盖的多余分配，并重新通过全部3测。随后同一native-physics测试二进制运行碰撞过滤test：1 passed /0 failed /0 ignored。命令、源码SHA和cfg边界见 `m01-move-validation.json`，完整日志为 `m01-move-final-tests.log` / `m01-move-final-collision-tests.log`。早期磁盘耗尽与并行类型中间态编译失败没有冒充通过。
5. 不执行 OS 输入；本轮 GPU/窗口联合验收由中央与 GPU owner 执行，不能把本文件的 CPU 测试替代画面验收。

## 下一轮可复用经验

- PDB void() 不是真实 ABI。手柄实际八参数且返回 RAX/RDX pair；错误签名会漏掉地形高度、stream 与返回 context。
- SIMD 寄存器按 Windows x64 参数位置分配。box_trigger_volume 的宽/高/深分别位于 XMM1/2/3，XMM2保持.5而 XMM3复制 XMM1。
- 修坏 CFG 时先核原字节、物理范围和相对表，再在专属 worker override；不得接受反编译器生成的十几项非法 switch。
- 类型名相同不代表同一资源。原 OnShapeMoved 与 pins union 需要共同 typed owner；不能往旧 kind/Vec/show 占位事件填默认值。
- 模块收口必须追被调函数。调用名字带 exclude 不证明排除已实现，原真实 predicate 在 vtable 中，需要检查环境载荷。
- 精确原业务允许不同路径有不同离散精度。先查两个原调用点，再决定差异是真缺口还是原版设计。
