# 原Weather/prepare登记是否存在共同gate

2026-10-18，只读取原B；结论：在已完整核过的系统/外层tuple/GameSystemSet配置链，**没有共同run_if**。TOD单独有is_gameplay_active，不能外推给Weather或prepare，也不能给它们编造resource_exists。

完整PDB长RNE asm/字节SHA见native-manifest.json；原run_game_thread完整ASM复用boot包TG-c459c7f73345e24709/main-chain-boot-20261018/run-game-thread-rne.asm。Ghidra辅助JSON保留截断/损坏原状，实际判断以原RNE为准。

## 具体登记

- `14059fc0b` DL=0x24，`fc0d` TOD in_set@1402cc220，`fc20` 对该配置调用run_if@1400d37d0。该run_if模板的真实输入是bool fn(Res<GameTime>,Res<State<AppLoadingState>>)，注册时与TOD配置相邻。同一个helper还用于另外两配置，不表示整tuple都被它包住。
- `14059fc5b` DL=0x24，`fc5d` Weather in_set@1403429a0，输出rbx+2bf0。没有后续run_if；`fe14..fe6f` 将它整0x70B搬入outertuple的rbx+0xcc0位置。
- `14059fc70` DL=0x2c，`fc72` 对prepare/update_rt_world/upload组调用in_set@1403c7a30，输出rbx+2590。该helper建立三子系统的配置及所属set，没有条件；`fe76..fed1` 将组配置搬入outertuple rbx+0xd30。
- `14059ff83` 将rbx+0xb00开始的七份配置交SubApp::add_systems@140327490。在helper内部`140327586`调七tuple into_configs@1400d4610；后者`1400d4afb..4b07`建立7元素Vec，`4b0f/4b17/4b22`外层condition Vec=(cap0,ptr8,len0)和chain字段0。即外层没有run_if，也没有额外先后链。不能把三系统tuple的排列视为chain。

## 全局set不是隐含状态gate

fresh xrefs证明Update的SubApp::configure_sets唯一实例@140b68190只有add_stages@140945f3f这个静态调用者；add_stages本身由run_game@1405948be与run_game_thread@1405a442a调用。add_stages@140945ed0构造All<GameSystemSet>.map迭代器，再配置chain。

Map单态@14130f350完整1451B仅把enum值intern并into_config@140cdd670、分配和追加配置；所有直接call已读，没有condition函数/system构造。into_config@140cdd670原`6cd/6d5/6dd`将condition Vec写成(cap0,ptr8,len0)。所以set36/set44也不因add_stages自动带AppLoadingState条件。

原is_gameplay_active@140a90e00完整22B：GameTime+30(ran)==1，再State<AppLoadingState>首byte==1，返回布尔AND；原函数没有resource_exists语义。

## 运行时资源前提

如果首Update缺ActiveSession，需继续检查原PostStartup Pending(Game)→StateTransition→OnEnter(Game)的实际session producer及App更新顺序，不能以本次未发现gate为由制造资源或追加猜测gate。boot已获反证并继续该startup闭包；本次没有修改任何生产源码。
