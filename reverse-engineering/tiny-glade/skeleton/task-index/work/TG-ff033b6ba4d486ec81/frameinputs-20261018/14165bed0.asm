14165bed0: push r15
14165bed2: push r14
14165bed4: push r13
14165bed6: push r12
14165bed8: push rsi
14165bed9: push rdi
14165beda: push rbp
14165bedb: push rbx
14165bedc: sub rsp, 0x1f8
14165bee3: movaps xmmword ptr [rsp + 0x1e0], xmm8
14165beec: movaps xmmword ptr [rsp + 0x1d0], xmm7
14165bef4: movaps xmmword ptr [rsp + 0x1c0], xmm6
14165befc: mov r12, rdx
14165beff: mov rax, qword ptr [rsp + 0x268]
14165bf07: mov rbp, qword ptr [rax + 0x20]
14165bf0b: mov rdx, qword ptr [rax + 0x30]
14165bf0f: mov r10d, dword ptr [rax + 0x3c]
14165bf13: mov rcx, qword ptr [rcx]
14165bf16: cmp dword ptr [rcx], 6
14165bf19: jne 0x14165c4af
14165bf1f: cmp byte ptr [rcx + 4], 1
14165bf23: jne 0x14165c4af
14165bf29: mov qword ptr [rsp + 0xf8], r9
14165bf31: mov dword ptr [rsp + 0x54], r10d
14165bf36: mov qword ptr [rsp + 0x90], rdx
14165bf3e: mov qword ptr [rsp + 0x28], r8
14165bf43: mov rbx, qword ptr [rsp + 0x288]
14165bf4b: mov rcx, qword ptr [rsp + 0x270]
14165bf53: mov rdx, qword ptr [rsp + 0x290]
14165bf5b: mov r13, qword ptr [rax]
14165bf5e: mov rax, qword ptr [rcx + 0x20]
14165bf62: mov qword ptr [rsp + 0x80], rax
14165bf6a: mov rax, qword ptr [rbx + 0x20]
14165bf6e: mov qword ptr [rsp + 0xd0], rax
14165bf76: mov rax, qword ptr [rbx + 0x30]
14165bf7a: mov qword ptr [rsp + 0x100], rax
14165bf82: mov eax, dword ptr [rbx + 0x3c]
14165bf85: mov dword ptr [rsp + 0xb0], eax
14165bf8c: mov rax, qword ptr [rbx + 0x40]
14165bf90: mov qword ptr [rsp + 0x78], rax
14165bf95: mov rsi, qword ptr [rdx]
14165bf98: mov rax, qword ptr [rdx + 0x20]
14165bf9c: mov qword ptr [rsp + 0x98], rax
14165bfa4: mov rdi, qword ptr [rdx + 0x30]
14165bfa8: mov eax, dword ptr [rdx + 0x3c]
14165bfab: mov dword ptr [rsp + 0xac], eax
14165bfb2: mov rax, qword ptr [rdx + 0x40]
14165bfb6: mov qword ptr [rsp + 0xe8], rax
14165bfbe: mov rax, qword ptr [rdx + 0x50]
14165bfc2: mov qword ptr [rsp + 0xd8], rax
14165bfca: mov eax, dword ptr [rdx + 0x5c]
14165bfcd: mov dword ptr [rsp + 0xa0], eax
14165bfd4: mov rcx, qword ptr [rcx]
14165bfd7: mov qword ptr [rsp + 0x110], 0
14165bfe3: mov qword ptr [rsp + 0x118], 8
14165bfef: mov qword ptr [rsp + 0x120], 0
14165bffb: mov dword ptr [rsp + 0x128], 0
14165c006: mov dword ptr [rsp + 0x140], 0
14165c011: mov dword ptr [rsp + 0x158], 0
14165c01c: mov dword ptr [rsp + 0x170], 0
14165c027: mov byte ptr [rsp + 0x188], 0
14165c02f: mov byte ptr [rsp + 0x198], 5
14165c037: mov dword ptr [rsp + 0x19c], 0x7f7fffff
14165c042: mov word ptr [rsp + 0x1b8], 0x100
14165c04c: mov rax, qword ptr [rbx + 0x50]
14165c050: mov qword ptr [rsp + 0x70], rax
14165c055: mov eax, dword ptr [rbx + 0x5c]
14165c058: mov dword ptr [rsp + 0x24], eax
14165c05c: mov rax, qword ptr [rbx + 0x60]
14165c060: mov qword ptr [rsp + 0xf0], rax
14165c068: mov rax, qword ptr [rbx + 0x70]
14165c06c: mov qword ptr [rsp + 0xe0], rax
14165c074: mov eax, dword ptr [rbx + 0x7c]
14165c077: mov dword ptr [rsp + 0xa4], eax
14165c07e: mov rax, qword ptr [rbx + 0x80]
14165c085: mov qword ptr [rsp + 0xc0], rax
14165c08d: mov r14, qword ptr [rbx + 0x90]
14165c094: mov eax, dword ptr [rbx + 0x9c]
14165c09a: mov dword ptr [rsp + 0xa8], eax
14165c0a1: mov rax, qword ptr [rbx + 0xa0]
14165c0a8: mov qword ptr [rsp + 0x58], rax
14165c0ad: mov rax, qword ptr [rbx + 0xb0]
14165c0b4: mov qword ptr [rsp + 0xc8], rax
14165c0bc: mov eax, dword ptr [rbx + 0xbc]
14165c0c2: mov dword ptr [rsp + 0x6c], eax
14165c0c6: lea rdx, [rip + 0x162612c]
14165c0cd: lea r9, [rsp + 0x110]
14165c0d5: mov r8d, 0x12
14165c0db: call 0x1409236d0
14165c0e0: mov r9, rax
14165c0e3: mov r10, rdx
14165c0e6: movss xmm0, dword ptr [r13 + 0x3c]
14165c0ec: ucomiss xmm0, dword ptr [rip + 0x13757d9]
14165c0f3: mov qword ptr [rsp + 0x88], r13
14165c0fb: mov qword ptr [rsp + 0x60], rax
14165c100: jb 0x14165c1ae
14165c106: mov rbx, rsi
14165c109: mov rax, qword ptr [rsp + 0x298]
14165c111: movups xmm0, xmmword ptr [rbp + 0x10]
14165c115: movaps xmmword ptr [rsp + 0x110], xmm0
14165c11d: mov ecx, dword ptr [rax + 0x1c]
14165c120: mov rdx, qword ptr [rax + 0x10]
14165c124: mov dword ptr [rdx], ecx
14165c126: mov r15, qword ptr [rax]
14165c129: mov r13, qword ptr [r15 + 0x30]
14165c12d: mov rsi, qword ptr [r15 + 0x40]
14165c131: cmp r13, qword ptr [r15 + 0x20]
14165c135: jne 0x14165c168
14165c137: lea rcx, [r15 + 0x20]
14165c13b: lea rdx, [rip + 0x15de006]
14165c142: mov qword ptr [rsp + 0x48], r14
14165c147: mov r14, r12
14165c14a: mov r12, rdi
14165c14d: mov rdi, r10
14165c150: call 0x140562ab0
14165c155: mov r10, rdi
14165c158: mov rdi, r12
14165c15b: mov r12, r14
14165c15e: mov r14, qword ptr [rsp + 0x48]
14165c163: mov r9, qword ptr [rsp + 0x60]
14165c168: mov rax, qword ptr [r15 + 0x28]
14165c16c: lea rcx, [r13*4]
14165c174: add rcx, r13
14165c177: mov qword ptr [rax + rcx*8], rsi
14165c17b: mov dword ptr [rax + rcx*8 + 8], 0
14165c183: movaps xmm0, xmmword ptr [rsp + 0x110]
14165c18b: movups xmmword ptr [rax + rcx*8 + 0xc], xmm0
14165c190: inc r13
14165c193: mov qword ptr [r15 + 0x30], r13
14165c197: inc qword ptr [r15 + 0x40]
14165c19b: mov r13, qword ptr [rsp + 0x88]
14165c1a3: mov rsi, rbx
14165c1a6: mov rbx, qword ptr [rsp + 0x288]
14165c1ae: mov r11, qword ptr [rsp + 0x278]
14165c1b6: mov r15b, 2
14165c1b9: cmp byte ptr [r9 + 0x18], 2
14165c1be: jb 0x14165c423
14165c1c4: cmp qword ptr [r9 + 0x38], r10
14165c1c8: jne 0x14165c294
14165c1ce: cmp byte ptr [r9 + 0x19], 3
14165c1d3: jne 0x14165c294
14165c1d9: mov qword ptr [rsp + 0x48], rsi
14165c1de: mov ecx, dword ptr [r11 + 0x1c]
14165c1e2: mov rdx, qword ptr [r11 + 0x10]
14165c1e6: mov dword ptr [rdx], ecx
14165c1e8: mov r8, qword ptr [r11]
14165c1eb: mov byte ptr [r8 + 0xc], 0
14165c1f0: mov rax, qword ptr [rsp + 0x28]
14165c1f5: mov rax, qword ptr [rax]
14165c1f8: movsd xmm0, qword ptr [rax]
14165c1fc: movsd qword ptr [r8 + 4], xmm0
14165c202: mov dword ptr [rdx], ecx
14165c204: mov dword ptr [r8], 0
14165c20b: mov rax, qword ptr [rsp + 0xc8]
14165c213: mov ecx, dword ptr [rsp + 0x6c]
14165c217: mov dword ptr [rax], ecx
14165c219: mov rcx, qword ptr [rsp + 0x58]
14165c21e: mov rsi, qword ptr [rcx + 0x30]
14165c222: mov r13, qword ptr [rcx + 0x40]
14165c226: cmp rsi, qword ptr [rcx + 0x20]
14165c22a: jne 0x14165c269
14165c22c: mov rax, qword ptr [rsp + 0x58]
14165c231: lea rcx, [rax + 0x20]
14165c235: lea rdx, [rip + 0x15ddf0c]
14165c23c: mov qword ptr [rsp + 0x28], r12
14165c241: mov r12, rdi
14165c244: mov rdi, r10
14165c247: call 0x140562870
14165c24c: mov rcx, qword ptr [rsp + 0x58]
14165c251: mov r11, qword ptr [rsp + 0x278]
14165c259: mov r10, rdi
14165c25c: mov rdi, r12
14165c25f: mov r12, qword ptr [rsp + 0x28]
14165c264: mov r9, qword ptr [rsp + 0x60]
14165c269: mov rax, qword ptr [rcx + 0x28]
14165c26d: mov qword ptr [rax + rsi*8], r13
14165c271: inc rsi
14165c274: mov qword ptr [rcx + 0x30], rsi
14165c278: inc qword ptr [rcx + 0x40]
14165c27c: cmp byte ptr [r9 + 0x18], 2
14165c281: mov r13, qword ptr [rsp + 0x88]
14165c289: mov rsi, qword ptr [rsp + 0x48]
14165c28e: jb 0x14165c423
14165c294: cmp qword ptr [r9 + 0x38], r10
14165c298: jne 0x14165c36b
14165c29e: cmp byte ptr [r9 + 0x19], 4
14165c2a3: jne 0x14165c36b
14165c2a9: mov qword ptr [rsp + 0x48], rsi
14165c2ae: mov qword ptr [rsp + 0x28], rdi
14165c2b3: mov rax, qword ptr [r12]
14165c2b7: movsd xmm0, qword ptr [rax + 0x140]
14165c2bf: mulps xmm0, xmm0
14165c2c2: movshdup xmm1, xmm0
14165c2c6: addss xmm1, xmm0
14165c2ca: xorps xmm0, xmm0
14165c2cd: sqrtss xmm0, xmm1
14165c2d1: mov eax, dword ptr [r11 + 0x1c]
14165c2d5: mov rcx, qword ptr [r11 + 0x10]
14165c2d9: mov dword ptr [rcx], eax
14165c2db: mov rdx, qword ptr [r11]
14165c2de: addss xmm0, dword ptr [rdx]
14165c2e2: movss dword ptr [rdx], xmm0
14165c2e6: ucomiss xmm0, dword ptr [rip + 0x12d83a7]
14165c2ed: jbe 0x14165c2f5
14165c2ef: mov dword ptr [rcx], eax
14165c2f1: mov byte ptr [rdx + 0xc], 1
14165c2f5: mov rax, qword ptr [rsp + 0xc8]
14165c2fd: mov ecx, dword ptr [rsp + 0x6c]
14165c301: mov dword ptr [rax], ecx
14165c303: mov rcx, qword ptr [rsp + 0x58]
14165c308: mov rsi, qword ptr [rcx + 0x30]
14165c30c: mov rdi, qword ptr [rcx + 0x40]
14165c310: cmp rsi, qword ptr [rcx + 0x20]
14165c314: jne 0x14165c343
14165c316: mov rax, qword ptr [rsp + 0x58]
14165c31b: lea rcx, [rax + 0x20]
14165c31f: lea rdx, [rip + 0x15dde22]
14165c326: mov r12, r10
14165c329: call 0x140562870
14165c32e: mov rcx, qword ptr [rsp + 0x58]
14165c333: mov r11, qword ptr [rsp + 0x278]
14165c33b: mov r10, r12
14165c33e: mov r9, qword ptr [rsp + 0x60]
14165c343: mov rax, qword ptr [rcx + 0x28]
14165c347: mov qword ptr [rax + rsi*8], rdi
14165c34b: inc rsi
14165c34e: mov qword ptr [rcx + 0x30], rsi
14165c352: inc qword ptr [rcx + 0x40]
14165c356: cmp byte ptr [r9 + 0x18], 2
14165c35b: mov rdi, qword ptr [rsp + 0x28]
14165c360: mov rsi, qword ptr [rsp + 0x48]
14165c365: jb 0x14165c423
14165c36b: cmp qword ptr [r9 + 0x38], r10
14165c36f: jne 0x14165c418
14165c375: cmp byte ptr [r9 + 0x19], 1
14165c37a: ja 0x14165c418
14165c380: mov rbx, rbp
14165c383: mov rbp, rsi
14165c386: mov r12, rdi
14165c389: mov rax, qword ptr [r11]
14165c38c: movsd xmm6, qword ptr [rax + 4]
14165c391: mov eax, dword ptr [rsp + 0xa8]
14165c398: mov dword ptr [r14], eax
14165c39b: mov rdx, qword ptr [rsp + 0xc0]
14165c3a3: mov rdi, qword ptr [rdx + 0x30]
14165c3a7: mov rsi, qword ptr [rdx + 0x40]
14165c3ab: cmp rdi, qword ptr [rdx + 0x20]
14165c3af: jne 0x14165c3dc
14165c3b1: lea rcx, [rdx + 0x20]
14165c3b5: lea rdx, [rip + 0x15ddd8c]
14165c3bc: mov r14, r10
14165c3bf: call 0x140562db0
14165c3c4: mov r11, qword ptr [rsp + 0x278]
14165c3cc: mov rdx, qword ptr [rsp + 0xc0]
14165c3d4: mov r10, r14
14165c3d7: mov r9, qword ptr [rsp + 0x60]
14165c3dc: mov rax, qword ptr [rdx + 0x28]
14165c3e0: lea rcx, [rdi + rdi*2]
14165c3e4: mov qword ptr [rax + rcx*8], rsi
14165c3e8: mov dword ptr [rax + rcx*8 + 8], 2
14165c3f0: movlps qword ptr [rax + rcx*8 + 0xc], xmm6
14165c3f5: inc rdi
14165c3f8: mov qword ptr [rdx + 0x30], rdi
14165c3fc: inc qword ptr [rdx + 0x40]
14165c400: cmp byte ptr [r9 + 0x18], 2
14165c405: mov rdi, r12
14165c408: mov rsi, rbp
14165c40b: mov rbp, rbx
14165c40e: mov rbx, qword ptr [rsp + 0x288]
14165c416: jb 0x14165c423
14165c418: cmp qword ptr [r9 + 0x38], r10
14165c41c: jne 0x14165c423
14165c41e: movzx r15d, byte ptr [r9 + 0x19]
14165c423: add r15b, 0xfe
14165c427: movzx ecx, r15b
14165c42b: cmp cl, 3
14165c42e: mov eax, 3
14165c433: cmovb eax, ecx
14165c436: cmp al, 2
14165c438: mov qword ptr [rsp + 0x108], r10
14165c440: jae 0x14165c4be
14165c442: movss xmm0, dword ptr [r13 + 0x30]
14165c448: movss xmm1, dword ptr [r13 + 0x38]
14165c44e: mov rax, qword ptr [rsp + 0x90]
14165c456: mov ecx, dword ptr [rsp + 0x54]
14165c45a: mov dword ptr [rax], ecx
14165c45c: movss dword ptr [rbp + 0x10], xmm0
14165c461: movss dword ptr [rbp + 0x14], xmm1
14165c466: movss xmm0, dword ptr [r13 + 0x34]
14165c46c: mov dword ptr [rax], ecx
14165c46e: movss dword ptr [rbp + 0x20], xmm0
14165c473: movups xmm0, xmmword ptr [rbp + 0x10]
14165c477: movaps xmmword ptr [rsp + 0x30], xmm0
14165c47c: lea r15, [rsp + 0x110]
14165c484: lea rdx, [rsp + 0x30]
14165c489: mov r12b, 1
14165c48c: mov rcx, r15
14165c48f: mov r8b, 1
14165c492: call 0x14165cd40
14165c497: mov rcx, qword ptr [rsp + 0x80]
14165c49f: mov rdx, r15
14165c4a2: call 0x14146c880
14165c4a7: xorps xmm7, xmm7
14165c4aa: jmp 0x14165c7d8
14165c4af: mov dword ptr [rdx], r10d
14165c4b2: mov dword ptr [rbp], 0
14165c4b9: jmp 0x14165cd12
14165c4be: movzx eax, al
14165c4c1: cmp eax, 2
14165c4c4: jne 0x14165c5af
14165c4ca: mov rax, qword ptr [r11]
14165c4cd: xorps xmm7, xmm7
14165c4d0: mov r12b, 1
14165c4d3: cmp byte ptr [rax + 0xc], 0
14165c4d7: mov r8b, 1
14165c4da: je 0x14165c578
14165c4e0: mov rcx, qword ptr [rsp + 0xf8]
14165c4e8: mov rcx, qword ptr [rcx]
14165c4eb: movss xmm0, dword ptr [r13 + 0x34]
14165c4f1: subss xmm0, dword ptr [rcx + 4]
14165c4f6: divss xmm0, dword ptr [rcx + 0x10]
14165c4fb: movsd xmm1, qword ptr [rbp + 0x10]
14165c500: movss xmm2, dword ptr [rcx]
14165c504: movss xmm3, dword ptr [rcx + 8]
14165c509: unpcklps xmm2, xmm3
14165c50c: movss xmm3, dword ptr [rcx + 0xc]
14165c511: movss xmm4, dword ptr [rcx + 0x14]
14165c516: unpcklps xmm3, xmm4
14165c519: movsldup xmm0, xmm0
14165c51d: mulps xmm0, xmm3
14165c520: addps xmm0, xmm2
14165c523: subps xmm1, xmm0
14165c526: mulps xmm1, xmm1
14165c529: movshdup xmm0, xmm1
14165c52d: addss xmm0, xmm1
14165c531: sqrtss xmm0, xmm0
14165c535: movss xmm1, dword ptr [rip + 0x1420343]
14165c53d: maxss xmm1, xmm0
14165c541: movss xmm0, dword ptr [rip + 0x12d80bb]
14165c549: minss xmm0, xmm1
14165c54d: movaps xmm7, xmm0
14165c550: subss xmm7, dword ptr [rbp + 0x18]
14165c555: mov rcx, qword ptr [rsp + 0x90]
14165c55d: mov edx, dword ptr [rsp + 0x54]
14165c561: mov dword ptr [rcx], edx
14165c563: movss dword ptr [rbp + 0x18], xmm0
14165c568: andps xmm7, xmmword ptr [rip + 0x12d2411]
14165c56f: movzx r8d, byte ptr [rax + 0xc]
14165c574: xor r8b, 1
14165c578: movups xmm0, xmmword ptr [rbp + 0x10]
14165c57c: movaps xmmword ptr [rsp + 0x30], xmm0
14165c581: and r8b, 1
14165c585: lea r15, [rsp + 0x110]
14165c58d: lea rdx, [rsp + 0x30]
14165c592: mov rcx, r15
14165c595: call 0x14165cd40
14165c59a: mov rcx, qword ptr [rsp + 0x80]
14165c5a2: mov rdx, r15
14165c5a5: call 0x14146c880
14165c5aa: jmp 0x14165c7d8
14165c5af: mov rax, qword ptr [r11]
14165c5b2: mov r12b, 1
14165c5b5: xorps xmm7, xmm7
14165c5b8: cmp byte ptr [rax + 0xc], 1
14165c5bc: je 0x14165c7d8
14165c5c2: mov qword ptr [rsp + 0x28], rdi
14165c5c7: movups xmm0, xmmword ptr [rbp + 0x10]
14165c5cb: movups xmmword ptr [rsp + 0x114], xmm0
14165c5d3: mov dword ptr [rsp + 0x110], 0
14165c5de: lea rdx, [rsp + 0x110]
14165c5e6: mov rcx, rsi
14165c5e9: call 0x140ad2970
14165c5ee: test al, al
14165c5f0: je 0x14165c602
14165c5f2: lea rdi, [rbp + 0x10]
14165c5f6: cmp byte ptr [r13 + 0x2c], 2
14165c5fb: jne 0x14165c65f
14165c5fd: xor r14d, r14d
14165c600: jmp 0x14165c669
14165c602: mov r14, rsi
14165c605: mov rax, qword ptr [rsp + 0xd8]
14165c60d: mov ecx, dword ptr [rsp + 0xa0]
14165c614: mov dword ptr [rax], ecx
14165c616: mov r15, qword ptr [rsp + 0xe8]
14165c61e: mov rsi, qword ptr [r15 + 0x30]
14165c622: mov rdi, qword ptr [r15 + 0x40]
14165c626: cmp rsi, qword ptr [r15 + 0x20]
14165c62a: jne 0x14165c63c
14165c62c: lea rcx, [r15 + 0x20]
14165c630: lea rdx, [rip + 0x15ddb11]
14165c637: call 0x140562870
14165c63c: mov rax, qword ptr [r15 + 0x28]
14165c640: mov qword ptr [rax + rsi*8], rdi
14165c644: inc rsi
14165c647: mov qword ptr [r15 + 0x30], rsi
14165c64b: inc qword ptr [r15 + 0x40]
14165c64f: xor r12d, r12d
14165c652: mov rdi, qword ptr [rsp + 0x28]
14165c657: mov rsi, r14
14165c65a: jmp 0x14165c7d8
14165c65f: mov rdx, qword ptr [r13 + 0x20]
14165c663: mov r14d, 1
14165c669: mov qword ptr [rsp + 0x48], rsi
14165c66e: movss xmm6, dword ptr [r13 + 0x40]
14165c674: movss xmm8, dword ptr [r13 + 0x3c]
14165c67a: movups xmm0, xmmword ptr [rdi]
14165c67d: movaps xmmword ptr [rsp + 0x110], xmm0
14165c685: mov rax, qword ptr [rsp + 0xe0]
14165c68d: mov ecx, dword ptr [rsp + 0xa4]
14165c694: mov dword ptr [rax], ecx
14165c696: mov rsi, qword ptr [rsp + 0xf0]
14165c69e: mov r13, qword ptr [rsi + 0x30]
14165c6a2: mov r15, qword ptr [rsi + 0x40]
14165c6a6: cmp r13, qword ptr [rsi + 0x20]
14165c6aa: jne 0x14165c6cc
14165c6ac: lea rcx, [rsi + 0x20]
14165c6b0: mov qword ptr [rsp + 0x80], rdx
14165c6b8: lea rdx, [rip + 0x15dda89]
14165c6bf: call 0x1405629f0
14165c6c4: mov rdx, qword ptr [rsp + 0x80]
14165c6cc: mov rax, qword ptr [rsi + 0x28]
14165c6d0: lea rcx, [r13*2]
14165c6d8: add rcx, r13
14165c6db: shl rcx, 4
14165c6df: mov qword ptr [rax + rcx], r14
14165c6e3: mov qword ptr [rax + rcx + 8], rdx
14165c6e8: movaps xmm0, xmmword ptr [rsp + 0x110]
14165c6f0: movups xmmword ptr [rax + rcx + 0x10], xmm0
14165c6f5: movss dword ptr [rax + rcx + 0x20], xmm6
14165c6fb: movss dword ptr [rax + rcx + 0x24], xmm8
14165c702: mov qword ptr [rax + rcx + 0x28], r15
14165c707: inc r13
14165c70a: mov qword ptr [rsi + 0x30], r13
14165c70e: inc qword ptr [rsi + 0x40]
14165c712: mov r13, qword ptr [rsp + 0x88]
14165c71a: mov eax, dword ptr [r13 + 0x38]
14165c71e: mov dword ptr [rsp + 0x3f], eax
14165c722: mov rax, qword ptr [r13 + 0x30]
14165c726: mov qword ptr [rsp + 0x37], rax
14165c72b: lea rcx, [rsp + 0x114]
14165c733: movups xmm0, xmmword ptr [rdi]
14165c736: movups xmmword ptr [rsp + 0x114], xmm0
14165c73e: mov dword ptr [rsp + 0x110], 0
14165c749: call 0x140c97580
14165c74e: movaps xmm2, xmm0
14165c751: movss xmm0, dword ptr [rip + 0x1625297]
14165c759: movss xmm1, dword ptr [rip + 0x1625293]
14165c761: call 0x140c904a0
14165c766: movaps xmm6, xmm0
14165c769: mov rax, qword ptr [rsp + 0x70]
14165c76e: mov ecx, dword ptr [rsp + 0x24]
14165c772: mov dword ptr [rax], ecx
14165c774: mov rsi, qword ptr [rsp + 0x78]
14165c779: mov rdi, qword ptr [rsi + 0x30]
14165c77d: mov r14, qword ptr [rsi + 0x40]
14165c781: cmp rdi, qword ptr [rsi + 0x20]
14165c785: jne 0x14165c797
14165c787: lea rcx, [rsi + 0x20]
14165c78b: lea rdx, [rip + 0x15dd9b6]
14165c792: call 0x140562c30
14165c797: mov rax, qword ptr [rsi + 0x28]
14165c79b: mov rcx, rdi
14165c79e: shl rcx, 5
14165c7a2: mov byte ptr [rax + rcx], 7
14165c7a6: movups xmm0, xmmword ptr [rsp + 0x30]
14165c7ab: movups xmmword ptr [rax + rcx + 1], xmm0
14165c7b0: mov edx, dword ptr [rsp + 0x3f]
14165c7b4: mov dword ptr [rax + rcx + 0x10], edx
14165c7b8: movss dword ptr [rax + rcx + 0x14], xmm6
14165c7be: mov qword ptr [rax + rcx + 0x18], r14
14165c7c3: inc rdi
14165c7c6: mov qword ptr [rsi + 0x30], rdi
14165c7ca: inc qword ptr [rsi + 0x40]
14165c7ce: mov rdi, qword ptr [rsp + 0x28]
14165c7d3: mov rsi, qword ptr [rsp + 0x48]
14165c7d8: movups xmm0, xmmword ptr [rbp + 0x10]
14165c7dc: movups xmmword ptr [rsp + 0x114], xmm0
14165c7e4: mov dword ptr [rsp + 0x110], 0
14165c7ef: lea rdx, [rsp + 0x110]
14165c7f7: mov rcx, rsi
14165c7fa: call 0x140ad2970
14165c7ff: test al, al
14165c801: jne 0x14165c855
14165c803: mov eax, dword ptr [rsp + 0xac]
14165c80a: mov dword ptr [rdi], eax
14165c80c: mov rcx, qword ptr [rsp + 0x98]
14165c814: mov rsi, qword ptr [rcx + 0x30]
14165c818: mov rdi, qword ptr [rcx + 0x40]
14165c81c: cmp rsi, qword ptr [rcx + 0x20]
14165c820: jne 0x14165c842
14165c822: mov rax, qword ptr [rsp + 0x98]
14165c82a: lea rcx, [rax + 0x20]
14165c82e: lea rdx, [rip + 0x15dd913]
14165c835: call 0x140562870
14165c83a: mov rcx, qword ptr [rsp + 0x98]
14165c842: mov rax, qword ptr [rcx + 0x28]
14165c846: mov qword ptr [rax + rsi*8], rdi
14165c84a: inc rsi
14165c84d: mov qword ptr [rcx + 0x30], rsi
14165c851: inc qword ptr [rcx + 0x40]
14165c855: mov rdi, qword ptr [rsp + 0x280]
14165c85d: test r12b, r12b
14165c860: je 0x14165c984
14165c866: mov r14, qword ptr [rbp + 0x10]
14165c86a: movss xmm6, dword ptr [rbp + 0x18]
14165c86f: mov r15d, dword ptr [rbp + 0x1c]
14165c873: addss xmm6, dword ptr [rip + 0x14241d1]
14165c87b: mov eax, dword ptr [r13 + 0x2c]
14165c87f: xor r12d, r12d
14165c882: cmp al, 2
14165c884: cmovne r12d, eax
14165c888: mov eax, dword ptr [rbx + 0x1c]
14165c88b: mov rcx, qword ptr [rbx + 0x10]
14165c88f: mov dword ptr [rcx], eax
14165c891: mov rbx, qword ptr [rbx]
14165c894: mov r13, qword ptr [rbx + 0x30]
14165c898: mov rsi, qword ptr [rbx + 0x40]
14165c89c: cmp r13, qword ptr [rbx + 0x20]
14165c8a0: jne 0x14165c8b2
14165c8a2: lea rcx, [rbx + 0x20]
14165c8a6: lea rdx, [rip + 0x15dd89b]
14165c8ad: call 0x140562ab0
14165c8b2: mov rax, qword ptr [rbx + 0x28]
14165c8b6: lea rcx, [r13*4]
14165c8be: add rcx, r13
14165c8c1: mov dword ptr [rax + rcx*8], 0
14165c8c8: mov qword ptr [rax + rcx*8 + 4], r14
14165c8cd: movss dword ptr [rax + rcx*8 + 0xc], xmm6
14165c8d3: mov dword ptr [rax + rcx*8 + 0x10], r15d
14165c8d8: mov byte ptr [rax + rcx*8 + 0x1c], r12b
14165c8dd: mov qword ptr [rax + rcx*8 + 0x20], rsi
14165c8e2: inc r13
14165c8e5: mov qword ptr [rbx + 0x30], r13
14165c8e9: inc qword ptr [rbx + 0x40]
14165c8ed: mov r13, qword ptr [rsp + 0x88]
14165c8f5: movss xmm8, dword ptr [r13 + 0x3c]
14165c8fb: mov rax, qword ptr [rsp + 0x100]
14165c903: mov ecx, dword ptr [rsp + 0xb0]
14165c90a: mov dword ptr [rax], ecx
14165c90c: mov rax, qword ptr [rsp + 0xd0]
14165c914: mov rbx, qword ptr [rax + 0x30]
14165c918: mov r12, qword ptr [rax + 0x40]
14165c91c: cmp rbx, qword ptr [rax + 0x20]
14165c920: jne 0x14165c93a
14165c922: lea rcx, [rax + 0x20]
14165c926: lea rdx, [rip + 0x15dd81b]
14165c92d: call 0x1405629f0
14165c932: mov rax, qword ptr [rsp + 0xd0]
14165c93a: mov rdx, rax
14165c93d: mov rax, qword ptr [rax + 0x28]
14165c941: lea rcx, [rbx + rbx*2]
14165c945: shl rcx, 4
14165c949: mov dword ptr [rax + rcx], 0
14165c950: mov qword ptr [rax + rcx + 4], r14
14165c955: movss dword ptr [rax + rcx + 0xc], xmm6
14165c95b: mov dword ptr [rax + rcx + 0x10], r15d
14165c960: movss dword ptr [rax + rcx + 0x1c], xmm8
14165c967: mov dword ptr [rax + rcx + 0x20], 0x40800000
14165c96f: mov byte ptr [rax + rcx + 0x24], 0
14165c974: mov qword ptr [rax + rcx + 0x28], r12
14165c979: inc rbx
14165c97c: mov qword ptr [rdx + 0x30], rbx
14165c980: inc qword ptr [rdx + 0x40]
14165c984: mov rdx, qword ptr [rdi]
14165c987: mov rcx, qword ptr [rdi + 8]
14165c98b: mov r8, qword ptr [rdx]
14165c98e: xor eax, eax
14165c990: mov r9, r8
14165c993: sub r9, qword ptr [rcx + 0x18]
14165c997: cmovb r9, rax
14165c99b: sub r8, qword ptr [rcx + 0x38]
14165c99f: cmovb r8, rax
14165c9a3: mov r10, qword ptr [rcx + 0x30]
14165c9a7: sub r10, r8
14165c9aa: cmovb r10, rax
14165c9ae: setbe r8b
14165c9b2: mov r11, qword ptr [rcx + 0x10]
14165c9b6: sub r11, r9
14165c9b9: cmovb r11, rax
14165c9bd: mov r9, qword ptr [rcx + 0x40]
14165c9c1: setbe cl
14165c9c4: add r11, r10
14165c9c7: and cl, r8b
14165c9ca: cmove r11, rax
14165c9ce: sub r9, r11
14165c9d1: mov qword ptr [rdx], r9
14165c9d4: mov rdx, qword ptr [rsp + 0x60]
14165c9d9: cmp byte ptr [rdx + 0x18], 2
14165c9dd: jb 0x14165c9f7
14165c9df: mov rax, qword ptr [rsp + 0x108]
14165c9e7: cmp qword ptr [rdx + 0x38], rax
14165c9eb: jne 0x14165c9f5
14165c9ed: movzx eax, byte ptr [rdx + 0x19]
14165c9f1: add al, 0xfe
14165c9f3: jmp 0x14165c9f7
14165c9f5: xor eax, eax
14165c9f7: lea rsi, [rbp + 0x10]
14165c9fb: movzx eax, al
14165c9fe: cmp al, 3
14165ca00: mov edx, 3
14165ca05: cmovb edx, eax
14165ca08: movzx eax, dl
14165ca0b: lea rdx, [rip + 0x1625caa]
14165ca12: movsxd rax, dword ptr [rdx + rax*4]
14165ca16: add rax, rdx
14165ca19: jmp rax
14165ca1b: test cl, cl
14165ca1d: jne 0x14165cd12
14165ca23: mov rbx, qword ptr [rsp + 0x260]
14165ca2b: movss xmm2, dword ptr [r13 + 0x3c]
14165ca31: lea rcx, [rsp + 0xb4]
14165ca39: mov rdx, rsi
14165ca3c: call 0x140c9cde0
14165ca41: movss xmm2, dword ptr [rsp + 0xb4]
14165ca4a: movss xmm0, dword ptr [rsp + 0xb8]
14165ca53: movss xmm1, dword ptr [rsp + 0xbc]
14165ca5c: test byte ptr [rbp], 1
14165ca60: jne 0x14165cbda
14165ca66: movaps xmm3, xmm2
14165ca69: movaps xmm4, xmm0
14165ca6c: movaps xmm5, xmm1
14165ca6f: jmp 0x14165cbe9
14165ca74: mov eax, dword ptr [r13 + 0x38]
14165ca78: mov dword ptr [rsp + 0x3f], eax
14165ca7c: mov rax, qword ptr [r13 + 0x30]
14165ca80: mov qword ptr [rsp + 0x37], rax
14165ca85: lea rcx, [rsp + 0x114]
14165ca8d: movups xmm0, xmmword ptr [rsi]
14165ca90: movups xmmword ptr [rsp + 0x114], xmm0
14165ca98: mov dword ptr [rsp + 0x110], 0
14165caa3: call 0x140c97580
14165caa8: movaps xmm2, xmm0
14165caab: movss xmm0, dword ptr [rip + 0x1624f3d]
14165cab3: movss xmm1, dword ptr [rip + 0x1624f39]
14165cabb: call 0x140c904a0
14165cac0: movaps xmm6, xmm0
14165cac3: mov rax, qword ptr [rsp + 0x70]
14165cac8: mov ecx, dword ptr [rsp + 0x24]
14165cacc: mov dword ptr [rax], ecx
14165cace: mov rbx, qword ptr [rsp + 0x78]
14165cad3: mov rsi, qword ptr [rbx + 0x30]
14165cad7: mov rdi, qword ptr [rbx + 0x40]
14165cadb: cmp rsi, qword ptr [rbx + 0x20]
14165cadf: jne 0x14165caf1
14165cae1: lea rcx, [rbx + 0x20]
14165cae5: lea rdx, [rip + 0x15dd65c]
14165caec: call 0x140562c30
14165caf1: mov rax, qword ptr [rbx + 0x28]
14165caf5: mov rcx, rsi
14165caf8: shl rcx, 5
14165cafc: mov byte ptr [rax + rcx], 2
14165cb00: movups xmm0, xmmword ptr [rsp + 0x30]
14165cb05: movups xmmword ptr [rax + rcx + 1], xmm0
14165cb0a: mov edx, dword ptr [rsp + 0x3f]
14165cb0e: jmp 0x14165cbbb
14165cb13: test cl, cl
14165cb15: jne 0x14165cd12
14165cb1b: mov eax, dword ptr [r13 + 0x38]
14165cb1f: mov dword ptr [rsp + 0x38], eax
14165cb23: mov rax, qword ptr [r13 + 0x30]
14165cb27: mov qword ptr [rsp + 0x30], rax
14165cb2c: lea rcx, [rsp + 0x114]
14165cb34: movups xmm0, xmmword ptr [rsi]
14165cb37: movups xmmword ptr [rsp + 0x114], xmm0
14165cb3f: mov dword ptr [rsp + 0x110], 0
14165cb4a: call 0x140c97580
14165cb4f: movaps xmm2, xmm0
14165cb52: movss xmm0, dword ptr [rip + 0x1624e96]
14165cb5a: movss xmm1, dword ptr [rip + 0x1624e92]
14165cb62: call 0x140c904a0
14165cb67: movaps xmm6, xmm0
14165cb6a: mov rax, qword ptr [rsp + 0x70]
14165cb6f: mov ecx, dword ptr [rsp + 0x24]
14165cb73: mov dword ptr [rax], ecx
14165cb75: mov rbx, qword ptr [rsp + 0x78]
14165cb7a: mov rsi, qword ptr [rbx + 0x30]
14165cb7e: mov rdi, qword ptr [rbx + 0x40]
14165cb82: cmp rsi, qword ptr [rbx + 0x20]
14165cb86: jne 0x14165cb98
14165cb88: lea rcx, [rbx + 0x20]
14165cb8c: lea rdx, [rip + 0x15dd5b5]
14165cb93: call 0x140562c30
14165cb98: mov rax, qword ptr [rbx + 0x28]
14165cb9c: mov rcx, rsi
14165cb9f: shl rcx, 5
14165cba3: mov byte ptr [rax + rcx], 0
14165cba7: movss dword ptr [rax + rcx + 4], xmm7
14165cbad: mov rdx, qword ptr [rsp + 0x30]
14165cbb2: mov qword ptr [rax + rcx + 8], rdx
14165cbb7: mov edx, dword ptr [rsp + 0x38]
14165cbbb: mov dword ptr [rax + rcx + 0x10], edx
14165cbbf: movss dword ptr [rax + rcx + 0x14], xmm6
14165cbc5: mov qword ptr [rax + rcx + 0x18], rdi
14165cbca: inc rsi
14165cbcd: mov qword ptr [rbx + 0x30], rsi
14165cbd1: inc qword ptr [rbx + 0x40]
14165cbd5: jmp 0x14165cd12
14165cbda: movss xmm3, dword ptr [rbp + 4]
14165cbdf: movss xmm4, dword ptr [rbp + 8]
14165cbe4: movss xmm5, dword ptr [rbp + 0xc]
14165cbe9: lea rdi, [rbp + 4]
14165cbed: subss xmm2, xmm3
14165cbf1: subss xmm0, xmm4
14165cbf5: subss xmm1, xmm5
14165cbf9: mulss xmm2, xmm2
14165cbfd: mulss xmm0, xmm0
14165cc01: addss xmm0, xmm2
14165cc05: mulss xmm1, xmm1
14165cc09: addss xmm1, xmm0
14165cc0d: sqrtss xmm7, xmm1
14165cc11: mov rax, qword ptr [rbx]
14165cc14: divss xmm7, dword ptr [rax + 0x28]
14165cc19: xorps xmm0, xmm0
14165cc1c: cmpeqss xmm0, dword ptr [r13 + 0x3c]
14165cc23: movd ebx, xmm0
14165cc27: and ebx, 1
14165cc2a: mov rax, qword ptr [r13 + 0x30]
14165cc2e: mov qword ptr [rsp + 0x30], rax
14165cc33: mov eax, dword ptr [r13 + 0x38]
14165cc37: mov dword ptr [rsp + 0x38], eax
14165cc3b: lea rcx, [rsp + 0x114]
14165cc43: movups xmm0, xmmword ptr [rsi]
14165cc46: movups xmmword ptr [rsp + 0x114], xmm0
14165cc4e: mov dword ptr [rsp + 0x110], 0
14165cc59: call 0x140c97580
14165cc5e: movaps xmm2, xmm0
14165cc61: movss xmm0, dword ptr [rip + 0x1624d87]
14165cc69: movss xmm1, dword ptr [rip + 0x1624d83]
14165cc71: call 0x140c904a0
14165cc76: movaps xmm6, xmm0
14165cc79: mov rax, qword ptr [rsp + 0x70]
14165cc7e: mov ecx, dword ptr [rsp + 0x24]
14165cc82: mov dword ptr [rax], ecx
14165cc84: mov r15, qword ptr [rsp + 0x78]
14165cc89: mov rsi, qword ptr [r15 + 0x30]
14165cc8d: mov r14, qword ptr [r15 + 0x40]
14165cc91: cmp rsi, qword ptr [r15 + 0x20]
14165cc95: jne 0x14165cca7
14165cc97: lea rcx, [r15 + 0x20]
14165cc9b: lea rdx, [rip + 0x15dd4a6]
14165cca2: call 0x140562c30
14165cca7: mov rax, qword ptr [r15 + 0x28]
14165ccab: mov rcx, rsi
14165ccae: shl rcx, 5
14165ccb2: mov byte ptr [rax + rcx], 3
14165ccb6: mov byte ptr [rax + rcx + 1], bl
14165ccba: movss dword ptr [rax + rcx + 4], xmm7
14165ccc0: mov rdx, qword ptr [rsp + 0x30]
14165ccc5: mov qword ptr [rax + rcx + 8], rdx
14165ccca: mov edx, dword ptr [rsp + 0x38]
14165ccce: mov dword ptr [rax + rcx + 0x10], edx
14165ccd2: movss dword ptr [rax + rcx + 0x14], xmm6
14165ccd8: mov qword ptr [rax + rcx + 0x18], r14
14165ccdd: inc rsi
14165cce0: mov qword ptr [r15 + 0x30], rsi
14165cce4: inc qword ptr [r15 + 0x40]
14165cce8: mov rax, qword ptr [rsp + 0x90]
14165ccf0: mov ecx, dword ptr [rsp + 0x54]
14165ccf4: mov dword ptr [rax], ecx
14165ccf6: mov dword ptr [rbp], 1
14165ccfd: mov eax, dword ptr [rsp + 0xbc]
14165cd04: mov dword ptr [rdi + 8], eax
14165cd07: mov rax, qword ptr [rsp + 0xb4]
14165cd0f: mov qword ptr [rdi], rax
14165cd12: movaps xmm6, xmmword ptr [rsp + 0x1c0]
14165cd1a: movaps xmm7, xmmword ptr [rsp + 0x1d0]
14165cd22: movaps xmm8, xmmword ptr [rsp + 0x1e0]
14165cd2b: add rsp, 0x1f8
14165cd32: pop rbx
14165cd33: pop rbp
14165cd34: pop rdi
14165cd35: pop rsi
14165cd36: pop r12
14165cd38: pop r13
14165cd3a: pop r14
14165cd3c: pop r15
14165cd3e: ret 
