# M01 灯笼点亮音频 side effect（2026-10-27）

本轮只补灯笼维护两轮 producer 发出的 `PlayDecoratorSoundCmd.kind=0x0e` 到现有音频系统的消费及一次性实例生命周期；不宣称整个 decorator_audio、FMOD 运行时或整游戏已验收。

## B 路径、身份与调用合同

修改前实际查看 `reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/audio/decorator_audio.rs`。PDB 记录该同名系统真实 body 为 **14145dc20**（module 1606），init 为 14145c6b0；PDB `void()` 是占位，不能当作零参数接口。本轮沿用现有系统 `(ResMut<DecoratorAudioState>, EventReader<PlayDecoratorSoundCmd>, Res<AudioCore>)` 和 `Studio` / `EventInstance` 真实后端接口，没有新建音频层或业务目录。

- 原 RNE SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- B PDB SHA256：`2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
- 原表 `142c631dc + 14*4` 的有符号偏移 `-25182312` 指向 **14145f174**；静态事件名位于 `142c62a00`，**25 字节 `event:/SFX/Lantern/TurnOn`**。
- 分支从原事件记录 `+18/+20` 拷贝三个世界坐标，直接调 `instantiate_audio_w_position` **140aa3360**，随后转正常下一事件入口 **14145e1a0**。没有音量、额外参数、排序、位置去重或消费端限额。kind 0xf 的距离收集是另一分支，不可混用。
- lantern producer 的两次维护 pass 各至多四条是上游合同，不能在消费者再次截断。producer 归 rectangle owner，本轮未修改。

原保留反编译：`reverse-engineering/tiny-glade/evidence/country-core-deep/audio-misc/decomp/decomp-0x14145dc20.txt`。本轮保存六段 `m01-lantern-audio-*.asm`、五个 helper fresh `.c`。ASM 优先裁决反编译省略/错读的错误处理。

## 恢复行为

1. 在现有 `decorator_one_shot` 中接入 kind14 名称，由现有 EventReader 消费，走现有 canonical helper。
2. `instantiate_audio_w_position`：原 140aa33a3 检查构造 Err，140aa33d9 unwrap 失败；恢复创建失败在位置调用前 panic。音频断开仍为 Disabled 正常无请求。保留现有返回 API；consumer 的 `let _` 在同一事件内 Drop，与原 void helper 内部 Drop 对齐。
3. `AudioInstance::set_position`：原 140a57da0 的 forward 位型为 `(-0,-0,-1)`，up `(0,1,0)`，velocity 全零；失败使用原 log-once 消息 `Failed to set 3d attributes for FMOD event {name}\n{error:?}` 告警后继续。保留当前 Result API，不将位置失败升级为构造失败。错误格式化指针 140a52af0 实读原 Fmod/function/code/message 标签并调用结构化 Debug formatter，使用 `{:?}`，不能换成 Display。
4. `start_if_not_playing`：原 0xb 位掩码跳过 Playing/Sustaining/Starting，Stopped/Stopping 启动。**查询状态和 start 失败都 unwrap**。初次只看 decompiler 曾误读 start 为丢弃 Err；原 140a57360→140a573b7→140a573fa 明确 fatal，已纠正再实现。
5. 原 Drop 14089a280：一次性实例 state=2 不 stop，直接 release，release 失败 fatal；现有实现已经一致。本轮不改 loop 算法。

## 验证与失败记录

原指令探针 `m01_lantern_audio_native.py` 绑定 RNE SHA，直接执行 kind14 分支及 140aa3360/140a57da0/140a57300/14089a280；只在 AudioCoreInner 构造、FMOD SDK、panic、释放边界桥接。

- 4 组位型（普通坐标、负零、NaN payload、正负无穷）验证静态名称及坐标逐位传递。
- 4 位型 × 5 播放状态 × 5 正常/创建/查询/start/release 状态 = **100 组**，68 fatal 符合原指令。
- 单独注入 set3D Err，执行真实分支到 140a57e2c 的原告警格式化路径；格式化/log-once 容器不伪造执行，静态核其完整错误路径。
- Rust `RecordingStudio` 实现真实 `Studio/EventInstance` trait，运行实际 ECS `decorator_audio_system`：两个非空位置、连续帧消费一次、同位置 12 条全部播放并释放、五种播放状态、创建/位置/查询/启动/释放失败和断开 Studio。完整记录 Create→3D attributes→Playback→Start→Release，断言没有 Volume/Parameter/Stop。
- Rust 对 100 组原指令输出逐项比较真实 canonical helper；probe 在 panic 入口终止，不伪造 Windows unwind，Rust unwind 额外执行原应有释放的边界单列。
- v1 差分发现 probe 的构造桥接漏填原 `state=2`，导致原 Drop 错走 stop 分支；修正桥接而未放宽断言。保留 `m01-lantern-audio-tests-v1.log`，v2 全绿。

命令（从 tiny-glade 目录执行）：

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_lantern_audio_native.py
CARGO_INCREMENTAL=0 cargo test -p country-core --lib --offline systems::audio::decorator_audio
CARGO_INCREMENTAL=0 cargo test -p country-core --lib --offline audio
git diff --check -- crates/country-core/src/resources/audio_resources.rs crates/country-core/src/systems/audio/roof_audio.rs crates/country-core/src/systems/audio/decorator_audio.rs
```

结果：decorator_audio **8/8**，受共享 helper 影响的音频相关 **169/169**，无失败/忽略。日志为 `m01-lantern-audio-tests.log`、`m01-lantern-shared-audio-tests.log`；源码和 probe/output 指纹为 `m01-lantern-audio-source-identity.json`。

## 验证边界与下轮经验

- 本机本轮未启用 `native-audio`，没有 FMOD SDK/银行/扬声器播放、听感验收；RecordingStudio 记录的是真实消费请求，不能称已听到音效。
- 既有 decorator loop 索引映射、kind0xf 最近距离驱动等仍在现文件标记待核，本轮未扩入整音频恢复。M01 灯笼点亮的 kind14 路径已闭合，不代表这些其它路径完成。
- 音频失败必须分别核创建、位置、查询、启动和释放；不能从某个 stop 的丢弃策略推断 start。共享 helper 需先核原调用合同，再恢复一次，让真实消费者复用。
- Native bridge 的原 owner/初始化状态也是验证输入；漏初始化 state 会把一次性实例误判为循环实例，不能反向修改生产逻辑以迁就错误探针。
- 成功路径仍为每事件 O(1) 分派与原创建/释放，不新增排序、共享锁、跨帧列表或额外克隆；未进行真实 FMOD 性能测量。
