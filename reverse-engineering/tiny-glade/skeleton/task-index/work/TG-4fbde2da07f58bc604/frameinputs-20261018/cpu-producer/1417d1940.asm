1417d1940: push r15
1417d1942: push r14
1417d1944: push rsi
1417d1945: push rdi
1417d1946: push rbx
1417d1947: sub rsp, 0xb0
1417d194e: movaps xmmword ptr [rsp + 0xa0], xmm13
1417d1957: movaps xmmword ptr [rsp + 0x90], xmm12
1417d1960: movaps xmmword ptr [rsp + 0x80], xmm11
1417d1969: movaps xmmword ptr [rsp + 0x70], xmm10
1417d196f: movaps xmmword ptr [rsp + 0x60], xmm9
1417d1975: movaps xmmword ptr [rsp + 0x50], xmm8
1417d197b: movaps xmmword ptr [rsp + 0x40], xmm7
1417d1980: movaps xmmword ptr [rsp + 0x30], xmm6
1417d1985: mov rsi, rcx
1417d1988: mov rax, qword ptr [rdx]
1417d198b: mov rdi, qword ptr [rax]
1417d198e: mov rbx, qword ptr [rax + 8]
1417d1992: mov r14, qword ptr [r8]
1417d1995: mov eax, dword ptr [r14 + 0x54]
1417d1999: mov qword ptr [rsp + 0x28], rax
1417d199e: lea rcx, [rsp + 0x28]
1417d19a3: call 0x140caf380
1417d19a8: xor eax, eax
1417d19aa: ucomiss xmm0, dword ptr [rdi]
1417d19ad: jb 0x1417d1b22
1417d19b3: movsd xmm7, qword ptr [r14 + 0x20]
1417d19b9: movss xmm8, dword ptr [r14 + 0x28]
1417d19bf: movss xmm9, dword ptr [r14 + 0x44]
1417d19c5: mulss xmm9, dword ptr [r14 + 0x48]
1417d19cb: movd xmm0, dword ptr [r14 + 0x4c]
1417d19d1: movd xmm1, dword ptr [rip + 0x1153eff]
1417d19d9: call 0x142923f70
1417d19de: movdqa xmm6, xmm0
1417d19e2: movsd xmm10, qword ptr [r14 + 0x58]
1417d19e8: movss xmm11, dword ptr [r14 + 0x60]
1417d19ee: mov edi, dword ptr [r14 + 0x54]
1417d19f2: movsd xmm12, qword ptr [r14 + 0x38]
1417d19f8: movss xmm13, dword ptr [r14 + 0x40]
1417d19fe: mov r15, qword ptr [rbx]
1417d1a01: mov rbx, qword ptr [r14 + 0x18]
1417d1a05: mov qword ptr [rsp + 0x28], rbx
1417d1a0a: cmp qword ptr [r15 + 0x18], 0
1417d1a0f: je 0x1417d1ab2
1417d1a15: lea rcx, [r15 + 0x20]
1417d1a19: lea rdx, [rsp + 0x28]
1417d1a1e: call 0x1405b0d70
1417d1a23: mov rcx, qword ptr [r15]
1417d1a26: mov rdx, qword ptr [r15 + 8]
1417d1a2a: mov r8, rdx
1417d1a2d: and r8, rax
1417d1a30: shr rax, 0x39
1417d1a34: movd xmm0, eax
1417d1a38: punpcklbw xmm0, xmm0
1417d1a3c: pshuflw xmm0, xmm0, 0
1417d1a41: pshufd xmm0, xmm0, 0
1417d1a46: lea rax, [rcx - 0x2a8]
1417d1a4d: xor r9d, r9d
1417d1a50: pcmpeqd xmm1, xmm1
1417d1a54: movdqu xmm2, xmmword ptr [rcx + r8]
1417d1a5a: movdqa xmm3, xmm2
1417d1a5e: pcmpeqb xmm3, xmm0
1417d1a62: pmovmskb r10d, xmm3
1417d1a67: test r10d, r10d
1417d1a6a: je 0x1417d1a94
1417d1a6c: tzcnt r11d, r10d
1417d1a71: add r11, r8
1417d1a74: and r11, rdx
1417d1a77: neg r11
1417d1a7a: imul r11, r11, 0x2a8
1417d1a81: cmp qword ptr [rax + r11], rbx
1417d1a85: je 0x1417d1abc
1417d1a87: lea r11d, [r10 - 1]
1417d1a8b: and r11w, r10w
1417d1a8f: mov r10d, r11d
1417d1a92: jne 0x1417d1a6c
1417d1a94: pcmpeqb xmm2, xmm1
1417d1a98: pmovmskb r10d, xmm2
1417d1a9d: test r10d, r10d
1417d1aa0: jne 0x1417d1ab2
1417d1aa2: add r8, r9
1417d1aa5: add r8, 0x10
1417d1aa9: add r9, 0x10
1417d1aad: and r8, rdx
1417d1ab0: jmp 0x1417d1a54
1417d1ab2: movd xmm0, dword ptr [rip + 0x12bb1a6]
1417d1aba: jmp 0x1417d1aea
1417d1abc: cmp qword ptr [rcx + r11 - 0x290], 2
1417d1ac5: jb 0x1417d1b6d
1417d1acb: movss xmm0, dword ptr [rcx + r11 - 0x270]
1417d1ad5: pxor xmm1, xmm1
1417d1ad9: ucomiss xmm0, xmm1
1417d1adc: jbe 0x1417d1b6d
1417d1ae2: add rcx, r11
1417d1ae5: movss xmm0, dword ptr [rcx - 0x18]
1417d1aea: mulss xmm9, xmm6
1417d1aef: movlps qword ptr [rsi + 4], xmm7
1417d1af3: movss dword ptr [rsi + 0xc], xmm8
1417d1af9: movss dword ptr [rsi + 0x10], xmm9
1417d1aff: movlps qword ptr [rsi + 0x14], xmm10
1417d1b04: movss dword ptr [rsi + 0x1c], xmm11
1417d1b0a: mov dword ptr [rsi + 0x20], edi
1417d1b0d: movlps qword ptr [rsi + 0x24], xmm12
1417d1b12: movss dword ptr [rsi + 0x2c], xmm13
1417d1b18: movd dword ptr [rsi + 0x30], xmm0
1417d1b1d: mov eax, 1
1417d1b22: mov dword ptr [rsi], eax
1417d1b24: mov rax, rsi
1417d1b27: movaps xmm6, xmmword ptr [rsp + 0x30]
1417d1b2c: movaps xmm7, xmmword ptr [rsp + 0x40]
1417d1b31: movaps xmm8, xmmword ptr [rsp + 0x50]
1417d1b37: movaps xmm9, xmmword ptr [rsp + 0x60]
1417d1b3d: movaps xmm10, xmmword ptr [rsp + 0x70]
1417d1b43: movaps xmm11, xmmword ptr [rsp + 0x80]
1417d1b4c: movaps xmm12, xmmword ptr [rsp + 0x90]
1417d1b55: movaps xmm13, xmmword ptr [rsp + 0xa0]
1417d1b5e: add rsp, 0xb0
1417d1b65: pop rbx
1417d1b66: pop rdi
1417d1b67: pop rsi
1417d1b68: pop r14
1417d1b6a: pop r15
1417d1b6c: ret 
1417d1b6d: lea rcx, [rip + 0x130861f]
1417d1b74: lea r8, [rip + 0x1512ff5]
1417d1b7b: mov edx, 0x32
1417d1b80: call 0x1428d9430
1417d1b85: int3 
