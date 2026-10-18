1412d9790: push rsi
1412d9791: push rdi
1412d9792: sub rsp, 0xe8
1412d9799: add qword ptr [r8], 0x70
1412d979d: test rdx, rdx
1412d97a0: je 0x1412d987b
1412d97a6: mov rsi, rdx
1412d97a9: mov rdi, qword ptr [rcx + 0x60]
1412d97ad: mov rdx, qword ptr [rcx + 0x50]
1412d97b1: movups xmm0, xmmword ptr [rcx + 0x40]
1412d97b5: movaps xmmword ptr [rsp + 0xd0], xmm0
1412d97bd: movups xmm0, xmmword ptr [rcx]
1412d97c0: movups xmm1, xmmword ptr [rcx + 0x10]
1412d97c4: movups xmm2, xmmword ptr [rcx + 0x20]
1412d97c8: movups xmm3, xmmword ptr [rcx + 0x30]
1412d97cc: movaps xmmword ptr [rsp + 0xc0], xmm3
1412d97d4: movaps xmmword ptr [rsp + 0xb0], xmm2
1412d97dc: movaps xmmword ptr [rsp + 0xa0], xmm1
1412d97e4: movaps xmmword ptr [rsp + 0x90], xmm0
1412d97ec: lea rcx, [rsp + 0x40]
1412d97f1: mov r8, rsi
1412d97f4: call 0x140ceb900 ; bevy_ecs::world::entity_fetch::impl$1::fetch_mut
1412d97f9: mov rdx, qword ptr [rsp + 0x40]
1412d97fe: mov rax, qword ptr [rsp + 0x48]
1412d9803: mov rcx, qword ptr [rsp + 0x50]
1412d9808: test rdx, rdx
1412d980b: je 0x1412d9885
1412d980d: mov r8, qword ptr [rsp + 0x58]
1412d9812: mov qword ptr [rsp + 0x20], rdx
1412d9817: mov qword ptr [rsp + 0x28], rax
1412d981c: mov qword ptr [rsp + 0x30], rcx
1412d9821: mov qword ptr [rsp + 0x38], r8
1412d9826: movaps xmm0, xmmword ptr [rsp + 0xd0]
1412d982e: movaps xmmword ptr [rsp + 0x80], xmm0
1412d9836: movaps xmm0, xmmword ptr [rsp + 0x90]
1412d983e: movaps xmm1, xmmword ptr [rsp + 0xa0]
1412d9846: movaps xmm2, xmmword ptr [rsp + 0xb0]
1412d984e: movaps xmm3, xmmword ptr [rsp + 0xc0]
1412d9856: movaps xmmword ptr [rsp + 0x70], xmm3
1412d985b: movaps xmmword ptr [rsp + 0x60], xmm2
1412d9860: movaps xmmword ptr [rsp + 0x50], xmm1
1412d9865: movaps xmmword ptr [rsp + 0x40], xmm0
1412d986a: lea rcx, [rsp + 0x40]
1412d986f: lea rdx, [rsp + 0x20]
1412d9874: call 0x142036030 ; bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::FlatLighting> > >
1412d9879: jmp 0x1412d988b
1412d987b: add rsp, 0xe8
1412d9882: pop rdi
1412d9883: pop rsi
1412d9884: ret 
1412d9885: cmp rax, 2
1412d9889: jne 0x1412d98ac
1412d988b: mov rcx, rsi
1412d988e: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
1412d9893: mov rcx, rsi
1412d9896: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
1412d989b: mov rcx, rsi
1412d989e: add rsp, 0xe8
1412d98a5: pop rdi
1412d98a6: pop rsi
1412d98a7: jmp 0x140cf8200
1412d98ac: mov qword ptr [rsp + 0x20], rax
1412d98b1: mov qword ptr [rsp + 0x28], rcx
1412d98b6: lea rcx, [rsp + 0x20]
1412d98bb: call 0x1428a81c0 ; bevy_ecs::error::bevy_error::impl$1::from<enum2$<bevy_ecs::world::error::EntityMutableFetchError> >
1412d98c0: movabs rcx, 0x8000000000000000
1412d98ca: mov qword ptr [rsp + 0x48], rcx
1412d98cf: lea rcx, [rip + 0x19979ba]
1412d98d6: mov qword ptr [rsp + 0x50], rcx
1412d98db: mov qword ptr [rsp + 0x58], 0x16d
1412d98e4: mov dword ptr [rsp + 0x40], 2
1412d98ec: lea rdx, [rsp + 0x40]
1412d98f1: mov rcx, rax
1412d98f4: call rdi
1412d98f6: jmp 0x1412d988b
