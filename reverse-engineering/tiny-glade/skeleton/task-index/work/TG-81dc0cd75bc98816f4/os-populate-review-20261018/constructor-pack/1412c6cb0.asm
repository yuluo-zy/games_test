1412c6cb0: push rbp
1412c6cb1: push rsi
1412c6cb2: sub rsp, 0xe8
1412c6cb9: lea rbp, [rsp + 0x80]
1412c6cc1: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c6cc9: mov rsi, rcx
1412c6ccc: lea rcx, [rbp - 0x58]
1412c6cd0: xor r8d, r8d
1412c6cd3: call 0x14161ca00
1412c6cd8: movzx eax, byte ptr [rip + 0x236df2a]
1412c6cdf: mov ecx, 0x38
1412c6ce4: mov edx, 8
1412c6ce9: call 0x140613c10
1412c6cee: test rax, rax
1412c6cf1: je 0x1412c6daf
1412c6cf7: mov rcx, qword ptr [rbp - 0x28]
1412c6cfb: mov qword ptr [rax + 0x30], rcx
1412c6cff: movups xmm0, xmmword ptr [rbp - 0x58]
1412c6d03: movups xmm1, xmmword ptr [rbp - 0x48]
1412c6d07: movups xmm2, xmmword ptr [rbp - 0x38]
1412c6d0b: movups xmmword ptr [rax + 0x20], xmm2
1412c6d0f: movups xmmword ptr [rax + 0x10], xmm1
1412c6d13: movups xmmword ptr [rax], xmm0
1412c6d16: lea rcx, [rip + 0x17d6395]
1412c6d1d: mov qword ptr [rbp + 0x10], rcx
1412c6d21: mov qword ptr [rbp + 0x18], 0x43
1412c6d29: mov qword ptr [rbp + 0x20], rax
1412c6d2d: lea rax, [rip + 0x196672c]
1412c6d34: mov qword ptr [rbp + 0x28], rax
1412c6d38: mov qword ptr [rbp + 0x40], 0
1412c6d40: mov byte ptr [rbp + 0x56], 5
1412c6d44: mov qword ptr [rbp + 0x30], rcx
1412c6d48: mov qword ptr [rbp + 0x38], 0x43
1412c6d50: movzx eax, byte ptr [rip + 0x236deb2]
1412c6d57: mov ecx, 0x40
1412c6d5c: mov edx, 8
1412c6d61: call 0x140613c10
1412c6d66: test rax, rax
1412c6d69: je 0x1412c6dc0
1412c6d6b: movups xmm0, xmmword ptr [rbp + 0x20]
1412c6d6f: movups xmm1, xmmword ptr [rbp + 0x30]
1412c6d73: movups xmm2, xmmword ptr [rbp + 0x40]
1412c6d77: movups xmm3, xmmword ptr [rbp + 0x50]
1412c6d7b: movups xmmword ptr [rax + 0x30], xmm3
1412c6d7f: movups xmmword ptr [rax + 0x20], xmm2
1412c6d83: movups xmmword ptr [rax + 0x10], xmm1
1412c6d87: movups xmmword ptr [rax], xmm0
1412c6d8a: movabs rdx, 0xb162982ccf7e16e0
1412c6d94: mov rcx, rsi
1412c6d97: mov r8, rax
1412c6d9a: call 0x140969bf0
1412c6d9f: cmp rax, 1
1412c6da3: je 0x1412c6dd1
1412c6da5: add rsp, 0xe8
1412c6dac: pop rsi
1412c6dad: pop rbp
1412c6dae: ret 
1412c6daf: mov ecx, 8
1412c6db4: mov edx, 0x38
1412c6db9: call 0x1428d9000
1412c6dbe: jmp 0x1412c6dcf
1412c6dc0: mov ecx, 8
1412c6dc5: mov edx, 0x40
1412c6dca: call 0x1428d9000
1412c6dcf: ud2 
1412c6dd1: mov rax, rdx
1412c6dd4: add rdx, 0x10
1412c6dd8: mov rsi, rdx
1412c6ddb: cmp qword ptr [rax + 0x18], 0x43
1412c6de0: jne 0x1412c6e5f
1412c6de2: mov rdx, qword ptr [rsi]
1412c6de5: lea rcx, [rip + 0x17d62c6]
1412c6dec: mov r8d, 0x43
1412c6df2: call 0x142923bf0
1412c6df7: test eax, eax
1412c6df9: jne 0x1412c6e5f
1412c6dfb: lea rax, [rip + 0x19668c6]
1412c6e02: mov qword ptr [rbp - 0x20], rax
1412c6e06: lea rax, [rip - 0xd9356d]
1412c6e0d: mov qword ptr [rbp - 0x18], rax
1412c6e11: lea rax, [rbp + 0x10]
1412c6e15: mov qword ptr [rbp - 0x10], rax
1412c6e19: lea rax, [rip - 0x11bcf90]
1412c6e20: mov qword ptr [rbp - 8], rax
1412c6e24: lea rax, [rip + 0x19667fd]
1412c6e2b: mov qword ptr [rbp + 0x20], rax
1412c6e2f: mov qword ptr [rbp + 0x28], 3
1412c6e37: mov qword ptr [rbp + 0x40], 0
1412c6e3f: lea rax, [rbp - 0x20]
1412c6e43: mov qword ptr [rbp + 0x30], rax
1412c6e47: mov qword ptr [rbp + 0x38], 2
1412c6e4f: lea rdx, [rip + 0x1966802]
1412c6e56: lea rcx, [rbp + 0x20]
1412c6e5a: call 0x1428d9390
1412c6e5f: lea rax, [rip + 0x1966862]
1412c6e66: mov qword ptr [rbp + 0x20], rax
1412c6e6a: lea rax, [rip - 0xd935d1]
1412c6e71: mov qword ptr [rbp + 0x28], rax
1412c6e75: mov qword ptr [rbp + 0x30], rsi
1412c6e79: lea rax, [rip - 0x11bcff0]
1412c6e80: mov qword ptr [rbp + 0x38], rax
1412c6e84: lea rcx, [rbp + 0x10]
1412c6e88: mov qword ptr [rbp + 0x40], rcx
1412c6e8c: mov qword ptr [rbp + 0x48], rax
1412c6e90: lea rax, [rip + 0x19666d1]
1412c6e97: mov qword ptr [rbp - 0x20], rax
1412c6e9b: mov qword ptr [rbp - 0x18], 3
1412c6ea3: mov qword ptr [rbp], 0
1412c6eab: lea rax, [rbp + 0x20]
1412c6eaf: mov qword ptr [rbp - 0x10], rax
1412c6eb3: mov qword ptr [rbp - 8], 3
1412c6ebb: lea rdx, [rip + 0x196670e]
1412c6ec2: lea rcx, [rbp - 0x20]
1412c6ec6: call 0x1428d9390
1412c6ecb: int3 
1412c6ecc: nop dword ptr [rax]
1412c6ed0: mov qword ptr [rsp + 0x10], rdx
1412c6ed5: push rbp
1412c6ed6: push rsi
1412c6ed7: sub rsp, 0x28
1412c6edb: lea rbp, [rdx + 0x80]
1412c6ee2: lea rcx, [rbp - 0x48]
1412c6ee6: call 0x140be24b0
1412c6eeb: nop 
1412c6eec: add rsp, 0x28
1412c6ef0: pop rsi
1412c6ef1: pop rbp
1412c6ef2: ret 
1412c6ef3: nop word ptr cs:[rax + rax]
1412c6f00: mov qword ptr [rsp + 0x10], rdx
1412c6f05: push rbp
1412c6f06: push rsi
1412c6f07: sub rsp, 0x28
1412c6f0b: lea rbp, [rdx + 0x80]
1412c6f12: lea rcx, [rbp + 0x20]
1412c6f16: call 0x1400c04b0
1412c6f1b: nop 
1412c6f1c: add rsp, 0x28
1412c6f20: pop rsi
1412c6f21: pop rbp
1412c6f22: ret 
