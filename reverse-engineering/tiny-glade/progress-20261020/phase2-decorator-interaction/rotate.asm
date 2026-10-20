1407a8580 push rbp
1407a8581 push r15
1407a8583 push r14
1407a8585 push r13
1407a8587 push r12
1407a8589 push rsi
1407a858a push rdi
1407a858b push rbx
1407a858c sub rsp, 0x2b8
1407a8593 lea rbp, [rsp + 0x80]
1407a859b movaps xmmword ptr [rbp + 0x220], xmm14
1407a85a3 movaps xmmword ptr [rbp + 0x210], xmm13
1407a85ab movaps xmmword ptr [rbp + 0x200], xmm12
1407a85b3 movaps xmmword ptr [rbp + 0x1f0], xmm11
1407a85bb movaps xmmword ptr [rbp + 0x1e0], xmm10
1407a85c3 movaps xmmword ptr [rbp + 0x1d0], xmm9
1407a85cb movaps xmmword ptr [rbp + 0x1c0], xmm8
1407a85d3 movaps xmmword ptr [rbp + 0x1b0], xmm7
1407a85da movaps xmmword ptr [rbp + 0x1a0], xmm6
1407a85e1 mov qword ptr [rbp + 0x198], 0xfffffffffffffffe
1407a85ec mov r12, rdx
1407a85ef mov rax, qword ptr [rbp + 0x2b8]
1407a85f6 mov r15, qword ptr [rax]
1407a85f9 mov rax, qword ptr [r15]
1407a85fc mov rdx, qword ptr [r15 + 8]
1407a8600 add rax, -2
1407a8604 adc rdx, -1
1407a8608 cmp rax, 5
1407a860c sbb rdx, 0
1407a8610 setb dl
1407a8613 cmp rax, 1
1407a8617 sete al
1407a861a and al, dl
1407a861c cmp al, 1
1407a861e jne 0x1407a9135
1407a8624 mov r13, r9
1407a8627 mov rbx, r8
1407a862a mov rax, qword ptr [rbp + 0x2d0]
1407a8631 mov rdi, qword ptr [rax]
1407a8634 mov rdx, qword ptr [rax + 0x10]
1407a8638 mov qword ptr [rbp + 0x158], rdx
1407a863f mov edx, dword ptr [rax + 0x1c]
1407a8642 mov dword ptr [rbp + 0x168], edx
1407a8648 mov rsi, qword ptr [rax + 0x20]
1407a864c mov rdx, qword ptr [rax + 0x30]
1407a8650 mov qword ptr [rbp + 0x160], rdx
1407a8657 mov edx, dword ptr [rax + 0x3c]
1407a865a mov dword ptr [rbp + 0x190], edx
1407a8660 mov r14, qword ptr [rax + 0x40]
1407a8664 mov eax, dword ptr [r15 + 0x58]
1407a8668 mov qword ptr [rbp + 0x170], rax
1407a866f mov rcx, qword ptr [rcx]
1407a8672 mov rdx, qword ptr [r15 + 0x10]
1407a8676 call 0x14067dd80
1407a867b mov qword ptr [rbp + 0x60], rax
1407a867f mov qword ptr [rbp + 0x68], rdx
1407a8683 movzx eax, byte ptr [r15 + 0x60]
1407a8688 mov qword ptr [rbp + 0x188], rax
1407a868f lea rcx, [rbp + 0x60]
1407a8693 call 0x14092e680
1407a8698 test al, al
1407a869a je 0x1407a9135
1407a86a0 lea rcx, [rbp + 0x60]
1407a86a4 call 0x14092df10
1407a86a9 mov byte ptr [rbp + 0x197], al
1407a86af cmp al, 1
1407a86b1 ja 0x1407a86ca
1407a86b3 mov rdx, qword ptr [rbp + 0x2e0]
1407a86ba mov eax, dword ptr [rdx + 0x1c]
1407a86bd mov rcx, qword ptr [rdx + 0x10]
1407a86c1 mov dword ptr [rcx], eax
1407a86c3 mov rax, qword ptr [rdx]
1407a86c6 mov byte ptr [rax + 0x48], 1
1407a86ca mov qword ptr [rbp + 0x118], r14
1407a86d1 mov qword ptr [rbp + 0x110], rsi
1407a86d8 mov rcx, rdi
1407a86db mov rsi, qword ptr [rbp + 0x170]
1407a86e2 mov edx, esi
1407a86e4 call 0x140a1c3e0
1407a86e9 mov r8, rdx
1407a86ec mov qword ptr [rbp + 0x108], rdi
1407a86f3 mov rcx, rdi
1407a86f6 mov qword ptr [rbp + 0x48], rax
1407a86fa mov rdx, rax
1407a86fd mov qword ptr [rbp + 0x40], r8
1407a8701 mov r9d, esi
1407a8704 call 0x140a1c5f0
1407a8709 test rax, rax
1407a870c je 0x1407a91a0
1407a8712 mov rdi, qword ptr [rbp + 0x2b0]
1407a8719 mov r8d, dword ptr [rax]
1407a871c mov r9d, dword ptr [rax + 4]
1407a8720 movss xmm11, dword ptr [rax + 8]
1407a8726 movss xmm10, dword ptr [rax + 0xc]
1407a872c movzx ecx, byte ptr [rax + 0x22]
1407a8730 add cl, 0xfe
1407a8733 movzx ecx, cl
1407a8736 lea rdx, [rcx + 1]
1407a873a xor r14d, r14d
1407a873d cmp cl, 3
1407a8740 cmovb r14, rdx
1407a8744 movzx ecx, byte ptr [rax + 0x44]
1407a8748 mov byte ptr [rbp + 0x196], cl
1407a874e lea rcx, [rip + 0x22da493]
1407a8755 movsxd rdx, dword ptr [rcx + r14*4]
1407a8759 add rdx, rcx
1407a875c mov dword ptr [rbp + 0x74], r8d
1407a8760 mov dword ptr [rbp + 0x78], r9d
1407a8764 jmp rdx
1407a8766 mov rdx, qword ptr [rdi]
1407a8769 lea r9, [rip + 0x22da388]
1407a8770 lea rcx, [rbp + 0x80]
1407a8777 mov r8, qword ptr [rbp + 0x170]
1407a877e call 0x140a1aa20
1407a8783 mov eax, dword ptr [rbp + 0x98]
1407a8789 mov dword ptr [rbp + 0x128], eax
1407a878f mov rax, qword ptr [rbp + 0x90]
1407a8796 mov qword ptr [rbp + 0x120], rax
1407a879d movaps xmm9, xmm11
1407a87a1 jmp 0x1407a8851
1407a87a6 mov rax, qword ptr [rbp + 0x2a0]
1407a87ad mov dword ptr [rbp], r8d
1407a87b1 mov dword ptr [rbp + 4], r9d
1407a87b5 movss dword ptr [rbp + 8], xmm11
1407a87bb mov rax, qword ptr [rax]
1407a87be movd xmm1, r8d
1407a87c3 movd xmm2, r9d
1407a87c8 mov rcx, qword ptr [rax + 8]
1407a87cc mov rdx, qword ptr [rax + 0x10]
1407a87d0 add rdx, rdx
1407a87d3 mov qword ptr [rbp + 0x80], rcx
1407a87da mov qword ptr [rbp + 0x88], rdx
1407a87e1 movsd xmm0, qword ptr [rax + 0x50]
1407a87e6 movsd qword ptr [rbp + 0x90], xmm0
1407a87ee lea rcx, [rbp + 0x80]
1407a87f5 call 0x140257ba0
1407a87fa lea rcx, [rbp + 0x120]
1407a8801 mov rdx, rbp
1407a8804 movaps xmm2, xmm0
1407a8807 call 0x140c9cde0
1407a880c movss xmm9, dword ptr [rbp + 8]
1407a8812 jmp 0x1407a8851
1407a8814 movss xmm9, dword ptr [rax + 0x10]
1407a881a mov rdx, qword ptr [rdi]
1407a881d lea r9, [rip + 0x22da2ec]
1407a8824 lea rcx, [rbp + 0x80]
1407a882b mov r8, qword ptr [rbp + 0x170]
1407a8832 call 0x140a1aa20
1407a8837 mov eax, dword ptr [rbp + 0x98]
1407a883d mov dword ptr [rbp + 0x128], eax
1407a8843 mov rax, qword ptr [rbp + 0x90]
1407a884a mov qword ptr [rbp + 0x120], rax
1407a8851 mov eax, dword ptr [rbp + 0x128]
1407a8857 mov dword ptr [rbp + 0x180], eax
1407a885d mov rax, qword ptr [rbp + 0x120]
1407a8864 mov qword ptr [rbp + 0x178], rax
1407a886b movss xmm13, dword ptr [rbp + 0x178]
1407a8874 movss xmm12, dword ptr [rbp + 0x180]
1407a887d movss dword ptr [rbp + 0x38], xmm13
1407a8883 mov rcx, qword ptr [rbp + 0x188]
1407a888a lea eax, [rcx - 0x38]
1407a888d cmp al, 8
1407a888f movzx eax, al
1407a8892 mov edx, 6
1407a8897 cmovb edx, eax
1407a889a mov dword ptr [rbp + 0x7c], edx
1407a889d movss dword ptr [rbp + 0x3c], xmm12
1407a88a3 mov eax, ecx
1407a88a5 and al, 0x3e
1407a88a7 cmp al, 0x3c
1407a88a9 jne 0x1407a8905
1407a88ab mov eax, dword ptr [rbp + 0x180]
1407a88b1 mov dword ptr [rbp + 0x128], eax
1407a88b7 mov rax, qword ptr [rbp + 0x178]
1407a88be mov qword ptr [rbp + 0x120], rax
1407a88c5 mov rsi, r12
1407a88c8 lea r12, [rbp + 0x80]
1407a88cf lea rdx, [rbp + 0x120]
1407a88d6 mov rcx, r12
1407a88d9 call 0x140b02490
1407a88de lea rdi, [rbp + 0x120]
1407a88e5 mov rcx, rdi
1407a88e8 mov rdx, r12
1407a88eb mov r12, rsi
1407a88ee call 0x140b02500
1407a88f3 mov rdx, qword ptr [rsi]
1407a88f6 mov rcx, rdi
1407a88f9 mov rdi, qword ptr [rbp + 0x2b0]
1407a8900 call 0x140b02520
1407a8905 movss xmm0, dword ptr [r15 + 0x5c]
1407a890b addss xmm0, dword ptr [rbp + 0x17c]
1407a8913 movss dword ptr [rbp + 0x17c], xmm0
1407a891b lea rcx, [rbp + 0x60]
1407a891f call 0x14092df10
1407a8924 add al, 0xfe
1407a8926 movzx eax, al
1407a8929 cmp al, 3
1407a892b mov ecx, 3
1407a8930 cmovb ecx, eax
1407a8933 movzx eax, cl
1407a8936 lea ecx, [rax - 2]
1407a8939 cmp ecx, 2
1407a893c jae 0x1407a8b29
1407a8942 mov rsi, qword ptr [rbp + 0x2d8]
1407a8949 lea rcx, [rbp + 0x60]
1407a894d call 0x14092df10
1407a8952 cmp al, 2
1407a8954 jae 0x1407a8970
1407a8956 mov rax, qword ptr [rbp + 0x2e8]
1407a895d mov ecx, dword ptr [rax + 0x1c]
1407a8960 mov rdx, qword ptr [rax + 0x10]
1407a8964 mov dword ptr [rdx], ecx
1407a8966 mov rax, qword ptr [rax]
1407a8969 mov byte ptr [rax + 0x280], 1
1407a8970 mov r15, qword ptr [rbp + 0x2a8]
1407a8977 movss xmm6, dword ptr [rbx + 0xc]
1407a897c mov rdx, qword ptr [r13]
1407a8980 lea rcx, [rbp + 0xfc]
1407a8987 movaps xmm2, xmm6
1407a898a call 0x1408ef180
1407a898f movups xmm0, xmmword ptr [rbx]
1407a8992 movss xmm7, dword ptr [rbp + 0xfc]
1407a899a movss xmm8, dword ptr [rbp + 0x104]
1407a89a3 movss xmm1, dword ptr [rbx + 8]
1407a89a8 movlhps xmm1, xmm0
1407a89ab shufps xmm1, xmm0, 0xe2
1407a89af movaps xmm0, xmm7
1407a89b2 unpcklps xmm0, xmm8
1407a89b6 subps xmm1, xmm0
1407a89b9 mulps xmm1, xmm1
1407a89bc movshdup xmm0, xmm1
1407a89c0 addss xmm0, xmm1
1407a89c4 sqrtss xmm14, xmm0
1407a89c9 mov rax, qword ptr [rsi + 0x80]
1407a89d0 movss xmm0, dword ptr [rax + 4]
1407a89d5 mov rdx, qword ptr [rsi]
1407a89d8 mov rax, qword ptr [rsi + 0x20]
1407a89dc mov eax, dword ptr [rax + 4]
1407a89df xorps xmm2, xmm2
1407a89e2 cvtsi2ss xmm2, rax
1407a89e7 call 0x14127c950
1407a89ec mulss xmm0, xmm14
1407a89f1 movss xmm1, dword ptr [rip + 0x2185cf7]
1407a89f9 ucomiss xmm1, xmm0
1407a89fc jbe 0x1407a8bc1
1407a8a02 lea rcx, [rbp + 0x60]
1407a8a06 call 0x14092df10
1407a8a0b cmp al, 1
1407a8a0d ja 0x1407a8bc1
1407a8a13 mov rcx, qword ptr [rbp + 0x188]
1407a8a1a call 0x140833150
1407a8a1f test al, al
1407a8a21 je 0x1407a8bc1
1407a8a27 cmp byte ptr [rbp + 0x197], 2
1407a8a2e setae r13b
1407a8a32 mov rax, qword ptr [rbp + 0x48]
1407a8a36 mov qword ptr [rbp + 0x80], rax
1407a8a3d mov rax, qword ptr [rbp + 0x40]
1407a8a41 mov qword ptr [rbp + 0x88], rax
1407a8a48 mov qword ptr [rbp + 0x90], 3
1407a8a53 mov dword ptr [rbp + 0xd0], 0
1407a8a5d mov dword ptr [rbp + 0xd8], 0
1407a8a67 mov byte ptr [rbp + 0xe0], 0
1407a8a6e mov dword ptr [rbp + 0xe2], 0x40020101
1407a8a78 mov r8, qword ptr [rbp + 0x170]
1407a8a7f mov dword ptr [rbp + 0xe8], r8d
1407a8a86 mov rax, qword ptr [rbp + 0x188]
1407a8a8d mov byte ptr [rbp + 0xec], al
1407a8a93 mov rdx, qword ptr [rdi]
1407a8a96 lea r9, [rip + 0x22da0a3]
1407a8a9d lea rdi, [rbp + 0x120]
1407a8aa4 mov rcx, rdi
1407a8aa7 call 0x140a1a9f0
1407a8aac mov byte ptr [rbp + 0x12c], 0
1407a8ab3 mov rax, qword ptr [rbp + 0x158]
1407a8aba mov ecx, dword ptr [rbp + 0x168]
1407a8ac0 mov dword ptr [rax], ecx
1407a8ac2 mov rax, qword ptr [rbp + 0x160]
1407a8ac9 mov ecx, dword ptr [rbp + 0x190]
1407a8acf mov dword ptr [rax], ecx
1407a8ad1 mov rdx, qword ptr [rbp + 0x2e0]
1407a8ad8 mov eax, dword ptr [rdx + 0x1c]
1407a8adb mov rcx, qword ptr [rdx + 0x10]
1407a8adf mov dword ptr [rcx], eax
1407a8ae1 mov rax, qword ptr [rdx]
1407a8ae4 mov rcx, qword ptr [rbp + 0x118]
1407a8aeb mov qword ptr [rsp + 0x38], rcx
1407a8af0 mov qword ptr [rsp + 0x30], r15
1407a8af5 mov qword ptr [rsp + 0x28], rax
1407a8afa mov rax, qword ptr [rbp + 0x110]
1407a8b01 mov qword ptr [rsp + 0x20], rax
1407a8b06 lea rcx, [rbp + 0x80]
1407a8b0d mov edx, r13d
1407a8b10 mov r8, rdi
1407a8b13 mov r9, qword ptr [rbp + 0x108]
1407a8b1a call 0x141276e30
1407a8b1f movss xmm6, dword ptr [rbx + 0x18]
1407a8b24 jmp 0x1407a8ec8
1407a8b29 cmp eax, 1
1407a8b2c jne 0x1407a8bb2
1407a8b32 mov rdx, qword ptr [rdi]
1407a8b35 lea r9, [rip + 0x22d9fec]
1407a8b3c lea rcx, [rbp + 0x80]
1407a8b43 mov r8, qword ptr [rbp + 0x170]
1407a8b4a call 0x140a1a9f0
1407a8b4f movss xmm6, dword ptr [rbp + 0x84]
1407a8b57 xorps xmm0, xmm0
1407a8b5a maxss xmm6, xmm0
1407a8b5e movss xmm0, dword ptr [r15 + 0x30]
1407a8b64 movss xmm1, dword ptr [r15 + 0x38]
1407a8b6a subss xmm0, xmm13
1407a8b6f subss xmm1, xmm12
1407a8b74 call 0x140833a00
1407a8b79 movaps xmm7, xmm0
1407a8b7c mov rdx, qword ptr [r13]
1407a8b80 lea rcx, [rbp + 0x80]
1407a8b87 movaps xmm2, xmm6
1407a8b8a call 0x1408ef180
1407a8b8f mov eax, dword ptr [rbp + 0x88]
1407a8b95 mov dword ptr [rbx + 8], eax
1407a8b98 mov rax, qword ptr [rbp + 0x80]
1407a8b9f mov qword ptr [rbx], rax
1407a8ba2 movss dword ptr [rbx + 0xc], xmm6
1407a8ba7 movss dword ptr [rbx + 0x10], xmm7
1407a8bac movss dword ptr [rbx + 0x14], xmm9
1407a8bb2 movss dword ptr [rbx + 0x18], xmm9
1407a8bb8 movaps xmm6, xmm9
1407a8bbc jmp 0x1407a8ec8
1407a8bc1 subss xmm7, xmm13
1407a8bc6 subss xmm8, xmm12
1407a8bcb movaps xmm0, xmm7
1407a8bce movaps xmm1, xmm8
1407a8bd2 call 0x140833a00
1407a8bd7 movaps xmm7, xmm0
1407a8bda movzx eax, byte ptr [rip + 0x2e8c028]
1407a8be1 mov ecx, 0x18
1407a8be6 mov edx, 4
1407a8beb call 0x140613c10
1407a8bf0 mov qword ptr [rbp + 0x58], rax
1407a8bf4 test rax, rax
1407a8bf7 je 0x1407a91ac
1407a8bfd mov qword ptr [rbp + 0x50], r12
1407a8c01 lea rcx, [rbp + 0x80]
1407a8c08 lea rdx, [rbp + 0x38]
1407a8c0c movaps xmm2, xmm6
1407a8c0f call 0x140c9cde0
1407a8c14 mov r13, qword ptr [rbp + 0x2c8]
1407a8c1b mov eax, dword ptr [rbp + 0x104]
1407a8c21 mov rcx, qword ptr [rbp + 0x58]
1407a8c25 mov dword ptr [rcx + 8], eax
1407a8c28 mov rax, qword ptr [rbp + 0xfc]
1407a8c2f mov qword ptr [rcx], rax
1407a8c32 mov eax, dword ptr [rbp + 0x88]
1407a8c38 mov dword ptr [rcx + 0x14], eax
1407a8c3b mov rax, qword ptr [rbp + 0x80]
1407a8c42 mov qword ptr [rcx + 0xc], rax
1407a8c46 mov qword ptr [rbp + 0x120], 2
1407a8c51 mov qword ptr [rbp + 0x128], rcx
1407a8c58 mov qword ptr [rbp + 0x130], 2
1407a8c63 lea rdi, [rbp + 0x80]
1407a8c6a lea rdx, [rbp + 0x120]
1407a8c71 mov rcx, rdi
1407a8c74 call 0x140b04e70
1407a8c79 mov rax, qword ptr [rbp + 0x50]
1407a8c7d mov rdx, qword ptr [rax]
1407a8c80 mov rcx, rdi
1407a8c83 call 0x140b04f70
1407a8c88 movss xmm1, dword ptr [rbx + 0x14]
1407a8c8d subss xmm1, dword ptr [rbx + 0x10]
1407a8c92 movaps xmm0, xmm7
1407a8c95 call 0x140833b80
1407a8c9a movaps xmm6, xmm0
1407a8c9d mov r12, r15
1407a8ca0 cmp r14d, 1
1407a8ca4 je 0x1407a8cb0
1407a8ca6 cmp r14d, 3
1407a8caa jne 0x1407a8cb4
1407a8cac mov al, 4
1407a8cae jmp 0x1407a8cba
1407a8cb0 mov al, 2
1407a8cb2 jmp 0x1407a8cb6
1407a8cb4 mov al, 3
1407a8cb6 movaps xmm11, xmm6
1407a8cba mov r9, qword ptr [rbp + 0x108]
1407a8cc1 mov r8, qword ptr [rbp + 0x110]
1407a8cc8 mov r10, qword ptr [rbp + 0x118]
1407a8ccf mov rcx, qword ptr [rbp + 0x158]
1407a8cd6 mov r11d, dword ptr [rbp + 0x168]
1407a8cdd mov rdi, qword ptr [rbp + 0x160]
1407a8ce4 mov r14d, dword ptr [rbp + 0x190]
1407a8ceb cmp byte ptr [rbp + 0x197], 2
1407a8cf2 setae dl
1407a8cf5 mov rsi, qword ptr [rbp + 0x48]
1407a8cf9 mov qword ptr [rbp + 0x80], rsi
1407a8d00 mov r15, qword ptr [rbp + 0x40]
1407a8d04 mov qword ptr [rbp + 0x88], r15
1407a8d0b mov qword ptr [rbp + 0x90], rsi
1407a8d12 mov qword ptr [rbp + 0x98], r15
1407a8d19 mov esi, dword ptr [rbp + 0x74]
1407a8d1c mov dword ptr [rbp + 0xa0], esi
1407a8d22 mov esi, dword ptr [rbp + 0x78]
1407a8d25 mov dword ptr [rbp + 0xa4], esi
1407a8d2b movss dword ptr [rbp + 0xa8], xmm11
1407a8d34 movss dword ptr [rbp + 0xac], xmm10
1407a8d3d movss dword ptr [rbp + 0xb0], xmm6
1407a8d45 mov byte ptr [rbp + 0xc2], al
1407a8d4b movzx eax, byte ptr [rbp + 0x196]
1407a8d52 mov byte ptr [rbp + 0xc8], al
1407a8d58 mov dword ptr [rbp + 0xd0], 0
1407a8d62 mov dword ptr [rbp + 0xd8], 0
1407a8d6c mov byte ptr [rbp + 0xe0], 0
1407a8d73 mov byte ptr [rbp + 0xe2], 0
1407a8d7a mov word ptr [rbp + 0xe4], 0x4002
1407a8d83 mov rax, qword ptr [rbp + 0x170]
1407a8d8a mov dword ptr [rbp + 0xe8], eax
1407a8d90 mov rax, qword ptr [rbp + 0x188]
1407a8d97 mov byte ptr [rbp + 0xec], al
1407a8d9d mov eax, dword ptr [rbp + 0x180]
1407a8da3 mov dword ptr [rbp + 0x128], eax
1407a8da9 mov rax, qword ptr [rbp + 0x178]
1407a8db0 mov qword ptr [rbp + 0x120], rax
1407a8db7 mov byte ptr [rbp + 0x12c], 0
1407a8dbe mov dword ptr [rcx], r11d
1407a8dc1 mov dword ptr [rdi], r14d
1407a8dc4 mov r11, qword ptr [rbp + 0x2e0]
1407a8dcb mov eax, dword ptr [r11 + 0x1c]
1407a8dcf mov rcx, qword ptr [r11 + 0x10]
1407a8dd3 mov dword ptr [rcx], eax
1407a8dd5 mov rax, qword ptr [r11]
1407a8dd8 mov qword ptr [rsp + 0x38], r10
1407a8ddd mov qword ptr [rsp + 0x30], r12
1407a8de2 mov qword ptr [rsp + 0x28], rax
1407a8de7 mov qword ptr [rsp + 0x20], r8
1407a8dec lea rcx, [rbp + 0x80]
1407a8df3 lea r8, [rbp + 0x120]
1407a8dfa call 0x141276e30
1407a8dff movss dword ptr [rbx + 0x18], xmm6
1407a8e04 subss xmm9, xmm6
1407a8e09 andps xmm9, xmmword ptr [rip + 0x2185b6f]
1407a8e11 mov rax, qword ptr [r13]
1407a8e15 divss xmm9, dword ptr [rax + 0x28]
1407a8e1b mov edx, dword ptr [rbp + 0x7c]
1407a8e1e cmp dl, 6
1407a8e21 mov eax, 0x22
1407a8e26 mov ecx, 0x18
1407a8e2b cmove ecx, eax
1407a8e2e cmp dl, 4
1407a8e31 mov esi, 0xd
1407a8e36 cmovne esi, ecx
1407a8e39 mov eax, dword ptr [rbp + 0x104]
1407a8e3f mov dword ptr [rbp + 0x90], eax
1407a8e45 mov rax, qword ptr [rbp + 0xfc]
1407a8e4c mov qword ptr [rbp + 0x88], rax
1407a8e53 mov r13, qword ptr [rbp + 0x2c0]
1407a8e5a mov eax, dword ptr [r13 + 0x1c]
1407a8e5e mov rcx, qword ptr [r13 + 0x10]
1407a8e62 mov dword ptr [rcx], eax
1407a8e64 mov rdi, qword ptr [r13]
1407a8e68 mov rbx, qword ptr [rdi + 0x30]
1407a8e6c mov r14, qword ptr [rdi + 0x40]
1407a8e70 cmp rbx, qword ptr [rdi + 0x20]
1407a8e74 jne 0x1407a8e86
1407a8e76 lea rcx, [rdi + 0x20]
1407a8e7a lea rdx, [rip + 0x24790ff]
1407a8e81 call 0x140562ab0
1407a8e86 mov rax, qword ptr [rdi + 0x28]
1407a8e8a lea rcx, [rbx + rbx*4]
1407a8e8e mov qword ptr [rax + rcx*8], r14
1407a8e92 mov byte ptr [rax + rcx*8 + 8], sil
1407a8e97 movss dword ptr [rax + rcx*8 + 0xc], xmm9
1407a8e9e movups xmm0, xmmword ptr [rbp + 0x80]
1407a8ea5 movups xmmword ptr [rax + rcx*8 + 0x10], xmm0
1407a8eaa mov edx, dword ptr [rbp + 0x90]
1407a8eb0 mov dword ptr [rax + rcx*8 + 0x20], edx
1407a8eb4 mov byte ptr [rax + rcx*8 + 0x24], 0
1407a8eb9 inc rbx
1407a8ebc mov qword ptr [rdi + 0x30], rbx
1407a8ec0 inc qword ptr [rdi + 0x40]
1407a8ec4 mov r12, qword ptr [rbp + 0x50]
1407a8ec8 addss xmm6, dword ptr [rip + 0x2244cd4]
1407a8ed0 mulss xmm6, dword ptr [rip + 0x217c998]
1407a8ed8 movaps xmm0, xmm6
1407a8edb call 0x142923fd0
1407a8ee0 movaps xmm7, xmm0
1407a8ee3 movaps xmm0, xmm6
1407a8ee6 call 0x142923e80
1407a8eeb xorps xmm1, xmm1
1407a8eee mulss xmm1, xmm7
1407a8ef2 movaps xmm2, xmm1
1407a8ef5 unpcklps xmm2, xmm0
1407a8ef8 unpcklps xmm1, xmm7
1407a8efb movlhps xmm1, xmm2
1407a8efe movaps xmmword ptr [rbp - 0x20], xmm1
1407a8f02 mov byte ptr [rbp + 0x80], 0
1407a8f09 lea rax, [rip + 0x22d9c48]
1407a8f10 mov qword ptr [rbp + 0x88], rax
1407a8f17 mov qword ptr [rbp + 0x90], 0xc
1407a8f22 mov byte ptr [rbp + 0x98], 1
1407a8f29 lea rax, [rip + 0x22d9c34]
1407a8f30 mov qword ptr [rbp + 0xa0], rax
1407a8f37 mov qword ptr [rbp + 0xa8], 0x14
1407a8f42 mov rax, qword ptr [r12]
1407a8f46 mov qword ptr [rbp + 0x160], rax
1407a8f4d xor edi, edi
1407a8f4f mov rbx, rbp
1407a8f52 lea r12, [rbp + 0x120]
1407a8f59 nop dword ptr [rax]
1407a8f60 cmp rdi, 2
1407a8f64 je 0x1407a9135
1407a8f6a lea rax, [rdi + rdi*2]
1407a8f6e movzx r14d, byte ptr [rbp + rax*8 + 0x80]
1407a8f77 cmp r14b, 2
1407a8f7b je 0x1407a9135
1407a8f81 mov qword ptr [rbp + 0x158], rdi
1407a8f88 mov rsi, qword ptr [rbp + rax*8 + 0x88]
1407a8f90 mov rdi, qword ptr [rbp + rax*8 + 0x90]
1407a8f98 mov rax, qword ptr [rbp + 0x170]
1407a8f9f mov qword ptr [rbp - 8], rax
1407a8fa3 lea rax, [rbp - 8]
1407a8fa7 mov qword ptr [rbp], rax
1407a8fab lea rax, [rip - 0x6e0af2]
1407a8fb2 mov qword ptr [rbp + 8], rax
1407a8fb6 lea rax, [rbp + 0x178]
1407a8fbd mov qword ptr [rbp + 0x10], rax
1407a8fc1 lea rax, [rip + 0x5a2a38]
1407a8fc8 mov qword ptr [rbp + 0x18], rax
1407a8fcc lea rax, [rbp - 0x20]
1407a8fd0 mov qword ptr [rbp + 0x20], rax
1407a8fd4 lea rax, [rip + 0x5a1fb5]
1407a8fdb mov qword ptr [rbp + 0x28], rax
1407a8fdf lea rax, [rip + 0x22d9ba2]
1407a8fe6 mov qword ptr [rbp + 0x120], rax
1407a8fed mov qword ptr [rbp + 0x128], 3
1407a8ff8 mov r15, rbx
1407a8ffb mov qword ptr [rbp + 0x130], rbx
1407a9002 mov qword ptr [rbp + 0x138], 3
1407a900d mov qword ptr [rbp + 0x140], 0
1407a9018 lea rcx, [rbp - 0x38]
1407a901c mov rdx, r12
1407a901f call 0x140d79f90
1407a9024 mov rax, qword ptr [rbp - 0x38]
1407a9028 mov qword ptr [rbp + 0x188], rax
1407a902f mov rbx, qword ptr [rbp - 0x30]
1407a9033 mov r13, qword ptr [rbp - 0x28]
1407a9037 mov eax, dword ptr [rbp + 0x180]
1407a903d mov dword ptr [rbp + 0x128], eax
1407a9043 mov rax, qword ptr [rbp + 0x178]
1407a904a mov qword ptr [rbp + 0x120], rax
1407a9051 movaps xmm6, xmmword ptr [rbp - 0x20]
1407a9055 movzx eax, byte ptr [rip + 0x2e8bbad]
1407a905c mov ecx, 0x40
1407a9061 mov edx, 0x10
1407a9066 call 0x140613c10
1407a906b test rax, rax
1407a906e mov qword ptr [rbp + 0x168], rbx
1407a9075 je 0x1407a918f
1407a907b mov qword ptr [rax], 1
1407a9082 mov qword ptr [rax + 8], 1
1407a908a mov qword ptr [rax + 0x10], rsi
1407a908e mov qword ptr [rax + 0x18], rdi
1407a9092 movaps xmmword ptr [rax + 0x20], xmm6
1407a9096 mov rcx, qword ptr [rbp + 0x120]
1407a909d mov qword ptr [rax + 0x30], rcx
1407a90a1 mov ecx, dword ptr [rbp + 0x128]
1407a90a7 mov dword ptr [rax + 0x38], ecx
1407a90aa mov byte ptr [rax + 0x3c], r14b
1407a90ae lea rcx, [rip + 0x22d9b03]
1407a90b5 mov qword ptr [rsp + 0x20], rcx
1407a90ba mov rsi, r12
1407a90bd mov rcx, r12
1407a90c0 mov rdx, rbx
1407a90c3 mov r8, r13
1407a90c6 mov r9, rax
1407a90c9 call 0x14089d8b0
1407a90ce mov rbx, r15
1407a90d1 mov rax, qword ptr [rbp + 0x140]
1407a90d8 mov qword ptr [rbp + 0x20], rax
1407a90dc movups xmm0, xmmword ptr [rbp + 0x120]
1407a90e3 movups xmm1, xmmword ptr [rbp + 0x130]
1407a90ea movaps xmmword ptr [rbp + 0x10], xmm1
1407a90ee movaps xmmword ptr [rbp], xmm0
1407a90f2 mov rcx, qword ptr [rbp + 0x160]
1407a90f9 mov rdx, r15
1407a90fc call 0x140b2cf90
1407a9101 mov rdi, qword ptr [rbp + 0x158]
1407a9108 inc rdi
1407a910b mov rdx, qword ptr [rbp + 0x188]
1407a9112 test rdx, rdx
1407a9115 mov r12, rsi
1407a9118 je 0x1407a8f60
1407a911e mov r8d, 1
1407a9124 mov rcx, qword ptr [rbp + 0x168]
1407a912b call 0x140613c20
1407a9130 jmp 0x1407a8f60
1407a9135 movaps xmm6, xmmword ptr [rbp + 0x1a0]
1407a913c movaps xmm7, xmmword ptr [rbp + 0x1b0]
1407a9143 movaps xmm8, xmmword ptr [rbp + 0x1c0]
1407a914b movaps xmm9, xmmword ptr [rbp + 0x1d0]
1407a9153 movaps xmm10, xmmword ptr [rbp + 0x1e0]
1407a915b movaps xmm11, xmmword ptr [rbp + 0x1f0]
1407a9163 movaps xmm12, xmmword ptr [rbp + 0x200]
1407a916b movaps xmm13, xmmword ptr [rbp + 0x210]
1407a9173 movaps xmm14, xmmword ptr [rbp + 0x220]
1407a917b add rsp, 0x2b8
1407a9182 pop rbx
1407a9183 pop rdi
1407a9184 pop rsi
1407a9185 pop r12
1407a9187 pop r13
1407a9189 pop r14
1407a918b pop r15
1407a918d pop rbp
1407a918e ret 
1407a918f mov ecx, 0x10
1407a9194 mov edx, 0x40
1407a9199 call 0x1428d9000
1407a919e ud2 
1407a91a0 lea rcx, [rip + 0x22d98f9]
1407a91a7 call 0x1428d9310
1407a91ac mov ecx, 4
1407a91b1 mov edx, 0x18
1407a91b6 call 0x1428d9000
1407a91bb lea rcx, [rip + 0x22d98f6]
1407a91c2 lea r8, [rip + 0x22d9917]
1407a91c9 mov edx, 0x28
1407a91ce call 0x1428d9430
1407a91d3 int3 
1407a91d4 nop word ptr cs:[rax + rax]
1407a91e0 mov qword ptr [rsp + 0x10], rdx
1407a91e5 push rbp
1407a91e6 push r15
1407a91e8 push r14
1407a91ea push r13
1407a91ec push r12
1407a91ee push rsi
1407a91ef push rdi
1407a91f0 push rbx
1407a91f1 sub rsp, 0xd8
1407a91f8 lea rbp, [rdx + 0x80]
1407a91ff movaps xmmword ptr [rsp + 0x40], xmm14
1407a9205 movaps xmmword ptr [rsp + 0x50], xmm13
1407a920b movaps xmmword ptr [rsp + 0x60], xmm12
1407a9211 movaps xmmword ptr [rsp + 0x70], xmm11
1407a9217 movaps xmmword ptr [rsp + 0x80], xmm10
1407a9220 movaps xmmword ptr [rsp + 0x90], xmm9
1407a9229 movaps xmmword ptr [rsp + 0xa0], xmm8
1407a9232 movaps xmmword ptr [rsp + 0xb0], xmm7
1407a923a movaps xmmword ptr [rsp + 0xc0], xmm6
1407a9242 mov edx, 0x18
1407a9247 mov r8d, 4
1407a924d mov rcx, qword ptr [rbp + 0x58]
1407a9251 call 0x140613c20
1407a9256 movaps xmm6, xmmword ptr [rsp + 0xc0]
1407a925e movaps xmm7, xmmword ptr [rsp + 0xb0]
1407a9266 movaps xmm8, xmmword ptr [rsp + 0xa0]
1407a926f movaps xmm9, xmmword ptr [rsp + 0x90]
1407a9278 movaps xmm10, xmmword ptr [rsp + 0x80]
1407a9281 movaps xmm11, xmmword ptr [rsp + 0x70]
1407a9287 movaps xmm12, xmmword ptr [rsp + 0x60]
1407a928d movaps xmm13, xmmword ptr [rsp + 0x50]
1407a9293 movaps xmm14, xmmword ptr [rsp + 0x40]
1407a9299 add rsp, 0xd8
1407a92a0 pop rbx
1407a92a1 pop rdi
1407a92a2 pop rsi
1407a92a3 pop r12
1407a92a5 pop r13
1407a92a7 pop r14
1407a92a9 pop r15
1407a92ab pop rbp
1407a92ac ret 
1407a92ad nop dword ptr [rax]
1407a92b0 mov qword ptr [rsp + 0x10], rdx
1407a92b5 push rbp
1407a92b6 push r15
1407a92b8 push r14
1407a92ba push r13
1407a92bc push r12
1407a92be push rsi
1407a92bf push rdi
1407a92c0 push rbx
1407a92c1 sub rsp, 0xd8
1407a92c8 lea rbp, [rdx + 0x80]
1407a92cf movaps xmmword ptr [rsp + 0x40], xmm14
1407a92d5 movaps xmmword ptr [rsp + 0x50], xmm13
1407a92db movaps xmmword ptr [rsp + 0x60], xmm12
1407a92e1 movaps xmmword ptr [rsp + 0x70], xmm11
1407a92e7 movaps xmmword ptr [rsp + 0x80], xmm10
1407a92f0 movaps xmmword ptr [rsp + 0x90], xmm9
1407a92f9 movaps xmmword ptr [rsp + 0xa0], xmm8
1407a9302 movaps xmmword ptr [rsp + 0xb0], xmm7
1407a930a movaps xmmword ptr [rsp + 0xc0], xmm6
1407a9312 cmp qword ptr [rbp + 0x188], 0
1407a931a je 0x1407a9335
1407a931c mov r8d, 1
1407a9322 mov rcx, qword ptr [rbp + 0x168]
1407a9329 mov rdx, qword ptr [rbp + 0x188]
1407a9330 call 0x140613c20
1407a9335 movaps xmm6, xmmword ptr [rsp + 0xc0]
1407a933d movaps xmm7, xmmword ptr [rsp + 0xb0]
1407a9345 movaps xmm8, xmmword ptr [rsp + 0xa0]
1407a934e movaps xmm9, xmmword ptr [rsp + 0x90]
1407a9357 movaps xmm10, xmmword ptr [rsp + 0x80]
1407a9360 movaps xmm11, xmmword ptr [rsp + 0x70]
1407a9366 movaps xmm12, xmmword ptr [rsp + 0x60]
1407a936c movaps xmm13, xmmword ptr [rsp + 0x50]
1407a9372 movaps xmm14, xmmword ptr [rsp + 0x40]
1407a9378 add rsp, 0xd8
1407a937f pop rbx
1407a9380 pop rdi
1407a9381 pop rsi
1407a9382 pop r12
1407a9384 pop r13
1407a9386 pop r14
1407a9388 pop r15
1407a938a pop rbp
1407a938b ret 