# resources/audio_resources.rs

> 文件作用：**音频核心状态：FMOD Studio 内核、银行表、AudioInstance 实例模型、全局失败报告注册表。**

函数总计 47：业务 19 / 内联无独立符号 20 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::audio_resources::AudioCore::visualize_sounds | 0x140a55ff0 | 0 | 调试可视化：遍历活动实例→位置→画 3D 标记（thread_local 访问） | O | audio-misc\decomp\decomp-0x140a55ff0.txt |
| country_core::resources::audio_resources::Bank::load | 0x140a53640 | 35 | loaded&1 早退；Studio::load_bank_file(path@+0x10)；失败→panic "Error loading FMOD bank file: {path} {err}"(fmt 0x142ad25b0)；成功 loaded=1+句柄@+8 | O | audio-misc\decomp\decomp-0x140a53640.txt |
| country_core::resources::audio_resources::Bank::load_sample_data | 0x140a53870 | 48 | Bank::load_sample_data+日志 "Loading sample data of FMOD bank " | O | audio-misc\decomp\decomp-0x140a53870.txt |
| country_core::resources::audio_resources::AudioCoreInner::update | 0x140a53a00 | 181 | 0x140a53a00（PDB 名 DelayedBankLoader::load 错位）：听者三断言(|forward|-1|/|up|-1|/dot<0.001)→set_listener_attributes(listener=0)→Studio::update；失败 log "FMOD update error: " | O | audio-misc\decomp\decomp-0x140a53a00.txt |
| country_core::resources::audio_resources::AudioCoreInner::set_parameter | 0x140a53db0 | 205 | Studio::set_parameter_by_name；失败→全局报告注册表(槽 0x143634d60/d58)：module_p_<名>path!(…)::Warn:<err>；锁失败 panic "Mutex was poisonned"；anyhow "Couldn't set global FMOD parameter " | O | audio-misc\decomp\decomp-0x140a53db0.txt |
| country_core::resources::audio_resources::AudioCoreInner::instantiate_audio | 0x140a54440 | 213 | Studio::get_event→create_instance；Active{event 8B@+8,name cap-first{cap@+0x10,ptr@+0x18,len@+0x20},state=2@+0x28}；失败 Err(anyhow "FMOD error: "/"get_event(") | O | audio-misc\decomp\decomp-0x140a54440.txt |
| country_core::resources::audio_resources::AudioCoreInner::set_vca_volume | 0x140a54810 | 253 | 0x140a54810（PDB 名 instantiate_audio::closure$0 错位）：Studio::get_vca→Vca::set_volume（unwrap panic） | O | audio-misc\decomp\decomp-0x140a54810.txt |
| country_core::resources::audio_resources::AudioCore::new | 0x140a54a40 | 266 | AudioCore::new：Studio::create→get_core_system→set_software_format(Mixer 日志)→initialize→扫 assets/audio/Desktop .bank→HashMap<String,Bank> extend→每银行 load+load_sample_data（WP14 合同 O） | O |  |
| country_core::resources::audio_resources::AudioCore::set_parameter | 0x140a55f20 | 314 | 0x140a57670（PDB 名 pause 错位）：签名(&self,name:&str,f32)→EventInstance::set_parameter_by_name；失败同全局报告机制 | O |  |
| country_core::resources::audio_resources::AudioCore::instantiate_audio | 0x140a55f40 | 321 | AudioCore::instantiate_audio：core[+0] 非空→Inner 路径；空→Disabled 变体（判 0） | O |  |
| country_core::resources::audio_resources::AudioSamplesLoadedPromise::recv | 0x140a57100 | 471 | 0x140a57100：crossbeam 三通道(array/list/zero) recv_timeout(1s)；超时丢 payload 正常返回；断开→unwrap panic | O | audio-misc\decomp\decomp-0x140a57100.txt |
| country_core::resources::audio_resources::AudioInstance::start | 0x140a57240 | 505 | 判别==1→EventInstance::start（0x1414372d0） | O |  |
| country_core::resources::audio_resources::AudioInstance::start_if_not_playing | 0x140a57300 | 515 | 判别&1→get_playback_state→位掩码 0xb（{Playing,Sustaining,Starting} 跳过）→start | O | audio-misc\decomp\decomp-0x140a57300.txt |
| country_core::resources::audio_resources::AudioInstance::stop | 0x140a57470 | 532 | 判别==1→EventInstance::stop(0=STOP_IMMEDIATE)（0x140a57470） | O |  |
| country_core::resources::audio_resources::AudioInstance::stop_on_drop | 0x140a57540 | 554 | Drop：判别==1→stop | O | audio-misc\decomp\decomp-0x140a57540.txt |
| country_core::resources::audio_resources::AudioInstance::is_playing | 0x140a57550 | 591 | get_playback_state==Playing（0x140a57550） | O |  |
| country_core::resources::audio_resources::impl$10::set_volume | 0x140a575a0 | 615 | EventInstance::set_volume（0x140a575a0） | O |  |
| country_core::resources::audio_resources::impl$10::set_parameter | 0x140a57670 | 629 | AudioInstance::set_parameter(&str,f32)（PDB 名 pause 错位）：set_parameter_by_name；失败→全局报告槽 0x143634d60 | O | audio-misc\decomp\decomp-0x140a57670.txt |
| country_core::resources::audio_resources::impl$10::set_position | 0x140a57da0 | 642 | 判别==1→set_3d_attributes：position=入参，forward=(0,0,-1)/up=(0,1,0)/velocity=0(常量 0x142ad0460)；失败 anyhow "Failed to set 3d attributes for FMOD event "+报告槽 0x143634d80/d78 | O | audio-misc\decomp\decomp-0x140a57da0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::audio_resources::DelayedBankLoader::load`（行 64）— IO/解析
- `country_core::resources::audio_resources::AudioCoreInner::new_impl`（行 101）— 构造
- `country_core::resources::audio_resources::AudioCoreInner::set_master_volume`（行 233）— 设置/写入
- `country_core::resources::audio_resources::AudioCoreInner::set_music_volume`（行 237）— 设置/写入
- `country_core::resources::audio_resources::AudioCoreInner::set_dawn_jingle_volume`（行 241）— 设置/写入
- `country_core::resources::audio_resources::AudioCoreInner::set_sfx_volume`（行 245）— 设置/写入
- `country_core::resources::audio_resources::AudioCoreInner::set_ambience_volume`（行 249）— 设置/写入
- `country_core::resources::audio_resources::AudioCore::update`（行 308）— 待推断
- `country_core::resources::audio_resources::AudioCore::set_master_volume`（行 333）— 设置/写入
- `country_core::resources::audio_resources::AudioCore::set_music_volume`（行 339）— 设置/写入
- `country_core::resources::audio_resources::AudioCore::set_sfx_volume`（行 348）— 设置/写入
- `country_core::resources::audio_resources::AudioCore::set_ambience_volume`（行 354）— 设置/写入
- `country_core::resources::audio_resources::AudioInstance::stop_immediately`（行 544）— 生命周期控制
- `country_core::resources::audio_resources::AudioInstance::pause`（行 562）— 生命周期控制
- `country_core::resources::audio_resources::AudioInstance::is_paused`（行 572）— 谓词/校验
- `country_core::resources::audio_resources::AudioInstance::resume`（行 581）— 生命周期控制
- `country_core::resources::audio_resources::AudioInstance::get_timeline_position`（行 603）— 查询/计算
- `country_core::resources::audio_resources::impl$10::has_parameter`（行 621）— 谓词/校验
- `country_core::resources::audio_resources::fmod_to_anyhow`（行 680）— 待推断
- `country_core::resources::audio_resources::impl$14::to_fmod`（行 689）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default`、`drop` 等 8 项
