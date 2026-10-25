142299250: push rbp
142299251: push r15
142299253: push r14
142299255: push r13
142299257: push r12
142299259: push rsi
14229925a: push rdi
14229925b: push rbx
14229925c: sub rsp, 0x178
142299263: lea rbp, [rsp + 0x80]
14229926b: movaps xmmword ptr [rbp + 0xe0], xmm8
142299273: movaps xmmword ptr [rbp + 0xd0], xmm7
14229927a: movaps xmmword ptr [rbp + 0xc0], xmm6
142299281: mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
14229928c: mov r14, r9
14229928f: mov rsi, rcx
142299292: mov r9, qword ptr [r8]
142299295: mov rbx, qword ptr [r8 + 8]
142299299: mov rax, qword ptr [rbx + 0x30]
14229929d: mov rcx, qword ptr [rbx + 0x40]
1422992a1: xor r8d, r8d
1422992a4: mov qword ptr [rbp + 0xa8], r9
1422992ab: sub rcx, qword ptr [r9]
1422992ae: cmovae r8, rcx
1422992b2: add rax, qword ptr [rbx + 0x10]
1422992b6: cmp rax, r8
1422992b9: cmovae rax, r8
1422992bd: test rax, rax
1422992c0: je 0x142299662
1422992c6: mov rcx, qword ptr [rdx]
1422992c9: lea rdx, [rip + 0xe9d440]
1422992d0: call 0x1425f18b0
1422992d5: test rax, rax
1422992d8: je 0x1422996dd
1422992de: mov rdi, rax
1422992e1: lea rax, [rip + 0xe9d57a]
1422992e8: mov qword ptr [rbp - 0x40], rax
1422992ec: mov qword ptr [rbp - 0x38], 0x38
1422992f4: cmp qword ptr [rdi + 0x18], 0x38
1422992f9: jne 0x1422996a4
1422992ff: mov rdx, qword ptr [rdi + 0x10]
142299303: lea rcx, [rip + 0xe9d558]
14229930a: mov r8d, 0x38
142299310: call 0x142923bf0
142299315: test eax, eax
142299317: jne 0x1422996a4
14229931d: mov rcx, qword ptr [rdi]
142299320: mov rax, qword ptr [rdi + 8]
142299324: call qword ptr [rax + 0x18]
142299327: mov qword ptr [rbp + 0x40], rax
14229932b: test rax, rax
14229932e: je 0x1422996dd
142299334: mov qword ptr [rbp + 0x38], rsi
142299338: mov rcx, qword ptr [rbp + 0x168]
14229933f: mov r15, qword ptr [rbp + 0xa8]
142299346: mov r9, qword ptr [r15]
142299349: xor r8d, r8d
14229934c: mov rax, r9
14229934f: sub rax, qword ptr [rbx + 0x18]
142299353: cmovb rax, r8
142299357: sub r9, qword ptr [rbx + 0x38]
14229935b: mov rdx, qword ptr [rbp + 0x160]
142299362: cmovb r9, r8
142299366: lea rsi, [rax + rax*2]
14229936a: shl rsi, 3
14229936e: add rsi, qword ptr [rbx + 8]
142299372: mov r10, qword ptr [rbx + 0x10]
142299376: mov r11, qword ptr [rbx + 0x30]
14229937a: sub r10, rax
14229937d: cmovb r10, r8
142299381: mov edi, 8
142299386: cmovb rsi, rdi
14229938a: lea rax, [r9 + r9*2]
14229938e: shl rax, 3
142299392: add rax, qword ptr [rbx + 0x28]
142299396: sub r11, r9
142299399: cmovb r11, r8
14229939d: cmovb rax, rdi
1422993a1: mov r8, qword ptr [rbx + 0x40]
1422993a5: sub r8, r11
1422993a8: sub r8, r10
1422993ab: mov qword ptr [r15], r8
1422993ae: lea r8, [r10 + r10*2]
1422993b2: lea r8, [rsi + r8*8]
1422993b6: mov qword ptr [rbp + 0x10], r8
1422993ba: lea r8, [r11 + r11*2]
1422993be: lea r8, [rax + r8*8]
1422993c2: mov qword ptr [rbp + 0x18], r8
1422993c6: mov r8d, dword ptr [rcx + 0x1c]
1422993ca: mov dword ptr [rbp + 0xb0], r8d
1422993d1: mov r15, qword ptr [rcx]
1422993d4: mov r12, qword ptr [rcx + 0x10]
1422993d8: mov rcx, qword ptr [r14]
1422993db: mov qword ptr [rbp + 0x20], rcx
1422993df: mov ecx, dword ptr [rdx + 0x1c]
1422993e2: mov dword ptr [rbp + 0xb4], ecx
1422993e8: mov r14, qword ptr [rdx]
1422993eb: mov rcx, qword ptr [rdx + 0x10]
1422993ef: mov qword ptr [rbp + 0x28], rcx
1422993f3: lea rcx, [r14 + 0x20]
1422993f7: mov qword ptr [rbp + 0x30], rcx
1422993fb: movaps xmm6, xmmword ptr [rip + 0x829a1e]
142299402: movaps xmm7, xmmword ptr [rip + 0x829a27]
142299409: movss xmm8, dword ptr [rip + 0x68c45e]
142299412: jmp 0x142299450
142299414: nop word ptr cs:[rax + rax]
142299420: mov rax, qword ptr [r14 + 0x28]
142299424: mov rcx, rsi
142299427: shl rcx, 5
14229942b: mov qword ptr [rax + rcx], rdi
14229942f: mov edx, dword ptr [rbp + 0x58]
142299432: mov dword ptr [rax + rcx + 0x18], edx
142299436: movups xmm0, xmmword ptr [rbp + 0x48]
14229943a: movups xmmword ptr [rax + rcx + 8], xmm0
14229943f: inc rsi
142299442: mov qword ptr [r14 + 0x30], rsi
142299446: inc qword ptr [r14 + 0x40]
14229944a: mov rax, rbx
14229944d: mov rsi, r13
142299450: test rsi, rsi
142299453: je 0x142299470
142299455: cmp rsi, qword ptr [rbp + 0x10]
142299459: je 0x142299470
14229945b: lea r13, [rsi + 0x18]
14229945f: mov rbx, rax
142299462: jmp 0x14229948d
142299464: nop word ptr cs:[rax + rax]
142299470: test rax, rax
142299473: je 0x1422995e8
142299479: cmp rax, qword ptr [rbp + 0x18]
14229947d: je 0x1422995e8
142299483: lea rbx, [rax + 0x18]
142299487: xor r13d, r13d
14229948a: mov rsi, rax
14229948d: lea rdi, [rsi + 8]
142299491: mov rax, qword ptr [rbp + 0xa8]
142299498: inc qword ptr [rax]
14229949b: mov rcx, rdi
14229949e: call 0x1400885d0
1422994a3: cmp rax, 0x5000
1422994a9: jae 0x14229968c
1422994af: mov ecx, dword ptr [rsi + 0x10]
1422994b2: mov edx, dword ptr [rbp + 0xb0]
1422994b8: mov dword ptr [r12], edx
1422994bc: mov rdx, qword ptr [r15 + 0x10]
1422994c0: cmp rax, rdx
1422994c3: jae 0x1422996ce
1422994c9: mov rdx, qword ptr [r15 + 8]
1422994cd: mov dword ptr [rdx + rax*4], ecx
1422994d0: cmp byte ptr [rsi + 0x14], 1
1422994d4: mov rax, rbx
1422994d7: mov rsi, r13
1422994da: jne 0x142299450
1422994e0: mov rdx, qword ptr [rdi]
1422994e3: mov rcx, qword ptr [rbp + 0x20]
1422994e7: lea r8, [rip + 0xda2922]
1422994ee: call 0x140a91ef0
1422994f3: mov rdi, rax
1422994f6: mov rax, rbx
1422994f9: mov rsi, r13
1422994fc: test rdi, rdi
1422994ff: je 0x142299450
142299505: mov rcx, qword ptr [rdi + 0x48]
142299509: movaps xmm0, xmm7
14229950c: movaps xmm1, xmm6
14229950f: test rcx, rcx
142299512: je 0x142299571
142299514: mov rax, qword ptr [rdi + 0x40]
142299518: lea rcx, [rcx + rcx*2]
14229951c: lea rcx, [rax + rcx*4]
142299520: movaps xmm0, xmm7
142299523: movaps xmm2, xmm6
142299526: movaps xmm1, xmm6
142299529: nop dword ptr [rax]
142299530: movss xmm3, dword ptr [rax]
142299534: movss xmm4, dword ptr [rax + 8]
142299539: unpcklps xmm3, xmm4
14229953c: movaps xmm4, xmm3
14229953f: minps xmm4, xmm0
142299542: cmpunordps xmm0, xmm0
142299546: movaps xmm5, xmm3
142299549: andps xmm5, xmm0
14229954c: andnps xmm0, xmm4
14229954f: orps xmm0, xmm5
142299552: cmpunordps xmm1, xmm1
142299556: movaps xmm4, xmm3
142299559: andps xmm4, xmm1
14229955c: maxps xmm3, xmm2
14229955f: andnps xmm1, xmm3
142299562: orps xmm1, xmm4
142299565: add rax, 0xc
142299569: movaps xmm2, xmm1
14229956c: cmp rax, rcx
14229956f: jne 0x142299530
142299571: movlhps xmm0, xmm1
142299574: movups xmmword ptr [rbp - 0x40], xmm0
142299578: lea rcx, [rbp - 0x40]
14229957c: call 0x1412400c0
142299581: movss dword ptr [rbp + 0x98], xmm0
142299589: movss dword ptr [rbp + 0x9c], xmm1
142299591: mov rcx, rdi
142299594: call 0x140b2a620
142299599: mulss xmm0, xmm8
14229959e: lea rcx, [rbp + 0x50]
1422995a2: lea rdx, [rbp + 0x98]
1422995a9: movaps xmm2, xmm0
1422995ac: call 0x140c9cde0
1422995b1: mov byte ptr [rbp + 0x48], 0xc
1422995b5: mov eax, dword ptr [rbp + 0xb4]
1422995bb: mov rcx, qword ptr [rbp + 0x28]
1422995bf: mov dword ptr [rcx], eax
1422995c1: mov rsi, qword ptr [r14 + 0x30]
1422995c5: mov rdi, qword ptr [r14 + 0x40]
1422995c9: cmp rsi, qword ptr [r14 + 0x20]
1422995cd: jne 0x142299420
1422995d3: mov rcx, qword ptr [rbp + 0x30]
1422995d7: lea rdx, [rip + 0xda1cea]
1422995de: call 0x140562c30
1422995e3: jmp 0x142299420
1422995e8: mov rax, qword ptr [rbp + 0x38]
1422995ec: mov rcx, qword ptr [rax]
1422995ef: call 0x140987ca0
1422995f4: mov rcx, qword ptr [r15 + 0x10]
1422995f8: test rcx, rcx
1422995fb: je 0x142299662
1422995fd: shl rcx, 2
142299601: mov r8, qword ptr [rbp + 0x40]
142299605: cmp rcx, qword ptr [r8 + 0x18]
142299609: ja 0x1422996e9
14229960f: mov r9, qword ptr [r15 + 8]
142299613: mov qword ptr [rbp + 0x98], 0
14229961e: mov qword ptr [rbp + 0xa0], rcx
142299625: lea rdx, [rbp + 0x98]
14229962c: mov qword ptr [rsp + 0x28], rdx
142299631: mov qword ptr [rsp + 0x20], rcx
142299636: mov qword ptr [rsp + 0x38], 0
14229963f: mov qword ptr [rsp + 0x30], 1
142299648: lea rcx, [rbp + 0x48]
14229964c: mov rdx, rax
14229964f: call 0x140beefa0
142299654: mov rax, qword ptr [rbp + 0x48]
142299658: cmp rax, 0xb
14229965c: jne 0x1422996f0
142299662: movaps xmm6, xmmword ptr [rbp + 0xc0]
142299669: movaps xmm7, xmmword ptr [rbp + 0xd0]
142299670: movaps xmm8, xmmword ptr [rbp + 0xe0]
142299678: add rsp, 0x178
14229967f: pop rbx
142299680: pop rdi
142299681: pop rsi
142299682: pop r12
142299684: pop r13
142299686: pop r14
142299688: pop r15
14229968a: pop rbp
14229968b: ret 
14229968c: lea rcx, [rip + 0xda2715]
142299693: lea r8, [rip + 0xda2746]
14229969a: mov edx, 0x32
14229969f: call 0x1428d9430
1422996a4: mov qword ptr [rbp + 0x48], 0
1422996ac: add rdi, 0x10
1422996b0: lea rax, [rip + 0xe9d061]
1422996b7: mov qword ptr [rsp + 0x20], rax
1422996bc: lea rdx, [rbp - 0x40]
1422996c0: lea r9, [rbp + 0x48]
1422996c4: xor ecx, ecx
1422996c6: mov r8, rdi
1422996c9: call 0x1428cab41
1422996ce: lea r8, [rip + 0xda2723]
1422996d5: mov rcx, rax
1422996d8: call 0x1428d9518
1422996dd: lea rcx, [rip + 0xda2694]
1422996e4: call 0x1428d9310
1422996e9: mov eax, 0xb
1422996ee: jmp 0x14229971e
1422996f0: mov rcx, qword ptr [rbp + 0x90]
1422996f7: mov qword ptr [rbp], rcx
1422996fb: movups xmm0, xmmword ptr [rbp + 0x50]
1422996ff: movups xmm1, xmmword ptr [rbp + 0x60]
142299703: movups xmm2, xmmword ptr [rbp + 0x70]
142299707: movups xmm3, xmmword ptr [rbp + 0x80]
14229970e: movaps xmmword ptr [rbp - 0x10], xmm3
142299712: movaps xmmword ptr [rbp - 0x20], xmm2
142299716: movaps xmmword ptr [rbp - 0x30], xmm1
14229971a: movaps xmmword ptr [rbp - 0x40], xmm0
14229971e: mov qword ptr [rbp + 0x48], rax
142299722: movaps xmm0, xmmword ptr [rbp - 0x40]
142299726: movaps xmm1, xmmword ptr [rbp - 0x30]
14229972a: movaps xmm2, xmmword ptr [rbp - 0x20]
14229972e: movaps xmm3, xmmword ptr [rbp - 0x10]
142299732: movups xmmword ptr [rbp + 0x50], xmm0
142299736: movups xmmword ptr [rbp + 0x60], xmm1
14229973a: movups xmmword ptr [rbp + 0x70], xmm2
14229973e: movups xmmword ptr [rbp + 0x80], xmm3
142299745: mov rax, qword ptr [rbp]
142299749: mov qword ptr [rbp + 0x90], rax
142299750: lea rax, [rip + 0xda2639]
142299757: mov qword ptr [rsp + 0x20], rax
