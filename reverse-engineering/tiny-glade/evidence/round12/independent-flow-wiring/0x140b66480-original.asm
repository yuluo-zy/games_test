140b66480: push rbp
140b66481: push rsi
140b66482: push rbx
140b66483: sub rsp, 0x140
140b6648a: lea rbp, [rsp + 0x80]
140b66492: mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
140b6649d: mov qword ptr [rbp + 0xa8], r8
140b664a4: mov bl, dl
140b664a6: mov rsi, rcx
140b664a9: add rcx, 0x38
140b664ad: mov byte ptr [rbp + 0xb7], 1
140b664b4: movabs rdx, 0x8e1bd251962ba784
140b664be: movabs r8, 0x6a782374c0e6529b
140b664c8: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140b664cd: test rax, rax
140b664d0: je 0x140b665f6
140b664d6: cmp qword ptr [rsi + 0x218], rdx
140b664dd: jbe 0x140b665f6
140b664e3: mov rax, qword ptr [rsi + 0x210]
140b664ea: mov rcx, qword ptr [rax + rdx*8]
140b664ee: test rcx, rcx
140b664f1: je 0x140b665f6
140b664f7: mov rax, qword ptr [rsi + 0x1e0]
140b664fe: not rcx
140b66501: lea rcx, [rcx + rcx*2]
140b66505: shl rcx, 5
140b66509: cmp qword ptr [rax + rcx + 0x20], 0
140b6650f: je 0x140b665f6
140b66515: add rax, rcx
140b66518: mov rcx, qword ptr [rax + 0x10]
140b6651c: mov edx, dword ptr [rsi + 0x5b0]
140b66522: mov dword ptr [rax + 0x5c], edx
140b66525: mov rax, qword ptr [rbp + 0xa8]
140b6652c: movups xmm0, xmmword ptr [rax + 0x60]
140b66530: movaps xmmword ptr [rbp + 0x70], xmm0
140b66534: movups xmm0, xmmword ptr [rax + 0x50]
140b66538: movaps xmmword ptr [rbp + 0x60], xmm0
140b6653c: movups xmm0, xmmword ptr [rax + 0x40]
140b66540: movaps xmmword ptr [rbp + 0x50], xmm0
140b66544: movups xmm0, xmmword ptr [rax]
140b66547: movups xmm1, xmmword ptr [rax + 0x10]
140b6654b: movups xmm2, xmmword ptr [rax + 0x20]
140b6654f: movups xmm3, xmmword ptr [rax + 0x30]
140b66553: movaps xmmword ptr [rbp + 0x40], xmm3
140b66557: movaps xmmword ptr [rbp + 0x30], xmm2
140b6655b: movaps xmmword ptr [rbp + 0x20], xmm1
140b6655f: movaps xmmword ptr [rbp + 0x10], xmm0
140b66563: mov byte ptr [rbp + 0xb6], 1
140b6656a: mov edx, ebx
140b6656c: call 0x1416a1800 ; bevy_ecs::schedule::schedule::Schedules::entry<bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState> >
140b66571: movaps xmm0, xmmword ptr [rbp + 0x70]
140b66575: movaps xmmword ptr [rbp], xmm0
140b66579: movaps xmm0, xmmword ptr [rbp + 0x60]
140b6657d: movaps xmmword ptr [rbp - 0x10], xmm0
140b66581: movaps xmm0, xmmword ptr [rbp + 0x50]
140b66585: movaps xmmword ptr [rbp - 0x20], xmm0
140b66589: movaps xmm0, xmmword ptr [rbp + 0x10]
140b6658d: movaps xmm1, xmmword ptr [rbp + 0x20]
140b66591: movaps xmm2, xmmword ptr [rbp + 0x30]
140b66595: movaps xmm3, xmmword ptr [rbp + 0x40]
140b66599: movaps xmmword ptr [rbp - 0x30], xmm3
140b6659d: movaps xmmword ptr [rbp - 0x40], xmm2
140b665a1: movaps xmmword ptr [rbp - 0x50], xmm1
140b665a5: movaps xmmword ptr [rbp - 0x60], xmm0
140b665a9: mov byte ptr [rbp + 0xb6], 0
140b665b0: lea rcx, [rbp + 0x88]
140b665b7: lea r8, [rbp - 0x60]
140b665bb: mov rdx, rax
140b665be: xor r9d, r9d
140b665c1: call 0x14169ed30 ; bevy_ecs::schedule::schedule::ScheduleGraph::process_configs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >
140b665c6: mov rdx, qword ptr [rbp + 0x88]
140b665cd: test rdx, rdx
140b665d0: je 0x140b665e8
140b665d2: mov rcx, qword ptr [rbp + 0x90]
140b665d9: shl rdx, 4
140b665dd: mov r8d, 8
140b665e3: call 0x140613c20 ; 
140b665e8: mov rax, rsi
140b665eb: add rsp, 0x140
140b665f2: pop rbx
140b665f3: pop rsi
140b665f4: pop rbp
140b665f5: ret 
140b665f6: lea rax, [rip + 0x1f8c6c3]
140b665fd: mov qword ptr [rbp + 0x10], rax
140b66601: mov qword ptr [rbp + 0x18], 0x27
140b66609: lea rax, [rbp + 0x10]
140b6660d: mov qword ptr [rbp + 0x88], rax
140b66614: lea rax, [rip - 0xa5c78b]
140b6661b: mov qword ptr [rbp + 0x90], rax
140b66622: lea rax, [rip + 0x1f8c7c7]
140b66629: mov qword ptr [rbp - 0x60], rax
140b6662d: mov qword ptr [rbp - 0x58], 2
140b66635: mov qword ptr [rbp - 0x40], 0
140b6663d: lea rax, [rbp + 0x88]
140b66644: mov qword ptr [rbp - 0x50], rax
140b66648: mov qword ptr [rbp - 0x48], 1
140b66650: mov byte ptr [rbp + 0xb7], 1
140b66657: lea rdx, [rip + 0x1f8c64a]
140b6665e: lea rcx, [rbp - 0x60]
140b66662: call 0x1428d9390 ; core::panicking::panic_fmt
140b66667: ud2 
140b66669: nop dword ptr [rax]
140b66670: mov qword ptr [rsp + 0x10], rdx
140b66675: push rbp
140b66676: push rsi
140b66677: push rbx
140b66678: sub rsp, 0x20
140b6667c: lea rbp, [rdx + 0x80]
140b66683: cmp byte ptr [rbp + 0xb6], 0
140b6668a: je 0x140b6669c
140b6668c: mov byte ptr [rbp + 0xb7], 0
140b66693: lea rcx, [rbp + 0x10]
140b66697: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
140b6669c: mov byte ptr [rbp + 0xb7], 0
140b666a3: add rsp, 0x20
140b666a7: pop rbx
140b666a8: pop rsi
140b666a9: pop rbp
140b666aa: ret 
140b666ab: nop dword ptr [rax + rax]
140b666b0: mov qword ptr [rsp + 0x10], rdx
140b666b5: push rbp
140b666b6: push rsi
140b666b7: push rbx
140b666b8: sub rsp, 0x20
140b666bc: lea rbp, [rdx + 0x80]
140b666c3: cmp byte ptr [rbp + 0xb7], 0
140b666ca: jne 0x140b666d4
140b666cc: add rsp, 0x20
140b666d0: pop rbx
140b666d1: pop rsi
140b666d2: pop rbp
140b666d3: ret 
140b666d4: mov rcx, qword ptr [rbp + 0xa8]
140b666db: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
140b666e0: jmp 0x140b666cc
