0x140341370: push rbp
0x140341371: push rsi
0x140341372: push rdi
0x140341373: sub rsp, 0xe0
0x14034137a: lea rbp, [rsp + 0x80]
0x140341382: movaps xmmword ptr [rbp + 0x50], xmm7
0x140341386: movaps xmmword ptr [rbp + 0x40], xmm6
0x14034138a: mov qword ptr [rbp + 0x38], 0xfffffffffffffffe
0x140341392: mov rsi, rcx
0x140341395: call 0x140d4d740
0x14034139a: mov rdi, rax
0x14034139d: test byte ptr [rax], 1
0x1403413a0: je 0x1403413b3
0x1403413a2: mov rcx, rdi
0x1403413a5: add rcx, 8
0x1403413a9: mov rax, qword ptr [rdi + 8]
0x1403413ad: mov rdx, qword ptr [rdi + 0x10]
0x1403413b1: jmp 0x1403413ce
0x1403413b3: call 0x140d64e00
0x1403413b8: mov rcx, rdi
0x1403413bb: add rcx, 8
0x1403413bf: mov qword ptr [rdi], 1
0x1403413c6: mov qword ptr [rdi + 8], rax
0x1403413ca: mov qword ptr [rdi + 0x10], rdx
0x1403413ce: lea r8, [rax + 1]
0x1403413d2: mov qword ptr [rcx], r8
0x1403413d5: movups xmm6, xmmword ptr [rip + 0x2681a84]
0x1403413dc: movaps xmmword ptr [rbp + 0x10], xmm6
0x1403413e0: movups xmm7, xmmword ptr [rip + 0x2681a69]
0x1403413e7: movaps xmmword ptr [rbp], xmm7
0x1403413eb: mov qword ptr [rbp + 0x20], rax
0x1403413ef: mov qword ptr [rbp + 0x28], rdx
0x1403413f3: xor ecx, ecx
0x1403413f5: call 0x1400d9b20
0x1403413fa: test rax, rax
0x1403413fd: je 0x140341549
0x140341403: movups xmm0, xmmword ptr [rax]
0x140341406: inc qword ptr [rax]
0x140341409: movaps xmmword ptr [rbp - 0x40], xmm0
0x14034140d: movups xmm0, xmmword ptr [rip + 0x266ec0c]
0x140341414: movaps xmmword ptr [rbp - 0x60], xmm0
0x140341418: movups xmm0, xmmword ptr [rip + 0x266ec11]
0x14034141f: movaps xmmword ptr [rbp - 0x50], xmm0
0x140341423: call 0x140d4d740
0x140341428: mov rdi, rax
0x14034142b: test byte ptr [rax], 1
0x14034142e: je 0x140341441
0x140341430: mov rcx, rdi
0x140341433: add rcx, 8
0x140341437: mov rax, qword ptr [rdi + 8]
0x14034143b: mov rdx, qword ptr [rdi + 0x10]
0x14034143f: jmp 0x14034145c
0x140341441: call 0x140d64e00
0x140341446: mov rcx, rdi
0x140341449: add rcx, 8
0x14034144d: mov qword ptr [rdi], 1
0x140341454: mov qword ptr [rdi + 8], rax
0x140341458: mov qword ptr [rdi + 0x10], rdx
0x14034145c: lea r8, [rax + 1]
0x140341460: mov qword ptr [rcx], r8
0x140341463: movaps xmmword ptr [rbp - 0x20], xmm6
0x140341467: movaps xmmword ptr [rbp - 0x30], xmm7
0x14034146b: mov qword ptr [rbp - 0x10], rax
0x14034146f: mov qword ptr [rbp - 8], rdx
0x140341473: call 0x140d4d740
0x140341478: mov rdi, rax
0x14034147b: test byte ptr [rax], 1
0x14034147e: je 0x140341491
0x140341480: mov rcx, rdi
0x140341483: add rcx, 8
0x140341487: mov rax, qword ptr [rdi + 8]
0x14034148b: mov rdx, qword ptr [rdi + 0x10]
0x14034148f: jmp 0x1403414ac
0x140341491: call 0x140d64e00
0x140341496: mov rcx, rdi
0x140341499: add rcx, 8
0x14034149d: mov qword ptr [rdi], 1
0x1403414a4: mov qword ptr [rdi + 8], rax
0x1403414a8: mov qword ptr [rdi + 0x10], rdx
0x1403414ac: lea r8, [rax + 1]
0x1403414b0: mov qword ptr [rcx], r8
0x1403414b3: movaps xmm0, xmmword ptr [rbp]
0x1403414b7: movaps xmm1, xmmword ptr [rbp + 0x10]
0x1403414bb: movaps xmm2, xmmword ptr [rbp + 0x20]
0x1403414bf: movups xmmword ptr [rsi + 0x20], xmm2
0x1403414c3: movups xmmword ptr [rsi + 0x10], xmm1
0x1403414c7: movups xmmword ptr [rsi], xmm0
0x1403414ca: mov qword ptr [rsi + 0xc0], 0
0x1403414d5: mov qword ptr [rsi + 0x150], 0
0x1403414e0: movaps xmm0, xmmword ptr [rbp - 0x60]
0x1403414e4: movaps xmm1, xmmword ptr [rbp - 0x50]
0x1403414e8: movaps xmm2, xmmword ptr [rbp - 0x40]
0x1403414ec: movups xmmword ptr [rsi + 0x30], xmm0
0x1403414f0: movups xmmword ptr [rsi + 0x40], xmm1
0x1403414f4: movups xmmword ptr [rsi + 0x50], xmm2
0x1403414f8: mov byte ptr [rsi + 0x1e0], 0
0x1403414ff: movaps xmm0, xmmword ptr [rbp - 0x30]
0x140341503: movaps xmm1, xmmword ptr [rbp - 0x20]
0x140341507: movaps xmm2, xmmword ptr [rbp - 0x10]
0x14034150b: movups xmmword ptr [rsi + 0x60], xmm0
0x14034150f: movups xmmword ptr [rsi + 0x70], xmm1
0x140341513: movups xmmword ptr [rsi + 0x80], xmm2
0x14034151a: movups xmmword ptr [rsi + 0x90], xmm7
0x140341521: movups xmmword ptr [rsi + 0xa0], xmm6
0x140341528: mov qword ptr [rsi + 0xb0], rax
0x14034152f: mov qword ptr [rsi + 0xb8], rdx
0x140341536: movaps xmm6, xmmword ptr [rbp + 0x40]
0x14034153a: movaps xmm7, xmmword ptr [rbp + 0x50]
0x14034153e: add rsp, 0xe0
0x140341545: pop rdi
0x140341546: pop rsi
0x140341547: pop rbp
0x140341548: ret 
0x140341549: lea rcx, [rip + 0x25f14c0]
0x140341550: call 0x1428d80d0
0x140341555: ud2 
0x140341557: nop word ptr [rax + rax]
0x140341560: mov qword ptr [rsp + 0x10], rdx
0x140341565: push rbp
0x140341566: push rsi
0x140341567: push rdi
0x140341568: sub rsp, 0x40
0x14034156c: lea rbp, [rdx + 0x80]
0x140341573: movaps xmmword ptr [rsp + 0x20], xmm7
0x140341578: movaps xmmword ptr [rsp + 0x30], xmm6
0x14034157d: mov rcx, rbp
0x140341580: mov r8d, 0x98
0x140341586: mov r9d, 0x10
0x14034158c: lea rdx, [rbp + 0x20]
0x140341590: call 0x140050900
0x140341595: movaps xmm6, xmmword ptr [rsp + 0x30]
0x14034159a: movaps xmm7, xmmword ptr [rsp + 0x20]
0x14034159f: add rsp, 0x40
0x1403415a3: pop rdi
0x1403415a4: pop rsi
0x1403415a5: pop rbp
0x1403415a6: ret 
0x1403415a7: nop word ptr [rax + rax]
0x1403415b0: mov qword ptr [rsp + 0x10], rdx
0x1403415b5: push rbp
0x1403415b6: push rsi
0x1403415b7: push rdi
0x1403415b8: sub rsp, 0x40
0x1403415bc: lea rbp, [rdx + 0x80]
0x1403415c3: movaps xmmword ptr [rsp + 0x20], xmm7
0x1403415c8: movaps xmmword ptr [rsp + 0x30], xmm6
0x1403415cd: lea rcx, [rbp - 0x60]
0x1403415d1: call 0x1405d6640
0x1403415d6: movaps xmm6, xmmword ptr [rsp + 0x30]
0x1403415db: movaps xmm7, xmmword ptr [rsp + 0x20]
0x1403415e0: add rsp, 0x40
0x1403415e4: pop rdi
0x1403415e5: pop rsi
0x1403415e6: pop rbp
0x1403415e7: ret 
0x1403415e8: nop dword ptr [rax + rax]
0x1403415f0: mov qword ptr [rsp + 0x10], rdx
0x1403415f5: push rbp
0x1403415f6: push rsi
0x1403415f7: push rdi
0x1403415f8: sub rsp, 0x40
0x1403415fc: lea rbp, [rdx + 0x80]
0x140341603: movaps xmmword ptr [rsp + 0x20], xmm7
0x140341608: movaps xmmword ptr [rsp + 0x30], xmm6
0x14034160d: lea rcx, [rbp - 0x30]
0x140341611: mov r8d, 0x50
0x140341617: mov r9d, 0x10
0x14034161d: lea rdx, [rbp - 0x10]
0x140341621: call 0x14004ff40
0x140341626: movaps xmm6, xmmword ptr [rsp + 0x30]
0x14034162b: movaps xmm7, xmmword ptr [rsp + 0x20]
0x140341630: add rsp, 0x40
0x140341634: pop rdi
0x140341635: pop rsi
0x140341636: pop rbp
0x140341637: ret 
