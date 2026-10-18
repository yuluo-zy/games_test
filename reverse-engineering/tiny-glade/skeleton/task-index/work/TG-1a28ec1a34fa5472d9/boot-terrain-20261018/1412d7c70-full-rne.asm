1412d7c70: push rsi
1412d7c71: push rdi
1412d7c72: sub rsp, 0xe8
1412d7c79: add qword ptr [r8], 0x70
1412d7c7d: test rdx, rdx
1412d7c80: je 0x1412d7d5b
1412d7c86: mov rsi, rdx
1412d7c89: mov rdi, qword ptr [rcx + 0x60]
1412d7c8d: mov rdx, qword ptr [rcx + 0x50]
1412d7c91: movups xmm0, xmmword ptr [rcx + 0x40]
1412d7c95: movaps xmmword ptr [rsp + 0xd0], xmm0
1412d7c9d: movups xmm0, xmmword ptr [rcx]
1412d7ca0: movups xmm1, xmmword ptr [rcx + 0x10]
1412d7ca4: movups xmm2, xmmword ptr [rcx + 0x20]
1412d7ca8: movups xmm3, xmmword ptr [rcx + 0x30]
1412d7cac: movaps xmmword ptr [rsp + 0xc0], xmm3
1412d7cb4: movaps xmmword ptr [rsp + 0xb0], xmm2
1412d7cbc: movaps xmmword ptr [rsp + 0xa0], xmm1
1412d7cc4: movaps xmmword ptr [rsp + 0x90], xmm0
1412d7ccc: lea rcx, [rsp + 0x40]
1412d7cd1: mov r8, rsi
1412d7cd4: call 0x140ceb900 ; bevy_ecs::world::entity_fetch::impl$1::fetch_mut
1412d7cd9: mov rdx, qword ptr [rsp + 0x40]
1412d7cde: mov rax, qword ptr [rsp + 0x48]
1412d7ce3: mov rcx, qword ptr [rsp + 0x50]
1412d7ce8: test rdx, rdx
1412d7ceb: je 0x1412d7d65
1412d7ced: mov r8, qword ptr [rsp + 0x58]
1412d7cf2: mov qword ptr [rsp + 0x20], rdx
1412d7cf7: mov qword ptr [rsp + 0x28], rax
1412d7cfc: mov qword ptr [rsp + 0x30], rcx
1412d7d01: mov qword ptr [rsp + 0x38], r8
1412d7d06: movaps xmm0, xmmword ptr [rsp + 0xd0]
1412d7d0e: movaps xmmword ptr [rsp + 0x80], xmm0
1412d7d16: movaps xmm0, xmmword ptr [rsp + 0x90]
1412d7d1e: movaps xmm1, xmmword ptr [rsp + 0xa0]
1412d7d26: movaps xmm2, xmmword ptr [rsp + 0xb0]
1412d7d2e: movaps xmm3, xmmword ptr [rsp + 0xc0]
1412d7d36: movaps xmmword ptr [rsp + 0x70], xmm3
1412d7d3b: movaps xmmword ptr [rsp + 0x60], xmm2
1412d7d40: movaps xmmword ptr [rsp + 0x50], xmm1
1412d7d45: movaps xmmword ptr [rsp + 0x40], xmm0
1412d7d4a: lea rcx, [rsp + 0x40]
1412d7d4f: lea rdx, [rsp + 0x20]
1412d7d54: call 0x14201bd60 ; bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle> > >
1412d7d59: jmp 0x1412d7d6b
1412d7d5b: add rsp, 0xe8
1412d7d62: pop rdi
1412d7d63: pop rsi
1412d7d64: ret 
1412d7d65: cmp rax, 2
1412d7d69: jne 0x1412d7d8c
1412d7d6b: mov rcx, rsi
1412d7d6e: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
1412d7d73: mov rcx, rsi
1412d7d76: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
1412d7d7b: mov rcx, rsi
1412d7d7e: add rsp, 0xe8
1412d7d85: pop rdi
1412d7d86: pop rsi
1412d7d87: jmp 0x140cf8200
1412d7d8c: mov qword ptr [rsp + 0x20], rax
1412d7d91: mov qword ptr [rsp + 0x28], rcx
1412d7d96: lea rcx, [rsp + 0x20]
1412d7d9b: call 0x1428a81c0 ; bevy_ecs::error::bevy_error::impl$1::from<enum2$<bevy_ecs::world::error::EntityMutableFetchError> >
1412d7da0: movabs rcx, 0x8000000000000000
1412d7daa: mov qword ptr [rsp + 0x48], rcx
1412d7daf: lea rcx, [rip + 0x1997a89]
1412d7db6: mov qword ptr [rsp + 0x50], rcx
1412d7dbb: mov qword ptr [rsp + 0x58], 0x145
1412d7dc4: mov dword ptr [rsp + 0x40], 2
1412d7dcc: lea rdx, [rsp + 0x40]
1412d7dd1: mov rcx, rax
1412d7dd4: call rdi
1412d7dd6: jmp 0x1412d7d6b
