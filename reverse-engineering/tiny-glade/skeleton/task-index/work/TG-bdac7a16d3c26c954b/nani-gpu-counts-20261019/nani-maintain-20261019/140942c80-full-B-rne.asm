140942c80: 55 push rbp
140942c81: 4156 push r14
140942c83: 56 push rsi
140942c84: 57 push rdi
140942c85: 53 push rbx
140942c86: 4881ec10010000 sub rsp, 0x110
140942c8d: 488dac2480000000 lea rbp, [rsp + 0x80]
140942c95: 48c78588000000feffffff mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
140942ca0: 4889d7 mov rdi, rdx
140942ca3: 4889ce mov rsi, rcx
140942ca6: 488d4d10 lea rcx, [rbp + 0x10]
140942caa: e8d18eab00 call 0x1413fbb80 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::cha
140942caf: c6858700000001 mov byte ptr [rbp + 0x87], 1
140942cb6: 488b0db396ce02 mov rcx, qword ptr [rip + 0x2ce96b3]
140942cbd: 4c8d059cd03102 lea r8, [rip + 0x231d09c]
140942cc4: 488d9587000000 lea rdx, [rbp + 0x87]
140942ccb: e8d0eb1d00 call 0x140b218a0 ; bevy_ecs::intern::Interner::intern<dyn$<bevy_ecs::schedule::set::SystemSet> >
140942cd0: 4989d0 mov r8, rdx
140942cd3: 488d4d10 lea rcx, [rbp + 0x10]
140942cd7: 4889c2 mov rdx, rax
140942cda: e82103b100 call 0x141453000 ; bevy_ecs::schedule::config::ScheduleConfigs::in_set_inner<bevy_ecs::intern::Interned<dyn$<bevy_ecs::schedule::set::SystemSet> > >
140942cdf: 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
140942ce3: 0f294500 movaps xmmword ptr [rbp], xmm0
140942ce7: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
140942ceb: 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140942cef: 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
140942cf3: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140942cf7: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
140942cfb: 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
140942cff: 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
140942d03: 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
140942d07: 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
140942d0b: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
140942d0f: 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
140942d13: 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
140942d17: 488d55a0 lea rdx, [rbp - 0x60]
140942d1b: 4889f1 mov rcx, rsi
140942d1e: e85d452200 call 0x140b67280 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::
140942d23: 4c8d0556fb1602 lea r8, [rip + 0x216fb56]
140942d2a: 488d5d10 lea rbx, [rbp + 0x10]
140942d2e: 4889d9 mov rcx, rbx
140942d31: b202 mov dl, 2
140942d33: e898aa1600 call 0x140aad7d0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<co
140942d38: 4889f1 mov rcx, rsi
140942d3b: 4889da mov rdx, rbx
140942d3e: e83d452200 call 0x140b67280 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::
140942d43: 4c8d054efb1602 lea r8, [rip + 0x216fb4e]
140942d4a: 4889d9 mov rcx, rbx
140942d4d: b20a mov dl, 0xa
140942d4f: e8dcca1700 call 0x140abf830 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_
140942d54: 4889f1 mov rcx, rsi
140942d57: 4889da mov rdx, rbx
140942d5a: e821452200 call 0x140b67280 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::
140942d5f: 4c8d054afb1602 lea r8, [rip + 0x216fb4a]
140942d66: 4889d9 mov rcx, rbx
140942d69: b203 mov dl, 3
140942d6b: e87093d400 call 0x14168c0e0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::reader::Event
140942d70: 4889f1 mov rcx, rsi
140942d73: 4889da mov rdx, rbx
140942d76: e805452200 call 0x140b67280 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::
140942d7b: 488d4d10 lea rcx, [rbp + 0x10]
140942d7f: e84c3d0400 call 0x140986ad0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::Cursor
140942d84: c6858700000005 mov byte ptr [rbp + 0x87], 5
140942d8b: 488b0dde95ce02 mov rcx, qword ptr [rip + 0x2ce95de]
140942d92: 4c8d05c7cf3102 lea r8, [rip + 0x231cfc7]
140942d99: 488d9587000000 lea rdx, [rbp + 0x87]
140942da0: e8fbea1d00 call 0x140b218a0 ; bevy_ecs::intern::Interner::intern<dyn$<bevy_ecs::schedule::set::SystemSet> >
140942da5: 4989d0 mov r8, rdx
140942da8: 488d4d10 lea rcx, [rbp + 0x10]
140942dac: 4889c2 mov rdx, rax
140942daf: e84c02b100 call 0x141453000 ; bevy_ecs::schedule::config::ScheduleConfigs::in_set_inner<bevy_ecs::intern::Interned<dyn$<bevy_ecs::schedule::set::SystemSet> > >
140942db4: 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
140942db8: 0f294500 movaps xmmword ptr [rbp], xmm0
140942dbc: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
140942dc0: 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140942dc4: 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
140942dc8: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140942dcc: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
140942dd0: 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
140942dd4: 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
140942dd8: 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
140942ddc: 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
140942de0: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
140942de4: 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
140942de8: 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
140942dec: 488d55a0 lea rdx, [rbp - 0x60]
140942df0: 4889f1 mov rcx, rsi
140942df3: e888442200 call 0x140b67280 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::
140942df8: 4889d9 mov rcx, rbx
140942dfb: e8a070c100 call 0x141559ea0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),alloc::boxed::Box<dyn$<bevy_ecs::
140942e00: 4c8d0dc1fa1602 lea r9, [rip + 0x216fac1]
140942e07: 4c8d75a0 lea r14, [rbp - 0x60]
140942e0b: 4c89f1 mov rcx, r14
140942e0e: 4889da mov rdx, rbx
140942e11: 4531c0 xor r8d, r8d
140942e14: e8d7fdb000 call 0x141452bf0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,
140942e19: 4889f1 mov rcx, rsi
140942e1c: 4c89f2 mov rdx, r14
140942e1f: e85c442200 call 0x140b67280 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::
140942e24: 488d4d10 lea rcx, [rbp + 0x10]
140942e28: e8e324cb00 call 0x1415f5310 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libra
140942e2d: c685870000002c mov byte ptr [rbp + 0x87], 0x2c
140942e34: 488b0d3595ce02 mov rcx, qword ptr [rip + 0x2ce9535]
140942e3b: 4c8d051ecf3102 lea r8, [rip + 0x231cf1e]
140942e42: 488d9587000000 lea rdx, [rbp + 0x87]
140942e49: e852ea1d00 call 0x140b218a0 ; bevy_ecs::intern::Interner::intern<dyn$<bevy_ecs::schedule::set::SystemSet> >
140942e4e: 4989d0 mov r8, rdx
140942e51: 488d4d10 lea rcx, [rbp + 0x10]
140942e55: 4889c2 mov rdx, rax
140942e58: e8a301b100 call 0x141453000 ; bevy_ecs::schedule::config::ScheduleConfigs::in_set_inner<bevy_ecs::intern::Interned<dyn$<bevy_ecs::schedule::set::SystemSet> > >
140942e5d: 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
140942e61: 0f294500 movaps xmmword ptr [rbp], xmm0
140942e65: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
140942e69: 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140942e6d: 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
140942e71: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140942e75: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
140942e79: 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
140942e7d: 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
140942e81: 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
140942e85: 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
140942e89: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
140942e8d: 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
140942e91: 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
140942e95: 488d55a0 lea rdx, [rbp - 0x60]
140942e99: 4889f1 mov rcx, rsi
140942e9c: e8df432200 call 0x140b67280 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::
140942ea1: 80bfe700000001 cmp byte ptr [rdi + 0xe7], 1
140942ea8: 7530 jne 0x140942eda
140942eaa: 488d7d10 lea rdi, [rbp + 0x10]
140942eae: 4889f9 mov rcx, rdi
140942eb1: e80a91ab00 call 0x1413fbfc0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<co
140942eb6: 4c8d0d23fa1602 lea r9, [rip + 0x216fa23]
140942ebd: 488d5da0 lea rbx, [rbp - 0x60]
140942ec1: 4889d9 mov rcx, rbx
140942ec4: 4889fa mov rdx, rdi
140942ec7: 4531c0 xor r8d, r8d
140942eca: e8f1a5d500 call 0x14169d4c0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detect
140942ecf: 4889f1 mov rcx, rsi
140942ed2: 4889da mov rdx, rbx
140942ed5: e8a6432200 call 0x140b67280 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::
140942eda: 4889f0 mov rax, rsi
140942edd: 4881c410010000 add rsp, 0x110
140942ee4: 5b pop rbx
140942ee5: 5f pop rdi
140942ee6: 5e pop rsi
140942ee7: 415e pop r14
140942ee9: 5d pop rbp
140942eea: c3 ret 
140942eeb: 0f1f440000 nop dword ptr [rax + rax]
140942ef0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140942ef5: 55 push rbp
140942ef6: 4156 push r14
140942ef8: 56 push rsi
140942ef9: 57 push rdi
140942efa: 53 push rbx
140942efb: 4883ec20 sub rsp, 0x20
140942eff: 488daa80000000 lea rbp, [rdx + 0x80]
140942f06: 488d4d10 lea rcx, [rbp + 0x10]
140942f0a: e871696eff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
140942f0f: 90 nop 
140942f10: 4883c420 add rsp, 0x20
140942f14: 5b pop rbx
140942f15: 5f pop rdi
140942f16: 5e pop rsi
140942f17: 415e pop r14
140942f19: 5d pop rbp
140942f1a: c3 ret 
140942f1b: 0f1f440000 nop dword ptr [rax + rax]
140942f20: 4889542410 mov qword ptr [rsp + 0x10], rdx
140942f25: 55 push rbp
140942f26: 4156 push r14
140942f28: 56 push rsi
140942f29: 57 push rdi
140942f2a: 53 push rbx
140942f2b: 4883ec20 sub rsp, 0x20
140942f2f: 488daa80000000 lea rbp, [rdx + 0x80]
140942f36: 488d4d10 lea rcx, [rbp + 0x10]
140942f3a: e841696eff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
140942f3f: 90 nop 
140942f40: 4883c420 add rsp, 0x20
140942f44: 5b pop rbx
140942f45: 5f pop rdi
140942f46: 5e pop rsi
140942f47: 415e pop r14
140942f49: 5d pop rbp
140942f4a: c3 ret 
140942f4b: 0f1f440000 nop dword ptr [rax + rax]
140942f50: 4889542410 mov qword ptr [rsp + 0x10], rdx
140942f55: 55 push rbp
140942f56: 4156 push r14
140942f58: 56 push rsi
140942f59: 57 push rdi
140942f5a: 53 push rbx
140942f5b: 4883ec20 sub rsp, 0x20
140942f5f: 488daa80000000 lea rbp, [rdx + 0x80]
140942f66: 488d4d10 lea rcx, [rbp + 0x10]
140942f6a: e811696eff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
140942f6f: 90 nop 
140942f70: 4883c420 add rsp, 0x20
140942f74: 5b pop rbx
140942f75: 5f pop rdi
140942f76: 5e pop rsi
140942f77: 415e pop r14
140942f79: 5d pop rbp
140942f7a: c3 ret 
