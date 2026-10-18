0x140ee3870 55 push rbp
0x140ee3871 4157 push r15
0x140ee3873 4156 push r14
0x140ee3875 4155 push r13
0x140ee3877 4154 push r12
0x140ee3879 56 push rsi
0x140ee387a 57 push rdi
0x140ee387b 53 push rbx
0x140ee387c 4881ece8000000 sub rsp, 0xe8
0x140ee3883 488dac2480000000 lea rbp, [rsp + 0x80]
0x140ee388b 66440f7f4550 movdqa xmmword ptr [rbp + 0x50], xmm8
0x140ee3891 660f7f7d40 movdqa xmmword ptr [rbp + 0x40], xmm7
0x140ee3896 660f7f7530 movdqa xmmword ptr [rbp + 0x30], xmm6
0x140ee389b 48c74528feffffff mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x140ee38a3 4c89c6 mov rsi, r8
0x140ee38a6 4889d7 mov rdi, rdx
0x140ee38a9 4889cb mov rbx, rcx
0x140ee38ac 8b01 mov eax, dword ptr [rcx]
0x140ee38ae 3dfdffff3f cmp eax, 0x3ffffffd
0x140ee38b3 0f87a7010000 ja 0x140ee3a60
0x140ee38b9 8d4801 lea ecx, [rax + 1]
0x140ee38bc f00fb10b lock cmpxchg dword ptr [rbx], ecx
0x140ee38c0 0f859a010000 jne 0x140ee3a60
0x140ee38c6 0fb64308 movzx eax, byte ptr [rbx + 8]
0x140ee38ca 48895d20 mov qword ptr [rbp + 0x20], rbx
0x140ee38ce 48837b2800 cmp qword ptr [rbx + 0x28], 0
0x140ee38d3 0f84a2010000 je 0x140ee3a7b
0x140ee38d9 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
0x140ee38e3 488945e0 mov qword ptr [rbp - 0x20], rax
0x140ee38e7 660fefc0 pxor xmm0, xmm0
0x140ee38eb 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x140ee38f0 c6450800 mov byte ptr [rbp + 8], 0
0x140ee38f4 48b87713d038e6212845 movabs rax, 0x452821e638d01377
0x140ee38fe 488945e8 mov qword ptr [rbp - 0x18], rax
0x140ee3902 48b86c0ce934cf6654be movabs rax, 0xbe5466cf34e90c6c
0x140ee390c 488945f0 mov qword ptr [rbp - 0x10], rax
0x140ee3910 48b8dd507cc9b729acc0 movabs rax, 0xc0ac29b7c97c50dd
0x140ee391a 488945f8 mov qword ptr [rbp - 8], rax
0x140ee391e 48b8170947b5b5d5843f movabs rax, 0x3f84d5b5b5470917
0x140ee3928 48894500 mov qword ptr [rbp], rax
0x140ee392c 4c8d056d900402 lea r8, [rip + 0x204906d]
0x140ee3933 488d55d0 lea rdx, [rbp - 0x30]
0x140ee3937 4889f9 mov rcx, rdi
0x140ee393a ff5640 call qword ptr [rsi + 0x40]
0x140ee393d 807d0800 cmp byte ptr [rbp + 8], 0
0x140ee3941 7418 je 0x140ee395b
0x140ee3943 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140ee3947 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140ee394b 48334dd0 xor rcx, qword ptr [rbp - 0x30]
0x140ee394f 483345d8 xor rax, qword ptr [rbp - 0x28]
0x140ee3953 48f7e1 mul rcx
0x140ee3956 4831c2 xor rdx, rax
0x140ee3959 eb04 jmp 0x140ee395f
0x140ee395b 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x140ee395f 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140ee3963 4c8b6010 mov r12, qword ptr [rax + 0x10]
0x140ee3967 4c8b7018 mov r14, qword ptr [rax + 0x18]
0x140ee396b 4d89f7 mov r15, r14
0x140ee396e 4921d7 and r15, rdx
0x140ee3971 48c1ea39 shr rdx, 0x39
0x140ee3975 660f6ec2 movd xmm0, edx
0x140ee3979 660f60c0 punpcklbw xmm0, xmm0
0x140ee397d f20f70c000 pshuflw xmm0, xmm0, 0
0x140ee3982 660f70f000 pshufd xmm6, xmm0, 0
0x140ee3987 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x140ee398f 660f76ff pcmpeqd xmm7, xmm7
0x140ee3993 f3470f6f043c movdqu xmm8, xmmword ptr [r12 + r15]
0x140ee3999 66410f6fc0 movdqa xmm0, xmm8
0x140ee399e 660f74c6 pcmpeqb xmm0, xmm6
0x140ee39a2 66440fd7e8 pmovmskb r13d, xmm0
0x140ee39a7 4585ed test r13d, r13d
0x140ee39aa 743d je 0x140ee39e9
0x140ee39ac 0f1f4000 nop dword ptr [rax]
0x140ee39b0 f3410fbcc5 tzcnt eax, r13d
0x140ee39b5 4c01f8 add rax, r15
0x140ee39b8 4c21f0 and rax, r14
0x140ee39bb 48c1e004 shl rax, 4
0x140ee39bf 4c89e3 mov rbx, r12
0x140ee39c2 4829c3 sub rbx, rax
0x140ee39c5 4c8b43f0 mov r8, qword ptr [rbx - 0x10]
0x140ee39c9 4c8b4bf8 mov r9, qword ptr [rbx - 8]
0x140ee39cd 4889f9 mov rcx, rdi
0x140ee39d0 4889f2 mov rdx, rsi
0x140ee39d3 e8e87de2ff call 0x140d0b7c0
0x140ee39d8 84c0 test al, al
0x140ee39da 7537 jne 0x140ee3a13
0x140ee39dc 418d45ff lea eax, [r13 - 1]
0x140ee39e0 664421e8 and ax, r13w
0x140ee39e4 4189c5 mov r13d, eax
0x140ee39e7 75c7 jne 0x140ee39b0
0x140ee39e9 66440f74c7 pcmpeqb xmm8, xmm7
0x140ee39ee 66410fd7c0 pmovmskb eax, xmm8
0x140ee39f3 85c0 test eax, eax
0x140ee39f5 0f8580000000 jne 0x140ee3a7b
0x140ee39fb 488b4518 mov rax, qword ptr [rbp + 0x18]
0x140ee39ff 4901c7 add r15, rax
0x140ee3a02 4983c710 add r15, 0x10
0x140ee3a06 4883c010 add rax, 0x10
0x140ee3a0a 48894518 mov qword ptr [rbp + 0x18], rax
0x140ee3a0e 4d21f7 and r15, r14
0x140ee3a11 eb80 jmp 0x140ee3993
0x140ee3a13 488b73f0 mov rsi, qword ptr [rbx - 0x10]
0x140ee3a17 488b7bf8 mov rdi, qword ptr [rbx - 8]
0x140ee3a1b baffffffff mov edx, 0xffffffff
0x140ee3a20 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x140ee3a24 f00fc111 lock xadd dword ptr [rcx], edx
0x140ee3a28 ffca dec edx
0x140ee3a2a 89d0 mov eax, edx
0x140ee3a2c 25ffffffbf and eax, 0xbfffffff
0x140ee3a31 f7d8 neg eax
0x140ee3a33 0f801c020000 jo 0x140ee3c55
0x140ee3a39 4889f0 mov rax, rsi
0x140ee3a3c 4889fa mov rdx, rdi
0x140ee3a3f 0f287530 movaps xmm6, xmmword ptr [rbp + 0x30]
0x140ee3a43 0f287d40 movaps xmm7, xmmword ptr [rbp + 0x40]
0x140ee3a47 440f284550 movaps xmm8, xmmword ptr [rbp + 0x50]
0x140ee3a4c 4881c4e8000000 add rsp, 0xe8
0x140ee3a53 5b pop rbx
0x140ee3a54 5f pop rdi
0x140ee3a55 5e pop rsi
0x140ee3a56 415c pop r12
0x140ee3a58 415d pop r13
0x140ee3a5a 415e pop r14
0x140ee3a5c 415f pop r15
0x140ee3a5e 5d pop rbp
0x140ee3a5f c3 ret 
0x140ee3a60 4889d9 mov rcx, rbx
0x140ee3a63 e8c84f9f01 call 0x1428d8a30
0x140ee3a68 0fb64308 movzx eax, byte ptr [rbx + 8]
0x140ee3a6c 48895d20 mov qword ptr [rbp + 0x20], rbx
0x140ee3a70 48837b2800 cmp qword ptr [rbx + 0x28], 0
0x140ee3a75 0f855efeffff jne 0x140ee38d9
0x140ee3a7b baffffffff mov edx, 0xffffffff
0x140ee3a80 488b5d20 mov rbx, qword ptr [rbp + 0x20]
0x140ee3a84 f00fc113 lock xadd dword ptr [rbx], edx
0x140ee3a88 ffca dec edx
0x140ee3a8a 89d0 mov eax, edx
0x140ee3a8c 25ffffffbf and eax, 0xbfffffff
0x140ee3a91 f7d8 neg eax
0x140ee3a93 0f80e9030000 jo 0x140ee3e82
0x140ee3a99 b9ffffff3f mov ecx, 0x3fffffff
0x140ee3a9e 31c0 xor eax, eax
0x140ee3aa0 f00fb10b lock cmpxchg dword ptr [rbx], ecx
0x140ee3aa4 0f85f1030000 jne 0x140ee3e9b
0x140ee3aaa 488b056f897402 mov rax, qword ptr [rip + 0x274896f]
0x140ee3ab1 488b00 mov rax, qword ptr [rax]
0x140ee3ab4 48d1e0 shl rax, 1
0x140ee3ab7 4885c0 test rax, rax
0x140ee3aba 0f85b6030000 jne 0x140ee3e76
0x140ee3ac0 31c0 xor eax, eax
0x140ee3ac2 0fb64b08 movzx ecx, byte ptr [rbx + 8]
0x140ee3ac6 48895dc0 mov qword ptr [rbp - 0x40], rbx
0x140ee3aca 8845c8 mov byte ptr [rbp - 0x38], al
0x140ee3acd 48837b2800 cmp qword ptr [rbx + 0x28], 0
0x140ee3ad2 0f8487010000 je 0x140ee3c5f
0x140ee3ad8 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
0x140ee3ae2 488945e0 mov qword ptr [rbp - 0x20], rax
0x140ee3ae6 660fefc0 pxor xmm0, xmm0
0x140ee3aea 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x140ee3aef c6450800 mov byte ptr [rbp + 8], 0
0x140ee3af3 48b87713d038e6212845 movabs rax, 0x452821e638d01377
0x140ee3afd 488945e8 mov qword ptr [rbp - 0x18], rax
0x140ee3b01 48b86c0ce934cf6654be movabs rax, 0xbe5466cf34e90c6c
0x140ee3b0b 488945f0 mov qword ptr [rbp - 0x10], rax
0x140ee3b0f 48b8dd507cc9b729acc0 movabs rax, 0xc0ac29b7c97c50dd
0x140ee3b19 488945f8 mov qword ptr [rbp - 8], rax
0x140ee3b1d 48b8170947b5b5d5843f movabs rax, 0x3f84d5b5b5470917
0x140ee3b27 48894500 mov qword ptr [rbp], rax
0x140ee3b2b 4c8d056e8e0402 lea r8, [rip + 0x2048e6e]
0x140ee3b32 488d55d0 lea rdx, [rbp - 0x30]
0x140ee3b36 4889f9 mov rcx, rdi
0x140ee3b39 ff5640 call qword ptr [rsi + 0x40]
0x140ee3b3c 807d0800 cmp byte ptr [rbp + 8], 0
0x140ee3b40 7418 je 0x140ee3b5a
0x140ee3b42 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140ee3b46 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140ee3b4a 48334dd0 xor rcx, qword ptr [rbp - 0x30]
0x140ee3b4e 483345d8 xor rax, qword ptr [rbp - 0x28]
0x140ee3b52 48f7e1 mul rcx
0x140ee3b55 4831c2 xor rdx, rax
0x140ee3b58 eb04 jmp 0x140ee3b5e
0x140ee3b5a 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x140ee3b5e 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140ee3b62 4c8b6010 mov r12, qword ptr [rax + 0x10]
0x140ee3b66 4c8b7018 mov r14, qword ptr [rax + 0x18]
0x140ee3b6a 4d89f7 mov r15, r14
0x140ee3b6d 4921d7 and r15, rdx
0x140ee3b70 48c1ea39 shr rdx, 0x39
0x140ee3b74 660f6ec2 movd xmm0, edx
0x140ee3b78 660f60c0 punpcklbw xmm0, xmm0
0x140ee3b7c f20f70c000 pshuflw xmm0, xmm0, 0
0x140ee3b81 660f70f000 pshufd xmm6, xmm0, 0
0x140ee3b86 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x140ee3b8e 660f76ff pcmpeqd xmm7, xmm7
0x140ee3b92 f3470f6f043c movdqu xmm8, xmmword ptr [r12 + r15]
0x140ee3b98 66410f6fc0 movdqa xmm0, xmm8
0x140ee3b9d 660f74c6 pcmpeqb xmm0, xmm6
0x140ee3ba1 66440fd7e8 pmovmskb r13d, xmm0
0x140ee3ba6 4585ed test r13d, r13d
0x140ee3ba9 743e je 0x140ee3be9
0x140ee3bab 0f1f440000 nop dword ptr [rax + rax]
0x140ee3bb0 f3410fbcc5 tzcnt eax, r13d
0x140ee3bb5 4c01f8 add rax, r15
0x140ee3bb8 4c21f0 and rax, r14
0x140ee3bbb 48c1e004 shl rax, 4
0x140ee3bbf 4c89e3 mov rbx, r12
0x140ee3bc2 4829c3 sub rbx, rax
0x140ee3bc5 4c8b43f0 mov r8, qword ptr [rbx - 0x10]
0x140ee3bc9 4c8b4bf8 mov r9, qword ptr [rbx - 8]
0x140ee3bcd 4889f9 mov rcx, rdi
0x140ee3bd0 4889f2 mov rdx, rsi
0x140ee3bd3 e8e87be2ff call 0x140d0b7c0
0x140ee3bd8 84c0 test al, al
0x140ee3bda 7536 jne 0x140ee3c12
0x140ee3bdc 418d45ff lea eax, [r13 - 1]
0x140ee3be0 664421e8 and ax, r13w
0x140ee3be4 4189c5 mov r13d, eax
0x140ee3be7 75c7 jne 0x140ee3bb0
0x140ee3be9 66440f74c7 pcmpeqb xmm8, xmm7
0x140ee3bee 66410fd7c0 pmovmskb eax, xmm8
0x140ee3bf3 85c0 test eax, eax
0x140ee3bf5 7568 jne 0x140ee3c5f
0x140ee3bf7 488b4518 mov rax, qword ptr [rbp + 0x18]
0x140ee3bfb 4901c7 add r15, rax
0x140ee3bfe 4983c710 add r15, 0x10
0x140ee3c02 4883c010 add rax, 0x10
0x140ee3c06 48894518 mov qword ptr [rbp + 0x18], rax
0x140ee3c0a 4d21f7 and r15, r14
0x140ee3c0d e980ffffff jmp 0x140ee3b92
0x140ee3c12 488b73f0 mov rsi, qword ptr [rbx - 0x10]
0x140ee3c16 488b7bf8 mov rdi, qword ptr [rbx - 8]
0x140ee3c1a 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
0x140ee3c1e 807dc800 cmp byte ptr [rbp - 0x38], 0
0x140ee3c22 7516 jne 0x140ee3c3a
0x140ee3c24 488b05f5877402 mov rax, qword ptr [rip + 0x27487f5]
0x140ee3c2b 488b00 mov rax, qword ptr [rax]
0x140ee3c2e 48d1e0 shl rax, 1
0x140ee3c31 4885c0 test rax, rax
0x140ee3c34 0f856e020000 jne 0x140ee3ea8
0x140ee3c3a ba010000c0 mov edx, 0xc0000001
0x140ee3c3f f00fc111 lock xadd dword ptr [rcx], edx
0x140ee3c43 81c2010000c0 add edx, 0xc0000001
0x140ee3c49 81fa00000040 cmp edx, 0x40000000
0x140ee3c4f 0f82e4fdffff jb 0x140ee3a39
0x140ee3c55 e8e6509f01 call 0x1428d8d40
0x140ee3c5a e9dafdffff jmp 0x140ee3a39
0x140ee3c5f 4889f9 mov rcx, rdi
0x140ee3c62 4889f2 mov rdx, rsi
0x140ee3c65 e8967be2ff call 0x140d0b800
0x140ee3c6a 4889c6 mov rsi, rax
0x140ee3c6d 4889d7 mov rdi, rdx
0x140ee3c70 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
0x140ee3c7a 488945e0 mov qword ptr [rbp - 0x20], rax
0x140ee3c7e 660fefc0 pxor xmm0, xmm0
0x140ee3c82 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x140ee3c87 c6450800 mov byte ptr [rbp + 8], 0
0x140ee3c8b 48b87713d038e6212845 movabs rax, 0x452821e638d01377
0x140ee3c95 488945e8 mov qword ptr [rbp - 0x18], rax
0x140ee3c99 48b86c0ce934cf6654be movabs rax, 0xbe5466cf34e90c6c
0x140ee3ca3 488945f0 mov qword ptr [rbp - 0x10], rax
0x140ee3ca7 48b8dd507cc9b729acc0 movabs rax, 0xc0ac29b7c97c50dd
0x140ee3cb1 488945f8 mov qword ptr [rbp - 8], rax
0x140ee3cb5 48b8170947b5b5d5843f movabs rax, 0x3f84d5b5b5470917
0x140ee3cbf 48894500 mov qword ptr [rbp], rax
0x140ee3cc3 4c8d05d68c0402 lea r8, [rip + 0x2048cd6]
0x140ee3cca 488d55d0 lea rdx, [rbp - 0x30]
0x140ee3cce 4889f1 mov rcx, rsi
0x140ee3cd1 ff5740 call qword ptr [rdi + 0x40]
0x140ee3cd4 807d0800 cmp byte ptr [rbp + 8], 0
0x140ee3cd8 742d je 0x140ee3d07
0x140ee3cda 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140ee3cde 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140ee3ce2 48334dd0 xor rcx, qword ptr [rbp - 0x30]
0x140ee3ce6 483345d8 xor rax, qword ptr [rbp - 0x28]
0x140ee3cea 48f7e1 mul rcx
0x140ee3ced 4889d3 mov rbx, rdx
0x140ee3cf0 4831c3 xor rbx, rax
0x140ee3cf3 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140ee3cf7 4c8d7010 lea r14, [rax + 0x10]
0x140ee3cfb 4883782000 cmp qword ptr [rax + 0x20], 0
0x140ee3d00 751c jne 0x140ee3d1e
0x140ee3d02 e9bd010000 jmp 0x140ee3ec4
0x140ee3d07 488b5de0 mov rbx, qword ptr [rbp - 0x20]
0x140ee3d0b 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140ee3d0f 4c8d7010 lea r14, [rax + 0x10]
0x140ee3d13 4883782000 cmp qword ptr [rax + 0x20], 0
0x140ee3d18 0f84a6010000 je 0x140ee3ec4
0x140ee3d1e 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140ee3d22 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140ee3d26 4c8b6018 mov r12, qword ptr [rax + 0x18]
0x140ee3d2a 4889d8 mov rax, rbx
0x140ee3d2d 48c1e839 shr rax, 0x39
0x140ee3d31 488945b8 mov qword ptr [rbp - 0x48], rax
0x140ee3d35 660f6ec0 movd xmm0, eax
0x140ee3d39 660f60c0 punpcklbw xmm0, xmm0
0x140ee3d3d f20f70c000 pshuflw xmm0, xmm0, 0
0x140ee3d42 660f70f000 pshufd xmm6, xmm0, 0
0x140ee3d47 31d2 xor edx, edx
0x140ee3d49 660f76ff pcmpeqd xmm7, xmm7
0x140ee3d4d 4531ed xor r13d, r13d
0x140ee3d50 48894da8 mov qword ptr [rbp - 0x58], rcx
0x140ee3d54 48895518 mov qword ptr [rbp + 0x18], rdx
0x140ee3d58 4c21e3 and rbx, r12
0x140ee3d5b f3440f6f0419 movdqu xmm8, xmmword ptr [rcx + rbx]
0x140ee3d61 66410f6fc0 movdqa xmm0, xmm8
0x140ee3d66 660f74c6 pcmpeqb xmm0, xmm6
0x140ee3d6a 66440fd7f8 pmovmskb r15d, xmm0
0x140ee3d6f 4585ff test r15d, r15d
0x140ee3d72 7450 je 0x140ee3dc4
0x140ee3d74 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ee3d80 f3410fbcc7 tzcnt eax, r15d
0x140ee3d85 4801d8 add rax, rbx
0x140ee3d88 4c21e0 and rax, r12
0x140ee3d8b 498b0e mov rcx, qword ptr [r14]
0x140ee3d8e 48c1e004 shl rax, 4
0x140ee3d92 4889ca mov rdx, rcx
0x140ee3d95 4829c2 sub rdx, rax
0x140ee3d98 4c8b42f0 mov r8, qword ptr [rdx - 0x10]
0x140ee3d9c 48f7d8 neg rax
0x140ee3d9f 4c8b4c01f8 mov r9, qword ptr [rcx + rax - 8]
0x140ee3da4 4889f1 mov rcx, rsi
0x140ee3da7 4889fa mov rdx, rdi
0x140ee3daa e8117ae2ff call 0x140d0b7c0
0x140ee3daf 84c0 test al, al
0x140ee3db1 0f8563feffff jne 0x140ee3c1a
0x140ee3db7 418d47ff lea eax, [r15 - 1]
0x140ee3dbb 664421f8 and ax, r15w
0x140ee3dbf 4189c7 mov r15d, eax
0x140ee3dc2 75bc jne 0x140ee3d80
0x140ee3dc4 4983fd01 cmp r13, 1
0x140ee3dc8 41bd01000000 mov r13d, 1
0x140ee3dce 741c je 0x140ee3dec
0x140ee3dd0 66410fd7c0 pmovmskb eax, xmm8
0x140ee3dd5 4531ed xor r13d, r13d
0x140ee3dd8 85c0 test eax, eax
0x140ee3dda 410f95c5 setne r13b
0x140ee3dde f30fbcc0 tzcnt eax, eax
0x140ee3de2 4801d8 add rax, rbx
0x140ee3de5 4c21e0 and rax, r12
0x140ee3de8 488945b0 mov qword ptr [rbp - 0x50], rax
0x140ee3dec 66440f74c7 pcmpeqb xmm8, xmm7
0x140ee3df1 66410fd7c0 pmovmskb eax, xmm8
0x140ee3df6 85c0 test eax, eax
0x140ee3df8 488b4da8 mov rcx, qword ptr [rbp - 0x58]
0x140ee3dfc 488b4518 mov rax, qword ptr [rbp + 0x18]
0x140ee3e00 7513 jne 0x140ee3e15
0x140ee3e02 4801c3 add rbx, rax
0x140ee3e05 4883c310 add rbx, 0x10
0x140ee3e09 4883c010 add rax, 0x10
0x140ee3e0d 4889c2 mov rdx, rax
0x140ee3e10 e93fffffff jmp 0x140ee3d54
0x140ee3e15 488b45b0 mov rax, qword ptr [rbp - 0x50]
0x140ee3e19 803c0100 cmp byte ptr [rcx + rax], 0
0x140ee3e1d 780c js 0x140ee3e2b
0x140ee3e1f 660f6f01 movdqa xmm0, xmmword ptr [rcx]
0x140ee3e23 660fd7c0 pmovmskb eax, xmm0
0x140ee3e27 f30fbcc0 tzcnt eax, eax
0x140ee3e2b 488b5520 mov rdx, qword ptr [rbp + 0x20]
0x140ee3e2f 4989c1 mov r9, rax
0x140ee3e32 488b4210 mov rax, qword ptr [rdx + 0x10]
0x140ee3e36 420fb60c08 movzx ecx, byte ptr [rax + r9]
0x140ee3e3b 83e101 and ecx, 1
0x140ee3e3e 48294a20 sub qword ptr [rdx + 0x20], rcx
0x140ee3e42 498d49f0 lea rcx, [r9 - 0x10]
0x140ee3e46 48234a18 and rcx, qword ptr [rdx + 0x18]
0x140ee3e4a 4c8b45b8 mov r8, qword ptr [rbp - 0x48]
0x140ee3e4e 46880408 mov byte ptr [rax + r9], r8b
0x140ee3e52 4488440810 mov byte ptr [rax + rcx + 0x10], r8b
0x140ee3e57 48ff4228 inc qword ptr [rdx + 0x28]
0x140ee3e5b 49c1e104 shl r9, 4
0x140ee3e5f 4889c1 mov rcx, rax
0x140ee3e62 4c29c9 sub rcx, r9
0x140ee3e65 49f7d9 neg r9
0x140ee3e68 4a897408f0 mov qword ptr [rax + r9 - 0x10], rsi
0x140ee3e6d 488979f8 mov qword ptr [rcx - 8], rdi
0x140ee3e71 e9a4fdffff jmp 0x140ee3c1a
0x140ee3e76 e835489f01 call 0x1428d86b0
0x140ee3e7b 3401 xor al, 1
0x140ee3e7d e940fcffff jmp 0x140ee3ac2
0x140ee3e82 4889d9 mov rcx, rbx
0x140ee3e85 e8b64e9f01 call 0x1428d8d40
0x140ee3e8a b9ffffff3f mov ecx, 0x3fffffff
0x140ee3e8f 31c0 xor eax, eax
0x140ee3e91 f00fb10b lock cmpxchg dword ptr [rbx], ecx
0x140ee3e95 0f840ffcffff je 0x140ee3aaa
0x140ee3e9b 4889d9 mov rcx, rbx
0x140ee3e9e e86d4d9f01 call 0x1428d8c10
0x140ee3ea3 e902fcffff jmp 0x140ee3aaa
0x140ee3ea8 4889cb mov rbx, rcx
0x140ee3eab e800489f01 call 0x1428d86b0
0x140ee3eb0 4889d9 mov rcx, rbx
0x140ee3eb3 84c0 test al, al
0x140ee3eb5 0f857ffdffff jne 0x140ee3c3a
0x140ee3ebb c6410801 mov byte ptr [rcx + 8], 1
0x140ee3ebf e976fdffff jmp 0x140ee3c3a
0x140ee3ec4 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140ee3ec8 4c8d4030 lea r8, [rax + 0x30]
0x140ee3ecc ba01000000 mov edx, 1
0x140ee3ed1 4c89f1 mov rcx, r14
0x140ee3ed4 41b101 mov r9b, 1
0x140ee3ed7 e854139f01 call 0x1428d5230
0x140ee3edc e93dfeffff jmp 0x140ee3d1e
0x140ee3ee1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ee3ef0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee3ef5 55 push rbp
0x140ee3ef6 4157 push r15
0x140ee3ef8 4156 push r14
0x140ee3efa 4155 push r13
0x140ee3efc 4154 push r12
0x140ee3efe 56 push rsi
0x140ee3eff 57 push rdi
0x140ee3f00 53 push rbx
0x140ee3f01 4883ec58 sub rsp, 0x58
0x140ee3f05 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee3f0c 66440f7f442420 movdqa xmmword ptr [rsp + 0x20], xmm8
0x140ee3f13 660f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm7
0x140ee3f19 660f7f742440 movdqa xmmword ptr [rsp + 0x40], xmm6
0x140ee3f1f baffffffff mov edx, 0xffffffff
0x140ee3f24 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140ee3f28 f00fc110 lock xadd dword ptr [rax], edx
0x140ee3f2c ffca dec edx
0x140ee3f2e 89d0 mov eax, edx
0x140ee3f30 25ffffffbf and eax, 0xbfffffff
0x140ee3f35 f7d8 neg eax
0x140ee3f37 7021 jo 0x140ee3f5a
0x140ee3f39 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140ee3f3e 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140ee3f43 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140ee3f49 4883c458 add rsp, 0x58
0x140ee3f4d 5b pop rbx
0x140ee3f4e 5f pop rdi
0x140ee3f4f 5e pop rsi
0x140ee3f50 415c pop r12
0x140ee3f52 415d pop r13
0x140ee3f54 415e pop r14
0x140ee3f56 415f pop r15
0x140ee3f58 5d pop rbp
0x140ee3f59 c3 ret 
0x140ee3f5a 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x140ee3f5e e8dd4d9f01 call 0x1428d8d40
0x140ee3f63 ebd4 jmp 0x140ee3f39
0x140ee3f65 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ee3f70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee3f75 55 push rbp
0x140ee3f76 4157 push r15
0x140ee3f78 4156 push r14
0x140ee3f7a 4155 push r13
0x140ee3f7c 4154 push r12
0x140ee3f7e 56 push rsi
0x140ee3f7f 57 push rdi
0x140ee3f80 53 push rbx
0x140ee3f81 4883ec58 sub rsp, 0x58
0x140ee3f85 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee3f8c 66440f7f442420 movdqa xmmword ptr [rsp + 0x20], xmm8
0x140ee3f93 660f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm7
0x140ee3f99 660f7f742440 movdqa xmmword ptr [rsp + 0x40], xmm6
0x140ee3f9f 488d4dc0 lea rcx, [rbp - 0x40]
0x140ee3fa3 e848ce6eff call 0x1405d0df0
0x140ee3fa8 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x140ee3fad 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140ee3fb2 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140ee3fb8 4883c458 add rsp, 0x58
0x140ee3fbc 5b pop rbx
0x140ee3fbd 5f pop rdi
0x140ee3fbe 5e pop rsi
0x140ee3fbf 415c pop r12
0x140ee3fc1 415d pop r13
0x140ee3fc3 415e pop r14
0x140ee3fc5 415f pop r15
0x140ee3fc7 5d pop rbp
0x140ee3fc8 c3 ret 
