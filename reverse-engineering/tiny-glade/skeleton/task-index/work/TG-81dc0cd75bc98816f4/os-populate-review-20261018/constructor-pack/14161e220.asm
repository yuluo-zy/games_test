14161e220: push rbp
14161e221: push r15
14161e223: push r14
14161e225: push r12
14161e227: push rsi
14161e228: push rdi
14161e229: push rbx
14161e22a: sub rsp, 0x110
14161e231: lea rbp, [rsp + 0x80]
14161e239: mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14161e244: mov rdi, rdx
14161e247: mov rsi, rcx
14161e24a: test r8, r8
14161e24d: je 0x14161e267
14161e24f: mov rbx, r9
14161e252: cmp r9, 0x40001
14161e259: jae 0x14161e42b
14161e25f: mov r15, r8
14161e262: xor r12d, r12d
14161e265: jmp 0x14161e2a1
14161e267: movzx eax, byte ptr [rip + 0x201699b]
14161e26e: mov ebx, 0x40000
14161e273: mov ecx, 0x40000
14161e278: mov edx, 1
14161e27d: call 0x140613c10
14161e282: test rax, rax
14161e285: je 0x14161e495
14161e28b: mov r15, rax
14161e28e: mov r8d, 0x40000
14161e294: mov rcx, rax
14161e297: xor edx, edx
14161e299: call 0x142923c20
14161e29e: mov r12b, 1
14161e2a1: mov qword ptr [rbp + 0x78], r15
14161e2a5: mov byte ptr [rbp + 0x87], r12b
14161e2ac: lea rcx, [rbp + 0x20]
14161e2b0: mov edx, 0x40000
14161e2b5: mov r8d, 0x22
14161e2bb: mov r9b, 2
14161e2be: call 0x140bd4de0
14161e2c3: mov qword ptr [rbp + 0x78], r15
14161e2c7: mov byte ptr [rbp + 0x87], r12b
14161e2ce: lea r14, [rbp + 8]
14161e2d2: lea rdx, [rbp + 0x20]
14161e2d6: mov r8d, 1
14161e2dc: mov rcx, r14
14161e2df: call 0x140bd4f20
14161e2e4: mov qword ptr [rbp + 0x78], r15
14161e2e8: mov byte ptr [rbp + 0x87], r12b
14161e2ef: mov qword ptr [rsp + 0x30], rbx
14161e2f4: mov qword ptr [rsp + 0x28], r15
14161e2f9: mov qword ptr [rsp + 0x20], r14
14161e2fe: lea r8, [rip + 0x1661636]
14161e305: lea rcx, [rbp - 0x48]
14161e309: mov r9d, 0x16
14161e30f: mov rdx, rdi
14161e312: call 0x1416a2dc0
14161e317: cmp dword ptr [rbp - 0x48], 0xb
14161e31b: jne 0x14161e3bd
14161e321: mov rax, qword ptr [rbp - 0x20]
14161e325: mov qword ptr [rbp + 0x40], rax
14161e329: movups xmm0, xmmword ptr [rbp - 0x40]
14161e32d: movups xmm1, xmmword ptr [rbp - 0x30]
14161e331: movaps xmmword ptr [rbp + 0x30], xmm1
14161e335: movaps xmmword ptr [rbp + 0x20], xmm0
14161e339: test r12b, r12b
14161e33c: je 0x14161e351
14161e33e: mov edx, 0x40000
14161e343: mov r8d, 1
14161e349: mov rcx, r15
14161e34c: call 0x140613c20
14161e351: lea rcx, [rbp + 0x20]
14161e355: mov rdx, rdi
14161e358: call 0x140bd4cd0
14161e35d: test rax, rax
14161e360: je 0x14161e41a
14161e366: cmp rdx, qword ptr [rbp + 0x38]
14161e36a: jb 0x14161e3a0
14161e36c: mov rcx, qword ptr [rbp + 0x40]
14161e370: mov qword ptr [rsi + 0x30], rcx
14161e374: movaps xmm0, xmmword ptr [rbp + 0x20]
14161e378: movaps xmm1, xmmword ptr [rbp + 0x30]
14161e37c: movups xmmword ptr [rsi + 0x20], xmm1
14161e380: movups xmmword ptr [rsi + 0x10], xmm0
14161e384: mov qword ptr [rsi], rax
14161e387: mov qword ptr [rsi + 8], rdx
14161e38b: mov rax, rsi
14161e38e: add rsp, 0x110
14161e395: pop rbx
14161e396: pop rdi
14161e397: pop rsi
14161e398: pop r12
14161e39a: pop r14
14161e39c: pop r15
14161e39e: pop rbp
14161e39f: ret 
14161e3a0: lea rcx, [rip + 0x1661e49]
14161e3a7: lea r8, [rip + 0x1661e7a]
14161e3ae: mov edx, 0x37
14161e3b3: call 0x1428d9430
14161e3b8: jmp 0x14161e4b2
14161e3bd: mov dword ptr [rbp + 0x74], r12d
14161e3c1: mov qword ptr [rbp + 0x18], r15
14161e3c5: movups xmm0, xmmword ptr [rbp - 8]
14161e3c9: movaps xmmword ptr [rbp + 0x60], xmm0
14161e3cd: movups xmm0, xmmword ptr [rbp - 0x48]
14161e3d1: movups xmm1, xmmword ptr [rbp - 0x38]
14161e3d5: movups xmm2, xmmword ptr [rbp - 0x28]
14161e3d9: movups xmm3, xmmword ptr [rbp - 0x18]
14161e3dd: movaps xmmword ptr [rbp + 0x50], xmm3
14161e3e1: movaps xmmword ptr [rbp + 0x40], xmm2
14161e3e5: movaps xmmword ptr [rbp + 0x30], xmm1
14161e3e9: movaps xmmword ptr [rbp + 0x20], xmm0
14161e3ed: lea rax, [rip + 0x1661db4]
14161e3f4: mov qword ptr [rsp + 0x20], rax
14161e3f9: lea rcx, [rip + 0x16606e8]
14161e400: lea r9, [rip + 0x16606c1]
14161e407: lea r8, [rbp + 0x20]
14161e40b: mov edx, 0x2b
14161e410: call 0x1428d9760
14161e415: jmp 0x14161e4b2
14161e41a: lea rcx, [rip + 0x1661db7]
14161e421: call 0x1428d9310
14161e426: jmp 0x14161e4b2
14161e42b: lea rax, [rip + 0x147f82a]
14161e432: mov qword ptr [rbp - 0x48], rax
14161e436: mov qword ptr [rbp - 0x40], 0x42
14161e43e: lea rax, [rbp - 0x48]
14161e442: mov qword ptr [rbp + 8], rax
14161e446: lea rax, [rip - 0x15145bd]
14161e44d: mov qword ptr [rbp + 0x10], rax
14161e451: lea rax, [rip + 0x1661370]
14161e458: mov qword ptr [rbp + 0x20], rax
14161e45c: mov qword ptr [rbp + 0x28], 1
14161e464: mov qword ptr [rbp + 0x40], 0
14161e46c: lea rax, [rbp + 8]
14161e470: mov qword ptr [rbp + 0x30], rax
14161e474: mov qword ptr [rbp + 0x38], 1
14161e47c: mov byte ptr [rbp + 0x87], 0
14161e483: lea rdx, [rip + 0x166134e]
14161e48a: lea rcx, [rbp + 0x20]
14161e48e: call 0x1428d9390
14161e493: jmp 0x14161e4b2
14161e495: mov byte ptr [rbp + 0x87], 0
14161e49c: lea r8, [rip + 0x166060d]
14161e4a3: mov ecx, 1
14161e4a8: mov edx, 0x40000
14161e4ad: call 0x1428d8fe3
14161e4b2: ud2 
14161e4b4: nop word ptr cs:[rax + rax]
14161e4c0: mov qword ptr [rsp + 0x10], rdx
14161e4c5: push rbp
14161e4c6: push r15
14161e4c8: push r14
14161e4ca: push r12
14161e4cc: push rsi
14161e4cd: push rdi
14161e4ce: push rbx
14161e4cf: sub rsp, 0x40
14161e4d3: lea rbp, [rdx + 0x80]
14161e4da: test byte ptr [rbp + 0x87], 1
14161e4e1: je 0x14161e4f7
14161e4e3: mov rcx, qword ptr [rbp + 0x78]
14161e4e7: mov edx, 0x40000
14161e4ec: mov r8d, 1
14161e4f2: call 0x140613c20
14161e4f7: nop 
14161e4f8: add rsp, 0x40
14161e4fc: pop rbx
14161e4fd: pop rdi
14161e4fe: pop rsi
14161e4ff: pop r12
14161e501: pop r14
14161e503: pop r15
14161e505: pop rbp
14161e506: ret 
14161e507: nop word ptr [rax + rax]
14161e510: mov qword ptr [rsp + 0x10], rdx
14161e515: push rbp
14161e516: push r15
14161e518: push r14
14161e51a: push r12
14161e51c: push rsi
14161e51d: push rdi
14161e51e: push rbx
14161e51f: sub rsp, 0x40
14161e523: lea rbp, [rdx + 0x80]
14161e52a: lea rcx, [rbp + 0x20]
14161e52e: call 0x140be24b0
14161e533: nop 
14161e534: add rsp, 0x40
14161e538: pop rbx
14161e539: pop rdi
14161e53a: pop rsi
14161e53b: pop r12
14161e53d: pop r14
14161e53f: pop r15
14161e541: pop rbp
14161e542: ret 
14161e543: nop word ptr cs:[rax + rax]
14161e550: mov qword ptr [rsp + 0x10], rdx
14161e555: push rbp
14161e556: push r15
14161e558: push r14
14161e55a: push r12
14161e55c: push rsi
14161e55d: push rdi
14161e55e: push rbx
14161e55f: sub rsp, 0x40
14161e563: lea rbp, [rdx + 0x80]
14161e56a: mov rsi, qword ptr [rbp + 0x18]
14161e56e: mov qword ptr [rbp + 0x78], rsi
14161e572: mov edi, dword ptr [rbp + 0x74]
14161e575: mov byte ptr [rbp + 0x87], dil
14161e57c: lea rcx, [rbp + 0x20]
14161e580: call 0x1407cea70
14161e585: mov qword ptr [rbp + 0x78], rsi
14161e589: mov byte ptr [rbp + 0x87], dil
14161e590: add rsp, 0x40
14161e594: pop rbx
14161e595: pop rdi
14161e596: pop rsi
14161e597: pop r12
14161e599: pop r14
14161e59b: pop r15
14161e59d: pop rbp
14161e59e: ret 
