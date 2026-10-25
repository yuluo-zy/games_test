142072b50: 55 push rbp
142072b51: 4157 push r15
142072b53: 4156 push r14
142072b55: 4155 push r13
142072b57: 4154 push r12
142072b59: 56 push rsi
142072b5a: 57 push rdi
142072b5b: 53 push rbx
142072b5c: 4881ec88010000 sub rsp, 0x188
142072b63: 488dac2480000000 lea rbp, [rsp + 0x80]
142072b6b: 440f29b5f0000000 movaps xmmword ptr [rbp + 0xf0], xmm14
142072b73: 440f29ade0000000 movaps xmmword ptr [rbp + 0xe0], xmm13
142072b7b: 440f29a5d0000000 movaps xmmword ptr [rbp + 0xd0], xmm12
142072b83: 440f299dc0000000 movaps xmmword ptr [rbp + 0xc0], xmm11
142072b8b: 440f2995b0000000 movaps xmmword ptr [rbp + 0xb0], xmm10
142072b93: 440f298da0000000 movaps xmmword ptr [rbp + 0xa0], xmm9
142072b9b: 440f298590000000 movaps xmmword ptr [rbp + 0x90], xmm8
142072ba3: 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
142072baa: 0f297570 movaps xmmword ptr [rbp + 0x70], xmm6
142072bae: 48c74568feffffff mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
142072bb6: 4889d6 mov rsi, rdx
142072bb9: 4889cf mov rdi, rcx
142072bbc: e83f3ebafe call 0x140c16a00 ; puffin::are_scopes_on
142072bc1: 84c0 test al, al
142072bc3: 745f je 0x142072c24
142072bc5: 4c8d35d801f600 lea r14, [rip + 0xf601d8]
142072bcc: bb4a000000 mov ebx, 0x4a
142072bd1: ba4a000000 mov edx, 0x4a
142072bd6: 4c89f1 mov rcx, r14
142072bd9: e892c2ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
142072bde: 4883f801 cmp rax, 1
142072be2: 0f85b1000000 jne 0x142072c99
142072be8: 4989d1 mov r9, rdx
142072beb: 4885d2 test rdx, rdx
142072bee: 743d je 0x142072c2d
142072bf0: 4983f94a cmp r9, 0x4a
142072bf4: 7335 jae 0x142072c2b
142072bf6: 488d05a701f600 lea rax, [rip + 0xf601a7]
142072bfd: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
142072c02: 7f29 jg 0x142072c2d
142072c04: 488d05cd07f600 lea rax, [rip + 0xf607cd]
142072c0b: 4889442420 mov qword ptr [rsp + 0x20], rax
142072c10: 488d0d8d01f600 lea rcx, [rip + 0xf6018d]
142072c17: ba4a000000 mov edx, 0x4a
142072c1c: 4531c0 xor r8d, r8d
142072c1f: e81c6d8600 call 0x1428d9940 ; core::str::slice_error_fail
142072c24: 31c0 xor eax, eax
142072c26: e9bf010000 jmp 0x142072dea ; unknown
142072c2b: 75d7 jne 0x142072c04
142072c2d: 4c8d357001f600 lea r14, [rip + 0xf60170]
142072c34: 4c89f1 mov rcx, r14
142072c37: 4c89ca mov rdx, r9
142072c3a: e831c2ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
142072c3f: 4883f801 cmp rax, 1
142072c43: 7554 jne 0x142072c99
142072c45: 4989d0 mov r8, rdx
142072c48: 4983c002 add r8, 2
142072c4c: 7439 je 0x142072c87
142072c4e: 4983f84a cmp r8, 0x4a
142072c52: 7331 jae 0x142072c85
142072c54: 488d054901f600 lea rax, [rip + 0xf60149]
142072c5b: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
142072c60: 7f25 jg 0x142072c87
142072c62: 488d058707f600 lea rax, [rip + 0xf60787]
142072c69: 4889442420 mov qword ptr [rsp + 0x20], rax
142072c6e: 488d0d2f01f600 lea rcx, [rip + 0xf6012f]
142072c75: ba4a000000 mov edx, 0x4a
142072c7a: 41b94a000000 mov r9d, 0x4a
142072c80: e8bb6c8600 call 0x1428d9940 ; core::str::slice_error_fail
142072c85: 75db jne 0x142072c62
142072c87: bb48000000 mov ebx, 0x48
142072c8c: 4829d3 sub rbx, rdx
142072c8f: 4c8d350e01f600 lea r14, [rip + 0xf6010e]
142072c96: 4d01c6 add r14, r8
142072c99: 4c8d05be07f600 lea r8, [rip + 0xf607be]
142072ca0: 4c8d3d8907f600 lea r15, [rip + 0xf60789]
142072ca7: 660f1f840000000000 nop word ptr [rax + rax]
142072cb0: 410fbe40ff movsx eax, byte ptr [r8 - 1]
142072cb5: 85c0 test eax, eax
142072cb7: 7817 js 0x142072cd0
142072cb9: 49ffc8 dec r8
142072cbc: 83f85c cmp eax, 0x5c
142072cbf: 7563 jne 0x142072d24
142072cc1: eb73 jmp 0x142072d36 ; unknown
142072cc3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142072cd0: 410fb648fe movzx ecx, byte ptr [r8 - 2]
142072cd5: 80f9c0 cmp cl, 0xc0
142072cd8: 7d23 jge 0x142072cfd
142072cda: 410fb650fd movzx edx, byte ptr [r8 - 3]
142072cdf: 80fac0 cmp dl, 0xc0
142072ce2: 7d22 jge 0x142072d06
142072ce4: 450fb648fc movzx r9d, byte ptr [r8 - 4]
142072ce9: 4983c0fc add r8, -4
142072ced: 4183e107 and r9d, 7
142072cf1: 41c1e106 shl r9d, 6
142072cf5: 83e23f and edx, 0x3f
142072cf8: 4409ca or edx, r9d
142072cfb: eb10 jmp 0x142072d0d ; unknown
142072cfd: 4983c0fe add r8, -2
142072d01: 83e11f and ecx, 0x1f
142072d04: eb0f jmp 0x142072d15 ; unknown
142072d06: 4983c0fd add r8, -3
142072d0a: 83e20f and edx, 0xf
142072d0d: c1e206 shl edx, 6
142072d10: 83e13f and ecx, 0x3f
142072d13: 09d1 or ecx, edx
142072d15: c1e106 shl ecx, 6
142072d18: 243f and al, 0x3f
142072d1a: 0fb6c0 movzx eax, al
142072d1d: 09c8 or eax, ecx
142072d1f: 83f85c cmp eax, 0x5c
142072d22: 7412 je 0x142072d36
142072d24: 83f82f cmp eax, 0x2f
142072d27: 740d je 0x142072d36
142072d29: 4d39f8 cmp r8, r15
142072d2c: 7582 jne 0x142072cb0
142072d2e: 41bc2e000000 mov r12d, 0x2e
142072d34: eb4d jmp 0x142072d83 ; unknown
142072d36: 4c8d3df306f600 lea r15, [rip + 0xf606f3]
142072d3d: 4d29f8 sub r8, r15
142072d40: 49ffc0 inc r8
142072d43: 7432 je 0x142072d77
142072d45: 4983f82e cmp r8, 0x2e
142072d49: 732a jae 0x142072d75
142072d4b: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
142072d50: 7f25 jg 0x142072d77
142072d52: 488d055f06f600 lea rax, [rip + 0xf6065f]
142072d59: 4889442420 mov qword ptr [rsp + 0x20], rax
142072d5e: 488d0dcb06f600 lea rcx, [rip + 0xf606cb]
142072d65: ba2e000000 mov edx, 0x2e
142072d6a: 41b92e000000 mov r9d, 0x2e
142072d70: e8cb6b8600 call 0x1428d9940 ; core::str::slice_error_fail
142072d75: 75db jne 0x142072d52
142072d77: 41bc2e000000 mov r12d, 0x2e
142072d7d: 4d29c4 sub r12, r8
142072d80: 4d01c7 add r15, r8
142072d83: e8a853bafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142072d88: 4889c1 mov rcx, rax
142072d8b: 488b00 mov rax, qword ptr [rax]
142072d8e: 4883f801 cmp rax, 1
142072d92: 0f8591050000 jne 0x142073329
142072d98: 4883c108 add rcx, 8
142072d9c: 48833900 cmp qword ptr [rcx], 0
142072da0: 0f85a7050000 jne 0x14207334d
142072da6: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142072dad: 48894d58 mov qword ptr [rbp + 0x58], rcx
142072db1: 4883c108 add rcx, 8
142072db5: 4c89642420 mov qword ptr [rsp + 0x20], r12
142072dba: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
142072dc3: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
142072dcc: 4c89f2 mov rdx, r14
142072dcf: 4989d8 mov r8, rbx
142072dd2: 4d89f9 mov r9, r15
142072dd5: e8663fbafe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
142072dda: 488b4d58 mov rcx, qword ptr [rbp + 0x58]
142072dde: 48ff01 inc qword ptr [rcx]
142072de1: 48894538 mov qword ptr [rbp + 0x38], rax
142072de5: b801000000 mov eax, 1
142072dea: 48894530 mov qword ptr [rbp + 0x30], rax
142072dee: 488b0e mov rcx, qword ptr [rsi]
142072df1: b210 mov dl, 0x10
142072df3: e858ee91fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
142072df8: 48895528 mov qword ptr [rbp + 0x28], rdx
142072dfc: 48894548 mov qword ptr [rbp + 0x48], rax
142072e00: 488b0f mov rcx, qword ptr [rdi]
142072e03: 4c8b7708 mov r14, qword ptr [rdi + 8]
142072e07: 4c8bb9f8000000 mov r15, qword ptr [rcx + 0xf8]
142072e0e: 488b8100010000 mov rax, qword ptr [rcx + 0x100]
142072e15: 4d8d2487 lea r12, [r15 + rax*4]
142072e19: 48894d50 mov qword ptr [rbp + 0x50], rcx
142072e1d: 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
142072e24: 884558 mov byte ptr [rbp + 0x58], al
142072e27: b808000000 mov eax, 8
142072e2c: 48894540 mov qword ptr [rbp + 0x40], rax
142072e30: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142072e38: 0f283541bb8b00 movaps xmm6, xmmword ptr [rip + 0x8bbb41]
142072e3f: f30f103dbd2a8b00 movss xmm7, dword ptr [rip + 0x8b2abd]
142072e47: 4531ed xor r13d, r13d
142072e4a: 31f6 xor esi, esi
142072e4c: 31ff xor edi, edi
142072e4e: 31db xor ebx, ebx
142072e50: 807d5800 cmp byte ptr [rbp + 0x58], 0
142072e54: 0f8536010000 jne 0x142072f90
142072e5a: 4839fb cmp rbx, rdi
142072e5d: 0f8517010000 jne 0x142072f7a
142072e63: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142072e70: 4d39e7 cmp r15, r12
142072e73: 0f84cc030000 je 0x142073245
142072e79: 418b0f mov ecx, dword ptr [r15]
142072e7c: 4983c704 add r15, 4
142072e80: 498b8600010000 mov rax, qword ptr [r14 + 0x100]
142072e87: 488d0c89 lea rcx, [rcx + rcx*4]
142072e8b: 48c1e105 shl rcx, 5
142072e8f: 488b7c0858 mov rdi, qword ptr [rax + rcx + 0x58]
142072e94: 4885ff test rdi, rdi
142072e97: 74d7 je 0x142072e70
142072e99: 4801c8 add rax, rcx
142072e9c: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
142072ea2: 498b96a8010000 mov rdx, qword ptr [r14 + 0x1a8]
142072ea9: 4c8d0cc9 lea r9, [rcx + rcx*8]
142072ead: 4a8d0cca lea rcx, [rdx + r9*8]
142072eb1: 4c8b5550 mov r10, qword ptr [rbp + 0x50]
142072eb5: 4d8b8210010000 mov r8, qword ptr [r10 + 0x110]
142072ebc: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
142072ec1: 4c39c2 cmp rdx, r8
142072ec4: 7633 jbe 0x142072ef9
142072ec6: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
142072eca: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
142072ece: 4d85c0 test r8, r8
142072ed1: 7426 je 0x142072ef9
142072ed3: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
142072ed7: 49f7d0 not r8
142072eda: 4f8d0440 lea r8, [r8 + r8*2]
142072ede: 49c1e004 shl r8, 4
142072ee2: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
142072ee7: 4c894560 mov qword ptr [rbp + 0x60], r8
142072eeb: 4d8b8218010000 mov r8, qword ptr [r10 + 0x118]
142072ef2: 4c39c2 cmp rdx, r8
142072ef5: 7716 ja 0x142072f0d
142072ef7: eb43 jmp 0x142072f3c ; unknown
142072ef9: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142072f01: 4d8b8218010000 mov r8, qword ptr [r10 + 0x118]
142072f08: 4c39c2 cmp rdx, r8
142072f0b: 762f jbe 0x142072f3c
142072f0d: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
142072f11: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
142072f15: 4d85c0 test r8, r8
142072f18: 7422 je 0x142072f3c
142072f1a: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
142072f1e: 49f7d0 not r8
142072f21: 4f8d0440 lea r8, [r8 + r8*2]
142072f25: 49c1e004 shl r8, 4
142072f29: 4f8b6c0110 mov r13, qword ptr [r9 + r8 + 0x10]
142072f2e: 4d8b8220010000 mov r8, qword ptr [r10 + 0x120]
142072f35: 4c39c2 cmp rdx, r8
142072f38: 7634 jbe 0x142072f6e
142072f3a: eb0f jmp 0x142072f4b ; unknown
142072f3c: 4531ed xor r13d, r13d
142072f3f: 4d8b8220010000 mov r8, qword ptr [r10 + 0x120]
142072f46: 4c39c2 cmp rdx, r8
142072f49: 7623 jbe 0x142072f6e
142072f4b: 488b5138 mov rdx, qword ptr [rcx + 0x38]
142072f4f: 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
142072f53: 4885d2 test rdx, rdx
142072f56: 7416 je 0x142072f6e
142072f58: 488b4918 mov rcx, qword ptr [rcx + 0x18]
142072f5c: 48f7d2 not rdx
142072f5f: 488d1452 lea rdx, [rdx + rdx*2]
142072f63: 48c1e204 shl rdx, 4
142072f67: 488b741110 mov rsi, qword ptr [rcx + rdx + 0x10]
142072f6c: eb02 jmp 0x142072f70 ; unknown
142072f6e: 31f6 xor esi, esi
142072f70: 488b4050 mov rax, qword ptr [rax + 0x50]
142072f74: 48894540 mov qword ptr [rbp + 0x40], rax
142072f78: 31db xor ebx, ebx
142072f7a: 4889d8 mov rax, rbx
142072f7d: 48c1e004 shl rax, 4
142072f81: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
142072f85: 8b440108 mov eax, dword ptr [rcx + rax + 8]
142072f89: e9fd000000 jmp 0x14207308b ; unknown
142072f8e: 6690 nop 
142072f90: 4839fb cmp rbx, rdi
142072f93: 0f85f0000000 jne 0x142073089
142072f99: 0f1f8000000000 nop dword ptr [rax]
142072fa0: 4d39e7 cmp r15, r12
142072fa3: 0f849c020000 je 0x142073245
142072fa9: 418b0f mov ecx, dword ptr [r15]
142072fac: 4983c704 add r15, 4
142072fb0: 498b86a8010000 mov rax, qword ptr [r14 + 0x1a8]
142072fb7: 488d0cc9 lea rcx, [rcx + rcx*8]
142072fbb: 488b7cc810 mov rdi, qword ptr [rax + rcx*8 + 0x10]
142072fc0: 4885ff test rdi, rdi
142072fc3: 74db je 0x142072fa0
142072fc5: 488d04c8 lea rax, [rax + rcx*8]
142072fc9: 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
142072fcd: 498b9110010000 mov rdx, qword ptr [r9 + 0x110]
142072fd4: 488b4840 mov rcx, qword ptr [rax + 0x40]
142072fd8: 4839d1 cmp rcx, rdx
142072fdb: 7633 jbe 0x142073010
142072fdd: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142072fe1: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142072fe5: 4885d2 test rdx, rdx
142072fe8: 7426 je 0x142073010
142072fea: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142072fee: 48f7d2 not rdx
142072ff1: 488d1452 lea rdx, [rdx + rdx*2]
142072ff5: 48c1e204 shl rdx, 4
142072ff9: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142072ffe: 48895560 mov qword ptr [rbp + 0x60], rdx
142073002: 498b9118010000 mov rdx, qword ptr [r9 + 0x118]
142073009: 4839d1 cmp rcx, rdx
14207300c: 7716 ja 0x142073024
14207300e: eb43 jmp 0x142073053 ; unknown
142073010: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142073018: 498b9118010000 mov rdx, qword ptr [r9 + 0x118]
14207301f: 4839d1 cmp rcx, rdx
142073022: 762f jbe 0x142073053
142073024: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142073028: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207302c: 4885d2 test rdx, rdx
14207302f: 7422 je 0x142073053
142073031: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142073035: 48f7d2 not rdx
142073038: 488d1452 lea rdx, [rdx + rdx*2]
14207303c: 48c1e204 shl rdx, 4
142073040: 4d8b6c1010 mov r13, qword ptr [r8 + rdx + 0x10]
142073045: 498b9120010000 mov rdx, qword ptr [r9 + 0x120]
14207304c: 4839d1 cmp rcx, rdx
14207304f: 7634 jbe 0x142073085
142073051: eb0f jmp 0x142073062 ; unknown
142073053: 4531ed xor r13d, r13d
142073056: 498b9120010000 mov rdx, qword ptr [r9 + 0x120]
14207305d: 4839d1 cmp rcx, rdx
142073060: 7623 jbe 0x142073085
142073062: 488b4838 mov rcx, qword ptr [rax + 0x38]
142073066: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
14207306a: 4885c9 test rcx, rcx
14207306d: 7416 je 0x142073085
14207306f: 488b4018 mov rax, qword ptr [rax + 0x18]
142073073: 48f7d1 not rcx
142073076: 488d0c49 lea rcx, [rcx + rcx*2]
14207307a: 48c1e104 shl rcx, 4
14207307e: 488b740810 mov rsi, qword ptr [rax + rcx + 0x10]
142073083: eb02 jmp 0x142073087 ; unknown
142073085: 31f6 xor esi, esi
142073087: 31db xor ebx, ebx
142073089: 89d8 mov eax, ebx
14207308b: 488d0c40 lea rcx, [rax + rax*2]
14207308f: 488b5560 mov rdx, qword ptr [rbp + 0x60]
142073093: 488d148a lea rdx, [rdx + rcx*4]
142073097: 48c1e104 shl rcx, 4
14207309b: 4c01e9 add rcx, r13
14207309e: 4801f0 add rax, rsi
1420730a1: 448b4118 mov r8d, dword ptr [rcx + 0x18]
1420730a5: 44894520 mov dword ptr [rbp + 0x20], r8d
1420730a9: 4c8b4110 mov r8, qword ptr [rcx + 0x10]
1420730ad: 4c894518 mov qword ptr [rbp + 0x18], r8
1420730b1: f30f10591c movss xmm3, dword ptr [rcx + 0x1c]
1420730b6: f30f104120 movss xmm0, dword ptr [rcx + 0x20]
1420730bb: f30f104924 movss xmm1, dword ptr [rcx + 0x24]
1420730c0: 0f54de andps xmm3, xmm6
1420730c3: 0f54c6 andps xmm0, xmm6
1420730c6: 0f54ce andps xmm1, xmm6
1420730c9: 0f28d0 movaps xmm2, xmm0
1420730cc: f30fc2d003 cmpunordss xmm2, xmm0
1420730d1: 0f28e2 movaps xmm4, xmm2
1420730d4: 0f54e1 andps xmm4, xmm1
1420730d7: f30f5fc8 maxss xmm1, xmm0
1420730db: 0f55d1 andnps xmm2, xmm1
1420730de: 0f56d4 orps xmm2, xmm4
1420730e1: 0f28c2 movaps xmm0, xmm2
1420730e4: f30f5fc3 maxss xmm0, xmm3
1420730e8: f30fc2db03 cmpunordss xmm3, xmm3
1420730ed: 0f28cb movaps xmm1, xmm3
1420730f0: 0f54ca andps xmm1, xmm2
1420730f3: 0f55d8 andnps xmm3, xmm0
1420730f6: 0f56d9 orps xmm3, xmm1
1420730f9: 0f10511c movups xmm2, xmmword ptr [rcx + 0x1c]
1420730fd: 0f104120 movups xmm0, xmmword ptr [rcx + 0x20]
142073101: 440f2811 movaps xmm10, xmmword ptr [rcx]
142073105: 0f284910 movaps xmm1, xmmword ptr [rcx + 0x10]
142073109: f3440f104108 movss xmm8, dword ptr [rcx + 8]
14207310f: 450f28d8 movaps xmm11, xmm8
142073113: f3450f58d8 addss xmm11, xmm8
142073118: f3450f59c3 mulss xmm8, xmm11
14207311d: 410f28ea movaps xmm5, xmm10
142073121: 410f58ea addps xmm5, xmm10
142073125: f30f16e5 movshdup xmm4, xmm5
142073129: f3410f59e2 mulss xmm4, xmm10
14207312e: 450f28ca movaps xmm9, xmm10
142073132: f3450f12e3 movsldup xmm12, xmm11
142073137: 450f59e2 mulps xmm12, xmm10
14207313b: 450fc6d2ff shufps xmm10, xmm10, 0xff
142073140: 440f59cd mulps xmm9, xmm5
142073144: 410f59ea mulps xmm5, xmm10
142073148: 450f28ea movaps xmm13, xmm10
14207314c: f3450f59eb mulss xmm13, xmm11
142073151: 0fc6ede1 shufps xmm5, xmm5, 0xe1
142073155: f3450f16f1 movshdup xmm14, xmm9
14207315a: 450f28d6 movaps xmm10, xmm14
14207315e: f3450f58d0 addss xmm10, xmm8
142073163: 440f28df movaps xmm11, xmm7
142073167: f3450f5cda subss xmm11, xmm10
14207316c: 440f28d4 movaps xmm10, xmm4
142073170: f3450f58d5 addss xmm10, xmm13
142073175: 450f14da unpcklps xmm11, xmm10
142073179: 450f28d4 movaps xmm10, xmm12
14207317d: 440f5cd5 subps xmm10, xmm5
142073181: 450f16da movlhps xmm11, xmm10
142073185: f3410f5ce5 subss xmm4, xmm13
14207318a: f3450f58c1 addss xmm8, xmm9
14207318f: 440f28ef movaps xmm13, xmm7
142073193: f3450f5ce8 subss xmm13, xmm8
142073198: 410f14e5 unpcklps xmm4, xmm13
14207319c: 410f58ec addps xmm5, xmm12
1420731a0: 0fc6e5d4 shufps xmm4, xmm5, 0xd4
1420731a4: 440fc6d501 shufps xmm10, xmm5, 1
1420731a9: f3450f58ce addss xmm9, xmm14
1420731ae: 440f28c7 movaps xmm8, xmm7
1420731b2: f3450f5cc1 subss xmm8, xmm9
1420731b7: 440fc6c530 shufps xmm8, xmm5, 0x30
1420731bc: 450fc6d082 shufps xmm10, xmm8, 0x82
1420731c1: 0fc6d200 shufps xmm2, xmm2, 0
1420731c5: 410f59d3 mulps xmm2, xmm11
1420731c9: 0f28e8 movaps xmm5, xmm0
1420731cc: 0fc6e800 shufps xmm5, xmm0, 0
1420731d0: 0f59ec mulps xmm5, xmm4
1420731d3: 0fc6c055 shufps xmm0, xmm0, 0x55
1420731d7: 410f59c2 mulps xmm0, xmm10
1420731db: 0fb600 movzx eax, byte ptr [rax]
1420731de: 488d0de306f600 lea rcx, [rip + 0xf606e3]
1420731e5: 8b0481 mov eax, dword ptr [rcx + rax*4]
1420731e8: 0f28e2 movaps xmm4, xmm2
1420731eb: f20f10e5 movsd xmm4, xmm5
1420731ef: 0fc6d424 shufps xmm2, xmm4, 0x24
1420731f3: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
1420731f7: 0fc6e849 shufps xmm5, xmm0, 0x49
1420731fb: 0f296dd0 movaps xmmword ptr [rbp - 0x30], xmm5
1420731ff: 0fc6c102 shufps xmm0, xmm1, 2
142073203: 0fc6c198 shufps xmm0, xmm1, 0x98
142073207: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
14207320b: 8945f0 mov dword ptr [rbp - 0x10], eax
14207320e: 48c745f400000000 mov qword ptr [rbp - 0xc], 0
142073216: c745fc00000000 mov dword ptr [rbp - 4], 0
14207321d: 488d45c0 lea rax, [rbp - 0x40]
142073221: 4889442420 mov qword ptr [rsp + 0x20], rax
142073226: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
14207322a: 4c8d4518 lea r8, [rbp + 0x18]
14207322e: e8cdc34fff call 0x14156f600 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>
142073233: 48ffc3 inc rbx
142073236: 807d5800 cmp byte ptr [rbp + 0x58], 0
14207323a: 0f841afcffff je 0x142072e5a
142073240: e94bfdffff jmp 0x142072f90 ; unknown
142073245: 488b7548 mov rsi, qword ptr [rbp + 0x48]
142073249: 4889f1 mov rcx, rsi
14207324c: e8bfebc0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
142073251: 4889f1 mov rcx, rsi
142073254: 4883c901 or rcx, 1
142073258: e8b3ebc0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207325d: 31c9 xor ecx, ecx
14207325f: b808000000 mov eax, 8
142073264: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
142073269: 0f85ea000000 jne 0x142073359
14207326f: 48837d3000 cmp qword ptr [rbp + 0x30], 0
142073274: 7442 je 0x1420732b8
142073276: e8b54ebafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207327b: 4889c1 mov rcx, rax
14207327e: 488b00 mov rax, qword ptr [rax]
142073281: 4883f801 cmp rax, 1
142073285: 0f8584000000 jne 0x14207330f
14207328b: 4883c108 add rcx, 8
14207328f: 48833900 cmp qword ptr [rcx], 0
142073293: 0f85b4000000 jne 0x14207334d
142073299: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1420732a0: 48894d58 mov qword ptr [rbp + 0x58], rcx
1420732a4: 4883c108 add rcx, 8
1420732a8: 488b5538 mov rdx, qword ptr [rbp + 0x38]
1420732ac: e83f3ebafe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1420732b1: 488b4558 mov rax, qword ptr [rbp + 0x58]
1420732b5: 48ff00 inc qword ptr [rax]
1420732b8: 0f287570 movaps xmm6, xmmword ptr [rbp + 0x70]
1420732bc: 0f28bd80000000 movaps xmm7, xmmword ptr [rbp + 0x80]
1420732c3: 440f288590000000 movaps xmm8, xmmword ptr [rbp + 0x90]
1420732cb: 440f288da0000000 movaps xmm9, xmmword ptr [rbp + 0xa0]
1420732d3: 440f2895b0000000 movaps xmm10, xmmword ptr [rbp + 0xb0]
1420732db: 440f289dc0000000 movaps xmm11, xmmword ptr [rbp + 0xc0]
1420732e3: 440f28a5d0000000 movaps xmm12, xmmword ptr [rbp + 0xd0]
1420732eb: 440f28ade0000000 movaps xmm13, xmmword ptr [rbp + 0xe0]
1420732f3: 440f28b5f0000000 movaps xmm14, xmmword ptr [rbp + 0xf0]
1420732fb: 4881c488010000 add rsp, 0x188
142073302: 5b pop rbx
142073303: 5f pop rdi
142073304: 5e pop rsi
142073305: 415c pop r12
142073307: 415d pop r13
142073309: 415e pop r14
14207330b: 415f pop r15
14207330d: 5d pop rbp
14207330e: c3 ret 
14207330f: 4885c0 test rax, rax
142073312: 752d jne 0x142073341
142073314: 31d2 xor edx, edx
142073316: e8b5548800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14207331b: 4889c1 mov rcx, rax
14207331e: 4885c0 test rax, rax
142073321: 0f8568ffffff jne 0x14207328f
142073327: eb18 jmp 0x142073341 ; unknown
142073329: 4885c0 test rax, rax
14207332c: 7513 jne 0x142073341
14207332e: 31d2 xor edx, edx
142073330: e89b548800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142073335: 4889c1 mov rcx, rax
142073338: 4885c0 test rax, rax
14207333b: 0f855bfaffff jne 0x142072d9c
142073341: 488d0d5877bc00 lea rcx, [rip + 0xbc7758]
142073348: e8834d8600 call 0x1428d80d0 ; std::thread::local::panic_access_error
14207334d: 488d0d1c78bc00 lea rcx, [rip + 0xbc781c]
142073354: e8f75e8600 call 0x1428d9250 ; core::cell::panic_already_borrowed
142073359: 4889f1 mov rcx, rsi
14207335c: 31d2 xor edx, edx
14207335e: e8fd8b8500 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
142073363: e907ffffff jmp 0x14207326f ; unknown
142073368: 0f1f840000000000 nop dword ptr [rax + rax]
142073370: 4889542410 mov qword ptr [rsp + 0x10], rdx
142073375: 55 push rbp
142073376: 4157 push r15
142073378: 4156 push r14
14207337a: 4155 push r13
14207337c: 4154 push r12
14207337e: 56 push rsi
14207337f: 57 push rdi
142073380: 53 push rbx
142073381: 4881ecc8000000 sub rsp, 0xc8
142073388: 488daa80000000 lea rbp, [rdx + 0x80]
14207338f: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
142073395: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
14207339b: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
1420733a1: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
1420733a7: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
1420733ad: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
1420733b6: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
1420733bf: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
1420733c7: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
1420733cf: 488b4558 mov rax, qword ptr [rbp + 0x58]
1420733d3: 48ff00 inc qword ptr [rax]
1420733d6: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
1420733de: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
1420733e6: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
1420733ef: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
1420733f8: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
1420733fe: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
142073404: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
14207340a: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
142073410: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
142073416: 4881c4c8000000 add rsp, 0xc8
14207341d: 5b pop rbx
14207341e: 5f pop rdi
14207341f: 5e pop rsi
142073420: 415c pop r12
142073422: 415d pop r13
142073424: 415e pop r14
142073426: 415f pop r15
142073428: 5d pop rbp
142073429: c3 ret 
14207342a: 660f1f440000 nop word ptr [rax + rax]
142073430: 4889542410 mov qword ptr [rsp + 0x10], rdx
142073435: 55 push rbp
142073436: 4157 push r15
142073438: 4156 push r14
14207343a: 4155 push r13
14207343c: 4154 push r12
14207343e: 56 push rsi
14207343f: 57 push rdi
142073440: 53 push rbx
142073441: 4881ecc8000000 sub rsp, 0xc8
142073448: 488daa80000000 lea rbp, [rdx + 0x80]
14207344f: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
142073455: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
14207345b: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
142073461: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
142073467: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
14207346d: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
142073476: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
14207347f: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
142073487: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
14207348f: 488b4558 mov rax, qword ptr [rbp + 0x58]
142073493: 48ff00 inc qword ptr [rax]
142073496: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
14207349e: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
1420734a6: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
1420734af: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
1420734b8: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
1420734be: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
1420734c4: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
1420734ca: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
1420734d0: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
1420734d6: 4881c4c8000000 add rsp, 0xc8
1420734dd: 5b pop rbx
1420734de: 5f pop rdi
1420734df: 5e pop rsi
1420734e0: 415c pop r12
1420734e2: 415d pop r13
1420734e4: 415e pop r14
1420734e6: 415f pop r15
1420734e8: 5d pop rbp
1420734e9: c3 ret 
1420734ea: 660f1f440000 nop word ptr [rax + rax]
1420734f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420734f5: 55 push rbp
1420734f6: 4157 push r15
1420734f8: 4156 push r14
1420734fa: 4155 push r13
1420734fc: 4154 push r12
1420734fe: 56 push rsi
1420734ff: 57 push rdi
142073500: 53 push rbx
142073501: 4881ecc8000000 sub rsp, 0xc8
142073508: 488daa80000000 lea rbp, [rdx + 0x80]
14207350f: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
142073515: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
14207351b: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
142073521: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
142073527: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
14207352d: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
142073536: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
14207353f: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
142073547: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
14207354f: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
142073553: e878ddfcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
142073558: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
142073560: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
142073568: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
142073571: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
14207357a: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
142073580: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
142073586: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
14207358c: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
142073592: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
142073598: 4881c4c8000000 add rsp, 0xc8
14207359f: 5b pop rbx
1420735a0: 5f pop rdi
1420735a1: 5e pop rsi
1420735a2: 415c pop r12
1420735a4: 415d pop r13
1420735a6: 415e pop r14
1420735a8: 415f pop r15
1420735aa: 5d pop rbp
1420735ab: c3 ret 
1420735ac: 0f1f4000 nop dword ptr [rax]
1420735b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420735b5: 55 push rbp
1420735b6: 4157 push r15
1420735b8: 4156 push r14
1420735ba: 4155 push r13
1420735bc: 4154 push r12
1420735be: 56 push rsi
1420735bf: 57 push rdi
1420735c0: 53 push rbx
1420735c1: 4881ecc8000000 sub rsp, 0xc8
1420735c8: 488daa80000000 lea rbp, [rdx + 0x80]
1420735cf: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
1420735d5: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
1420735db: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
1420735e1: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
1420735e7: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
1420735ed: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
1420735f6: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
1420735ff: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
142073607: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
14207360f: 48837d3000 cmp qword ptr [rbp + 0x30], 0
142073614: 7420 je 0x142073636
142073616: 488d4538 lea rax, [rbp + 0x38]
14207361a: 48894510 mov qword ptr [rbp + 0x10], rax
14207361e: 488d4510 lea rax, [rbp + 0x10]
142073622: 48894508 mov qword ptr [rbp + 8], rax
142073626: 488d0d2bfdf500 lea rcx, [rip + 0xf5fd2b]
14207362d: 488d5508 lea rdx, [rbp + 8]
142073631: e89abc2bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
142073636: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
14207363e: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
142073646: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
14207364f: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
142073658: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
14207365e: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
142073664: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
14207366a: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
142073670: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
142073676: 4881c4c8000000 add rsp, 0xc8
14207367d: 5b pop rbx
14207367e: 5f pop rdi
14207367f: 5e pop rsi
142073680: 415c pop r12
142073682: 415d pop r13
142073684: 415e pop r14
142073686: 415f pop r15
142073688: 5d pop rbp
142073689: c3 ret 
