1414c0c40: push r15
1414c0c42: push r14
1414c0c44: push r13
1414c0c46: push r12
1414c0c48: push rsi
1414c0c49: push rdi
1414c0c4a: push rbp
1414c0c4b: push rbx
1414c0c4c: sub rsp, 0x148
1414c0c53: mov rdi, rdx
1414c0c56: mov rsi, rcx
1414c0c59: call 0x1413f2cd0
1414c0c5e: mov ebx, 1
1414c0c63: lock xadd dword ptr [rdi + 0x5b0], ebx
1414c0c6b: xor eax, eax
1414c0c6d: cmp rax, qword ptr [rsi + 0x168]
1414c0c74: jo 0x1414c10a0
1414c0c7a: mov rax, qword ptr [rsi + 0x138]
1414c0c81: mov r11, qword ptr [rdi + 0x218]
1414c0c88: cmp r11, rax
1414c0c8b: jbe 0x1414c0f8a
1414c0c91: mov r14, qword ptr [rdi + 0x210]
1414c0c98: mov rax, qword ptr [r14 + rax*8]
1414c0c9c: test rax, rax
1414c0c9f: je 0x1414c0f8a
1414c0ca5: mov rdx, qword ptr [rdi + 0x1e0]
1414c0cac: not rax
1414c0caf: lea rax, [rax + rax*2]
1414c0cb3: shl rax, 5
1414c0cb7: cmp qword ptr [rdx + rax + 0x20], 0
1414c0cbd: je 0x1414c0f8a
1414c0cc3: mov rcx, qword ptr [rsi + 0x140]
1414c0cca: cmp r11, rcx
1414c0ccd: jbe 0x1414c0fab
1414c0cd3: mov rcx, qword ptr [r14 + rcx*8]
1414c0cd7: test rcx, rcx
1414c0cda: je 0x1414c0fab
1414c0ce0: not rcx
1414c0ce3: lea rcx, [rcx + rcx*2]
1414c0ce7: shl rcx, 5
1414c0ceb: cmp qword ptr [rdx + rcx + 0x20], 0
1414c0cf1: je 0x1414c0fab
1414c0cf7: mov r8, qword ptr [rsi + 0x148]
1414c0cfe: cmp r11, r8
1414c0d01: jbe 0x1414c0fbb
1414c0d07: mov r8, qword ptr [r14 + r8*8]
1414c0d0b: test r8, r8
1414c0d0e: je 0x1414c0fbb
1414c0d14: not r8
1414c0d17: lea r8, [r8 + r8*2]
1414c0d1b: shl r8, 5
1414c0d1f: cmp qword ptr [rdx + r8 + 0x20], 0
1414c0d25: je 0x1414c0fbb
1414c0d2b: mov r9, qword ptr [rsi + 0x150]
1414c0d32: cmp r11, r9
1414c0d35: jbe 0x1414c0fd9
1414c0d3b: mov r9, qword ptr [r14 + r9*8]
1414c0d3f: test r9, r9
1414c0d42: je 0x1414c0fd9
1414c0d48: not r9
1414c0d4b: lea r9, [r9 + r9*2]
1414c0d4f: shl r9, 5
1414c0d53: cmp qword ptr [rdx + r9 + 0x20], 0
1414c0d59: je 0x1414c0fd9
1414c0d5f: mov r10, qword ptr [rsi + 0x158]
1414c0d66: cmp r11, r10
1414c0d69: jbe 0x1414c0ff7
1414c0d6f: mov r10, qword ptr [r14 + r10*8]
1414c0d73: test r10, r10
1414c0d76: je 0x1414c0ff7
1414c0d7c: not r10
1414c0d7f: lea r10, [r10 + r10*2]
1414c0d83: shl r10, 5
1414c0d87: cmp qword ptr [rdx + r10 + 0x20], 0
1414c0d8d: je 0x1414c0ff7
1414c0d93: mov r15, qword ptr [rsi + 0x160]
1414c0d9a: cmp r11, r15
1414c0d9d: jbe 0x1414c1015
1414c0da3: mov r11, qword ptr [r14 + r15*8]
1414c0da7: test r11, r11
1414c0daa: je 0x1414c1015
1414c0db0: not r11
1414c0db3: lea r11, [r11 + r11*2]
1414c0db7: shl r11, 5
1414c0dbb: cmp qword ptr [rdx + r11 + 0x20], 0
1414c0dc1: je 0x1414c1015
1414c0dc7: add rax, rdx
1414c0dca: mov rbp, qword ptr [rax + 0x10]
1414c0dce: add rcx, rdx
1414c0dd1: mov r13, qword ptr [rcx + 0x10]
1414c0dd5: add r8, rdx
1414c0dd8: mov r12, qword ptr [r8 + 0x10]
1414c0ddc: add r9, rdx
1414c0ddf: mov r15, qword ptr [r9 + 0x10]
1414c0de3: add r10, rdx
1414c0de6: mov r14, qword ptr [r10 + 0x10]
1414c0dea: add r11, rdx
1414c0ded: mov rdx, qword ptr [r11 + 0x10]
1414c0df1: mov qword ptr [rsp + 0x68], rbp
1414c0df6: lea rbp, [rax + 0x5c]
1414c0dfa: add rax, 0x58
1414c0dfe: mov qword ptr [rsp + 0x70], rax
1414c0e03: mov qword ptr [rsp + 0x78], rbp
1414c0e08: mov eax, dword ptr [rsi + 0x130]
1414c0e0e: mov dword ptr [rsp + 0x80], eax
1414c0e15: mov dword ptr [rsp + 0x84], ebx
1414c0e1c: mov qword ptr [rsp + 0x88], r13
1414c0e24: lea r13, [rcx + 0x5c]
1414c0e28: add rcx, 0x58
1414c0e2c: mov qword ptr [rsp + 0x90], rcx
1414c0e34: mov qword ptr [rsp + 0x98], r13
1414c0e3c: mov dword ptr [rsp + 0xa0], eax
1414c0e43: mov dword ptr [rsp + 0xa4], ebx
1414c0e4a: mov qword ptr [rsp + 0xa8], r12
1414c0e52: lea rcx, [r8 + 0x5c]
1414c0e56: add r8, 0x58
1414c0e5a: mov qword ptr [rsp + 0xb0], r8
1414c0e62: mov qword ptr [rsp + 0xb8], rcx
1414c0e6a: mov dword ptr [rsp + 0xc0], eax
1414c0e71: mov dword ptr [rsp + 0xc4], ebx
1414c0e78: mov qword ptr [rsp + 0xc8], r15
1414c0e80: lea rcx, [r9 + 0x5c]
1414c0e84: add r9, 0x58
1414c0e88: mov qword ptr [rsp + 0xd0], r9
1414c0e90: mov qword ptr [rsp + 0xd8], rcx
1414c0e98: mov dword ptr [rsp + 0xe0], eax
1414c0e9f: mov dword ptr [rsp + 0xe4], ebx
1414c0ea6: mov qword ptr [rsp + 0xe8], r14
1414c0eae: lea rcx, [r10 + 0x5c]
1414c0eb2: add r10, 0x58
1414c0eb6: mov qword ptr [rsp + 0xf0], r10
1414c0ebe: lea r10, [rsi + 0x168]
1414c0ec5: mov qword ptr [rsp + 0xf8], rcx
1414c0ecd: lea rcx, [r11 + 0x58]
1414c0ed1: add r11, 0x5c
1414c0ed5: mov dword ptr [rsp + 0x100], eax
1414c0edc: mov dword ptr [rsp + 0x104], ebx
1414c0ee3: mov qword ptr [rsp + 0x108], rdx
1414c0eeb: lea rdx, [rsp + 0x88]
1414c0ef3: mov qword ptr [rsp + 0x110], rcx
1414c0efb: lea r8, [rsp + 0xa8]
1414c0f03: mov qword ptr [rsp + 0x118], r11
1414c0f0b: lea r9, [rsp + 0xc8]
1414c0f13: mov dword ptr [rsp + 0x120], eax
1414c0f1a: lea rax, [rsp + 0xe8]
1414c0f22: mov dword ptr [rsp + 0x124], ebx
1414c0f29: mov qword ptr [rsp + 0x128], rdi
1414c0f31: lea rcx, [rsp + 0x108]
1414c0f39: mov qword ptr [rsp + 0x130], 0
1414c0f45: mov qword ptr [rsp + 0x138], r10
1414c0f4d: lea r10, [rsp + 0x128]
1414c0f55: mov qword ptr [rsp + 0x30], r10
1414c0f5a: mov qword ptr [rsp + 0x28], rcx
1414c0f5f: mov qword ptr [rsp + 0x20], rax
1414c0f64: lea rcx, [rsp + 0x68]
1414c0f69: call 0x140926bf0
1414c0f6e: mov dword ptr [rsi + 0x130], ebx
1414c0f74: xor eax, eax
1414c0f76: add rsp, 0x148
1414c0f7d: pop rbx
1414c0f7e: pop rbp
1414c0f7f: pop rdi
1414c0f80: pop rsi
1414c0f81: pop r12
1414c0f83: pop r13
1414c0f85: pop r14
1414c0f87: pop r15
1414c0f89: ret 
1414c0f8a: add rsi, 0x98
1414c0f91: lea rax, [rip + 0x17a5f53]
1414c0f98: mov qword ptr [rsp + 0x38], rax
1414c0f9d: mov qword ptr [rsp + 0x40], 0x2c
1414c0fa6: jmp 0x1414c1031
1414c0fab: add rsi, 0x98
1414c0fb2: lea rax, [rip + 0x17a59c8]
1414c0fb9: jmp 0x1414c1005
1414c0fbb: add rsi, 0x98
1414c0fc2: lea rax, [rip + 0x17a74e2]
1414c0fc9: mov qword ptr [rsp + 0x38], rax
1414c0fce: mov qword ptr [rsp + 0x40], 0x33
1414c0fd7: jmp 0x1414c1031
1414c0fd9: add rsi, 0x98
1414c0fe0: lea rax, [rip + 0x17a8544]
1414c0fe7: mov qword ptr [rsp + 0x38], rax
1414c0fec: mov qword ptr [rsp + 0x40], 0x41
1414c0ff5: jmp 0x1414c1031
1414c0ff7: add rsi, 0x98
1414c0ffe: lea rax, [rip + 0x17a7a1b]
1414c1005: mov qword ptr [rsp + 0x38], rax
1414c100a: mov qword ptr [rsp + 0x40], 0x3d
1414c1013: jmp 0x1414c1031
1414c1015: add rsi, 0x98
1414c101c: lea rax, [rip + 0x17a71ae]
1414c1023: mov qword ptr [rsp + 0x38], rax
1414c1028: mov qword ptr [rsp + 0x40], 0x35
1414c1031: mov qword ptr [rsp + 0x48], rsi
1414c1036: lea rax, [rip - 0x148556d]
1414c103d: mov qword ptr [rsp + 0x50], rax
1414c1042: lea rax, [rsp + 0x38]
1414c1047: mov qword ptr [rsp + 0x58], rax
1414c104c: lea rax, [rip - 0x13b71c3]
1414c1053: mov qword ptr [rsp + 0x60], rax
1414c1058: lea rax, [rip + 0x17a4e21]
1414c105f: mov qword ptr [rsp + 0x68], rax
1414c1064: mov qword ptr [rsp + 0x70], 2
1414c106d: mov qword ptr [rsp + 0x88], 0
1414c1079: lea rax, [rsp + 0x48]
1414c107e: mov qword ptr [rsp + 0x78], rax
1414c1083: mov qword ptr [rsp + 0x80], 2
1414c108f: lea rdx, [rip + 0x17a4e72]
1414c1096: lea rcx, [rsp + 0x68]
1414c109b: call 0x1428d9390
1414c10a0: lea rcx, [rip + 0x17aa89a]
1414c10a7: lea r8, [rip + 0x17aa95a]
1414c10ae: mov edx, 0x59
1414c10b3: call 0x1428d9330
1414c10b8: int3 
