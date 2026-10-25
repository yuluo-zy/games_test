1412c1c50: push rbp
1412c1c51: push rsi
1412c1c52: sub rsp, 0xe8
1412c1c59: lea rbp, [rsp + 0x80]
1412c1c61: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c1c69: mov rsi, rcx
1412c1c6c: lea rcx, [rbp - 0x58]
1412c1c70: xor r8d, r8d
1412c1c73: call 0x14161f1e0
1412c1c78: movzx eax, byte ptr [rip + 0x2372f8a]
1412c1c7f: mov ecx, 0x38
1412c1c84: mov edx, 8
1412c1c89: call 0x140613c10
1412c1c8e: test rax, rax
1412c1c91: je 0x1412c1d4f
1412c1c97: mov rcx, qword ptr [rbp - 0x28]
1412c1c9b: mov qword ptr [rax + 0x30], rcx
1412c1c9f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c1ca3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c1ca7: movups xmm2, xmmword ptr [rbp - 0x38]
1412c1cab: movups xmmword ptr [rax + 0x20], xmm2
1412c1caf: movups xmmword ptr [rax + 0x10], xmm1
1412c1cb3: movups xmmword ptr [rax], xmm0
1412c1cb6: lea rcx, [rip + 0x17db46a]
1412c1cbd: mov qword ptr [rbp + 0x10], rcx
1412c1cc1: mov qword ptr [rbp + 0x18], 0x44
1412c1cc9: mov qword ptr [rbp + 0x20], rax
1412c1ccd: lea rax, [rip + 0x196b08c]
1412c1cd4: mov qword ptr [rbp + 0x28], rax
1412c1cd8: mov qword ptr [rbp + 0x40], 0
1412c1ce0: mov byte ptr [rbp + 0x56], 5
1412c1ce4: mov qword ptr [rbp + 0x30], rcx
1412c1ce8: mov qword ptr [rbp + 0x38], 0x44
1412c1cf0: movzx eax, byte ptr [rip + 0x2372f12]
1412c1cf7: mov ecx, 0x40
1412c1cfc: mov edx, 8
1412c1d01: call 0x140613c10
1412c1d06: test rax, rax
1412c1d09: je 0x1412c1d60
1412c1d0b: movups xmm0, xmmword ptr [rbp + 0x20]
1412c1d0f: movups xmm1, xmmword ptr [rbp + 0x30]
1412c1d13: movups xmm2, xmmword ptr [rbp + 0x40]
1412c1d17: movups xmm3, xmmword ptr [rbp + 0x50]
1412c1d1b: movups xmmword ptr [rax + 0x30], xmm3
1412c1d1f: movups xmmword ptr [rax + 0x20], xmm2
1412c1d23: movups xmmword ptr [rax + 0x10], xmm1
1412c1d27: movups xmmword ptr [rax], xmm0
1412c1d2a: movabs rdx, 0x29d4a64d5ea39871
1412c1d34: mov rcx, rsi
1412c1d37: mov r8, rax
1412c1d3a: call 0x140969bf0
1412c1d3f: cmp rax, 1
1412c1d43: je 0x1412c1d71
1412c1d45: add rsp, 0xe8
1412c1d4c: pop rsi
1412c1d4d: pop rbp
1412c1d4e: ret 
1412c1d4f: mov ecx, 8
1412c1d54: mov edx, 0x38
1412c1d59: call 0x1428d9000
1412c1d5e: jmp 0x1412c1d6f
1412c1d60: mov ecx, 8
1412c1d65: mov edx, 0x40
1412c1d6a: call 0x1428d9000
1412c1d6f: ud2 
1412c1d71: mov rax, rdx
1412c1d74: add rdx, 0x10
1412c1d78: mov rsi, rdx
1412c1d7b: cmp qword ptr [rax + 0x18], 0x44
1412c1d80: jne 0x1412c1dff
1412c1d82: mov rdx, qword ptr [rsi]
1412c1d85: lea rcx, [rip + 0x17db39b]
1412c1d8c: mov r8d, 0x44
1412c1d92: call 0x142923bf0
1412c1d97: test eax, eax
1412c1d99: jne 0x1412c1dff
1412c1d9b: lea rax, [rip + 0x196be06]
1412c1da2: mov qword ptr [rbp - 0x20], rax
1412c1da6: lea rax, [rip - 0xd8e50d]
1412c1dad: mov qword ptr [rbp - 0x18], rax
1412c1db1: lea rax, [rbp + 0x10]
1412c1db5: mov qword ptr [rbp - 0x10], rax
1412c1db9: lea rax, [rip - 0x11b7f30]
1412c1dc0: mov qword ptr [rbp - 8], rax
1412c1dc4: lea rax, [rip + 0x196b85d]
1412c1dcb: mov qword ptr [rbp + 0x20], rax
1412c1dcf: mov qword ptr [rbp + 0x28], 3
1412c1dd7: mov qword ptr [rbp + 0x40], 0
1412c1ddf: lea rax, [rbp - 0x20]
1412c1de3: mov qword ptr [rbp + 0x30], rax
1412c1de7: mov qword ptr [rbp + 0x38], 2
1412c1def: lea rdx, [rip + 0x196b862]
1412c1df6: lea rcx, [rbp + 0x20]
1412c1dfa: call 0x1428d9390
1412c1dff: lea rax, [rip + 0x196bda2]
1412c1e06: mov qword ptr [rbp + 0x20], rax
1412c1e0a: lea rax, [rip - 0xd8e571]
1412c1e11: mov qword ptr [rbp + 0x28], rax
1412c1e15: mov qword ptr [rbp + 0x30], rsi
1412c1e19: lea rax, [rip - 0x11b7f90]
1412c1e20: mov qword ptr [rbp + 0x38], rax
1412c1e24: lea rcx, [rbp + 0x10]
1412c1e28: mov qword ptr [rbp + 0x40], rcx
1412c1e2c: mov qword ptr [rbp + 0x48], rax
1412c1e30: lea rax, [rip + 0x196b731]
1412c1e37: mov qword ptr [rbp - 0x20], rax
1412c1e3b: mov qword ptr [rbp - 0x18], 3
1412c1e43: mov qword ptr [rbp], 0
1412c1e4b: lea rax, [rbp + 0x20]
1412c1e4f: mov qword ptr [rbp - 0x10], rax
1412c1e53: mov qword ptr [rbp - 8], 3
1412c1e5b: lea rdx, [rip + 0x196b76e]
1412c1e62: lea rcx, [rbp - 0x20]
1412c1e66: call 0x1428d9390
1412c1e6b: int3 
1412c1e6c: nop dword ptr [rax]
1412c1e70: mov qword ptr [rsp + 0x10], rdx
1412c1e75: push rbp
1412c1e76: push rsi
1412c1e77: sub rsp, 0x28
1412c1e7b: lea rbp, [rdx + 0x80]
1412c1e82: lea rcx, [rbp - 0x48]
1412c1e86: call 0x140be24b0
1412c1e8b: nop 
1412c1e8c: add rsp, 0x28
1412c1e90: pop rsi
1412c1e91: pop rbp
1412c1e92: ret 
1412c1e93: nop word ptr cs:[rax + rax]
1412c1ea0: mov qword ptr [rsp + 0x10], rdx
1412c1ea5: push rbp
1412c1ea6: push rsi
1412c1ea7: sub rsp, 0x28
1412c1eab: lea rbp, [rdx + 0x80]
1412c1eb2: lea rcx, [rbp + 0x20]
1412c1eb6: call 0x1400c04b0
1412c1ebb: nop 
1412c1ebc: add rsp, 0x28
1412c1ec0: pop rsi
1412c1ec1: pop rbp
1412c1ec2: ret 
