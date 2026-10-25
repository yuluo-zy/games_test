# 2026-10-21 15:08 history/save 增量审计

基线由中央确认 main f72938c81 → 1f73dbc73。仅读源码、remote-changes.json 与已存在日志；没有 Cargo/测试/逆向执行或生产修改。

结论：维持完整业务恢复区间 history 65–75%、save 70–80%。旧格式 Tree 的真实失败已窄修且 CPU 回归真实通过，验证置信度提高；原33存档/2120条全语料仍失败，不能计为已完成。研究包增长不折算业务完成率。

## 实际增量

remote-changes.json 中 history/save/snapshot/tree 相关生产源只有 country-core/src/resources/history/tree_edit.rs。save_game/load_game/snapshot/history_runtime 本轮没有对应业务恢复增量。

TreeEditData 现对 prefab_name 使用 default=missing_prefab、is_preplaced 使用 default=false（tree_edit.rs:39、41），只影响缺字段；没有对 required四字段加默认、没有吞类型错误或制造具体prefab。原Version18 full/25-swamp的13项Place/Delete不含这两字段，旧实际 corpus tg-postpush-original-histories日志因此missing field prefab_name失败。本补丁使用原MISSING_TREE_PREFAB_NAME；outer storage index 与 placement variant ID仍独立。

真实通过证据：postpush-render-20261021/tg-cache-sparse-tree-cpu-v3-20261021.log:1708、1709、1711，分别为真实Version18树片段、required/type/duplicate/unknown规则、逐字段默认3项。该次core总体2843 passed /0 failed /1 ignored（3702行），并非2844全部通过。Tree源当前SHA256 1ecc468cfb238ac4d560cf1be85253c43ac6dd2d967975229b8d36ddf778b7b5，与STOP-FREEZE source manifest一致。不能将这些unit通过当作真实prefab/forest重载全链通过。

## 原33/2120语料：仍未通过

1. tg-postpush-original-histories-20261021.log：full/25-swamp缺prefab_name，0 pass/1 fail。
2. tg-tree-defaults-original-histories-20261021.log：Tree默认修复后继续到03-halftimber-roof entry0 Tree position，原25.744875 vs Value宽化25.744874954223633，0/1。
3. tg-original-histories-wire-v3-20261021.log：到entry53 Terrain pos_xz，3.9437943 vs3.9437942504882817，0/1。
4. 最终tg-stop-original-histories-20261021.log:5：到entry54 Wall/anchor/anchor_ws/0，3.3601255 vs3.3601255416870117，0/1；STOP-FREEZE-RESULTS-20261021.json:243–246明确unresolved且未放宽断言。

这是同一个全语料测试逐步暴露的边界，不是4次业务回归完成。33/2120/408decorator aux/63stairs aux是测试最终预期总数；最新执行在此前失败，没有完成全部字段保留/双向命令展开验收。也不可因此断言生产世界坐标已经损坏。

## f32 原型与 JSON 比较合同

history-wire-numeric-review-v1/TYPE_AND_NUMERIC_CONTRACT_V1.md独立证明TerrainAttachment pos_xz/yaw为原f32；嵌套serde_json::Value先宽化f64，当前锁定serde_json1.0.85再解析可能偏移一个f64 ULP，而f32 bits不变。该已证schema叶比较允许按确切f32位，保留所有ID/结构/数组/其它字段严格断言；相邻f32 bit和signed zero仍必须失败。生产序列化器没有为测试更换算法。

当前 original_gameplay_chain.rs:116–159 的例外只覆盖确切Terrain路径，不能自行把最终Wall anchor失败纳入已证明覆盖。STOP最终失败应归类“原字段数值比较合同尚未完成裁决”，不能直接标记为坐标损坏，也不能当成通过。precision-wire-test-candidate/ROOT_ACTUAL_VALUE_MAP_ORDER_FOLLOWUP.json另记录decorators.diffs HashMap行顺序，限定该映射行规范化；其它顺序/值断言不删。

## 当前未变化的硬缺口

- snapshot.rs:1986仍会对Stairs保存checksum与边重算不一致记gap；load_game.rs:1083要求gaps为空，否则加载失败。现代楼梯格式与真实非空快照兼容完整验收仍缺。
- history_runtime.rs:106 prefab、136若干roof、165 terrain部分原注册点仍OPEN。当前工程接线存在不等于原调度合同全证明。
- 同一真实交互后的多次undo/redo →保存→退出→重启→非空场景实体/碰撞/上传/画面持久一致仍未验收。ACTUAL-RESULTS-V2.json:29明确editing_durability_restart=not tested；13个world帧/30帧GPU门通过不替代该链。
- STOP-FREEZE最终完整process另有device_ready断言，gameplay_frames=0；该次不能升为可玩启动验收。更早专项WSI成功仍是其明确范围内事实，不互相抹消。

相较上午审计：原“只有旧364项/round12编译失败证据”需要更新，现有本轮core CPU通过与部分GPU门通过；但history全原语料的最终结果仍失败。可以提高具体Tree旧格式与CPU局部验证置信度，不建议仅凭这次增量整体上调进度百分比。

参考根：D:/game/tiny-glade/reports/main-loop-render-recovery-2026-10-19/latest-audit-fa53d49a-20261020/postpush-render-20261021/。
