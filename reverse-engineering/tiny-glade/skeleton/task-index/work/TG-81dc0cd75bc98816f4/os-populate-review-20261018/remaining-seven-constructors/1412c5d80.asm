1412c5d80: push rbp
1412c5d81: push rsi
1412c5d82: sub rsp, 0xd8
1412c5d89: lea rbp, [rsp + 0x80]
1412c5d91: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c5d99: mov rsi, rcx
1412c5d9c: lea rcx, [rbp - 0x58]
1412c5da0: xor r8d, r8d
1412c5da3: call 0x141617be0
1412c5da8: mov qword ptr [rbp - 0x60], 0
1412c5db0: movzx eax, byte ptr [rip + 0x236ee52]
1412c5db7: mov ecx, 0x30
1412c5dbc: mov edx, 8
1412c5dc1: call 0x140613c10
1412c5dc6: test rax, rax
1412c5dc9: je 0x1412c5e7f
1412c5dcf: movups xmm0, xmmword ptr [rbp - 0x60]
1412c5dd3: movups xmm1, xmmword ptr [rbp - 0x50]
1412c5dd7: movups xmm2, xmmword ptr [rbp - 0x40]
1412c5ddb: movups xmmword ptr [rax + 0x20], xmm2
1412c5ddf: movups xmmword ptr [rax + 0x10], xmm1
1412c5de3: movups xmmword ptr [rax], xmm0
1412c5de6: lea rcx, [rip + 0x17d7546]
1412c5ded: mov qword ptr [rbp], rcx
1412c5df1: mov qword ptr [rbp + 8], 0x4b
1412c5df9: mov qword ptr [rbp + 0x10], rax
1412c5dfd: lea rax, [rip + 0x196750c]
1412c5e04: mov qword ptr [rbp + 0x18], rax
1412c5e08: mov qword ptr [rbp + 0x30], 0
1412c5e10: mov byte ptr [rbp + 0x46], 5
1412c5e14: mov qword ptr [rbp + 0x20], rcx
1412c5e18: mov qword ptr [rbp + 0x28], 0x4b
1412c5e20: movzx eax, byte ptr [rip + 0x236ede2]
1412c5e27: mov ecx, 0x40
1412c5e2c: mov edx, 8
1412c5e31: call 0x140613c10
1412c5e36: test rax, rax
1412c5e39: je 0x1412c5e90
1412c5e3b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c5e3f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c5e43: movups xmm2, xmmword ptr [rbp + 0x30]
1412c5e47: movups xmm3, xmmword ptr [rbp + 0x40]
1412c5e4b: movups xmmword ptr [rax + 0x30], xmm3
1412c5e4f: movups xmmword ptr [rax + 0x20], xmm2
1412c5e53: movups xmmword ptr [rax + 0x10], xmm1
1412c5e57: movups xmmword ptr [rax], xmm0
1412c5e5a: movabs rdx, 0xd6d72e989e1b8586
1412c5e64: mov rcx, rsi
1412c5e67: mov r8, rax
1412c5e6a: call 0x140969bf0
1412c5e6f: cmp rax, 1
1412c5e73: je 0x1412c5ea1
1412c5e75: add rsp, 0xd8
1412c5e7c: pop rsi
1412c5e7d: pop rbp
1412c5e7e: ret 
1412c5e7f: mov ecx, 8
1412c5e84: mov edx, 0x30
1412c5e89: call 0x1428d9000
1412c5e8e: jmp 0x1412c5e9f
1412c5e90: mov ecx, 8
1412c5e95: mov edx, 0x40
1412c5e9a: call 0x1428d9000
1412c5e9f: ud2 
1412c5ea1: mov rax, rdx
1412c5ea4: add rdx, 0x10
1412c5ea8: mov rsi, rdx
1412c5eab: cmp qword ptr [rax + 0x18], 0x4b
1412c5eb0: jne 0x1412c5f2e
1412c5eb2: mov rdx, qword ptr [rsi]
1412c5eb5: lea rcx, [rip + 0x17d7477]
1412c5ebc: mov r8d, 0x4b
1412c5ec2: call 0x142923bf0
1412c5ec7: test eax, eax
1412c5ec9: jne 0x1412c5f2e
1412c5ecb: lea rax, [rip + 0x1967e7e]
1412c5ed2: mov qword ptr [rbp - 0x30], rax
1412c5ed6: lea rax, [rip - 0xd9263d]
1412c5edd: mov qword ptr [rbp - 0x28], rax
1412c5ee1: mov rax, rbp
1412c5ee4: mov qword ptr [rbp - 0x20], rax
1412c5ee8: lea rax, [rip - 0x11bc05f]
1412c5eef: mov qword ptr [rbp - 0x18], rax
1412c5ef3: lea rax, [rip + 0x196772e]
1412c5efa: mov qword ptr [rbp + 0x10], rax
1412c5efe: mov qword ptr [rbp + 0x18], 3
1412c5f06: mov qword ptr [rbp + 0x30], 0
1412c5f0e: lea rax, [rbp - 0x30]
1412c5f12: mov qword ptr [rbp + 0x20], rax
1412c5f16: mov qword ptr [rbp + 0x28], 2
1412c5f1e: lea rdx, [rip + 0x1967733]
1412c5f25: lea rcx, [rbp + 0x10]
1412c5f29: call 0x1428d9390
1412c5f2e: lea rax, [rip + 0x1967e1b]
1412c5f35: mov qword ptr [rbp + 0x10], rax
1412c5f39: lea rax, [rip - 0xd926a0]
1412c5f40: mov qword ptr [rbp + 0x18], rax
1412c5f44: mov qword ptr [rbp + 0x20], rsi
1412c5f48: lea rax, [rip - 0x11bc0bf]
1412c5f4f: mov qword ptr [rbp + 0x28], rax
1412c5f53: mov rcx, rbp
1412c5f56: mov qword ptr [rbp + 0x30], rcx
1412c5f5a: mov qword ptr [rbp + 0x38], rax
1412c5f5e: lea rax, [rip + 0x1967603]
1412c5f65: mov qword ptr [rbp - 0x30], rax
1412c5f69: mov qword ptr [rbp - 0x28], 3
1412c5f71: mov qword ptr [rbp - 0x10], 0
1412c5f79: lea rax, [rbp + 0x10]
1412c5f7d: mov qword ptr [rbp - 0x20], rax
1412c5f81: mov qword ptr [rbp - 0x18], 3
1412c5f89: lea rdx, [rip + 0x1967640]
1412c5f90: lea rcx, [rbp - 0x30]
1412c5f94: call 0x1428d9390
1412c5f99: int3 
1412c5f9a: nop word ptr [rax + rax]
1412c5fa0: mov qword ptr [rsp + 0x10], rdx
1412c5fa5: push rbp
1412c5fa6: push rsi
1412c5fa7: sub rsp, 0x28
1412c5fab: lea rbp, [rdx + 0x80]
1412c5fb2: lea rcx, [rbp - 0x58]
1412c5fb6: call 0x140be24b0
1412c5fbb: nop 
1412c5fbc: add rsp, 0x28
1412c5fc0: pop rsi
1412c5fc1: pop rbp
1412c5fc2: ret 
1412c5fc3: nop word ptr cs:[rax + rax]
1412c5fd0: mov qword ptr [rsp + 0x10], rdx
1412c5fd5: push rbp
1412c5fd6: push rsi
1412c5fd7: sub rsp, 0x28
1412c5fdb: lea rbp, [rdx + 0x80]
1412c5fe2: lea rcx, [rbp + 0x10]
1412c5fe6: call 0x1400c04b0
1412c5feb: nop 
1412c5fec: add rsp, 0x28
1412c5ff0: pop rsi
1412c5ff1: pop rbp
1412c5ff2: ret 
