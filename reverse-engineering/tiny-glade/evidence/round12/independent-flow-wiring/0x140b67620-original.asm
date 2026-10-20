140b67620: push rbp
140b67621: push rsi
140b67622: sub rsp, 0x138
140b67629: lea rbp, [rsp + 0x80]
140b67631: mov qword ptr [rbp + 0xb0], 0xfffffffffffffffe
140b6763c: mov qword ptr [rbp + 0xa0], rdx
140b67643: mov rsi, rcx
140b67646: add rcx, 0x38
140b6764a: mov byte ptr [rbp + 0xaf], 1
140b67651: movabs rdx, 0x8e1bd251962ba784
140b6765b: movabs r8, 0x6a782374c0e6529b
140b67665: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140b6766a: test rax, rax
140b6766d: je 0x140b67790
140b67673: cmp qword ptr [rsi + 0x218], rdx
140b6767a: jbe 0x140b67790
140b67680: mov rax, qword ptr [rsi + 0x210]
140b67687: mov rcx, qword ptr [rax + rdx*8]
140b6768b: test rcx, rcx
140b6768e: je 0x140b67790
140b67694: mov rax, qword ptr [rsi + 0x1e0]
140b6769b: not rcx
140b6769e: lea rcx, [rcx + rcx*2]
140b676a2: shl rcx, 5
140b676a6: cmp qword ptr [rax + rcx + 0x20], 0
140b676ac: je 0x140b67790
140b676b2: add rax, rcx
140b676b5: mov rcx, qword ptr [rax + 0x10]
140b676b9: mov edx, dword ptr [rsi + 0x5b0]
140b676bf: mov dword ptr [rax + 0x5c], edx
140b676c2: mov rax, qword ptr [rbp + 0xa0]
140b676c9: movups xmm0, xmmword ptr [rax + 0x60]
140b676cd: movaps xmmword ptr [rbp + 0x70], xmm0
140b676d1: movups xmm0, xmmword ptr [rax + 0x50]
140b676d5: movaps xmmword ptr [rbp + 0x60], xmm0
140b676d9: movups xmm0, xmmword ptr [rax + 0x40]
140b676dd: movaps xmmword ptr [rbp + 0x50], xmm0
140b676e1: movups xmm0, xmmword ptr [rax]
140b676e4: movups xmm1, xmmword ptr [rax + 0x10]
140b676e8: movups xmm2, xmmword ptr [rax + 0x20]
140b676ec: movups xmm3, xmmword ptr [rax + 0x30]
140b676f0: movaps xmmword ptr [rbp + 0x40], xmm3
140b676f4: movaps xmmword ptr [rbp + 0x30], xmm2
140b676f8: movaps xmmword ptr [rbp + 0x20], xmm1
140b676fc: movaps xmmword ptr [rbp + 0x10], xmm0
140b67700: mov byte ptr [rbp + 0xae], 1
140b67707: call 0x1416a13f0 ; bevy_ecs::schedule::schedule::Schedules::entry<bevy_app::main_schedule::PreStartup>
140b6770c: movaps xmm0, xmmword ptr [rbp + 0x70]
140b67710: movaps xmmword ptr [rbp], xmm0
140b67714: movaps xmm0, xmmword ptr [rbp + 0x60]
140b67718: movaps xmmword ptr [rbp - 0x10], xmm0
140b6771c: movaps xmm0, xmmword ptr [rbp + 0x50]
140b67720: movaps xmmword ptr [rbp - 0x20], xmm0
140b67724: movaps xmm0, xmmword ptr [rbp + 0x10]
140b67728: movaps xmm1, xmmword ptr [rbp + 0x20]
140b6772c: movaps xmm2, xmmword ptr [rbp + 0x30]
140b67730: movaps xmm3, xmmword ptr [rbp + 0x40]
140b67734: movaps xmmword ptr [rbp - 0x30], xmm3
140b67738: movaps xmmword ptr [rbp - 0x40], xmm2
140b6773c: movaps xmmword ptr [rbp - 0x50], xmm1
140b67740: movaps xmmword ptr [rbp - 0x60], xmm0
140b67744: mov byte ptr [rbp + 0xae], 0
140b6774b: lea rcx, [rbp + 0x80]
140b67752: lea r8, [rbp - 0x60]
140b67756: mov rdx, rax
140b67759: xor r9d, r9d
140b6775c: call 0x14169ed30 ; bevy_ecs::schedule::schedule::ScheduleGraph::process_configs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >
140b67761: mov rdx, qword ptr [rbp + 0x80]
140b67768: test rdx, rdx
140b6776b: je 0x140b67783
140b6776d: mov rcx, qword ptr [rbp + 0x88]
140b67774: shl rdx, 4
140b67778: mov r8d, 8
140b6777e: call 0x140613c20 ; 
140b67783: mov rax, rsi
140b67786: add rsp, 0x138
140b6778d: pop rsi
140b6778e: pop rbp
140b6778f: ret 
140b67790: lea rax, [rip + 0x1f8b529]
140b67797: mov qword ptr [rbp + 0x10], rax
140b6779b: mov qword ptr [rbp + 0x18], 0x27
140b677a3: lea rax, [rbp + 0x10]
140b677a7: mov qword ptr [rbp + 0x80], rax
140b677ae: lea rax, [rip - 0xa5d925]
140b677b5: mov qword ptr [rbp + 0x88], rax
140b677bc: lea rax, [rip + 0x1f8b62d]
140b677c3: mov qword ptr [rbp - 0x60], rax
140b677c7: mov qword ptr [rbp - 0x58], 2
140b677cf: mov qword ptr [rbp - 0x40], 0
140b677d7: lea rax, [rbp + 0x80]
140b677de: mov qword ptr [rbp - 0x50], rax
140b677e2: mov qword ptr [rbp - 0x48], 1
140b677ea: mov byte ptr [rbp + 0xaf], 1
140b677f1: lea rdx, [rip + 0x1f8b4b0]
140b677f8: lea rcx, [rbp - 0x60]
140b677fc: call 0x1428d9390 ; core::panicking::panic_fmt
140b67801: ud2 
140b67803: nop word ptr cs:[rax + rax]
140b67810: mov qword ptr [rsp + 0x10], rdx
140b67815: push rbp
140b67816: push rsi
140b67817: sub rsp, 0x28
140b6781b: lea rbp, [rdx + 0x80]
140b67822: cmp byte ptr [rbp + 0xae], 0
140b67829: je 0x140b6783b
140b6782b: mov byte ptr [rbp + 0xaf], 0
140b67832: lea rcx, [rbp + 0x10]
140b67836: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
140b6783b: mov byte ptr [rbp + 0xaf], 0
140b67842: add rsp, 0x28
140b67846: pop rsi
140b67847: pop rbp
140b67848: ret 
140b67849: nop dword ptr [rax]
140b67850: mov qword ptr [rsp + 0x10], rdx
140b67855: push rbp
140b67856: push rsi
140b67857: sub rsp, 0x28
140b6785b: lea rbp, [rdx + 0x80]
140b67862: cmp byte ptr [rbp + 0xaf], 0
140b67869: jne 0x140b67872
140b6786b: add rsp, 0x28
140b6786f: pop rsi
140b67870: pop rbp
140b67871: ret 
140b67872: mov rcx, qword ptr [rbp + 0xa0]
140b67879: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
140b6787e: jmp 0x140b6786b
