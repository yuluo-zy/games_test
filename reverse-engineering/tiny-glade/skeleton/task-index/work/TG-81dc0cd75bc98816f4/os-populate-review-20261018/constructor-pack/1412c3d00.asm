1412c3d00: push rbp
1412c3d01: push rsi
1412c3d02: sub rsp, 0xd8
1412c3d09: lea rbp, [rsp + 0x80]
1412c3d11: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c3d19: mov rsi, rcx
1412c3d1c: lea rcx, [rbp - 0x58]
1412c3d20: xor r8d, r8d
1412c3d23: call 0x141611df0
1412c3d28: movzx eax, byte ptr [rip + 0x2370eda]
1412c3d2f: mov ecx, 0x28
1412c3d34: mov edx, 8
1412c3d39: call 0x140613c10
1412c3d3e: test rax, rax
1412c3d41: je 0x1412c3df7
1412c3d47: mov rcx, qword ptr [rbp - 0x38]
1412c3d4b: mov qword ptr [rax + 0x20], rcx
1412c3d4f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c3d53: movups xmm1, xmmword ptr [rbp - 0x48]
1412c3d57: movups xmmword ptr [rax + 0x10], xmm1
1412c3d5b: movups xmmword ptr [rax], xmm0
1412c3d5e: lea rcx, [rip + 0x17da9e7]
1412c3d65: mov qword ptr [rbp], rcx
1412c3d69: mov qword ptr [rbp + 8], 0x46
1412c3d71: mov qword ptr [rbp + 0x10], rax
1412c3d75: lea rax, [rip + 0x19692bc]
1412c3d7c: mov qword ptr [rbp + 0x18], rax
1412c3d80: mov qword ptr [rbp + 0x30], 0
1412c3d88: mov byte ptr [rbp + 0x46], 5
1412c3d8c: mov qword ptr [rbp + 0x20], rcx
1412c3d90: mov qword ptr [rbp + 0x28], 0x46
1412c3d98: movzx eax, byte ptr [rip + 0x2370e6a]
1412c3d9f: mov ecx, 0x40
1412c3da4: mov edx, 8
1412c3da9: call 0x140613c10
1412c3dae: test rax, rax
1412c3db1: je 0x1412c3e08
1412c3db3: movups xmm0, xmmword ptr [rbp + 0x10]
1412c3db7: movups xmm1, xmmword ptr [rbp + 0x20]
1412c3dbb: movups xmm2, xmmword ptr [rbp + 0x30]
1412c3dbf: movups xmm3, xmmword ptr [rbp + 0x40]
1412c3dc3: movups xmmword ptr [rax + 0x30], xmm3
1412c3dc7: movups xmmword ptr [rax + 0x20], xmm2
1412c3dcb: movups xmmword ptr [rax + 0x10], xmm1
1412c3dcf: movups xmmword ptr [rax], xmm0
1412c3dd2: movabs rdx, 0xbcda5e693b827511
1412c3ddc: mov rcx, rsi
1412c3ddf: mov r8, rax
1412c3de2: call 0x140969bf0
1412c3de7: cmp rax, 1
1412c3deb: je 0x1412c3e19
1412c3ded: add rsp, 0xd8
1412c3df4: pop rsi
1412c3df5: pop rbp
1412c3df6: ret 
1412c3df7: mov ecx, 8
1412c3dfc: mov edx, 0x28
1412c3e01: call 0x1428d9000
1412c3e06: jmp 0x1412c3e17
1412c3e08: mov ecx, 8
1412c3e0d: mov edx, 0x40
1412c3e12: call 0x1428d9000
1412c3e17: ud2 
1412c3e19: mov rax, rdx
1412c3e1c: add rdx, 0x10
1412c3e20: mov rsi, rdx
1412c3e23: cmp qword ptr [rax + 0x18], 0x46
1412c3e28: jne 0x1412c3ea6
1412c3e2a: mov rdx, qword ptr [rsi]
1412c3e2d: lea rcx, [rip + 0x17da918]
1412c3e34: mov r8d, 0x46
1412c3e3a: call 0x142923bf0
1412c3e3f: test eax, eax
1412c3e41: jne 0x1412c3ea6
1412c3e43: lea rax, [rip + 0x1969a7e]
1412c3e4a: mov qword ptr [rbp - 0x30], rax
1412c3e4e: lea rax, [rip - 0xd905b5]
1412c3e55: mov qword ptr [rbp - 0x28], rax
1412c3e59: mov rax, rbp
1412c3e5c: mov qword ptr [rbp - 0x20], rax
1412c3e60: lea rax, [rip - 0x11b9fd7]
1412c3e67: mov qword ptr [rbp - 0x18], rax
1412c3e6b: lea rax, [rip + 0x19697b6]
1412c3e72: mov qword ptr [rbp + 0x10], rax
1412c3e76: mov qword ptr [rbp + 0x18], 3
1412c3e7e: mov qword ptr [rbp + 0x30], 0
1412c3e86: lea rax, [rbp - 0x30]
1412c3e8a: mov qword ptr [rbp + 0x20], rax
1412c3e8e: mov qword ptr [rbp + 0x28], 2
1412c3e96: lea rdx, [rip + 0x19697bb]
1412c3e9d: lea rcx, [rbp + 0x10]
1412c3ea1: call 0x1428d9390
1412c3ea6: lea rax, [rip + 0x1969a1b]
1412c3ead: mov qword ptr [rbp + 0x10], rax
1412c3eb1: lea rax, [rip - 0xd90618]
1412c3eb8: mov qword ptr [rbp + 0x18], rax
1412c3ebc: mov qword ptr [rbp + 0x20], rsi
1412c3ec0: lea rax, [rip - 0x11ba037]
1412c3ec7: mov qword ptr [rbp + 0x28], rax
1412c3ecb: mov rcx, rbp
1412c3ece: mov qword ptr [rbp + 0x30], rcx
1412c3ed2: mov qword ptr [rbp + 0x38], rax
1412c3ed6: lea rax, [rip + 0x196968b]
1412c3edd: mov qword ptr [rbp - 0x30], rax
1412c3ee1: mov qword ptr [rbp - 0x28], 3
1412c3ee9: mov qword ptr [rbp - 0x10], 0
1412c3ef1: lea rax, [rbp + 0x10]
1412c3ef5: mov qword ptr [rbp - 0x20], rax
1412c3ef9: mov qword ptr [rbp - 0x18], 3
1412c3f01: lea rdx, [rip + 0x19696c8]
1412c3f08: lea rcx, [rbp - 0x30]
1412c3f0c: call 0x1428d9390
1412c3f11: int3 
1412c3f12: nop word ptr cs:[rax + rax]
1412c3f20: mov qword ptr [rsp + 0x10], rdx
1412c3f25: push rbp
1412c3f26: push rsi
1412c3f27: sub rsp, 0x28
1412c3f2b: lea rbp, [rdx + 0x80]
1412c3f32: lea rcx, [rbp - 0x58]
1412c3f36: call 0x140be24b0
1412c3f3b: nop 
1412c3f3c: add rsp, 0x28
1412c3f40: pop rsi
1412c3f41: pop rbp
1412c3f42: ret 
1412c3f43: nop word ptr cs:[rax + rax]
1412c3f50: mov qword ptr [rsp + 0x10], rdx
1412c3f55: push rbp
1412c3f56: push rsi
1412c3f57: sub rsp, 0x28
1412c3f5b: lea rbp, [rdx + 0x80]
1412c3f62: lea rcx, [rbp + 0x10]
1412c3f66: call 0x1400c04b0
1412c3f6b: nop 
1412c3f6c: add rsp, 0x28
1412c3f70: pop rsi
1412c3f71: pop rbp
1412c3f72: ret 
