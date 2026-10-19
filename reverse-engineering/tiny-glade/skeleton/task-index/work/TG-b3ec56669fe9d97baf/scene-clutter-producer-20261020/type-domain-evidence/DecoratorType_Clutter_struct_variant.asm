0x14140e700 55 push rbp
0x14140e701 4157 push r15
0x14140e703 4156 push r14
0x14140e705 4155 push r13
0x14140e707 4154 push r12
0x14140e709 56 push rsi
0x14140e70a 57 push rdi
0x14140e70b 53 push rbx
0x14140e70c 4883ec58 sub rsp, 0x58
0x14140e710 488d6c2450 lea rbp, [rsp + 0x50]
0x14140e715 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
0x14140e71d 4889d7 mov rdi, rdx
0x14140e720 4889ce mov rsi, rcx
0x14140e723 488b4a20 mov rcx, qword ptr [rdx + 0x20]
0x14140e727 488b4228 mov rax, qword ptr [rdx + 0x28]
0x14140e72b 4839c8 cmp rax, rcx
0x14140e72e 7346 jae 0x14140e776
0x14140e730 488d5f18 lea rbx, [rdi + 0x18]
0x14140e734 488b5718 mov rdx, qword ptr [rdi + 0x18]
0x14140e738 48ffc0 inc rax
0x14140e73b 4989c8 mov r8, rcx
0x14140e73e 49f7d8 neg r8
0x14140e741 49bc0026000001000000 movabs r12, 0x100002600
0x14140e74b 0f1f440000 nop dword ptr [rax + rax]
0x14140e750 440fb64c02ff movzx r9d, byte ptr [rdx + rax - 1]
0x14140e756 4983f920 cmp r9, 0x20
0x14140e75a 7733 ja 0x14140e78f
0x14140e75c 4d0fa3cc bt r12, r9
0x14140e760 732d jae 0x14140e78f
0x14140e762 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e766 4d8d0c00 lea r9, [r8 + rax]
0x14140e76a 49ffc1 inc r9
0x14140e76d 48ffc0 inc rax
0x14140e770 4983f901 cmp r9, 1
0x14140e774 75da jne 0x14140e750
0x14140e776 48c745e005000000 mov qword ptr [rbp - 0x20], 5
0x14140e77e 488d55e0 lea rdx, [rbp - 0x20]
0x14140e782 4889f9 mov rcx, rdi
0x14140e785 e896214901 call 0x1428a0920
0x14140e78a e927030000 jmp 0x14140eab6
0x14140e78f 4183f95b cmp r9d, 0x5b
0x14140e793 0f84be010000 je 0x14140e957
0x14140e799 4183f97b cmp r9d, 0x7b
0x14140e79d 0f85f5020000 jne 0x14140ea98
0x14140e7a3 fe4f30 dec byte ptr [rdi + 0x30]
0x14140e7a6 0f848c030000 je 0x14140eb38
0x14140e7ac 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e7b0 4839c8 cmp rax, rcx
0x14140e7b3 0f8363020000 jae 0x14140ea1c
0x14140e7b9 41b538 mov r13b, 0x38
0x14140e7bc 41b701 mov r15b, 1
0x14140e7bf 4c8d75e0 lea r14, [rbp - 0x20]
0x14140e7c3 488b13 mov rdx, qword ptr [rbx]
0x14140e7c6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14140e7d0 440fb60402 movzx r8d, byte ptr [rdx + rax]
0x14140e7d5 4983f82c cmp r8, 0x2c
0x14140e7d9 7775 ja 0x14140e850
0x14140e7db 4d0fa3c4 bt r12, r8
0x14140e7df 731f jae 0x14140e800
0x14140e7e1 48ffc0 inc rax
0x14140e7e4 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e7e8 4839c1 cmp rcx, rax
0x14140e7eb 75e3 jne 0x14140e7d0
0x14140e7ed e92a020000 jmp 0x14140ea1c
0x14140e7f2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14140e800 4983f82c cmp r8, 0x2c
0x14140e804 754a jne 0x14140e850
0x14140e806 41f6c701 test r15b, 1
0x14140e80a 0f851b030000 jne 0x14140eb2b
0x14140e810 48ffc0 inc rax
0x14140e813 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e817 4839c8 cmp rax, rcx
0x14140e81a 0f8306020000 jae 0x14140ea26
0x14140e820 440fb60402 movzx r8d, byte ptr [rdx + rax]
0x14140e825 4983f820 cmp r8, 0x20
0x14140e829 773c ja 0x14140e867
0x14140e82b 4d0fa3c4 bt r12, r8
0x14140e82f 7336 jae 0x14140e867
0x14140e831 48ffc0 inc rax
0x14140e834 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e838 4839c1 cmp rcx, rax
0x14140e83b 75e3 jne 0x14140e820
0x14140e83d e9e4010000 jmp 0x14140ea26
0x14140e842 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14140e850 4183f87d cmp r8d, 0x7d
0x14140e854 0f848c010000 je 0x14140e9e6
0x14140e85a 41f6c701 test r15b, 1
0x14140e85e 0f84e1020000 je 0x14140eb45
0x14140e864 4531ff xor r15d, r15d
0x14140e867 4180f822 cmp r8b, 0x22
0x14140e86b 0f85a4020000 jne 0x14140eb15
0x14140e871 48ffc0 inc rax
0x14140e874 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e878 48c7471000000000 mov qword ptr [rdi + 0x10], 0
0x14140e880 4c89f1 mov rcx, r14
0x14140e883 4889da mov rdx, rbx
0x14140e886 4989f8 mov r8, rdi
0x14140e889 e882c077ff call 0x140b8a910
0x14140e88e 837de002 cmp dword ptr [rbp - 0x20], 2
0x14140e892 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x14140e896 0f849e010000 je 0x14140ea3a
0x14140e89c 48837df002 cmp qword ptr [rbp - 0x10], 2
0x14140e8a1 750c jne 0x14140e8af
0x14140e8a3 0fb700 movzx eax, word ptr [rax]
0x14140e8a6 663b05da046c01 cmp ax, word ptr [rip + 0x16c04da]
0x14140e8ad 7427 je 0x14140e8d6
0x14140e8af 4889f9 mov rcx, rdi
0x14140e8b2 e849a8ffff call 0x141409100
0x14140e8b7 4885c0 test rax, rax
0x14140e8ba 0f857a010000 jne 0x14140ea3a
0x14140e8c0 488b4f20 mov rcx, qword ptr [rdi + 0x20]
0x14140e8c4 488b4728 mov rax, qword ptr [rdi + 0x28]
0x14140e8c8 4839c8 cmp rax, rcx
0x14140e8cb 0f82f2feffff jb 0x14140e7c3
0x14140e8d1 e946010000 jmp 0x14140ea1c
0x14140e8d6 4180fd38 cmp r13b, 0x38
0x14140e8da 0f8572020000 jne 0x14140eb52
0x14140e8e0 488b4f20 mov rcx, qword ptr [rdi + 0x20]
0x14140e8e4 488b4728 mov rax, qword ptr [rdi + 0x28]
0x14140e8e8 4839c8 cmp rax, rcx
0x14140e8eb 0f832b010000 jae 0x14140ea1c
0x14140e8f1 488b13 mov rdx, qword ptr [rbx]
0x14140e8f4 48ffc0 inc rax
0x14140e8f7 48f7d9 neg rcx
0x14140e8fa 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
0x14140e900 4983f83a cmp r8, 0x3a
0x14140e904 0f875e020000 ja 0x14140eb68
0x14140e90a 4d0fa3c4 bt r12, r8
0x14140e90e 7319 jae 0x14140e929
0x14140e910 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e914 4c8d0401 lea r8, [rcx + rax]
0x14140e918 49ffc0 inc r8
0x14140e91b 48ffc0 inc rax
0x14140e91e 4983f801 cmp r8, 1
0x14140e922 75d6 jne 0x14140e8fa
0x14140e924 e9f3000000 jmp 0x14140ea1c
0x14140e929 4983f83a cmp r8, 0x3a
0x14140e92d 0f8535020000 jne 0x14140eb68
0x14140e933 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e937 488d4dd0 lea rcx, [rbp - 0x30]
0x14140e93b 4889fa mov rdx, rdi
0x14140e93e e89dca3800 call 0x14179b3e0
0x14140e943 f645d001 test byte ptr [rbp - 0x30], 1
0x14140e947 0f85bf010000 jne 0x14140eb0c
0x14140e94d 440fb66dd1 movzx r13d, byte ptr [rbp - 0x2f]
0x14140e952 e969ffffff jmp 0x14140e8c0
0x14140e957 fe4f30 dec byte ptr [rdi + 0x30]
0x14140e95a 0f84d8010000 je 0x14140eb38
0x14140e960 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e964 4839c8 cmp rax, rcx
0x14140e967 733c jae 0x14140e9a5
0x14140e969 48ffc0 inc rax
0x14140e96c 41b801000000 mov r8d, 1
0x14140e972 4929c8 sub r8, rcx
0x14140e975 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14140e980 0fb64c02ff movzx ecx, byte ptr [rdx + rax - 1]
0x14140e985 4883f920 cmp rcx, 0x20
0x14140e989 7730 ja 0x14140e9bb
0x14140e98b 490fa3cc bt r12, rcx
0x14140e98f 732a jae 0x14140e9bb
0x14140e991 48894728 mov qword ptr [rdi + 0x28], rax
0x14140e995 498d0c00 lea rcx, [r8 + rax]
0x14140e999 48ffc1 inc rcx
0x14140e99c 48ffc0 inc rax
0x14140e99f 4883f902 cmp rcx, 2
0x14140e9a3 75db jne 0x14140e980
0x14140e9a5 48c745e002000000 mov qword ptr [rbp - 0x20], 2
0x14140e9ad 488d55e0 lea rdx, [rbp - 0x20]
0x14140e9b1 4889f9 mov rcx, rdi
0x14140e9b4 e8671f4901 call 0x1428a0920
0x14140e9b9 eb1f jmp 0x14140e9da
0x14140e9bb 83f95d cmp ecx, 0x5d
0x14140e9be 0f840e010000 je 0x14140ead2
0x14140e9c4 488d4de0 lea rcx, [rbp - 0x20]
0x14140e9c8 4889fa mov rdx, rdi
0x14140e9cb e810ca3800 call 0x14179b3e0
0x14140e9d0 807de001 cmp byte ptr [rbp - 0x20], 1
0x14140e9d4 7523 jne 0x14140e9f9
0x14140e9d6 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x14140e9da b101 mov cl, 1
0x14140e9dc 48c745f800000000 mov qword ptr [rbp - 8], 0
0x14140e9e4 eb1f jmp 0x14140ea05
0x14140e9e6 4180fd38 cmp r13b, 0x38
0x14140e9ea 0f8406010000 je 0x14140eaf6
0x14140e9f0 b001 mov al, 1
0x14140e9f2 8945f8 mov dword ptr [rbp - 8], eax
0x14140e9f5 31c9 xor ecx, ecx
0x14140e9f7 eb4a jmp 0x14140ea43
0x14140e9f9 0fb655e1 movzx edx, byte ptr [rbp - 0x1f]
0x14140e9fd b001 mov al, 1
0x14140e9ff 488945f8 mov qword ptr [rbp - 8], rax
0x14140ea03 31c9 xor ecx, ecx
0x14140ea05 fe4730 inc byte ptr [rdi + 0x30]
0x14140ea08 884dd0 mov byte ptr [rbp - 0x30], cl
0x14140ea0b 8855d1 mov byte ptr [rbp - 0x2f], dl
0x14140ea0e 488945d8 mov qword ptr [rbp - 0x28], rax
0x14140ea12 4889f9 mov rcx, rdi
0x14140ea15 e8d69ac4fe call 0x1400584f0
0x14140ea1a eb3d jmp 0x14140ea59
0x14140ea1c 48c745e003000000 mov qword ptr [rbp - 0x20], 3
0x14140ea24 eb08 jmp 0x14140ea2e
0x14140ea26 48c745e005000000 mov qword ptr [rbp - 0x20], 5
0x14140ea2e 488d55e0 lea rdx, [rbp - 0x20]
0x14140ea32 4889f9 mov rcx, rdi
0x14140ea35 e8e61e4901 call 0x1428a0920
0x14140ea3a b101 mov cl, 1
0x14140ea3c c745f800000000 mov dword ptr [rbp - 8], 0
0x14140ea43 fe4730 inc byte ptr [rdi + 0x30]
0x14140ea46 884dd0 mov byte ptr [rbp - 0x30], cl
0x14140ea49 44886dd1 mov byte ptr [rbp - 0x2f], r13b
0x14140ea4d 488945d8 mov qword ptr [rbp - 0x28], rax
0x14140ea51 4889f9 mov rcx, rdi
0x14140ea54 e8f799c4fe call 0x140058450
0x14140ea59 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x14140ea5d 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x14140ea61 488945f0 mov qword ptr [rbp - 0x10], rax
0x14140ea65 807de000 cmp byte ptr [rbp - 0x20], 0
0x14140ea69 7510 jne 0x14140ea7b
0x14140ea6b 4885c0 test rax, rax
0x14140ea6e 753b jne 0x14140eaab
0x14140ea70 0fb645e1 movzx eax, byte ptr [rbp - 0x1f]
0x14140ea74 884601 mov byte ptr [rsi + 1], al
0x14140ea77 31c0 xor eax, eax
0x14140ea79 eb41 jmp 0x14140eabc
0x14140ea7b 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x14140ea7f 4885c0 test rax, rax
0x14140ea82 742a je 0x14140eaae
0x14140ea84 488d45f0 lea rax, [rbp - 0x10]
0x14140ea88 4889cb mov rbx, rcx
0x14140ea8b 4889c1 mov rcx, rax
0x14140ea8e e80d9e1dff call 0x1405e88a0
0x14140ea93 4889d9 mov rcx, rbx
0x14140ea96 eb16 jmp 0x14140eaae
0x14140ea98 4c8d05d1db8401 lea r8, [rip + 0x184dbd1]
0x14140ea9f 488d55e0 lea rdx, [rbp - 0x20]
0x14140eaa3 4889f9 mov rcx, rdi
0x14140eaa6 e855a14e01 call 0x1428f8c00
0x14140eaab 4889c1 mov rcx, rax
0x14140eaae 4889fa mov rdx, rdi
0x14140eab1 e84a374901 call 0x1428a2200
0x14140eab6 48894608 mov qword ptr [rsi + 8], rax
0x14140eaba b001 mov al, 1
0x14140eabc 8806 mov byte ptr [rsi], al
0x14140eabe 4889f0 mov rax, rsi
0x14140eac1 4883c458 add rsp, 0x58
0x14140eac5 5b pop rbx
0x14140eac6 5f pop rdi
0x14140eac7 5e pop rsi
0x14140eac8 415c pop r12
0x14140eaca 415d pop r13
0x14140eacc 415e pop r14
0x14140eace 415f pop r15
0x14140ead0 5d pop rbp
0x14140ead1 c3 ret 
0x14140ead2 488d15afe88401 lea rdx, [rip + 0x184e8af]
0x14140ead9 4c8d0510d48401 lea r8, [rip + 0x184d410]
0x14140eae0 48c745f800000000 mov qword ptr [rbp - 8], 0
0x14140eae8 31c9 xor ecx, ecx
0x14140eaea e831b24e01 call 0x1428f9d20
0x14140eaef b101 mov cl, 1
0x14140eaf1 e90fffffff jmp 0x14140ea05
0x14140eaf6 488d0d9be88401 lea rcx, [rip + 0x184e89b]
0x14140eafd ba02000000 mov edx, 2
0x14140eb02 e8b9b04e01 call 0x1428f9bc0
0x14140eb07 e92effffff jmp 0x14140ea3a
0x14140eb0c 488b45d8 mov rax, qword ptr [rbp - 0x28]
0x14140eb10 e925ffffff jmp 0x14140ea3a
0x14140eb15 410fb6c0 movzx eax, r8b
0x14140eb19 83f87d cmp eax, 0x7d
0x14140eb1c 750d jne 0x14140eb2b
0x14140eb1e 48c745e012000000 mov qword ptr [rbp - 0x20], 0x12
0x14140eb26 e903ffffff jmp 0x14140ea2e
0x14140eb2b 48c745e010000000 mov qword ptr [rbp - 0x20], 0x10
0x14140eb33 e9f6feffff jmp 0x14140ea2e
0x14140eb38 48c745e015000000 mov qword ptr [rbp - 0x20], 0x15
0x14140eb40 e939fcffff jmp 0x14140e77e
0x14140eb45 48c745e008000000 mov qword ptr [rbp - 0x20], 8
0x14140eb4d e9dcfeffff jmp 0x14140ea2e
0x14140eb52 488d0d3fe88401 lea rcx, [rip + 0x184e83f]
0x14140eb59 ba02000000 mov edx, 2
0x14140eb5e e84db24e01 call 0x1428f9db0
0x14140eb63 e9d2feffff jmp 0x14140ea3a
0x14140eb68 48c745e006000000 mov qword ptr [rbp - 0x20], 6
0x14140eb70 e9b9feffff jmp 0x14140ea2e
0x14140eb75 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14140eb80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14140eb85 55 push rbp
0x14140eb86 4157 push r15
0x14140eb88 4156 push r14
0x14140eb8a 4155 push r13
0x14140eb8c 4154 push r12
0x14140eb8e 56 push rsi
0x14140eb8f 57 push rdi
0x14140eb90 53 push rbx
0x14140eb91 4883ec28 sub rsp, 0x28
0x14140eb95 488d6a50 lea rbp, [rdx + 0x50]
0x14140eb99 488d4dd8 lea rcx, [rbp - 0x28]
0x14140eb9d 807df800 cmp byte ptr [rbp - 8], 0
0x14140eba1 7505 jne 0x14140eba8
0x14140eba3 e8f89c1dff call 0x1405e88a0
0x14140eba8 90 nop 
0x14140eba9 4883c428 add rsp, 0x28
0x14140ebad 5b pop rbx
0x14140ebae 5f pop rdi
0x14140ebaf 5e pop rsi
0x14140ebb0 415c pop r12
0x14140ebb2 415d pop r13
0x14140ebb4 415e pop r14
0x14140ebb6 415f pop r15
0x14140ebb8 5d pop rbp
0x14140ebb9 c3 ret 
0x14140ebba 660f1f440000 nop word ptr [rax + rax]
0x14140ebc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14140ebc5 55 push rbp
0x14140ebc6 4157 push r15
0x14140ebc8 4156 push r14
0x14140ebca 4155 push r13
0x14140ebcc 4154 push r12
0x14140ebce 56 push rsi
0x14140ebcf 57 push rdi
0x14140ebd0 53 push rbx
0x14140ebd1 4883ec28 sub rsp, 0x28
0x14140ebd5 488d6a50 lea rbp, [rdx + 0x50]
0x14140ebd9 488d4dd8 lea rcx, [rbp - 0x28]
0x14140ebdd 807df800 cmp byte ptr [rbp - 8], 0
0x14140ebe1 7505 jne 0x14140ebe8
0x14140ebe3 e8b89c1dff call 0x1405e88a0
0x14140ebe8 90 nop 
0x14140ebe9 4883c428 add rsp, 0x28
0x14140ebed 5b pop rbx
0x14140ebee 5f pop rdi
0x14140ebef 5e pop rsi
0x14140ebf0 415c pop r12
0x14140ebf2 415d pop r13
0x14140ebf4 415e pop r14
0x14140ebf6 415f pop r15
0x14140ebf8 5d pop rbp
0x14140ebf9 c3 ret 
