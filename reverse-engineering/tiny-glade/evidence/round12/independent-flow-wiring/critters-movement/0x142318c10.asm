142318c10: push r15
142318c12: push r14
142318c14: push r13
142318c16: push r12
142318c18: push rsi
142318c19: push rdi
142318c1a: push rbp
142318c1b: push rbx
142318c1c: sub rsp, 0x88
142318c23: movaps xmmword ptr [rsp + 0x70], xmm7
142318c28: movaps xmmword ptr [rsp + 0x60], xmm6
142318c2d: mov rax, qword ptr [rdx]
142318c30: movss xmm6, dword ptr [rax + 0x28]
142318c35: mov rdx, qword ptr [rcx]
142318c38: mov rax, qword ptr [rcx + 8]
142318c3c: mov qword ptr [rsp + 0x50], rax
142318c41: mov eax, dword ptr [rcx + 0x14]
142318c44: mov dword ptr [rsp + 0x3c], eax
142318c48: mov r14, qword ptr [rdx + 0xf8]
142318c4f: mov rax, qword ptr [rdx + 0x100]
142318c56: lea r15, [r14 + rax*4]
142318c5a: mov qword ptr [rsp + 0x48], rdx
142318c5f: cmp byte ptr [rdx + 0x108], 0
142318c66: je 0x142318c8c
142318c68: mov qword ptr [rsp + 0x30], 0
142318c71: movss xmm7, dword ptr [rip + 0x60cc8b]
142318c79: mov qword ptr [rsp + 0x28], 0
142318c82: xor r12d, r12d
142318c85: xor esi, esi
142318c87: jmp 0x142318d70
142318c8c: mov eax, 8
142318c91: mov qword ptr [rsp + 0x40], rax
142318c96: mov qword ptr [rsp + 0x30], 0
142318c9f: movss xmm7, dword ptr [rip + 0x60cc5d]
142318ca7: mov qword ptr [rsp + 0x28], 0
142318cb0: xor r12d, r12d
142318cb3: xor esi, esi
142318cb5: jmp 0x142318ec7
142318cba: mov rcx, qword ptr [rcx + 0x18]
142318cbe: mov qword ptr [rsp + 0x28], 0
142318cc7: mov rdx, qword ptr [rsp + 0x48]
142318ccc: mov rdx, qword ptr [rdx + 0x118]
142318cd3: add r14, 4
142318cd7: mov rax, qword ptr [rax + rdx*8]
142318cdb: not rax
142318cde: lea rax, [rax + rax*2]
142318ce2: shl rax, 4
142318ce6: mov rdx, qword ptr [rcx + rax + 0x10]
142318ceb: mov qword ptr [rsp + 0x30], rdx
142318cf0: mov rax, qword ptr [rcx + rax + 0x28]
142318cf5: mov qword ptr [rsp + 0x40], rax
142318cfa: xor esi, esi
142318cfc: mov ebx, esi
142318cfe: mov rbp, rbx
142318d01: shl rbp, 4
142318d05: lea r13, [rbx + rbx*2]
142318d09: inc rsi
142318d0c: mov rdi, qword ptr [rsp + 0x28]
142318d11: movss xmm0, dword ptr [rdi + rbp + 0xc]
142318d17: movaps xmm1, xmm6
142318d1a: call 0x142923f70 ; 
142318d1f: movaps xmm1, xmm7
142318d22: subss xmm1, xmm0
142318d26: movsldup xmm2, xmm1
142318d2a: mulss xmm1, dword ptr [rdi + rbp + 8]
142318d30: movsldup xmm3, xmm0
142318d34: mov rcx, qword ptr [rsp + 0x30]
142318d39: mulss xmm0, dword ptr [rcx + r13*4 + 8]
142318d40: addss xmm0, xmm1
142318d44: movsd xmm1, qword ptr [rdi + rbp]
142318d49: movsd xmm4, qword ptr [rcx + r13*4]
142318d4f: mulps xmm1, xmm2
142318d52: mulps xmm4, xmm3
142318d55: addps xmm4, xmm1
142318d58: mov eax, dword ptr [rsp + 0x3c]
142318d5c: mov rdx, qword ptr [rsp + 0x40]
142318d61: mov dword ptr [rdx + rbx*4], eax
142318d64: movlps qword ptr [rcx + r13*4], xmm4
142318d69: movss dword ptr [rcx + r13*4 + 8], xmm0
142318d70: cmp rsi, r12
142318d73: jne 0x142318cfc
142318d75: cmp r14, r15
142318d78: je 0x142318f74
142318d7e: mov rax, qword ptr [rsp + 0x50]
142318d83: mov rax, qword ptr [rax + 0x1a8]
142318d8a: nop word ptr [rax + rax]
142318d90: mov ecx, dword ptr [r14]
142318d93: lea rcx, [rcx + rcx*8]
142318d97: mov r12, qword ptr [rax + rcx*8 + 0x10]
142318d9c: test r12, r12
142318d9f: jne 0x142318db0
142318da1: add r14, 4
142318da5: cmp r14, r15
142318da8: jne 0x142318d90
142318daa: jmp 0x142318f74
142318daf: nop 
142318db0: lea rcx, [rax + rcx*8]
142318db4: mov rax, qword ptr [rsp + 0x48]
142318db9: mov rdx, qword ptr [rax + 0x110]
142318dc0: mov rax, qword ptr [rcx + 0x38]
142318dc4: cmp qword ptr [rcx + 0x40], rdx
142318dc8: jbe 0x142318cba
142318dce: mov rdx, qword ptr [rax + rdx*8]
142318dd2: mov rcx, qword ptr [rcx + 0x18]
142318dd6: test rdx, rdx
142318dd9: je 0x142318cbe
142318ddf: not rdx
142318de2: lea rdx, [rdx + rdx*2]
142318de6: shl rdx, 4
142318dea: mov rdx, qword ptr [rcx + rdx + 0x10]
142318def: mov qword ptr [rsp + 0x28], rdx
142318df4: jmp 0x142318cc7
142318df9: mov rdx, qword ptr [rdx + 0x18]
142318dfd: mov qword ptr [rsp + 0x28], 0
142318e06: mov r8, qword ptr [rsp + 0x48]
142318e0b: mov r8, qword ptr [r8 + 0x118]
142318e12: add r14, 4
142318e16: mov rcx, qword ptr [rcx + r8*8]
142318e1a: not rcx
142318e1d: lea rcx, [rcx + rcx*2]
142318e21: shl rcx, 4
142318e25: mov r8, qword ptr [rdx + rcx + 0x10]
142318e2a: mov qword ptr [rsp + 0x30], r8
142318e2f: mov rcx, qword ptr [rdx + rcx + 0x28]
142318e34: mov qword ptr [rsp + 0x58], rcx
142318e39: mov rax, qword ptr [rax + 0x50]
142318e3d: mov qword ptr [rsp + 0x40], rax
142318e42: xor esi, esi
142318e44: mov rax, rsi
142318e47: shl rax, 4
142318e4b: mov rcx, qword ptr [rsp + 0x40]
142318e50: mov edi, dword ptr [rcx + rax + 8]
142318e54: mov rbx, rdi
142318e57: shl rbx, 4
142318e5b: lea r13, [rdi + rdi*2]
142318e5f: inc rsi
142318e62: mov rbp, qword ptr [rsp + 0x28]
142318e67: movss xmm0, dword ptr [rbp + rbx + 0xc]
142318e6d: movaps xmm1, xmm6
142318e70: call 0x142923f70 ; 
142318e75: movaps xmm1, xmm7
142318e78: subss xmm1, xmm0
142318e7c: movsldup xmm2, xmm1
142318e80: mulss xmm1, dword ptr [rbp + rbx + 8]
142318e86: movsldup xmm3, xmm0
142318e8a: mov rcx, qword ptr [rsp + 0x30]
142318e8f: mulss xmm0, dword ptr [rcx + r13*4 + 8]
142318e96: addss xmm0, xmm1
142318e9a: movsd xmm1, qword ptr [rbp + rbx]
142318ea0: movsd xmm4, qword ptr [rcx + r13*4]
142318ea6: mulps xmm1, xmm2
142318ea9: mulps xmm4, xmm3
142318eac: addps xmm4, xmm1
142318eaf: mov eax, dword ptr [rsp + 0x3c]
142318eb3: mov rdx, qword ptr [rsp + 0x58]
142318eb8: mov dword ptr [rdx + rdi*4], eax
142318ebb: movlps qword ptr [rcx + r13*4], xmm4
142318ec0: movss dword ptr [rcx + r13*4 + 8], xmm0
142318ec7: cmp rsi, r12
142318eca: jne 0x142318e44
142318ed0: cmp r14, r15
142318ed3: je 0x142318f74
142318ed9: mov rax, qword ptr [rsp + 0x50]
142318ede: mov rax, qword ptr [rax + 0x100]
142318ee5: nop word ptr cs:[rax + rax]
142318ef0: mov ecx, dword ptr [r14]
142318ef3: lea rcx, [rcx + rcx*4]
142318ef7: shl rcx, 5
142318efb: mov r12, qword ptr [rax + rcx + 0x58]
142318f00: test r12, r12
142318f03: jne 0x142318f10
142318f05: add r14, 4
142318f09: cmp r14, r15
142318f0c: jne 0x142318ef0
142318f0e: jmp 0x142318f74
142318f10: add rax, rcx
142318f13: mov ecx, dword ptr [rax + 0x94]
142318f19: mov rdx, qword ptr [rsp + 0x50]
142318f1e: mov r9, qword ptr [rdx + 0x1a8]
142318f25: lea r10, [rcx + rcx*8]
142318f29: lea rdx, [r9 + r10*8]
142318f2d: mov rcx, qword ptr [rsp + 0x48]
142318f32: mov r8, qword ptr [rcx + 0x110]
142318f39: mov rcx, qword ptr [r9 + r10*8 + 0x38]
142318f3e: cmp qword ptr [r9 + r10*8 + 0x40], r8
142318f43: jbe 0x142318df9
142318f49: mov r8, qword ptr [rcx + r8*8]
142318f4d: mov rdx, qword ptr [rdx + 0x18]
142318f51: test r8, r8
142318f54: je 0x142318dfd
142318f5a: not r8
142318f5d: lea r8, [r8 + r8*2]
142318f61: shl r8, 4
142318f65: mov r8, qword ptr [rdx + r8 + 0x10]
142318f6a: mov qword ptr [rsp + 0x28], r8
142318f6f: jmp 0x142318e06
142318f74: movaps xmm6, xmmword ptr [rsp + 0x60]
142318f79: movaps xmm7, xmmword ptr [rsp + 0x70]
142318f7e: add rsp, 0x88
142318f85: pop rbx
142318f86: pop rbp
142318f87: pop rdi
142318f88: pop rsi
142318f89: pop r12
142318f8b: pop r13
142318f8d: pop r14
142318f8f: pop r15
142318f91: ret 
