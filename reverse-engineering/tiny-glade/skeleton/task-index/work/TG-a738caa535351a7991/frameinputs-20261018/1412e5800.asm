1412e5800: push rbp
1412e5801: push r14
1412e5803: push rsi
1412e5804: push rdi
1412e5805: push rbx
1412e5806: sub rsp, 0xb0
1412e580d: lea rbp, [rsp + 0x80]
1412e5815: movdqa xmmword ptr [rbp + 0x20], xmm7
1412e581a: movdqa xmmword ptr [rbp + 0x10], xmm6
1412e581f: mov qword ptr [rbp + 8], 0xfffffffffffffffe
1412e5827: test r8, r8
1412e582a: jne 0x1412e5bf6
1412e5830: mov rdi, rdx
1412e5833: mov rsi, rcx
1412e5836: movzx eax, byte ptr [rip + 0x234f3cc]
1412e583d: mov ecx, 0x10001
1412e5842: mov edx, 1
1412e5847: call 0x140613c10
1412e584c: test rax, rax
1412e584f: je 0x1412e5c0e
1412e5855: mov rbx, rax
1412e5858: mov r8d, 0x10000
1412e585e: mov rcx, rax
1412e5861: xor edx, edx
1412e5863: call 0x142923c20
1412e5868: mov byte ptr [rbx + 0x10000], 0x20
1412e586f: movzx eax, byte ptr [rip + 0x234f393]
1412e5876: mov ecx, 0xc000000
1412e587b: mov edx, 4
1412e5880: call 0x140613c10
1412e5885: test rax, rax
1412e5888: mov qword ptr [rbp - 0x20], rbx
1412e588c: je 0x1412e5c24
1412e5892: mov r14, rax
1412e5895: mov eax, 0x16c
1412e589a: pxor xmm1, xmm1
1412e589e: movq xmm0, qword ptr [rip + 0x17a6d3a]
1412e58a6: nop word ptr cs:[rax + rax]
1412e58b0: movdqu xmmword ptr [r14 + rax - 0x13c], xmm1
1412e58ba: movdqu xmmword ptr [r14 + rax - 0x14c], xmm1
1412e58c4: movdqu xmmword ptr [r14 + rax - 0x15c], xmm1
1412e58ce: movdqu xmmword ptr [r14 + rax - 0x16c], xmm1
1412e58d8: movq qword ptr [r14 + rax - 0x12c], xmm0
1412e58e2: mov dword ptr [r14 + rax - 0x124], 0xc2c80000
1412e58ee: movdqu xmmword ptr [r14 + rax - 0x120], xmm1
1412e58f8: mov dword ptr [r14 + rax - 0x110], 0
1412e5904: movdqu xmmword ptr [r14 + rax - 0x10c], xmm1
1412e590e: movdqu xmmword ptr [r14 + rax - 0xfc], xmm1
1412e5918: movdqu xmmword ptr [r14 + rax - 0xec], xmm1
1412e5922: movdqu xmmword ptr [r14 + rax - 0xdc], xmm1
1412e592c: movq qword ptr [r14 + rax - 0xcc], xmm0
1412e5936: mov dword ptr [r14 + rax - 0xc4], 0xc2c80000
1412e5942: movdqu xmmword ptr [r14 + rax - 0xc0], xmm1
1412e594c: mov dword ptr [r14 + rax - 0xb0], 0
1412e5958: movdqu xmmword ptr [r14 + rax - 0x7c], xmm1
1412e595f: movdqu xmmword ptr [r14 + rax - 0x8c], xmm1
1412e5969: movdqu xmmword ptr [r14 + rax - 0x9c], xmm1
1412e5973: movdqu xmmword ptr [r14 + rax - 0xac], xmm1
1412e597d: movq qword ptr [r14 + rax - 0x6c], xmm0
1412e5984: mov dword ptr [r14 + rax - 0x64], 0xc2c80000
1412e598d: movdqu xmmword ptr [r14 + rax - 0x60], xmm1
1412e5994: mov dword ptr [r14 + rax - 0x50], 0
1412e599d: cmp rax, 0xbffffec
1412e59a3: je 0x1412e59ea
1412e59a5: lea rcx, [r14 + rax]
1412e59a9: add rcx, -0x4c
1412e59ad: movdqu xmmword ptr [rcx + 0x30], xmm1
1412e59b2: movdqu xmmword ptr [rcx + 0x20], xmm1
1412e59b7: movdqu xmmword ptr [rcx + 0x10], xmm1
1412e59bc: movdqu xmmword ptr [rcx], xmm1
1412e59c0: movq qword ptr [r14 + rax - 0xc], xmm0
1412e59c7: mov dword ptr [r14 + rax - 4], 0xc2c80000
1412e59d0: movdqu xmmword ptr [r14 + rax], xmm1
1412e59d6: mov dword ptr [r14 + rax + 0x10], 0
1412e59df: add rax, 0x180
1412e59e5: jmp 0x1412e58b0
1412e59ea: pxor xmm1, xmm1
1412e59ee: movdqu xmmword ptr [r14 + rax - 0x1c], xmm1
1412e59f5: movdqu xmmword ptr [r14 + rax - 0x2c], xmm1
1412e59fc: movdqu xmmword ptr [r14 + rax - 0x3c], xmm1
1412e5a03: movdqu xmmword ptr [r14 + rax - 0x4c], xmm1
1412e5a0a: movq qword ptr [r14 + rax - 0xc], xmm0
1412e5a11: mov dword ptr [r14 + rax - 4], 0xc2c80000
1412e5a1a: movdqu xmmword ptr [r14 + rax], xmm1
1412e5a20: mov dword ptr [r14 + rax + 0x10], 0
1412e5a29: movzx eax, byte ptr [rip + 0x234f1d9]
1412e5a30: mov ecx, 0x40000
1412e5a35: mov edx, 4
1412e5a3a: call 0x140613c10
1412e5a3f: test rax, rax
1412e5a42: mov qword ptr [rbp - 0x18], r14
1412e5a46: je 0x1412e5c40
1412e5a4c: movdqa xmm0, xmmword ptr [rip + 0x17eef0c]
1412e5a54: mov ecx, 0xc
1412e5a59: movdqa xmm1, xmmword ptr [rip + 0x183510f]
1412e5a61: movdqa xmm2, xmmword ptr [rip + 0x1948cb7]
1412e5a69: movdqa xmm3, xmmword ptr [rip + 0x1948cbf]
1412e5a71: movdqa xmm4, xmmword ptr [rip + 0x1948cc7]
1412e5a79: movdqa xmm5, xmmword ptr [rip + 0x1948c4f]
1412e5a81: nop word ptr cs:[rax + rax]
1412e5a90: movdqa xmm6, xmm1
1412e5a94: psubd xmm6, xmm0
1412e5a98: movdqa xmm7, xmm2
1412e5a9c: psubd xmm7, xmm0
1412e5aa0: movdqu xmmword ptr [rax + rcx*4 - 0x30], xmm6
1412e5aa6: movdqu xmmword ptr [rax + rcx*4 - 0x20], xmm7
1412e5aac: movdqa xmm6, xmm3
1412e5ab0: psubd xmm6, xmm0
1412e5ab4: movdqa xmm7, xmm4
1412e5ab8: psubd xmm7, xmm0
1412e5abc: movdqu xmmword ptr [rax + rcx*4 - 0x10], xmm6
1412e5ac2: movdqu xmmword ptr [rax + rcx*4], xmm7
1412e5ac7: paddd xmm0, xmm5
1412e5acb: add rcx, 0x10
1412e5acf: cmp rcx, 0x1000c
1412e5ad6: jne 0x1412e5a90
1412e5ad8: mov qword ptr [rbp - 0x10], rax
1412e5adc: mov byte ptr [rbp + 4], 1
1412e5ae0: mov byte ptr [rbp + 3], 1
1412e5ae4: lea rcx, [rbp - 0x60]
1412e5ae8: mov rdx, rdi
1412e5aeb: xor r8d, r8d
1412e5aee: call 0x141618890
1412e5af3: mov byte ptr [rbp + 2], 1
1412e5af7: mov edx, 0x400
1412e5afc: xor ecx, ecx
1412e5afe: call 0x1416969a0
1412e5b03: mov qword ptr [rbp - 0x30], rax
1412e5b07: mov qword ptr [rbp - 8], rdx
1412e5b0b: mov edx, 0x400
1412e5b10: xor ecx, ecx
1412e5b12: call 0x1416969a0
1412e5b17: mov rbx, rax
1412e5b1a: mov rdi, rdx
1412e5b1d: movzx eax, byte ptr [rip + 0x234f0e5]
1412e5b24: mov ecx, 0x30
1412e5b29: mov edx, 8
1412e5b2e: call 0x140613c10
1412e5b33: test rax, rax
1412e5b36: je 0x1412e5c60
1412e5b3c: mov qword ptr [rax], 1
1412e5b43: mov qword ptr [rax + 8], 1
1412e5b4b: mov byte ptr [rax + 0x10], 0
1412e5b4f: mov qword ptr [rax + 0x18], 0
1412e5b57: mov qword ptr [rax + 0x20], 8
1412e5b5f: mov qword ptr [rax + 0x28], 0
1412e5b67: mov rcx, qword ptr [rbp - 0x40]
1412e5b6b: mov qword ptr [rsi + 0x88], rcx
1412e5b72: movups xmm0, xmmword ptr [rbp - 0x60]
1412e5b76: movups xmm1, xmmword ptr [rbp - 0x50]
1412e5b7a: movups xmmword ptr [rsi + 0x78], xmm1
1412e5b7e: movups xmmword ptr [rsi + 0x68], xmm0
1412e5b82: mov rcx, qword ptr [rbp - 0x20]
1412e5b86: mov qword ptr [rsi + 0x20], rcx
1412e5b8a: mov qword ptr [rsi + 0x28], 0x10001
1412e5b92: mov rcx, qword ptr [rbp - 0x30]
1412e5b96: mov qword ptr [rsi + 0x30], rcx
1412e5b9a: mov rcx, qword ptr [rbp - 8]
1412e5b9e: mov qword ptr [rsi + 0x38], rcx
1412e5ba2: mov qword ptr [rsi], 0
1412e5ba9: mov qword ptr [rsi + 8], 0x10000
1412e5bb1: mov rcx, qword ptr [rbp - 0x10]
1412e5bb5: mov qword ptr [rsi + 0x10], rcx
1412e5bb9: mov qword ptr [rsi + 0x18], 0x10000
1412e5bc1: mov rcx, qword ptr [rbp - 0x18]
1412e5bc5: mov qword ptr [rsi + 0x40], rcx
1412e5bc9: mov qword ptr [rsi + 0x48], 0x200000
1412e5bd1: mov qword ptr [rsi + 0x50], rbx
1412e5bd5: mov qword ptr [rsi + 0x58], rdi
1412e5bd9: mov qword ptr [rsi + 0x60], rax
1412e5bdd: mov rax, rsi
1412e5be0: movaps xmm6, xmmword ptr [rbp + 0x10]
1412e5be4: movaps xmm7, xmmword ptr [rbp + 0x20]
1412e5be8: add rsp, 0xb0
1412e5bef: pop rbx
1412e5bf0: pop rdi
1412e5bf1: pop rsi
1412e5bf2: pop r14
1412e5bf4: pop rbp
1412e5bf5: ret 
1412e5bf6: lea rcx, [rip + 0x194913b]
1412e5bfd: lea r8, [rip + 0x1949154]
1412e5c04: mov edx, 0x20
1412e5c09: call 0x1428d9430
1412e5c0e: lea r8, [rip + 0x194915b]
1412e5c15: mov ecx, 1
1412e5c1a: mov edx, 0x10001
1412e5c1f: call 0x1428d8fe3
1412e5c24: mov byte ptr [rbp + 7], 1
1412e5c28: lea r8, [rip + 0x1949159]
1412e5c2f: mov ecx, 4
1412e5c34: mov edx, 0xc000000
1412e5c39: call 0x1428d8fe3
1412e5c3e: jmp 0x1412e5c77
1412e5c40: mov byte ptr [rbp + 6], 1
1412e5c44: mov byte ptr [rbp + 5], 1
1412e5c48: lea r8, [rip + 0x1948d49]
1412e5c4f: mov ecx, 4
1412e5c54: mov edx, 0x40000
1412e5c59: call 0x1428d8fe3
1412e5c5e: jmp 0x1412e5c77
1412e5c60: mov qword ptr [rbp - 0x38], rbx
1412e5c64: mov qword ptr [rbp - 0x28], rdi
1412e5c68: mov ecx, 8
1412e5c6d: mov edx, 0x30
1412e5c72: call 0x1428d9000
1412e5c77: ud2 
1412e5c79: nop dword ptr [rax]
1412e5c80: mov qword ptr [rsp + 0x10], rdx
1412e5c85: push rbp
1412e5c86: push r14
1412e5c88: push rsi
1412e5c89: push rdi
1412e5c8a: push rbx
1412e5c8b: sub rsp, 0x40
1412e5c8f: lea rbp, [rdx + 0x80]
1412e5c96: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e5c9c: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5ca2: movzx ebx, byte ptr [rbp + 2]
1412e5ca6: mov edx, 0x10001
1412e5cab: mov r8d, 1
1412e5cb1: mov rcx, qword ptr [rbp - 0x20]
1412e5cb5: call 0x140613c20
1412e5cba: lea rcx, [rbp - 0x60]
1412e5cbe: call 0x140be24b0
1412e5cc3: mov byte ptr [rbp + 4], 0
1412e5cc7: mov byte ptr [rbp + 3], bl
1412e5cca: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5cd0: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5cd6: add rsp, 0x40
1412e5cda: pop rbx
1412e5cdb: pop rdi
1412e5cdc: pop rsi
1412e5cdd: pop r14
1412e5cdf: pop rbp
1412e5ce0: ret 
1412e5ce1: nop word ptr cs:[rax + rax]
1412e5cf0: mov qword ptr [rsp + 0x10], rdx
1412e5cf5: push rbp
1412e5cf6: push r14
1412e5cf8: push rsi
1412e5cf9: push rdi
1412e5cfa: push rbx
1412e5cfb: sub rsp, 0x40
1412e5cff: lea rbp, [rdx + 0x80]
1412e5d06: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e5d0c: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5d12: mov edx, 0x40000
1412e5d17: mov r8d, 4
1412e5d1d: mov rcx, qword ptr [rbp - 0x10]
1412e5d21: call 0x140613c20
1412e5d26: mov edx, 0xc000000
1412e5d2b: mov r8d, 4
1412e5d31: mov rcx, qword ptr [rbp - 0x18]
1412e5d35: call 0x140613c20
1412e5d3a: cmp qword ptr [rbp - 8], 0
1412e5d3f: je 0x1412e5d58
1412e5d41: mov rdx, qword ptr [rbp - 8]
1412e5d45: shl rdx, 3
1412e5d49: mov r8d, 8
1412e5d4f: mov rcx, qword ptr [rbp - 0x30]
1412e5d53: call 0x140613c20
1412e5d58: mov byte ptr [rbp + 2], 0
1412e5d5c: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5d62: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5d68: add rsp, 0x40
1412e5d6c: pop rbx
1412e5d6d: pop rdi
1412e5d6e: pop rsi
1412e5d6f: pop r14
1412e5d71: pop rbp
1412e5d72: ret 
1412e5d73: nop word ptr cs:[rax + rax]
1412e5d80: mov qword ptr [rsp + 0x10], rdx
1412e5d85: push rbp
1412e5d86: push r14
1412e5d88: push rsi
1412e5d89: push rdi
1412e5d8a: push rbx
1412e5d8b: sub rsp, 0x40
1412e5d8f: lea rbp, [rdx + 0x80]
1412e5d96: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e5d9c: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5da2: cmp qword ptr [rbp - 0x28], 0
1412e5da7: je 0x1412e5dc0
1412e5da9: mov rdx, qword ptr [rbp - 0x28]
1412e5dad: shl rdx, 3
1412e5db1: mov r8d, 8
1412e5db7: mov rcx, qword ptr [rbp - 0x38]
1412e5dbb: call 0x140613c20
1412e5dc0: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5dc6: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5dcc: add rsp, 0x40
1412e5dd0: pop rbx
1412e5dd1: pop rdi
1412e5dd2: pop rsi
1412e5dd3: pop r14
1412e5dd5: pop rbp
1412e5dd6: ret 
1412e5dd7: nop word ptr [rax + rax]
1412e5de0: mov qword ptr [rsp + 0x10], rdx
1412e5de5: push rbp
1412e5de6: push r14
1412e5de8: push rsi
1412e5de9: push rdi
1412e5dea: push rbx
1412e5deb: sub rsp, 0x40
1412e5def: lea rbp, [rdx + 0x80]
1412e5df6: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e5dfc: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5e02: movzx ebx, byte ptr [rbp + 4]
1412e5e06: cmp byte ptr [rbp + 3], 0
1412e5e0a: je 0x1412e5e41
1412e5e0c: mov edx, 0x40000
1412e5e11: mov r8d, 4
1412e5e17: mov rcx, qword ptr [rbp - 0x10]
1412e5e1b: call 0x140613c20
1412e5e20: and bl, 1
1412e5e23: mov byte ptr [rbp + 6], bl
1412e5e26: mov byte ptr [rbp + 5], 1
1412e5e2a: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5e30: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5e36: add rsp, 0x40
1412e5e3a: pop rbx
1412e5e3b: pop rdi
1412e5e3c: pop rsi
1412e5e3d: pop r14
1412e5e3f: pop rbp
1412e5e40: ret 
1412e5e41: and bl, 1
1412e5e44: mov byte ptr [rbp + 6], bl
1412e5e47: mov byte ptr [rbp + 5], 0
1412e5e4b: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5e51: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5e57: add rsp, 0x40
1412e5e5b: pop rbx
1412e5e5c: pop rdi
1412e5e5d: pop rsi
1412e5e5e: pop r14
1412e5e60: pop rbp
1412e5e61: ret 
1412e5e62: nop word ptr cs:[rax + rax]
1412e5e70: mov qword ptr [rsp + 0x10], rdx
1412e5e75: push rbp
1412e5e76: push r14
1412e5e78: push rsi
1412e5e79: push rdi
1412e5e7a: push rbx
1412e5e7b: sub rsp, 0x40
1412e5e7f: lea rbp, [rdx + 0x80]
1412e5e86: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e5e8c: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5e92: movzx ebx, byte ptr [rbp + 6]
1412e5e96: cmp byte ptr [rbp + 5], 0
1412e5e9a: je 0x1412e5ecd
1412e5e9c: mov edx, 0xc000000
1412e5ea1: mov r8d, 4
1412e5ea7: mov rcx, qword ptr [rbp - 0x18]
1412e5eab: call 0x140613c20
1412e5eb0: and bl, 1
1412e5eb3: mov byte ptr [rbp + 7], bl
1412e5eb6: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5ebc: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5ec2: add rsp, 0x40
1412e5ec6: pop rbx
1412e5ec7: pop rdi
1412e5ec8: pop rsi
1412e5ec9: pop r14
1412e5ecb: pop rbp
1412e5ecc: ret 
1412e5ecd: and bl, 1
1412e5ed0: mov byte ptr [rbp + 7], bl
1412e5ed3: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5ed9: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5edf: add rsp, 0x40
1412e5ee3: pop rbx
1412e5ee4: pop rdi
1412e5ee5: pop rsi
1412e5ee6: pop r14
1412e5ee8: pop rbp
1412e5ee9: ret 
1412e5eea: nop word ptr [rax + rax]
1412e5ef0: mov qword ptr [rsp + 0x10], rdx
1412e5ef5: push rbp
1412e5ef6: push r14
1412e5ef8: push rsi
1412e5ef9: push rdi
1412e5efa: push rbx
1412e5efb: sub rsp, 0x40
1412e5eff: lea rbp, [rdx + 0x80]
1412e5f06: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e5f0c: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5f12: cmp byte ptr [rbp + 7], 0
1412e5f16: je 0x1412e5f2c
1412e5f18: mov edx, 0x10001
1412e5f1d: mov r8d, 1
1412e5f23: mov rcx, qword ptr [rbp - 0x20]
1412e5f27: call 0x140613c20
1412e5f2c: movaps xmm6, xmmword ptr [rsp + 0x30]
1412e5f31: movaps xmm7, xmmword ptr [rsp + 0x20]
1412e5f36: add rsp, 0x40
1412e5f3a: pop rbx
1412e5f3b: pop rdi
1412e5f3c: pop rsi
1412e5f3d: pop r14
1412e5f3f: pop rbp
1412e5f40: ret 
