14146d90d: mov byte ptr [rbp + 0x976], 1
14146d914: lea rcx, [rbp + 0x360]
14146d91b: call 0x14087b2f0
14146d920: jmp 0x14146f625
14146d925: neg r12d
14146d928: lea rax, [r15 + 0x20]
14146d92c: mov qword ptr [rbp + 0x928], rax
14146d933: xor r13d, r13d
14146d936: jmp 0x14146d965
14146d938: nop dword ptr [rax + rax]
14146d940: inc r13d
14146d943: mov rax, qword ptr [r15 + 0x28]
14146d947: mov rcx, rdi
14146d94a: shl rcx, 4
14146d94e: mov qword ptr [rax + rcx], rsi
14146d952: mov dword ptr [rax + rcx + 8], 0x3010001
14146d95a: inc rdi
14146d95d: mov qword ptr [r15 + 0x30], rdi
14146d961: inc qword ptr [r15 + 0x40]
14146d965: cmp r13d, r12d
14146d968: jge 0x14146d9a8
14146d96a: mov dword ptr [r14], ebx
14146d96d: mov rdi, qword ptr [r15 + 0x30]
14146d971: mov rsi, qword ptr [r15 + 0x40]
14146d975: cmp rdi, qword ptr [r15 + 0x20]
