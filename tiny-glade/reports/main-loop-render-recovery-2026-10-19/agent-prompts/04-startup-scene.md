# 启动资产、场景发布及资源生命周期

先读取同目录 COMMON.md。本文件覆盖此前创建6名子agent的所有指令；你现在是单一执行者。

目标：实际发行资产经正确注册/加载/初始化成为同World的可绘制场景，支持真实重载与清理。
输入：当前gameplay_startup/run_game、startup模块、Prefab/Forest/Nani/VAO维护与原OnEnter/Exit证据。已确认有声明却缺文件：clearing/sheep_assets/startup_critters/startup_sheep/audio_host，先核当前filesystem/已有候选及原证，不凭模块名补造。
唯一宿主写面：gameplay_startup.rs/run_game.rs候选。root已委派TG-c459/main-chain-boot-20261018；Forest TGd94/a433旧boot-leaf由你延续子目录。其他filefamily先合法claim/协调。
必须完成：72真实SSBO/prefab atlas及shader输入→真实Mesh→AssetVAOLibrary→实体Vao→drawlists，Forest的完整Placement48/event/动画链，Nani实际instance flush/count，原OnEnter三组chain与未chain子tuple、OnExit实际状态；Mac无FMOD时保留原None音频descriptor/state分支，不能强索NativeStudio资源、造fake SDK或跳critter。
与01协作：GameplayPreparation保存worker句柄、取消协议、panic/失败可见、退出join，不能丢JoinHandle让worker继续占GPU。
性能：不复制库/世界/设备，不每帧重载JSON/mesh/atlas，不用replace-previous掩盖重复注册；清理只清原owner规定范围。
验收：真实资产启动→非空drawlists/实例→渲染；加载失败可见；二次会话不重复路径/泄漏资源，后台取消/重载有界。
