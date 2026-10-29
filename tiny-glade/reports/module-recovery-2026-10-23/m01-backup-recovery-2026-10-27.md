# M01 验证失败回滚：九资源备份恢复

## 原文件与恢复范围

先实际读取了 `reverse-engineering/tiny-glade/skeleton/B/crates/systems/decorator/src/backup.rs`，原编译单元470，`create_or_restore_decorator_backup@1408238a0`、`clear@140823660`、`default@140824680`，以及原行46的 `consume_after_processing`。沿用现有 `systems/decorator/src/backup.rs`，没有新增生产目录。原九参数身份按系统构造`140489a80`及中央已锁定的合同恢复。

| 参数 | 保存/恢复内容 |
| --- | --- |
| Archivist | Wall、Terrain、Stair归档桶；保持其所有权与碰撞快照，本系统不清空它 |
| DecoratorStorage | 对归档所列地址逐桶保存最近有效状态；恢复时维护removed、modified、d2s索引 |
| DerivedDecoratorSubtypes | 原备份+230，完整真实子类型表 |
| DecoratorBlueprints | +1d0，完整Arc蓝图表；不能只恢复Info而留下拒绝预览的碰撞/视觉蓝图 |
| CachedDecoratorTransforms | +200，完整实际变换缓存表 |
| DeferredDecoratorOpEvents | +260，锁内Vec的0x88B载荷快照；独立拥有队列内容 |
| DeferredDecoratorEditHistory | +0，完整32B记录Vec；pending仅作预览state清理门，不属于复制字段 |
| DecoratorBackup | 最近有效快照和reset/created/state三个控制值 |
| StairsState | RESTORE最后调用现有restore_backup；CREATE不擅自创建第二份楼梯快照 |

原先的三个48B `BackupAuxiliary` 占位已删除，改为上述真实资源。备份存储也使用现有DecoratorStorage/Decorators结构，保存桶的原无d2s策略，活存储始终使用真实d2s策略。

## 状态、顺序与失败行为

1. 先消费state并置0。state0不处理；state1执行CREATE；其他非零state在created为真时RESTORE，不能只接受字节2。
2. CREATE按归档地址遍历，先get_or_insert备份桶，再读取活桶并unwrap。缺活桶必须失败，已经创建的备份桶保留；不能悄悄跳过或跑帧尾reset。
3. CREATE逐桶使用原`tracked_clone_from<()>`：空目标复制三集合；非空目标先清modified，把旧条目放入removed并清旧map，再传播source removed，插入新map、撤销对应removed并标modified。重复CREATE不是直接覆盖整个Storage。
4. 然后依次复制子类型、蓝图、变换、锁内0x88队列、历史records，最后created=true。只保存归档列出的桶，未参加当前编辑的其它活桶不受影响。
5. RESTORE先保存deferred.pending，第一遍清理归档列出的全部现有活桶：清modified、登记旧IDs为removed、drain条目并移除d2s。第二遍逐地址恢复，目标缺失unwrap失败。
6. 第二遍源选择由原`1408242ec TEST/1408242ef CMOVNE`确定：最近有效备份中有该地址就取它，否则使用**最初archivist桶**。不是空桶、不是跳过，也不是按Info.destination重新猜地址。
7. pending为真时遍历完整恢复map，只有state1才取可写视图、改0并标modified。随后反向复制子类型、蓝图、变换、队列和records，最后stairs.restore_backup。pending和potentially_surviving集合不被备份覆盖。
8. 正常返回路径最后处理reset_after，按存储→蓝图→缓存→子类型→队列→records→flags清除快照。失败路径不执行尾部清除。旧Arc按拥有者覆盖/清除自然释放。

## 原指令独立验证

RNE SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

- `m01-backup-original.asm`：直接从RNE导出的完整函数和清理范围。
- `m01-backup-original.c`：原工具留存C，作为控制流阅读材料；以机器码实际分支为准。
- `m01_backup_native.py`：直接执行`1408238a0`原控制流。容器get/clear/clone、锁内Vec克隆、records尾部扩展、楼梯恢复和default/drop在明确边界桥接；原状态分支、两遍地址迭代、fallback指针、记录前缀拷贝、资源调用顺序、flags写入均由原指令执行。不是完整容器算法的native等价声明。
- `m01-backup-native.json`：128组state/created/reset/pending/saved/missing组合，32组原unwrap失败；另2组Wall/Terrain/Stair三桶顺序、2组零归档桶路径。原三桶CREATE调用地址为`0,0,1,1,2,2`，RESTORE为`0,1,2,0,0,1,1,2,2`，证明先清全部再逐桶取备份恢复。
- `m01-backup-native-cases.rs`：从同次原执行结果生成的只读数据。真实九资源World逐组核对128组flags、三map、0x88事件全载荷、32B记录、Info预览状态及楼梯恢复，全部通过。桥接范围在JSON和脚本中保留，不以该探针冒充业务callee本体验证。

## 真实资源与多帧验收

最终`system-decorator --lib backup`过滤7项全部通过，包含128组原控制流对比、真实validator多帧拒绝回滚及缺d2s失败顺序。主要测试覆盖：

- 三种桶均非空，最初归档与最近有效状态不同，恢复选择正确；未归档桶继续保持当前内容。
- blueprint Arc在CREATE时由live+backup共同拥有；拒绝预览Arc在恢复时释放；清backup后有效Arc剩live唯一owner，最终live清除后释放。
- 重复CREATE保留原removed传播；新加入归档的地址使用初始归档fallback。
- CREATE缺活桶先消费state、先产生目标备份桶，再失败；reset不执行，三map不提前覆盖。
- RESTORE缺d2s时在破坏性清桶的原阶段失败；请求已经消费，reset与派生资源反拷贝、楼梯恢复均未抢先执行。
- 真实`validate_blueprints→create_or_restore_decorator_backup`两帧链：第一帧有效并CREATE，下一帧缺蓝图被validator拒绝并置RESTORE，实际九资源恢复，发出原静音预览通知。
- core三个Clone适配测试4项通过：同容量map复用、独立源数据；队列保留Mutex/Condvar身份及Vec指针/容量，with_events不消费也不复制数据。

工作目录`/Users/yuluo/Desktop/game/tiny-glade`：

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_backup_native.py
CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib backup --offline -q
CARGO_INCREMENTAL=0 cargo test -p country-core --lib backup --features native-physics --offline -q
```

完整system-decorator测试首次被同期cull owner新增测试的三处类型错误阻断，已反馈并由该owner修复。随后该owner统一全量实跑428通过、0失败、2项既有ignored（包含当时backup6项），日志`m01-cull-decorator-all-tests.log`。本链最后新增缺d2s失败测试也已定向通过，共backup7项；不为同一已通过代码重复全量构建，不能把这些CPU结果写成全游戏完成。定向日志为`m01-backup-tests.log`和`m01-backup-core-tests.log`。

## 本轮可复用经验

1. Ghidra的`all_decorators`名称不能直接对应当前flatten API：原调用遍历0x98B的WallId+桶，而Info map条目是0x50B；`140a1c2a0`只是从&WallId读qword并返回Wall判别。用真实步长和参数定位地址，避免把备份错误地按单个装饰目的地处理。
2. 三map的RawTable clone helper存在ICF别名，名字可显示tuple<ID,ID> set。资源身份以系统构造及调用数据流为准，不能据此把实际子类型表改成另一张关系表。
3. `clear_modification_tracking`的两层同名适配语义不同。Storage帧末只清tracking；原`Decorators::clear<DecoratorToStorageMap>`会清条目、登记removed并更新d2s，回滚必须后者。
4. derived资源默认派生Clone的clone_from可能替换整个包装对象。此处只在现有owner内补显式map.clone_from；事件队列的clone_from只更新独占目标Mutex内Vec，不替换同步对象。无需新增raw辅助字段或并行缓存。
5. 32B历史enum的3/4/5分支在原clone中没有活载荷，不能把C中未读的inactive words当作漏算法；测试使用0/1带载荷记录验证真实前缀拷贝，并保留当前typed记录数据。
6. 九资源测试必须给桶建立真实d2s，操作放在可写帧内，使用非空blueprint Arc、缓存、队列、历史、楼梯。只有Storage的空默认回滚不足以证明验证失败主链完成。

本轮不操作OS输入，不提交Git，不改渲染结构。生产初始化与原validator→backup→fullvisual/copy调度由中央owner完成并联合验收。

另外，兼容只读锁域时发现既有`ui_stairs_misc.rs::continue_stairs`在disc3、byte81='?'、byte80=0分支将四个u32（16B）收集后转换为24B端点并unwrap，实际必失败。已报告中央；本任务只获授权包入with_events锁域，不更改该楼梯算法。应按原`14079c3a0`单独核端点布局，不能用新备份队列已接线掩盖此后续楼梯缺口。
