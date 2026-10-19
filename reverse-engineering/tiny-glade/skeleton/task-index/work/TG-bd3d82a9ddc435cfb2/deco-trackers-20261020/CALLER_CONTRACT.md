# Tracker 调用者合同与设计 V1 勘误

本文件是 `DESIGN.md` V1（SHA256 `21cf2ae0bdbf109743f00dd700a28df2056e86eb763d9a7d5a9e5deea468de19`）的独立增补；不改 V1 原物或旧 manifest。仅目标 B，C 不计验收。无生产/Cargo/GPU/benchmark。

## 必须撤回的 adapter 提案

V1 的“callback 未显式消费后自动空 removal callback 归一化”只是新 Rust adapter 提案，现根据 07 设计审查撤回。它会把遗漏接入的 Chimney removal 音效/装饰读取伪装成成功。

新协议必须强制 callback 显式完成消费；未完成不能以成功返回，也不在 Drop 中静默归一化。优先用持有 scoped map 借用的消费类型，`with_each_deco_diff` 返回 removal continuation，只有显式 `with_removed` 可生成私有 completion token。空 removal callback 仅在原已证 Door 分支或 PublicWalls miss 分支由 caller 明确选择。不得用空 callback 兜底代替未恢复的真实 Chimney 业务。正常分支的显式原 no-op 与缺业务无关，二者必须区分。

该 API 是保留业务协议的安全 Rust 适配选择；原 wrapper 精确 Rust 签名仍未知，不宣称取回了原源码。不要为了 runtime 错误返回而提前停止 dirty drain、丢掉余下 dirty wall；完成约束应尽量由类型接口保证，或在违反协议时显式 panic，不能伪造成功。

## 已领取的文件族

| 文件 | task | 当前 claim |
|---|---|---|
| `decorator_visual/deco_differential_state.rs` | `TG-bd3d82a9ddc435cfb2` | owner `deco-trackers-20261020`，B/C |
| `decorator_visual/chimney_assemble_bricks.rs` | `TG-917d6130810801d337` | root 新正常 claim 同 owner，A/B/C；本轮只 B |
| `decorator_visual/door_stairs_assemble_bricks.rs` | `TG-e10831b95b26b67a3c` | root 新正常 claim 同 owner，B/C；本轮只 B |

精确 claim 原物路径、SHA、workdir 见 `caller-claims-snapshot.json`。不取得 country-core 依赖源或 fullvisual owner。源码候选必须放这些已领取隔离 workdir，root 唯一 production/Cargo/Git/GPU。

## 当前实际 source 调用

相对 `games_test/tiny-glade/crates/systems/decorator/src/`：

1. `decorator_visual/chimney_assemble_bricks.rs:35..52`：输入 scalar subtype、bbx min/max、手工 `AssembleResMut{value,tick}`、tracker、`current:&HashMap<u32,ChimneyAssemblyParams>`、非空外部 `rebuild(id,&params)`。执行 `resmut.value=resmut.tick`，构造后丢弃 `_derived`，line52 转发 diff 模块的 `chimney_reassemble(tracker,current,rebuild)`。
2. `decorator_visual/door_stairs_assemble_bricks.rs:30..42`：输入同类手工 ResMut、tracker、current map、非空 rebuild；line42 转发 `door_stairs_reassemble(tracker,current,rebuild)`。
3. `deco_differential_state.rs:175..196` 两 helper 又转发旧 `tracker.with_diffs(current, ...)`；该方法 clone 全 frame map 作为 previous，并只回调新增/参数变化。
4. 除这两 wrapper、diff 自身测试、两 wrapper 自身测试，`rg` 在全部 workspace Rust 文件未发现其他函数调用；跨 crates 扫描命中仅模块声明与 utils 注释，没有找到当前生产 schedule 注册这两个手工 wrapper。该事实只描述当前源码可见接线，不抹掉原实际 consumer 的义务。

这些 source API 没有 `WallId` / `DecoratorStorageAddr`，也没有 Removed callback 或 current/old Diff 视图。不能从平 id→params current map 推导新 id 的墙，不可把全 world 视为 wall0、扫描别的 registry 猜墙或增加第二 registry。当前非空 rebuild 也不等于原 current/removed/按墙几何业务全部已恢复。

## 新核实的 B 原入口

完整原入口采集见 `assembly-entry-B/`，PDB 记录见 `assembly-entry-PDB.json`：

- `0x1407f8db0` 为 **117B**（当前 source 注释791B错误）；`0x1407ff130` 为 **52B**（当前399B错误）。这两条短过程本身就足够解释入口，不把旧 decompiler 文本字数当代码长度。
- Chimney 在 `1407f8dd1..dd9` 把原 `ResMut` 当前系统 tick 写到资源 change tick；从同一个 ResMut 内提取 tracker 指针。调用 `140b26a70` 后将派生资源句柄同 SSBO、PublicWalls、Terrain、EventWriter 组成闭包环境，再调用 `1412ae660`。
- `140b26a70` 是 PDB `decorator_subtype::impl$8::from` **8B** 原过程，不是现 source 的 `DerivedDecoratorInfo::new(subtype,bbx_min,bbx_max)`。当前 country-core 的同地址便捷 From 恢复也不是该原双句柄资源面；它是外部依赖合同问题，本包不逆写。
- Door 在 `1407ff134..13c` 同样标记真实资源 change tick；组合 SSBO、PublicWalls、UiSignifierStream 环境再调 `1412af3a0`。原没有 scalar `AssembleResMut.value` 资源，也没有 current map 参数。

PDB 的真实系统参数 type spelling：

Chimney 七参：`DerivedDecoratorInfoResources`、`Res<PublicWalls>`、`Res<AssetSsboLibrary>`、`ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>`、`Res<TerrainHeightsData<TerrainHeightSsbo>>`、`Res<UiSignifierStream>`、`EventWriter<PlayDecoratorSoundCmd>`。

Door 四参：`Res<PublicWalls>`、`Res<AssetSsboLibrary>`、`ResMut<WallAttachedDecoTracker<DoorStairsAssemblyParams>>`、`Res<UiSignifierStream>`。

参数身份来自 PDB System wrapper type spelling；不是将 SystemTypeSet 的 TypeId 当资源 TypeId。真实 source 参数名字、lifetimes 的原拼写不宣称已恢复。核心几何 helper `construct_crossbrick_chimney`（14200bcd0/2836B）和 `construct_door_stairs`（14200c7f0/4668B）当前源码没有实现；不能把仅转发手工 rebuild 的旧 wrapper 称为完整真实 assembly。

## 给 02 的稳定接口

02 完整 instantiate 候选仅需要以下已证副作用，已经定型同步：

```rust
add(storage: country_core::resources::walls::decorator_storage::DecoratorStorageAddr,
    id: country_core::resources::walls::decorator::DecoratorId, params: &P);
remove(storage: DecoratorStorageAddr, id: DecoratorId);
on_wall_removed(wall: country_core::resources::walls::WallId);
on_wall_changed(wall: WallId);
```

旧 previous addr remove → current addr add，不能颠倒；on_wall_removed 仅清该墙 tracker map 与 dirty。参数只有一个 canonical 类型，拟回到原各 assembly 模块定义，并由 diff 模块 pub re-export 同类型。Chimney words10 / Door words8，ordered PartialEq，无 Eq/Default。重导出不能生成第二 Rust 类型或第二资源身份。

## 可实施的最小候选边界

第一包可先冻结 tracker 状态/参数 Eq 的候选和明确的新 per-wall log 消费接口；这不是完整可生产整合包。没有原正确 caller 适配之前，不接生产 schedule，不宣称 restored assembly consumer。root 已允许先裁确定依赖，不必等全部几何 helper 才产出候选。

两现 wrapper 应同批增量重构到 scoped wall-diff 消费面，保留外部真实业务 callback 执行机会并传递 WallId、DecoratorId、current/old 与 removal，而不是删函数、保 current-map 假兼容或提供默认空实现。类型层可要求 `for<'a> FnMut(WallId, DecoDiffLog<'a,P>) -> ConsumedDiffs<'a>`；`ConsumedDiffs` 私有构造，只有显式消费 Removed 才产生。该形状只是接口候选，07 还需审查其安全 lifetime 与强制完成约束；不得无证 unsafe 延长借用。

如果提供协议 callback adapter，应明确命名/注释为协议边界，不能以此替代或注册成原七参/四参完整系统。实际 original system 的接入仍需 PublicWalls lookup、每墙 SSBO writer clear/build、DerivedDecoratorInfoResources 句柄、terrain 读取、signifier/audio 的真实 host。当前 country-core 某些相关资源面未恢复，接口不能造 dummy 填洞。编译适配可以先暴露新真实协议接口并调整已有本地调用者/测试，但不得让空 callback 或假tick通过表象测试。

候选验收应分别记录：已证 tracker protocol；已证参数比较；现有 Rust callback adapter 的接口适配；仍未接原 assembly 几何/音效 host。只有前三者不能提升为原完整业务 consumer 已恢复。

## 必须保留的非空业务边界

- PublicWalls 存在时 Door 重建整墙，访问所有 current 参数，随后显式空 Removed 业务回调归一化（原如此）。
- Chimney current visitor 包含实际组装，并用 diff 前后状态影响音效；Removed visitor 另读 decorator 信息/旧参数发原删除相关事件。Changed drop old 不是假删除。
- PublicWalls miss 时，原明确空 Removed callback 做状态归一化；这是一个具名分支，不能借它覆盖可用墙的缺失消费者。
- `with_diffs` 单次遍历 dirty walls；不得额外 Vec、全世界 current map clone、排序或每帧额外 id→wall registry。

新的所有源码/接口变更仍待 07 设计审后实施；本文不是 Cargo 成功或运行验证报告。
