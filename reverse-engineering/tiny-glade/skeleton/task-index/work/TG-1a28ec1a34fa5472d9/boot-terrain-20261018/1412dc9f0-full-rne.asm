1412dc9f0: push rsi
1412dc9f1: push rdi
1412dc9f2: sub rsp, 0x108
1412dc9f9: sub qword ptr [r8], -0x80
1412dc9fd: test rdx, rdx
1412dca00: je 0x1412dcafb
1412dca06: mov rsi, rdx
1412dca09: mov rdi, qword ptr [rcx + 0x70]
1412dca0d: mov rdx, qword ptr [rcx + 0x60]
1412dca11: movups xmm0, xmmword ptr [rcx + 0x50]
1412dca15: movaps xmmword ptr [rsp + 0xf0], xmm0
1412dca1d: movups xmm0, xmmword ptr [rcx + 0x40]
1412dca21: movaps xmmword ptr [rsp + 0xe0], xmm0
1412dca29: movups xmm0, xmmword ptr [rcx]
1412dca2c: movups xmm1, xmmword ptr [rcx + 0x10]
1412dca30: movups xmm2, xmmword ptr [rcx + 0x20]
1412dca34: movups xmm3, xmmword ptr [rcx + 0x30]
1412dca38: movaps xmmword ptr [rsp + 0xd0], xmm3
1412dca40: movaps xmmword ptr [rsp + 0xc0], xmm2
1412dca48: movaps xmmword ptr [rsp + 0xb0], xmm1
1412dca50: movaps xmmword ptr [rsp + 0xa0], xmm0
1412dca58: lea rcx, [rsp + 0x40]
1412dca5d: mov r8, rsi
1412dca60: call 0x140ceb900 ; bevy_ecs::world::entity_fetch::impl$1::fetch_mut
1412dca65: mov rdx, qword ptr [rsp + 0x40]
1412dca6a: mov rax, qword ptr [rsp + 0x48]
1412dca6f: mov rcx, qword ptr [rsp + 0x50]
1412dca74: test rdx, rdx
1412dca77: je 0x1412dcb05
1412dca7d: mov r8, qword ptr [rsp + 0x58]
1412dca82: mov qword ptr [rsp + 0x20], rdx
1412dca87: mov qword ptr [rsp + 0x28], rax
1412dca8c: mov qword ptr [rsp + 0x30], rcx
1412dca91: mov qword ptr [rsp + 0x38], r8
1412dca96: movaps xmm0, xmmword ptr [rsp + 0xf0]
1412dca9e: movaps xmmword ptr [rsp + 0x90], xmm0
1412dcaa6: movaps xmm0, xmmword ptr [rsp + 0xe0]
1412dcaae: movaps xmmword ptr [rsp + 0x80], xmm0
1412dcab6: movaps xmm0, xmmword ptr [rsp + 0xa0]
1412dcabe: movaps xmm1, xmmword ptr [rsp + 0xb0]
1412dcac6: movaps xmm2, xmmword ptr [rsp + 0xc0]
1412dcace: movaps xmm3, xmmword ptr [rsp + 0xd0]
1412dcad6: movaps xmmword ptr [rsp + 0x70], xmm3
1412dcadb: movaps xmmword ptr [rsp + 0x60], xmm2
1412dcae0: movaps xmmword ptr [rsp + 0x50], xmm1
1412dcae5: movaps xmmword ptr [rsp + 0x40], xmm0
1412dcaea: lea rcx, [rsp + 0x40]
1412dcaef: lea rdx, [rsp + 0x20]
1412dcaf4: call 0x14203e2d0 ; bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::StairsStep> > >
1412dcaf9: jmp 0x1412dcb0b
1412dcafb: add rsp, 0x108
1412dcb02: pop rdi
1412dcb03: pop rsi
1412dcb04: ret 
1412dcb05: cmp rax, 2
1412dcb09: jne 0x1412dcb2c
1412dcb0b: mov rcx, rsi
1412dcb0e: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
1412dcb13: mov rcx, rsi
1412dcb16: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
1412dcb1b: mov rcx, rsi
1412dcb1e: add rsp, 0x108
1412dcb25: pop rdi
1412dcb26: pop rsi
1412dcb27: jmp 0x140cf8200
1412dcb2c: mov qword ptr [rsp + 0x20], rax
1412dcb31: mov qword ptr [rsp + 0x28], rcx
1412dcb36: lea rcx, [rsp + 0x20]
1412dcb3b: call 0x1428a81c0 ; bevy_ecs::error::bevy_error::impl$1::from<enum2$<bevy_ecs::world::error::EntityMutableFetchError> >
1412dcb40: movabs rcx, 0x8000000000000000
1412dcb4a: mov qword ptr [rsp + 0x48], rcx
1412dcb4f: lea rcx, [rip + 0x19925b8]
1412dcb56: mov qword ptr [rsp + 0x50], rcx
1412dcb5b: mov qword ptr [rsp + 0x58], 0x16b
1412dcb64: mov dword ptr [rsp + 0x40], 2
1412dcb6c: lea rdx, [rsp + 0x40]
1412dcb71: mov rcx, rax
1412dcb74: call rdi
1412dcb76: jmp 0x1412dcb0b
