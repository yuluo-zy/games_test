1412c5b00: push rbp
1412c5b01: push rsi
1412c5b02: sub rsp, 0xe8
1412c5b09: lea rbp, [rsp + 0x80]
1412c5b11: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c5b19: mov rsi, rcx
1412c5b1c: lea rcx, [rbp - 0x58]
1412c5b20: xor r8d, r8d
1412c5b23: call 0x14161c540
1412c5b28: movzx eax, byte ptr [rip + 0x236f0da]
1412c5b2f: mov ecx, 0x38
1412c5b34: mov edx, 8
1412c5b39: call 0x140613c10
1412c5b3e: test rax, rax
1412c5b41: je 0x1412c5bff
1412c5b47: mov rcx, qword ptr [rbp - 0x28]
1412c5b4b: mov qword ptr [rax + 0x30], rcx
1412c5b4f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c5b53: movups xmm1, xmmword ptr [rbp - 0x48]
1412c5b57: movups xmm2, xmmword ptr [rbp - 0x38]
1412c5b5b: movups xmmword ptr [rax + 0x20], xmm2
1412c5b5f: movups xmmword ptr [rax + 0x10], xmm1
1412c5b63: movups xmmword ptr [rax], xmm0
1412c5b66: lea rcx, [rip + 0x17d8a0f]
1412c5b6d: mov qword ptr [rbp + 0x10], rcx
1412c5b71: mov qword ptr [rbp + 0x18], 0x40
1412c5b79: mov qword ptr [rbp + 0x20], rax
1412c5b7d: lea rax, [rip + 0x1967754]
1412c5b84: mov qword ptr [rbp + 0x28], rax
1412c5b88: mov qword ptr [rbp + 0x40], 0
1412c5b90: mov byte ptr [rbp + 0x56], 5
1412c5b94: mov qword ptr [rbp + 0x30], rcx
1412c5b98: mov qword ptr [rbp + 0x38], 0x40
1412c5ba0: movzx eax, byte ptr [rip + 0x236f062]
1412c5ba7: mov ecx, 0x40
1412c5bac: mov edx, 8
1412c5bb1: call 0x140613c10
1412c5bb6: test rax, rax
1412c5bb9: je 0x1412c5c10
1412c5bbb: movups xmm0, xmmword ptr [rbp + 0x20]
1412c5bbf: movups xmm1, xmmword ptr [rbp + 0x30]
1412c5bc3: movups xmm2, xmmword ptr [rbp + 0x40]
1412c5bc7: movups xmm3, xmmword ptr [rbp + 0x50]
1412c5bcb: movups xmmword ptr [rax + 0x30], xmm3
1412c5bcf: movups xmmword ptr [rax + 0x20], xmm2
1412c5bd3: movups xmmword ptr [rax + 0x10], xmm1
1412c5bd7: movups xmmword ptr [rax], xmm0
1412c5bda: movabs rdx, 0x8000afca277fc42b
1412c5be4: mov rcx, rsi
1412c5be7: mov r8, rax
1412c5bea: call 0x140969bf0
1412c5bef: cmp rax, 1
1412c5bf3: je 0x1412c5c21
1412c5bf5: add rsp, 0xe8
1412c5bfc: pop rsi
1412c5bfd: pop rbp
1412c5bfe: ret 
1412c5bff: mov ecx, 8
1412c5c04: mov edx, 0x38
1412c5c09: call 0x1428d9000
1412c5c0e: jmp 0x1412c5c1f
1412c5c10: mov ecx, 8
1412c5c15: mov edx, 0x40
1412c5c1a: call 0x1428d9000
1412c5c1f: ud2 
1412c5c21: mov rax, rdx
1412c5c24: add rdx, 0x10
1412c5c28: mov rsi, rdx
1412c5c2b: cmp qword ptr [rax + 0x18], 0x40
1412c5c30: jne 0x1412c5caf
1412c5c32: mov rdx, qword ptr [rsi]
1412c5c35: lea rcx, [rip + 0x17d8940]
1412c5c3c: mov r8d, 0x40
1412c5c42: call 0x142923bf0
1412c5c47: test eax, eax
1412c5c49: jne 0x1412c5caf
1412c5c4b: lea rax, [rip + 0x1967ef6]
1412c5c52: mov qword ptr [rbp - 0x20], rax
1412c5c56: lea rax, [rip - 0xd923bd]
1412c5c5d: mov qword ptr [rbp - 0x18], rax
1412c5c61: lea rax, [rbp + 0x10]
1412c5c65: mov qword ptr [rbp - 0x10], rax
1412c5c69: lea rax, [rip - 0x11bbde0]
1412c5c70: mov qword ptr [rbp - 8], rax
1412c5c74: lea rax, [rip + 0x19679ad]
1412c5c7b: mov qword ptr [rbp + 0x20], rax
1412c5c7f: mov qword ptr [rbp + 0x28], 3
1412c5c87: mov qword ptr [rbp + 0x40], 0
1412c5c8f: lea rax, [rbp - 0x20]
1412c5c93: mov qword ptr [rbp + 0x30], rax
1412c5c97: mov qword ptr [rbp + 0x38], 2
1412c5c9f: lea rdx, [rip + 0x19679b2]
1412c5ca6: lea rcx, [rbp + 0x20]
1412c5caa: call 0x1428d9390
1412c5caf: lea rax, [rip + 0x1967e92]
1412c5cb6: mov qword ptr [rbp + 0x20], rax
1412c5cba: lea rax, [rip - 0xd92421]
1412c5cc1: mov qword ptr [rbp + 0x28], rax
1412c5cc5: mov qword ptr [rbp + 0x30], rsi
1412c5cc9: lea rax, [rip - 0x11bbe40]
1412c5cd0: mov qword ptr [rbp + 0x38], rax
1412c5cd4: lea rcx, [rbp + 0x10]
1412c5cd8: mov qword ptr [rbp + 0x40], rcx
1412c5cdc: mov qword ptr [rbp + 0x48], rax
1412c5ce0: lea rax, [rip + 0x1967881]
1412c5ce7: mov qword ptr [rbp - 0x20], rax
1412c5ceb: mov qword ptr [rbp - 0x18], 3
1412c5cf3: mov qword ptr [rbp], 0
1412c5cfb: lea rax, [rbp + 0x20]
1412c5cff: mov qword ptr [rbp - 0x10], rax
1412c5d03: mov qword ptr [rbp - 8], 3
1412c5d0b: lea rdx, [rip + 0x19678be]
1412c5d12: lea rcx, [rbp - 0x20]
1412c5d16: call 0x1428d9390
1412c5d1b: int3 
1412c5d1c: nop dword ptr [rax]
1412c5d20: mov qword ptr [rsp + 0x10], rdx
1412c5d25: push rbp
1412c5d26: push rsi
1412c5d27: sub rsp, 0x28
1412c5d2b: lea rbp, [rdx + 0x80]
1412c5d32: lea rcx, [rbp - 0x48]
1412c5d36: call 0x140be24b0
1412c5d3b: nop 
1412c5d3c: add rsp, 0x28
1412c5d40: pop rsi
1412c5d41: pop rbp
1412c5d42: ret 
1412c5d43: nop word ptr cs:[rax + rax]
1412c5d50: mov qword ptr [rsp + 0x10], rdx
1412c5d55: push rbp
1412c5d56: push rsi
1412c5d57: sub rsp, 0x28
1412c5d5b: lea rbp, [rdx + 0x80]
1412c5d62: lea rcx, [rbp + 0x20]
1412c5d66: call 0x1400c04b0
1412c5d6b: nop 
1412c5d6c: add rsp, 0x28
1412c5d70: pop rsi
1412c5d71: pop rbp
1412c5d72: ret 
