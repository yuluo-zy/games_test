14155d410: push r15
14155d412: push r14
14155d414: push r13
14155d416: push r12
14155d418: push rsi
14155d419: push rdi
14155d41a: push rbp
14155d41b: push rbx
14155d41c: sub rsp, 0x1c8
14155d423: movaps xmmword ptr [rsp + 0x1b0], xmm8
14155d42c: movaps xmmword ptr [rsp + 0x1a0], xmm7
14155d434: movaps xmmword ptr [rsp + 0x190], xmm6
14155d43c: mov r14, r9
14155d43f: mov rbp, qword ptr [r8 + 0x20]
14155d443: mov r9, qword ptr [r8 + 0x30]
14155d447: mov r10d, dword ptr [r8 + 0x3c]
14155d44b: mov rax, qword ptr [rcx]
14155d44e: cmp dword ptr [rax], 6
14155d451: jne 0x14155d96f
14155d457: cmp byte ptr [rax + 4], 2
14155d45b: jne 0x14155d96f
14155d461: mov dword ptr [rsp + 0x5c], r10d
14155d466: mov qword ptr [rsp + 0x88], r9
14155d46e: mov r9, qword ptr [rsp + 0x270]
14155d476: mov rcx, qword ptr [rsp + 0x268]
14155d47e: mov rax, qword ptr [rsp + 0x280]
14155d486: mov rsi, qword ptr [r8]
14155d489: mov rcx, qword ptr [rcx + 0x28]
14155d48d: mov qword ptr [rsp + 0x80], rcx
14155d495: mov rcx, qword ptr [r9 + 0x20]
14155d499: mov qword ptr [rsp + 0x178], rcx
14155d4a1: mov rcx, qword ptr [r9 + 0x30]
14155d4a5: mov qword ptr [rsp + 0x170], rcx
14155d4ad: mov ecx, dword ptr [r9 + 0x3c]
14155d4b1: mov dword ptr [rsp + 0xa0], ecx
14155d4b8: mov rdi, qword ptr [rax]
14155d4bb: mov rcx, qword ptr [rax + 0x20]
14155d4bf: mov qword ptr [rsp + 0x90], rcx
14155d4c7: mov rcx, qword ptr [rax + 0x30]
14155d4cb: mov qword ptr [rsp + 0x168], rcx
14155d4d3: mov ecx, dword ptr [rax + 0x3c]
14155d4d6: mov dword ptr [rsp + 0x9c], ecx
14155d4dd: mov rcx, qword ptr [rax + 0x40]
14155d4e1: mov qword ptr [rsp + 0x68], rcx
14155d4e6: mov rcx, qword ptr [rax + 0x50]
14155d4ea: mov qword ptr [rsp + 0x160], rcx
14155d4f2: mov eax, dword ptr [rax + 0x5c]
14155d4f5: mov dword ptr [rsp + 0x98], eax
14155d4fc: mov rcx, qword ptr [rdx]
14155d4ff: mov qword ptr [rsp + 0xb0], 0
14155d50b: mov qword ptr [rsp + 0xb8], 8
14155d517: mov qword ptr [rsp + 0xc0], 0
14155d523: mov dword ptr [rsp + 0xc8], 0
14155d52e: mov dword ptr [rsp + 0xe0], 0
14155d539: mov dword ptr [rsp + 0xf8], 0
14155d544: mov dword ptr [rsp + 0x110], 0
14155d54f: mov byte ptr [rsp + 0x128], 0
14155d557: mov byte ptr [rsp + 0x138], 5
14155d55f: mov dword ptr [rsp + 0x13c], 0x7f7fffff
14155d56a: mov word ptr [rsp + 0x158], 0x100
14155d574: mov r12, qword ptr [r9 + 0x40]
14155d578: mov r13, qword ptr [r9 + 0x50]
14155d57c: mov eax, dword ptr [r9 + 0x5c]
14155d580: mov dword ptr [rsp + 0x24], eax
14155d584: lea rdx, [rip + 0x1710d6d]
14155d58b: lea r9, [rsp + 0xb0]
14155d593: mov r8d, 0x15
14155d599: call 0x1409236d0
14155d59e: mov r9, rax
14155d5a1: movss xmm0, dword ptr [rsi + 0x3c]
14155d5a6: ucomiss xmm0, dword ptr [rip + 0x147431f]
14155d5ad: mov qword ptr [rsp + 0x78], rax
14155d5b2: mov qword ptr [rsp + 0x70], rdx
14155d5b7: jb 0x14155d66d
14155d5bd: mov qword ptr [rsp + 0x28], rsi
14155d5c2: mov rsi, r12
14155d5c5: mov r12, r14
14155d5c8: mov r14, r13
14155d5cb: mov r13, rdi
14155d5ce: mov rax, qword ptr [rsp + 0x288]
14155d5d6: mov rcx, qword ptr [rbp + 0x20]
14155d5da: mov qword ptr [rsp + 0xc0], rcx
14155d5e2: movups xmm0, xmmword ptr [rbp + 0x10]
14155d5e6: movaps xmmword ptr [rsp + 0xb0], xmm0
14155d5ee: mov ecx, dword ptr [rax + 0x1c]
14155d5f1: mov r8, qword ptr [rax + 0x10]
14155d5f5: mov dword ptr [r8], ecx
14155d5f8: mov rbx, qword ptr [rax]
14155d5fb: mov r15, qword ptr [rbx + 0x30]
14155d5ff: mov rdi, qword ptr [rbx + 0x40]
14155d603: cmp r15, qword ptr [rbx + 0x20]
14155d607: jne 0x14155d623
14155d609: lea rcx, [rbx + 0x20]
14155d60d: lea rdx, [rip + 0x16dcb34]
14155d614: call 0x140562ab0
14155d619: mov rdx, qword ptr [rsp + 0x70]
14155d61e: mov r9, qword ptr [rsp + 0x78]
14155d623: mov rax, qword ptr [rbx + 0x28]
14155d627: lea rcx, [r15 + r15*4]
14155d62b: mov qword ptr [rax + rcx*8], rdi
14155d62f: mov dword ptr [rax + rcx*8 + 8], 1
14155d637: mov r8, qword ptr [rsp + 0xc0]
14155d63f: mov qword ptr [rax + rcx*8 + 0x1c], r8
14155d644: movaps xmm0, xmmword ptr [rsp + 0xb0]
14155d64c: movups xmmword ptr [rax + rcx*8 + 0xc], xmm0
14155d651: inc r15
14155d654: mov qword ptr [rbx + 0x30], r15
14155d658: inc qword ptr [rbx + 0x40]
14155d65c: mov rdi, r13
14155d65f: mov r13, r14
14155d662: mov r14, r12
14155d665: mov r12, rsi
14155d668: mov rsi, qword ptr [rsp + 0x28]
14155d66d: mov r11, qword ptr [rsp + 0x240]
14155d675: mov bl, 2
14155d677: cmp byte ptr [r9 + 0x18], 2
14155d67c: jb 0x14155d8f8
14155d682: mov r15, qword ptr [rsp + 0x260]
14155d68a: cmp qword ptr [r9 + 0x38], rdx
14155d68e: jne 0x14155d754
14155d694: cmp byte ptr [r9 + 0x19], 3
14155d699: jne 0x14155d754
14155d69f: mov qword ptr [rsp + 0x60], rdi
14155d6a4: mov qword ptr [rsp + 0x28], rsi
14155d6a9: mov rsi, r13
14155d6ac: mov r13, r12
14155d6af: mov rax, qword ptr [rsp + 0x258]
14155d6b7: mov ecx, dword ptr [r11 + 0x1c]
14155d6bb: mov r10, qword ptr [r11 + 0x10]
14155d6bf: mov dword ptr [r10], ecx
14155d6c2: mov r8, qword ptr [r11]
14155d6c5: mov byte ptr [r8 + 0xc], 0
14155d6ca: mov rax, qword ptr [rax]
14155d6cd: movsd xmm0, qword ptr [rax]
14155d6d1: movsd qword ptr [r8 + 4], xmm0
14155d6d7: mov dword ptr [r10], ecx
14155d6da: mov dword ptr [r8], 0
14155d6e1: mov eax, dword ptr [r15 + 0x1c]
14155d6e5: mov rcx, qword ptr [r15 + 0x10]
14155d6e9: mov dword ptr [rcx], eax
14155d6eb: mov rdi, qword ptr [r15]
14155d6ee: mov r15, qword ptr [rdi + 0x30]
14155d6f2: mov r12, qword ptr [rdi + 0x40]
14155d6f6: cmp r15, qword ptr [rdi + 0x20]
14155d6fa: jne 0x14155d71e
14155d6fc: lea rcx, [rdi + 0x20]
14155d700: lea rdx, [rip + 0x16dca41]
14155d707: call 0x140562870
14155d70c: mov r11, qword ptr [rsp + 0x240]
14155d714: mov rdx, qword ptr [rsp + 0x70]
14155d719: mov r9, qword ptr [rsp + 0x78]
14155d71e: mov rax, qword ptr [rdi + 0x28]
14155d722: mov qword ptr [rax + r15*8], r12
14155d726: inc r15
14155d729: mov qword ptr [rdi + 0x30], r15
14155d72d: inc qword ptr [rdi + 0x40]
14155d731: cmp byte ptr [r9 + 0x18], 2
14155d736: mov r12, r13
14155d739: mov r13, rsi
14155d73c: mov rsi, qword ptr [rsp + 0x28]
14155d741: mov rdi, qword ptr [rsp + 0x60]
14155d746: mov r15, qword ptr [rsp + 0x260]
14155d74e: jb 0x14155d8f8
14155d754: cmp qword ptr [r9 + 0x38], rdx
14155d758: jne 0x14155d832
14155d75e: cmp byte ptr [r9 + 0x19], 4
14155d763: jne 0x14155d832
14155d769: mov qword ptr [rsp + 0x60], rdi
14155d76e: mov qword ptr [rsp + 0x28], rsi
14155d773: mov qword ptr [rsp + 0x50], r13
14155d778: mov r13, r12
14155d77b: mov rax, qword ptr [rsp + 0x248]
14155d783: mov rax, qword ptr [rax]
14155d786: movsd xmm0, qword ptr [rax + 0x140]
14155d78e: mulps xmm0, xmm0
14155d791: movshdup xmm1, xmm0
14155d795: addss xmm1, xmm0
14155d799: xorps xmm0, xmm0
14155d79c: sqrtss xmm0, xmm1
14155d7a0: mov eax, dword ptr [r11 + 0x1c]
14155d7a4: mov rcx, qword ptr [r11 + 0x10]
14155d7a8: mov dword ptr [rcx], eax
14155d7aa: mov r8, qword ptr [r11]
14155d7ad: addss xmm0, dword ptr [r8]
14155d7b2: movss dword ptr [r8], xmm0
14155d7b7: ucomiss xmm0, dword ptr [rip + 0x13d6ed6]
14155d7be: jbe 0x14155d7c7
14155d7c0: mov dword ptr [rcx], eax
14155d7c2: mov byte ptr [r8 + 0xc], 1
14155d7c7: mov eax, dword ptr [r15 + 0x1c]
14155d7cb: mov rcx, qword ptr [r15 + 0x10]
14155d7cf: mov dword ptr [rcx], eax
14155d7d1: mov rdi, qword ptr [r15]
14155d7d4: mov r15, qword ptr [rdi + 0x30]
14155d7d8: mov r12, qword ptr [rdi + 0x40]
14155d7dc: cmp r15, qword ptr [rdi + 0x20]
14155d7e0: jne 0x14155d802
14155d7e2: lea rcx, [rdi + 0x20]
14155d7e6: lea rdx, [rip + 0x16dc95b]
14155d7ed: mov rsi, r11
14155d7f0: call 0x140562870
14155d7f5: mov r11, rsi
14155d7f8: mov rdx, qword ptr [rsp + 0x70]
14155d7fd: mov r9, qword ptr [rsp + 0x78]
14155d802: mov rax, qword ptr [rdi + 0x28]
14155d806: mov qword ptr [rax + r15*8], r12
14155d80a: inc r15
14155d80d: mov qword ptr [rdi + 0x30], r15
14155d811: inc qword ptr [rdi + 0x40]
14155d815: cmp byte ptr [r9 + 0x18], 2
14155d81a: mov r12, r13
14155d81d: mov r13, qword ptr [rsp + 0x50]
14155d822: mov rsi, qword ptr [rsp + 0x28]
14155d827: mov rdi, qword ptr [rsp + 0x60]
14155d82c: jb 0x14155d8f8
14155d832: cmp qword ptr [r9 + 0x38], rdx
14155d836: jne 0x14155d8ed
14155d83c: cmp byte ptr [r9 + 0x19], 1
14155d841: ja 0x14155d8ed
14155d847: mov qword ptr [rsp + 0x50], rbp
14155d84c: mov qword ptr [rsp + 0x28], rsi
14155d851: mov qword ptr [rsp + 0x60], r12
14155d856: mov r12, r14
14155d859: mov r14, r13
14155d85c: mov r13, rdi
14155d85f: mov rax, qword ptr [rsp + 0x250]
14155d867: mov rcx, qword ptr [r11]
14155d86a: movsd xmm6, qword ptr [rcx + 4]
14155d86f: mov ecx, dword ptr [rax + 0x1c]
14155d872: mov r8, qword ptr [rax + 0x10]
14155d876: mov dword ptr [r8], ecx
14155d879: mov r15, qword ptr [rax]
14155d87c: mov rdi, qword ptr [r15 + 0x30]
14155d880: mov rsi, qword ptr [r15 + 0x40]
14155d884: cmp rdi, qword ptr [r15 + 0x20]
14155d888: jne 0x14155d8aa
14155d88a: lea rcx, [r15 + 0x20]
14155d88e: lea rdx, [rip + 0x16dc8b3]
14155d895: mov rbp, r11
14155d898: call 0x140562db0
14155d89d: mov r11, rbp
14155d8a0: mov rdx, qword ptr [rsp + 0x70]
14155d8a5: mov r9, qword ptr [rsp + 0x78]
14155d8aa: mov rax, qword ptr [r15 + 0x28]
14155d8ae: lea rcx, [rdi + rdi*2]
14155d8b2: mov qword ptr [rax + rcx*8], rsi
14155d8b6: mov dword ptr [rax + rcx*8 + 8], 2
14155d8be: movlps qword ptr [rax + rcx*8 + 0xc], xmm6
14155d8c3: inc rdi
14155d8c6: mov qword ptr [r15 + 0x30], rdi
14155d8ca: inc qword ptr [r15 + 0x40]
14155d8ce: cmp byte ptr [r9 + 0x18], 2
14155d8d3: mov rdi, r13
14155d8d6: mov r13, r14
14155d8d9: mov r14, r12
14155d8dc: mov r12, qword ptr [rsp + 0x60]
14155d8e1: mov rsi, qword ptr [rsp + 0x28]
14155d8e6: mov rbp, qword ptr [rsp + 0x50]
14155d8eb: jb 0x14155d8f8
14155d8ed: cmp qword ptr [r9 + 0x38], rdx
14155d8f1: jne 0x14155d8f8
14155d8f3: movzx ebx, byte ptr [r9 + 0x19]
14155d8f8: mov r15, qword ptr [rsp + 0x238]
14155d900: add bl, 0xfe
14155d903: movzx ecx, bl
14155d906: cmp cl, 3
14155d909: mov eax, 3
14155d90e: cmovb eax, ecx
14155d911: cmp al, 2
14155d913: jae 0x14155d97e
14155d915: cmp byte ptr [rsi + 0x2c], 2
14155d919: jne 0x14155daaf
14155d91f: mov rdx, qword ptr [r14]
14155d922: lea rcx, [rsp + 0xb0]
14155d92a: call 0x140c96df0
14155d92f: movss xmm1, dword ptr [rsp + 0xb0]
14155d938: movss xmm0, dword ptr [rsp + 0xb8]
14155d941: movaps xmm2, xmm1
14155d944: mulss xmm2, xmm1
14155d948: unpcklps xmm1, xmm0
14155d94b: mulss xmm0, xmm0
14155d94f: addss xmm0, xmm2
14155d953: sqrtss xmm0, xmm0
14155d957: movss xmm2, dword ptr [rip + 0x13c7fa5]
14155d95f: divss xmm2, xmm0
14155d963: movsldup xmm0, xmm2
14155d967: mulps xmm0, xmm1
14155d96a: jmp 0x14155dae9
14155d96f: mov dword ptr [r9], r10d
14155d972: mov dword ptr [rbp], 0
14155d979: jmp 0x14155e297
14155d97e: movzx eax, al
14155d981: cmp eax, 2
14155d984: jne 0x14155dda2
14155d98a: mov rax, qword ptr [r11]
14155d98d: xorps xmm7, xmm7
14155d990: mov bl, 1
14155d992: cmp byte ptr [rax + 0xc], 0
14155d996: mov r8b, 1
14155d999: je 0x14155da71
14155d99f: movss xmm0, dword ptr [rsi + 0x34]
14155d9a4: mov rcx, qword ptr [rsp + 0x80]
14155d9ac: subss xmm0, dword ptr [rcx + 4]
14155d9b1: divss xmm0, dword ptr [rcx + 0x10]
14155d9b6: movss xmm1, dword ptr [rcx + 0xc]
14155d9bb: mulss xmm1, xmm0
14155d9bf: mulss xmm0, dword ptr [rcx + 0x14]
14155d9c4: addss xmm1, dword ptr [rcx]
14155d9c8: addss xmm0, dword ptr [rcx + 8]
14155d9cd: subss xmm1, dword ptr [rbp + 0x18]
14155d9d2: subss xmm0, dword ptr [rbp + 0x1c]
14155d9d7: movsldup xmm2, xmm0
14155d9db: movsd xmm7, qword ptr [rbp + 0x10]
14155d9e0: movsd xmm0, qword ptr [rbp + 0x20]
14155d9e5: movsldup xmm1, xmm1
14155d9e9: mulps xmm1, xmm7
14155d9ec: shufps xmm7, xmm7, 0xe1
14155d9f0: mulps xmm7, xmm2
14155d9f3: movaps xmm2, xmm7
14155d9f6: addps xmm2, xmm1
14155d9f9: subps xmm7, xmm1
14155d9fc: shufps xmm7, xmm2, 1
14155da00: shufps xmm7, xmm2, 0xe2
14155da04: movaps xmm1, xmmword ptr [rip + 0x13d0f75]
14155da0b: andps xmm7, xmm1
14155da0e: addps xmm7, xmm7
14155da11: movaps xmm2, xmmword ptr [rip + 0x170ffe8]
14155da18: movaps xmm3, xmm2
14155da1b: maxps xmm3, xmm7
14155da1e: cmpunordps xmm7, xmm7
14155da22: andps xmm2, xmm7
14155da25: andnps xmm7, xmm3
14155da28: orps xmm7, xmm2
14155da2b: movaps xmm2, xmmword ptr [rip + 0x170ffde]
14155da32: movaps xmm3, xmm2
14155da35: minps xmm3, xmm7
14155da38: cmpunordps xmm7, xmm7
14155da3c: andps xmm2, xmm7
14155da3f: andnps xmm7, xmm3
14155da42: orps xmm7, xmm2
14155da45: movss dword ptr [rbp + 0x20], xmm7
14155da4a: pshufd xmm2, xmm7, 0xf5
14155da4f: subps xmm7, xmm0
14155da52: andps xmm7, xmm1
14155da55: mov rcx, qword ptr [rsp + 0x88]
14155da5d: mov edx, dword ptr [rsp + 0x5c]
14155da61: mov dword ptr [rcx], edx
14155da63: movd dword ptr [rbp + 0x24], xmm2
14155da68: movzx r8d, byte ptr [rax + 0xc]
14155da6d: xor r8b, 1
14155da71: mov r14, qword ptr [r15]
14155da74: mov rax, qword ptr [rbp + 0x20]
14155da78: mov qword ptr [rsp + 0x40], rax
14155da7d: movups xmm0, xmmword ptr [rbp + 0x10]
14155da81: movaps xmmword ptr [rsp + 0x30], xmm0
14155da86: and r8b, 1
14155da8a: lea r15, [rsp + 0xb0]
14155da92: lea rdx, [rsp + 0x30]
14155da97: mov rcx, r15
14155da9a: call 0x14155e460
14155da9f: mov rcx, r14
14155daa2: mov rdx, r15
14155daa5: call 0x14146c880
14155daaa: jmp 0x14155db5a
14155daaf: movss xmm0, dword ptr [rsi + 0x18]
14155dab4: movss xmm1, dword ptr [rsi + 0x1c]
14155dab9: movaps xmm2, xmmword ptr [rip + 0x13c7e30]
14155dac0: xorps xmm2, xmm1
14155dac3: mulss xmm1, xmm1
14155dac7: unpcklps xmm2, xmm0
14155daca: mulss xmm0, xmm0
14155dace: addss xmm0, xmm1
14155dad2: sqrtss xmm0, xmm0
14155dad6: movss xmm1, dword ptr [rip + 0x13c7e26]
14155dade: divss xmm1, xmm0
14155dae2: movsldup xmm0, xmm1
14155dae6: mulps xmm0, xmm2
14155dae9: mov rax, qword ptr [rsp + 0x88]
14155daf1: mov ecx, dword ptr [rsp + 0x5c]
14155daf5: mov dword ptr [rax], ecx
14155daf7: movlps qword ptr [rbp + 0x10], xmm0
14155dafb: movss xmm0, dword ptr [rsi + 0x30]
14155db00: movss xmm1, dword ptr [rsi + 0x38]
14155db05: mov dword ptr [rax], ecx
14155db07: movss dword ptr [rbp + 0x18], xmm0
14155db0c: movss dword ptr [rbp + 0x1c], xmm1
14155db11: movss xmm0, dword ptr [rsi + 0x34]
14155db16: mov dword ptr [rax], ecx
14155db18: movss dword ptr [rbp + 0x28], xmm0
14155db1d: mov r14, qword ptr [r15]
14155db20: mov rax, qword ptr [rbp + 0x20]
14155db24: mov qword ptr [rsp + 0x40], rax
14155db29: movups xmm0, xmmword ptr [rbp + 0x10]
14155db2d: movaps xmmword ptr [rsp + 0x30], xmm0
14155db32: lea r15, [rsp + 0xb0]
14155db3a: lea rdx, [rsp + 0x30]
14155db3f: mov bl, 1
14155db41: mov rcx, r15
14155db44: mov r8b, 1
14155db47: call 0x14155e460
14155db4c: mov rcx, r14
14155db4f: mov rdx, r15
14155db52: call 0x14146c880
14155db57: xorps xmm7, xmm7
14155db5a: mov rax, qword ptr [rbp + 0x20]
14155db5e: mov qword ptr [rsp + 0xc4], rax
14155db66: movups xmm0, xmmword ptr [rbp + 0x10]
14155db6a: movups xmmword ptr [rsp + 0xb4], xmm0
14155db72: mov dword ptr [rsp + 0xb0], 1
14155db7d: lea rdx, [rsp + 0xb0]
14155db85: mov rcx, rdi
14155db88: call 0x140ad2970
14155db8d: test al, al
14155db8f: jne 0x14155dbeb
14155db91: mov rax, qword ptr [rsp + 0x168]
14155db99: mov ecx, dword ptr [rsp + 0x9c]
14155dba0: mov dword ptr [rax], ecx
14155dba2: mov rcx, qword ptr [rsp + 0x90]
14155dbaa: mov rdi, qword ptr [rcx + 0x30]
14155dbae: mov r14, qword ptr [rcx + 0x40]
14155dbb2: cmp rdi, qword ptr [rcx + 0x20]
14155dbb6: jne 0x14155dbd8
14155dbb8: mov rax, qword ptr [rsp + 0x90]
14155dbc0: lea rcx, [rax + 0x20]
14155dbc4: lea rdx, [rip + 0x16dc57d]
14155dbcb: call 0x140562870
14155dbd0: mov rcx, qword ptr [rsp + 0x90]
14155dbd8: mov rax, qword ptr [rcx + 0x28]
14155dbdc: mov qword ptr [rax + rdi*8], r14
14155dbe0: inc rdi
14155dbe3: mov qword ptr [rcx + 0x30], rdi
14155dbe7: inc qword ptr [rcx + 0x40]
14155dbeb: lea r15, [rbp + 0x10]
14155dbef: test bl, bl
14155dbf1: je 0x14155dd28
14155dbf7: mov qword ptr [rsp + 0x50], r13
14155dbfc: mov r13, r12
14155dbff: movups xmm0, xmmword ptr [r15]
14155dc03: movaps xmmword ptr [rsp + 0x180], xmm0
14155dc0b: movsd xmm6, qword ptr [rbp + 0x20]
14155dc10: addps xmm6, xmmword ptr [rip + 0x14815b9]
14155dc17: mov eax, dword ptr [rsi + 0x2c]
14155dc1a: xor ebx, ebx
14155dc1c: cmp al, 2
14155dc1e: cmovne ebx, eax
14155dc21: mov r14, r15
14155dc24: movups xmm0, xmmword ptr [r15]
14155dc28: movaps xmmword ptr [rsp + 0xb0], xmm0
14155dc30: mov rdx, qword ptr [rsp + 0x270]
14155dc38: mov eax, dword ptr [rdx + 0x1c]
14155dc3b: mov rcx, qword ptr [rdx + 0x10]
14155dc3f: mov dword ptr [rcx], eax
14155dc41: mov rdi, qword ptr [rdx]
14155dc44: mov r15, qword ptr [rdi + 0x30]
14155dc48: mov r12, qword ptr [rdi + 0x40]
14155dc4c: cmp r15, qword ptr [rdi + 0x20]
14155dc50: jne 0x14155dc62
14155dc52: lea rcx, [rdi + 0x20]
14155dc56: lea rdx, [rip + 0x16dc4eb]
14155dc5d: call 0x140562ab0
14155dc62: mov rax, qword ptr [rdi + 0x28]
14155dc66: lea rcx, [r15 + r15*4]
14155dc6a: mov dword ptr [rax + rcx*8], 1
14155dc71: movaps xmm0, xmmword ptr [rsp + 0xb0]
14155dc79: movups xmmword ptr [rax + rcx*8 + 4], xmm0
14155dc7e: movlps qword ptr [rax + rcx*8 + 0x14], xmm6
14155dc83: mov byte ptr [rax + rcx*8 + 0x1c], bl
14155dc87: mov qword ptr [rax + rcx*8 + 0x20], r12
14155dc8c: inc r15
14155dc8f: mov qword ptr [rdi + 0x30], r15
14155dc93: inc qword ptr [rdi + 0x40]
14155dc97: movss xmm8, dword ptr [rsi + 0x3c]
14155dc9d: mov rax, qword ptr [rsp + 0x170]
14155dca5: mov ecx, dword ptr [rsp + 0xa0]
14155dcac: mov dword ptr [rax], ecx
14155dcae: mov r15, qword ptr [rsp + 0x178]
14155dcb6: mov rdi, qword ptr [r15 + 0x30]
14155dcba: mov rbx, qword ptr [r15 + 0x40]
14155dcbe: cmp rdi, qword ptr [r15 + 0x20]
14155dcc2: jne 0x14155dcd4
14155dcc4: lea rcx, [r15 + 0x20]
14155dcc8: lea rdx, [rip + 0x16dc479]
14155dccf: call 0x1405629f0
14155dcd4: mov rax, qword ptr [r15 + 0x28]
14155dcd8: lea rcx, [rdi + rdi*2]
14155dcdc: shl rcx, 4
14155dce0: mov dword ptr [rax + rcx], 1
14155dce7: movaps xmm0, xmmword ptr [rsp + 0x180]
14155dcef: movups xmmword ptr [rax + rcx + 4], xmm0
14155dcf4: movlps qword ptr [rax + rcx + 0x14], xmm6
14155dcf9: movss dword ptr [rax + rcx + 0x1c], xmm8
14155dd00: mov dword ptr [rax + rcx + 0x20], 0x40800000
14155dd08: mov byte ptr [rax + rcx + 0x24], 0
14155dd0d: mov qword ptr [rax + rcx + 0x28], rbx
14155dd12: inc rdi
14155dd15: mov qword ptr [r15 + 0x30], rdi
14155dd19: inc qword ptr [r15 + 0x40]
14155dd1d: mov r12, r13
14155dd20: mov r13, qword ptr [rsp + 0x50]
14155dd25: mov r15, r14
14155dd28: mov rax, qword ptr [rsp + 0x268]
14155dd30: mov rdx, qword ptr [rax]
14155dd33: mov rcx, qword ptr [rax + 8]
14155dd37: mov r8, qword ptr [rdx]
14155dd3a: xor eax, eax
14155dd3c: mov r9, r8
14155dd3f: sub r9, qword ptr [rcx + 0x18]
14155dd43: cmovb r9, rax
14155dd47: sub r8, qword ptr [rcx + 0x38]
14155dd4b: cmovb r8, rax
14155dd4f: mov r10, qword ptr [rcx + 0x30]
14155dd53: sub r10, r8
14155dd56: cmovb r10, rax
14155dd5a: setbe r8b
14155dd5e: mov r11, qword ptr [rcx + 0x10]
14155dd62: sub r11, r9
14155dd65: cmovb r11, rax
14155dd69: mov r9, qword ptr [rcx + 0x40]
14155dd6d: setbe cl
14155dd70: add r11, r10
14155dd73: and cl, r8b
14155dd76: cmove r11, rax
14155dd7a: sub r9, r11
14155dd7d: mov qword ptr [rdx], r9
14155dd80: mov rdx, qword ptr [rsp + 0x78]
14155dd85: cmp byte ptr [rdx + 0x18], 2
14155dd89: jb 0x14155de0f
14155dd8f: mov rax, qword ptr [rsp + 0x70]
14155dd94: cmp qword ptr [rdx + 0x38], rax
14155dd98: jne 0x14155de0d
14155dd9a: movzx eax, byte ptr [rdx + 0x19]
14155dd9e: add al, 0xfe
14155dda0: jmp 0x14155de0f
14155dda2: mov rax, qword ptr [r11]
14155dda5: xorps xmm7, xmm7
14155dda8: mov bl, 1
14155ddaa: cmp byte ptr [rax + 0xc], 0
14155ddae: jne 0x14155db5a
14155ddb4: mov rax, qword ptr [rbp + 0x20]
14155ddb8: mov qword ptr [rsp + 0xc4], rax
14155ddc0: movups xmm0, xmmword ptr [rbp + 0x10]
14155ddc4: movups xmmword ptr [rsp + 0xb4], xmm0
14155ddcc: mov dword ptr [rsp + 0xb0], 1
14155ddd7: lea rdx, [rsp + 0xb0]
14155dddf: mov rcx, rdi
14155dde2: call 0x140ad2970
14155dde7: test al, al
14155dde9: je 0x14155e0f0
14155ddef: mov rax, qword ptr [rsp + 0x230]
14155ddf7: lea r8, [rbp + 0x10]
14155ddfb: cmp byte ptr [rsi + 0x2c], 2
14155ddff: jne 0x14155e2c4
14155de05: xor r9d, r9d
14155de08: jmp 0x14155e2ce
14155de0d: xor eax, eax
14155de0f: movzx eax, al
14155de12: cmp al, 3
14155de14: mov edx, 3
14155de19: cmovb edx, eax
14155de1c: movzx eax, dl
14155de1f: lea rdx, [rip + 0x1710a3e]
14155de26: movsxd rax, dword ptr [rdx + rax*4]
14155de2a: add rax, rdx
14155de2d: jmp rax
14155de2f: test cl, cl
14155de31: jne 0x14155e297
14155de37: mov rax, rsi
14155de3a: mov rsi, qword ptr [rsp + 0x278]
14155de42: lea rdx, [rbp + 0x18]
14155de46: mov rdi, rax
14155de49: movss xmm2, dword ptr [rax + 0x3c]
14155de4e: lea rcx, [rsp + 0xa4]
14155de56: call 0x140c9cde0
14155de5b: movss xmm2, dword ptr [rsp + 0xa4]
14155de64: movss xmm0, dword ptr [rsp + 0xa8]
14155de6d: movss xmm1, dword ptr [rsp + 0xac]
14155de76: test byte ptr [rbp], 1
14155de7a: jne 0x14155e14e
14155de80: movaps xmm3, xmm2
14155de83: movaps xmm4, xmm0
14155de86: movaps xmm5, xmm1
14155de89: jmp 0x14155e15d
14155de8e: mov eax, dword ptr [rsi + 0x38]
14155de91: mov dword ptr [rsp + 0x3f], eax
14155de95: mov rax, qword ptr [rsi + 0x30]
14155de99: mov qword ptr [rsp + 0x37], rax
14155de9e: lea rcx, [rsp + 0xb4]
14155dea6: movdqu xmm0, xmmword ptr [r15]
14155deab: mov rax, qword ptr [r15 + 0x10]
14155deaf: mov qword ptr [rsp + 0xc4], rax
14155deb7: movdqu xmmword ptr [rsp + 0xb4], xmm0
14155dec0: mov dword ptr [rsp + 0xb0], 1
14155decb: call 0x140c99520
14155ded0: movdqa xmm2, xmm0
14155ded4: movss xmm0, dword ptr [rip + 0x170fb14]
14155dedc: movss xmm1, dword ptr [rip + 0x170fb10]
14155dee4: call 0x140c904a0
14155dee9: movaps xmm6, xmm0
14155deec: mov eax, dword ptr [rsp + 0x24]
14155def0: mov dword ptr [r13], eax
14155def4: mov rsi, qword ptr [r12 + 0x30]
14155def9: mov rdi, qword ptr [r12 + 0x40]
14155defe: cmp rsi, qword ptr [r12 + 0x20]
14155df03: jne 0x14155df16
14155df05: lea rcx, [r12 + 0x20]
14155df0a: lea rdx, [rip + 0x16dc237]
14155df11: call 0x140562c30
14155df16: mov rax, qword ptr [r12 + 0x28]
14155df1b: mov rcx, rsi
14155df1e: shl rcx, 5
14155df22: mov byte ptr [rax + rcx], 2
14155df26: movups xmm0, xmmword ptr [rsp + 0x30]
14155df2b: movups xmmword ptr [rax + rcx + 1], xmm0
14155df30: mov edx, dword ptr [rsp + 0x3f]
14155df34: jmp 0x14155e0cf
14155df39: xorps xmm8, xmm8
14155df3d: ucomiss xmm7, xmm8
14155df41: jbe 0x14155e015
14155df47: mov eax, dword ptr [rsi + 0x38]
14155df4a: mov dword ptr [rsp + 0x38], eax
14155df4e: mov rbx, rsi
14155df51: mov rax, qword ptr [rsi + 0x30]
14155df55: mov qword ptr [rsp + 0x30], rax
14155df5a: lea rcx, [rsp + 0xb4]
14155df62: movdqu xmm0, xmmword ptr [r15]
14155df67: mov rax, qword ptr [r15 + 0x10]
14155df6b: mov qword ptr [rsp + 0xc4], rax
14155df73: movdqu xmmword ptr [rsp + 0xb4], xmm0
14155df7c: mov dword ptr [rsp + 0xb0], 1
14155df87: call 0x140c99520
14155df8c: movdqa xmm2, xmm0
14155df90: movss xmm0, dword ptr [rip + 0x170fa58]
14155df98: movss xmm1, dword ptr [rip + 0x170fa54]
14155dfa0: call 0x140c904a0
14155dfa5: movaps xmm6, xmm0
14155dfa8: mov eax, dword ptr [rsp + 0x24]
14155dfac: mov dword ptr [r13], eax
14155dfb0: mov rsi, qword ptr [r12 + 0x30]
14155dfb5: mov rdi, qword ptr [r12 + 0x40]
14155dfba: cmp rsi, qword ptr [r12 + 0x20]
14155dfbf: jne 0x14155dfd2
14155dfc1: lea rcx, [r12 + 0x20]
14155dfc6: lea rdx, [rip + 0x16dc17b]
14155dfcd: call 0x140562c30
14155dfd2: mov rax, qword ptr [r12 + 0x28]
14155dfd7: mov rcx, rsi
14155dfda: shl rcx, 5
14155dfde: mov byte ptr [rax + rcx], 1
14155dfe2: movss dword ptr [rax + rcx + 4], xmm7
14155dfe8: mov rdx, qword ptr [rsp + 0x30]
14155dfed: mov qword ptr [rax + rcx + 8], rdx
14155dff2: mov edx, dword ptr [rsp + 0x38]
14155dff6: mov dword ptr [rax + rcx + 0x10], edx
14155dffa: movss dword ptr [rax + rcx + 0x14], xmm6
14155e000: mov qword ptr [rax + rcx + 0x18], rdi
14155e005: inc rsi
14155e008: mov qword ptr [r12 + 0x30], rsi
14155e00d: inc qword ptr [r12 + 0x40]
14155e012: mov rsi, rbx
14155e015: movshdup xmm7, xmm7
14155e019: ucomiss xmm7, xmm8
14155e01d: jbe 0x14155e297
14155e023: mov eax, dword ptr [rsi + 0x38]
14155e026: mov dword ptr [rsp + 0x38], eax
14155e02a: mov rax, qword ptr [rsi + 0x30]
14155e02e: mov qword ptr [rsp + 0x30], rax
14155e033: lea rcx, [rsp + 0xb4]
14155e03b: movdqu xmm0, xmmword ptr [r15]
14155e040: mov rax, qword ptr [r15 + 0x10]
14155e044: mov qword ptr [rsp + 0xc4], rax
14155e04c: movdqu xmmword ptr [rsp + 0xb4], xmm0
14155e055: mov dword ptr [rsp + 0xb0], 1
14155e060: call 0x140c99520
14155e065: movdqa xmm2, xmm0
14155e069: movss xmm0, dword ptr [rip + 0x170f97f]
14155e071: movss xmm1, dword ptr [rip + 0x170f97b]
14155e079: call 0x140c904a0
14155e07e: movaps xmm6, xmm0
14155e081: mov eax, dword ptr [rsp + 0x24]
14155e085: mov dword ptr [r13], eax
14155e089: mov rsi, qword ptr [r12 + 0x30]
14155e08e: mov rdi, qword ptr [r12 + 0x40]
14155e093: cmp rsi, qword ptr [r12 + 0x20]
14155e098: jne 0x14155e0ab
14155e09a: lea rcx, [r12 + 0x20]
14155e09f: lea rdx, [rip + 0x16dc0a2]
14155e0a6: call 0x140562c30
14155e0ab: mov rax, qword ptr [r12 + 0x28]
14155e0b0: mov rcx, rsi
14155e0b3: shl rcx, 5
14155e0b7: mov byte ptr [rax + rcx], 0
14155e0bb: movss dword ptr [rax + rcx + 4], xmm7
14155e0c1: mov rdx, qword ptr [rsp + 0x30]
14155e0c6: mov qword ptr [rax + rcx + 8], rdx
14155e0cb: mov edx, dword ptr [rsp + 0x38]
14155e0cf: mov dword ptr [rax + rcx + 0x10], edx
14155e0d3: movss dword ptr [rax + rcx + 0x14], xmm6
14155e0d9: mov qword ptr [rax + rcx + 0x18], rdi
14155e0de: inc rsi
14155e0e1: mov qword ptr [r12 + 0x30], rsi
14155e0e6: inc qword ptr [r12 + 0x40]
14155e0eb: jmp 0x14155e297
14155e0f0: mov r14, rdi
14155e0f3: mov rax, qword ptr [rsp + 0x160]
14155e0fb: mov ecx, dword ptr [rsp + 0x98]
14155e102: mov dword ptr [rax], ecx
14155e104: mov rcx, qword ptr [rsp + 0x68]
14155e109: mov rdi, qword ptr [rcx + 0x30]
14155e10d: mov rbx, qword ptr [rcx + 0x40]
14155e111: cmp rdi, qword ptr [rcx + 0x20]
14155e115: jne 0x14155e131
14155e117: mov rax, qword ptr [rsp + 0x68]
14155e11c: lea rcx, [rax + 0x20]
14155e120: lea rdx, [rip + 0x16dc021]
14155e127: call 0x140562870
14155e12c: mov rcx, qword ptr [rsp + 0x68]
14155e131: mov rax, qword ptr [rcx + 0x28]
14155e135: mov qword ptr [rax + rdi*8], rbx
14155e139: inc rdi
14155e13c: mov qword ptr [rcx + 0x30], rdi
14155e140: inc qword ptr [rcx + 0x40]
14155e144: xor ebx, ebx
14155e146: mov rdi, r14
14155e149: jmp 0x14155db5a
14155e14e: movss xmm3, dword ptr [rbp + 4]
14155e153: movss xmm4, dword ptr [rbp + 8]
14155e158: movss xmm5, dword ptr [rbp + 0xc]
14155e15d: subss xmm2, xmm3
14155e161: subss xmm0, xmm4
14155e165: subss xmm1, xmm5
14155e169: mulss xmm2, xmm2
14155e16d: mulss xmm0, xmm0
14155e171: addss xmm0, xmm2
14155e175: mulss xmm1, xmm1
14155e179: addss xmm1, xmm0
14155e17d: sqrtss xmm7, xmm1
14155e181: mov rax, qword ptr [rsi]
14155e184: divss xmm7, dword ptr [rax + 0x28]
14155e189: lea rsi, [rbp + 4]
14155e18d: xorps xmm0, xmm0
14155e190: mov rcx, rdi
14155e193: cmpeqss xmm0, dword ptr [rdi + 0x3c]
14155e199: movd edi, xmm0
14155e19d: and edi, 1
14155e1a0: mov rax, qword ptr [rcx + 0x30]
14155e1a4: mov qword ptr [rsp + 0x30], rax
14155e1a9: mov eax, dword ptr [rcx + 0x38]
14155e1ac: mov dword ptr [rsp + 0x38], eax
14155e1b0: lea rcx, [rsp + 0xb4]
14155e1b8: movdqu xmm0, xmmword ptr [r15]
14155e1bd: mov rax, qword ptr [r15 + 0x10]
14155e1c1: mov qword ptr [rsp + 0xc4], rax
14155e1c9: movdqu xmmword ptr [rsp + 0xb4], xmm0
14155e1d2: mov dword ptr [rsp + 0xb0], 1
14155e1dd: call 0x140c99520
14155e1e2: movdqa xmm2, xmm0
14155e1e6: movss xmm0, dword ptr [rip + 0x170f802]
14155e1ee: movss xmm1, dword ptr [rip + 0x170f7fe]
14155e1f6: call 0x140c904a0
14155e1fb: movaps xmm6, xmm0
14155e1fe: mov eax, dword ptr [rsp + 0x24]
14155e202: mov dword ptr [r13], eax
14155e206: mov rbx, qword ptr [r12 + 0x30]
14155e20b: mov r14, qword ptr [r12 + 0x40]
14155e210: cmp rbx, qword ptr [r12 + 0x20]
14155e215: jne 0x14155e228
14155e217: lea rcx, [r12 + 0x20]
14155e21c: lea rdx, [rip + 0x16dbf25]
14155e223: call 0x140562c30
14155e228: mov rax, qword ptr [r12 + 0x28]
14155e22d: mov rcx, rbx
14155e230: shl rcx, 5
14155e234: mov byte ptr [rax + rcx], 3
14155e238: mov byte ptr [rax + rcx + 1], dil
14155e23d: movss dword ptr [rax + rcx + 4], xmm7
14155e243: mov rdx, qword ptr [rsp + 0x30]
14155e248: mov qword ptr [rax + rcx + 8], rdx
14155e24d: mov edx, dword ptr [rsp + 0x38]
14155e251: mov dword ptr [rax + rcx + 0x10], edx
14155e255: movss dword ptr [rax + rcx + 0x14], xmm6
14155e25b: mov qword ptr [rax + rcx + 0x18], r14
14155e260: inc rbx
14155e263: mov qword ptr [r12 + 0x30], rbx
14155e268: inc qword ptr [r12 + 0x40]
14155e26d: mov rax, qword ptr [rsp + 0x88]
14155e275: mov ecx, dword ptr [rsp + 0x5c]
14155e279: mov dword ptr [rax], ecx
14155e27b: mov dword ptr [rbp], 1
14155e282: mov eax, dword ptr [rsp + 0xac]
14155e289: mov dword ptr [rsi + 8], eax
14155e28c: mov rax, qword ptr [rsp + 0xa4]
14155e294: mov qword ptr [rsi], rax
14155e297: movaps xmm6, xmmword ptr [rsp + 0x190]
14155e29f: movaps xmm7, xmmword ptr [rsp + 0x1a0]
14155e2a7: movaps xmm8, xmmword ptr [rsp + 0x1b0]
14155e2b0: add rsp, 0x1c8
14155e2b7: pop rbx
14155e2b8: pop rbp
14155e2b9: pop rdi
14155e2ba: pop rsi
14155e2bb: pop r12
14155e2bd: pop r13
14155e2bf: pop r14
14155e2c1: pop r15
14155e2c3: ret 
14155e2c4: mov r10, qword ptr [rsi + 0x20]
14155e2c8: mov r9d, 1
14155e2ce: movss xmm6, dword ptr [rsi + 0x40]
14155e2d3: movss xmm8, dword ptr [rsi + 0x3c]
14155e2d9: mov rcx, qword ptr [r8 + 0x10]
14155e2dd: mov qword ptr [rsp + 0xc0], rcx
14155e2e5: movups xmm0, xmmword ptr [r8]
14155e2e9: movaps xmmword ptr [rsp + 0xb0], xmm0
14155e2f1: mov ecx, dword ptr [rax + 0x1c]
14155e2f4: mov rdx, qword ptr [rax + 0x10]
14155e2f8: mov dword ptr [rdx], ecx
14155e2fa: mov r15, qword ptr [rax]
14155e2fd: mov r14, qword ptr [r15 + 0x30]
14155e301: mov r11, qword ptr [r15 + 0x40]
14155e305: cmp r14, qword ptr [r15 + 0x20]
14155e309: jne 0x14155e349
14155e30b: lea rcx, [r15 + 0x20]
14155e30f: lea rdx, [rip + 0x16dbe32]
14155e316: mov qword ptr [rsp + 0x28], r8
14155e31b: mov qword ptr [rsp + 0x50], r9
14155e320: mov qword ptr [rsp + 0x68], r10
14155e325: mov qword ptr [rsp + 0x80], r11
14155e32d: call 0x1406319d0
14155e332: mov r11, qword ptr [rsp + 0x80]
14155e33a: mov r10, qword ptr [rsp + 0x68]
14155e33f: mov r9, qword ptr [rsp + 0x50]
14155e344: mov r8, qword ptr [rsp + 0x28]
14155e349: mov rax, qword ptr [r15 + 0x28]
14155e34d: imul rcx, r14, 0x38
14155e351: mov qword ptr [rax + rcx], r9
14155e355: mov qword ptr [rax + rcx + 8], r10
14155e35a: movaps xmm0, xmmword ptr [rsp + 0xb0]
14155e362: movups xmmword ptr [rax + rcx + 0x10], xmm0
14155e367: mov rdx, qword ptr [rsp + 0xc0]
14155e36f: mov qword ptr [rax + rcx + 0x20], rdx
14155e374: movss dword ptr [rax + rcx + 0x28], xmm6
14155e37a: movss dword ptr [rax + rcx + 0x2c], xmm8
14155e381: mov qword ptr [rax + rcx + 0x30], r11
14155e386: inc r14
14155e389: mov qword ptr [r15 + 0x30], r14
14155e38d: inc qword ptr [r15 + 0x40]
14155e391: mov eax, dword ptr [rsi + 0x38]
14155e394: mov dword ptr [rsp + 0x3f], eax
14155e398: mov rax, qword ptr [rsi + 0x30]
14155e39c: mov qword ptr [rsp + 0x37], rax
14155e3a1: lea rcx, [rsp + 0xb4]
14155e3a9: movdqu xmm0, xmmword ptr [r8]
14155e3ae: mov rax, qword ptr [r8 + 0x10]
14155e3b2: mov qword ptr [rsp + 0xc4], rax
14155e3ba: movdqu xmmword ptr [rsp + 0xb4], xmm0
14155e3c3: mov dword ptr [rsp + 0xb0], 1
14155e3ce: call 0x140c99520
14155e3d3: movdqa xmm2, xmm0
14155e3d7: movss xmm0, dword ptr [rip + 0x170f611]
14155e3df: movss xmm1, dword ptr [rip + 0x170f60d]
14155e3e7: call 0x140c904a0
14155e3ec: movaps xmm6, xmm0
14155e3ef: mov eax, dword ptr [rsp + 0x24]
14155e3f3: mov dword ptr [r13], eax
14155e3f7: mov r14, qword ptr [r12 + 0x30]
14155e3fc: mov r15, qword ptr [r12 + 0x40]
14155e401: cmp r14, qword ptr [r12 + 0x20]
14155e406: jne 0x14155e419
14155e408: lea rcx, [r12 + 0x20]
14155e40d: lea rdx, [rip + 0x16dbd34]
14155e414: call 0x140562c30
14155e419: mov rax, qword ptr [r12 + 0x28]
14155e41e: mov rcx, r14
14155e421: shl rcx, 5
14155e425: mov byte ptr [rax + rcx], 7
14155e429: movups xmm0, xmmword ptr [rsp + 0x30]
14155e42e: movups xmmword ptr [rax + rcx + 1], xmm0
14155e433: mov edx, dword ptr [rsp + 0x3f]
14155e437: mov dword ptr [rax + rcx + 0x10], edx
14155e43b: movss dword ptr [rax + rcx + 0x14], xmm6
14155e441: mov qword ptr [rax + rcx + 0x18], r15
14155e446: inc r14
14155e449: mov qword ptr [r12 + 0x30], r14
14155e44e: inc qword ptr [r12 + 0x40]
14155e453: jmp 0x14155db5a
