0x14067e560 push r14
0x14067e562 push rsi
0x14067e563 push rdi
0x14067e564 push rbx
0x14067e565 sub rsp, 0x28
0x14067e569 mov rsi, rdx
0x14067e56c mov rdi, rcx
0x14067e56f mov rbx, qword ptr [rdx + 8]
0x14067e573 mov r14, qword ptr [rdx + 0x10]
0x14067e577 mov ecx, dword ptr [rcx + 0x258]
0x14067e57d movss xmm1, dword ptr [rdx + 0x18]
0x14067e582 call 0x140c918d0
0x14067e587 mov rcx, qword ptr [rsi]
0x14067e58a mov qword ptr [rsp + 0x20], rcx
0x14067e58f mov rcx, rdi
0x14067e592 mov rdx, rbx
0x14067e595 mov r8, r14
0x14067e598 mov r9d, eax
0x14067e59b call 0x140b91120
0x14067e5a0 nop 
0x14067e5a1 add rsp, 0x28
0x14067e5a5 pop rbx
0x14067e5a6 pop rdi
0x14067e5a7 pop rsi
0x14067e5a8 pop r14
0x14067e5aa ret 
