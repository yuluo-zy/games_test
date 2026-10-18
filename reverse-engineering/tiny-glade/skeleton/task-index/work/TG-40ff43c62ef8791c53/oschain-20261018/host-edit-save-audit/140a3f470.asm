140a3f470: push rbp
140a3f471: push r15
140a3f473: push r14
140a3f475: push r13
140a3f477: push r12
140a3f479: push rsi
140a3f47a: push rdi
140a3f47b: push rbx
140a3f47c: sub rsp, 0x448
140a3f483: lea rbp, [rsp + 0x80]
140a3f48b: movaps xmmword ptr [rbp + 0x3b0], xmm11
140a3f493: movaps xmmword ptr [rbp + 0x3a0], xmm10
140a3f49b: movaps xmmword ptr [rbp + 0x390], xmm9
140a3f4a3: movaps xmmword ptr [rbp + 0x380], xmm8
140a3f4ab: movaps xmmword ptr [rbp + 0x370], xmm7
140a3f4b2: movaps xmmword ptr [rbp + 0x360], xmm6
140a3f4b9: mov qword ptr [rbp + 0x358], 0xfffffffffffffffe
140a3f4c4: mov qword ptr [rbp + 0x2c8], r9
140a3f4cb: mov qword ptr [rbp + 0x2a8], r8
140a3f4d2: mov r15, rdx
140a3f4d5: mov rax, qword ptr [rcx]
140a3f4d8: mov qword ptr [rbp + 0x338], rax
140a3f4df: mov rax, qword ptr [rcx + 0x10]
140a3f4e3: mov qword ptr [rbp + 0x2c0], rax
140a3f4ea: mov eax, dword ptr [rcx + 0x1c]
140a3f4ed: mov dword ptr [rbp + 0x2d8], eax
140a3f4f3: mov rdi, qword ptr [rcx + 0x20]
140a3f4f7: mov rbx, qword ptr [rcx + 0x30]
140a3f4fb: mov esi, dword ptr [rcx + 0x3c]
140a3f4fe: mov rax, qword ptr [rcx + 0x40]
140a3f502: mov qword ptr [rbp + 0x10], rax
140a3f506: mov rax, qword ptr [rcx + 0x60]
140a3f50a: mov qword ptr [rbp + 0x290], rax
140a3f511: mov rax, qword ptr [rcx + 0x1a8]
140a3f518: mov qword ptr [rbp - 0x30], rax
140a3f51c: mov rax, qword ptr [rcx + 0x68]
140a3f520: mov qword ptr [rbp], rax
140a3f524: movups xmm0, xmmword ptr [rcx + 0x198]
140a3f52b: movaps xmmword ptr [rbp - 0x40], xmm0
140a3f52f: mov rax, qword ptr [rcx + 0x78]
140a3f533: mov qword ptr [rbp + 0x2a0], rax
140a3f53a: mov r14, qword ptr [rcx + 0x98]
140a3f541: mov rax, qword ptr [rcx + 0xb8]
140a3f548: mov qword ptr [rbp + 0xb8], rax
140a3f54f: mov rax, qword ptr [rcx + 0xd8]
140a3f556: mov qword ptr [rbp + 0xc0], rax
140a3f55d: mov rax, qword ptr [rcx + 0xf8]
140a3f564: mov qword ptr [rbp + 0x298], rax
140a3f56b: mov rax, qword ptr [rcx + 0x108]
140a3f572: mov qword ptr [rbp + 0x58], rax
140a3f576: mov eax, dword ptr [rcx + 0x114]
140a3f57c: mov dword ptr [rbp + 0xfc], eax
140a3f582: mov rax, qword ptr [rcx + 0x118]
140a3f589: mov qword ptr [rbp + 0xe8], rax
140a3f590: mov rax, qword ptr [rcx + 0x138]
140a3f597: mov qword ptr [rbp + 0xd0], rax
140a3f59e: mov rax, qword ptr [rcx + 0x158]
140a3f5a5: mov qword ptr [rbp + 0xd8], rax
140a3f5ac: mov rax, qword ptr [rcx + 0x178]
140a3f5b3: mov qword ptr [rbp + 0xe0], rax
140a3f5ba: mov r13, qword ptr [rcx + 0x1b0]
140a3f5c1: mov rax, qword ptr [rcx + 0x1d0]
140a3f5c8: mov qword ptr [rbp + 0x20], rax
140a3f5cc: mov r12, qword ptr [rdx + 0xc0]
140a3f5d3: mov rax, qword ptr [rdx + 0xe0]
140a3f5da: mov qword ptr [rbp + 0x2b8], rax
140a3f5e1: mov rax, qword ptr [rdx + 0x100]
140a3f5e8: mov qword ptr [rbp + 0x60], rax
140a3f5ec: mov rax, qword ptr [rdx + 0x120]
140a3f5f3: mov qword ptr [rbp + 0x30], rax
140a3f5f7: mov rax, qword ptr [rdx + 0x140]
140a3f5fe: mov qword ptr [rbp + 0xc8], rax
140a3f605: mov rax, qword ptr [rdx + 0x160]
140a3f60c: mov qword ptr [rbp + 0x2b0], rax
140a3f613: mov rax, qword ptr [rdx + 0x168]
140a3f61a: mov qword ptr [rbp + 0xb0], rax
140a3f621: call 0x140c16a00
140a3f626: test al, al
140a3f628: mov qword ptr [rbp + 0x348], rdi
140a3f62f: je 0x140a3f69a
140a3f631: mov dword ptr [rbp + 0x2d0], esi
140a3f637: mov qword ptr [rbp + 0x328], rbx
140a3f63e: mov ebx, 0x31
140a3f643: mov ecx, 0x31
140a3f648: call 0x140a3e840
140a3f64d: lea rsi, [rip + 0x208e6f5]
140a3f654: cmp rax, 1
140a3f658: jne 0x140a3f705
140a3f65e: mov r9, rdx
140a3f661: test rdx, rdx
140a3f664: je 0x140a3f6a3
140a3f666: cmp r9, 0x31
140a3f66a: jae 0x140a3f6a1
140a3f66c: lea rax, [rip + 0x208e6d6]
140a3f673: cmp byte ptr [r9 + rax], 0xbf
140a3f678: jg 0x140a3f6a3
140a3f67a: lea rax, [rip + 0x208e77f]
140a3f681: mov qword ptr [rsp + 0x20], rax
140a3f686: lea rcx, [rip + 0x208e6bc]
140a3f68d: mov edx, 0x31
140a3f692: xor r8d, r8d
140a3f695: call 0x1428d9940
140a3f69a: xor eax, eax
140a3f69c: jmp 0x140a3f879
140a3f6a1: jne 0x140a3f67a
140a3f6a3: mov rcx, r9
140a3f6a6: call 0x140a3e840
140a3f6ab: cmp rax, 1
140a3f6af: jne 0x140a3f705
140a3f6b1: mov r8, rdx
140a3f6b4: add r8, 2
140a3f6b8: je 0x140a3f6f3
140a3f6ba: cmp r8, 0x31
140a3f6be: jae 0x140a3f6f1
140a3f6c0: lea rax, [rip + 0x208e682]
140a3f6c7: cmp byte ptr [r8 + rax], 0xbf
140a3f6cc: jg 0x140a3f6f3
140a3f6ce: lea rax, [rip + 0x208e743]
140a3f6d5: mov qword ptr [rsp + 0x20], rax
140a3f6da: lea rcx, [rip + 0x208e668]
140a3f6e1: mov edx, 0x31
140a3f6e6: mov r9d, 0x31
140a3f6ec: call 0x1428d9940
140a3f6f1: jne 0x140a3f6ce
140a3f6f3: mov ebx, 0x2f
140a3f6f8: sub rbx, rdx
140a3f6fb: lea rsi, [rip + 0x208e647]
140a3f702: add rsi, r8
140a3f705: mov qword ptr [rbp + 0xf0], r12
140a3f70c: lea r8, [rip + 0x208e819]
140a3f713: lea r12, [rip + 0x208e7e1]
140a3f71a: nop word ptr [rax + rax]
140a3f720: movsx eax, byte ptr [r8 - 1]
140a3f725: test eax, eax
140a3f727: js 0x140a3f740
140a3f729: dec r8
140a3f72c: cmp eax, 0x5c
140a3f72f: jne 0x140a3f794
140a3f731: jmp 0x140a3f7a5
140a3f733: nop word ptr cs:[rax + rax]
140a3f740: movzx ecx, byte ptr [r8 - 2]
140a3f745: cmp cl, 0xc0
140a3f748: jge 0x140a3f76d
140a3f74a: movzx edx, byte ptr [r8 - 3]
140a3f74f: cmp dl, 0xc0
140a3f752: jge 0x140a3f776
140a3f754: movzx r9d, byte ptr [r8 - 4]
140a3f759: add r8, -4
140a3f75d: and r9d, 7
140a3f761: shl r9d, 6
140a3f765: and edx, 0x3f
140a3f768: or edx, r9d
140a3f76b: jmp 0x140a3f77d
140a3f76d: add r8, -2
140a3f771: and ecx, 0x1f
140a3f774: jmp 0x140a3f785
140a3f776: add r8, -3
140a3f77a: and edx, 0xf
140a3f77d: shl edx, 6
140a3f780: and ecx, 0x3f
140a3f783: or ecx, edx
140a3f785: shl ecx, 6
140a3f788: and al, 0x3f
140a3f78a: movzx eax, al
140a3f78d: or eax, ecx
140a3f78f: cmp eax, 0x5c
140a3f792: je 0x140a3f7a5
140a3f794: cmp eax, 0x2f
140a3f797: je 0x140a3f7a5
140a3f799: cmp r8, r12
140a3f79c: jne 0x140a3f720
140a3f79e: mov edi, 0x31
140a3f7a3: jmp 0x140a3f7f1
140a3f7a5: lea r12, [rip + 0x208e74f]
140a3f7ac: sub r8, r12
140a3f7af: inc r8
140a3f7b2: je 0x140a3f7e6
140a3f7b4: cmp r8, 0x31
140a3f7b8: jae 0x140a3f7e4
140a3f7ba: cmp byte ptr [r8 + r12], 0xbf
140a3f7bf: jg 0x140a3f7e6
140a3f7c1: lea rax, [rip + 0x208e618]
140a3f7c8: mov qword ptr [rsp + 0x20], rax
140a3f7cd: lea rcx, [rip + 0x208e727]
140a3f7d4: mov edx, 0x31
140a3f7d9: mov r9d, 0x31
140a3f7df: call 0x1428d9940
140a3f7e4: jne 0x140a3f7c1
140a3f7e6: mov edi, 0x31
140a3f7eb: sub rdi, r8
140a3f7ee: add r12, r8
140a3f7f1: call 0x140c18130
140a3f7f6: mov rcx, rax
140a3f7f9: mov rax, qword ptr [rax]
140a3f7fc: cmp rax, 1
140a3f800: jne 0x140a40fea
140a3f806: add rcx, 8
140a3f80a: cmp qword ptr [rcx], 0
140a3f80e: jne 0x140a41e0b
140a3f814: mov qword ptr [rcx], 0xffffffffffffffff
140a3f81b: mov qword ptr [rbp + 0x140], rcx
140a3f822: add rcx, 8
140a3f826: mov qword ptr [rsp + 0x20], rdi
140a3f82b: mov qword ptr [rsp + 0x30], 0
140a3f834: mov qword ptr [rsp + 0x28], 1
140a3f83d: mov rdx, rsi
140a3f840: mov r8, rbx
140a3f843: mov r9, r12
140a3f846: call 0x140c16d40
140a3f84b: mov rcx, qword ptr [rbp + 0x140]
140a3f852: inc qword ptr [rcx]
140a3f855: mov qword ptr [rbp + 0x50], rax
140a3f859: mov eax, 1
140a3f85e: mov rdi, qword ptr [rbp + 0x348]
140a3f865: mov r12, qword ptr [rbp + 0xf0]
140a3f86c: mov rbx, qword ptr [rbp + 0x328]
140a3f873: mov esi, dword ptr [rbp + 0x2d0]
140a3f879: mov qword ptr [rbp + 0x48], rax
140a3f87d: mov dword ptr [rbx], esi
140a3f87f: movss xmm0, dword ptr [r12 + 0x28]
140a3f886: addss xmm0, dword ptr [rdi + 0x148]
140a3f88e: movss dword ptr [rdi + 0x148], xmm0
140a3f896: lea rsi, [rdi + 0x128]
140a3f89d: mov cl, 1
140a3f89f: xor eax, eax
140a3f8a1: lock cmpxchg byte ptr [rdi + 0x128], cl
140a3f8a9: jne 0x140a410df
140a3f8af: mov rcx, rsi
140a3f8b2: call 0x140c81c70
140a3f8b7: mov rax, qword ptr [rdi + 0x130]
140a3f8be: mov rdx, qword ptr [rdi + 0x138]
140a3f8c5: cmp rax, 2
140a3f8c9: mov qword ptr [rbp + 0x328], rsi
140a3f8d0: je 0x140a3f8e2
140a3f8d2: cmp eax, 1
140a3f8d5: jne 0x140a3f8ed
140a3f8d7: lea rcx, [rbp - 0x18]
140a3f8db: call 0x141677c00
140a3f8e0: jmp 0x140a3f8f6
140a3f8e2: lea rcx, [rbp - 0x18]
140a3f8e6: call 0x1416d5fc0
140a3f8eb: jmp 0x140a3f8f6
140a3f8ed: lea rcx, [rbp - 0x18]
140a3f8f1: call 0x14167cb80
140a3f8f6: xor eax, eax
140a3f8f8: cmp rax, qword ptr [rbp - 0x18]
140a3f8fc: jno 0x140a3f92a
140a3f8fe: mov rsi, qword ptr [rbp + 0x328]
140a3f905: mov rcx, rsi
140a3f908: call 0x140c81e10
140a3f90d: xor ecx, ecx
140a3f90f: mov al, 1
140a3f911: lock cmpxchg byte ptr [rsi], cl
140a3f915: je 0x140a3f9fb
140a3f91b: mov rcx, rsi
140a3f91e: xor edx, edx
140a3f920: call 0x1428cdb90
140a3f925: jmp 0x140a3f9fb
140a3f92a: mov qword ptr [rbp + 0x2d0], r13
140a3f931: mov r13, r15
140a3f934: mov r15, r14
140a3f937: mov r14, r12
140a3f93a: lea rbx, [rdi + 0x10]
140a3f93e: mov rsi, qword ptr [rdi + 0x18]
140a3f942: mov rdi, qword ptr [rdi + 0x20]
140a3f946: test rdi, rdi
140a3f949: je 0x140a3f99e
140a3f94b: lea r12, [rsi + 0x30]
140a3f94f: jmp 0x140a3f969
140a3f951: nop word ptr cs:[rax + rax]
140a3f960: add r12, 0x40
140a3f964: dec rdi
140a3f967: je 0x140a3f99e
140a3f969: mov rdx, qword ptr [r12 - 0x30]
140a3f96e: test rdx, rdx
140a3f971: je 0x140a3f983
140a3f973: mov rcx, qword ptr [r12 - 0x28]
140a3f978: mov r8d, 1
140a3f97e: call 0x140613c20
140a3f983: mov rdx, qword ptr [r12 - 8]
140a3f988: test rdx, rdx
140a3f98b: je 0x140a3f960
140a3f98d: mov rcx, qword ptr [r12]
140a3f991: mov r8d, 1
140a3f997: call 0x140613c20
140a3f99c: jmp 0x140a3f960
140a3f99e: mov rdx, qword ptr [rbx]
140a3f9a1: test rdx, rdx
140a3f9a4: je 0x140a3f9b8
140a3f9a6: shl rdx, 6
140a3f9aa: mov r8d, 8
140a3f9b0: mov rcx, rsi
140a3f9b3: call 0x140613c20
140a3f9b8: mov rax, qword ptr [rbp - 8]
140a3f9bc: mov qword ptr [rbx + 0x10], rax
140a3f9c0: movups xmm0, xmmword ptr [rbp - 0x18]
140a3f9c4: movups xmmword ptr [rbx], xmm0
140a3f9c7: mov rsi, qword ptr [rbp + 0x328]
140a3f9ce: mov rcx, rsi
140a3f9d1: call 0x140c81e10
140a3f9d6: mov rdi, qword ptr [rbp + 0x348]
140a3f9dd: mov r12, r14
140a3f9e0: mov r14, r15
140a3f9e3: mov r15, r13
140a3f9e6: mov r13, qword ptr [rbp + 0x2d0]
140a3f9ed: xor ecx, ecx
140a3f9ef: mov al, 1
140a3f9f1: lock cmpxchg byte ptr [rsi], cl
140a3f9f5: jne 0x140a410f2
140a3f9fb: mov rcx, qword ptr [rbp + 0x338]
140a3fa02: mov rax, qword ptr [rcx + 0x58]
140a3fa06: cmp rax, qword ptr [rcx + 0x10]
140a3fa0a: jne 0x140a3fa19
140a3fa0c: cmp rax, qword ptr [rdi + 0x120]
140a3fa13: je 0x140a3faf9
140a3fa19: call 0x140d635e0
140a3fa1e: mov rdx, qword ptr [rbp + 0x338]
140a3fa25: mov rax, qword ptr [rdx + 8]
140a3fa29: mov rcx, qword ptr [rdx + 0x10]
140a3fa2d: lea rcx, [rcx + rcx*4]
140a3fa31: lea rcx, [rax + rcx*8]
140a3fa35: mov rdx, qword ptr [rdx + 0x58]
140a3fa39: mov qword ptr [rbp + 0x150], rax
140a3fa40: mov qword ptr [rbp + 0x158], rcx
140a3fa47: mov qword ptr [rbp + 0x160], rdx
140a3fa4e: lea r8, [rip + 0x208e243]
140a3fa55: lea rcx, [rbp + 0x88]
140a3fa5c: lea rdx, [rbp + 0x150]
140a3fa63: call 0x141302360
140a3fa68: mov rax, qword ptr [rbp + 0x338]
140a3fa6f: mov rax, qword ptr [rax + 0x58]
140a3fa73: mov rcx, qword ptr [rbp + 0x98]
140a3fa7a: mov qword ptr [rbp + 0x168], rcx
140a3fa81: movups xmm0, xmmword ptr [rbp + 0x88]
140a3fa88: movups xmmword ptr [rbp + 0x158], xmm0
140a3fa8f: mov qword ptr [rbp + 0x170], rax
140a3fa96: mov qword ptr [rbp + 0x150], 5
140a3faa1: lea rcx, [rbp + 0x2e0]
140a3faa8: lea r8, [rbp + 0x150]
140a3faaf: mov rdx, rdi
140a3fab2: call 0x1408da280
140a3fab7: cmp dword ptr [rbp + 0x2e0], 0xb
140a3fabe: jne 0x140a41e50
140a3fac4: mov dword ptr [rdi + 0x148], 0
140a3face: mov rcx, qword ptr [rbp + 0x338]
140a3fad5: mov rax, qword ptr [rcx + 0x10]
140a3fad9: mov rdx, qword ptr [rbp + 0x2c0]
140a3fae0: mov r8d, dword ptr [rbp + 0x2d8]
140a3fae7: mov dword ptr [rdx], r8d
140a3faea: mov qword ptr [rcx + 0x58], rax
140a3faee: mov rax, qword ptr [rcx + 0x10]
140a3faf2: mov qword ptr [rdi + 0x120], rax
140a3faf9: lea rcx, [rbp + 0x88]
140a3fb00: mov rdx, qword ptr [rbp + 0x2a0]
140a3fb07: call 0x140911710
140a3fb0c: mov rax, qword ptr [rbp + 0x438]
140a3fb13: mov rsi, qword ptr [rax]
140a3fb16: mov rbx, qword ptr [rax + 8]
140a3fb1a: mov r9d, dword ptr [rax + 0x10]
140a3fb1e: mov eax, dword ptr [rax + 0x14]
140a3fb21: lea rcx, [rbp + 0x170]
140a3fb28: mov dword ptr [rsp + 0x20], eax
140a3fb2c: mov rdx, rbx
140a3fb2f: mov r8, rsi
140a3fb32: call 0x141795b60
140a3fb37: lea rax, [rbx + 0xf8]
140a3fb3e: mov qword ptr [rbp + 0x150], rbx
140a3fb45: add rbx, 0x1a0
140a3fb4c: mov qword ptr [rbp + 0x158], rbx
140a3fb53: mov qword ptr [rbp + 0x160], rax
140a3fb5a: mov qword ptr [rbp + 0x168], rsi
140a3fb61: lea r8, [rip + 0x222e2c0]
140a3fb68: lea rcx, [rbp + 0x2e0]
140a3fb6f: lea rdx, [rbp + 0x150]
140a3fb76: call 0x141304bd0
140a3fb7b: mov rdx, qword ptr [rbp + 0x2e0]
140a3fb82: mov rax, qword ptr [rbp + 0x2e8]
140a3fb89: mov qword ptr [rbp + 0x2d0], rax
140a3fb90: mov rax, qword ptr [rbp + 0x2f0]
140a3fb97: mov qword ptr [rbp + 0x328], rax
140a3fb9e: mov rax, qword ptr [rbp + 0x60]
140a3fba2: cmp qword ptr [rax + 0x18], 0
140a3fba7: mov rax, qword ptr [rax + 0x20]
140a3fbab: je 0x140a3fbc2
140a3fbad: mov rcx, qword ptr [rbp + 0x2b0]
140a3fbb4: mov qword ptr [rcx], rax
140a3fbb7: xor ecx, ecx
140a3fbb9: mov rbx, qword ptr [rbp + 0x338]
140a3fbc0: jmp 0x140a3fbf1
140a3fbc2: xor ecx, ecx
140a3fbc4: mov r8, qword ptr [rbp + 0x2b0]
140a3fbcb: cmp rax, qword ptr [r8]
140a3fbce: mov qword ptr [r8], rax
140a3fbd1: jne 0x140a3fbb9
140a3fbd3: mov rax, qword ptr [rbp + 0x2b8]
140a3fbda: cmp qword ptr [rax + 0x10], 0
140a3fbdf: mov rbx, qword ptr [rbp + 0x338]
140a3fbe6: jne 0x140a3fbf1
140a3fbe8: xor eax, eax
140a3fbea: cmp rax, qword ptr [rbx + 0x30]
140a3fbee: seto cl
140a3fbf1: mov al, 1
140a3fbf3: mov dword ptr [rbp + 0x2b0], eax
140a3fbf9: movss xmm0, dword ptr [rip + 0x1f91ccb]
140a3fc01: ucomiss xmm0, dword ptr [rdi + 0x148]
140a3fc08: mov qword ptr [rbp + 0x2a0], rdx
140a3fc0f: jbe 0x140a3fd38
140a3fc15: movss xmm0, dword ptr [r12 + 0x28]
140a3fc1c: xorps xmm1, xmm1
140a3fc1f: ucomiss xmm0, xmm1
140a3fc22: jb 0x140a41101
140a3fc28: addss xmm0, dword ptr [rdi + 0x144]
140a3fc30: movss dword ptr [rdi + 0x144], xmm0
140a3fc38: test cl, cl
140a3fc3a: je 0x140a3fd38
140a3fc40: mov dword ptr [rbp + 0x2b8], 2
140a3fc4a: maxss xmm1, xmm0
140a3fc4e: movss xmm0, dword ptr [rip + 0x1ef49ae]
140a3fc56: minss xmm0, xmm1
140a3fc5a: movss dword ptr [rdi + 0x144], xmm0
140a3fc62: ucomiss xmm0, dword ptr [rip + 0x1ee5c9b]
140a3fc69: jb 0x140a3fd42
140a3fc6f: mov r12d, dword ptr [rdi + 0x140]
140a3fc76: mov edx, 0x91a2b3c5
140a3fc7b: movss xmm1, dword ptr [rip + 0x1f1a461]
140a3fc83: lea r8, [rip + 0x205f6ca]
140a3fc8a: lea r10, [rip + 0x205f6bf]
140a3fc91: lea r11, [rip + 0x205f6b4]
140a3fc98: lea rax, [rip + 0x205f6a9]
140a3fc9f: movss xmm2, dword ptr [rip + 0x1ee5c5d]
140a3fca7: nop word ptr [rax + rax]
140a3fcb0: mov r9d, r12d
140a3fcb3: mov rsi, r9
140a3fcb6: imul rsi, rdx
140a3fcba: mov rdi, rsi
140a3fcbd: shr rdi, 0x2c
140a3fcc1: imul edi, edi, 0x1c20
140a3fcc7: addss xmm0, xmm1
140a3fccb: cmp edi, r12d
140a3fcce: je 0x140a4100b
140a3fcd4: shr rsi, 0x2b
140a3fcd8: imul esi, esi, 0xe10
140a3fcde: cmp esi, r12d
140a3fce1: mov rdi, qword ptr [rbp + 0x348]
140a3fce8: je 0x140a4101a
140a3fcee: imul r9, r9, 0x1b4e81b5
140a3fcf5: mov rsi, r9
140a3fcf8: shr rsi, 0x26
140a3fcfc: imul esi, esi, 0x258
140a3fd02: cmp esi, r12d
140a3fd05: je 0x140a41025
140a3fd0b: shr r9, 0x25
140a3fd0f: imul r9d, r9d, 0x12c
140a3fd16: cmp r9d, r12d
140a3fd19: je 0x140a41030
140a3fd1f: inc r12d
140a3fd22: ucomiss xmm0, xmm2
140a3fd25: jae 0x140a3fcb0
140a3fd27: movss dword ptr [rdi + 0x144], xmm0
140a3fd2f: mov dword ptr [rdi + 0x140], r12d
140a3fd36: jmp 0x140a3fd42
140a3fd38: mov dword ptr [rbp + 0x2b8], 2
140a3fd42: mov rdi, qword ptr [rbp + 0x2c8]
140a3fd49: mov qword ptr [rbp + 0x2c8], r12
140a3fd50: mov rax, qword ptr [rbp + 0x2a8]
140a3fd57: mov r8, qword ptr [rax]
140a3fd5a: mov rdx, qword ptr [rax + 8]
140a3fd5e: mov r9, qword ptr [r8]
140a3fd61: xor eax, eax
140a3fd63: mov r10, r9
140a3fd66: sub r10, qword ptr [rdx + 0x18]
140a3fd6a: cmovb r10, rax
140a3fd6e: sub r9, qword ptr [rdx + 0x38]
140a3fd72: cmovb r9, rax
140a3fd76: mov r11, qword ptr [rdx + 0x30]
140a3fd7a: sub r11, r9
140a3fd7d: cmovb r11, rax
140a3fd81: setbe r9b
140a3fd85: mov rsi, qword ptr [rdx + 0x10]
140a3fd89: sub rsi, r10
140a3fd8c: cmovb rsi, rax
140a3fd90: mov r10, qword ptr [rdx + 0x40]
140a3fd94: setbe dl
140a3fd97: add rsi, r11
140a3fd9a: and dl, r9b
140a3fd9d: cmove rsi, rax
140a3fda1: sub r10, rsi
140a3fda4: mov qword ptr [r8], r10
140a3fda7: mov r8, qword ptr [rdi]
140a3fdaa: mov r9, qword ptr [rdi + 8]
140a3fdae: mov r10, qword ptr [r8]
140a3fdb1: mov r11, r10
140a3fdb4: sub r11, qword ptr [r9 + 0x18]
140a3fdb8: cmovb r11, rax
140a3fdbc: sub r10, qword ptr [r9 + 0x38]
140a3fdc0: cmovb r10, rax
140a3fdc4: mov rsi, qword ptr [r9 + 0x30]
140a3fdc8: sub rsi, r10
140a3fdcb: cmovb rsi, rax
140a3fdcf: setbe r10b
140a3fdd3: mov rdi, qword ptr [r9 + 0x10]
140a3fdd7: sub rdi, r11
140a3fdda: cmovb rdi, rax
140a3fdde: mov r9, qword ptr [r9 + 0x40]
140a3fde2: setbe r11b
140a3fde6: add rdi, rsi
140a3fde9: and r11b, r10b
140a3fdec: cmove rdi, rax
140a3fdf0: sub r9, rdi
140a3fdf3: mov qword ptr [r8], r9
140a3fdf6: and r11b, dl
140a3fdf9: mov byte ptr [rbp + 0x60], r11b
140a3fdfd: test byte ptr [rbp + 0x2b0], r11b
140a3fe04: mov rdi, qword ptr [rbp + 0x348]
140a3fe0b: jne 0x140a401e8
140a3fe11: xor cl, 1
140a3fe14: jne 0x140a401e8
140a3fe1a: call 0x140d635e0
140a3fe1f: mov rax, qword ptr [rbp + 0x2c0]
140a3fe26: mov ecx, dword ptr [rbp + 0x2d8]
140a3fe2c: mov dword ptr [rax], ecx
140a3fe2e: mov r9, qword ptr [rbp + 0x290]
140a3fe35: mov rax, qword ptr [r9 + 0xf8]
140a3fe3c: mov rcx, qword ptr [r9 + 0x100]
140a3fe43: lea rcx, [rax + rcx*4]
140a3fe47: movzx edx, byte ptr [r9 + 0x108]
140a3fe4f: xorps xmm0, xmm0
140a3fe52: movups xmmword ptr [rbp + 0x1a8], xmm0
140a3fe59: movups xmmword ptr [rbp + 0x198], xmm0
140a3fe60: mov r10, qword ptr [rbp]
140a3fe64: lea r8, [r10 + 0xf8]
140a3fe6b: mov qword ptr [rbp + 0x150], r10
140a3fe72: add r10, 0x1a0
140a3fe79: mov qword ptr [rbp + 0x158], r10
140a3fe80: mov qword ptr [rbp + 0x160], r8
140a3fe87: mov qword ptr [rbp + 0x168], r9
140a3fe8e: mov qword ptr [rbp + 0x170], rax
140a3fe95: mov qword ptr [rbp + 0x178], rcx
140a3fe9c: mov qword ptr [rbp + 0x180], 8
140a3fea7: mov qword ptr [rbp + 0x188], 0
140a3feb2: mov qword ptr [rbp + 0x190], 8
140a3febd: mov byte ptr [rbp + 0x1b8], dl
140a3fec3: lea r8, [rip + 0x208ddce]
140a3feca: lea rcx, [rbp + 0x2e0]
140a3fed1: lea rdx, [rbp + 0x150]
140a3fed8: call 0x14130cb20
140a3fedd: mov rax, qword ptr [rbp + 0x2e0]
140a3fee4: mov qword ptr [rbp + 0x2c0], rax
140a3feeb: mov rax, qword ptr [rbp + 0x2e8]
140a3fef2: mov qword ptr [rbp + 0x2d8], rax
140a3fef9: mov rsi, qword ptr [rbp + 0x2f0]
140a3ff00: movaps xmm0, xmmword ptr [rbp - 0x40]
140a3ff04: movaps xmmword ptr [rbp + 0x70], xmm0
140a3ff08: mov rax, qword ptr [rbp - 0x30]
140a3ff0c: mov qword ptr [rbp + 0x80], rax
140a3ff13: mov byte ptr [rbp + 0x357], 0
140a3ff1a: mov rcx, rbx
140a3ff1d: call 0x140a84650
140a3ff22: mov qword ptr [rbp + 0x150], rax
140a3ff29: mov dword ptr [rbp + 0x158], edx
140a3ff2f: test rax, rax
140a3ff32: je 0x140a3ff78
140a3ff34: lock dec qword ptr [rax]
140a3ff38: jne 0x140a3ff4d
140a3ff3a: mov byte ptr [rbp + 0x357], 0
140a3ff41: lea rcx, [rbp + 0x150]
140a3ff48: call 0x1402e5d80
140a3ff4d: mov rax, qword ptr [rbp + 0x2c0]
140a3ff54: test rax, rax
140a3ff57: je 0x140a401e8
140a3ff5d: imul rdx, rax, 0x58
140a3ff61: mov r8d, 8
140a3ff67: mov rcx, qword ptr [rbp + 0x2d8]
140a3ff6e: call 0x140613c20
140a3ff73: jmp 0x140a401e8
140a3ff78: mov rax, qword ptr [rip + 0x2bec429]
140a3ff7f: mov rax, qword ptr [rax]
140a3ff82: cmp rax, 4
140a3ff86: jb 0x140a40050
140a3ff8c: mov byte ptr [rbp + 0x357], 0
140a3ff93: lea rcx, [rip + 0x208e32e]
140a3ff9a: call 0x14067dd80
140a3ff9f: movups xmm0, xmmword ptr [rax]
140a3ffa2: mov eax, dword ptr [rax + 0x10]
140a3ffa5: mov qword ptr [rbp + 0x180], 4
140a3ffb0: lea rcx, [rip + 0x208df91]
140a3ffb7: mov qword ptr [rbp + 0x188], rcx
140a3ffbe: mov qword ptr [rbp + 0x190], 0x26
140a3ffc9: lea rdx, [rip + 0x208e2e8]
140a3ffd0: mov qword ptr [rbp + 0x1a0], rdx
140a3ffd7: mov qword ptr [rbp + 0x1a8], 1
140a3ffe2: mov qword ptr [rbp + 0x1b0], 8
140a3ffed: mov qword ptr [rbp + 0x150], 0
140a3fff8: xorps xmm1, xmm1
140a3fffb: movups xmmword ptr [rbp + 0x1b8], xmm1
140a40002: mov qword ptr [rbp + 0x158], rcx
140a40009: mov qword ptr [rbp + 0x160], 0x26
140a40014: mov qword ptr [rbp + 0x168], 0
140a4001f: movups xmmword ptr [rbp + 0x170], xmm0
140a40026: mov dword ptr [rbp + 0x198], 1
140a40030: mov dword ptr [rbp + 0x19c], eax
140a40036: mov byte ptr [rbp + 0x357], 0
140a4003d: lea rcx, [rbp + 0x347]
140a40044: lea rdx, [rbp + 0x150]
140a4004b: call 0x140d4a330
140a40050: mov rax, qword ptr [rbp + 0x2c0]
140a40057: mov qword ptr [rbp + 0x150], rax
140a4005e: mov rax, qword ptr [rbp + 0x2d8]
140a40065: mov qword ptr [rbp + 0x158], rax
140a4006c: mov qword ptr [rbp + 0x160], rsi
140a40073: mov rax, qword ptr [rbp + 0x10]
140a40077: mov qword ptr [rbp + 0x168], rax
140a4007e: mov rax, qword ptr [rbp + 0xe8]
140a40085: mov qword ptr [rbp + 0x170], rax
140a4008c: lea rcx, [rbp + 0x178]
140a40093: mov r8d, 0xc0
140a40099: mov rdx, r15
140a4009c: call 0x142923c00
140a400a1: mov rax, qword ptr [rbp + 0xd0]
140a400a8: mov qword ptr [rbp + 0x238], rax
140a400af: mov rax, qword ptr [rbp + 0xd8]
140a400b6: mov qword ptr [rbp + 0x240], rax
140a400bd: mov rax, qword ptr [rbp + 0xe0]
140a400c4: mov qword ptr [rbp + 0x248], rax
140a400cb: mov rax, qword ptr [rbp + 0xb8]
140a400d2: mov qword ptr [rbp + 0x250], rax
140a400d9: mov rax, qword ptr [rbp + 0xc0]
140a400e0: mov qword ptr [rbp + 0x258], rax
140a400e7: movaps xmm0, xmmword ptr [rbp + 0x70]
140a400eb: movups xmmword ptr [rbp + 0x260], xmm0
140a400f2: mov rax, qword ptr [rbp + 0x80]
140a400f9: mov qword ptr [rbp + 0x270], rax
140a40100: mov rax, qword ptr [rbp + 0x20]
140a40104: mov qword ptr [rbp + 0x278], rax
140a4010b: mov rax, qword ptr [rbp + 0x30]
140a4010f: mov qword ptr [rbp + 0x280], rax
140a40116: mov rax, qword ptr [rbp + 0xc8]
140a4011d: mov qword ptr [rbp + 0x288], rax
140a40124: mov byte ptr [rbp + 0x357], 1
140a4012b: lea rcx, [rbp + 0x150]
140a40132: call 0x1417a0430
140a40137: mov rsi, rax
140a4013a: movzx eax, byte ptr [rip + 0x2bf4ac8]
140a40141: mov ecx, 0x18
140a40146: mov edx, 8
140a4014b: call 0x140613c10
140a40150: test rax, rax
140a40153: mov qword ptr [rbp + 0x290], rsi
140a4015a: mov qword ptr [rbp + 0x2a8], rax
140a40161: je 0x140a42416
140a40167: mov qword ptr [rax], 1
140a4016e: mov qword ptr [rax + 8], 1
140a40176: mov qword ptr [rax + 0x10], 0
140a4017e: mov qword ptr [rbp + 0x138], rax
140a40185: lock inc qword ptr [rax]
140a40189: jle 0x140a4248a
140a4018f: mov qword ptr [rbp + 0x158], rsi
140a40196: mov qword ptr [rbp + 0x160], rax
140a4019d: mov qword ptr [rbp + 0x150], 9
140a401a8: mov byte ptr [rbp + 0x356], 1
140a401af: lea rcx, [rbp + 0x2e0]
140a401b6: lea r8, [rbp + 0x150]
140a401bd: mov rdx, rdi
140a401c0: call 0x1408da280
140a401c5: cmp dword ptr [rbp + 0x2e0], 0xb
140a401cc: jne 0x140a42427
140a401d2: mov byte ptr [rbp + 0x356], 0
140a401d9: mov rcx, rbx
140a401dc: mov rdx, qword ptr [rbp + 0x2a8]
140a401e3: call 0x140a84760
140a401e8: mov rsi, qword ptr [rbx + 0x60]
140a401ec: lea rax, [rdi + 0x28]
140a401f0: mov qword ptr [rbp + 0x2c0], rax
140a401f7: xor eax, eax
140a401f9: cmp rax, qword ptr [rdi + 0x28]
140a401fd: jno 0x140a40252
140a401ff: movsd xmm7, qword ptr [rbp + 0x88]
140a40207: movups xmm6, xmmword ptr [rbp + 0x9c]
140a4020e: movzx r15d, word ptr [r14 + 0x28]
140a40213: mov eax, dword ptr [r14]
140a40216: mov dword ptr [rbp + 0x2a8], eax
140a4021c: movups xmm10, xmmword ptr [r14 + 4]
140a40221: movups xmm9, xmmword ptr [r14 + 0x14]
140a40226: movss xmm8, dword ptr [r14 + 0x24]
140a4022c: mov rax, qword ptr [rbp + 0x2a0]
140a40233: neg rax
140a40236: mov qword ptr [rbp + 0x290], rsi
140a4023d: jno 0x140a403d2
140a40243: movabs r14, 0x8000000000000000
140a4024d: jmp 0x140a4093c
140a40252: cmp qword ptr [rdi + 0x58], rsi
140a40256: je 0x140a402d2
140a40258: mov qword ptr [rdi + 0x58], rsi
140a4025c: movzx eax, byte ptr [rip + 0x2bf49a6]
140a40263: mov ecx, 8
140a40268: mov edx, 8
140a4026d: call 0x140613c10
140a40272: test rax, rax
140a40275: je 0x140a421ed
140a4027b: mov qword ptr [rax], rsi
140a4027e: mov byte ptr [rbp + 0x168], 1
140a40285: mov qword ptr [rbp + 0x158], rax
140a4028c: lea rax, [rip + 0x208dcdd]
140a40293: mov qword ptr [rbp + 0x160], rax
140a4029a: mov qword ptr [rbp + 0x150], 6
140a402a5: lea rcx, [rbp + 0x2e0]
140a402ac: lea r8, [rbp + 0x150]
140a402b3: mov rdx, rdi
140a402b6: call 0x1408da280
140a402bb: cmp dword ptr [rbp + 0x2e0], 0xb
140a402c2: jne 0x140a42201
140a402c8: mov dword ptr [rdi + 0x148], 0
140a402d2: movss xmm0, dword ptr [rdi + 0xd0]
140a402da: movss xmm6, dword ptr [rbp + 0x88]
140a402e2: ucomiss xmm0, xmm6
140a402e5: jne 0x140a40508
140a402eb: jp 0x140a40508
140a402f1: movss xmm0, dword ptr [rdi + 0xd4]
140a402f9: movss xmm7, dword ptr [rbp + 0x8c]
140a40301: ucomiss xmm0, xmm7
140a40304: jne 0x140a40510
140a4030a: jp 0x140a40510
140a40310: movss xmm0, dword ptr [rdi + 0xd8]
140a40318: ucomiss xmm0, dword ptr [rbp + 0x90]
140a4031f: jne 0x140a40510
140a40325: jp 0x140a40510
140a4032b: movss xmm0, dword ptr [rdi + 0xdc]
140a40333: ucomiss xmm0, dword ptr [rbp + 0x94]
140a4033a: jne 0x140a40510
140a40340: jp 0x140a40510
140a40346: movss xmm0, dword ptr [rdi + 0xe0]
140a4034e: ucomiss xmm0, dword ptr [rbp + 0x98]
140a40355: jne 0x140a40510
140a4035b: jp 0x140a40510
140a40361: movss xmm0, dword ptr [rdi + 0xe4]
140a40369: ucomiss xmm0, dword ptr [rbp + 0x9c]
140a40370: jne 0x140a40510
140a40376: jp 0x140a40510
140a4037c: movss xmm0, dword ptr [rdi + 0xe8]
140a40384: ucomiss xmm0, dword ptr [rbp + 0xa0]
140a4038b: jne 0x140a40510
140a40391: jp 0x140a40510
140a40397: movss xmm0, dword ptr [rdi + 0xec]
140a4039f: ucomiss xmm0, dword ptr [rbp + 0xa4]
140a403a6: jne 0x140a40510
140a403ac: jp 0x140a40510
140a403b2: movss xmm0, dword ptr [rdi + 0xf0]
140a403ba: ucomiss xmm0, dword ptr [rbp + 0xa8]
140a403c1: jne 0x140a40510
140a403c7: jp 0x140a40510
140a403cd: jmp 0x140a405d5
140a403d2: xor r12d, r12d
140a403d5: mov ecx, 0x28
140a403da: mov rdi, qword ptr [rbp + 0x328]
140a403e1: mov rax, rdi
140a403e4: mul rcx
140a403e7: mov rsi, rax
140a403ea: jo 0x140a4215a
140a403f0: movabs rax, 0x7ffffffffffffffc
140a403fa: cmp rsi, rax
140a403fd: ja 0x140a4215a
140a40403: test rsi, rsi
140a40406: je 0x140a4092d
140a4040c: movzx eax, byte ptr [rip + 0x2bf47f6]
140a40413: mov r12d, 4
140a40419: mov edx, 4
140a4041e: mov rcx, rsi
140a40421: call 0x140613c10
140a40426: mov qword ptr [rbp + 0x148], rax
140a4042d: test rax, rax
140a40430: je 0x140a4215a
140a40436: test rdi, rdi
140a40439: je 0x140a40939
140a4043f: mov r8, qword ptr [rbp + 0x2d0]
140a40446: lea rax, [r8 + 4]
140a4044a: xor ecx, ecx
140a4044c: mov rdx, rdi
140a4044f: mov r11, qword ptr [rbp + 0x148]
140a40456: jmp 0x140a404d0
140a40458: nop dword ptr [rax + rax]
140a40460: dec rdx
140a40463: mov r10d, dword ptr [rbp + 0x158]
140a4046a: mov dword ptr [rbp + 0x2e8], r10d
140a40471: mov r10, qword ptr [rbp + 0x150]
140a40478: mov qword ptr [rbp + 0x2e0], r10
140a4047f: mov dword ptr [r11 + rcx], r9d
140a40483: mov r9d, dword ptr [rbp + 0x2e8]
140a4048a: mov dword ptr [r11 + rcx + 0xc], r9d
140a4048f: mov r9, qword ptr [rbp + 0x2e0]
140a40496: mov qword ptr [r11 + rcx + 4], r9
140a4049b: mov r9, qword ptr [r8 + 0x10]
140a4049f: mov qword ptr [r11 + rcx + 0x10], r9
140a404a4: mov r9d, dword ptr [r8 + 0x18]
140a404a8: mov dword ptr [r11 + rcx + 0x18], r9d
140a404ad: mov r9, qword ptr [r8 + 0x1c]
140a404b1: mov qword ptr [r11 + rcx + 0x1c], r9
140a404b6: mov r9d, dword ptr [r8 + 0x24]
140a404ba: add r8, 0x28
140a404be: mov dword ptr [r11 + rcx + 0x24], r9d
140a404c3: add rcx, 0x28
140a404c7: test rdx, rdx
140a404ca: je 0x140a40928
140a404d0: cmp rsi, rcx
140a404d3: je 0x140a40928
140a404d9: xor r9d, r9d
140a404dc: cmp dword ptr [r8], 1
140a404e0: jne 0x140a40460
140a404e6: mov r9d, dword ptr [rax + rcx + 8]
140a404eb: mov dword ptr [rbp + 0x158], r9d
140a404f2: mov r9, qword ptr [rax + rcx]
140a404f6: mov qword ptr [rbp + 0x150], r9
140a404fd: mov r9d, 1
140a40503: jmp 0x140a40460
140a40508: movss xmm7, dword ptr [rbp + 0x8c]
140a40510: movups xmm8, xmmword ptr [rbp + 0x9c]
140a40518: movss dword ptr [rdi + 0xd0], xmm6
140a40520: movss dword ptr [rdi + 0xd4], xmm7
140a40528: mov rax, qword ptr [rbp + 0x90]
140a4052f: mov qword ptr [rdi + 0xd8], rax
140a40536: mov eax, dword ptr [rbp + 0x98]
140a4053c: mov dword ptr [rdi + 0xe0], eax
140a40542: movups xmmword ptr [rdi + 0xe4], xmm8
140a4054a: movzx eax, byte ptr [rip + 0x2bf46b8]
140a40551: mov ecx, 0x24
140a40556: mov edx, 4
140a4055b: call 0x140613c10
140a40560: test rax, rax
140a40563: je 0x140a42062
140a40569: lea rcx, [rbp + 0x90]
140a40570: movss dword ptr [rax], xmm6
140a40574: movss dword ptr [rax + 4], xmm7
140a40579: mov rdx, qword ptr [rcx]
140a4057c: mov qword ptr [rax + 8], rdx
140a40580: mov ecx, dword ptr [rcx + 8]
140a40583: mov dword ptr [rax + 0x10], ecx
140a40586: movups xmmword ptr [rax + 0x14], xmm8
140a4058b: mov byte ptr [rbp + 0x168], 3
140a40592: mov qword ptr [rbp + 0x158], rax
140a40599: lea rax, [rip + 0x208da08]
140a405a0: mov qword ptr [rbp + 0x160], rax
140a405a7: mov qword ptr [rbp + 0x150], 6
140a405b2: lea rcx, [rbp + 0x2e0]
140a405b9: lea r8, [rbp + 0x150]
140a405c0: mov rdx, rdi
140a405c3: call 0x1408da280
140a405c8: cmp dword ptr [rbp + 0x2e0], 0xb
140a405cf: jne 0x140a42076
140a405d5: movss xmm1, dword ptr [rdi + 0xac]
140a405dd: movss xmm0, dword ptr [r14 + 8]
140a405e3: ucomiss xmm1, xmm0
140a405e6: jne 0x140a406b7
140a405ec: jp 0x140a406b7
140a405f2: movss xmm2, dword ptr [rdi + 0xb0]
140a405fa: movss xmm1, dword ptr [r14 + 0xc]
140a40600: ucomiss xmm2, xmm1
140a40603: jne 0x140a406bd
140a40609: jp 0x140a406bd
140a4060f: movss xmm2, dword ptr [rdi + 0xb4]
140a40617: ucomiss xmm2, dword ptr [r14 + 0x10]
140a4061c: jne 0x140a406bd
140a40622: jp 0x140a406bd
140a40628: movzx eax, word ptr [rdi + 0xcc]
140a4062f: cmp ax, word ptr [r14 + 0x28]
140a40634: jne 0x140a406bd
140a4063a: mov eax, dword ptr [rdi + 0xa4]
140a40640: cmp eax, dword ptr [r14]
140a40643: jne 0x140a406bd
140a40645: test eax, eax
140a40647: je 0x140a4065d
140a40649: cmp eax, 1
140a4064c: movss xmm2, dword ptr [rdi + 0xa8]
140a40654: ucomiss xmm2, dword ptr [r14 + 4]
140a40659: jne 0x140a406bd
140a4065b: jp 0x140a406bd
140a4065d: movss xmm2, dword ptr [rdi + 0xb8]
140a40665: ucomiss xmm2, dword ptr [r14 + 0x14]
140a4066a: jne 0x140a406bd
140a4066c: jp 0x140a406bd
140a4066e: movss xmm2, dword ptr [rdi + 0xbc]
140a40676: ucomiss xmm2, dword ptr [r14 + 0x18]
140a4067b: jne 0x140a406bd
140a4067d: jp 0x140a406bd
140a4067f: movss xmm2, dword ptr [rdi + 0xc0]
140a40687: ucomiss xmm2, dword ptr [r14 + 0x1c]
140a4068c: jne 0x140a406bd
140a4068e: jp 0x140a406bd
140a40690: movss xmm2, dword ptr [rdi + 0xc4]
140a40698: ucomiss xmm2, dword ptr [r14 + 0x20]
140a4069d: jne 0x140a406bd
140a4069f: jp 0x140a406bd
140a406a1: movss xmm2, dword ptr [rdi + 0xc8]
140a406a9: ucomiss xmm2, dword ptr [r14 + 0x24]
140a406ae: jne 0x140a406bd
140a406b0: jp 0x140a406bd
140a406b2: jmp 0x140a407a5
140a406b7: movss xmm1, dword ptr [r14 + 0xc]
140a406bd: movzx eax, word ptr [r14 + 0x28]
140a406c2: mov ecx, dword ptr [r14]
140a406c5: movss xmm2, dword ptr [r14 + 4]
140a406cb: movups xmm3, xmmword ptr [r14 + 0x10]
140a406d0: movsd xmm4, qword ptr [r14 + 0x20]
140a406d6: mov dword ptr [rdi + 0xa4], ecx
140a406dc: movss dword ptr [rdi + 0xa8], xmm2
140a406e4: movss dword ptr [rdi + 0xac], xmm0
140a406ec: movss dword ptr [rdi + 0xb0], xmm1
140a406f4: movups xmmword ptr [rdi + 0xb4], xmm3
140a406fb: movsd qword ptr [rdi + 0xc4], xmm4
140a40703: mov word ptr [rdi + 0xcc], ax
140a4070a: movzx esi, word ptr [r14 + 0x28]
140a4070f: mov r15, rdi
140a40712: mov edi, dword ptr [r14]
140a40715: movups xmm6, xmmword ptr [r14 + 4]
140a4071a: movups xmm7, xmmword ptr [r14 + 0x14]
140a4071f: movss xmm8, dword ptr [r14 + 0x24]
140a40725: movzx eax, byte ptr [rip + 0x2bf44dd]
140a4072c: mov ecx, 0x2c
140a40731: mov edx, 4
140a40736: call 0x140613c10
140a4073b: test rax, rax
140a4073e: je 0x140a420de
140a40744: mov dword ptr [rax], edi
140a40746: movups xmmword ptr [rax + 4], xmm6
140a4074a: movups xmmword ptr [rax + 0x14], xmm7
140a4074e: movss dword ptr [rax + 0x24], xmm8
140a40754: mov word ptr [rax + 0x28], si
140a40758: mov byte ptr [rbp + 0x168], 4
140a4075f: mov qword ptr [rbp + 0x158], rax
140a40766: lea rax, [rip + 0x208d873]
140a4076d: mov qword ptr [rbp + 0x160], rax
140a40774: mov qword ptr [rbp + 0x150], 6
140a4077f: lea rcx, [rbp + 0x2e0]
140a40786: lea r8, [rbp + 0x150]
140a4078d: mov rdx, r15
140a40790: call 0x1408da280
140a40795: cmp dword ptr [rbp + 0x2e0], 0xb
140a4079c: jne 0x140a420f2
140a407a2: mov rdi, r15
140a407a5: xor eax, eax
140a407a7: cmp rax, qword ptr [rdi + 0x40]
140a407ab: seto cl
140a407ae: mov rax, qword ptr [rbp + 0x2a0]
140a407b5: neg rax
140a407b8: seto al
140a407bb: mov edx, eax
140a407bd: or dl, cl
140a407bf: je 0x140a407c5
140a407c1: and al, cl
140a407c3: jmp 0x140a407e0
140a407c5: mov rcx, qword ptr [rdi + 0x48]
140a407c9: mov rdx, qword ptr [rdi + 0x50]
140a407cd: mov r8, qword ptr [rbp + 0x2d0]
140a407d4: mov r9, qword ptr [rbp + 0x328]
140a407db: call 0x1416f8330
140a407e0: test al, al
140a407e2: jne 0x140a40f17
140a407e8: mov rax, qword ptr [rbp + 0x2a0]
140a407ef: neg rax
140a407f2: jno 0x140a40803
140a407f4: movabs rdi, 0x8000000000000000
140a407fe: jmp 0x140a40d1a
140a40803: xor r15d, r15d
140a40806: mov ecx, 0x28
140a4080b: mov rdi, qword ptr [rbp + 0x328]
140a40812: mov rax, rdi
140a40815: mul rcx
140a40818: mov r12, rax
140a4081b: jo 0x140a423eb
140a40821: movabs rax, 0x7ffffffffffffffc
140a4082b: cmp r12, rax
140a4082e: ja 0x140a423eb
140a40834: test r12, r12
140a40837: je 0x140a40d13
140a4083d: movzx eax, byte ptr [rip + 0x2bf43c5]
140a40844: mov r15d, 4
140a4084a: mov edx, 4
140a4084f: mov rcx, r12
140a40852: call 0x140613c10
140a40857: test rax, rax
140a4085a: je 0x140a423eb
140a40860: mov rsi, rax
140a40863: test rdi, rdi
140a40866: je 0x140a40d18
140a4086c: mov r8, qword ptr [rbp + 0x2d0]
140a40873: lea rax, [r8 + 4]
140a40877: xor ecx, ecx
140a40879: mov rdx, rdi
140a4087c: jmp 0x140a408f0
140a4087e: nop 
140a40880: dec rdx
140a40883: mov r10d, dword ptr [rbp + 0x158]
140a4088a: mov dword ptr [rbp + 0x2e8], r10d
140a40891: mov r10, qword ptr [rbp + 0x150]
140a40898: mov qword ptr [rbp + 0x2e0], r10
140a4089f: mov dword ptr [rsi + rcx], r9d
140a408a3: mov r9d, dword ptr [rbp + 0x2e8]
140a408aa: mov dword ptr [rsi + rcx + 0xc], r9d
140a408af: mov r9, qword ptr [rbp + 0x2e0]
140a408b6: mov qword ptr [rsi + rcx + 4], r9
140a408bb: mov r9, qword ptr [r8 + 0x10]
140a408bf: mov qword ptr [rsi + rcx + 0x10], r9
140a408c4: mov r9d, dword ptr [r8 + 0x18]
140a408c8: mov dword ptr [rsi + rcx + 0x18], r9d
140a408cd: mov r9, qword ptr [r8 + 0x1c]
140a408d1: mov qword ptr [rsi + rcx + 0x1c], r9
140a408d6: mov r9d, dword ptr [r8 + 0x24]
140a408da: add r8, 0x28
140a408de: mov dword ptr [rsi + rcx + 0x24], r9d
140a408e3: add rcx, 0x28
140a408e7: test rdx, rdx
140a408ea: je 0x140a40d1a
140a408f0: cmp r12, rcx
140a408f3: je 0x140a40d1a
140a408f9: xor r9d, r9d
140a408fc: cmp dword ptr [r8], 1
140a40900: jne 0x140a40880
140a40906: mov r9d, dword ptr [rax + rcx + 8]
140a4090b: mov dword ptr [rbp + 0x158], r9d
140a40912: mov r9, qword ptr [rax + rcx]
140a40916: mov qword ptr [rbp + 0x150], r9
140a4091d: mov r9d, 1
140a40923: jmp 0x140a40880
140a40928: mov r14, rdi
140a4092b: jmp 0x140a4093c
140a4092d: mov eax, 4
140a40932: mov qword ptr [rbp + 0x148], rax
140a40939: xor r14d, r14d
140a4093c: mov r8, qword ptr [rbx + 0x28]
140a40940: mov rsi, r8
140a40943: shl rsi, 4
140a40947: xor r12d, r12d
140a4094a: mov rax, r8
140a4094d: shr rax, 0x3c
140a40951: mov qword ptr [rbp + 0x2d8], r14
140a40958: jne 0x140a41e28
140a4095e: movabs rax, 0x7ffffffffffffff8
140a40968: cmp rsi, rax
140a4096b: ja 0x140a41e28
140a40971: mov eax, dword ptr [r13 + 0x18]
140a40975: mov dword ptr [rbp + 0x118], eax
140a4097b: mov eax, dword ptr [r13 + 0x1c]
140a4097f: mov dword ptr [rbp + 0x110], eax
140a40985: mov eax, dword ptr [r13 + 0x20]
140a40989: mov dword ptr [rbp + 0x108], eax
140a4098f: movzx eax, byte ptr [r13 + 0x42]
140a40994: mov byte ptr [rbp + 0xd8], al
140a4099a: movzx eax, byte ptr [r13 + 0x40]
140a4099f: mov byte ptr [rbp + 0xe0], al
140a409a5: mov eax, dword ptr [r13 + 0x24]
140a409a9: mov dword ptr [rbp + 0x114], eax
140a409af: mov eax, dword ptr [r13 + 0x28]
140a409b3: mov dword ptr [rbp + 0x10c], eax
140a409b9: mov eax, dword ptr [r13 + 0x2c]
140a409bd: mov dword ptr [rbp + 0x104], eax
140a409c3: mov eax, dword ptr [r13 + 0x30]
140a409c7: mov dword ptr [rbp + 0x140], eax
140a409cd: mov eax, dword ptr [r13 + 0x34]
140a409d1: mov dword ptr [rbp + 0xf0], eax
140a409d7: mov eax, dword ptr [r13]
140a409db: mov dword ptr [rbp + 0x124], eax
140a409e1: mov eax, dword ptr [r13 + 4]
140a409e5: mov dword ptr [rbp + 0x120], eax
140a409eb: movzx eax, byte ptr [r13 + 0x41]
140a409f0: mov byte ptr [rbp + 0xe8], al
140a409f6: movss xmm11, dword ptr [r13 + 0x3c]
140a409fc: movzx eax, byte ptr [r13 + 0x14]
140a40a01: mov byte ptr [rbp + 0x337], al
140a40a07: mov eax, dword ptr [r13 + 0x38]
140a40a0b: mov dword ptr [rbp + 0x11c], eax
140a40a11: mov eax, dword ptr [r13 + 8]
140a40a15: mov dword ptr [rbp + 0x130], eax
140a40a1b: mov eax, dword ptr [r13 + 0xc]
140a40a1f: mov dword ptr [rbp + 0x12c], eax
140a40a25: mov eax, dword ptr [r13 + 0x10]
140a40a29: mov dword ptr [rbp + 0x128], eax
140a40a2f: mov eax, dword ptr [rbx + 0x68]
140a40a32: mov dword ptr [rbp + 0xd0], eax
140a40a38: mov r13d, dword ptr [rbx + 0x6c]
140a40a3c: test rsi, rsi
140a40a3f: movaps xmmword ptr [rbp], xmm6
140a40a43: movaps xmmword ptr [rbp + 0x10], xmm7
140a40a47: mov word ptr [rbp + 0xb8], r15w
140a40a4f: movss dword ptr [rbp + 0xc0], xmm8
140a40a58: movaps xmmword ptr [rbp + 0x20], xmm9
140a40a5d: movaps xmmword ptr [rbp + 0x30], xmm10
140a40a62: mov dword ptr [rbp + 0xc8], r13d
140a40a69: movss dword ptr [rbp + 0x100], xmm11
140a40a72: mov qword ptr [rbp + 0x338], r8
140a40a79: je 0x140a40aff
140a40a7f: mov rdi, qword ptr [rbx + 0x20]
140a40a83: movzx eax, byte ptr [rip + 0x2bf417f]
140a40a8a: mov r12d, 8
140a40a90: mov edx, 8
140a40a95: mov rcx, rsi
140a40a98: call 0x140613c10
140a40a9d: mov rcx, qword ptr [rbp + 0x338]
140a40aa4: test rax, rax
140a40aa7: je 0x140a41e28
140a40aad: test rcx, rcx
140a40ab0: mov qword ptr [rbp + 0x68], rax
140a40ab4: je 0x140a40b1f
140a40ab6: xor r9d, r9d
140a40ab9: mov r12, qword ptr [rbp + 0x298]
140a40ac0: cmp rsi, r9
140a40ac3: je 0x140a40aef
140a40ac5: mov rdx, qword ptr [rdi]
140a40ac8: mov r8d, dword ptr [rdi + 8]
140a40acc: lock inc qword ptr [rdx]
140a40ad0: jle 0x140a4248a
140a40ad6: dec rcx
140a40ad9: add rdi, 0x10
140a40add: mov qword ptr [rax + r9], rdx
140a40ae1: mov dword ptr [rax + r9 + 8], r8d
140a40ae6: add r9, 0x10
140a40aea: test rcx, rcx
140a40aed: jne 0x140a40ac0
140a40aef: xor eax, eax
140a40af1: mov rsi, qword ptr [rbp + 0x2c0]
140a40af8: cmp rax, qword ptr [rsi]
140a40afb: jo 0x140a40b68
140a40afd: jmp 0x140a40b34
140a40aff: mov eax, 8
140a40b04: mov qword ptr [rbp + 0x68], rax
140a40b08: mov r12, qword ptr [rbp + 0x298]
140a40b0f: xor eax, eax
140a40b11: mov rsi, qword ptr [rbp + 0x2c0]
140a40b18: cmp rax, qword ptr [rsi]
140a40b1b: jo 0x140a40b68
140a40b1d: jmp 0x140a40b34
140a40b1f: mov r12, qword ptr [rbp + 0x298]
140a40b26: xor eax, eax
140a40b28: mov rsi, qword ptr [rbp + 0x2c0]
140a40b2f: cmp rax, qword ptr [rsi]
140a40b32: jo 0x140a40b68
140a40b34: mov rdx, qword ptr [rbp + 0x348]
140a40b3b: mov rax, qword ptr [rdx + 0x40]
140a40b3f: mov rcx, rax
140a40b42: neg rcx
140a40b45: jo 0x140a40b60
140a40b47: jae 0x140a40b60
140a40b49: mov rcx, qword ptr [rdx + 0x48]
140a40b4d: shl rax, 3
140a40b51: lea rdx, [rax + rax*4]
140a40b55: mov r8d, 4
140a40b5b: call 0x140613c20
140a40b60: mov rcx, rsi
140a40b63: call 0x140600290
140a40b68: mov rdi, qword ptr [rbp + 0x348]
140a40b6f: mov rax, qword ptr [rbp + 0x338]
140a40b76: mov qword ptr [rdi + 0x28], rax
140a40b7a: mov rcx, qword ptr [rbp + 0x68]
140a40b7e: mov qword ptr [rdi + 0x30], rcx
140a40b82: mov qword ptr [rdi + 0x38], rax
140a40b86: mov rax, qword ptr [rbp + 0x2d8]
140a40b8d: mov qword ptr [rdi + 0x40], rax
140a40b91: mov rax, qword ptr [rbp + 0x148]
140a40b98: mov qword ptr [rdi + 0x48], rax
140a40b9c: mov rax, qword ptr [rbp + 0x328]
140a40ba3: mov qword ptr [rdi + 0x50], rax
140a40ba7: mov rax, qword ptr [rbp + 0x290]
140a40bae: mov qword ptr [rdi + 0x58], rax
140a40bb2: mov eax, dword ptr [rbp + 0x124]
140a40bb8: mov dword ptr [rdi + 0x60], eax
140a40bbb: mov eax, dword ptr [rbp + 0x120]
140a40bc1: mov dword ptr [rdi + 0x64], eax
140a40bc4: mov eax, dword ptr [rbp + 0x130]
140a40bca: mov dword ptr [rdi + 0x68], eax
140a40bcd: mov eax, dword ptr [rbp + 0x12c]
140a40bd3: mov dword ptr [rdi + 0x6c], eax
140a40bd6: mov eax, dword ptr [rbp + 0x128]
140a40bdc: mov dword ptr [rdi + 0x70], eax
140a40bdf: movzx eax, byte ptr [rbp + 0x337]
140a40be6: mov byte ptr [rdi + 0x74], al
140a40be9: mov eax, dword ptr [rbp + 0x118]
140a40bef: mov dword ptr [rdi + 0x78], eax
140a40bf2: mov eax, dword ptr [rbp + 0x110]
140a40bf8: mov dword ptr [rdi + 0x7c], eax
140a40bfb: mov eax, dword ptr [rbp + 0x108]
140a40c01: mov dword ptr [rdi + 0x80], eax
140a40c07: mov eax, dword ptr [rbp + 0x114]
140a40c0d: mov dword ptr [rdi + 0x84], eax
140a40c13: mov eax, dword ptr [rbp + 0x10c]
140a40c19: mov dword ptr [rdi + 0x88], eax
140a40c1f: mov eax, dword ptr [rbp + 0x104]
140a40c25: mov dword ptr [rdi + 0x8c], eax
140a40c2b: mov eax, dword ptr [rbp + 0x140]
140a40c31: mov dword ptr [rdi + 0x90], eax
140a40c37: mov eax, dword ptr [rbp + 0xf0]
140a40c3d: mov dword ptr [rdi + 0x94], eax
140a40c43: mov eax, dword ptr [rbp + 0x11c]
140a40c49: mov dword ptr [rdi + 0x98], eax
140a40c4f: movss xmm0, dword ptr [rbp + 0x100]
140a40c57: movss dword ptr [rdi + 0x9c], xmm0
140a40c5f: movzx eax, byte ptr [rbp + 0xe0]
140a40c66: mov byte ptr [rdi + 0xa0], al
140a40c6c: movzx eax, byte ptr [rbp + 0xe8]
140a40c73: mov byte ptr [rdi + 0xa1], al
140a40c79: movzx eax, byte ptr [rbp + 0xd8]
140a40c80: mov byte ptr [rdi + 0xa2], al
140a40c86: mov eax, dword ptr [rbp + 0x2a8]
140a40c8c: mov dword ptr [rdi + 0xa4], eax
140a40c92: movaps xmm0, xmmword ptr [rbp + 0x30]
140a40c96: movups xmmword ptr [rdi + 0xa8], xmm0
140a40c9d: movaps xmm0, xmmword ptr [rbp + 0x20]
140a40ca1: movups xmmword ptr [rdi + 0xb8], xmm0
140a40ca8: movss xmm0, dword ptr [rbp + 0xc0]
140a40cb0: movss dword ptr [rdi + 0xc8], xmm0
140a40cb8: movzx eax, word ptr [rbp + 0xb8]
140a40cbf: mov word ptr [rdi + 0xcc], ax
140a40cc6: movaps xmm0, xmmword ptr [rbp + 0x10]
140a40cca: movlps qword ptr [rdi + 0xd0], xmm0
140a40cd1: lea rcx, [rbp + 0x90]
140a40cd8: mov eax, dword ptr [rcx + 8]
140a40cdb: mov dword ptr [rdi + 0xe0], eax
140a40ce1: mov rax, qword ptr [rcx]
140a40ce4: mov qword ptr [rdi + 0xd8], rax
140a40ceb: movaps xmm0, xmmword ptr [rbp]
140a40cef: movups xmmword ptr [rdi + 0xe4], xmm0
140a40cf6: mov eax, dword ptr [rbp + 0xd0]
140a40cfc: mov dword ptr [rdi + 0xf4], eax
140a40d02: mov eax, dword ptr [rbp + 0xc8]
140a40d08: mov dword ptr [rdi + 0xf8], eax
140a40d0e: jmp 0x140a416b2
140a40d13: mov esi, 4
140a40d18: xor edi, edi
140a40d1a: mov rbx, qword ptr [rbp + 0x348]
140a40d21: mov rax, qword ptr [rbx + 0x40]
140a40d25: mov rcx, rax
140a40d28: neg rcx
140a40d2b: jo 0x140a40d46
140a40d2d: jae 0x140a40d46
140a40d2f: mov rcx, qword ptr [rbx + 0x48]
140a40d33: shl rax, 3
140a40d37: lea rdx, [rax + rax*4]
140a40d3b: mov r8d, 4
140a40d41: call 0x140613c20
140a40d46: mov rax, qword ptr [rbp + 0x2a0]
140a40d4d: neg rax
140a40d50: mov qword ptr [rbx + 0x40], rdi
140a40d54: mov qword ptr [rbx + 0x48], rsi
140a40d58: mov rax, qword ptr [rbp + 0x328]
140a40d5f: mov qword ptr [rbx + 0x50], rax
140a40d63: mov rdi, rbx
140a40d66: jno 0x140a40d77
140a40d68: movabs rsi, 0x8000000000000000
140a40d72: jmp 0x140a40e9c
140a40d77: xor r12d, r12d
140a40d7a: mov ecx, 0x28
140a40d7f: mul rcx
140a40d82: mov rsi, rax
140a40d85: jo 0x140a42402
140a40d8b: movabs rax, 0x7ffffffffffffffc
140a40d95: cmp rsi, rax
140a40d98: ja 0x140a42402
140a40d9e: test rsi, rsi
140a40da1: je 0x140a40e95
140a40da7: movzx eax, byte ptr [rip + 0x2bf3e5b]
140a40dae: mov r12d, 4
140a40db4: mov edx, 4
140a40db9: mov rcx, rsi
140a40dbc: call 0x140613c10
140a40dc1: test rax, rax
140a40dc4: je 0x140a42402
140a40dca: mov rbx, rax
140a40dcd: mov rdx, qword ptr [rbp + 0x328]
140a40dd4: test rdx, rdx
140a40dd7: je 0x140a40e9a
140a40ddd: mov r8, qword ptr [rbp + 0x2d0]
140a40de4: lea rax, [r8 + 4]
140a40de8: xor ecx, ecx
140a40dea: jmp 0x140a40e5c
140a40dec: nop dword ptr [rax]
140a40df0: dec rdx
140a40df3: mov r10d, dword ptr [rbp + 0x158]
140a40dfa: mov dword ptr [rbp + 0x2e8], r10d
140a40e01: mov r10, qword ptr [rbp + 0x150]
140a40e08: mov qword ptr [rbp + 0x2e0], r10
140a40e0f: mov dword ptr [rbx + rcx], r9d
140a40e13: mov r9d, dword ptr [rbp + 0x2e8]
140a40e1a: mov dword ptr [rbx + rcx + 0xc], r9d
140a40e1f: mov r9, qword ptr [rbp + 0x2e0]
140a40e26: mov qword ptr [rbx + rcx + 4], r9
140a40e2b: mov r9, qword ptr [r8 + 0x10]
140a40e2f: mov qword ptr [rbx + rcx + 0x10], r9
140a40e34: mov r9d, dword ptr [r8 + 0x18]
140a40e38: mov dword ptr [rbx + rcx + 0x18], r9d
140a40e3d: mov r9, qword ptr [r8 + 0x1c]
140a40e41: mov qword ptr [rbx + rcx + 0x1c], r9
140a40e46: mov r9d, dword ptr [r8 + 0x24]
140a40e4a: add r8, 0x28
140a40e4e: mov dword ptr [rbx + rcx + 0x24], r9d
140a40e53: add rcx, 0x28
140a40e57: test rdx, rdx
140a40e5a: je 0x140a40e8c
140a40e5c: cmp rsi, rcx
140a40e5f: je 0x140a40e8c
140a40e61: xor r9d, r9d
140a40e64: cmp dword ptr [r8], 1
140a40e68: jne 0x140a40df0
140a40e6a: mov r9d, dword ptr [rax + rcx + 8]
140a40e6f: mov dword ptr [rbp + 0x158], r9d
140a40e76: mov r9, qword ptr [rax + rcx]
140a40e7a: mov qword ptr [rbp + 0x150], r9
140a40e81: mov r9d, 1
140a40e87: jmp 0x140a40df0
140a40e8c: mov rsi, qword ptr [rbp + 0x328]
140a40e93: jmp 0x140a40e9c
140a40e95: mov ebx, 4
140a40e9a: xor esi, esi
140a40e9c: movzx eax, byte ptr [rip + 0x2bf3d66]
140a40ea3: mov ecx, 0x18
140a40ea8: mov edx, 8
140a40ead: call 0x140613c10
140a40eb2: test rax, rax
140a40eb5: je 0x140a42269
140a40ebb: mov qword ptr [rax], rsi
140a40ebe: mov qword ptr [rax + 8], rbx
140a40ec2: mov rcx, qword ptr [rbp + 0x328]
140a40ec9: mov qword ptr [rax + 0x10], rcx
140a40ecd: mov byte ptr [rbp + 0x168], 5
140a40ed4: mov qword ptr [rbp + 0x158], rax
140a40edb: lea rax, [rip + 0x208d136]
140a40ee2: mov qword ptr [rbp + 0x160], rax
140a40ee9: mov qword ptr [rbp + 0x150], 6
140a40ef4: lea rcx, [rbp + 0x2e0]
140a40efb: lea r8, [rbp + 0x150]
140a40f02: mov rdx, rdi
140a40f05: call 0x1408da280
140a40f0a: cmp dword ptr [rbp + 0x2e0], 0xb
140a40f11: jne 0x140a4228b
140a40f17: mov eax, dword ptr [r13 + 0x18]
140a40f1b: cmp dword ptr [rdi + 0x78], eax
140a40f1e: jne 0x140a40fe1
140a40f24: mov ecx, dword ptr [rdi + 0x7c]
140a40f27: mov edx, dword ptr [r13 + 0x1c]
140a40f2b: cmp ecx, edx
140a40f2d: jne 0x140a411cc
140a40f33: mov edx, dword ptr [rdi + 0x80]
140a40f39: cmp edx, dword ptr [r13 + 0x20]
140a40f3d: jne 0x140a411ca
140a40f43: movzx edx, byte ptr [rdi + 0xa2]
140a40f4a: cmp dl, byte ptr [r13 + 0x42]
140a40f4e: jne 0x140a411ca
140a40f54: movzx edx, byte ptr [rdi + 0xa0]
140a40f5b: cmp dl, byte ptr [r13 + 0x40]
140a40f5f: jne 0x140a411ca
140a40f65: mov edx, dword ptr [rdi + 0x84]
140a40f6b: cmp edx, dword ptr [r13 + 0x24]
140a40f6f: jne 0x140a411ca
140a40f75: mov edx, dword ptr [rdi + 0x88]
140a40f7b: cmp edx, dword ptr [r13 + 0x28]
140a40f7f: jne 0x140a411ca
140a40f85: mov edx, dword ptr [rdi + 0x8c]
140a40f8b: cmp edx, dword ptr [r13 + 0x2c]
140a40f8f: jne 0x140a411ca
140a40f95: mov edx, dword ptr [rdi + 0x90]
140a40f9b: cmp edx, dword ptr [r13 + 0x30]
140a40f9f: jne 0x140a411ca
140a40fa5: mov edx, dword ptr [rdi + 0x94]
140a40fab: cmp edx, dword ptr [r13 + 0x34]
140a40faf: jne 0x140a411ca
140a40fb5: movzx edx, byte ptr [r13]
140a40fba: test byte ptr [rdi + 0x60], 1
140a40fbe: je 0x140a4111e
140a40fc4: mov r8d, dword ptr [r13 + 4]
140a40fc8: cmp dword ptr [rdi + 0x64], r8d
140a40fcc: sete r8b
140a40fd0: and dl, r8b
140a40fd3: cmp dl, 1
140a40fd6: je 0x140a41127
140a40fdc: jmp 0x140a411ca
140a40fe1: mov edx, dword ptr [r13 + 0x1c]
140a40fe5: jmp 0x140a411cc
140a40fea: test rax, rax
140a40fed: jne 0x140a41dff
140a40ff3: xor edx, edx
140a40ff5: call 0x1428f87d0
140a40ffa: mov rcx, rax
140a40ffd: test rax, rax
140a41000: jne 0x140a3f80a
140a41006: jmp 0x140a41dff
140a4100b: mov r9d, 0x1c20
140a41011: mov rdi, qword ptr [rbp + 0x348]
140a41018: jmp 0x140a41039
140a4101a: mov r9d, 0xe10
140a41020: mov r8, r10
140a41023: jmp 0x140a41039
140a41025: mov r9d, 0x258
140a4102b: mov r8, r11
140a4102e: jmp 0x140a41039
140a41030: mov r9d, 0x12c
140a41036: mov r8, rax
140a41039: movss dword ptr [rdi + 0x144], xmm0
140a41041: lea edx, [r12 + 1]
140a41046: mov dword ptr [rdi + 0x140], edx
140a4104c: mov edx, r12d
140a4104f: sub edx, r9d
140a41052: jae 0x140a4106d
140a41054: mov dword ptr [rbp + 0x2b0], 0
140a4105e: mov dword ptr [rbp + 0x2b8], 0
140a41068: jmp 0x140a3fd42
140a4106d: sub r8, rax
140a41070: cmp r8, 0xf
140a41074: ja 0x140a410b8
140a41076: lea r10, [rip + 0x205e2cb]
140a4107d: mov edi, r12d
140a41080: sub edi, r9d
140a41083: mov r11, r8
140a41086: nop word ptr cs:[rax + rax]
140a41090: cmp r11, 0xc
140a41094: je 0x140a410c0
140a41096: mov esi, dword ptr [r11 + r10 + 4]
140a4109b: test esi, esi
140a4109d: je 0x140a41e3f
140a410a3: mov eax, edi
140a410a5: xor edx, edx
140a410a7: div esi
140a410a9: add r11, 4
140a410ad: test edx, edx
140a410af: jne 0x140a41090
140a410b1: cmp edi, r9d
140a410b4: jae 0x140a41080
140a410b6: jmp 0x140a41054
140a410b8: mov dword ptr [rbp + 0x134], edx
140a410be: jmp 0x140a410c6
140a410c0: mov dword ptr [rbp + 0x134], edi
140a410c6: mov dword ptr [rbp + 0x2b8], 1
140a410d0: mov dword ptr [rbp + 0x2b0], 0
140a410da: jmp 0x140a3fd42
140a410df: mov rcx, rsi
140a410e2: mov r8d, 0x3b9aca00
140a410e8: call 0x1428cd210
140a410ed: jmp 0x140a3f8af
140a410f2: mov rcx, rsi
140a410f5: xor edx, edx
140a410f7: call 0x1428cdb90
140a410fc: jmp 0x140a3f9fb
140a41101: lea rcx, [rip + 0x205e1b8]
140a41108: lea r8, [rip + 0x205e221]
140a4110f: mov edx, 0x32
140a41114: call 0x1428d9430
140a41119: jmp 0x140a4248a
140a4111e: test dl, 1
140a41121: jne 0x140a411ca
140a41127: mov edx, dword ptr [rdi + 0x98]
140a4112d: cmp edx, dword ptr [r13 + 0x38]
140a41131: jne 0x140a411ca
140a41137: movzx edx, byte ptr [rdi + 0xa1]
140a4113e: cmp dl, byte ptr [r13 + 0x41]
140a41142: jne 0x140a411ca
140a41148: movss xmm0, dword ptr [rdi + 0x9c]
140a41150: mov r9, r13
140a41153: ucomiss xmm0, dword ptr [r13 + 0x3c]
140a41158: jne 0x140a41e17
140a4115e: jp 0x140a41e17
140a41164: mov rdx, qword ptr [rbp + 0x348]
140a4116b: mov edx, dword ptr [rdx + 0x68]
140a4116e: cmp edx, dword ptr [r9 + 8]
140a41172: jne 0x140a41e17
140a41178: mov rdx, qword ptr [rbp + 0x348]
140a4117f: mov edx, dword ptr [rdx + 0x6c]
140a41182: cmp edx, dword ptr [r9 + 0xc]
140a41186: jne 0x140a41e17
140a4118c: mov rdx, qword ptr [rbp + 0x348]
140a41193: movzx edx, byte ptr [rdx + 0x74]
140a41197: cmp dl, byte ptr [r9 + 0x14]
140a4119b: jne 0x140a41e17
140a411a1: mov rdi, qword ptr [rbp + 0x348]
140a411a8: mov r8d, dword ptr [rdi + 0x70]
140a411ac: mov edx, ecx
140a411ae: mov r13, r9
140a411b1: cmp r8d, dword ptr [r9 + 0x10]
140a411b5: mov r12, qword ptr [rbp + 0x298]
140a411bc: mov rbx, qword ptr [rbp + 0x338]
140a411c3: jne 0x140a411cc
140a411c5: jmp 0x140a41305
140a411ca: mov edx, ecx
140a411cc: movzx ecx, byte ptr [r13 + 0x42]
140a411d1: movzx r8d, word ptr [r13 + 0x40]
140a411d6: mov r9d, dword ptr [r13 + 0x38]
140a411da: movss xmm0, dword ptr [r13 + 0x3c]
140a411e0: movzx r10d, byte ptr [r13 + 0x14]
140a411e5: mov r11d, dword ptr [r13 + 0x10]
140a411e9: movups xmm1, xmmword ptr [r13 + 0x20]
140a411ee: mov rsi, qword ptr [r13 + 0x30]
140a411f2: movups xmm2, xmmword ptr [r13]
140a411f7: movups xmmword ptr [rdi + 0x60], xmm2
140a411fb: mov dword ptr [rdi + 0x70], r11d
140a411ff: mov byte ptr [rdi + 0x74], r10b
140a41203: mov dword ptr [rdi + 0x78], eax
140a41206: mov dword ptr [rdi + 0x7c], edx
140a41209: movups xmmword ptr [rdi + 0x80], xmm1
140a41210: mov qword ptr [rdi + 0x90], rsi
140a41217: mov dword ptr [rdi + 0x98], r9d
140a4121e: movss dword ptr [rdi + 0x9c], xmm0
140a41226: mov word ptr [rdi + 0xa0], r8w
140a4122e: mov byte ptr [rdi + 0xa2], cl
140a41234: movzx esi, byte ptr [r13 + 0x42]
140a41239: movzx ebx, byte ptr [r13 + 0x40]
140a4123e: movups xmm6, xmmword ptr [r13 + 0x18]
140a41243: movups xmm7, xmmword ptr [r13 + 0x28]
140a41248: mov r15d, dword ptr [r13 + 0x38]
140a4124c: movzx r14d, byte ptr [r13 + 0x41]
140a41251: movss xmm8, dword ptr [r13 + 0x3c]
140a41257: movups xmm9, xmmword ptr [r13]
140a4125c: movzx r12d, byte ptr [r13 + 0x14]
140a41261: mov r13d, dword ptr [r13 + 0x10]
140a41265: movzx eax, byte ptr [rip + 0x2bf399d]
140a4126c: mov ecx, 0x44
140a41271: mov edx, 4
140a41276: call 0x140613c10
140a4127b: test rax, rax
140a4127e: je 0x140a41ee6
140a41284: movups xmmword ptr [rax], xmm9
140a41288: mov dword ptr [rax + 0x10], r13d
140a4128c: mov byte ptr [rax + 0x14], r12b
140a41290: movups xmmword ptr [rax + 0x18], xmm6
140a41294: movups xmmword ptr [rax + 0x28], xmm7
140a41298: mov dword ptr [rax + 0x38], r15d
140a4129c: movss dword ptr [rax + 0x3c], xmm8
140a412a2: mov byte ptr [rax + 0x40], bl
140a412a5: mov byte ptr [rax + 0x41], r14b
140a412a9: mov byte ptr [rax + 0x42], sil
140a412ad: mov byte ptr [rbp + 0x168], 6
140a412b4: mov qword ptr [rbp + 0x158], rax
140a412bb: lea rax, [rip + 0x208cd8e]
140a412c2: mov qword ptr [rbp + 0x160], rax
140a412c9: mov qword ptr [rbp + 0x150], 6
140a412d4: lea rcx, [rbp + 0x2e0]
140a412db: lea r8, [rbp + 0x150]
140a412e2: mov rdx, rdi
140a412e5: call 0x1408da280
140a412ea: mov r12, qword ptr [rbp + 0x298]
140a412f1: mov rbx, qword ptr [rbp + 0x338]
140a412f8: cmp dword ptr [rbp + 0x2e0], 0xb
140a412ff: jne 0x140a41efa
140a41305: mov rax, qword ptr [rdi + 0x38]
140a41309: mov r14, qword ptr [rbx + 0x20]
140a4130d: mov r15, qword ptr [rbx + 0x28]
140a41311: cmp rax, r15
140a41314: jne 0x140a41375
140a41316: test rax, rax
140a41319: je 0x140a415a0
140a4131f: mov rcx, qword ptr [rdi + 0x30]
140a41323: mov edx, 8
140a41328: jmp 0x140a41343
140a4132a: nop word ptr [rax + rax]
140a41330: cmp r8, 2
140a41334: jb 0x140a41375
140a41336: add rdx, 0x10
140a4133a: dec rax
140a4133d: je 0x140a415a0
140a41343: mov r8d, dword ptr [rcx + rdx]
140a41347: cmp r8d, dword ptr [r14 + rdx]
140a4134b: jne 0x140a41375
140a4134d: mov r10, qword ptr [rcx + rdx - 8]
140a41352: mov r9, qword ptr [r14 + rdx - 8]
140a41357: mov r8, qword ptr [r10 + 0x10]
140a4135b: cmp r10, r9
140a4135e: je 0x140a41330
140a41360: mov r9, qword ptr [r9 + 0x10]
140a41364: cmp r8, 2
140a41368: jb 0x140a41375
140a4136a: cmp r9, 2
140a4136e: jb 0x140a41375
140a41370: cmp r8, r9
140a41373: je 0x140a41336
140a41375: mov rsi, r15
140a41378: shl rsi, 4
140a4137c: xor r12d, r12d
140a4137f: mov rax, r15
140a41382: shr rax, 0x3c
140a41386: jne 0x140a41eb8
140a4138c: movabs r13, 0x7ffffffffffffff8
140a41396: cmp rsi, r13
140a41399: mov rdi, qword ptr [rbp + 0x2c0]
140a413a0: ja 0x140a41eb8
140a413a6: test rsi, rsi
140a413a9: mov qword ptr [rbp + 0x328], r15
140a413b0: je 0x140a41431
140a413b2: movzx eax, byte ptr [rip + 0x2bf3850]
140a413b9: mov r12d, 8
140a413bf: mov edx, 8
140a413c4: mov rcx, rsi
140a413c7: call 0x140613c10
140a413cc: mov qword ptr [rbp + 0x338], rax
140a413d3: test rax, rax
140a413d6: je 0x140a41eb8
140a413dc: test r15, r15
140a413df: mov r12, qword ptr [rbp + 0x298]
140a413e6: je 0x140a41444
140a413e8: xor eax, eax
140a413ea: mov rcx, r15
140a413ed: mov r9, qword ptr [rbp + 0x338]
140a413f4: nop word ptr cs:[rax + rax]
140a41400: cmp rsi, rax
140a41403: je 0x140a41444
140a41405: mov rdx, qword ptr [r14]
140a41408: mov r8d, dword ptr [r14 + 8]
140a4140c: lock inc qword ptr [rdx]
140a41410: jle 0x140a4248a
140a41416: dec rcx
140a41419: add r14, 0x10
140a4141d: mov qword ptr [r9 + rax], rdx
140a41421: mov dword ptr [r9 + rax + 8], r8d
140a41426: add rax, 0x10
140a4142a: test rcx, rcx
140a4142d: jne 0x140a41400
140a4142f: jmp 0x140a41444
140a41431: mov eax, 8
140a41436: mov qword ptr [rbp + 0x338], rax
140a4143d: mov r12, qword ptr [rbp + 0x298]
140a41444: mov rcx, rdi
140a41447: call 0x140600290
140a4144c: mov rax, qword ptr [rbp + 0x348]
140a41453: mov rcx, qword ptr [rbp + 0x328]
140a4145a: mov qword ptr [rax + 0x28], rcx
140a4145e: mov rdx, qword ptr [rbp + 0x338]
140a41465: mov qword ptr [rax + 0x30], rdx
140a41469: mov qword ptr [rax + 0x38], rcx
140a4146d: mov r14, qword ptr [rbx + 0x28]
140a41471: mov rsi, r14
140a41474: shl rsi, 4
140a41478: xor r15d, r15d
140a4147b: mov rax, r14
140a4147e: shr rax, 0x3c
140a41482: jne 0x140a41ecf
140a41488: cmp rsi, r13
140a4148b: ja 0x140a41ecf
140a41491: test rsi, rsi
140a41494: je 0x140a41501
140a41496: mov rdi, qword ptr [rbx + 0x20]
140a4149a: movzx eax, byte ptr [rip + 0x2bf3768]
140a414a1: mov r15d, 8
140a414a7: mov edx, 8
140a414ac: mov rcx, rsi
140a414af: call 0x140613c10
140a414b4: test rax, rax
140a414b7: je 0x140a41ecf
140a414bd: test r14, r14
140a414c0: je 0x140a41506
140a414c2: xor ecx, ecx
140a414c4: mov rdx, r14
140a414c7: nop word ptr [rax + rax]
140a414d0: cmp rsi, rcx
140a414d3: je 0x140a41506
140a414d5: mov r8, qword ptr [rdi]
140a414d8: mov r9d, dword ptr [rdi + 8]
140a414dc: lock inc qword ptr [r8]
140a414e0: jle 0x140a4248a
140a414e6: dec rdx
140a414e9: add rdi, 0x10
140a414ed: mov qword ptr [rax + rcx], r8
140a414f1: mov dword ptr [rax + rcx + 8], r9d
140a414f6: add rcx, 0x10
140a414fa: test rdx, rdx
140a414fd: jne 0x140a414d0
140a414ff: jmp 0x140a41506
140a41501: mov eax, 8
140a41506: mov qword ptr [rbp + 0x150], r14
140a4150d: mov qword ptr [rbp + 0x158], rax
140a41514: mov qword ptr [rbp + 0x160], r14
140a4151b: movzx eax, byte ptr [rip + 0x2bf36e7]
140a41522: mov ecx, 0x18
140a41527: mov edx, 8
140a4152c: call 0x140613c10
140a41531: test rax, rax
140a41534: mov rdi, qword ptr [rbp + 0x348]
140a4153b: je 0x140a42171
140a41541: mov rcx, qword ptr [rbp + 0x160]
140a41548: mov qword ptr [rax + 0x10], rcx
140a4154c: movups xmm0, xmmword ptr [rbp + 0x150]
140a41553: movups xmmword ptr [rax], xmm0
140a41556: mov byte ptr [rbp + 0x168], 9
140a4155d: mov qword ptr [rbp + 0x158], rax
140a41564: lea rax, [rip + 0x208cb1d]
140a4156b: mov qword ptr [rbp + 0x160], rax
140a41572: mov qword ptr [rbp + 0x150], 6
140a4157d: lea rcx, [rbp + 0x2e0]
140a41584: lea r8, [rbp + 0x150]
140a4158b: mov rdx, rdi
140a4158e: call 0x1408da280
140a41593: cmp dword ptr [rbp + 0x2e0], 0xb
140a4159a: jne 0x140a42185
140a415a0: mov eax, dword ptr [rbx + 0x68]
140a415a3: cmp dword ptr [rdi + 0xf4], eax
140a415a9: je 0x140a41629
140a415ab: mov dword ptr [rdi + 0xf4], eax
140a415b1: mov esi, dword ptr [rbx + 0x68]
140a415b4: movzx eax, byte ptr [rip + 0x2bf364e]
140a415bb: mov ecx, 4
140a415c0: mov edx, 4
140a415c5: call 0x140613c10
140a415ca: test rax, rax
140a415cd: je 0x140a422f3
140a415d3: mov dword ptr [rax], esi
140a415d5: mov byte ptr [rbp + 0x168], 7
140a415dc: mov qword ptr [rbp + 0x158], rax
140a415e3: lea rax, [rip + 0x208cad6]
140a415ea: mov qword ptr [rbp + 0x160], rax
140a415f1: mov qword ptr [rbp + 0x150], 6
140a415fc: lea rcx, [rbp + 0x2e0]
140a41603: lea r8, [rbp + 0x150]
140a4160a: mov rdx, rdi
140a4160d: call 0x1408da280
140a41612: cmp dword ptr [rbp + 0x2e0], 0xb
140a41619: jne 0x140a4231b
140a4161f: mov dword ptr [rdi + 0x148], 0
140a41629: mov eax, dword ptr [rbx + 0x6c]
140a4162c: cmp dword ptr [rdi + 0xf8], eax
140a41632: je 0x140a416b2
140a41634: mov dword ptr [rdi + 0xf8], eax
140a4163a: mov esi, dword ptr [rbx + 0x6c]
140a4163d: movzx eax, byte ptr [rip + 0x2bf35c5]
140a41644: mov ecx, 4
140a41649: mov edx, 4
140a4164e: call 0x140613c10
140a41653: test rax, rax
140a41656: je 0x140a42307
140a4165c: mov dword ptr [rax], esi
140a4165e: mov byte ptr [rbp + 0x168], 8
140a41665: mov qword ptr [rbp + 0x158], rax
140a4166c: lea rax, [rip + 0x208ca4d]
140a41673: mov qword ptr [rbp + 0x160], rax
140a4167a: mov qword ptr [rbp + 0x150], 6
140a41685: lea rcx, [rbp + 0x2e0]
140a4168c: lea r8, [rbp + 0x150]
140a41693: mov rdx, rdi
140a41696: call 0x1408da280
140a4169b: cmp dword ptr [rbp + 0x2e0], 0xb
140a416a2: jne 0x140a42383
140a416a8: mov dword ptr [rdi + 0x148], 0
140a416b2: cmp byte ptr [rbp + 0x2b0], 0
140a416b9: jne 0x140a4170e
140a416bb: mov eax, dword ptr [rbp + 0x2b8]
140a416c1: mov dword ptr [rbp + 0x158], eax
140a416c7: mov eax, dword ptr [rbp + 0x134]
140a416cd: mov dword ptr [rbp + 0x15c], eax
140a416d3: mov rax, qword ptr [rbp + 0x2c8]
140a416da: mov dword ptr [rbp + 0x160], eax
140a416e0: mov qword ptr [rbp + 0x150], 7
140a416eb: lea rcx, [rbp + 0x2e0]
140a416f2: lea r8, [rbp + 0x150]
140a416f9: mov rdx, rdi
140a416fc: call 0x1408da280
140a41701: cmp dword ptr [rbp + 0x2e0], 0xb
140a41708: jne 0x140a41f62
140a4170e: cmp byte ptr [rbp + 0x60], 0
140a41712: jne 0x140a41c25
140a41718: mov rcx, qword ptr [rbp + 0xb0]
140a4171f: movzx eax, byte ptr [rcx + 0xc0]
140a41726: shl eax, 6
140a41729: lea rsi, [rcx + rax]
140a4172d: movzx ebx, byte ptr [rcx + rax + 0x28]
140a41732: cmp bl, byte ptr [rcx + 0x28]
140a41735: jne 0x140a41760
140a41737: mov rax, rcx
140a4173a: mov r8, qword ptr [rsi + 0x10]
140a4173e: cmp r8, qword ptr [rcx + 0x10]
140a41742: jne 0x140a41760
140a41744: mov rdx, qword ptr [rax + 8]
140a41748: mov rcx, qword ptr [rsi + 8]
140a4174c: call 0x142923bf0
140a41751: test eax, eax
140a41753: mov rax, qword ptr [rbp + 0xb0]
140a4175a: je 0x140a41ced
140a41760: lea rcx, [rbp + 0x150]
140a41767: mov rdx, rsi
140a4176a: call 0x140d7bd90
140a4176f: mov byte ptr [rbp + 0x178], bl
140a41775: movups xmm0, xmmword ptr [rsi + 0x18]
140a41779: movups xmmword ptr [rbp + 0x168], xmm0
140a41780: movsd xmm6, qword ptr [rsi + 0x30]
140a41785: movss xmm7, dword ptr [rsi + 0x38]
140a4178a: movzx ebx, byte ptr [rsi + 0x3c]
140a4178e: movzx esi, byte ptr [rsi + 0x3d]
140a41792: movups xmm0, xmmword ptr [rbp + 0x160]
140a41799: mov rax, qword ptr [rbp + 0x170]
140a417a0: mov qword ptr [rbp + 0x300], rax
140a417a7: movzx eax, byte ptr [rbp + 0x178]
140a417ae: mov byte ptr [rbp + 0x308], al
140a417b4: mov eax, dword ptr [rbp + 0x179]
140a417ba: mov dword ptr [rbp + 0x309], eax
140a417c0: movzx eax, word ptr [rbp + 0x17d]
140a417c7: mov word ptr [rbp + 0x30d], ax
140a417ce: movzx eax, byte ptr [rbp + 0x17f]
140a417d5: mov byte ptr [rbp + 0x30f], al
140a417db: movaps xmmword ptr [rbp + 0x2f0], xmm0
140a417e2: movups xmm0, xmmword ptr [rbp + 0x150]
140a417e9: movaps xmmword ptr [rbp + 0x2e0], xmm0
140a417f0: mov rax, qword ptr [rbp + 0x58]
140a417f4: mov ecx, dword ptr [rbp + 0xfc]
140a417fa: mov dword ptr [rax], ecx
140a417fc: lea rdi, [r12 + 0x18]
140a41801: mov rdx, qword ptr [r12 + 0x18]
140a41806: mov rax, rdx
140a41809: neg rax
140a4180c: jo 0x140a41820
140a4180e: jae 0x140a41820
140a41810: mov rcx, qword ptr [r12 + 0x20]
140a41815: mov r8d, 1
140a4181b: call 0x140613c20
140a41820: movaps xmm0, xmmword ptr [rbp + 0x2e0]
140a41827: movaps xmm1, xmmword ptr [rbp + 0x2f0]
140a4182e: mov rax, qword ptr [rbp + 0x300]
140a41835: mov qword ptr [rdi + 0x20], rax
140a41839: movzx eax, byte ptr [rbp + 0x308]
140a41840: mov byte ptr [rdi + 0x28], al
140a41843: mov eax, dword ptr [rbp + 0x309]
140a41849: mov dword ptr [rdi + 0x29], eax
140a4184c: movzx eax, word ptr [rbp + 0x30d]
140a41853: mov word ptr [rdi + 0x2d], ax
140a41857: movzx eax, byte ptr [rbp + 0x30f]
140a4185e: mov byte ptr [rdi + 0x2f], al
140a41861: movups xmmword ptr [rdi + 0x10], xmm1
140a41865: movups xmmword ptr [rdi], xmm0
140a41868: movlps qword ptr [r12 + 0x48], xmm6
140a4186e: movss dword ptr [r12 + 0x50], xmm7
140a41875: mov byte ptr [r12 + 0x54], bl
140a4187a: mov byte ptr [r12 + 0x55], sil
140a4187f: mov rdi, qword ptr [rbp + 0x348]
140a41886: movups xmm0, xmmword ptr [r12 + 0x60]
140a4188c: movaps xmmword ptr [rbp + 0x150], xmm0
140a41893: lea rdx, [rbp + 0x150]
140a4189a: mov rcx, r12
140a4189d: call 0x140b463c0
140a418a2: test rax, rax
140a418a5: je 0x140a419a9
140a418ab: mov qword ptr [rbp + 0x70], rax
140a418af: mov rax, qword ptr [rip + 0x2beaaf2]
140a418b6: mov rax, qword ptr [rax]
140a418b9: test rax, rax
140a418bc: je 0x140a419a0
140a418c2: lea rax, [rbp + 0x70]
140a418c6: mov qword ptr [rbp + 0x2e0], rax
140a418cd: lea rax, [rip + 0x30b34c]
140a418d4: mov qword ptr [rbp + 0x2e8], rax
140a418db: lea rcx, [rip + 0x208c876]
140a418e2: call 0x14067dd80
140a418e7: movups xmm0, xmmword ptr [rax]
140a418ea: mov eax, dword ptr [rax + 0x10]
140a418ed: mov qword ptr [rbp + 0x180], 1
140a418f8: lea rcx, [rip + 0x208c649]
140a418ff: mov qword ptr [rbp + 0x188], rcx
140a41906: mov qword ptr [rbp + 0x190], 0x26
140a41911: lea rdx, [rip + 0x208c830]
140a41918: mov qword ptr [rbp + 0x1a0], rdx
140a4191f: mov qword ptr [rbp + 0x1a8], 1
140a4192a: lea rdx, [rbp + 0x2e0]
140a41931: mov qword ptr [rbp + 0x1b0], rdx
140a41938: mov qword ptr [rbp + 0x1b8], 1
140a41943: mov qword ptr [rbp + 0x1c0], 0
140a4194e: mov qword ptr [rbp + 0x150], 0
140a41959: mov qword ptr [rbp + 0x158], rcx
140a41960: mov qword ptr [rbp + 0x160], 0x26
140a4196b: mov qword ptr [rbp + 0x168], 0
140a41976: movups xmmword ptr [rbp + 0x170], xmm0
140a4197d: mov dword ptr [rbp + 0x198], 1
140a41987: mov dword ptr [rbp + 0x19c], eax
140a4198d: lea rcx, [rbp + 0x347]
140a41994: lea rdx, [rbp + 0x150]
140a4199b: call 0x140d4a330
140a419a0: lea rcx, [rbp + 0x70]
140a419a4: call 0x140d4cc30
140a419a9: lea rcx, [rbp + 0x150]
140a419b0: mov edx, 1
140a419b5: call 0x1408d9a20
140a419ba: movups xmm0, xmmword ptr [rbp + 0x150]
140a419c1: movups xmm1, xmmword ptr [rbp + 0x160]
140a419c8: movups xmmword ptr [rbp + 0x70], xmm1
140a419cc: movups xmmword ptr [rbp + 0x158], xmm0
140a419d3: mov qword ptr [rbp + 0x150], 0xa
140a419de: lea rsi, [rbp + 0x2e0]
140a419e5: lea r8, [rbp + 0x150]
140a419ec: mov rcx, rsi
140a419ef: mov rdx, rdi
140a419f2: call 0x1408da280
140a419f7: cmp dword ptr [rbp + 0x2e0], 0xb
140a419fe: jne 0x140a41fca
140a41a04: mov rax, qword ptr [rbp + 0x70]
140a41a08: mov rdx, qword ptr [rbp + 0x78]
140a41a0c: cmp rax, 2
140a41a10: je 0x140a41a2b
140a41a12: cmp eax, 1
140a41a15: jne 0x140a41a3f
140a41a17: lea rcx, [rbp + 0x150]
140a41a1e: mov r9d, 0x3b9aca00
140a41a24: call 0x1416741f0
140a41a29: jmp 0x140a41a51
140a41a2b: lea rcx, [rbp + 0x150]
140a41a32: mov r9d, 0x3b9aca00
140a41a38: call 0x1416cc990
140a41a3d: jmp 0x140a41a51
140a41a3f: lea rcx, [rbp + 0x150]
140a41a46: mov r9d, 0x3b9aca00
140a41a4c: call 0x1416798b0
140a41a51: cmp byte ptr [rbp + 0x150], 1
140a41a58: je 0x140a42032
140a41a5e: mov rax, qword ptr [rbp + 0x158]
140a41a65: test rax, rax
140a41a68: je 0x140a41b73
140a41a6e: mov qword ptr [rbp + 0x138], rax
140a41a75: mov rax, qword ptr [rip + 0x2bea92c]
140a41a7c: mov rax, qword ptr [rax]
140a41a7f: test rax, rax
140a41a82: je 0x140a41b62
140a41a88: lea rax, [rbp + 0x138]
140a41a8f: mov qword ptr [rbp + 0x2e0], rax
140a41a96: lea rax, [rip + 0x30b183]
140a41a9d: mov qword ptr [rbp + 0x2e8], rax
140a41aa4: lea rcx, [rip + 0x208c755]
140a41aab: call 0x14067dd80
140a41ab0: movups xmm0, xmmword ptr [rax]
140a41ab3: mov eax, dword ptr [rax + 0x10]
140a41ab6: mov qword ptr [rbp + 0x180], 1
140a41ac1: lea rcx, [rip + 0x208c480]
140a41ac8: mov qword ptr [rbp + 0x188], rcx
140a41acf: mov qword ptr [rbp + 0x190], 0x26
140a41ada: lea rdx, [rip + 0x208c70f]
140a41ae1: mov qword ptr [rbp + 0x1a0], rdx
140a41ae8: mov qword ptr [rbp + 0x1a8], 1
140a41af3: mov qword ptr [rbp + 0x1b0], rsi
140a41afa: mov qword ptr [rbp + 0x1b8], 1
140a41b05: mov qword ptr [rbp + 0x1c0], 0
140a41b10: mov qword ptr [rbp + 0x150], 0
140a41b1b: mov qword ptr [rbp + 0x158], rcx
140a41b22: mov qword ptr [rbp + 0x160], 0x26
140a41b2d: mov qword ptr [rbp + 0x168], 0
140a41b38: movups xmmword ptr [rbp + 0x170], xmm0
140a41b3f: mov dword ptr [rbp + 0x198], 1
140a41b49: mov dword ptr [rbp + 0x19c], eax
140a41b4f: lea rcx, [rbp + 0x347]
140a41b56: lea rdx, [rbp + 0x150]
140a41b5d: call 0x140d4a330
140a41b62: lea rcx, [rbp + 0x138]
140a41b69: call 0x140d4cc30
140a41b6e: jmp 0x140a41c1c
140a41b73: add r12, 0x60
140a41b77: lea rcx, [rbp + 0x150]
140a41b7e: mov rdx, r12
140a41b81: call 0x140b46000
140a41b86: mov rax, qword ptr [rbp + 0x430]
140a41b8d: mov rcx, qword ptr [rbp + 0x150]
140a41b94: mov qword ptr [rbp + 0x348], rcx
140a41b9b: mov rcx, qword ptr [rbp + 0x158]
140a41ba2: mov qword ptr [rbp + 0x338], rcx
140a41ba9: movups xmm0, xmmword ptr [rbp + 0x160]
140a41bb0: movaps xmmword ptr [rbp + 0x2e0], xmm0
140a41bb7: mov ecx, dword ptr [rax + 0x1c]
140a41bba: mov rdx, qword ptr [rax + 0x10]
140a41bbe: mov dword ptr [rdx], ecx
140a41bc0: mov rsi, qword ptr [rax]
140a41bc3: mov rdi, qword ptr [rsi + 0x30]
140a41bc7: mov rbx, qword ptr [rsi + 0x40]
140a41bcb: cmp rdi, qword ptr [rsi + 0x20]
140a41bcf: jne 0x140a41be1
140a41bd1: lea rcx, [rsi + 0x20]
140a41bd5: lea rdx, [rip + 0x21f856c]
140a41bdc: call 0x140562ab0
140a41be1: mov rax, qword ptr [rsi + 0x28]
140a41be5: lea rcx, [rdi + rdi*4]
140a41be9: mov rdx, qword ptr [rbp + 0x348]
140a41bf0: mov qword ptr [rax + rcx*8], rdx
140a41bf4: mov rdx, qword ptr [rbp + 0x338]
140a41bfb: mov qword ptr [rax + rcx*8 + 8], rdx
140a41c00: movaps xmm0, xmmword ptr [rbp + 0x2e0]
140a41c07: movups xmmword ptr [rax + rcx*8 + 0x10], xmm0
140a41c0c: mov qword ptr [rax + rcx*8 + 0x20], rbx
140a41c11: inc rdi
140a41c14: mov qword ptr [rsi + 0x30], rdi
140a41c18: inc qword ptr [rsi + 0x40]
140a41c1c: lea rcx, [rbp + 0x70]
140a41c20: call 0x140877430
140a41c25: mov rcx, qword ptr [rbp + 0x2a0]
140a41c2c: lea rax, [rcx*2]
140a41c34: test rax, rax
140a41c37: jne 0x140a41c42
140a41c39: cmp qword ptr [rbp + 0x48], 0
140a41c3e: jne 0x140a41c63
140a41c40: jmp 0x140a41cab
140a41c42: shl rcx, 3
140a41c46: lea rdx, [rcx + rcx*4]
140a41c4a: mov r8d, 4
140a41c50: mov rcx, qword ptr [rbp + 0x2d0]
140a41c57: call 0x140613c20
140a41c5c: cmp qword ptr [rbp + 0x48], 0
140a41c61: je 0x140a41cab
140a41c63: call 0x140c18130
140a41c68: mov rcx, rax
140a41c6b: mov rax, qword ptr [rax]
140a41c6e: cmp rax, 1
140a41c72: jne 0x140a41de7
140a41c78: add rcx, 8
140a41c7c: cmp qword ptr [rcx], 0
140a41c80: jne 0x140a41e0b
140a41c86: mov qword ptr [rcx], 0xffffffffffffffff
140a41c8d: mov qword ptr [rbp + 0x348], rcx
140a41c94: add rcx, 8
140a41c98: mov rdx, qword ptr [rbp + 0x50]
140a41c9c: call 0x140c170f0
140a41ca1: mov rax, qword ptr [rbp + 0x348]
140a41ca8: inc qword ptr [rax]
140a41cab: movaps xmm6, xmmword ptr [rbp + 0x360]
140a41cb2: movaps xmm7, xmmword ptr [rbp + 0x370]
140a41cb9: movaps xmm8, xmmword ptr [rbp + 0x380]
140a41cc1: movaps xmm9, xmmword ptr [rbp + 0x390]
140a41cc9: movaps xmm10, xmmword ptr [rbp + 0x3a0]
140a41cd1: movaps xmm11, xmmword ptr [rbp + 0x3b0]
140a41cd9: add rsp, 0x448
140a41ce0: pop rbx
140a41ce1: pop rdi
140a41ce2: pop rsi
140a41ce3: pop r12
140a41ce5: pop r13
140a41ce7: pop r14
140a41ce9: pop r15
140a41ceb: pop rbp
140a41cec: ret 
140a41ced: movss xmm0, dword ptr [rsi + 0x18]
140a41cf2: ucomiss xmm0, dword ptr [rax + 0x18]
140a41cf6: jne 0x140a41760
140a41cfc: jp 0x140a41760
140a41d02: movss xmm0, dword ptr [rsi + 0x1c]
140a41d07: ucomiss xmm0, dword ptr [rax + 0x1c]
140a41d0b: jne 0x140a41760
140a41d11: jp 0x140a41760
140a41d17: movss xmm0, dword ptr [rsi + 0x20]
140a41d1c: ucomiss xmm0, dword ptr [rax + 0x20]
140a41d20: jne 0x140a41760
140a41d26: jp 0x140a41760
140a41d2c: movss xmm0, dword ptr [rsi + 0x24]
140a41d31: ucomiss xmm0, dword ptr [rax + 0x24]
140a41d35: jne 0x140a41760
140a41d3b: jp 0x140a41760
140a41d41: movss xmm0, dword ptr [rsi + 0x30]
140a41d46: ucomiss xmm0, dword ptr [rax + 0x30]
140a41d4a: jne 0x140a41760
140a41d50: jp 0x140a41760
140a41d56: movss xmm0, dword ptr [rsi + 0x34]
140a41d5b: ucomiss xmm0, dword ptr [rax + 0x34]
140a41d5f: jne 0x140a41760
140a41d65: jp 0x140a41760
140a41d6b: movss xmm0, dword ptr [rsi + 0x38]
140a41d70: ucomiss xmm0, dword ptr [rax + 0x38]
140a41d74: jne 0x140a41760
140a41d7a: jp 0x140a41760
140a41d80: movzx eax, byte ptr [rsi + 0x3c]
140a41d84: mov rcx, qword ptr [rbp + 0xb0]
140a41d8b: cmp al, byte ptr [rcx + 0x3c]
140a41d8e: mov rax, qword ptr [rbp + 0xb0]
140a41d95: jne 0x140a41760
140a41d9b: movzx eax, byte ptr [rax + 0x3d]
140a41d9f: cmp byte ptr [rsi + 0x3d], al
140a41da2: jne 0x140a41760
140a41da8: mov rax, qword ptr [rbp + 0x58]
140a41dac: mov ecx, dword ptr [rbp + 0xfc]
140a41db2: mov dword ptr [rax], ecx
140a41db4: mov rdx, qword ptr [r12 + 0x18]
140a41db9: mov rax, rdx
140a41dbc: neg rax
140a41dbf: jo 0x140a41dd3
140a41dc1: jae 0x140a41dd3
140a41dc3: mov rcx, qword ptr [r12 + 0x20]
140a41dc8: mov r8d, 1
140a41dce: call 0x140613c20
140a41dd3: movabs rax, 0x8000000000000000
140a41ddd: mov qword ptr [r12 + 0x18], rax
140a41de2: jmp 0x140a41886
140a41de7: test rax, rax
140a41dea: jne 0x140a41dff
140a41dec: xor edx, edx
140a41dee: call 0x1428f87d0
140a41df3: mov rcx, rax
140a41df6: test rax, rax
140a41df9: jne 0x140a41c7c
140a41dff: lea rcx, [rip + 0x21f8c9a]
140a41e06: call 0x1428d80d0
140a41e0b: lea rcx, [rip + 0x21f8d5e]
140a41e12: call 0x1428d9250
140a41e17: mov edx, ecx
140a41e19: mov rdi, qword ptr [rbp + 0x348]
140a41e20: mov r13, r9
140a41e23: jmp 0x140a411cc
140a41e28: lea r8, [rip + 0x21f6d19]
140a41e2f: mov rcx, r12
140a41e32: mov rdx, rsi
140a41e35: call 0x1428d8fe3
140a41e3a: jmp 0x140a4248a
140a41e3f: lea rcx, [rip + 0x222cb92]
140a41e46: call 0x1428d9a20
140a41e4b: jmp 0x140a4248a
140a41e50: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a41e57: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a41e5e: movups xmm2, xmmword ptr [rbp + 0x300]
140a41e65: movups xmm3, xmmword ptr [rbp + 0x310]
140a41e6c: movaps xmmword ptr [rbp + 0x180], xmm3
140a41e73: movaps xmmword ptr [rbp + 0x170], xmm2
140a41e7a: movaps xmmword ptr [rbp + 0x160], xmm1
140a41e81: movaps xmmword ptr [rbp + 0x150], xmm0
140a41e88: lea rax, [rip + 0x208c0a1]
140a41e8f: mov qword ptr [rsp + 0x20], rax
140a41e94: lea rcx, [rip + 0x208be55]
140a41e9b: lea r9, [rip + 0x208be2e]
140a41ea2: lea r8, [rbp + 0x150]
140a41ea9: mov edx, 0x2b
140a41eae: call 0x1428d9760
140a41eb3: jmp 0x140a4248a
140a41eb8: lea r8, [rip + 0x21f6c89]
140a41ebf: mov rcx, r12
140a41ec2: mov rdx, rsi
140a41ec5: call 0x1428d8fe3
140a41eca: jmp 0x140a4248a
140a41ecf: lea r8, [rip + 0x21f6c72]
140a41ed6: mov rcx, r15
140a41ed9: mov rdx, rsi
140a41edc: call 0x1428d8fe3
140a41ee1: jmp 0x140a4248a
140a41ee6: mov ecx, 4
140a41eeb: mov edx, 0x44
140a41ef0: call 0x1428d9000
140a41ef5: jmp 0x140a4248a
140a41efa: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a41f01: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a41f08: movups xmm2, xmmword ptr [rbp + 0x300]
140a41f0f: movups xmm3, xmmword ptr [rbp + 0x310]
140a41f16: movaps xmmword ptr [rbp + 0x180], xmm3
140a41f1d: movaps xmmword ptr [rbp + 0x170], xmm2
140a41f24: movaps xmmword ptr [rbp + 0x160], xmm1
140a41f2b: movaps xmmword ptr [rbp + 0x150], xmm0
140a41f32: lea rax, [rip + 0x208c137]
140a41f39: mov qword ptr [rsp + 0x20], rax
140a41f3e: lea rcx, [rip + 0x208bdab]
140a41f45: lea r9, [rip + 0x208bd84]
140a41f4c: lea r8, [rbp + 0x150]
140a41f53: mov edx, 0x2b
140a41f58: call 0x1428d9760
140a41f5d: jmp 0x140a4248a
140a41f62: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a41f69: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a41f70: movups xmm2, xmmword ptr [rbp + 0x300]
140a41f77: movups xmm3, xmmword ptr [rbp + 0x310]
140a41f7e: movaps xmmword ptr [rbp + 0x180], xmm3
140a41f85: movaps xmmword ptr [rbp + 0x170], xmm2
140a41f8c: movaps xmmword ptr [rbp + 0x160], xmm1
140a41f93: movaps xmmword ptr [rbp + 0x150], xmm0
140a41f9a: lea rax, [rip + 0x208c16f]
140a41fa1: mov qword ptr [rsp + 0x20], rax
140a41fa6: lea rcx, [rip + 0x208bd43]
140a41fad: lea r9, [rip + 0x208bd1c]
140a41fb4: lea r8, [rbp + 0x150]
140a41fbb: mov edx, 0x2b
140a41fc0: call 0x1428d9760
140a41fc5: jmp 0x140a4248a
140a41fca: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a41fd1: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a41fd8: movups xmm2, xmmword ptr [rbp + 0x300]
140a41fdf: movups xmm3, xmmword ptr [rbp + 0x310]
140a41fe6: movaps xmmword ptr [rbp + 0x180], xmm3
140a41fed: movaps xmmword ptr [rbp + 0x170], xmm2
140a41ff4: movaps xmmword ptr [rbp + 0x160], xmm1
140a41ffb: movaps xmmword ptr [rbp + 0x150], xmm0
140a42002: lea rax, [rip + 0x208c167]
140a42009: mov qword ptr [rsp + 0x20], rax
140a4200e: lea rcx, [rip + 0x208bcdb]
140a42015: lea r9, [rip + 0x208bcb4]
140a4201c: lea r8, [rbp + 0x150]
140a42023: mov edx, 0x2b
140a42028: call 0x1428d9760
140a4202d: jmp 0x140a4248a
140a42032: lea rax, [rip + 0x208c16f]
140a42039: mov qword ptr [rsp + 0x20], rax
140a4203e: lea rcx, [rip + 0x208c143]
140a42045: lea r9, [rip + 0x208bc64]
140a4204c: lea r8, [rbp + 0x347]
140a42053: mov edx, 0x1e
140a42058: call 0x1428d9760
140a4205d: jmp 0x140a4248a
140a42062: mov ecx, 4
140a42067: mov edx, 0x24
140a4206c: call 0x1428d9000
140a42071: jmp 0x140a4248a
140a42076: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a4207d: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a42084: movups xmm2, xmmword ptr [rbp + 0x300]
140a4208b: movups xmm3, xmmword ptr [rbp + 0x310]
140a42092: movaps xmmword ptr [rbp + 0x180], xmm3
140a42099: movaps xmmword ptr [rbp + 0x170], xmm2
140a420a0: movaps xmmword ptr [rbp + 0x160], xmm1
140a420a7: movaps xmmword ptr [rbp + 0x150], xmm0
140a420ae: lea rax, [rip + 0x208bf13]
140a420b5: mov qword ptr [rsp + 0x20], rax
140a420ba: lea rcx, [rip + 0x208bc2f]
140a420c1: lea r9, [rip + 0x208bc08]
140a420c8: lea r8, [rbp + 0x150]
140a420cf: mov edx, 0x2b
140a420d4: call 0x1428d9760
140a420d9: jmp 0x140a4248a
140a420de: mov ecx, 4
140a420e3: mov edx, 0x2c
140a420e8: call 0x1428d9000
140a420ed: jmp 0x140a4248a
140a420f2: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a420f9: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a42100: movups xmm2, xmmword ptr [rbp + 0x300]
140a42107: movups xmm3, xmmword ptr [rbp + 0x310]
140a4210e: movaps xmmword ptr [rbp + 0x180], xmm3
140a42115: movaps xmmword ptr [rbp + 0x170], xmm2
140a4211c: movaps xmmword ptr [rbp + 0x160], xmm1
140a42123: movaps xmmword ptr [rbp + 0x150], xmm0
140a4212a: lea rax, [rip + 0x208becf]
140a42131: mov qword ptr [rsp + 0x20], rax
140a42136: lea rcx, [rip + 0x208bbb3]
140a4213d: lea r9, [rip + 0x208bb8c]
140a42144: lea r8, [rbp + 0x150]
140a4214b: mov edx, 0x2b
140a42150: call 0x1428d9760
140a42155: jmp 0x140a4248a
140a4215a: lea r8, [rip + 0x21f69e7]
140a42161: mov rcx, r12
140a42164: mov rdx, rsi
140a42167: call 0x1428d8fe3
140a4216c: jmp 0x140a4248a
140a42171: mov ecx, 8
140a42176: mov edx, 0x18
140a4217b: call 0x1428d9000
140a42180: jmp 0x140a4248a
140a42185: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a4218c: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a42193: movups xmm2, xmmword ptr [rbp + 0x300]
140a4219a: movups xmm3, xmmword ptr [rbp + 0x310]
140a421a1: movaps xmmword ptr [rbp + 0x180], xmm3
140a421a8: movaps xmmword ptr [rbp + 0x170], xmm2
140a421af: movaps xmmword ptr [rbp + 0x160], xmm1
140a421b6: movaps xmmword ptr [rbp + 0x150], xmm0
140a421bd: lea rax, [rip + 0x208bee4]
140a421c4: mov qword ptr [rsp + 0x20], rax
140a421c9: lea rcx, [rip + 0x208bb20]
140a421d0: lea r9, [rip + 0x208baf9]
140a421d7: lea r8, [rbp + 0x150]
140a421de: mov edx, 0x2b
140a421e3: call 0x1428d9760
140a421e8: jmp 0x140a4248a
140a421ed: mov ecx, 8
140a421f2: mov edx, 8
140a421f7: call 0x1428d9000
140a421fc: jmp 0x140a4248a
140a42201: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a42208: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a4220f: movups xmm2, xmmword ptr [rbp + 0x300]
140a42216: movups xmm3, xmmword ptr [rbp + 0x310]
140a4221d: movaps xmmword ptr [rbp + 0x180], xmm3
140a42224: movaps xmmword ptr [rbp + 0x170], xmm2
140a4222b: movaps xmmword ptr [rbp + 0x160], xmm1
140a42232: movaps xmmword ptr [rbp + 0x150], xmm0
140a42239: lea rax, [rip + 0x208bd50]
140a42240: mov qword ptr [rsp + 0x20], rax
140a42245: lea rcx, [rip + 0x208baa4]
140a4224c: lea r9, [rip + 0x208ba7d]
140a42253: lea r8, [rbp + 0x150]
140a4225a: mov edx, 0x2b
140a4225f: call 0x1428d9760
140a42264: jmp 0x140a4248a
140a42269: mov qword ptr [rbp + 0x348], rsi
140a42270: mov qword ptr [rbp + 0x338], rbx
140a42277: mov ecx, 8
140a4227c: mov edx, 0x18
140a42281: call 0x1428d9000
140a42286: jmp 0x140a4248a
140a4228b: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a42292: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a42299: movups xmm2, xmmword ptr [rbp + 0x300]
140a422a0: movups xmm3, xmmword ptr [rbp + 0x310]
140a422a7: movaps xmmword ptr [rbp + 0x180], xmm3
140a422ae: movaps xmmword ptr [rbp + 0x170], xmm2
140a422b5: movaps xmmword ptr [rbp + 0x160], xmm1
140a422bc: movaps xmmword ptr [rbp + 0x150], xmm0
140a422c3: lea rax, [rip + 0x208bd6e]
140a422ca: mov qword ptr [rsp + 0x20], rax
140a422cf: lea rcx, [rip + 0x208ba1a]
140a422d6: lea r9, [rip + 0x208b9f3]
140a422dd: lea r8, [rbp + 0x150]
140a422e4: mov edx, 0x2b
140a422e9: call 0x1428d9760
140a422ee: jmp 0x140a4248a
140a422f3: mov ecx, 4
140a422f8: mov edx, 4
140a422fd: call 0x1428d9000
140a42302: jmp 0x140a4248a
140a42307: mov ecx, 4
140a4230c: mov edx, 4
140a42311: call 0x1428d9000
140a42316: jmp 0x140a4248a
140a4231b: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a42322: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a42329: movups xmm2, xmmword ptr [rbp + 0x300]
140a42330: movups xmm3, xmmword ptr [rbp + 0x310]
140a42337: movaps xmmword ptr [rbp + 0x180], xmm3
140a4233e: movaps xmmword ptr [rbp + 0x170], xmm2
140a42345: movaps xmmword ptr [rbp + 0x160], xmm1
140a4234c: movaps xmmword ptr [rbp + 0x150], xmm0
140a42353: lea rax, [rip + 0x208bd86]
140a4235a: mov qword ptr [rsp + 0x20], rax
140a4235f: lea rcx, [rip + 0x208b98a]
140a42366: lea r9, [rip + 0x208b963]
140a4236d: lea r8, [rbp + 0x150]
140a42374: mov edx, 0x2b
140a42379: call 0x1428d9760
140a4237e: jmp 0x140a4248a
140a42383: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a4238a: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a42391: movups xmm2, xmmword ptr [rbp + 0x300]
140a42398: movups xmm3, xmmword ptr [rbp + 0x310]
140a4239f: movaps xmmword ptr [rbp + 0x180], xmm3
140a423a6: movaps xmmword ptr [rbp + 0x170], xmm2
140a423ad: movaps xmmword ptr [rbp + 0x160], xmm1
140a423b4: movaps xmmword ptr [rbp + 0x150], xmm0
140a423bb: lea rax, [rip + 0x208bd36]
140a423c2: mov qword ptr [rsp + 0x20], rax
140a423c7: lea rcx, [rip + 0x208b922]
140a423ce: lea r9, [rip + 0x208b8fb]
140a423d5: lea r8, [rbp + 0x150]
140a423dc: mov edx, 0x2b
140a423e1: call 0x1428d9760
140a423e6: jmp 0x140a4248a
140a423eb: lea r8, [rip + 0x21f6756]
140a423f2: mov rcx, r15
140a423f5: mov rdx, r12
140a423f8: call 0x1428d8fe3
140a423fd: jmp 0x140a4248a
140a42402: lea r8, [rip + 0x21f673f]
140a42409: mov rcx, r12
140a4240c: mov rdx, rsi
140a4240f: call 0x1428d8fe3
140a42414: jmp 0x140a4248a
140a42416: mov ecx, 8
140a4241b: mov edx, 0x18
140a42420: call 0x1428d9000
140a42425: jmp 0x140a4248a
140a42427: movups xmm0, xmmword ptr [rbp + 0x2e0]
140a4242e: movups xmm1, xmmword ptr [rbp + 0x2f0]
140a42435: movups xmm2, xmmword ptr [rbp + 0x300]
140a4243c: movups xmm3, xmmword ptr [rbp + 0x310]
140a42443: movaps xmmword ptr [rbp + 0x180], xmm3
140a4244a: movaps xmmword ptr [rbp + 0x170], xmm2
140a42451: movaps xmmword ptr [rbp + 0x160], xmm1
140a42458: movaps xmmword ptr [rbp + 0x150], xmm0
140a4245f: lea rax, [rip + 0x208be7a]
140a42466: mov qword ptr [rsp + 0x20], rax
140a4246b: lea rcx, [rip + 0x208b87e]
140a42472: lea r9, [rip + 0x208b857]
140a42479: lea r8, [rbp + 0x150]
140a42480: mov edx, 0x2b
140a42485: call 0x1428d9760
140a4248a: ud2 
140a4248c: nop dword ptr [rax]
140a42490: mov qword ptr [rsp + 0x10], rdx
140a42495: push rbp
140a42496: push r15
140a42498: push r14
140a4249a: push r13
140a4249c: push r12
140a4249e: push rsi
140a4249f: push rdi
140a424a0: push rbx
140a424a1: sub rsp, 0x98
140a424a8: lea rbp, [rdx + 0x80]
140a424af: movaps xmmword ptr [rsp + 0x30], xmm11
140a424b5: movaps xmmword ptr [rsp + 0x40], xmm10
140a424bb: movaps xmmword ptr [rsp + 0x50], xmm9
140a424c1: movaps xmmword ptr [rsp + 0x60], xmm8
140a424c7: movaps xmmword ptr [rsp + 0x70], xmm7
140a424cc: movaps xmmword ptr [rsp + 0x80], xmm6
140a424d4: cmp qword ptr [rbp + 0x348], 0
140a424dc: je 0x140a424f7
140a424de: mov r8d, 1
140a424e4: mov rcx, qword ptr [rbp + 0x338]
140a424eb: mov rdx, qword ptr [rbp + 0x348]
140a424f2: call 0x140613c20
140a424f7: movaps xmm6, xmmword ptr [rsp + 0x80]
140a424ff: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42504: movaps xmm8, xmmword ptr [rsp + 0x60]
140a4250a: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42510: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42516: movaps xmm11, xmmword ptr [rsp + 0x30]
140a4251c: add rsp, 0x98
140a42523: pop rbx
140a42524: pop rdi
140a42525: pop rsi
140a42526: pop r12
140a42528: pop r13
140a4252a: pop r14
140a4252c: pop r15
140a4252e: pop rbp
140a4252f: ret 
140a42530: mov qword ptr [rsp + 0x10], rdx
140a42535: push rbp
140a42536: push r15
140a42538: push r14
140a4253a: push r13
140a4253c: push r12
140a4253e: push rsi
140a4253f: push rdi
140a42540: push rbx
140a42541: sub rsp, 0x98
140a42548: lea rbp, [rdx + 0x80]
140a4254f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42555: movaps xmmword ptr [rsp + 0x40], xmm10
140a4255b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42561: movaps xmmword ptr [rsp + 0x60], xmm8
140a42567: movaps xmmword ptr [rsp + 0x70], xmm7
140a4256c: movaps xmmword ptr [rsp + 0x80], xmm6
140a42574: mov byte ptr [rbp + 0x356], 1
140a4257b: lea rcx, [rbp + 0x150]
140a42582: call 0x14088e590
140a42587: mov byte ptr [rbp + 0x356], 1
140a4258e: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42596: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4259b: movaps xmm8, xmmword ptr [rsp + 0x60]
140a425a1: movaps xmm9, xmmword ptr [rsp + 0x50]
140a425a7: movaps xmm10, xmmword ptr [rsp + 0x40]
140a425ad: movaps xmm11, xmmword ptr [rsp + 0x30]
140a425b3: add rsp, 0x98
140a425ba: pop rbx
140a425bb: pop rdi
140a425bc: pop rsi
140a425bd: pop r12
140a425bf: pop r13
140a425c1: pop r14
140a425c3: pop r15
140a425c5: pop rbp
140a425c6: ret 
140a425c7: nop word ptr [rax + rax]
140a425d0: mov qword ptr [rsp + 0x10], rdx
140a425d5: push rbp
140a425d6: push r15
140a425d8: push r14
140a425da: push r13
140a425dc: push r12
140a425de: push rsi
140a425df: push rdi
140a425e0: push rbx
140a425e1: sub rsp, 0x98
140a425e8: lea rbp, [rdx + 0x80]
140a425ef: movaps xmmword ptr [rsp + 0x30], xmm11
140a425f5: movaps xmmword ptr [rsp + 0x40], xmm10
140a425fb: movaps xmmword ptr [rsp + 0x50], xmm9
140a42601: movaps xmmword ptr [rsp + 0x60], xmm8
140a42607: movaps xmmword ptr [rsp + 0x70], xmm7
140a4260c: movaps xmmword ptr [rsp + 0x80], xmm6
140a42614: lea rcx, [rbp + 0x150]
140a4261b: call 0x14088e590
140a42620: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42628: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4262d: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42633: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42639: movaps xmm10, xmmword ptr [rsp + 0x40]
140a4263f: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42645: add rsp, 0x98
140a4264c: pop rbx
140a4264d: pop rdi
140a4264e: pop rsi
140a4264f: pop r12
140a42651: pop r13
140a42653: pop r14
140a42655: pop r15
140a42657: pop rbp
140a42658: ret 
140a42659: nop dword ptr [rax]
140a42660: mov qword ptr [rsp + 0x10], rdx
140a42665: push rbp
140a42666: push r15
140a42668: push r14
140a4266a: push r13
140a4266c: push r12
140a4266e: push rsi
140a4266f: push rdi
140a42670: push rbx
140a42671: sub rsp, 0x98
140a42678: lea rbp, [rdx + 0x80]
140a4267f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42685: movaps xmmword ptr [rsp + 0x40], xmm10
140a4268b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42691: movaps xmmword ptr [rsp + 0x60], xmm8
140a42697: movaps xmmword ptr [rsp + 0x70], xmm7
140a4269c: movaps xmmword ptr [rsp + 0x80], xmm6
140a426a4: lea rcx, [rbp + 0x150]
140a426ab: call 0x14088e590
140a426b0: movaps xmm6, xmmword ptr [rsp + 0x80]
140a426b8: movaps xmm7, xmmword ptr [rsp + 0x70]
140a426bd: movaps xmm8, xmmword ptr [rsp + 0x60]
140a426c3: movaps xmm9, xmmword ptr [rsp + 0x50]
140a426c9: movaps xmm10, xmmword ptr [rsp + 0x40]
140a426cf: movaps xmm11, xmmword ptr [rsp + 0x30]
140a426d5: add rsp, 0x98
140a426dc: pop rbx
140a426dd: pop rdi
140a426de: pop rsi
140a426df: pop r12
140a426e1: pop r13
140a426e3: pop r14
140a426e5: pop r15
140a426e7: pop rbp
140a426e8: ret 
140a426e9: nop dword ptr [rax]
140a426f0: mov qword ptr [rsp + 0x10], rdx
140a426f5: push rbp
140a426f6: push r15
140a426f8: push r14
140a426fa: push r13
140a426fc: push r12
140a426fe: push rsi
140a426ff: push rdi
140a42700: push rbx
140a42701: sub rsp, 0x98
140a42708: lea rbp, [rdx + 0x80]
140a4270f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42715: movaps xmmword ptr [rsp + 0x40], xmm10
140a4271b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42721: movaps xmmword ptr [rsp + 0x60], xmm8
140a42727: movaps xmmword ptr [rsp + 0x70], xmm7
140a4272c: movaps xmmword ptr [rsp + 0x80], xmm6
140a42734: lea rcx, [rbp + 0x150]
140a4273b: call 0x14088e590
140a42740: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42748: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4274d: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42753: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42759: movaps xmm10, xmmword ptr [rsp + 0x40]
140a4275f: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42765: add rsp, 0x98
140a4276c: pop rbx
140a4276d: pop rdi
140a4276e: pop rsi
140a4276f: pop r12
140a42771: pop r13
140a42773: pop r14
140a42775: pop r15
140a42777: pop rbp
140a42778: ret 
140a42779: nop dword ptr [rax]
140a42780: mov qword ptr [rsp + 0x10], rdx
140a42785: push rbp
140a42786: push r15
140a42788: push r14
140a4278a: push r13
140a4278c: push r12
140a4278e: push rsi
140a4278f: push rdi
140a42790: push rbx
140a42791: sub rsp, 0x98
140a42798: lea rbp, [rdx + 0x80]
140a4279f: movaps xmmword ptr [rsp + 0x30], xmm11
140a427a5: movaps xmmword ptr [rsp + 0x40], xmm10
140a427ab: movaps xmmword ptr [rsp + 0x50], xmm9
140a427b1: movaps xmmword ptr [rsp + 0x60], xmm8
140a427b7: movaps xmmword ptr [rsp + 0x70], xmm7
140a427bc: movaps xmmword ptr [rsp + 0x80], xmm6
140a427c4: mov rax, qword ptr [rbp + 0x348]
140a427cb: lea rax, [rax*2]
140a427d3: test rax, rax
140a427d6: je 0x140a427f9
140a427d8: mov rax, qword ptr [rbp + 0x348]
140a427df: shl rax, 3
140a427e3: lea rdx, [rax + rax*4]
140a427e7: mov r8d, 4
140a427ed: mov rcx, qword ptr [rbp + 0x338]
140a427f4: call 0x140613c20
140a427f9: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42801: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42806: movaps xmm8, xmmword ptr [rsp + 0x60]
140a4280c: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42812: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42818: movaps xmm11, xmmword ptr [rsp + 0x30]
140a4281e: add rsp, 0x98
140a42825: pop rbx
140a42826: pop rdi
140a42827: pop rsi
140a42828: pop r12
140a4282a: pop r13
140a4282c: pop r14
140a4282e: pop r15
140a42830: pop rbp
140a42831: ret 
140a42832: nop word ptr cs:[rax + rax]
140a42840: mov qword ptr [rsp + 0x10], rdx
140a42845: push rbp
140a42846: push r15
140a42848: push r14
140a4284a: push r13
140a4284c: push r12
140a4284e: push rsi
140a4284f: push rdi
140a42850: push rbx
140a42851: sub rsp, 0x98
140a42858: lea rbp, [rdx + 0x80]
140a4285f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42865: movaps xmmword ptr [rsp + 0x40], xmm10
140a4286b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42871: movaps xmmword ptr [rsp + 0x60], xmm8
140a42877: movaps xmmword ptr [rsp + 0x70], xmm7
140a4287c: movaps xmmword ptr [rsp + 0x80], xmm6
140a42884: mov rcx, qword ptr [rbp + 0x348]
140a4288b: mov rax, qword ptr [rbp + 0x338]
140a42892: mov qword ptr [rcx + 0x28], rax
140a42896: mov rdx, qword ptr [rbp + 0x68]
140a4289a: mov qword ptr [rcx + 0x30], rdx
140a4289e: mov qword ptr [rcx + 0x38], rax
140a428a2: mov rax, qword ptr [rbp + 0x2d8]
140a428a9: mov qword ptr [rcx + 0x40], rax
140a428ad: mov rax, qword ptr [rbp + 0x148]
140a428b4: mov qword ptr [rcx + 0x48], rax
140a428b8: mov rax, qword ptr [rbp + 0x328]
140a428bf: mov qword ptr [rcx + 0x50], rax
140a428c3: mov rax, qword ptr [rbp + 0x290]
140a428ca: mov qword ptr [rcx + 0x58], rax
140a428ce: mov eax, dword ptr [rbp + 0x124]
140a428d4: mov dword ptr [rcx + 0x60], eax
140a428d7: mov eax, dword ptr [rbp + 0x120]
140a428dd: mov dword ptr [rcx + 0x64], eax
140a428e0: mov eax, dword ptr [rbp + 0x130]
140a428e6: mov dword ptr [rcx + 0x68], eax
140a428e9: mov eax, dword ptr [rbp + 0x12c]
140a428ef: mov dword ptr [rcx + 0x6c], eax
140a428f2: mov eax, dword ptr [rbp + 0x128]
140a428f8: mov dword ptr [rcx + 0x70], eax
140a428fb: movzx eax, byte ptr [rbp + 0x337]
140a42902: mov byte ptr [rcx + 0x74], al
140a42905: mov eax, dword ptr [rbp + 0x118]
140a4290b: mov dword ptr [rcx + 0x78], eax
140a4290e: mov eax, dword ptr [rbp + 0x110]
140a42914: mov dword ptr [rcx + 0x7c], eax
140a42917: mov eax, dword ptr [rbp + 0x108]
140a4291d: mov dword ptr [rcx + 0x80], eax
140a42923: mov eax, dword ptr [rbp + 0x114]
140a42929: mov dword ptr [rcx + 0x84], eax
140a4292f: mov eax, dword ptr [rbp + 0x10c]
140a42935: mov dword ptr [rcx + 0x88], eax
140a4293b: mov eax, dword ptr [rbp + 0x104]
140a42941: mov dword ptr [rcx + 0x8c], eax
140a42947: mov eax, dword ptr [rbp + 0x140]
140a4294d: mov dword ptr [rcx + 0x90], eax
140a42953: mov eax, dword ptr [rbp + 0xf0]
140a42959: mov dword ptr [rcx + 0x94], eax
140a4295f: mov eax, dword ptr [rbp + 0x11c]
140a42965: mov dword ptr [rcx + 0x98], eax
140a4296b: movss xmm0, dword ptr [rbp + 0x100]
140a42973: movss dword ptr [rcx + 0x9c], xmm0
140a4297b: movzx eax, byte ptr [rbp + 0xe0]
140a42982: mov byte ptr [rcx + 0xa0], al
140a42988: movzx eax, byte ptr [rbp + 0xe8]
140a4298f: mov byte ptr [rcx + 0xa1], al
140a42995: movzx eax, byte ptr [rbp + 0xd8]
140a4299c: mov byte ptr [rcx + 0xa2], al
140a429a2: mov eax, dword ptr [rbp + 0x2a8]
140a429a8: mov dword ptr [rcx + 0xa4], eax
140a429ae: movaps xmm0, xmmword ptr [rbp + 0x30]
140a429b2: movups xmmword ptr [rcx + 0xa8], xmm0
140a429b9: movaps xmm0, xmmword ptr [rbp + 0x20]
140a429bd: movups xmmword ptr [rcx + 0xb8], xmm0
140a429c4: movss xmm0, dword ptr [rbp + 0xc0]
140a429cc: movss dword ptr [rcx + 0xc8], xmm0
140a429d4: movzx eax, word ptr [rbp + 0xb8]
140a429db: mov word ptr [rcx + 0xcc], ax
140a429e2: movaps xmm0, xmmword ptr [rbp + 0x10]
140a429e6: movlps qword ptr [rcx + 0xd0], xmm0
140a429ed: lea rdx, [rbp + 0x90]
140a429f4: mov eax, dword ptr [rdx + 8]
140a429f7: mov dword ptr [rcx + 0xe0], eax
140a429fd: mov rax, qword ptr [rdx]
140a42a00: mov qword ptr [rcx + 0xd8], rax
140a42a07: movaps xmm0, xmmword ptr [rbp]
140a42a0b: movups xmmword ptr [rcx + 0xe4], xmm0
140a42a12: mov eax, dword ptr [rbp + 0xd0]
140a42a18: mov dword ptr [rcx + 0xf4], eax
140a42a1e: mov eax, dword ptr [rbp + 0xc8]
140a42a24: mov dword ptr [rcx + 0xf8], eax
140a42a2a: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42a32: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42a37: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42a3d: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42a43: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42a49: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42a4f: add rsp, 0x98
140a42a56: pop rbx
140a42a57: pop rdi
140a42a58: pop rsi
140a42a59: pop r12
140a42a5b: pop r13
140a42a5d: pop r14
140a42a5f: pop r15
140a42a61: pop rbp
140a42a62: ret 
140a42a63: nop word ptr cs:[rax + rax]
140a42a70: mov qword ptr [rsp + 0x10], rdx
140a42a75: push rbp
140a42a76: push r15
140a42a78: push r14
140a42a7a: push r13
140a42a7c: push r12
140a42a7e: push rsi
140a42a7f: push rdi
140a42a80: push rbx
140a42a81: sub rsp, 0x98
140a42a88: lea rbp, [rdx + 0x80]
140a42a8f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42a95: movaps xmmword ptr [rsp + 0x40], xmm10
140a42a9b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42aa1: movaps xmmword ptr [rsp + 0x60], xmm8
140a42aa7: movaps xmmword ptr [rsp + 0x70], xmm7
140a42aac: movaps xmmword ptr [rsp + 0x80], xmm6
140a42ab4: lea rcx, [rbp + 0x150]
140a42abb: call 0x14088e590
140a42ac0: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42ac8: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42acd: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42ad3: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42ad9: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42adf: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42ae5: add rsp, 0x98
140a42aec: pop rbx
140a42aed: pop rdi
140a42aee: pop rsi
140a42aef: pop r12
140a42af1: pop r13
140a42af3: pop r14
140a42af5: pop r15
140a42af7: pop rbp
140a42af8: ret 
140a42af9: nop dword ptr [rax]
140a42b00: mov qword ptr [rsp + 0x10], rdx
140a42b05: push rbp
140a42b06: push r15
140a42b08: push r14
140a42b0a: push r13
140a42b0c: push r12
140a42b0e: push rsi
140a42b0f: push rdi
140a42b10: push rbx
140a42b11: sub rsp, 0x98
140a42b18: lea rbp, [rdx + 0x80]
140a42b1f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42b25: movaps xmmword ptr [rsp + 0x40], xmm10
140a42b2b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42b31: movaps xmmword ptr [rsp + 0x60], xmm8
140a42b37: movaps xmmword ptr [rsp + 0x70], xmm7
140a42b3c: movaps xmmword ptr [rsp + 0x80], xmm6
140a42b44: lea rcx, [rbp + 0x150]
140a42b4b: call 0x14088e590
140a42b50: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42b58: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42b5d: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42b63: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42b69: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42b6f: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42b75: add rsp, 0x98
140a42b7c: pop rbx
140a42b7d: pop rdi
140a42b7e: pop rsi
140a42b7f: pop r12
140a42b81: pop r13
140a42b83: pop r14
140a42b85: pop r15
140a42b87: pop rbp
140a42b88: ret 
140a42b89: nop dword ptr [rax]
140a42b90: mov qword ptr [rsp + 0x10], rdx
140a42b95: push rbp
140a42b96: push r15
140a42b98: push r14
140a42b9a: push r13
140a42b9c: push r12
140a42b9e: push rsi
140a42b9f: push rdi
140a42ba0: push rbx
140a42ba1: sub rsp, 0x98
140a42ba8: lea rbp, [rdx + 0x80]
140a42baf: movaps xmmword ptr [rsp + 0x30], xmm11
140a42bb5: movaps xmmword ptr [rsp + 0x40], xmm10
140a42bbb: movaps xmmword ptr [rsp + 0x50], xmm9
140a42bc1: movaps xmmword ptr [rsp + 0x60], xmm8
140a42bc7: movaps xmmword ptr [rsp + 0x70], xmm7
140a42bcc: movaps xmmword ptr [rsp + 0x80], xmm6
140a42bd4: lea rcx, [rbp + 0x150]
140a42bdb: call 0x140600290
140a42be0: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42be8: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42bed: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42bf3: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42bf9: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42bff: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42c05: add rsp, 0x98
140a42c0c: pop rbx
140a42c0d: pop rdi
140a42c0e: pop rsi
140a42c0f: pop r12
140a42c11: pop r13
140a42c13: pop r14
140a42c15: pop r15
140a42c17: pop rbp
140a42c18: ret 
140a42c19: nop dword ptr [rax]
140a42c20: mov qword ptr [rsp + 0x10], rdx
140a42c25: push rbp
140a42c26: push r15
140a42c28: push r14
140a42c2a: push r13
140a42c2c: push r12
140a42c2e: push rsi
140a42c2f: push rdi
140a42c30: push rbx
140a42c31: sub rsp, 0x98
140a42c38: lea rbp, [rdx + 0x80]
140a42c3f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42c45: movaps xmmword ptr [rsp + 0x40], xmm10
140a42c4b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42c51: movaps xmmword ptr [rsp + 0x60], xmm8
140a42c57: movaps xmmword ptr [rsp + 0x70], xmm7
140a42c5c: movaps xmmword ptr [rsp + 0x80], xmm6
140a42c64: mov rax, qword ptr [rbp + 0x348]
140a42c6b: mov rcx, qword ptr [rbp + 0x328]
140a42c72: mov qword ptr [rax + 0x28], rcx
140a42c76: mov rdx, qword ptr [rbp + 0x338]
140a42c7d: mov qword ptr [rax + 0x30], rdx
140a42c81: mov qword ptr [rax + 0x38], rcx
140a42c85: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42c8d: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42c92: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42c98: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42c9e: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42ca4: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42caa: add rsp, 0x98
140a42cb1: pop rbx
140a42cb2: pop rdi
140a42cb3: pop rsi
140a42cb4: pop r12
140a42cb6: pop r13
140a42cb8: pop r14
140a42cba: pop r15
140a42cbc: pop rbp
140a42cbd: ret 
140a42cbe: nop 
140a42cc0: mov qword ptr [rsp + 0x10], rdx
140a42cc5: push rbp
140a42cc6: push r15
140a42cc8: push r14
140a42cca: push r13
140a42ccc: push r12
140a42cce: push rsi
140a42ccf: push rdi
140a42cd0: push rbx
140a42cd1: sub rsp, 0x98
140a42cd8: lea rbp, [rdx + 0x80]
140a42cdf: movaps xmmword ptr [rsp + 0x30], xmm11
140a42ce5: movaps xmmword ptr [rsp + 0x40], xmm10
140a42ceb: movaps xmmword ptr [rsp + 0x50], xmm9
140a42cf1: movaps xmmword ptr [rsp + 0x60], xmm8
140a42cf7: movaps xmmword ptr [rsp + 0x70], xmm7
140a42cfc: movaps xmmword ptr [rsp + 0x80], xmm6
140a42d04: lea rcx, [rbp + 0x138]
140a42d0b: call 0x140d4cc30
140a42d10: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42d18: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42d1d: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42d23: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42d29: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42d2f: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42d35: add rsp, 0x98
140a42d3c: pop rbx
140a42d3d: pop rdi
140a42d3e: pop rsi
140a42d3f: pop r12
140a42d41: pop r13
140a42d43: pop r14
140a42d45: pop r15
140a42d47: pop rbp
140a42d48: ret 
140a42d49: nop dword ptr [rax]
140a42d50: mov qword ptr [rsp + 0x10], rdx
140a42d55: push rbp
140a42d56: push r15
140a42d58: push r14
140a42d5a: push r13
140a42d5c: push r12
140a42d5e: push rsi
140a42d5f: push rdi
140a42d60: push rbx
140a42d61: sub rsp, 0x98
140a42d68: lea rbp, [rdx + 0x80]
140a42d6f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42d75: movaps xmmword ptr [rsp + 0x40], xmm10
140a42d7b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42d81: movaps xmmword ptr [rsp + 0x60], xmm8
140a42d87: movaps xmmword ptr [rsp + 0x70], xmm7
140a42d8c: movaps xmmword ptr [rsp + 0x80], xmm6
140a42d94: lea rcx, [rbp + 0x70]
140a42d98: call 0x140d4cc30
140a42d9d: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42da5: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42daa: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42db0: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42db6: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42dbc: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42dc2: add rsp, 0x98
140a42dc9: pop rbx
140a42dca: pop rdi
140a42dcb: pop rsi
140a42dcc: pop r12
140a42dce: pop r13
140a42dd0: pop r14
140a42dd2: pop r15
140a42dd4: pop rbp
140a42dd5: ret 
140a42dd6: nop word ptr cs:[rax + rax]
140a42de0: mov qword ptr [rsp + 0x10], rdx
140a42de5: push rbp
140a42de6: push r15
140a42de8: push r14
140a42dea: push r13
140a42dec: push r12
140a42dee: push rsi
140a42def: push rdi
140a42df0: push rbx
140a42df1: sub rsp, 0x98
140a42df8: lea rbp, [rdx + 0x80]
140a42dff: movaps xmmword ptr [rsp + 0x30], xmm11
140a42e05: movaps xmmword ptr [rsp + 0x40], xmm10
140a42e0b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42e11: movaps xmmword ptr [rsp + 0x60], xmm8
140a42e17: movaps xmmword ptr [rsp + 0x70], xmm7
140a42e1c: movaps xmmword ptr [rsp + 0x80], xmm6
140a42e24: lea rcx, [rbp + 0x150]
140a42e2b: call 0x14088e590
140a42e30: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42e38: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42e3d: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42e43: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42e49: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42e4f: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42e55: add rsp, 0x98
140a42e5c: pop rbx
140a42e5d: pop rdi
140a42e5e: pop rsi
140a42e5f: pop r12
140a42e61: pop r13
140a42e63: pop r14
140a42e65: pop r15
140a42e67: pop rbp
140a42e68: ret 
140a42e69: nop dword ptr [rax]
140a42e70: mov qword ptr [rsp + 0x10], rdx
140a42e75: push rbp
140a42e76: push r15
140a42e78: push r14
140a42e7a: push r13
140a42e7c: push r12
140a42e7e: push rsi
140a42e7f: push rdi
140a42e80: push rbx
140a42e81: sub rsp, 0x98
140a42e88: lea rbp, [rdx + 0x80]
140a42e8f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42e95: movaps xmmword ptr [rsp + 0x40], xmm10
140a42e9b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42ea1: movaps xmmword ptr [rsp + 0x60], xmm8
140a42ea7: movaps xmmword ptr [rsp + 0x70], xmm7
140a42eac: movaps xmmword ptr [rsp + 0x80], xmm6
140a42eb4: lea rcx, [rbp + 0x150]
140a42ebb: call 0x14088e590
140a42ec0: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42ec8: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42ecd: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42ed3: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42ed9: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42edf: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42ee5: add rsp, 0x98
140a42eec: pop rbx
140a42eed: pop rdi
140a42eee: pop rsi
140a42eef: pop r12
140a42ef1: pop r13
140a42ef3: pop r14
140a42ef5: pop r15
140a42ef7: pop rbp
140a42ef8: ret 
140a42ef9: nop dword ptr [rax]
140a42f00: mov qword ptr [rsp + 0x10], rdx
140a42f05: push rbp
140a42f06: push r15
140a42f08: push r14
140a42f0a: push r13
140a42f0c: push r12
140a42f0e: push rsi
140a42f0f: push rdi
140a42f10: push rbx
140a42f11: sub rsp, 0x98
140a42f18: lea rbp, [rdx + 0x80]
140a42f1f: movaps xmmword ptr [rsp + 0x30], xmm11
140a42f25: movaps xmmword ptr [rsp + 0x40], xmm10
140a42f2b: movaps xmmword ptr [rsp + 0x50], xmm9
140a42f31: movaps xmmword ptr [rsp + 0x60], xmm8
140a42f37: movaps xmmword ptr [rsp + 0x70], xmm7
140a42f3c: movaps xmmword ptr [rsp + 0x80], xmm6
140a42f44: lea rcx, [rbp + 0x150]
140a42f4b: call 0x14088e590
140a42f50: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42f58: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42f5d: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42f63: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42f69: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42f6f: movaps xmm11, xmmword ptr [rsp + 0x30]
140a42f75: add rsp, 0x98
140a42f7c: pop rbx
140a42f7d: pop rdi
140a42f7e: pop rsi
140a42f7f: pop r12
140a42f81: pop r13
140a42f83: pop r14
140a42f85: pop r15
140a42f87: pop rbp
140a42f88: ret 
140a42f89: nop dword ptr [rax]
140a42f90: mov qword ptr [rsp + 0x10], rdx
140a42f95: push rbp
140a42f96: push r15
140a42f98: push r14
140a42f9a: push r13
140a42f9c: push r12
140a42f9e: push rsi
140a42f9f: push rdi
140a42fa0: push rbx
140a42fa1: sub rsp, 0x98
140a42fa8: lea rbp, [rdx + 0x80]
140a42faf: movaps xmmword ptr [rsp + 0x30], xmm11
140a42fb5: movaps xmmword ptr [rsp + 0x40], xmm10
140a42fbb: movaps xmmword ptr [rsp + 0x50], xmm9
140a42fc1: movaps xmmword ptr [rsp + 0x60], xmm8
140a42fc7: movaps xmmword ptr [rsp + 0x70], xmm7
140a42fcc: movaps xmmword ptr [rsp + 0x80], xmm6
140a42fd4: lea rcx, [rbp + 0x150]
140a42fdb: call 0x14088e590
140a42fe0: movaps xmm6, xmmword ptr [rsp + 0x80]
140a42fe8: movaps xmm7, xmmword ptr [rsp + 0x70]
140a42fed: movaps xmm8, xmmword ptr [rsp + 0x60]
140a42ff3: movaps xmm9, xmmword ptr [rsp + 0x50]
140a42ff9: movaps xmm10, xmmword ptr [rsp + 0x40]
140a42fff: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43005: add rsp, 0x98
140a4300c: pop rbx
140a4300d: pop rdi
140a4300e: pop rsi
140a4300f: pop r12
140a43011: pop r13
140a43013: pop r14
140a43015: pop r15
140a43017: pop rbp
140a43018: ret 
140a43019: nop dword ptr [rax]
140a43020: mov qword ptr [rsp + 0x10], rdx
140a43025: push rbp
140a43026: push r15
140a43028: push r14
140a4302a: push r13
140a4302c: push r12
140a4302e: push rsi
140a4302f: push rdi
140a43030: push rbx
140a43031: sub rsp, 0x98
140a43038: lea rbp, [rdx + 0x80]
140a4303f: movaps xmmword ptr [rsp + 0x30], xmm11
140a43045: movaps xmmword ptr [rsp + 0x40], xmm10
140a4304b: movaps xmmword ptr [rsp + 0x50], xmm9
140a43051: movaps xmmword ptr [rsp + 0x60], xmm8
140a43057: movaps xmmword ptr [rsp + 0x70], xmm7
140a4305c: movaps xmmword ptr [rsp + 0x80], xmm6
140a43064: lea rcx, [rbp + 0x150]
140a4306b: call 0x14088e590
140a43070: movaps xmm6, xmmword ptr [rsp + 0x80]
140a43078: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4307d: movaps xmm8, xmmword ptr [rsp + 0x60]
140a43083: movaps xmm9, xmmword ptr [rsp + 0x50]
140a43089: movaps xmm10, xmmword ptr [rsp + 0x40]
140a4308f: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43095: add rsp, 0x98
140a4309c: pop rbx
140a4309d: pop rdi
140a4309e: pop rsi
140a4309f: pop r12
140a430a1: pop r13
140a430a3: pop r14
140a430a5: pop r15
140a430a7: pop rbp
140a430a8: ret 
140a430a9: nop dword ptr [rax]
140a430b0: mov qword ptr [rsp + 0x10], rdx
140a430b5: push rbp
140a430b6: push r15
140a430b8: push r14
140a430ba: push r13
140a430bc: push r12
140a430be: push rsi
140a430bf: push rdi
140a430c0: push rbx
140a430c1: sub rsp, 0x98
140a430c8: lea rbp, [rdx + 0x80]
140a430cf: movaps xmmword ptr [rsp + 0x30], xmm11
140a430d5: movaps xmmword ptr [rsp + 0x40], xmm10
140a430db: movaps xmmword ptr [rsp + 0x50], xmm9
140a430e1: movaps xmmword ptr [rsp + 0x60], xmm8
140a430e7: movaps xmmword ptr [rsp + 0x70], xmm7
140a430ec: movaps xmmword ptr [rsp + 0x80], xmm6
140a430f4: mov rax, qword ptr [rbp + 0x140]
140a430fb: inc qword ptr [rax]
140a430fe: movaps xmm6, xmmword ptr [rsp + 0x80]
140a43106: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4310b: movaps xmm8, xmmword ptr [rsp + 0x60]
140a43111: movaps xmm9, xmmword ptr [rsp + 0x50]
140a43117: movaps xmm10, xmmword ptr [rsp + 0x40]
140a4311d: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43123: add rsp, 0x98
140a4312a: pop rbx
140a4312b: pop rdi
140a4312c: pop rsi
140a4312d: pop r12
140a4312f: pop r13
140a43131: pop r14
140a43133: pop r15
140a43135: pop rbp
140a43136: ret 
140a43137: nop word ptr [rax + rax]
140a43140: mov qword ptr [rsp + 0x10], rdx
140a43145: push rbp
140a43146: push r15
140a43148: push r14
140a4314a: push r13
140a4314c: push r12
140a4314e: push rsi
140a4314f: push rdi
140a43150: push rbx
140a43151: sub rsp, 0x98
140a43158: lea rbp, [rdx + 0x80]
140a4315f: movaps xmmword ptr [rsp + 0x30], xmm11
140a43165: movaps xmmword ptr [rsp + 0x40], xmm10
140a4316b: movaps xmmword ptr [rsp + 0x50], xmm9
140a43171: movaps xmmword ptr [rsp + 0x60], xmm8
140a43177: movaps xmmword ptr [rsp + 0x70], xmm7
140a4317c: movaps xmmword ptr [rsp + 0x80], xmm6
140a43184: cmp byte ptr [rbp + 0x356], 0
140a4318b: je 0x140a431df
140a4318d: mov rax, qword ptr [rbp + 0x2a8]
140a43194: lock dec qword ptr [rax]
140a43198: jne 0x140a431a6
140a4319a: lea rcx, [rbp + 0x138]
140a431a1: call 0x1402e5d80
140a431a6: movaps xmm6, xmmword ptr [rsp + 0x80]
140a431ae: movaps xmm7, xmmword ptr [rsp + 0x70]
140a431b3: movaps xmm8, xmmword ptr [rsp + 0x60]
140a431b9: movaps xmm9, xmmword ptr [rsp + 0x50]
140a431bf: movaps xmm10, xmmword ptr [rsp + 0x40]
140a431c5: movaps xmm11, xmmword ptr [rsp + 0x30]
140a431cb: add rsp, 0x98
140a431d2: pop rbx
140a431d3: pop rdi
140a431d4: pop rsi
140a431d5: pop r12
140a431d7: pop r13
140a431d9: pop r14
140a431db: pop r15
140a431dd: pop rbp
140a431de: ret 
140a431df: movaps xmm6, xmmword ptr [rsp + 0x80]
140a431e7: movaps xmm7, xmmword ptr [rsp + 0x70]
140a431ec: movaps xmm8, xmmword ptr [rsp + 0x60]
140a431f2: movaps xmm9, xmmword ptr [rsp + 0x50]
140a431f8: movaps xmm10, xmmword ptr [rsp + 0x40]
140a431fe: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43204: add rsp, 0x98
140a4320b: pop rbx
140a4320c: pop rdi
140a4320d: pop rsi
140a4320e: pop r12
140a43210: pop r13
140a43212: pop r14
140a43214: pop r15
140a43216: pop rbp
140a43217: ret 
140a43218: nop dword ptr [rax + rax]
140a43220: mov qword ptr [rsp + 0x10], rdx
140a43225: push rbp
140a43226: push r15
140a43228: push r14
140a4322a: push r13
140a4322c: push r12
140a4322e: push rsi
140a4322f: push rdi
140a43230: push rbx
140a43231: sub rsp, 0x98
140a43238: lea rbp, [rdx + 0x80]
140a4323f: movaps xmmword ptr [rsp + 0x30], xmm11
140a43245: movaps xmmword ptr [rsp + 0x40], xmm10
140a4324b: movaps xmmword ptr [rsp + 0x50], xmm9
140a43251: movaps xmmword ptr [rsp + 0x60], xmm8
140a43257: movaps xmmword ptr [rsp + 0x70], xmm7
140a4325c: movaps xmmword ptr [rsp + 0x80], xmm6
140a43264: mov rax, qword ptr [rbp + 0x348]
140a4326b: inc qword ptr [rax]
140a4326e: movaps xmm6, xmmword ptr [rsp + 0x80]
140a43276: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4327b: movaps xmm8, xmmword ptr [rsp + 0x60]
140a43281: movaps xmm9, xmmword ptr [rsp + 0x50]
140a43287: movaps xmm10, xmmword ptr [rsp + 0x40]
140a4328d: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43293: add rsp, 0x98
140a4329a: pop rbx
140a4329b: pop rdi
140a4329c: pop rsi
140a4329d: pop r12
140a4329f: pop r13
140a432a1: pop r14
140a432a3: pop r15
140a432a5: pop rbp
140a432a6: ret 
140a432a7: nop word ptr [rax + rax]
140a432b0: mov qword ptr [rsp + 0x10], rdx
140a432b5: push rbp
140a432b6: push r15
140a432b8: push r14
140a432ba: push r13
140a432bc: push r12
140a432be: push rsi
140a432bf: push rdi
140a432c0: push rbx
140a432c1: sub rsp, 0x98
140a432c8: lea rbp, [rdx + 0x80]
140a432cf: movaps xmmword ptr [rsp + 0x30], xmm11
140a432d5: movaps xmmword ptr [rsp + 0x40], xmm10
140a432db: movaps xmmword ptr [rsp + 0x50], xmm9
140a432e1: movaps xmmword ptr [rsp + 0x60], xmm8
140a432e7: movaps xmmword ptr [rsp + 0x70], xmm7
140a432ec: movaps xmmword ptr [rsp + 0x80], xmm6
140a432f4: cmp qword ptr [rbp + 0x2a8], 0
140a432fc: jne 0x140a4334a
140a432fe: mov rcx, qword ptr [rbp + 0x290]
140a43305: call 0x140a3e7a0
140a4330a: mov byte ptr [rbp + 0x357], 1
140a43311: movaps xmm6, xmmword ptr [rsp + 0x80]
140a43319: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4331e: movaps xmm8, xmmword ptr [rsp + 0x60]
140a43324: movaps xmm9, xmmword ptr [rsp + 0x50]
140a4332a: movaps xmm10, xmmword ptr [rsp + 0x40]
140a43330: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43336: add rsp, 0x98
140a4333d: pop rbx
140a4333e: pop rdi
140a4333f: pop rsi
140a43340: pop r12
140a43342: pop r13
140a43344: pop r14
140a43346: pop r15
140a43348: pop rbp
140a43349: ret 
140a4334a: mov byte ptr [rbp + 0x357], 1
140a43351: movaps xmm6, xmmword ptr [rsp + 0x80]
140a43359: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4335e: movaps xmm8, xmmword ptr [rsp + 0x60]
140a43364: movaps xmm9, xmmword ptr [rsp + 0x50]
140a4336a: movaps xmm10, xmmword ptr [rsp + 0x40]
140a43370: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43376: add rsp, 0x98
140a4337d: pop rbx
140a4337e: pop rdi
140a4337f: pop rsi
140a43380: pop r12
140a43382: pop r13
140a43384: pop r14
140a43386: pop r15
140a43388: pop rbp
140a43389: ret 
140a4338a: nop word ptr [rax + rax]
140a43390: mov qword ptr [rsp + 0x10], rdx
140a43395: push rbp
140a43396: push r15
140a43398: push r14
140a4339a: push r13
140a4339c: push r12
140a4339e: push rsi
140a4339f: push rdi
140a433a0: push rbx
140a433a1: sub rsp, 0x98
140a433a8: lea rbp, [rdx + 0x80]
140a433af: movaps xmmword ptr [rsp + 0x30], xmm11
140a433b5: movaps xmmword ptr [rsp + 0x40], xmm10
140a433bb: movaps xmmword ptr [rsp + 0x50], xmm9
140a433c1: movaps xmmword ptr [rsp + 0x60], xmm8
140a433c7: movaps xmmword ptr [rsp + 0x70], xmm7
140a433cc: movaps xmmword ptr [rsp + 0x80], xmm6
140a433d4: lea rcx, [rbp + 0x150]
140a433db: call 0x14088e590
140a433e0: movaps xmm6, xmmword ptr [rsp + 0x80]
140a433e8: movaps xmm7, xmmword ptr [rsp + 0x70]
140a433ed: movaps xmm8, xmmword ptr [rsp + 0x60]
140a433f3: movaps xmm9, xmmword ptr [rsp + 0x50]
140a433f9: movaps xmm10, xmmword ptr [rsp + 0x40]
140a433ff: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43405: add rsp, 0x98
140a4340c: pop rbx
140a4340d: pop rdi
140a4340e: pop rsi
140a4340f: pop r12
140a43411: pop r13
140a43413: pop r14
140a43415: pop r15
140a43417: pop rbp
140a43418: ret 
140a43419: nop dword ptr [rax]
140a43420: mov qword ptr [rsp + 0x10], rdx
140a43425: push rbp
140a43426: push r15
140a43428: push r14
140a4342a: push r13
140a4342c: push r12
140a4342e: push rsi
140a4342f: push rdi
140a43430: push rbx
140a43431: sub rsp, 0x98
140a43438: lea rbp, [rdx + 0x80]
140a4343f: movaps xmmword ptr [rsp + 0x30], xmm11
140a43445: movaps xmmword ptr [rsp + 0x40], xmm10
140a4344b: movaps xmmword ptr [rsp + 0x50], xmm9
140a43451: movaps xmmword ptr [rsp + 0x60], xmm8
140a43457: movaps xmmword ptr [rsp + 0x70], xmm7
140a4345c: movaps xmmword ptr [rsp + 0x80], xmm6
140a43464: mov rax, qword ptr [rbp + 0x2d8]
140a4346b: lea rax, [rax*2]
140a43473: test rax, rax
140a43476: je 0x140a43499
140a43478: mov rax, qword ptr [rbp + 0x2d8]
140a4347f: shl rax, 3
140a43483: lea rdx, [rax + rax*4]
140a43487: mov r8d, 4
140a4348d: mov rcx, qword ptr [rbp + 0x148]
140a43494: call 0x140613c20
140a43499: movaps xmm6, xmmword ptr [rsp + 0x80]
140a434a1: movaps xmm7, xmmword ptr [rsp + 0x70]
140a434a6: movaps xmm8, xmmword ptr [rsp + 0x60]
140a434ac: movaps xmm9, xmmword ptr [rsp + 0x50]
140a434b2: movaps xmm10, xmmword ptr [rsp + 0x40]
140a434b8: movaps xmm11, xmmword ptr [rsp + 0x30]
140a434be: add rsp, 0x98
140a434c5: pop rbx
140a434c6: pop rdi
140a434c7: pop rsi
140a434c8: pop r12
140a434ca: pop r13
140a434cc: pop r14
140a434ce: pop r15
140a434d0: pop rbp
140a434d1: ret 
140a434d2: nop word ptr cs:[rax + rax]
140a434e0: mov qword ptr [rsp + 0x10], rdx
140a434e5: push rbp
140a434e6: push r15
140a434e8: push r14
140a434ea: push r13
140a434ec: push r12
140a434ee: push rsi
140a434ef: push rdi
140a434f0: push rbx
140a434f1: sub rsp, 0x98
140a434f8: lea rbp, [rdx + 0x80]
140a434ff: movaps xmmword ptr [rsp + 0x30], xmm11
140a43505: movaps xmmword ptr [rsp + 0x40], xmm10
140a4350b: movaps xmmword ptr [rsp + 0x50], xmm9
140a43511: movaps xmmword ptr [rsp + 0x60], xmm8
140a43517: movaps xmmword ptr [rsp + 0x70], xmm7
140a4351c: movaps xmmword ptr [rsp + 0x80], xmm6
140a43524: mov rcx, qword ptr [rbp + 0x328]
140a4352b: call 0x140161c30
140a43530: movaps xmm6, xmmword ptr [rsp + 0x80]
140a43538: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4353d: movaps xmm8, xmmword ptr [rsp + 0x60]
140a43543: movaps xmm9, xmmword ptr [rsp + 0x50]
140a43549: movaps xmm10, xmmword ptr [rsp + 0x40]
140a4354f: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43555: add rsp, 0x98
140a4355c: pop rbx
140a4355d: pop rdi
140a4355e: pop rsi
140a4355f: pop r12
140a43561: pop r13
140a43563: pop r14
140a43565: pop r15
140a43567: pop rbp
140a43568: ret 
140a43569: nop dword ptr [rax]
140a43570: mov qword ptr [rsp + 0x10], rdx
140a43575: push rbp
140a43576: push r15
140a43578: push r14
140a4357a: push r13
140a4357c: push r12
140a4357e: push rsi
140a4357f: push rdi
140a43580: push rbx
140a43581: sub rsp, 0x98
140a43588: lea rbp, [rdx + 0x80]
140a4358f: movaps xmmword ptr [rsp + 0x30], xmm11
140a43595: movaps xmmword ptr [rsp + 0x40], xmm10
140a4359b: movaps xmmword ptr [rsp + 0x50], xmm9
140a435a1: movaps xmmword ptr [rsp + 0x60], xmm8
140a435a7: movaps xmmword ptr [rsp + 0x70], xmm7
140a435ac: movaps xmmword ptr [rsp + 0x80], xmm6
140a435b4: cmp qword ptr [rbp + 0x2c0], 0
140a435bc: sete al
140a435bf: or al, byte ptr [rbp + 0x357]
140a435c5: test al, 1
140a435c7: jne 0x140a435e3
140a435c9: imul rdx, qword ptr [rbp + 0x2c0], 0x58
140a435d1: mov r8d, 8
140a435d7: mov rcx, qword ptr [rbp + 0x2d8]
140a435de: call 0x140613c20
140a435e3: movaps xmm6, xmmword ptr [rsp + 0x80]
140a435eb: movaps xmm7, xmmword ptr [rsp + 0x70]
140a435f0: movaps xmm8, xmmword ptr [rsp + 0x60]
140a435f6: movaps xmm9, xmmword ptr [rsp + 0x50]
140a435fc: movaps xmm10, xmmword ptr [rsp + 0x40]
140a43602: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43608: add rsp, 0x98
140a4360f: pop rbx
140a43610: pop rdi
140a43611: pop rsi
140a43612: pop r12
140a43614: pop r13
140a43616: pop r14
140a43618: pop r15
140a4361a: pop rbp
140a4361b: ret 
140a4361c: nop dword ptr [rax]
140a43620: mov qword ptr [rsp + 0x10], rdx
140a43625: push rbp
140a43626: push r15
140a43628: push r14
140a4362a: push r13
140a4362c: push r12
140a4362e: push rsi
140a4362f: push rdi
140a43630: push rbx
140a43631: sub rsp, 0x98
140a43638: lea rbp, [rdx + 0x80]
140a4363f: movaps xmmword ptr [rsp + 0x30], xmm11
140a43645: movaps xmmword ptr [rsp + 0x40], xmm10
140a4364b: movaps xmmword ptr [rsp + 0x50], xmm9
140a43651: movaps xmmword ptr [rsp + 0x60], xmm8
140a43657: movaps xmmword ptr [rsp + 0x70], xmm7
140a4365c: movaps xmmword ptr [rsp + 0x80], xmm6
140a43664: lea rcx, [rbp + 0x70]
140a43668: call 0x140877430
140a4366d: movaps xmm6, xmmword ptr [rsp + 0x80]
140a43675: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4367a: movaps xmm8, xmmword ptr [rsp + 0x60]
140a43680: movaps xmm9, xmmword ptr [rsp + 0x50]
140a43686: movaps xmm10, xmmword ptr [rsp + 0x40]
140a4368c: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43692: add rsp, 0x98
140a43699: pop rbx
140a4369a: pop rdi
140a4369b: pop rsi
140a4369c: pop r12
140a4369e: pop r13
140a436a0: pop r14
140a436a2: pop r15
140a436a4: pop rbp
140a436a5: ret 
140a436a6: nop word ptr cs:[rax + rax]
140a436b0: mov qword ptr [rsp + 0x10], rdx
140a436b5: push rbp
140a436b6: push r15
140a436b8: push r14
140a436ba: push r13
140a436bc: push r12
140a436be: push rsi
140a436bf: push rdi
140a436c0: push rbx
140a436c1: sub rsp, 0x98
140a436c8: lea rbp, [rdx + 0x80]
140a436cf: movaps xmmword ptr [rsp + 0x30], xmm11
140a436d5: movaps xmmword ptr [rsp + 0x40], xmm10
140a436db: movaps xmmword ptr [rsp + 0x50], xmm9
140a436e1: movaps xmmword ptr [rsp + 0x60], xmm8
140a436e7: movaps xmmword ptr [rsp + 0x70], xmm7
140a436ec: movaps xmmword ptr [rsp + 0x80], xmm6
140a436f4: mov rcx, qword ptr [rbp + 0x2a0]
140a436fb: lea rax, [rcx*2]
140a43703: test rax, rax
140a43706: je 0x140a43722
140a43708: shl rcx, 3
140a4370c: lea rdx, [rcx + rcx*4]
140a43710: mov r8d, 4
140a43716: mov rcx, qword ptr [rbp + 0x2d0]
140a4371d: call 0x140613c20
140a43722: movaps xmm6, xmmword ptr [rsp + 0x80]
140a4372a: movaps xmm7, xmmword ptr [rsp + 0x70]
140a4372f: movaps xmm8, xmmword ptr [rsp + 0x60]
140a43735: movaps xmm9, xmmword ptr [rsp + 0x50]
140a4373b: movaps xmm10, xmmword ptr [rsp + 0x40]
140a43741: movaps xmm11, xmmword ptr [rsp + 0x30]
140a43747: add rsp, 0x98
140a4374e: pop rbx
140a4374f: pop rdi
140a43750: pop rsi
140a43751: pop r12
140a43753: pop r13
140a43755: pop r14
140a43757: pop r15
140a43759: pop rbp
140a4375a: ret 
140a4375b: nop dword ptr [rax + rax]
140a43760: mov qword ptr [rsp + 0x10], rdx
140a43765: push rbp
140a43766: push r15
140a43768: push r14
140a4376a: push r13
140a4376c: push r12
140a4376e: push rsi
140a4376f: push rdi
140a43770: push rbx
140a43771: sub rsp, 0x98
140a43778: lea rbp, [rdx + 0x80]
140a4377f: movaps xmmword ptr [rsp + 0x30], xmm11
140a43785: movaps xmmword ptr [rsp + 0x40], xmm10
140a4378b: movaps xmmword ptr [rsp + 0x50], xmm9
140a43791: movaps xmmword ptr [rsp + 0x60], xmm8
140a43797: movaps xmmword ptr [rsp + 0x70], xmm7
140a4379c: movaps xmmword ptr [rsp + 0x80], xmm6
140a437a4: cmp qword ptr [rbp + 0x48], 0
140a437a9: je 0x140a437cb
140a437ab: lea rax, [rbp + 0x50]
140a437af: mov qword ptr [rbp - 0x20], rax
140a437b3: lea rax, [rbp - 0x20]
140a437b7: mov qword ptr [rbp - 0x28], rax
140a437bb: lea rcx, [rip + 0x208a5be]
140a437c2: lea rdx, [rbp - 0x28]
140a437c6: call 0x14132f2d0
140a437cb: movaps xmm6, xmmword ptr [rsp + 0x80]
140a437d3: movaps xmm7, xmmword ptr [rsp + 0x70]
140a437d8: movaps xmm8, xmmword ptr [rsp + 0x60]
140a437de: movaps xmm9, xmmword ptr [rsp + 0x50]
140a437e4: movaps xmm10, xmmword ptr [rsp + 0x40]
140a437ea: movaps xmm11, xmmword ptr [rsp + 0x30]
140a437f0: add rsp, 0x98
140a437f7: pop rbx
140a437f8: pop rdi
140a437f9: pop rsi
140a437fa: pop r12
140a437fc: pop r13
140a437fe: pop r14
140a43800: pop r15
140a43802: pop rbp
140a43803: ret 
