1412dbb00: push rsi
1412dbb01: push rdi
1412dbb02: sub rsp, 0x108
1412dbb09: sub qword ptr [r8], -0x80
1412dbb0d: test rdx, rdx
1412dbb10: je 0x1412dbc0b
1412dbb16: mov rsi, rdx
1412dbb19: mov rdi, qword ptr [rcx + 0x70]
1412dbb1d: mov rdx, qword ptr [rcx + 0x60]
1412dbb21: movups xmm0, xmmword ptr [rcx + 0x50]
1412dbb25: movaps xmmword ptr [rsp + 0xf0], xmm0
1412dbb2d: movups xmm0, xmmword ptr [rcx + 0x40]
1412dbb31: movaps xmmword ptr [rsp + 0xe0], xmm0
1412dbb39: movups xmm0, xmmword ptr [rcx]
1412dbb3c: movups xmm1, xmmword ptr [rcx + 0x10]
1412dbb40: movups xmm2, xmmword ptr [rcx + 0x20]
1412dbb44: movups xmm3, xmmword ptr [rcx + 0x30]
1412dbb48: movaps xmmword ptr [rsp + 0xd0], xmm3
1412dbb50: movaps xmmword ptr [rsp + 0xc0], xmm2
1412dbb58: movaps xmmword ptr [rsp + 0xb0], xmm1
1412dbb60: movaps xmmword ptr [rsp + 0xa0], xmm0
1412dbb68: lea rcx, [rsp + 0x40]
1412dbb6d: mov r8, rsi
1412dbb70: call 0x140ceb900 ; bevy_ecs::world::entity_fetch::impl$1::fetch_mut
1412dbb75: mov rdx, qword ptr [rsp + 0x40]
1412dbb7a: mov rax, qword ptr [rsp + 0x48]
1412dbb7f: mov rcx, qword ptr [rsp + 0x50]
1412dbb84: test rdx, rdx
1412dbb87: je 0x1412dbc15
1412dbb8d: mov r8, qword ptr [rsp + 0x58]
1412dbb92: mov qword ptr [rsp + 0x20], rdx
1412dbb97: mov qword ptr [rsp + 0x28], rax
1412dbb9c: mov qword ptr [rsp + 0x30], rcx
1412dbba1: mov qword ptr [rsp + 0x38], r8
1412dbba6: movaps xmm0, xmmword ptr [rsp + 0xf0]
1412dbbae: movaps xmmword ptr [rsp + 0x90], xmm0
1412dbbb6: movaps xmm0, xmmword ptr [rsp + 0xe0]
1412dbbbe: movaps xmmword ptr [rsp + 0x80], xmm0
1412dbbc6: movaps xmm0, xmmword ptr [rsp + 0xa0]
1412dbbce: movaps xmm1, xmmword ptr [rsp + 0xb0]
1412dbbd6: movaps xmm2, xmmword ptr [rsp + 0xc0]
1412dbbde: movaps xmm3, xmmword ptr [rsp + 0xd0]
1412dbbe6: movaps xmmword ptr [rsp + 0x70], xmm3
1412dbbeb: movaps xmmword ptr [rsp + 0x60], xmm2
1412dbbf0: movaps xmmword ptr [rsp + 0x50], xmm1
1412dbbf5: movaps xmmword ptr [rsp + 0x40], xmm0
1412dbbfa: lea rcx, [rsp + 0x40]
1412dbbff: lea rdx, [rsp + 0x20]
1412dbc04: call 0x14202e160 ; bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::BackgroundTerrain,country_core::components::DontReceiveContactShadows,country_core::components::OverwriteRoughness> > >
1412dbc09: jmp 0x1412dbc1b
1412dbc0b: add rsp, 0x108
1412dbc12: pop rdi
1412dbc13: pop rsi
1412dbc14: ret 
1412dbc15: cmp rax, 2
1412dbc19: jne 0x1412dbc3c
1412dbc1b: mov rcx, rsi
1412dbc1e: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
1412dbc23: mov rcx, rsi
1412dbc26: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
1412dbc2b: mov rcx, rsi
1412dbc2e: add rsp, 0x108
1412dbc35: pop rdi
1412dbc36: pop rsi
1412dbc37: jmp 0x140cf8200
1412dbc3c: mov qword ptr [rsp + 0x20], rax
1412dbc41: mov qword ptr [rsp + 0x28], rcx
1412dbc46: lea rcx, [rsp + 0x20]
1412dbc4b: call 0x1428a81c0 ; bevy_ecs::error::bevy_error::impl$1::from<enum2$<bevy_ecs::world::error::EntityMutableFetchError> >
1412dbc50: movabs rcx, 0x8000000000000000
1412dbc5a: mov qword ptr [rsp + 0x48], rcx
1412dbc5f: lea rcx, [rip + 0x1996bae]
1412dbc66: mov qword ptr [rsp + 0x50], rcx
1412dbc6b: mov qword ptr [rsp + 0x58], 0x1d5
1412dbc74: mov dword ptr [rsp + 0x40], 2
1412dbc7c: lea rdx, [rsp + 0x40]
1412dbc81: mov rcx, rax
1412dbc84: call rdi
1412dbc86: jmp 0x1412dbc1b
