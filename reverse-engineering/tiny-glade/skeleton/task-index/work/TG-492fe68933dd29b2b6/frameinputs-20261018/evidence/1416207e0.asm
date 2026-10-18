1416207e0: push rbp
1416207e1: push rsi
1416207e2: push rdi
1416207e3: push rbx
1416207e4: sub rsp, 0x208
1416207eb: lea rbp, [rsp + 0x80]
1416207f3: mov qword ptr [rbp + 0x180], 0xfffffffffffffffe
1416207fe: mov rdi, rdx
141620801: mov rsi, rcx
141620804: test r8, r8
141620807: je 0x141620881
141620809: movabs rcx, 0x97b425ed097b425f
141620813: mov rax, r9
141620816: mul rcx
141620819: mov rbx, rdx
14162081c: shr rbx, 8
141620820: imul rax, rbx, 0x1b0
141620827: sub r9, rax
14162082a: mov qword ptr [rbp + 0x170], r9
141620831: jne 0x14162096a
141620837: mov rax, r8
14162083a: and rax, 3
14162083e: mov qword ptr [rbp + 0x170], rax
141620845: jne 0x14162097b
14162084b: mov byte ptr [rbp + 0x17f], 0
141620852: lea rcx, [rbp - 0x50]
141620856: mov rdx, rdi
141620859: mov r9, rbx
14162085c: call 0x1416154c0
141620861: mov rax, qword ptr [rbp - 0x30]
141620865: mov qword ptr [rsi + 0x28], rax
141620869: movups xmm0, xmmword ptr [rbp - 0x50]
14162086d: movups xmm1, xmmword ptr [rbp - 0x40]
141620871: movups xmmword ptr [rsi + 0x18], xmm1
141620875: movups xmmword ptr [rsi + 8], xmm0
141620879: mov qword ptr [rsi], rbx
14162087c: jmp 0x14162095b
141620881: mov byte ptr [rbp + 0x17f], 0
141620888: lea rcx, [rbp - 0x50]
14162088c: call 0x1408bb650
141620891: cmp dword ptr [rbp - 0x50], 1
141620895: jne 0x141620926
14162089b: movzx eax, byte ptr [rip + 0x2014367]
1416208a2: mov ecx, 0x1b0
1416208a7: mov edx, 4
1416208ac: call 0x140613c10
1416208b1: test rax, rax
1416208b4: je 0x1416209a8
1416208ba: mov rbx, rax
1416208bd: lea rdx, [rbp - 0x4c]
1416208c1: mov r8d, 0x1b0
1416208c7: mov rcx, rax
1416208ca: call 0x142923c00
1416208cf: mov qword ptr [rbp + 0x168], rbx
1416208d6: mov byte ptr [rbp + 0x17f], 1
1416208dd: lea rcx, [rbp - 0x50]
1416208e1: mov r9d, 1
1416208e7: mov rdx, rdi
1416208ea: mov r8, rbx
1416208ed: call 0x1416154c0
1416208f2: mov rax, qword ptr [rbp - 0x30]
1416208f6: mov qword ptr [rsi + 0x28], rax
1416208fa: movups xmm0, xmmword ptr [rbp - 0x50]
1416208fe: movups xmm1, xmmword ptr [rbp - 0x40]
141620902: movups xmmword ptr [rsi + 0x18], xmm1
141620906: movups xmmword ptr [rsi + 8], xmm0
14162090a: mov qword ptr [rsi], 1
141620911: mov edx, 0x1b0
141620916: mov r8d, 4
14162091c: mov rcx, rbx
14162091f: call 0x140613c20
141620924: jmp 0x14162095b
141620926: mov byte ptr [rbp + 0x17f], 0
14162092d: lea rcx, [rbp - 0x50]
141620931: mov rdx, rdi
141620934: xor r8d, r8d
141620937: call 0x1416154c0
14162093c: mov rax, qword ptr [rbp - 0x30]
141620940: mov qword ptr [rsi + 0x28], rax
141620944: movups xmm0, xmmword ptr [rbp - 0x50]
141620948: movups xmm1, xmmword ptr [rbp - 0x40]
14162094c: movups xmmword ptr [rsi + 0x18], xmm1
141620950: movups xmmword ptr [rsi + 8], xmm0
141620954: mov qword ptr [rsi], 0
14162095b: mov rax, rsi
14162095e: add rsp, 0x208
141620965: pop rbx
141620966: pop rdi
141620967: pop rsi
141620968: pop rbp
141620969: ret 
14162096a: mov qword ptr [rbp - 0x50], 0
141620972: lea rax, [rip + 0x16a95cf]
141620979: jmp 0x14162098a
14162097b: mov qword ptr [rbp - 0x50], 0
141620983: lea rax, [rip + 0x16a95d6]
14162098a: mov qword ptr [rsp + 0x20], rax
14162098f: lea r8, [rip + 0x16a95aa]
141620996: lea rdx, [rbp + 0x170]
14162099d: lea r9, [rbp - 0x50]
1416209a1: xor ecx, ecx
1416209a3: call 0x1428d9596
1416209a8: mov byte ptr [rbp + 0x17f], 0
1416209af: lea r8, [rip + 0x165f8c2]
1416209b6: mov ecx, 4
1416209bb: mov edx, 0x1b0
1416209c0: call 0x1428d8fe3
1416209c5: ud2 
1416209c7: nop word ptr [rax + rax]
1416209d0: mov qword ptr [rsp + 0x10], rdx
1416209d5: push rbp
1416209d6: push rsi
1416209d7: push rdi
1416209d8: push rbx
1416209d9: sub rsp, 0x28
1416209dd: lea rbp, [rdx + 0x80]
1416209e4: cmp byte ptr [rbp + 0x17f], 0
1416209eb: je 0x141620a04
1416209ed: mov rcx, qword ptr [rbp + 0x168]
1416209f4: mov edx, 0x1b0
1416209f9: mov r8d, 4
1416209ff: call 0x140613c20
141620a04: nop 
141620a05: add rsp, 0x28
141620a09: pop rbx
141620a0a: pop rdi
141620a0b: pop rsi
141620a0c: pop rbp
141620a0d: ret 
