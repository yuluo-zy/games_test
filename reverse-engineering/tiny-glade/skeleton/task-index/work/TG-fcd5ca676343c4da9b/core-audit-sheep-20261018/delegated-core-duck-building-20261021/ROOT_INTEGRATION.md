# 中央唯一宿主接线（不修改生产）

原Plugin已挂plan/update systems，本包签名由Res<当前CritterAudioCore>和实际WaterAudioGrid驱动；不能缺factory时默认禁音。与audio.rs/duck.rs候选、canonical WaterAudioGrid sampler、AudioInstance set_parameter log-once leaf一起合入。

## Native host（audio_host.rs，由root所属源集成）

在install创建或取得实际AudioCore<NativeStudio>以后，Startup/首个Update之前：

```rust
let critter_audio = system_critters::audio::CritterAudioCore::from_audio_core(
    share_core(app.world().resource::<AudioCore<NativeStudio>>()),
);
app.insert_resource(critter_audio);
```

share_core已是此文件真实函数，克隆NativeStudio内部共享引擎句柄及banks，不调用第二次create_audio_resources，也不新FMOD Studio。必须使用当前实际gameplay World的core（与load流程共享同engine）。

## Disabled host（gameplay_startup.rs，由root所属源集成）

install_disabled_audio原来插入AudioCore<GameplayAudioStudio>以后，按同一个None Studio构造分支安装，不通过未安装factory的默认结果：

```rust
let core = app.world().resource::<AudioCore<GameplayAudioStudio>>();
let shared = AudioCore::<GameplayAudioStudio> {
    studio: None,
    banks: core.banks.clone(),
};
app.insert_resource(system_critters::audio::CritterAudioCore::from_audio_core(shared));
```

只在原cfg(not(any(windows,feature="native-audio")))路径执行，None来自该路径实际安装的DisabledStudio资源；不是native失败后的静默降级。

## 启动/测试边界

CritterPlugin、register适配初始化不应自行创建None factory来盖住缺装配。独立需要实际运行plan的World测试显式安装AudioCore<DisabledStudio>的factory或记录Studio（随包同World测试已如此）。root已有schedule只构图不运行不需假resource。

暂不改constructor之外完整轨迹算法；仍有原先Quat/from_rotation_arc、dev/demo、计划retention生命周期等已登记差异，不把此音频包说成整个duck.rs完成。finish/UI producer保持合同待后续，并不因事件consumer测试通过就称实际编辑闭合。
