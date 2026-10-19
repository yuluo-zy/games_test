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
