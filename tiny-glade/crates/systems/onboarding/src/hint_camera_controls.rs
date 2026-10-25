//! hint_camera_controls（O：enable/display 条件 + 三个相机提示构造）。
//!
//! `hint_camera_enable_condition` 0x14227c780（396）：更新 CameraHintsState
//! （[0]=state int，[1..4] f32 计数器，阈值 100.0/10.0/0.1/1.5）。鼠标/相机移动事件
//! 累加计数器，判定满足则 `*state=1`。真逻辑（fresh 0x14227c780，batch4-camera）：
//! ```text
//! if *state != 0 return;
//! // 鼠标事件（CursorMovedScreenSpace）：有→counters[1]=0，counters[0]+=dt；
//! //   无→counters[1]+=dt，<10 才 counters[0]+=dt
//! // 相机事件（NotifyHintSystemCameraMoved）：有→counters[3]=0，counters[2]+=dt；
//! //   无→counters[3]+=dt
//! // counters[2]>0.1 ? (counters[3]>1.5 || counters[0]>100) : counters[0]>100 → *state=1
//! ```
//! `hint_camera_display_condition` 0x14227c910（1646）：state==1 才继续；三满足
//! （profile 0x1f6/0x1f7/0x1f8）→ state=2；AppMode（1/2/6+shape1/2，关闭吸附时
//! 另含 10）门控返回；逐条 [(0xa,pan),(0xb,rotate),(0xc,zoom)]：进度未跨 1.0 → 调
//! camera_*_hint 写 DisplayHintRequest（id 0xa/0xb/0xc）+ 递减 duration；duration<=0
//! 且完成标志 → 写 PlayUiSound（kind 0x13）+ 清标志。
//! `camera_pan_hint` 0x14227d000 / `camera_rotate_hint` 0x14227d2f0 / `camera_zoom_hint`
//! 0x14227d680：读 profile 0x1c0/0x1bc 字形模式 → alternate/together/adjacent 组合
//! （hint/mouse、hint/rmb、hint/rmb-move、hint/mmb、hint/mmb-move、hint/scroll）。
//!
//! 三个构造器已按原14227d000/d2f0/d680恢复完整绑定树：键位复用档案权威owner，
//! 动画图是一个三帧Graphic（每帧0.7秒、高度100），不是三个并列提示。

use bevy_ecs::prelude::*;
use country_core::game_profile::GameProfile;
use country_core::resources::time::GameTime;
use country_core::resources::window::CursorMovedScreenSpace;

use crate::BindingHintDisplay;
use crate::hint_orchestrator::{DisplayHintRequest, HintType, steam_query};

/// 相机提示状态（O：CameraHintsState 20B = state i32@+0 + 4×f32 计数器@+4..+0x14）。
/// counters[0] 鼠标活跃累计 / [1] 自鼠标移动时间 / [2] 相机活跃累计 / [3] 自相机移动时间。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct CameraHintsState {
    pub state: i32,
    pub counters: [f32; 4],
}

impl Default for CameraHintsState {
    fn default() -> Self {
        Self {
            state: 0,
            counters: [0.0; 4],
        }
    }
}

/// 原8B Local：完成后的0.4秒保留窗口，以及“三类均完成”提示音待发标志。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HintCompleteCooldown {
    pub duration: f32,
    pub all_complete_pending: bool,
}

/// 原14227d020、d313/d32c、d69c/d6b2的动作参数。
const STEAM_PAN_ACTION: u8 = 0;
const STEAM_ROTATE_ACTION: u8 = 1;
const STEAM_ZOOM_ACTION_A: u8 = 16;
const STEAM_ZOOM_ACTION_B: u8 = 17;

/// hint_camera_enable_condition（O 0x14227c780）：见模块头真逻辑。**判等**
/// （fresh 2026-10-17；counters 映射与四分支逐语句一致）。
pub fn hint_camera_enable_condition(
    mut state: ResMut<CameraHintsState>,
    mut mouse: EventReader<CursorMovedScreenSpace>,
    mut camera: EventReader<country_core::resources::onboarding::NotifyHintSystemCameraMoved>,
    time: Res<GameTime>,
) {
    if state.state != 0 {
        return;
    }
    let dt = time.delta;
    // 鼠标：有事件 → [1]=0, [0]+=dt；无事件 → [1]+=dt，<10 才 [0]+=dt。
    if mouse.read().next().is_some() {
        state.counters[1] = 0.0;
        state.counters[0] += dt;
    } else {
        state.counters[1] += dt;
        if state.counters[1] < 10.0 {
            state.counters[0] += dt;
        }
    }
    // 相机：有事件 → [3]=0, [2]+=dt；无事件 → [3]+=dt。
    if camera.read().next().is_some() {
        state.counters[3] = 0.0;
        state.counters[2] += dt;
    } else {
        state.counters[3] += dt;
    }
    let should_show = if state.counters[2] > 0.1 {
        state.counters[3] > 1.5 || state.counters[0] > 100.0
    } else {
        state.counters[0] > 100.0
    };
    if should_show {
        state.state = 1;
    }
}

/// 原14227c910：状态、AppMode 与 Modifiers 门控；读取首个在场提示并处理跨阈值与退出冷却。
/// 完成事件由前序hint_camera_progress_update产生，这里只负责Display及最终完成音。
pub fn hint_camera_display_condition(
    mode: Res<country_core::systems::mode_manager::AppMode>,
    profile: Res<GameProfile>,
    mut request: EventWriter<DisplayHintRequest>,
    mut state: ResMut<CameraHintsState>,
    query: Query<&HintType>,
    modifiers: Res<country_core::systems::backend::input::mod_key::Modifiers>,
    progress: Res<crate::hint_camera_progress::CameraHintProgress>,
    mut cooldown: Local<HintCompleteCooldown>,
    mut sounds: EventWriter<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    time: Res<GameTime>,
) {
    if state.state != 1 {
        return;
    }
    if profile.camera_pan_completed
        && profile.camera_rotate_completed
        && profile.camera_zoom_completed
    {
        // 原本帧仍继续处理模式/提示，下一帧state!=1才退出。
        state.state = 2;
    }
    let shape = match *mode {
        country_core::systems::mode_manager::AppMode::Wall(shape) => shape as u8,
        _ => 0,
    };
    if !camera_display_allowed(mode.native_tag(), shape, modifiers.disable_snapping()) {
        return;
    }
    let (display, complete_sounds) = camera_display_step(
        query.iter().next().copied(),
        [progress.pan, progress.rotate, progress.zoom],
        [progress.prev_pan, progress.prev_rotate, progress.prev_zoom],
        &mut cooldown,
        time.delta,
    );
    for _ in 0..complete_sounds {
        sounds.write(
            country_core::systems::ui::resources::widget_params::PlayUiSound {
                kind: 0x13,
                _payload: [0; 20],
            },
        );
    }
    for (index, id) in [
        HintType::CameraPan,
        HintType::CameraRotate,
        HintType::CameraZoom,
    ]
    .into_iter()
    .enumerate()
    {
        if display[index] {
            let binding = match index {
                0 => camera_pan_hint(&profile),
                1 => camera_rotate_hint(&profile),
                2 => camera_zoom_hint(),
                _ => unreachable!(),
            };
            request.write(DisplayHintRequest::new(binding, id, 1));
        }
    }
}

fn camera_display_allowed(mode: u32, shape: u8, disable_snapping: bool) -> bool {
    !(mode == 1 || mode == 2 || (mode == 6 && matches!(shape, 1 | 2)) || (disable_snapping && mode == 10))
}

/// 原逐类次序不能合并：一个共享Local仅对当前首提示递减一次，不在场时保持原值。
fn camera_display_step(
    first_hint: Option<HintType>,
    current: [f32; 3],
    previous: [f32; 3],
    cooldown: &mut HintCompleteCooldown,
    dt: f32,
) -> ([bool; 3], usize) {
    let mut display = [false; 3];
    let mut sounds = 0;
    for (index, id) in [
        HintType::CameraPan,
        HintType::CameraRotate,
        HintType::CameraZoom,
    ]
    .into_iter()
    .enumerate()
    {
        let mut keep = false;
        if first_hint == Some(id) {
            if previous[index] < 1.0 && current[index] >= 1.0 {
                cooldown.duration = 0.4;
                cooldown.all_complete_pending = current.iter().all(|value| *value >= 1.0);
            }
            cooldown.duration -= dt;
            keep = cooldown.duration > 0.0;
            // 原ucomiss+ja：NaN也沿“不大于零”分支，不能改成duration<=0。
            if !keep && cooldown.all_complete_pending {
                sounds += 1;
                cooldown.all_complete_pending = false;
                continue;
            }
        }
        display[index] = keep || current[index] < 1.0;
    }
    (display, sounds)
}

/// 原三帧鼠标手势，每帧0.7秒；路径直接来自B静态字符串表1430385c4起。
fn animated_mouse_hint(middle: bool) -> crate::hint_orchestrator::HintGraphicParams {
    let (button, moving) = if middle {
        ("hint/mmb", "hint/mmb-move")
    } else {
        ("hint/rmb", "hint/rmb-move")
    };
    crate::hint_orchestrator::HintGraphicParams::animated(
        vec!["hint/mouse".into(), button.into(), moving.into()],
        0.7,
        100.0,
    )
}

/// 当前档案接口允许未绑定鼠标；保留未绑定语义，不替换为默认左键。
fn mouse_binding(button: Option<crate::MouseButton>) -> BindingHintDisplay {
    button.map_or(BindingHintDisplay::Empty, BindingHintDisplay::Mouse)
}

/// B @14227d000：Steam平移或真实W/A/S/D行，再按档案选择鼠标动画或自定键。
pub fn camera_pan_hint(profile: &GameProfile) -> BindingHintDisplay {
    let keys = &profile.key_bindings.bindings;
    let base = BindingHintDisplay::Steam(steam_query(country_core::resources::steam::packed_set(
        STEAM_PAN_ACTION,
    )))
    .alternate([keys[0], keys[2], keys[1], keys[3]]);
    match profile.key_bindings.mouse[1] {
        Some(crate::MouseButton::Right) => base.alternate(animated_mouse_hint(false)),
        Some(crate::MouseButton::Middle) => base.alternate(animated_mouse_hint(true)),
        _ => base.alternate(mouse_binding(profile.key_bindings.mouse[1])),
    }
}

/// B @14227d2f0：Steam动作组合/鼠标手势/旋转左右两键为三个替代方案。
pub fn camera_rotate_hint(profile: &GameProfile) -> BindingHintDisplay {
    let base = BindingHintDisplay::Steam(steam_query(
        country_core::resources::steam::packed_set_plus_one(STEAM_ROTATE_ACTION),
    ))
    .together(steam_query(country_core::resources::steam::packed_set(
        STEAM_ROTATE_ACTION,
    )));
    let with_mouse = match profile.key_bindings.mouse[0] {
        Some(crate::MouseButton::Right) => base.alternate(animated_mouse_hint(false)),
        Some(crate::MouseButton::Middle) => base.alternate(animated_mouse_hint(true)),
        _ => base.alternate(mouse_binding(profile.key_bindings.mouse[0])),
    };
    let keys = &profile.key_bindings.bindings;
    with_mouse.alternate(BindingHintDisplay::Key(keys[6]).adjacent(keys[7]))
}

/// B @14227d680：Steam放大/缩小相邻；滚轮静态图作为另一方案。
pub fn camera_zoom_hint() -> BindingHintDisplay {
    BindingHintDisplay::Steam(steam_query(
        country_core::resources::steam::packed_set_plus_one(STEAM_ZOOM_ACTION_A),
    ))
    .adjacent(steam_query(
        country_core::resources::steam::packed_set_plus_one(STEAM_ZOOM_ACTION_B),
    ))
    .alternate(crate::hint_orchestrator::HintGraphicParams::image(
        "hint/scroll",
        100.0,
    ))
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::hint_orchestrator::HintGraphicImages;
    use country_core::resources::steam::ActionOriginGlyphQuery as SteamQuery;

    #[test]
    fn camera_constructors_match_complete_native_instruction_trees() {
        fn flatten(tree:&BindingHintDisplay,keys:&[crate::KeyCode;8],out:&mut Vec<String>) {
            match tree {
                BindingHintDisplay::Alternate(v)|BindingHintDisplay::Adjacent(v)|BindingHintDisplay::Together(v)=>{
                    out.push(match tree {BindingHintDisplay::Alternate(_)=>"Alternate",BindingHintDisplay::Adjacent(_)=>"Adjacent",_=>"Together"}.into());
                    for child in v {flatten(child,keys,out);}out.push("End".into());
                }
                BindingHintDisplay::Steam(query)=>out.push(format!("Steam:{}",query.0)),
                BindingHintDisplay::Row(row)=>{out.push("Row".into());for key in row {flatten(&BindingHintDisplay::Key(*key),keys,out);}out.push("End".into());}
                BindingHintDisplay::Key(Some(key))=>out.push(format!("Key:{}",keys.iter().position(|k|k==key).unwrap())),
                BindingHintDisplay::Mouse(mouse)=>{let (tag,value)=match mouse {crate::MouseButton::Left=>(0,0),crate::MouseButton::Right=>(1,0),crate::MouseButton::Middle=>(2,0),crate::MouseButton::Back=>(3,0),crate::MouseButton::Forward=>(4,0),crate::MouseButton::Other(v)=>(5,*v)};out.push(format!("Mouse:{tag}:{value}"));}
                BindingHintDisplay::Graphic(g)=>match &g.images {
                    HintGraphicImages::Image(path)=>out.push(format!("Image:{path}:{}",g.height)),
                    HintGraphicImages::Animation{frames,seconds_per_frame}=>{out.push(format!("Animation:{seconds_per_frame}:{}",g.height));out.extend(frames.clone());out.push("End".into());}
                },
                other=>panic!("unexpected native constructor leaf {other:?}"),
            }
        }
        let rows:Vec<(u8,u16,Vec<String>)>=ron::from_str(include_str!("../../../../tests/fixtures/native/hint-camera-constructors.ron")).unwrap();
        let keys=[crate::KeyCode::KeyA,crate::KeyCode::KeyB,crate::KeyCode::KeyC,crate::KeyCode::KeyD,crate::KeyCode::KeyE,crate::KeyCode::KeyF,crate::KeyCode::KeyG,crate::KeyCode::KeyH];
        for (function,tag,expected) in rows {
            let mut profile=GameProfile::default();for (i,key) in keys.iter().enumerate(){profile.key_bindings.bindings[i]=Some(*key);}
            let button=match tag {0=>crate::MouseButton::Left,1=>crate::MouseButton::Right,2=>crate::MouseButton::Middle,3=>crate::MouseButton::Back,4=>crate::MouseButton::Forward,5=>crate::MouseButton::Other(if function==0 {0x2345}else{0x1234}),_=>unreachable!()};
            profile.key_bindings.mouse[0]=Some(button);profile.key_bindings.mouse[1]=Some(button);
            let tree=match function {0=>camera_pan_hint(&profile),1=>camera_rotate_hint(&profile),2=>camera_zoom_hint(),_=>unreachable!()};
            let mut actual=Vec::new();flatten(&tree,&keys,&mut actual);assert_eq!(actual,expected,"function={function},mouse={tag}");
        }
    }

    #[test]
    fn camera_display_reads_live_modifiers_without_a_structure_resource() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::systems::{backend::input::mod_key::Modifiers, mode_manager::AppMode};
        let mut world = World::new();
        world.insert_resource(GameProfile::default());
        world.insert_resource(CameraHintsState { state: 1, ..Default::default() });
        world.init_resource::<crate::hint_camera_progress::CameraHintProgress>();
        world.init_resource::<Events<DisplayHintRequest>>();
        world.init_resource::<Events<country_core::systems::ui::resources::widget_params::PlayUiSound>>();
        world.init_resource::<GameTime>();
        for (mode, flags, expected) in [
            (AppMode::Terrain, 0, 3),
            (AppMode::Terrain, 1, 0),
            (AppMode::Terrain, 0x100, 3),
            (AppMode::Photomode, 1, 3),
            (AppMode::default(), 1, 3),
        ] {
            world.insert_resource(mode);
            world.insert_resource(Modifiers { flags });
            world.run_system_once(hint_camera_display_condition).unwrap();
            assert_eq!(world.resource_mut::<Events<DisplayHintRequest>>().drain().count(), expected);
        }
    }

    #[test]
    fn camera_bindings_keep_real_profile_keys_mouse_and_native_tree_order() {
        let mut profile = GameProfile::default();
        let keys = &mut profile.key_bindings.bindings;
        keys[0] = Some(crate::KeyCode::ArrowUp);
        keys[1] = Some(crate::KeyCode::ArrowDown);
        keys[2] = Some(crate::KeyCode::ArrowLeft);
        keys[3] = None;
        keys[6] = Some(crate::KeyCode::Unidentified(
            bevy_input::keyboard::NativeKeyCode::Windows(0xff),
        ));
        keys[7] = Some(crate::KeyCode::KeyB);
        let keys = *keys;
        profile.key_bindings.mouse[0] = Some(crate::MouseButton::Other(9));
        profile.key_bindings.mouse[1] = Some(crate::MouseButton::Back);
        assert_eq!(
            camera_pan_hint(&profile),
            BindingHintDisplay::Alternate(vec![
                BindingHintDisplay::Steam(SteamQuery(country_core::resources::steam::packed_set(
                    0
                ))),
                BindingHintDisplay::Row([keys[0], keys[2], keys[1], keys[3]]),
                BindingHintDisplay::Mouse(crate::MouseButton::Back),
            ])
        );
        assert_eq!(
            camera_rotate_hint(&profile),
            BindingHintDisplay::Alternate(vec![
                BindingHintDisplay::Together(vec![
                    BindingHintDisplay::Steam(SteamQuery(
                        country_core::resources::steam::packed_set_plus_one(1)
                    )),
                    BindingHintDisplay::Steam(SteamQuery(
                        country_core::resources::steam::packed_set(1)
                    ))
                ]),
                BindingHintDisplay::Mouse(crate::MouseButton::Other(9)),
                BindingHintDisplay::Adjacent(vec![
                    BindingHintDisplay::Key(keys[6]),
                    BindingHintDisplay::Key(keys[7])
                ]),
            ])
        );
        profile.key_bindings.mouse[0] = None;
        let BindingHintDisplay::Alternate(parts) = camera_rotate_hint(&profile) else {
            panic!()
        };
        assert_eq!(parts[1], BindingHintDisplay::Empty);
    }

    #[test]
    fn camera_animations_are_one_three_frame_graphic_and_zoom_has_original_scroll_image() {
        for mode in [1, 2] {
            let mut profile = GameProfile::default();
            let button = if mode == 1 {
                crate::MouseButton::Right
            } else {
                crate::MouseButton::Middle
            };
            profile.key_bindings.mouse[0] = Some(button);
            profile.key_bindings.mouse[1] = Some(button);
            // 旧误命名字段不再拥有鼠标绑定；故意相反以防再次从这里猜测业务。
            profile.camera_pan_glyph_mode = 3 - mode;
            profile.camera_rotate_glyph_mode = 3 - mode;
            for (tree, graphic_index) in [
                (camera_pan_hint(&profile), 2),
                (camera_rotate_hint(&profile), 1),
            ] {
                let BindingHintDisplay::Alternate(parts) = tree else {
                    panic!()
                };
                assert_eq!(parts.len(), 3);
                let BindingHintDisplay::Graphic(graphic) = &parts[graphic_index] else {
                    panic!("one Graphic")
                };
                assert_eq!(graphic.height, 100.0);
                let HintGraphicImages::Animation {
                    frames,
                    seconds_per_frame,
                } = &graphic.images
                else {
                    panic!()
                };
                assert_eq!(*seconds_per_frame, 0.7);
                assert_eq!(frames[0], "hint/mouse");
                assert_eq!(frames[1], if mode == 1 { "hint/rmb" } else { "hint/mmb" });
                assert_eq!(graphic.image_at_time(0.7), frames[1]);
                assert_eq!(graphic.image_at_time(2.11), frames[0]);
            }
        }
        assert_eq!(
            camera_zoom_hint(),
            BindingHintDisplay::Alternate(vec![
                BindingHintDisplay::Adjacent(vec![
                    BindingHintDisplay::Steam(SteamQuery(
                        country_core::resources::steam::packed_set_plus_one(16)
                    )),
                    BindingHintDisplay::Steam(SteamQuery(
                        country_core::resources::steam::packed_set_plus_one(17)
                    ))
                ]),
                BindingHintDisplay::Graphic(crate::hint_orchestrator::HintGraphicParams::image(
                    "hint/scroll",
                    100.0
                )),
            ])
        );
    }
}
