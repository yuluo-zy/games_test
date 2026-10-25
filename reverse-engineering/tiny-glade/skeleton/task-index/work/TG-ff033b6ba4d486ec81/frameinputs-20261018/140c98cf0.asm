140c98cf0: push rbp
140c98cf1: push rsi
140c98cf2: sub rsp, 0xf8
140c98cf9: lea rbp, [rsp + 0x80]
140c98d01: movaps xmmword ptr [rbp + 0x60], xmm10
140c98d06: movaps xmmword ptr [rbp + 0x50], xmm9
140c98d0b: movaps xmmword ptr [rbp + 0x40], xmm8
140c98d10: movaps xmmword ptr [rbp + 0x30], xmm7
140c98d14: movaps xmmword ptr [rbp + 0x20], xmm6
140c98d18: mov qword ptr [rbp + 0x18], 0xfffffffffffffffe
140c98d20: movss xmm6, dword ptr [rdx + 0x10]
140c98d25: movss xmm8, dword ptr [rdx + 0x14]
140c98d2b: movaps xmm0, xmm6
140c98d2e: mulss xmm0, xmm8
140c98d33: xorps xmm1, xmm1
140c98d36: ucomiss xmm0, xmm1
140c98d39: jbe 0x140c98eba
140c98d3f: mov rsi, rcx
140c98d42: movsd xmm9, qword ptr [rdx]
140c98d47: movsd xmm7, qword ptr [rdx + 8]
140c98d4c: movzx eax, byte ptr [rip + 0x299beb6]
140c98d53: mov ecx, 0x20
140c98d58: mov edx, 4
140c98d5d: call 0x140613c10
140c98d62: test rax, rax
140c98d65: je 0x140c98ed2
140c98d6b: movshdup xmm0, xmm9
140c98d70: xorps xmm0, xmmword ptr [rip + 0x1c8cb79]
140c98d77: movaps xmm1, xmm9
140c98d7b: movlhps xmm1, xmm0
140c98d7e: shufps xmm9, xmm1, 0x24
140c98d83: movss xmm1, dword ptr [rip + 0x1cc135d]
140c98d8b: movaps xmm2, xmm6
140c98d8e: mulss xmm2, xmm1
140c98d92: movaps xmm0, xmm2
140c98d95: shufps xmm0, xmm2, 0
140c98d99: mulss xmm1, xmm8
140c98d9e: shufps xmm2, xmm1, 0
140c98da2: mulps xmm2, xmm9
140c98da6: movaps xmm10, xmm2
140c98daa: unpckhpd xmm10, xmm2
140c98daf: addps xmm10, xmm2
140c98db3: addps xmm10, xmm7
140c98db7: movss xmm2, dword ptr [rip + 0x1c8cab1]
140c98dbf: mulss xmm6, xmm2
140c98dc3: movaps xmm3, xmm6
140c98dc6: shufps xmm3, xmm1, 0
140c98dca: mulps xmm3, xmm9
140c98dce: movaps xmm1, xmm3
140c98dd1: unpckhpd xmm1, xmm3
140c98dd5: addps xmm1, xmm3
140c98dd8: movddup xmm3, xmm7
140c98ddc: mulss xmm8, xmm2
140c98de1: shufps xmm0, xmm8, 4
140c98de6: mulps xmm0, xmm9
140c98dea: shufps xmm6, xmm8, 0
140c98def: mulps xmm6, xmm9
140c98df3: movlps qword ptr [rax], xmm10
140c98df7: movaps xmm2, xmm0
140c98dfa: unpckhpd xmm2, xmm6
140c98dfe: movlhps xmm0, xmm6
140c98e01: addps xmm0, xmm2
140c98e04: addps xmm0, xmm3
140c98e07: movups xmmword ptr [rax + 8], xmm0
140c98e0b: addps xmm1, xmm7
140c98e0e: movlps qword ptr [rax + 0x18], xmm1
140c98e12: mov qword ptr [rbp - 8], 4
140c98e1a: mov qword ptr [rbp], rax
140c98e1e: mov qword ptr [rbp + 8], 4
140c98e26: mov byte ptr [rbp + 0x17], 1
140c98e2a: lea rdx, [rip + 0x1e958e7]
140c98e31: lea rcx, [rbp - 8]
140c98e35: call 0x140562e70
140c98e3a: mov rax, qword ptr [rbp]
140c98e3e: movlps qword ptr [rax + 0x20], xmm10
140c98e43: mov qword ptr [rbp + 8], 5
140c98e4b: movups xmm0, xmmword ptr [rbp - 8]
140c98e4f: movaps xmmword ptr [rbp - 0x20], xmm0
140c98e53: mov qword ptr [rbp - 0x10], 5
140c98e5b: mov byte ptr [rbp + 0x17], 0
140c98e5f: lea rcx, [rbp - 0x58]
140c98e63: lea rdx, [rbp - 0x20]
140c98e67: xor r8d, r8d
140c98e6a: call 0x1419401b0
140c98e6f: xor eax, eax
140c98e71: cmp rax, qword ptr [rbp - 0x58]
140c98e75: jo 0x140c98ee1
140c98e77: mov rax, qword ptr [rbp - 0x28]
140c98e7b: mov qword ptr [rsi + 0x30], rax
140c98e7f: movups xmm0, xmmword ptr [rbp - 0x58]
140c98e83: movups xmm1, xmmword ptr [rbp - 0x48]
140c98e87: movups xmm2, xmmword ptr [rbp - 0x38]
140c98e8b: movups xmmword ptr [rsi + 0x20], xmm2
140c98e8f: movups xmmword ptr [rsi + 0x10], xmm1
140c98e93: movups xmmword ptr [rsi], xmm0
140c98e96: mov rax, rsi
140c98e99: movaps xmm6, xmmword ptr [rbp + 0x20]
140c98e9d: movaps xmm7, xmmword ptr [rbp + 0x30]
140c98ea1: movaps xmm8, xmmword ptr [rbp + 0x40]
140c98ea6: movaps xmm9, xmmword ptr [rbp + 0x50]
140c98eab: movaps xmm10, xmmword ptr [rbp + 0x60]
140c98eb0: add rsp, 0xf8
140c98eb7: pop rsi
140c98eb8: pop rbp
140c98eb9: ret 
140c98eba: lea rcx, [rip + 0x1e957ab]
140c98ec1: lea r8, [rip + 0x1e95838]
140c98ec8: mov edx, 0x23
140c98ecd: call 0x1428d9430
140c98ed2: mov ecx, 4
140c98ed7: mov edx, 0x20
140c98edc: call 0x1428d9000
140c98ee1: movups xmm0, xmmword ptr [rbp - 0x50]
140c98ee5: movaps xmmword ptr [rbp - 0x20], xmm0
140c98ee9: mov byte ptr [rbp + 0x17], 0
140c98eed: lea rax, [rip + 0x1e9583c]
140c98ef4: mov qword ptr [rsp + 0x20], rax
140c98ef9: lea rcx, [rip + 0x21d85b0]
140c98f00: lea r9, [rip + 0x21d8589]
140c98f07: lea r8, [rbp - 0x20]
140c98f0b: mov edx, 0x2b
140c98f10: call 0x1428d9760
140c98f15: ud2 
140c98f17: nop word ptr [rax + rax]
140c98f20: mov qword ptr [rsp + 0x10], rdx
140c98f25: push rbp
140c98f26: push rsi
140c98f27: sub rsp, 0x78
140c98f2b: lea rbp, [rdx + 0x80]
140c98f32: movaps xmmword ptr [rsp + 0x20], xmm10
140c98f38: movaps xmmword ptr [rsp + 0x30], xmm9
140c98f3e: movaps xmmword ptr [rsp + 0x40], xmm8
140c98f44: movaps xmmword ptr [rsp + 0x50], xmm7
140c98f49: movaps xmmword ptr [rsp + 0x60], xmm6
140c98f4e: cmp byte ptr [rbp + 0x17], 0
140c98f52: je 0x140c98f70
140c98f54: mov rdx, qword ptr [rbp - 8]
140c98f58: test rdx, rdx
140c98f5b: je 0x140c98f70
140c98f5d: mov rcx, qword ptr [rbp]
140c98f61: shl rdx, 3
140c98f65: mov r8d, 4
140c98f6b: call 0x140613c20
140c98f70: movaps xmm6, xmmword ptr [rsp + 0x60]
140c98f75: movaps xmm7, xmmword ptr [rsp + 0x50]
140c98f7a: movaps xmm8, xmmword ptr [rsp + 0x40]
140c98f80: movaps xmm9, xmmword ptr [rsp + 0x30]
140c98f86: movaps xmm10, xmmword ptr [rsp + 0x20]
140c98f8c: add rsp, 0x78
140c98f90: pop rsi
140c98f91: pop rbp
140c98f92: ret 
