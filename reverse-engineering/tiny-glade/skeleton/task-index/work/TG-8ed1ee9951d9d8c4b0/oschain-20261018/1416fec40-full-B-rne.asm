00000001416fec40: 55                               push rbp
00000001416fec41: 56                               push rsi
00000001416fec42: 57                               push rdi
00000001416fec43: 4881ec20020000                   sub rsp, 0x220
00000001416fec4a: 488dac2480000000                 lea rbp, [rsp + 0x80]
00000001416fec52: 48c78598010000feffffff           mov qword ptr [rbp + 0x198], 0xfffffffffffffffe
00000001416fec5d: 48899588010000                   mov qword ptr [rbp + 0x188], rdx
00000001416fec64: 4889ce                           mov rsi, rcx
00000001416fec67: 0f57c0                           xorps xmm0, xmm0
00000001416fec6a: 0f114550                         movups xmmword ptr [rbp + 0x50], xmm0
00000001416fec6e: 0f114568                         movups xmmword ptr [rbp + 0x68], xmm0
00000001416fec72: 0f118580000000                   movups xmmword ptr [rbp + 0x80], xmm0
00000001416fec79: 0f118598000000                   movups xmmword ptr [rbp + 0x98], xmm0
00000001416fec80: 0f1185b0000000                   movups xmmword ptr [rbp + 0xb0], xmm0
00000001416fec87: c785c000000000000000             mov dword ptr [rbp + 0xc0], 0
00000001416fec91: 0f1185e8000000                   movups xmmword ptr [rbp + 0xe8], xmm0
00000001416fec98: 0f118500010000                   movups xmmword ptr [rbp + 0x100], xmm0
00000001416fec9f: 0f118518010000                   movups xmmword ptr [rbp + 0x118], xmm0
00000001416feca6: 0f118530010000                   movups xmmword ptr [rbp + 0x130], xmm0
00000001416fecad: 0f118548010000                   movups xmmword ptr [rbp + 0x148], xmm0
00000001416fecb4: c7855801000000000000             mov dword ptr [rbp + 0x158], 0
00000001416fecbe: 48c7451000000000                 mov qword ptr [rbp + 0x10], 0
00000001416fecc6: 48c7453000000000                 mov qword ptr [rbp + 0x30], 0
00000001416fecce: 48c7453808000000                 mov qword ptr [rbp + 0x38], 8
00000001416fecd6: 48c7454000000000                 mov qword ptr [rbp + 0x40], 0
00000001416fecde: 48c7454810000000                 mov qword ptr [rbp + 0x48], 0x10
00000001416fece6: 48c7456010000000                 mov qword ptr [rbp + 0x60], 0x10
00000001416fecee: 48c7457810000000                 mov qword ptr [rbp + 0x78], 0x10
00000001416fecf6: 48c7859000000010000000           mov qword ptr [rbp + 0x90], 0x10
00000001416fed01: 48c785a800000010000000           mov qword ptr [rbp + 0xa8], 0x10
00000001416fed0c: 48b80000000000000080             movabs rax, 0x8000000000000000
00000001416fed16: 488985c8000000                   mov qword ptr [rbp + 0xc8], rax
00000001416fed1d: 488d058c415901                   lea rax, [rip + 0x159418c]
00000001416fed24: 488985d0000000                   mov qword ptr [rbp + 0xd0], rax
00000001416fed2b: 48c785d80000003d000000           mov qword ptr [rbp + 0xd8], 0x3d
00000001416fed36: 48c785e000000010000000           mov qword ptr [rbp + 0xe0], 0x10
00000001416fed41: 48c785f800000010000000           mov qword ptr [rbp + 0xf8], 0x10
00000001416fed4c: 48c7851001000010000000           mov qword ptr [rbp + 0x110], 0x10
00000001416fed57: 48c7852801000010000000           mov qword ptr [rbp + 0x128], 0x10
00000001416fed62: 48c7854001000010000000           mov qword ptr [rbp + 0x140], 0x10
00000001416fed6d: c7856001000000000000             mov dword ptr [rbp + 0x160], 0
00000001416fed77: 66c785640100000100               mov word ptr [rbp + 0x164], 1
00000001416fed80: c7856801000000000000             mov dword ptr [rbp + 0x168], 0
00000001416fed8a: 0fb605785ef301                   movzx eax, byte ptr [rip + 0x1f35e78]
00000001416fed91: b960010000                       mov ecx, 0x160
00000001416fed96: ba08000000                       mov edx, 8
00000001416fed9b: e8704ef1fe                       call 0x140613c10 ; 
00000001416feda0: 4885c0                           test rax, rax
00000001416feda3: 746c                             je 0x1416fee11
00000001416feda5: 4889c7                           mov rdi, rax
00000001416feda8: 488d5510                         lea rdx, [rbp + 0x10]
00000001416fedac: 41b860010000                     mov r8d, 0x160
00000001416fedb2: 4889c1                           mov rcx, rax
00000001416fedb5: e8464e2201                       call 0x142923c00 ; 
00000001416fedba: c6859701000001                   mov byte ptr [rbp + 0x197], 1
00000001416fedc1: 4c8d05e03f5901                   lea r8, [rip + 0x1593fe0]
00000001416fedc8: 488d4da0                         lea rcx, [rbp - 0x60]
00000001416fedcc: 4889fa                           mov rdx, rdi
00000001416fedcf: e83c5d5eff                       call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
00000001416fedd4: 488d7d10                         lea rdi, [rbp + 0x10]
00000001416fedd8: 41b878010000                     mov r8d, 0x178
00000001416fedde: 4889f9                           mov rcx, rdi
00000001416fede1: 488b9588010000                   mov rdx, qword ptr [rbp + 0x188]
00000001416fede8: e8134e2201                       call 0x142923c00 ; 
00000001416feded: c6859701000000                   mov byte ptr [rbp + 0x197], 0
00000001416fedf4: 488d55a0                         lea rdx, [rbp - 0x60]
00000001416fedf8: 4889f1                           mov rcx, rsi
00000001416fedfb: 4989f8                           mov r8, rdi
00000001416fedfe: e85dddd4ff                       call 0x14144cb60 ; bevy_ecs::schedule::config::impl$3::run_if<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<>,bevy_ecs::system::adapter_system::AdapterSystem<bevy_ecs::schedule::condition::NotMarker,bevy_ecs::system::function_system::FunctionSystem<bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >),bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)> > >
00000001416fee03: 4889f0                           mov rax, rsi
00000001416fee06: 4881c420020000                   add rsp, 0x220
00000001416fee0d: 5f                               pop rdi
00000001416fee0e: 5e                               pop rsi
00000001416fee0f: 5d                               pop rbp
00000001416fee10: c3                               ret 
00000001416fee11: b908000000                       mov ecx, 8
00000001416fee16: ba60010000                       mov edx, 0x160
00000001416fee1b: e8e0a11d01                       call 0x1428d9000 ; alloc::alloc::handle_alloc_error
00000001416fee20: 0f0b                             ud2 
00000001416fee22: 66666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
00000001416fee30: 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001416fee35: 55                               push rbp
00000001416fee36: 56                               push rsi
00000001416fee37: 57                               push rdi
00000001416fee38: 4883ec20                         sub rsp, 0x20
00000001416fee3c: 488daa80000000                   lea rbp, [rdx + 0x80]
00000001416fee43: c6859701000001                   mov byte ptr [rbp + 0x197], 1
00000001416fee4a: 488d4d10                         lea rcx, [rbp + 0x10]
00000001416fee4e: e81d70eefe                       call 0x1405e5e70 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>),void (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::InterShapeBrickStitches>,bevy_ecs::change_detection::Res<system_clutter::inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::InterShapeBrickStitches>,bevy_ecs::change_detection::Res<system_clutter::inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners>)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> > > | core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>,bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>),void (*)(bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>,bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>)> > >
00000001416fee53: c6859701000001                   mov byte ptr [rbp + 0x197], 1
00000001416fee5a: 4883c420                         add rsp, 0x20
00000001416fee5e: 5f                               pop rdi
00000001416fee5f: 5e                               pop rsi
00000001416fee60: 5d                               pop rbp
00000001416fee61: c3                               ret 
00000001416fee62: 66666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
00000001416fee70: 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001416fee75: 55                               push rbp
00000001416fee76: 56                               push rsi
00000001416fee77: 57                               push rdi
00000001416fee78: 4883ec20                         sub rsp, 0x20
00000001416fee7c: 488daa80000000                   lea rbp, [rdx + 0x80]
00000001416fee83: 80bd9701000000                   cmp byte ptr [rbp + 0x197], 0
00000001416fee8a: 7508                             jne 0x1416fee94
00000001416fee8c: 4883c420                         add rsp, 0x20
00000001416fee90: 5f                               pop rdi
00000001416fee91: 5e                               pop rsi
00000001416fee92: 5d                               pop rbp
00000001416fee93: c3                               ret 
00000001416fee94: 488b8d88010000                   mov rcx, qword ptr [rbp + 0x188]
00000001416fee9b: e8108e18ff                       call 0x140887cb0 ; core::ptr::drop_in_place<bevy_ecs::system::adapter_system::AdapterSystem<bevy_ecs::schedule::condition::NotMarker,bevy_ecs::system::function_system::FunctionSystem<bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >),bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)> > >
00000001416feea0: ebea                             jmp 0x1416fee8c
