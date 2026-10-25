141613720: push rbp
141613721: push r15
141613723: push r14
141613725: push r12
141613727: push rsi
141613728: push rdi
141613729: push rbx
14161372a: sub rsp, 0x100
141613731: lea rbp, [rsp + 0x80]
141613739: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141613741: mov rbx, r8
141613744: mov rdi, rdx
141613747: mov rsi, rcx
14161374a: test r8, r8
14161374d: je 0x1416137df
141613753: mov r14, r9
141613756: mov rax, r9
141613759: and rax, 3
14161375d: mov qword ptr [rbp - 0x40], rax
141613761: jne 0x14161386b
141613767: mov rax, rbx
14161376a: and rax, 3
14161376e: mov qword ptr [rbp - 0x40], rax
141613772: jne 0x14161387c
141613778: cmp r14, 8
14161377c: jb 0x1416137e2
14161377e: lea rax, [rip + 0x1489f9a]
141613785: mov qword ptr [rbp - 0x40], rax
141613789: mov qword ptr [rbp - 0x38], 0x5c
141613791: lea rax, [rbp - 0x40]
141613795: mov qword ptr [rbp + 0x68], rax
141613799: lea rax, [rip - 0x1509910]
1416137a0: mov qword ptr [rbp + 0x70], rax
1416137a4: lea rax, [rip + 0x166c01d]
1416137ab: mov qword ptr [rbp + 0x10], rax
1416137af: mov qword ptr [rbp + 0x18], 1
1416137b7: mov qword ptr [rbp + 0x30], 0
1416137bf: lea rax, [rbp + 0x68]
1416137c3: mov qword ptr [rbp + 0x20], rax
1416137c7: mov qword ptr [rbp + 0x28], 1
1416137cf: lea rdx, [rip + 0x166c002]
1416137d6: lea rcx, [rbp + 0x10]
1416137da: call 0x1428d9390
1416137df: xor r14d, r14d
1416137e2: lea r15, [rbp + 0x10]
1416137e6: mov edx, 4
1416137eb: mov rcx, r15
1416137ee: mov r8d, 0x22
1416137f4: mov r9b, 1
1416137f7: call 0x140bd4de0
1416137fc: lea r12, [rbp + 0x68]
141613800: mov r8d, 4
141613806: mov rcx, r12
141613809: mov rdx, r15
14161380c: call 0x140bd4f20
141613811: mov qword ptr [rsp + 0x30], r14
141613816: mov qword ptr [rsp + 0x28], rbx
14161381b: mov qword ptr [rsp + 0x20], r12
141613820: lea r8, [rip + 0x166c411]
141613827: lea rcx, [rbp - 0x40]
14161382b: mov r9d, 0x12
141613831: mov rdx, rdi
141613834: call 0x1416a2dc0
141613839: cmp dword ptr [rbp - 0x40], 0xb
14161383d: jne 0x1416138a6
14161383f: mov rax, qword ptr [rbp - 0x18]
141613843: mov qword ptr [rsi + 0x20], rax
141613847: movups xmm0, xmmword ptr [rbp - 0x38]
14161384b: movups xmm1, xmmword ptr [rbp - 0x28]
14161384f: movups xmmword ptr [rsi + 0x10], xmm1
141613853: movups xmmword ptr [rsi], xmm0
141613856: mov rax, rsi
141613859: add rsp, 0x100
141613860: pop rbx
141613861: pop rdi
141613862: pop rsi
141613863: pop r12
141613865: pop r14
141613867: pop r15
141613869: pop rbp
14161386a: ret 
14161386b: mov qword ptr [rbp + 0x10], 0
141613873: lea rax, [rip + 0x16b66ce]
14161387a: jmp 0x14161388b
14161387c: mov qword ptr [rbp + 0x10], 0
141613884: lea rax, [rip + 0x16b66d5]
14161388b: mov qword ptr [rsp + 0x20], rax
141613890: lea r8, [rip + 0x16b66a9]
141613897: lea rdx, [rbp - 0x40]
14161389b: lea r9, [rbp + 0x10]
14161389f: xor ecx, ecx
1416138a1: call 0x1428d9596
1416138a6: movups xmm0, xmmword ptr [rbp]
1416138aa: movaps xmmword ptr [rbp + 0x50], xmm0
1416138ae: movups xmm0, xmmword ptr [rbp - 0x40]
1416138b2: movups xmm1, xmmword ptr [rbp - 0x30]
1416138b6: movups xmm2, xmmword ptr [rbp - 0x20]
1416138ba: movups xmm3, xmmword ptr [rbp - 0x10]
1416138be: movaps xmmword ptr [rbp + 0x40], xmm3
1416138c2: movaps xmmword ptr [rbp + 0x30], xmm2
1416138c6: movaps xmmword ptr [rbp + 0x20], xmm1
1416138ca: movaps xmmword ptr [rbp + 0x10], xmm0
1416138ce: lea rax, [rip + 0x166c8d3]
1416138d5: mov qword ptr [rsp + 0x20], rax
1416138da: lea rcx, [rip + 0x166b207]
1416138e1: lea r9, [rip + 0x166b1e0]
1416138e8: lea r8, [rbp + 0x10]
1416138ec: mov edx, 0x2b
1416138f1: call 0x1428d9760
1416138f6: ud2 
1416138f8: nop dword ptr [rax + rax]
141613900: mov qword ptr [rsp + 0x10], rdx
141613905: push rbp
141613906: push r15
141613908: push r14
14161390a: push r12
14161390c: push rsi
14161390d: push rdi
14161390e: push rbx
14161390f: sub rsp, 0x40
141613913: lea rbp, [rdx + 0x80]
14161391a: lea rcx, [rbp + 0x10]
14161391e: call 0x1407cea70
141613923: nop 
141613924: add rsp, 0x40
141613928: pop rbx
141613929: pop rdi
14161392a: pop rsi
14161392b: pop r12
14161392d: pop r14
14161392f: pop r15
141613931: pop rbp
141613932: ret 
