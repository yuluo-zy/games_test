0x1416ec620 push rbp
0x1416ec621 push r15
0x1416ec623 push r14
0x1416ec625 push rsi
0x1416ec626 push rdi
0x1416ec627 push rbx
0x1416ec628 sub rsp, 0x118
0x1416ec62f lea rbp, [rsp + 0x80]
0x1416ec637 movaps xmmword ptr [rbp + 0x80], xmm7
0x1416ec63e movaps xmmword ptr [rbp + 0x70], xmm6
0x1416ec642 mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
0x1416ec64a mov rsi, rcx
0x1416ec64d movzx eax, byte ptr [rdx]
0x1416ec650 lea rcx, [rip + 0x15a3839]
0x1416ec657 movsxd rax, dword ptr [rcx + rax*4]
0x1416ec65b add rax, rcx
0x1416ec65e jmp rax
0x1416ec660 mov al, 0x12
0x1416ec662 jmp 0x1416ec70a
0x1416ec667 mov rax, qword ptr [rdx + 0x18]
0x1416ec66b mov qword ptr [rbp - 0x20], rax
0x1416ec66f movups xmm0, xmmword ptr [rdx + 8]
0x1416ec673 movaps xmmword ptr [rbp - 0x30], xmm0
0x1416ec677 lea rcx, [rbp + 0x30]
0x1416ec67b lea rdx, [rbp - 0x30]
0x1416ec67f call 0x1415b5140
0x1416ec684 movzx eax, byte ptr [rbp + 0x30]
0x1416ec688 cmp al, 0x16
0x1416ec68a je 0x1416ec701
0x1416ec68c jmp 0x1416ec70a
0x1416ec68e mov ecx, dword ptr [rdx + 8]
0x1416ec691 mov rax, qword ptr [rdx + 0x10]
0x1416ec695 mov qword ptr [rbp + 0x38], rax
0x1416ec699 shl ecx, 3
0x1416ec69c mov eax, 0xa0804
0x1416ec6a1 shr eax, cl
0x1416ec6a3 jmp 0x1416ec70a
0x1416ec6a5 movups xmm0, xmmword ptr [rdx + 8]
0x1416ec6a9 movups xmmword ptr [rbp - 8], xmm0
0x1416ec6ad mov rax, qword ptr [rdx + 0x18]
0x1416ec6b1 mov qword ptr [rbp + 8], rax
0x1416ec6b5 movups xmm0, xmmword ptr [rbp - 0xf]
0x1416ec6b9 movups xmmword ptr [rbp + 0x31], xmm0
0x1416ec6bd mov rax, qword ptr [rbp]
0x1416ec6c1 mov qword ptr [rbp + 0x40], rax
0x1416ec6c5 mov rax, qword ptr [rbp + 8]
0x1416ec6c9 mov qword ptr [rbp + 0x48], rax
0x1416ec6cd mov al, 0xc
0x1416ec6cf jmp 0x1416ec70a
0x1416ec6d1 movzx eax, byte ptr [rdx + 1]
0x1416ec6d5 mov byte ptr [rbp + 0x31], al
0x1416ec6d8 xor eax, eax
0x1416ec6da jmp 0x1416ec70a
0x1416ec6dc mov rax, qword ptr [rdx + 0x18]
0x1416ec6e0 mov qword ptr [rbp - 0x40], rax
0x1416ec6e4 movups xmm0, xmmword ptr [rdx + 8]
0x1416ec6e8 movaps xmmword ptr [rbp - 0x50], xmm0
0x1416ec6ec lea rcx, [rbp + 0x30]
0x1416ec6f0 lea rdx, [rbp - 0x50]
0x1416ec6f4 call 0x1415ba5b0
0x1416ec6f9 movzx eax, byte ptr [rbp + 0x30]
0x1416ec6fd cmp al, 0x16
0x1416ec6ff jne 0x1416ec70a
0x1416ec701 mov rdi, qword ptr [rbp + 0x38]
0x1416ec705 jmp 0x1416ec8e1
0x1416ec70a mov ecx, dword ptr [rbp + 0x31]
0x1416ec70d mov edx, dword ptr [rbp + 0x34]
0x1416ec710 mov dword ptr [rbp + 0x14], edx
0x1416ec713 mov dword ptr [rbp + 0x11], ecx
0x1416ec716 mov rcx, qword ptr [rbp + 0x38]
0x1416ec71a movups xmm0, xmmword ptr [rbp + 0x40]
0x1416ec71e movups xmmword ptr [rbp + 0x20], xmm0
0x1416ec722 mov byte ptr [rbp + 0x10], al
0x1416ec725 mov qword ptr [rbp + 0x18], rcx
0x1416ec729 lea rdi, [rip + 0x15a3568]
0x1416ec730 mov qword ptr [rsp + 0x20], rdi
0x1416ec735 mov qword ptr [rsp + 0x28], 2
0x1416ec73e lea r8, [rip + 0x15a353b]
0x1416ec745 lea rcx, [rbp + 0x30]
0x1416ec749 lea rdx, [rbp + 0x10]
0x1416ec74d mov r9d, 0x11
0x1416ec753 call 0x141282e60
0x1416ec758 cmp dword ptr [rbp + 0x30], 1
0x1416ec75c jne 0x1416ec796
0x1416ec75e mov rcx, qword ptr [rbp + 0x38]
0x1416ec762 mov rax, qword ptr [rcx]
0x1416ec765 cmp rax, 1
0x1416ec769 mov qword ptr [rbp + 0x60], rcx
0x1416ec76d je 0x1416ec7e2
0x1416ec76f test rax, rax
0x1416ec772 jne 0x1416ec7eb
0x1416ec774 mov rax, qword ptr [rbp + 0x60]
0x1416ec778 mov rdx, qword ptr [rax + 0x10]
0x1416ec77c test rdx, rdx
0x1416ec77f je 0x1416ec7eb
0x1416ec781 mov rax, qword ptr [rbp + 0x60]
0x1416ec785 mov rcx, qword ptr [rax + 8]
0x1416ec789 mov r8d, 1
0x1416ec78f call 0x140613c20
0x1416ec794 jmp 0x1416ec7eb
0x1416ec796 mov r14d, dword ptr [rbp + 0x34]
0x1416ec79a mov rbx, qword ptr [rbp + 0x38]
0x1416ec79e movsd xmm6, qword ptr [rbp + 0x40]
0x1416ec7a3 mov r15, qword ptr [rbp + 0x48]
0x1416ec7a7 lea rcx, [rbp + 0x10]
0x1416ec7ab call 0x1416e72c0
0x1416ec7b0 lea rdi, [rsi + 4]
0x1416ec7b4 cmp r14d, 2
0x1416ec7b8 jne 0x1416ec7c3
0x1416ec7ba movshdup xmm7, xmm6
0x1416ec7be jmp 0x1416ec880
0x1416ec7c3 mov qword ptr [rsi + 8], rbx
0x1416ec7c7 movlps qword ptr [rsi + 0x10], xmm6
0x1416ec7cb test r14d, r14d
0x1416ec7ce je 0x1416ec89f
0x1416ec7d4 mov qword ptr [rsi + 0x18], r15
0x1416ec7d8 mov eax, 1
0x1416ec7dd jmp 0x1416ec8a1
0x1416ec7e2 add rcx, 8
0x1416ec7e6 call 0x1405e7a00
0x1416ec7eb mov edx, 0x28
0x1416ec7f0 mov r8d, 8
0x1416ec7f6 mov rcx, qword ptr [rbp + 0x60]
0x1416ec7fa call 0x140613c20
0x1416ec7ff mov qword ptr [rsp + 0x20], rdi
0x1416ec804 mov qword ptr [rsp + 0x28], 2
0x1416ec80d lea r8, [rip + 0x15a34a4]
0x1416ec814 lea rcx, [rbp + 0x30]
0x1416ec818 lea rdx, [rbp + 0x10]
0x1416ec81c mov r9d, 0x11
0x1416ec822 call 0x141283000
0x1416ec827 cmp dword ptr [rbp + 0x30], 2
0x1416ec82b jne 0x1416ec865
0x1416ec82d mov rcx, qword ptr [rbp + 0x38]
0x1416ec831 mov rax, qword ptr [rcx]
0x1416ec834 cmp rax, 1
0x1416ec838 mov qword ptr [rbp + 0x60], rcx
0x1416ec83c je 0x1416ec8a7
0x1416ec83e test rax, rax
0x1416ec841 jne 0x1416ec8b0
0x1416ec843 mov rax, qword ptr [rbp + 0x60]
0x1416ec847 mov rdx, qword ptr [rax + 0x10]
0x1416ec84b test rdx, rdx
0x1416ec84e je 0x1416ec8b0
0x1416ec850 mov rax, qword ptr [rbp + 0x60]
0x1416ec854 mov rcx, qword ptr [rax + 8]
0x1416ec858 mov r8d, 1
0x1416ec85e call 0x140613c20
0x1416ec863 jmp 0x1416ec8b0
0x1416ec865 mov rbx, qword ptr [rbp + 0x30]
0x1416ec869 movss xmm6, dword ptr [rbp + 0x38]
0x1416ec86e movss xmm7, dword ptr [rbp + 0x3c]
0x1416ec873 lea rcx, [rbp + 0x10]
0x1416ec877 call 0x1416e72c0
0x1416ec87c lea rdi, [rsi + 4]
0x1416ec880 test ebx, ebx
0x1416ec882 jne 0x1416ec90a
0x1416ec888 shr rbx, 0x20
0x1416ec88c movd xmm1, ebx
0x1416ec890 lea rcx, [rsi + 8]
0x1416ec894 movaps xmm2, xmm6
0x1416ec897 movaps xmm3, xmm7
0x1416ec89a call 0x140c97540
0x1416ec89f xor eax, eax
0x1416ec8a1 mov dword ptr [rdi], eax
0x1416ec8a3 xor eax, eax
0x1416ec8a5 jmp 0x1416ec8ea
0x1416ec8a7 add rcx, 8
0x1416ec8ab call 0x1405e7a00
0x1416ec8b0 mov edx, 0x28
0x1416ec8b5 mov r8d, 8
0x1416ec8bb mov rcx, qword ptr [rbp + 0x60]
0x1416ec8bf call 0x140613c20
0x1416ec8c4 lea rcx, [rip + 0x15a3435]
0x1416ec8cb mov edx, 0x49
0x1416ec8d0 call 0x1428fa000
0x1416ec8d5 mov rdi, rax
0x1416ec8d8 lea rcx, [rbp + 0x10]
0x1416ec8dc call 0x1416e72c0
0x1416ec8e1 mov qword ptr [rsi + 8], rdi
0x1416ec8e5 mov eax, 1
0x1416ec8ea mov dword ptr [rsi], eax
0x1416ec8ec mov rax, rsi
0x1416ec8ef movaps xmm6, xmmword ptr [rbp + 0x70]
0x1416ec8f3 movaps xmm7, xmmword ptr [rbp + 0x80]
0x1416ec8fa add rsp, 0x118
0x1416ec901 pop rbx
0x1416ec902 pop rdi
0x1416ec903 pop rsi
0x1416ec904 pop r14
0x1416ec906 pop r15
0x1416ec908 pop rbp
0x1416ec909 ret 
0x1416ec90a lea rax, [rip + 0x152d09f]
0x1416ec911 mov qword ptr [rbp + 0x30], rax
0x1416ec915 mov qword ptr [rbp + 0x38], 1
0x1416ec91d mov qword ptr [rbp + 0x40], 8
0x1416ec925 xorps xmm0, xmm0
0x1416ec928 movups xmmword ptr [rbp + 0x48], xmm0
0x1416ec92c lea rdx, [rip + 0x152d08d]
0x1416ec933 lea rcx, [rbp + 0x30]
0x1416ec937 call 0x1428d9390
0x1416ec93c int3 
0x1416ec93d nop dword ptr [rax]
0x1416ec940 mov qword ptr [rsp + 0x10], rdx
0x1416ec945 push rbp
0x1416ec946 push r15
0x1416ec948 push r14
0x1416ec94a push rsi
0x1416ec94b push rdi
0x1416ec94c push rbx
0x1416ec94d sub rsp, 0x58
0x1416ec951 lea rbp, [rdx + 0x80]
0x1416ec958 movaps xmmword ptr [rsp + 0x30], xmm7
0x1416ec95d movaps xmmword ptr [rsp + 0x40], xmm6
0x1416ec962 mov edx, 0x28
0x1416ec967 mov r8d, 8
0x1416ec96d mov rcx, qword ptr [rbp + 0x60]
0x1416ec971 call 0x140613c20
0x1416ec976 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416ec97b movaps xmm7, xmmword ptr [rsp + 0x30]
0x1416ec980 add rsp, 0x58
0x1416ec984 pop rbx
0x1416ec985 pop rdi
0x1416ec986 pop rsi
0x1416ec987 pop r14
0x1416ec989 pop r15
0x1416ec98b pop rbp
0x1416ec98c ret 
0x1416ec98d nop dword ptr [rax]
0x1416ec990 mov qword ptr [rsp + 0x10], rdx
0x1416ec995 push rbp
0x1416ec996 push r15
0x1416ec998 push r14
0x1416ec99a push rsi
0x1416ec99b push rdi
0x1416ec99c push rbx
0x1416ec99d sub rsp, 0x58
0x1416ec9a1 lea rbp, [rdx + 0x80]
0x1416ec9a8 movaps xmmword ptr [rsp + 0x30], xmm7
0x1416ec9ad movaps xmmword ptr [rsp + 0x40], xmm6
0x1416ec9b2 mov edx, 0x28
0x1416ec9b7 mov r8d, 8
0x1416ec9bd mov rcx, qword ptr [rbp + 0x60]
0x1416ec9c1 call 0x140613c20
0x1416ec9c6 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416ec9cb movaps xmm7, xmmword ptr [rsp + 0x30]
0x1416ec9d0 add rsp, 0x58
0x1416ec9d4 pop rbx
0x1416ec9d5 pop rdi
0x1416ec9d6 pop rsi
0x1416ec9d7 pop r14
0x1416ec9d9 pop r15
0x1416ec9db pop rbp
0x1416ec9dc ret 
0x1416ec9dd nop dword ptr [rax]
0x1416ec9e0 mov qword ptr [rsp + 0x10], rdx
0x1416ec9e5 push rbp
0x1416ec9e6 push r15
0x1416ec9e8 push r14
0x1416ec9ea push rsi
0x1416ec9eb push rdi
0x1416ec9ec push rbx
0x1416ec9ed sub rsp, 0x58
0x1416ec9f1 lea rbp, [rdx + 0x80]
0x1416ec9f8 movaps xmmword ptr [rsp + 0x30], xmm7
0x1416ec9fd movaps xmmword ptr [rsp + 0x40], xmm6
0x1416eca02 lea rcx, [rbp + 0x10]
0x1416eca06 call 0x1416e72c0
0x1416eca0b movaps xmm6, xmmword ptr [rsp + 0x40]
0x1416eca10 movaps xmm7, xmmword ptr [rsp + 0x30]
0x1416eca15 add rsp, 0x58
0x1416eca19 pop rbx
0x1416eca1a pop rdi
0x1416eca1b pop rsi
0x1416eca1c pop r14
0x1416eca1e pop r15
0x1416eca20 pop rbp
0x1416eca21 ret 
