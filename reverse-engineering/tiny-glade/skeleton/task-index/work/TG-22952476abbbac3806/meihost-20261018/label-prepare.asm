0x14067e510 push rsi
0x14067e511 push rdi
0x14067e512 sub rsp, 0x28
0x14067e516 mov rsi, rdx
0x14067e519 mov rdi, rcx
0x14067e51c mov rdx, qword ptr [r8]
0x14067e51f mov rcx, rsi
0x14067e522 call 0x140b90f60
0x14067e527 mov rcx, rsi
0x14067e52a call 0x140b90f50
0x14067e52f movss xmm0, dword ptr [rax]
0x14067e533 movss xmm1, dword ptr [rax + 4]
0x14067e538 mov dword ptr [rdi], 0
0x14067e53e movss dword ptr [rdi + 4], xmm0
0x14067e543 mov dword ptr [rdi + 8], 0
0x14067e54a movss dword ptr [rdi + 0xc], xmm1
0x14067e54f mov rax, rdi
0x14067e552 add rsp, 0x28
0x14067e556 pop rdi
0x14067e557 pop rsi
0x14067e558 ret 
