1412c2dd0: push rbp
1412c2dd1: push rsi
1412c2dd2: sub rsp, 0xe8
1412c2dd9: lea rbp, [rsp + 0x80]
1412c2de1: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c2de9: mov rsi, rcx
1412c2dec: lea rcx, [rbp - 0x58]
1412c2df0: xor r8d, r8d
1412c2df3: call 0x14161d570
1412c2df8: movzx eax, byte ptr [rip + 0x2371e0a]
1412c2dff: mov ecx, 0x38
1412c2e04: mov edx, 8
1412c2e09: call 0x140613c10
1412c2e0e: test rax, rax
1412c2e11: je 0x1412c2ecf
1412c2e17: mov rcx, qword ptr [rbp - 0x28]
1412c2e1b: mov qword ptr [rax + 0x30], rcx
1412c2e1f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c2e23: movups xmm1, xmmword ptr [rbp - 0x48]
1412c2e27: movups xmm2, xmmword ptr [rbp - 0x38]
1412c2e2b: movups xmmword ptr [rax + 0x20], xmm2
1412c2e2f: movups xmmword ptr [rax + 0x10], xmm1
1412c2e33: movups xmmword ptr [rax], xmm0
1412c2e36: lea rcx, [rip + 0x17db36c]
1412c2e3d: mov qword ptr [rbp + 0x10], rcx
1412c2e41: mov qword ptr [rbp + 0x18], 0x32
1412c2e49: mov qword ptr [rbp + 0x20], rax
1412c2e4d: lea rax, [rip + 0x196a094]
1412c2e54: mov qword ptr [rbp + 0x28], rax
1412c2e58: mov qword ptr [rbp + 0x40], 0
1412c2e60: mov byte ptr [rbp + 0x56], 5
1412c2e64: mov qword ptr [rbp + 0x30], rcx
1412c2e68: mov qword ptr [rbp + 0x38], 0x32
1412c2e70: movzx eax, byte ptr [rip + 0x2371d92]
1412c2e77: mov ecx, 0x40
1412c2e7c: mov edx, 8
1412c2e81: call 0x140613c10
1412c2e86: test rax, rax
1412c2e89: je 0x1412c2ee0
1412c2e8b: movups xmm0, xmmword ptr [rbp + 0x20]
1412c2e8f: movups xmm1, xmmword ptr [rbp + 0x30]
1412c2e93: movups xmm2, xmmword ptr [rbp + 0x40]
1412c2e97: movups xmm3, xmmword ptr [rbp + 0x50]
1412c2e9b: movups xmmword ptr [rax + 0x30], xmm3
1412c2e9f: movups xmmword ptr [rax + 0x20], xmm2
1412c2ea3: movups xmmword ptr [rax + 0x10], xmm1
1412c2ea7: movups xmmword ptr [rax], xmm0
1412c2eaa: movabs rdx, 0x4652455297dab420
1412c2eb4: mov rcx, rsi
1412c2eb7: mov r8, rax
1412c2eba: call 0x140969bf0
1412c2ebf: cmp rax, 1
1412c2ec3: je 0x1412c2ef1
1412c2ec5: add rsp, 0xe8
1412c2ecc: pop rsi
1412c2ecd: pop rbp
1412c2ece: ret 
1412c2ecf: mov ecx, 8
1412c2ed4: mov edx, 0x38
1412c2ed9: call 0x1428d9000
1412c2ede: jmp 0x1412c2eef
1412c2ee0: mov ecx, 8
1412c2ee5: mov edx, 0x40
1412c2eea: call 0x1428d9000
1412c2eef: ud2 
1412c2ef1: mov rax, rdx
1412c2ef4: add rdx, 0x10
1412c2ef8: mov rsi, rdx
1412c2efb: cmp qword ptr [rax + 0x18], 0x32
1412c2f00: jne 0x1412c2f7f
1412c2f02: mov rdx, qword ptr [rsi]
1412c2f05: lea rcx, [rip + 0x17db29d]
1412c2f0c: mov r8d, 0x32
1412c2f12: call 0x142923bf0
1412c2f17: test eax, eax
1412c2f19: jne 0x1412c2f7f
1412c2f1b: lea rax, [rip + 0x196b016]
1412c2f22: mov qword ptr [rbp - 0x20], rax
1412c2f26: lea rax, [rip - 0xd8f68d]
1412c2f2d: mov qword ptr [rbp - 0x18], rax
1412c2f31: lea rax, [rbp + 0x10]
1412c2f35: mov qword ptr [rbp - 0x10], rax
1412c2f39: lea rax, [rip - 0x11b90b0]
1412c2f40: mov qword ptr [rbp - 8], rax
1412c2f44: lea rax, [rip + 0x196a6dd]
1412c2f4b: mov qword ptr [rbp + 0x20], rax
1412c2f4f: mov qword ptr [rbp + 0x28], 3
1412c2f57: mov qword ptr [rbp + 0x40], 0
1412c2f5f: lea rax, [rbp - 0x20]
1412c2f63: mov qword ptr [rbp + 0x30], rax
1412c2f67: mov qword ptr [rbp + 0x38], 2
1412c2f6f: lea rdx, [rip + 0x196a6e2]
1412c2f76: lea rcx, [rbp + 0x20]
1412c2f7a: call 0x1428d9390
1412c2f7f: lea rax, [rip + 0x196afb2]
1412c2f86: mov qword ptr [rbp + 0x20], rax
1412c2f8a: lea rax, [rip - 0xd8f6f1]
1412c2f91: mov qword ptr [rbp + 0x28], rax
1412c2f95: mov qword ptr [rbp + 0x30], rsi
1412c2f99: lea rax, [rip - 0x11b9110]
1412c2fa0: mov qword ptr [rbp + 0x38], rax
1412c2fa4: lea rcx, [rbp + 0x10]
1412c2fa8: mov qword ptr [rbp + 0x40], rcx
1412c2fac: mov qword ptr [rbp + 0x48], rax
1412c2fb0: lea rax, [rip + 0x196a5b1]
1412c2fb7: mov qword ptr [rbp - 0x20], rax
1412c2fbb: mov qword ptr [rbp - 0x18], 3
1412c2fc3: mov qword ptr [rbp], 0
1412c2fcb: lea rax, [rbp + 0x20]
1412c2fcf: mov qword ptr [rbp - 0x10], rax
1412c2fd3: mov qword ptr [rbp - 8], 3
1412c2fdb: lea rdx, [rip + 0x196a5ee]
1412c2fe2: lea rcx, [rbp - 0x20]
1412c2fe6: call 0x1428d9390
1412c2feb: int3 
1412c2fec: nop dword ptr [rax]
1412c2ff0: mov qword ptr [rsp + 0x10], rdx
1412c2ff5: push rbp
1412c2ff6: push rsi
1412c2ff7: sub rsp, 0x28
1412c2ffb: lea rbp, [rdx + 0x80]
1412c3002: lea rcx, [rbp - 0x48]
1412c3006: call 0x140be24b0
1412c300b: nop 
1412c300c: add rsp, 0x28
1412c3010: pop rsi
1412c3011: pop rbp
1412c3012: ret 
1412c3013: nop word ptr cs:[rax + rax]
1412c3020: mov qword ptr [rsp + 0x10], rdx
1412c3025: push rbp
1412c3026: push rsi
1412c3027: sub rsp, 0x28
1412c302b: lea rbp, [rdx + 0x80]
1412c3032: lea rcx, [rbp + 0x20]
1412c3036: call 0x1400c04b0
1412c303b: nop 
1412c303c: add rsp, 0x28
1412c3040: pop rsi
1412c3041: pop rbp
1412c3042: ret 
