# M01 装饰移动的墙体拆分分支

## 结论与范围

已在B原归属 `crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs` 替换零位置及空处理器，恢复完整picker、rank拆分、锚点、临时ID、归档、权威存储、历史队列、事件原载荷及动态指示符。最后定向测试 **5通过、0失败**，包括全部252组真实handler的ID/rank/事件顺序；日志 [v8](m01-split-native-tests-v8.log)，实现及证据SHA见 [identity](m01-split-identity.json)。

主 `ui_move_decorator` 由move owner独占，已交接typed API和调用条件。这里的真实World测试验证helper与原九资源backup的多帧预览/拒绝回滚、正式ID消耗及卸载引用；不能代替主系统释放输入、History提交/Undo、视觉重建或GPU窗口的联合验收。未操作OS输入，未修改渲染框架，未提交Git。

## 原函数和纠错

| 原地址 | 原职责 | 当前归属 |
|---|---|---|
| 1412782b0 | 完整墙装饰部件picker，原调用1407a566c | 同B文件同名函数 |
| 141278ed0/141279090/141279250 | 三种真实temp_sphere工厂 | 同B文件拥有型factory，原消费者负责创建/动画/回收 |
| 141279410 | 移出选中墙部件，原调用1407a60f8 | 同B文件同名typed handler |
| 14127a480 | 坐标投影、相对高度、cache及可选Move目的 | 同B文件split_anchor/split_anchor_in |
| 14125cf90 | std HashMap<DecoratorLibKey,DecoratorCollision>::get | 复用真实mesh library；不是位置算法 |

原C和全范围ASM保留于本目录 `m01-split-<address>.c`、`m01-split-display-original.asm`、`m01-split-move-original.asm`、`m01-split-factories-original.asm`。共享Ghidra只读，没有修改其函数body或跳表。实现前实际查看了B skeleton同路径、PDB候选调用点和原RNE；旧桩的标签有误，不能当原契约。

旧桩有四项实质错误：遍历rank-1而非rank、把箭缝':'垂直分支当panic、最近位置恒0、移出处理为空。原返回不是Option<u8>，而是usize，默认索引0且初始距离100；三件组合中间部件仅显示不可选。原碰撞库尺寸来自当前ID的真实derived subtype和完整rank，不由Info.word2c反推ID。

## picker与画面生命周期

原七项语义输入为缓存pose、完整hit、Info、Blueprint当前visual pose、DerivedInfo两owner视图、独立ID、UiSignifierStream。缓存pose旋转Z取得装饰平面法向，hit投影到该平面后比较完整rank的部件位置；visual pose的旋转、非均匀缩放和平移参与定位。不存在camera位置或固定屏幕阈值替代。

保留原求值顺序：部件归一化序号、尺寸和 `(1-1/rank)` 的乘法顺序，以及按列先scale再x/y/z相加。普通窗沿X，箭缝沿Y。真实factory名为 `outline_split_<part>_<id>` 和 `outline_split_<best_position Debug>_<id>`；mesh为temp_sphere，动画0.1秒，中间/普通/选中尺度分别0.05/0.2/0.35。每个记录持有Arc闭包；测试实际唤醒builder、检查实体Transform和SignifierAnimation并despawn，没有使用不能唤醒的旧数值record。

调用条件由主move保持：rank!=1、真实Modifiers.disable_snapping及原目的分支；1408c5680的ICF别名不证明此处是StructureType::is_fence。主move owner独立复核caller输入归属，避免只按反编译符号猜接口。

## rank、ID和锚点业务

未附着墙角时，按选中位置拆出左右余块；每个余块读取其自身rank碰撞尺寸。普通窗以完整尺寸减余块尺寸再乘0.5/方向的原顺序计算偏移；箭缝余块原tag0不激活offset，不能按视觉对称性擅自移动。选中原子部件使用rank1真实尺寸与原 `ratio+ratio-1` 因子得到独立offset。

附着墙角时，余块保持原坐标，只减少rank；选中原子部件也使用原无偏移分支。原第一余块复用旧ID，第二余块（存在时）生成新ID，最后移出原子部件始终生成新ID。返回仅 `{moving_id, remaining_ids}`；原sret+18是Vec长度。主move在之后读取新ID缓存并按剩余列表重算抓取，不得在返回结构中制造Option坐标。

锚点按原顺序验证墙地址→PublicWall→源墙坐标→WallSpace投影→attachment height→世界位置/切线→xz_rotation_bad→缓存insert。不偏移仍会投影并刷新cache，但不给权威Info写Move目的。激活偏移时保留platform和bottom_as_door，清corner_attachment，并保留Wall/WallCompound类别；相对高度由新y减真实attachment height取得。复用原已恢复WallSpace、曲线和高度算法，不写局部圆/矩形近似。

临时生成器复用move owner新恢复的借用型 `TransientDecoratorIdGen<'_>`：预览不消耗正式ID，提交直写同一个正式计数器，不能复制counter、reserve补洞或保存裸指针。

## 归档、历史与事件

每一步缓存后先归档真实资源，再应用change/place，使用同一个LiveArchivist、Storage、DeferredHistory及DeferredDecoratorOpEvents。原首次快照规则同时保留两张真实物理碰撞map；重复预览不得覆盖初始before。原完整72B源Info先自有复制，后续新部件从该记录恢复，仅替换原激活的目的/rank/独立新ID/demo门字段，不用后续被改rank的记录当模板。

重要纠错：141276e30第三参不能一律命名为world position。split的Change传anchor输出的首16B（有Move时是地址；NoMove时tag3，其余inactive），Place传 `{新ID u32,offset tag u32,offset x f32,offset y f32}`。原apply复制该16B到事件+70..80。本轮由rectangle owner在共享 `apply_decorator_op.rs` 落 `from_change_with_event_data` 并令writer读取唯一op.extra16；墙处理器完整保留defined raw，不额外复制事件队列，也不把地址位型当世界坐标。inactive字节安全初始化不声称原ambient stack固定为0。

state参数仍是原apply独立字节，demo另取唯一DevOverrideIsDemoMode（同DemoModeEnabled TypeId）资源/config.dev.demo，不能从state猜demo。handler只写原延迟历史，释放输入/提交决定留给主move；局部预览不能提前pending或提交。

## 验证范围与命令

| 项目 | 原侧执行 | Rust验证 |
|---|---|---|
| 拆分计划 | 完整141279410入口252组，540个descriptor/操作，覆盖56/57/58、rank2..5、全部部件、附着/compound | rank/可选offset位型/ID和顺序匹配；原容器桥接，不手写原计划 |
| picker | 完整1412782b0入口288组，1296个factory位置，非共轴旋转、非均匀scale、大坐标、不同hit | 选择和每个实体位置bit一致，真实factory动画/scale、创建/销毁一致 |
| anchor | 完整14127a480控制流144组，136接受、8原拒绝 | WallSpace投影、rotation、cache10float、目的字段位型一致，拒绝无成功输出 |
| 非空World | 实际公开墙、真实Rapier双map、归档、九资源backup及history/event队列 | 合法rank3端部拆分→拒绝回滚→预览ID重用→正式ID消耗，72B额外字段保持；释放World后mesh Arc回到1 |
| 失败顺序 | 原lookup/ID/anchor顺序 | 缺Info先于ID；错误源地址在生成新ID后失败，未写历史、事件或cache |

原252组中rank4/5是函数级探针，不表示游戏工具允许五件组合；原窗族max_rank=3且三件中间件不可选。原函数的第二余块分支仍完整保留，不能因为当前picker不产生某输入就删除算法。后续在同一真实World直接循环执行全部252组handler，比对原moving/remaining IDs、每条事件独立ID/rank、历史顺序及预览计数器；不是另写一个Rust ID模拟器自证。

原脚本为 [plan](m01_split_plan_native.py)、[picker](m01_split_display_native.py)、[anchor](m01_split_anchor_native.py)，JSON/TSV同名保留。三者均以完整RNE SHA拒绝错版，直接执行原机器码；不用Rust实现反向生成oracle。plan将anchor bridge为NoMove，仅验证计划/ID/操作序，不声称该次执行完整坐标；anchor另外执行原控制流、project和xz_rotation_bad，显式桥接已恢复曲线、attachment height及容器依赖，CRT fmodf import使用同f32库语义。picker仅桥接容器、格式化和UI注册，FP与选择分支原样执行。

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_split_plan_native.py
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_split_display_native.py
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_split_anchor_native.py
CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline handle_splitting_wall_decorators::native_tests -- --nocapture
```

最后命令5/5通过；v8日志中的panic来自8个原拒绝和两个失败序测试的预期catch_unwind，并非测试失败。只格式化独占文件，没有全仓fmt或新target；`git diff --check`通过。`cargo check -p system-decorator --lib --offline`此前通过；之后共享main重构期间的编译中间态已由各owner修复，保留v1..v8失败/成功日志，不把被阻断的执行计入通过数。

## 本轮经验

- 源码旧注释的原VA可能是ICF容器方法；先查callee真实body/调用参数再设计helper，不能从名字恢复一个不存在的位置算法。
- 不能从同一调用者的多数分支推定所有参数都是Vec3。原union有inactive字节，先锁定每条writer与consumer定义区，再提供owned raw通道；保留16B不意味着声称统一ABI或语义。
- caller后的Vec长度与另一个分支的Option必须逐指令分开。主move和本owner分别复核60fd..617e，撤回过早的“缺Option返回”推断，没有往schema添假字段。
- 对可疑竖向/附着算法，执行完整原控制流再裁定，不凭几何直觉“修正”原逻辑；native plan表明确只有指定offset tag才激活字段。
- 隔离执行RNE时可能在未装载的IAT取到低RVA，不能误判业务坏跳转。本次loop probe的0x3195524指向原fmodf import名称，明确桥接标准CRT thunk142923f10后继续执行原project控制流；没有绕过loop分支或重启共享后端。
- 先在非空多帧World验证authority、双物理归档、临时ID及拒绝回滚，再交主owner联合输入/History/GPU验收。局部5项测试通过不等于整个M01验收。
