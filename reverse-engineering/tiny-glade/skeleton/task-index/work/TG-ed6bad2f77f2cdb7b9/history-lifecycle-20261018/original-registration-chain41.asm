141737ae0: push rbp
141737ae1: push rsi
141737ae2: push rdi
141737ae3: push rbx
141737ae4: sub rsp, 0x2a8
141737aeb: lea rbp, [rsp + 0x80]
141737af3: mov qword ptr [rbp + 0x220], 0xfffffffffffffffe
141737afe: mov rsi, rcx
141737b01: movups xmm0, xmmword ptr [rdx + 0x60]
141737b05: movaps xmmword ptr [rbp + 0x70], xmm0
141737b09: movups xmm0, xmmword ptr [rdx + 0x50]
141737b0d: movaps xmmword ptr [rbp + 0x60], xmm0
141737b11: movups xmm0, xmmword ptr [rdx + 0x40]
141737b15: movaps xmmword ptr [rbp + 0x50], xmm0
141737b19: movups xmm0, xmmword ptr [rdx]
141737b1c: movups xmm1, xmmword ptr [rdx + 0x10]
141737b20: movups xmm2, xmmword ptr [rdx + 0x20]
141737b24: movups xmm3, xmmword ptr [rdx + 0x30]
141737b28: movaps xmmword ptr [rbp + 0x40], xmm3
141737b2c: movaps xmmword ptr [rbp + 0x30], xmm2
141737b30: movaps xmmword ptr [rbp + 0x20], xmm1
141737b34: movaps xmmword ptr [rbp + 0x10], xmm0
141737b38: movzx eax, byte ptr [rip + 0x1efd0ca]
141737b3f: mov ecx, 0x150
141737b44: mov edx, 8
141737b49: call 0x140613c10
141737b4e: test rax, rax
141737b51: je 0x141737e87
141737b57: mov qword ptr [rbp + 0x218], rax
141737b5e: xorps xmm0, xmm0
141737b61: movups xmmword ptr [rbp + 0xf8], xmm0
141737b68: movups xmmword ptr [rbp + 0x110], xmm0
141737b6f: movups xmmword ptr [rbp + 0x128], xmm0
141737b76: movups xmmword ptr [rbp + 0x140], xmm0
141737b7d: movups xmmword ptr [rbp + 0x158], xmm0
141737b84: mov dword ptr [rbp + 0x168], 0
141737b8e: movups xmmword ptr [rbp + 0x190], xmm0
141737b95: movups xmmword ptr [rbp + 0x1a8], xmm0
141737b9c: movups xmmword ptr [rbp + 0x1c0], xmm0
141737ba3: movups xmmword ptr [rbp + 0x1d8], xmm0
141737baa: movups xmmword ptr [rbp + 0x1f0], xmm0
141737bb1: mov dword ptr [rbp + 0x200], 0
141737bbb: mov qword ptr [rbp + 0x80], 0
141737bc6: mov qword ptr [rbp + 0xd8], 0
141737bd1: mov qword ptr [rbp + 0xe0], 8
141737bdc: mov qword ptr [rbp + 0xe8], 0
141737be7: mov qword ptr [rbp + 0xf0], 0x10
141737bf2: mov qword ptr [rbp + 0x108], 0x10
141737bfd: mov qword ptr [rbp + 0x120], 0x10
141737c08: mov qword ptr [rbp + 0x138], 0x10
141737c13: mov qword ptr [rbp + 0x150], 0x10
141737c1e: movabs rbx, 0x8000000000000000
141737c28: mov qword ptr [rbp + 0x170], rbx
141737c2f: lea rax, [rip + 0x1595897]
141737c36: mov qword ptr [rbp + 0x178], rax
141737c3d: mov qword ptr [rbp + 0x180], 0x44
141737c48: mov qword ptr [rbp + 0x188], 0x10
141737c53: mov qword ptr [rbp + 0x1a0], 0x10
141737c5e: mov qword ptr [rbp + 0x1b8], 0x10
141737c69: mov qword ptr [rbp + 0x1d0], 0x10
141737c74: mov qword ptr [rbp + 0x1e8], 0x10
141737c7f: mov dword ptr [rbp + 0x208], 0
141737c89: mov word ptr [rbp + 0x20c], 1
141737c92: mov dword ptr [rbp + 0x210], 0
141737c9c: movzx eax, byte ptr [rip + 0x1efcf66]
141737ca3: mov ecx, 0x198
141737ca8: mov edx, 8
141737cad: call 0x140613c10
141737cb2: test rax, rax
141737cb5: je 0x141737e98
141737cbb: mov rdi, rax
141737cbe: lea rdx, [rbp + 0x80]
141737cc5: mov r8d, 0x198
141737ccb: mov rcx, rax
141737cce: call 0x142923c00
141737cd3: lea r8, [rip + 0x159571e]
141737cda: lea rcx, [rbp - 0x60]
141737cde: mov rdx, rdi
141737ce1: call 0x140ce4b10
141737ce6: lea rcx, [rbp + 0x80]
141737ced: call 0x141740460
141737cf2: movaps xmm0, xmmword ptr [rbp + 0x70]
141737cf6: mov rax, qword ptr [rbp + 0x218]
141737cfd: movups xmmword ptr [rax + 0x140], xmm0
141737d04: movaps xmm0, xmmword ptr [rbp + 0x60]
141737d08: movups xmmword ptr [rax + 0x130], xmm0
141737d0f: movaps xmm0, xmmword ptr [rbp + 0x50]
141737d13: movups xmmword ptr [rax + 0x120], xmm0
141737d1a: movaps xmm0, xmmword ptr [rbp + 0x10]
141737d1e: movaps xmm1, xmmword ptr [rbp + 0x20]
141737d22: movaps xmm2, xmmword ptr [rbp + 0x30]
141737d26: movaps xmm3, xmmword ptr [rbp + 0x40]
141737d2a: movups xmmword ptr [rax + 0x110], xmm3
141737d31: movups xmmword ptr [rax + 0x100], xmm2
141737d38: movups xmmword ptr [rax + 0xf0], xmm1
141737d3f: movups xmmword ptr [rax + 0xe0], xmm0
141737d46: movups xmm0, xmmword ptr [rbp - 0x60]
141737d4a: movups xmm1, xmmword ptr [rbp - 0x50]
141737d4e: movups xmm2, xmmword ptr [rbp - 0x40]
141737d52: movups xmm3, xmmword ptr [rbp - 0x30]
141737d56: movups xmmword ptr [rax], xmm0
141737d59: movups xmmword ptr [rax + 0x10], xmm1
141737d5d: movups xmmword ptr [rax + 0x20], xmm2
141737d61: movups xmmword ptr [rax + 0x30], xmm3
141737d65: movups xmm0, xmmword ptr [rbp - 0x20]
141737d69: movups xmmword ptr [rax + 0x40], xmm0
141737d6d: movups xmm0, xmmword ptr [rbp - 0x10]
141737d71: movups xmmword ptr [rax + 0x50], xmm0
141737d75: movups xmm0, xmmword ptr [rbp]
141737d79: movups xmmword ptr [rax + 0x60], xmm0
141737d7d: movups xmm0, xmmword ptr [rbp + 0x80]
141737d84: movups xmm1, xmmword ptr [rbp + 0x90]
141737d8b: movups xmm2, xmmword ptr [rbp + 0xa0]
141737d92: movups xmm3, xmmword ptr [rbp + 0xb0]
141737d99: movups xmmword ptr [rax + 0x70], xmm0
141737d9d: movups xmmword ptr [rax + 0x80], xmm1
141737da4: movups xmmword ptr [rax + 0x90], xmm2
141737dab: movups xmmword ptr [rax + 0xa0], xmm3
141737db2: movups xmm0, xmmword ptr [rbp + 0xc0]
141737db9: movups xmmword ptr [rax + 0xb0], xmm0
141737dc0: movups xmm0, xmmword ptr [rbp + 0xd0]
141737dc7: movups xmmword ptr [rax + 0xc0], xmm0
141737dce: movups xmm0, xmmword ptr [rbp + 0xe0]
141737dd5: movups xmmword ptr [rax + 0xd0], xmm0
141737ddc: mov qword ptr [rbp + 0x88], 3
141737de7: mov qword ptr [rbp + 0x90], rax
141737dee: mov qword ptr [rbp + 0x98], 3
141737df9: mov qword ptr [rbp + 0xa0], 0
141737e04: mov qword ptr [rbp + 0xa8], 8
141737e0f: xorps xmm0, xmm0
141737e12: movups xmmword ptr [rbp + 0xb0], xmm0
141737e19: mov qword ptr [rbp + 0x80], rbx
141737e20: lea rcx, [rbp + 0xb8]
141737e27: call 0x1416a00a0
141737e2c: movups xmm0, xmmword ptr [rbp + 0xe0]
141737e33: movups xmmword ptr [rsi + 0x60], xmm0
141737e37: movups xmm0, xmmword ptr [rbp + 0xd0]
141737e3e: movups xmmword ptr [rsi + 0x50], xmm0
141737e42: movups xmm0, xmmword ptr [rbp + 0xc0]
141737e49: movups xmmword ptr [rsi + 0x40], xmm0
141737e4d: movups xmm0, xmmword ptr [rbp + 0x80]
141737e54: movups xmm1, xmmword ptr [rbp + 0x90]
141737e5b: movups xmm2, xmmword ptr [rbp + 0xa0]
141737e62: movups xmm3, xmmword ptr [rbp + 0xb0]
141737e69: movups xmmword ptr [rsi + 0x30], xmm3
141737e6d: movups xmmword ptr [rsi + 0x20], xmm2
141737e71: movups xmmword ptr [rsi + 0x10], xmm1
141737e75: movups xmmword ptr [rsi], xmm0
141737e78: mov rax, rsi
141737e7b: add rsp, 0x2a8
141737e82: pop rbx
141737e83: pop rdi
141737e84: pop rsi
141737e85: pop rbp
141737e86: ret 
141737e87: mov ecx, 8
