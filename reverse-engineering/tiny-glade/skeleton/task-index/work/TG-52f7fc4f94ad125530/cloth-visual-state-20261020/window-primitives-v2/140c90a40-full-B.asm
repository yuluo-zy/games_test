140c90a40: 56 push rsi
140c90a41: 4881ec90000000 sub rsp, 0x90
140c90a48: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
140c90a51: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140c90a57: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140c90a5d: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140c90a62: 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140c90a67: 4889ce mov rsi, rcx
140c90a6a: f20f1002 movsd xmm0, qword ptr [rdx]
140c90a6e: f3440f16c0 movshdup xmm8, xmm0
140c90a73: 66440f7ec0 movd eax, xmm8
140c90a78: 66410f7ec0 movd r8d, xmm0
140c90a7d: 660f6e4a08 movd xmm1, dword ptr [rdx + 8]
140c90a82: 660f7ec9 movd ecx, xmm1
140c90a86: 4181e0ffffff7f and r8d, 0x7fffffff
140c90a8d: 25ffffff7f and eax, 0x7fffffff
140c90a92: 3d0000807f cmp eax, 0x7f800000
140c90a97: 0f9cc0 setl al
140c90a9a: 81e1ffffff7f and ecx, 0x7fffffff
140c90aa0: 81f90000807f cmp ecx, 0x7f800000
140c90aa6: 0f9cc1 setl cl
140c90aa9: 4181f8ffff7f7f cmp r8d, 0x7f7fffff
140c90ab0: 0f8f19030000 jg 0x140c90dcf
140c90ab6: 84c0 test al, al
140c90ab8: 0f8411030000 je 0x140c90dcf
140c90abe: 84c9 test cl, cl
140c90ac0: 0f8409030000 je 0x140c90dcf
140c90ac6: 0f57f6 xorps xmm6, xmm6
140c90ac9: 660f6fe1 movdqa xmm4, xmm1
140c90acd: f30f59e6 mulss xmm4, xmm6
140c90ad1: 0f57ed xorps xmm5, xmm5
140c90ad4: 0f59e8 mulps xmm5, xmm0
140c90ad7: f30f16d5 movshdup xmm2, xmm5
140c90adb: f30f5cd1 subss xmm2, xmm1
140c90adf: f30f12d8 movsldup xmm3, xmm0
140c90ae3: f30f10dc movss xmm3, xmm4
140c90ae7: 0f5cdd subps xmm3, xmm5
140c90aea: 0f28e2 movaps xmm4, xmm2
140c90aed: f30f59e2 mulss xmm4, xmm2
140c90af1: 0f28eb movaps xmm5, xmm3
140c90af4: 0f59eb mulps xmm5, xmm3
140c90af7: f30f58e5 addss xmm4, xmm5
140c90afb: f30f16ed movshdup xmm5, xmm5
140c90aff: f30f58ec addss xmm5, xmm4
140c90b03: 0f57ff xorps xmm7, xmm7
140c90b06: f30f51fd sqrtss xmm7, xmm5
140c90b0a: f30f1025f24dc901 movss xmm4, dword ptr [rip + 0x1c94df2]
140c90b12: 0f28ec movaps xmm5, xmm4
140c90b15: f30f5eef divss xmm5, xmm7
140c90b19: 660f7ee8 movd eax, xmm5
140c90b1d: 85c0 test eax, eax
140c90b1f: 0f99c1 setns cl
140c90b22: 89c2 mov edx, eax
140c90b24: 81e2ffffff7f and edx, 0x7fffffff
140c90b2a: 81c2000080ff add edx, 0xff800000
140c90b30: 81fa0000007f cmp edx, 0x7f000000
140c90b36: 0f92c2 setb dl
140c90b39: 20ca and dl, cl
140c90b3b: ffc8 dec eax
140c90b3d: 3dffff7f00 cmp eax, 0x7fffff
140c90b42: 0f92c0 setb al
140c90b45: 08d0 or al, dl
140c90b47: f30f12fd movsldup xmm7, xmm5
140c90b4b: 0f59fb mulps xmm7, xmm3
140c90b4e: 0fb6c0 movzx eax, al
140c90b51: 660f6ed8 movd xmm3, eax
140c90b55: 660f70db50 pshufd xmm3, xmm3, 0x50
140c90b5a: 660f72f31f pslld xmm3, 0x1f
140c90b5f: 660f72e31f psrad xmm3, 0x1f
140c90b64: 660fdbdf pand xmm3, xmm7
140c90b68: 660f70fbf5 pshufd xmm7, xmm3, 0xf5
140c90b6d: 84c0 test al, al
140c90b6f: 7505 jne 0x140c90b76
140c90b71: 0f28d4 movaps xmm2, xmm4
140c90b74: eb04 jmp 0x140c90b7a
140c90b76: f30f59d5 mulss xmm2, xmm5
140c90b7a: 410f28e8 movaps xmm5, xmm8
140c90b7e: f30f59ef mulss xmm5, xmm7
140c90b82: 440f28c9 movaps xmm9, xmm1
140c90b86: f3440f59cb mulss xmm9, xmm3
140c90b8b: f3410f5ce9 subss xmm5, xmm9
140c90b90: 440f28c9 movaps xmm9, xmm1
140c90b94: f3440f59ca mulss xmm9, xmm2
140c90b99: f30f59f8 mulss xmm7, xmm0
140c90b9d: f3440f5ccf subss xmm9, xmm7
140c90ba2: 0f28f8 movaps xmm7, xmm0
140c90ba5: f30f59fb mulss xmm7, xmm3
140c90ba9: f3440f59c2 mulss xmm8, xmm2
140c90bae: f3410f5cf8 subss xmm7, xmm8
140c90bb3: 440f28c5 movaps xmm8, xmm5
140c90bb7: f3440f59c5 mulss xmm8, xmm5
140c90bbc: 450f28d1 movaps xmm10, xmm9
140c90bc0: f3450f59d1 mulss xmm10, xmm9
140c90bc5: f3450f58d0 addss xmm10, xmm8
140c90bca: 440f28c7 movaps xmm8, xmm7
140c90bce: f3440f59c7 mulss xmm8, xmm7
140c90bd3: f3450f58c2 addss xmm8, xmm10
140c90bd8: 450f57d2 xorps xmm10, xmm10
140c90bdc: f3450f51d0 sqrtss xmm10, xmm8
140c90be1: 440f28c4 movaps xmm8, xmm4
140c90be5: f3450f5ec2 divss xmm8, xmm10
140c90bea: 66440f7ec0 movd eax, xmm8
140c90bef: 85c0 test eax, eax
140c90bf1: 0f99c1 setns cl
140c90bf4: 89c2 mov edx, eax
140c90bf6: 81e2ffffff7f and edx, 0x7fffffff
140c90bfc: 81c2000080ff add edx, 0xff800000
140c90c02: 81fa0000007f cmp edx, 0x7f000000
140c90c08: 0f92c2 setb dl
140c90c0b: 20ca and dl, cl
140c90c0d: ffc8 dec eax
140c90c0f: 3dffff7f00 cmp eax, 0x7fffff
140c90c14: 0f92c0 setb al
140c90c17: 08d0 or al, dl
140c90c19: 7570 jne 0x140c90c8b
140c90c1b: 0f57ed xorps xmm5, xmm5
140c90c1e: 0f57ff xorps xmm7, xmm7
140c90c21: 0f2ef1 ucomiss xmm6, xmm1
140c90c24: 727d jb 0x140c90ca3
140c90c26: f30f5ce2 subss xmm4, xmm2
140c90c2a: f30f1015d24cc901 movss xmm2, dword ptr [rip + 0x1c94cd2]
140c90c32: f30f5cd1 subss xmm2, xmm1
140c90c36: 0f57c9 xorps xmm1, xmm1
140c90c39: 0f2ecc ucomiss xmm1, xmm4
140c90c3c: 0f83b9000000 jae 0x140c90cfb
140c90c42: 0f28cb movaps xmm1, xmm3
140c90c45: 0f14ca unpcklps xmm1, xmm2
140c90c48: 0fc6c814 shufps xmm1, xmm0, 0x14
140c90c4c: 0f14ec unpcklps xmm5, xmm4
140c90c4f: 0fc6dfc5 shufps xmm3, xmm7, 0xc5
140c90c53: 0fc6eb24 shufps xmm5, xmm3, 0x24
140c90c57: 0f28c1 movaps xmm0, xmm1
140c90c5a: 0f58c5 addps xmm0, xmm5
140c90c5d: 0f5ccd subps xmm1, xmm5
140c90c60: 0fc6c823 shufps xmm1, xmm0, 0x23
140c90c64: f30f16d0 movshdup xmm2, xmm0
140c90c68: 0fc6c124 shufps xmm0, xmm1, 0x24
140c90c6c: 0f57c9 xorps xmm1, xmm1
140c90c6f: f30f51ca sqrtss xmm1, xmm2
140c90c73: f30f1035f54bc901 movss xmm6, dword ptr [rip + 0x1c94bf5]
140c90c7b: f30f5ef1 divss xmm6, xmm1
140c90c7f: 0fc6f600 shufps xmm6, xmm6, 0
140c90c83: 0f59f0 mulps xmm6, xmm0
140c90c86: e9ee000000 jmp 0x140c90d79
140c90c8b: f3410f59e8 mulss xmm5, xmm8
140c90c90: f3450f59c8 mulss xmm9, xmm8
140c90c95: f3410f59f8 mulss xmm7, xmm8
140c90c9a: 410f28e1 movaps xmm4, xmm9
140c90c9e: 0f2ef1 ucomiss xmm6, xmm1
140c90ca1: 7383 jae 0x140c90c26
140c90ca3: f30f580d594cc901 addss xmm1, dword ptr [rip + 0x1c94c59]
140c90cab: f30f58d4 addss xmm2, xmm4
140c90caf: 0f57e4 xorps xmm4, xmm4
140c90cb2: 0f2ee2 ucomiss xmm4, xmm2
140c90cb5: 737f jae 0x140c90d36
140c90cb7: 0f14f8 unpcklps xmm7, xmm0
140c90cba: 0f28e7 movaps xmm4, xmm7
140c90cbd: 0f16e3 movlhps xmm4, xmm3
140c90cc0: 0f16cb movlhps xmm1, xmm3
140c90cc3: 0fc6f924 shufps xmm7, xmm1, 0x24
140c90cc7: 0fc6d855 shufps xmm3, xmm0, 0x55
140c90ccb: 0f16d5 movlhps xmm2, xmm5
140c90cce: 0fc6e8f4 shufps xmm5, xmm0, 0xf4
140c90cd2: 0f28c3 movaps xmm0, xmm3
140c90cd5: 0fc6c582 shufps xmm0, xmm5, 0x82
140c90cd9: 0f5ce0 subps xmm4, xmm0
140c90cdc: 0fc6da22 shufps xmm3, xmm2, 0x22
140c90ce0: 0f58df addps xmm3, xmm7
140c90ce3: 0f28c3 movaps xmm0, xmm3
140c90ce6: 0fc6c423 shufps xmm0, xmm4, 0x23
140c90cea: 0fc6e024 shufps xmm4, xmm0, 0x24
140c90cee: 0fc6dbff shufps xmm3, xmm3, 0xff
140c90cf2: 0f57c0 xorps xmm0, xmm0
140c90cf5: f30f51c3 sqrtss xmm0, xmm3
140c90cf9: eb6b jmp 0x140c90d66
140c90cfb: 0f16d3 movlhps xmm2, xmm3
140c90cfe: 0fc6fbd4 shufps xmm7, xmm3, 0xd4
140c90d02: 0f14db unpcklps xmm3, xmm3
140c90d05: 0fc6d728 shufps xmm2, xmm7, 0x28
140c90d09: 0f14e5 unpcklps xmm4, xmm5
140c90d0c: 0f16e0 movlhps xmm4, xmm0
140c90d0f: 0f5cd4 subps xmm2, xmm4
140c90d12: 0f58e3 addps xmm4, xmm3
140c90d15: f30f51c2 sqrtss xmm0, xmm2
140c90d19: 0fc6d49c shufps xmm2, xmm4, 0x9c
140c90d1d: 0fc6d278 shufps xmm2, xmm2, 0x78
140c90d21: f30f1035474bc901 movss xmm6, dword ptr [rip + 0x1c94b47]
140c90d29: f30f5ef0 divss xmm6, xmm0
140c90d2d: 0fc6f600 shufps xmm6, xmm6, 0
140c90d31: 0f59f2 mulps xmm6, xmm2
140c90d34: eb43 jmp 0x140c90d79
140c90d36: 0f28e3 movaps xmm4, xmm3
140c90d39: 0f16e1 movlhps xmm4, xmm1
140c90d3c: 0f28c8 movaps xmm1, xmm0
140c90d3f: 0fc6cc24 shufps xmm1, xmm4, 0x24
140c90d43: 0fc6fbd4 shufps xmm7, xmm3, 0xd4
140c90d47: 0f28e7 movaps xmm4, xmm7
140c90d4a: 0fc6e3f2 shufps xmm4, xmm3, 0xf2
140c90d4e: 0f16ea movlhps xmm5, xmm2
140c90d51: 0fc6fd22 shufps xmm7, xmm5, 0x22
140c90d55: 0f5ccf subps xmm1, xmm7
140c90d58: 0f58e0 addps xmm4, xmm0
140c90d5b: 0fc6e1e4 shufps xmm4, xmm1, 0xe4
140c90d5f: 0f12c9 movhlps xmm1, xmm1
140c90d62: f30f51c1 sqrtss xmm0, xmm1
140c90d66: f30f1035024bc901 movss xmm6, dword ptr [rip + 0x1c94b02]
140c90d6e: f30f5ef0 divss xmm6, xmm0
140c90d72: 0fc6f600 shufps xmm6, xmm6, 0
140c90d76: 0f59f4 mulps xmm6, xmm4
140c90d79: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140c90d7e: 488d4c2420 lea rcx, [rsp + 0x20]
140c90d83: 488d542430 lea rdx, [rsp + 0x30]
140c90d88: e8c3716200 call 0x1412b7f50
140c90d8d: 0f28442420 movaps xmm0, xmmword ptr [rsp + 0x20]
140c90d92: 0f50c0 movmskps eax, xmm0
140c90d95: 83f00f xor eax, 0xf
140c90d98: 7407 je 0x140c90da1
140c90d9a: 0f28351fd3ca01 movaps xmm6, xmmword ptr [rip + 0x1cad31f]
140c90da1: 0f2936 movaps xmmword ptr [rsi], xmm6
140c90da4: 4889f0 mov rax, rsi
140c90da7: 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140c90dac: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140c90db1: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140c90db7: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
140c90dbd: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
140c90dc6: 4881c490000000 add rsp, 0x90
140c90dcd: 5e pop rsi
140c90dce: c3 ret 
140c90dcf: 488d0dfacae901 lea rcx, [rip + 0x1e9cafa]
140c90dd6: 4c8d051bcbe901 lea r8, [rip + 0x1e9cb1b]
140c90ddd: ba24000000 mov edx, 0x24
140c90de2: e84986c401 call 0x1428d9430
140c90de7: cc int3 
