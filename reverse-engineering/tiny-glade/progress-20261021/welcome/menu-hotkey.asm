0x14067e6d0: push r15
0x14067e6d2: push r14
0x14067e6d4: push r13
0x14067e6d6: push r12
0x14067e6d8: push rsi
0x14067e6d9: push rdi
0x14067e6da: push rbx
0x14067e6db: sub rsp, 0x70
0x14067e6df: mov ebx, r8d
0x14067e6e2: mov rdi, rdx
0x14067e6e5: mov rsi, rcx
0x14067e6e8: mov rcx, rdx
0x14067e6eb: mov dl, 1
0x14067e6ed: call 0x140f9dba0
0x14067e6f2: mov byte ptr [rax + 0x25d], bl
0x14067e6f8: cmp bl, 0x12
0x14067e6fb: je 0x14067e89b
0x14067e701: mov r12, qword ptr [rdi]
0x14067e704: mov r14, qword ptr [rdi + 8]
0x14067e708: mov qword ptr [rsp + 0x40], r14
0x14067e70d: mov byte ptr [rsp + 0x48], bl
0x14067e711: lea rcx, [rsp + 0x40]
0x14067e716: call 0x140fabaf0
0x14067e71b: movabs r15, 0xb1695320004fb443
0x14067e725: xor r15, rax
0x14067e728: mov rcx, r12
0x14067e72b: mov rdx, r15
0x14067e72e: call 0x140683ec0
0x14067e733: mov byte ptr [rax + 0x5d], 0
0x14067e737: test dl, 1
0x14067e73a: jne 0x14067e793
0x14067e73c: mov r13, qword ptr [r12 + 0x10]
0x14067e741: cmp r13, qword ptr [r12]
0x14067e745: jne 0x14067e756
0x14067e747: lea rdx, [rip + 0x253087a]
0x14067e74e: mov rcx, r12
0x14067e751: call 0x1406319d0
0x14067e756: mov rax, qword ptr [r12 + 8]
0x14067e75b: imul rcx, r13, 0x38
0x14067e75f: mov qword ptr [rax + rcx], 3
0x14067e767: mov qword ptr [rax + rcx + 0x10], r14
0x14067e76c: movaps xmm0, xmmword ptr [rip + 0x23986dd]
0x14067e773: movups xmmword ptr [rax + rcx + 0x18], xmm0
0x14067e778: movq xmm0, qword ptr [rip + 0x22aff50]
0x14067e780: movq qword ptr [rax + rcx + 0x28], xmm0
0x14067e786: mov qword ptr [rax + rcx + 0x30], r15
0x14067e78b: inc r13
0x14067e78e: mov qword ptr [r12 + 0x10], r13
0x14067e793: mov qword ptr [rsp + 0x28], r12
0x14067e798: mov qword ptr [rsp + 0x30], r15
0x14067e79d: mov qword ptr [rsp + 0x38], 0
0x14067e7a6: cmp qword ptr [r12 + 0x1b8], 0
0x14067e7af: je 0x14067e8bc
0x14067e7b5: lea rdx, [rsp + 0x30]
0x14067e7ba: lea rcx, [r12 + 0x1c0]
0x14067e7c2: call 0x1405b0d70
0x14067e7c7: mov rcx, qword ptr [r12 + 0x1a0]
0x14067e7cf: mov rdx, qword ptr [r12 + 0x1a8]
0x14067e7d7: mov r8, rdx
0x14067e7da: and r8, rax
0x14067e7dd: shr rax, 0x39
0x14067e7e1: movd xmm0, eax
0x14067e7e5: punpcklbw xmm0, xmm0
0x14067e7e9: pshuflw xmm0, xmm0, 0
0x14067e7ee: pshufd xmm0, xmm0, 0
0x14067e7f3: lea rax, [rcx - 0xc0]
0x14067e7fa: xor r9d, r9d
0x14067e7fd: pcmpeqd xmm1, xmm1
0x14067e801: movdqu xmm2, xmmword ptr [rcx + r8]
0x14067e807: movdqa xmm3, xmm2
0x14067e80b: pcmpeqb xmm3, xmm0
0x14067e80f: pmovmskb r10d, xmm3
0x14067e814: test r10d, r10d
0x14067e817: je 0x14067e842
0x14067e819: tzcnt r11d, r10d
0x14067e81e: add r11, r8
0x14067e821: and r11, rdx
0x14067e824: neg r11
0x14067e827: lea r11, [r11 + r11*2]
0x14067e82b: shl r11, 6
0x14067e82f: cmp r15, qword ptr [rax + r11]
0x14067e833: je 0x14067e860
0x14067e835: lea r11d, [r10 - 1]
0x14067e839: and r11w, r10w
0x14067e83d: mov r10d, r11d
0x14067e840: jne 0x14067e819
0x14067e842: pcmpeqb xmm2, xmm1
0x14067e846: pmovmskb r10d, xmm2
0x14067e84b: test r10d, r10d
0x14067e84e: jne 0x14067e8bc
0x14067e850: add r8, r9
0x14067e853: add r8, 0x10
0x14067e857: add r9, 0x10
0x14067e85b: and r8, rdx
0x14067e85e: jmp 0x14067e801
0x14067e860: mov dword ptr [rcx + r11 - 0x10], 0x43480000
0x14067e869: mov rax, qword ptr [rsp + 0x38]
0x14067e86e: mov qword ptr [rsp + 0x60], rax
0x14067e873: movups xmm0, xmmword ptr [rsp + 0x28]
0x14067e878: movaps xmmword ptr [rsp + 0x50], xmm0
0x14067e87d: mov eax, 0xcafebabe
0x14067e882: imul r14, rax
0x14067e886: lea rcx, [rsp + 0x28]
0x14067e88b: lea rdx, [rsp + 0x50]
0x14067e890: mov r8, r14
0x14067e893: mov r9d, ebx
0x14067e896: call 0x140f9fdf0
0x14067e89b: mov rax, qword ptr [rdi + 0x10]
0x14067e89f: mov qword ptr [rsi + 0x10], rax
0x14067e8a3: movups xmm0, xmmword ptr [rdi]
0x14067e8a6: movups xmmword ptr [rsi], xmm0
0x14067e8a9: mov rax, rsi
0x14067e8ac: add rsp, 0x70
0x14067e8b0: pop rbx
0x14067e8b1: pop rdi
0x14067e8b2: pop rsi
0x14067e8b3: pop r12
0x14067e8b5: pop r13
0x14067e8b7: pop r14
0x14067e8b9: pop r15
0x14067e8bb: ret 
0x14067e8bc: lea rcx, [rip + 0x239972d]
0x14067e8c3: call 0x1428d9310
0x14067e8c8: int3 
