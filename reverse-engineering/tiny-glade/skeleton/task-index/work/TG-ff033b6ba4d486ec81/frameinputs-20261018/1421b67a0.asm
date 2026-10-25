1421b67a0: push rbp
1421b67a1: push r15
1421b67a3: push r14
1421b67a5: push r13
1421b67a7: push r12
1421b67a9: push rsi
1421b67aa: push rdi
1421b67ab: push rbx
1421b67ac: sub rsp, 0x328
1421b67b3: lea rbp, [rsp + 0x80]
1421b67bb: movaps xmmword ptr [rbp + 0x290], xmm15
1421b67c3: movaps xmmword ptr [rbp + 0x280], xmm14
1421b67cb: movaps xmmword ptr [rbp + 0x270], xmm13
1421b67d3: movaps xmmword ptr [rbp + 0x260], xmm12
1421b67db: movaps xmmword ptr [rbp + 0x250], xmm11
1421b67e3: movaps xmmword ptr [rbp + 0x240], xmm10
1421b67eb: movaps xmmword ptr [rbp + 0x230], xmm9
1421b67f3: movaps xmmword ptr [rbp + 0x220], xmm8
1421b67fb: movaps xmmword ptr [rbp + 0x210], xmm7
1421b6802: movaps xmmword ptr [rbp + 0x200], xmm6
1421b6809: mov qword ptr [rbp + 0x1f8], 0xfffffffffffffffe
1421b6814: mov rsi, r9
1421b6817: mov r14, rdx
1421b681a: mov rdi, rcx
1421b681d: mov rcx, qword ptr [r8]
1421b6820: lea rdx, [rip + 0xe42391]
1421b6827: call 0x1421b5d00
1421b682c: test rax, rax
1421b682f: je 0x1421b7989
1421b6835: mov rbx, rax
1421b6838: lea rax, [rip + 0xe424b1]
1421b683f: mov qword ptr [rbp - 0x50], rax
1421b6843: mov qword ptr [rbp - 0x48], 0x3d
1421b684b: cmp qword ptr [rbx + 0x18], 0x3d
1421b6850: jne 0x1421b7934
1421b6856: mov rdx, qword ptr [rbx + 0x10]
1421b685a: lea rcx, [rip + 0xe4248f]
1421b6861: mov r8d, 0x3d
1421b6867: call 0x142923bf0
1421b686c: test eax, eax
1421b686e: jne 0x1421b7934
1421b6874: mov rcx, qword ptr [rbx]
1421b6877: mov rax, qword ptr [rbx + 8]
1421b687b: call qword ptr [rax + 0x18]
1421b687e: test rax, rax
1421b6881: je 0x1421b7989
1421b6887: mov rbx, rax
1421b688a: mov rax, qword ptr [r14]
1421b688d: cmp qword ptr [rax], 0
1421b6891: je 0x1421b7995
1421b6897: mov r15, qword ptr [rax + 8]
1421b689b: test r15, r15
1421b689e: je 0x1421b79a1
1421b68a4: lea rcx, [r15 + 0x18]
1421b68a8: mov dl, 1
1421b68aa: xor eax, eax
1421b68ac: lock cmpxchg byte ptr [r15 + 0x18], dl
1421b68b2: jne 0x1421b7964
1421b68b8: call 0x140c81c70
1421b68bd: mov rax, qword ptr [r15 + 0x10]
1421b68c1: test rax, rax
1421b68c4: sete cl
1421b68c7: shl rax, 2
1421b68cb: add rax, qword ptr [r15 + 8]
1421b68cf: add rax, -4
1421b68d3: sete dl
1421b68d6: or dl, cl
1421b68d8: je 0x1421b68ec
1421b68da: mov rax, qword ptr [rbx + 0x28]
1421b68de: test rax, rax
1421b68e1: je 0x1421b79ad
1421b68e7: lea ecx, [rax - 1]
1421b68ea: jmp 0x1421b68f2
1421b68ec: mov ecx, dword ptr [rax]
1421b68ee: mov rax, qword ptr [rbx + 0x28]
1421b68f2: mov rdx, qword ptr [rbx + 0x20]
1421b68f6: movups xmm0, xmmword ptr [rbx + 0x30]
1421b68fa: movups xmm1, xmmword ptr [rbx + 0x40]
1421b68fe: movups xmm2, xmmword ptr [rbx + 0x50]
1421b6902: mov dword ptr [rbp], ecx
1421b6905: mov qword ptr [rbp - 0x50], rdx
1421b6909: mov qword ptr [rbp - 0x48], rax
1421b690d: movups xmmword ptr [rbp - 0x40], xmm0
1421b6911: mov qword ptr [rbp - 0x30], r15
1421b6915: movups xmmword ptr [rbp - 0x28], xmm1
1421b6919: mov qword ptr [rbp - 0x18], rbx
1421b691d: movups xmmword ptr [rbp - 0x10], xmm2
1421b6921: mov r10, qword ptr [rdi]
1421b6924: mov rax, qword ptr [rdi + 8]
1421b6928: mov rcx, qword ptr [r10]
1421b692b: xor ebx, ebx
1421b692d: mov rdx, rcx
1421b6930: sub rdx, qword ptr [rax + 0x18]
1421b6934: cmovb rdx, rbx
1421b6938: sub rcx, qword ptr [rax + 0x38]
1421b693c: cmovb rcx, rbx
1421b6940: lea r11, [rdx + rdx*4]
1421b6944: shl r11, 3
1421b6948: add r11, qword ptr [rax + 8]
1421b694c: mov r8, qword ptr [rax + 0x10]
1421b6950: mov r9, qword ptr [rax + 0x30]
1421b6954: sub r8, rdx
1421b6957: cmovb r8, rbx
1421b695b: mov edx, 8
1421b6960: cmovb r11, rdx
1421b6964: lea r12, [rcx + rcx*4]
1421b6968: shl r12, 3
1421b696c: add r12, qword ptr [rax + 0x28]
1421b6970: sub r9, rcx
1421b6973: cmovb r9, rbx
1421b6977: cmovb r12, rdx
1421b697b: mov rcx, qword ptr [rbp + 0x310]
1421b6982: mov rax, qword ptr [rax + 0x40]
1421b6986: sub rax, r9
1421b6989: sub rax, r8
1421b698c: mov qword ptr [rbp + 0x30], r10
1421b6990: mov qword ptr [r10], rax
1421b6993: lea rax, [r8 + r8*4]
1421b6997: lea rax, [r11 + rax*8]
1421b699b: mov qword ptr [rbp + 0x40], rax
1421b699f: lea rax, [r9 + r9*4]
1421b69a3: lea rax, [r12 + rax*8]
1421b69a7: mov qword ptr [rbp + 0x48], rax
1421b69ab: mov rax, qword ptr [rsi]
1421b69ae: mov qword ptr [rbp + 0x50], rax
1421b69b2: lea r14, [rbp + 0x130]
1421b69b9: mov eax, dword ptr [rcx + 0x1c]
1421b69bc: mov dword ptr [rbp + 0xbc], eax
1421b69c2: mov rax, qword ptr [rcx]
1421b69c5: mov qword ptr [rbp + 0x58], rax
1421b69c9: mov rax, qword ptr [rcx + 0x10]
1421b69cd: mov qword ptr [rbp + 0x60], rax
1421b69d1: lea rdi, [rbp + 0x9c]
1421b69d8: movss xmm14, dword ptr [rip + 0x76ef23]
1421b69e1: lea r15, [rbp + 0x160]
1421b69e8: lea r13, [rbp + 0xc0]
1421b69ef: mov rax, r12
1421b69f2: mov rcx, r11
1421b69f5: mov r8, qword ptr [rbp + 0x40]
1421b69f9: mov r9, qword ptr [rbp + 0x48]
1421b69fd: jmp 0x1421b6a1a
1421b69ff: nop 
1421b6a00: lea rdx, [rcx + 0x28]
1421b6a04: mov r10, rax
1421b6a07: mov r11, rcx
1421b6a0a: mov rcx, qword ptr [rbp + 0x30]
1421b6a0e: inc qword ptr [rcx]
1421b6a11: test byte ptr [r11], 1
1421b6a15: mov rcx, rdx
1421b6a18: je 0x1421b6a50
1421b6a1a: test rcx, rcx
1421b6a1d: je 0x1421b6a24
1421b6a1f: cmp rcx, r8
1421b6a22: jne 0x1421b6a00
1421b6a24: test rax, rax
1421b6a27: je 0x1421b7655
1421b6a2d: cmp rax, r9
1421b6a30: je 0x1421b7655
1421b6a36: lea r10, [rax + 0x28]
1421b6a3a: xor edx, edx
1421b6a3c: mov r11, rax
1421b6a3f: mov rax, r10
1421b6a42: jmp 0x1421b6a0a
1421b6a44: nop word ptr cs:[rax + rax]
1421b6a50: mov qword ptr [rbp + 0x78], r10
1421b6a54: mov qword ptr [rbp + 0x70], rdx
1421b6a58: mov qword ptr [rbp + 0x68], r11
1421b6a5c: lea rcx, [r11 + 4]
1421b6a60: mov rax, qword ptr [rbp + 0x50]
1421b6a64: movss xmm0, dword ptr [rax + 8]
1421b6a69: movaps xmmword ptr [rbp + 0x80], xmm0
1421b6a70: movss xmm0, dword ptr [rax + 0xc]
1421b6a75: movss dword ptr [rbp + 0x1f4], xmm0
1421b6a7d: mov qword ptr [rbp + 0x150], rcx
1421b6a84: call 0x140c97970
1421b6a89: divss xmm0, dword ptr [rip + 0x76ee3f]
1421b6a91: cvttss2si rax, xmm0
1421b6a96: mov rcx, rax
1421b6a99: sar rcx, 0x3f
1421b6a9d: movaps xmm1, xmm0
1421b6aa0: subss xmm1, dword ptr [rip + 0x7a3634]
1421b6aa8: cvttss2si rdx, xmm1
1421b6aad: and rdx, rcx
1421b6ab0: or rdx, rax
1421b6ab3: ucomiss xmm0, dword ptr [rip + 0x7a9bd6]
1421b6aba: mov r8d, 0
1421b6ac0: cmovae r8, rdx
1421b6ac4: ucomiss xmm0, dword ptr [rip + 0x7a3615]
1421b6acb: mov rax, 0xffffffffffffffff
1421b6ad2: cmova r8, rax
1421b6ad6: cmp r8, 7
1421b6ada: mov eax, 6
1421b6adf: cmovb r8, rax
1421b6ae3: mov rcx, r15
1421b6ae6: mov rdx, qword ptr [rbp + 0x150]
1421b6aed: mov qword ptr [rbp + 0x158], r8
1421b6af4: call 0x140c97ba0
1421b6af9: mov rax, qword ptr [rbp + 0x168]
1421b6b00: test rax, rax
1421b6b03: js 0x1421b6b10
1421b6b05: cvtsi2ss xmm7, rax
1421b6b0a: jmp 0x1421b6b25
1421b6b0c: nop dword ptr [rax]
1421b6b10: mov rcx, rax
1421b6b13: shr rcx, 1
1421b6b16: and eax, 1
1421b6b19: or rax, rcx
1421b6b1c: cvtsi2ss xmm7, rax
1421b6b21: addss xmm7, xmm7
1421b6b25: mov rax, qword ptr [rbp + 0x160]
1421b6b2c: mov qword ptr [rbp + 0x1e0], rax
1421b6b33: mov r12, qword ptr [rbp + 0x170]
1421b6b3a: mov rax, qword ptr [rbp + 0x178]
1421b6b41: mov qword ptr [rbp + 0x1e8], rax
1421b6b48: addss xmm7, dword ptr [rip + 0x7a3594]
1421b6b50: xor esi, esi
1421b6b52: cmp r12, qword ptr [rbp + 0x1e8]
1421b6b59: jae 0x1421b6cec
1421b6b5f: nop 
1421b6b60: test r12, r12
1421b6b63: js 0x1421b6b70
1421b6b65: xorps xmm1, xmm1
1421b6b68: cvtsi2ss xmm1, r12
1421b6b6d: jmp 0x1421b6b8b
1421b6b6f: nop 
1421b6b70: mov rax, r12
1421b6b73: shr rax, 1
1421b6b76: mov ecx, r12d
1421b6b79: and ecx, 1
1421b6b7c: or rcx, rax
1421b6b7f: xorps xmm1, xmm1
1421b6b82: cvtsi2ss xmm1, rcx
1421b6b87: addss xmm1, xmm1
1421b6b8b: divss xmm1, xmm7
1421b6b8f: movss dword ptr [rbp + 0xc0], xmm1
1421b6b97: ucomiss xmm1, dword ptr [rip + 0x7a9af2]
1421b6b9e: jb 0x1421b7836
1421b6ba4: ucomiss xmm14, xmm1
1421b6ba8: jb 0x1421b7836
1421b6bae: ucomiss xmm1, xmm1
1421b6bb1: jp 0x1421b78a0
1421b6bb7: mov rcx, qword ptr [rbp + 0x1e0]
1421b6bbe: call 0x140c975a0
1421b6bc3: movss dword ptr [rbp + 0xf0], xmm0
1421b6bcb: movss dword ptr [rbp + 0xf4], xmm1
1421b6bd3: mov rcx, r15
1421b6bd6: lea rdx, [rbp + 0xf0]
1421b6bdd: movaps xmm2, xmmword ptr [rbp + 0x80]
1421b6be4: call 0x140c9cde0
1421b6be9: mov eax, dword ptr [rbp + 0x168]
1421b6bef: mov dword ptr [rbp + 0xc8], eax
1421b6bf5: mov rax, qword ptr [rbp + 0x160]
1421b6bfc: mov qword ptr [rbp + 0xc0], rax
1421b6c03: mov dword ptr [rbp + 0xcc], 0x3e4ccccd
1421b6c0d: movss xmm0, dword ptr [rbp + 0x1f4]
1421b6c15: movss dword ptr [rbp + 0xd0], xmm0
1421b6c1d: mov dword ptr [rbp + 0xd4], 0x3e4ccccd
1421b6c27: movups xmm0, xmmword ptr [rbp + 0xcc]
1421b6c2e: movaps xmm1, xmmword ptr [rbp + 0xd0]
1421b6c35: movaps xmm2, xmmword ptr [rbp + 0xc0]
1421b6c3c: shufps xmm0, xmm0, 0
1421b6c40: movss xmm3, dword ptr [rip + 0x76ecbc]
1421b6c48: mulps xmm0, xmm3
1421b6c4b: movaps xmm3, xmm1
1421b6c4e: shufps xmm3, xmm1, 0
1421b6c52: mulps xmm3, xmmword ptr [rip + 0x77d957]
1421b6c59: shufps xmm1, xmm1, 0x55
1421b6c5d: mulps xmm1, xmmword ptr [rip + 0x8544fc]
1421b6c64: shufps xmm2, xmm2, 0xa4
1421b6c68: movaps xmmword ptr [rbp + 0x160], xmm0
1421b6c6f: movaps xmmword ptr [rbp + 0x170], xmm3
1421b6c76: movaps xmmword ptr [rbp + 0x180], xmm1
1421b6c7d: movaps xmmword ptr [rbp + 0x190], xmm2
1421b6c84: lea rcx, [rbp + 0x110]
1421b6c8b: mov rdx, r15
1421b6c8e: call 0x14089d560
1421b6c93: movups xmm0, xmmword ptr [rbp + 0x110]
1421b6c9a: movups xmm1, xmmword ptr [rbp + 0x120]
1421b6ca1: movups xmm2, xmmword ptr [rbp + 0x130]
1421b6ca8: movaps xmmword ptr [rbp + 0x160], xmm0
1421b6caf: movaps xmmword ptr [rbp + 0x170], xmm1
1421b6cb6: movaps xmmword ptr [rbp + 0x180], xmm2
1421b6cbd: movaps xmm0, xmmword ptr [rip + 0x95e1dc]
1421b6cc4: movaps xmmword ptr [rbp + 0x190], xmm0
1421b6ccb: lea rcx, [rbp - 0x50]
1421b6ccf: mov rdx, r15
1421b6cd2: call 0x1421b6000
1421b6cd7: inc r12
1421b6cda: or bl, al
1421b6cdc: inc rsi
1421b6cdf: cmp r12, qword ptr [rbp + 0x1e8]
1421b6ce6: jb 0x1421b6b60
1421b6cec: lea rcx, [rbp + 8]
1421b6cf0: movss xmm1, dword ptr [rbp + 0x1f4]
1421b6cf8: call 0x140b34f80
1421b6cfd: mov rcx, qword ptr [rbp + 0x10]
1421b6d01: mov rax, qword ptr [rbp + 0x18]
1421b6d05: lea rax, [rcx + rax*4]
1421b6d09: mov qword ptr [rbp + 0x38], rax
1421b6d0d: mov dword ptr [rbp + 0xa8], 0
1421b6d17: movss xmm0, dword ptr [rbp + 0x1f4]
1421b6d1f: movss dword ptr [rbp + 0xac], xmm0
1421b6d27: mov qword ptr [rbp + 0xb0], rcx
1421b6d2e: mov rax, rcx
1421b6d31: mov qword ptr [rbp + 0x1e0], 0
1421b6d3c: lea r12, [rbp + 0x110]
1421b6d43: jmp 0x1421b6d57
1421b6d45: nop word ptr cs:[rax + rax]
1421b6d50: mov rax, qword ptr [rbp + 0x1e8]
1421b6d57: test rax, rax
1421b6d5a: je 0x1421b6d70
1421b6d5c: cmp rax, qword ptr [rbp + 0x38]
1421b6d60: je 0x1421b6d70
1421b6d62: lea rcx, [rax + 4]
1421b6d66: mov qword ptr [rbp + 0x1e8], rcx
1421b6d6d: jmp 0x1421b6da3
1421b6d6f: nop 
1421b6d70: mov rax, qword ptr [rbp + 0x1e0]
1421b6d77: cmp rax, 8
1421b6d7b: je 0x1421b7220
1421b6d81: mov rcx, rax
1421b6d84: add rax, rbp
1421b6d87: add rax, 0xa8
1421b6d8d: add rcx, 4
1421b6d91: mov qword ptr [rbp + 0x1e0], rcx
1421b6d98: mov qword ptr [rbp + 0x1e8], 0
1421b6da3: movss xmm6, dword ptr [rax]
1421b6da7: mov rcx, r15
1421b6daa: mov rdx, qword ptr [rbp + 0x150]
1421b6db1: mov r8, qword ptr [rbp + 0x158]
1421b6db8: call 0x140c97ba0
1421b6dbd: movups xmm0, xmmword ptr [rbp + 0x160]
1421b6dc4: movups xmm1, xmmword ptr [rbp + 0x170]
1421b6dcb: movaps xmmword ptr [rbp + 0x120], xmm1
1421b6dd2: movaps xmmword ptr [rbp + 0x110], xmm0
1421b6dd9: mov dword ptr [rbp + 0x130], 0
1421b6de3: addss xmm6, dword ptr [rbp + 0x80]
1421b6deb: movaps xmm8, xmm6
1421b6def: mulss xmm8, dword ptr [rip + 0x7a9898]
1421b6df8: movaps xmm7, xmm8
1421b6dfc: unpcklps xmm7, xmm6
1421b6dff: mov rax, qword ptr [rbp + 0x120]
1421b6e06: cmp rax, qword ptr [rbp + 0x128]
1421b6e0d: jae 0x1421b6d50
1421b6e13: nop word ptr cs:[rax + rax]
1421b6e20: lea rcx, [rax + 1]
1421b6e24: mov qword ptr [rbp + 0x120], rcx
1421b6e2b: test rax, rax
1421b6e2e: js 0x1421b6e60
1421b6e30: xorps xmm1, xmm1
1421b6e33: cvtsi2ss xmm1, rax
1421b6e38: mov rax, qword ptr [rbp + 0x118]
1421b6e3f: test rax, rax
1421b6e42: jns 0x1421b6e84
1421b6e44: mov rcx, rax
1421b6e47: shr rcx, 1
1421b6e4a: and eax, 1
1421b6e4d: or rax, rcx
1421b6e50: xorps xmm0, xmm0
1421b6e53: cvtsi2ss xmm0, rax
1421b6e58: addss xmm0, xmm0
1421b6e5c: jmp 0x1421b6e8c
1421b6e5e: nop 
1421b6e60: mov rcx, rax
1421b6e63: shr rcx, 1
1421b6e66: and eax, 1
1421b6e69: or rax, rcx
1421b6e6c: xorps xmm1, xmm1
1421b6e6f: cvtsi2ss xmm1, rax
1421b6e74: addss xmm1, xmm1
1421b6e78: mov rax, qword ptr [rbp + 0x118]
1421b6e7f: test rax, rax
1421b6e82: js 0x1421b6e44
1421b6e84: xorps xmm0, xmm0
1421b6e87: cvtsi2ss xmm0, rax
1421b6e8c: mov rcx, qword ptr [rbp + 0x110]
1421b6e93: addss xmm0, dword ptr [rip + 0x7a3249]
1421b6e9b: divss xmm1, xmm0
1421b6e9f: movss dword ptr [rbp + 0xb8], xmm1
1421b6ea7: ucomiss xmm1, dword ptr [rip + 0x7a97e2]
1421b6eae: jb 0x1421b77a8
1421b6eb4: ucomiss xmm14, xmm1
1421b6eb8: jb 0x1421b77a8
1421b6ebe: ucomiss xmm1, xmm1
1421b6ec1: jp 0x1421b7819
1421b6ec7: call 0x140c975a0
1421b6ecc: movss dword ptr [rbp + 0x160], xmm0
1421b6ed4: movss dword ptr [rbp + 0x164], xmm1
1421b6edc: mov rcx, rdi
1421b6edf: mov rdx, r15
1421b6ee2: call 0x140c9cdc0
1421b6ee7: cmp dword ptr [rbp + 0x130], 1
1421b6eee: jne 0x1421b6f40
1421b6ef0: mov eax, dword ptr [r14 + 0x18]
1421b6ef4: mov dword ptr [r14 + 0xc], eax
1421b6ef8: mov rax, qword ptr [r14 + 0x10]
1421b6efc: mov qword ptr [r14 + 4], rax
1421b6f00: mov eax, dword ptr [rdi + 8]
1421b6f03: mov dword ptr [r14 + 0x18], eax
1421b6f07: mov rax, qword ptr [rdi]
1421b6f0a: mov qword ptr [r14 + 0x10], rax
1421b6f0e: movss xmm12, dword ptr [rbp + 0x13c]
1421b6f17: movsd xmm10, qword ptr [rbp + 0x134]
1421b6f20: movsd xmm9, qword ptr [rbp + 0x9c]
1421b6f29: movss xmm15, dword ptr [rbp + 0xa4]
1421b6f32: jmp 0x1421b6fb7
1421b6f37: nop word ptr [rax + rax]
1421b6f40: mov eax, dword ptr [rdi + 8]
1421b6f43: lea rcx, [rbp + 0xc4]
1421b6f4a: mov dword ptr [rcx + 8], eax
1421b6f4d: mov rax, qword ptr [rdi]
1421b6f50: mov qword ptr [rcx], rax
1421b6f53: mov dword ptr [rbp + 0xc0], 1
1421b6f5d: mov qword ptr [rbp + 0xd0], r12
1421b6f64: mov rcx, r15
1421b6f67: mov rdx, r13
1421b6f6a: call 0x14258e8e0
1421b6f6f: movups xmm0, xmmword ptr [rbp + 0x160]
1421b6f76: movups xmm1, xmmword ptr [rbp + 0x16c]
1421b6f7d: movups xmmword ptr [r14 + 0xc], xmm1
1421b6f82: movups xmmword ptr [r14], xmm0
1421b6f86: cmp dword ptr [rbp + 0x130], 1
1421b6f8d: jne 0x1421b6d50
1421b6f93: movss xmm12, dword ptr [rbp + 0x13c]
1421b6f9c: movsd xmm10, qword ptr [rbp + 0x134]
1421b6fa5: movsd xmm9, qword ptr [rbp + 0x140]
1421b6fae: movss xmm15, dword ptr [rbp + 0x148]
1421b6fb7: movaps xmm13, xmm10
1421b6fbb: addps xmm13, xmm9
1421b6fbf: movaps xmm6, xmm12
1421b6fc3: addss xmm6, xmm15
1421b6fc8: mulps xmm13, xmmword ptr [rip + 0x76e910]
1421b6fd0: mulss xmm6, dword ptr [rip + 0x76e898]
1421b6fd8: movaps xmm0, xmm9
1421b6fdc: subss xmm0, xmm10
1421b6fe1: movaps xmm1, xmm15
1421b6fe5: subss xmm1, xmm12
1421b6fea: movaps xmm2, xmm0
1421b6fed: mulss xmm2, xmm0
1421b6ff1: movaps xmm3, xmm1
1421b6ff4: mulss xmm3, xmm1
1421b6ff8: addss xmm3, xmm2
1421b6ffc: xorps xmm2, xmm2
1421b6fff: sqrtss xmm2, xmm3
1421b7003: movaps xmm3, xmm14
1421b7007: divss xmm3, xmm2
1421b700b: mulss xmm0, xmm3
1421b700f: mulss xmm3, xmm1
1421b7013: movss dword ptr [rbp + 0x98], xmm0
1421b701b: movss dword ptr [rbp + 0x9c], xmm3
1421b7023: xorps xmm2, xmm2
1421b7026: mov rcx, r13
1421b7029: lea rdx, [rbp + 0x98]
1421b7030: call 0x140c9cde0
1421b7035: mov rcx, r15
1421b7038: mov rdx, r13
1421b703b: call 0x140c90a40
1421b7040: addps xmm13, xmm7
1421b7044: addss xmm6, xmm8
1421b7049: movaps xmm11, xmmword ptr [rbp + 0x160]
1421b7051: movss xmm3, dword ptr [rbp + 0x168]
1421b7059: subps xmm10, xmm9
1421b705d: subss xmm12, xmm15
1421b7062: mulps xmm10, xmm10
1421b7066: movshdup xmm0, xmm10
1421b706b: addss xmm0, xmm10
1421b7070: mulss xmm12, xmm12
1421b7075: addss xmm12, xmm0
1421b707a: xorps xmm0, xmm0
1421b707d: sqrtss xmm0, xmm12
1421b7082: movlps qword ptr [rbp + 0xf0], xmm13
1421b708a: movss dword ptr [rbp + 0xf8], xmm6
1421b7092: movsd xmm1, qword ptr [rip + 0x8d5656]
1421b709a: movsd qword ptr [rbp + 0xfc], xmm1
1421b70a2: movss dword ptr [rbp + 0x104], xmm0
1421b70aa: movups xmm2, xmmword ptr [rbp + 0xfc]
1421b70b1: movaps xmm0, xmmword ptr [rbp + 0x100]
1421b70b8: movaps xmm1, xmmword ptr [rbp + 0xf0]
1421b70bf: movaps xmm9, xmm3
1421b70c3: addss xmm9, xmm3
1421b70c8: mulss xmm3, xmm9
1421b70cd: movaps xmm5, xmm11
1421b70d1: addps xmm5, xmm11
1421b70d5: movshdup xmm4, xmm5
1421b70d9: mulss xmm4, xmm11
1421b70de: movaps xmm6, xmm11
1421b70e2: movsldup xmm10, xmm9
1421b70e7: mulps xmm10, xmm11
1421b70eb: shufps xmm11, xmm11, 0xff
1421b70f0: mulps xmm6, xmm5
1421b70f3: mulps xmm5, xmm11
1421b70f7: movaps xmm12, xmm11
1421b70fb: mulss xmm12, xmm9
1421b7100: shufps xmm5, xmm5, 0xe1
1421b7104: movshdup xmm13, xmm6
1421b7109: movaps xmm9, xmm13
1421b710d: addss xmm9, xmm3
1421b7112: movaps xmm11, xmm14
1421b7116: subss xmm11, xmm9
1421b711b: movaps xmm9, xmm4
1421b711f: addss xmm9, xmm12
1421b7124: unpcklps xmm11, xmm9
1421b7128: movaps xmm9, xmm10
1421b712c: subps xmm9, xmm5
1421b7130: shufps xmm11, xmm9, 4
1421b7135: subss xmm4, xmm12
1421b713a: addss xmm3, xmm6
1421b713e: movaps xmm12, xmm14
1421b7142: subss xmm12, xmm3
1421b7147: unpcklps xmm4, xmm12
1421b714b: addps xmm5, xmm10
1421b714f: shufps xmm4, xmm5, 0x54
1421b7153: shufps xmm9, xmm5, 1
1421b7158: addss xmm6, xmm13
1421b715d: movaps xmm3, xmm14
1421b7161: subss xmm3, xmm6
1421b7165: shufps xmm9, xmm3, 2
1421b716a: shufps xmm2, xmm2, 0
1421b716e: mulps xmm2, xmm11
1421b7172: movaps xmm3, xmm0
1421b7175: shufps xmm3, xmm0, 0
1421b7179: mulps xmm3, xmm4
1421b717c: shufps xmm0, xmm0, 0x55
1421b7180: mulps xmm0, xmm9
1421b7184: shufps xmm1, xmm1, 0xa4
1421b7188: movaps xmmword ptr [rbp + 0x160], xmm2
1421b718f: movaps xmmword ptr [rbp + 0x170], xmm3
1421b7196: movaps xmmword ptr [rbp + 0x180], xmm0
1421b719d: movaps xmmword ptr [rbp + 0x190], xmm1
1421b71a4: mov rcx, r13
1421b71a7: mov rdx, r15
1421b71aa: call 0x14089d560
1421b71af: movups xmm0, xmmword ptr [rbp + 0xc0]
1421b71b6: movups xmm1, xmmword ptr [rbp + 0xd0]
1421b71bd: movups xmm2, xmmword ptr [rbp + 0xe0]
1421b71c4: movaps xmmword ptr [rbp + 0x160], xmm0
1421b71cb: movaps xmmword ptr [rbp + 0x170], xmm1
1421b71d2: movaps xmmword ptr [rbp + 0x180], xmm2
1421b71d9: movaps xmm0, xmmword ptr [rip + 0x95dcc0]
1421b71e0: movaps xmmword ptr [rbp + 0x190], xmm0
1421b71e7: lea rcx, [rbp - 0x50]
1421b71eb: mov rdx, r15
1421b71ee: call 0x1421b6000
1421b71f3: or bl, al
1421b71f5: inc rsi
1421b71f8: mov rax, qword ptr [rbp + 0x120]
1421b71ff: cmp rax, qword ptr [rbp + 0x128]
1421b7206: jb 0x1421b6e20
1421b720c: jmp 0x1421b6d50
1421b7211: nop word ptr cs:[rax + rax]
1421b7220: mov rdx, qword ptr [rbp + 8]
1421b7224: test rdx, rdx
1421b7227: je 0x1421b723f
1421b7229: shl rdx, 2
1421b722d: mov r8d, 4
1421b7233: mov rcx, qword ptr [rbp + 0xb0]
1421b723a: call 0x140613c20
1421b723f: mov rcx, qword ptr [rbp + 0x68]
1421b7243: cmp byte ptr [rcx + 0x1c], 0
1421b7247: je 0x1421b726a
1421b7249: mov eax, dword ptr [rbp + 0xbc]
1421b724f: mov rcx, qword ptr [rbp + 0x60]
1421b7253: mov dword ptr [rcx], eax
1421b7255: mov rax, qword ptr [rbp + 0x58]
1421b7259: mov qword ptr [rax + 8], rsi
1421b725d: mov r11, qword ptr [rbp + 0x70]
1421b7261: mov r12, qword ptr [rbp + 0x78]
1421b7265: jmp 0x1421b69ef
1421b726a: call 0x1408e3c70
1421b726f: movss dword ptr [rbp + 0x1e8], xmm0
1421b7277: mov rcx, r15
1421b727a: mov rdx, qword ptr [rbp + 0x150]
1421b7281: mov r8, qword ptr [rbp + 0x158]
1421b7288: call 0x140c97ba0
1421b728d: mov rax, qword ptr [rbp + 0x168]
1421b7294: test rax, rax
1421b7297: js 0x1421b72a0
1421b7299: cvtsi2ss xmm15, rax
1421b729e: jmp 0x1421b72b6
1421b72a0: mov rcx, rax
1421b72a3: shr rcx, 1
1421b72a6: and eax, 1
1421b72a9: or rax, rcx
1421b72ac: cvtsi2ss xmm15, rax
1421b72b1: addss xmm15, xmm15
1421b72b6: lea rdx, [rbp + 0x110]
1421b72bd: mov rax, qword ptr [rbp + 0x160]
1421b72c4: mov qword ptr [rbp + 0x158], rax
1421b72cb: mov rcx, qword ptr [rbp + 0x170]
1421b72d2: mov rax, qword ptr [rbp + 0x178]
1421b72d9: mov qword ptr [rbp + 0x1e0], rax
1421b72e0: movaps xmm0, xmmword ptr [rbp + 0x80]
1421b72e7: addss xmm0, dword ptr [rbp + 0x1f4]
1421b72ef: movaps xmm8, xmm0
1421b72f3: mulss xmm8, dword ptr [rip + 0x7a9394]
1421b72fc: movss xmm1, dword ptr [rbp + 0x1e8]
1421b7304: addss xmm1, xmm0
1421b7308: movss dword ptr [rbp + 0x1e8], xmm1
1421b7310: addss xmm15, dword ptr [rip + 0x7a2dcb]
1421b7319: movaps xmm7, xmm8
1421b731d: unpcklps xmm7, xmm0
1421b7320: cmp rcx, qword ptr [rbp + 0x1e0]
1421b7327: jae 0x1421b7249
1421b732d: nop dword ptr [rax]
1421b7330: test rcx, rcx
1421b7333: js 0x1421b7350
1421b7335: mov r12, rcx
1421b7338: xorps xmm1, xmm1
1421b733b: cvtsi2ss xmm1, rcx
1421b7340: jmp 0x1421b736b
1421b7342: nop word ptr cs:[rax + rax]
1421b7350: mov rax, rcx
1421b7353: shr rax, 1
1421b7356: mov r12, rcx
1421b7359: and ecx, 1
1421b735c: or rcx, rax
1421b735f: xorps xmm1, xmm1
1421b7362: cvtsi2ss xmm1, rcx
1421b7367: addss xmm1, xmm1
1421b736b: divss xmm1, xmm15
1421b7370: movss dword ptr [rbp + 0xc0], xmm1
1421b7378: ucomiss xmm1, dword ptr [rip + 0x7a9311]
1421b737f: jb 0x1421b78ba
1421b7385: ucomiss xmm14, xmm1
1421b7389: jb 0x1421b78ba
1421b738f: ucomiss xmm1, xmm1
1421b7392: jp 0x1421b791a
1421b7398: mov rcx, qword ptr [rbp + 0x158]
1421b739f: call 0x140c975a0
1421b73a4: movss dword ptr [rbp + 0x160], xmm0
1421b73ac: movss dword ptr [rbp + 0x164], xmm1
1421b73b4: lea rcx, [rbp + 0xf4]
1421b73bb: mov rdx, r15
1421b73be: call 0x140c9cdc0
1421b73c3: mov dword ptr [rbp + 0xf0], 1
1421b73cd: movsd xmm9, qword ptr [rbp + 0xf4]
1421b73d6: movss xmm6, dword ptr [rbp + 0xfc]
1421b73de: addss xmm6, xmm8
1421b73e3: lea rcx, [rbp + 0x20]
1421b73e7: mov rdx, qword ptr [rbp + 0x150]
1421b73ee: movss xmm2, dword ptr [rbp + 0x1e8]
1421b73f6: call 0x140c9cde0
1421b73fb: addps xmm9, xmm7
1421b73ff: movsd xmm12, qword ptr [rbp + 0x20]
1421b7405: movss xmm10, dword ptr [rbp + 0x28]
1421b740b: movaps xmm0, xmm10
1421b740f: subss xmm0, xmm6
1421b7413: movaps xmm1, xmm0
1421b7416: mulss xmm1, xmm0
1421b741a: movaps xmm2, xmm12
1421b741e: subps xmm2, xmm9
1421b7422: movaps xmm3, xmm2
1421b7425: mulps xmm3, xmm2
1421b7428: movshdup xmm4, xmm3
1421b742c: addss xmm4, xmm3
1421b7430: addss xmm4, xmm1
1421b7434: xorps xmm1, xmm1
1421b7437: sqrtss xmm1, xmm4
1421b743b: movaps xmm3, xmm14
1421b743f: divss xmm3, xmm1
1421b7443: movsldup xmm1, xmm3
1421b7447: mulps xmm1, xmm2
1421b744a: mulss xmm3, xmm0
1421b744e: movlps qword ptr [rbp + 0x110], xmm1
1421b7455: movss dword ptr [rbp + 0x118], xmm3
1421b745d: mov rcx, r15
1421b7460: lea rdx, [rbp + 0x110]
1421b7467: call 0x140c90a40
1421b746c: movaps xmm0, xmm6
1421b746f: addss xmm0, xmm10
1421b7474: mulss xmm0, dword ptr [rip + 0x76e3f4]
1421b747c: movaps xmm1, xmm9
1421b7480: addps xmm1, xmm12
1421b7484: movaps xmm13, xmmword ptr [rbp + 0x160]
1421b748c: movss xmm3, dword ptr [rbp + 0x168]
1421b7494: subps xmm9, xmm12
1421b7498: subss xmm6, xmm10
1421b749d: mulps xmm9, xmm9
1421b74a1: movshdup xmm2, xmm9
1421b74a6: addss xmm2, xmm9
1421b74ab: mulss xmm6, xmm6
1421b74af: addss xmm6, xmm2
1421b74b3: xorps xmm2, xmm2
1421b74b6: sqrtss xmm2, xmm6
1421b74ba: mulps xmm1, xmmword ptr [rip + 0x76e41f]
1421b74c1: movlps qword ptr [rbp + 0xc0], xmm1
1421b74c8: movss dword ptr [rbp + 0xc8], xmm0
1421b74d0: movsd xmm0, qword ptr [rip + 0x8d5218]
1421b74d8: movsd qword ptr [rbp + 0xcc], xmm0
1421b74e0: movss dword ptr [rbp + 0xd4], xmm2
1421b74e8: movups xmm2, xmmword ptr [rbp + 0xcc]
1421b74ef: movaps xmm0, xmmword ptr [rbp + 0xd0]
1421b74f6: movaps xmm1, xmmword ptr [rbp + 0xc0]
1421b74fd: movaps xmm9, xmm3
1421b7501: addss xmm9, xmm3
1421b7506: mulss xmm3, xmm9
1421b750b: movaps xmm5, xmm13
1421b750f: addps xmm5, xmm13
1421b7513: movshdup xmm4, xmm5
1421b7517: mulss xmm4, xmm13
1421b751c: movaps xmm6, xmm13
1421b7520: movsldup xmm10, xmm9
1421b7525: mulps xmm10, xmm13
1421b7529: shufps xmm13, xmm13, 0xff
1421b752e: mulps xmm6, xmm5
1421b7531: mulps xmm5, xmm13
1421b7535: mulss xmm13, xmm9
1421b753a: shufps xmm5, xmm5, 0xe1
1421b753e: movshdup xmm12, xmm6
1421b7543: movaps xmm9, xmm12
1421b7547: addss xmm9, xmm3
1421b754c: movaps xmm11, xmm14
1421b7550: subss xmm11, xmm9
1421b7555: movaps xmm9, xmm4
1421b7559: addss xmm9, xmm13
1421b755e: unpcklps xmm11, xmm9
1421b7562: movaps xmm9, xmm10
1421b7566: subps xmm9, xmm5
1421b756a: shufps xmm11, xmm9, 4
1421b756f: subss xmm4, xmm13
1421b7574: addss xmm3, xmm6
1421b7578: movaps xmm13, xmm14
1421b757c: subss xmm13, xmm3
1421b7581: unpcklps xmm4, xmm13
1421b7585: addps xmm5, xmm10
1421b7589: shufps xmm4, xmm5, 0x54
1421b758d: shufps xmm9, xmm5, 1
1421b7592: addss xmm6, xmm12
1421b7597: movaps xmm3, xmm14
1421b759b: subss xmm3, xmm6
1421b759f: shufps xmm9, xmm3, 2
1421b75a4: shufps xmm2, xmm2, 0
1421b75a8: mulps xmm2, xmm11
1421b75ac: movaps xmm3, xmm0
1421b75af: shufps xmm3, xmm0, 0
1421b75b3: mulps xmm3, xmm4
1421b75b6: shufps xmm0, xmm0, 0x55
1421b75ba: mulps xmm0, xmm9
1421b75be: shufps xmm1, xmm1, 0xa4
1421b75c2: movaps xmmword ptr [rbp + 0x160], xmm2
1421b75c9: movaps xmmword ptr [rbp + 0x170], xmm3
1421b75d0: movaps xmmword ptr [rbp + 0x180], xmm0
1421b75d7: movaps xmmword ptr [rbp + 0x190], xmm1
1421b75de: lea rcx, [rbp + 0x110]
1421b75e5: mov rdx, r15
1421b75e8: call 0x14089d560
1421b75ed: movups xmm0, xmmword ptr [rbp + 0x110]
1421b75f4: movups xmm1, xmmword ptr [rbp + 0x120]
1421b75fb: movups xmm2, xmmword ptr [rbp + 0x130]
1421b7602: movaps xmmword ptr [rbp + 0x160], xmm0
1421b7609: movaps xmmword ptr [rbp + 0x170], xmm1
1421b7610: movaps xmmword ptr [rbp + 0x180], xmm2
1421b7617: movaps xmm0, xmmword ptr [rip + 0x95d882]
1421b761e: movaps xmmword ptr [rbp + 0x190], xmm0
1421b7625: lea rcx, [rbp - 0x50]
1421b7629: mov rdx, r15
1421b762c: call 0x1421b6000
1421b7631: inc r12
1421b7634: or bl, al
1421b7636: mov rcx, r12
1421b7639: inc rsi
1421b763c: lea rdx, [rbp + 0x110]
1421b7643: cmp rcx, qword ptr [rbp + 0x1e0]
1421b764a: jb 0x1421b7330
1421b7650: jmp 0x1421b7249
1421b7655: test bl, 1
1421b7658: je 0x1421b7727
1421b765e: mov rax, qword ptr [rip + 0x1474d43]
1421b7665: mov rax, qword ptr [rax]
1421b7668: test rax, rax
1421b766b: je 0x1421b7727
1421b7671: lea rcx, [rip + 0xe418d8]
1421b7678: call 0x14067dd80
1421b767d: movups xmm0, xmmword ptr [rax]
1421b7680: mov eax, dword ptr [rax + 0x10]
1421b7683: mov qword ptr [rbp + 0x190], 1
1421b768e: lea rcx, [rip + 0xe418d3]
1421b7695: mov qword ptr [rbp + 0x198], rcx
1421b769c: mov qword ptr [rbp + 0x1a0], 0x1f
1421b76a7: lea rdx, [rip + 0xe41892]
1421b76ae: mov qword ptr [rbp + 0x1b0], rdx
1421b76b5: mov qword ptr [rbp + 0x1b8], 1
1421b76c0: mov qword ptr [rbp + 0x1c0], 8
1421b76cb: mov qword ptr [rbp + 0x160], 0
1421b76d6: xorps xmm1, xmm1
1421b76d9: movups xmmword ptr [rbp + 0x1c8], xmm1
1421b76e0: mov qword ptr [rbp + 0x168], rcx
1421b76e7: mov qword ptr [rbp + 0x170], 0x1f
1421b76f2: mov qword ptr [rbp + 0x178], 0
1421b76fd: movups xmmword ptr [rbp + 0x180], xmm0
1421b7704: mov dword ptr [rbp + 0x1a8], 1
1421b770e: mov dword ptr [rbp + 0x1ac], eax
1421b7714: lea rcx, [rbp + 0x110]
1421b771b: lea rdx, [rbp + 0x160]
1421b7722: call 0x140d4a330
1421b7727: mov rdi, qword ptr [rbp - 0x30]
1421b772b: lea rsi, [rdi + 0x18]
1421b772f: mov rcx, rsi
1421b7732: call 0x140c81e10
1421b7737: xor ecx, ecx
1421b7739: mov al, 1
1421b773b: lock cmpxchg byte ptr [rdi + 0x18], cl
1421b7740: jne 0x1421b797a
1421b7746: movaps xmm6, xmmword ptr [rbp + 0x200]
1421b774d: movaps xmm7, xmmword ptr [rbp + 0x210]
1421b7754: movaps xmm8, xmmword ptr [rbp + 0x220]
1421b775c: movaps xmm9, xmmword ptr [rbp + 0x230]
1421b7764: movaps xmm10, xmmword ptr [rbp + 0x240]
1421b776c: movaps xmm11, xmmword ptr [rbp + 0x250]
1421b7774: movaps xmm12, xmmword ptr [rbp + 0x260]
1421b777c: movaps xmm13, xmmword ptr [rbp + 0x270]
1421b7784: movaps xmm14, xmmword ptr [rbp + 0x280]
1421b778c: movaps xmm15, xmmword ptr [rbp + 0x290]
1421b7794: add rsp, 0x328
1421b779b: pop rbx
1421b779c: pop rdi
1421b779d: pop rsi
1421b779e: pop r12
1421b77a0: pop r13
1421b77a2: pop r14
1421b77a4: pop r15
1421b77a6: pop rbp
1421b77a7: ret 
1421b77a8: lea rax, [rbp + 0xb8]
1421b77af: mov qword ptr [rbp + 0xf0], rax
1421b77b6: lea rax, [rip - 0x142aead]
1421b77bd: mov qword ptr [rbp + 0xf8], rax
1421b77c4: lea rax, [rip + 0xf4adfd]
1421b77cb: mov qword ptr [rbp + 0x160], rax
1421b77d2: mov qword ptr [rbp + 0x168], 1
1421b77dd: mov qword ptr [rbp + 0x180], 0
1421b77e8: lea rax, [rbp + 0xf0]
1421b77ef: mov qword ptr [rbp + 0x170], rax
1421b77f6: mov qword ptr [rbp + 0x178], 1
1421b7801: lea rdx, [rip + 0xf4ae40]
1421b7808: lea rcx, [rbp + 0x160]
1421b780f: call 0x1428d9390
1421b7814: jmp 0x1421b7932
1421b7819: lea rcx, [rip + 0xf4adb8]
1421b7820: lea r8, [rip + 0xf4ae21]
1421b7827: mov edx, 0x1d
1421b782c: call 0x1428d9430
1421b7831: jmp 0x1421b7932
1421b7836: mov qword ptr [rbp + 0x110], r13
1421b783d: lea rax, [rip - 0x142af34]
1421b7844: mov qword ptr [rbp + 0x118], rax
1421b784b: lea rax, [rip + 0xf4ad76]
1421b7852: mov qword ptr [rbp + 0x160], rax
1421b7859: mov qword ptr [rbp + 0x168], 1
1421b7864: mov qword ptr [rbp + 0x180], 0
1421b786f: lea rax, [rbp + 0x110]
1421b7876: mov qword ptr [rbp + 0x170], rax
1421b787d: mov qword ptr [rbp + 0x178], 1
1421b7888: lea rdx, [rip + 0xf4adb9]
1421b788f: lea rcx, [rbp + 0x160]
1421b7896: call 0x1428d9390
1421b789b: jmp 0x1421b7932
1421b78a0: lea rcx, [rip + 0xf4ad31]
1421b78a7: lea r8, [rip + 0xf4ad9a]
1421b78ae: mov edx, 0x1d
1421b78b3: call 0x1428d9430
1421b78b8: jmp 0x1421b7932
1421b78ba: mov qword ptr [rbp + 0x110], r13
1421b78c1: lea rax, [rip - 0x142afb8]
1421b78c8: mov qword ptr [rbp + 0x118], rax
1421b78cf: lea rax, [rip + 0xf4acf2]
1421b78d6: mov qword ptr [rbp + 0x160], rax
1421b78dd: mov qword ptr [rbp + 0x168], 1
1421b78e8: mov qword ptr [rbp + 0x180], 0
1421b78f3: mov qword ptr [rbp + 0x170], rdx
1421b78fa: mov qword ptr [rbp + 0x178], 1
1421b7905: lea rdx, [rip + 0xf4ad3c]
1421b790c: lea rcx, [rbp + 0x160]
1421b7913: call 0x1428d9390
1421b7918: jmp 0x1421b7932
1421b791a: lea rcx, [rip + 0xf4acb7]
1421b7921: lea r8, [rip + 0xf4ad20]
1421b7928: mov edx, 0x1d
1421b792d: call 0x1428d9430
1421b7932: ud2 
1421b7934: mov qword ptr [rbp + 0x160], 0
1421b793f: add rbx, 0x10
1421b7943: lea rax, [rip + 0xe412de]
1421b794a: mov qword ptr [rsp + 0x20], rax
1421b794f: lea rdx, [rbp - 0x50]
1421b7953: lea r9, [rbp + 0x160]
1421b795a: xor ecx, ecx
1421b795c: mov r8, rbx
1421b795f: call 0x1428cab41
1421b7964: mov r14, rcx
1421b7967: mov r8d, 0x3b9aca00
1421b796d: call 0x1428cd210
1421b7972: mov rcx, r14
1421b7975: jmp 0x1421b68b8
1421b797a: mov rcx, rsi
1421b797d: xor edx, edx
1421b797f: call 0x1428cdb90
1421b7984: jmp 0x1421b7746
1421b7989: lea rcx, [rip + 0xe41548]
1421b7990: call 0x1428d9310
1421b7995: lea rcx, [rip + 0xe41554]
1421b799c: call 0x1428d9310
1421b79a1: lea rcx, [rip + 0xe41420]
1421b79a8: call 0x1428d9310
1421b79ad: lea rcx, [rip + 0xe4142c]
1421b79b4: call 0x1428d9310
1421b79b9: int3 
1421b79ba: nop word ptr [rax + rax]
1421b79c0: mov qword ptr [rsp + 0x10], rdx
1421b79c5: push rbp
1421b79c6: push r15
1421b79c8: push r14
1421b79ca: push r13
1421b79cc: push r12
1421b79ce: push rsi
1421b79cf: push rdi
1421b79d0: push rbx
1421b79d1: sub rsp, 0xc8
1421b79d8: lea rbp, [rdx + 0x80]
1421b79df: movaps xmmword ptr [rsp + 0x20], xmm15
1421b79e5: movaps xmmword ptr [rsp + 0x30], xmm14
1421b79eb: movaps xmmword ptr [rsp + 0x40], xmm13
1421b79f1: movaps xmmword ptr [rsp + 0x50], xmm12
1421b79f7: movaps xmmword ptr [rsp + 0x60], xmm11
1421b79fd: movaps xmmword ptr [rsp + 0x70], xmm10
1421b7a03: movaps xmmword ptr [rsp + 0x80], xmm9
1421b7a0c: movaps xmmword ptr [rsp + 0x90], xmm8
1421b7a15: movaps xmmword ptr [rsp + 0xa0], xmm7
1421b7a1d: movaps xmmword ptr [rsp + 0xb0], xmm6
1421b7a25: mov rdi, qword ptr [rbp - 0x30]
1421b7a29: lea rsi, [rdi + 0x18]
1421b7a2d: mov rcx, rsi
1421b7a30: call 0x140c81e10
1421b7a35: xor ecx, ecx
1421b7a37: mov al, 1
1421b7a39: lock cmpxchg byte ptr [rdi + 0x18], cl
1421b7a3e: jne 0x1421b7a9a
1421b7a40: movaps xmm6, xmmword ptr [rsp + 0xb0]
1421b7a48: movaps xmm7, xmmword ptr [rsp + 0xa0]
1421b7a50: movaps xmm8, xmmword ptr [rsp + 0x90]
1421b7a59: movaps xmm9, xmmword ptr [rsp + 0x80]
1421b7a62: movaps xmm10, xmmword ptr [rsp + 0x70]
1421b7a68: movaps xmm11, xmmword ptr [rsp + 0x60]
1421b7a6e: movaps xmm12, xmmword ptr [rsp + 0x50]
1421b7a74: movaps xmm13, xmmword ptr [rsp + 0x40]
1421b7a7a: movaps xmm14, xmmword ptr [rsp + 0x30]
1421b7a80: movaps xmm15, xmmword ptr [rsp + 0x20]
1421b7a86: add rsp, 0xc8
1421b7a8d: pop rbx
1421b7a8e: pop rdi
1421b7a8f: pop rsi
1421b7a90: pop r12
1421b7a92: pop r13
1421b7a94: pop r14
1421b7a96: pop r15
1421b7a98: pop rbp
1421b7a99: ret 
1421b7a9a: mov rcx, rsi
1421b7a9d: xor edx, edx
1421b7a9f: call 0x1428cdb90
1421b7aa4: jmp 0x1421b7a40
1421b7aa6: nop word ptr cs:[rax + rax]
1421b7ab0: mov qword ptr [rsp + 0x10], rdx
1421b7ab5: push rbp
1421b7ab6: push r15
1421b7ab8: push r14
1421b7aba: push r13
1421b7abc: push r12
1421b7abe: push rsi
1421b7abf: push rdi
1421b7ac0: push rbx
1421b7ac1: sub rsp, 0xc8
1421b7ac8: lea rbp, [rdx + 0x80]
1421b7acf: movaps xmmword ptr [rsp + 0x20], xmm15
1421b7ad5: movaps xmmword ptr [rsp + 0x30], xmm14
1421b7adb: movaps xmmword ptr [rsp + 0x40], xmm13
1421b7ae1: movaps xmmword ptr [rsp + 0x50], xmm12
1421b7ae7: movaps xmmword ptr [rsp + 0x60], xmm11
1421b7aed: movaps xmmword ptr [rsp + 0x70], xmm10
1421b7af3: movaps xmmword ptr [rsp + 0x80], xmm9
1421b7afc: movaps xmmword ptr [rsp + 0x90], xmm8
1421b7b05: movaps xmmword ptr [rsp + 0xa0], xmm7
1421b7b0d: movaps xmmword ptr [rsp + 0xb0], xmm6
1421b7b15: mov rdx, qword ptr [rbp + 8]
1421b7b19: test rdx, rdx
1421b7b1c: je 0x1421b7b34
1421b7b1e: shl rdx, 2
1421b7b22: mov r8d, 4
1421b7b28: mov rcx, qword ptr [rbp + 0xb0]
1421b7b2f: call 0x140613c20
1421b7b34: movaps xmm6, xmmword ptr [rsp + 0xb0]
1421b7b3c: movaps xmm7, xmmword ptr [rsp + 0xa0]
1421b7b44: movaps xmm8, xmmword ptr [rsp + 0x90]
1421b7b4d: movaps xmm9, xmmword ptr [rsp + 0x80]
1421b7b56: movaps xmm10, xmmword ptr [rsp + 0x70]
1421b7b5c: movaps xmm11, xmmword ptr [rsp + 0x60]
1421b7b62: movaps xmm12, xmmword ptr [rsp + 0x50]
1421b7b68: movaps xmm13, xmmword ptr [rsp + 0x40]
1421b7b6e: movaps xmm14, xmmword ptr [rsp + 0x30]
1421b7b74: movaps xmm15, xmmword ptr [rsp + 0x20]
1421b7b7a: add rsp, 0xc8
1421b7b81: pop rbx
1421b7b82: pop rdi
1421b7b83: pop rsi
1421b7b84: pop r12
1421b7b86: pop r13
1421b7b88: pop r14
1421b7b8a: pop r15
1421b7b8c: pop rbp
1421b7b8d: ret 
