1407f66b0: 55 push rbp
1407f66b1: 4157 push r15
1407f66b3: 4156 push r14
1407f66b5: 4155 push r13
1407f66b7: 4154 push r12
1407f66b9: 56 push rsi
1407f66ba: 57 push rdi
1407f66bb: 53 push rbx
1407f66bc: 4881ec48030000 sub rsp, 0x348
1407f66c3: 488dac2480000000 lea rbp, [rsp + 0x80]
1407f66cb: 440f29bdb0020000 movaps xmmword ptr [rbp + 0x2b0], xmm15
1407f66d3: 440f29b5a0020000 movaps xmmword ptr [rbp + 0x2a0], xmm14
1407f66db: 440f29ad90020000 movaps xmmword ptr [rbp + 0x290], xmm13
1407f66e3: 440f29a580020000 movaps xmmword ptr [rbp + 0x280], xmm12
1407f66eb: 440f299d70020000 movaps xmmword ptr [rbp + 0x270], xmm11
1407f66f3: 440f299560020000 movaps xmmword ptr [rbp + 0x260], xmm10
1407f66fb: 440f298d50020000 movaps xmmword ptr [rbp + 0x250], xmm9
1407f6703: 440f298540020000 movaps xmmword ptr [rbp + 0x240], xmm8
1407f670b: 0f29bd30020000 movaps xmmword ptr [rbp + 0x230], xmm7
1407f6712: 0f29b520020000 movaps xmmword ptr [rbp + 0x220], xmm6
1407f6719: 48c78518020000feffffff mov qword ptr [rbp + 0x218], 0xfffffffffffffffe
1407f6724: 4d89cd mov r13, r9
1407f6727: 4d89c7 mov r15, r8
1407f672a: 4889d3 mov rbx, rdx
1407f672d: 4989ce mov r14, rcx
1407f6730: e8cb024200 call 0x140c16a00 ; puffin::are_scopes_on
1407f6735: 888516020000 mov byte ptr [rbp + 0x216], al
1407f673b: 84c0 test al, al
1407f673d: 0f8420020000 je 0x1407f6963
1407f6743: 41bc5a000000 mov r12d, 0x5a
1407f6749: b95a000000 mov ecx, 0x5a
1407f674e: e8ddfaffff call 0x1407f6230 ; core::str::impl$0::rfind<ref$<str$> >
1407f6753: 488d3d46572902 lea rdi, [rip + 0x2295746]
1407f675a: 4883f801 cmp rax, 1
1407f675e: 0f85a1000000 jne 0x1407f6805
1407f6764: 4989d1 mov r9, rdx
1407f6767: 4885d2 test rdx, rdx
1407f676a: 7436 je 0x1407f67a2
1407f676c: 4983f95a cmp r9, 0x5a
1407f6770: 732e jae 0x1407f67a0
1407f6772: 488d0527572902 lea rax, [rip + 0x2295727]
1407f6779: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1407f677e: 7f22 jg 0x1407f67a2
1407f6780: 488d05f1572902 lea rax, [rip + 0x22957f1]
1407f6787: 4889442420 mov qword ptr [rsp + 0x20], rax
1407f678c: 488d0d0d572902 lea rcx, [rip + 0x229570d]
1407f6793: ba5a000000 mov edx, 0x5a
1407f6798: 4531c0 xor r8d, r8d
1407f679b: e8a0310e02 call 0x1428d9940 ; core::str::slice_error_fail
1407f67a0: 75de jne 0x1407f6780
1407f67a2: 4c89c9 mov rcx, r9
1407f67a5: e886faffff call 0x1407f6230 ; core::str::impl$0::rfind<ref$<str$> >
1407f67aa: 4883f801 cmp rax, 1
1407f67ae: 7555 jne 0x1407f6805
1407f67b0: 4989d0 mov r8, rdx
1407f67b3: 4983c002 add r8, 2
1407f67b7: 7439 je 0x1407f67f2
1407f67b9: 4983f85a cmp r8, 0x5a
1407f67bd: 7331 jae 0x1407f67f0
1407f67bf: 488d05da562902 lea rax, [rip + 0x22956da]
1407f67c6: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
1407f67cb: 7f25 jg 0x1407f67f2
1407f67cd: 488d05bc572902 lea rax, [rip + 0x22957bc]
1407f67d4: 4889442420 mov qword ptr [rsp + 0x20], rax
1407f67d9: 488d0dc0562902 lea rcx, [rip + 0x22956c0]
1407f67e0: ba5a000000 mov edx, 0x5a
1407f67e5: 41b95a000000 mov r9d, 0x5a
1407f67eb: e850310e02 call 0x1428d9940 ; core::str::slice_error_fail
1407f67f0: 75db jne 0x1407f67cd
1407f67f2: 41bc58000000 mov r12d, 0x58
1407f67f8: 4929d4 sub r12, rdx
1407f67fb: 488d3d9e562902 lea rdi, [rip + 0x229569e]
1407f6802: 4c01c7 add rdi, r8
1407f6805: 4c89ad60010000 mov qword ptr [rbp + 0x160], r13
1407f680c: 4c8d0521582902 lea r8, [rip + 0x2295821]
1407f6813: 4c8d2dd4572902 lea r13, [rip + 0x22957d4]
1407f681a: 660f1f440000 nop word ptr [rax + rax]
1407f6820: 410fbe40ff movsx eax, byte ptr [r8 - 1]
1407f6825: 85c0 test eax, eax
1407f6827: 7817 js 0x1407f6840
1407f6829: 49ffc8 dec r8
1407f682c: 83f85c cmp eax, 0x5c
1407f682f: 7563 jne 0x1407f6894
1407f6831: eb72 jmp 0x1407f68a5 ; unknown
1407f6833: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407f6840: 410fb648fe movzx ecx, byte ptr [r8 - 2]
1407f6845: 80f9c0 cmp cl, 0xc0
1407f6848: 7d23 jge 0x1407f686d
1407f684a: 410fb650fd movzx edx, byte ptr [r8 - 3]
1407f684f: 80fac0 cmp dl, 0xc0
1407f6852: 7d22 jge 0x1407f6876
1407f6854: 450fb648fc movzx r9d, byte ptr [r8 - 4]
1407f6859: 4983c0fc add r8, -4
1407f685d: 4183e107 and r9d, 7
1407f6861: 41c1e106 shl r9d, 6
1407f6865: 83e23f and edx, 0x3f
1407f6868: 4409ca or edx, r9d
1407f686b: eb10 jmp 0x1407f687d ; unknown
1407f686d: 4983c0fe add r8, -2
1407f6871: 83e11f and ecx, 0x1f
1407f6874: eb0f jmp 0x1407f6885 ; unknown
1407f6876: 4983c0fd add r8, -3
1407f687a: 83e20f and edx, 0xf
1407f687d: c1e206 shl edx, 6
1407f6880: 83e13f and ecx, 0x3f
1407f6883: 09d1 or ecx, edx
1407f6885: c1e106 shl ecx, 6
1407f6888: 243f and al, 0x3f
1407f688a: 0fb6c0 movzx eax, al
1407f688d: 09c8 or eax, ecx
1407f688f: 83f85c cmp eax, 0x5c
1407f6892: 7411 je 0x1407f68a5
1407f6894: 83f82f cmp eax, 0x2f
1407f6897: 740c je 0x1407f68a5
1407f6899: 4d39e8 cmp r8, r13
1407f689c: 7582 jne 0x1407f6820
1407f689e: be46000000 mov esi, 0x46
1407f68a3: eb4c jmp 0x1407f68f1 ; unknown
1407f68a5: 4c8d2d42572902 lea r13, [rip + 0x2295742]
1407f68ac: 4d29e8 sub r8, r13
1407f68af: 49ffc0 inc r8
1407f68b2: 7432 je 0x1407f68e6
1407f68b4: 4983f846 cmp r8, 0x46
1407f68b8: 732a jae 0x1407f68e4
1407f68ba: 43803c28bf cmp byte ptr [r8 + r13], 0xbf
1407f68bf: 7f25 jg 0x1407f68e6
1407f68c1: 488d0590562902 lea rax, [rip + 0x2295690]
1407f68c8: 4889442420 mov qword ptr [rsp + 0x20], rax
1407f68cd: 488d0d1a572902 lea rcx, [rip + 0x229571a]
1407f68d4: ba46000000 mov edx, 0x46
1407f68d9: 41b946000000 mov r9d, 0x46
1407f68df: e85c300e02 call 0x1428d9940 ; core::str::slice_error_fail
1407f68e4: 75db jne 0x1407f68c1
1407f68e6: be46000000 mov esi, 0x46
1407f68eb: 4c29c6 sub rsi, r8
1407f68ee: 4d01c5 add r13, r8
1407f68f1: e83a184200 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1407f68f6: 4889c1 mov rcx, rax
1407f68f9: 488b00 mov rax, qword ptr [rax]
1407f68fc: 4883f801 cmp rax, 1
1407f6900: 0f85981b0000 jne 0x1407f849e
1407f6906: 4883c108 add rcx, 8
1407f690a: 48833900 cmp qword ptr [rcx], 0
1407f690e: 0f85e51b0000 jne 0x1407f84f9
1407f6914: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1407f691b: 48898d30010000 mov qword ptr [rbp + 0x130], rcx
1407f6922: 4883c108 add rcx, 8
1407f6926: 4889742420 mov qword ptr [rsp + 0x20], rsi
1407f692b: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
1407f6934: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
1407f693d: 4889fa mov rdx, rdi
1407f6940: 4d89e0 mov r8, r12
1407f6943: 4d89e9 mov r9, r13
1407f6946: e8f5034200 call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1407f694b: 48898590000000 mov qword ptr [rbp + 0x90], rax
1407f6952: 488b8530010000 mov rax, qword ptr [rbp + 0x130]
1407f6959: 48ff00 inc qword ptr [rax]
1407f695c: 4c8bad60010000 mov r13, qword ptr [rbp + 0x160]
1407f6963: 4c89f1 mov rcx, r14
1407f6966: e805013300 call 0x140b26a70 ; country_core::resources::walls::decorator_subtype::impl$8::from
1407f696b: 48898520010000 mov qword ptr [rbp + 0x120], rax
1407f6972: 48899528010000 mov qword ptr [rbp + 0x128], rdx
1407f6979: 488b13 mov rdx, qword ptr [rbx]
1407f697c: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f6983: e878472200 call 0x140a1b100 ; country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids | country_core::resources::walls::decorator_storage::DecoratorStorage::iter_walls | country_core::resources::tree_storage::TreeStorage::iter
1407f6988: 488b8d38030000 mov rcx, qword ptr [rbp + 0x338]
1407f698f: 488b9530030000 mov rdx, qword ptr [rbp + 0x330]
1407f6996: 4c8bb5c0010000 mov r14, qword ptr [rbp + 0x1c0]
1407f699d: 488bb5c8010000 mov rsi, qword ptr [rbp + 0x1c8]
1407f69a4: 0fb785d8010000 movzx eax, word ptr [rbp + 0x1d8]
1407f69ab: 4c8b85e0010000 mov r8, qword ptr [rbp + 0x1e0]
1407f69b2: 4d8b4d00 mov r9, qword ptr [r13]
1407f69b6: 4c894d30 mov qword ptr [rbp + 0x30], r9
1407f69ba: 4d8b0f mov r9, qword ptr [r15]
1407f69bd: 4c894d38 mov qword ptr [rbp + 0x38], r9
1407f69c1: 448b4a1c mov r9d, dword ptr [rdx + 0x1c]
1407f69c5: 44898da4000000 mov dword ptr [rbp + 0xa4], r9d
1407f69cc: 4c8b0a mov r9, qword ptr [rdx]
1407f69cf: 4c894d48 mov qword ptr [rbp + 0x48], r9
1407f69d3: 488b5210 mov rdx, qword ptr [rdx + 0x10]
1407f69d7: 48895540 mov qword ptr [rbp + 0x40], rdx
1407f69db: 488b09 mov rcx, qword ptr [rcx]
1407f69de: 48894d50 mov qword ptr [rbp + 0x50], rcx
1407f69e2: 4c8985a8000000 mov qword ptr [rbp + 0xa8], r8
1407f69e9: 4d85c0 test r8, r8
1407f69ec: 0f84fc180000 je 0x1407f82ee
1407f69f2: 6685c0 test ax, ax
1407f69f5: 7419 je 0x1407f6a10
1407f69f7: 4d85f6 test r14, r14
1407f69fa: 0f84ee180000 je 0x1407f82ee
1407f6a00: 8d48ff lea ecx, [rax - 1]
1407f6a03: 21c1 and ecx, eax
1407f6a05: 898d5c010000 mov dword ptr [rbp + 0x15c], ecx
1407f6a0b: eb30 jmp 0x1407f6a3d ; unknown
1407f6a0d: 0f1f00 nop dword ptr [rax]
1407f6a10: 660f6f06 movdqa xmm0, xmmword ptr [rsi]
1407f6a14: 660fd7c0 pmovmskb eax, xmm0
1407f6a18: 4981c680f6ffff add r14, -0x980
1407f6a1f: 4883c610 add rsi, 0x10
1407f6a23: 3dffff0000 cmp eax, 0xffff
1407f6a28: 74e6 je 0x1407f6a10
1407f6a2a: b9feffffff mov ecx, 0xfffffffe
1407f6a2f: 29c1 sub ecx, eax
1407f6a31: f7d0 not eax
1407f6a33: 89c2 mov edx, eax
1407f6a35: 21ca and edx, ecx
1407f6a37: 89955c010000 mov dword ptr [rbp + 0x15c], edx
1407f6a3d: f30fbcc0 tzcnt eax, eax
1407f6a41: 48f7d8 neg rax
1407f6a44: 4869c098000000 imul rax, rax, 0x98
1407f6a4b: 498d1406 lea rdx, [r14 + rax]
1407f6a4f: 4881c270ffffff add rdx, -0x90
1407f6a56: 498b840668ffffff mov rax, qword ptr [r14 + rax - 0x98]
1407f6a5e: 488985c0000000 mov qword ptr [rbp + 0xc0], rax
1407f6a65: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f6a6c: e8ff452200 call 0x140a1b070 ; country_core::resources::walls::decorator_storage::Decorators::iter_modified
1407f6a71: 48ff8da8000000 dec qword ptr [rbp + 0xa8]
1407f6a78: 4c8ba5c0010000 mov r12, qword ptr [rbp + 0x1c0]
1407f6a7f: 4c8badc8010000 mov r13, qword ptr [rbp + 0x1c8]
1407f6a86: 0fb78dd8010000 movzx ecx, word ptr [rbp + 0x1d8]
1407f6a8d: 488b85e0010000 mov rax, qword ptr [rbp + 0x1e0]
1407f6a94: 48898560010000 mov qword ptr [rbp + 0x160], rax
1407f6a9b: 488b85e8010000 mov rax, qword ptr [rbp + 0x1e8]
1407f6aa2: 48898530010000 mov qword ptr [rbp + 0x130], rax
1407f6aa9: 4883c050 add rax, 0x50
1407f6aad: 48894528 mov qword ptr [rbp + 0x28], rax
1407f6ab1: 4883bd6001000000 cmp qword ptr [rbp + 0x160], 0
1407f6ab9: 0f8412180000 je 0x1407f82d1
1407f6abf: 6685c9 test cx, cx
1407f6ac2: 741c je 0x1407f6ae0
1407f6ac4: 8d41ff lea eax, [rcx - 1]
1407f6ac7: 21c8 and eax, ecx
1407f6ac9: 898574010000 mov dword ptr [rbp + 0x174], eax
1407f6acf: 4d85e4 test r12, r12
1407f6ad2: 7539 jne 0x1407f6b0d
1407f6ad4: e9f8170000 jmp 0x1407f82d1 ; unknown
1407f6ad9: 0f1f8000000000 nop dword ptr [rax]
1407f6ae0: 66410f6f4500 movdqa xmm0, xmmword ptr [r13]
1407f6ae6: 660fd7c8 pmovmskb ecx, xmm0
1407f6aea: 4983c4c0 add r12, -0x40
1407f6aee: 4983c510 add r13, 0x10
1407f6af2: 81f9ffff0000 cmp ecx, 0xffff
1407f6af8: 74e6 je 0x1407f6ae0
1407f6afa: b8feffffff mov eax, 0xfffffffe
1407f6aff: 29c8 sub eax, ecx
1407f6b01: f7d1 not ecx
1407f6b03: 89ca mov edx, ecx
1407f6b05: 21c2 and edx, eax
1407f6b07: 899574010000 mov dword ptr [rbp + 0x174], edx
1407f6b0d: f30fbcc1 tzcnt eax, ecx
1407f6b11: c1e002 shl eax, 2
1407f6b14: 4c89e1 mov rcx, r12
1407f6b17: 4829c1 sub rcx, rax
1407f6b1a: 8b41fc mov eax, dword ptr [rcx - 4]
1407f6b1d: 48898568010000 mov qword ptr [rbp + 0x168], rax
1407f6b24: 8985c0010000 mov dword ptr [rbp + 0x1c0], eax
1407f6b2a: 488b8530010000 mov rax, qword ptr [rbp + 0x130]
1407f6b31: 4883784800 cmp qword ptr [rax + 0x48], 0
1407f6b36: 0f8468180000 je 0x1407f83a4
1407f6b3c: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
1407f6b40: 488d95c0010000 lea rdx, [rbp + 0x1c0]
1407f6b47: e884a3dbff call 0x1405b0ed0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<shader_pipeline::reflection::BindingIndex> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::walls::d
1407f6b4c: 48ff8d60010000 dec qword ptr [rbp + 0x160]
1407f6b53: 488b8d30010000 mov rcx, qword ptr [rbp + 0x130]
1407f6b5a: 4c8b7930 mov r15, qword ptr [rcx + 0x30]
1407f6b5e: 488b4938 mov rcx, qword ptr [rcx + 0x38]
1407f6b62: 4989cb mov r11, rcx
1407f6b65: 4921c3 and r11, rax
1407f6b68: 48c1e839 shr rax, 0x39
1407f6b6c: 660f6ec0 movd xmm0, eax
1407f6b70: 660f60c0 punpcklbw xmm0, xmm0
1407f6b74: f20f70c000 pshuflw xmm0, xmm0, 0
1407f6b79: 660f70c000 pshufd xmm0, xmm0, 0
1407f6b7e: 498d47b0 lea rax, [r15 - 0x50]
1407f6b82: 4531c0 xor r8d, r8d
1407f6b85: f3430f6f0c1f movdqu xmm1, xmmword ptr [r15 + r11]
1407f6b8b: 660f6fd1 movdqa xmm2, xmm1
1407f6b8f: 660f74d0 pcmpeqb xmm2, xmm0
1407f6b93: 66440fd7ca pmovmskb r9d, xmm2
1407f6b98: 4585c9 test r9d, r9d
1407f6b9b: 7433 je 0x1407f6bd0
1407f6b9d: f3450fbcd1 tzcnt r10d, r9d
1407f6ba2: 4d01da add r10, r11
1407f6ba5: 4921ca and r10, rcx
1407f6ba8: 49f7da neg r10
1407f6bab: 4b8d3c92 lea rdi, [r10 + r10*4]
1407f6baf: 48c1e704 shl rdi, 4
1407f6bb3: 488b9568010000 mov rdx, qword ptr [rbp + 0x168]
1407f6bba: 3b1438 cmp edx, dword ptr [rax + rdi]
1407f6bbd: 7441 je 0x1407f6c00
1407f6bbf: 458d51ff lea r10d, [r9 - 1]
1407f6bc3: 664521ca and r10w, r9w
1407f6bc7: 4589d1 mov r9d, r10d
1407f6bca: 75d1 jne 0x1407f6b9d
1407f6bcc: 0f1f4000 nop dword ptr [rax]
1407f6bd0: 660f740db8ed1202 pcmpeqb xmm1, xmmword ptr [rip + 0x212edb8]
1407f6bd8: 66440fd7c9 pmovmskb r9d, xmm1
1407f6bdd: 4585c9 test r9d, r9d
1407f6be0: 0f85be170000 jne 0x1407f83a4
1407f6be6: 4d01c3 add r11, r8
1407f6be9: 4983c310 add r11, 0x10
1407f6bed: 4983c010 add r8, 0x10
1407f6bf1: 4921cb and r11, rcx
1407f6bf4: eb8f jmp 0x1407f6b85 ; unknown
1407f6bf6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407f6c00: 488d8d20010000 lea rcx, [rbp + 0x120]
1407f6c07: e864fd3200 call 0x140b26970 ; country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
1407f6c0c: 8b8d74010000 mov ecx, dword ptr [rbp + 0x174]
1407f6c12: 3c03 cmp al, 3
1407f6c14: 0f8597feffff jne 0x1407f6ab1
1407f6c1a: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
1407f6c1e: 488b95c0000000 mov rdx, qword ptr [rbp + 0xc0]
1407f6c25: 4c8d050c542902 lea r8, [rip + 0x229540c]
1407f6c2c: e8bfb22900 call 0x140a91ef0 ; country_core::resources::walls::public_walls::PublicWalls::get
1407f6c31: 488985b8010000 mov qword ptr [rbp + 0x1b8], rax
1407f6c38: 4883bdb801000000 cmp qword ptr [rbp + 0x1b8], 0
1407f6c40: 0f8441190000 je 0x1407f8587
1407f6c46: 498d1c3f lea rbx, [r15 + rdi]
1407f6c4a: 0fb643ff movzx eax, byte ptr [rbx - 1]
1407f6c4e: 888538010000 mov byte ptr [rbp + 0x138], al
1407f6c54: 8b43e0 mov eax, dword ptr [rbx - 0x20]
1407f6c57: 8985c8000000 mov dword ptr [rbp + 0xc8], eax
1407f6c5d: 488d4de0 lea rcx, [rbp - 0x20]
1407f6c61: 488b5538 mov rdx, qword ptr [rbp + 0x38]
1407f6c65: 4c8b8568010000 mov r8, qword ptr [rbp + 0x168]
1407f6c6c: 4c8d0df5532902 lea r9, [rip + 0x22953f5]
1407f6c73: e8a83d2200 call 0x140a1aa20 ; country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
1407f6c78: 4c89b588000000 mov qword ptr [rbp + 0x88], r14
1407f6c7f: 48899d08020000 mov qword ptr [rbp + 0x208], rbx
1407f6c86: 440fb673fc movzx r14d, byte ptr [rbx - 4]
1407f6c8b: 4d85f6 test r14, r14
1407f6c8e: 7428 je 0x1407f6cb8
1407f6c90: 4489f0 mov eax, r14d
1407f6c93: c1e004 shl eax, 4
1407f6c96: 488d1c40 lea rbx, [rax + rax*2]
1407f6c9a: 0fb60568dfe302 movzx eax, byte ptr [rip + 0x2e3df68]
1407f6ca1: ba10000000 mov edx, 0x10
1407f6ca6: 4889d9 mov rcx, rbx
1407f6ca9: e862cfe1ff call 0x140613c10 ; unknown
1407f6cae: 4885c0 test rax, rax
1407f6cb1: 750a jne 0x1407f6cbd
1407f6cb3: e9e0180000 jmp 0x1407f8598 ; unknown
1407f6cb8: b810000000 mov eax, 0x10
1407f6cbd: 4c89b540010000 mov qword ptr [rbp + 0x140], r14
1407f6cc4: 48898548010000 mov qword ptr [rbp + 0x148], rax
1407f6ccb: 48c7855001000000000000 mov qword ptr [rbp + 0x150], 0
1407f6cd6: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f6cdd: 488b95b8010000 mov rdx, qword ptr [rbp + 0x1b8]
1407f6ce4: e8a73a3300 call 0x140b2a790 ; country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
1407f6ce9: 4c8bb588000000 mov r14, qword ptr [rbp + 0x88]
1407f6cf0: 498d143f lea rdx, [r15 + rdi]
1407f6cf4: 4883c2b8 add rdx, -0x48
1407f6cf8: 83bdc001000001 cmp dword ptr [rbp + 0x1c0], 1
1407f6cff: 488995d0000000 mov qword ptr [rbp + 0xd0], rdx
1407f6d06: 0f85bb080000 jne 0x1407f75c7
1407f6d0c: 488d8dc4010000 lea rcx, [rbp + 0x1c4]
1407f6d13: 488b4110 mov rax, qword ptr [rcx + 0x10]
1407f6d17: 488985a0010000 mov qword ptr [rbp + 0x1a0], rax
1407f6d1e: 0f1001 movups xmm0, xmmword ptr [rcx]
1407f6d21: 0f298590010000 movaps xmmword ptr [rbp + 0x190], xmm0
1407f6d28: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f6d2f: 8078da02 cmp byte ptr [rax - 0x26], 2
1407f6d33: 0f8331180000 jae 0x1407f856a
1407f6d39: 0fb648d8 movzx ecx, byte ptr [rax - 0x28]
1407f6d3d: e8aee93200 call 0x140b256f0 ; country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
1407f6d42: a801 test al, 1
1407f6d44: 0f84f5180000 je 0x1407f863f
1407f6d4a: 4889d3 mov rbx, rdx
1407f6d4d: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f6d54: e8e7334a00 call 0x140c9a140 ; utils::geometry::rectangle::Rectangle2d::get_corner_normal
1407f6d59: 0f28f8 movaps xmm7, xmm0
1407f6d5c: 660f6ff1 movdqa xmm6, xmm1
1407f6d60: 488d8dec000000 lea rcx, [rbp + 0xec]
1407f6d67: 488d9590010000 lea rdx, [rbp + 0x190]
1407f6d6e: 4989d8 mov r8, rbx
1407f6d71: e89a344a00 call 0x140c9a210 ; utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
1407f6d76: f2440f10bdec000000 movsd xmm15, qword ptr [rbp + 0xec]
1407f6d7f: f2440f109df4000000 movsd xmm11, qword ptr [rbp + 0xf4]
1407f6d88: f30f11bd80000000 movss dword ptr [rbp + 0x80], xmm7
1407f6d90: f30f11b584000000 movss dword ptr [rbp + 0x84], xmm6
1407f6d98: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f6d9f: 448a78fc mov r15b, byte ptr [rax - 4]
1407f6da3: 488b9520010000 mov rdx, qword ptr [rbp + 0x120]
1407f6daa: b103 mov cl, 3
1407f6dac: e89fc40300 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407f6db1: 488b9d28010000 mov rbx, qword ptr [rbp + 0x128]
1407f6db8: 4489f9 mov ecx, r15d
1407f6dbb: 89c2 mov edx, eax
1407f6dbd: e8ceee3200 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407f6dc2: 888590010000 mov byte ptr [rbp + 0x190], al
1407f6dc8: 889591010000 mov byte ptr [rbp + 0x191], dl
1407f6dce: 4889d9 mov rcx, rbx
1407f6dd1: 488d9590010000 lea rdx, [rbp + 0x190]
1407f6dd8: e8b361a600 call 0x14125cf90 ; std::collections::hash::map::HashMap::get<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,en
1407f6ddd: 4885c0 test rax, rax
1407f6de0: 0f84cb170000 je 0x1407f85b1
1407f6de6: f30f104030 movss xmm0, dword ptr [rax + 0x30]
1407f6deb: f30f5e0501791302 divss xmm0, dword ptr [rip + 0x2137901]
1407f6df3: 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
1407f6df7: 660f6e75f4 movd xmm6, dword ptr [rbp - 0xc]
1407f6dfc: f3440f12e0 movsldup xmm12, xmm0
1407f6e01: 450f28c7 movaps xmm8, xmm15
1407f6e05: 450f59c4 mulps xmm8, xmm12
1407f6e09: f3440f106df8 movss xmm13, dword ptr [rbp - 8]
1407f6e0f: 440f16e9 movlhps xmm13, xmm1
1407f6e13: 440fc6e9e2 shufps xmm13, xmm1, 0xe2
1407f6e18: 450f58c5 addps xmm8, xmm13
1407f6e1c: 450f59e3 mulps xmm12, xmm11
1407f6e20: 450f58e5 addps xmm12, xmm13
1407f6e24: 410f28c0 movaps xmm0, xmm8
1407f6e28: 0f2815b1ea1202 movaps xmm2, xmmword ptr [rip + 0x212eab1]
1407f6e2f: 0f59c2 mulps xmm0, xmm2
1407f6e32: 410f28cc movaps xmm1, xmm12
1407f6e36: 0f59ca mulps xmm1, xmm2
1407f6e39: 0f58c8 addps xmm1, xmm0
1407f6e3c: 0f138d78010000 movlps qword ptr [rbp + 0x178], xmm1
1407f6e43: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f6e4a: 488d9578010000 lea rdx, [rbp + 0x178]
1407f6e51: 660f6fd6 movdqa xmm2, xmm6
1407f6e55: e8865f4a00 call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
1407f6e5a: 488d8de0000000 lea rcx, [rbp + 0xe0]
1407f6e61: 488d9580000000 lea rdx, [rbp + 0x80]
1407f6e68: e8535f4a00 call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
1407f6e6d: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f6e74: 488d95e0000000 lea rdx, [rbp + 0xe0]
1407f6e7b: e850a34900 call 0x140c911d0 ; utils::xz_projective_rotation_bad
1407f6e80: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f6e87: 0fb640fc movzx eax, byte ptr [rax - 4]
1407f6e8b: 48ffc8 dec rax
1407f6e8e: 4883f803 cmp rax, 3
1407f6e92: 0f8344160000 jae 0x1407f84dc
1407f6e98: 440f2895c0010000 movaps xmm10, xmmword ptr [rbp + 0x1c0]
1407f6ea0: f20f10bd90010000 movsd xmm7, qword ptr [rbp + 0x190]
1407f6ea8: f3440f108d98010000 movss xmm9, dword ptr [rbp + 0x198]
1407f6eb1: 440f592d874d2902 mulps xmm13, xmmword ptr [rip + 0x2294d87]
1407f6eb9: 440f59058f4d2902 mulps xmm8, xmmword ptr [rip + 0x2294d8f]
1407f6ec1: 450f58c5 addps xmm8, xmm13
1407f6ec5: 440f138510010000 movlps qword ptr [rbp + 0x110], xmm8
1407f6ecd: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f6ed4: 488d9510010000 lea rdx, [rbp + 0x110]
1407f6edb: 660f6fd6 movdqa xmm2, xmm6
1407f6edf: e8fc5e4a00 call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
1407f6ee4: f3410f16c7 movshdup xmm0, xmm15
1407f6ee9: 0f570500ea1202 xorps xmm0, xmmword ptr [rip + 0x212ea00]
1407f6ef0: f30f118578010000 movss dword ptr [rbp + 0x178], xmm0
1407f6ef8: f3440f11bd7c010000 movss dword ptr [rbp + 0x17c], xmm15
1407f6f01: 488d8de0000000 lea rcx, [rbp + 0xe0]
1407f6f08: 488d9578010000 lea rdx, [rbp + 0x178]
1407f6f0f: e8ac5e4a00 call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
1407f6f14: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f6f1b: 488d95e0000000 lea rdx, [rbp + 0xe0]
1407f6f22: e8a9a24900 call 0x140c911d0 ; utils::xz_projective_rotation_bad
1407f6f27: 440f2885c0010000 movaps xmm8, xmmword ptr [rbp + 0x1c0]
1407f6f2f: 488b8590010000 mov rax, qword ptr [rbp + 0x190]
1407f6f36: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407f6f3d: 8b8598010000 mov eax, dword ptr [rbp + 0x198]
1407f6f43: 8985c8010000 mov dword ptr [rbp + 0x1c8], eax
1407f6f49: 488bbd50010000 mov rdi, qword ptr [rbp + 0x150]
1407f6f50: 483bbd40010000 cmp rdi, qword ptr [rbp + 0x140]
1407f6f57: 7513 jne 0x1407f6f6c
1407f6f59: 488d8d40010000 lea rcx, [rbp + 0x140]
1407f6f60: 488d1571512902 lea rdx, [rip + 0x2295171]
1407f6f67: e804450300 call 0x14082b470 ; alloc::raw_vec::RawVec::grow_one<utils::components::transform::Transform,alloc::alloc::Global>
1407f6f6c: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
1407f6f73: 488d0c7f lea rcx, [rdi + rdi*2]
1407f6f77: 48c1e104 shl rcx, 4
1407f6f7b: 440f290408 movaps xmmword ptr [rax + rcx], xmm8
1407f6f80: 488b95c0010000 mov rdx, qword ptr [rbp + 0x1c0]
1407f6f87: 4889540810 mov qword ptr [rax + rcx + 0x10], rdx
1407f6f8c: 8b95c8010000 mov edx, dword ptr [rbp + 0x1c8]
1407f6f92: 89540818 mov dword ptr [rax + rcx + 0x18], edx
1407f6f96: 48ba0000803f0000803f movabs rdx, 0x3f8000003f800000
1407f6fa0: 488954081c mov qword ptr [rax + rcx + 0x1c], rdx
1407f6fa5: c74408240000803f mov dword ptr [rax + rcx + 0x24], 0x3f800000
1407f6fad: 48ffc7 inc rdi
1407f6fb0: 4889bd50010000 mov qword ptr [rbp + 0x150], rdi
1407f6fb7: 440f5925914c2902 mulps xmm12, xmmword ptr [rip + 0x2294c91]
1407f6fbf: 450f58ec addps xmm13, xmm12
1407f6fc3: 440f13ad10010000 movlps qword ptr [rbp + 0x110], xmm13
1407f6fcb: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f6fd2: 488d9510010000 lea rdx, [rbp + 0x110]
1407f6fd9: 660f6fd6 movdqa xmm2, xmm6
1407f6fdd: e8fe5d4a00 call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
1407f6fe2: f3410f16c3 movshdup xmm0, xmm11
1407f6fe7: 0f570502e91202 xorps xmm0, xmmword ptr [rip + 0x212e902]
1407f6fee: f30f118578010000 movss dword ptr [rbp + 0x178], xmm0
1407f6ff6: f3440f119d7c010000 movss dword ptr [rbp + 0x17c], xmm11
1407f6fff: 488d8de0000000 lea rcx, [rbp + 0xe0]
1407f7006: 488d9578010000 lea rdx, [rbp + 0x178]
1407f700d: e8ae5d4a00 call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
1407f7012: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f7019: 488d95e0000000 lea rdx, [rbp + 0xe0]
1407f7020: e8aba14900 call 0x140c911d0 ; utils::xz_projective_rotation_bad
1407f7025: 0f28b5c0010000 movaps xmm6, xmmword ptr [rbp + 0x1c0]
1407f702c: 488b8590010000 mov rax, qword ptr [rbp + 0x190]
1407f7033: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407f703a: 8b8598010000 mov eax, dword ptr [rbp + 0x198]
1407f7040: 8985c8010000 mov dword ptr [rbp + 0x1c8], eax
1407f7046: 488bbd50010000 mov rdi, qword ptr [rbp + 0x150]
1407f704d: 483bbd40010000 cmp rdi, qword ptr [rbp + 0x140]
1407f7054: 7513 jne 0x1407f7069
1407f7056: 488d8d40010000 lea rcx, [rbp + 0x140]
1407f705d: 488d158c502902 lea rdx, [rip + 0x229508c]
1407f7064: e807440300 call 0x14082b470 ; alloc::raw_vec::RawVec::grow_one<utils::components::transform::Transform,alloc::alloc::Global>
1407f7069: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
1407f7070: 488d0c7f lea rcx, [rdi + rdi*2]
1407f7074: 48c1e104 shl rcx, 4
1407f7078: 0f293408 movaps xmmword ptr [rax + rcx], xmm6
1407f707c: 488b95c0010000 mov rdx, qword ptr [rbp + 0x1c0]
1407f7083: 4889540810 mov qword ptr [rax + rcx + 0x10], rdx
1407f7088: 8b95c8010000 mov edx, dword ptr [rbp + 0x1c8]
1407f708e: 89540818 mov dword ptr [rax + rcx + 0x18], edx
1407f7092: 48ba0000803f0000803f movabs rdx, 0x3f8000003f800000
1407f709c: 488954081c mov qword ptr [rax + rcx + 0x1c], rdx
1407f70a1: c74408240000803f mov dword ptr [rax + rcx + 0x24], 0x3f800000
1407f70a9: 48ffc7 inc rdi
1407f70ac: 4889bd50010000 mov qword ptr [rbp + 0x150], rdi
1407f70b3: 8b85a4000000 mov eax, dword ptr [rbp + 0xa4]
1407f70b9: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
1407f70bd: 8901 mov dword ptr [rcx], eax
1407f70bf: 488b8dc0000000 mov rcx, qword ptr [rbp + 0xc0]
1407f70c6: e855540c00 call 0x1408bc520 ; country_core::resources::rt::impl$5::from | country_core::resources::walls::decorator_storage::impl$15::from
1407f70cb: 4989d1 mov r9, rdx
1407f70ce: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
1407f70d5: 0fb649ff movzx ecx, byte ptr [rcx - 1]
1407f70d9: 884c2420 mov byte ptr [rsp + 0x20], cl
1407f70dd: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
1407f70e1: 488b9568010000 mov rdx, qword ptr [rbp + 0x168]
1407f70e8: 4989c0 mov r8, rax
1407f70eb: e8c0e70200 call 0x1408258b0 ; system_decorator::blueprint::DecoratorBlueprints::insert
1407f70f0: 48898500020000 mov qword ptr [rbp + 0x200], rax
1407f70f7: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f70fe: 0fb640fc movzx eax, byte ptr [rax - 4]
1407f7102: fec8 dec al
1407f7104: 3c03 cmp al, 3
1407f7106: 0f8316150000 jae 0x1407f8622
1407f710c: 0fb6c0 movzx eax, al
1407f710f: 488d0d32522902 lea rcx, [rip + 0x2295232]
1407f7116: 4c633c81 movsxd r15, dword ptr [rcx + rax*4]
1407f711a: 450f28c2 movaps xmm8, xmm10
1407f711e: 440f59054a4b2902 mulps xmm8, xmmword ptr [rip + 0x2294b4a]
1407f7126: 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
1407f712d: 488d82b0000000 lea rax, [rdx + 0xb0]
1407f7134: 48898598000000 mov qword ptr [rbp + 0x98], rax
1407f713b: 488d85c4010000 lea rax, [rbp + 0x1c4]
1407f7142: c7400700000000 mov dword ptr [rax + 7], 0
1407f7149: 48c740ff00000000 mov qword ptr [rax - 1], 0
1407f7151: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1407f7158: 488b8dc7010000 mov rcx, qword ptr [rbp + 0x1c7]
1407f715f: 48898d7f010000 mov qword ptr [rbp + 0x17f], rcx
1407f7166: 48898578010000 mov qword ptr [rbp + 0x178], rax
1407f716d: 488b9ac0000000 mov rbx, qword ptr [rdx + 0xc0]
1407f7174: 483b9ab0000000 cmp rbx, qword ptr [rdx + 0xb0]
1407f717b: 7513 jne 0x1407f7190
1407f717d: 488b8d98000000 mov rcx, qword ptr [rbp + 0x98]
1407f7184: 488d150d502902 lea rdx, [rip + 0x229500d]
1407f718b: e820420300 call 0x14082b3b0 ; alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
1407f7190: 488d05b1512902 lea rax, [rip + 0x22951b1]
1407f7197: 4901c7 add r15, rax
1407f719a: 410f28f2 movaps xmm6, xmm10
1407f719e: 410f59f2 mulps xmm6, xmm10
1407f71a2: f30f16c6 movshdup xmm0, xmm6
1407f71a6: f30f58c6 addss xmm0, xmm6
1407f71aa: 0f28ce movaps xmm1, xmm6
1407f71ad: 660f15ce unpckhpd xmm1, xmm6
1407f71b1: f30f58c8 addss xmm1, xmm0
1407f71b5: 0fc6c900 shufps xmm1, xmm1, 0
1407f71b9: 0fc6f6ff shufps xmm6, xmm6, 0xff
1407f71bd: 0f5cf1 subps xmm6, xmm1
1407f71c0: 0f28c6 movaps xmm0, xmm6
1407f71c3: 0f281d964a2902 movaps xmm3, xmmword ptr [rip + 0x2294a96]
1407f71ca: 0f59c3 mulps xmm0, xmm3
1407f71cd: 410f28ca movaps xmm1, xmm10
1407f71d1: 0f59cb mulps xmm1, xmm3
1407f71d4: f30f16d1 movshdup xmm2, xmm1
1407f71d8: f30f58d1 addss xmm2, xmm1
1407f71dc: 0f12c9 movhlps xmm1, xmm1
1407f71df: f30f58ca addss xmm1, xmm2
1407f71e3: f30f58c9 addss xmm1, xmm1
1407f71e7: 0fc6c900 shufps xmm1, xmm1, 0
1407f71eb: 410f59ca mulps xmm1, xmm10
1407f71ef: 0f58c8 addps xmm1, xmm0
1407f71f2: 450f28da movaps xmm11, xmm10
1407f71f6: 450fc6dad2 shufps xmm11, xmm10, 0xd2
1407f71fb: 410f28c3 movaps xmm0, xmm11
1407f71ff: 0f59c3 mulps xmm0, xmm3
1407f7202: 410f58c0 addps xmm0, xmm8
1407f7206: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407f720a: 450f28fa movaps xmm15, xmm10
1407f720e: 450f58fa addps xmm15, xmm10
1407f7212: 450fc6ffff shufps xmm15, xmm15, 0xff
1407f7217: 410f59c7 mulps xmm0, xmm15
1407f721b: 0f58c1 addps xmm0, xmm1
1407f721e: 0f58f8 addps xmm7, xmm0
1407f7221: 0f12c0 movhlps xmm0, xmm0
1407f7224: f3440f58c8 addss xmm9, xmm0
1407f7229: 4c8b9500020000 mov r10, qword ptr [rbp + 0x200]
1407f7230: 498b82b8000000 mov rax, qword ptr [r10 + 0xb8]
1407f7237: 4869cb60010000 imul rcx, rbx, 0x160
1407f723e: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407f7246: 48c744081000000000 mov qword ptr [rax + rcx + 0x10], 0
1407f724f: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407f7258: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407f7260: 488b555c mov rdx, qword ptr [rbp + 0x5c]
1407f7264: 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
1407f7269: 8b5564 mov edx, dword ptr [rbp + 0x64]
1407f726c: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407f7270: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407f7278: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
1407f727f: 0f108dd0010000 movups xmm1, xmmword ptr [rbp + 0x1d0]
1407f7286: f30f6f95e0010000 movdqu xmm2, xmmword ptr [rbp + 0x1e0]
1407f728e: 0f109dec010000 movups xmm3, xmmword ptr [rbp + 0x1ec]
1407f7295: 0f11440844 movups xmmword ptr [rax + rcx + 0x44], xmm0
1407f729a: 0f114c0854 movups xmmword ptr [rax + rcx + 0x54], xmm1
1407f729f: f30f7f540864 movdqu xmmword ptr [rax + rcx + 0x64], xmm2
1407f72a5: 0f115c0870 movups xmmword ptr [rax + rcx + 0x70], xmm3
1407f72aa: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407f72b5: 488b5568 mov rdx, qword ptr [rbp + 0x68]
1407f72b9: 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
1407f72c1: 8b5570 mov edx, dword ptr [rbp + 0x70]
1407f72c4: 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
1407f72cb: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407f72d6: 488b5574 mov rdx, qword ptr [rbp + 0x74]
1407f72da: 4889940894000000 mov qword ptr [rax + rcx + 0x94], rdx
1407f72e2: 8b557c mov edx, dword ptr [rbp + 0x7c]
1407f72e5: 8994089c000000 mov dword ptr [rax + rcx + 0x9c], edx
1407f72ec: 4c89bc08a0000000 mov qword ptr [rax + rcx + 0xa0], r15
1407f72f4: 48c78408a800000024000000 mov qword ptr [rax + rcx + 0xa8], 0x24
1407f7300: 488d156d4e2902 lea rdx, [rip + 0x2294e6d]
1407f7307: 48899408b0000000 mov qword ptr [rax + rcx + 0xb0], rdx
1407f730f: 48c78408b80000001e000000 mov qword ptr [rax + rcx + 0xb8], 0x1e
1407f731b: 440f299408c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm10
1407f7324: 0f13bc08d0000000 movlps qword ptr [rax + rcx + 0xd0], xmm7
1407f732c: f3440f118c08d8000000 movss dword ptr [rax + rcx + 0xd8], xmm9
1407f7336: 49b90000803f0000803f movabs r9, 0x3f8000003f800000
1407f7340: 4c898c08dc000000 mov qword ptr [rax + rcx + 0xdc], r9
1407f7348: c78408e40000000000803f mov dword ptr [rax + rcx + 0xe4], 0x3f800000
1407f7353: c78408f000000001000000 mov dword ptr [rax + rcx + 0xf0], 1
1407f735e: 8b95c8000000 mov edx, dword ptr [rbp + 0xc8]
1407f7364: 899408f4000000 mov dword ptr [rax + rcx + 0xf4], edx
1407f736b: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407f7376: 0f108590010000 movups xmm0, xmmword ptr [rbp + 0x190]
1407f737d: 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
1407f7385: 488b95a0010000 mov rdx, qword ptr [rbp + 0x1a0]
1407f738c: 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
1407f7394: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407f739c: 0fb695b7010000 movzx edx, byte ptr [rbp + 0x1b7]
1407f73a3: 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
1407f73aa: 0fb795b5010000 movzx edx, word ptr [rbp + 0x1b5]
1407f73b1: 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
1407f73b9: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407f73c4: f30f6f85e0000000 movdqu xmm0, xmmword ptr [rbp + 0xe0]
1407f73cc: f30f6f8df0000000 movdqu xmm1, xmmword ptr [rbp + 0xf0]
1407f73d4: f30f7f8c082c010000 movdqu xmmword ptr [rax + rcx + 0x12c], xmm1
1407f73dd: f30f7f84081c010000 movdqu xmmword ptr [rax + rcx + 0x11c], xmm0
1407f73e6: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407f73ee: 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
1407f73f5: 4c8b857f010000 mov r8, qword ptr [rbp + 0x17f]
1407f73fc: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407f7404: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407f740c: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407f7414: 0fb69538010000 movzx edx, byte ptr [rbp + 0x138]
1407f741b: 8894084e010000 mov byte ptr [rax + rcx + 0x14e], dl
1407f7422: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407f742c: 488b9510010000 mov rdx, qword ptr [rbp + 0x110]
1407f7433: 4c8b8517010000 mov r8, qword ptr [rbp + 0x117]
1407f743a: 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
1407f7442: 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
1407f744a: 48ffc3 inc rbx
1407f744d: 49899ac0000000 mov qword ptr [r10 + 0xc0], rbx
1407f7454: 498b02 mov rax, qword ptr [r10]
1407f7457: 490b4208 or rax, qword ptr [r10 + 8]
1407f745b: 0f85a4100000 jne 0x1407f8505
1407f7461: 49c7420800000000 mov qword ptr [r10 + 8], 0
1407f7469: 49c70201000000 mov qword ptr [r10], 1
1407f7470: 450f295210 movaps xmmword ptr [r10 + 0x10], xmm10
1407f7475: 0f29bdb0000000 movaps xmmword ptr [rbp + 0xb0], xmm7
1407f747c: 410f137a20 movlps qword ptr [r10 + 0x20], xmm7
1407f7481: f3450f114a28 movss dword ptr [r10 + 0x28], xmm9
1407f7487: 4d894a2c mov qword ptr [r10 + 0x2c], r9
1407f748b: 41c742340000803f mov dword ptr [r10 + 0x34], 0x3f800000
1407f7493: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f749a: 8a58fc mov bl, byte ptr [rax - 4]
1407f749d: 488b9520010000 mov rdx, qword ptr [rbp + 0x120]
1407f74a4: b103 mov cl, 3
1407f74a6: e8a5bd0300 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407f74ab: 488bbd28010000 mov rdi, qword ptr [rbp + 0x128]
1407f74b2: 89d9 mov ecx, ebx
1407f74b4: 89c2 mov edx, eax
1407f74b6: e8d5e73200 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407f74bb: 888590010000 mov byte ptr [rbp + 0x190], al
1407f74c1: 889591010000 mov byte ptr [rbp + 0x191], dl
1407f74c7: 48837f1800 cmp qword ptr [rdi + 0x18], 0
1407f74cc: 0f84e30e0000 je 0x1407f83b5
1407f74d2: 488d4f20 lea rcx, [rdi + 0x20]
1407f74d6: 488d9590010000 lea rdx, [rbp + 0x190]
1407f74dd: e8de08a000 call 0x1411f7dc0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey> > > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$
1407f74e2: 488b0f mov rcx, qword ptr [rdi]
1407f74e5: 488b5708 mov rdx, qword ptr [rdi + 8]
1407f74e9: 4989d0 mov r8, rdx
1407f74ec: 4921c0 and r8, rax
1407f74ef: 48c1e839 shr rax, 0x39
1407f74f3: 660f6ec0 movd xmm0, eax
1407f74f7: 660f60c0 punpcklbw xmm0, xmm0
1407f74fb: f20f70c000 pshuflw xmm0, xmm0, 0
1407f7500: 660f70c000 pshufd xmm0, xmm0, 0
1407f7505: 488d41b8 lea rax, [rcx - 0x48]
1407f7509: 440fb68d90010000 movzx r9d, byte ptr [rbp + 0x190]
1407f7511: 440fb69591010000 movzx r10d, byte ptr [rbp + 0x191]
1407f7519: 4531db xor r11d, r11d
1407f751c: f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
1407f7522: 660f6fd1 movdqa xmm2, xmm1
1407f7526: 660f74d0 pcmpeqb xmm2, xmm0
1407f752a: 660fd7fa pmovmskb edi, xmm2
1407f752e: 85ff test edi, edi
1407f7530: 0f28bdb0000000 movaps xmm7, xmmword ptr [rbp + 0xb0]
1407f7537: 7467 je 0x1407f75a0
1407f7539: 4180f91a cmp r9b, 0x1a
1407f753d: 7528 jne 0x1407f7567
1407f753f: f30fbcdf tzcnt ebx, edi
1407f7543: 4c01c3 add rbx, r8
1407f7546: 4821d3 and rbx, rdx
1407f7549: 48f7db neg rbx
1407f754c: 4c8d34db lea r14, [rbx + rbx*8]
1407f7550: 42803cf01a cmp byte ptr [rax + r14*8], 0x1a
1407f7555: 0f84cb020000 je 0x1407f7826
1407f755b: 8d5fff lea ebx, [rdi - 1]
1407f755e: 6621fb and bx, di
1407f7561: 89df mov edi, ebx
1407f7563: 75da jne 0x1407f753f
1407f7565: eb39 jmp 0x1407f75a0 ; unknown
1407f7567: f30fbcdf tzcnt ebx, edi
1407f756b: 4c01c3 add rbx, r8
1407f756e: 4821d3 and rbx, rdx
1407f7571: 48f7db neg rbx
1407f7574: 4c8d34db lea r14, [rbx + rbx*8]
1407f7578: 460fb63cf0 movzx r15d, byte ptr [rax + r14*8]
1407f757d: 4180ff1a cmp r15b, 0x1a
1407f7581: 7413 je 0x1407f7596
1407f7583: 4538f9 cmp r9b, r15b
1407f7586: 750e jne 0x1407f7596
1407f7588: 4e8d34f0 lea r14, [rax + r14*8]
1407f758c: 453a5601 cmp r10b, byte ptr [r14 + 1]
1407f7590: 0f8490020000 je 0x1407f7826
1407f7596: 8d5fff lea ebx, [rdi - 1]
1407f7599: 6621fb and bx, di
1407f759c: 89df mov edi, ebx
1407f759e: 75c7 jne 0x1407f7567
1407f75a0: 660f740de8e31202 pcmpeqb xmm1, xmmword ptr [rip + 0x212e3e8]
1407f75a8: 660fd7f9 pmovmskb edi, xmm1
1407f75ac: 85ff test edi, edi
1407f75ae: 0f85010e0000 jne 0x1407f83b5
1407f75b4: 4d01d8 add r8, r11
1407f75b7: 4983c010 add r8, 0x10
1407f75bb: 4983c310 add r11, 0x10
1407f75bf: 4921d0 and r8, rdx
1407f75c2: e955ffffff jmp 0x1407f751c ; unknown
1407f75c7: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f75ce: 8078da02 cmp byte ptr [rax - 0x26], 2
1407f75d2: 0f83750f0000 jae 0x1407f854d
1407f75d8: f20f1032 movsd xmm6, qword ptr [rdx]
1407f75dc: f20f11b510010000 movsd qword ptr [rbp + 0x110], xmm6
1407f75e4: f20f11b578010000 movsd qword ptr [rbp + 0x178], xmm6
1407f75ec: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
1407f75f3: e848313300 call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
1407f75f8: 8985c0010000 mov dword ptr [rbp + 0x1c0], eax
1407f75fe: 660f7e85c4010000 movd dword ptr [rbp + 0x1c4], xmm0
1407f7606: f30f108d78010000 movss xmm1, dword ptr [rbp + 0x178]
1407f760e: f30f580de2401f02 addss xmm1, dword ptr [rip + 0x21f40e2]
1407f7616: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f761d: e88ea84900 call 0x140c91eb0 ; utils::wall_space::impl$1::project_clamp
1407f7622: 660f7e8578010000 movd dword ptr [rbp + 0x178], xmm0
1407f762a: 0f13b590010000 movlps qword ptr [rbp + 0x190], xmm6
1407f7631: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
1407f7638: e803313300 call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
1407f763d: 8985c0010000 mov dword ptr [rbp + 0x1c0], eax
1407f7643: 660f7e85c4010000 movd dword ptr [rbp + 0x1c4], xmm0
1407f764b: f30f108d90010000 movss xmm1, dword ptr [rbp + 0x190]
1407f7653: f30f580d29e31202 addss xmm1, dword ptr [rip + 0x212e329]
1407f765b: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f7662: e849a84900 call 0x140c91eb0 ; utils::wall_space::impl$1::project_clamp
1407f7667: 660f7e8590010000 movd dword ptr [rbp + 0x190], xmm0
1407f766f: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f7676: 488d9510010000 lea rdx, [rbp + 0x110]
1407f767d: 4c8b85b8010000 mov r8, qword ptr [rbp + 0x1b8]
1407f7684: e807aa4900 call 0x140c92090 ; utils::wall_space::WallSpaceCoord::to_world_space
1407f7689: f30f10b5c0010000 movss xmm6, dword ptr [rbp + 0x1c0]
1407f7691: f30f10bdc8010000 movss xmm7, dword ptr [rbp + 0x1c8]
1407f7699: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f76a0: 488d9578010000 lea rdx, [rbp + 0x178]
1407f76a7: 4c8b85b8010000 mov r8, qword ptr [rbp + 0x1b8]
1407f76ae: e8dda94900 call 0x140c92090 ; utils::wall_space::WallSpaceCoord::to_world_space
1407f76b3: f3440f10bdc0010000 movss xmm15, dword ptr [rbp + 0x1c0]
1407f76bc: f3440f1085c8010000 movss xmm8, dword ptr [rbp + 0x1c8]
1407f76c5: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f76cc: 488d9590010000 lea rdx, [rbp + 0x190]
1407f76d3: 4c8b85b8010000 mov r8, qword ptr [rbp + 0x1b8]
1407f76da: e8b1a94900 call 0x140c92090 ; utils::wall_space::WallSpaceCoord::to_world_space
1407f76df: 450f14f8 unpcklps xmm15, xmm8
1407f76e3: 0f14f7 unpcklps xmm6, xmm7
1407f76e6: 440f5cfe subps xmm15, xmm6
1407f76ea: 410f28c7 movaps xmm0, xmm15
1407f76ee: 410f59c7 mulps xmm0, xmm15
1407f76f2: f30f16c8 movshdup xmm1, xmm0
1407f76f6: f30f58c8 addss xmm1, xmm0
1407f76fa: f30f51c9 sqrtss xmm1, xmm1
1407f76fe: f30f101dfee11202 movss xmm3, dword ptr [rip + 0x212e1fe]
1407f7706: 0f28c3 movaps xmm0, xmm3
1407f7709: f30f5ec1 divss xmm0, xmm1
1407f770d: 660f7ec0 movd eax, xmm0
1407f7711: 85c0 test eax, eax
1407f7713: 0f99c1 setns cl
1407f7716: 89c2 mov edx, eax
1407f7718: 81e2ffffff7f and edx, 0x7fffffff
1407f771e: 81c2000080ff add edx, 0xff800000
1407f7724: 81fa0000007f cmp edx, 0x7f000000
1407f772a: 0f92c2 setb dl
1407f772d: 20ca and dl, cl
1407f772f: ffc8 dec eax
1407f7731: 3dffff7f00 cmp eax, 0x7fffff
1407f7736: 0f92c0 setb al
1407f7739: 08d0 or al, dl
1407f773b: 0f845d0b0000 je 0x1407f829e
1407f7741: f3440f109dc0010000 movss xmm11, dword ptr [rbp + 0x1c0]
1407f774a: f30f108dc8010000 movss xmm1, dword ptr [rbp + 0x1c8]
1407f7752: 440f14d9 unpcklps xmm11, xmm1
1407f7756: 440f5cde subps xmm11, xmm6
1407f775a: 410f28cb movaps xmm1, xmm11
1407f775e: 410f59cb mulps xmm1, xmm11
1407f7762: f30f16d1 movshdup xmm2, xmm1
1407f7766: f30f58d1 addss xmm2, xmm1
1407f776a: f30f51d2 sqrtss xmm2, xmm2
1407f776e: 0f28cb movaps xmm1, xmm3
1407f7771: f30f5eca divss xmm1, xmm2
1407f7775: 660f7ec8 movd eax, xmm1
1407f7779: 85c0 test eax, eax
1407f777b: 0f98c1 sets cl
1407f777e: 89c2 mov edx, eax
1407f7780: 81e2ffffff7f and edx, 0x7fffffff
1407f7786: 81c2000080ff add edx, 0xff800000
1407f778c: 81fa0000007f cmp edx, 0x7f000000
1407f7792: 0f93c2 setae dl
1407f7795: 08ca or dl, cl
1407f7797: ffc8 dec eax
1407f7799: 3dffff7f00 cmp eax, 0x7fffff
1407f779e: 0f93c0 setae al
1407f77a1: 84d0 test al, dl
1407f77a3: 0f85f50a0000 jne 0x1407f829e
1407f77a9: f30f12c0 movsldup xmm0, xmm0
1407f77ad: 440f59f8 mulps xmm15, xmm0
1407f77b1: f30f12c1 movsldup xmm0, xmm1
1407f77b5: 440f59d8 mulps xmm11, xmm0
1407f77b9: 410f28f3 movaps xmm6, xmm11
1407f77bd: 410f5cf7 subps xmm6, xmm15
1407f77c1: 0f28c6 movaps xmm0, xmm6
1407f77c4: 0f59c6 mulps xmm0, xmm6
1407f77c7: f30f16c8 movshdup xmm1, xmm0
1407f77cb: f30f58c8 addss xmm1, xmm0
1407f77cf: f30f51c9 sqrtss xmm1, xmm1
1407f77d3: 0f28c3 movaps xmm0, xmm3
1407f77d6: f30f5ec1 divss xmm0, xmm1
1407f77da: 660f7ec0 movd eax, xmm0
1407f77de: 85c0 test eax, eax
1407f77e0: 0f98c1 sets cl
1407f77e3: 89c2 mov edx, eax
1407f77e5: 81e2ffffff7f and edx, 0x7fffffff
1407f77eb: 81c2000080ff add edx, 0xff800000
1407f77f1: 81fa0000007f cmp edx, 0x7f000000
1407f77f7: 0f93c2 setae dl
1407f77fa: 08ca or dl, cl
1407f77fc: ffc8 dec eax
1407f77fe: 3dffff7f00 cmp eax, 0x7fffff
1407f7803: 0f93c0 setae al
1407f7806: 84d0 test al, dl
1407f7808: 0f85900a0000 jne 0x1407f829e
1407f780e: f30f16fe movshdup xmm7, xmm6
1407f7812: 0f573dd7e01202 xorps xmm7, xmmword ptr [rip + 0x212e0d7]
1407f7819: f30f59f8 mulss xmm7, xmm0
1407f781d: f30f59f0 mulss xmm6, xmm0
1407f7821: e962f5ffff jmp 0x1407f6d88 ; unknown
1407f7826: 488d04db lea rax, [rbx + rbx*8]
1407f782a: 488b44c1d8 mov rax, qword ptr [rcx + rax*8 - 0x28]
1407f782f: f048ff00 lock inc qword ptr [rax]
1407f7833: 0f8ec80e0000 jle 0x1407f8701
1407f7839: 488985d8000000 mov qword ptr [rbp + 0xd8], rax
1407f7840: 48898590010000 mov qword ptr [rbp + 0x190], rax
1407f7847: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f784e: 8a58fc mov bl, byte ptr [rax - 4]
1407f7851: 488b9520010000 mov rdx, qword ptr [rbp + 0x120]
1407f7858: c6851702000001 mov byte ptr [rbp + 0x217], 1
1407f785f: b103 mov cl, 3
1407f7861: e8eab90300 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407f7866: 488bbd28010000 mov rdi, qword ptr [rbp + 0x128]
1407f786d: c6851702000001 mov byte ptr [rbp + 0x217], 1
1407f7874: 89d9 mov ecx, ebx
1407f7876: 89c2 mov edx, eax
1407f7878: e813e43200 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407f787d: 888578010000 mov byte ptr [rbp + 0x178], al
1407f7883: 889579010000 mov byte ptr [rbp + 0x179], dl
1407f7889: 48837f4800 cmp qword ptr [rdi + 0x48], 0
1407f788e: 0f84920b0000 je 0x1407f8426
1407f7894: 488d4f50 lea rcx, [rdi + 0x50]
1407f7898: c6851702000001 mov byte ptr [rbp + 0x217], 1
1407f789f: 488d9578010000 lea rdx, [rbp + 0x178]
1407f78a6: e81505a000 call 0x1411f7dc0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey> > > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$
1407f78ab: 488b4f30 mov rcx, qword ptr [rdi + 0x30]
1407f78af: 488b5738 mov rdx, qword ptr [rdi + 0x38]
1407f78b3: 4989d0 mov r8, rdx
1407f78b6: 4921c0 and r8, rax
1407f78b9: 48c1e839 shr rax, 0x39
1407f78bd: 660f6ec0 movd xmm0, eax
1407f78c1: 660f60c0 punpcklbw xmm0, xmm0
1407f78c5: f20f70c000 pshuflw xmm0, xmm0, 0
1407f78ca: 660f70c000 pshufd xmm0, xmm0, 0
1407f78cf: 488d41b8 lea rax, [rcx - 0x48]
1407f78d3: 440fb68d78010000 movzx r9d, byte ptr [rbp + 0x178]
1407f78db: 440fb69579010000 movzx r10d, byte ptr [rbp + 0x179]
1407f78e3: 4531db xor r11d, r11d
1407f78e6: f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
1407f78ec: 660f6fd1 movdqa xmm2, xmm1
1407f78f0: 660f74d0 pcmpeqb xmm2, xmm0
1407f78f4: 660fd7fa pmovmskb edi, xmm2
1407f78f8: 85ff test edi, edi
1407f78fa: 745f je 0x1407f795b
1407f78fc: 4180f91a cmp r9b, 0x1a
1407f7900: 7524 jne 0x1407f7926
1407f7902: f30fbcdf tzcnt ebx, edi
1407f7906: 4c01c3 add rbx, r8
1407f7909: 4821d3 and rbx, rdx
1407f790c: 48f7db neg rbx
1407f790f: 4c8d34db lea r14, [rbx + rbx*8]
1407f7913: 42803cf01a cmp byte ptr [rax + r14*8], 0x1a
1407f7918: 7468 je 0x1407f7982
1407f791a: 8d5fff lea ebx, [rdi - 1]
1407f791d: 6621fb and bx, di
1407f7920: 89df mov edi, ebx
1407f7922: 75de jne 0x1407f7902
1407f7924: eb35 jmp 0x1407f795b ; unknown
1407f7926: f30fbcdf tzcnt ebx, edi
1407f792a: 4c01c3 add rbx, r8
1407f792d: 4821d3 and rbx, rdx
1407f7930: 48f7db neg rbx
1407f7933: 4c8d34db lea r14, [rbx + rbx*8]
1407f7937: 460fb63cf0 movzx r15d, byte ptr [rax + r14*8]
1407f793c: 4180ff1a cmp r15b, 0x1a
1407f7940: 740f je 0x1407f7951
1407f7942: 4538f9 cmp r9b, r15b
1407f7945: 750a jne 0x1407f7951
1407f7947: 4e8d34f0 lea r14, [rax + r14*8]
1407f794b: 453a5601 cmp r10b, byte ptr [r14 + 1]
1407f794f: 7431 je 0x1407f7982
1407f7951: 8d5fff lea ebx, [rdi - 1]
1407f7954: 6621fb and bx, di
1407f7957: 89df mov edi, ebx
1407f7959: 75cb jne 0x1407f7926
1407f795b: 660f740d2de01202 pcmpeqb xmm1, xmmword ptr [rip + 0x212e02d]
1407f7963: 660fd7f9 pmovmskb edi, xmm1
1407f7967: 85ff test edi, edi
1407f7969: 0f85b70a0000 jne 0x1407f8426
1407f796f: 4d01d8 add r8, r11
1407f7972: 4983c010 add r8, 0x10
1407f7976: 4983c310 add r11, 0x10
1407f797a: 4921d0 and r8, rdx
1407f797d: e964ffffff jmp 0x1407f78e6 ; unknown
1407f7982: 488d04db lea rax, [rbx + rbx*8]
1407f7986: 4c8b44c1d8 mov r8, qword ptr [rcx + rax*8 - 0x28]
1407f798b: f049ff00 lock inc qword ptr [r8]
1407f798f: 0f8e6c0d0000 jle 0x1407f8701
1407f7995: 0f13bdc0010000 movlps qword ptr [rbp + 0x1c0], xmm7
1407f799c: f3440f118dc8010000 movss dword ptr [rbp + 0x1c8], xmm9
1407f79a5: 440f295510 movaps xmmword ptr [rbp + 0x10], xmm10
1407f79aa: c6851702000000 mov byte ptr [rbp + 0x217], 0
1407f79b1: 488d4510 lea rax, [rbp + 0x10]
1407f79b5: 4889442420 mov qword ptr [rsp + 0x20], rax
1407f79ba: 488b8d00020000 mov rcx, qword ptr [rbp + 0x200]
1407f79c1: 488b95d8000000 mov rdx, qword ptr [rbp + 0xd8]
1407f79c8: 4c8d8dc0010000 lea r9, [rbp + 0x1c0]
1407f79cf: e8dcd80200 call 0x1408252b0 ; system_decorator::blueprint::DecoratorBlueprint::set_colliders
1407f79d4: 4c8bb588000000 mov r14, qword ptr [rbp + 0x88]
1407f79db: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1407f79e2: 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
1407f79e9: 83b85802000002 cmp dword ptr [rax + 0x258], 2
1407f79f0: 0f83e9020000 jae 0x1407f7cdf
1407f79f6: 440fb67afc movzx r15d, byte ptr [rdx - 4]
1407f79fb: 418d47ff lea eax, [r15 - 1]
1407f79ff: 3c03 cmp al, 3
1407f7a01: 0f83b70c0000 jae 0x1407f86be
1407f7a07: 0fb6c0 movzx eax, al
1407f7a0a: 488d0d43492902 lea rcx, [rip + 0x2294943]
1407f7a11: 48631c81 movsxd rbx, dword ptr [rcx + rax*4]
1407f7a15: 488d85c4010000 lea rax, [rbp + 0x1c4]
1407f7a1c: c7400700000000 mov dword ptr [rax + 7], 0
1407f7a23: 48c740ff00000000 mov qword ptr [rax - 1], 0
1407f7a2b: 488d4ae8 lea rcx, [rdx - 0x18]
1407f7a2f: 488d15fa472902 lea rdx, [rip + 0x22947fa]
1407f7a36: e875190400 call 0x1408393b0 ; country_core::resources::walls::decorator::DecoratorExtraInfo::as_window | country_core::resources::walls::decorator::DecoratorExtraInfo::as_window_mut
1407f7a3b: 8b00 mov eax, dword ptr [rax]
1407f7a3d: 8985d8000000 mov dword ptr [rbp + 0xd8], eax
1407f7a43: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1407f7a4a: 488b8dc7010000 mov rcx, qword ptr [rbp + 0x1c7]
1407f7a51: 48898d7f010000 mov qword ptr [rbp + 0x17f], rcx
1407f7a58: 48898578010000 mov qword ptr [rbp + 0x178], rax
1407f7a5f: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7a66: 488bb8c0000000 mov rdi, qword ptr [rax + 0xc0]
1407f7a6d: 483bb8b0000000 cmp rdi, qword ptr [rax + 0xb0]
1407f7a74: 7513 jne 0x1407f7a89
1407f7a76: 488b8d98000000 mov rcx, qword ptr [rbp + 0x98]
1407f7a7d: 488d15c4472902 lea rdx, [rip + 0x22947c4]
1407f7a84: e827390300 call 0x14082b3b0 ; alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
1407f7a89: 488d05c4482902 lea rax, [rip + 0x22948c4]
1407f7a90: 4801c3 add rbx, rax
1407f7a93: 4969d7e8030000 imul rdx, r15, 0x3e8
1407f7a9a: 48039568010000 add rdx, qword ptr [rbp + 0x168]
1407f7aa1: 4c8b8d00020000 mov r9, qword ptr [rbp + 0x200]
1407f7aa8: 498b81b8000000 mov rax, qword ptr [r9 + 0xb8]
1407f7aaf: 4869cf60010000 imul rcx, rdi, 0x160
1407f7ab6: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407f7abe: 48c744081001000000 mov qword ptr [rax + rcx + 0x10], 1
1407f7ac7: 4889540818 mov qword ptr [rax + rcx + 0x18], rdx
1407f7acc: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407f7ad5: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407f7add: 488b555c mov rdx, qword ptr [rbp + 0x5c]
1407f7ae1: 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
1407f7ae6: 8b5564 mov edx, dword ptr [rbp + 0x64]
1407f7ae9: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407f7aed: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407f7af5: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
1407f7afc: 0f108dd0010000 movups xmm1, xmmword ptr [rbp + 0x1d0]
1407f7b03: f30f6f95e0010000 movdqu xmm2, xmmword ptr [rbp + 0x1e0]
1407f7b0b: 0f109dec010000 movups xmm3, xmmword ptr [rbp + 0x1ec]
1407f7b12: 0f11440844 movups xmmword ptr [rax + rcx + 0x44], xmm0
1407f7b17: 0f114c0854 movups xmmword ptr [rax + rcx + 0x54], xmm1
1407f7b1c: f30f7f540864 movdqu xmmword ptr [rax + rcx + 0x64], xmm2
1407f7b22: 0f115c0870 movups xmmword ptr [rax + rcx + 0x70], xmm3
1407f7b27: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407f7b32: 488b5568 mov rdx, qword ptr [rbp + 0x68]
1407f7b36: 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
1407f7b3e: 8b5570 mov edx, dword ptr [rbp + 0x70]
1407f7b41: 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
1407f7b48: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407f7b53: 488b5574 mov rdx, qword ptr [rbp + 0x74]
1407f7b57: 4889940894000000 mov qword ptr [rax + rcx + 0x94], rdx
1407f7b5f: 8b557c mov edx, dword ptr [rbp + 0x7c]
1407f7b62: 8994089c000000 mov dword ptr [rax + rcx + 0x9c], edx
1407f7b69: 48899c08a0000000 mov qword ptr [rax + rcx + 0xa0], rbx
1407f7b71: 48c78408a80000002a000000 mov qword ptr [rax + rcx + 0xa8], 0x2a
1407f7b7d: 488d15c8964202 lea rdx, [rip + 0x24296c8]
1407f7b84: 48899408b0000000 mov qword ptr [rax + rcx + 0xb0], rdx
1407f7b8c: 48c78408b800000024000000 mov qword ptr [rax + rcx + 0xb8], 0x24
1407f7b98: 440f299408c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm10
1407f7ba1: 0f2885b0000000 movaps xmm0, xmmword ptr [rbp + 0xb0]
1407f7ba8: 0f138408d0000000 movlps qword ptr [rax + rcx + 0xd0], xmm0
1407f7bb0: f3440f118c08d8000000 movss dword ptr [rax + rcx + 0xd8], xmm9
1407f7bba: 48ba0000803f0000803f movabs rdx, 0x3f8000003f800000
1407f7bc4: 48899408dc000000 mov qword ptr [rax + rcx + 0xdc], rdx
1407f7bcc: c78408e40000000000803f mov dword ptr [rax + rcx + 0xe4], 0x3f800000
1407f7bd7: c78408f000000001000000 mov dword ptr [rax + rcx + 0xf0], 1
1407f7be2: 8b95d8000000 mov edx, dword ptr [rbp + 0xd8]
1407f7be8: 899408f4000000 mov dword ptr [rax + rcx + 0xf4], edx
1407f7bef: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407f7bfa: 0f108590010000 movups xmm0, xmmword ptr [rbp + 0x190]
1407f7c01: 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
1407f7c09: 488b95a0010000 mov rdx, qword ptr [rbp + 0x1a0]
1407f7c10: 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
1407f7c18: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407f7c20: 0fb695b7010000 movzx edx, byte ptr [rbp + 0x1b7]
1407f7c27: 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
1407f7c2e: 0fb795b5010000 movzx edx, word ptr [rbp + 0x1b5]
1407f7c35: 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
1407f7c3d: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407f7c48: f30f6f85e0000000 movdqu xmm0, xmmword ptr [rbp + 0xe0]
1407f7c50: f30f6f8df0000000 movdqu xmm1, xmmword ptr [rbp + 0xf0]
1407f7c58: f30f7f8c082c010000 movdqu xmmword ptr [rax + rcx + 0x12c], xmm1
1407f7c61: f30f7f84081c010000 movdqu xmmword ptr [rax + rcx + 0x11c], xmm0
1407f7c6a: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407f7c72: 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
1407f7c79: 4c8b857f010000 mov r8, qword ptr [rbp + 0x17f]
1407f7c80: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407f7c88: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407f7c90: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407f7c98: 0fb69538010000 movzx edx, byte ptr [rbp + 0x138]
1407f7c9f: 8894084e010000 mov byte ptr [rax + rcx + 0x14e], dl
1407f7ca6: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407f7cb0: 488b9510010000 mov rdx, qword ptr [rbp + 0x110]
1407f7cb7: 4c8b8517010000 mov r8, qword ptr [rbp + 0x117]
1407f7cbe: 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
1407f7cc6: 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
1407f7cce: 48ffc7 inc rdi
1407f7cd1: 4989b9c0000000 mov qword ptr [r9 + 0xc0], rdi
1407f7cd8: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1407f7cdf: 448a7afc mov r15b, byte ptr [rdx - 4]
1407f7ce3: 488b9520010000 mov rdx, qword ptr [rbp + 0x120]
1407f7cea: b103 mov cl, 3
1407f7cec: e85fb50300 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407f7cf1: 488b9d28010000 mov rbx, qword ptr [rbp + 0x128]
1407f7cf8: 4489f9 mov ecx, r15d
1407f7cfb: 89c2 mov edx, eax
1407f7cfd: e88edf3200 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407f7d02: 888590010000 mov byte ptr [rbp + 0x190], al
1407f7d08: 889591010000 mov byte ptr [rbp + 0x191], dl
1407f7d0e: 4889d9 mov rcx, rbx
1407f7d11: 488d9590010000 lea rdx, [rbp + 0x190]
1407f7d18: e87352a600 call 0x14125cf90 ; std::collections::hash::map::HashMap::get<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,en
1407f7d1d: 4885c0 test rax, rax
1407f7d20: 0f842a090000 je 0x1407f8650
1407f7d26: f30f107830 movss xmm7, dword ptr [rax + 0x30]
1407f7d2b: f3440f10603c movss xmm12, dword ptr [rax + 0x3c]
1407f7d31: f3440f5c6038 subss xmm12, dword ptr [rax + 0x38]
1407f7d37: 440f28c7 movaps xmm8, xmm7
1407f7d3b: f3440f5e05b0691302 divss xmm8, dword ptr [rip + 0x21369b0]
1407f7d44: 488b8dd0000000 mov rcx, qword ptr [rbp + 0xd0]
1407f7d4b: 488d150e452902 lea rdx, [rip + 0x229450e]
1407f7d52: e809b50300 call 0x140833260 ; country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
1407f7d57: f2440f1030 movsd xmm14, qword ptr [rax]
1407f7d5c: 450f28e8 movaps xmm13, xmm8
1407f7d60: f3450f58e8 addss xmm13, xmm8
1407f7d65: f3440f582d123f2902 addss xmm13, dword ptr [rip + 0x2293f12]
1407f7d6e: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7d75: 488bb8d8000000 mov rdi, qword ptr [rax + 0xd8]
1407f7d7c: 483bb8c8000000 cmp rdi, qword ptr [rax + 0xc8]
1407f7d83: 751a jne 0x1407f7d9f
1407f7d85: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7d8c: 488d88c8000000 lea rcx, [rax + 0xc8]
1407f7d93: 488d15de442902 lea rdx, [rip + 0x22944de]
1407f7d9a: e861b8d6ff call 0x140563600 ; alloc::raw_vec::RawVec::grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::vec2::Vec2,utils::curve::curve_u::CurveU<utils::curve::CurveCoor
1407f7d9f: 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
1407f7da6: 488b82d0000000 mov rax, qword ptr [rdx + 0xd0]
1407f7dad: 488d0cbf lea rcx, [rdi + rdi*4]
1407f7db1: 440f133488 movlps qword ptr [rax + rcx*4], xmm14
1407f7db6: f3440f116c8808 movss dword ptr [rax + rcx*4 + 8], xmm13
1407f7dbd: f3440f1164880c movss dword ptr [rax + rcx*4 + 0xc], xmm12
1407f7dc4: c644881000 mov byte ptr [rax + rcx*4 + 0x10], 0
1407f7dc9: 48ffc7 inc rdi
1407f7dcc: 4889bad8000000 mov qword ptr [rdx + 0xd8], rdi
1407f7dd3: 4c8b8548010000 mov r8, qword ptr [rbp + 0x148]
1407f7dda: 4c8b8d50010000 mov r9, qword ptr [rbp + 0x150]
1407f7de1: f3440f11442458 movss dword ptr [rsp + 0x58], xmm8
1407f7de8: 488d8520010000 lea rax, [rbp + 0x120]
1407f7def: 4889442438 mov qword ptr [rsp + 0x38], rax
1407f7df4: 4889542430 mov qword ptr [rsp + 0x30], rdx
1407f7df9: 8b85c8000000 mov eax, dword ptr [rbp + 0xc8]
1407f7dff: 89442428 mov dword ptr [rsp + 0x28], eax
1407f7e03: 0fb68538010000 movzx eax, byte ptr [rbp + 0x138]
1407f7e0a: 88442420 mov byte ptr [rsp + 0x20], al
1407f7e0e: c744245001000000 mov dword ptr [rsp + 0x50], 1
1407f7e16: c744244800000000 mov dword ptr [rsp + 0x48], 0
1407f7e1e: c744244000000000 mov dword ptr [rsp + 0x40], 0
1407f7e26: 488b8d68010000 mov rcx, qword ptr [rbp + 0x168]
1407f7e2d: 488b95d0000000 mov rdx, qword ptr [rbp + 0xd0]
1407f7e34: e8577bffff call 0x1407ef990 ; system_decorator::decorator_visual::cottage_wall_window::add_lintels
1407f7e39: 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
1407f7e40: 83b85802000002 cmp dword ptr [rax + 0x258], 2
1407f7e47: 0f8351040000 jae 0x1407f829e
1407f7e4d: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1407f7e54: 8b4ae4 mov ecx, dword ptr [rdx - 0x1c]
1407f7e57: 69c1b6aaaaaa imul eax, ecx, 0xaaaaaab6
1407f7e5d: 054b885655 add eax, 0x5556884b
1407f7e62: 3d55555555 cmp eax, 0x55555555
1407f7e67: 0f871b010000 ja 0x1407f7f88
1407f7e6d: 0fb642fc movzx eax, byte ptr [rdx - 4]
1407f7e71: 83f803 cmp eax, 3
1407f7e74: 0f830e010000 jae 0x1407f7f88
1407f7e7a: 83f801 cmp eax, 1
1407f7e7d: 488b9d68010000 mov rbx, qword ptr [rbp + 0x168]
1407f7e84: 740d je 0x1407f7e93
1407f7e86: 83f802 cmp eax, 2
1407f7e89: 0f855a080000 jne 0x1407f86e9
1407f7e8f: 440f28c7 movaps xmm8, xmm7
1407f7e93: 0fb6056fcde302 movzx eax, byte ptr [rip + 0x2e3cd6f]
1407f7e9a: b950000000 mov ecx, 0x50
1407f7e9f: ba10000000 mov edx, 0x10
1407f7ea4: e867bde1ff call 0x140613c10 ; unknown
1407f7ea9: 4885c0 test rax, rax
1407f7eac: 0f8426080000 je 0x1407f86d8
1407f7eb2: 4889c7 mov rdi, rax
1407f7eb5: 0f280dd43d2902 movaps xmm1, xmmword ptr [rip + 0x2293dd4]
1407f7ebc: 440f59d9 mulps xmm11, xmm1
1407f7ec0: 410f28c2 movaps xmm0, xmm10
1407f7ec4: 0f5905d53d2902 mulps xmm0, xmmword ptr [rip + 0x2293dd5]
1407f7ecb: 410f58c3 addps xmm0, xmm11
1407f7ecf: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407f7ed3: 440f59f8 mulps xmm15, xmm0
1407f7ed7: 0f59f1 mulps xmm6, xmm1
1407f7eda: 410f28c2 movaps xmm0, xmm10
1407f7ede: 0f59c1 mulps xmm0, xmm1
1407f7ee1: f30f16c8 movshdup xmm1, xmm0
1407f7ee5: f30f58c8 addss xmm1, xmm0
1407f7ee9: 0f12c0 movhlps xmm0, xmm0
1407f7eec: f30f58c1 addss xmm0, xmm1
1407f7ef0: f30f58c0 addss xmm0, xmm0
1407f7ef4: 0fc6c000 shufps xmm0, xmm0, 0
1407f7ef8: 410f59c2 mulps xmm0, xmm10
1407f7efc: 0f58c6 addps xmm0, xmm6
1407f7eff: 410f58c7 addps xmm0, xmm15
1407f7f03: 0f28c8 movaps xmm1, xmm0
1407f7f06: 660f15c8 unpckhpd xmm1, xmm0
1407f7f0a: f3410f58c9 addss xmm1, xmm9
1407f7f0f: 440f2910 movaps xmmword ptr [rax], xmm10
1407f7f13: 0f5885b0000000 addps xmm0, xmmword ptr [rbp + 0xb0]
1407f7f1a: 0f134010 movlps qword ptr [rax + 0x10], xmm0
1407f7f1e: f30f114818 movss dword ptr [rax + 0x18], xmm1
1407f7f23: 48b80000803f0000803f movabs rax, 0x3f8000003f800000
1407f7f2d: 4889471c mov qword ptr [rdi + 0x1c], rax
1407f7f31: c747240000803f mov dword ptr [rdi + 0x24], 0x3f800000
1407f7f38: 488b85c0000000 mov rax, qword ptr [rbp + 0xc0]
1407f7f3f: 48894730 mov qword ptr [rdi + 0x30], rax
1407f7f43: f3440f114738 movss dword ptr [rdi + 0x38], xmm8
1407f7f49: f3440f11673c movss dword ptr [rdi + 0x3c], xmm12
1407f7f4f: 895f40 mov dword ptr [rdi + 0x40], ebx
1407f7f52: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7f59: 488b8810010000 mov rcx, qword ptr [rax + 0x110]
1407f7f60: 4885c9 test rcx, rcx
1407f7f63: 7410 je 0x1407f7f75
1407f7f65: ba50000000 mov edx, 0x50
1407f7f6a: 41b810000000 mov r8d, 0x10
1407f7f70: e8abbce1ff call 0x140613c20 ; unknown
1407f7f75: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7f7c: 4889b810010000 mov qword ptr [rax + 0x110], rdi
1407f7f83: e916030000 jmp 0x1407f829e ; unknown
1407f7f88: e8b3e24900 call 0x140c96240 ; utils::rng::init_rng
1407f7f8d: 4889c7 mov rdi, rax
1407f7f90: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407f7f9a: 4801c7 add rdi, rax
1407f7f9d: 4889f8 mov rax, rdi
1407f7fa0: 48b9db28b4a0d17e03e7 movabs rcx, 0xe7037ed1a0b428db
1407f7faa: 4831c8 xor rax, rcx
1407f7fad: 48f7e7 mul rdi
1407f7fb0: 31d0 xor eax, edx
1407f7fb2: c1e81e shr eax, 0x1e
1407f7fb5: 83f801 cmp eax, 1
1407f7fb8: 0f85e0020000 jne 0x1407f829e
1407f7fbe: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f7fc5: 0fb650fc movzx edx, byte ptr [rax - 4]
1407f7fc9: 440f2995c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm10
1407f7fd1: 660f6f85b0000000 movdqa xmm0, xmmword ptr [rbp + 0xb0]
1407f7fd9: 660fd685d0010000 movq qword ptr [rbp + 0x1d0], xmm0
1407f7fe1: f3440f118dd8010000 movss dword ptr [rbp + 0x1d8], xmm9
1407f7fea: 488d85c4010000 lea rax, [rbp + 0x1c4]
1407f7ff1: 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
1407f7ffb: 48894818 mov qword ptr [rax + 0x18], rcx
1407f7fff: c740200000803f mov dword ptr [rax + 0x20], 0x3f800000
1407f8006: 488b4550 mov rax, qword ptr [rbp + 0x50]
1407f800a: 4889442420 mov qword ptr [rsp + 0x20], rax
1407f800f: 488d8d78010000 lea rcx, [rbp + 0x178]
1407f8016: 41b003 mov r8b, 3
1407f8019: 4c8d8dc0010000 lea r9, [rbp + 0x1c0]
1407f8020: e88b77ffff call 0x1407ef7b0 ; system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
1407f8025: 488b8580010000 mov rax, qword ptr [rbp + 0x180]
1407f802c: 488985b0000000 mov qword ptr [rbp + 0xb0], rax
1407f8033: 488b8588010000 mov rax, qword ptr [rbp + 0x188]
1407f803a: 4885c0 test rax, rax
1407f803d: 0f8435020000 je 0x1407f8278
1407f8043: 488d0440 lea rax, [rax + rax*2]
1407f8047: 48c1e004 shl rax, 4
1407f804b: 4c8bbdb0000000 mov r15, qword ptr [rbp + 0xb0]
1407f8052: 4c01f8 add rax, r15
1407f8055: 488985c8000000 mov qword ptr [rbp + 0xc8], rax
1407f805c: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f8063: 4805f8000000 add rax, 0xf8
1407f8069: 488985d0000000 mov qword ptr [rbp + 0xd0], rax
1407f8070: 4531c9 xor r9d, r9d
1407f8073: e995000000 jmp 0x1407f810d ; unknown
1407f8078: 0f1f840000000000 nop dword ptr [rax + rax]
1407f8080: 4983c730 add r15, 0x30
1407f8084: 4c8b8db8010000 mov r9, qword ptr [rbp + 0x1b8]
1407f808b: 49ffc1 inc r9
1407f808e: 4c8b8500020000 mov r8, qword ptr [rbp + 0x200]
1407f8095: 498b8000010000 mov rax, qword ptr [r8 + 0x100]
1407f809c: 488d0c9b lea rcx, [rbx + rbx*4]
1407f80a0: 48c1e104 shl rcx, 4
1407f80a4: c6040801 mov byte ptr [rax + rcx], 1
1407f80a8: 488b9538010000 mov rdx, qword ptr [rbp + 0x138]
1407f80af: 4889540808 mov qword ptr [rax + rcx + 8], rdx
1407f80b4: 660f6f85e0000000 movdqa xmm0, xmmword ptr [rbp + 0xe0]
1407f80bc: 660f6f8df0000000 movdqa xmm1, xmmword ptr [rbp + 0xf0]
1407f80c4: 660f6f9500010000 movdqa xmm2, xmmword ptr [rbp + 0x100]
1407f80cc: 660f7f440810 movdqa xmmword ptr [rax + rcx + 0x10], xmm0
1407f80d2: 660f7f4c0820 movdqa xmmword ptr [rax + rcx + 0x20], xmm1
1407f80d8: 660f7f540830 movdqa xmmword ptr [rax + rcx + 0x30], xmm2
1407f80de: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1407f80e5: 4889540840 mov qword ptr [rax + rcx + 0x40], rdx
1407f80ea: 488b95c0000000 mov rdx, qword ptr [rbp + 0xc0]
1407f80f1: 4889540848 mov qword ptr [rax + rcx + 0x48], rdx
1407f80f6: 48ffc3 inc rbx
1407f80f9: 49899808010000 mov qword ptr [r8 + 0x108], rbx
1407f8100: 4c3bbdc8000000 cmp r15, qword ptr [rbp + 0xc8]
1407f8107: 0f846b010000 je 0x1407f8278
1407f810d: 66410f6f07 movdqa xmm0, xmmword ptr [r15]
1407f8112: 66410f6f4f10 movdqa xmm1, xmmword ptr [r15 + 0x10]
1407f8118: 66410f6f5720 movdqa xmm2, xmmword ptr [r15 + 0x20]
1407f811e: 660f7f95e0010000 movdqa xmmword ptr [rbp + 0x1e0], xmm2
1407f8126: 660f7f8dd0010000 movdqa xmmword ptr [rbp + 0x1d0], xmm1
1407f812e: 660f7f85c0010000 movdqa xmmword ptr [rbp + 0x1c0], xmm0
1407f8136: 488b8568010000 mov rax, qword ptr [rbp + 0x168]
1407f813d: 8985a0010000 mov dword ptr [rbp + 0x1a0], eax
1407f8143: 488d0546412902 lea rax, [rip + 0x2294146]
1407f814a: 48898590010000 mov qword ptr [rbp + 0x190], rax
1407f8151: 48c785980100000a000000 mov qword ptr [rbp + 0x198], 0xa
1407f815c: 4c898db8010000 mov qword ptr [rbp + 0x1b8], r9
1407f8163: 4c898da8010000 mov qword ptr [rbp + 0x1a8], r9
1407f816a: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f8171: e8fa48a900 call 0x14128ca70 ; utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize> >
1407f8176: 48898508020000 mov qword ptr [rbp + 0x208], rax
1407f817d: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407f8187: 4c8d0407 lea r8, [rdi + rax]
1407f818b: 4c89c0 mov rax, r8
1407f818e: 48b9db28b4a0d17e03e7 movabs rcx, 0xe7037ed1a0b428db
1407f8198: 4831c8 xor rax, rcx
1407f819b: 49f7e0 mul r8
1407f819e: 4889d1 mov rcx, rdx
1407f81a1: 4831c1 xor rcx, rax
1407f81a4: 4889c8 mov rax, rcx
1407f81a7: ba06000000 mov edx, 6
1407f81ac: 48f7e2 mul rdx
1407f81af: 4801c9 add rcx, rcx
1407f81b2: 488d0449 lea rax, [rcx + rcx*2]
1407f81b6: 4883f803 cmp rax, 3
1407f81ba: 774f ja 0x1407f820b
1407f81bc: 48b85ec87af1c83aec40 movabs rax, 0x40ec3ac8f17ac85e
1407f81c6: 4801c7 add rdi, rax
1407f81c9: 48b92f64bd78641d76a0 movabs rcx, 0xa0761d6478bd642f
1407f81d3: 49b9db28b4a0d17e03e7 movabs r9, 0xe7037ed1a0b428db
1407f81dd: 0f1f00 nop dword ptr [rax]
1407f81e0: 4889f8 mov rax, rdi
1407f81e3: 4c31c8 xor rax, r9
1407f81e6: 48f7e7 mul rdi
1407f81e9: 4901c8 add r8, rcx
1407f81ec: 4831c2 xor rdx, rax
1407f81ef: 488d0412 lea rax, [rdx + rdx]
1407f81f3: 488d0440 lea rax, [rax + rax*2]
1407f81f7: 4801cf add rdi, rcx
1407f81fa: 4883f804 cmp rax, 4
1407f81fe: 72e0 jb 0x1407f81e0
1407f8200: 4889d0 mov rax, rdx
1407f8203: b906000000 mov ecx, 6
1407f8208: 48f7e1 mul rcx
1407f820b: 4c89c7 mov rdi, r8
1407f820e: 48899538010000 mov qword ptr [rbp + 0x138], rdx
1407f8215: 660f6f85c0010000 movdqa xmm0, xmmword ptr [rbp + 0x1c0]
1407f821d: 660f6f8dd0010000 movdqa xmm1, xmmword ptr [rbp + 0x1d0]
1407f8225: 660f6f95e0010000 movdqa xmm2, xmmword ptr [rbp + 0x1e0]
1407f822d: 660f7f9500010000 movdqa xmmword ptr [rbp + 0x100], xmm2
1407f8235: 660f7f8df0000000 movdqa xmmword ptr [rbp + 0xf0], xmm1
1407f823d: 660f7f85e0000000 movdqa xmmword ptr [rbp + 0xe0], xmm0
1407f8245: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f824c: 488b9808010000 mov rbx, qword ptr [rax + 0x108]
1407f8253: 483b98f8000000 cmp rbx, qword ptr [rax + 0xf8]
1407f825a: 0f8520feffff jne 0x1407f8080
1407f8260: 488b8dd0000000 mov rcx, qword ptr [rbp + 0xd0]
1407f8267: 488d1532402902 lea rdx, [rip + 0x2294032]
1407f826e: e88d47f9ff call 0x14078ca00 ; alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::affine3a::Affine3A,u32,array$<f32,2> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc
1407f8273: e908feffff jmp 0x1407f8080 ; unknown
1407f8278: 488b8578010000 mov rax, qword ptr [rbp + 0x178]
1407f827f: 4885c0 test rax, rax
1407f8282: 741a je 0x1407f829e
1407f8284: 48c1e004 shl rax, 4
1407f8288: 488d1440 lea rdx, [rax + rax*2]
1407f828c: 41b810000000 mov r8d, 0x10
1407f8292: 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
1407f8299: e882b9e1ff call 0x140613c20 ; unknown
1407f829e: 488b8540010000 mov rax, qword ptr [rbp + 0x140]
1407f82a5: 4885c0 test rax, rax
1407f82a8: 741a je 0x1407f82c4
1407f82aa: 488b8d48010000 mov rcx, qword ptr [rbp + 0x148]
1407f82b1: 48c1e004 shl rax, 4
1407f82b5: 488d1440 lea rdx, [rax + rax*2]
1407f82b9: 41b810000000 mov r8d, 0x10
1407f82bf: e85cb9e1ff call 0x140613c20 ; unknown
1407f82c4: 8b8574010000 mov eax, dword ptr [rbp + 0x174]
1407f82ca: 89c1 mov ecx, eax
1407f82cc: e9e0e7ffff jmp 0x1407f6ab1 ; unknown
1407f82d1: 8b855c010000 mov eax, dword ptr [rbp + 0x15c]
1407f82d7: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
1407f82de: 4c8985a8000000 mov qword ptr [rbp + 0xa8], r8
1407f82e5: 4d85c0 test r8, r8
1407f82e8: 0f8504e7ffff jne 0x1407f69f2
1407f82ee: 80bd1602000000 cmp byte ptr [rbp + 0x216], 0
1407f82f5: 744b je 0x1407f8342
1407f82f7: e834fe4100 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1407f82fc: 4889c1 mov rcx, rax
1407f82ff: 488b00 mov rax, qword ptr [rax]
1407f8302: 4883f801 cmp rax, 1
1407f8306: 0f85ac010000 jne 0x1407f84b8
1407f830c: 4883c108 add rcx, 8
1407f8310: 48833900 cmp qword ptr [rcx], 0
1407f8314: 0f85df010000 jne 0x1407f84f9
1407f831a: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1407f8321: 48898d60010000 mov qword ptr [rbp + 0x160], rcx
1407f8328: 4883c108 add rcx, 8
1407f832c: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
1407f8333: e8b8ed4100 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1407f8338: 488b8560010000 mov rax, qword ptr [rbp + 0x160]
1407f833f: 48ff00 inc qword ptr [rax]
1407f8342: 0f28b520020000 movaps xmm6, xmmword ptr [rbp + 0x220]
1407f8349: 0f28bd30020000 movaps xmm7, xmmword ptr [rbp + 0x230]
1407f8350: 440f288540020000 movaps xmm8, xmmword ptr [rbp + 0x240]
1407f8358: 440f288d50020000 movaps xmm9, xmmword ptr [rbp + 0x250]
1407f8360: 440f289560020000 movaps xmm10, xmmword ptr [rbp + 0x260]
1407f8368: 440f289d70020000 movaps xmm11, xmmword ptr [rbp + 0x270]
1407f8370: 440f28a580020000 movaps xmm12, xmmword ptr [rbp + 0x280]
1407f8378: 440f28ad90020000 movaps xmm13, xmmword ptr [rbp + 0x290]
1407f8380: 440f28b5a0020000 movaps xmm14, xmmword ptr [rbp + 0x2a0]
1407f8388: 440f28bdb0020000 movaps xmm15, xmmword ptr [rbp + 0x2b0]
1407f8390: 4881c448030000 add rsp, 0x348
1407f8397: 5b pop rbx
1407f8398: 5f pop rdi
1407f8399: 5e pop rsi
1407f839a: 415c pop r12
1407f839c: 415d pop r13
1407f839e: 415e pop r14
1407f83a0: 415f pop r15
1407f83a2: 5d pop rbp
1407f83a3: c3 ret 
1407f83a4: 488d0d3d8f4202 lea rcx, [rip + 0x2428f3d]
1407f83ab: e8600f0e02 call 0x1428d9310 ; core::option::unwrap_failed
1407f83b0: e94c030000 jmp 0x1407f8701 ; unknown
1407f83b5: 488d8590010000 lea rax, [rbp + 0x190]
1407f83bc: 488985e0000000 mov qword ptr [rbp + 0xe0], rax
1407f83c3: 488d05c6ddffff lea rax, [rip - 0x223a]
1407f83ca: 488985e8000000 mov qword ptr [rbp + 0xe8], rax
1407f83d1: 488d05d8392902 lea rax, [rip + 0x22939d8]
1407f83d8: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407f83df: 48c785c801000001000000 mov qword ptr [rbp + 0x1c8], 1
1407f83ea: 48c785e001000000000000 mov qword ptr [rbp + 0x1e0], 0
1407f83f5: 488d85e0000000 lea rax, [rbp + 0xe0]
1407f83fc: 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
1407f8403: 48c785d801000001000000 mov qword ptr [rbp + 0x1d8], 1
1407f840e: 488d15ab392902 lea rdx, [rip + 0x22939ab]
1407f8415: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f841c: e86f0f0e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407f8421: e9db020000 jmp 0x1407f8701 ; unknown
1407f8426: 488d8578010000 lea rax, [rbp + 0x178]
1407f842d: 488985e0000000 mov qword ptr [rbp + 0xe0], rax
1407f8434: 488d0555ddffff lea rax, [rip - 0x22ab]
1407f843b: 488985e8000000 mov qword ptr [rbp + 0xe8], rax
1407f8442: 488d0567392902 lea rax, [rip + 0x2293967]
1407f8449: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407f8450: 48c785c801000001000000 mov qword ptr [rbp + 0x1c8], 1
1407f845b: 48c785e001000000000000 mov qword ptr [rbp + 0x1e0], 0
1407f8466: 488d85e0000000 lea rax, [rbp + 0xe0]
1407f846d: 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
1407f8474: 48c785d801000001000000 mov qword ptr [rbp + 0x1d8], 1
1407f847f: c6851702000001 mov byte ptr [rbp + 0x217], 1
1407f8486: 488d154b392902 lea rdx, [rip + 0x229394b]
1407f848d: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f8494: e8f70e0e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407f8499: e963020000 jmp 0x1407f8701 ; unknown
1407f849e: 4885c0 test rax, rax
1407f84a1: 752d jne 0x1407f84d0
1407f84a3: 31d2 xor edx, edx
1407f84a5: e8d6a00f02 call 0x1428f2580 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1407f84aa: 4889c1 mov rcx, rax
1407f84ad: 4885c0 test rax, rax
1407f84b0: 0f8554e4ffff jne 0x1407f690a
1407f84b6: eb18 jmp 0x1407f84d0 ; unknown
1407f84b8: 4885c0 test rax, rax
1407f84bb: 7513 jne 0x1407f84d0
1407f84bd: 31d2 xor edx, edx
1407f84bf: e8bca00f02 call 0x1428f2580 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1407f84c4: 4889c1 mov rcx, rax
1407f84c7: 4885c0 test rax, rax
1407f84ca: 0f8540feffff jne 0x1407f8310
1407f84d0: 488d0df18e4202 lea rcx, [rip + 0x2428ef1]
1407f84d7: e8f4fb0d02 call 0x1428d80d0 ; std::thread::local::panic_access_error
1407f84dc: 488d0db53b2902 lea rcx, [rip + 0x2293bb5]
1407f84e3: 4c8d05d63b2902 lea r8, [rip + 0x2293bd6]
1407f84ea: ba28000000 mov edx, 0x28
1407f84ef: e83c0f0e02 call 0x1428d9430 ; core::panicking::panic
1407f84f4: e908020000 jmp 0x1407f8701 ; unknown
1407f84f9: 488d0d388f4202 lea rcx, [rip + 0x2428f38]
1407f8500: e84b0d0e02 call 0x1428d9250 ; core::cell::panic_already_borrowed
1407f8505: 488d051c922902 lea rax, [rip + 0x229921c]
1407f850c: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407f8513: 48c785c801000001000000 mov qword ptr [rbp + 0x1c8], 1
1407f851e: 48c785d001000008000000 mov qword ptr [rbp + 0x1d0], 8
1407f8529: 660fefc0 pxor xmm0, xmm0
1407f852d: f30f7f85d8010000 movdqu xmmword ptr [rbp + 0x1d8], xmm0
1407f8535: 488d15fc912902 lea rdx, [rip + 0x22991fc]
1407f853c: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f8543: e8480e0e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407f8548: e9b4010000 jmp 0x1407f8701 ; unknown
1407f854d: 488d0d443b2902 lea rcx, [rip + 0x2293b44]
1407f8554: 4c8d05d53d2902 lea r8, [rip + 0x2293dd5]
1407f855b: ba28000000 mov edx, 0x28
1407f8560: e8cb0e0e02 call 0x1428d9430 ; core::panicking::panic
1407f8565: e997010000 jmp 0x1407f8701 ; unknown
1407f856a: 488d0d273b2902 lea rcx, [rip + 0x2293b27]
1407f8571: 4c8d05a03d2902 lea r8, [rip + 0x2293da0]
1407f8578: ba28000000 mov edx, 0x28
1407f857d: e8ae0e0e02 call 0x1428d9430 ; core::panicking::panic
1407f8582: e97a010000 jmp 0x1407f8701 ; unknown
1407f8587: 488d0dc23a2902 lea rcx, [rip + 0x2293ac2]
1407f858e: e87d0d0e02 call 0x1428d9310 ; core::option::unwrap_failed
1407f8593: e969010000 jmp 0x1407f8701 ; unknown
1407f8598: 4c8d05e13a2902 lea r8, [rip + 0x2293ae1]
1407f859f: b910000000 mov ecx, 0x10
1407f85a4: 4889da mov rdx, rbx
1407f85a7: e8370a0e02 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1407f85ac: e950010000 jmp 0x1407f8701 ; unknown
1407f85b1: 488d8590010000 lea rax, [rbp + 0x190]
1407f85b8: 488985e0000000 mov qword ptr [rbp + 0xe0], rax
1407f85bf: 488d052a49a600 lea rax, [rip + 0xa6492a]
1407f85c6: 488985e8000000 mov qword ptr [rbp + 0xe8], rax
1407f85cd: 488d05ac964202 lea rax, [rip + 0x24296ac]
1407f85d4: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407f85db: 48c785c801000001000000 mov qword ptr [rbp + 0x1c8], 1
1407f85e6: 48c785e001000000000000 mov qword ptr [rbp + 0x1e0], 0
1407f85f1: 488d85e0000000 lea rax, [rbp + 0xe0]
1407f85f8: 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
1407f85ff: 48c785d801000001000000 mov qword ptr [rbp + 0x1d8], 1
1407f860a: 488d1597964202 lea rdx, [rip + 0x2429697]
1407f8611: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f8618: e8730d0e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407f861d: e9df000000 jmp 0x1407f8701 ; unknown
1407f8622: 488d0d6f3a2902 lea rcx, [rip + 0x2293a6f]
1407f8629: 4c8d05b83c2902 lea r8, [rip + 0x2293cb8]
1407f8630: ba28000000 mov edx, 0x28
1407f8635: e8f60d0e02 call 0x1428d9430 ; core::panicking::panic
1407f863a: e9c2000000 jmp 0x1407f8701 ; unknown
1407f863f: 488d0dba3c2902 lea rcx, [rip + 0x2293cba]
1407f8646: e8c50c0e02 call 0x1428d9310 ; core::option::unwrap_failed
1407f864b: e9b1000000 jmp 0x1407f8701 ; unknown
1407f8650: 488d8590010000 lea rax, [rbp + 0x190]
1407f8657: 488985e0000000 mov qword ptr [rbp + 0xe0], rax
1407f865e: 488d058b48a600 lea rax, [rip + 0xa6488b]
1407f8665: 488985e8000000 mov qword ptr [rbp + 0xe8], rax
1407f866c: 488d050d964202 lea rax, [rip + 0x242960d]
1407f8673: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407f867a: 48c785c801000001000000 mov qword ptr [rbp + 0x1c8], 1
1407f8685: 48c785e001000000000000 mov qword ptr [rbp + 0x1e0], 0
1407f8690: 488d85e0000000 lea rax, [rbp + 0xe0]
1407f8697: 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
1407f869e: 48c785d801000001000000 mov qword ptr [rbp + 0x1d8], 1
1407f86a9: 488d15f8954202 lea rdx, [rip + 0x24295f8]
1407f86b0: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f86b7: e8d40c0e02 call 0x1428d9390 ; core::panicking::panic_fmt
1407f86bc: eb43 jmp 0x1407f8701 ; unknown
1407f86be: 488d0dd3392902 lea rcx, [rip + 0x22939d3]
1407f86c5: 4c8d05043c2902 lea r8, [rip + 0x2293c04]
1407f86cc: ba28000000 mov edx, 0x28
1407f86d1: e85a0d0e02 call 0x1428d9430 ; core::panicking::panic
1407f86d6: eb29 jmp 0x1407f8701 ; unknown
1407f86d8: b910000000 mov ecx, 0x10
1407f86dd: ba50000000 mov edx, 0x50
1407f86e2: e819090e02 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1407f86e7: eb18 jmp 0x1407f8701 ; unknown
1407f86e9: 488d0da8392902 lea rcx, [rip + 0x22939a8]
1407f86f0: 4c8d05c13b2902 lea r8, [rip + 0x2293bc1]
1407f86f7: ba28000000 mov edx, 0x28
1407f86fc: e82f0d0e02 call 0x1428d9430 ; core::panicking::panic
1407f8701: 0f0b ud2 
1407f8703: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407f8710: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407f8715: 55 push rbp
1407f8716: 4157 push r15
1407f8718: 4156 push r14
1407f871a: 4155 push r13
1407f871c: 4154 push r12
1407f871e: 56 push rsi
1407f871f: 57 push rdi
1407f8720: 53 push rbx
1407f8721: 4881ec08010000 sub rsp, 0x108
1407f8728: 488daa80000000 lea rbp, [rdx + 0x80]
1407f872f: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407f8735: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407f873b: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407f8744: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407f874d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407f8756: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407f875f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407f8768: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407f8771: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407f8779: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407f8781: 488b8530010000 mov rax, qword ptr [rbp + 0x130]
1407f8788: 48ff00 inc qword ptr [rax]
1407f878b: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407f8793: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407f879b: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407f87a4: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407f87ad: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407f87b6: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407f87bf: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407f87c8: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407f87d1: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407f87d7: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407f87dd: 4881c408010000 add rsp, 0x108
1407f87e4: 5b pop rbx
1407f87e5: 5f pop rdi
1407f87e6: 5e pop rsi
1407f87e7: 415c pop r12
1407f87e9: 415d pop r13
1407f87eb: 415e pop r14
1407f87ed: 415f pop r15
1407f87ef: 5d pop rbp
1407f87f0: c3 ret 
1407f87f1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407f8800: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407f8805: 55 push rbp
1407f8806: 4157 push r15
1407f8808: 4156 push r14
1407f880a: 4155 push r13
1407f880c: 4154 push r12
1407f880e: 56 push rsi
1407f880f: 57 push rdi
1407f8810: 53 push rbx
1407f8811: 4881ec08010000 sub rsp, 0x108
1407f8818: 488daa80000000 lea rbp, [rdx + 0x80]
1407f881f: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407f8825: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407f882b: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407f8834: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407f883d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407f8846: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407f884f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407f8858: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407f8861: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407f8869: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407f8871: 80bd1602000000 cmp byte ptr [rbp + 0x216], 0
1407f8878: 743d je 0x1407f88b7
1407f887a: e8b1f84100 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1407f887f: 4889c6 mov rsi, rax
1407f8882: 488b00 mov rax, qword ptr [rax]
1407f8885: 4883f801 cmp rax, 1
1407f8889: 0f858e000000 jne 0x1407f891d
1407f888f: 4883c608 add rsi, 8
1407f8893: 48833e00 cmp qword ptr [rsi], 0
1407f8897: 0f85a7000000 jne 0x1407f8944
1407f889d: 48c706ffffffff mov qword ptr [rsi], 0xffffffffffffffff
1407f88a4: 488d4e08 lea rcx, [rsi + 8]
1407f88a8: 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
1407f88af: e83ce84100 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1407f88b4: 48ff06 inc qword ptr [rsi]
1407f88b7: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407f88bf: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407f88c7: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407f88d0: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407f88d9: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407f88e2: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407f88eb: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407f88f4: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407f88fd: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407f8903: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407f8909: 4881c408010000 add rsp, 0x108
1407f8910: 5b pop rbx
1407f8911: 5f pop rdi
1407f8912: 5e pop rsi
1407f8913: 415c pop r12
1407f8915: 415d pop r13
1407f8917: 415e pop r14
1407f8919: 415f pop r15
1407f891b: 5d pop rbp
1407f891c: c3 ret 
1407f891d: 4885c0 test rax, rax
1407f8920: 7516 jne 0x1407f8938
1407f8922: 4889f1 mov rcx, rsi
1407f8925: 31d2 xor edx, edx
1407f8927: e8549c0f02 call 0x1428f2580 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1407f892c: 4889c6 mov rsi, rax
1407f892f: 4885c0 test rax, rax
1407f8932: 0f855bffffff jne 0x1407f8893
1407f8938: 488d0d898a4202 lea rcx, [rip + 0x2428a89]
1407f893f: e88cf70d02 call 0x1428d80d0 ; std::thread::local::panic_access_error
1407f8944: 488d0ded8a4202 lea rcx, [rip + 0x2428aed]
1407f894b: e800090e02 call 0x1428d9250 ; core::cell::panic_already_borrowed
1407f8950: cc int3 
1407f8951: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407f8960: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407f8965: 55 push rbp
1407f8966: 4157 push r15
1407f8968: 4156 push r14
1407f896a: 4155 push r13
1407f896c: 4154 push r12
1407f896e: 56 push rsi
1407f896f: 57 push rdi
1407f8970: 53 push rbx
1407f8971: 4881ec08010000 sub rsp, 0x108
1407f8978: 488daa80000000 lea rbp, [rdx + 0x80]
1407f897f: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407f8985: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407f898b: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407f8994: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407f899d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407f89a6: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407f89af: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407f89b8: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407f89c1: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407f89c9: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407f89d1: 488b8560010000 mov rax, qword ptr [rbp + 0x160]
1407f89d8: 48ff00 inc qword ptr [rax]
1407f89db: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407f89e3: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407f89eb: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407f89f4: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407f89fd: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407f8a06: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407f8a0f: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407f8a18: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407f8a21: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407f8a27: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407f8a2d: 4881c408010000 add rsp, 0x108
1407f8a34: 5b pop rbx
1407f8a35: 5f pop rdi
1407f8a36: 5e pop rsi
1407f8a37: 415c pop r12
1407f8a39: 415d pop r13
1407f8a3b: 415e pop r14
1407f8a3d: 415f pop r15
1407f8a3f: 5d pop rbp
1407f8a40: c3 ret 
1407f8a41: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407f8a50: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407f8a55: 55 push rbp
1407f8a56: 4157 push r15
1407f8a58: 4156 push r14
1407f8a5a: 4155 push r13
1407f8a5c: 4154 push r12
1407f8a5e: 56 push rsi
1407f8a5f: 57 push rdi
1407f8a60: 53 push rbx
1407f8a61: 4881ec08010000 sub rsp, 0x108
1407f8a68: 488daa80000000 lea rbp, [rdx + 0x80]
1407f8a6f: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407f8a75: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407f8a7b: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407f8a84: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407f8a8d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407f8a96: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407f8a9f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407f8aa8: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407f8ab1: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407f8ab9: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407f8ac1: 488b8540010000 mov rax, qword ptr [rbp + 0x140]
1407f8ac8: 4885c0 test rax, rax
1407f8acb: 741a je 0x1407f8ae7
1407f8acd: 488b8d48010000 mov rcx, qword ptr [rbp + 0x148]
1407f8ad4: 48c1e004 shl rax, 4
1407f8ad8: 488d1440 lea rdx, [rax + rax*2]
1407f8adc: 41b810000000 mov r8d, 0x10
1407f8ae2: e839b1e1ff call 0x140613c20 ; unknown
1407f8ae7: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407f8aef: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407f8af7: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407f8b00: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407f8b09: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407f8b12: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407f8b1b: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407f8b24: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407f8b2d: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407f8b33: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407f8b39: 4881c408010000 add rsp, 0x108
1407f8b40: 5b pop rbx
1407f8b41: 5f pop rdi
1407f8b42: 5e pop rsi
1407f8b43: 415c pop r12
1407f8b45: 415d pop r13
1407f8b47: 415e pop r14
1407f8b49: 415f pop r15
1407f8b4b: 5d pop rbp
1407f8b4c: c3 ret 
1407f8b4d: 0f1f00 nop dword ptr [rax]
1407f8b50: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407f8b55: 55 push rbp
1407f8b56: 4157 push r15
1407f8b58: 4156 push r14
1407f8b5a: 4155 push r13
1407f8b5c: 4154 push r12
1407f8b5e: 56 push rsi
1407f8b5f: 57 push rdi
1407f8b60: 53 push rbx
1407f8b61: 4881ec08010000 sub rsp, 0x108
1407f8b68: 488daa80000000 lea rbp, [rdx + 0x80]
1407f8b6f: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407f8b75: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407f8b7b: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407f8b84: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407f8b8d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407f8b96: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407f8b9f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407f8ba8: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407f8bb1: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407f8bb9: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407f8bc1: 488b8578010000 mov rax, qword ptr [rbp + 0x178]
1407f8bc8: 4885c0 test rax, rax
1407f8bcb: 741a je 0x1407f8be7
1407f8bcd: 48c1e004 shl rax, 4
1407f8bd1: 488d1440 lea rdx, [rax + rax*2]
1407f8bd5: 41b810000000 mov r8d, 0x10
1407f8bdb: 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
1407f8be2: e839b0e1ff call 0x140613c20 ; unknown
1407f8be7: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407f8bef: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407f8bf7: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407f8c00: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407f8c09: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407f8c12: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407f8c1b: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407f8c24: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407f8c2d: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407f8c33: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407f8c39: 4881c408010000 add rsp, 0x108
1407f8c40: 5b pop rbx
1407f8c41: 5f pop rdi
1407f8c42: 5e pop rsi
1407f8c43: 415c pop r12
1407f8c45: 415d pop r13
1407f8c47: 415e pop r14
1407f8c49: 415f pop r15
1407f8c4b: 5d pop rbp
1407f8c4c: c3 ret 
1407f8c4d: 0f1f00 nop dword ptr [rax]
1407f8c50: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407f8c55: 55 push rbp
1407f8c56: 4157 push r15
1407f8c58: 4156 push r14
1407f8c5a: 4155 push r13
1407f8c5c: 4154 push r12
1407f8c5e: 56 push rsi
1407f8c5f: 57 push rdi
1407f8c60: 53 push rbx
1407f8c61: 4881ec08010000 sub rsp, 0x108
1407f8c68: 488daa80000000 lea rbp, [rdx + 0x80]
1407f8c6f: 440f297c2460 movaps xmmword ptr [rsp + 0x60], xmm15
1407f8c75: 440f29742470 movaps xmmword ptr [rsp + 0x70], xmm14
1407f8c7b: 440f29ac2480000000 movaps xmmword ptr [rsp + 0x80], xmm13
1407f8c84: 440f29a42490000000 movaps xmmword ptr [rsp + 0x90], xmm12
1407f8c8d: 440f299c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm11
1407f8c96: 440f299424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm10
1407f8c9f: 440f298c24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm9
1407f8ca8: 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
1407f8cb1: 0f29bc24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm7
1407f8cb9: 0f29b424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm6
1407f8cc1: 80bd1702000000 cmp byte ptr [rbp + 0x217], 0
1407f8cc8: 747f je 0x1407f8d49
1407f8cca: 488b85d8000000 mov rax, qword ptr [rbp + 0xd8]
1407f8cd1: f048ff08 lock dec qword ptr [rax]
1407f8cd5: 750c jne 0x1407f8ce3
1407f8cd7: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f8cde: e83d393600 call 0x140b5c620 ; alloc::sync::Arc::drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
1407f8ce3: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407f8ceb: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407f8cf3: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407f8cfc: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407f8d05: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407f8d0e: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407f8d17: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407f8d20: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407f8d29: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407f8d2f: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407f8d35: 4881c408010000 add rsp, 0x108
1407f8d3c: 5b pop rbx
1407f8d3d: 5f pop rdi
1407f8d3e: 5e pop rsi
1407f8d3f: 415c pop r12
1407f8d41: 415d pop r13
1407f8d43: 415e pop r14
1407f8d45: 415f pop r15
1407f8d47: 5d pop rbp
1407f8d48: c3 ret 
1407f8d49: 0f28b424f0000000 movaps xmm6, xmmword ptr [rsp + 0xf0]
1407f8d51: 0f28bc24e0000000 movaps xmm7, xmmword ptr [rsp + 0xe0]
1407f8d59: 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
1407f8d62: 440f288c24c0000000 movaps xmm9, xmmword ptr [rsp + 0xc0]
1407f8d6b: 440f289424b0000000 movaps xmm10, xmmword ptr [rsp + 0xb0]
1407f8d74: 440f289c24a0000000 movaps xmm11, xmmword ptr [rsp + 0xa0]
1407f8d7d: 440f28a42490000000 movaps xmm12, xmmword ptr [rsp + 0x90]
1407f8d86: 440f28ac2480000000 movaps xmm13, xmmword ptr [rsp + 0x80]
1407f8d8f: 440f28742470 movaps xmm14, xmmword ptr [rsp + 0x70]
1407f8d95: 440f287c2460 movaps xmm15, xmmword ptr [rsp + 0x60]
1407f8d9b: 4881c408010000 add rsp, 0x108
1407f8da2: 5b pop rbx
1407f8da3: 5f pop rdi
1407f8da4: 5e pop rsi
1407f8da5: 415c pop r12
1407f8da7: 415d pop r13
1407f8da9: 415e pop r14
1407f8dab: 415f pop r15
1407f8dad: 5d pop rbp
1407f8dae: c3 ret 
