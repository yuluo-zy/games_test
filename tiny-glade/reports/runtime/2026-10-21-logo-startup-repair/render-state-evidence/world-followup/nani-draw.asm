0x140904ae0: push rbp
0x140904ae1: push r15
0x140904ae3: push r14
0x140904ae5: push r13
0x140904ae7: push r12
0x140904ae9: push rsi
0x140904aea: push rdi
0x140904aeb: push rbx
0x140904aec: sub rsp, 0x388
0x140904af3: lea rbp, [rsp + 0x80]
0x140904afb: mov qword ptr [rbp + 0x300], 0xfffffffffffffffe
0x140904b06: mov esi, edx
0x140904b08: mov qword ptr [rbp + 0x2f0], rcx
0x140904b0f: mov rax, qword ptr [rcx + 0xe0]
0x140904b16: mov r12, qword ptr [rax + 0x1f0]
0x140904b1d: lea r14, [r12 + 0x38]
0x140904b22: mov byte ptr [rbp + 0x2ff], 1
0x140904b29: movabs rdx, 0x662978458699a5ef
0x140904b33: movabs r8, 0x3ec057c162419b46
0x140904b3d: mov rcx, r14
0x140904b40: call 0x1400dd6f0
0x140904b45: test rax, rax
0x140904b48: je 0x140904f39
0x140904b4e: cmp qword ptr [r12 + 0x218], rdx
0x140904b56: jbe 0x140904f39
0x140904b5c: mov rax, qword ptr [r12 + 0x210]
0x140904b64: mov rcx, qword ptr [rax + rdx*8]
0x140904b68: test rcx, rcx
0x140904b6b: je 0x140904f39
0x140904b71: mov rax, qword ptr [r12 + 0x1e0]
0x140904b79: not rcx
0x140904b7c: lea rcx, [rcx + rcx*2]
0x140904b80: shl rcx, 5
0x140904b84: cmp qword ptr [rax + rcx + 0x20], 0
0x140904b8a: je 0x140904f39
0x140904b90: add rax, rcx
0x140904b93: mov rcx, qword ptr [rbp + 0x2f0]
0x140904b9a: movzx ecx, byte ptr [rcx + 0xf0]
0x140904ba1: shl cl, 3
0x140904ba4: movabs rbx, 0x302010000
0x140904bae: shr rbx, cl
0x140904bb1: mov r13, qword ptr [rax + 0x10]
0x140904bb5: movzx eax, sil
0x140904bb9: mov qword ptr [rbp + 0x2d8], rax
0x140904bc0: lea rax, [rax + rax*2]
0x140904bc4: shl eax, 6
0x140904bc7: lea r15, [rax + r13]
0x140904bcb: mov rax, qword ptr [r13 + rax]
0x140904bd0: mov ecx, eax
0x140904bd2: and ecx, 8
0x140904bd5: shr ecx, 3
0x140904bd8: cmp rax, -0x10
0x140904bdc: setae dl
0x140904bdf: or dl, cl
0x140904be1: je 0x140904bfc
0x140904be3: mov byte ptr [rbp + 0x2ff], 1
0x140904bea: mov rcx, r15
0x140904bed: xor edx, edx
0x140904bef: mov r9d, 0x3b9aca00
0x140904bf5: call 0x1428cc4b0
0x140904bfa: jmp 0x140904c07
0x140904bfc: lea rcx, [rax + 0x10]
0x140904c00: lock cmpxchg qword ptr [r15], rcx
0x140904c05: jne 0x140904be3
0x140904c07: mov byte ptr [rbp + 0x2ff], 1
0x140904c0e: mov rcx, r15
0x140904c11: call 0x140c81c70
0x140904c16: mov rdi, r15
0x140904c19: or rdi, 1
0x140904c1d: mov byte ptr [rbp + 0x2ff], 1
0x140904c24: mov rcx, rdi
0x140904c27: call 0x140c81c70
0x140904c2c: mov rax, qword ptr [r15 + 8]
0x140904c30: movabs rcx, 0x8000000000000001
0x140904c3a: cmp rax, rcx
0x140904c3d: mov qword ptr [rbp + 0x2e8], r15
0x140904c44: je 0x140904f63
0x140904c4a: lea rcx, [r15 + 0xa8]
0x140904c51: lea rdx, [r15 + 8]
0x140904c55: neg rax
0x140904c58: cmovo rcx, rdx
0x140904c5c: mov rax, qword ptr [rcx + 8]
0x140904c60: mov qword ptr [rbp + 0x2e0], rax
0x140904c67: jno 0x140904c6e
0x140904c69: xor r15d, r15d
0x140904c6c: jmp 0x140904c8c
0x140904c6e: mov rcx, qword ptr [r15 + 0x98]
0x140904c75: mov rax, qword ptr [r15 + 0xa0]
0x140904c7c: mov rax, qword ptr [rax + 0x38]
0x140904c80: mov byte ptr [rbp + 0x2fe], 1
0x140904c87: call rax
0x140904c89: mov r15, rax
0x140904c8c: lea eax, [rsi*4]
0x140904c93: or al, bl
0x140904c95: movzx ebx, al
0x140904c98: mov rax, qword ptr [r13 + 0x1450]
0x140904c9f: mov rdx, qword ptr [rax + 0x28]
0x140904ca3: cmp rdx, rbx
0x140904ca6: jbe 0x140904f71
0x140904cac: mov rax, qword ptr [rax + 0x20]
0x140904cb0: shl ebx, 2
0x140904cb3: mov r13d, dword ptr [rax + rbx]
0x140904cb7: mov byte ptr [rbp + 0x2fe], 1
0x140904cbe: movabs rdx, 0x8226cc786e679355
0x140904cc8: movabs r8, 0x65cfda6a55a841ac
0x140904cd2: mov rcx, r14
0x140904cd5: call 0x1400dd6f0
0x140904cda: test rax, rax
0x140904cdd: je 0x140904f4e
0x140904ce3: cmp qword ptr [r12 + 0x218], rdx
0x140904ceb: jbe 0x140904f4e
0x140904cf1: mov rax, qword ptr [r12 + 0x210]
0x140904cf9: mov rcx, qword ptr [rax + rdx*8]
0x140904cfd: test rcx, rcx
0x140904d00: je 0x140904f4e
0x140904d06: mov rax, qword ptr [r12 + 0x1e0]
0x140904d0e: not rcx
0x140904d11: lea rcx, [rcx + rcx*2]
0x140904d15: shl rcx, 5
0x140904d19: cmp qword ptr [rax + rcx + 0x20], 0
0x140904d1f: je 0x140904f4e
0x140904d25: add rax, rcx
0x140904d28: mov r12, qword ptr [rax + 0x10]
0x140904d2c: mov rax, qword ptr [rbp + 0x2f0]
0x140904d33: mov r14, qword ptr [rax + 0xe0]
0x140904d3a: mov r8, qword ptr [r14 + 0xd0]
0x140904d41: mov byte ptr [rbp + 0x2fe], 1
0x140904d48: lea rcx, [rbp + 0x2a8]
0x140904d4f: mov rdx, r12
0x140904d52: call 0x1412d0b20
0x140904d57: mov r8, qword ptr [r14 + 0xd0]
0x140904d5e: mov byte ptr [rbp + 0x2fe], 1
0x140904d65: lea rcx, [rbp + 0x2c0]
0x140904d6c: mov rdx, r12
0x140904d6f: call 0x1412cf1a0
0x140904d74: lea r12, [rbp - 0x40]
0x140904d78: mov r8d, 0xf8
0x140904d7e: mov rcx, r12
0x140904d81: mov rdx, qword ptr [rbp + 0x2f0]
0x140904d88: call 0x142923c00
0x140904d8d: mov byte ptr [rbp + 0x2fe], 0
0x140904d94: lea rcx, [rbp + 0xb8]
0x140904d9b: movabs r8, 0xcdf9c36a6a9c8c80
0x140904da5: mov rdx, r12
0x140904da8: call 0x14162e690
0x140904dad: test r15, r15
0x140904db0: je 0x140904df3
0x140904db2: mov byte ptr [rbp + 0x2fe], 0
0x140904db9: lea rcx, [rbp + 0x1b0]
0x140904dc0: lea rdx, [rbp + 0xb8]
0x140904dc7: movabs r8, 0x2106412d631790c6
0x140904dd1: mov r9, r15
0x140904dd4: call 0x140906150
0x140904dd9: shl r13, 2
0x140904ddd: lea r15, [r13*4]
0x140904de5: add r15, r13
0x140904de8: cmp byte ptr [rip + 0x2d2ffad], 0
0x140904def: je 0x140904e24
0x140904df1: jmp 0x140904e33
0x140904df3: lea rcx, [rbp + 0x1b0]
0x140904dfa: lea rdx, [rbp + 0xb8]
0x140904e01: mov r8d, 0xf8
0x140904e07: call 0x142923c00
0x140904e0c: shl r13, 2
0x140904e10: lea r15, [r13*4]
0x140904e18: add r15, r13
0x140904e1b: cmp byte ptr [rip + 0x2d2ff7a], 0
0x140904e22: jne 0x140904e33
0x140904e24: cmp sil, 1
0x140904e28: ja 0x140904e95
0x140904e2a: cmp byte ptr [rip + 0x2d2ff6c], 0
0x140904e31: jne 0x140904e95
0x140904e33: lea rsi, [rbp - 0x40]
0x140904e37: lea rdx, [rbp + 0x1b0]
0x140904e3e: mov r8d, 0xf8
0x140904e44: mov rcx, rsi
0x140904e47: call 0x142923c00
0x140904e4c: lea rax, [rip + 0x21a6435]
0x140904e53: mov rcx, qword ptr [rbp + 0x2d8]
0x140904e5a: mov rax, qword ptr [rax + rcx*8]
0x140904e5e: mov byte ptr [rbp + 0x2fe], 0
0x140904e65: mov qword ptr [rsp + 0x30], rax
0x140904e6a: mov qword ptr [rsp + 0x28], rbx
0x140904e6f: lea rax, [rbp + 0x2a8]
0x140904e76: mov qword ptr [rsp + 0x20], rax
0x140904e7b: lea rdx, [rbp + 0x2c0]
0x140904e82: mov r9d, 0x14
0x140904e88: mov rcx, rsi
0x140904e8b: mov r8, r15
0x140904e8e: call 0x140907f90
0x140904e93: jmp 0x140904ed9
0x140904e95: lea rsi, [rbp - 0x40]
0x140904e99: lea rdx, [rbp + 0x1b0]
0x140904ea0: mov r8d, 0xf8
0x140904ea6: mov rcx, rsi
0x140904ea9: call 0x142923c00
0x140904eae: mov byte ptr [rbp + 0x2fe], 0
0x140904eb5: mov rax, qword ptr [rbp + 0x2e0]
0x140904ebc: mov qword ptr [rsp + 0x20], rax
0x140904ec1: lea rdx, [rbp + 0x2c0]
0x140904ec8: mov r9d, 0x14
0x140904ece: mov rcx, rsi
0x140904ed1: mov r8, r15
0x140904ed4: call 0x140907cc0
0x140904ed9: mov byte ptr [rbp + 0x2ff], 0
0x140904ee0: mov rsi, qword ptr [rbp + 0x2e8]
0x140904ee7: mov rcx, rsi
0x140904eea: call 0x140c81e10
0x140904eef: mov byte ptr [rbp + 0x2ff], 0
0x140904ef6: mov rcx, rdi
0x140904ef9: call 0x140c81e10
0x140904efe: mov rax, 0xfffffffffffffff0
0x140904f05: lock xadd qword ptr [rsi], rax
0x140904f0a: and rax, 0xfffffffffffffff2
0x140904f0e: cmp rax, 0x12
0x140904f12: je 0x140904f28
0x140904f14: add rsp, 0x388
0x140904f1b: pop rbx
0x140904f1c: pop rdi
0x140904f1d: pop rsi
0x140904f1e: pop r12
0x140904f20: pop r13
0x140904f22: pop r14
0x140904f24: pop r15
0x140904f26: pop rbp
0x140904f27: ret 
0x140904f28: mov byte ptr [rbp + 0x2ff], 0
0x140904f2f: mov rcx, rsi
0x140904f32: call 0x1428ccf50
0x140904f37: jmp 0x140904f14
0x140904f39: mov byte ptr [rbp + 0x2ff], 1
0x140904f40: lea rcx, [rip + 0x21a5641]
0x140904f47: call 0x1428d9310
0x140904f4c: jmp 0x140904f87
0x140904f4e: mov byte ptr [rbp + 0x2fe], 1
0x140904f55: lea rcx, [rip + 0x21a5644]
0x140904f5c: call 0x1428d9310
0x140904f61: jmp 0x140904f87
0x140904f63: lea rcx, [rip + 0x21b54a6]
0x140904f6a: call 0x1428d9310
0x140904f6f: jmp 0x140904f87
0x140904f71: mov byte ptr [rbp + 0x2fe], 1
0x140904f78: lea r8, [rip + 0x21b5571]
0x140904f7f: mov rcx, rbx
0x140904f82: call 0x1428d9518
0x140904f87: ud2 
0x140904f89: nop dword ptr [rax]
0x140904f90: mov qword ptr [rsp + 0x10], rdx
0x140904f95: push rbp
0x140904f96: push r15
0x140904f98: push r14
0x140904f9a: push r13
0x140904f9c: push r12
0x140904f9e: push rsi
0x140904f9f: push rdi
0x140904fa0: push rbx
0x140904fa1: sub rsp, 0x38
0x140904fa5: lea rbp, [rdx + 0x80]
0x140904fac: mov byte ptr [rbp + 0x2ff], 1
0x140904fb3: mov rcx, qword ptr [rbp + 0x2e8]
0x140904fba: call 0x140041280
0x140904fbf: mov byte ptr [rbp + 0x2ff], 1
0x140904fc6: add rsp, 0x38
0x140904fca: pop rbx
0x140904fcb: pop rdi
0x140904fcc: pop rsi
0x140904fcd: pop r12
0x140904fcf: pop r13
0x140904fd1: pop r14
0x140904fd3: pop r15
0x140904fd5: pop rbp
0x140904fd6: ret 
0x140904fd7: nop word ptr [rax + rax]
0x140904fe0: mov qword ptr [rsp + 0x10], rdx
0x140904fe5: push rbp
0x140904fe6: push r15
0x140904fe8: push r14
0x140904fea: push r13
0x140904fec: push r12
0x140904fee: push rsi
0x140904fef: push rdi
0x140904ff0: push rbx
0x140904ff1: sub rsp, 0x38
0x140904ff5: lea rbp, [rdx + 0x80]
0x140904ffc: movzx ebx, byte ptr [rbp + 0x2fe]
0x140905003: mov rcx, qword ptr [rbp + 0x2e8]
0x14090500a: call 0x140041280
0x14090500f: mov byte ptr [rbp + 0x2ff], bl
0x140905015: add rsp, 0x38
0x140905019: pop rbx
0x14090501a: pop rdi
0x14090501b: pop rsi
0x14090501c: pop r12
0x14090501e: pop r13
0x140905020: pop r14
0x140905022: pop r15
0x140905024: pop rbp
0x140905025: ret 
0x140905026: nop word ptr cs:[rax + rax]
0x140905030: mov qword ptr [rsp + 0x10], rdx
0x140905035: push rbp
0x140905036: push r15
0x140905038: push r14
0x14090503a: push r13
0x14090503c: push r12
0x14090503e: push rsi
0x14090503f: push rdi
0x140905040: push rbx
0x140905041: sub rsp, 0x38
0x140905045: lea rbp, [rdx + 0x80]
0x14090504c: cmp byte ptr [rbp + 0x2ff], 0
0x140905053: jne 0x140905066
0x140905055: add rsp, 0x38
0x140905059: pop rbx
0x14090505a: pop rdi
0x14090505b: pop rsi
0x14090505c: pop r12
0x14090505e: pop r13
0x140905060: pop r14
0x140905062: pop r15
0x140905064: pop rbp
0x140905065: ret 
0x140905066: mov rcx, qword ptr [rbp + 0x2f0]
0x14090506d: call 0x1408935c0
0x140905072: jmp 0x140905055
