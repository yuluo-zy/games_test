# 指示符生命周期独立候选（未集成、未编译、未执行）

本目录只修改候选文件，不覆盖中央 proposed，也不修改 live。基于当前用户重构库增量恢复，未复制旧 rhapsody。中央按 owner-api-fragments.rs 窄合并 WidgetParams/UiState/Factory；四个 signifiers 实现文件与 tests.rs 可逐差异审查。

## 已确证完整算法

1. SignifierId 是16B，hash在+0，widget在+8。新增第二tuple字段保留现 `.0` 哈希访问面；wall_splitter/structure_outline的两处 `SignifierId(0)` 需补WidgetId(0)，实际create统一覆盖返回实体。
2. create先从已有且未marked的实体建立**只读**哈希快照。hover=0时保留stream，原140b2d419/41d跳过memtake/reset。不能延用旧“隐藏即丢弃”测试。
3. hover!=0时取走stream，把每项真实工厂追加到唯一拥有的active_params.signifiers。原140b2d4f9..514检查Arc弱引用/强引用唯一性。候选只在非空追加时用Arc::get_mut，绝不clone再取mut，也不make_mut复制整套mesh/geometry。
4. 遍历完整captured工厂列表。hash(widget,Option<name>)已经存在则跳过，否则wake并为每个实体写hash+widget。**本批新hash不加入快照**：原循环只有contains，不是seen.insert；同批同名工厂可各自wake，不能私自简化。
5. mark从同一个captured列表建立live hash。hover0时为空。nonlive且无最短寿命，或 `!(min_lifespan > t)` 才标记，NaN不会被错误排除。
6. animation_t使用现有完整7类easing和原float推进公式。mark Commands此时尚未在本链内部flush，新标记不立即反向。
7. transform的scale与translation门独立。translation需id、ParentToVolumeOrigin、hover!=0、id.widget匹配。Sphere=center；SphereCloud=centers[intersection.element]，保bounds panic；PlaneDisc=point；tag2/3/4/5/7不改位置。原8项表142ad55ac，旧Ghidra8..16为垃圾。
8. despawn只在有删除标记且无animation或animation.t精确等于±0时执行；NaN/Inf/非零包括负值均等待。使用原chain_ignore_deferred，保持退场与再次进入的实体隔离。

## 集成阻断，不得宣称已完成

- 旧 UiSignifierEntry::Raw 没有可执行工厂，原版合法域全是真实String+Arc closure。当前旧consumer只spawn标识占位是假完成。候选遇到此路径明确失败，不猜图形/业务；**不能把候选直接投产后以panic当修复**，必须先清查并恢复活跃raw writer的真实工厂。暂未解决的原U在下表登记。
- 旧 SignifierFactory::new(payload,null/vtable) 若无拥有closure，同样不能靠wake空Vec蒙混；候选检查has_closure。合法返回空Vec的真实closure仍允许。
- 原九成员set6链还有debug volumes和roof UV缺实现；本目录只提供已有五成员signifier slice，禁止补空体。host接入必须在ui_update后，MEI pre也有独立原after(ui_update)约束。
- 现有 tests.rs 的首次同批请求去重、inactive清空stream、立即删除等断言固化错误行为，应替换为本目录真实原合同用例，不能为了旧测试倒改原逻辑。
- 需合并owner fragments后再编译和运行。没有执行Cargo、GPU、游戏或原函数；本目录不是已通过的验收结果。

## 原证

原binary SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。create140b2cfa0、mark140929160、tick140a69340、transform140a69c30、despawn140929a90。round12/signifier-runtime保存全函数原证；DEPEDENCY报告及signifier-transform-original.asm.txt保存独立核对。原host140945c60→chain_ignore_deferred14175a380含九成员。读取原字节，不执行原exe。

## 负例测试要求

本目录提供CPU ECS候选：16B身份；inactive保留与重新激活；同批同名双wake和次帧已有实体去重；共享active Arc追加失败、零追加不要求唯一；captured列表保活、原deferred退场多帧；NaN寿命门及despawn零/非零/Inf/NaN；widget身份隔离；flag8不阻断translation；Sphere/Cloud/Plane与其余tag；Cloud索引越界。

后续独立验证仍必须加入原工厂mesh实体、TransientMesh卸载/重入无残留、真实toolbar Replay cross显示/跟鼠标/消失、实体删除时机，与真实input→ui_update贯通。最终联合验收不能替这些单模块补漏。

## 性能门槛

按 D:/game/PERFORMANCE-STANDARD.md，完整业务/算法/画质的Release物理960×540小窗口持续60FPS最低、120FPS才合格；报告平均、1%低帧和p95/p99帧时间与场景配置，禁止用删工厂、删帧、错误去重、提前销毁或截断capture来达标。候选避免额外geometry复制；HashSet快照构建符合原算法，优化须保留创建重复与deferred可见性。当前性能未测。
