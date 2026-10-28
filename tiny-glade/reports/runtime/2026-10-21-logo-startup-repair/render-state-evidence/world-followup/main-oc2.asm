0x140835a40: push rbp
0x140835a41: push r15
0x140835a43: push r14
0x140835a45: push r13
0x140835a47: push r12
0x140835a49: push rsi
0x140835a4a: push rdi
0x140835a4b: push rbx
0x140835a4c: sub rsp, 0xec8
0x140835a53: lea rbp, [rsp + 0x80]
0x140835a5b: movaps xmmword ptr [rbp + 0xe30], xmm6
0x140835a62: mov qword ptr [rbp + 0xe28], 0xfffffffffffffffe
0x140835a6d: mov qword ptr [rbp + 0xcf0], r9
0x140835a74: mov qword ptr [rbp + 0xe10], r8
0x140835a7b: mov r14, rdx
0x140835a7e: mov qword ptr [rbp + 0xe08], rcx
0x140835a85: call 0x140c16a00
0x140835a8a: test al, al
0x140835a8c: mov byte ptr [rbp + 0xe22], al
0x140835a92: je 0x140835af3
0x140835a94: lea r12, [rip + 0x225caa1]
0x140835a9b: mov edi, 0x49
0x140835aa0: mov edx, 0x49
0x140835aa5: mov rcx, r12
0x140835aa8: call 0x140832920
0x140835aad: cmp rax, 1
0x140835ab1: jne 0x140835b68
0x140835ab7: mov r9, rdx
0x140835aba: test rdx, rdx
0x140835abd: je 0x140835afc
0x140835abf: cmp r9, 0x49
0x140835ac3: jae 0x140835afa
0x140835ac5: lea rax, [rip + 0x225ca70]
0x140835acc: cmp byte ptr [r9 + rax], 0xbf
0x140835ad1: jg 0x140835afc
0x140835ad3: lea rax, [rip + 0x225cb2e]
0x140835ada: mov qword ptr [rsp + 0x20], rax
0x140835adf: lea rcx, [rip + 0x225ca56]
0x140835ae6: mov edx, 0x49
0x140835aeb: xor r8d, r8d
0x140835aee: call 0x1428d9940
0x140835af3: xor eax, eax
0x140835af5: jmp 0x140835cd4
0x140835afa: jne 0x140835ad3
0x140835afc: lea r12, [rip + 0x225ca39]
0x140835b03: mov rcx, r12
0x140835b06: mov rdx, r9
0x140835b09: call 0x140832920
0x140835b0e: cmp rax, 1
0x140835b12: jne 0x140835b68
0x140835b14: mov r8, rdx
0x140835b17: add r8, 2
0x140835b1b: je 0x140835b56
0x140835b1d: cmp r8, 0x49
0x140835b21: jae 0x140835b54
0x140835b23: lea rax, [rip + 0x225ca12]
0x140835b2a: cmp byte ptr [r8 + rax], 0xbf
0x140835b2f: jg 0x140835b56
0x140835b31: lea rax, [rip + 0x225cae8]
0x140835b38: mov qword ptr [rsp + 0x20], rax
0x140835b3d: lea rcx, [rip + 0x225c9f8]
0x140835b44: mov edx, 0x49
0x140835b49: mov r9d, 0x49
0x140835b4f: call 0x1428d9940
0x140835b54: jne 0x140835b31
0x140835b56: mov edi, 0x47
0x140835b5b: sub rdi, rdx
0x140835b5e: lea r12, [rip + 0x225c9d7]
0x140835b65: add r12, r8
0x140835b68: lea rax, [rip + 0x225ce8d]
0x140835b6f: lea r13, [rip + 0x225ce42]
0x140835b76: nop word ptr cs:[rax + rax]
0x140835b80: movsx ecx, byte ptr [rax - 1]
0x140835b84: test ecx, ecx
0x140835b86: js 0x140835ba0
0x140835b88: dec rax
0x140835b8b: cmp ecx, 0x5c
0x140835b8e: jne 0x140835bf9
0x140835b90: jmp 0x140835c0e
0x140835b92: nop word ptr cs:[rax + rax]
0x140835ba0: movzx edx, byte ptr [rax - 2]
0x140835ba4: cmp dl, 0xc0
0x140835ba7: jge 0x140835bce
0x140835ba9: movzx r8d, byte ptr [rax - 3]
0x140835bae: cmp r8b, 0xc0
0x140835bb2: jge 0x140835bd7
0x140835bb4: movzx r9d, byte ptr [rax - 4]
0x140835bb9: add rax, -4
0x140835bbd: and r9d, 7
0x140835bc1: shl r9d, 6
0x140835bc5: and r8d, 0x3f
0x140835bc9: or r8d, r9d
0x140835bcc: jmp 0x140835bdf
0x140835bce: add rax, -2
0x140835bd2: and edx, 0x1f
0x140835bd5: jmp 0x140835be9
0x140835bd7: add rax, -3
0x140835bdb: and r8d, 0xf
0x140835bdf: shl r8d, 6
0x140835be3: and edx, 0x3f
0x140835be6: or edx, r8d
0x140835be9: shl edx, 6
0x140835bec: and cl, 0x3f
0x140835bef: movzx ecx, cl
0x140835bf2: or ecx, edx
0x140835bf4: cmp ecx, 0x5c
0x140835bf7: je 0x140835c0e
0x140835bf9: cmp ecx, 0x2f
0x140835bfc: je 0x140835c0e
0x140835bfe: cmp rax, r13
0x140835c01: jne 0x140835b80
0x140835c07: mov esi, 0x44
0x140835c0c: jmp 0x140835c5d
0x140835c0e: lea r13, [rip + 0x225cda3]
0x140835c15: sub rax, r13
0x140835c18: mov r8, rax
0x140835c1b: inc r8
0x140835c1e: je 0x140835c52
0x140835c20: cmp r8, 0x44
0x140835c24: jae 0x140835c50
0x140835c26: cmp byte ptr [r8 + r13], 0xbf
0x140835c2b: jg 0x140835c52
0x140835c2d: lea rax, [rip + 0x225c9b4]
0x140835c34: mov qword ptr [rsp + 0x20], rax
0x140835c39: lea rcx, [rip + 0x225cd78]
0x140835c40: mov edx, 0x44
0x140835c45: mov r9d, 0x44
0x140835c4b: call 0x1428d9940
0x140835c50: jne 0x140835c2d
0x140835c52: mov esi, 0x43
0x140835c57: sub rsi, rax
0x140835c5a: add r13, r8
0x140835c5d: call 0x140c18130
0x140835c62: mov rcx, rax
0x140835c65: mov rax, qword ptr [rax]
0x140835c68: cmp rax, 1
0x140835c6c: jne 0x140836dc8
0x140835c72: add rcx, 8
0x140835c76: cmp qword ptr [rcx], 0
0x140835c7a: jne 0x140836e02
0x140835c80: mov qword ptr [rcx], 0xffffffffffffffff
0x140835c87: mov qword ptr [rbp + 0xe18], rcx
0x140835c8e: add rcx, 8
0x140835c92: mov qword ptr [rsp + 0x20], rsi
0x140835c97: mov qword ptr [rsp + 0x30], 0
0x140835ca0: mov qword ptr [rsp + 0x28], 1
0x140835ca9: mov rdx, r12
0x140835cac: mov r8, rdi
0x140835caf: mov r9, r13
0x140835cb2: call 0x140c16d40
0x140835cb7: mov rcx, qword ptr [rbp + 0xe18]
0x140835cbe: inc qword ptr [rcx]
0x140835cc1: mov qword ptr [rbp + 0xce0], rax
0x140835cc8: mov qword ptr [rbp + 0xc80], rax
0x140835ccf: mov eax, 1
0x140835cd4: mov qword ptr [rbp + 0xc78], rax
0x140835cdb: mov rax, qword ptr [rbp + 0xe10]
0x140835ce2: lea rcx, [rax + 0x28]
0x140835ce6: mov qword ptr [rbp + 0xdf0], rcx
0x140835ced: call 0x140bd5790
0x140835cf2: mov rcx, qword ptr [rbp + 0xe10]
0x140835cf9: lea rdx, [rcx + 0x24]
0x140835cfd: mov dword ptr [rbp + 0x890], 0
0x140835d07: mov qword ptr [rbp + 0x898], rcx
0x140835d0e: mov qword ptr [rbp + 0xcf8], 0
0x140835d19: mov qword ptr [rbp + 0xdf8], rdx
0x140835d20: mov qword ptr [rbp + 0xd00], rdx
0x140835d27: mov qword ptr [rbp + 0xd08], 0
0x140835d32: mov rcx, qword ptr [rbp + 0xe08]
0x140835d39: mov qword ptr [rsp + 0x40], rcx
0x140835d3e: mov qword ptr [rsp + 0x38], r14
0x140835d43: mov qword ptr [rsp + 0x30], rax
0x140835d48: lea rax, [rbp + 0xcf8]
0x140835d4f: mov qword ptr [rsp + 0x28], rax
0x140835d54: mov qword ptr [rsp + 0x20], 1
0x140835d5d: lea rdx, [rip + 0x225cd74]
0x140835d64: lea rcx, [rbp + 0x4a8]
0x140835d6b: lea r9, [rbp + 0x890]
0x140835d72: mov r8d, 0xd
0x140835d78: call 0x140867970
0x140835d7d: mov qword ptr [rbp + 0xe18], r14
0x140835d84: lea rcx, [r14 + 0x38]
0x140835d88: mov byte ptr [rbp + 0xe27], 1
0x140835d8f: movabs rdx, 0x4bc362195b80f729
0x140835d99: movabs r8, 0x982798ce80cbcefb
0x140835da3: mov qword ptr [rbp + 0xe00], rcx
0x140835daa: call 0x1400dd6f0
0x140835daf: test rax, rax
0x140835db2: mov r8, qword ptr [rbp + 0xe18]
0x140835db9: je 0x140836ec4
0x140835dbf: cmp qword ptr [r8 + 0x218], rdx
0x140835dc6: jbe 0x140836ec4
0x140835dcc: mov rax, qword ptr [r8 + 0x210]
0x140835dd3: mov rcx, qword ptr [rax + rdx*8]
0x140835dd7: test rcx, rcx
0x140835dda: je 0x140836ec4
0x140835de0: mov rax, qword ptr [r8 + 0x1e0]
0x140835de7: not rcx
0x140835dea: lea rcx, [rcx + rcx*2]
0x140835dee: shl rcx, 5
0x140835df2: cmp qword ptr [rax + rcx + 0x20], 0
0x140835df8: je 0x140836ec4
0x140835dfe: add rax, rcx
0x140835e01: mov rax, qword ptr [rax + 0x10]
0x140835e05: mov rcx, qword ptr [rax]
0x140835e08: test rcx, rcx
0x140835e0b: js 0x140835e14
0x140835e0d: cvtsi2ss xmm0, rcx
0x140835e12: jmp 0x140835e29
0x140835e14: mov rdx, rcx
0x140835e17: shr rdx, 1
0x140835e1a: and ecx, 1
0x140835e1d: or rcx, rdx
0x140835e20: cvtsi2ss xmm0, rcx
0x140835e25: addss xmm0, xmm0
0x140835e29: cvtsi2ss xmm1, dword ptr [rax + 8]
0x140835e2e: divss xmm1, dword ptr [rip + 0x217180a]
0x140835e36: addss xmm1, xmm0
0x140835e3a: movss dword ptr [rbp + 0xc98], xmm1
0x140835e42: mov byte ptr [rbp + 0xc9c], 0
0x140835e49: mov rax, qword ptr [rbp + 0xeb0]
0x140835e50: mov r15, qword ptr [rax + 0xb0]
0x140835e57: mov r14, qword ptr [rax + 0xb8]
0x140835e5e: shl r14, 7
0x140835e62: add r14, r15
0x140835e65: lea rbx, [rbp + 0x890]
0x140835e6c: lea rdi, [rbp + 0xcf8]
0x140835e73: cmp r15, r14
0x140835e76: je 0x14083600b
0x140835e7c: nop dword ptr [rax]
0x140835e80: mov r12, qword ptr [r15 + 0x50]
0x140835e84: mov r13, qword ptr [r15 + 0x58]
0x140835e88: mov byte ptr [rbp + 0xe27], 1
0x140835e8f: call 0x140be6750
0x140835e94: mov esi, eax
0x140835e96: mov byte ptr [rbp + 0xe27], 1
0x140835e9d: xor ecx, ecx
0x140835e9f: call 0x1400d9b20
0x140835ea4: test rax, rax
0x140835ea7: je 0x140836e0e
0x140835ead: mov dword ptr [rbp + 0xda0], esi
0x140835eb3: mov word ptr [rbp + 0xda8], 0
0x140835ebc: mov dword ptr [rbp + 0xda4], 0x3f800000
0x140835ec6: mov qword ptr [rbp + 0xcf8], 1
0x140835ed1: mov qword ptr [rbp + 0xd00], r12
0x140835ed8: mov qword ptr [rbp + 0xd98], r13
0x140835edf: mov qword ptr [rbp + 0xd08], 0
0x140835eea: mov qword ptr [rbp + 0xd10], 4
0x140835ef5: mov qword ptr [rbp + 0xd18], 0
0x140835f00: movups xmm0, xmmword ptr [rax]
0x140835f03: inc qword ptr [rax]
0x140835f06: movups xmm1, xmmword ptr [rip + 0x2260823]
0x140835f0d: lea rax, [rbp + 0xd68]
0x140835f14: movups xmmword ptr [rax + 0x10], xmm1
0x140835f18: movups xmm1, xmmword ptr [rip + 0x2260801]
0x140835f1f: movups xmmword ptr [rax], xmm1
0x140835f22: movups xmmword ptr [rbp + 0xd88], xmm0
0x140835f29: movabs rax, 0x100000003
0x140835f33: mov qword ptr [rbp + 0xd38], rax
0x140835f3a: mov qword ptr [rbp + 0xd20], 0
0x140835f45: mov qword ptr [rbp + 0xd28], 1
0x140835f50: mov qword ptr [rbp + 0xd30], 0
0x140835f5b: mov byte ptr [rbp + 0xe27], 1
0x140835f62: mov rcx, rbx
0x140835f65: mov rdx, rdi
0x140835f68: lea r8, [rbp + 0x4a8]
0x140835f6f: xor r9d, r9d
0x140835f72: call 0x140905be0
0x140835f77: cmp byte ptr [r15 + 0x64], 0
0x140835f7c: je 0x140836e26
0x140835f82: movzx eax, byte ptr [r15 + 0x65]
0x140835f87: mov byte ptr [rbp + 0x941], al
0x140835f8d: mov r8d, 0xf8
0x140835f93: mov rcx, rdi
0x140835f96: mov rdx, rbx
0x140835f99: call 0x142923c00
0x140835f9e: mov byte ptr [rbp + 0xe27], 1
0x140835fa5: lea rcx, [rbp + 0x3b0]
0x140835fac: mov rdx, rdi
0x140835faf: lea r8, [rbp + 0xc98]
0x140835fb6: mov r9, r15
0x140835fb9: call 0x14090b460
0x140835fbe: mov rsi, qword ptr [r15 + 0x70]
0x140835fc2: movzx eax, byte ptr [rbp + 0x459]
0x140835fc9: and al, 0x8e
0x140835fcb: or al, 0x21
0x140835fcd: mov byte ptr [rbp + 0x459], al
0x140835fd3: mov r8d, 0xf8
0x140835fd9: mov rcx, rbx
0x140835fdc: lea rdx, [rbp + 0x3b0]
0x140835fe3: call 0x142923c00
0x140835fe8: mov byte ptr [rbp + 0xe27], 1
0x140835fef: mov rcx, rbx
0x140835ff2: mov rdx, r15
0x140835ff5: mov r8, qword ptr [rbp + 0xe18]
0x140835ffc: call rsi
0x140835ffe: sub r15, -0x80
0x140836002: cmp r15, r14
0x140836005: jne 0x140835e80
0x14083600b: lea rsi, [rbp + 0x890]
0x140836012: lea rdx, [rbp + 0x4a8]
0x140836019: mov r8d, 0x3e8
0x14083601f: mov rcx, rsi
0x140836022: call 0x142923c00
0x140836027: mov byte ptr [rbp + 0xe27], 0
0x14083602e: mov rcx, rsi
0x140836031: call 0x140868370
0x140836036: mov rcx, qword ptr [rbp + 0xdf0]
0x14083603d: call 0x140bd5790
0x140836042: mov rcx, qword ptr [rbp + 0xe18]
0x140836049: mov dword ptr [rbp + 0x890], 0
0x140836053: mov rdx, qword ptr [rbp + 0xe10]
0x14083605a: mov qword ptr [rbp + 0x898], rdx
0x140836061: mov dword ptr [rbp + 0x8b0], 0
0x14083606b: mov rdx, qword ptr [rbp + 0xcf0]
0x140836072: mov qword ptr [rbp + 0x8b8], rdx
0x140836079: mov qword ptr [rbp + 0xcf8], 0
0x140836084: mov rdx, qword ptr [rbp + 0xdf8]
0x14083608b: mov qword ptr [rbp + 0xd00], rdx
0x140836092: mov qword ptr [rbp + 0xd08], 0
0x14083609d: mov rdx, qword ptr [rbp + 0xe08]
0x1408360a4: mov qword ptr [rsp + 0x40], rdx
0x1408360a9: mov qword ptr [rsp + 0x38], rcx
0x1408360ae: mov qword ptr [rsp + 0x30], rax
0x1408360b3: mov qword ptr [rsp + 0x28], rdi
0x1408360b8: mov qword ptr [rsp + 0x20], 2
0x1408360c1: lea rdx, [rip + 0x225ca1d]
0x1408360c8: lea rcx, [rbp + 0x4a8]
0x1408360cf: lea r9, [rbp + 0x890]
0x1408360d6: mov r8d, 0x16
0x1408360dc: call 0x140867970
0x1408360e1: mov byte ptr [rbp + 0xe26], 1
0x1408360e8: movabs rdx, 0x4bc362195b80f729
0x1408360f2: movabs r8, 0x982798ce80cbcefb
0x1408360fc: mov rcx, qword ptr [rbp + 0xe00]
0x140836103: call 0x1400dd6f0
0x140836108: test rax, rax
0x14083610b: mov r8, qword ptr [rbp + 0xe18]
0x140836112: je 0x140836ed9
0x140836118: cmp qword ptr [r8 + 0x218], rdx
0x14083611f: jbe 0x140836ed9
0x140836125: mov rax, qword ptr [r8 + 0x210]
0x14083612c: mov rcx, qword ptr [rax + rdx*8]
0x140836130: test rcx, rcx
0x140836133: je 0x140836ed9
0x140836139: mov rax, qword ptr [r8 + 0x1e0]
0x140836140: not rcx
0x140836143: lea rcx, [rcx + rcx*2]
0x140836147: shl rcx, 5
0x14083614b: cmp qword ptr [rax + rcx + 0x20], 0
0x140836151: je 0x140836ed9
0x140836157: add rax, rcx
0x14083615a: mov rax, qword ptr [rax + 0x10]
0x14083615e: mov rcx, qword ptr [rax]
0x140836161: test rcx, rcx
0x140836164: js 0x14083616d
0x140836166: cvtsi2ss xmm0, rcx
0x14083616b: jmp 0x140836182
0x14083616d: mov rdx, rcx
0x140836170: shr rdx, 1
0x140836173: and ecx, 1
0x140836176: or rcx, rdx
0x140836179: cvtsi2ss xmm0, rcx
0x14083617e: addss xmm0, xmm0
0x140836182: cvtsi2ss xmm1, dword ptr [rax + 8]
0x140836187: divss xmm1, dword ptr [rip + 0x21714b1]
0x14083618f: addss xmm1, xmm0
0x140836193: movss dword ptr [rbp + 0xc98], xmm1
0x14083619b: mov byte ptr [rbp + 0xc9c], 0
0x1408361a2: mov rax, qword ptr [rbp + 0xeb0]
0x1408361a9: mov r15, qword ptr [rax + 0xc8]
0x1408361b0: mov r14, qword ptr [rax + 0xd0]
0x1408361b7: shl r14, 7
0x1408361bb: add r14, r15
0x1408361be: lea rbx, [rbp + 0x890]
0x1408361c5: lea rdi, [rbp + 0xcf8]
0x1408361cc: cmp r15, r14
0x1408361cf: je 0x14083636b
0x1408361d5: nop word ptr cs:[rax + rax]
0x1408361e0: mov r12, qword ptr [r15 + 0x50]
0x1408361e4: mov r13, qword ptr [r15 + 0x58]
0x1408361e8: mov byte ptr [rbp + 0xe26], 1
0x1408361ef: call 0x140be6750
0x1408361f4: mov esi, eax
0x1408361f6: mov byte ptr [rbp + 0xe26], 1
0x1408361fd: xor ecx, ecx
0x1408361ff: call 0x1400d9b20
0x140836204: test rax, rax
0x140836207: je 0x140836e37
0x14083620d: mov dword ptr [rbp + 0xda0], esi
0x140836213: mov word ptr [rbp + 0xda8], 0
0x14083621c: mov dword ptr [rbp + 0xda4], 0x3f800000
0x140836226: mov qword ptr [rbp + 0xcf8], 1
0x140836231: mov qword ptr [rbp + 0xd00], r12
0x140836238: mov qword ptr [rbp + 0xd98], r13
0x14083623f: mov qword ptr [rbp + 0xd08], 0
0x14083624a: mov qword ptr [rbp + 0xd10], 4
0x140836255: mov qword ptr [rbp + 0xd18], 0
0x140836260: movups xmm0, xmmword ptr [rax]
0x140836263: inc qword ptr [rax]
0x140836266: movups xmm1, xmmword ptr [rip + 0x22604c3]
0x14083626d: lea rax, [rbp + 0xd68]
0x140836274: movups xmmword ptr [rax + 0x10], xmm1
0x140836278: movups xmm1, xmmword ptr [rip + 0x22604a1]
0x14083627f: movups xmmword ptr [rax], xmm1
0x140836282: movups xmmword ptr [rbp + 0xd88], xmm0
0x140836289: movabs rax, 0x100000003
0x140836293: mov qword ptr [rbp + 0xd38], rax
0x14083629a: mov qword ptr [rbp + 0xd20], 0
0x1408362a5: mov qword ptr [rbp + 0xd28], 1
0x1408362b0: mov qword ptr [rbp + 0xd30], 0
0x1408362bb: mov byte ptr [rbp + 0xe26], 1
0x1408362c2: mov rcx, rbx
0x1408362c5: mov rdx, rdi
0x1408362c8: lea r8, [rbp + 0x4a8]
0x1408362cf: xor r9d, r9d
0x1408362d2: call 0x140905be0
0x1408362d7: cmp byte ptr [r15 + 0x64], 0
0x1408362dc: je 0x140836e4f
0x1408362e2: movzx eax, byte ptr [r15 + 0x65]
0x1408362e7: mov byte ptr [rbp + 0x941], al
0x1408362ed: mov r8d, 0xf8
0x1408362f3: mov rcx, rdi
0x1408362f6: mov rdx, rbx
0x1408362f9: call 0x142923c00
0x1408362fe: mov byte ptr [rbp + 0xe26], 1
0x140836305: lea rcx, [rbp + 0x2b8]
0x14083630c: mov rdx, rdi
0x14083630f: lea r8, [rbp + 0xc98]
0x140836316: mov r9, r15
0x140836319: call 0x14090b460
0x14083631e: mov rsi, qword ptr [r15 + 0x70]
0x140836322: movzx eax, byte ptr [rbp + 0x361]
0x140836329: and al, 0x8e
0x14083632b: or al, 0x21
0x14083632d: mov byte ptr [rbp + 0x361], al
0x140836333: mov r8d, 0xf8
0x140836339: mov rcx, rbx
0x14083633c: lea rdx, [rbp + 0x2b8]
0x140836343: call 0x142923c00
0x140836348: mov byte ptr [rbp + 0xe26], 1
0x14083634f: mov rcx, rbx
0x140836352: mov rdx, r15
0x140836355: mov r8, qword ptr [rbp + 0xe18]
0x14083635c: call rsi
0x14083635e: sub r15, -0x80
0x140836362: cmp r15, r14
0x140836365: jne 0x1408361e0
0x14083636b: lea rsi, [rbp + 0x890]
0x140836372: lea rdx, [rbp + 0x4a8]
0x140836379: mov r8d, 0x3e8
0x14083637f: mov rcx, rsi
0x140836382: call 0x142923c00
0x140836387: mov byte ptr [rbp + 0xe26], 0
0x14083638e: mov rcx, rsi
0x140836391: call 0x140868370
0x140836396: mov rcx, qword ptr [rbp + 0xdf0]
0x14083639d: call 0x140bd5790
0x1408363a2: mov rcx, qword ptr [rbp + 0xe18]
0x1408363a9: mov dword ptr [rbp + 0x890], 0
0x1408363b3: mov rdx, qword ptr [rbp + 0xe10]
0x1408363ba: mov qword ptr [rbp + 0x898], rdx
0x1408363c1: add rdx, 0x48
0x1408363c5: mov dword ptr [rbp + 0x8b0], 0
0x1408363cf: mov qword ptr [rbp + 0xe10], rdx
0x1408363d6: mov qword ptr [rbp + 0x8b8], rdx
0x1408363dd: mov qword ptr [rbp + 0xcf8], 0
0x1408363e8: mov rdx, qword ptr [rbp + 0xdf8]
0x1408363ef: mov qword ptr [rbp + 0xd00], rdx
0x1408363f6: mov qword ptr [rbp + 0xd08], 0
0x140836401: mov rdx, qword ptr [rbp + 0xe08]
0x140836408: mov qword ptr [rsp + 0x40], rdx
0x14083640d: mov qword ptr [rsp + 0x38], rcx
0x140836412: mov qword ptr [rsp + 0x30], rax
0x140836417: mov qword ptr [rsp + 0x28], rdi
0x14083641c: mov qword ptr [rsp + 0x20], 2
0x140836425: lea rdx, [rip + 0x225c6cf]
0x14083642c: lea rcx, [rbp + 0x4a8]
0x140836433: lea r9, [rbp + 0x890]
0x14083643a: mov r8d, 0x16
0x140836440: call 0x140867970
0x140836445: mov byte ptr [rbp + 0xe25], 1
0x14083644c: movabs rdx, 0x4bc362195b80f729
0x140836456: movabs r8, 0x982798ce80cbcefb
0x140836460: mov rcx, qword ptr [rbp + 0xe00]
0x140836467: call 0x1400dd6f0
0x14083646c: test rax, rax
0x14083646f: mov r8, qword ptr [rbp + 0xe18]
0x140836476: je 0x140836eee
0x14083647c: cmp qword ptr [r8 + 0x218], rdx
0x140836483: jbe 0x140836eee
0x140836489: mov rax, qword ptr [r8 + 0x210]
0x140836490: mov rcx, qword ptr [rax + rdx*8]
0x140836494: test rcx, rcx
0x140836497: je 0x140836eee
0x14083649d: mov rax, qword ptr [r8 + 0x1e0]
0x1408364a4: not rcx
0x1408364a7: lea rcx, [rcx + rcx*2]
0x1408364ab: shl rcx, 5
0x1408364af: cmp qword ptr [rax + rcx + 0x20], 0
0x1408364b5: je 0x140836eee
0x1408364bb: add rax, rcx
0x1408364be: mov rax, qword ptr [rax + 0x10]
0x1408364c2: mov rcx, qword ptr [rax]
0x1408364c5: test rcx, rcx
0x1408364c8: js 0x1408364d1
0x1408364ca: cvtsi2ss xmm0, rcx
0x1408364cf: jmp 0x1408364e6
0x1408364d1: mov rdx, rcx
0x1408364d4: shr rdx, 1
0x1408364d7: and ecx, 1
0x1408364da: or rcx, rdx
0x1408364dd: cvtsi2ss xmm0, rcx
0x1408364e2: addss xmm0, xmm0
0x1408364e6: cvtsi2ss xmm1, dword ptr [rax + 8]
0x1408364eb: divss xmm1, dword ptr [rip + 0x217114d]
0x1408364f3: addss xmm1, xmm0
0x1408364f7: movss dword ptr [rbp + 0xc98], xmm1
0x1408364ff: mov byte ptr [rbp + 0xc9c], 0
0x140836506: mov rax, qword ptr [rbp + 0xeb0]
0x14083650d: mov r15, qword ptr [rax + 0xe0]
0x140836514: mov r14, qword ptr [rax + 0xe8]
0x14083651b: shl r14, 7
0x14083651f: add r14, r15
0x140836522: lea rbx, [rbp + 0x890]
0x140836529: lea rdi, [rbp + 0xcf8]
0x140836530: cmp r15, r14
0x140836533: je 0x1408366cb
0x140836539: nop dword ptr [rax]
0x140836540: mov r12, qword ptr [r15 + 0x50]
0x140836544: mov r13, qword ptr [r15 + 0x58]
0x140836548: mov byte ptr [rbp + 0xe25], 1
0x14083654f: call 0x140be6750
0x140836554: mov esi, eax
0x140836556: mov byte ptr [rbp + 0xe25], 1
0x14083655d: xor ecx, ecx
0x14083655f: call 0x1400d9b20
0x140836564: test rax, rax
0x140836567: je 0x140836e60
0x14083656d: mov dword ptr [rbp + 0xda0], esi
0x140836573: mov word ptr [rbp + 0xda8], 0
0x14083657c: mov dword ptr [rbp + 0xda4], 0x3f800000
0x140836586: mov qword ptr [rbp + 0xcf8], 1
0x140836591: mov qword ptr [rbp + 0xd00], r12
0x140836598: mov qword ptr [rbp + 0xd98], r13
0x14083659f: mov qword ptr [rbp + 0xd08], 0
0x1408365aa: mov qword ptr [rbp + 0xd10], 4
0x1408365b5: mov qword ptr [rbp + 0xd18], 0
0x1408365c0: movups xmm0, xmmword ptr [rax]
0x1408365c3: inc qword ptr [rax]
0x1408365c6: movups xmm1, xmmword ptr [rip + 0x2260163]
0x1408365cd: lea rax, [rbp + 0xd68]
0x1408365d4: movups xmmword ptr [rax + 0x10], xmm1
0x1408365d8: movups xmm1, xmmword ptr [rip + 0x2260141]
0x1408365df: movups xmmword ptr [rax], xmm1
0x1408365e2: movups xmmword ptr [rbp + 0xd88], xmm0
0x1408365e9: movabs rax, 0x100000003
0x1408365f3: mov qword ptr [rbp + 0xd38], rax
0x1408365fa: mov qword ptr [rbp + 0xd20], 0
0x140836605: mov qword ptr [rbp + 0xd28], 1
0x140836610: mov qword ptr [rbp + 0xd30], 0
0x14083661b: mov byte ptr [rbp + 0xe25], 1
0x140836622: mov rcx, rbx
0x140836625: mov rdx, rdi
0x140836628: lea r8, [rbp + 0x4a8]
0x14083662f: xor r9d, r9d
0x140836632: call 0x140905be0
0x140836637: cmp byte ptr [r15 + 0x64], 0
0x14083663c: je 0x140836e78
0x140836642: movzx eax, byte ptr [r15 + 0x65]
0x140836647: mov byte ptr [rbp + 0x941], al
0x14083664d: mov r8d, 0xf8
0x140836653: mov rcx, rdi
0x140836656: mov rdx, rbx
0x140836659: call 0x142923c00
0x14083665e: mov byte ptr [rbp + 0xe25], 1
0x140836665: lea rcx, [rbp + 0x1c0]
0x14083666c: mov rdx, rdi
0x14083666f: lea r8, [rbp + 0xc98]
0x140836676: mov r9, r15
0x140836679: call 0x14090b460
0x14083667e: mov rsi, qword ptr [r15 + 0x70]
0x140836682: movzx eax, byte ptr [rbp + 0x269]
0x140836689: and al, 0x8e
0x14083668b: or al, 0x21
0x14083668d: mov byte ptr [rbp + 0x269], al
0x140836693: mov r8d, 0xf8
0x140836699: mov rcx, rbx
0x14083669c: lea rdx, [rbp + 0x1c0]
0x1408366a3: call 0x142923c00
0x1408366a8: mov byte ptr [rbp + 0xe25], 1
0x1408366af: mov rcx, rbx
0x1408366b2: mov rdx, r15
0x1408366b5: mov r8, qword ptr [rbp + 0xe18]
0x1408366bc: call rsi
0x1408366be: sub r15, -0x80
0x1408366c2: cmp r15, r14
0x1408366c5: jne 0x140836540
0x1408366cb: lea rsi, [rbp + 0x890]
0x1408366d2: lea rdx, [rbp + 0x4a8]
0x1408366d9: mov r8d, 0x3e8
0x1408366df: mov rcx, rsi
0x1408366e2: call 0x142923c00
0x1408366e7: mov byte ptr [rbp + 0xe25], 0
0x1408366ee: mov rcx, rsi
0x1408366f1: call 0x140868370
0x1408366f6: mov rcx, qword ptr [rbp + 0xdf0]
0x1408366fd: call 0x140bd5790
0x140836702: mov rcx, qword ptr [rbp + 0xe18]
0x140836709: mov dword ptr [rbp + 0x890], 0
0x140836713: mov rdx, qword ptr [rbp + 0xcf0]
0x14083671a: mov qword ptr [rbp + 0x898], rdx
0x140836721: mov qword ptr [rbp + 0xcf8], 0
0x14083672c: mov rdx, qword ptr [rbp + 0xdf8]
0x140836733: mov qword ptr [rbp + 0xd00], rdx
0x14083673a: mov qword ptr [rbp + 0xd08], 0
0x140836745: mov rdx, qword ptr [rbp + 0xe08]
0x14083674c: mov qword ptr [rsp + 0x40], rdx
0x140836751: mov qword ptr [rsp + 0x38], rcx
0x140836756: mov qword ptr [rsp + 0x30], rax
0x14083675b: mov qword ptr [rsp + 0x28], rdi
0x140836760: mov qword ptr [rsp + 0x20], 1
0x140836769: lea rdx, [rip + 0x225c3a1]
0x140836770: lea rcx, [rbp + 0x4a8]
0x140836777: lea r9, [rbp + 0x890]
0x14083677e: mov r8d, 0xc
0x140836784: call 0x140867970
0x140836789: mov byte ptr [rbp + 0xe24], 1
0x140836790: movabs rdx, 0x4bc362195b80f729
0x14083679a: movabs r8, 0x982798ce80cbcefb
0x1408367a4: mov rcx, qword ptr [rbp + 0xe00]
0x1408367ab: call 0x1400dd6f0
0x1408367b0: test rax, rax
0x1408367b3: mov r8, qword ptr [rbp + 0xe18]
0x1408367ba: je 0x140836f03
0x1408367c0: cmp qword ptr [r8 + 0x218], rdx
0x1408367c7: jbe 0x140836f03
0x1408367cd: mov rax, qword ptr [r8 + 0x210]
0x1408367d4: mov rcx, qword ptr [rax + rdx*8]
0x1408367d8: test rcx, rcx
0x1408367db: je 0x140836f03
0x1408367e1: mov rax, qword ptr [r8 + 0x1e0]
0x1408367e8: not rcx
0x1408367eb: lea rcx, [rcx + rcx*2]
0x1408367ef: shl rcx, 5
0x1408367f3: cmp qword ptr [rax + rcx + 0x20], 0
0x1408367f9: je 0x140836f03
0x1408367ff: add rax, rcx
0x140836802: mov rax, qword ptr [rax + 0x10]
0x140836806: mov rcx, qword ptr [rax]
0x140836809: test rcx, rcx
0x14083680c: js 0x140836815
0x14083680e: cvtsi2ss xmm0, rcx
0x140836813: jmp 0x14083682a
0x140836815: mov rdx, rcx
0x140836818: shr rdx, 1
0x14083681b: and ecx, 1
0x14083681e: or rcx, rdx
0x140836821: cvtsi2ss xmm0, rcx
0x140836826: addss xmm0, xmm0
0x14083682a: cvtsi2ss xmm1, dword ptr [rax + 8]
0x14083682f: divss xmm1, dword ptr [rip + 0x2170e09]
0x140836837: addss xmm1, xmm0
0x14083683b: movss dword ptr [rbp + 0xc98], xmm1
0x140836843: mov byte ptr [rbp + 0xc9c], 0
0x14083684a: mov rax, qword ptr [rbp + 0xeb0]
0x140836851: mov r15, qword ptr [rax + 0xf8]
0x140836858: mov r14, qword ptr [rax + 0x100]
0x14083685f: shl r14, 7
0x140836863: add r14, r15
0x140836866: lea rbx, [rbp + 0x890]
0x14083686d: lea rdi, [rbp + 0xcf8]
0x140836874: cmp r15, r14
0x140836877: je 0x140836a0b
0x14083687d: nop dword ptr [rax]
0x140836880: mov r12, qword ptr [r15 + 0x50]
0x140836884: mov r13, qword ptr [r15 + 0x58]
0x140836888: mov byte ptr [rbp + 0xe24], 1
0x14083688f: call 0x140be6750
0x140836894: mov esi, eax
0x140836896: mov byte ptr [rbp + 0xe24], 1
0x14083689d: xor ecx, ecx
0x14083689f: call 0x1400d9b20
0x1408368a4: test rax, rax
0x1408368a7: je 0x140836e89
0x1408368ad: mov dword ptr [rbp + 0xda0], esi
0x1408368b3: mov word ptr [rbp + 0xda8], 0
0x1408368bc: mov dword ptr [rbp + 0xda4], 0x3f800000
0x1408368c6: mov qword ptr [rbp + 0xcf8], 1
0x1408368d1: mov qword ptr [rbp + 0xd00], r12
0x1408368d8: mov qword ptr [rbp + 0xd98], r13
0x1408368df: mov qword ptr [rbp + 0xd08], 0
0x1408368ea: mov qword ptr [rbp + 0xd10], 4
0x1408368f5: mov qword ptr [rbp + 0xd18], 0
0x140836900: movups xmm0, xmmword ptr [rax]
0x140836903: inc qword ptr [rax]
0x140836906: movups xmm1, xmmword ptr [rip + 0x225fe23]
0x14083690d: lea rax, [rbp + 0xd68]
0x140836914: movups xmmword ptr [rax + 0x10], xmm1
0x140836918: movups xmm1, xmmword ptr [rip + 0x225fe01]
0x14083691f: movups xmmword ptr [rax], xmm1
0x140836922: movups xmmword ptr [rbp + 0xd88], xmm0
0x140836929: movabs rax, 0x100000003
0x140836933: mov qword ptr [rbp + 0xd38], rax
0x14083693a: mov qword ptr [rbp + 0xd20], 0
0x140836945: mov qword ptr [rbp + 0xd28], 1
0x140836950: mov qword ptr [rbp + 0xd30], 0
0x14083695b: mov byte ptr [rbp + 0xe24], 1
0x140836962: mov rcx, rbx
0x140836965: mov rdx, rdi
0x140836968: lea r8, [rbp + 0x4a8]
0x14083696f: xor r9d, r9d
0x140836972: call 0x140905be0
0x140836977: cmp byte ptr [r15 + 0x64], 0
0x14083697c: je 0x140836ea1
0x140836982: movzx eax, byte ptr [r15 + 0x65]
0x140836987: mov byte ptr [rbp + 0x941], al
0x14083698d: mov r8d, 0xf8
0x140836993: mov rcx, rdi
0x140836996: mov rdx, rbx
0x140836999: call 0x142923c00
0x14083699e: mov byte ptr [rbp + 0xe24], 1
0x1408369a5: lea rcx, [rbp + 0xc8]
0x1408369ac: mov rdx, rdi
0x1408369af: lea r8, [rbp + 0xc98]
0x1408369b6: mov r9, r15
0x1408369b9: call 0x14090b460
0x1408369be: mov rsi, qword ptr [r15 + 0x70]
0x1408369c2: movzx eax, byte ptr [rbp + 0x171]
0x1408369c9: and al, 0x8e
0x1408369cb: or al, 0x21
0x1408369cd: mov byte ptr [rbp + 0x171], al
0x1408369d3: mov r8d, 0xf8
0x1408369d9: mov rcx, rbx
0x1408369dc: lea rdx, [rbp + 0xc8]
0x1408369e3: call 0x142923c00
0x1408369e8: mov byte ptr [rbp + 0xe24], 1
0x1408369ef: mov rcx, rbx
0x1408369f2: mov rdx, r15
0x1408369f5: mov r8, qword ptr [rbp + 0xe18]
0x1408369fc: call rsi
0x1408369fe: sub r15, -0x80
0x140836a02: cmp r15, r14
0x140836a05: jne 0x140836880
0x140836a0b: lea rsi, [rbp + 0x890]
0x140836a12: lea rdx, [rbp + 0x4a8]
0x140836a19: mov r8d, 0x3e8
0x140836a1f: mov rcx, rsi
0x140836a22: call 0x142923c00
0x140836a27: mov byte ptr [rbp + 0xe24], 0
0x140836a2e: mov rcx, rsi
0x140836a31: call 0x140868370
0x140836a36: mov rcx, qword ptr [rbp + 0xdf0]
0x140836a3d: call 0x140bd5790
0x140836a42: mov rcx, qword ptr [rbp + 0xe18]
0x140836a49: mov dword ptr [rbp + 0x890], 0
0x140836a53: mov rdx, qword ptr [rbp + 0xe10]
0x140836a5a: mov qword ptr [rbp + 0x898], rdx
0x140836a61: mov qword ptr [rbp + 0xcf8], 1
0x140836a6c: mov rdx, qword ptr [rbp + 0xdf8]
0x140836a73: mov qword ptr [rbp + 0xd00], rdx
0x140836a7a: mov qword ptr [rbp + 0xd08], 0
0x140836a85: mov rdx, qword ptr [rbp + 0xe08]
0x140836a8c: mov qword ptr [rsp + 0x40], rdx
0x140836a91: mov qword ptr [rsp + 0x38], rcx
0x140836a96: mov qword ptr [rsp + 0x30], rax
0x140836a9b: mov qword ptr [rsp + 0x28], rdi
0x140836aa0: mov qword ptr [rsp + 0x20], 1
0x140836aa9: lea rdx, [rip + 0x225c06d]
0x140836ab0: lea rcx, [rbp + 0x4a8]
0x140836ab7: lea r9, [rbp + 0x890]
0x140836abe: mov r8d, 0xd
0x140836ac4: call 0x140867970
0x140836ac9: mov byte ptr [rbp + 0xe23], 1
0x140836ad0: movabs rdx, 0x4bc362195b80f729
0x140836ada: movabs r8, 0x982798ce80cbcefb
0x140836ae4: mov rcx, qword ptr [rbp + 0xe00]
0x140836aeb: call 0x1400dd6f0
0x140836af0: test rax, rax
0x140836af3: mov r8, qword ptr [rbp + 0xe18]
0x140836afa: je 0x140836f18
0x140836b00: cmp qword ptr [r8 + 0x218], rdx
0x140836b07: jbe 0x140836f18
0x140836b0d: mov rax, qword ptr [r8 + 0x210]
0x140836b14: mov rcx, qword ptr [rax + rdx*8]
0x140836b18: test rcx, rcx
0x140836b1b: je 0x140836f18
0x140836b21: mov rax, qword ptr [r8 + 0x1e0]
0x140836b28: not rcx
0x140836b2b: lea rcx, [rcx + rcx*2]
0x140836b2f: shl rcx, 5
0x140836b33: cmp qword ptr [rax + rcx + 0x20], 0
0x140836b39: je 0x140836f18
0x140836b3f: add rax, rcx
0x140836b42: mov rax, qword ptr [rax + 0x10]
0x140836b46: mov rcx, qword ptr [rax]
0x140836b49: test rcx, rcx
0x140836b4c: js 0x140836b55
0x140836b4e: cvtsi2ss xmm0, rcx
0x140836b53: jmp 0x140836b6a
0x140836b55: mov rdx, rcx
0x140836b58: shr rdx, 1
0x140836b5b: and ecx, 1
0x140836b5e: or rcx, rdx
0x140836b61: cvtsi2ss xmm0, rcx
0x140836b66: addss xmm0, xmm0
0x140836b6a: cvtsi2ss xmm1, dword ptr [rax + 8]
0x140836b6f: divss xmm1, dword ptr [rip + 0x2170ac9]
0x140836b77: addss xmm1, xmm0
0x140836b7b: movss dword ptr [rbp + 0xce8], xmm1
0x140836b83: mov byte ptr [rbp + 0xcec], 0
0x140836b8a: mov rax, qword ptr [rbp + 0xeb0]
0x140836b91: mov rdi, qword ptr [rax + 0x110]
0x140836b98: mov r14, qword ptr [rax + 0x118]
0x140836b9f: shl r14, 7
0x140836ba3: add r14, rdi
0x140836ba6: lea r13, [rbp + 0xd18]
0x140836bad: xorps xmm6, xmm6
0x140836bb0: lea r12, [rbp - 0x30]
0x140836bb4: cmp rdi, r14
0x140836bb7: je 0x140836d36
0x140836bbd: nop dword ptr [rax]
0x140836bc0: mov rbx, qword ptr [rdi + 0x50]
0x140836bc4: mov r15, qword ptr [rdi + 0x58]
0x140836bc8: mov byte ptr [rbp + 0xe23], 1
0x140836bcf: call 0x140be6750
0x140836bd4: mov esi, eax
0x140836bd6: mov byte ptr [rbp + 0xe23], 1
0x140836bdd: xor ecx, ecx
0x140836bdf: call 0x1400d9b20
0x140836be4: test rax, rax
0x140836be7: je 0x140836eaf
0x140836bed: movups xmm0, xmmword ptr [rax]
0x140836bf0: inc qword ptr [rax]
0x140836bf3: movups xmm1, xmmword ptr [rip + 0x225fb36]
0x140836bfa: lea rax, [rbp + 0xcc0]
0x140836c01: movups xmmword ptr [rax + 0x10], xmm1
0x140836c05: movups xmm1, xmmword ptr [rip + 0x225fb14]
0x140836c0c: movups xmmword ptr [rax], xmm1
0x140836c0f: mov qword ptr [rbp + 0xcf8], 1
0x140836c1a: mov qword ptr [rbp + 0xd00], rbx
0x140836c21: mov qword ptr [rbp + 0xd08], 0
0x140836c2c: mov qword ptr [rbp + 0xd10], 4
0x140836c37: movups xmmword ptr [r13], xmm6
0x140836c3c: mov qword ptr [rbp + 0xd28], 1
0x140836c47: mov qword ptr [rbp + 0xd30], 0
0x140836c52: movabs rax, 0x100000003
0x140836c5c: mov qword ptr [rbp + 0xd38], rax
0x140836c63: mov rax, qword ptr [rbp + 0xcd8]
0x140836c6a: mov qword ptr [r13 + 0x68], rax
0x140836c6e: movups xmm1, xmmword ptr [rbp + 0xc98]
0x140836c75: movups xmm2, xmmword ptr [rbp + 0xca8]
0x140836c7c: movups xmm3, xmmword ptr [rbp + 0xcb8]
0x140836c83: movups xmm4, xmmword ptr [rbp + 0xcc8]
0x140836c8a: movups xmmword ptr [r13 + 0x58], xmm4
0x140836c8f: movups xmmword ptr [r13 + 0x48], xmm3
0x140836c94: movups xmmword ptr [r13 + 0x38], xmm2
0x140836c99: movups xmmword ptr [r13 + 0x28], xmm1
0x140836c9e: movups xmmword ptr [rbp + 0xd88], xmm0
0x140836ca5: mov qword ptr [rbp + 0xd98], r15
0x140836cac: and esi, 0xffffff8f
0x140836caf: or esi, 0x20
0x140836cb2: mov dword ptr [rbp + 0xda0], esi
0x140836cb8: mov dword ptr [rbp + 0xda4], 0x3f800000
0x140836cc2: mov word ptr [rbp + 0xda8], 0
0x140836ccb: mov byte ptr [rbp + 0xe23], 1
0x140836cd2: lea rcx, [rbp + 0x890]
0x140836cd9: lea rdx, [rbp + 0xcf8]
0x140836ce0: lea r8, [rbp + 0x4a8]
0x140836ce7: xor r9d, r9d
0x140836cea: call 0x140905be0
0x140836cef: mov byte ptr [rbp + 0xe23], 1
0x140836cf6: mov rcx, r12
0x140836cf9: lea rdx, [rbp + 0x890]
0x140836d00: lea r8, [rbp + 0xce8]
0x140836d07: mov r9, rdi
0x140836d0a: call 0x14090b460
0x140836d0f: mov rax, qword ptr [rdi + 0x70]
0x140836d13: mov byte ptr [rbp + 0xe23], 1
0x140836d1a: mov rcx, r12
0x140836d1d: mov rdx, rdi
0x140836d20: mov r8, qword ptr [rbp + 0xe18]
0x140836d27: call rax
0x140836d29: sub rdi, -0x80
0x140836d2d: cmp rdi, r14
0x140836d30: jne 0x140836bc0
0x140836d36: lea rsi, [rbp + 0x890]
0x140836d3d: lea rdx, [rbp + 0x4a8]
0x140836d44: mov r8d, 0x3e8
0x140836d4a: mov rcx, rsi
0x140836d4d: call 0x142923c00
0x140836d52: mov byte ptr [rbp + 0xe23], 0
0x140836d59: mov rcx, rsi
0x140836d5c: call 0x140868370
0x140836d61: cmp byte ptr [rbp + 0xe22], 0
0x140836d68: je 0x140836dad
0x140836d6a: call 0x140c18130
0x140836d6f: mov rcx, rax
0x140836d72: mov rax, qword ptr [rax]
0x140836d75: cmp rax, 1
0x140836d79: jne 0x140836de2
0x140836d7b: add rcx, 8
0x140836d7f: cmp qword ptr [rcx], 0
0x140836d83: jne 0x140836e02
0x140836d85: mov qword ptr [rcx], 0xffffffffffffffff
0x140836d8c: mov qword ptr [rbp + 0xe18], rcx
0x140836d93: add rcx, 8
0x140836d97: mov rdx, qword ptr [rbp + 0xce0]
0x140836d9e: call 0x140c170f0
0x140836da3: mov rax, qword ptr [rbp + 0xe18]
0x140836daa: inc qword ptr [rax]
0x140836dad: movaps xmm6, xmmword ptr [rbp + 0xe30]
0x140836db4: add rsp, 0xec8
0x140836dbb: pop rbx
0x140836dbc: pop rdi
0x140836dbd: pop rsi
0x140836dbe: pop r12
0x140836dc0: pop r13
0x140836dc2: pop r14
0x140836dc4: pop r15
0x140836dc6: pop rbp
0x140836dc7: ret 
0x140836dc8: test rax, rax
0x140836dcb: jne 0x140836df6
0x140836dcd: xor edx, edx
0x140836dcf: call 0x1428f87d0
0x140836dd4: mov rcx, rax
0x140836dd7: test rax, rax
0x140836dda: jne 0x140835c76
0x140836de0: jmp 0x140836df6
0x140836de2: test rax, rax
0x140836de5: jne 0x140836df6
0x140836de7: xor edx, edx
0x140836de9: call 0x1428f87d0
0x140836dee: mov rcx, rax
0x140836df1: test rax, rax
0x140836df4: jne 0x140836d7f
0x140836df6: lea rcx, [rip + 0x2403ca3]
0x140836dfd: call 0x1428d80d0
0x140836e02: lea rcx, [rip + 0x2403d67]
0x140836e09: call 0x1428d9250
0x140836e0e: mov byte ptr [rbp + 0xe27], 1
0x140836e15: lea rcx, [rip + 0x2403c84]
0x140836e1c: call 0x1428d80d0
0x140836e21: jmp 0x140836f2b
0x140836e26: lea rcx, [rip + 0x225bd4b]
0x140836e2d: call 0x1428d9310
0x140836e32: jmp 0x140836f2b
0x140836e37: mov byte ptr [rbp + 0xe26], 1
0x140836e3e: lea rcx, [rip + 0x2403c5b]
0x140836e45: call 0x1428d80d0
0x140836e4a: jmp 0x140836f2b
0x140836e4f: lea rcx, [rip + 0x225bd0a]
0x140836e56: call 0x1428d9310
0x140836e5b: jmp 0x140836f2b
0x140836e60: mov byte ptr [rbp + 0xe25], 1
0x140836e67: lea rcx, [rip + 0x2403c32]
0x140836e6e: call 0x1428d80d0
0x140836e73: jmp 0x140836f2b
0x140836e78: lea rcx, [rip + 0x225bcc9]
0x140836e7f: call 0x1428d9310
0x140836e84: jmp 0x140836f2b
0x140836e89: mov byte ptr [rbp + 0xe24], 1
0x140836e90: lea rcx, [rip + 0x2403c09]
0x140836e97: call 0x1428d80d0
0x140836e9c: jmp 0x140836f2b
0x140836ea1: lea rcx, [rip + 0x225bc88]
0x140836ea8: call 0x1428d9310
0x140836ead: jmp 0x140836f2b
0x140836eaf: mov byte ptr [rbp + 0xe23], 1
0x140836eb6: lea rcx, [rip + 0x2403be3]
0x140836ebd: call 0x1428d80d0
0x140836ec2: jmp 0x140836f2b
0x140836ec4: mov byte ptr [rbp + 0xe27], 1
0x140836ecb: lea rcx, [rip + 0x226ed06]
0x140836ed2: call 0x1428d9310
0x140836ed7: jmp 0x140836f2b
0x140836ed9: mov byte ptr [rbp + 0xe26], 1
0x140836ee0: lea rcx, [rip + 0x226ecf1]
0x140836ee7: call 0x1428d9310
0x140836eec: jmp 0x140836f2b
0x140836eee: mov byte ptr [rbp + 0xe25], 1
0x140836ef5: lea rcx, [rip + 0x226ecdc]
0x140836efc: call 0x1428d9310
0x140836f01: jmp 0x140836f2b
0x140836f03: mov byte ptr [rbp + 0xe24], 1
0x140836f0a: lea rcx, [rip + 0x226ecc7]
0x140836f11: call 0x1428d9310
0x140836f16: jmp 0x140836f2b
0x140836f18: mov byte ptr [rbp + 0xe23], 1
0x140836f1f: lea rcx, [rip + 0x226ecb2]
0x140836f26: call 0x1428d9310
0x140836f2b: ud2 
0x140836f2d: nop dword ptr [rax]
0x140836f30: mov qword ptr [rsp + 0x10], rdx
0x140836f35: push rbp
0x140836f36: push r15
0x140836f38: push r14
0x140836f3a: push r13
0x140836f3c: push r12
0x140836f3e: push rsi
0x140836f3f: push rdi
0x140836f40: push rbx
0x140836f41: sub rsp, 0x58
0x140836f45: lea rbp, [rdx + 0x80]
0x140836f4c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140836f51: mov rax, qword ptr [rbp + 0xe18]
0x140836f58: inc qword ptr [rax]
0x140836f5b: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140836f60: add rsp, 0x58
0x140836f64: pop rbx
0x140836f65: pop rdi
0x140836f66: pop rsi
0x140836f67: pop r12
0x140836f69: pop r13
0x140836f6b: pop r14
0x140836f6d: pop r15
0x140836f6f: pop rbp
0x140836f70: ret 
0x140836f71: nop word ptr cs:[rax + rax]
0x140836f80: mov qword ptr [rsp + 0x10], rdx
0x140836f85: push rbp
0x140836f86: push r15
0x140836f88: push r14
0x140836f8a: push r13
0x140836f8c: push r12
0x140836f8e: push rsi
0x140836f8f: push rdi
0x140836f90: push rbx
0x140836f91: sub rsp, 0x58
0x140836f95: lea rbp, [rdx + 0x80]
0x140836f9c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140836fa1: cmp byte ptr [rbp + 0xe22], 0
0x140836fa8: je 0x140836fd9
0x140836faa: lea rax, [rbp + 0xc80]
0x140836fb1: mov qword ptr [rbp + 0xc90], rax
0x140836fb8: lea rax, [rbp + 0xc90]
0x140836fbf: mov qword ptr [rbp + 0xc88], rax
0x140836fc6: lea rcx, [rip + 0x225b5bb]
0x140836fcd: lea rdx, [rbp + 0xc88]
0x140836fd4: call 0x14132f2d0
0x140836fd9: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140836fde: add rsp, 0x58
0x140836fe2: pop rbx
0x140836fe3: pop rdi
0x140836fe4: pop rsi
0x140836fe5: pop r12
0x140836fe7: pop r13
0x140836fe9: pop r14
0x140836feb: pop r15
0x140836fed: pop rbp
0x140836fee: ret 
0x140836fef: nop 
0x140836ff0: mov qword ptr [rsp + 0x10], rdx
0x140836ff5: push rbp
0x140836ff6: push r15
0x140836ff8: push r14
0x140836ffa: push r13
0x140836ffc: push r12
0x140836ffe: push rsi
0x140836fff: push rdi
0x140837000: push rbx
0x140837001: sub rsp, 0x58
0x140837005: lea rbp, [rdx + 0x80]
0x14083700c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140837011: mov rax, qword ptr [rbp + 0xe18]
0x140837018: inc qword ptr [rax]
0x14083701b: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140837020: add rsp, 0x58
0x140837024: pop rbx
0x140837025: pop rdi
0x140837026: pop rsi
0x140837027: pop r12
0x140837029: pop r13
0x14083702b: pop r14
0x14083702d: pop r15
0x14083702f: pop rbp
0x140837030: ret 
0x140837031: nop word ptr cs:[rax + rax]
0x140837040: mov qword ptr [rsp + 0x10], rdx
0x140837045: push rbp
0x140837046: push r15
0x140837048: push r14
0x14083704a: push r13
0x14083704c: push r12
0x14083704e: push rsi
0x14083704f: push rdi
0x140837050: push rbx
0x140837051: sub rsp, 0x58
0x140837055: lea rbp, [rdx + 0x80]
0x14083705c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140837061: cmp byte ptr [rbp + 0xe23], 0
0x140837068: je 0x14083708c
0x14083706a: lea rcx, [rbp + 0x4a8]
0x140837071: call 0x1408923c0
0x140837076: movaps xmm6, xmmword ptr [rsp + 0x40]
0x14083707b: add rsp, 0x58
0x14083707f: pop rbx
0x140837080: pop rdi
0x140837081: pop rsi
0x140837082: pop r12
0x140837084: pop r13
0x140837086: pop r14
0x140837088: pop r15
0x14083708a: pop rbp
0x14083708b: ret 
0x14083708c: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140837091: add rsp, 0x58
0x140837095: pop rbx
0x140837096: pop rdi
0x140837097: pop rsi
0x140837098: pop r12
0x14083709a: pop r13
0x14083709c: pop r14
0x14083709e: pop r15
0x1408370a0: pop rbp
0x1408370a1: ret 
0x1408370a2: nop word ptr cs:[rax + rax]
0x1408370b0: mov qword ptr [rsp + 0x10], rdx
0x1408370b5: push rbp
0x1408370b6: push r15
0x1408370b8: push r14
0x1408370ba: push r13
0x1408370bc: push r12
0x1408370be: push rsi
0x1408370bf: push rdi
0x1408370c0: push rbx
0x1408370c1: sub rsp, 0x58
0x1408370c5: lea rbp, [rdx + 0x80]
0x1408370cc: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408370d1: lea rcx, [rbp + 0x890]
0x1408370d8: call 0x1408935c0
0x1408370dd: mov byte ptr [rbp + 0xe24], 1
0x1408370e4: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408370e9: add rsp, 0x58
0x1408370ed: pop rbx
0x1408370ee: pop rdi
0x1408370ef: pop rsi
0x1408370f0: pop r12
0x1408370f2: pop r13
0x1408370f4: pop r14
0x1408370f6: pop r15
0x1408370f8: pop rbp
0x1408370f9: ret 
0x1408370fa: nop word ptr [rax + rax]
0x140837100: mov qword ptr [rsp + 0x10], rdx
0x140837105: push rbp
0x140837106: push r15
0x140837108: push r14
0x14083710a: push r13
0x14083710c: push r12
0x14083710e: push rsi
0x14083710f: push rdi
0x140837110: push rbx
0x140837111: sub rsp, 0x58
0x140837115: lea rbp, [rdx + 0x80]
0x14083711c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140837121: cmp byte ptr [rbp + 0xe24], 0
0x140837128: je 0x14083714c
0x14083712a: lea rcx, [rbp + 0x4a8]
0x140837131: call 0x1408923c0
0x140837136: movaps xmm6, xmmword ptr [rsp + 0x40]
0x14083713b: add rsp, 0x58
0x14083713f: pop rbx
0x140837140: pop rdi
0x140837141: pop rsi
0x140837142: pop r12
0x140837144: pop r13
0x140837146: pop r14
0x140837148: pop r15
0x14083714a: pop rbp
0x14083714b: ret 
0x14083714c: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140837151: add rsp, 0x58
0x140837155: pop rbx
0x140837156: pop rdi
0x140837157: pop rsi
0x140837158: pop r12
0x14083715a: pop r13
0x14083715c: pop r14
0x14083715e: pop r15
0x140837160: pop rbp
0x140837161: ret 
0x140837162: nop word ptr cs:[rax + rax]
0x140837170: mov qword ptr [rsp + 0x10], rdx
0x140837175: push rbp
0x140837176: push r15
0x140837178: push r14
0x14083717a: push r13
0x14083717c: push r12
0x14083717e: push rsi
0x14083717f: push rdi
0x140837180: push rbx
0x140837181: sub rsp, 0x58
0x140837185: lea rbp, [rdx + 0x80]
0x14083718c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140837191: lea rcx, [rbp + 0x890]
0x140837198: call 0x1408935c0
0x14083719d: mov byte ptr [rbp + 0xe25], 1
0x1408371a4: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408371a9: add rsp, 0x58
0x1408371ad: pop rbx
0x1408371ae: pop rdi
0x1408371af: pop rsi
0x1408371b0: pop r12
0x1408371b2: pop r13
0x1408371b4: pop r14
0x1408371b6: pop r15
0x1408371b8: pop rbp
0x1408371b9: ret 
0x1408371ba: nop word ptr [rax + rax]
0x1408371c0: mov qword ptr [rsp + 0x10], rdx
0x1408371c5: push rbp
0x1408371c6: push r15
0x1408371c8: push r14
0x1408371ca: push r13
0x1408371cc: push r12
0x1408371ce: push rsi
0x1408371cf: push rdi
0x1408371d0: push rbx
0x1408371d1: sub rsp, 0x58
0x1408371d5: lea rbp, [rdx + 0x80]
0x1408371dc: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408371e1: cmp byte ptr [rbp + 0xe25], 0
0x1408371e8: je 0x14083720c
0x1408371ea: lea rcx, [rbp + 0x4a8]
0x1408371f1: call 0x1408923c0
0x1408371f6: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408371fb: add rsp, 0x58
0x1408371ff: pop rbx
0x140837200: pop rdi
0x140837201: pop rsi
0x140837202: pop r12
0x140837204: pop r13
0x140837206: pop r14
0x140837208: pop r15
0x14083720a: pop rbp
0x14083720b: ret 
0x14083720c: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140837211: add rsp, 0x58
0x140837215: pop rbx
0x140837216: pop rdi
0x140837217: pop rsi
0x140837218: pop r12
0x14083721a: pop r13
0x14083721c: pop r14
0x14083721e: pop r15
0x140837220: pop rbp
0x140837221: ret 
0x140837222: nop word ptr cs:[rax + rax]
0x140837230: mov qword ptr [rsp + 0x10], rdx
0x140837235: push rbp
0x140837236: push r15
0x140837238: push r14
0x14083723a: push r13
0x14083723c: push r12
0x14083723e: push rsi
0x14083723f: push rdi
0x140837240: push rbx
0x140837241: sub rsp, 0x58
0x140837245: lea rbp, [rdx + 0x80]
0x14083724c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140837251: lea rcx, [rbp + 0x890]
0x140837258: call 0x1408935c0
0x14083725d: mov byte ptr [rbp + 0xe26], 1
0x140837264: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140837269: add rsp, 0x58
0x14083726d: pop rbx
0x14083726e: pop rdi
0x14083726f: pop rsi
0x140837270: pop r12
0x140837272: pop r13
0x140837274: pop r14
0x140837276: pop r15
0x140837278: pop rbp
0x140837279: ret 
0x14083727a: nop word ptr [rax + rax]
0x140837280: mov qword ptr [rsp + 0x10], rdx
0x140837285: push rbp
0x140837286: push r15
0x140837288: push r14
0x14083728a: push r13
0x14083728c: push r12
0x14083728e: push rsi
0x14083728f: push rdi
0x140837290: push rbx
0x140837291: sub rsp, 0x58
0x140837295: lea rbp, [rdx + 0x80]
0x14083729c: movaps xmmword ptr [rsp + 0x40], xmm6
0x1408372a1: cmp byte ptr [rbp + 0xe26], 0
0x1408372a8: je 0x1408372cc
0x1408372aa: lea rcx, [rbp + 0x4a8]
0x1408372b1: call 0x1408923c0
0x1408372b6: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408372bb: add rsp, 0x58
0x1408372bf: pop rbx
0x1408372c0: pop rdi
0x1408372c1: pop rsi
0x1408372c2: pop r12
0x1408372c4: pop r13
0x1408372c6: pop r14
0x1408372c8: pop r15
0x1408372ca: pop rbp
0x1408372cb: ret 
0x1408372cc: movaps xmm6, xmmword ptr [rsp + 0x40]
0x1408372d1: add rsp, 0x58
0x1408372d5: pop rbx
0x1408372d6: pop rdi
0x1408372d7: pop rsi
0x1408372d8: pop r12
0x1408372da: pop r13
0x1408372dc: pop r14
0x1408372de: pop r15
0x1408372e0: pop rbp
0x1408372e1: ret 
0x1408372e2: nop word ptr cs:[rax + rax]
0x1408372f0: mov qword ptr [rsp + 0x10], rdx
0x1408372f5: push rbp
0x1408372f6: push r15
0x1408372f8: push r14
0x1408372fa: push r13
0x1408372fc: push r12
0x1408372fe: push rsi
0x1408372ff: push rdi
0x140837300: push rbx
0x140837301: sub rsp, 0x58
0x140837305: lea rbp, [rdx + 0x80]
0x14083730c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140837311: lea rcx, [rbp + 0x890]
0x140837318: call 0x1408935c0
0x14083731d: mov byte ptr [rbp + 0xe27], 1
0x140837324: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140837329: add rsp, 0x58
0x14083732d: pop rbx
0x14083732e: pop rdi
0x14083732f: pop rsi
0x140837330: pop r12
0x140837332: pop r13
0x140837334: pop r14
0x140837336: pop r15
0x140837338: pop rbp
0x140837339: ret 
0x14083733a: nop word ptr [rax + rax]
0x140837340: mov qword ptr [rsp + 0x10], rdx
0x140837345: push rbp
0x140837346: push r15
0x140837348: push r14
0x14083734a: push r13
0x14083734c: push r12
0x14083734e: push rsi
0x14083734f: push rdi
0x140837350: push rbx
0x140837351: sub rsp, 0x58
0x140837355: lea rbp, [rdx + 0x80]
0x14083735c: movaps xmmword ptr [rsp + 0x40], xmm6
0x140837361: cmp byte ptr [rbp + 0xe27], 0
0x140837368: je 0x14083738c
0x14083736a: lea rcx, [rbp + 0x4a8]
0x140837371: call 0x1408923c0
0x140837376: movaps xmm6, xmmword ptr [rsp + 0x40]
0x14083737b: add rsp, 0x58
0x14083737f: pop rbx
0x140837380: pop rdi
0x140837381: pop rsi
0x140837382: pop r12
0x140837384: pop r13
0x140837386: pop r14
0x140837388: pop r15
0x14083738a: pop rbp
0x14083738b: ret 
0x14083738c: movaps xmm6, xmmword ptr [rsp + 0x40]
0x140837391: add rsp, 0x58
0x140837395: pop rbx
0x140837396: pop rdi
0x140837397: pop rsi
0x140837398: pop r12
0x14083739a: pop r13
0x14083739c: pop r14
0x14083739e: pop r15
0x1408373a0: pop rbp
0x1408373a1: ret 
