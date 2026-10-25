0x140a8b530 55 push rbp
0x140a8b531 4157 push r15
0x140a8b533 4156 push r14
0x140a8b535 4155 push r13
0x140a8b537 4154 push r12
0x140a8b539 56 push rsi
0x140a8b53a 57 push rdi
0x140a8b53b 53 push rbx
0x140a8b53c 4881ec880e0000 sub rsp, 0xe88
0x140a8b543 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a8b54b 440f29bdf00d0000 movaps xmmword ptr [rbp + 0xdf0], xmm15
0x140a8b553 440f29b5e00d0000 movaps xmmword ptr [rbp + 0xde0], xmm14
0x140a8b55b 440f29add00d0000 movaps xmmword ptr [rbp + 0xdd0], xmm13
0x140a8b563 440f29a5c00d0000 movaps xmmword ptr [rbp + 0xdc0], xmm12
0x140a8b56b 440f299db00d0000 movaps xmmword ptr [rbp + 0xdb0], xmm11
0x140a8b573 440f2995a00d0000 movaps xmmword ptr [rbp + 0xda0], xmm10
0x140a8b57b 440f298d900d0000 movaps xmmword ptr [rbp + 0xd90], xmm9
0x140a8b583 440f2985800d0000 movaps xmmword ptr [rbp + 0xd80], xmm8
0x140a8b58b 0f29bd700d0000 movaps xmmword ptr [rbp + 0xd70], xmm7
0x140a8b592 660f7fb5600d0000 movdqa xmmword ptr [rbp + 0xd60], xmm6
0x140a8b59a 48c785580d0000feffffff mov qword ptr [rbp + 0xd58], 0xfffffffffffffffe
0x140a8b5a5 4d89ce mov r14, r9
0x140a8b5a8 4c89c3 mov rbx, r8
0x140a8b5ab 4889d6 mov rsi, rdx
0x140a8b5ae 4889cf mov rdi, rcx
0x140a8b5b1 e84ab41800 call 0x140c16a00
0x140a8b5b6 84c0 test al, al
0x140a8b5b8 48899d100d0000 mov qword ptr [rbp + 0xd10], rbx
0x140a8b5bf 7460 je 0x140a8b621
0x140a8b5c1 4c8d2509d60402 lea r12, [rip + 0x204d609]
0x140a8b5c8 41bf5d000000 mov r15d, 0x5d
0x140a8b5ce ba5d000000 mov edx, 0x5d
0x140a8b5d3 4c89e1 mov rcx, r12
0x140a8b5d6 e8a5c6ffff call 0x140a87c80
0x140a8b5db 4883f801 cmp rax, 1
0x140a8b5df 0f85b2000000 jne 0x140a8b697
0x140a8b5e5 4989d1 mov r9, rdx
0x140a8b5e8 4885d2 test rdx, rdx
0x140a8b5eb 743d je 0x140a8b62a
0x140a8b5ed 4983f95d cmp r9, 0x5d
0x140a8b5f1 7335 jae 0x140a8b628
0x140a8b5f3 488d05d7d50402 lea rax, [rip + 0x204d5d7]
0x140a8b5fa 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x140a8b5ff 7f29 jg 0x140a8b62a
0x140a8b601 488d0540d70402 lea rax, [rip + 0x204d740]
0x140a8b608 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a8b60d 488d0dbdd50402 lea rcx, [rip + 0x204d5bd]
0x140a8b614 ba5d000000 mov edx, 0x5d
0x140a8b619 4531c0 xor r8d, r8d
0x140a8b61c e81fe3e401 call 0x1428d9940
0x140a8b621 31c0 xor eax, eax
0x140a8b623 e9d5010000 jmp 0x140a8b7fd
0x140a8b628 75d7 jne 0x140a8b601
0x140a8b62a 4c8d25a0d50402 lea r12, [rip + 0x204d5a0]
0x140a8b631 4c89e1 mov rcx, r12
0x140a8b634 4c89ca mov rdx, r9
0x140a8b637 e844c6ffff call 0x140a87c80
0x140a8b63c 4883f801 cmp rax, 1
0x140a8b640 7555 jne 0x140a8b697
0x140a8b642 4989d0 mov r8, rdx
0x140a8b645 4983c002 add r8, 2
0x140a8b649 7439 je 0x140a8b684
0x140a8b64b 4983f85d cmp r8, 0x5d
0x140a8b64f 7331 jae 0x140a8b682
0x140a8b651 488d0579d50402 lea rax, [rip + 0x204d579]
0x140a8b658 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x140a8b65d 7f25 jg 0x140a8b684
0x140a8b65f 488d05fad60402 lea rax, [rip + 0x204d6fa]
0x140a8b666 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a8b66b 488d0d5fd50402 lea rcx, [rip + 0x204d55f]
0x140a8b672 ba5d000000 mov edx, 0x5d
0x140a8b677 41b95d000000 mov r9d, 0x5d
0x140a8b67d e8bee2e401 call 0x1428d9940
0x140a8b682 75db jne 0x140a8b65f
0x140a8b684 41bf5b000000 mov r15d, 0x5b
0x140a8b68a 4929d7 sub r15, rdx
0x140a8b68d 4c8d253dd50402 lea r12, [rip + 0x204d53d]
0x140a8b694 4d01c4 add r12, r8
0x140a8b697 488d0516dc0402 lea rax, [rip + 0x204dc16]
0x140a8b69e 4c8d2dc3db0402 lea r13, [rip + 0x204dbc3]
0x140a8b6a5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8b6b0 0fbe48ff movsx ecx, byte ptr [rax - 1]
0x140a8b6b4 85c9 test ecx, ecx
0x140a8b6b6 7818 js 0x140a8b6d0
0x140a8b6b8 48ffc8 dec rax
0x140a8b6bb 83f95c cmp ecx, 0x5c
0x140a8b6be 7569 jne 0x140a8b729
0x140a8b6c0 eb7c jmp 0x140a8b73e
0x140a8b6c2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8b6d0 0fb650fe movzx edx, byte ptr [rax - 2]
0x140a8b6d4 80fac0 cmp dl, 0xc0
0x140a8b6d7 7d25 jge 0x140a8b6fe
0x140a8b6d9 440fb640fd movzx r8d, byte ptr [rax - 3]
0x140a8b6de 4180f8c0 cmp r8b, 0xc0
0x140a8b6e2 7d23 jge 0x140a8b707
0x140a8b6e4 440fb648fc movzx r9d, byte ptr [rax - 4]
0x140a8b6e9 4883c0fc add rax, -4
0x140a8b6ed 4183e107 and r9d, 7
0x140a8b6f1 41c1e106 shl r9d, 6
0x140a8b6f5 4183e03f and r8d, 0x3f
0x140a8b6f9 4509c8 or r8d, r9d
0x140a8b6fc eb11 jmp 0x140a8b70f
0x140a8b6fe 4883c0fe add rax, -2
0x140a8b702 83e21f and edx, 0x1f
0x140a8b705 eb12 jmp 0x140a8b719
0x140a8b707 4883c0fd add rax, -3
0x140a8b70b 4183e00f and r8d, 0xf
0x140a8b70f 41c1e006 shl r8d, 6
0x140a8b713 83e23f and edx, 0x3f
0x140a8b716 4409c2 or edx, r8d
0x140a8b719 c1e206 shl edx, 6
0x140a8b71c 80e13f and cl, 0x3f
0x140a8b71f 0fb6c9 movzx ecx, cl
0x140a8b722 09d1 or ecx, edx
0x140a8b724 83f95c cmp ecx, 0x5c
0x140a8b727 7415 je 0x140a8b73e
0x140a8b729 83f92f cmp ecx, 0x2f
0x140a8b72c 7410 je 0x140a8b73e
0x140a8b72e 4c39e8 cmp rax, r13
0x140a8b731 0f8579ffffff jne 0x140a8b6b0
0x140a8b737 bb4c000000 mov ebx, 0x4c
0x140a8b73c eb4f jmp 0x140a8b78d
0x140a8b73e 4c8d2d23db0402 lea r13, [rip + 0x204db23]
0x140a8b745 4c29e8 sub rax, r13
0x140a8b748 4989c0 mov r8, rax
0x140a8b74b 49ffc0 inc r8
0x140a8b74e 7432 je 0x140a8b782
0x140a8b750 4983f84c cmp r8, 0x4c
0x140a8b754 732a jae 0x140a8b780
0x140a8b756 43803c28bf cmp byte ptr [r8 + r13], 0xbf
0x140a8b75b 7f25 jg 0x140a8b782
0x140a8b75d 488d05c4d50402 lea rax, [rip + 0x204d5c4]
0x140a8b764 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a8b769 488d0df8da0402 lea rcx, [rip + 0x204daf8]
0x140a8b770 ba4c000000 mov edx, 0x4c
0x140a8b775 41b94c000000 mov r9d, 0x4c
0x140a8b77b e8c0e1e401 call 0x1428d9940
0x140a8b780 75db jne 0x140a8b75d
0x140a8b782 bb4b000000 mov ebx, 0x4b
0x140a8b787 4829c3 sub rbx, rax
0x140a8b78a 4d01c5 add r13, r8
0x140a8b78d e89ec91800 call 0x140c18130
0x140a8b792 4889c1 mov rcx, rax
0x140a8b795 488b00 mov rax, qword ptr [rax]
0x140a8b798 4883f801 cmp rax, 1
0x140a8b79c 0f85f5250000 jne 0x140a8dd97
0x140a8b7a2 4883c108 add rcx, 8
0x140a8b7a6 48833900 cmp qword ptr [rcx], 0
0x140a8b7aa 0f85e7270000 jne 0x140a8df97
0x140a8b7b0 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140a8b7b7 48898d480d0000 mov qword ptr [rbp + 0xd48], rcx
0x140a8b7be 4883c108 add rcx, 8
0x140a8b7c2 48895c2420 mov qword ptr [rsp + 0x20], rbx
0x140a8b7c7 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x140a8b7d0 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x140a8b7d9 4c89e2 mov rdx, r12
0x140a8b7dc 4d89f8 mov r8, r15
0x140a8b7df 4d89e9 mov r9, r13
0x140a8b7e2 e859b51800 call 0x140c16d40
0x140a8b7e7 488b8d480d0000 mov rcx, qword ptr [rbp + 0xd48]
0x140a8b7ee 48ff01 inc qword ptr [rcx]
0x140a8b7f1 488985080c0000 mov qword ptr [rbp + 0xc08], rax
0x140a8b7f8 b801000000 mov eax, 1
0x140a8b7fd 4c89b5000d0000 mov qword ptr [rbp + 0xd00], r14
0x140a8b804 488b8d900e0000 mov rcx, qword ptr [rbp + 0xe90]
0x140a8b80b 4c8bbd780e0000 mov r15, qword ptr [rbp + 0xe78]
0x140a8b812 488985000c0000 mov qword ptr [rbp + 0xc00], rax
0x140a8b819 8b461c mov eax, dword ptr [rsi + 0x1c]
0x140a8b81c 488b5610 mov rdx, qword ptr [rsi + 0x10]
0x140a8b820 8902 mov dword ptr [rdx], eax
0x140a8b822 488b1e mov rbx, qword ptr [rsi]
0x140a8b825 8b471c mov eax, dword ptr [rdi + 0x1c]
0x140a8b828 488b5710 mov rdx, qword ptr [rdi + 0x10]
0x140a8b82c 8902 mov dword ptr [rdx], eax
0x140a8b82e 488b07 mov rax, qword ptr [rdi]
0x140a8b831 488985480d0000 mov qword ptr [rbp + 0xd48], rax
0x140a8b838 488b01 mov rax, qword ptr [rcx]
0x140a8b83b 488b4908 mov rcx, qword ptr [rcx + 8]
0x140a8b83f 488b10 mov rdx, qword ptr [rax]
0x140a8b842 4531c0 xor r8d, r8d
0x140a8b845 4989d1 mov r9, rdx
0x140a8b848 4c2b4918 sub r9, qword ptr [rcx + 0x18]
0x140a8b84c 4d0f42c8 cmovb r9, r8
0x140a8b850 482b5138 sub rdx, qword ptr [rcx + 0x38]
0x140a8b854 490f42d0 cmovb rdx, r8
0x140a8b858 4c8b5130 mov r10, qword ptr [rcx + 0x30]
0x140a8b85c 4929d2 sub r10, rdx
0x140a8b85f 4d0f42d0 cmovb r10, r8
0x140a8b863 0f97c2 seta dl
0x140a8b866 4c8b5910 mov r11, qword ptr [rcx + 0x10]
0x140a8b86a 4d29cb sub r11, r9
0x140a8b86d 4d0f42d8 cmovb r11, r8
0x140a8b871 488b4940 mov rcx, qword ptr [rcx + 0x40]
0x140a8b875 410f97c0 seta r8b
0x140a8b879 4d01d3 add r11, r10
0x140a8b87c 4989c9 mov r9, rcx
0x140a8b87f 4d29d9 sub r9, r11
0x140a8b882 4c8908 mov qword ptr [rax], r9
0x140a8b885 4108d0 or r8b, dl
0x140a8b888 48899df80c0000 mov qword ptr [rbp + 0xcf8], rbx
0x140a8b88f 0f8574050000 jne 0x140a8be09
0x140a8b895 418b471c mov eax, dword ptr [r15 + 0x1c]
0x140a8b899 8985e00c0000 mov dword ptr [rbp + 0xce0], eax
0x140a8b89f 4d8b37 mov r14, qword ptr [r15]
0x140a8b8a2 498b4710 mov rax, qword ptr [r15 + 0x10]
0x140a8b8a6 488985e80c0000 mov qword ptr [rbp + 0xce8], rax
0x140a8b8ad 488b85800e0000 mov rax, qword ptr [rbp + 0xe80]
0x140a8b8b4 4c8b28 mov r13, qword ptr [rax]
0x140a8b8b7 488b4808 mov rcx, qword ptr [rax + 8]
0x140a8b8bb 4d8b4500 mov r8, qword ptr [r13]
0x140a8b8bf 31d2 xor edx, edx
0x140a8b8c1 4c89c0 mov rax, r8
0x140a8b8c4 482b4118 sub rax, qword ptr [rcx + 0x18]
0x140a8b8c8 480f42c2 cmovb rax, rdx
0x140a8b8cc 4c2b4138 sub r8, qword ptr [rcx + 0x38]
0x140a8b8d0 4c0f42c2 cmovb r8, rdx
0x140a8b8d4 4889c3 mov rbx, rax
0x140a8b8d7 48c1e304 shl rbx, 4
0x140a8b8db 48035908 add rbx, qword ptr [rcx + 8]
0x140a8b8df 4c8b5110 mov r10, qword ptr [rcx + 0x10]
0x140a8b8e3 4c8b5930 mov r11, qword ptr [rcx + 0x30]
0x140a8b8e7 4929c2 sub r10, rax
0x140a8b8ea 4c0f42d2 cmovb r10, rdx
0x140a8b8ee 41b908000000 mov r9d, 8
0x140a8b8f4 490f42d9 cmovb rbx, r9
0x140a8b8f8 4c89c0 mov rax, r8
0x140a8b8fb 48c1e004 shl rax, 4
0x140a8b8ff 48034128 add rax, qword ptr [rcx + 0x28]
0x140a8b903 4d29c3 sub r11, r8
0x140a8b906 4c0f42da cmovb r11, rdx
0x140a8b90a 490f42c1 cmovb rax, r9
0x140a8b90e 488b4940 mov rcx, qword ptr [rcx + 0x40]
0x140a8b912 4c29d9 sub rcx, r11
0x140a8b915 4c29d1 sub rcx, r10
0x140a8b918 49894d00 mov qword ptr [r13], rcx
0x140a8b91c 49c1e204 shl r10, 4
0x140a8b920 4901da add r10, rbx
0x140a8b923 4c8995300d0000 mov qword ptr [rbp + 0xd30], r10
0x140a8b92a 49c1e304 shl r11, 4
0x140a8b92e 4901c3 add r11, rax
0x140a8b931 4c899df00c0000 mov qword ptr [rbp + 0xcf0], r11
0x140a8b938 488b8d480d0000 mov rcx, qword ptr [rbp + 0xd48]
0x140a8b93f 4883c120 add rcx, 0x20
0x140a8b943 48898dc00c0000 mov qword ptr [rbp + 0xcc0], rcx
0x140a8b94a 660f76f6 pcmpeqd xmm6, xmm6
0x140a8b94e 498d4e20 lea rcx, [r14 + 0x20]
0x140a8b952 48898d200d0000 mov qword ptr [rbp + 0xd20], rcx
0x140a8b959 eb2a jmp 0x140a8b985
0x140a8b95b 0f1f440000 nop dword ptr [rax + rax]
0x140a8b960 498b4628 mov rax, qword ptr [r14 + 0x28]
0x140a8b964 4889f1 mov rcx, rsi
0x140a8b967 48c1e104 shl rcx, 4
0x140a8b96b 4c893c08 mov qword ptr [rax + rcx], r15
0x140a8b96f 48895c0808 mov qword ptr [rax + rcx + 8], rbx
0x140a8b974 48ffc6 inc rsi
0x140a8b977 49897630 mov qword ptr [r14 + 0x30], rsi
0x140a8b97b 49ff4640 inc qword ptr [r14 + 0x40]
0x140a8b97f 4889f8 mov rax, rdi
0x140a8b982 4c89e3 mov rbx, r12
0x140a8b985 4885db test rbx, rbx
0x140a8b988 7416 je 0x140a8b9a0
0x140a8b98a 483b9d300d0000 cmp rbx, qword ptr [rbp + 0xd30]
0x140a8b991 740d je 0x140a8b9a0
0x140a8b993 4c8d6310 lea r12, [rbx + 0x10]
0x140a8b997 4889c7 mov rdi, rax
0x140a8b99a eb24 jmp 0x140a8b9c0
0x140a8b99c 0f1f4000 nop dword ptr [rax]
0x140a8b9a0 4885c0 test rax, rax
0x140a8b9a3 0f8424010000 je 0x140a8bacd
0x140a8b9a9 483b85f00c0000 cmp rax, qword ptr [rbp + 0xcf0]
0x140a8b9b0 0f8417010000 je 0x140a8bacd
0x140a8b9b6 488d7810 lea rdi, [rax + 0x10]
0x140a8b9ba 4531e4 xor r12d, r12d
0x140a8b9bd 4889c3 mov rbx, rax
0x140a8b9c0 49ff4500 inc qword ptr [r13]
0x140a8b9c4 488b7308 mov rsi, qword ptr [rbx + 8]
0x140a8b9c8 4889b540080000 mov qword ptr [rbp + 0x840], rsi
0x140a8b9cf 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8b9d6 4883781800 cmp qword ptr [rax + 0x18], 0
0x140a8b9db 74a2 je 0x140a8b97f
0x140a8b9dd 488b8dc00c0000 mov rcx, qword ptr [rbp + 0xcc0]
0x140a8b9e4 488d9540080000 lea rdx, [rbp + 0x840]
0x140a8b9eb e88053b2ff call 0x1405b0d70
0x140a8b9f0 488b95480d0000 mov rdx, qword ptr [rbp + 0xd48]
0x140a8b9f7 488b0a mov rcx, qword ptr [rdx]
0x140a8b9fa 488b5208 mov rdx, qword ptr [rdx + 8]
0x140a8b9fe 4989d0 mov r8, rdx
0x140a8ba01 4921c0 and r8, rax
0x140a8ba04 48c1e839 shr rax, 0x39
0x140a8ba08 660f6ec0 movd xmm0, eax
0x140a8ba0c 660f60c0 punpcklbw xmm0, xmm0
0x140a8ba10 f20f70c000 pshuflw xmm0, xmm0, 0
0x140a8ba15 660f70c000 pshufd xmm0, xmm0, 0
0x140a8ba1a 488d4188 lea rax, [rcx - 0x78]
0x140a8ba1e 4531c9 xor r9d, r9d
0x140a8ba21 f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
0x140a8ba27 660f6fd1 movdqa xmm2, xmm1
0x140a8ba2b 660f74d0 pcmpeqb xmm2, xmm0
0x140a8ba2f 66440fd7d2 pmovmskb r10d, xmm2
0x140a8ba34 4585d2 test r10d, r10d
0x140a8ba37 7427 je 0x140a8ba60
0x140a8ba39 f3450fbcda tzcnt r11d, r10d
0x140a8ba3e 4d01c3 add r11, r8
0x140a8ba41 4921d3 and r11, rdx
0x140a8ba44 49f7db neg r11
0x140a8ba47 4d6bdb78 imul r11, r11, 0x78
0x140a8ba4b 4a393418 cmp qword ptr [rax + r11], rsi
0x140a8ba4f 743f je 0x140a8ba90
0x140a8ba51 458d5aff lea r11d, [r10 - 1]
0x140a8ba55 664521d3 and r11w, r10w
0x140a8ba59 4589da mov r10d, r11d
0x140a8ba5c 75db jne 0x140a8ba39
0x140a8ba5e 6690 nop 
0x140a8ba60 660f74ce pcmpeqb xmm1, xmm6
0x140a8ba64 66440fd7d1 pmovmskb r10d, xmm1
0x140a8ba69 4585d2 test r10d, r10d
0x140a8ba6c 0f850dffffff jne 0x140a8b97f
0x140a8ba72 4d01c8 add r8, r9
0x140a8ba75 4983c010 add r8, 0x10
0x140a8ba79 4983c110 add r9, 0x10
0x140a8ba7d 4921d0 and r8, rdx
0x140a8ba80 eb9f jmp 0x140a8ba21
0x140a8ba82 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8ba90 488b5b08 mov rbx, qword ptr [rbx + 8]
0x140a8ba94 8b85e00c0000 mov eax, dword ptr [rbp + 0xce0]
0x140a8ba9a 488b8de80c0000 mov rcx, qword ptr [rbp + 0xce8]
0x140a8baa1 8901 mov dword ptr [rcx], eax
0x140a8baa3 498b7630 mov rsi, qword ptr [r14 + 0x30]
0x140a8baa7 4d8b7e40 mov r15, qword ptr [r14 + 0x40]
0x140a8baab 493b7620 cmp rsi, qword ptr [r14 + 0x20]
0x140a8baaf 0f85abfeffff jne 0x140a8b960
0x140a8bab5 488b8d200d0000 mov rcx, qword ptr [rbp + 0xd20]
0x140a8babc 488d1585e61a02 lea rdx, [rip + 0x21ae685]
0x140a8bac3 e8686badff call 0x140562630
0x140a8bac8 e993feffff jmp 0x140a8b960
0x140a8bacd 488b85880e0000 mov rax, qword ptr [rbp + 0xe88]
0x140a8bad4 4c8b28 mov r13, qword ptr [rax]
0x140a8bad7 488b4808 mov rcx, qword ptr [rax + 8]
0x140a8badb 4d8b4500 mov r8, qword ptr [r13]
0x140a8badf 31d2 xor edx, edx
0x140a8bae1 4c89c0 mov rax, r8
0x140a8bae4 482b4118 sub rax, qword ptr [rcx + 0x18]
0x140a8bae8 480f42c2 cmovb rax, rdx
0x140a8baec 4c2b4138 sub r8, qword ptr [rcx + 0x38]
0x140a8baf0 4c0f42c2 cmovb r8, rdx
0x140a8baf4 4889c3 mov rbx, rax
0x140a8baf7 48c1e304 shl rbx, 4
0x140a8bafb 48035908 add rbx, qword ptr [rcx + 8]
0x140a8baff 4c8b5110 mov r10, qword ptr [rcx + 0x10]
0x140a8bb03 4c8b5930 mov r11, qword ptr [rcx + 0x30]
0x140a8bb07 4929c2 sub r10, rax
0x140a8bb0a 4c0f42d2 cmovb r10, rdx
0x140a8bb0e 41b908000000 mov r9d, 8
0x140a8bb14 490f42d9 cmovb rbx, r9
0x140a8bb18 4c89c0 mov rax, r8
0x140a8bb1b 48c1e004 shl rax, 4
0x140a8bb1f 48034128 add rax, qword ptr [rcx + 0x28]
0x140a8bb23 4d29c3 sub r11, r8
0x140a8bb26 4c0f42da cmovb r11, rdx
0x140a8bb2a 490f42c1 cmovb rax, r9
0x140a8bb2e 488b4940 mov rcx, qword ptr [rcx + 0x40]
0x140a8bb32 4c29d9 sub rcx, r11
0x140a8bb35 4c29d1 sub rcx, r10
0x140a8bb38 49894d00 mov qword ptr [r13], rcx
0x140a8bb3c 49c1e204 shl r10, 4
0x140a8bb40 4901da add r10, rbx
0x140a8bb43 4c8995300d0000 mov qword ptr [rbp + 0xd30], r10
0x140a8bb4a 49c1e304 shl r11, 4
0x140a8bb4e 4901c3 add r11, rax
0x140a8bb51 4c899df00c0000 mov qword ptr [rbp + 0xcf0], r11
0x140a8bb58 660f76f6 pcmpeqd xmm6, xmm6
0x140a8bb5c eb27 jmp 0x140a8bb85
0x140a8bb5e 6690 nop 
0x140a8bb60 498b4628 mov rax, qword ptr [r14 + 0x28]
0x140a8bb64 4889d9 mov rcx, rbx
0x140a8bb67 48c1e104 shl rcx, 4
0x140a8bb6b 48893408 mov qword ptr [rax + rcx], rsi
0x140a8bb6f 4c897c0808 mov qword ptr [rax + rcx + 8], r15
0x140a8bb74 48ffc3 inc rbx
0x140a8bb77 49895e30 mov qword ptr [r14 + 0x30], rbx
0x140a8bb7b 49ff4640 inc qword ptr [r14 + 0x40]
0x140a8bb7f 4c89e3 mov rbx, r12
0x140a8bb82 4889f8 mov rax, rdi
0x140a8bb85 4885db test rbx, rbx
0x140a8bb88 7416 je 0x140a8bba0
0x140a8bb8a 483b9d300d0000 cmp rbx, qword ptr [rbp + 0xd30]
0x140a8bb91 740d je 0x140a8bba0
0x140a8bb93 4c8d6310 lea r12, [rbx + 0x10]
0x140a8bb97 4889c7 mov rdi, rax
0x140a8bb9a eb24 jmp 0x140a8bbc0
0x140a8bb9c 0f1f4000 nop dword ptr [rax]
0x140a8bba0 4885c0 test rax, rax
0x140a8bba3 0f8424010000 je 0x140a8bccd
0x140a8bba9 483b85f00c0000 cmp rax, qword ptr [rbp + 0xcf0]
0x140a8bbb0 0f8417010000 je 0x140a8bccd
0x140a8bbb6 488d7810 lea rdi, [rax + 0x10]
0x140a8bbba 4531e4 xor r12d, r12d
0x140a8bbbd 4889c3 mov rbx, rax
0x140a8bbc0 49ff4500 inc qword ptr [r13]
0x140a8bbc4 4c8b7b08 mov r15, qword ptr [rbx + 8]
0x140a8bbc8 4c89bd40080000 mov qword ptr [rbp + 0x840], r15
0x140a8bbcf 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8bbd6 4883781800 cmp qword ptr [rax + 0x18], 0
0x140a8bbdb 74a2 je 0x140a8bb7f
0x140a8bbdd 488b8dc00c0000 mov rcx, qword ptr [rbp + 0xcc0]
0x140a8bbe4 488d9540080000 lea rdx, [rbp + 0x840]
0x140a8bbeb e88051b2ff call 0x1405b0d70
0x140a8bbf0 488b95480d0000 mov rdx, qword ptr [rbp + 0xd48]
0x140a8bbf7 488b0a mov rcx, qword ptr [rdx]
0x140a8bbfa 488b5208 mov rdx, qword ptr [rdx + 8]
0x140a8bbfe 4989d0 mov r8, rdx
0x140a8bc01 4921c0 and r8, rax
0x140a8bc04 48c1e839 shr rax, 0x39
0x140a8bc08 660f6ec0 movd xmm0, eax
0x140a8bc0c 660f60c0 punpcklbw xmm0, xmm0
0x140a8bc10 f20f70c000 pshuflw xmm0, xmm0, 0
0x140a8bc15 660f70c000 pshufd xmm0, xmm0, 0
0x140a8bc1a 488d4188 lea rax, [rcx - 0x78]
0x140a8bc1e 4531c9 xor r9d, r9d
0x140a8bc21 f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
0x140a8bc27 660f6fd1 movdqa xmm2, xmm1
0x140a8bc2b 660f74d0 pcmpeqb xmm2, xmm0
0x140a8bc2f 66440fd7d2 pmovmskb r10d, xmm2
0x140a8bc34 4585d2 test r10d, r10d
0x140a8bc37 7427 je 0x140a8bc60
0x140a8bc39 f3450fbcda tzcnt r11d, r10d
0x140a8bc3e 4d01c3 add r11, r8
0x140a8bc41 4921d3 and r11, rdx
0x140a8bc44 49f7db neg r11
0x140a8bc47 4d6bdb78 imul r11, r11, 0x78
0x140a8bc4b 4e393c18 cmp qword ptr [rax + r11], r15
0x140a8bc4f 743f je 0x140a8bc90
0x140a8bc51 458d5aff lea r11d, [r10 - 1]
0x140a8bc55 664521d3 and r11w, r10w
0x140a8bc59 4589da mov r10d, r11d
0x140a8bc5c 75db jne 0x140a8bc39
0x140a8bc5e 6690 nop 
0x140a8bc60 660f74ce pcmpeqb xmm1, xmm6
0x140a8bc64 66440fd7d1 pmovmskb r10d, xmm1
0x140a8bc69 4585d2 test r10d, r10d
0x140a8bc6c 0f850dffffff jne 0x140a8bb7f
0x140a8bc72 4d01c8 add r8, r9
0x140a8bc75 4983c010 add r8, 0x10
0x140a8bc79 4983c110 add r9, 0x10
0x140a8bc7d 4921d0 and r8, rdx
0x140a8bc80 eb9f jmp 0x140a8bc21
0x140a8bc82 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8bc90 4c8b7b08 mov r15, qword ptr [rbx + 8]
0x140a8bc94 8b85e00c0000 mov eax, dword ptr [rbp + 0xce0]
0x140a8bc9a 488b8de80c0000 mov rcx, qword ptr [rbp + 0xce8]
0x140a8bca1 8901 mov dword ptr [rcx], eax
0x140a8bca3 498b5e30 mov rbx, qword ptr [r14 + 0x30]
0x140a8bca7 498b7640 mov rsi, qword ptr [r14 + 0x40]
0x140a8bcab 493b5e20 cmp rbx, qword ptr [r14 + 0x20]
0x140a8bcaf 0f85abfeffff jne 0x140a8bb60
0x140a8bcb5 488b8d200d0000 mov rcx, qword ptr [rbp + 0xd20]
0x140a8bcbc 488d1585e41a02 lea rdx, [rip + 0x21ae485]
0x140a8bcc3 e86869adff call 0x140562630
0x140a8bcc8 e993feffff jmp 0x140a8bb60
0x140a8bccd 488b85100d0000 mov rax, qword ptr [rbp + 0xd10]
0x140a8bcd4 488b30 mov rsi, qword ptr [rax]
0x140a8bcd7 488b85000d0000 mov rax, qword ptr [rbp + 0xd00]
0x140a8bcde 488b00 mov rax, qword ptr [rax]
0x140a8bce1 488985e00c0000 mov qword ptr [rbp + 0xce0], rax
0x140a8bce8 488b95980e0000 mov rdx, qword ptr [rbp + 0xe98]
0x140a8bcef 8b421c mov eax, dword ptr [rdx + 0x1c]
0x140a8bcf2 488b4a10 mov rcx, qword ptr [rdx + 0x10]
0x140a8bcf6 8901 mov dword ptr [rcx], eax
0x140a8bcf8 4c8b32 mov r14, qword ptr [rdx]
0x140a8bcfb 4889b540080000 mov qword ptr [rbp + 0x840], rsi
0x140a8bd02 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8bd09 48898548080000 mov qword ptr [rbp + 0x848], rax
0x140a8bd10 488b85f80c0000 mov rax, qword ptr [rbp + 0xcf8]
0x140a8bd17 48898550080000 mov qword ptr [rbp + 0x850], rax
0x140a8bd1e 4c8dbd40080000 lea r15, [rbp + 0x840]
0x140a8bd25 4c89f1 mov rcx, r14
0x140a8bd28 4c89fa mov rdx, r15
0x140a8bd2b e8f0cd9c00 call 0x141458b20
0x140a8bd30 31c9 xor ecx, ecx
0x140a8bd32 e8e9dd64ff call 0x1400d9b20
0x140a8bd37 4885c0 test rax, rax
0x140a8bd3a 0f848d230000 je 0x140a8e0cd
0x140a8bd40 0f1000 movups xmm0, xmmword ptr [rax]
0x140a8bd43 48ff00 inc qword ptr [rax]
0x140a8bd46 0f298560080000 movaps xmmword ptr [rbp + 0x860], xmm0
0x140a8bd4d 0f1005ac011e02 movups xmm0, xmmword ptr [rip + 0x21e01ac]
0x140a8bd54 0f298540080000 movaps xmmword ptr [rbp + 0x840], xmm0
0x140a8bd5b f30f6f05ad011e02 movdqu xmm0, xmmword ptr [rip + 0x21e01ad]
0x140a8bd63 660f7f8550080000 movdqa xmmword ptr [rbp + 0x850], xmm0
0x140a8bd6b 4983c608 add r14, 8
0x140a8bd6f 4889b560020000 mov qword ptr [rbp + 0x260], rsi
0x140a8bd76 488bb5480d0000 mov rsi, qword ptr [rbp + 0xd48]
0x140a8bd7d 4889b568020000 mov qword ptr [rbp + 0x268], rsi
0x140a8bd84 488b9df80c0000 mov rbx, qword ptr [rbp + 0xcf8]
0x140a8bd8b 48899d70020000 mov qword ptr [rbp + 0x270], rbx
0x140a8bd92 4c89bd78020000 mov qword ptr [rbp + 0x278], r15
0x140a8bd99 c685550d000001 mov byte ptr [rbp + 0xd55], 1
0x140a8bda0 488d9560020000 lea rdx, [rbp + 0x260]
0x140a8bda7 4c89f1 mov rcx, r14
0x140a8bdaa e8a1cc9c00 call 0x141458a50
0x140a8bdaf 488bbd40080000 mov rdi, qword ptr [rbp + 0x840]
0x140a8bdb6 488b8d48080000 mov rcx, qword ptr [rbp + 0x848]
0x140a8bdbd 660f6f07 movdqa xmm0, xmmword ptr [rdi]
0x140a8bdc1 4885c9 test rcx, rcx
0x140a8bdc4 48898d300d0000 mov qword ptr [rbp + 0xd30], rcx
0x140a8bdcb 0f844d010000 je 0x140a8bf1e
0x140a8bdd1 488d04cd17000000 lea rax, [rcx*8 + 0x17]
0x140a8bdd9 4883e0f0 and rax, 0xfffffffffffffff0
0x140a8bddd 4801c1 add rcx, rax
0x140a8bde0 4883c111 add rcx, 0x11
0x140a8bde4 48898d200d0000 mov qword ptr [rbp + 0xd20], rcx
0x140a8bdeb 4889f9 mov rcx, rdi
0x140a8bdee 4829c1 sub rcx, rax
0x140a8bdf1 48898d000d0000 mov qword ptr [rbp + 0xd00], rcx
0x140a8bdf8 b810000000 mov eax, 0x10
0x140a8bdfd 488985100d0000 mov qword ptr [rbp + 0xd10], rax
0x140a8be04 e920010000 jmp 0x140a8bf29
0x140a8be09 488908 mov qword ptr [rax], rcx
0x140a8be0c 488b33 mov rsi, qword ptr [rbx]
0x140a8be0f 488b5318 mov rdx, qword ptr [rbx + 0x18]
0x140a8be13 660f6f06 movdqa xmm0, xmmword ptr [rsi]
0x140a8be17 660fd7c0 pmovmskb eax, xmm0
0x140a8be1b f7d0 not eax
0x140a8be1d 488d5e10 lea rbx, [rsi + 0x10]
0x140a8be21 418b4f1c mov ecx, dword ptr [r15 + 0x1c]
0x140a8be25 898de00c0000 mov dword ptr [rbp + 0xce0], ecx
0x140a8be2b 4d8b37 mov r14, qword ptr [r15]
0x140a8be2e 498b4f10 mov rcx, qword ptr [r15 + 0x10]
0x140a8be32 48898de80c0000 mov qword ptr [rbp + 0xce8], rcx
0x140a8be39 498d4e20 lea rcx, [r14 + 0x20]
0x140a8be3d 48898d300d0000 mov qword ptr [rbp + 0xd30], rcx
0x140a8be44 eb33 jmp 0x140a8be79
0x140a8be46 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8be50 498b4628 mov rax, qword ptr [r14 + 0x28]
0x140a8be54 4c89f9 mov rcx, r15
0x140a8be57 48c1e104 shl rcx, 4
0x140a8be5b 48893c08 mov qword ptr [rax + rcx], rdi
0x140a8be5f 4c89640808 mov qword ptr [rax + rcx + 8], r12
0x140a8be64 49ffc7 inc r15
0x140a8be67 4d897e30 mov qword ptr [r14 + 0x30], r15
0x140a8be6b 49ff4640 inc qword ptr [r14 + 0x40]
0x140a8be6f 4489e8 mov eax, r13d
0x140a8be72 488b95c00c0000 mov rdx, qword ptr [rbp + 0xcc0]
0x140a8be79 4885d2 test rdx, rdx
0x140a8be7c 0f842bfaffff je 0x140a8b8ad
0x140a8be82 6685c0 test ax, ax
0x140a8be85 7409 je 0x140a8be90
0x140a8be87 448d68ff lea r13d, [rax - 1]
0x140a8be8b 4121c5 and r13d, eax
0x140a8be8e eb29 jmp 0x140a8beb9
0x140a8be90 660f6f03 movdqa xmm0, xmmword ptr [rbx]
0x140a8be94 660fd7c0 pmovmskb eax, xmm0
0x140a8be98 4881c680d5ffff add rsi, -0x2a80
0x140a8be9f 4883c310 add rbx, 0x10
0x140a8bea3 3dffff0000 cmp eax, 0xffff
0x140a8bea8 74e6 je 0x140a8be90
0x140a8beaa b9feffffff mov ecx, 0xfffffffe
0x140a8beaf 29c1 sub ecx, eax
0x140a8beb1 f7d0 not eax
0x140a8beb3 4189c5 mov r13d, eax
0x140a8beb6 4121cd and r13d, ecx
0x140a8beb9 48ffca dec rdx
0x140a8bebc f30fbcc0 tzcnt eax, eax
0x140a8bec0 48f7d8 neg rax
0x140a8bec3 4869c8a8020000 imul rcx, rax, 0x2a8
0x140a8beca 807c0ef901 cmp byte ptr [rsi + rcx - 7], 1
0x140a8becf 4489e8 mov eax, r13d
0x140a8bed2 76a5 jbe 0x140a8be79
0x140a8bed4 488995c00c0000 mov qword ptr [rbp + 0xcc0], rdx
0x140a8bedb 4801f1 add rcx, rsi
0x140a8bede 4c8ba158fdffff mov r12, qword ptr [rcx - 0x2a8]
0x140a8bee5 8b85e00c0000 mov eax, dword ptr [rbp + 0xce0]
0x140a8beeb 488b8de80c0000 mov rcx, qword ptr [rbp + 0xce8]
0x140a8bef2 8901 mov dword ptr [rcx], eax
0x140a8bef4 4d8b7e30 mov r15, qword ptr [r14 + 0x30]
0x140a8bef8 498b7e40 mov rdi, qword ptr [r14 + 0x40]
0x140a8befc 4d3b7e20 cmp r15, qword ptr [r14 + 0x20]
0x140a8bf00 0f854affffff jne 0x140a8be50
0x140a8bf06 488b8d300d0000 mov rcx, qword ptr [rbp + 0xd30]
0x140a8bf0d 488d1534e21a02 lea rdx, [rip + 0x21ae234]
0x140a8bf14 e81767adff call 0x140562630
0x140a8bf19 e932ffffff jmp 0x140a8be50
0x140a8bf1e 48c785100d000000000000 mov qword ptr [rbp + 0xd10], 0
0x140a8bf29 4c8bb558080000 mov r14, qword ptr [rbp + 0x858]
0x140a8bf30 4c8d7f10 lea r15, [rdi + 0x10]
0x140a8bf34 660fd7c0 pmovmskb eax, xmm0
0x140a8bf38 f7d0 not eax
0x140a8bf3a 488d4e60 lea rcx, [rsi + 0x60]
0x140a8bf3e 48898df00c0000 mov qword ptr [rbp + 0xcf0], rcx
0x140a8bf45 488d4b20 lea rcx, [rbx + 0x20]
0x140a8bf49 48898de80c0000 mov qword ptr [rbp + 0xce8], rcx
0x140a8bf50 660f76f6 pcmpeqd xmm6, xmm6
0x140a8bf54 0f57ff xorps xmm7, xmm7
0x140a8bf57 e9f4010000 jmp 0x140a8c150
0x140a8bf5c 0f1f4000 nop dword ptr [rax]
0x140a8bf60 f30f6f0c16 movdqu xmm1, xmmword ptr [rsi + rdx]
0x140a8bf65 660f6fd1 movdqa xmm2, xmm1
0x140a8bf69 660f74d0 pcmpeqb xmm2, xmm0
0x140a8bf6d 66440fd7ca pmovmskb r9d, xmm2
0x140a8bf72 4585c9 test r9d, r9d
0x140a8bf75 7429 je 0x140a8bfa0
0x140a8bf77 f3450fbcd1 tzcnt r10d, r9d
0x140a8bf7c 4901d2 add r10, rdx
0x140a8bf7f 4921ca and r10, rcx
0x140a8bf82 49f7da neg r10
0x140a8bf85 4d69e2a8020000 imul r12, r10, 0x2a8
0x140a8bf8c 4e392c20 cmp qword ptr [rax + r12], r13
0x140a8bf90 743e je 0x140a8bfd0
0x140a8bf92 458d51ff lea r10d, [r9 - 1]
0x140a8bf96 664521ca and r10w, r9w
0x140a8bf9a 4589d1 mov r9d, r10d
0x140a8bf9d 75d8 jne 0x140a8bf77
0x140a8bf9f 90 nop 
0x140a8bfa0 660f74ce pcmpeqb xmm1, xmm6
0x140a8bfa4 66440fd7c9 pmovmskb r9d, xmm1
0x140a8bfa9 4585c9 test r9d, r9d
0x140a8bfac 0f855e020000 jne 0x140a8c210
0x140a8bfb2 4c01c2 add rdx, r8
0x140a8bfb5 4883c210 add rdx, 0x10
0x140a8bfb9 4983c010 add r8, 0x10
0x140a8bfbd 4821ca and rdx, rcx
0x140a8bfc0 eb9e jmp 0x140a8bf60
0x140a8bfc2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8bfd0 4a83bc2670fdffff02 cmp qword ptr [rsi + r12 - 0x290], 2
0x140a8bfd9 0f827a1f0000 jb 0x140a8df59
0x140a8bfdf f3420f10842690fdffff movss xmm0, dword ptr [rsi + r12 - 0x270]
0x140a8bfe9 0f2ec7 ucomiss xmm0, xmm7
0x140a8bfec 0f86671f0000 jbe 0x140a8df59
0x140a8bff2 4c89ad60020000 mov qword ptr [rbp + 0x260], r13
0x140a8bff9 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8c000 4883781800 cmp qword ptr [rax + 0x18], 0
0x140a8c005 0f840c020000 je 0x140a8c217
0x140a8c00b 488b8dc00c0000 mov rcx, qword ptr [rbp + 0xcc0]
0x140a8c012 488d9560020000 lea rdx, [rbp + 0x260]
0x140a8c019 e8524db2ff call 0x1405b0d70
0x140a8c01e 4c01e6 add rsi, r12
0x140a8c021 488b95480d0000 mov rdx, qword ptr [rbp + 0xd48]
0x140a8c028 488b0a mov rcx, qword ptr [rdx]
0x140a8c02b 488b5208 mov rdx, qword ptr [rdx + 8]
0x140a8c02f 4989d0 mov r8, rdx
0x140a8c032 4921c0 and r8, rax
0x140a8c035 48c1e839 shr rax, 0x39
0x140a8c039 660f6ec0 movd xmm0, eax
0x140a8c03d 660f60c0 punpcklbw xmm0, xmm0
0x140a8c041 f20f70c000 pshuflw xmm0, xmm0, 0
0x140a8c046 660f70c000 pshufd xmm0, xmm0, 0
0x140a8c04b 488d4188 lea rax, [rcx - 0x78]
0x140a8c04f 4531c9 xor r9d, r9d
0x140a8c052 f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
0x140a8c058 660f6fd1 movdqa xmm2, xmm1
0x140a8c05c 660f74d0 pcmpeqb xmm2, xmm0
0x140a8c060 66440fd7d2 pmovmskb r10d, xmm2
0x140a8c065 4585d2 test r10d, r10d
0x140a8c068 7426 je 0x140a8c090
0x140a8c06a f3450fbcda tzcnt r11d, r10d
0x140a8c06f 4d01c3 add r11, r8
0x140a8c072 4921d3 and r11, rdx
0x140a8c075 49f7db neg r11
0x140a8c078 4d6bdb78 imul r11, r11, 0x78
0x140a8c07c 4e392c18 cmp qword ptr [rax + r11], r13
0x140a8c080 743e je 0x140a8c0c0
0x140a8c082 458d5aff lea r11d, [r10 - 1]
0x140a8c086 664521d3 and r11w, r10w
0x140a8c08a 4589da mov r10d, r11d
0x140a8c08d 75db jne 0x140a8c06a
0x140a8c08f 90 nop 
0x140a8c090 660f74ce pcmpeqb xmm1, xmm6
0x140a8c094 66440fd7d1 pmovmskb r10d, xmm1
0x140a8c099 4585d2 test r10d, r10d
0x140a8c09c 0f8575010000 jne 0x140a8c217
0x140a8c0a2 4d01c8 add r8, r9
0x140a8c0a5 4983c010 add r8, 0x10
0x140a8c0a9 4983c110 add r9, 0x10
0x140a8c0ad 4921d0 and r8, rdx
0x140a8c0b0 eba0 jmp 0x140a8c052
0x140a8c0b2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8c0c0 f3460f104419e8 movss xmm8, dword ptr [rcx + r11 - 0x18]
0x140a8c0c7 4c8ba6a0fdffff mov r12, qword ptr [rsi - 0x260]
0x140a8c0ce 488b86a8fdffff mov rax, qword ptr [rsi - 0x258]
0x140a8c0d5 488d0440 lea rax, [rax + rax*2]
0x140a8c0d9 498d3484 lea rsi, [r12 + rax*4]
0x140a8c0dd eb06 jmp 0x140a8c0e5
0x140a8c0df 90 nop 
0x140a8c0e0 0f2ec1 ucomiss xmm0, xmm1
0x140a8c0e3 774d ja 0x140a8c132
0x140a8c0e5 4939f4 cmp r12, rsi
0x140a8c0e8 7444 je 0x140a8c12e
0x140a8c0ea 66410f6e0c24 movd xmm1, dword ptr [r12]
0x140a8c0f0 66410f6e542408 movd xmm2, dword ptr [r12 + 8]
0x140a8c0f7 488b8de00c0000 mov rcx, qword ptr [rbp + 0xce0]
0x140a8c0fe e83dd5f5ff call 0x1409e9640
0x140a8c103 f3410f104c2404 movss xmm1, dword ptr [r12 + 4]
0x140a8c10a 4983c40c add r12, 0xc
0x140a8c10e 410f2ec8 ucomiss xmm1, xmm8
0x140a8c112 76cc jbe 0x140a8c0e0
0x140a8c114 0f2ec1 ucomiss xmm0, xmm1
0x140a8c117 7502 jne 0x140a8c11b
0x140a8c119 7bca jnp 0x140a8c0e5
0x140a8c11b 488b8df00c0000 mov rcx, qword ptr [rbp + 0xcf0]
0x140a8c122 4c89ea mov rdx, r13
0x140a8c125 e8c6e183ff call 0x1402ca2f0
0x140a8c12a 89d8 mov eax, ebx
0x140a8c12c eb22 jmp 0x140a8c150
0x140a8c12e 89d8 mov eax, ebx
0x140a8c130 eb1e jmp 0x140a8c150
0x140a8c132 488b8df00c0000 mov rcx, qword ptr [rbp + 0xcf0]
0x140a8c139 4c89ea mov rdx, r13
0x140a8c13c e8afe183ff call 0x1402ca2f0
0x140a8c141 89d8 mov eax, ebx
0x140a8c143 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8c150 4d85f6 test r14, r14
0x140a8c153 0f84cf000000 je 0x140a8c228
0x140a8c159 6685c0 test ax, ax
0x140a8c15c 7412 je 0x140a8c170
0x140a8c15e 8d58ff lea ebx, [rax - 1]
0x140a8c161 21c3 and ebx, eax
0x140a8c163 eb30 jmp 0x140a8c195
0x140a8c165 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8c170 66410f6f07 movdqa xmm0, xmmword ptr [r15]
0x140a8c175 660fd7c0 pmovmskb eax, xmm0
0x140a8c179 4883c780 add rdi, -0x80
0x140a8c17d 4983c710 add r15, 0x10
0x140a8c181 3dffff0000 cmp eax, 0xffff
0x140a8c186 74e8 je 0x140a8c170
0x140a8c188 b9feffffff mov ecx, 0xfffffffe
0x140a8c18d 29c1 sub ecx, eax
0x140a8c18f f7d0 not eax
0x140a8c191 89c3 mov ebx, eax
0x140a8c193 21cb and ebx, ecx
0x140a8c195 49ffce dec r14
0x140a8c198 f30fbcc0 tzcnt eax, eax
0x140a8c19c c1e003 shl eax, 3
0x140a8c19f 4889f9 mov rcx, rdi
0x140a8c1a2 4829c1 sub rcx, rax
0x140a8c1a5 4c8b69f8 mov r13, qword ptr [rcx - 8]
0x140a8c1a9 4c89ad60020000 mov qword ptr [rbp + 0x260], r13
0x140a8c1b0 488b85f80c0000 mov rax, qword ptr [rbp + 0xcf8]
0x140a8c1b7 4883781800 cmp qword ptr [rax + 0x18], 0
0x140a8c1bc 7452 je 0x140a8c210
0x140a8c1be 488b8de80c0000 mov rcx, qword ptr [rbp + 0xce8]
0x140a8c1c5 488d9560020000 lea rdx, [rbp + 0x260]
0x140a8c1cc e89f4bb2ff call 0x1405b0d70
0x140a8c1d1 488b8df80c0000 mov rcx, qword ptr [rbp + 0xcf8]
0x140a8c1d8 488b31 mov rsi, qword ptr [rcx]
0x140a8c1db 488b4908 mov rcx, qword ptr [rcx + 8]
0x140a8c1df 4889ca mov rdx, rcx
0x140a8c1e2 4821c2 and rdx, rax
0x140a8c1e5 48c1e839 shr rax, 0x39
0x140a8c1e9 660f6ec0 movd xmm0, eax
0x140a8c1ed 660f60c0 punpcklbw xmm0, xmm0
0x140a8c1f1 f20f70c000 pshuflw xmm0, xmm0, 0
0x140a8c1f6 660f70c000 pshufd xmm0, xmm0, 0
0x140a8c1fb 488d8658fdffff lea rax, [rsi - 0x2a8]
0x140a8c202 4531c0 xor r8d, r8d
0x140a8c205 e956fdffff jmp 0x140a8bf60
0x140a8c20a 660f1f440000 nop word ptr [rax + rax]
0x140a8c210 89d8 mov eax, ebx
0x140a8c212 e939ffffff jmp 0x140a8c150
0x140a8c217 488d0d02d10402 lea rcx, [rip + 0x204d102]
0x140a8c21e e8edd0e401 call 0x1428d9310
0x140a8c223 e9df1f0000 jmp 0x140a8e207
0x140a8c228 4883bd300d000000 cmp qword ptr [rbp + 0xd30], 0
0x140a8c230 488bb5480d0000 mov rsi, qword ptr [rbp + 0xd48]
0x140a8c237 488b95200d0000 mov rdx, qword ptr [rbp + 0xd20]
0x140a8c23e 488b8d000d0000 mov rcx, qword ptr [rbp + 0xd00]
0x140a8c245 7411 je 0x140a8c258
0x140a8c247 4885d2 test rdx, rdx
0x140a8c24a 740c je 0x140a8c258
0x140a8c24c 4c8b85100d0000 mov r8, qword ptr [rbp + 0xd10]
0x140a8c253 e8c879b8ff call 0x140613c20
0x140a8c258 488d8690000000 lea rax, [rsi + 0x90]
0x140a8c25f 488985800b0000 mov qword ptr [rbp + 0xb80], rax
0x140a8c266 48898540080000 mov qword ptr [rbp + 0x840], rax
0x140a8c26d 488d9540080000 lea rdx, [rbp + 0x840]
0x140a8c274 488b8df00c0000 mov rcx, qword ptr [rbp + 0xcf0]
0x140a8c27b e80013aa00 call 0x14152d580
0x140a8c280 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8c287 488bb090000000 mov rsi, qword ptr [rax + 0x90]
0x140a8c28e 488bb8a8000000 mov rdi, qword ptr [rax + 0xa8]
0x140a8c295 660f6f06 movdqa xmm0, xmmword ptr [rsi]
0x140a8c299 66440fd7e0 pmovmskb r12d, xmm0
0x140a8c29e 41f7d4 not r12d
0x140a8c2a1 488d5e10 lea rbx, [rsi + 0x10]
0x140a8c2a5 488b85700e0000 mov rax, qword ptr [rbp + 0xe70]
0x140a8c2ac 8b481c mov ecx, dword ptr [rax + 0x1c]
0x140a8c2af 898d200d0000 mov dword ptr [rbp + 0xd20], ecx
0x140a8c2b5 488b08 mov rcx, qword ptr [rax]
0x140a8c2b8 488b4010 mov rax, qword ptr [rax + 0x10]
0x140a8c2bc 488985100d0000 mov qword ptr [rbp + 0xd10], rax
0x140a8c2c3 48898d300d0000 mov qword ptr [rbp + 0xd30], rcx
0x140a8c2ca 488d4120 lea rax, [rcx + 0x20]
0x140a8c2ce 488985000d0000 mov qword ptr [rbp + 0xd00], rax
0x140a8c2d5 eb2f jmp 0x140a8c306
0x140a8c2d7 660f1f840000000000 nop word ptr [rax + rax]
0x140a8c2e0 488b95300d0000 mov rdx, qword ptr [rbp + 0xd30]
0x140a8c2e7 488b4228 mov rax, qword ptr [rdx + 0x28]
0x140a8c2eb 4c89f9 mov rcx, r15
0x140a8c2ee 48c1e104 shl rcx, 4
0x140a8c2f2 4c892c08 mov qword ptr [rax + rcx], r13
0x140a8c2f6 4c89740808 mov qword ptr [rax + rcx + 8], r14
0x140a8c2fb 49ffc7 inc r15
0x140a8c2fe 4c897a30 mov qword ptr [rdx + 0x30], r15
0x140a8c302 48ff4240 inc qword ptr [rdx + 0x40]
0x140a8c306 4885ff test rdi, rdi
0x140a8c309 0f840e010000 je 0x140a8c41d
0x140a8c30f 664585e4 test r12w, r12w
0x140a8c313 7528 jne 0x140a8c33d
0x140a8c315 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8c320 660f6f03 movdqa xmm0, xmmword ptr [rbx]
0x140a8c324 66440fd7e0 pmovmskb r12d, xmm0
0x140a8c329 4883c680 add rsi, -0x80
0x140a8c32d 4883c310 add rbx, 0x10
0x140a8c331 4181fcffff0000 cmp r12d, 0xffff
0x140a8c338 74e6 je 0x140a8c320
0x140a8c33a 41f7d4 not r12d
0x140a8c33d f3410fbcc4 tzcnt eax, r12d
0x140a8c342 c1e003 shl eax, 3
0x140a8c345 4889f1 mov rcx, rsi
0x140a8c348 4829c1 sub rcx, rax
0x140a8c34b 488b41f8 mov rax, qword ptr [rcx - 8]
0x140a8c34f 488985700c0000 mov qword ptr [rbp + 0xc70], rax
0x140a8c356 488b8de80c0000 mov rcx, qword ptr [rbp + 0xce8]
0x140a8c35d 488d95700c0000 lea rdx, [rbp + 0xc70]
0x140a8c364 e8074ab2ff call 0x1405b0d70
0x140a8c369 488d8d40080000 lea rcx, [rbp + 0x840]
0x140a8c370 488b95f80c0000 mov rdx, qword ptr [rbp + 0xcf8]
0x140a8c377 4989c0 mov r8, rax
0x140a8c37a 4c8d8d700c0000 lea r9, [rbp + 0xc70]
0x140a8c381 e82aaf0e00 call 0x140b772b0
0x140a8c386 418d4424ff lea eax, [r12 - 1]
0x140a8c38b 4421e0 and eax, r12d
0x140a8c38e 4189c4 mov r12d, eax
0x140a8c391 48ffcf dec rdi
0x140a8c394 4531f6 xor r14d, r14d
0x140a8c397 4c3bb548080000 cmp r14, qword ptr [rbp + 0x848]
0x140a8c39e 0f8062ffffff jo 0x140a8c306
0x140a8c3a4 41b8a0020000 mov r8d, 0x2a0
0x140a8c3aa 488d8d60020000 lea rcx, [rbp + 0x260]
0x140a8c3b1 488d9548080000 lea rdx, [rbp + 0x848]
0x140a8c3b8 e84378e901 call 0x142923c00
0x140a8c3bd 4c3bb560020000 cmp r14, qword ptr [rbp + 0x260]
0x140a8c3c4 0f803cffffff jo 0x140a8c306
0x140a8c3ca 488d8d60020000 lea rcx, [rbp + 0x260]
0x140a8c3d1 e82a92e0ff call 0x140895600
0x140a8c3d6 4c8bb5700c0000 mov r14, qword ptr [rbp + 0xc70]
0x140a8c3dd 8b85200d0000 mov eax, dword ptr [rbp + 0xd20]
0x140a8c3e3 488b8d100d0000 mov rcx, qword ptr [rbp + 0xd10]
0x140a8c3ea 8901 mov dword ptr [rcx], eax
0x140a8c3ec 488b85300d0000 mov rax, qword ptr [rbp + 0xd30]
0x140a8c3f3 4c8b7830 mov r15, qword ptr [rax + 0x30]
0x140a8c3f7 4c8b6840 mov r13, qword ptr [rax + 0x40]
0x140a8c3fb 4c3b7820 cmp r15, qword ptr [rax + 0x20]
0x140a8c3ff 0f85dbfeffff jne 0x140a8c2e0
0x140a8c405 488b8d000d0000 mov rcx, qword ptr [rbp + 0xd00]
0x140a8c40c 488d1535dd1a02 lea rdx, [rip + 0x21add35]
0x140a8c413 e81862adff call 0x140562630
0x140a8c418 e9c3feffff jmp 0x140a8c2e0
0x140a8c41d 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8c424 4c8b4030 mov r8, qword ptr [rax + 0x30]
0x140a8c428 488b4848 mov rcx, qword ptr [rax + 0x48]
0x140a8c42c 48898d300d0000 mov qword ptr [rbp + 0xd30], rcx
0x140a8c433 66410f6f00 movdqa xmm0, xmmword ptr [r8]
0x140a8c438 660fd7c8 pmovmskb ecx, xmm0
0x140a8c43c f7d1 not ecx
0x140a8c43e 48898d000d0000 mov qword ptr [rbp + 0xd00], rcx
0x140a8c445 4d8d6810 lea r13, [r8 + 0x10]
0x140a8c449 4c8b6060 mov r12, qword ptr [rax + 0x60]
0x140a8c44d 66410f6f0424 movdqa xmm0, xmmword ptr [r12]
0x140a8c453 66440fd7f8 pmovmskb r15d, xmm0
0x140a8c458 41f7d7 not r15d
0x140a8c45b 488b8da00e0000 mov rcx, qword ptr [rbp + 0xea0]
0x140a8c462 488b09 mov rcx, qword ptr [rcx]
0x140a8c465 48898d180c0000 mov qword ptr [rbp + 0xc18], rcx
0x140a8c46c 488b8db80e0000 mov rcx, qword ptr [rbp + 0xeb8]
0x140a8c473 488b09 mov rcx, qword ptr [rcx]
0x140a8c476 48898d600b0000 mov qword ptr [rbp + 0xb60], rcx
0x140a8c47d 488b8db00e0000 mov rcx, qword ptr [rbp + 0xeb0]
0x140a8c484 488b09 mov rcx, qword ptr [rcx]
0x140a8c487 48898d680b0000 mov qword ptr [rbp + 0xb68], rcx
0x140a8c48e 488b8da80e0000 mov rcx, qword ptr [rbp + 0xea8]
0x140a8c495 488b09 mov rcx, qword ptr [rcx]
0x140a8c498 48898d780b0000 mov qword ptr [rbp + 0xb78], rcx
0x140a8c49f 488b95780e0000 mov rdx, qword ptr [rbp + 0xe78]
0x140a8c4a6 8b4a1c mov ecx, dword ptr [rdx + 0x1c]
0x140a8c4a9 898d600c0000 mov dword ptr [rbp + 0xc60], ecx
0x140a8c4af 488b0a mov rcx, qword ptr [rdx]
0x140a8c4b2 488b5210 mov rdx, qword ptr [rdx + 0x10]
0x140a8c4b6 488995580b0000 mov qword ptr [rbp + 0xb58], rdx
0x140a8c4bd 660f76ff pcmpeqd xmm7, xmm7
0x140a8c4c1 450f57c0 xorps xmm8, xmm8
0x140a8c4c5 f30f1035a393e901 movss xmm6, dword ptr [rip + 0x1e993a3]
0x140a8c4cd f3440f10152a81ea01 movss xmm10, dword ptr [rip + 0x1ea812a]
0x140a8c4d6 f3440f101d8d02ea01 movss xmm11, dword ptr [rip + 0x1ea028d]
0x140a8c4df f3440f103df093e901 movss xmm15, dword ptr [rip + 0x1e993f0]
0x140a8c4e8 f3440f1035730bf401 movss xmm14, dword ptr [rip + 0x1f40b73]
0x140a8c4f1 4d8d742410 lea r14, [r12 + 0x10]
0x140a8c4f6 488b4078 mov rax, qword ptr [rax + 0x78]
0x140a8c4fa 488985100d0000 mov qword ptr [rbp + 0xd10], rax
0x140a8c501 48898d100c0000 mov qword ptr [rbp + 0xc10], rcx
0x140a8c508 488d4120 lea rax, [rcx + 0x20]
0x140a8c50c 488985700b0000 mov qword ptr [rbp + 0xb70], rax
0x140a8c513 4d85c0 test r8, r8
0x140a8c516 0f847c000000 je 0x140a8c598
0x140a8c51c 4883bd300d000000 cmp qword ptr [rbp + 0xd30], 0
0x140a8c524 7467 je 0x140a8c58d
0x140a8c526 488b8d000d0000 mov rcx, qword ptr [rbp + 0xd00]
0x140a8c52d 6685c9 test cx, cx
0x140a8c530 740e je 0x140a8c540
0x140a8c532 8d51ff lea edx, [rcx - 1]
0x140a8c535 21ca and edx, ecx
0x140a8c537 eb2e jmp 0x140a8c567
0x140a8c539 0f1f8000000000 nop dword ptr [rax]
0x140a8c540 66410f6f4500 movdqa xmm0, xmmword ptr [r13]
0x140a8c546 660fd7c8 pmovmskb ecx, xmm0
0x140a8c54a 4983c080 add r8, -0x80
0x140a8c54e 4983c510 add r13, 0x10
0x140a8c552 81f9ffff0000 cmp ecx, 0xffff
0x140a8c558 74e6 je 0x140a8c540
0x140a8c55a b8feffffff mov eax, 0xfffffffe
0x140a8c55f 29c8 sub eax, ecx
0x140a8c561 f7d1 not ecx
0x140a8c563 89ca mov edx, ecx
0x140a8c565 21c2 and edx, eax
0x140a8c567 48ff8d300d0000 dec qword ptr [rbp + 0xd30]
0x140a8c56e b001 mov al, 1
0x140a8c570 488985380d0000 mov qword ptr [rbp + 0xd38], rax
0x140a8c577 4c89c0 mov rax, r8
0x140a8c57a 4c8985200d0000 mov qword ptr [rbp + 0xd20], r8
0x140a8c581 488995000d0000 mov qword ptr [rbp + 0xd00], rdx
0x140a8c588 e994000000 jmp 0x140a8c621
0x140a8c58d 48c785300d000000000000 mov qword ptr [rbp + 0xd30], 0
0x140a8c598 488bbd480d0000 mov rdi, qword ptr [rbp + 0xd48]
0x140a8c59f 4d85e4 test r12, r12
0x140a8c5a2 0f8413180000 je 0x140a8ddbb
0x140a8c5a8 4883bd100d000000 cmp qword ptr [rbp + 0xd10], 0
0x140a8c5b0 0f8405180000 je 0x140a8ddbb
0x140a8c5b6 664585ff test r15w, r15w
0x140a8c5ba 7414 je 0x140a8c5d0
0x140a8c5bc 418d57ff lea edx, [r15 - 1]
0x140a8c5c0 4421fa and edx, r15d
0x140a8c5c3 eb36 jmp 0x140a8c5fb
0x140a8c5c5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8c5d0 66410f6f06 movdqa xmm0, xmmword ptr [r14]
0x140a8c5d5 66440fd7f8 pmovmskb r15d, xmm0
0x140a8c5da 4983c480 add r12, -0x80
0x140a8c5de 4983c610 add r14, 0x10
0x140a8c5e2 4181ffffff0000 cmp r15d, 0xffff
0x140a8c5e9 74e5 je 0x140a8c5d0
0x140a8c5eb b8feffffff mov eax, 0xfffffffe
0x140a8c5f0 4429f8 sub eax, r15d
0x140a8c5f3 41f7d7 not r15d
0x140a8c5f6 4489fa mov edx, r15d
0x140a8c5f9 21c2 and edx, eax
0x140a8c5fb 48ff8d100d0000 dec qword ptr [rbp + 0xd10]
0x140a8c602 48c785200d000000000000 mov qword ptr [rbp + 0xd20], 0
0x140a8c60d 4489f9 mov ecx, r15d
0x140a8c610 4c89e0 mov rax, r12
0x140a8c613 4189d7 mov r15d, edx
0x140a8c616 48c785380d000000000000 mov qword ptr [rbp + 0xd38], 0
0x140a8c621 f30fbcc9 tzcnt ecx, ecx
0x140a8c625 c1e103 shl ecx, 3
0x140a8c628 4829c8 sub rax, rcx
0x140a8c62b 488b70f8 mov rsi, qword ptr [rax - 8]
0x140a8c62f 4889b5b80c0000 mov qword ptr [rbp + 0xcb8], rsi
0x140a8c636 4889b540080000 mov qword ptr [rbp + 0x840], rsi
0x140a8c63d 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8c644 4883781800 cmp qword ptr [rax + 0x18], 0
0x140a8c649 0f8437170000 je 0x140a8dd86
0x140a8c64f 488b8dc00c0000 mov rcx, qword ptr [rbp + 0xcc0]
0x140a8c656 488d9540080000 lea rdx, [rbp + 0x840]
0x140a8c65d e80e47b2ff call 0x1405b0d70
0x140a8c662 488b8d480d0000 mov rcx, qword ptr [rbp + 0xd48]
0x140a8c669 488b19 mov rbx, qword ptr [rcx]
0x140a8c66c 488b4908 mov rcx, qword ptr [rcx + 8]
0x140a8c670 4889ca mov rdx, rcx
0x140a8c673 4821c2 and rdx, rax
0x140a8c676 48c1e839 shr rax, 0x39
0x140a8c67a 660f6ec0 movd xmm0, eax
0x140a8c67e 660f60c0 punpcklbw xmm0, xmm0
0x140a8c682 f20f70c000 pshuflw xmm0, xmm0, 0
0x140a8c687 660f70c000 pshufd xmm0, xmm0, 0
0x140a8c68c 488d4388 lea rax, [rbx - 0x78]
0x140a8c690 4531c0 xor r8d, r8d
0x140a8c693 f30f6f0c13 movdqu xmm1, xmmword ptr [rbx + rdx]
0x140a8c698 660f6fd1 movdqa xmm2, xmm1
0x140a8c69c 660f74d0 pcmpeqb xmm2, xmm0
0x140a8c6a0 66440fd7ca pmovmskb r9d, xmm2
0x140a8c6a5 4585c9 test r9d, r9d
0x140a8c6a8 7425 je 0x140a8c6cf
0x140a8c6aa f3450fbcd1 tzcnt r10d, r9d
0x140a8c6af 4901d2 add r10, rdx
0x140a8c6b2 4921ca and r10, rcx
0x140a8c6b5 49f7da neg r10
0x140a8c6b8 496bfa78 imul rdi, r10, 0x78
0x140a8c6bc 48393438 cmp qword ptr [rax + rdi], rsi
0x140a8c6c0 743e je 0x140a8c700
0x140a8c6c2 458d51ff lea r10d, [r9 - 1]
0x140a8c6c6 664521ca and r10w, r9w
0x140a8c6ca 4589d1 mov r9d, r10d
0x140a8c6cd 75db jne 0x140a8c6aa
0x140a8c6cf 660f74cf pcmpeqb xmm1, xmm7
0x140a8c6d3 66440fd7c9 pmovmskb r9d, xmm1
0x140a8c6d8 4585c9 test r9d, r9d
0x140a8c6db 0f85a5160000 jne 0x140a8dd86
0x140a8c6e1 4c01c2 add rdx, r8
0x140a8c6e4 4883c210 add rdx, 0x10
0x140a8c6e8 4983c010 add r8, 0x10
0x140a8c6ec 4821ca and rdx, rcx
0x140a8c6ef eba2 jmp 0x140a8c693
0x140a8c6f1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8c700 48837c3ba002 cmp qword ptr [rbx + rdi - 0x60], 2
0x140a8c706 4c8b85200d0000 mov r8, qword ptr [rbp + 0xd20]
0x140a8c70d 0f8200feffff jb 0x140a8c513
0x140a8c713 f30f10443bc0 movss xmm0, dword ptr [rbx + rdi - 0x40]
0x140a8c719 410f2ec0 ucomiss xmm0, xmm8
0x140a8c71d 0f86f0fdffff jbe 0x140a8c513
0x140a8c723 488b85b80c0000 mov rax, qword ptr [rbp + 0xcb8]
0x140a8c72a 488985400d0000 mov qword ptr [rbp + 0xd40], rax
0x140a8c731 8b85600c0000 mov eax, dword ptr [rbp + 0xc60]
0x140a8c737 488b8d580b0000 mov rcx, qword ptr [rbp + 0xb58]
0x140a8c73e 8901 mov dword ptr [rcx], eax
0x140a8c740 488b85100c0000 mov rax, qword ptr [rbp + 0xc10]
0x140a8c747 488b7030 mov rsi, qword ptr [rax + 0x30]
0x140a8c74b 488b4840 mov rcx, qword ptr [rax + 0x40]
0x140a8c74f 48898d180d0000 mov qword ptr [rbp + 0xd18], rcx
0x140a8c756 483b7020 cmp rsi, qword ptr [rax + 0x20]
0x140a8c75a 7513 jne 0x140a8c76f
0x140a8c75c 488b8d700b0000 mov rcx, qword ptr [rbp + 0xb70]
0x140a8c763 488d15ded91a02 lea rdx, [rip + 0x21ad9de]
0x140a8c76a e8c15eadff call 0x140562630
0x140a8c76f 488b95100c0000 mov rdx, qword ptr [rbp + 0xc10]
0x140a8c776 488b4228 mov rax, qword ptr [rdx + 0x28]
0x140a8c77a 4889f1 mov rcx, rsi
0x140a8c77d 48c1e104 shl rcx, 4
0x140a8c781 4c8b85180d0000 mov r8, qword ptr [rbp + 0xd18]
0x140a8c788 4c890408 mov qword ptr [rax + rcx], r8
0x140a8c78c 4c8b85400d0000 mov r8, qword ptr [rbp + 0xd40]
0x140a8c793 4c89440808 mov qword ptr [rax + rcx + 8], r8
0x140a8c798 48ffc6 inc rsi
0x140a8c79b 48897230 mov qword ptr [rdx + 0x30], rsi
0x140a8c79f 48ff4240 inc qword ptr [rdx + 0x40]
0x140a8c7a3 488b8de80c0000 mov rcx, qword ptr [rbp + 0xce8]
0x140a8c7aa 488d95b80c0000 lea rdx, [rbp + 0xcb8]
0x140a8c7b1 e8ba45b2ff call 0x1405b0d70
0x140a8c7b6 488d8d40080000 lea rcx, [rbp + 0x840]
0x140a8c7bd 488b95f80c0000 mov rdx, qword ptr [rbp + 0xcf8]
0x140a8c7c4 4989c0 mov r8, rax
0x140a8c7c7 4c8d8db80c0000 lea r9, [rbp + 0xcb8]
0x140a8c7ce e8ddaa0e00 call 0x140b772b0
0x140a8c7d3 31c0 xor eax, eax
0x140a8c7d5 483b8548080000 cmp rax, qword ptr [rbp + 0x848]
0x140a8c7dc 711e jno 0x140a8c7fc
0x140a8c7de 48b80000000000000080 movabs rax, 0x8000000000000000
0x140a8c7e8 488945c0 mov qword ptr [rbp - 0x40], rax
0x140a8c7ec 41b201 mov r10b, 1
0x140a8c7ef 48c785c80c000000000000 mov qword ptr [rbp + 0xcc8], 0
0x140a8c7fa eb2f jmp 0x140a8c82b
0x140a8c7fc 41b8a0020000 mov r8d, 0x2a0
0x140a8c802 488d75c0 lea rsi, [rbp - 0x40]
0x140a8c806 4889f1 mov rcx, rsi
0x140a8c809 488d9548080000 lea rdx, [rbp + 0x848]
0x140a8c810 e8eb73e901 call 0x142923c00
0x140a8c815 31c0 xor eax, eax
0x140a8c817 483b45c0 cmp rax, qword ptr [rbp - 0x40]
0x140a8c81b 711b jno 0x140a8c838
0x140a8c81d 48c785c80c000000000000 mov qword ptr [rbp + 0xcc8], 0
0x140a8c828 41b201 mov r10b, 1
0x140a8c82b b001 mov al, 1
0x140a8c82d 8985280d0000 mov dword ptr [rbp + 0xd28], eax
0x140a8c833 e99e020000 jmp 0x140a8cad6
0x140a8c838 41b8a0020000 mov r8d, 0x2a0
0x140a8c83e 488d8d40080000 lea rcx, [rbp + 0x840]
0x140a8c845 4889f2 mov rdx, rsi
0x140a8c848 e8b373e901 call 0x142923c00
0x140a8c84d 80bd380d000000 cmp byte ptr [rbp + 0xd38], 0
0x140a8c854 0f8556180000 jne 0x140a8e0b0
0x140a8c85a 0fb685b0010000 movzx eax, byte ptr [rbp + 0x1b0]
0x140a8c861 88852f0d0000 mov byte ptr [rbp + 0xd2f], al
0x140a8c867 488d95b1010000 lea rdx, [rbp + 0x1b1]
0x140a8c86e 8b02 mov eax, dword ptr [rdx]
0x140a8c870 8b4a03 mov ecx, dword ptr [rdx + 3]
0x140a8c873 898d930c0000 mov dword ptr [rbp + 0xc93], ecx
0x140a8c879 8985900c0000 mov dword ptr [rbp + 0xc90], eax
0x140a8c87f 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
0x140a8c886 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x140a8c88d 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
0x140a8c894 488985380c0000 mov qword ptr [rbp + 0xc38], rax
0x140a8c89b 0f104217 movups xmm0, xmmword ptr [rdx + 0x17]
0x140a8c89f 0f2985b0070000 movaps xmmword ptr [rbp + 0x7b0], xmm0
0x140a8c8a6 488b4257 mov rax, qword ptr [rdx + 0x57]
0x140a8c8aa 48898530080000 mov qword ptr [rbp + 0x830], rax
0x140a8c8b1 0f104227 movups xmm0, xmmword ptr [rdx + 0x27]
0x140a8c8b5 f30f6f4a37 movdqu xmm1, xmmword ptr [rdx + 0x37]
0x140a8c8ba f30f6f5247 movdqu xmm2, xmmword ptr [rdx + 0x47]
0x140a8c8bf 660f7f9520080000 movdqa xmmword ptr [rbp + 0x820], xmm2
0x140a8c8c7 660f7f8d10080000 movdqa xmmword ptr [rbp + 0x810], xmm1
0x140a8c8cf 0f298500080000 movaps xmmword ptr [rbp + 0x800], xmm0
0x140a8c8d6 8b8510020000 mov eax, dword ptr [rbp + 0x210]
0x140a8c8dc 89856c0c0000 mov dword ptr [rbp + 0xc6c], eax
0x140a8c8e2 8b8514020000 mov eax, dword ptr [rbp + 0x214]
0x140a8c8e8 8985640c0000 mov dword ptr [rbp + 0xc64], eax
0x140a8c8ee 488b9540080000 mov rdx, qword ptr [rbp + 0x840]
0x140a8c8f5 4885d2 test rdx, rdx
0x140a8c8f8 7416 je 0x140a8c910
0x140a8c8fa 488b8d48080000 mov rcx, qword ptr [rbp + 0x848]
0x140a8c901 48c1e203 shl rdx, 3
0x140a8c905 41b804000000 mov r8d, 4
0x140a8c90b e81073b8ff call 0x140613c20
0x140a8c910 488b9558080000 mov rdx, qword ptr [rbp + 0x858]
0x140a8c917 4885d2 test rdx, rdx
0x140a8c91a 7416 je 0x140a8c932
0x140a8c91c 488b8d60080000 mov rcx, qword ptr [rbp + 0x860]
0x140a8c923 48c1e202 shl rdx, 2
0x140a8c927 41b804000000 mov r8d, 4
0x140a8c92d e8ee72b8ff call 0x140613c20
0x140a8c932 488b8578080000 mov rax, qword ptr [rbp + 0x878]
0x140a8c939 4885c0 test rax, rax
0x140a8c93c 741a je 0x140a8c958
0x140a8c93e 488b8d80080000 mov rcx, qword ptr [rbp + 0x880]
0x140a8c945 48c1e002 shl rax, 2
0x140a8c949 488d1440 lea rdx, [rax + rax*2]
0x140a8c94d 41b804000000 mov r8d, 4
0x140a8c953 e8c872b8ff call 0x140613c20
0x140a8c958 488b9590080000 mov rdx, qword ptr [rbp + 0x890]
0x140a8c95f 4885d2 test rdx, rdx
0x140a8c962 7416 je 0x140a8c97a
0x140a8c964 488b8d98080000 mov rcx, qword ptr [rbp + 0x898]
0x140a8c96b 48c1e202 shl rdx, 2
0x140a8c96f 41b804000000 mov r8d, 4
0x140a8c975 e8a672b8ff call 0x140613c20
0x140a8c97a 488b95b0080000 mov rdx, qword ptr [rbp + 0x8b0]
0x140a8c981 4885d2 test rdx, rdx
0x140a8c984 7416 je 0x140a8c99c
0x140a8c986 488b8db8080000 mov rcx, qword ptr [rbp + 0x8b8]
0x140a8c98d 48c1e202 shl rdx, 2
0x140a8c991 41b804000000 mov r8d, 4
0x140a8c997 e88472b8ff call 0x140613c20
0x140a8c99c 488b85c8080000 mov rax, qword ptr [rbp + 0x8c8]
0x140a8c9a3 4885c0 test rax, rax
0x140a8c9a6 741a je 0x140a8c9c2
0x140a8c9a8 488b8dd0080000 mov rcx, qword ptr [rbp + 0x8d0]
0x140a8c9af 48c1e002 shl rax, 2
0x140a8c9b3 488d1440 lea rdx, [rax + rax*2]
0x140a8c9b7 41b804000000 mov r8d, 4
0x140a8c9bd e85e72b8ff call 0x140613c20
0x140a8c9c2 488b95e0080000 mov rdx, qword ptr [rbp + 0x8e0]
0x140a8c9c9 4885d2 test rdx, rdx
0x140a8c9cc 7416 je 0x140a8c9e4
0x140a8c9ce 488b8de8080000 mov rcx, qword ptr [rbp + 0x8e8]
0x140a8c9d5 48c1e202 shl rdx, 2
0x140a8c9d9 41b804000000 mov r8d, 4
0x140a8c9df e83c72b8ff call 0x140613c20
0x140a8c9e4 488b9500090000 mov rdx, qword ptr [rbp + 0x900]
0x140a8c9eb 4885d2 test rdx, rdx
0x140a8c9ee 7416 je 0x140a8ca06
0x140a8c9f0 488b8d08090000 mov rcx, qword ptr [rbp + 0x908]
0x140a8c9f7 48c1e202 shl rdx, 2
0x140a8c9fb 41b804000000 mov r8d, 4
0x140a8ca01 e81a72b8ff call 0x140613c20
0x140a8ca06 488d8d58090000 lea rcx, [rbp + 0x958]
0x140a8ca0d e87e29dfff call 0x14087f390
0x140a8ca12 488d8d68090000 lea rcx, [rbp + 0x968]
0x140a8ca19 e87229dfff call 0x14087f390
0x140a8ca1e 488d8d78090000 lea rcx, [rbp + 0x978]
0x140a8ca25 e86629dfff call 0x14087f390
0x140a8ca2a 488d8d88090000 lea rcx, [rbp + 0x988]
0x140a8ca31 e85a29dfff call 0x14087f390
0x140a8ca36 488d8d98090000 lea rcx, [rbp + 0x998]
0x140a8ca3d e84e29dfff call 0x14087f390
0x140a8ca42 488d8da8090000 lea rcx, [rbp + 0x9a8]
0x140a8ca49 e84229dfff call 0x14087f390
0x140a8ca4e 488d8db8090000 lea rcx, [rbp + 0x9b8]
0x140a8ca55 e83629dfff call 0x14087f390
0x140a8ca5a 488b9518090000 mov rdx, qword ptr [rbp + 0x918]
0x140a8ca61 4885d2 test rdx, rdx
0x140a8ca64 7416 je 0x140a8ca7c
0x140a8ca66 488b8d20090000 mov rcx, qword ptr [rbp + 0x920]
0x140a8ca6d 48c1e202 shl rdx, 2
0x140a8ca71 41b804000000 mov r8d, 4
0x140a8ca77 e8a471b8ff call 0x140613c20
0x140a8ca7c 488b9530090000 mov rdx, qword ptr [rbp + 0x930]
0x140a8ca83 4885d2 test rdx, rdx
0x140a8ca86 7416 je 0x140a8ca9e
0x140a8ca88 488b8d38090000 mov rcx, qword ptr [rbp + 0x938]
0x140a8ca8f 48c1e202 shl rdx, 2
0x140a8ca93 41b804000000 mov r8d, 4
0x140a8ca99 e88271b8ff call 0x140613c20
0x140a8ca9e 488b95d0090000 mov rdx, qword ptr [rbp + 0x9d0]
0x140a8caa5 4885d2 test rdx, rdx
0x140a8caa8 7416 je 0x140a8cac0
0x140a8caaa 488b8dd8090000 mov rcx, qword ptr [rbp + 0x9d8]
0x140a8cab1 48c1e203 shl rdx, 3
0x140a8cab5 41b804000000 mov r8d, 4
0x140a8cabb e86071b8ff call 0x140613c20
0x140a8cac0 b001 mov al, 1
0x140a8cac2 488985c80c0000 mov qword ptr [rbp + 0xcc8], rax
0x140a8cac9 4531d2 xor r10d, r10d
0x140a8cacc c785280d000000000000 mov dword ptr [rbp + 0xd28], 0
0x140a8cad6 488d043b lea rax, [rbx + rdi]
0x140a8cada 488985400d0000 mov qword ptr [rbp + 0xd40], rax
0x140a8cae1 4c8d1c3b lea r11, [rbx + rdi]
0x140a8cae5 4983c390 add r11, -0x70
0x140a8cae9 8b85900c0000 mov eax, dword ptr [rbp + 0xc90]
0x140a8caef 8b8d930c0000 mov ecx, dword ptr [rbp + 0xc93]
0x140a8caf5 8985880c0000 mov dword ptr [rbp + 0xc88], eax
0x140a8cafb 898d8b0c0000 mov dword ptr [rbp + 0xc8b], ecx
0x140a8cb01 0f2885b0070000 movaps xmm0, xmmword ptr [rbp + 0x7b0]
0x140a8cb08 0f2985a0070000 movaps xmmword ptr [rbp + 0x7a0], xmm0
0x140a8cb0f 488b8530080000 mov rax, qword ptr [rbp + 0x830]
0x140a8cb16 488985f0070000 mov qword ptr [rbp + 0x7f0], rax
0x140a8cb1d 0f288500080000 movaps xmm0, xmmword ptr [rbp + 0x800]
0x140a8cb24 0f288d10080000 movaps xmm1, xmmword ptr [rbp + 0x810]
0x140a8cb2b 0f289520080000 movaps xmm2, xmmword ptr [rbp + 0x820]
0x140a8cb32 0f2995e0070000 movaps xmmword ptr [rbp + 0x7e0], xmm2
0x140a8cb39 0f298dd0070000 movaps xmmword ptr [rbp + 0x7d0], xmm1
0x140a8cb40 0f2985c0070000 movaps xmmword ptr [rbp + 0x7c0], xmm0
0x140a8cb47 488b85b80c0000 mov rax, qword ptr [rbp + 0xcb8]
0x140a8cb4e 488b95600b0000 mov rdx, qword ptr [rbp + 0xb60]
0x140a8cb55 488b4a08 mov rcx, qword ptr [rdx + 8]
0x140a8cb59 4869521098000000 imul rdx, qword ptr [rdx + 0x10], 0x98
0x140a8cb61 4989d1 mov r9, rdx
0x140a8cb64 4885d2 test rdx, rdx
0x140a8cb67 7417 je 0x140a8cb80
0x140a8cb69 4c8d8198000000 lea r8, [rcx + 0x98]
0x140a8cb70 498d9168ffffff lea rdx, [r9 - 0x98]
0x140a8cb77 48394170 cmp qword ptr [rcx + 0x70], rax
0x140a8cb7b 4c89c1 mov rcx, r8
0x140a8cb7e 75e1 jne 0x140a8cb61
0x140a8cb80 4c898d880b0000 mov qword ptr [rbp + 0xb88], r9
0x140a8cb87 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8cb8e 488b78a0 mov rdi, qword ptr [rax - 0x60]
0x140a8cb92 4c8d0cfd00000000 lea r9, [rdi*8]
0x140a8cb9a 4889f8 mov rax, rdi
0x140a8cb9d 48c1e83d shr rax, 0x3d
0x140a8cba1 448995a80c0000 mov dword ptr [rbp + 0xca8], r10d
0x140a8cba8 0f85f5150000 jne 0x140a8e1a3
0x140a8cbae b900000000 mov ecx, 0
0x140a8cbb3 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
0x140a8cbbd 4939c1 cmp r9, rax
0x140a8cbc0 0f87df150000 ja 0x140a8e1a5
0x140a8cbc6 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8cbcd 488b7098 mov rsi, qword ptr [rax - 0x68]
0x140a8cbd1 4d85c9 test r9, r9
0x140a8cbd4 4c899d980c0000 mov qword ptr [rbp + 0xc98], r11
0x140a8cbdb 7439 je 0x140a8cc16
0x140a8cbdd 0fb6052580ba02 movzx eax, byte ptr [rip + 0x2ba8025]
0x140a8cbe4 ba04000000 mov edx, 4
0x140a8cbe9 4c89c9 mov rcx, r9
0x140a8cbec 4c898d180d0000 mov qword ptr [rbp + 0xd18], r9
0x140a8cbf3 e81870b8ff call 0x140613c10
0x140a8cbf8 b904000000 mov ecx, 4
0x140a8cbfd 4c8b8d180d0000 mov r9, qword ptr [rbp + 0xd18]
0x140a8cc04 4885c0 test rax, rax
0x140a8cc07 0f8498150000 je 0x140a8e1a5
0x140a8cc0d 4889bd380d0000 mov qword ptr [rbp + 0xd38], rdi
0x140a8cc14 eb10 jmp 0x140a8cc26
0x140a8cc16 b804000000 mov eax, 4
0x140a8cc1b 48c785380d000000000000 mov qword ptr [rbp + 0xd38], 0
0x140a8cc26 488985180d0000 mov qword ptr [rbp + 0xd18], rax
0x140a8cc2d 4889c1 mov rcx, rax
0x140a8cc30 4889f2 mov rdx, rsi
0x140a8cc33 4d89c8 mov r8, r9
0x140a8cc36 e8c56fe901 call 0x142923c00
0x140a8cc3b 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8cc42 488b58b8 mov rbx, qword ptr [rax - 0x48]
0x140a8cc46 4c8d0c9d00000000 lea r9, [rbx*4]
0x140a8cc4e 4889d8 mov rax, rbx
0x140a8cc51 48c1e83e shr rax, 0x3e
0x140a8cc55 0f855b150000 jne 0x140a8e1b6
0x140a8cc5b b900000000 mov ecx, 0
0x140a8cc60 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
0x140a8cc6a 4939c1 cmp r9, rax
0x140a8cc6d 0f8745150000 ja 0x140a8e1b8
0x140a8cc73 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8cc7a 488b70b0 mov rsi, qword ptr [rax - 0x50]
0x140a8cc7e 4d85c9 test r9, r9
0x140a8cc81 7439 je 0x140a8ccbc
0x140a8cc83 0fb6057f7fba02 movzx eax, byte ptr [rip + 0x2ba7f7f]
0x140a8cc8a ba04000000 mov edx, 4
0x140a8cc8f 4c89c9 mov rcx, r9
0x140a8cc92 4c898dd00c0000 mov qword ptr [rbp + 0xcd0], r9
0x140a8cc99 e8726fb8ff call 0x140613c10
0x140a8cc9e b904000000 mov ecx, 4
0x140a8cca3 4c8b8dd00c0000 mov r9, qword ptr [rbp + 0xcd0]
0x140a8ccaa 4885c0 test rax, rax
0x140a8ccad 0f8405150000 je 0x140a8e1b8
0x140a8ccb3 48899d080d0000 mov qword ptr [rbp + 0xd08], rbx
0x140a8ccba eb10 jmp 0x140a8cccc
0x140a8ccbc b804000000 mov eax, 4
0x140a8ccc1 48c785080d000000000000 mov qword ptr [rbp + 0xd08], 0
0x140a8cccc 4889c1 mov rcx, rax
0x140a8cccf 4889f2 mov rdx, rsi
0x140a8ccd2 4d89c8 mov r8, r9
0x140a8ccd5 4889c6 mov rsi, rax
0x140a8ccd8 e8236fe901 call 0x142923c00
0x140a8ccdd 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8cce4 660f6e78c0 movd xmm7, dword ptr [rax - 0x40]
0x140a8cce9 488b8d380d0000 mov rcx, qword ptr [rbp + 0xd38]
0x140a8ccf0 48898d40080000 mov qword ptr [rbp + 0x840], rcx
0x140a8ccf7 488b8d180d0000 mov rcx, qword ptr [rbp + 0xd18]
0x140a8ccfe 48898d48080000 mov qword ptr [rbp + 0x848], rcx
0x140a8cd05 4889bd50080000 mov qword ptr [rbp + 0x850], rdi
0x140a8cd0c 488b8d080d0000 mov rcx, qword ptr [rbp + 0xd08]
0x140a8cd13 48898d58080000 mov qword ptr [rbp + 0x858], rcx
0x140a8cd1a 4889b560080000 mov qword ptr [rbp + 0x860], rsi
0x140a8cd21 48899d68080000 mov qword ptr [rbp + 0x868], rbx
0x140a8cd28 660f7ebd70080000 movd dword ptr [rbp + 0x870], xmm7
0x140a8cd30 8b40c8 mov eax, dword ptr [rax - 0x38]
0x140a8cd33 488985d00c0000 mov qword ptr [rbp + 0xcd0], rax
0x140a8cd3a 83f801 cmp eax, 1
0x140a8cd3d 410f94c0 sete r8b
0x140a8cd41 488b85980c0000 mov rax, qword ptr [rbp + 0xc98]
0x140a8cd48 4889442428 mov qword ptr [rsp + 0x28], rax
0x140a8cd4d 488b85e00c0000 mov rax, qword ptr [rbp + 0xce0]
0x140a8cd54 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a8cd59 488d8d60020000 lea rcx, [rbp + 0x260]
0x140a8cd60 488d9540080000 lea rdx, [rbp + 0x840]
0x140a8cd67 0f28de movaps xmm3, xmm6
0x140a8cd6a e801ee9d00 call 0x14146bb70
0x140a8cd6f 488b9540080000 mov rdx, qword ptr [rbp + 0x840]
0x140a8cd76 4885d2 test rdx, rdx
0x140a8cd79 7416 je 0x140a8cd91
0x140a8cd7b 488b8d48080000 mov rcx, qword ptr [rbp + 0x848]
0x140a8cd82 48c1e203 shl rdx, 3
0x140a8cd86 41b804000000 mov r8d, 4
0x140a8cd8c e88f6eb8ff call 0x140613c20
0x140a8cd91 488b9558080000 mov rdx, qword ptr [rbp + 0x858]
0x140a8cd98 4885d2 test rdx, rdx
0x140a8cd9b 7416 je 0x140a8cdb3
0x140a8cd9d 488b8d60080000 mov rcx, qword ptr [rbp + 0x860]
0x140a8cda4 48c1e202 shl rdx, 2
0x140a8cda8 41b804000000 mov r8d, 4
0x140a8cdae e86d6eb8ff call 0x140613c20
0x140a8cdb3 488b8568020000 mov rax, qword ptr [rbp + 0x268]
0x140a8cdba 488985080d0000 mov qword ptr [rbp + 0xd08], rax
0x140a8cdc1 488b9d70020000 mov rbx, qword ptr [rbp + 0x270]
0x140a8cdc8 488d0c9d00000000 lea rcx, [rbx*4]
0x140a8cdd0 4885db test rbx, rbx
0x140a8cdd3 48899d180d0000 mov qword ptr [rbp + 0xd18], rbx
0x140a8cdda 48898dd80c0000 mov qword ptr [rbp + 0xcd8], rcx
0x140a8cde1 7469 je 0x140a8ce4c
0x140a8cde3 0fb6051f7eba02 movzx eax, byte ptr [rip + 0x2ba7e1f]
0x140a8cdea ba04000000 mov edx, 4
0x140a8cdef e81c6eb8ff call 0x140613c10
0x140a8cdf4 488985380d0000 mov qword ptr [rbp + 0xd38], rax
0x140a8cdfb 4885c0 test rax, rax
0x140a8cdfe 0f8485130000 je 0x140a8e189
0x140a8ce04 488b85080d0000 mov rax, qword ptr [rbp + 0xd08]
0x140a8ce0b 488d7008 lea rsi, [rax + 8]
0x140a8ce0f 31ff xor edi, edi
0x140a8ce11 f30f104ef8 movss xmm1, dword ptr [rsi - 8]
0x140a8ce16 f3440f104efc movss xmm9, dword ptr [rsi - 4]
0x140a8ce1c f30f1016 movss xmm2, dword ptr [rsi]
0x140a8ce20 488b8de00c0000 mov rcx, qword ptr [rbp + 0xce0]
0x140a8ce27 e814c8f5ff call 0x1409e9640
0x140a8ce2c f3440f5cc8 subss xmm9, xmm0
0x140a8ce31 488b85380d0000 mov rax, qword ptr [rbp + 0xd38]
0x140a8ce38 f3440f110cb8 movss dword ptr [rax + rdi*4], xmm9
0x140a8ce3e 48ffc7 inc rdi
0x140a8ce41 4883c60c add rsi, 0xc
0x140a8ce45 4839fb cmp rbx, rdi
0x140a8ce48 75c7 jne 0x140a8ce11
0x140a8ce4a eb0c jmp 0x140a8ce58
0x140a8ce4c b804000000 mov eax, 4
0x140a8ce51 488985380d0000 mov qword ptr [rbp + 0xd38], rax
0x140a8ce58 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8ce5f f3440f1060e4 movss xmm12, dword ptr [rax - 0x1c]
0x140a8ce65 0fb640f5 movzx eax, byte ptr [rax - 0xb]
0x140a8ce69 8885980c0000 mov byte ptr [rbp + 0xc98], al
0x140a8ce6f 488b8d180c0000 mov rcx, qword ptr [rbp + 0xc18]
0x140a8ce76 488d153b141a02 lea rdx, [rip + 0x21a143b]
0x140a8ce7d e89ec8edff call 0x140969720
0x140a8ce82 4889c6 mov rsi, rax
0x140a8ce85 4885c0 test rax, rax
0x140a8ce88 0f8450120000 je 0x140a8e0de
0x140a8ce8e 488d0d6a090102 lea rcx, [rip + 0x201096a]
0x140a8ce95 48898d400c0000 mov qword ptr [rbp + 0xc40], rcx
0x140a8ce9c 48c785480c00003a000000 mov qword ptr [rbp + 0xc48], 0x3a
0x140a8cea7 4889f7 mov rdi, rsi
0x140a8ceaa 4883c710 add rdi, 0x10
0x140a8ceae 48837e183a cmp qword ptr [rsi + 0x18], 0x3a
0x140a8ceb3 0f8592110000 jne 0x140a8e04b
0x140a8ceb9 488b17 mov rdx, qword ptr [rdi]
0x140a8cebc 41b83a000000 mov r8d, 0x3a
0x140a8cec2 e8296de901 call 0x142923bf0
0x140a8cec7 85c0 test eax, eax
0x140a8cec9 0f857c110000 jne 0x140a8e04b
0x140a8cecf 488b0e mov rcx, qword ptr [rsi]
0x140a8ced2 488b4608 mov rax, qword ptr [rsi + 8]
0x140a8ced6 ff5018 call qword ptr [rax + 0x18]
0x140a8ced9 4889c7 mov rdi, rax
0x140a8cedc 4885c0 test rax, rax
0x140a8cedf 0f84f9110000 je 0x140a8e0de
0x140a8cee5 0fb6051d7dba02 movzx eax, byte ptr [rip + 0x2ba7d1d]
0x140a8ceec b920000000 mov ecx, 0x20
0x140a8cef1 ba08000000 mov edx, 8
0x140a8cef6 e8156db8ff call 0x140613c10
0x140a8cefb 4885c0 test rax, rax
0x140a8cefe 0f84fc110000 je 0x140a8e100
0x140a8cf04 4889c2 mov rdx, rax
0x140a8cf07 48c70000000000 mov qword ptr [rax], 0
0x140a8cf0e 48c7400804000000 mov qword ptr [rax + 8], 4
0x140a8cf16 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x140a8cf1e c6401800 mov byte ptr [rax + 0x18], 0
0x140a8cf22 488b7760 mov rsi, qword ptr [rdi + 0x60]
0x140a8cf26 eb02 jmp 0x140a8cf2a
0x140a8cf28 f390 pause 
0x140a8cf2a 488b4608 mov rax, qword ptr [rsi + 8]
0x140a8cf2e 6690 nop 
0x140a8cf30 4883f8ff cmp rax, -1
0x140a8cf34 74f2 je 0x140a8cf28
0x140a8cf36 4885c0 test rax, rax
0x140a8cf39 0f8864100000 js 0x140a8dfa3
0x140a8cf3f 488d4801 lea rcx, [rax + 1]
0x140a8cf43 f0480fb14e08 lock cmpxchg qword ptr [rsi + 8], rcx
0x140a8cf49 75e5 jne 0x140a8cf30
0x140a8cf4b 4889b5480b0000 mov qword ptr [rbp + 0xb48], rsi
0x140a8cf52 488995900b0000 mov qword ptr [rbp + 0xb90], rdx
0x140a8cf59 488995500b0000 mov qword ptr [rbp + 0xb50], rdx
0x140a8cf60 0fb605a27cba02 movzx eax, byte ptr [rip + 0x2ba7ca2]
0x140a8cf67 b920000000 mov ecx, 0x20
0x140a8cf6c ba08000000 mov edx, 8
0x140a8cf71 e89a6cb8ff call 0x140613c10
0x140a8cf76 4885c0 test rax, rax
0x140a8cf79 0f8495110000 je 0x140a8e114
0x140a8cf7f 4889c3 mov rbx, rax
0x140a8cf82 48c70000000000 mov qword ptr [rax], 0
0x140a8cf89 48c7400804000000 mov qword ptr [rax + 8], 4
0x140a8cf91 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x140a8cf99 c6401800 mov byte ptr [rax + 0x18], 0
0x140a8cf9d 488b5760 mov rdx, qword ptr [rdi + 0x60]
0x140a8cfa1 eb02 jmp 0x140a8cfa5
0x140a8cfa3 f390 pause 
0x140a8cfa5 488b4208 mov rax, qword ptr [rdx + 8]
0x140a8cfa9 0f1f8000000000 nop dword ptr [rax]
0x140a8cfb0 4883f8ff cmp rax, -1
0x140a8cfb4 74ed je 0x140a8cfa3
0x140a8cfb6 4885c0 test rax, rax
0x140a8cfb9 0f88fc0f0000 js 0x140a8dfbb
0x140a8cfbf 488d4801 lea rcx, [rax + 1]
0x140a8cfc3 f0480fb14a08 lock cmpxchg qword ptr [rdx + 8], rcx
0x140a8cfc9 75e5 jne 0x140a8cfb0
0x140a8cfcb 488995980b0000 mov qword ptr [rbp + 0xb98], rdx
0x140a8cfd2 488995380b0000 mov qword ptr [rbp + 0xb38], rdx
0x140a8cfd9 48899d400b0000 mov qword ptr [rbp + 0xb40], rbx
0x140a8cfe0 0fb605227cba02 movzx eax, byte ptr [rip + 0x2ba7c22]
0x140a8cfe7 b920000000 mov ecx, 0x20
0x140a8cfec ba08000000 mov edx, 8
0x140a8cff1 e81a6cb8ff call 0x140613c10
0x140a8cff6 4885c0 test rax, rax
0x140a8cff9 0f8429110000 je 0x140a8e128
0x140a8cfff 4889c2 mov rdx, rax
0x140a8d002 48c70000000000 mov qword ptr [rax], 0
0x140a8d009 48c7400804000000 mov qword ptr [rax + 8], 4
0x140a8d011 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x140a8d019 c6401800 mov byte ptr [rax + 0x18], 0
0x140a8d01d 4c8b4760 mov r8, qword ptr [rdi + 0x60]
0x140a8d021 eb02 jmp 0x140a8d025
0x140a8d023 f390 pause 
0x140a8d025 498b4008 mov rax, qword ptr [r8 + 8]
0x140a8d029 0f1f8000000000 nop dword ptr [rax]
0x140a8d030 4883f8ff cmp rax, -1
0x140a8d034 74ed je 0x140a8d023
0x140a8d036 4885c0 test rax, rax
0x140a8d039 0f88940f0000 js 0x140a8dfd3
0x140a8d03f 488d4801 lea rcx, [rax + 1]
0x140a8d043 f0490fb14808 lock cmpxchg qword ptr [r8 + 8], rcx
0x140a8d049 75e5 jne 0x140a8d030
0x140a8d04b 4c8985a80b0000 mov qword ptr [rbp + 0xba8], r8
0x140a8d052 4c8985280b0000 mov qword ptr [rbp + 0xb28], r8
0x140a8d059 488995a00b0000 mov qword ptr [rbp + 0xba0], rdx
0x140a8d060 488995300b0000 mov qword ptr [rbp + 0xb30], rdx
0x140a8d067 0fb6059b7bba02 movzx eax, byte ptr [rip + 0x2ba7b9b]
0x140a8d06e b920000000 mov ecx, 0x20
0x140a8d073 ba08000000 mov edx, 8
0x140a8d078 e8936bb8ff call 0x140613c10
0x140a8d07d 4885c0 test rax, rax
0x140a8d080 0f84b6100000 je 0x140a8e13c
0x140a8d086 4889c2 mov rdx, rax
0x140a8d089 48c70000000000 mov qword ptr [rax], 0
0x140a8d090 48c7400804000000 mov qword ptr [rax + 8], 4
0x140a8d098 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x140a8d0a0 c6401800 mov byte ptr [rax + 0x18], 0
0x140a8d0a4 4c8b4760 mov r8, qword ptr [rdi + 0x60]
0x140a8d0a8 eb02 jmp 0x140a8d0ac
0x140a8d0aa f390 pause 
0x140a8d0ac 498b4008 mov rax, qword ptr [r8 + 8]
0x140a8d0b0 4883f8ff cmp rax, -1
0x140a8d0b4 74f4 je 0x140a8d0aa
0x140a8d0b6 4885c0 test rax, rax
0x140a8d0b9 0f882c0f0000 js 0x140a8dfeb
0x140a8d0bf 488d4801 lea rcx, [rax + 1]
0x140a8d0c3 f0490fb14808 lock cmpxchg qword ptr [r8 + 8], rcx
0x140a8d0c9 75e5 jne 0x140a8d0b0
0x140a8d0cb 4c8985b80b0000 mov qword ptr [rbp + 0xbb8], r8
0x140a8d0d2 4c8985180b0000 mov qword ptr [rbp + 0xb18], r8
0x140a8d0d9 488995b00b0000 mov qword ptr [rbp + 0xbb0], rdx
0x140a8d0e0 488995200b0000 mov qword ptr [rbp + 0xb20], rdx
0x140a8d0e7 488b8d180c0000 mov rcx, qword ptr [rbp + 0xc18]
0x140a8d0ee 488d156b121a02 lea rdx, [rip + 0x21a126b]
0x140a8d0f5 e826c6edff call 0x140969720
0x140a8d0fa 4889c7 mov rdi, rax
0x140a8d0fd 4885c0 test rax, rax
0x140a8d100 0f84e90f0000 je 0x140a8e0ef
0x140a8d106 488d0d2c0d0102 lea rcx, [rip + 0x2010d2c]
0x140a8d10d 48898d400c0000 mov qword ptr [rbp + 0xc40], rcx
0x140a8d114 48c785480c00003d000000 mov qword ptr [rbp + 0xc48], 0x3d
0x140a8d11f 4989f8 mov r8, rdi
0x140a8d122 4983c010 add r8, 0x10
0x140a8d126 48837f183d cmp qword ptr [rdi + 0x18], 0x3d
0x140a8d12b 0f854e0f0000 jne 0x140a8e07f
0x140a8d131 498b10 mov rdx, qword ptr [r8]
0x140a8d134 4c8985780c0000 mov qword ptr [rbp + 0xc78], r8
0x140a8d13b 41b83d000000 mov r8d, 0x3d
0x140a8d141 e8aa6ae901 call 0x142923bf0
0x140a8d146 4c8b85780c0000 mov r8, qword ptr [rbp + 0xc78]
0x140a8d14d 85c0 test eax, eax
0x140a8d14f 0f852a0f0000 jne 0x140a8e07f
0x140a8d155 488b0f mov rcx, qword ptr [rdi]
0x140a8d158 488b4708 mov rax, qword ptr [rdi + 8]
0x140a8d15c ff5018 call qword ptr [rax + 0x18]
0x140a8d15f 4889c7 mov rdi, rax
0x140a8d162 4885c0 test rax, rax
0x140a8d165 0f84840f0000 je 0x140a8e0ef
0x140a8d16b 48899dc00b0000 mov qword ptr [rbp + 0xbc0], rbx
0x140a8d172 4889b5780c0000 mov qword ptr [rbp + 0xc78], rsi
0x140a8d179 0fb605897aba02 movzx eax, byte ptr [rip + 0x2ba7a89]
0x140a8d180 b920000000 mov ecx, 0x20
0x140a8d185 ba08000000 mov edx, 8
0x140a8d18a e8816ab8ff call 0x140613c10
0x140a8d18f 4885c0 test rax, rax
0x140a8d192 488bb5180d0000 mov rsi, qword ptr [rbp + 0xd18]
0x140a8d199 488b9d380d0000 mov rbx, qword ptr [rbp + 0xd38]
0x140a8d1a0 0f84aa0f0000 je 0x140a8e150
0x140a8d1a6 4889c2 mov rdx, rax
0x140a8d1a9 48c70000000000 mov qword ptr [rax], 0
0x140a8d1b0 48c7400804000000 mov qword ptr [rax + 8], 4
0x140a8d1b8 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x140a8d1c0 c6401800 mov byte ptr [rax + 0x18], 0
0x140a8d1c4 4c8b4760 mov r8, qword ptr [rdi + 0x60]
0x140a8d1c8 eb02 jmp 0x140a8d1cc
0x140a8d1ca f390 pause 
0x140a8d1cc 498b4008 mov rax, qword ptr [r8 + 8]
0x140a8d1d0 4883f8ff cmp rax, -1
0x140a8d1d4 74f4 je 0x140a8d1ca
0x140a8d1d6 4885c0 test rax, rax
0x140a8d1d9 0f88240e0000 js 0x140a8e003
0x140a8d1df 488d4801 lea rcx, [rax + 1]
0x140a8d1e3 f0490fb14808 lock cmpxchg qword ptr [r8 + 8], rcx
0x140a8d1e9 75e5 jne 0x140a8d1d0
0x140a8d1eb 4c8985d00b0000 mov qword ptr [rbp + 0xbd0], r8
0x140a8d1f2 4c8985080b0000 mov qword ptr [rbp + 0xb08], r8
0x140a8d1f9 488995c80b0000 mov qword ptr [rbp + 0xbc8], rdx
0x140a8d200 488995100b0000 mov qword ptr [rbp + 0xb10], rdx
0x140a8d207 0fb605fb79ba02 movzx eax, byte ptr [rip + 0x2ba79fb]
0x140a8d20e b920000000 mov ecx, 0x20
0x140a8d213 ba08000000 mov edx, 8
0x140a8d218 e8f369b8ff call 0x140613c10
0x140a8d21d 4885c0 test rax, rax
0x140a8d220 0f843e0f0000 je 0x140a8e164
0x140a8d226 4889c2 mov rdx, rax
0x140a8d229 48c70000000000 mov qword ptr [rax], 0
0x140a8d230 48c7400804000000 mov qword ptr [rax + 8], 4
0x140a8d238 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x140a8d240 c6401800 mov byte ptr [rax + 0x18], 0
0x140a8d244 4c8b4760 mov r8, qword ptr [rdi + 0x60]
0x140a8d248 eb02 jmp 0x140a8d24c
0x140a8d24a f390 pause 
0x140a8d24c 498b4008 mov rax, qword ptr [r8 + 8]
0x140a8d250 4883f8ff cmp rax, -1
0x140a8d254 74f4 je 0x140a8d24a
0x140a8d256 4885c0 test rax, rax
0x140a8d259 0f88bc0d0000 js 0x140a8e01b
0x140a8d25f 488d4801 lea rcx, [rax + 1]
0x140a8d263 f0490fb14808 lock cmpxchg qword ptr [r8 + 8], rcx
0x140a8d269 75e5 jne 0x140a8d250
0x140a8d26b 4c8985e00b0000 mov qword ptr [rbp + 0xbe0], r8
0x140a8d272 4c8985f80a0000 mov qword ptr [rbp + 0xaf8], r8
0x140a8d279 488995d80b0000 mov qword ptr [rbp + 0xbd8], rdx
0x140a8d280 488995000b0000 mov qword ptr [rbp + 0xb00], rdx
0x140a8d287 0fb6057b79ba02 movzx eax, byte ptr [rip + 0x2ba797b]
0x140a8d28e b920000000 mov ecx, 0x20
0x140a8d293 ba08000000 mov edx, 8
0x140a8d298 e87369b8ff call 0x140613c10
0x140a8d29d 4885c0 test rax, rax
0x140a8d2a0 0f84d20e0000 je 0x140a8e178
0x140a8d2a6 4989c2 mov r10, rax
0x140a8d2a9 48c70000000000 mov qword ptr [rax], 0
0x140a8d2b0 48c7400804000000 mov qword ptr [rax + 8], 4
0x140a8d2b8 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x140a8d2c0 c6401800 mov byte ptr [rax + 0x18], 0
0x140a8d2c4 488b7f60 mov rdi, qword ptr [rdi + 0x60]
0x140a8d2c8 eb02 jmp 0x140a8d2cc
0x140a8d2ca f390 pause 
0x140a8d2cc 488b4708 mov rax, qword ptr [rdi + 8]
0x140a8d2d0 4883f8ff cmp rax, -1
0x140a8d2d4 74f4 je 0x140a8d2ca
0x140a8d2d6 4885c0 test rax, rax
0x140a8d2d9 0f88540d0000 js 0x140a8e033
0x140a8d2df 488d4801 lea rcx, [rax + 1]
0x140a8d2e3 f0480fb14f08 lock cmpxchg qword ptr [rdi + 8], rcx
0x140a8d2e9 75e5 jne 0x140a8d2d0
0x140a8d2eb 4889bde80a0000 mov qword ptr [rbp + 0xae8], rdi
0x140a8d2f2 4c8995f00a0000 mov qword ptr [rbp + 0xaf0], r10
0x140a8d2f9 450f28ca movaps xmm9, xmm10
0x140a8d2fd 4885f6 test rsi, rsi
0x140a8d300 0f8497000000 je 0x140a8d39d
0x140a8d306 4883fe01 cmp rsi, 1
0x140a8d30a 7508 jne 0x140a8d314
0x140a8d30c 450f28ca movaps xmm9, xmm10
0x140a8d310 31c0 xor eax, eax
0x140a8d312 eb5f jmp 0x140a8d373
0x140a8d314 4889f1 mov rcx, rsi
0x140a8d317 4883e1fe and rcx, 0xfffffffffffffffe
0x140a8d31b 410f28c2 movaps xmm0, xmm10
0x140a8d31f 31c0 xor eax, eax
0x140a8d321 450f28ca movaps xmm9, xmm10
0x140a8d325 f30f100c83 movss xmm1, dword ptr [rbx + rax*4]
0x140a8d32a f30f10548304 movss xmm2, dword ptr [rbx + rax*4 + 4]
0x140a8d330 f3450fc2c903 cmpunordss xmm9, xmm9
0x140a8d336 410f28d9 movaps xmm3, xmm9
0x140a8d33a 0f54d9 andps xmm3, xmm1
0x140a8d33d f30f5dc8 minss xmm1, xmm0
0x140a8d341 440f55c9 andnps xmm9, xmm1
0x140a8d345 440f56cb orps xmm9, xmm3
0x140a8d349 0f28c2 movaps xmm0, xmm2
0x140a8d34c f3410f5dc1 minss xmm0, xmm9
0x140a8d351 f3450fc2c903 cmpunordss xmm9, xmm9
0x140a8d357 410f28c9 movaps xmm1, xmm9
0x140a8d35b 0f55c8 andnps xmm1, xmm0
0x140a8d35e 440f54ca andps xmm9, xmm2
0x140a8d362 440f56c9 orps xmm9, xmm1
0x140a8d366 4883c002 add rax, 2
0x140a8d36a 410f28c1 movaps xmm0, xmm9
0x140a8d36e 4839c1 cmp rcx, rax
0x140a8d371 75b2 jne 0x140a8d325
0x140a8d373 40f6c601 test sil, 1
0x140a8d377 7424 je 0x140a8d39d
0x140a8d379 f30f100483 movss xmm0, dword ptr [rbx + rax*4]
0x140a8d37e 410f28c9 movaps xmm1, xmm9
0x140a8d382 f3410fc2c903 cmpunordss xmm1, xmm9
0x140a8d388 0f28d1 movaps xmm2, xmm1
0x140a8d38b 0f54d0 andps xmm2, xmm0
0x140a8d38e f3410f5dc1 minss xmm0, xmm9
0x140a8d393 0f55c8 andnps xmm1, xmm0
0x140a8d396 0f56ca orps xmm1, xmm2
0x140a8d399 440f28c9 movaps xmm9, xmm1
0x140a8d39d 4885f6 test rsi, rsi
0x140a8d3a0 7478 je 0x140a8d41a
0x140a8d3a2 4889f0 mov rax, rsi
0x140a8d3a5 48ffc8 dec rax
0x140a8d3a8 747d je 0x140a8d427
0x140a8d3aa 4889f2 mov rdx, rsi
0x140a8d3ad 4883e2fe and rdx, 0xfffffffffffffffe
0x140a8d3b1 4c8b8d080d0000 mov r9, qword ptr [rbp + 0xd08]
0x140a8d3b8 4d8d4110 lea r8, [r9 + 0x10]
0x140a8d3bc 410f28c3 movaps xmm0, xmm11
0x140a8d3c0 31c9 xor ecx, ecx
0x140a8d3c2 450f28d3 movaps xmm10, xmm11
0x140a8d3c6 f3410f1048f4 movss xmm1, dword ptr [r8 - 0xc]
0x140a8d3cc f3410f1010 movss xmm2, dword ptr [r8]
0x140a8d3d1 f3450fc2d203 cmpunordss xmm10, xmm10
0x140a8d3d7 410f28da movaps xmm3, xmm10
0x140a8d3db 0f54d9 andps xmm3, xmm1
0x140a8d3de f30f5dc8 minss xmm1, xmm0
0x140a8d3e2 440f55d1 andnps xmm10, xmm1
0x140a8d3e6 440f56d3 orps xmm10, xmm3
0x140a8d3ea 0f28c2 movaps xmm0, xmm2
0x140a8d3ed f3410f5dc2 minss xmm0, xmm10
0x140a8d3f2 f3450fc2d203 cmpunordss xmm10, xmm10
0x140a8d3f8 410f28ca movaps xmm1, xmm10
0x140a8d3fc 0f55c8 andnps xmm1, xmm0
0x140a8d3ff 440f54d2 andps xmm10, xmm2
0x140a8d403 440f56d1 orps xmm10, xmm1
0x140a8d407 4883c102 add rcx, 2
0x140a8d40b 4983c018 add r8, 0x18
0x140a8d40f 410f28c2 movaps xmm0, xmm10
0x140a8d413 4839ca cmp rdx, rcx
0x140a8d416 75ae jne 0x140a8d3c6
0x140a8d418 eb1a jmp 0x140a8d434
0x140a8d41a 450f57ed xorps xmm13, xmm13
0x140a8d41e 450f28d3 movaps xmm10, xmm11
0x140a8d422 e9de000000 jmp 0x140a8d505
0x140a8d427 450f28d3 movaps xmm10, xmm11
0x140a8d42b 31c9 xor ecx, ecx
0x140a8d42d 4c8b8d080d0000 mov r9, qword ptr [rbp + 0xd08]
0x140a8d434 40f6c601 test sil, 1
0x140a8d438 742a je 0x140a8d464
0x140a8d43a 488d0c49 lea rcx, [rcx + rcx*2]
0x140a8d43e f3410f10448904 movss xmm0, dword ptr [r9 + rcx*4 + 4]
0x140a8d445 410f28ca movaps xmm1, xmm10
0x140a8d449 f3410fc2ca03 cmpunordss xmm1, xmm10
0x140a8d44f 0f28d1 movaps xmm2, xmm1
0x140a8d452 0f54d0 andps xmm2, xmm0
0x140a8d455 f3410f5dc2 minss xmm0, xmm10
0x140a8d45a 0f55c8 andnps xmm1, xmm0
0x140a8d45d 0f56ca orps xmm1, xmm2
0x140a8d460 440f28d1 movaps xmm10, xmm1
0x140a8d464 4885c0 test rax, rax
0x140a8d467 7466 je 0x140a8d4cf
0x140a8d469 4889f1 mov rcx, rsi
0x140a8d46c 4883e1fe and rcx, 0xfffffffffffffffe
0x140a8d470 498d5110 lea rdx, [r9 + 0x10]
0x140a8d474 0f57c0 xorps xmm0, xmm0
0x140a8d477 31c0 xor eax, eax
0x140a8d479 440f28e8 movaps xmm13, xmm0
0x140a8d47d f30f104af4 movss xmm1, dword ptr [rdx - 0xc]
0x140a8d482 f30f1012 movss xmm2, dword ptr [rdx]
0x140a8d486 f3450fc2ed03 cmpunordss xmm13, xmm13
0x140a8d48c 410f28dd movaps xmm3, xmm13
0x140a8d490 0f54d9 andps xmm3, xmm1
0x140a8d493 f30f5fc8 maxss xmm1, xmm0
0x140a8d497 440f55e9 andnps xmm13, xmm1
0x140a8d49b 440f56eb orps xmm13, xmm3
0x140a8d49f 0f28c2 movaps xmm0, xmm2
0x140a8d4a2 f3410f5fc5 maxss xmm0, xmm13
0x140a8d4a7 f3450fc2ed03 cmpunordss xmm13, xmm13
0x140a8d4ad 410f28cd movaps xmm1, xmm13
0x140a8d4b1 0f55c8 andnps xmm1, xmm0
0x140a8d4b4 440f54ea andps xmm13, xmm2
0x140a8d4b8 440f56e9 orps xmm13, xmm1
0x140a8d4bc 4883c002 add rax, 2
0x140a8d4c0 4883c218 add rdx, 0x18
0x140a8d4c4 410f28c5 movaps xmm0, xmm13
0x140a8d4c8 4839c1 cmp rcx, rax
0x140a8d4cb 75b0 jne 0x140a8d47d
0x140a8d4cd eb06 jmp 0x140a8d4d5
0x140a8d4cf 450f57ed xorps xmm13, xmm13
0x140a8d4d3 31c0 xor eax, eax
0x140a8d4d5 40f6c601 test sil, 1
0x140a8d4d9 742a je 0x140a8d505
0x140a8d4db 488d0440 lea rax, [rax + rax*2]
0x140a8d4df f3410f10448104 movss xmm0, dword ptr [r9 + rax*4 + 4]
0x140a8d4e6 410f28cd movaps xmm1, xmm13
0x140a8d4ea f3410fc2cd03 cmpunordss xmm1, xmm13
0x140a8d4f0 0f28d1 movaps xmm2, xmm1
0x140a8d4f3 0f54d0 andps xmm2, xmm0
0x140a8d4f6 f3410f5fc5 maxss xmm0, xmm13
0x140a8d4fb 0f55c8 andnps xmm1, xmm0
0x140a8d4fe 0f56ca orps xmm1, xmm2
0x140a8d501 440f28e9 movaps xmm13, xmm1
0x140a8d505 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8d50c 488b40a0 mov rax, qword ptr [rax - 0x60]
0x140a8d510 488d34c500000000 lea rsi, [rax*8]
0x140a8d518 488985080d0000 mov qword ptr [rbp + 0xd08], rax
0x140a8d51f 48c1e83d shr rax, 0x3d
0x140a8d523 0f85a00c0000 jne 0x140a8e1c9
0x140a8d529 b900000000 mov ecx, 0
0x140a8d52e 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
0x140a8d538 4839c6 cmp rsi, rax
0x140a8d53b 0f878a0c0000 ja 0x140a8e1cb
0x140a8d541 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8d548 488b5898 mov rbx, qword ptr [rax - 0x68]
0x140a8d54c 4885f6 test rsi, rsi
0x140a8d54f 4c8995e80b0000 mov qword ptr [rbp + 0xbe8], r10
0x140a8d556 7432 je 0x140a8d58a
0x140a8d558 0fb605aa76ba02 movzx eax, byte ptr [rip + 0x2ba76aa]
0x140a8d55f ba04000000 mov edx, 4
0x140a8d564 4889f1 mov rcx, rsi
0x140a8d567 e8a466b8ff call 0x140613c10
0x140a8d56c b904000000 mov ecx, 4
0x140a8d571 4885c0 test rax, rax
0x140a8d574 0f84510c0000 je 0x140a8e1cb
0x140a8d57a 488b8d080d0000 mov rcx, qword ptr [rbp + 0xd08]
0x140a8d581 48898da00c0000 mov qword ptr [rbp + 0xca0], rcx
0x140a8d588 eb10 jmp 0x140a8d59a
0x140a8d58a b804000000 mov eax, 4
0x140a8d58f 48c785a00c000000000000 mov qword ptr [rbp + 0xca0], 0
0x140a8d59a 488985200c0000 mov qword ptr [rbp + 0xc20], rax
0x140a8d5a1 4889c1 mov rcx, rax
0x140a8d5a4 4889da mov rdx, rbx
0x140a8d5a7 4989f0 mov r8, rsi
0x140a8d5aa e85166e901 call 0x142923c00
0x140a8d5af 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8d5b6 488b70b8 mov rsi, qword ptr [rax - 0x48]
0x140a8d5ba 4c8d0cb500000000 lea r9, [rsi*4]
0x140a8d5c2 4889f0 mov rax, rsi
0x140a8d5c5 48c1e83e shr rax, 0x3e
0x140a8d5c9 0f850d0c0000 jne 0x140a8e1dc
0x140a8d5cf b900000000 mov ecx, 0
0x140a8d5d4 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
0x140a8d5de 4939c1 cmp r9, rax
0x140a8d5e1 0f87f70b0000 ja 0x140a8e1de
0x140a8d5e7 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8d5ee 488b58b0 mov rbx, qword ptr [rax - 0x50]
0x140a8d5f2 4d85c9 test r9, r9
0x140a8d5f5 743f je 0x140a8d636
0x140a8d5f7 0fb6050b76ba02 movzx eax, byte ptr [rip + 0x2ba760b]
0x140a8d5fe ba04000000 mov edx, 4
0x140a8d603 4c89c9 mov rcx, r9
0x140a8d606 4889b5800c0000 mov qword ptr [rbp + 0xc80], rsi
0x140a8d60d 4c89ce mov rsi, r9
0x140a8d610 e8fb65b8ff call 0x140613c10
0x140a8d615 b904000000 mov ecx, 4
0x140a8d61a 4989f1 mov r9, rsi
0x140a8d61d 488bb5800c0000 mov rsi, qword ptr [rbp + 0xc80]
0x140a8d624 4885c0 test rax, rax
0x140a8d627 0f84b10b0000 je 0x140a8e1de
0x140a8d62d 4889b5a80c0000 mov qword ptr [rbp + 0xca8], rsi
0x140a8d634 eb10 jmp 0x140a8d646
0x140a8d636 b804000000 mov eax, 4
0x140a8d63b 48c785a80c000000000000 mov qword ptr [rbp + 0xca8], 0
0x140a8d646 488985800c0000 mov qword ptr [rbp + 0xc80], rax
0x140a8d64d 4889c1 mov rcx, rax
0x140a8d650 4889da mov rdx, rbx
0x140a8d653 4d89c8 mov r8, r9
0x140a8d656 e8a565e901 call 0x142923c00
0x140a8d65b 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
0x140a8d662 f30f1040c0 movss xmm0, dword ptr [rax - 0x40]
0x140a8d667 488b95d00c0000 mov rdx, qword ptr [rbp + 0xcd0]
0x140a8d66e 8d4afe lea ecx, [rdx - 2]
0x140a8d671 83f902 cmp ecx, 2
0x140a8d674 b802000000 mov eax, 2
0x140a8d679 0f43c8 cmovae ecx, eax
0x140a8d67c 85c9 test ecx, ecx
0x140a8d67e 742b je 0x140a8d6ab
0x140a8d680 83f902 cmp ecx, 2
0x140a8d683 7521 jne 0x140a8d6a6
0x140a8d685 488b8d400d0000 mov rcx, qword ptr [rbp + 0xd40]
0x140a8d68c 488b41dc mov rax, qword ptr [rcx - 0x24]
0x140a8d690 488985500c0000 mov qword ptr [rbp + 0xc50], rax
0x140a8d697 0f1049cc movups xmm1, xmmword ptr [rcx - 0x34]
0x140a8d69b 0f298d400c0000 movaps xmmword ptr [rbp + 0xc40], xmm1
0x140a8d6a2 89d0 mov eax, edx
0x140a8d6a4 eb05 jmp 0x140a8d6ab
0x140a8d6a6 b803000000 mov eax, 3
0x140a8d6ab 80bdc80c000000 cmp byte ptr [rbp + 0xcc8], 0
0x140a8d6b2 0f8496000000 je 0x140a8d74e
0x140a8d6b8 8b8d880c0000 mov ecx, dword ptr [rbp + 0xc88]
0x140a8d6be 8b958b0c0000 mov edx, dword ptr [rbp + 0xc8b]
0x140a8d6c4 8995b30c0000 mov dword ptr [rbp + 0xcb3], edx
0x140a8d6ca 898db00c0000 mov dword ptr [rbp + 0xcb0], ecx
0x140a8d6d0 0f288da0070000 movaps xmm1, xmmword ptr [rbp + 0x7a0]
0x140a8d6d7 0f298df00b0000 movaps xmmword ptr [rbp + 0xbf0], xmm1
0x140a8d6de 0f288dc0070000 movaps xmm1, xmmword ptr [rbp + 0x7c0]
0x140a8d6e5 0f2895d0070000 movaps xmm2, xmmword ptr [rbp + 0x7d0]
0x140a8d6ec 0f289de0070000 movaps xmm3, xmmword ptr [rbp + 0x7e0]
0x140a8d6f3 0f298d40080000 movaps xmmword ptr [rbp + 0x840], xmm1
0x140a8d6fa 0f299550080000 movaps xmmword ptr [rbp + 0x850], xmm2
0x140a8d701 0f299d60080000 movaps xmmword ptr [rbp + 0x860], xmm3
0x140a8d708 488b8df0070000 mov rcx, qword ptr [rbp + 0x7f0]
0x140a8d70f 48898d70080000 mov qword ptr [rbp + 0x870], rcx
0x140a8d716 0fb68d2f0d0000 movzx ecx, byte ptr [rbp + 0xd2f]
0x140a8d71d 48898dd80c0000 mov qword ptr [rbp + 0xcd8], rcx
0x140a8d724 488b8d300c0000 mov rcx, qword ptr [rbp + 0xc30]
0x140a8d72b 48898d280c0000 mov qword ptr [rbp + 0xc28], rcx
0x140a8d732 488b8d380c0000 mov rcx, qword ptr [rbp + 0xc38]
0x140a8d739 48898dd00c0000 mov qword ptr [rbp + 0xcd0], rcx
0x140a8d740 8b8d6c0c0000 mov ecx, dword ptr [rbp + 0xc6c]
0x140a8d746 898dc80c0000 mov dword ptr [rbp + 0xcc8], ecx
0x140a8d74c eb56 jmp 0x140a8d7a4
0x140a8d74e 0f57c9 xorps xmm1, xmm1
0x140a8d751 0f298d60080000 movaps xmmword ptr [rbp + 0x860], xmm1
0x140a8d758 0f298d50080000 movaps xmmword ptr [rbp + 0x850], xmm1
0x140a8d75f 0f298d40080000 movaps xmmword ptr [rbp + 0x840], xmm1
0x140a8d766 48c7857008000000000000 mov qword ptr [rbp + 0x870], 0
0x140a8d771 0f298df00b0000 movaps xmmword ptr [rbp + 0xbf0], xmm1
0x140a8d778 48c785d80c000000000000 mov qword ptr [rbp + 0xcd8], 0
0x140a8d783 b904000000 mov ecx, 4
0x140a8d788 48898dd00c0000 mov qword ptr [rbp + 0xcd0], rcx
0x140a8d78f 48c785280c000000000000 mov qword ptr [rbp + 0xc28], 0
0x140a8d79a c785c80c000000000000 mov dword ptr [rbp + 0xcc8], 0
0x140a8d7a4 450f28c7 movaps xmm8, xmm15
0x140a8d7a8 410f28df movaps xmm3, xmm15
0x140a8d7ac 80bd980c000000 cmp byte ptr [rbp + 0xc98], 0
0x140a8d7b3 7503 jne 0x140a8d7b8
0x140a8d7b5 0f28de movaps xmm3, xmm6
0x140a8d7b8 440f28fe movaps xmm15, xmm6
0x140a8d7bc 410f28f3 movaps xmm6, xmm11
0x140a8d7c0 410f28d6 movaps xmm2, xmm14
0x140a8d7c4 7508 jne 0x140a8d7ce
0x140a8d7c6 f30f10153206ff01 movss xmm2, dword ptr [rip + 0x1ff0632]
0x140a8d7ce 4883bd880b000000 cmp qword ptr [rbp + 0xb88], 0
0x140a8d7d6 0f57c9 xorps xmm1, xmm1
0x140a8d7d9 4c8d8508070000 lea r8, [rbp + 0x708]
0x140a8d7e0 410f114830 movups xmmword ptr [r8 + 0x30], xmm1
0x140a8d7e5 410f114820 movups xmmword ptr [r8 + 0x20], xmm1
0x140a8d7ea 410f114810 movups xmmword ptr [r8 + 0x10], xmm1
0x140a8d7ef 410f1108 movups xmmword ptr [r8], xmm1
0x140a8d7f3 49c7404000000000 mov qword ptr [r8 + 0x40], 0
0x140a8d7fb 488b8d180d0000 mov rcx, qword ptr [rbp + 0xd18]
0x140a8d802 48898d70050000 mov qword ptr [rbp + 0x570], rcx
0x140a8d809 488b95380d0000 mov rdx, qword ptr [rbp + 0xd38]
0x140a8d810 48899578050000 mov qword ptr [rbp + 0x578], rdx
0x140a8d817 48898d80050000 mov qword ptr [rbp + 0x580], rcx
0x140a8d81e 4c8b9d400d0000 mov r11, qword ptr [rbp + 0xd40]
0x140a8d825 f3410f1063e8 movss xmm4, dword ptr [r11 - 0x18]
0x140a8d82b f3410f106bec movss xmm5, dword ptr [r11 - 0x14]
0x140a8d831 66410f6e4bf0 movd xmm1, dword ptr [r11 - 0x10]
0x140a8d837 410fb64bf4 movzx ecx, byte ptr [r11 - 0xc]
0x140a8d83c 410fb653f6 movzx edx, byte ptr [r11 - 0xa]
0x140a8d841 450fb653f7 movzx r10d, byte ptr [r11 - 9]
0x140a8d846 450fb75bf8 movzx r11d, word ptr [r11 - 8]
0x140a8d84b 4c8b8da00c0000 mov r9, qword ptr [rbp + 0xca0]
0x140a8d852 4c898d00050000 mov qword ptr [rbp + 0x500], r9
0x140a8d859 4c8b8d200c0000 mov r9, qword ptr [rbp + 0xc20]
0x140a8d860 4c898d08050000 mov qword ptr [rbp + 0x508], r9
0x140a8d867 4c8b8d080d0000 mov r9, qword ptr [rbp + 0xd08]
0x140a8d86e 4c898d10050000 mov qword ptr [rbp + 0x510], r9
0x140a8d875 4c8b8da80c0000 mov r9, qword ptr [rbp + 0xca8]
0x140a8d87c 4c898d18050000 mov qword ptr [rbp + 0x518], r9
0x140a8d883 4c8b8d800c0000 mov r9, qword ptr [rbp + 0xc80]
0x140a8d88a 4c898d20050000 mov qword ptr [rbp + 0x520], r9
0x140a8d891 4889b528050000 mov qword ptr [rbp + 0x528], rsi
0x140a8d898 660f7ebd30050000 movd dword ptr [rbp + 0x530], xmm7
0x140a8d8a0 488bb590020000 mov rsi, qword ptr [rbp + 0x290]
0x140a8d8a7 4989b060feffff mov qword ptr [r8 - 0x1a0], rsi
0x140a8d8ae f3410f5fdc maxss xmm3, xmm12
0x140a8d8b3 f30f5dd3 minss xmm2, xmm3
0x140a8d8b7 0f109d60020000 movups xmm3, xmmword ptr [rbp + 0x260]
0x140a8d8be 440f109d70020000 movups xmm11, xmmword ptr [rbp + 0x270]
0x140a8d8c6 440f10a580020000 movups xmm12, xmmword ptr [rbp + 0x280]
0x140a8d8ce 450f11a050feffff movups xmmword ptr [r8 - 0x1b0], xmm12
0x140a8d8d6 450f119840feffff movups xmmword ptr [r8 - 0x1c0], xmm11
0x140a8d8de 410f119830feffff movups xmmword ptr [r8 - 0x1d0], xmm3
0x140a8d8e6 f3440f119574070000 movss dword ptr [rbp + 0x774], xmm10
0x140a8d8ef 0f57db xorps xmm3, xmm3
0x140a8d8f2 f3440f11ad78070000 movss dword ptr [rbp + 0x778], xmm13
0x140a8d8fb f3440f118d7c070000 movss dword ptr [rbp + 0x77c], xmm9
0x140a8d904 f30f11a580070000 movss dword ptr [rbp + 0x780], xmm4
0x140a8d90c f30f119584070000 movss dword ptr [rbp + 0x784], xmm2
0x140a8d914 f30f11ad88070000 movss dword ptr [rbp + 0x788], xmm5
0x140a8d91c 888d94070000 mov byte ptr [rbp + 0x794], cl
0x140a8d922 889595070000 mov byte ptr [rbp + 0x795], dl
0x140a8d928 48c7858805000000000000 mov qword ptr [rbp + 0x588], 0
0x140a8d933 48c7859005000004000000 mov qword ptr [rbp + 0x590], 4
0x140a8d93e 410f119890feffff movups xmmword ptr [r8 - 0x170], xmm3
0x140a8d946 48c785a805000004000000 mov qword ptr [rbp + 0x5a8], 4
0x140a8d951 48c785b005000000000000 mov qword ptr [rbp + 0x5b0], 0
0x140a8d95c c785b805000000000000 mov dword ptr [rbp + 0x5b8], 0
0x140a8d966 48c785c005000000000000 mov qword ptr [rbp + 0x5c0], 0
0x140a8d971 48c785c805000004000000 mov qword ptr [rbp + 0x5c8], 4
0x140a8d97c 48c785d005000000000000 mov qword ptr [rbp + 0x5d0], 0
0x140a8d987 f30f11858c070000 movss dword ptr [rbp + 0x78c], xmm0
0x140a8d98f 898558070000 mov dword ptr [rbp + 0x758], eax
0x140a8d995 488b85500c0000 mov rax, qword ptr [rbp + 0xc50]
0x140a8d99c 49894064 mov qword ptr [r8 + 0x64], rax
0x140a8d9a0 0f2885400c0000 movaps xmm0, xmmword ptr [rbp + 0xc40]
0x140a8d9a7 410f114054 movups xmmword ptr [r8 + 0x54], xmm0
0x140a8d9ac 488b85780c0000 mov rax, qword ptr [rbp + 0xc78]
0x140a8d9b3 48898518060000 mov qword ptr [rbp + 0x618], rax
0x140a8d9ba 488b85900b0000 mov rax, qword ptr [rbp + 0xb90]
0x140a8d9c1 48898520060000 mov qword ptr [rbp + 0x620], rax
0x140a8d9c8 488b85d00b0000 mov rax, qword ptr [rbp + 0xbd0]
0x140a8d9cf 48898528060000 mov qword ptr [rbp + 0x628], rax
0x140a8d9d6 488b85c80b0000 mov rax, qword ptr [rbp + 0xbc8]
0x140a8d9dd 48898530060000 mov qword ptr [rbp + 0x630], rax
0x140a8d9e4 488b85e00b0000 mov rax, qword ptr [rbp + 0xbe0]
0x140a8d9eb 48898538060000 mov qword ptr [rbp + 0x638], rax
0x140a8d9f2 488b85d80b0000 mov rax, qword ptr [rbp + 0xbd8]
0x140a8d9f9 48898540060000 mov qword ptr [rbp + 0x640], rax
0x140a8da00 488b85980b0000 mov rax, qword ptr [rbp + 0xb98]
0x140a8da07 48898548060000 mov qword ptr [rbp + 0x648], rax
0x140a8da0e 488b85c00b0000 mov rax, qword ptr [rbp + 0xbc0]
0x140a8da15 48898550060000 mov qword ptr [rbp + 0x650], rax
0x140a8da1c 488b85a80b0000 mov rax, qword ptr [rbp + 0xba8]
0x140a8da23 48898558060000 mov qword ptr [rbp + 0x658], rax
0x140a8da2a 488b85a00b0000 mov rax, qword ptr [rbp + 0xba0]
0x140a8da31 48898560060000 mov qword ptr [rbp + 0x660], rax
0x140a8da38 488b85b80b0000 mov rax, qword ptr [rbp + 0xbb8]
0x140a8da3f 48898568060000 mov qword ptr [rbp + 0x668], rax
0x140a8da46 488b85b00b0000 mov rax, qword ptr [rbp + 0xbb0]
0x140a8da4d 48898570060000 mov qword ptr [rbp + 0x670], rax
0x140a8da54 4889bd78060000 mov qword ptr [rbp + 0x678], rdi
0x140a8da5b 488b85e80b0000 mov rax, qword ptr [rbp + 0xbe8]
0x140a8da62 48898580060000 mov qword ptr [rbp + 0x680], rax
0x140a8da69 48c785d805000000000000 mov qword ptr [rbp + 0x5d8], 0
0x140a8da74 48c785e005000004000000 mov qword ptr [rbp + 0x5e0], 4
0x140a8da7f 410f1198e0feffff movups xmmword ptr [r8 - 0x120], xmm3
0x140a8da87 48c785f805000004000000 mov qword ptr [rbp + 0x5f8], 4
0x140a8da92 48c7850006000000000000 mov qword ptr [rbp + 0x600], 0
0x140a8da9d 488b85d80c0000 mov rax, qword ptr [rbp + 0xcd8]
0x140a8daa4 888588060000 mov byte ptr [rbp + 0x688], al
0x140a8daaa 8b85b30c0000 mov eax, dword ptr [rbp + 0xcb3]
0x140a8dab0 41894084 mov dword ptr [r8 - 0x7c], eax
0x140a8dab4 8b85b00c0000 mov eax, dword ptr [rbp + 0xcb0]
0x140a8daba 41894081 mov dword ptr [r8 - 0x7f], eax
0x140a8dabe 488b85280c0000 mov rax, qword ptr [rbp + 0xc28]
0x140a8dac5 48898590060000 mov qword ptr [rbp + 0x690], rax
0x140a8dacc 488b85d00c0000 mov rax, qword ptr [rbp + 0xcd0]
0x140a8dad3 48898598060000 mov qword ptr [rbp + 0x698], rax
0x140a8dada 0f2885f00b0000 movaps xmm0, xmmword ptr [rbp + 0xbf0]
0x140a8dae1 410f114098 movups xmmword ptr [r8 - 0x68], xmm0
0x140a8dae6 488b8570080000 mov rax, qword ptr [rbp + 0x870]
0x140a8daed 498940d8 mov qword ptr [r8 - 0x28], rax
0x140a8daf1 660f6f8540080000 movdqa xmm0, xmmword ptr [rbp + 0x840]
0x140a8daf9 660f6f9550080000 movdqa xmm2, xmmword ptr [rbp + 0x850]
0x140a8db01 0f289d60080000 movaps xmm3, xmmword ptr [rbp + 0x860]
0x140a8db08 410f1158c8 movups xmmword ptr [r8 - 0x38], xmm3
0x140a8db0d f3410f7f50b8 movdqu xmmword ptr [r8 - 0x48], xmm2
0x140a8db13 f3410f7f40a8 movdqu xmmword ptr [r8 - 0x58], xmm0
0x140a8db19 8b85c80c0000 mov eax, dword ptr [rbp + 0xcc8]
0x140a8db1f 8985e8060000 mov dword ptr [rbp + 0x6e8], eax
0x140a8db25 8b85640c0000 mov eax, dword ptr [rbp + 0xc64]
0x140a8db2b 8985ec060000 mov dword ptr [rbp + 0x6ec], eax
0x140a8db31 c685f006000000 mov byte ptr [rbp + 0x6f0], 0
0x140a8db38 48c785f806000000000000 mov qword ptr [rbp + 0x6f8], 0
0x140a8db43 48c7850007000004000000 mov qword ptr [rbp + 0x700], 4
0x140a8db4e c7855007000000000000 mov dword ptr [rbp + 0x750], 0
0x140a8db58 c7850806000000000000 mov dword ptr [rbp + 0x608], 0
0x140a8db62 c7851006000000000000 mov dword ptr [rbp + 0x610], 0
0x140a8db6c 6644899d99070000 mov word ptr [rbp + 0x799], r11w
0x140a8db74 0fb685980c0000 movzx eax, byte ptr [rbp + 0xc98]
0x140a8db7b 888596070000 mov byte ptr [rbp + 0x796], al
0x140a8db81 0f958597070000 setne byte ptr [rbp + 0x797]
0x140a8db88 660f7e8d90070000 movd dword ptr [rbp + 0x790], xmm1
0x140a8db90 44889598070000 mov byte ptr [rbp + 0x798], r10b
0x140a8db97 488b85680b0000 mov rax, qword ptr [rbp + 0xb68]
0x140a8db9e 80b8f400000001 cmp byte ptr [rax + 0xf4], 1
0x140a8dba5 0f859c000000 jne 0x140a8dc47
0x140a8dbab 488b9d48050000 mov rbx, qword ptr [rbp + 0x548]
0x140a8dbb2 4889d8 mov rax, rbx
0x140a8dbb5 b90c000000 mov ecx, 0xc
0x140a8dbba 48f7e1 mul rcx
0x140a8dbbd 4989c1 mov r9, rax
0x140a8dbc0 488bbdf80c0000 mov rdi, qword ptr [rbp + 0xcf8]
0x140a8dbc7 0f8022060000 jo 0x140a8e1ef
0x140a8dbcd 440f28de movaps xmm11, xmm6
0x140a8dbd1 b900000000 mov ecx, 0
0x140a8dbd6 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
0x140a8dbe0 4939c1 cmp r9, rax
0x140a8dbe3 410f28f7 movaps xmm6, xmm15
0x140a8dbe7 f3440f1015106aea01 movss xmm10, dword ptr [rip + 0x1ea6a10]
0x140a8dbf0 450f28f8 movaps xmm15, xmm8
0x140a8dbf4 450f57c0 xorps xmm8, xmm8
0x140a8dbf8 660f76ff pcmpeqd xmm7, xmm7
0x140a8dbfc 0f87ef050000 ja 0x140a8e1f1
0x140a8dc02 488bb540050000 mov rsi, qword ptr [rbp + 0x540]
0x140a8dc09 4d85c9 test r9, r9
0x140a8dc0c 7462 je 0x140a8dc70
0x140a8dc0e 0fb605f46fba02 movzx eax, byte ptr [rip + 0x2ba6ff4]
0x140a8dc15 ba04000000 mov edx, 4
0x140a8dc1a 4c89c9 mov rcx, r9
0x140a8dc1d 4c898d400d0000 mov qword ptr [rbp + 0xd40], r9
0x140a8dc24 e8e75fb8ff call 0x140613c10
0x140a8dc29 b904000000 mov ecx, 4
0x140a8dc2e 4c8b8d400d0000 mov r9, qword ptr [rbp + 0xd40]
0x140a8dc35 4885c0 test rax, rax
0x140a8dc38 0f84b3050000 je 0x140a8e1f1
0x140a8dc3e 48899d380d0000 mov qword ptr [rbp + 0xd38], rbx
0x140a8dc45 eb39 jmp 0x140a8dc80
0x140a8dc47 488bbdf80c0000 mov rdi, qword ptr [rbp + 0xcf8]
0x140a8dc4e 440f28de movaps xmm11, xmm6
0x140a8dc52 410f28f7 movaps xmm6, xmm15
0x140a8dc56 f3440f1015a169ea01 movss xmm10, dword ptr [rip + 0x1ea69a1]
0x140a8dc5f 450f28f8 movaps xmm15, xmm8
0x140a8dc63 450f57c0 xorps xmm8, xmm8
0x140a8dc67 660f76ff pcmpeqd xmm7, xmm7
0x140a8dc6b e996000000 jmp 0x140a8dd06
0x140a8dc70 b804000000 mov eax, 4
0x140a8dc75 48c785380d000000000000 mov qword ptr [rbp + 0xd38], 0
0x140a8dc80 4889c1 mov rcx, rax
0x140a8dc83 4889f2 mov rdx, rsi
0x140a8dc86 4d89c8 mov r8, r9
0x140a8dc89 4889c6 mov rsi, rax
0x140a8dc8c e86f5fe901 call 0x142923c00
0x140a8dc91 8b8d680c0000 mov ecx, dword ptr [rbp + 0xc68]
0x140a8dc97 81e10000ffff and ecx, 0xffff0000
0x140a8dc9d ffc1 inc ecx
0x140a8dc9f 488b85380d0000 mov rax, qword ptr [rbp + 0xd38]
0x140a8dca6 48898540080000 mov qword ptr [rbp + 0x840], rax
0x140a8dcad 4889b548080000 mov qword ptr [rbp + 0x848], rsi
0x140a8dcb4 48899d50080000 mov qword ptr [rbp + 0x850], rbx
0x140a8dcbb 48b80000000000000080 movabs rax, 0x8000000000000000
0x140a8dcc5 48898558080000 mov qword ptr [rbp + 0x858], rax
0x140a8dccc c7857008000001000000 mov dword ptr [rbp + 0x870], 1
0x140a8dcd6 c7858008000000004040 mov dword ptr [rbp + 0x880], 0x40400000
0x140a8dce0 898d680c0000 mov dword ptr [rbp + 0xc68], ecx
0x140a8dce6 898d84080000 mov dword ptr [rbp + 0x884], ecx
0x140a8dcec c685560d000001 mov byte ptr [rbp + 0xd56], 1
0x140a8dcf3 488d8d40080000 lea rcx, [rbp + 0x840]
0x140a8dcfa 488b95780b0000 mov rdx, qword ptr [rbp + 0xb78]
0x140a8dd01 e86a720700 call 0x140b04f70
0x140a8dd06 41b8a0020000 mov r8d, 0x2a0
0x140a8dd0c 488db540080000 lea rsi, [rbp + 0x840]
0x140a8dd13 4889f1 mov rcx, rsi
0x140a8dd16 488d9500050000 lea rdx, [rbp + 0x500]
0x140a8dd1d e8de5ee901 call 0x142923c00
0x140a8dd22 4c8b85b80c0000 mov r8, qword ptr [rbp + 0xcb8]
0x140a8dd29 c685560d000000 mov byte ptr [rbp + 0xd56], 0
0x140a8dd30 488d8d60020000 lea rcx, [rbp + 0x260]
0x140a8dd37 4889fa mov rdx, rdi
0x140a8dd3a 4989f1 mov r9, rsi
0x140a8dd3d e81ec6a900 call 0x14152a360
0x140a8dd42 31c0 xor eax, eax
0x140a8dd44 483b8560020000 cmp rax, qword ptr [rbp + 0x260]
0x140a8dd4b 7013 jo 0x140a8dd60
0x140a8dd4d c685560d000000 mov byte ptr [rbp + 0xd56], 0
0x140a8dd54 488d8d60020000 lea rcx, [rbp + 0x260]
0x140a8dd5b e8a078e0ff call 0x140895600
0x140a8dd60 31c0 xor eax, eax
0x140a8dd62 483b45c0 cmp rax, qword ptr [rbp - 0x40]
0x140a8dd66 7012 jo 0x140a8dd7a
0x140a8dd68 80bd280d000000 cmp byte ptr [rbp + 0xd28], 0
0x140a8dd6f 7409 je 0x140a8dd7a
0x140a8dd71 488d4dc0 lea rcx, [rbp - 0x40]
0x140a8dd75 e88678e0ff call 0x140895600
0x140a8dd7a 4c8b85200d0000 mov r8, qword ptr [rbp + 0xd20]
0x140a8dd81 e98de7ffff jmp 0x140a8c513
0x140a8dd86 488d0d2bb50402 lea rcx, [rip + 0x204b52b]
0x140a8dd8d e87eb5e401 call 0x1428d9310
0x140a8dd92 e970040000 jmp 0x140a8e207
0x140a8dd97 4885c0 test rax, rax
0x140a8dd9a 7513 jne 0x140a8ddaf
0x140a8dd9c 31d2 xor edx, edx
0x140a8dd9e e82daae601 call 0x1428f87d0
0x140a8dda3 4889c1 mov rcx, rax
0x140a8dda6 4885c0 test rax, rax
0x140a8dda9 0f85f7d9ffff jne 0x140a8b7a6
0x140a8ddaf 488d0deacc1a02 lea rcx, [rip + 0x21accea]
0x140a8ddb6 e815a3e401 call 0x1428d80d0
0x140a8ddbb 48837f4800 cmp qword ptr [rdi + 0x48], 0
0x140a8ddc0 743e je 0x140a8de00
0x140a8ddc2 488b7738 mov rsi, qword ptr [rdi + 0x38]
0x140a8ddc6 4885f6 test rsi, rsi
0x140a8ddc9 740f je 0x140a8ddda
0x140a8ddcb 488b4f30 mov rcx, qword ptr [rdi + 0x30]
0x140a8ddcf 4c8d4611 lea r8, [rsi + 0x11]
0x140a8ddd3 b2ff mov dl, 0xff
0x140a8ddd5 e8465ee901 call 0x142923c20
0x140a8ddda 48c7474800000000 mov qword ptr [rdi + 0x48], 0
0x140a8dde2 488d4601 lea rax, [rsi + 1]
0x140a8dde6 4889c1 mov rcx, rax
0x140a8dde9 48c1e903 shr rcx, 3
0x140a8dded 4883e0f8 and rax, 0xfffffffffffffff8
0x140a8ddf1 4829c8 sub rax, rcx
0x140a8ddf4 4883fe08 cmp rsi, 8
0x140a8ddf8 480f42c6 cmovb rax, rsi
0x140a8ddfc 48894740 mov qword ptr [rdi + 0x40], rax
0x140a8de00 48837f7800 cmp qword ptr [rdi + 0x78], 0
0x140a8de05 7444 je 0x140a8de4b
0x140a8de07 488b7768 mov rsi, qword ptr [rdi + 0x68]
0x140a8de0b 4885f6 test rsi, rsi
0x140a8de0e 7415 je 0x140a8de25
0x140a8de10 488b85f00c0000 mov rax, qword ptr [rbp + 0xcf0]
0x140a8de17 488b08 mov rcx, qword ptr [rax]
0x140a8de1a 4c8d4611 lea r8, [rsi + 0x11]
0x140a8de1e b2ff mov dl, 0xff
0x140a8de20 e8fb5de901 call 0x142923c20
0x140a8de25 48c7477800000000 mov qword ptr [rdi + 0x78], 0
0x140a8de2d 488d4601 lea rax, [rsi + 1]
0x140a8de31 4889c1 mov rcx, rax
0x140a8de34 48c1e903 shr rcx, 3
0x140a8de38 4883e0f8 and rax, 0xfffffffffffffff8
0x140a8de3c 4829c8 sub rax, rcx
0x140a8de3f 4883fe08 cmp rsi, 8
0x140a8de43 480f42c6 cmovb rax, rsi
0x140a8de47 48894770 mov qword ptr [rdi + 0x70], rax
0x140a8de4b 4883bfa800000000 cmp qword ptr [rdi + 0xa8], 0
0x140a8de53 744d je 0x140a8dea2
0x140a8de55 488bb798000000 mov rsi, qword ptr [rdi + 0x98]
0x140a8de5c 4885f6 test rsi, rsi
0x140a8de5f 7415 je 0x140a8de76
0x140a8de61 488b85800b0000 mov rax, qword ptr [rbp + 0xb80]
0x140a8de68 488b08 mov rcx, qword ptr [rax]
0x140a8de6b 4c8d4611 lea r8, [rsi + 0x11]
0x140a8de6f b2ff mov dl, 0xff
0x140a8de71 e8aa5de901 call 0x142923c20
0x140a8de76 48c787a800000000000000 mov qword ptr [rdi + 0xa8], 0
0x140a8de81 488d4601 lea rax, [rsi + 1]
0x140a8de85 4889c1 mov rcx, rax
0x140a8de88 48c1e903 shr rcx, 3
0x140a8de8c 4883e0f8 and rax, 0xfffffffffffffff8
0x140a8de90 4829c8 sub rax, rcx
0x140a8de93 4883fe08 cmp rsi, 8
0x140a8de97 480f42c6 cmovb rax, rsi
0x140a8de9b 488987a0000000 mov qword ptr [rdi + 0xa0], rax
0x140a8dea2 4883bd000c000000 cmp qword ptr [rbp + 0xc00], 0
0x140a8deaa 744b je 0x140a8def7
0x140a8deac e87fa21800 call 0x140c18130
0x140a8deb1 4889c1 mov rcx, rax
0x140a8deb4 488b00 mov rax, qword ptr [rax]
0x140a8deb7 4883f801 cmp rax, 1
0x140a8debb 0f85b5000000 jne 0x140a8df76
0x140a8dec1 4883c108 add rcx, 8
0x140a8dec5 48833900 cmp qword ptr [rcx], 0
0x140a8dec9 0f85c8000000 jne 0x140a8df97
0x140a8decf 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140a8ded6 48898d480d0000 mov qword ptr [rbp + 0xd48], rcx
0x140a8dedd 4883c108 add rcx, 8
0x140a8dee1 488b95080c0000 mov rdx, qword ptr [rbp + 0xc08]
0x140a8dee8 e803921800 call 0x140c170f0
0x140a8deed 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8def4 48ff00 inc qword ptr [rax]
0x140a8def7 0f28b5600d0000 movaps xmm6, xmmword ptr [rbp + 0xd60]
0x140a8defe 0f28bd700d0000 movaps xmm7, xmmword ptr [rbp + 0xd70]
0x140a8df05 440f2885800d0000 movaps xmm8, xmmword ptr [rbp + 0xd80]
0x140a8df0d 440f288d900d0000 movaps xmm9, xmmword ptr [rbp + 0xd90]
0x140a8df15 440f2895a00d0000 movaps xmm10, xmmword ptr [rbp + 0xda0]
0x140a8df1d 440f289db00d0000 movaps xmm11, xmmword ptr [rbp + 0xdb0]
0x140a8df25 440f28a5c00d0000 movaps xmm12, xmmword ptr [rbp + 0xdc0]
0x140a8df2d 440f28add00d0000 movaps xmm13, xmmword ptr [rbp + 0xdd0]
0x140a8df35 440f28b5e00d0000 movaps xmm14, xmmword ptr [rbp + 0xde0]
0x140a8df3d 440f28bdf00d0000 movaps xmm15, xmmword ptr [rbp + 0xdf0]
0x140a8df45 4881c4880e0000 add rsp, 0xe88
0x140a8df4c 5b pop rbx
0x140a8df4d 5f pop rdi
0x140a8df4e 5e pop rsi
0x140a8df4f 415c pop r12
0x140a8df51 415d pop r13
0x140a8df53 415e pop r14
0x140a8df55 415f pop r15
0x140a8df57 5d pop rbp
0x140a8df58 c3 ret 
0x140a8df59 488d0d33c20402 lea rcx, [rip + 0x204c233]
0x140a8df60 4c8d05a1b30402 lea r8, [rip + 0x204b3a1]
0x140a8df67 ba32000000 mov edx, 0x32
0x140a8df6c e8bfb4e401 call 0x1428d9430
0x140a8df71 e991020000 jmp 0x140a8e207
0x140a8df76 4885c0 test rax, rax
0x140a8df79 0f8530feffff jne 0x140a8ddaf
0x140a8df7f 31d2 xor edx, edx
0x140a8df81 e84aa8e601 call 0x1428f87d0
0x140a8df86 4889c1 mov rcx, rax
0x140a8df89 4885c0 test rax, rax
0x140a8df8c 0f8533ffffff jne 0x140a8dec5
0x140a8df92 e918feffff jmp 0x140a8ddaf
0x140a8df97 488d0dd2cb1a02 lea rcx, [rip + 0x21acbd2]
0x140a8df9e e8adb2e401 call 0x1428d9250
0x140a8dfa3 488d0d96330602 lea rcx, [rip + 0x2063396]
0x140a8dfaa 488d15ef330602 lea rdx, [rip + 0x20633ef]
0x140a8dfb1 e84abae101 call 0x1428a9a00
0x140a8dfb6 e94c020000 jmp 0x140a8e207
0x140a8dfbb 488d0d7e330602 lea rcx, [rip + 0x206337e]
0x140a8dfc2 488d15d7330602 lea rdx, [rip + 0x20633d7]
0x140a8dfc9 e832bae101 call 0x1428a9a00
0x140a8dfce e934020000 jmp 0x140a8e207
0x140a8dfd3 488d0d66330602 lea rcx, [rip + 0x2063366]
0x140a8dfda 488d15bf330602 lea rdx, [rip + 0x20633bf]
0x140a8dfe1 e81abae101 call 0x1428a9a00
0x140a8dfe6 e91c020000 jmp 0x140a8e207
0x140a8dfeb 488d0d4e330602 lea rcx, [rip + 0x206334e]
0x140a8dff2 488d15a7330602 lea rdx, [rip + 0x20633a7]
0x140a8dff9 e802bae101 call 0x1428a9a00
0x140a8dffe e904020000 jmp 0x140a8e207
0x140a8e003 488d0d36330602 lea rcx, [rip + 0x2063336]
0x140a8e00a 488d158f330602 lea rdx, [rip + 0x206338f]
0x140a8e011 e8eab9e101 call 0x1428a9a00
0x140a8e016 e9ec010000 jmp 0x140a8e207
0x140a8e01b 488d0d1e330602 lea rcx, [rip + 0x206331e]
0x140a8e022 488d1577330602 lea rdx, [rip + 0x2063377]
0x140a8e029 e8d2b9e101 call 0x1428a9a00
0x140a8e02e e9d4010000 jmp 0x140a8e207
0x140a8e033 488d0d06330602 lea rcx, [rip + 0x2063306]
0x140a8e03a 488d155f330602 lea rdx, [rip + 0x206335f]
0x140a8e041 e8bab9e101 call 0x1428a9a00
0x140a8e046 e9bc010000 jmp 0x140a8e207
0x140a8e04b 48c7854008000000000000 mov qword ptr [rbp + 0x840], 0
0x140a8e056 488d0523011a02 lea rax, [rip + 0x21a0123]
0x140a8e05d 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a8e062 488d95400c0000 lea rdx, [rbp + 0xc40]
0x140a8e069 4c8d8d40080000 lea r9, [rbp + 0x840]
0x140a8e070 31c9 xor ecx, ecx
0x140a8e072 4989f8 mov r8, rdi
0x140a8e075 e8c7cae301 call 0x1428cab41
0x140a8e07a e988010000 jmp 0x140a8e207
0x140a8e07f 48c7854008000000000000 mov qword ptr [rbp + 0x840], 0
0x140a8e08a 488d05ef001a02 lea rax, [rip + 0x21a00ef]
0x140a8e091 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a8e096 488d95400c0000 lea rdx, [rbp + 0xc40]
0x140a8e09d 4c8d8d40080000 lea r9, [rbp + 0x840]
0x140a8e0a4 31c9 xor ecx, ecx
0x140a8e0a6 e896cae301 call 0x1428cab41
0x140a8e0ab e957010000 jmp 0x140a8e207
0x140a8e0b0 488d0d19b20402 lea rcx, [rip + 0x204b219]
0x140a8e0b7 4c8d0532b20402 lea r8, [rip + 0x204b232]
0x140a8e0be ba1c000000 mov edx, 0x1c
0x140a8e0c3 e868b3e401 call 0x1428d9430
0x140a8e0c8 e93a010000 jmp 0x140a8e207
0x140a8e0cd 488d0dccc91a02 lea rcx, [rip + 0x21ac9cc]
0x140a8e0d4 e8f79fe401 call 0x1428d80d0
0x140a8e0d9 e929010000 jmp 0x140a8e207
0x140a8e0de 488d0d53b20402 lea rcx, [rip + 0x204b253]
0x140a8e0e5 e826b2e401 call 0x1428d9310
0x140a8e0ea e918010000 jmp 0x140a8e207
0x140a8e0ef 488d0d5ab20402 lea rcx, [rip + 0x204b25a]
0x140a8e0f6 e815b2e401 call 0x1428d9310
0x140a8e0fb e907010000 jmp 0x140a8e207
0x140a8e100 b908000000 mov ecx, 8
0x140a8e105 ba20000000 mov edx, 0x20
0x140a8e10a e8f1aee401 call 0x1428d9000
0x140a8e10f e9f3000000 jmp 0x140a8e207
0x140a8e114 b908000000 mov ecx, 8
0x140a8e119 ba20000000 mov edx, 0x20
0x140a8e11e e8ddaee401 call 0x1428d9000
0x140a8e123 e9df000000 jmp 0x140a8e207
0x140a8e128 b908000000 mov ecx, 8
0x140a8e12d ba20000000 mov edx, 0x20
0x140a8e132 e8c9aee401 call 0x1428d9000
0x140a8e137 e9cb000000 jmp 0x140a8e207
0x140a8e13c b908000000 mov ecx, 8
0x140a8e141 ba20000000 mov edx, 0x20
0x140a8e146 e8b5aee401 call 0x1428d9000
0x140a8e14b e9b7000000 jmp 0x140a8e207
0x140a8e150 b908000000 mov ecx, 8
0x140a8e155 ba20000000 mov edx, 0x20
0x140a8e15a e8a1aee401 call 0x1428d9000
0x140a8e15f e9a3000000 jmp 0x140a8e207
0x140a8e164 b908000000 mov ecx, 8
0x140a8e169 ba20000000 mov edx, 0x20
0x140a8e16e e88daee401 call 0x1428d9000
0x140a8e173 e98f000000 jmp 0x140a8e207
0x140a8e178 b908000000 mov ecx, 8
0x140a8e17d ba20000000 mov edx, 0x20
0x140a8e182 e879aee401 call 0x1428d9000
0x140a8e187 eb7e jmp 0x140a8e207
0x140a8e189 4c8d05a0a90402 lea r8, [rip + 0x204a9a0]
0x140a8e190 b904000000 mov ecx, 4
0x140a8e195 488b95d80c0000 mov rdx, qword ptr [rbp + 0xcd8]
0x140a8e19c e842aee401 call 0x1428d8fe3
0x140a8e1a1 eb64 jmp 0x140a8e207
0x140a8e1a3 31c9 xor ecx, ecx
0x140a8e1a5 4c8d05cca91a02 lea r8, [rip + 0x21aa9cc]
0x140a8e1ac 4c89ca mov rdx, r9
0x140a8e1af e82faee401 call 0x1428d8fe3
0x140a8e1b4 eb51 jmp 0x140a8e207
0x140a8e1b6 31c9 xor ecx, ecx
0x140a8e1b8 4c8d05b9a91a02 lea r8, [rip + 0x21aa9b9]
0x140a8e1bf 4c89ca mov rdx, r9
0x140a8e1c2 e81caee401 call 0x1428d8fe3
0x140a8e1c7 eb3e jmp 0x140a8e207
0x140a8e1c9 31c9 xor ecx, ecx
0x140a8e1cb 4c8d05a6a91a02 lea r8, [rip + 0x21aa9a6]
0x140a8e1d2 4889f2 mov rdx, rsi
0x140a8e1d5 e809aee401 call 0x1428d8fe3
0x140a8e1da eb2b jmp 0x140a8e207
0x140a8e1dc 31c9 xor ecx, ecx
0x140a8e1de 4c8d0593a91a02 lea r8, [rip + 0x21aa993]
0x140a8e1e5 4c89ca mov rdx, r9
0x140a8e1e8 e8f6ade401 call 0x1428d8fe3
0x140a8e1ed eb18 jmp 0x140a8e207
0x140a8e1ef 31c9 xor ecx, ecx
0x140a8e1f1 c685560d000001 mov byte ptr [rbp + 0xd56], 1
0x140a8e1f8 4c8d0579a91a02 lea r8, [rip + 0x21aa979]
0x140a8e1ff 4c89ca mov rdx, r9
0x140a8e202 e8dcade401 call 0x1428d8fe3
0x140a8e207 0f0b ud2 
0x140a8e209 0f1f8000000000 nop dword ptr [rax]
0x140a8e210 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8e215 55 push rbp
0x140a8e216 4157 push r15
0x140a8e218 4156 push r14
0x140a8e21a 4155 push r13
0x140a8e21c 4154 push r12
0x140a8e21e 56 push rsi
0x140a8e21f 57 push rdi
0x140a8e220 53 push rbx
0x140a8e221 4881ecd8000000 sub rsp, 0xd8
0x140a8e228 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8e22f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8e235 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8e23b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8e241 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8e247 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8e24d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8e256 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8e25f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8e268 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a8e270 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x140a8e279 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8e280 48ff00 inc qword ptr [rax]
0x140a8e283 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8e28b 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8e293 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8e29c 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8e2a5 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8e2ae 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8e2b4 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8e2ba 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8e2c0 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8e2c6 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8e2cc 4881c4d8000000 add rsp, 0xd8
0x140a8e2d3 5b pop rbx
0x140a8e2d4 5f pop rdi
0x140a8e2d5 5e pop rsi
0x140a8e2d6 415c pop r12
0x140a8e2d8 415d pop r13
0x140a8e2da 415e pop r14
0x140a8e2dc 415f pop r15
0x140a8e2de 5d pop rbp
0x140a8e2df c3 ret 
0x140a8e2e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8e2e5 55 push rbp
0x140a8e2e6 4157 push r15
0x140a8e2e8 4156 push r14
0x140a8e2ea 4155 push r13
0x140a8e2ec 4154 push r12
0x140a8e2ee 56 push rsi
0x140a8e2ef 57 push rdi
0x140a8e2f0 53 push rbx
0x140a8e2f1 4881ecd8000000 sub rsp, 0xd8
0x140a8e2f8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8e2ff 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8e305 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8e30b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8e311 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8e317 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8e31d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8e326 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8e32f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8e338 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a8e340 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x140a8e349 4883bd000c000000 cmp qword ptr [rbp + 0xc00], 0
0x140a8e351 742f je 0x140a8e382
0x140a8e353 488d85080c0000 lea rax, [rbp + 0xc08]
0x140a8e35a 488985b00c0000 mov qword ptr [rbp + 0xcb0], rax
0x140a8e361 488d85b00c0000 lea rax, [rbp + 0xcb0]
0x140a8e368 488985f00b0000 mov qword ptr [rbp + 0xbf0], rax
0x140a8e36f 488d0d52a90402 lea rcx, [rip + 0x204a952]
0x140a8e376 488d95f00b0000 lea rdx, [rbp + 0xbf0]
0x140a8e37d e84e0f8a00 call 0x14132f2d0
0x140a8e382 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8e38a 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8e392 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8e39b 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8e3a4 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8e3ad 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8e3b3 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8e3b9 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8e3bf 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8e3c5 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8e3cb 4881c4d8000000 add rsp, 0xd8
0x140a8e3d2 5b pop rbx
0x140a8e3d3 5f pop rdi
0x140a8e3d4 5e pop rsi
0x140a8e3d5 415c pop r12
0x140a8e3d7 415d pop r13
0x140a8e3d9 415e pop r14
0x140a8e3db 415f pop r15
0x140a8e3dd 5d pop rbp
0x140a8e3de c3 ret 
0x140a8e3df 90 nop 
0x140a8e3e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8e3e5 55 push rbp
0x140a8e3e6 4157 push r15
0x140a8e3e8 4156 push r14
0x140a8e3ea 4155 push r13
0x140a8e3ec 4154 push r12
0x140a8e3ee 56 push rsi
0x140a8e3ef 57 push rdi
0x140a8e3f0 53 push rbx
0x140a8e3f1 4881ecd8000000 sub rsp, 0xd8
0x140a8e3f8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8e3ff 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8e405 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8e40b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8e411 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8e417 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8e41d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8e426 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8e42f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8e438 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a8e440 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x140a8e449 4883bd300d000000 cmp qword ptr [rbp + 0xd30], 0
0x140a8e451 488b95200d0000 mov rdx, qword ptr [rbp + 0xd20]
0x140a8e458 7418 je 0x140a8e472
0x140a8e45a 4885d2 test rdx, rdx
0x140a8e45d 7413 je 0x140a8e472
0x140a8e45f 488b8d000d0000 mov rcx, qword ptr [rbp + 0xd00]
0x140a8e466 4c8b85100d0000 mov r8, qword ptr [rbp + 0xd10]
0x140a8e46d e8ae57b8ff call 0x140613c20
0x140a8e472 c685550d000000 mov byte ptr [rbp + 0xd55], 0
0x140a8e479 660f6fb424c0000000 movdqa xmm6, xmmword ptr [rsp + 0xc0]
0x140a8e482 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8e48a 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8e493 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8e49c 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8e4a5 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8e4ab 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8e4b1 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8e4b7 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8e4bd 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8e4c3 4881c4d8000000 add rsp, 0xd8
0x140a8e4ca 5b pop rbx
0x140a8e4cb 5f pop rdi
0x140a8e4cc 5e pop rsi
0x140a8e4cd 415c pop r12
0x140a8e4cf 415d pop r13
0x140a8e4d1 415e pop r14
0x140a8e4d3 415f pop r15
0x140a8e4d5 5d pop rbp
0x140a8e4d6 c3 ret 
0x140a8e4d7 660f1f840000000000 nop word ptr [rax + rax]
0x140a8e4e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8e4e5 55 push rbp
0x140a8e4e6 4157 push r15
0x140a8e4e8 4156 push r14
0x140a8e4ea 4155 push r13
0x140a8e4ec 4154 push r12
0x140a8e4ee 56 push rsi
0x140a8e4ef 57 push rdi
0x140a8e4f0 53 push rbx
0x140a8e4f1 4881ecd8000000 sub rsp, 0xd8
0x140a8e4f8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8e4ff 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8e505 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8e50b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8e511 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8e517 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8e51d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8e526 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8e52f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8e538 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a8e540 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x140a8e549 80bd550d000000 cmp byte ptr [rbp + 0xd55], 0
0x140a8e550 740c je 0x140a8e55e
0x140a8e552 488d8d40080000 lea rcx, [rbp + 0x840]
0x140a8e559 e8f2b96bff call 0x140149f50
0x140a8e55e 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8e566 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8e56e 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8e577 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8e580 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8e589 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8e58f 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8e595 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8e59b 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8e5a1 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8e5a7 4881c4d8000000 add rsp, 0xd8
0x140a8e5ae 5b pop rbx
0x140a8e5af 5f pop rdi
0x140a8e5b0 5e pop rsi
0x140a8e5b1 415c pop r12
0x140a8e5b3 415d pop r13
0x140a8e5b5 415e pop r14
0x140a8e5b7 415f pop r15
0x140a8e5b9 5d pop rbp
0x140a8e5ba c3 ret 
0x140a8e5bb 0f1f440000 nop dword ptr [rax + rax]
0x140a8e5c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8e5c5 55 push rbp
0x140a8e5c6 4157 push r15
0x140a8e5c8 4156 push r14
0x140a8e5ca 4155 push r13
0x140a8e5cc 4154 push r12
0x140a8e5ce 56 push rsi
0x140a8e5cf 57 push rdi
0x140a8e5d0 53 push rbx
0x140a8e5d1 4881ecd8000000 sub rsp, 0xd8
0x140a8e5d8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8e5df 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8e5e5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8e5eb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8e5f1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8e5f7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8e5fd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8e606 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8e60f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8e618 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8e621 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8e629 488b85480d0000 mov rax, qword ptr [rbp + 0xd48]
0x140a8e630 48ff00 inc qword ptr [rax]
0x140a8e633 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8e63b 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8e643 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8e64c 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8e655 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8e65e 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8e664 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8e66a 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8e670 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8e676 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8e67c 4881c4d8000000 add rsp, 0xd8
0x140a8e683 5b pop rbx
0x140a8e684 5f pop rdi
0x140a8e685 5e pop rsi
0x140a8e686 415c pop r12
0x140a8e688 415d pop r13
0x140a8e68a 415e pop r14
0x140a8e68c 415f pop r15
0x140a8e68e 5d pop rbp
0x140a8e68f c3 ret 
0x140a8e690 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8e695 55 push rbp
0x140a8e696 4157 push r15
0x140a8e698 4156 push r14
0x140a8e69a 4155 push r13
0x140a8e69c 4154 push r12
0x140a8e69e 56 push rsi
0x140a8e69f 57 push rdi
0x140a8e6a0 53 push rbx
0x140a8e6a1 4881ecd8000000 sub rsp, 0xd8
0x140a8e6a8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8e6af 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8e6b5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8e6bb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8e6c1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8e6c7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8e6cd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8e6d6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8e6df 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8e6e8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8e6f1 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8e6f9 488b9518090000 mov rdx, qword ptr [rbp + 0x918]
0x140a8e700 4885d2 test rdx, rdx
0x140a8e703 0f8596000000 jne 0x140a8e79f
0x140a8e709 488b9530090000 mov rdx, qword ptr [rbp + 0x930]
0x140a8e710 4885d2 test rdx, rdx
0x140a8e713 0f85ac000000 jne 0x140a8e7c5
0x140a8e719 488b95d0090000 mov rdx, qword ptr [rbp + 0x9d0]
0x140a8e720 4885d2 test rdx, rdx
0x140a8e723 7416 je 0x140a8e73b
0x140a8e725 488b8dd8090000 mov rcx, qword ptr [rbp + 0x9d8]
0x140a8e72c 48c1e203 shl rdx, 3
0x140a8e730 41b804000000 mov r8d, 4
0x140a8e736 e8e554b8ff call 0x140613c20
0x140a8e73b c685570d000000 mov byte ptr [rbp + 0xd57], 0
0x140a8e742 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8e74a 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8e752 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8e75b 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8e764 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8e76d 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8e773 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8e779 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8e77f 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8e785 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8e78b 4881c4d8000000 add rsp, 0xd8
0x140a8e792 5b pop rbx
0x140a8e793 5f pop rdi
0x140a8e794 5e pop rsi
0x140a8e795 415c pop r12
0x140a8e797 415d pop r13
0x140a8e799 415e pop r14
0x140a8e79b 415f pop r15
0x140a8e79d 5d pop rbp
0x140a8e79e c3 ret 
0x140a8e79f 488b8d20090000 mov rcx, qword ptr [rbp + 0x920]
0x140a8e7a6 48c1e202 shl rdx, 2
0x140a8e7aa 41b804000000 mov r8d, 4
0x140a8e7b0 e86b54b8ff call 0x140613c20
0x140a8e7b5 488b9530090000 mov rdx, qword ptr [rbp + 0x930]
0x140a8e7bc 4885d2 test rdx, rdx
0x140a8e7bf 0f8454ffffff je 0x140a8e719
0x140a8e7c5 488b8d38090000 mov rcx, qword ptr [rbp + 0x938]
0x140a8e7cc 48c1e202 shl rdx, 2
0x140a8e7d0 41b804000000 mov r8d, 4
0x140a8e7d6 e84554b8ff call 0x140613c20
0x140a8e7db 488b95d0090000 mov rdx, qword ptr [rbp + 0x9d0]
0x140a8e7e2 4885d2 test rdx, rdx
0x140a8e7e5 0f853affffff jne 0x140a8e725
0x140a8e7eb e94bffffff jmp 0x140a8e73b
0x140a8e7f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8e7f5 55 push rbp
0x140a8e7f6 4157 push r15
0x140a8e7f8 4156 push r14
0x140a8e7fa 4155 push r13
0x140a8e7fc 4154 push r12
0x140a8e7fe 56 push rsi
0x140a8e7ff 57 push rdi
0x140a8e800 53 push rbx
0x140a8e801 4881ecd8000000 sub rsp, 0xd8
0x140a8e808 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8e80f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8e815 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8e81b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8e821 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8e827 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8e82d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8e836 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8e83f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8e848 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8e851 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8e859 488d8d40080000 lea rcx, [rbp + 0x840]
0x140a8e860 e89b6de0ff call 0x140895600
0x140a8e865 c685570d000000 mov byte ptr [rbp + 0xd57], 0
0x140a8e86c 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8e874 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8e87c 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8e885 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8e88e 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8e897 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8e89d 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8e8a3 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8e8a9 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8e8af 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8e8b5 4881c4d8000000 add rsp, 0xd8
0x140a8e8bc 5b pop rbx
0x140a8e8bd 5f pop rdi
0x140a8e8be 5e pop rsi
0x140a8e8bf 415c pop r12
0x140a8e8c1 415d pop r13
0x140a8e8c3 415e pop r14
0x140a8e8c5 415f pop r15
0x140a8e8c7 5d pop rbp
0x140a8e8c8 c3 ret 
0x140a8e8c9 0f1f8000000000 nop dword ptr [rax]
0x140a8e8d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8e8d5 55 push rbp
0x140a8e8d6 4157 push r15
0x140a8e8d8 4156 push r14
0x140a8e8da 4155 push r13
0x140a8e8dc 4154 push r12
0x140a8e8de 56 push rsi
0x140a8e8df 57 push rdi
0x140a8e8e0 53 push rbx
0x140a8e8e1 4881ecd8000000 sub rsp, 0xd8
0x140a8e8e8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8e8ef 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8e8f5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8e8fb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8e901 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8e907 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8e90d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8e916 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8e91f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8e928 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8e931 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8e939 4883bd380d000000 cmp qword ptr [rbp + 0xd38], 0
0x140a8e941 741d je 0x140a8e960
0x140a8e943 488b95380d0000 mov rdx, qword ptr [rbp + 0xd38]
0x140a8e94a 48c1e203 shl rdx, 3
0x140a8e94e 41b804000000 mov r8d, 4
0x140a8e954 488b8d180d0000 mov rcx, qword ptr [rbp + 0xd18]
0x140a8e95b e8c052b8ff call 0x140613c20
0x140a8e960 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8e968 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8e971 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8e97a 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8e983 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8e98c 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8e992 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8e998 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8e99e 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8e9a4 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8e9aa 4881c4d8000000 add rsp, 0xd8
0x140a8e9b1 5b pop rbx
0x140a8e9b2 5f pop rdi
0x140a8e9b3 5e pop rsi
0x140a8e9b4 415c pop r12
0x140a8e9b6 415d pop r13
0x140a8e9b8 415e pop r14
0x140a8e9ba 415f pop r15
0x140a8e9bc 5d pop rbp
0x140a8e9bd c3 ret 
0x140a8e9be 6690 nop 
0x140a8e9c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8e9c5 55 push rbp
0x140a8e9c6 4157 push r15
0x140a8e9c8 4156 push r14
0x140a8e9ca 4155 push r13
0x140a8e9cc 4154 push r12
0x140a8e9ce 56 push rsi
0x140a8e9cf 57 push rdi
0x140a8e9d0 53 push rbx
0x140a8e9d1 4881ecd8000000 sub rsp, 0xd8
0x140a8e9d8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8e9df 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8e9e5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8e9eb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8e9f1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8e9f7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8e9fd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8ea06 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8ea0f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8ea18 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8ea21 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8ea29 488d8d40080000 lea rcx, [rbp + 0x840]
0x140a8ea30 e8bb3ab4ff call 0x1405d24f0
0x140a8ea35 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8ea3d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8ea46 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8ea4f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8ea58 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8ea61 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8ea67 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8ea6d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8ea73 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8ea79 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8ea7f 4881c4d8000000 add rsp, 0xd8
0x140a8ea86 5b pop rbx
0x140a8ea87 5f pop rdi
0x140a8ea88 5e pop rsi
0x140a8ea89 415c pop r12
0x140a8ea8b 415d pop r13
0x140a8ea8d 415e pop r14
0x140a8ea8f 415f pop r15
0x140a8ea91 5d pop rbp
0x140a8ea92 c3 ret 
0x140a8ea93 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8eaa0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8eaa5 55 push rbp
0x140a8eaa6 4157 push r15
0x140a8eaa8 4156 push r14
0x140a8eaaa 4155 push r13
0x140a8eaac 4154 push r12
0x140a8eaae 56 push rsi
0x140a8eaaf 57 push rdi
0x140a8eab0 53 push rbx
0x140a8eab1 4881ecd8000000 sub rsp, 0xd8
0x140a8eab8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8eabf 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8eac5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8eacb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8ead1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8ead7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8eadd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8eae6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8eaef 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8eaf8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8eb01 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8eb09 488d8d60020000 lea rcx, [rbp + 0x260]
0x140a8eb10 e80bcf6bff call 0x14014ba20
0x140a8eb15 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8eb1d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8eb26 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8eb2f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8eb38 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8eb41 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8eb47 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8eb4d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8eb53 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8eb59 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8eb5f 4881c4d8000000 add rsp, 0xd8
0x140a8eb66 5b pop rbx
0x140a8eb67 5f pop rdi
0x140a8eb68 5e pop rsi
0x140a8eb69 415c pop r12
0x140a8eb6b 415d pop r13
0x140a8eb6d 415e pop r14
0x140a8eb6f 415f pop r15
0x140a8eb71 5d pop rbp
0x140a8eb72 c3 ret 
0x140a8eb73 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8eb80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8eb85 55 push rbp
0x140a8eb86 4157 push r15
0x140a8eb88 4156 push r14
0x140a8eb8a 4155 push r13
0x140a8eb8c 4154 push r12
0x140a8eb8e 56 push rsi
0x140a8eb8f 57 push rdi
0x140a8eb90 53 push rbx
0x140a8eb91 4881ecd8000000 sub rsp, 0xd8
0x140a8eb98 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8eb9f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8eba5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8ebab 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8ebb1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8ebb7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8ebbd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8ebc6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8ebcf 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8ebd8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8ebe1 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8ebe9 41b804000000 mov r8d, 4
0x140a8ebef 488b8d380d0000 mov rcx, qword ptr [rbp + 0xd38]
0x140a8ebf6 488b95d80c0000 mov rdx, qword ptr [rbp + 0xcd8]
0x140a8ebfd e81e50b8ff call 0x140613c20
0x140a8ec02 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8ec0a 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8ec13 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8ec1c 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8ec25 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8ec2e 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8ec34 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8ec3a 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8ec40 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8ec46 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8ec4c 4881c4d8000000 add rsp, 0xd8
0x140a8ec53 5b pop rbx
0x140a8ec54 5f pop rdi
0x140a8ec55 5e pop rsi
0x140a8ec56 415c pop r12
0x140a8ec58 415d pop r13
0x140a8ec5a 415e pop r14
0x140a8ec5c 415f pop r15
0x140a8ec5e 5d pop rbp
0x140a8ec5f c3 ret 
0x140a8ec60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8ec65 55 push rbp
0x140a8ec66 4157 push r15
0x140a8ec68 4156 push r14
0x140a8ec6a 4155 push r13
0x140a8ec6c 4154 push r12
0x140a8ec6e 56 push rsi
0x140a8ec6f 57 push rdi
0x140a8ec70 53 push rbx
0x140a8ec71 4881ecd8000000 sub rsp, 0xd8
0x140a8ec78 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8ec7f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8ec85 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8ec8b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8ec91 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8ec97 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8ec9d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8eca6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8ecaf 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8ecb8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8ecc1 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8ecc9 4883bd180d000000 cmp qword ptr [rbp + 0xd18], 0
0x140a8ecd1 7419 je 0x140a8ecec
0x140a8ecd3 41b804000000 mov r8d, 4
0x140a8ecd9 488b8d380d0000 mov rcx, qword ptr [rbp + 0xd38]
0x140a8ece0 488b95d80c0000 mov rdx, qword ptr [rbp + 0xcd8]
0x140a8ece7 e8344fb8ff call 0x140613c20
0x140a8ecec 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8ecf4 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8ecfd 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8ed06 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8ed0f 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8ed18 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8ed1e 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8ed24 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8ed2a 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8ed30 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8ed36 4881c4d8000000 add rsp, 0xd8
0x140a8ed3d 5b pop rbx
0x140a8ed3e 5f pop rdi
0x140a8ed3f 5e pop rsi
0x140a8ed40 415c pop r12
0x140a8ed42 415d pop r13
0x140a8ed44 415e pop r14
0x140a8ed46 415f pop r15
0x140a8ed48 5d pop rbp
0x140a8ed49 c3 ret 
0x140a8ed4a 660f1f440000 nop word ptr [rax + rax]
0x140a8ed50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8ed55 55 push rbp
0x140a8ed56 4157 push r15
0x140a8ed58 4156 push r14
0x140a8ed5a 4155 push r13
0x140a8ed5c 4154 push r12
0x140a8ed5e 56 push rsi
0x140a8ed5f 57 push rdi
0x140a8ed60 53 push rbx
0x140a8ed61 4881ecd8000000 sub rsp, 0xd8
0x140a8ed68 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8ed6f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8ed75 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8ed7b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8ed81 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8ed87 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8ed8d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8ed96 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8ed9f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8eda8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8edb1 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8edb9 488d8d480b0000 lea rcx, [rbp + 0xb48]
0x140a8edc0 e8cb05dfff call 0x14087f390
0x140a8edc5 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8edcd 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8edd6 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8eddf 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8ede8 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8edf1 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8edf7 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8edfd 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8ee03 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8ee09 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8ee0f 4881c4d8000000 add rsp, 0xd8
0x140a8ee16 5b pop rbx
0x140a8ee17 5f pop rdi
0x140a8ee18 5e pop rsi
0x140a8ee19 415c pop r12
0x140a8ee1b 415d pop r13
0x140a8ee1d 415e pop r14
0x140a8ee1f 415f pop r15
0x140a8ee21 5d pop rbp
0x140a8ee22 c3 ret 
0x140a8ee23 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8ee30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8ee35 55 push rbp
0x140a8ee36 4157 push r15
0x140a8ee38 4156 push r14
0x140a8ee3a 4155 push r13
0x140a8ee3c 4154 push r12
0x140a8ee3e 56 push rsi
0x140a8ee3f 57 push rdi
0x140a8ee40 53 push rbx
0x140a8ee41 4881ecd8000000 sub rsp, 0xd8
0x140a8ee48 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8ee4f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8ee55 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8ee5b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8ee61 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8ee67 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8ee6d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8ee76 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8ee7f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8ee88 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8ee91 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8ee99 488d8d380b0000 lea rcx, [rbp + 0xb38]
0x140a8eea0 e8eb04dfff call 0x14087f390
0x140a8eea5 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8eead 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8eeb6 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8eebf 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8eec8 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8eed1 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8eed7 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8eedd 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8eee3 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8eee9 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8eeef 4881c4d8000000 add rsp, 0xd8
0x140a8eef6 5b pop rbx
0x140a8eef7 5f pop rdi
0x140a8eef8 5e pop rsi
0x140a8eef9 415c pop r12
0x140a8eefb 415d pop r13
0x140a8eefd 415e pop r14
0x140a8eeff 415f pop r15
0x140a8ef01 5d pop rbp
0x140a8ef02 c3 ret 
0x140a8ef03 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8ef10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8ef15 55 push rbp
0x140a8ef16 4157 push r15
0x140a8ef18 4156 push r14
0x140a8ef1a 4155 push r13
0x140a8ef1c 4154 push r12
0x140a8ef1e 56 push rsi
0x140a8ef1f 57 push rdi
0x140a8ef20 53 push rbx
0x140a8ef21 4881ecd8000000 sub rsp, 0xd8
0x140a8ef28 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8ef2f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8ef35 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8ef3b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8ef41 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8ef47 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8ef4d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8ef56 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8ef5f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8ef68 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8ef71 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8ef79 488d8d280b0000 lea rcx, [rbp + 0xb28]
0x140a8ef80 e80b04dfff call 0x14087f390
0x140a8ef85 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8ef8d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8ef96 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8ef9f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8efa8 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8efb1 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8efb7 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8efbd 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8efc3 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8efc9 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8efcf 4881c4d8000000 add rsp, 0xd8
0x140a8efd6 5b pop rbx
0x140a8efd7 5f pop rdi
0x140a8efd8 5e pop rsi
0x140a8efd9 415c pop r12
0x140a8efdb 415d pop r13
0x140a8efdd 415e pop r14
0x140a8efdf 415f pop r15
0x140a8efe1 5d pop rbp
0x140a8efe2 c3 ret 
0x140a8efe3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8eff0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8eff5 55 push rbp
0x140a8eff6 4157 push r15
0x140a8eff8 4156 push r14
0x140a8effa 4155 push r13
0x140a8effc 4154 push r12
0x140a8effe 56 push rsi
0x140a8efff 57 push rdi
0x140a8f000 53 push rbx
0x140a8f001 4881ecd8000000 sub rsp, 0xd8
0x140a8f008 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f00f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f015 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f01b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f021 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f027 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f02d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f036 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f03f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f048 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8f051 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f059 488d8d180b0000 lea rcx, [rbp + 0xb18]
0x140a8f060 e82b03dfff call 0x14087f390
0x140a8f065 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f06d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f076 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f07f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f088 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f091 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f097 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f09d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f0a3 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f0a9 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f0af 4881c4d8000000 add rsp, 0xd8
0x140a8f0b6 5b pop rbx
0x140a8f0b7 5f pop rdi
0x140a8f0b8 5e pop rsi
0x140a8f0b9 415c pop r12
0x140a8f0bb 415d pop r13
0x140a8f0bd 415e pop r14
0x140a8f0bf 415f pop r15
0x140a8f0c1 5d pop rbp
0x140a8f0c2 c3 ret 
0x140a8f0c3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8f0d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8f0d5 55 push rbp
0x140a8f0d6 4157 push r15
0x140a8f0d8 4156 push r14
0x140a8f0da 4155 push r13
0x140a8f0dc 4154 push r12
0x140a8f0de 56 push rsi
0x140a8f0df 57 push rdi
0x140a8f0e0 53 push rbx
0x140a8f0e1 4881ecd8000000 sub rsp, 0xd8
0x140a8f0e8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f0ef 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f0f5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f0fb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f101 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f107 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f10d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f116 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f11f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f128 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8f131 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f139 488d8d080b0000 lea rcx, [rbp + 0xb08]
0x140a8f140 e84b02dfff call 0x14087f390
0x140a8f145 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f14d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f156 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f15f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f168 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f171 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f177 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f17d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f183 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f189 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f18f 4881c4d8000000 add rsp, 0xd8
0x140a8f196 5b pop rbx
0x140a8f197 5f pop rdi
0x140a8f198 5e pop rsi
0x140a8f199 415c pop r12
0x140a8f19b 415d pop r13
0x140a8f19d 415e pop r14
0x140a8f19f 415f pop r15
0x140a8f1a1 5d pop rbp
0x140a8f1a2 c3 ret 
0x140a8f1a3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8f1b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8f1b5 55 push rbp
0x140a8f1b6 4157 push r15
0x140a8f1b8 4156 push r14
0x140a8f1ba 4155 push r13
0x140a8f1bc 4154 push r12
0x140a8f1be 56 push rsi
0x140a8f1bf 57 push rdi
0x140a8f1c0 53 push rbx
0x140a8f1c1 4881ecd8000000 sub rsp, 0xd8
0x140a8f1c8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f1cf 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f1d5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f1db 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f1e1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f1e7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f1ed 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f1f6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f1ff 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f208 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8f211 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f219 488d8df80a0000 lea rcx, [rbp + 0xaf8]
0x140a8f220 e86b01dfff call 0x14087f390
0x140a8f225 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f22d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f236 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f23f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f248 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f251 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f257 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f25d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f263 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f269 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f26f 4881c4d8000000 add rsp, 0xd8
0x140a8f276 5b pop rbx
0x140a8f277 5f pop rdi
0x140a8f278 5e pop rsi
0x140a8f279 415c pop r12
0x140a8f27b 415d pop r13
0x140a8f27d 415e pop r14
0x140a8f27f 415f pop r15
0x140a8f281 5d pop rbp
0x140a8f282 c3 ret 
0x140a8f283 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8f290 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8f295 55 push rbp
0x140a8f296 4157 push r15
0x140a8f298 4156 push r14
0x140a8f29a 4155 push r13
0x140a8f29c 4154 push r12
0x140a8f29e 56 push rsi
0x140a8f29f 57 push rdi
0x140a8f2a0 53 push rbx
0x140a8f2a1 4881ecd8000000 sub rsp, 0xd8
0x140a8f2a8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f2af 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f2b5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f2bb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f2c1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f2c7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f2cd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f2d6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f2df 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f2e8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8f2f1 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f2f9 488d8de80a0000 lea rcx, [rbp + 0xae8]
0x140a8f300 e88b00dfff call 0x14087f390
0x140a8f305 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f30d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f316 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f31f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f328 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f331 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f337 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f33d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f343 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f349 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f34f 4881c4d8000000 add rsp, 0xd8
0x140a8f356 5b pop rbx
0x140a8f357 5f pop rdi
0x140a8f358 5e pop rsi
0x140a8f359 415c pop r12
0x140a8f35b 415d pop r13
0x140a8f35d 415e pop r14
0x140a8f35f 415f pop r15
0x140a8f361 5d pop rbp
0x140a8f362 c3 ret 
0x140a8f363 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8f370 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8f375 55 push rbp
0x140a8f376 4157 push r15
0x140a8f378 4156 push r14
0x140a8f37a 4155 push r13
0x140a8f37c 4154 push r12
0x140a8f37e 56 push rsi
0x140a8f37f 57 push rdi
0x140a8f380 53 push rbx
0x140a8f381 4881ecd8000000 sub rsp, 0xd8
0x140a8f388 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f38f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f395 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f39b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f3a1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f3a7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f3ad 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f3b6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f3bf 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f3c8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8f3d1 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f3d9 4883bda00c000000 cmp qword ptr [rbp + 0xca0], 0
0x140a8f3e1 741d je 0x140a8f400
0x140a8f3e3 488b95a00c0000 mov rdx, qword ptr [rbp + 0xca0]
0x140a8f3ea 48c1e203 shl rdx, 3
0x140a8f3ee 41b804000000 mov r8d, 4
0x140a8f3f4 488b8d200c0000 mov rcx, qword ptr [rbp + 0xc20]
0x140a8f3fb e82048b8ff call 0x140613c20
0x140a8f400 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f408 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f411 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f41a 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f423 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f42c 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f432 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f438 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f43e 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f444 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f44a 4881c4d8000000 add rsp, 0xd8
0x140a8f451 5b pop rbx
0x140a8f452 5f pop rdi
0x140a8f453 5e pop rsi
0x140a8f454 415c pop r12
0x140a8f456 415d pop r13
0x140a8f458 415e pop r14
0x140a8f45a 415f pop r15
0x140a8f45c 5d pop rbp
0x140a8f45d c3 ret 
0x140a8f45e 6690 nop 
0x140a8f460 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8f465 55 push rbp
0x140a8f466 4157 push r15
0x140a8f468 4156 push r14
0x140a8f46a 4155 push r13
0x140a8f46c 4154 push r12
0x140a8f46e 56 push rsi
0x140a8f46f 57 push rdi
0x140a8f470 53 push rbx
0x140a8f471 4881ecd8000000 sub rsp, 0xd8
0x140a8f478 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f47f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f485 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f48b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f491 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f497 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f49d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f4a6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f4af 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f4b8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8f4c1 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f4c9 488b95300c0000 mov rdx, qword ptr [rbp + 0xc30]
0x140a8f4d0 4885d2 test rdx, rdx
0x140a8f4d3 0f94c0 sete al
0x140a8f4d6 8b8da80c0000 mov ecx, dword ptr [rbp + 0xca8]
0x140a8f4dc 08c1 or cl, al
0x140a8f4de 7516 jne 0x140a8f4f6
0x140a8f4e0 48c1e203 shl rdx, 3
0x140a8f4e4 41b804000000 mov r8d, 4
0x140a8f4ea 488b8d380c0000 mov rcx, qword ptr [rbp + 0xc38]
0x140a8f4f1 e82a47b8ff call 0x140613c20
0x140a8f4f6 8b85280d0000 mov eax, dword ptr [rbp + 0xd28]
0x140a8f4fc 8885570d0000 mov byte ptr [rbp + 0xd57], al
0x140a8f502 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f50a 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f512 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f51b 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f524 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f52d 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f533 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f539 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f53f 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f545 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f54b 4881c4d8000000 add rsp, 0xd8
0x140a8f552 5b pop rbx
0x140a8f553 5f pop rdi
0x140a8f554 5e pop rsi
0x140a8f555 415c pop r12
0x140a8f557 415d pop r13
0x140a8f559 415e pop r14
0x140a8f55b 415f pop r15
0x140a8f55d 5d pop rbp
0x140a8f55e c3 ret 
0x140a8f55f 90 nop 
0x140a8f560 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8f565 55 push rbp
0x140a8f566 4157 push r15
0x140a8f568 4156 push r14
0x140a8f56a 4155 push r13
0x140a8f56c 4154 push r12
0x140a8f56e 56 push rsi
0x140a8f56f 57 push rdi
0x140a8f570 53 push rbx
0x140a8f571 4881ecd8000000 sub rsp, 0xd8
0x140a8f578 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f57f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f585 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f58b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f591 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f597 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f59d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f5a6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f5af 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f5b8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8f5c1 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f5c9 80bd560d000000 cmp byte ptr [rbp + 0xd56], 0
0x140a8f5d0 7475 je 0x140a8f647
0x140a8f5d2 488d8d00050000 lea rcx, [rbp + 0x500]
0x140a8f5d9 e82260e0ff call 0x140895600
0x140a8f5de 8b85280d0000 mov eax, dword ptr [rbp + 0xd28]
0x140a8f5e4 8885570d0000 mov byte ptr [rbp + 0xd57], al
0x140a8f5ea 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f5f2 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f5fa 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f603 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f60c 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f615 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f61b 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f621 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f627 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f62d 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f633 4881c4d8000000 add rsp, 0xd8
0x140a8f63a 5b pop rbx
0x140a8f63b 5f pop rdi
0x140a8f63c 5e pop rsi
0x140a8f63d 415c pop r12
0x140a8f63f 415d pop r13
0x140a8f641 415e pop r14
0x140a8f643 415f pop r15
0x140a8f645 5d pop rbp
0x140a8f646 c3 ret 
0x140a8f647 8b85280d0000 mov eax, dword ptr [rbp + 0xd28]
0x140a8f64d 8885570d0000 mov byte ptr [rbp + 0xd57], al
0x140a8f653 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f65b 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f663 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f66c 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f675 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f67e 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f684 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f68a 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f690 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f696 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f69c 4881c4d8000000 add rsp, 0xd8
0x140a8f6a3 5b pop rbx
0x140a8f6a4 5f pop rdi
0x140a8f6a5 5e pop rsi
0x140a8f6a6 415c pop r12
0x140a8f6a8 415d pop r13
0x140a8f6aa 415e pop r14
0x140a8f6ac 415f pop r15
0x140a8f6ae 5d pop rbp
0x140a8f6af c3 ret 
0x140a8f6b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8f6b5 55 push rbp
0x140a8f6b6 4157 push r15
0x140a8f6b8 4156 push r14
0x140a8f6ba 4155 push r13
0x140a8f6bc 4154 push r12
0x140a8f6be 56 push rsi
0x140a8f6bf 57 push rdi
0x140a8f6c0 53 push rbx
0x140a8f6c1 4881ecd8000000 sub rsp, 0xd8
0x140a8f6c8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f6cf 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f6d5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f6db 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f6e1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f6e7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f6ed 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f6f6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f6ff 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f708 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a8f710 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f718 31c0 xor eax, eax
0x140a8f71a 483b45c0 cmp rax, qword ptr [rbp - 0x40]
0x140a8f71e 7073 jo 0x140a8f793
0x140a8f720 f685570d000001 test byte ptr [rbp + 0xd57], 1
0x140a8f727 0f84c3000000 je 0x140a8f7f0
0x140a8f72d 488d4dc0 lea rcx, [rbp - 0x40]
0x140a8f731 e8ca5ee0ff call 0x140895600
0x140a8f736 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f73e 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f746 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f74f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f758 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f761 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f767 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f76d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f773 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f779 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f77f 4881c4d8000000 add rsp, 0xd8
0x140a8f786 5b pop rbx
0x140a8f787 5f pop rdi
0x140a8f788 5e pop rsi
0x140a8f789 415c pop r12
0x140a8f78b 415d pop r13
0x140a8f78d 415e pop r14
0x140a8f78f 415f pop r15
0x140a8f791 5d pop rbp
0x140a8f792 c3 ret 
0x140a8f793 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f79b 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f7a3 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f7ac 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f7b5 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f7be 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f7c4 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f7ca 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f7d0 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f7d6 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f7dc 4881c4d8000000 add rsp, 0xd8
0x140a8f7e3 5b pop rbx
0x140a8f7e4 5f pop rdi
0x140a8f7e5 5e pop rsi
0x140a8f7e6 415c pop r12
0x140a8f7e8 415d pop r13
0x140a8f7ea 415e pop r14
0x140a8f7ec 415f pop r15
0x140a8f7ee 5d pop rbp
0x140a8f7ef c3 ret 
0x140a8f7f0 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f7f8 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f800 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f809 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f812 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f81b 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f821 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f827 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f82d 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f833 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f839 4881c4d8000000 add rsp, 0xd8
0x140a8f840 5b pop rbx
0x140a8f841 5f pop rdi
0x140a8f842 5e pop rsi
0x140a8f843 415c pop r12
0x140a8f845 415d pop r13
0x140a8f847 415e pop r14
0x140a8f849 415f pop r15
0x140a8f84b 5d pop rbp
0x140a8f84c c3 ret 
0x140a8f84d 0f1f00 nop dword ptr [rax]
0x140a8f850 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8f855 55 push rbp
0x140a8f856 4157 push r15
0x140a8f858 4156 push r14
0x140a8f85a 4155 push r13
0x140a8f85c 4154 push r12
0x140a8f85e 56 push rsi
0x140a8f85f 57 push rdi
0x140a8f860 53 push rbx
0x140a8f861 4881ecd8000000 sub rsp, 0xd8
0x140a8f868 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f86f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f875 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f87b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f881 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f887 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f88d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f896 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f89f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f8a8 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8f8b1 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f8b9 488d8d68090000 lea rcx, [rbp + 0x968]
0x140a8f8c0 e8cbfadeff call 0x14087f390
0x140a8f8c5 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f8cd 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f8d6 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f8df 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f8e8 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f8f1 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f8f7 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f8fd 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f903 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f909 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f90f 4881c4d8000000 add rsp, 0xd8
0x140a8f916 5b pop rbx
0x140a8f917 5f pop rdi
0x140a8f918 5e pop rsi
0x140a8f919 415c pop r12
0x140a8f91b 415d pop r13
0x140a8f91d 415e pop r14
0x140a8f91f 415f pop r15
0x140a8f921 5d pop rbp
0x140a8f922 c3 ret 
0x140a8f923 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8f930 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8f935 55 push rbp
0x140a8f936 4157 push r15
0x140a8f938 4156 push r14
0x140a8f93a 4155 push r13
0x140a8f93c 4154 push r12
0x140a8f93e 56 push rsi
0x140a8f93f 57 push rdi
0x140a8f940 53 push rbx
0x140a8f941 4881ecd8000000 sub rsp, 0xd8
0x140a8f948 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8f94f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8f955 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8f95b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8f961 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8f967 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8f96d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8f976 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8f97f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8f988 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8f991 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8f999 488d8d78090000 lea rcx, [rbp + 0x978]
0x140a8f9a0 e8ebf9deff call 0x14087f390
0x140a8f9a5 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8f9ad 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8f9b6 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8f9bf 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8f9c8 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8f9d1 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8f9d7 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8f9dd 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8f9e3 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8f9e9 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8f9ef 4881c4d8000000 add rsp, 0xd8
0x140a8f9f6 5b pop rbx
0x140a8f9f7 5f pop rdi
0x140a8f9f8 5e pop rsi
0x140a8f9f9 415c pop r12
0x140a8f9fb 415d pop r13
0x140a8f9fd 415e pop r14
0x140a8f9ff 415f pop r15
0x140a8fa01 5d pop rbp
0x140a8fa02 c3 ret 
0x140a8fa03 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8fa10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8fa15 55 push rbp
0x140a8fa16 4157 push r15
0x140a8fa18 4156 push r14
0x140a8fa1a 4155 push r13
0x140a8fa1c 4154 push r12
0x140a8fa1e 56 push rsi
0x140a8fa1f 57 push rdi
0x140a8fa20 53 push rbx
0x140a8fa21 4881ecd8000000 sub rsp, 0xd8
0x140a8fa28 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8fa2f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8fa35 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8fa3b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8fa41 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8fa47 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8fa4d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8fa56 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8fa5f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8fa68 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8fa71 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8fa79 488d8d88090000 lea rcx, [rbp + 0x988]
0x140a8fa80 e80bf9deff call 0x14087f390
0x140a8fa85 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8fa8d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8fa96 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8fa9f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8faa8 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8fab1 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8fab7 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8fabd 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8fac3 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8fac9 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8facf 4881c4d8000000 add rsp, 0xd8
0x140a8fad6 5b pop rbx
0x140a8fad7 5f pop rdi
0x140a8fad8 5e pop rsi
0x140a8fad9 415c pop r12
0x140a8fadb 415d pop r13
0x140a8fadd 415e pop r14
0x140a8fadf 415f pop r15
0x140a8fae1 5d pop rbp
0x140a8fae2 c3 ret 
0x140a8fae3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8faf0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8faf5 55 push rbp
0x140a8faf6 4157 push r15
0x140a8faf8 4156 push r14
0x140a8fafa 4155 push r13
0x140a8fafc 4154 push r12
0x140a8fafe 56 push rsi
0x140a8faff 57 push rdi
0x140a8fb00 53 push rbx
0x140a8fb01 4881ecd8000000 sub rsp, 0xd8
0x140a8fb08 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8fb0f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8fb15 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8fb1b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8fb21 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8fb27 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8fb2d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8fb36 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8fb3f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8fb48 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8fb51 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8fb59 488d8d98090000 lea rcx, [rbp + 0x998]
0x140a8fb60 e82bf8deff call 0x14087f390
0x140a8fb65 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8fb6d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8fb76 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8fb7f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8fb88 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8fb91 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8fb97 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8fb9d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8fba3 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8fba9 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8fbaf 4881c4d8000000 add rsp, 0xd8
0x140a8fbb6 5b pop rbx
0x140a8fbb7 5f pop rdi
0x140a8fbb8 5e pop rsi
0x140a8fbb9 415c pop r12
0x140a8fbbb 415d pop r13
0x140a8fbbd 415e pop r14
0x140a8fbbf 415f pop r15
0x140a8fbc1 5d pop rbp
0x140a8fbc2 c3 ret 
0x140a8fbc3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8fbd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8fbd5 55 push rbp
0x140a8fbd6 4157 push r15
0x140a8fbd8 4156 push r14
0x140a8fbda 4155 push r13
0x140a8fbdc 4154 push r12
0x140a8fbde 56 push rsi
0x140a8fbdf 57 push rdi
0x140a8fbe0 53 push rbx
0x140a8fbe1 4881ecd8000000 sub rsp, 0xd8
0x140a8fbe8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8fbef 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8fbf5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8fbfb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8fc01 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8fc07 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8fc0d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8fc16 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8fc1f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8fc28 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8fc31 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8fc39 488d8da8090000 lea rcx, [rbp + 0x9a8]
0x140a8fc40 e84bf7deff call 0x14087f390
0x140a8fc45 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8fc4d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8fc56 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8fc5f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8fc68 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8fc71 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8fc77 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8fc7d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8fc83 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8fc89 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8fc8f 4881c4d8000000 add rsp, 0xd8
0x140a8fc96 5b pop rbx
0x140a8fc97 5f pop rdi
0x140a8fc98 5e pop rsi
0x140a8fc99 415c pop r12
0x140a8fc9b 415d pop r13
0x140a8fc9d 415e pop r14
0x140a8fc9f 415f pop r15
0x140a8fca1 5d pop rbp
0x140a8fca2 c3 ret 
0x140a8fca3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a8fcb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a8fcb5 55 push rbp
0x140a8fcb6 4157 push r15
0x140a8fcb8 4156 push r14
0x140a8fcba 4155 push r13
0x140a8fcbc 4154 push r12
0x140a8fcbe 56 push rsi
0x140a8fcbf 57 push rdi
0x140a8fcc0 53 push rbx
0x140a8fcc1 4881ecd8000000 sub rsp, 0xd8
0x140a8fcc8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a8fccf 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a8fcd5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a8fcdb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a8fce1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a8fce7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a8fced 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a8fcf6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a8fcff 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a8fd08 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x140a8fd11 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a8fd19 488d8db8090000 lea rcx, [rbp + 0x9b8]
0x140a8fd20 e86bf6deff call 0x14087f390
0x140a8fd25 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a8fd2d 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x140a8fd36 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a8fd3f 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a8fd48 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a8fd51 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a8fd57 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a8fd5d 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a8fd63 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a8fd69 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a8fd6f 4881c4d8000000 add rsp, 0xd8
0x140a8fd76 5b pop rbx
0x140a8fd77 5f pop rdi
0x140a8fd78 5e pop rsi
0x140a8fd79 415c pop r12
0x140a8fd7b 415d pop r13
0x140a8fd7d 415e pop r14
0x140a8fd7f 415f pop r15
0x140a8fd81 5d pop rbp
0x140a8fd82 c3 ret 
