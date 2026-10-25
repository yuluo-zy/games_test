1403c0e50: push rbp
1403c0e51: push rsi
1403c0e52: push rdi
1403c0e53: sub rsp, 0x1e0
1403c0e5a: lea rbp, [rsp + 0x80]
1403c0e62: mov qword ptr [rbp + 0x158], 0xfffffffffffffffe
1403c0e6d: mov rsi, rcx
1403c0e70: xorps xmm0, xmm0
1403c0e73: movups xmmword ptr [rbp + 0x38], xmm0
1403c0e77: movups xmmword ptr [rbp + 0x50], xmm0
1403c0e7b: movups xmmword ptr [rbp + 0x68], xmm0
1403c0e7f: movups xmmword ptr [rbp + 0x80], xmm0
1403c0e86: movups xmmword ptr [rbp + 0x98], xmm0
1403c0e8d: mov dword ptr [rbp + 0xa8], 0
1403c0e97: movups xmmword ptr [rbp + 0xd0], xmm0
1403c0e9e: movups xmmword ptr [rbp + 0xe8], xmm0
1403c0ea5: movups xmmword ptr [rbp + 0x100], xmm0
1403c0eac: movups xmmword ptr [rbp + 0x118], xmm0
1403c0eb3: movups xmmword ptr [rbp + 0x130], xmm0
1403c0eba: mov dword ptr [rbp + 0x140], 0
1403c0ec4: mov qword ptr [rbp - 0x60], 0
1403c0ecc: mov qword ptr [rbp + 0x18], 0
1403c0ed4: mov qword ptr [rbp + 0x20], 8
1403c0edc: mov qword ptr [rbp + 0x28], 0
1403c0ee4: mov qword ptr [rbp + 0x30], 0x10
1403c0eec: mov qword ptr [rbp + 0x48], 0x10
1403c0ef4: mov qword ptr [rbp + 0x60], 0x10
1403c0efc: mov qword ptr [rbp + 0x78], 0x10
1403c0f04: mov qword ptr [rbp + 0x90], 0x10
1403c0f0f: movabs rax, 0x8000000000000000
1403c0f19: mov qword ptr [rbp + 0xb0], rax
1403c0f20: lea rax, [rip + 0x260d3a4]
1403c0f27: mov qword ptr [rbp + 0xb8], rax
1403c0f2e: mov qword ptr [rbp + 0xc0], 0x2e
1403c0f39: mov qword ptr [rbp + 0xc8], 0x10
1403c0f44: mov qword ptr [rbp + 0xe0], 0x10
1403c0f4f: mov qword ptr [rbp + 0xf8], 0x10
1403c0f5a: mov qword ptr [rbp + 0x110], 0x10
1403c0f65: mov qword ptr [rbp + 0x128], 0x10
1403c0f70: mov dword ptr [rbp + 0x148], 0
1403c0f7a: mov word ptr [rbp + 0x14c], 1
1403c0f83: mov dword ptr [rbp + 0x150], 0
1403c0f8d: movzx eax, byte ptr [rip + 0x3273c75]
1403c0f94: mov ecx, 0x1b8
1403c0f99: mov edx, 8
1403c0f9e: call 0x140613c10
1403c0fa3: test rax, rax
1403c0fa6: je 0x1403c0fdd
1403c0fa8: mov rdi, rax
1403c0fab: lea rdx, [rbp - 0x60]
1403c0faf: mov r8d, 0x1b8
1403c0fb5: mov rcx, rax
1403c0fb8: call 0x142923c00
1403c0fbd: lea r8, [rip + 0x260d1f4]
1403c0fc4: mov rcx, rsi
1403c0fc7: mov rdx, rdi
1403c0fca: call 0x140ce4b10
1403c0fcf: mov rax, rsi
1403c0fd2: add rsp, 0x1e0
1403c0fd9: pop rdi
1403c0fda: pop rsi
1403c0fdb: pop rbp
1403c0fdc: ret 
1403c0fdd: mov ecx, 8
1403c0fe2: mov edx, 0x1b8
1403c0fe7: call 0x1428d9000
1403c0fec: ud2 
1403c0fee: nop 
1403c0ff0: mov qword ptr [rsp + 0x10], rdx
1403c0ff5: push rbp
1403c0ff6: push rsi
1403c0ff7: push rdi
1403c0ff8: sub rsp, 0x20
1403c0ffc: lea rbp, [rdx + 0x80]
1403c1003: lea rcx, [rbp - 0x60]
1403c1007: call 0x1405db700
1403c100c: nop 
1403c100d: add rsp, 0x20
1403c1011: pop rdi
1403c1012: pop rsi
1403c1013: pop rbp
1403c1014: ret 
