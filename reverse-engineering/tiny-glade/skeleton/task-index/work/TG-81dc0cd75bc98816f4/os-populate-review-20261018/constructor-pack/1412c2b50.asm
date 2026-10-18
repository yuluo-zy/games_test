1412c2b50: push rbp
1412c2b51: push rsi
1412c2b52: sub rsp, 0xd8
1412c2b59: lea rbp, [rsp + 0x80]
1412c2b61: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c2b69: mov rsi, rcx
1412c2b6c: lea rcx, [rbp - 0x58]
1412c2b70: xor r8d, r8d
1412c2b73: call 0x141611730
1412c2b78: movzx eax, byte ptr [rip + 0x237208a]
1412c2b7f: mov ecx, 0x28
1412c2b84: mov edx, 8
1412c2b89: call 0x140613c10
1412c2b8e: test rax, rax
1412c2b91: je 0x1412c2c47
1412c2b97: mov rcx, qword ptr [rbp - 0x38]
1412c2b9b: mov qword ptr [rax + 0x20], rcx
1412c2b9f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c2ba3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c2ba7: movups xmmword ptr [rax + 0x10], xmm1
1412c2bab: movups xmmword ptr [rax], xmm0
1412c2bae: lea rcx, [rip + 0x17daf0e]
1412c2bb5: mov qword ptr [rbp], rcx
1412c2bb9: mov qword ptr [rbp + 8], 0x49
1412c2bc1: mov qword ptr [rbp + 0x10], rax
1412c2bc5: lea rax, [rip + 0x196a2e4]
1412c2bcc: mov qword ptr [rbp + 0x18], rax
1412c2bd0: mov qword ptr [rbp + 0x30], 0
1412c2bd8: mov byte ptr [rbp + 0x46], 5
1412c2bdc: mov qword ptr [rbp + 0x20], rcx
1412c2be0: mov qword ptr [rbp + 0x28], 0x49
1412c2be8: movzx eax, byte ptr [rip + 0x237201a]
1412c2bef: mov ecx, 0x40
1412c2bf4: mov edx, 8
1412c2bf9: call 0x140613c10
1412c2bfe: test rax, rax
1412c2c01: je 0x1412c2c58
1412c2c03: movups xmm0, xmmword ptr [rbp + 0x10]
1412c2c07: movups xmm1, xmmword ptr [rbp + 0x20]
1412c2c0b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c2c0f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c2c13: movups xmmword ptr [rax + 0x30], xmm3
1412c2c17: movups xmmword ptr [rax + 0x20], xmm2
1412c2c1b: movups xmmword ptr [rax + 0x10], xmm1
1412c2c1f: movups xmmword ptr [rax], xmm0
1412c2c22: movabs rdx, 0xfe99a2349de64a50
1412c2c2c: mov rcx, rsi
1412c2c2f: mov r8, rax
1412c2c32: call 0x140969bf0
1412c2c37: cmp rax, 1
1412c2c3b: je 0x1412c2c69
1412c2c3d: add rsp, 0xd8
1412c2c44: pop rsi
1412c2c45: pop rbp
1412c2c46: ret 
1412c2c47: mov ecx, 8
1412c2c4c: mov edx, 0x28
1412c2c51: call 0x1428d9000
1412c2c56: jmp 0x1412c2c67
1412c2c58: mov ecx, 8
1412c2c5d: mov edx, 0x40
1412c2c62: call 0x1428d9000
1412c2c67: ud2 
1412c2c69: mov rax, rdx
1412c2c6c: add rdx, 0x10
1412c2c70: mov rsi, rdx
1412c2c73: cmp qword ptr [rax + 0x18], 0x49
1412c2c78: jne 0x1412c2cf6
1412c2c7a: mov rdx, qword ptr [rsi]
1412c2c7d: lea rcx, [rip + 0x17dae3f]
1412c2c84: mov r8d, 0x49
1412c2c8a: call 0x142923bf0
1412c2c8f: test eax, eax
1412c2c91: jne 0x1412c2cf6
1412c2c93: lea rax, [rip + 0x196af66]
1412c2c9a: mov qword ptr [rbp - 0x30], rax
1412c2c9e: lea rax, [rip - 0xd8f405]
1412c2ca5: mov qword ptr [rbp - 0x28], rax
1412c2ca9: mov rax, rbp
1412c2cac: mov qword ptr [rbp - 0x20], rax
1412c2cb0: lea rax, [rip - 0x11b8e27]
1412c2cb7: mov qword ptr [rbp - 0x18], rax
1412c2cbb: lea rax, [rip + 0x196a966]
1412c2cc2: mov qword ptr [rbp + 0x10], rax
1412c2cc6: mov qword ptr [rbp + 0x18], 3
1412c2cce: mov qword ptr [rbp + 0x30], 0
1412c2cd6: lea rax, [rbp - 0x30]
1412c2cda: mov qword ptr [rbp + 0x20], rax
1412c2cde: mov qword ptr [rbp + 0x28], 2
1412c2ce6: lea rdx, [rip + 0x196a96b]
1412c2ced: lea rcx, [rbp + 0x10]
1412c2cf1: call 0x1428d9390
1412c2cf6: lea rax, [rip + 0x196af03]
1412c2cfd: mov qword ptr [rbp + 0x10], rax
1412c2d01: lea rax, [rip - 0xd8f468]
1412c2d08: mov qword ptr [rbp + 0x18], rax
1412c2d0c: mov qword ptr [rbp + 0x20], rsi
1412c2d10: lea rax, [rip - 0x11b8e87]
1412c2d17: mov qword ptr [rbp + 0x28], rax
1412c2d1b: mov rcx, rbp
1412c2d1e: mov qword ptr [rbp + 0x30], rcx
1412c2d22: mov qword ptr [rbp + 0x38], rax
1412c2d26: lea rax, [rip + 0x196a83b]
1412c2d2d: mov qword ptr [rbp - 0x30], rax
1412c2d31: mov qword ptr [rbp - 0x28], 3
1412c2d39: mov qword ptr [rbp - 0x10], 0
1412c2d41: lea rax, [rbp + 0x10]
1412c2d45: mov qword ptr [rbp - 0x20], rax
1412c2d49: mov qword ptr [rbp - 0x18], 3
1412c2d51: lea rdx, [rip + 0x196a878]
1412c2d58: lea rcx, [rbp - 0x30]
1412c2d5c: call 0x1428d9390
1412c2d61: int3 
1412c2d62: nop word ptr cs:[rax + rax]
1412c2d70: mov qword ptr [rsp + 0x10], rdx
1412c2d75: push rbp
1412c2d76: push rsi
1412c2d77: sub rsp, 0x28
1412c2d7b: lea rbp, [rdx + 0x80]
1412c2d82: lea rcx, [rbp - 0x58]
1412c2d86: call 0x140be24b0
1412c2d8b: nop 
1412c2d8c: add rsp, 0x28
1412c2d90: pop rsi
1412c2d91: pop rbp
1412c2d92: ret 
1412c2d93: nop word ptr cs:[rax + rax]
1412c2da0: mov qword ptr [rsp + 0x10], rdx
1412c2da5: push rbp
1412c2da6: push rsi
1412c2da7: sub rsp, 0x28
1412c2dab: lea rbp, [rdx + 0x80]
1412c2db2: lea rcx, [rbp + 0x10]
1412c2db6: call 0x1400c04b0
1412c2dbb: nop 
1412c2dbc: add rsp, 0x28
1412c2dc0: pop rsi
1412c2dc1: pop rbp
1412c2dc2: ret 
