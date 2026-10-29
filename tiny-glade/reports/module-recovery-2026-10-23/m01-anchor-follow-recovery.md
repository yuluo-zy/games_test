# M01 高度/形状编辑后装饰重锚复原

2026-10-27，基线main `858da5891`。本轮恢复原B `move_decorators_following_anchors` 14081c790及同文件两个内联业务函数；复用既有core公共投影与utils CurveSlice算法，未扩展装饰编辑或cull业务。独立真实GPU/OS输入不在本次World验证范围。

## 原所属路径与身份

编辑前已实际核对 `reverse-engineering/tiny-glade/skeleton/B/crates/systems/decorator/src/move_decorators_following_anchors.rs`，编译单元466，Source SHA256 `d1c405c758554b80303860f49e4b1e23c95c98f24bf0ea9802df09d921a36f4f`。原行133 `invalidate_roof_decorators`、行151 `reanchor_doors_attached_to_bottom`均属于此文件，直接恢复同原文件，无新代码目录。

RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。fresh Ghidra body/constructor/callees在`m01-anchor-follow-native.json`，原RNE ASM在`m01-anchor-follow.asm`；`m01-anchor-follow-iterators-native.json`记录原modified迭代器的缺ID失败语义。没有修改共享数据库。

## 恢复的业务与调用合同

原PDB九参（constructor14037d280）逐项为：`Res<PublicWalls>`、OnWallDeleted Reader、OnWallChanged Reader、RecalculateRoofCmd Reader、AdjustWallHeightCmd Reader、`ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>`、`ResMut<WallAttachedDecoTracker<DoorStairsAssemblyParams>>`、`ResMut<DecoratorStorage>`、`DerivedDecoratorInfoResources`。第九项借用唯一子类型表与mesh library，绝非CachedDecoratorTransforms。两tracker是实际装配差分资源，不以返回Vec或第二套dirty资源代替。

| 原链 | 实现与写回 |
|---|---|
| deleted流 | 全量调用真实`DecoratorStorage::on_wall_removed`，清桶和d2s索引；不从removed存储日志反推事件，也不额外清原未清的tracker |
| changed流 | 存在桶则mark_all_modified；无论桶是否存在，分别向两真实tracker写on_wall_changed |
| roof流 | 全量RecalculateRoofCmd读取真实wall，仅将目标桶raw dst tag==2的记录标modified |
| height流 | 全量AdjustWallHeightCmd，同样只失效屋顶目的记录；不能整桶强行标脏 |
| 遍历 | 全部墙桶的modified集合；仅有modified才查PublicWalls原map。缺发布墙panic；modified ID不在记录map按原impl$5::next unwrap_failed失败，不静默丢弃 |
| 公共投影 | side0先以旧u取实际附着高，再从anchor_ws.xz求source Curve2最近段、原lerp→wall_space clamp，写coord.u/y；side1保留，其他side按原panic |
| 门底 | 公共写入完成后严格检查Info+21==1；缺subtype保留前段写入并跳过门底；有subtype取实际rank对应collision bounds，缺碰撞表按原失败 |
| profile | 沿原mapped Curve3（Public+38，现geometry.outline）的XZ弧长取点，读取该3D轮廓的y；不是prepared height_outline、不是arch heights，也不是另建近似terrain采样 |
| 三处高度 | 原算法写anchor_ws.y，随后读取max_y并写relative_y=y-max_y，最后写coord.y；保留缺cache时已经发生的前写入次序 |

公共投影沿用core原普通算法，只抽出`move_wall_decorator_following_anchor_base`供完整ECS前段使用。旧独立helper仍保留BottomDoorConsumerRequired guard，以免只调用数学helper的旧用户误以为完成门底；真实ECS不走该guard逃过门底业务。

门底垂直公式严格逐指令：`profile=t*(b-a)+a`；`half=(top-bottom)*0.5`；profile用MAXSS语义取非负（NaN→+0）；`candidate=(profile_nonnegative+half)+0.05`；candidate为NaN时选half，其余按原MAXSS选最大值。碰撞bounds直接借实际subtype/rank表，不使用默认bbox或猜测门高。

## 性能与原顺序

按原删除→墙变化→屋顶→高度顺序消费四流，持久Reader游标不重放。迭代拆借map和modified集合，不为每墙分配ID Vec；门底CurveSlice直接借`&[[f32;3]]`并按需映射XZ，不复制每门整条曲线。保留72B字节桥，不以unsafe切片转换假定glam/数组布局。

注册原外四组为auto_allow→五consumer普通tuple→move_follow→cache；五tuple内部无chain。父19链再包含cache和后续subtype/visual/generator。此证据及初次cull误判更正见`m01-decorator-edit-registration-audit.md`。中央负责生产host挂接及联合验收，本报告不以定向测试代替生产窗口运行。

## 原执行与真实World验证

- `m01-anchor-readers-native.py`在隔离Unicorn执行原14081c790事件前段，保留四流、双缓冲、游标和原次序；只在存储/tracker容器边界记录调用，32例×2帧=64帧。原调用与游标见同名JSON，TXT为Rust fixture。
- `m01-anchor-bottom-native.py`执行原14081d3de..14081d421算术块，不用Rust公式生成期望值。1100例含±0、NaN载荷、±Inf、次正规数、极值和512组确定性随机输入。TXT保留输入/输出bits，JSON声明二进制身份和fixture SHA。
- Rust真实World九参系统对照上述32例/64帧，验证删除、全部/仅屋顶标脏、两实际tracker、未读游标无重播、无新事件时消费上游modified、缺subtype保持普通写入、真实碰撞rank/profile及三处高度写回。另测缺collision和损坏modified ID按原失败。

2026-10-27实跑：`CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline move_decorators_following_anchors -- --test-threads=1`：**9 passed，0 failed**，日志`m01-anchor-follow-test.log`。原四参集成用例迁真实World九参后实跑`--test core_paths_integration ... anchor_follow_marks_modified_and_registers_changed_wall`：**1 passed，0 failed**，日志`m01-anchor-follow-integration.log`。

相关height模块50项、height消费者5项日志同时保留本目录`m01-height-test.log`、`m01-height-decorator-test.log`。完整M01、生产GPU、存档联合验收由中央报告，A/C完整同义性未在本B有界恢复中声明。

## 下轮复用经验

1. 原wrapper/PDB展开的SystemParam比章节中的寄存器猜测可靠；Res组可能在物理ABI展开，不能把最后一项想当然映成刚好存在的cache资源。
2. 区分Public+38 mapped曲线、+88 prepared曲线、+e0 arch高度；同一消费者不同分支可能分别读它们。
3. 原MaybeMut标脏与modified迭代不同。原impl$5::next缺ID会unwrap，不能复用静默filter_map而掩盖损坏。
4. 保留先普通写入再门底lookup的提交次序，missing-subtype是合法保留前写入；missing-collision是失败，不能统一当缺数据跳过。
5. 数值块直接运行原机器码可覆盖NaN/零值边界；容器边界原生oracle加真实World资源验证能同时核Reader协议与真正消费，但不夸成整个原Bevy二进制已执行。
