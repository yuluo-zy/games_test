142073690: 55 push rbp
142073691: 4157 push r15
142073693: 4156 push r14
142073695: 4155 push r13
142073697: 4154 push r12
142073699: 56 push rsi
14207369a: 57 push rdi
14207369b: 53 push rbx
14207369c: 4881ec08020000 sub rsp, 0x208
1420736a3: 488dac2480000000 lea rbp, [rsp + 0x80]
1420736ab: 440f29bd70010000 movaps xmmword ptr [rbp + 0x170], xmm15
1420736b3: 440f29b560010000 movaps xmmword ptr [rbp + 0x160], xmm14
1420736bb: 440f29ad50010000 movaps xmmword ptr [rbp + 0x150], xmm13
1420736c3: 440f29a540010000 movaps xmmword ptr [rbp + 0x140], xmm12
1420736cb: 440f299d30010000 movaps xmmword ptr [rbp + 0x130], xmm11
1420736d3: 440f299520010000 movaps xmmword ptr [rbp + 0x120], xmm10
1420736db: 440f298d10010000 movaps xmmword ptr [rbp + 0x110], xmm9
1420736e3: 440f298500010000 movaps xmmword ptr [rbp + 0x100], xmm8
1420736eb: 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
1420736f2: 0f29b5e0000000 movaps xmmword ptr [rbp + 0xe0], xmm6
1420736f9: 48c785d8000000feffffff mov qword ptr [rbp + 0xd8], 0xfffffffffffffffe
142073704: 4d89ce mov r14, r9
142073707: 4d89c7 mov r15, r8
14207370a: 4889d6 mov rsi, rdx
14207370d: 4889cb mov rbx, rcx
142073710: e8eb32bafe call 0x140c16a00 ; puffin::are_scopes_on
142073715: 84c0 test al, al
142073717: 745f je 0x142073778
142073719: 4c8d25d1f6f500 lea r12, [rip + 0xf5f6d1]
142073720: bf41000000 mov edi, 0x41
142073725: ba41000000 mov edx, 0x41
14207372a: 4c89e1 mov rcx, r12
14207372d: e83eb7ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
142073732: 4883f801 cmp rax, 1
142073736: 0f85b1000000 jne 0x1420737ed
14207373c: 4989d1 mov r9, rdx
14207373f: 4885d2 test rdx, rdx
142073742: 743d je 0x142073781
142073744: 4983f941 cmp r9, 0x41
142073748: 7335 jae 0x14207377f
14207374a: 488d05a0f6f500 lea rax, [rip + 0xf5f6a0]
142073751: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
142073756: 7f29 jg 0x142073781
142073758: 488d0579fcf500 lea rax, [rip + 0xf5fc79]
14207375f: 4889442420 mov qword ptr [rsp + 0x20], rax
142073764: 488d0d86f6f500 lea rcx, [rip + 0xf5f686]
14207376b: ba41000000 mov edx, 0x41
142073770: 4531c0 xor r8d, r8d
142073773: e8c8618600 call 0x1428d9940 ; core::str::slice_error_fail
142073778: 31c0 xor eax, eax
14207377a: e9e4010000 jmp 0x142073963 ; unknown
14207377f: 75d7 jne 0x142073758
142073781: 4c8d2569f6f500 lea r12, [rip + 0xf5f669]
142073788: 4c89e1 mov rcx, r12
14207378b: 4c89ca mov rdx, r9
14207378e: e8ddb6ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
142073793: 4883f801 cmp rax, 1
142073797: 7554 jne 0x1420737ed
142073799: 4989d0 mov r8, rdx
14207379c: 4983c002 add r8, 2
1420737a0: 7439 je 0x1420737db
1420737a2: 4983f841 cmp r8, 0x41
1420737a6: 7331 jae 0x1420737d9
1420737a8: 488d0542f6f500 lea rax, [rip + 0xf5f642]
1420737af: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
1420737b4: 7f25 jg 0x1420737db
1420737b6: 488d0533fcf500 lea rax, [rip + 0xf5fc33]
1420737bd: 4889442420 mov qword ptr [rsp + 0x20], rax
1420737c2: 488d0d28f6f500 lea rcx, [rip + 0xf5f628]
1420737c9: ba41000000 mov edx, 0x41
1420737ce: 41b941000000 mov r9d, 0x41
1420737d4: e867618600 call 0x1428d9940 ; core::str::slice_error_fail
1420737d9: 75db jne 0x1420737b6
1420737db: bf3f000000 mov edi, 0x3f
1420737e0: 4829d7 sub rdi, rdx
1420737e3: 4c8d2507f6f500 lea r12, [rip + 0xf5f607]
1420737ea: 4d01c4 add r12, r8
1420737ed: 4c8d056afcf500 lea r8, [rip + 0xf5fc6a]
1420737f4: 4c8d2d35fcf500 lea r13, [rip + 0xf5fc35]
1420737fb: 48899dc0000000 mov qword ptr [rbp + 0xc0], rbx
142073802: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142073810: 410fbe40ff movsx eax, byte ptr [r8 - 1]
142073815: 85c0 test eax, eax
142073817: 7817 js 0x142073830
142073819: 49ffc8 dec r8
14207381c: 83f85c cmp eax, 0x5c
14207381f: 7563 jne 0x142073884
142073821: eb76 jmp 0x142073899 ; unknown
142073823: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142073830: 410fb648fe movzx ecx, byte ptr [r8 - 2]
142073835: 80f9c0 cmp cl, 0xc0
142073838: 7d23 jge 0x14207385d
14207383a: 410fb650fd movzx edx, byte ptr [r8 - 3]
14207383f: 80fac0 cmp dl, 0xc0
142073842: 7d22 jge 0x142073866
142073844: 450fb648fc movzx r9d, byte ptr [r8 - 4]
142073849: 4983c0fc add r8, -4
14207384d: 4183e107 and r9d, 7
142073851: 41c1e106 shl r9d, 6
142073855: 83e23f and edx, 0x3f
142073858: 4409ca or edx, r9d
14207385b: eb10 jmp 0x14207386d ; unknown
14207385d: 4983c0fe add r8, -2
142073861: 83e11f and ecx, 0x1f
142073864: eb0f jmp 0x142073875 ; unknown
142073866: 4983c0fd add r8, -3
14207386a: 83e20f and edx, 0xf
14207386d: c1e206 shl edx, 6
142073870: 83e13f and ecx, 0x3f
142073873: 09d1 or ecx, edx
142073875: c1e106 shl ecx, 6
142073878: 243f and al, 0x3f
14207387a: 0fb6c0 movzx eax, al
14207387d: 09c8 or eax, ecx
14207387f: 83f85c cmp eax, 0x5c
142073882: 7415 je 0x142073899
142073884: 83f82f cmp eax, 0x2f
142073887: 7410 je 0x142073899
142073889: 4d39e8 cmp r8, r13
14207388c: 7582 jne 0x142073810
14207388e: 4c89fb mov rbx, r15
142073891: 41bf2e000000 mov r15d, 0x2e
142073897: eb50 jmp 0x1420738e9 ; unknown
142073899: 4c8d2d90fbf500 lea r13, [rip + 0xf5fb90]
1420738a0: 4d29e8 sub r8, r13
1420738a3: 49ffc0 inc r8
1420738a6: 7432 je 0x1420738da
1420738a8: 4983f82e cmp r8, 0x2e
1420738ac: 732a jae 0x1420738d8
1420738ae: 43803c28bf cmp byte ptr [r8 + r13], 0xbf
1420738b3: 7f25 jg 0x1420738da
1420738b5: 488d05fcfaf500 lea rax, [rip + 0xf5fafc]
1420738bc: 4889442420 mov qword ptr [rsp + 0x20], rax
1420738c1: 488d0d68fbf500 lea rcx, [rip + 0xf5fb68]
1420738c8: ba2e000000 mov edx, 0x2e
1420738cd: 41b92e000000 mov r9d, 0x2e
1420738d3: e868608600 call 0x1428d9940 ; core::str::slice_error_fail
1420738d8: 75db jne 0x1420738b5
1420738da: 4c89fb mov rbx, r15
1420738dd: 41bf2e000000 mov r15d, 0x2e
1420738e3: 4d29c7 sub r15, r8
1420738e6: 4d01c5 add r13, r8
1420738e9: e84248bafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1420738ee: 4889c1 mov rcx, rax
1420738f1: 488b00 mov rax, qword ptr [rax]
1420738f4: 4883f801 cmp rax, 1
1420738f8: 0f85300a0000 jne 0x14207432e
1420738fe: 4883c108 add rcx, 8
142073902: 48833900 cmp qword ptr [rcx], 0
142073906: 0f85460a0000 jne 0x142074352
14207390c: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142073913: 48898db0000000 mov qword ptr [rbp + 0xb0], rcx
14207391a: 4883c108 add rcx, 8
14207391e: 4c897c2420 mov qword ptr [rsp + 0x20], r15
142073923: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
14207392c: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
142073935: 4c89e2 mov rdx, r12
142073938: 4989f8 mov r8, rdi
14207393b: 4d89e9 mov r9, r13
14207393e: e8fd33bafe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
142073943: 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
14207394a: 48ff01 inc qword ptr [rcx]
14207394d: 48898588000000 mov qword ptr [rbp + 0x88], rax
142073954: b801000000 mov eax, 1
142073959: 4989df mov r15, rbx
14207395c: 488b9dc0000000 mov rbx, qword ptr [rbp + 0xc0]
142073963: 48898580000000 mov qword ptr [rbp + 0x80], rax
14207396a: 488b0e mov rcx, qword ptr [rsi]
14207396d: b20e mov dl, 0xe
14207396f: e8dce291fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
142073974: 48899590000000 mov qword ptr [rbp + 0x90], rdx
14207397b: 488985a8000000 mov qword ptr [rbp + 0xa8], rax
142073982: 498b0f mov rcx, qword ptr [r15]
142073985: 488d151ca9bb00 lea rdx, [rip + 0xbba91c]
14207398c: e88f5d8ffe call 0x140969720 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::ha
142073991: 4889c7 mov rdi, rax
142073994: 4885c0 test rax, rax
142073997: 0f840d0a0000 je 0x1420743aa
14207399d: 488d051d9ba200 lea rax, [rip + 0xa29b1d]
1420739a4: 48894550 mov qword ptr [rbp + 0x50], rax
1420739a8: 48c745583f000000 mov qword ptr [rbp + 0x58], 0x3f
1420739b0: 4989ff mov r15, rdi
1420739b3: 4983c710 add r15, 0x10
1420739b7: 48837f183f cmp qword ptr [rdi + 0x18], 0x3f
1420739bc: 0f859c090000 jne 0x14207435e
1420739c2: 498b17 mov rdx, qword ptr [r15]
1420739c5: 488d0df59aa200 lea rcx, [rip + 0xa29af5]
1420739cc: 41b83f000000 mov r8d, 0x3f
1420739d2: e819028b00 call 0x142923bf0 ; unknown
1420739d7: 85c0 test eax, eax
1420739d9: 0f857f090000 jne 0x14207435e
1420739df: 488b0f mov rcx, qword ptr [rdi]
1420739e2: 488b4708 mov rax, qword ptr [rdi + 8]
1420739e6: ff5018 call qword ptr [rax + 0x18]
1420739e9: 4885c0 test rax, rax
1420739ec: 0f84b8090000 je 0x1420743aa
1420739f2: 488b8df0010000 mov rcx, qword ptr [rbp + 0x1f0]
1420739f9: 488b10 mov rdx, qword ptr [rax]
1420739fc: 48895528 mov qword ptr [rbp + 0x28], rdx
142073a00: 488b4008 mov rax, qword ptr [rax + 8]
142073a04: 48898598000000 mov qword ptr [rbp + 0x98], rax
142073a0b: 488b01 mov rax, qword ptr [rcx]
142073a0e: 488d4808 lea rcx, [rax + 8]
142073a12: 488d5020 lea rdx, [rax + 0x20]
142073a16: 4c8d4010 lea r8, [rax + 0x10]
142073a1a: 4c8d4828 lea r9, [rax + 0x28]
142073a1e: 80b89802000000 cmp byte ptr [rax + 0x298], 0
142073a25: 480f45ca cmovne rcx, rdx
142073a29: 4d0f44c8 cmove r9, r8
142073a2d: 498b01 mov rax, qword ptr [r9]
142073a30: 488b09 mov rcx, qword ptr [rcx]
142073a33: 48894db8 mov qword ptr [rbp - 0x48], rcx
142073a37: 488945c0 mov qword ptr [rbp - 0x40], rax
142073a3b: 498b06 mov rax, qword ptr [r14]
142073a3e: 498b5620 mov rdx, qword ptr [r14 + 0x20]
142073a42: 488d4808 lea rcx, [rax + 8]
142073a46: 4c8d4020 lea r8, [rax + 0x20]
142073a4a: 4c8d4810 lea r9, [rax + 0x10]
142073a4e: 4c8d5028 lea r10, [rax + 0x28]
142073a52: 80b89802000000 cmp byte ptr [rax + 0x298], 0
142073a59: 490f45c8 cmovne rcx, r8
142073a5d: 4d0f44d1 cmove r10, r9
142073a61: 498b02 mov rax, qword ptr [r10]
142073a64: 488b09 mov rcx, qword ptr [rcx]
142073a67: 4c8d4208 lea r8, [rdx + 8]
142073a6b: 4c8d4a20 lea r9, [rdx + 0x20]
142073a6f: 4c8d5210 lea r10, [rdx + 0x10]
142073a73: 4c8d5a28 lea r11, [rdx + 0x28]
142073a77: 80ba9802000000 cmp byte ptr [rdx + 0x298], 0
142073a7e: 4d0f45c1 cmovne r8, r9
142073a82: 4d0f44da cmove r11, r10
142073a86: 498b13 mov rdx, qword ptr [r11]
142073a89: 4d8b00 mov r8, qword ptr [r8]
142073a8c: 4d8b4e40 mov r9, qword ptr [r14 + 0x40]
142073a90: 4d8d5108 lea r10, [r9 + 8]
142073a94: 4d8d5920 lea r11, [r9 + 0x20]
142073a98: 498d7110 lea rsi, [r9 + 0x10]
142073a9c: 4180b99802000000 cmp byte ptr [r9 + 0x298], 0
142073aa4: 4d0f45d3 cmovne r10, r11
142073aa8: 4d8d4928 lea r9, [r9 + 0x28]
142073aac: 4c0f44ce cmove r9, rsi
142073ab0: 4d8b09 mov r9, qword ptr [r9]
142073ab3: 4d8b12 mov r10, qword ptr [r10]
142073ab6: 4d8b5e60 mov r11, qword ptr [r14 + 0x60]
142073aba: 498d7308 lea rsi, [r11 + 8]
142073abe: 498d7b20 lea rdi, [r11 + 0x20]
142073ac2: 4d8d7310 lea r14, [r11 + 0x10]
142073ac6: 4180bb9802000000 cmp byte ptr [r11 + 0x298], 0
142073ace: 4d8d5b28 lea r11, [r11 + 0x28]
142073ad2: 480f45f7 cmovne rsi, rdi
142073ad6: 4d0f44de cmove r11, r14
142073ada: 4d8b1b mov r11, qword ptr [r11]
142073add: 488b36 mov rsi, qword ptr [rsi]
142073ae0: 48894dc8 mov qword ptr [rbp - 0x38], rcx
142073ae4: 488945d0 mov qword ptr [rbp - 0x30], rax
142073ae8: 4c8945d8 mov qword ptr [rbp - 0x28], r8
142073aec: 488955e0 mov qword ptr [rbp - 0x20], rdx
142073af0: 4c8955e8 mov qword ptr [rbp - 0x18], r10
142073af4: 4c894df0 mov qword ptr [rbp - 0x10], r9
142073af8: 488975f8 mov qword ptr [rbp - 8], rsi
142073afc: 4c895d00 mov qword ptr [rbp], r11
142073b00: 488b0b mov rcx, qword ptr [rbx]
142073b03: 4c8b7308 mov r14, qword ptr [rbx + 8]
142073b07: 488bb1f8000000 mov rsi, qword ptr [rcx + 0xf8]
142073b0e: 488b8100010000 mov rax, qword ptr [rcx + 0x100]
142073b15: 488d1c86 lea rbx, [rsi + rax*4]
142073b19: 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
142073b20: 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
142073b27: 8885b0000000 mov byte ptr [rbp + 0xb0], al
142073b2d: b808000000 mov eax, 8
142073b32: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
142073b39: f30f1035177ba000 movss xmm6, dword ptr [rip + 0xa07b17]
142073b41: 450f57ed xorps xmm13, xmm13
142073b45: f3440f1035b61d8b00 movss xmm14, dword ptr [rip + 0x8b1db6]
142073b4e: f3440f102579cc9200 movss xmm12, dword ptr [rip + 0x92cc79]
142073b57: 48c785c800000000000000 mov qword ptr [rbp + 0xc8], 0
142073b62: 48c785d000000000000000 mov qword ptr [rbp + 0xd0], 0
142073b6d: 31ff xor edi, edi
142073b6f: 4531e4 xor r12d, r12d
142073b72: 80bdb000000000 cmp byte ptr [rbp + 0xb0], 0
142073b79: 0f84a1000000 je 0x142073c20
142073b7f: 4939fc cmp r12, rdi
142073b82: 0f85e2010000 jne 0x142073d6a
142073b88: 0f1f840000000000 nop dword ptr [rax + rax]
142073b90: 4839de cmp rsi, rbx
142073b93: 0f8497060000 je 0x142074230
142073b99: 8b0e mov ecx, dword ptr [rsi]
142073b9b: 4883c604 add rsi, 4
142073b9f: 498b86a8010000 mov rax, qword ptr [r14 + 0x1a8]
142073ba6: 488d0cc9 lea rcx, [rcx + rcx*8]
142073baa: 488b7cc810 mov rdi, qword ptr [rax + rcx*8 + 0x10]
142073baf: 4885ff test rdi, rdi
142073bb2: 74dc je 0x142073b90
142073bb4: 488d04c8 lea rax, [rax + rcx*8]
142073bb8: 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
142073bbf: 488b9110010000 mov rdx, qword ptr [rcx + 0x110]
142073bc6: 488b4840 mov rcx, qword ptr [rax + 0x40]
142073bca: 4839d1 cmp rcx, rdx
142073bcd: 0f86f9000000 jbe 0x142073ccc
142073bd3: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142073bd7: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142073bdb: 4885d2 test rdx, rdx
142073bde: 0f84e8000000 je 0x142073ccc
142073be4: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142073be8: 48f7d2 not rdx
142073beb: 488d1452 lea rdx, [rdx + rdx*2]
142073bef: 48c1e204 shl rdx, 4
142073bf3: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142073bf8: 488995c8000000 mov qword ptr [rbp + 0xc8], rdx
142073bff: 488b95b8000000 mov rdx, qword ptr [rbp + 0xb8]
142073c06: 488b9218010000 mov rdx, qword ptr [rdx + 0x118]
142073c0d: 4839d1 cmp rcx, rdx
142073c10: 0f87d4000000 ja 0x142073cea
142073c16: e941010000 jmp 0x142073d5c ; unknown
142073c1b: 0f1f440000 nop dword ptr [rax + rax]
142073c20: 4939fc cmp r12, rdi
142073c23: 0f855f010000 jne 0x142073d88
142073c29: 0f1f8000000000 nop dword ptr [rax]
142073c30: 4839de cmp rsi, rbx
142073c33: 0f84f7050000 je 0x142074230
142073c39: 8b0e mov ecx, dword ptr [rsi]
142073c3b: 4883c604 add rsi, 4
142073c3f: 498b8600010000 mov rax, qword ptr [r14 + 0x100]
142073c46: 488d0c89 lea rcx, [rcx + rcx*4]
142073c4a: 48c1e105 shl rcx, 5
142073c4e: 488b7c0858 mov rdi, qword ptr [rax + rcx + 0x58]
142073c53: 4885ff test rdi, rdi
142073c56: 74d8 je 0x142073c30
142073c58: 4801c8 add rax, rcx
142073c5b: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
142073c61: 498b96a8010000 mov rdx, qword ptr [r14 + 0x1a8]
142073c68: 4c8d0cc9 lea r9, [rcx + rcx*8]
142073c6c: 4a8d0cca lea rcx, [rdx + r9*8]
142073c70: 4c8b85b8000000 mov r8, qword ptr [rbp + 0xb8]
142073c77: 4d8b8010010000 mov r8, qword ptr [r8 + 0x110]
142073c7e: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
142073c83: 4c39c2 cmp rdx, r8
142073c86: 0f8688000000 jbe 0x142073d14
142073c8c: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
142073c90: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
142073c94: 4d85c0 test r8, r8
142073c97: 747b je 0x142073d14
142073c99: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
142073c9d: 49f7d0 not r8
142073ca0: 4f8d0440 lea r8, [r8 + r8*2]
142073ca4: 49c1e004 shl r8, 4
142073ca8: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
142073cad: 4c8985c8000000 mov qword ptr [rbp + 0xc8], r8
142073cb4: 4c8b85b8000000 mov r8, qword ptr [rbp + 0xb8]
142073cbb: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
142073cc2: 4c39c2 cmp rdx, r8
142073cc5: 776b ja 0x142073d32
142073cc7: e9a3000000 jmp 0x142073d6f ; unknown
142073ccc: 48c785c800000000000000 mov qword ptr [rbp + 0xc8], 0
142073cd7: 488b95b8000000 mov rdx, qword ptr [rbp + 0xb8]
142073cde: 488b9218010000 mov rdx, qword ptr [rdx + 0x118]
142073ce5: 4839d1 cmp rcx, rdx
142073ce8: 7672 jbe 0x142073d5c
142073cea: 488b4838 mov rcx, qword ptr [rax + 0x38]
142073cee: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
142073cf2: 4885c9 test rcx, rcx
142073cf5: 7465 je 0x142073d5c
142073cf7: 488b4018 mov rax, qword ptr [rax + 0x18]
142073cfb: 48f7d1 not rcx
142073cfe: 488d0c49 lea rcx, [rcx + rcx*2]
142073d02: 48c1e104 shl rcx, 4
142073d06: 488b440810 mov rax, qword ptr [rax + rcx + 0x10]
142073d0b: 488985d0000000 mov qword ptr [rbp + 0xd0], rax
142073d12: eb53 jmp 0x142073d67 ; unknown
142073d14: 48c785c800000000000000 mov qword ptr [rbp + 0xc8], 0
142073d1f: 4c8b85b8000000 mov r8, qword ptr [rbp + 0xb8]
142073d26: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
142073d2d: 4c39c2 cmp rdx, r8
142073d30: 763d jbe 0x142073d6f
142073d32: 488b5138 mov rdx, qword ptr [rcx + 0x38]
142073d36: 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
142073d3a: 4885d2 test rdx, rdx
142073d3d: 7430 je 0x142073d6f
142073d3f: 488b4918 mov rcx, qword ptr [rcx + 0x18]
142073d43: 48f7d2 not rdx
142073d46: 488d1452 lea rdx, [rdx + rdx*2]
142073d4a: 48c1e204 shl rdx, 4
142073d4e: 488b4c1110 mov rcx, qword ptr [rcx + rdx + 0x10]
142073d53: 48898dd0000000 mov qword ptr [rbp + 0xd0], rcx
142073d5a: eb1e jmp 0x142073d7a ; unknown
142073d5c: 48c785d000000000000000 mov qword ptr [rbp + 0xd0], 0
142073d67: 4531e4 xor r12d, r12d
142073d6a: 4489e0 mov eax, r12d
142073d6d: eb2b jmp 0x142073d9a ; unknown
142073d6f: 48c785d000000000000000 mov qword ptr [rbp + 0xd0], 0
142073d7a: 488b4050 mov rax, qword ptr [rax + 0x50]
142073d7e: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
142073d85: 4531e4 xor r12d, r12d
142073d88: 4c89e0 mov rax, r12
142073d8b: 48c1e004 shl rax, 4
142073d8f: 488b8da0000000 mov rcx, qword ptr [rbp + 0xa0]
142073d96: 8b440108 mov eax, dword ptr [rcx + rax + 8]
142073d9a: 4c8d2c40 lea r13, [rax + rax*2]
142073d9e: 488b85c8000000 mov rax, qword ptr [rbp + 0xc8]
142073da5: 4a8d04a8 lea rax, [rax + r13*4]
142073da9: 488985c0000000 mov qword ptr [rbp + 0xc0], rax
142073db0: 49c1e504 shl r13, 4
142073db4: 4c03add0000000 add r13, qword ptr [rbp + 0xd0]
142073dbb: f3410f107d10 movss xmm7, dword ptr [r13 + 0x10]
142073dc1: f3450f104518 movss xmm8, dword ptr [r13 + 0x18]
142073dc7: 0f28c7 movaps xmm0, xmm7
142073dca: f30f5ec6 divss xmm0, xmm6
142073dce: 410f28c8 movaps xmm1, xmm8
142073dd2: f30f5ece divss xmm1, xmm6
142073dd6: f30f1015921a8b00 movss xmm2, dword ptr [rip + 0x8b1a92]
142073dde: f30f58c2 addss xmm0, xmm2
142073de2: f30f58ca addss xmm1, xmm2
142073de6: f3410f5fc5 maxss xmm0, xmm13
142073deb: f3410f5fcd maxss xmm1, xmm13
142073df0: f3410f5dc6 minss xmm0, xmm14
142073df5: f3410f5dce minss xmm1, xmm14
142073dfa: f30f1015aefe9200 movss xmm2, dword ptr [rip + 0x92feae]
142073e02: f30f59c2 mulss xmm0, xmm2
142073e06: f3480f2cc0 cvttss2si rax, xmm0
142073e0b: 410f2ec5 ucomiss xmm0, xmm13
142073e0f: ba00000000 mov edx, 0
142073e14: 0f42c2 cmovb eax, edx
142073e17: 410f2ec4 ucomiss xmm0, xmm12
142073e1b: f30f59ca mulss xmm1, xmm2
142073e1f: 41b8ffffffff mov r8d, 0xffffffff
142073e25: 410f47c0 cmova eax, r8d
142073e29: f3480f2cc9 cvttss2si rcx, xmm1
142073e2e: 410f2ecd ucomiss xmm1, xmm13
142073e32: 0f42ca cmovb ecx, edx
142073e35: 410f2ecc ucomiss xmm1, xmm12
142073e39: 410f47c8 cmova ecx, r8d
142073e3d: 3dff010000 cmp eax, 0x1ff
142073e42: baff010000 mov edx, 0x1ff
142073e47: 0f43c2 cmovae eax, edx
142073e4a: 81f9ff010000 cmp ecx, 0x1ff
142073e50: 0f43ca cmovae ecx, edx
142073e53: c1e109 shl ecx, 9
142073e56: 09c1 or ecx, eax
142073e58: c1e905 shr ecx, 5
142073e5b: 48398d98000000 cmp qword ptr [rbp + 0x98], rcx
142073e62: 0f862d050000 jbe 0x142074395
142073e68: 49ffc4 inc r12
142073e6b: 4d8d7d10 lea r15, [r13 + 0x10]
142073e6f: 488b5528 mov rdx, qword ptr [rbp + 0x28]
142073e73: 8b0c8a mov ecx, dword ptr [rdx + rcx*4]
142073e76: 0fa3c1 bt ecx, eax
142073e79: 0f82d1000000 jb 0x142073f50
142073e7f: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
142073e87: 488d4db8 lea rcx, [rbp - 0x48]
142073e8b: 0f28cf movaps xmm1, xmm7
142073e8e: 410f28d0 movaps xmm2, xmm8
142073e92: 0f28de movaps xmm3, xmm6
142073e95: e8560f9afe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142073e9a: 0f2e05d3c78e00 ucomiss xmm0, dword ptr [rip + 0x8ec7d3]
142073ea1: 0f87a9000000 ja 0x142073f50
142073ea7: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
142073eaf: 488d4dc8 lea rcx, [rbp - 0x38]
142073eb3: 0f28cf movaps xmm1, xmm7
142073eb6: 410f28d0 movaps xmm2, xmm8
142073eba: 0f28de movaps xmm3, xmm6
142073ebd: e82e0f9afe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142073ec2: 440f28c8 movaps xmm9, xmm0
142073ec6: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
142073ece: 488d4dd8 lea rcx, [rbp - 0x28]
142073ed2: 0f28cf movaps xmm1, xmm7
142073ed5: 410f28d0 movaps xmm2, xmm8
142073ed9: 0f28de movaps xmm3, xmm6
142073edc: e80f0f9afe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142073ee1: 440f28d0 movaps xmm10, xmm0
142073ee5: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
142073eed: 488d4de8 lea rcx, [rbp - 0x18]
142073ef1: 0f28cf movaps xmm1, xmm7
142073ef4: 410f28d0 movaps xmm2, xmm8
142073ef8: 0f28de movaps xmm3, xmm6
142073efb: e8f00e9afe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142073f00: 440f28d8 movaps xmm11, xmm0
142073f04: c744242000000243 mov dword ptr [rsp + 0x20], 0x43020000
142073f0c: 488d4df8 lea rcx, [rbp - 8]
142073f10: 0f28cf movaps xmm1, xmm7
142073f13: 410f28d0 movaps xmm2, xmm8
142073f17: 0f28de movaps xmm3, xmm6
142073f1a: e8d10e9afe call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142073f1f: f3450f58ca addss xmm9, xmm10
142073f24: f3450f58cb addss xmm9, xmm11
142073f29: f3440f58c8 addss xmm9, xmm0
142073f2e: 0f57c0 xorps xmm0, xmm0
142073f31: f3410f5fc1 maxss xmm0, xmm9
142073f36: 410f28ce movaps xmm1, xmm14
142073f3a: f30f5dc8 minss xmm1, xmm0
142073f3e: 0f2e0d3f1a8b00 ucomiss xmm1, dword ptr [rip + 0x8b1a3f]
142073f45: 0f866d010000 jbe 0x1420740b8
142073f4b: 0f1f440000 nop dword ptr [rax + rax]
142073f50: 418b4708 mov eax, dword ptr [r15 + 8]
142073f54: 894520 mov dword ptr [rbp + 0x20], eax
142073f57: 498b07 mov rax, qword ptr [r15]
142073f5a: 48894518 mov qword ptr [rbp + 0x18], rax
142073f5e: 488d4d50 lea rcx, [rbp + 0x50]
142073f62: e81992c2fe call 0x140c9d180 ; utils::components::transform::impl$1::default
142073f67: f20f100571baa100 movsd xmm0, qword ptr [rip + 0xa1ba71]
142073f6f: f20f114530 movsd qword ptr [rbp + 0x30], xmm0
142073f74: c7453800000080 mov dword ptr [rbp + 0x38], 0x80000000
142073f7b: 440f284550 movaps xmm8, xmmword ptr [rbp + 0x50]
142073f80: 488d4d6c lea rcx, [rbp + 0x6c]
142073f84: 8b4108 mov eax, dword ptr [rcx + 8]
142073f87: f30f106558 movss xmm4, dword ptr [rbp + 0x58]
142073f8c: 488d553c lea rdx, [rbp + 0x3c]
142073f90: 894208 mov dword ptr [rdx + 8], eax
142073f93: 488b01 mov rax, qword ptr [rcx]
142073f96: 488902 mov qword ptr [rdx], rax
142073f99: 0f10553c movups xmm2, xmmword ptr [rbp + 0x3c]
142073f9d: 0f284540 movaps xmm0, xmmword ptr [rbp + 0x40]
142073fa1: 0f284d30 movaps xmm1, xmmword ptr [rbp + 0x30]
142073fa5: 440f28cc movaps xmm9, xmm4
142073fa9: f3440f58cc addss xmm9, xmm4
142073fae: f3410f59e1 mulss xmm4, xmm9
142073fb3: 410f28e8 movaps xmm5, xmm8
142073fb7: 410f58e8 addps xmm5, xmm8
142073fbb: f30f16dd movshdup xmm3, xmm5
142073fbf: f3410f59d8 mulss xmm3, xmm8
142073fc4: 410f28f8 movaps xmm7, xmm8
142073fc8: f3450f12d1 movsldup xmm10, xmm9
142073fcd: 450f59d0 mulps xmm10, xmm8
142073fd1: 450fc6c0ff shufps xmm8, xmm8, 0xff
142073fd6: 0f59fd mulps xmm7, xmm5
142073fd9: 410f59e8 mulps xmm5, xmm8
142073fdd: 450f28d8 movaps xmm11, xmm8
142073fe1: f3450f59d9 mulss xmm11, xmm9
142073fe6: 0fc6ede1 shufps xmm5, xmm5, 0xe1
142073fea: f3440f16ff movshdup xmm15, xmm7
142073fef: 450f28c7 movaps xmm8, xmm15
142073ff3: f3440f58c4 addss xmm8, xmm4
142073ff8: 450f28ce movaps xmm9, xmm14
142073ffc: f3450f5cc8 subss xmm9, xmm8
142074001: 440f28c3 movaps xmm8, xmm3
142074005: f3450f58c3 addss xmm8, xmm11
14207400a: 450f14c8 unpcklps xmm9, xmm8
14207400e: 450f28c2 movaps xmm8, xmm10
142074012: 440f5cc5 subps xmm8, xmm5
142074016: 450f16c8 movlhps xmm9, xmm8
14207401a: f3410f5cdb subss xmm3, xmm11
14207401f: f30f58e7 addss xmm4, xmm7
142074023: 450f28de movaps xmm11, xmm14
142074027: f3440f5cdc subss xmm11, xmm4
14207402c: 410f14db unpcklps xmm3, xmm11
142074030: 410f58ea addps xmm5, xmm10
142074034: 0fc6ddd4 shufps xmm3, xmm5, 0xd4
142074038: 440fc6c501 shufps xmm8, xmm5, 1
14207403d: f3410f58ff addss xmm7, xmm15
142074042: 410f28e6 movaps xmm4, xmm14
142074046: f30f5ce7 subss xmm4, xmm7
14207404a: 0fc6e530 shufps xmm4, xmm5, 0x30
14207404e: 440fc6c482 shufps xmm8, xmm4, 0x82
142074053: 0fc6d200 shufps xmm2, xmm2, 0
142074057: 410f59d1 mulps xmm2, xmm9
14207405b: 0f28e0 movaps xmm4, xmm0
14207405e: 0fc6e000 shufps xmm4, xmm0, 0
142074062: 0f59e3 mulps xmm4, xmm3
142074065: 0fc6c055 shufps xmm0, xmm0, 0x55
142074069: 410f59c0 mulps xmm0, xmm8
14207406d: 0f28da movaps xmm3, xmm2
142074070: f20f10dc movsd xmm3, xmm4
142074074: 0fc6d324 shufps xmm2, xmm3, 0x24
142074078: 0f295550 movaps xmmword ptr [rbp + 0x50], xmm2
14207407c: 0fc6e049 shufps xmm4, xmm0, 0x49
142074080: 0f296560 movaps xmmword ptr [rbp + 0x60], xmm4
142074084: 0fc6c102 shufps xmm0, xmm1, 2
142074088: 0fc6c198 shufps xmm0, xmm1, 0x98
14207408c: 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
142074090: 488d4550 lea rax, [rbp + 0x50]
142074094: 4889442420 mov qword ptr [rsp + 0x20], rax
142074099: 0f57db xorps xmm3, xmm3
14207409c: 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
1420740a3: 488b95c0000000 mov rdx, qword ptr [rbp + 0xc0]
1420740aa: 4c8d4518 lea r8, [rbp + 0x18]
1420740ae: e82ddd4fff call 0x141571de0 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>
1420740b3: e9bafaffff jmp 0x142073b72 ; unknown
1420740b8: 418b4708 mov eax, dword ptr [r15 + 8]
1420740bc: 894538 mov dword ptr [rbp + 0x38], eax
1420740bf: 498b07 mov rax, qword ptr [r15]
1420740c2: 48894530 mov qword ptr [rbp + 0x30], rax
1420740c6: 410f286500 movaps xmm4, xmmword ptr [r13]
1420740cb: f3410f105d08 movss xmm3, dword ptr [r13 + 8]
1420740d1: 0f28fb movaps xmm7, xmm3
1420740d4: f30f58fb addss xmm7, xmm3
1420740d8: f30f59df mulss xmm3, xmm7
1420740dc: 0f28cc movaps xmm1, xmm4
1420740df: 0f58cc addps xmm1, xmm4
1420740e2: f30f16c1 movshdup xmm0, xmm1
1420740e6: f30f59c4 mulss xmm0, xmm4
1420740ea: 0f28d4 movaps xmm2, xmm4
1420740ed: f30f12ef movsldup xmm5, xmm7
1420740f1: 0f59ec mulps xmm5, xmm4
1420740f4: 0fc6e4ff shufps xmm4, xmm4, 0xff
1420740f8: 0f59d1 mulps xmm2, xmm1
1420740fb: 0f59cc mulps xmm1, xmm4
1420740fe: 440f28c4 movaps xmm8, xmm4
142074102: f3440f59c7 mulss xmm8, xmm7
142074107: f3440f16ca movshdup xmm9, xmm2
14207410c: 410f28f9 movaps xmm7, xmm9
142074110: f30f58fb addss xmm7, xmm3
142074114: 410f28e6 movaps xmm4, xmm14
142074118: f30f5ce7 subss xmm4, xmm7
14207411c: 0f28f8 movaps xmm7, xmm0
14207411f: f3410f58f8 addss xmm7, xmm8
142074124: 0f14e7 unpcklps xmm4, xmm7
142074127: f3410f5cc0 subss xmm0, xmm8
14207412c: f30f58da addss xmm3, xmm2
142074130: 410f28fe movaps xmm7, xmm14
142074134: f30f5cfb subss xmm7, xmm3
142074138: 0fc6c9e1 shufps xmm1, xmm1, 0xe1
14207413c: 0f14c7 unpcklps xmm0, xmm7
14207413f: 0f28fd movaps xmm7, xmm5
142074142: 0f5cf9 subps xmm7, xmm1
142074145: 0f58cd addps xmm1, xmm5
142074148: f3410f105d20 movss xmm3, dword ptr [r13 + 0x20]
14207414e: f3410f58d1 addss xmm2, xmm9
142074153: 450f28c6 movaps xmm8, xmm14
142074157: f3440f5cc2 subss xmm8, xmm2
14207415c: f3410f106d24 movss xmm5, dword ptr [r13 + 0x24]
142074162: 0f16e7 movlhps xmm4, xmm7
142074165: 0fc6c1d4 shufps xmm0, xmm1, 0xd4
142074169: 0fc6f901 shufps xmm7, xmm1, 1
14207416d: 440fc6c130 shufps xmm8, xmm1, 0x30
142074172: 410f10551c movups xmm2, xmmword ptr [r13 + 0x1c]
142074177: 410fc6f882 shufps xmm7, xmm8, 0x82
14207417c: 410f104d20 movups xmm1, xmmword ptr [r13 + 0x20]
142074181: 0fc6d200 shufps xmm2, xmm2, 0
142074185: 0f59d4 mulps xmm2, xmm4
142074188: 0f28e1 movaps xmm4, xmm1
14207418b: 0fc6e100 shufps xmm4, xmm1, 0
14207418f: 0f59e0 mulps xmm4, xmm0
142074192: 0fc6c955 shufps xmm1, xmm1, 0x55
142074196: 0f59cf mulps xmm1, xmm7
142074199: 440f2805dfa78b00 movaps xmm8, xmmword ptr [rip + 0x8ba7df]
1420741a1: 410f54d8 andps xmm3, xmm8
1420741a5: 410f54e8 andps xmm5, xmm8
1420741a9: 0f28c3 movaps xmm0, xmm3
1420741ac: f30fc2c303 cmpunordss xmm0, xmm3
1420741b1: 0f28f8 movaps xmm7, xmm0
1420741b4: 0f54fd andps xmm7, xmm5
1420741b7: f30f5feb maxss xmm5, xmm3
1420741bb: f3410f105d1c movss xmm3, dword ptr [r13 + 0x1c]
1420741c1: 410f54d8 andps xmm3, xmm8
1420741c5: 0f55c5 andnps xmm0, xmm5
1420741c8: 0f56c7 orps xmm0, xmm7
1420741cb: 0f28e8 movaps xmm5, xmm0
1420741ce: f30f5feb maxss xmm5, xmm3
1420741d2: f30fc2db03 cmpunordss xmm3, xmm3
1420741d7: 0f28fb movaps xmm7, xmm3
1420741da: 0f54f8 andps xmm7, xmm0
1420741dd: 0f55dd andnps xmm3, xmm5
1420741e0: 0f56df orps xmm3, xmm7
1420741e3: 0f28c2 movaps xmm0, xmm2
1420741e6: f20f10c4 movsd xmm0, xmm4
1420741ea: 0fc6d024 shufps xmm2, xmm0, 0x24
1420741ee: 410f284510 movaps xmm0, xmmword ptr [r13 + 0x10]
1420741f3: 0f295550 movaps xmmword ptr [rbp + 0x50], xmm2
1420741f7: 0fc6e149 shufps xmm4, xmm1, 0x49
1420741fb: 0f296560 movaps xmmword ptr [rbp + 0x60], xmm4
1420741ff: 0fc6c802 shufps xmm1, xmm0, 2
142074203: 0fc6c898 shufps xmm1, xmm0, 0x98
142074207: 0f294d70 movaps xmmword ptr [rbp + 0x70], xmm1
14207420b: 488d4550 lea rax, [rbp + 0x50]
14207420f: 4889442420 mov qword ptr [rsp + 0x20], rax
142074214: 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
14207421b: 488b95c0000000 mov rdx, qword ptr [rbp + 0xc0]
142074222: 4c8d4530 lea r8, [rbp + 0x30]
142074226: e8b5db4fff call 0x141571de0 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>
14207422b: e942f9ffff jmp 0x142073b72 ; unknown
142074230: 488bb5a8000000 mov rsi, qword ptr [rbp + 0xa8]
142074237: 4889f1 mov rcx, rsi
14207423a: e8d1dbc0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207423f: 4889f1 mov rcx, rsi
142074242: 4883c901 or rcx, 1
142074246: e8c5dbc0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207424b: 31c9 xor ecx, ecx
14207424d: b808000000 mov eax, 8
142074252: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
142074257: 0f8529010000 jne 0x142074386
14207425d: 4883bd8000000000 cmp qword ptr [rbp + 0x80], 0
142074265: 744b je 0x1420742b2
142074267: e8c43ebafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207426c: 4889c1 mov rcx, rax
14207426f: 488b00 mov rax, qword ptr [rax]
142074272: 4883f801 cmp rax, 1
142074276: 0f8598000000 jne 0x142074314
14207427c: 4883c108 add rcx, 8
142074280: 48833900 cmp qword ptr [rcx], 0
142074284: 0f85c8000000 jne 0x142074352
14207428a: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142074291: 48898dc0000000 mov qword ptr [rbp + 0xc0], rcx
142074298: 4883c108 add rcx, 8
14207429c: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
1420742a3: e8482ebafe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1420742a8: 488b85c0000000 mov rax, qword ptr [rbp + 0xc0]
1420742af: 48ff00 inc qword ptr [rax]
1420742b2: 0f28b5e0000000 movaps xmm6, xmmword ptr [rbp + 0xe0]
1420742b9: 0f28bdf0000000 movaps xmm7, xmmword ptr [rbp + 0xf0]
1420742c0: 440f288500010000 movaps xmm8, xmmword ptr [rbp + 0x100]
1420742c8: 440f288d10010000 movaps xmm9, xmmword ptr [rbp + 0x110]
1420742d0: 440f289520010000 movaps xmm10, xmmword ptr [rbp + 0x120]
1420742d8: 440f289d30010000 movaps xmm11, xmmword ptr [rbp + 0x130]
1420742e0: 440f28a540010000 movaps xmm12, xmmword ptr [rbp + 0x140]
1420742e8: 440f28ad50010000 movaps xmm13, xmmword ptr [rbp + 0x150]
1420742f0: 440f28b560010000 movaps xmm14, xmmword ptr [rbp + 0x160]
1420742f8: 440f28bd70010000 movaps xmm15, xmmword ptr [rbp + 0x170]
142074300: 4881c408020000 add rsp, 0x208
142074307: 5b pop rbx
142074308: 5f pop rdi
142074309: 5e pop rsi
14207430a: 415c pop r12
14207430c: 415d pop r13
14207430e: 415e pop r14
142074310: 415f pop r15
142074312: 5d pop rbp
142074313: c3 ret 
142074314: 4885c0 test rax, rax
142074317: 752d jne 0x142074346
142074319: 31d2 xor edx, edx
14207431b: e8b0448800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142074320: 4889c1 mov rcx, rax
142074323: 4885c0 test rax, rax
142074326: 0f8554ffffff jne 0x142074280
14207432c: eb18 jmp 0x142074346 ; unknown
14207432e: 4885c0 test rax, rax
142074331: 7513 jne 0x142074346
142074333: 31d2 xor edx, edx
142074335: e896448800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14207433a: 4889c1 mov rcx, rax
14207433d: 4885c0 test rax, rax
142074340: 0f85bcf5ffff jne 0x142073902
142074346: 488d0d5367bc00 lea rcx, [rip + 0xbc6753]
14207434d: e87e3d8600 call 0x1428d80d0 ; std::thread::local::panic_access_error
142074352: 488d0d1768bc00 lea rcx, [rip + 0xbc6817]
142074359: e8f24e8600 call 0x1428d9250 ; core::cell::panic_already_borrowed
14207435e: 48c745c800000000 mov qword ptr [rbp - 0x38], 0
142074366: 488d05139ebb00 lea rax, [rip + 0xbb9e13]
14207436d: 4889442420 mov qword ptr [rsp + 0x20], rax
142074372: 488d5550 lea rdx, [rbp + 0x50]
142074376: 4c8d4dc8 lea r9, [rbp - 0x38]
14207437a: 31c9 xor ecx, ecx
14207437c: 4d89f8 mov r8, r15
14207437f: e8bd678500 call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
142074384: eb30 jmp 0x1420743b6 ; unknown
142074386: 4889f1 mov rcx, rsi
142074389: 31d2 xor edx, edx
14207438b: e8d07b8500 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
142074390: e9c8feffff jmp 0x14207425d ; unknown
142074395: 4c8d05f40ca500 lea r8, [rip + 0xa50cf4]
14207439c: 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
1420743a3: e870518600 call 0x1428d9518 ; core::panicking::panic_bounds_check
1420743a8: eb0c jmp 0x1420743b6 ; unknown
1420743aa: 488d0d77f2f500 lea rcx, [rip + 0xf5f277]
1420743b1: e85a4f8600 call 0x1428d9310 ; core::option::unwrap_failed
1420743b6: 0f0b ud2 
1420743b8: 0f1f840000000000 nop dword ptr [rax + rax]
1420743c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420743c5: 55 push rbp
1420743c6: 4157 push r15
1420743c8: 4156 push r14
1420743ca: 4155 push r13
1420743cc: 4154 push r12
1420743ce: 56 push rsi
1420743cf: 57 push rdi
1420743d0: 53 push rbx
1420743d1: 4881ecd8000000 sub rsp, 0xd8
1420743d8: 488daa80000000 lea rbp, [rdx + 0x80]
1420743df: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
1420743e5: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
1420743eb: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
1420743f1: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
1420743f7: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
1420743fd: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
142074406: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
14207440f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
142074418: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
142074420: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
142074428: 488b85b0000000 mov rax, qword ptr [rbp + 0xb0]
14207442f: 48ff00 inc qword ptr [rax]
142074432: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
14207443a: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
142074442: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
14207444b: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
142074454: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
14207445d: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
142074463: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
142074469: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
14207446f: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
142074475: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
14207447b: 4881c4d8000000 add rsp, 0xd8
142074482: 5b pop rbx
142074483: 5f pop rdi
142074484: 5e pop rsi
142074485: 415c pop r12
142074487: 415d pop r13
142074489: 415e pop r14
14207448b: 415f pop r15
14207448d: 5d pop rbp
14207448e: c3 ret 
14207448f: 90 nop 
142074490: 4889542410 mov qword ptr [rsp + 0x10], rdx
142074495: 55 push rbp
142074496: 4157 push r15
142074498: 4156 push r14
14207449a: 4155 push r13
14207449c: 4154 push r12
14207449e: 56 push rsi
14207449f: 57 push rdi
1420744a0: 53 push rbx
1420744a1: 4881ecd8000000 sub rsp, 0xd8
1420744a8: 488daa80000000 lea rbp, [rdx + 0x80]
1420744af: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
1420744b5: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
1420744bb: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
1420744c1: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
1420744c7: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
1420744cd: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
1420744d6: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
1420744df: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
1420744e8: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
1420744f0: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1420744f8: 488b85c0000000 mov rax, qword ptr [rbp + 0xc0]
1420744ff: 48ff00 inc qword ptr [rax]
142074502: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
14207450a: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
142074512: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
14207451b: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
142074524: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
14207452d: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
142074533: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
142074539: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
14207453f: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
142074545: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
14207454b: 4881c4d8000000 add rsp, 0xd8
142074552: 5b pop rbx
142074553: 5f pop rdi
142074554: 5e pop rsi
142074555: 415c pop r12
142074557: 415d pop r13
142074559: 415e pop r14
14207455b: 415f pop r15
14207455d: 5d pop rbp
14207455e: c3 ret 
14207455f: 90 nop 
142074560: 4889542410 mov qword ptr [rsp + 0x10], rdx
142074565: 55 push rbp
142074566: 4157 push r15
142074568: 4156 push r14
14207456a: 4155 push r13
14207456c: 4154 push r12
14207456e: 56 push rsi
14207456f: 57 push rdi
142074570: 53 push rbx
142074571: 4881ecd8000000 sub rsp, 0xd8
142074578: 488daa80000000 lea rbp, [rdx + 0x80]
14207457f: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
142074585: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
14207458b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
142074591: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
142074597: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
14207459d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
1420745a6: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
1420745af: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
1420745b8: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
1420745c0: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1420745c8: 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
1420745cf: e8fcccfcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
1420745d4: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
1420745dc: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
1420745e4: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
1420745ed: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
1420745f6: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
1420745ff: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
142074605: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
14207460b: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
142074611: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
142074617: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
14207461d: 4881c4d8000000 add rsp, 0xd8
142074624: 5b pop rbx
142074625: 5f pop rdi
142074626: 5e pop rsi
142074627: 415c pop r12
142074629: 415d pop r13
14207462b: 415e pop r14
14207462d: 415f pop r15
14207462f: 5d pop rbp
142074630: c3 ret 
142074631: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142074640: 4889542410 mov qword ptr [rsp + 0x10], rdx
142074645: 55 push rbp
142074646: 4157 push r15
142074648: 4156 push r14
14207464a: 4155 push r13
14207464c: 4154 push r12
14207464e: 56 push rsi
14207464f: 57 push rdi
142074650: 53 push rbx
142074651: 4881ecd8000000 sub rsp, 0xd8
142074658: 488daa80000000 lea rbp, [rdx + 0x80]
14207465f: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
142074665: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
14207466b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
142074671: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
142074677: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
14207467d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
142074686: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
14207468f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
142074698: 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
1420746a0: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1420746a8: 4883bd8000000000 cmp qword ptr [rbp + 0x80], 0
1420746b0: 7423 je 0x1420746d5
1420746b2: 488d8588000000 lea rax, [rbp + 0x88]
1420746b9: 48894510 mov qword ptr [rbp + 0x10], rax
1420746bd: 488d4510 lea rax, [rbp + 0x10]
1420746c1: 48894508 mov qword ptr [rbp + 8], rax
1420746c5: 488d0d8cecf500 lea rcx, [rip + 0xf5ec8c]
1420746cc: 488d5508 lea rdx, [rbp + 8]
1420746d0: e8fbab2bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
1420746d5: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
1420746dd: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
1420746e5: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
1420746ee: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
1420746f7: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
142074700: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
142074706: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
14207470c: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
142074712: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
142074718: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
14207471e: 4881c4d8000000 add rsp, 0xd8
142074725: 5b pop rbx
142074726: 5f pop rdi
142074727: 5e pop rsi
142074728: 415c pop r12
14207472a: 415d pop r13
14207472c: 415e pop r14
14207472e: 415f pop r15
142074730: 5d pop rbp
142074731: c3 ret 
