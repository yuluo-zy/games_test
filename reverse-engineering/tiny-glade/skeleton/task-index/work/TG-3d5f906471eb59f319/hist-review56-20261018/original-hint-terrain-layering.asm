142283cc0: push rbp
142283cc1: push r15
142283cc3: push r14
142283cc5: push r13
142283cc7: push r12
142283cc9: push rsi
142283cca: push rdi
142283ccb: push rbx
142283ccc: sub rsp, 0x128
142283cd3: lea rbp, [rsp + 0x80]
142283cdb: mov qword ptr [rbp + 0xa0], 0xfffffffffffffffe
142283ce6: mov qword ptr [rbp + 0x90], rdx
142283ced: mov qword ptr [rbp + 0x88], rcx
142283cf4: mov r15, qword ptr [r9]
142283cf7: mov rdx, qword ptr [r9 + 8]
142283cfb: mov r11, qword ptr [r15]
142283cfe: xor r10d, r10d
142283d01: mov rcx, r11
142283d04: sub rcx, qword ptr [rdx + 0x18]
142283d08: cmovb rcx, r10
142283d0c: sub r11, qword ptr [rdx + 0x38]
142283d10: cmovb r11, r10
142283d14: mov rax, rcx
142283d17: shl rax, 4
142283d1b: add rax, qword ptr [rdx + 8]
142283d1f: mov rdi, qword ptr [rdx + 0x10]
142283d23: sub rdi, rcx
142283d26: cmovb rdi, r10
142283d2a: mov r9, qword ptr [rbp + 0x110]
142283d31: mov esi, 8
142283d36: cmovb rax, rsi
142283d3a: mov rcx, r11
142283d3d: shl rcx, 4
142283d41: add rcx, qword ptr [rdx + 0x28]
142283d45: mov rbx, qword ptr [rdx + 0x30]
142283d49: sub rbx, r11
142283d4c: mov r11, rbx
142283d4f: cmovb r11, r10
142283d53: cmovb rcx, rsi
142283d57: mov rdx, qword ptr [rdx + 0x40]
142283d5b: sub rdx, r11
142283d5e: sub rdx, rdi
142283d61: mov r10, rdi
142283d64: mov qword ptr [r15], rdx
142283d67: shl r10, 4
142283d6b: add r10, rax
142283d6e: shl r11, 4
142283d72: add r11, rcx
142283d75: mov r12, qword ptr [r8]
142283d78: mov rdx, qword ptr [r8 + 0x10]
142283d7c: mov esi, dword ptr [r8 + 0x1c]
142283d80: mov r8, rdx
142283d83: mov edx, dword ptr [r9 + 0x1c]
142283d87: mov dword ptr [rbp + 0x98], edx
142283d8d: mov rbx, qword ptr [r9]
142283d90: mov rdx, qword ptr [r9 + 0x10]
142283d94: mov qword ptr [rbp + 0x60], rdx
142283d98: mov r9d, esi
142283d9b: lea rdx, [rbx + 0x20]
142283d9f: mov qword ptr [rbp + 0x80], rdx
142283da6: mov qword ptr [rbp + 0x68], r15
142283daa: mov qword ptr [rbp + 0x70], r10
142283dae: mov qword ptr [rbp + 0x78], r8
142283db2: mov dword ptr [rbp + 0x9c], esi
142283db8: jmp 0x142283de8
142283dba: mov rax, qword ptr [rbx + 0x28]
142283dbe: mov rcx, r14
142283dc1: shl rcx, 4
142283dc5: mov qword ptr [rax + rcx], r13
142283dc9: mov byte ptr [rax + rcx + 8], 6
142283dce: inc r14
142283dd1: mov qword ptr [rbx + 0x30], r14
142283dd5: inc qword ptr [rbx + 0x40]
142283dd9: mov dword ptr [r8], r9d
142283ddc: mov byte ptr [r12 + 8], 1
142283de2: mov rcx, rsi
142283de5: mov rax, rdi
142283de8: test rax, rax
142283deb: je 0x142283e70
142283df1: cmp rax, r10
142283df4: je 0x142283e70
142283df6: lea rdi, [rax + 0x10]
142283dfa: mov rsi, rcx
142283dfd: inc qword ptr [r15]
142283e00: movzx edx, byte ptr [rax + 8]
142283e04: test edx, edx
142283e06: je 0x142283e92
142283e0c: mov rcx, rsi
142283e0f: mov rax, rdi
142283e12: cmp edx, 1
142283e15: jne 0x142283de8
142283e17: cmp byte ptr [r12 + 8], 0
142283e1d: jne 0x142283dd9
142283e1f: mov eax, dword ptr [rbp + 0x98]
142283e25: mov rcx, qword ptr [rbp + 0x60]
142283e29: mov dword ptr [rcx], eax
142283e2b: mov r14, qword ptr [rbx + 0x30]
142283e2f: mov r13, qword ptr [rbx + 0x40]
142283e33: cmp r14, qword ptr [rbx + 0x20]
142283e37: jne 0x142283dba
142283e39: mov rcx, qword ptr [rbp + 0x80]
142283e40: lea rdx, [rip + 0xdb4689]
142283e47: mov r15, r11
142283e4a: call 0x140562630
142283e4f: mov r9d, dword ptr [rbp + 0x9c]
142283e56: mov r8, qword ptr [rbp + 0x78]
142283e5a: mov r11, r15
142283e5d: mov r10, qword ptr [rbp + 0x70]
142283e61: mov r15, qword ptr [rbp + 0x68]
142283e65: jmp 0x142283dba
142283e6a: nop word ptr [rax + rax]
142283e70: test rcx, rcx
142283e73: je 0x142283e9e
142283e75: cmp rcx, r11
142283e78: je 0x142283e9e
142283e7a: lea rsi, [rcx + 0x10]
142283e7e: xor edi, edi
142283e80: mov rax, rcx
142283e83: inc qword ptr [r15]
142283e86: movzx edx, byte ptr [rax + 8]
142283e8a: test edx, edx
142283e8c: jne 0x142283e0c
142283e92: mov dword ptr [r8], r9d
142283e95: inc qword ptr [r12]
142283e99: jmp 0x142283de2
142283e9e: mov rax, qword ptr [rbp + 0x88]
142283ea5: mov rax, qword ptr [rax]
142283ea8: cmp dword ptr [rax], 0xa
142283eab: jne 0x142283ffe
142283eb1: mov rax, qword ptr [rbp + 0x90]
142283eb8: mov rsi, qword ptr [rax]
142283ebb: cmp byte ptr [rsi + 0x1f9], 0
142283ec2: jne 0x142283ffe
142283ec8: cmp qword ptr [r12], 2
142283ecd: jb 0x142283ffe
142283ed3: mov rax, qword ptr [rbp + 0x120]
142283eda: mov rax, qword ptr [rax]
142283edd: cmp qword ptr [rax], 0
142283ee1: je 0x142283ffe
142283ee7: cmp byte ptr [r12 + 8], 0
142283eed: jne 0x142283ffe
142283ef3: mov cl, 0xf
142283ef5: call 0x1409e8d90
142283efa: mov word ptr [rbp - 0xc], ax
142283efe: mov dword ptr [rbp - 0x10], 0xc9
142283f05: mov eax, dword ptr [rsi + 0x128]
142283f0b: mov dword ptr [rbp + 0x58], eax
142283f0e: mov rax, qword ptr [rsi + 0x120]
142283f15: mov qword ptr [rbp + 0x50], rax
142283f19: lea rcx, [rbp - 0x48]
142283f1d: lea rdx, [rbp - 0x10]
142283f21: lea r8, [rbp + 0x50]
142283f25: call 0x141c4e4f0
142283f2a: mov word ptr [rbp - 0x18], 0x600
142283f30: mov dword ptr [rbp - 0x60], 6
142283f37: mov rdx, qword ptr [rbp + 0x118]
142283f3e: mov eax, dword ptr [rdx + 0x1c]
142283f41: mov rcx, qword ptr [rdx + 0x10]
142283f45: mov dword ptr [rcx], eax
142283f47: mov rsi, qword ptr [rdx]
142283f4a: mov rax, qword ptr [rsi + 0x40]
142283f4e: mov qword ptr [rbp + 0x40], rax
142283f52: mov rax, qword ptr [rbp - 0x20]
142283f56: mov qword ptr [rbp + 0x30], rax
142283f5a: movzx eax, word ptr [rbp - 0x18]
142283f5e: mov word ptr [rbp + 0x38], ax
142283f62: mov eax, dword ptr [rbp - 0x16]
142283f65: mov dword ptr [rbp + 0x3a], eax
142283f68: movzx eax, word ptr [rbp - 0x12]
142283f6c: mov word ptr [rbp + 0x3e], ax
142283f70: mov eax, dword ptr [rbp - 0x60]
142283f73: mov rcx, qword ptr [rbp - 0x5c]
142283f77: mov edx, dword ptr [rbp - 0x54]
142283f7a: movups xmm0, xmmword ptr [rbp - 0x50]
142283f7e: movups xmm1, xmmword ptr [rbp - 0x40]
142283f82: movups xmm2, xmmword ptr [rbp - 0x30]
142283f86: movaps xmmword ptr [rbp + 0x20], xmm2
142283f8a: movaps xmmword ptr [rbp + 0x10], xmm1
142283f8e: movaps xmmword ptr [rbp], xmm0
142283f92: mov dword ptr [rbp - 0x10], eax
142283f95: mov qword ptr [rbp - 0xc], rcx
142283f99: mov dword ptr [rbp - 4], edx
142283f9c: mov rdi, qword ptr [rsi + 0x30]
142283fa0: cmp rdi, qword ptr [rsi + 0x20]
142283fa4: jne 0x142283fb6
142283fa6: lea rcx, [rsi + 0x20]
142283faa: lea rdx, [rip + 0xdb451f]
142283fb1: call 0x140562f30
142283fb6: mov rax, qword ptr [rsi + 0x28]
142283fba: imul rcx, rdi, 0x58
142283fbe: movaps xmm0, xmmword ptr [rbp - 0x10]
142283fc2: movaps xmm1, xmmword ptr [rbp]
142283fc6: movaps xmm2, xmmword ptr [rbp + 0x10]
142283fca: movaps xmm3, xmmword ptr [rbp + 0x20]
142283fce: movaps xmm4, xmmword ptr [rbp + 0x30]
142283fd2: mov rdx, qword ptr [rbp + 0x40]
142283fd6: mov qword ptr [rax + rcx + 0x50], rdx
142283fdb: movups xmmword ptr [rax + rcx + 0x40], xmm4
142283fe0: movups xmmword ptr [rax + rcx + 0x30], xmm3
142283fe5: movups xmmword ptr [rax + rcx + 0x20], xmm2
142283fea: movups xmmword ptr [rax + rcx + 0x10], xmm1
142283fef: movups xmmword ptr [rax + rcx], xmm0
142283ff3: inc rdi
142283ff6: mov qword ptr [rsi + 0x30], rdi
142283ffa: inc qword ptr [rsi + 0x40]
142283ffe: add rsp, 0x128
142284005: pop rbx
142284006: pop rdi
142284007: pop rsi
142284008: pop r12
14228400a: pop r13
14228400c: pop r14
14228400e: pop r15
142284010: pop rbp
142284011: ret 
142284012: nop word ptr cs:[rax + rax]
142284020: mov qword ptr [rsp + 0x10], rdx
142284025: push rbp
142284026: push r15
142284028: push r14
14228402a: push r13
14228402c: push r12
14228402e: push rsi
14228402f: push rdi
142284030: push rbx
142284031: sub rsp, 0x28
142284035: lea rbp, [rdx + 0x80]
14228403c: lea rcx, [rbp + 8]
142284040: call 0x140eaaa50
142284045: nop 
142284046: add rsp, 0x28
14228404a: pop rbx
14228404b: pop rdi
14228404c: pop rsi
14228404d: pop r12
14228404f: pop r13
142284051: pop r14
142284053: pop r15
142284055: pop rbp
142284056: ret 
