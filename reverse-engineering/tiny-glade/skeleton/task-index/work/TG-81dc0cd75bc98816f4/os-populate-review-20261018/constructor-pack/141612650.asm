141612650: push rbp
141612651: push r15
141612653: push r14
141612655: push r12
141612657: push rsi
141612658: push rdi
141612659: push rbx
14161265a: sub rsp, 0x100
141612661: lea rbp, [rsp + 0x80]
141612669: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141612671: mov rbx, r8
141612674: mov rdi, rdx
141612677: mov rsi, rcx
14161267a: test r8, r8
14161267d: je 0x141612712
141612683: mov r14, r9
141612686: mov rax, r9
141612689: and rax, 3
14161268d: mov qword ptr [rbp - 0x40], rax
141612691: jne 0x14161279e
141612697: mov rax, rbx
14161269a: and rax, 3
14161269e: mov qword ptr [rbp - 0x40], rax
1416126a2: jne 0x1416127af
1416126a8: cmp r14, 0x1b4
1416126af: jb 0x141612715
1416126b1: lea rax, [rip + 0x148afe6]
1416126b8: mov qword ptr [rbp - 0x40], rax
1416126bc: mov qword ptr [rbp - 0x38], 0x46
1416126c4: lea rax, [rbp - 0x40]
1416126c8: mov qword ptr [rbp + 0x68], rax
1416126cc: lea rax, [rip - 0x1508843]
1416126d3: mov qword ptr [rbp + 0x70], rax
1416126d7: lea rax, [rip + 0x166d0ea]
1416126de: mov qword ptr [rbp + 0x10], rax
1416126e2: mov qword ptr [rbp + 0x18], 1
1416126ea: mov qword ptr [rbp + 0x30], 0
1416126f2: lea rax, [rbp + 0x68]
1416126f6: mov qword ptr [rbp + 0x20], rax
1416126fa: mov qword ptr [rbp + 0x28], 1
141612702: lea rdx, [rip + 0x166d0cf]
141612709: lea rcx, [rbp + 0x10]
14161270d: call 0x1428d9390
141612712: xor r14d, r14d
141612715: lea r15, [rbp + 0x10]
141612719: mov edx, 0x1b0
14161271e: mov rcx, r15
141612721: mov r8d, 0x22
141612727: mov r9b, 1
14161272a: call 0x140bd4de0
14161272f: lea r12, [rbp + 0x68]
141612733: mov r8d, 4
141612739: mov rcx, r12
14161273c: mov rdx, r15
14161273f: call 0x140bd4f20
141612744: mov qword ptr [rsp + 0x30], r14
141612749: mov qword ptr [rsp + 0x28], rbx
14161274e: mov qword ptr [rsp + 0x20], r12
141612753: lea r8, [rip + 0x166d546]
14161275a: lea rcx, [rbp - 0x40]
14161275e: mov r9d, 0x21
141612764: mov rdx, rdi
141612767: call 0x1416a2dc0
14161276c: cmp dword ptr [rbp - 0x40], 0xb
141612770: jne 0x1416127d9
141612772: mov rax, qword ptr [rbp - 0x18]
141612776: mov qword ptr [rsi + 0x20], rax
14161277a: movups xmm0, xmmword ptr [rbp - 0x38]
14161277e: movups xmm1, xmmword ptr [rbp - 0x28]
141612782: movups xmmword ptr [rsi + 0x10], xmm1
141612786: movups xmmword ptr [rsi], xmm0
141612789: mov rax, rsi
14161278c: add rsp, 0x100
141612793: pop rbx
141612794: pop rdi
141612795: pop rsi
141612796: pop r12
141612798: pop r14
14161279a: pop r15
14161279c: pop rbp
14161279d: ret 
14161279e: mov qword ptr [rbp + 0x10], 0
1416127a6: lea rax, [rip + 0x16b779b]
1416127ad: jmp 0x1416127be
1416127af: mov qword ptr [rbp + 0x10], 0
1416127b7: lea rax, [rip + 0x16b77a2]
1416127be: mov qword ptr [rsp + 0x20], rax
1416127c3: lea r8, [rip + 0x16b7776]
1416127ca: lea rdx, [rbp - 0x40]
1416127ce: lea r9, [rbp + 0x10]
1416127d2: xor ecx, ecx
1416127d4: call 0x1428d9596
1416127d9: movups xmm0, xmmword ptr [rbp]
1416127dd: movaps xmmword ptr [rbp + 0x50], xmm0
1416127e1: movups xmm0, xmmword ptr [rbp - 0x40]
1416127e5: movups xmm1, xmmword ptr [rbp - 0x30]
1416127e9: movups xmm2, xmmword ptr [rbp - 0x20]
1416127ed: movups xmm3, xmmword ptr [rbp - 0x10]
1416127f1: movaps xmmword ptr [rbp + 0x40], xmm3
1416127f5: movaps xmmword ptr [rbp + 0x30], xmm2
1416127f9: movaps xmmword ptr [rbp + 0x20], xmm1
1416127fd: movaps xmmword ptr [rbp + 0x10], xmm0
141612801: lea rax, [rip + 0x166d9a0]
141612808: mov qword ptr [rsp + 0x20], rax
14161280d: lea rcx, [rip + 0x166c2d4]
141612814: lea r9, [rip + 0x166c2ad]
14161281b: lea r8, [rbp + 0x10]
14161281f: mov edx, 0x2b
141612824: call 0x1428d9760
141612829: ud2 
14161282b: nop dword ptr [rax + rax]
141612830: mov qword ptr [rsp + 0x10], rdx
141612835: push rbp
141612836: push r15
141612838: push r14
14161283a: push r12
14161283c: push rsi
14161283d: push rdi
14161283e: push rbx
14161283f: sub rsp, 0x40
141612843: lea rbp, [rdx + 0x80]
14161284a: lea rcx, [rbp + 0x10]
14161284e: call 0x1407cea70
141612853: nop 
141612854: add rsp, 0x40
141612858: pop rbx
141612859: pop rdi
14161285a: pop rsi
14161285b: pop r12
14161285d: pop r14
14161285f: pop r15
141612861: pop rbp
141612862: ret 
