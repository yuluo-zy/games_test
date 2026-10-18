140af5b30: push r15
140af5b32: push r14
140af5b34: push r13
140af5b36: push r12
140af5b38: push rsi
140af5b39: push rdi
140af5b3a: push rbp
140af5b3b: push rbx
140af5b3c: sub rsp, 0x138
140af5b43: mov rax, qword ptr [rsp + 0x1c0]
140af5b4b: mov qword ptr [rsp + 0x70], rax
140af5b50: mov rax, qword ptr [rsp + 0x1b0]
140af5b58: mov qword ptr [rsp + 0x68], rax
140af5b5d: mov rsi, qword ptr [rsp + 0x1a8]
140af5b65: mov rdi, qword ptr [r9]
140af5b68: mov r11, qword ptr [r9 + 8]
140af5b6c: mov r9, qword ptr [rdi]
140af5b6f: xor ebx, ebx
140af5b71: mov rax, r9
140af5b74: sub rax, qword ptr [r11 + 0x18]
140af5b78: cmovb rax, rbx
140af5b7c: sub r9, qword ptr [r11 + 0x38]
140af5b80: cmovb r9, rbx
140af5b84: mov r13, rax
140af5b87: shl r13, 4
140af5b8b: add r13, qword ptr [r11 + 8]
140af5b8f: mov r14, qword ptr [r11 + 0x10]
140af5b93: sub r14, rax
140af5b96: cmovb r14, rbx
140af5b9a: mov r15, qword ptr [r11 + 0x30]
140af5b9e: mov r10d, 8
140af5ba4: cmovb r13, r10
140af5ba8: mov rax, r9
140af5bab: shl rax, 4
140af5baf: add rax, qword ptr [r11 + 0x28]
140af5bb3: sub r15, r9
140af5bb6: mov r9, qword ptr [rsp + 0x1b8]
140af5bbe: cmovb r15, rbx
140af5bc2: cmovb rax, r10
140af5bc6: mov r11, qword ptr [r11 + 0x40]
140af5bca: sub r11, r15
140af5bcd: sub r11, r14
140af5bd0: mov qword ptr [rsp + 0xb0], rdi
140af5bd8: mov qword ptr [rdi], r11
140af5bdb: shl r14, 4
140af5bdf: add r14, r13
140af5be2: mov qword ptr [rsp + 0xa8], r14
140af5bea: shl r15, 4
140af5bee: add r15, rax
140af5bf1: mov qword ptr [rsp + 0x98], r15
140af5bf9: mov rbx, qword ptr [rdx]
140af5bfc: mov r10, qword ptr [rdx + 0x10]
140af5c00: mov qword ptr [rsp + 0x38], r10
140af5c05: mov edx, dword ptr [rdx + 0x1c]
140af5c08: mov dword ptr [rsp + 0x30], edx
140af5c0c: mov edx, dword ptr [r9 + 0x1c]
140af5c10: mov dword ptr [rsp + 0x4c], edx
140af5c14: mov rdi, qword ptr [r9]
140af5c17: mov rdx, qword ptr [r9 + 0x10]
140af5c1b: mov qword ptr [rsp + 0xa0], rdx
140af5c23: mov edx, dword ptr [rsi + 0x1c]
140af5c26: mov dword ptr [rsp + 0x48], edx
140af5c2a: mov r14, qword ptr [rsi]
140af5c2d: mov rdx, qword ptr [rsi + 0x10]
140af5c31: mov qword ptr [rsp + 0x90], rdx
140af5c39: mov qword ptr [rsp + 0x58], rcx
140af5c3e: mov rcx, qword ptr [rcx]
140af5c41: mov qword ptr [rsp + 0x28], rcx
140af5c46: mov ecx, dword ptr [r8 + 0x1c]
140af5c4a: mov dword ptr [rsp + 0x44], ecx
140af5c4e: mov rcx, qword ptr [r8]
140af5c51: mov qword ptr [rsp + 0x60], rcx
140af5c56: mov qword ptr [rsp + 0x50], r8
140af5c5b: mov rcx, qword ptr [r8 + 0x10]
140af5c5f: mov qword ptr [rsp + 0x80], rcx
140af5c67: lea rcx, [rdi + 0x20]
140af5c6b: mov qword ptr [rsp + 0x88], rcx
140af5c73: lea rcx, [r14 + 0x20]
140af5c77: mov qword ptr [rsp + 0x78], rcx
140af5c7c: jmp 0x140af5ca5
140af5c7e: nop 
140af5c80: mov rax, qword ptr [r14 + 0x28]
140af5c84: mov rcx, r13
140af5c87: shl rcx, 4
140af5c8b: mov qword ptr [rax + rcx], rsi
140af5c8f: mov byte ptr [rax + rcx + 8], r12b
140af5c94: inc r13
140af5c97: mov qword ptr [r14 + 0x30], r13
140af5c9b: inc qword ptr [r14 + 0x40]
140af5c9f: mov rax, rbp
140af5ca2: mov r13, r15
140af5ca5: test r13, r13
140af5ca8: je 0x140af5cc0
140af5caa: cmp r13, qword ptr [rsp + 0xa8]
140af5cb2: je 0x140af5cc0
140af5cb4: lea r15, [r13 + 0x10]
140af5cb8: mov rbp, rax
140af5cbb: jmp 0x140af5ce1
140af5cbd: nop dword ptr [rax]
140af5cc0: test rax, rax
140af5cc3: je 0x140af5f81
140af5cc9: cmp rax, qword ptr [rsp + 0x98]
140af5cd1: je 0x140af5f81
140af5cd7: lea rbp, [rax + 0x10]
140af5cdb: xor r15d, r15d
140af5cde: mov r13, rax
140af5ce1: mov rax, qword ptr [rsp + 0xb0]
140af5ce9: inc qword ptr [rax]
140af5cec: mov rax, qword ptr [rsp + 0x28]
140af5cf1: xor ecx, ecx
140af5cf3: cmp rcx, qword ptr [rax + 0x30]
140af5cf7: jno 0x140af5ea8
140af5cfd: movzx eax, byte ptr [r13 + 9]
140af5d02: lea rcx, [rip + 0x1fef717]
140af5d09: movsxd rax, dword ptr [rcx + rax*4]
140af5d0d: add rax, rcx
140af5d10: jmp rax
140af5d12: mov rax, qword ptr [rbx + 0x18]
140af5d16: test rax, rax
140af5d19: je 0x140af5d80
140af5d1b: mov rcx, qword ptr [rbx + 0x10]
140af5d1f: add rax, rcx
140af5d22: dec rax
140af5d25: mov rcx, qword ptr [rbx]
140af5d28: cmp rax, rcx
140af5d2b: mov edx, 0
140af5d30: cmovae rdx, rcx
140af5d34: mov rcx, qword ptr [rbx + 8]
140af5d38: sub rax, rdx
140af5d3b: cmp byte ptr [rcx + rax*4 + 1], 1
140af5d40: jne 0x140af5d80
140af5d42: jmp 0x140af5df8
140af5d47: nop word ptr [rax + rax]
140af5d50: mov rax, qword ptr [rbx + 0x18]
140af5d54: test rax, rax
140af5d57: je 0x140af5d80
140af5d59: mov rcx, qword ptr [rbx + 0x10]
140af5d5d: add rax, rcx
140af5d60: dec rax
140af5d63: mov rcx, qword ptr [rbx]
140af5d66: cmp rax, rcx
140af5d69: mov edx, 0
140af5d6e: cmovae rdx, rcx
140af5d72: mov rcx, qword ptr [rbx + 8]
140af5d76: sub rax, rdx
140af5d79: cmp byte ptr [rcx + rax*4 + 1], 0
140af5d7e: je 0x140af5df8
140af5d80: mov rax, qword ptr [rsp + 0x38]
140af5d85: mov ecx, dword ptr [rsp + 0x30]
140af5d89: mov dword ptr [rax], ecx
140af5d8b: mov r12d, dword ptr [r13 + 8]
140af5d8f: mov rcx, qword ptr [rbx]
140af5d92: mov rax, qword ptr [rbx + 0x18]
140af5d96: cmp rax, rcx
140af5d99: jne 0x140af5db1
140af5d9b: mov rcx, rbx
140af5d9e: lea rdx, [rip + 0x1fef493]
140af5da5: call 0x140b5ab60
140af5daa: mov rcx, qword ptr [rbx]
140af5dad: mov rax, qword ptr [rbx + 0x18]
140af5db1: add rax, qword ptr [rbx + 0x10]
140af5db5: cmp rax, rcx
140af5db8: mov edx, 0
140af5dbd: cmovae rdx, rcx
140af5dc1: sub rax, rdx
140af5dc4: mov rcx, qword ptr [rbx + 8]
140af5dc8: mov dword ptr [rcx + rax*4], r12d
140af5dcc: inc qword ptr [rbx + 0x18]
140af5dd0: jmp 0x140af5e5d
140af5dd5: nop word ptr cs:[rax + rax]
140af5de0: mov eax, dword ptr [rsp + 0x44]
140af5de4: mov rcx, qword ptr [rsp + 0x80]
140af5dec: mov dword ptr [rcx], eax
140af5dee: mov rcx, qword ptr [rsp + 0x60]
140af5df3: call 0x1409f1ab0
140af5df8: mov eax, dword ptr [rsp + 0x4c]
140af5dfc: mov rcx, qword ptr [rsp + 0xa0]
140af5e04: mov dword ptr [rcx], eax
140af5e06: mov r12, qword ptr [rdi + 0x30]
140af5e0a: mov rsi, qword ptr [rdi + 0x40]
140af5e0e: cmp r12, qword ptr [rdi + 0x20]
140af5e12: jne 0x140af5e28
140af5e14: mov rcx, qword ptr [rsp + 0x88]
140af5e1c: lea rdx, [rip + 0x2144325]
140af5e23: call 0x140562870
140af5e28: mov rax, qword ptr [rdi + 0x28]
140af5e2c: mov qword ptr [rax + r12*8], rsi
140af5e30: inc r12
140af5e33: mov qword ptr [rdi + 0x30], r12
140af5e37: inc qword ptr [rdi + 0x40]
140af5e3b: mov rax, qword ptr [rsp + 0x38]
140af5e40: mov ecx, dword ptr [rsp + 0x30]
140af5e44: mov dword ptr [rax], ecx
140af5e46: cmp qword ptr [rbx + 0x18], 0
140af5e4b: je 0x140af5e55
140af5e4d: mov qword ptr [rbx + 0x18], 0
140af5e55: mov qword ptr [rbx + 0x10], 0
140af5e5d: movzx r12d, byte ptr [r13 + 0xb]
140af5e62: mov rax, rbp
140af5e65: mov r13, r15
140af5e68: cmp r12b, 3
140af5e6c: je 0x140af5ca5
140af5e72: mov eax, dword ptr [rsp + 0x48]
140af5e76: mov rcx, qword ptr [rsp + 0x90]
140af5e7e: mov dword ptr [rcx], eax
140af5e80: mov r13, qword ptr [r14 + 0x30]
140af5e84: mov rsi, qword ptr [r14 + 0x40]
140af5e88: cmp r13, qword ptr [r14 + 0x20]
140af5e8c: jne 0x140af5c80
140af5e92: mov rcx, qword ptr [rsp + 0x78]
140af5e97: lea rdx, [rip + 0x21442aa]
140af5e9e: call 0x140562630
140af5ea3: jmp 0x140af5c80
140af5ea8: mov rax, qword ptr [rip + 0x2b364f9]
140af5eaf: mov rax, qword ptr [rax]
140af5eb2: cmp rax, 2
140af5eb6: jb 0x140af5f9c
140af5ebc: lea rcx, [rip + 0x1fef3dd]
140af5ec3: call 0x14067dd80
140af5ec8: movups xmm0, xmmword ptr [rax]
140af5ecb: mov eax, dword ptr [rax + 0x10]
140af5ece: mov qword ptr [rsp + 0xe8], 2
140af5eda: lea rcx, [rip + 0x1fef3d7]
140af5ee1: mov qword ptr [rsp + 0xf0], rcx
140af5ee9: mov qword ptr [rsp + 0xf8], 0x2f
140af5ef5: lea rdx, [rip + 0x1fef394]
140af5efc: mov qword ptr [rsp + 0x108], rdx
140af5f04: mov qword ptr [rsp + 0x110], 1
140af5f10: mov qword ptr [rsp + 0x118], 8
140af5f1c: mov qword ptr [rsp + 0xb8], 0
140af5f28: xorps xmm1, xmm1
140af5f2b: movups xmmword ptr [rsp + 0x120], xmm1
140af5f33: mov qword ptr [rsp + 0xc0], rcx
140af5f3b: mov qword ptr [rsp + 0xc8], 0x2f
140af5f47: mov qword ptr [rsp + 0xd0], 0
140af5f53: movups xmmword ptr [rsp + 0xd8], xmm0
140af5f5b: mov dword ptr [rsp + 0x100], 1
140af5f66: mov dword ptr [rsp + 0x104], eax
140af5f6d: lea rcx, [rsp + 0x37]
140af5f72: lea rdx, [rsp + 0xb8]
140af5f7a: call 0x140d4a330
140af5f7f: jmp 0x140af5f9c
140af5f81: mov rax, qword ptr [rsp + 0x60]
140af5f86: cmp qword ptr [rax + 0x18], 0
140af5f8b: jne 0x140af5f9c
140af5f8d: mov rax, qword ptr [rsp + 0x70]
140af5f92: mov rax, qword ptr [rax]
140af5f95: cmp qword ptr [rax + 0x10], 0
140af5f9a: je 0x140af5fb0
140af5f9c: add rsp, 0x138
140af5fa3: pop rbx
140af5fa4: pop rbp
140af5fa5: pop rdi
140af5fa6: pop rsi
140af5fa7: pop r12
140af5fa9: pop r13
140af5fab: pop r14
140af5fad: pop r15
140af5faf: ret 
140af5fb0: mov rax, qword ptr [rsp + 0x38]
140af5fb5: mov ecx, dword ptr [rsp + 0x30]
140af5fb9: mov dword ptr [rax], ecx
140af5fbb: mov rdx, qword ptr [rbx + 0x18]
140af5fbf: test rdx, rdx
140af5fc2: je 0x140af5f9c
140af5fc4: mov rax, qword ptr [rbx + 0x10]
140af5fc8: lea r8, [rax + 1]
140af5fcc: mov r9, qword ptr [rbx]
140af5fcf: mov rcx, qword ptr [rbx + 8]
140af5fd3: xor r10d, r10d
140af5fd6: cmp r8, r9
140af5fd9: cmovae r10, r9
140af5fdd: neg r10
140af5fe0: lea r8, [rax + r10]
140af5fe4: inc r8
140af5fe7: mov qword ptr [rbx + 0x10], r8
140af5feb: dec rdx
140af5fee: mov qword ptr [rbx + 0x18], rdx
140af5ff2: movzx esi, word ptr [rcx + rax*4 + 1]
140af5ff7: cmp byte ptr [rcx + rax*4], 2
140af5ffb: sete dl
140af5ffe: cmp sil, 4
140af6002: sete r8b
140af6006: or r8b, dl
140af6009: jne 0x140af5f9c
140af600b: movzx eax, byte ptr [rcx + rax*4 + 3]
140af6010: shl eax, 0x10
140af6013: mov ebx, esi
140af6015: or ebx, eax
140af6017: mov rdx, qword ptr [rsp + 0x68]
140af601c: mov eax, dword ptr [rdx + 0x1c]
140af601f: mov rcx, qword ptr [rdx + 0x10]
140af6023: mov dword ptr [rcx], eax
140af6025: mov rdi, qword ptr [rdx]
140af6028: mov r14, qword ptr [rdi + 0x30]
140af602c: mov r15, qword ptr [rdi + 0x40]
140af6030: cmp r14, qword ptr [rdi + 0x20]
140af6034: jne 0x140af6046
140af6036: lea rcx, [rdi + 0x20]
140af603a: lea rdx, [rip + 0x2193037]
140af6041: call 0x140562870
140af6046: shr ebx, 8
140af6049: mov rax, qword ptr [rdi + 0x28]
140af604d: mov qword ptr [rax + r14*8], r15
140af6051: inc r14
140af6054: mov qword ptr [rdi + 0x30], r14
140af6058: inc qword ptr [rdi + 0x40]
140af605c: test sil, sil
140af605f: je 0x140af60c1
140af6061: movzx eax, sil
140af6065: cmp eax, 1
140af6068: jne 0x140af6151
140af606e: mov rcx, qword ptr [rsp + 0x58]
140af6073: mov eax, dword ptr [rcx + 0x1c]
140af6076: mov rcx, qword ptr [rcx + 0x10]
140af607a: mov dword ptr [rcx], eax
140af607c: mov rax, qword ptr [rsp + 0x28]
140af6081: mov eax, dword ptr [rax + 0x68]
140af6084: cmp eax, -2
140af6087: jle 0x140af6139
140af608d: inc eax
140af608f: js 0x140af5f9c
140af6095: mov ecx, eax
140af6097: mov rdx, qword ptr [rsp + 0x28]
140af609c: cmp qword ptr [rdx + 0x10], rcx
140af60a0: jbe 0x140af5f9c
140af60a6: lea rcx, [rcx + rcx*4]
140af60aa: shl rcx, 3
140af60ae: mov rdx, qword ptr [rsp + 0x28]
140af60b3: add rcx, qword ptr [rdx + 8]
140af60b7: mov dword ptr [rdx + 0x68], eax
140af60ba: and bl, 1
140af60bd: xor edx, edx
140af60bf: jmp 0x140af6119
140af60c1: mov rcx, qword ptr [rsp + 0x58]
140af60c6: mov eax, dword ptr [rcx + 0x1c]
140af60c9: mov rcx, qword ptr [rcx + 0x10]
140af60cd: mov dword ptr [rcx], eax
140af60cf: mov rax, qword ptr [rsp + 0x28]
140af60d4: movsxd rax, dword ptr [rax + 0x68]
140af60d8: test rax, rax
140af60db: js 0x140af5f9c
140af60e1: mov rcx, qword ptr [rsp + 0x28]
140af60e6: cmp eax, dword ptr [rcx + 0x6c]
140af60e9: jle 0x140af5f9c
140af60ef: mov rcx, qword ptr [rsp + 0x28]
140af60f4: cmp qword ptr [rcx + 0x10], rax
140af60f8: jbe 0x140af5f9c
140af60fe: lea rcx, [rax + rax*4]
140af6102: shl rcx, 3
140af6106: mov rdx, qword ptr [rsp + 0x28]
140af610b: add rcx, qword ptr [rdx + 8]
140af610f: dec eax
140af6111: mov dword ptr [rdx + 0x68], eax
140af6114: and bl, 1
140af6117: mov dl, 1
140af6119: mov r8d, ebx
140af611c: mov r9, qword ptr [rsp + 0x50]
140af6121: add rsp, 0x138
140af6128: pop rbx
140af6129: pop rbp
140af612a: pop rdi
140af612b: pop rsi
140af612c: pop r12
140af612e: pop r13
140af6130: pop r14
140af6132: pop r15
140af6134: jmp 0x140af6170
140af6139: lea rcx, [rip + 0x1ff7540]
140af6140: lea r8, [rip + 0x1ff7559]
140af6147: mov edx, 0x1e
140af614c: call 0x1428d9430
140af6151: lea rcx, [rip + 0x1fef09a]
140af6158: lea r8, [rip + 0x1fef0c1]
140af615f: mov edx, 0x28
140af6164: call 0x1428d9430
140af6169: int3 
