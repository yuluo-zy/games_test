14146d863: jmp 0x14146d9d8
14146d868: mov qword ptr [rbp + 0x900], r8
14146d86f: mov r12d, dword ptr [r8 + 0x4c]
14146d873: test r12d, r12d
14146d876: js 0x14146d925
14146d87c: lea rax, [r15 + 0x20]
14146d880: mov qword ptr [rbp + 0x928], rax
14146d887: xor r13d, r13d
14146d88a: jmp 0x14146d8b5
14146d88c: nop dword ptr [rax]
14146d890: inc r13d
14146d893: mov rax, qword ptr [r15 + 0x28]
14146d897: mov rcx, rdi
14146d89a: shl rcx, 4
14146d89e: mov qword ptr [rax + rcx], rsi
14146d8a2: mov dword ptr [rax + rcx + 8], 0x3010101
14146d8aa: inc rdi
14146d8ad: mov qword ptr [r15 + 0x30], rdi
14146d8b1: inc qword ptr [r15 + 0x40]
14146d8b5: cmp r13d, r12d
14146d8b8: jge 0x14146d9a8
14146d8be: mov dword ptr [r14], ebx
14146d8c1: mov rdi, qword ptr [r15 + 0x30]
14146d8c5: mov rsi, qword ptr [r15 + 0x40]
14146d8c9: cmp rdi, qword ptr [r15 + 0x20]
