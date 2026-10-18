1412da440: push rsi
1412da441: push rdi
1412da442: sub rsp, 0x108
1412da449: sub qword ptr [r8], -0x80
1412da44d: test rdx, rdx
1412da450: je 0x1412da54b
1412da456: mov rsi, rdx
1412da459: mov rdi, qword ptr [rcx + 0x70]
1412da45d: mov rdx, qword ptr [rcx + 0x60]
1412da461: movups xmm0, xmmword ptr [rcx + 0x50]
1412da465: movaps xmmword ptr [rsp + 0xf0], xmm0
1412da46d: movups xmm0, xmmword ptr [rcx + 0x40]
1412da471: movaps xmmword ptr [rsp + 0xe0], xmm0
1412da479: movups xmm0, xmmword ptr [rcx]
1412da47c: movups xmm1, xmmword ptr [rcx + 0x10]
1412da480: movups xmm2, xmmword ptr [rcx + 0x20]
1412da484: movups xmm3, xmmword ptr [rcx + 0x30]
1412da488: movaps xmmword ptr [rsp + 0xd0], xmm3
1412da490: movaps xmmword ptr [rsp + 0xc0], xmm2
1412da498: movaps xmmword ptr [rsp + 0xb0], xmm1
1412da4a0: movaps xmmword ptr [rsp + 0xa0], xmm0
1412da4a8: lea rcx, [rsp + 0x40]
1412da4ad: mov r8, rsi
1412da4b0: call 0x140ceb900 ; bevy_ecs::world::entity_fetch::impl$1::fetch_mut
1412da4b5: mov rdx, qword ptr [rsp + 0x40]
1412da4ba: mov rax, qword ptr [rsp + 0x48]
1412da4bf: mov rcx, qword ptr [rsp + 0x50]
1412da4c4: test rdx, rdx
1412da4c7: je 0x1412da555
1412da4cd: mov r8, qword ptr [rsp + 0x58]
1412da4d2: mov qword ptr [rsp + 0x20], rdx
1412da4d7: mov qword ptr [rsp + 0x28], rax
1412da4dc: mov qword ptr [rsp + 0x30], rcx
1412da4e1: mov qword ptr [rsp + 0x38], r8
1412da4e6: movaps xmm0, xmmword ptr [rsp + 0xf0]
1412da4ee: movaps xmmword ptr [rsp + 0x90], xmm0
1412da4f6: movaps xmm0, xmmword ptr [rsp + 0xe0]
1412da4fe: movaps xmmword ptr [rsp + 0x80], xmm0
1412da506: movaps xmm0, xmmword ptr [rsp + 0xa0]
1412da50e: movaps xmm1, xmmword ptr [rsp + 0xb0]
1412da516: movaps xmm2, xmmword ptr [rsp + 0xc0]
1412da51e: movaps xmm3, xmmword ptr [rsp + 0xd0]
1412da526: movaps xmmword ptr [rsp + 0x70], xmm3
1412da52b: movaps xmmword ptr [rsp + 0x60], xmm2
1412da530: movaps xmmword ptr [rsp + 0x50], xmm1
1412da535: movaps xmmword ptr [rsp + 0x40], xmm0
1412da53a: lea rcx, [rsp + 0x40]
1412da53f: lea rdx, [rsp + 0x20]
1412da544: call 0x14203ecb0 ; bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DesignatedRenderPass> > >
1412da549: jmp 0x1412da55b
1412da54b: add rsp, 0x108
1412da552: pop rdi
1412da553: pop rsi
1412da554: ret 
1412da555: cmp rax, 2
1412da559: jne 0x1412da57c
1412da55b: mov rcx, rsi
1412da55e: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
1412da563: mov rcx, rsi
1412da566: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
1412da56b: mov rcx, rsi
1412da56e: add rsp, 0x108
1412da575: pop rdi
1412da576: pop rsi
1412da577: jmp 0x140cf8200
1412da57c: mov qword ptr [rsp + 0x20], rax
1412da581: mov qword ptr [rsp + 0x28], rcx
1412da586: lea rcx, [rsp + 0x20]
1412da58b: call 0x1428a81c0 ; bevy_ecs::error::bevy_error::impl$1::from<enum2$<bevy_ecs::world::error::EntityMutableFetchError> >
1412da590: movabs rcx, 0x8000000000000000
1412da59a: mov qword ptr [rsp + 0x48], rcx
1412da59f: lea rcx, [rip + 0x199a8f3]
1412da5a6: mov qword ptr [rsp + 0x50], rcx
1412da5ab: mov qword ptr [rsp + 0x58], 0x17f
1412da5b4: mov dword ptr [rsp + 0x40], 2
1412da5bc: lea rdx, [rsp + 0x40]
1412da5c1: mov rcx, rax
1412da5c4: call rdi
1412da5c6: jmp 0x1412da55b
