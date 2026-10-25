1412c19d0: push rbp
1412c19d1: push rsi
1412c19d2: sub rsp, 0xd8
1412c19d9: lea rbp, [rsp + 0x80]
1412c19e1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c19e9: mov rsi, rcx
1412c19ec: lea rcx, [rbp - 0x58]
1412c19f0: xor r8d, r8d
1412c19f3: call 0x141611980
1412c19f8: movzx eax, byte ptr [rip + 0x237320a]
1412c19ff: mov ecx, 0x28
1412c1a04: mov edx, 8
1412c1a09: call 0x140613c10
1412c1a0e: test rax, rax
1412c1a11: je 0x1412c1ac7
1412c1a17: mov rcx, qword ptr [rbp - 0x38]
1412c1a1b: mov qword ptr [rax + 0x20], rcx
1412c1a1f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c1a23: movups xmm1, xmmword ptr [rbp - 0x48]
1412c1a27: movups xmmword ptr [rax + 0x10], xmm1
1412c1a2b: movups xmmword ptr [rax], xmm0
1412c1a2e: lea rcx, [rip + 0x17db841]
1412c1a35: mov qword ptr [rbp], rcx
1412c1a39: mov qword ptr [rbp + 8], 0x4d
1412c1a41: mov qword ptr [rbp + 0x10], rax
1412c1a45: lea rax, [rip + 0x196b2dc]
1412c1a4c: mov qword ptr [rbp + 0x18], rax
1412c1a50: mov qword ptr [rbp + 0x30], 0
1412c1a58: mov byte ptr [rbp + 0x46], 5
1412c1a5c: mov qword ptr [rbp + 0x20], rcx
1412c1a60: mov qword ptr [rbp + 0x28], 0x4d
1412c1a68: movzx eax, byte ptr [rip + 0x237319a]
1412c1a6f: mov ecx, 0x40
1412c1a74: mov edx, 8
1412c1a79: call 0x140613c10
1412c1a7e: test rax, rax
1412c1a81: je 0x1412c1ad8
1412c1a83: movups xmm0, xmmword ptr [rbp + 0x10]
1412c1a87: movups xmm1, xmmword ptr [rbp + 0x20]
1412c1a8b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c1a8f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c1a93: movups xmmword ptr [rax + 0x30], xmm3
1412c1a97: movups xmmword ptr [rax + 0x20], xmm2
1412c1a9b: movups xmmword ptr [rax + 0x10], xmm1
1412c1a9f: movups xmmword ptr [rax], xmm0
1412c1aa2: movabs rdx, 0x821b83720fc03446
1412c1aac: mov rcx, rsi
1412c1aaf: mov r8, rax
1412c1ab2: call 0x140969bf0
1412c1ab7: cmp rax, 1
1412c1abb: je 0x1412c1ae9
1412c1abd: add rsp, 0xd8
1412c1ac4: pop rsi
1412c1ac5: pop rbp
1412c1ac6: ret 
1412c1ac7: mov ecx, 8
1412c1acc: mov edx, 0x28
1412c1ad1: call 0x1428d9000
1412c1ad6: jmp 0x1412c1ae7
1412c1ad8: mov ecx, 8
1412c1add: mov edx, 0x40
1412c1ae2: call 0x1428d9000
1412c1ae7: ud2 
1412c1ae9: mov rax, rdx
1412c1aec: add rdx, 0x10
1412c1af0: mov rsi, rdx
1412c1af3: cmp qword ptr [rax + 0x18], 0x4d
1412c1af8: jne 0x1412c1b76
1412c1afa: mov rdx, qword ptr [rsi]
1412c1afd: lea rcx, [rip + 0x17db772]
1412c1b04: mov r8d, 0x4d
1412c1b0a: call 0x142923bf0
1412c1b0f: test eax, eax
1412c1b11: jne 0x1412c1b76
1412c1b13: lea rax, [rip + 0x196bc96]
1412c1b1a: mov qword ptr [rbp - 0x30], rax
1412c1b1e: lea rax, [rip - 0xd8e285]
1412c1b25: mov qword ptr [rbp - 0x28], rax
1412c1b29: mov rax, rbp
1412c1b2c: mov qword ptr [rbp - 0x20], rax
1412c1b30: lea rax, [rip - 0x11b7ca7]
1412c1b37: mov qword ptr [rbp - 0x18], rax
1412c1b3b: lea rax, [rip + 0x196bae6]
1412c1b42: mov qword ptr [rbp + 0x10], rax
1412c1b46: mov qword ptr [rbp + 0x18], 3
1412c1b4e: mov qword ptr [rbp + 0x30], 0
1412c1b56: lea rax, [rbp - 0x30]
1412c1b5a: mov qword ptr [rbp + 0x20], rax
1412c1b5e: mov qword ptr [rbp + 0x28], 2
1412c1b66: lea rdx, [rip + 0x196baeb]
1412c1b6d: lea rcx, [rbp + 0x10]
1412c1b71: call 0x1428d9390
1412c1b76: lea rax, [rip + 0x196bc33]
1412c1b7d: mov qword ptr [rbp + 0x10], rax
1412c1b81: lea rax, [rip - 0xd8e2e8]
1412c1b88: mov qword ptr [rbp + 0x18], rax
1412c1b8c: mov qword ptr [rbp + 0x20], rsi
1412c1b90: lea rax, [rip - 0x11b7d07]
1412c1b97: mov qword ptr [rbp + 0x28], rax
1412c1b9b: mov rcx, rbp
1412c1b9e: mov qword ptr [rbp + 0x30], rcx
1412c1ba2: mov qword ptr [rbp + 0x38], rax
1412c1ba6: lea rax, [rip + 0x196b9bb]
1412c1bad: mov qword ptr [rbp - 0x30], rax
1412c1bb1: mov qword ptr [rbp - 0x28], 3
1412c1bb9: mov qword ptr [rbp - 0x10], 0
1412c1bc1: lea rax, [rbp + 0x10]
1412c1bc5: mov qword ptr [rbp - 0x20], rax
1412c1bc9: mov qword ptr [rbp - 0x18], 3
1412c1bd1: lea rdx, [rip + 0x196b9f8]
1412c1bd8: lea rcx, [rbp - 0x30]
1412c1bdc: call 0x1428d9390
1412c1be1: int3 
1412c1be2: nop word ptr cs:[rax + rax]
1412c1bf0: mov qword ptr [rsp + 0x10], rdx
1412c1bf5: push rbp
1412c1bf6: push rsi
1412c1bf7: sub rsp, 0x28
1412c1bfb: lea rbp, [rdx + 0x80]
1412c1c02: lea rcx, [rbp - 0x58]
1412c1c06: call 0x140be24b0
1412c1c0b: nop 
1412c1c0c: add rsp, 0x28
1412c1c10: pop rsi
1412c1c11: pop rbp
1412c1c12: ret 
1412c1c13: nop word ptr cs:[rax + rax]
1412c1c20: mov qword ptr [rsp + 0x10], rdx
1412c1c25: push rbp
1412c1c26: push rsi
1412c1c27: sub rsp, 0x28
1412c1c2b: lea rbp, [rdx + 0x80]
1412c1c32: lea rcx, [rbp + 0x10]
1412c1c36: call 0x1400c04b0
1412c1c3b: nop 
1412c1c3c: add rsp, 0x28
1412c1c40: pop rsi
1412c1c41: pop rbp
1412c1c42: ret 
