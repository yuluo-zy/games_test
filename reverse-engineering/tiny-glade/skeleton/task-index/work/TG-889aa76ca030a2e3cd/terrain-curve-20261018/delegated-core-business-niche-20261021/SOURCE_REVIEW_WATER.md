# Storage niche 两源独立源码复核

复核者water_activity_chain，非候选作者。**通过这两份冻结候选的最窄SOURCE复核及独立patch-F0复现，可交root成组集成验证。** 本次直接读取两份完整candidate/patch，不是重复合同审查。没有执行Cargo/rustc/GPU，没有写生产源，不等同整个storage/history/restore通过运行验收。

## 冻结源绑定

- `tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs`：before `8b3dbb7a3decc31951c1e00ef38dd64717bf3890a38281e065e2336101b0a16c` → after `98494a4397e50cfe92ffca99d7d5d62d548944f50410aec9ccff2090cd0b5105`；patch `167ab202189a2c47e00835af7cf1ffd8b00d37b1d7205af6dda6b0a05a7a4d82`。
- `tiny-glade/crates/systems/decorator/src/restore_archived_decorators.rs`：before `f42dd7bc295a217d7d666765818681d9d12a73474f9ba664a852152e533ba30e` → after `d30e246003dba0ff0fe42220f2931ba7af7259601c4e3b8695f145c630a43d92`；patch `45b6a1ae83abac20b9f27f3863ad9a0502783bec3fb73b09037caf78b8aa78c3`。
- 完整candidate/patch绝对路径分别由本目录delivery.json和 TG-2f6a8a4d921089b1a2/core-storage-niche-20261021/before-frozen-niche-20261021/delivery.json 指定。源码实际哈希与两份delivery逐项匹配。
- 适用合同：本目录CONTRACT.md、CONTRACT_REQUIRED_OLD_OPTION_ADDENDUM_WATER.md和CONTRACT_REVIEW_WATER.md。原证仍为既有TG609 storage-niche五完整包、14155afe0与restore原3967B包，无新采集。

## 逐项源码裁决

1. **真实旧Option及顺序已落实。** `Decorators::insert` 改为removed.remove→modified.insert→map.insert，返回真实旧值；没有按Info.state造返回Option。apply_place保存此返回值，None调用唯一d2s.insert、Some调用update，最后返回原输入Info副本，immutable/get_or_insert门保持。输入和返回的flags及所有其他字段均无mask或强写2。fresh d2s冲突发生前bucket、modified和removed撤销已完成，原部分mutation保留；d2s.insert本身仍按既有真实HashMap insert后冲突panic，未加默认成功或回滚。
2. **merge与唯一caller成组闭合。** merge source改owned Decorators；stage1只借source.removed_ids，stage2保目标既有d2s重索引顺序，stage3实际consume source.decorators的IntoIter，无state2额外break。每项先required d2s.remove，调用已纠正顺序的self.insert，随后按old None/Some执行insert/update，和原b678/b67d/b6a8/b52e相符。on_walls_merged仍先remove(a)再remove(b)，match两个owned Option；单源reindex后直接move，双源move target/source并调用新签名，最后per_wall.insert。目标桶不再clone，表allocation可以延用。全crates生产`.merge_with`实际只有此同文件caller，签名迁移没有遗留foreign借用调用。
3. **tracked慢路径是原Drain。** 先清modified、先把全部旧keys标removed，再`for (id, _) in self.decorators.drain()`逐项required d2s.remove。异常会析构真实Drain，清理未消费旧条目并归还空表/原容量，未留下borrow-for后再clear的差异。快路径完整保留；慢路径新项仍是原map.insert→removed.remove→modified.insert，未机械改用通用self.insert而重排；公共尾部d2s.insert与冲突panic保持。
4. **split为owned迭代和真实子桶旧值门。** removed桶的removed_ids先传播，随后consume source.decorators；Info原样按已有side/geometry分支处理，required d2s.remove保持。子墙命中时保存真实child.insert的old，None insert、Some update，吻合原a2013e调用14155afe0；未匹配/side1仍注册原墙removed id。没有新增None几何、flags门或丢弃未知记录的默认路径；原side>=2 panic保持。此包没有宣称原split几何公式全新核验。
5. **归档helper配对正确。** 缺tracked/缺id仍在任何storage mutation前返回false；真实bytes_to_info只解码已有记录。bucket.insert返回old决定d2s，输入flags未改；借用在最后insert后结束，符合Rust NLL分字段借用，不是另外的Bevy SystemParam/schema变更。无更换source owner或新表。
6. **不混同tracking owner。** 顶层clear_modification_tracking函数体未改，桶同名destructive adapter未改；没有因名称相同将两个不同生命周期合并。没有改本scope之外的history/collider/restore事件生产和注册。

## 测试源码检查（本复核未运行）

- 新place测试覆盖两种合法flags、fresh与replace、实际Info多字段保持、d2s冲突后的bucket/modified/removed部分mutation。使用真实storage/d2s和catch_unwind；没有raw2成功fixture或假callback。
- owned merge测试预reserve目标map，验证合并后真实entry指针保持、所有0/1记录及d2s正确。预留容量足够，不因必然rehash错误地要求无条件地址稳定。
- tracked测试走真实旧项、新项、同capacity，以及故意缺required d2s时panic后旧map空/全部removed/modified清空，源保持；不只测成功setter。
- split旧错误raw2测试改为多条合法0/1及u45位原样迁移，现默认坐标确为child首区间的合法输入，未靠特殊无效域验证错误niche规则。既有miss/side1/unreachable测试保留。
- 归档新增测试覆盖合法0/1、重复恢复旧Some避免错误insert冲突、fresh索引冲突后真实bucket/modified mutation；旧测试把不合法raw2 fixture纠回0。
- 代码只使用既有std HashMap/HashSet/Drain、Info Copy/PartialEq和已有ECS类型；没有新的依赖或Bevy0.16系统签名改动。source解析/类型判断是静态审查，最终编译和这些测试由root执行。

## 独立机械复现

`SOURCE_PATCH_REPRO_WATER.json`：从**实际生产before**逐文件复制到独立 `/private/tmp/tg-storage-niche-water-f0-*`，两次执行`patch -N -f -F0 -p1 -i <absolute patch>`，各返回0、stderr空、无fuzz；产物逐字节等于各candidate且after哈希匹配delivery。生产复现前后分别保持上述两个before哈希。

## 保留边界

本结论只接受已证oldOption/迭代生命周期及必要调用方配套。未验收所有flags创建/加载/历史入口、完整归档ECS事件/碰撞删除、split几何业务或整个游戏。人工raw2仍不被升级为原合法输入域；也不重复旧audit已纠正的“正常放置必然把2存map并阻启动”的过度结论。
