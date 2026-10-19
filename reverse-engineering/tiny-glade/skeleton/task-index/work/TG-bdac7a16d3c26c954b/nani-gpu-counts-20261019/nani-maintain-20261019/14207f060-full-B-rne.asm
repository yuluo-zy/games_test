14207f060: 55 push rbp
14207f061: 4157 push r15
14207f063: 4156 push r14
14207f065: 4155 push r13
14207f067: 4154 push r12
14207f069: 56 push rsi
14207f06a: 57 push rdi
14207f06b: 53 push rbx
14207f06c: 4881eca8010000 sub rsp, 0x1a8
14207f073: 488dac2480000000 lea rbp, [rsp + 0x80]
14207f07b: 440f29b510010000 movaps xmmword ptr [rbp + 0x110], xmm14
14207f083: 440f29ad00010000 movaps xmmword ptr [rbp + 0x100], xmm13
14207f08b: 440f29a5f0000000 movaps xmmword ptr [rbp + 0xf0], xmm12
14207f093: 440f299de0000000 movaps xmmword ptr [rbp + 0xe0], xmm11
14207f09b: 440f2995d0000000 movaps xmmword ptr [rbp + 0xd0], xmm10
14207f0a3: 440f298dc0000000 movaps xmmword ptr [rbp + 0xc0], xmm9
14207f0ab: 440f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm8
14207f0b3: 0f29bda0000000 movaps xmmword ptr [rbp + 0xa0], xmm7
14207f0ba: 0f29b590000000 movaps xmmword ptr [rbp + 0x90], xmm6
14207f0c1: 48c78588000000feffffff mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14207f0cc: 4889d6 mov rsi, rdx
14207f0cf: 4889cf mov rdi, rcx
14207f0d2: e82979b9fe call 0x140c16a00 ; puffin::are_scopes_on
14207f0d7: 84c0 test al, al
14207f0d9: 745f je 0x14207f13a
14207f0db: 4c8d355f3ef500 lea r14, [rip + 0xf53e5f]
14207f0e2: bb3e000000 mov ebx, 0x3e
14207f0e7: ba3e000000 mov edx, 0x3e
14207f0ec: 4c89f1 mov rcx, r14
14207f0ef: e87cfdfeff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207f0f4: 4883f801 cmp rax, 1
14207f0f8: 0f85b1000000 jne 0x14207f1af
14207f0fe: 4989d1 mov r9, rdx
14207f101: 4885d2 test rdx, rdx
14207f104: 743d je 0x14207f143
14207f106: 4983f93e cmp r9, 0x3e
14207f10a: 7335 jae 0x14207f141
14207f10c: 488d052e3ef500 lea rax, [rip + 0xf53e2e]
14207f113: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
14207f118: 7f29 jg 0x14207f143
14207f11a: 488d05b742f500 lea rax, [rip + 0xf542b7]
14207f121: 4889442420 mov qword ptr [rsp + 0x20], rax
14207f126: 488d0d143ef500 lea rcx, [rip + 0xf53e14]
14207f12d: ba3e000000 mov edx, 0x3e
14207f132: 4531c0 xor r8d, r8d
14207f135: e806a88500 call 0x1428d9940 ; core::str::slice_error_fail
14207f13a: 31c0 xor eax, eax
14207f13c: e9b9010000 jmp 0x14207f2fa ; unknown
14207f141: 75d7 jne 0x14207f11a
14207f143: 4c8d35f73df500 lea r14, [rip + 0xf53df7]
14207f14a: 4c89f1 mov rcx, r14
14207f14d: 4c89ca mov rdx, r9
14207f150: e81bfdfeff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207f155: 4883f801 cmp rax, 1
14207f159: 7554 jne 0x14207f1af
14207f15b: 4989d0 mov r8, rdx
14207f15e: 4983c002 add r8, 2
14207f162: 7439 je 0x14207f19d
14207f164: 4983f83e cmp r8, 0x3e
14207f168: 7331 jae 0x14207f19b
14207f16a: 488d05d03df500 lea rax, [rip + 0xf53dd0]
14207f171: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
14207f176: 7f25 jg 0x14207f19d
14207f178: 488d057142f500 lea rax, [rip + 0xf54271]
14207f17f: 4889442420 mov qword ptr [rsp + 0x20], rax
14207f184: 488d0db63df500 lea rcx, [rip + 0xf53db6]
14207f18b: ba3e000000 mov edx, 0x3e
14207f190: 41b93e000000 mov r9d, 0x3e
14207f196: e8a5a78500 call 0x1428d9940 ; core::str::slice_error_fail
14207f19b: 75db jne 0x14207f178
14207f19d: bb3c000000 mov ebx, 0x3c
14207f1a2: 4829d3 sub rbx, rdx
14207f1a5: 4c8d35953df500 lea r14, [rip + 0xf53d95]
14207f1ac: 4d01c6 add r14, r8
14207f1af: 4c8d05a842f500 lea r8, [rip + 0xf542a8]
14207f1b6: 4c8d3d7342f500 lea r15, [rip + 0xf54273]
14207f1bd: 0f1f00 nop dword ptr [rax]
14207f1c0: 410fbe40ff movsx eax, byte ptr [r8 - 1]
14207f1c5: 85c0 test eax, eax
14207f1c7: 7817 js 0x14207f1e0
14207f1c9: 49ffc8 dec r8
14207f1cc: 83f85c cmp eax, 0x5c
14207f1cf: 7563 jne 0x14207f234
14207f1d1: eb73 jmp 0x14207f246 ; unknown
14207f1d3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207f1e0: 410fb648fe movzx ecx, byte ptr [r8 - 2]
14207f1e5: 80f9c0 cmp cl, 0xc0
14207f1e8: 7d23 jge 0x14207f20d
14207f1ea: 410fb650fd movzx edx, byte ptr [r8 - 3]
14207f1ef: 80fac0 cmp dl, 0xc0
14207f1f2: 7d22 jge 0x14207f216
14207f1f4: 450fb648fc movzx r9d, byte ptr [r8 - 4]
14207f1f9: 4983c0fc add r8, -4
14207f1fd: 4183e107 and r9d, 7
14207f201: 41c1e106 shl r9d, 6
14207f205: 83e23f and edx, 0x3f
14207f208: 4409ca or edx, r9d
14207f20b: eb10 jmp 0x14207f21d ; unknown
14207f20d: 4983c0fe add r8, -2
14207f211: 83e11f and ecx, 0x1f
14207f214: eb0f jmp 0x14207f225 ; unknown
14207f216: 4983c0fd add r8, -3
14207f21a: 83e20f and edx, 0xf
14207f21d: c1e206 shl edx, 6
14207f220: 83e13f and ecx, 0x3f
14207f223: 09d1 or ecx, edx
14207f225: c1e106 shl ecx, 6
14207f228: 243f and al, 0x3f
14207f22a: 0fb6c0 movzx eax, al
14207f22d: 09c8 or eax, ecx
14207f22f: 83f85c cmp eax, 0x5c
14207f232: 7412 je 0x14207f246
14207f234: 83f82f cmp eax, 0x2f
14207f237: 740d je 0x14207f246
14207f239: 4d39f8 cmp r8, r15
14207f23c: 7582 jne 0x14207f1c0
14207f23e: 41bc2e000000 mov r12d, 0x2e
14207f244: eb4d jmp 0x14207f293 ; unknown
14207f246: 4c8d3de341f500 lea r15, [rip + 0xf541e3]
14207f24d: 4d29f8 sub r8, r15
14207f250: 49ffc0 inc r8
14207f253: 7432 je 0x14207f287
14207f255: 4983f82e cmp r8, 0x2e
14207f259: 732a jae 0x14207f285
14207f25b: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
14207f260: 7f25 jg 0x14207f287
14207f262: 488d054f41f500 lea rax, [rip + 0xf5414f]
14207f269: 4889442420 mov qword ptr [rsp + 0x20], rax
14207f26e: 488d0dbb41f500 lea rcx, [rip + 0xf541bb]
14207f275: ba2e000000 mov edx, 0x2e
14207f27a: 41b92e000000 mov r9d, 0x2e
14207f280: e8bba68500 call 0x1428d9940 ; core::str::slice_error_fail
14207f285: 75db jne 0x14207f262
14207f287: 41bc2e000000 mov r12d, 0x2e
14207f28d: 4d29c4 sub r12, r8
14207f290: 4d01c7 add r15, r8
14207f293: e8988eb9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207f298: 4889c1 mov rcx, rax
14207f29b: 488b00 mov rax, qword ptr [rax]
14207f29e: 4883f801 cmp rax, 1
14207f2a2: 0f85b0060000 jne 0x14207f958
14207f2a8: 4883c108 add rcx, 8
14207f2ac: 48833900 cmp qword ptr [rcx], 0
14207f2b0: 0f85c6060000 jne 0x14207f97c
14207f2b6: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207f2bd: 48894d70 mov qword ptr [rbp + 0x70], rcx
14207f2c1: 4883c108 add rcx, 8
14207f2c5: 4c89642420 mov qword ptr [rsp + 0x20], r12
14207f2ca: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
14207f2d3: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14207f2dc: 4c89f2 mov rdx, r14
14207f2df: 4989d8 mov r8, rbx
14207f2e2: 4d89f9 mov r9, r15
14207f2e5: e8567ab9fe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
14207f2ea: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
14207f2ee: 48ff01 inc qword ptr [rcx]
14207f2f1: 48894558 mov qword ptr [rbp + 0x58], rax
14207f2f5: b801000000 mov eax, 1
14207f2fa: 48894550 mov qword ptr [rbp + 0x50], rax
14207f2fe: 488b0e mov rcx, qword ptr [rsi]
14207f301: b219 mov dl, 0x19
14207f303: e8482991fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
14207f308: 48895548 mov qword ptr [rbp + 0x48], rdx
14207f30c: 48894568 mov qword ptr [rbp + 0x68], rax
14207f310: 488b1f mov rbx, qword ptr [rdi]
14207f313: 4c8b7708 mov r14, qword ptr [rdi + 8]
14207f317: 4c8bbbf8000000 mov r15, qword ptr [rbx + 0xf8]
14207f31e: 488b8300010000 mov rax, qword ptr [rbx + 0x100]
14207f325: 4d8d2487 lea r12, [r15 + rax*4]
14207f329: 0fb68308010000 movzx eax, byte ptr [rbx + 0x108]
14207f330: 884570 mov byte ptr [rbp + 0x70], al
14207f333: b808000000 mov eax, 8
14207f338: 48894560 mov qword ptr [rbp + 0x60], rax
14207f33c: 48c7457800000000 mov qword ptr [rbp + 0x78], 0
14207f344: 0f283535f68a00 movaps xmm6, xmmword ptr [rip + 0x8af635]
14207f34b: f30f103db1658a00 movss xmm7, dword ptr [rip + 0x8a65b1]
14207f353: 48c7858000000000000000 mov qword ptr [rbp + 0x80], 0
14207f35e: 31f6 xor esi, esi
14207f360: 31ff xor edi, edi
14207f362: 4531ed xor r13d, r13d
14207f365: 807d7000 cmp byte ptr [rbp + 0x70], 0
14207f369: 0f8541010000 jne 0x14207f4b0
14207f36f: 4939fd cmp r13, rdi
14207f372: 0f851e010000 jne 0x14207f496
14207f378: 0f1f840000000000 nop dword ptr [rax + rax]
14207f380: 4d39e7 cmp r15, r12
14207f383: 0f84e8040000 je 0x14207f871
14207f389: 418b0f mov ecx, dword ptr [r15]
14207f38c: 4983c704 add r15, 4
14207f390: 498b8600010000 mov rax, qword ptr [r14 + 0x100]
14207f397: 488d0c89 lea rcx, [rcx + rcx*4]
14207f39b: 48c1e105 shl rcx, 5
14207f39f: 488b7c0858 mov rdi, qword ptr [rax + rcx + 0x58]
14207f3a4: 4885ff test rdi, rdi
14207f3a7: 74d7 je 0x14207f380
14207f3a9: 4801c8 add rax, rcx
14207f3ac: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
14207f3b2: 498b96a8010000 mov rdx, qword ptr [r14 + 0x1a8]
14207f3b9: 4c8d0cc9 lea r9, [rcx + rcx*8]
14207f3bd: 4a8d0cca lea rcx, [rdx + r9*8]
14207f3c1: 4c8b8310010000 mov r8, qword ptr [rbx + 0x110]
14207f3c8: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
14207f3cd: 4c39c2 cmp rdx, r8
14207f3d0: 7633 jbe 0x14207f405
14207f3d2: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207f3d6: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207f3da: 4d85c0 test r8, r8
14207f3dd: 7426 je 0x14207f405
14207f3df: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207f3e3: 49f7d0 not r8
14207f3e6: 4f8d0440 lea r8, [r8 + r8*2]
14207f3ea: 49c1e004 shl r8, 4
14207f3ee: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14207f3f3: 4c894578 mov qword ptr [rbp + 0x78], r8
14207f3f7: 4c8b8318010000 mov r8, qword ptr [rbx + 0x118]
14207f3fe: 4c39c2 cmp rdx, r8
14207f401: 7716 ja 0x14207f419
14207f403: eb4a jmp 0x14207f44f ; unknown
14207f405: 48c7457800000000 mov qword ptr [rbp + 0x78], 0
14207f40d: 4c8b8318010000 mov r8, qword ptr [rbx + 0x118]
14207f414: 4c39c2 cmp rdx, r8
14207f417: 7636 jbe 0x14207f44f
14207f419: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207f41d: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207f421: 4d85c0 test r8, r8
14207f424: 7429 je 0x14207f44f
14207f426: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207f42a: 49f7d0 not r8
14207f42d: 4f8d0440 lea r8, [r8 + r8*2]
14207f431: 49c1e004 shl r8, 4
14207f435: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14207f43a: 4c898580000000 mov qword ptr [rbp + 0x80], r8
14207f441: 4c8b8320010000 mov r8, qword ptr [rbx + 0x120]
14207f448: 4c39c2 cmp rdx, r8
14207f44b: 763c jbe 0x14207f489
14207f44d: eb17 jmp 0x14207f466 ; unknown
14207f44f: 48c7858000000000000000 mov qword ptr [rbp + 0x80], 0
14207f45a: 4c8b8320010000 mov r8, qword ptr [rbx + 0x120]
14207f461: 4c39c2 cmp rdx, r8
14207f464: 7623 jbe 0x14207f489
14207f466: 488b5138 mov rdx, qword ptr [rcx + 0x38]
14207f46a: 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
14207f46e: 4885d2 test rdx, rdx
14207f471: 7416 je 0x14207f489
14207f473: 488b4918 mov rcx, qword ptr [rcx + 0x18]
14207f477: 48f7d2 not rdx
14207f47a: 488d1452 lea rdx, [rdx + rdx*2]
14207f47e: 48c1e204 shl rdx, 4
14207f482: 488b741110 mov rsi, qword ptr [rcx + rdx + 0x10]
14207f487: eb02 jmp 0x14207f48b ; unknown
14207f489: 31f6 xor esi, esi
14207f48b: 488b4050 mov rax, qword ptr [rax + 0x50]
14207f48f: 48894560 mov qword ptr [rbp + 0x60], rax
14207f493: 4531ed xor r13d, r13d
14207f496: 4c89e8 mov rax, r13
14207f499: 48c1e004 shl rax, 4
14207f49d: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
14207f4a1: 8b440108 mov eax, dword ptr [rcx + rax + 8]
14207f4a5: e90e010000 jmp 0x14207f5b8 ; unknown
14207f4aa: 660f1f440000 nop word ptr [rax + rax]
14207f4b0: 4939fd cmp r13, rdi
14207f4b3: 0f85fc000000 jne 0x14207f5b5
14207f4b9: 0f1f8000000000 nop dword ptr [rax]
14207f4c0: 4d39e7 cmp r15, r12
14207f4c3: 0f84a8030000 je 0x14207f871
14207f4c9: 418b0f mov ecx, dword ptr [r15]
14207f4cc: 4983c704 add r15, 4
14207f4d0: 498b86a8010000 mov rax, qword ptr [r14 + 0x1a8]
14207f4d7: 488d0cc9 lea rcx, [rcx + rcx*8]
14207f4db: 488b7cc810 mov rdi, qword ptr [rax + rcx*8 + 0x10]
14207f4e0: 4885ff test rdi, rdi
14207f4e3: 74db je 0x14207f4c0
14207f4e5: 488d04c8 lea rax, [rax + rcx*8]
14207f4e9: 488b9310010000 mov rdx, qword ptr [rbx + 0x110]
14207f4f0: 488b4840 mov rcx, qword ptr [rax + 0x40]
14207f4f4: 4839d1 cmp rcx, rdx
14207f4f7: 7633 jbe 0x14207f52c
14207f4f9: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207f4fd: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207f501: 4885d2 test rdx, rdx
14207f504: 7426 je 0x14207f52c
14207f506: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14207f50a: 48f7d2 not rdx
14207f50d: 488d1452 lea rdx, [rdx + rdx*2]
14207f511: 48c1e204 shl rdx, 4
14207f515: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
14207f51a: 48895578 mov qword ptr [rbp + 0x78], rdx
14207f51e: 488b9318010000 mov rdx, qword ptr [rbx + 0x118]
14207f525: 4839d1 cmp rcx, rdx
14207f528: 7716 ja 0x14207f540
14207f52a: eb4a jmp 0x14207f576 ; unknown
14207f52c: 48c7457800000000 mov qword ptr [rbp + 0x78], 0
14207f534: 488b9318010000 mov rdx, qword ptr [rbx + 0x118]
14207f53b: 4839d1 cmp rcx, rdx
14207f53e: 7636 jbe 0x14207f576
14207f540: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207f544: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207f548: 4885d2 test rdx, rdx
14207f54b: 7429 je 0x14207f576
14207f54d: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14207f551: 48f7d2 not rdx
14207f554: 488d1452 lea rdx, [rdx + rdx*2]
14207f558: 48c1e204 shl rdx, 4
14207f55c: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
14207f561: 48899580000000 mov qword ptr [rbp + 0x80], rdx
14207f568: 488b9320010000 mov rdx, qword ptr [rbx + 0x120]
14207f56f: 4839d1 cmp rcx, rdx
14207f572: 763c jbe 0x14207f5b0
14207f574: eb17 jmp 0x14207f58d ; unknown
14207f576: 48c7858000000000000000 mov qword ptr [rbp + 0x80], 0
14207f581: 488b9320010000 mov rdx, qword ptr [rbx + 0x120]
14207f588: 4839d1 cmp rcx, rdx
14207f58b: 7623 jbe 0x14207f5b0
14207f58d: 488b4838 mov rcx, qword ptr [rax + 0x38]
14207f591: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
14207f595: 4885c9 test rcx, rcx
14207f598: 7416 je 0x14207f5b0
14207f59a: 488b4018 mov rax, qword ptr [rax + 0x18]
14207f59e: 48f7d1 not rcx
14207f5a1: 488d0c49 lea rcx, [rcx + rcx*2]
14207f5a5: 48c1e104 shl rcx, 4
14207f5a9: 488b740810 mov rsi, qword ptr [rax + rcx + 0x10]
14207f5ae: eb02 jmp 0x14207f5b2 ; unknown
14207f5b0: 31f6 xor esi, esi
14207f5b2: 4531ed xor r13d, r13d
14207f5b5: 4489e8 mov eax, r13d
14207f5b8: 488d0c40 lea rcx, [rax + rax*2]
14207f5bc: 488b5578 mov rdx, qword ptr [rbp + 0x78]
14207f5c0: 488d148a lea rdx, [rdx + rcx*4]
14207f5c4: 48c1e004 shl rax, 4
14207f5c8: 488d0440 lea rax, [rax + rax*2]
14207f5cc: 488b8d80000000 mov rcx, qword ptr [rbp + 0x80]
14207f5d3: 4801c1 add rcx, rax
14207f5d6: 4801f0 add rax, rsi
14207f5d9: 448b4118 mov r8d, dword ptr [rcx + 0x18]
14207f5dd: 44894540 mov dword ptr [rbp + 0x40], r8d
14207f5e1: 4c8b4110 mov r8, qword ptr [rcx + 0x10]
14207f5e5: 4c894538 mov qword ptr [rbp + 0x38], r8
14207f5e9: f30f10591c movss xmm3, dword ptr [rcx + 0x1c]
14207f5ee: f30f104120 movss xmm0, dword ptr [rcx + 0x20]
14207f5f3: f30f104924 movss xmm1, dword ptr [rcx + 0x24]
14207f5f8: 0f54de andps xmm3, xmm6
14207f5fb: 0f54c6 andps xmm0, xmm6
14207f5fe: 0f54ce andps xmm1, xmm6
14207f601: 0f28d0 movaps xmm2, xmm0
14207f604: f30fc2d003 cmpunordss xmm2, xmm0
14207f609: 0f28e2 movaps xmm4, xmm2
14207f60c: 0f54e1 andps xmm4, xmm1
14207f60f: f30f5fc8 maxss xmm1, xmm0
14207f613: 0f55d1 andnps xmm2, xmm1
14207f616: 0f56d4 orps xmm2, xmm4
14207f619: 0f28c2 movaps xmm0, xmm2
14207f61c: f30f5fc3 maxss xmm0, xmm3
14207f620: f30fc2db03 cmpunordss xmm3, xmm3
14207f625: 0f28cb movaps xmm1, xmm3
14207f628: 0f54ca andps xmm1, xmm2
14207f62b: 0f55d8 andnps xmm3, xmm0
14207f62e: 0f56d9 orps xmm3, xmm1
14207f631: 0f10491c movups xmm1, xmmword ptr [rcx + 0x1c]
14207f635: 0f104120 movups xmm0, xmmword ptr [rcx + 0x20]
14207f639: 440f2809 movaps xmm9, xmmword ptr [rcx]
14207f63d: f30f106908 movss xmm5, dword ptr [rcx + 8]
14207f642: 440f28d5 movaps xmm10, xmm5
14207f646: f3440f58d5 addss xmm10, xmm5
14207f64b: f3410f59ea mulss xmm5, xmm10
14207f650: 410f28d1 movaps xmm2, xmm9
14207f654: 410f58d1 addps xmm2, xmm9
14207f658: f30f16e2 movshdup xmm4, xmm2
14207f65c: f3410f59e1 mulss xmm4, xmm9
14207f661: 450f28c1 movaps xmm8, xmm9
14207f665: f3450f12da movsldup xmm11, xmm10
14207f66a: 450f59d9 mulps xmm11, xmm9
14207f66e: 450fc6c9ff shufps xmm9, xmm9, 0xff
14207f673: 440f59c2 mulps xmm8, xmm2
14207f677: 410f59d1 mulps xmm2, xmm9
14207f67b: 450f28e1 movaps xmm12, xmm9
14207f67f: f3450f59e2 mulss xmm12, xmm10
14207f684: 0fc6d2e1 shufps xmm2, xmm2, 0xe1
14207f688: f3450f16e8 movshdup xmm13, xmm8
14207f68d: 450f28cd movaps xmm9, xmm13
14207f691: f3440f58cd addss xmm9, xmm5
14207f696: 440f28d7 movaps xmm10, xmm7
14207f69a: f3450f5cd1 subss xmm10, xmm9
14207f69f: 440f28cc movaps xmm9, xmm4
14207f6a3: f3450f58cc addss xmm9, xmm12
14207f6a8: 450f14d1 unpcklps xmm10, xmm9
14207f6ac: 450f28cb movaps xmm9, xmm11
14207f6b0: 440f5cca subps xmm9, xmm2
14207f6b4: 450f16d1 movlhps xmm10, xmm9
14207f6b8: f3410f5ce4 subss xmm4, xmm12
14207f6bd: f3410f58e8 addss xmm5, xmm8
14207f6c2: 440f28e7 movaps xmm12, xmm7
14207f6c6: f3440f5ce5 subss xmm12, xmm5
14207f6cb: 410f14e4 unpcklps xmm4, xmm12
14207f6cf: 410f58d3 addps xmm2, xmm11
14207f6d3: 0fc6e2d4 shufps xmm4, xmm2, 0xd4
14207f6d7: 440fc6ca01 shufps xmm9, xmm2, 1
14207f6dc: f3450f58c5 addss xmm8, xmm13
14207f6e1: 0f28ef movaps xmm5, xmm7
14207f6e4: f3410f5ce8 subss xmm5, xmm8
14207f6e9: 0fc6ea30 shufps xmm5, xmm2, 0x30
14207f6ed: 440fc6cd82 shufps xmm9, xmm5, 0x82
14207f6f2: 0fc6c900 shufps xmm1, xmm1, 0
14207f6f6: 410f59ca mulps xmm1, xmm10
14207f6fa: 0f28d0 movaps xmm2, xmm0
14207f6fd: 0fc6d000 shufps xmm2, xmm0, 0
14207f701: 0f59d4 mulps xmm2, xmm4
14207f704: 0fc6c055 shufps xmm0, xmm0, 0x55
14207f708: 410f59c1 mulps xmm0, xmm9
14207f70c: 440f2810 movaps xmm10, xmmword ptr [rax]
14207f710: f3440f104808 movss xmm9, dword ptr [rax + 8]
14207f716: 450f28e1 movaps xmm12, xmm9
14207f71a: f3450f58e1 addss xmm12, xmm9
14207f71f: f3450f59cc mulss xmm9, xmm12
14207f724: 410f28ea movaps xmm5, xmm10
14207f728: 410f58ea addps xmm5, xmm10
14207f72c: f30f16e5 movshdup xmm4, xmm5
14207f730: f3410f59e2 mulss xmm4, xmm10
14207f735: 450f28c2 movaps xmm8, xmm10
14207f739: f3450f12dc movsldup xmm11, xmm12
14207f73e: 450f59da mulps xmm11, xmm10
14207f742: 450fc6d2ff shufps xmm10, xmm10, 0xff
14207f747: 440f59c5 mulps xmm8, xmm5
14207f74b: 410f59ea mulps xmm5, xmm10
14207f74f: 450f28ea movaps xmm13, xmm10
14207f753: f3450f59ec mulss xmm13, xmm12
14207f758: f3450f16e0 movshdup xmm12, xmm8
14207f75d: 450f28f4 movaps xmm14, xmm12
14207f761: f3450f58f1 addss xmm14, xmm9
14207f766: 440f28d7 movaps xmm10, xmm7
14207f76a: f3450f5cd6 subss xmm10, xmm14
14207f76f: 440f28f4 movaps xmm14, xmm4
14207f773: f3450f58f5 addss xmm14, xmm13
14207f778: 450f14d6 unpcklps xmm10, xmm14
14207f77c: f3410f5ce5 subss xmm4, xmm13
14207f781: f3450f58c8 addss xmm9, xmm8
14207f786: 440f28ef movaps xmm13, xmm7
14207f78a: f3450f5ce9 subss xmm13, xmm9
14207f78f: 0fc6ede1 shufps xmm5, xmm5, 0xe1
14207f793: 410f14e5 unpcklps xmm4, xmm13
14207f797: 450f28cb movaps xmm9, xmm11
14207f79b: 440f5ccd subps xmm9, xmm5
14207f79f: 410f58eb addps xmm5, xmm11
14207f7a3: f3450f58c4 addss xmm8, xmm12
14207f7a8: 440f28df movaps xmm11, xmm7
14207f7ac: f3450f5cd8 subss xmm11, xmm8
14207f7b1: 450f16d1 movlhps xmm10, xmm9
14207f7b5: 0fc6e5d4 shufps xmm4, xmm5, 0xd4
14207f7b9: 440fc6cd01 shufps xmm9, xmm5, 1
14207f7be: 440fc6dd30 shufps xmm11, xmm5, 0x30
14207f7c3: 440f10401c movups xmm8, xmmword ptr [rax + 0x1c]
14207f7c8: 450fc6cb82 shufps xmm9, xmm11, 0x82
14207f7cd: 0f106820 movups xmm5, xmmword ptr [rax + 0x20]
14207f7d1: 450fc6c000 shufps xmm8, xmm8, 0
14207f7d6: 450f59c2 mulps xmm8, xmm10
14207f7da: 440f28d5 movaps xmm10, xmm5
14207f7de: 440fc6d500 shufps xmm10, xmm5, 0
14207f7e3: 440f59d4 mulps xmm10, xmm4
14207f7e7: 0f286110 movaps xmm4, xmmword ptr [rcx + 0x10]
14207f7eb: 0fc6ed55 shufps xmm5, xmm5, 0x55
14207f7ef: 410f59e9 mulps xmm5, xmm9
14207f7f3: 440f28c9 movaps xmm9, xmm1
14207f7f7: f2440f10ca movsd xmm9, xmm2
14207f7fc: 410fc6c924 shufps xmm1, xmm9, 0x24
14207f801: 440f284810 movaps xmm9, xmmword ptr [rax + 0x10]
14207f806: 0f294dc0 movaps xmmword ptr [rbp - 0x40], xmm1
14207f80a: 0fc6d049 shufps xmm2, xmm0, 0x49
14207f80e: 0f2955d0 movaps xmmword ptr [rbp - 0x30], xmm2
14207f812: 0fc6c402 shufps xmm0, xmm4, 2
14207f816: 0fc6c498 shufps xmm0, xmm4, 0x98
14207f81a: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
14207f81e: 410f28c0 movaps xmm0, xmm8
14207f822: f2410f10c2 movsd xmm0, xmm10
14207f827: 440fc6c024 shufps xmm8, xmm0, 0x24
14207f82c: 440f2945f0 movaps xmmword ptr [rbp - 0x10], xmm8
14207f831: 440fc6d549 shufps xmm10, xmm5, 0x49
14207f836: 440f295500 movaps xmmword ptr [rbp], xmm10
14207f83b: 410fc6e902 shufps xmm5, xmm9, 2
14207f840: 410fc6e998 shufps xmm5, xmm9, 0x98
14207f845: 0f296d10 movaps xmmword ptr [rbp + 0x10], xmm5
14207f849: 488d45c0 lea rax, [rbp - 0x40]
14207f84d: 4889442420 mov qword ptr [rsp + 0x20], rax
14207f852: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
14207f856: 4c8d4538 lea r8, [rbp + 0x38]
14207f85a: e831fa4eff call 0x14156f290 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::VelocityOnlyInstanceData>
14207f85f: 49ffc5 inc r13
14207f862: 807d7000 cmp byte ptr [rbp + 0x70], 0
14207f866: 0f8403fbffff je 0x14207f36f
14207f86c: e93ffcffff jmp 0x14207f4b0 ; unknown
14207f871: 488b7568 mov rsi, qword ptr [rbp + 0x68]
14207f875: 4889f1 mov rcx, rsi
14207f878: e89325c0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207f87d: 4889f1 mov rcx, rsi
14207f880: 4883c901 or rcx, 1
14207f884: e88725c0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207f889: 31c9 xor ecx, ecx
14207f88b: b808000000 mov eax, 8
14207f890: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
14207f895: 0f85ed000000 jne 0x14207f988
14207f89b: 48837d5000 cmp qword ptr [rbp + 0x50], 0
14207f8a0: 7442 je 0x14207f8e4
14207f8a2: e88988b9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207f8a7: 4889c1 mov rcx, rax
14207f8aa: 488b00 mov rax, qword ptr [rax]
14207f8ad: 4883f801 cmp rax, 1
14207f8b1: 0f8587000000 jne 0x14207f93e
14207f8b7: 4883c108 add rcx, 8
14207f8bb: 48833900 cmp qword ptr [rcx], 0
14207f8bf: 0f85b7000000 jne 0x14207f97c
14207f8c5: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207f8cc: 48894d70 mov qword ptr [rbp + 0x70], rcx
14207f8d0: 4883c108 add rcx, 8
14207f8d4: 488b5558 mov rdx, qword ptr [rbp + 0x58]
14207f8d8: e81378b9fe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
14207f8dd: 488b4570 mov rax, qword ptr [rbp + 0x70]
14207f8e1: 48ff00 inc qword ptr [rax]
14207f8e4: 0f28b590000000 movaps xmm6, xmmword ptr [rbp + 0x90]
14207f8eb: 0f28bda0000000 movaps xmm7, xmmword ptr [rbp + 0xa0]
14207f8f2: 440f2885b0000000 movaps xmm8, xmmword ptr [rbp + 0xb0]
14207f8fa: 440f288dc0000000 movaps xmm9, xmmword ptr [rbp + 0xc0]
14207f902: 440f2895d0000000 movaps xmm10, xmmword ptr [rbp + 0xd0]
14207f90a: 440f289de0000000 movaps xmm11, xmmword ptr [rbp + 0xe0]
14207f912: 440f28a5f0000000 movaps xmm12, xmmword ptr [rbp + 0xf0]
14207f91a: 440f28ad00010000 movaps xmm13, xmmword ptr [rbp + 0x100]
14207f922: 440f28b510010000 movaps xmm14, xmmword ptr [rbp + 0x110]
14207f92a: 4881c4a8010000 add rsp, 0x1a8
14207f931: 5b pop rbx
14207f932: 5f pop rdi
14207f933: 5e pop rsi
14207f934: 415c pop r12
14207f936: 415d pop r13
14207f938: 415e pop r14
14207f93a: 415f pop r15
14207f93c: 5d pop rbp
14207f93d: c3 ret 
14207f93e: 4885c0 test rax, rax
14207f941: 752d jne 0x14207f970
14207f943: 31d2 xor edx, edx
14207f945: e8868e8700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14207f94a: 4889c1 mov rcx, rax
14207f94d: 4885c0 test rax, rax
14207f950: 0f8565ffffff jne 0x14207f8bb
14207f956: eb18 jmp 0x14207f970 ; unknown
14207f958: 4885c0 test rax, rax
14207f95b: 7513 jne 0x14207f970
14207f95d: 31d2 xor edx, edx
14207f95f: e86c8e8700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14207f964: 4889c1 mov rcx, rax
14207f967: 4885c0 test rax, rax
14207f96a: 0f853cf9ffff jne 0x14207f2ac
14207f970: 488d0d29b1bb00 lea rcx, [rip + 0xbbb129]
14207f977: e854878500 call 0x1428d80d0 ; std::thread::local::panic_access_error
14207f97c: 488d0dedb1bb00 lea rcx, [rip + 0xbbb1ed]
14207f983: e8c8988500 call 0x1428d9250 ; core::cell::panic_already_borrowed
14207f988: 4889f1 mov rcx, rsi
14207f98b: 31d2 xor edx, edx
14207f98d: e8cec58400 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
14207f992: e904ffffff jmp 0x14207f89b ; unknown
14207f997: 660f1f840000000000 nop word ptr [rax + rax]
14207f9a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207f9a5: 55 push rbp
14207f9a6: 4157 push r15
14207f9a8: 4156 push r14
14207f9aa: 4155 push r13
14207f9ac: 4154 push r12
14207f9ae: 56 push rsi
14207f9af: 57 push rdi
14207f9b0: 53 push rbx
14207f9b1: 4881ecc8000000 sub rsp, 0xc8
14207f9b8: 488daa80000000 lea rbp, [rdx + 0x80]
14207f9bf: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
14207f9c5: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
14207f9cb: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
14207f9d1: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
14207f9d7: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
14207f9dd: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
14207f9e6: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
14207f9ef: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
14207f9f7: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
14207f9ff: 488b4570 mov rax, qword ptr [rbp + 0x70]
14207fa03: 48ff00 inc qword ptr [rax]
14207fa06: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
14207fa0e: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
14207fa16: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
14207fa1f: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
14207fa28: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
14207fa2e: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
14207fa34: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
14207fa3a: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
14207fa40: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
14207fa46: 4881c4c8000000 add rsp, 0xc8
14207fa4d: 5b pop rbx
14207fa4e: 5f pop rdi
14207fa4f: 5e pop rsi
14207fa50: 415c pop r12
14207fa52: 415d pop r13
14207fa54: 415e pop r14
14207fa56: 415f pop r15
14207fa58: 5d pop rbp
14207fa59: c3 ret 
14207fa5a: 660f1f440000 nop word ptr [rax + rax]
14207fa60: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207fa65: 55 push rbp
14207fa66: 4157 push r15
14207fa68: 4156 push r14
14207fa6a: 4155 push r13
14207fa6c: 4154 push r12
14207fa6e: 56 push rsi
14207fa6f: 57 push rdi
14207fa70: 53 push rbx
14207fa71: 4881ecc8000000 sub rsp, 0xc8
14207fa78: 488daa80000000 lea rbp, [rdx + 0x80]
14207fa7f: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
14207fa85: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
14207fa8b: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
14207fa91: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
14207fa97: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
14207fa9d: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
14207faa6: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
14207faaf: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
14207fab7: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
14207fabf: 488b4570 mov rax, qword ptr [rbp + 0x70]
14207fac3: 48ff00 inc qword ptr [rax]
14207fac6: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
14207face: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
14207fad6: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
14207fadf: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
14207fae8: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
14207faee: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
14207faf4: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
14207fafa: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
14207fb00: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
14207fb06: 4881c4c8000000 add rsp, 0xc8
14207fb0d: 5b pop rbx
14207fb0e: 5f pop rdi
14207fb0f: 5e pop rsi
14207fb10: 415c pop r12
14207fb12: 415d pop r13
14207fb14: 415e pop r14
14207fb16: 415f pop r15
14207fb18: 5d pop rbp
14207fb19: c3 ret 
14207fb1a: 660f1f440000 nop word ptr [rax + rax]
14207fb20: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207fb25: 55 push rbp
14207fb26: 4157 push r15
14207fb28: 4156 push r14
14207fb2a: 4155 push r13
14207fb2c: 4154 push r12
14207fb2e: 56 push rsi
14207fb2f: 57 push rdi
14207fb30: 53 push rbx
14207fb31: 4881ecc8000000 sub rsp, 0xc8
14207fb38: 488daa80000000 lea rbp, [rdx + 0x80]
14207fb3f: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
14207fb45: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
14207fb4b: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
14207fb51: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
14207fb57: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
14207fb5d: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
14207fb66: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
14207fb6f: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
14207fb77: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
14207fb7f: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
14207fb83: e84817fcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
14207fb88: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
14207fb90: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
14207fb98: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
14207fba1: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
14207fbaa: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
14207fbb0: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
14207fbb6: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
14207fbbc: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
14207fbc2: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
14207fbc8: 4881c4c8000000 add rsp, 0xc8
14207fbcf: 5b pop rbx
14207fbd0: 5f pop rdi
14207fbd1: 5e pop rsi
14207fbd2: 415c pop r12
14207fbd4: 415d pop r13
14207fbd6: 415e pop r14
14207fbd8: 415f pop r15
14207fbda: 5d pop rbp
14207fbdb: c3 ret 
14207fbdc: 0f1f4000 nop dword ptr [rax]
14207fbe0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207fbe5: 55 push rbp
14207fbe6: 4157 push r15
14207fbe8: 4156 push r14
14207fbea: 4155 push r13
14207fbec: 4154 push r12
14207fbee: 56 push rsi
14207fbef: 57 push rdi
14207fbf0: 53 push rbx
14207fbf1: 4881ecc8000000 sub rsp, 0xc8
14207fbf8: 488daa80000000 lea rbp, [rdx + 0x80]
14207fbff: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
14207fc05: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
14207fc0b: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
14207fc11: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
14207fc17: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
14207fc1d: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
14207fc26: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
14207fc2f: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
14207fc37: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
14207fc3f: 48837d5000 cmp qword ptr [rbp + 0x50], 0
14207fc44: 7420 je 0x14207fc66
14207fc46: 488d4558 lea rax, [rbp + 0x58]
14207fc4a: 48894530 mov qword ptr [rbp + 0x30], rax
14207fc4e: 488d4530 lea rax, [rbp + 0x30]
14207fc52: 48894528 mov qword ptr [rbp + 0x28], rax
14207fc56: 488d0dfb36f500 lea rcx, [rip + 0xf536fb]
14207fc5d: 488d5528 lea rdx, [rbp + 0x28]
14207fc61: e86af62aff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
14207fc66: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
14207fc6e: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
14207fc76: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
14207fc7f: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
14207fc88: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
14207fc8e: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
14207fc94: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
14207fc9a: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
14207fca0: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
14207fca6: 4881c4c8000000 add rsp, 0xc8
14207fcad: 5b pop rbx
14207fcae: 5f pop rdi
14207fcaf: 5e pop rsi
14207fcb0: 415c pop r12
14207fcb2: 415d pop r13
14207fcb4: 415e pop r14
14207fcb6: 415f pop r15
14207fcb8: 5d pop rbp
14207fcb9: c3 ret 
