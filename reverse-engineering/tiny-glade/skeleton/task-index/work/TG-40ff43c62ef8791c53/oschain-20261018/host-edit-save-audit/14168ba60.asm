14168ba60: push rbp
14168ba61: push rsi
14168ba62: push rdi
14168ba63: sub rsp, 0x180
14168ba6a: lea rbp, [rsp + 0x80]
14168ba72: mov qword ptr [rbp + 0xf8], 0xfffffffffffffffe
14168ba7d: mov rsi, rcx
14168ba80: xorps xmm0, xmm0
14168ba83: movups xmmword ptr [rbp - 0x28], xmm0
14168ba87: movups xmmword ptr [rbp - 0x10], xmm0
14168ba8b: movups xmmword ptr [rbp + 8], xmm0
14168ba8f: movups xmmword ptr [rbp + 0x20], xmm0
14168ba93: movups xmmword ptr [rbp + 0x38], xmm0
14168ba97: mov dword ptr [rbp + 0x48], 0
14168ba9e: movups xmmword ptr [rbp + 0x70], xmm0
14168baa2: movups xmmword ptr [rbp + 0x88], xmm0
14168baa9: movups xmmword ptr [rbp + 0xa0], xmm0
14168bab0: movups xmmword ptr [rbp + 0xb8], xmm0
14168bab7: movups xmmword ptr [rbp + 0xd0], xmm0
14168babe: mov dword ptr [rbp + 0xe0], 0
14168bac8: mov qword ptr [rbp - 0x60], 0
14168bad0: mov qword ptr [rbp - 0x48], 0
14168bad8: mov qword ptr [rbp - 0x40], 8
14168bae0: mov qword ptr [rbp - 0x38], 0
14168bae8: mov qword ptr [rbp - 0x30], 0x10
14168baf0: mov qword ptr [rbp - 0x18], 0x10
14168baf8: mov qword ptr [rbp], 0x10
14168bb00: mov qword ptr [rbp + 0x18], 0x10
14168bb08: mov qword ptr [rbp + 0x30], 0x10
14168bb10: movabs rax, 0x8000000000000000
14168bb1a: mov qword ptr [rbp + 0x50], rax
14168bb1e: lea rax, [rip + 0x15fa8a6]
14168bb25: mov qword ptr [rbp + 0x58], rax
14168bb29: mov qword ptr [rbp + 0x60], 0x56
14168bb31: mov qword ptr [rbp + 0x68], 0x10
14168bb39: mov qword ptr [rbp + 0x80], 0x10
14168bb44: mov qword ptr [rbp + 0x98], 0x10
14168bb4f: mov qword ptr [rbp + 0xb0], 0x10
14168bb5a: mov qword ptr [rbp + 0xc8], 0x10
14168bb65: mov dword ptr [rbp + 0xe8], 0
14168bb6f: mov word ptr [rbp + 0xec], 1
14168bb78: mov dword ptr [rbp + 0xf0], 0
14168bb82: movzx eax, byte ptr [rip + 0x1fa9080]
14168bb89: mov ecx, 0x158
14168bb8e: mov edx, 8
14168bb93: call 0x140613c10
14168bb98: test rax, rax
14168bb9b: je 0x14168bbd2
14168bb9d: mov rdi, rax
14168bba0: lea rdx, [rbp - 0x60]
14168bba4: mov r8d, 0x158
14168bbaa: mov rcx, rax
14168bbad: call 0x142923c00
14168bbb2: lea r8, [rip + 0x15fa5b7]
14168bbb9: mov rcx, rsi
14168bbbc: mov rdx, rdi
14168bbbf: call 0x140ce4b10
14168bbc4: mov rax, rsi
14168bbc7: add rsp, 0x180
14168bbce: pop rdi
14168bbcf: pop rsi
14168bbd0: pop rbp
14168bbd1: ret 
14168bbd2: mov ecx, 8
14168bbd7: mov edx, 0x158
14168bbdc: call 0x1428d9000
14168bbe1: ud2 
14168bbe3: nop word ptr cs:[rax + rax]
14168bbf0: mov qword ptr [rsp + 0x10], rdx
14168bbf5: push rbp
14168bbf6: push rsi
14168bbf7: push rdi
14168bbf8: sub rsp, 0x20
14168bbfc: lea rbp, [rdx + 0x80]
14168bc03: lea rcx, [rbp - 0x60]
14168bc07: call 0x1405e5200
14168bc0c: nop 
14168bc0d: add rsp, 0x20
14168bc11: pop rdi
14168bc12: pop rsi
14168bc13: pop rbp
14168bc14: ret 
