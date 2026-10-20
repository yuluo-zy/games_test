0x141277bb0: push r15
0x141277bb2: push r14
0x141277bb4: push r13
0x141277bb6: push r12
0x141277bb8: push rsi
0x141277bb9: push rdi
0x141277bba: push rbp
0x141277bbb: push rbx
0x141277bbc: sub rsp, 0xf8
0x141277bc3: movaps xmmword ptr [rsp + 0xe0], xmm9
0x141277bcc: movaps xmmword ptr [rsp + 0xd0], xmm8
0x141277bd5: movaps xmmword ptr [rsp + 0xc0], xmm7
0x141277bdd: movaps xmmword ptr [rsp + 0xb0], xmm6
0x141277be5: mov rdi, r8
0x141277be8: mov rsi, rcx
0x141277beb: mov r13, qword ptr [rsp + 0x188]
0x141277bf3: movzx eax, byte ptr [r8 + 0x22]
0x141277bf8: lea ecx, [rax - 2]
0x141277bfb: dec rax
0x141277bfe: xor r12d, r12d
0x141277c01: cmp cl, 3
0x141277c04: cmovb r12, rax
0x141277c08: mov rbx, qword ptr [rsp + 0x168]
0x141277c10: lea rax, [rip + 0x19abd79]
0x141277c17: movsxd rcx, dword ptr [rax + r12*4]
0x141277c1b: add rcx, rax
0x141277c1e: jmp rcx
0x141277c20: mov eax, dword ptr [rdx + 0x18]
0x141277c23: mov dword ptr [rsp + 0x70], eax
0x141277c27: mov rax, qword ptr [rdx + 0x10]
0x141277c2b: mov qword ptr [rsp + 0x68], rax
0x141277c30: mov r14, qword ptr [r13 + 0x20]
0x141277c34: mov rcx, r14
0x141277c37: call 0x140949c10
0x141277c3c: movaps xmm2, xmm1
0x141277c3f: mov r15, qword ptr [r13]
0x141277c43: lea rcx, [rsp + 0x68]
0x141277c48: movaps xmm1, xmm0
0x141277c4b: mov r9, r15
0x141277c4e: call 0x140ac8780
0x141277c53: movaps xmm6, xmm0
0x141277c56: unpcklps xmm6, xmm1
0x141277c59: mov rax, qword ptr [rsp + 0x190]
0x141277c61: movss xmm0, dword ptr [rsp + 0x180]
0x141277c6a: movss xmm3, dword ptr [rsp + 0x178]
0x141277c73: movzx ebp, byte ptr [rdi + 0x46]
0x141277c77: movaps xmm7, xmm3
0x141277c7a: unpcklps xmm7, xmm0
0x141277c7d: subps xmm6, xmm7
0x141277c80: movshdup xmm2, xmm6
0x141277c84: mov qword ptr [rsp + 0x38], rax
0x141277c89: mov qword ptr [rsp + 0x30], r13
0x141277c8e: mov byte ptr [rsp + 0x28], bpl
0x141277c93: movss dword ptr [rsp + 0x20], xmm0
0x141277c99: lea rcx, [rsp + 0x68]
0x141277c9e: movaps xmm1, xmm6
0x141277ca1: call 0x141278050
0x141277ca6: mov eax, dword ptr [rsp + 0x88]
0x141277cad: mov dword ptr [rsp + 0x50], eax
0x141277cb1: mov rax, qword ptr [rsp + 0x80]
0x141277cb9: mov qword ptr [rsp + 0x48], rax
0x141277cbe: mov rcx, r14
0x141277cc1: call 0x140949c10
0x141277cc6: movaps xmm2, xmm1
0x141277cc9: lea rcx, [rsp + 0x48]
0x141277cce: movaps xmm1, xmm0
0x141277cd1: mov r9, r15
0x141277cd4: call 0x140ac8780
0x141277cd9: lea eax, [rbp - 0x38]
0x141277cdc: cmp al, 8
0x141277cde: setae cl
0x141277ce1: cmp al, 6
0x141277ce3: sete al
0x141277ce6: or al, cl
0x141277ce8: jne 0x141277cf4
0x141277cea: cmp bpl, 0x3f
0x141277cee: jne 0x141277e92
0x141277cf4: addps xmm7, xmm6
0x141277cf7: unpcklps xmm0, xmm1
0x141277cfa: subps xmm7, xmm0
0x141277cfd: addps xmm7, xmm6
0x141277d00: movlps qword ptr [rsi + 4], xmm7
0x141277d04: mov ebp, 1
0x141277d09: mov dword ptr [rsi], ebp
0x141277d0b: mov rax, rsi
0x141277d0e: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x141277d16: movaps xmm7, xmmword ptr [rsp + 0xc0]
0x141277d1e: movaps xmm8, xmmword ptr [rsp + 0xd0]
0x141277d27: movaps xmm9, xmmword ptr [rsp + 0xe0]
0x141277d30: add rsp, 0xf8
0x141277d37: pop rbx
0x141277d38: pop rbp
0x141277d39: pop rdi
0x141277d3a: pop rsi
0x141277d3b: pop r12
0x141277d3d: pop r13
0x141277d3f: pop r14
0x141277d41: pop r15
0x141277d43: ret 
0x141277d44: mov rbp, rdx
0x141277d47: movsd xmm6, qword ptr [rdi]
0x141277d4b: mov rcx, rbx
0x141277d4e: mov r14, r9
0x141277d51: call 0x140a1c240
0x141277d56: lea r8, [rip + 0x19abbd3]
0x141277d5d: mov rcx, r14
0x141277d60: mov rdx, rax
0x141277d63: call 0x140a91ef0
0x141277d68: test rax, rax
0x141277d6b: je 0x14127802d
0x141277d71: cmp dword ptr [rax + 0x258], 2
0x141277d78: jae 0x141277ee3
0x141277d7e: mov r14, rax
0x141277d81: mov r15, qword ptr [rsp + 0x160]
0x141277d89: movlps qword ptr [rsp + 0xa8], xmm6
0x141277d91: mov rcx, rbx
0x141277d94: call 0x140a1c240
0x141277d99: mov rcx, r15
0x141277d9c: mov rdx, rax
0x141277d9f: call 0x140a91e00
0x141277da4: mov rdx, rax
0x141277da7: shr rdx, 0x20
0x141277dab: je 0x141278039
0x141277db1: mov r11, qword ptr [rsp + 0x170]
0x141277db9: mov rcx, qword ptr [r11 + 8]
0x141277dbd: mov r9d, eax
0x141277dc0: cmp r9, qword ptr [rcx + 0x10]
0x141277dc4: jae 0x141277fe5
0x141277dca: mov r8, qword ptr [rcx + 8]
0x141277dce: mov rdx, rax
0x141277dd1: shr rdx, 0x20
0x141277dd5: lea r10, [r9 + r9*4]
0x141277dd9: cmp dword ptr [r8 + r10*4], edx
0x141277ddd: jne 0x141277fe5
0x141277de3: lea r10, [r8 + r10*4]
0x141277de7: mov r8d, dword ptr [r10 + 4]
0x141277deb: mov r15d, 0xffffffff
0x141277df1: cmp r8, r15
0x141277df4: je 0x141277fe5
0x141277dfa: mov rdx, qword ptr [r11]
0x141277dfd: xor r9d, r9d
0x141277e00: cmp qword ptr [rdx + 0x38], r8
0x141277e04: jbe 0x141277ff5
0x141277e0a: mov r11, qword ptr [rdx + 0x28]
0x141277e0e: mov r15d, r8d
0x141277e11: shr r15d, 6
0x141277e15: mov r11, qword ptr [r11 + r15*8]
0x141277e19: bt r11, r8
0x141277e1d: jae 0x141277ff5
0x141277e23: mov eax, dword ptr [r10 + 0xc]
0x141277e27: mov r8d, dword ptr [r10 + 0x10]
0x141277e2b: mov rcx, qword ptr [rcx + 0x1a8]
0x141277e32: lea rax, [rax + rax*8]
0x141277e36: mov rdx, qword ptr [rdx + 0x110]
0x141277e3d: mov r9, qword ptr [rcx + rax*8 + 0x18]
0x141277e42: mov rax, qword ptr [rcx + rax*8 + 0x38]
0x141277e47: mov rax, qword ptr [rax + rdx*8]
0x141277e4b: not rax
0x141277e4e: lea rax, [rax + rax*2]
0x141277e52: shl rax, 4
0x141277e56: imul rdx, r8, 0x68
0x141277e5a: add rdx, qword ptr [r9 + rax + 0x10]
0x141277e5f: movshdup xmm3, xmm6
0x141277e63: lea rcx, [rsp + 0x68]
0x141277e68: movaps xmm2, xmm6
0x141277e6b: call 0x1408ec0a0
0x141277e70: test byte ptr [rsp + 0x68], 1
0x141277e75: je 0x141277f18
0x141277e7b: mov eax, dword ptr [rsp + 0x74]
0x141277e7f: mov dword ptr [rsp + 0x60], eax
0x141277e83: mov rax, qword ptr [rsp + 0x6c]
0x141277e88: mov qword ptr [rsp + 0x58], rax
0x141277e8d: jmp 0x141277fc8
0x141277e92: movzx eax, byte ptr [rsp + 0x68]
0x141277e97: mov r14, qword ptr [rsp + 0x70]
0x141277e9c: cmp r12, 2
0x141277ea0: jae 0x141277ed2
0x141277ea2: xor ebp, ebp
0x141277ea4: test al, 0xfd
0x141277ea6: jne 0x141277d09
0x141277eac: mov rcx, rbx
0x141277eaf: movaps xmm8, xmm0
0x141277eb3: movaps xmm9, xmm1
0x141277eb7: call 0x140a1c240
0x141277ebc: movaps xmm1, xmm9
0x141277ec0: movaps xmm0, xmm8
0x141277ec4: cmp r14, rax
0x141277ec7: je 0x141277cf4
0x141277ecd: jmp 0x141277d09
0x141277ed2: jne 0x141277ef3
0x141277ed4: xor ebp, ebp
0x141277ed6: cmp al, 0xb
0x141277ed8: je 0x141277cf4
0x141277ede: jmp 0x141277d09
0x141277ee3: mov eax, dword ptr [rbp + 0x18]
0x141277ee6: mov dword ptr [rsp + 0x70], eax
0x141277eea: mov rax, qword ptr [rbp + 0x10]
0x141277eee: jmp 0x141277c2b
0x141277ef3: xor ebp, ebp
0x141277ef5: cmp al, 0xd
0x141277ef7: jne 0x141277d09
0x141277efd: mov eax, dword ptr [rdi]
0x141277eff: cmp dword ptr [rsp + 0x6c], eax
0x141277f03: jne 0x141277d09
0x141277f09: cmp dword ptr [rdi + 4], r14d
0x141277f0d: je 0x141277cf4
0x141277f13: jmp 0x141277d09
0x141277f18: lea rcx, [rsp + 0x48]
0x141277f1d: lea rdx, [rsp + 0xa8]
0x141277f25: mov r8, r14
0x141277f28: call 0x140c92090
0x141277f2d: movaps xmm0, xmmword ptr [rbp]
0x141277f31: movaps xmm2, xmm0
0x141277f34: mulps xmm2, xmm0
0x141277f37: movshdup xmm1, xmm2
0x141277f3b: addss xmm1, xmm2
0x141277f3f: movaps xmm3, xmm2
0x141277f42: unpckhpd xmm3, xmm2
0x141277f46: addss xmm3, xmm1
0x141277f4a: shufps xmm3, xmm3, 0
0x141277f4e: shufps xmm2, xmm2, 0xff
0x141277f52: subps xmm2, xmm3
0x141277f55: movaps xmm1, xmmword ptr [rip + 0x19ab864]
0x141277f5c: mulps xmm2, xmm1
0x141277f5f: mulps xmm1, xmm0
0x141277f62: movshdup xmm3, xmm1
0x141277f66: addss xmm3, xmm1
0x141277f6a: movhlps xmm1, xmm1
0x141277f6d: addss xmm1, xmm3
0x141277f71: addss xmm1, xmm1
0x141277f75: shufps xmm1, xmm1, 0
0x141277f79: mulps xmm1, xmm0
0x141277f7c: addps xmm1, xmm2
0x141277f7f: movss xmm2, dword ptr [rip + 0x182b2f9]
0x141277f87: movaps xmm3, xmm0
0x141277f8a: mulps xmm3, xmm2
0x141277f8d: movaps xmm4, xmm0
0x141277f90: shufps xmm4, xmm0, 0xc9
0x141277f94: mulps xmm4, xmm2
0x141277f97: shufps xmm3, xmm3, 0xd2
0x141277f9b: subps xmm4, xmm3
0x141277f9e: addps xmm0, xmm0
0x141277fa1: shufps xmm0, xmm0, 0xff
0x141277fa5: mulps xmm0, xmm4
0x141277fa8: addps xmm0, xmm1
0x141277fab: movsd xmm1, qword ptr [rsp + 0x48]
0x141277fb1: addps xmm1, xmm0
0x141277fb4: movhlps xmm0, xmm0
0x141277fb7: addss xmm0, dword ptr [rsp + 0x50]
0x141277fbd: movlps qword ptr [rsp + 0x58], xmm1
0x141277fc2: movss dword ptr [rsp + 0x60], xmm0
0x141277fc8: mov r14, qword ptr [r13 + 0x20]
0x141277fcc: mov rcx, r14
0x141277fcf: call 0x140949c10
0x141277fd4: movaps xmm2, xmm1
0x141277fd7: mov r15, qword ptr [r13]
0x141277fdb: lea rcx, [rsp + 0x58]
0x141277fe0: jmp 0x141277c48
0x141277fe5: shl rdx, 0x20
0x141277fe9: or rdx, r9
0x141277fec: mov r9d, 1
0x141277ff2: mov rax, rdx
0x141277ff5: mov dword ptr [rsp + 0x68], r9d
0x141277ffa: mov dword ptr [rsp + 0x6c], r8d
0x141277fff: mov qword ptr [rsp + 0x70], rax
0x141278004: lea rax, [rip + 0x19ab96d]
0x14127800b: mov qword ptr [rsp + 0x20], rax
0x141278010: lea rcx, [rip + 0x19ab7d9]
0x141278017: lea r9, [rip + 0x19ab7b2]
0x14127801e: lea r8, [rsp + 0x68]
0x141278023: mov edx, 0x2b
0x141278028: call 0x1428d9760
0x14127802d: lea rcx, [rip + 0x19ab914]
0x141278034: call 0x1428d9310
0x141278039: lea rcx, [rip + 0x19ab920]
0x141278040: call 0x1428d9310
0x141278045: int3 
