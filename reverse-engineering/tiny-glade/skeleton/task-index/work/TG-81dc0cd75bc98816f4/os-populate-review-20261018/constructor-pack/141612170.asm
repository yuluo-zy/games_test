141612170: push rbp
141612171: push r15
141612173: push r14
141612175: push r12
141612177: push rsi
141612178: push rdi
141612179: push rbx
14161217a: sub rsp, 0x120
141612181: lea rbp, [rsp + 0x80]
141612189: mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
141612194: mov rdi, rdx
141612197: mov rsi, rcx
14161219a: test r8, r8
14161219d: je 0x1416121dc
14161219f: mov rbx, r9
1416121a2: mov rax, r9
1416121a5: and rax, 3
1416121a9: mov qword ptr [rbp - 0x40], rax
1416121ad: jne 0x141612479
1416121b3: mov r15, r8
1416121b6: mov rax, r8
1416121b9: and rax, 3
1416121bd: mov qword ptr [rbp - 0x40], rax
1416121c1: jne 0x14161248a
1416121c7: cmp rbx, 0x404
1416121ce: jae 0x141612514
1416121d4: xor r12d, r12d
1416121d7: jmp 0x1416123b0
1416121dc: movzx eax, byte ptr [rip + 0x2022a26]
1416121e3: mov ebx, 0x400
1416121e8: mov ecx, 0x400
1416121ed: mov edx, 4
1416121f2: call 0x140613c10
1416121f7: test rax, rax
1416121fa: je 0x14161257e
141612200: mov r15, rax
141612203: xorps xmm0, xmm0
141612206: movups xmmword ptr [rax], xmm0
141612209: movups xmmword ptr [rax + 0x10], xmm0
14161220d: movups xmmword ptr [rax + 0x20], xmm0
141612211: movups xmmword ptr [rax + 0x30], xmm0
141612215: movups xmmword ptr [rax + 0x40], xmm0
141612219: movups xmmword ptr [rax + 0x50], xmm0
14161221d: movups xmmword ptr [rax + 0x60], xmm0
141612221: movups xmmword ptr [rax + 0x70], xmm0
141612225: movups xmmword ptr [rax + 0x80], xmm0
14161222c: movups xmmword ptr [rax + 0x90], xmm0
141612233: movups xmmword ptr [rax + 0xa0], xmm0
14161223a: movups xmmword ptr [rax + 0xb0], xmm0
141612241: movups xmmword ptr [rax + 0xc0], xmm0
141612248: movups xmmword ptr [rax + 0xd0], xmm0
14161224f: movups xmmword ptr [rax + 0xe0], xmm0
141612256: movups xmmword ptr [rax + 0xf0], xmm0
14161225d: movups xmmword ptr [rax + 0x100], xmm0
141612264: movups xmmword ptr [rax + 0x110], xmm0
14161226b: movups xmmword ptr [rax + 0x120], xmm0
141612272: movups xmmword ptr [rax + 0x130], xmm0
141612279: movups xmmword ptr [rax + 0x140], xmm0
141612280: movups xmmword ptr [rax + 0x150], xmm0
141612287: movups xmmword ptr [rax + 0x160], xmm0
14161228e: movups xmmword ptr [rax + 0x170], xmm0
141612295: movups xmmword ptr [rax + 0x180], xmm0
14161229c: movups xmmword ptr [rax + 0x190], xmm0
1416122a3: movups xmmword ptr [rax + 0x1a0], xmm0
1416122aa: movups xmmword ptr [rax + 0x1b0], xmm0
1416122b1: movups xmmword ptr [rax + 0x1c0], xmm0
1416122b8: movups xmmword ptr [rax + 0x1d0], xmm0
1416122bf: movups xmmword ptr [rax + 0x1e0], xmm0
1416122c6: movups xmmword ptr [rax + 0x1f0], xmm0
1416122cd: movups xmmword ptr [rax + 0x200], xmm0
1416122d4: movups xmmword ptr [rax + 0x210], xmm0
1416122db: movups xmmword ptr [rax + 0x220], xmm0
1416122e2: movups xmmword ptr [rax + 0x230], xmm0
1416122e9: movups xmmword ptr [rax + 0x240], xmm0
1416122f0: movups xmmword ptr [rax + 0x250], xmm0
1416122f7: movups xmmword ptr [rax + 0x260], xmm0
1416122fe: movups xmmword ptr [rax + 0x270], xmm0
141612305: movups xmmword ptr [rax + 0x280], xmm0
14161230c: movups xmmword ptr [rax + 0x290], xmm0
141612313: movups xmmword ptr [rax + 0x2a0], xmm0
14161231a: movups xmmword ptr [rax + 0x2b0], xmm0
141612321: movups xmmword ptr [rax + 0x2c0], xmm0
141612328: movups xmmword ptr [rax + 0x2d0], xmm0
14161232f: movups xmmword ptr [rax + 0x2e0], xmm0
141612336: movups xmmword ptr [rax + 0x2f0], xmm0
14161233d: movups xmmword ptr [rax + 0x300], xmm0
141612344: movups xmmword ptr [rax + 0x310], xmm0
14161234b: movups xmmword ptr [rax + 0x320], xmm0
141612352: movups xmmword ptr [rax + 0x330], xmm0
141612359: movups xmmword ptr [rax + 0x340], xmm0
141612360: movups xmmword ptr [rax + 0x350], xmm0
141612367: movups xmmword ptr [rax + 0x360], xmm0
14161236e: movups xmmword ptr [rax + 0x370], xmm0
141612375: movups xmmword ptr [rax + 0x380], xmm0
14161237c: movups xmmword ptr [rax + 0x390], xmm0
141612383: movups xmmword ptr [rax + 0x3a0], xmm0
14161238a: movups xmmword ptr [rax + 0x3b0], xmm0
141612391: movups xmmword ptr [rax + 0x3c0], xmm0
141612398: movups xmmword ptr [rax + 0x3d0], xmm0
14161239f: movups xmmword ptr [rax + 0x3e0], xmm0
1416123a6: movups xmmword ptr [rax + 0x3f0], xmm0
1416123ad: mov r12b, 1
1416123b0: mov qword ptr [rbp + 0x88], r15
1416123b7: mov byte ptr [rbp + 0x97], r12b
1416123be: lea rcx, [rbp + 0x10]
1416123c2: mov edx, 0x400
1416123c7: mov r8d, 0x22
1416123cd: mov r9b, 1
1416123d0: call 0x140bd4de0
1416123d5: mov qword ptr [rbp + 0x88], r15
1416123dc: mov byte ptr [rbp + 0x97], r12b
1416123e3: lea r14, [rbp + 0x68]
1416123e7: lea rdx, [rbp + 0x10]
1416123eb: mov r8d, 4
1416123f1: mov rcx, r14
1416123f4: call 0x140bd4f20
1416123f9: mov qword ptr [rbp + 0x88], r15
141612400: mov byte ptr [rbp + 0x97], r12b
141612407: mov qword ptr [rsp + 0x30], rbx
14161240c: mov qword ptr [rsp + 0x28], r15
141612411: mov qword ptr [rsp + 0x20], r14
141612416: lea r8, [rip + 0x166d90a]
14161241d: lea rcx, [rbp - 0x40]
141612421: mov r9d, 0x17
141612427: mov rdx, rdi
14161242a: call 0x1416a2dc0
14161242f: cmp dword ptr [rbp - 0x40], 0xb
141612433: jne 0x1416124b4
141612435: mov rax, qword ptr [rbp - 0x18]
141612439: mov qword ptr [rsi + 0x20], rax
14161243d: movups xmm0, xmmword ptr [rbp - 0x38]
141612441: movups xmm1, xmmword ptr [rbp - 0x28]
141612445: movups xmmword ptr [rsi + 0x10], xmm1
141612449: movups xmmword ptr [rsi], xmm0
14161244c: test r12b, r12b
14161244f: je 0x141612464
141612451: mov edx, 0x400
141612456: mov r8d, 4
14161245c: mov rcx, r15
14161245f: call 0x140613c20
141612464: mov rax, rsi
141612467: add rsp, 0x120
14161246e: pop rbx
14161246f: pop rdi
141612470: pop rsi
141612471: pop r12
141612473: pop r14
141612475: pop r15
141612477: pop rbp
141612478: ret 
141612479: mov qword ptr [rbp + 0x10], 0
141612481: lea rax, [rip + 0x16b7ac0]
141612488: jmp 0x141612499
14161248a: mov qword ptr [rbp + 0x10], 0
141612492: lea rax, [rip + 0x16b7ac7]
141612499: mov qword ptr [rsp + 0x20], rax
14161249e: lea r8, [rip + 0x16b7a9b]
1416124a5: lea rdx, [rbp - 0x40]
1416124a9: lea r9, [rbp + 0x10]
1416124ad: xor ecx, ecx
1416124af: call 0x1428d9596
1416124b4: mov dword ptr [rbp + 0x84], r12d
1416124bb: mov qword ptr [rbp + 0x78], r15
1416124bf: movups xmm0, xmmword ptr [rbp]
1416124c3: movaps xmmword ptr [rbp + 0x50], xmm0
1416124c7: movups xmm0, xmmword ptr [rbp - 0x40]
1416124cb: movups xmm1, xmmword ptr [rbp - 0x30]
1416124cf: movups xmm2, xmmword ptr [rbp - 0x20]
1416124d3: movups xmm3, xmmword ptr [rbp - 0x10]
1416124d7: movaps xmmword ptr [rbp + 0x40], xmm3
1416124db: movaps xmmword ptr [rbp + 0x30], xmm2
1416124df: movaps xmmword ptr [rbp + 0x20], xmm1
1416124e3: movaps xmmword ptr [rbp + 0x10], xmm0
1416124e7: lea rax, [rip + 0x166dcba]
1416124ee: mov qword ptr [rsp + 0x20], rax
1416124f3: lea rcx, [rip + 0x166c5ee]
1416124fa: lea r9, [rip + 0x166c5c7]
141612501: lea r8, [rbp + 0x10]
141612505: mov edx, 0x2b
14161250a: call 0x1428d9760
14161250f: jmp 0x14161259b
141612514: lea rax, [rip + 0x148b783]
14161251b: mov qword ptr [rbp - 0x40], rax
14161251f: mov qword ptr [rbp - 0x38], 0x61
141612527: lea rax, [rbp - 0x40]
14161252b: mov qword ptr [rbp + 0x68], rax
14161252f: lea rax, [rip - 0x15086a6]
141612536: mov qword ptr [rbp + 0x70], rax
14161253a: lea rax, [rip + 0x166d287]
141612541: mov qword ptr [rbp + 0x10], rax
141612545: mov qword ptr [rbp + 0x18], 1
14161254d: mov qword ptr [rbp + 0x30], 0
141612555: lea rax, [rbp + 0x68]
141612559: mov qword ptr [rbp + 0x20], rax
14161255d: mov qword ptr [rbp + 0x28], 1
141612565: mov byte ptr [rbp + 0x97], 0
14161256c: lea rdx, [rip + 0x166d265]
141612573: lea rcx, [rbp + 0x10]
141612577: call 0x1428d9390
14161257c: jmp 0x14161259b
14161257e: mov byte ptr [rbp + 0x97], 0
141612585: lea r8, [rip + 0x166c524]
14161258c: mov ecx, 4
141612591: mov edx, 0x400
141612596: call 0x1428d8fe3
14161259b: ud2 
14161259d: nop dword ptr [rax]
1416125a0: mov qword ptr [rsp + 0x10], rdx
1416125a5: push rbp
1416125a6: push r15
1416125a8: push r14
1416125aa: push r12
1416125ac: push rsi
1416125ad: push rdi
1416125ae: push rbx
1416125af: sub rsp, 0x40
1416125b3: lea rbp, [rdx + 0x80]
1416125ba: mov rsi, qword ptr [rbp + 0x78]
1416125be: mov qword ptr [rbp + 0x88], rsi
1416125c5: mov edi, dword ptr [rbp + 0x84]
1416125cb: mov byte ptr [rbp + 0x97], dil
1416125d2: lea rcx, [rbp + 0x10]
1416125d6: call 0x1407cea70
1416125db: mov qword ptr [rbp + 0x88], rsi
1416125e2: mov byte ptr [rbp + 0x97], dil
1416125e9: add rsp, 0x40
1416125ed: pop rbx
1416125ee: pop rdi
1416125ef: pop rsi
1416125f0: pop r12
1416125f2: pop r14
1416125f4: pop r15
1416125f6: pop rbp
1416125f7: ret 
1416125f8: nop dword ptr [rax + rax]
141612600: mov qword ptr [rsp + 0x10], rdx
141612605: push rbp
141612606: push r15
141612608: push r14
14161260a: push r12
14161260c: push rsi
14161260d: push rdi
14161260e: push rbx
14161260f: sub rsp, 0x40
141612613: lea rbp, [rdx + 0x80]
14161261a: test byte ptr [rbp + 0x97], 1
141612621: je 0x14161263a
141612623: mov rcx, qword ptr [rbp + 0x88]
14161262a: mov edx, 0x400
14161262f: mov r8d, 4
141612635: call 0x140613c20
14161263a: nop 
14161263b: add rsp, 0x40
14161263f: pop rbx
141612640: pop rdi
141612641: pop rsi
141612642: pop r12
141612644: pop r14
141612646: pop r15
141612648: pop rbp
141612649: ret 
