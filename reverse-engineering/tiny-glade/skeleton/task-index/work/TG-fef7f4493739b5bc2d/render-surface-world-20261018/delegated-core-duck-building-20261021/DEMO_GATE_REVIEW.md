# Duck Demo 与 WaterAudioGrid 必要依赖复核

2026-10-21，中央经 core_business_lead 委派只读窄查；未修改生产、候选或 claim，未运行 Cargo/GPU/原程序。此记录不改变已交 v2 行为包的 B 静态接受边界；Demo 主链仍未验收。输入冻结见 demo-review-inputs.json。

## 已观察原事实

- B plan_duck_trajectories 14230ff00，PDB m3877:2592，8958B：14230ff74 读 import14362c098，ff7b compare byte0，ff7e JNZ14231131c。该全系统早退在RNG/dev输入之前。
- B move_ducks_along_trajectories 142312200，PDB m3877:22812，6447B：没有相同入口return；每匹配实体142312c2e/35/38读同demo，非0转1423126c0 EntityCommands::despawn，继续下一实体。该分支发生在轨迹、运动、水格采样、所有音频之前。完整旧asm/反编译互证，仅作为原B事实。
- 既存PDB构造证据：IntoScheduleConfigs::chain_ignore_deferred 140f7e4e0，m1170，3623B；move into_configs141d8fa10，m2711，421B。函数参数第三项是 required Res<WaterAudioGrid>，不是Option；原调用链没有逐move的run_if包装。Plugin build14067d440经14067d45d/470/48a/495将链放Update GameSystemSet(0x24)。这不证明该set最外层的运行条件已闭合。
- water worker已确认完整原init1409ebf20/3955B：1409ebf74比较143634d90，ebf7b JNE1409ec3ae，跳过全部100格/100环和资源insert。唯一已存insert调用1409ec3a9→1412ded40。
- public-symbol窄筛仅命中 Commands::insert_resource<WaterAudioGrid>（PDB m1580:144136，section1 offset19782976+1000 = RVA12ded40）与queue/apply回调。未匹配到独立App/World.init_resource、new/default；这类阴性不排除内联生产者。demo-grid-symbols.json保留完整原记录。未匹配GLOBAL_ERROR_HANDLER/default_error_handler/set_error_handler命名记录，不能把这一阴性当原fork没有自定义handler。

## 当前依赖观察，不能外推原fork

本机锁定bevy_ecs0.16.0上游source：system_param.rs:884的Res::validate_param在资源缺失时返回invalid(skipped=false)；single_threaded.rs:96在body前校验，再调用error_handler；handler.rs:124默认panic。故当前宿主如果在Demo缺grid时执行此system，body内despawn不提供缺res保护。原fork等价未独立验证，该当前依赖事实不是原程序Demo会panic的证明。

## 未决，禁止猜补

原GameSystemSet0x24最外条件、可能内联的第二个empty Grid producer和原fork错误处理仍unknown；没有证据把move改Option、添加ActiveSession/resource_exists门或插默认Grid。full demo=false验证可推进；Demo启动必须作为独立剩余边界报告。plan入口Demo早退及move实体despawn当前仍是恢复缺口，须在权威demo输入与注册外门事实闭合后实现，不能借本次音频叶组声称完成。
