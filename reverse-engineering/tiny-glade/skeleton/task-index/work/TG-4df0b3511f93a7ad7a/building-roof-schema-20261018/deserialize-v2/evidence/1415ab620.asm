0x1415ab620 55 push rbp
0x1415ab621 4157 push r15
0x1415ab623 4156 push r14
0x1415ab625 4155 push r13
0x1415ab627 4154 push r12
0x1415ab629 56 push rsi
0x1415ab62a 57 push rdi
0x1415ab62b 53 push rbx
0x1415ab62c 4881ecd8000000 sub rsp, 0xd8
0x1415ab633 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415ab63b 440f294d40 movaps xmmword ptr [rbp + 0x40], xmm9
0x1415ab640 440f294530 movaps xmmword ptr [rbp + 0x30], xmm8
0x1415ab645 0f297d20 movaps xmmword ptr [rbp + 0x20], xmm7
0x1415ab649 0f297510 movaps xmmword ptr [rbp + 0x10], xmm6
0x1415ab64d 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x1415ab655 4889ce mov rsi, rcx
0x1415ab658 4c8b7210 mov r14, qword ptr [rdx + 0x10]
0x1415ab65c 488d4db8 lea rcx, [rbp - 0x48]
0x1415ab660 e81bca5dff call 0x140b88080
0x1415ab665 4c8b65c0 mov r12, qword ptr [rbp - 0x40]
0x1415ab669 488b7dd0 mov rdi, qword ptr [rbp - 0x30]
0x1415ab66d 4939fc cmp r12, rdi
0x1415ab670 7442 je 0x1415ab6b4
0x1415ab672 498d5c2420 lea rbx, [r12 + 0x20]
0x1415ab677 48895dc0 mov qword ptr [rbp - 0x40], rbx
0x1415ab67b 410fb60424 movzx eax, byte ptr [r12]
0x1415ab680 3c06 cmp al, 6
0x1415ab682 7433 je 0x1415ab6b7
0x1415ab684 8845d8 mov byte ptr [rbp - 0x28], al
0x1415ab687 410f10442401 movups xmm0, xmmword ptr [r12 + 1]
0x1415ab68d 410f104c2410 movups xmm1, xmmword ptr [r12 + 0x10]
0x1415ab693 0f1145d9 movups xmmword ptr [rbp - 0x27], xmm0
0x1415ab697 0f114de8 movups xmmword ptr [rbp - 0x18], xmm1
0x1415ab69b 488d4df8 lea rcx, [rbp - 8]
0x1415ab69f 488d55d8 lea rdx, [rbp - 0x28]
0x1415ab6a3 e8c8ecfeff call 0x14159a370
0x1415ab6a8 837df801 cmp dword ptr [rbp - 8], 1
0x1415ab6ac 757f jne 0x1415ab72d
0x1415ab6ae 488b4500 mov rax, qword ptr [rbp]
0x1415ab6b2 eb18 jmp 0x1415ab6cc
0x1415ab6b4 4c89e3 mov rbx, r12
0x1415ab6b7 488d15a2f56c01 lea rdx, [rip + 0x16cf5a2]
0x1415ab6be 4c8d059be96c01 lea r8, [rip + 0x16ce99b]
0x1415ab6c5 31c9 xor ecx, ecx
0x1415ab6c7 e854e63401 call 0x1428f9d20
0x1415ab6cc 48894608 mov qword ptr [rsi + 8], rax
0x1415ab6d0 c70601000000 mov dword ptr [rsi], 1
0x1415ab6d6 4829df sub rdi, rbx
0x1415ab6d9 48c1ef05 shr rdi, 5
0x1415ab6dd 4889d9 mov rcx, rbx
0x1415ab6e0 4889fa mov rdx, rdi
0x1415ab6e3 e888d62dff call 0x140888d70
0x1415ab6e8 488b55c8 mov rdx, qword ptr [rbp - 0x38]
0x1415ab6ec 4885d2 test rdx, rdx
0x1415ab6ef 7413 je 0x1415ab704
0x1415ab6f1 488b4db8 mov rcx, qword ptr [rbp - 0x48]
0x1415ab6f5 48c1e205 shl rdx, 5
0x1415ab6f9 41b808000000 mov r8d, 8
0x1415ab6ff e81c8506ff call 0x140613c20
0x1415ab704 4889f0 mov rax, rsi
0x1415ab707 0f287510 movaps xmm6, xmmword ptr [rbp + 0x10]
0x1415ab70b 0f287d20 movaps xmm7, xmmword ptr [rbp + 0x20]
0x1415ab70f 440f284530 movaps xmm8, xmmword ptr [rbp + 0x30]
0x1415ab714 440f284d40 movaps xmm9, xmmword ptr [rbp + 0x40]
0x1415ab719 4881c4d8000000 add rsp, 0xd8
0x1415ab720 5b pop rbx
0x1415ab721 5f pop rdi
0x1415ab722 5e pop rsi
0x1415ab723 415c pop r12
0x1415ab725 415d pop r13
0x1415ab727 415e pop r14
0x1415ab729 415f pop r15
0x1415ab72b 5d pop rbp
0x1415ab72c c3 ret 
0x1415ab72d 4839fb cmp rbx, rdi
0x1415ab730 7418 je 0x1415ab74a
0x1415ab732 f30f1075fc movss xmm6, dword ptr [rbp - 4]
0x1415ab737 498d5c2440 lea rbx, [r12 + 0x40]
0x1415ab73c 48895dc0 mov qword ptr [rbp - 0x40], rbx
0x1415ab740 410fb6442420 movzx eax, byte ptr [r12 + 0x20]
0x1415ab746 3c06 cmp al, 6
0x1415ab748 751d jne 0x1415ab767
0x1415ab74a 488d150ff56c01 lea rdx, [rip + 0x16cf50f]
0x1415ab751 4c8d0508e96c01 lea r8, [rip + 0x16ce908]
0x1415ab758 b901000000 mov ecx, 1
0x1415ab75d e8bee53401 call 0x1428f9d20
0x1415ab762 e965ffffff jmp 0x1415ab6cc
0x1415ab767 8845d8 mov byte ptr [rbp - 0x28], al
0x1415ab76a 410f10442421 movups xmm0, xmmword ptr [r12 + 0x21]
0x1415ab770 410f104c2430 movups xmm1, xmmword ptr [r12 + 0x30]
0x1415ab776 0f1145d9 movups xmmword ptr [rbp - 0x27], xmm0
0x1415ab77a 0f114de8 movups xmmword ptr [rbp - 0x18], xmm1
0x1415ab77e 488d4df8 lea rcx, [rbp - 8]
0x1415ab782 488d55d8 lea rdx, [rbp - 0x28]
0x1415ab786 e8e5ebfeff call 0x14159a370
0x1415ab78b 837df801 cmp dword ptr [rbp - 8], 1
0x1415ab78f 0f8419ffffff je 0x1415ab6ae
0x1415ab795 4839fb cmp rbx, rdi
0x1415ab798 7418 je 0x1415ab7b2
0x1415ab79a f30f107dfc movss xmm7, dword ptr [rbp - 4]
0x1415ab79f 498d5c2460 lea rbx, [r12 + 0x60]
0x1415ab7a4 48895dc0 mov qword ptr [rbp - 0x40], rbx
0x1415ab7a8 410fb6442440 movzx eax, byte ptr [r12 + 0x40]
0x1415ab7ae 3c06 cmp al, 6
0x1415ab7b0 751d jne 0x1415ab7cf
0x1415ab7b2 488d15a7f46c01 lea rdx, [rip + 0x16cf4a7]
0x1415ab7b9 4c8d05a0e86c01 lea r8, [rip + 0x16ce8a0]
0x1415ab7c0 b902000000 mov ecx, 2
0x1415ab7c5 e856e53401 call 0x1428f9d20
0x1415ab7ca e9fdfeffff jmp 0x1415ab6cc
0x1415ab7cf 8845d8 mov byte ptr [rbp - 0x28], al
0x1415ab7d2 410f10442441 movups xmm0, xmmword ptr [r12 + 0x41]
0x1415ab7d8 410f104c2450 movups xmm1, xmmword ptr [r12 + 0x50]
0x1415ab7de 0f1145d9 movups xmmword ptr [rbp - 0x27], xmm0
0x1415ab7e2 0f114de8 movups xmmword ptr [rbp - 0x18], xmm1
0x1415ab7e6 48c744242002000000 mov qword ptr [rsp + 0x20], 2
0x1415ab7ef 4c8d0552bc6c01 lea r8, [rip + 0x16cbc52]
0x1415ab7f6 488d4df8 lea rcx, [rbp - 8]
0x1415ab7fa 488d55d8 lea rdx, [rbp - 0x28]
0x1415ab7fe 41b904000000 mov r9d, 4
0x1415ab804 e80782ffff call 0x1415a3a10
0x1415ab809 837df801 cmp dword ptr [rbp - 8], 1
0x1415ab80d 0f849bfeffff je 0x1415ab6ae
0x1415ab813 f20f1045fc movsd xmm0, qword ptr [rbp - 4]
0x1415ab818 f20f1145a8 movsd qword ptr [rbp - 0x58], xmm0
0x1415ab81d 4c8b7da8 mov r15, qword ptr [rbp - 0x58]
0x1415ab821 4839fb cmp rbx, rdi
0x1415ab824 7416 je 0x1415ab83c
0x1415ab826 498d9c2480000000 lea rbx, [r12 + 0x80]
0x1415ab82e 48895dc0 mov qword ptr [rbp - 0x40], rbx
0x1415ab832 410fb6442460 movzx eax, byte ptr [r12 + 0x60]
0x1415ab838 3c06 cmp al, 6
0x1415ab83a 754e jne 0x1415ab88a
0x1415ab83c f3440f10052ba13701 movss xmm8, dword ptr [rip + 0x137a12b]
0x1415ab845 4839fb cmp rbx, rdi
0x1415ab848 7479 je 0x1415ab8c3
0x1415ab84a 4c8d6b20 lea r13, [rbx + 0x20]
0x1415ab84e 4c896dc0 mov qword ptr [rbp - 0x40], r13
0x1415ab852 0fb603 movzx eax, byte ptr [rbx]
0x1415ab855 3c06 cmp al, 6
0x1415ab857 746d je 0x1415ab8c6
0x1415ab859 8845d8 mov byte ptr [rbp - 0x28], al
0x1415ab85c 0f104301 movups xmm0, xmmword ptr [rbx + 1]
0x1415ab860 0f104b10 movups xmm1, xmmword ptr [rbx + 0x10]
0x1415ab864 0f1145d9 movups xmmword ptr [rbp - 0x27], xmm0
0x1415ab868 0f114de8 movups xmmword ptr [rbp - 0x18], xmm1
0x1415ab86c 488d4df8 lea rcx, [rbp - 8]
0x1415ab870 488d55d8 lea rdx, [rbp - 0x28]
0x1415ab874 e827f71300 call 0x1416eafa0
0x1415ab879 807df801 cmp byte ptr [rbp - 8], 1
0x1415ab87d 0f8425010000 je 0x1415ab9a8
0x1415ab883 440fb665f9 movzx r12d, byte ptr [rbp - 7]
0x1415ab888 eb3f jmp 0x1415ab8c9
0x1415ab88a 8845d8 mov byte ptr [rbp - 0x28], al
0x1415ab88d 410f10442461 movups xmm0, xmmword ptr [r12 + 0x61]
0x1415ab893 410f104c2470 movups xmm1, xmmword ptr [r12 + 0x70]
0x1415ab899 0f1145d9 movups xmmword ptr [rbp - 0x27], xmm0
0x1415ab89d 0f114de8 movups xmmword ptr [rbp - 0x18], xmm1
0x1415ab8a1 488d4df8 lea rcx, [rbp - 8]
0x1415ab8a5 488d55d8 lea rdx, [rbp - 0x28]
0x1415ab8a9 e8c2eafeff call 0x14159a370
0x1415ab8ae 837df801 cmp dword ptr [rbp - 8], 1
0x1415ab8b2 0f84f6fdffff je 0x1415ab6ae
0x1415ab8b8 f3440f1045fc movss xmm8, dword ptr [rbp - 4]
0x1415ab8be 4839fb cmp rbx, rdi
0x1415ab8c1 7587 jne 0x1415ab84a
0x1415ab8c3 4989dd mov r13, rbx
0x1415ab8c6 4531e4 xor r12d, r12d
0x1415ab8c9 4939fd cmp r13, rdi
0x1415ab8cc 7445 je 0x1415ab913
0x1415ab8ce 498d5d20 lea rbx, [r13 + 0x20]
0x1415ab8d2 48895dc0 mov qword ptr [rbp - 0x40], rbx
0x1415ab8d6 410fb64500 movzx eax, byte ptr [r13]
0x1415ab8db 3c06 cmp al, 6
0x1415ab8dd 7437 je 0x1415ab916
0x1415ab8df 8845d8 mov byte ptr [rbp - 0x28], al
0x1415ab8e2 410f104501 movups xmm0, xmmword ptr [r13 + 1]
0x1415ab8e7 410f104d10 movups xmm1, xmmword ptr [r13 + 0x10]
0x1415ab8ec 0f1145d9 movups xmmword ptr [rbp - 0x27], xmm0
0x1415ab8f0 0f114de8 movups xmmword ptr [rbp - 0x18], xmm1
0x1415ab8f4 488d4df8 lea rcx, [rbp - 8]
0x1415ab8f8 488d55d8 lea rdx, [rbp - 0x28]
0x1415ab8fc e86feafeff call 0x14159a370
0x1415ab901 837df801 cmp dword ptr [rbp - 8], 1
0x1415ab905 0f84a3fdffff je 0x1415ab6ae
0x1415ab90b f3440f104dfc movss xmm9, dword ptr [rbp - 4]
0x1415ab911 eb0c jmp 0x1415ab91f
0x1415ab913 4c89eb mov rbx, r13
0x1415ab916 f3440f100d49174201 movss xmm9, dword ptr [rip + 0x1421749]
0x1415ab91f 4989fd mov r13, rdi
0x1415ab922 4839fb cmp rbx, rdi
0x1415ab925 740f je 0x1415ab936
0x1415ab927 4c8d6b20 lea r13, [rbx + 0x20]
0x1415ab92b 4c896dc0 mov qword ptr [rbp - 0x40], r13
0x1415ab92f 0fb603 movzx eax, byte ptr [rbx]
0x1415ab932 3c06 cmp al, 6
0x1415ab934 754c jne 0x1415ab982
0x1415ab936 31c0 xor eax, eax
0x1415ab938 4c89f9 mov rcx, r15
0x1415ab93b 48c1e920 shr rcx, 0x20
0x1415ab93f 894db0 mov dword ptr [rbp - 0x50], ecx
0x1415ab942 f30f1175b4 movss dword ptr [rbp - 0x4c], xmm6
0x1415ab947 4c39ef cmp rdi, r13
0x1415ab94a 7571 jne 0x1415ab9bd
0x1415ab94c 488b4db0 mov rcx, qword ptr [rbp - 0x50]
0x1415ab950 44897e04 mov dword ptr [rsi + 4], r15d
0x1415ab954 48894e08 mov qword ptr [rsi + 8], rcx
0x1415ab958 f30f117e10 movss dword ptr [rsi + 0x10], xmm7
0x1415ab95d f3440f114614 movss dword ptr [rsi + 0x14], xmm8
0x1415ab963 f3440f114e18 movss dword ptr [rsi + 0x18], xmm9
0x1415ab969 4488661c mov byte ptr [rsi + 0x1c], r12b
0x1415ab96d 88461d mov byte ptr [rsi + 0x1d], al
0x1415ab970 31c0 xor eax, eax
0x1415ab972 8906 mov dword ptr [rsi], eax
0x1415ab974 488d4db8 lea rcx, [rbp - 0x48]
0x1415ab978 e8e3eb2dff call 0x14088a560
0x1415ab97d e982fdffff jmp 0x1415ab704
0x1415ab982 8845d8 mov byte ptr [rbp - 0x28], al
0x1415ab985 0f104301 movups xmm0, xmmword ptr [rbx + 1]
0x1415ab989 0f104b10 movups xmm1, xmmword ptr [rbx + 0x10]
0x1415ab98d 0f1145d9 movups xmmword ptr [rbp - 0x27], xmm0
0x1415ab991 0f114de8 movups xmmword ptr [rbp - 0x18], xmm1
0x1415ab995 488d4df8 lea rcx, [rbp - 8]
0x1415ab999 488d55d8 lea rdx, [rbp - 0x28]
0x1415ab99d e8bee61300 call 0x1416ea060
0x1415ab9a2 807df800 cmp byte ptr [rbp - 8], 0
0x1415ab9a6 740c je 0x1415ab9b4
0x1415ab9a8 488b4500 mov rax, qword ptr [rbp]
0x1415ab9ac 4c89eb mov rbx, r13
0x1415ab9af e918fdffff jmp 0x1415ab6cc
0x1415ab9b4 0fb645f9 movzx eax, byte ptr [rbp - 7]
0x1415ab9b8 e97bffffff jmp 0x1415ab938
0x1415ab9bd 488d158ce66c01 lea rdx, [rip + 0x16ce68c]
0x1415ab9c4 4c8d0595e66c01 lea r8, [rip + 0x16ce695]
0x1415ab9cb 4c89f1 mov rcx, r14
0x1415ab9ce e84de33401 call 0x1428f9d20
0x1415ab9d3 48894608 mov qword ptr [rsi + 8], rax
0x1415ab9d7 b801000000 mov eax, 1
0x1415ab9dc eb94 jmp 0x1415ab972
0x1415ab9de 6690 nop 
0x1415ab9e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415ab9e5 55 push rbp
0x1415ab9e6 4157 push r15
0x1415ab9e8 4156 push r14
0x1415ab9ea 4155 push r13
0x1415ab9ec 4154 push r12
0x1415ab9ee 56 push rsi
0x1415ab9ef 57 push rdi
0x1415ab9f0 53 push rbx
0x1415ab9f1 4883ec68 sub rsp, 0x68
0x1415ab9f5 488daa80000000 lea rbp, [rdx + 0x80]
0x1415ab9fc 440f294c2420 movaps xmmword ptr [rsp + 0x20], xmm9
0x1415aba02 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x1415aba08 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x1415aba0d 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1415aba12 488d4db8 lea rcx, [rbp - 0x48]
0x1415aba16 e845eb2dff call 0x14088a560
0x1415aba1b 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1415aba20 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x1415aba25 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x1415aba2b 440f284c2420 movaps xmm9, xmmword ptr [rsp + 0x20]
0x1415aba31 4883c468 add rsp, 0x68
0x1415aba35 5b pop rbx
0x1415aba36 5f pop rdi
0x1415aba37 5e pop rsi
0x1415aba38 415c pop r12
0x1415aba3a 415d pop r13
0x1415aba3c 415e pop r14
0x1415aba3e 415f pop r15
0x1415aba40 5d pop rbp
0x1415aba41 c3 ret 
0x1415aba42 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415aba50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415aba55 55 push rbp
0x1415aba56 4157 push r15
0x1415aba58 4156 push r14
0x1415aba5a 4155 push r13
0x1415aba5c 4154 push r12
0x1415aba5e 56 push rsi
0x1415aba5f 57 push rdi
0x1415aba60 53 push rbx
0x1415aba61 4883ec68 sub rsp, 0x68
0x1415aba65 488daa80000000 lea rbp, [rdx + 0x80]
0x1415aba6c 440f294c2420 movaps xmmword ptr [rsp + 0x20], xmm9
0x1415aba72 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x1415aba78 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x1415aba7d 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1415aba82 488b55c8 mov rdx, qword ptr [rbp - 0x38]
0x1415aba86 4885d2 test rdx, rdx
0x1415aba89 7413 je 0x1415aba9e
0x1415aba8b 488b4db8 mov rcx, qword ptr [rbp - 0x48]
0x1415aba8f 48c1e205 shl rdx, 5
0x1415aba93 41b808000000 mov r8d, 8
0x1415aba99 e8828106ff call 0x140613c20
0x1415aba9e 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1415abaa3 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x1415abaa8 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x1415abaae 440f284c2420 movaps xmm9, xmmword ptr [rsp + 0x20]
0x1415abab4 4883c468 add rsp, 0x68
0x1415abab8 5b pop rbx
0x1415abab9 5f pop rdi
0x1415ababa 5e pop rsi
0x1415ababb 415c pop r12
0x1415ababd 415d pop r13
0x1415ababf 415e pop r14
0x1415abac1 415f pop r15
0x1415abac3 5d pop rbp
0x1415abac4 c3 ret 
