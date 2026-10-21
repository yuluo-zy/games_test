0x1408373b0: push rbp
0x1408373b1: push r15
0x1408373b3: push r14
0x1408373b5: push r13
0x1408373b7: push r12
0x1408373b9: push rsi
0x1408373ba: push rdi
0x1408373bb: push rbx
0x1408373bc: sub rsp, 0xb38
0x1408373c3: lea rbp, [rsp + 0x80]
0x1408373cb: mov qword ptr [rbp + 0xab0], 0xfffffffffffffffe
0x1408373d6: mov r14, r9
0x1408373d9: mov rdi, rdx
0x1408373dc: mov rbx, rcx
0x1408373df: mov rax, qword ptr [rbp + 0xb20]
0x1408373e6: mov rcx, qword ptr [rax + 0x130]
0x1408373ed: test rcx, rcx
0x1408373f0: mov qword ptr [rbp + 0xaa0], rdx
0x1408373f7: je 0x140837753
0x1408373fd: mov r12, rcx
0x140837400: mov qword ptr [rbp + 0xa80], r14
0x140837407: mov r15, qword ptr [rax + 0x128]
0x14083740e: lea rsi, [r8 + 0x24]
0x140837412: lea rcx, [r8 + 0x28]
0x140837416: mov r14, r8
0x140837419: call 0x140bd5790
0x14083741e: mov dword ptr [rbp + 0x4a8], 0
0x140837428: mov qword ptr [rbp + 0xa88], r14
0x14083742f: mov qword ptr [rbp + 0x4b0], r14
0x140837436: mov qword ptr [rbp + 0x988], 0
0x140837441: mov qword ptr [rbp + 0x990], rsi
0x140837448: mov qword ptr [rbp + 0x998], 0
0x140837453: mov qword ptr [rbp + 0xa90], rbx
0x14083745a: mov qword ptr [rsp + 0x40], rbx
0x14083745f: mov qword ptr [rsp + 0x38], rdi
0x140837464: mov qword ptr [rsp + 0x30], rax
0x140837469: lea rax, [rbp + 0x988]
0x140837470: mov qword ptr [rsp + 0x28], rax
0x140837475: mov qword ptr [rsp + 0x20], 1
0x14083747e: lea rdx, [rip + 0x225b70b]
0x140837485: lea rcx, [rbp - 0x38]
0x140837489: lea r9, [rbp + 0x4a8]
0x140837490: mov r8d, 0xd
0x140837496: call 0x140867970
0x14083749b: lea rcx, [rdi + 0x38]
0x14083749f: mov byte ptr [rbp + 0xaaf], 1
0x1408374a6: movabs rdx, 0x4bc362195b80f729
0x1408374b0: movabs r8, 0x982798ce80cbcefb
0x1408374ba: call 0x1400dd6f0
0x1408374bf: test rax, rax
0x1408374c2: mov r8, qword ptr [rbp + 0xaa0]
0x1408374c9: je 0x140837ae9
0x1408374cf: cmp qword ptr [r8 + 0x218], rdx
0x1408374d6: jbe 0x140837ae9
0x1408374dc: mov rax, qword ptr [r8 + 0x210]
0x1408374e3: mov rcx, qword ptr [rax + rdx*8]
0x1408374e7: test rcx, rcx
0x1408374ea: je 0x140837ae9
0x1408374f0: mov rax, qword ptr [r8 + 0x1e0]
0x1408374f7: not rcx
0x1408374fa: lea rcx, [rcx + rcx*2]
0x1408374fe: shl rcx, 5
0x140837502: cmp qword ptr [rax + rcx + 0x20], 0
0x140837508: je 0x140837ae9
0x14083750e: add rax, rcx
0x140837511: mov rax, qword ptr [rax + 0x10]
0x140837515: mov rcx, qword ptr [rax]
0x140837518: test rcx, rcx
0x14083751b: js 0x140837524
0x14083751d: cvtsi2ss xmm0, rcx
0x140837522: jmp 0x140837539
0x140837524: mov rdx, rcx
0x140837527: shr rdx, 1
0x14083752a: and ecx, 1
0x14083752d: or rcx, rdx
0x140837530: cvtsi2ss xmm0, rcx
0x140837535: addss xmm0, xmm0
0x140837539: cvtsi2ss xmm1, dword ptr [rax + 8]
0x14083753e: divss xmm1, dword ptr [rip + 0x21700fa]
0x140837546: addss xmm1, xmm0
0x14083754a: movss dword ptr [rbp + 0xa98], xmm1
0x140837552: mov byte ptr [rbp + 0xa9c], 0
0x140837559: mov rdi, r12
0x14083755c: shl rdi, 7
0x140837560: add rdi, r15
0x140837563: lea r12, [rbp + 0x4a8]
0x14083756a: lea r13, [rbp + 0x988]
0x140837571: cmp r15, rdi
0x140837574: je 0x140837708
0x14083757a: nop word ptr [rax + rax]
0x140837580: mov rsi, qword ptr [r15 + 0x50]
0x140837584: mov r14, qword ptr [r15 + 0x58]
0x140837588: mov byte ptr [rbp + 0xaaf], 1
0x14083758f: call 0x140be6750
0x140837594: mov ebx, eax
0x140837596: mov byte ptr [rbp + 0xaaf], 1
0x14083759d: xor ecx, ecx
0x14083759f: call 0x1400d9b20
0x1408375a4: test rax, rax
0x1408375a7: je 0x140837aa3
0x1408375ad: mov dword ptr [rbp + 0xa30], ebx
0x1408375b3: mov word ptr [rbp + 0xa38], 0
0x1408375bc: mov dword ptr [rbp + 0xa34], 0x3f800000
0x1408375c6: mov qword ptr [rbp + 0x988], 1
0x1408375d1: mov qword ptr [rbp + 0x990], rsi
0x1408375d8: mov qword ptr [rbp + 0xa28], r14
0x1408375df: mov qword ptr [rbp + 0x998], 0
0x1408375ea: mov qword ptr [rbp + 0x9a0], 4
0x1408375f5: mov qword ptr [rbp + 0x9a8], 0
0x140837600: movups xmm0, xmmword ptr [rax]
0x140837603: inc qword ptr [rax]
0x140837606: movups xmm1, xmmword ptr [rip + 0x225f123]
0x14083760d: lea rax, [rbp + 0x9f8]
0x140837614: movups xmmword ptr [rax + 0x10], xmm1
0x140837618: movups xmm1, xmmword ptr [rip + 0x225f101]
0x14083761f: movups xmmword ptr [rax], xmm1
0x140837622: movups xmmword ptr [rbp + 0xa18], xmm0
0x140837629: movabs rax, 0x100000003
0x140837633: mov qword ptr [rbp + 0x9c8], rax
0x14083763a: mov qword ptr [rbp + 0x9b0], 0
0x140837645: mov qword ptr [rbp + 0x9b8], 1
0x140837650: mov qword ptr [rbp + 0x9c0], 0
0x14083765b: mov byte ptr [rbp + 0xaaf], 1
0x140837662: mov rcx, r12
0x140837665: mov rdx, r13
0x140837668: lea r8, [rbp - 0x38]
0x14083766c: xor r9d, r9d
0x14083766f: call 0x140905be0
0x140837674: cmp byte ptr [r15 + 0x64], 0
0x140837679: je 0x140837ab8
0x14083767f: movzx eax, byte ptr [r15 + 0x65]
0x140837684: mov byte ptr [rbp + 0x559], al
0x14083768a: mov r8d, 0xf8
0x140837690: mov rcx, r13
0x140837693: mov rdx, r12
0x140837696: call 0x142923c00
0x14083769b: mov byte ptr [rbp + 0xaaf], 1
0x1408376a2: lea rcx, [rbp + 0x890]
0x1408376a9: mov rdx, r13
0x1408376ac: lea r8, [rbp + 0xa98]
0x1408376b3: mov r9, r15
0x1408376b6: call 0x14090b460
0x1408376bb: mov rsi, qword ptr [r15 + 0x70]
0x1408376bf: movzx eax, byte ptr [rbp + 0x939]
0x1408376c6: and al, 0x8e
0x1408376c8: or al, 0x21
0x1408376ca: mov byte ptr [rbp + 0x939], al
0x1408376d0: mov r8d, 0xf8
0x1408376d6: mov rcx, r12
0x1408376d9: lea rdx, [rbp + 0x890]
0x1408376e0: call 0x142923c00
0x1408376e5: mov byte ptr [rbp + 0xaaf], 1
0x1408376ec: mov rcx, r12
0x1408376ef: mov rdx, r15
0x1408376f2: mov r8, qword ptr [rbp + 0xaa0]
0x1408376f9: call rsi
0x1408376fb: sub r15, -0x80
0x1408376ff: cmp r15, rdi
0x140837702: jne 0x140837580
0x140837708: lea rdi, [rbp + 0x4a8]
0x14083770f: lea rdx, [rbp - 0x38]
0x140837713: mov r8d, 0x3e8
0x140837719: mov rcx, rdi
0x14083771c: call 0x142923c00
0x140837721: mov byte ptr [rbp + 0xaaf], 0
0x140837728: mov rcx, rdi
0x14083772b: call 0x140868370
0x140837730: mov rdi, qword ptr [rbp + 0xaa0]
0x140837737: mov rbx, qword ptr [rbp + 0xa90]
0x14083773e: mov r14, qword ptr [rbp + 0xa80]
0x140837745: mov r8, qword ptr [rbp + 0xa88]
0x14083774c: mov rax, qword ptr [rbp + 0xb20]
0x140837753: mov rcx, qword ptr [rax + 0x148]
0x14083775a: test rcx, rcx
0x14083775d: je 0x140837a8e
0x140837763: mov r12, rcx
0x140837766: mov r15, qword ptr [rax + 0x140]
0x14083776d: lea rsi, [r8 + 0x24]
0x140837771: add r8, 0x28
0x140837775: mov rcx, r8
0x140837778: call 0x140bd5790
0x14083777d: mov dword ptr [rbp + 0x4a8], 0
0x140837787: mov qword ptr [rbp + 0x4b0], r14
0x14083778e: mov qword ptr [rbp + 0x988], 0
0x140837799: mov qword ptr [rbp + 0x990], rsi
0x1408377a0: mov qword ptr [rbp + 0x998], 0
0x1408377ab: mov qword ptr [rsp + 0x40], rbx
0x1408377b0: mov qword ptr [rsp + 0x38], rdi
0x1408377b5: mov qword ptr [rsp + 0x30], rax
0x1408377ba: lea rax, [rbp + 0x988]
0x1408377c1: mov qword ptr [rsp + 0x28], rax
0x1408377c6: mov qword ptr [rsp + 0x20], 1
0x1408377cf: lea rdx, [rip + 0x225b3e2]
0x1408377d6: lea rcx, [rbp - 0x38]
0x1408377da: lea r9, [rbp + 0x4a8]
0x1408377e1: mov r8d, 0xc
0x1408377e7: call 0x140867970
0x1408377ec: lea rcx, [rdi + 0x38]
0x1408377f0: mov byte ptr [rbp + 0xaae], 1
0x1408377f7: movabs rdx, 0x4bc362195b80f729
0x140837801: movabs r8, 0x982798ce80cbcefb
0x14083780b: call 0x1400dd6f0
0x140837810: test rax, rax
0x140837813: mov r8, qword ptr [rbp + 0xaa0]
0x14083781a: je 0x140837afe
0x140837820: cmp qword ptr [r8 + 0x218], rdx
0x140837827: jbe 0x140837afe
0x14083782d: mov rax, qword ptr [r8 + 0x210]
0x140837834: mov rcx, qword ptr [rax + rdx*8]
0x140837838: test rcx, rcx
0x14083783b: je 0x140837afe
0x140837841: mov rax, qword ptr [r8 + 0x1e0]
0x140837848: not rcx
0x14083784b: lea rcx, [rcx + rcx*2]
0x14083784f: shl rcx, 5
0x140837853: cmp qword ptr [rax + rcx + 0x20], 0
0x140837859: je 0x140837afe
0x14083785f: add rax, rcx
0x140837862: mov rax, qword ptr [rax + 0x10]
0x140837866: mov rcx, qword ptr [rax]
0x140837869: test rcx, rcx
0x14083786c: js 0x140837875
0x14083786e: cvtsi2ss xmm0, rcx
0x140837873: jmp 0x14083788a
0x140837875: mov rdx, rcx
0x140837878: shr rdx, 1
0x14083787b: and ecx, 1
0x14083787e: or rcx, rdx
0x140837881: cvtsi2ss xmm0, rcx
0x140837886: addss xmm0, xmm0
0x14083788a: cvtsi2ss xmm1, dword ptr [rax + 8]
0x14083788f: divss xmm1, dword ptr [rip + 0x216fda9]
0x140837897: addss xmm1, xmm0
0x14083789b: movss dword ptr [rbp + 0xa98], xmm1
0x1408378a3: mov byte ptr [rbp + 0xa9c], 0
0x1408378aa: shl r12, 7
0x1408378ae: add r12, r15
0x1408378b1: mov qword ptr [rbp + 0xa90], r12
0x1408378b8: lea rdi, [rbp + 0x4a8]
0x1408378bf: lea rbx, [rbp + 0x988]
0x1408378c6: lea r12, [rbp + 0x3b0]
0x1408378cd: cmp r15, qword ptr [rbp + 0xa90]
0x1408378d4: je 0x140837a66
0x1408378da: nop word ptr [rax + rax]
0x1408378e0: mov rsi, qword ptr [r15 + 0x50]
0x1408378e4: mov r13, qword ptr [r15 + 0x58]
0x1408378e8: mov byte ptr [rbp + 0xaae], 1
0x1408378ef: call 0x140be6750
0x1408378f4: mov r14d, eax
0x1408378f7: mov byte ptr [rbp + 0xaae], 1
0x1408378fe: xor ecx, ecx
0x140837900: call 0x1400d9b20
0x140837905: test rax, rax
0x140837908: je 0x140837ac6
0x14083790e: mov dword ptr [rbp + 0xa30], r14d
0x140837915: mov word ptr [rbp + 0xa38], 0
0x14083791e: mov dword ptr [rbp + 0xa34], 0x3f800000
0x140837928: mov qword ptr [rbp + 0x988], 1
0x140837933: mov qword ptr [rbp + 0x990], rsi
0x14083793a: mov qword ptr [rbp + 0xa28], r13
0x140837941: mov qword ptr [rbp + 0x998], 0
0x14083794c: mov qword ptr [rbp + 0x9a0], 4
0x140837957: mov qword ptr [rbp + 0x9a8], 0
0x140837962: movups xmm0, xmmword ptr [rax]
0x140837965: inc qword ptr [rax]
0x140837968: movups xmm1, xmmword ptr [rip + 0x225edc1]
0x14083796f: lea rax, [rbp + 0x9f8]
0x140837976: movups xmmword ptr [rax + 0x10], xmm1
0x14083797a: movups xmm1, xmmword ptr [rip + 0x225ed9f]
0x140837981: movups xmmword ptr [rax], xmm1
0x140837984: movups xmmword ptr [rbp + 0xa18], xmm0
0x14083798b: movabs rax, 0x100000003
0x140837995: mov qword ptr [rbp + 0x9c8], rax
0x14083799c: mov qword ptr [rbp + 0x9b0], 0
0x1408379a7: mov qword ptr [rbp + 0x9b8], 1
0x1408379b2: mov qword ptr [rbp + 0x9c0], 0
0x1408379bd: mov byte ptr [rbp + 0xaae], 1
0x1408379c4: mov rcx, rdi
0x1408379c7: mov rdx, rbx
0x1408379ca: lea r8, [rbp - 0x38]
0x1408379ce: xor r9d, r9d
0x1408379d1: call 0x140905be0
0x1408379d6: cmp byte ptr [r15 + 0x64], 0
0x1408379db: je 0x140837adb
0x1408379e1: movzx eax, byte ptr [r15 + 0x65]
0x1408379e6: mov byte ptr [rbp + 0x559], al
0x1408379ec: mov r8d, 0xf8
0x1408379f2: mov rcx, rbx
0x1408379f5: mov rdx, rdi
0x1408379f8: call 0x142923c00
0x1408379fd: mov byte ptr [rbp + 0xaae], 1
0x140837a04: mov rcx, r12
0x140837a07: mov rdx, rbx
0x140837a0a: lea r8, [rbp + 0xa98]
0x140837a11: mov r9, r15
0x140837a14: call 0x14090b460
0x140837a19: mov rsi, qword ptr [r15 + 0x70]
0x140837a1d: movzx eax, byte ptr [rbp + 0x459]
0x140837a24: and al, 0x8e
0x140837a26: or al, 0x21
0x140837a28: mov byte ptr [rbp + 0x459], al
0x140837a2e: mov r8d, 0xf8
0x140837a34: mov rcx, rdi
0x140837a37: mov rdx, r12
0x140837a3a: call 0x142923c00
0x140837a3f: mov byte ptr [rbp + 0xaae], 1
0x140837a46: mov rcx, rdi
0x140837a49: mov rdx, r15
0x140837a4c: mov r8, qword ptr [rbp + 0xaa0]
0x140837a53: call rsi
0x140837a55: sub r15, -0x80
0x140837a59: cmp r15, qword ptr [rbp + 0xa90]
0x140837a60: jne 0x1408378e0
0x140837a66: lea rsi, [rbp + 0x4a8]
0x140837a6d: lea rdx, [rbp - 0x38]
0x140837a71: mov r8d, 0x3e8
0x140837a77: mov rcx, rsi
0x140837a7a: call 0x142923c00
0x140837a7f: mov byte ptr [rbp + 0xaae], 0
0x140837a86: mov rcx, rsi
0x140837a89: call 0x140868370
0x140837a8e: nop 
0x140837a8f: add rsp, 0xb38
0x140837a96: pop rbx
0x140837a97: pop rdi
0x140837a98: pop rsi
0x140837a99: pop r12
0x140837a9b: pop r13
0x140837a9d: pop r14
0x140837a9f: pop r15
0x140837aa1: pop rbp
0x140837aa2: ret 
0x140837aa3: mov byte ptr [rbp + 0xaaf], 1
0x140837aaa: lea rcx, [rip + 0x2402fef]
0x140837ab1: call 0x1428d80d0
0x140837ab6: jmp 0x140837b11
0x140837ab8: lea rcx, [rip + 0x225b0e1]
0x140837abf: call 0x1428d9310
0x140837ac4: jmp 0x140837b11
0x140837ac6: mov byte ptr [rbp + 0xaae], 1
0x140837acd: lea rcx, [rip + 0x2402fcc]
0x140837ad4: call 0x1428d80d0
0x140837ad9: jmp 0x140837b11
0x140837adb: lea rcx, [rip + 0x225b0e6]
0x140837ae2: call 0x1428d9310
0x140837ae7: jmp 0x140837b11
0x140837ae9: mov byte ptr [rbp + 0xaaf], 1
0x140837af0: lea rcx, [rip + 0x226e0e1]
0x140837af7: call 0x1428d9310
0x140837afc: jmp 0x140837b11
0x140837afe: mov byte ptr [rbp + 0xaae], 1
0x140837b05: lea rcx, [rip + 0x226e0cc]
0x140837b0c: call 0x1428d9310
0x140837b11: ud2 
0x140837b13: nop word ptr cs:[rax + rax]
0x140837b20: mov qword ptr [rsp + 0x10], rdx
0x140837b25: push rbp
0x140837b26: push r15
0x140837b28: push r14
0x140837b2a: push r13
0x140837b2c: push r12
0x140837b2e: push rsi
0x140837b2f: push rdi
0x140837b30: push rbx
0x140837b31: sub rsp, 0x48
0x140837b35: lea rbp, [rdx + 0x80]
0x140837b3c: lea rcx, [rbp + 0x4a8]
0x140837b43: call 0x1408935c0
0x140837b48: mov byte ptr [rbp + 0xaaf], 1
0x140837b4f: add rsp, 0x48
0x140837b53: pop rbx
0x140837b54: pop rdi
0x140837b55: pop rsi
0x140837b56: pop r12
0x140837b58: pop r13
0x140837b5a: pop r14
0x140837b5c: pop r15
0x140837b5e: pop rbp
0x140837b5f: ret 
0x140837b60: mov qword ptr [rsp + 0x10], rdx
0x140837b65: push rbp
0x140837b66: push r15
0x140837b68: push r14
0x140837b6a: push r13
0x140837b6c: push r12
0x140837b6e: push rsi
0x140837b6f: push rdi
0x140837b70: push rbx
0x140837b71: sub rsp, 0x48
0x140837b75: lea rbp, [rdx + 0x80]
0x140837b7c: cmp byte ptr [rbp + 0xaaf], 0
0x140837b83: jne 0x140837b96
0x140837b85: add rsp, 0x48
0x140837b89: pop rbx
0x140837b8a: pop rdi
0x140837b8b: pop rsi
0x140837b8c: pop r12
0x140837b8e: pop r13
0x140837b90: pop r14
0x140837b92: pop r15
0x140837b94: pop rbp
0x140837b95: ret 
0x140837b96: lea rcx, [rbp - 0x38]
0x140837b9a: call 0x1408923c0
0x140837b9f: jmp 0x140837b85
0x140837ba1: nop word ptr cs:[rax + rax]
0x140837bb0: mov qword ptr [rsp + 0x10], rdx
0x140837bb5: push rbp
0x140837bb6: push r15
0x140837bb8: push r14
0x140837bba: push r13
0x140837bbc: push r12
0x140837bbe: push rsi
0x140837bbf: push rdi
0x140837bc0: push rbx
0x140837bc1: sub rsp, 0x48
0x140837bc5: lea rbp, [rdx + 0x80]
0x140837bcc: lea rcx, [rbp + 0x4a8]
0x140837bd3: call 0x1408935c0
0x140837bd8: mov byte ptr [rbp + 0xaae], 1
0x140837bdf: add rsp, 0x48
0x140837be3: pop rbx
0x140837be4: pop rdi
0x140837be5: pop rsi
0x140837be6: pop r12
0x140837be8: pop r13
0x140837bea: pop r14
0x140837bec: pop r15
0x140837bee: pop rbp
0x140837bef: ret 
0x140837bf0: mov qword ptr [rsp + 0x10], rdx
0x140837bf5: push rbp
0x140837bf6: push r15
0x140837bf8: push r14
0x140837bfa: push r13
0x140837bfc: push r12
0x140837bfe: push rsi
0x140837bff: push rdi
0x140837c00: push rbx
0x140837c01: sub rsp, 0x48
0x140837c05: lea rbp, [rdx + 0x80]
0x140837c0c: cmp byte ptr [rbp + 0xaae], 0
0x140837c13: jne 0x140837c26
0x140837c15: add rsp, 0x48
0x140837c19: pop rbx
0x140837c1a: pop rdi
0x140837c1b: pop rsi
0x140837c1c: pop r12
0x140837c1e: pop r13
0x140837c20: pop r14
0x140837c22: pop r15
0x140837c24: pop rbp
0x140837c25: ret 
0x140837c26: lea rcx, [rbp - 0x38]
0x140837c2a: call 0x1408923c0
0x140837c2f: jmp 0x140837c15
