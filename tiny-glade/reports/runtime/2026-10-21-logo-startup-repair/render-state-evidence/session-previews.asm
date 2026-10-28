0x1408f6c00: push rbp
0x1408f6c01: push r15
0x1408f6c03: push r14
0x1408f6c05: push r13
0x1408f6c07: push r12
0x1408f6c09: push rsi
0x1408f6c0a: push rdi
0x1408f6c0b: push rbx
0x1408f6c0c: mov eax, 0x1158
0x1408f6c11: call 0x14289aa40
0x1408f6c16: sub rsp, rax
0x1408f6c19: lea rbp, [rsp + 0x80]
0x1408f6c21: movaps xmmword ptr [rbp + 0x10c0], xmm13
0x1408f6c29: movaps xmmword ptr [rbp + 0x10b0], xmm12
0x1408f6c31: movaps xmmword ptr [rbp + 0x10a0], xmm11
0x1408f6c39: movaps xmmword ptr [rbp + 0x1090], xmm10
0x1408f6c41: movaps xmmword ptr [rbp + 0x1080], xmm9
0x1408f6c49: movaps xmmword ptr [rbp + 0x1070], xmm8
0x1408f6c51: movaps xmmword ptr [rbp + 0x1060], xmm7
0x1408f6c58: movaps xmmword ptr [rbp + 0x1050], xmm6
0x1408f6c5f: mov qword ptr [rbp + 0x1048], 0xfffffffffffffffe
0x1408f6c6a: mov rdi, r8
0x1408f6c6d: mov rbx, rdx
0x1408f6c70: mov r14, rcx
0x1408f6c73: lea rsi, [rdx + 0x38]
0x1408f6c77: movabs rdx, 0xd38dae617fb79c7a
0x1408f6c81: movabs r8, 0x22aadf5c5dad1eae
0x1408f6c8b: mov rcx, rsi
0x1408f6c8e: call 0x1400dd6f0
0x1408f6c93: test rax, rax
0x1408f6c96: je 0x1408f979e
0x1408f6c9c: cmp qword ptr [rbx + 0x218], rdx
0x1408f6ca3: jbe 0x1408f979e
0x1408f6ca9: mov rax, qword ptr [rbx + 0x210]
0x1408f6cb0: mov rcx, qword ptr [rax + rdx*8]
0x1408f6cb4: test rcx, rcx
0x1408f6cb7: je 0x1408f979e
0x1408f6cbd: mov rax, qword ptr [rbx + 0x1e0]
0x1408f6cc4: not rcx
0x1408f6cc7: lea rcx, [rcx + rcx*2]
0x1408f6ccb: shl rcx, 5
0x1408f6ccf: cmp qword ptr [rax + rcx + 0x20], 0
0x1408f6cd5: je 0x1408f979e
0x1408f6cdb: add rax, rcx
0x1408f6cde: mov rax, qword ptr [rax + 0x10]
0x1408f6ce2: mov qword ptr [rbp + 0x1008], rax
0x1408f6ce9: mov rax, qword ptr [rax]
0x1408f6cec: add rax, -2
0x1408f6cf0: cmp rax, 9
0x1408f6cf4: setb cl
0x1408f6cf7: cmp rax, 4
0x1408f6cfb: setne al
0x1408f6cfe: test cl, al
0x1408f6d00: jne 0x1408f954d
0x1408f6d06: mov rcx, rbx
0x1408f6d09: call 0x1408f5eb0
0x1408f6d0e: test rax, rax
0x1408f6d11: je 0x1408f97c2
0x1408f6d17: lea rdx, [rip + 0x21b21a3]
0x1408f6d1e: mov r8d, 0x1c
0x1408f6d24: mov rcx, rax
0x1408f6d27: call 0x140937aa0
0x1408f6d2c: mov qword ptr [rbp + 0xd98], rdx
0x1408f6d33: test al, 1
0x1408f6d35: je 0x1408f97ce
0x1408f6d3b: mov qword ptr [rbp + 0xd90], rdi
0x1408f6d42: mov rcx, rbx
0x1408f6d45: call 0x1408f5eb0
0x1408f6d4a: test rax, rax
0x1408f6d4d: je 0x1408f97da
0x1408f6d53: lea rdx, [rip + 0x21b21f8]
0x1408f6d5a: mov r8d, 0x25
0x1408f6d60: mov rcx, rax
0x1408f6d63: call 0x140937aa0
0x1408f6d68: test al, 1
0x1408f6d6a: je 0x1408f97e6
0x1408f6d70: mov rdi, rdx
0x1408f6d73: mov rcx, rbx
0x1408f6d76: call 0x1408f5eb0
0x1408f6d7b: test rax, rax
0x1408f6d7e: je 0x1408f97f2
0x1408f6d84: lea rdx, [rip + 0x21b217a]
0x1408f6d8b: mov r8d, 0x1b
0x1408f6d91: mov rcx, rax
0x1408f6d94: call 0x140937aa0
0x1408f6d99: mov qword ptr [rbp + 0xd80], rdx
0x1408f6da0: test al, 1
0x1408f6da2: je 0x1408f97fe
0x1408f6da8: mov rcx, rbx
0x1408f6dab: call 0x1408f5eb0
0x1408f6db0: test rax, rax
0x1408f6db3: je 0x1408f980a
0x1408f6db9: mov qword ptr [rbp + 0xeb0], r14
0x1408f6dc0: mov qword ptr [rbp + 0xea8], rbx
0x1408f6dc7: mov qword ptr [rbp + 0xda0], rdi
0x1408f6dce: lea rdx, [rip + 0x21b21d4]
0x1408f6dd5: mov r8d, 0x25
0x1408f6ddb: mov rcx, rax
0x1408f6dde: call 0x140937aa0
0x1408f6de3: mov qword ptr [rbp + 0xd88], rdx
0x1408f6dea: test al, 1
0x1408f6dec: je 0x1408f9816
0x1408f6df2: mov rax, qword ptr [rbp + 0x1008]
0x1408f6df9: mov r14, qword ptr [rax + 0x30]
0x1408f6dfd: lea rdi, [r14 + 0x10]
0x1408f6e01: mov cl, 1
0x1408f6e03: xor eax, eax
0x1408f6e05: lock cmpxchg byte ptr [r14 + 0x10], cl
0x1408f6e0b: jne 0x1408f95f4
0x1408f6e11: mov rcx, rdi
0x1408f6e14: call 0x140c81c70
0x1408f6e19: mov rbx, qword ptr [r14 + 0x14]
0x1408f6e1d: movups xmm6, xmmword ptr [r14 + 0x1c]
0x1408f6e22: movss xmm8, dword ptr [r14 + 0x2c]
0x1408f6e28: mov rax, qword ptr [r14 + 0x40]
0x1408f6e2c: mov qword ptr [rbp + 0x7b0], rax
0x1408f6e33: movups xmm0, xmmword ptr [r14 + 0x30]
0x1408f6e38: movaps xmmword ptr [rbp + 0x7a0], xmm0
0x1408f6e3f: mov rcx, rdi
0x1408f6e42: call 0x140c81e10
0x1408f6e47: xor ecx, ecx
0x1408f6e49: mov al, 1
0x1408f6e4b: lock cmpxchg byte ptr [r14 + 0x10], cl
0x1408f6e51: jne 0x1408f9607
0x1408f6e57: movabs rdx, 0x4da5153112b09210
0x1408f6e61: movabs r8, 0x5b807edb4e205af1
0x1408f6e6b: mov rcx, rsi
0x1408f6e6e: call 0x1400dd6f0
0x1408f6e73: test rax, rax
0x1408f6e76: mov rdi, qword ptr [rbp + 0xea8]
0x1408f6e7d: mov r14, qword ptr [rbp + 0xeb0]
0x1408f6e84: je 0x1408f97aa
0x1408f6e8a: cmp qword ptr [rdi + 0x218], rdx
0x1408f6e91: jbe 0x1408f97aa
0x1408f6e97: mov rax, qword ptr [rdi + 0x210]
0x1408f6e9e: mov rcx, qword ptr [rax + rdx*8]
0x1408f6ea2: test rcx, rcx
0x1408f6ea5: je 0x1408f97aa
0x1408f6eab: mov rax, qword ptr [rdi + 0x1e0]
0x1408f6eb2: not rcx
0x1408f6eb5: lea rcx, [rcx + rcx*2]
0x1408f6eb9: shl rcx, 5
0x1408f6ebd: cmp qword ptr [rax + rcx + 0x20], 0
0x1408f6ec3: je 0x1408f97aa
0x1408f6ec9: add rax, rcx
0x1408f6ecc: mov rax, qword ptr [rax + 0x10]
0x1408f6ed0: mov qword ptr [rbp + 0xeb8], rax
0x1408f6ed7: movabs rdx, 0xfa1ac26e4587c759
0x1408f6ee1: movabs r8, 0xdb4d89b65fe6f378
0x1408f6eeb: mov rcx, rsi
0x1408f6eee: call 0x1400dd6f0
0x1408f6ef3: test rax, rax
0x1408f6ef6: je 0x1408f97b6
0x1408f6efc: cmp qword ptr [rdi + 0x218], rdx
0x1408f6f03: jbe 0x1408f97b6
0x1408f6f09: mov rax, qword ptr [rdi + 0x210]
0x1408f6f10: mov rcx, qword ptr [rax + rdx*8]
0x1408f6f14: test rcx, rcx
0x1408f6f17: je 0x1408f97b6
0x1408f6f1d: mov rax, qword ptr [rdi + 0x1e0]
0x1408f6f24: not rcx
0x1408f6f27: lea rcx, [rcx + rcx*2]
0x1408f6f2b: shl rcx, 5
0x1408f6f2f: cmp qword ptr [rax + rcx + 0x20], 0
0x1408f6f35: je 0x1408f97b6
0x1408f6f3b: add rax, rcx
0x1408f6f3e: mov rcx, qword ptr [rax + 0x10]
0x1408f6f42: mov qword ptr [rbp + 0x1028], rcx
0x1408f6f49: mov ecx, dword ptr [rdi + 0x5b0]
0x1408f6f4f: mov dword ptr [rax + 0x5c], ecx
0x1408f6f52: mov rax, qword ptr [r14 + 0x190]
0x1408f6f59: mov qword ptr [rbp + 0xda8], rax
0x1408f6f60: mov qword ptr [rbp + 0x5a8], rbx
0x1408f6f67: movups xmmword ptr [rbp + 0x5b0], xmm6
0x1408f6f6e: movss dword ptr [rbp + 0x5c0], xmm8
0x1408f6f77: mov rax, qword ptr [rbp + 0x7b0]
0x1408f6f7e: mov qword ptr [rbp + 0x5d4], rax
0x1408f6f85: movaps xmm0, xmmword ptr [rbp + 0x7a0]
0x1408f6f8c: movups xmmword ptr [rbp + 0x5c4], xmm0
0x1408f6f93: cmp eax, 1
0x1408f6f96: jne 0x1408f9616
0x1408f6f9c: cmp byte ptr [rbp + 0x5d8], 0
0x1408f6fa3: je 0x1408f966c
0x1408f6fa9: mov rax, qword ptr [rbp + 0xeb8]
0x1408f6fb0: mov rcx, qword ptr [rax + 8]
0x1408f6fb4: mov qword ptr [rbp + 0x1020], rcx
0x1408f6fbb: mov rdi, qword ptr [rax + 0x10]
0x1408f6fbf: mov rax, qword ptr [rbp + 0x1028]
0x1408f6fc6: mov rsi, qword ptr [rax + 0x10]
0x1408f6fca: xorps xmm0, xmm0
0x1408f6fcd: cvtsi2ss xmm0, rdi
0x1408f6fd2: xorps xmm7, xmm7
0x1408f6fd5: xorps xmm1, xmm1
0x1408f6fd8: maxss xmm1, dword ptr [rbp + 0x5c8]
0x1408f6fe0: minss xmm0, xmm1
0x1408f6fe4: xorps xmm1, xmm1
0x1408f6fe7: cvtsi2ss xmm1, rsi
0x1408f6fec: movss xmm2, dword ptr [rip + 0x202e87c]
0x1408f6ff4: mulss xmm1, xmm2
0x1408f6ff8: subss xmm0, xmm1
0x1408f6ffc: addss xmm0, xmm2
0x1408f7000: call 0x142923fa0
0x1408f7005: maxss xmm0, xmm7
0x1408f7009: cvttss2si rax, xmm0
0x1408f700e: mov rcx, rax
0x1408f7011: movaps xmm1, xmm0
0x1408f7014: subss xmm1, dword ptr [rip + 0x20630c0]
0x1408f701c: cvttss2si rdx, xmm1
0x1408f7021: sar rcx, 0x3f
0x1408f7025: and rdx, rcx
0x1408f7028: or rdx, rax
0x1408f702b: xor eax, eax
0x1408f702d: ucomiss xmm0, xmm7
0x1408f7030: cmovb rdx, rax
0x1408f7034: ucomiss xmm0, dword ptr [rip + 0x20630a5]
0x1408f703b: mov rcx, 0xffffffffffffffff
0x1408f7042: cmovbe rcx, rdx
0x1408f7046: lea r9, [rcx + rsi]
0x1408f704a: cmp rdi, r9
0x1408f704d: mov rdx, r9
0x1408f7050: cmovb rdx, rdi
0x1408f7054: mov r10, rdx
0x1408f7057: mov qword ptr [rbp + 0x1010], rcx
0x1408f705e: sub r10, rcx
0x1408f7061: cmovb r10, rax
0x1408f7065: cmp rdi, 6
0x1408f7069: mov eax, 6
0x1408f706e: cmovb rax, rdi
0x1408f7072: cmp r10, rax
0x1408f7075: ja 0x1408f96ad
0x1408f707b: mov qword ptr [rbp + 0x7c0], 0
0x1408f7086: mov qword ptr [rbp + 0x7d0], 0
0x1408f7091: mov qword ptr [rbp + 0x7e0], 0
0x1408f709c: mov qword ptr [rbp + 0x7f0], 0
0x1408f70a7: mov qword ptr [rbp + 0x800], 0
0x1408f70b2: mov qword ptr [rbp + 0x810], 0
0x1408f70bd: test rsi, rsi
0x1408f70c0: je 0x1408f70ef
0x1408f70c2: mov rax, qword ptr [rbp + 0x1028]
0x1408f70c9: mov rax, qword ptr [rax + 8]
0x1408f70cd: imul rcx, rsi, 0x68
0x1408f70d1: xor r8d, r8d
0x1408f70d4: nop word ptr cs:[rax + rax]
0x1408f70e0: mov byte ptr [rax + r8 + 0x60], 0
0x1408f70e6: add r8, 0x68
0x1408f70ea: cmp rcx, r8
0x1408f70ed: jne 0x1408f70e0
0x1408f70ef: mov rax, rdx
0x1408f70f2: sub rax, qword ptr [rbp + 0x1010]
0x1408f70f9: jb 0x1408f96c5
0x1408f70ff: mov qword ptr [rbp + 0xed0], r10
0x1408f7106: mov qword ptr [rbp + 0xec8], r9
0x1408f710d: mov qword ptr [rbp + 0x1000], rdi
0x1408f7114: mov qword ptr [rbp + 0xec0], rdx
0x1408f711b: lea rcx, [rax + rax*8]
0x1408f711f: lea rcx, [rcx + rcx*2]
0x1408f7123: add rcx, rax
0x1408f7126: mov rax, rcx
0x1408f7129: shr rax, 2
0x1408f712d: movabs rdx, 0x6db6db6db6db6db7
0x1408f7137: imul rdx, rax
0x1408f713b: cmp rdx, 6
0x1408f713f: mov eax, 6
0x1408f7144: cmovb rax, rdx
0x1408f7148: mov qword ptr [rbp + 0x1030], rax
0x1408f714f: test rcx, rcx
0x1408f7152: je 0x1408f7624
0x1408f7158: mov rcx, qword ptr [rbp + 0x1010]
0x1408f715f: lea rax, [rcx + rcx*8]
0x1408f7163: lea rax, [rax + rax*2]
0x1408f7167: add rax, rcx
0x1408f716a: add qword ptr [rbp + 0x1020], rax
0x1408f7171: xor edi, edi
0x1408f7173: xor r15d, r15d
0x1408f7176: jmp 0x1408f7190
0x1408f7178: nop dword ptr [rax + rax]
0x1408f7180: inc r15
0x1408f7183: cmp r15, qword ptr [rbp + 0x1030]
0x1408f718a: je 0x1408f74ce
0x1408f7190: lea rax, [r15 + r15*8]
0x1408f7194: lea rdx, [rax + rax*2]
0x1408f7198: add rdx, r15
0x1408f719b: add rdx, qword ptr [rbp + 0x1020]
0x1408f71a2: lea rsi, [rbp + 0x1c0]
0x1408f71a9: mov rcx, rsi
0x1408f71ac: call 0x140b46000
0x1408f71b1: mov rcx, rsi
0x1408f71b4: call 0x140d76ed0
0x1408f71b9: mov r8, rdx
0x1408f71bc: mov qword ptr [rsp + 0x20], 0xe
0x1408f71c5: lea rcx, [rbp + 0xed8]
0x1408f71cc: mov rdx, rax
0x1408f71cf: lea r9, [rip + 0x21f832d]
0x1408f71d6: call 0x140d62810
0x1408f71db: mov rdx, qword ptr [rbp + 0x1c0]
0x1408f71e2: test rdx, rdx
0x1408f71e5: je 0x1408f71f9
0x1408f71e7: mov rcx, qword ptr [rbp + 0x1c8]
0x1408f71ee: mov r8d, 1
0x1408f71f4: call 0x140613c20
0x1408f71f9: mov rax, qword ptr [rbp + 0x1028]
0x1408f7200: mov rax, qword ptr [rax + 0x10]
0x1408f7204: test rax, rax
0x1408f7207: je 0x1408f74a7
0x1408f720d: mov rcx, r15
0x1408f7210: shl rcx, 4
0x1408f7214: add rcx, rbp
0x1408f7217: add rcx, 0x7c0
0x1408f721e: mov qword ptr [rbp + 0x1018], rcx
0x1408f7225: mov rcx, qword ptr [rbp + 0x1028]
0x1408f722c: mov r14, qword ptr [rcx + 8]
0x1408f7230: imul rsi, rax, 0x68
0x1408f7234: xor r12d, r12d
0x1408f7237: jmp 0x1408f7251
0x1408f7239: nop dword ptr [rax]
0x1408f7240: add r14, 0x68
0x1408f7244: inc r12
0x1408f7247: add rsi, -0x68
0x1408f724b: je 0x1408f74a7
0x1408f7251: cmp rdi, qword ptr [r14]
0x1408f7254: jo 0x1408f7240
0x1408f7256: mov rcx, r14
0x1408f7259: call 0x140d76ed0
0x1408f725e: mov r8, rdx
0x1408f7261: lea rcx, [rbp + 0xbb0]
0x1408f7268: mov rdx, rax
0x1408f726b: call 0x140d62c00
0x1408f7270: lea rcx, [rbp + 0xed8]
0x1408f7277: call 0x140d76ed0
0x1408f727c: mov r8, rdx
0x1408f727f: lea rcx, [rbp + 0xdb0]
0x1408f7286: mov rdx, rax
0x1408f7289: call 0x140d62c00
0x1408f728e: mov rdi, qword ptr [rbp + 0xbb8]
0x1408f7295: mov r13, qword ptr [rbp + 0xdb8]
0x1408f729c: cmp rdi, r13
0x1408f729f: jne 0x1408f72f0
0x1408f72a1: movzx ebx, byte ptr [rbp + 0xbc0]
0x1408f72a8: movzx eax, byte ptr [rbp + 0xbe8]
0x1408f72af: cmp al, byte ptr [rbp + 0xde8]
0x1408f72b5: jne 0x1408f72f7
0x1408f72b7: cmp byte ptr [rbp + 0xbe9], 2
0x1408f72be: jne 0x1408f72f7
0x1408f72c0: cmp byte ptr [rbp + 0xde9], 2
0x1408f72c7: jne 0x1408f72f7
0x1408f72c9: cmp bl, 6
0x1408f72cc: jne 0x1408f7436
0x1408f72d2: cmp byte ptr [rbp + 0xdc0], 2
0x1408f72d9: ja 0x1408f7446
0x1408f72df: mov cl, 6
0x1408f72e1: mov rax, qword ptr [rbp + 0xbb0]
0x1408f72e8: jmp 0x1408f732e
0x1408f72ea: nop word ptr [rax + rax]
0x1408f72f0: movzx ebx, byte ptr [rbp + 0xbc0]
0x1408f72f7: mov rax, qword ptr [rbp + 0xbb0]
0x1408f72fe: mov cl, 6
0x1408f7300: cmp bl, 6
0x1408f7303: je 0x1408f732e
0x1408f7305: lea rdx, [rbp + 0xbc1]
0x1408f730c: mov rcx, qword ptr [rdx + 0x1f]
0x1408f7310: mov qword ptr [rbp + 0xccf], rcx
0x1408f7317: movups xmm0, xmmword ptr [rdx]
0x1408f731a: movups xmm1, xmmword ptr [rdx + 0x10]
0x1408f731e: movaps xmmword ptr [rbp + 0xcc0], xmm1
0x1408f7325: movaps xmmword ptr [rbp + 0xcb0], xmm0
0x1408f732c: mov ecx, ebx
0x1408f732e: movzx edx, byte ptr [rbp + 0xbea]
0x1408f7335: movzx r8d, word ptr [rbp + 0xbe8]
0x1408f733d: mov qword ptr [rbp + 0xf00], rax
0x1408f7344: mov qword ptr [rbp + 0xf08], rdi
0x1408f734b: mov byte ptr [rbp + 0xf10], cl
0x1408f7351: mov rax, qword ptr [rbp + 0xccf]
0x1408f7358: lea rcx, [rbp + 0xf11]
0x1408f735f: mov qword ptr [rcx + 0x1f], rax
0x1408f7363: movaps xmm0, xmmword ptr [rbp + 0xcb0]
0x1408f736a: movaps xmm1, xmmword ptr [rbp + 0xcc0]
0x1408f7371: movups xmmword ptr [rcx + 0x10], xmm1
0x1408f7375: movups xmmword ptr [rcx], xmm0
0x1408f7378: mov word ptr [rbp + 0xf38], r8w
0x1408f7380: mov byte ptr [rbp + 0xf3a], dl
0x1408f7386: movzx eax, byte ptr [rbp + 0xdc0]
0x1408f738d: cmp al, 6
0x1408f738f: je 0x1408f73b8
0x1408f7391: lea rdx, [rbp + 0xdc1]
0x1408f7398: mov rcx, qword ptr [rdx + 0x1f]
0x1408f739c: mov qword ptr [rbp + 0x6bf], rcx
0x1408f73a3: movups xmm0, xmmword ptr [rdx]
0x1408f73a6: movups xmm1, xmmword ptr [rdx + 0x10]
0x1408f73aa: movaps xmmword ptr [rbp + 0x6b0], xmm1
0x1408f73b1: movaps xmmword ptr [rbp + 0x6a0], xmm0
0x1408f73b8: mov rcx, qword ptr [rbp + 0xdb0]
0x1408f73bf: movzx edx, byte ptr [rbp + 0xdea]
0x1408f73c6: movzx r8d, word ptr [rbp + 0xde8]
0x1408f73ce: mov qword ptr [rbp + 0x1c0], rcx
0x1408f73d5: mov qword ptr [rbp + 0x1c8], r13
0x1408f73dc: mov byte ptr [rbp + 0x1d0], al
0x1408f73e2: mov rax, qword ptr [rbp + 0x6bf]
0x1408f73e9: lea rcx, [rbp + 0x1d1]
0x1408f73f0: mov qword ptr [rcx + 0x1f], rax
0x1408f73f4: movaps xmm0, xmmword ptr [rbp + 0x6a0]
0x1408f73fb: movaps xmm1, xmmword ptr [rbp + 0x6b0]
0x1408f7402: movups xmmword ptr [rcx + 0x10], xmm1
0x1408f7406: movups xmmword ptr [rcx], xmm0
0x1408f7409: mov word ptr [rbp + 0x1f8], r8w
0x1408f7411: mov byte ptr [rbp + 0x1fa], dl
0x1408f7417: lea rcx, [rbp + 0xf00]
0x1408f741e: lea rdx, [rbp + 0x1c0]
0x1408f7425: call 0x1416f6360
0x1408f742a: xor edi, edi
0x1408f742c: test al, al
0x1408f742e: je 0x1408f7240
0x1408f7434: jmp 0x1408f7490
0x1408f7436: movzx eax, byte ptr [rbp + 0xdc0]
0x1408f743d: cmp al, 6
0x1408f743f: jne 0x1408f7466
0x1408f7441: cmp bl, 3
0x1408f7444: jb 0x1408f7475
0x1408f7446: mov rcx, qword ptr [rbp + 0xbb0]
0x1408f744d: mov rdx, qword ptr [rbp + 0xdb0]
0x1408f7454: mov r8, rdi
0x1408f7457: call 0x142923bf0
0x1408f745c: test eax, eax
0x1408f745e: jne 0x1408f72f7
0x1408f7464: jmp 0x1408f7481
0x1408f7466: cmp bl, 3
0x1408f7469: setb cl
0x1408f746c: cmp al, 3
0x1408f746e: setb al
0x1408f7471: xor al, cl
0x1408f7473: je 0x1408f7446
0x1408f7475: mov rax, qword ptr [rbp + 0xbb0]
0x1408f747c: jmp 0x1408f7305
0x1408f7481: xor edi, edi
0x1408f7483: nop word ptr cs:[rax + rax]
0x1408f7490: mov byte ptr [r14 + 0x60], 1
0x1408f7495: mov rax, qword ptr [rbp + 0x1018]
0x1408f749c: mov qword ptr [rax], 1
0x1408f74a3: mov qword ptr [rax + 8], r12
0x1408f74a7: mov rdx, qword ptr [rbp + 0xed8]
0x1408f74ae: test rdx, rdx
0x1408f74b1: je 0x1408f7180
0x1408f74b7: mov rcx, qword ptr [rbp + 0xee0]
0x1408f74be: mov r8d, 1
0x1408f74c4: call 0x140613c20
0x1408f74c9: jmp 0x1408f7180
0x1408f74ce: xor r14d, r14d
0x1408f74d1: lea rsi, [rbp + 0x1c0]
0x1408f74d8: jmp 0x1408f7521
0x1408f74da: nop word ptr [rax + rax]
0x1408f74e0: movups xmm0, xmmword ptr [rbp + 0xf00]
0x1408f74e7: movups xmm1, xmmword ptr [rbp + 0xf10]
0x1408f74ee: movups xmmword ptr [rbx + 0x10], xmm1
0x1408f74f2: movups xmmword ptr [rbx], xmm0
0x1408f74f5: movabs rax, 0x19db1ded53e8000
0x1408f74ff: mov qword ptr [r13 + r15 + 0x20], rax
0x1408f7504: mov qword ptr [r12], 1
0x1408f750c: mov qword ptr [r12 + 8], rdi
0x1408f7511: inc r14
0x1408f7514: cmp r14, qword ptr [rbp + 0x1030]
0x1408f751b: je 0x1408f7624
0x1408f7521: mov rax, r14
0x1408f7524: shl rax, 4
0x1408f7528: cmp qword ptr [rbp + rax + 0x7c0], 0
0x1408f7531: jne 0x1408f7511
0x1408f7533: mov rcx, qword ptr [rbp + 0x1028]
0x1408f753a: mov rcx, qword ptr [rcx + 0x10]
0x1408f753e: test rcx, rcx
0x1408f7541: je 0x1408f96d8
0x1408f7547: lea rdx, [r14 + r14*8]
0x1408f754b: lea rdx, [rdx + rdx*2]
0x1408f754f: add rdx, r14
0x1408f7552: add rdx, qword ptr [rbp + 0x1020]
0x1408f7559: lea r12, [rax + rbp]
0x1408f755d: add r12, 0x7c0
0x1408f7564: mov rax, qword ptr [rbp + 0x1028]
0x1408f756b: mov r13, qword ptr [rax + 8]
0x1408f756f: imul rax, rcx, 0x68
0x1408f7573: xor r15d, r15d
0x1408f7576: xor edi, edi
0x1408f7578: nop dword ptr [rax + rax]
0x1408f7580: cmp byte ptr [r13 + r15 + 0x60], 0
0x1408f7586: je 0x1408f75a0
0x1408f7588: inc rdi
0x1408f758b: add r15, 0x68
0x1408f758f: cmp rax, r15
0x1408f7592: jne 0x1408f7580
0x1408f7594: jmp 0x1408f96d8
0x1408f7599: nop dword ptr [rax]
0x1408f75a0: mov byte ptr [r13 + r15 + 0x60], 1
0x1408f75a6: mov rcx, rsi
0x1408f75a9: call 0x140b46000
0x1408f75ae: mov rcx, rsi
0x1408f75b1: call 0x140d76ed0
0x1408f75b6: mov r8, rdx
0x1408f75b9: mov qword ptr [rsp + 0x20], 0xe
0x1408f75c2: lea rcx, [rbp + 0xf00]
0x1408f75c9: mov rdx, rax
0x1408f75cc: lea r9, [rip + 0x21f7f30]
0x1408f75d3: call 0x140d62810
0x1408f75d8: mov rdx, qword ptr [rbp + 0x1c0]
0x1408f75df: test rdx, rdx
0x1408f75e2: je 0x1408f75f6
0x1408f75e4: mov rcx, qword ptr [rbp + 0x1c8]
0x1408f75eb: mov r8d, 1
0x1408f75f1: call 0x140613c20
0x1408f75f6: lea rbx, [r15 + r13]
0x1408f75fa: mov rdx, qword ptr [rbx]
0x1408f75fd: mov rax, rdx
0x1408f7600: neg rax
0x1408f7603: jo 0x1408f74e0
0x1408f7609: jae 0x1408f74e0
0x1408f760f: mov rcx, qword ptr [r13 + r15 + 8]
0x1408f7614: mov r8d, 1
0x1408f761a: call 0x140613c20
0x1408f761f: jmp 0x1408f74e0
0x1408f7624: mov rax, 0xffffffffffffffff
0x1408f762b: mov r11, qword ptr [rbp + 0xec0]
0x1408f7632: cmp r11, qword ptr [rbp + 0x1010]
0x1408f7639: jbe 0x1408f767a
0x1408f763b: test byte ptr [rbp + 0x7c0], 1
0x1408f7642: mov rsi, qword ptr [rbp + 0x1000]
0x1408f7649: mov rdi, qword ptr [rbp + 0xec8]
0x1408f7650: mov rbx, qword ptr [rbp + 0xed0]
0x1408f7657: je 0x1408f984c
0x1408f765d: mov rcx, qword ptr [rbp + 0x7c8]
0x1408f7664: cmp rbx, 1
0x1408f7668: jne 0x1408f76b4
0x1408f766a: mov rdx, 0xffffffffffffffff
0x1408f7671: mov r8, 0xffffffffffffffff
0x1408f7678: jmp 0x1408f76f5
0x1408f767a: mov rcx, 0xffffffffffffffff
0x1408f7681: mov rdx, 0xffffffffffffffff
0x1408f7688: mov r8, 0xffffffffffffffff
0x1408f768f: mov r9, 0xffffffffffffffff
0x1408f7696: mov r10, 0xffffffffffffffff
0x1408f769d: mov rsi, qword ptr [rbp + 0x1000]
0x1408f76a4: mov rdi, qword ptr [rbp + 0xec8]
0x1408f76ab: mov rbx, qword ptr [rbp + 0xed0]
0x1408f76b2: jmp 0x1408f7703
0x1408f76b4: cmp dword ptr [rbp + 0x7d0], 1
0x1408f76bb: jne 0x1408f984c
0x1408f76c1: mov rdx, qword ptr [rbp + 0x7d8]
0x1408f76c8: cmp rbx, 2
0x1408f76cc: jne 0x1408f76d7
0x1408f76ce: mov r8, 0xffffffffffffffff
0x1408f76d5: jmp 0x1408f76f5
0x1408f76d7: cmp dword ptr [rbp + 0x7e0], 1
0x1408f76de: jne 0x1408f984c
0x1408f76e4: mov r8, qword ptr [rbp + 0x7e8]
0x1408f76eb: cmp rbx, 3
0x1408f76ef: jne 0x1408f959f
0x1408f76f5: mov r9, 0xffffffffffffffff
0x1408f76fc: mov r10, 0xffffffffffffffff
0x1408f7703: mov qword ptr [rbp + 0x6a0], rcx
0x1408f770a: mov qword ptr [rbp + 0x6a8], rdx
0x1408f7711: mov qword ptr [rbp + 0x6b0], r8
0x1408f7718: mov qword ptr [rbp + 0x6b8], r9
0x1408f771f: mov qword ptr [rbp + 0x6c0], r10
0x1408f7726: mov qword ptr [rbp + 0x6c8], rax
0x1408f772d: lea r9, [rbx*8 + 0x6a0]
0x1408f7735: add r9, rbp
0x1408f7738: mov rax, qword ptr [rbp + 0x1010]
0x1408f773f: cmp rax, rsi
0x1408f7742: cmovb rsi, rax
0x1408f7746: mov rax, qword ptr [rbp + 0x1028]
0x1408f774d: mov rax, qword ptr [rax + 0x10]
0x1408f7751: cmp rsi, rdi
0x1408f7754: cmovae rsi, rdi
0x1408f7758: lea rcx, [r11*8]
0x1408f7760: shl rsi, 3
0x1408f7764: sub rcx, rsi
0x1408f7767: xor edx, edx
0x1408f7769: nop dword ptr [rax]
0x1408f7770: cmp rcx, rdx
0x1408f7773: je 0x1408f779e
0x1408f7775: lea r8, [rdx + 8]
0x1408f7779: cmp qword ptr [rbp + rdx + 0x6a0], rax
0x1408f7781: mov rdx, r8
0x1408f7784: jb 0x1408f7770
0x1408f7786: lea rcx, [rip + 0x21b1588]
0x1408f778d: lea r8, [rip + 0x21b15b4]
0x1408f7794: mov edx, 0x2f
0x1408f7799: call 0x1428d9430
0x1408f779e: mov qword ptr [rbp + 0x1000], r9
0x1408f77a5: mov rcx, qword ptr [rbp + 0x1028]
0x1408f77ac: mov r12, qword ptr [rcx + 8]
0x1408f77b0: imul rsi, rax, 0x68
0x1408f77b4: add rsi, r12
0x1408f77b7: lea r15, [rbp + 0x7c0]
0x1408f77be: lea r13, [rbp + 0xf00]
0x1408f77c5: lea rdi, [rbp + 0xcb0]
0x1408f77cc: movaps xmm7, xmmword ptr [rip + 0x21b0bfd]
0x1408f77d3: movsd xmm9, qword ptr [rip + 0x21b0c04]
0x1408f77dc: xorps xmm10, xmm10
0x1408f77e0: jmp 0x1408f77fc
0x1408f77e2: nop word ptr cs:[rax + rax]
0x1408f77f0: mov byte ptr [rbx + 0x60], 0
0x1408f77f4: mov rcx, rdi
0x1408f77f7: call 0x140d4cc30
0x1408f77fc: cmp r12, rsi
0x1408f77ff: je 0x1408f8336
0x1408f7805: mov rax, r12
0x1408f7808: add r12, 0x68
0x1408f780c: cmp byte ptr [rax + 0x60], 1
0x1408f7810: jne 0x1408f77fc
0x1408f7812: mov rcx, rax
0x1408f7815: xor eax, eax
0x1408f7817: cmp rax, qword ptr [rcx]
0x1408f781a: jo 0x1408f9720
0x1408f7820: mov qword ptr [rbp + 0x1030], rcx
0x1408f7827: call 0x140d76ed0
0x1408f782c: mov r8, rdx
0x1408f782f: mov rcx, r15
0x1408f7832: mov rdx, rax
0x1408f7835: call 0x140d6a710
0x1408f783a: mov rax, qword ptr [rbp + 0x7c0]
0x1408f7841: mov rbx, qword ptr [rbp + 0x7c8]
0x1408f7848: cmp rax, 2
0x1408f784c: jne 0x1408f78d0
0x1408f7852: mov qword ptr [rbp + 0xbb0], rbx
0x1408f7859: mov byte ptr [rbp + 0x1045], 1
0x1408f7860: mov rcx, r15
0x1408f7863: call 0x140d56700
0x1408f7868: movups xmm0, xmmword ptr [rbp + 0x7c0]
0x1408f786f: movups xmm1, xmmword ptr [rbp + 0x7d0]
0x1408f7876: movups xmm2, xmmword ptr [rbp + 0x7e0]
0x1408f787d: movaps xmmword ptr [rbp + 0xf20], xmm2
0x1408f7884: movaps xmmword ptr [rbp + 0xf10], xmm1
0x1408f788b: movaps xmmword ptr [rbp + 0xf00], xmm0
0x1408f7892: lea rax, [rip + 0x21b1567]
0x1408f7899: mov qword ptr [rbp + 0x7c0], rax
0x1408f78a0: mov qword ptr [rbp + 0x7c8], 0x10
0x1408f78ab: mov qword ptr [rbp + 0x7d0], rbx
0x1408f78b2: mov byte ptr [rbp + 0x1045], 0
0x1408f78b9: mov rcx, r15
0x1408f78bc: mov rdx, r13
0x1408f78bf: call 0x1428a8e80
0x1408f78c4: jmp 0x1408f7bc0
0x1408f78c9: nop dword ptr [rax]
0x1408f78d0: lea r8, [rbp + 0x7d0]
0x1408f78d7: mov rcx, qword ptr [r8 + 0x40]
0x1408f78db: lea rdx, [rbp + 0x1d0]
0x1408f78e2: mov qword ptr [rdx + 0x40], rcx
0x1408f78e6: movups xmm0, xmmword ptr [r8]
0x1408f78ea: movups xmm1, xmmword ptr [r8 + 0x10]
0x1408f78ef: movups xmm2, xmmword ptr [r8 + 0x20]
0x1408f78f4: movups xmm3, xmmword ptr [r8 + 0x30]
0x1408f78f9: movups xmmword ptr [rdx + 0x30], xmm3
0x1408f78fd: movups xmmword ptr [rdx + 0x20], xmm2
0x1408f7901: movups xmmword ptr [rdx + 0x10], xmm1
0x1408f7905: movups xmmword ptr [rdx], xmm0
0x1408f7908: mov qword ptr [rbp + 0x1c0], rax
0x1408f790f: mov qword ptr [rbp + 0x1c8], rbx
0x1408f7916: lea rcx, [rbp + 0xdb0]
0x1408f791d: lea rdx, [rbp + 0x1c0]
0x1408f7924: call 0x140d59e60
0x1408f7929: cmp dword ptr [rbp + 0xdb0], 1
0x1408f7930: jne 0x1408f79b0
0x1408f7932: mov rbx, qword ptr [rbp + 0xdb8]
0x1408f7939: mov qword ptr [rbp + 0xbb0], rbx
0x1408f7940: mov byte ptr [rbp + 0x1044], 1
0x1408f7947: mov rcx, r15
0x1408f794a: call 0x140d56700
0x1408f794f: movups xmm0, xmmword ptr [rbp + 0x7c0]
0x1408f7956: movups xmm1, xmmword ptr [rbp + 0x7d0]
0x1408f795d: movups xmm2, xmmword ptr [rbp + 0x7e0]
0x1408f7964: movaps xmmword ptr [rbp + 0xf20], xmm2
0x1408f796b: movaps xmmword ptr [rbp + 0xf10], xmm1
0x1408f7972: movaps xmmword ptr [rbp + 0xf00], xmm0
0x1408f7979: lea rax, [rip + 0x21b1490]
0x1408f7980: mov qword ptr [rbp + 0x7c0], rax
0x1408f7987: mov qword ptr [rbp + 0x7c8], 0x1a
0x1408f7992: mov qword ptr [rbp + 0x7d0], rbx
0x1408f7999: mov byte ptr [rbp + 0x1044], 0
0x1408f79a0: mov rcx, r15
0x1408f79a3: mov rdx, r13
0x1408f79a6: call 0x1428a8e80
0x1408f79ab: jmp 0x1408f7bc0
0x1408f79b0: mov ebx, dword ptr [rbp + 0xdb4]
0x1408f79b6: mov r14d, dword ptr [rbp + 0xdb8]
0x1408f79bd: mov rax, r14
0x1408f79c0: shl rax, 0x20
0x1408f79c4: or rax, rbx
0x1408f79c7: mov rcx, qword ptr [rbp + 0x1030]
0x1408f79ce: cmp qword ptr [rcx + 0x20], rax
0x1408f79d2: jge 0x1408f77fc
0x1408f79d8: call 0x140d76ed0
0x1408f79dd: mov r8, rdx
0x1408f79e0: lea rcx, [rbp + 0x1c0]
0x1408f79e7: mov rdx, rax
0x1408f79ea: call 0x140ba3a10
0x1408f79ef: mov rcx, qword ptr [rbp + 0x1c0]
0x1408f79f6: mov rax, qword ptr [rbp + 0x1c8]
0x1408f79fd: cmp rcx, 2
0x1408f7a01: jne 0x1408f7a20
0x1408f7a03: movabs rcx, 0x8000000000000009
0x1408f7a0d: mov qword ptr [rbp + 0xf00], rcx
0x1408f7a14: mov qword ptr [rbp + 0xf08], rax
0x1408f7a1b: jmp 0x1408f7ad1
0x1408f7a20: lea r8, [rbp + 0x1d0]
0x1408f7a27: mov rdx, qword ptr [r8 + 0x40]
0x1408f7a2b: lea r9, [rbp + 0x7d0]
0x1408f7a32: mov qword ptr [r9 + 0x40], rdx
0x1408f7a36: movups xmm0, xmmword ptr [r8]
0x1408f7a3a: movups xmm1, xmmword ptr [r8 + 0x10]
0x1408f7a3f: movups xmm2, xmmword ptr [r8 + 0x20]
0x1408f7a44: movups xmm3, xmmword ptr [r8 + 0x30]
0x1408f7a49: movups xmmword ptr [r9 + 0x30], xmm3
0x1408f7a4e: movups xmmword ptr [r9 + 0x20], xmm2
0x1408f7a53: movups xmmword ptr [r9 + 0x10], xmm1
0x1408f7a58: movups xmmword ptr [r9], xmm0
0x1408f7a5c: mov qword ptr [rbp + 0x7c0], rcx
0x1408f7a63: mov qword ptr [rbp + 0x7c8], rax
0x1408f7a6a: mov rcx, r13
0x1408f7a6d: mov rdx, r15
0x1408f7a70: call 0x14091bbb0
0x1408f7a75: movabs rax, 0x800000000000000a
0x1408f7a7f: cmp qword ptr [rbp + 0xf00], rax
0x1408f7a86: jne 0x1408f7ad1
0x1408f7a88: mov rcx, qword ptr [rbp + 0xf08]
0x1408f7a8f: mov rax, qword ptr [rbp + 0xf10]
0x1408f7a96: lea rdx, [rbp + 0xf18]
0x1408f7a9d: movups xmm0, xmmword ptr [rdx]
0x1408f7aa0: movaps xmmword ptr [rbp + 0xbb0], xmm0
0x1408f7aa7: mov rdx, qword ptr [rdx + 0x10]
0x1408f7aab: mov qword ptr [rbp + 0xbc0], rdx
0x1408f7ab2: cmp rcx, 0xa
0x1408f7ab6: jne 0x1408f7fe2
0x1408f7abc: test rax, rax
0x1408f7abf: mov rcx, qword ptr [rbp + 0x1030]
0x1408f7ac6: jne 0x1408f7bc7
0x1408f7acc: jmp 0x1408f77fc
0x1408f7ad1: mov rax, qword ptr [rbp + 0xf00]
0x1408f7ad8: mov rcx, qword ptr [rbp + 0xf08]
0x1408f7adf: movups xmm0, xmmword ptr [rbp + 0xf10]
0x1408f7ae6: movups xmm1, xmmword ptr [rbp + 0xf20]
0x1408f7aed: movups xmm2, xmmword ptr [rbp + 0xf30]
0x1408f7af4: movaps xmmword ptr [rbp + 0x1f0], xmm2
0x1408f7afb: movaps xmmword ptr [rbp + 0x1e0], xmm1
0x1408f7b02: movaps xmmword ptr [rbp + 0x1d0], xmm0
0x1408f7b09: mov qword ptr [rbp + 0x1c0], rax
0x1408f7b10: mov qword ptr [rbp + 0x1c8], rcx
0x1408f7b17: mov byte ptr [rbp + 0x1043], 1
0x1408f7b1e: mov rcx, r15
0x1408f7b21: call 0x140d56700
0x1408f7b26: movups xmm0, xmmword ptr [rbp + 0x7c0]
0x1408f7b2d: movups xmm1, xmmword ptr [rbp + 0x7d0]
0x1408f7b34: movups xmm2, xmmword ptr [rbp + 0x7e0]
0x1408f7b3b: movaps xmmword ptr [rbp + 0xdd0], xmm2
0x1408f7b42: movaps xmmword ptr [rbp + 0xdc0], xmm1
0x1408f7b49: movaps xmmword ptr [rbp + 0xdb0], xmm0
0x1408f7b50: movaps xmm0, xmmword ptr [rbp + 0x1c0]
0x1408f7b57: movaps xmm1, xmmword ptr [rbp + 0x1d0]
0x1408f7b5e: movaps xmm2, xmmword ptr [rbp + 0x1e0]
0x1408f7b65: movaps xmm3, xmmword ptr [rbp + 0x1f0]
0x1408f7b6c: lea rax, [rbp + 0x7d0]
0x1408f7b73: movups xmmword ptr [rax + 0x30], xmm3
0x1408f7b77: movups xmmword ptr [rax + 0x20], xmm2
0x1408f7b7b: movups xmmword ptr [rax + 0x10], xmm1
0x1408f7b7f: movups xmmword ptr [rax], xmm0
0x1408f7b82: lea rax, [rip + 0x21b12a1]
0x1408f7b89: mov qword ptr [rbp + 0x7c0], rax
0x1408f7b90: mov qword ptr [rbp + 0x7c8], 0x11
0x1408f7b9b: mov byte ptr [rbp + 0x1043], 0
0x1408f7ba2: mov rcx, r15
0x1408f7ba5: lea rdx, [rbp + 0xdb0]
0x1408f7bac: call 0x1428a8cb0
0x1408f7bb1: nop word ptr cs:[rax + rax]
0x1408f7bc0: mov rcx, qword ptr [rbp + 0x1030]
0x1408f7bc7: mov qword ptr [rbp + 0xcb0], rax
0x1408f7bce: call 0x140d76ed0
0x1408f7bd3: mov r8, rdx
0x1408f7bd6: mov r9d, 3
0x1408f7bdc: mov rcx, r13
0x1408f7bdf: mov rdx, rax
0x1408f7be2: call 0x1416f6ac0
0x1408f7be7: mov qword ptr [rbp + 0xdb0], r13
0x1408f7bee: lea rax, [rip + 0x469eeb]
0x1408f7bf5: mov qword ptr [rbp + 0xdb8], rax
0x1408f7bfc: mov qword ptr [rbp + 0xdc0], rdi
0x1408f7c03: lea rax, [rip + 0x455006]
0x1408f7c0a: mov qword ptr [rbp + 0xdc8], rax
0x1408f7c11: mov qword ptr [rbp + 0x7c0], 2
0x1408f7c1c: mov qword ptr [rbp + 0x7d0], 2
0x1408f7c27: mov qword ptr [rbp + 0x7e0], 0
0x1408f7c32: mov qword ptr [rbp + 0x7e8], 0x20
0x1408f7c3d: mov byte ptr [rbp + 0x7f0], 3
0x1408f7c44: mov qword ptr [rbp + 0x7f8], 2
0x1408f7c4f: mov qword ptr [rbp + 0x808], 2
0x1408f7c5a: mov qword ptr [rbp + 0x818], 1
0x1408f7c65: movabs rax, 0x400000020
0x1408f7c6f: mov qword ptr [rbp + 0x820], rax
0x1408f7c76: mov byte ptr [rbp + 0x828], 3
0x1408f7c7d: lea rax, [rip + 0x21b0fe4]
0x1408f7c84: mov qword ptr [rbp + 0x1c0], rax
0x1408f7c8b: mov qword ptr [rbp + 0x1c8], 2
0x1408f7c96: lea rax, [rbp + 0xdb0]
0x1408f7c9d: mov qword ptr [rbp + 0x1d0], rax
0x1408f7ca4: mov qword ptr [rbp + 0x1d8], 2
0x1408f7caf: mov qword ptr [rbp + 0x1e0], r15
0x1408f7cb6: mov qword ptr [rbp + 0x1e8], 2
0x1408f7cc1: lea rcx, [rbp + 0xbb0]
0x1408f7cc8: lea rdx, [rbp + 0x1c0]
0x1408f7ccf: call 0x140d79f90
0x1408f7cd4: mov rdx, qword ptr [rbp + 0xf00]
0x1408f7cdb: test rdx, rdx
0x1408f7cde: je 0x1408f7cf2
0x1408f7ce0: mov rcx, qword ptr [rbp + 0xf08]
0x1408f7ce7: mov r8d, 1
0x1408f7ced: call 0x140613c20
0x1408f7cf2: movups xmm0, xmmword ptr [rbp + 0xbb0]
0x1408f7cf9: movaps xmmword ptr [rbp + 0xf00], xmm0
0x1408f7d00: mov rax, qword ptr [rbp + 0xbc0]
0x1408f7d07: mov qword ptr [rbp + 0xf10], rax
0x1408f7d0e: mov eax, dword ptr [rip + 0x2d3cfb4]
0x1408f7d14: cmp eax, 3
0x1408f7d17: jne 0x1408f82a7
0x1408f7d1d: mov rdx, qword ptr [rip + 0x2d3cf9c]
0x1408f7d24: mov rcx, r15
0x1408f7d27: call 0x140542890
0x1408f7d2c: cmp dword ptr [rbp + 0x7c0], 1
0x1408f7d33: je 0x1408f9738
0x1408f7d39: mov rdi, r12
0x1408f7d3c: mov r12, rsi
0x1408f7d3f: mov rax, qword ptr [rbp + 0x7c8]
0x1408f7d46: mov qword ptr [rbp + 0x1020], rax
0x1408f7d4d: movzx eax, byte ptr [rbp + 0x7d0]
0x1408f7d54: mov byte ptr [rbp + 0x1018], al
0x1408f7d5a: movzx eax, byte ptr [rip + 0x2d3cea8]
0x1408f7d61: mov ecx, 0xf
0x1408f7d66: mov edx, 1
0x1408f7d6b: call 0x140613c10
0x1408f7d70: test rax, rax
0x1408f7d73: je 0x1408f9783
0x1408f7d79: mov rsi, r13
0x1408f7d7c: movabs rcx, 0x2928202168746170
0x1408f7d86: mov qword ptr [rax + 7], rcx
0x1408f7d8a: movabs rcx, 0x705f656c75646f6d
0x1408f7d94: mov qword ptr [rax], rcx
0x1408f7d97: mov qword ptr [rbp + 0x7c0], 0xf
0x1408f7da2: mov qword ptr [rbp + 0x7c8], rax
0x1408f7da9: mov qword ptr [rbp + 0x7d0], 0xf
0x1408f7db4: mov qword ptr [rsp + 0x20], 1
0x1408f7dbd: mov edx, 0xf
0x1408f7dc2: mov r8d, 0x18
0x1408f7dc8: mov r9d, 1
0x1408f7dce: mov rcx, r15
0x1408f7dd1: call 0x1428a9a60
0x1408f7dd6: mov rax, qword ptr [rbp + 0x7c8]
0x1408f7ddd: mov r14, qword ptr [rbp + 0x7d0]
0x1408f7de4: movups xmm0, xmmword ptr [rip + 0x21b0ec8]
0x1408f7deb: movups xmmword ptr [rax + r14], xmm0
0x1408f7df0: movabs rcx, 0x726f727245203a3a
0x1408f7dfa: mov qword ptr [rax + r14 + 0x10], rcx
0x1408f7dff: add r14, 0x18
0x1408f7e03: mov qword ptr [rbp + 0x1d0], r14
0x1408f7e0a: movups xmm0, xmmword ptr [rbp + 0x7c0]
0x1408f7e11: movaps xmmword ptr [rbp + 0x1c0], xmm0
0x1408f7e18: mov rbx, qword ptr [rbp + 0xf08]
0x1408f7e1f: mov r13, qword ptr [rbp + 0xf10]
0x1408f7e26: mov rax, qword ptr [rbp + 0x1c0]
0x1408f7e2d: sub rax, r14
0x1408f7e30: cmp rax, r13
0x1408f7e33: jb 0x1408f82e5
0x1408f7e39: mov rcx, qword ptr [rbp + 0x1c8]
0x1408f7e40: add rcx, r14
0x1408f7e43: mov rdx, rbx
0x1408f7e46: mov r8, r13
0x1408f7e49: call 0x142923c00
0x1408f7e4e: add r14, r13
0x1408f7e51: mov qword ptr [rbp + 0x1d0], r14
0x1408f7e58: mov qword ptr [rbp + 0x7d0], r14
0x1408f7e5f: movaps xmm0, xmmword ptr [rbp + 0x1c0]
0x1408f7e66: movaps xmmword ptr [rbp + 0x7c0], xmm0
0x1408f7e6d: mov rax, qword ptr [rbp + 0x1020]
0x1408f7e74: lea rcx, [rax + 8]
0x1408f7e78: mov rdx, r15
0x1408f7e7b: call 0x14143b8b0
0x1408f7e80: mov r13, rsi
0x1408f7e83: mov rsi, r12
0x1408f7e86: mov r12, rdi
0x1408f7e89: lea rdi, [rbp + 0xcb0]
0x1408f7e90: mov rbx, qword ptr [rbp + 0x1030]
0x1408f7e97: test al, al
0x1408f7e99: jne 0x1408f7f88
0x1408f7e9f: mov rax, qword ptr [rip + 0x2d34502]
0x1408f7ea6: mov rax, qword ptr [rax]
0x1408f7ea9: test rax, rax
0x1408f7eac: je 0x1408f7f88
0x1408f7eb2: mov qword ptr [rbp + 0x1c0], r13
0x1408f7eb9: lea rax, [rip - 0x8bc3f0]
0x1408f7ec0: mov qword ptr [rbp + 0x1c8], rax
0x1408f7ec7: lea rcx, [rip + 0x21b0dba]
0x1408f7ece: call 0x14067dd80
0x1408f7ed3: movups xmm0, xmmword ptr [rax]
0x1408f7ed6: mov eax, dword ptr [rax + 0x10]
0x1408f7ed9: mov qword ptr [rbp + 0x7f0], 1
0x1408f7ee4: lea rdx, [rip + 0x21b0df5]
0x1408f7eeb: mov qword ptr [rbp + 0x7f8], rdx
0x1408f7ef2: mov qword ptr [rbp + 0x800], 0x35
0x1408f7efd: lea rcx, [rip + 0x21b0dcc]
0x1408f7f04: mov qword ptr [rbp + 0x810], rcx
0x1408f7f0b: mov qword ptr [rbp + 0x818], 1
0x1408f7f16: lea rcx, [rbp + 0x1c0]
0x1408f7f1d: mov qword ptr [rbp + 0x820], rcx
0x1408f7f24: mov qword ptr [rbp + 0x828], 1
0x1408f7f2f: mov qword ptr [rbp + 0x830], 0
0x1408f7f3a: mov qword ptr [rbp + 0x7c0], 0
0x1408f7f45: mov qword ptr [rbp + 0x7c8], rdx
0x1408f7f4c: mov qword ptr [rbp + 0x7d0], 0x35
0x1408f7f57: mov qword ptr [rbp + 0x7d8], 0
0x1408f7f62: movups xmmword ptr [rbp + 0x7e0], xmm0
0x1408f7f69: mov dword ptr [rbp + 0x808], 1
0x1408f7f73: mov dword ptr [rbp + 0x80c], eax
0x1408f7f79: lea rcx, [rbp + 0xdb0]
0x1408f7f80: mov rdx, r15
0x1408f7f83: call 0x140d4a330
0x1408f7f88: cmp byte ptr [rbp + 0x1018], 0
0x1408f7f8f: mov r14, qword ptr [rbp + 0x1020]
0x1408f7f96: jne 0x1408f7fae
0x1408f7f98: mov rax, qword ptr [rip + 0x2d34481]
0x1408f7f9f: mov rax, qword ptr [rax]
0x1408f7fa2: shl rax, 1
0x1408f7fa5: test rax, rax
0x1408f7fa8: jne 0x1408f831f
0x1408f7fae: xor eax, eax
0x1408f7fb0: xchg byte ptr [r14], al
0x1408f7fb3: cmp al, 2
0x1408f7fb5: je 0x1408f8312
0x1408f7fbb: mov rdx, qword ptr [rbp + 0xf00]
0x1408f7fc2: test rdx, rdx
0x1408f7fc5: je 0x1408f77f0
0x1408f7fcb: mov rcx, qword ptr [rbp + 0xf08]
0x1408f7fd2: mov r8d, 1
0x1408f7fd8: call 0x140613c20
0x1408f7fdd: jmp 0x1408f77f0
0x1408f7fe2: mov rdx, qword ptr [rbp + 0xbc0]
0x1408f7fe9: lea r8, [rbp + 0xcc0]
0x1408f7ff0: mov qword ptr [r8 + 0x10], rdx
0x1408f7ff4: movaps xmm0, xmmword ptr [rbp + 0xbb0]
0x1408f7ffb: movups xmmword ptr [r8], xmm0
0x1408f7fff: mov qword ptr [rbp + 0xcb0], rcx
0x1408f8006: mov qword ptr [rbp + 0xcb8], rax
0x1408f800d: lea rcx, [rbp + 0xed8]
0x1408f8014: mov rdx, rdi
0x1408f8017: call 0x140bad8d0
0x1408f801c: mov rax, qword ptr [rbp + 0x1030]
0x1408f8023: lea r8, [rax + 0x28]
0x1408f8027: mov rcx, qword ptr [rbp + 0xee0]
0x1408f802e: mov rax, qword ptr [rbp + 0xee8]
0x1408f8035: mov qword ptr [rbp + 0x1020], rcx
0x1408f803c: mov qword ptr [rbp + 0xbb0], rcx
0x1408f8043: mov qword ptr [rbp + 0xbb8], rax
0x1408f804a: movups xmmword ptr [rbp + 0xbc0], xmm7
0x1408f8051: movups xmmword ptr [rbp + 0xbd0], xmm9
0x1408f8059: mov byte ptr [rsp + 0x28], 0x22
0x1408f805e: mov qword ptr [rsp + 0x20], 1
0x1408f8067: mov rcx, r13
0x1408f806a: mov rdx, qword ptr [rbp + 0xda8]
0x1408f8071: lea r9, [rbp + 0xbb0]
0x1408f8078: call 0x140bfe9c0
0x1408f807d: cmp dword ptr [rbp + 0xf00], 0xb
0x1408f8084: jne 0x1408f818b
0x1408f808a: mov rax, qword ptr [rip + 0x2d34317]
0x1408f8091: mov rax, qword ptr [rax]
0x1408f8094: cmp rax, 4
0x1408f8098: jb 0x1408f8152
0x1408f809e: lea rcx, [rip + 0x21b0ddb]
0x1408f80a5: call 0x14067dd80
0x1408f80aa: movups xmm0, xmmword ptr [rax]
0x1408f80ad: mov eax, dword ptr [rax + 0x10]
0x1408f80b0: mov qword ptr [rbp + 0x7f0], 4
0x1408f80bb: lea rdx, [rip + 0x21b0c1e]
0x1408f80c2: mov qword ptr [rbp + 0x7f8], rdx
0x1408f80c9: mov qword ptr [rbp + 0x800], 0x35
0x1408f80d4: lea rcx, [rip + 0x21b0d95]
0x1408f80db: mov qword ptr [rbp + 0x810], rcx
0x1408f80e2: mov qword ptr [rbp + 0x818], 1
0x1408f80ed: mov qword ptr [rbp + 0x820], 8
0x1408f80f8: mov qword ptr [rbp + 0x7c0], 0
0x1408f8103: lea rcx, [rbp + 0x7d0]
0x1408f810a: movups xmmword ptr [rcx + 0x58], xmm10
0x1408f810f: mov qword ptr [rbp + 0x7c8], rdx
0x1408f8116: mov qword ptr [rbp + 0x7d0], 0x35
0x1408f8121: mov qword ptr [rbp + 0x7d8], 0
0x1408f812c: movups xmmword ptr [rbp + 0x7e0], xmm0
0x1408f8133: mov dword ptr [rbp + 0x808], 1
0x1408f813d: mov dword ptr [rbp + 0x80c], eax
0x1408f8143: lea rcx, [rbp + 0xdb0]
0x1408f814a: mov rdx, r15
0x1408f814d: call 0x140d4a330
0x1408f8152: mov rax, qword ptr [rbp + 0x1030]
0x1408f8159: mov dword ptr [rax + 0x20], ebx
0x1408f815c: mov dword ptr [rax + 0x24], r14d
0x1408f8160: mov rdx, qword ptr [rbp + 0xed8]
0x1408f8167: test rdx, rdx
0x1408f816a: je 0x1408f817e
0x1408f816c: mov r8d, 1
0x1408f8172: mov rcx, qword ptr [rbp + 0x1020]
0x1408f8179: call 0x140613c20
0x1408f817e: mov rcx, rdi
0x1408f8181: call 0x1408f5e40
0x1408f8186: jmp 0x1408f77fc
0x1408f818b: movups xmm0, xmmword ptr [rbp + 0xf40]
0x1408f8192: movaps xmmword ptr [rbp + 0x200], xmm0
0x1408f8199: movups xmm0, xmmword ptr [rbp + 0xf00]
0x1408f81a0: movups xmm1, xmmword ptr [rbp + 0xf10]
0x1408f81a7: movups xmm2, xmmword ptr [rbp + 0xf20]
0x1408f81ae: movups xmm3, xmmword ptr [rbp + 0xf30]
0x1408f81b5: movaps xmmword ptr [rbp + 0x1f0], xmm3
0x1408f81bc: movaps xmmword ptr [rbp + 0x1e0], xmm2
0x1408f81c3: movaps xmmword ptr [rbp + 0x1d0], xmm1
0x1408f81ca: movaps xmmword ptr [rbp + 0x1c0], xmm0
0x1408f81d1: mov byte ptr [rbp + 0x1042], 1
0x1408f81d8: mov rcx, r15
0x1408f81db: call 0x140d56700
0x1408f81e0: movups xmm0, xmmword ptr [rbp + 0x7c0]
0x1408f81e7: movups xmm1, xmmword ptr [rbp + 0x7d0]
0x1408f81ee: movups xmm2, xmmword ptr [rbp + 0x7e0]
0x1408f81f5: movaps xmmword ptr [rbp + 0xdd0], xmm2
0x1408f81fc: movaps xmmword ptr [rbp + 0xdc0], xmm1
0x1408f8203: movaps xmmword ptr [rbp + 0xdb0], xmm0
0x1408f820a: movaps xmm0, xmmword ptr [rbp + 0x200]
0x1408f8211: lea rax, [rbp + 0x7d0]
0x1408f8218: movups xmmword ptr [rax + 0x40], xmm0
0x1408f821c: movaps xmm0, xmmword ptr [rbp + 0x1c0]
0x1408f8223: movaps xmm1, xmmword ptr [rbp + 0x1d0]
0x1408f822a: movaps xmm2, xmmword ptr [rbp + 0x1e0]
0x1408f8231: movaps xmm3, xmmword ptr [rbp + 0x1f0]
0x1408f8238: movups xmmword ptr [rax + 0x30], xmm3
0x1408f823c: movups xmmword ptr [rax + 0x20], xmm2
0x1408f8240: movups xmmword ptr [rax + 0x10], xmm1
0x1408f8244: movups xmmword ptr [rax], xmm0
0x1408f8247: lea rax, [rip + 0x21b0bed]
0x1408f824e: mov qword ptr [rbp + 0x7c0], rax
0x1408f8255: mov qword ptr [rbp + 0x7c8], 0x10
0x1408f8260: mov byte ptr [rbp + 0x1042], 0
0x1408f8267: mov rcx, r15
0x1408f826a: lea rdx, [rbp + 0xdb0]
0x1408f8271: call 0x1428a8850
0x1408f8276: mov rbx, rax
0x1408f8279: mov rdx, qword ptr [rbp + 0xed8]
0x1408f8280: test rdx, rdx
0x1408f8283: je 0x1408f8297
0x1408f8285: mov r8d, 1
0x1408f828b: mov rcx, qword ptr [rbp + 0x1020]
0x1408f8292: call 0x140613c20
0x1408f8297: mov rcx, rdi
0x1408f829a: call 0x1408f5e40
0x1408f829f: mov rax, rbx
0x1408f82a2: jmp 0x1408f7bc0
0x1408f82a7: mov byte ptr [rbp + 0x1c0], 1
0x1408f82ae: lea rax, [rbp + 0x1c0]
0x1408f82b5: mov qword ptr [rbp + 0x7c0], rax
0x1408f82bc: lea rax, [rip + 0x21b09c5]
0x1408f82c3: mov qword ptr [rsp + 0x20], rax
0x1408f82c8: lea rcx, [rip + 0x2d3c9f9]
0x1408f82cf: xor edx, edx
0x1408f82d1: mov r8, r15
0x1408f82d4: lea r9, [rip + 0x21b05fd]
0x1408f82db: call 0x1428d8820
0x1408f82e0: jmp 0x1408f7d1d
0x1408f82e5: mov qword ptr [rsp + 0x20], 1
0x1408f82ee: mov r9d, 1
0x1408f82f4: lea rcx, [rbp + 0x1c0]
0x1408f82fb: mov rdx, r14
0x1408f82fe: mov r8, r13
0x1408f8301: call 0x1428a9a60
0x1408f8306: mov r14, qword ptr [rbp + 0x1d0]
0x1408f830d: jmp 0x1408f7e39
0x1408f8312: mov rcx, r14
0x1408f8315: call 0x1428d8810
0x1408f831a: jmp 0x1408f7fbb
0x1408f831f: call 0x1428d86b0
0x1408f8324: test al, al
0x1408f8326: jne 0x1408f7fae
0x1408f832c: mov byte ptr [r14 + 1], 1
0x1408f8331: jmp 0x1408f7fae
0x1408f8336: lea rax, [rbp + 0x6a0]
0x1408f833d: mov qword ptr [rbp + 0x7c0], rax
0x1408f8344: mov rax, qword ptr [rbp + 0x1000]
0x1408f834b: mov qword ptr [rbp + 0x7c8], rax
0x1408f8352: mov rax, qword ptr [rbp + 0x1028]
0x1408f8359: mov qword ptr [rbp + 0x7d0], rax
0x1408f8360: lea r8, [rip + 0x21b0661]
0x1408f8367: lea rsi, [rbp + 0xed8]
0x1408f836e: lea rdx, [rbp + 0x7c0]
0x1408f8375: mov rcx, rsi
0x1408f8378: call 0x14130f1f0
0x1408f837d: mov rax, qword ptr [rbp + 0x1010]
0x1408f8384: mov qword ptr [rbp + 0xef0], rax
0x1408f838b: mov rax, qword ptr [rbp + 0xec0]
0x1408f8392: mov qword ptr [rbp + 0xef8], rax
0x1408f8399: mov rdi, qword ptr [rbp + 0xd90]
0x1408f83a0: lea rcx, [rdi + 4]
0x1408f83a4: mov qword ptr [rbp + 0x1028], rcx
0x1408f83ab: call 0x140bd5790
0x1408f83b0: mov dword ptr [rbp + 0x7c0], 0
0x1408f83ba: mov qword ptr [rbp + 0x7c8], rdi
0x1408f83c1: mov dword ptr [rbp + 0xf00], 2
0x1408f83cb: mov rcx, qword ptr [rbp + 0xeb0]
0x1408f83d2: mov qword ptr [rsp + 0x40], rcx
0x1408f83d7: mov rcx, qword ptr [rbp + 0xea8]
0x1408f83de: mov qword ptr [rsp + 0x38], rcx
0x1408f83e3: mov qword ptr [rsp + 0x30], rax
0x1408f83e8: mov qword ptr [rsp + 0x28], r13
0x1408f83ed: mov qword ptr [rsp + 0x20], 1
0x1408f83f6: lea rdx, [rip + 0x21b0cdb]
0x1408f83fd: lea rcx, [rbp + 0x1c0]
0x1408f8404: lea r9, [rbp + 0x7c0]
0x1408f840b: mov r8d, 0x1b
0x1408f8411: call 0x140867970
0x1408f8416: mov byte ptr [rbp + 0x1046], 1
0x1408f841d: call 0x140be6750
0x1408f8422: mov edi, eax
0x1408f8424: mov byte ptr [rbp + 0x1046], 1
0x1408f842b: xor ecx, ecx
0x1408f842d: call 0x1400d9b20
0x1408f8432: test rax, rax
0x1408f8435: je 0x1408f9822
0x1408f843b: mov dword ptr [rbp + 0xfa8], edi
0x1408f8441: mov word ptr [rbp + 0xfb0], 0
0x1408f844a: mov dword ptr [rbp + 0xfac], 0x3f800000
0x1408f8454: mov qword ptr [rbp + 0xf00], 0
0x1408f845f: mov rcx, qword ptr [rbp + 0xd98]
0x1408f8466: mov qword ptr [rbp + 0xfa0], rcx
0x1408f846d: mov qword ptr [rbp + 0xf10], 0
0x1408f8478: mov qword ptr [rbp + 0xf18], 4
0x1408f8483: mov qword ptr [rbp + 0xf20], 0
0x1408f848e: movups xmm0, xmmword ptr [rax]
0x1408f8491: inc qword ptr [rax]
0x1408f8494: movups xmm9, xmmword ptr [rip + 0x219e294]
0x1408f849c: movups xmmword ptr [rbp + 0xf80], xmm9
0x1408f84a4: movups xmm10, xmmword ptr [rip + 0x219e274]
0x1408f84ac: movups xmmword ptr [rbp + 0xf70], xmm10
0x1408f84b4: movups xmmword ptr [rbp + 0xf90], xmm0
0x1408f84bb: movabs rax, 0x100000003
0x1408f84c5: mov qword ptr [rbp + 0xf40], rax
0x1408f84cc: mov qword ptr [rbp + 0xf28], 0
0x1408f84d7: mov qword ptr [rbp + 0xf30], 1
0x1408f84e2: mov qword ptr [rbp + 0xf38], 0
0x1408f84ed: mov byte ptr [rbp + 0x1046], 1
0x1408f84f4: lea rcx, [rbp + 0x7c0]
0x1408f84fb: lea rdx, [rbp + 0xf00]
0x1408f8502: lea r8, [rbp + 0x1c0]
0x1408f8509: xor r9d, r9d
0x1408f850c: call 0x140905be0
0x1408f8511: mov byte ptr [rbp + 0x1041], 1
0x1408f8518: mov rcx, qword ptr [rbp + 0x1028]
0x1408f851f: call 0x140bd5790
0x1408f8524: mov byte ptr [rbp + 0x1041], 0
0x1408f852b: lea rcx, [rbp + 0xf00]
0x1408f8532: lea rdi, [rbp + 0x7c0]
0x1408f8539: movabs r8, 0x531bb9318eea4971
0x1408f8543: mov rdx, rdi
0x1408f8546: mov r9, rax
0x1408f8549: call 0x141629d20
0x1408f854e: cmp byte ptr [rbp + 0xfde], 8
0x1408f8555: movshdup xmm11, xmm6
0x1408f855a: je 0x1408f8588
0x1408f855c: lea rcx, [rbp + 0xfb8]
0x1408f8563: mov edx, 0xe8
0x1408f8568: add rdx, qword ptr [rbp + 0xfe8]
0x1408f856f: movss dword ptr [rsp + 0x20], xmm11
0x1408f8576: movabs r8, 0x27763b3d9cc2a8a
0x1408f8580: movaps xmm3, xmm6
0x1408f8583: call 0x14096b940
0x1408f8588: lea rbx, [rbp + 0xdb0]
0x1408f858f: lea rdx, [rbp + 0xf00]
0x1408f8596: mov r8d, 0xf8
0x1408f859c: mov rcx, rbx
0x1408f859f: call 0x142923c00
0x1408f85a4: movabs rax, 0x730032821e11adf2
0x1408f85ae: mov qword ptr [rbp + 0xf00], rax
0x1408f85b5: mov dword ptr [rbp + 0x7c0], 2
0x1408f85bf: mov qword ptr [rsp + 0x20], rdi
0x1408f85c4: lea rcx, [rbp + 0xf00]
0x1408f85cb: movabs r8, 0x7f41ce7531a06fe3
0x1408f85d5: mov rdx, rbx
0x1408f85d8: mov r9b, 0x1d
0x1408f85db: call 0x140908610
0x1408f85e0: mov rdi, qword ptr [rbp + 0xdb0]
0x1408f85e7: lea rdx, [rbp + 0xdb8]
0x1408f85ee: lea rcx, [rbp + 0x7c8]
0x1408f85f5: mov r8d, 0xa0
0x1408f85fb: call 0x142923c00
0x1408f8600: movzx eax, byte ptr [rbp + 0xe58]
0x1408f8607: movups xmm0, xmmword ptr [rbp + 0xe59]
0x1408f860e: movups xmmword ptr [rbp + 0x869], xmm0
0x1408f8615: movups xmm0, xmmword ptr [rbp + 0xe69]
0x1408f861c: movups xmmword ptr [rbp + 0x879], xmm0
0x1408f8623: movups xmm0, xmmword ptr [rbp + 0xe79]
0x1408f862a: movups xmmword ptr [rbp + 0x889], xmm0
0x1408f8631: movups xmm0, xmmword ptr [rbp + 0xe89]
0x1408f8638: movups xmmword ptr [rbp + 0x899], xmm0
0x1408f863f: movups xmm0, xmmword ptr [rbp + 0xe98]
0x1408f8646: movups xmmword ptr [rbp + 0x8a8], xmm0
0x1408f864d: and al, 0x7f
0x1408f864f: mov qword ptr [rbp + 0x7c0], rdi
0x1408f8656: mov byte ptr [rbp + 0x868], al
0x1408f865c: mov byte ptr [rbp + 0x1046], 1
0x1408f8663: lea rcx, [rbp + 0x7c0]
0x1408f866a: mov edx, 3
0x1408f866f: call 0x140906a90
0x1408f8674: lea rax, [rbp + 0xef0]
0x1408f867b: mov rdx, qword ptr [rbp + 0xeb8]
0x1408f8682: mov rcx, qword ptr [rdx + 8]
0x1408f8686: mov rdx, qword ptr [rdx + 0x10]
0x1408f868a: lea r8, [rdx + rdx*8]
0x1408f868e: lea r8, [r8 + r8*2]
0x1408f8692: add rdx, rcx
0x1408f8695: add rdx, r8
0x1408f8698: mov qword ptr [rbp + 0x7c0], rcx
0x1408f869f: mov qword ptr [rbp + 0x7c8], rdx
0x1408f86a6: mov qword ptr [rbp + 0x7d0], 0
0x1408f86b1: mov qword ptr [rbp + 0x7d8], rax
0x1408f86b8: mov qword ptr [rbp + 0x7e0], rsi
0x1408f86bf: mov byte ptr [rbp + 0x1046], 1
0x1408f86c6: lea r8, [rip + 0x21b02fb]
0x1408f86cd: lea rcx, [rbp + 0xd68]
0x1408f86d4: lea rdx, [rbp + 0x7c0]
0x1408f86db: call 0x1412fdcc0
0x1408f86e0: mov byte ptr [rbp + 0x1047], 1
0x1408f86e7: call 0x140be6750
0x1408f86ec: mov esi, eax
0x1408f86ee: mov byte ptr [rbp + 0x1047], 1
0x1408f86f5: xor ecx, ecx
0x1408f86f7: call 0x1400d9b20
0x1408f86fc: test rax, rax
0x1408f86ff: mov rcx, qword ptr [rbp + 0xda0]
0x1408f8706: je 0x1408f9837
0x1408f870c: mov dword ptr [rbp + 0xfa8], esi
0x1408f8712: mov word ptr [rbp + 0xfb0], 0
0x1408f871b: mov dword ptr [rbp + 0xfac], 0x3f800000
0x1408f8725: mov qword ptr [rbp + 0xf00], 0
0x1408f8730: mov qword ptr [rbp + 0xfa0], rcx
0x1408f8737: mov qword ptr [rbp + 0xf10], 0
0x1408f8742: mov qword ptr [rbp + 0xf18], 4
0x1408f874d: mov qword ptr [rbp + 0xf20], 0
0x1408f8758: movups xmm0, xmmword ptr [rax]
0x1408f875b: inc qword ptr [rax]
0x1408f875e: movups xmmword ptr [rbp + 0xf80], xmm9
0x1408f8766: movups xmmword ptr [rbp + 0xf70], xmm10
0x1408f876e: movups xmmword ptr [rbp + 0xf90], xmm0
0x1408f8775: movabs rax, 0x100000003
0x1408f877f: mov qword ptr [rbp + 0xf40], rax
0x1408f8786: mov qword ptr [rbp + 0xf28], 0
0x1408f8791: mov qword ptr [rbp + 0xf30], 1
0x1408f879c: mov qword ptr [rbp + 0xf38], 0
0x1408f87a7: mov byte ptr [rbp + 0x1047], 1
0x1408f87ae: lea rcx, [rbp + 0x7c0]
0x1408f87b5: lea rdx, [rbp + 0xf00]
0x1408f87bc: lea r8, [rbp + 0x1c0]
0x1408f87c3: xor r9d, r9d
0x1408f87c6: call 0x140905be0
0x1408f87cb: mov byte ptr [rbp + 0x1040], 1
0x1408f87d2: mov rcx, qword ptr [rbp + 0x1028]
0x1408f87d9: call 0x140bd5790
0x1408f87de: mov byte ptr [rbp + 0x1040], 0
0x1408f87e5: lea rcx, [rbp + 0xf00]
0x1408f87ec: lea rdx, [rbp + 0x7c0]
0x1408f87f3: movabs r8, 0x531bb9318eea4971
0x1408f87fd: mov r9, rax
0x1408f8800: call 0x141629d20
0x1408f8805: cmp byte ptr [rbp + 0xfde], 8
0x1408f880c: je 0x1408f8859
0x1408f880e: mov rcx, qword ptr [rbp + 0x1008]
0x1408f8815: mov rax, qword ptr [rcx]
0x1408f8818: cmp qword ptr [rcx + 8], 0
0x1408f881d: je 0x1408f8828
0x1408f881f: xorps xmm3, xmm3
0x1408f8822: test al, 1
0x1408f8824: je 0x1408f8834
0x1408f8826: jmp 0x1408f8837
0x1408f8828: movss xmm3, dword ptr [rip + 0x202d0d4]
0x1408f8830: test al, 1
0x1408f8832: jne 0x1408f8837
0x1408f8834: xorps xmm3, xmm3
0x1408f8837: lea rcx, [rbp + 0xfb8]
0x1408f883e: mov edx, 0xe8
0x1408f8843: add rdx, qword ptr [rbp + 0xfe8]
0x1408f884a: movabs r8, 0x54f25b7b526cbbad
0x1408f8854: call 0x14096ba30
0x1408f8859: lea rcx, [rbp + 0x7c0]
0x1408f8860: lea rdx, [rbp + 0xf00]
0x1408f8867: mov r8d, 0xf8
0x1408f886d: call 0x142923c00
0x1408f8872: xorps xmm3, xmm3
0x1408f8875: addss xmm11, xmm3
0x1408f887a: cmp byte ptr [rbp + 0x89e], 8
0x1408f8881: je 0x1408f88b5
0x1408f8883: mulss xmm3, xmm8
0x1408f8888: addss xmm3, xmm6
0x1408f888c: lea rcx, [rbp + 0x878]
0x1408f8893: mov edx, 0xe8
0x1408f8898: add rdx, qword ptr [rbp + 0x8a8]
0x1408f889f: movss dword ptr [rsp + 0x20], xmm11
0x1408f88a6: movabs r8, 0x27763b3d9cc2a8a
0x1408f88b0: call 0x14096b940
0x1408f88b5: lea rcx, [rbp + 0xf00]
0x1408f88bc: lea rdx, [rbp + 0x7c0]
0x1408f88c3: mov r8d, 0xf8
0x1408f88c9: call 0x142923c00
0x1408f88ce: cmp byte ptr [rbp + 0xfde], 8
0x1408f88d5: movaps xmm7, xmm6
0x1408f88d8: unpckhpd xmm7, xmm6
0x1408f88dc: movaps xmm12, xmm6
0x1408f88e0: shufps xmm12, xmm6, 0xff
0x1408f88e5: je 0x1408f8913
0x1408f88e7: lea rcx, [rbp + 0xfb8]
0x1408f88ee: mov edx, 0xe8
0x1408f88f3: add rdx, qword ptr [rbp + 0xfe8]
0x1408f88fa: movss dword ptr [rsp + 0x20], xmm12
0x1408f8901: movabs r8, 0x7038b90f28755919
0x1408f890b: movaps xmm3, xmm7
0x1408f890e: call 0x14096b940
0x1408f8913: lea rcx, [rbp + 0x7c0]
0x1408f891a: lea rdx, [rbp + 0xf00]
0x1408f8921: mov r8d, 0xf8
0x1408f8927: call 0x142923c00
0x1408f892c: cmp byte ptr [rbp + 0x89e], 8
0x1408f8933: je 0x1408f895a
0x1408f8935: lea rcx, [rbp + 0x878]
0x1408f893c: mov edx, 0xe8
0x1408f8941: add rdx, qword ptr [rbp + 0x8a8]
0x1408f8948: movabs r8, 0x57ca25b57d60d824
0x1408f8952: xor r9d, r9d
0x1408f8955: call 0x14096b570
0x1408f895a: lea rsi, [rbp + 0xf00]
0x1408f8961: lea rdi, [rbp + 0x7c0]
0x1408f8968: mov r8d, 0xf8
0x1408f896e: mov rcx, rsi
0x1408f8971: mov rdx, rdi
0x1408f8974: call 0x142923c00
0x1408f8979: movabs rax, 0xecf66ed2d52361b7
0x1408f8983: mov qword ptr [rbp + 0xdb0], rax
0x1408f898a: mov dword ptr [rbp + 0x7c0], 2
0x1408f8994: mov qword ptr [rsp + 0x20], rdi
0x1408f8999: lea rcx, [rbp + 0xdb0]
0x1408f89a0: movabs r8, 0xe63d19ff7655ff61
0x1408f89aa: mov rdx, rsi
0x1408f89ad: mov r9b, 0x1d
0x1408f89b0: call 0x140908610
0x1408f89b5: mov rsi, qword ptr [rbp + 0xf00]
0x1408f89bc: lea rdx, [rbp + 0xf08]
0x1408f89c3: lea rcx, [rbp + 0x7c8]
0x1408f89ca: mov r8d, 0xa0
0x1408f89d0: call 0x142923c00
0x1408f89d5: movzx eax, byte ptr [rbp + 0xfa8]
0x1408f89dc: movzx ecx, word ptr [rbp + 0xfa9]
0x1408f89e3: movzx edx, byte ptr [rbp + 0xfab]
0x1408f89ea: movups xmm0, xmmword ptr [rbp + 0xfac]
0x1408f89f1: movups xmmword ptr [rbp + 0x86c], xmm0
0x1408f89f8: movups xmm0, xmmword ptr [rbp + 0xfbc]
0x1408f89ff: movups xmmword ptr [rbp + 0x87c], xmm0
0x1408f8a06: movups xmm0, xmmword ptr [rbp + 0xfcc]
0x1408f8a0d: movups xmmword ptr [rbp + 0x88c], xmm0
0x1408f8a14: movups xmm0, xmmword ptr [rbp + 0xfdc]
0x1408f8a1b: movups xmmword ptr [rbp + 0x89c], xmm0
0x1408f8a22: movups xmm0, xmmword ptr [rbp + 0xfe8]
0x1408f8a29: movups xmmword ptr [rbp + 0x8a8], xmm0
0x1408f8a30: and al, 0x7f
0x1408f8a32: and dl, 0x9f
0x1408f8a35: or dl, 0x40
0x1408f8a38: mov qword ptr [rbp + 0x7c0], rsi
0x1408f8a3f: mov byte ptr [rbp + 0x868], al
0x1408f8a45: mov word ptr [rbp + 0x869], cx
0x1408f8a4c: mov byte ptr [rbp + 0x86b], dl
0x1408f8a52: mov byte ptr [rbp + 0x1047], 1
0x1408f8a59: lea r12, [rbp + 0x7c0]
0x1408f8a60: mov edx, 6
0x1408f8a65: mov rcx, r12
0x1408f8a68: call 0x140906a90
0x1408f8a6d: mov rbx, qword ptr [rbp + 0xd70]
0x1408f8a74: mov rax, qword ptr [rbp + 0xd78]
0x1408f8a7b: shl rax, 4
0x1408f8a7f: add rax, rbx
0x1408f8a82: mov qword ptr [rbp + 0x1030], rax
0x1408f8a89: lea r14, [rbp - 0x28]
0x1408f8a8d: movss xmm13, dword ptr [rip + 0x202ce6e]
0x1408f8a96: mov qword ptr [rbp + 0x1018], rbx
0x1408f8a9d: xor r13d, r13d
0x1408f8aa0: cmp rbx, qword ptr [rbp + 0x1030]
0x1408f8aa7: je 0x1408f8f80
0x1408f8aad: nop dword ptr [rax]
0x1408f8ab0: mov r15, qword ptr [rbx]
0x1408f8ab3: mov rdi, qword ptr [rbx + 8]
0x1408f8ab7: mov byte ptr [rbp + 0x1047], 1
0x1408f8abe: call 0x140be6750
0x1408f8ac3: mov esi, eax
0x1408f8ac5: mov byte ptr [rbp + 0x1047], 1
0x1408f8acc: xor ecx, ecx
0x1408f8ace: call 0x1400d9b20
0x1408f8ad3: test rax, rax
0x1408f8ad6: je 0x1408f96f0
0x1408f8adc: mov dword ptr [rbp + 0xd58], esi
0x1408f8ae2: mov word ptr [rbp + 0xd60], 0
0x1408f8aeb: mov dword ptr [rbp + 0xd5c], 0x3f800000
0x1408f8af5: mov qword ptr [rbp + 0xcb0], 0
0x1408f8b00: mov rcx, qword ptr [rbp + 0xd80]
0x1408f8b07: mov qword ptr [rbp + 0xd50], rcx
0x1408f8b0e: mov qword ptr [rbp + 0xcc0], 0
0x1408f8b19: mov qword ptr [rbp + 0xcc8], 4
0x1408f8b24: mov qword ptr [rbp + 0xcd0], 0
0x1408f8b2f: movups xmm0, xmmword ptr [rax]
0x1408f8b32: inc qword ptr [rax]
0x1408f8b35: lea rax, [rbp + 0xd20]
0x1408f8b3c: movups xmmword ptr [rax + 0x10], xmm9
0x1408f8b41: movups xmmword ptr [rax], xmm10
0x1408f8b45: movups xmmword ptr [rbp + 0xd40], xmm0
0x1408f8b4c: movabs rax, 0x100000003
0x1408f8b56: mov qword ptr [rbp + 0xcf0], rax
0x1408f8b5d: mov qword ptr [rbp + 0xcd8], 0
0x1408f8b68: mov qword ptr [rbp + 0xce0], 1
0x1408f8b73: mov qword ptr [rbp + 0xce8], 0
0x1408f8b7e: mov byte ptr [rbp + 0x1047], 1
0x1408f8b85: mov rcx, r12
0x1408f8b88: lea rdx, [rbp + 0xcb0]
0x1408f8b8f: lea r8, [rbp + 0x1c0]
0x1408f8b96: xor r9d, r9d
0x1408f8b99: call 0x140905be0
0x1408f8b9e: mov byte ptr [rbp + 0x103e], 1
0x1408f8ba5: mov rcx, qword ptr [rbp + 0x1028]
0x1408f8bac: call 0x140bd5790
0x1408f8bb1: mov byte ptr [rbp + 0x103e], 0
0x1408f8bb8: lea rcx, [rbp + 0xf00]
0x1408f8bbf: mov rdx, r12
0x1408f8bc2: movabs r8, 0x531bb9318eea4971
0x1408f8bcc: mov r9, rax
0x1408f8bcf: call 0x141629d20
0x1408f8bd4: lea rsi, [r13 + 1]
0x1408f8bd8: cmp byte ptr [rbp + 0xfde], 8
0x1408f8bdf: je 0x1408f8c2b
0x1408f8be1: movaps xmm3, xmm13
0x1408f8be5: mov rax, qword ptr [rbp + 0x1008]
0x1408f8bec: cmp rsi, qword ptr [rax + 8]
0x1408f8bf0: je 0x1408f8bf5
0x1408f8bf2: xorps xmm3, xmm3
0x1408f8bf5: mov rax, qword ptr [rbp + 0x1008]
0x1408f8bfc: mov rax, qword ptr [rax]
0x1408f8bff: test al, 1
0x1408f8c01: jne 0x1408f8c06
0x1408f8c03: xorps xmm3, xmm3
0x1408f8c06: mov rdx, qword ptr [rbp + 0xfe8]
0x1408f8c0d: mov eax, 0xe8
0x1408f8c12: add rdx, rax
0x1408f8c15: lea rcx, [rbp + 0xfb8]
0x1408f8c1c: movabs r8, 0x54f25b7b526cbbad
0x1408f8c26: call 0x14096ba30
0x1408f8c2b: mov r8d, 0xf8
0x1408f8c31: lea rcx, [rbp + 0xdb0]
0x1408f8c38: lea rdx, [rbp + 0xf00]
0x1408f8c3f: call 0x142923c00
0x1408f8c44: cmp byte ptr [rbp + 0xe8e], 8
0x1408f8c4b: je 0x1408f8caf
0x1408f8c4d: test rsi, rsi
0x1408f8c50: js 0x1408f8c60
0x1408f8c52: xorps xmm3, xmm3
0x1408f8c55: cvtsi2ss xmm3, rsi
0x1408f8c5a: jmp 0x1408f8c7a
0x1408f8c5c: nop dword ptr [rax]
0x1408f8c60: mov rax, rsi
0x1408f8c63: shr rax, 1
0x1408f8c66: mov ecx, esi
0x1408f8c68: and ecx, 1
0x1408f8c6b: or rcx, rax
0x1408f8c6e: xorps xmm3, xmm3
0x1408f8c71: cvtsi2ss xmm3, rcx
0x1408f8c76: addss xmm3, xmm3
0x1408f8c7a: mulss xmm3, xmm8
0x1408f8c7f: addss xmm3, xmm6
0x1408f8c83: mov rdx, qword ptr [rbp + 0xe98]
0x1408f8c8a: mov eax, 0xe8
0x1408f8c8f: add rdx, rax
0x1408f8c92: movss dword ptr [rsp + 0x20], xmm11
0x1408f8c99: lea rcx, [rbp + 0xe68]
0x1408f8ca0: movabs r8, 0x27763b3d9cc2a8a
0x1408f8caa: call 0x14096b940
0x1408f8caf: mov r8d, 0xf8
0x1408f8cb5: lea rcx, [rbp + 0xbb0]
0x1408f8cbc: lea rdx, [rbp + 0xdb0]
0x1408f8cc3: call 0x142923c00
0x1408f8cc8: cmp byte ptr [rbp + 0xc8e], 8
0x1408f8ccf: je 0x1408f8d00
0x1408f8cd1: mov rdx, qword ptr [rbp + 0xc98]
0x1408f8cd8: mov eax, 0xe8
0x1408f8cdd: add rdx, rax
0x1408f8ce0: movss dword ptr [rsp + 0x20], xmm12
0x1408f8ce7: lea rcx, [rbp + 0xc68]
0x1408f8cee: movabs r8, 0x7038b90f28755919
0x1408f8cf8: movaps xmm3, xmm7
0x1408f8cfb: call 0x14096b940
0x1408f8d00: mov r8d, 0xf8
0x1408f8d06: lea rcx, [rbp + 0x5a8]
0x1408f8d0d: lea rdx, [rbp + 0xbb0]
0x1408f8d14: call 0x142923c00
0x1408f8d19: cmp byte ptr [rbp + 0x686], 8
0x1408f8d20: je 0x1408f8d4a
0x1408f8d22: mov rdx, qword ptr [rbp + 0x690]
0x1408f8d29: mov eax, 0xe8
0x1408f8d2e: add rdx, rax
0x1408f8d31: lea rcx, [rbp + 0x660]
0x1408f8d38: movabs r8, 0x57ca25b57d60d824
0x1408f8d42: mov r9d, esi
0x1408f8d45: call 0x14096b570
0x1408f8d4a: mov r8d, 0xf8
0x1408f8d50: lea rcx, [rbp + 0x6a0]
0x1408f8d57: lea rdx, [rbp + 0x5a8]
0x1408f8d5e: call 0x142923c00
0x1408f8d63: cmp r13, 3
0x1408f8d67: jb 0x1408f8d90
0x1408f8d69: mov r9b, 1
0x1408f8d6c: cmp byte ptr [rip + 0x2d3c01d], 0
0x1408f8d73: je 0x1408f8d90
0x1408f8d75: cmp byte ptr [rbp + 0x77e], 8
0x1408f8d7c: lea r13, [rbp - 0x30]
0x1408f8d80: jne 0x1408f8da6
0x1408f8d82: jmp 0x1408f8dcb
0x1408f8d84: nop word ptr cs:[rax + rax]
0x1408f8d90: cmp byte ptr [r15 + 0x18], 0
0x1408f8d95: setne r9b
0x1408f8d99: cmp byte ptr [rbp + 0x77e], 8
0x1408f8da0: lea r13, [rbp - 0x30]
0x1408f8da4: je 0x1408f8dcb
0x1408f8da6: mov rdx, qword ptr [rbp + 0x788]
0x1408f8dad: mov eax, 0xe8
0x1408f8db2: add rdx, rax
0x1408f8db5: lea rcx, [rbp + 0x758]
0x1408f8dbc: movabs r8, 0x10b711313e595441
0x1408f8dc6: call 0x14096b850
0x1408f8dcb: mov r8d, 0xf8
0x1408f8dd1: mov rcx, r13
0x1408f8dd4: lea rdx, [rbp + 0x6a0]
0x1408f8ddb: call 0x142923c00
0x1408f8de0: test rdi, rdi
0x1408f8de3: lea r15, [rbp + 0xf00]
0x1408f8dea: je 0x1408f8e20
0x1408f8dec: mov qword ptr [rbp + 0xbb0], rdi
0x1408f8df3: mov dword ptr [rbp + 0x7c0], 2
0x1408f8dfd: mov qword ptr [rsp + 0x20], r12
0x1408f8e02: lea rcx, [rbp + 0xbb0]
0x1408f8e09: mov rdx, r13
0x1408f8e0c: movabs r8, 0xe63d19ff7655ff61
0x1408f8e16: mov r9b, 0x1d
0x1408f8e19: call 0x1408fd6c0
0x1408f8e1e: jmp 0x1408f8e5c
0x1408f8e20: movabs rax, 0x806c29bbf7c5030d
0x1408f8e2a: mov qword ptr [rbp + 0xbb0], rax
0x1408f8e31: mov dword ptr [rbp + 0x7c0], 2
0x1408f8e3b: mov qword ptr [rsp + 0x20], r12
0x1408f8e40: lea rcx, [rbp + 0xbb0]
0x1408f8e47: mov rdx, r13
0x1408f8e4a: movabs r8, 0xe63d19ff7655ff61
0x1408f8e54: mov r9b, 0x1d
0x1408f8e57: call 0x140908610
0x1408f8e5c: mov rdi, qword ptr [rbp - 0x30]
0x1408f8e60: mov r8d, 0xa0
0x1408f8e66: mov rcx, r15
0x1408f8e69: mov rdx, r14
0x1408f8e6c: call 0x142923c00
0x1408f8e71: mov rdx, r15
0x1408f8e74: mov r15b, byte ptr [rbp + 0x78]
0x1408f8e78: mov r13w, word ptr [rbp + 0x79]
0x1408f8e7d: movzx r12d, byte ptr [rbp + 0x7b]
0x1408f8e82: movups xmm0, xmmword ptr [r14 + 0xa4]
0x1408f8e8a: movups xmm1, xmmword ptr [r14 + 0xb4]
0x1408f8e92: movups xmm2, xmmword ptr [r14 + 0xc4]
0x1408f8e9a: movups xmm3, xmmword ptr [r14 + 0xd4]
0x1408f8ea2: movaps xmmword ptr [rbp + 0xdb0], xmm0
0x1408f8ea9: movaps xmmword ptr [rbp + 0xdc0], xmm1
0x1408f8eb0: movaps xmmword ptr [rbp + 0xdd0], xmm2
0x1408f8eb7: movaps xmmword ptr [rbp + 0xde0], xmm3
0x1408f8ebe: movups xmm0, xmmword ptr [r14 + 0xe0]
0x1408f8ec6: movups xmmword ptr [rbp + 0xdec], xmm0
0x1408f8ecd: and r15b, 0x7f
0x1408f8ed1: and r12b, 0x9f
0x1408f8ed5: or r12b, 0x40
0x1408f8ed9: mov qword ptr [rbp + 0x7c0], rdi
0x1408f8ee0: mov r8d, 0xa0
0x1408f8ee6: lea rdi, [rbp + 0x7c8]
0x1408f8eed: mov rcx, rdi
0x1408f8ef0: call 0x142923c00
0x1408f8ef5: mov byte ptr [rbp + 0x868], r15b
0x1408f8efc: mov word ptr [rbp + 0x869], r13w
0x1408f8f04: mov byte ptr [rbp + 0x86b], r12b
0x1408f8f0b: movaps xmm0, xmmword ptr [rbp + 0xdb0]
0x1408f8f12: movaps xmm1, xmmword ptr [rbp + 0xdc0]
0x1408f8f19: movaps xmm2, xmmword ptr [rbp + 0xdd0]
0x1408f8f20: movaps xmm3, xmmword ptr [rbp + 0xde0]
0x1408f8f27: movups xmmword ptr [rdi + 0xa4], xmm0
0x1408f8f2e: movups xmmword ptr [rdi + 0xb4], xmm1
0x1408f8f35: movups xmmword ptr [rdi + 0xc4], xmm2
0x1408f8f3c: movups xmmword ptr [rdi + 0xd4], xmm3
0x1408f8f43: movups xmm0, xmmword ptr [rbp + 0xdec]
0x1408f8f4a: movups xmmword ptr [rdi + 0xe0], xmm0
0x1408f8f51: mov byte ptr [rbp + 0x1047], 1
0x1408f8f58: mov edx, 6
0x1408f8f5d: lea r12, [rbp + 0x7c0]
0x1408f8f64: mov rcx, r12
0x1408f8f67: call 0x140906a90
0x1408f8f6c: add rbx, 0x10
0x1408f8f70: mov r13, rsi
0x1408f8f73: cmp rbx, qword ptr [rbp + 0x1030]
0x1408f8f7a: jne 0x1408f8ab0
0x1408f8f80: movss xmm13, dword ptr [rip + 0x202c97b]
0x1408f8f89: lea r14, [rbp + 0xc8]
0x1408f8f90: mov esi, 0xe8
0x1408f8f95: mov rax, qword ptr [rbp + 0x1018]
0x1408f8f9c: xor r15d, r15d
0x1408f8f9f: cmp rax, qword ptr [rbp + 0x1030]
0x1408f8fa6: je 0x1408f94de
0x1408f8fac: nop dword ptr [rax]
0x1408f8fb0: mov r13, qword ptr [rax]
0x1408f8fb3: mov qword ptr [rbp + 0x1020], rax
0x1408f8fba: mov rdi, qword ptr [rax + 8]
0x1408f8fbe: mov byte ptr [rbp + 0x1047], 1
0x1408f8fc5: call 0x140be6750
0x1408f8fca: mov ebx, eax
0x1408f8fcc: mov byte ptr [rbp + 0x1047], 1
0x1408f8fd3: xor ecx, ecx
0x1408f8fd5: call 0x1400d9b20
0x1408f8fda: test rax, rax
0x1408f8fdd: je 0x1408f9708
0x1408f8fe3: mov dword ptr [rbp + 0xd58], ebx
0x1408f8fe9: mov word ptr [rbp + 0xd60], 0
0x1408f8ff2: mov dword ptr [rbp + 0xd5c], 0x3f800000
0x1408f8ffc: mov qword ptr [rbp + 0xcb0], 0
0x1408f9007: mov rcx, qword ptr [rbp + 0xd88]
0x1408f900e: mov qword ptr [rbp + 0xd50], rcx
0x1408f9015: mov qword ptr [rbp + 0xcc0], 0
0x1408f9020: mov qword ptr [rbp + 0xcc8], 4
0x1408f902b: mov qword ptr [rbp + 0xcd0], 0
0x1408f9036: movups xmm0, xmmword ptr [rax]
0x1408f9039: inc qword ptr [rax]
0x1408f903c: lea rax, [rbp + 0xd20]
0x1408f9043: movups xmmword ptr [rax + 0x10], xmm9
0x1408f9048: movups xmmword ptr [rax], xmm10
0x1408f904c: movups xmmword ptr [rbp + 0xd40], xmm0
0x1408f9053: movabs rax, 0x100000003
0x1408f905d: mov qword ptr [rbp + 0xcf0], rax
0x1408f9064: mov qword ptr [rbp + 0xcd8], 0
0x1408f906f: mov qword ptr [rbp + 0xce0], 1
0x1408f907a: mov qword ptr [rbp + 0xce8], 0
0x1408f9085: mov byte ptr [rbp + 0x1047], 1
0x1408f908c: mov rcx, r12
0x1408f908f: lea rdx, [rbp + 0xcb0]
0x1408f9096: lea r8, [rbp + 0x1c0]
0x1408f909d: xor r9d, r9d
0x1408f90a0: call 0x140905be0
0x1408f90a5: mov byte ptr [rbp + 0x103f], 1
0x1408f90ac: mov rcx, qword ptr [rbp + 0x1028]
0x1408f90b3: call 0x140bd5790
0x1408f90b8: mov byte ptr [rbp + 0x103f], 0
0x1408f90bf: lea rcx, [rbp + 0xf00]
0x1408f90c6: mov rdx, r12
0x1408f90c9: movabs r8, 0x531bb9318eea4971
0x1408f90d3: mov r9, rax
0x1408f90d6: call 0x141629d20
0x1408f90db: lea rbx, [r15 + 1]
0x1408f90df: cmp byte ptr [rbp + 0xfde], 8
0x1408f90e6: je 0x1408f912d
0x1408f90e8: movaps xmm3, xmm13
0x1408f90ec: mov rax, qword ptr [rbp + 0x1008]
0x1408f90f3: cmp rbx, qword ptr [rax + 8]
0x1408f90f7: je 0x1408f90fc
0x1408f90f9: xorps xmm3, xmm3
0x1408f90fc: mov rax, qword ptr [rbp + 0x1008]
0x1408f9103: mov rax, qword ptr [rax]
0x1408f9106: test al, 1
0x1408f9108: jne 0x1408f910d
0x1408f910a: xorps xmm3, xmm3
0x1408f910d: mov rdx, qword ptr [rbp + 0xfe8]
0x1408f9114: add rdx, rsi
0x1408f9117: lea rcx, [rbp + 0xfb8]
0x1408f911e: movabs r8, 0x54f25b7b526cbbad
0x1408f9128: call 0x14096ba30
0x1408f912d: mov r8d, 0xf8
0x1408f9133: lea rcx, [rbp + 0xdb0]
0x1408f913a: lea rdx, [rbp + 0xf00]
0x1408f9141: call 0x142923c00
0x1408f9146: cmp byte ptr [rbp + 0xe8e], 8
0x1408f914d: je 0x1408f91aa
0x1408f914f: test rbx, rbx
0x1408f9152: js 0x1408f9160
0x1408f9154: xorps xmm3, xmm3
0x1408f9157: cvtsi2ss xmm3, rbx
0x1408f915c: jmp 0x1408f917a
0x1408f915e: nop 
0x1408f9160: mov rax, rbx
0x1408f9163: shr rax, 1
0x1408f9166: mov ecx, ebx
0x1408f9168: and ecx, 1
0x1408f916b: or rcx, rax
0x1408f916e: xorps xmm3, xmm3
0x1408f9171: cvtsi2ss xmm3, rcx
0x1408f9176: addss xmm3, xmm3
0x1408f917a: mulss xmm3, xmm8
0x1408f917f: addss xmm3, xmm6
0x1408f9183: mov rdx, qword ptr [rbp + 0xe98]
0x1408f918a: add rdx, rsi
0x1408f918d: movss dword ptr [rsp + 0x20], xmm11
0x1408f9194: lea rcx, [rbp + 0xe68]
0x1408f919b: movabs r8, 0x27763b3d9cc2a8a
0x1408f91a5: call 0x14096b940
0x1408f91aa: mov r8d, 0xf8
0x1408f91b0: lea rcx, [rbp + 0xbb0]
0x1408f91b7: lea rdx, [rbp + 0xdb0]
0x1408f91be: call 0x142923c00
0x1408f91c3: cmp byte ptr [rbp + 0xc8e], 8
0x1408f91ca: je 0x1408f91f6
0x1408f91cc: mov rdx, qword ptr [rbp + 0xc98]
0x1408f91d3: add rdx, rsi
0x1408f91d6: movss dword ptr [rsp + 0x20], xmm12
0x1408f91dd: lea rcx, [rbp + 0xc68]
0x1408f91e4: movabs r8, 0x7038b90f28755919
0x1408f91ee: movaps xmm3, xmm7
0x1408f91f1: call 0x14096b940
0x1408f91f6: mov r8d, 0xf8
0x1408f91fc: mov rcx, r12
0x1408f91ff: lea rdx, [rbp + 0xbb0]
0x1408f9206: call 0x142923c00
0x1408f920b: movabs rax, 0xf7a85e8ee9b2d03c
0x1408f9215: mov qword ptr [rbp + 0xdb0], rax
0x1408f921c: mov dword ptr [rbp + 0xf00], 2
0x1408f9226: lea rax, [rbp + 0xf00]
0x1408f922d: mov qword ptr [rsp + 0x20], rax
0x1408f9232: lea rcx, [rbp + 0xdb0]
0x1408f9239: mov rdx, r12
0x1408f923c: movabs r8, 0x7337daaff5b5b2d5
0x1408f9246: mov r9b, 0x1d
0x1408f9249: call 0x140908610
0x1408f924e: mov r8d, 0xf8
0x1408f9254: lea rcx, [rbp + 0x5a8]
0x1408f925b: mov rdx, r12
0x1408f925e: call 0x142923c00
0x1408f9263: cmp byte ptr [rbp + 0x686], 8
0x1408f926a: je 0x1408f928f
0x1408f926c: mov rdx, qword ptr [rbp + 0x690]
0x1408f9273: add rdx, rsi
0x1408f9276: lea rcx, [rbp + 0x660]
0x1408f927d: movabs r8, 0x57ca25b57d60d824
0x1408f9287: mov r9d, ebx
0x1408f928a: call 0x14096b570
0x1408f928f: mov r8d, 0xf8
0x1408f9295: lea rcx, [rbp + 0x6a0]
0x1408f929c: lea rdx, [rbp + 0x5a8]
0x1408f92a3: call 0x142923c00
0x1408f92a8: cmp r15, 3
0x1408f92ac: jb 0x1408f92d0
0x1408f92ae: mov r9b, 1
0x1408f92b1: cmp byte ptr [rip + 0x2d3bad8], 0
0x1408f92b8: je 0x1408f92d0
0x1408f92ba: cmp byte ptr [rbp + 0x77e], 8
0x1408f92c1: lea r15, [rbp + 0xf00]
0x1408f92c8: jne 0x1408f92e9
0x1408f92ca: jmp 0x1408f9309
0x1408f92cc: nop dword ptr [rax]
0x1408f92d0: cmp byte ptr [r13 + 0x18], 0
0x1408f92d5: setne r9b
0x1408f92d9: cmp byte ptr [rbp + 0x77e], 8
0x1408f92e0: lea r15, [rbp + 0xf00]
0x1408f92e7: je 0x1408f9309
0x1408f92e9: mov rdx, qword ptr [rbp + 0x788]
0x1408f92f0: add rdx, rsi
0x1408f92f3: lea rcx, [rbp + 0x758]
0x1408f92fa: movabs r8, 0x10b711313e595441
0x1408f9304: call 0x14096b850
0x1408f9309: mov r8d, 0xf8
0x1408f930f: mov r13, r14
0x1408f9312: mov rcx, r14
0x1408f9315: lea rdx, [rbp + 0x6a0]
0x1408f931c: call 0x142923c00
0x1408f9321: test rdi, rdi
0x1408f9324: je 0x1408f9360
0x1408f9326: mov qword ptr [rbp + 0xbb0], rdi
0x1408f932d: mov dword ptr [rbp + 0x7c0], 2
0x1408f9337: mov qword ptr [rsp + 0x20], r12
0x1408f933c: lea rcx, [rbp + 0xbb0]
0x1408f9343: mov rdx, r13
0x1408f9346: movabs r8, 0xe63d19ff7655ff61
0x1408f9350: mov r9b, 0x1d
0x1408f9353: call 0x1408fd6c0
0x1408f9358: jmp 0x1408f939c
0x1408f935a: nop word ptr [rax + rax]
0x1408f9360: movabs rax, 0x806c29bbf7c5030d
0x1408f936a: mov qword ptr [rbp + 0xbb0], rax
0x1408f9371: mov dword ptr [rbp + 0x7c0], 2
0x1408f937b: mov qword ptr [rsp + 0x20], r12
0x1408f9380: lea rcx, [rbp + 0xbb0]
0x1408f9387: mov rdx, r13
0x1408f938a: movabs r8, 0xe63d19ff7655ff61
0x1408f9394: mov r9b, 0x1d
0x1408f9397: call 0x140908610
0x1408f939c: mov rdi, qword ptr [rbp + 0xc8]
0x1408f93a3: mov r8d, 0xa0
0x1408f93a9: mov rcx, r15
0x1408f93ac: lea rsi, [rbp + 0xd0]
0x1408f93b3: mov rdx, rsi
0x1408f93b6: call 0x142923c00
0x1408f93bb: mov rdx, r15
0x1408f93be: mov r15b, byte ptr [rbp + 0x170]
0x1408f93c5: mov r13w, word ptr [rbp + 0x171]
0x1408f93cd: movzx r12d, byte ptr [rbp + 0x173]
0x1408f93d5: movups xmm0, xmmword ptr [rsi + 0xa4]
0x1408f93dc: movups xmm1, xmmword ptr [rsi + 0xb4]
0x1408f93e3: movups xmm2, xmmword ptr [rsi + 0xc4]
0x1408f93ea: movups xmm3, xmmword ptr [rsi + 0xd4]
0x1408f93f1: movaps xmmword ptr [rbp + 0xdb0], xmm0
0x1408f93f8: movaps xmmword ptr [rbp + 0xdc0], xmm1
0x1408f93ff: movaps xmmword ptr [rbp + 0xdd0], xmm2
0x1408f9406: movaps xmmword ptr [rbp + 0xde0], xmm3
0x1408f940d: movups xmm0, xmmword ptr [rsi + 0xe0]
0x1408f9414: movups xmmword ptr [rbp + 0xdec], xmm0
0x1408f941b: and r15b, 0x7f
0x1408f941f: and r12b, 0x9f
0x1408f9423: or r12b, 0x40
0x1408f9427: mov qword ptr [rbp + 0x7c0], rdi
0x1408f942e: mov r8d, 0xa0
0x1408f9434: lea rcx, [rbp + 0x7c8]
0x1408f943b: call 0x142923c00
0x1408f9440: mov byte ptr [rbp + 0x868], r15b
0x1408f9447: mov word ptr [rbp + 0x869], r13w
0x1408f944f: mov byte ptr [rbp + 0x86b], r12b
0x1408f9456: movaps xmm0, xmmword ptr [rbp + 0xdb0]
0x1408f945d: movaps xmm1, xmmword ptr [rbp + 0xdc0]
0x1408f9464: movaps xmm2, xmmword ptr [rbp + 0xdd0]
0x1408f946b: movaps xmm3, xmmword ptr [rbp + 0xde0]
0x1408f9472: lea rax, [rbp + 0x7c8]
0x1408f9479: movups xmmword ptr [rax + 0xa4], xmm0
0x1408f9480: movups xmmword ptr [rax + 0xb4], xmm1
0x1408f9487: movups xmmword ptr [rax + 0xc4], xmm2
0x1408f948e: movups xmmword ptr [rax + 0xd4], xmm3
0x1408f9495: movups xmm0, xmmword ptr [rbp + 0xdec]
0x1408f949c: movups xmmword ptr [rax + 0xe0], xmm0
0x1408f94a3: mov byte ptr [rbp + 0x1047], 1
0x1408f94aa: mov edx, 0x300
0x1408f94af: lea r12, [rbp + 0x7c0]
0x1408f94b6: mov rcx, r12
0x1408f94b9: call 0x140906a90
0x1408f94be: mov esi, 0xe8
0x1408f94c3: mov rax, qword ptr [rbp + 0x1020]
0x1408f94ca: add rax, 0x10
0x1408f94ce: mov r15, rbx
0x1408f94d1: cmp rax, qword ptr [rbp + 0x1030]
0x1408f94d8: jne 0x1408f8fb0
0x1408f94de: lea rsi, [rbp + 0x7c0]
0x1408f94e5: lea rdx, [rbp + 0x1c0]
0x1408f94ec: mov r8d, 0x3e8
0x1408f94f2: mov rcx, rsi
0x1408f94f5: call 0x142923c00
0x1408f94fa: mov byte ptr [rbp + 0x1047], 0
0x1408f9501: mov rcx, rsi
0x1408f9504: call 0x140868370
0x1408f9509: mov rdx, qword ptr [rbp + 0xd68]
0x1408f9510: test rdx, rdx
0x1408f9513: je 0x1408f952b
0x1408f9515: shl rdx, 4
0x1408f9519: mov r8d, 8
0x1408f951f: mov rcx, qword ptr [rbp + 0x1018]
0x1408f9526: call 0x140613c20
0x1408f952b: mov rdx, qword ptr [rbp + 0xed8]
0x1408f9532: test rdx, rdx
0x1408f9535: je 0x1408f954d
0x1408f9537: mov rcx, qword ptr [rbp + 0xee0]
0x1408f953e: shl rdx, 3
0x1408f9542: mov r8d, 8
0x1408f9548: call 0x140613c20
0x1408f954d: movaps xmm6, xmmword ptr [rbp + 0x1050]
0x1408f9554: movaps xmm7, xmmword ptr [rbp + 0x1060]
0x1408f955b: movaps xmm8, xmmword ptr [rbp + 0x1070]
0x1408f9563: movaps xmm9, xmmword ptr [rbp + 0x1080]
0x1408f956b: movaps xmm10, xmmword ptr [rbp + 0x1090]
0x1408f9573: movaps xmm11, xmmword ptr [rbp + 0x10a0]
0x1408f957b: movaps xmm12, xmmword ptr [rbp + 0x10b0]
0x1408f9583: movaps xmm13, xmmword ptr [rbp + 0x10c0]
0x1408f958b: add rsp, 0x1158
0x1408f9592: pop rbx
0x1408f9593: pop rdi
0x1408f9594: pop rsi
0x1408f9595: pop r12
0x1408f9597: pop r13
0x1408f9599: pop r14
0x1408f959b: pop r15
0x1408f959d: pop rbp
0x1408f959e: ret 
0x1408f959f: cmp dword ptr [rbp + 0x7f0], 1
0x1408f95a6: jne 0x1408f984c
0x1408f95ac: mov r9, qword ptr [rbp + 0x7f8]
0x1408f95b3: cmp rbx, 4
0x1408f95b7: je 0x1408f76fc
0x1408f95bd: cmp dword ptr [rbp + 0x800], 1
0x1408f95c4: jne 0x1408f984c
0x1408f95ca: mov r10, qword ptr [rbp + 0x808]
0x1408f95d1: cmp rbx, 5
0x1408f95d5: je 0x1408f7703
0x1408f95db: cmp dword ptr [rbp + 0x810], 1
0x1408f95e2: jne 0x1408f984c
0x1408f95e8: mov rax, qword ptr [rbp + 0x818]
0x1408f95ef: jmp 0x1408f7703
0x1408f95f4: mov rcx, rdi
0x1408f95f7: mov r8d, 0x3b9aca00
0x1408f95fd: call 0x1428cd210
0x1408f9602: jmp 0x1408f6e11
0x1408f9607: mov rcx, rdi
0x1408f960a: xor edx, edx
0x1408f960c: call 0x1428cdb90
0x1408f9611: jmp 0x1408f6e57
0x1408f9616: lea rax, [rip + 0x21af74b]
0x1408f961d: mov qword ptr [rbp + 0x7c0], rax
0x1408f9624: mov qword ptr [rbp + 0x7c8], 1
0x1408f962f: mov qword ptr [rbp + 0x7d0], 8
0x1408f963a: xorps xmm0, xmm0
0x1408f963d: movups xmmword ptr [rbp + 0x7d8], xmm0
0x1408f9644: lea rdx, [rbp + 0x5d4]
0x1408f964b: lea rax, [rip + 0x21af726]
0x1408f9652: mov qword ptr [rsp + 0x20], rax
0x1408f9657: lea r8, [rip + 0x21af702]
0x1408f965e: lea r9, [rbp + 0x7c0]
0x1408f9665: xor ecx, ecx
0x1408f9667: call 0x1428fbfcf
0x1408f966c: lea rax, [rip + 0x21af6f5]
0x1408f9673: mov qword ptr [rbp + 0x7c0], rax
0x1408f967a: mov qword ptr [rbp + 0x7c8], 1
0x1408f9685: mov qword ptr [rbp + 0x7d0], 8
0x1408f9690: xorps xmm0, xmm0
0x1408f9693: movups xmmword ptr [rbp + 0x7d8], xmm0
0x1408f969a: lea rdx, [rip + 0x21af6ef]
0x1408f96a1: lea rcx, [rbp + 0x7c0]
0x1408f96a8: call 0x1428d9390
0x1408f96ad: lea rcx, [rip + 0x21af4b5]
0x1408f96b4: lea r8, [rip + 0x21af54d]
0x1408f96bb: mov edx, 0x59
0x1408f96c0: call 0x1428d9430
0x1408f96c5: lea r8, [rip + 0x21af6dc]
0x1408f96cc: mov rcx, qword ptr [rbp + 0x1010]
0x1408f96d3: call 0x1428d9850
0x1408f96d8: lea rcx, [rip + 0x21af6f9]
0x1408f96df: lea r8, [rip + 0x21af702]
0x1408f96e6: mov edx, 0x10
0x1408f96eb: call 0x1428d9330
0x1408f96f0: mov byte ptr [rbp + 0x1047], 1
0x1408f96f7: lea rcx, [rip + 0x23413a2]
0x1408f96fe: call 0x1428d80d0
0x1408f9703: jmp 0x1408f984a
0x1408f9708: mov byte ptr [rbp + 0x1047], 1
0x1408f970f: lea rcx, [rip + 0x234138a]
0x1408f9716: call 0x1428d80d0
0x1408f971b: jmp 0x1408f984a
0x1408f9720: lea rcx, [rip + 0x21af4f9]
0x1408f9727: lea r8, [rip + 0x21af502]
0x1408f972e: mov edx, 0xf
0x1408f9733: call 0x1428d9330
0x1408f9738: mov rax, qword ptr [rbp + 0x7c8]
0x1408f973f: movzx ecx, byte ptr [rbp + 0x7d0]
0x1408f9746: mov qword ptr [rbp + 0x1c0], rax
0x1408f974d: mov byte ptr [rbp + 0x1c8], cl
0x1408f9753: lea rax, [rip + 0x21af52e]
0x1408f975a: mov qword ptr [rsp + 0x20], rax
0x1408f975f: lea rcx, [rip + 0x21af53a]
0x1408f9766: lea r9, [rip + 0x21af273]
0x1408f976d: lea r8, [rbp + 0x1c0]
0x1408f9774: mov edx, 0x13
0x1408f9779: call 0x1428d9760
0x1408f977e: jmp 0x1408f984a
0x1408f9783: lea r8, [rip + 0x21af2c6]
0x1408f978a: mov ecx, 1
0x1408f978f: mov edx, 0xf
0x1408f9794: call 0x1428d8fe3
0x1408f9799: jmp 0x1408f984a
0x1408f979e: lea rcx, [rip + 0x21af82b]
0x1408f97a5: call 0x1428d9310
0x1408f97aa: lea rcx, [rip + 0x21af8f7]
0x1408f97b1: call 0x1428d9310
0x1408f97b6: lea rcx, [rip + 0x21af903]
0x1408f97bd: call 0x1428d9310
0x1408f97c2: lea rcx, [rip + 0x21af81f]
0x1408f97c9: call 0x1428d9310
0x1408f97ce: lea rcx, [rip + 0x21af82b]
0x1408f97d5: call 0x1428d9310
0x1408f97da: lea rcx, [rip + 0x21af837]
0x1408f97e1: call 0x1428d9310
0x1408f97e6: lea rcx, [rip + 0x21af843]
0x1408f97ed: call 0x1428d9310
0x1408f97f2: lea rcx, [rip + 0x21af84f]
0x1408f97f9: call 0x1428d9310
0x1408f97fe: lea rcx, [rip + 0x21af85b]
0x1408f9805: call 0x1428d9310
0x1408f980a: lea rcx, [rip + 0x21af867]
0x1408f9811: call 0x1428d9310
0x1408f9816: lea rcx, [rip + 0x21af873]
0x1408f981d: call 0x1428d9310
0x1408f9822: mov byte ptr [rbp + 0x1046], 1
0x1408f9829: lea rcx, [rip + 0x2341270]
0x1408f9830: call 0x1428d80d0
0x1408f9835: jmp 0x1408f984a
0x1408f9837: mov byte ptr [rbp + 0x1047], 1
0x1408f983e: lea rcx, [rip + 0x234125b]
0x1408f9845: call 0x1428d80d0
0x1408f984a: ud2 
0x1408f984c: lea rcx, [rip + 0x21af56d]
0x1408f9853: call 0x1428d9310
0x1408f9858: int3 
0x1408f9859: nop dword ptr [rax]
0x1408f9860: mov qword ptr [rsp + 0x10], rdx
0x1408f9865: push rbp
0x1408f9866: push r15
0x1408f9868: push r14
0x1408f986a: push r13
0x1408f986c: push r12
0x1408f986e: push rsi
0x1408f986f: push rdi
0x1408f9870: push rbx
0x1408f9871: sub rsp, 0xc8
0x1408f9878: lea rbp, [rdx + 0x80]
0x1408f987f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9885: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f988b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9891: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9897: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f98a0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f98a9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f98b1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f98b9: mov rdx, qword ptr [rbp + 0x1c0]
0x1408f98c0: test rdx, rdx
0x1408f98c3: je 0x1408f98d7
0x1408f98c5: mov rcx, qword ptr [rbp + 0x1c8]
0x1408f98cc: mov r8d, 1
0x1408f98d2: call 0x140613c20
0x1408f98d7: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408f98df: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408f98e7: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408f98f0: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408f98f9: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408f98ff: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408f9905: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408f990b: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408f9911: add rsp, 0xc8
0x1408f9918: pop rbx
0x1408f9919: pop rdi
0x1408f991a: pop rsi
0x1408f991b: pop r12
0x1408f991d: pop r13
0x1408f991f: pop r14
0x1408f9921: pop r15
0x1408f9923: pop rbp
0x1408f9924: ret 
0x1408f9925: nop word ptr cs:[rax + rax]
0x1408f9930: mov qword ptr [rsp + 0x10], rdx
0x1408f9935: push rbp
0x1408f9936: push r15
0x1408f9938: push r14
0x1408f993a: push r13
0x1408f993c: push r12
0x1408f993e: push rsi
0x1408f993f: push rdi
0x1408f9940: push rbx
0x1408f9941: sub rsp, 0xc8
0x1408f9948: lea rbp, [rdx + 0x80]
0x1408f994f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9955: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f995b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9961: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9967: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f9970: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f9979: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f9981: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f9989: cmp byte ptr [rbp + 0x1042], 0
0x1408f9990: je 0x1408f999e
0x1408f9992: lea rcx, [rbp + 0x1c0]
0x1408f9999: call 0x1407cea70
0x1408f999e: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408f99a6: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408f99ae: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408f99b7: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408f99c0: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408f99c6: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408f99cc: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408f99d2: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408f99d8: add rsp, 0xc8
0x1408f99df: pop rbx
0x1408f99e0: pop rdi
0x1408f99e1: pop rsi
0x1408f99e2: pop r12
0x1408f99e4: pop r13
0x1408f99e6: pop r14
0x1408f99e8: pop r15
0x1408f99ea: pop rbp
0x1408f99eb: ret 
0x1408f99ec: nop dword ptr [rax]
0x1408f99f0: mov qword ptr [rsp + 0x10], rdx
0x1408f99f5: push rbp
0x1408f99f6: push r15
0x1408f99f8: push r14
0x1408f99fa: push r13
0x1408f99fc: push r12
0x1408f99fe: push rsi
0x1408f99ff: push rdi
0x1408f9a00: push rbx
0x1408f9a01: sub rsp, 0xc8
0x1408f9a08: lea rbp, [rdx + 0x80]
0x1408f9a0f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9a15: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f9a1b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9a21: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9a27: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f9a30: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f9a39: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f9a41: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f9a49: mov byte ptr [rbp + 0x1047], 1
0x1408f9a50: lea rcx, [rbp + 0x7c0]
0x1408f9a57: call 0x1408935c0
0x1408f9a5c: mov byte ptr [rbp + 0x1047], 1
0x1408f9a63: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408f9a6b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408f9a73: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408f9a7c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408f9a85: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408f9a8b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408f9a91: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408f9a97: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408f9a9d: add rsp, 0xc8
0x1408f9aa4: pop rbx
0x1408f9aa5: pop rdi
0x1408f9aa6: pop rsi
0x1408f9aa7: pop r12
0x1408f9aa9: pop r13
0x1408f9aab: pop r14
0x1408f9aad: pop r15
0x1408f9aaf: pop rbp
0x1408f9ab0: ret 
0x1408f9ab1: nop word ptr cs:[rax + rax]
0x1408f9ac0: mov qword ptr [rsp + 0x10], rdx
0x1408f9ac5: push rbp
0x1408f9ac6: push r15
0x1408f9ac8: push r14
0x1408f9aca: push r13
0x1408f9acc: push r12
0x1408f9ace: push rsi
0x1408f9acf: push rdi
0x1408f9ad0: push rbx
0x1408f9ad1: sub rsp, 0xc8
0x1408f9ad8: lea rbp, [rdx + 0x80]
0x1408f9adf: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9ae5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f9aeb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9af1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9af7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f9b00: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f9b09: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f9b11: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f9b19: mov byte ptr [rbp + 0x1047], 1
0x1408f9b20: lea rcx, [rbp + 0xf00]
0x1408f9b27: call 0x1408935c0
0x1408f9b2c: mov byte ptr [rbp + 0x1047], 1
0x1408f9b33: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408f9b3b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408f9b43: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408f9b4c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408f9b55: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408f9b5b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408f9b61: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408f9b67: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408f9b6d: add rsp, 0xc8
0x1408f9b74: pop rbx
0x1408f9b75: pop rdi
0x1408f9b76: pop rsi
0x1408f9b77: pop r12
0x1408f9b79: pop r13
0x1408f9b7b: pop r14
0x1408f9b7d: pop r15
0x1408f9b7f: pop rbp
0x1408f9b80: ret 
0x1408f9b81: nop word ptr cs:[rax + rax]
0x1408f9b90: mov qword ptr [rsp + 0x10], rdx
0x1408f9b95: push rbp
0x1408f9b96: push r15
0x1408f9b98: push r14
0x1408f9b9a: push r13
0x1408f9b9c: push r12
0x1408f9b9e: push rsi
0x1408f9b9f: push rdi
0x1408f9ba0: push rbx
0x1408f9ba1: sub rsp, 0xc8
0x1408f9ba8: lea rbp, [rdx + 0x80]
0x1408f9baf: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9bb5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f9bbb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9bc1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9bc7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f9bd0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f9bd9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f9be1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f9be9: mov byte ptr [rbp + 0x1047], 1
0x1408f9bf0: lea rcx, [rbp + 0x7c0]
0x1408f9bf7: call 0x1408935c0
0x1408f9bfc: mov byte ptr [rbp + 0x1047], 1
0x1408f9c03: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408f9c0b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408f9c13: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408f9c1c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408f9c25: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408f9c2b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408f9c31: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408f9c37: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408f9c3d: add rsp, 0xc8
0x1408f9c44: pop rbx
0x1408f9c45: pop rdi
0x1408f9c46: pop rsi
0x1408f9c47: pop r12
0x1408f9c49: pop r13
0x1408f9c4b: pop r14
0x1408f9c4d: pop r15
0x1408f9c4f: pop rbp
0x1408f9c50: ret 
0x1408f9c51: nop word ptr cs:[rax + rax]
0x1408f9c60: mov qword ptr [rsp + 0x10], rdx
0x1408f9c65: push rbp
0x1408f9c66: push r15
0x1408f9c68: push r14
0x1408f9c6a: push r13
0x1408f9c6c: push r12
0x1408f9c6e: push rsi
0x1408f9c6f: push rdi
0x1408f9c70: push rbx
0x1408f9c71: sub rsp, 0xc8
0x1408f9c78: lea rbp, [rdx + 0x80]
0x1408f9c7f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9c85: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f9c8b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9c91: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9c97: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f9ca0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f9ca9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f9cb1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f9cb9: mov byte ptr [rbp + 0x1047], 1
0x1408f9cc0: lea rcx, [rbp + 0xf00]
0x1408f9cc7: call 0x1408935c0
0x1408f9ccc: mov byte ptr [rbp + 0x1047], 1
0x1408f9cd3: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408f9cdb: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408f9ce3: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408f9cec: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408f9cf5: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408f9cfb: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408f9d01: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408f9d07: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408f9d0d: add rsp, 0xc8
0x1408f9d14: pop rbx
0x1408f9d15: pop rdi
0x1408f9d16: pop rsi
0x1408f9d17: pop r12
0x1408f9d19: pop r13
0x1408f9d1b: pop r14
0x1408f9d1d: pop r15
0x1408f9d1f: pop rbp
0x1408f9d20: ret 
0x1408f9d21: nop word ptr cs:[rax + rax]
0x1408f9d30: mov qword ptr [rsp + 0x10], rdx
0x1408f9d35: push rbp
0x1408f9d36: push r15
0x1408f9d38: push r14
0x1408f9d3a: push r13
0x1408f9d3c: push r12
0x1408f9d3e: push rsi
0x1408f9d3f: push rdi
0x1408f9d40: push rbx
0x1408f9d41: sub rsp, 0xc8
0x1408f9d48: lea rbp, [rdx + 0x80]
0x1408f9d4f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9d55: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f9d5b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9d61: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9d67: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f9d70: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f9d79: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f9d81: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f9d89: mov byte ptr [rbp + 0x1046], 1
0x1408f9d90: lea rcx, [rbp + 0xf00]
0x1408f9d97: call 0x1408935c0
0x1408f9d9c: mov byte ptr [rbp + 0x1046], 1
0x1408f9da3: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408f9dab: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408f9db3: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408f9dbc: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408f9dc5: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408f9dcb: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408f9dd1: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408f9dd7: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408f9ddd: add rsp, 0xc8
0x1408f9de4: pop rbx
0x1408f9de5: pop rdi
0x1408f9de6: pop rsi
0x1408f9de7: pop r12
0x1408f9de9: pop r13
0x1408f9deb: pop r14
0x1408f9ded: pop r15
0x1408f9def: pop rbp
0x1408f9df0: ret 
0x1408f9df1: nop word ptr cs:[rax + rax]
0x1408f9e00: mov qword ptr [rsp + 0x10], rdx
0x1408f9e05: push rbp
0x1408f9e06: push r15
0x1408f9e08: push r14
0x1408f9e0a: push r13
0x1408f9e0c: push r12
0x1408f9e0e: push rsi
0x1408f9e0f: push rdi
0x1408f9e10: push rbx
0x1408f9e11: sub rsp, 0xc8
0x1408f9e18: lea rbp, [rdx + 0x80]
0x1408f9e1f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9e25: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f9e2b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9e31: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9e37: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f9e40: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f9e49: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f9e51: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f9e59: mov rdx, qword ptr [rbp + 0xed8]
0x1408f9e60: test rdx, rdx
0x1408f9e63: je 0x1408f9e77
0x1408f9e65: mov r8d, 1
0x1408f9e6b: mov rcx, qword ptr [rbp + 0x1020]
0x1408f9e72: call 0x140613c20
0x1408f9e77: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408f9e7f: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408f9e87: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408f9e90: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408f9e99: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408f9e9f: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408f9ea5: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408f9eab: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408f9eb1: add rsp, 0xc8
0x1408f9eb8: pop rbx
0x1408f9eb9: pop rdi
0x1408f9eba: pop rsi
0x1408f9ebb: pop r12
0x1408f9ebd: pop r13
0x1408f9ebf: pop r14
0x1408f9ec1: pop r15
0x1408f9ec3: pop rbp
0x1408f9ec4: ret 
0x1408f9ec5: nop word ptr cs:[rax + rax]
0x1408f9ed0: mov qword ptr [rsp + 0x10], rdx
0x1408f9ed5: push rbp
0x1408f9ed6: push r15
0x1408f9ed8: push r14
0x1408f9eda: push r13
0x1408f9edc: push r12
0x1408f9ede: push rsi
0x1408f9edf: push rdi
0x1408f9ee0: push rbx
0x1408f9ee1: sub rsp, 0xc8
0x1408f9ee8: lea rbp, [rdx + 0x80]
0x1408f9eef: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9ef5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f9efb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9f01: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9f07: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f9f10: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f9f19: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f9f21: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f9f29: lea rcx, [rbp + 0xcb0]
0x1408f9f30: call 0x1408f5e40
0x1408f9f35: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408f9f3d: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408f9f45: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408f9f4e: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408f9f57: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408f9f5d: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408f9f63: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408f9f69: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408f9f6f: add rsp, 0xc8
0x1408f9f76: pop rbx
0x1408f9f77: pop rdi
0x1408f9f78: pop rsi
0x1408f9f79: pop r12
0x1408f9f7b: pop r13
0x1408f9f7d: pop r14
0x1408f9f7f: pop r15
0x1408f9f81: pop rbp
0x1408f9f82: ret 
0x1408f9f83: nop word ptr cs:[rax + rax]
0x1408f9f90: mov qword ptr [rsp + 0x10], rdx
0x1408f9f95: push rbp
0x1408f9f96: push r15
0x1408f9f98: push r14
0x1408f9f9a: push r13
0x1408f9f9c: push r12
0x1408f9f9e: push rsi
0x1408f9f9f: push rdi
0x1408f9fa0: push rbx
0x1408f9fa1: sub rsp, 0xc8
0x1408f9fa8: lea rbp, [rdx + 0x80]
0x1408f9faf: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408f9fb5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408f9fbb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408f9fc1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408f9fc7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408f9fd0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408f9fd9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408f9fe1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408f9fe9: mov byte ptr [rbp + 0x1047], 1
0x1408f9ff0: lea rcx, [rbp + 0xf00]
0x1408f9ff7: call 0x1408935c0
0x1408f9ffc: mov byte ptr [rbp + 0x1047], 1
0x1408fa003: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa00b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa013: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa01c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa025: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa02b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa031: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa037: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa03d: add rsp, 0xc8
0x1408fa044: pop rbx
0x1408fa045: pop rdi
0x1408fa046: pop rsi
0x1408fa047: pop r12
0x1408fa049: pop r13
0x1408fa04b: pop r14
0x1408fa04d: pop r15
0x1408fa04f: pop rbp
0x1408fa050: ret 
0x1408fa051: nop word ptr cs:[rax + rax]
0x1408fa060: mov qword ptr [rsp + 0x10], rdx
0x1408fa065: push rbp
0x1408fa066: push r15
0x1408fa068: push r14
0x1408fa06a: push r13
0x1408fa06c: push r12
0x1408fa06e: push rsi
0x1408fa06f: push rdi
0x1408fa070: push rbx
0x1408fa071: sub rsp, 0xc8
0x1408fa078: lea rbp, [rdx + 0x80]
0x1408fa07f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa085: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa08b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa091: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa097: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa0a0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa0a9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa0b1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa0b9: mov byte ptr [rbp + 0x1046], 1
0x1408fa0c0: lea rcx, [rbp + 0xdb0]
0x1408fa0c7: call 0x1408935c0
0x1408fa0cc: mov byte ptr [rbp + 0x1046], 1
0x1408fa0d3: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa0db: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa0e3: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa0ec: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa0f5: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa0fb: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa101: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa107: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa10d: add rsp, 0xc8
0x1408fa114: pop rbx
0x1408fa115: pop rdi
0x1408fa116: pop rsi
0x1408fa117: pop r12
0x1408fa119: pop r13
0x1408fa11b: pop r14
0x1408fa11d: pop r15
0x1408fa11f: pop rbp
0x1408fa120: ret 
0x1408fa121: nop word ptr cs:[rax + rax]
0x1408fa130: mov qword ptr [rsp + 0x10], rdx
0x1408fa135: push rbp
0x1408fa136: push r15
0x1408fa138: push r14
0x1408fa13a: push r13
0x1408fa13c: push r12
0x1408fa13e: push rsi
0x1408fa13f: push rdi
0x1408fa140: push rbx
0x1408fa141: sub rsp, 0xc8
0x1408fa148: lea rbp, [rdx + 0x80]
0x1408fa14f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa155: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa15b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa161: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa167: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa170: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa179: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa181: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa189: cmp byte ptr [rbp + 0x1040], 0
0x1408fa190: je 0x1408fa1f3
0x1408fa192: lea rcx, [rbp + 0x7c0]
0x1408fa199: call 0x1408935c0
0x1408fa19e: mov byte ptr [rbp + 0x1047], 1
0x1408fa1a5: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa1ad: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa1b5: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa1be: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa1c7: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa1cd: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa1d3: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa1d9: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa1df: add rsp, 0xc8
0x1408fa1e6: pop rbx
0x1408fa1e7: pop rdi
0x1408fa1e8: pop rsi
0x1408fa1e9: pop r12
0x1408fa1eb: pop r13
0x1408fa1ed: pop r14
0x1408fa1ef: pop r15
0x1408fa1f1: pop rbp
0x1408fa1f2: ret 
0x1408fa1f3: mov byte ptr [rbp + 0x1047], 1
0x1408fa1fa: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa202: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa20a: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa213: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa21c: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa222: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa228: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa22e: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa234: add rsp, 0xc8
0x1408fa23b: pop rbx
0x1408fa23c: pop rdi
0x1408fa23d: pop rsi
0x1408fa23e: pop r12
0x1408fa240: pop r13
0x1408fa242: pop r14
0x1408fa244: pop r15
0x1408fa246: pop rbp
0x1408fa247: ret 
0x1408fa248: nop dword ptr [rax + rax]
0x1408fa250: mov qword ptr [rsp + 0x10], rdx
0x1408fa255: push rbp
0x1408fa256: push r15
0x1408fa258: push r14
0x1408fa25a: push r13
0x1408fa25c: push r12
0x1408fa25e: push rsi
0x1408fa25f: push rdi
0x1408fa260: push rbx
0x1408fa261: sub rsp, 0xc8
0x1408fa268: lea rbp, [rdx + 0x80]
0x1408fa26f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa275: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa27b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa281: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa287: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa290: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa299: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa2a1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa2a9: cmp byte ptr [rbp + 0x1041], 0
0x1408fa2b0: je 0x1408fa313
0x1408fa2b2: lea rcx, [rbp + 0x7c0]
0x1408fa2b9: call 0x1408935c0
0x1408fa2be: mov byte ptr [rbp + 0x1046], 1
0x1408fa2c5: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa2cd: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa2d5: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa2de: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa2e7: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa2ed: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa2f3: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa2f9: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa2ff: add rsp, 0xc8
0x1408fa306: pop rbx
0x1408fa307: pop rdi
0x1408fa308: pop rsi
0x1408fa309: pop r12
0x1408fa30b: pop r13
0x1408fa30d: pop r14
0x1408fa30f: pop r15
0x1408fa311: pop rbp
0x1408fa312: ret 
0x1408fa313: mov byte ptr [rbp + 0x1046], 1
0x1408fa31a: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa322: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa32a: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa333: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa33c: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa342: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa348: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa34e: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa354: add rsp, 0xc8
0x1408fa35b: pop rbx
0x1408fa35c: pop rdi
0x1408fa35d: pop rsi
0x1408fa35e: pop r12
0x1408fa360: pop r13
0x1408fa362: pop r14
0x1408fa364: pop r15
0x1408fa366: pop rbp
0x1408fa367: ret 
0x1408fa368: nop dword ptr [rax + rax]
0x1408fa370: mov qword ptr [rsp + 0x10], rdx
0x1408fa375: push rbp
0x1408fa376: push r15
0x1408fa378: push r14
0x1408fa37a: push r13
0x1408fa37c: push r12
0x1408fa37e: push rsi
0x1408fa37f: push rdi
0x1408fa380: push rbx
0x1408fa381: sub rsp, 0xc8
0x1408fa388: lea rbp, [rdx + 0x80]
0x1408fa38f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa395: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa39b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa3a1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa3a7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa3b0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa3b9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa3c1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa3c9: cmp byte ptr [rbp + 0x1043], 0
0x1408fa3d0: jne 0x1408fa420
0x1408fa3d2: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa3da: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa3e2: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa3eb: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa3f4: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa3fa: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa400: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa406: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa40c: add rsp, 0xc8
0x1408fa413: pop rbx
0x1408fa414: pop rdi
0x1408fa415: pop rsi
0x1408fa416: pop r12
0x1408fa418: pop r13
0x1408fa41a: pop r14
0x1408fa41c: pop r15
0x1408fa41e: pop rbp
0x1408fa41f: ret 
0x1408fa420: lea rcx, [rbp + 0x1c0]
0x1408fa427: call 0x1409a3750
0x1408fa42c: jmp 0x1408fa3d2
0x1408fa42e: nop 
0x1408fa430: mov qword ptr [rsp + 0x10], rdx
0x1408fa435: push rbp
0x1408fa436: push r15
0x1408fa438: push r14
0x1408fa43a: push r13
0x1408fa43c: push r12
0x1408fa43e: push rsi
0x1408fa43f: push rdi
0x1408fa440: push rbx
0x1408fa441: sub rsp, 0xc8
0x1408fa448: lea rbp, [rdx + 0x80]
0x1408fa44f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa455: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa45b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa461: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa467: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa470: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa479: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa481: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa489: cmp byte ptr [rbp + 0x1044], 0
0x1408fa490: jne 0x1408fa4e0
0x1408fa492: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa49a: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa4a2: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa4ab: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa4b4: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa4ba: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa4c0: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa4c6: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa4cc: add rsp, 0xc8
0x1408fa4d3: pop rbx
0x1408fa4d4: pop rdi
0x1408fa4d5: pop rsi
0x1408fa4d6: pop r12
0x1408fa4d8: pop r13
0x1408fa4da: pop r14
0x1408fa4dc: pop r15
0x1408fa4de: pop rbp
0x1408fa4df: ret 
0x1408fa4e0: lea rcx, [rbp + 0xbb0]
0x1408fa4e7: call 0x1405e7a00
0x1408fa4ec: jmp 0x1408fa492
0x1408fa4ee: nop 
0x1408fa4f0: mov qword ptr [rsp + 0x10], rdx
0x1408fa4f5: push rbp
0x1408fa4f6: push r15
0x1408fa4f8: push r14
0x1408fa4fa: push r13
0x1408fa4fc: push r12
0x1408fa4fe: push rsi
0x1408fa4ff: push rdi
0x1408fa500: push rbx
0x1408fa501: sub rsp, 0xc8
0x1408fa508: lea rbp, [rdx + 0x80]
0x1408fa50f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa515: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa51b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa521: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa527: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa530: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa539: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa541: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa549: mov byte ptr [rbp + 0x1047], 1
0x1408fa550: lea rcx, [rbp + 0xc8]
0x1408fa557: call 0x1408935c0
0x1408fa55c: mov byte ptr [rbp + 0x1047], 1
0x1408fa563: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa56b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa573: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa57c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa585: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa58b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa591: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa597: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa59d: add rsp, 0xc8
0x1408fa5a4: pop rbx
0x1408fa5a5: pop rdi
0x1408fa5a6: pop rsi
0x1408fa5a7: pop r12
0x1408fa5a9: pop r13
0x1408fa5ab: pop r14
0x1408fa5ad: pop r15
0x1408fa5af: pop rbp
0x1408fa5b0: ret 
0x1408fa5b1: nop word ptr cs:[rax + rax]
0x1408fa5c0: mov qword ptr [rsp + 0x10], rdx
0x1408fa5c5: push rbp
0x1408fa5c6: push r15
0x1408fa5c8: push r14
0x1408fa5ca: push r13
0x1408fa5cc: push r12
0x1408fa5ce: push rsi
0x1408fa5cf: push rdi
0x1408fa5d0: push rbx
0x1408fa5d1: sub rsp, 0xc8
0x1408fa5d8: lea rbp, [rdx + 0x80]
0x1408fa5df: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa5e5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa5eb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa5f1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa5f7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa600: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa609: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa611: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa619: mov byte ptr [rbp + 0x1047], 1
0x1408fa620: lea rcx, [rbp - 0x30]
0x1408fa624: call 0x1408935c0
0x1408fa629: mov byte ptr [rbp + 0x1047], 1
0x1408fa630: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa638: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa640: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa649: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa652: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa658: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa65e: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa664: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa66a: add rsp, 0xc8
0x1408fa671: pop rbx
0x1408fa672: pop rdi
0x1408fa673: pop rsi
0x1408fa674: pop r12
0x1408fa676: pop r13
0x1408fa678: pop r14
0x1408fa67a: pop r15
0x1408fa67c: pop rbp
0x1408fa67d: ret 
0x1408fa67e: nop 
0x1408fa680: mov qword ptr [rsp + 0x10], rdx
0x1408fa685: push rbp
0x1408fa686: push r15
0x1408fa688: push r14
0x1408fa68a: push r13
0x1408fa68c: push r12
0x1408fa68e: push rsi
0x1408fa68f: push rdi
0x1408fa690: push rbx
0x1408fa691: sub rsp, 0xc8
0x1408fa698: lea rbp, [rdx + 0x80]
0x1408fa69f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa6a5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa6ab: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa6b1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa6b7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa6c0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa6c9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa6d1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa6d9: lea rcx, [rbp + 0x1c0]
0x1408fa6e0: call 0x1405d2870
0x1408fa6e5: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa6ed: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa6f5: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa6fe: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa707: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa70d: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa713: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa719: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa71f: add rsp, 0xc8
0x1408fa726: pop rbx
0x1408fa727: pop rdi
0x1408fa728: pop rsi
0x1408fa729: pop r12
0x1408fa72b: pop r13
0x1408fa72d: pop r14
0x1408fa72f: pop r15
0x1408fa731: pop rbp
0x1408fa732: ret 
0x1408fa733: nop word ptr cs:[rax + rax]
0x1408fa740: mov qword ptr [rsp + 0x10], rdx
0x1408fa745: push rbp
0x1408fa746: push r15
0x1408fa748: push r14
0x1408fa74a: push r13
0x1408fa74c: push r12
0x1408fa74e: push rsi
0x1408fa74f: push rdi
0x1408fa750: push rbx
0x1408fa751: sub rsp, 0xc8
0x1408fa758: lea rbp, [rdx + 0x80]
0x1408fa75f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa765: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa76b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa771: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa777: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa780: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa789: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa791: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa799: mov rdx, qword ptr [rbp + 0x7c0]
0x1408fa7a0: test rdx, rdx
0x1408fa7a3: je 0x1408fa7b7
0x1408fa7a5: mov rcx, qword ptr [rbp + 0x7c8]
0x1408fa7ac: mov r8d, 1
0x1408fa7b2: call 0x140613c20
0x1408fa7b7: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa7bf: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa7c7: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa7d0: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa7d9: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa7df: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa7e5: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa7eb: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa7f1: add rsp, 0xc8
0x1408fa7f8: pop rbx
0x1408fa7f9: pop rdi
0x1408fa7fa: pop rsi
0x1408fa7fb: pop r12
0x1408fa7fd: pop r13
0x1408fa7ff: pop r14
0x1408fa801: pop r15
0x1408fa803: pop rbp
0x1408fa804: ret 
0x1408fa805: nop word ptr cs:[rax + rax]
0x1408fa810: mov qword ptr [rsp + 0x10], rdx
0x1408fa815: push rbp
0x1408fa816: push r15
0x1408fa818: push r14
0x1408fa81a: push r13
0x1408fa81c: push r12
0x1408fa81e: push rsi
0x1408fa81f: push rdi
0x1408fa820: push rbx
0x1408fa821: sub rsp, 0xc8
0x1408fa828: lea rbp, [rdx + 0x80]
0x1408fa82f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa835: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa83b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa841: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa847: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa850: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa859: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa861: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa869: mov rdx, qword ptr [rbp + 0xf00]
0x1408fa870: test rdx, rdx
0x1408fa873: je 0x1408fa887
0x1408fa875: mov rcx, qword ptr [rbp + 0xf08]
0x1408fa87c: mov r8d, 1
0x1408fa882: call 0x140613c20
0x1408fa887: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa88f: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa897: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa8a0: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa8a9: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa8af: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa8b5: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa8bb: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa8c1: add rsp, 0xc8
0x1408fa8c8: pop rbx
0x1408fa8c9: pop rdi
0x1408fa8ca: pop rsi
0x1408fa8cb: pop r12
0x1408fa8cd: pop r13
0x1408fa8cf: pop r14
0x1408fa8d1: pop r15
0x1408fa8d3: pop rbp
0x1408fa8d4: ret 
0x1408fa8d5: nop word ptr cs:[rax + rax]
0x1408fa8e0: mov qword ptr [rsp + 0x10], rdx
0x1408fa8e5: push rbp
0x1408fa8e6: push r15
0x1408fa8e8: push r14
0x1408fa8ea: push r13
0x1408fa8ec: push r12
0x1408fa8ee: push rsi
0x1408fa8ef: push rdi
0x1408fa8f0: push rbx
0x1408fa8f1: sub rsp, 0xc8
0x1408fa8f8: lea rbp, [rdx + 0x80]
0x1408fa8ff: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa905: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa90b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa911: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa917: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa920: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa929: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fa931: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fa939: mov rdx, qword ptr [rbp + 0x1c0]
0x1408fa940: test rdx, rdx
0x1408fa943: je 0x1408fa957
0x1408fa945: mov rcx, qword ptr [rbp + 0x1c8]
0x1408fa94c: mov r8d, 1
0x1408fa952: call 0x140613c20
0x1408fa957: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fa95f: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fa967: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fa970: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fa979: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fa97f: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fa985: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fa98b: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fa991: add rsp, 0xc8
0x1408fa998: pop rbx
0x1408fa999: pop rdi
0x1408fa99a: pop rsi
0x1408fa99b: pop r12
0x1408fa99d: pop r13
0x1408fa99f: pop r14
0x1408fa9a1: pop r15
0x1408fa9a3: pop rbp
0x1408fa9a4: ret 
0x1408fa9a5: nop word ptr cs:[rax + rax]
0x1408fa9b0: mov qword ptr [rsp + 0x10], rdx
0x1408fa9b5: push rbp
0x1408fa9b6: push r15
0x1408fa9b8: push r14
0x1408fa9ba: push r13
0x1408fa9bc: push r12
0x1408fa9be: push rsi
0x1408fa9bf: push rdi
0x1408fa9c0: push rbx
0x1408fa9c1: sub rsp, 0xc8
0x1408fa9c8: lea rbp, [rdx + 0x80]
0x1408fa9cf: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fa9d5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fa9db: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fa9e1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fa9e7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fa9f0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fa9f9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408faa01: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408faa09: mov byte ptr [rbp + 0x1047], 1
0x1408faa10: lea rcx, [rbp + 0x5a8]
0x1408faa17: call 0x1408935c0
0x1408faa1c: mov byte ptr [rbp + 0x1047], 1
0x1408faa23: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408faa2b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408faa33: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408faa3c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408faa45: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408faa4b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408faa51: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408faa57: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408faa5d: add rsp, 0xc8
0x1408faa64: pop rbx
0x1408faa65: pop rdi
0x1408faa66: pop rsi
0x1408faa67: pop r12
0x1408faa69: pop r13
0x1408faa6b: pop r14
0x1408faa6d: pop r15
0x1408faa6f: pop rbp
0x1408faa70: ret 
0x1408faa71: nop word ptr cs:[rax + rax]
0x1408faa80: mov qword ptr [rsp + 0x10], rdx
0x1408faa85: push rbp
0x1408faa86: push r15
0x1408faa88: push r14
0x1408faa8a: push r13
0x1408faa8c: push r12
0x1408faa8e: push rsi
0x1408faa8f: push rdi
0x1408faa90: push rbx
0x1408faa91: sub rsp, 0xc8
0x1408faa98: lea rbp, [rdx + 0x80]
0x1408faa9f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408faaa5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408faaab: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408faab1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408faab7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408faac0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408faac9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408faad1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408faad9: mov byte ptr [rbp + 0x1047], 1
0x1408faae0: lea rcx, [rbp + 0xf00]
0x1408faae7: call 0x1408935c0
0x1408faaec: mov byte ptr [rbp + 0x1047], 1
0x1408faaf3: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408faafb: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fab03: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fab0c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fab15: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fab1b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fab21: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fab27: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fab2d: add rsp, 0xc8
0x1408fab34: pop rbx
0x1408fab35: pop rdi
0x1408fab36: pop rsi
0x1408fab37: pop r12
0x1408fab39: pop r13
0x1408fab3b: pop r14
0x1408fab3d: pop r15
0x1408fab3f: pop rbp
0x1408fab40: ret 
0x1408fab41: nop word ptr cs:[rax + rax]
0x1408fab50: mov qword ptr [rsp + 0x10], rdx
0x1408fab55: push rbp
0x1408fab56: push r15
0x1408fab58: push r14
0x1408fab5a: push r13
0x1408fab5c: push r12
0x1408fab5e: push rsi
0x1408fab5f: push rdi
0x1408fab60: push rbx
0x1408fab61: sub rsp, 0xc8
0x1408fab68: lea rbp, [rdx + 0x80]
0x1408fab6f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fab75: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fab7b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fab81: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fab87: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fab90: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fab99: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408faba1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408faba9: mov byte ptr [rbp + 0x1047], 1
0x1408fabb0: lea rcx, [rbp + 0xbb0]
0x1408fabb7: call 0x1408935c0
0x1408fabbc: mov byte ptr [rbp + 0x1047], 1
0x1408fabc3: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fabcb: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fabd3: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fabdc: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fabe5: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fabeb: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fabf1: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fabf7: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fabfd: add rsp, 0xc8
0x1408fac04: pop rbx
0x1408fac05: pop rdi
0x1408fac06: pop rsi
0x1408fac07: pop r12
0x1408fac09: pop r13
0x1408fac0b: pop r14
0x1408fac0d: pop r15
0x1408fac0f: pop rbp
0x1408fac10: ret 
0x1408fac11: nop word ptr cs:[rax + rax]
0x1408fac20: mov qword ptr [rsp + 0x10], rdx
0x1408fac25: push rbp
0x1408fac26: push r15
0x1408fac28: push r14
0x1408fac2a: push r13
0x1408fac2c: push r12
0x1408fac2e: push rsi
0x1408fac2f: push rdi
0x1408fac30: push rbx
0x1408fac31: sub rsp, 0xc8
0x1408fac38: lea rbp, [rdx + 0x80]
0x1408fac3f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fac45: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fac4b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fac51: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fac57: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fac60: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fac69: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fac71: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fac79: mov byte ptr [rbp + 0x1047], 1
0x1408fac80: lea rcx, [rbp + 0x6a0]
0x1408fac87: call 0x1408935c0
0x1408fac8c: mov byte ptr [rbp + 0x1047], 1
0x1408fac93: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fac9b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408faca3: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408facac: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408facb5: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408facbb: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408facc1: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408facc7: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408faccd: add rsp, 0xc8
0x1408facd4: pop rbx
0x1408facd5: pop rdi
0x1408facd6: pop rsi
0x1408facd7: pop r12
0x1408facd9: pop r13
0x1408facdb: pop r14
0x1408facdd: pop r15
0x1408facdf: pop rbp
0x1408face0: ret 
0x1408face1: nop word ptr cs:[rax + rax]
0x1408facf0: mov qword ptr [rsp + 0x10], rdx
0x1408facf5: push rbp
0x1408facf6: push r15
0x1408facf8: push r14
0x1408facfa: push r13
0x1408facfc: push r12
0x1408facfe: push rsi
0x1408facff: push rdi
0x1408fad00: push rbx
0x1408fad01: sub rsp, 0xc8
0x1408fad08: lea rbp, [rdx + 0x80]
0x1408fad0f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fad15: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fad1b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fad21: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fad27: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fad30: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fad39: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fad41: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fad49: mov byte ptr [rbp + 0x1047], 1
0x1408fad50: lea rcx, [rbp + 0xdb0]
0x1408fad57: call 0x1408935c0
0x1408fad5c: mov byte ptr [rbp + 0x1047], 1
0x1408fad63: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fad6b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fad73: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fad7c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fad85: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fad8b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fad91: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fad97: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fad9d: add rsp, 0xc8
0x1408fada4: pop rbx
0x1408fada5: pop rdi
0x1408fada6: pop rsi
0x1408fada7: pop r12
0x1408fada9: pop r13
0x1408fadab: pop r14
0x1408fadad: pop r15
0x1408fadaf: pop rbp
0x1408fadb0: ret 
0x1408fadb1: nop word ptr cs:[rax + rax]
0x1408fadc0: mov qword ptr [rsp + 0x10], rdx
0x1408fadc5: push rbp
0x1408fadc6: push r15
0x1408fadc8: push r14
0x1408fadca: push r13
0x1408fadcc: push r12
0x1408fadce: push rsi
0x1408fadcf: push rdi
0x1408fadd0: push rbx
0x1408fadd1: sub rsp, 0xc8
0x1408fadd8: lea rbp, [rdx + 0x80]
0x1408faddf: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fade5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fadeb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fadf1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fadf7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fae00: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fae09: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fae11: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fae19: mov byte ptr [rbp + 0x1047], 1
0x1408fae20: lea rcx, [rbp + 0xdb0]
0x1408fae27: call 0x1408935c0
0x1408fae2c: mov byte ptr [rbp + 0x1047], 1
0x1408fae33: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fae3b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fae43: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fae4c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fae55: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fae5b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fae61: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fae67: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fae6d: add rsp, 0xc8
0x1408fae74: pop rbx
0x1408fae75: pop rdi
0x1408fae76: pop rsi
0x1408fae77: pop r12
0x1408fae79: pop r13
0x1408fae7b: pop r14
0x1408fae7d: pop r15
0x1408fae7f: pop rbp
0x1408fae80: ret 
0x1408fae81: nop word ptr cs:[rax + rax]
0x1408fae90: mov qword ptr [rsp + 0x10], rdx
0x1408fae95: push rbp
0x1408fae96: push r15
0x1408fae98: push r14
0x1408fae9a: push r13
0x1408fae9c: push r12
0x1408fae9e: push rsi
0x1408fae9f: push rdi
0x1408faea0: push rbx
0x1408faea1: sub rsp, 0xc8
0x1408faea8: lea rbp, [rdx + 0x80]
0x1408faeaf: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408faeb5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408faebb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408faec1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408faec7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408faed0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408faed9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408faee1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408faee9: mov byte ptr [rbp + 0x1047], 1
0x1408faef0: lea rcx, [rbp + 0x5a8]
0x1408faef7: call 0x1408935c0
0x1408faefc: mov byte ptr [rbp + 0x1047], 1
0x1408faf03: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408faf0b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408faf13: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408faf1c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408faf25: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408faf2b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408faf31: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408faf37: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408faf3d: add rsp, 0xc8
0x1408faf44: pop rbx
0x1408faf45: pop rdi
0x1408faf46: pop rsi
0x1408faf47: pop r12
0x1408faf49: pop r13
0x1408faf4b: pop r14
0x1408faf4d: pop r15
0x1408faf4f: pop rbp
0x1408faf50: ret 
0x1408faf51: nop word ptr cs:[rax + rax]
0x1408faf60: mov qword ptr [rsp + 0x10], rdx
0x1408faf65: push rbp
0x1408faf66: push r15
0x1408faf68: push r14
0x1408faf6a: push r13
0x1408faf6c: push r12
0x1408faf6e: push rsi
0x1408faf6f: push rdi
0x1408faf70: push rbx
0x1408faf71: sub rsp, 0xc8
0x1408faf78: lea rbp, [rdx + 0x80]
0x1408faf7f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408faf85: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408faf8b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408faf91: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408faf97: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fafa0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fafa9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fafb1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fafb9: mov byte ptr [rbp + 0x1047], 1
0x1408fafc0: lea rcx, [rbp + 0xf00]
0x1408fafc7: call 0x1408935c0
0x1408fafcc: mov byte ptr [rbp + 0x1047], 1
0x1408fafd3: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fafdb: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fafe3: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fafec: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408faff5: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408faffb: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb001: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb007: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb00d: add rsp, 0xc8
0x1408fb014: pop rbx
0x1408fb015: pop rdi
0x1408fb016: pop rsi
0x1408fb017: pop r12
0x1408fb019: pop r13
0x1408fb01b: pop r14
0x1408fb01d: pop r15
0x1408fb01f: pop rbp
0x1408fb020: ret 
0x1408fb021: nop word ptr cs:[rax + rax]
0x1408fb030: mov qword ptr [rsp + 0x10], rdx
0x1408fb035: push rbp
0x1408fb036: push r15
0x1408fb038: push r14
0x1408fb03a: push r13
0x1408fb03c: push r12
0x1408fb03e: push rsi
0x1408fb03f: push rdi
0x1408fb040: push rbx
0x1408fb041: sub rsp, 0xc8
0x1408fb048: lea rbp, [rdx + 0x80]
0x1408fb04f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb055: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb05b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb061: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb067: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb070: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb079: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb081: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb089: mov byte ptr [rbp + 0x1047], 1
0x1408fb090: lea rcx, [rbp + 0xbb0]
0x1408fb097: call 0x1408935c0
0x1408fb09c: mov byte ptr [rbp + 0x1047], 1
0x1408fb0a3: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb0ab: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb0b3: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb0bc: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb0c5: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb0cb: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb0d1: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb0d7: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb0dd: add rsp, 0xc8
0x1408fb0e4: pop rbx
0x1408fb0e5: pop rdi
0x1408fb0e6: pop rsi
0x1408fb0e7: pop r12
0x1408fb0e9: pop r13
0x1408fb0eb: pop r14
0x1408fb0ed: pop r15
0x1408fb0ef: pop rbp
0x1408fb0f0: ret 
0x1408fb0f1: nop word ptr cs:[rax + rax]
0x1408fb100: mov qword ptr [rsp + 0x10], rdx
0x1408fb105: push rbp
0x1408fb106: push r15
0x1408fb108: push r14
0x1408fb10a: push r13
0x1408fb10c: push r12
0x1408fb10e: push rsi
0x1408fb10f: push rdi
0x1408fb110: push rbx
0x1408fb111: sub rsp, 0xc8
0x1408fb118: lea rbp, [rdx + 0x80]
0x1408fb11f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb125: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb12b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb131: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb137: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb140: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb149: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb151: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb159: mov byte ptr [rbp + 0x1047], 1
0x1408fb160: lea rcx, [rbp + 0x6a0]
0x1408fb167: call 0x1408935c0
0x1408fb16c: mov byte ptr [rbp + 0x1047], 1
0x1408fb173: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb17b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb183: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb18c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb195: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb19b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb1a1: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb1a7: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb1ad: add rsp, 0xc8
0x1408fb1b4: pop rbx
0x1408fb1b5: pop rdi
0x1408fb1b6: pop rsi
0x1408fb1b7: pop r12
0x1408fb1b9: pop r13
0x1408fb1bb: pop r14
0x1408fb1bd: pop r15
0x1408fb1bf: pop rbp
0x1408fb1c0: ret 
0x1408fb1c1: nop word ptr cs:[rax + rax]
0x1408fb1d0: mov qword ptr [rsp + 0x10], rdx
0x1408fb1d5: push rbp
0x1408fb1d6: push r15
0x1408fb1d8: push r14
0x1408fb1da: push r13
0x1408fb1dc: push r12
0x1408fb1de: push rsi
0x1408fb1df: push rdi
0x1408fb1e0: push rbx
0x1408fb1e1: sub rsp, 0xc8
0x1408fb1e8: lea rbp, [rdx + 0x80]
0x1408fb1ef: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb1f5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb1fb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb201: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb207: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb210: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb219: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb221: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb229: cmp byte ptr [rbp + 0x1045], 0
0x1408fb230: jne 0x1408fb280
0x1408fb232: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb23a: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb242: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb24b: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb254: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb25a: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb260: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb266: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb26c: add rsp, 0xc8
0x1408fb273: pop rbx
0x1408fb274: pop rdi
0x1408fb275: pop rsi
0x1408fb276: pop r12
0x1408fb278: pop r13
0x1408fb27a: pop r14
0x1408fb27c: pop r15
0x1408fb27e: pop rbp
0x1408fb27f: ret 
0x1408fb280: lea rcx, [rbp + 0xbb0]
0x1408fb287: call 0x1405e7a00
0x1408fb28c: jmp 0x1408fb232
0x1408fb28e: nop 
0x1408fb290: mov qword ptr [rsp + 0x10], rdx
0x1408fb295: push rbp
0x1408fb296: push r15
0x1408fb298: push r14
0x1408fb29a: push r13
0x1408fb29c: push r12
0x1408fb29e: push rsi
0x1408fb29f: push rdi
0x1408fb2a0: push rbx
0x1408fb2a1: sub rsp, 0xc8
0x1408fb2a8: lea rbp, [rdx + 0x80]
0x1408fb2af: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb2b5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb2bb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb2c1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb2c7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb2d0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb2d9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb2e1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb2e9: mov byte ptr [rbp + 0x1047], 1
0x1408fb2f0: lea rcx, [rbp + 0xc8]
0x1408fb2f7: call 0x1408935c0
0x1408fb2fc: mov byte ptr [rbp + 0x1047], 1
0x1408fb303: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb30b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb313: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb31c: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb325: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb32b: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb331: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb337: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb33d: add rsp, 0xc8
0x1408fb344: pop rbx
0x1408fb345: pop rdi
0x1408fb346: pop rsi
0x1408fb347: pop r12
0x1408fb349: pop r13
0x1408fb34b: pop r14
0x1408fb34d: pop r15
0x1408fb34f: pop rbp
0x1408fb350: ret 
0x1408fb351: nop word ptr cs:[rax + rax]
0x1408fb360: mov qword ptr [rsp + 0x10], rdx
0x1408fb365: push rbp
0x1408fb366: push r15
0x1408fb368: push r14
0x1408fb36a: push r13
0x1408fb36c: push r12
0x1408fb36e: push rsi
0x1408fb36f: push rdi
0x1408fb370: push rbx
0x1408fb371: sub rsp, 0xc8
0x1408fb378: lea rbp, [rdx + 0x80]
0x1408fb37f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb385: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb38b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb391: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb397: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb3a0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb3a9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb3b1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb3b9: mov byte ptr [rbp + 0x1047], 1
0x1408fb3c0: lea rcx, [rbp - 0x30]
0x1408fb3c4: call 0x1408935c0
0x1408fb3c9: mov byte ptr [rbp + 0x1047], 1
0x1408fb3d0: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb3d8: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb3e0: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb3e9: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb3f2: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb3f8: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb3fe: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb404: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb40a: add rsp, 0xc8
0x1408fb411: pop rbx
0x1408fb412: pop rdi
0x1408fb413: pop rsi
0x1408fb414: pop r12
0x1408fb416: pop r13
0x1408fb418: pop r14
0x1408fb41a: pop r15
0x1408fb41c: pop rbp
0x1408fb41d: ret 
0x1408fb41e: nop 
0x1408fb420: mov qword ptr [rsp + 0x10], rdx
0x1408fb425: push rbp
0x1408fb426: push r15
0x1408fb428: push r14
0x1408fb42a: push r13
0x1408fb42c: push r12
0x1408fb42e: push rsi
0x1408fb42f: push rdi
0x1408fb430: push rbx
0x1408fb431: sub rsp, 0xc8
0x1408fb438: lea rbp, [rdx + 0x80]
0x1408fb43f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb445: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb44b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb451: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb457: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb460: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb469: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb471: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb479: mov byte ptr [rbp + 0x1047], 1
0x1408fb480: lea rcx, [rbp + 0x7c0]
0x1408fb487: call 0x1408935c0
0x1408fb48c: mov byte ptr [rbp + 0x1047], 1
0x1408fb493: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb49b: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb4a3: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb4ac: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb4b5: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb4bb: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb4c1: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb4c7: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb4cd: add rsp, 0xc8
0x1408fb4d4: pop rbx
0x1408fb4d5: pop rdi
0x1408fb4d6: pop rsi
0x1408fb4d7: pop r12
0x1408fb4d9: pop r13
0x1408fb4db: pop r14
0x1408fb4dd: pop r15
0x1408fb4df: pop rbp
0x1408fb4e0: ret 
0x1408fb4e1: nop word ptr cs:[rax + rax]
0x1408fb4f0: mov qword ptr [rsp + 0x10], rdx
0x1408fb4f5: push rbp
0x1408fb4f6: push r15
0x1408fb4f8: push r14
0x1408fb4fa: push r13
0x1408fb4fc: push r12
0x1408fb4fe: push rsi
0x1408fb4ff: push rdi
0x1408fb500: push rbx
0x1408fb501: sub rsp, 0xc8
0x1408fb508: lea rbp, [rdx + 0x80]
0x1408fb50f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb515: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb51b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb521: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb527: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb530: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb539: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb541: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb549: mov rdx, qword ptr [rbp + 0x1c0]
0x1408fb550: test rdx, rdx
0x1408fb553: je 0x1408fb567
0x1408fb555: mov rcx, qword ptr [rbp + 0x1c8]
0x1408fb55c: mov r8d, 1
0x1408fb562: call 0x140613c20
0x1408fb567: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb56f: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb577: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb580: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb589: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb58f: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb595: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb59b: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb5a1: add rsp, 0xc8
0x1408fb5a8: pop rbx
0x1408fb5a9: pop rdi
0x1408fb5aa: pop rsi
0x1408fb5ab: pop r12
0x1408fb5ad: pop r13
0x1408fb5af: pop r14
0x1408fb5b1: pop r15
0x1408fb5b3: pop rbp
0x1408fb5b4: ret 
0x1408fb5b5: nop word ptr cs:[rax + rax]
0x1408fb5c0: mov qword ptr [rsp + 0x10], rdx
0x1408fb5c5: push rbp
0x1408fb5c6: push r15
0x1408fb5c8: push r14
0x1408fb5ca: push r13
0x1408fb5cc: push r12
0x1408fb5ce: push rsi
0x1408fb5cf: push rdi
0x1408fb5d0: push rbx
0x1408fb5d1: sub rsp, 0xc8
0x1408fb5d8: lea rbp, [rdx + 0x80]
0x1408fb5df: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb5e5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb5eb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb5f1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb5f7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb600: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb609: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb611: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb619: mov rcx, qword ptr [rbp + 0x1020]
0x1408fb620: movzx edx, byte ptr [rbp + 0x1018]
0x1408fb627: call 0x140043000
0x1408fb62c: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb634: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb63c: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb645: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb64e: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb654: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb65a: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb660: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb666: add rsp, 0xc8
0x1408fb66d: pop rbx
0x1408fb66e: pop rdi
0x1408fb66f: pop rsi
0x1408fb670: pop r12
0x1408fb672: pop r13
0x1408fb674: pop r14
0x1408fb676: pop r15
0x1408fb678: pop rbp
0x1408fb679: ret 
0x1408fb67a: nop word ptr [rax + rax]
0x1408fb680: mov qword ptr [rsp + 0x10], rdx
0x1408fb685: push rbp
0x1408fb686: push r15
0x1408fb688: push r14
0x1408fb68a: push r13
0x1408fb68c: push r12
0x1408fb68e: push rsi
0x1408fb68f: push rdi
0x1408fb690: push rbx
0x1408fb691: sub rsp, 0xc8
0x1408fb698: lea rbp, [rdx + 0x80]
0x1408fb69f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb6a5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb6ab: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb6b1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb6b7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb6c0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb6c9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb6d1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb6d9: cmp byte ptr [rbp + 0x103f], 0
0x1408fb6e0: je 0x1408fb743
0x1408fb6e2: lea rcx, [rbp + 0x7c0]
0x1408fb6e9: call 0x1408935c0
0x1408fb6ee: mov byte ptr [rbp + 0x1047], 1
0x1408fb6f5: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb6fd: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb705: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb70e: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb717: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb71d: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb723: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb729: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb72f: add rsp, 0xc8
0x1408fb736: pop rbx
0x1408fb737: pop rdi
0x1408fb738: pop rsi
0x1408fb739: pop r12
0x1408fb73b: pop r13
0x1408fb73d: pop r14
0x1408fb73f: pop r15
0x1408fb741: pop rbp
0x1408fb742: ret 
0x1408fb743: mov byte ptr [rbp + 0x1047], 1
0x1408fb74a: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb752: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb75a: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb763: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb76c: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb772: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb778: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb77e: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb784: add rsp, 0xc8
0x1408fb78b: pop rbx
0x1408fb78c: pop rdi
0x1408fb78d: pop rsi
0x1408fb78e: pop r12
0x1408fb790: pop r13
0x1408fb792: pop r14
0x1408fb794: pop r15
0x1408fb796: pop rbp
0x1408fb797: ret 
0x1408fb798: nop dword ptr [rax + rax]
0x1408fb7a0: mov qword ptr [rsp + 0x10], rdx
0x1408fb7a5: push rbp
0x1408fb7a6: push r15
0x1408fb7a8: push r14
0x1408fb7aa: push r13
0x1408fb7ac: push r12
0x1408fb7ae: push rsi
0x1408fb7af: push rdi
0x1408fb7b0: push rbx
0x1408fb7b1: sub rsp, 0xc8
0x1408fb7b8: lea rbp, [rdx + 0x80]
0x1408fb7bf: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb7c5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb7cb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb7d1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb7d7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb7e0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb7e9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb7f1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb7f9: cmp byte ptr [rbp + 0x103e], 0
0x1408fb800: je 0x1408fb863
0x1408fb802: lea rcx, [rbp + 0x7c0]
0x1408fb809: call 0x1408935c0
0x1408fb80e: mov byte ptr [rbp + 0x1047], 1
0x1408fb815: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb81d: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb825: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb82e: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb837: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb83d: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb843: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb849: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb84f: add rsp, 0xc8
0x1408fb856: pop rbx
0x1408fb857: pop rdi
0x1408fb858: pop rsi
0x1408fb859: pop r12
0x1408fb85b: pop r13
0x1408fb85d: pop r14
0x1408fb85f: pop r15
0x1408fb861: pop rbp
0x1408fb862: ret 
0x1408fb863: mov byte ptr [rbp + 0x1047], 1
0x1408fb86a: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb872: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb87a: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb883: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb88c: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb892: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb898: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb89e: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb8a4: add rsp, 0xc8
0x1408fb8ab: pop rbx
0x1408fb8ac: pop rdi
0x1408fb8ad: pop rsi
0x1408fb8ae: pop r12
0x1408fb8b0: pop r13
0x1408fb8b2: pop r14
0x1408fb8b4: pop r15
0x1408fb8b6: pop rbp
0x1408fb8b7: ret 
0x1408fb8b8: nop dword ptr [rax + rax]
0x1408fb8c0: mov qword ptr [rsp + 0x10], rdx
0x1408fb8c5: push rbp
0x1408fb8c6: push r15
0x1408fb8c8: push r14
0x1408fb8ca: push r13
0x1408fb8cc: push r12
0x1408fb8ce: push rsi
0x1408fb8cf: push rdi
0x1408fb8d0: push rbx
0x1408fb8d1: sub rsp, 0xc8
0x1408fb8d8: lea rbp, [rdx + 0x80]
0x1408fb8df: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb8e5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb8eb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb8f1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb8f7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb900: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb909: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb911: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb919: mov rdx, qword ptr [rbp + 0xf00]
0x1408fb920: test rdx, rdx
0x1408fb923: je 0x1408fb937
0x1408fb925: mov rcx, qword ptr [rbp + 0xf08]
0x1408fb92c: mov r8d, 1
0x1408fb932: call 0x140613c20
0x1408fb937: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb93f: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fb947: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fb950: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fb959: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fb95f: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fb965: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fb96b: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fb971: add rsp, 0xc8
0x1408fb978: pop rbx
0x1408fb979: pop rdi
0x1408fb97a: pop rsi
0x1408fb97b: pop r12
0x1408fb97d: pop r13
0x1408fb97f: pop r14
0x1408fb981: pop r15
0x1408fb983: pop rbp
0x1408fb984: ret 
0x1408fb985: nop word ptr cs:[rax + rax]
0x1408fb990: mov qword ptr [rsp + 0x10], rdx
0x1408fb995: push rbp
0x1408fb996: push r15
0x1408fb998: push r14
0x1408fb99a: push r13
0x1408fb99c: push r12
0x1408fb99e: push rsi
0x1408fb99f: push rdi
0x1408fb9a0: push rbx
0x1408fb9a1: sub rsp, 0xc8
0x1408fb9a8: lea rbp, [rdx + 0x80]
0x1408fb9af: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fb9b5: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fb9bb: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fb9c1: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fb9c7: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fb9d0: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fb9d9: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fb9e1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fb9e9: lea rcx, [rbp + 0xcb0]
0x1408fb9f0: call 0x140d4cc30
0x1408fb9f5: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fb9fd: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fba05: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fba0e: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fba17: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fba1d: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fba23: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fba29: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fba2f: add rsp, 0xc8
0x1408fba36: pop rbx
0x1408fba37: pop rdi
0x1408fba38: pop rsi
0x1408fba39: pop r12
0x1408fba3b: pop r13
0x1408fba3d: pop r14
0x1408fba3f: pop r15
0x1408fba41: pop rbp
0x1408fba42: ret 
0x1408fba43: nop word ptr cs:[rax + rax]
0x1408fba50: mov qword ptr [rsp + 0x10], rdx
0x1408fba55: push rbp
0x1408fba56: push r15
0x1408fba58: push r14
0x1408fba5a: push r13
0x1408fba5c: push r12
0x1408fba5e: push rsi
0x1408fba5f: push rdi
0x1408fba60: push rbx
0x1408fba61: sub rsp, 0xc8
0x1408fba68: lea rbp, [rdx + 0x80]
0x1408fba6f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fba75: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fba7b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fba81: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fba87: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fba90: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fba99: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fbaa1: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fbaa9: mov rdx, qword ptr [rbp + 0xed8]
0x1408fbab0: test rdx, rdx
0x1408fbab3: je 0x1408fbac7
0x1408fbab5: mov rcx, qword ptr [rbp + 0xee0]
0x1408fbabc: mov r8d, 1
0x1408fbac2: call 0x140613c20
0x1408fbac7: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fbacf: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fbad7: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fbae0: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fbae9: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fbaef: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fbaf5: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fbafb: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fbb01: add rsp, 0xc8
0x1408fbb08: pop rbx
0x1408fbb09: pop rdi
0x1408fbb0a: pop rsi
0x1408fbb0b: pop r12
0x1408fbb0d: pop r13
0x1408fbb0f: pop r14
0x1408fbb11: pop r15
0x1408fbb13: pop rbp
0x1408fbb14: ret 
0x1408fbb15: nop word ptr cs:[rax + rax]
0x1408fbb20: mov qword ptr [rsp + 0x10], rdx
0x1408fbb25: push rbp
0x1408fbb26: push r15
0x1408fbb28: push r14
0x1408fbb2a: push r13
0x1408fbb2c: push r12
0x1408fbb2e: push rsi
0x1408fbb2f: push rdi
0x1408fbb30: push rbx
0x1408fbb31: sub rsp, 0xc8
0x1408fbb38: lea rbp, [rdx + 0x80]
0x1408fbb3f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fbb45: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fbb4b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fbb51: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fbb57: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fbb60: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fbb69: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fbb71: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fbb79: movzx ebx, byte ptr [rbp + 0x1047]
0x1408fbb80: mov rdx, qword ptr [rbp + 0xd68]
0x1408fbb87: test rdx, rdx
0x1408fbb8a: je 0x1408fbba2
0x1408fbb8c: mov rcx, qword ptr [rbp + 0xd70]
0x1408fbb93: shl rdx, 4
0x1408fbb97: mov r8d, 8
0x1408fbb9d: call 0x140613c20
0x1408fbba2: and bl, 1
0x1408fbba5: mov byte ptr [rbp + 0x1046], bl
0x1408fbbab: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fbbb3: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fbbbb: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fbbc4: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fbbcd: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fbbd3: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fbbd9: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fbbdf: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fbbe5: add rsp, 0xc8
0x1408fbbec: pop rbx
0x1408fbbed: pop rdi
0x1408fbbee: pop rsi
0x1408fbbef: pop r12
0x1408fbbf1: pop r13
0x1408fbbf3: pop r14
0x1408fbbf5: pop r15
0x1408fbbf7: pop rbp
0x1408fbbf8: ret 
0x1408fbbf9: nop dword ptr [rax]
0x1408fbc00: mov qword ptr [rsp + 0x10], rdx
0x1408fbc05: push rbp
0x1408fbc06: push r15
0x1408fbc08: push r14
0x1408fbc0a: push r13
0x1408fbc0c: push r12
0x1408fbc0e: push rsi
0x1408fbc0f: push rdi
0x1408fbc10: push rbx
0x1408fbc11: sub rsp, 0xc8
0x1408fbc18: lea rbp, [rdx + 0x80]
0x1408fbc1f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fbc25: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fbc2b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fbc31: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fbc37: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fbc40: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fbc49: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fbc51: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fbc59: cmp byte ptr [rbp + 0x1046], 0
0x1408fbc60: je 0x1408fbcbc
0x1408fbc62: lea rcx, [rbp + 0x1c0]
0x1408fbc69: call 0x1408923c0
0x1408fbc6e: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fbc76: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fbc7e: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fbc87: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fbc90: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fbc96: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fbc9c: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fbca2: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fbca8: add rsp, 0xc8
0x1408fbcaf: pop rbx
0x1408fbcb0: pop rdi
0x1408fbcb1: pop rsi
0x1408fbcb2: pop r12
0x1408fbcb4: pop r13
0x1408fbcb6: pop r14
0x1408fbcb8: pop r15
0x1408fbcba: pop rbp
0x1408fbcbb: ret 
0x1408fbcbc: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fbcc4: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fbccc: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fbcd5: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fbcde: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fbce4: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fbcea: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fbcf0: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fbcf6: add rsp, 0xc8
0x1408fbcfd: pop rbx
0x1408fbcfe: pop rdi
0x1408fbcff: pop rsi
0x1408fbd00: pop r12
0x1408fbd02: pop r13
0x1408fbd04: pop r14
0x1408fbd06: pop r15
0x1408fbd08: pop rbp
0x1408fbd09: ret 
0x1408fbd0a: nop word ptr [rax + rax]
0x1408fbd10: mov qword ptr [rsp + 0x10], rdx
0x1408fbd15: push rbp
0x1408fbd16: push r15
0x1408fbd18: push r14
0x1408fbd1a: push r13
0x1408fbd1c: push r12
0x1408fbd1e: push rsi
0x1408fbd1f: push rdi
0x1408fbd20: push rbx
0x1408fbd21: sub rsp, 0xc8
0x1408fbd28: lea rbp, [rdx + 0x80]
0x1408fbd2f: movaps xmmword ptr [rsp + 0x40], xmm13
0x1408fbd35: movaps xmmword ptr [rsp + 0x50], xmm12
0x1408fbd3b: movaps xmmword ptr [rsp + 0x60], xmm11
0x1408fbd41: movaps xmmword ptr [rsp + 0x70], xmm10
0x1408fbd47: movaps xmmword ptr [rsp + 0x80], xmm9
0x1408fbd50: movaps xmmword ptr [rsp + 0x90], xmm8
0x1408fbd59: movaps xmmword ptr [rsp + 0xa0], xmm7
0x1408fbd61: movaps xmmword ptr [rsp + 0xb0], xmm6
0x1408fbd69: mov rdx, qword ptr [rbp + 0xed8]
0x1408fbd70: test rdx, rdx
0x1408fbd73: je 0x1408fbd8b
0x1408fbd75: mov rcx, qword ptr [rbp + 0xee0]
0x1408fbd7c: shl rdx, 3
0x1408fbd80: mov r8d, 8
0x1408fbd86: call 0x140613c20
0x1408fbd8b: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1408fbd93: movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1408fbd9b: movaps xmm8, xmmword ptr [rsp + 0x90]
0x1408fbda4: movaps xmm9, xmmword ptr [rsp + 0x80]
0x1408fbdad: movaps xmm10, xmmword ptr [rsp + 0x70]
0x1408fbdb3: movaps xmm11, xmmword ptr [rsp + 0x60]
0x1408fbdb9: movaps xmm12, xmmword ptr [rsp + 0x50]
0x1408fbdbf: movaps xmm13, xmmword ptr [rsp + 0x40]
0x1408fbdc5: add rsp, 0xc8
0x1408fbdcc: pop rbx
0x1408fbdcd: pop rdi
0x1408fbdce: pop rsi
0x1408fbdcf: pop r12
0x1408fbdd1: pop r13
0x1408fbdd3: pop r14
0x1408fbdd5: pop r15
0x1408fbdd7: pop rbp
0x1408fbdd8: ret 
