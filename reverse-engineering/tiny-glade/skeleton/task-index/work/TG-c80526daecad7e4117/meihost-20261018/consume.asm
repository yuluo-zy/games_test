0x140fac810 push rbp
0x140fac811 push r15
0x140fac813 push r14
0x140fac815 push r13
0x140fac817 push r12
0x140fac819 push rsi
0x140fac81a push rdi
0x140fac81b push rbx
0x140fac81c sub rsp, 0x108
0x140fac823 lea rbp, [rsp + 0x80]
0x140fac82b mov qword ptr [rbp + 0x80], 0xfffffffffffffffe
0x140fac836 mov r14, r8
0x140fac839 mov qword ptr [rbp - 0x40], rdx
0x140fac83d mov rsi, rcx
0x140fac840 mov r15, qword ptr [rcx + 0x418]
0x140fac847 mov rdi, qword ptr [rcx + 0x420]
0x140fac84e mov qword ptr [rcx + 0x440], rdi
0x140fac855 mov rax, qword ptr [rcx + 0x430]
0x140fac85c mov qword ptr [rbp - 0x38], rax
0x140fac860 mov r13, qword ptr [rcx + 0x438]
0x140fac867 mov r12, qword ptr [r8]
0x140fac86a mov rbx, qword ptr [r8 + 8]
0x140fac86e mov rcx, qword ptr [rbx]
0x140fac871 call 0x140987ca0
0x140fac876 test rdi, rdi
0x140fac879 je 0x140fac8da
0x140fac87b shl rdi, 4
0x140fac87f lea rcx, [rdi + rdi*2]
0x140fac883 cmp rcx, qword ptr [r12 + 0x18]
0x140fac888 ja 0x140fac982
0x140fac88e mov qword ptr [rbp + 0x20], 0
0x140fac896 mov qword ptr [rbp + 0x28], rcx
0x140fac89a lea rdx, [rbp + 0x20]
0x140fac89e mov qword ptr [rsp + 0x28], rdx
0x140fac8a3 mov qword ptr [rsp + 0x20], rcx
0x140fac8a8 mov qword ptr [rsp + 0x38], 0
0x140fac8b1 mov qword ptr [rsp + 0x30], 1
0x140fac8ba lea rcx, [rbp + 0x30]
0x140fac8be mov rdx, rax
0x140fac8c1 mov r8, r12
0x140fac8c4 mov r9, r15
0x140fac8c7 call 0x140beefa0
0x140fac8cc mov rax, qword ptr [rbp + 0x30]
0x140fac8d0 cmp rax, 0xb
0x140fac8d4 jne 0x140fac989
0x140fac8da mov r14, qword ptr [r14 + 0x10]
0x140fac8de mov rcx, qword ptr [rbx]
0x140fac8e1 call 0x140987ca0
0x140fac8e6 test r13, r13
0x140fac8e9 je 0x140fac946
0x140fac8eb shl r13, 2
0x140fac8ef cmp r13, qword ptr [r14 + 0x18]
0x140fac8f3 ja 0x140faca0a
0x140fac8f9 mov qword ptr [rbp + 0x20], 0
0x140fac901 mov qword ptr [rbp + 0x28], r13
0x140fac905 lea rcx, [rbp + 0x20]
0x140fac909 mov qword ptr [rsp + 0x28], rcx
0x140fac90e mov qword ptr [rsp + 0x20], r13
0x140fac913 mov qword ptr [rsp + 0x38], 0
0x140fac91c mov qword ptr [rsp + 0x30], 1
0x140fac925 lea rcx, [rbp + 0x30]
0x140fac929 mov rdx, rax
0x140fac92c mov r8, r14
0x140fac92f mov r9, qword ptr [rbp - 0x38]
0x140fac933 call 0x140beefa0
0x140fac938 mov rax, qword ptr [rbp + 0x30]
0x140fac93c cmp rax, 0xb
0x140fac940 jne 0x140faca11
0x140fac946 mov qword ptr [rsi + 0x420], 0
0x140fac951 mov qword ptr [rsi + 0x438], 0
0x140fac95c mov rcx, rsi
0x140fac95f mov rdx, qword ptr [rbp - 0x40]
0x140fac963 call 0x140b8f680
0x140fac968 inc dword ptr [rsi + 0x4c8]
0x140fac96e add rsp, 0x108
0x140fac975 pop rbx
0x140fac976 pop rdi
0x140fac977 pop rsi
0x140fac978 pop r12
0x140fac97a pop r13
0x140fac97c pop r14
0x140fac97e pop r15
0x140fac980 pop rbp
0x140fac981 ret 
0x140fac982 mov eax, 0xb
0x140fac987 jmp 0x140fac9b1
0x140fac989 mov rcx, qword ptr [rbp + 0x78]
0x140fac98d mov qword ptr [rbp + 0x10], rcx
0x140fac991 movups xmm0, xmmword ptr [rbp + 0x38]
0x140fac995 movups xmm1, xmmword ptr [rbp + 0x48]
0x140fac999 movups xmm2, xmmword ptr [rbp + 0x58]
0x140fac99d movups xmm3, xmmword ptr [rbp + 0x68]
0x140fac9a1 movaps xmmword ptr [rbp], xmm3
0x140fac9a5 movaps xmmword ptr [rbp - 0x10], xmm2
0x140fac9a9 movaps xmmword ptr [rbp - 0x20], xmm1
0x140fac9ad movaps xmmword ptr [rbp - 0x30], xmm0
0x140fac9b1 mov qword ptr [rbp + 0x30], rax
0x140fac9b5 movaps xmm0, xmmword ptr [rbp - 0x30]
0x140fac9b9 movaps xmm1, xmmword ptr [rbp - 0x20]
0x140fac9bd movaps xmm2, xmmword ptr [rbp - 0x10]
0x140fac9c1 movaps xmm3, xmmword ptr [rbp]
0x140fac9c5 movups xmmword ptr [rbp + 0x38], xmm0
0x140fac9c9 movups xmmword ptr [rbp + 0x48], xmm1
0x140fac9cd movups xmmword ptr [rbp + 0x58], xmm2
0x140fac9d1 movups xmmword ptr [rbp + 0x68], xmm3
0x140fac9d5 mov rax, qword ptr [rbp + 0x10]
0x140fac9d9 mov qword ptr [rbp + 0x78], rax
0x140fac9dd lea rax, [rip + 0x1c047cc]
0x140fac9e4 mov qword ptr [rsp + 0x20], rax
0x140fac9e9 lea rcx, [rip + 0x1c043c8] ; b'called `Result::unwrap()` on an `Err` value/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\alloc\\src\\slice.rs'
0x140fac9f0 lea r9, [rip + 0x1c043a1]
0x140fac9f7 lea r8, [rbp + 0x30]
0x140fac9fb mov edx, 0x2b
0x140faca00 call 0x1428d9760
0x140faca05 jmp 0x140faca8d
0x140faca0a mov eax, 0xb
0x140faca0f jmp 0x140faca39
0x140faca11 mov rcx, qword ptr [rbp + 0x78]
0x140faca15 mov qword ptr [rbp + 0x10], rcx
0x140faca19 movups xmm0, xmmword ptr [rbp + 0x38]
0x140faca1d movups xmm1, xmmword ptr [rbp + 0x48]
0x140faca21 movups xmm2, xmmword ptr [rbp + 0x58]
0x140faca25 movups xmm3, xmmword ptr [rbp + 0x68]
0x140faca29 movaps xmmword ptr [rbp], xmm3
0x140faca2d movaps xmmword ptr [rbp - 0x10], xmm2
0x140faca31 movaps xmmword ptr [rbp - 0x20], xmm1
0x140faca35 movaps xmmword ptr [rbp - 0x30], xmm0
0x140faca39 mov qword ptr [rbp + 0x30], rax
0x140faca3d movaps xmm0, xmmword ptr [rbp - 0x30]
0x140faca41 movaps xmm1, xmmword ptr [rbp - 0x20]
0x140faca45 movaps xmm2, xmmword ptr [rbp - 0x10]
0x140faca49 movaps xmm3, xmmword ptr [rbp]
0x140faca4d movups xmmword ptr [rbp + 0x38], xmm0
0x140faca51 movups xmmword ptr [rbp + 0x48], xmm1
0x140faca55 movups xmmword ptr [rbp + 0x58], xmm2
0x140faca59 movups xmmword ptr [rbp + 0x68], xmm3
0x140faca5d mov rax, qword ptr [rbp + 0x10]
0x140faca61 mov qword ptr [rbp + 0x78], rax
0x140faca65 lea rax, [rip + 0x1c0475c]
0x140faca6c mov qword ptr [rsp + 0x20], rax
0x140faca71 lea rcx, [rip + 0x1c04340] ; b'called `Result::unwrap()` on an `Err` value/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\alloc\\src\\slice.rs'
0x140faca78 lea r9, [rip + 0x1c04319]
0x140faca7f lea r8, [rbp + 0x30]
0x140faca83 mov edx, 0x2b
0x140faca88 call 0x1428d9760
0x140faca8d ud2 
0x140faca8f nop 
0x140faca90 mov qword ptr [rsp + 0x10], rdx
0x140faca95 push rbp
0x140faca96 push r15
0x140faca98 push r14
0x140faca9a push r13
0x140faca9c push r12
0x140faca9e push rsi
0x140faca9f push rdi
0x140facaa0 push rbx
0x140facaa1 sub rsp, 0x48
0x140facaa5 lea rbp, [rdx + 0x80]
0x140facaac lea rcx, [rbp + 0x30]
0x140facab0 call 0x1407b1810
0x140facab5 nop 
0x140facab6 add rsp, 0x48
0x140facaba pop rbx
0x140facabb pop rdi
0x140facabc pop rsi
0x140facabd pop r12
0x140facabf pop r13
0x140facac1 pop r14
0x140facac3 pop r15
0x140facac5 pop rbp
0x140facac6 ret 
0x140facac7 nop word ptr [rax + rax]
0x140facad0 mov qword ptr [rsp + 0x10], rdx
0x140facad5 push rbp
0x140facad6 push r15
0x140facad8 push r14
0x140facada push r13
0x140facadc push r12
0x140facade push rsi
0x140facadf push rdi
0x140facae0 push rbx
0x140facae1 sub rsp, 0x48
0x140facae5 lea rbp, [rdx + 0x80]
0x140facaec lea rcx, [rbp + 0x30]
0x140facaf0 call 0x1407b1810
0x140facaf5 nop 
0x140facaf6 add rsp, 0x48
0x140facafa pop rbx
0x140facafb pop rdi
0x140facafc pop rsi
0x140facafd pop r12
0x140facaff pop r13
0x140facb01 pop r14
0x140facb03 pop r15
0x140facb05 pop rbp
0x140facb06 ret 