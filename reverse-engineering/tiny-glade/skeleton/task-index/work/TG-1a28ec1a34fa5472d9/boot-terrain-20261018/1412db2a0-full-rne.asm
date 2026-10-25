1412db2a0: push rsi
1412db2a1: push rdi
1412db2a2: sub rsp, 0xe8
1412db2a9: add qword ptr [r8], 0x70
1412db2ad: test rdx, rdx
1412db2b0: je 0x1412db38b
1412db2b6: mov rsi, rdx
1412db2b9: mov rdi, qword ptr [rcx + 0x60]
1412db2bd: mov rdx, qword ptr [rcx + 0x50]
1412db2c1: movups xmm0, xmmword ptr [rcx + 0x40]
1412db2c5: movaps xmmword ptr [rsp + 0xd0], xmm0
1412db2cd: movups xmm0, xmmword ptr [rcx]
1412db2d0: movups xmm1, xmmword ptr [rcx + 0x10]
1412db2d4: movups xmm2, xmmword ptr [rcx + 0x20]
1412db2d8: movups xmm3, xmmword ptr [rcx + 0x30]
1412db2dc: movaps xmmword ptr [rsp + 0xc0], xmm3
1412db2e4: movaps xmmword ptr [rsp + 0xb0], xmm2
1412db2ec: movaps xmmword ptr [rsp + 0xa0], xmm1
1412db2f4: movaps xmmword ptr [rsp + 0x90], xmm0
1412db2fc: lea rcx, [rsp + 0x40]
1412db301: mov r8, rsi
1412db304: call 0x140ceb900 ; bevy_ecs::world::entity_fetch::impl$1::fetch_mut
1412db309: mov rdx, qword ptr [rsp + 0x40]
1412db30e: mov rax, qword ptr [rsp + 0x48]
1412db313: mov rcx, qword ptr [rsp + 0x50]
1412db318: test rdx, rdx
1412db31b: je 0x1412db395
1412db31d: mov r8, qword ptr [rsp + 0x58]
1412db322: mov qword ptr [rsp + 0x20], rdx
1412db327: mov qword ptr [rsp + 0x28], rax
1412db32c: mov qword ptr [rsp + 0x30], rcx
1412db331: mov qword ptr [rsp + 0x38], r8
1412db336: movaps xmm0, xmmword ptr [rsp + 0xd0]
1412db33e: movaps xmmword ptr [rsp + 0x80], xmm0
1412db346: movaps xmm0, xmmword ptr [rsp + 0x90]
1412db34e: movaps xmm1, xmmword ptr [rsp + 0xa0]
1412db356: movaps xmm2, xmmword ptr [rsp + 0xb0]
1412db35e: movaps xmm3, xmmword ptr [rsp + 0xc0]
1412db366: movaps xmmword ptr [rsp + 0x70], xmm3
1412db36b: movaps xmmword ptr [rsp + 0x60], xmm2
1412db370: movaps xmmword ptr [rsp + 0x50], xmm1
1412db375: movaps xmmword ptr [rsp + 0x40], xmm0
1412db37a: lea rcx, [rsp + 0x40]
1412db37f: lea rdx, [rsp + 0x20]
1412db384: call 0x14201d5e0 ; bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow> > >
1412db389: jmp 0x1412db39b
1412db38b: add rsp, 0xe8
1412db392: pop rdi
1412db393: pop rsi
1412db394: ret 
1412db395: cmp rax, 2
1412db399: jne 0x1412db3bc
1412db39b: mov rcx, rsi
1412db39e: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
1412db3a3: mov rcx, rsi
1412db3a6: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
1412db3ab: mov rcx, rsi
1412db3ae: add rsp, 0xe8
1412db3b5: pop rdi
1412db3b6: pop rsi
1412db3b7: jmp 0x140cf8200
1412db3bc: mov qword ptr [rsp + 0x20], rax
1412db3c1: mov qword ptr [rsp + 0x28], rcx
1412db3c6: lea rcx, [rsp + 0x20]
1412db3cb: call 0x1428a81c0 ; bevy_ecs::error::bevy_error::impl$1::from<enum2$<bevy_ecs::world::error::EntityMutableFetchError> >
1412db3d0: movabs rcx, 0x8000000000000000
1412db3da: mov qword ptr [rsp + 0x48], rcx
1412db3df: lea rcx, [rip + 0x1994e9e]
1412db3e6: mov qword ptr [rsp + 0x50], rcx
1412db3eb: mov qword ptr [rsp + 0x58], 0x179
1412db3f4: mov dword ptr [rsp + 0x40], 2
1412db3fc: lea rdx, [rsp + 0x40]
1412db401: mov rcx, rax
1412db404: call rdi
1412db406: jmp 0x1412db39b
