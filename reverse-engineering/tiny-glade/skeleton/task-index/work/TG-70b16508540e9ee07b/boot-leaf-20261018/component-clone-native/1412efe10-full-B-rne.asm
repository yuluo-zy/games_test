1412efe10: 4883ec38 sub rsp, 0x38
1412efe14: 488b4108 mov rax, qword ptr [rcx + 8]
1412efe18: 48833800 cmp qword ptr [rax], 0
1412efe1c: 7459 je 0x1412efe77
1412efe1e: 49b8d6c8b2293dff62c6 movabs r8, 0xc662ff3d29b2c8d6
1412efe28: 4c394010 cmp qword ptr [rax + 0x10], r8
1412efe2c: 7549 jne 0x1412efe77
1412efe2e: 49b891ee76fb635c2c0e movabs r8, 0xe2c5c63fb76ee91
1412efe38: 4c394008 cmp qword ptr [rax + 8], r8
1412efe3c: 7539 jne 0x1412efe77
1412efe3e: 488b01 mov rax, qword ptr [rcx]
1412efe41: 8b08 mov ecx, dword ptr [rax]
1412efe43: f30f10400c movss xmm0, dword ptr [rax + 0xc]
1412efe48: 440fb64010 movzx r8d, byte ptr [rax + 0x10]
1412efe4d: 894c2424 mov dword ptr [rsp + 0x24], ecx
1412efe51: f20f104804 movsd xmm1, qword ptr [rax + 4]
1412efe56: f20f114c2428 movsd qword ptr [rsp + 0x28], xmm1
1412efe5c: f30f11442430 movss dword ptr [rsp + 0x30], xmm0
1412efe62: 4488442434 mov byte ptr [rsp + 0x34], r8b
1412efe67: 488d442424 lea rax, [rsp + 0x24]
1412efe6c: 4889d1 mov rcx, rdx
1412efe6f: 4889c2 mov rdx, rax
1412efe72: e889874d00 call 0x1417c8600 ; bevy_ecs::entity::clone_entities::ComponentCloneCtx::write_target_component<country_core::components::WallFlagCloth>
1412efe77: 90 nop 
1412efe78: 4883c438 add rsp, 0x38
1412efe7c: c3 ret 
