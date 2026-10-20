1405bef90: push r15
1405bef92: push r14
1405bef94: push r13
1405bef96: push r12
1405bef98: push rsi
1405bef99: push rdi
1405bef9a: push rbp
1405bef9b: push rbx
1405bef9c: sub rsp, 0x188
1405befa3: mov edi, 1
1405befa8: lock xadd dword ptr [rdx + 0x5b0], edi
1405befb0: test byte ptr [rcx], 1
1405befb3: je 0x1405bf5c7
1405befb9: mov rsi, rcx
1405befbc: mov rax, qword ptr [rcx + 8]
1405befc0: cmp qword ptr [rdx + 0x218], rax
1405befc7: jbe 0x1405bf425
1405befcd: mov rcx, qword ptr [rdx + 0x210]
1405befd4: mov rax, qword ptr [rcx + rax*8]
1405befd8: test rax, rax
1405befdb: je 0x1405bf425
1405befe1: mov rcx, qword ptr [rdx + 0x1e0]
1405befe8: not rax
1405befeb: lea rax, [rax + rax*2]
1405befef: shl rax, 5
1405beff3: cmp qword ptr [rcx + rax + 0x20], 0
1405beff9: je 0x1405bf425
1405befff: add rcx, rax
1405bf002: mov rbx, qword ptr [rcx + 0x10]
1405bf006: mov eax, dword ptr [rdx + 0x5b0]
1405bf00c: mov rax, qword ptr [rsi + 0x10]
1405bf010: cmp qword ptr [rdx + 0x218], rax
1405bf017: jbe 0x1405bf443
1405bf01d: mov r8, qword ptr [rdx + 0x210]
1405bf024: mov rax, qword ptr [r8 + rax*8]
1405bf028: test rax, rax
1405bf02b: je 0x1405bf443
1405bf031: mov r9, qword ptr [rdx + 0x1e0]
1405bf038: not rax
1405bf03b: lea r8, [rax + rax*2]
1405bf03f: shl r8, 5
1405bf043: cmp qword ptr [r9 + r8 + 0x20], 0
1405bf049: je 0x1405bf443
1405bf04f: mov eax, dword ptr [rsi + 0x188]
1405bf055: add r9, r8
1405bf058: mov r15, qword ptr [r9 + 0x10]
1405bf05c: mov r8d, dword ptr [rdx + 0x5b0]
1405bf063: mov r8, qword ptr [rsi + 0x18]
1405bf067: cmp qword ptr [rdx + 0x218], r8
1405bf06e: jbe 0x1405bf453
1405bf074: mov r10, qword ptr [rdx + 0x210]
1405bf07b: mov r8, qword ptr [r10 + r8*8]
1405bf07f: test r8, r8
1405bf082: je 0x1405bf453
1405bf088: mov r11, qword ptr [rdx + 0x1e0]
1405bf08f: not r8
1405bf092: lea r8, [r8 + r8*2]
1405bf096: shl r8, 5
1405bf09a: cmp qword ptr [r11 + r8 + 0x20], 0
1405bf0a0: je 0x1405bf453
1405bf0a6: add r11, r8
1405bf0a9: mov r13, qword ptr [r11 + 0x10]
1405bf0ad: mov r8d, dword ptr [rdx + 0x5b0]
1405bf0b4: mov r8, qword ptr [rsi + 0x20]
1405bf0b8: cmp qword ptr [rdx + 0x218], r8
1405bf0bf: jbe 0x1405bf463
1405bf0c5: mov r10, qword ptr [rdx + 0x210]
1405bf0cc: mov r8, qword ptr [r10 + r8*8]
1405bf0d0: test r8, r8
1405bf0d3: je 0x1405bf463
1405bf0d9: mov r14, qword ptr [rdx + 0x1e0]
1405bf0e0: not r8
1405bf0e3: lea r8, [r8 + r8*2]
1405bf0e7: shl r8, 5
1405bf0eb: cmp qword ptr [r14 + r8 + 0x20], 0
1405bf0f1: je 0x1405bf463
1405bf0f7: add r14, r8
1405bf0fa: mov r8, qword ptr [r14 + 0x10]
1405bf0fe: mov qword ptr [rsp + 0x180], r8
1405bf106: mov r8d, dword ptr [rdx + 0x5b0]
1405bf10d: mov r8, qword ptr [rsi + 0x28]
1405bf111: cmp qword ptr [rdx + 0x218], r8
1405bf118: jbe 0x1405bf481
1405bf11e: mov r10, qword ptr [rdx + 0x210]
1405bf125: mov r8, qword ptr [r10 + r8*8]
1405bf129: test r8, r8
1405bf12c: je 0x1405bf481
1405bf132: mov r12, qword ptr [rdx + 0x1e0]
1405bf139: not r8
1405bf13c: lea r8, [r8 + r8*2]
1405bf140: shl r8, 5
1405bf144: cmp qword ptr [r12 + r8 + 0x20], 0
1405bf14a: je 0x1405bf481
1405bf150: mov qword ptr [rsp + 0x170], r13
1405bf158: mov qword ptr [rsp + 0x178], r15
1405bf160: add r12, r8
1405bf163: mov r8, qword ptr [r12 + 0x10]
1405bf168: mov qword ptr [rsp + 0x168], r8
1405bf170: mov r8d, dword ptr [rdx + 0x5b0]
1405bf177: mov r8, qword ptr [rsi + 0x30]
1405bf17b: cmp qword ptr [rdx + 0x218], r8
1405bf182: jbe 0x1405bf49f
1405bf188: mov r10, qword ptr [rdx + 0x210]
1405bf18f: mov r8, qword ptr [r10 + r8*8]
1405bf193: test r8, r8
1405bf196: je 0x1405bf49f
1405bf19c: mov rbp, qword ptr [rdx + 0x1e0]
1405bf1a3: not r8
1405bf1a6: lea r8, [r8 + r8*2]
1405bf1aa: shl r8, 5
1405bf1ae: cmp qword ptr [rbp + r8 + 0x20], 0
1405bf1b4: je 0x1405bf49f
1405bf1ba: mov qword ptr [rsp + 0x160], rbx
1405bf1c2: add rbp, r8
1405bf1c5: mov r8, qword ptr [rbp + 0x10]
1405bf1c9: mov r10d, dword ptr [rdx + 0x5b0]
1405bf1d0: mov rbx, qword ptr [rsi + 0x38]
1405bf1d4: mov r10, qword ptr [rdx + 0x218]
1405bf1db: cmp r10, rbx
1405bf1de: jbe 0x1405bf524
1405bf1e4: mov r15, qword ptr [rdx + 0x210]
1405bf1eb: mov r13, qword ptr [r15 + rbx*8]
1405bf1ef: test r13, r13
1405bf1f2: je 0x1405bf524
1405bf1f8: mov rbx, qword ptr [rdx + 0x1e0]
1405bf1ff: not r13
1405bf202: lea rdx, [r13*2]
1405bf20a: add rdx, r13
1405bf20d: shl rdx, 5
1405bf211: cmp qword ptr [rbx + rdx + 0x20], 0
1405bf217: je 0x1405bf524
1405bf21d: mov r13, qword ptr [rsi + 0x48]
1405bf221: cmp r10, r13
1405bf224: jbe 0x1405bf542
1405bf22a: mov r10, qword ptr [r15 + r13*8]
1405bf22e: test r10, r10
1405bf231: je 0x1405bf542
1405bf237: not r10
1405bf23a: lea r10, [r10 + r10*2]
1405bf23e: shl r10, 5
1405bf242: cmp qword ptr [rbx + r10 + 0x20], 0
1405bf248: je 0x1405bf542
1405bf24e: add rdx, rbx
1405bf251: add r10, rbx
1405bf254: mov rbx, qword ptr [rdx + 0x10]
1405bf258: mov r15, qword ptr [r10 + 0x10]
1405bf25c: mov r13, qword ptr [rsp + 0x160]
1405bf264: mov qword ptr [rsp + 0x58], r13
1405bf269: lea r13, [rcx + 0x58]
1405bf26d: mov qword ptr [rsp + 0x60], r13
1405bf272: add rcx, 0x5c
1405bf276: mov qword ptr [rsp + 0x68], rcx
1405bf27b: mov dword ptr [rsp + 0x70], eax
1405bf27f: mov dword ptr [rsp + 0x74], edi
1405bf283: mov rcx, qword ptr [rsp + 0x178]
1405bf28b: mov qword ptr [rsp + 0x78], rcx
1405bf290: lea rcx, [r9 + 0x58]
1405bf294: mov qword ptr [rsp + 0x80], rcx
1405bf29c: add r9, 0x5c
1405bf2a0: mov qword ptr [rsp + 0x88], r9
1405bf2a8: mov dword ptr [rsp + 0x90], eax
1405bf2af: mov dword ptr [rsp + 0x94], edi
1405bf2b6: mov rcx, qword ptr [rsp + 0x170]
1405bf2be: mov qword ptr [rsp + 0x98], rcx
1405bf2c6: lea rcx, [r11 + 0x58]
1405bf2ca: mov qword ptr [rsp + 0xa0], rcx
1405bf2d2: add r11, 0x5c
1405bf2d6: mov qword ptr [rsp + 0xa8], r11
1405bf2de: mov dword ptr [rsp + 0xb0], eax
1405bf2e5: mov dword ptr [rsp + 0xb4], edi
1405bf2ec: mov rcx, qword ptr [rsp + 0x180]
1405bf2f4: mov qword ptr [rsp + 0xb8], rcx
1405bf2fc: lea rcx, [r14 + 0x58]
1405bf300: mov qword ptr [rsp + 0xc0], rcx
1405bf308: add r14, 0x5c
1405bf30c: mov qword ptr [rsp + 0xc8], r14
1405bf314: mov dword ptr [rsp + 0xd0], eax
1405bf31b: mov dword ptr [rsp + 0xd4], edi
1405bf322: mov rcx, qword ptr [rsp + 0x168]
1405bf32a: mov qword ptr [rsp + 0xd8], rcx
1405bf332: lea rcx, [r12 + 0x58]
1405bf337: mov qword ptr [rsp + 0xe0], rcx
1405bf33f: add r12, 0x5c
1405bf343: mov qword ptr [rsp + 0xe8], r12
1405bf34b: mov dword ptr [rsp + 0xf0], eax
1405bf352: mov dword ptr [rsp + 0xf4], edi
1405bf359: mov qword ptr [rsp + 0xf8], r8
1405bf361: lea rcx, [rbp + 0x5c]
1405bf365: add rbp, 0x58
1405bf369: mov qword ptr [rsp + 0x100], rbp
1405bf371: mov qword ptr [rsp + 0x108], rcx
1405bf379: mov dword ptr [rsp + 0x110], eax
1405bf380: mov dword ptr [rsp + 0x114], edi
1405bf387: mov qword ptr [rsp + 0x118], rbx
1405bf38f: lea rcx, [rdx + 0x5c]
1405bf393: add rdx, 0x58
1405bf397: mov qword ptr [rsp + 0x120], rdx
1405bf39f: mov qword ptr [rsp + 0x128], rcx
1405bf3a7: lea rcx, [rsi + 0x40]
1405bf3ab: mov dword ptr [rsp + 0x130], eax
1405bf3b2: mov dword ptr [rsp + 0x134], edi
1405bf3b9: mov qword ptr [rsp + 0x138], rcx
1405bf3c1: mov qword ptr [rsp + 0x140], r15
1405bf3c9: lea rcx, [r10 + 0x58]
1405bf3cd: mov qword ptr [rsp + 0x148], rcx
1405bf3d5: add r10, 0x5c
1405bf3d9: mov qword ptr [rsp + 0x150], r10
1405bf3e1: lea rdx, [rsp + 0x118]
1405bf3e9: mov dword ptr [rsp + 0x158], eax
1405bf3f0: lea r8, [rsp + 0x138]
1405bf3f8: mov dword ptr [rsp + 0x15c], edi
1405bf3ff: lea rcx, [rsp + 0x58]
1405bf404: call 0x140957bf0
1405bf409: mov dword ptr [rsi + 0x188], edi
1405bf40f: xor eax, eax
1405bf411: add rsp, 0x188
1405bf418: pop rbx
1405bf419: pop rbp
1405bf41a: pop rdi
1405bf41b: pop rsi
1405bf41c: pop r12
1405bf41e: pop r13
1405bf420: pop r14
1405bf422: pop r15
1405bf424: ret 
1405bf425: add rsi, 0xf0
1405bf42c: lea rax, [rip + 0x2445cac]
1405bf433: mov qword ptr [rsp + 0x28], rax
1405bf438: mov qword ptr [rsp + 0x30], 0x36
1405bf441: jmp 0x1405bf4bb
1405bf443: add rsi, 0xf0
1405bf44a: lea rax, [rip + 0x2445c50]
1405bf451: jmp 0x1405bf471
1405bf453: add rsi, 0xf0
1405bf45a: lea rax, [rip + 0x2445b4b]
1405bf461: jmp 0x1405bf471
1405bf463: add rsi, 0xf0
1405bf46a: lea rax, [rip + 0x2445ca4]
1405bf471: mov qword ptr [rsp + 0x28], rax
1405bf476: mov qword ptr [rsp + 0x30], 0x3e
1405bf47f: jmp 0x1405bf4bb
1405bf481: add rsi, 0xf0
1405bf488: lea rax, [rip + 0x2445bdb]
1405bf48f: mov qword ptr [rsp + 0x28], rax
1405bf494: mov qword ptr [rsp + 0x30], 0x37
1405bf49d: jmp 0x1405bf4bb
1405bf49f: add rsi, 0xf0
1405bf4a6: lea rax, [rip + 0x2445ca6]
1405bf4ad: mov qword ptr [rsp + 0x28], rax
1405bf4b2: mov qword ptr [rsp + 0x30], 0x38
1405bf4bb: mov qword ptr [rsp + 0x38], rsi
1405bf4c0: lea rax, [rip - 0x5839f7]
1405bf4c7: mov qword ptr [rsp + 0x40], rax
1405bf4cc: lea rax, [rsp + 0x28]
1405bf4d1: mov qword ptr [rsp + 0x48], rax
1405bf4d6: lea rax, [rip - 0x4b564d]
1405bf4dd: mov qword ptr [rsp + 0x50], rax
1405bf4e2: lea rax, [rip + 0x244597f]
1405bf4e9: mov qword ptr [rsp + 0x58], rax
1405bf4ee: mov qword ptr [rsp + 0x60], 2
1405bf4f7: mov qword ptr [rsp + 0x78], 0
1405bf500: lea rax, [rsp + 0x38]
1405bf505: mov qword ptr [rsp + 0x68], rax
1405bf50a: mov qword ptr [rsp + 0x70], 2
1405bf513: lea rdx, [rip + 0x2445ad6]
1405bf51a: lea rcx, [rsp + 0x58]
1405bf51f: call 0x1428d9390
1405bf524: add rsi, 0xf0
1405bf52b: lea rax, [rip + 0x24458cd]
1405bf532: mov qword ptr [rsp + 0x28], rax
1405bf537: mov qword ptr [rsp + 0x30], 0x3d
1405bf540: jmp 0x1405bf55e
1405bf542: add rsi, 0xf0
1405bf549: lea rax, [rip + 0x24459b8]
1405bf550: mov qword ptr [rsp + 0x28], rax
1405bf555: mov qword ptr [rsp + 0x30], 0x71
1405bf55e: mov qword ptr [rsp + 0x38], rsi
1405bf563: lea rax, [rip - 0x583a9a]
1405bf56a: mov qword ptr [rsp + 0x40], rax
1405bf56f: lea rax, [rsp + 0x28]
1405bf574: mov qword ptr [rsp + 0x48], rax
1405bf579: lea rax, [rip - 0x4b56f0]
1405bf580: mov qword ptr [rsp + 0x50], rax
1405bf585: lea rax, [rip + 0x24458dc]
1405bf58c: mov qword ptr [rsp + 0x58], rax
1405bf591: mov qword ptr [rsp + 0x60], 2
1405bf59a: mov qword ptr [rsp + 0x78], 0
1405bf5a3: lea rax, [rsp + 0x38]
1405bf5a8: mov qword ptr [rsp + 0x68], rax
1405bf5ad: mov qword ptr [rsp + 0x70], 2
1405bf5b6: lea rdx, [rip + 0x2445933]
1405bf5bd: lea rcx, [rsp + 0x58]
1405bf5c2: call 0x1428d9390
1405bf5c7: lea rcx, [rip + 0x2445c9a]
1405bf5ce: lea r8, [rip + 0x2445cf3]
1405bf5d5: mov edx, 0x59
1405bf5da: call 0x1428d9330
1405bf5df: int3 