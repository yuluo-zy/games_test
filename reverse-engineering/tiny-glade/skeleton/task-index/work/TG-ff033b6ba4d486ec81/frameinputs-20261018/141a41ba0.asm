141a41ba0: push rbp
141a41ba1: push r15
141a41ba3: push r14
141a41ba5: push r13
141a41ba7: push r12
141a41ba9: push rsi
141a41baa: push rdi
141a41bab: push rbx
141a41bac: sub rsp, 0x368
141a41bb3: lea rbp, [rsp + 0x80]
141a41bbb: movaps xmmword ptr [rbp + 0x2d0], xmm15
141a41bc3: movaps xmmword ptr [rbp + 0x2c0], xmm14
141a41bcb: movaps xmmword ptr [rbp + 0x2b0], xmm13
141a41bd3: movaps xmmword ptr [rbp + 0x2a0], xmm12
141a41bdb: movaps xmmword ptr [rbp + 0x290], xmm11
141a41be3: movaps xmmword ptr [rbp + 0x280], xmm10
141a41beb: movaps xmmword ptr [rbp + 0x270], xmm9
141a41bf3: movaps xmmword ptr [rbp + 0x260], xmm8
141a41bfb: movaps xmmword ptr [rbp + 0x250], xmm7
141a41c02: movaps xmmword ptr [rbp + 0x240], xmm6
141a41c09: mov qword ptr [rbp + 0x238], 0xfffffffffffffffe
141a41c14: mov rsi, r9
141a41c17: mov rdi, rcx
141a41c1a: mov r12, qword ptr [rdx]
141a41c1d: cmp qword ptr [r12], 0
141a41c22: mov rbx, qword ptr [r8]
141a41c25: jne 0x141a41d5b
141a41c2b: mov r15, rdx
141a41c2e: lea rdx, [rip + 0x15b6f83]
141a41c35: mov rcx, rbx
141a41c38: call 0x1421b5d00
141a41c3d: test rax, rax
141a41c40: je 0x141a438a7
141a41c46: mov r14, rax
141a41c49: lea rax, [rip + 0x15b70a0]
141a41c50: mov qword ptr [rbp + 0x20], rax
141a41c54: mov qword ptr [rbp + 0x28], 0x3d
141a41c5c: cmp qword ptr [r14 + 0x18], 0x3d
141a41c61: jne 0x141a43853
141a41c67: mov rdx, qword ptr [r14 + 0x10]
141a41c6b: lea rcx, [rip + 0x15b707e]
141a41c72: mov r8d, 0x3d
141a41c78: call 0x142923bf0
141a41c7d: test eax, eax
141a41c7f: jne 0x141a43853
141a41c85: mov rcx, qword ptr [r14]
141a41c88: mov rax, qword ptr [r14 + 8]
141a41c8c: call qword ptr [rax + 0x18]
141a41c8f: test rax, rax
141a41c92: je 0x141a438a7
141a41c98: mov r14, rax
141a41c9b: movzx eax, byte ptr [rip + 0x1bf2f67]
141a41ca2: mov ecx, 0x20
141a41ca7: mov edx, 8
141a41cac: call 0x140613c10
141a41cb1: test rax, rax
141a41cb4: je 0x141a438bf
141a41cba: mov rdx, rax
141a41cbd: mov qword ptr [rax], 0
141a41cc4: mov qword ptr [rax + 8], 4
141a41ccc: mov qword ptr [rax + 0x10], 0
141a41cd4: mov byte ptr [rax + 0x18], 0
141a41cd8: mov r8, qword ptr [r14 + 0x60]
141a41cdc: jmp 0x141a41ce2
141a41cde: nop 
141a41ce0: pause 
141a41ce2: mov rax, qword ptr [r8 + 8]
141a41ce6: nop word ptr cs:[rax + rax]
141a41cf0: cmp rax, -1
141a41cf4: je 0x141a41ce0
141a41cf6: test rax, rax
141a41cf9: js 0x141a437eb
141a41cff: lea rcx, [rax + 1]
141a41d03: lock cmpxchg qword ptr [r8 + 8], rcx
141a41d09: jne 0x141a41cf0
141a41d0b: mov qword ptr [rbp + 0x230], r8
141a41d12: mov qword ptr [rbp + 0x220], rdx
141a41d19: mov eax, dword ptr [r15 + 0x1c]
141a41d1d: mov rcx, qword ptr [r15 + 0x10]
141a41d21: mov dword ptr [rcx], eax
141a41d23: cmp qword ptr [r12], 0
141a41d28: mov qword ptr [rbp + 0x228], r12
141a41d2f: je 0x141a41d3d
141a41d31: mov rcx, qword ptr [rbp + 0x228]
141a41d38: call 0x141a3ce70
141a41d3d: mov rax, qword ptr [rbp + 0x230]
141a41d44: mov r12, qword ptr [rbp + 0x228]
141a41d4b: mov qword ptr [r12], rax
141a41d4f: mov rax, qword ptr [rbp + 0x220]
141a41d56: mov qword ptr [r12 + 8], rax
141a41d5b: lea rdx, [rip + 0x15b6e56]
141a41d62: mov rcx, rbx
141a41d65: call 0x1421b5d00
141a41d6a: test rax, rax
141a41d6d: je 0x141a43883
141a41d73: mov rbx, rax
141a41d76: lea rax, [rip + 0x15b6f73]
141a41d7d: mov qword ptr [rbp + 0x20], rax
141a41d81: mov qword ptr [rbp + 0x28], 0x3d
141a41d89: cmp qword ptr [rbx + 0x18], 0x3d
141a41d8e: jne 0x141a437fe
141a41d94: mov rdx, qword ptr [rbx + 0x10]
141a41d98: lea rcx, [rip + 0x15b6f51]
141a41d9f: mov r8d, 0x3d
141a41da5: call 0x142923bf0
141a41daa: test eax, eax
141a41dac: jne 0x141a437fe
141a41db2: mov rcx, qword ptr [rbx]
141a41db5: mov rax, qword ptr [rbx + 8]
141a41db9: call qword ptr [rax + 0x18]
141a41dbc: test rax, rax
141a41dbf: je 0x141a43883
141a41dc5: cmp qword ptr [r12], 0
141a41dca: je 0x141a4388f
141a41dd0: mov r15, qword ptr [r12 + 8]
141a41dd5: test r15, r15
141a41dd8: je 0x141a4389b
141a41dde: mov rbx, rax
141a41de1: lea rcx, [r15 + 0x18]
141a41de5: mov dl, 1
141a41de7: xor eax, eax
141a41de9: lock cmpxchg byte ptr [r15 + 0x18], dl
141a41def: jne 0x141a4382e
141a41df5: call 0x140c81c70
141a41dfa: mov rax, qword ptr [r15 + 0x10]
141a41dfe: test rax, rax
141a41e01: sete cl
141a41e04: shl rax, 2
141a41e08: add rax, qword ptr [r15 + 8]
141a41e0c: add rax, -4
141a41e10: sete dl
141a41e13: or dl, cl
141a41e15: je 0x141a41e29
141a41e17: mov rax, qword ptr [rbx + 0x28]
141a41e1b: test rax, rax
141a41e1e: je 0x141a438b3
141a41e24: lea ecx, [rax - 1]
141a41e27: jmp 0x141a41e2f
141a41e29: mov ecx, dword ptr [rax]
141a41e2b: mov rax, qword ptr [rbx + 0x28]
141a41e2f: mov rdx, qword ptr [rbx + 0x20]
141a41e33: movups xmm0, xmmword ptr [rbx + 0x30]
141a41e37: movups xmm1, xmmword ptr [rbx + 0x40]
141a41e3b: movups xmm2, xmmword ptr [rbx + 0x50]
141a41e3f: mov dword ptr [rbp + 0x70], ecx
141a41e42: mov qword ptr [rbp + 0x20], rdx
141a41e46: mov qword ptr [rbp + 0x28], rax
141a41e4a: movups xmmword ptr [rbp + 0x30], xmm0
141a41e4e: mov qword ptr [rbp + 0x40], r15
141a41e52: movups xmmword ptr [rbp + 0x48], xmm1
141a41e56: mov qword ptr [rbp + 0x58], rbx
141a41e5a: movups xmmword ptr [rbp + 0x60], xmm2
141a41e5e: lea rcx, [rbp + 0x20]
141a41e62: call 0x1421b62e0
141a41e67: mov r11, qword ptr [rdi]
141a41e6a: mov rax, qword ptr [rdi + 8]
141a41e6e: mov rdx, qword ptr [r11]
141a41e71: xor r14d, r14d
141a41e74: mov r8, rdx
141a41e77: sub r8, qword ptr [rax + 0x18]
141a41e7b: cmovb r8, r14
141a41e7f: sub rdx, qword ptr [rax + 0x38]
141a41e83: mov rcx, qword ptr [rbp + 0x350]
141a41e8a: cmovb rdx, r14
141a41e8e: lea rdi, [r8 + r8*4]
141a41e92: shl rdi, 3
141a41e96: add rdi, qword ptr [rax + 8]
141a41e9a: mov r9, qword ptr [rax + 0x10]
141a41e9e: mov r10, qword ptr [rax + 0x30]
141a41ea2: sub r9, r8
141a41ea5: cmovb r9, r14
141a41ea9: mov r8d, 8
141a41eaf: cmovb rdi, r8
141a41eb3: lea r12, [rdx + rdx*4]
141a41eb7: shl r12, 3
141a41ebb: add r12, qword ptr [rax + 0x28]
141a41ebf: sub r10, rdx
141a41ec2: cmovb r10, r14
141a41ec6: cmovb r12, r8
141a41eca: mov rax, qword ptr [rax + 0x40]
141a41ece: sub rax, r10
141a41ed1: sub rax, r9
141a41ed4: mov qword ptr [rbp - 0x10], r11
141a41ed8: mov qword ptr [r11], rax
141a41edb: lea rax, [r9 + r9*4]
141a41edf: lea rax, [rdi + rax*8]
141a41ee3: mov qword ptr [rbp - 8], rax
141a41ee7: lea rax, [r10 + r10*4]
141a41eeb: lea rax, [r12 + rax*8]
141a41eef: mov qword ptr [rbp], rax
141a41ef3: mov rax, qword ptr [rcx]
141a41ef6: mov qword ptr [rbp + 8], rax
141a41efa: mov eax, dword ptr [rsi + 0x1c]
141a41efd: mov dword ptr [rbp + 0x8c], eax
141a41f03: mov rax, qword ptr [rsi]
141a41f06: mov qword ptr [rbp + 0x10], rax
141a41f0a: mov rax, qword ptr [rsi + 0x10]
141a41f0e: mov qword ptr [rbp + 0x18], rax
141a41f12: mov rsi, rdi
141a41f15: lea rbx, [rbp + 0x160]
141a41f1c: movss xmm10, dword ptr [rip + 0xee39df]
141a41f25: jmp 0x141a41f41
141a41f27: or r14b, al
141a41f2a: inc rdi
141a41f2d: mov eax, dword ptr [rbp + 0x8c]
141a41f33: mov rcx, qword ptr [rbp + 0x18]
141a41f37: mov dword ptr [rcx], eax
141a41f39: mov rax, qword ptr [rbp + 0x10]
141a41f3d: mov qword ptr [rax + 8], rdi
141a41f41: mov rax, r12
141a41f44: mov rcx, rsi
141a41f47: jmp 0x141a41f69
141a41f49: nop dword ptr [rax]
141a41f50: lea rsi, [rcx + 0x28]
141a41f54: mov rdi, rax
141a41f57: mov rdx, rcx
141a41f5a: mov rcx, qword ptr [rbp - 0x10]
141a41f5e: inc qword ptr [rcx]
141a41f61: test byte ptr [rdx], 1
141a41f64: mov rcx, rsi
141a41f67: jne 0x141a41fa0
141a41f69: test rcx, rcx
141a41f6c: je 0x141a41f74
141a41f6e: cmp rcx, qword ptr [rbp - 8]
141a41f72: jne 0x141a41f50
141a41f74: test rax, rax
141a41f77: je 0x141a43697
141a41f7d: cmp rax, qword ptr [rbp]
141a41f81: je 0x141a43697
141a41f87: lea rdi, [rax + 0x28]
141a41f8b: xor esi, esi
141a41f8d: mov rdx, rax
141a41f90: mov rax, rdi
141a41f93: jmp 0x141a41f5a
141a41f95: nop word ptr cs:[rax + rax]
141a41fa0: mov qword ptr [rbp + 0x98], rdx
141a41fa7: add rdx, 4
141a41fab: mov rax, qword ptr [rbp + 8]
141a41faf: movss xmm2, dword ptr [rax + 8]
141a41fb4: movss xmm7, dword ptr [rax + 0xc]
141a41fb9: lea rcx, [rbp + 0x120]
141a41fc0: mov qword ptr [rbp + 0x228], rdx
141a41fc7: movss dword ptr [rbp + 0x21c], xmm2
141a41fcf: call 0x140c991f0
141a41fd4: mov qword ptr [rbp + 0xb8], rdi
141a41fdb: mov r12, qword ptr [rbp + 0x128]
141a41fe2: mov rax, qword ptr [rbp + 0x130]
141a41fe9: test rax, rax
141a41fec: je 0x141a428f0
141a41ff2: shl rax, 2
141a41ff6: lea r15, [rax + rax*2]
141a41ffa: xor r13d, r13d
141a41ffd: xor edi, edi
141a41fff: nop 
141a42000: mov eax, dword ptr [r12 + r13 + 8]
141a42005: mov dword ptr [rbp + 0xc8], eax
141a4200b: mov rax, qword ptr [r12 + r13]
141a4200f: mov qword ptr [rbp + 0xc0], rax
141a42016: mov rcx, rbx
141a42019: mov rdx, qword ptr [rbp + 0x228]
141a42020: call 0x140c99570
141a42025: movss xmm6, dword ptr [rbp + 0x160]
141a4202d: movss xmm4, dword ptr [rbp + 0x164]
141a42035: movss xmm3, dword ptr [rbp + 0x168]
141a4203d: movss xmm2, dword ptr [rbp + 0x16c]
141a42045: movss xmm8, dword ptr [rbp + 0x170]
141a4204e: movsd xmm1, qword ptr [rbp + 0x174]
141a42056: movss xmm0, dword ptr [rbp + 0x17c]
141a4205e: movss xmm5, dword ptr [rbp + 0x180]
141a42066: xorps xmm9, xmm9
141a4206a: ucomiss xmm9, xmm5
141a4206e: jae 0x141a420d0
141a42070: addss xmm6, xmm8
141a42075: addss xmm5, xmm10
141a4207a: ucomiss xmm9, xmm6
141a4207e: jae 0x141a42120
141a42084: unpcklps xmm4, xmm6
141a42087: shufps xmm4, xmm1, 0x14
141a4208b: unpcklps xmm3, xmm0
141a4208e: unpcklps xmm2, xmm5
141a42091: movlhps xmm2, xmm3
141a42094: movaps xmm0, xmm4
141a42097: subps xmm0, xmm2
141a4209a: addps xmm2, xmm4
141a4209d: movshdup xmm1, xmm2
141a420a1: shufps xmm2, xmm0, 1
141a420a5: shufps xmm2, xmm0, 0xe2
141a420a9: sqrtss xmm1, xmm1
141a420ad: movss xmm0, dword ptr [rip + 0xee37bb]
141a420b5: divss xmm0, xmm1
141a420b9: shufps xmm0, xmm0, 0
141a420bd: mulps xmm0, xmm2
141a420c0: jmp 0x141a4218d
141a420c5: nop word ptr cs:[rax + rax]
141a420d0: subss xmm8, xmm6
141a420d5: movaps xmm6, xmm10
141a420d9: subss xmm6, xmm5
141a420dd: ucomiss xmm9, xmm8
141a420e1: jae 0x141a42140
141a420e3: movlhps xmm4, xmm8
141a420e7: movaps xmm5, xmm1
141a420ea: shufps xmm5, xmm4, 0x24
141a420ee: unpcklps xmm0, xmm3
141a420f1: movlhps xmm2, xmm6
141a420f4: subps xmm1, xmm0
141a420f7: shufps xmm0, xmm2, 0x24
141a420fb: addps xmm0, xmm5
141a420fe: shufps xmm1, xmm0, 1
141a42102: shufps xmm1, xmm0, 0xe2
141a42106: movhlps xmm0, xmm0
141a42109: xorps xmm2, xmm2
141a4210c: sqrtss xmm2, xmm0
141a42110: jmp 0x141a4217a
141a42112: nop word ptr cs:[rax + rax]
141a42120: unpcklps xmm5, xmm4
141a42123: movlhps xmm5, xmm1
141a42126: unpcklps xmm0, xmm3
141a42129: unpcklps xmm6, xmm2
141a4212c: movlhps xmm6, xmm0
141a4212f: movaps xmm1, xmm5
141a42132: addps xmm1, xmm6
141a42135: subps xmm5, xmm6
141a42138: movsd xmm1, xmm5
141a4213c: jmp 0x141a42173
141a4213e: nop 
141a42140: movaps xmm5, xmm3
141a42143: unpcklps xmm5, xmm1
141a42146: movlhps xmm3, xmm4
141a42149: movlhps xmm6, xmm4
141a4214c: shufps xmm5, xmm6, 0x24
141a42150: shufps xmm0, xmm1, 0xd4
141a42154: shufps xmm1, xmm2, 0xc5
141a42158: addps xmm1, xmm3
141a4215b: movlhps xmm8, xmm2
141a4215f: shufps xmm0, xmm8, 0x22
141a42164: subps xmm5, xmm0
141a42167: shufps xmm1, xmm5, 0xd8
141a4216b: shufps xmm1, xmm1, 0xd8
141a4216f: shufps xmm5, xmm5, 0xff
141a42173: xorps xmm2, xmm2
141a42176: sqrtss xmm2, xmm5
141a4217a: movss xmm0, dword ptr [rip + 0xee36ee]
141a42182: divss xmm0, xmm2
141a42186: shufps xmm0, xmm0, 0
141a4218a: mulps xmm0, xmm1
141a4218d: mov eax, dword ptr [rbp + 0xc8]
141a42193: mov dword ptr [rbp + 0x1e8], eax
141a42199: mov rax, qword ptr [rbp + 0xc0]
141a421a0: mov qword ptr [rbp + 0x1e0], rax
141a421a7: mov dword ptr [rbp + 0x1ec], 0x3e4ccccd
141a421b1: movss dword ptr [rbp + 0x1f0], xmm7
141a421b9: mov dword ptr [rbp + 0x1f4], 0x3e4ccccd
141a421c3: movups xmm3, xmmword ptr [rbp + 0x1ec]
141a421ca: movaps xmm1, xmmword ptr [rbp + 0x1f0]
141a421d1: movaps xmm2, xmmword ptr [rbp + 0x1e0]
141a421d8: movaps xmm4, xmm0
141a421db: shufps xmm4, xmm0, 0xff
141a421df: movaps xmm12, xmm0
141a421e3: addps xmm12, xmm0
141a421e7: movaps xmm6, xmm4
141a421ea: mulss xmm6, xmm12
141a421ef: movaps xmm5, xmm12
141a421f3: unpckhpd xmm5, xmm12
141a421f8: mulss xmm4, xmm5
141a421fc: movaps xmm15, xmm0
141a42200: mulps xmm15, xmm12
141a42204: movshdup xmm9, xmm0
141a42209: movhlps xmm0, xmm0
141a4220c: mulss xmm0, xmm12
141a42211: movaps xmm8, xmm12
141a42215: shufps xmm8, xmm12, 0xff
141a4221a: mulss xmm8, xmm9
141a4221f: mulss xmm5, xmm9
141a42224: mulss xmm12, xmm9
141a42229: movaps xmm14, xmm15
141a4222d: unpckhpd xmm14, xmm15
141a42232: movaps xmm9, xmm14
141a42236: addss xmm9, xmm15
141a4223b: movaps xmm11, xmm10
141a4223f: subss xmm11, xmm9
141a42244: movaps xmm9, xmm4
141a42248: addss xmm9, xmm12
141a4224d: unpcklps xmm11, xmm9
141a42251: movaps xmm9, xmm6
141a42255: subss xmm9, xmm5
141a4225a: shufps xmm11, xmm9, 4
141a4225f: subss xmm4, xmm12
141a42264: movaps xmm9, xmm15
141a42268: shufps xmm9, xmm15, 0xff
141a4226d: addss xmm15, xmm9
141a42272: movaps xmm12, xmm10
141a42276: subss xmm12, xmm15
141a4227b: unpcklps xmm4, xmm12
141a4227f: movaps xmm12, xmm0
141a42283: addss xmm12, xmm8
141a42288: shufps xmm4, xmm12, 4
141a4228d: addss xmm5, xmm6
141a42291: subss xmm0, xmm8
141a42296: unpcklps xmm5, xmm0
141a42299: addss xmm9, xmm14
141a4229e: movaps xmm0, xmm10
141a422a2: subss xmm0, xmm9
141a422a7: shufps xmm5, xmm0, 4
141a422ab: shufps xmm3, xmm3, 0
141a422af: mulps xmm3, xmm11
141a422b3: movaps xmm0, xmm1
141a422b6: shufps xmm0, xmm1, 0
141a422ba: mulps xmm0, xmm4
141a422bd: shufps xmm1, xmm1, 0x55
141a422c1: mulps xmm1, xmm5
141a422c4: shufps xmm2, xmm2, 0xa4
141a422c8: movaps xmmword ptr [rbp + 0x160], xmm3
141a422cf: movaps xmmword ptr [rbp + 0x170], xmm0
141a422d6: movaps xmmword ptr [rbp + 0x180], xmm1
141a422dd: movaps xmmword ptr [rbp + 0x190], xmm2
141a422e4: lea rcx, [rbp + 0xc0]
141a422eb: mov rdx, rbx
141a422ee: call 0x14089d560
141a422f3: movups xmm0, xmmword ptr [rbp + 0xc0]
141a422fa: movups xmm1, xmmword ptr [rbp + 0xd0]
141a42301: movups xmm2, xmmword ptr [rbp + 0xe0]
141a42308: movaps xmmword ptr [rbp + 0x160], xmm0
141a4230f: movaps xmmword ptr [rbp + 0x170], xmm1
141a42316: movaps xmmword ptr [rbp + 0x180], xmm2
141a4231d: movaps xmm0, xmmword ptr [rip + 0x10d2b7c]
141a42324: movaps xmmword ptr [rbp + 0x190], xmm0
141a4232b: lea rcx, [rbp + 0x20]
141a4232f: mov rdx, rbx
141a42332: call 0x1421b6000
141a42337: or r14b, al
141a4233a: inc rdi
141a4233d: add r13, 0xc
141a42341: cmp r15, r13
141a42344: jne 0x141a42000
141a4234a: mov rax, qword ptr [rbp + 0x120]
141a42351: test rax, rax
141a42354: je 0x141a4236c
141a42356: shl rax, 2
141a4235a: lea rdx, [rax + rax*2]
141a4235e: mov r8d, 4
141a42364: mov rcx, r12
141a42367: call 0x140613c20
141a4236c: mov rdx, qword ptr [rbp + 0x138]
141a42373: test rdx, rdx
141a42376: je 0x141a4238e
141a42378: mov rcx, qword ptr [rbp + 0x140]
141a4237f: shl rdx, 2
141a42383: mov r8d, 4
141a42389: call 0x140613c20
141a4238e: lea rcx, [rbp + 0x120]
141a42395: mov rdx, qword ptr [rbp + 0x228]
141a4239c: movss xmm2, dword ptr [rbp + 0x21c]
141a423a4: call 0x140c991f0
141a423a9: mov r12, qword ptr [rbp + 0xb8]
141a423b0: mov rcx, qword ptr [rbp + 0x128]
141a423b7: mov rax, qword ptr [rbp + 0x130]
141a423be: test rax, rax
141a423c1: mov qword ptr [rbp + 0x90], rcx
141a423c8: je 0x141a42915
141a423ce: lea rax, [rax + rax*2]
141a423d2: lea rax, [rcx + rax*4]
141a423d6: mov qword ptr [rbp + 0x210], rax
141a423dd: mov al, 1
141a423df: xorps xmm6, xmm6
141a423e2: jmp 0x141a42409
141a423e4: nop word ptr cs:[rax + rax]
141a423f0: xor eax, eax
141a423f2: mov rdx, qword ptr [rbp + 0x230]
141a423f9: mov rcx, rdx
141a423fc: cmp rdx, qword ptr [rbp + 0x210]
141a42403: je 0x141a42915
141a42409: lea rdx, [rcx + 0xc]
141a4240d: test al, 1
141a4240f: je 0x141a42440
141a42411: cmp rdx, qword ptr [rbp + 0x210]
141a42418: je 0x141a42907
141a4241e: lea rax, [rcx + 0x18]
141a42422: mov qword ptr [rbp + 0x230], rax
141a42429: mov r13, rcx
141a4242c: mov rcx, rdx
141a4242f: jmp 0x141a4244e
141a42431: nop word ptr cs:[rax + rax]
141a42440: mov qword ptr [rbp + 0x230], rdx
141a42447: mov r13, qword ptr [rbp + 0x220]
141a4244e: movss xmm0, dword ptr [r13 + 4]
141a42454: subss xmm0, dword ptr [rcx + 4]
141a42459: mulss xmm0, xmm0
141a4245d: movss xmm1, dword ptr [r13 + 8]
141a42463: movss xmm2, dword ptr [r13]
141a42469: unpcklps xmm2, xmm1
141a4246c: movss xmm1, dword ptr [rcx]
141a42470: mov qword ptr [rbp + 0x220], rcx
141a42477: movss xmm3, dword ptr [rcx + 8]
141a4247c: unpcklps xmm1, xmm3
141a4247f: subps xmm2, xmm1
141a42482: mulps xmm2, xmm2
141a42485: addss xmm0, xmm2
141a42489: movshdup xmm1, xmm2
141a4248d: addss xmm1, xmm0
141a42491: xorps xmm0, xmm0
141a42494: sqrtss xmm0, xmm1
141a42498: divss xmm0, dword ptr [rip + 0x1479e60]
141a424a0: call 0x142923ee0
141a424a5: cvttss2si rax, xmm0
141a424aa: mov rcx, rax
141a424ad: sar rcx, 0x3f
141a424b1: movaps xmm1, xmm0
141a424b4: subss xmm1, dword ptr [rip + 0xf17c20]
141a424bc: cvttss2si rdx, xmm1
141a424c1: and rdx, rcx
141a424c4: or rdx, rax
141a424c7: ucomiss xmm0, xmm6
141a424ca: mov r15d, 0
141a424d0: cmovae r15, rdx
141a424d4: ucomiss xmm0, dword ptr [rip + 0xf17c05]
141a424db: mov rax, 0xffffffffffffffff
141a424e2: cmova r15, rax
141a424e6: cmp r15, 2
141a424ea: jb 0x141a423f0
141a424f0: test r15, r15
141a424f3: js 0x141a42500
141a424f5: cvtsi2ss xmm8, r15
141a424fa: jmp 0x141a42519
141a424fc: nop dword ptr [rax]
141a42500: mov rax, r15
141a42503: shr rax, 1
141a42506: mov ecx, r15d
141a42509: and ecx, 1
141a4250c: or rcx, rax
141a4250f: cvtsi2ss xmm8, rcx
141a42514: addss xmm8, xmm8
141a42519: add rdi, r15
141a4251c: dec rdi
141a4251f: mov r12d, 1
141a42525: nop word ptr cs:[rax + rax]
141a42530: test r12, r12
141a42533: js 0x141a42540
141a42535: xorps xmm6, xmm6
141a42538: cvtsi2ss xmm6, r12
141a4253d: jmp 0x141a4255b
141a4253f: nop 
141a42540: mov rax, r12
141a42543: shr rax, 1
141a42546: mov ecx, r12d
141a42549: and ecx, 1
141a4254c: or rcx, rax
141a4254f: xorps xmm6, xmm6
141a42552: cvtsi2ss xmm6, rcx
141a42557: addss xmm6, xmm6
141a4255b: mov rax, qword ptr [rbp + 0x220]
141a42562: divss xmm6, xmm8
141a42567: movaps xmm12, xmm10
141a4256b: subss xmm12, xmm6
141a42570: movss xmm15, dword ptr [r13 + 8]
141a42576: mulss xmm15, xmm12
141a4257b: movsd xmm14, qword ptr [r13]
141a42581: movsd xmm13, qword ptr [rax]
141a42586: movss xmm11, dword ptr [rax + 8]
141a4258c: mulss xmm11, xmm6
141a42591: mov rcx, rbx
141a42594: mov rdx, qword ptr [rbp + 0x228]
141a4259b: call 0x140c99570
141a425a0: movsldup xmm1, xmm12
141a425a5: mulps xmm1, xmm14
141a425a9: movsldup xmm0, xmm6
141a425ad: mulps xmm0, xmm13
141a425b1: addps xmm0, xmm1
141a425b4: addss xmm15, xmm11
141a425b9: movss xmm11, dword ptr [rbp + 0x160]
141a425c2: movss xmm5, dword ptr [rbp + 0x164]
141a425ca: movss xmm4, dword ptr [rbp + 0x168]
141a425d2: movss xmm3, dword ptr [rbp + 0x16c]
141a425da: movss xmm12, dword ptr [rbp + 0x170]
141a425e3: movsd xmm2, qword ptr [rbp + 0x174]
141a425eb: movss xmm1, dword ptr [rbp + 0x17c]
141a425f3: movss xmm6, dword ptr [rbp + 0x180]
141a425fb: xorps xmm9, xmm9
141a425ff: ucomiss xmm9, xmm6
141a42603: jae 0x141a42660
141a42605: addss xmm11, xmm12
141a4260a: addss xmm6, xmm10
141a4260f: ucomiss xmm9, xmm11
141a42613: jae 0x141a426b0
141a42619: unpcklps xmm5, xmm11
141a4261d: shufps xmm5, xmm2, 0x14
141a42621: unpcklps xmm4, xmm1
141a42624: unpcklps xmm3, xmm6
141a42627: movlhps xmm3, xmm4
141a4262a: movaps xmm1, xmm5
141a4262d: subps xmm1, xmm3
141a42630: addps xmm3, xmm5
141a42633: movshdup xmm2, xmm3
141a42637: shufps xmm3, xmm1, 1
141a4263b: shufps xmm3, xmm1, 0xe2
141a4263f: sqrtss xmm2, xmm2
141a42643: movss xmm1, dword ptr [rip + 0xee3225]
141a4264b: divss xmm1, xmm2
141a4264f: shufps xmm1, xmm1, 0
141a42653: mulps xmm1, xmm3
141a42656: jmp 0x141a4272f
141a4265b: nop dword ptr [rax + rax]
141a42660: subss xmm12, xmm11
141a42665: movaps xmm11, xmm10
141a42669: subss xmm11, xmm6
141a4266e: ucomiss xmm9, xmm12
141a42672: jae 0x141a426e0
141a42674: movlhps xmm5, xmm12
141a42678: movaps xmm6, xmm2
141a4267b: shufps xmm6, xmm5, 0x24
141a4267f: unpcklps xmm1, xmm4
141a42682: movlhps xmm3, xmm11
141a42686: subps xmm2, xmm1
141a42689: shufps xmm1, xmm3, 0x24
141a4268d: addps xmm1, xmm6
141a42690: shufps xmm2, xmm1, 1
141a42694: shufps xmm2, xmm1, 0xe2
141a42698: movhlps xmm1, xmm1
141a4269b: xorps xmm3, xmm3
141a4269e: sqrtss xmm3, xmm1
141a426a2: jmp 0x141a4271c
141a426a4: nop word ptr cs:[rax + rax]
141a426b0: unpcklps xmm6, xmm5
141a426b3: movlhps xmm6, xmm2
141a426b6: unpcklps xmm1, xmm4
141a426b9: unpcklps xmm11, xmm3
141a426bd: movlhps xmm11, xmm1
141a426c1: movaps xmm2, xmm6
141a426c4: addps xmm2, xmm11
141a426c8: subps xmm6, xmm11
141a426cc: movsd xmm2, xmm6
141a426d0: jmp 0x141a42715
141a426d2: nop word ptr cs:[rax + rax]
141a426e0: movaps xmm6, xmm4
141a426e3: unpcklps xmm6, xmm2
141a426e6: movlhps xmm4, xmm5
141a426e9: movlhps xmm11, xmm5
141a426ed: shufps xmm6, xmm11, 0x24
141a426f2: shufps xmm1, xmm2, 0xd4
141a426f6: shufps xmm2, xmm3, 0xc5
141a426fa: addps xmm2, xmm4
141a426fd: movlhps xmm12, xmm3
141a42701: shufps xmm1, xmm12, 0x22
141a42706: subps xmm6, xmm1
141a42709: shufps xmm2, xmm6, 0xd8
141a4270d: shufps xmm2, xmm2, 0xd8
141a42711: shufps xmm6, xmm6, 0xff
141a42715: xorps xmm3, xmm3
141a42718: sqrtss xmm3, xmm6
141a4271c: movss xmm1, dword ptr [rip + 0xee314c]
141a42724: divss xmm1, xmm3
141a42728: shufps xmm1, xmm1, 0
141a4272c: mulps xmm1, xmm2
141a4272f: movlps qword ptr [rbp + 0x1e0], xmm0
141a42736: movss dword ptr [rbp + 0x1e8], xmm15
141a4273f: mov dword ptr [rbp + 0x1ec], 0x3e3851eb
141a42749: movss dword ptr [rbp + 0x1f0], xmm7
141a42751: mov dword ptr [rbp + 0x1f4], 0x3e3851eb
141a4275b: movups xmm3, xmmword ptr [rbp + 0x1ec]
141a42762: movaps xmm0, xmmword ptr [rbp + 0x1f0]
141a42769: movaps xmm2, xmmword ptr [rbp + 0x1e0]
141a42770: movaps xmm4, xmm1
141a42773: shufps xmm4, xmm1, 0xff
141a42777: movaps xmm15, xmm1
141a4277b: addps xmm15, xmm1
141a4277f: movaps xmm6, xmm4
141a42782: mulss xmm6, xmm15
141a42787: movaps xmm5, xmm15
141a4278b: unpckhpd xmm5, xmm15
141a42790: mulss xmm4, xmm5
141a42794: movaps xmm14, xmm1
141a42798: mulps xmm14, xmm15
141a4279c: movshdup xmm9, xmm1
141a427a1: movhlps xmm1, xmm1
141a427a4: mulss xmm1, xmm15
141a427a9: movaps xmm11, xmm15
141a427ad: shufps xmm11, xmm15, 0xff
141a427b2: mulss xmm11, xmm9
141a427b7: mulss xmm5, xmm9
141a427bc: mulss xmm15, xmm9
141a427c1: movaps xmm13, xmm14
141a427c5: unpckhpd xmm13, xmm14
141a427ca: movaps xmm9, xmm13
141a427ce: addss xmm9, xmm14
141a427d3: movaps xmm12, xmm10
141a427d7: subss xmm12, xmm9
141a427dc: movaps xmm9, xmm4
141a427e0: addss xmm9, xmm15
141a427e5: unpcklps xmm12, xmm9
141a427e9: movaps xmm9, xmm6
141a427ed: subss xmm9, xmm5
141a427f2: shufps xmm12, xmm9, 4
141a427f7: subss xmm4, xmm15
141a427fc: movaps xmm9, xmm14
141a42800: shufps xmm9, xmm14, 0xff
141a42805: addss xmm14, xmm9
141a4280a: movaps xmm15, xmm10
141a4280e: subss xmm15, xmm14
141a42813: unpcklps xmm4, xmm15
141a42817: movaps xmm14, xmm1
141a4281b: addss xmm14, xmm11
141a42820: shufps xmm4, xmm14, 4
141a42825: addss xmm5, xmm6
141a42829: subss xmm1, xmm11
141a4282e: unpcklps xmm5, xmm1
141a42831: addss xmm9, xmm13
141a42836: movaps xmm1, xmm10
141a4283a: subss xmm1, xmm9
141a4283f: shufps xmm5, xmm1, 4
141a42843: shufps xmm3, xmm3, 0
141a42847: mulps xmm3, xmm12
141a4284b: movaps xmm1, xmm0
141a4284e: shufps xmm1, xmm0, 0
141a42852: mulps xmm1, xmm4
141a42855: shufps xmm0, xmm0, 0x55
141a42859: mulps xmm0, xmm5
141a4285c: shufps xmm2, xmm2, 0xa4
141a42860: movaps xmmword ptr [rbp + 0x160], xmm3
141a42867: movaps xmmword ptr [rbp + 0x170], xmm1
141a4286e: movaps xmmword ptr [rbp + 0x180], xmm0
141a42875: movaps xmmword ptr [rbp + 0x190], xmm2
141a4287c: lea rcx, [rbp + 0xc0]
141a42883: mov rdx, rbx
141a42886: call 0x14089d560
141a4288b: movups xmm0, xmmword ptr [rbp + 0xc0]
141a42892: movups xmm1, xmmword ptr [rbp + 0xd0]
141a42899: movups xmm2, xmmword ptr [rbp + 0xe0]
141a428a0: movaps xmmword ptr [rbp + 0x160], xmm0
141a428a7: movaps xmmword ptr [rbp + 0x170], xmm1
141a428ae: movaps xmmword ptr [rbp + 0x180], xmm2
141a428b5: movaps xmm0, xmmword ptr [rip + 0x10d25e4]
141a428bc: movaps xmmword ptr [rbp + 0x190], xmm0
141a428c3: lea rcx, [rbp + 0x20]
141a428c7: mov rdx, rbx
141a428ca: call 0x1421b6000
141a428cf: inc r12
141a428d2: or r14b, al
141a428d5: cmp r15, r12
141a428d8: jne 0x141a42530
141a428de: mov r12, qword ptr [rbp + 0xb8]
141a428e5: xorps xmm6, xmm6
141a428e8: jmp 0x141a423f0
141a428ed: nop dword ptr [rax]
141a428f0: xor edi, edi
141a428f2: mov rax, qword ptr [rbp + 0x120]
141a428f9: test rax, rax
141a428fc: jne 0x141a42356
141a42902: jmp 0x141a4236c
141a42907: mov rax, qword ptr [rbp + 0x210]
141a4290e: mov qword ptr [rbp + 0x220], rax
141a42915: mov rax, qword ptr [rbp + 0x120]
141a4291c: test rax, rax
141a4291f: je 0x141a4293b
141a42921: shl rax, 2
141a42925: lea rdx, [rax + rax*2]
141a42929: mov r8d, 4
141a4292f: mov rcx, qword ptr [rbp + 0x90]
141a42936: call 0x140613c20
141a4293b: mov rdx, qword ptr [rbp + 0x138]
141a42942: test rdx, rdx
141a42945: je 0x141a4295d
141a42947: mov rcx, qword ptr [rbp + 0x140]
141a4294e: shl rdx, 2
141a42952: mov r8d, 4
141a42958: call 0x140613c20
141a4295d: lea rcx, [rbp + 0xa0]
141a42964: movaps xmm1, xmm7
141a42967: call 0x140b34f80
141a4296c: mov rcx, qword ptr [rbp + 0xa8]
141a42973: mov rax, qword ptr [rbp + 0xb0]
141a4297a: lea rax, [rcx + rax*4]
141a4297e: mov qword ptr [rbp - 0x18], rax
141a42982: mov dword ptr [rbp + 0x78], 0
141a42989: movss dword ptr [rbp + 0x7c], xmm7
141a4298e: mov qword ptr [rbp + 0x80], rcx
141a42995: mov rax, rcx
141a42998: mov qword ptr [rbp + 0x210], 0
141a429a3: mov qword ptr [rbp + 0x90], rsi
141a429aa: jmp 0x141a429b3
141a429ac: nop dword ptr [rax]
141a429b0: mov rax, r15
141a429b3: test rax, rax
141a429b6: je 0x141a429d0
141a429b8: cmp rax, qword ptr [rbp - 0x18]
141a429bc: je 0x141a429d0
141a429be: lea r15, [rax + 4]
141a429c2: jmp 0x141a429f9
141a429c4: nop word ptr cs:[rax + rax]
141a429d0: mov rax, qword ptr [rbp + 0x210]
141a429d7: cmp rax, 8
141a429db: je 0x141a42fd0
141a429e1: mov rcx, rax
141a429e4: add rax, rbp
141a429e7: add rax, 0x78
141a429eb: add rcx, 4
141a429ef: mov qword ptr [rbp + 0x210], rcx
141a429f6: xor r15d, r15d
141a429f9: movss xmm8, dword ptr [rax]
141a429fe: xorps xmm2, xmm2
141a42a01: lea rcx, [rbp + 0xc0]
141a42a08: mov rdx, qword ptr [rbp + 0x228]
141a42a0f: call 0x140c991f0
141a42a14: mov rax, qword ptr [rbp + 0xc8]
141a42a1b: mov qword ptr [rbp + 0x230], rax
141a42a22: mov r13, qword ptr [rbp + 0xd0]
141a42a29: test r13, r13
141a42a2c: je 0x141a42f70
141a42a32: cmp r13, 1
141a42a36: je 0x141a42f70
141a42a3c: mov rax, qword ptr [rbp + 0x230]
141a42a43: movss xmm0, dword ptr [rax + 8]
141a42a48: movss xmm1, dword ptr [rax + 0x14]
141a42a4d: movaps xmm6, xmm0
141a42a50: addss xmm6, xmm1
141a42a54: movsd xmm2, qword ptr [rax]
141a42a58: movsd xmm3, qword ptr [rax + 0xc]
141a42a5d: movaps xmm11, xmm2
141a42a61: addps xmm11, xmm3
141a42a65: mulps xmm11, xmmword ptr [rip + 0xee2e73]
141a42a6d: mulss xmm6, dword ptr [rip + 0xee2dfb]
141a42a75: subss xmm3, xmm2
141a42a79: subss xmm1, xmm0
141a42a7d: movaps xmm0, xmm3
141a42a80: mulss xmm0, xmm3
141a42a84: movaps xmm2, xmm1
141a42a87: mulss xmm2, xmm1
141a42a8b: addss xmm2, xmm0
141a42a8f: xorps xmm0, xmm0
141a42a92: sqrtss xmm0, xmm2
141a42a96: movaps xmm2, xmm10
141a42a9a: divss xmm2, xmm0
141a42a9e: mulss xmm3, xmm2
141a42aa2: mulss xmm2, xmm1
141a42aa6: movss dword ptr [rbp + 0x1e0], xmm3
141a42aae: movss dword ptr [rbp + 0x1e4], xmm2
141a42ab6: xorps xmm2, xmm2
141a42ab9: lea rcx, [rbp + 0x120]
141a42ac0: lea rdx, [rbp + 0x1e0]
141a42ac7: call 0x140c9cde0
141a42acc: mov rcx, rbx
141a42acf: lea rdx, [rbp + 0x120]
141a42ad6: call 0x140c90a40
141a42adb: addss xmm8, dword ptr [rbp + 0x21c]
141a42ae4: movaps xmm15, xmm8
141a42ae8: mulss xmm15, dword ptr [rip + 0xf1db9f]
141a42af1: movaps xmm12, xmm15
141a42af5: unpcklps xmm12, xmm8
141a42af9: addps xmm11, xmm12
141a42afd: addss xmm6, xmm15
141a42b02: movaps xmm3, xmmword ptr [rbp + 0x160]
141a42b09: movss xmm0, dword ptr [rbp + 0x168]
141a42b11: mov rax, qword ptr [rbp + 0x230]
141a42b18: movss xmm1, dword ptr [rax]
141a42b1c: movss xmm2, dword ptr [rax + 4]
141a42b21: subss xmm1, dword ptr [rax + 0xc]
141a42b26: subss xmm2, dword ptr [rax + 0x10]
141a42b2b: movss xmm4, dword ptr [rax + 8]
141a42b30: subss xmm4, dword ptr [rax + 0x14]
141a42b35: mulss xmm1, xmm1
141a42b39: mulss xmm2, xmm2
141a42b3d: addss xmm2, xmm1
141a42b41: mulss xmm4, xmm4
141a42b45: addss xmm4, xmm2
141a42b49: xorps xmm1, xmm1
141a42b4c: sqrtss xmm1, xmm4
141a42b50: movlps qword ptr [rbp + 0x100], xmm11
141a42b58: movss dword ptr [rbp + 0x108], xmm6
141a42b60: movsd xmm2, qword ptr [rip + 0x1049b88]
141a42b68: movsd qword ptr [rbp + 0x10c], xmm2
141a42b70: movss dword ptr [rbp + 0x114], xmm1
141a42b78: movaps xmm6, xmm0
141a42b7b: addss xmm6, xmm0
141a42b7f: mulss xmm0, xmm6
141a42b83: movaps xmm2, xmm3
141a42b86: addps xmm2, xmm3
141a42b89: movshdup xmm1, xmm2
141a42b8d: mulss xmm1, xmm3
141a42b91: movaps xmm4, xmm3
141a42b94: movsldup xmm5, xmm6
141a42b98: mulps xmm5, xmm3
141a42b9b: shufps xmm3, xmm3, 0xff
141a42b9f: mulps xmm4, xmm2
141a42ba2: mulps xmm2, xmm3
141a42ba5: movaps xmm8, xmm3
141a42ba9: mulss xmm8, xmm6
141a42bae: movshdup xmm6, xmm4
141a42bb2: movaps xmm11, xmm6
141a42bb6: addss xmm11, xmm0
141a42bbb: movaps xmm3, xmm10
141a42bbf: subss xmm3, xmm11
141a42bc4: movaps xmm11, xmm1
141a42bc8: addss xmm11, xmm8
141a42bcd: unpcklps xmm3, xmm11
141a42bd1: subss xmm1, xmm8
141a42bd6: addss xmm0, xmm4
141a42bda: movaps xmm8, xmm10
141a42bde: subss xmm8, xmm0
141a42be3: shufps xmm2, xmm2, 0xe1
141a42be7: unpcklps xmm1, xmm8
141a42beb: movaps xmm0, xmm5
141a42bee: subps xmm0, xmm2
141a42bf1: addps xmm2, xmm5
141a42bf4: shufps xmm3, xmm0, 4
141a42bf8: shufps xmm1, xmm2, 0x54
141a42bfc: shufps xmm0, xmm2, 1
141a42c00: addss xmm4, xmm6
141a42c04: movaps xmm2, xmm10
141a42c08: subss xmm2, xmm4
141a42c0c: movups xmm4, xmmword ptr [rbp + 0x10c]
141a42c13: shufps xmm0, xmm2, 2
141a42c17: movaps xmm2, xmmword ptr [rbp + 0x110]
141a42c1e: shufps xmm4, xmm4, 0
141a42c22: mulps xmm4, xmm3
141a42c25: movaps xmm3, xmm2
141a42c28: shufps xmm3, xmm2, 0
141a42c2c: mulps xmm3, xmm1
141a42c2f: shufps xmm2, xmm2, 0x55
141a42c33: mulps xmm2, xmm0
141a42c36: movaps xmmword ptr [rbp + 0x160], xmm4
141a42c3d: movaps xmmword ptr [rbp + 0x170], xmm3
141a42c44: movaps xmmword ptr [rbp + 0x180], xmm2
141a42c4b: movaps xmm0, xmmword ptr [rbp + 0x100]
141a42c52: shufps xmm0, xmm0, 0xa4
141a42c56: movaps xmmword ptr [rbp + 0x190], xmm0
141a42c5d: lea rcx, [rbp + 0x1e0]
141a42c64: mov rdx, rbx
141a42c67: call 0x14089d560
141a42c6c: movups xmm0, xmmword ptr [rbp + 0x1e0]
141a42c73: movups xmm1, xmmword ptr [rbp + 0x1f0]
141a42c7a: movups xmm2, xmmword ptr [rbp + 0x200]
141a42c81: movaps xmmword ptr [rbp + 0x120], xmm0
141a42c88: movaps xmmword ptr [rbp + 0x130], xmm1
141a42c8f: movaps xmmword ptr [rbp + 0x140], xmm2
141a42c96: movaps xmm0, xmmword ptr [rip + 0x10d2203]
141a42c9d: movaps xmmword ptr [rbp + 0x150], xmm0
141a42ca4: lea rcx, [rbp + 0x20]
141a42ca8: lea rdx, [rbp + 0x120]
141a42caf: call 0x1421b6000
141a42cb4: or r14b, al
141a42cb7: inc rdi
141a42cba: cmp r13, 2
141a42cbe: je 0x141a42f70
141a42cc4: lea rax, [r13*2]
141a42ccc: add rax, r13
141a42ccf: mov rcx, qword ptr [rbp + 0x230]
141a42cd6: lea r13, [rcx + rax*4]
141a42cda: lea r12, [rcx + 0xc]
141a42cde: lea rsi, [rcx + 0x18]
141a42ce2: nop word ptr cs:[rax + rax]
141a42cf0: movss xmm0, dword ptr [r12 + 8]
141a42cf7: movss xmm1, dword ptr [rsi + 8]
141a42cfc: movaps xmm6, xmm0
141a42cff: addss xmm6, xmm1
141a42d03: movsd xmm2, qword ptr [r12]
141a42d09: movsd xmm3, qword ptr [rsi]
141a42d0d: movaps xmm8, xmm2
141a42d11: addps xmm8, xmm3
141a42d15: mulps xmm8, xmmword ptr [rip + 0xee2bc3]
141a42d1d: mulss xmm6, dword ptr [rip + 0xee2b4b]
141a42d25: subss xmm3, xmm2
141a42d29: subss xmm1, xmm0
141a42d2d: movaps xmm0, xmm3
141a42d30: mulss xmm0, xmm3
141a42d34: movaps xmm2, xmm1
141a42d37: mulss xmm2, xmm1
141a42d3b: addss xmm2, xmm0
141a42d3f: xorps xmm0, xmm0
141a42d42: sqrtss xmm0, xmm2
141a42d46: movaps xmm2, xmm10
141a42d4a: divss xmm2, xmm0
141a42d4e: mulss xmm3, xmm2
141a42d52: mulss xmm2, xmm1
141a42d56: movss dword ptr [rbp + 0x1e0], xmm3
141a42d5e: movss dword ptr [rbp + 0x1e4], xmm2
141a42d66: xorps xmm2, xmm2
141a42d69: lea rcx, [rbp + 0x120]
141a42d70: lea rdx, [rbp + 0x1e0]
141a42d77: call 0x140c9cde0
141a42d7c: mov rcx, rbx
141a42d7f: lea rdx, [rbp + 0x120]
141a42d86: call 0x140c90a40
141a42d8b: addps xmm8, xmm12
141a42d8f: addss xmm6, xmm15
141a42d94: movaps xmm5, xmmword ptr [rbp + 0x160]
141a42d9b: movss xmm3, dword ptr [rbp + 0x168]
141a42da3: movss xmm0, dword ptr [r12]
141a42da9: movss xmm1, dword ptr [r12 + 4]
141a42db0: subss xmm0, dword ptr [rsi]
141a42db4: subss xmm1, dword ptr [rsi + 4]
141a42db9: movss xmm2, dword ptr [r12 + 8]
141a42dc0: subss xmm2, dword ptr [rsi + 8]
141a42dc5: mulss xmm0, xmm0
141a42dc9: mulss xmm1, xmm1
141a42dcd: addss xmm1, xmm0
141a42dd1: mulss xmm2, xmm2
141a42dd5: addss xmm2, xmm1
141a42dd9: xorps xmm0, xmm0
141a42ddc: sqrtss xmm0, xmm2
141a42de0: movlps qword ptr [rbp + 0x100], xmm8
141a42de8: movss dword ptr [rbp + 0x108], xmm6
141a42df0: movsd xmm1, qword ptr [rip + 0x10498f8]
141a42df8: movsd qword ptr [rbp + 0x10c], xmm1
141a42e00: movss dword ptr [rbp + 0x114], xmm0
141a42e08: movups xmm2, xmmword ptr [rbp + 0x10c]
141a42e0f: movaps xmm0, xmmword ptr [rbp + 0x110]
141a42e16: movaps xmm1, xmmword ptr [rbp + 0x100]
141a42e1d: movaps xmm13, xmm3
141a42e21: addss xmm13, xmm3
141a42e26: mulss xmm3, xmm13
141a42e2b: movaps xmm6, xmm5
141a42e2e: addps xmm6, xmm5
141a42e31: movshdup xmm4, xmm6
141a42e35: mulss xmm4, xmm5
141a42e39: movaps xmm8, xmm5
141a42e3d: movsldup xmm11, xmm13
141a42e42: mulps xmm11, xmm5
141a42e46: shufps xmm5, xmm5, 0xff
141a42e4a: mulps xmm8, xmm6
141a42e4e: mulps xmm6, xmm5
141a42e51: movaps xmm14, xmm5
141a42e55: mulss xmm14, xmm13
141a42e5a: shufps xmm6, xmm6, 0xe1
141a42e5e: movshdup xmm9, xmm8
141a42e63: movaps xmm5, xmm9
141a42e67: addss xmm5, xmm3
141a42e6b: movaps xmm13, xmm10
141a42e6f: subss xmm13, xmm5
141a42e74: movaps xmm5, xmm4
141a42e77: addss xmm5, xmm14
141a42e7c: unpcklps xmm13, xmm5
141a42e80: movaps xmm5, xmm11
141a42e84: subps xmm5, xmm6
141a42e87: shufps xmm13, xmm5, 4
141a42e8c: subss xmm4, xmm14
141a42e91: addss xmm3, xmm8
141a42e96: movaps xmm14, xmm10
141a42e9a: subss xmm14, xmm3
141a42e9f: unpcklps xmm4, xmm14
141a42ea3: addps xmm6, xmm11
141a42ea7: shufps xmm4, xmm6, 0x54
141a42eab: shufps xmm5, xmm6, 1
141a42eaf: addss xmm8, xmm9
141a42eb4: movaps xmm3, xmm10
141a42eb8: subss xmm3, xmm8
141a42ebd: shufps xmm5, xmm3, 2
141a42ec1: shufps xmm2, xmm2, 0
141a42ec5: mulps xmm2, xmm13
141a42ec9: movaps xmm3, xmm0
141a42ecc: shufps xmm3, xmm0, 0
141a42ed0: mulps xmm3, xmm4
141a42ed3: shufps xmm0, xmm0, 0x55
141a42ed7: mulps xmm0, xmm5
141a42eda: shufps xmm1, xmm1, 0xa4
141a42ede: movaps xmmword ptr [rbp + 0x160], xmm2
141a42ee5: movaps xmmword ptr [rbp + 0x170], xmm3
141a42eec: movaps xmmword ptr [rbp + 0x180], xmm0
141a42ef3: movaps xmmword ptr [rbp + 0x190], xmm1
141a42efa: lea rcx, [rbp + 0x1e0]
141a42f01: mov rdx, rbx
141a42f04: call 0x14089d560
141a42f09: movups xmm0, xmmword ptr [rbp + 0x1e0]
141a42f10: movups xmm1, xmmword ptr [rbp + 0x1f0]
141a42f17: movups xmm2, xmmword ptr [rbp + 0x200]
141a42f1e: movaps xmmword ptr [rbp + 0x120], xmm0
141a42f25: movaps xmmword ptr [rbp + 0x130], xmm1
141a42f2c: movaps xmmword ptr [rbp + 0x140], xmm2
141a42f33: movaps xmm0, xmmword ptr [rip + 0x10d1f66]
141a42f3a: movaps xmmword ptr [rbp + 0x150], xmm0
141a42f41: lea rcx, [rbp + 0x20]
141a42f45: lea rdx, [rbp + 0x120]
141a42f4c: call 0x1421b6000
141a42f51: mov r12, rsi
141a42f54: lea rcx, [rsi + 0xc]
141a42f58: or r14b, al
141a42f5b: inc rdi
141a42f5e: mov rsi, rcx
141a42f61: cmp rcx, r13
141a42f64: jne 0x141a42cf0
141a42f6a: nop word ptr [rax + rax]
141a42f70: mov rax, qword ptr [rbp + 0xc0]
141a42f77: test rax, rax
141a42f7a: je 0x141a42f96
141a42f7c: shl rax, 2
141a42f80: lea rdx, [rax + rax*2]
141a42f84: mov r8d, 4
141a42f8a: mov rcx, qword ptr [rbp + 0x230]
141a42f91: call 0x140613c20
141a42f96: mov rdx, qword ptr [rbp + 0xd8]
141a42f9d: test rdx, rdx
141a42fa0: mov rsi, qword ptr [rbp + 0x90]
141a42fa7: mov r12, qword ptr [rbp + 0xb8]
141a42fae: je 0x141a429b0
141a42fb4: mov rcx, qword ptr [rbp + 0xe0]
141a42fbb: shl rdx, 2
141a42fbf: mov r8d, 4
141a42fc5: call 0x140613c20
141a42fca: jmp 0x141a429b0
141a42fcf: nop 
141a42fd0: mov rdx, qword ptr [rbp + 0xa0]
141a42fd7: test rdx, rdx
141a42fda: je 0x141a42ff2
141a42fdc: shl rdx, 2
141a42fe0: mov r8d, 4
141a42fe6: mov rcx, qword ptr [rbp + 0x80]
141a42fed: call 0x140613c20
141a42ff2: mov rcx, qword ptr [rbp + 0x98]
141a42ff9: cmp byte ptr [rcx + 0x1c], 0
141a42ffd: jne 0x141a41f2d
141a43003: call 0x1408e3c70
141a43008: movaps xmm8, xmm0
141a4300c: mov rax, qword ptr [rbp + 0x98]
141a43013: movss xmm0, dword ptr [rax + 0x14]
141a43018: movss xmm1, dword ptr [rax + 0x18]
141a4301d: call 0x1408e3b60
141a43022: mov rax, qword ptr [rbp + 0x98]
141a43029: movss xmm2, dword ptr [rax + 0x14]
141a4302e: movss xmm3, dword ptr [rax + 0x18]
141a43033: ucomiss xmm3, xmm2
141a43036: seta dl
141a43039: call 0x1408e2700
141a4303e: mov qword ptr [rbp + 0x160], rax
141a43045: mov rcx, rbx
141a43048: call 0x1400885d0
141a4304d: mov rdx, qword ptr [rbp + 0x228]
141a43054: movups xmm0, xmmword ptr [rdx]
141a43057: movaps xmmword ptr [rbp + 0xa0], xmm0
141a4305e: mov qword ptr [rbp + 0xb0], rax
141a43065: lea rcx, [rbp + 0xc0]
141a4306c: call 0x140c98cf0
141a43071: mov r15, qword ptr [rbp + 0xc8]
141a43078: mov r13, qword ptr [rbp + 0xd0]
141a4307f: lea rcx, [rbp + 0x120]
141a43086: lea rdx, [rbp + 0xa0]
141a4308d: call 0x140c98cf0
141a43092: mov rax, qword ptr [rbp + 0x130]
141a43099: cmp rax, r13
141a4309c: cmovb r13, rax
141a430a0: mov r12, qword ptr [rbp + 0x128]
141a430a7: movss xmm0, dword ptr [rbp + 0x21c]
141a430af: addss xmm0, xmm7
141a430b3: addss xmm8, xmm0
141a430b8: test r13, r13
141a430bb: mov qword ptr [rbp + 0x228], r15
141a430c2: je 0x141a43349
141a430c8: movaps xmm9, xmm0
141a430cc: add rdi, r13
141a430cf: mov qword ptr [rbp + 0x230], r12
141a430d6: nop word ptr cs:[rax + rax]
141a430e0: lea rcx, [rbp - 0x28]
141a430e4: mov rdx, r15
141a430e7: movaps xmm2, xmm9
141a430eb: call 0x140c9cde0
141a430f0: lea rcx, [rbp - 0x38]
141a430f4: mov rdx, r12
141a430f7: movaps xmm2, xmm8
141a430fb: call 0x140c9cde0
141a43100: movss xmm6, dword ptr [rbp - 0x20]
141a43105: movsd xmm7, qword ptr [rbp - 0x28]
141a4310a: movsd xmm12, qword ptr [rbp - 0x38]
141a43110: movss xmm11, dword ptr [rbp - 0x30]
141a43116: movaps xmm0, xmm11
141a4311a: subss xmm0, xmm6
141a4311e: movaps xmm1, xmm0
141a43121: mulss xmm1, xmm0
141a43125: movaps xmm2, xmm12
141a43129: subps xmm2, xmm7
141a4312c: movaps xmm3, xmm2
141a4312f: mulps xmm3, xmm2
141a43132: movshdup xmm4, xmm3
141a43136: addss xmm4, xmm3
141a4313a: addss xmm4, xmm1
141a4313e: xorps xmm1, xmm1
141a43141: sqrtss xmm1, xmm4
141a43145: movaps xmm3, xmm10
141a43149: divss xmm3, xmm1
141a4314d: movsldup xmm1, xmm3
141a43151: mulps xmm1, xmm2
141a43154: mulss xmm3, xmm0
141a43158: movlps qword ptr [rbp + 0x1e0], xmm1
141a4315f: movss dword ptr [rbp + 0x1e8], xmm3
141a43167: mov rcx, rbx
141a4316a: lea rdx, [rbp + 0x1e0]
141a43171: call 0x140c90a40
141a43176: movaps xmm0, xmm6
141a43179: addss xmm0, xmm11
141a4317e: mulss xmm0, dword ptr [rip + 0xee26ea]
141a43186: movaps xmm1, xmm7
141a43189: addps xmm1, xmm12
141a4318d: movaps xmm15, xmmword ptr [rbp + 0x160]
141a43195: movss xmm3, dword ptr [rbp + 0x168]
141a4319d: subps xmm7, xmm12
141a431a1: subss xmm6, xmm11
141a431a6: mulps xmm7, xmm7
141a431a9: movshdup xmm2, xmm7
141a431ad: addss xmm2, xmm7
141a431b1: mulss xmm6, xmm6
141a431b5: addss xmm6, xmm2
141a431b9: xorps xmm2, xmm2
141a431bc: sqrtss xmm2, xmm6
141a431c0: mulps xmm1, xmmword ptr [rip + 0xee2719]
141a431c7: movlps qword ptr [rbp + 0x100], xmm1
141a431ce: movss dword ptr [rbp + 0x108], xmm0
141a431d6: movsd xmm0, qword ptr [rip + 0x1049512]
141a431de: movsd qword ptr [rbp + 0x10c], xmm0
141a431e6: movss dword ptr [rbp + 0x114], xmm2
141a431ee: movups xmm2, xmmword ptr [rbp + 0x10c]
141a431f5: movaps xmm0, xmmword ptr [rbp + 0x110]
141a431fc: movaps xmm1, xmmword ptr [rbp + 0x100]
141a43203: movaps xmm7, xmm3
141a43206: addss xmm7, xmm3
141a4320a: mulss xmm3, xmm7
141a4320e: movaps xmm5, xmm15
141a43212: addps xmm5, xmm15
141a43216: movshdup xmm4, xmm5
141a4321a: mulss xmm4, xmm15
141a4321f: movaps xmm6, xmm15
141a43223: movsldup xmm11, xmm7
141a43228: mulps xmm11, xmm15
141a4322c: shufps xmm15, xmm15, 0xff
141a43231: mulps xmm6, xmm5
141a43234: mulps xmm5, xmm15
141a43238: mulss xmm15, xmm7
141a4323d: shufps xmm5, xmm5, 0xe1
141a43241: movshdup xmm13, xmm6
141a43246: movaps xmm7, xmm13
141a4324a: addss xmm7, xmm3
141a4324e: movaps xmm12, xmm10
141a43252: subss xmm12, xmm7
141a43257: movaps xmm7, xmm4
141a4325a: addss xmm7, xmm15
141a4325f: unpcklps xmm12, xmm7
141a43263: movaps xmm7, xmm11
141a43267: subps xmm7, xmm5
141a4326a: shufps xmm12, xmm7, 4
141a4326f: subss xmm4, xmm15
141a43274: addss xmm3, xmm6
141a43278: movaps xmm14, xmm10
141a4327c: subss xmm14, xmm3
141a43281: unpcklps xmm4, xmm14
141a43285: addps xmm5, xmm11
141a43289: shufps xmm4, xmm5, 0x54
141a4328d: shufps xmm7, xmm5, 1
141a43291: addss xmm6, xmm13
141a43296: movaps xmm3, xmm10
141a4329a: subss xmm3, xmm6
141a4329e: shufps xmm7, xmm3, 2
141a432a2: shufps xmm2, xmm2, 0
141a432a6: mulps xmm2, xmm12
141a432aa: movaps xmm3, xmm0
141a432ad: shufps xmm3, xmm0, 0
141a432b1: mulps xmm3, xmm4
141a432b4: shufps xmm0, xmm0, 0x55
141a432b8: mulps xmm0, xmm7
141a432bb: shufps xmm1, xmm1, 0xa4
141a432bf: movaps xmmword ptr [rbp + 0x160], xmm2
141a432c6: movaps xmmword ptr [rbp + 0x170], xmm3
141a432cd: movaps xmmword ptr [rbp + 0x180], xmm0
141a432d4: movaps xmmword ptr [rbp + 0x190], xmm1
141a432db: lea rcx, [rbp + 0x1e0]
141a432e2: mov rdx, rbx
141a432e5: call 0x14089d560
141a432ea: movups xmm0, xmmword ptr [rbp + 0x1e0]
141a432f1: movups xmm1, xmmword ptr [rbp + 0x1f0]
141a432f8: movups xmm2, xmmword ptr [rbp + 0x200]
141a432ff: movaps xmmword ptr [rbp + 0x160], xmm0
141a43306: movaps xmmword ptr [rbp + 0x170], xmm1
141a4330d: movaps xmmword ptr [rbp + 0x180], xmm2
141a43314: movaps xmm0, xmmword ptr [rip + 0x10d1b85]
141a4331b: movaps xmmword ptr [rbp + 0x190], xmm0
141a43322: lea rcx, [rbp + 0x20]
141a43326: mov rdx, rbx
141a43329: call 0x1421b6000
141a4332e: or r14b, al
141a43331: add r12, 8
141a43335: add r15, 8
141a43339: dec r13
141a4333c: jne 0x141a430e0
141a43342: mov r12, qword ptr [rbp + 0x230]
141a43349: mov rdx, qword ptr [rbp + 0x120]
141a43350: test rdx, rdx
141a43353: je 0x141a43367
141a43355: shl rdx, 3
141a43359: mov r8d, 4
141a4335f: mov rcx, r12
141a43362: call 0x140613c20
141a43367: mov rdx, qword ptr [rbp + 0x138]
141a4336e: test rdx, rdx
141a43371: mov r12, qword ptr [rbp + 0xb8]
141a43378: je 0x141a43390
141a4337a: mov rcx, qword ptr [rbp + 0x140]
141a43381: shl rdx, 2
141a43385: mov r8d, 4
141a4338b: call 0x140613c20
141a43390: mov rdx, qword ptr [rbp + 0xc0]
141a43397: test rdx, rdx
141a4339a: je 0x141a433b2
141a4339c: shl rdx, 3
141a433a0: mov r8d, 4
141a433a6: mov rcx, qword ptr [rbp + 0x228]
141a433ad: call 0x140613c20
141a433b2: mov rdx, qword ptr [rbp + 0xd8]
141a433b9: test rdx, rdx
141a433bc: je 0x141a433d4
141a433be: mov rcx, qword ptr [rbp + 0xe0]
141a433c5: shl rdx, 2
141a433c9: mov r8d, 4
141a433cf: call 0x140613c20
141a433d4: lea rcx, [rbp + 0xa0]
141a433db: call 0x140c99530
141a433e0: ucomiss xmm0, dword ptr [rip + 0xee2489]
141a433e7: jbe 0x141a41f2d
141a433ed: movss xmm0, dword ptr [rbp + 0xb4]
141a433f5: xor r15d, r15d
141a433f8: ucomiss xmm0, dword ptr [rbp + 0xb0]
141a433ff: setbe r15b
141a43403: lea rcx, [rbp + 0xa0]
141a4340a: mov rdx, r15
141a4340d: call 0x140c99600
141a43412: movss dword ptr [rbp + 0x160], xmm0
141a4341a: movss dword ptr [rbp + 0x164], xmm1
141a43422: lea rcx, [rbp - 0x48]
141a43426: mov rdx, rbx
141a43429: movaps xmm2, xmm8
141a4342d: call 0x140c9cde0
141a43432: inc r15
141a43435: lea rcx, [rbp + 0xa0]
141a4343c: mov rdx, r15
141a4343f: call 0x140c99600
141a43444: movss dword ptr [rbp + 0x160], xmm0
141a4344c: movss dword ptr [rbp + 0x164], xmm1
141a43454: lea rcx, [rbp - 0x58]
141a43458: mov rdx, rbx
141a4345b: movaps xmm2, xmm8
141a4345f: call 0x140c9cde0
141a43464: movss xmm6, dword ptr [rbp - 0x40]
141a43469: movsd xmm7, qword ptr [rbp - 0x48]
141a4346e: movsd xmm11, qword ptr [rbp - 0x58]
141a43474: movss xmm8, dword ptr [rbp - 0x50]
141a4347a: movaps xmm0, xmm8
141a4347e: subss xmm0, xmm6
141a43482: movaps xmm1, xmm0
141a43485: mulss xmm1, xmm0
141a43489: movaps xmm2, xmm11
141a4348d: subps xmm2, xmm7
141a43490: movaps xmm3, xmm2
141a43493: mulps xmm3, xmm2
141a43496: movshdup xmm4, xmm3
141a4349a: addss xmm4, xmm3
141a4349e: addss xmm4, xmm1
141a434a2: xorps xmm1, xmm1
141a434a5: sqrtss xmm1, xmm4
141a434a9: movaps xmm3, xmm10
141a434ad: divss xmm3, xmm1
141a434b1: movsldup xmm1, xmm3
141a434b5: mulps xmm1, xmm2
141a434b8: mulss xmm3, xmm0
141a434bc: movlps qword ptr [rbp + 0x120], xmm1
141a434c3: movss dword ptr [rbp + 0x128], xmm3
141a434cb: mov rcx, rbx
141a434ce: lea rdx, [rbp + 0x120]
141a434d5: call 0x140c90a40
141a434da: movaps xmm0, xmm6
141a434dd: addss xmm0, xmm8
141a434e2: mulss xmm0, dword ptr [rip + 0xee2386]
141a434ea: movaps xmm1, xmm7
141a434ed: addps xmm1, xmm11
141a434f1: movaps xmm12, xmmword ptr [rbp + 0x160]
141a434f9: movss xmm3, dword ptr [rbp + 0x168]
141a43501: subps xmm7, xmm11
141a43505: subss xmm6, xmm8
141a4350a: mulps xmm7, xmm7
141a4350d: movshdup xmm2, xmm7
141a43511: addss xmm2, xmm7
141a43515: mulss xmm6, xmm6
141a43519: addss xmm6, xmm2
141a4351d: xorps xmm2, xmm2
141a43520: sqrtss xmm2, xmm6
141a43524: mulps xmm1, xmmword ptr [rip + 0xee23b5]
141a4352b: movlps qword ptr [rbp + 0xc0], xmm1
141a43532: movss dword ptr [rbp + 0xc8], xmm0
141a4353a: movsd xmm0, qword ptr [rip + 0x10491ae]
141a43542: movsd qword ptr [rbp + 0xcc], xmm0
141a4354a: movss dword ptr [rbp + 0xd4], xmm2
141a43552: movups xmm2, xmmword ptr [rbp + 0xcc]
141a43559: movaps xmm0, xmmword ptr [rbp + 0xd0]
141a43560: movaps xmm1, xmmword ptr [rbp + 0xc0]
141a43567: movaps xmm7, xmm3
141a4356a: addss xmm7, xmm3
141a4356e: mulss xmm3, xmm7
141a43572: movaps xmm5, xmm12
141a43576: addps xmm5, xmm12
141a4357a: movshdup xmm4, xmm5
141a4357e: mulss xmm4, xmm12
141a43583: movaps xmm6, xmm12
141a43587: movsldup xmm8, xmm7
141a4358c: mulps xmm8, xmm12
141a43590: shufps xmm12, xmm12, 0xff
141a43595: mulps xmm6, xmm5
141a43598: mulps xmm5, xmm12
141a4359c: mulss xmm12, xmm7
141a435a1: shufps xmm5, xmm5, 0xe1
141a435a5: movshdup xmm13, xmm6
141a435aa: movaps xmm7, xmm13
141a435ae: addss xmm7, xmm3
141a435b2: movaps xmm11, xmm10
141a435b6: subss xmm11, xmm7
141a435bb: movaps xmm7, xmm4
141a435be: addss xmm7, xmm12
141a435c3: unpcklps xmm11, xmm7
141a435c7: movaps xmm7, xmm8
141a435cb: subps xmm7, xmm5
141a435ce: shufps xmm11, xmm7, 4
141a435d3: subss xmm4, xmm12
141a435d8: addss xmm3, xmm6
141a435dc: movaps xmm12, xmm10
141a435e0: subss xmm12, xmm3
141a435e5: unpcklps xmm4, xmm12
141a435e9: addps xmm5, xmm8
141a435ed: shufps xmm4, xmm5, 0x54
141a435f1: shufps xmm7, xmm5, 1
141a435f5: addss xmm6, xmm13
141a435fa: movaps xmm3, xmm10
141a435fe: subss xmm3, xmm6
141a43602: shufps xmm7, xmm3, 2
141a43606: shufps xmm2, xmm2, 0
141a4360a: mulps xmm2, xmm11
141a4360e: movaps xmm3, xmm0
141a43611: shufps xmm3, xmm0, 0
141a43615: mulps xmm3, xmm4
141a43618: shufps xmm0, xmm0, 0x55
141a4361c: mulps xmm0, xmm7
141a4361f: shufps xmm1, xmm1, 0xa4
141a43623: movaps xmmword ptr [rbp + 0x160], xmm2
141a4362a: movaps xmmword ptr [rbp + 0x170], xmm3
141a43631: movaps xmmword ptr [rbp + 0x180], xmm0
141a43638: movaps xmmword ptr [rbp + 0x190], xmm1
141a4363f: lea rcx, [rbp + 0x120]
141a43646: mov rdx, rbx
141a43649: call 0x14089d560
141a4364e: movups xmm0, xmmword ptr [rbp + 0x120]
141a43655: movups xmm1, xmmword ptr [rbp + 0x130]
141a4365c: movups xmm2, xmmword ptr [rbp + 0x140]
141a43663: movaps xmmword ptr [rbp + 0x160], xmm0
141a4366a: movaps xmmword ptr [rbp + 0x170], xmm1
141a43671: movaps xmmword ptr [rbp + 0x180], xmm2
141a43678: movaps xmm0, xmmword ptr [rip + 0x10d1821]
141a4367f: movaps xmmword ptr [rbp + 0x190], xmm0
141a43686: lea rcx, [rbp + 0x20]
141a4368a: mov rdx, rbx
141a4368d: call 0x1421b6000
141a43692: jmp 0x141a41f27
141a43697: test r14b, 1
141a4369b: je 0x141a4376a
141a436a1: mov rax, qword ptr [rip + 0x1be8d00]
141a436a8: mov rax, qword ptr [rax]
141a436ab: test rax, rax
141a436ae: je 0x141a4376a
141a436b4: lea rcx, [rip + 0x1478d15]
141a436bb: call 0x14067dd80
141a436c0: movups xmm0, xmmword ptr [rax]
141a436c3: mov eax, dword ptr [rax + 0x10]
141a436c6: mov qword ptr [rbp + 0x190], 1
141a436d1: lea rcx, [rip + 0x1478d10]
141a436d8: mov qword ptr [rbp + 0x198], rcx
141a436df: mov qword ptr [rbp + 0x1a0], 0x22
141a436ea: lea rdx, [rip + 0x1478ccf]
141a436f1: mov qword ptr [rbp + 0x1b0], rdx
141a436f8: mov qword ptr [rbp + 0x1b8], 1
141a43703: mov qword ptr [rbp + 0x1c0], 8
141a4370e: mov qword ptr [rbp + 0x160], 0
141a43719: xorps xmm1, xmm1
141a4371c: movups xmmword ptr [rbp + 0x1c8], xmm1
141a43723: mov qword ptr [rbp + 0x168], rcx
141a4372a: mov qword ptr [rbp + 0x170], 0x22
141a43735: mov qword ptr [rbp + 0x178], 0
141a43740: movups xmmword ptr [rbp + 0x180], xmm0
141a43747: mov dword ptr [rbp + 0x1a8], 1
141a43751: mov dword ptr [rbp + 0x1ac], eax
141a43757: lea rcx, [rbp + 0x120]
141a4375e: lea rdx, [rbp + 0x160]
141a43765: call 0x140d4a330
141a4376a: mov rdi, qword ptr [rbp + 0x40]
141a4376e: lea rsi, [rdi + 0x18]
141a43772: mov rcx, rsi
141a43775: call 0x140c81e10
141a4377a: xor ecx, ecx
141a4377c: mov al, 1
141a4377e: lock cmpxchg byte ptr [rdi + 0x18], cl
141a43783: jne 0x141a43844
141a43789: movaps xmm6, xmmword ptr [rbp + 0x240]
141a43790: movaps xmm7, xmmword ptr [rbp + 0x250]
141a43797: movaps xmm8, xmmword ptr [rbp + 0x260]
141a4379f: movaps xmm9, xmmword ptr [rbp + 0x270]
141a437a7: movaps xmm10, xmmword ptr [rbp + 0x280]
141a437af: movaps xmm11, xmmword ptr [rbp + 0x290]
141a437b7: movaps xmm12, xmmword ptr [rbp + 0x2a0]
141a437bf: movaps xmm13, xmmword ptr [rbp + 0x2b0]
141a437c7: movaps xmm14, xmmword ptr [rbp + 0x2c0]
141a437cf: movaps xmm15, xmmword ptr [rbp + 0x2d0]
141a437d7: add rsp, 0x368
141a437de: pop rbx
141a437df: pop rdi
141a437e0: pop rsi
141a437e1: pop r12
141a437e3: pop r13
141a437e5: pop r14
141a437e7: pop r15
141a437e9: pop rbp
141a437ea: ret 
141a437eb: lea rcx, [rip + 0x15b523e]
141a437f2: lea rdx, [rip + 0x15b5297]
141a437f9: call 0x1428a9a00
141a437fe: mov qword ptr [rbp + 0x160], 0
141a43809: add rbx, 0x10
141a4380d: lea rax, [rip + 0x15b5414]
141a43814: mov qword ptr [rsp + 0x20], rax
141a43819: lea rdx, [rbp + 0x20]
141a4381d: lea r9, [rbp + 0x160]
141a43824: xor ecx, ecx
141a43826: mov r8, rbx
141a43829: call 0x1428cab41
141a4382e: mov r14, rcx
141a43831: mov r8d, 0x3b9aca00
141a43837: call 0x1428cd210
141a4383c: mov rcx, r14
141a4383f: jmp 0x141a41df5
141a43844: mov rcx, rsi
141a43847: xor edx, edx
141a43849: call 0x1428cdb90
141a4384e: jmp 0x141a43789
141a43853: mov qword ptr [rbp + 0x160], 0
141a4385e: add r14, 0x10
141a43862: lea rax, [rip + 0x15b53bf]
141a43869: mov qword ptr [rsp + 0x20], rax
141a4386e: lea rdx, [rbp + 0x20]
141a43872: lea r9, [rbp + 0x160]
141a43879: xor ecx, ecx
141a4387b: mov r8, r14
141a4387e: call 0x1428cab41
141a43883: lea rcx, [rip + 0x1478ace]
141a4388a: call 0x1428d9310
141a4388f: lea rcx, [rip + 0x1478ada]
141a43896: call 0x1428d9310
141a4389b: lea rcx, [rip + 0x15b5526]
141a438a2: call 0x1428d9310
141a438a7: lea rcx, [rip + 0x1478a92]
141a438ae: call 0x1428d9310
141a438b3: lea rcx, [rip + 0x15b5526]
141a438ba: call 0x1428d9310
141a438bf: mov ecx, 8
141a438c4: mov edx, 0x20
141a438c9: call 0x1428d9000
141a438ce: int3 
141a438cf: nop 
141a438d0: mov qword ptr [rsp + 0x10], rdx
141a438d5: push rbp
141a438d6: push r15
141a438d8: push r14
141a438da: push r13
141a438dc: push r12
141a438de: push rsi
141a438df: push rdi
141a438e0: push rbx
141a438e1: sub rsp, 0xc8
141a438e8: lea rbp, [rdx + 0x80]
141a438ef: movaps xmmword ptr [rsp + 0x20], xmm15
141a438f5: movaps xmmword ptr [rsp + 0x30], xmm14
141a438fb: movaps xmmword ptr [rsp + 0x40], xmm13
141a43901: movaps xmmword ptr [rsp + 0x50], xmm12
141a43907: movaps xmmword ptr [rsp + 0x60], xmm11
141a4390d: movaps xmmword ptr [rsp + 0x70], xmm10
141a43913: movaps xmmword ptr [rsp + 0x80], xmm9
141a4391c: movaps xmmword ptr [rsp + 0x90], xmm8
141a43925: movaps xmmword ptr [rsp + 0xa0], xmm7
141a4392d: movaps xmmword ptr [rsp + 0xb0], xmm6
141a43935: mov rdi, qword ptr [rbp + 0x40]
141a43939: lea rsi, [rdi + 0x18]
141a4393d: mov rcx, rsi
141a43940: call 0x140c81e10
141a43945: xor ecx, ecx
141a43947: mov al, 1
141a43949: lock cmpxchg byte ptr [rdi + 0x18], cl
141a4394e: jne 0x141a439aa
141a43950: movaps xmm6, xmmword ptr [rsp + 0xb0]
141a43958: movaps xmm7, xmmword ptr [rsp + 0xa0]
141a43960: movaps xmm8, xmmword ptr [rsp + 0x90]
141a43969: movaps xmm9, xmmword ptr [rsp + 0x80]
141a43972: movaps xmm10, xmmword ptr [rsp + 0x70]
141a43978: movaps xmm11, xmmword ptr [rsp + 0x60]
141a4397e: movaps xmm12, xmmword ptr [rsp + 0x50]
141a43984: movaps xmm13, xmmword ptr [rsp + 0x40]
141a4398a: movaps xmm14, xmmword ptr [rsp + 0x30]
141a43990: movaps xmm15, xmmword ptr [rsp + 0x20]
141a43996: add rsp, 0xc8
141a4399d: pop rbx
141a4399e: pop rdi
141a4399f: pop rsi
141a439a0: pop r12
141a439a2: pop r13
141a439a4: pop r14
141a439a6: pop r15
141a439a8: pop rbp
141a439a9: ret 
141a439aa: mov rcx, rsi
141a439ad: xor edx, edx
141a439af: call 0x1428cdb90
141a439b4: jmp 0x141a43950
141a439b6: nop word ptr cs:[rax + rax]
141a439c0: mov qword ptr [rsp + 0x10], rdx
141a439c5: push rbp
141a439c6: push r15
141a439c8: push r14
141a439ca: push r13
141a439cc: push r12
141a439ce: push rsi
141a439cf: push rdi
141a439d0: push rbx
141a439d1: sub rsp, 0xc8
141a439d8: lea rbp, [rdx + 0x80]
141a439df: movaps xmmword ptr [rsp + 0x20], xmm15
141a439e5: movaps xmmword ptr [rsp + 0x30], xmm14
141a439eb: movaps xmmword ptr [rsp + 0x40], xmm13
141a439f1: movaps xmmword ptr [rsp + 0x50], xmm12
141a439f7: movaps xmmword ptr [rsp + 0x60], xmm11
141a439fd: movaps xmmword ptr [rsp + 0x70], xmm10
141a43a03: movaps xmmword ptr [rsp + 0x80], xmm9
141a43a0c: movaps xmmword ptr [rsp + 0x90], xmm8
141a43a15: movaps xmmword ptr [rsp + 0xa0], xmm7
141a43a1d: movaps xmmword ptr [rsp + 0xb0], xmm6
141a43a25: lea rcx, [rbp + 0x120]
141a43a2c: call 0x14014ba20
141a43a31: movaps xmm6, xmmword ptr [rsp + 0xb0]
141a43a39: movaps xmm7, xmmword ptr [rsp + 0xa0]
141a43a41: movaps xmm8, xmmword ptr [rsp + 0x90]
141a43a4a: movaps xmm9, xmmword ptr [rsp + 0x80]
141a43a53: movaps xmm10, xmmword ptr [rsp + 0x70]
141a43a59: movaps xmm11, xmmword ptr [rsp + 0x60]
141a43a5f: movaps xmm12, xmmword ptr [rsp + 0x50]
141a43a65: movaps xmm13, xmmword ptr [rsp + 0x40]
141a43a6b: movaps xmm14, xmmword ptr [rsp + 0x30]
141a43a71: movaps xmm15, xmmword ptr [rsp + 0x20]
141a43a77: add rsp, 0xc8
141a43a7e: pop rbx
141a43a7f: pop rdi
141a43a80: pop rsi
141a43a81: pop r12
141a43a83: pop r13
141a43a85: pop r14
141a43a87: pop r15
141a43a89: pop rbp
141a43a8a: ret 
141a43a8b: nop dword ptr [rax + rax]
141a43a90: mov qword ptr [rsp + 0x10], rdx
141a43a95: push rbp
141a43a96: push r15
141a43a98: push r14
141a43a9a: push r13
141a43a9c: push r12
141a43a9e: push rsi
141a43a9f: push rdi
141a43aa0: push rbx
141a43aa1: sub rsp, 0xc8
141a43aa8: lea rbp, [rdx + 0x80]
141a43aaf: movaps xmmword ptr [rsp + 0x20], xmm15
141a43ab5: movaps xmmword ptr [rsp + 0x30], xmm14
141a43abb: movaps xmmword ptr [rsp + 0x40], xmm13
141a43ac1: movaps xmmword ptr [rsp + 0x50], xmm12
141a43ac7: movaps xmmword ptr [rsp + 0x60], xmm11
141a43acd: movaps xmmword ptr [rsp + 0x70], xmm10
141a43ad3: movaps xmmword ptr [rsp + 0x80], xmm9
141a43adc: movaps xmmword ptr [rsp + 0x90], xmm8
141a43ae5: movaps xmmword ptr [rsp + 0xa0], xmm7
141a43aed: movaps xmmword ptr [rsp + 0xb0], xmm6
141a43af5: lea rcx, [rbp + 0x120]
141a43afc: call 0x14014ba20
141a43b01: movaps xmm6, xmmword ptr [rsp + 0xb0]
141a43b09: movaps xmm7, xmmword ptr [rsp + 0xa0]
141a43b11: movaps xmm8, xmmword ptr [rsp + 0x90]
141a43b1a: movaps xmm9, xmmword ptr [rsp + 0x80]
141a43b23: movaps xmm10, xmmword ptr [rsp + 0x70]
141a43b29: movaps xmm11, xmmword ptr [rsp + 0x60]
141a43b2f: movaps xmm12, xmmword ptr [rsp + 0x50]
141a43b35: movaps xmm13, xmmword ptr [rsp + 0x40]
141a43b3b: movaps xmm14, xmmword ptr [rsp + 0x30]
141a43b41: movaps xmm15, xmmword ptr [rsp + 0x20]
141a43b47: add rsp, 0xc8
141a43b4e: pop rbx
141a43b4f: pop rdi
141a43b50: pop rsi
141a43b51: pop r12
141a43b53: pop r13
141a43b55: pop r14
141a43b57: pop r15
141a43b59: pop rbp
141a43b5a: ret 
141a43b5b: nop dword ptr [rax + rax]
141a43b60: mov qword ptr [rsp + 0x10], rdx
141a43b65: push rbp
141a43b66: push r15
141a43b68: push r14
141a43b6a: push r13
141a43b6c: push r12
141a43b6e: push rsi
141a43b6f: push rdi
141a43b70: push rbx
141a43b71: sub rsp, 0xc8
141a43b78: lea rbp, [rdx + 0x80]
141a43b7f: movaps xmmword ptr [rsp + 0x20], xmm15
141a43b85: movaps xmmword ptr [rsp + 0x30], xmm14
141a43b8b: movaps xmmword ptr [rsp + 0x40], xmm13
141a43b91: movaps xmmword ptr [rsp + 0x50], xmm12
141a43b97: movaps xmmword ptr [rsp + 0x60], xmm11
141a43b9d: movaps xmmword ptr [rsp + 0x70], xmm10
141a43ba3: movaps xmmword ptr [rsp + 0x80], xmm9
141a43bac: movaps xmmword ptr [rsp + 0x90], xmm8
141a43bb5: movaps xmmword ptr [rsp + 0xa0], xmm7
141a43bbd: movaps xmmword ptr [rsp + 0xb0], xmm6
141a43bc5: mov rdx, qword ptr [rbp + 0xa0]
141a43bcc: test rdx, rdx
141a43bcf: je 0x141a43be7
141a43bd1: shl rdx, 2
141a43bd5: mov r8d, 4
141a43bdb: mov rcx, qword ptr [rbp + 0x80]
141a43be2: call 0x140613c20
141a43be7: movaps xmm6, xmmword ptr [rsp + 0xb0]
141a43bef: movaps xmm7, xmmword ptr [rsp + 0xa0]
141a43bf7: movaps xmm8, xmmword ptr [rsp + 0x90]
141a43c00: movaps xmm9, xmmword ptr [rsp + 0x80]
141a43c09: movaps xmm10, xmmword ptr [rsp + 0x70]
141a43c0f: movaps xmm11, xmmword ptr [rsp + 0x60]
141a43c15: movaps xmm12, xmmword ptr [rsp + 0x50]
141a43c1b: movaps xmm13, xmmword ptr [rsp + 0x40]
141a43c21: movaps xmm14, xmmword ptr [rsp + 0x30]
141a43c27: movaps xmm15, xmmword ptr [rsp + 0x20]
141a43c2d: add rsp, 0xc8
141a43c34: pop rbx
141a43c35: pop rdi
141a43c36: pop rsi
141a43c37: pop r12
141a43c39: pop r13
141a43c3b: pop r14
141a43c3d: pop r15
141a43c3f: pop rbp
141a43c40: ret 
141a43c41: nop word ptr cs:[rax + rax]
141a43c50: mov qword ptr [rsp + 0x10], rdx
141a43c55: push rbp
141a43c56: push r15
141a43c58: push r14
141a43c5a: push r13
141a43c5c: push r12
141a43c5e: push rsi
141a43c5f: push rdi
141a43c60: push rbx
141a43c61: sub rsp, 0xc8
141a43c68: lea rbp, [rdx + 0x80]
141a43c6f: movaps xmmword ptr [rsp + 0x20], xmm15
141a43c75: movaps xmmword ptr [rsp + 0x30], xmm14
141a43c7b: movaps xmmword ptr [rsp + 0x40], xmm13
141a43c81: movaps xmmword ptr [rsp + 0x50], xmm12
141a43c87: movaps xmmword ptr [rsp + 0x60], xmm11
141a43c8d: movaps xmmword ptr [rsp + 0x70], xmm10
141a43c93: movaps xmmword ptr [rsp + 0x80], xmm9
141a43c9c: movaps xmmword ptr [rsp + 0x90], xmm8
141a43ca5: movaps xmmword ptr [rsp + 0xa0], xmm7
141a43cad: movaps xmmword ptr [rsp + 0xb0], xmm6
141a43cb5: mov rax, qword ptr [rbp + 0xc0]
141a43cbc: test rax, rax
141a43cbf: je 0x141a43cdb
141a43cc1: shl rax, 2
141a43cc5: lea rdx, [rax + rax*2]
141a43cc9: mov r8d, 4
141a43ccf: mov rcx, qword ptr [rbp + 0x230]
141a43cd6: call 0x140613c20
141a43cdb: mov rdx, qword ptr [rbp + 0xd8]
141a43ce2: test rdx, rdx
141a43ce5: je 0x141a43cfd
141a43ce7: mov rcx, qword ptr [rbp + 0xe0]
141a43cee: shl rdx, 2
141a43cf2: mov r8d, 4
141a43cf8: call 0x140613c20
141a43cfd: movaps xmm6, xmmword ptr [rsp + 0xb0]
141a43d05: movaps xmm7, xmmword ptr [rsp + 0xa0]
141a43d0d: movaps xmm8, xmmword ptr [rsp + 0x90]
141a43d16: movaps xmm9, xmmword ptr [rsp + 0x80]
141a43d1f: movaps xmm10, xmmword ptr [rsp + 0x70]
141a43d25: movaps xmm11, xmmword ptr [rsp + 0x60]
141a43d2b: movaps xmm12, xmmword ptr [rsp + 0x50]
141a43d31: movaps xmm13, xmmword ptr [rsp + 0x40]
141a43d37: movaps xmm14, xmmword ptr [rsp + 0x30]
141a43d3d: movaps xmm15, xmmword ptr [rsp + 0x20]
141a43d43: add rsp, 0xc8
141a43d4a: pop rbx
141a43d4b: pop rdi
141a43d4c: pop rsi
141a43d4d: pop r12
141a43d4f: pop r13
141a43d51: pop r14
141a43d53: pop r15
141a43d55: pop rbp
141a43d56: ret 
141a43d57: nop word ptr [rax + rax]
141a43d60: mov qword ptr [rsp + 0x10], rdx
141a43d65: push rbp
141a43d66: push r15
141a43d68: push r14
141a43d6a: push r13
141a43d6c: push r12
141a43d6e: push rsi
141a43d6f: push rdi
141a43d70: push rbx
141a43d71: sub rsp, 0xc8
141a43d78: lea rbp, [rdx + 0x80]
141a43d7f: movaps xmmword ptr [rsp + 0x20], xmm15
141a43d85: movaps xmmword ptr [rsp + 0x30], xmm14
141a43d8b: movaps xmmword ptr [rsp + 0x40], xmm13
141a43d91: movaps xmmword ptr [rsp + 0x50], xmm12
141a43d97: movaps xmmword ptr [rsp + 0x60], xmm11
141a43d9d: movaps xmmword ptr [rsp + 0x70], xmm10
141a43da3: movaps xmmword ptr [rsp + 0x80], xmm9
141a43dac: movaps xmmword ptr [rsp + 0x90], xmm8
141a43db5: movaps xmmword ptr [rsp + 0xa0], xmm7
141a43dbd: movaps xmmword ptr [rsp + 0xb0], xmm6
141a43dc5: mov rax, qword ptr [rbp + 0x230]
141a43dcc: mov rcx, qword ptr [rbp + 0x228]
141a43dd3: mov qword ptr [rcx], rax
141a43dd6: mov rax, qword ptr [rbp + 0x220]
141a43ddd: mov qword ptr [rcx + 8], rax
141a43de1: movaps xmm6, xmmword ptr [rsp + 0xb0]
141a43de9: movaps xmm7, xmmword ptr [rsp + 0xa0]
141a43df1: movaps xmm8, xmmword ptr [rsp + 0x90]
141a43dfa: movaps xmm9, xmmword ptr [rsp + 0x80]
141a43e03: movaps xmm10, xmmword ptr [rsp + 0x70]
141a43e09: movaps xmm11, xmmword ptr [rsp + 0x60]
141a43e0f: movaps xmm12, xmmword ptr [rsp + 0x50]
141a43e15: movaps xmm13, xmmword ptr [rsp + 0x40]
141a43e1b: movaps xmm14, xmmword ptr [rsp + 0x30]
141a43e21: movaps xmm15, xmmword ptr [rsp + 0x20]
141a43e27: add rsp, 0xc8
141a43e2e: pop rbx
141a43e2f: pop rdi
141a43e30: pop rsi
141a43e31: pop r12
141a43e33: pop r13
141a43e35: pop r14
141a43e37: pop r15
141a43e39: pop rbp
141a43e3a: ret 
141a43e3b: nop dword ptr [rax + rax]
141a43e40: mov qword ptr [rsp + 0x10], rdx
141a43e45: push rbp
141a43e46: push r15
141a43e48: push r14
141a43e4a: push r13
141a43e4c: push r12
141a43e4e: push rsi
141a43e4f: push rdi
141a43e50: push rbx
141a43e51: sub rsp, 0xc8
141a43e58: lea rbp, [rdx + 0x80]
141a43e5f: movaps xmmword ptr [rsp + 0x20], xmm15
141a43e65: movaps xmmword ptr [rsp + 0x30], xmm14
141a43e6b: movaps xmmword ptr [rsp + 0x40], xmm13
141a43e71: movaps xmmword ptr [rsp + 0x50], xmm12
141a43e77: movaps xmmword ptr [rsp + 0x60], xmm11
141a43e7d: movaps xmmword ptr [rsp + 0x70], xmm10
141a43e83: movaps xmmword ptr [rsp + 0x80], xmm9
141a43e8c: movaps xmmword ptr [rsp + 0x90], xmm8
141a43e95: movaps xmmword ptr [rsp + 0xa0], xmm7
141a43e9d: movaps xmmword ptr [rsp + 0xb0], xmm6
141a43ea5: lea rcx, [rbp + 0xc0]
141a43eac: call 0x1405d24f0
141a43eb1: movaps xmm6, xmmword ptr [rsp + 0xb0]
141a43eb9: movaps xmm7, xmmword ptr [rsp + 0xa0]
141a43ec1: movaps xmm8, xmmword ptr [rsp + 0x90]
141a43eca: movaps xmm9, xmmword ptr [rsp + 0x80]
141a43ed3: movaps xmm10, xmmword ptr [rsp + 0x70]
141a43ed9: movaps xmm11, xmmword ptr [rsp + 0x60]
141a43edf: movaps xmm12, xmmword ptr [rsp + 0x50]
141a43ee5: movaps xmm13, xmmword ptr [rsp + 0x40]
141a43eeb: movaps xmm14, xmmword ptr [rsp + 0x30]
141a43ef1: movaps xmm15, xmmword ptr [rsp + 0x20]
141a43ef7: add rsp, 0xc8
141a43efe: pop rbx
141a43eff: pop rdi
141a43f00: pop rsi
141a43f01: pop r12
141a43f03: pop r13
141a43f05: pop r14
141a43f07: pop r15
141a43f09: pop rbp
141a43f0a: ret 
141a43f0b: nop dword ptr [rax + rax]
141a43f10: mov qword ptr [rsp + 0x10], rdx
141a43f15: push rbp
141a43f16: push r15
141a43f18: push r14
141a43f1a: push r13
141a43f1c: push r12
141a43f1e: push rsi
141a43f1f: push rdi
141a43f20: push rbx
141a43f21: sub rsp, 0xc8
141a43f28: lea rbp, [rdx + 0x80]
141a43f2f: movaps xmmword ptr [rsp + 0x20], xmm15
141a43f35: movaps xmmword ptr [rsp + 0x30], xmm14
141a43f3b: movaps xmmword ptr [rsp + 0x40], xmm13
141a43f41: movaps xmmword ptr [rsp + 0x50], xmm12
141a43f47: movaps xmmword ptr [rsp + 0x60], xmm11
141a43f4d: movaps xmmword ptr [rsp + 0x70], xmm10
141a43f53: movaps xmmword ptr [rsp + 0x80], xmm9
141a43f5c: movaps xmmword ptr [rsp + 0x90], xmm8
141a43f65: movaps xmmword ptr [rsp + 0xa0], xmm7
141a43f6d: movaps xmmword ptr [rsp + 0xb0], xmm6
141a43f75: lea rcx, [rbp + 0x120]
141a43f7c: call 0x1405d24f0
141a43f81: movaps xmm6, xmmword ptr [rsp + 0xb0]
141a43f89: movaps xmm7, xmmword ptr [rsp + 0xa0]
141a43f91: movaps xmm8, xmmword ptr [rsp + 0x90]
141a43f9a: movaps xmm9, xmmword ptr [rsp + 0x80]
141a43fa3: movaps xmm10, xmmword ptr [rsp + 0x70]
141a43fa9: movaps xmm11, xmmword ptr [rsp + 0x60]
141a43faf: movaps xmm12, xmmword ptr [rsp + 0x50]
141a43fb5: movaps xmm13, xmmword ptr [rsp + 0x40]
141a43fbb: movaps xmm14, xmmword ptr [rsp + 0x30]
141a43fc1: movaps xmm15, xmmword ptr [rsp + 0x20]
141a43fc7: add rsp, 0xc8
141a43fce: pop rbx
141a43fcf: pop rdi
141a43fd0: pop rsi
141a43fd1: pop r12
141a43fd3: pop r13
141a43fd5: pop r14
141a43fd7: pop r15
141a43fd9: pop rbp
141a43fda: ret 
