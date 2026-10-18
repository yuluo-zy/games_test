0x140b218a0 55 push rbp
0x140b218a1 4157 push r15
0x140b218a3 4156 push r14
0x140b218a5 4155 push r13
0x140b218a7 4154 push r12
0x140b218a9 56 push rsi
0x140b218aa 57 push rdi
0x140b218ab 53 push rbx
0x140b218ac 4881ece8000000 sub rsp, 0xe8
0x140b218b3 488dac2480000000 lea rbp, [rsp + 0x80]
0x140b218bb 66440f7f4550 movdqa xmmword ptr [rbp + 0x50], xmm8
0x140b218c1 660f7f7d40 movdqa xmmword ptr [rbp + 0x40], xmm7
0x140b218c6 660f7f7530 movdqa xmmword ptr [rbp + 0x30], xmm6
0x140b218cb 48c74528feffffff mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x140b218d3 4c89c6 mov rsi, r8
0x140b218d6 4889d7 mov rdi, rdx
0x140b218d9 4889cb mov rbx, rcx
0x140b218dc 8b01 mov eax, dword ptr [rcx]
0x140b218de 3dfdffff3f cmp eax, 0x3ffffffd
0x140b218e3 0f87a7010000 ja 0x140b21a90
0x140b218e9 8d4801 lea ecx, [rax + 1]
0x140b218ec f00fb10b lock cmpxchg dword ptr [rbx], ecx
0x140b218f0 0f859a010000 jne 0x140b21a90
0x140b218f6 0fb64308 movzx eax, byte ptr [rbx + 8]
0x140b218fa 48895d20 mov qword ptr [rbp + 0x20], rbx
0x140b218fe 48837b2800 cmp qword ptr [rbx + 0x28], 0
0x140b21903 0f84a2010000 je 0x140b21aab
0x140b21909 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
0x140b21913 488945e0 mov qword ptr [rbp - 0x20], rax
0x140b21917 660fefc0 pxor xmm0, xmm0
0x140b2191b 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x140b21920 c6450800 mov byte ptr [rbp + 8], 0
0x140b21924 48b87713d038e6212845 movabs rax, 0x452821e638d01377
0x140b2192e 488945e8 mov qword ptr [rbp - 0x18], rax
0x140b21932 48b86c0ce934cf6654be movabs rax, 0xbe5466cf34e90c6c
0x140b2193c 488945f0 mov qword ptr [rbp - 0x10], rax
0x140b21940 48b8dd507cc9b729acc0 movabs rax, 0xc0ac29b7c97c50dd
0x140b2194a 488945f8 mov qword ptr [rbp - 8], rax
0x140b2194e 48b8170947b5b5d5843f movabs rax, 0x3f84d5b5b5470917
0x140b21958 48894500 mov qword ptr [rbp], rax
0x140b2195c 4c8d05edb61702 lea r8, [rip + 0x217b6ed]
0x140b21963 488d55d0 lea rdx, [rbp - 0x30]
0x140b21967 4889f9 mov rcx, rdi
0x140b2196a ff5640 call qword ptr [rsi + 0x40]
0x140b2196d 807d0800 cmp byte ptr [rbp + 8], 0
0x140b21971 7418 je 0x140b2198b
0x140b21973 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140b21977 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140b2197b 48334dd0 xor rcx, qword ptr [rbp - 0x30]
0x140b2197f 483345d8 xor rax, qword ptr [rbp - 0x28]
0x140b21983 48f7e1 mul rcx
0x140b21986 4831c2 xor rdx, rax
0x140b21989 eb04 jmp 0x140b2198f
0x140b2198b 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x140b2198f 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140b21993 4c8b6010 mov r12, qword ptr [rax + 0x10]
0x140b21997 4c8b7018 mov r14, qword ptr [rax + 0x18]
0x140b2199b 4d89f7 mov r15, r14
0x140b2199e 4921d7 and r15, rdx
0x140b219a1 48c1ea39 shr rdx, 0x39
0x140b219a5 660f6ec2 movd xmm0, edx
0x140b219a9 660f60c0 punpcklbw xmm0, xmm0
0x140b219ad f20f70c000 pshuflw xmm0, xmm0, 0
0x140b219b2 660f70f000 pshufd xmm6, xmm0, 0
0x140b219b7 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x140b219bf 660f76ff pcmpeqd xmm7, xmm7
0x140b219c3 f3470f6f043c movdqu xmm8, xmmword ptr [r12 + r15]
0x140b219c9 66410f6fc0 movdqa xmm0, xmm8
0x140b219ce 660f74c6 pcmpeqb xmm0, xmm6
0x140b219d2 66440fd7e8 pmovmskb r13d, xmm0
0x140b219d7 4585ed test r13d, r13d
0x140b219da 743d je 0x140b21a19
0x140b219dc 0f1f4000 nop dword ptr [rax]
0x140b219e0 f3410fbcc5 tzcnt eax, r13d
0x140b219e5 4c01f8 add rax, r15
0x140b219e8 4c21f0 and rax, r14
0x140b219eb 48c1e004 shl rax, 4
0x140b219ef 4c89e3 mov rbx, r12
0x140b219f2 4829c3 sub rbx, rax
0x140b219f5 4c8b43f0 mov r8, qword ptr [rbx - 0x10]
0x140b219f9 4c8b4bf8 mov r9, qword ptr [rbx - 8]
0x140b219fd 4889f9 mov rcx, rdi
0x140b21a00 4889f2 mov rdx, rsi
0x140b21a03 e8b89d1e00 call 0x140d0b7c0
0x140b21a08 84c0 test al, al
0x140b21a0a 7537 jne 0x140b21a43
0x140b21a0c 418d45ff lea eax, [r13 - 1]
0x140b21a10 664421e8 and ax, r13w
0x140b21a14 4189c5 mov r13d, eax
0x140b21a17 75c7 jne 0x140b219e0
0x140b21a19 66440f74c7 pcmpeqb xmm8, xmm7
0x140b21a1e 66410fd7c0 pmovmskb eax, xmm8
0x140b21a23 85c0 test eax, eax
0x140b21a25 0f8580000000 jne 0x140b21aab
0x140b21a2b 488b4518 mov rax, qword ptr [rbp + 0x18]
0x140b21a2f 4901c7 add r15, rax
0x140b21a32 4983c710 add r15, 0x10
0x140b21a36 4883c010 add rax, 0x10
0x140b21a3a 48894518 mov qword ptr [rbp + 0x18], rax
0x140b21a3e 4d21f7 and r15, r14
0x140b21a41 eb80 jmp 0x140b219c3
0x140b21a43 488b73f0 mov rsi, qword ptr [rbx - 0x10]
0x140b21a47 488b7bf8 mov rdi, qword ptr [rbx - 8]
0x140b21a4b baffffffff mov edx, 0xffffffff
0x140b21a50 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x140b21a54 f00fc111 lock xadd dword ptr [rcx], edx
0x140b21a58 ffca dec edx
0x140b21a5a 89d0 mov eax, edx
0x140b21a5c 25ffffffbf and eax, 0xbfffffff
0x140b21a61 f7d8 neg eax
0x140b21a63 0f801c020000 jo 0x140b21c85
0x140b21a69 4889f0 mov rax, rsi
0x140b21a6c 4889fa mov rdx, rdi
0x140b21a6f 0f287530 movaps xmm6, xmmword ptr [rbp + 0x30]
0x140b21a73 0f287d40 movaps xmm7, xmmword ptr [rbp + 0x40]
0x140b21a77 440f284550 movaps xmm8, xmmword ptr [rbp + 0x50]
0x140b21a7c 4881c4e8000000 add rsp, 0xe8
0x140b21a83 5b pop rbx
0x140b21a84 5f pop rdi
0x140b21a85 5e pop rsi
0x140b21a86 415c pop r12
0x140b21a88 415d pop r13
0x140b21a8a 415e pop r14
0x140b21a8c 415f pop r15
0x140b21a8e 5d pop rbp
0x140b21a8f c3 ret 
0x140b21a90 4889d9 mov rcx, rbx
0x140b21a93 e8986fdb01 call 0x1428d8a30
0x140b21a98 0fb64308 movzx eax, byte ptr [rbx + 8]
0x140b21a9c 48895d20 mov qword ptr [rbp + 0x20], rbx
0x140b21aa0 48837b2800 cmp qword ptr [rbx + 0x28], 0
0x140b21aa5 0f855efeffff jne 0x140b21909
0x140b21aab baffffffff mov edx, 0xffffffff
0x140b21ab0 488b5d20 mov rbx, qword ptr [rbp + 0x20]
0x140b21ab4 f00fc113 lock xadd dword ptr [rbx], edx
0x140b21ab8 ffca dec edx
0x140b21aba 89d0 mov eax, edx
0x140b21abc 25ffffffbf and eax, 0xbfffffff
0x140b21ac1 f7d8 neg eax
0x140b21ac3 0f80e9030000 jo 0x140b21eb2
0x140b21ac9 b9ffffff3f mov ecx, 0x3fffffff
0x140b21ace 31c0 xor eax, eax
0x140b21ad0 f00fb10b lock cmpxchg dword ptr [rbx], ecx
0x140b21ad4 0f85f1030000 jne 0x140b21ecb
0x140b21ada 488b053fa9b002 mov rax, qword ptr [rip + 0x2b0a93f]
0x140b21ae1 488b00 mov rax, qword ptr [rax]
0x140b21ae4 48d1e0 shl rax, 1
0x140b21ae7 4885c0 test rax, rax
0x140b21aea 0f85b6030000 jne 0x140b21ea6
0x140b21af0 31c0 xor eax, eax
0x140b21af2 0fb64b08 movzx ecx, byte ptr [rbx + 8]
0x140b21af6 48895dc0 mov qword ptr [rbp - 0x40], rbx
0x140b21afa 8845c8 mov byte ptr [rbp - 0x38], al
0x140b21afd 48837b2800 cmp qword ptr [rbx + 0x28], 0
0x140b21b02 0f8487010000 je 0x140b21c8f
0x140b21b08 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
0x140b21b12 488945e0 mov qword ptr [rbp - 0x20], rax
0x140b21b16 660fefc0 pxor xmm0, xmm0
0x140b21b1a 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x140b21b1f c6450800 mov byte ptr [rbp + 8], 0
0x140b21b23 48b87713d038e6212845 movabs rax, 0x452821e638d01377
0x140b21b2d 488945e8 mov qword ptr [rbp - 0x18], rax
0x140b21b31 48b86c0ce934cf6654be movabs rax, 0xbe5466cf34e90c6c
0x140b21b3b 488945f0 mov qword ptr [rbp - 0x10], rax
0x140b21b3f 48b8dd507cc9b729acc0 movabs rax, 0xc0ac29b7c97c50dd
0x140b21b49 488945f8 mov qword ptr [rbp - 8], rax
0x140b21b4d 48b8170947b5b5d5843f movabs rax, 0x3f84d5b5b5470917
0x140b21b57 48894500 mov qword ptr [rbp], rax
0x140b21b5b 4c8d05eeb41702 lea r8, [rip + 0x217b4ee]
0x140b21b62 488d55d0 lea rdx, [rbp - 0x30]
0x140b21b66 4889f9 mov rcx, rdi
0x140b21b69 ff5640 call qword ptr [rsi + 0x40]
0x140b21b6c 807d0800 cmp byte ptr [rbp + 8], 0
0x140b21b70 7418 je 0x140b21b8a
0x140b21b72 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140b21b76 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140b21b7a 48334dd0 xor rcx, qword ptr [rbp - 0x30]
0x140b21b7e 483345d8 xor rax, qword ptr [rbp - 0x28]
0x140b21b82 48f7e1 mul rcx
0x140b21b85 4831c2 xor rdx, rax
0x140b21b88 eb04 jmp 0x140b21b8e
0x140b21b8a 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x140b21b8e 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140b21b92 4c8b6010 mov r12, qword ptr [rax + 0x10]
0x140b21b96 4c8b7018 mov r14, qword ptr [rax + 0x18]
0x140b21b9a 4d89f7 mov r15, r14
0x140b21b9d 4921d7 and r15, rdx
0x140b21ba0 48c1ea39 shr rdx, 0x39
0x140b21ba4 660f6ec2 movd xmm0, edx
0x140b21ba8 660f60c0 punpcklbw xmm0, xmm0
0x140b21bac f20f70c000 pshuflw xmm0, xmm0, 0
0x140b21bb1 660f70f000 pshufd xmm6, xmm0, 0
0x140b21bb6 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x140b21bbe 660f76ff pcmpeqd xmm7, xmm7
0x140b21bc2 f3470f6f043c movdqu xmm8, xmmword ptr [r12 + r15]
0x140b21bc8 66410f6fc0 movdqa xmm0, xmm8
0x140b21bcd 660f74c6 pcmpeqb xmm0, xmm6
0x140b21bd1 66440fd7e8 pmovmskb r13d, xmm0
0x140b21bd6 4585ed test r13d, r13d
0x140b21bd9 743e je 0x140b21c19
0x140b21bdb 0f1f440000 nop dword ptr [rax + rax]
0x140b21be0 f3410fbcc5 tzcnt eax, r13d
0x140b21be5 4c01f8 add rax, r15
0x140b21be8 4c21f0 and rax, r14
0x140b21beb 48c1e004 shl rax, 4
0x140b21bef 4c89e3 mov rbx, r12
0x140b21bf2 4829c3 sub rbx, rax
0x140b21bf5 4c8b43f0 mov r8, qword ptr [rbx - 0x10]
0x140b21bf9 4c8b4bf8 mov r9, qword ptr [rbx - 8]
0x140b21bfd 4889f9 mov rcx, rdi
0x140b21c00 4889f2 mov rdx, rsi
0x140b21c03 e8b89b1e00 call 0x140d0b7c0
0x140b21c08 84c0 test al, al
0x140b21c0a 7536 jne 0x140b21c42
0x140b21c0c 418d45ff lea eax, [r13 - 1]
0x140b21c10 664421e8 and ax, r13w
0x140b21c14 4189c5 mov r13d, eax
0x140b21c17 75c7 jne 0x140b21be0
0x140b21c19 66440f74c7 pcmpeqb xmm8, xmm7
0x140b21c1e 66410fd7c0 pmovmskb eax, xmm8
0x140b21c23 85c0 test eax, eax
0x140b21c25 7568 jne 0x140b21c8f
0x140b21c27 488b4518 mov rax, qword ptr [rbp + 0x18]
0x140b21c2b 4901c7 add r15, rax
0x140b21c2e 4983c710 add r15, 0x10
0x140b21c32 4883c010 add rax, 0x10
0x140b21c36 48894518 mov qword ptr [rbp + 0x18], rax
0x140b21c3a 4d21f7 and r15, r14
0x140b21c3d e980ffffff jmp 0x140b21bc2
0x140b21c42 488b73f0 mov rsi, qword ptr [rbx - 0x10]
0x140b21c46 488b7bf8 mov rdi, qword ptr [rbx - 8]
0x140b21c4a 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
0x140b21c4e 807dc800 cmp byte ptr [rbp - 0x38], 0
0x140b21c52 7516 jne 0x140b21c6a
0x140b21c54 488b05c5a7b002 mov rax, qword ptr [rip + 0x2b0a7c5]
0x140b21c5b 488b00 mov rax, qword ptr [rax]
0x140b21c5e 48d1e0 shl rax, 1
0x140b21c61 4885c0 test rax, rax
0x140b21c64 0f856e020000 jne 0x140b21ed8
0x140b21c6a ba010000c0 mov edx, 0xc0000001
0x140b21c6f f00fc111 lock xadd dword ptr [rcx], edx
0x140b21c73 81c2010000c0 add edx, 0xc0000001
0x140b21c79 81fa00000040 cmp edx, 0x40000000
0x140b21c7f 0f82e4fdffff jb 0x140b21a69
0x140b21c85 e8b670db01 call 0x1428d8d40
0x140b21c8a e9dafdffff jmp 0x140b21a69
0x140b21c8f 4889f9 mov rcx, rdi
0x140b21c92 4889f2 mov rdx, rsi
0x140b21c95 e8669b1e00 call 0x140d0b800
0x140b21c9a 4889c6 mov rsi, rax
0x140b21c9d 4889d7 mov rdi, rdx
0x140b21ca0 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
0x140b21caa 488945e0 mov qword ptr [rbp - 0x20], rax
0x140b21cae 660fefc0 pxor xmm0, xmm0
0x140b21cb2 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x140b21cb7 c6450800 mov byte ptr [rbp + 8], 0
0x140b21cbb 48b87713d038e6212845 movabs rax, 0x452821e638d01377
0x140b21cc5 488945e8 mov qword ptr [rbp - 0x18], rax
0x140b21cc9 48b86c0ce934cf6654be movabs rax, 0xbe5466cf34e90c6c
0x140b21cd3 488945f0 mov qword ptr [rbp - 0x10], rax
0x140b21cd7 48b8dd507cc9b729acc0 movabs rax, 0xc0ac29b7c97c50dd
0x140b21ce1 488945f8 mov qword ptr [rbp - 8], rax
0x140b21ce5 48b8170947b5b5d5843f movabs rax, 0x3f84d5b5b5470917
0x140b21cef 48894500 mov qword ptr [rbp], rax
0x140b21cf3 4c8d0556b31702 lea r8, [rip + 0x217b356]
0x140b21cfa 488d55d0 lea rdx, [rbp - 0x30]
0x140b21cfe 4889f1 mov rcx, rsi
0x140b21d01 ff5740 call qword ptr [rdi + 0x40]
0x140b21d04 807d0800 cmp byte ptr [rbp + 8], 0
0x140b21d08 742d je 0x140b21d37
0x140b21d0a 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140b21d0e 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140b21d12 48334dd0 xor rcx, qword ptr [rbp - 0x30]
0x140b21d16 483345d8 xor rax, qword ptr [rbp - 0x28]
0x140b21d1a 48f7e1 mul rcx
0x140b21d1d 4889d3 mov rbx, rdx
0x140b21d20 4831c3 xor rbx, rax
0x140b21d23 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140b21d27 4c8d7010 lea r14, [rax + 0x10]
0x140b21d2b 4883782000 cmp qword ptr [rax + 0x20], 0
0x140b21d30 751c jne 0x140b21d4e
0x140b21d32 e9bd010000 jmp 0x140b21ef4
0x140b21d37 488b5de0 mov rbx, qword ptr [rbp - 0x20]
0x140b21d3b 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140b21d3f 4c8d7010 lea r14, [rax + 0x10]
0x140b21d43 4883782000 cmp qword ptr [rax + 0x20], 0
0x140b21d48 0f84a6010000 je 0x140b21ef4
0x140b21d4e 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140b21d52 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140b21d56 4c8b6018 mov r12, qword ptr [rax + 0x18]
0x140b21d5a 4889d8 mov rax, rbx
0x140b21d5d 48c1e839 shr rax, 0x39
0x140b21d61 488945b8 mov qword ptr [rbp - 0x48], rax
0x140b21d65 660f6ec0 movd xmm0, eax
0x140b21d69 660f60c0 punpcklbw xmm0, xmm0
0x140b21d6d f20f70c000 pshuflw xmm0, xmm0, 0
0x140b21d72 660f70f000 pshufd xmm6, xmm0, 0
0x140b21d77 31d2 xor edx, edx
0x140b21d79 660f76ff pcmpeqd xmm7, xmm7
0x140b21d7d 4531ed xor r13d, r13d
0x140b21d80 48894da8 mov qword ptr [rbp - 0x58], rcx
0x140b21d84 48895518 mov qword ptr [rbp + 0x18], rdx
0x140b21d88 4c21e3 and rbx, r12
0x140b21d8b f3440f6f0419 movdqu xmm8, xmmword ptr [rcx + rbx]
0x140b21d91 66410f6fc0 movdqa xmm0, xmm8
0x140b21d96 660f74c6 pcmpeqb xmm0, xmm6
0x140b21d9a 66440fd7f8 pmovmskb r15d, xmm0
0x140b21d9f 4585ff test r15d, r15d
0x140b21da2 7450 je 0x140b21df4
0x140b21da4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140b21db0 f3410fbcc7 tzcnt eax, r15d
0x140b21db5 4801d8 add rax, rbx
0x140b21db8 4c21e0 and rax, r12
0x140b21dbb 498b0e mov rcx, qword ptr [r14]
0x140b21dbe 48c1e004 shl rax, 4
0x140b21dc2 4889ca mov rdx, rcx
0x140b21dc5 4829c2 sub rdx, rax
0x140b21dc8 4c8b42f0 mov r8, qword ptr [rdx - 0x10]
0x140b21dcc 48f7d8 neg rax
0x140b21dcf 4c8b4c01f8 mov r9, qword ptr [rcx + rax - 8]
0x140b21dd4 4889f1 mov rcx, rsi
0x140b21dd7 4889fa mov rdx, rdi
0x140b21dda e8e1991e00 call 0x140d0b7c0
0x140b21ddf 84c0 test al, al
0x140b21de1 0f8563feffff jne 0x140b21c4a
0x140b21de7 418d47ff lea eax, [r15 - 1]
0x140b21deb 664421f8 and ax, r15w
0x140b21def 4189c7 mov r15d, eax
0x140b21df2 75bc jne 0x140b21db0
0x140b21df4 4983fd01 cmp r13, 1
0x140b21df8 41bd01000000 mov r13d, 1
0x140b21dfe 741c je 0x140b21e1c
0x140b21e00 66410fd7c0 pmovmskb eax, xmm8
0x140b21e05 4531ed xor r13d, r13d
0x140b21e08 85c0 test eax, eax
0x140b21e0a 410f95c5 setne r13b
0x140b21e0e f30fbcc0 tzcnt eax, eax
0x140b21e12 4801d8 add rax, rbx
0x140b21e15 4c21e0 and rax, r12
0x140b21e18 488945b0 mov qword ptr [rbp - 0x50], rax
0x140b21e1c 66440f74c7 pcmpeqb xmm8, xmm7
0x140b21e21 66410fd7c0 pmovmskb eax, xmm8
0x140b21e26 85c0 test eax, eax
0x140b21e28 488b4da8 mov rcx, qword ptr [rbp - 0x58]
0x140b21e2c 488b4518 mov rax, qword ptr [rbp + 0x18]
0x140b21e30 7513 jne 0x140b21e45
0x140b21e32 4801c3 add rbx, rax
0x140b21e35 4883c310 add rbx, 0x10
0x140b21e39 4883c010 add rax, 0x10
0x140b21e3d 4889c2 mov rdx, rax
0x140b21e40 e93fffffff jmp 0x140b21d84
0x140b21e45 488b45b0 mov rax, qword ptr [rbp - 0x50]
0x140b21e49 803c0100 cmp byte ptr [rcx + rax], 0
0x140b21e4d 780c js 0x140b21e5b
0x140b21e4f 660f6f01 movdqa xmm0, xmmword ptr [rcx]
0x140b21e53 660fd7c0 pmovmskb eax, xmm0
0x140b21e57 f30fbcc0 tzcnt eax, eax
0x140b21e5b 488b5520 mov rdx, qword ptr [rbp + 0x20]
0x140b21e5f 4989c1 mov r9, rax
0x140b21e62 488b4210 mov rax, qword ptr [rdx + 0x10]
0x140b21e66 420fb60c08 movzx ecx, byte ptr [rax + r9]
0x140b21e6b 83e101 and ecx, 1
0x140b21e6e 48294a20 sub qword ptr [rdx + 0x20], rcx
0x140b21e72 498d49f0 lea rcx, [r9 - 0x10]
0x140b21e76 48234a18 and rcx, qword ptr [rdx + 0x18]
0x140b21e7a 4c8b45b8 mov r8, qword ptr [rbp - 0x48]
0x140b21e7e 46880408 mov byte ptr [rax + r9], r8b
0x140b21e82 4488440810 mov byte ptr [rax + rcx + 0x10], r8b
0x140b21e87 48ff4228 inc qword ptr [rdx + 0x28]
0x140b21e8b 49c1e104 shl r9, 4
0x140b21e8f 4889c1 mov rcx, rax
0x140b21e92 4c29c9 sub rcx, r9
0x140b21e95 49f7d9 neg r9
0x140b21e98 4a897408f0 mov qword ptr [rax + r9 - 0x10], rsi
0x140b21e9d 488979f8 mov qword ptr [rcx - 8], rdi
0x140b21ea1 e9a4fdffff jmp 0x140b21c4a
0x140b21ea6 e80568db01 call 0x1428d86b0
0x140b21eab 3401 xor al, 1
0x140b21ead e940fcffff jmp 0x140b21af2
0x140b21eb2 4889d9 mov rcx, rbx
0x140b21eb5 e8866edb01 call 0x1428d8d40
0x140b21eba b9ffffff3f mov ecx, 0x3fffffff
0x140b21ebf 31c0 xor eax, eax
0x140b21ec1 f00fb10b lock cmpxchg dword ptr [rbx], ecx
0x140b21ec5 0f840ffcffff je 0x140b21ada
0x140b21ecb 4889d9 mov rcx, rbx
0x140b21ece e83d6ddb01 call 0x1428d8c10
0x140b21ed3 e902fcffff jmp 0x140b21ada
0x140b21ed8 4889cb mov rbx, rcx
0x140b21edb e8d067db01 call 0x1428d86b0
0x140b21ee0 4889d9 mov rcx, rbx
0x140b21ee3 84c0 test al, al
0x140b21ee5 0f857ffdffff jne 0x140b21c6a
0x140b21eeb c6410801 mov byte ptr [rcx + 8], 1
0x140b21eef e976fdffff jmp 0x140b21c6a
0x140b21ef4 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140b21ef8 4c8d4030 lea r8, [rax + 0x30]
0x140b21efc ba01000000 mov edx, 1
0x140b21f01 4c89f1 mov rcx, r14
0x140b21f04 41b101 mov r9b, 1
0x140b21f07 e82433db01 call 0x1428d5230
0x140b21f0c e93dfeffff jmp 0x140b21d4e
0x140b21f11 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140b21f20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b21f25 55 push rbp
0x140b21f26 4157 push r15
0x140b21f28 4156 push r14
0x140b21f2a 4155 push r13
0x140b21f2c 4154 push r12
0x140b21f2e 56 push rsi
0x140b21f2f 57 push rdi
0x140b21f30 53 push rbx
0x140b21f31 4883ec58 sub rsp, 0x58
0x140b21f35 488daa80000000 lea rbp, [rdx + 0x80]
0x140b21f3c 66440f7f442420 movdqa xmmword ptr [rsp + 0x20], xmm8
0x140b21f43 660f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm7
0x140b21f49 660f7f742440 movdqa xmmword ptr [rsp + 0x40], xmm6
0x140b21f4f baffffffff mov edx, 0xffffffff
0x140b21f54 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140b21f58 f00fc110 lock xadd dword ptr [rax], edx
0x140b21f5c ffca dec edx
0x140b21f5e 89d0 mov eax, edx
0x140b21f60 25ffffffbf and eax, 0xbfffffff
0x140b21f65 f7d8 neg eax
0x140b21f67 7021 jo 0x140b21f8a
0x140b21f69 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b21f6e 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b21f73 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b21f79 4883c458 add rsp, 0x58
0x140b21f7d 5b pop rbx
0x140b21f7e 5f pop rdi
0x140b21f7f 5e pop rsi
0x140b21f80 415c pop r12
0x140b21f82 415d pop r13
0x140b21f84 415e pop r14
0x140b21f86 415f pop r15
0x140b21f88 5d pop rbp
0x140b21f89 c3 ret 
0x140b21f8a 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x140b21f8e e8ad6ddb01 call 0x1428d8d40
0x140b21f93 ebd4 jmp 0x140b21f69
0x140b21f95 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140b21fa0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b21fa5 55 push rbp
0x140b21fa6 4157 push r15
0x140b21fa8 4156 push r14
0x140b21faa 4155 push r13
0x140b21fac 4154 push r12
0x140b21fae 56 push rsi
0x140b21faf 57 push rdi
0x140b21fb0 53 push rbx
0x140b21fb1 4883ec58 sub rsp, 0x58
0x140b21fb5 488daa80000000 lea rbp, [rdx + 0x80]
0x140b21fbc 66440f7f442420 movdqa xmmword ptr [rsp + 0x20], xmm8
0x140b21fc3 660f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm7
0x140b21fc9 660f7f742440 movdqa xmmword ptr [rsp + 0x40], xmm6
0x140b21fcf 488d4dc0 lea rcx, [rbp - 0x40]
0x140b21fd3 e818eeaaff call 0x1405d0df0
0x140b21fd8 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140b21fdd 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140b21fe2 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140b21fe8 4883c458 add rsp, 0x58
0x140b21fec 5b pop rbx
0x140b21fed 5f pop rdi
0x140b21fee 5e pop rsi
0x140b21fef 415c pop r12
0x140b21ff1 415d pop r13
0x140b21ff3 415e pop r14
0x140b21ff5 415f pop r15
0x140b21ff7 5d pop rbp
0x140b21ff8 c3 ret 
