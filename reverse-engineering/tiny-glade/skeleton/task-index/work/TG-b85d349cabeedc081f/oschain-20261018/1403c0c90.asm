1403c0c90: push rbp
1403c0c91: push rsi
1403c0c92: push rdi
1403c0c93: sub rsp, 0x190
1403c0c9a: lea rbp, [rsp + 0x80]
1403c0ca2: mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
1403c0cad: mov rsi, rcx
1403c0cb0: xorps xmm0, xmm0
1403c0cb3: movups xmmword ptr [rbp - 0x18], xmm0
1403c0cb7: movups xmmword ptr [rbp], xmm0
1403c0cbb: movups xmmword ptr [rbp + 0x18], xmm0
1403c0cbf: movups xmmword ptr [rbp + 0x30], xmm0
1403c0cc3: movups xmmword ptr [rbp + 0x48], xmm0
1403c0cc7: mov dword ptr [rbp + 0x58], 0
1403c0cce: movups xmmword ptr [rbp + 0x80], xmm0
1403c0cd5: movups xmmword ptr [rbp + 0x98], xmm0
1403c0cdc: movups xmmword ptr [rbp + 0xb0], xmm0
1403c0ce3: movups xmmword ptr [rbp + 0xc8], xmm0
1403c0cea: movups xmmword ptr [rbp + 0xe0], xmm0
1403c0cf1: mov dword ptr [rbp + 0xf0], 0
1403c0cfb: mov qword ptr [rbp - 0x58], 0
1403c0d03: mov qword ptr [rbp - 0x38], 0
1403c0d0b: mov qword ptr [rbp - 0x30], 8
1403c0d13: mov qword ptr [rbp - 0x28], 0
1403c0d1b: mov qword ptr [rbp - 0x20], 0x10
1403c0d23: mov qword ptr [rbp - 8], 0x10
1403c0d2b: mov qword ptr [rbp + 0x10], 0x10
1403c0d33: mov qword ptr [rbp + 0x28], 0x10
1403c0d3b: mov qword ptr [rbp + 0x40], 0x10
1403c0d43: movabs rax, 0x8000000000000000
1403c0d4d: mov qword ptr [rbp + 0x60], rax
1403c0d51: lea rax, [rip + 0x260d518]
1403c0d58: mov qword ptr [rbp + 0x68], rax
1403c0d5c: mov qword ptr [rbp + 0x70], 0x5b
1403c0d64: mov qword ptr [rbp + 0x78], 0x10
1403c0d6c: mov qword ptr [rbp + 0x90], 0x10
1403c0d77: mov qword ptr [rbp + 0xa8], 0x10
1403c0d82: mov qword ptr [rbp + 0xc0], 0x10
1403c0d8d: mov qword ptr [rbp + 0xd8], 0x10
1403c0d98: mov dword ptr [rbp + 0xf8], 0
1403c0da2: mov word ptr [rbp + 0xfc], 1
1403c0dab: mov dword ptr [rbp + 0x100], 0
1403c0db5: movzx eax, byte ptr [rip + 0x3273e4d]
1403c0dbc: mov ecx, 0x160
1403c0dc1: mov edx, 8
1403c0dc6: call 0x140613c10
1403c0dcb: test rax, rax
1403c0dce: je 0x1403c0e05
1403c0dd0: mov rdi, rax
1403c0dd3: lea rdx, [rbp - 0x58]
1403c0dd7: mov r8d, 0x160
1403c0ddd: mov rcx, rax
1403c0de0: call 0x142923c00
1403c0de5: lea r8, [rip + 0x260d314]
1403c0dec: mov rcx, rsi
1403c0def: mov rdx, rdi
1403c0df2: call 0x140ce4b10
1403c0df7: mov rax, rsi
1403c0dfa: add rsp, 0x190
1403c0e01: pop rdi
1403c0e02: pop rsi
1403c0e03: pop rbp
1403c0e04: ret 
1403c0e05: mov ecx, 8
1403c0e0a: mov edx, 0x160
1403c0e0f: call 0x1428d9000
1403c0e14: ud2 
1403c0e16: nop word ptr cs:[rax + rax]
1403c0e20: mov qword ptr [rsp + 0x10], rdx
1403c0e25: push rbp
1403c0e26: push rsi
1403c0e27: push rdi
1403c0e28: sub rsp, 0x20
1403c0e2c: lea rbp, [rdx + 0x80]
1403c0e33: lea rcx, [rbp - 0x58]
1403c0e37: call 0x1405e5e70
1403c0e3c: nop 
1403c0e3d: add rsp, 0x20
1403c0e41: pop rdi
1403c0e42: pop rsi
1403c0e43: pop rbp
1403c0e44: ret 
