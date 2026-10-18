0x140965980 55 push rbp
0x140965981 4157 push r15
0x140965983 4156 push r14
0x140965985 4155 push r13
0x140965987 4154 push r12
0x140965989 56 push rsi
0x14096598a 57 push rdi
0x14096598b 53 push rbx
0x14096598c 4883ec78 sub rsp, 0x78
0x140965990 488d6c2470 lea rbp, [rsp + 0x70]
0x140965995 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
0x14096599d 4889ce mov rsi, rcx
0x1409659a0 488955d8 mov qword ptr [rbp - 0x28], rdx
0x1409659a4 448845e0 mov byte ptr [rbp - 0x20], r8b
0x1409659a8 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1409659b0 48c745f004000000 mov qword ptr [rbp - 0x10], 4
0x1409659b8 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1409659c0 4c8d65b4 lea r12, [rbp - 0x4c]
0x1409659c4 488d7db0 lea rdi, [rbp - 0x50]
0x1409659c8 488d5dd8 lea rbx, [rbp - 0x28]
0x1409659cc 4c8d35cdf91402 lea r14, [rip + 0x214f9cd]
0x1409659d3 4c8d7de8 lea r15, [rbp - 0x18]
0x1409659d7 eb3a jmp 0x140965a13
0x1409659d9 0f1f8000000000 nop dword ptr [rax]
0x1409659e0 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x1409659e4 4a8d0ced00000000 lea rcx, [r13*8]
0x1409659ec 4c01e9 add rcx, r13
0x1409659ef 418b542420 mov edx, dword ptr [r12 + 0x20]
0x1409659f4 89548820 mov dword ptr [rax + rcx*4 + 0x20], edx
0x1409659f8 410f100424 movups xmm0, xmmword ptr [r12]
0x1409659fd 410f104c2410 movups xmm1, xmmword ptr [r12 + 0x10]
0x140965a03 0f114c8810 movups xmmword ptr [rax + rcx*4 + 0x10], xmm1
0x140965a08 0f110488 movups xmmword ptr [rax + rcx*4], xmm0
0x140965a0c 49ffc5 inc r13
0x140965a0f 4c896df8 mov qword ptr [rbp - 8], r13
0x140965a13 4889f9 mov rcx, rdi
0x140965a16 4889da mov rdx, rbx
0x140965a19 e85263aa00 call 0x14140bd70
0x140965a1e 8b45b0 mov eax, dword ptr [rbp - 0x50]
0x140965a21 83f801 cmp eax, 1
0x140965a24 7517 jne 0x140965a3d
0x140965a26 4c8b6df8 mov r13, qword ptr [rbp - 8]
0x140965a2a 4c3b6de8 cmp r13, qword ptr [rbp - 0x18]
0x140965a2e 75b0 jne 0x1409659e0
0x140965a30 4c89f9 mov rcx, r15
0x140965a33 4c89f2 mov rdx, r14
0x140965a36 e8f5a11f00 call 0x140b5fc30
0x140965a3b eba3 jmp 0x1409659e0
0x140965a3d 83f802 cmp eax, 2
0x140965a40 7537 jne 0x140965a79
0x140965a42 488b45b8 mov rax, qword ptr [rbp - 0x48]
0x140965a46 48894608 mov qword ptr [rsi + 8], rax
0x140965a4a 48b80000000000000080 movabs rax, 0x8000000000000000
0x140965a54 488906 mov qword ptr [rsi], rax
0x140965a57 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140965a5b 4885c0 test rax, rax
0x140965a5e 7428 je 0x140965a88
0x140965a60 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x140965a64 48c1e002 shl rax, 2
0x140965a68 488d14c0 lea rdx, [rax + rax*8]
0x140965a6c 41b804000000 mov r8d, 4
0x140965a72 e8a9e1caff call 0x140613c20
0x140965a77 eb0f jmp 0x140965a88
0x140965a79 488b45f8 mov rax, qword ptr [rbp - 8]
0x140965a7d 48894610 mov qword ptr [rsi + 0x10], rax
0x140965a81 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
0x140965a85 0f1106 movups xmmword ptr [rsi], xmm0
0x140965a88 4889f0 mov rax, rsi
0x140965a8b 4883c478 add rsp, 0x78
0x140965a8f 5b pop rbx
0x140965a90 5f pop rdi
0x140965a91 5e pop rsi
0x140965a92 415c pop r12
0x140965a94 415d pop r13
0x140965a96 415e pop r14
0x140965a98 415f pop r15
0x140965a9a 5d pop rbp
0x140965a9b c3 ret 
0x140965a9c 0f1f4000 nop dword ptr [rax]
0x140965aa0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140965aa5 55 push rbp
0x140965aa6 4157 push r15
0x140965aa8 4156 push r14
0x140965aaa 4155 push r13
0x140965aac 4154 push r12
0x140965aae 56 push rsi
0x140965aaf 57 push rdi
0x140965ab0 53 push rbx
0x140965ab1 4883ec28 sub rsp, 0x28
0x140965ab5 488d6a70 lea rbp, [rdx + 0x70]
0x140965ab9 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140965abd 4885c0 test rax, rax
0x140965ac0 7417 je 0x140965ad9
0x140965ac2 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x140965ac6 48c1e002 shl rax, 2
0x140965aca 488d14c0 lea rdx, [rax + rax*8]
0x140965ace 41b804000000 mov r8d, 4
0x140965ad4 e847e1caff call 0x140613c20
0x140965ad9 90 nop 
0x140965ada 4883c428 add rsp, 0x28
0x140965ade 5b pop rbx
0x140965adf 5f pop rdi
0x140965ae0 5e pop rsi
0x140965ae1 415c pop r12
0x140965ae3 415d pop r13
0x140965ae5 415e pop r14
0x140965ae7 415f pop r15
0x140965ae9 5d pop rbp
0x140965aea c3 ret 
