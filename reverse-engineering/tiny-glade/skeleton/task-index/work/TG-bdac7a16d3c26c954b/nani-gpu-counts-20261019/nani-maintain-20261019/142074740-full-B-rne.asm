142074740: 55 push rbp
142074741: 4157 push r15
142074743: 4156 push r14
142074745: 4155 push r13
142074747: 4154 push r12
142074749: 56 push rsi
14207474a: 57 push rdi
14207474b: 53 push rbx
14207474c: 4881ec28010000 sub rsp, 0x128
142074753: 488dac2480000000 lea rbp, [rsp + 0x80]
14207475b: 440f299590000000 movaps xmmword ptr [rbp + 0x90], xmm10
142074763: 440f298d80000000 movaps xmmword ptr [rbp + 0x80], xmm9
14207476b: 440f294570 movaps xmmword ptr [rbp + 0x70], xmm8
142074770: 0f297d60 movaps xmmword ptr [rbp + 0x60], xmm7
142074774: 0f297550 movaps xmmword ptr [rbp + 0x50], xmm6
142074778: 48c74548feffffff mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
142074780: 4889d6 mov rsi, rdx
142074783: 4889cf mov rdi, rcx
142074786: e87522bafe call 0x140c16a00 ; puffin::are_scopes_on
14207478b: 84c0 test al, al
14207478d: 745f je 0x1420747ee
14207478f: 4c8d35ece7f500 lea r14, [rip + 0xf5e7ec]
142074796: bb36000000 mov ebx, 0x36
14207479b: ba36000000 mov edx, 0x36
1420747a0: 4c89f1 mov rcx, r14
1420747a3: e8c8a6ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
1420747a8: 4883f801 cmp rax, 1
1420747ac: 0f85b1000000 jne 0x142074863
1420747b2: 4989d1 mov r9, rdx
1420747b5: 4885d2 test rdx, rdx
1420747b8: 743d je 0x1420747f7
1420747ba: 4983f936 cmp r9, 0x36
1420747be: 7335 jae 0x1420747f5
1420747c0: 488d05bbe7f500 lea rax, [rip + 0xf5e7bb]
1420747c7: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1420747cc: 7f29 jg 0x1420747f7
1420747ce: 488d0503ecf500 lea rax, [rip + 0xf5ec03]
1420747d5: 4889442420 mov qword ptr [rsp + 0x20], rax
1420747da: 488d0da1e7f500 lea rcx, [rip + 0xf5e7a1]
1420747e1: ba36000000 mov edx, 0x36
1420747e6: 4531c0 xor r8d, r8d
1420747e9: e852518600 call 0x1428d9940 ; core::str::slice_error_fail
1420747ee: 31c0 xor eax, eax
1420747f0: e9c5010000 jmp 0x1420749ba ; unknown
1420747f5: 75d7 jne 0x1420747ce
1420747f7: 4c8d3584e7f500 lea r14, [rip + 0xf5e784]
1420747fe: 4c89f1 mov rcx, r14
142074801: 4c89ca mov rdx, r9
142074804: e867a6ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
142074809: 4883f801 cmp rax, 1
14207480d: 7554 jne 0x142074863
14207480f: 4989d0 mov r8, rdx
142074812: 4983c002 add r8, 2
142074816: 7439 je 0x142074851
142074818: 4983f836 cmp r8, 0x36
14207481c: 7331 jae 0x14207484f
14207481e: 488d055de7f500 lea rax, [rip + 0xf5e75d]
142074825: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
14207482a: 7f25 jg 0x142074851
14207482c: 488d05bdebf500 lea rax, [rip + 0xf5ebbd]
142074833: 4889442420 mov qword ptr [rsp + 0x20], rax
142074838: 488d0d43e7f500 lea rcx, [rip + 0xf5e743]
14207483f: ba36000000 mov edx, 0x36
142074844: 41b936000000 mov r9d, 0x36
14207484a: e8f1508600 call 0x1428d9940 ; core::str::slice_error_fail
14207484f: 75db jne 0x14207482c
142074851: bb34000000 mov ebx, 0x34
142074856: 4829d3 sub rbx, rdx
142074859: 4c8d3522e7f500 lea r14, [rip + 0xf5e722]
142074860: 4d01c6 add r14, r8
142074863: 4c8d05f4ebf500 lea r8, [rip + 0xf5ebf4]
14207486a: 4c8d3dbfebf500 lea r15, [rip + 0xf5ebbf]
142074871: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142074880: 410fbe40ff movsx eax, byte ptr [r8 - 1]
142074885: 85c0 test eax, eax
142074887: 7817 js 0x1420748a0
142074889: 49ffc8 dec r8
14207488c: 83f85c cmp eax, 0x5c
14207488f: 7563 jne 0x1420748f4
142074891: eb73 jmp 0x142074906 ; unknown
142074893: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1420748a0: 410fb648fe movzx ecx, byte ptr [r8 - 2]
1420748a5: 80f9c0 cmp cl, 0xc0
1420748a8: 7d23 jge 0x1420748cd
1420748aa: 410fb650fd movzx edx, byte ptr [r8 - 3]
1420748af: 80fac0 cmp dl, 0xc0
1420748b2: 7d22 jge 0x1420748d6
1420748b4: 450fb648fc movzx r9d, byte ptr [r8 - 4]
1420748b9: 4983c0fc add r8, -4
1420748bd: 4183e107 and r9d, 7
1420748c1: 41c1e106 shl r9d, 6
1420748c5: 83e23f and edx, 0x3f
1420748c8: 4409ca or edx, r9d
1420748cb: eb10 jmp 0x1420748dd ; unknown
1420748cd: 4983c0fe add r8, -2
1420748d1: 83e11f and ecx, 0x1f
1420748d4: eb0f jmp 0x1420748e5 ; unknown
1420748d6: 4983c0fd add r8, -3
1420748da: 83e20f and edx, 0xf
1420748dd: c1e206 shl edx, 6
1420748e0: 83e13f and ecx, 0x3f
1420748e3: 09d1 or ecx, edx
1420748e5: c1e106 shl ecx, 6
1420748e8: 243f and al, 0x3f
1420748ea: 0fb6c0 movzx eax, al
1420748ed: 09c8 or eax, ecx
1420748ef: 83f85c cmp eax, 0x5c
1420748f2: 7412 je 0x142074906
1420748f4: 83f82f cmp eax, 0x2f
1420748f7: 740d je 0x142074906
1420748f9: 4d39f8 cmp r8, r15
1420748fc: 7582 jne 0x142074880
1420748fe: 41bc2e000000 mov r12d, 0x2e
142074904: eb4d jmp 0x142074953 ; unknown
142074906: 4c8d3d23ebf500 lea r15, [rip + 0xf5eb23]
14207490d: 4d29f8 sub r8, r15
142074910: 49ffc0 inc r8
142074913: 7432 je 0x142074947
142074915: 4983f82e cmp r8, 0x2e
142074919: 732a jae 0x142074945
14207491b: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
142074920: 7f25 jg 0x142074947
142074922: 488d058feaf500 lea rax, [rip + 0xf5ea8f]
142074929: 4889442420 mov qword ptr [rsp + 0x20], rax
14207492e: 488d0dfbeaf500 lea rcx, [rip + 0xf5eafb]
142074935: ba2e000000 mov edx, 0x2e
14207493a: 41b92e000000 mov r9d, 0x2e
142074940: e8fb4f8600 call 0x1428d9940 ; core::str::slice_error_fail
142074945: 75db jne 0x142074922
142074947: 41bc2e000000 mov r12d, 0x2e
14207494d: 4d29c4 sub r12, r8
142074950: 4d01c7 add r15, r8
142074953: e8d837bafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142074958: 4889c1 mov rcx, rax
14207495b: 488b00 mov rax, qword ptr [rax]
14207495e: 4883f801 cmp rax, 1
142074962: 0f85a3040000 jne 0x142074e0b
142074968: 4883c108 add rcx, 8
14207496c: 48833900 cmp qword ptr [rcx], 0
142074970: 0f85b9040000 jne 0x142074e2f
142074976: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207497d: 48894d40 mov qword ptr [rbp + 0x40], rcx
142074981: 4883c108 add rcx, 8
142074985: 4c89642420 mov qword ptr [rsp + 0x20], r12
14207498a: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
142074993: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14207499c: 4c89f2 mov rdx, r14
14207499f: 4989d8 mov r8, rbx
1420749a2: 4d89f9 mov r9, r15
1420749a5: e89623bafe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1420749aa: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
1420749ae: 48ff01 inc qword ptr [rcx]
1420749b1: 48894520 mov qword ptr [rbp + 0x20], rax
1420749b5: b801000000 mov eax, 1
1420749ba: 48894518 mov qword ptr [rbp + 0x18], rax
1420749be: 488b0e mov rcx, qword ptr [rsi]
1420749c1: b20a mov dl, 0xa
1420749c3: e888d291fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
1420749c8: 48895510 mov qword ptr [rbp + 0x10], rdx
1420749cc: 48894530 mov qword ptr [rbp + 0x30], rax
1420749d0: 488b0f mov rcx, qword ptr [rdi]
1420749d3: 4c8b7708 mov r14, qword ptr [rdi + 8]
1420749d7: 4c8bb9f8000000 mov r15, qword ptr [rcx + 0xf8]
1420749de: 488b8100010000 mov rax, qword ptr [rcx + 0x100]
1420749e5: 4d8d2487 lea r12, [r15 + rax*4]
1420749e9: 48894d38 mov qword ptr [rbp + 0x38], rcx
1420749ed: 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
1420749f4: 884540 mov byte ptr [rbp + 0x40], al
1420749f7: b808000000 mov eax, 8
1420749fc: 48894528 mov qword ptr [rbp + 0x28], rax
142074a00: 31f6 xor esi, esi
142074a02: f30f1035fa0e8b00 movss xmm6, dword ptr [rip + 0x8b0efa]
142074a0a: 0f283d6f9f8b00 movaps xmm7, xmmword ptr [rip + 0x8b9f6f]
142074a11: 4531ed xor r13d, r13d
142074a14: 31ff xor edi, edi
142074a16: 31db xor ebx, ebx
142074a18: 807d4000 cmp byte ptr [rbp + 0x40], 0
142074a1c: 0f85ee000000 jne 0x142074b10
142074a22: 4839fb cmp rbx, rdi
142074a25: 0f85d0000000 jne 0x142074afb
142074a2b: 0f1f440000 nop dword ptr [rax + rax]
142074a30: 4d39e7 cmp r15, r12
142074a33: 0f841c030000 je 0x142074d55
142074a39: 418b0f mov ecx, dword ptr [r15]
142074a3c: 4983c704 add r15, 4
142074a40: 498b8600010000 mov rax, qword ptr [r14 + 0x100]
142074a47: 488d0c89 lea rcx, [rcx + rcx*4]
142074a4b: 48c1e105 shl rcx, 5
142074a4f: 488b7c0858 mov rdi, qword ptr [rax + rcx + 0x58]
142074a54: 4885ff test rdi, rdi
142074a57: 74d7 je 0x142074a30
142074a59: 4801c8 add rax, rcx
142074a5c: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
142074a62: 498b96a8010000 mov rdx, qword ptr [r14 + 0x1a8]
142074a69: 4c8d0cc9 lea r9, [rcx + rcx*8]
142074a6d: 4a8d0cca lea rcx, [rdx + r9*8]
142074a71: 4c8b4538 mov r8, qword ptr [rbp + 0x38]
142074a75: 4d8b8010010000 mov r8, qword ptr [r8 + 0x110]
142074a7c: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
142074a81: 4c39c2 cmp rdx, r8
142074a84: 7633 jbe 0x142074ab9
142074a86: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
142074a8a: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
142074a8e: 4d85c0 test r8, r8
142074a91: 7426 je 0x142074ab9
142074a93: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
142074a97: 49f7d0 not r8
142074a9a: 4f8d0440 lea r8, [r8 + r8*2]
142074a9e: 49c1e004 shl r8, 4
142074aa2: 4b8b740110 mov rsi, qword ptr [r9 + r8 + 0x10]
142074aa7: 4c8b4538 mov r8, qword ptr [rbp + 0x38]
142074aab: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
142074ab2: 4c39c2 cmp rdx, r8
142074ab5: 7714 ja 0x142074acb
142074ab7: eb35 jmp 0x142074aee ; unknown
142074ab9: 31f6 xor esi, esi
142074abb: 4c8b4538 mov r8, qword ptr [rbp + 0x38]
142074abf: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
142074ac6: 4c39c2 cmp rdx, r8
142074ac9: 7623 jbe 0x142074aee
142074acb: 488b5138 mov rdx, qword ptr [rcx + 0x38]
142074acf: 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
142074ad3: 4885d2 test rdx, rdx
142074ad6: 7416 je 0x142074aee
142074ad8: 488b4918 mov rcx, qword ptr [rcx + 0x18]
142074adc: 48f7d2 not rdx
142074adf: 488d1452 lea rdx, [rdx + rdx*2]
142074ae3: 48c1e204 shl rdx, 4
142074ae7: 4c8b6c1110 mov r13, qword ptr [rcx + rdx + 0x10]
142074aec: eb03 jmp 0x142074af1 ; unknown
142074aee: 4531ed xor r13d, r13d
142074af1: 488b4050 mov rax, qword ptr [rax + 0x50]
142074af5: 48894528 mov qword ptr [rbp + 0x28], rax
142074af9: 31db xor ebx, ebx
142074afb: 4889d8 mov rax, rbx
142074afe: 48c1e004 shl rax, 4
142074b02: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
142074b06: 8b440108 mov eax, dword ptr [rcx + rax + 8]
142074b0a: e9bd000000 jmp 0x142074bcc ; unknown
142074b0f: 90 nop 
142074b10: 4839fb cmp rbx, rdi
142074b13: 0f85b1000000 jne 0x142074bca
142074b19: 0f1f8000000000 nop dword ptr [rax]
142074b20: 4d39e7 cmp r15, r12
142074b23: 0f842c020000 je 0x142074d55
142074b29: 418b0f mov ecx, dword ptr [r15]
142074b2c: 4983c704 add r15, 4
142074b30: 498b86a8010000 mov rax, qword ptr [r14 + 0x1a8]
142074b37: 488d0cc9 lea rcx, [rcx + rcx*8]
142074b3b: 488b7cc810 mov rdi, qword ptr [rax + rcx*8 + 0x10]
142074b40: 4885ff test rdi, rdi
142074b43: 74db je 0x142074b20
142074b45: 488d04c8 lea rax, [rax + rcx*8]
142074b49: 488b4d38 mov rcx, qword ptr [rbp + 0x38]
142074b4d: 488b9110010000 mov rdx, qword ptr [rcx + 0x110]
142074b54: 488b4840 mov rcx, qword ptr [rax + 0x40]
142074b58: 4839d1 cmp rcx, rdx
142074b5b: 7633 jbe 0x142074b90
142074b5d: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142074b61: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142074b65: 4885d2 test rdx, rdx
142074b68: 7426 je 0x142074b90
142074b6a: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142074b6e: 48f7d2 not rdx
142074b71: 488d1452 lea rdx, [rdx + rdx*2]
142074b75: 48c1e204 shl rdx, 4
142074b79: 498b741010 mov rsi, qword ptr [r8 + rdx + 0x10]
142074b7e: 488b5538 mov rdx, qword ptr [rbp + 0x38]
142074b82: 488b9218010000 mov rdx, qword ptr [rdx + 0x118]
142074b89: 4839d1 cmp rcx, rdx
142074b8c: 7714 ja 0x142074ba2
142074b8e: eb35 jmp 0x142074bc5 ; unknown
142074b90: 31f6 xor esi, esi
142074b92: 488b5538 mov rdx, qword ptr [rbp + 0x38]
142074b96: 488b9218010000 mov rdx, qword ptr [rdx + 0x118]
142074b9d: 4839d1 cmp rcx, rdx
142074ba0: 7623 jbe 0x142074bc5
142074ba2: 488b4838 mov rcx, qword ptr [rax + 0x38]
142074ba6: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
142074baa: 4885c9 test rcx, rcx
142074bad: 7416 je 0x142074bc5
142074baf: 488b4018 mov rax, qword ptr [rax + 0x18]
142074bb3: 48f7d1 not rcx
142074bb6: 488d0c49 lea rcx, [rcx + rcx*2]
142074bba: 48c1e104 shl rcx, 4
142074bbe: 4c8b6c0810 mov r13, qword ptr [rax + rcx + 0x10]
142074bc3: eb03 jmp 0x142074bc8 ; unknown
142074bc5: 4531ed xor r13d, r13d
142074bc8: 31db xor ebx, ebx
142074bca: 89d8 mov eax, ebx
142074bcc: 488d0440 lea rax, [rax + rax*2]
142074bd0: 488d1486 lea rdx, [rsi + rax*4]
142074bd4: 48c1e004 shl rax, 4
142074bd8: 4c01e8 add rax, r13
142074bdb: 8b4818 mov ecx, dword ptr [rax + 0x18]
142074bde: 894d08 mov dword ptr [rbp + 8], ecx
142074be1: 488b4810 mov rcx, qword ptr [rax + 0x10]
142074be5: 48894d00 mov qword ptr [rbp], rcx
142074be9: 0f2820 movaps xmm4, xmmword ptr [rax]
142074bec: f30f105808 movss xmm3, dword ptr [rax + 8]
142074bf1: 440f28c3 movaps xmm8, xmm3
142074bf5: f3440f58c3 addss xmm8, xmm3
142074bfa: f3410f59d8 mulss xmm3, xmm8
142074bff: 0f28cc movaps xmm1, xmm4
142074c02: 0f58cc addps xmm1, xmm4
142074c05: f30f16c1 movshdup xmm0, xmm1
142074c09: f30f59c4 mulss xmm0, xmm4
142074c0d: 0f28d4 movaps xmm2, xmm4
142074c10: f3410f12e8 movsldup xmm5, xmm8
142074c15: 0f59ec mulps xmm5, xmm4
142074c18: 0fc6e4ff shufps xmm4, xmm4, 0xff
142074c1c: 0f59d1 mulps xmm2, xmm1
142074c1f: 0f59cc mulps xmm1, xmm4
142074c22: 440f28cc movaps xmm9, xmm4
142074c26: f3450f59c8 mulss xmm9, xmm8
142074c2b: f3440f16d2 movshdup xmm10, xmm2
142074c30: 450f28c2 movaps xmm8, xmm10
142074c34: f3440f58c3 addss xmm8, xmm3
142074c39: 0f28e6 movaps xmm4, xmm6
142074c3c: f3410f5ce0 subss xmm4, xmm8
142074c41: 440f28c0 movaps xmm8, xmm0
142074c45: f3450f58c1 addss xmm8, xmm9
142074c4a: 410f14e0 unpcklps xmm4, xmm8
142074c4e: f3410f5cc1 subss xmm0, xmm9
142074c53: f30f58da addss xmm3, xmm2
142074c57: 440f28c6 movaps xmm8, xmm6
142074c5b: f3440f5cc3 subss xmm8, xmm3
142074c60: 0fc6c9e1 shufps xmm1, xmm1, 0xe1
142074c64: 410f14c0 unpcklps xmm0, xmm8
142074c68: 440f28c5 movaps xmm8, xmm5
142074c6c: 440f5cc1 subps xmm8, xmm1
142074c70: 0f58cd addps xmm1, xmm5
142074c73: f30f105820 movss xmm3, dword ptr [rax + 0x20]
142074c78: f3410f58d2 addss xmm2, xmm10
142074c7d: 440f28ce movaps xmm9, xmm6
142074c81: f3440f5cca subss xmm9, xmm2
142074c86: f30f106824 movss xmm5, dword ptr [rax + 0x24]
142074c8b: 410f16e0 movlhps xmm4, xmm8
142074c8f: 0fc6c1d4 shufps xmm0, xmm1, 0xd4
142074c93: 440fc6c101 shufps xmm8, xmm1, 1
142074c98: 440fc6c930 shufps xmm9, xmm1, 0x30
142074c9d: 0f10501c movups xmm2, xmmword ptr [rax + 0x1c]
142074ca1: 450fc6c182 shufps xmm8, xmm9, 0x82
142074ca6: 0f104820 movups xmm1, xmmword ptr [rax + 0x20]
142074caa: 0fc6d200 shufps xmm2, xmm2, 0
142074cae: 0f59d4 mulps xmm2, xmm4
142074cb1: 0f28e1 movaps xmm4, xmm1
142074cb4: 0fc6e100 shufps xmm4, xmm1, 0
142074cb8: 0f59e0 mulps xmm4, xmm0
142074cbb: 0fc6c955 shufps xmm1, xmm1, 0x55
142074cbf: 410f59c8 mulps xmm1, xmm8
142074cc3: 0f54df andps xmm3, xmm7
142074cc6: 0f54ef andps xmm5, xmm7
142074cc9: 0f28c3 movaps xmm0, xmm3
142074ccc: f30fc2c303 cmpunordss xmm0, xmm3
142074cd1: 440f28c0 movaps xmm8, xmm0
142074cd5: 440f54c5 andps xmm8, xmm5
142074cd9: f30f5feb maxss xmm5, xmm3
142074cdd: f30f10581c movss xmm3, dword ptr [rax + 0x1c]
142074ce2: 0f54df andps xmm3, xmm7
142074ce5: 0f55c5 andnps xmm0, xmm5
142074ce8: 410f56c0 orps xmm0, xmm8
142074cec: 0f28e8 movaps xmm5, xmm0
142074cef: f30f5feb maxss xmm5, xmm3
142074cf3: f30fc2db03 cmpunordss xmm3, xmm3
142074cf8: 440f28c3 movaps xmm8, xmm3
142074cfc: 440f54c0 andps xmm8, xmm0
142074d00: 0f55dd andnps xmm3, xmm5
142074d03: 410f56d8 orps xmm3, xmm8
142074d07: 0f28c2 movaps xmm0, xmm2
142074d0a: f20f10c4 movsd xmm0, xmm4
142074d0e: 0fc6d024 shufps xmm2, xmm0, 0x24
142074d12: 0f284010 movaps xmm0, xmmword ptr [rax + 0x10]
142074d16: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
142074d1a: 0fc6e149 shufps xmm4, xmm1, 0x49
142074d1e: 0f2965d0 movaps xmmword ptr [rbp - 0x30], xmm4
142074d22: 0fc6c802 shufps xmm1, xmm0, 2
142074d26: 0fc6c898 shufps xmm1, xmm0, 0x98
142074d2a: 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
142074d2e: 488d45c0 lea rax, [rbp - 0x40]
142074d32: 4889442420 mov qword ptr [rsp + 0x20], rax
142074d37: 488b4d10 mov rcx, qword ptr [rbp + 0x10]
142074d3b: 4989e8 mov r8, rbp
142074d3e: e8adb24fff call 0x14156fff0 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::PlantInstanceData>
142074d43: 48ffc3 inc rbx
142074d46: 807d4000 cmp byte ptr [rbp + 0x40], 0
142074d4a: 0f84d2fcffff je 0x142074a22
142074d50: e9bbfdffff jmp 0x142074b10 ; unknown
142074d55: 488b7530 mov rsi, qword ptr [rbp + 0x30]
142074d59: 4889f1 mov rcx, rsi
142074d5c: e8afd0c0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
142074d61: 4889f1 mov rcx, rsi
142074d64: 4883c901 or rcx, 1
142074d68: e8a3d0c0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
142074d6d: 31c9 xor ecx, ecx
142074d6f: b808000000 mov eax, 8
142074d74: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
142074d79: 0f85bc000000 jne 0x142074e3b
142074d7f: 48837d1800 cmp qword ptr [rbp + 0x18], 0
142074d84: 743e je 0x142074dc4
142074d86: e8a533bafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142074d8b: 4889c1 mov rcx, rax
142074d8e: 488b00 mov rax, qword ptr [rax]
142074d91: 4883f801 cmp rax, 1
142074d95: 755e jne 0x142074df5
142074d97: 4883c108 add rcx, 8
142074d9b: 48833900 cmp qword ptr [rcx], 0
142074d9f: 0f858a000000 jne 0x142074e2f
142074da5: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142074dac: 48894d40 mov qword ptr [rbp + 0x40], rcx
142074db0: 4883c108 add rcx, 8
142074db4: 488b5520 mov rdx, qword ptr [rbp + 0x20]
142074db8: e83323bafe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
142074dbd: 488b4540 mov rax, qword ptr [rbp + 0x40]
142074dc1: 48ff00 inc qword ptr [rax]
142074dc4: 0f287550 movaps xmm6, xmmword ptr [rbp + 0x50]
142074dc8: 0f287d60 movaps xmm7, xmmword ptr [rbp + 0x60]
142074dcc: 440f284570 movaps xmm8, xmmword ptr [rbp + 0x70]
142074dd1: 440f288d80000000 movaps xmm9, xmmword ptr [rbp + 0x80]
142074dd9: 440f289590000000 movaps xmm10, xmmword ptr [rbp + 0x90]
142074de1: 4881c428010000 add rsp, 0x128
142074de8: 5b pop rbx
142074de9: 5f pop rdi
142074dea: 5e pop rsi
142074deb: 415c pop r12
142074ded: 415d pop r13
142074def: 415e pop r14
142074df1: 415f pop r15
142074df3: 5d pop rbp
142074df4: c3 ret 
142074df5: 4885c0 test rax, rax
142074df8: 7529 jne 0x142074e23
142074dfa: 31d2 xor edx, edx
142074dfc: e8cf398800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142074e01: 4889c1 mov rcx, rax
142074e04: 4885c0 test rax, rax
142074e07: 7592 jne 0x142074d9b
142074e09: eb18 jmp 0x142074e23 ; unknown
142074e0b: 4885c0 test rax, rax
142074e0e: 7513 jne 0x142074e23
142074e10: 31d2 xor edx, edx
142074e12: e8b9398800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142074e17: 4889c1 mov rcx, rax
142074e1a: 4885c0 test rax, rax
142074e1d: 0f8549fbffff jne 0x14207496c
142074e23: 488d0d765cbc00 lea rcx, [rip + 0xbc5c76]
142074e2a: e8a1328600 call 0x1428d80d0 ; std::thread::local::panic_access_error
142074e2f: 488d0d3a5dbc00 lea rcx, [rip + 0xbc5d3a]
142074e36: e815448600 call 0x1428d9250 ; core::cell::panic_already_borrowed
142074e3b: 4889f1 mov rcx, rsi
142074e3e: 31d2 xor edx, edx
142074e40: e81b718500 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
142074e45: e935ffffff jmp 0x142074d7f ; unknown
142074e4a: 660f1f440000 nop word ptr [rax + rax]
142074e50: 4889542410 mov qword ptr [rsp + 0x10], rdx
142074e55: 55 push rbp
142074e56: 4157 push r15
142074e58: 4156 push r14
142074e5a: 4155 push r13
142074e5c: 4154 push r12
142074e5e: 56 push rsi
142074e5f: 57 push rdi
142074e60: 53 push rbx
142074e61: 4881ec88000000 sub rsp, 0x88
142074e68: 488daa80000000 lea rbp, [rdx + 0x80]
142074e6f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142074e75: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
142074e7b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142074e81: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142074e86: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
142074e8b: 488b4540 mov rax, qword ptr [rbp + 0x40]
142074e8f: 48ff00 inc qword ptr [rax]
142074e92: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142074e97: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
142074e9c: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142074ea2: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142074ea8: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
142074eae: 4881c488000000 add rsp, 0x88
142074eb5: 5b pop rbx
142074eb6: 5f pop rdi
142074eb7: 5e pop rsi
142074eb8: 415c pop r12
142074eba: 415d pop r13
142074ebc: 415e pop r14
142074ebe: 415f pop r15
142074ec0: 5d pop rbp
142074ec1: c3 ret 
142074ec2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142074ed0: 4889542410 mov qword ptr [rsp + 0x10], rdx
142074ed5: 55 push rbp
142074ed6: 4157 push r15
142074ed8: 4156 push r14
142074eda: 4155 push r13
142074edc: 4154 push r12
142074ede: 56 push rsi
142074edf: 57 push rdi
142074ee0: 53 push rbx
142074ee1: 4881ec88000000 sub rsp, 0x88
142074ee8: 488daa80000000 lea rbp, [rdx + 0x80]
142074eef: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142074ef5: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
142074efb: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142074f01: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142074f06: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
142074f0b: 488b4540 mov rax, qword ptr [rbp + 0x40]
142074f0f: 48ff00 inc qword ptr [rax]
142074f12: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142074f17: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
142074f1c: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142074f22: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142074f28: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
142074f2e: 4881c488000000 add rsp, 0x88
142074f35: 5b pop rbx
142074f36: 5f pop rdi
142074f37: 5e pop rsi
142074f38: 415c pop r12
142074f3a: 415d pop r13
142074f3c: 415e pop r14
142074f3e: 415f pop r15
142074f40: 5d pop rbp
142074f41: c3 ret 
142074f42: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142074f50: 4889542410 mov qword ptr [rsp + 0x10], rdx
142074f55: 55 push rbp
142074f56: 4157 push r15
142074f58: 4156 push r14
142074f5a: 4155 push r13
142074f5c: 4154 push r12
142074f5e: 56 push rsi
142074f5f: 57 push rdi
142074f60: 53 push rbx
142074f61: 4881ec88000000 sub rsp, 0x88
142074f68: 488daa80000000 lea rbp, [rdx + 0x80]
142074f6f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142074f75: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
142074f7b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142074f81: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142074f86: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
142074f8b: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
142074f8f: e83cc3fcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
142074f94: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142074f99: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
142074f9e: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142074fa4: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142074faa: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
142074fb0: 4881c488000000 add rsp, 0x88
142074fb7: 5b pop rbx
142074fb8: 5f pop rdi
142074fb9: 5e pop rsi
142074fba: 415c pop r12
142074fbc: 415d pop r13
142074fbe: 415e pop r14
142074fc0: 415f pop r15
142074fc2: 5d pop rbp
142074fc3: c3 ret 
142074fc4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142074fd0: 4889542410 mov qword ptr [rsp + 0x10], rdx
142074fd5: 55 push rbp
142074fd6: 4157 push r15
142074fd8: 4156 push r14
142074fda: 4155 push r13
142074fdc: 4154 push r12
142074fde: 56 push rsi
142074fdf: 57 push rdi
142074fe0: 53 push rbx
142074fe1: 4881ec88000000 sub rsp, 0x88
142074fe8: 488daa80000000 lea rbp, [rdx + 0x80]
142074fef: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142074ff5: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
142074ffb: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142075001: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142075006: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
14207500b: 48837d1800 cmp qword ptr [rbp + 0x18], 0
142075010: 7420 je 0x142075032
142075012: 488d4520 lea rax, [rbp + 0x20]
142075016: 488945f8 mov qword ptr [rbp - 8], rax
14207501a: 488d45f8 lea rax, [rbp - 8]
14207501e: 488945f0 mov qword ptr [rbp - 0x10], rax
142075022: 488d0d2fe3f500 lea rcx, [rip + 0xf5e32f]
142075029: 488d55f0 lea rdx, [rbp - 0x10]
14207502d: e89ea22bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
142075032: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142075037: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
14207503c: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142075042: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142075048: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
14207504e: 4881c488000000 add rsp, 0x88
142075055: 5b pop rbx
142075056: 5f pop rdi
142075057: 5e pop rsi
142075058: 415c pop r12
14207505a: 415d pop r13
14207505c: 415e pop r14
14207505e: 415f pop r15
142075060: 5d pop rbp
142075061: c3 ret 
