# M01 Reload 碰撞世界生命周期边界复核

结论：当前菜单 Reload 保留同一个 App/World，缺失原版退出碰撞系统会留下旧 collider、其网格 Arc 和网格相交图。启动时的 Loading→Gameplay SubApp handoff 确实通过 Drop 释放旧加载 World，但只发生一次，不能解释后续菜单重载。本文在原卸载报告的初步 rg 发现之上，补齐真实宿主路径、原版注册链和可复现对照，避免凭“没搜到调用”断言业务缺口。

## 已实际读取的 B 路径

- `skeleton/B/crates/tiny-glade/src/game_loop.rs`：原主循环模块归属。
- `skeleton/B/crates/country-core/src/systems/collision.rs`：build@140b685e0，PDB module596 record104764，真实code_size2139。
- `skeleton/B/crates/country-core/src/systems/collision/world.rs`：on_level_reload@1408d8710、原单ResMut系统宿主候选和两个调用点。
- `skeleton/B/crates/country-core/src/systems/collision/intersect_meshes.rs`：on_exit_game_state原line852、直接内联GraphMap::clear。

没有创建项目目录或新业务文件；只在既有 `country-core/src/systems/collision/intersect_meshes.rs` 补该原函数和有界测试。原20资源装饰卸载、game_loop、run_game、渲染线程均未再改。

## 现生产边界

1. `toolbar/menu/load_ui.rs::queue_load` 将真实NextSession插入Commands，写NextState(Reload)；`new_session.rs`选定真实模板后也写相同状态。
2. `run_game.rs` PostUpdate的原closure1403bf630在Reload状态写NextState(Game)。既有OnExit(Game)卸载后，OnEnter(Game)加载同一World中的NextSession。
3. `game_loop.rs`只在preparation为Some且Logo完成时调用`GameplayPreparation::poll`，随后把preparation设None。`run_game.rs`的`*loading.main_mut()=gameplay.take()`是这个一次性启动handoff；菜单重载不再调用它，也不创建新GameplayPreparation。
4. 因而原App Drop在完整关闭、错误/取消准备或一次性交接中有意义；对菜单Game→Reload→Game没有释放旧physics的作用。

## 原版注册链（全部只读取证）

```text
collision::build 140b685e0（2139B）
  140b6878a: mov dl,1  [Game真实tag]
  140b6878c: call 140b66ff0
    SubApp::add_systems<OnExit<AppLoadingState>,(GraphSystem,WorldSystem)>
      140b67082: call 1417d0600  [未chain的二元tuple构造]
        首system名@142ce450f: collision::intersect_meshes::on_exit_game_state
          B源码line852内联GraphMap::clear
        第二system ctor140b3dae0
          140b3dc32: lea r8,[142aeeb70]  [真实System vtable]
            +0x50 @142aeebc0 -> run_unsafe入口140b3c7e0
              140b3c870: call RaycastWorld::on_level_reload@1408d8710
```

原add_systems泛型明确OnExit；状态实参DL=1，匹配AppLoadingState::Game。tuple构造尾部的chain配置为空，未调用set_chained；没有run_if，也没有before/after/chain把它与其它插件独立注册的装饰物卸载、墙卸载、通用实体despawn强行排序。不能根据源码排列顺序制造原版未要求的调度边。

`native-evidence-pack` 对原build、add_systems和pair constructor三段（2139/323/666B）验证全SHA、匹配PDB、全部指令bytes与7个源码采样，PASS。日志及native query/ASM/JSON保存在同目录；没有修改共享Ghidra，没有执行原程序，没有另起分析worker。

## 最小恢复建议与已补原owner

已恢复：

```rust
// existing country-core/src/systems/collision/intersect_meshes.rs
pub fn on_exit_game_state(mut graph: ResMut<MeshIntersectionGraph>) {
    graph.inner_mut().clear();
}
```

当前run_game只初始化RaycastWorld，MeshIntersectionGraph没有生产初始化；原collision build明确安装两者，因此中央宿主应先初始化同一份Graph owner，再安装原二元退出系统，不用resource_exists门禁掩盖缺失。现手工重构库的RaycastWorld方法仍保留，不重构rhapsody、不重建渲染线程、不修改20资源卸载签名：

```rust
app.init_resource::<collision::MeshIntersectionGraph>();
app.add_systems(OnExit(AppLoadingState::Game), (
    collision::intersect_meshes::on_exit_game_state,
    |mut world: ResMut<RaycastWorld>| world.on_level_reload(),
));
```

clear相交图会释放边载荷、清节点/边并保留GraphMap容量。世界reset会销毁旧17槽Rapier世界、网格sources和发布器后恢复默认。它必须执行于退出旧关卡，不能每帧执行，也不能在新关卡发布后才执行。

## 有界真实 CPU/物理验收

命令：`CARGO_INCREMENTAL=0 cargo test --manifest-path tiny-glade/Cargo.toml -p country-core --features native-physics --lib --offline reload_owner_boundary_tests -- --nocapture`。

结果：1 passed、0 failed、0 ignored、3000 filtered，0.01秒。单测试含有/没有原pair两条真实Bevy App对照，使用真实StatesPlugin/NextState/App.update、真实Rapier collider和网格Arc，不伪造命中结果。

- 在同AppID下执行Game→Reload→Game；无退出pair：旧collider仍被射线命中、Weak网格仍能升级、相交图旧边仍在。
- 有pair：射线miss、最后mesh Arc释放、旧相交图清空且capacity不变，退出count恰一次；进入Game后的下一帧不会再次reset。
- 两条路径最终drop整个App后Weak均失效，确认完整App销毁本身可正确释放物理世界，不能把显式reset绝对化。

这只证明同App重载所需的物理生命周期和原owner算法。中央生产挂载、GPU Reload和真实OS输入仍由中央验收，不用此CPU测试冒充。

## 后续可复用取证经验

1. 对生命周期先查真实owner生存期。App替换和State重载都叫Reload，但物理清理需求不同；只rg函数名字不能裁定泄漏。
2. 原Bevy系统可能没有任何业务函数地址LEA引用。应从单态run_unsafe的函数入口→System vtable→constructor→tuple→add_systems→插件callsite反查。
3. 反查xref必须使用真实函数入口。本例callsite140b3c870位于run_unsafe140b3c7e0内；对前者请求xref为空，不意味着后者无人使用。
4. 状态泛型只能证明OnExit的枚举类型，还要核调用实参DL=1，才可判为Game。
5. 原plain tuple没有自动链式顺序；清理系统之间缺显式调度边时，保留原调度语义，不凭“想当然更安全”添加after依赖。
