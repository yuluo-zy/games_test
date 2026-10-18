140c90a40: push rsi
140c90a41: sub rsp, 0x90
140c90a48: movaps xmmword ptr [rsp + 0x80], xmm10
140c90a51: movaps xmmword ptr [rsp + 0x70], xmm9
140c90a57: movaps xmmword ptr [rsp + 0x60], xmm8
140c90a5d: movaps xmmword ptr [rsp + 0x50], xmm7
140c90a62: movaps xmmword ptr [rsp + 0x40], xmm6
140c90a67: mov rsi, rcx
140c90a6a: movsd xmm0, qword ptr [rdx]
140c90a6e: movshdup xmm8, xmm0
140c90a73: movd eax, xmm8
140c90a78: movd r8d, xmm0
140c90a7d: movd xmm1, dword ptr [rdx + 8]
140c90a82: movd ecx, xmm1
140c90a86: and r8d, 0x7fffffff
140c90a8d: and eax, 0x7fffffff
140c90a92: cmp eax, 0x7f800000
140c90a97: setl al
140c90a9a: and ecx, 0x7fffffff
140c90aa0: cmp ecx, 0x7f800000
140c90aa6: setl cl
140c90aa9: cmp r8d, 0x7f7fffff
140c90ab0: jg 0x140c90dcf
140c90ab6: test al, al
140c90ab8: je 0x140c90dcf
140c90abe: test cl, cl
140c90ac0: je 0x140c90dcf
140c90ac6: xorps xmm6, xmm6
140c90ac9: movdqa xmm4, xmm1
140c90acd: mulss xmm4, xmm6
140c90ad1: xorps xmm5, xmm5
140c90ad4: mulps xmm5, xmm0
140c90ad7: movshdup xmm2, xmm5
140c90adb: subss xmm2, xmm1
140c90adf: movsldup xmm3, xmm0
140c90ae3: movss xmm3, xmm4
140c90ae7: subps xmm3, xmm5
140c90aea: movaps xmm4, xmm2
140c90aed: mulss xmm4, xmm2
140c90af1: movaps xmm5, xmm3
140c90af4: mulps xmm5, xmm3
140c90af7: addss xmm4, xmm5
140c90afb: movshdup xmm5, xmm5
140c90aff: addss xmm5, xmm4
140c90b03: xorps xmm7, xmm7
140c90b06: sqrtss xmm7, xmm5
140c90b0a: movss xmm4, dword ptr [rip + 0x1c94df2]
140c90b12: movaps xmm5, xmm4
140c90b15: divss xmm5, xmm7
140c90b19: movd eax, xmm5
140c90b1d: test eax, eax
140c90b1f: setns cl
140c90b22: mov edx, eax
140c90b24: and edx, 0x7fffffff
140c90b2a: add edx, 0xff800000
140c90b30: cmp edx, 0x7f000000
140c90b36: setb dl
140c90b39: and dl, cl
140c90b3b: dec eax
140c90b3d: cmp eax, 0x7fffff
140c90b42: setb al
140c90b45: or al, dl
140c90b47: movsldup xmm7, xmm5
140c90b4b: mulps xmm7, xmm3
140c90b4e: movzx eax, al
140c90b51: movd xmm3, eax
140c90b55: pshufd xmm3, xmm3, 0x50
140c90b5a: pslld xmm3, 0x1f
140c90b5f: psrad xmm3, 0x1f
140c90b64: pand xmm3, xmm7
140c90b68: pshufd xmm7, xmm3, 0xf5
140c90b6d: test al, al
140c90b6f: jne 0x140c90b76
140c90b71: movaps xmm2, xmm4
140c90b74: jmp 0x140c90b7a
140c90b76: mulss xmm2, xmm5
140c90b7a: movaps xmm5, xmm8
140c90b7e: mulss xmm5, xmm7
140c90b82: movaps xmm9, xmm1
140c90b86: mulss xmm9, xmm3
140c90b8b: subss xmm5, xmm9
140c90b90: movaps xmm9, xmm1
140c90b94: mulss xmm9, xmm2
140c90b99: mulss xmm7, xmm0
140c90b9d: subss xmm9, xmm7
140c90ba2: movaps xmm7, xmm0
140c90ba5: mulss xmm7, xmm3
140c90ba9: mulss xmm8, xmm2
140c90bae: subss xmm7, xmm8
140c90bb3: movaps xmm8, xmm5
140c90bb7: mulss xmm8, xmm5
140c90bbc: movaps xmm10, xmm9
140c90bc0: mulss xmm10, xmm9
140c90bc5: addss xmm10, xmm8
140c90bca: movaps xmm8, xmm7
140c90bce: mulss xmm8, xmm7
140c90bd3: addss xmm8, xmm10
140c90bd8: xorps xmm10, xmm10
140c90bdc: sqrtss xmm10, xmm8
140c90be1: movaps xmm8, xmm4
140c90be5: divss xmm8, xmm10
140c90bea: movd eax, xmm8
140c90bef: test eax, eax
140c90bf1: setns cl
140c90bf4: mov edx, eax
140c90bf6: and edx, 0x7fffffff
140c90bfc: add edx, 0xff800000
140c90c02: cmp edx, 0x7f000000
140c90c08: setb dl
140c90c0b: and dl, cl
140c90c0d: dec eax
140c90c0f: cmp eax, 0x7fffff
140c90c14: setb al
140c90c17: or al, dl
140c90c19: jne 0x140c90c8b
140c90c1b: xorps xmm5, xmm5
140c90c1e: xorps xmm7, xmm7
140c90c21: ucomiss xmm6, xmm1
140c90c24: jb 0x140c90ca3
140c90c26: subss xmm4, xmm2
140c90c2a: movss xmm2, dword ptr [rip + 0x1c94cd2]
140c90c32: subss xmm2, xmm1
140c90c36: xorps xmm1, xmm1
140c90c39: ucomiss xmm1, xmm4
140c90c3c: jae 0x140c90cfb
140c90c42: movaps xmm1, xmm3
140c90c45: unpcklps xmm1, xmm2
140c90c48: shufps xmm1, xmm0, 0x14
140c90c4c: unpcklps xmm5, xmm4
140c90c4f: shufps xmm3, xmm7, 0xc5
140c90c53: shufps xmm5, xmm3, 0x24
140c90c57: movaps xmm0, xmm1
140c90c5a: addps xmm0, xmm5
140c90c5d: subps xmm1, xmm5
140c90c60: shufps xmm1, xmm0, 0x23
140c90c64: movshdup xmm2, xmm0
140c90c68: shufps xmm0, xmm1, 0x24
140c90c6c: xorps xmm1, xmm1
140c90c6f: sqrtss xmm1, xmm2
140c90c73: movss xmm6, dword ptr [rip + 0x1c94bf5]
140c90c7b: divss xmm6, xmm1
140c90c7f: shufps xmm6, xmm6, 0
140c90c83: mulps xmm6, xmm0
140c90c86: jmp 0x140c90d79
140c90c8b: mulss xmm5, xmm8
140c90c90: mulss xmm9, xmm8
140c90c95: mulss xmm7, xmm8
140c90c9a: movaps xmm4, xmm9
140c90c9e: ucomiss xmm6, xmm1
140c90ca1: jae 0x140c90c26
140c90ca3: addss xmm1, dword ptr [rip + 0x1c94c59]
140c90cab: addss xmm2, xmm4
140c90caf: xorps xmm4, xmm4
140c90cb2: ucomiss xmm4, xmm2
140c90cb5: jae 0x140c90d36
140c90cb7: unpcklps xmm7, xmm0
140c90cba: movaps xmm4, xmm7
140c90cbd: movlhps xmm4, xmm3
140c90cc0: movlhps xmm1, xmm3
140c90cc3: shufps xmm7, xmm1, 0x24
140c90cc7: shufps xmm3, xmm0, 0x55
140c90ccb: movlhps xmm2, xmm5
140c90cce: shufps xmm5, xmm0, 0xf4
140c90cd2: movaps xmm0, xmm3
140c90cd5: shufps xmm0, xmm5, 0x82
140c90cd9: subps xmm4, xmm0
140c90cdc: shufps xmm3, xmm2, 0x22
140c90ce0: addps xmm3, xmm7
140c90ce3: movaps xmm0, xmm3
140c90ce6: shufps xmm0, xmm4, 0x23
140c90cea: shufps xmm4, xmm0, 0x24
140c90cee: shufps xmm3, xmm3, 0xff
140c90cf2: xorps xmm0, xmm0
140c90cf5: sqrtss xmm0, xmm3
140c90cf9: jmp 0x140c90d66
140c90cfb: movlhps xmm2, xmm3
140c90cfe: shufps xmm7, xmm3, 0xd4
140c90d02: unpcklps xmm3, xmm3
140c90d05: shufps xmm2, xmm7, 0x28
140c90d09: unpcklps xmm4, xmm5
140c90d0c: movlhps xmm4, xmm0
140c90d0f: subps xmm2, xmm4
140c90d12: addps xmm4, xmm3
140c90d15: sqrtss xmm0, xmm2
140c90d19: shufps xmm2, xmm4, 0x9c
140c90d1d: shufps xmm2, xmm2, 0x78
140c90d21: movss xmm6, dword ptr [rip + 0x1c94b47]
140c90d29: divss xmm6, xmm0
140c90d2d: shufps xmm6, xmm6, 0
140c90d31: mulps xmm6, xmm2
140c90d34: jmp 0x140c90d79
140c90d36: movaps xmm4, xmm3
140c90d39: movlhps xmm4, xmm1
140c90d3c: movaps xmm1, xmm0
140c90d3f: shufps xmm1, xmm4, 0x24
140c90d43: shufps xmm7, xmm3, 0xd4
140c90d47: movaps xmm4, xmm7
140c90d4a: shufps xmm4, xmm3, 0xf2
140c90d4e: movlhps xmm5, xmm2
140c90d51: shufps xmm7, xmm5, 0x22
140c90d55: subps xmm1, xmm7
140c90d58: addps xmm4, xmm0
140c90d5b: shufps xmm4, xmm1, 0xe4
140c90d5f: movhlps xmm1, xmm1
140c90d62: sqrtss xmm0, xmm1
140c90d66: movss xmm6, dword ptr [rip + 0x1c94b02]
140c90d6e: divss xmm6, xmm0
140c90d72: shufps xmm6, xmm6, 0
140c90d76: mulps xmm6, xmm4
140c90d79: movaps xmmword ptr [rsp + 0x30], xmm6
140c90d7e: lea rcx, [rsp + 0x20]
140c90d83: lea rdx, [rsp + 0x30]
140c90d88: call 0x1412b7f50
140c90d8d: movaps xmm0, xmmword ptr [rsp + 0x20]
140c90d92: movmskps eax, xmm0
140c90d95: xor eax, 0xf
140c90d98: je 0x140c90da1
140c90d9a: movaps xmm6, xmmword ptr [rip + 0x1cad31f]
140c90da1: movaps xmmword ptr [rsi], xmm6
140c90da4: mov rax, rsi
140c90da7: movaps xmm6, xmmword ptr [rsp + 0x40]
140c90dac: movaps xmm7, xmmword ptr [rsp + 0x50]
140c90db1: movaps xmm8, xmmword ptr [rsp + 0x60]
140c90db7: movaps xmm9, xmmword ptr [rsp + 0x70]
140c90dbd: movaps xmm10, xmmword ptr [rsp + 0x80]
140c90dc6: add rsp, 0x90
140c90dcd: pop rsi
140c90dce: ret 
140c90dcf: lea rcx, [rip + 0x1e9cafa]
140c90dd6: lea r8, [rip + 0x1e9cb1b]
140c90ddd: mov edx, 0x24
140c90de2: call 0x1428d9430
140c90de7: int3 
