142072010: 55 push rbp
142072011: 4157 push r15
142072013: 4156 push r14
142072015: 4155 push r13
142072017: 4154 push r12
142072019: 56 push rsi
14207201a: 57 push rdi
14207201b: 53 push rbx
14207201c: 4881ec88010000 sub rsp, 0x188
142072023: 488dac2480000000 lea rbp, [rsp + 0x80]
14207202b: 440f29b5f0000000 movaps xmmword ptr [rbp + 0xf0], xmm14
142072033: 440f29ade0000000 movaps xmmword ptr [rbp + 0xe0], xmm13
14207203b: 440f29a5d0000000 movaps xmmword ptr [rbp + 0xd0], xmm12
142072043: 440f299dc0000000 movaps xmmword ptr [rbp + 0xc0], xmm11
14207204b: 440f2995b0000000 movaps xmmword ptr [rbp + 0xb0], xmm10
142072053: 440f298da0000000 movaps xmmword ptr [rbp + 0xa0], xmm9
14207205b: 440f298590000000 movaps xmmword ptr [rbp + 0x90], xmm8
142072063: 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
14207206a: 0f297570 movaps xmmword ptr [rbp + 0x70], xmm6
14207206e: 48c74568feffffff mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
142072076: 4889d6 mov rsi, rdx
142072079: 4889cf mov rdi, rcx
14207207c: e87f49bafe call 0x140c16a00 ; puffin::are_scopes_on
142072081: 84c0 test al, al
142072083: 745f je 0x1420720e4
142072085: 4c8d352c10f600 lea r14, [rip + 0xf6102c]
14207208c: bb41000000 mov ebx, 0x41
142072091: ba41000000 mov edx, 0x41
142072096: 4c89f1 mov rcx, r14
142072099: e8d2cdffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207209e: 4883f801 cmp rax, 1
1420720a2: 0f85b1000000 jne 0x142072159
1420720a8: 4989d1 mov r9, rdx
1420720ab: 4885d2 test rdx, rdx
1420720ae: 743d je 0x1420720ed
1420720b0: 4983f941 cmp r9, 0x41
1420720b4: 7335 jae 0x1420720eb
1420720b6: 488d05fb0ff600 lea rax, [rip + 0xf60ffb]
1420720bd: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1420720c2: 7f29 jg 0x1420720ed
1420720c4: 488d050d13f600 lea rax, [rip + 0xf6130d]
1420720cb: 4889442420 mov qword ptr [rsp + 0x20], rax
1420720d0: 488d0de10ff600 lea rcx, [rip + 0xf60fe1]
1420720d7: ba41000000 mov edx, 0x41
1420720dc: 4531c0 xor r8d, r8d
1420720df: e85c788600 call 0x1428d9940 ; core::str::slice_error_fail
1420720e4: 31c0 xor eax, eax
1420720e6: e9bf010000 jmp 0x1420722aa ; unknown
1420720eb: 75d7 jne 0x1420720c4
1420720ed: 4c8d35c40ff600 lea r14, [rip + 0xf60fc4]
1420720f4: 4c89f1 mov rcx, r14
1420720f7: 4c89ca mov rdx, r9
1420720fa: e871cdffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
1420720ff: 4883f801 cmp rax, 1
142072103: 7554 jne 0x142072159
142072105: 4989d0 mov r8, rdx
142072108: 4983c002 add r8, 2
14207210c: 7439 je 0x142072147
14207210e: 4983f841 cmp r8, 0x41
142072112: 7331 jae 0x142072145
142072114: 488d059d0ff600 lea rax, [rip + 0xf60f9d]
14207211b: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
142072120: 7f25 jg 0x142072147
142072122: 488d05c712f600 lea rax, [rip + 0xf612c7]
142072129: 4889442420 mov qword ptr [rsp + 0x20], rax
14207212e: 488d0d830ff600 lea rcx, [rip + 0xf60f83]
142072135: ba41000000 mov edx, 0x41
14207213a: 41b941000000 mov r9d, 0x41
142072140: e8fb778600 call 0x1428d9940 ; core::str::slice_error_fail
142072145: 75db jne 0x142072122
142072147: bb3f000000 mov ebx, 0x3f
14207214c: 4829d3 sub rbx, rdx
14207214f: 4c8d35620ff600 lea r14, [rip + 0xf60f62]
142072156: 4d01c6 add r14, r8
142072159: 4c8d05fe12f600 lea r8, [rip + 0xf612fe]
142072160: 4c8d3dc912f600 lea r15, [rip + 0xf612c9]
142072167: 660f1f840000000000 nop word ptr [rax + rax]
142072170: 410fbe40ff movsx eax, byte ptr [r8 - 1]
142072175: 85c0 test eax, eax
142072177: 7817 js 0x142072190
142072179: 49ffc8 dec r8
14207217c: 83f85c cmp eax, 0x5c
14207217f: 7563 jne 0x1420721e4
142072181: eb73 jmp 0x1420721f6 ; unknown
142072183: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142072190: 410fb648fe movzx ecx, byte ptr [r8 - 2]
142072195: 80f9c0 cmp cl, 0xc0
142072198: 7d23 jge 0x1420721bd
14207219a: 410fb650fd movzx edx, byte ptr [r8 - 3]
14207219f: 80fac0 cmp dl, 0xc0
1420721a2: 7d22 jge 0x1420721c6
1420721a4: 450fb648fc movzx r9d, byte ptr [r8 - 4]
1420721a9: 4983c0fc add r8, -4
1420721ad: 4183e107 and r9d, 7
1420721b1: 41c1e106 shl r9d, 6
1420721b5: 83e23f and edx, 0x3f
1420721b8: 4409ca or edx, r9d
1420721bb: eb10 jmp 0x1420721cd ; unknown
1420721bd: 4983c0fe add r8, -2
1420721c1: 83e11f and ecx, 0x1f
1420721c4: eb0f jmp 0x1420721d5 ; unknown
1420721c6: 4983c0fd add r8, -3
1420721ca: 83e20f and edx, 0xf
1420721cd: c1e206 shl edx, 6
1420721d0: 83e13f and ecx, 0x3f
1420721d3: 09d1 or ecx, edx
1420721d5: c1e106 shl ecx, 6
1420721d8: 243f and al, 0x3f
1420721da: 0fb6c0 movzx eax, al
1420721dd: 09c8 or eax, ecx
1420721df: 83f85c cmp eax, 0x5c
1420721e2: 7412 je 0x1420721f6
1420721e4: 83f82f cmp eax, 0x2f
1420721e7: 740d je 0x1420721f6
1420721e9: 4d39f8 cmp r8, r15
1420721ec: 7582 jne 0x142072170
1420721ee: 41bc2e000000 mov r12d, 0x2e
1420721f4: eb4d jmp 0x142072243 ; unknown
1420721f6: 4c8d3d3312f600 lea r15, [rip + 0xf61233]
1420721fd: 4d29f8 sub r8, r15
142072200: 49ffc0 inc r8
142072203: 7432 je 0x142072237
142072205: 4983f82e cmp r8, 0x2e
142072209: 732a jae 0x142072235
14207220b: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
142072210: 7f25 jg 0x142072237
142072212: 488d059f11f600 lea rax, [rip + 0xf6119f]
142072219: 4889442420 mov qword ptr [rsp + 0x20], rax
14207221e: 488d0d0b12f600 lea rcx, [rip + 0xf6120b]
142072225: ba2e000000 mov edx, 0x2e
14207222a: 41b92e000000 mov r9d, 0x2e
142072230: e80b778600 call 0x1428d9940 ; core::str::slice_error_fail
142072235: 75db jne 0x142072212
142072237: 41bc2e000000 mov r12d, 0x2e
14207223d: 4d29c4 sub r12, r8
142072240: 4d01c7 add r15, r8
142072243: e8e85ebafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142072248: 4889c1 mov rcx, rax
14207224b: 488b00 mov rax, qword ptr [rax]
14207224e: 4883f801 cmp rax, 1
142072252: 0f8591050000 jne 0x1420727e9
142072258: 4883c108 add rcx, 8
14207225c: 48833900 cmp qword ptr [rcx], 0
142072260: 0f85a7050000 jne 0x14207280d
142072266: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207226d: 48894d58 mov qword ptr [rbp + 0x58], rcx
142072271: 4883c108 add rcx, 8
142072275: 4c89642420 mov qword ptr [rsp + 0x20], r12
14207227a: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
142072283: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14207228c: 4c89f2 mov rdx, r14
14207228f: 4989d8 mov r8, rbx
142072292: 4d89f9 mov r9, r15
142072295: e8a64abafe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
14207229a: 488b4d58 mov rcx, qword ptr [rbp + 0x58]
14207229e: 48ff01 inc qword ptr [rcx]
1420722a1: 48894538 mov qword ptr [rbp + 0x38], rax
1420722a5: b801000000 mov eax, 1
1420722aa: 48894530 mov qword ptr [rbp + 0x30], rax
1420722ae: 488b0e mov rcx, qword ptr [rsi]
1420722b1: b20f mov dl, 0xf
1420722b3: e898f991fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
1420722b8: 48895528 mov qword ptr [rbp + 0x28], rdx
1420722bc: 48894548 mov qword ptr [rbp + 0x48], rax
1420722c0: 488b0f mov rcx, qword ptr [rdi]
1420722c3: 4c8b7708 mov r14, qword ptr [rdi + 8]
1420722c7: 4c8bb9f8000000 mov r15, qword ptr [rcx + 0xf8]
1420722ce: 488b8100010000 mov rax, qword ptr [rcx + 0x100]
1420722d5: 4d8d2487 lea r12, [r15 + rax*4]
1420722d9: 48894d50 mov qword ptr [rbp + 0x50], rcx
1420722dd: 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
1420722e4: 884558 mov byte ptr [rbp + 0x58], al
1420722e7: b808000000 mov eax, 8
1420722ec: 48894540 mov qword ptr [rbp + 0x40], rax
1420722f0: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
1420722f8: 0f283581c68b00 movaps xmm6, xmmword ptr [rip + 0x8bc681]
1420722ff: f30f103dfd358b00 movss xmm7, dword ptr [rip + 0x8b35fd]
142072307: 4531ed xor r13d, r13d
14207230a: 31f6 xor esi, esi
14207230c: 31ff xor edi, edi
14207230e: 31db xor ebx, ebx
142072310: 807d5800 cmp byte ptr [rbp + 0x58], 0
142072314: 0f8536010000 jne 0x142072450
14207231a: 4839fb cmp rbx, rdi
14207231d: 0f8517010000 jne 0x14207243a
142072323: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142072330: 4d39e7 cmp r15, r12
142072333: 0f84cc030000 je 0x142072705
142072339: 418b0f mov ecx, dword ptr [r15]
14207233c: 4983c704 add r15, 4
142072340: 498b8600010000 mov rax, qword ptr [r14 + 0x100]
142072347: 488d0c89 lea rcx, [rcx + rcx*4]
14207234b: 48c1e105 shl rcx, 5
14207234f: 488b7c0858 mov rdi, qword ptr [rax + rcx + 0x58]
142072354: 4885ff test rdi, rdi
142072357: 74d7 je 0x142072330
142072359: 4801c8 add rax, rcx
14207235c: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
142072362: 498b96a8010000 mov rdx, qword ptr [r14 + 0x1a8]
142072369: 4c8d0cc9 lea r9, [rcx + rcx*8]
14207236d: 4a8d0cca lea rcx, [rdx + r9*8]
142072371: 4c8b5550 mov r10, qword ptr [rbp + 0x50]
142072375: 4d8b8210010000 mov r8, qword ptr [r10 + 0x110]
14207237c: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
142072381: 4c39c2 cmp rdx, r8
142072384: 7633 jbe 0x1420723b9
142072386: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207238a: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207238e: 4d85c0 test r8, r8
142072391: 7426 je 0x1420723b9
142072393: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
142072397: 49f7d0 not r8
14207239a: 4f8d0440 lea r8, [r8 + r8*2]
14207239e: 49c1e004 shl r8, 4
1420723a2: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
1420723a7: 4c894560 mov qword ptr [rbp + 0x60], r8
1420723ab: 4d8b8218010000 mov r8, qword ptr [r10 + 0x118]
1420723b2: 4c39c2 cmp rdx, r8
1420723b5: 7716 ja 0x1420723cd
1420723b7: eb43 jmp 0x1420723fc ; unknown
1420723b9: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
1420723c1: 4d8b8218010000 mov r8, qword ptr [r10 + 0x118]
1420723c8: 4c39c2 cmp rdx, r8
1420723cb: 762f jbe 0x1420723fc
1420723cd: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
1420723d1: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
1420723d5: 4d85c0 test r8, r8
1420723d8: 7422 je 0x1420723fc
1420723da: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
1420723de: 49f7d0 not r8
1420723e1: 4f8d0440 lea r8, [r8 + r8*2]
1420723e5: 49c1e004 shl r8, 4
1420723e9: 4f8b6c0110 mov r13, qword ptr [r9 + r8 + 0x10]
1420723ee: 4d8b8220010000 mov r8, qword ptr [r10 + 0x120]
1420723f5: 4c39c2 cmp rdx, r8
1420723f8: 7634 jbe 0x14207242e
1420723fa: eb0f jmp 0x14207240b ; unknown
1420723fc: 4531ed xor r13d, r13d
1420723ff: 4d8b8220010000 mov r8, qword ptr [r10 + 0x120]
142072406: 4c39c2 cmp rdx, r8
142072409: 7623 jbe 0x14207242e
14207240b: 488b5138 mov rdx, qword ptr [rcx + 0x38]
14207240f: 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
142072413: 4885d2 test rdx, rdx
142072416: 7416 je 0x14207242e
142072418: 488b4918 mov rcx, qword ptr [rcx + 0x18]
14207241c: 48f7d2 not rdx
14207241f: 488d1452 lea rdx, [rdx + rdx*2]
142072423: 48c1e204 shl rdx, 4
142072427: 488b741110 mov rsi, qword ptr [rcx + rdx + 0x10]
14207242c: eb02 jmp 0x142072430 ; unknown
14207242e: 31f6 xor esi, esi
142072430: 488b4050 mov rax, qword ptr [rax + 0x50]
142072434: 48894540 mov qword ptr [rbp + 0x40], rax
142072438: 31db xor ebx, ebx
14207243a: 4889d8 mov rax, rbx
14207243d: 48c1e004 shl rax, 4
142072441: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
142072445: 8b440108 mov eax, dword ptr [rcx + rax + 8]
142072449: e9fd000000 jmp 0x14207254b ; unknown
14207244e: 6690 nop 
142072450: 4839fb cmp rbx, rdi
142072453: 0f85f0000000 jne 0x142072549
142072459: 0f1f8000000000 nop dword ptr [rax]
142072460: 4d39e7 cmp r15, r12
142072463: 0f849c020000 je 0x142072705
142072469: 418b0f mov ecx, dword ptr [r15]
14207246c: 4983c704 add r15, 4
142072470: 498b86a8010000 mov rax, qword ptr [r14 + 0x1a8]
142072477: 488d0cc9 lea rcx, [rcx + rcx*8]
14207247b: 488b7cc810 mov rdi, qword ptr [rax + rcx*8 + 0x10]
142072480: 4885ff test rdi, rdi
142072483: 74db je 0x142072460
142072485: 488d04c8 lea rax, [rax + rcx*8]
142072489: 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
14207248d: 498b9110010000 mov rdx, qword ptr [r9 + 0x110]
142072494: 488b4840 mov rcx, qword ptr [rax + 0x40]
142072498: 4839d1 cmp rcx, rdx
14207249b: 7633 jbe 0x1420724d0
14207249d: 4c8b4038 mov r8, qword ptr [rax + 0x38]
1420724a1: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
1420724a5: 4885d2 test rdx, rdx
1420724a8: 7426 je 0x1420724d0
1420724aa: 4c8b4018 mov r8, qword ptr [rax + 0x18]
1420724ae: 48f7d2 not rdx
1420724b1: 488d1452 lea rdx, [rdx + rdx*2]
1420724b5: 48c1e204 shl rdx, 4
1420724b9: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
1420724be: 48895560 mov qword ptr [rbp + 0x60], rdx
1420724c2: 498b9118010000 mov rdx, qword ptr [r9 + 0x118]
1420724c9: 4839d1 cmp rcx, rdx
1420724cc: 7716 ja 0x1420724e4
1420724ce: eb43 jmp 0x142072513 ; unknown
1420724d0: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
1420724d8: 498b9118010000 mov rdx, qword ptr [r9 + 0x118]
1420724df: 4839d1 cmp rcx, rdx
1420724e2: 762f jbe 0x142072513
1420724e4: 4c8b4038 mov r8, qword ptr [rax + 0x38]
1420724e8: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
1420724ec: 4885d2 test rdx, rdx
1420724ef: 7422 je 0x142072513
1420724f1: 4c8b4018 mov r8, qword ptr [rax + 0x18]
1420724f5: 48f7d2 not rdx
1420724f8: 488d1452 lea rdx, [rdx + rdx*2]
1420724fc: 48c1e204 shl rdx, 4
142072500: 4d8b6c1010 mov r13, qword ptr [r8 + rdx + 0x10]
142072505: 498b9120010000 mov rdx, qword ptr [r9 + 0x120]
14207250c: 4839d1 cmp rcx, rdx
14207250f: 7634 jbe 0x142072545
142072511: eb0f jmp 0x142072522 ; unknown
142072513: 4531ed xor r13d, r13d
142072516: 498b9120010000 mov rdx, qword ptr [r9 + 0x120]
14207251d: 4839d1 cmp rcx, rdx
142072520: 7623 jbe 0x142072545
142072522: 488b4838 mov rcx, qword ptr [rax + 0x38]
142072526: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
14207252a: 4885c9 test rcx, rcx
14207252d: 7416 je 0x142072545
14207252f: 488b4018 mov rax, qword ptr [rax + 0x18]
142072533: 48f7d1 not rcx
142072536: 488d0c49 lea rcx, [rcx + rcx*2]
14207253a: 48c1e104 shl rcx, 4
14207253e: 488b740810 mov rsi, qword ptr [rax + rcx + 0x10]
142072543: eb02 jmp 0x142072547 ; unknown
142072545: 31f6 xor esi, esi
142072547: 31db xor ebx, ebx
142072549: 89d8 mov eax, ebx
14207254b: 488d0c40 lea rcx, [rax + rax*2]
14207254f: 488b5560 mov rdx, qword ptr [rbp + 0x60]
142072553: 488d148a lea rdx, [rdx + rcx*4]
142072557: 48c1e104 shl rcx, 4
14207255b: 4c01e9 add rcx, r13
14207255e: 4801f0 add rax, rsi
142072561: 448b4118 mov r8d, dword ptr [rcx + 0x18]
142072565: 44894520 mov dword ptr [rbp + 0x20], r8d
142072569: 4c8b4110 mov r8, qword ptr [rcx + 0x10]
14207256d: 4c894518 mov qword ptr [rbp + 0x18], r8
142072571: f30f10591c movss xmm3, dword ptr [rcx + 0x1c]
142072576: f30f104120 movss xmm0, dword ptr [rcx + 0x20]
14207257b: f30f104924 movss xmm1, dword ptr [rcx + 0x24]
142072580: 0f54de andps xmm3, xmm6
142072583: 0f54c6 andps xmm0, xmm6
142072586: 0f54ce andps xmm1, xmm6
142072589: 0f28d0 movaps xmm2, xmm0
14207258c: f30fc2d003 cmpunordss xmm2, xmm0
142072591: 0f28e2 movaps xmm4, xmm2
142072594: 0f54e1 andps xmm4, xmm1
142072597: f30f5fc8 maxss xmm1, xmm0
14207259b: 0f55d1 andnps xmm2, xmm1
14207259e: 0f56d4 orps xmm2, xmm4
1420725a1: 0f28c2 movaps xmm0, xmm2
1420725a4: f30f5fc3 maxss xmm0, xmm3
1420725a8: f30fc2db03 cmpunordss xmm3, xmm3
1420725ad: 0f28cb movaps xmm1, xmm3
1420725b0: 0f54ca andps xmm1, xmm2
1420725b3: 0f55d8 andnps xmm3, xmm0
1420725b6: 0f56d9 orps xmm3, xmm1
1420725b9: 0f10511c movups xmm2, xmmword ptr [rcx + 0x1c]
1420725bd: 0f104120 movups xmm0, xmmword ptr [rcx + 0x20]
1420725c1: 440f2811 movaps xmm10, xmmword ptr [rcx]
1420725c5: 0f284910 movaps xmm1, xmmword ptr [rcx + 0x10]
1420725c9: f3440f104108 movss xmm8, dword ptr [rcx + 8]
1420725cf: 450f28d8 movaps xmm11, xmm8
1420725d3: f3450f58d8 addss xmm11, xmm8
1420725d8: f3450f59c3 mulss xmm8, xmm11
1420725dd: 410f28ea movaps xmm5, xmm10
1420725e1: 410f58ea addps xmm5, xmm10
1420725e5: f30f16e5 movshdup xmm4, xmm5
1420725e9: f3410f59e2 mulss xmm4, xmm10
1420725ee: 450f28ca movaps xmm9, xmm10
1420725f2: f3450f12e3 movsldup xmm12, xmm11
1420725f7: 450f59e2 mulps xmm12, xmm10
1420725fb: 450fc6d2ff shufps xmm10, xmm10, 0xff
142072600: 440f59cd mulps xmm9, xmm5
142072604: 410f59ea mulps xmm5, xmm10
142072608: 450f28ea movaps xmm13, xmm10
14207260c: f3450f59eb mulss xmm13, xmm11
142072611: 0fc6ede1 shufps xmm5, xmm5, 0xe1
142072615: f3450f16f1 movshdup xmm14, xmm9
14207261a: 450f28d6 movaps xmm10, xmm14
14207261e: f3450f58d0 addss xmm10, xmm8
142072623: 440f28df movaps xmm11, xmm7
142072627: f3450f5cda subss xmm11, xmm10
14207262c: 440f28d4 movaps xmm10, xmm4
142072630: f3450f58d5 addss xmm10, xmm13
142072635: 450f14da unpcklps xmm11, xmm10
142072639: 450f28d4 movaps xmm10, xmm12
14207263d: 440f5cd5 subps xmm10, xmm5
142072641: 450f16da movlhps xmm11, xmm10
142072645: f3410f5ce5 subss xmm4, xmm13
14207264a: f3450f58c1 addss xmm8, xmm9
14207264f: 440f28ef movaps xmm13, xmm7
142072653: f3450f5ce8 subss xmm13, xmm8
142072658: 410f14e5 unpcklps xmm4, xmm13
14207265c: 410f58ec addps xmm5, xmm12
142072660: 0fc6e5d4 shufps xmm4, xmm5, 0xd4
142072664: 440fc6d501 shufps xmm10, xmm5, 1
142072669: f3450f58ce addss xmm9, xmm14
14207266e: 440f28c7 movaps xmm8, xmm7
142072672: f3450f5cc1 subss xmm8, xmm9
142072677: 440fc6c530 shufps xmm8, xmm5, 0x30
14207267c: 450fc6d082 shufps xmm10, xmm8, 0x82
142072681: 0fc6d200 shufps xmm2, xmm2, 0
142072685: 410f59d3 mulps xmm2, xmm11
142072689: 0f28e8 movaps xmm5, xmm0
14207268c: 0fc6e800 shufps xmm5, xmm0, 0
142072690: 0f59ec mulps xmm5, xmm4
142072693: 0fc6c055 shufps xmm0, xmm0, 0x55
142072697: 410f59c2 mulps xmm0, xmm10
14207269b: 0fb600 movzx eax, byte ptr [rax]
14207269e: 488d0d2312f600 lea rcx, [rip + 0xf61223]
1420726a5: 8b0481 mov eax, dword ptr [rcx + rax*4]
1420726a8: 0f28e2 movaps xmm4, xmm2
1420726ab: f20f10e5 movsd xmm4, xmm5
1420726af: 0fc6d424 shufps xmm2, xmm4, 0x24
1420726b3: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
1420726b7: 0fc6e849 shufps xmm5, xmm0, 0x49
1420726bb: 0f296dd0 movaps xmmword ptr [rbp - 0x30], xmm5
1420726bf: 0fc6c102 shufps xmm0, xmm1, 2
1420726c3: 0fc6c198 shufps xmm0, xmm1, 0x98
1420726c7: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
1420726cb: 8945f0 mov dword ptr [rbp - 0x10], eax
1420726ce: 48c745f400000000 mov qword ptr [rbp - 0xc], 0
1420726d6: c745fc00000000 mov dword ptr [rbp - 4], 0
1420726dd: 488d45c0 lea rax, [rbp - 0x40]
1420726e1: 4889442420 mov qword ptr [rsp + 0x20], rax
1420726e6: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
1420726ea: 4c8d4518 lea r8, [rbp + 0x18]
1420726ee: e83de64fff call 0x141570d30 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>
1420726f3: 48ffc3 inc rbx
1420726f6: 807d5800 cmp byte ptr [rbp + 0x58], 0
1420726fa: 0f841afcffff je 0x14207231a
142072700: e94bfdffff jmp 0x142072450 ; unknown
142072705: 488b7548 mov rsi, qword ptr [rbp + 0x48]
142072709: 4889f1 mov rcx, rsi
14207270c: e8fff6c0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
142072711: 4889f1 mov rcx, rsi
142072714: 4883c901 or rcx, 1
142072718: e8f3f6c0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207271d: 31c9 xor ecx, ecx
14207271f: b808000000 mov eax, 8
142072724: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
142072729: 0f85ea000000 jne 0x142072819
14207272f: 48837d3000 cmp qword ptr [rbp + 0x30], 0
142072734: 7442 je 0x142072778
142072736: e8f559bafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207273b: 4889c1 mov rcx, rax
14207273e: 488b00 mov rax, qword ptr [rax]
142072741: 4883f801 cmp rax, 1
142072745: 0f8584000000 jne 0x1420727cf
14207274b: 4883c108 add rcx, 8
14207274f: 48833900 cmp qword ptr [rcx], 0
142072753: 0f85b4000000 jne 0x14207280d
142072759: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142072760: 48894d58 mov qword ptr [rbp + 0x58], rcx
142072764: 4883c108 add rcx, 8
142072768: 488b5538 mov rdx, qword ptr [rbp + 0x38]
14207276c: e87f49bafe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
142072771: 488b4558 mov rax, qword ptr [rbp + 0x58]
142072775: 48ff00 inc qword ptr [rax]
142072778: 0f287570 movaps xmm6, xmmword ptr [rbp + 0x70]
14207277c: 0f28bd80000000 movaps xmm7, xmmword ptr [rbp + 0x80]
142072783: 440f288590000000 movaps xmm8, xmmword ptr [rbp + 0x90]
14207278b: 440f288da0000000 movaps xmm9, xmmword ptr [rbp + 0xa0]
142072793: 440f2895b0000000 movaps xmm10, xmmword ptr [rbp + 0xb0]
14207279b: 440f289dc0000000 movaps xmm11, xmmword ptr [rbp + 0xc0]
1420727a3: 440f28a5d0000000 movaps xmm12, xmmword ptr [rbp + 0xd0]
1420727ab: 440f28ade0000000 movaps xmm13, xmmword ptr [rbp + 0xe0]
1420727b3: 440f28b5f0000000 movaps xmm14, xmmword ptr [rbp + 0xf0]
1420727bb: 4881c488010000 add rsp, 0x188
1420727c2: 5b pop rbx
1420727c3: 5f pop rdi
1420727c4: 5e pop rsi
1420727c5: 415c pop r12
1420727c7: 415d pop r13
1420727c9: 415e pop r14
1420727cb: 415f pop r15
1420727cd: 5d pop rbp
1420727ce: c3 ret 
1420727cf: 4885c0 test rax, rax
1420727d2: 752d jne 0x142072801
1420727d4: 31d2 xor edx, edx
1420727d6: e8f55f8800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1420727db: 4889c1 mov rcx, rax
1420727de: 4885c0 test rax, rax
1420727e1: 0f8568ffffff jne 0x14207274f
1420727e7: eb18 jmp 0x142072801 ; unknown
1420727e9: 4885c0 test rax, rax
1420727ec: 7513 jne 0x142072801
1420727ee: 31d2 xor edx, edx
1420727f0: e8db5f8800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1420727f5: 4889c1 mov rcx, rax
1420727f8: 4885c0 test rax, rax
1420727fb: 0f855bfaffff jne 0x14207225c
142072801: 488d0d9882bc00 lea rcx, [rip + 0xbc8298]
142072808: e8c3588600 call 0x1428d80d0 ; std::thread::local::panic_access_error
14207280d: 488d0d5c83bc00 lea rcx, [rip + 0xbc835c]
142072814: e8376a8600 call 0x1428d9250 ; core::cell::panic_already_borrowed
142072819: 4889f1 mov rcx, rsi
14207281c: 31d2 xor edx, edx
14207281e: e83d978500 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
142072823: e907ffffff jmp 0x14207272f ; unknown
142072828: 0f1f840000000000 nop dword ptr [rax + rax]
142072830: 4889542410 mov qword ptr [rsp + 0x10], rdx
142072835: 55 push rbp
142072836: 4157 push r15
142072838: 4156 push r14
14207283a: 4155 push r13
14207283c: 4154 push r12
14207283e: 56 push rsi
14207283f: 57 push rdi
142072840: 53 push rbx
142072841: 4881ecc8000000 sub rsp, 0xc8
142072848: 488daa80000000 lea rbp, [rdx + 0x80]
14207284f: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
142072855: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
14207285b: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
142072861: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
142072867: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
14207286d: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
142072876: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
14207287f: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
142072887: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
14207288f: 488b4558 mov rax, qword ptr [rbp + 0x58]
142072893: 48ff00 inc qword ptr [rax]
142072896: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
14207289e: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
1420728a6: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
1420728af: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
1420728b8: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
1420728be: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
1420728c4: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
1420728ca: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
1420728d0: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
1420728d6: 4881c4c8000000 add rsp, 0xc8
1420728dd: 5b pop rbx
1420728de: 5f pop rdi
1420728df: 5e pop rsi
1420728e0: 415c pop r12
1420728e2: 415d pop r13
1420728e4: 415e pop r14
1420728e6: 415f pop r15
1420728e8: 5d pop rbp
1420728e9: c3 ret 
1420728ea: 660f1f440000 nop word ptr [rax + rax]
1420728f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420728f5: 55 push rbp
1420728f6: 4157 push r15
1420728f8: 4156 push r14
1420728fa: 4155 push r13
1420728fc: 4154 push r12
1420728fe: 56 push rsi
1420728ff: 57 push rdi
142072900: 53 push rbx
142072901: 4881ecc8000000 sub rsp, 0xc8
142072908: 488daa80000000 lea rbp, [rdx + 0x80]
14207290f: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
142072915: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
14207291b: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
142072921: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
142072927: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
14207292d: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
142072936: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
14207293f: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
142072947: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
14207294f: 488b4558 mov rax, qword ptr [rbp + 0x58]
142072953: 48ff00 inc qword ptr [rax]
142072956: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
14207295e: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
142072966: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
14207296f: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
142072978: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
14207297e: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
142072984: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
14207298a: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
142072990: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
142072996: 4881c4c8000000 add rsp, 0xc8
14207299d: 5b pop rbx
14207299e: 5f pop rdi
14207299f: 5e pop rsi
1420729a0: 415c pop r12
1420729a2: 415d pop r13
1420729a4: 415e pop r14
1420729a6: 415f pop r15
1420729a8: 5d pop rbp
1420729a9: c3 ret 
1420729aa: 660f1f440000 nop word ptr [rax + rax]
1420729b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420729b5: 55 push rbp
1420729b6: 4157 push r15
1420729b8: 4156 push r14
1420729ba: 4155 push r13
1420729bc: 4154 push r12
1420729be: 56 push rsi
1420729bf: 57 push rdi
1420729c0: 53 push rbx
1420729c1: 4881ecc8000000 sub rsp, 0xc8
1420729c8: 488daa80000000 lea rbp, [rdx + 0x80]
1420729cf: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
1420729d5: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
1420729db: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
1420729e1: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
1420729e7: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
1420729ed: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
1420729f6: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
1420729ff: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
142072a07: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
142072a0f: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
142072a13: e8b8e8fcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
142072a18: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
142072a20: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
142072a28: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
142072a31: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
142072a3a: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
142072a40: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
142072a46: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
142072a4c: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
142072a52: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
142072a58: 4881c4c8000000 add rsp, 0xc8
142072a5f: 5b pop rbx
142072a60: 5f pop rdi
142072a61: 5e pop rsi
142072a62: 415c pop r12
142072a64: 415d pop r13
142072a66: 415e pop r14
142072a68: 415f pop r15
142072a6a: 5d pop rbp
142072a6b: c3 ret 
142072a6c: 0f1f4000 nop dword ptr [rax]
142072a70: 4889542410 mov qword ptr [rsp + 0x10], rdx
142072a75: 55 push rbp
142072a76: 4157 push r15
142072a78: 4156 push r14
142072a7a: 4155 push r13
142072a7c: 4154 push r12
142072a7e: 56 push rsi
142072a7f: 57 push rdi
142072a80: 53 push rbx
142072a81: 4881ecc8000000 sub rsp, 0xc8
142072a88: 488daa80000000 lea rbp, [rdx + 0x80]
142072a8f: 440f29742430 movaps xmmword ptr [rsp + 0x30], xmm14
142072a95: 440f296c2440 movaps xmmword ptr [rsp + 0x40], xmm13
142072a9b: 440f29642450 movaps xmmword ptr [rsp + 0x50], xmm12
142072aa1: 440f295c2460 movaps xmmword ptr [rsp + 0x60], xmm11
142072aa7: 440f29542470 movaps xmmword ptr [rsp + 0x70], xmm10
142072aad: 440f298c2480000000 movaps xmmword ptr [rsp + 0x80], xmm9
142072ab6: 440f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm8
142072abf: 0f29bc24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm7
142072ac7: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
142072acf: 48837d3000 cmp qword ptr [rbp + 0x30], 0
142072ad4: 7420 je 0x142072af6
142072ad6: 488d4538 lea rax, [rbp + 0x38]
142072ada: 48894510 mov qword ptr [rbp + 0x10], rax
142072ade: 488d4510 lea rax, [rbp + 0x10]
142072ae2: 48894508 mov qword ptr [rbp + 8], rax
142072ae6: 488d0d6b08f600 lea rcx, [rip + 0xf6086b]
142072aed: 488d5508 lea rdx, [rbp + 8]
142072af1: e8dac72bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
142072af6: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
142072afe: 0f28bc24a0000000 movaps xmm7, xmmword ptr [rsp + 0xa0]
142072b06: 440f28842490000000 movaps xmm8, xmmword ptr [rsp + 0x90]
142072b0f: 440f288c2480000000 movaps xmm9, xmmword ptr [rsp + 0x80]
142072b18: 440f28542470 movaps xmm10, xmmword ptr [rsp + 0x70]
142072b1e: 440f285c2460 movaps xmm11, xmmword ptr [rsp + 0x60]
142072b24: 440f28642450 movaps xmm12, xmmword ptr [rsp + 0x50]
142072b2a: 440f286c2440 movaps xmm13, xmmword ptr [rsp + 0x40]
142072b30: 440f28742430 movaps xmm14, xmmword ptr [rsp + 0x30]
142072b36: 4881c4c8000000 add rsp, 0xc8
142072b3d: 5b pop rbx
142072b3e: 5f pop rdi
142072b3f: 5e pop rsi
142072b40: 415c pop r12
142072b42: 415d pop r13
142072b44: 415e pop r14
142072b46: 415f pop r15
142072b48: 5d pop rbp
142072b49: c3 ret 
