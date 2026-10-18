0x141e5e490 push rbp
0x141e5e491 push r15
0x141e5e493 push r14
0x141e5e495 push r13
0x141e5e497 push r12
0x141e5e499 push rsi
0x141e5e49a push rdi
0x141e5e49b push rbx
0x141e5e49c sub rsp, 0x58
0x141e5e4a0 lea rbp, [rsp + 0x50]
0x141e5e4a5 movdqa xmmword ptr [rbp - 0x10], xmm6
0x141e5e4aa mov qword ptr [rbp - 0x18], 0xfffffffffffffffe
0x141e5e4b2 mov rax, qword ptr [r8]
0x141e5e4b5 mov qword ptr [rbp - 0x20], rax
0x141e5e4b9 mov r9, qword ptr [r8 + 8]
0x141e5e4bd cmp rcx, rdx
0x141e5e4c0 je 0x141e5e617
0x141e5e4c6 mov rsi, rdx
0x141e5e4c9 mov rdi, rcx
0x141e5e4cc mov r15, qword ptr [r8 + 0x10]
0x141e5e4d0 mov r12, qword ptr [r8 + 0x18]
0x141e5e4d4 sub rsi, rcx
0x141e5e4d7 shr rsi, 3
0x141e5e4db lea rbx, [r12 + 0x20]
0x141e5e4e0 xor r13d, r13d
0x141e5e4e3 pcmpeqd xmm6, xmm6
0x141e5e4e7 jmp 0x141e5e5a9
0x141e5e4ec nop dword ptr [rax]
0x141e5e4f0 movdqu xmm1, xmmword ptr [rcx + r8]
0x141e5e4f6 movdqa xmm2, xmm1
0x141e5e4fa pcmpeqb xmm2, xmm0
0x141e5e4fe pmovmskb r10d, xmm2
0x141e5e503 test r10d, r10d
0x141e5e506 je 0x141e5e540
0x141e5e508 tzcnt r11d, r10d
0x141e5e50d add r11, r8
0x141e5e510 and r11, rdx
0x141e5e513 neg r11
0x141e5e516 lea r11, [r11 + r11*2]
0x141e5e51a shl r11, 6
0x141e5e51e cmp r14, qword ptr [rax + r11]
0x141e5e522 je 0x141e5e570
0x141e5e524 lea r11d, [r10 - 1]
0x141e5e528 and r11w, r10w
0x141e5e52c mov r10d, r11d
0x141e5e52f jne 0x141e5e508
0x141e5e531 nop word ptr cs:[rax + rax]
0x141e5e540 pcmpeqb xmm1, xmm6
0x141e5e544 pmovmskb r10d, xmm1
0x141e5e549 test r10d, r10d
0x141e5e54c jne 0x141e5e5fd
0x141e5e552 add r8, r9
0x141e5e555 add r8, 0x10
0x141e5e559 add r9, 0x10
0x141e5e55d and r8, rdx
0x141e5e560 jmp 0x141e5e4f0
0x141e5e562 nop word ptr cs:[rax + rax]
0x141e5e570 lea rax, [rcx + r11]
0x141e5e574 add rax, -0xa0
0x141e5e57a lea rdx, [rcx + r11 - 0x58]
0x141e5e57f mov r9, qword ptr [rbp - 0x28]
0x141e5e583 lea r8, [r9 + r9*2]
0x141e5e587 movq xmm0, qword ptr [rcx + r11 - 0x48]
0x141e5e58e mov qword ptr [r15 + r8*8], rax
0x141e5e592 mov qword ptr [r15 + r8*8 + 8], rdx
0x141e5e597 movq qword ptr [r15 + r8*8 + 0x10], xmm0
0x141e5e59e inc r9
0x141e5e5a1 inc r13
0x141e5e5a4 cmp r13, rsi
0x141e5e5a7 je 0x141e5e617
0x141e5e5a9 mov qword ptr [rbp - 0x28], r9
0x141e5e5ad mov r14, qword ptr [rdi + r13*8]
0x141e5e5b1 mov qword ptr [rbp - 0x30], r14
0x141e5e5b5 cmp qword ptr [r12 + 0x18], 0
0x141e5e5bb je 0x141e5e5fd
0x141e5e5bd mov rcx, rbx
0x141e5e5c0 lea rdx, [rbp - 0x30]
0x141e5e5c4 call 0x1405b0d70
0x141e5e5c9 mov rcx, qword ptr [r12]
0x141e5e5cd mov rdx, qword ptr [r12 + 8]
0x141e5e5d2 mov r8, rdx
0x141e5e5d5 and r8, rax
0x141e5e5d8 shr rax, 0x39
0x141e5e5dc movd xmm0, eax
0x141e5e5e0 punpcklbw xmm0, xmm0
0x141e5e5e4 pshuflw xmm0, xmm0, 0
0x141e5e5e9 pshufd xmm0, xmm0, 0
0x141e5e5ee lea rax, [rcx - 0xc0]
0x141e5e5f5 xor r9d, r9d
0x141e5e5f8 jmp 0x141e5e4f0
0x141e5e5fd lea rcx, [rip + 0x1134dfc]
0x141e5e604 lea r8, [rip + 0x1134e35]
0x141e5e60b mov edx, 0x16
0x141e5e610 call 0x1428d9330
0x141e5e615 ud2 
0x141e5e617 mov rax, qword ptr [rbp - 0x20]
0x141e5e61b mov qword ptr [rax], r9
0x141e5e61e movaps xmm6, xmmword ptr [rbp - 0x10]
0x141e5e622 add rsp, 0x58
0x141e5e626 pop rbx
0x141e5e627 pop rdi
0x141e5e628 pop rsi
0x141e5e629 pop r12
0x141e5e62b pop r13
0x141e5e62d pop r14
0x141e5e62f pop r15
0x141e5e631 pop rbp
0x141e5e632 ret 
0x141e5e633 nop word ptr cs:[rax + rax]
0x141e5e640 mov qword ptr [rsp + 0x10], rdx
0x141e5e645 push rbp
0x141e5e646 push r15
0x141e5e648 push r14
0x141e5e64a push r13
0x141e5e64c push r12
0x141e5e64e push rsi
0x141e5e64f push rdi
0x141e5e650 push rbx
0x141e5e651 sub rsp, 0x38
0x141e5e655 lea rbp, [rdx + 0x50]
0x141e5e659 movdqa xmmword ptr [rsp + 0x20], xmm6
0x141e5e65f mov rax, qword ptr [rbp - 0x20]
0x141e5e663 mov rcx, qword ptr [rbp - 0x28]
0x141e5e667 mov qword ptr [rax], rcx
0x141e5e66a movaps xmm6, xmmword ptr [rsp + 0x20]
0x141e5e66f add rsp, 0x38
0x141e5e673 pop rbx
0x141e5e674 pop rdi
0x141e5e675 pop rsi
0x141e5e676 pop r12
0x141e5e678 pop r13
0x141e5e67a pop r14
0x141e5e67c pop r15
0x141e5e67e pop rbp
0x141e5e67f ret 
