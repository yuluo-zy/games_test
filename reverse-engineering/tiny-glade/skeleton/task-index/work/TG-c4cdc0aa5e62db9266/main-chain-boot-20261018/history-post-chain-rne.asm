0x141737ae0: push rbp
0x141737ae1: push rsi
0x141737ae2: push rdi
0x141737ae3: push rbx
0x141737ae4: sub rsp, 0x2a8
0x141737aeb: lea rbp, [rsp + 0x80]
0x141737af3: mov qword ptr [rbp + 0x220], 0xfffffffffffffffe
0x141737afe: mov rsi, rcx
0x141737b01: movups xmm0, xmmword ptr [rdx + 0x60]
0x141737b05: movaps xmmword ptr [rbp + 0x70], xmm0
0x141737b09: movups xmm0, xmmword ptr [rdx + 0x50]
0x141737b0d: movaps xmmword ptr [rbp + 0x60], xmm0
0x141737b11: movups xmm0, xmmword ptr [rdx + 0x40]
0x141737b15: movaps xmmword ptr [rbp + 0x50], xmm0
0x141737b19: movups xmm0, xmmword ptr [rdx]
0x141737b1c: movups xmm1, xmmword ptr [rdx + 0x10]
0x141737b20: movups xmm2, xmmword ptr [rdx + 0x20]
0x141737b24: movups xmm3, xmmword ptr [rdx + 0x30]
0x141737b28: movaps xmmword ptr [rbp + 0x40], xmm3
0x141737b2c: movaps xmmword ptr [rbp + 0x30], xmm2
0x141737b30: movaps xmmword ptr [rbp + 0x20], xmm1
0x141737b34: movaps xmmword ptr [rbp + 0x10], xmm0
0x141737b38: movzx eax, byte ptr [rip + 0x1efd0ca]
0x141737b3f: mov ecx, 0x150
0x141737b44: mov edx, 8
0x141737b49: call 0x140613c10
0x141737b4e: test rax, rax
0x141737b51: je 0x141737e87
0x141737b57: mov qword ptr [rbp + 0x218], rax
0x141737b5e: xorps xmm0, xmm0
0x141737b61: movups xmmword ptr [rbp + 0xf8], xmm0
0x141737b68: movups xmmword ptr [rbp + 0x110], xmm0
0x141737b6f: movups xmmword ptr [rbp + 0x128], xmm0
0x141737b76: movups xmmword ptr [rbp + 0x140], xmm0
0x141737b7d: movups xmmword ptr [rbp + 0x158], xmm0
0x141737b84: mov dword ptr [rbp + 0x168], 0
0x141737b8e: movups xmmword ptr [rbp + 0x190], xmm0
0x141737b95: movups xmmword ptr [rbp + 0x1a8], xmm0
0x141737b9c: movups xmmword ptr [rbp + 0x1c0], xmm0
0x141737ba3: movups xmmword ptr [rbp + 0x1d8], xmm0
0x141737baa: movups xmmword ptr [rbp + 0x1f0], xmm0
0x141737bb1: mov dword ptr [rbp + 0x200], 0
0x141737bbb: mov qword ptr [rbp + 0x80], 0
0x141737bc6: mov qword ptr [rbp + 0xd8], 0
0x141737bd1: mov qword ptr [rbp + 0xe0], 8
0x141737bdc: mov qword ptr [rbp + 0xe8], 0
0x141737be7: mov qword ptr [rbp + 0xf0], 0x10
0x141737bf2: mov qword ptr [rbp + 0x108], 0x10
0x141737bfd: mov qword ptr [rbp + 0x120], 0x10
0x141737c08: mov qword ptr [rbp + 0x138], 0x10
0x141737c13: mov qword ptr [rbp + 0x150], 0x10
0x141737c1e: movabs rbx, 0x8000000000000000
0x141737c28: mov qword ptr [rbp + 0x170], rbx
0x141737c2f: lea rax, [rip + 0x1595897]
0x141737c36: mov qword ptr [rbp + 0x178], rax
0x141737c3d: mov qword ptr [rbp + 0x180], 0x44
0x141737c48: mov qword ptr [rbp + 0x188], 0x10
0x141737c53: mov qword ptr [rbp + 0x1a0], 0x10
0x141737c5e: mov qword ptr [rbp + 0x1b8], 0x10
0x141737c69: mov qword ptr [rbp + 0x1d0], 0x10
0x141737c74: mov qword ptr [rbp + 0x1e8], 0x10
0x141737c7f: mov dword ptr [rbp + 0x208], 0
0x141737c89: mov word ptr [rbp + 0x20c], 1
0x141737c92: mov dword ptr [rbp + 0x210], 0
0x141737c9c: movzx eax, byte ptr [rip + 0x1efcf66]
0x141737ca3: mov ecx, 0x198
0x141737ca8: mov edx, 8
0x141737cad: call 0x140613c10
0x141737cb2: test rax, rax
0x141737cb5: je 0x141737e98
0x141737cbb: mov rdi, rax
0x141737cbe: lea rdx, [rbp + 0x80]
0x141737cc5: mov r8d, 0x198
0x141737ccb: mov rcx, rax
0x141737cce: call 0x142923c00
0x141737cd3: lea r8, [rip + 0x159571e]
0x141737cda: lea rcx, [rbp - 0x60]
0x141737cde: mov rdx, rdi
0x141737ce1: call 0x140ce4b10
0x141737ce6: lea rcx, [rbp + 0x80]
0x141737ced: call 0x141740460
0x141737cf2: movaps xmm0, xmmword ptr [rbp + 0x70]
0x141737cf6: mov rax, qword ptr [rbp + 0x218]
0x141737cfd: movups xmmword ptr [rax + 0x140], xmm0
0x141737d04: movaps xmm0, xmmword ptr [rbp + 0x60]
0x141737d08: movups xmmword ptr [rax + 0x130], xmm0
0x141737d0f: movaps xmm0, xmmword ptr [rbp + 0x50]
0x141737d13: movups xmmword ptr [rax + 0x120], xmm0
0x141737d1a: movaps xmm0, xmmword ptr [rbp + 0x10]
0x141737d1e: movaps xmm1, xmmword ptr [rbp + 0x20]
0x141737d22: movaps xmm2, xmmword ptr [rbp + 0x30]
0x141737d26: movaps xmm3, xmmword ptr [rbp + 0x40]
0x141737d2a: movups xmmword ptr [rax + 0x110], xmm3
0x141737d31: movups xmmword ptr [rax + 0x100], xmm2
0x141737d38: movups xmmword ptr [rax + 0xf0], xmm1
0x141737d3f: movups xmmword ptr [rax + 0xe0], xmm0
0x141737d46: movups xmm0, xmmword ptr [rbp - 0x60]
0x141737d4a: movups xmm1, xmmword ptr [rbp - 0x50]
0x141737d4e: movups xmm2, xmmword ptr [rbp - 0x40]
0x141737d52: movups xmm3, xmmword ptr [rbp - 0x30]
0x141737d56: movups xmmword ptr [rax], xmm0
0x141737d59: movups xmmword ptr [rax + 0x10], xmm1
0x141737d5d: movups xmmword ptr [rax + 0x20], xmm2
0x141737d61: movups xmmword ptr [rax + 0x30], xmm3
0x141737d65: movups xmm0, xmmword ptr [rbp - 0x20]
0x141737d69: movups xmmword ptr [rax + 0x40], xmm0
0x141737d6d: movups xmm0, xmmword ptr [rbp - 0x10]
0x141737d71: movups xmmword ptr [rax + 0x50], xmm0
0x141737d75: movups xmm0, xmmword ptr [rbp]
0x141737d79: movups xmmword ptr [rax + 0x60], xmm0
0x141737d7d: movups xmm0, xmmword ptr [rbp + 0x80]
0x141737d84: movups xmm1, xmmword ptr [rbp + 0x90]
0x141737d8b: movups xmm2, xmmword ptr [rbp + 0xa0]
0x141737d92: movups xmm3, xmmword ptr [rbp + 0xb0]
0x141737d99: movups xmmword ptr [rax + 0x70], xmm0
0x141737d9d: movups xmmword ptr [rax + 0x80], xmm1
0x141737da4: movups xmmword ptr [rax + 0x90], xmm2
0x141737dab: movups xmmword ptr [rax + 0xa0], xmm3
0x141737db2: movups xmm0, xmmword ptr [rbp + 0xc0]
0x141737db9: movups xmmword ptr [rax + 0xb0], xmm0
0x141737dc0: movups xmm0, xmmword ptr [rbp + 0xd0]
0x141737dc7: movups xmmword ptr [rax + 0xc0], xmm0
0x141737dce: movups xmm0, xmmword ptr [rbp + 0xe0]
0x141737dd5: movups xmmword ptr [rax + 0xd0], xmm0
0x141737ddc: mov qword ptr [rbp + 0x88], 3
0x141737de7: mov qword ptr [rbp + 0x90], rax
0x141737dee: mov qword ptr [rbp + 0x98], 3
0x141737df9: mov qword ptr [rbp + 0xa0], 0
0x141737e04: mov qword ptr [rbp + 0xa8], 8
0x141737e0f: xorps xmm0, xmm0
0x141737e12: movups xmmword ptr [rbp + 0xb0], xmm0
0x141737e19: mov qword ptr [rbp + 0x80], rbx
0x141737e20: lea rcx, [rbp + 0xb8]
0x141737e27: call 0x1416a00a0
0x141737e2c: movups xmm0, xmmword ptr [rbp + 0xe0]
0x141737e33: movups xmmword ptr [rsi + 0x60], xmm0
0x141737e37: movups xmm0, xmmword ptr [rbp + 0xd0]
0x141737e3e: movups xmmword ptr [rsi + 0x50], xmm0
0x141737e42: movups xmm0, xmmword ptr [rbp + 0xc0]
0x141737e49: movups xmmword ptr [rsi + 0x40], xmm0
0x141737e4d: movups xmm0, xmmword ptr [rbp + 0x80]
0x141737e54: movups xmm1, xmmword ptr [rbp + 0x90]
0x141737e5b: movups xmm2, xmmword ptr [rbp + 0xa0]
0x141737e62: movups xmm3, xmmword ptr [rbp + 0xb0]
0x141737e69: movups xmmword ptr [rsi + 0x30], xmm3
0x141737e6d: movups xmmword ptr [rsi + 0x20], xmm2
0x141737e71: movups xmmword ptr [rsi + 0x10], xmm1
0x141737e75: movups xmmword ptr [rsi], xmm0
0x141737e78: mov rax, rsi
0x141737e7b: add rsp, 0x2a8
0x141737e82: pop rbx
0x141737e83: pop rdi
0x141737e84: pop rsi
0x141737e85: pop rbp
0x141737e86: ret 
0x141737e87: mov ecx, 8
0x141737e8c: mov edx, 0x150
0x141737e91: call 0x1428d9000
0x141737e96: jmp 0x141737ea7
0x141737e98: mov ecx, 8
0x141737e9d: mov edx, 0x198
0x141737ea2: call 0x1428d9000
0x141737ea7: ud2 
0x141737ea9: nop dword ptr [rax]
0x141737eb0: mov qword ptr [rsp + 0x10], rdx
0x141737eb5: push rbp
0x141737eb6: push rsi
0x141737eb7: push rdi
0x141737eb8: push rbx
0x141737eb9: sub rsp, 0x28
0x141737ebd: lea rbp, [rdx + 0x80]
0x141737ec4: lea rcx, [rbp + 0x80]
0x141737ecb: call 0x1405d0f10
0x141737ed0: nop 
0x141737ed1: add rsp, 0x28
0x141737ed5: pop rbx
0x141737ed6: pop rdi
0x141737ed7: pop rsi
0x141737ed8: pop rbp
0x141737ed9: ret 
0x141737eda: nop word ptr [rax + rax]
0x141737ee0: mov qword ptr [rsp + 0x10], rdx
0x141737ee5: push rbp
0x141737ee6: push rsi
0x141737ee7: push rdi
0x141737ee8: push rbx
0x141737ee9: sub rsp, 0x28
0x141737eed: lea rbp, [rdx + 0x80]
0x141737ef4: mov edx, 0x150
0x141737ef9: mov r8d, 8
0x141737eff: mov rcx, qword ptr [rbp + 0x218]
0x141737f06: call 0x140613c20
0x141737f0b: nop 
0x141737f0c: add rsp, 0x28
0x141737f10: pop rbx
0x141737f11: pop rdi
0x141737f12: pop rsi
0x141737f13: pop rbp
0x141737f14: ret 
0x141737f15: nop word ptr cs:[rax + rax]
0x141737f20: mov qword ptr [rsp + 0x10], rdx
0x141737f25: push rbp
0x141737f26: push rsi
0x141737f27: push rdi
0x141737f28: push rbx
0x141737f29: sub rsp, 0x28
0x141737f2d: lea rbp, [rdx + 0x80]
0x141737f34: lea rcx, [rbp - 0x60]
0x141737f38: call 0x140029880
0x141737f3d: nop 
0x141737f3e: add rsp, 0x28
0x141737f42: pop rbx
0x141737f43: pop rdi
0x141737f44: pop rsi
0x141737f45: pop rbp
0x141737f46: ret 
0x141737f47: nop word ptr [rax + rax]
0x141737f50: mov qword ptr [rsp + 0x10], rdx
0x141737f55: push rbp
0x141737f56: push rsi
0x141737f57: push rdi
0x141737f58: push rbx
0x141737f59: sub rsp, 0x28
0x141737f5d: lea rbp, [rdx + 0x80]
0x141737f64: lea rcx, [rbp + 0x10]
0x141737f68: call 0x140029880
0x141737f6d: nop 
0x141737f6e: add rsp, 0x28
0x141737f72: pop rbx
0x141737f73: pop rdi
0x141737f74: pop rsi
0x141737f75: pop rbp
0x141737f76: ret 
0x141737f77: nop word ptr [rax + rax]
0x141737f80: mov qword ptr [rsp + 0x10], rdx
0x141737f85: push rbp
0x141737f86: push rsi
0x141737f87: push rdi
0x141737f88: push rbx
0x141737f89: sub rsp, 0x28
