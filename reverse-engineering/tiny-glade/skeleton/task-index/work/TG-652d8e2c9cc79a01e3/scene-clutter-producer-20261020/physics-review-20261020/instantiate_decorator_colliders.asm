0x140829850 55 push rbp
0x140829851 4157 push r15
0x140829853 4156 push r14
0x140829855 4155 push r13
0x140829857 4154 push r12
0x140829859 56 push rsi
0x14082985a 57 push rdi
0x14082985b 53 push rbx
0x14082985c 4881ec18020000 sub rsp, 0x218
0x140829863 488dac2480000000 lea rbp, [rsp + 0x80]
0x14082986b 66440f7f8580010000 movdqa xmmword ptr [rbp + 0x180], xmm8
0x140829874 0f29bd70010000 movaps xmmword ptr [rbp + 0x170], xmm7
0x14082987b 660f7fb560010000 movdqa xmmword ptr [rbp + 0x160], xmm6
0x140829883 48c78558010000feffffff mov qword ptr [rbp + 0x158], 0xfffffffffffffffe
0x14082988e 4c898da0000000 mov qword ptr [rbp + 0xa0], r9
0x140829895 4c89c7 mov rdi, r8
0x140829898 4989d5 mov r13, rdx
0x14082989b 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
0x1408298a2 e859d13e00 call 0x140c16a00
0x1408298a7 84c0 test al, al
0x1408298a9 745f je 0x14082990a
0x1408298ab 4c8d35db7b2602 lea r14, [rip + 0x2267bdb]
0x1408298b2 bb3c000000 mov ebx, 0x3c
0x1408298b7 ba3c000000 mov edx, 0x3c
0x1408298bc 4c89f1 mov rcx, r14
0x1408298bf e8fcb2ffff call 0x140824bc0
0x1408298c4 4883f801 cmp rax, 1
0x1408298c8 0f85b1000000 jne 0x14082997f
0x1408298ce 4989d1 mov r9, rdx
0x1408298d1 4885d2 test rdx, rdx
0x1408298d4 743d je 0x140829913
0x1408298d6 4983f93c cmp r9, 0x3c
0x1408298da 7335 jae 0x140829911
0x1408298dc 488d05aa7b2602 lea rax, [rip + 0x2267baa]
0x1408298e3 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x1408298e8 7f29 jg 0x140829913
0x1408298ea 488d059f7c2602 lea rax, [rip + 0x2267c9f]
0x1408298f1 4889442420 mov qword ptr [rsp + 0x20], rax
0x1408298f6 488d0d907b2602 lea rcx, [rip + 0x2267b90]
0x1408298fd ba3c000000 mov edx, 0x3c
0x140829902 4531c0 xor r8d, r8d
0x140829905 e836000b02 call 0x1428d9940
0x14082990a 31c0 xor eax, eax
0x14082990c e9bd010000 jmp 0x140829ace
0x140829911 75d7 jne 0x1408298ea
0x140829913 4c8d35737b2602 lea r14, [rip + 0x2267b73]
0x14082991a 4c89f1 mov rcx, r14
0x14082991d 4c89ca mov rdx, r9
0x140829920 e89bb2ffff call 0x140824bc0
0x140829925 4883f801 cmp rax, 1
0x140829929 7554 jne 0x14082997f
0x14082992b 4989d0 mov r8, rdx
0x14082992e 4983c002 add r8, 2
0x140829932 7439 je 0x14082996d
0x140829934 4983f83c cmp r8, 0x3c
0x140829938 7331 jae 0x14082996b
0x14082993a 488d054c7b2602 lea rax, [rip + 0x2267b4c]
0x140829941 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x140829946 7f25 jg 0x14082996d
0x140829948 488d05597c2602 lea rax, [rip + 0x2267c59]
0x14082994f 4889442420 mov qword ptr [rsp + 0x20], rax
0x140829954 488d0d327b2602 lea rcx, [rip + 0x2267b32]
0x14082995b ba3c000000 mov edx, 0x3c
0x140829960 41b93c000000 mov r9d, 0x3c
0x140829966 e8d5ff0a02 call 0x1428d9940
0x14082996b 75db jne 0x140829948
0x14082996d bb3a000000 mov ebx, 0x3a
0x140829972 4829d3 sub rbx, rdx
0x140829975 4c8d35117b2602 lea r14, [rip + 0x2267b11]
0x14082997c 4d01c6 add r14, r8
0x14082997f 4c8d05f37c2602 lea r8, [rip + 0x2267cf3]
0x140829986 4c8d3dc37c2602 lea r15, [rip + 0x2267cc3]
0x14082998d 0f1f00 nop dword ptr [rax]
0x140829990 410fbe40ff movsx eax, byte ptr [r8 - 1]
0x140829995 85c0 test eax, eax
0x140829997 7817 js 0x1408299b0
0x140829999 49ffc8 dec r8
0x14082999c 83f85c cmp eax, 0x5c
0x14082999f 7563 jne 0x140829a04
0x1408299a1 eb72 jmp 0x140829a15
0x1408299a3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1408299b0 410fb648fe movzx ecx, byte ptr [r8 - 2]
0x1408299b5 80f9c0 cmp cl, 0xc0
0x1408299b8 7d23 jge 0x1408299dd
0x1408299ba 410fb650fd movzx edx, byte ptr [r8 - 3]
0x1408299bf 80fac0 cmp dl, 0xc0
0x1408299c2 7d22 jge 0x1408299e6
0x1408299c4 450fb648fc movzx r9d, byte ptr [r8 - 4]
0x1408299c9 4983c0fc add r8, -4
0x1408299cd 4183e107 and r9d, 7
0x1408299d1 41c1e106 shl r9d, 6
0x1408299d5 83e23f and edx, 0x3f
0x1408299d8 4409ca or edx, r9d
0x1408299db eb10 jmp 0x1408299ed
0x1408299dd 4983c0fe add r8, -2
0x1408299e1 83e11f and ecx, 0x1f
0x1408299e4 eb0f jmp 0x1408299f5
0x1408299e6 4983c0fd add r8, -3
0x1408299ea 83e20f and edx, 0xf
0x1408299ed c1e206 shl edx, 6
0x1408299f0 83e13f and ecx, 0x3f
0x1408299f3 09d1 or ecx, edx
0x1408299f5 c1e106 shl ecx, 6
0x1408299f8 243f and al, 0x3f
0x1408299fa 0fb6c0 movzx eax, al
0x1408299fd 09c8 or eax, ecx
0x1408299ff 83f85c cmp eax, 0x5c
0x140829a02 7411 je 0x140829a15
0x140829a04 83f82f cmp eax, 0x2f
0x140829a07 740c je 0x140829a15
0x140829a09 4d39f8 cmp r8, r15
0x140829a0c 7582 jne 0x140829990
0x140829a0e be29000000 mov esi, 0x29
0x140829a13 eb4c jmp 0x140829a61
0x140829a15 4c8d3d347c2602 lea r15, [rip + 0x2267c34]
0x140829a1c 4d29f8 sub r8, r15
0x140829a1f 49ffc0 inc r8
0x140829a22 7432 je 0x140829a56
0x140829a24 4983f829 cmp r8, 0x29
0x140829a28 732a jae 0x140829a54
0x140829a2a 43803c38bf cmp byte ptr [r8 + r15], 0xbf
0x140829a2f 7f25 jg 0x140829a56
0x140829a31 488d05387b2602 lea rax, [rip + 0x2267b38]
0x140829a38 4889442420 mov qword ptr [rsp + 0x20], rax
0x140829a3d 488d0d0c7c2602 lea rcx, [rip + 0x2267c0c]
0x140829a44 ba29000000 mov edx, 0x29
0x140829a49 41b929000000 mov r9d, 0x29
0x140829a4f e8ecfe0a02 call 0x1428d9940
0x140829a54 75db jne 0x140829a31
0x140829a56 be29000000 mov esi, 0x29
0x140829a5b 4c29c6 sub rsi, r8
0x140829a5e 4d01c7 add r15, r8
0x140829a61 e8cae63e00 call 0x140c18130
0x140829a66 4889c1 mov rcx, rax
0x140829a69 488b00 mov rax, qword ptr [rax]
0x140829a6c 4883f801 cmp rax, 1
0x140829a70 0f8549110000 jne 0x14082abbf
0x140829a76 4883c108 add rcx, 8
0x140829a7a 48833900 cmp qword ptr [rcx], 0
0x140829a7e 0f855f110000 jne 0x14082abe3
0x140829a84 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140829a8b 48898d40010000 mov qword ptr [rbp + 0x140], rcx
0x140829a92 4883c108 add rcx, 8
0x140829a96 4889742420 mov qword ptr [rsp + 0x20], rsi
0x140829a9b 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x140829aa4 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x140829aad 4c89f2 mov rdx, r14
0x140829ab0 4989d8 mov r8, rbx
0x140829ab3 4d89f9 mov r9, r15
0x140829ab6 e885d23e00 call 0x140c16d40
0x140829abb 488b8d40010000 mov rcx, qword ptr [rbp + 0x140]
0x140829ac2 48ff01 inc qword ptr [rcx]
0x140829ac5 48894530 mov qword ptr [rbp + 0x30], rax
0x140829ac9 b801000000 mov eax, 1
0x140829ace 48894528 mov qword ptr [rbp + 0x28], rax
0x140829ad2 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
0x140829ad9 488b30 mov rsi, qword ptr [rax]
0x140829adc 4c8b7808 mov r15, qword ptr [rax + 8]
0x140829ae0 66440f6f06 movdqa xmm8, xmmword ptr [rsi]
0x140829ae5 488b5818 mov rbx, qword ptr [rax + 0x18]
0x140829ae9 e8523c5200 call 0x140d4d740
0x140829aee 4989c6 mov r14, rax
0x140829af1 f60001 test byte ptr [rax], 1
0x140829af4 7411 je 0x140829b07
0x140829af6 4c89f1 mov rcx, r14
0x140829af9 4883c108 add rcx, 8
0x140829afd 498b4608 mov rax, qword ptr [r14 + 8]
0x140829b01 498b5610 mov rdx, qword ptr [r14 + 0x10]
0x140829b05 eb1b jmp 0x140829b22
0x140829b07 e8f4b25300 call 0x140d64e00
0x140829b0c 4c89f1 mov rcx, r14
0x140829b0f 4883c108 add rcx, 8
0x140829b13 49c70601000000 mov qword ptr [r14], 1
0x140829b1a 49894608 mov qword ptr [r14 + 8], rax
0x140829b1e 49895610 mov qword ptr [r14 + 0x10], rdx
0x140829b22 4c8d4001 lea r8, [rax + 1]
0x140829b26 4c8901 mov qword ptr [rcx], r8
0x140829b29 f30f6f358f772602 movdqu xmm6, xmmword ptr [rip + 0x226778f]
0x140829b31 660f7fb5d0000000 movdqa xmmword ptr [rbp + 0xd0], xmm6
0x140829b39 0f103d70772602 movups xmm7, xmmword ptr [rip + 0x2267770]
0x140829b40 0f29bdc0000000 movaps xmmword ptr [rbp + 0xc0], xmm7
0x140829b47 488985e0000000 mov qword ptr [rbp + 0xe0], rax
0x140829b4e 488995e8000000 mov qword ptr [rbp + 0xe8], rdx
0x140829b55 4885db test rbx, rbx
0x140829b58 0f8591100000 jne 0x14082abef
0x140829b5e 4a8d043e lea rax, [rsi + r15]
0x140829b62 48ffc0 inc rax
0x140829b65 66410fd7c8 pmovmskb ecx, xmm8
0x140829b6a f7d1 not ecx
0x140829b6c 488d95c0000000 lea rdx, [rbp + 0xc0]
0x140829b73 48899580000000 mov qword ptr [rbp + 0x80], rdx
0x140829b7a 488d9580000000 lea rdx, [rbp + 0x80]
0x140829b81 488955e0 mov qword ptr [rbp - 0x20], rdx
0x140829b85 48897540 mov qword ptr [rbp + 0x40], rsi
0x140829b89 4883c610 add rsi, 0x10
0x140829b8d 48897548 mov qword ptr [rbp + 0x48], rsi
0x140829b91 48894550 mov qword ptr [rbp + 0x50], rax
0x140829b95 66894d58 mov word ptr [rbp + 0x58], cx
0x140829b99 488d4d40 lea rcx, [rbp + 0x40]
0x140829b9d 4c8d45e0 lea r8, [rbp - 0x20]
0x140829ba1 4889da mov rdx, rbx
0x140829ba4 e8b7a5a300 call 0x141264160
0x140829ba9 660f6f85c0000000 movdqa xmm0, xmmword ptr [rbp + 0xc0]
0x140829bb1 660f6f8dd0000000 movdqa xmm1, xmmword ptr [rbp + 0xd0]
0x140829bb9 660f6f95e0000000 movdqa xmm2, xmmword ptr [rbp + 0xe0]
0x140829bc1 660f7f45e0 movdqa xmmword ptr [rbp - 0x20], xmm0
0x140829bc6 660f7f4df0 movdqa xmmword ptr [rbp - 0x10], xmm1
0x140829bcb 660f7f5500 movdqa xmmword ptr [rbp], xmm2
0x140829bd0 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
0x140829bd7 488b7030 mov rsi, qword ptr [rax + 0x30]
0x140829bdb 4c8b6038 mov r12, qword ptr [rax + 0x38]
0x140829bdf 66440f6f06 movdqa xmm8, xmmword ptr [rsi]
0x140829be4 488b5848 mov rbx, qword ptr [rax + 0x48]
0x140829be8 e8533b5200 call 0x140d4d740
0x140829bed 4989c6 mov r14, rax
0x140829bf0 f60001 test byte ptr [rax], 1
0x140829bf3 7411 je 0x140829c06
0x140829bf5 4c89f1 mov rcx, r14
0x140829bf8 4883c108 add rcx, 8
0x140829bfc 498b4608 mov rax, qword ptr [r14 + 8]
0x140829c00 498b5610 mov rdx, qword ptr [r14 + 0x10]
0x140829c04 eb22 jmp 0x140829c28
0x140829c06 c6855501000001 mov byte ptr [rbp + 0x155], 1
0x140829c0d e8eeb15300 call 0x140d64e00
0x140829c12 4c89f1 mov rcx, r14
0x140829c15 4883c108 add rcx, 8
0x140829c19 49c70601000000 mov qword ptr [r14], 1
0x140829c20 49894608 mov qword ptr [r14 + 8], rax
0x140829c24 49895610 mov qword ptr [r14 + 0x10], rdx
0x140829c28 4c8d4001 lea r8, [rax + 1]
0x140829c2c 4c8901 mov qword ptr [rcx], r8
0x140829c2f 660f7fb5d0000000 movdqa xmmword ptr [rbp + 0xd0], xmm6
0x140829c37 0f29bdc0000000 movaps xmmword ptr [rbp + 0xc0], xmm7
0x140829c3e 488985e0000000 mov qword ptr [rbp + 0xe0], rax
0x140829c45 488995e8000000 mov qword ptr [rbp + 0xe8], rdx
0x140829c4c 4885db test rbx, rbx
0x140829c4f 0f85b80f0000 jne 0x14082ac0d
0x140829c55 4a8d0426 lea rax, [rsi + r12]
0x140829c59 48ffc0 inc rax
0x140829c5c 66410fd7c8 pmovmskb ecx, xmm8
0x140829c61 f7d1 not ecx
0x140829c63 488d95c0000000 lea rdx, [rbp + 0xc0]
0x140829c6a 48895510 mov qword ptr [rbp + 0x10], rdx
0x140829c6e 488d5510 lea rdx, [rbp + 0x10]
0x140829c72 48899580000000 mov qword ptr [rbp + 0x80], rdx
0x140829c79 48897540 mov qword ptr [rbp + 0x40], rsi
0x140829c7d 4883c610 add rsi, 0x10
0x140829c81 48897548 mov qword ptr [rbp + 0x48], rsi
0x140829c85 48894550 mov qword ptr [rbp + 0x50], rax
0x140829c89 66894d58 mov word ptr [rbp + 0x58], cx
0x140829c8d 488d4d40 lea rcx, [rbp + 0x40]
0x140829c91 4c8d8580000000 lea r8, [rbp + 0x80]
0x140829c98 4889da mov rdx, rbx
0x140829c9b e8c0a4a300 call 0x141264160
0x140829ca0 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
0x140829ca7 4883c030 add rax, 0x30
0x140829cab 48894538 mov qword ptr [rbp + 0x38], rax
0x140829caf 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
0x140829cb6 660f6f8dd0000000 movdqa xmm1, xmmword ptr [rbp + 0xd0]
0x140829cbe 660f6f95e0000000 movdqa xmm2, xmmword ptr [rbp + 0xe0]
0x140829cc6 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
0x140829cca 660f7f4d50 movdqa xmmword ptr [rbp + 0x50], xmm1
0x140829ccf 660f7f5560 movdqa xmmword ptr [rbp + 0x60], xmm2
0x140829cd4 498b5d00 mov rbx, qword ptr [r13]
0x140829cd8 4d8b6d18 mov r13, qword ptr [r13 + 0x18]
0x140829cdc 660f6f03 movdqa xmm0, xmmword ptr [rbx]
0x140829ce0 66440fd7e0 pmovmskb r12d, xmm0
0x140829ce5 41f7d4 not r12d
0x140829ce8 4c8d7310 lea r14, [rbx + 0x10]
0x140829cec 488b4718 mov rax, qword ptr [rdi + 0x18]
0x140829cf0 488985b0000000 mov qword ptr [rbp + 0xb0], rax
0x140829cf7 488d4720 lea rax, [rdi + 0x20]
0x140829cfb 48894570 mov qword ptr [rbp + 0x70], rax
0x140829cff 488b07 mov rax, qword ptr [rdi]
0x140829d02 488b4f08 mov rcx, qword ptr [rdi + 8]
0x140829d06 48898d40010000 mov qword ptr [rbp + 0x140], rcx
0x140829d0d 488d48f0 lea rcx, [rax - 0x10]
0x140829d11 48894dd0 mov qword ptr [rbp - 0x30], rcx
0x140829d15 488985a8000000 mov qword ptr [rbp + 0xa8], rax
0x140829d1c 4883c0f8 add rax, -8
0x140829d20 488945d8 mov qword ptr [rbp - 0x28], rax
0x140829d24 660f76f6 pcmpeqd xmm6, xmm6
0x140829d28 eb0f jmp 0x140829d39
0x140829d2a 660f1f440000 nop word ptr [rax + rax]
0x140829d30 83f801 cmp eax, 1
0x140829d33 0f85f20e0000 jne 0x14082ac2b
0x140829d39 4d85ed test r13, r13
0x140829d3c 0f95854f010000 setne byte ptr [rbp + 0x14f]
0x140829d43 0f84390a0000 je 0x14082a782
0x140829d49 664585e4 test r12w, r12w
0x140829d4d 7411 je 0x140829d60
0x140829d4f 418d4424ff lea eax, [r12 - 1]
0x140829d54 4421e0 and eax, r12d
0x140829d57 eb35 jmp 0x140829d8e
0x140829d59 0f1f8000000000 nop dword ptr [rax]
0x140829d60 66410f6f06 movdqa xmm0, xmmword ptr [r14]
0x140829d65 66440fd7e0 pmovmskb r12d, xmm0
0x140829d6a 4881c300ffffff add rbx, -0x100
0x140829d71 4983c610 add r14, 0x10
0x140829d75 4181fcffff0000 cmp r12d, 0xffff
0x140829d7c 74e2 je 0x140829d60
0x140829d7e b9feffffff mov ecx, 0xfffffffe
0x140829d83 4429e1 sub ecx, r12d
0x140829d86 41f7d4 not r12d
0x140829d89 4489e0 mov eax, r12d
0x140829d8c 21c8 and eax, ecx
0x140829d8e 4489e1 mov ecx, r12d
0x140829d91 f30fbcc9 tzcnt ecx, ecx
0x140829d95 4189c4 mov r12d, eax
0x140829d98 49ffcd dec r13
0x140829d9b c1e104 shl ecx, 4
0x140829d9e 4889d8 mov rax, rbx
0x140829da1 4829c8 sub rax, rcx
0x140829da4 8b78f0 mov edi, dword ptr [rax - 0x10]
0x140829da7 488b70f8 mov rsi, qword ptr [rax - 8]
0x140829dab 89bdc0000000 mov dword ptr [rbp + 0xc0], edi
0x140829db1 4883bdb000000000 cmp qword ptr [rbp + 0xb0], 0
0x140829db9 0f84b6000000 je 0x140829e75
0x140829dbf 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x140829dc3 488d95c0000000 lea rdx, [rbp + 0xc0]
0x140829dca e80171d8ff call 0x1405b0ed0
0x140829dcf 488b8d40010000 mov rcx, qword ptr [rbp + 0x140]
0x140829dd6 4821c1 and rcx, rax
0x140829dd9 48c1e839 shr rax, 0x39
0x140829ddd 660f6ec0 movd xmm0, eax
0x140829de1 660f60c0 punpcklbw xmm0, xmm0
0x140829de5 f20f70c000 pshuflw xmm0, xmm0, 0
0x140829dea 660f70c000 pshufd xmm0, xmm0, 0
0x140829def 31c0 xor eax, eax
0x140829df1 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x140829df8 f30f6f0c0a movdqu xmm1, xmmword ptr [rdx + rcx]
0x140829dfd 660f6fd1 movdqa xmm2, xmm1
0x140829e01 660f74d0 pcmpeqb xmm2, xmm0
0x140829e05 660fd7d2 pmovmskb edx, xmm2
0x140829e09 85d2 test edx, edx
0x140829e0b 7433 je 0x140829e40
0x140829e0d f3440fbcc2 tzcnt r8d, edx
0x140829e12 4901c8 add r8, rcx
0x140829e15 4c238540010000 and r8, qword ptr [rbp + 0x140]
0x140829e1c 4d89c1 mov r9, r8
0x140829e1f 49c1e104 shl r9, 4
0x140829e23 4c8b55d0 mov r10, qword ptr [rbp - 0x30]
0x140829e27 4d29ca sub r10, r9
0x140829e2a 413b3a cmp edi, dword ptr [r10]
0x140829e2d 7431 je 0x140829e60
0x140829e2f 448d42ff lea r8d, [rdx - 1]
0x140829e33 664121d0 and r8w, dx
0x140829e37 4489c2 mov edx, r8d
0x140829e3a 75d1 jne 0x140829e0d
0x140829e3c 0f1f4000 nop dword ptr [rax]
0x140829e40 660f74ce pcmpeqb xmm1, xmm6
0x140829e44 660fd7d1 pmovmskb edx, xmm1
0x140829e48 85d2 test edx, edx
0x140829e4a 7529 jne 0x140829e75
0x140829e4c 4801c1 add rcx, rax
0x140829e4f 4883c110 add rcx, 0x10
0x140829e53 4883c010 add rax, 0x10
0x140829e57 48238d40010000 and rcx, qword ptr [rbp + 0x140]
0x140829e5e eb91 jmp 0x140829df1
0x140829e60 49f7d8 neg r8
0x140829e63 49c1e004 shl r8, 4
0x140829e67 488b45d8 mov rax, qword ptr [rbp - 0x28]
0x140829e6b 4a393400 cmp qword ptr [rax + r8], rsi
0x140829e6f 0f8423030000 je 0x14082a198
0x140829e75 89bd50010000 mov dword ptr [rbp + 0x150], edi
0x140829e7b 80be3801000000 cmp byte ptr [rsi + 0x138], 0
0x140829e82 0f85b1feffff jne 0x140829d39
0x140829e88 837e6000 cmp dword ptr [rsi + 0x60], 0
0x140829e8c 0f858b030000 jne 0x14082a21d
0x140829e92 488b5670 mov rdx, qword ptr [rsi + 0x70]
0x140829e96 f048ff02 lock inc qword ptr [rdx]
0x140829e9a 0f8e190e0000 jle 0x14082acb9
0x140829ea0 89bdc4000000 mov dword ptr [rbp + 0xc4], edi
0x140829ea6 0f284650 movaps xmm0, xmmword ptr [rsi + 0x50]
0x140829eaa 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x140829eb1 c685c000000003 mov byte ptr [rbp + 0xc0], 3
0x140829eb8 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x140829ebf 488d8d80000000 lea rcx, [rbp + 0x80]
0x140829ec6 4c8d85c0000000 lea r8, [rbp + 0xc0]
0x140829ecd e83e4c0a00 call 0x1408ceb10
0x140829ed2 8b466c mov eax, dword ptr [rsi + 0x6c]
0x140829ed5 8985c8000000 mov dword ptr [rbp + 0xc8], eax
0x140829edb 488b4664 mov rax, qword ptr [rsi + 0x64]
0x140829edf 488985c0000000 mov qword ptr [rbp + 0xc0], rax
0x140829ee6 660f6f8680000000 movdqa xmm0, xmmword ptr [rsi + 0x80]
0x140829eee 660f7f85d0000000 movdqa xmmword ptr [rbp + 0xd0], xmm0
0x140829ef6 448b8d50010000 mov r9d, dword ptr [rbp + 0x150]
0x140829efd c6855701000001 mov byte ptr [rbp + 0x157], 1
0x140829f04 488d85c0000000 lea rax, [rbp + 0xc0]
0x140829f0b 4889442428 mov qword ptr [rsp + 0x28], rax
0x140829f10 488d8580000000 lea rax, [rbp + 0x80]
0x140829f17 4889442420 mov qword ptr [rsp + 0x20], rax
0x140829f1c 488d4d10 lea rcx, [rbp + 0x10]
0x140829f20 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x140829f27 4c8b85b8000000 mov r8, qword ptr [rbp + 0xb8]
0x140829f2e e84daaa600 call 0x141294980
0x140829f33 8b451c mov eax, dword ptr [rbp + 0x1c]
0x140829f36 89457c mov dword ptr [rbp + 0x7c], eax
0x140829f39 488b05c820e002 mov rax, qword ptr [rip + 0x2e020c8]
0x140829f40 803801 cmp byte ptr [rax], 1
0x140829f43 0f85a5020000 jne 0x14082a1ee
0x140829f49 8b457c mov eax, dword ptr [rbp + 0x7c]
0x140829f4c 85c0 test eax, eax
0x140829f4e 0f8424070000 je 0x14082a678
0x140829f54 83f801 cmp eax, 1
0x140829f57 0f8597020000 jne 0x14082a1f4
0x140829f5d 488b054424e002 mov rax, qword ptr [rip + 0x2e02444]
0x140829f64 488b00 mov rax, qword ptr [rax]
0x140829f67 4883f803 cmp rax, 3
0x140829f6b 0f82f1000000 jb 0x14082a062
0x140829f71 488d8550010000 lea rax, [rbp + 0x150]
0x140829f78 48898580000000 mov qword ptr [rbp + 0x80], rax
0x140829f7f 488d05eab2ffff lea rax, [rip - 0x4d16]
0x140829f86 48898588000000 mov qword ptr [rbp + 0x88], rax
0x140829f8d c6855701000001 mov byte ptr [rbp + 0x157], 1
0x140829f94 488d0dcd7a2602 lea rcx, [rip + 0x2267acd]
0x140829f9b e8e03de5ff call 0x14067dd80
0x140829fa0 f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x140829fa4 8b4010 mov eax, dword ptr [rax + 0x10]
0x140829fa7 48c785f000000003000000 mov qword ptr [rbp + 0xf0], 3
0x140829fb2 488d15d7782602 lea rdx, [rip + 0x22678d7]
0x140829fb9 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
0x140829fc0 48c785000100001b000000 mov qword ptr [rbp + 0x100], 0x1b
0x140829fcb 488d0d767a2602 lea rcx, [rip + 0x2267a76]
0x140829fd2 48898d10010000 mov qword ptr [rbp + 0x110], rcx
0x140829fd9 48c7851801000002000000 mov qword ptr [rbp + 0x118], 2
0x140829fe4 488d8d80000000 lea rcx, [rbp + 0x80]
0x140829feb 48898d20010000 mov qword ptr [rbp + 0x120], rcx
0x140829ff2 48c7852801000001000000 mov qword ptr [rbp + 0x128], 1
0x140829ffd 48c7853001000000000000 mov qword ptr [rbp + 0x130], 0
0x14082a008 48c785c000000000000000 mov qword ptr [rbp + 0xc0], 0
0x14082a013 488995c8000000 mov qword ptr [rbp + 0xc8], rdx
0x14082a01a 48c785d00000001b000000 mov qword ptr [rbp + 0xd0], 0x1b
0x14082a025 48c785d800000000000000 mov qword ptr [rbp + 0xd8], 0
0x14082a030 f30f7f85e0000000 movdqu xmmword ptr [rbp + 0xe0], xmm0
0x14082a038 c7850801000001000000 mov dword ptr [rbp + 0x108], 1
0x14082a042 89850c010000 mov dword ptr [rbp + 0x10c], eax
0x14082a048 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a04f 488d8d56010000 lea rcx, [rbp + 0x156]
0x14082a056 488d95c0000000 lea rdx, [rbp + 0xc0]
0x14082a05d e8ce025200 call 0x140d4a330
0x14082a062 488bbec8000000 mov rdi, qword ptr [rsi + 0xc8]
0x14082a069 4c69bed000000060010000 imul r15, qword ptr [rsi + 0xd0], 0x160
0x14082a074 4901ff add r15, rdi
0x14082a077 660f1f840000000000 nop word ptr [rax + rax]
0x14082a080 4c39ff cmp rdi, r15
0x14082a083 0f8465010000 je 0x14082a1ee
0x14082a089 4889f8 mov rax, rdi
0x14082a08c 4881c760010000 add rdi, 0x160
0x14082a093 488b0d0e23e002 mov rcx, qword ptr [rip + 0x2e0230e]
0x14082a09a 488b09 mov rcx, qword ptr [rcx]
0x14082a09d 4883f903 cmp rcx, 3
0x14082a0a1 72dd jb 0x14082a080
0x14082a0a3 4805a0000000 add rax, 0xa0
0x14082a0a9 48898580000000 mov qword ptr [rbp + 0x80], rax
0x14082a0b0 488d05d9fd8dff lea rax, [rip - 0x720227]
0x14082a0b7 48898588000000 mov qword ptr [rbp + 0x88], rax
0x14082a0be c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a0c5 488d0dd4792602 lea rcx, [rip + 0x22679d4]
0x14082a0cc e8af3ce5ff call 0x14067dd80
0x14082a0d1 f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x14082a0d5 8b4010 mov eax, dword ptr [rax + 0x10]
0x14082a0d8 48c785f000000003000000 mov qword ptr [rbp + 0xf0], 3
0x14082a0e3 488d15a6772602 lea rdx, [rip + 0x22677a6]
0x14082a0ea 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
0x14082a0f1 48c785000100001b000000 mov qword ptr [rbp + 0x100], 0x1b
0x14082a0fc 488d0d8d792602 lea rcx, [rip + 0x226798d]
0x14082a103 48898d10010000 mov qword ptr [rbp + 0x110], rcx
0x14082a10a 48c7851801000001000000 mov qword ptr [rbp + 0x118], 1
0x14082a115 488d8d80000000 lea rcx, [rbp + 0x80]
0x14082a11c 48898d20010000 mov qword ptr [rbp + 0x120], rcx
0x14082a123 48c7852801000001000000 mov qword ptr [rbp + 0x128], 1
0x14082a12e 48c7853001000000000000 mov qword ptr [rbp + 0x130], 0
0x14082a139 48c785c000000000000000 mov qword ptr [rbp + 0xc0], 0
0x14082a144 488995c8000000 mov qword ptr [rbp + 0xc8], rdx
0x14082a14b 48c785d00000001b000000 mov qword ptr [rbp + 0xd0], 0x1b
0x14082a156 48c785d800000000000000 mov qword ptr [rbp + 0xd8], 0
0x14082a161 f30f7f85e0000000 movdqu xmmword ptr [rbp + 0xe0], xmm0
0x14082a169 c7850801000001000000 mov dword ptr [rbp + 0x108], 1
0x14082a173 89850c010000 mov dword ptr [rbp + 0x10c], eax
0x14082a179 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a180 488d8d56010000 lea rcx, [rbp + 0x156]
0x14082a187 488d95c0000000 lea rdx, [rbp + 0xc0]
0x14082a18e e89d015200 call 0x140d4a330
0x14082a193 e9e8feffff jmp 0x14082a080
0x14082a198 89bd50010000 mov dword ptr [rbp + 0x150], edi
0x14082a19e 488d4d00 lea rcx, [rbp]
0x14082a1a2 488db550010000 lea rsi, [rbp + 0x150]
0x14082a1a9 4889f2 mov rdx, rsi
0x14082a1ac e81f6dd8ff call 0x1405b0ed0
0x14082a1b1 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a1b8 488d4de0 lea rcx, [rbp - 0x20]
0x14082a1bc 4889c2 mov rdx, rax
0x14082a1bf 4989f0 mov r8, rsi
0x14082a1c2 e809d53400 call 0x140b776d0
0x14082a1c7 488d4d60 lea rcx, [rbp + 0x60]
0x14082a1cb 4889f2 mov rdx, rsi
0x14082a1ce e8fd6cd8ff call 0x1405b0ed0
0x14082a1d3 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a1da 488d4d40 lea rcx, [rbp + 0x40]
0x14082a1de 4889c2 mov rdx, rax
0x14082a1e1 4989f0 mov r8, rsi
0x14082a1e4 e8e7d43400 call 0x140b776d0
0x14082a1e9 e94bfbffff jmp 0x140829d39
0x14082a1ee 837d7c00 cmp dword ptr [rbp + 0x7c], 0
0x14082a1f2 7429 je 0x14082a21d
0x14082a1f4 488d4d00 lea rcx, [rbp]
0x14082a1f8 488dbd50010000 lea rdi, [rbp + 0x150]
0x14082a1ff 4889fa mov rdx, rdi
0x14082a202 e8c96cd8ff call 0x1405b0ed0
0x14082a207 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a20e 488d4de0 lea rcx, [rbp - 0x20]
0x14082a212 4889c2 mov rdx, rax
0x14082a215 4989f8 mov r8, rdi
0x14082a218 e8b3d43400 call 0x140b776d0
0x14082a21d 8b8690000000 mov eax, dword ptr [rsi + 0x90]
0x14082a223 85c0 test eax, eax
0x14082a225 0f8505fbffff jne 0x140829d30
0x14082a22b 488b96a0000000 mov rdx, qword ptr [rsi + 0xa0]
0x14082a232 f048ff02 lock inc qword ptr [rdx]
0x14082a236 0f8e7d0a0000 jle 0x14082acb9
0x14082a23c 0fb68639010000 movzx eax, byte ptr [rsi + 0x139]
0x14082a243 0c04 or al, 4
0x14082a245 8b8d50010000 mov ecx, dword ptr [rbp + 0x150]
0x14082a24b 898dc4000000 mov dword ptr [rbp + 0xc4], ecx
0x14082a251 0f284650 movaps xmm0, xmmword ptr [rsi + 0x50]
0x14082a255 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x14082a25c 8885c0000000 mov byte ptr [rbp + 0xc0], al
0x14082a262 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a269 488d8d80000000 lea rcx, [rbp + 0x80]
0x14082a270 4c8d85c0000000 lea r8, [rbp + 0xc0]
0x14082a277 e894480a00 call 0x1408ceb10
0x14082a27c 8b869c000000 mov eax, dword ptr [rsi + 0x9c]
0x14082a282 8985c8000000 mov dword ptr [rbp + 0xc8], eax
0x14082a288 488b8694000000 mov rax, qword ptr [rsi + 0x94]
0x14082a28f 488985c0000000 mov qword ptr [rbp + 0xc0], rax
0x14082a296 660f6f86b0000000 movdqa xmm0, xmmword ptr [rsi + 0xb0]
0x14082a29e 660f7f85d0000000 movdqa xmmword ptr [rbp + 0xd0], xmm0
0x14082a2a6 448b8d50010000 mov r9d, dword ptr [rbp + 0x150]
0x14082a2ad c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a2b4 488d85c0000000 lea rax, [rbp + 0xc0]
0x14082a2bb 4889442428 mov qword ptr [rsp + 0x28], rax
0x14082a2c0 488d8580000000 lea rax, [rbp + 0x80]
0x14082a2c7 4889442420 mov qword ptr [rsp + 0x20], rax
0x14082a2cc 488d4d10 lea rcx, [rbp + 0x10]
0x14082a2d0 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x14082a2d7 4c8b4538 mov r8, qword ptr [rbp + 0x38]
0x14082a2db e8a0a6a600 call 0x141294980
0x14082a2e0 8b7d1c mov edi, dword ptr [rbp + 0x1c]
0x14082a2e3 488b051e1de002 mov rax, qword ptr [rip + 0x2e01d1e]
0x14082a2ea 803801 cmp byte ptr [rax], 1
0x14082a2ed 0f8545020000 jne 0x14082a538
0x14082a2f3 85ff test edi, edi
0x14082a2f5 0f8473020000 je 0x14082a56e
0x14082a2fb 83ff01 cmp edi, 1
0x14082a2fe 0f853c020000 jne 0x14082a540
0x14082a304 488b059d20e002 mov rax, qword ptr [rip + 0x2e0209d]
0x14082a30b 488b00 mov rax, qword ptr [rax]
0x14082a30e 4883f803 cmp rax, 3
0x14082a312 0f82f1000000 jb 0x14082a409
0x14082a318 488d8550010000 lea rax, [rbp + 0x150]
0x14082a31f 48898580000000 mov qword ptr [rbp + 0x80], rax
0x14082a326 488d0543afffff lea rax, [rip - 0x50bd]
0x14082a32d 48898588000000 mov qword ptr [rbp + 0x88], rax
0x14082a334 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a33b 488d0d26782602 lea rcx, [rip + 0x2267826]
0x14082a342 e8393ae5ff call 0x14067dd80
0x14082a347 f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x14082a34b 8b4010 mov eax, dword ptr [rax + 0x10]
0x14082a34e 48c785f000000003000000 mov qword ptr [rbp + 0xf0], 3
0x14082a359 488d1530752602 lea rdx, [rip + 0x2267530]
0x14082a360 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
0x14082a367 48c785000100001b000000 mov qword ptr [rbp + 0x100], 0x1b
0x14082a372 488d0dcf772602 lea rcx, [rip + 0x22677cf]
0x14082a379 48898d10010000 mov qword ptr [rbp + 0x110], rcx
0x14082a380 48c7851801000002000000 mov qword ptr [rbp + 0x118], 2
0x14082a38b 488d8d80000000 lea rcx, [rbp + 0x80]
0x14082a392 48898d20010000 mov qword ptr [rbp + 0x120], rcx
0x14082a399 48c7852801000001000000 mov qword ptr [rbp + 0x128], 1
0x14082a3a4 48c7853001000000000000 mov qword ptr [rbp + 0x130], 0
0x14082a3af 48c785c000000000000000 mov qword ptr [rbp + 0xc0], 0
0x14082a3ba 488995c8000000 mov qword ptr [rbp + 0xc8], rdx
0x14082a3c1 48c785d00000001b000000 mov qword ptr [rbp + 0xd0], 0x1b
0x14082a3cc 48c785d800000000000000 mov qword ptr [rbp + 0xd8], 0
0x14082a3d7 f30f7f85e0000000 movdqu xmmword ptr [rbp + 0xe0], xmm0
0x14082a3df c7850801000001000000 mov dword ptr [rbp + 0x108], 1
0x14082a3e9 89850c010000 mov dword ptr [rbp + 0x10c], eax
0x14082a3ef c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a3f6 488d8d56010000 lea rcx, [rbp + 0x156]
0x14082a3fd 488d95c0000000 lea rdx, [rbp + 0xc0]
0x14082a404 e827ff5100 call 0x140d4a330
0x14082a409 4c8bbec8000000 mov r15, qword ptr [rsi + 0xc8]
0x14082a410 4869b6d000000060010000 imul rsi, qword ptr [rsi + 0xd0], 0x160
0x14082a41b 4c01fe add rsi, r15
0x14082a41e 6690 nop
0x14082a420 4939f7 cmp r15, rsi
0x14082a423 0f840f010000 je 0x14082a538
0x14082a429 4c89f8 mov rax, r15
0x14082a42c 4981c760010000 add r15, 0x160
0x14082a433 488b0d6e1fe002 mov rcx, qword ptr [rip + 0x2e01f6e]
0x14082a43a 488b09 mov rcx, qword ptr [rcx]
0x14082a43d 4883f903 cmp rcx, 3
0x14082a441 72dd jb 0x14082a420
0x14082a443 4805a0000000 add rax, 0xa0
0x14082a449 48898580000000 mov qword ptr [rbp + 0x80], rax
0x14082a450 488d0539fa8dff lea rax, [rip - 0x7205c7]
0x14082a457 48898588000000 mov qword ptr [rbp + 0x88], rax
0x14082a45e c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a465 488d0d14772602 lea rcx, [rip + 0x2267714]
0x14082a46c e80f39e5ff call 0x14067dd80
0x14082a471 f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x14082a475 8b4010 mov eax, dword ptr [rax + 0x10]
0x14082a478 48c785f000000003000000 mov qword ptr [rbp + 0xf0], 3
0x14082a483 488d1506742602 lea rdx, [rip + 0x2267406]
0x14082a48a 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
0x14082a491 48c785000100001b000000 mov qword ptr [rbp + 0x100], 0x1b
0x14082a49c 488d0ded752602 lea rcx, [rip + 0x22675ed]
0x14082a4a3 48898d10010000 mov qword ptr [rbp + 0x110], rcx
0x14082a4aa 48c7851801000001000000 mov qword ptr [rbp + 0x118], 1
0x14082a4b5 488d8d80000000 lea rcx, [rbp + 0x80]
0x14082a4bc 48898d20010000 mov qword ptr [rbp + 0x120], rcx
0x14082a4c3 48c7852801000001000000 mov qword ptr [rbp + 0x128], 1
0x14082a4ce 48c7853001000000000000 mov qword ptr [rbp + 0x130], 0
0x14082a4d9 48c785c000000000000000 mov qword ptr [rbp + 0xc0], 0
0x14082a4e4 488995c8000000 mov qword ptr [rbp + 0xc8], rdx
0x14082a4eb 48c785d00000001b000000 mov qword ptr [rbp + 0xd0], 0x1b
0x14082a4f6 48c785d800000000000000 mov qword ptr [rbp + 0xd8], 0
0x14082a501 f30f7f85e0000000 movdqu xmmword ptr [rbp + 0xe0], xmm0
0x14082a509 c7850801000001000000 mov dword ptr [rbp + 0x108], 1
0x14082a513 89850c010000 mov dword ptr [rbp + 0x10c], eax
0x14082a519 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a520 488d8d56010000 lea rcx, [rbp + 0x156]
0x14082a527 488d95c0000000 lea rdx, [rbp + 0xc0]
0x14082a52e e8fdfd5100 call 0x140d4a330
0x14082a533 e9e8feffff jmp 0x14082a420
0x14082a538 85ff test edi, edi
0x14082a53a 0f84f9f7ffff je 0x140829d39
0x14082a540 488d4d60 lea rcx, [rbp + 0x60]
0x14082a544 488db550010000 lea rsi, [rbp + 0x150]
0x14082a54b 4889f2 mov rdx, rsi
0x14082a54e e87d69d8ff call 0x1405b0ed0
0x14082a553 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a55a 488d4d40 lea rcx, [rbp + 0x40]
0x14082a55e 4889c2 mov rdx, rax
0x14082a561 4989f0 mov r8, rsi
0x14082a564 e867d13400 call 0x140b776d0
0x14082a569 e9cbf7ffff jmp 0x140829d39
0x14082a56e 488b05331ee002 mov rax, qword ptr [rip + 0x2e01e33]
0x14082a575 488b00 mov rax, qword ptr [rax]
0x14082a578 4883f803 cmp rax, 3
0x14082a57c 0f82b7f7ffff jb 0x140829d39
0x14082a582 488d8550010000 lea rax, [rbp + 0x150]
0x14082a589 48898580000000 mov qword ptr [rbp + 0x80], rax
0x14082a590 488d05d9acffff lea rax, [rip - 0x5327]
0x14082a597 48898588000000 mov qword ptr [rbp + 0x88], rax
0x14082a59e c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a5a5 488d0d64752602 lea rcx, [rip + 0x2267564]
0x14082a5ac e8cf37e5ff call 0x14067dd80
0x14082a5b1 f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x14082a5b5 8b4010 mov eax, dword ptr [rax + 0x10]
0x14082a5b8 48c785f000000003000000 mov qword ptr [rbp + 0xf0], 3
0x14082a5c3 488d15c6722602 lea rdx, [rip + 0x22672c6]
0x14082a5ca 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
0x14082a5d1 48c785000100001b000000 mov qword ptr [rbp + 0x100], 0x1b
0x14082a5dc 488d0d0d752602 lea rcx, [rip + 0x226750d]
0x14082a5e3 48898d10010000 mov qword ptr [rbp + 0x110], rcx
0x14082a5ea 48c7851801000002000000 mov qword ptr [rbp + 0x118], 2
0x14082a5f5 488d8d80000000 lea rcx, [rbp + 0x80]
0x14082a5fc 48898d20010000 mov qword ptr [rbp + 0x120], rcx
0x14082a603 48c7852801000001000000 mov qword ptr [rbp + 0x128], 1
0x14082a60e 48c7853001000000000000 mov qword ptr [rbp + 0x130], 0
0x14082a619 48c785c000000000000000 mov qword ptr [rbp + 0xc0], 0
0x14082a624 488995c8000000 mov qword ptr [rbp + 0xc8], rdx
0x14082a62b 48c785d00000001b000000 mov qword ptr [rbp + 0xd0], 0x1b
0x14082a636 48c785d800000000000000 mov qword ptr [rbp + 0xd8], 0
0x14082a641 f30f7f85e0000000 movdqu xmmword ptr [rbp + 0xe0], xmm0
0x14082a649 c7850801000001000000 mov dword ptr [rbp + 0x108], 1
0x14082a653 89850c010000 mov dword ptr [rbp + 0x10c], eax
0x14082a659 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a660 488d8d56010000 lea rcx, [rbp + 0x156]
0x14082a667 488d95c0000000 lea rdx, [rbp + 0xc0]
0x14082a66e e8bdfc5100 call 0x140d4a330
0x14082a673 e9c1f6ffff jmp 0x140829d39
0x14082a678 488b05291de002 mov rax, qword ptr [rip + 0x2e01d29]
0x14082a67f 488b00 mov rax, qword ptr [rax]
0x14082a682 4883f803 cmp rax, 3
0x14082a686 0f8291fbffff jb 0x14082a21d
0x14082a68c 488d8550010000 lea rax, [rbp + 0x150]
0x14082a693 48898580000000 mov qword ptr [rbp + 0x80], rax
0x14082a69a 488d05cfabffff lea rax, [rip - 0x5431]
0x14082a6a1 48898588000000 mov qword ptr [rbp + 0x88], rax
0x14082a6a8 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a6af 488d0d62732602 lea rcx, [rip + 0x2267362]
0x14082a6b6 e8c536e5ff call 0x14067dd80
0x14082a6bb f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x14082a6bf 8b4010 mov eax, dword ptr [rax + 0x10]
0x14082a6c2 48c785f000000003000000 mov qword ptr [rbp + 0xf0], 3
0x14082a6cd 488d15bc712602 lea rdx, [rip + 0x22671bc]
0x14082a6d4 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
0x14082a6db 48c785000100001b000000 mov qword ptr [rbp + 0x100], 0x1b
0x14082a6e6 488d0d0b732602 lea rcx, [rip + 0x226730b]
0x14082a6ed 48898d10010000 mov qword ptr [rbp + 0x110], rcx
0x14082a6f4 48c7851801000002000000 mov qword ptr [rbp + 0x118], 2
0x14082a6ff 488d8d80000000 lea rcx, [rbp + 0x80]
0x14082a706 48898d20010000 mov qword ptr [rbp + 0x120], rcx
0x14082a70d 48c7852801000001000000 mov qword ptr [rbp + 0x128], 1
0x14082a718 48c7853001000000000000 mov qword ptr [rbp + 0x130], 0
0x14082a723 48c785c000000000000000 mov qword ptr [rbp + 0xc0], 0
0x14082a72e 488995c8000000 mov qword ptr [rbp + 0xc8], rdx
0x14082a735 48c785d00000001b000000 mov qword ptr [rbp + 0xd0], 0x1b
0x14082a740 48c785d800000000000000 mov qword ptr [rbp + 0xd8], 0
0x14082a74b f30f7f85e0000000 movdqu xmmword ptr [rbp + 0xe0], xmm0
0x14082a753 c7850801000001000000 mov dword ptr [rbp + 0x108], 1
0x14082a75d 89850c010000 mov dword ptr [rbp + 0x10c], eax
0x14082a763 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082a76a 488d8d56010000 lea rcx, [rbp + 0x156]
0x14082a771 488d95c0000000 lea rdx, [rbp + 0xc0]
0x14082a778 e8b3fb5100 call 0x140d4a330
0x14082a77d e99bfaffff jmp 0x14082a21d
0x14082a782 488b75e0 mov rsi, qword ptr [rbp - 0x20]
0x14082a786 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x14082a78a 488b7df8 mov rdi, qword ptr [rbp - 8]
0x14082a78e 4885c9 test rcx, rcx
0x14082a791 48898d40010000 mov qword ptr [rbp + 0x140], rcx
0x14082a798 743a je 0x14082a7d4
0x14082a79a 488d048d13000000 lea rax, [rcx*4 + 0x13]
0x14082a7a2 4883e0f0 and rax, 0xfffffffffffffff0
0x14082a7a6 4801c1 add rcx, rax
0x14082a7a9 4883c111 add rcx, 0x11
0x14082a7ad 48898db0000000 mov qword ptr [rbp + 0xb0], rcx
0x14082a7b4 4889f1 mov rcx, rsi
0x14082a7b7 4829c1 sub rcx, rax
0x14082a7ba 48894d70 mov qword ptr [rbp + 0x70], rcx
0x14082a7be b810000000 mov eax, 0x10
0x14082a7c3 488985a8000000 mov qword ptr [rbp + 0xa8], rax
0x14082a7ca 4885ff test rdi, rdi
0x14082a7cd 7519 jne 0x14082a7e8
0x14082a7cf e9df010000 jmp 0x14082a9b3
0x14082a7d4 48c785a800000000000000 mov qword ptr [rbp + 0xa8], 0
0x14082a7df 4885ff test rdi, rdi
0x14082a7e2 0f84cb010000 je 0x14082a9b3
0x14082a7e8 660f6f06 movdqa xmm0, xmmword ptr [rsi]
0x14082a7ec 66440fd7f0 pmovmskb r14d, xmm0
0x14082a7f1 41f7d6 not r14d
0x14082a7f4 488d5e10 lea rbx, [rsi + 0x10]
0x14082a7f8 4c8d2591712602 lea r12, [rip + 0x2267191]
0x14082a7ff 4c8dad56010000 lea r13, [rbp + 0x156]
0x14082a806 4c8dbdc0000000 lea r15, [rbp + 0xc0]
0x14082a80d eb0a jmp 0x14082a819
0x14082a80f 90 nop
0x14082a810 48ffcf dec rdi
0x14082a813 0f849a010000 je 0x14082a9b3
0x14082a819 664585f6 test r14w, r14w
0x14082a81d 7411 je 0x14082a830
0x14082a81f 418d46ff lea eax, [r14 - 1]
0x14082a823 4421f0 and eax, r14d
0x14082a826 eb32 jmp 0x14082a85a
0x14082a828 0f1f840000000000 nop dword ptr [rax + rax]
0x14082a830 660f6f03 movdqa xmm0, xmmword ptr [rbx]
0x14082a834 66440fd7f0 pmovmskb r14d, xmm0
0x14082a839 4883c6c0 add rsi, -0x40
0x14082a83d 4883c310 add rbx, 0x10
0x14082a841 4181feffff0000 cmp r14d, 0xffff
0x14082a848 74e6 je 0x14082a830
0x14082a84a b9feffffff mov ecx, 0xfffffffe
0x14082a84f 4429f1 sub ecx, r14d
0x14082a852 41f7d6 not r14d
0x14082a855 4489f0 mov eax, r14d
0x14082a858 21c8 and eax, ecx
0x14082a85a 4489f1 mov ecx, r14d
0x14082a85d f30fbcc9 tzcnt ecx, ecx
0x14082a861 4189c6 mov r14d, eax
0x14082a864 c1e102 shl ecx, 2
0x14082a867 4889f0 mov rax, rsi
0x14082a86a 4829c8 sub rax, rcx
0x14082a86d 448b48fc mov r9d, dword ptr [rax - 4]
0x14082a871 44894d78 mov dword ptr [rbp + 0x78], r9d
0x14082a875 488b058c17e002 mov rax, qword ptr [rip + 0x2e0178c]
0x14082a87c 803800 cmp byte ptr [rax], 0
0x14082a87f 0f84e9000000 je 0x14082a96e
0x14082a885 488b051c1be002 mov rax, qword ptr [rip + 0x2e01b1c]
0x14082a88c 488b00 mov rax, qword ptr [rax]
0x14082a88f 4883f803 cmp rax, 3
0x14082a893 0f82d5000000 jb 0x14082a96e
0x14082a899 488d4578 lea rax, [rbp + 0x78]
0x14082a89d 48898580000000 mov qword ptr [rbp + 0x80], rax
0x14082a8a4 488d05c5a9ffff lea rax, [rip - 0x563b]
0x14082a8ab 48898588000000 mov qword ptr [rbp + 0x88], rax
0x14082a8b2 488d0df7702602 lea rcx, [rip + 0x22670f7]
0x14082a8b9 e8c234e5ff call 0x14067dd80
0x14082a8be f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x14082a8c2 8b4010 mov eax, dword ptr [rax + 0x10]
0x14082a8c5 48c785f000000003000000 mov qword ptr [rbp + 0xf0], 3
0x14082a8d0 488d15b96f2602 lea rdx, [rip + 0x2266fb9]
0x14082a8d7 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
0x14082a8de 48c785000100001b000000 mov qword ptr [rbp + 0x100], 0x1b
0x14082a8e9 4c89a510010000 mov qword ptr [rbp + 0x110], r12
0x14082a8f0 48c7851801000002000000 mov qword ptr [rbp + 0x118], 2
0x14082a8fb 488d8d80000000 lea rcx, [rbp + 0x80]
0x14082a902 48898d20010000 mov qword ptr [rbp + 0x120], rcx
0x14082a909 48c7852801000001000000 mov qword ptr [rbp + 0x128], 1
0x14082a914 48c7853001000000000000 mov qword ptr [rbp + 0x130], 0
0x14082a91f 48c785c000000000000000 mov qword ptr [rbp + 0xc0], 0
0x14082a92a 488995c8000000 mov qword ptr [rbp + 0xc8], rdx
0x14082a931 48c785d00000001b000000 mov qword ptr [rbp + 0xd0], 0x1b
0x14082a93c 48c785d800000000000000 mov qword ptr [rbp + 0xd8], 0
0x14082a947 f30f7f85e0000000 movdqu xmmword ptr [rbp + 0xe0], xmm0
0x14082a94f c7850801000001000000 mov dword ptr [rbp + 0x108], 1
0x14082a959 89850c010000 mov dword ptr [rbp + 0x10c], eax
0x14082a95f 4c89e9 mov rcx, r13
0x14082a962 4c89fa mov rdx, r15
0x14082a965 e8c6f95100 call 0x140d4a330
0x14082a96a 448b4d78 mov r9d, dword ptr [rbp + 0x78]
0x14082a96e 4c89f9 mov rcx, r15
0x14082a971 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x14082a978 4c8b85b8000000 mov r8, qword ptr [rbp + 0xb8]
0x14082a97f e82c603100 call 0x140b409b0
0x14082a984 80bde000000011 cmp byte ptr [rbp + 0xe0], 0x11
0x14082a98b 0f847ffeffff je 0x14082a810
0x14082a991 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
0x14082a998 f048ff08 lock dec qword ptr [rax]
0x14082a99c 0f856efeffff jne 0x14082a810
0x14082a9a2 488d8df8000000 lea rcx, [rbp + 0xf8]
0x14082a9a9 e8721c3300 call 0x140b5c620
0x14082a9ae e95dfeffff jmp 0x14082a810
0x14082a9b3 4883bd4001000000 cmp qword ptr [rbp + 0x140], 0
0x14082a9bb 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
0x14082a9c2 7415 je 0x14082a9d9
0x14082a9c4 4885d2 test rdx, rdx
0x14082a9c7 7410 je 0x14082a9d9
0x14082a9c9 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x14082a9cd 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
0x14082a9d4 e84792deff call 0x140613c20
0x14082a9d9 488b7540 mov rsi, qword ptr [rbp + 0x40]
0x14082a9dd 488b4d48 mov rcx, qword ptr [rbp + 0x48]
0x14082a9e1 488b5d58 mov rbx, qword ptr [rbp + 0x58]
0x14082a9e5 4885c9 test rcx, rcx
0x14082a9e8 4c8b6d38 mov r13, qword ptr [rbp + 0x38]
0x14082a9ec 48898d40010000 mov qword ptr [rbp + 0x140], rcx
0x14082a9f3 743d je 0x14082aa32
0x14082a9f5 488d048d13000000 lea rax, [rcx*4 + 0x13]
0x14082a9fd 4883e0f0 and rax, 0xfffffffffffffff0
0x14082aa01 4801c1 add rcx, rax
0x14082aa04 4883c111 add rcx, 0x11
0x14082aa08 48898db0000000 mov qword ptr [rbp + 0xb0], rcx
0x14082aa0f 4889f1 mov rcx, rsi
0x14082aa12 4829c1 sub rcx, rax
0x14082aa15 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
0x14082aa1c b810000000 mov eax, 0x10
0x14082aa21 488985b8000000 mov qword ptr [rbp + 0xb8], rax
0x14082aa28 4885db test rbx, rbx
0x14082aa2b 7519 jne 0x14082aa46
0x14082aa2d e9d9000000 jmp 0x14082ab0b
0x14082aa32 48c785b800000000000000 mov qword ptr [rbp + 0xb8], 0
0x14082aa3d 4885db test rbx, rbx
0x14082aa40 0f84c5000000 je 0x14082ab0b
0x14082aa46 660f6f06 movdqa xmm0, xmmword ptr [rsi]
0x14082aa4a 66440fd7e0 pmovmskb r12d, xmm0
0x14082aa4f 41f7d4 not r12d
0x14082aa52 4c8d7e10 lea r15, [rsi + 0x10]
0x14082aa56 488dbdf8000000 lea rdi, [rbp + 0xf8]
0x14082aa5d 4c8db5c0000000 lea r14, [rbp + 0xc0]
0x14082aa64 eb13 jmp 0x14082aa79
0x14082aa66 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14082aa70 48ffcb dec rbx
0x14082aa73 0f8492000000 je 0x14082ab0b
0x14082aa79 664585e4 test r12w, r12w
0x14082aa7d 7411 je 0x14082aa90
0x14082aa7f 418d4424ff lea eax, [r12 - 1]
0x14082aa84 4421e0 and eax, r12d
0x14082aa87 eb32 jmp 0x14082aabb
0x14082aa89 0f1f8000000000 nop dword ptr [rax]
0x14082aa90 66410f6f07 movdqa xmm0, xmmword ptr [r15]
0x14082aa95 66440fd7e0 pmovmskb r12d, xmm0
0x14082aa9a 4883c6c0 add rsi, -0x40
0x14082aa9e 4983c710 add r15, 0x10
0x14082aaa2 4181fcffff0000 cmp r12d, 0xffff
0x14082aaa9 74e5 je 0x14082aa90
0x14082aaab b9feffffff mov ecx, 0xfffffffe
0x14082aab0 4429e1 sub ecx, r12d
0x14082aab3 41f7d4 not r12d
0x14082aab6 4489e0 mov eax, r12d
0x14082aab9 21c8 and eax, ecx
0x14082aabb 4489e1 mov ecx, r12d
0x14082aabe f30fbcc9 tzcnt ecx, ecx
0x14082aac2 4189c4 mov r12d, eax
0x14082aac5 c1e102 shl ecx, 2
0x14082aac8 4889f0 mov rax, rsi
0x14082aacb 4829c8 sub rax, rcx
0x14082aace 448b48fc mov r9d, dword ptr [rax - 4]
0x14082aad2 4c89f1 mov rcx, r14
0x14082aad5 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x14082aadc 4d89e8 mov r8, r13
0x14082aadf e8cc5e3100 call 0x140b409b0
0x14082aae4 80bde000000011 cmp byte ptr [rbp + 0xe0], 0x11
0x14082aaeb 7483 je 0x14082aa70
0x14082aaed 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
0x14082aaf4 f048ff08 lock dec qword ptr [rax]
0x14082aaf8 0f8572ffffff jne 0x14082aa70
0x14082aafe 4889f9 mov rcx, rdi
0x14082ab01 e81a1b3300 call 0x140b5c620
0x14082ab06 e965ffffff jmp 0x14082aa70
0x14082ab0b 4883bd4001000000 cmp qword ptr [rbp + 0x140], 0
0x14082ab13 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
0x14082ab1a 7418 je 0x14082ab34
0x14082ab1c 4885d2 test rdx, rdx
0x14082ab1f 7413 je 0x14082ab34
0x14082ab21 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x14082ab28 4c8b85b8000000 mov r8, qword ptr [rbp + 0xb8]
0x14082ab2f e8ec90deff call 0x140613c20
0x14082ab34 48837d2800 cmp qword ptr [rbp + 0x28], 0
0x14082ab39 7444 je 0x14082ab7f
0x14082ab3b e8f0d53e00 call 0x140c18130
0x14082ab40 4889c1 mov rcx, rax
0x14082ab43 488b00 mov rax, qword ptr [rax]
0x14082ab46 4883f801 cmp rax, 1
0x14082ab4a 755d jne 0x14082aba9
0x14082ab4c 4883c108 add rcx, 8
0x14082ab50 48833900 cmp qword ptr [rcx], 0
0x14082ab54 0f8589000000 jne 0x14082abe3
0x14082ab5a 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x14082ab61 48898da0000000 mov qword ptr [rbp + 0xa0], rcx
0x14082ab68 4883c108 add rcx, 8
0x14082ab6c 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x14082ab70 e87bc53e00 call 0x140c170f0
0x14082ab75 488b85a0000000 mov rax, qword ptr [rbp + 0xa0]
0x14082ab7c 48ff00 inc qword ptr [rax]
0x14082ab7f 0f28b560010000 movaps xmm6, xmmword ptr [rbp + 0x160]
0x14082ab86 0f28bd70010000 movaps xmm7, xmmword ptr [rbp + 0x170]
0x14082ab8d 440f288580010000 movaps xmm8, xmmword ptr [rbp + 0x180]
0x14082ab95 4881c418020000 add rsp, 0x218
0x14082ab9c 5b pop rbx
0x14082ab9d 5f pop rdi
0x14082ab9e 5e pop rsi
0x14082ab9f 415c pop r12
0x14082aba1 415d pop r13
0x14082aba3 415e pop r14
0x14082aba5 415f pop r15
0x14082aba7 5d pop rbp
0x14082aba8 c3 ret
0x14082aba9 4885c0 test rax, rax
0x14082abac 7529 jne 0x14082abd7
0x14082abae 31d2 xor edx, edx
0x14082abb0 e8cb790c02 call 0x1428f2580
0x14082abb5 4889c1 mov rcx, rax
0x14082abb8 4885c0 test rax, rax
0x14082abbb 7593 jne 0x14082ab50
0x14082abbd eb18 jmp 0x14082abd7
0x14082abbf 4885c0 test rax, rax
0x14082abc2 7513 jne 0x14082abd7
0x14082abc4 31d2 xor edx, edx
0x14082abc6 e8b5790c02 call 0x1428f2580
0x14082abcb 4889c1 mov rcx, rax
0x14082abce 4885c0 test rax, rax
0x14082abd1 0f85a3eeffff jne 0x140829a7a
0x14082abd7 488d0dea673f02 lea rcx, [rip + 0x23f67ea]
0x14082abde e8edd40a02 call 0x1428d80d0
0x14082abe3 488d0d4e683f02 lea rcx, [rip + 0x23f684e]
0x14082abea e861e60a02 call 0x1428d9250
0x14082abef 4c8d85e0000000 lea r8, [rbp + 0xe0]
0x14082abf6 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x14082abfd 4889da mov rdx, rbx
0x14082ac00 41b101 mov r9b, 1
0x14082ac03 e8b8960902 call 0x1428c42c0
0x14082ac08 e951efffff jmp 0x140829b5e
0x14082ac0d 4c8d85e0000000 lea r8, [rbp + 0xe0]
0x14082ac14 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x14082ac1b 4889da mov rdx, rbx
0x14082ac1e 41b101 mov r9b, 1
0x14082ac21 e89a960902 call 0x1428c42c0
0x14082ac26 e92af0ffff jmp 0x140829c55
0x14082ac2b 488b96c8000000 mov rdx, qword ptr [rsi + 0xc8]
0x14082ac32 4c6986d000000060010000 imul r8, qword ptr [rsi + 0xd0], 0x160
0x14082ac3d 4901d0 add r8, rdx
0x14082ac40 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082ac47 4c8d0d8a672602 lea r9, [rip + 0x226678a]
0x14082ac4e 488db580000000 lea rsi, [rbp + 0x80]
0x14082ac55 4889f1 mov rcx, rsi
0x14082ac58 e8c377a300 call 0x141262420
0x14082ac5d 48897510 mov qword ptr [rbp + 0x10], rsi
0x14082ac61 488d056844a300 lea rax, [rip + 0xa34468]
0x14082ac68 48894518 mov qword ptr [rbp + 0x18], rax
0x14082ac6c 488d05656f2602 lea rax, [rip + 0x2266f65]
0x14082ac73 488985c0000000 mov qword ptr [rbp + 0xc0], rax
0x14082ac7a 48c785c800000001000000 mov qword ptr [rbp + 0xc8], 1
0x14082ac85 48c785e000000000000000 mov qword ptr [rbp + 0xe0], 0
0x14082ac90 488d4510 lea rax, [rbp + 0x10]
0x14082ac94 488985d0000000 mov qword ptr [rbp + 0xd0], rax
0x14082ac9b 48c785d800000001000000 mov qword ptr [rbp + 0xd8], 1
0x14082aca6 488d153b6f2602 lea rdx, [rip + 0x2266f3b]
0x14082acad 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x14082acb4 e8d7e60a02 call 0x1428d9390
0x14082acb9 0f0b ud2
0x14082acbb 0f1f440000 nop dword ptr [rax + rax]
0x14082acc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082acc5 55 push rbp
0x14082acc6 4157 push r15
0x14082acc8 4156 push r14
0x14082acca 4155 push r13
0x14082accc 4154 push r12
0x14082acce 56 push rsi
0x14082accf 57 push rdi
0x14082acd0 53 push rbx
0x14082acd1 4883ec68 sub rsp, 0x68
0x14082acd5 488daa80000000 lea rbp, [rdx + 0x80]
0x14082acdc 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082ace3 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082ace8 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082acee 488b8540010000 mov rax, qword ptr [rbp + 0x140]
0x14082acf5 48ff00 inc qword ptr [rax]
0x14082acf8 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14082acfd 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082ad02 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x14082ad08 4883c468 add rsp, 0x68
0x14082ad0c 5b pop rbx
0x14082ad0d 5f pop rdi
0x14082ad0e 5e pop rsi
0x14082ad0f 415c pop r12
0x14082ad11 415d pop r13
0x14082ad13 415e pop r14
0x14082ad15 415f pop r15
0x14082ad17 5d pop rbp
0x14082ad18 c3 ret
0x14082ad19 0f1f8000000000 nop dword ptr [rax]
0x14082ad20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082ad25 55 push rbp
0x14082ad26 4157 push r15
0x14082ad28 4156 push r14
0x14082ad2a 4155 push r13
0x14082ad2c 4154 push r12
0x14082ad2e 56 push rsi
0x14082ad2f 57 push rdi
0x14082ad30 53 push rbx
0x14082ad31 4883ec68 sub rsp, 0x68
0x14082ad35 488daa80000000 lea rbp, [rdx + 0x80]
0x14082ad3c 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082ad43 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082ad48 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082ad4e 48837d2800 cmp qword ptr [rbp + 0x28], 0
0x14082ad53 7420 je 0x14082ad75
0x14082ad55 488d4530 lea rax, [rbp + 0x30]
0x14082ad59 488945c8 mov qword ptr [rbp - 0x38], rax
0x14082ad5d 488d45c8 lea rax, [rbp - 0x38]
0x14082ad61 488945c0 mov qword ptr [rbp - 0x40], rax
0x14082ad65 488d0da4672602 lea rcx, [rip + 0x22667a4]
0x14082ad6c 488d55c0 lea rdx, [rbp - 0x40]
0x14082ad70 e84bcea200 call 0x141257bc0
0x14082ad75 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14082ad7a 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082ad7f 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x14082ad85 4883c468 add rsp, 0x68
0x14082ad89 5b pop rbx
0x14082ad8a 5f pop rdi
0x14082ad8b 5e pop rsi
0x14082ad8c 415c pop r12
0x14082ad8e 415d pop r13
0x14082ad90 415e pop r14
0x14082ad92 415f pop r15
0x14082ad94 5d pop rbp
0x14082ad95 c3 ret
0x14082ad96 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14082ada0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082ada5 55 push rbp
0x14082ada6 4157 push r15
0x14082ada8 4156 push r14
0x14082adaa 4155 push r13
0x14082adac 4154 push r12
0x14082adae 56 push rsi
0x14082adaf 57 push rdi
0x14082adb0 53 push rbx
0x14082adb1 4883ec68 sub rsp, 0x68
0x14082adb5 488daa80000000 lea rbp, [rdx + 0x80]
0x14082adbc 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082adc3 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082adc8 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082adce 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x14082add5 e866b8daff call 0x1405d6640
0x14082adda 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14082ade0 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082ade5 66440f6f442430 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14082adec 4883c468 add rsp, 0x68
0x14082adf0 5b pop rbx
0x14082adf1 5f pop rdi
0x14082adf2 5e pop rsi
0x14082adf3 415c pop r12
0x14082adf5 415d pop r13
0x14082adf7 415e pop r14
0x14082adf9 415f pop r15
0x14082adfb 5d pop rbp
0x14082adfc c3 ret
0x14082adfd 0f1f00 nop dword ptr [rax]
0x14082ae00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082ae05 55 push rbp
0x14082ae06 4157 push r15
0x14082ae08 4156 push r14
0x14082ae0a 4155 push r13
0x14082ae0c 4154 push r12
0x14082ae0e 56 push rsi
0x14082ae0f 57 push rdi
0x14082ae10 53 push rbx
0x14082ae11 4883ec68 sub rsp, 0x68
0x14082ae15 488daa80000000 lea rbp, [rdx + 0x80]
0x14082ae1c 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082ae23 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082ae28 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082ae2e c6855501000001 mov byte ptr [rbp + 0x155], 1
0x14082ae35 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x14082ae3c e8ffb7daff call 0x1405d6640
0x14082ae41 c6855501000001 mov byte ptr [rbp + 0x155], 1
0x14082ae48 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14082ae4e 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082ae53 66440f6f442430 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14082ae5a 4883c468 add rsp, 0x68
0x14082ae5e 5b pop rbx
0x14082ae5f 5f pop rdi
0x14082ae60 5e pop rsi
0x14082ae61 415c pop r12
0x14082ae63 415d pop r13
0x14082ae65 415e pop r14
0x14082ae67 415f pop r15
0x14082ae69 5d pop rbp
0x14082ae6a c3 ret
0x14082ae6b 0f1f440000 nop dword ptr [rax + rax]
0x14082ae70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082ae75 55 push rbp
0x14082ae76 4157 push r15
0x14082ae78 4156 push r14
0x14082ae7a 4155 push r13
0x14082ae7c 4154 push r12
0x14082ae7e 56 push rsi
0x14082ae7f 57 push rdi
0x14082ae80 53 push rbx
0x14082ae81 4883ec68 sub rsp, 0x68
0x14082ae85 488daa80000000 lea rbp, [rdx + 0x80]
0x14082ae8c 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082ae93 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082ae98 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082ae9e 4883bd4001000000 cmp qword ptr [rbp + 0x140], 0
0x14082aea6 7421 je 0x14082aec9
0x14082aea8 4883bdb000000000 cmp qword ptr [rbp + 0xb0], 0
0x14082aeb0 7417 je 0x14082aec9
0x14082aeb2 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x14082aeb6 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
0x14082aebd 4c8b85a8000000 mov r8, qword ptr [rbp + 0xa8]
0x14082aec4 e8578ddeff call 0x140613c20
0x14082aec9 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082aed0 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14082aed5 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082aeda 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x14082aee0 4883c468 add rsp, 0x68
0x14082aee4 5b pop rbx
0x14082aee5 5f pop rdi
0x14082aee6 5e pop rsi
0x14082aee7 415c pop r12
0x14082aee9 415d pop r13
0x14082aeeb 415e pop r14
0x14082aeed 415f pop r15
0x14082aeef 5d pop rbp
0x14082aef0 c3 ret
0x14082aef1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14082af00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082af05 55 push rbp
0x14082af06 4157 push r15
0x14082af08 4156 push r14
0x14082af0a 4155 push r13
0x14082af0c 4154 push r12
0x14082af0e 56 push rsi
0x14082af0f 57 push rdi
0x14082af10 53 push rbx
0x14082af11 4883ec68 sub rsp, 0x68
0x14082af15 488daa80000000 lea rbp, [rdx + 0x80]
0x14082af1c 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082af23 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082af28 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082af2e 4883bd4001000000 cmp qword ptr [rbp + 0x140], 0
0x14082af36 7424 je 0x14082af5c
0x14082af38 4883bdb000000000 cmp qword ptr [rbp + 0xb0], 0
0x14082af40 741a je 0x14082af5c
0x14082af42 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x14082af49 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
0x14082af50 4c8b85b8000000 mov r8, qword ptr [rbp + 0xb8]
0x14082af57 e8c48cdeff call 0x140613c20
0x14082af5c c6855701000000 mov byte ptr [rbp + 0x157], 0
0x14082af63 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14082af68 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082af6d 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x14082af73 4883c468 add rsp, 0x68
0x14082af77 5b pop rbx
0x14082af78 5f pop rdi
0x14082af79 5e pop rsi
0x14082af7a 415c pop r12
0x14082af7c 415d pop r13
0x14082af7e 415e pop r14
0x14082af80 415f pop r15
0x14082af82 5d pop rbp
0x14082af83 c3 ret
0x14082af84 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14082af90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082af95 55 push rbp
0x14082af96 4157 push r15
0x14082af98 4156 push r14
0x14082af9a 4155 push r13
0x14082af9c 4154 push r12
0x14082af9e 56 push rsi
0x14082af9f 57 push rdi
0x14082afa0 53 push rbx
0x14082afa1 4883ec68 sub rsp, 0x68
0x14082afa5 488daa80000000 lea rbp, [rdx + 0x80]
0x14082afac 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082afb3 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082afb8 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082afbe 488b85a0000000 mov rax, qword ptr [rbp + 0xa0]
0x14082afc5 48ff00 inc qword ptr [rax]
0x14082afc8 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14082afcd 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082afd2 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x14082afd8 4883c468 add rsp, 0x68
0x14082afdc 5b pop rbx
0x14082afdd 5f pop rdi
0x14082afde 5e pop rsi
0x14082afdf 415c pop r12
0x14082afe1 415d pop r13
0x14082afe3 415e pop r14
0x14082afe5 415f pop r15
0x14082afe7 5d pop rbp
0x14082afe8 c3 ret
0x14082afe9 0f1f8000000000 nop dword ptr [rax]
0x14082aff0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082aff5 55 push rbp
0x14082aff6 4157 push r15
0x14082aff8 4156 push r14
0x14082affa 4155 push r13
0x14082affc 4154 push r12
0x14082affe 56 push rsi
0x14082afff 57 push rdi
0x14082b000 53 push rbx
0x14082b001 4883ec68 sub rsp, 0x68
0x14082b005 488daa80000000 lea rbp, [rdx + 0x80]
0x14082b00c 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082b013 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082b018 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082b01e 488b9580000000 mov rdx, qword ptr [rbp + 0x80]
0x14082b025 4885d2 test rdx, rdx
0x14082b028 7416 je 0x14082b040
0x14082b02a 488b8d88000000 mov rcx, qword ptr [rbp + 0x88]
0x14082b031 48c1e204 shl rdx, 4
0x14082b035 41b808000000 mov r8d, 8
0x14082b03b e8e08bdeff call 0x140613c20
0x14082b040 c6855701000001 mov byte ptr [rbp + 0x157], 1
0x14082b047 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14082b04c 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082b051 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x14082b057 4883c468 add rsp, 0x68
0x14082b05b 5b pop rbx
0x14082b05c 5f pop rdi
0x14082b05d 5e pop rsi
0x14082b05e 415c pop r12
0x14082b060 415d pop r13
0x14082b062 415e pop r14
0x14082b064 415f pop r15
0x14082b066 5d pop rbp
0x14082b067 c3 ret
0x14082b068 0f1f840000000000 nop dword ptr [rax + rax]
0x14082b070 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082b075 55 push rbp
0x14082b076 4157 push r15
0x14082b078 4156 push r14
0x14082b07a 4155 push r13
0x14082b07c 4154 push r12
0x14082b07e 56 push rsi
0x14082b07f 57 push rdi
0x14082b080 53 push rbx
0x14082b081 4883ec68 sub rsp, 0x68
0x14082b085 488daa80000000 lea rbp, [rdx + 0x80]
0x14082b08c 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082b093 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082b098 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082b09e 80bd5701000000 cmp byte ptr [rbp + 0x157], 0
0x14082b0a5 7439 je 0x14082b0e0
0x14082b0a7 488d4d40 lea rcx, [rbp + 0x40]
0x14082b0ab e890b5daff call 0x1405d6640
0x14082b0b0 0fb6854f010000 movzx eax, byte ptr [rbp + 0x14f]
0x14082b0b7 888555010000 mov byte ptr [rbp + 0x155], al
0x14082b0bd 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14082b0c3 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082b0c8 66440f6f442430 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14082b0cf 4883c468 add rsp, 0x68
0x14082b0d3 5b pop rbx
0x14082b0d4 5f pop rdi
0x14082b0d5 5e pop rsi
0x14082b0d6 415c pop r12
0x14082b0d8 415d pop r13
0x14082b0da 415e pop r14
0x14082b0dc 415f pop r15
0x14082b0de 5d pop rbp
0x14082b0df c3 ret
0x14082b0e0 0fb6854f010000 movzx eax, byte ptr [rbp + 0x14f]
0x14082b0e7 888555010000 mov byte ptr [rbp + 0x155], al
0x14082b0ed 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14082b0f3 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082b0f8 66440f6f442430 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14082b0ff 4883c468 add rsp, 0x68
0x14082b103 5b pop rbx
0x14082b104 5f pop rdi
0x14082b105 5e pop rsi
0x14082b106 415c pop r12
0x14082b108 415d pop r13
0x14082b10a 415e pop r14
0x14082b10c 415f pop r15
0x14082b10e 5d pop rbp
0x14082b10f c3 ret
0x14082b110 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14082b115 55 push rbp
0x14082b116 4157 push r15
0x14082b118 4156 push r14
0x14082b11a 4155 push r13
0x14082b11c 4154 push r12
0x14082b11e 56 push rsi
0x14082b11f 57 push rdi
0x14082b120 53 push rbx
0x14082b121 4883ec68 sub rsp, 0x68
0x14082b125 488daa80000000 lea rbp, [rdx + 0x80]
0x14082b12c 66440f7f442430 movdqa xmmword ptr [rsp + 0x30], xmm8
0x14082b133 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
0x14082b138 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14082b13e 80bd5501000000 cmp byte ptr [rbp + 0x155], 0
0x14082b145 742c je 0x14082b173
0x14082b147 488d4de0 lea rcx, [rbp - 0x20]
0x14082b14b e8f0b4daff call 0x1405d6640
0x14082b150 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14082b156 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082b15b 66440f6f442430 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14082b162 4883c468 add rsp, 0x68
0x14082b166 5b pop rbx
0x14082b167 5f pop rdi
0x14082b168 5e pop rsi
0x14082b169 415c pop r12
0x14082b16b 415d pop r13
0x14082b16d 415e pop r14
0x14082b16f 415f pop r15
0x14082b171 5d pop rbp
0x14082b172 c3 ret
0x14082b173 660f6f742450 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14082b179 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14082b17e 66440f6f442430 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14082b185 4883c468 add rsp, 0x68
0x14082b189 5b pop rbx
0x14082b18a 5f pop rdi
0x14082b18b 5e pop rsi
0x14082b18c 415c pop r12
0x14082b18e 415d pop r13
0x14082b190 415e pop r14
0x14082b192 415f pop r15
0x14082b194 5d pop rbp
0x14082b195 c3 ret
