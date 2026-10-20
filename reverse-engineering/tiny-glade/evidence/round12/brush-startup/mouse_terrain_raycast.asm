
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408ef320: 55                          	pushq	%rbp
1408ef321: 41 57                       	pushq	%r15
1408ef323: 41 56                       	pushq	%r14
1408ef325: 41 55                       	pushq	%r13
1408ef327: 41 54                       	pushq	%r12
1408ef329: 56                          	pushq	%rsi
1408ef32a: 57                          	pushq	%rdi
1408ef32b: 53                          	pushq	%rbx
1408ef32c: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1408ef333: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408ef33b: 0f 29 75 40                 	movaps	%xmm6, 0x40(%rbp)
1408ef33f: 48 c7 45 38 fe ff ff ff     	movq	$-0x2, 0x38(%rbp)
1408ef347: 4c 89 cb                    	movq	%r9, %rbx
1408ef34a: 4d 89 c7                    	movq	%r8, %r15
1408ef34d: 48 89 d7                    	movq	%rdx, %rdi
1408ef350: 49 89 ce                    	movq	%rcx, %r14
1408ef353: e8 a8 76 32 00              	callq	0x140c16a00 <.text+0xc15a00>
1408ef358: 89 c6                       	movl	%eax, %esi
1408ef35a: 84 c0                       	testb	%al, %al
1408ef35c: 48 89 5d 30                 	movq	%rbx, 0x30(%rbp)
1408ef360: 74 60                       	je	0x1408ef3c2 <.text+0x8ee3c2>
1408ef362: 4c 8d 25 57 77 1b 02        	leaq	0x21b7757(%rip), %r12   # 0x142aa6ac0
1408ef369: 41 bd 45 00 00 00           	movl	$0x45, %r13d
1408ef36f: ba 45 00 00 00              	movl	$0x45, %edx
1408ef374: 4c 89 e1                    	movq	%r12, %rcx
1408ef377: e8 a4 e0 ff ff              	callq	0x1408ed420 <.text+0x8ec420>
1408ef37c: 48 83 f8 01                 	cmpq	$0x1, %rax
1408ef380: 0f 85 b2 00 00 00           	jne	0x1408ef438 <.text+0x8ee438>
1408ef386: 49 89 d1                    	movq	%rdx, %r9
1408ef389: 48 85 d2                    	testq	%rdx, %rdx
1408ef38c: 74 3d                       	je	0x1408ef3cb <.text+0x8ee3cb>
1408ef38e: 49 83 f9 45                 	cmpq	$0x45, %r9
1408ef392: 73 35                       	jae	0x1408ef3c9 <.text+0x8ee3c9>
1408ef394: 48 8d 05 25 77 1b 02        	leaq	0x21b7725(%rip), %rax   # 0x142aa6ac0
1408ef39b: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1408ef3a0: 7f 29                       	jg	0x1408ef3cb <.text+0x8ee3cb>
1408ef3a2: 48 8d 05 3f 78 1b 02        	leaq	0x21b783f(%rip), %rax   # 0x142aa6be8
1408ef3a9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408ef3ae: 48 8d 0d 0b 77 1b 02        	leaq	0x21b770b(%rip), %rcx   # 0x142aa6ac0
1408ef3b5: ba 45 00 00 00              	movl	$0x45, %edx
1408ef3ba: 45 31 c0                    	xorl	%r8d, %r8d
1408ef3bd: e8 7e a5 fe 01              	callq	0x1428d9940 <.text+0x28d8940>
1408ef3c2: 31 c0                       	xorl	%eax, %eax
1408ef3c4: e9 f3 01 00 00              	jmp	0x1408ef5bc <.text+0x8ee5bc>
1408ef3c9: 75 d7                       	jne	0x1408ef3a2 <.text+0x8ee3a2>
1408ef3cb: 4c 8d 25 ee 76 1b 02        	leaq	0x21b76ee(%rip), %r12   # 0x142aa6ac0
1408ef3d2: 4c 89 e1                    	movq	%r12, %rcx
1408ef3d5: 4c 89 ca                    	movq	%r9, %rdx
1408ef3d8: e8 43 e0 ff ff              	callq	0x1408ed420 <.text+0x8ec420>
1408ef3dd: 48 83 f8 01                 	cmpq	$0x1, %rax
1408ef3e1: 75 55                       	jne	0x1408ef438 <.text+0x8ee438>
1408ef3e3: 49 89 d0                    	movq	%rdx, %r8
1408ef3e6: 49 83 c0 02                 	addq	$0x2, %r8
1408ef3ea: 74 39                       	je	0x1408ef425 <.text+0x8ee425>
1408ef3ec: 49 83 f8 45                 	cmpq	$0x45, %r8
1408ef3f0: 73 31                       	jae	0x1408ef423 <.text+0x8ee423>
1408ef3f2: 48 8d 05 c7 76 1b 02        	leaq	0x21b76c7(%rip), %rax   # 0x142aa6ac0
1408ef3f9: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1408ef3fe: 7f 25                       	jg	0x1408ef425 <.text+0x8ee425>
1408ef400: 48 8d 05 f9 77 1b 02        	leaq	0x21b77f9(%rip), %rax   # 0x142aa6c00
1408ef407: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408ef40c: 48 8d 0d ad 76 1b 02        	leaq	0x21b76ad(%rip), %rcx   # 0x142aa6ac0
1408ef413: ba 45 00 00 00              	movl	$0x45, %edx
1408ef418: 41 b9 45 00 00 00           	movl	$0x45, %r9d
1408ef41e: e8 1d a5 fe 01              	callq	0x1428d9940 <.text+0x28d8940>
1408ef423: 75 db                       	jne	0x1408ef400 <.text+0x8ee400>
1408ef425: 41 bd 43 00 00 00           	movl	$0x43, %r13d
1408ef42b: 49 29 d5                    	subq	%rdx, %r13
1408ef42e: 4c 8d 25 8b 76 1b 02        	leaq	0x21b768b(%rip), %r12   # 0x142aa6ac0
1408ef435: 4d 01 c4                    	addq	%r8, %r12
1408ef438: 4c 89 6d f8                 	movq	%r13, -0x8(%rbp)
1408ef43c: 48 8d 05 4f 80 1b 02        	leaq	0x21b804f(%rip), %rax   # 0x142aa7492
1408ef443: 4c 8d 2d 0e 80 1b 02        	leaq	0x21b800e(%rip), %r13   # 0x142aa7458
1408ef44a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1408ef450: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
1408ef454: 85 c9                       	testl	%ecx, %ecx
1408ef456: 78 18                       	js	0x1408ef470 <.text+0x8ee470>
1408ef458: 48 ff c8                    	decq	%rax
1408ef45b: 83 f9 5c                    	cmpl	$0x5c, %ecx
1408ef45e: 75 69                       	jne	0x1408ef4c9 <.text+0x8ee4c9>
1408ef460: e9 85 00 00 00              	jmp	0x1408ef4ea <.text+0x8ee4ea>
1408ef465: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1408ef470: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
1408ef474: 80 fa c0                    	cmpb	$-0x40, %dl
1408ef477: 7d 25                       	jge	0x1408ef49e <.text+0x8ee49e>
1408ef479: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
1408ef47e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
1408ef482: 7d 23                       	jge	0x1408ef4a7 <.text+0x8ee4a7>
1408ef484: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
1408ef489: 48 83 c0 fc                 	addq	$-0x4, %rax
1408ef48d: 41 83 e1 07                 	andl	$0x7, %r9d
1408ef491: 41 c1 e1 06                 	shll	$0x6, %r9d
1408ef495: 41 83 e0 3f                 	andl	$0x3f, %r8d
1408ef499: 45 09 c8                    	orl	%r9d, %r8d
1408ef49c: eb 11                       	jmp	0x1408ef4af <.text+0x8ee4af>
1408ef49e: 48 83 c0 fe                 	addq	$-0x2, %rax
1408ef4a2: 83 e2 1f                    	andl	$0x1f, %edx
1408ef4a5: eb 12                       	jmp	0x1408ef4b9 <.text+0x8ee4b9>
1408ef4a7: 48 83 c0 fd                 	addq	$-0x3, %rax
1408ef4ab: 41 83 e0 0f                 	andl	$0xf, %r8d
1408ef4af: 41 c1 e0 06                 	shll	$0x6, %r8d
1408ef4b3: 83 e2 3f                    	andl	$0x3f, %edx
1408ef4b6: 44 09 c2                    	orl	%r8d, %edx
1408ef4b9: c1 e2 06                    	shll	$0x6, %edx
1408ef4bc: 80 e1 3f                    	andb	$0x3f, %cl
1408ef4bf: 0f b6 c9                    	movzbl	%cl, %ecx
1408ef4c2: 09 d1                       	orl	%edx, %ecx
1408ef4c4: 83 f9 5c                    	cmpl	$0x5c, %ecx
1408ef4c7: 74 21                       	je	0x1408ef4ea <.text+0x8ee4ea>
1408ef4c9: 83 f9 2f                    	cmpl	$0x2f, %ecx
1408ef4cc: 74 1c                       	je	0x1408ef4ea <.text+0x8ee4ea>
1408ef4ce: 4c 39 e8                    	cmpq	%r13, %rax
1408ef4d1: 0f 85 79 ff ff ff           	jne	0x1408ef450 <.text+0x8ee450>
1408ef4d7: 48 89 fb                    	movq	%rdi, %rbx
1408ef4da: 4c 89 ff                    	movq	%r15, %rdi
1408ef4dd: 4d 89 f7                    	movq	%r14, %r15
1408ef4e0: 41 89 f6                    	movl	%esi, %r14d
1408ef4e3: be 3a 00 00 00              	movl	$0x3a, %esi
1408ef4e8: eb 5b                       	jmp	0x1408ef545 <.text+0x8ee545>
1408ef4ea: 4c 8d 2d 67 7f 1b 02        	leaq	0x21b7f67(%rip), %r13   # 0x142aa7458
1408ef4f1: 4c 29 e8                    	subq	%r13, %rax
1408ef4f4: 49 89 c0                    	movq	%rax, %r8
1408ef4f7: 49 ff c0                    	incq	%r8
1408ef4fa: 74 32                       	je	0x1408ef52e <.text+0x8ee52e>
1408ef4fc: 49 83 f8 3a                 	cmpq	$0x3a, %r8
1408ef500: 73 2a                       	jae	0x1408ef52c <.text+0x8ee52c>
1408ef502: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1408ef507: 7f 25                       	jg	0x1408ef52e <.text+0x8ee52e>
1408ef509: 48 8d 05 b8 76 1b 02        	leaq	0x21b76b8(%rip), %rax   # 0x142aa6bc8
1408ef510: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408ef515: 48 8d 0d 3c 7f 1b 02        	leaq	0x21b7f3c(%rip), %rcx   # 0x142aa7458
1408ef51c: ba 3a 00 00 00              	movl	$0x3a, %edx
1408ef521: 41 b9 3a 00 00 00           	movl	$0x3a, %r9d
1408ef527: e8 14 a4 fe 01              	callq	0x1428d9940 <.text+0x28d8940>
1408ef52c: 75 db                       	jne	0x1408ef509 <.text+0x8ee509>
1408ef52e: 48 89 fb                    	movq	%rdi, %rbx
1408ef531: 4c 89 ff                    	movq	%r15, %rdi
1408ef534: 4d 89 f7                    	movq	%r14, %r15
1408ef537: 41 89 f6                    	movl	%esi, %r14d
1408ef53a: be 39 00 00 00              	movl	$0x39, %esi
1408ef53f: 48 29 c6                    	subq	%rax, %rsi
1408ef542: 4d 01 c5                    	addq	%r8, %r13
1408ef545: e8 e6 8b 32 00              	callq	0x140c18130 <.text+0xc17130>
1408ef54a: 48 89 c1                    	movq	%rax, %rcx
1408ef54d: 48 8b 00                    	movq	(%rax), %rax
1408ef550: 48 83 f8 01                 	cmpq	$0x1, %rax
1408ef554: 0f 85 8d 03 00 00           	jne	0x1408ef8e7 <.text+0x8ee8e7>
1408ef55a: 48 83 c1 08                 	addq	$0x8, %rcx
1408ef55e: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1408ef562: 0f 85 d7 03 00 00           	jne	0x1408ef93f <.text+0x8ee93f>
1408ef568: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1408ef56f: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
1408ef573: 48 83 c1 08                 	addq	$0x8, %rcx
1408ef577: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1408ef57c: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1408ef585: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1408ef58e: 4c 89 e2                    	movq	%r12, %rdx
1408ef591: 4c 8b 45 f8                 	movq	-0x8(%rbp), %r8
1408ef595: 4d 89 e9                    	movq	%r13, %r9
1408ef598: e8 a3 77 32 00              	callq	0x140c16d40 <.text+0xc15d40>
1408ef59d: 49 89 c5                    	movq	%rax, %r13
1408ef5a0: 48 8b 45 10                 	movq	0x10(%rbp), %rax
1408ef5a4: 48 ff 00                    	incq	(%rax)
1408ef5a7: 4c 89 6d e0                 	movq	%r13, -0x20(%rbp)
1408ef5ab: b8 01 00 00 00              	movl	$0x1, %eax
1408ef5b0: 44 89 f6                    	movl	%r14d, %esi
1408ef5b3: 4d 89 fe                    	movq	%r15, %r14
1408ef5b6: 49 89 ff                    	movq	%rdi, %r15
1408ef5b9: 48 89 df                    	movq	%rbx, %rdi
1408ef5bc: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
1408ef5c3: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1408ef5c7: 48 8b 01                    	movq	(%rcx), %rax
1408ef5ca: 83 38 00                    	cmpl	$0x0, (%rax)
1408ef5cd: 74 5e                       	je	0x1408ef62d <.text+0x8ee62d>
1408ef5cf: 40 84 f6                    	testb	%sil, %sil
1408ef5d2: 74 41                       	je	0x1408ef615 <.text+0x8ee615>
1408ef5d4: e8 57 8b 32 00              	callq	0x140c18130 <.text+0xc17130>
1408ef5d9: 48 89 c1                    	movq	%rax, %rcx
1408ef5dc: 48 8b 00                    	movq	(%rax), %rax
1408ef5df: 48 83 f8 01                 	cmpq	$0x1, %rax
1408ef5e3: 0f 85 18 03 00 00           	jne	0x1408ef901 <.text+0x8ee901>
1408ef5e9: 48 83 c1 08                 	addq	$0x8, %rcx
1408ef5ed: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1408ef5f1: 0f 85 48 03 00 00           	jne	0x1408ef93f <.text+0x8ee93f>
1408ef5f7: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1408ef5fe: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
1408ef602: 48 83 c1 08                 	addq	$0x8, %rcx
1408ef606: 4c 89 ea                    	movq	%r13, %rdx
1408ef609: e8 e2 7a 32 00              	callq	0x140c170f0 <.text+0xc160f0>
1408ef60e: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1408ef612: 48 ff 00                    	incq	(%rax)
1408ef615: 0f 28 75 40                 	movaps	0x40(%rbp), %xmm6
1408ef619: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1408ef620: 5b                          	popq	%rbx
1408ef621: 5f                          	popq	%rdi
1408ef622: 5e                          	popq	%rsi
1408ef623: 41 5c                       	popq	%r12
1408ef625: 41 5d                       	popq	%r13
1408ef627: 41 5e                       	popq	%r14
1408ef629: 41 5f                       	popq	%r15
1408ef62b: 5d                          	popq	%rbp
1408ef62c: c3                          	retq
1408ef62d: 48 8b 9d d0 00 00 00        	movq	0xd0(%rbp), %rbx
1408ef634: 4c 8b a5 c8 00 00 00        	movq	0xc8(%rbp), %r12
1408ef63b: 49 8b 04 24                 	movq	(%r12), %rax
1408ef63f: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1408ef643: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
1408ef647: 0f 10 00                    	movups	(%rax), %xmm0
1408ef64a: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1408ef64e: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1408ef652: 48 8b 08                    	movq	(%rax), %rcx
1408ef655: 83 39 01                    	cmpl	$0x1, (%rcx)
1408ef658: 0f 85 0e 01 00 00           	jne	0x1408ef76c <.text+0x8ee76c>
1408ef65e: 8b 47 1c                    	movl	0x1c(%rdi), %eax
1408ef661: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1408ef665: 89 02                       	movl	%eax, (%rdx)
1408ef667: 48 8b 07                    	movq	(%rdi), %rax
1408ef66a: 49 8b 14 24                 	movq	(%r12), %rdx
1408ef66e: f3 0f 10 41 04              	movss	0x4(%rcx), %xmm0
1408ef673: f2 0f 10 12                 	movsd	(%rdx), %xmm2
1408ef677: f2 0f 10 5a 0c              	movsd	0xc(%rdx), %xmm3
1408ef67c: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
1408ef680: f3 0f 5c c1                 	subss	%xmm1, %xmm0
1408ef684: f3 0f 16 cb                 	movshdup	%xmm3, %xmm1            # xmm1 = xmm3[1,1,3,3]
1408ef688: f3 0f 5e c1                 	divss	%xmm1, %xmm0
1408ef68c: f3 0f 12 c8                 	movsldup	%xmm0, %xmm1            # xmm1 = xmm0[0,0,2,2]
1408ef690: 0f 59 cb                    	mulps	%xmm3, %xmm1
1408ef693: f3 0f 59 42 14              	mulss	0x14(%rdx), %xmm0
1408ef698: 0f 58 ca                    	addps	%xmm2, %xmm1
1408ef69b: f3 0f 58 42 08              	addss	0x8(%rdx), %xmm0
1408ef6a0: 83 38 01                    	cmpl	$0x1, (%rax)
1408ef6a3: 75 4c                       	jne	0x1408ef6f1 <.text+0x8ee6f1>
1408ef6a5: 48 8b 0b                    	movq	(%rbx), %rcx
1408ef6a8: f3 0f 10 51 58              	movss	0x58(%rcx), %xmm2
1408ef6ad: f2 0f 10 58 04              	movsd	0x4(%rax), %xmm3
1408ef6b2: 0f 28 e1                    	movaps	%xmm1, %xmm4
1408ef6b5: 0f 5c e3                    	subps	%xmm3, %xmm4
1408ef6b8: 0f 28 d8                    	movaps	%xmm0, %xmm3
1408ef6bb: f3 0f 5c 58 0c              	subss	0xc(%rax), %xmm3
1408ef6c0: 0f 13 60 10                 	movlps	%xmm4, 0x10(%rax)
1408ef6c4: f3 0f 11 58 18              	movss	%xmm3, 0x18(%rax)
1408ef6c9: 0f 57 ed                    	xorps	%xmm5, %xmm5
1408ef6cc: 0f 2e d5                    	ucomiss	%xmm5, %xmm2
1408ef6cf: 74 20                       	je	0x1408ef6f1 <.text+0x8ee6f1>
1408ef6d1: 0f 59 e4                    	mulps	%xmm4, %xmm4
1408ef6d4: f3 0f 16 ec                 	movshdup	%xmm4, %xmm5            # xmm5 = xmm4[1,1,3,3]
1408ef6d8: f3 0f 58 ec                 	addss	%xmm4, %xmm5
1408ef6dc: f3 0f 59 db                 	mulss	%xmm3, %xmm3
1408ef6e0: f3 0f 58 dd                 	addss	%xmm5, %xmm3
1408ef6e4: f3 0f 51 db                 	sqrtss	%xmm3, %xmm3
1408ef6e8: f3 0f 5e da                 	divss	%xmm2, %xmm3
1408ef6ec: f3 0f 11 58 1c              	movss	%xmm3, 0x1c(%rax)
1408ef6f1: c7 00 01 00 00 00           	movl	$0x1, (%rax)
1408ef6f7: 0f 13 48 04                 	movlps	%xmm1, 0x4(%rax)
1408ef6fb: f3 0f 11 40 0c              	movss	%xmm0, 0xc(%rax)
1408ef700: 8b 48 0c                    	movl	0xc(%rax), %ecx
1408ef703: 89 4d 28                    	movl	%ecx, 0x28(%rbp)
1408ef706: 48 8b 40 04                 	movq	0x4(%rax), %rax
1408ef70a: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1408ef70e: c7 45 1c 01 00 00 00        	movl	$0x1, 0x1c(%rbp)
1408ef715: 4c 8d 45 1c                 	leaq	0x1c(%rbp), %r8
1408ef719: 4c 89 f1                    	movq	%r14, %rcx
1408ef71c: 4c 89 e2                    	movq	%r12, %rdx
1408ef71f: e8 4c 03 00 00              	callq	0x1408efa70 <.text+0x8eea70>
1408ef724: 40 84 f6                    	testb	%sil, %sil
1408ef727: 0f 84 e8 fe ff ff           	je	0x1408ef615 <.text+0x8ee615>
1408ef72d: e8 fe 89 32 00              	callq	0x140c18130 <.text+0xc17130>
1408ef732: 48 89 c1                    	movq	%rax, %rcx
1408ef735: 48 8b 00                    	movq	(%rax), %rax
1408ef738: 48 83 f8 01                 	cmpq	$0x1, %rax
1408ef73c: 0f 85 d9 01 00 00           	jne	0x1408ef91b <.text+0x8ee91b>
1408ef742: 48 83 c1 08                 	addq	$0x8, %rcx
1408ef746: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1408ef74a: 0f 85 ef 01 00 00           	jne	0x1408ef93f <.text+0x8ee93f>
1408ef750: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1408ef757: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
1408ef75b: 48 83 c1 08                 	addq	$0x8, %rcx
1408ef75f: 4c 89 ea                    	movq	%r13, %rdx
1408ef762: e8 89 79 32 00              	callq	0x140c170f0 <.text+0xc160f0>
1408ef767: e9 a2 fe ff ff              	jmp	0x1408ef60e <.text+0x8ee60e>
1408ef76c: 40 88 75 30                 	movb	%sil, 0x30(%rbp)
1408ef770: 49 8b 17                    	movq	(%r15), %rdx
1408ef773: 48 8d 4d 1c                 	leaq	0x1c(%rbp), %rcx
1408ef777: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
1408ef77b: f3 0f 10 1d fd 61 03 02     	movss	0x20361fd(%rip), %xmm3  # 0x142925980
1408ef783: e8 98 a2 0f 00              	callq	0x1409e9a20 <.text+0x9e8a20>
1408ef788: f6 45 1c 01                 	testb	$0x1, 0x1c(%rbp)
1408ef78c: 0f 84 21 01 00 00           	je	0x1408ef8b3 <.text+0x8ee8b3>
1408ef792: f3 0f 10 55 2c              	movss	0x2c(%rbp), %xmm2
1408ef797: 49 8b 04 24                 	movq	(%r12), %rax
1408ef79b: f3 0f 10 60 08              	movss	0x8(%rax), %xmm4
1408ef7a0: f2 0f 10 5d 24              	movsd	0x24(%rbp), %xmm3
1408ef7a5: f2 0f 10 28                 	movsd	(%rax), %xmm5
1408ef7a9: f2 0f 10 70 0c              	movsd	0xc(%rax), %xmm6
1408ef7ae: f3 0f 16 cd                 	movshdup	%xmm5, %xmm1            # xmm1 = xmm5[1,1,3,3]
1408ef7b2: 0f 57 c0                    	xorps	%xmm0, %xmm0
1408ef7b5: f3 0f 5c c1                 	subss	%xmm1, %xmm0
1408ef7b9: f3 0f 16 ce                 	movshdup	%xmm6, %xmm1            # xmm1 = xmm6[1,1,3,3]
1408ef7bd: f3 0f 5e c1                 	divss	%xmm1, %xmm0
1408ef7c1: f3 0f 12 c8                 	movsldup	%xmm0, %xmm1            # xmm1 = xmm0[0,0,2,2]
1408ef7c5: 0f 59 ce                    	mulps	%xmm6, %xmm1
1408ef7c8: f3 0f 59 40 14              	mulss	0x14(%rax), %xmm0
1408ef7cd: 0f 58 cd                    	addps	%xmm5, %xmm1
1408ef7d0: f3 0f 58 c4                 	addss	%xmm4, %xmm0
1408ef7d4: 0f 28 f5                    	movaps	%xmm5, %xmm6
1408ef7d7: 0f 5c f1                    	subps	%xmm1, %xmm6
1408ef7da: 0f 59 f6                    	mulps	%xmm6, %xmm6
1408ef7dd: 0f 5c eb                    	subps	%xmm3, %xmm5
1408ef7e0: 0f 59 ed                    	mulps	%xmm5, %xmm5
1408ef7e3: f2 0f 7c f5                 	haddps	%xmm5, %xmm6
1408ef7e7: f3 0f 12 e4                 	movsldup	%xmm4, %xmm4            # xmm4 = xmm4[0,0,2,2]
1408ef7eb: 0f 28 e8                    	movaps	%xmm0, %xmm5
1408ef7ee: 0f 14 ea                    	unpcklps	%xmm2, %xmm5            # xmm5 = xmm5[0],xmm2[0],xmm5[1],xmm2[1]
1408ef7f1: 0f 5c e5                    	subps	%xmm5, %xmm4
1408ef7f4: 0f c6 f6 e8                 	shufps	$0xe8, %xmm6, %xmm6     # xmm6 = xmm6[0,2,2,3]
1408ef7f8: 0f 59 e4                    	mulps	%xmm4, %xmm4
1408ef7fb: 0f 58 e6                    	addps	%xmm6, %xmm4
1408ef7fe: 0f 51 e4                    	sqrtps	%xmm4, %xmm4
1408ef801: f3 0f 16 ec                 	movshdup	%xmm4, %xmm5            # xmm5 = xmm4[1,1,3,3]
1408ef805: 31 c0                       	xorl	%eax, %eax
1408ef807: 0f 2e ec                    	ucomiss	%xmm4, %xmm5
1408ef80a: 0f 97 c0                    	seta	%al
1408ef80d: 66 0f 6e e0                 	movd	%eax, %xmm4
1408ef811: 66 0f 70 e4 50              	pshufd	$0x50, %xmm4, %xmm4     # xmm4 = xmm4[0,0,1,1]
1408ef816: 66 0f 72 f4 1f              	pslld	$0x1f, %xmm4
1408ef81b: 66 0f 72 e4 1f              	psrad	$0x1f, %xmm4
1408ef820: 0f 54 cc                    	andps	%xmm4, %xmm1
1408ef823: 0f 55 e3                    	andnps	%xmm3, %xmm4
1408ef826: 77 03                       	ja	0x1408ef82b <.text+0x8ee82b>
1408ef828: 0f 28 c2                    	movaps	%xmm2, %xmm0
1408ef82b: 0f 56 cc                    	orps	%xmm4, %xmm1
1408ef82e: 8b 47 1c                    	movl	0x1c(%rdi), %eax
1408ef831: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
1408ef835: 89 01                       	movl	%eax, (%rcx)
1408ef837: 48 8b 07                    	movq	(%rdi), %rax
1408ef83a: 83 38 01                    	cmpl	$0x1, (%rax)
1408ef83d: 0f b6 75 30                 	movzbl	0x30(%rbp), %esi
1408ef841: 75 4c                       	jne	0x1408ef88f <.text+0x8ee88f>
1408ef843: 48 8b 0b                    	movq	(%rbx), %rcx
1408ef846: f3 0f 10 51 58              	movss	0x58(%rcx), %xmm2
1408ef84b: f2 0f 10 58 04              	movsd	0x4(%rax), %xmm3
1408ef850: 0f 28 e1                    	movaps	%xmm1, %xmm4
1408ef853: 0f 5c e3                    	subps	%xmm3, %xmm4
1408ef856: 0f 28 d8                    	movaps	%xmm0, %xmm3
1408ef859: f3 0f 5c 58 0c              	subss	0xc(%rax), %xmm3
1408ef85e: 0f 13 60 10                 	movlps	%xmm4, 0x10(%rax)
1408ef862: f3 0f 11 58 18              	movss	%xmm3, 0x18(%rax)
1408ef867: 0f 57 ed                    	xorps	%xmm5, %xmm5
1408ef86a: 0f 2e d5                    	ucomiss	%xmm5, %xmm2
1408ef86d: 74 20                       	je	0x1408ef88f <.text+0x8ee88f>
1408ef86f: 0f 59 e4                    	mulps	%xmm4, %xmm4
1408ef872: f3 0f 16 ec                 	movshdup	%xmm4, %xmm5            # xmm5 = xmm4[1,1,3,3]
1408ef876: f3 0f 58 ec                 	addss	%xmm4, %xmm5
1408ef87a: f3 0f 59 db                 	mulss	%xmm3, %xmm3
1408ef87e: f3 0f 58 dd                 	addss	%xmm5, %xmm3
1408ef882: f3 0f 51 db                 	sqrtss	%xmm3, %xmm3
1408ef886: f3 0f 5e da                 	divss	%xmm2, %xmm3
1408ef88a: f3 0f 11 58 1c              	movss	%xmm3, 0x1c(%rax)
1408ef88f: c7 00 01 00 00 00           	movl	$0x1, (%rax)
1408ef895: 0f 13 48 04                 	movlps	%xmm1, 0x4(%rax)
1408ef899: f3 0f 11 40 0c              	movss	%xmm0, 0xc(%rax)
1408ef89e: 8b 48 0c                    	movl	0xc(%rax), %ecx
1408ef8a1: 89 4d 08                    	movl	%ecx, 0x8(%rbp)
1408ef8a4: 48 8b 40 04                 	movq	0x4(%rax), %rax
1408ef8a8: 48 89 45 00                 	movq	%rax, (%rbp)
1408ef8ac: b8 01 00 00 00              	movl	$0x1, %eax
1408ef8b1: eb 06                       	jmp	0x1408ef8b9 <.text+0x8ee8b9>
1408ef8b3: 31 c0                       	xorl	%eax, %eax
1408ef8b5: 0f b6 75 30                 	movzbl	0x30(%rbp), %esi
1408ef8b9: 89 45 1c                    	movl	%eax, 0x1c(%rbp)
1408ef8bc: 48 8b 45 00                 	movq	(%rbp), %rax
1408ef8c0: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1408ef8c4: 8b 45 08                    	movl	0x8(%rbp), %eax
1408ef8c7: 89 45 28                    	movl	%eax, 0x28(%rbp)
1408ef8ca: 4c 8d 45 1c                 	leaq	0x1c(%rbp), %r8
1408ef8ce: 4c 89 f1                    	movq	%r14, %rcx
1408ef8d1: 4c 89 e2                    	movq	%r12, %rdx
1408ef8d4: e8 97 01 00 00              	callq	0x1408efa70 <.text+0x8eea70>
1408ef8d9: 40 84 f6                    	testb	%sil, %sil
1408ef8dc: 0f 85 f2 fc ff ff           	jne	0x1408ef5d4 <.text+0x8ee5d4>
1408ef8e2: e9 2e fd ff ff              	jmp	0x1408ef615 <.text+0x8ee615>
1408ef8e7: 48 85 c0                    	testq	%rax, %rax
1408ef8ea: 75 47                       	jne	0x1408ef933 <.text+0x8ee933>
1408ef8ec: 31 d2                       	xorl	%edx, %edx
1408ef8ee: e8 dd 8e 00 02              	callq	0x1428f87d0 <.text+0x28f77d0>
1408ef8f3: 48 89 c1                    	movq	%rax, %rcx
1408ef8f6: 48 85 c0                    	testq	%rax, %rax
1408ef8f9: 0f 85 5f fc ff ff           	jne	0x1408ef55e <.text+0x8ee55e>
1408ef8ff: eb 32                       	jmp	0x1408ef933 <.text+0x8ee933>
1408ef901: 48 85 c0                    	testq	%rax, %rax
1408ef904: 75 2d                       	jne	0x1408ef933 <.text+0x8ee933>
1408ef906: 31 d2                       	xorl	%edx, %edx
1408ef908: e8 c3 8e 00 02              	callq	0x1428f87d0 <.text+0x28f77d0>
1408ef90d: 48 89 c1                    	movq	%rax, %rcx
1408ef910: 48 85 c0                    	testq	%rax, %rax
1408ef913: 0f 85 d4 fc ff ff           	jne	0x1408ef5ed <.text+0x8ee5ed>
1408ef919: eb 18                       	jmp	0x1408ef933 <.text+0x8ee933>
1408ef91b: 48 85 c0                    	testq	%rax, %rax
1408ef91e: 75 13                       	jne	0x1408ef933 <.text+0x8ee933>
1408ef920: 31 d2                       	xorl	%edx, %edx
1408ef922: e8 a9 8e 00 02              	callq	0x1428f87d0 <.text+0x28f77d0>
1408ef927: 48 89 c1                    	movq	%rax, %rcx
1408ef92a: 48 85 c0                    	testq	%rax, %rax
1408ef92d: 0f 85 13 fe ff ff           	jne	0x1408ef746 <.text+0x8ee746>
1408ef933: 48 8d 0d 66 b1 34 02        	leaq	0x234b166(%rip), %rcx   # 0x142c3aaa0
1408ef93a: e8 91 87 fe 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1408ef93f: 48 8d 0d 2a b2 34 02        	leaq	0x234b22a(%rip), %rcx   # 0x142c3ab70
1408ef946: e8 05 99 fe 01              	callq	0x1428d9250 <.text+0x28d8250>
1408ef94b: cc                          	int3
1408ef94c: 0f 1f 40 00                 	nopl	(%rax)
1408ef950: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408ef955: 55                          	pushq	%rbp
1408ef956: 41 57                       	pushq	%r15
1408ef958: 41 56                       	pushq	%r14
1408ef95a: 41 55                       	pushq	%r13
1408ef95c: 41 54                       	pushq	%r12
1408ef95e: 56                          	pushq	%rsi
1408ef95f: 57                          	pushq	%rdi
1408ef960: 53                          	pushq	%rbx
1408ef961: 48 83 ec 48                 	subq	$0x48, %rsp
1408ef965: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408ef96c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408ef971: 48 8b 45 10                 	movq	0x10(%rbp), %rax
1408ef975: 48 ff 00                    	incq	(%rax)
1408ef978: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408ef97d: 48 83 c4 48                 	addq	$0x48, %rsp
1408ef981: 5b                          	popq	%rbx
1408ef982: 5f                          	popq	%rdi
1408ef983: 5e                          	popq	%rsi
1408ef984: 41 5c                       	popq	%r12
1408ef986: 41 5d                       	popq	%r13
1408ef988: 41 5e                       	popq	%r14
1408ef98a: 41 5f                       	popq	%r15
1408ef98c: 5d                          	popq	%rbp
1408ef98d: c3                          	retq
1408ef98e: 66 90                       	nop
1408ef990: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408ef995: 55                          	pushq	%rbp
1408ef996: 41 57                       	pushq	%r15
1408ef998: 41 56                       	pushq	%r14
1408ef99a: 41 55                       	pushq	%r13
1408ef99c: 41 54                       	pushq	%r12
1408ef99e: 56                          	pushq	%rsi
1408ef99f: 57                          	pushq	%rdi
1408ef9a0: 53                          	pushq	%rbx
1408ef9a1: 48 83 ec 48                 	subq	$0x48, %rsp
1408ef9a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408ef9ac: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408ef9b1: 80 7d 30 00                 	cmpb	$0x0, 0x30(%rbp)
1408ef9b5: 74 20                       	je	0x1408ef9d7 <.text+0x8ee9d7>
1408ef9b7: 48 8d 45 e0                 	leaq	-0x20(%rbp), %rax
1408ef9bb: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1408ef9bf: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
1408ef9c3: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1408ef9c7: 48 8d 0d 9a 71 1b 02        	leaq	0x21b719a(%rip), %rcx   # 0x142aa6b68
1408ef9ce: 48 8d 55 e8                 	leaq	-0x18(%rbp), %rdx
1408ef9d2: e8 f9 f8 a3 00              	callq	0x14132f2d0 <.text+0x132e2d0>
1408ef9d7: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408ef9dc: 48 83 c4 48                 	addq	$0x48, %rsp
1408ef9e0: 5b                          	popq	%rbx
1408ef9e1: 5f                          	popq	%rdi
1408ef9e2: 5e                          	popq	%rsi
1408ef9e3: 41 5c                       	popq	%r12
1408ef9e5: 41 5d                       	popq	%r13
1408ef9e7: 41 5e                       	popq	%r14
1408ef9e9: 41 5f                       	popq	%r15
1408ef9eb: 5d                          	popq	%rbp
1408ef9ec: c3                          	retq
1408ef9ed: 0f 1f 00                    	nopl	(%rax)
1408ef9f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408ef9f5: 55                          	pushq	%rbp
1408ef9f6: 41 57                       	pushq	%r15
1408ef9f8: 41 56                       	pushq	%r14
1408ef9fa: 41 55                       	pushq	%r13
1408ef9fc: 41 54                       	pushq	%r12
1408ef9fe: 56                          	pushq	%rsi
1408ef9ff: 57                          	pushq	%rdi
1408efa00: 53                          	pushq	%rbx
1408efa01: 48 83 ec 48                 	subq	$0x48, %rsp
1408efa05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408efa0c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408efa11: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1408efa15: 48 ff 00                    	incq	(%rax)
1408efa18: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408efa1d: 48 83 c4 48                 	addq	$0x48, %rsp
1408efa21: 5b                          	popq	%rbx
1408efa22: 5f                          	popq	%rdi
1408efa23: 5e                          	popq	%rsi
1408efa24: 41 5c                       	popq	%r12
1408efa26: 41 5d                       	popq	%r13
1408efa28: 41 5e                       	popq	%r14
1408efa2a: 41 5f                       	popq	%r15
1408efa2c: 5d                          	popq	%rbp
1408efa2d: c3                          	retq
1408efa2e: 66 90                       	nop
1408efa30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408efa35: 55                          	pushq	%rbp
1408efa36: 41 57                       	pushq	%r15
1408efa38: 41 56                       	pushq	%r14
1408efa3a: 41 55                       	pushq	%r13
1408efa3c: 41 54                       	pushq	%r12
1408efa3e: 56                          	pushq	%rsi
1408efa3f: 57                          	pushq	%rdi
1408efa40: 53                          	pushq	%rbx
1408efa41: 48 83 ec 48                 	subq	$0x48, %rsp
1408efa45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408efa4c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408efa51: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1408efa55: 48 ff 00                    	incq	(%rax)
1408efa58: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408efa5d: 48 83 c4 48                 	addq	$0x48, %rsp
1408efa61: 5b                          	popq	%rbx
1408efa62: 5f                          	popq	%rdi
1408efa63: 5e                          	popq	%rsi
1408efa64: 41 5c                       	popq	%r12
1408efa66: 41 5d                       	popq	%r13
1408efa68: 41 5e                       	popq	%r14
1408efa6a: 41 5f                       	popq	%r15
1408efa6c: 5d                          	popq	%rbp
1408efa6d: c3                          	retq
