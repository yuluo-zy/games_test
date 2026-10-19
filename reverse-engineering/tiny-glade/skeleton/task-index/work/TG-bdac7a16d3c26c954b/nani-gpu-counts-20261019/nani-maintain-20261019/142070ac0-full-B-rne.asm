142070ac0: 55 push rbp
142070ac1: 4157 push r15
142070ac3: 4156 push r14
142070ac5: 4155 push r13
142070ac7: 4154 push r12
142070ac9: 56 push rsi
142070aca: 57 push rdi
142070acb: 53 push rbx
142070acc: 4881ec58010000 sub rsp, 0x158
142070ad3: 488dac2480000000 lea rbp, [rsp + 0x80]
142070adb: 440f2995c0000000 movaps xmmword ptr [rbp + 0xc0], xmm10
142070ae3: 440f298db0000000 movaps xmmword ptr [rbp + 0xb0], xmm9
142070aeb: 440f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm8
142070af3: 0f29bd90000000 movaps xmmword ptr [rbp + 0x90], xmm7
142070afa: 0f29b580000000 movaps xmmword ptr [rbp + 0x80], xmm6
142070b01: 48c74578feffffff mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
142070b09: 4889d6 mov rsi, rdx
142070b0c: 4889cf mov rdi, rcx
142070b0f: e8ec5ebafe call 0x140c16a00 ; puffin::are_scopes_on
142070b14: 84c0 test al, al
142070b16: 745f je 0x142070b77
142070b18: 4c8d35e026f600 lea r14, [rip + 0xf626e0]
142070b1f: bb43000000 mov ebx, 0x43
142070b24: ba43000000 mov edx, 0x43
142070b29: 4c89f1 mov rcx, r14
142070b2c: e83fe3ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
142070b31: 4883f801 cmp rax, 1
142070b35: 0f85b1000000 jne 0x142070bec
142070b3b: 4989d1 mov r9, rdx
142070b3e: 4885d2 test rdx, rdx
142070b41: 743d je 0x142070b80
142070b43: 4983f943 cmp r9, 0x43
142070b47: 7335 jae 0x142070b7e
142070b49: 488d05af26f600 lea rax, [rip + 0xf626af]
142070b50: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
142070b55: 7f29 jg 0x142070b80
142070b57: 488d057a28f600 lea rax, [rip + 0xf6287a]
142070b5e: 4889442420 mov qword ptr [rsp + 0x20], rax
142070b63: 488d0d9526f600 lea rcx, [rip + 0xf62695]
142070b6a: ba43000000 mov edx, 0x43
142070b6f: 4531c0 xor r8d, r8d
142070b72: e8c98d8600 call 0x1428d9940 ; core::str::slice_error_fail
142070b77: 31c0 xor eax, eax
142070b79: e9bc010000 jmp 0x142070d3a ; unknown
142070b7e: 75d7 jne 0x142070b57
142070b80: 4c8d357826f600 lea r14, [rip + 0xf62678]
142070b87: 4c89f1 mov rcx, r14
142070b8a: 4c89ca mov rdx, r9
142070b8d: e8dee2ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
142070b92: 4883f801 cmp rax, 1
142070b96: 7554 jne 0x142070bec
142070b98: 4989d0 mov r8, rdx
142070b9b: 4983c002 add r8, 2
142070b9f: 7439 je 0x142070bda
142070ba1: 4983f843 cmp r8, 0x43
142070ba5: 7331 jae 0x142070bd8
142070ba7: 488d055126f600 lea rax, [rip + 0xf62651]
142070bae: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
142070bb3: 7f25 jg 0x142070bda
142070bb5: 488d053428f600 lea rax, [rip + 0xf62834]
142070bbc: 4889442420 mov qword ptr [rsp + 0x20], rax
142070bc1: 488d0d3726f600 lea rcx, [rip + 0xf62637]
142070bc8: ba43000000 mov edx, 0x43
142070bcd: 41b943000000 mov r9d, 0x43
142070bd3: e8688d8600 call 0x1428d9940 ; core::str::slice_error_fail
142070bd8: 75db jne 0x142070bb5
142070bda: bb41000000 mov ebx, 0x41
142070bdf: 4829d3 sub rbx, rdx
142070be2: 4c8d351626f600 lea r14, [rip + 0xf62616]
142070be9: 4d01c6 add r14, r8
142070bec: 4c8d056b28f600 lea r8, [rip + 0xf6286b]
142070bf3: 4c8d3d3628f600 lea r15, [rip + 0xf62836]
142070bfa: 660f1f440000 nop word ptr [rax + rax]
142070c00: 410fbe40ff movsx eax, byte ptr [r8 - 1]
142070c05: 85c0 test eax, eax
142070c07: 7817 js 0x142070c20
142070c09: 49ffc8 dec r8
142070c0c: 83f85c cmp eax, 0x5c
142070c0f: 7563 jne 0x142070c74
142070c11: eb73 jmp 0x142070c86 ; unknown
142070c13: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142070c20: 410fb648fe movzx ecx, byte ptr [r8 - 2]
142070c25: 80f9c0 cmp cl, 0xc0
142070c28: 7d23 jge 0x142070c4d
142070c2a: 410fb650fd movzx edx, byte ptr [r8 - 3]
142070c2f: 80fac0 cmp dl, 0xc0
142070c32: 7d22 jge 0x142070c56
142070c34: 450fb648fc movzx r9d, byte ptr [r8 - 4]
142070c39: 4983c0fc add r8, -4
142070c3d: 4183e107 and r9d, 7
142070c41: 41c1e106 shl r9d, 6
142070c45: 83e23f and edx, 0x3f
142070c48: 4409ca or edx, r9d
142070c4b: eb10 jmp 0x142070c5d ; unknown
142070c4d: 4983c0fe add r8, -2
142070c51: 83e11f and ecx, 0x1f
142070c54: eb0f jmp 0x142070c65 ; unknown
142070c56: 4983c0fd add r8, -3
142070c5a: 83e20f and edx, 0xf
142070c5d: c1e206 shl edx, 6
142070c60: 83e13f and ecx, 0x3f
142070c63: 09d1 or ecx, edx
142070c65: c1e106 shl ecx, 6
142070c68: 243f and al, 0x3f
142070c6a: 0fb6c0 movzx eax, al
142070c6d: 09c8 or eax, ecx
142070c6f: 83f85c cmp eax, 0x5c
142070c72: 7412 je 0x142070c86
142070c74: 83f82f cmp eax, 0x2f
142070c77: 740d je 0x142070c86
142070c79: 4d39f8 cmp r8, r15
142070c7c: 7582 jne 0x142070c00
142070c7e: 41bc2e000000 mov r12d, 0x2e
142070c84: eb4d jmp 0x142070cd3 ; unknown
142070c86: 4c8d3da327f600 lea r15, [rip + 0xf627a3]
142070c8d: 4d29f8 sub r8, r15
142070c90: 49ffc0 inc r8
142070c93: 7432 je 0x142070cc7
142070c95: 4983f82e cmp r8, 0x2e
142070c99: 732a jae 0x142070cc5
142070c9b: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
142070ca0: 7f25 jg 0x142070cc7
142070ca2: 488d050f27f600 lea rax, [rip + 0xf6270f]
142070ca9: 4889442420 mov qword ptr [rsp + 0x20], rax
142070cae: 488d0d7b27f600 lea rcx, [rip + 0xf6277b]
142070cb5: ba2e000000 mov edx, 0x2e
142070cba: 41b92e000000 mov r9d, 0x2e
142070cc0: e87b8c8600 call 0x1428d9940 ; core::str::slice_error_fail
142070cc5: 75db jne 0x142070ca2
142070cc7: 41bc2e000000 mov r12d, 0x2e
142070ccd: 4d29c4 sub r12, r8
142070cd0: 4d01c7 add r15, r8
142070cd3: e85874bafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142070cd8: 4889c1 mov rcx, rax
142070cdb: 488b00 mov rax, qword ptr [rax]
142070cde: 4883f801 cmp rax, 1
142070ce2: 0f853c060000 jne 0x142071324
142070ce8: 4883c108 add rcx, 8
142070cec: 48833900 cmp qword ptr [rcx], 0
142070cf0: 0f8552060000 jne 0x142071348
142070cf6: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142070cfd: 48894d58 mov qword ptr [rbp + 0x58], rcx
142070d01: 4883c108 add rcx, 8
142070d05: 4c89642420 mov qword ptr [rsp + 0x20], r12
142070d0a: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
142070d13: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
142070d1c: 4c89f2 mov rdx, r14
142070d1f: 4989d8 mov r8, rbx
142070d22: 4d89f9 mov r9, r15
142070d25: e81660bafe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
142070d2a: 488b4d58 mov rcx, qword ptr [rbp + 0x58]
142070d2e: 48ff01 inc qword ptr [rcx]
142070d31: 48894538 mov qword ptr [rbp + 0x38], rax
142070d35: b801000000 mov eax, 1
142070d3a: 48894530 mov qword ptr [rbp + 0x30], rax
142070d3e: 488b0e mov rcx, qword ptr [rsi]
142070d41: b203 mov dl, 3
142070d43: e8080f92fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
142070d48: 48895528 mov qword ptr [rbp + 0x28], rdx
142070d4c: 48894548 mov qword ptr [rbp + 0x48], rax
142070d50: 488b1f mov rbx, qword ptr [rdi]
142070d53: 4c8b7708 mov r14, qword ptr [rdi + 8]
142070d57: 4c8bbbf8000000 mov r15, qword ptr [rbx + 0xf8]
142070d5e: 488b8300010000 mov rax, qword ptr [rbx + 0x100]
142070d65: 4d8d2487 lea r12, [r15 + rax*4]
142070d69: 0fb68308010000 movzx eax, byte ptr [rbx + 0x108]
142070d70: 884558 mov byte ptr [rbp + 0x58], al
142070d73: b808000000 mov eax, 8
142070d78: 48894540 mov qword ptr [rbp + 0x40], rax
142070d7c: 31ff xor edi, edi
142070d7e: f30f10357e4b8b00 movss xmm6, dword ptr [rip + 0x8b4b7e]
142070d86: 0f283df3db8b00 movaps xmm7, xmmword ptr [rip + 0x8bdbf3]
142070d8d: 31f6 xor esi, esi
142070d8f: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
142070d97: c7457400000000 mov dword ptr [rbp + 0x74], 0
142070d9e: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142070da6: 48c7456800000000 mov qword ptr [rbp + 0x68], 0
142070dae: 4531ed xor r13d, r13d
142070db1: 807d5800 cmp byte ptr [rbp + 0x58], 0
142070db5: 0f85b5010000 jne 0x142070f70
142070dbb: 4839fe cmp rsi, rdi
142070dbe: 0f8590010000 jne 0x142070f54
142070dc4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142070dd0: 4d39e7 cmp r15, r12
142070dd3: 0f848c040000 je 0x142071265
142070dd9: 418b0f mov ecx, dword ptr [r15]
142070ddc: 4983c704 add r15, 4
142070de0: 498b8600010000 mov rax, qword ptr [r14 + 0x100]
142070de7: 488d0c89 lea rcx, [rcx + rcx*4]
142070deb: 48c1e105 shl rcx, 5
142070def: 488b7c0858 mov rdi, qword ptr [rax + rcx + 0x58]
142070df4: 4885ff test rdi, rdi
142070df7: 74d7 je 0x142070dd0
142070df9: 4801c8 add rax, rcx
142070dfc: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
142070e02: 498b96a8010000 mov rdx, qword ptr [r14 + 0x1a8]
142070e09: 4c8d0cc9 lea r9, [rcx + rcx*8]
142070e0d: 4a8d0cca lea rcx, [rdx + r9*8]
142070e11: 4c8b8310010000 mov r8, qword ptr [rbx + 0x110]
142070e18: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
142070e1d: 4c39c2 cmp rdx, r8
142070e20: 7633 jbe 0x142070e55
142070e22: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
142070e26: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
142070e2a: 4d85c0 test r8, r8
142070e2d: 7426 je 0x142070e55
142070e2f: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
142070e33: 49f7d0 not r8
142070e36: 4f8d0440 lea r8, [r8 + r8*2]
142070e3a: 49c1e004 shl r8, 4
142070e3e: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
142070e43: 4c894560 mov qword ptr [rbp + 0x60], r8
142070e47: 4c8b8318010000 mov r8, qword ptr [rbx + 0x118]
142070e4e: 4c39c2 cmp rdx, r8
142070e51: 7716 ja 0x142070e69
142070e53: eb4e jmp 0x142070ea3 ; unknown
142070e55: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142070e5d: 4c8b8318010000 mov r8, qword ptr [rbx + 0x118]
142070e64: 4c39c2 cmp rdx, r8
142070e67: 763a jbe 0x142070ea3
142070e69: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
142070e6d: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
142070e71: 4d85c0 test r8, r8
142070e74: 742d je 0x142070ea3
142070e76: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
142070e7a: 49f7d0 not r8
142070e7d: 4f8d0440 lea r8, [r8 + r8*2]
142070e81: 49c1e004 shl r8, 4
142070e85: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
142070e8a: 4c894568 mov qword ptr [rbp + 0x68], r8
142070e8e: 4c8b8b20010000 mov r9, qword ptr [rbx + 0x120]
142070e95: 4c8b8088000000 mov r8, qword ptr [rax + 0x88]
142070e9c: 4d39c8 cmp r8, r9
142070e9f: 766a jbe 0x142070f0b
142070ea1: eb1b jmp 0x142070ebe ; unknown
142070ea3: 48c7456800000000 mov qword ptr [rbp + 0x68], 0
142070eab: 4c8b8b20010000 mov r9, qword ptr [rbx + 0x120]
142070eb2: 4c8b8088000000 mov r8, qword ptr [rax + 0x88]
142070eb9: 4d39c8 cmp r8, r9
142070ebc: 764d jbe 0x142070f0b
142070ebe: 4c8b9080000000 mov r10, qword ptr [rax + 0x80]
142070ec5: 4b833cca00 cmp qword ptr [r10 + r9*8], 0
142070eca: 743f je 0x142070f0b
142070ecc: 41b201 mov r10b, 1
142070ecf: 44895574 mov dword ptr [rbp + 0x74], r10d
142070ed3: 4c39ca cmp rdx, r9
142070ed6: 7648 jbe 0x142070f20
142070ed8: 488b5138 mov rdx, qword ptr [rcx + 0x38]
142070edc: 4a8b14ca mov rdx, qword ptr [rdx + r9*8]
142070ee0: 4885d2 test rdx, rdx
142070ee3: 743b je 0x142070f20
142070ee5: 488b4918 mov rcx, qword ptr [rcx + 0x18]
142070ee9: 48f7d2 not rdx
142070eec: 488d1452 lea rdx, [rdx + rdx*2]
142070ef0: 48c1e204 shl rdx, 4
142070ef4: 488b4c1110 mov rcx, qword ptr [rcx + rdx + 0x10]
142070ef9: 48894d50 mov qword ptr [rbp + 0x50], rcx
142070efd: 488b8b28010000 mov rcx, qword ptr [rbx + 0x128]
142070f04: 4939c8 cmp r8, rcx
142070f07: 772b ja 0x142070f34
142070f09: eb3c jmp 0x142070f47 ; unknown
142070f0b: c7457400000000 mov dword ptr [rbp + 0x74], 0
142070f12: 488b8b28010000 mov rcx, qword ptr [rbx + 0x128]
142070f19: 4939c8 cmp r8, rcx
142070f1c: 7629 jbe 0x142070f47
142070f1e: eb14 jmp 0x142070f34 ; unknown
142070f20: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
142070f28: 488b8b28010000 mov rcx, qword ptr [rbx + 0x128]
142070f2f: 4939c8 cmp r8, rcx
142070f32: 7613 jbe 0x142070f47
142070f34: 488b9080000000 mov rdx, qword ptr [rax + 0x80]
142070f3b: 48833cca00 cmp qword ptr [rdx + rcx*8], 0
142070f40: 7405 je 0x142070f47
142070f42: 41b501 mov r13b, 1
142070f45: eb03 jmp 0x142070f4a ; unknown
142070f47: 4531ed xor r13d, r13d
142070f4a: 488b4050 mov rax, qword ptr [rax + 0x50]
142070f4e: 48894540 mov qword ptr [rbp + 0x40], rax
142070f52: 31f6 xor esi, esi
142070f54: 4889f0 mov rax, rsi
142070f57: 48c1e004 shl rax, 4
142070f5b: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
142070f5f: 8b440108 mov eax, dword ptr [rcx + rax + 8]
142070f63: e94a010000 jmp 0x1420710b2 ; unknown
142070f68: 0f1f840000000000 nop dword ptr [rax + rax]
142070f70: 4839fe cmp rsi, rdi
142070f73: 0f8537010000 jne 0x1420710b0
142070f79: 0f1f8000000000 nop dword ptr [rax]
142070f80: 4d39e7 cmp r15, r12
142070f83: 0f84dc020000 je 0x142071265
142070f89: 418b0f mov ecx, dword ptr [r15]
142070f8c: 4983c704 add r15, 4
142070f90: 498b86a8010000 mov rax, qword ptr [r14 + 0x1a8]
142070f97: 488d0cc9 lea rcx, [rcx + rcx*8]
142070f9b: 488b7cc810 mov rdi, qword ptr [rax + rcx*8 + 0x10]
142070fa0: 4885ff test rdi, rdi
142070fa3: 74db je 0x142070f80
142070fa5: 488d04c8 lea rax, [rax + rcx*8]
142070fa9: 488b9310010000 mov rdx, qword ptr [rbx + 0x110]
142070fb0: 488b4840 mov rcx, qword ptr [rax + 0x40]
142070fb4: 4839d1 cmp rcx, rdx
142070fb7: 7633 jbe 0x142070fec
142070fb9: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142070fbd: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142070fc1: 4885d2 test rdx, rdx
142070fc4: 7426 je 0x142070fec
142070fc6: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142070fca: 48f7d2 not rdx
142070fcd: 488d1452 lea rdx, [rdx + rdx*2]
142070fd1: 48c1e204 shl rdx, 4
142070fd5: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142070fda: 48895560 mov qword ptr [rbp + 0x60], rdx
142070fde: 488b9318010000 mov rdx, qword ptr [rbx + 0x118]
142070fe5: 4839d1 cmp rcx, rdx
142070fe8: 7716 ja 0x142071000
142070fea: eb47 jmp 0x142071033 ; unknown
142070fec: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142070ff4: 488b9318010000 mov rdx, qword ptr [rbx + 0x118]
142070ffb: 4839d1 cmp rcx, rdx
142070ffe: 7633 jbe 0x142071033
142071000: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142071004: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142071008: 4885d2 test rdx, rdx
14207100b: 7426 je 0x142071033
14207100d: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142071011: 48f7d2 not rdx
142071014: 488d1452 lea rdx, [rdx + rdx*2]
142071018: 48c1e204 shl rdx, 4
14207101c: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142071021: 48895568 mov qword ptr [rbp + 0x68], rdx
142071025: 488b9320010000 mov rdx, qword ptr [rbx + 0x120]
14207102c: 4839d1 cmp rcx, rdx
14207102f: 7653 jbe 0x142071084
142071031: eb14 jmp 0x142071047 ; unknown
142071033: 48c7456800000000 mov qword ptr [rbp + 0x68], 0
14207103b: 488b9320010000 mov rdx, qword ptr [rbx + 0x120]
142071042: 4839d1 cmp rcx, rdx
142071045: 763d jbe 0x142071084
142071047: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207104b: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207104f: 4885d2 test rdx, rdx
142071052: 7430 je 0x142071084
142071054: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142071058: 48f7d2 not rdx
14207105b: 488d1452 lea rdx, [rdx + rdx*2]
14207105f: 48c1e204 shl rdx, 4
142071063: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142071068: 48895550 mov qword ptr [rbp + 0x50], rdx
14207106c: b201 mov dl, 1
14207106e: 895574 mov dword ptr [rbp + 0x74], edx
142071071: 488b9328010000 mov rdx, qword ptr [rbx + 0x128]
142071078: 4839d1 cmp rcx, rdx
14207107b: 771a ja 0x142071097
14207107d: 31f6 xor esi, esi
14207107f: 4531ed xor r13d, r13d
142071082: eb2c jmp 0x1420710b0 ; unknown
142071084: c7457400000000 mov dword ptr [rbp + 0x74], 0
14207108b: 488b9328010000 mov rdx, qword ptr [rbx + 0x128]
142071092: 4839d1 cmp rcx, rdx
142071095: 76e6 jbe 0x14207107d
142071097: 488b4038 mov rax, qword ptr [rax + 0x38]
14207109b: 48833cd000 cmp qword ptr [rax + rdx*8], 0
1420710a0: 410f95c5 setne r13b
1420710a4: 31f6 xor esi, esi
1420710a6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1420710b0: 89f0 mov eax, esi
1420710b2: 488d0440 lea rax, [rax + rax*2]
1420710b6: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
1420710ba: 488d1481 lea rdx, [rcx + rax*4]
1420710be: 48c1e004 shl rax, 4
1420710c2: 48034568 add rax, qword ptr [rbp + 0x68]
1420710c6: 8b4818 mov ecx, dword ptr [rax + 0x18]
1420710c9: 894d20 mov dword ptr [rbp + 0x20], ecx
1420710cc: 488b4810 mov rcx, qword ptr [rax + 0x10]
1420710d0: 48894d18 mov qword ptr [rbp + 0x18], rcx
1420710d4: 0f2820 movaps xmm4, xmmword ptr [rax]
1420710d7: f30f105808 movss xmm3, dword ptr [rax + 8]
1420710dc: 440f28c3 movaps xmm8, xmm3
1420710e0: f3440f58c3 addss xmm8, xmm3
1420710e5: f3410f59d8 mulss xmm3, xmm8
1420710ea: 0f28cc movaps xmm1, xmm4
1420710ed: 0f58cc addps xmm1, xmm4
1420710f0: f30f16c1 movshdup xmm0, xmm1
1420710f4: f30f59c4 mulss xmm0, xmm4
1420710f8: 0f28d4 movaps xmm2, xmm4
1420710fb: f3410f12e8 movsldup xmm5, xmm8
142071100: 0f59ec mulps xmm5, xmm4
142071103: 0fc6e4ff shufps xmm4, xmm4, 0xff
142071107: 0f59d1 mulps xmm2, xmm1
14207110a: 0f59cc mulps xmm1, xmm4
14207110d: 440f28cc movaps xmm9, xmm4
142071111: f3450f59c8 mulss xmm9, xmm8
142071116: f3440f16d2 movshdup xmm10, xmm2
14207111b: 450f28c2 movaps xmm8, xmm10
14207111f: f3440f58c3 addss xmm8, xmm3
142071124: 0f28e6 movaps xmm4, xmm6
142071127: f3410f5ce0 subss xmm4, xmm8
14207112c: 440f28c0 movaps xmm8, xmm0
142071130: f3450f58c1 addss xmm8, xmm9
142071135: 410f14e0 unpcklps xmm4, xmm8
142071139: f3410f5cc1 subss xmm0, xmm9
14207113e: f30f58da addss xmm3, xmm2
142071142: 440f28c6 movaps xmm8, xmm6
142071146: f3440f5cc3 subss xmm8, xmm3
14207114b: 0fc6c9e1 shufps xmm1, xmm1, 0xe1
14207114f: 410f14c0 unpcklps xmm0, xmm8
142071153: 440f28c5 movaps xmm8, xmm5
142071157: 440f5cc1 subps xmm8, xmm1
14207115b: 0f58cd addps xmm1, xmm5
14207115e: f30f105820 movss xmm3, dword ptr [rax + 0x20]
142071163: f3410f58d2 addss xmm2, xmm10
142071168: 440f28ce movaps xmm9, xmm6
14207116c: f3440f5cca subss xmm9, xmm2
142071171: f30f106824 movss xmm5, dword ptr [rax + 0x24]
142071176: 410f16e0 movlhps xmm4, xmm8
14207117a: 0fc6c1d4 shufps xmm0, xmm1, 0xd4
14207117e: 440fc6c101 shufps xmm8, xmm1, 1
142071183: 440fc6c930 shufps xmm9, xmm1, 0x30
142071188: 0f10501c movups xmm2, xmmword ptr [rax + 0x1c]
14207118c: 450fc6c182 shufps xmm8, xmm9, 0x82
142071191: 0f104820 movups xmm1, xmmword ptr [rax + 0x20]
142071195: 0fc6d200 shufps xmm2, xmm2, 0
142071199: 0f59d4 mulps xmm2, xmm4
14207119c: 0f28e1 movaps xmm4, xmm1
14207119f: 0fc6e100 shufps xmm4, xmm1, 0
1420711a3: 0f59e0 mulps xmm4, xmm0
1420711a6: 0fc6c955 shufps xmm1, xmm1, 0x55
1420711aa: 410f59c8 mulps xmm1, xmm8
1420711ae: 0f54df andps xmm3, xmm7
1420711b1: 0f54ef andps xmm5, xmm7
1420711b4: 0f28c3 movaps xmm0, xmm3
1420711b7: f30fc2c303 cmpunordss xmm0, xmm3
1420711bc: 440f28c0 movaps xmm8, xmm0
1420711c0: 440f54c5 andps xmm8, xmm5
1420711c4: f30f5feb maxss xmm5, xmm3
1420711c8: f30f10581c movss xmm3, dword ptr [rax + 0x1c]
1420711cd: 0f54df andps xmm3, xmm7
1420711d0: 0f55c5 andnps xmm0, xmm5
1420711d3: 410f56c0 orps xmm0, xmm8
1420711d7: 0f28e8 movaps xmm5, xmm0
1420711da: f30f5feb maxss xmm5, xmm3
1420711de: f30fc2db03 cmpunordss xmm3, xmm3
1420711e3: 440f28c3 movaps xmm8, xmm3
1420711e7: 440f54c0 andps xmm8, xmm0
1420711eb: 0f55dd andnps xmm3, xmm5
1420711ee: 410f56d8 orps xmm3, xmm8
1420711f2: 0f28c2 movaps xmm0, xmm2
1420711f5: f20f10c4 movsd xmm0, xmm4
1420711f9: 0fc6d024 shufps xmm2, xmm0, 0x24
1420711fd: 0f284010 movaps xmm0, xmmword ptr [rax + 0x10]
142071201: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
142071205: 0fc6e149 shufps xmm4, xmm1, 0x49
142071209: 0f2965d0 movaps xmmword ptr [rbp - 0x30], xmm4
14207120d: 0fc6c802 shufps xmm1, xmm0, 2
142071211: 0fc6c898 shufps xmm1, xmm0, 0x98
142071215: 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
142071219: 4489e8 mov eax, r13d
14207121c: 2401 and al, 1
14207121e: 0fb6c0 movzx eax, al
142071221: 48837d5000 cmp qword ptr [rbp + 0x50], 0
142071226: 0f95c1 setne cl
142071229: 224d74 and cl, byte ptr [rbp + 0x74]
14207122c: 0fb6c9 movzx ecx, cl
14207122f: 894df0 mov dword ptr [rbp - 0x10], ecx
142071232: 8945f4 mov dword ptr [rbp - 0xc], eax
142071235: 48c745f800000000 mov qword ptr [rbp - 8], 0
14207123d: 488d45c0 lea rax, [rbp - 0x40]
142071241: 4889442420 mov qword ptr [rsp + 0x20], rax
142071246: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
14207124a: 4c8d4518 lea r8, [rbp + 0x18]
14207124e: e83dcc4fff call 0x14156de90 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorInstanceData>
142071253: 48ffc6 inc rsi
142071256: 807d5800 cmp byte ptr [rbp + 0x58], 0
14207125a: 0f845bfbffff je 0x142070dbb
142071260: e90bfdffff jmp 0x142070f70 ; unknown
142071265: 488b7548 mov rsi, qword ptr [rbp + 0x48]
142071269: 4889f1 mov rcx, rsi
14207126c: e89f0bc1fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
142071271: 4889f1 mov rcx, rsi
142071274: 4883c901 or rcx, 1
142071278: e8930bc1fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207127d: 31c9 xor ecx, ecx
14207127f: b808000000 mov eax, 8
142071284: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
142071289: 0f85c5000000 jne 0x142071354
14207128f: 48837d3000 cmp qword ptr [rbp + 0x30], 0
142071294: 743e je 0x1420712d4
142071296: e8956ebafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207129b: 4889c1 mov rcx, rax
14207129e: 488b00 mov rax, qword ptr [rax]
1420712a1: 4883f801 cmp rax, 1
1420712a5: 7567 jne 0x14207130e
1420712a7: 4883c108 add rcx, 8
1420712ab: 48833900 cmp qword ptr [rcx], 0
1420712af: 0f8593000000 jne 0x142071348
1420712b5: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1420712bc: 48894d58 mov qword ptr [rbp + 0x58], rcx
1420712c0: 4883c108 add rcx, 8
1420712c4: 488b5538 mov rdx, qword ptr [rbp + 0x38]
1420712c8: e8235ebafe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1420712cd: 488b4558 mov rax, qword ptr [rbp + 0x58]
1420712d1: 48ff00 inc qword ptr [rax]
1420712d4: 0f28b580000000 movaps xmm6, xmmword ptr [rbp + 0x80]
1420712db: 0f28bd90000000 movaps xmm7, xmmword ptr [rbp + 0x90]
1420712e2: 440f2885a0000000 movaps xmm8, xmmword ptr [rbp + 0xa0]
1420712ea: 440f288db0000000 movaps xmm9, xmmword ptr [rbp + 0xb0]
1420712f2: 440f2895c0000000 movaps xmm10, xmmword ptr [rbp + 0xc0]
1420712fa: 4881c458010000 add rsp, 0x158
142071301: 5b pop rbx
142071302: 5f pop rdi
142071303: 5e pop rsi
142071304: 415c pop r12
142071306: 415d pop r13
142071308: 415e pop r14
14207130a: 415f pop r15
14207130c: 5d pop rbp
14207130d: c3 ret 
14207130e: 4885c0 test rax, rax
142071311: 7529 jne 0x14207133c
142071313: 31d2 xor edx, edx
142071315: e8b6748800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14207131a: 4889c1 mov rcx, rax
14207131d: 4885c0 test rax, rax
142071320: 7589 jne 0x1420712ab
142071322: eb18 jmp 0x14207133c ; unknown
142071324: 4885c0 test rax, rax
142071327: 7513 jne 0x14207133c
142071329: 31d2 xor edx, edx
14207132b: e8a0748800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142071330: 4889c1 mov rcx, rax
142071333: 4885c0 test rax, rax
142071336: 0f85b0f9ffff jne 0x142070cec
14207133c: 488d0d5d97bc00 lea rcx, [rip + 0xbc975d]
142071343: e8886d8600 call 0x1428d80d0 ; std::thread::local::panic_access_error
142071348: 488d0d2198bc00 lea rcx, [rip + 0xbc9821]
14207134f: e8fc7e8600 call 0x1428d9250 ; core::cell::panic_already_borrowed
142071354: 4889f1 mov rcx, rsi
142071357: 31d2 xor edx, edx
142071359: e802ac8500 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
14207135e: e92cffffff jmp 0x14207128f ; unknown
142071363: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142071370: 4889542410 mov qword ptr [rsp + 0x10], rdx
142071375: 55 push rbp
142071376: 4157 push r15
142071378: 4156 push r14
14207137a: 4155 push r13
14207137c: 4154 push r12
14207137e: 56 push rsi
14207137f: 57 push rdi
142071380: 53 push rbx
142071381: 4881ec88000000 sub rsp, 0x88
142071388: 488daa80000000 lea rbp, [rdx + 0x80]
14207138f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142071395: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
14207139b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1420713a1: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1420713a6: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1420713ab: 488b4558 mov rax, qword ptr [rbp + 0x58]
1420713af: 48ff00 inc qword ptr [rax]
1420713b2: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1420713b7: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1420713bc: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1420713c2: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1420713c8: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1420713ce: 4881c488000000 add rsp, 0x88
1420713d5: 5b pop rbx
1420713d6: 5f pop rdi
1420713d7: 5e pop rsi
1420713d8: 415c pop r12
1420713da: 415d pop r13
1420713dc: 415e pop r14
1420713de: 415f pop r15
1420713e0: 5d pop rbp
1420713e1: c3 ret 
1420713e2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1420713f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420713f5: 55 push rbp
1420713f6: 4157 push r15
1420713f8: 4156 push r14
1420713fa: 4155 push r13
1420713fc: 4154 push r12
1420713fe: 56 push rsi
1420713ff: 57 push rdi
142071400: 53 push rbx
142071401: 4881ec88000000 sub rsp, 0x88
142071408: 488daa80000000 lea rbp, [rdx + 0x80]
14207140f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142071415: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
14207141b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142071421: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142071426: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
14207142b: 488b4558 mov rax, qword ptr [rbp + 0x58]
14207142f: 48ff00 inc qword ptr [rax]
142071432: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142071437: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
14207143c: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142071442: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142071448: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
14207144e: 4881c488000000 add rsp, 0x88
142071455: 5b pop rbx
142071456: 5f pop rdi
142071457: 5e pop rsi
142071458: 415c pop r12
14207145a: 415d pop r13
14207145c: 415e pop r14
14207145e: 415f pop r15
142071460: 5d pop rbp
142071461: c3 ret 
142071462: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142071470: 4889542410 mov qword ptr [rsp + 0x10], rdx
142071475: 55 push rbp
142071476: 4157 push r15
142071478: 4156 push r14
14207147a: 4155 push r13
14207147c: 4154 push r12
14207147e: 56 push rsi
14207147f: 57 push rdi
142071480: 53 push rbx
142071481: 4881ec88000000 sub rsp, 0x88
142071488: 488daa80000000 lea rbp, [rdx + 0x80]
14207148f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142071495: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
14207149b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1420714a1: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1420714a6: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1420714ab: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
1420714af: e81cfefcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
1420714b4: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1420714b9: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1420714be: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1420714c4: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1420714ca: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1420714d0: 4881c488000000 add rsp, 0x88
1420714d7: 5b pop rbx
1420714d8: 5f pop rdi
1420714d9: 5e pop rsi
1420714da: 415c pop r12
1420714dc: 415d pop r13
1420714de: 415e pop r14
1420714e0: 415f pop r15
1420714e2: 5d pop rbp
1420714e3: c3 ret 
1420714e4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1420714f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420714f5: 55 push rbp
1420714f6: 4157 push r15
1420714f8: 4156 push r14
1420714fa: 4155 push r13
1420714fc: 4154 push r12
1420714fe: 56 push rsi
1420714ff: 57 push rdi
142071500: 53 push rbx
142071501: 4881ec88000000 sub rsp, 0x88
142071508: 488daa80000000 lea rbp, [rdx + 0x80]
14207150f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142071515: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
14207151b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142071521: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142071526: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
14207152b: 48837d3000 cmp qword ptr [rbp + 0x30], 0
142071530: 7420 je 0x142071552
142071532: 488d4538 lea rax, [rbp + 0x38]
142071536: 48894510 mov qword ptr [rbp + 0x10], rax
14207153a: 488d4510 lea rax, [rbp + 0x10]
14207153e: 48894508 mov qword ptr [rbp + 8], rax
142071542: 488d0d0f1ef600 lea rcx, [rip + 0xf61e0f]
142071549: 488d5508 lea rdx, [rbp + 8]
14207154d: e87edd2bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
142071552: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142071557: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
14207155c: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142071562: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142071568: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
14207156e: 4881c488000000 add rsp, 0x88
142071575: 5b pop rbx
142071576: 5f pop rdi
142071577: 5e pop rsi
142071578: 415c pop r12
14207157a: 415d pop r13
14207157c: 415e pop r14
14207157e: 415f pop r15
142071580: 5d pop rbp
142071581: c3 ret 
