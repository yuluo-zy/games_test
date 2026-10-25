0x140965af0 55 push rbp
0x140965af1 4157 push r15
0x140965af3 4156 push r14
0x140965af5 4155 push r13
0x140965af7 4154 push r12
0x140965af9 56 push rsi
0x140965afa 57 push rdi
0x140965afb 53 push rbx
0x140965afc 4883ec68 sub rsp, 0x68
0x140965b00 488d6c2460 lea rbp, [rsp + 0x60]
0x140965b05 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
0x140965b0d 4889ce mov rsi, rcx
0x140965b10 488955c0 mov qword ptr [rbp - 0x40], rdx
0x140965b14 448845c8 mov byte ptr [rbp - 0x38], r8b
0x140965b18 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x140965b20 48c745f004000000 mov qword ptr [rbp - 0x10], 4
0x140965b28 48c745f800000000 mov qword ptr [rbp - 8], 0
0x140965b30 4c8d65d8 lea r12, [rbp - 0x28]
0x140965b34 488d7dd0 lea rdi, [rbp - 0x30]
0x140965b38 488d5dc0 lea rbx, [rbp - 0x40]
0x140965b3c 4c8d355df81402 lea r14, [rip + 0x214f85d]
0x140965b43 4c8d7de8 lea r15, [rbp - 0x18]
0x140965b47 eb22 jmp 0x140965b6b
0x140965b49 0f1f8000000000 nop dword ptr [rax]
0x140965b50 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x140965b54 4c89e9 mov rcx, r13
0x140965b57 48c1e104 shl rcx, 4
0x140965b5b 410f100424 movups xmm0, xmmword ptr [r12]
0x140965b60 0f110408 movups xmmword ptr [rax + rcx], xmm0
0x140965b64 49ffc5 inc r13
0x140965b67 4c896df8 mov qword ptr [rbp - 8], r13
0x140965b6b 4889f9 mov rcx, rdi
0x140965b6e 4889da mov rdx, rbx
0x140965b71 e85a5baa00 call 0x14140b6d0
0x140965b76 f645d001 test byte ptr [rbp - 0x30], 1
0x140965b7a 751d jne 0x140965b99
0x140965b7c 837dd401 cmp dword ptr [rbp - 0x2c], 1
0x140965b80 754a jne 0x140965bcc
0x140965b82 4c8b6df8 mov r13, qword ptr [rbp - 8]
0x140965b86 4c3b6de8 cmp r13, qword ptr [rbp - 0x18]
0x140965b8a 75c4 jne 0x140965b50
0x140965b8c 4c89f9 mov rcx, r15
0x140965b8f 4c89f2 mov rdx, r14
0x140965b92 e83978d2ff call 0x14068d3d0
0x140965b97 ebb7 jmp 0x140965b50
0x140965b99 488b45d8 mov rax, qword ptr [rbp - 0x28]
0x140965b9d 48894608 mov qword ptr [rsi + 8], rax
0x140965ba1 48b80000000000000080 movabs rax, 0x8000000000000000
0x140965bab 488906 mov qword ptr [rsi], rax
0x140965bae 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x140965bb2 4885d2 test rdx, rdx
0x140965bb5 7424 je 0x140965bdb
0x140965bb7 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x140965bbb 48c1e204 shl rdx, 4
0x140965bbf 41b804000000 mov r8d, 4
0x140965bc5 e856e0caff call 0x140613c20
0x140965bca eb0f jmp 0x140965bdb
0x140965bcc 488b45f8 mov rax, qword ptr [rbp - 8]
0x140965bd0 48894610 mov qword ptr [rsi + 0x10], rax
0x140965bd4 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
0x140965bd8 0f1106 movups xmmword ptr [rsi], xmm0
0x140965bdb 4889f0 mov rax, rsi
0x140965bde 4883c468 add rsp, 0x68
0x140965be2 5b pop rbx
0x140965be3 5f pop rdi
0x140965be4 5e pop rsi
0x140965be5 415c pop r12
0x140965be7 415d pop r13
0x140965be9 415e pop r14
0x140965beb 415f pop r15
0x140965bed 5d pop rbp
0x140965bee c3 ret 
0x140965bef 90 nop 
0x140965bf0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140965bf5 55 push rbp
0x140965bf6 4157 push r15
0x140965bf8 4156 push r14
0x140965bfa 4155 push r13
0x140965bfc 4154 push r12
0x140965bfe 56 push rsi
0x140965bff 57 push rdi
0x140965c00 53 push rbx
0x140965c01 4883ec28 sub rsp, 0x28
0x140965c05 488d6a60 lea rbp, [rdx + 0x60]
0x140965c09 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x140965c0d 4885d2 test rdx, rdx
0x140965c10 7413 je 0x140965c25
0x140965c12 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x140965c16 48c1e204 shl rdx, 4
0x140965c1a 41b804000000 mov r8d, 4
0x140965c20 e8fbdfcaff call 0x140613c20
0x140965c25 90 nop 
0x140965c26 4883c428 add rsp, 0x28
0x140965c2a 5b pop rbx
0x140965c2b 5f pop rdi
0x140965c2c 5e pop rsi
0x140965c2d 415c pop r12
0x140965c2f 415d pop r13
0x140965c31 415e pop r14
0x140965c33 415f pop r15
0x140965c35 5d pop rbp
0x140965c36 c3 ret 
