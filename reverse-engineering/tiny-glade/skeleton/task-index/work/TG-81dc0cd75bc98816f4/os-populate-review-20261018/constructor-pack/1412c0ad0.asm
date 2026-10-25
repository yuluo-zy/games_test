1412c0ad0: push rbp
1412c0ad1: push rsi
1412c0ad2: sub rsp, 0xe8
1412c0ad9: lea rbp, [rsp + 0x80]
1412c0ae1: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c0ae9: mov rsi, rcx
1412c0aec: lea rcx, [rbp - 0x58]
1412c0af0: xor r8d, r8d
1412c0af3: call 0x14161b0f0
1412c0af8: movzx eax, byte ptr [rip + 0x237410a]
1412c0aff: mov ecx, 0x38
1412c0b04: mov edx, 8
1412c0b09: call 0x140613c10
1412c0b0e: test rax, rax
1412c0b11: je 0x1412c0bcf
1412c0b17: mov rcx, qword ptr [rbp - 0x28]
1412c0b1b: mov qword ptr [rax + 0x30], rcx
1412c0b1f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c0b23: movups xmm1, xmmword ptr [rbp - 0x48]
1412c0b27: movups xmm2, xmmword ptr [rbp - 0x38]
1412c0b2b: movups xmmword ptr [rax + 0x20], xmm2
1412c0b2f: movups xmmword ptr [rax + 0x10], xmm1
1412c0b33: movups xmmword ptr [rax], xmm0
1412c0b36: lea rcx, [rip + 0x17dd24f]
1412c0b3d: mov qword ptr [rbp + 0x10], rcx
1412c0b41: mov qword ptr [rbp + 0x18], 0x46
1412c0b49: mov qword ptr [rbp + 0x20], rax
1412c0b4d: lea rax, [rip + 0x196c084]
1412c0b54: mov qword ptr [rbp + 0x28], rax
1412c0b58: mov qword ptr [rbp + 0x40], 0
1412c0b60: mov byte ptr [rbp + 0x56], 5
1412c0b64: mov qword ptr [rbp + 0x30], rcx
1412c0b68: mov qword ptr [rbp + 0x38], 0x46
1412c0b70: movzx eax, byte ptr [rip + 0x2374092]
1412c0b77: mov ecx, 0x40
1412c0b7c: mov edx, 8
1412c0b81: call 0x140613c10
1412c0b86: test rax, rax
1412c0b89: je 0x1412c0be0
1412c0b8b: movups xmm0, xmmword ptr [rbp + 0x20]
1412c0b8f: movups xmm1, xmmword ptr [rbp + 0x30]
1412c0b93: movups xmm2, xmmword ptr [rbp + 0x40]
1412c0b97: movups xmm3, xmmword ptr [rbp + 0x50]
1412c0b9b: movups xmmword ptr [rax + 0x30], xmm3
1412c0b9f: movups xmmword ptr [rax + 0x20], xmm2
1412c0ba3: movups xmmword ptr [rax + 0x10], xmm1
1412c0ba7: movups xmmword ptr [rax], xmm0
1412c0baa: movabs rdx, 0x61619a2e1d85da94
1412c0bb4: mov rcx, rsi
1412c0bb7: mov r8, rax
1412c0bba: call 0x140969bf0
1412c0bbf: cmp rax, 1
1412c0bc3: je 0x1412c0bf1
1412c0bc5: add rsp, 0xe8
1412c0bcc: pop rsi
1412c0bcd: pop rbp
1412c0bce: ret 
1412c0bcf: mov ecx, 8
1412c0bd4: mov edx, 0x38
1412c0bd9: call 0x1428d9000
1412c0bde: jmp 0x1412c0bef
1412c0be0: mov ecx, 8
1412c0be5: mov edx, 0x40
1412c0bea: call 0x1428d9000
1412c0bef: ud2 
1412c0bf1: mov rax, rdx
1412c0bf4: add rdx, 0x10
1412c0bf8: mov rsi, rdx
1412c0bfb: cmp qword ptr [rax + 0x18], 0x46
1412c0c00: jne 0x1412c0c7f
1412c0c02: mov rdx, qword ptr [rsi]
1412c0c05: lea rcx, [rip + 0x17dd180]
1412c0c0c: mov r8d, 0x46
1412c0c12: call 0x142923bf0
1412c0c17: test eax, eax
1412c0c19: jne 0x1412c0c7f
1412c0c1b: lea rax, [rip + 0x196cf46]
1412c0c22: mov qword ptr [rbp - 0x20], rax
1412c0c26: lea rax, [rip - 0xd8d38d]
1412c0c2d: mov qword ptr [rbp - 0x18], rax
1412c0c31: lea rax, [rbp + 0x10]
1412c0c35: mov qword ptr [rbp - 0x10], rax
1412c0c39: lea rax, [rip - 0x11b6db0]
1412c0c40: mov qword ptr [rbp - 8], rax
1412c0c44: lea rax, [rip + 0x196c9dd]
1412c0c4b: mov qword ptr [rbp + 0x20], rax
1412c0c4f: mov qword ptr [rbp + 0x28], 3
1412c0c57: mov qword ptr [rbp + 0x40], 0
1412c0c5f: lea rax, [rbp - 0x20]
1412c0c63: mov qword ptr [rbp + 0x30], rax
1412c0c67: mov qword ptr [rbp + 0x38], 2
1412c0c6f: lea rdx, [rip + 0x196c9e2]
1412c0c76: lea rcx, [rbp + 0x20]
1412c0c7a: call 0x1428d9390
1412c0c7f: lea rax, [rip + 0x196cee2]
1412c0c86: mov qword ptr [rbp + 0x20], rax
1412c0c8a: lea rax, [rip - 0xd8d3f1]
1412c0c91: mov qword ptr [rbp + 0x28], rax
1412c0c95: mov qword ptr [rbp + 0x30], rsi
1412c0c99: lea rax, [rip - 0x11b6e10]
1412c0ca0: mov qword ptr [rbp + 0x38], rax
1412c0ca4: lea rcx, [rbp + 0x10]
1412c0ca8: mov qword ptr [rbp + 0x40], rcx
1412c0cac: mov qword ptr [rbp + 0x48], rax
1412c0cb0: lea rax, [rip + 0x196c8b1]
1412c0cb7: mov qword ptr [rbp - 0x20], rax
1412c0cbb: mov qword ptr [rbp - 0x18], 3
1412c0cc3: mov qword ptr [rbp], 0
1412c0ccb: lea rax, [rbp + 0x20]
1412c0ccf: mov qword ptr [rbp - 0x10], rax
1412c0cd3: mov qword ptr [rbp - 8], 3
1412c0cdb: lea rdx, [rip + 0x196c8ee]
1412c0ce2: lea rcx, [rbp - 0x20]
1412c0ce6: call 0x1428d9390
1412c0ceb: int3 
1412c0cec: nop dword ptr [rax]
1412c0cf0: mov qword ptr [rsp + 0x10], rdx
1412c0cf5: push rbp
1412c0cf6: push rsi
1412c0cf7: sub rsp, 0x28
1412c0cfb: lea rbp, [rdx + 0x80]
1412c0d02: lea rcx, [rbp - 0x48]
1412c0d06: call 0x140be24b0
1412c0d0b: nop 
1412c0d0c: add rsp, 0x28
1412c0d10: pop rsi
1412c0d11: pop rbp
1412c0d12: ret 
1412c0d13: nop word ptr cs:[rax + rax]
1412c0d20: mov qword ptr [rsp + 0x10], rdx
1412c0d25: push rbp
1412c0d26: push rsi
1412c0d27: sub rsp, 0x28
1412c0d2b: lea rbp, [rdx + 0x80]
1412c0d32: lea rcx, [rbp + 0x20]
1412c0d36: call 0x1400c04b0
1412c0d3b: nop 
1412c0d3c: add rsp, 0x28
1412c0d40: pop rsi
1412c0d41: pop rbp
1412c0d42: ret 
