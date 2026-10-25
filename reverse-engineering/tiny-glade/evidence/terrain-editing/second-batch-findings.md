# 第二批：历史schema、原指令数值规则与重算连接

## 交付范围

本轮继续实际取证，不修改任何AGENTS.md、共享台账/依赖、原二进制或数据库。新增6个B函数与1个游戏私有helper的反编译，目录`B-second/`与`B-second-helper/`；均`-readOnly -noanalysis`。

`B-second-provenance.json`当前完成6函数12段与原RNE字节核查，全部一致；helper已导出dossier但更新校验脚本的重跑被工具权限分类器阻止，因此helper本轮只记反编译已取，未冒称来源范围校验完成。首次69段来源核查仍沿用首批报告。

新增两份原指令局部验证：
- `B-edit-replay-verification.json`：512例，reapply/revert的Edit分支从原函数入口至return，0差异。
- `B-delta-edit-verification.json`：512例，resolved Delta业务片段的原指令，0差异。

它们验证有限f32输入的数值/观察输出，不是Rust工程实现、真实Bevy集成或完整地形系统验收。未覆盖NaN/Inf、库分配/扩容、GPU、A/C及完整undo/redo事务。

## 1. 原作序列化确认真实历史变体与布局

`B-second/1417c1590.asm.txt:8-17,26-81,91-134,146-193`的分支、游戏键字符串与字段地址，结合原RNE读取确认（详见`B-history-schema.json`）。只恢复游戏schema，排除serde/Vec编码实现。

| 变体 | native第一qword | B版字段 |
|---|---|---|
| NewTerrainStroke | 0x8000000000000000 | Vec cap/ptr/len=+8/+16/+24；index +0x20 u32；height_ws +0x24 f32 |
| EditTerrainStroke | 0x8000000000000001 | index +8；prev/new height +0xc/+0x10；prev/new falloff +0x14/+0x18 |
| MoveTerrainStroke | Vec容量占第一qword，走niche fallback | Vec cap/ptr/len=+0/+8/+16；move_vec +0x18 Vec2；id +0x20 u32 |

不是普通tag0/1/2结构，不能把New解释为Move或将容量当enum tag。persisted index/id有各自字段来源，不据此断言所有标识可互换。前一轮agent将niche分支、偏移和Vec3含义混淆的假说未采纳。

## 2. Edit正反回放的实际事件字段，512例通过

`B-core/1408592d0.c:83-109`、`140858d30.c:215-241`及原汇编：
- reapply：高度/衰减各为 `new*t + prev*(1-t)`，按原f32乘/加顺序求值。
- revert：各为 `prev*t + new*(1-t)`。
- 两个operation字段均由prev/new的“不相等”决定：0保持、1绝对设置。即使当前t输出刚好等于已有状态，也不按实际输出重新决定flag。
- 40字节EventInstance观察布局：falloff operation/value在+0/+4，height operation/value在+8/+0xc，位置字段+0x10..+0x18清零，index+0x1c，event id+0x20。
- actual SystemParam是EventWriter<EditTerrainStrokeCmd>。伪代码grow_one别名里的indexmap/AlignKind不是游戏事件类型。

`verify_edit_replay.py`完整执行原Edit分支，不进入库调用；只以预分配、已解析writer作为ABI测试输入，未实现Bevy容器。检查payload、队列len/event id和change tick，包含正负零、同值和t越出[0,1]的有限值。

## 3. Delta规则不是单纯的clamp旧值+变化

`B-core/14094e000.c:575-627,632-748`、`.asm.txt:14094eb76..14094ef1b`，通过512例原片段校验：

### falloff
- operation=2使用Delta：`clamp(old+delta,0,1)`。
- 即使同值也写入字段；变化才发对应音频tag1。
- operation=1使用Absolute直接写raw值并发音频，不能把Delta clamp规则套到Absolute。

### height
- Delta且old==0、delta>0时，先将old提升为1再加delta。
- 加法结果clamp[0,20]，随后小于1的结果置0，因此不是连续的[0,20]标高。
- 变化时发音频tag2；从正值落到0另外发tag4。记录的变化幅度以该业务分支前后值求绝对差（激活时基线可能已从0提升为1）。
- Absolute写raw目标，不套Delta限制；若从正值绝对设为0有额外tag4分支。
- operation=0保持对应字段；参数字段语义已经由Edit序列化与replay输出交叉确认。

音频variant语义（raise/lower/remove等）仍须PlayTerrainSound消费者/序列化确认，当前仅记录数字tag。

Delta验证从已解析bucket指针进入业务片段，排除HashMap查询和事件扩容；不宣称完整edit_terrain_stroke函数或真实UI验证通过。

## 4. ReplayBuffer的业务回放策略

`B-second/140995150.c:76-161`：只处理外层ReplayCmd tag3的terrain命令，方向byte决定reapply或revert；实际CALL为0x14099539a/0x14099536a。

elapsed/duration达到约0.001的时长分支后，进度clamp[0,1]并使用cubic ease-out `1-(1-progress)^3`；时长低于阈值时直接完成。每次应用后才递增elapsed；进度完成后计数减1，变负才安排移除。真实时间/回放buffer资源字段依据SystemParam识别，但全history cursor/跨bucket提交尚未关闭。

New正向首次应用会从history复制原点列插入TerrainEditingData，并将next counter设为max(旧值,index+1)，默认falloff=0.4、height取历史字段；保留原ID上限检查（`.asm.txt:185-203`）。

New的revert在t≥0.99时进入对象移除路径，其余阶段经update_terrain_curve生成收缩效果；点列构造/删除与分配路径本轮尚无oracle，不能当完整删除系统。

## 5. 从history到重新计算的连接已找到

`B-second-dossier/dossier.json`：replay_terrain同时写EditTerrainStrokeCmd、EvTerrainStrokeChanged、SetTerrainStrokePtsCmd。`B-second/140a03ba0.c:46-76`显示：
- 只有存在未读EvTerrainStrokeChanged时，消费全部未读记录；
- 发一次RecalculateHeightMapCmd；
- ScheduledTerrainReadback计数原子+1；
- 发一次PlayTerrainSound(tag0)。

所以事件桥把多个未读stroke-change聚合成一次重算通知，而非每条变化都独立发dispatch。并未证明全局系统排序/GPU fence同步。

这里事件被错误命名为WindowFocusEvent或WriteSettingsFileCmd的伪代码泛型别名必须排除，以真实FunctionSystem参数为准。它们不是自动产生PendingTerrainHistory的证明：history_record_terrain明确取PendingTerrainHistory；replay事件链是另一条通路，不应混称“回放自动重新录制历史”。

## 6. UI伪代码目前不能完整信任

`B-core/140aef1f0.c:2-5,745-748,773-789`有重叠指令、未恢复jump table及异常IN/OUT伪指令。即使decompileCompleted=true，也不能把整份UI图写成已恢复。

其中case3的pending Edit构造候选在`:716-742`，字段与新schema吻合，但其完整分支归属和控制流必须用原PDB范围、jump table及机器码复核。下一步应提取游戏UI本体范围，不直接在共享数据库改类型或重分析。

## 7. 新发现的自有helper

`B-second-helper/140859740.c`真实名称TerrainEdit::update_terrain_curve；它包含获取stroke、复制原点、去重/Curve3构造、退化曲线fallback以及长度相关的收缩采样策略。底层Vec/hashbrown/glam实现复用，私有Curve策略需与已恢复曲线模块对照。

该helper并非两个Vec3线性插值。`0.1/curve_length`、easing和曲线截取是下一批高价值取证点；本轮未完成其数值与异常行为验证。

## 下一批优先级

1. 补齐New/update_terrain_curve和Move点变换closure，原指令oracle覆盖创建/收缩/移除/移动；保留enum niche布局。
2. 用PDB及原RNE修正UI本体和jump table解释，确认pending Edit producer及事件发布，不将库/坏反编译当业务。
3. 扩展Absolute/Keep与NaN/Inf边界，取得PlayTerrainSound实际variant含义；补edit→Recalculate/原子通知的完整入口验证。
4. 栅格化uniform/shader计算与读回同步、dirty→建筑/碰撞消费者继续单列；未宣称GPU计算公式或全调度图恢复。

本轮业务实现仍未交付，系统验收not_verified；新增的是实际schema、原指令局部数值验收和可靠事件桥。
