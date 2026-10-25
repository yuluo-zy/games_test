140912c20: push r15
140912c22: push r14
140912c24: push r13
140912c26: push r12
140912c28: push rsi
140912c29: push rdi
140912c2a: push rbp
140912c2b: push rbx
140912c2c: sub rsp, 0x3d8
140912c33: movaps xmmword ptr [rsp + 0x3c0], xmm13
140912c3c: movaps xmmword ptr [rsp + 0x3b0], xmm12
140912c45: movaps xmmword ptr [rsp + 0x3a0], xmm11
140912c4e: movaps xmmword ptr [rsp + 0x390], xmm10
140912c57: movaps xmmword ptr [rsp + 0x380], xmm9
140912c60: movaps xmmword ptr [rsp + 0x370], xmm8
140912c69: movaps xmmword ptr [rsp + 0x360], xmm7
140912c71: movaps xmmword ptr [rsp + 0x350], xmm6
140912c79: mov qword ptr [rsp + 0x80], r9
140912c81: mov qword ptr [rsp + 0x88], r8
140912c89: mov rax, qword ptr [rsp + 0x460]
140912c91: mov r8, qword ptr [rsp + 0x458]
140912c99: mov r10, qword ptr [rsp + 0x448]
140912ca1: mov r9, qword ptr [rsp + 0x440]
140912ca9: mov r11, qword ptr [rsp + 0x468]
140912cb1: mov rsi, qword ptr [r10]
140912cb4: mov qword ptr [rsp + 0x60], rsi
140912cb9: mov rsi, qword ptr [r10 + 8]
140912cbd: mov qword ptr [rsp + 0x50], rsi
140912cc2: mov r14, qword ptr [r10 + 0x18]
140912cc6: mov rsi, qword ptr [r10 + 0x38]
140912cca: mov qword ptr [rsp + 0x40], rsi
140912ccf: movups xmm0, xmmword ptr [r10 + 0x58]
140912cd4: movups xmm1, xmmword ptr [r10 + 0x68]
140912cd9: movups xmm2, xmmword ptr [r10 + 0x78]
140912cde: movups xmm3, xmmword ptr [r10 + 0x88]
140912ce6: movaps xmmword ptr [rsp + 0x340], xmm3
140912cee: movaps xmmword ptr [rsp + 0x330], xmm2
140912cf6: movaps xmmword ptr [rsp + 0x320], xmm1
140912cfe: movaps xmmword ptr [rsp + 0x310], xmm0
140912d06: mov rsi, qword ptr [r10 + 0xd8]
140912d0d: mov r11, qword ptr [r11]
140912d10: cmp dword ptr [r11 + 0x48], 2
140912d15: je 0x140912d21
140912d17: xorps xmm6, xmm6
140912d1a: test byte ptr [r11 + 0x55], 1
140912d1f: jne 0x140912d2e
140912d21: mov r11, qword ptr [r10 + 0xb8]
140912d28: movss xmm6, dword ptr [r11 + 0x28]
140912d2e: mov rbx, qword ptr [r10 + 0x98]
140912d35: mov r11, qword ptr [r10 + 0xf8]
140912d3c: mov qword ptr [rsp + 0x90], r11
140912d44: mov r11, qword ptr [r10 + 0x118]
140912d4b: mov qword ptr [rsp + 0x58], r11
140912d50: mov r15, qword ptr [r10 + 0x138]
140912d57: mov r11, qword ptr [rax + 0x20]
140912d5b: mov r10d, dword ptr [rsi]
140912d5e: xorps xmm0, xmm0
140912d61: cvtsi2ss xmm0, r10
140912d66: mov r10d, dword ptr [rsi + 4]
140912d6a: xorps xmm1, xmm1
140912d6d: cvtsi2ss xmm1, r10
140912d72: mov r10, qword ptr [rax + 0x30]
140912d76: mov qword ptr [rsp + 0x70], r10
140912d7b: mov r10d, dword ptr [rax + 0x3c]
140912d7f: mov dword ptr [rsp + 0x3c], r10d
140912d84: divss xmm0, xmm1
140912d88: mov r12d, dword ptr [r9 + 0x1c]
140912d8c: mov r10, qword ptr [r9 + 0x10]
140912d90: mov qword ptr [rsp + 0x48], r10
140912d95: mov dword ptr [r10], r12d
140912d98: mov rdi, qword ptr [r9]
140912d9b: movss dword ptr [rdi + 0x4d8], xmm0
140912da3: mov r9, qword ptr [r8]
140912da6: cmp dword ptr [r9 + 0x18], 0
140912dab: mov rbp, qword ptr [rcx]
140912dae: je 0x140912de8
140912db0: cmp dword ptr [rbp + 0x18], 3
140912db4: jb 0x140912dd5
140912db6: movsd xmm0, qword ptr [rbp + 0x160]
140912dbe: mulps xmm0, xmm0
140912dc1: movshdup xmm1, xmm0
140912dc5: addss xmm1, xmm0
140912dc9: xorps xmm0, xmm0
140912dcc: ucomiss xmm1, xmm0
140912dcf: jbe 0x140914076
140912dd5: mov ecx, dword ptr [r8 + 0x1c]
140912dd9: mov r8, qword ptr [r8 + 0x10]
140912ddd: mov dword ptr [r8], ecx
140912de0: mov dword ptr [r9 + 0x18], 0
140912de8: mov qword ptr [rsp + 0x1b0], r15
140912df0: mov qword ptr [rsp + 0x1b8], rbx
140912df8: mov qword ptr [rsp + 0x78], r11
140912dfd: cmp dword ptr [rbp + 0x10], 3
140912e01: mov dword ptr [rsp + 0x38], r12d
140912e06: jb 0x140912e0e
140912e08: cmp dword ptr [rbp + 0x18], 3
140912e0c: jae 0x140912e66
140912e0e: mov r12, rdi
140912e11: mov rdi, rbp
140912e14: mov ecx, dword ptr [rax + 0x1c]
140912e17: mov r8, qword ptr [rax + 0x10]
140912e1b: mov dword ptr [r8], ecx
140912e1e: mov r15, qword ptr [rax]
140912e21: mov r13, qword ptr [r15 + 0x30]
140912e25: mov rbp, qword ptr [r15 + 0x40]
140912e29: cmp r13, qword ptr [r15 + 0x20]
140912e2d: jne 0x140912e48
140912e2f: lea rcx, [r15 + 0x20]
140912e33: lea rax, [rip + 0x232730e]
140912e3a: mov rbx, rdx
140912e3d: mov rdx, rax
140912e40: call 0x140562870
140912e45: mov rdx, rbx
140912e48: mov rax, qword ptr [r15 + 0x28]
140912e4c: mov qword ptr [rax + r13*8], rbp
140912e50: inc r13
140912e53: mov qword ptr [r15 + 0x30], r13
140912e57: inc qword ptr [r15 + 0x40]
140912e5b: mov rbp, rdi
140912e5e: mov rdi, r12
140912e61: mov r12d, dword ptr [rsp + 0x38]
140912e66: mov rax, qword ptr [rdx]
140912e69: movzx eax, byte ptr [rax]
140912e6c: mov rcx, qword ptr [rsp + 0x48]
140912e71: mov dword ptr [rcx], r12d
140912e74: mov byte ptr [rdi + 0x344], al
140912e7a: mov rax, qword ptr [rsp + 0x90]
140912e82: movzx eax, byte ptr [rax + 0x103]
140912e89: mov byte ptr [rsp + 0x37], al
140912e8d: mov eax, dword ptr [rbp + 0x10]
140912e90: cmp eax, 3
140912e93: mov ecx, 0x11d
140912e98: sbb rcx, 0
140912e9c: mov rdx, qword ptr [rsp + 0x58]
140912ea1: movzx edx, byte ptr [rdx + rcx]
140912ea5: xor ecx, ecx
140912ea7: cmp dl, 6
140912eaa: cmove edx, ecx
140912ead: movzx edx, dl
140912eb0: shl edx, 3
140912eb3: lea r8, [rip + 0x2198d0e]
140912eba: mov r8, qword ptr [rdx + r8]
140912ebe: lea r9, [rip + 0x2198d33]
140912ec5: mov r9, qword ptr [rdx + r9]
140912ec9: lea r10, [rip + 0x2198d58]
140912ed0: mov rdx, qword ptr [rdx + r10]
140912ed4: mov r10, qword ptr [rsp + 0x40]
140912ed9: movss xmm7, dword ptr [r10 + rdx]
140912edf: mov edx, dword ptr [r10 + r9]
140912ee3: mov dword ptr [rsp + 0x6c], edx
140912ee7: movss xmm8, dword ptr [r10 + r8]
140912eed: movss xmm9, dword ptr [rbp + 0x168]
140912ef6: mov r13b, 5
140912ef9: xorps xmm0, xmm0
140912efc: ucomiss xmm0, xmm9
140912f00: je 0x14091312e
140912f06: movss xmm1, dword ptr [r14]
140912f0b: movss xmm2, dword ptr [r14 + 4]
140912f11: mov eax, dword ptr [rsi]
140912f13: xorps xmm0, xmm0
140912f16: cvtsi2ss xmm0, rax
140912f1b: mov eax, dword ptr [rsi + 4]
140912f1e: cvtsi2ss xmm3, rax
140912f23: movss dword ptr [rsp + 0x28], xmm3
140912f29: movss dword ptr [rsp + 0x20], xmm0
140912f2f: lea r15, [rsp + 0x240]
140912f37: mov rcx, r15
140912f3a: mov r9, rdi
140912f3d: call 0x14127cab0
140912f42: movss xmm2, dword ptr [r14]
140912f47: movss xmm10, dword ptr [rip + 0x20129b4]
140912f50: movaps xmm0, xmm10
140912f54: subss xmm0, dword ptr [r14 + 4]
140912f5a: mov eax, dword ptr [rsi]
140912f5c: mov ecx, dword ptr [rsi + 4]
140912f5f: xorps xmm1, xmm1
140912f62: cvtsi2ss xmm1, rax
140912f67: divss xmm2, xmm1
140912f6b: xorps xmm1, xmm1
140912f6e: cvtsi2ss xmm1, rcx
140912f73: divss xmm0, xmm1
140912f77: addss xmm2, xmm2
140912f7b: addss xmm0, xmm0
140912f7f: addss xmm2, dword ptr [rip + 0x204715d]
140912f87: addss xmm0, xmm10
140912f8c: shufps xmm2, xmm2, 0
140912f90: mulps xmm2, xmmword ptr [rdi + 0x40]
140912f94: movaps xmm1, xmmword ptr [rdi + 0x60]
140912f98: shufps xmm0, xmm0, 0
140912f9c: mulps xmm0, xmmword ptr [rdi + 0x50]
140912fa0: movaps xmm3, xmmword ptr [rdi + 0x70]
140912fa4: addps xmm0, xmm2
140912fa7: movaps xmm2, xmm1
140912faa: addps xmm2, xmm0
140912fad: addps xmm2, xmm3
140912fb0: movaps xmm4, xmm2
140912fb3: shufps xmm4, xmm2, 0xff
140912fb7: divps xmm2, xmm4
140912fba: mulps xmm1, xmmword ptr [rip + 0x219851f]
140912fc1: addps xmm1, xmm0
140912fc4: addps xmm1, xmm3
140912fc7: movaps xmm0, xmm1
140912fca: shufps xmm0, xmm1, 0xff
140912fce: divps xmm1, xmm0
140912fd1: subps xmm1, xmm2
140912fd4: movaps xmm0, xmm1
140912fd7: unpckhpd xmm0, xmm1
140912fdb: movaps xmm2, xmm1
140912fde: mulps xmm2, xmm1
140912fe1: movshdup xmm3, xmm2
140912fe5: addss xmm3, xmm2
140912fe9: movaps xmm2, xmm0
140912fec: mulss xmm2, xmm0
140912ff0: addss xmm2, xmm3
140912ff4: sqrtss xmm2, xmm2
140912ff8: divss xmm10, xmm2
140912ffd: movsldup xmm11, xmm10
140913002: mulps xmm11, xmm1
140913006: mulss xmm10, xmm0
14091300b: movzx eax, r13b
14091300f: cmp al, 5
140913011: ja 0x14091302e
140913013: mov ecx, 0x2c
140913018: bt ecx, eax
14091301b: jae 0x14091302e
14091301d: movaps xmm11, xmmword ptr [rip + 0x20ba30b]
140913025: movss xmm10, dword ptr [rip + 0x20470b6]
14091302e: lea rcx, [rip + 0x2198cfb]
140913035: movsxd rax, dword ptr [rcx + rax*4]
140913039: add rax, rcx
14091303c: movaps xmm12, xmm8
140913040: mov esi, dword ptr [rsp + 0x6c]
140913044: movaps xmm13, xmm7
140913048: jmp rax
14091304a: lea rcx, [rsp + 0x1c0]
140913052: mov rdx, rdi
140913055: call 0x140c96d40
14091305a: lea r12, [rsp + 0xb0]
140913062: mov rcx, r12
140913065: mov rdx, rdi
140913068: call 0x140c96d80
14091306d: movss xmm12, dword ptr [rsp + 0xb8]
140913077: mulss xmm12, dword ptr [rip + 0x209b2c0]
140913080: movsd xmm0, qword ptr [rsp + 0xb0]
140913089: mulps xmm0, xmmword ptr [rip + 0x2179550]
140913090: movsd xmm13, qword ptr [rsp + 0x1c0]
14091309a: addps xmm13, xmm0
14091309e: addss xmm12, dword ptr [rsp + 0x1c8]
1409130a8: lea rcx, [rsp + 0x1c0]
1409130b0: mov rdx, rdi
1409130b3: call 0x140c96d80
1409130b8: movlps qword ptr [rsp + 0xb0], xmm13
1409130c1: movss dword ptr [rsp + 0xb8], xmm12
1409130cb: mov rax, qword ptr [rsp + 0x1c0]
1409130d3: mov qword ptr [rsp + 0xbc], rax
1409130db: mov eax, dword ptr [rsp + 0x1c8]
1409130e2: mov dword ptr [rsp + 0xc4], eax
1409130e9: mov rdx, qword ptr [rsp + 0x330]
1409130f1: cmp r13b, 4
1409130f5: cmovne r15, r12
1409130f9: lea rcx, [rsp + 0x1c0]
140913101: movss xmm3, dword ptr [rip + 0x2012877]
140913109: mov r8, r15
14091310c: call 0x1409e9a20
140913111: xor esi, esi
140913113: cmp byte ptr [rsp + 0x1c0], 0
14091311b: xorps xmm12, xmm12
14091311f: jne 0x140913162
140913121: xorps xmm13, xmm13
140913125: mov r12d, dword ptr [rsp + 0x38]
14091312a: jne 0x140913173
14091312c: jmp 0x14091317a
14091312e: mov rdx, qword ptr [rsp + 0x58]
140913133: mov edx, dword ptr [rdx + 0x11c]
140913139: cmp dl, 6
14091313c: cmovne ecx, edx
14091313f: mov r13d, ecx
140913142: mov ecx, eax
140913144: lea rdx, [rip + 0x2198bb9]
14091314b: movsxd rcx, dword ptr [rdx + rcx*4]
14091314f: add rcx, rdx
140913152: jmp rcx
140913154: cmp eax, 3
140913157: jb 0x14091326a
14091315d: jmp 0x1409135c3
140913162: movss xmm13, dword ptr [rsp + 0x1d0]
14091316c: mov r12d, dword ptr [rsp + 0x38]
140913171: je 0x14091317a
140913173: mov esi, dword ptr [rsp + 0x1cc]
14091317a: je 0x1409131c8
14091317c: movss xmm12, dword ptr [rsp + 0x1c8]
140913186: jmp 0x1409131c8
140913188: mov rax, qword ptr [rsp + 0x40]
14091318d: movss xmm12, dword ptr [rax + 0x14]
140913193: mov esi, dword ptr [rax + 0x18]
140913196: movss xmm13, dword ptr [rax + 0x1c]
14091319c: jmp 0x1409131c8
14091319e: mov rax, qword ptr [rsp + 0x40]
1409131a3: movss xmm12, dword ptr [rax + 0x2c]
1409131a9: mov esi, dword ptr [rax + 0x30]
1409131ac: movss xmm13, dword ptr [rax + 0x34]
1409131b2: jmp 0x1409131c8
1409131b4: mov rax, qword ptr [rsp + 0x40]
1409131b9: movss xmm12, dword ptr [rax + 0x20]
1409131bf: mov esi, dword ptr [rax + 0x24]
1409131c2: movss xmm13, dword ptr [rax + 0x28]
1409131c8: movaps xmm0, xmm12
1409131cc: mulss xmm0, xmm12
1409131d1: movaps xmm1, xmm13
1409131d5: mulss xmm1, xmm13
1409131da: addss xmm1, xmm0
1409131de: xorps xmm0, xmm0
1409131e1: sqrtss xmm0, xmm1
1409131e5: movss xmm1, dword ptr [rip + 0x2198303]
1409131ed: ucomiss xmm1, xmm0
1409131f0: jbe 0x14091324e
1409131f2: mov rax, qword ptr [rsp + 0x48]
1409131f7: mov dword ptr [rax], r12d
1409131fa: lea rcx, [rsp + 0xb0]
140913202: mov rdx, rdi
140913205: call 0x140c96d40
14091320a: movss xmm2, dword ptr [rsp + 0xb0]
140913213: movss xmm0, dword ptr [rsp + 0xb4]
14091321c: movss xmm1, dword ptr [rsp + 0xb8]
140913225: movzx eax, byte ptr [rdi + 0x4d0]
14091322c: lea rcx, [rip + 0x2198b15]
140913233: movsxd rax, dword ptr [rcx + rax*4]
140913237: add rax, rcx
14091323a: jmp rax
14091323c: xorps xmm0, xmm0
14091323f: ucomiss xmm9, xmm0
140913243: jne 0x140913467
140913249: jmp 0x1409134dc
14091324e: xorps xmm0, xmm0
140913251: ucomiss xmm9, xmm0
140913255: jne 0x140913467
14091325b: jmp 0x1409135c3
140913260: ucomiss xmm0, dword ptr [rbp + 0x14]
140913264: jae 0x140912f06
14091326a: mov rcx, qword ptr [rsp + 0x88]
140913272: mov rax, qword ptr [rcx]
140913275: mov r11, qword ptr [rcx + 8]
140913279: mov rcx, qword ptr [rax]
14091327c: xor esi, esi
14091327e: mov rdx, rcx
140913281: sub rdx, qword ptr [r11 + 0x18]
140913285: cmovb rdx, rsi
140913289: sub rcx, qword ptr [r11 + 0x38]
14091328d: cmovb rcx, rsi
140913291: mov r10, qword ptr [r11 + 8]
140913295: mov r8, qword ptr [r11 + 0x30]
140913299: mov qword ptr [rsp + 0x40], r8
14091329e: sub r8, rcx
1409132a1: cmovb r8, rsi
1409132a5: mov r15, qword ptr [r11 + 0x10]
1409132a9: setbe bl
1409132ac: mov r14, r15
1409132af: sub r14, rdx
1409132b2: cmovb r14, rsi
1409132b6: mov rsi, qword ptr [r11 + 0x28]
1409132ba: mov r11, qword ptr [r11 + 0x40]
1409132be: mov r9, rdi
1409132c1: mov rdi, rbp
1409132c4: setbe bpl
1409132c8: mov r13, r11
1409132cb: sub r13, r8
1409132ce: sub r13, r14
1409132d1: mov r12d, dword ptr [rsp + 0x38]
1409132d6: mov qword ptr [rax], r13
1409132d9: test bpl, bl
1409132dc: mov rbp, rdi
1409132df: mov rdi, r9
1409132e2: jne 0x1409135c3
1409132e8: mov rbx, rcx
1409132eb: shl rbx, 4
1409132ef: add rsi, rbx
1409132f2: shl r8, 4
1409132f6: mov rbx, rdx
1409132f9: shl rbx, 4
1409132fd: add r10, rbx
140913300: shl r14, 4
140913304: xor ebx, ebx
140913306: cmp r15, rdx
140913309: mov edx, 8
14091330e: cmovb r10, rdx
140913312: lea r10, [r10 + r14 - 0x10]
140913317: cmova rbx, r10
14091331b: cmp qword ptr [rsp + 0x40], rcx
140913320: cmovb rsi, rdx
140913324: lea rcx, [rsi + r8 - 0x10]
140913329: cmovbe rcx, rbx
14091332d: mov qword ptr [rax], r11
140913330: movss xmm1, dword ptr [rcx + 8]
140913335: movss xmm2, dword ptr [rcx + 0xc]
14091333a: mov r8, qword ptr [rsp + 0x58]
14091333f: movzx eax, byte ptr [r8 + 0x11e]
140913347: movzx ecx, byte ptr [r8 + 0x11f]
14091334f: mov rdx, qword ptr [rsp + 0x48]
140913354: mov dword ptr [rdx], r12d
140913357: mulss xmm1, dword ptr [rip + 0x2179905]
14091335f: mulss xmm1, dword ptr [rip + 0x20125e9]
140913367: movss xmm0, dword ptr [r8 + 0x108]
140913370: mulss xmm1, xmm0
140913374: mulss xmm2, dword ptr [rip + 0x216f894]
14091337c: movd r8d, xmm1
140913381: mulss xmm2, xmm0
140913385: movd edx, xmm2
140913389: and r8d, 0x7fffffff
140913390: and edx, 0x7fffffff
140913396: cmp edx, 0x7f800000
14091339c: setl dl
14091339f: cmp r8d, 0x7f7fffff
1409133a6: jg 0x1409135c3
1409133ac: test dl, dl
1409133ae: je 0x1409135c3
1409133b4: movaps xmm0, xmmword ptr [rip + 0x2012535]
1409133bb: test cl, cl
1409133bd: je 0x1409133c2
1409133bf: xorps xmm2, xmm0
1409133c2: test al, al
1409133c4: je 0x1409133c9
1409133c6: xorps xmm1, xmm0
1409133c9: movzx ecx, byte ptr [rdi + 0x4d0]
1409133d0: lea rax, [rip + 0x237e8c1]
1409133d7: lea rdx, [rip + 0x219893a]
1409133de: movsxd rcx, dword ptr [rdx + rcx*4]
1409133e2: add rcx, rdx
1409133e5: jmp rcx
1409133e7: lea rcx, [rdi + 0x100]
1409133ee: call qword ptr [rax + 0x18]
1409133f1: jmp 0x1409135c3
1409133f6: lea rax, [rdi + 0x100]
1409133fd: jmp 0x14091340f
1409133ff: lea rax, [rdi + 0x390]
140913406: jmp 0x14091340f
140913408: lea rax, [rdi + 0x1b8]
14091340f: subss xmm2, xmm12
140913414: movd xmm3, esi
140913418: subss xmm0, xmm3
14091341c: subss xmm1, xmm13
140913421: mulss xmm2, xmm2
140913425: mulss xmm0, xmm0
140913429: addss xmm0, xmm2
14091342d: mulss xmm1, xmm1
140913431: addss xmm1, xmm0
140913435: xorps xmm0, xmm0
140913438: sqrtss xmm0, xmm1
14091343c: mov dword ptr [rax + 0x18], 1
140913443: movss dword ptr [rax + 0x1c], xmm0
140913448: movss dword ptr [rax + 0x20], xmm0
14091344d: xorps xmm0, xmm0
140913450: movlhps xmm0, xmm11
140913454: movups xmmword ptr [rax + 0x24], xmm0
140913458: movss dword ptr [rax + 0x34], xmm10
14091345e: xorps xmm0, xmm0
140913461: ucomiss xmm9, xmm0
140913465: je 0x1409134dc
140913467: cmp byte ptr [rdi + 0x4d0], 0
14091346e: jne 0x1409135c3
140913474: mov rax, qword ptr [rsp + 0x48]
140913479: mov dword ptr [rax], r12d
14091347c: movss xmm1, dword ptr [rbp + 0x168]
140913484: mulss xmm1, dword ptr [rip + 0x20124c4]
14091348c: mov rax, qword ptr [rsp + 0x58]
140913491: mulss xmm1, dword ptr [rax + 0x108]
140913499: movd eax, xmm1
14091349d: and eax, 0x7fffffff
1409134a2: cmp eax, 0x7f7fffff
1409134a7: mov rbx, qword ptr [rsp + 0x78]
1409134ac: jg 0x140913578
1409134b2: movzx ecx, byte ptr [rdi + 0x4d0]
1409134b9: lea rax, [rip + 0x237e7d8]
1409134c0: lea rdx, [rip + 0x2198899]
1409134c7: movsxd rcx, dword ptr [rdx + rcx*4]
1409134cb: add rcx, rdx
1409134ce: jmp rcx
1409134d0: lea rcx, [rdi + 0x100]
1409134d7: jmp 0x140913572
1409134dc: mov rdx, qword ptr [rsp + 0x80]
1409134e4: mov eax, dword ptr [rdx + 0x1c]
1409134e7: mov rcx, qword ptr [rdx + 0x10]
1409134eb: mov dword ptr [rcx], eax
1409134ed: mov rsi, qword ptr [rdx]
1409134f0: mov r14, qword ptr [rsi + 0x30]
1409134f4: mov r15, qword ptr [rsi + 0x40]
1409134f8: cmp r14, qword ptr [rsi + 0x20]
1409134fc: jne 0x14091350e
1409134fe: lea rcx, [rsi + 0x20]
140913502: lea rdx, [rip + 0x2326c3f]
140913509: call 0x140562630
14091350e: mov rax, qword ptr [rsi + 0x28]
140913512: mov rcx, r14
140913515: shl rcx, 4
140913519: mov qword ptr [rax + rcx], r15
14091351d: mov byte ptr [rax + rcx + 8], 0
140913522: inc r14
140913525: mov qword ptr [rsi + 0x30], r14
140913529: inc qword ptr [rsi + 0x40]
14091352d: jmp 0x1409135c3
140913532: lea rcx, [rdi + 0x390]
140913539: jmp 0x140913572
14091353b: lea rcx, [rdi + 0x270]
140913542: lea rax, [rip + 0x237e6df]
140913549: jmp 0x140913572
14091354b: lea rcx, [rdi + 0x2f0]
140913552: lea rax, [rip + 0x237e697]
140913559: jmp 0x140913572
14091355b: lea rcx, [rdi + 0x1b8]
140913562: jmp 0x140913572
140913564: lea rcx, [rdi + 0x448]
14091356b: lea rax, [rip + 0x237e6ee]
140913572: xorps xmm2, xmm2
140913575: call qword ptr [rax + 0x18]
140913578: mov rax, qword ptr [rsp + 0x70]
14091357d: mov ecx, dword ptr [rsp + 0x3c]
140913581: mov dword ptr [rax], ecx
140913583: mov rsi, qword ptr [rbx + 0x30]
140913587: mov r14, qword ptr [rbx + 0x40]
14091358b: cmp rsi, qword ptr [rbx + 0x20]
14091358f: jne 0x1409135a1
140913591: lea rcx, [rbx + 0x20]
140913595: lea rdx, [rip + 0x2326bac]
14091359c: call 0x140562630
1409135a1: mov rax, qword ptr [rbx + 0x28]
1409135a5: mov rcx, rsi
1409135a8: shl rcx, 4
1409135ac: mov dword ptr [rax + rcx], 2
1409135b3: mov qword ptr [rax + rcx + 8], r14
1409135b8: inc rsi
1409135bb: mov qword ptr [rbx + 0x30], rsi
1409135bf: inc qword ptr [rbx + 0x40]
1409135c3: cmp dword ptr [rbp + 0x10], 3
1409135c7: jne 0x140913635
1409135c9: mov rdx, qword ptr [rsp + 0x80]
1409135d1: mov eax, dword ptr [rdx + 0x1c]
1409135d4: mov rcx, qword ptr [rdx + 0x10]
1409135d8: mov dword ptr [rcx], eax
1409135da: mov rsi, qword ptr [rdx]
1409135dd: mov r14, qword ptr [rsi + 0x30]
1409135e1: mov r15, qword ptr [rsi + 0x40]
1409135e5: cmp r14, qword ptr [rsi + 0x20]
1409135e9: jne 0x1409135fb
1409135eb: lea rcx, [rsi + 0x20]
1409135ef: lea rdx, [rip + 0x2326b52]
1409135f6: call 0x140562630
1409135fb: mov rax, qword ptr [rsi + 0x28]
1409135ff: mov rcx, r14
140913602: shl rcx, 4
140913606: mov qword ptr [rax + rcx], r15
14091360a: mov byte ptr [rax + rcx + 8], 1
14091360f: inc r14
140913612: mov qword ptr [rsi + 0x30], r14
140913616: inc qword ptr [rsi + 0x40]
14091361a: mov rax, qword ptr [rsp + 0x48]
14091361f: mov dword ptr [rax], r12d
140913622: movzx eax, byte ptr [rdi + 0x4d0]
140913629: lea ecx, [rax - 2]
14091362c: cmp ecx, 4
14091362f: jae 0x1409137ce
140913635: xorps xmm10, xmm10
140913639: xorps xmm9, xmm9
14091363d: cmp dword ptr [rbp + 0x18], 2
140913641: ja 0x14091387a
140913647: mov rcx, qword ptr [rsp + 0x88]
14091364f: mov rax, qword ptr [rcx]
140913652: mov r11, qword ptr [rcx + 8]
140913656: mov rcx, qword ptr [rax]
140913659: xor esi, esi
14091365b: mov rdx, rcx
14091365e: sub rdx, qword ptr [r11 + 0x18]
140913662: cmovb rdx, rsi
140913666: sub rcx, qword ptr [r11 + 0x38]
14091366a: cmovb rcx, rsi
14091366e: mov r10, qword ptr [r11 + 8]
140913672: mov r8, qword ptr [r11 + 0x30]
140913676: mov qword ptr [rsp + 0x40], r8
14091367b: sub r8, rcx
14091367e: cmovb r8, rsi
140913682: mov r15, qword ptr [r11 + 0x10]
140913686: seta bl
140913689: mov r14, r15
14091368c: sub r14, rdx
14091368f: cmovb r14, rsi
140913693: mov rsi, qword ptr [r11 + 0x28]
140913697: mov r11, qword ptr [r11 + 0x40]
14091369b: mov r9, rdi
14091369e: mov rdi, rbp
1409136a1: seta bpl
1409136a5: mov r13, r11
1409136a8: sub r13, r8
1409136ab: sub r13, r14
1409136ae: mov r12d, dword ptr [rsp + 0x38]
1409136b3: mov qword ptr [rax], r13
1409136b6: or bpl, bl
1409136b9: mov rbp, rdi
1409136bc: mov rdi, r9
1409136bf: mov r13, qword ptr [rsp + 0x78]
1409136c4: je 0x14091378d
1409136ca: mov rbx, rcx
1409136cd: shl rbx, 4
1409136d1: add rsi, rbx
1409136d4: shl r8, 4
1409136d8: mov rbx, rdx
1409136db: shl rbx, 4
1409136df: add r10, rbx
1409136e2: shl r14, 4
1409136e6: xor ebx, ebx
1409136e8: cmp r15, rdx
1409136eb: mov edx, 8
1409136f0: cmovb r10, rdx
1409136f4: lea r10, [r10 + r14 - 0x10]
1409136f9: cmova rbx, r10
1409136fd: cmp qword ptr [rsp + 0x40], rcx
140913702: cmovb rsi, rdx
140913706: lea rdx, [rsi + r8 - 0x10]
14091370b: cmovbe rdx, rbx
14091370f: add rdx, 8
140913713: mov qword ptr [rax], r11
140913716: lea rcx, [rsp + 0xb0]
14091371e: call 0x140c9cdc0
140913723: movss xmm0, dword ptr [rsp + 0xb0]
14091372c: mulss xmm0, dword ptr [rip + 0x2178ebc]
140913734: movsd xmm2, qword ptr [rsp + 0xb4]
14091373d: mulps xmm2, xmmword ptr [rip + 0x2197dbc]
140913744: mov rax, qword ptr [rsp + 0x58]
140913749: movss xmm1, dword ptr [rax + 0x10c]
140913751: mulss xmm0, xmm1
140913755: movsldup xmm1, xmm1
140913759: mulps xmm1, xmm2
14091375c: movzx eax, byte ptr [rdi + 0x4d0]
140913763: cmp eax, 4
140913766: je 0x14091376d
140913768: cmp eax, 1
14091376b: jne 0x14091377c
14091376d: mulss xmm0, dword ptr [rip + 0x20120fb]
140913775: mulps xmm1, xmmword ptr [rip + 0x2012164]
14091377c: xorps xmm9, xmm9
140913780: addss xmm9, xmm0
140913785: xorps xmm10, xmm10
140913789: addps xmm10, xmm1
14091378d: movzx eax, byte ptr [rdi + 0x4d0]
140913794: cmp eax, 1
140913797: mov r15, qword ptr [rsp + 0x48]
14091379c: je 0x140913894
1409137a2: cmp eax, 2
1409137a5: jne 0x1409137b5
1409137a7: movss xmm11, dword ptr [rip + 0x201af40]
1409137b0: jmp 0x14091389d
1409137b5: xor ecx, ecx
1409137b7: cmp al, 4
1409137b9: sete cl
1409137bc: lea rax, [rip + 0x2197d4d]
1409137c3: movss xmm11, dword ptr [rax + rcx*4]
1409137c9: jmp 0x14091389d
1409137ce: test eax, eax
1409137d0: jne 0x1409137ed
1409137d2: lea r14, [rdi + 0x100]
1409137d9: test byte ptr [r14 + 0x18], 1
1409137de: mov dword ptr [r14 + 0x18], 0
1409137e6: jne 0x140913807
1409137e8: jmp 0x140913635
1409137ed: lea r14, [rdi + 0x1b8]
1409137f4: test byte ptr [r14 + 0x18], 1
1409137f9: mov dword ptr [r14 + 0x18], 0
140913801: je 0x140913635
140913807: movups xmm0, xmmword ptr [r14 + 0x1c]
14091380c: movups xmm1, xmmword ptr [r14 + 0x28]
140913811: movups xmmword ptr [rsp + 0xbc], xmm1
140913819: movaps xmmword ptr [rsp + 0xb0], xmm0
140913821: mov rsi, qword ptr [r14 + 0x10]
140913825: cmp rsi, qword ptr [r14]
140913828: jne 0x140913839
14091382a: lea rdx, [rip + 0x2197f17]
140913831: mov rcx, r14
140913834: call 0x14078cb80
140913839: mov rax, qword ptr [r14 + 8]
14091383d: lea rcx, [rsi + rsi*8]
140913841: lea rcx, [rcx + rcx*2]
140913845: add rcx, rsi
140913848: movups xmm0, xmmword ptr [rsp + 0xbc]
140913850: movups xmmword ptr [rax + rcx + 0xc], xmm0
140913855: movaps xmm0, xmmword ptr [rsp + 0xb0]
14091385d: movups xmmword ptr [rax + rcx], xmm0
140913861: inc rsi
140913864: mov qword ptr [r14 + 0x10], rsi
140913868: xorps xmm10, xmm10
14091386c: xorps xmm9, xmm9
140913870: cmp dword ptr [rbp + 0x18], 2
140913874: jbe 0x140913647
14091387a: mov r13, qword ptr [rsp + 0x78]
14091387f: movzx eax, byte ptr [rdi + 0x4d0]
140913886: cmp eax, 1
140913889: mov r15, qword ptr [rsp + 0x48]
14091388e: jne 0x1409137a2
140913894: movss xmm11, dword ptr [rip + 0x20120b3]
14091389d: lea rdx, [rbp + 0x150]
1409138a4: lea rcx, [rsp + 0xb0]
1409138ac: call 0x140c9cdc0
1409138b1: movss xmm0, dword ptr [rsp + 0xb4]
1409138ba: mulss xmm0, xmm11
1409138bf: movss xmm1, dword ptr [rsp + 0xb8]
1409138c8: mulss xmm1, xmm11
1409138cd: movss xmm2, dword ptr [rsp + 0xb0]
1409138d6: mov rax, qword ptr [rsp + 0x58]
1409138db: movss xmm3, dword ptr [rax + 0x10c]
1409138e3: mulss xmm0, xmm3
1409138e7: mulss xmm1, xmm3
1409138eb: addss xmm0, xmm10
1409138f0: unpcklps xmm9, xmm0
1409138f4: movshdup xmm0, xmm10
1409138f9: addss xmm0, xmm1
1409138fd: unpcklps xmm11, xmm3
140913901: movss xmm1, dword ptr [rbp + 0x158]
140913909: unpcklps xmm2, xmm1
14091390c: mulps xmm2, xmm11
140913910: shufps xmm11, xmm11, 0xe1
140913915: mulps xmm11, xmm2
140913919: addps xmm11, xmm9
14091391d: movshdup xmm1, xmm11
140913922: xorps xmm2, xmm2
140913925: ucomiss xmm11, xmm2
140913929: jne 0x14091393f
14091392b: jp 0x14091393f
14091392d: ucomiss xmm1, xmm2
140913930: jne 0x14091393f
140913932: jp 0x14091393f
140913934: ucomiss xmm0, xmm2
140913937: jne 0x14091393f
140913939: jnp 0x140913a6d
14091393f: movd edx, xmm11
140913944: and edx, 0x7fffffff
14091394a: movd eax, xmm1
14091394e: and eax, 0x7fffffff
140913953: cmp eax, 0x7f800000
140913958: setl al
14091395b: movd ecx, xmm0
14091395f: and ecx, 0x7fffffff
140913965: cmp ecx, 0x7f800000
14091396b: setl cl
14091396e: cmp edx, 0x7f7fffff
140913974: jg 0x140913a6d
14091397a: test al, al
14091397c: je 0x140913a6d
140913982: test cl, cl
140913984: je 0x140913a6d
14091398a: mov dword ptr [r15], r12d
14091398d: movlpd qword ptr [rsp + 0xb0], xmm11
140913997: movss dword ptr [rsp + 0xb8], xmm0
1409139a0: movzx ecx, byte ptr [rdi + 0x4d0]
1409139a7: lea rax, [rip + 0x237e2ea]
1409139ae: lea rdx, [rip + 0x21983c3]
1409139b5: movsxd rcx, dword ptr [rdx + rcx*4]
1409139b9: add rcx, rdx
1409139bc: jmp rcx
1409139be: lea rcx, [rdi + 0x100]
1409139c5: jmp 0x140913a07
1409139c7: lea rcx, [rdi + 0x390]
1409139ce: jmp 0x140913a07
1409139d0: lea rcx, [rdi + 0x270]
1409139d7: lea rax, [rip + 0x237e24a]
1409139de: jmp 0x140913a07
1409139e0: lea rcx, [rdi + 0x2f0]
1409139e7: lea rax, [rip + 0x237e202]
1409139ee: jmp 0x140913a07
1409139f0: lea rcx, [rdi + 0x1b8]
1409139f7: jmp 0x140913a07
1409139f9: lea rcx, [rdi + 0x448]
140913a00: lea rax, [rip + 0x237e259]
140913a07: mov qword ptr [rsp + 0x20], rdi
140913a0c: lea rdx, [rsp + 0xb0]
140913a14: lea r9, [rsp + 0x310]
140913a1c: movaps xmm2, xmm6
140913a1f: call qword ptr [rax + 0x28]
140913a22: mov rax, qword ptr [rsp + 0x70]
140913a27: mov ecx, dword ptr [rsp + 0x3c]
140913a2b: mov dword ptr [rax], ecx
140913a2d: mov rsi, qword ptr [r13 + 0x30]
140913a31: mov r14, qword ptr [r13 + 0x40]
140913a35: cmp rsi, qword ptr [r13 + 0x20]
140913a39: jne 0x140913a4b
140913a3b: lea rcx, [r13 + 0x20]
140913a3f: lea rdx, [rip + 0x2326702]
140913a46: call 0x140562630
140913a4b: mov rax, qword ptr [r13 + 0x28]
140913a4f: mov rcx, rsi
140913a52: shl rcx, 4
140913a56: mov dword ptr [rax + rcx], 0
140913a5d: mov qword ptr [rax + rcx + 8], r14
140913a62: inc rsi
140913a65: mov qword ptr [r13 + 0x30], rsi
140913a69: inc qword ptr [r13 + 0x40]
140913a6d: movss xmm1, dword ptr [rbp + 0x160]
140913a75: xorps xmm0, xmm0
140913a78: ucomiss xmm0, xmm1
140913a7b: je 0x140913ada
140913a7d: movzx eax, byte ptr [rdi + 0x4d0]
140913a84: cmp eax, 2
140913a87: jb 0x140913b32
140913a8d: cmp eax, 4
140913a90: je 0x140913b32
140913a96: cmp eax, 5
140913a99: jne 0x140913ada
140913a9b: andps xmm1, xmmword ptr [rip + 0x2011e4e]
140913aa2: orps xmm1, xmmword ptr [rip + 0x2011e87]
140913aa9: cvttss2si eax, xmm1
140913aad: ucomiss xmm1, dword ptr [rip + 0x2090f08]
140913ab4: mov ecx, 0x7fffffff
140913ab9: cmovbe ecx, eax
140913abc: mov eax, dword ptr [rdi + 0x484]
140913ac2: xor edx, edx
140913ac4: ucomiss xmm1, xmm1
140913ac7: cmovp ecx, edx
140913aca: sub eax, ecx
140913acc: test eax, eax
140913ace: cmovle eax, edx
140913ad1: mov dword ptr [r15], r12d
140913ad4: mov dword ptr [rdi + 0x484], eax
140913ada: cmp byte ptr [rsp + 0x37], 5
140913adf: jne 0x140913bd4
140913ae5: mov rdx, qword ptr [rsp + 0x60]
140913aea: mov rax, qword ptr [rdx + 0xf8]
140913af1: mov rcx, qword ptr [rdx + 0x100]
140913af8: lea rcx, [rax + rcx*4]
140913afc: movzx r8d, byte ptr [rdx + 0x108]
140913b04: mov edx, dword ptr [rdi + 0x484]
140913b0a: test rdx, rdx
140913b0d: je 0x140913c35
140913b13: test r8b, r8b
140913b16: je 0x1409140d0
140913b1c: xor r9d, r9d
140913b1f: xor r10d, r10d
140913b22: xor r11d, r11d
140913b25: xor r8d, r8d
140913b28: mov rsi, qword ptr [rsp + 0x50]
140913b2d: jmp 0x1409141af
140913b32: movaps xmm9, xmmword ptr [rip + 0x201ae46]
140913b3a: andps xmm9, xmm1
140913b3e: mulss xmm1, dword ptr [rip + 0x20465a2]
140913b46: mov dword ptr [r15], r12d
140913b49: movss dword ptr [rsp + 0xb0], xmm8
140913b53: mov eax, dword ptr [rsp + 0x6c]
140913b57: mov dword ptr [rsp + 0xb4], eax
140913b5e: movss dword ptr [rsp + 0xb8], xmm7
140913b67: lea r8, [rsp + 0xb0]
140913b6f: mov rcx, rdi
140913b72: call 0x140911280
140913b77: mov rax, qword ptr [rsp + 0x70]
140913b7c: mov ecx, dword ptr [rsp + 0x3c]
140913b80: mov dword ptr [rax], ecx
140913b82: mov rsi, qword ptr [r13 + 0x30]
140913b86: mov r14, qword ptr [r13 + 0x40]
140913b8a: cmp rsi, qword ptr [r13 + 0x20]
140913b8e: jne 0x140913ba0
140913b90: lea rcx, [r13 + 0x20]
140913b94: lea rdx, [rip + 0x23265ad]
140913b9b: call 0x140562630
140913ba0: mov rax, qword ptr [r13 + 0x28]
140913ba4: mov rcx, rsi
140913ba7: shl rcx, 4
140913bab: mov dword ptr [rax + rcx], 1
140913bb2: movss dword ptr [rax + rcx + 4], xmm9
140913bb9: mov qword ptr [rax + rcx + 8], r14
140913bbe: inc rsi
140913bc1: mov qword ptr [r13 + 0x30], rsi
140913bc5: inc qword ptr [r13 + 0x40]
140913bc9: cmp byte ptr [rsp + 0x37], 5
140913bce: je 0x140913ae5
140913bd4: mov dword ptr [r15], r12d
140913bd7: mov rax, qword ptr [rsp + 0x90]
140913bdf: movzx eax, byte ptr [rax + 0x103]
140913be6: mov byte ptr [rdi + 0x4d0], al
140913bec: lea rcx, [rsp + 0x98]
140913bf4: mov rdx, rdi
140913bf7: call 0x140c96d40
140913bfc: mov dword ptr [r15], r12d
140913bff: lea rbx, [rdi + 0x100]
140913c06: movzx eax, byte ptr [rdi + 0x4d0]
140913c0d: lea rsi, [rip + 0x237e084]
140913c14: lea rcx, [rip + 0x2198175]
140913c1b: movsxd rax, dword ptr [rcx + rax*4]
140913c1f: add rax, rcx
140913c22: jmp rax
140913c24: mov r14d, 0x8c
140913c2a: mov rdx, rbx
140913c2d: mov rax, rsi
140913c30: jmp 0x140913d23
140913c35: test r8b, r8b
140913c38: je 0x14091423a
140913c3e: mov r9, qword ptr [rsp + 0x50]
140913c43: nop word ptr cs:[rax + rax]
140913c50: cmp rax, rcx
140913c53: je 0x140913bec
140913c55: mov r8d, dword ptr [rax]
140913c58: add rax, 4
140913c5c: mov rdx, qword ptr [r9 + 0x1a8]
140913c63: lea r8, [r8 + r8*8]
140913c67: cmp qword ptr [rdx + r8*8 + 0x10], 0
140913c6d: je 0x140913c50
140913c6f: lea rax, [rdx + r8*8]
140913c73: mov rcx, qword ptr [rsp + 0x60]
140913c78: mov rcx, qword ptr [rcx + 0x110]
140913c7f: xor r9d, r9d
140913c82: mov r11d, 0
140913c88: cmp qword ptr [rax + 0x40], rcx
140913c8c: jbe 0x1409142e5
140913c92: mov rdx, qword ptr [rax + 0x38]
140913c96: mov rcx, qword ptr [rdx + rcx*8]
140913c9a: test rcx, rcx
140913c9d: je 0x1409142df
140913ca3: mov rax, qword ptr [rax + 0x18]
140913ca7: not rcx
140913caa: lea rcx, [rcx + rcx*2]
140913cae: shl rcx, 4
140913cb2: mov r11, qword ptr [rax + rcx + 0x10]
140913cb7: xor r9d, r9d
140913cba: jmp 0x1409142e5
140913cbf: lea rdx, [rdi + 0x390]
140913cc6: mov r14d, 0x31c
140913ccc: mov rax, rsi
140913ccf: jmp 0x140913d23
140913cd1: lea rdx, [rdi + 0x270]
140913cd8: lea rax, [rip + 0x237df49]
140913cdf: mov r14d, 0x1c4
140913ce5: jmp 0x140913d23
140913ce7: lea rdx, [rdi + 0x2f0]
140913cee: lea rax, [rip + 0x237defb]
140913cf5: mov r14d, 0x264
140913cfb: jmp 0x140913d23
140913cfd: lea rdx, [rdi + 0x1b8]
140913d04: mov r14d, 0x144
140913d0a: mov rax, rsi
140913d0d: jmp 0x140913d23
140913d0f: lea rdx, [rdi + 0x448]
140913d16: lea rax, [rip + 0x237df43]
140913d1d: mov r14d, 0x3a4
140913d23: lea rcx, [rsp + 0xb0]
140913d2b: movaps xmm2, xmm6
140913d2e: call qword ptr [rax + 0x30]
140913d31: mov eax, dword ptr [rsp + 0xb8]
140913d38: mov dword ptr [rsp + 0x2c8], eax
140913d3f: mov rax, qword ptr [rsp + 0xb0]
140913d47: mov qword ptr [rsp + 0x2c0], rax
140913d4f: movaps xmm7, xmmword ptr [rsp + 0xc0]
140913d57: movaps xmm0, xmm6
140913d5a: divss xmm0, dword ptr [rip + 0x20cb40a]
140913d62: call 0x142923ec0
140913d67: movss xmm1, dword ptr [rip + 0x2011b95]
140913d6f: movaps xmm2, xmm1
140913d72: subss xmm2, xmm0
140913d76: movsd xmm0, qword ptr [rdi + r14 + 0x104]
140913d80: subss xmm1, xmm2
140913d84: unpcklps xmm1, xmm2
140913d87: mulps xmm1, xmm0
140913d8a: movshdup xmm0, xmm1
140913d8e: addss xmm0, xmm1
140913d92: movss dword ptr [rdi + r14 + 0x104], xmm0
140913d9c: movaps xmmword ptr [rsp + 0x2d0], xmm7
140913da4: movss xmm0, dword ptr [rdi + 0x4d8]
140913dac: movups xmm1, xmmword ptr [rbx + r14]
140913db1: movups xmm2, xmmword ptr [rbx + r14 + 0x10]
140913db7: mov rax, qword ptr [rbx + r14 + 0x20]
140913dbc: mov ecx, dword ptr [rbx + r14 + 0x28]
140913dc1: mov dword ptr [rsp + 0x30c], ecx
140913dc8: mov qword ptr [rsp + 0x304], rax
140913dd0: movups xmmword ptr [rsp + 0x2f4], xmm2
140913dd8: movups xmmword ptr [rsp + 0x2e4], xmm1
140913de0: movss dword ptr [rsp + 0x2e0], xmm0
140913de9: lea rcx, [rsp + 0x1c0]
140913df1: lea rdx, [rsp + 0x2c0]
140913df9: call 0x140c8ff70
140913dfe: lea rcx, [rsp + 0x240]
140913e06: lea rdx, [rsp + 0x2e0]
140913e0e: call 0x140c97390
140913e13: movaps xmm0, xmmword ptr [rsp + 0x240]
140913e1b: movaps xmm1, xmmword ptr [rsp + 0x250]
140913e23: movaps xmm2, xmmword ptr [rsp + 0x260]
140913e2b: movaps xmm3, xmmword ptr [rsp + 0x270]
140913e33: movaps xmmword ptr [rsp + 0xe0], xmm3
140913e3b: movaps xmmword ptr [rsp + 0xd0], xmm2
140913e43: movaps xmmword ptr [rsp + 0xc0], xmm1
140913e4b: movaps xmmword ptr [rsp + 0xb0], xmm0
140913e53: movaps xmm0, xmmword ptr [rsp + 0x280]
140913e5b: movaps xmmword ptr [rsp + 0xf0], xmm0
140913e63: movaps xmm0, xmmword ptr [rsp + 0x290]
140913e6b: movaps xmmword ptr [rsp + 0x100], xmm0
140913e73: movaps xmm0, xmmword ptr [rsp + 0x2a0]
140913e7b: movaps xmmword ptr [rsp + 0x110], xmm0
140913e83: movaps xmm0, xmmword ptr [rsp + 0x2b0]
140913e8b: movaps xmmword ptr [rsp + 0x120], xmm0
140913e93: movaps xmm0, xmmword ptr [rsp + 0x1c0]
140913e9b: movaps xmm1, xmmword ptr [rsp + 0x1d0]
140913ea3: movaps xmm2, xmmword ptr [rsp + 0x1e0]
140913eab: movaps xmm3, xmmword ptr [rsp + 0x1f0]
140913eb3: movaps xmmword ptr [rsp + 0x130], xmm0
140913ebb: movaps xmmword ptr [rsp + 0x140], xmm1
140913ec3: movaps xmmword ptr [rsp + 0x150], xmm2
140913ecb: movaps xmmword ptr [rsp + 0x160], xmm3
140913ed3: movaps xmm0, xmmword ptr [rsp + 0x200]
140913edb: movaps xmm1, xmmword ptr [rsp + 0x210]
140913ee3: movaps xmm2, xmmword ptr [rsp + 0x220]
140913eeb: movaps xmm3, xmmword ptr [rsp + 0x230]
140913ef3: movaps xmmword ptr [rsp + 0x1a0], xmm3
140913efb: movaps xmmword ptr [rsp + 0x190], xmm2
140913f03: movaps xmmword ptr [rsp + 0x180], xmm1
140913f0b: movaps xmmword ptr [rsp + 0x170], xmm0
140913f13: lea rdx, [rsp + 0xb0]
140913f1b: mov r8d, 0x100
140913f21: mov rcx, rdi
140913f24: call 0x142923c00
140913f29: mov dword ptr [r15], r12d
140913f2c: mov rax, qword ptr [rsp + 0x1b0]
140913f34: movzx eax, byte ptr [rax + 0x58]
140913f38: movzx ecx, byte ptr [rdi + 0x4d0]
140913f3f: lea rdx, [rip + 0x2197e62]
140913f46: movsxd rcx, dword ptr [rdx + rcx*4]
140913f4a: add rcx, rdx
140913f4d: jmp rcx
140913f4f: lea rbx, [rdi + 0x1b8]
140913f56: jmp 0x140913f8f
140913f58: lea rbx, [rdi + 0x390]
140913f5f: jmp 0x140913f8f
140913f61: lea rbx, [rdi + 0x270]
140913f68: lea rsi, [rip + 0x237dcb9]
140913f6f: jmp 0x140913f8f
140913f71: lea rbx, [rdi + 0x2f0]
140913f78: lea rsi, [rip + 0x237dc71]
140913f7f: jmp 0x140913f8f
140913f81: lea rbx, [rdi + 0x448]
140913f88: lea rsi, [rip + 0x237dcd1]
140913f8f: cmp al, 4
140913f91: sete r9b
140913f95: lea rdx, [rsp + 0x310]
140913f9d: mov rcx, rbx
140913fa0: mov r8, qword ptr [rsp + 0x1b8]
140913fa8: call qword ptr [rsi + 0x20]
140913fab: lea rcx, [rsp + 0xa4]
140913fb3: mov rdx, rdi
140913fb6: call 0x140c96d40
140913fbb: cmp dword ptr [rbp + 0x10], 3
140913fbf: jae 0x140914076
140913fc5: movss xmm0, dword ptr [rsp + 0x9c]
140913fce: subss xmm0, dword ptr [rsp + 0xa8]
140913fd7: mulss xmm0, xmm0
140913fdb: movss xmm1, dword ptr [rsp + 0xa0]
140913fe4: movss xmm2, dword ptr [rsp + 0x98]
140913fed: unpcklps xmm2, xmm1
140913ff0: movss xmm1, dword ptr [rsp + 0xa4]
140913ff9: movss xmm3, dword ptr [rsp + 0xac]
140914002: unpcklps xmm1, xmm3
140914005: subps xmm2, xmm1
140914008: mulps xmm2, xmm2
14091400b: addss xmm0, xmm2
14091400f: movshdup xmm1, xmm2
140914013: addss xmm1, xmm0
140914017: xorps xmm0, xmm0
14091401a: sqrtss xmm0, xmm1
14091401e: mulss xmm6, dword ptr [rip + 0x20b9042]
140914026: ucomiss xmm0, xmm6
140914029: jbe 0x140914076
14091402b: mov rax, qword ptr [rsp + 0x70]
140914030: mov ecx, dword ptr [rsp + 0x3c]
140914034: mov dword ptr [rax], ecx
140914036: mov rsi, qword ptr [r13 + 0x30]
14091403a: mov rdi, qword ptr [r13 + 0x40]
14091403e: cmp rsi, qword ptr [r13 + 0x20]
140914042: jne 0x140914054
140914044: lea rcx, [r13 + 0x20]
140914048: lea rdx, [rip + 0x23260f9]
14091404f: call 0x140562630
140914054: mov rax, qword ptr [r13 + 0x28]
140914058: mov rcx, rsi
14091405b: shl rcx, 4
14091405f: mov dword ptr [rax + rcx], 2
140914066: mov qword ptr [rax + rcx + 8], rdi
14091406b: inc rsi
14091406e: mov qword ptr [r13 + 0x30], rsi
140914072: inc qword ptr [r13 + 0x40]
140914076: movaps xmm6, xmmword ptr [rsp + 0x350]
14091407e: movaps xmm7, xmmword ptr [rsp + 0x360]
140914086: movaps xmm8, xmmword ptr [rsp + 0x370]
14091408f: movaps xmm9, xmmword ptr [rsp + 0x380]
140914098: movaps xmm10, xmmword ptr [rsp + 0x390]
1409140a1: movaps xmm11, xmmword ptr [rsp + 0x3a0]
1409140aa: movaps xmm12, xmmword ptr [rsp + 0x3b0]
1409140b3: movaps xmm13, xmmword ptr [rsp + 0x3c0]
1409140bc: add rsp, 0x3d8
1409140c3: pop rbx
1409140c4: pop rbp
1409140c5: pop rdi
1409140c6: pop rsi
1409140c7: pop r12
1409140c9: pop r13
1409140cb: pop r14
1409140cd: pop r15
1409140cf: ret 
1409140d0: mov r11d, 8
1409140d6: xor esi, esi
1409140d8: xor r10d, r10d
1409140db: xor r9d, r9d
1409140de: xor r8d, r8d
1409140e1: jmp 0x1409140fb
1409140e3: xor r9d, r9d
1409140e6: mov r11, qword ptr [r11 + 0x50]
1409140ea: xor esi, esi
1409140ec: inc r8
1409140ef: inc rsi
1409140f2: cmp r8, rdx
1409140f5: je 0x140914231
1409140fb: cmp rsi, r10
1409140fe: jne 0x1409140ec
140914100: mov rsi, qword ptr [rsp + 0x50]
140914105: nop word ptr cs:[rax + rax]
140914110: cmp rax, rcx
140914113: je 0x140913bec
140914119: mov r9d, dword ptr [rax]
14091411c: add rax, 4
140914120: mov r11, qword ptr [rsi + 0x100]
140914127: lea r9, [r9 + r9*4]
14091412b: shl r9, 5
14091412f: mov r10, qword ptr [r11 + r9 + 0x58]
140914134: test r10, r10
140914137: je 0x140914110
140914139: add r11, r9
14091413c: mov r9d, dword ptr [r11 + 0x94]
140914143: mov rsi, qword ptr [rsi + 0x1a8]
14091414a: lea r14, [r9 + r9*8]
14091414e: mov r9, qword ptr [rsp + 0x60]
140914153: mov r9, qword ptr [r9 + 0x110]
14091415a: cmp qword ptr [rsi + r14*8 + 0x40], r9
14091415f: jbe 0x1409140e3
140914161: lea rsi, [rsi + r14*8]
140914165: mov rbx, qword ptr [rsi + 0x38]
140914169: mov r9, qword ptr [rbx + r9*8]
14091416d: test r9, r9
140914170: je 0x1409140e3
140914176: mov rsi, qword ptr [rsi + 0x18]
14091417a: not r9
14091417d: lea r9, [r9 + r9*2]
140914181: shl r9, 4
140914185: mov r9, qword ptr [rsi + r9 + 0x10]
14091418a: jmp 0x1409140e6
14091418f: xor r9d, r9d
140914192: xor r11d, r11d
140914195: nop word ptr cs:[rax + rax]
1409141a0: inc r8
1409141a3: inc r9
1409141a6: cmp r8, rdx
1409141a9: je 0x1409142d4
1409141af: cmp r9, r10
1409141b2: jne 0x1409141a0
1409141b4: nop word ptr cs:[rax + rax]
1409141c0: cmp rax, rcx
1409141c3: je 0x140913bec
1409141c9: mov r10d, dword ptr [rax]
1409141cc: add rax, 4
1409141d0: mov r9, qword ptr [rsi + 0x1a8]
1409141d7: lea r11, [r10 + r10*8]
1409141db: mov r10, qword ptr [r9 + r11*8 + 0x10]
1409141e0: test r10, r10
1409141e3: je 0x1409141c0
1409141e5: lea r9, [r9 + r11*8]
1409141e9: mov r11, qword ptr [rsp + 0x60]
1409141ee: mov r11, qword ptr [r11 + 0x110]
1409141f5: cmp qword ptr [r9 + 0x40], r11
1409141f9: jbe 0x14091418f
1409141fb: mov rsi, qword ptr [r9 + 0x38]
1409141ff: mov r11, qword ptr [rsi + r11*8]
140914203: test r11, r11
140914206: je 0x140914221
140914208: mov r9, qword ptr [r9 + 0x18]
14091420c: not r11
14091420f: lea r11, [r11 + r11*2]
140914213: shl r11, 4
140914217: mov r11, qword ptr [r9 + r11 + 0x10]
14091421c: xor r9d, r9d
14091421f: jmp 0x140914227
140914221: xor r9d, r9d
140914224: xor r11d, r11d
140914227: mov rsi, qword ptr [rsp + 0x50]
14091422c: jmp 0x1409141a0
140914231: cmp rsi, r10
140914234: jne 0x1409142be
14091423a: mov r9, qword ptr [rsp + 0x50]
14091423f: nop 
140914240: cmp rax, rcx
140914243: je 0x140913bec
140914249: mov r8d, dword ptr [rax]
14091424c: add rax, 4
140914250: mov rdx, qword ptr [r9 + 0x100]
140914257: lea r8, [r8 + r8*4]
14091425b: shl r8, 5
14091425f: cmp qword ptr [rdx + r8 + 0x58], 0
140914265: je 0x140914240
140914267: add rdx, r8
14091426a: mov eax, dword ptr [rdx + 0x94]
140914270: mov rcx, qword ptr [r9 + 0x1a8]
140914277: lea r8, [rax + rax*8]
14091427b: mov rax, qword ptr [rsp + 0x60]
140914280: mov rax, qword ptr [rax + 0x110]
140914287: cmp qword ptr [rcx + r8*8 + 0x40], rax
14091428c: jbe 0x1409142b5
14091428e: lea rcx, [rcx + r8*8]
140914292: mov r8, qword ptr [rcx + 0x38]
140914296: mov rax, qword ptr [r8 + rax*8]
14091429a: test rax, rax
14091429d: je 0x1409142b5
14091429f: mov rcx, qword ptr [rcx + 0x18]
1409142a3: not rax
1409142a6: lea rax, [rax + rax*2]
1409142aa: shl rax, 4
1409142ae: mov r9, qword ptr [rcx + rax + 0x10]
1409142b3: jmp 0x1409142b8
1409142b5: xor r9d, r9d
1409142b8: mov r11, qword ptr [rdx + 0x50]
1409142bc: xor esi, esi
1409142be: shl rsi, 4
1409142c2: mov eax, dword ptr [r11 + rsi + 8]
1409142c7: lea rax, [rax + rax*2]
1409142cb: shl rax, 4
1409142cf: add rax, r9
1409142d2: jmp 0x1409142f3
1409142d4: cmp r9, r10
1409142d7: je 0x140913c3e
1409142dd: jmp 0x1409142e5
1409142df: xor r9d, r9d
1409142e2: xor r11d, r11d
1409142e5: mov eax, r9d
1409142e8: lea rax, [rax + rax*2]
1409142ec: shl rax, 4
1409142f0: add rax, r11
1409142f3: mov dword ptr [r15], r12d
1409142f6: movsd xmm7, qword ptr [rax + 0x10]
1409142fb: movss xmm8, dword ptr [rax + 0x18]
140914301: movaps xmm9, xmmword ptr [rax]
140914305: mov byte ptr [rdi + 0x4d0], 5
14091430c: mov r8, qword ptr [rdi + 0x450]
140914313: mov rsi, qword ptr [rdi + 0x458]
14091431a: shl rsi, 4
14091431e: movabs r14, 0x6b8463577afc1a66
140914328: movabs r15, 0xe4b9f294168d7e3
140914332: nop word ptr cs:[rax + rax]
140914340: test rsi, rsi
140914343: je 0x1409144e5
140914349: lea r12, [r8 + 0x10]
14091434d: mov rcx, qword ptr [r8]
140914350: mov rax, qword ptr [r8 + 8]
140914354: call qword ptr [rax + 0x40]
140914357: mov rbx, rax
14091435a: mov rcx, rax
14091435d: call qword ptr [rdx + 0x18]
140914360: add rsi, -0x10
140914364: mov r8, r12
140914367: test rbx, rbx
14091436a: je 0x140914340
14091436c: mov r8, r12
14091436f: cmp rax, r14
140914372: jne 0x140914340
140914374: mov r8, r12
140914377: cmp rdx, r15
14091437a: jne 0x140914340
14091437c: movlps qword ptr [rbx], xmm7
14091437f: movss dword ptr [rbx + 8], xmm8
140914385: mov r8, qword ptr [rdi + 0x450]
14091438c: mov rsi, qword ptr [rdi + 0x458]
140914393: shl rsi, 4
140914397: movabs r14, 0x9f8cbdc83fdf50d6
1409143a1: movabs r15, 0xc69d4571dbc0c107
1409143ab: nop dword ptr [rax + rax]
1409143b0: test rsi, rsi
1409143b3: je 0x1409144ee
1409143b9: lea r12, [r8 + 0x10]
1409143bd: mov rcx, qword ptr [r8]
1409143c0: mov rax, qword ptr [r8 + 8]
1409143c4: call qword ptr [rax + 0x40]
1409143c7: mov rbx, rax
1409143ca: mov rcx, rax
1409143cd: call qword ptr [rdx + 0x18]
1409143d0: add rsi, -0x10
1409143d4: mov r8, r12
1409143d7: test rbx, rbx
1409143da: je 0x1409143b0
1409143dc: mov r8, r12
1409143df: cmp rax, r14
1409143e2: jne 0x1409143b0
1409143e4: mov r8, r12
1409143e7: cmp rdx, r15
1409143ea: jne 0x1409143b0
1409143ec: addps xmm7, xmmword ptr [rip + 0x20201bd]
1409143f3: movups xmmword ptr [rbx], xmm9
1409143f7: xorps xmm0, xmm0
1409143fa: addss xmm8, xmm0
1409143ff: mov r8, qword ptr [rdi + 0x450]
140914406: mov rsi, qword ptr [rdi + 0x458]
14091440d: shl rsi, 4
140914411: movabs r14, 0x601d3feb7ea275df
14091441b: movabs r15, 0xcb8ec583f4439065
140914425: nop word ptr cs:[rax + rax]
140914430: test rsi, rsi
140914433: je 0x14091450b
140914439: lea r12, [r8 + 0x10]
14091443d: mov rcx, qword ptr [r8]
140914440: mov rax, qword ptr [r8 + 8]
140914444: call qword ptr [rax + 0x40]
140914447: mov rbx, rax
14091444a: mov rcx, rax
14091444d: call qword ptr [rdx + 0x18]
140914450: add rsi, -0x10
140914454: mov r8, r12
140914457: test rbx, rbx
14091445a: je 0x140914430
14091445c: mov r8, r12
14091445f: cmp rax, r14
140914462: jne 0x140914430
140914464: mov r8, r12
140914467: cmp rdx, r15
14091446a: jne 0x140914430
14091446c: movlps qword ptr [rbx + 0x14], xmm7
140914470: movss dword ptr [rbx + 0x1c], xmm8
140914476: mov r12d, dword ptr [rsp + 0x38]
14091447b: mov r15, qword ptr [rsp + 0x48]
140914480: jmp 0x140913bec
140914485: lea rcx, [rdi + 0x390]
14091448c: call qword ptr [rax + 0x18]
14091448f: jmp 0x1409135c3
140914494: lea rcx, [rdi + 0x270]
14091449b: lea rax, [rip + 0x237d786]
1409144a2: call qword ptr [rax + 0x18]
1409144a5: jmp 0x1409135c3
1409144aa: lea rcx, [rdi + 0x2f0]
1409144b1: lea rax, [rip + 0x237d738]
1409144b8: call qword ptr [rax + 0x18]
1409144bb: jmp 0x1409135c3
1409144c0: lea rcx, [rdi + 0x1b8]
1409144c7: call qword ptr [rax + 0x18]
1409144ca: jmp 0x1409135c3
1409144cf: lea rcx, [rdi + 0x448]
1409144d6: lea rax, [rip + 0x237d783]
1409144dd: call qword ptr [rax + 0x18]
1409144e0: jmp 0x1409135c3
1409144e5: lea rax, [rip + 0x21d87fc]
1409144ec: jmp 0x1409144f5
1409144ee: lea rax, [rip + 0x21d88ee]
1409144f5: mov qword ptr [rsp + 0x240], rax
1409144fd: mov qword ptr [rsp + 0x248], 0x22
140914509: jmp 0x140914526
14091450b: lea rax, [rip + 0x21d88f3]
140914512: mov qword ptr [rsp + 0x240], rax
14091451a: mov qword ptr [rsp + 0x248], 0x1f
140914526: lea rax, [rsp + 0x240]
14091452e: mov qword ptr [rsp + 0x1c0], rax
140914536: lea rax, [rip - 0x80a6ad]
14091453d: mov qword ptr [rsp + 0x1c8], rax
140914545: lea rax, [rip + 0x21d87e4]
14091454c: mov qword ptr [rsp + 0xb0], rax
140914554: mov qword ptr [rsp + 0xb8], 2
140914560: mov qword ptr [rsp + 0xd0], 0
14091456c: lea rax, [rsp + 0x1c0]
140914574: mov qword ptr [rsp + 0xc0], rax
14091457c: mov qword ptr [rsp + 0xc8], 1
140914588: lea rdx, [rip + 0x21d8819]
14091458f: lea rcx, [rsp + 0xb0]
140914597: call 0x1428d9390
14091459c: int3 
