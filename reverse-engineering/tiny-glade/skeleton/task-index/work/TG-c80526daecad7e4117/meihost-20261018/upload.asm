0x140b8f680 push rbp
0x140b8f681 push r14
0x140b8f683 push rsi
0x140b8f684 push rdi
0x140b8f685 push rbx
0x140b8f686 sub rsp, 0x360
0x140b8f68d lea rbp, [rsp + 0x80]
0x140b8f695 mov qword ptr [rbp + 0x2d8], 0xfffffffffffffffe
0x140b8f6a0 mov rdi, rdx
0x140b8f6a3 mov rsi, rcx
0x140b8f6a6 cmp byte ptr [rcx + 0x4d4], 0
0x140b8f6ad je 0x140b8f9be
0x140b8f6b3 mov byte ptr [rsi + 0x4d4], 0
0x140b8f6ba mov eax, dword ptr [rsi + 0x484]
0x140b8f6c0 add eax, eax
0x140b8f6c2 mov dword ptr [rbp + 0x2cc], eax
0x140b8f6c8 mov rcx, qword ptr [rip + 0x2a9ccd9]
0x140b8f6cf mov rcx, qword ptr [rcx]
0x140b8f6d2 cmp rcx, 2
0x140b8f6d6 jbe 0x140b8f7a1
0x140b8f6dc lea rax, [rbp + 0x2cc]
0x140b8f6e3 mov qword ptr [rbp + 0x260], rax
0x140b8f6ea lea rcx, [rip + 0x1fde6f]
0x140b8f6f1 mov qword ptr [rbp + 0x268], rcx
0x140b8f6f8 mov qword ptr [rbp + 0x270], rax
0x140b8f6ff mov qword ptr [rbp + 0x278], rcx
0x140b8f706 lea rcx, [rip + 0x1f6cb7b]
0x140b8f70d call 0x14067dd80
0x140b8f712 movups xmm0, xmmword ptr [rax]
0x140b8f715 mov eax, dword ptr [rax + 0x10]
0x140b8f718 mov qword ptr [rbp - 0x20], 3
0x140b8f720 lea rcx, [rip + 0x1f6cb79] ; b'tg_text'
0x140b8f727 mov qword ptr [rbp - 0x18], rcx
0x140b8f72b mov qword ptr [rbp - 0x10], 7
0x140b8f733 lea rdx, [rip + 0x1f6cb2e]
0x140b8f73a mov qword ptr [rbp], rdx
0x140b8f73e mov qword ptr [rbp + 8], 2
0x140b8f746 lea rdx, [rbp + 0x260]
0x140b8f74d mov qword ptr [rbp + 0x10], rdx
0x140b8f751 mov qword ptr [rbp + 0x18], 2
0x140b8f759 mov qword ptr [rbp + 0x20], 0
0x140b8f761 mov qword ptr [rbp - 0x50], 0
0x140b8f769 mov qword ptr [rbp - 0x48], rcx
0x140b8f76d mov qword ptr [rbp - 0x40], 7
0x140b8f775 mov qword ptr [rbp - 0x38], 0
0x140b8f77d movups xmmword ptr [rbp - 0x30], xmm0
0x140b8f781 mov dword ptr [rbp - 8], 1
0x140b8f788 mov dword ptr [rbp - 4], eax
0x140b8f78b lea rcx, [rbp + 0x2b0]
0x140b8f792 lea rdx, [rbp - 0x50]
0x140b8f796 call 0x140d4a330
0x140b8f79b mov eax, dword ptr [rbp + 0x2cc]
0x140b8f7a1 mov eax, eax
0x140b8f7a3 mov rdx, rax
0x140b8f7a6 shl rdx, 0x20
0x140b8f7aa or rdx, rax
0x140b8f7ad lea rbx, [rbp - 0x50]
0x140b8f7b1 mov rcx, rbx
0x140b8f7b4 mov r8d, 0x2b
0x140b8f7ba call 0x140bd52a0
0x140b8f7bf lea r14, [rbp + 0x260]
0x140b8f7c6 mov rcx, r14
0x140b8f7c9 mov rdx, rbx
0x140b8f7cc call 0x140bd5370
0x140b8f7d1 mov rcx, rbx
0x140b8f7d4 mov rdx, r14
0x140b8f7d7 call 0x140bd53b0
0x140b8f7dc mov qword ptr [rbp + 0x2b0], 1
0x140b8f7e7 lea rax, [rbp + 0x2b0]
0x140b8f7ee mov qword ptr [rsp + 0x28], rax
0x140b8f7f3 mov qword ptr [rsp + 0x20], rbx
0x140b8f7f8 lea r8, [rip + 0x1f6c969] ; b'Glyph atlascreate glyph atlas texturecrates/tg-text/src/lib.rs'
0x140b8f7ff lea rcx, [rbp + 0x260]
0x140b8f806 mov r9d, 0xb
0x140b8f80c mov rdx, rdi
0x140b8f80f call 0x1417faa80
0x140b8f814 cmp dword ptr [rbp + 0x260], 0xb
0x140b8f81b jne 0x140b8fc3b
0x140b8f821 lea rcx, [rsi + 0x460]
0x140b8f828 mov rax, qword ptr [rbp + 0x298]
0x140b8f82f mov qword ptr [rbp - 0x20], rax
0x140b8f833 movups xmm0, xmmword ptr [rbp + 0x268]
0x140b8f83a movups xmm1, xmmword ptr [rbp + 0x278]
0x140b8f841 movups xmm2, xmmword ptr [rbp + 0x288]
0x140b8f848 movaps xmmword ptr [rbp - 0x30], xmm2
0x140b8f84c movaps xmmword ptr [rbp - 0x40], xmm1
0x140b8f850 movaps xmmword ptr [rbp - 0x50], xmm0
0x140b8f854 mov qword ptr [rbp + 0x2d0], rcx
0x140b8f85b call 0x14003bb80
0x140b8f860 mov rcx, qword ptr [rbp + 0x2d0]
0x140b8f867 call 0x140be24b0
0x140b8f86c mov rax, qword ptr [rbp - 0x20]
0x140b8f870 mov rcx, qword ptr [rbp + 0x2d0]
0x140b8f877 mov qword ptr [rcx + 0x30], rax
0x140b8f87b movaps xmm0, xmmword ptr [rbp - 0x50]
0x140b8f87f movaps xmm1, xmmword ptr [rbp - 0x40]
0x140b8f883 movaps xmm2, xmmword ptr [rbp - 0x30]
0x140b8f887 movups xmmword ptr [rcx + 0x20], xmm2
0x140b8f88b movups xmmword ptr [rcx + 0x10], xmm1
0x140b8f88f movups xmmword ptr [rcx], xmm0
0x140b8f892 lea rcx, [rbp - 0x50]
0x140b8f896 call 0x140e128c0
0x140b8f89b lea rcx, [rsi + 0x110]
0x140b8f8a2 mov qword ptr [rbp + 0x2d0], rcx
0x140b8f8a9 call 0x1417f8890
0x140b8f8ae lea rdx, [rbp - 0x50]
0x140b8f8b2 mov r8d, 0x298
0x140b8f8b8 mov rcx, qword ptr [rbp + 0x2d0]
0x140b8f8bf call 0x142923c00
0x140b8f8c4 mov edx, dword ptr [rbp + 0x2cc]
0x140b8f8ca lea rcx, [rbp - 0x50]
0x140b8f8ce mov r8d, edx
0x140b8f8d1 call 0x1417f99e0
0x140b8f8d6 mov rdx, qword ptr [rsi + 0x3a8]
0x140b8f8dd test rdx, rdx
0x140b8f8e0 je 0x140b8f8ff
0x140b8f8e2 mov rcx, qword ptr [rsi + 0x3b0]
0x140b8f8e9 lea rax, [rdx + rdx]
0x140b8f8ed shl rdx, 4
0x140b8f8f1 sub rdx, rax
0x140b8f8f4 mov r8d, 2
0x140b8f8fa call 0x140613c20
0x140b8f8ff lea rdi, [rsi + 0x3a8]
0x140b8f906 mov rdx, qword ptr [rsi + 0x3c0]
0x140b8f90d test rdx, rdx
0x140b8f910 je 0x140b8f92f
0x140b8f912 mov rcx, qword ptr [rsi + 0x3c8]
0x140b8f919 lea rax, [rdx + rdx]
0x140b8f91d shl rdx, 4
0x140b8f921 sub rdx, rax
0x140b8f924 mov r8d, 2
0x140b8f92a call 0x140613c20
0x140b8f92f movups xmm0, xmmword ptr [rbp - 0x10]
0x140b8f933 movups xmmword ptr [rdi + 0x40], xmm0
0x140b8f937 movups xmm0, xmmword ptr [rbp - 0x50]
0x140b8f93b movups xmm1, xmmword ptr [rbp - 0x40]
0x140b8f93f movups xmm2, xmmword ptr [rbp - 0x30]
0x140b8f943 movups xmm3, xmmword ptr [rbp - 0x20]
0x140b8f947 movups xmmword ptr [rdi + 0x30], xmm3
0x140b8f94b movups xmmword ptr [rdi + 0x20], xmm2
0x140b8f94f movups xmmword ptr [rdi + 0x10], xmm1
0x140b8f953 movups xmmword ptr [rdi], xmm0
0x140b8f956 mov rdx, qword ptr [rsi + 0x3f8]
0x140b8f95d test rdx, rdx
0x140b8f960 je 0x140b8f978
0x140b8f962 mov rcx, qword ptr [rsi + 0x400]
0x140b8f969 shl rdx, 3
0x140b8f96d mov r8d, 4
0x140b8f973 call 0x140613c20
0x140b8f978 mov qword ptr [rsi + 0x3f8], 0
0x140b8f983 mov qword ptr [rsi + 0x400], 4
0x140b8f98e mov qword ptr [rsi + 0x408], 0
0x140b8f999 call 0x140d4d740
0x140b8f99e mov r14, rax
0x140b8f9a1 test byte ptr [rax], 1
0x140b8f9a4 je 0x140b8fb11
0x140b8f9aa mov rax, r14
0x140b8f9ad add rax, 8
0x140b8f9b1 mov rdi, qword ptr [r14 + 8]
0x140b8f9b5 mov rbx, qword ptr [r14 + 0x10]
0x140b8f9b9 jmp 0x140b8fb32
0x140b8f9be cmp qword ptr [rsi + 0x458], 0
0x140b8f9c6 je 0x140b8fc2d
0x140b8f9cc lea rbx, [rbp + 0x248]
0x140b8f9d3 mov rcx, rbx
0x140b8f9d6 call 0x140d4a3b0
0x140b8f9db mov rax, qword ptr [rsi + 0x450]
0x140b8f9e2 mov rcx, qword ptr [rsi + 0x458]
0x140b8f9e9 shl rcx, 6
0x140b8f9ed add rcx, rax
0x140b8f9f0 mov qword ptr [rbp - 0x50], rax
0x140b8f9f4 mov qword ptr [rbp - 0x48], rcx
0x140b8f9f8 mov qword ptr [rbp - 0x40], rbx
0x140b8f9fc lea r8, [rip + 0x1f6c475]
0x140b8fa03 lea rcx, [rbp + 0x2b0]
0x140b8fa0a lea rdx, [rbp - 0x50]
0x140b8fa0e call 0x141800310
0x140b8fa13 mov r9, qword ptr [rbp + 0x2b8]
0x140b8fa1a mov rax, qword ptr [rbp + 0x2c0]
0x140b8fa21 lea r8, [rsi + 0x460]
0x140b8fa28 mov qword ptr [rsp + 0x20], rax
0x140b8fa2d mov byte ptr [rsp + 0x28], 0x22
0x140b8fa32 lea rcx, [rbp + 0x260]
0x140b8fa39 mov rdx, rdi
0x140b8fa3c mov qword ptr [rbp + 0x2d0], r9
0x140b8fa43 call 0x140bfe9c0
0x140b8fa48 cmp dword ptr [rbp + 0x260], 0xb
0x140b8fa4f jne 0x140b8fc9c
0x140b8fa55 mov rdi, qword ptr [rsi + 0x450]
0x140b8fa5c mov rbx, qword ptr [rsi + 0x458]
0x140b8fa63 mov qword ptr [rsi + 0x458], 0
0x140b8fa6e test rbx, rbx
0x140b8fa71 je 0x140b8faa2
0x140b8fa73 add rdi, 8
0x140b8fa77 jmp 0x140b8fa89
0x140b8fa79 nop dword ptr [rax]
0x140b8fa80 add rdi, 0x40
0x140b8fa84 dec rbx
0x140b8fa87 je 0x140b8faa2
0x140b8fa89 mov rdx, qword ptr [rdi - 8]
0x140b8fa8d test rdx, rdx
0x140b8fa90 je 0x140b8fa80
0x140b8fa92 mov rcx, qword ptr [rdi]
0x140b8fa95 mov r8d, 1
0x140b8fa9b call 0x140613c20
0x140b8faa0 jmp 0x140b8fa80
0x140b8faa2 mov rax, qword ptr [rbp + 0x2b0]
0x140b8faa9 test rax, rax
0x140b8faac je 0x140b8fac8
0x140b8faae shl rax, 4
0x140b8fab2 lea rdx, [rax + rax*2]
0x140b8fab6 mov r8d, 8
0x140b8fabc mov rcx, qword ptr [rbp + 0x2d0]
0x140b8fac3 call 0x140613c20
0x140b8fac8 mov rax, qword ptr [rbp + 0x258]
0x140b8facf cmp rax, qword ptr [rip + 0x2a9c90a]
0x140b8fad6 je 0x140b8fc2d
0x140b8fadc mov rsi, qword ptr [rip + 0x2a9c8fd]
0x140b8fae3 nop word ptr cs:[rax + rax]
0x140b8faf0 mov rdi, qword ptr [rax + 0x18]
0x140b8faf4 mov rcx, qword ptr [rax]
0x140b8faf7 mov r8, qword ptr [rax + 8]
0x140b8fafb mov rdx, qword ptr [rax + 0x10]
0x140b8faff call 0x140613c20
0x140b8fb04 mov rax, rdi
0x140b8fb07 cmp rdi, rsi
0x140b8fb0a jne 0x140b8faf0
0x140b8fb0c jmp 0x140b8fc2d
0x140b8fb11 call 0x140d64e00
0x140b8fb16 mov rdi, rax
0x140b8fb19 mov rbx, rdx
0x140b8fb1c mov rax, r14
0x140b8fb1f add rax, 8
0x140b8fb23 mov qword ptr [r14], 1
0x140b8fb2a mov qword ptr [r14 + 8], rdi
0x140b8fb2e mov qword ptr [r14 + 0x10], rdx
0x140b8fb32 lea rcx, [rdi + 1]
0x140b8fb36 mov qword ptr [rax], rcx
0x140b8fb39 lea r14, [rsi + 0x498]
0x140b8fb40 mov r9, qword ptr [rsi + 0x4a0]
0x140b8fb47 test r9, r9
0x140b8fb4a je 0x140b8fb7d
0x140b8fb4c lea rax, [r9 + 1]
0x140b8fb50 mov ecx, 0x24
0x140b8fb55 mul rcx
0x140b8fb58 add rax, 0xf
0x140b8fb5c and rax, 0xfffffffffffffff0
0x140b8fb60 add r9, rax
0x140b8fb63 add r9, 0x11
0x140b8fb67 je 0x140b8fb7d
0x140b8fb69 mov rcx, qword ptr [r14]
0x140b8fb6c sub rcx, rax
0x140b8fb6f mov r8d, 0x10
0x140b8fb75 mov rdx, r9
0x140b8fb78 call 0x140613c20
0x140b8fb7d movups xmm0, xmmword ptr [rip + 0x1f6c28c]
0x140b8fb84 movups xmmword ptr [r14 + 0x10], xmm0
0x140b8fb89 movups xmm0, xmmword ptr [rip + 0x1f6c270]
0x140b8fb90 movups xmmword ptr [r14], xmm0
0x140b8fb94 mov qword ptr [rsi + 0x4b8], rdi
0x140b8fb9b mov qword ptr [rsi + 0x4c0], rbx
0x140b8fba2 mov rdi, qword ptr [rsi + 0x450]
0x140b8fba9 mov rbx, qword ptr [rsi + 0x458]
0x140b8fbb0 test rbx, rbx
0x140b8fbb3 je 0x140b8fbe2
0x140b8fbb5 lea r14, [rdi + 8]
0x140b8fbb9 jmp 0x140b8fbc9
0x140b8fbbb nop dword ptr [rax + rax]
0x140b8fbc0 add r14, 0x40
0x140b8fbc4 dec rbx
0x140b8fbc7 je 0x140b8fbe2
0x140b8fbc9 mov rdx, qword ptr [r14 - 8]
0x140b8fbcd test rdx, rdx
0x140b8fbd0 je 0x140b8fbc0
0x140b8fbd2 mov rcx, qword ptr [r14]
0x140b8fbd5 mov r8d, 1
0x140b8fbdb call 0x140613c20
0x140b8fbe0 jmp 0x140b8fbc0
0x140b8fbe2 mov rdx, qword ptr [rsi + 0x448]
0x140b8fbe9 test rdx, rdx
0x140b8fbec je 0x140b8fc00
0x140b8fbee shl rdx, 6
0x140b8fbf2 mov r8d, 8
0x140b8fbf8 mov rcx, rdi
0x140b8fbfb call 0x140613c20
0x140b8fc00 mov qword ptr [rsi + 0x448], 0
0x140b8fc0b mov qword ptr [rsi + 0x450], 8
0x140b8fc16 mov qword ptr [rsi + 0x458], 0
0x140b8fc21 mov eax, dword ptr [rsi + 0x4c8]
0x140b8fc27 mov dword ptr [rsi + 0x4cc], eax
0x140b8fc2d add rsp, 0x360
0x140b8fc34 pop rbx
0x140b8fc35 pop rdi
0x140b8fc36 pop rsi
0x140b8fc37 pop r14
0x140b8fc39 pop rbp
0x140b8fc3a ret 
0x140b8fc3b movups xmm0, xmmword ptr [rbp + 0x2a0]
0x140b8fc42 movaps xmmword ptr [rbp - 0x10], xmm0
0x140b8fc46 movups xmm0, xmmword ptr [rbp + 0x260]
0x140b8fc4d movups xmm1, xmmword ptr [rbp + 0x270]
0x140b8fc54 movups xmm2, xmmword ptr [rbp + 0x280]
0x140b8fc5b movups xmm3, xmmword ptr [rbp + 0x290]
0x140b8fc62 movaps xmmword ptr [rbp - 0x20], xmm3
0x140b8fc66 movaps xmmword ptr [rbp - 0x30], xmm2
0x140b8fc6a movaps xmmword ptr [rbp - 0x40], xmm1
0x140b8fc6e movaps xmmword ptr [rbp - 0x50], xmm0
0x140b8fc72 lea rax, [rip + 0x1f6c62f]
0x140b8fc79 mov qword ptr [rsp + 0x20], rax
0x140b8fc7e lea rcx, [rip + 0x1f6c4ee] ; b'create glyph atlas texturecrates/tg-text/src/lib.rs'
0x140b8fc85 lea r9, [rip + 0x1f6c204]
0x140b8fc8c lea r8, [rbp - 0x50]
0x140b8fc90 mov edx, 0x1a
0x140b8fc95 call 0x1428d9760
0x140b8fc9a jmp 0x140b8fcfb
0x140b8fc9c movups xmm0, xmmword ptr [rbp + 0x2a0]
0x140b8fca3 movaps xmmword ptr [rbp - 0x10], xmm0
0x140b8fca7 movups xmm0, xmmword ptr [rbp + 0x260]
0x140b8fcae movups xmm1, xmmword ptr [rbp + 0x270]
0x140b8fcb5 movups xmm2, xmmword ptr [rbp + 0x280]
0x140b8fcbc movups xmm3, xmmword ptr [rbp + 0x290]
0x140b8fcc3 movaps xmmword ptr [rbp - 0x20], xmm3
0x140b8fcc7 movaps xmmword ptr [rbp - 0x30], xmm2
0x140b8fccb movaps xmmword ptr [rbp - 0x40], xmm1
0x140b8fccf movaps xmmword ptr [rbp - 0x50], xmm0
0x140b8fcd3 lea rax, [rip + 0x1f6c556]
0x140b8fcda mov qword ptr [rsp + 0x20], rax
0x140b8fcdf lea rcx, [rip + 0x1f6c532] ; b'upload new glyphs'
0x140b8fce6 lea r9, [rip + 0x1f6c1a3]
0x140b8fced lea r8, [rbp - 0x50]
0x140b8fcf1 mov edx, 0x11
0x140b8fcf6 call 0x1428d9760
0x140b8fcfb ud2 
0x140b8fcfd nop dword ptr [rax]
0x140b8fd00 mov qword ptr [rsp + 0x10], rdx
0x140b8fd05 push rbp
0x140b8fd06 push r14
0x140b8fd08 push rsi
0x140b8fd09 push rdi
0x140b8fd0a push rbx
0x140b8fd0b sub rsp, 0x30
0x140b8fd0f lea rbp, [rdx + 0x80]
0x140b8fd16 mov rax, qword ptr [rbp + 0x2b0]
0x140b8fd1d test rax, rax
0x140b8fd20 je 0x140b8fd3c
0x140b8fd22 shl rax, 4
0x140b8fd26 lea rdx, [rax + rax*2]
0x140b8fd2a mov r8d, 8
0x140b8fd30 mov rcx, qword ptr [rbp + 0x2d0]
0x140b8fd37 call 0x140613c20
0x140b8fd3c nop 
0x140b8fd3d add rsp, 0x30
0x140b8fd41 pop rbx
0x140b8fd42 pop rdi
0x140b8fd43 pop rsi
0x140b8fd44 pop r14
0x140b8fd46 pop rbp
0x140b8fd47 ret 
0x140b8fd48 nop dword ptr [rax + rax]
0x140b8fd50 mov qword ptr [rsp + 0x10], rdx
0x140b8fd55 push rbp
0x140b8fd56 push r14
0x140b8fd58 push rsi
0x140b8fd59 push rdi
0x140b8fd5a push rbx
0x140b8fd5b sub rsp, 0x30
0x140b8fd5f lea rbp, [rdx + 0x80]
0x140b8fd66 lea rcx, [rbp - 0x50]
0x140b8fd6a call 0x1407cea70
0x140b8fd6f nop 
0x140b8fd70 add rsp, 0x30
0x140b8fd74 pop rbx
0x140b8fd75 pop rdi
0x140b8fd76 pop rsi
0x140b8fd77 pop r14
0x140b8fd79 pop rbp
0x140b8fd7a ret 
0x140b8fd7b nop dword ptr [rax + rax]
0x140b8fd80 mov qword ptr [rsp + 0x10], rdx
0x140b8fd85 push rbp
0x140b8fd86 push r14
0x140b8fd88 push rsi
0x140b8fd89 push rdi
0x140b8fd8a push rbx
0x140b8fd8b sub rsp, 0x30
0x140b8fd8f lea rbp, [rdx + 0x80]
0x140b8fd96 mov rcx, qword ptr [rbp + 0x2d0]
0x140b8fd9d call 0x140be24b0
0x140b8fda2 nop 
0x140b8fda3 add rsp, 0x30
0x140b8fda7 pop rbx
0x140b8fda8 pop rdi
0x140b8fda9 pop rsi
0x140b8fdaa pop r14
0x140b8fdac pop rbp
0x140b8fdad ret 
0x140b8fdae nop 
0x140b8fdb0 mov qword ptr [rsp + 0x10], rdx
0x140b8fdb5 push rbp
0x140b8fdb6 push r14
0x140b8fdb8 push rsi
0x140b8fdb9 push rdi
0x140b8fdba push rbx
0x140b8fdbb sub rsp, 0x30
0x140b8fdbf lea rbp, [rdx + 0x80]
0x140b8fdc6 lea rcx, [rbp + 0x248]
0x140b8fdcd call 0x1405e6620
0x140b8fdd2 nop 
0x140b8fdd3 add rsp, 0x30
0x140b8fdd7 pop rbx
0x140b8fdd8 pop rdi
0x140b8fdd9 pop rsi
0x140b8fdda pop r14
0x140b8fddc pop rbp
0x140b8fddd ret 
0x140b8fdde nop 
0x140b8fde0 mov qword ptr [rsp + 0x10], rdx
0x140b8fde5 push rbp
0x140b8fde6 push r14
0x140b8fde8 push rsi
0x140b8fde9 push rdi
0x140b8fdea push rbx
0x140b8fdeb sub rsp, 0x30
0x140b8fdef lea rbp, [rdx + 0x80]
0x140b8fdf6 lea rcx, [rbp - 0x50]
0x140b8fdfa call 0x1407cea70
0x140b8fdff nop 
0x140b8fe00 add rsp, 0x30
0x140b8fe04 pop rbx
0x140b8fe05 pop rdi
0x140b8fe06 pop rsi
0x140b8fe07 pop r14
0x140b8fe09 pop rbp
0x140b8fe0a ret 
0x140b8fe0b nop dword ptr [rax + rax]
0x140b8fe10 mov qword ptr [rsp + 0x10], rdx
0x140b8fe15 push rbp
0x140b8fe16 push r14
0x140b8fe18 push rsi
0x140b8fe19 push rdi
0x140b8fe1a push rbx
0x140b8fe1b sub rsp, 0x30
0x140b8fe1f lea rbp, [rdx + 0x80]
0x140b8fe26 mov rax, qword ptr [rbp - 0x20]
0x140b8fe2a mov rcx, qword ptr [rbp + 0x2d0]
0x140b8fe31 mov qword ptr [rcx + 0x30], rax
0x140b8fe35 movaps xmm0, xmmword ptr [rbp - 0x50]
0x140b8fe39 movaps xmm1, xmmword ptr [rbp - 0x40]
0x140b8fe3d movaps xmm2, xmmword ptr [rbp - 0x30]
0x140b8fe41 movups xmmword ptr [rcx + 0x20], xmm2
0x140b8fe45 movups xmmword ptr [rcx + 0x10], xmm1
0x140b8fe49 movups xmmword ptr [rcx], xmm0
0x140b8fe4c add rsp, 0x30
0x140b8fe50 pop rbx
0x140b8fe51 pop rdi
0x140b8fe52 pop rsi
0x140b8fe53 pop r14
0x140b8fe55 pop rbp
0x140b8fe56 ret 
0x140b8fe57 nop word ptr [rax + rax]
0x140b8fe60 mov qword ptr [rsp + 0x10], rdx
0x140b8fe65 push rbp
0x140b8fe66 push r14
0x140b8fe68 push rsi
0x140b8fe69 push rdi
0x140b8fe6a push rbx
0x140b8fe6b sub rsp, 0x30
0x140b8fe6f lea rbp, [rdx + 0x80]
0x140b8fe76 lea rdx, [rbp - 0x50]
0x140b8fe7a mov r8d, 0x298
0x140b8fe80 mov rcx, qword ptr [rbp + 0x2d0]
0x140b8fe87 call 0x142923c00
0x140b8fe8c nop 
0x140b8fe8d add rsp, 0x30
0x140b8fe91 pop rbx
0x140b8fe92 pop rdi
0x140b8fe93 pop rsi
0x140b8fe94 pop r14
0x140b8fe96 pop rbp
0x140b8fe97 ret 