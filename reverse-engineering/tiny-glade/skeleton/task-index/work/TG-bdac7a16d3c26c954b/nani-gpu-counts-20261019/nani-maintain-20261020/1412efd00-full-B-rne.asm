1412efd00: 488b4108 mov rax, qword ptr [rcx + 8]
1412efd04: 48833800 cmp qword ptr [rax], 0
1412efd08: 7430 je 0x1412efd3a
1412efd0a: 49b83815e1791d76a7da movabs r8, 0xdaa7761d79e11538
1412efd14: 4c394010 cmp qword ptr [rax + 0x10], r8
1412efd18: 7520 jne 0x1412efd3a
1412efd1a: 49b8df13ac4c1f79bee9 movabs r8, 0xe9be791f4cac13df
1412efd24: 4c394008 cmp qword ptr [rax + 8], r8
1412efd28: 7510 jne 0x1412efd3a
1412efd2a: 488b01 mov rax, qword ptr [rcx]
1412efd2d: 0fb600 movzx eax, byte ptr [rax]
1412efd30: 4889d1 mov rcx, rdx
1412efd33: 89c2 mov edx, eax
1412efd35: e976834d00 jmp 0x1417c80b0 ; bevy_ecs::entity::clone_entities::ComponentCloneCtx::write_target_component<country_core::components::drawable::NaniSolidTexture>
1412efd3a: c3 ret 
