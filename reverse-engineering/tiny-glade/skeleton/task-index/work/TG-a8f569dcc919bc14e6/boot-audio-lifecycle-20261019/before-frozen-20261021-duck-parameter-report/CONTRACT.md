# 原 AudioInstance::set_parameter 错误报告 leaf

TG-a8f569dcc919bc14e6 旧owner boot-audio-lifecycle-20261019 的中央最窄委派；不改生产/Cargo，root专属集成。

原函数0x140a57670完整1838B/385指令，identity绑定pack在TG-fcd5旧work/delegated-core-duck-building-20261021/native-parameter-evidence；PDBmodule548 record62852。仅Active调用FMOD；成功返回；失败先格式化消息，再原log-once去重→Warn，之后继续，不导致Duck构造unwrap。三字符串fragment绑定见同fcd目录instance-parameter-warning-format.json。

原静态报表并非当前GLOBAL_AUDIO_PARAM_ERRORS。原0x143634d68存log_once消息集指针，0x143634d70为Once状态；0x140542890的PDB同地址别名含log_once::__MessagesSet::lock(module1192)，0x14143b8b0是BTreeMap<String,SetValZST>::insert。log-once0.3.1源码明确宏每展开点持有Once+Box<__MessagesSet>，内部Mutex<BTreeSet<String>>；key为stringify!(target)+stringify!(level)+格式化消息，lock poison expect("Mutex was poisonned")。插入先于log::log!的level guard。无需另造表/锁/本地报告镜像。

原B Cargo.lock:4040 exact包log-once0.3.1，校验aba6f00890254f37df9778e2df6042249f543ab1789cbe001c50b4debe264a8c。发布包已下载且哈希完全一致，见dependency-identity.json；发布源码留证dependency-src-lib.rs。仅依赖复用不算业务进度。root须在workspace/CountryCore加入exact依赖并更新lock，再验证，本worker不写共享Cargo/lock。

最窄method hunk调用原warn_once!宏，消息`Failed to set parameter named {} on FMOD event {}\n{:?}`，依次parameter/event_name/error。现EventInstance::Error只有Debug约束，Native libfmod Error原fmt身份为impl$341::fmt；保留现有Result Err返回以维持重构API，Duck原continue调用discard此Err。未引入catch panic/默认成功。

已给Duck真实Studio Pitch失败测试检查后缀构造/前缀释放；log-once本身去重由原依赖复用，不在游戏再抄其算法。不称A/C或原进程执行验证。


## formatter 的原证确认（不是 fragment 推测）

原方法将错误参数的格式化函数指针设为VA0x140a52af0。该函数PDB物理record为module548:18024，code_size486，短名libfmod::impl$341::fmt；对原RNE使用匹配PDB的LLVM symbolizer，明确长名为`<libfmod::Error as core::fmt::Debug>::fmt`，源行libfmod-2.222.6/src/lib.rs:14。该锁定依赖的本机源码第14行为derive(Debug)，Display另在第29行独立实现。完整formatter机器码及7处身份绑定源定位已纳入TG-fcd5旧work/delegated-core-duck-building-20261021/native-parameter-evidence-v2/fmod-error-debug-formatter.json，pack全部ok。因此最后参数使用{:?}已经确认；不是将Display任意换成Debug。

## 当前 caller 策略审计（2026-10-21 before-frozen）

机器可查清单为callsite-inventory.json，保留当前生产src中62处`.set_parameter(...)`或`AudioInstance::set_parameter(...)`、连续多行的完整调用、后续表达式与每源SHA。独立查找直接unwrap/expect（包括跨行）结果0；这不是原版全域native caller等价证明，只是本次代码实际盘点。没有无证删除任何unwrap。

| 实际接收者/源 | 当前策略 | 本leaf影响与边界 |
|---|---|---|
| AudioInstance<E>本体；resources/audio_resources.rs的LoopEntry/LoopInstance两个adapter | 两个都`let _ = AudioInstance::set_parameter(...)` | 参数错误先原warn_once，再adapter继续；不传播到AudioLoop/AudioLoopPlayer的start/volume错误通道 |
| systems/critters/audio.rs的CritterAudioSink blanket | `let _ = AudioInstance::set_parameter(...)`，trait返回unit | Duck/Sheep等AudioHandle调用继续。不要把AudioHandle上无`.unwrap()`误判成吞了一条未恢复的构造失败 |
| 本次DuckAudio::from_audio_core Quack Pitch | `let _ = quack.set_parameter("Pitch",pitch)` | 原warn继续，TakeOff/Land/Flap仍构造；instantiate Err才panic。随包Pitch失败真实Studio测试覆盖后缀构造（未运行） |
| 本次DuckFlapLoop参数转发 | current_mut().set_parameter的Result discard | 原loop/current参数错误warn继续；不误停止环或触发新实例创建 |
| utils/audio_utils.rs的loop=1/0 | 调LoopEntry::set_parameter，unit | 上述AudioInstance adapter同步discard，环start/stop另有Result，与此参数错误不同 |
| systems/audio/terrain_stairs_audio、terrain_audio、roof_audio、sheep_audio、playermade_stairs_audio、wall_audio_fmod、garden_audio、tree_audio、clutter_audio、timberframe_audio、wall_audio_ui、building_audio中的instance/current/entries句柄 | 实际调用均`let _ = ...set_parameter(...)`（含多行） | 此次新增原warn_once后继续，没有新增早退/panic；各模块完整业务与其余音频调用的native失败规则未在本任务全域复核，不虚报完成 |
| startup_sheep.rs:100 | AudioHandle::set_parameter，unit | 继承上述blanket策略；用户dirty文件保持未动 |
| time_of_day.rs的TodAudioParameters；music/audio_core_backend.rs的MusicBackend for AudioCore；audio_host.rs/photo_audio；各systems/audio中`core.set_parameter` | **AudioCore全局参数**或其全局trait，非AudioInstance | 不受本leaf影响。MusicBackend全局Result交music_system helper Warn继续；NativeMusicBackend的底层Studio方法与此实例参数leaf不同。未删除其任何失败传播 |
| resources/native_audio.rs及systems/audio/music/native.rs的Studio::set_parameter_by_name/global parameter | 原生底层adapter，非本方法 | 继续保留其Result；是否还有全局错误报告差异由其源owner复核，本包不修改 |
| duck.rs的_audio_sink_contract | 未调用的trait示例函数 | 不充当真实producer或业务验收；不据此提高完成度 |

原生库接口或未来外部crate可能显式unwrap该public Result，不在本仓src盘点范围。为保手工架构不破坏公开签名，这次仍返回Err；当前实际游戏消费者已确认discard继续。如果root将来发现实际caller.unwrap，须为那条原路径独立取证后修复，不能全域把unwrap替换默认成功。
