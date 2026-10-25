1412c6a30: push rbp
1412c6a31: push rsi
1412c6a32: sub rsp, 0xe8
1412c6a39: lea rbp, [rsp + 0x80]
1412c6a41: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c6a49: mov rsi, rcx
1412c6a4c: lea rcx, [rbp - 0x58]
1412c6a50: xor r8d, r8d
1412c6a53: call 0x14161b870
1412c6a58: movzx eax, byte ptr [rip + 0x236e1aa]
1412c6a5f: mov ecx, 0x38
1412c6a64: mov edx, 8
1412c6a69: call 0x140613c10
1412c6a6e: test rax, rax
1412c6a71: je 0x1412c6b2f
1412c6a77: mov rcx, qword ptr [rbp - 0x28]
1412c6a7b: mov qword ptr [rax + 0x30], rcx
1412c6a7f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c6a83: movups xmm1, xmmword ptr [rbp - 0x48]
1412c6a87: movups xmm2, xmmword ptr [rbp - 0x38]
1412c6a8b: movups xmmword ptr [rax + 0x20], xmm2
1412c6a8f: movups xmmword ptr [rax + 0x10], xmm1
1412c6a93: movups xmmword ptr [rax], xmm0
1412c6a96: lea rcx, [rip + 0x17d782c]
1412c6a9d: mov qword ptr [rbp + 0x10], rcx
1412c6aa1: mov qword ptr [rbp + 0x18], 0x3a
1412c6aa9: mov qword ptr [rbp + 0x20], rax
1412c6aad: lea rax, [rip + 0x1966974]
1412c6ab4: mov qword ptr [rbp + 0x28], rax
1412c6ab8: mov qword ptr [rbp + 0x40], 0
1412c6ac0: mov byte ptr [rbp + 0x56], 5
1412c6ac4: mov qword ptr [rbp + 0x30], rcx
1412c6ac8: mov qword ptr [rbp + 0x38], 0x3a
1412c6ad0: movzx eax, byte ptr [rip + 0x236e132]
1412c6ad7: mov ecx, 0x40
1412c6adc: mov edx, 8
1412c6ae1: call 0x140613c10
1412c6ae6: test rax, rax
1412c6ae9: je 0x1412c6b40
1412c6aeb: movups xmm0, xmmword ptr [rbp + 0x20]
1412c6aef: movups xmm1, xmmword ptr [rbp + 0x30]
1412c6af3: movups xmm2, xmmword ptr [rbp + 0x40]
1412c6af7: movups xmm3, xmmword ptr [rbp + 0x50]
1412c6afb: movups xmmword ptr [rax + 0x30], xmm3
1412c6aff: movups xmmword ptr [rax + 0x20], xmm2
1412c6b03: movups xmmword ptr [rax + 0x10], xmm1
1412c6b07: movups xmmword ptr [rax], xmm0
1412c6b0a: movabs rdx, 0xca1400a37d9c0ee0
1412c6b14: mov rcx, rsi
1412c6b17: mov r8, rax
1412c6b1a: call 0x140969bf0
1412c6b1f: cmp rax, 1
1412c6b23: je 0x1412c6b51
1412c6b25: add rsp, 0xe8
1412c6b2c: pop rsi
1412c6b2d: pop rbp
1412c6b2e: ret 
1412c6b2f: mov ecx, 8
1412c6b34: mov edx, 0x38
1412c6b39: call 0x1428d9000
1412c6b3e: jmp 0x1412c6b4f
1412c6b40: mov ecx, 8
1412c6b45: mov edx, 0x40
1412c6b4a: call 0x1428d9000
1412c6b4f: ud2 
1412c6b51: mov rax, rdx
1412c6b54: add rdx, 0x10
1412c6b58: mov rsi, rdx
1412c6b5b: cmp qword ptr [rax + 0x18], 0x3a
1412c6b60: jne 0x1412c6bdf
1412c6b62: mov rdx, qword ptr [rsi]
1412c6b65: lea rcx, [rip + 0x17d775d]
1412c6b6c: mov r8d, 0x3a
1412c6b72: call 0x142923bf0
1412c6b77: test eax, eax
1412c6b79: jne 0x1412c6bdf
1412c6b7b: lea rax, [rip + 0x196709e]
1412c6b82: mov qword ptr [rbp - 0x20], rax
1412c6b86: lea rax, [rip - 0xd932ed]
1412c6b8d: mov qword ptr [rbp - 0x18], rax
1412c6b91: lea rax, [rbp + 0x10]
1412c6b95: mov qword ptr [rbp - 0x10], rax
1412c6b99: lea rax, [rip - 0x11bcd10]
1412c6ba0: mov qword ptr [rbp - 8], rax
1412c6ba4: lea rax, [rip + 0x1966a7d]
1412c6bab: mov qword ptr [rbp + 0x20], rax
1412c6baf: mov qword ptr [rbp + 0x28], 3
1412c6bb7: mov qword ptr [rbp + 0x40], 0
1412c6bbf: lea rax, [rbp - 0x20]
1412c6bc3: mov qword ptr [rbp + 0x30], rax
1412c6bc7: mov qword ptr [rbp + 0x38], 2
1412c6bcf: lea rdx, [rip + 0x1966a82]
1412c6bd6: lea rcx, [rbp + 0x20]
1412c6bda: call 0x1428d9390
1412c6bdf: lea rax, [rip + 0x196703a]
1412c6be6: mov qword ptr [rbp + 0x20], rax
1412c6bea: lea rax, [rip - 0xd93351]
1412c6bf1: mov qword ptr [rbp + 0x28], rax
1412c6bf5: mov qword ptr [rbp + 0x30], rsi
1412c6bf9: lea rax, [rip - 0x11bcd70]
1412c6c00: mov qword ptr [rbp + 0x38], rax
1412c6c04: lea rcx, [rbp + 0x10]
1412c6c08: mov qword ptr [rbp + 0x40], rcx
1412c6c0c: mov qword ptr [rbp + 0x48], rax
1412c6c10: lea rax, [rip + 0x1966951]
1412c6c17: mov qword ptr [rbp - 0x20], rax
1412c6c1b: mov qword ptr [rbp - 0x18], 3
1412c6c23: mov qword ptr [rbp], 0
1412c6c2b: lea rax, [rbp + 0x20]
1412c6c2f: mov qword ptr [rbp - 0x10], rax
1412c6c33: mov qword ptr [rbp - 8], 3
1412c6c3b: lea rdx, [rip + 0x196698e]
1412c6c42: lea rcx, [rbp - 0x20]
1412c6c46: call 0x1428d9390
1412c6c4b: int3 
1412c6c4c: nop dword ptr [rax]
1412c6c50: mov qword ptr [rsp + 0x10], rdx
1412c6c55: push rbp
1412c6c56: push rsi
1412c6c57: sub rsp, 0x28
1412c6c5b: lea rbp, [rdx + 0x80]
1412c6c62: lea rcx, [rbp - 0x48]
1412c6c66: call 0x140be24b0
1412c6c6b: nop 
1412c6c6c: add rsp, 0x28
1412c6c70: pop rsi
1412c6c71: pop rbp
1412c6c72: ret 
1412c6c73: nop word ptr cs:[rax + rax]
1412c6c80: mov qword ptr [rsp + 0x10], rdx
1412c6c85: push rbp
1412c6c86: push rsi
1412c6c87: sub rsp, 0x28
1412c6c8b: lea rbp, [rdx + 0x80]
1412c6c92: lea rcx, [rbp + 0x20]
1412c6c96: call 0x1400c04b0
1412c6c9b: nop 
1412c6c9c: add rsp, 0x28
1412c6ca0: pop rsi
1412c6ca1: pop rbp
1412c6ca2: ret 
