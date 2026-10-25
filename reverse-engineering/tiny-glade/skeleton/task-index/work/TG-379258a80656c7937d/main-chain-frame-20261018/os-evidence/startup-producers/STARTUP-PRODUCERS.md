# 每帧系统所需三项真实资源 producer

2026-10-18，原B `.rne`，SHA见native-manifest.json。完整函数按PDB长度取原字节，不以截断Ghidra body补猜。只读Ghidra辅助指认xref和模板身份，保存xrefs-original.json、producer-functions-original.json（add_wall_events的反编译因未实现指令截断，故实际使用RNE asm）。

- DirectionalLight：`country_core::resources::impl$1::insert_resources@140940d80`，`14094108c` 从原rdata142ab23a0取两个f32，直接读原PE为24.0、48.0，`109d`第三分量24.0；`10a8`与`10b3`清target三个float为0；`10d7`调用真实look_at@140b18550，`10e2`调用具名App::insert_resource<DirectionalLight>@14154d590。API：`DirectionalLight::look_at(Vec3::new(24.,48.,24.),Vec3::ZERO)`。这是原启动值，非测试造灯。加载App/main资源装配位置由boot按原调用链处理。
- EditingHighlight：`tiny_glade::app_systems::add_wall_events@1404936b0`，`140493cf5`对[rbp-5b]写qword0，`cfd`对[rbp-60]写qword0，覆盖13个value字节；`3d0c`调用App::insert_resource<EditingHighlight>@1401175d0。即color Vec3 ZERO、enabled false；当前canonical Default与原值一致。run_game_thread原`14059f514`调用add_wall_events，不能误称此资源在core共同insert_resources内。
- TransientStairEdgeIdAllocator：`tiny_glade::decorator_plugin::build@140342c10`，`1403442fc`/`4303`分别复制两个16B常量，来源在`140342fc3`/`2fd1`首次装入非易失XMM6/7；原rdata1429c2e50两个u64为{1429c2e40空ctrl,0}，1429c2e60为{0,0}。`14034430a`/`4311`保存真实RandomState两个seed（前段按原线程缓存生成/推进），`44322`调用App::insert_resource<TransientStairEdgeIdAllocator>@14013b2d0。业务初值就是空映射，当前canonical Default可以生产；不要预造已分配ID，也不复制native ctrl指针。当前hashbrown Map是source依赖适配，不声称复刻原std::HashMap ABI。

## 无SDK TOD入口

root授权下仅修改country-core/systems/time_of_day.rs。`time_of_day<S>`原13参数入口保留；新增`time_of_day_audio_disabled`原其余12参数ECS入口，显式None传入同一个私有业务体。其对象安全audio视图仅实现于既有AudioCore<S>以转发原is_connected与set_parameter/logging，不新建Studio实现或Resource。

原三个门分别控制TOD-Enabled、Timetravel、DayNight；无SDK入口只跳过这些调用。GUI角度/小时、read().last()事件位置、dev热键、播放状态、suncalc、snapshot和完整DerivedAtmosphere writer仍同序执行。新增`audio_disabled_ecs_still_produces_actual_atmosphere_without_studio_resource`直接运行12资源真实ECS系统；使用原TOD第一entry摘录，明确测试的是原合法one-entry序列分支，不以该fixture代替生产完整asset loader。由root统一运行，未自行Cargo。

用户rhapsody重构约束保持：本次只core业务薄适配，没有更改rhapsody内部架构。
