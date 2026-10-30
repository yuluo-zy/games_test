
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14207a4f0: 55                          	pushq	%rbp
14207a4f1: 41 57                       	pushq	%r15
14207a4f3: 41 56                       	pushq	%r14
14207a4f5: 41 55                       	pushq	%r13
14207a4f7: 41 54                       	pushq	%r12
14207a4f9: 56                          	pushq	%rsi
14207a4fa: 57                          	pushq	%rdi
14207a4fb: 53                          	pushq	%rbx
14207a4fc: 48 81 ec 98 01 00 00        	subq	$0x198, %rsp            # imm = 0x198
14207a503: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14207a50b: 44 0f 29 95 00 01 00 00     	movaps	%xmm10, 0x100(%rbp)
14207a513: 44 0f 29 8d f0 00 00 00     	movaps	%xmm9, 0xf0(%rbp)
14207a51b: 44 0f 29 85 e0 00 00 00     	movaps	%xmm8, 0xe0(%rbp)
14207a523: 0f 29 bd d0 00 00 00        	movaps	%xmm7, 0xd0(%rbp)
14207a52a: 0f 29 b5 c0 00 00 00        	movaps	%xmm6, 0xc0(%rbp)
14207a531: 48 c7 85 b8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xb8(%rbp)
14207a53c: 48 89 d6                    	movq	%rdx, %rsi
14207a53f: 48 89 cf                    	movq	%rcx, %rdi
14207a542: e8 b9 c4 b9 fe              	callq	0x140c16a00 <.text+0xc15a00>
14207a547: 84 c0                       	testb	%al, %al
14207a549: 74 5f                       	je	0x14207a5aa <.text+0x20795aa>
14207a54b: 4c 8d 35 1c 8c f5 00        	leaq	0xf58c1c(%rip), %r14    # 0x142fd316e
14207a552: bb 41 00 00 00              	movl	$0x41, %ebx
14207a557: ba 41 00 00 00              	movl	$0x41, %edx
14207a55c: 4c 89 f1                    	movq	%r14, %rcx
14207a55f: e8 0c 49 ff ff              	callq	0x14206ee70 <.text+0x206de70>
14207a564: 48 83 f8 01                 	cmpq	$0x1, %rax
14207a568: 0f 85 b1 00 00 00           	jne	0x14207a61f <.text+0x207961f>
14207a56e: 49 89 d1                    	movq	%rdx, %r9
14207a571: 48 85 d2                    	testq	%rdx, %rdx
14207a574: 74 3d                       	je	0x14207a5b3 <.text+0x20795b3>
14207a576: 49 83 f9 41                 	cmpq	$0x41, %r9
14207a57a: 73 35                       	jae	0x14207a5b1 <.text+0x20795b1>
14207a57c: 48 8d 05 eb 8b f5 00        	leaq	0xf58beb(%rip), %rax    # 0x142fd316e
14207a583: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14207a588: 7f 29                       	jg	0x14207a5b3 <.text+0x20795b3>
14207a58a: 48 8d 05 47 8e f5 00        	leaq	0xf58e47(%rip), %rax    # 0x142fd33d8
14207a591: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14207a596: 48 8d 0d d1 8b f5 00        	leaq	0xf58bd1(%rip), %rcx    # 0x142fd316e
14207a59d: ba 41 00 00 00              	movl	$0x41, %edx
14207a5a2: 45 31 c0                    	xorl	%r8d, %r8d
14207a5a5: e8 96 f3 85 00              	callq	0x1428d9940 <.text+0x28d8940>
14207a5aa: 31 c0                       	xorl	%eax, %eax
14207a5ac: e9 bf 01 00 00              	jmp	0x14207a770 <.text+0x2079770>
14207a5b1: 75 d7                       	jne	0x14207a58a <.text+0x207958a>
14207a5b3: 4c 8d 35 b4 8b f5 00        	leaq	0xf58bb4(%rip), %r14    # 0x142fd316e
14207a5ba: 4c 89 f1                    	movq	%r14, %rcx
14207a5bd: 4c 89 ca                    	movq	%r9, %rdx
14207a5c0: e8 ab 48 ff ff              	callq	0x14206ee70 <.text+0x206de70>
14207a5c5: 48 83 f8 01                 	cmpq	$0x1, %rax
14207a5c9: 75 54                       	jne	0x14207a61f <.text+0x207961f>
14207a5cb: 49 89 d0                    	movq	%rdx, %r8
14207a5ce: 49 83 c0 02                 	addq	$0x2, %r8
14207a5d2: 74 39                       	je	0x14207a60d <.text+0x207960d>
14207a5d4: 49 83 f8 41                 	cmpq	$0x41, %r8
14207a5d8: 73 31                       	jae	0x14207a60b <.text+0x207960b>
14207a5da: 48 8d 05 8d 8b f5 00        	leaq	0xf58b8d(%rip), %rax    # 0x142fd316e
14207a5e1: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14207a5e6: 7f 25                       	jg	0x14207a60d <.text+0x207960d>
14207a5e8: 48 8d 05 01 8e f5 00        	leaq	0xf58e01(%rip), %rax    # 0x142fd33f0
14207a5ef: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14207a5f4: 48 8d 0d 73 8b f5 00        	leaq	0xf58b73(%rip), %rcx    # 0x142fd316e
14207a5fb: ba 41 00 00 00              	movl	$0x41, %edx
14207a600: 41 b9 41 00 00 00           	movl	$0x41, %r9d
14207a606: e8 35 f3 85 00              	callq	0x1428d9940 <.text+0x28d8940>
14207a60b: 75 db                       	jne	0x14207a5e8 <.text+0x20795e8>
14207a60d: bb 3f 00 00 00              	movl	$0x3f, %ebx
14207a612: 48 29 d3                    	subq	%rdx, %rbx
14207a615: 4c 8d 35 52 8b f5 00        	leaq	0xf58b52(%rip), %r14    # 0x142fd316e
14207a61c: 4d 01 c6                    	addq	%r8, %r14
14207a61f: 4c 8d 05 38 8e f5 00        	leaq	0xf58e38(%rip), %r8     # 0x142fd345e
14207a626: 4c 8d 3d 03 8e f5 00        	leaq	0xf58e03(%rip), %r15    # 0x142fd3430
14207a62d: 0f 1f 00                    	nopl	(%rax)
14207a630: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14207a635: 85 c0                       	testl	%eax, %eax
14207a637: 78 17                       	js	0x14207a650 <.text+0x2079650>
14207a639: 49 ff c8                    	decq	%r8
14207a63c: 83 f8 5c                    	cmpl	$0x5c, %eax
14207a63f: 75 63                       	jne	0x14207a6a4 <.text+0x20796a4>
14207a641: eb 73                       	jmp	0x14207a6b6 <.text+0x20796b6>
14207a643: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14207a650: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14207a655: 80 f9 c0                    	cmpb	$-0x40, %cl
14207a658: 7d 23                       	jge	0x14207a67d <.text+0x207967d>
14207a65a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14207a65f: 80 fa c0                    	cmpb	$-0x40, %dl
14207a662: 7d 22                       	jge	0x14207a686 <.text+0x2079686>
14207a664: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14207a669: 49 83 c0 fc                 	addq	$-0x4, %r8
14207a66d: 41 83 e1 07                 	andl	$0x7, %r9d
14207a671: 41 c1 e1 06                 	shll	$0x6, %r9d
14207a675: 83 e2 3f                    	andl	$0x3f, %edx
14207a678: 44 09 ca                    	orl	%r9d, %edx
14207a67b: eb 10                       	jmp	0x14207a68d <.text+0x207968d>
14207a67d: 49 83 c0 fe                 	addq	$-0x2, %r8
14207a681: 83 e1 1f                    	andl	$0x1f, %ecx
14207a684: eb 0f                       	jmp	0x14207a695 <.text+0x2079695>
14207a686: 49 83 c0 fd                 	addq	$-0x3, %r8
14207a68a: 83 e2 0f                    	andl	$0xf, %edx
14207a68d: c1 e2 06                    	shll	$0x6, %edx
14207a690: 83 e1 3f                    	andl	$0x3f, %ecx
14207a693: 09 d1                       	orl	%edx, %ecx
14207a695: c1 e1 06                    	shll	$0x6, %ecx
14207a698: 24 3f                       	andb	$0x3f, %al
14207a69a: 0f b6 c0                    	movzbl	%al, %eax
14207a69d: 09 c8                       	orl	%ecx, %eax
14207a69f: 83 f8 5c                    	cmpl	$0x5c, %eax
14207a6a2: 74 12                       	je	0x14207a6b6 <.text+0x20796b6>
14207a6a4: 83 f8 2f                    	cmpl	$0x2f, %eax
14207a6a7: 74 0d                       	je	0x14207a6b6 <.text+0x20796b6>
14207a6a9: 4d 39 f8                    	cmpq	%r15, %r8
14207a6ac: 75 82                       	jne	0x14207a630 <.text+0x2079630>
14207a6ae: 41 bc 2e 00 00 00           	movl	$0x2e, %r12d
14207a6b4: eb 4d                       	jmp	0x14207a703 <.text+0x2079703>
14207a6b6: 4c 8d 3d 73 8d f5 00        	leaq	0xf58d73(%rip), %r15    # 0x142fd3430
14207a6bd: 4d 29 f8                    	subq	%r15, %r8
14207a6c0: 49 ff c0                    	incq	%r8
14207a6c3: 74 32                       	je	0x14207a6f7 <.text+0x20796f7>
14207a6c5: 49 83 f8 2e                 	cmpq	$0x2e, %r8
14207a6c9: 73 2a                       	jae	0x14207a6f5 <.text+0x20796f5>
14207a6cb: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
14207a6d0: 7f 25                       	jg	0x14207a6f7 <.text+0x20796f7>
14207a6d2: 48 8d 05 df 8c f5 00        	leaq	0xf58cdf(%rip), %rax    # 0x142fd33b8
14207a6d9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14207a6de: 48 8d 0d 4b 8d f5 00        	leaq	0xf58d4b(%rip), %rcx    # 0x142fd3430
14207a6e5: ba 2e 00 00 00              	movl	$0x2e, %edx
14207a6ea: 41 b9 2e 00 00 00           	movl	$0x2e, %r9d
14207a6f0: e8 4b f2 85 00              	callq	0x1428d9940 <.text+0x28d8940>
14207a6f5: 75 db                       	jne	0x14207a6d2 <.text+0x20796d2>
14207a6f7: 41 bc 2e 00 00 00           	movl	$0x2e, %r12d
14207a6fd: 4d 29 c4                    	subq	%r8, %r12
14207a700: 4d 01 c7                    	addq	%r8, %r15
14207a703: e8 28 da b9 fe              	callq	0x140c18130 <.text+0xc17130>
14207a708: 48 89 c1                    	movq	%rax, %rcx
14207a70b: 48 8b 00                    	movq	(%rax), %rax
14207a70e: 48 83 f8 01                 	cmpq	$0x1, %rax
14207a712: 0f 85 14 09 00 00           	jne	0x14207b02c <.text+0x207a02c>
14207a718: 48 83 c1 08                 	addq	$0x8, %rcx
14207a71c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14207a720: 0f 85 2a 09 00 00           	jne	0x14207b050 <.text+0x207a050>
14207a726: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14207a72d: 48 89 8d a0 00 00 00        	movq	%rcx, 0xa0(%rbp)
14207a734: 48 83 c1 08                 	addq	$0x8, %rcx
14207a738: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
14207a73d: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14207a746: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14207a74f: 4c 89 f2                    	movq	%r14, %rdx
14207a752: 49 89 d8                    	movq	%rbx, %r8
14207a755: 4d 89 f9                    	movq	%r15, %r9
14207a758: e8 e3 c5 b9 fe              	callq	0x140c16d40 <.text+0xc15d40>
14207a75d: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
14207a764: 48 ff 01                    	incq	(%rcx)
14207a767: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14207a76b: b8 01 00 00 00              	movl	$0x1, %eax
14207a770: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14207a774: 48 8b 0e                    	movq	(%rsi), %rcx
14207a777: b2 12                       	movb	$0x12, %dl
14207a779: e8 d2 74 91 fe              	callq	0x140991c50 <.text+0x990c50>
14207a77e: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
14207a782: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14207a786: 48 8b 0f                    	movq	(%rdi), %rcx
14207a789: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
14207a78d: 4c 8b a9 f8 00 00 00        	movq	0xf8(%rcx), %r13
14207a794: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
14207a79b: 4d 8d 44 85 00              	leaq	(%r13,%rax,4), %r8
14207a7a0: 48 89 8d a8 00 00 00        	movq	%rcx, 0xa8(%rbp)
14207a7a7: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
14207a7ae: 88 85 b3 00 00 00           	movb	%al, 0xb3(%rbp)
14207a7b4: b8 08 00 00 00              	movl	$0x8, %eax
14207a7b9: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14207a7bd: 31 f6                       	xorl	%esi, %esi
14207a7bf: f3 0f 10 3d 3d b1 8a 00     	movss	0x8ab13d(%rip), %xmm7   # 0x142925904
14207a7c7: 44 0f 28 05 b1 41 8b 00     	movaps	0x8b41b1(%rip), %xmm8   # 0x14292e980
14207a7cf: 31 c0                       	xorl	%eax, %eax
14207a7d1: 48 c7 85 98 00 00 00 00 00 00 00    	movq	$0x0, 0x98(%rbp)
14207a7dc: c7 85 b4 00 00 00 00 00 00 00       	movl	$0x0, 0xb4(%rbp)
14207a7e6: 48 c7 45 78 00 00 00 00     	movq	$0x0, 0x78(%rbp)
14207a7ee: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
14207a7f9: 48 c7 85 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbp)
14207a804: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
14207a80c: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
14207a817: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
14207a81b: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
14207a81f: 80 bd b3 00 00 00 00        	cmpb	$0x0, 0xb3(%rbp)
14207a826: 0f 85 94 02 00 00           	jne	0x14207aac0 <.text+0x2079ac0>
14207a82c: 48 39 f0                    	cmpq	%rsi, %rax
14207a82f: 0f 85 61 02 00 00           	jne	0x14207aa96 <.text+0x2079a96>
14207a835: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14207a840: 4d 39 c5                    	cmpq	%r8, %r13
14207a843: 0f 84 1e 07 00 00           	je	0x14207af67 <.text+0x2079f67>
14207a849: 41 8b 4d 00                 	movl	(%r13), %ecx
14207a84d: 49 83 c5 04                 	addq	$0x4, %r13
14207a851: 48 8b 82 00 01 00 00        	movq	0x100(%rdx), %rax
14207a858: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14207a85c: 48 c1 e1 05                 	shlq	$0x5, %rcx
14207a860: 48 8b 74 08 58              	movq	0x58(%rax,%rcx), %rsi
14207a865: 48 85 f6                    	testq	%rsi, %rsi
14207a868: 74 d6                       	je	0x14207a840 <.text+0x2079840>
14207a86a: 48 01 c8                    	addq	%rcx, %rax
14207a86d: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
14207a873: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
14207a87a: 4c 8d 0c c9                 	leaq	(%rcx,%rcx,8), %r9
14207a87e: 4a 8d 0c ca                 	leaq	(%rdx,%r9,8), %rcx
14207a882: 4c 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %r8
14207a889: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
14207a890: 4a 8b 54 ca 40              	movq	0x40(%rdx,%r9,8), %rdx
14207a895: 4c 39 c2                    	cmpq	%r8, %rdx
14207a898: 76 3a                       	jbe	0x14207a8d4 <.text+0x20798d4>
14207a89a: 4c 8b 49 38                 	movq	0x38(%rcx), %r9
14207a89e: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
14207a8a2: 4d 85 c0                    	testq	%r8, %r8
14207a8a5: 74 2d                       	je	0x14207a8d4 <.text+0x20798d4>
14207a8a7: 4c 8b 49 18                 	movq	0x18(%rcx), %r9
14207a8ab: 49 f7 d0                    	notq	%r8
14207a8ae: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14207a8b2: 49 c1 e0 04                 	shlq	$0x4, %r8
14207a8b6: 4f 8b 44 01 10              	movq	0x10(%r9,%r8), %r8
14207a8bb: 4c 89 45 78                 	movq	%r8, 0x78(%rbp)
14207a8bf: 4c 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %r8
14207a8c6: 4d 8b 80 18 01 00 00        	movq	0x118(%r8), %r8
14207a8cd: 4c 39 c2                    	cmpq	%r8, %rdx
14207a8d0: 77 1d                       	ja	0x14207a8ef <.text+0x20798ef>
14207a8d2: eb 58                       	jmp	0x14207a92c <.text+0x207992c>
14207a8d4: 48 c7 45 78 00 00 00 00     	movq	$0x0, 0x78(%rbp)
14207a8dc: 4c 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %r8
14207a8e3: 4d 8b 80 18 01 00 00        	movq	0x118(%r8), %r8
14207a8ea: 4c 39 c2                    	cmpq	%r8, %rdx
14207a8ed: 76 3d                       	jbe	0x14207a92c <.text+0x207992c>
14207a8ef: 4c 8b 49 38                 	movq	0x38(%rcx), %r9
14207a8f3: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
14207a8f7: 4d 85 c0                    	testq	%r8, %r8
14207a8fa: 74 30                       	je	0x14207a92c <.text+0x207992c>
14207a8fc: 4c 8b 49 18                 	movq	0x18(%rcx), %r9
14207a900: 49 f7 d0                    	notq	%r8
14207a903: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14207a907: 49 c1 e0 04                 	shlq	$0x4, %r8
14207a90b: 4f 8b 44 01 10              	movq	0x10(%r9,%r8), %r8
14207a910: 4c 89 85 80 00 00 00        	movq	%r8, 0x80(%rbp)
14207a917: 4c 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %r8
14207a91e: 4d 8b 80 20 01 00 00        	movq	0x120(%r8), %r8
14207a925: 4c 39 c2                    	cmpq	%r8, %rdx
14207a928: 76 4a                       	jbe	0x14207a974 <.text+0x2079974>
14207a92a: eb 1e                       	jmp	0x14207a94a <.text+0x207994a>
14207a92c: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
14207a937: 4c 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %r8
14207a93e: 4d 8b 80 20 01 00 00        	movq	0x120(%r8), %r8
14207a945: 4c 39 c2                    	cmpq	%r8, %rdx
14207a948: 76 2a                       	jbe	0x14207a974 <.text+0x2079974>
14207a94a: 4c 8b 49 38                 	movq	0x38(%rcx), %r9
14207a94e: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
14207a952: 4d 85 c0                    	testq	%r8, %r8
14207a955: 74 1d                       	je	0x14207a974 <.text+0x2079974>
14207a957: 4c 8b 49 18                 	movq	0x18(%rcx), %r9
14207a95b: 49 f7 d0                    	notq	%r8
14207a95e: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14207a962: 49 c1 e0 04                 	shlq	$0x4, %r8
14207a966: 4f 8b 44 01 10              	movq	0x10(%r9,%r8), %r8
14207a96b: 4c 89 85 88 00 00 00        	movq	%r8, 0x88(%rbp)
14207a972: eb 0b                       	jmp	0x14207a97f <.text+0x207997f>
14207a974: 48 c7 85 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbp)
14207a97f: 4c 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %r8
14207a986: 4d 8b 88 28 01 00 00        	movq	0x128(%r8), %r9
14207a98d: 4c 8b 80 88 00 00 00        	movq	0x88(%rax), %r8
14207a994: 4d 39 c8                    	cmpq	%r9, %r8
14207a997: 76 5d                       	jbe	0x14207a9f6 <.text+0x20799f6>
14207a999: 4c 8b 90 80 00 00 00        	movq	0x80(%rax), %r10
14207a9a0: 4b 83 3c ca 00              	cmpq	$0x0, (%r10,%r9,8)
14207a9a5: 74 4f                       	je	0x14207a9f6 <.text+0x20799f6>
14207a9a7: 41 b2 01                    	movb	$0x1, %r10b
14207a9aa: 44 89 95 b4 00 00 00        	movl	%r10d, 0xb4(%rbp)
14207a9b1: 4c 39 ca                    	cmpq	%r9, %rdx
14207a9b4: 76 5f                       	jbe	0x14207aa15 <.text+0x2079a15>
14207a9b6: 4c 8b 51 38                 	movq	0x38(%rcx), %r10
14207a9ba: 4f 8b 0c ca                 	movq	(%r10,%r9,8), %r9
14207a9be: 4d 85 c9                    	testq	%r9, %r9
14207a9c1: 74 52                       	je	0x14207aa15 <.text+0x2079a15>
14207a9c3: 4c 8b 51 18                 	movq	0x18(%rcx), %r10
14207a9c7: 49 f7 d1                    	notq	%r9
14207a9ca: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
14207a9ce: 49 c1 e1 04                 	shlq	$0x4, %r9
14207a9d2: 4f 8b 4c 0a 10              	movq	0x10(%r10,%r9), %r9
14207a9d7: 4c 89 8d 98 00 00 00        	movq	%r9, 0x98(%rbp)
14207a9de: 4c 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %r9
14207a9e5: 4d 8b 89 30 01 00 00        	movq	0x130(%r9), %r9
14207a9ec: 4d 39 c8                    	cmpq	%r9, %r8
14207a9ef: 77 42                       	ja	0x14207aa33 <.text+0x2079a33>
14207a9f1: e9 81 00 00 00              	jmp	0x14207aa77 <.text+0x2079a77>
14207a9f6: c7 85 b4 00 00 00 00 00 00 00       	movl	$0x0, 0xb4(%rbp)
14207aa00: 4c 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %r9
14207aa07: 4d 8b 89 30 01 00 00        	movq	0x130(%r9), %r9
14207aa0e: 4d 39 c8                    	cmpq	%r9, %r8
14207aa11: 77 20                       	ja	0x14207aa33 <.text+0x2079a33>
14207aa13: eb 62                       	jmp	0x14207aa77 <.text+0x2079a77>
14207aa15: 48 c7 85 98 00 00 00 00 00 00 00    	movq	$0x0, 0x98(%rbp)
14207aa20: 4c 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %r9
14207aa27: 4d 8b 89 30 01 00 00        	movq	0x130(%r9), %r9
14207aa2e: 4d 39 c8                    	cmpq	%r9, %r8
14207aa31: 76 44                       	jbe	0x14207aa77 <.text+0x2079a77>
14207aa33: 4c 8b 80 80 00 00 00        	movq	0x80(%rax), %r8
14207aa3a: 4b 83 3c c8 00              	cmpq	$0x0, (%r8,%r9,8)
14207aa3f: 74 36                       	je	0x14207aa77 <.text+0x2079a77>
14207aa41: 41 b0 01                    	movb	$0x1, %r8b
14207aa44: 4c 89 85 90 00 00 00        	movq	%r8, 0x90(%rbp)
14207aa4b: 4c 39 ca                    	cmpq	%r9, %rdx
14207aa4e: 76 34                       	jbe	0x14207aa84 <.text+0x2079a84>
14207aa50: 48 8b 51 38                 	movq	0x38(%rcx), %rdx
14207aa54: 4a 8b 14 ca                 	movq	(%rdx,%r9,8), %rdx
14207aa58: 48 85 d2                    	testq	%rdx, %rdx
14207aa5b: 74 27                       	je	0x14207aa84 <.text+0x2079a84>
14207aa5d: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
14207aa61: 48 f7 d2                    	notq	%rdx
14207aa64: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14207aa68: 48 c1 e2 04                 	shlq	$0x4, %rdx
14207aa6c: 48 8b 4c 11 10              	movq	0x10(%rcx,%rdx), %rcx
14207aa71: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
14207aa75: eb 15                       	jmp	0x14207aa8c <.text+0x2079a8c>
14207aa77: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
14207aa82: eb 08                       	jmp	0x14207aa8c <.text+0x2079a8c>
14207aa84: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
14207aa8c: 48 8b 40 50                 	movq	0x50(%rax), %rax
14207aa90: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14207aa94: 31 c0                       	xorl	%eax, %eax
14207aa96: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14207aa9a: 48 c1 e0 04                 	shlq	$0x4, %rax
14207aa9e: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
14207aaa2: 8b 44 01 08                 	movl	0x8(%rcx,%rax), %eax
14207aaa6: e9 bb 00 00 00              	jmp	0x14207ab66 <.text+0x2079b66>
14207aaab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14207aab0: 4c 89 8d a0 00 00 00        	movq	%r9, 0xa0(%rbp)
14207aab7: e9 d9 02 00 00              	jmp	0x14207ad95 <.text+0x2079d95>
14207aabc: 0f 1f 40 00                 	nopl	(%rax)
14207aac0: 48 39 f0                    	cmpq	%rsi, %rax
14207aac3: 0f 85 97 00 00 00           	jne	0x14207ab60 <.text+0x2079b60>
14207aac9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14207aad0: 4d 39 c5                    	cmpq	%r8, %r13
14207aad3: 0f 84 8e 04 00 00           	je	0x14207af67 <.text+0x2079f67>
14207aad9: 41 8b 4d 00                 	movl	(%r13), %ecx
14207aadd: 49 83 c5 04                 	addq	$0x4, %r13
14207aae1: 48 8b 82 a8 01 00 00        	movq	0x1a8(%rdx), %rax
14207aae8: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14207aaec: 48 8b 74 c8 10              	movq	0x10(%rax,%rcx,8), %rsi
14207aaf1: 48 85 f6                    	testq	%rsi, %rsi
14207aaf4: 74 da                       	je	0x14207aad0 <.text+0x2079ad0>
14207aaf6: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
14207aafa: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14207ab01: 48 8b 91 10 01 00 00        	movq	0x110(%rcx), %rdx
14207ab08: 48 8b 48 40                 	movq	0x40(%rax), %rcx
14207ab0c: 48 39 d1                    	cmpq	%rdx, %rcx
14207ab0f: 0f 86 bf 00 00 00           	jbe	0x14207abd4 <.text+0x2079bd4>
14207ab15: 4c 8b 40 38                 	movq	0x38(%rax), %r8
14207ab19: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
14207ab1d: 48 85 d2                    	testq	%rdx, %rdx
14207ab20: 0f 84 ae 00 00 00           	je	0x14207abd4 <.text+0x2079bd4>
14207ab26: 4c 8b 40 18                 	movq	0x18(%rax), %r8
14207ab2a: 48 f7 d2                    	notq	%rdx
14207ab2d: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14207ab31: 48 c1 e2 04                 	shlq	$0x4, %rdx
14207ab35: 4d 8b 4c 10 10              	movq	0x10(%r8,%rdx), %r9
14207ab3a: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
14207ab41: 48 8b 92 18 01 00 00        	movq	0x118(%rdx), %rdx
14207ab48: 48 39 d1                    	cmpq	%rdx, %rcx
14207ab4b: 0f 87 99 00 00 00           	ja	0x14207abea <.text+0x2079bea>
14207ab51: e9 ca 00 00 00              	jmp	0x14207ac20 <.text+0x2079c20>
14207ab56: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14207ab60: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14207ab64: 89 c0                       	movl	%eax, %eax
14207ab66: 48 8d 1c 40                 	leaq	(%rax,%rax,2), %rbx
14207ab6a: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14207ab6e: 4c 8d 0c 99                 	leaq	(%rcx,%rbx,4), %r9
14207ab72: 48 c1 e3 04                 	shlq	$0x4, %rbx
14207ab76: 48 03 9d 80 00 00 00        	addq	0x80(%rbp), %rbx
14207ab7d: 48 8d 0c c0                 	leaq	(%rax,%rax,8), %rcx
14207ab81: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
14207ab88: 4c 8d 3c 8a                 	leaq	(%rdx,%rcx,4), %r15
14207ab8c: 45 31 e4                    	xorl	%r12d, %r12d
14207ab8f: f6 85 b4 00 00 00 01        	testb	$0x1, 0xb4(%rbp)
14207ab96: b9 00 00 00 00              	movl	$0x0, %ecx
14207ab9b: 48 0f 45 8d 98 00 00 00     	cmovneq	0x98(%rbp), %rcx
14207aba3: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
14207aba7: f6 85 90 00 00 00 01        	testb	$0x1, 0x90(%rbp)
14207abae: 0f 84 fc fe ff ff           	je	0x14207aab0 <.text+0x2079ab0>
14207abb4: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14207abb8: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
14207abbc: 48 85 c0                    	testq	%rax, %rax
14207abbf: 4c 89 8d a0 00 00 00        	movq	%r9, 0xa0(%rbp)
14207abc6: 0f 84 c6 01 00 00           	je	0x14207ad92 <.text+0x2079d92>
14207abcc: 44 8b 20                    	movl	(%rax), %r12d
14207abcf: e9 c1 01 00 00              	jmp	0x14207ad95 <.text+0x2079d95>
14207abd4: 45 31 c9                    	xorl	%r9d, %r9d
14207abd7: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
14207abde: 48 8b 92 18 01 00 00        	movq	0x118(%rdx), %rdx
14207abe5: 48 39 d1                    	cmpq	%rdx, %rcx
14207abe8: 76 36                       	jbe	0x14207ac20 <.text+0x2079c20>
14207abea: 4c 8b 40 38                 	movq	0x38(%rax), %r8
14207abee: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
14207abf2: 48 85 d2                    	testq	%rdx, %rdx
14207abf5: 74 29                       	je	0x14207ac20 <.text+0x2079c20>
14207abf7: 4c 8b 40 18                 	movq	0x18(%rax), %r8
14207abfb: 48 f7 d2                    	notq	%rdx
14207abfe: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14207ac02: 48 c1 e2 04                 	shlq	$0x4, %rdx
14207ac06: 49 8b 5c 10 10              	movq	0x10(%r8,%rdx), %rbx
14207ac0b: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
14207ac12: 48 8b 92 20 01 00 00        	movq	0x120(%rdx), %rdx
14207ac19: 48 39 d1                    	cmpq	%rdx, %rcx
14207ac1c: 76 4d                       	jbe	0x14207ac6b <.text+0x2079c6b>
14207ac1e: eb 15                       	jmp	0x14207ac35 <.text+0x2079c35>
14207ac20: 31 db                       	xorl	%ebx, %ebx
14207ac22: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
14207ac29: 48 8b 92 20 01 00 00        	movq	0x120(%rdx), %rdx
14207ac30: 48 39 d1                    	cmpq	%rdx, %rcx
14207ac33: 76 36                       	jbe	0x14207ac6b <.text+0x2079c6b>
14207ac35: 4c 8b 40 38                 	movq	0x38(%rax), %r8
14207ac39: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
14207ac3d: 48 85 d2                    	testq	%rdx, %rdx
14207ac40: 74 29                       	je	0x14207ac6b <.text+0x2079c6b>
14207ac42: 4c 8b 40 18                 	movq	0x18(%rax), %r8
14207ac46: 48 f7 d2                    	notq	%rdx
14207ac49: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14207ac4d: 48 c1 e2 04                 	shlq	$0x4, %rdx
14207ac51: 4d 8b 7c 10 10              	movq	0x10(%r8,%rdx), %r15
14207ac56: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
14207ac5d: 48 8b 92 28 01 00 00        	movq	0x128(%rdx), %rdx
14207ac64: 48 39 d1                    	cmpq	%rdx, %rcx
14207ac67: 76 61                       	jbe	0x14207acca <.text+0x2079cca>
14207ac69: eb 16                       	jmp	0x14207ac81 <.text+0x2079c81>
14207ac6b: 45 31 ff                    	xorl	%r15d, %r15d
14207ac6e: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
14207ac75: 48 8b 92 28 01 00 00        	movq	0x128(%rdx), %rdx
14207ac7c: 48 39 d1                    	cmpq	%rdx, %rcx
14207ac7f: 76 49                       	jbe	0x14207acca <.text+0x2079cca>
14207ac81: 4c 8b 40 38                 	movq	0x38(%rax), %r8
14207ac85: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
14207ac89: 48 85 d2                    	testq	%rdx, %rdx
14207ac8c: 74 3c                       	je	0x14207acca <.text+0x2079cca>
14207ac8e: 4c 8b 40 18                 	movq	0x18(%rax), %r8
14207ac92: 48 f7 d2                    	notq	%rdx
14207ac95: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14207ac99: 48 c1 e2 04                 	shlq	$0x4, %rdx
14207ac9d: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
14207aca2: 48 89 95 98 00 00 00        	movq	%rdx, 0x98(%rbp)
14207aca9: b2 01                       	movb	$0x1, %dl
14207acab: 89 95 b4 00 00 00           	movl	%edx, 0xb4(%rbp)
14207acb1: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
14207acb8: 48 8b 92 30 01 00 00        	movq	0x130(%rdx), %rdx
14207acbf: 48 39 d1                    	cmpq	%rdx, %rcx
14207acc2: 0f 86 86 00 00 00           	jbe	0x14207ad4e <.text+0x2079d4e>
14207acc8: eb 1d                       	jmp	0x14207ace7 <.text+0x2079ce7>
14207acca: c7 85 b4 00 00 00 00 00 00 00       	movl	$0x0, 0xb4(%rbp)
14207acd4: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
14207acdb: 48 8b 92 30 01 00 00        	movq	0x130(%rdx), %rdx
14207ace2: 48 39 d1                    	cmpq	%rdx, %rcx
14207ace5: 76 67                       	jbe	0x14207ad4e <.text+0x2079d4e>
14207ace7: 48 8b 48 38                 	movq	0x38(%rax), %rcx
14207aceb: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
14207acef: 48 85 c9                    	testq	%rcx, %rcx
14207acf2: 74 5a                       	je	0x14207ad4e <.text+0x2079d4e>
14207acf4: 48 8b 40 18                 	movq	0x18(%rax), %rax
14207acf8: 48 f7 d1                    	notq	%rcx
14207acfb: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14207acff: 48 c1 e1 04                 	shlq	$0x4, %rcx
14207ad03: 48 8b 44 08 10              	movq	0x10(%rax,%rcx), %rax
14207ad08: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14207ad0c: 80 bd b4 00 00 00 00        	cmpb	$0x0, 0xb4(%rbp)
14207ad13: b8 00 00 00 00              	movl	$0x0, %eax
14207ad18: 48 0f 45 85 98 00 00 00     	cmovneq	0x98(%rbp), %rax
14207ad20: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14207ad24: b0 01                       	movb	$0x1, %al
14207ad26: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14207ad2d: 31 c0                       	xorl	%eax, %eax
14207ad2f: 4c 89 bd 88 00 00 00        	movq	%r15, 0x88(%rbp)
14207ad36: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
14207ad3d: 4c 89 4d 78                 	movq	%r9, 0x78(%rbp)
14207ad41: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14207ad49: e9 66 fe ff ff              	jmp	0x14207abb4 <.text+0x2079bb4>
14207ad4e: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14207ad56: 80 bd b4 00 00 00 00        	cmpb	$0x0, 0xb4(%rbp)
14207ad5d: b8 00 00 00 00              	movl	$0x0, %eax
14207ad62: 48 0f 45 85 98 00 00 00     	cmovneq	0x98(%rbp), %rax
14207ad6a: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14207ad6e: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
14207ad79: 4c 89 bd 88 00 00 00        	movq	%r15, 0x88(%rbp)
14207ad80: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
14207ad87: 4c 89 8d a0 00 00 00        	movq	%r9, 0xa0(%rbp)
14207ad8e: 4c 89 4d 78                 	movq	%r9, 0x78(%rbp)
14207ad92: 45 31 e4                    	xorl	%r12d, %r12d
14207ad95: f3 41 0f 10 47 0c           	movss	0xc(%r15), %xmm0
14207ad9b: f3 41 0f 10 4f 10           	movss	0x10(%r15), %xmm1
14207ada1: f3 41 0f 10 77 14           	movss	0x14(%r15), %xmm6
14207ada7: 41 0f b6 7f 20              	movzbl	0x20(%r15), %edi
14207adac: e8 3f 60 c1 fe              	callq	0x140c90df0 <.text+0xc8fdf0>
14207adb1: 41 89 c6                    	movl	%eax, %r14d
14207adb4: f3 41 0f 10 4f 18           	movss	0x18(%r15), %xmm1
14207adba: 0f 28 c6                    	movaps	%xmm6, %xmm0
14207adbd: e8 2e 60 c1 fe              	callq	0x140c90df0 <.text+0xc8fdf0>
14207adc2: 8b 4b 18                    	movl	0x18(%rbx), %ecx
14207adc5: 89 4d 20                    	movl	%ecx, 0x20(%rbp)
14207adc8: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
14207adcc: 48 89 4d 18                 	movq	%rcx, 0x18(%rbp)
14207add0: 0f 28 23                    	movaps	(%rbx), %xmm4
14207add3: f3 0f 10 5b 08              	movss	0x8(%rbx), %xmm3
14207add8: 0f 28 f3                    	movaps	%xmm3, %xmm6
14207addb: f3 0f 58 f3                 	addss	%xmm3, %xmm6
14207addf: f3 0f 59 de                 	mulss	%xmm6, %xmm3
14207ade3: 0f 28 cc                    	movaps	%xmm4, %xmm1
14207ade6: 0f 58 cc                    	addps	%xmm4, %xmm1
14207ade9: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14207aded: f3 0f 59 c4                 	mulss	%xmm4, %xmm0
14207adf1: 0f 28 d4                    	movaps	%xmm4, %xmm2
14207adf4: f3 0f 12 ee                 	movsldup	%xmm6, %xmm5            # xmm5 = xmm6[0,0,2,2]
14207adf8: 0f 59 ec                    	mulps	%xmm4, %xmm5
14207adfb: 0f c6 e4 ff                 	shufps	$0xff, %xmm4, %xmm4     # xmm4 = xmm4[3,3,3,3]
14207adff: 0f 59 d1                    	mulps	%xmm1, %xmm2
14207ae02: 0f 59 cc                    	mulps	%xmm4, %xmm1
14207ae05: 44 0f 28 cc                 	movaps	%xmm4, %xmm9
14207ae09: f3 44 0f 59 ce              	mulss	%xmm6, %xmm9
14207ae0e: f3 44 0f 16 d2              	movshdup	%xmm2, %xmm10           # xmm10 = xmm2[1,1,3,3]
14207ae13: 41 0f 28 f2                 	movaps	%xmm10, %xmm6
14207ae17: f3 0f 58 f3                 	addss	%xmm3, %xmm6
14207ae1b: 0f 28 e7                    	movaps	%xmm7, %xmm4
14207ae1e: f3 0f 5c e6                 	subss	%xmm6, %xmm4
14207ae22: 0f 28 f0                    	movaps	%xmm0, %xmm6
14207ae25: f3 41 0f 58 f1              	addss	%xmm9, %xmm6
14207ae2a: 0f 14 e6                    	unpcklps	%xmm6, %xmm4            # xmm4 = xmm4[0],xmm6[0],xmm4[1],xmm6[1]
14207ae2d: f3 41 0f 5c c1              	subss	%xmm9, %xmm0
14207ae32: f3 0f 58 da                 	addss	%xmm2, %xmm3
14207ae36: 0f 28 f7                    	movaps	%xmm7, %xmm6
14207ae39: f3 0f 5c f3                 	subss	%xmm3, %xmm6
14207ae3d: 0f c6 c9 e1                 	shufps	$0xe1, %xmm1, %xmm1     # xmm1 = xmm1[1,0,2,3]
14207ae41: 0f 14 c6                    	unpcklps	%xmm6, %xmm0            # xmm0 = xmm0[0],xmm6[0],xmm0[1],xmm6[1]
14207ae44: 0f 28 f5                    	movaps	%xmm5, %xmm6
14207ae47: 0f 5c f1                    	subps	%xmm1, %xmm6
14207ae4a: 0f 58 cd                    	addps	%xmm5, %xmm1
14207ae4d: f3 0f 10 5b 20              	movss	0x20(%rbx), %xmm3
14207ae52: f3 41 0f 58 d2              	addss	%xmm10, %xmm2
14207ae57: 44 0f 28 cf                 	movaps	%xmm7, %xmm9
14207ae5b: f3 44 0f 5c ca              	subss	%xmm2, %xmm9
14207ae60: f3 0f 10 6b 24              	movss	0x24(%rbx), %xmm5
14207ae65: 0f 16 e6                    	movlhps	%xmm6, %xmm4            # xmm4 = xmm4[0],xmm6[0]
14207ae68: 0f c6 c1 d4                 	shufps	$0xd4, %xmm1, %xmm0     # xmm0 = xmm0[0,1],xmm1[1,3]
14207ae6c: 0f c6 f1 01                 	shufps	$0x1, %xmm1, %xmm6      # xmm6 = xmm6[1,0],xmm1[0,0]
14207ae70: 44 0f c6 c9 30              	shufps	$0x30, %xmm1, %xmm9     # xmm9 = xmm9[0,0],xmm1[3,0]
14207ae75: 0f 10 53 1c                 	movups	0x1c(%rbx), %xmm2
14207ae79: 41 0f c6 f1 82              	shufps	$0x82, %xmm9, %xmm6     # xmm6 = xmm6[2,0],xmm9[0,2]
14207ae7e: 0f 10 4b 20                 	movups	0x20(%rbx), %xmm1
14207ae82: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
14207ae86: 0f 59 d4                    	mulps	%xmm4, %xmm2
14207ae89: 0f 28 e1                    	movaps	%xmm1, %xmm4
14207ae8c: 0f c6 e1 00                 	shufps	$0x0, %xmm1, %xmm4      # xmm4 = xmm4[0,0],xmm1[0,0]
14207ae90: 0f 59 e0                    	mulps	%xmm0, %xmm4
14207ae93: 0f c6 c9 55                 	shufps	$0x55, %xmm1, %xmm1     # xmm1 = xmm1[1,1,1,1]
14207ae97: 0f 59 ce                    	mulps	%xmm6, %xmm1
14207ae9a: 41 0f 54 d8                 	andps	%xmm8, %xmm3
14207ae9e: 41 0f 54 e8                 	andps	%xmm8, %xmm5
14207aea2: 0f 28 c3                    	movaps	%xmm3, %xmm0
14207aea5: f3 0f c2 c3 03              	cmpunordss	%xmm3, %xmm0
14207aeaa: 0f 28 f0                    	movaps	%xmm0, %xmm6
14207aead: 0f 54 f5                    	andps	%xmm5, %xmm6
14207aeb0: f3 0f 5f eb                 	maxss	%xmm3, %xmm5
14207aeb4: f3 0f 10 5b 1c              	movss	0x1c(%rbx), %xmm3
14207aeb9: 41 0f 54 d8                 	andps	%xmm8, %xmm3
14207aebd: 0f 55 c5                    	andnps	%xmm5, %xmm0
14207aec0: 0f 56 c6                    	orps	%xmm6, %xmm0
14207aec3: 0f 28 e8                    	movaps	%xmm0, %xmm5
14207aec6: f3 0f 5f eb                 	maxss	%xmm3, %xmm5
14207aeca: f3 0f c2 db 03              	cmpunordss	%xmm3, %xmm3
14207aecf: 0f 28 f3                    	movaps	%xmm3, %xmm6
14207aed2: 0f 54 f0                    	andps	%xmm0, %xmm6
14207aed5: 0f 55 dd                    	andnps	%xmm5, %xmm3
14207aed8: 0f 56 de                    	orps	%xmm6, %xmm3
14207aedb: 0f 28 c2                    	movaps	%xmm2, %xmm0
14207aede: f2 0f 10 c4                 	movsd	%xmm4, %xmm0            # xmm0 = xmm4[0],xmm0[1]
14207aee2: 0f c6 d0 24                 	shufps	$0x24, %xmm0, %xmm2     # xmm2 = xmm2[0,1],xmm0[2,0]
14207aee6: 0f 28 43 10                 	movaps	0x10(%rbx), %xmm0
14207aeea: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
14207aeee: 0f c6 e1 49                 	shufps	$0x49, %xmm1, %xmm4     # xmm4 = xmm4[1,2],xmm1[0,1]
14207aef2: 0f 29 65 d0                 	movaps	%xmm4, -0x30(%rbp)
14207aef6: 0f c6 c8 02                 	shufps	$0x2, %xmm0, %xmm1      # xmm1 = xmm1[2,0],xmm0[0,0]
14207aefa: 0f c6 c8 98                 	shufps	$0x98, %xmm0, %xmm1     # xmm1 = xmm1[0,2],xmm0[1,2]
14207aefe: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
14207af02: 31 c9                       	xorl	%ecx, %ecx
14207af04: 48 83 7d 60 00              	cmpq	$0x0, 0x60(%rbp)
14207af09: 0f 95 c1                    	setne	%cl
14207af0c: c1 e1 1f                    	shll	$0x1f, %ecx
14207af0f: 41 09 cc                    	orl	%ecx, %r12d
14207af12: c1 e7 1e                    	shll	$0x1e, %edi
14207af15: 44 09 e7                    	orl	%r12d, %edi
14207af18: 44 89 75 f0                 	movl	%r14d, -0x10(%rbp)
14207af1c: 89 45 f4                    	movl	%eax, -0xc(%rbp)
14207af1f: 89 7d f8                    	movl	%edi, -0x8(%rbp)
14207af22: c7 45 fc 00 00 00 00        	movl	$0x0, -0x4(%rbp)
14207af29: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
14207af2d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14207af32: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
14207af36: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
14207af3d: 4c 8d 45 18                 	leaq	0x18(%rbp), %r8
14207af41: e8 4a 6b 4f ff              	callq	0x141571a90 <.text+0x1570a90>
14207af46: 48 8b 45 68                 	movq	0x68(%rbp), %rax
14207af4a: 48 ff c0                    	incq	%rax
14207af4d: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
14207af51: 4c 8b 45 38                 	movq	0x38(%rbp), %r8
14207af55: 80 bd b3 00 00 00 00        	cmpb	$0x0, 0xb3(%rbp)
14207af5c: 0f 84 ca f8 ff ff           	je	0x14207a82c <.text+0x207982c>
14207af62: e9 59 fb ff ff              	jmp	0x14207aac0 <.text+0x2079ac0>
14207af67: 48 8b 75 58                 	movq	0x58(%rbp), %rsi
14207af6b: 48 89 f1                    	movq	%rsi, %rcx
14207af6e: e8 9d 6e c0 fe              	callq	0x140c81e10 <.text+0xc80e10>
14207af73: 48 89 f1                    	movq	%rsi, %rcx
14207af76: 48 83 c9 01                 	orq	$0x1, %rcx
14207af7a: e8 91 6e c0 fe              	callq	0x140c81e10 <.text+0xc80e10>
14207af7f: 31 c9                       	xorl	%ecx, %ecx
14207af81: b8 08 00 00 00              	movl	$0x8, %eax
14207af86: f0                          	lock
14207af87: 48 0f b1 0e                 	cmpxchgq	%rcx, (%rsi)
14207af8b: 0f 85 cb 00 00 00           	jne	0x14207b05c <.text+0x207a05c>
14207af91: 48 83 7d 40 00              	cmpq	$0x0, 0x40(%rbp)
14207af96: 74 44                       	je	0x14207afdc <.text+0x2079fdc>
14207af98: e8 93 d1 b9 fe              	callq	0x140c18130 <.text+0xc17130>
14207af9d: 48 89 c1                    	movq	%rax, %rcx
14207afa0: 48 8b 00                    	movq	(%rax), %rax
14207afa3: 48 83 f8 01                 	cmpq	$0x1, %rax
14207afa7: 75 6d                       	jne	0x14207b016 <.text+0x207a016>
14207afa9: 48 83 c1 08                 	addq	$0x8, %rcx
14207afad: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14207afb1: 0f 85 99 00 00 00           	jne	0x14207b050 <.text+0x207a050>
14207afb7: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14207afbe: 48 89 8d a0 00 00 00        	movq	%rcx, 0xa0(%rbp)
14207afc5: 48 83 c1 08                 	addq	$0x8, %rcx
14207afc9: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
14207afcd: e8 1e c1 b9 fe              	callq	0x140c170f0 <.text+0xc160f0>
14207afd2: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14207afd9: 48 ff 00                    	incq	(%rax)
14207afdc: 0f 28 b5 c0 00 00 00        	movaps	0xc0(%rbp), %xmm6
14207afe3: 0f 28 bd d0 00 00 00        	movaps	0xd0(%rbp), %xmm7
14207afea: 44 0f 28 85 e0 00 00 00     	movaps	0xe0(%rbp), %xmm8
14207aff2: 44 0f 28 8d f0 00 00 00     	movaps	0xf0(%rbp), %xmm9
14207affa: 44 0f 28 95 00 01 00 00     	movaps	0x100(%rbp), %xmm10
14207b002: 48 81 c4 98 01 00 00        	addq	$0x198, %rsp            # imm = 0x198
14207b009: 5b                          	popq	%rbx
14207b00a: 5f                          	popq	%rdi
14207b00b: 5e                          	popq	%rsi
14207b00c: 41 5c                       	popq	%r12
14207b00e: 41 5d                       	popq	%r13
14207b010: 41 5e                       	popq	%r14
14207b012: 41 5f                       	popq	%r15
14207b014: 5d                          	popq	%rbp
14207b015: c3                          	retq
14207b016: 48 85 c0                    	testq	%rax, %rax
14207b019: 75 29                       	jne	0x14207b044 <.text+0x207a044>
14207b01b: 31 d2                       	xorl	%edx, %edx
14207b01d: e8 ae d7 87 00              	callq	0x1428f87d0 <.text+0x28f77d0>
14207b022: 48 89 c1                    	movq	%rax, %rcx
14207b025: 48 85 c0                    	testq	%rax, %rax
14207b028: 75 83                       	jne	0x14207afad <.text+0x2079fad>
14207b02a: eb 18                       	jmp	0x14207b044 <.text+0x207a044>
14207b02c: 48 85 c0                    	testq	%rax, %rax
14207b02f: 75 13                       	jne	0x14207b044 <.text+0x207a044>
14207b031: 31 d2                       	xorl	%edx, %edx
14207b033: e8 98 d7 87 00              	callq	0x1428f87d0 <.text+0x28f77d0>
14207b038: 48 89 c1                    	movq	%rax, %rcx
14207b03b: 48 85 c0                    	testq	%rax, %rax
14207b03e: 0f 85 d8 f6 ff ff           	jne	0x14207a71c <.text+0x207971c>
14207b044: 48 8d 0d 55 fa bb 00        	leaq	0xbbfa55(%rip), %rcx    # 0x142c3aaa0
14207b04b: e8 80 d0 85 00              	callq	0x1428d80d0 <.text+0x28d70d0>
14207b050: 48 8d 0d 19 fb bb 00        	leaq	0xbbfb19(%rip), %rcx    # 0x142c3ab70
14207b057: e8 f4 e1 85 00              	callq	0x1428d9250 <.text+0x28d8250>
14207b05c: 48 89 f1                    	movq	%rsi, %rcx
14207b05f: 31 d2                       	xorl	%edx, %edx
14207b061: e8 fa 0e 85 00              	callq	0x1428cbf60 <.text+0x28caf60>
14207b066: e9 26 ff ff ff              	jmp	0x14207af91 <.text+0x2079f91>
14207b06b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14207b070: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14207b075: 55                          	pushq	%rbp
14207b076: 41 57                       	pushq	%r15
14207b078: 41 56                       	pushq	%r14
14207b07a: 41 55                       	pushq	%r13
14207b07c: 41 54                       	pushq	%r12
14207b07e: 56                          	pushq	%rsi
14207b07f: 57                          	pushq	%rdi
14207b080: 53                          	pushq	%rbx
14207b081: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
14207b088: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14207b08f: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
14207b095: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
14207b09b: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
14207b0a1: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
14207b0a6: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
14207b0ab: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14207b0b2: 48 ff 00                    	incq	(%rax)
14207b0b5: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
14207b0ba: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
14207b0bf: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
14207b0c5: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
14207b0cb: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
14207b0d1: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
14207b0d8: 5b                          	popq	%rbx
14207b0d9: 5f                          	popq	%rdi
14207b0da: 5e                          	popq	%rsi
14207b0db: 41 5c                       	popq	%r12
14207b0dd: 41 5d                       	popq	%r13
14207b0df: 41 5e                       	popq	%r14
14207b0e1: 41 5f                       	popq	%r15
14207b0e3: 5d                          	popq	%rbp
14207b0e4: c3                          	retq
14207b0e5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14207b0f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14207b0f5: 55                          	pushq	%rbp
14207b0f6: 41 57                       	pushq	%r15
14207b0f8: 41 56                       	pushq	%r14
14207b0fa: 41 55                       	pushq	%r13
14207b0fc: 41 54                       	pushq	%r12
14207b0fe: 56                          	pushq	%rsi
14207b0ff: 57                          	pushq	%rdi
14207b100: 53                          	pushq	%rbx
14207b101: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
14207b108: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14207b10f: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
14207b115: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
14207b11b: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
14207b121: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
14207b126: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
14207b12b: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14207b132: 48 ff 00                    	incq	(%rax)
14207b135: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
14207b13a: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
14207b13f: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
14207b145: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
14207b14b: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
14207b151: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
14207b158: 5b                          	popq	%rbx
14207b159: 5f                          	popq	%rdi
14207b15a: 5e                          	popq	%rsi
14207b15b: 41 5c                       	popq	%r12
14207b15d: 41 5d                       	popq	%r13
14207b15f: 41 5e                       	popq	%r14
14207b161: 41 5f                       	popq	%r15
14207b163: 5d                          	popq	%rbp
14207b164: c3                          	retq
14207b165: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14207b170: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14207b175: 55                          	pushq	%rbp
14207b176: 41 57                       	pushq	%r15
14207b178: 41 56                       	pushq	%r14
14207b17a: 41 55                       	pushq	%r13
14207b17c: 41 54                       	pushq	%r12
14207b17e: 56                          	pushq	%rsi
14207b17f: 57                          	pushq	%rdi
14207b180: 53                          	pushq	%rbx
14207b181: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
14207b188: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14207b18f: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
14207b195: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
14207b19b: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
14207b1a1: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
14207b1a6: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
14207b1ab: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
14207b1af: e8 1c 61 fc fd              	callq	0x1400412d0 <.text+0x402d0>
14207b1b4: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
14207b1b9: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
14207b1be: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
14207b1c4: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
14207b1ca: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
14207b1d0: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
14207b1d7: 5b                          	popq	%rbx
14207b1d8: 5f                          	popq	%rdi
14207b1d9: 5e                          	popq	%rsi
14207b1da: 41 5c                       	popq	%r12
14207b1dc: 41 5d                       	popq	%r13
14207b1de: 41 5e                       	popq	%r14
14207b1e0: 41 5f                       	popq	%r15
14207b1e2: 5d                          	popq	%rbp
14207b1e3: c3                          	retq
14207b1e4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14207b1f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14207b1f5: 55                          	pushq	%rbp
14207b1f6: 41 57                       	pushq	%r15
14207b1f8: 41 56                       	pushq	%r14
14207b1fa: 41 55                       	pushq	%r13
14207b1fc: 41 54                       	pushq	%r12
14207b1fe: 56                          	pushq	%rsi
14207b1ff: 57                          	pushq	%rdi
14207b200: 53                          	pushq	%rbx
14207b201: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
14207b208: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14207b20f: 44 0f 29 54 24 30           	movaps	%xmm10, 0x30(%rsp)
14207b215: 44 0f 29 4c 24 40           	movaps	%xmm9, 0x40(%rsp)
14207b21b: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
14207b221: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
14207b226: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
14207b22b: 48 83 7d 40 00              	cmpq	$0x0, 0x40(%rbp)
14207b230: 74 20                       	je	0x14207b252 <.text+0x207a252>
14207b232: 48 8d 45 48                 	leaq	0x48(%rbp), %rax
14207b236: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14207b23a: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
14207b23e: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14207b242: 48 8d 0d 0f 81 f5 00        	leaq	0xf5810f(%rip), %rcx    # 0x142fd3358
14207b249: 48 8d 55 08                 	leaq	0x8(%rbp), %rdx
14207b24d: e8 7e 40 2b ff              	callq	0x14132f2d0 <.text+0x132e2d0>
14207b252: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
14207b257: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
14207b25c: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
14207b262: 44 0f 28 4c 24 40           	movaps	0x40(%rsp), %xmm9
14207b268: 44 0f 28 54 24 30           	movaps	0x30(%rsp), %xmm10
14207b26e: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
14207b275: 5b                          	popq	%rbx
14207b276: 5f                          	popq	%rdi
14207b277: 5e                          	popq	%rsi
14207b278: 41 5c                       	popq	%r12
14207b27a: 41 5d                       	popq	%r13
14207b27c: 41 5e                       	popq	%r14
14207b27e: 41 5f                       	popq	%r15
14207b280: 5d                          	popq	%rbp
14207b281: c3                          	retq
