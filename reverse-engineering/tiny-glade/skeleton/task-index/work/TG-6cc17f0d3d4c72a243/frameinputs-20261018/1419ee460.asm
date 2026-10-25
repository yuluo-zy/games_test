1419ee460: push r14
1419ee462: push rsi
1419ee463: push rdi
1419ee464: push rbx
1419ee465: sub rsp, 0xa8
1419ee46c: mov edi, 1
1419ee471: lock xadd dword ptr [rdx + 0x5b0], edi
1419ee479: xor eax, eax
1419ee47b: cmp rax, qword ptr [rcx + 0x230]
1419ee482: jo 0x1419ee65b
1419ee488: mov rsi, rcx
1419ee48b: mov rax, qword ptr [rcx + 0x268]
1419ee492: mov r8, qword ptr [rdx + 0x218]
1419ee499: cmp r8, rax
1419ee49c: jbe 0x1419ee5a9
1419ee4a2: mov r9, qword ptr [rdx + 0x210]
1419ee4a9: mov rax, qword ptr [r9 + rax*8]
1419ee4ad: test rax, rax
1419ee4b0: je 0x1419ee5a9
1419ee4b6: mov rcx, qword ptr [rdx + 0x1e0]
1419ee4bd: not rax
1419ee4c0: lea rax, [rax + rax*2]
1419ee4c4: shl rax, 5
1419ee4c8: cmp qword ptr [rcx + rax + 0x20], 0
1419ee4ce: je 0x1419ee5a9
1419ee4d4: mov r10, qword ptr [rsi + 0x270]
1419ee4db: cmp r8, r10
1419ee4de: jbe 0x1419ee5c7
1419ee4e4: mov r8, qword ptr [r9 + r10*8]
1419ee4e8: test r8, r8
1419ee4eb: je 0x1419ee5c7
1419ee4f1: not r8
1419ee4f4: lea r9, [r8 + r8*2]
1419ee4f8: shl r9, 5
1419ee4fc: cmp qword ptr [rcx + r9 + 0x20], 0
1419ee502: je 0x1419ee5c7
1419ee508: add rax, rcx
1419ee50b: mov r8, qword ptr [rax + 0x10]
1419ee50f: mov r10d, dword ptr [rsi + 0x130]
1419ee516: add r9, rcx
1419ee519: lea rcx, [rsi + 0x140]
1419ee520: lea r11, [rax + 0x5c]
1419ee524: add rax, 0x58
1419ee528: mov rbx, qword ptr [r9 + 0x10]
1419ee52c: lea r14, [r9 + 0x58]
1419ee530: add r9, 0x5c
1419ee534: mov qword ptr [rsp + 0x30], r8
1419ee539: mov qword ptr [rsp + 0x38], rax
1419ee53e: mov qword ptr [rsp + 0x40], r11
1419ee543: mov dword ptr [rsp + 0x48], r10d
1419ee548: mov dword ptr [rsp + 0x4c], edi
1419ee54c: lea rax, [rsp + 0x50]
1419ee551: mov qword ptr [rsp + 0x50], rcx
1419ee556: mov qword ptr [rsp + 0x58], rdx
1419ee55b: mov dword ptr [rsp + 0x60], r10d
1419ee560: mov dword ptr [rsp + 0x64], edi
1419ee564: lea r8, [rsp + 0x68]
1419ee569: mov qword ptr [rsp + 0x68], rbx
1419ee56e: mov qword ptr [rsp + 0x70], r14
1419ee573: mov qword ptr [rsp + 0x78], r9
1419ee578: mov dword ptr [rsp + 0x80], r10d
1419ee580: mov dword ptr [rsp + 0x84], edi
1419ee587: lea rcx, [rsp + 0x30]
1419ee58c: mov rdx, rax
1419ee58f: call 0x142196e70
1419ee594: mov dword ptr [rsi + 0x130], edi
1419ee59a: xor eax, eax
1419ee59c: add rsp, 0xa8
1419ee5a3: pop rbx
1419ee5a4: pop rdi
1419ee5a5: pop rsi
1419ee5a6: pop r14
1419ee5a8: ret 
1419ee5a9: add rsi, 0x98
1419ee5b0: lea rax, [rip + 0x14bb711]
1419ee5b7: mov qword ptr [rsp + 0x20], rax
1419ee5bc: mov qword ptr [rsp + 0x28], 0x2c
1419ee5c5: jmp 0x1419ee5e3
1419ee5c7: add rsi, 0x98
1419ee5ce: lea rax, [rip + 0x14bb5ea]
1419ee5d5: mov qword ptr [rsp + 0x20], rax
1419ee5da: mov qword ptr [rsp + 0x28], 0x3d
1419ee5e3: mov qword ptr [rsp + 0x88], rsi
1419ee5eb: lea rax, [rip - 0x19b2b22]
1419ee5f2: mov qword ptr [rsp + 0x90], rax
1419ee5fa: lea rax, [rsp + 0x20]
1419ee5ff: mov qword ptr [rsp + 0x98], rax
1419ee607: lea rax, [rip - 0x18e477e]
1419ee60e: mov qword ptr [rsp + 0xa0], rax
1419ee616: lea rax, [rip + 0x14bb60b]
1419ee61d: mov qword ptr [rsp + 0x30], rax
1419ee622: mov qword ptr [rsp + 0x38], 2
1419ee62b: mov qword ptr [rsp + 0x50], 0
1419ee634: lea rax, [rsp + 0x88]
1419ee63c: mov qword ptr [rsp + 0x40], rax
1419ee641: mov qword ptr [rsp + 0x48], 2
1419ee64a: lea rdx, [rip + 0x14bb65f]
1419ee651: lea rcx, [rsp + 0x30]
1419ee656: call 0x1428d9390
1419ee65b: lea rcx, [rip + 0x14bb76e]
1419ee662: lea r8, [rip + 0x14bb7c7]
1419ee669: mov edx, 0x59
1419ee66e: call 0x1428d9330
1419ee673: int3 
