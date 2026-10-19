14207e1f0: 55 push rbp
14207e1f1: 4157 push r15
14207e1f3: 4156 push r14
14207e1f5: 4155 push r13
14207e1f7: 4154 push r12
14207e1f9: 56 push rsi
14207e1fa: 57 push rdi
14207e1fb: 53 push rbx
14207e1fc: 4881ecb8010000 sub rsp, 0x1b8
14207e203: 488dac2480000000 lea rbp, [rsp + 0x80]
14207e20b: 440f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm11
14207e213: 440f299510010000 movaps xmmword ptr [rbp + 0x110], xmm10
14207e21b: 440f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm9
14207e223: 66440f7f85f0000000 movdqa xmmword ptr [rbp + 0xf0], xmm8
14207e22c: 0f29bde0000000 movaps xmmword ptr [rbp + 0xe0], xmm7
14207e233: 0f29b5d0000000 movaps xmmword ptr [rbp + 0xd0], xmm6
14207e23a: 48c785c8000000feffffff mov qword ptr [rbp + 0xc8], 0xfffffffffffffffe
14207e245: 4c89c6 mov rsi, r8
14207e248: 4889d7 mov rdi, rdx
14207e24b: 4889cb mov rbx, rcx
14207e24e: e8ad87b9fe call 0x140c16a00 ; puffin::are_scopes_on
14207e253: 84c0 test al, al
14207e255: 7460 je 0x14207e2b7
14207e257: 4c8d3dea4df500 lea r15, [rip + 0xf54dea]
14207e25e: 41be35000000 mov r14d, 0x35
14207e264: ba35000000 mov edx, 0x35
14207e269: 4c89f9 mov rcx, r15
14207e26c: e8ff0bffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207e271: 4883f801 cmp rax, 1
14207e275: 0f85b2000000 jne 0x14207e32d
14207e27b: 4989d1 mov r9, rdx
14207e27e: 4885d2 test rdx, rdx
14207e281: 743d je 0x14207e2c0
14207e283: 4983f935 cmp r9, 0x35
14207e287: 7335 jae 0x14207e2be
14207e289: 488d05b84df500 lea rax, [rip + 0xf54db8]
14207e290: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
14207e295: 7f29 jg 0x14207e2c0
14207e297: 488d053a51f500 lea rax, [rip + 0xf5513a]
14207e29e: 4889442420 mov qword ptr [rsp + 0x20], rax
14207e2a3: 488d0d9e4df500 lea rcx, [rip + 0xf54d9e]
14207e2aa: ba35000000 mov edx, 0x35
14207e2af: 4531c0 xor r8d, r8d
14207e2b2: e889b68500 call 0x1428d9940 ; core::str::slice_error_fail
14207e2b7: 31c0 xor eax, eax
14207e2b9: e9c2010000 jmp 0x14207e480 ; unknown
14207e2be: 75d7 jne 0x14207e297
14207e2c0: 4c8d3d814df500 lea r15, [rip + 0xf54d81]
14207e2c7: 4c89f9 mov rcx, r15
14207e2ca: 4c89ca mov rdx, r9
14207e2cd: e89e0bffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207e2d2: 4883f801 cmp rax, 1
14207e2d6: 7555 jne 0x14207e32d
14207e2d8: 4989d0 mov r8, rdx
14207e2db: 4983c002 add r8, 2
14207e2df: 7439 je 0x14207e31a
14207e2e1: 4983f835 cmp r8, 0x35
14207e2e5: 7331 jae 0x14207e318
14207e2e7: 488d055a4df500 lea rax, [rip + 0xf54d5a]
14207e2ee: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
14207e2f3: 7f25 jg 0x14207e31a
14207e2f5: 488d05f450f500 lea rax, [rip + 0xf550f4]
14207e2fc: 4889442420 mov qword ptr [rsp + 0x20], rax
14207e301: 488d0d404df500 lea rcx, [rip + 0xf54d40]
14207e308: ba35000000 mov edx, 0x35
14207e30d: 41b935000000 mov r9d, 0x35
14207e313: e828b68500 call 0x1428d9940 ; core::str::slice_error_fail
14207e318: 75db jne 0x14207e2f5
14207e31a: 41be33000000 mov r14d, 0x33
14207e320: 4929d6 sub r14, rdx
14207e323: 4c8d3d1e4df500 lea r15, [rip + 0xf54d1e]
14207e32a: 4d01c7 add r15, r8
14207e32d: 4c8d052a51f500 lea r8, [rip + 0xf5512a]
14207e334: 4c8d25f550f500 lea r12, [rip + 0xf550f5]
14207e33b: 0f1f440000 nop dword ptr [rax + rax]
14207e340: 410fbe40ff movsx eax, byte ptr [r8 - 1]
14207e345: 85c0 test eax, eax
14207e347: 7817 js 0x14207e360
14207e349: 49ffc8 dec r8
14207e34c: 83f85c cmp eax, 0x5c
14207e34f: 7563 jne 0x14207e3b4
14207e351: eb73 jmp 0x14207e3c6 ; unknown
14207e353: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207e360: 410fb648fe movzx ecx, byte ptr [r8 - 2]
14207e365: 80f9c0 cmp cl, 0xc0
14207e368: 7d23 jge 0x14207e38d
14207e36a: 410fb650fd movzx edx, byte ptr [r8 - 3]
14207e36f: 80fac0 cmp dl, 0xc0
14207e372: 7d22 jge 0x14207e396
14207e374: 450fb648fc movzx r9d, byte ptr [r8 - 4]
14207e379: 4983c0fc add r8, -4
14207e37d: 4183e107 and r9d, 7
14207e381: 41c1e106 shl r9d, 6
14207e385: 83e23f and edx, 0x3f
14207e388: 4409ca or edx, r9d
14207e38b: eb10 jmp 0x14207e39d ; unknown
14207e38d: 4983c0fe add r8, -2
14207e391: 83e11f and ecx, 0x1f
14207e394: eb0f jmp 0x14207e3a5 ; unknown
14207e396: 4983c0fd add r8, -3
14207e39a: 83e20f and edx, 0xf
14207e39d: c1e206 shl edx, 6
14207e3a0: 83e13f and ecx, 0x3f
14207e3a3: 09d1 or ecx, edx
14207e3a5: c1e106 shl ecx, 6
14207e3a8: 243f and al, 0x3f
14207e3aa: 0fb6c0 movzx eax, al
14207e3ad: 09c8 or eax, ecx
14207e3af: 83f85c cmp eax, 0x5c
14207e3b2: 7412 je 0x14207e3c6
14207e3b4: 83f82f cmp eax, 0x2f
14207e3b7: 740d je 0x14207e3c6
14207e3b9: 4d39e0 cmp r8, r12
14207e3bc: 7582 jne 0x14207e340
14207e3be: 41bd2e000000 mov r13d, 0x2e
14207e3c4: eb4d jmp 0x14207e413 ; unknown
14207e3c6: 4c8d256350f500 lea r12, [rip + 0xf55063]
14207e3cd: 4d29e0 sub r8, r12
14207e3d0: 49ffc0 inc r8
14207e3d3: 7432 je 0x14207e407
14207e3d5: 4983f82e cmp r8, 0x2e
14207e3d9: 732a jae 0x14207e405
14207e3db: 43803c20bf cmp byte ptr [r8 + r12], 0xbf
14207e3e0: 7f25 jg 0x14207e407
14207e3e2: 488d05cf4ff500 lea rax, [rip + 0xf54fcf]
14207e3e9: 4889442420 mov qword ptr [rsp + 0x20], rax
14207e3ee: 488d0d3b50f500 lea rcx, [rip + 0xf5503b]
14207e3f5: ba2e000000 mov edx, 0x2e
14207e3fa: 41b92e000000 mov r9d, 0x2e
14207e400: e83bb58500 call 0x1428d9940 ; core::str::slice_error_fail
14207e405: 75db jne 0x14207e3e2
14207e407: 41bd2e000000 mov r13d, 0x2e
14207e40d: 4d29c5 sub r13, r8
14207e410: 4d01c4 add r12, r8
14207e413: e8189db9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207e418: 4889c1 mov rcx, rax
14207e41b: 488b00 mov rax, qword ptr [rax]
14207e41e: 4883f801 cmp rax, 1
14207e422: 0f8571090000 jne 0x14207ed99
14207e428: 4883c108 add rcx, 8
14207e42c: 48833900 cmp qword ptr [rcx], 0
14207e430: 0f85a1090000 jne 0x14207edd7
14207e436: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207e43d: 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
14207e444: 4883c108 add rcx, 8
14207e448: 4c896c2420 mov qword ptr [rsp + 0x20], r13
14207e44d: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
14207e456: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14207e45f: 4c89fa mov rdx, r15
14207e462: 4d89f0 mov r8, r14
14207e465: 4d89e1 mov r9, r12
14207e468: e8d388b9fe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
14207e46d: 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
14207e474: 48ff01 inc qword ptr [rcx]
14207e477: 48894558 mov qword ptr [rbp + 0x58], rax
14207e47b: b801000000 mov eax, 1
14207e480: 48894550 mov qword ptr [rbp + 0x50], rax
14207e484: 488b0e mov rcx, qword ptr [rsi]
14207e487: b217 mov dl, 0x17
14207e489: e8c23791fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
14207e48e: 48895520 mov qword ptr [rbp + 0x20], rdx
14207e492: 48894570 mov qword ptr [rbp + 0x70], rax
14207e496: 488b0b mov rcx, qword ptr [rbx]
14207e499: 488b5308 mov rdx, qword ptr [rbx + 8]
14207e49d: 4c8ba9f8000000 mov r13, qword ptr [rcx + 0xf8]
14207e4a4: 488b8100010000 mov rax, qword ptr [rcx + 0x100]
14207e4ab: 4d8d748500 lea r14, [r13 + rax*4]
14207e4b0: 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
14207e4b7: 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
14207e4be: 4c8b1f mov r11, qword ptr [rdi]
14207e4c1: b908000000 mov ecx, 8
14207e4c6: 48894d68 mov qword ptr [rbp + 0x68], rcx
14207e4ca: 31f6 xor esi, esi
14207e4cc: f30f103530748a00 movss xmm6, dword ptr [rip + 0x8a7430]
14207e4d4: 0f283da5048b00 movaps xmm7, xmmword ptr [rip + 0x8b04a5]
14207e4db: 66450f76c0 pcmpeqd xmm8, xmm8
14207e4e0: 31c9 xor ecx, ecx
14207e4e2: 48c785b000000000000000 mov qword ptr [rbp + 0xb0], 0
14207e4ed: c785c400000000000000 mov dword ptr [rbp + 0xc4], 0
14207e4f7: 48c7859000000000000000 mov qword ptr [rbp + 0x90], 0
14207e502: 48c7859800000000000000 mov qword ptr [rbp + 0x98], 0
14207e50d: 48c7858800000000000000 mov qword ptr [rbp + 0x88], 0
14207e518: 48c785a000000000000000 mov qword ptr [rbp + 0xa0], 0
14207e523: 8885c3000000 mov byte ptr [rbp + 0xc3], al
14207e529: 4c895d60 mov qword ptr [rbp + 0x60], r11
14207e52d: 48895528 mov qword ptr [rbp + 0x28], rdx
14207e531: 84c0 test al, al
14207e533: 0f8547020000 jne 0x14207e780
14207e539: 4839f1 cmp rcx, rsi
14207e53c: 0f8515020000 jne 0x14207e757
14207e542: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207e550: 4d39f5 cmp r13, r14
14207e553: 0f8477070000 je 0x14207ecd0
14207e559: 418b4d00 mov ecx, dword ptr [r13]
14207e55d: 4983c504 add r13, 4
14207e561: 488b8200010000 mov rax, qword ptr [rdx + 0x100]
14207e568: 488d0c89 lea rcx, [rcx + rcx*4]
14207e56c: 48c1e105 shl rcx, 5
14207e570: 488b740858 mov rsi, qword ptr [rax + rcx + 0x58]
14207e575: 4885f6 test rsi, rsi
14207e578: 74d6 je 0x14207e550
14207e57a: 4801c8 add rax, rcx
14207e57d: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
14207e583: 488b92a8010000 mov rdx, qword ptr [rdx + 0x1a8]
14207e58a: 4c8d0cc9 lea r9, [rcx + rcx*8]
14207e58e: 4a8d0cca lea rcx, [rdx + r9*8]
14207e592: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207e599: 4d8b8010010000 mov r8, qword ptr [r8 + 0x110]
14207e5a0: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
14207e5a5: 4c39c2 cmp rdx, r8
14207e5a8: 763d jbe 0x14207e5e7
14207e5aa: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207e5ae: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207e5b2: 4d85c0 test r8, r8
14207e5b5: 7430 je 0x14207e5e7
14207e5b7: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207e5bb: 49f7d0 not r8
14207e5be: 4f8d0440 lea r8, [r8 + r8*2]
14207e5c2: 49c1e004 shl r8, 4
14207e5c6: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14207e5cb: 4c898590000000 mov qword ptr [rbp + 0x90], r8
14207e5d2: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207e5d9: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
14207e5e0: 4c39c2 cmp rdx, r8
14207e5e3: 7720 ja 0x14207e605
14207e5e5: eb48 jmp 0x14207e62f ; unknown
14207e5e7: 48c7859000000000000000 mov qword ptr [rbp + 0x90], 0
14207e5f2: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207e5f9: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
14207e600: 4c39c2 cmp rdx, r8
14207e603: 762a jbe 0x14207e62f
14207e605: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207e609: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207e60d: 4d85c0 test r8, r8
14207e610: 741d je 0x14207e62f
14207e612: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207e616: 49f7d0 not r8
14207e619: 4f8d0440 lea r8, [r8 + r8*2]
14207e61d: 49c1e004 shl r8, 4
14207e621: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14207e626: 4c898598000000 mov qword ptr [rbp + 0x98], r8
14207e62d: eb0b jmp 0x14207e63a ; unknown
14207e62f: 48c7859800000000000000 mov qword ptr [rbp + 0x98], 0
14207e63a: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207e641: 4d8b8820010000 mov r9, qword ptr [r8 + 0x120]
14207e648: 4c8b8088000000 mov r8, qword ptr [rax + 0x88]
14207e64f: 4d39c8 cmp r8, r9
14207e652: 765d jbe 0x14207e6b1
14207e654: 4c8b9080000000 mov r10, qword ptr [rax + 0x80]
14207e65b: 4b833cca00 cmp qword ptr [r10 + r9*8], 0
14207e660: 744f je 0x14207e6b1
14207e662: 41b201 mov r10b, 1
14207e665: 448995c4000000 mov dword ptr [rbp + 0xc4], r10d
14207e66c: 4c39ca cmp rdx, r9
14207e66f: 765f jbe 0x14207e6d0
14207e671: 4c8b5138 mov r10, qword ptr [rcx + 0x38]
14207e675: 4f8b0cca mov r9, qword ptr [r10 + r9*8]
14207e679: 4d85c9 test r9, r9
14207e67c: 7452 je 0x14207e6d0
14207e67e: 4c8b5118 mov r10, qword ptr [rcx + 0x18]
14207e682: 49f7d1 not r9
14207e685: 4f8d0c49 lea r9, [r9 + r9*2]
14207e689: 49c1e104 shl r9, 4
14207e68d: 4f8b4c0a10 mov r9, qword ptr [r10 + r9 + 0x10]
14207e692: 4c898db0000000 mov qword ptr [rbp + 0xb0], r9
14207e699: 4c8b8da8000000 mov r9, qword ptr [rbp + 0xa8]
14207e6a0: 4d8b8928010000 mov r9, qword ptr [r9 + 0x128]
14207e6a7: 4d39c8 cmp r8, r9
14207e6aa: 7742 ja 0x14207e6ee
14207e6ac: e984000000 jmp 0x14207e735 ; unknown
14207e6b1: c785c400000000000000 mov dword ptr [rbp + 0xc4], 0
14207e6bb: 4c8b8da8000000 mov r9, qword ptr [rbp + 0xa8]
14207e6c2: 4d8b8928010000 mov r9, qword ptr [r9 + 0x128]
14207e6c9: 4d39c8 cmp r8, r9
14207e6cc: 7720 ja 0x14207e6ee
14207e6ce: eb65 jmp 0x14207e735 ; unknown
14207e6d0: 48c785b000000000000000 mov qword ptr [rbp + 0xb0], 0
14207e6db: 4c8b8da8000000 mov r9, qword ptr [rbp + 0xa8]
14207e6e2: 4d8b8928010000 mov r9, qword ptr [r9 + 0x128]
14207e6e9: 4d39c8 cmp r8, r9
14207e6ec: 7647 jbe 0x14207e735
14207e6ee: 4c8b8080000000 mov r8, qword ptr [rax + 0x80]
14207e6f5: 4b833cc800 cmp qword ptr [r8 + r9*8], 0
14207e6fa: 7439 je 0x14207e735
14207e6fc: 41b001 mov r8b, 1
14207e6ff: 4c8985a0000000 mov qword ptr [rbp + 0xa0], r8
14207e706: 4c39ca cmp rdx, r9
14207e709: 7637 jbe 0x14207e742
14207e70b: 488b5138 mov rdx, qword ptr [rcx + 0x38]
14207e70f: 4a8b14ca mov rdx, qword ptr [rdx + r9*8]
14207e713: 4885d2 test rdx, rdx
14207e716: 742a je 0x14207e742
14207e718: 488b4918 mov rcx, qword ptr [rcx + 0x18]
14207e71c: 48f7d2 not rdx
14207e71f: 488d1452 lea rdx, [rdx + rdx*2]
14207e723: 48c1e204 shl rdx, 4
14207e727: 488b4c1110 mov rcx, qword ptr [rcx + rdx + 0x10]
14207e72c: 48898d88000000 mov qword ptr [rbp + 0x88], rcx
14207e733: eb18 jmp 0x14207e74d ; unknown
14207e735: 48c785a000000000000000 mov qword ptr [rbp + 0xa0], 0
14207e740: eb0b jmp 0x14207e74d ; unknown
14207e742: 48c7858800000000000000 mov qword ptr [rbp + 0x88], 0
14207e74d: 488b4050 mov rax, qword ptr [rax + 0x50]
14207e751: 48894568 mov qword ptr [rbp + 0x68], rax
14207e755: 31c9 xor ecx, ecx
14207e757: 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
14207e75e: 4889c8 mov rax, rcx
14207e761: 48c1e004 shl rax, 4
14207e765: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
14207e769: 8b440108 mov eax, dword ptr [rcx + rax + 8]
14207e76d: e9b7000000 jmp 0x14207e829 ; unknown
14207e772: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207e780: 4839f1 cmp rcx, rsi
14207e783: 0f8597000000 jne 0x14207e820
14207e789: 0f1f8000000000 nop dword ptr [rax]
14207e790: 4d39f5 cmp r13, r14
14207e793: 0f8437050000 je 0x14207ecd0
14207e799: 418b4d00 mov ecx, dword ptr [r13]
14207e79d: 4983c504 add r13, 4
14207e7a1: 488b82a8010000 mov rax, qword ptr [rdx + 0x1a8]
14207e7a8: 488d0cc9 lea rcx, [rcx + rcx*8]
14207e7ac: 488b74c810 mov rsi, qword ptr [rax + rcx*8 + 0x10]
14207e7b1: 4885f6 test rsi, rsi
14207e7b4: 74da je 0x14207e790
14207e7b6: 488d04c8 lea rax, [rax + rcx*8]
14207e7ba: 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
14207e7c1: 488b9110010000 mov rdx, qword ptr [rcx + 0x110]
14207e7c8: 488b4840 mov rcx, qword ptr [rax + 0x40]
14207e7cc: 4839d1 cmp rcx, rdx
14207e7cf: 0f86be010000 jbe 0x14207e993
14207e7d5: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207e7d9: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207e7dd: 4885d2 test rdx, rdx
14207e7e0: 0f84ad010000 je 0x14207e993
14207e7e6: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14207e7ea: 48f7d2 not rdx
14207e7ed: 488d1452 lea rdx, [rdx + rdx*2]
14207e7f1: 48c1e204 shl rdx, 4
14207e7f5: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
14207e7fa: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207e801: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
14207e808: 4c39c1 cmp rcx, r8
14207e80b: 0f8797010000 ja 0x14207e9a8
14207e811: e9c8010000 jmp 0x14207e9de ; unknown
14207e816: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207e820: 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
14207e827: 89c8 mov eax, ecx
14207e829: 488d1c40 lea rbx, [rax + rax*2]
14207e82d: 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
14207e834: 488d1499 lea rdx, [rcx + rbx*4]
14207e838: 48c1e304 shl rbx, 4
14207e83c: 48039d98000000 add rbx, qword ptr [rbp + 0x98]
14207e843: 31c9 xor ecx, ecx
14207e845: f685c400000001 test byte ptr [rbp + 0xc4], 1
14207e84c: 41b900000000 mov r9d, 0
14207e852: 4c0f458db0000000 cmovne r9, qword ptr [rbp + 0xb0]
14207e85a: f685a000000001 test byte ptr [rbp + 0xa0], 1
14207e861: 0f84b9020000 je 0x14207eb20
14207e867: 488b8d88000000 mov rcx, qword ptr [rbp + 0x88]
14207e86e: 488d04c1 lea rax, [rcx + rax*8]
14207e872: 4885c0 test rax, rax
14207e875: 0f8465020000 je 0x14207eae0
14207e87b: 488b00 mov rax, qword ptr [rax]
14207e87e: 488945c0 mov qword ptr [rbp - 0x40], rax
14207e882: 49837b4800 cmp qword ptr [r11 + 0x48], 0
14207e887: 0f84fe040000 je 0x14207ed8b
14207e88d: 4d8b6330 mov r12, qword ptr [r11 + 0x30]
14207e891: 4d8b4338 mov r8, qword ptr [r11 + 0x38]
14207e895: 4c89c7 mov rdi, r8
14207e898: 4821c7 and rdi, rax
14207e89b: 48c1e839 shr rax, 0x39
14207e89f: 660f6ec0 movd xmm0, eax
14207e8a3: 660f60c0 punpcklbw xmm0, xmm0
14207e8a7: f20f70c000 pshuflw xmm0, xmm0, 0
14207e8ac: 66440f70c800 pshufd xmm9, xmm0, 0
14207e8b2: 4d8d5424e8 lea r10, [r12 - 0x18]
14207e8b7: 4531db xor r11d, r11d
14207e8ba: f3450f6f143c movdqu xmm10, xmmword ptr [r12 + rdi]
14207e8c0: 66410f6fc2 movdqa xmm0, xmm10
14207e8c5: 66410f74c1 pcmpeqb xmm0, xmm9
14207e8ca: 660fd7c0 pmovmskb eax, xmm0
14207e8ce: 85c0 test eax, eax
14207e8d0: 746e je 0x14207e940
14207e8d2: 48895578 mov qword ptr [rbp + 0x78], rdx
14207e8d6: 4c898d80000000 mov qword ptr [rbp + 0x80], r9
14207e8dd: 4c894538 mov qword ptr [rbp + 0x38], r8
14207e8e1: 4c895540 mov qword ptr [rbp + 0x40], r10
14207e8e5: 4c895d48 mov qword ptr [rbp + 0x48], r11
14207e8e9: 48894530 mov qword ptr [rbp + 0x30], rax
14207e8ed: f30fbcc0 tzcnt eax, eax
14207e8f1: 4801f8 add rax, rdi
14207e8f4: 4c21c0 and rax, r8
14207e8f7: 48f7d8 neg rax
14207e8fa: 4c8d3c40 lea r15, [rax + rax*2]
14207e8fe: 4b8d14fa lea rdx, [r10 + r15*8]
14207e902: 488d4dc0 lea rcx, [rbp - 0x40]
14207e906: e865e4c1fe call 0x140c9cd70 ; utils::name_hash::impl$2::eq
14207e90b: 84c0 test al, al
14207e90d: 7561 jne 0x14207e970
14207e90f: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
14207e913: 8d41ff lea eax, [rcx - 1]
14207e916: 6621c8 and ax, cx
14207e919: 488b5578 mov rdx, qword ptr [rbp + 0x78]
14207e91d: 4c8b8d80000000 mov r9, qword ptr [rbp + 0x80]
14207e924: 4c8b4538 mov r8, qword ptr [rbp + 0x38]
14207e928: 4c8b5540 mov r10, qword ptr [rbp + 0x40]
14207e92c: 4c8b5d48 mov r11, qword ptr [rbp + 0x48]
14207e930: 75b7 jne 0x14207e8e9
14207e932: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207e940: 66450f74d0 pcmpeqb xmm10, xmm8
14207e945: 66410fd7c2 pmovmskb eax, xmm10
14207e94a: 85c0 test eax, eax
14207e94c: 0f8539040000 jne 0x14207ed8b
14207e952: 4c01df add rdi, r11
14207e955: 4883c710 add rdi, 0x10
14207e959: 4983c310 add r11, 0x10
14207e95d: 4c21c7 and rdi, r8
14207e960: e955ffffff jmp 0x14207e8ba ; unknown
14207e965: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207e970: 4b8b54fcf0 mov rdx, qword ptr [r12 + r15*8 - 0x10]
14207e975: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
14207e979: e892e98bfe call 0x14093d310 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index
14207e97e: b901000000 mov ecx, 1
14207e983: 488b5578 mov rdx, qword ptr [rbp + 0x78]
14207e987: 4c8b8d80000000 mov r9, qword ptr [rbp + 0x80]
14207e98e: e98f010000 jmp 0x14207eb22 ; unknown
14207e993: 31d2 xor edx, edx
14207e995: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207e99c: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
14207e9a3: 4c39c1 cmp rcx, r8
14207e9a6: 7636 jbe 0x14207e9de
14207e9a8: 4c8b4838 mov r9, qword ptr [rax + 0x38]
14207e9ac: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207e9b0: 4d85c0 test r8, r8
14207e9b3: 7429 je 0x14207e9de
14207e9b5: 4c8b4818 mov r9, qword ptr [rax + 0x18]
14207e9b9: 49f7d0 not r8
14207e9bc: 4f8d0440 lea r8, [r8 + r8*2]
14207e9c0: 49c1e004 shl r8, 4
14207e9c4: 4b8b5c0110 mov rbx, qword ptr [r9 + r8 + 0x10]
14207e9c9: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207e9d0: 4d8b8020010000 mov r8, qword ptr [r8 + 0x120]
14207e9d7: 4c39c1 cmp rcx, r8
14207e9da: 7662 jbe 0x14207ea3e
14207e9dc: eb15 jmp 0x14207e9f3 ; unknown
14207e9de: 31db xor ebx, ebx
14207e9e0: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207e9e7: 4d8b8020010000 mov r8, qword ptr [r8 + 0x120]
14207e9ee: 4c39c1 cmp rcx, r8
14207e9f1: 764b jbe 0x14207ea3e
14207e9f3: 4c8b4838 mov r9, qword ptr [rax + 0x38]
14207e9f7: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207e9fb: 4d85c0 test r8, r8
14207e9fe: 743e je 0x14207ea3e
14207ea00: 4c8b4818 mov r9, qword ptr [rax + 0x18]
14207ea04: 49f7d0 not r8
14207ea07: 4f8d0440 lea r8, [r8 + r8*2]
14207ea0b: 49c1e004 shl r8, 4
14207ea0f: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14207ea14: 4c8985b0000000 mov qword ptr [rbp + 0xb0], r8
14207ea1b: 41b001 mov r8b, 1
14207ea1e: 448985c4000000 mov dword ptr [rbp + 0xc4], r8d
14207ea25: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207ea2c: 4d8b8028010000 mov r8, qword ptr [r8 + 0x128]
14207ea33: 4c39c1 cmp rcx, r8
14207ea36: 0f86a8000000 jbe 0x14207eae4
14207ea3c: eb21 jmp 0x14207ea5f ; unknown
14207ea3e: c785c400000000000000 mov dword ptr [rbp + 0xc4], 0
14207ea48: 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
14207ea4f: 4d8b8028010000 mov r8, qword ptr [r8 + 0x128]
14207ea56: 4c39c1 cmp rcx, r8
14207ea59: 0f8685000000 jbe 0x14207eae4
14207ea5f: 488b4838 mov rcx, qword ptr [rax + 0x38]
14207ea63: 4a8b0cc1 mov rcx, qword ptr [rcx + r8*8]
14207ea67: 4885c9 test rcx, rcx
14207ea6a: 7478 je 0x14207eae4
14207ea6c: 488b4018 mov rax, qword ptr [rax + 0x18]
14207ea70: 48f7d1 not rcx
14207ea73: 488d0c49 lea rcx, [rcx + rcx*2]
14207ea77: 48c1e104 shl rcx, 4
14207ea7b: 488b440810 mov rax, qword ptr [rax + rcx + 0x10]
14207ea80: 48898588000000 mov qword ptr [rbp + 0x88], rax
14207ea87: 80bdc400000000 cmp byte ptr [rbp + 0xc4], 0
14207ea8e: 41b900000000 mov r9d, 0
14207ea94: 4c0f458db0000000 cmovne r9, qword ptr [rbp + 0xb0]
14207ea9c: b001 mov al, 1
14207ea9e: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
14207eaa5: 31c0 xor eax, eax
14207eaa7: 48899d98000000 mov qword ptr [rbp + 0x98], rbx
14207eaae: 48899590000000 mov qword ptr [rbp + 0x90], rdx
14207eab5: 48c785b800000000000000 mov qword ptr [rbp + 0xb8], 0
14207eac0: 488b8d88000000 mov rcx, qword ptr [rbp + 0x88]
14207eac7: 488d04c1 lea rax, [rcx + rax*8]
14207eacb: 4885c0 test rax, rax
14207eace: 0f85a7fdffff jne 0x14207e87b
14207ead4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207eae0: 31c9 xor ecx, ecx
14207eae2: eb3c jmp 0x14207eb20 ; unknown
14207eae4: 31c9 xor ecx, ecx
14207eae6: 80bdc400000000 cmp byte ptr [rbp + 0xc4], 0
14207eaed: 41b900000000 mov r9d, 0
14207eaf3: 4c0f458db0000000 cmovne r9, qword ptr [rbp + 0xb0]
14207eafb: 48c785b800000000000000 mov qword ptr [rbp + 0xb8], 0
14207eb06: 48c785a000000000000000 mov qword ptr [rbp + 0xa0], 0
14207eb11: 48899d98000000 mov qword ptr [rbp + 0x98], rbx
14207eb18: 48899590000000 mov qword ptr [rbp + 0x90], rdx
14207eb1f: 90 nop 
14207eb20: 31c0 xor eax, eax
14207eb22: 448b4318 mov r8d, dword ptr [rbx + 0x18]
14207eb26: 44894518 mov dword ptr [rbp + 0x18], r8d
14207eb2a: 4c8b4310 mov r8, qword ptr [rbx + 0x10]
14207eb2e: 4c894510 mov qword ptr [rbp + 0x10], r8
14207eb32: 0f2823 movaps xmm4, xmmword ptr [rbx]
14207eb35: f30f105b08 movss xmm3, dword ptr [rbx + 8]
14207eb3a: 440f28cb movaps xmm9, xmm3
14207eb3e: f3440f58cb addss xmm9, xmm3
14207eb43: f3410f59d9 mulss xmm3, xmm9
14207eb48: 0f28cc movaps xmm1, xmm4
14207eb4b: 0f58cc addps xmm1, xmm4
14207eb4e: f30f16c1 movshdup xmm0, xmm1
14207eb52: f30f59c4 mulss xmm0, xmm4
14207eb56: 0f28d4 movaps xmm2, xmm4
14207eb59: f3410f12e9 movsldup xmm5, xmm9
14207eb5e: 0f59ec mulps xmm5, xmm4
14207eb61: 0fc6e4ff shufps xmm4, xmm4, 0xff
14207eb65: 0f59d1 mulps xmm2, xmm1
14207eb68: 0f59cc mulps xmm1, xmm4
14207eb6b: 440f28d4 movaps xmm10, xmm4
14207eb6f: f3450f59d1 mulss xmm10, xmm9
14207eb74: f3440f16da movshdup xmm11, xmm2
14207eb79: 450f28cb movaps xmm9, xmm11
14207eb7d: f3440f58cb addss xmm9, xmm3
14207eb82: 0f28e6 movaps xmm4, xmm6
14207eb85: f3410f5ce1 subss xmm4, xmm9
14207eb8a: 440f28c8 movaps xmm9, xmm0
14207eb8e: f3450f58ca addss xmm9, xmm10
14207eb93: 410f14e1 unpcklps xmm4, xmm9
14207eb97: f3410f5cc2 subss xmm0, xmm10
14207eb9c: f30f58da addss xmm3, xmm2
14207eba0: 440f28ce movaps xmm9, xmm6
14207eba4: f3440f5ccb subss xmm9, xmm3
14207eba9: 0fc6c9e1 shufps xmm1, xmm1, 0xe1
14207ebad: 410f14c1 unpcklps xmm0, xmm9
14207ebb1: 440f28cd movaps xmm9, xmm5
14207ebb5: 440f5cc9 subps xmm9, xmm1
14207ebb9: 0f58cd addps xmm1, xmm5
14207ebbc: f30f105b20 movss xmm3, dword ptr [rbx + 0x20]
14207ebc1: f3410f58d3 addss xmm2, xmm11
14207ebc6: 440f28d6 movaps xmm10, xmm6
14207ebca: f3440f5cd2 subss xmm10, xmm2
14207ebcf: f30f106b24 movss xmm5, dword ptr [rbx + 0x24]
14207ebd4: 410f16e1 movlhps xmm4, xmm9
14207ebd8: 0fc6c1d4 shufps xmm0, xmm1, 0xd4
14207ebdc: 440fc6c901 shufps xmm9, xmm1, 1
14207ebe1: 440fc6d130 shufps xmm10, xmm1, 0x30
14207ebe6: 0f10531c movups xmm2, xmmword ptr [rbx + 0x1c]
14207ebea: 450fc6ca82 shufps xmm9, xmm10, 0x82
14207ebef: 0f104b20 movups xmm1, xmmword ptr [rbx + 0x20]
14207ebf3: 0fc6d200 shufps xmm2, xmm2, 0
14207ebf7: 0f59d4 mulps xmm2, xmm4
14207ebfa: 0f28e1 movaps xmm4, xmm1
14207ebfd: 0fc6e100 shufps xmm4, xmm1, 0
14207ec01: 0f59e0 mulps xmm4, xmm0
14207ec04: 0fc6c955 shufps xmm1, xmm1, 0x55
14207ec08: 410f59c9 mulps xmm1, xmm9
14207ec0c: 0f54df andps xmm3, xmm7
14207ec0f: 0f54ef andps xmm5, xmm7
14207ec12: 0f28c3 movaps xmm0, xmm3
14207ec15: f30fc2c303 cmpunordss xmm0, xmm3
14207ec1a: 440f28c8 movaps xmm9, xmm0
14207ec1e: 440f54cd andps xmm9, xmm5
14207ec22: f30f5feb maxss xmm5, xmm3
14207ec26: f30f105b1c movss xmm3, dword ptr [rbx + 0x1c]
14207ec2b: 0f54df andps xmm3, xmm7
14207ec2e: 0f55c5 andnps xmm0, xmm5
14207ec31: 410f56c1 orps xmm0, xmm9
14207ec35: 0f28e8 movaps xmm5, xmm0
14207ec38: f30f5feb maxss xmm5, xmm3
14207ec3c: f30fc2db03 cmpunordss xmm3, xmm3
14207ec41: 440f28cb movaps xmm9, xmm3
14207ec45: 440f54c8 andps xmm9, xmm0
14207ec49: 0f55dd andnps xmm3, xmm5
14207ec4c: 410f56d9 orps xmm3, xmm9
14207ec50: 0f28c2 movaps xmm0, xmm2
14207ec53: f20f10c4 movsd xmm0, xmm4
14207ec57: 0fc6d024 shufps xmm2, xmm0, 0x24
14207ec5b: 0f284310 movaps xmm0, xmmword ptr [rbx + 0x10]
14207ec5f: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
14207ec63: 0fc6e149 shufps xmm4, xmm1, 0x49
14207ec67: 0f2965d0 movaps xmmword ptr [rbp - 0x30], xmm4
14207ec6b: 0fc6c802 shufps xmm1, xmm0, 2
14207ec6f: 0fc6c898 shufps xmm1, xmm0, 0x98
14207ec73: 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
14207ec77: 4531c0 xor r8d, r8d
14207ec7a: 4d85c9 test r9, r9
14207ec7d: 410f95c0 setne r8b
14207ec81: 428d4c4104 lea ecx, [rcx + r8*2 + 4]
14207ec86: 894df0 mov dword ptr [rbp - 0x10], ecx
14207ec89: 8945f4 mov dword ptr [rbp - 0xc], eax
14207ec8c: 48c745f800000000 mov qword ptr [rbp - 8], 0
14207ec94: 488d45c0 lea rax, [rbp - 0x40]
14207ec98: 4889442420 mov qword ptr [rsp + 0x20], rax
14207ec9d: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
14207eca1: 4c8d4510 lea r8, [rbp + 0x10]
14207eca5: e896ee4eff call 0x14156db40 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::UberInstanceData>
14207ecaa: 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
14207ecb1: 48ffc1 inc rcx
14207ecb4: 0fb685c3000000 movzx eax, byte ptr [rbp + 0xc3]
14207ecbb: 4c8b5d60 mov r11, qword ptr [rbp + 0x60]
14207ecbf: 488b5528 mov rdx, qword ptr [rbp + 0x28]
14207ecc3: 84c0 test al, al
14207ecc5: 0f846ef8ffff je 0x14207e539
14207eccb: e9b0faffff jmp 0x14207e780 ; unknown
14207ecd0: 488b7570 mov rsi, qword ptr [rbp + 0x70]
14207ecd4: 4889f1 mov rcx, rsi
14207ecd7: e83431c0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207ecdc: 4889f1 mov rcx, rsi
14207ecdf: 4883c901 or rcx, 1
14207ece3: e82831c0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207ece8: 31c9 xor ecx, ecx
14207ecea: b808000000 mov eax, 8
14207ecef: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
14207ecf4: 0f85e9000000 jne 0x14207ede3
14207ecfa: 48837d5000 cmp qword ptr [rbp + 0x50], 0
14207ecff: 7448 je 0x14207ed49
14207ed01: e82a94b9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207ed06: 4889c1 mov rcx, rax
14207ed09: 488b00 mov rax, qword ptr [rax]
14207ed0c: 4883f801 cmp rax, 1
14207ed10: 0f859d000000 jne 0x14207edb3
14207ed16: 4883c108 add rcx, 8
14207ed1a: 48833900 cmp qword ptr [rcx], 0
14207ed1e: 0f85b3000000 jne 0x14207edd7
14207ed24: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207ed2b: 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
14207ed32: 4883c108 add rcx, 8
14207ed36: 488b5558 mov rdx, qword ptr [rbp + 0x58]
14207ed3a: e8b183b9fe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
14207ed3f: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
14207ed46: 48ff00 inc qword ptr [rax]
14207ed49: 0f28b5d0000000 movaps xmm6, xmmword ptr [rbp + 0xd0]
14207ed50: 0f28bde0000000 movaps xmm7, xmmword ptr [rbp + 0xe0]
14207ed57: 440f2885f0000000 movaps xmm8, xmmword ptr [rbp + 0xf0]
14207ed5f: 440f288d00010000 movaps xmm9, xmmword ptr [rbp + 0x100]
14207ed67: 440f289510010000 movaps xmm10, xmmword ptr [rbp + 0x110]
14207ed6f: 440f289d20010000 movaps xmm11, xmmword ptr [rbp + 0x120]
14207ed77: 4881c4b8010000 add rsp, 0x1b8
14207ed7e: 5b pop rbx
14207ed7f: 5f pop rdi
14207ed80: 5e pop rsi
14207ed81: 415c pop r12
14207ed83: 415d pop r13
14207ed85: 415e pop r14
14207ed87: 415f pop r15
14207ed89: 5d pop rbp
14207ed8a: c3 ret 
14207ed8b: 488d0d5649f500 lea rcx, [rip + 0xf54956]
14207ed92: e879a58500 call 0x1428d9310 ; core::option::unwrap_failed
14207ed97: 0f0b ud2 
14207ed99: 4885c0 test rax, rax
14207ed9c: 752d jne 0x14207edcb
14207ed9e: 31d2 xor edx, edx
14207eda0: e82b9a8700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14207eda5: 4889c1 mov rcx, rax
14207eda8: 4885c0 test rax, rax
14207edab: 0f857bf6ffff jne 0x14207e42c
14207edb1: eb18 jmp 0x14207edcb ; unknown
14207edb3: 4885c0 test rax, rax
14207edb6: 7513 jne 0x14207edcb
14207edb8: 31d2 xor edx, edx
14207edba: e8119a8700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14207edbf: 4889c1 mov rcx, rax
14207edc2: 4885c0 test rax, rax
14207edc5: 0f854fffffff jne 0x14207ed1a
14207edcb: 488d0dcebcbb00 lea rcx, [rip + 0xbbbcce]
14207edd2: e8f9928500 call 0x1428d80d0 ; std::thread::local::panic_access_error
14207edd7: 488d0d92bdbb00 lea rcx, [rip + 0xbbbd92]
14207edde: e86da48500 call 0x1428d9250 ; core::cell::panic_already_borrowed
14207ede3: 4889f1 mov rcx, rsi
14207ede6: 31d2 xor edx, edx
14207ede8: e873d18400 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
14207eded: e908ffffff jmp 0x14207ecfa ; unknown
14207edf2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207ee00: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207ee05: 55 push rbp
14207ee06: 4157 push r15
14207ee08: 4156 push r14
14207ee0a: 4155 push r13
14207ee0c: 4154 push r12
14207ee0e: 56 push rsi
14207ee0f: 57 push rdi
14207ee10: 53 push rbx
14207ee11: 4881ec98000000 sub rsp, 0x98
14207ee18: 488daa80000000 lea rbp, [rdx + 0x80]
14207ee1f: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
14207ee25: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14207ee2b: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
14207ee31: 66440f7f442460 movdqa xmmword ptr [rsp + 0x60], xmm8
14207ee38: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
14207ee3d: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
14207ee45: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
14207ee4c: 48ff00 inc qword ptr [rax]
14207ee4f: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14207ee57: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14207ee5c: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14207ee62: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14207ee68: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
14207ee6e: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14207ee74: 4881c498000000 add rsp, 0x98
14207ee7b: 5b pop rbx
14207ee7c: 5f pop rdi
14207ee7d: 5e pop rsi
14207ee7e: 415c pop r12
14207ee80: 415d pop r13
14207ee82: 415e pop r14
14207ee84: 415f pop r15
14207ee86: 5d pop rbp
14207ee87: c3 ret 
14207ee88: 0f1f840000000000 nop dword ptr [rax + rax]
14207ee90: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207ee95: 55 push rbp
14207ee96: 4157 push r15
14207ee98: 4156 push r14
14207ee9a: 4155 push r13
14207ee9c: 4154 push r12
14207ee9e: 56 push rsi
14207ee9f: 57 push rdi
14207eea0: 53 push rbx
14207eea1: 4881ec98000000 sub rsp, 0x98
14207eea8: 488daa80000000 lea rbp, [rdx + 0x80]
14207eeaf: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
14207eeb5: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14207eebb: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
14207eec1: 66440f7f442460 movdqa xmmword ptr [rsp + 0x60], xmm8
14207eec8: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
14207eecd: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
14207eed5: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
14207eedc: 48ff00 inc qword ptr [rax]
14207eedf: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14207eee7: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14207eeec: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14207eef2: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14207eef8: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
14207eefe: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14207ef04: 4881c498000000 add rsp, 0x98
14207ef0b: 5b pop rbx
14207ef0c: 5f pop rdi
14207ef0d: 5e pop rsi
14207ef0e: 415c pop r12
14207ef10: 415d pop r13
14207ef12: 415e pop r14
14207ef14: 415f pop r15
14207ef16: 5d pop rbp
14207ef17: c3 ret 
14207ef18: 0f1f840000000000 nop dword ptr [rax + rax]
14207ef20: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207ef25: 55 push rbp
14207ef26: 4157 push r15
14207ef28: 4156 push r14
14207ef2a: 4155 push r13
14207ef2c: 4154 push r12
14207ef2e: 56 push rsi
14207ef2f: 57 push rdi
14207ef30: 53 push rbx
14207ef31: 4881ec98000000 sub rsp, 0x98
14207ef38: 488daa80000000 lea rbp, [rdx + 0x80]
14207ef3f: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
14207ef45: 66440f7f542440 movdqa xmmword ptr [rsp + 0x40], xmm10
14207ef4c: 66440f7f4c2450 movdqa xmmword ptr [rsp + 0x50], xmm9
14207ef53: 66440f7f442460 movdqa xmmword ptr [rsp + 0x60], xmm8
14207ef5a: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
14207ef5f: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
14207ef67: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
14207ef6b: e86023fcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
14207ef70: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14207ef78: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14207ef7d: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14207ef83: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14207ef89: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
14207ef8f: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14207ef95: 4881c498000000 add rsp, 0x98
14207ef9c: 5b pop rbx
14207ef9d: 5f pop rdi
14207ef9e: 5e pop rsi
14207ef9f: 415c pop r12
14207efa1: 415d pop r13
14207efa3: 415e pop r14
14207efa5: 415f pop r15
14207efa7: 5d pop rbp
14207efa8: c3 ret 
14207efa9: 0f1f8000000000 nop dword ptr [rax]
14207efb0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207efb5: 55 push rbp
14207efb6: 4157 push r15
14207efb8: 4156 push r14
14207efba: 4155 push r13
14207efbc: 4154 push r12
14207efbe: 56 push rsi
14207efbf: 57 push rdi
14207efc0: 53 push rbx
14207efc1: 4881ec98000000 sub rsp, 0x98
14207efc8: 488daa80000000 lea rbp, [rdx + 0x80]
14207efcf: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
14207efd5: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14207efdb: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
14207efe1: 66440f7f442460 movdqa xmmword ptr [rsp + 0x60], xmm8
14207efe8: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
14207efed: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
14207eff5: 48837d5000 cmp qword ptr [rbp + 0x50], 0
14207effa: 741f je 0x14207f01b
14207effc: 488d4558 lea rax, [rbp + 0x58]
14207f000: 48894508 mov qword ptr [rbp + 8], rax
14207f004: 488d4508 lea rax, [rbp + 8]
14207f008: 48894500 mov qword ptr [rbp], rax
14207f00c: 488d0d4543f500 lea rcx, [rip + 0xf54345]
14207f013: 4889ea mov rdx, rbp
14207f016: e8b5022bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
14207f01b: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14207f023: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14207f028: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14207f02e: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14207f034: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
14207f03a: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14207f040: 4881c498000000 add rsp, 0x98
14207f047: 5b pop rbx
14207f048: 5f pop rdi
14207f049: 5e pop rsi
14207f04a: 415c pop r12
14207f04c: 415d pop r13
14207f04e: 415e pop r14
14207f050: 415f pop r15
14207f052: 5d pop rbp
14207f053: c3 ret 
