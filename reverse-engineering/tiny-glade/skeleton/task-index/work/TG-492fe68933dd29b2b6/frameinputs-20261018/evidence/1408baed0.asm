1408baed0: push rbp
1408baed1: push r15
1408baed3: push r14
1408baed5: push r13
1408baed7: push r12
1408baed9: push rsi
1408baeda: push rdi
1408baedb: push rbx
1408baedc: sub rsp, 0xa8
1408baee3: lea rbp, [rsp + 0x80]
1408baeeb: mov qword ptr [rbp + 0x20], 0xfffffffffffffffe
1408baef3: mov qword ptr [rbp - 0x58], rcx
1408baef7: mov qword ptr [rbp - 8], 0
1408baeff: mov qword ptr [rbp], 4
1408baf07: mov qword ptr [rbp + 8], 0
1408baf0f: mov qword ptr [rbp - 0x20], 0
1408baf17: mov qword ptr [rbp - 0x18], 4
1408baf1f: mov qword ptr [rbp - 0x10], 0
1408baf27: mov qword ptr [rbp - 0x38], 0
1408baf2f: mov qword ptr [rbp - 0x30], 4
1408baf37: mov qword ptr [rbp - 0x28], 0
1408baf3f: mov qword ptr [rbp - 0x50], 0
1408baf47: mov qword ptr [rbp - 0x48], 4
1408baf4f: mov qword ptr [rbp - 0x40], 0
1408baf57: xor ebx, ebx
1408baf59: mov dl, 2
1408baf5b: movzx r15d, bl
1408baf5f: lea rcx, [rip + 0x21e6642]
1408baf66: movsxd rax, dword ptr [rcx + r15*4]
1408baf6a: add rax, rcx
1408baf6d: mov dword ptr [rbp + 0x18], 0
1408baf74: mov dword ptr [rbp + 0x1c], 0
1408baf7b: xor esi, esi
1408baf7d: jmp rax
1408baf7f: mov dl, 4
1408baf81: jmp 0x1408bb090
1408baf86: mov dl, 5
1408baf88: jmp 0x1408bb090
1408baf8d: mov dl, 6
1408baf8f: jmp 0x1408bb090
1408baf94: mov dl, 7
1408baf96: jmp 0x1408bb090
1408baf9b: mov dl, 8
1408baf9d: jmp 0x1408bb090
1408bafa2: mov dl, 9
1408bafa4: jmp 0x1408bb090
1408bafa9: mov dl, 0xa
1408bafab: jmp 0x1408bb090
1408bafb0: mov dl, 0xb
1408bafb2: jmp 0x1408bb090
1408bafb7: mov dl, 0xc
1408bafb9: jmp 0x1408bb090
1408bafbe: mov dl, 0xd
1408bafc0: jmp 0x1408bb090
1408bafc5: mov dl, 0xe
1408bafc7: jmp 0x1408bb090
1408bafcc: mov dl, 0xf
1408bafce: jmp 0x1408bb090
1408bafd3: mov dl, 0x10
1408bafd5: jmp 0x1408bb090
1408bafda: mov dl, 0x11
1408bafdc: jmp 0x1408bb090
1408bafe1: mov dl, 0x12
1408bafe3: jmp 0x1408bb090
1408bafe8: mov dl, 0x13
1408bafea: jmp 0x1408bb090
1408bafef: mov dl, 0x14
1408baff1: jmp 0x1408bb090
1408baff6: mov dl, 0x15
1408baff8: jmp 0x1408bb090
1408baffd: mov dl, 0x16
1408bafff: jmp 0x1408bb090
1408bb004: mov dl, 0x17
1408bb006: jmp 0x1408bb090
1408bb00b: mov dl, 0x18
1408bb00d: jmp 0x1408bb090
1408bb012: mov dl, 0x19
1408bb014: jmp 0x1408bb090
1408bb016: mov dl, 0x1a
1408bb018: jmp 0x1408bb090
1408bb01a: mov dl, 0x1b
1408bb01c: jmp 0x1408bb090
1408bb01e: mov dl, 1
1408bb020: jmp 0x1408bb090
1408bb022: mov rcx, qword ptr [rbp + 8]
1408bb026: mov rax, qword ptr [rbp - 0x58]
1408bb02a: mov qword ptr [rax + 0x10], rcx
1408bb02e: movups xmm0, xmmword ptr [rbp - 8]
1408bb032: movups xmmword ptr [rax], xmm0
1408bb035: mov rcx, qword ptr [rbp - 0x10]
1408bb039: mov qword ptr [rax + 0x28], rcx
1408bb03d: movups xmm0, xmmword ptr [rbp - 0x20]
1408bb041: movups xmmword ptr [rax + 0x18], xmm0
1408bb045: movups xmm0, xmmword ptr [rbp - 0x38]
1408bb049: movups xmmword ptr [rax + 0x30], xmm0
1408bb04d: mov rcx, qword ptr [rbp - 0x28]
1408bb051: mov qword ptr [rax + 0x40], rcx
1408bb055: movups xmm0, xmmword ptr [rbp - 0x50]
1408bb059: movups xmmword ptr [rax + 0x48], xmm0
1408bb05d: mov rcx, qword ptr [rbp - 0x40]
1408bb061: mov qword ptr [rax + 0x58], rcx
1408bb065: mov dword ptr [rax + 0x60], ebx
1408bb068: mov ecx, dword ptr [rbp + 0x18]
1408bb06b: mov dword ptr [rax + 0x64], ecx
1408bb06e: mov ecx, dword ptr [rbp + 0x1c]
1408bb071: mov dword ptr [rax + 0x68], ecx
1408bb074: mov dword ptr [rax + 0x6c], esi
1408bb077: add rsp, 0xa8
1408bb07e: pop rbx
1408bb07f: pop rdi
1408bb080: pop rsi
1408bb081: pop r12
1408bb083: pop r13
1408bb085: pop r14
1408bb087: pop r15
1408bb089: pop rbp
1408bb08a: ret 
1408bb08b: mov dl, 3
1408bb08d: nop dword ptr [rax]
1408bb090: mov byte ptr [rbp + 0x17], dl
1408bb093: movsx r14, r15b
1408bb097: lea rax, [rip + 0x21e6422]
1408bb09e: mov r12d, dword ptr [rax + r14*4]
1408bb0a2: mov rdi, qword ptr [rbp + 8]
1408bb0a6: cmp rdi, qword ptr [rbp - 8]
1408bb0aa: jne 0x1408bb0bc
1408bb0ac: lea rdx, [rip + 0x21e5875]
1408bb0b3: lea rcx, [rbp - 8]
1408bb0b7: call 0x1416a83a0
1408bb0bc: cmp r15b, 2
1408bb0c0: setae cl
1408bb0c3: mov rax, qword ptr [rbp]
1408bb0c7: mov dword ptr [rax + rdi*4], ebx
1408bb0ca: inc rdi
1408bb0cd: mov qword ptr [rbp + 8], rdi
1408bb0d1: add r12d, ebx
1408bb0d4: movzx eax, byte ptr [rip + 0x2d79cc1]
1408bb0db: test al, al
1408bb0dd: jne 0x1408bb100
1408bb0df: or cl, byte ptr [rip + 0x2d79cb8]
1408bb0e5: mov edi, 0x40
1408bb0ea: test cl, 1
1408bb0ed: je 0x1408bb100
1408bb0ef: mov r13, qword ptr [rbp - 0x10]
1408bb0f3: cmp r13, qword ptr [rbp - 0x20]
1408bb0f7: je 0x1408bb115
1408bb0f9: jmp 0x1408bb12c
1408bb0fb: nop dword ptr [rax + rax]
1408bb100: lea rcx, [rip + 0x21e63b9]
1408bb107: mov edi, dword ptr [rcx + r14*4]
1408bb10b: mov r13, qword ptr [rbp - 0x10]
1408bb10f: cmp r13, qword ptr [rbp - 0x20]
1408bb113: jne 0x1408bb12c
1408bb115: lea rdx, [rip + 0x21e5824]
1408bb11c: lea rcx, [rbp - 0x20]
1408bb120: call 0x1416a83a0
1408bb125: movzx eax, byte ptr [rip + 0x2d79c70]
1408bb12c: mov rcx, qword ptr [rbp - 0x18]
1408bb130: mov edx, dword ptr [rbp + 0x18]
1408bb133: mov dword ptr [rcx + r13*4], edx
1408bb137: inc r13
1408bb13a: add edx, edi
1408bb13c: mov dword ptr [rbp + 0x18], edx
1408bb13f: shr edi, 5
1408bb142: xor ecx, ecx
1408bb144: test byte ptr [rip + 0x2d79c52], 1
1408bb14b: mov ebx, edi
1408bb14d: cmovne ebx, ecx
1408bb150: cmp r15b, 2
1408bb154: cmovae ebx, ecx
1408bb157: test al, 1
1408bb159: cmovne ebx, edi
1408bb15c: mov qword ptr [rbp - 0x10], r13
1408bb160: mov rdi, qword ptr [rbp - 0x28]
1408bb164: cmp rdi, qword ptr [rbp - 0x38]
1408bb168: jne 0x1408bb17a
1408bb16a: lea rdx, [rip + 0x21e57e7]
1408bb171: lea rcx, [rbp - 0x38]
1408bb175: call 0x1416a83a0
1408bb17a: mov rax, qword ptr [rbp - 0x30]
1408bb17e: mov ecx, dword ptr [rbp + 0x1c]
1408bb181: mov dword ptr [rax + rdi*4], ecx
1408bb184: inc rdi
1408bb187: mov qword ptr [rbp - 0x28], rdi
1408bb18b: add ecx, ebx
1408bb18d: mov dword ptr [rbp + 0x1c], ecx
1408bb190: mov r13d, ebx
1408bb193: shr r13d, 0xa
1408bb197: xor edi, edi
1408bb199: test ebx, 0x3fe
1408bb19f: setne dil
1408bb1a3: mov rbx, qword ptr [rbp - 0x40]
1408bb1a7: cmp rbx, qword ptr [rbp - 0x50]
1408bb1ab: jne 0x1408bb1bd
1408bb1ad: lea rdx, [rip + 0x21e57bc]
1408bb1b4: lea rcx, [rbp - 0x50]
1408bb1b8: call 0x1416a83a0
1408bb1bd: mov rax, qword ptr [rbp - 0x48]
1408bb1c1: mov dword ptr [rax + rbx*4], esi
1408bb1c4: inc rbx
1408bb1c7: mov qword ptr [rbp - 0x40], rbx
1408bb1cb: add r13d, esi
1408bb1ce: add r13d, edi
1408bb1d1: lea rax, [rip + 0x21e62e8]
1408bb1d8: mov ebx, dword ptr [rax + r14*4]
1408bb1dc: mov rsi, qword ptr [rbp + 8]
1408bb1e0: cmp rsi, qword ptr [rbp - 8]
1408bb1e4: jne 0x1408bb1f6
1408bb1e6: lea rdx, [rip + 0x21e573b]
1408bb1ed: lea rcx, [rbp - 8]
1408bb1f1: call 0x1416a83a0
1408bb1f6: cmp r15b, 2
1408bb1fa: setae cl
1408bb1fd: mov rax, qword ptr [rbp]
1408bb201: mov dword ptr [rax + rsi*4], r12d
1408bb205: inc rsi
1408bb208: mov qword ptr [rbp + 8], rsi
1408bb20c: add ebx, r12d
1408bb20f: movzx eax, byte ptr [rip + 0x2d79b86]
1408bb216: test al, al
1408bb218: jne 0x1408bb240
1408bb21a: or cl, byte ptr [rip + 0x2d79b7d]
1408bb220: mov esi, 0x40
1408bb225: test cl, 1
1408bb228: je 0x1408bb240
1408bb22a: mov r12, qword ptr [rbp - 0x10]
1408bb22e: cmp r12, qword ptr [rbp - 0x20]
1408bb232: je 0x1408bb255
1408bb234: jmp 0x1408bb26c
1408bb236: nop word ptr cs:[rax + rax]
1408bb240: lea rcx, [rip + 0x21e6279]
1408bb247: mov esi, dword ptr [rcx + r14*4]
1408bb24b: mov r12, qword ptr [rbp - 0x10]
1408bb24f: cmp r12, qword ptr [rbp - 0x20]
1408bb253: jne 0x1408bb26c
1408bb255: lea rdx, [rip + 0x21e56e4]
1408bb25c: lea rcx, [rbp - 0x20]
1408bb260: call 0x1416a83a0
1408bb265: movzx eax, byte ptr [rip + 0x2d79b30]
1408bb26c: mov rcx, qword ptr [rbp - 0x18]
1408bb270: mov edx, dword ptr [rbp + 0x18]
1408bb273: mov dword ptr [rcx + r12*4], edx
1408bb277: inc r12
1408bb27a: add edx, esi
1408bb27c: mov dword ptr [rbp + 0x18], edx
1408bb27f: shr esi, 5
1408bb282: xor ecx, ecx
1408bb284: test byte ptr [rip + 0x2d79b12], 1
1408bb28b: mov edi, esi
1408bb28d: cmovne edi, ecx
1408bb290: cmp r15b, 2
1408bb294: cmovae edi, ecx
1408bb297: test al, 1
1408bb299: cmovne edi, esi
1408bb29c: mov qword ptr [rbp - 0x10], r12
1408bb2a0: mov rsi, qword ptr [rbp - 0x28]
1408bb2a4: cmp rsi, qword ptr [rbp - 0x38]
1408bb2a8: jne 0x1408bb2ba
1408bb2aa: lea rdx, [rip + 0x21e56a7]
1408bb2b1: lea rcx, [rbp - 0x38]
1408bb2b5: call 0x1416a83a0
1408bb2ba: mov rax, qword ptr [rbp - 0x30]
1408bb2be: mov ecx, dword ptr [rbp + 0x1c]
1408bb2c1: mov dword ptr [rax + rsi*4], ecx
1408bb2c4: inc rsi
1408bb2c7: mov qword ptr [rbp - 0x28], rsi
1408bb2cb: add ecx, edi
1408bb2cd: mov dword ptr [rbp + 0x1c], ecx
1408bb2d0: mov esi, edi
1408bb2d2: shr esi, 0xa
1408bb2d5: xor r12d, r12d
1408bb2d8: test edi, 0x3fe
1408bb2de: setne r12b
1408bb2e2: mov rdi, qword ptr [rbp - 0x40]
1408bb2e6: cmp rdi, qword ptr [rbp - 0x50]
1408bb2ea: jne 0x1408bb2fc
1408bb2ec: lea rdx, [rip + 0x21e567d]
1408bb2f3: lea rcx, [rbp - 0x50]
1408bb2f7: call 0x1416a83a0
1408bb2fc: mov rax, qword ptr [rbp - 0x48]
1408bb300: mov dword ptr [rax + rdi*4], r13d
1408bb304: inc rdi
1408bb307: mov qword ptr [rbp - 0x40], rdi
1408bb30b: add esi, r13d
1408bb30e: add esi, r12d
1408bb311: lea rax, [rip + 0x21e61a8]
1408bb318: mov r13d, dword ptr [rax + r14*4]
1408bb31c: mov rdi, qword ptr [rbp + 8]
1408bb320: cmp rdi, qword ptr [rbp - 8]
1408bb324: jne 0x1408bb336
1408bb326: lea rdx, [rip + 0x21e55fb]
1408bb32d: lea rcx, [rbp - 8]
1408bb331: call 0x1416a83a0
1408bb336: cmp r15b, 2
1408bb33a: setae cl
1408bb33d: mov rax, qword ptr [rbp]
1408bb341: mov dword ptr [rax + rdi*4], ebx
1408bb344: inc rdi
1408bb347: mov qword ptr [rbp + 8], rdi
1408bb34b: add r13d, ebx
1408bb34e: movzx eax, byte ptr [rip + 0x2d79a47]
1408bb355: test al, al
1408bb357: jne 0x1408bb380
1408bb359: or cl, byte ptr [rip + 0x2d79a3e]
1408bb35f: mov ebx, 0x40
1408bb364: test cl, 1
1408bb367: je 0x1408bb380
1408bb369: mov r12, qword ptr [rbp - 0x10]
1408bb36d: cmp r12, qword ptr [rbp - 0x20]
1408bb371: je 0x1408bb395
1408bb373: jmp 0x1408bb3ac
1408bb375: nop word ptr cs:[rax + rax]
1408bb380: lea rcx, [rip + 0x21e6139]
1408bb387: mov ebx, dword ptr [rcx + r14*4]
1408bb38b: mov r12, qword ptr [rbp - 0x10]
1408bb38f: cmp r12, qword ptr [rbp - 0x20]
1408bb393: jne 0x1408bb3ac
1408bb395: lea rdx, [rip + 0x21e55a4]
1408bb39c: lea rcx, [rbp - 0x20]
1408bb3a0: call 0x1416a83a0
1408bb3a5: movzx eax, byte ptr [rip + 0x2d799f0]
1408bb3ac: mov rcx, qword ptr [rbp - 0x18]
1408bb3b0: mov edx, dword ptr [rbp + 0x18]
1408bb3b3: mov dword ptr [rcx + r12*4], edx
1408bb3b7: inc r12
1408bb3ba: add edx, ebx
1408bb3bc: mov dword ptr [rbp + 0x18], edx
1408bb3bf: shr ebx, 5
1408bb3c2: xor ecx, ecx
1408bb3c4: test byte ptr [rip + 0x2d799d2], 1
1408bb3cb: mov edi, ebx
1408bb3cd: cmovne edi, ecx
1408bb3d0: cmp r15b, 2
1408bb3d4: cmovae edi, ecx
1408bb3d7: test al, 1
1408bb3d9: cmovne edi, ebx
1408bb3dc: mov qword ptr [rbp - 0x10], r12
1408bb3e0: mov rbx, qword ptr [rbp - 0x28]
1408bb3e4: cmp rbx, qword ptr [rbp - 0x38]
1408bb3e8: jne 0x1408bb3fa
1408bb3ea: lea rdx, [rip + 0x21e5567]
1408bb3f1: lea rcx, [rbp - 0x38]
1408bb3f5: call 0x1416a83a0
1408bb3fa: mov rax, qword ptr [rbp - 0x30]
1408bb3fe: mov ecx, dword ptr [rbp + 0x1c]
1408bb401: mov dword ptr [rax + rbx*4], ecx
1408bb404: inc rbx
1408bb407: mov qword ptr [rbp - 0x28], rbx
1408bb40b: add ecx, edi
1408bb40d: mov dword ptr [rbp + 0x1c], ecx
1408bb410: mov r12d, edi
1408bb413: shr r12d, 0xa
1408bb417: xor ebx, ebx
1408bb419: test edi, 0x3fe
1408bb41f: setne bl
1408bb422: mov rdi, qword ptr [rbp - 0x40]
1408bb426: cmp rdi, qword ptr [rbp - 0x50]
1408bb42a: jne 0x1408bb43c
1408bb42c: lea rdx, [rip + 0x21e553d]
1408bb433: lea rcx, [rbp - 0x50]
1408bb437: call 0x1416a83a0
1408bb43c: mov rax, qword ptr [rbp - 0x48]
1408bb440: mov dword ptr [rax + rdi*4], esi
1408bb443: inc rdi
1408bb446: mov qword ptr [rbp - 0x40], rdi
1408bb44a: add r12d, esi
1408bb44d: add r12d, ebx
1408bb450: lea rax, [rip + 0x21e6069]
1408bb457: mov ebx, dword ptr [rax + r14*4]
1408bb45b: mov rsi, qword ptr [rbp + 8]
1408bb45f: cmp rsi, qword ptr [rbp - 8]
1408bb463: jne 0x1408bb475
1408bb465: lea rdx, [rip + 0x21e54bc]
1408bb46c: lea rcx, [rbp - 8]
1408bb470: call 0x1416a83a0
1408bb475: cmp r15b, 2
1408bb479: setae cl
1408bb47c: mov rax, qword ptr [rbp]
1408bb480: mov dword ptr [rax + rsi*4], r13d
1408bb484: inc rsi
1408bb487: mov qword ptr [rbp + 8], rsi
1408bb48b: add ebx, r13d
1408bb48e: movzx eax, byte ptr [rip + 0x2d79907]
1408bb495: test al, al
1408bb497: jne 0x1408bb4c0
1408bb499: or cl, byte ptr [rip + 0x2d798fe]
1408bb49f: mov esi, 0x40
1408bb4a4: test cl, 1
1408bb4a7: je 0x1408bb4c0
1408bb4a9: mov r14, qword ptr [rbp - 0x10]
1408bb4ad: cmp r14, qword ptr [rbp - 0x20]
1408bb4b1: je 0x1408bb4d5
1408bb4b3: jmp 0x1408bb4ec
1408bb4b5: nop word ptr cs:[rax + rax]
1408bb4c0: lea rcx, [rip + 0x21e5ff9]
1408bb4c7: mov esi, dword ptr [rcx + r14*4]
1408bb4cb: mov r14, qword ptr [rbp - 0x10]
1408bb4cf: cmp r14, qword ptr [rbp - 0x20]
1408bb4d3: jne 0x1408bb4ec
1408bb4d5: lea rdx, [rip + 0x21e5464]
1408bb4dc: lea rcx, [rbp - 0x20]
1408bb4e0: call 0x1416a83a0
1408bb4e5: movzx eax, byte ptr [rip + 0x2d798b0]
1408bb4ec: mov rcx, qword ptr [rbp - 0x18]
1408bb4f0: mov edx, dword ptr [rbp + 0x18]
1408bb4f3: mov dword ptr [rcx + r14*4], edx
1408bb4f7: inc r14
1408bb4fa: add edx, esi
1408bb4fc: mov dword ptr [rbp + 0x18], edx
1408bb4ff: shr esi, 5
1408bb502: xor ecx, ecx
1408bb504: test byte ptr [rip + 0x2d79892], 1
1408bb50b: mov edi, esi
1408bb50d: cmovne edi, ecx
1408bb510: cmp r15b, 2
1408bb514: cmovae edi, ecx
1408bb517: test al, 1
1408bb519: cmovne edi, esi
1408bb51c: mov qword ptr [rbp - 0x10], r14
1408bb520: mov rsi, qword ptr [rbp - 0x28]
1408bb524: cmp rsi, qword ptr [rbp - 0x38]
1408bb528: jne 0x1408bb53a
1408bb52a: lea rdx, [rip + 0x21e5427]
1408bb531: lea rcx, [rbp - 0x38]
1408bb535: call 0x1416a83a0
1408bb53a: mov rax, qword ptr [rbp - 0x30]
1408bb53e: mov ecx, dword ptr [rbp + 0x1c]
1408bb541: mov dword ptr [rax + rsi*4], ecx
1408bb544: inc rsi
1408bb547: mov qword ptr [rbp - 0x28], rsi
1408bb54b: add ecx, edi
1408bb54d: mov dword ptr [rbp + 0x1c], ecx
1408bb550: mov esi, edi
1408bb552: shr esi, 0xa
1408bb555: xor r14d, r14d
1408bb558: test edi, 0x3fe
1408bb55e: setne r14b
1408bb562: mov rdi, qword ptr [rbp - 0x40]
1408bb566: cmp rdi, qword ptr [rbp - 0x50]
1408bb56a: jne 0x1408bb57c
1408bb56c: lea rdx, [rip + 0x21e53fd]
1408bb573: lea rcx, [rbp - 0x50]
1408bb577: call 0x1416a83a0
1408bb57c: mov rax, qword ptr [rbp - 0x48]
1408bb580: mov dword ptr [rax + rdi*4], r12d
1408bb584: inc rdi
1408bb587: mov qword ptr [rbp - 0x40], rdi
1408bb58b: add esi, r12d
1408bb58e: add esi, r14d
1408bb591: movzx r15d, byte ptr [rbp + 0x17]
1408bb596: lea rcx, [rip + 0x21e600b]
1408bb59d: movsxd rax, dword ptr [rcx + r15*4]
1408bb5a1: add rax, rcx
1408bb5a4: mov dl, 2
1408bb5a6: jmp rax
1408bb5a8: nop dword ptr [rax + rax]
1408bb5b0: mov qword ptr [rsp + 0x10], rdx
1408bb5b5: push rbp
1408bb5b6: push r15
1408bb5b8: push r14
1408bb5ba: push r13
1408bb5bc: push r12
1408bb5be: push rsi
1408bb5bf: push rdi
1408bb5c0: push rbx
1408bb5c1: sub rsp, 0x28
1408bb5c5: lea rbp, [rdx + 0x80]
1408bb5cc: mov rdx, qword ptr [rbp - 0x50]
1408bb5d0: test rdx, rdx
1408bb5d3: je 0x1408bb5e8
1408bb5d5: mov rcx, qword ptr [rbp - 0x48]
1408bb5d9: shl rdx, 2
1408bb5dd: mov r8d, 4
1408bb5e3: call 0x140613c20
1408bb5e8: mov rdx, qword ptr [rbp - 0x38]
1408bb5ec: test rdx, rdx
1408bb5ef: je 0x1408bb604
1408bb5f1: mov rcx, qword ptr [rbp - 0x30]
1408bb5f5: shl rdx, 2
1408bb5f9: mov r8d, 4
1408bb5ff: call 0x140613c20
1408bb604: mov rdx, qword ptr [rbp - 0x20]
1408bb608: test rdx, rdx
1408bb60b: je 0x1408bb620
1408bb60d: mov rcx, qword ptr [rbp - 0x18]
1408bb611: shl rdx, 2
1408bb615: mov r8d, 4
1408bb61b: call 0x140613c20
1408bb620: mov rdx, qword ptr [rbp - 8]
1408bb624: test rdx, rdx
1408bb627: je 0x1408bb63c
1408bb629: mov rcx, qword ptr [rbp]
1408bb62d: shl rdx, 2
1408bb631: mov r8d, 4
1408bb637: call 0x140613c20
1408bb63c: nop 
1408bb63d: add rsp, 0x28
1408bb641: pop rbx
1408bb642: pop rdi
1408bb643: pop rsi
1408bb644: pop r12
1408bb646: pop r13
1408bb648: pop r14
1408bb64a: pop r15
1408bb64c: pop rbp
1408bb64d: ret 
