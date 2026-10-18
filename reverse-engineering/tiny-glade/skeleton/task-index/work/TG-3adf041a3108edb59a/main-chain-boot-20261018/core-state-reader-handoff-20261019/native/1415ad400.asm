1415ad400: 55  push rbp
1415ad401: 4156  push r14
1415ad403: 56  push rsi
1415ad404: 57  push rdi
1415ad405: 53  push rbx
1415ad406: 4881ec80000000  sub rsp, 0x80
1415ad40d: 488dac2480000000  lea rbp, [rsp + 0x80]
1415ad415: 48c745f8feffffff  mov qword ptr [rbp - 8], 0xfffffffffffffffe
1415ad41d: 4889ce  mov rsi, rcx
1415ad420: 4c8b7210  mov r14, qword ptr [rdx + 0x10]
1415ad424: 488d4dd8  lea rcx, [rbp - 0x28]
1415ad428: e853ac5dff  call 0x140b88080
1415ad42d: 488b45e0  mov rax, qword ptr [rbp - 0x20]
1415ad431: 488b7df0  mov rdi, qword ptr [rbp - 0x10]
1415ad435: 4839f8  cmp rax, rdi
1415ad438: 743c  je 0x1415ad476
1415ad43a: 488d5820  lea rbx, [rax + 0x20]
1415ad43e: 48895de0  mov qword ptr [rbp - 0x20], rbx
1415ad442: 0fb608  movzx ecx, byte ptr [rax]
1415ad445: 80f906  cmp cl, 6
1415ad448: 742f  je 0x1415ad479
1415ad44a: 884da8  mov byte ptr [rbp - 0x58], cl
1415ad44d: 0f104001  movups xmm0, xmmword ptr [rax + 1]
1415ad451: 0f104810  movups xmm1, xmmword ptr [rax + 0x10]
1415ad455: 0f1145a9  movups xmmword ptr [rbp - 0x57], xmm0
1415ad459: 0f114db8  movups xmmword ptr [rbp - 0x48], xmm1
1415ad45d: 488d4dc8  lea rcx, [rbp - 0x38]
1415ad461: 488d55a8  lea rdx, [rbp - 0x58]
1415ad465: e816cdfeff  call 0x14159a180
1415ad46a: 807dc801  cmp byte ptr [rbp - 0x38], 1
1415ad46e: 7564  jne 0x1415ad4d4
1415ad470: 488b45d0  mov rax, qword ptr [rbp - 0x30]
1415ad474: eb18  jmp 0x1415ad48e
1415ad476: 4889c3  mov rbx, rax
1415ad479: 488d15a0da6c01  lea rdx, [rip + 0x16cdaa0]
1415ad480: 4c8d05d9cb6c01  lea r8, [rip + 0x16ccbd9]
1415ad487: 31c9  xor ecx, ecx
1415ad489: e892c83401  call 0x1428f9d20
1415ad48e: 48894608  mov qword ptr [rsi + 8], rax
1415ad492: c60601  mov byte ptr [rsi], 1
1415ad495: 4829df  sub rdi, rbx
1415ad498: 48c1ef05  shr rdi, 5
1415ad49c: 4889d9  mov rcx, rbx
1415ad49f: 4889fa  mov rdx, rdi
1415ad4a2: e8c9b82dff  call 0x140888d70
1415ad4a7: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415ad4ab: 4885d2  test rdx, rdx
1415ad4ae: 7413  je 0x1415ad4c3
1415ad4b0: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415ad4b4: 48c1e205  shl rdx, 5
1415ad4b8: 41b808000000  mov r8d, 8
1415ad4be: e85d6706ff  call 0x140613c20
1415ad4c3: 4889f0  mov rax, rsi
1415ad4c6: 4881c480000000  add rsp, 0x80
1415ad4cd: 5b  pop rbx
1415ad4ce: 5f  pop rdi
1415ad4cf: 5e  pop rsi
1415ad4d0: 415e  pop r14
1415ad4d2: 5d  pop rbp
1415ad4d3: c3  ret 
1415ad4d4: 4829df  sub rdi, rbx
1415ad4d7: 751c  jne 0x1415ad4f5
1415ad4d9: 0fb645c9  movzx eax, byte ptr [rbp - 0x37]
1415ad4dd: 884601  mov byte ptr [rsi + 1], al
1415ad4e0: 31c0  xor eax, eax
1415ad4e2: 8806  mov byte ptr [rsi], al
1415ad4e4: 48c1ef05  shr rdi, 5
1415ad4e8: 4889d9  mov rcx, rbx
1415ad4eb: 4889fa  mov rdx, rdi
1415ad4ee: e87db82dff  call 0x140888d70
1415ad4f3: ebb2  jmp 0x1415ad4a7
1415ad4f5: 488d1554cb6c01  lea rdx, [rip + 0x16ccb54]
1415ad4fc: 4c8d055dcb6c01  lea r8, [rip + 0x16ccb5d]
1415ad503: 4c89f1  mov rcx, r14
1415ad506: e815c83401  call 0x1428f9d20
1415ad50b: 48894608  mov qword ptr [rsi + 8], rax
1415ad50f: b001  mov al, 1
1415ad511: ebcf  jmp 0x1415ad4e2
1415ad513: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415ad520: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ad525: 55  push rbp
1415ad526: 4156  push r14
1415ad528: 56  push rsi
1415ad529: 57  push rdi
1415ad52a: 53  push rbx
1415ad52b: 4883ec20  sub rsp, 0x20
1415ad52f: 488daa80000000  lea rbp, [rdx + 0x80]
1415ad536: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415ad53a: 4885d2  test rdx, rdx
1415ad53d: 7413  je 0x1415ad552
1415ad53f: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415ad543: 48c1e205  shl rdx, 5
1415ad547: 41b808000000  mov r8d, 8
1415ad54d: e8ce6606ff  call 0x140613c20
1415ad552: 90  nop 
1415ad553: 4883c420  add rsp, 0x20
1415ad557: 5b  pop rbx
1415ad558: 5f  pop rdi
1415ad559: 5e  pop rsi
1415ad55a: 415e  pop r14
1415ad55c: 5d  pop rbp
1415ad55d: c3  ret 
1415ad55e: 6690  nop 
1415ad560: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ad565: 55  push rbp
1415ad566: 4156  push r14
1415ad568: 56  push rsi
1415ad569: 57  push rdi
1415ad56a: 53  push rbx
1415ad56b: 4883ec20  sub rsp, 0x20
1415ad56f: 488daa80000000  lea rbp, [rdx + 0x80]
1415ad576: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415ad57a: 4885d2  test rdx, rdx
1415ad57d: 7413  je 0x1415ad592
1415ad57f: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415ad583: 48c1e205  shl rdx, 5
1415ad587: 41b808000000  mov r8d, 8
1415ad58d: e88e6606ff  call 0x140613c20
1415ad592: 90  nop 
1415ad593: 4883c420  add rsp, 0x20
1415ad597: 5b  pop rbx
1415ad598: 5f  pop rdi
1415ad599: 5e  pop rsi
1415ad59a: 415e  pop r14
1415ad59c: 5d  pop rbp
1415ad59d: c3  ret 
1415ad59e: 6690  nop 
1415ad5a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ad5a5: 55  push rbp
1415ad5a6: 4156  push r14
1415ad5a8: 56  push rsi
1415ad5a9: 57  push rdi
1415ad5aa: 53  push rbx
1415ad5ab: 4883ec20  sub rsp, 0x20
1415ad5af: 488daa80000000  lea rbp, [rdx + 0x80]
1415ad5b6: 488d4dd8  lea rcx, [rbp - 0x28]
1415ad5ba: e8a1cf2dff  call 0x14088a560
1415ad5bf: 90  nop 
1415ad5c0: 4883c420  add rsp, 0x20
1415ad5c4: 5b  pop rbx
1415ad5c5: 5f  pop rdi
1415ad5c6: 5e  pop rsi
1415ad5c7: 415e  pop r14
1415ad5c9: 5d  pop rbp
1415ad5ca: c3  ret 
