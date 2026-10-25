1409daaa0: push rbp
1409daaa1: push r15
1409daaa3: push r14
1409daaa5: push r13
1409daaa7: push r12
1409daaa9: push rsi
1409daaaa: push rdi
1409daaab: push rbx
1409daaac: sub rsp, 0x9d8
1409daab3: lea rbp, [rsp + 0x80]
1409daabb: movaps xmmword ptr [rbp + 0x940], xmm6
1409daac2: mov qword ptr [rbp + 0x938], 0xfffffffffffffffe
1409daacd: mov rsi, rdx
1409daad0: mov qword ptr [rbp + 0x928], rcx
1409daad7: call 0x140c16a00 ; puffin::are_scopes_on
1409daadc: test al, al
1409daade: je 0x1409dab3f
1409daae0: lea r14, [rip + 0x20e7439]
1409daae7: mov ebx, 0x5a
1409daaec: mov edx, 0x5a
1409daaf1: mov rcx, r14
1409daaf4: call 0x1409d42d0 ; core::str::impl$0::rfind<ref$<str$> >
1409daaf9: cmp rax, 1
1409daafd: jne 0x1409dabb4
1409dab03: mov r9, rdx
1409dab06: test rdx, rdx
1409dab09: je 0x1409dab48
1409dab0b: cmp r9, 0x5a
1409dab0f: jae 0x1409dab46
1409dab11: lea rax, [rip + 0x20e7408]
1409dab18: cmp byte ptr [r9 + rax], 0xbf
1409dab1d: jg 0x1409dab48
1409dab1f: lea rax, [rip + 0x20e74da]
1409dab26: mov qword ptr [rsp + 0x20], rax
1409dab2b: lea rcx, [rip + 0x20e73ee]
1409dab32: mov edx, 0x5a
1409dab37: xor r8d, r8d
1409dab3a: call 0x1428d9940 ; core::str::slice_error_fail
1409dab3f: xor eax, eax
1409dab41: jmp 0x1409dad1d
1409dab46: jne 0x1409dab1f
1409dab48: lea r14, [rip + 0x20e73d1]
1409dab4f: mov rcx, r14
1409dab52: mov rdx, r9
1409dab55: call 0x1409d42d0 ; core::str::impl$0::rfind<ref$<str$> >
1409dab5a: cmp rax, 1
1409dab5e: jne 0x1409dabb4
1409dab60: mov r8, rdx
1409dab63: add r8, 2
1409dab67: je 0x1409daba2
1409dab69: cmp r8, 0x5a
1409dab6d: jae 0x1409daba0
1409dab6f: lea rax, [rip + 0x20e73aa]
1409dab76: cmp byte ptr [r8 + rax], 0xbf
1409dab7b: jg 0x1409daba2
1409dab7d: lea rax, [rip + 0x20e7494]
1409dab84: mov qword ptr [rsp + 0x20], rax
1409dab89: lea rcx, [rip + 0x20e7390]
1409dab90: mov edx, 0x5a
1409dab95: mov r9d, 0x5a
1409dab9b: call 0x1428d9940 ; core::str::slice_error_fail
1409daba0: jne 0x1409dab7d
1409daba2: mov ebx, 0x58
1409daba7: sub rbx, rdx
1409dabaa: lea r14, [rip + 0x20e736f]
1409dabb1: add r14, r8
1409dabb4: lea rax, [rip + 0x20e7f61]
1409dabbb: lea r15, [rip + 0x20e7f0e]
1409dabc2: nop word ptr cs:[rax + rax]
1409dabd0: movsx ecx, byte ptr [rax - 1]
1409dabd4: test ecx, ecx
1409dabd6: js 0x1409dabf0
1409dabd8: dec rax
1409dabdb: cmp ecx, 0x5c
1409dabde: jne 0x1409dac49
1409dabe0: jmp 0x1409dac5e
1409dabe2: nop word ptr cs:[rax + rax]
1409dabf0: movzx edx, byte ptr [rax - 2]
1409dabf4: cmp dl, 0xc0
1409dabf7: jge 0x1409dac1e
1409dabf9: movzx r8d, byte ptr [rax - 3]
1409dabfe: cmp r8b, 0xc0
1409dac02: jge 0x1409dac27
1409dac04: movzx r9d, byte ptr [rax - 4]
1409dac09: add rax, -4
1409dac0d: and r9d, 7
1409dac11: shl r9d, 6
1409dac15: and r8d, 0x3f
1409dac19: or r8d, r9d
1409dac1c: jmp 0x1409dac2f
1409dac1e: add rax, -2
1409dac22: and edx, 0x1f
1409dac25: jmp 0x1409dac39
1409dac27: add rax, -3
1409dac2b: and r8d, 0xf
1409dac2f: shl r8d, 6
1409dac33: and edx, 0x3f
1409dac36: or edx, r8d
1409dac39: shl edx, 6
1409dac3c: and cl, 0x3f
1409dac3f: movzx ecx, cl
1409dac42: or ecx, edx
1409dac44: cmp ecx, 0x5c
1409dac47: je 0x1409dac5e
1409dac49: cmp ecx, 0x2f
1409dac4c: je 0x1409dac5e
1409dac4e: cmp rax, r15
1409dac51: jne 0x1409dabd0
1409dac57: mov edi, 0x4c
1409dac5c: jmp 0x1409dacad
1409dac5e: lea r15, [rip + 0x20e7e6b]
1409dac65: sub rax, r15
1409dac68: mov r8, rax
1409dac6b: inc r8
1409dac6e: je 0x1409daca2
1409dac70: cmp r8, 0x4c
1409dac74: jae 0x1409daca0
1409dac76: cmp byte ptr [r8 + r15], 0xbf
1409dac7b: jg 0x1409daca2
1409dac7d: lea rax, [rip + 0x20e735c]
1409dac84: mov qword ptr [rsp + 0x20], rax
1409dac89: lea rcx, [rip + 0x20e7e40]
1409dac90: mov edx, 0x4c
1409dac95: mov r9d, 0x4c
1409dac9b: call 0x1428d9940 ; core::str::slice_error_fail
1409daca0: jne 0x1409dac7d
1409daca2: mov edi, 0x4b
1409daca7: sub rdi, rax
1409dacaa: add r15, r8
1409dacad: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1409dacb2: mov rcx, rax
1409dacb5: mov rax, qword ptr [rax]
1409dacb8: cmp rax, 1
1409dacbc: jne 0x1409db63c
1409dacc2: add rcx, 8
1409dacc6: cmp qword ptr [rcx], 0
1409dacca: jne 0x1409db660
1409dacd0: mov qword ptr [rcx], 0xffffffffffffffff
1409dacd7: mov qword ptr [rbp + 0x920], rcx
1409dacde: add rcx, 8
1409dace2: mov qword ptr [rsp + 0x20], rdi
1409dace7: mov qword ptr [rsp + 0x30], 0
1409dacf0: mov qword ptr [rsp + 0x28], 1
1409dacf9: mov rdx, r14
1409dacfc: mov r8, rbx
1409dacff: mov r9, r15
1409dad02: call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1409dad07: mov rcx, qword ptr [rbp + 0x920]
1409dad0e: inc qword ptr [rcx]
1409dad11: mov qword ptr [rbp + 0x908], rax
1409dad18: mov eax, 1
1409dad1d: mov qword ptr [rbp + 0x900], rax
1409dad24: lea r15, [rsi + 0x38]
1409dad28: movabs rdx, 0x8597ee0727b461b3
1409dad32: movabs r8, 0x8b297ec5ae15f15c
1409dad3c: mov rcx, r15
1409dad3f: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409dad44: test rax, rax
1409dad47: je 0x1409db68b
1409dad4d: cmp qword ptr [rsi + 0x218], rdx
1409dad54: jbe 0x1409db68b
1409dad5a: mov rax, qword ptr [rsi + 0x210]
1409dad61: mov rcx, qword ptr [rax + rdx*8]
1409dad65: test rcx, rcx
1409dad68: je 0x1409db68b
1409dad6e: mov rax, qword ptr [rsi + 0x1e0]
1409dad75: not rcx
1409dad78: lea rcx, [rcx + rcx*2]
1409dad7c: shl rcx, 5
1409dad80: cmp qword ptr [rax + rcx + 0x20], 0
1409dad86: je 0x1409db68b
1409dad8c: add rax, rcx
1409dad8f: mov rcx, qword ptr [rax + 0x10]
1409dad93: lea rdx, [rip + 0x20e7d9e]
1409dad9a: mov r8d, 0x11
1409dada0: call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
1409dada5: mov qword ptr [rbp + 0x8f8], rdx
1409dadac: test al, 1
1409dadae: je 0x1409db6d1
1409dadb4: movabs rdx, 0xf9c1dd2a6b8903b0
1409dadbe: movabs r8, 0xf1d4572b4d023299
1409dadc8: mov rcx, r15
1409dadcb: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409dadd0: test rax, rax
1409dadd3: je 0x1409db699
1409dadd9: cmp qword ptr [rsi + 0x218], rdx
1409dade0: jbe 0x1409db699
1409dade6: mov rax, qword ptr [rsi + 0x210]
1409daded: mov rcx, qword ptr [rax + rdx*8]
1409dadf1: test rcx, rcx
1409dadf4: je 0x1409db699
1409dadfa: mov rax, qword ptr [rsi + 0x1e0]
1409dae01: not rcx
1409dae04: lea rcx, [rcx + rcx*2]
1409dae08: shl rcx, 5
1409dae0c: cmp qword ptr [rax + rcx + 0x20], 0
1409dae12: je 0x1409db699
1409dae18: add rax, rcx
1409dae1b: mov rdi, qword ptr [rax + 0x10]
1409dae1f: movabs rdx, 0xa68561b396784eaa
1409dae29: movabs r8, 0xf5fd74e6faf4ed14
1409dae33: mov rcx, r15
1409dae36: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409dae3b: test rax, rax
1409dae3e: je 0x1409db6a7
1409dae44: cmp qword ptr [rsi + 0x218], rdx
1409dae4b: jbe 0x1409db6a7
1409dae51: mov rax, qword ptr [rsi + 0x210]
1409dae58: mov rcx, qword ptr [rax + rdx*8]
1409dae5c: test rcx, rcx
1409dae5f: je 0x1409db6a7
1409dae65: mov rax, qword ptr [rsi + 0x1e0]
1409dae6c: not rcx
1409dae6f: lea rcx, [rcx + rcx*2]
1409dae73: shl rcx, 5
1409dae77: cmp qword ptr [rax + rcx + 0x20], 0
1409dae7d: je 0x1409db6a7
1409dae83: add rax, rcx
1409dae86: mov rdx, qword ptr [rax + 0x10]
1409dae8a: mov eax, dword ptr [rsi + 0x5b0]
1409dae90: lea r14, [rbp + 0x7a4]
1409dae97: mov rcx, r14
1409dae9a: mov r8b, 0xf
1409dae9d: call 0x14093cf30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
1409daea2: movabs rdx, 0xc2b8e941c91a6428
1409daeac: movabs r8, 0x59f3c6d1d3da43a0
1409daeb6: mov rcx, r15
1409daeb9: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409daebe: test rax, rax
1409daec1: je 0x1409db6b5
1409daec7: cmp qword ptr [rsi + 0x218], rdx
1409daece: jbe 0x1409db6b5
1409daed4: mov rax, qword ptr [rsi + 0x210]
1409daedb: mov rcx, qword ptr [rax + rdx*8]
1409daedf: test rcx, rcx
1409daee2: je 0x1409db6b5
1409daee8: mov rax, qword ptr [rsi + 0x1e0]
1409daeef: not rcx
1409daef2: lea rcx, [rcx + rcx*2]
1409daef6: shl rcx, 5
1409daefa: cmp qword ptr [rax + rcx + 0x20], 0
1409daf00: je 0x1409db6b5
1409daf06: add rax, rcx
1409daf09: mov rax, qword ptr [rax + 0x10]
1409daf0d: mov edx, dword ptr [rax]
1409daf0f: lea r8d, [rdx - 6]
1409daf13: add rdx, -5
1409daf17: xor ecx, ecx
1409daf19: cmp r8d, 0xf
1409daf1d: cmovb rcx, rdx
1409daf21: xorps xmm6, xmm6
1409daf24: dec rcx
1409daf27: cmp rcx, 0xd
1409daf2b: ja 0x1409db013
1409daf31: lea rdx, [rip + 0x20e7ea0]
1409daf38: movsxd rcx, dword ptr [rdx + rcx*4]
1409daf3c: add rcx, rdx
1409daf3f: jmp rcx
1409daf41: movss xmm6, dword ptr [rdi]
1409daf45: addss xmm6, dword ptr [rip + 0x1f59743]
1409daf4d: jmp 0x1409db013
1409daf52: cmp byte ptr [rax + 4], 3
1409daf56: je 0x1409db013
1409daf5c: movss xmm6, dword ptr [rip + 0x20e6a0c]
1409daf64: jmp 0x1409db013
1409daf69: cmp byte ptr [rax + 4], 0
1409daf6d: jne 0x1409db013
1409daf73: movss xmm6, dword ptr [rip + 0x1f4a955]
1409daf7b: jmp 0x1409db013
1409daf80: movss xmm6, dword ptr [rdi + 8]
1409daf85: mulss xmm6, dword ptr [rip + 0x2012c03]
1409daf8d: addss xmm6, dword ptr [rip + 0x1f596fb]
1409daf95: jmp 0x1409db013
1409daf97: movabs rdx, 0xbba451ee6a08a3ff
1409dafa1: movabs r8, 0x8d008d15febcf29d
1409dafab: mov rcx, r15
1409dafae: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409dafb3: test rax, rax
1409dafb6: je 0x1409db6fb
1409dafbc: cmp qword ptr [rsi + 0x218], rdx
1409dafc3: jbe 0x1409db6fb
1409dafc9: mov rax, qword ptr [rsi + 0x210]
1409dafd0: mov rcx, qword ptr [rax + rdx*8]
1409dafd4: test rcx, rcx
1409dafd7: je 0x1409db6fb
1409dafdd: mov rax, qword ptr [rsi + 0x1e0]
1409dafe4: not rcx
1409dafe7: lea rcx, [rcx + rcx*2]
1409dafeb: shl rcx, 5
1409dafef: cmp qword ptr [rax + rcx + 0x20], 0
1409daff5: je 0x1409db6fb
1409daffb: add rax, rcx
1409daffe: mov rax, qword ptr [rax + 0x10]
1409db002: cmp byte ptr [rax + 0x4d0], 3
1409db009: jne 0x1409db013
1409db00b: movss xmm6, dword ptr [rip + 0x20a0a71]
1409db013: lea rcx, [rbp + 0x3b8]
1409db01a: mov rdx, rsi
1409db01d: call 0x14165a540 ; bevy_ecs::query::state::QueryState::new<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >
1409db022: lea r12, [rbp + 0x3b8]
1409db029: mov rcx, r12
1409db02c: mov rdx, rsi
1409db02f: call 0x141657e90 ; bevy_ecs::query::state::QueryState::update_archetypes_unsafe_world_cell<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >
1409db034: mov rdx, qword ptr [rsi + 0x5a0]
1409db03b: mov rcx, qword ptr [rbp + 0x3b8]
1409db042: cmp rcx, rdx
1409db045: jne 0x1409db66c
1409db04b: mov eax, dword ptr [rsi + 0x5b0]
1409db051: mov r13, qword ptr [rbp + 0x4b0]
1409db058: mov rdi, qword ptr [rbp + 0x4b8]
1409db05f: movzx ebx, byte ptr [rbp + 0x4c0]
1409db066: mov eax, dword ptr [rbp - 0x30]
1409db069: mov ecx, dword ptr [rbp - 0x2d]
1409db06c: mov dword ptr [rbp + 0x7c8], eax
1409db072: mov dword ptr [rbp + 0x7cb], ecx
1409db078: xorps xmm0, xmm0
1409db07b: ucomiss xmm6, xmm0
1409db07e: jbe 0x1409db18a
1409db084: movabs rdx, 0xbba451ee6a08a3ff
1409db08e: movabs r8, 0x8d008d15febcf29d
1409db098: mov rcx, r15
1409db09b: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409db0a0: test rax, rax
1409db0a3: je 0x1409db6c3
1409db0a9: cmp qword ptr [rsi + 0x218], rdx
1409db0b0: jbe 0x1409db6c3
1409db0b6: mov rax, qword ptr [rsi + 0x210]
1409db0bd: mov rcx, qword ptr [rax + rdx*8]
1409db0c1: test rcx, rcx
1409db0c4: je 0x1409db6c3
1409db0ca: mov rax, qword ptr [rsi + 0x1e0]
1409db0d1: not rcx
1409db0d4: lea rcx, [rcx + rcx*2]
1409db0d8: shl rcx, 5
1409db0dc: cmp qword ptr [rax + rcx + 0x20], 0
1409db0e2: je 0x1409db6c3
1409db0e8: add rax, rcx
1409db0eb: mov rax, qword ptr [rax + 0x10]
1409db0ef: cmp byte ptr [rax + 0x4d0], 3
1409db0f6: jne 0x1409db191
1409db0fc: movabs rdx, 0xbba451ee6a08a3ff
1409db106: movabs r8, 0x8d008d15febcf29d
1409db110: mov rcx, r15
1409db113: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409db118: test rax, rax
1409db11b: je 0x1409db6df
1409db121: cmp qword ptr [rsi + 0x218], rdx
1409db128: jbe 0x1409db6df
1409db12e: mov rax, qword ptr [rsi + 0x210]
1409db135: mov rcx, qword ptr [rax + rdx*8]
1409db139: test rcx, rcx
1409db13c: je 0x1409db6df
1409db142: mov rax, qword ptr [rsi + 0x1e0]
1409db149: not rcx
1409db14c: lea rcx, [rcx + rcx*2]
1409db150: shl rcx, 5
1409db154: cmp qword ptr [rax + rcx + 0x20], 0
1409db15a: je 0x1409db6df
1409db160: add rax, rcx
1409db163: mov rdx, qword ptr [rax + 0x10]
1409db167: lea rcx, [rbp - 0x30]
1409db16b: call 0x140c96d40 ; utils::camera::CameraMatrices::eye_position
1409db170: movsd xmm1, qword ptr [rbp - 0x2c]
1409db175: movss xmm0, dword ptr [rbp - 0x30]
1409db17a: movlhps xmm0, xmm1
1409db17d: shufps xmm6, xmm1, 0xd4
1409db181: shufps xmm0, xmm6, 0x28
1409db185: jmp 0x1409db21a
1409db18a: xor eax, eax
1409db18c: jmp 0x1409db21f
1409db191: movabs rdx, 0x62dfe8f62b6fcc5
1409db19b: movabs r8, 0xf65008e548632ba7
1409db1a5: mov rcx, r15
1409db1a8: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409db1ad: test rax, rax
1409db1b0: je 0x1409db6ed
1409db1b6: cmp qword ptr [rsi + 0x218], rdx
1409db1bd: jbe 0x1409db6ed
1409db1c3: mov rax, qword ptr [rsi + 0x210]
1409db1ca: mov rcx, qword ptr [rax + rdx*8]
1409db1ce: test rcx, rcx
1409db1d1: je 0x1409db6ed
1409db1d7: mov rax, qword ptr [rsi + 0x1e0]
1409db1de: not rcx
1409db1e1: lea rcx, [rcx + rcx*2]
1409db1e5: shl rcx, 5
1409db1e9: cmp qword ptr [rax + rcx + 0x20], 0
1409db1ef: je 0x1409db6ed
1409db1f5: add rax, rcx
1409db1f8: mov rax, qword ptr [rax + 0x10]
1409db1fc: test byte ptr [rax], 1
1409db1ff: je 0x1409db20d
1409db201: movsd xmm0, qword ptr [rax + 4]
1409db206: movss xmm1, dword ptr [rax + 0xc]
1409db20b: jmp 0x1409db213
1409db20d: xorps xmm0, xmm0
1409db210: xorps xmm1, xmm1
1409db213: movlhps xmm6, xmm1
1409db216: shufps xmm0, xmm6, 0x24
1409db21a: mov eax, 1
1409db21f: xor ecx, ecx
1409db221: lea rdx, [rdi*4]
1409db229: add rdx, r13
1409db22c: lea r8, [rsi + 0xf8]
1409db233: lea r9, [rsi + 0x1a0]
1409db23a: mov qword ptr [rbp - 0x10], rsi
1409db23e: mov qword ptr [rbp - 8], r9
1409db242: mov qword ptr [rbp], r8
1409db246: mov qword ptr [rbp + 8], r12
1409db24a: mov qword ptr [rbp + 0x10], r13
1409db24e: mov qword ptr [rbp + 0x18], rdx
1409db252: mov qword ptr [rbp + 0x20], 8
1409db25a: mov qword ptr [rbp + 0x28], 0
1409db262: mov qword ptr [rbp + 0x30], 8
1409db26a: xorps xmm1, xmm1
1409db26d: movups xmmword ptr [rbp + 0x38], xmm1
1409db271: movups xmmword ptr [rbp + 0x48], xmm1
1409db275: mov qword ptr [rbp + 0x58], 0
1409db27d: mov byte ptr [rbp + 0x60], bl
1409db280: mov edx, dword ptr [rbp + 0x7c8]
1409db286: mov r8d, dword ptr [rbp + 0x7cb]
1409db28d: mov dword ptr [rbp + 0x61], edx
1409db290: mov dword ptr [rbp + 0x64], r8d
1409db294: mov qword ptr [rbp - 0x30], rax
1409db298: mov qword ptr [rbp - 0x28], rcx
1409db29c: movaps xmmword ptr [rbp - 0x20], xmm0
1409db2a0: lea r8, [rip + 0x20e6b99]
1409db2a7: lea rcx, [rbp + 0x8e0]
1409db2ae: lea rdx, [rbp - 0x30]
1409db2b2: call 0x1412fde80 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1409db2b7: mov byte ptr [rbp + 0x937], 1
1409db2be: lea rcx, [rbp + 0x3b8]
1409db2c5: call 0x1405d0be0 ; core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> > >
1409db2ca: lea rcx, [rbp + 0x7a8]
1409db2d1: mov byte ptr [rbp + 0x937], 1
1409db2d8: call 0x140bd5790 ; rhapsody::raw::texture::TextureDesc::extent_2d
1409db2dd: mov dword ptr [rbp + 0x3b8], 1
1409db2e7: xorps xmm0, xmm0
1409db2ea: movups xmmword ptr [rbp + 0x3bc], xmm0
1409db2f1: mov dword ptr [rbp + 0x3cc], 0
1409db2fb: mov qword ptr [rbp + 0x3d0], r14
1409db302: mov dword ptr [rbp + 0x7c8], 2
1409db30c: mov byte ptr [rbp + 0x937], 1
1409db313: mov rcx, qword ptr [rbp + 0x928]
1409db31a: mov qword ptr [rsp + 0x40], rcx
1409db31f: mov qword ptr [rsp + 0x38], rsi
1409db324: mov qword ptr [rsp + 0x30], rax
1409db329: lea rax, [rbp + 0x7c8]
1409db330: mov qword ptr [rsp + 0x28], rax
1409db335: mov qword ptr [rsp + 0x20], 1
1409db33e: lea rdx, [rip + 0x20e7883]
1409db345: lea rcx, [rbp - 0x30]
1409db349: lea r9, [rbp + 0x3b8]
1409db350: mov r8d, 0x16
1409db356: call 0x140867970 ; country_core::render::pass_builder::draw_stream::RasterPass::new_impl
1409db35b: mov rax, qword ptr [rbp + 0x8e0]
1409db362: mov qword ptr [rbp + 0x910], rax
1409db369: mov rcx, qword ptr [rbp + 0x8e8]
1409db370: mov rax, qword ptr [rbp + 0x8f0]
1409db377: shl rax, 4
1409db37b: add rax, rcx
1409db37e: mov qword ptr [rbp + 0x920], rax
1409db385: lea rdi, [rbp + 0x428]
1409db38c: lea rsi, [rbp + 0x880]
1409db393: movabs r14, 0x100000003
1409db39d: lea r15, [rbp + 0x7c8]
1409db3a4: lea r13, [rbp - 0x30]
1409db3a8: mov qword ptr [rbp + 0x918], rcx
1409db3af: mov rax, rcx
1409db3b2: cmp rax, qword ptr [rbp + 0x920]
1409db3b9: mov qword ptr [rbp + 0x928], rax
1409db3c0: je 0x1409db57b
1409db3c6: nop word ptr cs:[rax + rax]
1409db3d0: movaps xmm6, xmmword ptr [rax]
1409db3d3: call 0x140be6750 ; rhapsody::raw::shader::impl$7::default
1409db3d8: mov ebx, eax
1409db3da: xor ecx, ecx
1409db3dc: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
1409db3e1: test rax, rax
1409db3e4: je 0x1409db67d
1409db3ea: mov dword ptr [rbp + 0x460], ebx
1409db3f0: mov word ptr [rbp + 0x468], 0
1409db3f9: mov dword ptr [rbp + 0x464], 0x3f800000
1409db403: mov qword ptr [rbp + 0x3b8], 0
1409db40e: mov rcx, qword ptr [rbp + 0x8f8]
1409db415: mov qword ptr [rbp + 0x458], rcx
1409db41c: mov qword ptr [rbp + 0x3c8], 0
1409db427: mov qword ptr [rbp + 0x3d0], 4
1409db432: mov qword ptr [rbp + 0x3d8], 0
1409db43d: movups xmm0, xmmword ptr [rax]
1409db440: inc qword ptr [rax]
1409db443: movups xmm1, xmmword ptr [rip + 0x20bb2e6]
1409db44a: movups xmmword ptr [rdi + 0x10], xmm1
1409db44e: movups xmm1, xmmword ptr [rip + 0x20bb2cb]
1409db455: movups xmmword ptr [rdi], xmm1
1409db458: movups xmmword ptr [rbp + 0x448], xmm0
1409db45f: mov qword ptr [rbp + 0x3f8], r14
1409db466: mov qword ptr [rbp + 0x3e0], 0
1409db471: mov qword ptr [rbp + 0x3e8], 1
1409db47c: mov qword ptr [rbp + 0x3f0], 0
1409db487: mov rcx, r15
1409db48a: mov rdx, r12
1409db48d: mov r8, r13
1409db490: xor r9d, r9d
1409db493: call 0x140905be0 ; country_core::render::pass_builder::draw_builder::DrawBuilder::new
1409db498: cmp byte ptr [rbp + 0x8a6], 8
1409db49f: je 0x1409db4d0
1409db4a1: mov rdx, qword ptr [rbp + 0x8b0]
1409db4a8: mov eax, 0xe8
1409db4ad: add rdx, rax
1409db4b0: movaps xmmword ptr [rbp + 0x8c0], xmm6
1409db4b7: mov rcx, rsi
1409db4ba: movabs r8, 0x337ad802c8b34f67
1409db4c4: lea r9, [rbp + 0x8c0]
1409db4cb: call 0x14096bb20 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<glam::f32::sse2::vec4::Vec4>,glam::f32::sse2::vec4::Vec4>
1409db4d0: mov rbx, qword ptr [rbp + 0x7c8]
1409db4d7: mov r8d, 0xa0
1409db4dd: lea rcx, [rbp + 0x3c0]
1409db4e4: lea rdx, [rbp + 0x7d0]
1409db4eb: call 0x142923c00 ; ?
1409db4f0: movzx eax, byte ptr [rbp + 0x870]
1409db4f7: movzx ecx, word ptr [rbp + 0x871]
1409db4fe: movzx edx, byte ptr [rbp + 0x873]
1409db505: movups xmm0, xmmword ptr [rsi - 0xc]
1409db509: movups xmm1, xmmword ptr [rsi + 4]
1409db50d: movups xmm2, xmmword ptr [rsi + 0x14]
1409db511: movups xmm3, xmmword ptr [rsi + 0x24]
1409db515: movups xmmword ptr [rdi + 0x3c], xmm0
1409db519: movups xmmword ptr [rdi + 0x4c], xmm1
1409db51d: movups xmmword ptr [rdi + 0x5c], xmm2
1409db521: movups xmmword ptr [rdi + 0x6c], xmm3
1409db525: movups xmm0, xmmword ptr [rsi + 0x30]
1409db529: movups xmmword ptr [rdi + 0x78], xmm0
1409db52d: and al, 0x7f
1409db52f: and dl, 0x9f
1409db532: or dl, 0x20
1409db535: mov qword ptr [rbp + 0x3b8], rbx
1409db53c: mov byte ptr [rbp + 0x460], al
1409db542: mov word ptr [rbp + 0x461], cx
1409db549: mov byte ptr [rbp + 0x463], dl
1409db54f: mov edx, 6
1409db554: mov rcx, r12
1409db557: call 0x140906a90 ; country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
1409db55c: mov rax, qword ptr [rbp + 0x928]
1409db563: add rax, 0x10
1409db567: cmp rax, qword ptr [rbp + 0x920]
1409db56e: mov qword ptr [rbp + 0x928], rax
1409db575: jne 0x1409db3d0
1409db57b: mov rdx, qword ptr [rbp + 0x910]
1409db582: test rdx, rdx
1409db585: je 0x1409db59d
1409db587: shl rdx, 4
1409db58b: mov r8d, 0x10
1409db591: mov rcx, qword ptr [rbp + 0x918]
1409db598: call 0x140613c20 ; ?
1409db59d: lea rsi, [rbp + 0x3b8]
1409db5a4: lea rdx, [rbp - 0x30]
1409db5a8: mov r8d, 0x3e8
1409db5ae: mov rcx, rsi
1409db5b1: call 0x142923c00 ; ?
1409db5b6: mov rcx, rsi
1409db5b9: call 0x140868370 ; country_core::render::pass_builder::draw_stream::RasterPass::render
1409db5be: cmp qword ptr [rbp + 0x900], 0
1409db5c6: je 0x1409db60b
1409db5c8: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1409db5cd: mov rcx, rax
1409db5d0: mov rax, qword ptr [rax]
1409db5d3: cmp rax, 1
1409db5d7: jne 0x1409db626
1409db5d9: add rcx, 8
1409db5dd: cmp qword ptr [rcx], 0
1409db5e1: jne 0x1409db660
1409db5e3: mov qword ptr [rcx], 0xffffffffffffffff
1409db5ea: mov qword ptr [rbp + 0x928], rcx
1409db5f1: add rcx, 8
1409db5f5: mov rdx, qword ptr [rbp + 0x908]
1409db5fc: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1409db601: mov rax, qword ptr [rbp + 0x928]
1409db608: inc qword ptr [rax]
1409db60b: movaps xmm6, xmmword ptr [rbp + 0x940]
1409db612: add rsp, 0x9d8
1409db619: pop rbx
1409db61a: pop rdi
1409db61b: pop rsi
1409db61c: pop r12
1409db61e: pop r13
1409db620: pop r14
1409db622: pop r15
1409db624: pop rbp
1409db625: ret 
1409db626: test rax, rax
1409db629: jne 0x1409db654
1409db62b: xor edx, edx
1409db62d: call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1409db632: mov rcx, rax
1409db635: test rax, rax
1409db638: jne 0x1409db5dd
1409db63a: jmp 0x1409db654
1409db63c: test rax, rax
1409db63f: jne 0x1409db654
1409db641: xor edx, edx
1409db643: call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1409db648: mov rcx, rax
1409db64b: test rax, rax
1409db64e: jne 0x1409dacc6
1409db654: lea rcx, [rip + 0x225f445]
1409db65b: call 0x1428d80d0 ; std::thread::local::panic_access_error
1409db660: lea rcx, [rip + 0x225f509]
1409db667: call 0x1428d9250 ; core::cell::panic_already_borrowed
1409db66c: lea r8, [rip + 0x22a6275]
1409db673: call 0x1428cf200 ; bevy_ecs::query::state::impl$2::validate_world::panic_mismatched
1409db678: jmp 0x1409db707
1409db67d: lea rcx, [rip + 0x225f41c]
1409db684: call 0x1428d80d0 ; std::thread::local::panic_access_error
1409db689: jmp 0x1409db707
1409db68b: lea rcx, [rip + 0x20e748e]
1409db692: call 0x1428d9310 ; core::option::unwrap_failed
1409db697: jmp 0x1409db707
1409db699: lea rcx, [rip + 0x20e74c8]
1409db6a0: call 0x1428d9310 ; core::option::unwrap_failed
1409db6a5: jmp 0x1409db707
1409db6a7: lea rcx, [rip + 0x20e74d2]
1409db6ae: call 0x1428d9310 ; core::option::unwrap_failed
1409db6b3: jmp 0x1409db707
1409db6b5: lea rcx, [rip + 0x20e74dc]
1409db6bc: call 0x1428d9310 ; core::option::unwrap_failed
1409db6c1: jmp 0x1409db707
1409db6c3: lea rcx, [rip + 0x20e7516]
1409db6ca: call 0x1428d9310 ; core::option::unwrap_failed
1409db6cf: jmp 0x1409db707
1409db6d1: lea rcx, [rip + 0x20e7478]
1409db6d8: call 0x1428d9310 ; core::option::unwrap_failed
1409db6dd: jmp 0x1409db707
1409db6df: lea rcx, [rip + 0x20e7512]
1409db6e6: call 0x1428d9310 ; core::option::unwrap_failed
1409db6eb: jmp 0x1409db707
1409db6ed: lea rcx, [rip + 0x20e751c]
1409db6f4: call 0x1428d9310 ; core::option::unwrap_failed
1409db6f9: jmp 0x1409db707
1409db6fb: lea rcx, [rip + 0x20e74ae]
1409db702: call 0x1428d9310 ; core::option::unwrap_failed
1409db707: ud2 
1409db709: nop dword ptr [rax]
1409db710: mov qword ptr [rsp + 0x10], rdx
1409db715: push rbp
1409db716: push r15
1409db718: push r14
1409db71a: push r13
1409db71c: push r12
1409db71e: push rsi
1409db71f: push rdi
1409db720: push rbx
1409db721: sub rsp, 0x58
1409db725: lea rbp, [rdx + 0x80]
1409db72c: movaps xmmword ptr [rsp + 0x40], xmm6
1409db731: mov rax, qword ptr [rbp + 0x920]
1409db738: inc qword ptr [rax]
1409db73b: movaps xmm6, xmmword ptr [rsp + 0x40]
1409db740: add rsp, 0x58
1409db744: pop rbx
1409db745: pop rdi
1409db746: pop rsi
1409db747: pop r12
1409db749: pop r13
1409db74b: pop r14
1409db74d: pop r15
1409db74f: pop rbp
1409db750: ret 
1409db751: nop word ptr cs:[rax + rax]
1409db760: mov qword ptr [rsp + 0x10], rdx
1409db765: push rbp
1409db766: push r15
1409db768: push r14
1409db76a: push r13
1409db76c: push r12
1409db76e: push rsi
1409db76f: push rdi
1409db770: push rbx
1409db771: sub rsp, 0x58
1409db775: lea rbp, [rdx + 0x80]
1409db77c: movaps xmmword ptr [rsp + 0x40], xmm6
1409db781: cmp qword ptr [rbp + 0x900], 0
1409db789: je 0x1409db7ba
1409db78b: lea rax, [rbp + 0x908]
1409db792: mov qword ptr [rbp + 0x8d8], rax
1409db799: lea rax, [rbp + 0x8d8]
1409db7a0: mov qword ptr [rbp + 0x8d0], rax
1409db7a7: lea rcx, [rip + 0x20e67d2]
1409db7ae: lea rdx, [rbp + 0x8d0]
1409db7b5: call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
1409db7ba: movaps xmm6, xmmword ptr [rsp + 0x40]
1409db7bf: add rsp, 0x58
1409db7c3: pop rbx
1409db7c4: pop rdi
1409db7c5: pop rsi
1409db7c6: pop r12
1409db7c8: pop r13
1409db7ca: pop r14
1409db7cc: pop r15
1409db7ce: pop rbp
1409db7cf: ret 
1409db7d0: mov qword ptr [rsp + 0x10], rdx
1409db7d5: push rbp
1409db7d6: push r15
1409db7d8: push r14
1409db7da: push r13
1409db7dc: push r12
1409db7de: push rsi
1409db7df: push rdi
1409db7e0: push rbx
1409db7e1: sub rsp, 0x58
1409db7e5: lea rbp, [rdx + 0x80]
1409db7ec: movaps xmmword ptr [rsp + 0x40], xmm6
1409db7f1: lea rcx, [rbp + 0x3b8]
1409db7f8: call 0x1405d0be0 ; core::ptr::drop_in_place<bevy_ecs::query::state::QueryState<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> > >
1409db7fd: movaps xmm6, xmmword ptr [rsp + 0x40]
1409db802: add rsp, 0x58
1409db806: pop rbx
1409db807: pop rdi
1409db808: pop rsi
1409db809: pop r12
1409db80b: pop r13
1409db80d: pop r14
1409db80f: pop r15
1409db811: pop rbp
1409db812: ret 
1409db813: nop word ptr cs:[rax + rax]
1409db820: mov qword ptr [rsp + 0x10], rdx
1409db825: push rbp
1409db826: push r15
1409db828: push r14
1409db82a: push r13
1409db82c: push r12
1409db82e: push rsi
1409db82f: push rdi
1409db830: push rbx
1409db831: sub rsp, 0x58
1409db835: lea rbp, [rdx + 0x80]
1409db83c: movaps xmmword ptr [rsp + 0x40], xmm6
1409db841: mov rdx, qword ptr [rbp + 0x910]
1409db848: test rdx, rdx
1409db84b: je 0x1409db863
1409db84d: shl rdx, 4
1409db851: mov r8d, 0x10
1409db857: mov rcx, qword ptr [rbp + 0x918]
1409db85e: call 0x140613c20 ; ?
1409db863: movaps xmm6, xmmword ptr [rsp + 0x40]
1409db868: add rsp, 0x58
1409db86c: pop rbx
1409db86d: pop rdi
1409db86e: pop rsi
1409db86f: pop r12
1409db871: pop r13
1409db873: pop r14
1409db875: pop r15
1409db877: pop rbp
1409db878: ret 
1409db879: nop dword ptr [rax]
1409db880: mov qword ptr [rsp + 0x10], rdx
1409db885: push rbp
1409db886: push r15
1409db888: push r14
1409db88a: push r13
1409db88c: push r12
1409db88e: push rsi
1409db88f: push rdi
1409db890: push rbx
1409db891: sub rsp, 0x58
1409db895: lea rbp, [rdx + 0x80]
1409db89c: movaps xmmword ptr [rsp + 0x40], xmm6
1409db8a1: mov rax, qword ptr [rbp + 0x928]
1409db8a8: inc qword ptr [rax]
1409db8ab: movaps xmm6, xmmword ptr [rsp + 0x40]
1409db8b0: add rsp, 0x58
1409db8b4: pop rbx
1409db8b5: pop rdi
1409db8b6: pop rsi
1409db8b7: pop r12
1409db8b9: pop r13
1409db8bb: pop r14
1409db8bd: pop r15
1409db8bf: pop rbp
1409db8c0: ret 
1409db8c1: nop word ptr cs:[rax + rax]
1409db8d0: mov qword ptr [rsp + 0x10], rdx
1409db8d5: push rbp
1409db8d6: push r15
1409db8d8: push r14
1409db8da: push r13
1409db8dc: push r12
1409db8de: push rsi
1409db8df: push rdi
1409db8e0: push rbx
1409db8e1: sub rsp, 0x58
1409db8e5: lea rbp, [rdx + 0x80]
1409db8ec: movaps xmmword ptr [rsp + 0x40], xmm6
1409db8f1: mov rax, qword ptr [rbp + 0x928]
1409db8f8: cmp rax, qword ptr [rbp + 0x920]
1409db8ff: je 0x1409db927
1409db901: lea rcx, [rbp - 0x30]
1409db905: call 0x1408923c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>
1409db90a: mov byte ptr [rbp + 0x937], 0
1409db911: movaps xmm6, xmmword ptr [rsp + 0x40]
1409db916: add rsp, 0x58
1409db91a: pop rbx
1409db91b: pop rdi
1409db91c: pop rsi
1409db91d: pop r12
1409db91f: pop r13
1409db921: pop r14
1409db923: pop r15
1409db925: pop rbp
1409db926: ret 
1409db927: mov byte ptr [rbp + 0x937], 0
1409db92e: movaps xmm6, xmmword ptr [rsp + 0x40]
1409db933: add rsp, 0x58
1409db937: pop rbx
1409db938: pop rdi
1409db939: pop rsi
1409db93a: pop r12
1409db93c: pop r13
1409db93e: pop r14
1409db940: pop r15
1409db942: pop rbp
1409db943: ret 
1409db944: nop word ptr cs:[rax + rax]
1409db950: mov qword ptr [rsp + 0x10], rdx
1409db955: push rbp
1409db956: push r15
1409db958: push r14
1409db95a: push r13
1409db95c: push r12
1409db95e: push rsi
1409db95f: push rdi
1409db960: push rbx
1409db961: sub rsp, 0x58
1409db965: lea rbp, [rdx + 0x80]
1409db96c: movaps xmmword ptr [rsp + 0x40], xmm6
1409db971: cmp byte ptr [rbp + 0x937], 0
1409db978: je 0x1409db9b2
1409db97a: mov rdx, qword ptr [rbp + 0x8e0]
1409db981: test rdx, rdx
1409db984: je 0x1409db99c
1409db986: mov rcx, qword ptr [rbp + 0x8e8]
1409db98d: shl rdx, 4
1409db991: mov r8d, 0x10
1409db997: call 0x140613c20 ; ?
1409db99c: movaps xmm6, xmmword ptr [rsp + 0x40]
1409db9a1: add rsp, 0x58
1409db9a5: pop rbx
1409db9a6: pop rdi
1409db9a7: pop rsi
1409db9a8: pop r12
1409db9aa: pop r13
1409db9ac: pop r14
1409db9ae: pop r15
1409db9b0: pop rbp
1409db9b1: ret 
1409db9b2: movaps xmm6, xmmword ptr [rsp + 0x40]
1409db9b7: add rsp, 0x58
1409db9bb: pop rbx
1409db9bc: pop rdi
1409db9bd: pop rsi
1409db9be: pop r12
1409db9c0: pop r13
1409db9c2: pop r14
1409db9c4: pop r15
1409db9c6: pop rbp
1409db9c7: ret 
1409db9c8: nop dword ptr [rax + rax]
1409db9d0: mov qword ptr [rsp + 0x10], rdx
1409db9d5: push rbp
1409db9d6: push r15
1409db9d8: push r14
1409db9da: push r13
1409db9dc: push r12
1409db9de: push rsi
1409db9df: push rdi
1409db9e0: push rbx
1409db9e1: sub rsp, 0x58
1409db9e5: lea rbp, [rdx + 0x80]
1409db9ec: movaps xmmword ptr [rsp + 0x40], xmm6
1409db9f1: lea rcx, [rbp + 0x7c8]
1409db9f8: call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409db9fd: movaps xmm6, xmmword ptr [rsp + 0x40]
1409dba02: add rsp, 0x58
1409dba06: pop rbx
1409dba07: pop rdi
1409dba08: pop rsi
1409dba09: pop r12
1409dba0b: pop r13
1409dba0d: pop r14
1409dba0f: pop r15
1409dba11: pop rbp
1409dba12: ret 
