14227a0d0: 55 push rbp
14227a0d1: 4157 push r15
14227a0d3: 4156 push r14
14227a0d5: 4155 push r13
14227a0d7: 4154 push r12
14227a0d9: 56 push rsi
14227a0da: 57 push rdi
14227a0db: 53 push rbx
14227a0dc: 4881ec38010000 sub rsp, 0x138
14227a0e3: 488dac2480000000 lea rbp, [rsp + 0x80]
14227a0eb: 440f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm8
14227a0f3: 0f29bd90000000 movaps xmmword ptr [rbp + 0x90], xmm7
14227a0fa: 0f29b580000000 movaps xmmword ptr [rbp + 0x80], xmm6
14227a101: 48c74578feffffff mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
14227a109: 4c89cf mov rdi, r9
14227a10c: 4c89c6 mov rsi, r8
14227a10f: 4989d7 mov r15, rdx
14227a112: 4989ce mov r14, rcx
14227a115: e8e6c899fe call 0x140c16a00 ; puffin::are_scopes_on
14227a11a: 84c0 test al, al
14227a11c: 7464 je 0x14227a182
14227a11e: 4c897d60 mov qword ptr [rbp + 0x60], r15
14227a122: 4c8d25a7dddb00 lea r12, [rip + 0xdbdda7] ; @143037ed0 'forest::tree_animation::forest_tree_animation::fforest::tree_animation::handplaced_tree_animatio'
14227a129: 41bf2d000000 mov r15d, 0x2d
14227a12f: ba2d000000 mov edx, 0x2d
14227a134: 4c89e1 mov rcx, r12
14227a137: e8a4efffff call 0x1422790e0 ; core::str::impl$0::rfind<ref$<str$> >
14227a13c: 4883f801 cmp rax, 1
14227a140: 0f85b2000000 jne 0x14227a1f8
14227a146: 4989d1 mov r9, rdx
14227a149: 4885d2 test rdx, rdx
14227a14c: 743d je 0x14227a18b
14227a14e: 4983f92d cmp r9, 0x2d
14227a152: 7335 jae 0x14227a189
14227a154: 488d0575dddb00 lea rax, [rip + 0xdbdd75] ; @143037ed0 'forest::tree_animation::forest_tree_animation::fforest::tree_animation::handplaced_tree_animatio'
14227a15b: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
14227a160: 7f29 jg 0x14227a18b
14227a162: 488d054fdedb00 lea rax, [rip + 0xdbde4f]
14227a169: 4889442420 mov qword ptr [rsp + 0x20], rax
14227a16e: 488d0d5bdddb00 lea rcx, [rip + 0xdbdd5b] ; @143037ed0 'forest::tree_animation::forest_tree_animation::fforest::tree_animation::handplaced_tree_animatio'
14227a175: ba2d000000 mov edx, 0x2d
14227a17a: 4531c0 xor r8d, r8d
14227a17d: e8bef76500 call 0x1428d9940 ; core::str::slice_error_fail
14227a182: 31c0 xor eax, eax
14227a184: e9c3010000 jmp 0x14227a34c
14227a189: 75d7 jne 0x14227a162
14227a18b: 4c8d253edddb00 lea r12, [rip + 0xdbdd3e] ; @143037ed0 'forest::tree_animation::forest_tree_animation::fforest::tree_animation::handplaced_tree_animatio'
14227a192: 4c89e1 mov rcx, r12
14227a195: 4c89ca mov rdx, r9
14227a198: e843efffff call 0x1422790e0 ; core::str::impl$0::rfind<ref$<str$> >
14227a19d: 4883f801 cmp rax, 1
14227a1a1: 7555 jne 0x14227a1f8
14227a1a3: 4989d0 mov r8, rdx
14227a1a6: 4983c002 add r8, 2
14227a1aa: 7439 je 0x14227a1e5
14227a1ac: 4983f82d cmp r8, 0x2d
14227a1b0: 7331 jae 0x14227a1e3
14227a1b2: 488d0517dddb00 lea rax, [rip + 0xdbdd17] ; @143037ed0 'forest::tree_animation::forest_tree_animation::fforest::tree_animation::handplaced_tree_animatio'
14227a1b9: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
14227a1be: 7f25 jg 0x14227a1e5
14227a1c0: 488d0509dedb00 lea rax, [rip + 0xdbde09]
14227a1c7: 4889442420 mov qword ptr [rsp + 0x20], rax
14227a1cc: 488d0dfddcdb00 lea rcx, [rip + 0xdbdcfd] ; @143037ed0 'forest::tree_animation::forest_tree_animation::fforest::tree_animation::handplaced_tree_animatio'
14227a1d3: ba2d000000 mov edx, 0x2d
14227a1d8: 41b92d000000 mov r9d, 0x2d
14227a1de: e85df76500 call 0x1428d9940 ; core::str::slice_error_fail
14227a1e3: 75db jne 0x14227a1c0
14227a1e5: 41bf2b000000 mov r15d, 0x2b
14227a1eb: 4929d7 sub r15, rdx
14227a1ee: 4c8d25dbdcdb00 lea r12, [rip + 0xdbdcdb] ; @143037ed0 'forest::tree_animation::forest_tree_animation::fforest::tree_animation::handplaced_tree_animatio'
14227a1f5: 4d01c4 add r12, r8
14227a1f8: 4c8d052cdedb00 lea r8, [rip + 0xdbde2c]
14227a1ff: 4c8d2dfadddb00 lea r13, [rip + 0xdbddfa] ; @143038000 'crates/systems/forest/src/tree_animation.rs'
14227a206: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14227a210: 410fbe40ff movsx eax, byte ptr [r8 - 1]
14227a215: 85c0 test eax, eax
14227a217: 7817 js 0x14227a230
14227a219: 49ffc8 dec r8
14227a21c: 83f85c cmp eax, 0x5c
14227a21f: 7563 jne 0x14227a284
14227a221: eb72 jmp 0x14227a295
14227a223: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14227a230: 410fb648fe movzx ecx, byte ptr [r8 - 2]
14227a235: 80f9c0 cmp cl, 0xc0
14227a238: 7d23 jge 0x14227a25d
14227a23a: 410fb650fd movzx edx, byte ptr [r8 - 3]
14227a23f: 80fac0 cmp dl, 0xc0
14227a242: 7d22 jge 0x14227a266
14227a244: 450fb648fc movzx r9d, byte ptr [r8 - 4]
14227a249: 4983c0fc add r8, -4
14227a24d: 4183e107 and r9d, 7
14227a251: 41c1e106 shl r9d, 6
14227a255: 83e23f and edx, 0x3f
14227a258: 4409ca or edx, r9d
14227a25b: eb10 jmp 0x14227a26d
14227a25d: 4983c0fe add r8, -2
14227a261: 83e11f and ecx, 0x1f
14227a264: eb0f jmp 0x14227a275
14227a266: 4983c0fd add r8, -3
14227a26a: 83e20f and edx, 0xf
14227a26d: c1e206 shl edx, 6
14227a270: 83e13f and ecx, 0x3f
14227a273: 09d1 or ecx, edx
14227a275: c1e106 shl ecx, 6
14227a278: 243f and al, 0x3f
14227a27a: 0fb6c0 movzx eax, al
14227a27d: 09c8 or eax, ecx
14227a27f: 83f85c cmp eax, 0x5c
14227a282: 7411 je 0x14227a295
14227a284: 83f82f cmp eax, 0x2f
14227a287: 740c je 0x14227a295
14227a289: 4d39e8 cmp r8, r13
14227a28c: 7582 jne 0x14227a210
14227a28e: bb2b000000 mov ebx, 0x2b
14227a293: eb4c jmp 0x14227a2e1
14227a295: 4c8d2d64dddb00 lea r13, [rip + 0xdbdd64] ; @143038000 'crates/systems/forest/src/tree_animation.rs'
14227a29c: 4d29e8 sub r8, r13
14227a29f: 49ffc0 inc r8
14227a2a2: 7432 je 0x14227a2d6
14227a2a4: 4983f82b cmp r8, 0x2b
14227a2a8: 732a jae 0x14227a2d4
14227a2aa: 43803c28bf cmp byte ptr [r8 + r13], 0xbf
14227a2af: 7f25 jg 0x14227a2d6
14227a2b1: 488d05e0dcdb00 lea rax, [rip + 0xdbdce0]
14227a2b8: 4889442420 mov qword ptr [rsp + 0x20], rax
14227a2bd: 488d0d3cdddb00 lea rcx, [rip + 0xdbdd3c] ; @143038000 'crates/systems/forest/src/tree_animation.rs'
14227a2c4: ba2b000000 mov edx, 0x2b
14227a2c9: 41b92b000000 mov r9d, 0x2b
14227a2cf: e86cf66500 call 0x1428d9940 ; core::str::slice_error_fail
14227a2d4: 75db jne 0x14227a2b1
14227a2d6: bb2b000000 mov ebx, 0x2b
14227a2db: 4c29c3 sub rbx, r8
14227a2de: 4d01c5 add r13, r8
14227a2e1: e84ade99fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14227a2e6: 4889c1 mov rcx, rax
14227a2e9: 488b00 mov rax, qword ptr [rax]
14227a2ec: 4883f801 cmp rax, 1
14227a2f0: 0f850f050000 jne 0x14227a805
14227a2f6: 4883c108 add rcx, 8
14227a2fa: 48833900 cmp qword ptr [rcx], 0
14227a2fe: 0f8525050000 jne 0x14227a829
14227a304: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14227a30b: 48894d68 mov qword ptr [rbp + 0x68], rcx
14227a30f: 4883c108 add rcx, 8
14227a313: 48895c2420 mov qword ptr [rsp + 0x20], rbx
14227a318: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
14227a321: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14227a32a: 4c89e2 mov rdx, r12
14227a32d: 4d89f8 mov r8, r15
14227a330: 4d89e9 mov r9, r13
14227a333: e808ca99fe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
14227a338: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
14227a33c: 48ff01 inc qword ptr [rcx]
14227a33f: 488945f0 mov qword ptr [rbp - 0x10], rax
14227a343: b801000000 mov eax, 1
14227a348: 4c8b7d60 mov r15, qword ptr [rbp + 0x60]
14227a34c: 488945e8 mov qword ptr [rbp - 0x18], rax
14227a350: 498b0e mov rcx, qword ptr [r14]
14227a353: 4d8b6608 mov r12, qword ptr [r14 + 8]
14227a357: 418b4614 mov eax, dword ptr [r14 + 0x14]
14227a35b: 894560 mov dword ptr [rbp + 0x60], eax
14227a35e: 4c8ba9f8000000 mov r13, qword ptr [rcx + 0xf8]
14227a365: 488b8100010000 mov rax, qword ptr [rcx + 0x100]
14227a36c: 498d5c8500 lea rbx, [r13 + rax*4]
14227a371: 48894d58 mov qword ptr [rbp + 0x58], rcx
14227a375: 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
14227a37c: 884577 mov byte ptr [rbp + 0x77], al
14227a37f: 498b07 mov rax, qword ptr [r15]
14227a382: 48894508 mov qword ptr [rbp + 8], rax
14227a386: 8b471c mov eax, dword ptr [rdi + 0x1c]
14227a389: 894534 mov dword ptr [rbp + 0x34], eax
14227a38c: 488b07 mov rax, qword ptr [rdi]
14227a38f: 488945d0 mov qword ptr [rbp - 0x30], rax
14227a393: 488b4710 mov rax, qword ptr [rdi + 0x10]
14227a397: 488945d8 mov qword ptr [rbp - 0x28], rax
14227a39b: 8b461c mov eax, dword ptr [rsi + 0x1c]
14227a39e: 894530 mov dword ptr [rbp + 0x30], eax
14227a3a1: 488b06 mov rax, qword ptr [rsi]
14227a3a4: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
14227a3a8: 48894dc8 mov qword ptr [rbp - 0x38], rcx
14227a3ac: f30f10354cb56a00 movss xmm6, dword ptr [rip + 0x6ab54c]
14227a3b4: 0f57ff xorps xmm7, xmm7
14227a3b7: f3440f100544b56a00 movss xmm8, dword ptr [rip + 0x6ab544]
14227a3c0: 48894528 mov qword ptr [rbp + 0x28], rax
14227a3c4: 4883c020 add rax, 0x20
14227a3c8: 488945e0 mov qword ptr [rbp - 0x20], rax
14227a3cc: b808000000 mov eax, 8
14227a3d1: 48894518 mov qword ptr [rbp + 0x18], rax
14227a3d5: 48c7454800000000 mov qword ptr [rbp + 0x48], 0
14227a3dd: 4531f6 xor r14d, r14d
14227a3e0: 31f6 xor esi, esi
14227a3e2: b808000000 mov eax, 8
14227a3e7: 48894520 mov qword ptr [rbp + 0x20], rax
14227a3eb: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
14227a3f3: c7457000000000 mov dword ptr [rbp + 0x70], 0
14227a3fa: 807d7700 cmp byte ptr [rbp + 0x77], 0
14227a3fe: 7460 je 0x14227a460
14227a400: 4939f6 cmp r14, rsi
14227a403: 0f852e010000 jne 0x14227a537
14227a409: 0f1f8000000000 nop dword ptr [rax]
14227a410: 4939dd cmp r13, rbx
14227a413: 0f8467030000 je 0x14227a780
14227a419: 418b4d00 mov ecx, dword ptr [r13]
14227a41d: 4983c504 add r13, 4
14227a421: 498b8424a8010000 mov rax, qword ptr [r12 + 0x1a8]
14227a429: 488d0cc9 lea rcx, [rcx + rcx*8]
14227a42d: 488b74c810 mov rsi, qword ptr [rax + rcx*8 + 0x10]
14227a432: 4885f6 test rsi, rsi
14227a435: 74d9 je 0x14227a410
14227a437: 488d04c8 lea rax, [rax + rcx*8]
14227a43b: 488b4838 mov rcx, qword ptr [rax + 0x38]
14227a43f: 488b5558 mov rdx, qword ptr [rbp + 0x58]
14227a443: 488b9220010000 mov rdx, qword ptr [rdx + 0x120]
14227a44a: 48395040 cmp qword ptr [rax + 0x40], rdx
14227a44e: 767d jbe 0x14227a4cd
14227a450: 48833cd100 cmp qword ptr [rcx + rdx*8], 0
14227a455: 0f95c2 setne dl
14227a458: 895570 mov dword ptr [rbp + 0x70], edx
14227a45b: eb77 jmp 0x14227a4d4
14227a45d: 0f1f00 nop dword ptr [rax]
14227a460: 4939f6 cmp r14, rsi
14227a463: 0f8564010000 jne 0x14227a5cd
14227a469: 0f1f8000000000 nop dword ptr [rax]
14227a470: 4939dd cmp r13, rbx
14227a473: 0f8407030000 je 0x14227a780
14227a479: 418b4d00 mov ecx, dword ptr [r13]
14227a47d: 4983c504 add r13, 4
14227a481: 498b842400010000 mov rax, qword ptr [r12 + 0x100]
14227a489: 488d0c89 lea rcx, [rcx + rcx*4]
14227a48d: 48c1e105 shl rcx, 5
14227a491: 488b740858 mov rsi, qword ptr [rax + rcx + 0x58]
14227a496: 4885f6 test rsi, rsi
14227a499: 74d5 je 0x14227a470
14227a49b: 4801c8 add rax, rcx
14227a49e: 488b4d58 mov rcx, qword ptr [rbp + 0x58]
14227a4a2: 488b8920010000 mov rcx, qword ptr [rcx + 0x120]
14227a4a9: 48398888000000 cmp qword ptr [rax + 0x88], rcx
14227a4b0: 0f8695000000 jbe 0x14227a54b
14227a4b6: 488b9080000000 mov rdx, qword ptr [rax + 0x80]
14227a4bd: 48833cca00 cmp qword ptr [rdx + rcx*8], 0
14227a4c2: 0f95c1 setne cl
14227a4c5: 894d70 mov dword ptr [rbp + 0x70], ecx
14227a4c8: e985000000 jmp 0x14227a552
14227a4cd: c7457000000000 mov dword ptr [rbp + 0x70], 0
14227a4d4: 488b5018 mov rdx, qword ptr [rax + 0x18]
14227a4d8: 4c8b4d58 mov r9, qword ptr [rbp + 0x58]
14227a4dc: 4d8b8110010000 mov r8, qword ptr [r9 + 0x110]
14227a4e3: 4d8b8918010000 mov r9, qword ptr [r9 + 0x118]
14227a4ea: 4e8b04c1 mov r8, qword ptr [rcx + r8*8]
14227a4ee: 49f7d0 not r8
14227a4f1: 4f8d0440 lea r8, [r8 + r8*2]
14227a4f5: 49c1e004 shl r8, 4
14227a4f9: 4e8b540210 mov r10, qword ptr [rdx + r8 + 0x10]
14227a4fe: 4c895548 mov qword ptr [rbp + 0x48], r10
14227a502: 4e8b440228 mov r8, qword ptr [rdx + r8 + 0x28]
14227a507: 4c894538 mov qword ptr [rbp + 0x38], r8
14227a50b: 4a8b0cc9 mov rcx, qword ptr [rcx + r9*8]
14227a50f: 48f7d1 not rcx
14227a512: 488d0c49 lea rcx, [rcx + rcx*2]
14227a516: 48c1e104 shl rcx, 4
14227a51a: 4c8b440a10 mov r8, qword ptr [rdx + rcx + 0x10]
14227a51f: 4c894550 mov qword ptr [rbp + 0x50], r8
14227a523: 488b4c0a28 mov rcx, qword ptr [rdx + rcx + 0x28]
14227a528: 48894d40 mov qword ptr [rbp + 0x40], rcx
14227a52c: 488b4008 mov rax, qword ptr [rax + 8]
14227a530: 48894520 mov qword ptr [rbp + 0x20], rax
14227a534: 4531f6 xor r14d, r14d
14227a537: 488b4520 mov rax, qword ptr [rbp + 0x20]
14227a53b: 4a8b04f0 mov rax, qword ptr [rax + r14*8]
14227a53f: 48894510 mov qword ptr [rbp + 0x10], rax
14227a543: 4489f1 mov ecx, r14d
14227a546: e999000000 jmp 0x14227a5e4
14227a54b: c7457000000000 mov dword ptr [rbp + 0x70], 0
14227a552: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
14227a558: 498b9424a8010000 mov rdx, qword ptr [r12 + 0x1a8]
14227a560: 488d0cc9 lea rcx, [rcx + rcx*8]
14227a564: 4c8b44ca18 mov r8, qword ptr [rdx + rcx*8 + 0x18]
14227a569: 488b4cca38 mov rcx, qword ptr [rdx + rcx*8 + 0x38]
14227a56e: 4c8b4d58 mov r9, qword ptr [rbp + 0x58]
14227a572: 498b9110010000 mov rdx, qword ptr [r9 + 0x110]
14227a579: 4d8b8918010000 mov r9, qword ptr [r9 + 0x118]
14227a580: 488b14d1 mov rdx, qword ptr [rcx + rdx*8]
14227a584: 48f7d2 not rdx
14227a587: 488d1452 lea rdx, [rdx + rdx*2]
14227a58b: 48c1e204 shl rdx, 4
14227a58f: 4d8b541010 mov r10, qword ptr [r8 + rdx + 0x10]
14227a594: 4c895548 mov qword ptr [rbp + 0x48], r10
14227a598: 498b541028 mov rdx, qword ptr [r8 + rdx + 0x28]
14227a59d: 48895538 mov qword ptr [rbp + 0x38], rdx
14227a5a1: 4a8b0cc9 mov rcx, qword ptr [rcx + r9*8]
14227a5a5: 48f7d1 not rcx
14227a5a8: 488d0c49 lea rcx, [rcx + rcx*2]
14227a5ac: 48c1e104 shl rcx, 4
14227a5b0: 498b540810 mov rdx, qword ptr [r8 + rcx + 0x10]
14227a5b5: 48895550 mov qword ptr [rbp + 0x50], rdx
14227a5b9: 498b4c0828 mov rcx, qword ptr [r8 + rcx + 0x28]
14227a5be: 48894d40 mov qword ptr [rbp + 0x40], rcx
14227a5c2: 488b4050 mov rax, qword ptr [rax + 0x50]
14227a5c6: 48894518 mov qword ptr [rbp + 0x18], rax
14227a5ca: 4531f6 xor r14d, r14d
14227a5cd: 4c89f0 mov rax, r14
14227a5d0: 48c1e004 shl rax, 4
14227a5d4: 488b4d18 mov rcx, qword ptr [rbp + 0x18]
14227a5d8: 488b1401 mov rdx, qword ptr [rcx + rax]
14227a5dc: 48895510 mov qword ptr [rbp + 0x10], rdx
14227a5e0: 8b4c0108 mov ecx, dword ptr [rcx + rax + 8]
14227a5e4: 488d3c49 lea rdi, [rcx + rcx*2]
14227a5e8: 488b4548 mov rax, qword ptr [rbp + 0x48]
14227a5ec: 4c8d3cf8 lea r15, [rax + rdi*8]
14227a5f0: 488b4538 mov rax, qword ptr [rbp + 0x38]
14227a5f4: 488d0488 lea rax, [rax + rcx*4]
14227a5f8: 48c1e704 shl rdi, 4
14227a5fc: 48037d50 add rdi, qword ptr [rbp + 0x50]
14227a600: 488b5540 mov rdx, qword ptr [rbp + 0x40]
14227a604: 488d0c8a lea rcx, [rdx + rcx*4]
14227a608: 48894df8 mov qword ptr [rbp - 8], rcx
14227a60c: f6457001 test byte ptr [rbp + 0x70], 1
14227a610: 742e je 0x14227a640
14227a612: 498d4f0c lea rcx, [r15 + 0xc]
14227a616: 48894d00 mov qword ptr [rbp], rcx
14227a61a: f3410f10470c movss xmm0, dword ptr [r15 + 0xc]
14227a620: 488b4d08 mov rcx, qword ptr [rbp + 8]
14227a624: f30f104928 movss xmm1, dword ptr [rcx + 0x28]
14227a629: f30f5ece divss xmm1, xmm6
14227a62d: c7456800000000 mov dword ptr [rbp + 0x68], 0
14227a634: eb4f jmp 0x14227a685
14227a636: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14227a640: 498d4f0c lea rcx, [r15 + 0xc]
14227a644: 48894d00 mov qword ptr [rbp], rcx
14227a648: f3410f10470c movss xmm0, dword ptr [r15 + 0xc]
14227a64e: f3410f104f10 movss xmm1, dword ptr [r15 + 0x10]
14227a654: 0f2ecf ucomiss xmm1, xmm7
14227a657: 488b4d08 mov rcx, qword ptr [rbp + 8]
14227a65b: f30f104928 movss xmm1, dword ptr [rcx + 0x28]
14227a660: 761a jbe 0x14227a67c
14227a662: 8b4d60 mov ecx, dword ptr [rbp + 0x60]
14227a665: 8908 mov dword ptr [rax], ecx
14227a667: f3410f105710 movss xmm2, dword ptr [r15 + 0x10]
14227a66d: f30f5cd1 subss xmm2, xmm1
14227a671: f3410f115710 movss dword ptr [r15 + 0x10], xmm2
14227a677: 0f57c9 xorps xmm1, xmm1
14227a67a: eb04 jmp 0x14227a680
14227a67c: f30f58c9 addss xmm1, xmm1
14227a680: b101 mov cl, 1
14227a682: 894d68 mov dword ptr [rbp + 0x68], ecx
14227a685: 8b4d60 mov ecx, dword ptr [rbp + 0x60]
14227a688: 8908 mov dword ptr [rax], ecx
14227a68a: f3410f584f0c addss xmm1, dword ptr [r15 + 0xc]
14227a690: 0f57d2 xorps xmm2, xmm2
14227a693: f30f5fd1 maxss xmm2, xmm1
14227a697: 410f28c8 movaps xmm1, xmm8
14227a69b: f30f5dca minss xmm1, xmm2
14227a69f: f3410f114f0c movss dword ptr [r15 + 0xc], xmm1
14227a6a5: 0f2ec8 ucomiss xmm1, xmm0
14227a6a8: 7502 jne 0x14227a6ac
14227a6aa: 7b20 jnp 0x14227a6cc
14227a6ac: 488d0d79d9db00 lea rcx, [rip + 0xdbd979]
14227a6b3: e838b8a1fe call 0x140c95ef0 ; utils::easing_functions::EasingFunction::apply
14227a6b8: f3410f594708 mulss xmm0, dword ptr [r15 + 8]
14227a6be: 8b4560 mov eax, dword ptr [rbp + 0x60]
14227a6c1: 488b4df8 mov rcx, qword ptr [rbp - 8]
14227a6c5: 8901 mov dword ptr [rcx], eax
14227a6c7: f30f11470c movss dword ptr [rdi + 0xc], xmm0
14227a6cc: 49ffc6 inc r14
14227a6cf: 807d6800 cmp byte ptr [rbp + 0x68], 0
14227a6d3: 0f8521fdffff jne 0x14227a3fa
14227a6d9: 488b4500 mov rax, qword ptr [rbp]
14227a6dd: f30f1000 movss xmm0, dword ptr [rax]
14227a6e1: 0f2ec7 ucomiss xmm0, xmm7
14227a6e4: 0f8510fdffff jne 0x14227a3fa
14227a6ea: 0f8a0afdffff jp 0x14227a3fa
14227a6f0: 8b4530 mov eax, dword ptr [rbp + 0x30]
14227a6f3: 488b4dc8 mov rcx, qword ptr [rbp - 0x38]
14227a6f7: 8901 mov dword ptr [rcx], eax
14227a6f9: 488b4528 mov rax, qword ptr [rbp + 0x28]
14227a6fd: 488b7830 mov rdi, qword ptr [rax + 0x30]
14227a701: 488b4840 mov rcx, qword ptr [rax + 0x40]
14227a705: 48894d68 mov qword ptr [rbp + 0x68], rcx
14227a709: 483b7820 cmp rdi, qword ptr [rax + 0x20]
14227a70d: 7510 jne 0x14227a71f
14227a70f: 488b4de0 mov rcx, qword ptr [rbp - 0x20]
14227a713: 488d15fed5db00 lea rdx, [rip + 0xdbd5fe]
14227a71a: e8117f2efe call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<toolbar::ui_interaction::UiJustBecameInFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::WallId,ref$<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemWindow>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<asset::prefab::PrefabName,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<alloc::ffi::c_str::CString,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::onboarding::NotifyHintSystemRoofShape>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ref$<std::ffi::os_str::OsStr>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::systems::prefab::DespawnPrefabCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_onboarding::tutorial_completion::TutorialHintComplete>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_color::set_decorator_color_id::SetStairRailingCmd>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<u64,ref$<system_clutter::autoclutter::AutoClutterObject> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<core::option::Option<bevy_ecs::entity::Entity> >,bool>,alloc::alloc::Global>
14227a71f: 488b5528 mov rdx, qword ptr [rbp + 0x28]
14227a723: 488b4228 mov rax, qword ptr [rdx + 0x28]
14227a727: 4889f9 mov rcx, rdi
14227a72a: 48c1e104 shl rcx, 4
14227a72e: 4c8b4568 mov r8, qword ptr [rbp + 0x68]
14227a732: 4c890408 mov qword ptr [rax + rcx], r8
14227a736: 4c8b4510 mov r8, qword ptr [rbp + 0x10]
14227a73a: 4c89440808 mov qword ptr [rax + rcx + 8], r8
14227a73f: 48ffc7 inc rdi
14227a742: 48897a30 mov qword ptr [rdx + 0x30], rdi
14227a746: 48ff4240 inc qword ptr [rdx + 0x40]
14227a74a: 8b4534 mov eax, dword ptr [rbp + 0x34]
14227a74d: 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
14227a751: 8901 mov dword ptr [rcx], eax
14227a753: 498b0f mov rcx, qword ptr [r15]
14227a756: 488b45d0 mov rax, qword ptr [rbp - 0x30]
14227a75a: 488b5010 mov rdx, qword ptr [rax + 0x10]
14227a75e: 4839d1 cmp rcx, rdx
14227a761: 0f83ce000000 jae 0x14227a835
14227a767: 488b4008 mov rax, qword ptr [rax + 8]
14227a76b: 488d0c89 lea rcx, [rcx + rcx*4]
14227a76f: 48c1e104 shl rcx, 4
14227a773: c744084400000000 mov dword ptr [rax + rcx + 0x44], 0
14227a77b: e97afcffff jmp 0x14227a3fa
14227a780: 48837de800 cmp qword ptr [rbp - 0x18], 0
14227a785: 743e je 0x14227a7c5
14227a787: e8a4d999fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14227a78c: 4889c1 mov rcx, rax
14227a78f: 488b00 mov rax, qword ptr [rax]
14227a792: 4883f801 cmp rax, 1
14227a796: 7557 jne 0x14227a7ef
14227a798: 4883c108 add rcx, 8
14227a79c: 48833900 cmp qword ptr [rcx], 0
14227a7a0: 0f8583000000 jne 0x14227a829
14227a7a6: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14227a7ad: 48894d60 mov qword ptr [rbp + 0x60], rcx
14227a7b1: 4883c108 add rcx, 8
14227a7b5: 488b55f0 mov rdx, qword ptr [rbp - 0x10]
14227a7b9: e832c999fe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
14227a7be: 488b4560 mov rax, qword ptr [rbp + 0x60]
14227a7c2: 48ff00 inc qword ptr [rax]
14227a7c5: 0f28b580000000 movaps xmm6, xmmword ptr [rbp + 0x80]
14227a7cc: 0f28bd90000000 movaps xmm7, xmmword ptr [rbp + 0x90]
14227a7d3: 440f2885a0000000 movaps xmm8, xmmword ptr [rbp + 0xa0]
14227a7db: 4881c438010000 add rsp, 0x138
14227a7e2: 5b pop rbx
14227a7e3: 5f pop rdi
14227a7e4: 5e pop rsi
14227a7e5: 415c pop r12
14227a7e7: 415d pop r13
14227a7e9: 415e pop r14
14227a7eb: 415f pop r15
14227a7ed: 5d pop rbp
14227a7ee: c3 ret 
14227a7ef: 4885c0 test rax, rax
14227a7f2: 7529 jne 0x14227a81d
14227a7f4: 31d2 xor edx, edx
14227a7f6: e865fd6900 call 0x14291a560 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14227a7fb: 4889c1 mov rcx, rax
14227a7fe: 4885c0 test rax, rax
14227a801: 7599 jne 0x14227a79c
14227a803: eb18 jmp 0x14227a81d
14227a805: 4885c0 test rax, rax
14227a808: 7513 jne 0x14227a81d
14227a80a: 31d2 xor edx, edx
14227a80c: e84ffd6900 call 0x14291a560 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14227a811: 4889c1 mov rcx, rax
14227a814: 4885c0 test rax, rax
14227a817: 0f85ddfaffff jne 0x14227a2fa
14227a81d: 488d0d5cd5db00 lea rcx, [rip + 0xdbd55c]
14227a824: e8a7d86500 call 0x1428d80d0 ; std::thread::local::panic_access_error
14227a829: 488d0dc0d5db00 lea rcx, [rip + 0xdbd5c0]
14227a830: e81bea6500 call 0x1428d9250 ; core::cell::panic_already_borrowed
14227a835: 4c8d05fcd7db00 lea r8, [rip + 0xdbd7fc]
14227a83c: e8d7ec6500 call 0x1428d9518 ; core::panicking::panic_bounds_check
14227a841: 0f0b ud2 
14227a843: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14227a850: 4889542410 mov qword ptr [rsp + 0x10], rdx
14227a855: 55 push rbp
14227a856: 4157 push r15
14227a858: 4156 push r14
14227a85a: 4155 push r13
14227a85c: 4154 push r12
14227a85e: 56 push rsi
14227a85f: 57 push rdi
14227a860: 53 push rbx
14227a861: 4883ec68 sub rsp, 0x68
14227a865: 488daa80000000 lea rbp, [rdx + 0x80]
14227a86c: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
14227a872: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
14227a877: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
14227a87c: 488b4568 mov rax, qword ptr [rbp + 0x68]
14227a880: 48ff00 inc qword ptr [rax]
14227a883: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
14227a888: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
14227a88d: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
14227a893: 4883c468 add rsp, 0x68
14227a897: 5b pop rbx
14227a898: 5f pop rdi
14227a899: 5e pop rsi
14227a89a: 415c pop r12
14227a89c: 415d pop r13
14227a89e: 415e pop r14
14227a8a0: 415f pop r15
14227a8a2: 5d pop rbp
14227a8a3: c3 ret 
14227a8a4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14227a8b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14227a8b5: 55 push rbp
14227a8b6: 4157 push r15
14227a8b8: 4156 push r14
14227a8ba: 4155 push r13
14227a8bc: 4154 push r12
14227a8be: 56 push rsi
14227a8bf: 57 push rdi
14227a8c0: 53 push rbx
14227a8c1: 4883ec68 sub rsp, 0x68
14227a8c5: 488daa80000000 lea rbp, [rdx + 0x80]
14227a8cc: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
14227a8d2: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
14227a8d7: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
14227a8dc: 48837de800 cmp qword ptr [rbp - 0x18], 0
14227a8e1: 7420 je 0x14227a903
14227a8e3: 488d45f0 lea rax, [rbp - 0x10]
14227a8e7: 488945c0 mov qword ptr [rbp - 0x40], rax
14227a8eb: 488d45c0 lea rax, [rbp - 0x40]
14227a8ef: 488945b8 mov qword ptr [rbp - 0x48], rax
14227a8f3: 488d0d3ed6db00 lea rcx, [rip + 0xdbd63e]
14227a8fa: 488d55b8 lea rdx, [rbp - 0x48]
14227a8fe: e84de7ffff call 0x142279050 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
14227a903: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
14227a908: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
14227a90d: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
14227a913: 4883c468 add rsp, 0x68
14227a917: 5b pop rbx
14227a918: 5f pop rdi
14227a919: 5e pop rsi
14227a91a: 415c pop r12
14227a91c: 415d pop r13
14227a91e: 415e pop r14
14227a920: 415f pop r15
14227a922: 5d pop rbp
14227a923: c3 ret 
14227a924: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14227a930: 4889542410 mov qword ptr [rsp + 0x10], rdx
14227a935: 55 push rbp
14227a936: 4157 push r15
14227a938: 4156 push r14
14227a93a: 4155 push r13
14227a93c: 4154 push r12
14227a93e: 56 push rsi
14227a93f: 57 push rdi
14227a940: 53 push rbx
14227a941: 4883ec68 sub rsp, 0x68
14227a945: 488daa80000000 lea rbp, [rdx + 0x80]
14227a94c: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
14227a952: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
14227a957: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
14227a95c: 488b4560 mov rax, qword ptr [rbp + 0x60]
14227a960: 48ff00 inc qword ptr [rax]
14227a963: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
14227a968: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
14227a96d: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
14227a973: 4883c468 add rsp, 0x68
14227a977: 5b pop rbx
14227a978: 5f pop rdi
14227a979: 5e pop rsi
14227a97a: 415c pop r12
14227a97c: 415d pop r13
14227a97e: 415e pop r14
14227a980: 415f pop r15
14227a982: 5d pop rbp
14227a983: c3 ret 
