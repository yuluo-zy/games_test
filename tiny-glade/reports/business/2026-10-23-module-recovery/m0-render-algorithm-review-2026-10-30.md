# M0 渲染与计算复杂函数复查

## 已实施修复（2026-10-30）

用户授权修复当前已确认问题。生产修改位于既有`mei/layout.rs`和`mei/systems.rs`：

- 最小尺寸的主轴从gap总量开始逐子项累加；两轴均保持原计算次序。
- 子布局的主轴／交叉轴先求padding和，再扣除；expand剩余量恢复原`(main-(gap_total+child_sum))-padding_sum`顺序（原140fe14a6..14af）。本项是在沿已确认边距差异修复同一后续计算链时一起核对的必要修正。
- 绘制采用`sort_unstable_by`并保留`f32::total_cmp`，恢复原不稳定排序策略。继续复用标准库，不复制原std算法；不宣称不同Rust版本的全部等值元素排列逐项一致。

新增3项原证数值回归，连同已有4项布局测试，直接编译当前`layout.rs`源码共7项全部通过（Rust1.97.1，0忽略）。命令为`rustc --test --edition=2024 crates/systems/toolbar/src/mei/layout.rs -L dependency=target/debug/deps --extern glam=<现有rlib> --extern serde=<现有rlib> -o D:/game/tmp-merge/m0-layout-source-tests.exe`，随后执行测试；日志`tmp-merge/m0-layout-source-tests.log`。测试使用现有构建目录的真实glam／serde依赖，无布局算法替身。覆盖主轴累加、双轴百分比可用尺寸、交叉轴尺寸及expand最终尺寸的f32位值。`git diff --check`通过。

`cargo test --offline -p toolbar --lib mei:: -- --test-threads=1`因其他构建占用共享目录锁始终未进入编译，已停止本次等待；不计整包测试通过。未启动实际GPU画面验收。摄影焦点会话边界仍按下方调查保留验证边界，滑杆交错按原规则保留；辅助业务继续跳过。当前源码SHA与日志SHA见`tmp-merge/m0-render-fix-validation.json`。

以下保留修复前调查及原证，不表示修复后的源码仍采用旧表达式。

按调查范围，只查画面、布局、相机、动画及数值计算。备份还原、保存、键盘／剪贴板接线等辅助业务暂不调查。没有把旧的“原证不足”记录批量改成已通过。

## 确认的数值差异

两处都在现有 `crates/systems/toolbar/src/mei/layout.rs`，完整布局算法存在；需要的是浮点计算顺序保真，不能描述成整个布局未实现。

1. `compute_min_size_with_children`（约309行）：原140fe0e3d..0e72先计算 `(child_count-1)*gap`，横向循环140fe1060..106e随后逐个累加子宽；当前先从0累加所有子宽，再加间距。gap=10、三个子宽均为0.1时，原累加结果bits=`41a26667`，当前源码表达式为`41a26666`。纵轴采用对应的主轴累加。这个结果随后参与最小尺寸、父级布局和后续定位。
2. `place_childen_impl`（约356行）：原140fe13c3先将两个主轴padding相加，140fe13cc再从main减去该和；当前写成`main-start_pad-end_pad`。main=100、左右padding=0.1/0.2、三个子项gap=10时，原available bits=`429f6666`，当前表达式为`429f6667`；100/33.3/33.3样本为`41566668`对`41566664`。available继续影响百分比尺寸、剩余空间和子项位置。

这两项属于可证明的数值结果变化；目前没有证明它们造成肉眼可见的像素差异，也不应升级为严重画面缺陷。按“只筛算法简化／业务缺失”的严格口径，它们应单列为低优先级数值保真修正，不能充当发现了大段简化算法的证据。

验证：`tmp-merge/m0-render-numeric-review.py`直接执行SHA为`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`的原RNE算术区间，无业务指令patch/hook。结果与当前源码表达式的逐步f32重现比较，共4个差异样本。不是当前Rust编译产物或完整布局／GPU差分。结果JSON保存原指令范围hash和当前源码hash。

## 已复查的复杂链与筛选结论

| 链 | 当前源码与原证核查 | 本轮结论 |
|---|---|---|
| MEI最小尺寸与子布局 | 两轴sum/max、绝对／百分比、padding/gap、expand剩余分配、五种justify及四种cross-align；对照`TG-19924eac9946ff87be/meihost-20261018`原汇编与合同 | 主算法存在；上列两处数值差异。没有确认省略主分支 |
| MEI根布局与绘制 | frame-events栈、逆序最小尺寸、FullWindow/Safe/WideSafe/AtPos/Relative、z和opacity、内容prepare→布局→绘制；Label/Frame对照`TG-22952476abbbac3806/meihost-20261018/CONTRACT.md` | 当前有真实文字／shape输出，没有发现可据此判定的占位绘制。不能把跨兄弟opacity累乘自行改成按父节点，因为已有原指令证据支持现有遍历累乘 |
| Image内容 | 实际纹理索引、宽高比、fit-parent、stretch-width、blur-extend、中心转origin及opacity提交 | 有实际画面算法；没有因字段名／renderer适配列修改任务。未做全图片样本像素判等 |
| Taffy锚定 | order排序、window尺寸、四方向relative尺寸／gap／offset、递归绝对坐标 | 调用真实Taffy；没有发现用固定坐标替代布局。各方向浮点全域差分未完成 |
| 摄影zoom／focus | exp2缩放与镜头上下界；lock响应、focus tag0..3、拖拽写面、view→projection两次矩阵运算、透视除法及marker | 主算法与真实GPU读回消费者存在。原首次读回及同World Reload边界仍需联合验证；本轮未确证其算法被简化，不能因使用异步readback消息就要求回退渲染架构 |
| 摄影滑杆 | 几何、1.3倍命中框、CubicOut hover、导航armed、逐drag事件、末候选参数写入、14桶声音、模式／类别门控 | 不是仅能展示的假slider；未确认缺失计算分支。grab保留与多slider事件交错仍需原侧完整时序对照，不据静态直觉修改 |
| TOD圆盘 | 两个分段映射、真实目标／实际marker、角度归一、拖拽中心奇点、30桶变化、hover与pulse | 分段与动画主体存在；历史153个原指令映射样本和50个输入样本有界，不能代替整圆盘函数判等。没有新增已确认的简化项 |
| 工具栏／undo几何与动画 | 四向激活区、button数量、边距、隐藏进度、CubicInOut位移、focus／appear、select脉冲、locked的sin(x)/x | 主公式和消费者存在；原sinc在0处没有保护，不把NaN分支擅改成1。未发现应按“未实现”列出的主算法 |

## 三条画面边界深入复查（2026-10-30续查）

### 等深度绘制：确认有可影响合成画面的排序行为差异

当前`mei/systems.rs:468`使用稳定`sort_by`，比较器是`z.total_cmp`，同深度保留输入次序。原`mei_post_update`在`14068acef`按节点数分派：少于21个走`140fe4040`插入排序，大集合走`140fe4a40`并进入`141e77860`的原不稳定排序。实际比较是变换f32位后有符号比较，与`total_cmp`对应，不能误读为普通浮点`<`。

复跑`tmp-merge/m0-three-render-boundaries.py`：直接执行原版完整排序helper及其真实内部调用，无排序替身或指令修改。对8/20/21/32/64个节点、深度0..3的固定输入测试33组，64节点seed0发现反例：同为深度0的节点56与2，原输出56→2，稳定排序为2→56。完整输入和输出保存于同名JSON。小集合及全等集合可能保持输入顺序，不能凭两个节点或全等测试推出两种算法等价。

这会传到实际消费者：`NodeContent::draw`依排序逐项调用，`tg-text::TextRenderer::rect`向primitive Vec追加，`consume`及宿主上传保留Vec顺序，UI pass以`6*prim_count`绘制；overlay使用预乘source-over混合。受支持的同深度重叠Frame/Image节点因此可改变合成：黑底上半透明红→蓝是RGB(0.25,0,0.5)，蓝→红是(0.5,0,0.25)。该例是按真实混合公式计算的可观察反例，不是已捕获的某个内置菜单截图。新建页面的两张背景有显式不同z，本次没有把它们误判成这组反例。

筛选：保留为绘制顺序修正项，不能仅归为无影响的实现方式区别。修复应沿用当前renderer和原比较器，恢复原不稳定排序语义，并验证重叠内容；现代Rust的`sort_unstable_by`也不能据名称就宣称逐项复现旧std对所有等值元素的排列。原HashMap桶序本身有随机性，禁止新造GlobalId或创建次序作为原版保证。

### 摄影焦点读回：门控一致；同World残留已定位，未确证新增恢复缺口

原`14234ba4b..14234ba99`仅检查focus tag<2、响应非零且响应等于当前相机模式；命中即写tag1和返回的世界位置。不检查会话ID、请求序号或当前位置是否来自本次点击。原指令36组（tag0..3、相机mode0/1/3、响应0/1/3）与当前`update_photo_focus`分支结果一致。旧位置(11,22,33)只要响应mode相符，原版和当前门控都能接受；不能把这种输入单独当作当前新bug。

当前异步生命周期已沿生产路径核对：`ReadbackPublisher::finish_frame`对不同World拒收，且`GpuReadbacks`按frame拒绝倒退；同World的Reload只对地形高度实施session epoch筛选，DofFocusReadback不筛选，也不在toolbar清理时删除。`PhotomodeState`在OnEnter(Game)重置，PhotomodeScreen在退出时关闭。SSBO在startup初始化；shader每次dispatch末尾写返回位置和mode。原侧是同一SSBO持久映射、模式号握手，本轮未取得原Reload必须清零该SSBO的证据。

因此，已确认“同World旧摄影副本可能暂时继续可见”，但尚未证明这是算法简化或原版缺失：不能擅自用地形epoch类推给摄影、强制清零焦点或回退并行渲染架构。异步复制可能增加可见反馈延迟，是否在实际Reload→摄影场景造成额外错误锁定，仍缺真实GPU场景差分。该项保留具体风险与验证边界，不列为已确认必须修改的算法项。

### 多滑杆交错：原共享偏移和末候选规则一致，不列修改项

直接核对原`1423491ab..1423492df`：phase3写同一个Local<Option<f32>>抓取偏移；phase0/1更新类别候选；后续事件有偏移便覆盖参数候选；结束事件不清偏移。原`1423492e4`才开始按最终实体重新查询并进入参数处理，不是每个事件立即更新formation或lens。

原指令探针执行`142349232/26e..142348fa1`及真实inverse-lerp helper，无运算替代。6组序列覆盖A开始→B继续、A开始→B完成、A开始→B开始→A继续、A完成→B继续，以及未曾抓取时的继续／结束。当前源码逐步f32表达式与原决策／最终候选一致。例如A偏移5，B开始改偏移-10，随后A继续，最终确为A使用-10；这是原全局Local语义，不应按直觉改成每实体一个offset。参数候选与类别候选可来自不同实体，也与原分支一致。

边界：phase3探针从AABB已解析后的偏移写入开始，不包含ECS查询、完整命中算法、真实鼠标或GPU；对照对象为当前源码表达式重现，未编译运行Rust整系统。当前源码EventReader依事件顺序处理，query失败继续、末实体重新查询、finite门和一次参数写入均存在。本次未发现可列为画面算法缺失的交错处理简化。

复跑命令：`D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe tmp-merge/m0-three-render-boundaries.py`。JSON绑定原RNE SHA、当前相关源码SHA、具体序列和验证边界。探针退出0，33组排序、6组交错及36组焦点门控完成。没有执行Cargo／真实GPU像素验收，也未修改生产代码。

## 前轮保留的边界（由上方续查细化）

- `mei_post_update`当前prepare按GlobalId排序，原侧按桶序；draw阶段当前`sort_by`而历史原证记为`sort_unstable_by`。这属于顺序差异，等深度重叠节点是否改变合成结果尚未闭合，不能直接判为算法简化或改动方案。
- 摄影focus的初始读回、同World切换会话及锁定响应需要真实GPU场景；已看到按World拒收与地形专用epoch逻辑，不能把地形epoch的保护外推为摄影已验收，也不能无原证认定原持久映射会在Reload清零。
- 本表是指定复杂链的复查，不宣称547条全部完成原版判等。辅助业务按本轮用户要求跳过。

没有执行Cargo、启动游戏或GPU画面验收。新增文件只有此独立复查记录与`tmp-merge`下的可复跑算术探针／结果。
