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
0x140a8d7d9 4c8d8508070000 lea r8, [rbp + 0x708]
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
0x140a8dd06 41b8a0020000 mov r8d, 0x2a0
0x140a8dd0c 488db540080000 lea rsi, [rbp + 0x840]
0x140a8dd13 4889f1 mov rcx, rsi
0x140a8dd16 488d9500050000 lea rdx, [rbp + 0x500]
0x140a8dd1d e8de5ee901 call 0x142923c00
0x140a8dd22 4c8b85b80c0000 mov r8, qword ptr [rbp + 0xcb8]
0x140a8dd29 c685560d000000 mov byte ptr [rbp + 0xd56], 0
0x14081d2ea 807b2101 cmpb	$0x1, 0x21(%rbx)
0x14081d2ee 0f85f6feffff jne	0x14081d1ea <.text+0x81c1ea>
0x14081d2f4 408a7644 movb	0x44(%rsi), %sil
0x14081d2f8 488d4d10 leaq	0x10(%rbp), %rcx
0x14081d2fc 4489e2 movl	%r12d, %edx
0x14081d2ff e87c963000 callq	0x140b26980 <.text+0xb25980>
0x14081d304 4885c0 testq	%rax, %rax
0x14081d307 0f84ddfeffff je	0x14081d1ea <.text+0x81c1ea>
0x14081d30d 498b8da8fdffff movq	-0x258(%r13), %rcx
0x14081d314 4883f901 cmpq	$0x1, %rcx
0x14081d318 0f8402020000 je	0x14081d520 <.text+0x81c520>
0x14081d31e 4885c9 testq	%rcx, %rcx
0x14081d321 0f84be020000 je	0x14081d5e5 <.text+0x81c5e5>
0x14081d327 f30f100b movss	(%rbx), %xmm1
0x14081d32b f3410f5fc9 maxss	%xmm9, %xmm1
0x14081d330 498b95a0fdffff movq	-0x260(%r13), %rdx
0x14081d337 48895550 movq	%rdx, 0x50(%rbp)
0x14081d33b 48894d58 movq	%rcx, 0x58(%rbp)
0x14081d33f 488d0dfa989e00 leaq	0x9e98fa(%rip), %rcx    # 0x141206c40 <.text+0x1205c40>
0x14081d346 48894d60 movq	%rcx, 0x60(%rbp)
0x14081d34a 488d4d68 leaq	0x68(%rbp), %rcx
0x14081d34e 440f1119 movups	%xmm11, (%rcx)
0x14081d352 488d4d50 leaq	0x50(%rbp), %rcx
0x14081d356 4989c4 movq	%rax, %r12
0x14081d359 e882c5a600 callq	0x1412898e0 <.text+0x12888e0>
0x14081d35e 8b4d70 movl	0x70(%rbp), %ecx
0x14081d361 498b95a8fdffff movq	-0x258(%r13), %rdx
0x14081d368 4839ca cmpq	%rcx, %rdx
0x14081d36b 0f86d9020000 jbe	0x14081d64a <.text+0x81c64a>
0x14081d371 488d4101 leaq	0x1(%rcx), %rax
0x14081d375 4839d0 cmpq	%rdx, %rax
0x14081d378 0f83da020000 jae	0x14081d658 <.text+0x81c658>
0x14081d37e f30f107574 movss	0x74(%rbp), %xmm6
0x14081d383 498b95a0fdffff movq	-0x260(%r13), %rdx
0x14081d38a 488d0c49 leaq	(%rcx,%rcx,2), %rcx
0x14081d38e f30f107c8a04 movss	0x4(%rdx,%rcx,4), %xmm7
0x14081d394 488d0440 leaq	(%rax,%rax,2), %rax
0x14081d398 f3440f10748204 movss	0x4(%rdx,%rax,4), %xmm14
0x14081d39f 410fb60c24 movzbl	(%r12), %ecx
0x14081d3a4 488b5510 movq	0x10(%rbp), %rdx
