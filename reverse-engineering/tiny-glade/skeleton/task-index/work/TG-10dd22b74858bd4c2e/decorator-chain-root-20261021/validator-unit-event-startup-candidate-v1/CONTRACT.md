# 验证单位静音事件：唯一启动注册

TG10dd本owner decorator-chain-root-20261021；保当前生产source before，仅在已实际从run_game.rs:405调用的 build_startup_resources 中增加 App.add_event::<MutePreviewDecoratorOpSounds>()。原539B验证入口产生单位事件，原两个真实音频reader count入口消费，必要事件type由TGc4候选唯一导出。

使用原App Events资源+更新系统，绝不手动全局clear或另建队列。真实App启动可达，不仅向未调用helper加注册。相关现调用run_game→build_startup_resources 已rg核实；现app_systems.rs:211已注册OutlineWallCmd，不重复注册。新增注释中文。无系统mount、无调度顺序变更；历史build()整体系统仍不作可达声明。本文件其余字节保持before。不新增镜像实现测试；完整writer/两个独立reader的保留周期由Mute V2 actual-system源码测试覆盖，待root实际执行。
