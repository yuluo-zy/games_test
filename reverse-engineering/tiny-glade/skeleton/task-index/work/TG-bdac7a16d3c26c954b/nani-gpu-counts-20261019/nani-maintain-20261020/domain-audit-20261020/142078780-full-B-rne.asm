142078780: 55 push rbp
142078781: 4157 push r15
142078783: 4156 push r14
142078785: 4155 push r13
142078787: 4154 push r12
142078789: 56 push rsi
14207878a: 57 push rdi
14207878b: 53 push rbx
14207878c: 4881ecd8010000 sub rsp, 0x1d8
142078793: 488dac2480000000 lea rbp, [rsp + 0x80]
14207879b: 440f29bd40010000 movaps xmmword ptr [rbp + 0x140], xmm15
1420787a3: 440f29b530010000 movaps xmmword ptr [rbp + 0x130], xmm14
1420787ab: 440f29ad20010000 movaps xmmword ptr [rbp + 0x120], xmm13
1420787b3: 440f29a510010000 movaps xmmword ptr [rbp + 0x110], xmm12
1420787bb: 440f299d00010000 movaps xmmword ptr [rbp + 0x100], xmm11
1420787c3: 440f2995f0000000 movaps xmmword ptr [rbp + 0xf0], xmm10
1420787cb: 440f298de0000000 movaps xmmword ptr [rbp + 0xe0], xmm9
1420787d3: 440f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm8
1420787db: 660f7fbdc0000000 movdqa xmmword ptr [rbp + 0xc0], xmm7
1420787e3: 0f29b5b0000000 movaps xmmword ptr [rbp + 0xb0], xmm6
1420787ea: 48c785a8000000feffffff mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
1420787f5: 4889d6 mov rsi, rdx
1420787f8: 4889cf mov rdi, rcx
1420787fb: e800e2b9fe call 0x140c16a00 ; puffin::are_scopes_on
142078800: 84c0 test al, al
142078802: 745f je 0x142078863
142078804: 4c8d357da6f500 lea r14, [rip + 0xf5a67d]
14207880b: bb3c000000 mov ebx, 0x3c
142078810: ba3c000000 mov edx, 0x3c
142078815: 4c89f1 mov rcx, r14
142078818: e85366ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207881d: 4883f801 cmp rax, 1
142078821: 0f85b1000000 jne 0x1420788d8
142078827: 4989d1 mov r9, rdx
14207882a: 4885d2 test rdx, rdx
14207882d: 743d je 0x14207886c
14207882f: 4983f93c cmp r9, 0x3c
142078833: 7335 jae 0x14207886a
142078835: 488d054ca6f500 lea rax, [rip + 0xf5a64c]
14207883c: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
142078841: 7f29 jg 0x14207886c
142078843: 488d058eabf500 lea rax, [rip + 0xf5ab8e]
14207884a: 4889442420 mov qword ptr [rsp + 0x20], rax
14207884f: 488d0d32a6f500 lea rcx, [rip + 0xf5a632]
142078856: ba3c000000 mov edx, 0x3c
14207885b: 4531c0 xor r8d, r8d
14207885e: e8dd108600 call 0x1428d9940 ; core::str::slice_error_fail
142078863: 31c0 xor eax, eax
142078865: e9c0010000 jmp 0x142078a2a ; unknown
14207886a: 75d7 jne 0x142078843
14207886c: 4c8d3515a6f500 lea r14, [rip + 0xf5a615]
142078873: 4c89f1 mov rcx, r14
142078876: 4c89ca mov rdx, r9
142078879: e8f265ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207887e: 4883f801 cmp rax, 1
142078882: 7554 jne 0x1420788d8
142078884: 4989d0 mov r8, rdx
142078887: 4983c002 add r8, 2
14207888b: 7439 je 0x1420788c6
14207888d: 4983f83c cmp r8, 0x3c
142078891: 7331 jae 0x1420788c4
142078893: 488d05eea5f500 lea rax, [rip + 0xf5a5ee]
14207889a: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
14207889f: 7f25 jg 0x1420788c6
1420788a1: 488d0548abf500 lea rax, [rip + 0xf5ab48]
1420788a8: 4889442420 mov qword ptr [rsp + 0x20], rax
1420788ad: 488d0dd4a5f500 lea rcx, [rip + 0xf5a5d4]
1420788b4: ba3c000000 mov edx, 0x3c
1420788b9: 41b93c000000 mov r9d, 0x3c
1420788bf: e87c108600 call 0x1428d9940 ; core::str::slice_error_fail
1420788c4: 75db jne 0x1420788a1
1420788c6: bb3a000000 mov ebx, 0x3a
1420788cb: 4829d3 sub rbx, rdx
1420788ce: 4c8d35b3a5f500 lea r14, [rip + 0xf5a5b3]
1420788d5: 4d01c6 add r14, r8
1420788d8: 4c8d057fabf500 lea r8, [rip + 0xf5ab7f]
1420788df: 4c8d3d4aabf500 lea r15, [rip + 0xf5ab4a]
1420788e6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1420788f0: 410fbe40ff movsx eax, byte ptr [r8 - 1]
1420788f5: 85c0 test eax, eax
1420788f7: 7817 js 0x142078910
1420788f9: 49ffc8 dec r8
1420788fc: 83f85c cmp eax, 0x5c
1420788ff: 7563 jne 0x142078964
142078901: eb73 jmp 0x142078976 ; unknown
142078903: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142078910: 410fb648fe movzx ecx, byte ptr [r8 - 2]
142078915: 80f9c0 cmp cl, 0xc0
142078918: 7d23 jge 0x14207893d
14207891a: 410fb650fd movzx edx, byte ptr [r8 - 3]
14207891f: 80fac0 cmp dl, 0xc0
142078922: 7d22 jge 0x142078946
142078924: 450fb648fc movzx r9d, byte ptr [r8 - 4]
142078929: 4983c0fc add r8, -4
14207892d: 4183e107 and r9d, 7
142078931: 41c1e106 shl r9d, 6
142078935: 83e23f and edx, 0x3f
142078938: 4409ca or edx, r9d
14207893b: eb10 jmp 0x14207894d ; unknown
14207893d: 4983c0fe add r8, -2
142078941: 83e11f and ecx, 0x1f
142078944: eb0f jmp 0x142078955 ; unknown
142078946: 4983c0fd add r8, -3
14207894a: 83e20f and edx, 0xf
14207894d: c1e206 shl edx, 6
142078950: 83e13f and ecx, 0x3f
142078953: 09d1 or ecx, edx
142078955: c1e106 shl ecx, 6
142078958: 243f and al, 0x3f
14207895a: 0fb6c0 movzx eax, al
14207895d: 09c8 or eax, ecx
14207895f: 83f85c cmp eax, 0x5c
142078962: 7412 je 0x142078976
142078964: 83f82f cmp eax, 0x2f
142078967: 740d je 0x142078976
142078969: 4d39f8 cmp r8, r15
14207896c: 7582 jne 0x1420788f0
14207896e: 41bc2e000000 mov r12d, 0x2e
142078974: eb4d jmp 0x1420789c3 ; unknown
142078976: 4c8d3db3aaf500 lea r15, [rip + 0xf5aab3]
14207897d: 4d29f8 sub r8, r15
142078980: 49ffc0 inc r8
142078983: 7432 je 0x1420789b7
142078985: 4983f82e cmp r8, 0x2e
142078989: 732a jae 0x1420789b5
14207898b: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
142078990: 7f25 jg 0x1420789b7
142078992: 488d051faaf500 lea rax, [rip + 0xf5aa1f]
142078999: 4889442420 mov qword ptr [rsp + 0x20], rax
14207899e: 488d0d8baaf500 lea rcx, [rip + 0xf5aa8b]
1420789a5: ba2e000000 mov edx, 0x2e
1420789aa: 41b92e000000 mov r9d, 0x2e
1420789b0: e88b0f8600 call 0x1428d9940 ; core::str::slice_error_fail
1420789b5: 75db jne 0x142078992
1420789b7: 41bc2e000000 mov r12d, 0x2e
1420789bd: 4d29c4 sub r12, r8
1420789c0: 4d01c7 add r15, r8
1420789c3: e868f7b9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1420789c8: 4889c1 mov rcx, rax
1420789cb: 488b00 mov rax, qword ptr [rax]
1420789ce: 4883f801 cmp rax, 1
1420789d2: 0f851e0a0000 jne 0x1420793f6
1420789d8: 4883c108 add rcx, 8
1420789dc: 48833900 cmp qword ptr [rcx], 0
1420789e0: 0f85340a0000 jne 0x14207941a
1420789e6: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1420789ed: 48894d78 mov qword ptr [rbp + 0x78], rcx
1420789f1: 4883c108 add rcx, 8
1420789f5: 4c89642420 mov qword ptr [rsp + 0x20], r12
1420789fa: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
142078a03: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
142078a0c: 4c89f2 mov rdx, r14
142078a0f: 4989d8 mov r8, rbx
142078a12: 4d89f9 mov r9, r15
142078a15: e826e3b9fe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
142078a1a: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
142078a1e: 48ff01 inc qword ptr [rcx]
142078a21: 48894540 mov qword ptr [rbp + 0x40], rax
142078a25: b801000000 mov eax, 1
142078a2a: 48894538 mov qword ptr [rbp + 0x38], rax
142078a2e: 488b0e mov rcx, qword ptr [rsi]
142078a31: b209 mov dl, 9
142078a33: e8189291fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
142078a38: 48895510 mov qword ptr [rbp + 0x10], rdx
142078a3c: 48894558 mov qword ptr [rbp + 0x58], rax
142078a40: 488b0f mov rcx, qword ptr [rdi]
142078a43: 488b5708 mov rdx, qword ptr [rdi + 8]
142078a47: 4c8ba1f8000000 mov r12, qword ptr [rcx + 0xf8]
142078a4e: 488b8100010000 mov rax, qword ptr [rcx + 0x100]
142078a55: 4d8d0484 lea r8, [r12 + rax*4]
142078a59: 48898d90000000 mov qword ptr [rbp + 0x90], rcx
142078a60: 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
142078a67: 88859b000000 mov byte ptr [rbp + 0x9b], al
142078a6d: b808000000 mov eax, 8
142078a72: 48894548 mov qword ptr [rbp + 0x48], rax
142078a76: f30f103586ce8a00 movss xmm6, dword ptr [rip + 0x8ace86]
142078a7e: 660f6f3d2a5c8b00 movdqa xmm7, xmmword ptr [rip + 0x8b5c2a]
142078a86: 440f2805f25e8b00 movaps xmm8, xmmword ptr [rip + 0x8b5ef2]
142078a8e: f3440f100dedce8a00 movss xmm9, dword ptr [rip + 0x8aceed]
142078a97: f3440f101dd0cd8a00 movss xmm11, dword ptr [rip + 0x8acdd0]
142078aa0: f3440f10252bce8a00 movss xmm12, dword ptr [rip + 0x8ace2b]
142078aa9: 31f6 xor esi, esi
142078aab: 31c0 xor eax, eax
142078aad: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142078ab5: c7859c00000000000000 mov dword ptr [rbp + 0x9c], 0
142078abf: 48c7457000000000 mov qword ptr [rbp + 0x70], 0
142078ac7: c785a000000000000000 mov dword ptr [rbp + 0xa0], 0
142078ad1: 48c7858000000000000000 mov qword ptr [rbp + 0x80], 0
142078adc: 48c7858800000000000000 mov qword ptr [rbp + 0x88], 0
142078ae7: 48c7456800000000 mov qword ptr [rbp + 0x68], 0
142078aef: c785a400000000000000 mov dword ptr [rbp + 0xa4], 0
142078af9: 48895518 mov qword ptr [rbp + 0x18], rdx
142078afd: 4c894520 mov qword ptr [rbp + 0x20], r8
142078b01: 80bd9b00000000 cmp byte ptr [rbp + 0x9b], 0
142078b08: 0f8572030000 jne 0x142078e80
142078b0e: 4839f0 cmp rax, rsi
142078b11: 0f8594020000 jne 0x142078dab
142078b17: 660f1f840000000000 nop word ptr [rax + rax]
142078b20: 4d39c4 cmp r12, r8
142078b23: 0f84de070000 je 0x142079307
142078b29: 418b0c24 mov ecx, dword ptr [r12]
142078b2d: 4983c404 add r12, 4
142078b31: 488b8200010000 mov rax, qword ptr [rdx + 0x100]
142078b38: 488d0c89 lea rcx, [rcx + rcx*4]
142078b3c: 48c1e105 shl rcx, 5
142078b40: 488b740858 mov rsi, qword ptr [rax + rcx + 0x58]
142078b45: 4885f6 test rsi, rsi
142078b48: 74d6 je 0x142078b20
142078b4a: 4801c8 add rax, rcx
142078b4d: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
142078b53: 488b92a8010000 mov rdx, qword ptr [rdx + 0x1a8]
142078b5a: 4c8d0cc9 lea r9, [rcx + rcx*8]
142078b5e: 4a8d0cca lea rcx, [rdx + r9*8]
142078b62: 4c8b8590000000 mov r8, qword ptr [rbp + 0x90]
142078b69: 4d8b8010010000 mov r8, qword ptr [r8 + 0x110]
142078b70: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
142078b75: 4c39c2 cmp rdx, r8
142078b78: 763d jbe 0x142078bb7
142078b7a: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
142078b7e: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
142078b82: 4d85c0 test r8, r8
142078b85: 7430 je 0x142078bb7
142078b87: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
142078b8b: 49f7d0 not r8
142078b8e: 4f8d0440 lea r8, [r8 + r8*2]
142078b92: 49c1e004 shl r8, 4
142078b96: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
142078b9b: 4c898580000000 mov qword ptr [rbp + 0x80], r8
142078ba2: 4c8b8590000000 mov r8, qword ptr [rbp + 0x90]
142078ba9: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
142078bb0: 4c39c2 cmp rdx, r8
142078bb3: 7720 ja 0x142078bd5
142078bb5: eb48 jmp 0x142078bff ; unknown
142078bb7: 48c7858000000000000000 mov qword ptr [rbp + 0x80], 0
142078bc2: 4c8b8590000000 mov r8, qword ptr [rbp + 0x90]
142078bc9: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
142078bd0: 4c39c2 cmp rdx, r8
142078bd3: 762a jbe 0x142078bff
142078bd5: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
142078bd9: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
142078bdd: 4d85c0 test r8, r8
142078be0: 741d je 0x142078bff
142078be2: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
142078be6: 49f7d0 not r8
142078be9: 4f8d0440 lea r8, [r8 + r8*2]
142078bed: 49c1e004 shl r8, 4
142078bf1: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
142078bf6: 4c898588000000 mov qword ptr [rbp + 0x88], r8
142078bfd: eb0b jmp 0x142078c0a ; unknown
142078bff: 48c7858800000000000000 mov qword ptr [rbp + 0x88], 0
142078c0a: 4c8b8590000000 mov r8, qword ptr [rbp + 0x90]
142078c11: 4d8b8820010000 mov r9, qword ptr [r8 + 0x120]
142078c18: 4c8b8088000000 mov r8, qword ptr [rax + 0x88]
142078c1f: 4d39c8 cmp r8, r9
142078c22: 765a jbe 0x142078c7e
142078c24: 4c8b9080000000 mov r10, qword ptr [rax + 0x80]
142078c2b: 4b833cca00 cmp qword ptr [r10 + r9*8], 0
142078c30: 744c je 0x142078c7e
142078c32: 41b201 mov r10b, 1
142078c35: 4489959c000000 mov dword ptr [rbp + 0x9c], r10d
142078c3c: 4c39ca cmp rdx, r9
142078c3f: 765c jbe 0x142078c9d
142078c41: 4c8b5138 mov r10, qword ptr [rcx + 0x38]
142078c45: 4f8b0cca mov r9, qword ptr [r10 + r9*8]
142078c49: 4d85c9 test r9, r9
142078c4c: 744f je 0x142078c9d
142078c4e: 4c8b5118 mov r10, qword ptr [rcx + 0x18]
142078c52: 49f7d1 not r9
142078c55: 4f8d0c49 lea r9, [r9 + r9*2]
142078c59: 49c1e104 shl r9, 4
142078c5d: 4f8b4c0a10 mov r9, qword ptr [r10 + r9 + 0x10]
142078c62: 4c894d60 mov qword ptr [rbp + 0x60], r9
142078c66: 4c8b8d90000000 mov r9, qword ptr [rbp + 0x90]
142078c6d: 4d8b8928010000 mov r9, qword ptr [r9 + 0x128]
142078c74: 4d39c8 cmp r8, r9
142078c77: 773f ja 0x142078cb8
142078c79: e991000000 jmp 0x142078d0f ; unknown
142078c7e: c7859c00000000000000 mov dword ptr [rbp + 0x9c], 0
142078c88: 4c8b8d90000000 mov r9, qword ptr [rbp + 0x90]
142078c8f: 4d8b8928010000 mov r9, qword ptr [r9 + 0x128]
142078c96: 4d39c8 cmp r8, r9
142078c99: 771d ja 0x142078cb8
142078c9b: eb72 jmp 0x142078d0f ; unknown
142078c9d: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142078ca5: 4c8b8d90000000 mov r9, qword ptr [rbp + 0x90]
142078cac: 4d8b8928010000 mov r9, qword ptr [r9 + 0x128]
142078cb3: 4d39c8 cmp r8, r9
142078cb6: 7657 jbe 0x142078d0f
142078cb8: 4c8b9080000000 mov r10, qword ptr [rax + 0x80]
142078cbf: 4b833cca00 cmp qword ptr [r10 + r9*8], 0
142078cc4: 7449 je 0x142078d0f
142078cc6: 41b201 mov r10b, 1
142078cc9: 448995a0000000 mov dword ptr [rbp + 0xa0], r10d
142078cd0: 4c39ca cmp rdx, r9
142078cd3: 7659 jbe 0x142078d2e
142078cd5: 4c8b5138 mov r10, qword ptr [rcx + 0x38]
142078cd9: 4f8b0cca mov r9, qword ptr [r10 + r9*8]
142078cdd: 4d85c9 test r9, r9
142078ce0: 744c je 0x142078d2e
142078ce2: 4c8b5118 mov r10, qword ptr [rcx + 0x18]
142078ce6: 49f7d1 not r9
142078ce9: 4f8d0c49 lea r9, [r9 + r9*2]
142078ced: 49c1e104 shl r9, 4
142078cf1: 4f8b4c0a10 mov r9, qword ptr [r10 + r9 + 0x10]
142078cf6: 4c894d70 mov qword ptr [rbp + 0x70], r9
142078cfa: 4c8b8d90000000 mov r9, qword ptr [rbp + 0x90]
142078d01: 4d8b8930010000 mov r9, qword ptr [r9 + 0x130]
142078d08: 4d39c8 cmp r8, r9
142078d0b: 773c ja 0x142078d49
142078d0d: eb7e jmp 0x142078d8d ; unknown
142078d0f: c785a000000000000000 mov dword ptr [rbp + 0xa0], 0
142078d19: 4c8b8d90000000 mov r9, qword ptr [rbp + 0x90]
142078d20: 4d8b8930010000 mov r9, qword ptr [r9 + 0x130]
142078d27: 4d39c8 cmp r8, r9
142078d2a: 7661 jbe 0x142078d8d
142078d2c: eb1b jmp 0x142078d49 ; unknown
142078d2e: 48c7457000000000 mov qword ptr [rbp + 0x70], 0
142078d36: 4c8b8d90000000 mov r9, qword ptr [rbp + 0x90]
142078d3d: 4d8b8930010000 mov r9, qword ptr [r9 + 0x130]
142078d44: 4d39c8 cmp r8, r9
142078d47: 7644 jbe 0x142078d8d
142078d49: 4c8b8080000000 mov r8, qword ptr [rax + 0x80]
142078d50: 4b833cc800 cmp qword ptr [r8 + r9*8], 0
142078d55: 7436 je 0x142078d8d
142078d57: 41b001 mov r8b, 1
142078d5a: 448985a4000000 mov dword ptr [rbp + 0xa4], r8d
142078d61: 4c39ca cmp rdx, r9
142078d64: 7633 jbe 0x142078d99
142078d66: 488b5138 mov rdx, qword ptr [rcx + 0x38]
142078d6a: 4a8b14ca mov rdx, qword ptr [rdx + r9*8]
142078d6e: 4885d2 test rdx, rdx
142078d71: 7426 je 0x142078d99
142078d73: 488b4918 mov rcx, qword ptr [rcx + 0x18]
142078d77: 48f7d2 not rdx
142078d7a: 488d1452 lea rdx, [rdx + rdx*2]
142078d7e: 48c1e204 shl rdx, 4
142078d82: 488b4c1110 mov rcx, qword ptr [rcx + rdx + 0x10]
142078d87: 48894d68 mov qword ptr [rbp + 0x68], rcx
142078d8b: eb14 jmp 0x142078da1 ; unknown
142078d8d: c785a400000000000000 mov dword ptr [rbp + 0xa4], 0
142078d97: eb08 jmp 0x142078da1 ; unknown
142078d99: 48c7456800000000 mov qword ptr [rbp + 0x68], 0
142078da1: 488b4050 mov rax, qword ptr [rax + 0x50]
142078da5: 48894548 mov qword ptr [rbp + 0x48], rax
142078da9: 31c0 xor eax, eax
142078dab: 48894578 mov qword ptr [rbp + 0x78], rax
142078daf: 48c1e004 shl rax, 4
142078db3: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
142078db7: 8b440108 mov eax, dword ptr [rcx + rax + 8]
142078dbb: e9c8020000 jmp 0x142079088 ; unknown
142078dc0: 440f28fe movaps xmm15, xmm6
142078dc4: 4d85f6 test r14, r14
142078dc7: 0f8431030000 je 0x1420790fe
142078dcd: 4d8b3e mov r15, qword ptr [r14]
142078dd0: f3410f104e30 movss xmm1, dword ptr [r14 + 0x30]
142078dd6: f3410f10462c movss xmm0, dword ptr [r14 + 0x2c]
142078ddc: f3410f59c3 mulss xmm0, xmm11
142078de1: f3410f58c4 addss xmm0, xmm12
142078de6: 440f28f1 movaps xmm14, xmm1
142078dea: f3450fc2f102 cmpless xmm14, xmm9
142078df0: 440f55f0 andnps xmm14, xmm0
142078df4: 41f6460801 test byte ptr [r14 + 8], 1
142078df9: 7465 je 0x142078e60
142078dfb: 498d7e0c lea rdi, [r14 + 0xc]
142078dff: f2410f10460c movsd xmm0, qword ptr [r14 + 0xc]
142078e05: 4531ed xor r13d, r13d
142078e08: 440f2ec9 ucomiss xmm9, xmm1
142078e0c: 0f8311030000 jae 0x142079123
142078e12: f3410f104e34 movss xmm1, dword ptr [r14 + 0x34]
142078e18: 0f2ece ucomiss xmm1, xmm6
142078e1b: 0f8502030000 jne 0x142079123
142078e21: 0f8afc020000 jp 0x142079123
142078e27: 66410f6e4e3c movd xmm1, dword ptr [r14 + 0x3c]
142078e2d: 660f70d150 pshufd xmm2, xmm1, 0x50
142078e32: 660f72f21f pslld xmm2, 0x1f
142078e37: 660f72e21f psrad xmm2, 0x1f
142078e3c: 660f6e0dc0ca8a00 movd xmm1, dword ptr [rip + 0x8acac0]
142078e44: 660fdbca pand xmm1, xmm2
142078e48: 660fdf1560b78b00 pandn xmm2, xmmword ptr [rip + 0x8bb760]
142078e50: 660febd1 por xmm2, xmm1
142078e54: e9ce020000 jmp 0x142079127 ; unknown
142078e59: 0f1f8000000000 nop dword ptr [rax]
142078e60: 4531ed xor r13d, r13d
142078e63: 4c897d50 mov qword ptr [rbp + 0x50], r15
142078e67: 4989d7 mov r15, rdx
142078e6a: 4c89f7 mov rdi, r14
142078e6d: e91f030000 jmp 0x142079191 ; unknown
142078e72: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142078e80: 4839f0 cmp rax, rsi
142078e83: 0f85f9010000 jne 0x142079082
142078e89: 0f1f8000000000 nop dword ptr [rax]
142078e90: 4d39c4 cmp r12, r8
142078e93: 0f846e040000 je 0x142079307
142078e99: 418b0c24 mov ecx, dword ptr [r12]
142078e9d: 4983c404 add r12, 4
142078ea1: 488b82a8010000 mov rax, qword ptr [rdx + 0x1a8]
142078ea8: 488d0cc9 lea rcx, [rcx + rcx*8]
142078eac: 488b74c810 mov rsi, qword ptr [rax + rcx*8 + 0x10]
142078eb1: 4885f6 test rsi, rsi
142078eb4: 74da je 0x142078e90
142078eb6: 488d04c8 lea rax, [rax + rcx*8]
142078eba: 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
142078ec1: 488b9110010000 mov rdx, qword ptr [rcx + 0x110]
142078ec8: 488b4840 mov rcx, qword ptr [rax + 0x40]
142078ecc: 4839d1 cmp rcx, rdx
142078ecf: 763d jbe 0x142078f0e
142078ed1: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142078ed5: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142078ed9: 4885d2 test rdx, rdx
142078edc: 7430 je 0x142078f0e
142078ede: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142078ee2: 48f7d2 not rdx
142078ee5: 488d1452 lea rdx, [rdx + rdx*2]
142078ee9: 48c1e204 shl rdx, 4
142078eed: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142078ef2: 48899580000000 mov qword ptr [rbp + 0x80], rdx
142078ef9: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
142078f00: 488b9218010000 mov rdx, qword ptr [rdx + 0x118]
142078f07: 4839d1 cmp rcx, rdx
142078f0a: 7720 ja 0x142078f2c
142078f0c: eb5b jmp 0x142078f69 ; unknown
142078f0e: 48c7858000000000000000 mov qword ptr [rbp + 0x80], 0
142078f19: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
142078f20: 488b9218010000 mov rdx, qword ptr [rdx + 0x118]
142078f27: 4839d1 cmp rcx, rdx
142078f2a: 763d jbe 0x142078f69
142078f2c: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142078f30: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142078f34: 4885d2 test rdx, rdx
142078f37: 7430 je 0x142078f69
142078f39: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142078f3d: 48f7d2 not rdx
142078f40: 488d1452 lea rdx, [rdx + rdx*2]
142078f44: 48c1e204 shl rdx, 4
142078f48: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142078f4d: 48899588000000 mov qword ptr [rbp + 0x88], rdx
142078f54: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
142078f5b: 488b9220010000 mov rdx, qword ptr [rdx + 0x120]
142078f62: 4839d1 cmp rcx, rdx
142078f65: 7662 jbe 0x142078fc9
142078f67: eb1e jmp 0x142078f87 ; unknown
142078f69: 48c7858800000000000000 mov qword ptr [rbp + 0x88], 0
142078f74: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
142078f7b: 488b9220010000 mov rdx, qword ptr [rdx + 0x120]
142078f82: 4839d1 cmp rcx, rdx
142078f85: 7642 jbe 0x142078fc9
142078f87: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142078f8b: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142078f8f: 4885d2 test rdx, rdx
142078f92: 7435 je 0x142078fc9
142078f94: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142078f98: 48f7d2 not rdx
142078f9b: 488d1452 lea rdx, [rdx + rdx*2]
142078f9f: 48c1e204 shl rdx, 4
142078fa3: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142078fa8: 48895560 mov qword ptr [rbp + 0x60], rdx
142078fac: b201 mov dl, 1
142078fae: 89959c000000 mov dword ptr [rbp + 0x9c], edx
142078fb4: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
142078fbb: 488b9228010000 mov rdx, qword ptr [rdx + 0x128]
142078fc2: 4839d1 cmp rcx, rdx
142078fc5: 7661 jbe 0x142079028
142078fc7: eb1d jmp 0x142078fe6 ; unknown
142078fc9: c7859c00000000000000 mov dword ptr [rbp + 0x9c], 0
142078fd3: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
142078fda: 488b9228010000 mov rdx, qword ptr [rdx + 0x128]
142078fe1: 4839d1 cmp rcx, rdx
142078fe4: 7642 jbe 0x142079028
142078fe6: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142078fea: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142078fee: 4885d2 test rdx, rdx
142078ff1: 7435 je 0x142079028
142078ff3: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142078ff7: 48f7d2 not rdx
142078ffa: 488d1452 lea rdx, [rdx + rdx*2]
142078ffe: 48c1e204 shl rdx, 4
142079002: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142079007: 48895570 mov qword ptr [rbp + 0x70], rdx
14207900b: b201 mov dl, 1
14207900d: 8995a0000000 mov dword ptr [rbp + 0xa0], edx
142079013: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
14207901a: 488b9230010000 mov rdx, qword ptr [rdx + 0x130]
142079021: 4839d1 cmp rcx, rdx
142079024: 7650 jbe 0x142079076
142079026: eb1d jmp 0x142079045 ; unknown
142079028: c785a000000000000000 mov dword ptr [rbp + 0xa0], 0
142079032: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
142079039: 488b9230010000 mov rdx, qword ptr [rdx + 0x130]
142079040: 4839d1 cmp rcx, rdx
142079043: 7631 jbe 0x142079076
142079045: 488b4838 mov rcx, qword ptr [rax + 0x38]
142079049: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
14207904d: 4885c9 test rcx, rcx
142079050: 7424 je 0x142079076
142079052: 488b4018 mov rax, qword ptr [rax + 0x18]
142079056: 48f7d1 not rcx
142079059: 488d0c49 lea rcx, [rcx + rcx*2]
14207905d: 48c1e104 shl rcx, 4
142079061: 488b440810 mov rax, qword ptr [rax + rcx + 0x10]
142079066: 48894568 mov qword ptr [rbp + 0x68], rax
14207906a: b001 mov al, 1
14207906c: 8985a4000000 mov dword ptr [rbp + 0xa4], eax
142079072: 31c0 xor eax, eax
142079074: eb0c jmp 0x142079082 ; unknown
142079076: 31c0 xor eax, eax
142079078: c785a400000000000000 mov dword ptr [rbp + 0xa4], 0
142079082: 48894578 mov qword ptr [rbp + 0x78], rax
142079086: 89c0 mov eax, eax
142079088: 488d1c40 lea rbx, [rax + rax*2]
14207908c: 488b8d80000000 mov rcx, qword ptr [rbp + 0x80]
142079093: 488d1499 lea rdx, [rcx + rbx*4]
142079097: 48c1e304 shl rbx, 4
14207909b: 48039d88000000 add rbx, qword ptr [rbp + 0x88]
1420790a2: 4c6bf058 imul r14, rax, 0x58
1420790a6: 4c037560 add r14, qword ptr [rbp + 0x60]
1420790aa: f6859c00000001 test byte ptr [rbp + 0x9c], 1
1420790b1: 41b900000000 mov r9d, 0
1420790b7: 4d0f44f1 cmove r14, r9
1420790bb: 488d0c80 lea rcx, [rax + rax*4]
1420790bf: 4c8b4568 mov r8, qword ptr [rbp + 0x68]
1420790c3: 498d3cc8 lea rdi, [r8 + rcx*8]
1420790c7: f685a400000001 test byte ptr [rbp + 0xa4], 1
1420790ce: 490f44f9 cmove rdi, r9
1420790d2: f685a000000001 test byte ptr [rbp + 0xa0], 1
1420790d9: 0f84e1fcffff je 0x142078dc0
1420790df: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
1420790e3: 488d04c1 lea rax, [rcx + rax*8]
1420790e7: 4885c0 test rax, rax
1420790ea: 0f84d0fcffff je 0x142078dc0
1420790f0: f3440f1038 movss xmm15, dword ptr [rax]
1420790f5: 4d85f6 test r14, r14
1420790f8: 0f85cffcffff jne 0x142078dcd
1420790fe: 4885ff test rdi, rdi
142079101: 0f842e030000 je 0x142079435
142079107: 4c8b3f mov r15, qword ptr [rdi]
14207910a: f6470801 test byte ptr [rdi + 8], 1
14207910e: 7470 je 0x142079180
142079110: f20f10470c movsd xmm0, qword ptr [rdi + 0xc]
142079115: 4883c70c add rdi, 0xc
142079119: 450f57f6 xorps xmm14, xmm14
14207911d: 41bd01000000 mov r13d, 1
142079123: 660f6fd7 movdqa xmm2, xmm7
142079127: f20f105f10 movsd xmm3, qword ptr [rdi + 0x10]
14207912c: f3410f12ce movsldup xmm1, xmm14
142079131: 0f59ca mulps xmm1, xmm2
142079134: 0f58cb addps xmm1, xmm3
142079137: b801000000 mov eax, 1
14207913c: 4d85f6 test r14, r14
14207913f: 0f8480000000 je 0x1420791c5
142079145: 450f2e4e30 ucomiss xmm9, dword ptr [r14 + 0x30]
14207914a: 418b4e08 mov ecx, dword ptr [r14 + 8]
14207914e: 498d5cce10 lea rbx, [r14 + rcx*8 + 0x10]
142079153: 4d8d44ce0c lea r8, [r14 + rcx*8 + 0xc]
142079158: f3410f10564c movss xmm2, dword ptr [r14 + 0x4c]
14207915e: 7310 jae 0x142079170
142079160: 0f57c9 xorps xmm1, xmm1
142079163: b901000000 mov ecx, 1
142079168: eb6a jmp 0x1420791d4 ; unknown
14207916a: 660f1f440000 nop word ptr [rax + rax]
142079170: f30f58150037a000 addss xmm2, dword ptr [rip + 0xa03700]
142079178: eb58 jmp 0x1420791d2 ; unknown
14207917a: 660f1f440000 nop word ptr [rax + rax]
142079180: 4c897d50 mov qword ptr [rbp + 0x50], r15
142079184: 4989d7 mov r15, rdx
142079187: 41bd01000000 mov r13d, 1
14207918d: 450f57f6 xorps xmm14, xmm14
142079191: 488d4f0c lea rcx, [rdi + 0xc]
142079195: 0f57c9 xorps xmm1, xmm1
142079198: e8c3e4c1fe call 0x140c97660 ; utils::geometry::circle::Circle2d::get_tangent_at_u
14207919d: f30f105714 movss xmm2, dword ptr [rdi + 0x14]
1420791a2: f30f58d2 addss xmm2, xmm2
1420791a6: f3440f58f2 addss xmm14, xmm2
1420791ab: 0f14c1 unpcklps xmm0, xmm1
1420791ae: f3410f12ce movsldup xmm1, xmm14
1420791b3: 31c0 xor eax, eax
1420791b5: 4c89fa mov rdx, r15
1420791b8: 4c8b7d50 mov r15, qword ptr [rbp + 0x50]
1420791bc: 4d85f6 test r14, r14
1420791bf: 0f8580ffffff jne 0x142079145
1420791c5: 4c8d4310 lea r8, [rbx + 0x10]
1420791c9: f30f105314 movss xmm2, dword ptr [rbx + 0x14]
1420791ce: 4883c318 add rbx, 0x18
1420791d2: 31c9 xor ecx, ecx
1420791d4: f30f1023 movss xmm4, dword ptr [rbx]
1420791d8: f3410f1028 movss xmm5, dword ptr [r8]
1420791dd: f30f116d2c movss dword ptr [rbp + 0x2c], xmm5
1420791e2: f30f115530 movss dword ptr [rbp + 0x30], xmm2
1420791e7: f30f116534 movss dword ptr [rbp + 0x34], xmm4
1420791ec: 4d85f6 test r14, r14
1420791ef: 742f je 0x142079220
1420791f1: 450fb65640 movzx r10d, byte ptr [r14 + 0x40]
1420791f6: 4183fa02 cmp r10d, 2
1420791fa: 7434 je 0x142079230
1420791fc: 4531db xor r11d, r11d
1420791ff: 4183fa01 cmp r10d, 1
142079203: 0f85e7000000 jne 0x1420792f0
142079209: 4531c0 xor r8d, r8d
14207920c: 4531c9 xor r9d, r9d
14207920f: 4180fa02 cmp r10b, 2
142079213: 7331 jae 0x142079246
142079215: eb33 jmp 0x14207924a ; unknown
142079217: 660f1f840000000000 nop word ptr [rax + rax]
142079220: 4531c9 xor r9d, r9d
142079223: 4531db xor r11d, r11d
142079226: 4531c0 xor r8d, r8d
142079229: eb1f jmp 0x14207924a ; unknown
14207922b: 0f1f440000 nop dword ptr [rax + rax]
142079230: 450fb64641 movzx r8d, byte ptr [r14 + 0x41]
142079235: 4183c802 or r8d, 2
142079239: 458b5e44 mov r11d, dword ptr [r14 + 0x44]
14207923d: 4531c9 xor r9d, r9d
142079240: 4180fa02 cmp r10b, 2
142079244: 7204 jb 0x14207924a
142079246: 458b4e48 mov r9d, dword ptr [r14 + 0x48]
14207924a: 440f28f1 movaps xmm14, xmm1
14207924e: 450f54f0 andps xmm14, xmm8
142079252: f3440f16e9 movshdup xmm13, xmm1
142079257: 450f54e8 andps xmm13, xmm8
14207925b: 410f28de movaps xmm3, xmm14
14207925f: f3410fc2de03 cmpunordss xmm3, xmm14
142079265: 440f28d3 movaps xmm10, xmm3
142079269: 450f54d5 andps xmm10, xmm13
14207926d: f3450f5fee maxss xmm13, xmm14
142079272: 410f55dd andnps xmm3, xmm13
142079276: 410f56da orps xmm3, xmm10
14207927a: f30f116dc0 movss dword ptr [rbp - 0x40], xmm5
14207927f: f30f1155c4 movss dword ptr [rbp - 0x3c], xmm2
142079284: f30f1165c8 movss dword ptr [rbp - 0x38], xmm4
142079289: 44897dcc mov dword ptr [rbp - 0x34], r15d
14207928d: 0f16c1 movlhps xmm0, xmm1
142079290: 0f1145d0 movups xmmword ptr [rbp - 0x30], xmm0
142079294: f3440f117de0 movss dword ptr [rbp - 0x20], xmm15
14207929a: 8945e4 mov dword ptr [rbp - 0x1c], eax
14207929d: 44896de8 mov dword ptr [rbp - 0x18], r13d
1420792a1: 894dec mov dword ptr [rbp - 0x14], ecx
1420792a4: 448945f0 mov dword ptr [rbp - 0x10], r8d
1420792a8: 44895df4 mov dword ptr [rbp - 0xc], r11d
1420792ac: 44894df8 mov dword ptr [rbp - 8], r9d
1420792b0: c745fc00000000 mov dword ptr [rbp - 4], 0
1420792b7: 488d45c0 lea rax, [rbp - 0x40]
1420792bb: 4889442420 mov qword ptr [rsp + 0x20], rax
1420792c0: 488b4d10 mov rcx, qword ptr [rbp + 0x10]
1420792c4: 4c8d452c lea r8, [rbp + 0x2c]
1420792c8: e8c3734fff call 0x141570690 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::StoneFloorInstanceData>
1420792cd: 488b4578 mov rax, qword ptr [rbp + 0x78]
1420792d1: 48ffc0 inc rax
1420792d4: 488b5518 mov rdx, qword ptr [rbp + 0x18]
1420792d8: 4c8b4520 mov r8, qword ptr [rbp + 0x20]
1420792dc: 80bd9b00000000 cmp byte ptr [rbp + 0x9b], 0
1420792e3: 0f8425f8ffff je 0x142078b0e
1420792e9: e992fbffff jmp 0x142078e80 ; unknown
1420792ee: 6690 nop 
1420792f0: 450fb64654 movzx r8d, byte ptr [r14 + 0x54]
1420792f5: 4531c9 xor r9d, r9d
1420792f8: 4180fa02 cmp r10b, 2
1420792fc: 0f8344ffffff jae 0x142079246
142079302: e943ffffff jmp 0x14207924a ; unknown
142079307: 488b7558 mov rsi, qword ptr [rbp + 0x58]
14207930b: 4889f1 mov rcx, rsi
14207930e: e8fd8ac0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
142079313: 4889f1 mov rcx, rsi
142079316: 4883c901 or rcx, 1
14207931a: e8f18ac0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207931f: 31c9 xor ecx, ecx
142079321: b808000000 mov eax, 8
142079326: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
14207932b: 0f85f5000000 jne 0x142079426
142079331: 48837d3800 cmp qword ptr [rbp + 0x38], 0
142079336: 7442 je 0x14207937a
142079338: e8f3edb9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207933d: 4889c1 mov rcx, rax
142079340: 488b00 mov rax, qword ptr [rax]
142079343: 4883f801 cmp rax, 1
142079347: 0f858f000000 jne 0x1420793dc
14207934d: 4883c108 add rcx, 8
142079351: 48833900 cmp qword ptr [rcx], 0
142079355: 0f85bf000000 jne 0x14207941a
14207935b: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142079362: 48894d78 mov qword ptr [rbp + 0x78], rcx
142079366: 4883c108 add rcx, 8
14207936a: 488b5540 mov rdx, qword ptr [rbp + 0x40]
14207936e: e87dddb9fe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
142079373: 488b4578 mov rax, qword ptr [rbp + 0x78]
142079377: 48ff00 inc qword ptr [rax]
14207937a: 0f28b5b0000000 movaps xmm6, xmmword ptr [rbp + 0xb0]
142079381: 0f28bdc0000000 movaps xmm7, xmmword ptr [rbp + 0xc0]
142079388: 440f2885d0000000 movaps xmm8, xmmword ptr [rbp + 0xd0]
142079390: 440f288de0000000 movaps xmm9, xmmword ptr [rbp + 0xe0]
142079398: 440f2895f0000000 movaps xmm10, xmmword ptr [rbp + 0xf0]
1420793a0: 440f289d00010000 movaps xmm11, xmmword ptr [rbp + 0x100]
1420793a8: 440f28a510010000 movaps xmm12, xmmword ptr [rbp + 0x110]
1420793b0: 440f28ad20010000 movaps xmm13, xmmword ptr [rbp + 0x120]
1420793b8: 440f28b530010000 movaps xmm14, xmmword ptr [rbp + 0x130]
1420793c0: 440f28bd40010000 movaps xmm15, xmmword ptr [rbp + 0x140]
1420793c8: 4881c4d8010000 add rsp, 0x1d8
1420793cf: 5b pop rbx
1420793d0: 5f pop rdi
1420793d1: 5e pop rsi
1420793d2: 415c pop r12
1420793d4: 415d pop r13
1420793d6: 415e pop r14
1420793d8: 415f pop r15
1420793da: 5d pop rbp
1420793db: c3 ret 
1420793dc: 4885c0 test rax, rax
1420793df: 752d jne 0x14207940e
1420793e1: 31d2 xor edx, edx
1420793e3: e8e8f38700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1420793e8: 4889c1 mov rcx, rax
1420793eb: 4885c0 test rax, rax
1420793ee: 0f855dffffff jne 0x142079351
1420793f4: eb18 jmp 0x14207940e ; unknown
1420793f6: 4885c0 test rax, rax
1420793f9: 7513 jne 0x14207940e
1420793fb: 31d2 xor edx, edx
1420793fd: e8cef38700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142079402: 4889c1 mov rcx, rax
142079405: 4885c0 test rax, rax
142079408: 0f85cef5ffff jne 0x1420789dc
14207940e: 488d0d8b16bc00 lea rcx, [rip + 0xbc168b]
142079415: e8b6ec8500 call 0x1428d80d0 ; std::thread::local::panic_access_error
14207941a: 488d0d4f17bc00 lea rcx, [rip + 0xbc174f]
142079421: e82afe8500 call 0x1428d9250 ; core::cell::panic_already_borrowed
142079426: 4889f1 mov rcx, rsi
142079429: 31d2 xor edx, edx
14207942b: e8302b8500 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
142079430: e9fcfeffff jmp 0x142079331 ; unknown
142079435: 488d0d4ca2f500 lea rcx, [rip + 0xf5a24c]
14207943c: e8cffe8500 call 0x1428d9310 ; core::option::unwrap_failed
142079441: 0f0b ud2 
142079443: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142079450: 4889542410 mov qword ptr [rsp + 0x10], rdx
142079455: 55 push rbp
142079456: 4157 push r15
142079458: 4156 push r14
14207945a: 4155 push r13
14207945c: 4154 push r12
14207945e: 56 push rsi
14207945f: 57 push rdi
142079460: 53 push rbx
142079461: 4881ecd8000000 sub rsp, 0xd8
142079468: 488daa80000000 lea rbp, [rdx + 0x80]
14207946f: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
142079475: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
14207947b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
142079481: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
142079487: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
14207948d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
142079496: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
14207949f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
1420794a8: 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
1420794b1: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1420794b9: 488b4578 mov rax, qword ptr [rbp + 0x78]
1420794bd: 48ff00 inc qword ptr [rax]
1420794c0: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
1420794c8: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
1420794d0: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
1420794d9: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
1420794e2: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
1420794eb: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
1420794f1: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
1420794f7: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
1420794fd: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
142079503: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
142079509: 4881c4d8000000 add rsp, 0xd8
142079510: 5b pop rbx
142079511: 5f pop rdi
142079512: 5e pop rsi
142079513: 415c pop r12
142079515: 415d pop r13
142079517: 415e pop r14
142079519: 415f pop r15
14207951b: 5d pop rbp
14207951c: c3 ret 
14207951d: 0f1f00 nop dword ptr [rax]
142079520: 4889542410 mov qword ptr [rsp + 0x10], rdx
142079525: 55 push rbp
142079526: 4157 push r15
142079528: 4156 push r14
14207952a: 4155 push r13
14207952c: 4154 push r12
14207952e: 56 push rsi
14207952f: 57 push rdi
142079530: 53 push rbx
142079531: 4881ecd8000000 sub rsp, 0xd8
142079538: 488daa80000000 lea rbp, [rdx + 0x80]
14207953f: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
142079545: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
14207954b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
142079551: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
142079557: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
14207955d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
142079566: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
14207956f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
142079578: 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
142079581: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
142079589: 488b4578 mov rax, qword ptr [rbp + 0x78]
14207958d: 48ff00 inc qword ptr [rax]
142079590: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
142079598: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
1420795a0: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
1420795a9: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
1420795b2: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
1420795bb: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
1420795c1: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
1420795c7: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
1420795cd: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
1420795d3: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
1420795d9: 4881c4d8000000 add rsp, 0xd8
1420795e0: 5b pop rbx
1420795e1: 5f pop rdi
1420795e2: 5e pop rsi
1420795e3: 415c pop r12
1420795e5: 415d pop r13
1420795e7: 415e pop r14
1420795e9: 415f pop r15
1420795eb: 5d pop rbp
1420795ec: c3 ret 
1420795ed: 0f1f00 nop dword ptr [rax]
1420795f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420795f5: 55 push rbp
1420795f6: 4157 push r15
1420795f8: 4156 push r14
1420795fa: 4155 push r13
1420795fc: 4154 push r12
1420795fe: 56 push rsi
1420795ff: 57 push rdi
142079600: 53 push rbx
142079601: 4881ecd8000000 sub rsp, 0xd8
142079608: 488daa80000000 lea rbp, [rdx + 0x80]
14207960f: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
142079615: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
14207961b: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
142079621: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
142079627: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
14207962d: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
142079636: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
14207963f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
142079648: 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
142079651: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
142079659: 488b4d58 mov rcx, qword ptr [rbp + 0x58]
14207965d: e86e7cfcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> > | core::ptr::drop_in_place<lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::Ra
142079662: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
14207966a: 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
142079673: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
14207967c: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
142079685: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
14207968e: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
142079694: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
14207969a: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
1420796a0: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
1420796a6: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
1420796ac: 4881c4d8000000 add rsp, 0xd8
1420796b3: 5b pop rbx
1420796b4: 5f pop rdi
1420796b5: 5e pop rsi
1420796b6: 415c pop r12
1420796b8: 415d pop r13
1420796ba: 415e pop r14
1420796bc: 415f pop r15
1420796be: 5d pop rbp
1420796bf: c3 ret 
1420796c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420796c5: 55 push rbp
1420796c6: 4157 push r15
1420796c8: 4156 push r14
1420796ca: 4155 push r13
1420796cc: 4154 push r12
1420796ce: 56 push rsi
1420796cf: 57 push rdi
1420796d0: 53 push rbx
1420796d1: 4881ecd8000000 sub rsp, 0xd8
1420796d8: 488daa80000000 lea rbp, [rdx + 0x80]
1420796df: 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
1420796e5: 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
1420796eb: 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
1420796f1: 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
1420796f7: 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
1420796fd: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
142079706: 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
14207970f: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
142079718: 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
142079721: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
142079729: 48837d3800 cmp qword ptr [rbp + 0x38], 0
14207972e: 741f je 0x14207974f
142079730: 488d4540 lea rax, [rbp + 0x40]
142079734: 48894508 mov qword ptr [rbp + 8], rax
142079738: 488d4508 lea rax, [rbp + 8]
14207973c: 48894500 mov qword ptr [rbp], rax
142079740: 488d0d119cf500 lea rcx, [rip + 0xf59c11]
142079747: 4889ea mov rdx, rbp
14207974a: e8815b2bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
14207974f: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
142079757: 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
14207975f: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
142079768: 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
142079771: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
14207977a: 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
142079780: 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
142079786: 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
14207978c: 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
142079792: 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
142079798: 4881c4d8000000 add rsp, 0xd8
14207979f: 5b pop rbx
1420797a0: 5f pop rdi
1420797a1: 5e pop rsi
1420797a2: 415c pop r12
1420797a4: 415d pop r13
1420797a6: 415e pop r14
1420797a8: 415f pop r15
1420797aa: 5d pop rbp
1420797ab: c3 ret 
