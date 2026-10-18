140a5fa20: push rbp
140a5fa21: push rsi
140a5fa22: push rdi
140a5fa23: push rbx
140a5fa24: sub rsp, 0x1f8
140a5fa2b: lea rbp, [rsp + 0x80]
140a5fa33: mov qword ptr [rbp + 0x170], 0xfffffffffffffffe
140a5fa3e: mov rsi, rcx
140a5fa41: xorps xmm0, xmm0
140a5fa44: movups xmmword ptr [rbp + 0x50], xmm0
140a5fa48: movups xmmword ptr [rbp + 0x68], xmm0
140a5fa4c: movups xmmword ptr [rbp + 0x80], xmm0
140a5fa53: movups xmmword ptr [rbp + 0x98], xmm0
140a5fa5a: movups xmmword ptr [rbp + 0xb0], xmm0
140a5fa61: mov dword ptr [rbp + 0xc0], 0
140a5fa6b: movups xmmword ptr [rbp + 0xe8], xmm0
140a5fa72: movups xmmword ptr [rbp + 0x100], xmm0
140a5fa79: movups xmmword ptr [rbp + 0x118], xmm0
140a5fa80: movups xmmword ptr [rbp + 0x130], xmm0
140a5fa87: movups xmmword ptr [rbp + 0x148], xmm0
140a5fa8e: mov dword ptr [rbp + 0x158], 0
140a5fa98: mov qword ptr [rbp + 0x18], 0
140a5faa0: mov qword ptr [rbp + 0x30], 0
140a5faa8: mov qword ptr [rbp + 0x38], 8
140a5fab0: mov qword ptr [rbp + 0x40], 0
140a5fab8: mov qword ptr [rbp + 0x48], 0x10
140a5fac0: mov qword ptr [rbp + 0x60], 0x10
140a5fac8: mov qword ptr [rbp + 0x78], 0x10
140a5fad0: mov qword ptr [rbp + 0x90], 0x10
140a5fadb: mov qword ptr [rbp + 0xa8], 0x10
140a5fae6: movabs rax, 0x8000000000000000
140a5faf0: mov qword ptr [rbp + 0xc8], rax
140a5faf7: lea rax, [rip + 0x20742df]
140a5fafe: mov qword ptr [rbp + 0xd0], rax
140a5fb05: mov qword ptr [rbp + 0xd8], 0x2d
140a5fb10: mov qword ptr [rbp + 0xe0], 0x10
140a5fb1b: mov qword ptr [rbp + 0xf8], 0x10
140a5fb26: mov qword ptr [rbp + 0x110], 0x10
140a5fb31: mov qword ptr [rbp + 0x128], 0x10
140a5fb3c: mov qword ptr [rbp + 0x140], 0x10
140a5fb47: mov dword ptr [rbp + 0x160], 0
140a5fb51: mov word ptr [rbp + 0x164], 1
140a5fb5a: mov dword ptr [rbp + 0x168], 0
140a5fb64: movzx eax, byte ptr [rip + 0x2bd509e]
140a5fb6b: mov ecx, 0x158
140a5fb70: mov edx, 8
140a5fb75: call 0x140613c10
140a5fb7a: test rax, rax
140a5fb7d: je 0x140a5fbc4
140a5fb7f: mov rdi, rax
140a5fb82: lea rdx, [rbp + 0x18]
140a5fb86: mov r8d, 0x158
140a5fb8c: mov rcx, rax
140a5fb8f: call 0x142923c00
140a5fb94: lea r8, [rip + 0x2073e8d]
140a5fb9b: lea rbx, [rbp - 0x58]
140a5fb9f: mov rcx, rbx
140a5fba2: mov rdx, rdi
140a5fba5: call 0x140ce4b10
140a5fbaa: mov rcx, rsi
140a5fbad: mov rdx, rbx
140a5fbb0: call 0x14144d380
140a5fbb5: mov rax, rsi
140a5fbb8: add rsp, 0x1f8
140a5fbbf: pop rbx
140a5fbc0: pop rdi
140a5fbc1: pop rsi
140a5fbc2: pop rbp
140a5fbc3: ret 
140a5fbc4: mov ecx, 8
140a5fbc9: mov edx, 0x158
140a5fbce: call 0x1428d9000
140a5fbd3: ud2 
140a5fbd5: nop word ptr cs:[rax + rax]
140a5fbe0: mov qword ptr [rsp + 0x10], rdx
140a5fbe5: push rbp
140a5fbe6: push rsi
140a5fbe7: push rdi
140a5fbe8: push rbx
140a5fbe9: sub rsp, 0x28
140a5fbed: lea rbp, [rdx + 0x80]
140a5fbf4: lea rcx, [rbp + 0x18]
140a5fbf8: call 0x1405e5200
140a5fbfd: nop 
140a5fbfe: add rsp, 0x28
140a5fc02: pop rbx
140a5fc03: pop rdi
140a5fc04: pop rsi
140a5fc05: pop rbp
140a5fc06: ret 
