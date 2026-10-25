140a08d30: push rbp
140a08d31: push r15
140a08d33: push r14
140a08d35: push r13
140a08d37: push r12
140a08d39: push rsi
140a08d3a: push rdi
140a08d3b: push rbx
140a08d3c: sub rsp, 0x118
140a08d43: lea rbp, [rsp + 0x80]
140a08d4b: movaps xmmword ptr [rbp + 0x80], xmm6
140a08d52: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
140a08d5a: mov qword ptr [rbp - 0x60], r8
140a08d5e: mov qword ptr [rbp - 0x58], r9
140a08d62: cmp qword ptr [rdx + 0x50], 0
140a08d67: je 0x140a08ee8
140a08d6d: mov rdi, rdx
140a08d70: mov rsi, rcx
140a08d73: lea rbx, [rdx + 0x38]
140a08d77: lea rcx, [rdx + 0x58]
140a08d7b: lea rdx, [rip + 0x20bd80e]
140a08d82: mov r8d, 0xc
140a08d88: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
140a08d8d: mov rcx, qword ptr [rdi + 0x38]
140a08d91: mov rdx, qword ptr [rdi + 0x40]
140a08d95: mov r8, rdx
140a08d98: and r8, rax
140a08d9b: shr rax, 0x39
140a08d9f: movd xmm0, eax
140a08da3: punpcklbw xmm0, xmm0
140a08da7: pshuflw xmm0, xmm0, 0
140a08dac: pshufd xmm0, xmm0, 0
140a08db1: lea rax, [rcx - 0x38]
140a08db5: xor r9d, r9d
140a08db8: pcmpeqd xmm1, xmm1
140a08dbc: movabs r10, 0x435f786574726556
140a08dc6: movdqu xmm2, xmmword ptr [rcx + r8]
140a08dcc: movdqa xmm3, xmm2
140a08dd0: pcmpeqb xmm3, xmm0
140a08dd4: pmovmskb r11d, xmm3
140a08dd9: test r11d, r11d
140a08ddc: je 0x140a08e30
140a08dde: tzcnt r14d, r11d
140a08de3: add r14, r8
140a08de6: and r14, rdx
140a08de9: neg r14
140a08dec: imul r14, r14, 0x38
140a08df0: cmp qword ptr [rax + r14 + 0x10], 0xc
140a08df6: jne 0x140a08e16
140a08df8: lea r15, [rax + r14]
140a08dfc: mov r15, qword ptr [r15 + 8]
140a08e00: mov r12, qword ptr [r15]
140a08e03: xor r12, r10
140a08e06: mov r15d, dword ptr [r15 + 8]
140a08e0a: xor r15, 0x726f6c6f
140a08e11: or r15, r12
140a08e14: je 0x140a08e55
140a08e16: lea r14d, [r11 - 1]
140a08e1a: and r14w, r11w
140a08e1e: mov r11d, r14d
140a08e21: jne 0x140a08dde
140a08e23: nop word ptr cs:[rax + rax]
140a08e30: pcmpeqb xmm2, xmm1
140a08e34: pmovmskb r11d, xmm2
140a08e39: test r11d, r11d
140a08e3c: jne 0x140a08ee8
140a08e42: add r8, r9
140a08e45: add r8, 0x10
140a08e49: add r9, 0x10
140a08e4d: and r8, rdx
140a08e50: jmp 0x140a08dc6 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
140a08e55: cmp dword ptr [rcx + r14 - 0x20], 3
140a08e5b: jne 0x140a09213
140a08e61: add rcx, r14
140a08e64: mov rax, qword ptr [rcx - 0x10]
140a08e68: mov qword ptr [rbp + 0x60], rax
140a08e6c: mov r13, qword ptr [rcx - 8]
140a08e70: movups xmm6, xmmword ptr [rdi + 0x58]
140a08e74: lea rcx, [rbp - 0x50]
140a08e78: mov rdx, rbx
140a08e7b: call 0x140b72ed0 ; hashbrown::raw::impl$13::clone<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global>
140a08e80: movups xmmword ptr [rbp - 0x30], xmm6
140a08e84: mov r12, qword ptr [rdi + 0x10]
140a08e88: lea rbx, [r12*4]
140a08e90: xor r15d, r15d
140a08e93: mov rax, r12
140a08e96: shr rax, 0x3e
140a08e9a: jne 0x140a09247
140a08ea0: movabs rax, 0x7ffffffffffffffc
140a08eaa: cmp rbx, rax
140a08ead: ja 0x140a09247
140a08eb3: mov r14, qword ptr [rdi + 8]
140a08eb7: test rbx, rbx
140a08eba: je 0x140a08f45
140a08ec0: movzx eax, byte ptr [rip + 0x2c2bd42]
140a08ec7: mov r15d, 4
140a08ecd: mov edx, 4
140a08ed2: mov rcx, rbx
140a08ed5: call 0x140613c10
140a08eda: test rax, rax
140a08edd: je 0x140a09247
140a08ee3: mov r15, r12
140a08ee6: jmp 0x140a08f4d ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
140a08ee8: lea rax, [rip + 0x20bdac9]
140a08eef: mov qword ptr [rbp - 0x50], rax
140a08ef3: lea rax, [rip - 0x4d565a]
140a08efa: mov qword ptr [rbp - 0x48], rax
140a08efe: lea rcx, [rbp - 0x60]
140a08f02: mov qword ptr [rbp - 0x40], rcx
140a08f06: mov qword ptr [rbp - 0x38], rax
140a08f0a: lea rax, [rip + 0x20bdacf]
140a08f11: mov qword ptr [rbp - 0x20], rax
140a08f15: mov qword ptr [rbp - 0x18], 2
140a08f1d: mov qword ptr [rbp], 0
140a08f25: lea rax, [rbp - 0x50]
140a08f29: mov qword ptr [rbp - 0x10], rax
140a08f2d: mov qword ptr [rbp - 8], 2
140a08f35: lea rdx, [rip + 0x20bdac4]
140a08f3c: lea rcx, [rbp - 0x20]
140a08f40: call 0x1428d9390 ; core::panicking::panic_fmt
140a08f45: mov eax, 4
140a08f4a: xor r15d, r15d
140a08f4d: mov qword ptr [rbp + 0x68], rax
140a08f51: mov rcx, rax
140a08f54: mov rdx, r14
140a08f57: mov r8, rbx
140a08f5a: call 0x142923c00
140a08f5f: xor eax, eax
140a08f61: cmp rax, qword ptr [rdi + 0x18]
140a08f65: jno 0x140a08f73
140a08f67: movabs rbx, 0x8000000000000000
140a08f71: jmp 0x140a08fd1 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
140a08f73: mov qword ptr [rbp + 0x70], r15
140a08f77: mov rbx, qword ptr [rdi + 0x28]
140a08f7b: test rbx, rbx
140a08f7e: js 0x140a09271
140a08f84: mov r15, qword ptr [rdi + 0x20]
140a08f88: je 0x140a08fb5
140a08f8a: movzx eax, byte ptr [rip + 0x2c2bc78]
140a08f91: mov eax, 1
140a08f96: mov qword ptr [rbp + 0x58], rax
140a08f9a: mov edx, 1
140a08f9f: mov rcx, rbx
140a08fa2: call 0x140613c10
140a08fa7: test rax, rax
140a08faa: je 0x140a09279
140a08fb0: mov r14, rax
140a08fb3: jmp 0x140a08fbb ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
140a08fb5: mov r14d, 1
140a08fbb: mov rcx, r14
140a08fbe: mov rdx, r15
140a08fc1: mov r8, rbx
140a08fc4: call 0x142923c00
140a08fc9: movzx eax, byte ptr [rdi + 0x30]
140a08fcd: mov r15, qword ptr [rbp + 0x70]
140a08fd1: movups xmm0, xmmword ptr [rdi + 0x68]
140a08fd5: movups xmm1, xmmword ptr [rbp - 0x50]
140a08fd9: movups xmm2, xmmword ptr [rbp - 0x40]
140a08fdd: movups xmm3, xmmword ptr [rbp - 0x30]
140a08fe1: movups xmmword ptr [rbp + 0x18], xmm1
140a08fe5: movups xmmword ptr [rbp + 0x28], xmm2
140a08fe9: movups xmmword ptr [rbp + 0x38], xmm3
140a08fed: mov qword ptr [rbp - 0x20], r15
140a08ff1: mov rcx, qword ptr [rbp + 0x68]
140a08ff5: mov qword ptr [rbp - 0x18], rcx
140a08ff9: mov qword ptr [rbp - 0x10], r12
140a08ffd: mov qword ptr [rbp - 8], rbx
140a09001: mov qword ptr [rbp], r14
140a09005: mov qword ptr [rbp + 8], rbx
140a09009: mov byte ptr [rbp + 0x10], al
140a0900c: movups xmmword ptr [rbp + 0x48], xmm0
140a09010: lea rbx, [r13*4]
140a09018: test r13, r13
140a0901b: mov qword ptr [rbp + 0x70], rbx
140a0901f: je 0x140a090f5
140a09025: movzx eax, byte ptr [rip + 0x2c2bbdd]
140a0902c: mov edx, 4
140a09031: mov rcx, rbx
140a09034: call 0x140613c10
140a09039: test rax, rax
140a0903c: je 0x140a0925b
140a09042: mov rdx, rax
140a09045: cmp r13, 0xb
140a09049: mov r8, qword ptr [rbp + 0x60]
140a0904d: jbe 0x140a0907c
140a0904f: lea rax, [rdx + r13*4]
140a09053: lea rcx, [r8 + 8]
140a09057: lea r9, [r13*2]
140a0905f: add r9, r13
140a09062: lea r9, [r8 + r9*4]
140a09066: cmp rdx, r9
140a09069: setb r9b
140a0906d: cmp rcx, rax
140a09070: setb al
140a09073: test r9b, al
140a09076: je 0x140a09185
140a0907c: xor eax, eax
140a0907e: mov rcx, rax
140a09081: or rcx, 1
140a09085: test r13b, 1
140a09089: je 0x140a090aa
140a0908b: lea r9, [rax + rax*2]
140a0908f: movss xmm0, dword ptr [r8 + r9*4 + 8]
140a09096: xorps xmm1, xmm1
140a09099: xor r9d, r9d
140a0909c: ucomiss xmm0, xmm1
140a0909f: seta r9b
140a090a3: mov dword ptr [rdx + rax*4], r9d
140a090a7: mov rax, rcx
140a090aa: cmp r13, rcx
140a090ad: je 0x140a090fa
140a090af: lea rcx, [rax + rax*2]
140a090b3: lea rcx, [r8 + rcx*4]
140a090b7: add rcx, 0x14
140a090bb: xorps xmm0, xmm0
140a090be: nop 
140a090c0: movss xmm1, dword ptr [rcx - 0xc]
140a090c5: xor r8d, r8d
140a090c8: ucomiss xmm1, xmm0
140a090cb: seta r8b
140a090cf: mov dword ptr [rdx + rax*4], r8d
140a090d3: movss xmm1, dword ptr [rcx]
140a090d7: xor r8d, r8d
140a090da: ucomiss xmm1, xmm0
140a090dd: seta r8b
140a090e1: mov dword ptr [rdx + rax*4 + 4], r8d
140a090e6: add rax, 2
140a090ea: add rcx, 0x18
140a090ee: cmp r13, rax
140a090f1: jne 0x140a090c0
140a090f3: jmp 0x140a090fa ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
140a090f5: mov edx, 4
140a090fa: lea rcx, [rbp - 0x20]
140a090fe: mov qword ptr [rbp + 0x60], rdx
140a09102: mov r8, r13
140a09105: mov qword ptr [rbp + 0x68], r13
140a09109: call 0x140a08090 ; country_core::render::mesh::Mesh::retain_vertices
140a0910e: mov rax, qword ptr [rbp + 0x50]
140a09112: mov qword ptr [rsi + 0x70], rax
140a09116: movups xmm0, xmmword ptr [rbp + 0x40]
140a0911a: movups xmmword ptr [rsi + 0x60], xmm0
140a0911e: movups xmm0, xmmword ptr [rbp + 0x30]
140a09122: movups xmmword ptr [rsi + 0x50], xmm0
140a09126: movups xmm0, xmmword ptr [rbp + 0x20]
140a0912a: movups xmmword ptr [rsi + 0x40], xmm0
140a0912e: movups xmm0, xmmword ptr [rbp - 0x20]
140a09132: movups xmm1, xmmword ptr [rbp - 0x10]
140a09136: movups xmm2, xmmword ptr [rbp]
140a0913a: movups xmm3, xmmword ptr [rbp + 0x10]
140a0913e: movups xmmword ptr [rsi + 0x30], xmm3
140a09142: movups xmmword ptr [rsi + 0x20], xmm2
140a09146: movups xmmword ptr [rsi + 0x10], xmm1
140a0914a: movups xmmword ptr [rsi], xmm0
140a0914d: cmp qword ptr [rbp + 0x68], 0
140a09152: je 0x140a09167
140a09154: mov r8d, 4
140a0915a: mov rcx, qword ptr [rbp + 0x60]
140a0915e: mov rdx, qword ptr [rbp + 0x70]
140a09162: call 0x140613c20
140a09167: mov rax, rsi
140a0916a: movaps xmm6, xmmword ptr [rbp + 0x80]
140a09171: add rsp, 0x118
140a09178: pop rbx
140a09179: pop rdi
140a0917a: pop rsi
140a0917b: pop r12
140a0917d: pop r13
140a0917f: pop r14
140a09181: pop r15
140a09183: pop rbp
140a09184: ret 
140a09185: mov rax, r13
140a09188: and rax, 0xfffffffffffffff8
140a0918c: lea rcx, [r8 + 0x5c]
140a09190: xor r9d, r9d
140a09193: movaps xmm0, xmmword ptr [rip + 0x1f4fe46]
140a0919a: nop word ptr [rax + rax]
140a091a0: movss xmm1, dword ptr [rcx - 0x54]
140a091a5: movss xmm2, dword ptr [rcx - 0x48]
140a091aa: unpcklps xmm1, xmm2
140a091ad: movss xmm2, dword ptr [rcx - 0x3c]
140a091b2: movss xmm3, dword ptr [rcx - 0x30]
140a091b7: unpcklps xmm2, xmm3
140a091ba: movlhps xmm1, xmm2
140a091bd: movss xmm2, dword ptr [rcx]
140a091c1: movss xmm3, dword ptr [rcx - 0xc]
140a091c6: unpcklps xmm3, xmm2
140a091c9: movss xmm2, dword ptr [rcx - 0x18]
140a091ce: movss xmm4, dword ptr [rcx - 0x24]
140a091d3: unpcklps xmm4, xmm2
140a091d6: movlhps xmm4, xmm3
140a091d9: xorps xmm2, xmm2
140a091dc: cmpltps xmm2, xmm1
140a091e0: andps xmm2, xmm0
140a091e3: xorps xmm1, xmm1
140a091e6: cmpltps xmm1, xmm4
140a091ea: andps xmm1, xmm0
140a091ed: movups xmmword ptr [rdx + r9*4], xmm2
140a091f2: movups xmmword ptr [rdx + r9*4 + 0x10], xmm1
140a091f8: add r9, 8
140a091fc: add rcx, 0x60
140a09200: cmp rax, r9
140a09203: jne 0x140a091a0
140a09205: cmp r13, rax
140a09208: jne 0x140a0907e
140a0920e: jmp 0x140a090fa ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
140a09213: lea rax, [rip + 0x20bddfe]
140a0921a: mov qword ptr [rbp - 0x20], rax
140a0921e: mov qword ptr [rbp - 0x18], 1
140a09226: mov qword ptr [rbp - 0x10], 8
140a0922e: pxor xmm0, xmm0
140a09232: movdqu xmmword ptr [rbp - 8], xmm0
140a09237: lea rdx, [rip + 0x20bde02]
140a0923e: lea rcx, [rbp - 0x20]
140a09242: call 0x1428d9390 ; core::panicking::panic_fmt
140a09247: lea r8, [rip + 0x222f92a]
140a0924e: mov rcx, r15
140a09251: mov rdx, rbx
140a09254: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a09259: jmp 0x140a0928c ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
140a0925b: lea r8, [rip + 0x20bd0c6]
140a09262: mov ecx, 4
140a09267: mov rdx, rbx
140a0926a: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a0926f: jmp 0x140a0928c ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
140a09271: mov qword ptr [rbp + 0x58], 0
140a09279: lea r8, [rip + 0x222f8f8]
140a09280: mov rcx, qword ptr [rbp + 0x58]
140a09284: mov rdx, rbx
140a09287: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a0928c: ud2 
140a0928e: nop 
140a09290: mov qword ptr [rsp + 0x10], rdx
140a09295: push rbp
140a09296: push r15
140a09298: push r14
140a0929a: push r13
140a0929c: push r12
140a0929e: push rsi
140a0929f: push rdi
140a092a0: push rbx
140a092a1: sub rsp, 0x38
140a092a5: lea rbp, [rdx + 0x80]
140a092ac: movaps xmmword ptr [rsp + 0x20], xmm6
140a092b1: lea rcx, [rbp - 0x50]
140a092b5: mov r8d, 0x38
140a092bb: mov r9d, 0x10
140a092c1: lea rdx, [rbp - 0x30]
140a092c5: call 0x140b75360 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global>
140a092ca: movaps xmm6, xmmword ptr [rsp + 0x20]
140a092cf: add rsp, 0x38
140a092d3: pop rbx
140a092d4: pop rdi
140a092d5: pop rsi
140a092d6: pop r12
140a092d8: pop r13
140a092da: pop r14
140a092dc: pop r15
140a092de: pop rbp
140a092df: ret 
140a092e0: mov qword ptr [rsp + 0x10], rdx
140a092e5: push rbp
140a092e6: push r15
140a092e8: push r14
140a092ea: push r13
140a092ec: push r12
140a092ee: push rsi
140a092ef: push rdi
140a092f0: push rbx
140a092f1: sub rsp, 0x38
140a092f5: lea rbp, [rdx + 0x80]
140a092fc: movaps xmmword ptr [rsp + 0x20], xmm6
140a09301: cmp qword ptr [rbp + 0x70], 0
140a09306: je 0x140a0931f
140a09308: mov rdx, qword ptr [rbp + 0x70]
140a0930c: shl rdx, 2
140a09310: mov r8d, 4
140a09316: mov rcx, qword ptr [rbp + 0x68]
140a0931a: call 0x140613c20
140a0931f: movaps xmm6, xmmword ptr [rsp + 0x20]
140a09324: add rsp, 0x38
140a09328: pop rbx
140a09329: pop rdi
140a0932a: pop rsi
140a0932b: pop r12
140a0932d: pop r13
140a0932f: pop r14
140a09331: pop r15
140a09333: pop rbp
140a09334: ret 
140a09335: nop word ptr cs:[rax + rax]
140a09340: mov qword ptr [rsp + 0x10], rdx
140a09345: push rbp
140a09346: push r15
140a09348: push r14
140a0934a: push r13
140a0934c: push r12
140a0934e: push rsi
140a0934f: push rdi
140a09350: push rbx
140a09351: sub rsp, 0x38
140a09355: lea rbp, [rdx + 0x80]
140a0935c: movaps xmmword ptr [rsp + 0x20], xmm6
140a09361: cmp qword ptr [rbp + 0x68], 0
140a09366: je 0x140a0937b
140a09368: mov r8d, 4
140a0936e: mov rcx, qword ptr [rbp + 0x60]
140a09372: mov rdx, qword ptr [rbp + 0x70]
140a09376: call 0x140613c20
140a0937b: movaps xmm6, xmmword ptr [rsp + 0x20]
140a09380: add rsp, 0x38
140a09384: pop rbx
140a09385: pop rdi
140a09386: pop rsi
140a09387: pop r12
140a09389: pop r13
140a0938b: pop r14
140a0938d: pop r15
140a0938f: pop rbp
140a09390: ret 
140a09391: nop word ptr cs:[rax + rax]
140a093a0: mov qword ptr [rsp + 0x10], rdx
140a093a5: push rbp
140a093a6: push r15
140a093a8: push r14
140a093aa: push r13
140a093ac: push r12
140a093ae: push rsi
140a093af: push rdi
140a093b0: push rbx
140a093b1: sub rsp, 0x38
140a093b5: lea rbp, [rdx + 0x80]
140a093bc: movaps xmmword ptr [rsp + 0x20], xmm6
140a093c1: lea rcx, [rbp - 0x20]
140a093c5: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
140a093ca: movaps xmm6, xmmword ptr [rsp + 0x20]
140a093cf: add rsp, 0x38
140a093d3: pop rbx
140a093d4: pop rdi
140a093d5: pop rsi
140a093d6: pop r12
140a093d8: pop r13
140a093da: pop r14
140a093dc: pop r15
140a093de: pop rbp
140a093df: ret 
