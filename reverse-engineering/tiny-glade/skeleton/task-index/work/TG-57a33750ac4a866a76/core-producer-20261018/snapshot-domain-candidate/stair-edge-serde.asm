14157d500: 55  push rbp
14157d501: 4156  push r14
14157d503: 56  push rsi
14157d504: 57  push rdi
14157d505: 53  push rbx
14157d506: 4883ec60  sub rsp, 0x60
14157d50a: 488d6c2460  lea rbp, [rsp + 0x60]
14157d50f: 48c745f8feffffff  mov qword ptr [rbp - 8], 0xfffffffffffffffe
14157d517: 4889d7  mov rdi, rdx
14157d51a: 4889ce  mov rsi, rcx
14157d51d: 488d1534a26f01  lea rdx, [rip + 0x16fa234]
14157d524: 488d4dd8  lea rcx, [rbp - 0x28]
14157d528: 41b80e000000  mov r8d, 0xe
14157d52e: 41b902000000  mov r9d, 2
14157d534: e8678e60ff  call 0x140b863a0
14157d539: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
14157d53d: 488b45e0  mov rax, qword ptr [rbp - 0x20]
14157d541: 4889ca  mov rdx, rcx
14157d544: 48f7da  neg rdx
14157d547: 710c  jno 0x14157d555
14157d549: 48894608  mov qword ptr [rsi + 8], rax
14157d54d: c60606  mov byte ptr [rsi], 6
14157d550: e9ef000000  jmp 0x14157d644
14157d555: 488b5de8  mov rbx, qword ptr [rbp - 0x18]
14157d559: 48894dc0  mov qword ptr [rbp - 0x40], rcx
14157d55d: 488945c8  mov qword ptr [rbp - 0x38], rax
14157d561: 48895dd0  mov qword ptr [rbp - 0x30], rbx
14157d565: 8b17  mov edx, dword ptr [rdi]
14157d567: c645d802  mov byte ptr [rbp - 0x28], 2
14157d56b: 48c745e000000000  mov qword ptr [rbp - 0x20], 0
14157d573: 488955e8  mov qword ptr [rbp - 0x18], rdx
14157d577: 4839cb  cmp rbx, rcx
14157d57a: 7518  jne 0x14157d594
14157d57c: 488d1515ad6f01  lea rdx, [rip + 0x16fad15]
14157d583: 488d4dc0  lea rcx, [rbp - 0x40]
14157d587: e8d44a60ff  call 0x140b82060
14157d58c: 488b4dc0  mov rcx, qword ptr [rbp - 0x40]
14157d590: 488b45c8  mov rax, qword ptr [rbp - 0x38]
14157d594: 4889da  mov rdx, rbx
14157d597: 48c1e205  shl rdx, 5
14157d59b: 0f1045d8  movups xmm0, xmmword ptr [rbp - 0x28]
14157d59f: 0f104de8  movups xmm1, xmmword ptr [rbp - 0x18]
14157d5a3: 0f114c1010  movups xmmword ptr [rax + rdx + 0x10], xmm1
14157d5a8: 0f110410  movups xmmword ptr [rax + rdx], xmm0
14157d5ac: 4c8d7301  lea r14, [rbx + 1]
14157d5b0: 4c8975d0  mov qword ptr [rbp - 0x30], r14
14157d5b4: 8b5704  mov edx, dword ptr [rdi + 4]
14157d5b7: c645d802  mov byte ptr [rbp - 0x28], 2
14157d5bb: 48c745e000000000  mov qword ptr [rbp - 0x20], 0
14157d5c3: 488955e8  mov qword ptr [rbp - 0x18], rdx
14157d5c7: 4939ce  cmp r14, rcx
14157d5ca: 7514  jne 0x14157d5e0
14157d5cc: 488d15c5ac6f01  lea rdx, [rip + 0x16facc5]
14157d5d3: 488d4dc0  lea rcx, [rbp - 0x40]
14157d5d7: e8844a60ff  call 0x140b82060
14157d5dc: 488b45c8  mov rax, qword ptr [rbp - 0x38]
14157d5e0: 49c1e605  shl r14, 5
14157d5e4: 0f1045d8  movups xmm0, xmmword ptr [rbp - 0x28]
14157d5e8: 0f104de8  movups xmm1, xmmword ptr [rbp - 0x18]
14157d5ec: 420f114c3010  movups xmmword ptr [rax + r14 + 0x10], xmm1
14157d5f2: 420f110430  movups xmmword ptr [rax + r14], xmm0
14157d5f7: 4883c302  add rbx, 2
14157d5fb: 48895dd0  mov qword ptr [rbp - 0x30], rbx
14157d5ff: 48895def  mov qword ptr [rbp - 0x11], rbx
14157d603: 0f1045c0  movups xmm0, xmmword ptr [rbp - 0x40]
14157d607: 0f1145df  movups xmmword ptr [rbp - 0x21], xmm0
14157d60b: c60604  mov byte ptr [rsi], 4
14157d60e: 0fb645d8  movzx eax, byte ptr [rbp - 0x28]
14157d612: 8b4dd9  mov ecx, dword ptr [rbp - 0x27]
14157d615: 0fb755dd  movzx edx, word ptr [rbp - 0x23]
14157d619: 440fb645df  movzx r8d, byte ptr [rbp - 0x21]
14157d61e: 4c8b4de0  mov r9, qword ptr [rbp - 0x20]
14157d622: 4c8b55e7  mov r10, qword ptr [rbp - 0x19]
14157d626: 4c8b5def  mov r11, qword ptr [rbp - 0x11]
14157d62a: 884601  mov byte ptr [rsi + 1], al
14157d62d: 894e02  mov dword ptr [rsi + 2], ecx
14157d630: 66895606  mov word ptr [rsi + 6], dx
14157d634: 44884608  mov byte ptr [rsi + 8], r8b
14157d638: 4c894e09  mov qword ptr [rsi + 9], r9
14157d63c: 4c895610  mov qword ptr [rsi + 0x10], r10
14157d640: 4c895e18  mov qword ptr [rsi + 0x18], r11
14157d644: 4889f0  mov rax, rsi
14157d647: 4883c460  add rsp, 0x60
14157d64b: 5b  pop rbx
14157d64c: 5f  pop rdi
14157d64d: 5e  pop rsi
14157d64e: 415e  pop r14
14157d650: 5d  pop rbp
14157d651: c3  ret
14157d652: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
14157d660: 4889542410  mov qword ptr [rsp + 0x10], rdx
14157d665: 55  push rbp
14157d666: 4156  push r14
14157d668: 56  push rsi
14157d669: 57  push rdi
14157d66a: 53  push rbx
14157d66b: 4883ec20  sub rsp, 0x20
14157d66f: 488d6a60  lea rbp, [rdx + 0x60]
14157d673: 488d4dc0  lea rcx, [rbp - 0x40]
14157d677: e814bd30ff  call 0x140889390
14157d67c: 90  nop
14157d67d: 4883c420  add rsp, 0x20
14157d681: 5b  pop rbx
14157d682: 5f  pop rdi
14157d683: 5e  pop rsi
14157d684: 415e  pop r14
14157d686: 5d  pop rbp
14157d687: c3  ret
14157d688: 0f1f840000000000  nop dword ptr [rax + rax]
14157d690: 4889542410  mov qword ptr [rsp + 0x10], rdx
14157d695: 55  push rbp
14157d696: 4156  push r14
14157d698: 56  push rsi
14157d699: 57  push rdi
14157d69a: 53  push rbx
14157d69b: 4883ec20  sub rsp, 0x20
14157d69f: 488d6a60  lea rbp, [rdx + 0x60]
14157d6a3: 488d4dd8  lea rcx, [rbp - 0x28]
14157d6a7: e8e4032eff  call 0x14085da90
14157d6ac: 90  nop
14157d6ad: 4883c420  add rsp, 0x20
14157d6b1: 5b  pop rbx
14157d6b2: 5f  pop rdi
14157d6b3: 5e  pop rsi
14157d6b4: 415e  pop r14
14157d6b6: 5d  pop rbp
14157d6b7: c3  ret
14157d6b8: 0f1f840000000000  nop dword ptr [rax + rax]
14157d6c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
14157d6c5: 55  push rbp
14157d6c6: 4156  push r14
14157d6c8: 56  push rsi
14157d6c9: 57  push rdi
14157d6ca: 53  push rbx
14157d6cb: 4883ec20  sub rsp, 0x20
14157d6cf: 488d6a60  lea rbp, [rdx + 0x60]
14157d6d3: 488d4dd8  lea rcx, [rbp - 0x28]
14157d6d7: e8b4032eff  call 0x14085da90
14157d6dc: 90  nop
14157d6dd: 4883c420  add rsp, 0x20
14157d6e1: 5b  pop rbx
14157d6e2: 5f  pop rdi
14157d6e3: 5e  pop rsi
14157d6e4: 415e  pop r14
14157d6e6: 5d  pop rbp
14157d6e7: c3  ret
