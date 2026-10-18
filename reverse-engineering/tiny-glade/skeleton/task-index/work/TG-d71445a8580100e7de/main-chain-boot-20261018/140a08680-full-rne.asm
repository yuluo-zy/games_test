140a08680: push rbp
140a08681: push r15
140a08683: push r14
140a08685: push r13
140a08687: push r12
140a08689: push rsi
140a0868a: push rdi
140a0868b: push rbx
140a0868c: sub rsp, 0x118
140a08693: lea rbp, [rsp + 0x80]
140a0869b: movaps xmmword ptr [rbp + 0x80], xmm6
140a086a2: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
140a086aa: mov qword ptr [rbp - 0x60], r8
140a086ae: mov qword ptr [rbp - 0x58], r9
140a086b2: cmp qword ptr [rdx + 0x50], 0
140a086b7: je 0x140a08838
140a086bd: mov rdi, rdx
140a086c0: mov rsi, rcx
140a086c3: lea rbx, [rdx + 0x38]
140a086c7: lea rcx, [rdx + 0x58]
140a086cb: lea rdx, [rip + 0x20bdebe]
140a086d2: mov r8d, 0xc
140a086d8: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
140a086dd: mov rcx, qword ptr [rdi + 0x38]
140a086e1: mov rdx, qword ptr [rdi + 0x40]
140a086e5: mov r8, rdx
140a086e8: and r8, rax
140a086eb: shr rax, 0x39
140a086ef: movd xmm0, eax
140a086f3: punpcklbw xmm0, xmm0
140a086f7: pshuflw xmm0, xmm0, 0
140a086fc: pshufd xmm0, xmm0, 0
140a08701: lea rax, [rcx - 0x38]
140a08705: xor r9d, r9d
140a08708: pcmpeqd xmm1, xmm1
140a0870c: movabs r10, 0x435f786574726556
140a08716: movdqu xmm2, xmmword ptr [rcx + r8]
140a0871c: movdqa xmm3, xmm2
140a08720: pcmpeqb xmm3, xmm0
140a08724: pmovmskb r11d, xmm3
140a08729: test r11d, r11d
140a0872c: je 0x140a08780
140a0872e: tzcnt r14d, r11d
140a08733: add r14, r8
140a08736: and r14, rdx
140a08739: neg r14
140a0873c: imul r14, r14, 0x38
140a08740: cmp qword ptr [rax + r14 + 0x10], 0xc
140a08746: jne 0x140a08766
140a08748: lea r15, [rax + r14]
140a0874c: mov r15, qword ptr [r15 + 8]
140a08750: mov r12, qword ptr [r15]
140a08753: xor r12, r10
140a08756: mov r15d, dword ptr [r15 + 8]
140a0875a: xor r15, 0x726f6c6f
140a08761: or r15, r12
140a08764: je 0x140a087a5
140a08766: lea r14d, [r11 - 1]
140a0876a: and r14w, r11w
140a0876e: mov r11d, r14d
140a08771: jne 0x140a0872e
140a08773: nop word ptr cs:[rax + rax]
140a08780: pcmpeqb xmm2, xmm1
140a08784: pmovmskb r11d, xmm2
140a08789: test r11d, r11d
140a0878c: jne 0x140a08838
140a08792: add r8, r9
140a08795: add r8, 0x10
140a08799: add r9, 0x10
140a0879d: and r8, rdx
140a087a0: jmp 0x140a08716 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
140a087a5: cmp dword ptr [rcx + r14 - 0x20], 3
140a087ab: jne 0x140a08b5d
140a087b1: add rcx, r14
140a087b4: mov rax, qword ptr [rcx - 0x10]
140a087b8: mov qword ptr [rbp + 0x60], rax
140a087bc: mov r13, qword ptr [rcx - 8]
140a087c0: movups xmm6, xmmword ptr [rdi + 0x58]
140a087c4: lea rcx, [rbp - 0x50]
140a087c8: mov rdx, rbx
140a087cb: call 0x140b72ed0 ; hashbrown::raw::impl$13::clone<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global>
140a087d0: movups xmmword ptr [rbp - 0x30], xmm6
140a087d4: mov r12, qword ptr [rdi + 0x10]
140a087d8: lea rbx, [r12*4]
140a087e0: xor r15d, r15d
140a087e3: mov rax, r12
140a087e6: shr rax, 0x3e
140a087ea: jne 0x140a08b91
140a087f0: movabs rax, 0x7ffffffffffffffc
140a087fa: cmp rbx, rax
140a087fd: ja 0x140a08b91
140a08803: mov r14, qword ptr [rdi + 8]
140a08807: test rbx, rbx
140a0880a: je 0x140a08895
140a08810: movzx eax, byte ptr [rip + 0x2c2c3f2]
140a08817: mov r15d, 4
140a0881d: mov edx, 4
140a08822: mov rcx, rbx
140a08825: call 0x140613c10
140a0882a: test rax, rax
140a0882d: je 0x140a08b91
140a08833: mov r15, r12
140a08836: jmp 0x140a0889d ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
140a08838: lea rax, [rip + 0x20be179]
140a0883f: mov qword ptr [rbp - 0x50], rax
140a08843: lea rax, [rip - 0x4d4faa]
140a0884a: mov qword ptr [rbp - 0x48], rax
140a0884e: lea rcx, [rbp - 0x60]
140a08852: mov qword ptr [rbp - 0x40], rcx
140a08856: mov qword ptr [rbp - 0x38], rax
140a0885a: lea rax, [rip + 0x20be17f]
140a08861: mov qword ptr [rbp - 0x20], rax
140a08865: mov qword ptr [rbp - 0x18], 2
140a0886d: mov qword ptr [rbp], 0
140a08875: lea rax, [rbp - 0x50]
140a08879: mov qword ptr [rbp - 0x10], rax
140a0887d: mov qword ptr [rbp - 8], 2
140a08885: lea rdx, [rip + 0x20be174]
140a0888c: lea rcx, [rbp - 0x20]
140a08890: call 0x1428d9390 ; core::panicking::panic_fmt
140a08895: mov eax, 4
140a0889a: xor r15d, r15d
140a0889d: mov qword ptr [rbp + 0x68], rax
140a088a1: mov rcx, rax
140a088a4: mov rdx, r14
140a088a7: mov r8, rbx
140a088aa: call 0x142923c00
140a088af: xor eax, eax
140a088b1: cmp rax, qword ptr [rdi + 0x18]
140a088b5: jno 0x140a088c3
140a088b7: movabs rbx, 0x8000000000000000
140a088c1: jmp 0x140a08921 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
140a088c3: mov qword ptr [rbp + 0x70], r15
140a088c7: mov rbx, qword ptr [rdi + 0x28]
140a088cb: test rbx, rbx
140a088ce: js 0x140a08bbb
140a088d4: mov r15, qword ptr [rdi + 0x20]
140a088d8: je 0x140a08905
140a088da: movzx eax, byte ptr [rip + 0x2c2c328]
140a088e1: mov eax, 1
140a088e6: mov qword ptr [rbp + 0x58], rax
140a088ea: mov edx, 1
140a088ef: mov rcx, rbx
140a088f2: call 0x140613c10
140a088f7: test rax, rax
140a088fa: je 0x140a08bc3
140a08900: mov r14, rax
140a08903: jmp 0x140a0890b ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
140a08905: mov r14d, 1
140a0890b: mov rcx, r14
140a0890e: mov rdx, r15
140a08911: mov r8, rbx
140a08914: call 0x142923c00
140a08919: movzx eax, byte ptr [rdi + 0x30]
140a0891d: mov r15, qword ptr [rbp + 0x70]
140a08921: movups xmm0, xmmword ptr [rdi + 0x68]
140a08925: movups xmm1, xmmword ptr [rbp - 0x50]
140a08929: movups xmm2, xmmword ptr [rbp - 0x40]
140a0892d: movups xmm3, xmmword ptr [rbp - 0x30]
140a08931: movups xmmword ptr [rbp + 0x18], xmm1
140a08935: movups xmmword ptr [rbp + 0x28], xmm2
140a08939: movups xmmword ptr [rbp + 0x38], xmm3
140a0893d: mov qword ptr [rbp - 0x20], r15
140a08941: mov rcx, qword ptr [rbp + 0x68]
140a08945: mov qword ptr [rbp - 0x18], rcx
140a08949: mov qword ptr [rbp - 0x10], r12
140a0894d: mov qword ptr [rbp - 8], rbx
140a08951: mov qword ptr [rbp], r14
140a08955: mov qword ptr [rbp + 8], rbx
140a08959: mov byte ptr [rbp + 0x10], al
140a0895c: movups xmmword ptr [rbp + 0x48], xmm0
140a08960: lea rbx, [r13*4]
140a08968: test r13, r13
140a0896b: mov qword ptr [rbp + 0x70], rbx
140a0896f: je 0x140a08a3d
140a08975: movzx eax, byte ptr [rip + 0x2c2c28d]
140a0897c: mov edx, 4
140a08981: mov rcx, rbx
140a08984: call 0x140613c10
140a08989: test rax, rax
140a0898c: je 0x140a08ba5
140a08992: mov rdx, rax
140a08995: cmp r13, 0xb
140a08999: mov r9, qword ptr [rbp + 0x60]
140a0899d: jbe 0x140a089cc
140a0899f: lea rax, [rdx + r13*4]
140a089a3: lea rcx, [r9 + 8]
140a089a7: lea r8, [r13*2]
140a089af: add r8, r13
140a089b2: lea r8, [r9 + r8*4]
140a089b6: cmp rdx, r8
140a089b9: setb r8b
140a089bd: cmp rcx, rax
140a089c0: setb al
140a089c3: test r8b, al
140a089c6: je 0x140a08acd
140a089cc: xor eax, eax
140a089ce: mov rcx, rax
140a089d1: or rcx, 1
140a089d5: test r13b, 1
140a089d9: je 0x140a089f6
140a089db: lea r10, [rax + rax*2]
140a089df: xorps xmm0, xmm0
140a089e2: xor r8d, r8d
140a089e5: ucomiss xmm0, dword ptr [r9 + r10*4 + 8]
140a089eb: setae r8b
140a089ef: mov dword ptr [rdx + rax*4], r8d
140a089f3: mov rax, rcx
140a089f6: cmp r13, rcx
140a089f9: je 0x140a08a42
140a089fb: lea rcx, [rax + rax*2]
140a089ff: lea rcx, [r9 + rcx*4]
140a08a03: add rcx, 0x14
140a08a07: xorps xmm0, xmm0
140a08a0a: nop word ptr [rax + rax]
140a08a10: xor r8d, r8d
140a08a13: ucomiss xmm0, dword ptr [rcx - 0xc]
140a08a17: setae r8b
140a08a1b: mov dword ptr [rdx + rax*4], r8d
140a08a1f: xor r8d, r8d
140a08a22: ucomiss xmm0, dword ptr [rcx]
140a08a25: setae r8b
140a08a29: mov dword ptr [rdx + rax*4 + 4], r8d
140a08a2e: add rax, 2
140a08a32: add rcx, 0x18
140a08a36: cmp r13, rax
140a08a39: jne 0x140a08a10
140a08a3b: jmp 0x140a08a42 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
140a08a3d: mov edx, 4
140a08a42: lea rcx, [rbp - 0x20]
140a08a46: mov qword ptr [rbp + 0x60], rdx
140a08a4a: mov r8, r13
140a08a4d: mov qword ptr [rbp + 0x68], r13
140a08a51: call 0x140a08090 ; country_core::render::mesh::Mesh::retain_vertices
140a08a56: mov rax, qword ptr [rbp + 0x50]
140a08a5a: mov qword ptr [rsi + 0x70], rax
140a08a5e: movups xmm0, xmmword ptr [rbp + 0x40]
140a08a62: movups xmmword ptr [rsi + 0x60], xmm0
140a08a66: movups xmm0, xmmword ptr [rbp + 0x30]
140a08a6a: movups xmmword ptr [rsi + 0x50], xmm0
140a08a6e: movups xmm0, xmmword ptr [rbp + 0x20]
140a08a72: movups xmmword ptr [rsi + 0x40], xmm0
140a08a76: movups xmm0, xmmword ptr [rbp - 0x20]
140a08a7a: movups xmm1, xmmword ptr [rbp - 0x10]
140a08a7e: movups xmm2, xmmword ptr [rbp]
140a08a82: movups xmm3, xmmword ptr [rbp + 0x10]
140a08a86: movups xmmword ptr [rsi + 0x30], xmm3
140a08a8a: movups xmmword ptr [rsi + 0x20], xmm2
140a08a8e: movups xmmword ptr [rsi + 0x10], xmm1
140a08a92: movups xmmword ptr [rsi], xmm0
140a08a95: cmp qword ptr [rbp + 0x68], 0
140a08a9a: je 0x140a08aaf
140a08a9c: mov r8d, 4
140a08aa2: mov rcx, qword ptr [rbp + 0x60]
140a08aa6: mov rdx, qword ptr [rbp + 0x70]
140a08aaa: call 0x140613c20
140a08aaf: mov rax, rsi
140a08ab2: movaps xmm6, xmmword ptr [rbp + 0x80]
140a08ab9: add rsp, 0x118
140a08ac0: pop rbx
140a08ac1: pop rdi
140a08ac2: pop rsi
140a08ac3: pop r12
140a08ac5: pop r13
140a08ac7: pop r14
140a08ac9: pop r15
140a08acb: pop rbp
140a08acc: ret 
140a08acd: mov rax, r13
140a08ad0: and rax, 0xfffffffffffffff8
140a08ad4: lea rcx, [r9 + 0x5c]
140a08ad8: xor r8d, r8d
140a08adb: xorps xmm0, xmm0
140a08ade: movaps xmm1, xmmword ptr [rip + 0x1f504fb]
140a08ae5: nop word ptr cs:[rax + rax]
140a08af0: movss xmm2, dword ptr [rcx - 0x54]
140a08af5: movss xmm3, dword ptr [rcx - 0x48]
140a08afa: unpcklps xmm2, xmm3
140a08afd: movss xmm3, dword ptr [rcx - 0x3c]
140a08b02: movss xmm4, dword ptr [rcx - 0x30]
140a08b07: unpcklps xmm3, xmm4
140a08b0a: movlhps xmm2, xmm3
140a08b0d: movss xmm3, dword ptr [rcx]
140a08b11: movss xmm4, dword ptr [rcx - 0xc]
140a08b16: unpcklps xmm4, xmm3
140a08b19: movss xmm3, dword ptr [rcx - 0x18]
140a08b1e: movss xmm5, dword ptr [rcx - 0x24]
140a08b23: unpcklps xmm5, xmm3
140a08b26: movlhps xmm5, xmm4
140a08b29: cmpleps xmm2, xmm0
140a08b2d: andps xmm2, xmm1
140a08b30: cmpleps xmm5, xmm0
140a08b34: andps xmm5, xmm1
140a08b37: movups xmmword ptr [rdx + r8*4], xmm2
140a08b3c: movups xmmword ptr [rdx + r8*4 + 0x10], xmm5
140a08b42: add r8, 8
140a08b46: add rcx, 0x60
140a08b4a: cmp rax, r8
140a08b4d: jne 0x140a08af0
140a08b4f: cmp r13, rax
140a08b52: jne 0x140a089ce
140a08b58: jmp 0x140a08a42 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
140a08b5d: lea rax, [rip + 0x20be4b4]
140a08b64: mov qword ptr [rbp - 0x20], rax
140a08b68: mov qword ptr [rbp - 0x18], 1
140a08b70: mov qword ptr [rbp - 0x10], 8
140a08b78: pxor xmm0, xmm0
140a08b7c: movdqu xmmword ptr [rbp - 8], xmm0
140a08b81: lea rdx, [rip + 0x20be4b8]
140a08b88: lea rcx, [rbp - 0x20]
140a08b8c: call 0x1428d9390 ; core::panicking::panic_fmt
140a08b91: lea r8, [rip + 0x222ffe0]
140a08b98: mov rcx, r15
140a08b9b: mov rdx, rbx
140a08b9e: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a08ba3: jmp 0x140a08bd6 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
140a08ba5: lea r8, [rip + 0x20bd77c]
140a08bac: mov ecx, 4
140a08bb1: mov rdx, rbx
140a08bb4: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a08bb9: jmp 0x140a08bd6 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
140a08bbb: mov qword ptr [rbp + 0x58], 0
140a08bc3: lea r8, [rip + 0x222ffae]
140a08bca: mov rcx, qword ptr [rbp + 0x58]
140a08bce: mov rdx, rbx
140a08bd1: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a08bd6: ud2 
140a08bd8: nop dword ptr [rax + rax]
140a08be0: mov qword ptr [rsp + 0x10], rdx
140a08be5: push rbp
140a08be6: push r15
140a08be8: push r14
140a08bea: push r13
140a08bec: push r12
140a08bee: push rsi
140a08bef: push rdi
140a08bf0: push rbx
140a08bf1: sub rsp, 0x38
140a08bf5: lea rbp, [rdx + 0x80]
140a08bfc: movaps xmmword ptr [rsp + 0x20], xmm6
140a08c01: lea rcx, [rbp - 0x50]
140a08c05: mov r8d, 0x38
140a08c0b: mov r9d, 0x10
140a08c11: lea rdx, [rbp - 0x30]
140a08c15: call 0x140b75360 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global>
140a08c1a: movaps xmm6, xmmword ptr [rsp + 0x20]
140a08c1f: add rsp, 0x38
140a08c23: pop rbx
140a08c24: pop rdi
140a08c25: pop rsi
140a08c26: pop r12
140a08c28: pop r13
140a08c2a: pop r14
140a08c2c: pop r15
140a08c2e: pop rbp
140a08c2f: ret 
140a08c30: mov qword ptr [rsp + 0x10], rdx
140a08c35: push rbp
140a08c36: push r15
140a08c38: push r14
140a08c3a: push r13
140a08c3c: push r12
140a08c3e: push rsi
140a08c3f: push rdi
140a08c40: push rbx
140a08c41: sub rsp, 0x38
140a08c45: lea rbp, [rdx + 0x80]
140a08c4c: movaps xmmword ptr [rsp + 0x20], xmm6
140a08c51: cmp qword ptr [rbp + 0x70], 0
140a08c56: je 0x140a08c6f
140a08c58: mov rdx, qword ptr [rbp + 0x70]
140a08c5c: shl rdx, 2
140a08c60: mov r8d, 4
140a08c66: mov rcx, qword ptr [rbp + 0x68]
140a08c6a: call 0x140613c20
140a08c6f: movaps xmm6, xmmword ptr [rsp + 0x20]
140a08c74: add rsp, 0x38
140a08c78: pop rbx
140a08c79: pop rdi
140a08c7a: pop rsi
140a08c7b: pop r12
140a08c7d: pop r13
140a08c7f: pop r14
140a08c81: pop r15
140a08c83: pop rbp
140a08c84: ret 
140a08c85: nop word ptr cs:[rax + rax]
140a08c90: mov qword ptr [rsp + 0x10], rdx
140a08c95: push rbp
140a08c96: push r15
140a08c98: push r14
140a08c9a: push r13
140a08c9c: push r12
140a08c9e: push rsi
140a08c9f: push rdi
140a08ca0: push rbx
140a08ca1: sub rsp, 0x38
140a08ca5: lea rbp, [rdx + 0x80]
140a08cac: movaps xmmword ptr [rsp + 0x20], xmm6
140a08cb1: cmp qword ptr [rbp + 0x68], 0
140a08cb6: je 0x140a08ccb
140a08cb8: mov r8d, 4
140a08cbe: mov rcx, qword ptr [rbp + 0x60]
140a08cc2: mov rdx, qword ptr [rbp + 0x70]
140a08cc6: call 0x140613c20
140a08ccb: movaps xmm6, xmmword ptr [rsp + 0x20]
140a08cd0: add rsp, 0x38
140a08cd4: pop rbx
140a08cd5: pop rdi
140a08cd6: pop rsi
140a08cd7: pop r12
140a08cd9: pop r13
140a08cdb: pop r14
140a08cdd: pop r15
140a08cdf: pop rbp
140a08ce0: ret 
140a08ce1: nop word ptr cs:[rax + rax]
140a08cf0: mov qword ptr [rsp + 0x10], rdx
140a08cf5: push rbp
140a08cf6: push r15
140a08cf8: push r14
140a08cfa: push r13
140a08cfc: push r12
140a08cfe: push rsi
140a08cff: push rdi
140a08d00: push rbx
140a08d01: sub rsp, 0x38
140a08d05: lea rbp, [rdx + 0x80]
140a08d0c: movaps xmmword ptr [rsp + 0x20], xmm6
140a08d11: lea rcx, [rbp - 0x20]
140a08d15: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
140a08d1a: movaps xmm6, xmmword ptr [rsp + 0x20]
140a08d1f: add rsp, 0x38
140a08d23: pop rbx
140a08d24: pop rdi
140a08d25: pop rsi
140a08d26: pop r12
140a08d28: pop r13
140a08d2a: pop r14
140a08d2c: pop r15
140a08d2e: pop rbp
140a08d2f: ret 
