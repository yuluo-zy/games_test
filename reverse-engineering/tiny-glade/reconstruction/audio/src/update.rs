//! country-core systems/audio：事件→FMOD 系统草稿（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/audio-events-analysis.md（2026-10-13 reva/radare2 精读）。
//! 依赖复用：bevy_ecs（Res/ResMut/EventReader/App/add_systems/register_event）、glam。
//! FMOD（libfmod）调用只标边界 `[fmod]`，不还原。本文件为"事件读取→构造 FMOD 调用→发送"骨架，
//! 具体 FMOD 调用以 `_audio_core` / `_audio_loop` 占位函数标注证据，实现在 reconstruction 的
//! audio_resources 草稿中。

use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::events::*;

// ===== mod.rs：add_audio_events / add_audio_systems（事件注册骨架）=====

/// 注册 16 个音频事件（add_audio_events，VA 0x140941450，源码行 179–196）。
/// 顺序即反汇编 register_event 调用顺序（证据，见 analysis §1a）。
/// 除 events.rs 定义的类型外，其余事件类型（PlayTerrainSound 等在 terrain_audio、
/// SuppressPathSound 在 terrain_stairs_audio、PlayReplayAudio 在 replay_audio、…）位于各自
/// 模块草稿，本文件仅按注册顺序引用占位。
pub fn add_audio_events(app: &mut App) {
    app.add_event::<PlayTerrainSound>()
        .add_event::<SuppressPathSound>()
        .add_event::<PlayReplayAudio>()
        .add_event::<PlayDecoratorSoundCmd>()
        .add_event::<PlayStairsAudioCmd>()
        .add_event::<PlayWallUiSound>()
        .add_event::<PlayWallAudio>()
        .add_event::<RequestWallMaterialAudio>()
        .add_event::<WallBendAudioParamReset>()
        .add_event::<PlayTimberframeSound>()
        .add_event::<PlaySheepReactSound>()
        .add_event::<PlayMenuUiSound>()
        .add_event::<PlayBuildingAudio>()
        .add_event::<PlayElevationSupportSoundCmd>()
        .add_event::<PlayClutterSoundCmd>()
        .add_event::<PlayTreeSoundCmd>();
    // 反汇编中每个事件先 get_resource_id 查组件表、未注册才 register_event：
    // bevy add_event 幂等逻辑，依赖内部，复用不还原。
}

/// 注册音频系统（add_audio_systems，VA 0x1409411a0，源码行 95–142）。
/// 证据：两次 add_systems + 两个 run_if。music_debug_gui 与 water 系列系统在本函数注册；
/// 其余音频系统（wall/building/stairs/…）由其他模块注册（unknown，见 analysis §9）。
pub fn add_audio_systems(app: &mut App) {
    app.add_systems(Update, music_debug_gui)
        .add_systems(
            Update,
            (water_audio_system.run_if(water_audio_condition),
             water_ambience_audio_system.run_if(water_ambience_condition)),
        );
}

// 下列符号仅为占位（真实系统在对应模块），此处不实现：
pub fn music_debug_gui() {}
pub fn water_audio_system() {}
pub fn water_ambience_audio_system() {}
pub fn water_audio_condition() -> bool {
    false
}
pub fn water_ambience_condition() -> bool {
    false
}

// ===== audio_update.rs：每帧音频更新 =====

/// 音频设置资源（Res<AudioSettings>）。
/// 证据：VCA 音量字段 +0xf8（Master）/ +0xfc（Music/Music-DawnJingle）/ +0x100（SFX）/
/// +0x104（Amb），均 f32。
pub struct AudioSettings {
    pub vca_master: f32,
    pub vca_music: f32,
    pub vca_sfx: f32,
    pub vca_amb: f32,
}

/// 每帧音频更新（audio_update_system，VA 0x1409c8dd0，源码行 11–34）。
/// ABI：arg1=Res<AudioCore>，arg2=Res<听者变换资源>，arg3=Res<AudioSettings>。
pub fn audio_update_system(
    audio_core: Res<AudioCore>,
    listener_transform: Res<[u8; 0]>, // unknown：+0xc0/+0xd0/+0xe0 变换矩阵
    settings: Res<AudioSettings>,
) {
    // 1) VCA 音量（证据：set_vca_volume(AudioCore, &"vca:/…", len, vol)，0x140a54810）。
    set_vca_volume(&audio_core, "vca:/Master", settings.vca_master);
    set_vca_volume(&audio_core, "vca:/Music", settings.vca_music);
    set_vca_volume(&audio_core, "vca:/Music-DawnJingle", settings.vca_music);
    set_vca_volume(&audio_core, "vca:/SFX", settings.vca_sfx);
    set_vca_volume(&audio_core, "vca:/Amb", settings.vca_amb);
    // 2) 听者位置：arg2 变换 +0xc0..+0xf0 → Vec3，AudioCore::update（0x140a53a00）。
    let pos = compute_listener_position(&listener_transform); // [evidence] SIMD 3x3+offset
    audio_core_update(&audio_core, pos);
    // 3) puffin scope（are_scopes_on/begin_scope/end_scope）为依赖，省略。
}

/// 启动期音频更新（boot_audio_update_system，VA 0x1409c9320，源码行 37–54）。
/// ABI：arg1=Res<AudioCore>，arg2=Res<AudioSettings>（无听者资源）。
pub fn boot_audio_update_system(audio_core: Res<AudioCore>, settings: Res<AudioSettings>) {
    // VCA 段与 audio_update_system 逐条一致。
    set_vca_volume(&audio_core, "vca:/Master", settings.vca_master);
    set_vca_volume(&audio_core, "vca:/Music", settings.vca_music);
    set_vca_volume(&audio_core, "vca:/Music-DawnJingle", settings.vca_music);
    set_vca_volume(&audio_core, "vca:/SFX", settings.vca_sfx);
    set_vca_volume(&audio_core, "vca:/Amb", settings.vca_amb);
    // 听者：固定默认位姿（0 化 + -1.0 + 1.0 常量），AudioCore::update。
    audio_core_update(&audio_core, Vec3::new(0.0, 0.0, 0.0));
}

// ===== wall_audio_fmod.rs：墙体音效（代表事件系统①）=====

/// 初始化（wall_audio_init_system，VA 0x14094fb80，源码行 40–94）。
/// ABI：arg1=Res<AudioCore>，arg2=App。
pub fn wall_audio_init_system(audio_core: Res<AudioCore>, app: &mut App) {
    let mut state = WallAudioState::default();
    // 10 个循环事件（证据：AudioCore::instantiate_loop(desc{name,len,count=3,max=0x32})）。
    for (i, name) in [
        "event:/SFX/Wall Draw",
        "event:/SFX/Wall Change Height",
        "event:/SFX/FenceExtend",
        "event:/SFX/LintelExtend",
        "event:/SFX/StoneArchExtend",
        "event:/SFX/WideningGate",
        "event:/SFX/PlatformStairsExtend",
        "event:/SFX/SewerGateWiden",
        "event:/SFX/FenceHeight",
        "event:/SFX/FenceMove",
    ]
    .iter()
    .enumerate()
    {
        instantiate_loop(&audio_core, name, &mut state.loops[i]); // [evidence] desc{name,len,3,0x32}
    }
    app.insert_resource(state); // insert_resource<WallAudioState> 0x1412df4a0
}

/// 主系统（wall_audio_fmod，VA 0x140950920，源码行 134–…）。
/// ABI：arg1=Res<AppMode>（+0 u32==13），arg2=Res<AudioCore>，arg3=EventReader<PlayWallAudio>，
/// arg4=ResMut<WallAudioState>；音量源为栈参（entry_rsp+0x40，deref 后 +0x1c f32 /
/// [0x1429d18cc]），AppMode==13 时音量=1.0 常量。全量参数清单待钉死（unknown）。
pub fn wall_audio_fmod(
    _app_mode: Res<[u8; 0]>, // [evidence] +0 u32；+0x1c 仅经栈参音量源读取
    audio_core: Res<AudioCore>,
    mut events: EventReader<PlayWallAudio>,
    mut state: ResMut<WallAudioState>,
    _volume_source: Res<[u8; 0]>, // [evidence] 栈参，[+0x1c] f32 音量源
) {
    let volume = normalized_volume(&_volume_source); // [evidence] xmm14 = [r+0x1c] / 常量
    for ev in events.read() {
        if ev.kind_is_loop() {
            // 循环类：request_play(loop, position, volume, event+0x1c→set_volume)
            let loop_i = loop_index_for_kind(&ev); // [evidence] state+0x0/0x40/… 引用
            request_play(&mut state.loops[loop_i], ev.position, ev.volume, volume);
        } else {
            // 瞬发类：instantiate_audio_w_position_and_volume(core, name, len, pos, volume)
            let name = wall_event_name(&ev); // [evidence] .data 事件名表 0x142ab41c8 起
            instantiate_audio_w_position_and_volume(
                &audio_core, name, ev.position, ev.volume);
        }
    }
    // 帧尾：按"应播"标志 start_if_not_playing/stop，清标志；+0x280 音量调制。
    for i in 0..state.loops.len() {
        if state.playing_flags[i] {
            start_if_not_playing(&state.loops[i]);
        } else {
            stop_loop(&state.loops[i]);
        }
        state.playing_flags[i] = false;
    }
    state.volume_mod = volume_modulate(state.volume_mod); // [evidence] powf 时间项
}

// ===== building_audio.rs：建筑音效（代表事件系统②）=====

/// 主系统（building_audio，VA 0x1417d2ad0，源码行 101–…）。
/// ABI：arg1=ResMut<BuildingAudioState>，arg2=EventReader<PlayBuildingAudio>，
/// arg3=Res<AudioCore>，arg4=Res<构建数据表>。
pub fn building_audio(
    mut _state: ResMut<BuildingAudioState>,
    mut events: EventReader<PlayBuildingAudio>,
    audio_core: Res<AudioCore>,
    buildings: Res<[u8; 0]>, // [evidence] hash_one<RandomState,GlobalId> 查 0x2a8 条目
) {
    for ev in events.read() {
        // 按 id 查构建条目（hashbrown，值 0x2a8）；条件 [entry-0x290]>=2 且 [entry-0x270]>0。
        let Some(entry) = building_lookup(&buildings, ev.id) else {
            continue;
        };
        // kind→(事件名,长度)（表 0x142ce4c88/0x142ce4ca0）。
        let name = building_event_name(ev.kind); // [evidence] DetachFromGround/AttachToGround/PeakReached
        let inst = instantiate_audio(&audio_core, name); // 0x140a54440
        set_position(&inst, ev.position); // 0x140a57da0
        set_parameter(&inst, "editing_speed", ev.editing_speed); // 0x140a57670
        let size = building_size(&entry, ev.kind); // [evidence] area + inverse_lerp
        set_parameter(&inst, "BuildingSize", size);
        start_if_not_playing(&inst); // 0x140a57300
    }
}

// ===== playermade_stairs_audio.rs：楼梯音效（代表事件系统③）=====

/// 初始化（stairs_audio_init_system，VA 0x140f878d0，源码行 28–…）。
pub fn stairs_audio_init_system(audio_core: Res<AudioCore>, app: &mut App) {
    let mut state = StairsAudioState::default();
    for (i, name) in [
        "event:/SFX/Stairs/StairMove",
        "event:/SFX/Stairs/StairChangeHeight",
        "event:/SFX/Stairs/StairChangeWidth",
        "event:/SFX/Stairs/LadderMove",
        "event:/SFX/Stairs/LadderChangeHeight",
        "event:/SFX/Stairs/LadderChangeWidth",
    ]
    .iter()
    .enumerate()
    {
        instantiate_loop(&audio_core, name, &mut state.loops[i]);
    }
    app.insert_resource(state); // insert_resource<StairsAudioState> 0x1412e04f0
}

/// 主系统（stairs_audio_system，VA 0x140f880a0，源码行 140–318）。
/// ABI：arg1=ResMut<StairsAudioState>，arg2=EventReader<PlayStairsAudioCmd>，
/// arg3=Res<元素表>。
pub fn stairs_audio_system(
    mut state: ResMut<StairsAudioState>,
    mut events: EventReader<PlayStairsAudioCmd>,
    _elements: Res<[u8; 0]>,
) {
    // 帧首停掉全部 6 loop。
    for l in state.loops.iter_mut() {
        stop_loop(l);
    }
    for ev in events.read() {
        if ev.kind == 0 {
            // StairMove 循环路径：start_if_not_playing(state+0x0) + 参数。
            start_if_not_playing(&state.loops[0]);
            continue;
        }
        // 瞬发路径：instantiate_audio + set_position + set_parameter(stair_width/size) + start。
        let name = stair_event_name(ev.kind); // [evidence] 0x142bad6c8 起事件名表
        let inst = instantiate_audio(&audio_core_from(&_elements), name);
        set_position(&inst, ev.position);
        set_parameter(&inst, "stair_width", stair_width_param(ev, &_elements)); // 0x142bad818
        if let Some(size) = stair_size_param(ev.kind, ev.field_c) {
            set_parameter(&inst, "size", size); // 0x142bad823
        }
        start_if_not_playing(&inst);
    }
}

// ===== 占位：FMOD 胶水 consumer（真实实现见 audio_resources / wall_audio_utils 草稿）=====

pub fn set_vca_volume(_core: &AudioCore, _vca: &str, _volume: f32) {}
pub fn audio_core_update(_core: &AudioCore, _listener: Vec3) {}
pub fn instantiate_loop(_core: &AudioCore, _name: &str, _out: &mut AudioLoopPlayer) {}
pub fn request_play(_loop: &mut AudioLoopPlayer, _pos: Vec3, _set_volume: f32, _editing_speed: f32) {}
pub fn instantiate_audio_w_position_and_volume(
    _core: &AudioCore, _name: &str, _pos: Vec3, _volume: f32,
) {
}
pub fn instantiate_audio(_core: &AudioCore, _name: &str) -> AudioInstance {
    AudioInstance::default()
}
pub fn set_position(_inst: &AudioInstance, _pos: Vec3) {}
pub fn set_parameter(_inst: &AudioInstance, _name: &str, _value: f32) {}
pub fn start_if_not_playing(_inst: &AudioInstance) {}
pub fn stop_loop(_inst: &AudioInstance) {}

// ---- [evidence] 辅助（证据位，语义 unknown）----

fn normalized_volume(_src: &[u8; 0]) -> f32 {
    0.0
}
fn compute_listener_position(_t: &[u8; 0]) -> Vec3 {
    Vec3::ZERO
}
fn loop_index_for_kind(_ev: &PlayWallAudio) -> usize {
    0
}
fn wall_event_name(_ev: &PlayWallAudio) -> &str {
    ""
}
fn building_lookup<'a>(_b: &'a [u8; 0], _id: u64) -> Option<&'a [u8]> {
    None
}
fn building_event_name(_kind: u8) -> &str {
    ""
}
fn building_size(_entry: &[u8], _kind: u8) -> f32 {
    0.0
}
fn stair_event_name(_kind: u8) -> &str {
    ""
}
fn stair_width_param(_ev: &PlayStairsAudioCmd, _e: &[u8; 0]) -> f32 {
    0.0
}
fn stair_size_param(_kind: u8, _f: f32) -> Option<f32> {
    None
}
fn volume_modulate(v: f32) -> f32 {
    v
}
fn audio_core_from(_e: &[u8; 0]) -> AudioCore {
    AudioCore::default()
}
