0x141e4c6c0 push r15
0x141e4c6c2 push r14
0x141e4c6c4 push r13
0x141e4c6c6 push r12
0x141e4c6c8 push rsi
0x141e4c6c9 push rdi
0x141e4c6ca push rbp
0x141e4c6cb push rbx
0x141e4c6cc sub rsp, 0xc8
0x141e4c6d3 movaps xmmword ptr [rsp + 0xb0], xmm9
0x141e4c6dc movaps xmmword ptr [rsp + 0xa0], xmm8
0x141e4c6e5 movaps xmmword ptr [rsp + 0x90], xmm7
0x141e4c6ed movaps xmmword ptr [rsp + 0x80], xmm6
0x141e4c6f5 movaps xmm6, xmm3
0x141e4c6f8 mov rsi, rcx
0x141e4c6fb movss dword ptr [rsp + 0x78], xmm1
0x141e4c701 movss dword ptr [rsp + 0x7c], xmm2
0x141e4c707 mov rdi, qword ptr [rcx + 0xa0]
0x141e4c70e mov rbx, qword ptr [rcx + 0xa8]
0x141e4c715 lea rax, [rsp + 0x58]
0x141e4c71a mov qword ptr [rsp + 0x30], rax
0x141e4c71f cmp rbx, 2
0x141e4c723 jae 0x141e4c95b
0x141e4c729 test rbx, rbx
0x141e4c72c je 0x141e4c925
0x141e4c732 shl rbx, 3
0x141e4c736 lea r13, [rbx + rbx*4]
0x141e4c73a xor r12d, r12d
0x141e4c73d lea r14, [rsp + 0x58]
0x141e4c742 movss xmm7, dword ptr [rip + 0xad9126]
0x141e4c74a movaps xmm8, xmmword ptr [rip + 0xad919e]
0x141e4c752 jmp 0x141e4c7ae
0x141e4c754 mulss xmm9, xmm6
0x141e4c759 addss xmm3, xmm9
0x141e4c75e mulss xmm0, xmm7
0x141e4c762 movss xmm1, dword ptr [rax + 4]
0x141e4c767 mulss xmm1, xmm7
0x141e4c76b subss xmm0, xmm1
0x141e4c76f addss xmm3, dword ptr [r15 + 8]
0x141e4c775 addss xmm0, dword ptr [r15 + 0xc]
0x141e4c77b addss xmm3, dword ptr [rdi + r12 + 8]
0x141e4c782 addss xmm0, dword ptr [rdi + r12 + 0xc]
0x141e4c789 mov edx, dword ptr [rdi + r12 + 0x10]
0x141e4c78e mov r8d, dword ptr [rdi + r12 + 0x14]
0x141e4c793 movss dword ptr [rsp + 0x20], xmm0
0x141e4c799 mov rcx, rsi
0x141e4c79c call 0x142374e10
0x141e4c7a1 add r12, 0x28
0x141e4c7a5 cmp r13, r12
0x141e4c7a8 je 0x141e4c925
0x141e4c7ae movzx ebx, byte ptr [rdi + r12 + 0x24]
0x141e4c7b4 cmp ebx, 4
0x141e4c7b7 jne 0x141e4c830
0x141e4c7b9 mov edx, dword ptr [rdi + r12 + 0x10]
0x141e4c7be mov r8d, dword ptr [rdi + r12 + 0x14]
0x141e4c7c3 lea rax, [rsp + 0x7c]
0x141e4c7c8 mov qword ptr [rsp + 0x20], rax
0x141e4c7cd mov rcx, rsi
0x141e4c7d0 lea r9, [rsp + 0x78]
0x141e4c7d5 call 0x142376120
0x141e4c7da mov r8d, dword ptr [rdi + r12 + 0x10]
0x141e4c7df mov r9d, dword ptr [rdi + r12 + 0x14]
0x141e4c7e4 movups xmm0, xmmword ptr [rip + 0x1142d2d]
0x141e4c7eb movaps xmmword ptr [rsp + 0x30], xmm0
0x141e4c7f0 lea rax, [rsp + 0x30]
0x141e4c7f5 mov qword ptr [rsp + 0x20], rax
0x141e4c7fa mov rcx, r14
0x141e4c7fd mov rdx, rsi
0x141e4c800 call 0x140fe7e40
0x141e4c805 cmp dword ptr [rsp + 0x58], 4
0x141e4c80a jne 0x141e4c980
0x141e4c810 mov edx, dword ptr [rdi + r12 + 0x10]
0x141e4c815 mov r8d, dword ptr [rdi + r12 + 0x14]
0x141e4c81a movss xmm3, dword ptr [rdi + r12 + 8]
0x141e4c821 movss xmm0, dword ptr [rdi + r12 + 0xc]
0x141e4c828 jmp 0x141e4c793
0x141e4c82d nop dword ptr [rax]
0x141e4c830 mov ebp, dword ptr [rdi + r12 + 0x18]
0x141e4c835 mov r15d, dword ptr [rdi + r12 + 0x1c]
0x141e4c83a movss xmm9, dword ptr [rdi + r12 + 0x20]
0x141e4c841 mov r8d, dword ptr [rdi + r12 + 0x10]
0x141e4c846 mov r9d, dword ptr [rdi + r12 + 0x14]
0x141e4c84b movups xmm0, xmmword ptr [rip + 0x1142cc6]
0x141e4c852 movaps xmmword ptr [rsp + 0x30], xmm0
0x141e4c857 lea rax, [rsp + 0x30]
0x141e4c85c mov qword ptr [rsp + 0x20], rax
0x141e4c861 mov rcx, r14
0x141e4c864 mov rdx, rsi
0x141e4c867 call 0x140fe7e40
0x141e4c86c cmp dword ptr [rsp + 0x58], 4
0x141e4c871 jne 0x141e4c9d7
0x141e4c877 mov rcx, r14
0x141e4c87a mov rdx, rsi
0x141e4c87d mov r8d, ebp
0x141e4c880 mov r9d, r15d
0x141e4c883 call 0x140fe7d00
0x141e4c888 cmp dword ptr [rsp + 0x58], 4
0x141e4c88d jne 0x141e4c99d
0x141e4c893 mov r15, qword ptr [rsp + 0x60]
0x141e4c898 mov r8d, dword ptr [rdi + r12 + 0x10]
0x141e4c89d mov r9d, dword ptr [rdi + r12 + 0x14]
0x141e4c8a2 mov rcx, r14
0x141e4c8a5 mov rdx, rsi
0x141e4c8a8 call 0x140fe7d00
0x141e4c8ad cmp dword ptr [rsp + 0x58], 4
0x141e4c8b2 jne 0x141e4c9ba
0x141e4c8b8 mov rax, qword ptr [rsp + 0x60]
0x141e4c8bd movss xmm3, dword ptr [r15]
0x141e4c8c2 movss xmm0, dword ptr [r15 + 4]
0x141e4c8c8 cmp ebx, 2
0x141e4c8cb jae 0x141e4c8f0
0x141e4c8cd mulss xmm3, xmm7
0x141e4c8d1 movss xmm1, dword ptr [rax]
0x141e4c8d5 mulss xmm1, xmm7
0x141e4c8d9 subss xmm3, xmm1
0x141e4c8dd test bl, bl
0x141e4c8df je 0x141e4c90d
0x141e4c8e1 mulss xmm9, xmm6
0x141e4c8e6 addss xmm0, xmm9
0x141e4c8eb jmp 0x141e4c76f
0x141e4c8f0 jne 0x141e4c754
0x141e4c8f6 movss xmm3, dword ptr [rax]
0x141e4c8fa xorps xmm3, xmm8
0x141e4c8fe mulss xmm9, xmm6
0x141e4c903 subss xmm3, xmm9
0x141e4c908 jmp 0x141e4c75e
0x141e4c90d movss xmm0, dword ptr [rax + 4]
0x141e4c912 xorps xmm0, xmm8
0x141e4c916 mulss xmm9, xmm6
0x141e4c91b subss xmm0, xmm9
0x141e4c920 jmp 0x141e4c76f
0x141e4c925 movaps xmm6, xmmword ptr [rsp + 0x80]
0x141e4c92d movaps xmm7, xmmword ptr [rsp + 0x90]
0x141e4c935 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x141e4c93e movaps xmm9, xmmword ptr [rsp + 0xb0]
0x141e4c947 add rsp, 0xc8
0x141e4c94e pop rbx
0x141e4c94f pop rbp
0x141e4c950 pop rdi
0x141e4c951 pop rsi
0x141e4c952 pop r12
0x141e4c954 pop r13
0x141e4c956 pop r14
0x141e4c958 pop r15
0x141e4c95a ret 
0x141e4c95b cmp rbx, 0x15
0x141e4c95f jae 0x141e4ca14
0x141e4c965 lea r9, [rsp + 0x30]
0x141e4c96a mov r8d, 1
0x141e4c970 mov rcx, rdi
0x141e4c973 mov rdx, rbx
0x141e4c976 call 0x140fe4110
0x141e4c97b jmp 0x141e4c732
0x141e4c980 movups xmm0, xmmword ptr [rsp + 0x58]
0x141e4c985 movups xmm1, xmmword ptr [rsp + 0x68]
0x141e4c98a movaps xmmword ptr [rsp + 0x40], xmm1
0x141e4c98f movaps xmmword ptr [rsp + 0x30], xmm0
0x141e4c994 lea rax, [rip + 0x1142b8d]
0x141e4c99b jmp 0x141e4c9f2
0x141e4c99d movups xmm0, xmmword ptr [rsp + 0x58]
0x141e4c9a2 movups xmm1, xmmword ptr [rsp + 0x68]
0x141e4c9a7 movaps xmmword ptr [rsp + 0x40], xmm1
0x141e4c9ac movaps xmmword ptr [rsp + 0x30], xmm0
0x141e4c9b1 lea rax, [rip + 0x1142ba0]
0x141e4c9b8 jmp 0x141e4c9f2
0x141e4c9ba movups xmm0, xmmword ptr [rsp + 0x58]
0x141e4c9bf movups xmm1, xmmword ptr [rsp + 0x68]
0x141e4c9c4 movaps xmmword ptr [rsp + 0x40], xmm1
0x141e4c9c9 movaps xmmword ptr [rsp + 0x30], xmm0
0x141e4c9ce lea rax, [rip + 0x1142b9b]
0x141e4c9d5 jmp 0x141e4c9f2
0x141e4c9d7 movups xmm0, xmmword ptr [rsp + 0x58]
0x141e4c9dc movups xmm1, xmmword ptr [rsp + 0x68]
0x141e4c9e1 movaps xmmword ptr [rsp + 0x40], xmm1
0x141e4c9e6 movaps xmmword ptr [rsp + 0x30], xmm0
0x141e4c9eb lea rax, [rip + 0x1142b4e]
0x141e4c9f2 mov qword ptr [rsp + 0x20], rax
0x141e4c9f7 lea rcx, [rip + 0x114295a]
0x141e4c9fe lea r9, [rip + 0x1142933]
0x141e4ca05 lea r8, [rsp + 0x30]
0x141e4ca0a mov edx, 0x2b
0x141e4ca0f call 0x1428d9760
0x141e4ca14 lea r8, [rsp + 0x30]
0x141e4ca19 mov rcx, rdi
0x141e4ca1c mov rdx, rbx
0x141e4ca1f call 0x140fe4c00
0x141e4ca24 jmp 0x141e4c732
