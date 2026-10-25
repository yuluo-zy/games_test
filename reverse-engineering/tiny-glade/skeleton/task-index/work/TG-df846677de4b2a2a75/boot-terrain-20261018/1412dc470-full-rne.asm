1412dc470: push rsi
1412dc471: push rdi
1412dc472: sub rsp, 0x108
1412dc479: sub qword ptr [r8], -0x80
1412dc47d: test rdx, rdx
1412dc480: je 0x1412dc57b
1412dc486: mov rsi, rdx
1412dc489: mov rdi, qword ptr [rcx + 0x70]
1412dc48d: mov rdx, qword ptr [rcx + 0x60]
1412dc491: movups xmm0, xmmword ptr [rcx + 0x50]
1412dc495: movaps xmmword ptr [rsp + 0xf0], xmm0
1412dc49d: movups xmm0, xmmword ptr [rcx + 0x40]
1412dc4a1: movaps xmmword ptr [rsp + 0xe0], xmm0
1412dc4a9: movups xmm0, xmmword ptr [rcx]
1412dc4ac: movups xmm1, xmmword ptr [rcx + 0x10]
1412dc4b0: movups xmm2, xmmword ptr [rcx + 0x20]
1412dc4b4: movups xmm3, xmmword ptr [rcx + 0x30]
1412dc4b8: movaps xmmword ptr [rsp + 0xd0], xmm3
1412dc4c0: movaps xmmword ptr [rsp + 0xc0], xmm2
1412dc4c8: movaps xmmword ptr [rsp + 0xb0], xmm1
1412dc4d0: movaps xmmword ptr [rsp + 0xa0], xmm0
1412dc4d8: lea rcx, [rsp + 0x40]
1412dc4dd: mov r8, rsi
1412dc4e0: call 0x140ceb900 ; bevy_ecs::world::entity_fetch::impl$1::fetch_mut
1412dc4e5: mov rdx, qword ptr [rsp + 0x40]
1412dc4ea: mov rax, qword ptr [rsp + 0x48]
1412dc4ef: mov rcx, qword ptr [rsp + 0x50]
1412dc4f4: test rdx, rdx
1412dc4f7: je 0x1412dc585
1412dc4fd: mov r8, qword ptr [rsp + 0x58]
1412dc502: mov qword ptr [rsp + 0x20], rdx
1412dc507: mov qword ptr [rsp + 0x28], rax
1412dc50c: mov qword ptr [rsp + 0x30], rcx
1412dc511: mov qword ptr [rsp + 0x38], r8
1412dc516: movaps xmm0, xmmword ptr [rsp + 0xf0]
1412dc51e: movaps xmmword ptr [rsp + 0x90], xmm0
1412dc526: movaps xmm0, xmmword ptr [rsp + 0xe0]
1412dc52e: movaps xmmword ptr [rsp + 0x80], xmm0
1412dc536: movaps xmm0, xmmword ptr [rsp + 0xa0]
1412dc53e: movaps xmm1, xmmword ptr [rsp + 0xb0]
1412dc546: movaps xmm2, xmmword ptr [rsp + 0xc0]
1412dc54e: movaps xmm3, xmmword ptr [rsp + 0xd0]
1412dc556: movaps xmmword ptr [rsp + 0x70], xmm3
1412dc55b: movaps xmmword ptr [rsp + 0x60], xmm2
1412dc560: movaps xmmword ptr [rsp + 0x50], xmm1
1412dc565: movaps xmmword ptr [rsp + 0x40], xmm0
1412dc56a: lea rcx, [rsp + 0x40]
1412dc56f: lea rdx, [rsp + 0x20]
1412dc574: call 0x142042db0 ; bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::components::drawable::DesignatedRenderPass> > >
1412dc579: jmp 0x1412dc58b
1412dc57b: add rsp, 0x108
1412dc582: pop rdi
1412dc583: pop rsi
1412dc584: ret 
1412dc585: cmp rax, 2
1412dc589: jne 0x1412dc5ac
1412dc58b: mov rcx, rsi
1412dc58e: call 0x140cf81b0 ; bevy_ecs::world::World::flush_entities
1412dc593: mov rcx, rsi
1412dc596: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
1412dc59b: mov rcx, rsi
1412dc59e: add rsp, 0x108
1412dc5a5: pop rdi
1412dc5a6: pop rsi
1412dc5a7: jmp 0x140cf8200
1412dc5ac: mov qword ptr [rsp + 0x20], rax
1412dc5b1: mov qword ptr [rsp + 0x28], rcx
1412dc5b6: lea rcx, [rsp + 0x20]
1412dc5bb: call 0x1428a81c0 ; bevy_ecs::error::bevy_error::impl$1::from<enum2$<bevy_ecs::world::error::EntityMutableFetchError> >
1412dc5c0: movabs rcx, 0x8000000000000000
1412dc5ca: mov qword ptr [rsp + 0x48], rcx
1412dc5cf: lea rcx, [rip + 0x1997f1c]
1412dc5d6: mov qword ptr [rsp + 0x50], rcx
1412dc5db: mov qword ptr [rsp + 0x58], 0x186
1412dc5e4: mov dword ptr [rsp + 0x40], 2
1412dc5ec: lea rdx, [rsp + 0x40]
1412dc5f1: mov rcx, rax
1412dc5f4: call rdi
1412dc5f6: jmp 0x1412dc58b
