0x140833bc0: push rbp
0x140833bc1: push r15
0x140833bc3: push r14
0x140833bc5: push r13
0x140833bc7: push r12
0x140833bc9: push rsi
0x140833bca: push rdi
0x140833bcb: push rbx
0x140833bcc: sub rsp, 0xfc8
0x140833bd3: lea rbp, [rsp + 0x80]
0x140833bdb: movaps xmmword ptr [rbp + 0xf30], xmm6
0x140833be2: mov qword ptr [rbp + 0xf28], 0xfffffffffffffffe
0x140833bed: mov qword ptr [rbp + 0xee0], r9
0x140833bf4: mov qword ptr [rbp + 0xf08], r8
0x140833bfb: mov r14, rdx
0x140833bfe: mov qword ptr [rbp + 0xef8], rcx
0x140833c05: call 0x140c16a00
0x140833c0a: test al, al
0x140833c0c: mov byte ptr [rbp + 0xf17], al
0x140833c12: je 0x140833c73
0x140833c14: lea r12, [rip + 0x225e8d9]
0x140833c1b: mov edi, 0x45
0x140833c20: mov edx, 0x45
0x140833c25: mov rcx, r12
0x140833c28: call 0x140832920
0x140833c2d: cmp rax, 1
0x140833c31: jne 0x140833ce8
0x140833c37: mov r9, rdx
0x140833c3a: test rdx, rdx
0x140833c3d: je 0x140833c7c
0x140833c3f: cmp r9, 0x45
0x140833c43: jae 0x140833c7a
0x140833c45: lea rax, [rip + 0x225e8a8]
0x140833c4c: cmp byte ptr [r9 + rax], 0xbf
0x140833c51: jg 0x140833c7c
0x140833c53: lea rax, [rip + 0x225e9ae]
0x140833c5a: mov qword ptr [rsp + 0x20], rax
0x140833c5f: lea rcx, [rip + 0x225e88e]
0x140833c66: mov edx, 0x45
0x140833c6b: xor r8d, r8d
0x140833c6e: call 0x1428d9940
0x140833c73: xor eax, eax
0x140833c75: jmp 0x140833e54
0x140833c7a: jne 0x140833c53
0x140833c7c: lea r12, [rip + 0x225e871]
0x140833c83: mov rcx, r12
0x140833c86: mov rdx, r9
0x140833c89: call 0x140832920
0x140833c8e: cmp rax, 1
0x140833c92: jne 0x140833ce8
0x140833c94: mov r8, rdx
0x140833c97: add r8, 2
0x140833c9b: je 0x140833cd6
0x140833c9d: cmp r8, 0x45
0x140833ca1: jae 0x140833cd4
0x140833ca3: lea rax, [rip + 0x225e84a]
0x140833caa: cmp byte ptr [r8 + rax], 0xbf
0x140833caf: jg 0x140833cd6
0x140833cb1: lea rax, [rip + 0x225e968]
0x140833cb8: mov qword ptr [rsp + 0x20], rax
0x140833cbd: lea rcx, [rip + 0x225e830]
0x140833cc4: mov edx, 0x45
0x140833cc9: mov r9d, 0x45
0x140833ccf: call 0x1428d9940
0x140833cd4: jne 0x140833cb1
0x140833cd6: mov edi, 0x43
0x140833cdb: sub rdi, rdx
0x140833cde: lea r12, [rip + 0x225e80f]
0x140833ce5: add r12, r8
0x140833ce8: lea rax, [rip + 0x225ed0d]
0x140833cef: lea r13, [rip + 0x225ecc2]
0x140833cf6: nop word ptr cs:[rax + rax]
0x140833d00: movsx ecx, byte ptr [rax - 1]
0x140833d04: test ecx, ecx
0x140833d06: js 0x140833d20
0x140833d08: dec rax
0x140833d0b: cmp ecx, 0x5c
0x140833d0e: jne 0x140833d79
0x140833d10: jmp 0x140833d8e
0x140833d12: nop word ptr cs:[rax + rax]
0x140833d20: movzx edx, byte ptr [rax - 2]
0x140833d24: cmp dl, 0xc0
0x140833d27: jge 0x140833d4e
0x140833d29: movzx r8d, byte ptr [rax - 3]
0x140833d2e: cmp r8b, 0xc0
0x140833d32: jge 0x140833d57
0x140833d34: movzx r9d, byte ptr [rax - 4]
0x140833d39: add rax, -4
0x140833d3d: and r9d, 7
0x140833d41: shl r9d, 6
0x140833d45: and r8d, 0x3f
0x140833d49: or r8d, r9d
0x140833d4c: jmp 0x140833d5f
0x140833d4e: add rax, -2
0x140833d52: and edx, 0x1f
0x140833d55: jmp 0x140833d69
0x140833d57: add rax, -3
0x140833d5b: and r8d, 0xf
0x140833d5f: shl r8d, 6
0x140833d63: and edx, 0x3f
0x140833d66: or edx, r8d
0x140833d69: shl edx, 6
0x140833d6c: and cl, 0x3f
0x140833d6f: movzx ecx, cl
0x140833d72: or ecx, edx
0x140833d74: cmp ecx, 0x5c
0x140833d77: je 0x140833d8e
0x140833d79: cmp ecx, 0x2f
0x140833d7c: je 0x140833d8e
0x140833d7e: cmp rax, r13
0x140833d81: jne 0x140833d00
0x140833d87: mov esi, 0x44
0x140833d8c: jmp 0x140833ddd
0x140833d8e: lea r13, [rip + 0x225ec23]
0x140833d95: sub rax, r13
0x140833d98: mov r8, rax
0x140833d9b: inc r8
0x140833d9e: je 0x140833dd2
0x140833da0: cmp r8, 0x44
0x140833da4: jae 0x140833dd0
0x140833da6: cmp byte ptr [r8 + r13], 0xbf
0x140833dab: jg 0x140833dd2
0x140833dad: lea rax, [rip + 0x225e834]
0x140833db4: mov qword ptr [rsp + 0x20], rax
0x140833db9: lea rcx, [rip + 0x225ebf8]
0x140833dc0: mov edx, 0x44
0x140833dc5: mov r9d, 0x44
0x140833dcb: call 0x1428d9940
0x140833dd0: jne 0x140833dad
0x140833dd2: mov esi, 0x43
0x140833dd7: sub rsi, rax
0x140833dda: add r13, r8
0x140833ddd: call 0x140c18130
0x140833de2: mov rcx, rax
0x140833de5: mov rax, qword ptr [rax]
0x140833de8: cmp rax, 1
0x140833dec: jne 0x140835348
0x140833df2: add rcx, 8
0x140833df6: cmp qword ptr [rcx], 0
0x140833dfa: jne 0x140835382
0x140833e00: mov qword ptr [rcx], 0xffffffffffffffff
0x140833e07: mov qword ptr [rbp + 0xf18], rcx
0x140833e0e: add rcx, 8
0x140833e12: mov qword ptr [rsp + 0x20], rsi
0x140833e17: mov qword ptr [rsp + 0x30], 0
0x140833e20: mov qword ptr [rsp + 0x28], 1
0x140833e29: mov rdx, r12
0x140833e2c: mov r8, rdi
0x140833e2f: mov r9, r13
0x140833e32: call 0x140c16d40
0x140833e37: mov rcx, qword ptr [rbp + 0xf18]
0x140833e3e: inc qword ptr [rcx]
0x140833e41: mov qword ptr [rbp + 0xd90], rax
0x140833e48: mov qword ptr [rbp + 0xd78], rax
0x140833e4f: mov eax, 1
0x140833e54: mov qword ptr [rbp + 0xd70], rax
0x140833e5b: mov rax, qword ptr [rbp + 0xf08]
0x140833e62: lea rcx, [rax + 0x28]
0x140833e66: mov qword ptr [rbp + 0xee8], rcx
0x140833e6d: call 0x140bd5790
0x140833e72: mov rcx, qword ptr [rbp + 0xf08]
0x140833e79: lea rdx, [rcx + 0x24]
0x140833e7d: mov dword ptr [rbp + 0x988], 2
0x140833e87: mov qword ptr [rbp + 0x990], rcx
0x140833e8e: mov qword ptr [rbp + 0xde8], 0
0x140833e99: mov qword ptr [rbp + 0xef0], rdx
0x140833ea0: mov qword ptr [rbp + 0xdf0], rdx
0x140833ea7: mov qword ptr [rbp + 0xdf8], 1
0x140833eb2: mov rcx, qword ptr [rbp + 0xef8]
0x140833eb9: mov qword ptr [rsp + 0x40], rcx
0x140833ebe: mov qword ptr [rsp + 0x38], r14
0x140833ec3: mov qword ptr [rsp + 0x30], rax
0x140833ec8: lea rax, [rbp + 0xde8]
0x140833ecf: mov qword ptr [rsp + 0x28], rax
0x140833ed4: mov qword ptr [rsp + 0x20], 1
0x140833edd: lea rdx, [rip + 0x225eb18]
0x140833ee4: lea rcx, [rbp + 0x5a0]
0x140833eeb: lea r9, [rbp + 0x988]
0x140833ef2: mov r8d, 8
0x140833ef8: call 0x140867970
0x140833efd: mov qword ptr [rbp + 0xf18], r14
0x140833f04: lea rcx, [r14 + 0x38]
0x140833f08: mov byte ptr [rbp + 0xf27], 1
0x140833f0f: movabs rdx, 0x4bc362195b80f729
0x140833f19: movabs r8, 0x982798ce80cbcefb
0x140833f23: mov qword ptr [rbp + 0xf00], rcx
0x140833f2a: call 0x1400dd6f0
0x140833f2f: test rax, rax
0x140833f32: mov r8, qword ptr [rbp + 0xf18]
0x140833f39: je 0x140835473
0x140833f3f: cmp qword ptr [r8 + 0x218], rdx
0x140833f46: jbe 0x140835473
0x140833f4c: mov rax, qword ptr [r8 + 0x210]
0x140833f53: mov rcx, qword ptr [rax + rdx*8]
0x140833f57: test rcx, rcx
0x140833f5a: je 0x140835473
0x140833f60: mov rax, qword ptr [r8 + 0x1e0]
0x140833f67: not rcx
0x140833f6a: lea rcx, [rcx + rcx*2]
0x140833f6e: shl rcx, 5
0x140833f72: cmp qword ptr [rax + rcx + 0x20], 0
0x140833f78: je 0x140835473
0x140833f7e: add rax, rcx
0x140833f81: mov rax, qword ptr [rax + 0x10]
0x140833f85: mov rcx, qword ptr [rax]
0x140833f88: test rcx, rcx
0x140833f8b: js 0x140833f94
0x140833f8d: cvtsi2ss xmm0, rcx
0x140833f92: jmp 0x140833fa9
0x140833f94: mov rdx, rcx
0x140833f97: shr rdx, 1
0x140833f9a: and ecx, 1
0x140833f9d: or rcx, rdx
0x140833fa0: cvtsi2ss xmm0, rcx
0x140833fa5: addss xmm0, xmm0
0x140833fa9: cvtsi2ss xmm1, dword ptr [rax + 8]
0x140833fae: divss xmm1, dword ptr [rip + 0x217368a]
0x140833fb6: addss xmm1, xmm0
0x140833fba: movss dword ptr [rbp + 0xd98], xmm1
0x140833fc2: mov byte ptr [rbp + 0xd9c], 0
0x140833fc9: mov rax, qword ptr [rbp + 0xfb0]
0x140833fd0: mov r15, qword ptr [rax + 0x20]
0x140833fd4: mov r12, qword ptr [rax + 0x28]
0x140833fd8: shl r12, 7
0x140833fdc: add r12, r15
0x140833fdf: lea rbx, [rbp + 0x988]
0x140833fe6: lea rdi, [rbp + 0xde8]
0x140833fed: cmp r15, r12
0x140833ff0: je 0x14083418b
0x140833ff6: nop word ptr cs:[rax + rax]
0x140834000: mov r13, qword ptr [r15 + 0x50]
0x140834004: mov r14, qword ptr [r15 + 0x58]
0x140834008: mov byte ptr [rbp + 0xf27], 1
0x14083400f: call 0x140be6750
0x140834014: mov esi, eax
0x140834016: mov byte ptr [rbp + 0xf27], 1
0x14083401d: xor ecx, ecx
0x14083401f: call 0x1400d9b20
0x140834024: test rax, rax
0x140834027: je 0x14083538e
0x14083402d: mov dword ptr [rbp + 0xe90], esi
0x140834033: mov word ptr [rbp + 0xe98], 0
0x14083403c: mov dword ptr [rbp + 0xe94], 0x3f800000
0x140834046: mov qword ptr [rbp + 0xde8], 1
0x140834051: mov qword ptr [rbp + 0xdf0], r13
0x140834058: mov qword ptr [rbp + 0xe88], r14
0x14083405f: mov qword ptr [rbp + 0xdf8], 0
0x14083406a: mov qword ptr [rbp + 0xe00], 4
0x140834075: mov qword ptr [rbp + 0xe08], 0
0x140834080: movups xmm0, xmmword ptr [rax]
0x140834083: inc qword ptr [rax]
0x140834086: movups xmm1, xmmword ptr [rip + 0x22626a3]
0x14083408d: lea rax, [rbp + 0xe58]
0x140834094: movups xmmword ptr [rax + 0x10], xmm1
0x140834098: movups xmm1, xmmword ptr [rip + 0x2262681]
0x14083409f: movups xmmword ptr [rax], xmm1
0x1408340a2: movups xmmword ptr [rbp + 0xe78], xmm0
0x1408340a9: movabs rax, 0x100000003
0x1408340b3: mov qword ptr [rbp + 0xe28], rax
0x1408340ba: mov qword ptr [rbp + 0xe10], 0
0x1408340c5: mov qword ptr [rbp + 0xe18], 1
0x1408340d0: mov qword ptr [rbp + 0xe20], 0
0x1408340db: mov byte ptr [rbp + 0xf27], 1
0x1408340e2: mov rcx, rbx
0x1408340e5: mov rdx, rdi
0x1408340e8: lea r8, [rbp + 0x5a0]
0x1408340ef: xor r9d, r9d
0x1408340f2: call 0x140905be0
0x1408340f7: cmp byte ptr [r15 + 0x64], 0
0x1408340fc: je 0x1408353a6
0x140834102: movzx eax, byte ptr [r15 + 0x65]
0x140834107: mov byte ptr [rbp + 0xa39], al
0x14083410d: mov r8d, 0xf8
0x140834113: mov rcx, rdi
0x140834116: mov rdx, rbx
0x140834119: call 0x142923c00
0x14083411e: mov byte ptr [rbp + 0xf27], 1
0x140834125: lea rcx, [rbp + 0x4a8]
0x14083412c: mov rdx, rdi
0x14083412f: lea r8, [rbp + 0xd98]
0x140834136: mov r9, r15
0x140834139: call 0x14090b460
0x14083413e: mov rsi, qword ptr [r15 + 0x70]
0x140834142: movzx eax, byte ptr [rbp + 0x551]
0x140834149: and al, 0x8e
0x14083414b: or al, 0x21
0x14083414d: mov byte ptr [rbp + 0x551], al
0x140834153: mov r8d, 0xf8
0x140834159: mov rcx, rbx
0x14083415c: lea rdx, [rbp + 0x4a8]
0x140834163: call 0x142923c00
0x140834168: mov byte ptr [rbp + 0xf27], 1
0x14083416f: mov rcx, rbx
0x140834172: mov rdx, r15
0x140834175: mov r8, qword ptr [rbp + 0xf18]
0x14083417c: call rsi
0x14083417e: sub r15, -0x80
0x140834182: cmp r15, r12
0x140834185: jne 0x140834000
0x14083418b: lea rsi, [rbp + 0x988]
0x140834192: lea rdx, [rbp + 0x5a0]
0x140834199: mov r8d, 0x3e8
0x14083419f: mov rcx, rsi
0x1408341a2: call 0x142923c00
0x1408341a7: mov byte ptr [rbp + 0xf27], 0
0x1408341ae: mov rcx, rsi
0x1408341b1: call 0x140868370
0x1408341b6: mov rcx, qword ptr [rbp + 0xee8]
0x1408341bd: call 0x140bd5790
0x1408341c2: mov rcx, qword ptr [rbp + 0xf18]
0x1408341c9: mov dword ptr [rbp + 0x988], 0
0x1408341d3: mov rdx, qword ptr [rbp + 0xf08]
0x1408341da: mov qword ptr [rbp + 0x990], rdx
0x1408341e1: mov dword ptr [rbp + 0x9a8], 1
0x1408341eb: xorps xmm0, xmm0
0x1408341ee: movups xmmword ptr [rbp + 0x9ac], xmm0
0x1408341f5: mov rdx, qword ptr [rbp + 0xee0]
0x1408341fc: mov qword ptr [rbp + 0x9c0], rdx
0x140834203: mov qword ptr [rbp + 0xde8], 0
0x14083420e: mov rdx, qword ptr [rbp + 0xef0]
0x140834215: mov qword ptr [rbp + 0xdf0], rdx
0x14083421c: mov qword ptr [rbp + 0xdf8], 0
0x140834227: mov rdx, qword ptr [rbp + 0xef8]
0x14083422e: mov qword ptr [rsp + 0x40], rdx
0x140834233: mov qword ptr [rsp + 0x38], rcx
0x140834238: mov qword ptr [rsp + 0x30], rax
0x14083423d: mov qword ptr [rsp + 0x28], rdi
0x140834242: mov qword ptr [rsp + 0x20], 2
0x14083424b: lea rdx, [rip + 0x225e7b2]
0x140834252: lea rcx, [rbp + 0x5a0]
0x140834259: lea r9, [rbp + 0x988]
0x140834260: mov r8d, 0x11
0x140834266: call 0x140867970
0x14083426b: mov byte ptr [rbp + 0xf26], 1
0x140834272: movabs rdx, 0x4bc362195b80f729
0x14083427c: movabs r8, 0x982798ce80cbcefb
0x140834286: mov rcx, qword ptr [rbp + 0xf00]
0x14083428d: call 0x1400dd6f0
0x140834292: test rax, rax
0x140834295: mov r8, qword ptr [rbp + 0xf18]
0x14083429c: je 0x140835488
0x1408342a2: cmp qword ptr [r8 + 0x218], rdx
0x1408342a9: jbe 0x140835488
0x1408342af: mov rax, qword ptr [r8 + 0x210]
0x1408342b6: mov rcx, qword ptr [rax + rdx*8]
0x1408342ba: test rcx, rcx
0x1408342bd: je 0x140835488
0x1408342c3: mov rax, qword ptr [r8 + 0x1e0]
0x1408342ca: not rcx
0x1408342cd: lea rcx, [rcx + rcx*2]
0x1408342d1: shl rcx, 5
0x1408342d5: cmp qword ptr [rax + rcx + 0x20], 0
0x1408342db: je 0x140835488
0x1408342e1: add rax, rcx
0x1408342e4: mov rax, qword ptr [rax + 0x10]
0x1408342e8: mov rcx, qword ptr [rax]
0x1408342eb: test rcx, rcx
0x1408342ee: js 0x1408342fa
0x1408342f0: xorps xmm0, xmm0
0x1408342f3: cvtsi2ss xmm0, rcx
0x1408342f8: jmp 0x140834312
0x1408342fa: mov rdx, rcx
0x1408342fd: shr rdx, 1
0x140834300: and ecx, 1
0x140834303: or rcx, rdx
0x140834306: xorps xmm0, xmm0
0x140834309: cvtsi2ss xmm0, rcx
0x14083430e: addss xmm0, xmm0
0x140834312: cvtsi2ss xmm1, dword ptr [rax + 8]
0x140834317: divss xmm1, dword ptr [rip + 0x2173321]
0x14083431f: addss xmm1, xmm0
0x140834323: movss dword ptr [rbp + 0xd98], xmm1
0x14083432b: mov byte ptr [rbp + 0xd9c], 0
0x140834332: mov rax, qword ptr [rbp + 0xfb0]
0x140834339: mov r15, qword ptr [rax + 0x38]
0x14083433d: mov r12, qword ptr [rax + 0x40]
0x140834341: shl r12, 7
0x140834345: add r12, r15
0x140834348: lea rbx, [rbp + 0x988]
0x14083434f: lea rdi, [rbp + 0xde8]
0x140834356: cmp r15, r12
0x140834359: je 0x1408344eb
0x14083435f: nop 
0x140834360: mov r13, qword ptr [r15 + 0x50]
0x140834364: mov r14, qword ptr [r15 + 0x58]
0x140834368: mov byte ptr [rbp + 0xf26], 1
0x14083436f: call 0x140be6750
0x140834374: mov esi, eax
0x140834376: mov byte ptr [rbp + 0xf26], 1
0x14083437d: xor ecx, ecx
0x14083437f: call 0x1400d9b20
0x140834384: test rax, rax
0x140834387: je 0x1408353b7
0x14083438d: mov dword ptr [rbp + 0xe90], esi
0x140834393: mov word ptr [rbp + 0xe98], 0
0x14083439c: mov dword ptr [rbp + 0xe94], 0x3f800000
0x1408343a6: mov qword ptr [rbp + 0xde8], 1
0x1408343b1: mov qword ptr [rbp + 0xdf0], r13
0x1408343b8: mov qword ptr [rbp + 0xe88], r14
0x1408343bf: mov qword ptr [rbp + 0xdf8], 0
0x1408343ca: mov qword ptr [rbp + 0xe00], 4
0x1408343d5: mov qword ptr [rbp + 0xe08], 0
0x1408343e0: movups xmm0, xmmword ptr [rax]
0x1408343e3: inc qword ptr [rax]
0x1408343e6: movups xmm1, xmmword ptr [rip + 0x2262343]
0x1408343ed: lea rax, [rbp + 0xe58]
0x1408343f4: movups xmmword ptr [rax + 0x10], xmm1
0x1408343f8: movups xmm1, xmmword ptr [rip + 0x2262321]
0x1408343ff: movups xmmword ptr [rax], xmm1
0x140834402: movups xmmword ptr [rbp + 0xe78], xmm0
0x140834409: movabs rax, 0x100000003
0x140834413: mov qword ptr [rbp + 0xe28], rax
0x14083441a: mov qword ptr [rbp + 0xe10], 0
0x140834425: mov qword ptr [rbp + 0xe18], 1
0x140834430: mov qword ptr [rbp + 0xe20], 0
0x14083443b: mov byte ptr [rbp + 0xf26], 1
0x140834442: mov rcx, rbx
0x140834445: mov rdx, rdi
0x140834448: lea r8, [rbp + 0x5a0]
0x14083444f: xor r9d, r9d
0x140834452: call 0x140905be0
0x140834457: cmp byte ptr [r15 + 0x64], 0
0x14083445c: je 0x1408353cf
0x140834462: movzx eax, byte ptr [r15 + 0x65]
0x140834467: mov byte ptr [rbp + 0xa39], al
0x14083446d: mov r8d, 0xf8
0x140834473: mov rcx, rdi
0x140834476: mov rdx, rbx
0x140834479: call 0x142923c00
0x14083447e: mov byte ptr [rbp + 0xf26], 1
0x140834485: lea rcx, [rbp + 0x3b0]
0x14083448c: mov rdx, rdi
0x14083448f: lea r8, [rbp + 0xd98]
0x140834496: mov r9, r15
0x140834499: call 0x14090b460
0x14083449e: mov rsi, qword ptr [r15 + 0x70]
0x1408344a2: movzx eax, byte ptr [rbp + 0x459]
0x1408344a9: and al, 0x8e
0x1408344ab: or al, 0x21
0x1408344ad: mov byte ptr [rbp + 0x459], al
0x1408344b3: mov r8d, 0xf8
0x1408344b9: mov rcx, rbx
0x1408344bc: lea rdx, [rbp + 0x3b0]
0x1408344c3: call 0x142923c00
0x1408344c8: mov byte ptr [rbp + 0xf26], 1
0x1408344cf: mov rcx, rbx
0x1408344d2: mov rdx, r15
0x1408344d5: mov r8, qword ptr [rbp + 0xf18]
0x1408344dc: call rsi
0x1408344de: sub r15, -0x80
0x1408344e2: cmp r15, r12
0x1408344e5: jne 0x140834360
0x1408344eb: lea rsi, [rbp + 0x988]
0x1408344f2: lea rdx, [rbp + 0x5a0]
0x1408344f9: mov r8d, 0x3e8
0x1408344ff: mov rcx, rsi
0x140834502: call 0x142923c00
0x140834507: mov byte ptr [rbp + 0xf26], 0
0x14083450e: mov rcx, rsi
0x140834511: call 0x140868370
0x140834516: mov rcx, qword ptr [rbp + 0xee8]
0x14083451d: call 0x140bd5790
0x140834522: mov rcx, qword ptr [rbp + 0xf18]
0x140834529: mov dword ptr [rbp + 0x988], 0
0x140834533: mov rdx, qword ptr [rbp + 0xf08]
0x14083453a: mov qword ptr [rbp + 0x990], rdx
0x140834541: add rdx, 0x48
0x140834545: mov dword ptr [rbp + 0x9a8], 1
0x14083454f: xorps xmm0, xmm0
0x140834552: movups xmmword ptr [rbp + 0x9ac], xmm0
0x140834559: mov qword ptr [rbp + 0xf08], rdx
0x140834560: mov qword ptr [rbp + 0x9c0], rdx
0x140834567: mov qword ptr [rbp + 0xde8], 0
0x140834572: mov rdx, qword ptr [rbp + 0xef0]
0x140834579: mov qword ptr [rbp + 0xdf0], rdx
0x140834580: mov qword ptr [rbp + 0xdf8], 0
0x14083458b: mov rdx, qword ptr [rbp + 0xef8]
0x140834592: mov qword ptr [rsp + 0x40], rdx
0x140834597: mov qword ptr [rsp + 0x38], rcx
0x14083459c: mov qword ptr [rsp + 0x30], rax
0x1408345a1: mov qword ptr [rsp + 0x28], rdi
0x1408345a6: mov qword ptr [rsp + 0x20], 2
0x1408345af: lea rdx, [rip + 0x225e45f]
0x1408345b6: lea rcx, [rbp + 0x5a0]
0x1408345bd: lea r9, [rbp + 0x988]
0x1408345c4: mov r8d, 0x11
0x1408345ca: call 0x140867970
0x1408345cf: mov byte ptr [rbp + 0xf25], 1
0x1408345d6: movabs rdx, 0x4bc362195b80f729
0x1408345e0: movabs r8, 0x982798ce80cbcefb
0x1408345ea: mov rcx, qword ptr [rbp + 0xf00]
0x1408345f1: call 0x1400dd6f0
0x1408345f6: test rax, rax
0x1408345f9: mov r8, qword ptr [rbp + 0xf18]
0x140834600: je 0x14083549d
0x140834606: cmp qword ptr [r8 + 0x218], rdx
0x14083460d: jbe 0x14083549d
0x140834613: mov rax, qword ptr [r8 + 0x210]
0x14083461a: mov rcx, qword ptr [rax + rdx*8]
0x14083461e: test rcx, rcx
0x140834621: je 0x14083549d
0x140834627: mov rax, qword ptr [r8 + 0x1e0]
0x14083462e: not rcx
0x140834631: lea rcx, [rcx + rcx*2]
0x140834635: shl rcx, 5
0x140834639: cmp qword ptr [rax + rcx + 0x20], 0
0x14083463f: je 0x14083549d
0x140834645: add rax, rcx
0x140834648: mov rax, qword ptr [rax + 0x10]
0x14083464c: mov rcx, qword ptr [rax]
0x14083464f: test rcx, rcx
0x140834652: js 0x14083465e
0x140834654: xorps xmm0, xmm0
0x140834657: cvtsi2ss xmm0, rcx
0x14083465c: jmp 0x140834676
0x14083465e: mov rdx, rcx
0x140834661: shr rdx, 1
0x140834664: and ecx, 1
0x140834667: or rcx, rdx
0x14083466a: xorps xmm0, xmm0
0x14083466d: cvtsi2ss xmm0, rcx
0x140834672: addss xmm0, xmm0
0x140834676: cvtsi2ss xmm1, dword ptr [rax + 8]
0x14083467b: divss xmm1, dword ptr [rip + 0x2172fbd]
0x140834683: addss xmm1, xmm0
0x140834687: movss dword ptr [rbp + 0xd98], xmm1
0x14083468f: mov byte ptr [rbp + 0xd9c], 0
0x140834696: mov rax, qword ptr [rbp + 0xfb0]
0x14083469d: mov r15, qword ptr [rax + 0x50]
0x1408346a1: mov rdi, qword ptr [rax + 0x58]
0x1408346a5: shl rdi, 7
0x1408346a9: add rdi, r15
0x1408346ac: lea rbx, [rbp + 0x988]
0x1408346b3: lea r13, [rbp + 0xc8]
0x1408346ba: cmp r15, rdi
0x1408346bd: je 0x14083485e
0x1408346c3: nop word ptr cs:[rax + rax]
0x1408346d0: mov r14, qword ptr [r15 + 0x50]
0x1408346d4: mov r12, qword ptr [r15 + 0x58]
0x1408346d8: mov byte ptr [rbp + 0xf25], 1
0x1408346df: call 0x140be6750
0x1408346e4: mov esi, eax
0x1408346e6: mov byte ptr [rbp + 0xf25], 1
0x1408346ed: xor ecx, ecx
0x1408346ef: call 0x1400d9b20
0x1408346f4: test rax, rax
0x1408346f7: je 0x1408353e0
0x1408346fd: mov dword ptr [rbp + 0xe90], esi
0x140834703: mov word ptr [rbp + 0xe98], 0
0x14083470c: mov dword ptr [rbp + 0xe94], 0x3f800000
0x140834716: mov qword ptr [rbp + 0xde8], 1
0x140834721: mov qword ptr [rbp + 0xdf0], r14
0x140834728: mov qword ptr [rbp + 0xe88], r12
0x14083472f: mov qword ptr [rbp + 0xdf8], 0
0x14083473a: mov qword ptr [rbp + 0xe00], 4
0x140834745: mov qword ptr [rbp + 0xe08], 0
0x140834750: movups xmm0, xmmword ptr [rax]
0x140834753: inc qword ptr [rax]
0x140834756: movups xmm1, xmmword ptr [rip + 0x2261fd3]
0x14083475d: lea rax, [rbp + 0xe58]
0x140834764: movups xmmword ptr [rax + 0x10], xmm1
0x140834768: movups xmm1, xmmword ptr [rip + 0x2261fb1]
0x14083476f: movups xmmword ptr [rax], xmm1
0x140834772: movups xmmword ptr [rbp + 0xe78], xmm0
0x140834779: movabs rax, 0x100000003
0x140834783: mov qword ptr [rbp + 0xe28], rax
0x14083478a: mov qword ptr [rbp + 0xe10], 0
0x140834795: mov qword ptr [rbp + 0xe18], 1
0x1408347a0: mov qword ptr [rbp + 0xe20], 0
0x1408347ab: mov byte ptr [rbp + 0xf25], 1
0x1408347b2: mov rcx, rbx
0x1408347b5: lea rdx, [rbp + 0xde8]
0x1408347bc: lea r8, [rbp + 0x5a0]
0x1408347c3: xor r9d, r9d
0x1408347c6: call 0x140905be0
0x1408347cb: cmp byte ptr [r15 + 0x64], 0
0x1408347d0: je 0x1408353f8
0x1408347d6: movzx eax, byte ptr [r15 + 0x65]
0x1408347db: mov byte ptr [rbp + 0xa39], al
0x1408347e1: mov r8d, 0xf8
0x1408347e7: lea rsi, [rbp + 0xde8]
0x1408347ee: mov rcx, rsi
0x1408347f1: mov rdx, rbx
0x1408347f4: call 0x142923c00
0x1408347f9: mov byte ptr [rbp + 0xf25], 1
0x140834800: mov rcx, r13
0x140834803: mov rdx, rsi
0x140834806: lea r8, [rbp + 0xd98]
0x14083480d: mov r9, r15
0x140834810: call 0x14090b460
0x140834815: mov rsi, qword ptr [r15 + 0x70]
0x140834819: movzx eax, byte ptr [rbp + 0x171]
0x140834820: and al, 0x8e
0x140834822: or al, 0x21
0x140834824: mov byte ptr [rbp + 0x171], al
0x14083482a: mov r8d, 0xf8
0x140834830: mov rcx, rbx
0x140834833: mov rdx, r13
0x140834836: call 0x142923c00
0x14083483b: mov byte ptr [rbp + 0xf25], 1
0x140834842: mov rcx, rbx
0x140834845: mov rdx, r15
0x140834848: mov r8, qword ptr [rbp + 0xf18]
0x14083484f: call rsi
0x140834851: sub r15, -0x80
0x140834855: cmp r15, rdi
0x140834858: jne 0x1408346d0
0x14083485e: lea rsi, [rbp + 0x988]
0x140834865: lea rdx, [rbp + 0x5a0]
0x14083486c: mov r8d, 0x3e8
0x140834872: mov rcx, rsi
0x140834875: call 0x142923c00
0x14083487a: mov byte ptr [rbp + 0xf25], 0
0x140834881: mov rcx, rsi
0x140834884: call 0x140868370
0x140834889: movabs rdx, 0xc95ed87f413070ab
0x140834893: movabs r8, 0xe67d7e3df983f8ce
0x14083489d: mov rcx, qword ptr [rbp + 0xf00]
0x1408348a4: call 0x1400dd6f0
0x1408348a9: mov rsi, qword ptr [rbp + 0xf18]
0x1408348b0: test rax, rax
0x1408348b3: je 0x1408354b2
0x1408348b9: cmp qword ptr [rsi + 0x218], rdx
0x1408348c0: jbe 0x1408354b2
0x1408348c6: mov rax, qword ptr [rsi + 0x210]
0x1408348cd: mov rcx, qword ptr [rax + rdx*8]
0x1408348d1: test rcx, rcx
0x1408348d4: je 0x1408354b2
0x1408348da: mov rax, qword ptr [rsi + 0x1e0]
0x1408348e1: not rcx
0x1408348e4: lea rcx, [rcx + rcx*2]
0x1408348e8: shl rcx, 5
0x1408348ec: cmp qword ptr [rax + rcx + 0x20], 0
0x1408348f2: je 0x1408354b2
0x1408348f8: add rax, rcx
0x1408348fb: mov rax, qword ptr [rax + 0x10]
0x1408348ff: cmp dword ptr [rax], 0
0x140834902: jne 0x140834fab
0x140834908: mov rcx, qword ptr [rbp + 0xee8]
0x14083490f: call 0x140bd5790
0x140834914: mov dword ptr [rbp + 0x988], 0
0x14083491e: mov rcx, qword ptr [rbp + 0xee0]
0x140834925: mov qword ptr [rbp + 0x990], rcx
0x14083492c: mov qword ptr [rbp + 0xde8], 0
0x140834937: mov rcx, qword ptr [rbp + 0xef0]
0x14083493e: mov qword ptr [rbp + 0xdf0], rcx
0x140834945: mov qword ptr [rbp + 0xdf8], 0
0x140834950: mov rcx, qword ptr [rbp + 0xef8]
0x140834957: mov qword ptr [rsp + 0x40], rcx
0x14083495c: mov qword ptr [rsp + 0x38], rsi
0x140834961: mov qword ptr [rsp + 0x30], rax
0x140834966: lea rax, [rbp + 0xde8]
0x14083496d: mov qword ptr [rsp + 0x28], rax
0x140834972: mov qword ptr [rsp + 0x20], 1
0x14083497b: lea rdx, [rip + 0x225e0be]
0x140834982: lea rcx, [rbp + 0x5a0]
0x140834989: lea r9, [rbp + 0x988]
0x140834990: mov r8d, 7
0x140834996: call 0x140867970
0x14083499b: mov byte ptr [rbp + 0xf24], 1
0x1408349a2: movabs rdx, 0x4bc362195b80f729
0x1408349ac: movabs r8, 0x982798ce80cbcefb
0x1408349b6: mov rcx, qword ptr [rbp + 0xf00]
0x1408349bd: call 0x1400dd6f0
0x1408349c2: test rax, rax
0x1408349c5: mov r8, qword ptr [rbp + 0xf18]
0x1408349cc: je 0x1408354d5
0x1408349d2: cmp qword ptr [r8 + 0x218], rdx
0x1408349d9: jbe 0x1408354d5
0x1408349df: mov rax, qword ptr [r8 + 0x210]
0x1408349e6: mov rcx, qword ptr [rax + rdx*8]
0x1408349ea: test rcx, rcx
0x1408349ed: je 0x1408354d5
0x1408349f3: mov rax, qword ptr [r8 + 0x1e0]
0x1408349fa: not rcx
0x1408349fd: lea rcx, [rcx + rcx*2]
0x140834a01: shl rcx, 5
0x140834a05: cmp qword ptr [rax + rcx + 0x20], 0
0x140834a0b: je 0x1408354d5
0x140834a11: add rax, rcx
0x140834a14: mov rax, qword ptr [rax + 0x10]
0x140834a18: mov rcx, qword ptr [rax]
0x140834a1b: test rcx, rcx
0x140834a1e: js 0x140834a27
0x140834a20: cvtsi2ss xmm0, rcx
0x140834a25: jmp 0x140834a3c
0x140834a27: mov rdx, rcx
0x140834a2a: shr rdx, 1
0x140834a2d: and ecx, 1
0x140834a30: or rcx, rdx
0x140834a33: cvtsi2ss xmm0, rcx
0x140834a38: addss xmm0, xmm0
0x140834a3c: cvtsi2ss xmm1, dword ptr [rax + 8]
0x140834a41: divss xmm1, dword ptr [rip + 0x2172bf7]
0x140834a49: addss xmm1, xmm0
0x140834a4d: movss dword ptr [rbp + 0xd98], xmm1
0x140834a55: mov byte ptr [rbp + 0xd9c], 0
0x140834a5c: mov rax, qword ptr [rbp + 0xfb0]
0x140834a63: mov rbx, qword ptr [rax + 0x68]
0x140834a67: mov rdi, qword ptr [rax + 0x70]
0x140834a6b: shl rdi, 7
0x140834a6f: add rdi, rbx
0x140834a72: lea r12, [rbp + 0x988]
0x140834a79: lea r15, [rbp + 0xde8]
0x140834a80: cmp rbx, rdi
0x140834a83: je 0x140834c19
0x140834a89: nop dword ptr [rax]
0x140834a90: mov r14, qword ptr [rbx + 0x50]
0x140834a94: mov r13, qword ptr [rbx + 0x58]
0x140834a98: mov byte ptr [rbp + 0xf24], 1
0x140834a9f: call 0x140be6750
0x140834aa4: mov esi, eax
0x140834aa6: mov byte ptr [rbp + 0xf24], 1
0x140834aad: xor ecx, ecx
0x140834aaf: call 0x1400d9b20
0x140834ab4: test rax, rax
0x140834ab7: je 0x140835421
0x140834abd: mov dword ptr [rbp + 0xe90], esi
0x140834ac3: mov word ptr [rbp + 0xe98], 0
0x140834acc: mov dword ptr [rbp + 0xe94], 0x3f800000
0x140834ad6: mov qword ptr [rbp + 0xde8], 1
0x140834ae1: mov qword ptr [rbp + 0xdf0], r14
0x140834ae8: mov qword ptr [rbp + 0xe88], r13
0x140834aef: mov qword ptr [rbp + 0xdf8], 0
0x140834afa: mov qword ptr [rbp + 0xe00], 4
0x140834b05: mov qword ptr [rbp + 0xe08], 0
0x140834b10: movups xmm0, xmmword ptr [rax]
0x140834b13: inc qword ptr [rax]
0x140834b16: movups xmm1, xmmword ptr [rip + 0x2261c13]
0x140834b1d: lea rax, [rbp + 0xe58]
0x140834b24: movups xmmword ptr [rax + 0x10], xmm1
0x140834b28: movups xmm1, xmmword ptr [rip + 0x2261bf1]
0x140834b2f: movups xmmword ptr [rax], xmm1
0x140834b32: movups xmmword ptr [rbp + 0xe78], xmm0
0x140834b39: movabs rax, 0x100000003
0x140834b43: mov qword ptr [rbp + 0xe28], rax
0x140834b4a: mov qword ptr [rbp + 0xe10], 0
0x140834b55: mov qword ptr [rbp + 0xe18], 1
0x140834b60: mov qword ptr [rbp + 0xe20], 0
0x140834b6b: mov byte ptr [rbp + 0xf24], 1
0x140834b72: mov rcx, r12
0x140834b75: mov rdx, r15
0x140834b78: lea r8, [rbp + 0x5a0]
0x140834b7f: xor r9d, r9d
0x140834b82: call 0x140905be0
0x140834b87: cmp byte ptr [rbx + 0x64], 0
0x140834b8b: je 0x140835439
0x140834b91: movzx eax, byte ptr [rbx + 0x65]
0x140834b95: mov byte ptr [rbp + 0xa39], al
0x140834b9b: mov r8d, 0xf8
0x140834ba1: mov rcx, r15
0x140834ba4: mov rdx, r12
0x140834ba7: call 0x142923c00
0x140834bac: mov byte ptr [rbp + 0xf24], 1
0x140834bb3: lea rcx, [rbp + 0x2b8]
0x140834bba: mov rdx, r15
0x140834bbd: lea r8, [rbp + 0xd98]
0x140834bc4: mov r9, rbx
0x140834bc7: call 0x14090b460
0x140834bcc: mov rsi, qword ptr [rbx + 0x70]
0x140834bd0: movzx eax, byte ptr [rbp + 0x361]
0x140834bd7: and al, 0x8e
0x140834bd9: or al, 0x21
0x140834bdb: mov byte ptr [rbp + 0x361], al
0x140834be1: mov r8d, 0xf8
0x140834be7: mov rcx, r12
0x140834bea: lea rdx, [rbp + 0x2b8]
0x140834bf1: call 0x142923c00
0x140834bf6: mov byte ptr [rbp + 0xf24], 1
0x140834bfd: mov rcx, r12
0x140834c00: mov rdx, rbx
0x140834c03: mov r8, qword ptr [rbp + 0xf18]
0x140834c0a: call rsi
0x140834c0c: sub rbx, -0x80
0x140834c10: cmp rbx, rdi
0x140834c13: jne 0x140834a90
0x140834c19: lea rsi, [rbp + 0x988]
0x140834c20: lea rdx, [rbp + 0x5a0]
0x140834c27: mov r8d, 0x3e8
0x140834c2d: mov rcx, rsi
0x140834c30: call 0x142923c00
0x140834c35: mov byte ptr [rbp + 0xf24], 0
0x140834c3c: mov rcx, rsi
0x140834c3f: call 0x140868370
0x140834c44: mov rcx, qword ptr [rbp + 0xee8]
0x140834c4b: call 0x140bd5790
0x140834c50: mov rcx, qword ptr [rbp + 0xf18]
0x140834c57: mov dword ptr [rbp + 0x988], 0
0x140834c61: mov rdx, qword ptr [rbp + 0xee0]
0x140834c68: mov qword ptr [rbp + 0x990], rdx
0x140834c6f: mov dword ptr [rbp + 0x9a8], 0
0x140834c79: mov rdx, qword ptr [rbp + 0xf08]
0x140834c80: mov qword ptr [rbp + 0x9b0], rdx
0x140834c87: mov qword ptr [rbp + 0xde8], 0
0x140834c92: mov rdx, qword ptr [rbp + 0xef0]
0x140834c99: mov qword ptr [rbp + 0xdf0], rdx
0x140834ca0: mov qword ptr [rbp + 0xdf8], 0
0x140834cab: mov rdx, qword ptr [rbp + 0xef8]
0x140834cb2: mov qword ptr [rsp + 0x40], rdx
0x140834cb7: mov qword ptr [rsp + 0x38], rcx
0x140834cbc: mov qword ptr [rsp + 0x30], rax
0x140834cc1: mov qword ptr [rsp + 0x28], r15
0x140834cc6: mov qword ptr [rsp + 0x20], 2
0x140834ccf: lea rdx, [rip + 0x225dd71]
0x140834cd6: lea rcx, [rbp + 0x5a0]
0x140834cdd: lea r9, [rbp + 0x988]
0x140834ce4: mov r8d, 0xf
0x140834cea: call 0x140867970
0x140834cef: mov byte ptr [rbp + 0xf23], 1
0x140834cf6: movabs rdx, 0x4bc362195b80f729
0x140834d00: movabs r8, 0x982798ce80cbcefb
0x140834d0a: mov rcx, qword ptr [rbp + 0xf00]
0x140834d11: call 0x1400dd6f0
0x140834d16: test rax, rax
0x140834d19: mov r8, qword ptr [rbp + 0xf18]
0x140834d20: je 0x1408354ea
0x140834d26: cmp qword ptr [r8 + 0x218], rdx
0x140834d2d: jbe 0x1408354ea
0x140834d33: mov rax, qword ptr [r8 + 0x210]
0x140834d3a: mov rcx, qword ptr [rax + rdx*8]
0x140834d3e: test rcx, rcx
0x140834d41: je 0x1408354ea
0x140834d47: mov rax, qword ptr [r8 + 0x1e0]
0x140834d4e: not rcx
0x140834d51: lea rcx, [rcx + rcx*2]
0x140834d55: shl rcx, 5
0x140834d59: cmp qword ptr [rax + rcx + 0x20], 0
0x140834d5f: je 0x1408354ea
0x140834d65: add rax, rcx
0x140834d68: mov rax, qword ptr [rax + 0x10]
0x140834d6c: mov rcx, qword ptr [rax]
0x140834d6f: test rcx, rcx
0x140834d72: js 0x140834d7b
0x140834d74: cvtsi2ss xmm0, rcx
0x140834d79: jmp 0x140834d90
0x140834d7b: mov rdx, rcx
0x140834d7e: shr rdx, 1
0x140834d81: and ecx, 1
0x140834d84: or rcx, rdx
0x140834d87: cvtsi2ss xmm0, rcx
0x140834d8c: addss xmm0, xmm0
0x140834d90: cvtsi2ss xmm1, dword ptr [rax + 8]
0x140834d95: divss xmm1, dword ptr [rip + 0x21728a3]
0x140834d9d: addss xmm1, xmm0
0x140834da1: movss dword ptr [rbp + 0xd98], xmm1
0x140834da9: mov byte ptr [rbp + 0xd9c], 0
0x140834db0: mov rax, qword ptr [rbp + 0xfb0]
0x140834db7: mov rbx, qword ptr [rax + 0x80]
0x140834dbe: mov rdi, qword ptr [rax + 0x88]
0x140834dc5: shl rdi, 7
0x140834dc9: add rdi, rbx
0x140834dcc: lea r15, [rbp + 0x988]
0x140834dd3: lea r12, [rbp + 0xde8]
0x140834dda: cmp rbx, rdi
0x140834ddd: je 0x140834f79
0x140834de3: nop word ptr cs:[rax + rax]
0x140834df0: mov r14, qword ptr [rbx + 0x50]
0x140834df4: mov r13, qword ptr [rbx + 0x58]
0x140834df8: mov byte ptr [rbp + 0xf23], 1
0x140834dff: call 0x140be6750
0x140834e04: mov esi, eax
0x140834e06: mov byte ptr [rbp + 0xf23], 1
0x140834e0d: xor ecx, ecx
0x140834e0f: call 0x1400d9b20
0x140834e14: test rax, rax
0x140834e17: je 0x14083544a
0x140834e1d: mov dword ptr [rbp + 0xe90], esi
0x140834e23: mov word ptr [rbp + 0xe98], 0
0x140834e2c: mov dword ptr [rbp + 0xe94], 0x3f800000
0x140834e36: mov qword ptr [rbp + 0xde8], 1
0x140834e41: mov qword ptr [rbp + 0xdf0], r14
0x140834e48: mov qword ptr [rbp + 0xe88], r13
0x140834e4f: mov qword ptr [rbp + 0xdf8], 0
0x140834e5a: mov qword ptr [rbp + 0xe00], 4
0x140834e65: mov qword ptr [rbp + 0xe08], 0
0x140834e70: movups xmm0, xmmword ptr [rax]
0x140834e73: inc qword ptr [rax]
0x140834e76: movups xmm1, xmmword ptr [rip + 0x22618b3]
0x140834e7d: lea rax, [rbp + 0xe58]
0x140834e84: movups xmmword ptr [rax + 0x10], xmm1
0x140834e88: movups xmm1, xmmword ptr [rip + 0x2261891]
0x140834e8f: movups xmmword ptr [rax], xmm1
0x140834e92: movups xmmword ptr [rbp + 0xe78], xmm0
0x140834e99: movabs rax, 0x100000003
0x140834ea3: mov qword ptr [rbp + 0xe28], rax
0x140834eaa: mov qword ptr [rbp + 0xe10], 0
0x140834eb5: mov qword ptr [rbp + 0xe18], 1
0x140834ec0: mov qword ptr [rbp + 0xe20], 0
0x140834ecb: mov byte ptr [rbp + 0xf23], 1
0x140834ed2: mov rcx, r15
0x140834ed5: mov rdx, r12
0x140834ed8: lea r8, [rbp + 0x5a0]
0x140834edf: xor r9d, r9d
0x140834ee2: call 0x140905be0
0x140834ee7: cmp byte ptr [rbx + 0x64], 0
0x140834eeb: je 0x140835462
0x140834ef1: movzx eax, byte ptr [rbx + 0x65]
0x140834ef5: mov byte ptr [rbp + 0xa39], al
0x140834efb: mov r8d, 0xf8
0x140834f01: mov rcx, r12
0x140834f04: mov rdx, r15
0x140834f07: call 0x142923c00
0x140834f0c: mov byte ptr [rbp + 0xf23], 1
0x140834f13: lea rcx, [rbp + 0x1c0]
0x140834f1a: mov rdx, r12
0x140834f1d: lea r8, [rbp + 0xd98]
0x140834f24: mov r9, rbx
0x140834f27: call 0x14090b460
0x140834f2c: mov rsi, qword ptr [rbx + 0x70]
0x140834f30: movzx eax, byte ptr [rbp + 0x269]
0x140834f37: and al, 0x8e
0x140834f39: or al, 0x21
0x140834f3b: mov byte ptr [rbp + 0x269], al
0x140834f41: mov r8d, 0xf8
0x140834f47: mov rcx, r15
0x140834f4a: lea rdx, [rbp + 0x1c0]
0x140834f51: call 0x142923c00
0x140834f56: mov byte ptr [rbp + 0xf23], 1
0x140834f5d: mov rcx, r15
0x140834f60: mov rdx, rbx
0x140834f63: mov r8, qword ptr [rbp + 0xf18]
0x140834f6a: call rsi
0x140834f6c: sub rbx, -0x80
0x140834f70: cmp rbx, rdi
0x140834f73: jne 0x140834df0
0x140834f79: lea rsi, [rbp + 0x988]
0x140834f80: lea rdx, [rbp + 0x5a0]
0x140834f87: mov r8d, 0x3e8
0x140834f8d: mov rcx, rsi
0x140834f90: call 0x142923c00
0x140834f95: mov byte ptr [rbp + 0xf23], 0
0x140834f9c: mov rcx, rsi
0x140834f9f: call 0x140868370
0x140834fa4: mov rsi, qword ptr [rbp + 0xf18]
0x140834fab: mov rcx, qword ptr [rbp + 0xee8]
0x140834fb2: call 0x140bd5790
0x140834fb7: mov dword ptr [rbp + 0x988], 0
0x140834fc1: mov rcx, qword ptr [rbp + 0xf08]
0x140834fc8: mov qword ptr [rbp + 0x990], rcx
0x140834fcf: mov qword ptr [rbp + 0xde8], 1
0x140834fda: mov rcx, qword ptr [rbp + 0xef0]
0x140834fe1: mov qword ptr [rbp + 0xdf0], rcx
0x140834fe8: mov qword ptr [rbp + 0xdf8], 0
0x140834ff3: mov rcx, qword ptr [rbp + 0xef8]
0x140834ffa: mov qword ptr [rsp + 0x40], rcx
0x140834fff: mov qword ptr [rsp + 0x38], rsi
0x140835004: mov qword ptr [rsp + 0x30], rax
0x140835009: lea rax, [rbp + 0xde8]
0x140835010: mov qword ptr [rsp + 0x28], rax
0x140835015: mov qword ptr [rsp + 0x20], 1
0x14083501e: lea rdx, [rip + 0x225da31]
0x140835025: lea rcx, [rbp + 0x5a0]
0x14083502c: lea r9, [rbp + 0x988]
0x140835033: mov r8d, 8
0x140835039: call 0x140867970
0x14083503e: mov byte ptr [rbp + 0xf22], 1
0x140835045: movabs rdx, 0x4bc362195b80f729
0x14083504f: movabs r8, 0x982798ce80cbcefb
0x140835059: mov rcx, qword ptr [rbp + 0xf00]
0x140835060: call 0x1400dd6f0
0x140835065: test rax, rax
0x140835068: mov r8, qword ptr [rbp + 0xf18]
0x14083506f: je 0x1408354c0
0x140835075: cmp qword ptr [r8 + 0x218], rdx
0x14083507c: jbe 0x1408354c0
0x140835082: mov rax, qword ptr [r8 + 0x210]
0x140835089: mov rcx, qword ptr [rax + rdx*8]
0x14083508d: test rcx, rcx
0x140835090: je 0x1408354c0
0x140835096: mov rax, qword ptr [r8 + 0x1e0]
0x14083509d: not rcx
0x1408350a0: lea rcx, [rcx + rcx*2]
0x1408350a4: shl rcx, 5
0x1408350a8: cmp qword ptr [rax + rcx + 0x20], 0
0x1408350ae: je 0x1408354c0
0x1408350b4: add rax, rcx
0x1408350b7: mov rax, qword ptr [rax + 0x10]
0x1408350bb: mov rcx, qword ptr [rax]
0x1408350be: test rcx, rcx
0x1408350c1: js 0x1408350ca
0x1408350c3: cvtsi2ss xmm0, rcx
0x1408350c8: jmp 0x1408350df
0x1408350ca: mov rdx, rcx
0x1408350cd: shr rdx, 1
0x1408350d0: and ecx, 1
0x1408350d3: or rcx, rdx
0x1408350d6: cvtsi2ss xmm0, rcx
0x1408350db: addss xmm0, xmm0
0x1408350df: cvtsi2ss xmm1, dword ptr [rax + 8]
0x1408350e4: divss xmm1, dword ptr [rip + 0x2172554]
0x1408350ec: addss xmm1, xmm0
0x1408350f0: movss dword ptr [rbp + 0xde0], xmm1
0x1408350f8: mov byte ptr [rbp + 0xde4], 0
0x1408350ff: mov rax, qword ptr [rbp + 0xfb0]
0x140835106: mov rdi, qword ptr [rax + 0x98]
0x14083510d: mov r14, qword ptr [rax + 0xa0]
0x140835114: shl r14, 7
0x140835118: add r14, rdi
0x14083511b: lea r13, [rbp + 0xe08]
0x140835122: xorps xmm6, xmm6
0x140835125: lea r12, [rbp - 0x30]
0x140835129: cmp rdi, r14
0x14083512c: je 0x1408352b6
0x140835132: nop word ptr cs:[rax + rax]
0x140835140: mov rbx, qword ptr [rdi + 0x50]
0x140835144: mov r15, qword ptr [rdi + 0x58]
0x140835148: mov byte ptr [rbp + 0xf22], 1
0x14083514f: call 0x140be6750
0x140835154: mov esi, eax
0x140835156: mov byte ptr [rbp + 0xf22], 1
0x14083515d: xor ecx, ecx
0x14083515f: call 0x1400d9b20
0x140835164: test rax, rax
0x140835167: je 0x140835409
0x14083516d: movups xmm0, xmmword ptr [rax]
0x140835170: inc qword ptr [rax]
0x140835173: movups xmm1, xmmword ptr [rip + 0x22615b6]
0x14083517a: lea rax, [rbp + 0xdc0]
0x140835181: movups xmmword ptr [rax + 0x10], xmm1
0x140835185: movups xmm1, xmmword ptr [rip + 0x2261594]
0x14083518c: movups xmmword ptr [rax], xmm1
0x14083518f: mov qword ptr [rbp + 0xde8], 1
0x14083519a: mov qword ptr [rbp + 0xdf0], rbx
0x1408351a1: mov qword ptr [rbp + 0xdf8], 0
0x1408351ac: mov qword ptr [rbp + 0xe00], 4
0x1408351b7: movups xmmword ptr [r13], xmm6
0x1408351bc: mov qword ptr [rbp + 0xe18], 1
0x1408351c7: mov qword ptr [rbp + 0xe20], 0
0x1408351d2: movabs rax, 0x100000003
0x1408351dc: mov qword ptr [rbp + 0xe28], rax
0x1408351e3: mov rax, qword ptr [rbp + 0xdd8]
0x1408351ea: mov qword ptr [r13 + 0x68], rax
0x1408351ee: movups xmm1, xmmword ptr [rbp + 0xd98]
0x1408351f5: movups xmm2, xmmword ptr [rbp + 0xda8]
0x1408351fc: movups xmm3, xmmword ptr [rbp + 0xdb8]
0x140835203: movups xmm4, xmmword ptr [rbp + 0xdc8]
0x14083520a: movups xmmword ptr [r13 + 0x58], xmm4
0x14083520f: movups xmmword ptr [r13 + 0x48], xmm3
0x140835214: movups xmmword ptr [r13 + 0x38], xmm2
0x140835219: movups xmmword ptr [r13 + 0x28], xmm1
0x14083521e: movups xmmword ptr [rbp + 0xe78], xmm0
0x140835225: mov qword ptr [rbp + 0xe88], r15
0x14083522c: and esi, 0xffffff8f
0x14083522f: or esi, 0x20
0x140835232: mov dword ptr [rbp + 0xe90], esi
0x140835238: mov dword ptr [rbp + 0xe94], 0x3f800000
0x140835242: mov word ptr [rbp + 0xe98], 0
0x14083524b: mov byte ptr [rbp + 0xf22], 1
0x140835252: lea rcx, [rbp + 0x988]
0x140835259: lea rdx, [rbp + 0xde8]
0x140835260: lea r8, [rbp + 0x5a0]
0x140835267: xor r9d, r9d
0x14083526a: call 0x140905be0
0x14083526f: mov byte ptr [rbp + 0xf22], 1
0x140835276: mov rcx, r12
0x140835279: lea rdx, [rbp + 0x988]
0x140835280: lea r8, [rbp + 0xde0]
0x140835287: mov r9, rdi
0x14083528a: call 0x14090b460
0x14083528f: mov rax, qword ptr [rdi + 0x70]
0x140835293: mov byte ptr [rbp + 0xf22], 1
0x14083529a: mov rcx, r12
0x14083529d: mov rdx, rdi
0x1408352a0: mov r8, qword ptr [rbp + 0xf18]
0x1408352a7: call rax
0x1408352a9: sub rdi, -0x80
0x1408352ad: cmp rdi, r14
0x1408352b0: jne 0x140835140
0x1408352b6: lea rsi, [rbp + 0x988]
0x1408352bd: lea rdx, [rbp + 0x5a0]
0x1408352c4: mov r8d, 0x3e8
0x1408352ca: mov rcx, rsi
0x1408352cd: call 0x142923c00
0x1408352d2: mov byte ptr [rbp + 0xf22], 0
0x1408352d9: mov rcx, rsi
0x1408352dc: call 0x140868370
0x1408352e1: cmp byte ptr [rbp + 0xf17], 0
0x1408352e8: je 0x14083532d
0x1408352ea: call 0x140c18130
0x1408352ef: mov rcx, rax
0x1408352f2: mov rax, qword ptr [rax]
0x1408352f5: cmp rax, 1
0x1408352f9: jne 0x140835362
0x1408352fb: add rcx, 8
0x1408352ff: cmp qword ptr [rcx], 0
0x140835303: jne 0x140835382
0x140835305: mov qword ptr [rcx], 0xffffffffffffffff
0x14083530c: mov qword ptr [rbp + 0xf18], rcx
0x140835313: add rcx, 8
0x140835317: mov rdx, qword ptr [rbp + 0xd90]
0x14083531e: call 0x140c170f0
0x140835323: mov rax, qword ptr [rbp + 0xf18]
0x14083532a: inc qword ptr [rax]
0x14083532d: movaps xmm6, xmmword ptr [rbp + 0xf30]
0x140835334: add rsp, 0xfc8
0x14083533b: pop rbx
0x14083533c: pop rdi
0x14083533d: pop rsi
0x14083533e: pop r12
0x140835340: pop r13
0x140835342: pop r14
0x140835344: pop r15
0x140835346: pop rbp
0x140835347: ret 
0x140835348: test rax, rax
0x14083534b: jne 0x140835376
0x14083534d: xor edx, edx
0x14083534f: call 0x1428f87d0
0x140835354: mov rcx, rax
0x140835357: test rax, rax
0x14083535a: jne 0x140833df6
0x140835360: jmp 0x140835376
0x140835362: test rax, rax
0x140835365: jne 0x140835376
0x140835367: xor edx, edx
0x140835369: call 0x1428f87d0
0x14083536e: mov rcx, rax
0x140835371: test rax, rax
0x140835374: jne 0x1408352ff
0x140835376: lea rcx, [rip + 0x2405723]
0x14083537d: call 0x1428d80d0
0x140835382: lea rcx, [rip + 0x24057e7]
0x140835389: call 0x1428d9250
0x14083538e: mov byte ptr [rbp + 0xf27], 1
0x140835395: lea rcx, [rip + 0x2405704]
0x14083539c: call 0x1428d80d0
0x1408353a1: jmp 0x1408354fd
0x1408353a6: lea rcx, [rip + 0x225d713]
0x1408353ad: call 0x1428d9310
0x1408353b2: jmp 0x1408354fd
0x1408353b7: mov byte ptr [rbp + 0xf26], 1
0x1408353be: lea rcx, [rip + 0x24056db]
0x1408353c5: call 0x1428d80d0
0x1408353ca: jmp 0x1408354fd
0x1408353cf: lea rcx, [rip + 0x225d6d2]
0x1408353d6: call 0x1428d9310
0x1408353db: jmp 0x1408354fd
0x1408353e0: mov byte ptr [rbp + 0xf25], 1
0x1408353e7: lea rcx, [rip + 0x24056b2]
0x1408353ee: call 0x1428d80d0
0x1408353f3: jmp 0x1408354fd
0x1408353f8: lea rcx, [rip + 0x225d691]
0x1408353ff: call 0x1428d9310
0x140835404: jmp 0x1408354fd
0x140835409: mov byte ptr [rbp + 0xf22], 1
0x140835410: lea rcx, [rip + 0x2405689]
0x140835417: call 0x1428d80d0
0x14083541c: jmp 0x1408354fd
0x140835421: mov byte ptr [rbp + 0xf24], 1
0x140835428: lea rcx, [rip + 0x2405671]
0x14083542f: call 0x1428d80d0
0x140835434: jmp 0x1408354fd
0x140835439: lea rcx, [rip + 0x225d638]
0x140835440: call 0x1428d9310
0x140835445: jmp 0x1408354fd
0x14083544a: mov byte ptr [rbp + 0xf23], 1
0x140835451: lea rcx, [rip + 0x2405648]
0x140835458: call 0x1428d80d0
0x14083545d: jmp 0x1408354fd
0x140835462: lea rcx, [rip + 0x225d5f7]
0x140835469: call 0x1428d9310
0x14083546e: jmp 0x1408354fd
0x140835473: mov byte ptr [rbp + 0xf27], 1
0x14083547a: lea rcx, [rip + 0x2270757]
0x140835481: call 0x1428d9310
0x140835486: jmp 0x1408354fd
0x140835488: mov byte ptr [rbp + 0xf26], 1
0x14083548f: lea rcx, [rip + 0x2270742]
0x140835496: call 0x1428d9310
0x14083549b: jmp 0x1408354fd
0x14083549d: mov byte ptr [rbp + 0xf25], 1
0x1408354a4: lea rcx, [rip + 0x227072d]
0x1408354ab: call 0x1428d9310
0x1408354b0: jmp 0x1408354fd
0x1408354b2: lea rcx, [rip + 0x225d56f]
0x1408354b9: call 0x1428d9310
0x1408354be: jmp 0x1408354fd
0x1408354c0: mov byte ptr [rbp + 0xf22], 1
0x1408354c7: lea rcx, [rip + 0x227070a]
0x1408354ce: call 0x1428d9310
0x1408354d3: jmp 0x1408354fd
0x1408354d5: mov byte ptr [rbp + 0xf24], 1
0x1408354dc: lea rcx, [rip + 0x22706f5]
0x1408354e3: call 0x1428d9310
0x1408354e8: jmp 0x1408354fd
0x1408354ea: mov byte ptr [rbp + 0xf23], 1
0x1408354f1: lea rcx, [rip + 0x22706e0]
0x1408354f8: call 0x1428d9310
0x1408354fd: ud2 
0x1408354ff: nop 
0x140835500: mov qword ptr [rsp + 0x10], rdx
0x140835505: push rbp
0x140835506: push r15
0x140835508: push r14
0x14083550a: push r13
0x14083550c: push r12
0x14083550e: push rsi
0x14083550f: push rdi
0x140835510: push rbx
0x140835511: sub rsp, 0x58
0x140835515: lea rbp, [rdx + 0x80]
0x14083551c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140835521: mov rax, qword ptr [rbp + 0xf18]
0x140835528: inc qword ptr [rax]
0x14083552b: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140835530: add rsp, 0x58
0x140835534: pop rbx
0x140835535: pop rdi
0x140835536: pop rsi
0x140835537: pop r12
0x140835539: pop r13
0x14083553b: pop r14
0x14083553d: pop r15
0x14083553f: pop rbp
0x140835540: ret 
0x140835541: nop word ptr cs:[rax + rax]
0x140835550: mov qword ptr [rsp + 0x10], rdx
0x140835555: push rbp
0x140835556: push r15
0x140835558: push r14
0x14083555a: push r13
0x14083555c: push r12
0x14083555e: push rsi
0x14083555f: push rdi
0x140835560: push rbx
0x140835561: sub rsp, 0x58
0x140835565: lea rbp, [rdx + 0x80]
0x14083556c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140835571: cmp byte ptr [rbp + 0xf17], 0
0x140835578: je 0x1408355a9
0x14083557a: lea rax, [rbp + 0xd78]
0x140835581: mov qword ptr [rbp + 0xd88], rax
0x140835588: lea rax, [rbp + 0xd88]
0x14083558f: mov qword ptr [rbp + 0xd80], rax
0x140835596: lea rcx, [rip + 0x225cfeb]
0x14083559d: lea rdx, [rbp + 0xd80]
0x1408355a4: call 0x14132f2d0
0x1408355a9: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408355ae: add rsp, 0x58
0x1408355b2: pop rbx
0x1408355b3: pop rdi
0x1408355b4: pop rsi
0x1408355b5: pop r12
0x1408355b7: pop r13
0x1408355b9: pop r14
0x1408355bb: pop r15
0x1408355bd: pop rbp
0x1408355be: ret 
0x1408355bf: nop 
0x1408355c0: mov qword ptr [rsp + 0x10], rdx
0x1408355c5: push rbp
0x1408355c6: push r15
0x1408355c8: push r14
0x1408355ca: push r13
0x1408355cc: push r12
0x1408355ce: push rsi
0x1408355cf: push rdi
0x1408355d0: push rbx
0x1408355d1: sub rsp, 0x58
0x1408355d5: lea rbp, [rdx + 0x80]
0x1408355dc: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408355e1: mov rax, qword ptr [rbp + 0xf18]
0x1408355e8: inc qword ptr [rax]
0x1408355eb: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408355f0: add rsp, 0x58
0x1408355f4: pop rbx
0x1408355f5: pop rdi
0x1408355f6: pop rsi
0x1408355f7: pop r12
0x1408355f9: pop r13
0x1408355fb: pop r14
0x1408355fd: pop r15
0x1408355ff: pop rbp
0x140835600: ret 
0x140835601: nop word ptr cs:[rax + rax]
0x140835610: mov qword ptr [rsp + 0x10], rdx
0x140835615: push rbp
0x140835616: push r15
0x140835618: push r14
0x14083561a: push r13
0x14083561c: push r12
0x14083561e: push rsi
0x14083561f: push rdi
0x140835620: push rbx
0x140835621: sub rsp, 0x58
0x140835625: lea rbp, [rdx + 0x80]
0x14083562c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140835631: cmp byte ptr [rbp + 0xf22], 0
0x140835638: je 0x14083565c
0x14083563a: lea rcx, [rbp + 0x5a0]
0x140835641: call 0x1408923c0
0x140835646: movaps xmm6, xmmword ptr [rsp + 0x40]
0x14083564b: add rsp, 0x58
0x14083564f: pop rbx
0x140835650: pop rdi
0x140835651: pop rsi
0x140835652: pop r12
0x140835654: pop r13
0x140835656: pop r14
0x140835658: pop r15
0x14083565a: pop rbp
0x14083565b: ret 
0x14083565c: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140835661: add rsp, 0x58
0x140835665: pop rbx
0x140835666: pop rdi
0x140835667: pop rsi
0x140835668: pop r12
0x14083566a: pop r13
0x14083566c: pop r14
0x14083566e: pop r15
0x140835670: pop rbp
0x140835671: ret 
0x140835672: nop word ptr cs:[rax + rax]
0x140835680: mov qword ptr [rsp + 0x10], rdx
0x140835685: push rbp
0x140835686: push r15
0x140835688: push r14
0x14083568a: push r13
0x14083568c: push r12
0x14083568e: push rsi
0x14083568f: push rdi
0x140835690: push rbx
0x140835691: sub rsp, 0x58
0x140835695: lea rbp, [rdx + 0x80]
0x14083569c: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408356a1: lea rcx, [rbp + 0x988]
0x1408356a8: call 0x1408935c0
0x1408356ad: mov byte ptr [rbp + 0xf23], 1
0x1408356b4: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408356b9: add rsp, 0x58
0x1408356bd: pop rbx
0x1408356be: pop rdi
0x1408356bf: pop rsi
0x1408356c0: pop r12
0x1408356c2: pop r13
0x1408356c4: pop r14
0x1408356c6: pop r15
0x1408356c8: pop rbp
0x1408356c9: ret 
0x1408356ca: nop word ptr [rax + rax]
0x1408356d0: mov qword ptr [rsp + 0x10], rdx
0x1408356d5: push rbp
0x1408356d6: push r15
0x1408356d8: push r14
0x1408356da: push r13
0x1408356dc: push r12
0x1408356de: push rsi
0x1408356df: push rdi
0x1408356e0: push rbx
0x1408356e1: sub rsp, 0x58
0x1408356e5: lea rbp, [rdx + 0x80]
0x1408356ec: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408356f1: cmp byte ptr [rbp + 0xf23], 0
0x1408356f8: je 0x14083571c
0x1408356fa: lea rcx, [rbp + 0x5a0]
0x140835701: call 0x1408923c0
0x140835706: movaps xmm6, xmmword ptr [rsp + 0x40]
0x14083570b: add rsp, 0x58
0x14083570f: pop rbx
0x140835710: pop rdi
0x140835711: pop rsi
0x140835712: pop r12
0x140835714: pop r13
0x140835716: pop r14
0x140835718: pop r15
0x14083571a: pop rbp
0x14083571b: ret 
0x14083571c: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140835721: add rsp, 0x58
0x140835725: pop rbx
0x140835726: pop rdi
0x140835727: pop rsi
0x140835728: pop r12
0x14083572a: pop r13
0x14083572c: pop r14
0x14083572e: pop r15
0x140835730: pop rbp
0x140835731: ret 
0x140835732: nop word ptr cs:[rax + rax]
0x140835740: mov qword ptr [rsp + 0x10], rdx
0x140835745: push rbp
0x140835746: push r15
0x140835748: push r14
0x14083574a: push r13
0x14083574c: push r12
0x14083574e: push rsi
0x14083574f: push rdi
0x140835750: push rbx
0x140835751: sub rsp, 0x58
0x140835755: lea rbp, [rdx + 0x80]
0x14083575c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140835761: lea rcx, [rbp + 0x988]
0x140835768: call 0x1408935c0
0x14083576d: mov byte ptr [rbp + 0xf24], 1
0x140835774: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140835779: add rsp, 0x58
0x14083577d: pop rbx
0x14083577e: pop rdi
0x14083577f: pop rsi
0x140835780: pop r12
0x140835782: pop r13
0x140835784: pop r14
0x140835786: pop r15
0x140835788: pop rbp
0x140835789: ret 
0x14083578a: nop word ptr [rax + rax]
0x140835790: mov qword ptr [rsp + 0x10], rdx
0x140835795: push rbp
0x140835796: push r15
0x140835798: push r14
0x14083579a: push r13
0x14083579c: push r12
0x14083579e: push rsi
0x14083579f: push rdi
0x1408357a0: push rbx
0x1408357a1: sub rsp, 0x58
0x1408357a5: lea rbp, [rdx + 0x80]
0x1408357ac: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408357b1: cmp byte ptr [rbp + 0xf24], 0
0x1408357b8: je 0x1408357dc
0x1408357ba: lea rcx, [rbp + 0x5a0]
0x1408357c1: call 0x1408923c0
0x1408357c6: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408357cb: add rsp, 0x58
0x1408357cf: pop rbx
0x1408357d0: pop rdi
0x1408357d1: pop rsi
0x1408357d2: pop r12
0x1408357d4: pop r13
0x1408357d6: pop r14
0x1408357d8: pop r15
0x1408357da: pop rbp
0x1408357db: ret 
0x1408357dc: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408357e1: add rsp, 0x58
0x1408357e5: pop rbx
0x1408357e6: pop rdi
0x1408357e7: pop rsi
0x1408357e8: pop r12
0x1408357ea: pop r13
0x1408357ec: pop r14
0x1408357ee: pop r15
0x1408357f0: pop rbp
0x1408357f1: ret 
0x1408357f2: nop word ptr cs:[rax + rax]
0x140835800: mov qword ptr [rsp + 0x10], rdx
0x140835805: push rbp
0x140835806: push r15
0x140835808: push r14
0x14083580a: push r13
0x14083580c: push r12
0x14083580e: push rsi
0x14083580f: push rdi
0x140835810: push rbx
0x140835811: sub rsp, 0x58
0x140835815: lea rbp, [rdx + 0x80]
0x14083581c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140835821: lea rcx, [rbp + 0x988]
0x140835828: call 0x1408935c0
0x14083582d: mov byte ptr [rbp + 0xf25], 1
0x140835834: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140835839: add rsp, 0x58
0x14083583d: pop rbx
0x14083583e: pop rdi
0x14083583f: pop rsi
0x140835840: pop r12
0x140835842: pop r13
0x140835844: pop r14
0x140835846: pop r15
0x140835848: pop rbp
0x140835849: ret 
0x14083584a: nop word ptr [rax + rax]
0x140835850: mov qword ptr [rsp + 0x10], rdx
0x140835855: push rbp
0x140835856: push r15
0x140835858: push r14
0x14083585a: push r13
0x14083585c: push r12
0x14083585e: push rsi
0x14083585f: push rdi
0x140835860: push rbx
0x140835861: sub rsp, 0x58
0x140835865: lea rbp, [rdx + 0x80]
0x14083586c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140835871: cmp byte ptr [rbp + 0xf25], 0
0x140835878: je 0x14083589c
0x14083587a: lea rcx, [rbp + 0x5a0]
0x140835881: call 0x1408923c0
0x140835886: movaps xmm6, xmmword ptr [rsp + 0x40]
0x14083588b: add rsp, 0x58
0x14083588f: pop rbx
0x140835890: pop rdi
0x140835891: pop rsi
0x140835892: pop r12
0x140835894: pop r13
0x140835896: pop r14
0x140835898: pop r15
0x14083589a: pop rbp
0x14083589b: ret 
0x14083589c: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408358a1: add rsp, 0x58
0x1408358a5: pop rbx
0x1408358a6: pop rdi
0x1408358a7: pop rsi
0x1408358a8: pop r12
0x1408358aa: pop r13
0x1408358ac: pop r14
0x1408358ae: pop r15
0x1408358b0: pop rbp
0x1408358b1: ret 
0x1408358b2: nop word ptr cs:[rax + rax]
0x1408358c0: mov qword ptr [rsp + 0x10], rdx
0x1408358c5: push rbp
0x1408358c6: push r15
0x1408358c8: push r14
0x1408358ca: push r13
0x1408358cc: push r12
0x1408358ce: push rsi
0x1408358cf: push rdi
0x1408358d0: push rbx
0x1408358d1: sub rsp, 0x58
0x1408358d5: lea rbp, [rdx + 0x80]
0x1408358dc: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408358e1: lea rcx, [rbp + 0x988]
0x1408358e8: call 0x1408935c0
0x1408358ed: mov byte ptr [rbp + 0xf26], 1
0x1408358f4: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408358f9: add rsp, 0x58
0x1408358fd: pop rbx
0x1408358fe: pop rdi
0x1408358ff: pop rsi
0x140835900: pop r12
0x140835902: pop r13
0x140835904: pop r14
0x140835906: pop r15
0x140835908: pop rbp
0x140835909: ret 
0x14083590a: nop word ptr [rax + rax]
0x140835910: mov qword ptr [rsp + 0x10], rdx
0x140835915: push rbp
0x140835916: push r15
0x140835918: push r14
0x14083591a: push r13
0x14083591c: push r12
0x14083591e: push rsi
0x14083591f: push rdi
0x140835920: push rbx
0x140835921: sub rsp, 0x58
0x140835925: lea rbp, [rdx + 0x80]
0x14083592c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140835931: cmp byte ptr [rbp + 0xf26], 0
0x140835938: je 0x14083595c
0x14083593a: lea rcx, [rbp + 0x5a0]
0x140835941: call 0x1408923c0
0x140835946: movaps xmm6, xmmword ptr [rsp + 0x40]
0x14083594b: add rsp, 0x58
0x14083594f: pop rbx
0x140835950: pop rdi
0x140835951: pop rsi
0x140835952: pop r12
0x140835954: pop r13
0x140835956: pop r14
0x140835958: pop r15
0x14083595a: pop rbp
0x14083595b: ret 
0x14083595c: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140835961: add rsp, 0x58
0x140835965: pop rbx
0x140835966: pop rdi
0x140835967: pop rsi
0x140835968: pop r12
0x14083596a: pop r13
0x14083596c: pop r14
0x14083596e: pop r15
0x140835970: pop rbp
0x140835971: ret 
0x140835972: nop word ptr cs:[rax + rax]
0x140835980: mov qword ptr [rsp + 0x10], rdx
0x140835985: push rbp
0x140835986: push r15
0x140835988: push r14
0x14083598a: push r13
0x14083598c: push r12
0x14083598e: push rsi
0x14083598f: push rdi
0x140835990: push rbx
0x140835991: sub rsp, 0x58
0x140835995: lea rbp, [rdx + 0x80]
0x14083599c: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408359a1: lea rcx, [rbp + 0x988]
0x1408359a8: call 0x1408935c0
0x1408359ad: mov byte ptr [rbp + 0xf27], 1
0x1408359b4: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408359b9: add rsp, 0x58
0x1408359bd: pop rbx
0x1408359be: pop rdi
0x1408359bf: pop rsi
0x1408359c0: pop r12
0x1408359c2: pop r13
0x1408359c4: pop r14
0x1408359c6: pop r15
0x1408359c8: pop rbp
0x1408359c9: ret 
0x1408359ca: nop word ptr [rax + rax]
0x1408359d0: mov qword ptr [rsp + 0x10], rdx
0x1408359d5: push rbp
0x1408359d6: push r15
0x1408359d8: push r14
0x1408359da: push r13
0x1408359dc: push r12
0x1408359de: push rsi
0x1408359df: push rdi
0x1408359e0: push rbx
0x1408359e1: sub rsp, 0x58
0x1408359e5: lea rbp, [rdx + 0x80]
0x1408359ec: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408359f1: cmp byte ptr [rbp + 0xf27], 0
0x1408359f8: je 0x140835a1c
0x1408359fa: lea rcx, [rbp + 0x5a0]
0x140835a01: call 0x1408923c0
0x140835a06: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140835a0b: add rsp, 0x58
0x140835a0f: pop rbx
0x140835a10: pop rdi
0x140835a11: pop rsi
0x140835a12: pop r12
0x140835a14: pop r13
0x140835a16: pop r14
0x140835a18: pop r15
0x140835a1a: pop rbp
0x140835a1b: ret 
0x140835a1c: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140835a21: add rsp, 0x58
0x140835a25: pop rbx
0x140835a26: pop rdi
0x140835a27: pop rsi
0x140835a28: pop r12
0x140835a2a: pop r13
0x140835a2c: pop r14
0x140835a2e: pop r15
0x140835a30: pop rbp
0x140835a31: ret 
