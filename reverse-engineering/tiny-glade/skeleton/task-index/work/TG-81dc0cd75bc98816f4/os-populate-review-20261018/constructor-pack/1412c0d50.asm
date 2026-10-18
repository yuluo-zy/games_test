1412c0d50: push rbp
1412c0d51: push rsi
1412c0d52: sub rsp, 0xd8
1412c0d59: lea rbp, [rsp + 0x80]
1412c0d61: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c0d69: mov rsi, rcx
1412c0d6c: lea rcx, [rbp - 0x58]
1412c0d70: xor r8d, r8d
1412c0d73: call 0x141618a60
1412c0d78: mov qword ptr [rbp - 0x60], 0
1412c0d80: movzx eax, byte ptr [rip + 0x2373e82]
1412c0d87: mov ecx, 0x30
1412c0d8c: mov edx, 8
1412c0d91: call 0x140613c10
1412c0d96: test rax, rax
1412c0d99: je 0x1412c0e4f
1412c0d9f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c0da3: movups xmm1, xmmword ptr [rbp - 0x50]
1412c0da7: movups xmm2, xmmword ptr [rbp - 0x40]
1412c0dab: movups xmmword ptr [rax + 0x20], xmm2
1412c0daf: movups xmmword ptr [rax + 0x10], xmm1
1412c0db3: movups xmmword ptr [rax], xmm0
1412c0db6: lea rcx, [rip + 0x17dd969]
1412c0dbd: mov qword ptr [rbp], rcx
1412c0dc1: mov qword ptr [rbp + 8], 0x26
1412c0dc9: mov qword ptr [rbp + 0x10], rax
1412c0dcd: lea rax, [rip + 0x196be3c]
1412c0dd4: mov qword ptr [rbp + 0x18], rax
1412c0dd8: mov qword ptr [rbp + 0x30], 0
1412c0de0: mov byte ptr [rbp + 0x46], 5
1412c0de4: mov qword ptr [rbp + 0x20], rcx
1412c0de8: mov qword ptr [rbp + 0x28], 0x26
1412c0df0: movzx eax, byte ptr [rip + 0x2373e12]
1412c0df7: mov ecx, 0x40
1412c0dfc: mov edx, 8
1412c0e01: call 0x140613c10
1412c0e06: test rax, rax
1412c0e09: je 0x1412c0e60
1412c0e0b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c0e0f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c0e13: movups xmm2, xmmword ptr [rbp + 0x30]
1412c0e17: movups xmm3, xmmword ptr [rbp + 0x40]
1412c0e1b: movups xmmword ptr [rax + 0x30], xmm3
1412c0e1f: movups xmmword ptr [rax + 0x20], xmm2
1412c0e23: movups xmmword ptr [rax + 0x10], xmm1
1412c0e27: movups xmmword ptr [rax], xmm0
1412c0e2a: movabs rdx, 0x282b72fc752587c0
1412c0e34: mov rcx, rsi
1412c0e37: mov r8, rax
1412c0e3a: call 0x140969bf0
1412c0e3f: cmp rax, 1
1412c0e43: je 0x1412c0e71
1412c0e45: add rsp, 0xd8
1412c0e4c: pop rsi
1412c0e4d: pop rbp
1412c0e4e: ret 
1412c0e4f: mov ecx, 8
1412c0e54: mov edx, 0x30
1412c0e59: call 0x1428d9000
1412c0e5e: jmp 0x1412c0e6f
1412c0e60: mov ecx, 8
1412c0e65: mov edx, 0x40
1412c0e6a: call 0x1428d9000
1412c0e6f: ud2 
1412c0e71: mov rax, rdx
1412c0e74: add rdx, 0x10
1412c0e78: mov rsi, rdx
1412c0e7b: cmp qword ptr [rax + 0x18], 0x26
1412c0e80: jne 0x1412c0efe
1412c0e82: mov rdx, qword ptr [rsi]
1412c0e85: lea rcx, [rip + 0x17dd89a]
1412c0e8c: mov r8d, 0x26
1412c0e92: call 0x142923bf0
1412c0e97: test eax, eax
1412c0e99: jne 0x1412c0efe
1412c0e9b: lea rax, [rip + 0x196d0ae]
1412c0ea2: mov qword ptr [rbp - 0x30], rax
1412c0ea6: lea rax, [rip - 0xd8d60d]
1412c0ead: mov qword ptr [rbp - 0x28], rax
1412c0eb1: mov rax, rbp
1412c0eb4: mov qword ptr [rbp - 0x20], rax
1412c0eb8: lea rax, [rip - 0x11b702f]
1412c0ebf: mov qword ptr [rbp - 0x18], rax
1412c0ec3: lea rax, [rip + 0x196c75e]
1412c0eca: mov qword ptr [rbp + 0x10], rax
1412c0ece: mov qword ptr [rbp + 0x18], 3
1412c0ed6: mov qword ptr [rbp + 0x30], 0
1412c0ede: lea rax, [rbp - 0x30]
1412c0ee2: mov qword ptr [rbp + 0x20], rax
1412c0ee6: mov qword ptr [rbp + 0x28], 2
1412c0eee: lea rdx, [rip + 0x196c763]
1412c0ef5: lea rcx, [rbp + 0x10]
1412c0ef9: call 0x1428d9390
1412c0efe: lea rax, [rip + 0x196d04b]
1412c0f05: mov qword ptr [rbp + 0x10], rax
1412c0f09: lea rax, [rip - 0xd8d670]
1412c0f10: mov qword ptr [rbp + 0x18], rax
1412c0f14: mov qword ptr [rbp + 0x20], rsi
1412c0f18: lea rax, [rip - 0x11b708f]
1412c0f1f: mov qword ptr [rbp + 0x28], rax
1412c0f23: mov rcx, rbp
1412c0f26: mov qword ptr [rbp + 0x30], rcx
1412c0f2a: mov qword ptr [rbp + 0x38], rax
1412c0f2e: lea rax, [rip + 0x196c633]
1412c0f35: mov qword ptr [rbp - 0x30], rax
1412c0f39: mov qword ptr [rbp - 0x28], 3
1412c0f41: mov qword ptr [rbp - 0x10], 0
1412c0f49: lea rax, [rbp + 0x10]
1412c0f4d: mov qword ptr [rbp - 0x20], rax
1412c0f51: mov qword ptr [rbp - 0x18], 3
1412c0f59: lea rdx, [rip + 0x196c670]
1412c0f60: lea rcx, [rbp - 0x30]
1412c0f64: call 0x1428d9390
1412c0f69: int3 
1412c0f6a: nop word ptr [rax + rax]
1412c0f70: mov qword ptr [rsp + 0x10], rdx
1412c0f75: push rbp
1412c0f76: push rsi
1412c0f77: sub rsp, 0x28
1412c0f7b: lea rbp, [rdx + 0x80]
1412c0f82: lea rcx, [rbp - 0x58]
1412c0f86: call 0x140be24b0
1412c0f8b: nop 
1412c0f8c: add rsp, 0x28
1412c0f90: pop rsi
1412c0f91: pop rbp
1412c0f92: ret 
1412c0f93: nop word ptr cs:[rax + rax]
1412c0fa0: mov qword ptr [rsp + 0x10], rdx
1412c0fa5: push rbp
1412c0fa6: push rsi
1412c0fa7: sub rsp, 0x28
1412c0fab: lea rbp, [rdx + 0x80]
1412c0fb2: lea rcx, [rbp + 0x10]
1412c0fb6: call 0x1400c04b0
1412c0fbb: nop 
1412c0fbc: add rsp, 0x28
1412c0fc0: pop rsi
1412c0fc1: pop rbp
1412c0fc2: ret 
