# WP13 — `resources::prefab` 业务逻辑合同

B 版本。`crates/country-core/src/resources/prefab.rs`（51 PDB 元记录 / 8 函数）。
恢复 prefab 库/目录状态。实现见 `src/resources/prefab/`。

## O — 直接观察

### PrefabLibrary（Resource）
`hashbrown::map::HashMap<asset::prefab::PrefabName, resources::prefab::Prefab,
std::hash::random::RandomState>`（O：`add` 0x1408e2540 的
`HashMap::insert<PrefabName,Prefab,RandomState>` 过程）。
- `add` 0x1408e2540：insert；被替换的旧 Prefab 逐 `PrefabPart` drop。
- `get<&PrefabName>` 0x141552d70：按名字 hash 查找 → `&Prefab`。
- `has<&PrefabNameRef>` 0x141552ff0：按名字 hash 查找 → bool。
- `iter`：遍历 entry。
- `impl$7::register_required_components`（PDB 名）。
- Bevy 事件 `PrefabsModifiedCmd`（库变更通知）；Plugin `PrefabWatchPlugin`。

### 关联类型
- `Prefab`：库值结构（字段 U）。
- `PrefabPart`：`drop_in_place`（add 逐部分清理）。
- `PrefabInstance`：ECS Component（`PrefabInstanceData` Query 出现）。
- `AudioTriggerShape`：与 `asset_libraries::audio_trigger_library` 的
  `AudioTrigger` 成对存储（Drain 过程出现）。
- `asset::prefab::PrefabName`（外部 `asset` crate，候选 TG-a3f23bd62617630cba）：
  allocation 含 16B header；`as_name_ref` B=0x141556130 首指针+16、长度不变。

## I — 推断

- 库变更由 `PrefabsModifiedCmd` 广播（WatchPlugin 存在）；未确认消费者。
- prefab 值由名称查询（get/has），供 `tree_edit::missing_prefab` 等消费
  （history 侧 `missing_prefab` 0x1409e6210 属树编辑回放）。

## U — 未知（typed gap）

- `Prefab` / `PrefabPart` / `PrefabInstance` 字段布局与语义。
- `PrefabWatchPlugin` 的 watch 目录/触发策略。
- `register_required_components` 注册的组件集合。
- `PrefabName` 的字符串编码/所有权细节（借用视图而非完整实现）。
