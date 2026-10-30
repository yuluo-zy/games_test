# M01 装饰交互派发与转换杂物入口恢复

## 范围和原身份

本轮沿用B目录 `crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs`，恢复 `ui_dispatch_decorator_affordance` 原 `0x1407af340`。B PDB模块425给出的过程长度9340字节；主业务及panic至 `0x1407b1417`，其后为异常清理。没有创建替代dispatcher或第二碰撞世界。

原RNE、PDB、主代码窗口与保留证据的SHA见 [identity](m01-dispatch-identity.json)。全文反编译和原字节反汇编分别为 [C](m01-dispatch-full.c)、[ASM](m01-dispatch-full.asm)。明确区分原AppMode和dispatcher的两个enum；旧实现把两者数值混用，五参数系统不能读取真实StartPlace事件，且把header第二word当成widget/id。

## 原始数据流和恢复结果

原系统十四组参数已落实为：AppMode、当前RaycastWorld、CameraCursorRay、Dispatcher、UiState、CursorTerrainRaycast、UiSignifierStream、六项借用tuple（Storage/Library/Terrain/PublicWalls/Stairs/Cache）、EscMenu、PrevBlueprints、Modifiers、四项event tuple（StartPlace/Move/Rotate/Notify）、LastClonedClutter、同一Stairs资源的第二只读参数。宿主raycasting拆分使用同World当前碰撞与地形，不克隆物理世界。

1. Esc直接返回，失活AppMode清空；允许模式为0/3/4/11/20。只允许的模式注册真实 `ui_decide_decorator_interaction`，距离f32::MAX、空音效、遮挡关闭。
2. StartPlace事件按跨两个buffer的最新未读值处理，优先于旧拖动及Move/Rotate。原96B保留相对四元数、widget、完整命中和ClutterMeshTy；事件步长112B的尾部EventId不混入业务载荷。原hit有60活跃字节及4字节不活跃padding，当前安全RaycastWorldResult多出的toi语义字段不能裸transmute。
3. 活跃拖动保持widget/id/type/原flag，真实射线刷新hit和距离-0.05；dragged()==2时回到发现阶段。CloneClutter还保留此模式的强制续帧条件，使用LastClonedClutter原四元数和真实类型，距离-0.01。
4. mode3按Move优先Rotate读取，并保留下优先级reader；mode4先读Rotate，再发现碰撞。
5. 全17种原碰撞来源switch已按原跳表恢复：普通表面放置、光标回退、装饰移动/旋转、地形灯笼/旗的旋转包围盒、烟囱高度包围盒、楼梯连接/继续/高度和原不可达panic。编辑楼梯只接受原图可达节点，并保留Notify事件及度数限制。
6. Dispatcher活跃变体为0普通放置、1杂物放置、2移动、3旋转、4高度、5楼梯继续、6空；owned getters返回真实载荷。原构造14034362c/3637及insert_resource140139690已证明Default=(6,0,None)。Move事件reader的+5d标志为0；发现移动标志为type!=63。生产只注册完整系统，文件中的窄focused入口仅供旧旋转差分测试，不得双注册。

交叉consumer复核补充：普通0的最终原发布1407b11ed..1407b124b同样把hit写+20、widget写+60、flag写+68=0、type写+69；此前向owner转述其widget+10/hit+18是错误的偏移结论，已撤回。普通0的+10..1f有共同块store，不证明活跃Quat；StartPlace1才有已证的相对Quat。Rust普通Place owned载荷保留widget/hit/type，消费者按普通writer取flag=false，不创造相对Quat。height owner已沿消费者确认普通分支identity旋转、杂物分支真实相对旋转，公共存储位置不能代替变体活跃字段判定。

上游rectangle owner恢复真实13参转换生产者，使用同一个typed StartPlace事件。相对四元数为 `clutter_rotation * camera_xz_rotation.conjugate()`，OnBuilding特定分支法线为q*Z；reader原值保留，不重复归一化或换乘法顺序。下游放置、移动、历史提交由对应owner继续同一业务链，本报告不把dispatcher的无历史副作用等同于完整放置历史验收。

## 灯笼原unit key与真实资产

原key Debug1407af060在tag0x1a分支输出22字符 `LanternTerrainRotation`（142a834e2）。原HashMap查找1407af100只比较该tag，不把第二字节不活跃padding当rank。保留宿主key API，以unit const和手工Eq/Hash表示；26/0是宿主carrier，不宣称第27个普通Subtype。

原load_decorators1407b66e0的writer在subtype18普通插入之后：1407b8e90..8f43加载43字符 `decorators/lantern_terrain_rotate_collision`，构造真实indexed collision、None主outline、同资产lines，再通过140b25ef0插入既有四map。不是在末尾补空BBX。Demo也需要此资产，缺失按原完成此前prefix后失败。full正常key52+unit=53；Demo34+unit=35。原[loader ASM](m01-dispatch-loader-original.asm)保留完整顺序。

## 差分与真实World验证

| 验证 | 原侧执行及覆盖 | Rust结果 |
|---|---|---|
| StartPlace原入口 | 339组，63接受/276模式或Esc拒绝；最新event、两buffer、已读cursor、非单位非共轴/NaN四元数位型 | 291个安全AppMode可表示输入逐字段一致；非法raw tag保留原证据不虚构Rust变体 |
| 空事件发现mask | 84组，从原入口执行至1408cc8a0调用前截获，不桥接mask决策 | 全一致；mode4=0x5000、原type63 base0x23、其它base0x30ab、100/4000组合 |
| 烟囱几何 | 90组原1407b08a5..097a，实际执行140c98380构造/140c985a0 contains | 八个几何float位型及contains全一致，含旋转轴、边界、大坐标 |
| 距离 | 512组原1407b0d1d..0d3d算术 | bit一致，保留sqrt(z²+(y²+x²))求和顺序 |
| 楼梯高度 | 162组原terrain/roof FP块、UCOMISS/JBE | 包含NaN、无穷、严格边界全一致；NaN为false |
| 真实Rapier多帧 | 非空slot8装饰实体悬停→修饰键旋转→真实remove | widget/id/hit/type/音效/距离一致；卸载后clear、Arc计数回到1 |
| loader | 7项普通测试，2项原B资产测试 | full53/demo35全部四map含真实几何/lines；共享Arc和独立tri Vec、缺失prefix顺序成立 |
| unit key | 256个第二字节padding | Eq/Hash/查表同unit，和正常0..25/rank1..3不冲突 |

脚本为 [原reader](m01_dispatch_start_place_native.py)、[mask](m01_dispatch_mask_native.py)、[几何](m01_dispatch_geometry_native.py)，对应TSV/JSON同目录。脚本以全RNE SHA拒绝错版本，机器码执行仅桥接Esc/UI注册等依赖；几何脚本无任何算术/contains桥接。运行命令使用 `/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/<script.py>`；Unicorn JIT需沙箱外隔离执行授权，未启动Windows程序或操作OS输入。

Rust命令：

```sh
CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline ui_decorator_affordance_dispatch -- --nocapture
CARGO_INCREMENTAL=0 cargo test -p country-core --lib --features native-physics --offline lantern_rotation_unit_key_tests -- --nocapture
CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline load_decorators::tests -- --nocapture
TG_INSTALL_ROOT='/Users/yuluo/Desktop/game/B版本内容' CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline load_decorators::tests::original_b_asset_tree_ -- --ignored --nocapture
```

本轮不声称真实鼠标/键盘或画面观感已验收；用户此前暂停真实输入，GPU联合验证由窗口owner执行。下游完整放置/历史回滚由该系统owner联合验证，本轮派发系统不擅自提交历史。

最终 [v5日志](m01-dispatch-world-tests-v5.log) 为 **11通过、0失败**：另补真实terrain Clone→held drag→pending Move（非单位非共轴q位型、原widget/type/flag和两种距离offset保持），以及非空Rapier source5/slot14楼梯高度→hitY+1严格边界→继续→modifier/度数2拒绝→Stairs工具跳过不可继续节点。Notify事件和真实graph/cache参与，未用虚构可继续布尔资源。country-core unit key测试1/1，loader普通7/7（另2项环境相关ignored已显式用原B资产运行并2/2通过）。

## 本轮可复用取证经验

- shared Ghidra原body只有1217字节、5KB输出，不能因反编译正常返回就信为完整过程。PDB长度、原return、所有跳表和反汇编闭环必须交叉核对。本轮只在独立worker修body/JumpTable overrides，shared数据库只读。
- 第三跳表被两个jump共用，实际17项；试读20项会把下一段数据误当目标并触发坏指令。先验枚举范围和原bounds-check再限定表项，不能靠反编译出现的case数填补。最终原输出46KB，无坏指令/未恢复跳表。最终[修复job](m01-dispatch-body-repair-job.json)和[结果](m01-dispatch-body-repair-result.json)保留，附原字节窗口SHA。
- 相对旋转用非共轴输入；identity会掩盖乘法顺序。几何浮点要保留求和/加法顺序，不能用通常等价的Vec3.length或重结合替代bit-sensitive路径。
- 原call的栈偏移/完整union活跃字段优先于现宿主repr；不能把hit尾padding、enumheader第二word或native key inactive byte变成额外业务值。
- 上下游共享typed事件和owned载荷，同一Resource身份；先证消费者和真实资产writer，再补接线。真实Rapier测试应包含非空命中与卸载引用回收，空World通过不能证明过滤mask正确。
- 失败日志保留而不抹平：早期类型导入/WallShape无Default、原hit padding断言假设均已修；最后共享apply_decorator_op.rs的WallId导入中间态阻断由其owner修复，不跨owner覆盖。
