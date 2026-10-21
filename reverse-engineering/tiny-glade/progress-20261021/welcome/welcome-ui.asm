0x141ebe130: push rbp
0x141ebe131: push r15
0x141ebe133: push r14
0x141ebe135: push r13
0x141ebe137: push r12
0x141ebe139: push rsi
0x141ebe13a: push rdi
0x141ebe13b: push rbx
0x141ebe13c: sub rsp, 0x108
0x141ebe143: lea rbp, [rsp + 0x80]
0x141ebe14b: movaps xmmword ptr [rbp + 0x70], xmm6
0x141ebe14f: mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
0x141ebe157: mov r14, r9
0x141ebe15a: mov qword ptr [rbp + 0x58], r8
0x141ebe15e: mov rsi, rdx
0x141ebe161: mov qword ptr [rbp + 0x28], rcx
0x141ebe165: mov rbx, qword ptr [rbp + 0x118]
0x141ebe16c: call 0x140c16a00
0x141ebe171: test al, al
0x141ebe173: je 0x141ebe1d2
0x141ebe175: mov r15d, 0x1a
0x141ebe17b: mov ecx, 0x1a
0x141ebe180: call 0x141ebdd00
0x141ebe185: lea r12, [rip + 0x10dcd4c]
0x141ebe18c: cmp rax, 1
0x141ebe190: jne 0x141ebe23e
0x141ebe196: mov r9, rdx
0x141ebe199: test rdx, rdx
0x141ebe19c: je 0x141ebe1db
0x141ebe19e: cmp r9, 0x1a
0x141ebe1a2: jae 0x141ebe1d9
0x141ebe1a4: lea rax, [rip + 0x10dcd2d]
0x141ebe1ab: cmp byte ptr [r9 + rax], 0xbf
0x141ebe1b0: jg 0x141ebe1db
0x141ebe1b2: lea rax, [rip + 0x10dcdbf]
0x141ebe1b9: mov qword ptr [rsp + 0x20], rax
0x141ebe1be: lea rcx, [rip + 0x10dcd13]
0x141ebe1c5: mov edx, 0x1a
0x141ebe1ca: xor r8d, r8d
0x141ebe1cd: call 0x1428d9940
0x141ebe1d2: xor eax, eax
0x141ebe1d4: jmp 0x141ebe388
0x141ebe1d9: jne 0x141ebe1b2
0x141ebe1db: mov rcx, r9
0x141ebe1de: call 0x141ebdd00
0x141ebe1e3: cmp rax, 1
0x141ebe1e7: jne 0x141ebe23e
0x141ebe1e9: mov r8, rdx
0x141ebe1ec: add r8, 2
0x141ebe1f0: je 0x141ebe22b
0x141ebe1f2: cmp r8, 0x1a
0x141ebe1f6: jae 0x141ebe229
0x141ebe1f8: lea rax, [rip + 0x10dccd9]
0x141ebe1ff: cmp byte ptr [r8 + rax], 0xbf
0x141ebe204: jg 0x141ebe22b
0x141ebe206: lea rax, [rip + 0x10dcd83]
0x141ebe20d: mov qword ptr [rsp + 0x20], rax
0x141ebe212: lea rcx, [rip + 0x10dccbf]
0x141ebe219: mov edx, 0x1a
0x141ebe21e: mov r9d, 0x1a
0x141ebe224: call 0x1428d9940
0x141ebe229: jne 0x141ebe206
0x141ebe22b: mov r15d, 0x18
0x141ebe231: sub r15, rdx
0x141ebe234: lea r12, [rip + 0x10dcc9d]
0x141ebe23b: add r12, r8
0x141ebe23e: lea r8, [rip + 0x10dcda5]
0x141ebe245: lea r13, [rip + 0x10dcd74]
0x141ebe24c: nop dword ptr [rax]
0x141ebe250: movsx eax, byte ptr [r8 - 1]
0x141ebe255: test eax, eax
0x141ebe257: js 0x141ebe270
0x141ebe259: dec r8
0x141ebe25c: cmp eax, 0x5c
0x141ebe25f: jne 0x141ebe2c4
0x141ebe261: jmp 0x141ebe2d5
0x141ebe263: nop word ptr cs:[rax + rax]
0x141ebe270: movzx ecx, byte ptr [r8 - 2]
0x141ebe275: cmp cl, 0xc0
0x141ebe278: jge 0x141ebe29d
0x141ebe27a: movzx edx, byte ptr [r8 - 3]
0x141ebe27f: cmp dl, 0xc0
0x141ebe282: jge 0x141ebe2a6
0x141ebe284: movzx r9d, byte ptr [r8 - 4]
0x141ebe289: add r8, -4
0x141ebe28d: and r9d, 7
0x141ebe291: shl r9d, 6
0x141ebe295: and edx, 0x3f
0x141ebe298: or edx, r9d
0x141ebe29b: jmp 0x141ebe2ad
0x141ebe29d: add r8, -2
0x141ebe2a1: and ecx, 0x1f
0x141ebe2a4: jmp 0x141ebe2b5
0x141ebe2a6: add r8, -3
0x141ebe2aa: and edx, 0xf
0x141ebe2ad: shl edx, 6
0x141ebe2b0: and ecx, 0x3f
0x141ebe2b3: or ecx, edx
0x141ebe2b5: shl ecx, 6
0x141ebe2b8: and al, 0x3f
0x141ebe2ba: movzx eax, al
0x141ebe2bd: or eax, ecx
0x141ebe2bf: cmp eax, 0x5c
0x141ebe2c2: je 0x141ebe2d5
0x141ebe2c4: cmp eax, 0x2f
0x141ebe2c7: je 0x141ebe2d5
0x141ebe2c9: cmp r8, r13
0x141ebe2cc: jne 0x141ebe250
0x141ebe2ce: mov edi, 0x2a
0x141ebe2d3: jmp 0x141ebe321
0x141ebe2d5: lea r13, [rip + 0x10dcce4]
0x141ebe2dc: sub r8, r13
0x141ebe2df: inc r8
0x141ebe2e2: je 0x141ebe316
0x141ebe2e4: cmp r8, 0x2a
0x141ebe2e8: jae 0x141ebe314
0x141ebe2ea: cmp byte ptr [r8 + r13], 0xbf
0x141ebe2ef: jg 0x141ebe316
0x141ebe2f1: lea rax, [rip + 0x10dcc60]
0x141ebe2f8: mov qword ptr [rsp + 0x20], rax
0x141ebe2fd: lea rcx, [rip + 0x10dccbc]
0x141ebe304: mov edx, 0x2a
0x141ebe309: mov r9d, 0x2a
0x141ebe30f: call 0x1428d9940
0x141ebe314: jne 0x141ebe2f1
0x141ebe316: mov edi, 0x2a
0x141ebe31b: sub rdi, r8
0x141ebe31e: add r13, r8
0x141ebe321: call 0x140c18130
0x141ebe326: mov rcx, rax
0x141ebe329: mov rax, qword ptr [rax]
0x141ebe32c: cmp rax, 1
0x141ebe330: jne 0x141ebe5a8
0x141ebe336: add rcx, 8
0x141ebe33a: cmp qword ptr [rcx], 0
0x141ebe33e: jne 0x141ebe859
0x141ebe344: mov qword ptr [rcx], 0xffffffffffffffff
0x141ebe34b: mov qword ptr [rbp + 0x48], rcx
0x141ebe34f: add rcx, 8
0x141ebe353: mov qword ptr [rsp + 0x20], rdi
0x141ebe358: mov qword ptr [rsp + 0x30], 0
0x141ebe361: mov qword ptr [rsp + 0x28], 1
0x141ebe36a: mov rdx, r12
0x141ebe36d: mov r8, r15
0x141ebe370: mov r9, r13
0x141ebe373: call 0x140c16d40
0x141ebe378: mov rcx, qword ptr [rbp + 0x48]
0x141ebe37c: inc qword ptr [rcx]
0x141ebe37f: mov qword ptr [rbp + 0x40], rax
0x141ebe383: mov eax, 1
0x141ebe388: mov rcx, qword ptr [rbp + 0xf8]
0x141ebe38f: mov qword ptr [rbp + 0x38], rax
0x141ebe393: mov rax, qword ptr [rcx]
0x141ebe396: cmp byte ptr [rax + 0xe5], 0
0x141ebe39d: jne 0x141ebe3b4
0x141ebe39f: mov rax, qword ptr [rbp + 0x100]
0x141ebe3a6: mov r12, qword ptr [rax]
0x141ebe3a9: cmp byte ptr [r12 + 0x206], 0
0x141ebe3b2: je 0x141ebe427
0x141ebe3b4: mov eax, dword ptr [rsi + 0x1c]
0x141ebe3b7: mov rcx, qword ptr [rsi + 0x10]
0x141ebe3bb: mov dword ptr [rcx], eax
0x141ebe3bd: mov rax, qword ptr [rsi]
0x141ebe3c0: mov dword ptr [rax], 2
0x141ebe3c6: cmp qword ptr [rbp + 0x38], 0
0x141ebe3cb: je 0x141ebe40f
0x141ebe3cd: call 0x140c18130
0x141ebe3d2: mov rcx, rax
0x141ebe3d5: mov rax, qword ptr [rax]
0x141ebe3d8: cmp rax, 1
0x141ebe3dc: jne 0x141ebe587
0x141ebe3e2: add rcx, 8
0x141ebe3e6: cmp qword ptr [rcx], 0
0x141ebe3ea: jne 0x141ebe859
0x141ebe3f0: mov qword ptr [rcx], 0xffffffffffffffff
0x141ebe3f7: mov qword ptr [rbp + 0x58], rcx
0x141ebe3fb: add rcx, 8
0x141ebe3ff: mov rdx, qword ptr [rbp + 0x40]
0x141ebe403: call 0x140c170f0
0x141ebe408: mov rax, qword ptr [rbp + 0x58]
0x141ebe40c: inc qword ptr [rax]
0x141ebe40f: movaps xmm6, xmmword ptr [rbp + 0x70]
0x141ebe413: add rsp, 0x108
0x141ebe41a: pop rbx
0x141ebe41b: pop rdi
0x141ebe41c: pop rsi
0x141ebe41d: pop r12
0x141ebe41f: pop r13
0x141ebe421: pop r14
0x141ebe423: pop r15
0x141ebe425: pop rbp
0x141ebe426: ret 
0x141ebe427: mov qword ptr [rbp + 0x30], r14
0x141ebe42b: mov qword ptr [rbp + 0x48], rbx
0x141ebe42f: mov rax, qword ptr [rbp + 0x110]
0x141ebe436: mov rcx, qword ptr [rax]
0x141ebe439: test rcx, rcx
0x141ebe43c: je 0x141ebe47c
0x141ebe43e: call 0x14067dd80
0x141ebe443: mov r15, rax
0x141ebe446: call 0x142923570
0x141ebe44b: mov rdi, qword ptr [r15]
0x141ebe44e: lock inc qword ptr [rdi]
0x141ebe452: jle 0x141ebe87d
0x141ebe458: mov qword ptr [rbp - 8], rdi
0x141ebe45c: mov qword ptr [rbp], rax
0x141ebe460: mov rcx, rax
0x141ebe463: call 0x1429234f0
0x141ebe468: mov r15d, eax
0x141ebe46b: lock dec qword ptr [rdi]
0x141ebe46f: jne 0x141ebe47f
0x141ebe471: lea rcx, [rbp - 8]
0x141ebe475: call 0x140b85350
0x141ebe47a: jmp 0x141ebe47f
0x141ebe47c: xor r15d, r15d
0x141ebe47f: mov r14, qword ptr [rbp + 0xf0]
0x141ebe486: mov edi, dword ptr [rsi + 0x1c]
0x141ebe489: mov rbx, qword ptr [rsi + 0x10]
0x141ebe48d: mov dword ptr [rbx], edi
0x141ebe48f: mov r13, qword ptr [rsi]
0x141ebe492: mov eax, dword ptr [r13]
0x141ebe496: test eax, eax
0x141ebe498: je 0x141ebe4bc
0x141ebe49a: cmp eax, 1
0x141ebe49d: jne 0x141ebe4cc
0x141ebe49f: mov rax, qword ptr [rbp + 0x58]
0x141ebe4a3: mov rcx, qword ptr [rax]
0x141ebe4a6: call 0x1408a8260
0x141ebe4ab: test al, al
0x141ebe4ad: je 0x141ebe517
0x141ebe4af: mov dword ptr [r13 + 4], 0
0x141ebe4b7: jmp 0x141ebe3c6
0x141ebe4bc: mov rax, qword ptr [r14]
0x141ebe4bf: cmp dword ptr [rax], 2
0x141ebe4c2: jne 0x141ebe4cc
0x141ebe4c4: mov qword ptr [r13], 1
0x141ebe4cc: cmp qword ptr [rbp + 0x38], 0
0x141ebe4d1: je 0x141ebe40f
0x141ebe4d7: call 0x140c18130
0x141ebe4dc: mov rcx, rax
0x141ebe4df: mov rax, qword ptr [rax]
0x141ebe4e2: cmp rax, 1
0x141ebe4e6: jne 0x141ebe736
0x141ebe4ec: add rcx, 8
0x141ebe4f0: cmp qword ptr [rcx], 0
0x141ebe4f4: jne 0x141ebe859
0x141ebe4fa: mov qword ptr [rcx], 0xffffffffffffffff
0x141ebe501: mov qword ptr [rbp + 0x58], rcx
0x141ebe505: add rcx, 8
0x141ebe509: mov rdx, qword ptr [rbp + 0x40]
0x141ebe50d: call 0x140c170f0
0x141ebe512: jmp 0x141ebe408
0x141ebe517: mov rdx, qword ptr [rbp + 0x108]
0x141ebe51e: mov rax, qword ptr [r14]
0x141ebe521: cmp dword ptr [rax], 2
0x141ebe524: jne 0x141ebe5c9
0x141ebe52a: mov rdi, rdx
0x141ebe52d: test r15b, r15b
0x141ebe530: je 0x141ebe62e
0x141ebe536: mov rdx, qword ptr [rbp + 0x28]
0x141ebe53a: mov eax, dword ptr [rdx + 0x1c]
0x141ebe53d: mov rcx, qword ptr [rdx + 0x10]
0x141ebe541: mov dword ptr [rcx], eax
0x141ebe543: mov rdx, qword ptr [rdx]
0x141ebe546: mov byte ptr [rsp + 0x20], 1
0x141ebe54b: lea r8, [rip + 0x10dca98]
0x141ebe552: lea rcx, [rbp - 8]
0x141ebe556: mov r9d, 7
0x141ebe55c: call 0x140f99230
0x141ebe561: mov rax, qword ptr [rbp + 0x48]
0x141ebe565: mov rax, qword ptr [rax]
0x141ebe568: test rax, rax
0x141ebe56b: je 0x141ebe75a
0x141ebe571: cmp rax, 1
0x141ebe575: jne 0x141ebe865
0x141ebe57b: lea r8, [rip + 0x10dcaa6]
0x141ebe582: jmp 0x141ebe761
0x141ebe587: test rax, rax
0x141ebe58a: jne 0x141ebe74e
0x141ebe590: xor edx, edx
0x141ebe592: call 0x1428e6250
0x141ebe597: mov rcx, rax
0x141ebe59a: test rax, rax
0x141ebe59d: jne 0x141ebe3e6
0x141ebe5a3: jmp 0x141ebe74e
0x141ebe5a8: test rax, rax
0x141ebe5ab: jne 0x141ebe74e
0x141ebe5b1: xor edx, edx
0x141ebe5b3: call 0x1428e6250
0x141ebe5b8: mov rcx, rax
0x141ebe5bb: test rax, rax
0x141ebe5be: jne 0x141ebe33a
0x141ebe5c4: jmp 0x141ebe74e
0x141ebe5c9: mov rcx, qword ptr [rbp + 0x100]
0x141ebe5d0: mov eax, dword ptr [rcx + 0x1c]
0x141ebe5d3: mov rcx, qword ptr [rcx + 0x10]
0x141ebe5d7: mov dword ptr [rcx], eax
0x141ebe5d9: mov byte ptr [r12 + 0x206], 1
0x141ebe5e2: mov eax, dword ptr [rdx + 0x1c]
0x141ebe5e5: mov rcx, qword ptr [rdx + 0x10]
0x141ebe5e9: mov dword ptr [rcx], eax
0x141ebe5eb: mov rsi, qword ptr [rdx]
0x141ebe5ee: mov r14, qword ptr [rsi + 0x30]
0x141ebe5f2: mov r15, qword ptr [rsi + 0x40]
0x141ebe5f6: cmp r14, qword ptr [rsi + 0x20]
0x141ebe5fa: jne 0x141ebe60c
0x141ebe5fc: lea rcx, [rsi + 0x20]
0x141ebe600: lea rdx, [rip + 0xcf6c69]
0x141ebe607: call 0x140562870
0x141ebe60c: mov rax, qword ptr [rsi + 0x28]
0x141ebe610: mov qword ptr [rax + r14*8], r15
0x141ebe614: inc r14
0x141ebe617: mov qword ptr [rsi + 0x30], r14
0x141ebe61b: inc qword ptr [rsi + 0x40]
0x141ebe61f: mov dword ptr [rbx], edi
0x141ebe621: mov dword ptr [r13], 2
0x141ebe629: jmp 0x141ebe3c6
0x141ebe62e: mov dword ptr [rbp + 0x54], 0x41900000
0x141ebe635: mov rdx, qword ptr [rbp + 0x28]
0x141ebe639: mov eax, dword ptr [rdx + 0x1c]
0x141ebe63c: mov rcx, qword ptr [rdx + 0x10]
0x141ebe640: mov dword ptr [rcx], eax
0x141ebe642: mov rdx, qword ptr [rdx]
0x141ebe645: mov byte ptr [rsp + 0x20], 1
0x141ebe64a: lea r8, [rip + 0x10dc999]
0x141ebe651: lea rcx, [rbp - 0x30]
0x141ebe655: mov r9d, 7
0x141ebe65b: call 0x140f99230
0x141ebe660: mov rax, qword ptr [rip + 0x176da31]
0x141ebe667: movzx eax, byte ptr [rax]
0x141ebe66a: lea rcx, [rip + 0x10dc98c]
0x141ebe671: lea r8, [rip + 0x10dc979]
0x141ebe678: test rax, rax
0x141ebe67b: cmovne r8, rcx
0x141ebe67f: lea r9, [rax + rax*4]
0x141ebe683: add r9, 0xc
0x141ebe687: mov qword ptr [rsp + 0x20], 0
0x141ebe690: lea rcx, [rbp - 8]
0x141ebe694: lea rdx, [rbp - 0x30]
0x141ebe698: call 0x140f9e970
0x141ebe69d: mov rax, qword ptr [rbp + 0x30]
0x141ebe6a1: mov rax, qword ptr [rax]
0x141ebe6a4: movss xmm6, dword ptr [rax + 0x28]
0x141ebe6a9: addss xmm6, dword ptr [r13 + 4]
0x141ebe6af: movss dword ptr [r13 + 4], xmm6
0x141ebe6b5: lea rcx, [rbp - 8]
0x141ebe6b9: call 0x14068ea60
0x141ebe6be: divss xmm6, dword ptr [rip + 0xa67212]
0x141ebe6c6: xorps xmm0, xmm0
0x141ebe6c9: maxss xmm0, xmm6
0x141ebe6cd: movss xmm1, dword ptr [rip + 0xa6722f]
0x141ebe6d5: minss xmm1, xmm0
0x141ebe6d9: movaps xmm0, xmm1
0x141ebe6dc: mulss xmm0, xmm1
0x141ebe6e0: addss xmm1, xmm1
0x141ebe6e4: movss xmm2, dword ptr [rip + 0xb0e98c]
0x141ebe6ec: subss xmm2, xmm1
0x141ebe6f0: mulss xmm2, xmm0
0x141ebe6f4: movss dword ptr [rax + 0xb0], xmm2
0x141ebe6fc: lea rax, [rbp + 0x54]
0x141ebe700: mov qword ptr [rbp - 0x30], rax
0x141ebe704: mov rax, qword ptr [rbp + 0x100]
0x141ebe70b: mov qword ptr [rbp - 0x28], rax
0x141ebe70f: mov qword ptr [rbp - 0x20], rdi
0x141ebe713: mov qword ptr [rbp - 0x18], rsi
0x141ebe717: lea rdx, [rip + 0x10dc8f0]
0x141ebe71e: lea rcx, [rbp - 8]
0x141ebe722: lea r9, [rbp - 0x30]
0x141ebe726: mov r8d, 7
0x141ebe72c: call 0x140fb97a0
0x141ebe731: jmp 0x141ebe4cc
0x141ebe736: test rax, rax
0x141ebe739: jne 0x141ebe74e
0x141ebe73b: xor edx, edx
0x141ebe73d: call 0x1428e6250
0x141ebe742: mov rcx, rax
0x141ebe745: test rax, rax
0x141ebe748: jne 0x141ebe4f0
0x141ebe74e: lea rcx, [rip + 0xcf6b83]
0x141ebe755: call 0x1428d80d0
0x141ebe75a: lea r8, [rip + 0x10dc8b4]
0x141ebe761: mov qword ptr [rsp + 0x20], 0
0x141ebe76a: lea rcx, [rbp - 0x30]
0x141ebe76e: lea rdx, [rbp - 8]
0x141ebe772: mov r9d, 0x13
0x141ebe778: call 0x140f9e970
0x141ebe77d: mov rax, qword ptr [rbp + 0x48]
0x141ebe781: cmp qword ptr [rax], 0
0x141ebe785: je 0x141ebe79a
0x141ebe787: lea rcx, [rbp - 0x30]
0x141ebe78b: call 0x14068ea60
0x141ebe790: movss xmm0, dword ptr [rip + 0xa6716c]
0x141ebe798: jmp 0x141ebe7f1
0x141ebe79a: mov rax, qword ptr [rbp + 0x30]
0x141ebe79e: mov rax, qword ptr [rax]
0x141ebe7a1: movss xmm6, dword ptr [rax + 0x28]
0x141ebe7a6: addss xmm6, dword ptr [r13 + 4]
0x141ebe7ac: movss dword ptr [r13 + 4], xmm6
0x141ebe7b2: lea rcx, [rbp - 0x30]
0x141ebe7b6: call 0x14068ea60
0x141ebe7bb: divss xmm6, dword ptr [rip + 0xa67115]
0x141ebe7c3: xorps xmm0, xmm0
0x141ebe7c6: maxss xmm0, xmm6
0x141ebe7ca: movss xmm1, dword ptr [rip + 0xa67132]
0x141ebe7d2: minss xmm1, xmm0
0x141ebe7d6: movaps xmm2, xmm1
0x141ebe7d9: mulss xmm2, xmm1
0x141ebe7dd: addss xmm1, xmm1
0x141ebe7e1: movss xmm0, dword ptr [rip + 0xb0e88f]
0x141ebe7e9: subss xmm0, xmm1
0x141ebe7ed: mulss xmm0, xmm2
0x141ebe7f1: movss dword ptr [rax + 0xb0], xmm0
0x141ebe7f9: mov dword ptr [rbp + 0x54], 0x41d80000
0x141ebe800: mov rax, qword ptr [rbp + 0x48]
0x141ebe804: cmp qword ptr [rax], 1
0x141ebe808: sete byte ptr [rbp + 0x67]
0x141ebe80c: lea rax, [rbp + 0x67]
0x141ebe810: mov qword ptr [rbp - 8], rax
0x141ebe814: lea rax, [rbp + 0x54]
0x141ebe818: mov qword ptr [rbp], rax
0x141ebe81c: mov rax, qword ptr [rbp + 0x100]
0x141ebe823: mov qword ptr [rbp + 8], rax
0x141ebe827: mov qword ptr [rbp + 0x10], rdi
0x141ebe82b: mov qword ptr [rbp + 0x18], rsi
0x141ebe82f: lea rax, [rbp + 0x118]
0x141ebe836: mov qword ptr [rbp + 0x20], rax
0x141ebe83a: lea rdx, [rip + 0x10dc7cd]
0x141ebe841: lea rcx, [rbp - 0x30]
0x141ebe845: lea r9, [rbp - 8]
0x141ebe849: mov r8d, 7
0x141ebe84f: call 0x140fbf9a0
0x141ebe854: jmp 0x141ebe4cc
0x141ebe859: lea rcx, [rip + 0xcf6ae8]
0x141ebe860: call 0x1428d9250
0x141ebe865: lea rcx, [rip + 0x10dc7cf]
0x141ebe86c: lea r8, [rip + 0x10dc7f5]
0x141ebe873: mov edx, 0x28
0x141ebe878: call 0x1428d9430
0x141ebe87d: ud2 
0x141ebe87f: nop 
0x141ebe880: mov qword ptr [rsp + 0x10], rdx
0x141ebe885: push rbp
0x141ebe886: push r15
0x141ebe888: push r14
0x141ebe88a: push r13
0x141ebe88c: push r12
0x141ebe88e: push rsi
0x141ebe88f: push rdi
0x141ebe890: push rbx
0x141ebe891: sub rsp, 0x48
0x141ebe895: lea rbp, [rdx + 0x80]
0x141ebe89c: movaps xmmword ptr [rsp + 0x30], xmm6
0x141ebe8a1: mov rax, qword ptr [rbp + 0x48]
0x141ebe8a5: inc qword ptr [rax]
0x141ebe8a8: movaps xmm6, xmmword ptr [rsp + 0x30]
0x141ebe8ad: add rsp, 0x48
0x141ebe8b1: pop rbx
0x141ebe8b2: pop rdi
0x141ebe8b3: pop rsi
0x141ebe8b4: pop r12
0x141ebe8b6: pop r13
0x141ebe8b8: pop r14
0x141ebe8ba: pop r15
0x141ebe8bc: pop rbp
0x141ebe8bd: ret 
0x141ebe8be: nop 
0x141ebe8c0: mov qword ptr [rsp + 0x10], rdx
0x141ebe8c5: push rbp
0x141ebe8c6: push r15
0x141ebe8c8: push r14
0x141ebe8ca: push r13
0x141ebe8cc: push r12
0x141ebe8ce: push rsi
0x141ebe8cf: push rdi
0x141ebe8d0: push rbx
0x141ebe8d1: sub rsp, 0x48
0x141ebe8d5: lea rbp, [rdx + 0x80]
0x141ebe8dc: movaps xmmword ptr [rsp + 0x30], xmm6
0x141ebe8e1: cmp qword ptr [rbp + 0x38], 0
0x141ebe8e6: je 0x141ebe908
0x141ebe8e8: lea rax, [rbp + 0x40]
0x141ebe8ec: mov qword ptr [rbp - 0x38], rax
0x141ebe8f0: lea rax, [rbp - 0x38]
0x141ebe8f4: mov qword ptr [rbp - 0x40], rax
0x141ebe8f8: lea rcx, [rip + 0x10dc5f9]
0x141ebe8ff: lea rdx, [rbp - 0x40]
0x141ebe903: call 0x140fcfc20
0x141ebe908: movaps xmm6, xmmword ptr [rsp + 0x30]
0x141ebe90d: add rsp, 0x48
0x141ebe911: pop rbx
0x141ebe912: pop rdi
0x141ebe913: pop rsi
0x141ebe914: pop r12
0x141ebe916: pop r13
0x141ebe918: pop r14
0x141ebe91a: pop r15
0x141ebe91c: pop rbp
0x141ebe91d: ret 
0x141ebe91e: nop 
0x141ebe920: mov qword ptr [rsp + 0x10], rdx
0x141ebe925: push rbp
0x141ebe926: push r15
0x141ebe928: push r14
0x141ebe92a: push r13
0x141ebe92c: push r12
0x141ebe92e: push rsi
0x141ebe92f: push rdi
0x141ebe930: push rbx
0x141ebe931: sub rsp, 0x48
0x141ebe935: lea rbp, [rdx + 0x80]
0x141ebe93c: movaps xmmword ptr [rsp + 0x30], xmm6
0x141ebe941: mov rax, qword ptr [rbp + 0x58]
0x141ebe945: inc qword ptr [rax]
0x141ebe948: movaps xmm6, xmmword ptr [rsp + 0x30]
0x141ebe94d: add rsp, 0x48
0x141ebe951: pop rbx
0x141ebe952: pop rdi
0x141ebe953: pop rsi
0x141ebe954: pop r12
0x141ebe956: pop r13
0x141ebe958: pop r14
0x141ebe95a: pop r15
0x141ebe95c: pop rbp
0x141ebe95d: ret 
0x141ebe95e: nop 
0x141ebe960: mov qword ptr [rsp + 0x10], rdx
0x141ebe965: push rbp
0x141ebe966: push r15
0x141ebe968: push r14
0x141ebe96a: push r13
0x141ebe96c: push r12
0x141ebe96e: push rsi
0x141ebe96f: push rdi
0x141ebe970: push rbx
0x141ebe971: sub rsp, 0x48
0x141ebe975: lea rbp, [rdx + 0x80]
0x141ebe97c: movaps xmmword ptr [rsp + 0x30], xmm6
0x141ebe981: mov rax, qword ptr [rbp + 0x58]
0x141ebe985: inc qword ptr [rax]
0x141ebe988: movaps xmm6, xmmword ptr [rsp + 0x30]
0x141ebe98d: add rsp, 0x48
0x141ebe991: pop rbx
0x141ebe992: pop rdi
0x141ebe993: pop rsi
0x141ebe994: pop r12
0x141ebe996: pop r13
0x141ebe998: pop r14
0x141ebe99a: pop r15
0x141ebe99c: pop rbp
0x141ebe99d: ret 
