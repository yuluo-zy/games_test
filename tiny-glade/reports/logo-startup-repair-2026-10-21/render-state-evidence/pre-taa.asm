0x1409c3a80: push rbp
0x1409c3a81: push r15
0x1409c3a83: push r14
0x1409c3a85: push r13
0x1409c3a87: push r12
0x1409c3a89: push rsi
0x1409c3a8a: push rdi
0x1409c3a8b: push rbx
0x1409c3a8c: sub rsp, 0xa48
0x1409c3a93: lea rbp, [rsp + 0x80]
0x1409c3a9b: mov qword ptr [rbp + 0x9c0], 0xfffffffffffffffe
0x1409c3aa6: mov rbx, r9
0x1409c3aa9: mov rsi, r8
0x1409c3aac: mov r15, rdx
0x1409c3aaf: mov rdi, rcx
0x1409c3ab2: lea rcx, [r9 + 4]
0x1409c3ab6: call 0x140bd5790
0x1409c3abb: mov dword ptr [rbp + 0x3b8], 0
0x1409c3ac5: mov qword ptr [rbp + 0x3c0], rsi
0x1409c3acc: mov qword ptr [rbp + 0x898], 1
0x1409c3ad7: mov qword ptr [rbp + 0x998], rbx
0x1409c3ade: mov qword ptr [rbp + 0x8a0], rbx
0x1409c3ae5: mov qword ptr [rbp + 0x8a8], 0
0x1409c3af0: mov qword ptr [rsp + 0x40], rdi
0x1409c3af5: mov qword ptr [rsp + 0x38], r15
0x1409c3afa: mov qword ptr [rsp + 0x30], rax
0x1409c3aff: lea rax, [rbp + 0x898]
0x1409c3b06: mov qword ptr [rsp + 0x28], rax
0x1409c3b0b: mov qword ptr [rsp + 0x20], 1
0x1409c3b14: lea rdx, [rip + 0x20fc3ab]
0x1409c3b1b: lea rcx, [rbp - 0x30]
0x1409c3b1f: lea r9, [rbp + 0x3b8]
0x1409c3b26: mov r8d, 0x14
0x1409c3b2c: call 0x140867970
0x1409c3b31: mov qword ptr [rbp + 0x9b0], r15
0x1409c3b38: lea rcx, [r15 + 0x38]
0x1409c3b3c: mov byte ptr [rbp + 0x9bf], 1
0x1409c3b43: movabs rdx, 0x4bc362195b80f729
0x1409c3b4d: movabs r8, 0x982798ce80cbcefb
0x1409c3b57: call 0x1400dd6f0
0x1409c3b5c: test rax, rax
0x1409c3b5f: mov rcx, qword ptr [rbp + 0x9b0]
0x1409c3b66: je 0x1409c3f5b
0x1409c3b6c: cmp qword ptr [rcx + 0x218], rdx
0x1409c3b73: jbe 0x1409c3f5b
0x1409c3b79: mov rax, qword ptr [rcx + 0x210]
0x1409c3b80: mov rax, qword ptr [rax + rdx*8]
0x1409c3b84: test rax, rax
0x1409c3b87: je 0x1409c3f5b
0x1409c3b8d: mov rcx, qword ptr [rcx + 0x1e0]
0x1409c3b94: not rax
0x1409c3b97: lea rdx, [rax + rax*2]
0x1409c3b9b: shl rdx, 5
0x1409c3b9f: cmp qword ptr [rcx + rdx + 0x20], 0
0x1409c3ba5: je 0x1409c3f5b
0x1409c3bab: mov rax, qword ptr [rbp + 0xa48]
0x1409c3bb2: add rcx, rdx
0x1409c3bb5: mov rcx, qword ptr [rcx + 0x10]
0x1409c3bb9: mov rdx, qword ptr [rcx]
0x1409c3bbc: test rdx, rdx
0x1409c3bbf: js 0x1409c3bc8
0x1409c3bc1: cvtsi2ss xmm0, rdx
0x1409c3bc6: jmp 0x1409c3bdd
0x1409c3bc8: mov r8, rdx
0x1409c3bcb: shr r8, 1
0x1409c3bce: and edx, 1
0x1409c3bd1: or rdx, r8
0x1409c3bd4: cvtsi2ss xmm0, rdx
0x1409c3bd9: addss xmm0, xmm0
0x1409c3bdd: cvtsi2ss xmm1, dword ptr [rcx + 8]
0x1409c3be2: divss xmm1, dword ptr [rip + 0x1fe3a56]
0x1409c3bea: addss xmm1, xmm0
0x1409c3bee: movss dword ptr [rbp + 0x9a0], xmm1
0x1409c3bf6: mov byte ptr [rbp + 0x9a4], 0
0x1409c3bfd: mov r15, qword ptr [rax + 0x188]
0x1409c3c04: mov rax, qword ptr [rax + 0x190]
0x1409c3c0b: shl rax, 7
0x1409c3c0f: add rax, r15
0x1409c3c12: mov qword ptr [rbp + 0x990], rax
0x1409c3c19: lea r12, [rbp + 0x7a8]
0x1409c3c20: lea r13, [rbp + 0x3c0]
0x1409c3c27: lea r14, [rbp + 0x9a8]
0x1409c3c2e: cmp r15, qword ptr [rbp + 0x990]
0x1409c3c35: je 0x1409c3f09
0x1409c3c3b: nop dword ptr [rax + rax]
0x1409c3c40: mov rbx, qword ptr [r15 + 0x50]
0x1409c3c44: mov rdi, qword ptr [r15 + 0x58]
0x1409c3c48: mov byte ptr [rbp + 0x9bf], 1
0x1409c3c4f: call 0x140be6750
0x1409c3c54: mov esi, eax
0x1409c3c56: mov byte ptr [rbp + 0x9bf], 1
0x1409c3c5d: xor ecx, ecx
0x1409c3c5f: call 0x1400d9b20
0x1409c3c64: test rax, rax
0x1409c3c67: je 0x1409c3f46
0x1409c3c6d: mov dword ptr [rbp + 0x940], esi
0x1409c3c73: mov word ptr [rbp + 0x948], 0
0x1409c3c7c: mov dword ptr [rbp + 0x944], 0x3f800000
0x1409c3c86: mov qword ptr [rbp + 0x898], 1
0x1409c3c91: mov qword ptr [rbp + 0x8a0], rbx
0x1409c3c98: mov qword ptr [rbp + 0x938], rdi
0x1409c3c9f: mov qword ptr [rbp + 0x8a8], 0
0x1409c3caa: mov qword ptr [rbp + 0x8b0], 4
0x1409c3cb5: mov qword ptr [rbp + 0x8b8], 0
0x1409c3cc0: movups xmm0, xmmword ptr [rax]
0x1409c3cc3: inc qword ptr [rax]
0x1409c3cc6: movups xmm1, xmmword ptr [rip + 0x20d2a63]
0x1409c3ccd: lea rax, [rbp + 0x908]
0x1409c3cd4: movups xmmword ptr [rax + 0x10], xmm1
0x1409c3cd8: movups xmm1, xmmword ptr [rip + 0x20d2a41]
0x1409c3cdf: movups xmmword ptr [rax], xmm1
0x1409c3ce2: movups xmmword ptr [rbp + 0x928], xmm0
0x1409c3ce9: movabs rax, 0x100000003
0x1409c3cf3: mov qword ptr [rbp + 0x8d8], rax
0x1409c3cfa: mov qword ptr [rbp + 0x8c0], 0
0x1409c3d05: mov qword ptr [rbp + 0x8c8], 1
0x1409c3d10: mov qword ptr [rbp + 0x8d0], 0
0x1409c3d1b: mov byte ptr [rbp + 0x9bf], 1
0x1409c3d22: lea rcx, [rbp + 0x3b8]
0x1409c3d29: lea rdx, [rbp + 0x898]
0x1409c3d30: lea r8, [rbp - 0x30]
0x1409c3d34: xor r9d, r9d
0x1409c3d37: call 0x140905be0
0x1409c3d3c: mov byte ptr [rbp + 0x9bf], 1
0x1409c3d43: lea rcx, [rbp + 0x898]
0x1409c3d4a: lea rdx, [rbp + 0x3b8]
0x1409c3d51: lea r8, [rbp + 0x9a0]
0x1409c3d58: mov r9, r15
0x1409c3d5b: call 0x14090b460
0x1409c3d60: mov rax, qword ptr [rbp + 0x998]
0x1409c3d67: mov qword ptr [rbp + 0x9a8], rax
0x1409c3d6e: mov dword ptr [rbp + 0x3b8], 2
0x1409c3d78: lea rbx, [rbp + 0x3b8]
0x1409c3d7f: mov qword ptr [rsp + 0x20], rbx
0x1409c3d84: mov rdi, r14
0x1409c3d87: mov rcx, r14
0x1409c3d8a: lea rsi, [rbp + 0x898]
0x1409c3d91: mov rdx, rsi
0x1409c3d94: movabs r8, 0x4861579da049fbfc
0x1409c3d9e: mov r9b, 0x1d
0x1409c3da1: call 0x1408ee800
0x1409c3da6: mov r8d, 0xf8
0x1409c3dac: mov rcx, rbx
0x1409c3daf: mov rdx, rsi
0x1409c3db2: call 0x142923c00
0x1409c3db7: mov rax, qword ptr [rbp + 0xa30]
0x1409c3dbe: mov qword ptr [rbp + 0x9a8], rax
0x1409c3dc5: mov dword ptr [rbp + 0x898], 2
0x1409c3dcf: mov qword ptr [rsp + 0x20], rsi
0x1409c3dd4: mov rcx, rdi
0x1409c3dd7: mov rdx, rbx
0x1409c3dda: movabs r8, 0x2a333c3a28028230
0x1409c3de4: mov r9b, 0x1d
0x1409c3de7: call 0x1408ee800
0x1409c3dec: mov r8d, 0xf8
0x1409c3df2: lea rdi, [rbp + 0x7a0]
0x1409c3df9: mov rcx, rdi
0x1409c3dfc: mov rdx, rbx
0x1409c3dff: call 0x142923c00
0x1409c3e04: mov rax, qword ptr [rbp + 0xa38]
0x1409c3e0b: mov qword ptr [rbp + 0x898], rax
0x1409c3e12: mov dword ptr [rbp + 0x3b8], 2
0x1409c3e1c: mov qword ptr [rsp + 0x20], rbx
0x1409c3e21: mov rcx, rsi
0x1409c3e24: mov rdx, rdi
0x1409c3e27: movabs r8, 0x4175185826c09d0
0x1409c3e31: mov r9b, 0x1d
0x1409c3e34: call 0x1408ee800
0x1409c3e39: mov rsi, qword ptr [rbp + 0x7a0]
0x1409c3e40: mov r8d, 0xa0
0x1409c3e46: mov rcx, r13
0x1409c3e49: mov rdx, r12
0x1409c3e4c: call 0x142923c00
0x1409c3e51: movzx eax, byte ptr [rbp + 0x848]
0x1409c3e58: movzx ecx, word ptr [rbp + 0x849]
0x1409c3e5f: movzx edx, byte ptr [rbp + 0x84b]
0x1409c3e66: movups xmm0, xmmword ptr [r12 + 0xa4]
0x1409c3e6f: movups xmm1, xmmword ptr [r12 + 0xb4]
0x1409c3e78: movups xmm2, xmmword ptr [r12 + 0xc4]
0x1409c3e81: movups xmm3, xmmword ptr [r12 + 0xd4]
0x1409c3e8a: movups xmmword ptr [r13 + 0xa4], xmm0
0x1409c3e92: movups xmmword ptr [r13 + 0xb4], xmm1
0x1409c3e9a: movups xmmword ptr [r13 + 0xc4], xmm2
0x1409c3ea2: movups xmmword ptr [r13 + 0xd4], xmm3
0x1409c3eaa: movups xmm0, xmmword ptr [r12 + 0xe0]
0x1409c3eb3: movups xmmword ptr [r13 + 0xe0], xmm0
0x1409c3ebb: and al, 0xbf
0x1409c3ebd: and dl, 0x9f
0x1409c3ec0: or dl, 0x40
0x1409c3ec3: mov r9, qword ptr [r15 + 0x70]
0x1409c3ec7: mov qword ptr [rbp + 0x3b8], rsi
0x1409c3ece: mov byte ptr [rbp + 0x460], al
0x1409c3ed4: mov word ptr [rbp + 0x461], cx
0x1409c3edb: mov byte ptr [rbp + 0x463], dl
0x1409c3ee1: mov byte ptr [rbp + 0x9bf], 1
0x1409c3ee8: mov rcx, rbx
0x1409c3eeb: mov rdx, r15
0x1409c3eee: mov r8, qword ptr [rbp + 0x9b0]
0x1409c3ef5: call r9
0x1409c3ef8: sub r15, -0x80
0x1409c3efc: cmp r15, qword ptr [rbp + 0x990]
0x1409c3f03: jne 0x1409c3c40
0x1409c3f09: lea rsi, [rbp + 0x3b8]
0x1409c3f10: lea rdx, [rbp - 0x30]
0x1409c3f14: mov r8d, 0x3e8
0x1409c3f1a: mov rcx, rsi
0x1409c3f1d: call 0x142923c00
0x1409c3f22: mov byte ptr [rbp + 0x9bf], 0
0x1409c3f29: mov rcx, rsi
0x1409c3f2c: call 0x140868370
0x1409c3f31: nop 
0x1409c3f32: add rsp, 0xa48
0x1409c3f39: pop rbx
0x1409c3f3a: pop rdi
0x1409c3f3b: pop rsi
0x1409c3f3c: pop r12
0x1409c3f3e: pop r13
0x1409c3f40: pop r14
0x1409c3f42: pop r15
0x1409c3f44: pop rbp
0x1409c3f45: ret 
0x1409c3f46: mov byte ptr [rbp + 0x9bf], 1
0x1409c3f4d: lea rcx, [rip + 0x2276b4c]
0x1409c3f54: call 0x1428d80d0
0x1409c3f59: jmp 0x1409c3f6e
0x1409c3f5b: mov byte ptr [rbp + 0x9bf], 1
0x1409c3f62: lea rcx, [rip + 0x20e1c6f]
0x1409c3f69: call 0x1428d9310
0x1409c3f6e: ud2 
0x1409c3f70: mov qword ptr [rsp + 0x10], rdx
0x1409c3f75: push rbp
0x1409c3f76: push r15
0x1409c3f78: push r14
0x1409c3f7a: push r13
0x1409c3f7c: push r12
0x1409c3f7e: push rsi
0x1409c3f7f: push rdi
0x1409c3f80: push rbx
0x1409c3f81: sub rsp, 0x48
0x1409c3f85: lea rbp, [rdx + 0x80]
0x1409c3f8c: cmp byte ptr [rbp + 0x9bf], 0
0x1409c3f93: jne 0x1409c3fa6
0x1409c3f95: add rsp, 0x48
0x1409c3f99: pop rbx
0x1409c3f9a: pop rdi
0x1409c3f9b: pop rsi
0x1409c3f9c: pop r12
0x1409c3f9e: pop r13
0x1409c3fa0: pop r14
0x1409c3fa2: pop r15
0x1409c3fa4: pop rbp
0x1409c3fa5: ret 
0x1409c3fa6: lea rcx, [rbp - 0x30]
0x1409c3faa: call 0x1408923c0
0x1409c3faf: jmp 0x1409c3f95
0x1409c3fb1: nop word ptr cs:[rax + rax]
0x1409c3fc0: mov qword ptr [rsp + 0x10], rdx
0x1409c3fc5: push rbp
0x1409c3fc6: push r15
0x1409c3fc8: push r14
0x1409c3fca: push r13
0x1409c3fcc: push r12
0x1409c3fce: push rsi
0x1409c3fcf: push rdi
0x1409c3fd0: push rbx
0x1409c3fd1: sub rsp, 0x48
0x1409c3fd5: lea rbp, [rdx + 0x80]
0x1409c3fdc: mov byte ptr [rbp + 0x9bf], 1
0x1409c3fe3: lea rcx, [rbp + 0x898]
0x1409c3fea: call 0x1408935c0
0x1409c3fef: mov byte ptr [rbp + 0x9bf], 1
0x1409c3ff6: add rsp, 0x48
0x1409c3ffa: pop rbx
0x1409c3ffb: pop rdi
0x1409c3ffc: pop rsi
0x1409c3ffd: pop r12
0x1409c3fff: pop r13
0x1409c4001: pop r14
0x1409c4003: pop r15
0x1409c4005: pop rbp
0x1409c4006: ret 
0x1409c4007: nop word ptr [rax + rax]
0x1409c4010: mov qword ptr [rsp + 0x10], rdx
0x1409c4015: push rbp
0x1409c4016: push r15
0x1409c4018: push r14
0x1409c401a: push r13
0x1409c401c: push r12
0x1409c401e: push rsi
0x1409c401f: push rdi
0x1409c4020: push rbx
0x1409c4021: sub rsp, 0x48
0x1409c4025: lea rbp, [rdx + 0x80]
0x1409c402c: mov byte ptr [rbp + 0x9bf], 1
0x1409c4033: lea rcx, [rbp + 0x3b8]
0x1409c403a: call 0x1408935c0
0x1409c403f: mov byte ptr [rbp + 0x9bf], 1
0x1409c4046: add rsp, 0x48
0x1409c404a: pop rbx
0x1409c404b: pop rdi
0x1409c404c: pop rsi
0x1409c404d: pop r12
0x1409c404f: pop r13
0x1409c4051: pop r14
0x1409c4053: pop r15
0x1409c4055: pop rbp
0x1409c4056: ret 
0x1409c4057: nop word ptr [rax + rax]
0x1409c4060: mov qword ptr [rsp + 0x10], rdx
0x1409c4065: push rbp
0x1409c4066: push r15
0x1409c4068: push r14
0x1409c406a: push r13
0x1409c406c: push r12
0x1409c406e: push rsi
0x1409c406f: push rdi
0x1409c4070: push rbx
0x1409c4071: sub rsp, 0x48
0x1409c4075: lea rbp, [rdx + 0x80]
0x1409c407c: mov byte ptr [rbp + 0x9bf], 1
0x1409c4083: lea rcx, [rbp + 0x7a0]
0x1409c408a: call 0x1408935c0
0x1409c408f: mov byte ptr [rbp + 0x9bf], 1
0x1409c4096: add rsp, 0x48
0x1409c409a: pop rbx
0x1409c409b: pop rdi
0x1409c409c: pop rsi
0x1409c409d: pop r12
0x1409c409f: pop r13
0x1409c40a1: pop r14
0x1409c40a3: pop r15
0x1409c40a5: pop rbp
0x1409c40a6: ret 
