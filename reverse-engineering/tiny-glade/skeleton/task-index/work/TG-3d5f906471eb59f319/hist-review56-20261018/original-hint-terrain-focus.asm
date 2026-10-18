142283980: push rbp
142283981: push r15
142283983: push r14
142283985: push r13
142283987: push r12
142283989: push rsi
14228398a: push rdi
14228398b: push rbx
14228398c: sub rsp, 0x118
142283993: lea rbp, [rsp + 0x80]
14228399b: mov qword ptr [rbp + 0x90], 0xfffffffffffffffe
1422839a6: mov qword ptr [rbp + 0x80], rdx
1422839ad: mov qword ptr [rbp + 0x78], rcx
1422839b1: mov r15, qword ptr [r9]
1422839b4: mov rdx, qword ptr [r9 + 8]
1422839b8: mov r11, qword ptr [r15]
1422839bb: xor r10d, r10d
1422839be: mov rax, r11
1422839c1: sub rax, qword ptr [rdx + 0x18]
1422839c5: cmovb rax, r10
1422839c9: sub r11, qword ptr [rdx + 0x38]
1422839cd: cmovb r11, r10
1422839d1: mov rcx, rax
1422839d4: shl rcx, 4
1422839d8: add rcx, qword ptr [rdx + 8]
1422839dc: mov r12, qword ptr [rdx + 0x10]
1422839e0: sub r12, rax
1422839e3: cmovb r12, r10
1422839e7: mov r9, qword ptr [rbp + 0x100]
1422839ee: mov esi, 8
1422839f3: cmovb rcx, rsi
1422839f7: mov rax, r11
1422839fa: shl rax, 4
1422839fe: add rax, qword ptr [rdx + 0x28]
142283a02: mov rdi, qword ptr [rdx + 0x30]
142283a06: sub rdi, r11
142283a09: cmovb rdi, r10
142283a0d: cmovb rax, rsi
142283a11: mov rdx, qword ptr [rdx + 0x40]
142283a15: sub rdx, rdi
142283a18: sub rdx, r12
142283a1b: mov qword ptr [r15], rdx
142283a1e: mov rdx, rdi
142283a21: shl r12, 4
142283a25: add r12, rcx
142283a28: shl rdx, 4
142283a2c: add rdx, rax
142283a2f: mov r11d, dword ptr [r8 + 0x1c]
142283a33: mov r10, qword ptr [r8]
142283a36: mov r8, qword ptr [r8 + 0x10]
142283a3a: mov esi, dword ptr [r9 + 0x1c]
142283a3e: mov dword ptr [rbp + 0x88], esi
142283a44: mov rsi, qword ptr [r9]
142283a47: mov r9, qword ptr [r9 + 0x10]
142283a4b: lea rdi, [rsi + 0x20]
142283a4f: mov qword ptr [rbp + 0x70], rdi
142283a53: mov qword ptr [rbp + 0x50], r15
142283a57: mov qword ptr [rbp + 0x58], r12
142283a5b: mov qword ptr [rbp + 0x60], rdx
142283a5f: mov dword ptr [rbp + 0x8c], r11d
142283a66: mov qword ptr [rbp + 0x68], r9
142283a6a: jmp 0x142283a95
142283a6c: nop dword ptr [rax]
142283a70: mov rax, qword ptr [rsi + 0x28]
142283a74: mov rcx, rdi
142283a77: shl rcx, 4
142283a7b: mov qword ptr [rax + rcx], r13
142283a7f: mov byte ptr [rax + rcx + 8], 5
142283a84: inc rdi
142283a87: mov qword ptr [rsi + 0x30], rdi
142283a8b: inc qword ptr [rsi + 0x40]
142283a8f: mov rax, r14
142283a92: mov rcx, rbx
142283a95: test rcx, rcx
142283a98: je 0x142283ac0
142283a9a: cmp rcx, r12
142283a9d: je 0x142283ac0
142283a9f: lea rbx, [rcx + 0x10]
142283aa3: mov r14, rax
142283aa6: inc qword ptr [r15]
142283aa9: cmp byte ptr [rcx + 8], 2
142283aad: mov rcx, rbx
142283ab0: jne 0x142283a95
142283ab2: jmp 0x142283ae2
142283ab4: nop word ptr cs:[rax + rax]
142283ac0: test rax, rax
142283ac3: je 0x142283b3c
142283ac5: cmp rax, rdx
142283ac8: je 0x142283b3c
142283aca: lea r14, [rax + 0x10]
142283ace: xor ebx, ebx
142283ad0: mov rcx, rax
142283ad3: mov rax, r14
142283ad6: inc qword ptr [r15]
142283ad9: cmp byte ptr [rcx + 8], 2
142283add: mov rcx, rbx
142283ae0: jne 0x142283a95
142283ae2: mov dword ptr [r8], r11d
142283ae5: mov byte ptr [r10], 1
142283ae9: mov eax, dword ptr [rbp + 0x88]
142283aef: mov dword ptr [r9], eax
142283af2: mov rdi, qword ptr [rsi + 0x30]
142283af6: mov r13, qword ptr [rsi + 0x40]
142283afa: cmp rdi, qword ptr [rsi + 0x20]
142283afe: jne 0x142283a70
142283b04: mov rcx, qword ptr [rbp + 0x70]
142283b08: lea rdx, [rip + 0xdb49c1]
142283b0f: mov r15, r10
142283b12: mov r12, r8
142283b15: call 0x140562630
142283b1a: mov r9, qword ptr [rbp + 0x68]
142283b1e: mov r8, r12
142283b21: mov r11d, dword ptr [rbp + 0x8c]
142283b28: mov r10, r15
142283b2b: mov rdx, qword ptr [rbp + 0x60]
142283b2f: mov r12, qword ptr [rbp + 0x58]
142283b33: mov r15, qword ptr [rbp + 0x50]
142283b37: jmp 0x142283a70
142283b3c: mov rax, qword ptr [rbp + 0x78]
142283b40: mov rax, qword ptr [rax]
142283b43: cmp dword ptr [rax], 0xa
142283b46: jne 0x142283c5e
142283b4c: mov rax, qword ptr [rbp + 0x80]
142283b53: mov rdx, qword ptr [rax]
142283b56: cmp byte ptr [rdx + 0x1fb], 0
142283b5d: jne 0x142283c5e
142283b63: mov rax, qword ptr [rbp + 0x110]
142283b6a: mov rax, qword ptr [rax]
142283b6d: cmp qword ptr [rax], 0
142283b71: je 0x142283c5e
142283b77: cmp byte ptr [r10], 0
142283b7b: jne 0x142283c5e
142283b81: lea rcx, [rbp - 0x48]
142283b85: call 0x141c3de00
142283b8a: mov word ptr [rbp - 0x18], 0x100
142283b90: mov dword ptr [rbp - 0x60], 6
142283b97: mov rdx, qword ptr [rbp + 0x108]
142283b9e: mov eax, dword ptr [rdx + 0x1c]
142283ba1: mov rcx, qword ptr [rdx + 0x10]
142283ba5: mov dword ptr [rcx], eax
142283ba7: mov rsi, qword ptr [rdx]
142283baa: mov rax, qword ptr [rsi + 0x40]
142283bae: mov qword ptr [rbp + 0x40], rax
142283bb2: mov rax, qword ptr [rbp - 0x20]
142283bb6: mov qword ptr [rbp + 0x30], rax
142283bba: movzx eax, word ptr [rbp - 0x18]
142283bbe: mov word ptr [rbp + 0x38], ax
142283bc2: mov eax, dword ptr [rbp - 0x16]
142283bc5: mov dword ptr [rbp + 0x3a], eax
142283bc8: movzx eax, word ptr [rbp - 0x12]
142283bcc: mov word ptr [rbp + 0x3e], ax
142283bd0: mov eax, dword ptr [rbp - 0x60]
142283bd3: mov rcx, qword ptr [rbp - 0x5c]
142283bd7: mov edx, dword ptr [rbp - 0x54]
142283bda: movups xmm0, xmmword ptr [rbp - 0x50]
142283bde: movups xmm1, xmmword ptr [rbp - 0x40]
142283be2: movups xmm2, xmmword ptr [rbp - 0x30]
142283be6: movaps xmmword ptr [rbp + 0x20], xmm2
142283bea: movaps xmmword ptr [rbp + 0x10], xmm1
142283bee: movaps xmmword ptr [rbp], xmm0
142283bf2: mov dword ptr [rbp - 0x10], eax
142283bf5: mov qword ptr [rbp - 0xc], rcx
142283bf9: mov dword ptr [rbp - 4], edx
142283bfc: mov rdi, qword ptr [rsi + 0x30]
142283c00: cmp rdi, qword ptr [rsi + 0x20]
142283c04: jne 0x142283c16
142283c06: lea rcx, [rsi + 0x20]
142283c0a: lea rdx, [rip + 0xdb48bf]
142283c11: call 0x140562f30
142283c16: mov rax, qword ptr [rsi + 0x28]
142283c1a: imul rcx, rdi, 0x58
142283c1e: movaps xmm0, xmmword ptr [rbp - 0x10]
142283c22: movaps xmm1, xmmword ptr [rbp]
142283c26: movaps xmm2, xmmword ptr [rbp + 0x10]
142283c2a: movaps xmm3, xmmword ptr [rbp + 0x20]
142283c2e: movaps xmm4, xmmword ptr [rbp + 0x30]
142283c32: mov rdx, qword ptr [rbp + 0x40]
142283c36: mov qword ptr [rax + rcx + 0x50], rdx
142283c3b: movups xmmword ptr [rax + rcx + 0x40], xmm4
142283c40: movups xmmword ptr [rax + rcx + 0x30], xmm3
142283c45: movups xmmword ptr [rax + rcx + 0x20], xmm2
142283c4a: movups xmmword ptr [rax + rcx + 0x10], xmm1
142283c4f: movups xmmword ptr [rax + rcx], xmm0
142283c53: inc rdi
142283c56: mov qword ptr [rsi + 0x30], rdi
142283c5a: inc qword ptr [rsi + 0x40]
142283c5e: add rsp, 0x118
142283c65: pop rbx
142283c66: pop rdi
142283c67: pop rsi
142283c68: pop r12
142283c6a: pop r13
142283c6c: pop r14
142283c6e: pop r15
142283c70: pop rbp
142283c71: ret 
142283c72: nop word ptr cs:[rax + rax]
142283c80: mov qword ptr [rsp + 0x10], rdx
142283c85: push rbp
142283c86: push r15
142283c88: push r14
142283c8a: push r13
142283c8c: push r12
142283c8e: push rsi
142283c8f: push rdi
142283c90: push rbx
142283c91: sub rsp, 0x28
142283c95: lea rbp, [rdx + 0x80]
142283c9c: lea rcx, [rbp + 8]
142283ca0: call 0x140eaaa50
142283ca5: nop 
142283ca6: add rsp, 0x28
142283caa: pop rbx
142283cab: pop rdi
142283cac: pop rsi
142283cad: pop r12
142283caf: pop r13
142283cb1: pop r14
142283cb3: pop r15
142283cb5: pop rbp
142283cb6: ret 
