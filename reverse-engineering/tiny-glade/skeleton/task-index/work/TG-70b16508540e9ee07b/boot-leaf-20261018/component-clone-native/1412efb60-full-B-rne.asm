1412efb60: 4883ec58 sub rsp, 0x58
1412efb64: 488b4108 mov rax, qword ptr [rcx + 8]
1412efb68: 48833800 cmp qword ptr [rax], 0
1412efb6c: 744d je 0x1412efbbb
1412efb6e: 49b8c9c4a7268ce362fc movabs r8, 0xfc62e38c26a7c4c9
1412efb78: 4c394010 cmp qword ptr [rax + 0x10], r8
1412efb7c: 753d jne 0x1412efbbb
1412efb7e: 49b8d9a34cc82f02daa0 movabs r8, 0xa0da022fc84ca3d9
1412efb88: 4c394008 cmp qword ptr [rax + 8], r8
1412efb8c: 752d jne 0x1412efbbb
1412efb8e: 488b01 mov rax, qword ptr [rcx]
1412efb91: 0f2800 movaps xmm0, xmmword ptr [rax]
1412efb94: 0f284810 movaps xmm1, xmmword ptr [rax + 0x10]
1412efb98: 0f285020 movaps xmm2, xmmword ptr [rax + 0x20]
1412efb9c: 0f29542440 movaps xmmword ptr [rsp + 0x40], xmm2
1412efba1: 0f294c2430 movaps xmmword ptr [rsp + 0x30], xmm1
1412efba6: 0f29442420 movaps xmmword ptr [rsp + 0x20], xmm0
1412efbab: 488d442420 lea rax, [rsp + 0x20]
1412efbb0: 4889d1 mov rcx, rdx
1412efbb3: 4889c2 mov rdx, rax
1412efbb6: e8657b4d00 call 0x1417c7720 ; bevy_ecs::entity::clone_entities::ComponentCloneCtx::write_target_component<country_core::components::drawable::PrevTransform>
1412efbbb: 90 nop 
1412efbbc: 4883c458 add rsp, 0x58
1412efbc0: c3 ret 
