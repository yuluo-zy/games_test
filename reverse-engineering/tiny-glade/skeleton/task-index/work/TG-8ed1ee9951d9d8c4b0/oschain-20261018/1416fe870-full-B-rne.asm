00000001416fe870: 55                               push rbp
00000001416fe871: 56                               push rsi
00000001416fe872: 57                               push rdi
00000001416fe873: 53                               push rbx
00000001416fe874: 4881ec18020000                   sub rsp, 0x218
00000001416fe87b: 488dac2480000000                 lea rbp, [rsp + 0x80]
00000001416fe883: 48c78590010000feffffff           mov qword ptr [rbp + 0x190], 0xfffffffffffffffe
00000001416fe88e: 4889ce                           mov rsi, rcx
00000001416fe891: 0f57c0                           xorps xmm0, xmm0
00000001416fe894: 0f114570                         movups xmmword ptr [rbp + 0x70], xmm0
00000001416fe898: 0f118588000000                   movups xmmword ptr [rbp + 0x88], xmm0
00000001416fe89f: 0f1185a0000000                   movups xmmword ptr [rbp + 0xa0], xmm0
00000001416fe8a6: 0f1185b8000000                   movups xmmword ptr [rbp + 0xb8], xmm0
00000001416fe8ad: 0f1185d0000000                   movups xmmword ptr [rbp + 0xd0], xmm0
00000001416fe8b4: c785e000000000000000             mov dword ptr [rbp + 0xe0], 0
00000001416fe8be: 0f118508010000                   movups xmmword ptr [rbp + 0x108], xmm0
00000001416fe8c5: 0f118520010000                   movups xmmword ptr [rbp + 0x120], xmm0
00000001416fe8cc: 0f118538010000                   movups xmmword ptr [rbp + 0x138], xmm0
00000001416fe8d3: 0f118550010000                   movups xmmword ptr [rbp + 0x150], xmm0
00000001416fe8da: 0f118568010000                   movups xmmword ptr [rbp + 0x168], xmm0
00000001416fe8e1: c7857801000000000000             mov dword ptr [rbp + 0x178], 0
00000001416fe8eb: 48c7451800000000                 mov qword ptr [rbp + 0x18], 0
00000001416fe8f3: 48c7455000000000                 mov qword ptr [rbp + 0x50], 0
00000001416fe8fb: 48c7455808000000                 mov qword ptr [rbp + 0x58], 8
00000001416fe903: 48c7456000000000                 mov qword ptr [rbp + 0x60], 0
00000001416fe90b: 48c7456810000000                 mov qword ptr [rbp + 0x68], 0x10
00000001416fe913: 48c7858000000010000000           mov qword ptr [rbp + 0x80], 0x10
00000001416fe91e: 48c7859800000010000000           mov qword ptr [rbp + 0x98], 0x10
00000001416fe929: 48c785b000000010000000           mov qword ptr [rbp + 0xb0], 0x10
00000001416fe934: 48c785c800000010000000           mov qword ptr [rbp + 0xc8], 0x10
00000001416fe93f: 48b80000000000000080             movabs rax, 0x8000000000000000
00000001416fe949: 488985e8000000                   mov qword ptr [rbp + 0xe8], rax
00000001416fe950: 488d0526455901                   lea rax, [rip + 0x1594526]
00000001416fe957: 488985f0000000                   mov qword ptr [rbp + 0xf0], rax
00000001416fe95e: 48c785f800000033000000           mov qword ptr [rbp + 0xf8], 0x33
00000001416fe969: 48c7850001000010000000           mov qword ptr [rbp + 0x100], 0x10
00000001416fe974: 48c7851801000010000000           mov qword ptr [rbp + 0x118], 0x10
00000001416fe97f: 48c7853001000010000000           mov qword ptr [rbp + 0x130], 0x10
00000001416fe98a: 48c7854801000010000000           mov qword ptr [rbp + 0x148], 0x10
00000001416fe995: 48c7856001000010000000           mov qword ptr [rbp + 0x160], 0x10
00000001416fe9a0: c7858001000000000000             mov dword ptr [rbp + 0x180], 0
00000001416fe9aa: 66c785840100000100               mov word ptr [rbp + 0x184], 1
00000001416fe9b3: c7858801000000000000             mov dword ptr [rbp + 0x188], 0
00000001416fe9bd: 0fb6054562f301                   movzx eax, byte ptr [rip + 0x1f36245]
00000001416fe9c4: b978010000                       mov ecx, 0x178
00000001416fe9c9: ba08000000                       mov edx, 8
00000001416fe9ce: e83d52f1fe                       call 0x140613c10 ; 
00000001416fe9d3: 4885c0                           test rax, rax
00000001416fe9d6: 7445                             je 0x1416fea1d
00000001416fe9d8: 4889c7                           mov rdi, rax
00000001416fe9db: 488d5518                         lea rdx, [rbp + 0x18]
00000001416fe9df: 41b878010000                     mov r8d, 0x178
00000001416fe9e5: 4889c1                           mov rcx, rax
00000001416fe9e8: e813522201                       call 0x142923c00 ; 
00000001416fe9ed: 4c8d0544425901                   lea r8, [rip + 0x1594244]
00000001416fe9f4: 488d5da8                         lea rbx, [rbp - 0x58]
00000001416fe9f8: 4889d9                           mov rcx, rbx
00000001416fe9fb: 4889fa                           mov rdx, rdi
00000001416fe9fe: e80d615eff                       call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
00000001416fea03: 4889f1                           mov rcx, rsi
00000001416fea06: 4889da                           mov rdx, rbx
00000001416fea09: e8520cd5ff                       call 0x14144f660 ; bevy_ecs::schedule::config::impl$3::run_if<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)>,bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)>
00000001416fea0e: 4889f0                           mov rax, rsi
00000001416fea11: 4881c418020000                   add rsp, 0x218
00000001416fea18: 5b                               pop rbx
00000001416fea19: 5f                               pop rdi
00000001416fea1a: 5e                               pop rsi
00000001416fea1b: 5d                               pop rbp
00000001416fea1c: c3                               ret 
00000001416fea1d: b908000000                       mov ecx, 8
00000001416fea22: ba78010000                       mov edx, 0x178
00000001416fea27: e8d4a51d01                       call 0x1428d9000 ; alloc::alloc::handle_alloc_error
00000001416fea2c: 0f0b                             ud2 
00000001416fea2e: 6690                             nop 
00000001416fea30: 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001416fea35: 55                               push rbp
00000001416fea36: 56                               push rsi
00000001416fea37: 57                               push rdi
00000001416fea38: 53                               push rbx
00000001416fea39: 4883ec28                         sub rsp, 0x28
00000001416fea3d: 488daa80000000                   lea rbp, [rdx + 0x80]
00000001416fea44: 488d4d18                         lea rcx, [rbp + 0x18]
00000001416fea48: e8a3b0eefe                       call 0x1405e9af0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardFocusLost>),void (*)(bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardFocusLost>)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::NotifyHintSystemWindow>,bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::event::writer::EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::NotifyHintSystemWindow>,bevy_ecs::event::writer::EventWriter<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::event::writer::EventWriter<system_onboarding::tutorial_completion::TutorialHintComplete>)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::reader::EventReader<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::reader::EventReader<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::timberframe_audio::PlayTimberframeSound>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::timberframe_audio::PlayTimberframeSound>)> > >
00000001416fea4d: 90                               nop 
00000001416fea4e: 4883c428                         add rsp, 0x28
00000001416fea52: 5b                               pop rbx
00000001416fea53: 5f                               pop rdi
00000001416fea54: 5e                               pop rsi
00000001416fea55: 5d                               pop rbp
00000001416fea56: c3                               ret 
