
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a8b530: 55                          	pushq	%rbp
140a8b531: 41 57                       	pushq	%r15
140a8b533: 41 56                       	pushq	%r14
140a8b535: 41 55                       	pushq	%r13
140a8b537: 41 54                       	pushq	%r12
140a8b539: 56                          	pushq	%rsi
140a8b53a: 57                          	pushq	%rdi
140a8b53b: 53                          	pushq	%rbx
140a8b53c: 48 81 ec 88 0e 00 00        	subq	$0xe88, %rsp            # imm = 0xE88
140a8b543: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a8b54b: 44 0f 29 bd f0 0d 00 00     	movaps	%xmm15, 0xdf0(%rbp)
140a8b553: 44 0f 29 b5 e0 0d 00 00     	movaps	%xmm14, 0xde0(%rbp)
140a8b55b: 44 0f 29 ad d0 0d 00 00     	movaps	%xmm13, 0xdd0(%rbp)
140a8b563: 44 0f 29 a5 c0 0d 00 00     	movaps	%xmm12, 0xdc0(%rbp)
140a8b56b: 44 0f 29 9d b0 0d 00 00     	movaps	%xmm11, 0xdb0(%rbp)
140a8b573: 44 0f 29 95 a0 0d 00 00     	movaps	%xmm10, 0xda0(%rbp)
140a8b57b: 44 0f 29 8d 90 0d 00 00     	movaps	%xmm9, 0xd90(%rbp)
140a8b583: 44 0f 29 85 80 0d 00 00     	movaps	%xmm8, 0xd80(%rbp)
140a8b58b: 0f 29 bd 70 0d 00 00        	movaps	%xmm7, 0xd70(%rbp)
140a8b592: 66 0f 7f b5 60 0d 00 00     	movdqa	%xmm6, 0xd60(%rbp)
140a8b59a: 48 c7 85 58 0d 00 00 fe ff ff ff    	movq	$-0x2, 0xd58(%rbp)
140a8b5a5: 4d 89 ce                    	movq	%r9, %r14
140a8b5a8: 4c 89 c3                    	movq	%r8, %rbx
140a8b5ab: 48 89 d6                    	movq	%rdx, %rsi
140a8b5ae: 48 89 cf                    	movq	%rcx, %rdi
140a8b5b1: e8 4a b4 18 00              	callq	0x140c16a00 <.text+0xc15a00>
140a8b5b6: 84 c0                       	testb	%al, %al
140a8b5b8: 48 89 9d 10 0d 00 00        	movq	%rbx, 0xd10(%rbp)
140a8b5bf: 74 60                       	je	0x140a8b621 <.text+0xa8a621>
140a8b5c1: 4c 8d 25 09 d6 04 02        	leaq	0x204d609(%rip), %r12   # 0x142ad8bd1
140a8b5c8: 41 bf 5d 00 00 00           	movl	$0x5d, %r15d
140a8b5ce: ba 5d 00 00 00              	movl	$0x5d, %edx
140a8b5d3: 4c 89 e1                    	movq	%r12, %rcx
140a8b5d6: e8 a5 c6 ff ff              	callq	0x140a87c80 <.text+0xa86c80>
140a8b5db: 48 83 f8 01                 	cmpq	$0x1, %rax
140a8b5df: 0f 85 b2 00 00 00           	jne	0x140a8b697 <.text+0xa8a697>
140a8b5e5: 49 89 d1                    	movq	%rdx, %r9
140a8b5e8: 48 85 d2                    	testq	%rdx, %rdx
140a8b5eb: 74 3d                       	je	0x140a8b62a <.text+0xa8a62a>
140a8b5ed: 49 83 f9 5d                 	cmpq	$0x5d, %r9
140a8b5f1: 73 35                       	jae	0x140a8b628 <.text+0xa8a628>
140a8b5f3: 48 8d 05 d7 d5 04 02        	leaq	0x204d5d7(%rip), %rax   # 0x142ad8bd1
140a8b5fa: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140a8b5ff: 7f 29                       	jg	0x140a8b62a <.text+0xa8a62a>
140a8b601: 48 8d 05 40 d7 04 02        	leaq	0x204d740(%rip), %rax   # 0x142ad8d48
140a8b608: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a8b60d: 48 8d 0d bd d5 04 02        	leaq	0x204d5bd(%rip), %rcx   # 0x142ad8bd1
140a8b614: ba 5d 00 00 00              	movl	$0x5d, %edx
140a8b619: 45 31 c0                    	xorl	%r8d, %r8d
140a8b61c: e8 1f e3 e4 01              	callq	0x1428d9940 <.text+0x28d8940>
140a8b621: 31 c0                       	xorl	%eax, %eax
140a8b623: e9 d5 01 00 00              	jmp	0x140a8b7fd <.text+0xa8a7fd>
140a8b628: 75 d7                       	jne	0x140a8b601 <.text+0xa8a601>
140a8b62a: 4c 8d 25 a0 d5 04 02        	leaq	0x204d5a0(%rip), %r12   # 0x142ad8bd1
140a8b631: 4c 89 e1                    	movq	%r12, %rcx
140a8b634: 4c 89 ca                    	movq	%r9, %rdx
140a8b637: e8 44 c6 ff ff              	callq	0x140a87c80 <.text+0xa86c80>
140a8b63c: 48 83 f8 01                 	cmpq	$0x1, %rax
140a8b640: 75 55                       	jne	0x140a8b697 <.text+0xa8a697>
140a8b642: 49 89 d0                    	movq	%rdx, %r8
140a8b645: 49 83 c0 02                 	addq	$0x2, %r8
140a8b649: 74 39                       	je	0x140a8b684 <.text+0xa8a684>
140a8b64b: 49 83 f8 5d                 	cmpq	$0x5d, %r8
140a8b64f: 73 31                       	jae	0x140a8b682 <.text+0xa8a682>
140a8b651: 48 8d 05 79 d5 04 02        	leaq	0x204d579(%rip), %rax   # 0x142ad8bd1
140a8b658: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140a8b65d: 7f 25                       	jg	0x140a8b684 <.text+0xa8a684>
140a8b65f: 48 8d 05 fa d6 04 02        	leaq	0x204d6fa(%rip), %rax   # 0x142ad8d60
140a8b666: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a8b66b: 48 8d 0d 5f d5 04 02        	leaq	0x204d55f(%rip), %rcx   # 0x142ad8bd1
140a8b672: ba 5d 00 00 00              	movl	$0x5d, %edx
140a8b677: 41 b9 5d 00 00 00           	movl	$0x5d, %r9d
140a8b67d: e8 be e2 e4 01              	callq	0x1428d9940 <.text+0x28d8940>
140a8b682: 75 db                       	jne	0x140a8b65f <.text+0xa8a65f>
140a8b684: 41 bf 5b 00 00 00           	movl	$0x5b, %r15d
140a8b68a: 49 29 d7                    	subq	%rdx, %r15
140a8b68d: 4c 8d 25 3d d5 04 02        	leaq	0x204d53d(%rip), %r12   # 0x142ad8bd1
140a8b694: 4d 01 c4                    	addq	%r8, %r12
140a8b697: 48 8d 05 16 dc 04 02        	leaq	0x204dc16(%rip), %rax   # 0x142ad92b4
140a8b69e: 4c 8d 2d c3 db 04 02        	leaq	0x204dbc3(%rip), %r13   # 0x142ad9268
140a8b6a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a8b6b0: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140a8b6b4: 85 c9                       	testl	%ecx, %ecx
140a8b6b6: 78 18                       	js	0x140a8b6d0 <.text+0xa8a6d0>
140a8b6b8: 48 ff c8                    	decq	%rax
140a8b6bb: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a8b6be: 75 69                       	jne	0x140a8b729 <.text+0xa8a729>
140a8b6c0: eb 7c                       	jmp	0x140a8b73e <.text+0xa8a73e>
140a8b6c2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a8b6d0: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
140a8b6d4: 80 fa c0                    	cmpb	$-0x40, %dl
140a8b6d7: 7d 25                       	jge	0x140a8b6fe <.text+0xa8a6fe>
140a8b6d9: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
140a8b6de: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
140a8b6e2: 7d 23                       	jge	0x140a8b707 <.text+0xa8a707>
140a8b6e4: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
140a8b6e9: 48 83 c0 fc                 	addq	$-0x4, %rax
140a8b6ed: 41 83 e1 07                 	andl	$0x7, %r9d
140a8b6f1: 41 c1 e1 06                 	shll	$0x6, %r9d
140a8b6f5: 41 83 e0 3f                 	andl	$0x3f, %r8d
140a8b6f9: 45 09 c8                    	orl	%r9d, %r8d
140a8b6fc: eb 11                       	jmp	0x140a8b70f <.text+0xa8a70f>
140a8b6fe: 48 83 c0 fe                 	addq	$-0x2, %rax
140a8b702: 83 e2 1f                    	andl	$0x1f, %edx
140a8b705: eb 12                       	jmp	0x140a8b719 <.text+0xa8a719>
140a8b707: 48 83 c0 fd                 	addq	$-0x3, %rax
140a8b70b: 41 83 e0 0f                 	andl	$0xf, %r8d
140a8b70f: 41 c1 e0 06                 	shll	$0x6, %r8d
140a8b713: 83 e2 3f                    	andl	$0x3f, %edx
140a8b716: 44 09 c2                    	orl	%r8d, %edx
140a8b719: c1 e2 06                    	shll	$0x6, %edx
140a8b71c: 80 e1 3f                    	andb	$0x3f, %cl
140a8b71f: 0f b6 c9                    	movzbl	%cl, %ecx
140a8b722: 09 d1                       	orl	%edx, %ecx
140a8b724: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a8b727: 74 15                       	je	0x140a8b73e <.text+0xa8a73e>
140a8b729: 83 f9 2f                    	cmpl	$0x2f, %ecx
140a8b72c: 74 10                       	je	0x140a8b73e <.text+0xa8a73e>
140a8b72e: 4c 39 e8                    	cmpq	%r13, %rax
140a8b731: 0f 85 79 ff ff ff           	jne	0x140a8b6b0 <.text+0xa8a6b0>
140a8b737: bb 4c 00 00 00              	movl	$0x4c, %ebx
140a8b73c: eb 4f                       	jmp	0x140a8b78d <.text+0xa8a78d>
140a8b73e: 4c 8d 2d 23 db 04 02        	leaq	0x204db23(%rip), %r13   # 0x142ad9268
140a8b745: 4c 29 e8                    	subq	%r13, %rax
140a8b748: 49 89 c0                    	movq	%rax, %r8
140a8b74b: 49 ff c0                    	incq	%r8
140a8b74e: 74 32                       	je	0x140a8b782 <.text+0xa8a782>
140a8b750: 49 83 f8 4c                 	cmpq	$0x4c, %r8
140a8b754: 73 2a                       	jae	0x140a8b780 <.text+0xa8a780>
140a8b756: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
140a8b75b: 7f 25                       	jg	0x140a8b782 <.text+0xa8a782>
140a8b75d: 48 8d 05 c4 d5 04 02        	leaq	0x204d5c4(%rip), %rax   # 0x142ad8d28
140a8b764: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a8b769: 48 8d 0d f8 da 04 02        	leaq	0x204daf8(%rip), %rcx   # 0x142ad9268
140a8b770: ba 4c 00 00 00              	movl	$0x4c, %edx
140a8b775: 41 b9 4c 00 00 00           	movl	$0x4c, %r9d
140a8b77b: e8 c0 e1 e4 01              	callq	0x1428d9940 <.text+0x28d8940>
140a8b780: 75 db                       	jne	0x140a8b75d <.text+0xa8a75d>
140a8b782: bb 4b 00 00 00              	movl	$0x4b, %ebx
140a8b787: 48 29 c3                    	subq	%rax, %rbx
140a8b78a: 4d 01 c5                    	addq	%r8, %r13
140a8b78d: e8 9e c9 18 00              	callq	0x140c18130 <.text+0xc17130>
140a8b792: 48 89 c1                    	movq	%rax, %rcx
140a8b795: 48 8b 00                    	movq	(%rax), %rax
140a8b798: 48 83 f8 01                 	cmpq	$0x1, %rax
140a8b79c: 0f 85 f5 25 00 00           	jne	0x140a8dd97 <.text+0xa8cd97>
140a8b7a2: 48 83 c1 08                 	addq	$0x8, %rcx
140a8b7a6: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a8b7aa: 0f 85 e7 27 00 00           	jne	0x140a8df97 <.text+0xa8cf97>
140a8b7b0: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a8b7b7: 48 89 8d 48 0d 00 00        	movq	%rcx, 0xd48(%rbp)
140a8b7be: 48 83 c1 08                 	addq	$0x8, %rcx
140a8b7c2: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140a8b7c7: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140a8b7d0: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140a8b7d9: 4c 89 e2                    	movq	%r12, %rdx
140a8b7dc: 4d 89 f8                    	movq	%r15, %r8
140a8b7df: 4d 89 e9                    	movq	%r13, %r9
140a8b7e2: e8 59 b5 18 00              	callq	0x140c16d40 <.text+0xc15d40>
140a8b7e7: 48 8b 8d 48 0d 00 00        	movq	0xd48(%rbp), %rcx
140a8b7ee: 48 ff 01                    	incq	(%rcx)
140a8b7f1: 48 89 85 08 0c 00 00        	movq	%rax, 0xc08(%rbp)
140a8b7f8: b8 01 00 00 00              	movl	$0x1, %eax
140a8b7fd: 4c 89 b5 00 0d 00 00        	movq	%r14, 0xd00(%rbp)
140a8b804: 48 8b 8d 90 0e 00 00        	movq	0xe90(%rbp), %rcx
140a8b80b: 4c 8b bd 78 0e 00 00        	movq	0xe78(%rbp), %r15
140a8b812: 48 89 85 00 0c 00 00        	movq	%rax, 0xc00(%rbp)
140a8b819: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140a8b81c: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140a8b820: 89 02                       	movl	%eax, (%rdx)
140a8b822: 48 8b 1e                    	movq	(%rsi), %rbx
140a8b825: 8b 47 1c                    	movl	0x1c(%rdi), %eax
140a8b828: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140a8b82c: 89 02                       	movl	%eax, (%rdx)
140a8b82e: 48 8b 07                    	movq	(%rdi), %rax
140a8b831: 48 89 85 48 0d 00 00        	movq	%rax, 0xd48(%rbp)
140a8b838: 48 8b 01                    	movq	(%rcx), %rax
140a8b83b: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
140a8b83f: 48 8b 10                    	movq	(%rax), %rdx
140a8b842: 45 31 c0                    	xorl	%r8d, %r8d
140a8b845: 49 89 d1                    	movq	%rdx, %r9
140a8b848: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
140a8b84c: 4d 0f 42 c8                 	cmovbq	%r8, %r9
140a8b850: 48 2b 51 38                 	subq	0x38(%rcx), %rdx
140a8b854: 49 0f 42 d0                 	cmovbq	%r8, %rdx
140a8b858: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
140a8b85c: 49 29 d2                    	subq	%rdx, %r10
140a8b85f: 4d 0f 42 d0                 	cmovbq	%r8, %r10
140a8b863: 0f 97 c2                    	seta	%dl
140a8b866: 4c 8b 59 10                 	movq	0x10(%rcx), %r11
140a8b86a: 4d 29 cb                    	subq	%r9, %r11
140a8b86d: 4d 0f 42 d8                 	cmovbq	%r8, %r11
140a8b871: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
140a8b875: 41 0f 97 c0                 	seta	%r8b
140a8b879: 4d 01 d3                    	addq	%r10, %r11
140a8b87c: 49 89 c9                    	movq	%rcx, %r9
140a8b87f: 4d 29 d9                    	subq	%r11, %r9
140a8b882: 4c 89 08                    	movq	%r9, (%rax)
140a8b885: 41 08 d0                    	orb	%dl, %r8b
140a8b888: 48 89 9d f8 0c 00 00        	movq	%rbx, 0xcf8(%rbp)
140a8b88f: 0f 85 74 05 00 00           	jne	0x140a8be09 <.text+0xa8ae09>
140a8b895: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
140a8b899: 89 85 e0 0c 00 00           	movl	%eax, 0xce0(%rbp)
140a8b89f: 4d 8b 37                    	movq	(%r15), %r14
140a8b8a2: 49 8b 47 10                 	movq	0x10(%r15), %rax
140a8b8a6: 48 89 85 e8 0c 00 00        	movq	%rax, 0xce8(%rbp)
140a8b8ad: 48 8b 85 80 0e 00 00        	movq	0xe80(%rbp), %rax
140a8b8b4: 4c 8b 28                    	movq	(%rax), %r13
140a8b8b7: 48 8b 48 08                 	movq	0x8(%rax), %rcx
140a8b8bb: 4d 8b 45 00                 	movq	(%r13), %r8
140a8b8bf: 31 d2                       	xorl	%edx, %edx
140a8b8c1: 4c 89 c0                    	movq	%r8, %rax
140a8b8c4: 48 2b 41 18                 	subq	0x18(%rcx), %rax
140a8b8c8: 48 0f 42 c2                 	cmovbq	%rdx, %rax
140a8b8cc: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
140a8b8d0: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
140a8b8d4: 48 89 c3                    	movq	%rax, %rbx
140a8b8d7: 48 c1 e3 04                 	shlq	$0x4, %rbx
140a8b8db: 48 03 59 08                 	addq	0x8(%rcx), %rbx
140a8b8df: 4c 8b 51 10                 	movq	0x10(%rcx), %r10
140a8b8e3: 4c 8b 59 30                 	movq	0x30(%rcx), %r11
140a8b8e7: 49 29 c2                    	subq	%rax, %r10
140a8b8ea: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
140a8b8ee: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140a8b8f4: 49 0f 42 d9                 	cmovbq	%r9, %rbx
140a8b8f8: 4c 89 c0                    	movq	%r8, %rax
140a8b8fb: 48 c1 e0 04                 	shlq	$0x4, %rax
140a8b8ff: 48 03 41 28                 	addq	0x28(%rcx), %rax
140a8b903: 4d 29 c3                    	subq	%r8, %r11
140a8b906: 4c 0f 42 da                 	cmovbq	%rdx, %r11
140a8b90a: 49 0f 42 c1                 	cmovbq	%r9, %rax
140a8b90e: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
140a8b912: 4c 29 d9                    	subq	%r11, %rcx
140a8b915: 4c 29 d1                    	subq	%r10, %rcx
140a8b918: 49 89 4d 00                 	movq	%rcx, (%r13)
140a8b91c: 49 c1 e2 04                 	shlq	$0x4, %r10
140a8b920: 49 01 da                    	addq	%rbx, %r10
140a8b923: 4c 89 95 30 0d 00 00        	movq	%r10, 0xd30(%rbp)
140a8b92a: 49 c1 e3 04                 	shlq	$0x4, %r11
140a8b92e: 49 01 c3                    	addq	%rax, %r11
140a8b931: 4c 89 9d f0 0c 00 00        	movq	%r11, 0xcf0(%rbp)
140a8b938: 48 8b 8d 48 0d 00 00        	movq	0xd48(%rbp), %rcx
140a8b93f: 48 83 c1 20                 	addq	$0x20, %rcx
140a8b943: 48 89 8d c0 0c 00 00        	movq	%rcx, 0xcc0(%rbp)
140a8b94a: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140a8b94e: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
140a8b952: 48 89 8d 20 0d 00 00        	movq	%rcx, 0xd20(%rbp)
140a8b959: eb 2a                       	jmp	0x140a8b985 <.text+0xa8a985>
140a8b95b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140a8b960: 49 8b 46 28                 	movq	0x28(%r14), %rax
140a8b964: 48 89 f1                    	movq	%rsi, %rcx
140a8b967: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a8b96b: 4c 89 3c 08                 	movq	%r15, (%rax,%rcx)
140a8b96f: 48 89 5c 08 08              	movq	%rbx, 0x8(%rax,%rcx)
140a8b974: 48 ff c6                    	incq	%rsi
140a8b977: 49 89 76 30                 	movq	%rsi, 0x30(%r14)
140a8b97b: 49 ff 46 40                 	incq	0x40(%r14)
140a8b97f: 48 89 f8                    	movq	%rdi, %rax
140a8b982: 4c 89 e3                    	movq	%r12, %rbx
140a8b985: 48 85 db                    	testq	%rbx, %rbx
140a8b988: 74 16                       	je	0x140a8b9a0 <.text+0xa8a9a0>
140a8b98a: 48 3b 9d 30 0d 00 00        	cmpq	0xd30(%rbp), %rbx
140a8b991: 74 0d                       	je	0x140a8b9a0 <.text+0xa8a9a0>
140a8b993: 4c 8d 63 10                 	leaq	0x10(%rbx), %r12
140a8b997: 48 89 c7                    	movq	%rax, %rdi
140a8b99a: eb 24                       	jmp	0x140a8b9c0 <.text+0xa8a9c0>
140a8b99c: 0f 1f 40 00                 	nopl	(%rax)
140a8b9a0: 48 85 c0                    	testq	%rax, %rax
140a8b9a3: 0f 84 24 01 00 00           	je	0x140a8bacd <.text+0xa8aacd>
140a8b9a9: 48 3b 85 f0 0c 00 00        	cmpq	0xcf0(%rbp), %rax
140a8b9b0: 0f 84 17 01 00 00           	je	0x140a8bacd <.text+0xa8aacd>
140a8b9b6: 48 8d 78 10                 	leaq	0x10(%rax), %rdi
140a8b9ba: 45 31 e4                    	xorl	%r12d, %r12d
140a8b9bd: 48 89 c3                    	movq	%rax, %rbx
140a8b9c0: 49 ff 45 00                 	incq	(%r13)
140a8b9c4: 48 8b 73 08                 	movq	0x8(%rbx), %rsi
140a8b9c8: 48 89 b5 40 08 00 00        	movq	%rsi, 0x840(%rbp)
140a8b9cf: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8b9d6: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
140a8b9db: 74 a2                       	je	0x140a8b97f <.text+0xa8a97f>
140a8b9dd: 48 8b 8d c0 0c 00 00        	movq	0xcc0(%rbp), %rcx
140a8b9e4: 48 8d 95 40 08 00 00        	leaq	0x840(%rbp), %rdx
140a8b9eb: e8 80 53 b2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a8b9f0: 48 8b 95 48 0d 00 00        	movq	0xd48(%rbp), %rdx
140a8b9f7: 48 8b 0a                    	movq	(%rdx), %rcx
140a8b9fa: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140a8b9fe: 49 89 d0                    	movq	%rdx, %r8
140a8ba01: 49 21 c0                    	andq	%rax, %r8
140a8ba04: 48 c1 e8 39                 	shrq	$0x39, %rax
140a8ba08: 66 0f 6e c0                 	movd	%eax, %xmm0
140a8ba0c: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8ba10: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8ba15: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8ba1a: 48 8d 41 88                 	leaq	-0x78(%rcx), %rax
140a8ba1e: 45 31 c9                    	xorl	%r9d, %r9d
140a8ba21: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140a8ba27: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a8ba2b: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a8ba2f: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140a8ba34: 45 85 d2                    	testl	%r10d, %r10d
140a8ba37: 74 27                       	je	0x140a8ba60 <.text+0xa8aa60>
140a8ba39: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140a8ba3e: 4d 01 c3                    	addq	%r8, %r11
140a8ba41: 49 21 d3                    	andq	%rdx, %r11
140a8ba44: 49 f7 db                    	negq	%r11
140a8ba47: 4d 6b db 78                 	imulq	$0x78, %r11, %r11
140a8ba4b: 4a 39 34 18                 	cmpq	%rsi, (%rax,%r11)
140a8ba4f: 74 3f                       	je	0x140a8ba90 <.text+0xa8aa90>
140a8ba51: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140a8ba55: 66 45 21 d3                 	andw	%r10w, %r11w
140a8ba59: 45 89 da                    	movl	%r11d, %r10d
140a8ba5c: 75 db                       	jne	0x140a8ba39 <.text+0xa8aa39>
140a8ba5e: 66 90                       	nop
140a8ba60: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140a8ba64: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140a8ba69: 45 85 d2                    	testl	%r10d, %r10d
140a8ba6c: 0f 85 0d ff ff ff           	jne	0x140a8b97f <.text+0xa8a97f>
140a8ba72: 4d 01 c8                    	addq	%r9, %r8
140a8ba75: 49 83 c0 10                 	addq	$0x10, %r8
140a8ba79: 49 83 c1 10                 	addq	$0x10, %r9
140a8ba7d: 49 21 d0                    	andq	%rdx, %r8
140a8ba80: eb 9f                       	jmp	0x140a8ba21 <.text+0xa8aa21>
140a8ba82: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a8ba90: 48 8b 5b 08                 	movq	0x8(%rbx), %rbx
140a8ba94: 8b 85 e0 0c 00 00           	movl	0xce0(%rbp), %eax
140a8ba9a: 48 8b 8d e8 0c 00 00        	movq	0xce8(%rbp), %rcx
140a8baa1: 89 01                       	movl	%eax, (%rcx)
140a8baa3: 49 8b 76 30                 	movq	0x30(%r14), %rsi
140a8baa7: 4d 8b 7e 40                 	movq	0x40(%r14), %r15
140a8baab: 49 3b 76 20                 	cmpq	0x20(%r14), %rsi
140a8baaf: 0f 85 ab fe ff ff           	jne	0x140a8b960 <.text+0xa8a960>
140a8bab5: 48 8b 8d 20 0d 00 00        	movq	0xd20(%rbp), %rcx
140a8babc: 48 8d 15 85 e6 1a 02        	leaq	0x21ae685(%rip), %rdx   # 0x142c3a148
140a8bac3: e8 68 6b ad ff              	callq	0x140562630 <.text+0x561630>
140a8bac8: e9 93 fe ff ff              	jmp	0x140a8b960 <.text+0xa8a960>
140a8bacd: 48 8b 85 88 0e 00 00        	movq	0xe88(%rbp), %rax
140a8bad4: 4c 8b 28                    	movq	(%rax), %r13
140a8bad7: 48 8b 48 08                 	movq	0x8(%rax), %rcx
140a8badb: 4d 8b 45 00                 	movq	(%r13), %r8
140a8badf: 31 d2                       	xorl	%edx, %edx
140a8bae1: 4c 89 c0                    	movq	%r8, %rax
140a8bae4: 48 2b 41 18                 	subq	0x18(%rcx), %rax
140a8bae8: 48 0f 42 c2                 	cmovbq	%rdx, %rax
140a8baec: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
140a8baf0: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
140a8baf4: 48 89 c3                    	movq	%rax, %rbx
140a8baf7: 48 c1 e3 04                 	shlq	$0x4, %rbx
140a8bafb: 48 03 59 08                 	addq	0x8(%rcx), %rbx
140a8baff: 4c 8b 51 10                 	movq	0x10(%rcx), %r10
140a8bb03: 4c 8b 59 30                 	movq	0x30(%rcx), %r11
140a8bb07: 49 29 c2                    	subq	%rax, %r10
140a8bb0a: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
140a8bb0e: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140a8bb14: 49 0f 42 d9                 	cmovbq	%r9, %rbx
140a8bb18: 4c 89 c0                    	movq	%r8, %rax
140a8bb1b: 48 c1 e0 04                 	shlq	$0x4, %rax
140a8bb1f: 48 03 41 28                 	addq	0x28(%rcx), %rax
140a8bb23: 4d 29 c3                    	subq	%r8, %r11
140a8bb26: 4c 0f 42 da                 	cmovbq	%rdx, %r11
140a8bb2a: 49 0f 42 c1                 	cmovbq	%r9, %rax
140a8bb2e: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
140a8bb32: 4c 29 d9                    	subq	%r11, %rcx
140a8bb35: 4c 29 d1                    	subq	%r10, %rcx
140a8bb38: 49 89 4d 00                 	movq	%rcx, (%r13)
140a8bb3c: 49 c1 e2 04                 	shlq	$0x4, %r10
140a8bb40: 49 01 da                    	addq	%rbx, %r10
140a8bb43: 4c 89 95 30 0d 00 00        	movq	%r10, 0xd30(%rbp)
140a8bb4a: 49 c1 e3 04                 	shlq	$0x4, %r11
140a8bb4e: 49 01 c3                    	addq	%rax, %r11
140a8bb51: 4c 89 9d f0 0c 00 00        	movq	%r11, 0xcf0(%rbp)
140a8bb58: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140a8bb5c: eb 27                       	jmp	0x140a8bb85 <.text+0xa8ab85>
140a8bb5e: 66 90                       	nop
140a8bb60: 49 8b 46 28                 	movq	0x28(%r14), %rax
140a8bb64: 48 89 d9                    	movq	%rbx, %rcx
140a8bb67: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a8bb6b: 48 89 34 08                 	movq	%rsi, (%rax,%rcx)
140a8bb6f: 4c 89 7c 08 08              	movq	%r15, 0x8(%rax,%rcx)
140a8bb74: 48 ff c3                    	incq	%rbx
140a8bb77: 49 89 5e 30                 	movq	%rbx, 0x30(%r14)
140a8bb7b: 49 ff 46 40                 	incq	0x40(%r14)
140a8bb7f: 4c 89 e3                    	movq	%r12, %rbx
140a8bb82: 48 89 f8                    	movq	%rdi, %rax
140a8bb85: 48 85 db                    	testq	%rbx, %rbx
140a8bb88: 74 16                       	je	0x140a8bba0 <.text+0xa8aba0>
140a8bb8a: 48 3b 9d 30 0d 00 00        	cmpq	0xd30(%rbp), %rbx
140a8bb91: 74 0d                       	je	0x140a8bba0 <.text+0xa8aba0>
140a8bb93: 4c 8d 63 10                 	leaq	0x10(%rbx), %r12
140a8bb97: 48 89 c7                    	movq	%rax, %rdi
140a8bb9a: eb 24                       	jmp	0x140a8bbc0 <.text+0xa8abc0>
140a8bb9c: 0f 1f 40 00                 	nopl	(%rax)
140a8bba0: 48 85 c0                    	testq	%rax, %rax
140a8bba3: 0f 84 24 01 00 00           	je	0x140a8bccd <.text+0xa8accd>
140a8bba9: 48 3b 85 f0 0c 00 00        	cmpq	0xcf0(%rbp), %rax
140a8bbb0: 0f 84 17 01 00 00           	je	0x140a8bccd <.text+0xa8accd>
140a8bbb6: 48 8d 78 10                 	leaq	0x10(%rax), %rdi
140a8bbba: 45 31 e4                    	xorl	%r12d, %r12d
140a8bbbd: 48 89 c3                    	movq	%rax, %rbx
140a8bbc0: 49 ff 45 00                 	incq	(%r13)
140a8bbc4: 4c 8b 7b 08                 	movq	0x8(%rbx), %r15
140a8bbc8: 4c 89 bd 40 08 00 00        	movq	%r15, 0x840(%rbp)
140a8bbcf: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8bbd6: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
140a8bbdb: 74 a2                       	je	0x140a8bb7f <.text+0xa8ab7f>
140a8bbdd: 48 8b 8d c0 0c 00 00        	movq	0xcc0(%rbp), %rcx
140a8bbe4: 48 8d 95 40 08 00 00        	leaq	0x840(%rbp), %rdx
140a8bbeb: e8 80 51 b2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a8bbf0: 48 8b 95 48 0d 00 00        	movq	0xd48(%rbp), %rdx
140a8bbf7: 48 8b 0a                    	movq	(%rdx), %rcx
140a8bbfa: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140a8bbfe: 49 89 d0                    	movq	%rdx, %r8
140a8bc01: 49 21 c0                    	andq	%rax, %r8
140a8bc04: 48 c1 e8 39                 	shrq	$0x39, %rax
140a8bc08: 66 0f 6e c0                 	movd	%eax, %xmm0
140a8bc0c: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8bc10: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8bc15: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8bc1a: 48 8d 41 88                 	leaq	-0x78(%rcx), %rax
140a8bc1e: 45 31 c9                    	xorl	%r9d, %r9d
140a8bc21: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140a8bc27: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a8bc2b: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a8bc2f: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140a8bc34: 45 85 d2                    	testl	%r10d, %r10d
140a8bc37: 74 27                       	je	0x140a8bc60 <.text+0xa8ac60>
140a8bc39: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140a8bc3e: 4d 01 c3                    	addq	%r8, %r11
140a8bc41: 49 21 d3                    	andq	%rdx, %r11
140a8bc44: 49 f7 db                    	negq	%r11
140a8bc47: 4d 6b db 78                 	imulq	$0x78, %r11, %r11
140a8bc4b: 4e 39 3c 18                 	cmpq	%r15, (%rax,%r11)
140a8bc4f: 74 3f                       	je	0x140a8bc90 <.text+0xa8ac90>
140a8bc51: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140a8bc55: 66 45 21 d3                 	andw	%r10w, %r11w
140a8bc59: 45 89 da                    	movl	%r11d, %r10d
140a8bc5c: 75 db                       	jne	0x140a8bc39 <.text+0xa8ac39>
140a8bc5e: 66 90                       	nop
140a8bc60: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140a8bc64: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140a8bc69: 45 85 d2                    	testl	%r10d, %r10d
140a8bc6c: 0f 85 0d ff ff ff           	jne	0x140a8bb7f <.text+0xa8ab7f>
140a8bc72: 4d 01 c8                    	addq	%r9, %r8
140a8bc75: 49 83 c0 10                 	addq	$0x10, %r8
140a8bc79: 49 83 c1 10                 	addq	$0x10, %r9
140a8bc7d: 49 21 d0                    	andq	%rdx, %r8
140a8bc80: eb 9f                       	jmp	0x140a8bc21 <.text+0xa8ac21>
140a8bc82: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a8bc90: 4c 8b 7b 08                 	movq	0x8(%rbx), %r15
140a8bc94: 8b 85 e0 0c 00 00           	movl	0xce0(%rbp), %eax
140a8bc9a: 48 8b 8d e8 0c 00 00        	movq	0xce8(%rbp), %rcx
140a8bca1: 89 01                       	movl	%eax, (%rcx)
140a8bca3: 49 8b 5e 30                 	movq	0x30(%r14), %rbx
140a8bca7: 49 8b 76 40                 	movq	0x40(%r14), %rsi
140a8bcab: 49 3b 5e 20                 	cmpq	0x20(%r14), %rbx
140a8bcaf: 0f 85 ab fe ff ff           	jne	0x140a8bb60 <.text+0xa8ab60>
140a8bcb5: 48 8b 8d 20 0d 00 00        	movq	0xd20(%rbp), %rcx
140a8bcbc: 48 8d 15 85 e4 1a 02        	leaq	0x21ae485(%rip), %rdx   # 0x142c3a148
140a8bcc3: e8 68 69 ad ff              	callq	0x140562630 <.text+0x561630>
140a8bcc8: e9 93 fe ff ff              	jmp	0x140a8bb60 <.text+0xa8ab60>
140a8bccd: 48 8b 85 10 0d 00 00        	movq	0xd10(%rbp), %rax
140a8bcd4: 48 8b 30                    	movq	(%rax), %rsi
140a8bcd7: 48 8b 85 00 0d 00 00        	movq	0xd00(%rbp), %rax
140a8bcde: 48 8b 00                    	movq	(%rax), %rax
140a8bce1: 48 89 85 e0 0c 00 00        	movq	%rax, 0xce0(%rbp)
140a8bce8: 48 8b 95 98 0e 00 00        	movq	0xe98(%rbp), %rdx
140a8bcef: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140a8bcf2: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140a8bcf6: 89 01                       	movl	%eax, (%rcx)
140a8bcf8: 4c 8b 32                    	movq	(%rdx), %r14
140a8bcfb: 48 89 b5 40 08 00 00        	movq	%rsi, 0x840(%rbp)
140a8bd02: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8bd09: 48 89 85 48 08 00 00        	movq	%rax, 0x848(%rbp)
140a8bd10: 48 8b 85 f8 0c 00 00        	movq	0xcf8(%rbp), %rax
140a8bd17: 48 89 85 50 08 00 00        	movq	%rax, 0x850(%rbp)
140a8bd1e: 4c 8d bd 40 08 00 00        	leaq	0x840(%rbp), %r15
140a8bd25: 4c 89 f1                    	movq	%r14, %rcx
140a8bd28: 4c 89 fa                    	movq	%r15, %rdx
140a8bd2b: e8 f0 cd 9c 00              	callq	0x141458b20 <.text+0x1457b20>
140a8bd30: 31 c9                       	xorl	%ecx, %ecx
140a8bd32: e8 e9 dd 64 ff              	callq	0x1400d9b20 <.text+0xd8b20>
140a8bd37: 48 85 c0                    	testq	%rax, %rax
140a8bd3a: 0f 84 8d 23 00 00           	je	0x140a8e0cd <.text+0xa8d0cd>
140a8bd40: 0f 10 00                    	movups	(%rax), %xmm0
140a8bd43: 48 ff 00                    	incq	(%rax)
140a8bd46: 0f 29 85 60 08 00 00        	movaps	%xmm0, 0x860(%rbp)
140a8bd4d: 0f 10 05 ac 01 1e 02        	movups	0x21e01ac(%rip), %xmm0  # 0x142c6bf00
140a8bd54: 0f 29 85 40 08 00 00        	movaps	%xmm0, 0x840(%rbp)
140a8bd5b: f3 0f 6f 05 ad 01 1e 02     	movdqu	0x21e01ad(%rip), %xmm0  # 0x142c6bf10
140a8bd63: 66 0f 7f 85 50 08 00 00     	movdqa	%xmm0, 0x850(%rbp)
140a8bd6b: 49 83 c6 08                 	addq	$0x8, %r14
140a8bd6f: 48 89 b5 60 02 00 00        	movq	%rsi, 0x260(%rbp)
140a8bd76: 48 8b b5 48 0d 00 00        	movq	0xd48(%rbp), %rsi
140a8bd7d: 48 89 b5 68 02 00 00        	movq	%rsi, 0x268(%rbp)
140a8bd84: 48 8b 9d f8 0c 00 00        	movq	0xcf8(%rbp), %rbx
140a8bd8b: 48 89 9d 70 02 00 00        	movq	%rbx, 0x270(%rbp)
140a8bd92: 4c 89 bd 78 02 00 00        	movq	%r15, 0x278(%rbp)
140a8bd99: c6 85 55 0d 00 00 01        	movb	$0x1, 0xd55(%rbp)
140a8bda0: 48 8d 95 60 02 00 00        	leaq	0x260(%rbp), %rdx
140a8bda7: 4c 89 f1                    	movq	%r14, %rcx
140a8bdaa: e8 a1 cc 9c 00              	callq	0x141458a50 <.text+0x1457a50>
140a8bdaf: 48 8b bd 40 08 00 00        	movq	0x840(%rbp), %rdi
140a8bdb6: 48 8b 8d 48 08 00 00        	movq	0x848(%rbp), %rcx
140a8bdbd: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
140a8bdc1: 48 85 c9                    	testq	%rcx, %rcx
140a8bdc4: 48 89 8d 30 0d 00 00        	movq	%rcx, 0xd30(%rbp)
140a8bdcb: 0f 84 4d 01 00 00           	je	0x140a8bf1e <.text+0xa8af1e>
140a8bdd1: 48 8d 04 cd 17 00 00 00     	leaq	0x17(,%rcx,8), %rax
140a8bdd9: 48 83 e0 f0                 	andq	$-0x10, %rax
140a8bddd: 48 01 c1                    	addq	%rax, %rcx
140a8bde0: 48 83 c1 11                 	addq	$0x11, %rcx
140a8bde4: 48 89 8d 20 0d 00 00        	movq	%rcx, 0xd20(%rbp)
140a8bdeb: 48 89 f9                    	movq	%rdi, %rcx
140a8bdee: 48 29 c1                    	subq	%rax, %rcx
140a8bdf1: 48 89 8d 00 0d 00 00        	movq	%rcx, 0xd00(%rbp)
140a8bdf8: b8 10 00 00 00              	movl	$0x10, %eax
140a8bdfd: 48 89 85 10 0d 00 00        	movq	%rax, 0xd10(%rbp)
140a8be04: e9 20 01 00 00              	jmp	0x140a8bf29 <.text+0xa8af29>
140a8be09: 48 89 08                    	movq	%rcx, (%rax)
140a8be0c: 48 8b 33                    	movq	(%rbx), %rsi
140a8be0f: 48 8b 53 18                 	movq	0x18(%rbx), %rdx
140a8be13: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
140a8be17: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a8be1b: f7 d0                       	notl	%eax
140a8be1d: 48 8d 5e 10                 	leaq	0x10(%rsi), %rbx
140a8be21: 41 8b 4f 1c                 	movl	0x1c(%r15), %ecx
140a8be25: 89 8d e0 0c 00 00           	movl	%ecx, 0xce0(%rbp)
140a8be2b: 4d 8b 37                    	movq	(%r15), %r14
140a8be2e: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
140a8be32: 48 89 8d e8 0c 00 00        	movq	%rcx, 0xce8(%rbp)
140a8be39: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
140a8be3d: 48 89 8d 30 0d 00 00        	movq	%rcx, 0xd30(%rbp)
140a8be44: eb 33                       	jmp	0x140a8be79 <.text+0xa8ae79>
140a8be46: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a8be50: 49 8b 46 28                 	movq	0x28(%r14), %rax
140a8be54: 4c 89 f9                    	movq	%r15, %rcx
140a8be57: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a8be5b: 48 89 3c 08                 	movq	%rdi, (%rax,%rcx)
140a8be5f: 4c 89 64 08 08              	movq	%r12, 0x8(%rax,%rcx)
140a8be64: 49 ff c7                    	incq	%r15
140a8be67: 4d 89 7e 30                 	movq	%r15, 0x30(%r14)
140a8be6b: 49 ff 46 40                 	incq	0x40(%r14)
140a8be6f: 44 89 e8                    	movl	%r13d, %eax
140a8be72: 48 8b 95 c0 0c 00 00        	movq	0xcc0(%rbp), %rdx
140a8be79: 48 85 d2                    	testq	%rdx, %rdx
140a8be7c: 0f 84 2b fa ff ff           	je	0x140a8b8ad <.text+0xa8a8ad>
140a8be82: 66 85 c0                    	testw	%ax, %ax
140a8be85: 74 09                       	je	0x140a8be90 <.text+0xa8ae90>
140a8be87: 44 8d 68 ff                 	leal	-0x1(%rax), %r13d
140a8be8b: 41 21 c5                    	andl	%eax, %r13d
140a8be8e: eb 29                       	jmp	0x140a8beb9 <.text+0xa8aeb9>
140a8be90: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
140a8be94: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a8be98: 48 81 c6 80 d5 ff ff        	addq	$-0x2a80, %rsi          # imm = 0xD580
140a8be9f: 48 83 c3 10                 	addq	$0x10, %rbx
140a8bea3: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140a8bea8: 74 e6                       	je	0x140a8be90 <.text+0xa8ae90>
140a8beaa: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140a8beaf: 29 c1                       	subl	%eax, %ecx
140a8beb1: f7 d0                       	notl	%eax
140a8beb3: 41 89 c5                    	movl	%eax, %r13d
140a8beb6: 41 21 cd                    	andl	%ecx, %r13d
140a8beb9: 48 ff ca                    	decq	%rdx
140a8bebc: f3 0f bc c0                 	tzcntl	%eax, %eax
140a8bec0: 48 f7 d8                    	negq	%rax
140a8bec3: 48 69 c8 a8 02 00 00        	imulq	$0x2a8, %rax, %rcx      # imm = 0x2A8
140a8beca: 80 7c 0e f9 01              	cmpb	$0x1, -0x7(%rsi,%rcx)
140a8becf: 44 89 e8                    	movl	%r13d, %eax
140a8bed2: 76 a5                       	jbe	0x140a8be79 <.text+0xa8ae79>
140a8bed4: 48 89 95 c0 0c 00 00        	movq	%rdx, 0xcc0(%rbp)
140a8bedb: 48 01 f1                    	addq	%rsi, %rcx
140a8bede: 4c 8b a1 58 fd ff ff        	movq	-0x2a8(%rcx), %r12
140a8bee5: 8b 85 e0 0c 00 00           	movl	0xce0(%rbp), %eax
140a8beeb: 48 8b 8d e8 0c 00 00        	movq	0xce8(%rbp), %rcx
140a8bef2: 89 01                       	movl	%eax, (%rcx)
140a8bef4: 4d 8b 7e 30                 	movq	0x30(%r14), %r15
140a8bef8: 49 8b 7e 40                 	movq	0x40(%r14), %rdi
140a8befc: 4d 3b 7e 20                 	cmpq	0x20(%r14), %r15
140a8bf00: 0f 85 4a ff ff ff           	jne	0x140a8be50 <.text+0xa8ae50>
140a8bf06: 48 8b 8d 30 0d 00 00        	movq	0xd30(%rbp), %rcx
140a8bf0d: 48 8d 15 34 e2 1a 02        	leaq	0x21ae234(%rip), %rdx   # 0x142c3a148
140a8bf14: e8 17 67 ad ff              	callq	0x140562630 <.text+0x561630>
140a8bf19: e9 32 ff ff ff              	jmp	0x140a8be50 <.text+0xa8ae50>
140a8bf1e: 48 c7 85 10 0d 00 00 00 00 00 00    	movq	$0x0, 0xd10(%rbp)
140a8bf29: 4c 8b b5 58 08 00 00        	movq	0x858(%rbp), %r14
140a8bf30: 4c 8d 7f 10                 	leaq	0x10(%rdi), %r15
140a8bf34: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a8bf38: f7 d0                       	notl	%eax
140a8bf3a: 48 8d 4e 60                 	leaq	0x60(%rsi), %rcx
140a8bf3e: 48 89 8d f0 0c 00 00        	movq	%rcx, 0xcf0(%rbp)
140a8bf45: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140a8bf49: 48 89 8d e8 0c 00 00        	movq	%rcx, 0xce8(%rbp)
140a8bf50: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140a8bf54: 0f 57 ff                    	xorps	%xmm7, %xmm7
140a8bf57: e9 f4 01 00 00              	jmp	0x140a8c150 <.text+0xa8b150>
140a8bf5c: 0f 1f 40 00                 	nopl	(%rax)
140a8bf60: f3 0f 6f 0c 16              	movdqu	(%rsi,%rdx), %xmm1
140a8bf65: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a8bf69: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a8bf6d: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140a8bf72: 45 85 c9                    	testl	%r9d, %r9d
140a8bf75: 74 29                       	je	0x140a8bfa0 <.text+0xa8afa0>
140a8bf77: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140a8bf7c: 49 01 d2                    	addq	%rdx, %r10
140a8bf7f: 49 21 ca                    	andq	%rcx, %r10
140a8bf82: 49 f7 da                    	negq	%r10
140a8bf85: 4d 69 e2 a8 02 00 00        	imulq	$0x2a8, %r10, %r12      # imm = 0x2A8
140a8bf8c: 4e 39 2c 20                 	cmpq	%r13, (%rax,%r12)
140a8bf90: 74 3e                       	je	0x140a8bfd0 <.text+0xa8afd0>
140a8bf92: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140a8bf96: 66 45 21 ca                 	andw	%r9w, %r10w
140a8bf9a: 45 89 d1                    	movl	%r10d, %r9d
140a8bf9d: 75 d8                       	jne	0x140a8bf77 <.text+0xa8af77>
140a8bf9f: 90                          	nop
140a8bfa0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140a8bfa4: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
140a8bfa9: 45 85 c9                    	testl	%r9d, %r9d
140a8bfac: 0f 85 5e 02 00 00           	jne	0x140a8c210 <.text+0xa8b210>
140a8bfb2: 4c 01 c2                    	addq	%r8, %rdx
140a8bfb5: 48 83 c2 10                 	addq	$0x10, %rdx
140a8bfb9: 49 83 c0 10                 	addq	$0x10, %r8
140a8bfbd: 48 21 ca                    	andq	%rcx, %rdx
140a8bfc0: eb 9e                       	jmp	0x140a8bf60 <.text+0xa8af60>
140a8bfc2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a8bfd0: 4a 83 bc 26 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rsi,%r12)
140a8bfd9: 0f 82 7a 1f 00 00           	jb	0x140a8df59 <.text+0xa8cf59>
140a8bfdf: f3 42 0f 10 84 26 90 fd ff ff       	movss	-0x270(%rsi,%r12), %xmm0
140a8bfe9: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
140a8bfec: 0f 86 67 1f 00 00           	jbe	0x140a8df59 <.text+0xa8cf59>
140a8bff2: 4c 89 ad 60 02 00 00        	movq	%r13, 0x260(%rbp)
140a8bff9: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8c000: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
140a8c005: 0f 84 0c 02 00 00           	je	0x140a8c217 <.text+0xa8b217>
140a8c00b: 48 8b 8d c0 0c 00 00        	movq	0xcc0(%rbp), %rcx
140a8c012: 48 8d 95 60 02 00 00        	leaq	0x260(%rbp), %rdx
140a8c019: e8 52 4d b2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a8c01e: 4c 01 e6                    	addq	%r12, %rsi
140a8c021: 48 8b 95 48 0d 00 00        	movq	0xd48(%rbp), %rdx
140a8c028: 48 8b 0a                    	movq	(%rdx), %rcx
140a8c02b: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140a8c02f: 49 89 d0                    	movq	%rdx, %r8
140a8c032: 49 21 c0                    	andq	%rax, %r8
140a8c035: 48 c1 e8 39                 	shrq	$0x39, %rax
140a8c039: 66 0f 6e c0                 	movd	%eax, %xmm0
140a8c03d: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8c041: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8c046: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8c04b: 48 8d 41 88                 	leaq	-0x78(%rcx), %rax
140a8c04f: 45 31 c9                    	xorl	%r9d, %r9d
140a8c052: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140a8c058: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a8c05c: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a8c060: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140a8c065: 45 85 d2                    	testl	%r10d, %r10d
140a8c068: 74 26                       	je	0x140a8c090 <.text+0xa8b090>
140a8c06a: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140a8c06f: 4d 01 c3                    	addq	%r8, %r11
140a8c072: 49 21 d3                    	andq	%rdx, %r11
140a8c075: 49 f7 db                    	negq	%r11
140a8c078: 4d 6b db 78                 	imulq	$0x78, %r11, %r11
140a8c07c: 4e 39 2c 18                 	cmpq	%r13, (%rax,%r11)
140a8c080: 74 3e                       	je	0x140a8c0c0 <.text+0xa8b0c0>
140a8c082: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140a8c086: 66 45 21 d3                 	andw	%r10w, %r11w
140a8c08a: 45 89 da                    	movl	%r11d, %r10d
140a8c08d: 75 db                       	jne	0x140a8c06a <.text+0xa8b06a>
140a8c08f: 90                          	nop
140a8c090: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140a8c094: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140a8c099: 45 85 d2                    	testl	%r10d, %r10d
140a8c09c: 0f 85 75 01 00 00           	jne	0x140a8c217 <.text+0xa8b217>
140a8c0a2: 4d 01 c8                    	addq	%r9, %r8
140a8c0a5: 49 83 c0 10                 	addq	$0x10, %r8
140a8c0a9: 49 83 c1 10                 	addq	$0x10, %r9
140a8c0ad: 49 21 d0                    	andq	%rdx, %r8
140a8c0b0: eb a0                       	jmp	0x140a8c052 <.text+0xa8b052>
140a8c0b2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a8c0c0: f3 46 0f 10 44 19 e8        	movss	-0x18(%rcx,%r11), %xmm8
140a8c0c7: 4c 8b a6 a0 fd ff ff        	movq	-0x260(%rsi), %r12
140a8c0ce: 48 8b 86 a8 fd ff ff        	movq	-0x258(%rsi), %rax
140a8c0d5: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140a8c0d9: 49 8d 34 84                 	leaq	(%r12,%rax,4), %rsi
140a8c0dd: eb 06                       	jmp	0x140a8c0e5 <.text+0xa8b0e5>
140a8c0df: 90                          	nop
140a8c0e0: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140a8c0e3: 77 4d                       	ja	0x140a8c132 <.text+0xa8b132>
140a8c0e5: 49 39 f4                    	cmpq	%rsi, %r12
140a8c0e8: 74 44                       	je	0x140a8c12e <.text+0xa8b12e>
140a8c0ea: 66 41 0f 6e 0c 24           	movd	(%r12), %xmm1
140a8c0f0: 66 41 0f 6e 54 24 08        	movd	0x8(%r12), %xmm2
140a8c0f7: 48 8b 8d e0 0c 00 00        	movq	0xce0(%rbp), %rcx
140a8c0fe: e8 3d d5 f5 ff              	callq	0x1409e9640 <.text+0x9e8640>
140a8c103: f3 41 0f 10 4c 24 04        	movss	0x4(%r12), %xmm1
140a8c10a: 49 83 c4 0c                 	addq	$0xc, %r12
140a8c10e: 41 0f 2e c8                 	ucomiss	%xmm8, %xmm1
140a8c112: 76 cc                       	jbe	0x140a8c0e0 <.text+0xa8b0e0>
140a8c114: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140a8c117: 75 02                       	jne	0x140a8c11b <.text+0xa8b11b>
140a8c119: 7b ca                       	jnp	0x140a8c0e5 <.text+0xa8b0e5>
140a8c11b: 48 8b 8d f0 0c 00 00        	movq	0xcf0(%rbp), %rcx
140a8c122: 4c 89 ea                    	movq	%r13, %rdx
140a8c125: e8 c6 e1 83 ff              	callq	0x1402ca2f0 <.text+0x2c92f0>
140a8c12a: 89 d8                       	movl	%ebx, %eax
140a8c12c: eb 22                       	jmp	0x140a8c150 <.text+0xa8b150>
140a8c12e: 89 d8                       	movl	%ebx, %eax
140a8c130: eb 1e                       	jmp	0x140a8c150 <.text+0xa8b150>
140a8c132: 48 8b 8d f0 0c 00 00        	movq	0xcf0(%rbp), %rcx
140a8c139: 4c 89 ea                    	movq	%r13, %rdx
140a8c13c: e8 af e1 83 ff              	callq	0x1402ca2f0 <.text+0x2c92f0>
140a8c141: 89 d8                       	movl	%ebx, %eax
140a8c143: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8c150: 4d 85 f6                    	testq	%r14, %r14
140a8c153: 0f 84 cf 00 00 00           	je	0x140a8c228 <.text+0xa8b228>
140a8c159: 66 85 c0                    	testw	%ax, %ax
140a8c15c: 74 12                       	je	0x140a8c170 <.text+0xa8b170>
140a8c15e: 8d 58 ff                    	leal	-0x1(%rax), %ebx
140a8c161: 21 c3                       	andl	%eax, %ebx
140a8c163: eb 30                       	jmp	0x140a8c195 <.text+0xa8b195>
140a8c165: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a8c170: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
140a8c175: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a8c179: 48 83 c7 80                 	addq	$-0x80, %rdi
140a8c17d: 49 83 c7 10                 	addq	$0x10, %r15
140a8c181: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140a8c186: 74 e8                       	je	0x140a8c170 <.text+0xa8b170>
140a8c188: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140a8c18d: 29 c1                       	subl	%eax, %ecx
140a8c18f: f7 d0                       	notl	%eax
140a8c191: 89 c3                       	movl	%eax, %ebx
140a8c193: 21 cb                       	andl	%ecx, %ebx
140a8c195: 49 ff ce                    	decq	%r14
140a8c198: f3 0f bc c0                 	tzcntl	%eax, %eax
140a8c19c: c1 e0 03                    	shll	$0x3, %eax
140a8c19f: 48 89 f9                    	movq	%rdi, %rcx
140a8c1a2: 48 29 c1                    	subq	%rax, %rcx
140a8c1a5: 4c 8b 69 f8                 	movq	-0x8(%rcx), %r13
140a8c1a9: 4c 89 ad 60 02 00 00        	movq	%r13, 0x260(%rbp)
140a8c1b0: 48 8b 85 f8 0c 00 00        	movq	0xcf8(%rbp), %rax
140a8c1b7: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
140a8c1bc: 74 52                       	je	0x140a8c210 <.text+0xa8b210>
140a8c1be: 48 8b 8d e8 0c 00 00        	movq	0xce8(%rbp), %rcx
140a8c1c5: 48 8d 95 60 02 00 00        	leaq	0x260(%rbp), %rdx
140a8c1cc: e8 9f 4b b2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a8c1d1: 48 8b 8d f8 0c 00 00        	movq	0xcf8(%rbp), %rcx
140a8c1d8: 48 8b 31                    	movq	(%rcx), %rsi
140a8c1db: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
140a8c1df: 48 89 ca                    	movq	%rcx, %rdx
140a8c1e2: 48 21 c2                    	andq	%rax, %rdx
140a8c1e5: 48 c1 e8 39                 	shrq	$0x39, %rax
140a8c1e9: 66 0f 6e c0                 	movd	%eax, %xmm0
140a8c1ed: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8c1f1: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8c1f6: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8c1fb: 48 8d 86 58 fd ff ff        	leaq	-0x2a8(%rsi), %rax
140a8c202: 45 31 c0                    	xorl	%r8d, %r8d
140a8c205: e9 56 fd ff ff              	jmp	0x140a8bf60 <.text+0xa8af60>
140a8c20a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140a8c210: 89 d8                       	movl	%ebx, %eax
140a8c212: e9 39 ff ff ff              	jmp	0x140a8c150 <.text+0xa8b150>
140a8c217: 48 8d 0d 02 d1 04 02        	leaq	0x204d102(%rip), %rcx   # 0x142ad9320
140a8c21e: e8 ed d0 e4 01              	callq	0x1428d9310 <.text+0x28d8310>
140a8c223: e9 df 1f 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8c228: 48 83 bd 30 0d 00 00 00     	cmpq	$0x0, 0xd30(%rbp)
140a8c230: 48 8b b5 48 0d 00 00        	movq	0xd48(%rbp), %rsi
140a8c237: 48 8b 95 20 0d 00 00        	movq	0xd20(%rbp), %rdx
140a8c23e: 48 8b 8d 00 0d 00 00        	movq	0xd00(%rbp), %rcx
140a8c245: 74 11                       	je	0x140a8c258 <.text+0xa8b258>
140a8c247: 48 85 d2                    	testq	%rdx, %rdx
140a8c24a: 74 0c                       	je	0x140a8c258 <.text+0xa8b258>
140a8c24c: 4c 8b 85 10 0d 00 00        	movq	0xd10(%rbp), %r8
140a8c253: e8 c8 79 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8c258: 48 8d 86 90 00 00 00        	leaq	0x90(%rsi), %rax
140a8c25f: 48 89 85 80 0b 00 00        	movq	%rax, 0xb80(%rbp)
140a8c266: 48 89 85 40 08 00 00        	movq	%rax, 0x840(%rbp)
140a8c26d: 48 8d 95 40 08 00 00        	leaq	0x840(%rbp), %rdx
140a8c274: 48 8b 8d f0 0c 00 00        	movq	0xcf0(%rbp), %rcx
140a8c27b: e8 00 13 aa 00              	callq	0x14152d580 <.text+0x152c580>
140a8c280: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8c287: 48 8b b0 90 00 00 00        	movq	0x90(%rax), %rsi
140a8c28e: 48 8b b8 a8 00 00 00        	movq	0xa8(%rax), %rdi
140a8c295: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
140a8c299: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
140a8c29e: 41 f7 d4                    	notl	%r12d
140a8c2a1: 48 8d 5e 10                 	leaq	0x10(%rsi), %rbx
140a8c2a5: 48 8b 85 70 0e 00 00        	movq	0xe70(%rbp), %rax
140a8c2ac: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140a8c2af: 89 8d 20 0d 00 00           	movl	%ecx, 0xd20(%rbp)
140a8c2b5: 48 8b 08                    	movq	(%rax), %rcx
140a8c2b8: 48 8b 40 10                 	movq	0x10(%rax), %rax
140a8c2bc: 48 89 85 10 0d 00 00        	movq	%rax, 0xd10(%rbp)
140a8c2c3: 48 89 8d 30 0d 00 00        	movq	%rcx, 0xd30(%rbp)
140a8c2ca: 48 8d 41 20                 	leaq	0x20(%rcx), %rax
140a8c2ce: 48 89 85 00 0d 00 00        	movq	%rax, 0xd00(%rbp)
140a8c2d5: eb 2f                       	jmp	0x140a8c306 <.text+0xa8b306>
140a8c2d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a8c2e0: 48 8b 95 30 0d 00 00        	movq	0xd30(%rbp), %rdx
140a8c2e7: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140a8c2eb: 4c 89 f9                    	movq	%r15, %rcx
140a8c2ee: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a8c2f2: 4c 89 2c 08                 	movq	%r13, (%rax,%rcx)
140a8c2f6: 4c 89 74 08 08              	movq	%r14, 0x8(%rax,%rcx)
140a8c2fb: 49 ff c7                    	incq	%r15
140a8c2fe: 4c 89 7a 30                 	movq	%r15, 0x30(%rdx)
140a8c302: 48 ff 42 40                 	incq	0x40(%rdx)
140a8c306: 48 85 ff                    	testq	%rdi, %rdi
140a8c309: 0f 84 0e 01 00 00           	je	0x140a8c41d <.text+0xa8b41d>
140a8c30f: 66 45 85 e4                 	testw	%r12w, %r12w
140a8c313: 75 28                       	jne	0x140a8c33d <.text+0xa8b33d>
140a8c315: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a8c320: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
140a8c324: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
140a8c329: 48 83 c6 80                 	addq	$-0x80, %rsi
140a8c32d: 48 83 c3 10                 	addq	$0x10, %rbx
140a8c331: 41 81 fc ff ff 00 00        	cmpl	$0xffff, %r12d          # imm = 0xFFFF
140a8c338: 74 e6                       	je	0x140a8c320 <.text+0xa8b320>
140a8c33a: 41 f7 d4                    	notl	%r12d
140a8c33d: f3 41 0f bc c4              	tzcntl	%r12d, %eax
140a8c342: c1 e0 03                    	shll	$0x3, %eax
140a8c345: 48 89 f1                    	movq	%rsi, %rcx
140a8c348: 48 29 c1                    	subq	%rax, %rcx
140a8c34b: 48 8b 41 f8                 	movq	-0x8(%rcx), %rax
140a8c34f: 48 89 85 70 0c 00 00        	movq	%rax, 0xc70(%rbp)
140a8c356: 48 8b 8d e8 0c 00 00        	movq	0xce8(%rbp), %rcx
140a8c35d: 48 8d 95 70 0c 00 00        	leaq	0xc70(%rbp), %rdx
140a8c364: e8 07 4a b2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a8c369: 48 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %rcx
140a8c370: 48 8b 95 f8 0c 00 00        	movq	0xcf8(%rbp), %rdx
140a8c377: 49 89 c0                    	movq	%rax, %r8
140a8c37a: 4c 8d 8d 70 0c 00 00        	leaq	0xc70(%rbp), %r9
140a8c381: e8 2a af 0e 00              	callq	0x140b772b0 <.text+0xb762b0>
140a8c386: 41 8d 44 24 ff              	leal	-0x1(%r12), %eax
140a8c38b: 44 21 e0                    	andl	%r12d, %eax
140a8c38e: 41 89 c4                    	movl	%eax, %r12d
140a8c391: 48 ff cf                    	decq	%rdi
140a8c394: 45 31 f6                    	xorl	%r14d, %r14d
140a8c397: 4c 3b b5 48 08 00 00        	cmpq	0x848(%rbp), %r14
140a8c39e: 0f 80 62 ff ff ff           	jo	0x140a8c306 <.text+0xa8b306>
140a8c3a4: 41 b8 a0 02 00 00           	movl	$0x2a0, %r8d            # imm = 0x2A0
140a8c3aa: 48 8d 8d 60 02 00 00        	leaq	0x260(%rbp), %rcx
140a8c3b1: 48 8d 95 48 08 00 00        	leaq	0x848(%rbp), %rdx
140a8c3b8: e8 43 78 e9 01              	callq	0x142923c00 <.text+0x2922c00>
140a8c3bd: 4c 3b b5 60 02 00 00        	cmpq	0x260(%rbp), %r14
140a8c3c4: 0f 80 3c ff ff ff           	jo	0x140a8c306 <.text+0xa8b306>
140a8c3ca: 48 8d 8d 60 02 00 00        	leaq	0x260(%rbp), %rcx
140a8c3d1: e8 2a 92 e0 ff              	callq	0x140895600 <.text+0x894600>
140a8c3d6: 4c 8b b5 70 0c 00 00        	movq	0xc70(%rbp), %r14
140a8c3dd: 8b 85 20 0d 00 00           	movl	0xd20(%rbp), %eax
140a8c3e3: 48 8b 8d 10 0d 00 00        	movq	0xd10(%rbp), %rcx
140a8c3ea: 89 01                       	movl	%eax, (%rcx)
140a8c3ec: 48 8b 85 30 0d 00 00        	movq	0xd30(%rbp), %rax
140a8c3f3: 4c 8b 78 30                 	movq	0x30(%rax), %r15
140a8c3f7: 4c 8b 68 40                 	movq	0x40(%rax), %r13
140a8c3fb: 4c 3b 78 20                 	cmpq	0x20(%rax), %r15
140a8c3ff: 0f 85 db fe ff ff           	jne	0x140a8c2e0 <.text+0xa8b2e0>
140a8c405: 48 8b 8d 00 0d 00 00        	movq	0xd00(%rbp), %rcx
140a8c40c: 48 8d 15 35 dd 1a 02        	leaq	0x21add35(%rip), %rdx   # 0x142c3a148
140a8c413: e8 18 62 ad ff              	callq	0x140562630 <.text+0x561630>
140a8c418: e9 c3 fe ff ff              	jmp	0x140a8c2e0 <.text+0xa8b2e0>
140a8c41d: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8c424: 4c 8b 40 30                 	movq	0x30(%rax), %r8
140a8c428: 48 8b 48 48                 	movq	0x48(%rax), %rcx
140a8c42c: 48 89 8d 30 0d 00 00        	movq	%rcx, 0xd30(%rbp)
140a8c433: 66 41 0f 6f 00              	movdqa	(%r8), %xmm0
140a8c438: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
140a8c43c: f7 d1                       	notl	%ecx
140a8c43e: 48 89 8d 00 0d 00 00        	movq	%rcx, 0xd00(%rbp)
140a8c445: 4d 8d 68 10                 	leaq	0x10(%r8), %r13
140a8c449: 4c 8b 60 60                 	movq	0x60(%rax), %r12
140a8c44d: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
140a8c453: 66 44 0f d7 f8              	pmovmskb	%xmm0, %r15d
140a8c458: 41 f7 d7                    	notl	%r15d
140a8c45b: 48 8b 8d a0 0e 00 00        	movq	0xea0(%rbp), %rcx
140a8c462: 48 8b 09                    	movq	(%rcx), %rcx
140a8c465: 48 89 8d 18 0c 00 00        	movq	%rcx, 0xc18(%rbp)
140a8c46c: 48 8b 8d b8 0e 00 00        	movq	0xeb8(%rbp), %rcx
140a8c473: 48 8b 09                    	movq	(%rcx), %rcx
140a8c476: 48 89 8d 60 0b 00 00        	movq	%rcx, 0xb60(%rbp)
140a8c47d: 48 8b 8d b0 0e 00 00        	movq	0xeb0(%rbp), %rcx
140a8c484: 48 8b 09                    	movq	(%rcx), %rcx
140a8c487: 48 89 8d 68 0b 00 00        	movq	%rcx, 0xb68(%rbp)
140a8c48e: 48 8b 8d a8 0e 00 00        	movq	0xea8(%rbp), %rcx
140a8c495: 48 8b 09                    	movq	(%rcx), %rcx
140a8c498: 48 89 8d 78 0b 00 00        	movq	%rcx, 0xb78(%rbp)
140a8c49f: 48 8b 95 78 0e 00 00        	movq	0xe78(%rbp), %rdx
140a8c4a6: 8b 4a 1c                    	movl	0x1c(%rdx), %ecx
140a8c4a9: 89 8d 60 0c 00 00           	movl	%ecx, 0xc60(%rbp)
140a8c4af: 48 8b 0a                    	movq	(%rdx), %rcx
140a8c4b2: 48 8b 52 10                 	movq	0x10(%rdx), %rdx
140a8c4b6: 48 89 95 58 0b 00 00        	movq	%rdx, 0xb58(%rbp)
140a8c4bd: 66 0f 76 ff                 	pcmpeqd	%xmm7, %xmm7
140a8c4c1: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
140a8c4c5: f3 0f 10 35 a3 93 e9 01     	movss	0x1e993a3(%rip), %xmm6  # 0x142925870
140a8c4cd: f3 44 0f 10 15 2a 81 ea 01  	movss	0x1ea812a(%rip), %xmm10 # 0x142934600
140a8c4d6: f3 44 0f 10 1d 8d 02 ea 01  	movss	0x1ea028d(%rip), %xmm11 # 0x14292c76c
140a8c4df: f3 44 0f 10 3d f0 93 e9 01  	movss	0x1e993f0(%rip), %xmm15 # 0x1429258d8
140a8c4e8: f3 44 0f 10 35 73 0b f4 01  	movss	0x1f40b73(%rip), %xmm14 # 0x1429cd064
140a8c4f1: 4d 8d 74 24 10              	leaq	0x10(%r12), %r14
140a8c4f6: 48 8b 40 78                 	movq	0x78(%rax), %rax
140a8c4fa: 48 89 85 10 0d 00 00        	movq	%rax, 0xd10(%rbp)
140a8c501: 48 89 8d 10 0c 00 00        	movq	%rcx, 0xc10(%rbp)
140a8c508: 48 8d 41 20                 	leaq	0x20(%rcx), %rax
140a8c50c: 48 89 85 70 0b 00 00        	movq	%rax, 0xb70(%rbp)
140a8c513: 4d 85 c0                    	testq	%r8, %r8
140a8c516: 0f 84 7c 00 00 00           	je	0x140a8c598 <.text+0xa8b598>
140a8c51c: 48 83 bd 30 0d 00 00 00     	cmpq	$0x0, 0xd30(%rbp)
140a8c524: 74 67                       	je	0x140a8c58d <.text+0xa8b58d>
140a8c526: 48 8b 8d 00 0d 00 00        	movq	0xd00(%rbp), %rcx
140a8c52d: 66 85 c9                    	testw	%cx, %cx
140a8c530: 74 0e                       	je	0x140a8c540 <.text+0xa8b540>
140a8c532: 8d 51 ff                    	leal	-0x1(%rcx), %edx
140a8c535: 21 ca                       	andl	%ecx, %edx
140a8c537: eb 2e                       	jmp	0x140a8c567 <.text+0xa8b567>
140a8c539: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a8c540: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
140a8c546: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
140a8c54a: 49 83 c0 80                 	addq	$-0x80, %r8
140a8c54e: 49 83 c5 10                 	addq	$0x10, %r13
140a8c552: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
140a8c558: 74 e6                       	je	0x140a8c540 <.text+0xa8b540>
140a8c55a: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140a8c55f: 29 c8                       	subl	%ecx, %eax
140a8c561: f7 d1                       	notl	%ecx
140a8c563: 89 ca                       	movl	%ecx, %edx
140a8c565: 21 c2                       	andl	%eax, %edx
140a8c567: 48 ff 8d 30 0d 00 00        	decq	0xd30(%rbp)
140a8c56e: b0 01                       	movb	$0x1, %al
140a8c570: 48 89 85 38 0d 00 00        	movq	%rax, 0xd38(%rbp)
140a8c577: 4c 89 c0                    	movq	%r8, %rax
140a8c57a: 4c 89 85 20 0d 00 00        	movq	%r8, 0xd20(%rbp)
140a8c581: 48 89 95 00 0d 00 00        	movq	%rdx, 0xd00(%rbp)
140a8c588: e9 94 00 00 00              	jmp	0x140a8c621 <.text+0xa8b621>
140a8c58d: 48 c7 85 30 0d 00 00 00 00 00 00    	movq	$0x0, 0xd30(%rbp)
140a8c598: 48 8b bd 48 0d 00 00        	movq	0xd48(%rbp), %rdi
140a8c59f: 4d 85 e4                    	testq	%r12, %r12
140a8c5a2: 0f 84 13 18 00 00           	je	0x140a8ddbb <.text+0xa8cdbb>
140a8c5a8: 48 83 bd 10 0d 00 00 00     	cmpq	$0x0, 0xd10(%rbp)
140a8c5b0: 0f 84 05 18 00 00           	je	0x140a8ddbb <.text+0xa8cdbb>
140a8c5b6: 66 45 85 ff                 	testw	%r15w, %r15w
140a8c5ba: 74 14                       	je	0x140a8c5d0 <.text+0xa8b5d0>
140a8c5bc: 41 8d 57 ff                 	leal	-0x1(%r15), %edx
140a8c5c0: 44 21 fa                    	andl	%r15d, %edx
140a8c5c3: eb 36                       	jmp	0x140a8c5fb <.text+0xa8b5fb>
140a8c5c5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a8c5d0: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
140a8c5d5: 66 44 0f d7 f8              	pmovmskb	%xmm0, %r15d
140a8c5da: 49 83 c4 80                 	addq	$-0x80, %r12
140a8c5de: 49 83 c6 10                 	addq	$0x10, %r14
140a8c5e2: 41 81 ff ff ff 00 00        	cmpl	$0xffff, %r15d          # imm = 0xFFFF
140a8c5e9: 74 e5                       	je	0x140a8c5d0 <.text+0xa8b5d0>
140a8c5eb: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140a8c5f0: 44 29 f8                    	subl	%r15d, %eax
140a8c5f3: 41 f7 d7                    	notl	%r15d
140a8c5f6: 44 89 fa                    	movl	%r15d, %edx
140a8c5f9: 21 c2                       	andl	%eax, %edx
140a8c5fb: 48 ff 8d 10 0d 00 00        	decq	0xd10(%rbp)
140a8c602: 48 c7 85 20 0d 00 00 00 00 00 00    	movq	$0x0, 0xd20(%rbp)
140a8c60d: 44 89 f9                    	movl	%r15d, %ecx
140a8c610: 4c 89 e0                    	movq	%r12, %rax
140a8c613: 41 89 d7                    	movl	%edx, %r15d
140a8c616: 48 c7 85 38 0d 00 00 00 00 00 00    	movq	$0x0, 0xd38(%rbp)
140a8c621: f3 0f bc c9                 	tzcntl	%ecx, %ecx
140a8c625: c1 e1 03                    	shll	$0x3, %ecx
140a8c628: 48 29 c8                    	subq	%rcx, %rax
140a8c62b: 48 8b 70 f8                 	movq	-0x8(%rax), %rsi
140a8c62f: 48 89 b5 b8 0c 00 00        	movq	%rsi, 0xcb8(%rbp)
140a8c636: 48 89 b5 40 08 00 00        	movq	%rsi, 0x840(%rbp)
140a8c63d: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8c644: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
140a8c649: 0f 84 37 17 00 00           	je	0x140a8dd86 <.text+0xa8cd86>
140a8c64f: 48 8b 8d c0 0c 00 00        	movq	0xcc0(%rbp), %rcx
140a8c656: 48 8d 95 40 08 00 00        	leaq	0x840(%rbp), %rdx
140a8c65d: e8 0e 47 b2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a8c662: 48 8b 8d 48 0d 00 00        	movq	0xd48(%rbp), %rcx
140a8c669: 48 8b 19                    	movq	(%rcx), %rbx
140a8c66c: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
140a8c670: 48 89 ca                    	movq	%rcx, %rdx
140a8c673: 48 21 c2                    	andq	%rax, %rdx
140a8c676: 48 c1 e8 39                 	shrq	$0x39, %rax
140a8c67a: 66 0f 6e c0                 	movd	%eax, %xmm0
140a8c67e: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8c682: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8c687: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8c68c: 48 8d 43 88                 	leaq	-0x78(%rbx), %rax
140a8c690: 45 31 c0                    	xorl	%r8d, %r8d
140a8c693: f3 0f 6f 0c 13              	movdqu	(%rbx,%rdx), %xmm1
140a8c698: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a8c69c: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a8c6a0: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140a8c6a5: 45 85 c9                    	testl	%r9d, %r9d
140a8c6a8: 74 25                       	je	0x140a8c6cf <.text+0xa8b6cf>
140a8c6aa: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140a8c6af: 49 01 d2                    	addq	%rdx, %r10
140a8c6b2: 49 21 ca                    	andq	%rcx, %r10
140a8c6b5: 49 f7 da                    	negq	%r10
140a8c6b8: 49 6b fa 78                 	imulq	$0x78, %r10, %rdi
140a8c6bc: 48 39 34 38                 	cmpq	%rsi, (%rax,%rdi)
140a8c6c0: 74 3e                       	je	0x140a8c700 <.text+0xa8b700>
140a8c6c2: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140a8c6c6: 66 45 21 ca                 	andw	%r9w, %r10w
140a8c6ca: 45 89 d1                    	movl	%r10d, %r9d
140a8c6cd: 75 db                       	jne	0x140a8c6aa <.text+0xa8b6aa>
140a8c6cf: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
140a8c6d3: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
140a8c6d8: 45 85 c9                    	testl	%r9d, %r9d
140a8c6db: 0f 85 a5 16 00 00           	jne	0x140a8dd86 <.text+0xa8cd86>
140a8c6e1: 4c 01 c2                    	addq	%r8, %rdx
140a8c6e4: 48 83 c2 10                 	addq	$0x10, %rdx
140a8c6e8: 49 83 c0 10                 	addq	$0x10, %r8
140a8c6ec: 48 21 ca                    	andq	%rcx, %rdx
140a8c6ef: eb a2                       	jmp	0x140a8c693 <.text+0xa8b693>
140a8c6f1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140a8c700: 48 83 7c 3b a0 02           	cmpq	$0x2, -0x60(%rbx,%rdi)
140a8c706: 4c 8b 85 20 0d 00 00        	movq	0xd20(%rbp), %r8
140a8c70d: 0f 82 00 fe ff ff           	jb	0x140a8c513 <.text+0xa8b513>
140a8c713: f3 0f 10 44 3b c0           	movss	-0x40(%rbx,%rdi), %xmm0
140a8c719: 41 0f 2e c0                 	ucomiss	%xmm8, %xmm0
140a8c71d: 0f 86 f0 fd ff ff           	jbe	0x140a8c513 <.text+0xa8b513>
140a8c723: 48 8b 85 b8 0c 00 00        	movq	0xcb8(%rbp), %rax
140a8c72a: 48 89 85 40 0d 00 00        	movq	%rax, 0xd40(%rbp)
140a8c731: 8b 85 60 0c 00 00           	movl	0xc60(%rbp), %eax
140a8c737: 48 8b 8d 58 0b 00 00        	movq	0xb58(%rbp), %rcx
140a8c73e: 89 01                       	movl	%eax, (%rcx)
140a8c740: 48 8b 85 10 0c 00 00        	movq	0xc10(%rbp), %rax
140a8c747: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140a8c74b: 48 8b 48 40                 	movq	0x40(%rax), %rcx
140a8c74f: 48 89 8d 18 0d 00 00        	movq	%rcx, 0xd18(%rbp)
140a8c756: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140a8c75a: 75 13                       	jne	0x140a8c76f <.text+0xa8b76f>
140a8c75c: 48 8b 8d 70 0b 00 00        	movq	0xb70(%rbp), %rcx
140a8c763: 48 8d 15 de d9 1a 02        	leaq	0x21ad9de(%rip), %rdx   # 0x142c3a148
140a8c76a: e8 c1 5e ad ff              	callq	0x140562630 <.text+0x561630>
140a8c76f: 48 8b 95 10 0c 00 00        	movq	0xc10(%rbp), %rdx
140a8c776: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140a8c77a: 48 89 f1                    	movq	%rsi, %rcx
140a8c77d: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a8c781: 4c 8b 85 18 0d 00 00        	movq	0xd18(%rbp), %r8
140a8c788: 4c 89 04 08                 	movq	%r8, (%rax,%rcx)
140a8c78c: 4c 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %r8
140a8c793: 4c 89 44 08 08              	movq	%r8, 0x8(%rax,%rcx)
140a8c798: 48 ff c6                    	incq	%rsi
140a8c79b: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
140a8c79f: 48 ff 42 40                 	incq	0x40(%rdx)
140a8c7a3: 48 8b 8d e8 0c 00 00        	movq	0xce8(%rbp), %rcx
140a8c7aa: 48 8d 95 b8 0c 00 00        	leaq	0xcb8(%rbp), %rdx
140a8c7b1: e8 ba 45 b2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a8c7b6: 48 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %rcx
140a8c7bd: 48 8b 95 f8 0c 00 00        	movq	0xcf8(%rbp), %rdx
140a8c7c4: 49 89 c0                    	movq	%rax, %r8
140a8c7c7: 4c 8d 8d b8 0c 00 00        	leaq	0xcb8(%rbp), %r9
140a8c7ce: e8 dd aa 0e 00              	callq	0x140b772b0 <.text+0xb762b0>
140a8c7d3: 31 c0                       	xorl	%eax, %eax
140a8c7d5: 48 3b 85 48 08 00 00        	cmpq	0x848(%rbp), %rax
140a8c7dc: 71 1e                       	jno	0x140a8c7fc <.text+0xa8b7fc>
140a8c7de: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a8c7e8: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140a8c7ec: 41 b2 01                    	movb	$0x1, %r10b
140a8c7ef: 48 c7 85 c8 0c 00 00 00 00 00 00    	movq	$0x0, 0xcc8(%rbp)
140a8c7fa: eb 2f                       	jmp	0x140a8c82b <.text+0xa8b82b>
140a8c7fc: 41 b8 a0 02 00 00           	movl	$0x2a0, %r8d            # imm = 0x2A0
140a8c802: 48 8d 75 c0                 	leaq	-0x40(%rbp), %rsi
140a8c806: 48 89 f1                    	movq	%rsi, %rcx
140a8c809: 48 8d 95 48 08 00 00        	leaq	0x848(%rbp), %rdx
140a8c810: e8 eb 73 e9 01              	callq	0x142923c00 <.text+0x2922c00>
140a8c815: 31 c0                       	xorl	%eax, %eax
140a8c817: 48 3b 45 c0                 	cmpq	-0x40(%rbp), %rax
140a8c81b: 71 1b                       	jno	0x140a8c838 <.text+0xa8b838>
140a8c81d: 48 c7 85 c8 0c 00 00 00 00 00 00    	movq	$0x0, 0xcc8(%rbp)
140a8c828: 41 b2 01                    	movb	$0x1, %r10b
140a8c82b: b0 01                       	movb	$0x1, %al
140a8c82d: 89 85 28 0d 00 00           	movl	%eax, 0xd28(%rbp)
140a8c833: e9 9e 02 00 00              	jmp	0x140a8cad6 <.text+0xa8bad6>
140a8c838: 41 b8 a0 02 00 00           	movl	$0x2a0, %r8d            # imm = 0x2A0
140a8c83e: 48 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %rcx
140a8c845: 48 89 f2                    	movq	%rsi, %rdx
140a8c848: e8 b3 73 e9 01              	callq	0x142923c00 <.text+0x2922c00>
140a8c84d: 80 bd 38 0d 00 00 00        	cmpb	$0x0, 0xd38(%rbp)
140a8c854: 0f 85 56 18 00 00           	jne	0x140a8e0b0 <.text+0xa8d0b0>
140a8c85a: 0f b6 85 b0 01 00 00        	movzbl	0x1b0(%rbp), %eax
140a8c861: 88 85 2f 0d 00 00           	movb	%al, 0xd2f(%rbp)
140a8c867: 48 8d 95 b1 01 00 00        	leaq	0x1b1(%rbp), %rdx
140a8c86e: 8b 02                       	movl	(%rdx), %eax
140a8c870: 8b 4a 03                    	movl	0x3(%rdx), %ecx
140a8c873: 89 8d 93 0c 00 00           	movl	%ecx, 0xc93(%rbp)
140a8c879: 89 85 90 0c 00 00           	movl	%eax, 0xc90(%rbp)
140a8c87f: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
140a8c886: 48 89 85 30 0c 00 00        	movq	%rax, 0xc30(%rbp)
140a8c88d: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
140a8c894: 48 89 85 38 0c 00 00        	movq	%rax, 0xc38(%rbp)
140a8c89b: 0f 10 42 17                 	movups	0x17(%rdx), %xmm0
140a8c89f: 0f 29 85 b0 07 00 00        	movaps	%xmm0, 0x7b0(%rbp)
140a8c8a6: 48 8b 42 57                 	movq	0x57(%rdx), %rax
140a8c8aa: 48 89 85 30 08 00 00        	movq	%rax, 0x830(%rbp)
140a8c8b1: 0f 10 42 27                 	movups	0x27(%rdx), %xmm0
140a8c8b5: f3 0f 6f 4a 37              	movdqu	0x37(%rdx), %xmm1
140a8c8ba: f3 0f 6f 52 47              	movdqu	0x47(%rdx), %xmm2
140a8c8bf: 66 0f 7f 95 20 08 00 00     	movdqa	%xmm2, 0x820(%rbp)
140a8c8c7: 66 0f 7f 8d 10 08 00 00     	movdqa	%xmm1, 0x810(%rbp)
140a8c8cf: 0f 29 85 00 08 00 00        	movaps	%xmm0, 0x800(%rbp)
140a8c8d6: 8b 85 10 02 00 00           	movl	0x210(%rbp), %eax
140a8c8dc: 89 85 6c 0c 00 00           	movl	%eax, 0xc6c(%rbp)
140a8c8e2: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140a8c8e8: 89 85 64 0c 00 00           	movl	%eax, 0xc64(%rbp)
140a8c8ee: 48 8b 95 40 08 00 00        	movq	0x840(%rbp), %rdx
140a8c8f5: 48 85 d2                    	testq	%rdx, %rdx
140a8c8f8: 74 16                       	je	0x140a8c910 <.text+0xa8b910>
140a8c8fa: 48 8b 8d 48 08 00 00        	movq	0x848(%rbp), %rcx
140a8c901: 48 c1 e2 03                 	shlq	$0x3, %rdx
140a8c905: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8c90b: e8 10 73 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8c910: 48 8b 95 58 08 00 00        	movq	0x858(%rbp), %rdx
140a8c917: 48 85 d2                    	testq	%rdx, %rdx
140a8c91a: 74 16                       	je	0x140a8c932 <.text+0xa8b932>
140a8c91c: 48 8b 8d 60 08 00 00        	movq	0x860(%rbp), %rcx
140a8c923: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8c927: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8c92d: e8 ee 72 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8c932: 48 8b 85 78 08 00 00        	movq	0x878(%rbp), %rax
140a8c939: 48 85 c0                    	testq	%rax, %rax
140a8c93c: 74 1a                       	je	0x140a8c958 <.text+0xa8b958>
140a8c93e: 48 8b 8d 80 08 00 00        	movq	0x880(%rbp), %rcx
140a8c945: 48 c1 e0 02                 	shlq	$0x2, %rax
140a8c949: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140a8c94d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8c953: e8 c8 72 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8c958: 48 8b 95 90 08 00 00        	movq	0x890(%rbp), %rdx
140a8c95f: 48 85 d2                    	testq	%rdx, %rdx
140a8c962: 74 16                       	je	0x140a8c97a <.text+0xa8b97a>
140a8c964: 48 8b 8d 98 08 00 00        	movq	0x898(%rbp), %rcx
140a8c96b: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8c96f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8c975: e8 a6 72 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8c97a: 48 8b 95 b0 08 00 00        	movq	0x8b0(%rbp), %rdx
140a8c981: 48 85 d2                    	testq	%rdx, %rdx
140a8c984: 74 16                       	je	0x140a8c99c <.text+0xa8b99c>
140a8c986: 48 8b 8d b8 08 00 00        	movq	0x8b8(%rbp), %rcx
140a8c98d: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8c991: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8c997: e8 84 72 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8c99c: 48 8b 85 c8 08 00 00        	movq	0x8c8(%rbp), %rax
140a8c9a3: 48 85 c0                    	testq	%rax, %rax
140a8c9a6: 74 1a                       	je	0x140a8c9c2 <.text+0xa8b9c2>
140a8c9a8: 48 8b 8d d0 08 00 00        	movq	0x8d0(%rbp), %rcx
140a8c9af: 48 c1 e0 02                 	shlq	$0x2, %rax
140a8c9b3: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140a8c9b7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8c9bd: e8 5e 72 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8c9c2: 48 8b 95 e0 08 00 00        	movq	0x8e0(%rbp), %rdx
140a8c9c9: 48 85 d2                    	testq	%rdx, %rdx
140a8c9cc: 74 16                       	je	0x140a8c9e4 <.text+0xa8b9e4>
140a8c9ce: 48 8b 8d e8 08 00 00        	movq	0x8e8(%rbp), %rcx
140a8c9d5: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8c9d9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8c9df: e8 3c 72 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8c9e4: 48 8b 95 00 09 00 00        	movq	0x900(%rbp), %rdx
140a8c9eb: 48 85 d2                    	testq	%rdx, %rdx
140a8c9ee: 74 16                       	je	0x140a8ca06 <.text+0xa8ba06>
140a8c9f0: 48 8b 8d 08 09 00 00        	movq	0x908(%rbp), %rcx
140a8c9f7: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8c9fb: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8ca01: e8 1a 72 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8ca06: 48 8d 8d 58 09 00 00        	leaq	0x958(%rbp), %rcx
140a8ca0d: e8 7e 29 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8ca12: 48 8d 8d 68 09 00 00        	leaq	0x968(%rbp), %rcx
140a8ca19: e8 72 29 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8ca1e: 48 8d 8d 78 09 00 00        	leaq	0x978(%rbp), %rcx
140a8ca25: e8 66 29 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8ca2a: 48 8d 8d 88 09 00 00        	leaq	0x988(%rbp), %rcx
140a8ca31: e8 5a 29 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8ca36: 48 8d 8d 98 09 00 00        	leaq	0x998(%rbp), %rcx
140a8ca3d: e8 4e 29 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8ca42: 48 8d 8d a8 09 00 00        	leaq	0x9a8(%rbp), %rcx
140a8ca49: e8 42 29 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8ca4e: 48 8d 8d b8 09 00 00        	leaq	0x9b8(%rbp), %rcx
140a8ca55: e8 36 29 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8ca5a: 48 8b 95 18 09 00 00        	movq	0x918(%rbp), %rdx
140a8ca61: 48 85 d2                    	testq	%rdx, %rdx
140a8ca64: 74 16                       	je	0x140a8ca7c <.text+0xa8ba7c>
140a8ca66: 48 8b 8d 20 09 00 00        	movq	0x920(%rbp), %rcx
140a8ca6d: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8ca71: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8ca77: e8 a4 71 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8ca7c: 48 8b 95 30 09 00 00        	movq	0x930(%rbp), %rdx
140a8ca83: 48 85 d2                    	testq	%rdx, %rdx
140a8ca86: 74 16                       	je	0x140a8ca9e <.text+0xa8ba9e>
140a8ca88: 48 8b 8d 38 09 00 00        	movq	0x938(%rbp), %rcx
140a8ca8f: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8ca93: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8ca99: e8 82 71 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8ca9e: 48 8b 95 d0 09 00 00        	movq	0x9d0(%rbp), %rdx
140a8caa5: 48 85 d2                    	testq	%rdx, %rdx
140a8caa8: 74 16                       	je	0x140a8cac0 <.text+0xa8bac0>
140a8caaa: 48 8b 8d d8 09 00 00        	movq	0x9d8(%rbp), %rcx
140a8cab1: 48 c1 e2 03                 	shlq	$0x3, %rdx
140a8cab5: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8cabb: e8 60 71 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8cac0: b0 01                       	movb	$0x1, %al
140a8cac2: 48 89 85 c8 0c 00 00        	movq	%rax, 0xcc8(%rbp)
140a8cac9: 45 31 d2                    	xorl	%r10d, %r10d
140a8cacc: c7 85 28 0d 00 00 00 00 00 00       	movl	$0x0, 0xd28(%rbp)
140a8cad6: 48 8d 04 3b                 	leaq	(%rbx,%rdi), %rax
140a8cada: 48 89 85 40 0d 00 00        	movq	%rax, 0xd40(%rbp)
140a8cae1: 4c 8d 1c 3b                 	leaq	(%rbx,%rdi), %r11
140a8cae5: 49 83 c3 90                 	addq	$-0x70, %r11
140a8cae9: 8b 85 90 0c 00 00           	movl	0xc90(%rbp), %eax
140a8caef: 8b 8d 93 0c 00 00           	movl	0xc93(%rbp), %ecx
140a8caf5: 89 85 88 0c 00 00           	movl	%eax, 0xc88(%rbp)
140a8cafb: 89 8d 8b 0c 00 00           	movl	%ecx, 0xc8b(%rbp)
140a8cb01: 0f 28 85 b0 07 00 00        	movaps	0x7b0(%rbp), %xmm0
140a8cb08: 0f 29 85 a0 07 00 00        	movaps	%xmm0, 0x7a0(%rbp)
140a8cb0f: 48 8b 85 30 08 00 00        	movq	0x830(%rbp), %rax
140a8cb16: 48 89 85 f0 07 00 00        	movq	%rax, 0x7f0(%rbp)
140a8cb1d: 0f 28 85 00 08 00 00        	movaps	0x800(%rbp), %xmm0
140a8cb24: 0f 28 8d 10 08 00 00        	movaps	0x810(%rbp), %xmm1
140a8cb2b: 0f 28 95 20 08 00 00        	movaps	0x820(%rbp), %xmm2
140a8cb32: 0f 29 95 e0 07 00 00        	movaps	%xmm2, 0x7e0(%rbp)
140a8cb39: 0f 29 8d d0 07 00 00        	movaps	%xmm1, 0x7d0(%rbp)
140a8cb40: 0f 29 85 c0 07 00 00        	movaps	%xmm0, 0x7c0(%rbp)
140a8cb47: 48 8b 85 b8 0c 00 00        	movq	0xcb8(%rbp), %rax
140a8cb4e: 48 8b 95 60 0b 00 00        	movq	0xb60(%rbp), %rdx
140a8cb55: 48 8b 4a 08                 	movq	0x8(%rdx), %rcx
140a8cb59: 48 69 52 10 98 00 00 00     	imulq	$0x98, 0x10(%rdx), %rdx
140a8cb61: 49 89 d1                    	movq	%rdx, %r9
140a8cb64: 48 85 d2                    	testq	%rdx, %rdx
140a8cb67: 74 17                       	je	0x140a8cb80 <.text+0xa8bb80>
140a8cb69: 4c 8d 81 98 00 00 00        	leaq	0x98(%rcx), %r8
140a8cb70: 49 8d 91 68 ff ff ff        	leaq	-0x98(%r9), %rdx
140a8cb77: 48 39 41 70                 	cmpq	%rax, 0x70(%rcx)
140a8cb7b: 4c 89 c1                    	movq	%r8, %rcx
140a8cb7e: 75 e1                       	jne	0x140a8cb61 <.text+0xa8bb61>
140a8cb80: 4c 89 8d 88 0b 00 00        	movq	%r9, 0xb88(%rbp)
140a8cb87: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8cb8e: 48 8b 78 a0                 	movq	-0x60(%rax), %rdi
140a8cb92: 4c 8d 0c fd 00 00 00 00     	leaq	(,%rdi,8), %r9
140a8cb9a: 48 89 f8                    	movq	%rdi, %rax
140a8cb9d: 48 c1 e8 3d                 	shrq	$0x3d, %rax
140a8cba1: 44 89 95 a8 0c 00 00        	movl	%r10d, 0xca8(%rbp)
140a8cba8: 0f 85 f5 15 00 00           	jne	0x140a8e1a3 <.text+0xa8d1a3>
140a8cbae: b9 00 00 00 00              	movl	$0x0, %ecx
140a8cbb3: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8cbbd: 49 39 c1                    	cmpq	%rax, %r9
140a8cbc0: 0f 87 df 15 00 00           	ja	0x140a8e1a5 <.text+0xa8d1a5>
140a8cbc6: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8cbcd: 48 8b 70 98                 	movq	-0x68(%rax), %rsi
140a8cbd1: 4d 85 c9                    	testq	%r9, %r9
140a8cbd4: 4c 89 9d 98 0c 00 00        	movq	%r11, 0xc98(%rbp)
140a8cbdb: 74 39                       	je	0x140a8cc16 <.text+0xa8bc16>
140a8cbdd: 0f b6 05 25 80 ba 02        	movzbl	0x2ba8025(%rip), %eax   # 0x143634c09
140a8cbe4: ba 04 00 00 00              	movl	$0x4, %edx
140a8cbe9: 4c 89 c9                    	movq	%r9, %rcx
140a8cbec: 4c 89 8d 18 0d 00 00        	movq	%r9, 0xd18(%rbp)
140a8cbf3: e8 18 70 b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8cbf8: b9 04 00 00 00              	movl	$0x4, %ecx
140a8cbfd: 4c 8b 8d 18 0d 00 00        	movq	0xd18(%rbp), %r9
140a8cc04: 48 85 c0                    	testq	%rax, %rax
140a8cc07: 0f 84 98 15 00 00           	je	0x140a8e1a5 <.text+0xa8d1a5>
140a8cc0d: 48 89 bd 38 0d 00 00        	movq	%rdi, 0xd38(%rbp)
140a8cc14: eb 10                       	jmp	0x140a8cc26 <.text+0xa8bc26>
140a8cc16: b8 04 00 00 00              	movl	$0x4, %eax
140a8cc1b: 48 c7 85 38 0d 00 00 00 00 00 00    	movq	$0x0, 0xd38(%rbp)
140a8cc26: 48 89 85 18 0d 00 00        	movq	%rax, 0xd18(%rbp)
140a8cc2d: 48 89 c1                    	movq	%rax, %rcx
140a8cc30: 48 89 f2                    	movq	%rsi, %rdx
140a8cc33: 4d 89 c8                    	movq	%r9, %r8
140a8cc36: e8 c5 6f e9 01              	callq	0x142923c00 <.text+0x2922c00>
140a8cc3b: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8cc42: 48 8b 58 b8                 	movq	-0x48(%rax), %rbx
140a8cc46: 4c 8d 0c 9d 00 00 00 00     	leaq	(,%rbx,4), %r9
140a8cc4e: 48 89 d8                    	movq	%rbx, %rax
140a8cc51: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140a8cc55: 0f 85 5b 15 00 00           	jne	0x140a8e1b6 <.text+0xa8d1b6>
140a8cc5b: b9 00 00 00 00              	movl	$0x0, %ecx
140a8cc60: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8cc6a: 49 39 c1                    	cmpq	%rax, %r9
140a8cc6d: 0f 87 45 15 00 00           	ja	0x140a8e1b8 <.text+0xa8d1b8>
140a8cc73: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8cc7a: 48 8b 70 b0                 	movq	-0x50(%rax), %rsi
140a8cc7e: 4d 85 c9                    	testq	%r9, %r9
140a8cc81: 74 39                       	je	0x140a8ccbc <.text+0xa8bcbc>
140a8cc83: 0f b6 05 7f 7f ba 02        	movzbl	0x2ba7f7f(%rip), %eax   # 0x143634c09
140a8cc8a: ba 04 00 00 00              	movl	$0x4, %edx
140a8cc8f: 4c 89 c9                    	movq	%r9, %rcx
140a8cc92: 4c 89 8d d0 0c 00 00        	movq	%r9, 0xcd0(%rbp)
140a8cc99: e8 72 6f b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8cc9e: b9 04 00 00 00              	movl	$0x4, %ecx
140a8cca3: 4c 8b 8d d0 0c 00 00        	movq	0xcd0(%rbp), %r9
140a8ccaa: 48 85 c0                    	testq	%rax, %rax
140a8ccad: 0f 84 05 15 00 00           	je	0x140a8e1b8 <.text+0xa8d1b8>
140a8ccb3: 48 89 9d 08 0d 00 00        	movq	%rbx, 0xd08(%rbp)
140a8ccba: eb 10                       	jmp	0x140a8cccc <.text+0xa8bccc>
140a8ccbc: b8 04 00 00 00              	movl	$0x4, %eax
140a8ccc1: 48 c7 85 08 0d 00 00 00 00 00 00    	movq	$0x0, 0xd08(%rbp)
140a8cccc: 48 89 c1                    	movq	%rax, %rcx
140a8cccf: 48 89 f2                    	movq	%rsi, %rdx
140a8ccd2: 4d 89 c8                    	movq	%r9, %r8
140a8ccd5: 48 89 c6                    	movq	%rax, %rsi
140a8ccd8: e8 23 6f e9 01              	callq	0x142923c00 <.text+0x2922c00>
140a8ccdd: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8cce4: 66 0f 6e 78 c0              	movd	-0x40(%rax), %xmm7
140a8cce9: 48 8b 8d 38 0d 00 00        	movq	0xd38(%rbp), %rcx
140a8ccf0: 48 89 8d 40 08 00 00        	movq	%rcx, 0x840(%rbp)
140a8ccf7: 48 8b 8d 18 0d 00 00        	movq	0xd18(%rbp), %rcx
140a8ccfe: 48 89 8d 48 08 00 00        	movq	%rcx, 0x848(%rbp)
140a8cd05: 48 89 bd 50 08 00 00        	movq	%rdi, 0x850(%rbp)
140a8cd0c: 48 8b 8d 08 0d 00 00        	movq	0xd08(%rbp), %rcx
140a8cd13: 48 89 8d 58 08 00 00        	movq	%rcx, 0x858(%rbp)
140a8cd1a: 48 89 b5 60 08 00 00        	movq	%rsi, 0x860(%rbp)
140a8cd21: 48 89 9d 68 08 00 00        	movq	%rbx, 0x868(%rbp)
140a8cd28: 66 0f 7e bd 70 08 00 00     	movd	%xmm7, 0x870(%rbp)
140a8cd30: 8b 40 c8                    	movl	-0x38(%rax), %eax
140a8cd33: 48 89 85 d0 0c 00 00        	movq	%rax, 0xcd0(%rbp)
140a8cd3a: 83 f8 01                    	cmpl	$0x1, %eax
140a8cd3d: 41 0f 94 c0                 	sete	%r8b
140a8cd41: 48 8b 85 98 0c 00 00        	movq	0xc98(%rbp), %rax
140a8cd48: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140a8cd4d: 48 8b 85 e0 0c 00 00        	movq	0xce0(%rbp), %rax
140a8cd54: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a8cd59: 48 8d 8d 60 02 00 00        	leaq	0x260(%rbp), %rcx
140a8cd60: 48 8d 95 40 08 00 00        	leaq	0x840(%rbp), %rdx
140a8cd67: 0f 28 de                    	movaps	%xmm6, %xmm3
140a8cd6a: e8 01 ee 9d 00              	callq	0x14146bb70 <.text+0x146ab70>
140a8cd6f: 48 8b 95 40 08 00 00        	movq	0x840(%rbp), %rdx
140a8cd76: 48 85 d2                    	testq	%rdx, %rdx
140a8cd79: 74 16                       	je	0x140a8cd91 <.text+0xa8bd91>
140a8cd7b: 48 8b 8d 48 08 00 00        	movq	0x848(%rbp), %rcx
140a8cd82: 48 c1 e2 03                 	shlq	$0x3, %rdx
140a8cd86: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8cd8c: e8 8f 6e b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8cd91: 48 8b 95 58 08 00 00        	movq	0x858(%rbp), %rdx
140a8cd98: 48 85 d2                    	testq	%rdx, %rdx
140a8cd9b: 74 16                       	je	0x140a8cdb3 <.text+0xa8bdb3>
140a8cd9d: 48 8b 8d 60 08 00 00        	movq	0x860(%rbp), %rcx
140a8cda4: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8cda8: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8cdae: e8 6d 6e b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8cdb3: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
140a8cdba: 48 89 85 08 0d 00 00        	movq	%rax, 0xd08(%rbp)
140a8cdc1: 48 8b 9d 70 02 00 00        	movq	0x270(%rbp), %rbx
140a8cdc8: 48 8d 0c 9d 00 00 00 00     	leaq	(,%rbx,4), %rcx
140a8cdd0: 48 85 db                    	testq	%rbx, %rbx
140a8cdd3: 48 89 9d 18 0d 00 00        	movq	%rbx, 0xd18(%rbp)
140a8cdda: 48 89 8d d8 0c 00 00        	movq	%rcx, 0xcd8(%rbp)
140a8cde1: 74 69                       	je	0x140a8ce4c <.text+0xa8be4c>
140a8cde3: 0f b6 05 1f 7e ba 02        	movzbl	0x2ba7e1f(%rip), %eax   # 0x143634c09
140a8cdea: ba 04 00 00 00              	movl	$0x4, %edx
140a8cdef: e8 1c 6e b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8cdf4: 48 89 85 38 0d 00 00        	movq	%rax, 0xd38(%rbp)
140a8cdfb: 48 85 c0                    	testq	%rax, %rax
140a8cdfe: 0f 84 85 13 00 00           	je	0x140a8e189 <.text+0xa8d189>
140a8ce04: 48 8b 85 08 0d 00 00        	movq	0xd08(%rbp), %rax
140a8ce0b: 48 8d 70 08                 	leaq	0x8(%rax), %rsi
140a8ce0f: 31 ff                       	xorl	%edi, %edi
140a8ce11: f3 0f 10 4e f8              	movss	-0x8(%rsi), %xmm1
140a8ce16: f3 44 0f 10 4e fc           	movss	-0x4(%rsi), %xmm9
140a8ce1c: f3 0f 10 16                 	movss	(%rsi), %xmm2
140a8ce20: 48 8b 8d e0 0c 00 00        	movq	0xce0(%rbp), %rcx
140a8ce27: e8 14 c8 f5 ff              	callq	0x1409e9640 <.text+0x9e8640>
140a8ce2c: f3 44 0f 5c c8              	subss	%xmm0, %xmm9
140a8ce31: 48 8b 85 38 0d 00 00        	movq	0xd38(%rbp), %rax
140a8ce38: f3 44 0f 11 0c b8           	movss	%xmm9, (%rax,%rdi,4)
140a8ce3e: 48 ff c7                    	incq	%rdi
140a8ce41: 48 83 c6 0c                 	addq	$0xc, %rsi
140a8ce45: 48 39 fb                    	cmpq	%rdi, %rbx
140a8ce48: 75 c7                       	jne	0x140a8ce11 <.text+0xa8be11>
140a8ce4a: eb 0c                       	jmp	0x140a8ce58 <.text+0xa8be58>
140a8ce4c: b8 04 00 00 00              	movl	$0x4, %eax
140a8ce51: 48 89 85 38 0d 00 00        	movq	%rax, 0xd38(%rbp)
140a8ce58: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8ce5f: f3 44 0f 10 60 e4           	movss	-0x1c(%rax), %xmm12
140a8ce65: 0f b6 40 f5                 	movzbl	-0xb(%rax), %eax
140a8ce69: 88 85 98 0c 00 00           	movb	%al, 0xc98(%rbp)
140a8ce6f: 48 8b 8d 18 0c 00 00        	movq	0xc18(%rbp), %rcx
140a8ce76: 48 8d 15 3b 14 1a 02        	leaq	0x21a143b(%rip), %rdx   # 0x142c2e2b8
140a8ce7d: e8 9e c8 ed ff              	callq	0x140969720 <.text+0x968720>
140a8ce82: 48 89 c6                    	movq	%rax, %rsi
140a8ce85: 48 85 c0                    	testq	%rax, %rax
140a8ce88: 0f 84 50 12 00 00           	je	0x140a8e0de <.text+0xa8d0de>
140a8ce8e: 48 8d 0d 6a 09 01 02        	leaq	0x201096a(%rip), %rcx   # 0x142a9d7ff
140a8ce95: 48 89 8d 40 0c 00 00        	movq	%rcx, 0xc40(%rbp)
140a8ce9c: 48 c7 85 48 0c 00 00 3a 00 00 00    	movq	$0x3a, 0xc48(%rbp)
140a8cea7: 48 89 f7                    	movq	%rsi, %rdi
140a8ceaa: 48 83 c7 10                 	addq	$0x10, %rdi
140a8ceae: 48 83 7e 18 3a              	cmpq	$0x3a, 0x18(%rsi)
140a8ceb3: 0f 85 92 11 00 00           	jne	0x140a8e04b <.text+0xa8d04b>
140a8ceb9: 48 8b 17                    	movq	(%rdi), %rdx
140a8cebc: 41 b8 3a 00 00 00           	movl	$0x3a, %r8d
140a8cec2: e8 29 6d e9 01              	callq	0x142923bf0 <.text+0x2922bf0>
140a8cec7: 85 c0                       	testl	%eax, %eax
140a8cec9: 0f 85 7c 11 00 00           	jne	0x140a8e04b <.text+0xa8d04b>
140a8cecf: 48 8b 0e                    	movq	(%rsi), %rcx
140a8ced2: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140a8ced6: ff 50 18                    	callq	*0x18(%rax)
140a8ced9: 48 89 c7                    	movq	%rax, %rdi
140a8cedc: 48 85 c0                    	testq	%rax, %rax
140a8cedf: 0f 84 f9 11 00 00           	je	0x140a8e0de <.text+0xa8d0de>
140a8cee5: 0f b6 05 1d 7d ba 02        	movzbl	0x2ba7d1d(%rip), %eax   # 0x143634c09
140a8ceec: b9 20 00 00 00              	movl	$0x20, %ecx
140a8cef1: ba 08 00 00 00              	movl	$0x8, %edx
140a8cef6: e8 15 6d b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8cefb: 48 85 c0                    	testq	%rax, %rax
140a8cefe: 0f 84 fc 11 00 00           	je	0x140a8e100 <.text+0xa8d100>
140a8cf04: 48 89 c2                    	movq	%rax, %rdx
140a8cf07: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140a8cf0e: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140a8cf16: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140a8cf1e: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140a8cf22: 48 8b 77 60                 	movq	0x60(%rdi), %rsi
140a8cf26: eb 02                       	jmp	0x140a8cf2a <.text+0xa8bf2a>
140a8cf28: f3 90                       	pause
140a8cf2a: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140a8cf2e: 66 90                       	nop
140a8cf30: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140a8cf34: 74 f2                       	je	0x140a8cf28 <.text+0xa8bf28>
140a8cf36: 48 85 c0                    	testq	%rax, %rax
140a8cf39: 0f 88 64 10 00 00           	js	0x140a8dfa3 <.text+0xa8cfa3>
140a8cf3f: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
140a8cf43: f0                          	lock
140a8cf44: 48 0f b1 4e 08              	cmpxchgq	%rcx, 0x8(%rsi)
140a8cf49: 75 e5                       	jne	0x140a8cf30 <.text+0xa8bf30>
140a8cf4b: 48 89 b5 48 0b 00 00        	movq	%rsi, 0xb48(%rbp)
140a8cf52: 48 89 95 90 0b 00 00        	movq	%rdx, 0xb90(%rbp)
140a8cf59: 48 89 95 50 0b 00 00        	movq	%rdx, 0xb50(%rbp)
140a8cf60: 0f b6 05 a2 7c ba 02        	movzbl	0x2ba7ca2(%rip), %eax   # 0x143634c09
140a8cf67: b9 20 00 00 00              	movl	$0x20, %ecx
140a8cf6c: ba 08 00 00 00              	movl	$0x8, %edx
140a8cf71: e8 9a 6c b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8cf76: 48 85 c0                    	testq	%rax, %rax
140a8cf79: 0f 84 95 11 00 00           	je	0x140a8e114 <.text+0xa8d114>
140a8cf7f: 48 89 c3                    	movq	%rax, %rbx
140a8cf82: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140a8cf89: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140a8cf91: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140a8cf99: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140a8cf9d: 48 8b 57 60                 	movq	0x60(%rdi), %rdx
140a8cfa1: eb 02                       	jmp	0x140a8cfa5 <.text+0xa8bfa5>
140a8cfa3: f3 90                       	pause
140a8cfa5: 48 8b 42 08                 	movq	0x8(%rdx), %rax
140a8cfa9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a8cfb0: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140a8cfb4: 74 ed                       	je	0x140a8cfa3 <.text+0xa8bfa3>
140a8cfb6: 48 85 c0                    	testq	%rax, %rax
140a8cfb9: 0f 88 fc 0f 00 00           	js	0x140a8dfbb <.text+0xa8cfbb>
140a8cfbf: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
140a8cfc3: f0                          	lock
140a8cfc4: 48 0f b1 4a 08              	cmpxchgq	%rcx, 0x8(%rdx)
140a8cfc9: 75 e5                       	jne	0x140a8cfb0 <.text+0xa8bfb0>
140a8cfcb: 48 89 95 98 0b 00 00        	movq	%rdx, 0xb98(%rbp)
140a8cfd2: 48 89 95 38 0b 00 00        	movq	%rdx, 0xb38(%rbp)
140a8cfd9: 48 89 9d 40 0b 00 00        	movq	%rbx, 0xb40(%rbp)
140a8cfe0: 0f b6 05 22 7c ba 02        	movzbl	0x2ba7c22(%rip), %eax   # 0x143634c09
140a8cfe7: b9 20 00 00 00              	movl	$0x20, %ecx
140a8cfec: ba 08 00 00 00              	movl	$0x8, %edx
140a8cff1: e8 1a 6c b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8cff6: 48 85 c0                    	testq	%rax, %rax
140a8cff9: 0f 84 29 11 00 00           	je	0x140a8e128 <.text+0xa8d128>
140a8cfff: 48 89 c2                    	movq	%rax, %rdx
140a8d002: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140a8d009: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140a8d011: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140a8d019: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140a8d01d: 4c 8b 47 60                 	movq	0x60(%rdi), %r8
140a8d021: eb 02                       	jmp	0x140a8d025 <.text+0xa8c025>
140a8d023: f3 90                       	pause
140a8d025: 49 8b 40 08                 	movq	0x8(%r8), %rax
140a8d029: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a8d030: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140a8d034: 74 ed                       	je	0x140a8d023 <.text+0xa8c023>
140a8d036: 48 85 c0                    	testq	%rax, %rax
140a8d039: 0f 88 94 0f 00 00           	js	0x140a8dfd3 <.text+0xa8cfd3>
140a8d03f: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
140a8d043: f0                          	lock
140a8d044: 49 0f b1 48 08              	cmpxchgq	%rcx, 0x8(%r8)
140a8d049: 75 e5                       	jne	0x140a8d030 <.text+0xa8c030>
140a8d04b: 4c 89 85 a8 0b 00 00        	movq	%r8, 0xba8(%rbp)
140a8d052: 4c 89 85 28 0b 00 00        	movq	%r8, 0xb28(%rbp)
140a8d059: 48 89 95 a0 0b 00 00        	movq	%rdx, 0xba0(%rbp)
140a8d060: 48 89 95 30 0b 00 00        	movq	%rdx, 0xb30(%rbp)
140a8d067: 0f b6 05 9b 7b ba 02        	movzbl	0x2ba7b9b(%rip), %eax   # 0x143634c09
140a8d06e: b9 20 00 00 00              	movl	$0x20, %ecx
140a8d073: ba 08 00 00 00              	movl	$0x8, %edx
140a8d078: e8 93 6b b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8d07d: 48 85 c0                    	testq	%rax, %rax
140a8d080: 0f 84 b6 10 00 00           	je	0x140a8e13c <.text+0xa8d13c>
140a8d086: 48 89 c2                    	movq	%rax, %rdx
140a8d089: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140a8d090: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140a8d098: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140a8d0a0: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140a8d0a4: 4c 8b 47 60                 	movq	0x60(%rdi), %r8
140a8d0a8: eb 02                       	jmp	0x140a8d0ac <.text+0xa8c0ac>
140a8d0aa: f3 90                       	pause
140a8d0ac: 49 8b 40 08                 	movq	0x8(%r8), %rax
140a8d0b0: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140a8d0b4: 74 f4                       	je	0x140a8d0aa <.text+0xa8c0aa>
140a8d0b6: 48 85 c0                    	testq	%rax, %rax
140a8d0b9: 0f 88 2c 0f 00 00           	js	0x140a8dfeb <.text+0xa8cfeb>
140a8d0bf: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
140a8d0c3: f0                          	lock
140a8d0c4: 49 0f b1 48 08              	cmpxchgq	%rcx, 0x8(%r8)
140a8d0c9: 75 e5                       	jne	0x140a8d0b0 <.text+0xa8c0b0>
140a8d0cb: 4c 89 85 b8 0b 00 00        	movq	%r8, 0xbb8(%rbp)
140a8d0d2: 4c 89 85 18 0b 00 00        	movq	%r8, 0xb18(%rbp)
140a8d0d9: 48 89 95 b0 0b 00 00        	movq	%rdx, 0xbb0(%rbp)
140a8d0e0: 48 89 95 20 0b 00 00        	movq	%rdx, 0xb20(%rbp)
140a8d0e7: 48 8b 8d 18 0c 00 00        	movq	0xc18(%rbp), %rcx
140a8d0ee: 48 8d 15 6b 12 1a 02        	leaq	0x21a126b(%rip), %rdx   # 0x142c2e360
140a8d0f5: e8 26 c6 ed ff              	callq	0x140969720 <.text+0x968720>
140a8d0fa: 48 89 c7                    	movq	%rax, %rdi
140a8d0fd: 48 85 c0                    	testq	%rax, %rax
140a8d100: 0f 84 e9 0f 00 00           	je	0x140a8e0ef <.text+0xa8d0ef>
140a8d106: 48 8d 0d 2c 0d 01 02        	leaq	0x2010d2c(%rip), %rcx   # 0x142a9de39
140a8d10d: 48 89 8d 40 0c 00 00        	movq	%rcx, 0xc40(%rbp)
140a8d114: 48 c7 85 48 0c 00 00 3d 00 00 00    	movq	$0x3d, 0xc48(%rbp)
140a8d11f: 49 89 f8                    	movq	%rdi, %r8
140a8d122: 49 83 c0 10                 	addq	$0x10, %r8
140a8d126: 48 83 7f 18 3d              	cmpq	$0x3d, 0x18(%rdi)
140a8d12b: 0f 85 4e 0f 00 00           	jne	0x140a8e07f <.text+0xa8d07f>
140a8d131: 49 8b 10                    	movq	(%r8), %rdx
140a8d134: 4c 89 85 78 0c 00 00        	movq	%r8, 0xc78(%rbp)
140a8d13b: 41 b8 3d 00 00 00           	movl	$0x3d, %r8d
140a8d141: e8 aa 6a e9 01              	callq	0x142923bf0 <.text+0x2922bf0>
140a8d146: 4c 8b 85 78 0c 00 00        	movq	0xc78(%rbp), %r8
140a8d14d: 85 c0                       	testl	%eax, %eax
140a8d14f: 0f 85 2a 0f 00 00           	jne	0x140a8e07f <.text+0xa8d07f>
140a8d155: 48 8b 0f                    	movq	(%rdi), %rcx
140a8d158: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140a8d15c: ff 50 18                    	callq	*0x18(%rax)
140a8d15f: 48 89 c7                    	movq	%rax, %rdi
140a8d162: 48 85 c0                    	testq	%rax, %rax
140a8d165: 0f 84 84 0f 00 00           	je	0x140a8e0ef <.text+0xa8d0ef>
140a8d16b: 48 89 9d c0 0b 00 00        	movq	%rbx, 0xbc0(%rbp)
140a8d172: 48 89 b5 78 0c 00 00        	movq	%rsi, 0xc78(%rbp)
140a8d179: 0f b6 05 89 7a ba 02        	movzbl	0x2ba7a89(%rip), %eax   # 0x143634c09
140a8d180: b9 20 00 00 00              	movl	$0x20, %ecx
140a8d185: ba 08 00 00 00              	movl	$0x8, %edx
140a8d18a: e8 81 6a b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8d18f: 48 85 c0                    	testq	%rax, %rax
140a8d192: 48 8b b5 18 0d 00 00        	movq	0xd18(%rbp), %rsi
140a8d199: 48 8b 9d 38 0d 00 00        	movq	0xd38(%rbp), %rbx
140a8d1a0: 0f 84 aa 0f 00 00           	je	0x140a8e150 <.text+0xa8d150>
140a8d1a6: 48 89 c2                    	movq	%rax, %rdx
140a8d1a9: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140a8d1b0: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140a8d1b8: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140a8d1c0: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140a8d1c4: 4c 8b 47 60                 	movq	0x60(%rdi), %r8
140a8d1c8: eb 02                       	jmp	0x140a8d1cc <.text+0xa8c1cc>
140a8d1ca: f3 90                       	pause
140a8d1cc: 49 8b 40 08                 	movq	0x8(%r8), %rax
140a8d1d0: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140a8d1d4: 74 f4                       	je	0x140a8d1ca <.text+0xa8c1ca>
140a8d1d6: 48 85 c0                    	testq	%rax, %rax
140a8d1d9: 0f 88 24 0e 00 00           	js	0x140a8e003 <.text+0xa8d003>
140a8d1df: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
140a8d1e3: f0                          	lock
140a8d1e4: 49 0f b1 48 08              	cmpxchgq	%rcx, 0x8(%r8)
140a8d1e9: 75 e5                       	jne	0x140a8d1d0 <.text+0xa8c1d0>
140a8d1eb: 4c 89 85 d0 0b 00 00        	movq	%r8, 0xbd0(%rbp)
140a8d1f2: 4c 89 85 08 0b 00 00        	movq	%r8, 0xb08(%rbp)
140a8d1f9: 48 89 95 c8 0b 00 00        	movq	%rdx, 0xbc8(%rbp)
140a8d200: 48 89 95 10 0b 00 00        	movq	%rdx, 0xb10(%rbp)
140a8d207: 0f b6 05 fb 79 ba 02        	movzbl	0x2ba79fb(%rip), %eax   # 0x143634c09
140a8d20e: b9 20 00 00 00              	movl	$0x20, %ecx
140a8d213: ba 08 00 00 00              	movl	$0x8, %edx
140a8d218: e8 f3 69 b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8d21d: 48 85 c0                    	testq	%rax, %rax
140a8d220: 0f 84 3e 0f 00 00           	je	0x140a8e164 <.text+0xa8d164>
140a8d226: 48 89 c2                    	movq	%rax, %rdx
140a8d229: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140a8d230: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140a8d238: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140a8d240: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140a8d244: 4c 8b 47 60                 	movq	0x60(%rdi), %r8
140a8d248: eb 02                       	jmp	0x140a8d24c <.text+0xa8c24c>
140a8d24a: f3 90                       	pause
140a8d24c: 49 8b 40 08                 	movq	0x8(%r8), %rax
140a8d250: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140a8d254: 74 f4                       	je	0x140a8d24a <.text+0xa8c24a>
140a8d256: 48 85 c0                    	testq	%rax, %rax
140a8d259: 0f 88 bc 0d 00 00           	js	0x140a8e01b <.text+0xa8d01b>
140a8d25f: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
140a8d263: f0                          	lock
140a8d264: 49 0f b1 48 08              	cmpxchgq	%rcx, 0x8(%r8)
140a8d269: 75 e5                       	jne	0x140a8d250 <.text+0xa8c250>
140a8d26b: 4c 89 85 e0 0b 00 00        	movq	%r8, 0xbe0(%rbp)
140a8d272: 4c 89 85 f8 0a 00 00        	movq	%r8, 0xaf8(%rbp)
140a8d279: 48 89 95 d8 0b 00 00        	movq	%rdx, 0xbd8(%rbp)
140a8d280: 48 89 95 00 0b 00 00        	movq	%rdx, 0xb00(%rbp)
140a8d287: 0f b6 05 7b 79 ba 02        	movzbl	0x2ba797b(%rip), %eax   # 0x143634c09
140a8d28e: b9 20 00 00 00              	movl	$0x20, %ecx
140a8d293: ba 08 00 00 00              	movl	$0x8, %edx
140a8d298: e8 73 69 b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8d29d: 48 85 c0                    	testq	%rax, %rax
140a8d2a0: 0f 84 d2 0e 00 00           	je	0x140a8e178 <.text+0xa8d178>
140a8d2a6: 49 89 c2                    	movq	%rax, %r10
140a8d2a9: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140a8d2b0: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140a8d2b8: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140a8d2c0: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140a8d2c4: 48 8b 7f 60                 	movq	0x60(%rdi), %rdi
140a8d2c8: eb 02                       	jmp	0x140a8d2cc <.text+0xa8c2cc>
140a8d2ca: f3 90                       	pause
140a8d2cc: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140a8d2d0: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140a8d2d4: 74 f4                       	je	0x140a8d2ca <.text+0xa8c2ca>
140a8d2d6: 48 85 c0                    	testq	%rax, %rax
140a8d2d9: 0f 88 54 0d 00 00           	js	0x140a8e033 <.text+0xa8d033>
140a8d2df: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
140a8d2e3: f0                          	lock
140a8d2e4: 48 0f b1 4f 08              	cmpxchgq	%rcx, 0x8(%rdi)
140a8d2e9: 75 e5                       	jne	0x140a8d2d0 <.text+0xa8c2d0>
140a8d2eb: 48 89 bd e8 0a 00 00        	movq	%rdi, 0xae8(%rbp)
140a8d2f2: 4c 89 95 f0 0a 00 00        	movq	%r10, 0xaf0(%rbp)
140a8d2f9: 45 0f 28 ca                 	movaps	%xmm10, %xmm9
140a8d2fd: 48 85 f6                    	testq	%rsi, %rsi
140a8d300: 0f 84 97 00 00 00           	je	0x140a8d39d <.text+0xa8c39d>
140a8d306: 48 83 fe 01                 	cmpq	$0x1, %rsi
140a8d30a: 75 08                       	jne	0x140a8d314 <.text+0xa8c314>
140a8d30c: 45 0f 28 ca                 	movaps	%xmm10, %xmm9
140a8d310: 31 c0                       	xorl	%eax, %eax
140a8d312: eb 5f                       	jmp	0x140a8d373 <.text+0xa8c373>
140a8d314: 48 89 f1                    	movq	%rsi, %rcx
140a8d317: 48 83 e1 fe                 	andq	$-0x2, %rcx
140a8d31b: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
140a8d31f: 31 c0                       	xorl	%eax, %eax
140a8d321: 45 0f 28 ca                 	movaps	%xmm10, %xmm9
140a8d325: f3 0f 10 0c 83              	movss	(%rbx,%rax,4), %xmm1
140a8d32a: f3 0f 10 54 83 04           	movss	0x4(%rbx,%rax,4), %xmm2
140a8d330: f3 45 0f c2 c9 03           	cmpunordss	%xmm9, %xmm9
140a8d336: 41 0f 28 d9                 	movaps	%xmm9, %xmm3
140a8d33a: 0f 54 d9                    	andps	%xmm1, %xmm3
140a8d33d: f3 0f 5d c8                 	minss	%xmm0, %xmm1
140a8d341: 44 0f 55 c9                 	andnps	%xmm1, %xmm9
140a8d345: 44 0f 56 cb                 	orps	%xmm3, %xmm9
140a8d349: 0f 28 c2                    	movaps	%xmm2, %xmm0
140a8d34c: f3 41 0f 5d c1              	minss	%xmm9, %xmm0
140a8d351: f3 45 0f c2 c9 03           	cmpunordss	%xmm9, %xmm9
140a8d357: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
140a8d35b: 0f 55 c8                    	andnps	%xmm0, %xmm1
140a8d35e: 44 0f 54 ca                 	andps	%xmm2, %xmm9
140a8d362: 44 0f 56 c9                 	orps	%xmm1, %xmm9
140a8d366: 48 83 c0 02                 	addq	$0x2, %rax
140a8d36a: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
140a8d36e: 48 39 c1                    	cmpq	%rax, %rcx
140a8d371: 75 b2                       	jne	0x140a8d325 <.text+0xa8c325>
140a8d373: 40 f6 c6 01                 	testb	$0x1, %sil
140a8d377: 74 24                       	je	0x140a8d39d <.text+0xa8c39d>
140a8d379: f3 0f 10 04 83              	movss	(%rbx,%rax,4), %xmm0
140a8d37e: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
140a8d382: f3 41 0f c2 c9 03           	cmpunordss	%xmm9, %xmm1
140a8d388: 0f 28 d1                    	movaps	%xmm1, %xmm2
140a8d38b: 0f 54 d0                    	andps	%xmm0, %xmm2
140a8d38e: f3 41 0f 5d c1              	minss	%xmm9, %xmm0
140a8d393: 0f 55 c8                    	andnps	%xmm0, %xmm1
140a8d396: 0f 56 ca                    	orps	%xmm2, %xmm1
140a8d399: 44 0f 28 c9                 	movaps	%xmm1, %xmm9
140a8d39d: 48 85 f6                    	testq	%rsi, %rsi
140a8d3a0: 74 78                       	je	0x140a8d41a <.text+0xa8c41a>
140a8d3a2: 48 89 f0                    	movq	%rsi, %rax
140a8d3a5: 48 ff c8                    	decq	%rax
140a8d3a8: 74 7d                       	je	0x140a8d427 <.text+0xa8c427>
140a8d3aa: 48 89 f2                    	movq	%rsi, %rdx
140a8d3ad: 48 83 e2 fe                 	andq	$-0x2, %rdx
140a8d3b1: 4c 8b 8d 08 0d 00 00        	movq	0xd08(%rbp), %r9
140a8d3b8: 4d 8d 41 10                 	leaq	0x10(%r9), %r8
140a8d3bc: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
140a8d3c0: 31 c9                       	xorl	%ecx, %ecx
140a8d3c2: 45 0f 28 d3                 	movaps	%xmm11, %xmm10
140a8d3c6: f3 41 0f 10 48 f4           	movss	-0xc(%r8), %xmm1
140a8d3cc: f3 41 0f 10 10              	movss	(%r8), %xmm2
140a8d3d1: f3 45 0f c2 d2 03           	cmpunordss	%xmm10, %xmm10
140a8d3d7: 41 0f 28 da                 	movaps	%xmm10, %xmm3
140a8d3db: 0f 54 d9                    	andps	%xmm1, %xmm3
140a8d3de: f3 0f 5d c8                 	minss	%xmm0, %xmm1
140a8d3e2: 44 0f 55 d1                 	andnps	%xmm1, %xmm10
140a8d3e6: 44 0f 56 d3                 	orps	%xmm3, %xmm10
140a8d3ea: 0f 28 c2                    	movaps	%xmm2, %xmm0
140a8d3ed: f3 41 0f 5d c2              	minss	%xmm10, %xmm0
140a8d3f2: f3 45 0f c2 d2 03           	cmpunordss	%xmm10, %xmm10
140a8d3f8: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
140a8d3fc: 0f 55 c8                    	andnps	%xmm0, %xmm1
140a8d3ff: 44 0f 54 d2                 	andps	%xmm2, %xmm10
140a8d403: 44 0f 56 d1                 	orps	%xmm1, %xmm10
140a8d407: 48 83 c1 02                 	addq	$0x2, %rcx
140a8d40b: 49 83 c0 18                 	addq	$0x18, %r8
140a8d40f: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
140a8d413: 48 39 ca                    	cmpq	%rcx, %rdx
140a8d416: 75 ae                       	jne	0x140a8d3c6 <.text+0xa8c3c6>
140a8d418: eb 1a                       	jmp	0x140a8d434 <.text+0xa8c434>
140a8d41a: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
140a8d41e: 45 0f 28 d3                 	movaps	%xmm11, %xmm10
140a8d422: e9 de 00 00 00              	jmp	0x140a8d505 <.text+0xa8c505>
140a8d427: 45 0f 28 d3                 	movaps	%xmm11, %xmm10
140a8d42b: 31 c9                       	xorl	%ecx, %ecx
140a8d42d: 4c 8b 8d 08 0d 00 00        	movq	0xd08(%rbp), %r9
140a8d434: 40 f6 c6 01                 	testb	$0x1, %sil
140a8d438: 74 2a                       	je	0x140a8d464 <.text+0xa8c464>
140a8d43a: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a8d43e: f3 41 0f 10 44 89 04        	movss	0x4(%r9,%rcx,4), %xmm0
140a8d445: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
140a8d449: f3 41 0f c2 ca 03           	cmpunordss	%xmm10, %xmm1
140a8d44f: 0f 28 d1                    	movaps	%xmm1, %xmm2
140a8d452: 0f 54 d0                    	andps	%xmm0, %xmm2
140a8d455: f3 41 0f 5d c2              	minss	%xmm10, %xmm0
140a8d45a: 0f 55 c8                    	andnps	%xmm0, %xmm1
140a8d45d: 0f 56 ca                    	orps	%xmm2, %xmm1
140a8d460: 44 0f 28 d1                 	movaps	%xmm1, %xmm10
140a8d464: 48 85 c0                    	testq	%rax, %rax
140a8d467: 74 66                       	je	0x140a8d4cf <.text+0xa8c4cf>
140a8d469: 48 89 f1                    	movq	%rsi, %rcx
140a8d46c: 48 83 e1 fe                 	andq	$-0x2, %rcx
140a8d470: 49 8d 51 10                 	leaq	0x10(%r9), %rdx
140a8d474: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a8d477: 31 c0                       	xorl	%eax, %eax
140a8d479: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
140a8d47d: f3 0f 10 4a f4              	movss	-0xc(%rdx), %xmm1
140a8d482: f3 0f 10 12                 	movss	(%rdx), %xmm2
140a8d486: f3 45 0f c2 ed 03           	cmpunordss	%xmm13, %xmm13
140a8d48c: 41 0f 28 dd                 	movaps	%xmm13, %xmm3
140a8d490: 0f 54 d9                    	andps	%xmm1, %xmm3
140a8d493: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
140a8d497: 44 0f 55 e9                 	andnps	%xmm1, %xmm13
140a8d49b: 44 0f 56 eb                 	orps	%xmm3, %xmm13
140a8d49f: 0f 28 c2                    	movaps	%xmm2, %xmm0
140a8d4a2: f3 41 0f 5f c5              	maxss	%xmm13, %xmm0
140a8d4a7: f3 45 0f c2 ed 03           	cmpunordss	%xmm13, %xmm13
140a8d4ad: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
140a8d4b1: 0f 55 c8                    	andnps	%xmm0, %xmm1
140a8d4b4: 44 0f 54 ea                 	andps	%xmm2, %xmm13
140a8d4b8: 44 0f 56 e9                 	orps	%xmm1, %xmm13
140a8d4bc: 48 83 c0 02                 	addq	$0x2, %rax
140a8d4c0: 48 83 c2 18                 	addq	$0x18, %rdx
140a8d4c4: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
140a8d4c8: 48 39 c1                    	cmpq	%rax, %rcx
140a8d4cb: 75 b0                       	jne	0x140a8d47d <.text+0xa8c47d>
140a8d4cd: eb 06                       	jmp	0x140a8d4d5 <.text+0xa8c4d5>
140a8d4cf: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
140a8d4d3: 31 c0                       	xorl	%eax, %eax
140a8d4d5: 40 f6 c6 01                 	testb	$0x1, %sil
140a8d4d9: 74 2a                       	je	0x140a8d505 <.text+0xa8c505>
140a8d4db: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140a8d4df: f3 41 0f 10 44 81 04        	movss	0x4(%r9,%rax,4), %xmm0
140a8d4e6: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
140a8d4ea: f3 41 0f c2 cd 03           	cmpunordss	%xmm13, %xmm1
140a8d4f0: 0f 28 d1                    	movaps	%xmm1, %xmm2
140a8d4f3: 0f 54 d0                    	andps	%xmm0, %xmm2
140a8d4f6: f3 41 0f 5f c5              	maxss	%xmm13, %xmm0
140a8d4fb: 0f 55 c8                    	andnps	%xmm0, %xmm1
140a8d4fe: 0f 56 ca                    	orps	%xmm2, %xmm1
140a8d501: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
140a8d505: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8d50c: 48 8b 40 a0                 	movq	-0x60(%rax), %rax
140a8d510: 48 8d 34 c5 00 00 00 00     	leaq	(,%rax,8), %rsi
140a8d518: 48 89 85 08 0d 00 00        	movq	%rax, 0xd08(%rbp)
140a8d51f: 48 c1 e8 3d                 	shrq	$0x3d, %rax
140a8d523: 0f 85 a0 0c 00 00           	jne	0x140a8e1c9 <.text+0xa8d1c9>
140a8d529: b9 00 00 00 00              	movl	$0x0, %ecx
140a8d52e: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8d538: 48 39 c6                    	cmpq	%rax, %rsi
140a8d53b: 0f 87 8a 0c 00 00           	ja	0x140a8e1cb <.text+0xa8d1cb>
140a8d541: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8d548: 48 8b 58 98                 	movq	-0x68(%rax), %rbx
140a8d54c: 48 85 f6                    	testq	%rsi, %rsi
140a8d54f: 4c 89 95 e8 0b 00 00        	movq	%r10, 0xbe8(%rbp)
140a8d556: 74 32                       	je	0x140a8d58a <.text+0xa8c58a>
140a8d558: 0f b6 05 aa 76 ba 02        	movzbl	0x2ba76aa(%rip), %eax   # 0x143634c09
140a8d55f: ba 04 00 00 00              	movl	$0x4, %edx
140a8d564: 48 89 f1                    	movq	%rsi, %rcx
140a8d567: e8 a4 66 b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8d56c: b9 04 00 00 00              	movl	$0x4, %ecx
140a8d571: 48 85 c0                    	testq	%rax, %rax
140a8d574: 0f 84 51 0c 00 00           	je	0x140a8e1cb <.text+0xa8d1cb>
140a8d57a: 48 8b 8d 08 0d 00 00        	movq	0xd08(%rbp), %rcx
140a8d581: 48 89 8d a0 0c 00 00        	movq	%rcx, 0xca0(%rbp)
140a8d588: eb 10                       	jmp	0x140a8d59a <.text+0xa8c59a>
140a8d58a: b8 04 00 00 00              	movl	$0x4, %eax
140a8d58f: 48 c7 85 a0 0c 00 00 00 00 00 00    	movq	$0x0, 0xca0(%rbp)
140a8d59a: 48 89 85 20 0c 00 00        	movq	%rax, 0xc20(%rbp)
140a8d5a1: 48 89 c1                    	movq	%rax, %rcx
140a8d5a4: 48 89 da                    	movq	%rbx, %rdx
140a8d5a7: 49 89 f0                    	movq	%rsi, %r8
140a8d5aa: e8 51 66 e9 01              	callq	0x142923c00 <.text+0x2922c00>
140a8d5af: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8d5b6: 48 8b 70 b8                 	movq	-0x48(%rax), %rsi
140a8d5ba: 4c 8d 0c b5 00 00 00 00     	leaq	(,%rsi,4), %r9
140a8d5c2: 48 89 f0                    	movq	%rsi, %rax
140a8d5c5: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140a8d5c9: 0f 85 0d 0c 00 00           	jne	0x140a8e1dc <.text+0xa8d1dc>
140a8d5cf: b9 00 00 00 00              	movl	$0x0, %ecx
140a8d5d4: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8d5de: 49 39 c1                    	cmpq	%rax, %r9
140a8d5e1: 0f 87 f7 0b 00 00           	ja	0x140a8e1de <.text+0xa8d1de>
140a8d5e7: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8d5ee: 48 8b 58 b0                 	movq	-0x50(%rax), %rbx
140a8d5f2: 4d 85 c9                    	testq	%r9, %r9
140a8d5f5: 74 3f                       	je	0x140a8d636 <.text+0xa8c636>
140a8d5f7: 0f b6 05 0b 76 ba 02        	movzbl	0x2ba760b(%rip), %eax   # 0x143634c09
140a8d5fe: ba 04 00 00 00              	movl	$0x4, %edx
140a8d603: 4c 89 c9                    	movq	%r9, %rcx
140a8d606: 48 89 b5 80 0c 00 00        	movq	%rsi, 0xc80(%rbp)
140a8d60d: 4c 89 ce                    	movq	%r9, %rsi
140a8d610: e8 fb 65 b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8d615: b9 04 00 00 00              	movl	$0x4, %ecx
140a8d61a: 49 89 f1                    	movq	%rsi, %r9
140a8d61d: 48 8b b5 80 0c 00 00        	movq	0xc80(%rbp), %rsi
140a8d624: 48 85 c0                    	testq	%rax, %rax
140a8d627: 0f 84 b1 0b 00 00           	je	0x140a8e1de <.text+0xa8d1de>
140a8d62d: 48 89 b5 a8 0c 00 00        	movq	%rsi, 0xca8(%rbp)
140a8d634: eb 10                       	jmp	0x140a8d646 <.text+0xa8c646>
140a8d636: b8 04 00 00 00              	movl	$0x4, %eax
140a8d63b: 48 c7 85 a8 0c 00 00 00 00 00 00    	movq	$0x0, 0xca8(%rbp)
140a8d646: 48 89 85 80 0c 00 00        	movq	%rax, 0xc80(%rbp)
140a8d64d: 48 89 c1                    	movq	%rax, %rcx
140a8d650: 48 89 da                    	movq	%rbx, %rdx
140a8d653: 4d 89 c8                    	movq	%r9, %r8
140a8d656: e8 a5 65 e9 01              	callq	0x142923c00 <.text+0x2922c00>
140a8d65b: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
140a8d662: f3 0f 10 40 c0              	movss	-0x40(%rax), %xmm0
140a8d667: 48 8b 95 d0 0c 00 00        	movq	0xcd0(%rbp), %rdx
140a8d66e: 8d 4a fe                    	leal	-0x2(%rdx), %ecx
140a8d671: 83 f9 02                    	cmpl	$0x2, %ecx
140a8d674: b8 02 00 00 00              	movl	$0x2, %eax
140a8d679: 0f 43 c8                    	cmovael	%eax, %ecx
140a8d67c: 85 c9                       	testl	%ecx, %ecx
140a8d67e: 74 2b                       	je	0x140a8d6ab <.text+0xa8c6ab>
140a8d680: 83 f9 02                    	cmpl	$0x2, %ecx
140a8d683: 75 21                       	jne	0x140a8d6a6 <.text+0xa8c6a6>
140a8d685: 48 8b 8d 40 0d 00 00        	movq	0xd40(%rbp), %rcx
140a8d68c: 48 8b 41 dc                 	movq	-0x24(%rcx), %rax
140a8d690: 48 89 85 50 0c 00 00        	movq	%rax, 0xc50(%rbp)
140a8d697: 0f 10 49 cc                 	movups	-0x34(%rcx), %xmm1
140a8d69b: 0f 29 8d 40 0c 00 00        	movaps	%xmm1, 0xc40(%rbp)
140a8d6a2: 89 d0                       	movl	%edx, %eax
140a8d6a4: eb 05                       	jmp	0x140a8d6ab <.text+0xa8c6ab>
140a8d6a6: b8 03 00 00 00              	movl	$0x3, %eax
140a8d6ab: 80 bd c8 0c 00 00 00        	cmpb	$0x0, 0xcc8(%rbp)
140a8d6b2: 0f 84 96 00 00 00           	je	0x140a8d74e <.text+0xa8c74e>
140a8d6b8: 8b 8d 88 0c 00 00           	movl	0xc88(%rbp), %ecx
140a8d6be: 8b 95 8b 0c 00 00           	movl	0xc8b(%rbp), %edx
140a8d6c4: 89 95 b3 0c 00 00           	movl	%edx, 0xcb3(%rbp)
140a8d6ca: 89 8d b0 0c 00 00           	movl	%ecx, 0xcb0(%rbp)
140a8d6d0: 0f 28 8d a0 07 00 00        	movaps	0x7a0(%rbp), %xmm1
140a8d6d7: 0f 29 8d f0 0b 00 00        	movaps	%xmm1, 0xbf0(%rbp)
140a8d6de: 0f 28 8d c0 07 00 00        	movaps	0x7c0(%rbp), %xmm1
140a8d6e5: 0f 28 95 d0 07 00 00        	movaps	0x7d0(%rbp), %xmm2
140a8d6ec: 0f 28 9d e0 07 00 00        	movaps	0x7e0(%rbp), %xmm3
140a8d6f3: 0f 29 8d 40 08 00 00        	movaps	%xmm1, 0x840(%rbp)
140a8d6fa: 0f 29 95 50 08 00 00        	movaps	%xmm2, 0x850(%rbp)
140a8d701: 0f 29 9d 60 08 00 00        	movaps	%xmm3, 0x860(%rbp)
140a8d708: 48 8b 8d f0 07 00 00        	movq	0x7f0(%rbp), %rcx
140a8d70f: 48 89 8d 70 08 00 00        	movq	%rcx, 0x870(%rbp)
140a8d716: 0f b6 8d 2f 0d 00 00        	movzbl	0xd2f(%rbp), %ecx
140a8d71d: 48 89 8d d8 0c 00 00        	movq	%rcx, 0xcd8(%rbp)
140a8d724: 48 8b 8d 30 0c 00 00        	movq	0xc30(%rbp), %rcx
140a8d72b: 48 89 8d 28 0c 00 00        	movq	%rcx, 0xc28(%rbp)
140a8d732: 48 8b 8d 38 0c 00 00        	movq	0xc38(%rbp), %rcx
140a8d739: 48 89 8d d0 0c 00 00        	movq	%rcx, 0xcd0(%rbp)
140a8d740: 8b 8d 6c 0c 00 00           	movl	0xc6c(%rbp), %ecx
140a8d746: 89 8d c8 0c 00 00           	movl	%ecx, 0xcc8(%rbp)
140a8d74c: eb 56                       	jmp	0x140a8d7a4 <.text+0xa8c7a4>
140a8d74e: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a8d751: 0f 29 8d 60 08 00 00        	movaps	%xmm1, 0x860(%rbp)
140a8d758: 0f 29 8d 50 08 00 00        	movaps	%xmm1, 0x850(%rbp)
140a8d75f: 0f 29 8d 40 08 00 00        	movaps	%xmm1, 0x840(%rbp)
140a8d766: 48 c7 85 70 08 00 00 00 00 00 00    	movq	$0x0, 0x870(%rbp)
140a8d771: 0f 29 8d f0 0b 00 00        	movaps	%xmm1, 0xbf0(%rbp)
140a8d778: 48 c7 85 d8 0c 00 00 00 00 00 00    	movq	$0x0, 0xcd8(%rbp)
140a8d783: b9 04 00 00 00              	movl	$0x4, %ecx
140a8d788: 48 89 8d d0 0c 00 00        	movq	%rcx, 0xcd0(%rbp)
140a8d78f: 48 c7 85 28 0c 00 00 00 00 00 00    	movq	$0x0, 0xc28(%rbp)
140a8d79a: c7 85 c8 0c 00 00 00 00 00 00       	movl	$0x0, 0xcc8(%rbp)
140a8d7a4: 45 0f 28 c7                 	movaps	%xmm15, %xmm8
140a8d7a8: 41 0f 28 df                 	movaps	%xmm15, %xmm3
140a8d7ac: 80 bd 98 0c 00 00 00        	cmpb	$0x0, 0xc98(%rbp)
140a8d7b3: 75 03                       	jne	0x140a8d7b8 <.text+0xa8c7b8>
140a8d7b5: 0f 28 de                    	movaps	%xmm6, %xmm3
140a8d7b8: 44 0f 28 fe                 	movaps	%xmm6, %xmm15
140a8d7bc: 41 0f 28 f3                 	movaps	%xmm11, %xmm6
140a8d7c0: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
140a8d7c4: 75 08                       	jne	0x140a8d7ce <.text+0xa8c7ce>
140a8d7c6: f3 0f 10 15 32 06 ff 01     	movss	0x1ff0632(%rip), %xmm2  # 0x142a7de00
140a8d7ce: 48 83 bd 88 0b 00 00 00     	cmpq	$0x0, 0xb88(%rbp)
140a8d7d6: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a8d7d9: 4c 8d 85 08 07 00 00        	leaq	0x708(%rbp), %r8
140a8d7e0: 41 0f 11 48 30              	movups	%xmm1, 0x30(%r8)
140a8d7e5: 41 0f 11 48 20              	movups	%xmm1, 0x20(%r8)
140a8d7ea: 41 0f 11 48 10              	movups	%xmm1, 0x10(%r8)
140a8d7ef: 41 0f 11 08                 	movups	%xmm1, (%r8)
140a8d7f3: 49 c7 40 40 00 00 00 00     	movq	$0x0, 0x40(%r8)
140a8d7fb: 48 8b 8d 18 0d 00 00        	movq	0xd18(%rbp), %rcx
140a8d802: 48 89 8d 70 05 00 00        	movq	%rcx, 0x570(%rbp)
140a8d809: 48 8b 95 38 0d 00 00        	movq	0xd38(%rbp), %rdx
140a8d810: 48 89 95 78 05 00 00        	movq	%rdx, 0x578(%rbp)
140a8d817: 48 89 8d 80 05 00 00        	movq	%rcx, 0x580(%rbp)
140a8d81e: 4c 8b 9d 40 0d 00 00        	movq	0xd40(%rbp), %r11
140a8d825: f3 41 0f 10 63 e8           	movss	-0x18(%r11), %xmm4
140a8d82b: f3 41 0f 10 6b ec           	movss	-0x14(%r11), %xmm5
140a8d831: 66 41 0f 6e 4b f0           	movd	-0x10(%r11), %xmm1
140a8d837: 41 0f b6 4b f4              	movzbl	-0xc(%r11), %ecx
140a8d83c: 41 0f b6 53 f6              	movzbl	-0xa(%r11), %edx
140a8d841: 45 0f b6 53 f7              	movzbl	-0x9(%r11), %r10d
140a8d846: 45 0f b7 5b f8              	movzwl	-0x8(%r11), %r11d
140a8d84b: 4c 8b 8d a0 0c 00 00        	movq	0xca0(%rbp), %r9
140a8d852: 4c 89 8d 00 05 00 00        	movq	%r9, 0x500(%rbp)
140a8d859: 4c 8b 8d 20 0c 00 00        	movq	0xc20(%rbp), %r9
140a8d860: 4c 89 8d 08 05 00 00        	movq	%r9, 0x508(%rbp)
140a8d867: 4c 8b 8d 08 0d 00 00        	movq	0xd08(%rbp), %r9
140a8d86e: 4c 89 8d 10 05 00 00        	movq	%r9, 0x510(%rbp)
140a8d875: 4c 8b 8d a8 0c 00 00        	movq	0xca8(%rbp), %r9
140a8d87c: 4c 89 8d 18 05 00 00        	movq	%r9, 0x518(%rbp)
140a8d883: 4c 8b 8d 80 0c 00 00        	movq	0xc80(%rbp), %r9
140a8d88a: 4c 89 8d 20 05 00 00        	movq	%r9, 0x520(%rbp)
140a8d891: 48 89 b5 28 05 00 00        	movq	%rsi, 0x528(%rbp)
140a8d898: 66 0f 7e bd 30 05 00 00     	movd	%xmm7, 0x530(%rbp)
140a8d8a0: 48 8b b5 90 02 00 00        	movq	0x290(%rbp), %rsi
140a8d8a7: 49 89 b0 60 fe ff ff        	movq	%rsi, -0x1a0(%r8)
140a8d8ae: f3 41 0f 5f dc              	maxss	%xmm12, %xmm3
140a8d8b3: f3 0f 5d d3                 	minss	%xmm3, %xmm2
140a8d8b7: 0f 10 9d 60 02 00 00        	movups	0x260(%rbp), %xmm3
140a8d8be: 44 0f 10 9d 70 02 00 00     	movups	0x270(%rbp), %xmm11
140a8d8c6: 44 0f 10 a5 80 02 00 00     	movups	0x280(%rbp), %xmm12
140a8d8ce: 45 0f 11 a0 50 fe ff ff     	movups	%xmm12, -0x1b0(%r8)
140a8d8d6: 45 0f 11 98 40 fe ff ff     	movups	%xmm11, -0x1c0(%r8)
140a8d8de: 41 0f 11 98 30 fe ff ff     	movups	%xmm3, -0x1d0(%r8)
140a8d8e6: f3 44 0f 11 95 74 07 00 00  	movss	%xmm10, 0x774(%rbp)
140a8d8ef: 0f 57 db                    	xorps	%xmm3, %xmm3
140a8d8f2: f3 44 0f 11 ad 78 07 00 00  	movss	%xmm13, 0x778(%rbp)
140a8d8fb: f3 44 0f 11 8d 7c 07 00 00  	movss	%xmm9, 0x77c(%rbp)
140a8d904: f3 0f 11 a5 80 07 00 00     	movss	%xmm4, 0x780(%rbp)
140a8d90c: f3 0f 11 95 84 07 00 00     	movss	%xmm2, 0x784(%rbp)
140a8d914: f3 0f 11 ad 88 07 00 00     	movss	%xmm5, 0x788(%rbp)
140a8d91c: 88 8d 94 07 00 00           	movb	%cl, 0x794(%rbp)
140a8d922: 88 95 95 07 00 00           	movb	%dl, 0x795(%rbp)
140a8d928: 48 c7 85 88 05 00 00 00 00 00 00    	movq	$0x0, 0x588(%rbp)
140a8d933: 48 c7 85 90 05 00 00 04 00 00 00    	movq	$0x4, 0x590(%rbp)
140a8d93e: 41 0f 11 98 90 fe ff ff     	movups	%xmm3, -0x170(%r8)
140a8d946: 48 c7 85 a8 05 00 00 04 00 00 00    	movq	$0x4, 0x5a8(%rbp)
140a8d951: 48 c7 85 b0 05 00 00 00 00 00 00    	movq	$0x0, 0x5b0(%rbp)
140a8d95c: c7 85 b8 05 00 00 00 00 00 00       	movl	$0x0, 0x5b8(%rbp)
140a8d966: 48 c7 85 c0 05 00 00 00 00 00 00    	movq	$0x0, 0x5c0(%rbp)
140a8d971: 48 c7 85 c8 05 00 00 04 00 00 00    	movq	$0x4, 0x5c8(%rbp)
140a8d97c: 48 c7 85 d0 05 00 00 00 00 00 00    	movq	$0x0, 0x5d0(%rbp)
140a8d987: f3 0f 11 85 8c 07 00 00     	movss	%xmm0, 0x78c(%rbp)
140a8d98f: 89 85 58 07 00 00           	movl	%eax, 0x758(%rbp)
140a8d995: 48 8b 85 50 0c 00 00        	movq	0xc50(%rbp), %rax
140a8d99c: 49 89 40 64                 	movq	%rax, 0x64(%r8)
140a8d9a0: 0f 28 85 40 0c 00 00        	movaps	0xc40(%rbp), %xmm0
140a8d9a7: 41 0f 11 40 54              	movups	%xmm0, 0x54(%r8)
140a8d9ac: 48 8b 85 78 0c 00 00        	movq	0xc78(%rbp), %rax
140a8d9b3: 48 89 85 18 06 00 00        	movq	%rax, 0x618(%rbp)
140a8d9ba: 48 8b 85 90 0b 00 00        	movq	0xb90(%rbp), %rax
140a8d9c1: 48 89 85 20 06 00 00        	movq	%rax, 0x620(%rbp)
140a8d9c8: 48 8b 85 d0 0b 00 00        	movq	0xbd0(%rbp), %rax
140a8d9cf: 48 89 85 28 06 00 00        	movq	%rax, 0x628(%rbp)
140a8d9d6: 48 8b 85 c8 0b 00 00        	movq	0xbc8(%rbp), %rax
140a8d9dd: 48 89 85 30 06 00 00        	movq	%rax, 0x630(%rbp)
140a8d9e4: 48 8b 85 e0 0b 00 00        	movq	0xbe0(%rbp), %rax
140a8d9eb: 48 89 85 38 06 00 00        	movq	%rax, 0x638(%rbp)
140a8d9f2: 48 8b 85 d8 0b 00 00        	movq	0xbd8(%rbp), %rax
140a8d9f9: 48 89 85 40 06 00 00        	movq	%rax, 0x640(%rbp)
140a8da00: 48 8b 85 98 0b 00 00        	movq	0xb98(%rbp), %rax
140a8da07: 48 89 85 48 06 00 00        	movq	%rax, 0x648(%rbp)
140a8da0e: 48 8b 85 c0 0b 00 00        	movq	0xbc0(%rbp), %rax
140a8da15: 48 89 85 50 06 00 00        	movq	%rax, 0x650(%rbp)
140a8da1c: 48 8b 85 a8 0b 00 00        	movq	0xba8(%rbp), %rax
140a8da23: 48 89 85 58 06 00 00        	movq	%rax, 0x658(%rbp)
140a8da2a: 48 8b 85 a0 0b 00 00        	movq	0xba0(%rbp), %rax
140a8da31: 48 89 85 60 06 00 00        	movq	%rax, 0x660(%rbp)
140a8da38: 48 8b 85 b8 0b 00 00        	movq	0xbb8(%rbp), %rax
140a8da3f: 48 89 85 68 06 00 00        	movq	%rax, 0x668(%rbp)
140a8da46: 48 8b 85 b0 0b 00 00        	movq	0xbb0(%rbp), %rax
140a8da4d: 48 89 85 70 06 00 00        	movq	%rax, 0x670(%rbp)
140a8da54: 48 89 bd 78 06 00 00        	movq	%rdi, 0x678(%rbp)
140a8da5b: 48 8b 85 e8 0b 00 00        	movq	0xbe8(%rbp), %rax
140a8da62: 48 89 85 80 06 00 00        	movq	%rax, 0x680(%rbp)
140a8da69: 48 c7 85 d8 05 00 00 00 00 00 00    	movq	$0x0, 0x5d8(%rbp)
140a8da74: 48 c7 85 e0 05 00 00 04 00 00 00    	movq	$0x4, 0x5e0(%rbp)
140a8da7f: 41 0f 11 98 e0 fe ff ff     	movups	%xmm3, -0x120(%r8)
140a8da87: 48 c7 85 f8 05 00 00 04 00 00 00    	movq	$0x4, 0x5f8(%rbp)
140a8da92: 48 c7 85 00 06 00 00 00 00 00 00    	movq	$0x0, 0x600(%rbp)
140a8da9d: 48 8b 85 d8 0c 00 00        	movq	0xcd8(%rbp), %rax
140a8daa4: 88 85 88 06 00 00           	movb	%al, 0x688(%rbp)
140a8daaa: 8b 85 b3 0c 00 00           	movl	0xcb3(%rbp), %eax
140a8dab0: 41 89 40 84                 	movl	%eax, -0x7c(%r8)
140a8dab4: 8b 85 b0 0c 00 00           	movl	0xcb0(%rbp), %eax
140a8daba: 41 89 40 81                 	movl	%eax, -0x7f(%r8)
140a8dabe: 48 8b 85 28 0c 00 00        	movq	0xc28(%rbp), %rax
140a8dac5: 48 89 85 90 06 00 00        	movq	%rax, 0x690(%rbp)
140a8dacc: 48 8b 85 d0 0c 00 00        	movq	0xcd0(%rbp), %rax
140a8dad3: 48 89 85 98 06 00 00        	movq	%rax, 0x698(%rbp)
140a8dada: 0f 28 85 f0 0b 00 00        	movaps	0xbf0(%rbp), %xmm0
140a8dae1: 41 0f 11 40 98              	movups	%xmm0, -0x68(%r8)
140a8dae6: 48 8b 85 70 08 00 00        	movq	0x870(%rbp), %rax
140a8daed: 49 89 40 d8                 	movq	%rax, -0x28(%r8)
140a8daf1: 66 0f 6f 85 40 08 00 00     	movdqa	0x840(%rbp), %xmm0
140a8daf9: 66 0f 6f 95 50 08 00 00     	movdqa	0x850(%rbp), %xmm2
140a8db01: 0f 28 9d 60 08 00 00        	movaps	0x860(%rbp), %xmm3
140a8db08: 41 0f 11 58 c8              	movups	%xmm3, -0x38(%r8)
140a8db0d: f3 41 0f 7f 50 b8           	movdqu	%xmm2, -0x48(%r8)
140a8db13: f3 41 0f 7f 40 a8           	movdqu	%xmm0, -0x58(%r8)
140a8db19: 8b 85 c8 0c 00 00           	movl	0xcc8(%rbp), %eax
140a8db1f: 89 85 e8 06 00 00           	movl	%eax, 0x6e8(%rbp)
140a8db25: 8b 85 64 0c 00 00           	movl	0xc64(%rbp), %eax
140a8db2b: 89 85 ec 06 00 00           	movl	%eax, 0x6ec(%rbp)
140a8db31: c6 85 f0 06 00 00 00        	movb	$0x0, 0x6f0(%rbp)
140a8db38: 48 c7 85 f8 06 00 00 00 00 00 00    	movq	$0x0, 0x6f8(%rbp)
140a8db43: 48 c7 85 00 07 00 00 04 00 00 00    	movq	$0x4, 0x700(%rbp)
140a8db4e: c7 85 50 07 00 00 00 00 00 00       	movl	$0x0, 0x750(%rbp)
140a8db58: c7 85 08 06 00 00 00 00 00 00       	movl	$0x0, 0x608(%rbp)
140a8db62: c7 85 10 06 00 00 00 00 00 00       	movl	$0x0, 0x610(%rbp)
140a8db6c: 66 44 89 9d 99 07 00 00     	movw	%r11w, 0x799(%rbp)
140a8db74: 0f b6 85 98 0c 00 00        	movzbl	0xc98(%rbp), %eax
140a8db7b: 88 85 96 07 00 00           	movb	%al, 0x796(%rbp)
140a8db81: 0f 95 85 97 07 00 00        	setne	0x797(%rbp)
140a8db88: 66 0f 7e 8d 90 07 00 00     	movd	%xmm1, 0x790(%rbp)
140a8db90: 44 88 95 98 07 00 00        	movb	%r10b, 0x798(%rbp)
140a8db97: 48 8b 85 68 0b 00 00        	movq	0xb68(%rbp), %rax
140a8db9e: 80 b8 f4 00 00 00 01        	cmpb	$0x1, 0xf4(%rax)
140a8dba5: 0f 85 9c 00 00 00           	jne	0x140a8dc47 <.text+0xa8cc47>
140a8dbab: 48 8b 9d 48 05 00 00        	movq	0x548(%rbp), %rbx
140a8dbb2: 48 89 d8                    	movq	%rbx, %rax
140a8dbb5: b9 0c 00 00 00              	movl	$0xc, %ecx
140a8dbba: 48 f7 e1                    	mulq	%rcx
140a8dbbd: 49 89 c1                    	movq	%rax, %r9
140a8dbc0: 48 8b bd f8 0c 00 00        	movq	0xcf8(%rbp), %rdi
140a8dbc7: 0f 80 22 06 00 00           	jo	0x140a8e1ef <.text+0xa8d1ef>
140a8dbcd: 44 0f 28 de                 	movaps	%xmm6, %xmm11
140a8dbd1: b9 00 00 00 00              	movl	$0x0, %ecx
140a8dbd6: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8dbe0: 49 39 c1                    	cmpq	%rax, %r9
140a8dbe3: 41 0f 28 f7                 	movaps	%xmm15, %xmm6
140a8dbe7: f3 44 0f 10 15 10 6a ea 01  	movss	0x1ea6a10(%rip), %xmm10 # 0x142934600
140a8dbf0: 45 0f 28 f8                 	movaps	%xmm8, %xmm15
140a8dbf4: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
140a8dbf8: 66 0f 76 ff                 	pcmpeqd	%xmm7, %xmm7
140a8dbfc: 0f 87 ef 05 00 00           	ja	0x140a8e1f1 <.text+0xa8d1f1>
140a8dc02: 48 8b b5 40 05 00 00        	movq	0x540(%rbp), %rsi
140a8dc09: 4d 85 c9                    	testq	%r9, %r9
140a8dc0c: 74 62                       	je	0x140a8dc70 <.text+0xa8cc70>
140a8dc0e: 0f b6 05 f4 6f ba 02        	movzbl	0x2ba6ff4(%rip), %eax   # 0x143634c09
140a8dc15: ba 04 00 00 00              	movl	$0x4, %edx
140a8dc1a: 4c 89 c9                    	movq	%r9, %rcx
140a8dc1d: 4c 89 8d 40 0d 00 00        	movq	%r9, 0xd40(%rbp)
140a8dc24: e8 e7 5f b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a8dc29: b9 04 00 00 00              	movl	$0x4, %ecx
140a8dc2e: 4c 8b 8d 40 0d 00 00        	movq	0xd40(%rbp), %r9
140a8dc35: 48 85 c0                    	testq	%rax, %rax
140a8dc38: 0f 84 b3 05 00 00           	je	0x140a8e1f1 <.text+0xa8d1f1>
140a8dc3e: 48 89 9d 38 0d 00 00        	movq	%rbx, 0xd38(%rbp)
140a8dc45: eb 39                       	jmp	0x140a8dc80 <.text+0xa8cc80>
140a8dc47: 48 8b bd f8 0c 00 00        	movq	0xcf8(%rbp), %rdi
140a8dc4e: 44 0f 28 de                 	movaps	%xmm6, %xmm11
140a8dc52: 41 0f 28 f7                 	movaps	%xmm15, %xmm6
140a8dc56: f3 44 0f 10 15 a1 69 ea 01  	movss	0x1ea69a1(%rip), %xmm10 # 0x142934600
140a8dc5f: 45 0f 28 f8                 	movaps	%xmm8, %xmm15
140a8dc63: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
140a8dc67: 66 0f 76 ff                 	pcmpeqd	%xmm7, %xmm7
140a8dc6b: e9 96 00 00 00              	jmp	0x140a8dd06 <.text+0xa8cd06>
140a8dc70: b8 04 00 00 00              	movl	$0x4, %eax
140a8dc75: 48 c7 85 38 0d 00 00 00 00 00 00    	movq	$0x0, 0xd38(%rbp)
140a8dc80: 48 89 c1                    	movq	%rax, %rcx
140a8dc83: 48 89 f2                    	movq	%rsi, %rdx
140a8dc86: 4d 89 c8                    	movq	%r9, %r8
140a8dc89: 48 89 c6                    	movq	%rax, %rsi
140a8dc8c: e8 6f 5f e9 01              	callq	0x142923c00 <.text+0x2922c00>
140a8dc91: 8b 8d 68 0c 00 00           	movl	0xc68(%rbp), %ecx
140a8dc97: 81 e1 00 00 ff ff           	andl	$0xffff0000, %ecx       # imm = 0xFFFF0000
140a8dc9d: ff c1                       	incl	%ecx
140a8dc9f: 48 8b 85 38 0d 00 00        	movq	0xd38(%rbp), %rax
140a8dca6: 48 89 85 40 08 00 00        	movq	%rax, 0x840(%rbp)
140a8dcad: 48 89 b5 48 08 00 00        	movq	%rsi, 0x848(%rbp)
140a8dcb4: 48 89 9d 50 08 00 00        	movq	%rbx, 0x850(%rbp)
140a8dcbb: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a8dcc5: 48 89 85 58 08 00 00        	movq	%rax, 0x858(%rbp)
140a8dccc: c7 85 70 08 00 00 01 00 00 00       	movl	$0x1, 0x870(%rbp)
140a8dcd6: c7 85 80 08 00 00 00 00 40 40       	movl	$0x40400000, 0x880(%rbp) # imm = 0x40400000
140a8dce0: 89 8d 68 0c 00 00           	movl	%ecx, 0xc68(%rbp)
140a8dce6: 89 8d 84 08 00 00           	movl	%ecx, 0x884(%rbp)
140a8dcec: c6 85 56 0d 00 00 01        	movb	$0x1, 0xd56(%rbp)
140a8dcf3: 48 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %rcx
140a8dcfa: 48 8b 95 78 0b 00 00        	movq	0xb78(%rbp), %rdx
140a8dd01: e8 6a 72 07 00              	callq	0x140b04f70 <.text+0xb03f70>
140a8dd06: 41 b8 a0 02 00 00           	movl	$0x2a0, %r8d            # imm = 0x2A0
140a8dd0c: 48 8d b5 40 08 00 00        	leaq	0x840(%rbp), %rsi
140a8dd13: 48 89 f1                    	movq	%rsi, %rcx
140a8dd16: 48 8d 95 00 05 00 00        	leaq	0x500(%rbp), %rdx
140a8dd1d: e8 de 5e e9 01              	callq	0x142923c00 <.text+0x2922c00>
140a8dd22: 4c 8b 85 b8 0c 00 00        	movq	0xcb8(%rbp), %r8
140a8dd29: c6 85 56 0d 00 00 00        	movb	$0x0, 0xd56(%rbp)
140a8dd30: 48 8d 8d 60 02 00 00        	leaq	0x260(%rbp), %rcx
140a8dd37: 48 89 fa                    	movq	%rdi, %rdx
140a8dd3a: 49 89 f1                    	movq	%rsi, %r9
140a8dd3d: e8 1e c6 a9 00              	callq	0x14152a360 <.text+0x1529360>
140a8dd42: 31 c0                       	xorl	%eax, %eax
140a8dd44: 48 3b 85 60 02 00 00        	cmpq	0x260(%rbp), %rax
140a8dd4b: 70 13                       	jo	0x140a8dd60 <.text+0xa8cd60>
140a8dd4d: c6 85 56 0d 00 00 00        	movb	$0x0, 0xd56(%rbp)
140a8dd54: 48 8d 8d 60 02 00 00        	leaq	0x260(%rbp), %rcx
140a8dd5b: e8 a0 78 e0 ff              	callq	0x140895600 <.text+0x894600>
140a8dd60: 31 c0                       	xorl	%eax, %eax
140a8dd62: 48 3b 45 c0                 	cmpq	-0x40(%rbp), %rax
140a8dd66: 70 12                       	jo	0x140a8dd7a <.text+0xa8cd7a>
140a8dd68: 80 bd 28 0d 00 00 00        	cmpb	$0x0, 0xd28(%rbp)
140a8dd6f: 74 09                       	je	0x140a8dd7a <.text+0xa8cd7a>
140a8dd71: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140a8dd75: e8 86 78 e0 ff              	callq	0x140895600 <.text+0x894600>
140a8dd7a: 4c 8b 85 20 0d 00 00        	movq	0xd20(%rbp), %r8
140a8dd81: e9 8d e7 ff ff              	jmp	0x140a8c513 <.text+0xa8b513>
140a8dd86: 48 8d 0d 2b b5 04 02        	leaq	0x204b52b(%rip), %rcx   # 0x142ad92b8
140a8dd8d: e8 7e b5 e4 01              	callq	0x1428d9310 <.text+0x28d8310>
140a8dd92: e9 70 04 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8dd97: 48 85 c0                    	testq	%rax, %rax
140a8dd9a: 75 13                       	jne	0x140a8ddaf <.text+0xa8cdaf>
140a8dd9c: 31 d2                       	xorl	%edx, %edx
140a8dd9e: e8 2d aa e6 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a8dda3: 48 89 c1                    	movq	%rax, %rcx
140a8dda6: 48 85 c0                    	testq	%rax, %rax
140a8dda9: 0f 85 f7 d9 ff ff           	jne	0x140a8b7a6 <.text+0xa8a7a6>
140a8ddaf: 48 8d 0d ea cc 1a 02        	leaq	0x21accea(%rip), %rcx   # 0x142c3aaa0
140a8ddb6: e8 15 a3 e4 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a8ddbb: 48 83 7f 48 00              	cmpq	$0x0, 0x48(%rdi)
140a8ddc0: 74 3e                       	je	0x140a8de00 <.text+0xa8ce00>
140a8ddc2: 48 8b 77 38                 	movq	0x38(%rdi), %rsi
140a8ddc6: 48 85 f6                    	testq	%rsi, %rsi
140a8ddc9: 74 0f                       	je	0x140a8ddda <.text+0xa8cdda>
140a8ddcb: 48 8b 4f 30                 	movq	0x30(%rdi), %rcx
140a8ddcf: 4c 8d 46 11                 	leaq	0x11(%rsi), %r8
140a8ddd3: b2 ff                       	movb	$-0x1, %dl
140a8ddd5: e8 46 5e e9 01              	callq	0x142923c20 <.text+0x2922c20>
140a8ddda: 48 c7 47 48 00 00 00 00     	movq	$0x0, 0x48(%rdi)
140a8dde2: 48 8d 46 01                 	leaq	0x1(%rsi), %rax
140a8dde6: 48 89 c1                    	movq	%rax, %rcx
140a8dde9: 48 c1 e9 03                 	shrq	$0x3, %rcx
140a8dded: 48 83 e0 f8                 	andq	$-0x8, %rax
140a8ddf1: 48 29 c8                    	subq	%rcx, %rax
140a8ddf4: 48 83 fe 08                 	cmpq	$0x8, %rsi
140a8ddf8: 48 0f 42 c6                 	cmovbq	%rsi, %rax
140a8ddfc: 48 89 47 40                 	movq	%rax, 0x40(%rdi)
140a8de00: 48 83 7f 78 00              	cmpq	$0x0, 0x78(%rdi)
140a8de05: 74 44                       	je	0x140a8de4b <.text+0xa8ce4b>
140a8de07: 48 8b 77 68                 	movq	0x68(%rdi), %rsi
140a8de0b: 48 85 f6                    	testq	%rsi, %rsi
140a8de0e: 74 15                       	je	0x140a8de25 <.text+0xa8ce25>
140a8de10: 48 8b 85 f0 0c 00 00        	movq	0xcf0(%rbp), %rax
140a8de17: 48 8b 08                    	movq	(%rax), %rcx
140a8de1a: 4c 8d 46 11                 	leaq	0x11(%rsi), %r8
140a8de1e: b2 ff                       	movb	$-0x1, %dl
140a8de20: e8 fb 5d e9 01              	callq	0x142923c20 <.text+0x2922c20>
140a8de25: 48 c7 47 78 00 00 00 00     	movq	$0x0, 0x78(%rdi)
140a8de2d: 48 8d 46 01                 	leaq	0x1(%rsi), %rax
140a8de31: 48 89 c1                    	movq	%rax, %rcx
140a8de34: 48 c1 e9 03                 	shrq	$0x3, %rcx
140a8de38: 48 83 e0 f8                 	andq	$-0x8, %rax
140a8de3c: 48 29 c8                    	subq	%rcx, %rax
140a8de3f: 48 83 fe 08                 	cmpq	$0x8, %rsi
140a8de43: 48 0f 42 c6                 	cmovbq	%rsi, %rax
140a8de47: 48 89 47 70                 	movq	%rax, 0x70(%rdi)
140a8de4b: 48 83 bf a8 00 00 00 00     	cmpq	$0x0, 0xa8(%rdi)
140a8de53: 74 4d                       	je	0x140a8dea2 <.text+0xa8cea2>
140a8de55: 48 8b b7 98 00 00 00        	movq	0x98(%rdi), %rsi
140a8de5c: 48 85 f6                    	testq	%rsi, %rsi
140a8de5f: 74 15                       	je	0x140a8de76 <.text+0xa8ce76>
140a8de61: 48 8b 85 80 0b 00 00        	movq	0xb80(%rbp), %rax
140a8de68: 48 8b 08                    	movq	(%rax), %rcx
140a8de6b: 4c 8d 46 11                 	leaq	0x11(%rsi), %r8
140a8de6f: b2 ff                       	movb	$-0x1, %dl
140a8de71: e8 aa 5d e9 01              	callq	0x142923c20 <.text+0x2922c20>
140a8de76: 48 c7 87 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rdi)
140a8de81: 48 8d 46 01                 	leaq	0x1(%rsi), %rax
140a8de85: 48 89 c1                    	movq	%rax, %rcx
140a8de88: 48 c1 e9 03                 	shrq	$0x3, %rcx
140a8de8c: 48 83 e0 f8                 	andq	$-0x8, %rax
140a8de90: 48 29 c8                    	subq	%rcx, %rax
140a8de93: 48 83 fe 08                 	cmpq	$0x8, %rsi
140a8de97: 48 0f 42 c6                 	cmovbq	%rsi, %rax
140a8de9b: 48 89 87 a0 00 00 00        	movq	%rax, 0xa0(%rdi)
140a8dea2: 48 83 bd 00 0c 00 00 00     	cmpq	$0x0, 0xc00(%rbp)
140a8deaa: 74 4b                       	je	0x140a8def7 <.text+0xa8cef7>
140a8deac: e8 7f a2 18 00              	callq	0x140c18130 <.text+0xc17130>
140a8deb1: 48 89 c1                    	movq	%rax, %rcx
140a8deb4: 48 8b 00                    	movq	(%rax), %rax
140a8deb7: 48 83 f8 01                 	cmpq	$0x1, %rax
140a8debb: 0f 85 b5 00 00 00           	jne	0x140a8df76 <.text+0xa8cf76>
140a8dec1: 48 83 c1 08                 	addq	$0x8, %rcx
140a8dec5: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a8dec9: 0f 85 c8 00 00 00           	jne	0x140a8df97 <.text+0xa8cf97>
140a8decf: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a8ded6: 48 89 8d 48 0d 00 00        	movq	%rcx, 0xd48(%rbp)
140a8dedd: 48 83 c1 08                 	addq	$0x8, %rcx
140a8dee1: 48 8b 95 08 0c 00 00        	movq	0xc08(%rbp), %rdx
140a8dee8: e8 03 92 18 00              	callq	0x140c170f0 <.text+0xc160f0>
140a8deed: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8def4: 48 ff 00                    	incq	(%rax)
140a8def7: 0f 28 b5 60 0d 00 00        	movaps	0xd60(%rbp), %xmm6
140a8defe: 0f 28 bd 70 0d 00 00        	movaps	0xd70(%rbp), %xmm7
140a8df05: 44 0f 28 85 80 0d 00 00     	movaps	0xd80(%rbp), %xmm8
140a8df0d: 44 0f 28 8d 90 0d 00 00     	movaps	0xd90(%rbp), %xmm9
140a8df15: 44 0f 28 95 a0 0d 00 00     	movaps	0xda0(%rbp), %xmm10
140a8df1d: 44 0f 28 9d b0 0d 00 00     	movaps	0xdb0(%rbp), %xmm11
140a8df25: 44 0f 28 a5 c0 0d 00 00     	movaps	0xdc0(%rbp), %xmm12
140a8df2d: 44 0f 28 ad d0 0d 00 00     	movaps	0xdd0(%rbp), %xmm13
140a8df35: 44 0f 28 b5 e0 0d 00 00     	movaps	0xde0(%rbp), %xmm14
140a8df3d: 44 0f 28 bd f0 0d 00 00     	movaps	0xdf0(%rbp), %xmm15
140a8df45: 48 81 c4 88 0e 00 00        	addq	$0xe88, %rsp            # imm = 0xE88
140a8df4c: 5b                          	popq	%rbx
140a8df4d: 5f                          	popq	%rdi
140a8df4e: 5e                          	popq	%rsi
140a8df4f: 41 5c                       	popq	%r12
140a8df51: 41 5d                       	popq	%r13
140a8df53: 41 5e                       	popq	%r14
140a8df55: 41 5f                       	popq	%r15
140a8df57: 5d                          	popq	%rbp
140a8df58: c3                          	retq
140a8df59: 48 8d 0d 33 c2 04 02        	leaq	0x204c233(%rip), %rcx   # 0x142ada193
140a8df60: 4c 8d 05 a1 b3 04 02        	leaq	0x204b3a1(%rip), %r8    # 0x142ad9308
140a8df67: ba 32 00 00 00              	movl	$0x32, %edx
140a8df6c: e8 bf b4 e4 01              	callq	0x1428d9430 <.text+0x28d8430>
140a8df71: e9 91 02 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8df76: 48 85 c0                    	testq	%rax, %rax
140a8df79: 0f 85 30 fe ff ff           	jne	0x140a8ddaf <.text+0xa8cdaf>
140a8df7f: 31 d2                       	xorl	%edx, %edx
140a8df81: e8 4a a8 e6 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a8df86: 48 89 c1                    	movq	%rax, %rcx
140a8df89: 48 85 c0                    	testq	%rax, %rax
140a8df8c: 0f 85 33 ff ff ff           	jne	0x140a8dec5 <.text+0xa8cec5>
140a8df92: e9 18 fe ff ff              	jmp	0x140a8ddaf <.text+0xa8cdaf>
140a8df97: 48 8d 0d d2 cb 1a 02        	leaq	0x21acbd2(%rip), %rcx   # 0x142c3ab70
140a8df9e: e8 ad b2 e4 01              	callq	0x1428d9250 <.text+0x28d8250>
140a8dfa3: 48 8d 0d 96 33 06 02        	leaq	0x2063396(%rip), %rcx   # 0x142af1340
140a8dfaa: 48 8d 15 ef 33 06 02        	leaq	0x20633ef(%rip), %rdx   # 0x142af13a0
140a8dfb1: e8 4a ba e1 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140a8dfb6: e9 4c 02 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8dfbb: 48 8d 0d 7e 33 06 02        	leaq	0x206337e(%rip), %rcx   # 0x142af1340
140a8dfc2: 48 8d 15 d7 33 06 02        	leaq	0x20633d7(%rip), %rdx   # 0x142af13a0
140a8dfc9: e8 32 ba e1 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140a8dfce: e9 34 02 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8dfd3: 48 8d 0d 66 33 06 02        	leaq	0x2063366(%rip), %rcx   # 0x142af1340
140a8dfda: 48 8d 15 bf 33 06 02        	leaq	0x20633bf(%rip), %rdx   # 0x142af13a0
140a8dfe1: e8 1a ba e1 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140a8dfe6: e9 1c 02 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8dfeb: 48 8d 0d 4e 33 06 02        	leaq	0x206334e(%rip), %rcx   # 0x142af1340
140a8dff2: 48 8d 15 a7 33 06 02        	leaq	0x20633a7(%rip), %rdx   # 0x142af13a0
140a8dff9: e8 02 ba e1 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140a8dffe: e9 04 02 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e003: 48 8d 0d 36 33 06 02        	leaq	0x2063336(%rip), %rcx   # 0x142af1340
140a8e00a: 48 8d 15 8f 33 06 02        	leaq	0x206338f(%rip), %rdx   # 0x142af13a0
140a8e011: e8 ea b9 e1 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140a8e016: e9 ec 01 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e01b: 48 8d 0d 1e 33 06 02        	leaq	0x206331e(%rip), %rcx   # 0x142af1340
140a8e022: 48 8d 15 77 33 06 02        	leaq	0x2063377(%rip), %rdx   # 0x142af13a0
140a8e029: e8 d2 b9 e1 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140a8e02e: e9 d4 01 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e033: 48 8d 0d 06 33 06 02        	leaq	0x2063306(%rip), %rcx   # 0x142af1340
140a8e03a: 48 8d 15 5f 33 06 02        	leaq	0x206335f(%rip), %rdx   # 0x142af13a0
140a8e041: e8 ba b9 e1 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140a8e046: e9 bc 01 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e04b: 48 c7 85 40 08 00 00 00 00 00 00    	movq	$0x0, 0x840(%rbp)
140a8e056: 48 8d 05 23 01 1a 02        	leaq	0x21a0123(%rip), %rax   # 0x142c2e180
140a8e05d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a8e062: 48 8d 95 40 0c 00 00        	leaq	0xc40(%rbp), %rdx
140a8e069: 4c 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %r9
140a8e070: 31 c9                       	xorl	%ecx, %ecx
140a8e072: 49 89 f8                    	movq	%rdi, %r8
140a8e075: e8 c7 ca e3 01              	callq	0x1428cab41 <.text+0x28c9b41>
140a8e07a: e9 88 01 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e07f: 48 c7 85 40 08 00 00 00 00 00 00    	movq	$0x0, 0x840(%rbp)
140a8e08a: 48 8d 05 ef 00 1a 02        	leaq	0x21a00ef(%rip), %rax   # 0x142c2e180
140a8e091: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a8e096: 48 8d 95 40 0c 00 00        	leaq	0xc40(%rbp), %rdx
140a8e09d: 4c 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %r9
140a8e0a4: 31 c9                       	xorl	%ecx, %ecx
140a8e0a6: e8 96 ca e3 01              	callq	0x1428cab41 <.text+0x28c9b41>
140a8e0ab: e9 57 01 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e0b0: 48 8d 0d 19 b2 04 02        	leaq	0x204b219(%rip), %rcx   # 0x142ad92d0
140a8e0b7: 4c 8d 05 32 b2 04 02        	leaq	0x204b232(%rip), %r8    # 0x142ad92f0
140a8e0be: ba 1c 00 00 00              	movl	$0x1c, %edx
140a8e0c3: e8 68 b3 e4 01              	callq	0x1428d9430 <.text+0x28d8430>
140a8e0c8: e9 3a 01 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e0cd: 48 8d 0d cc c9 1a 02        	leaq	0x21ac9cc(%rip), %rcx   # 0x142c3aaa0
140a8e0d4: e8 f7 9f e4 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a8e0d9: e9 29 01 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e0de: 48 8d 0d 53 b2 04 02        	leaq	0x204b253(%rip), %rcx   # 0x142ad9338
140a8e0e5: e8 26 b2 e4 01              	callq	0x1428d9310 <.text+0x28d8310>
140a8e0ea: e9 18 01 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e0ef: 48 8d 0d 5a b2 04 02        	leaq	0x204b25a(%rip), %rcx   # 0x142ad9350
140a8e0f6: e8 15 b2 e4 01              	callq	0x1428d9310 <.text+0x28d8310>
140a8e0fb: e9 07 01 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e100: b9 08 00 00 00              	movl	$0x8, %ecx
140a8e105: ba 20 00 00 00              	movl	$0x20, %edx
140a8e10a: e8 f1 ae e4 01              	callq	0x1428d9000 <.text+0x28d8000>
140a8e10f: e9 f3 00 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e114: b9 08 00 00 00              	movl	$0x8, %ecx
140a8e119: ba 20 00 00 00              	movl	$0x20, %edx
140a8e11e: e8 dd ae e4 01              	callq	0x1428d9000 <.text+0x28d8000>
140a8e123: e9 df 00 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e128: b9 08 00 00 00              	movl	$0x8, %ecx
140a8e12d: ba 20 00 00 00              	movl	$0x20, %edx
140a8e132: e8 c9 ae e4 01              	callq	0x1428d9000 <.text+0x28d8000>
140a8e137: e9 cb 00 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e13c: b9 08 00 00 00              	movl	$0x8, %ecx
140a8e141: ba 20 00 00 00              	movl	$0x20, %edx
140a8e146: e8 b5 ae e4 01              	callq	0x1428d9000 <.text+0x28d8000>
140a8e14b: e9 b7 00 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e150: b9 08 00 00 00              	movl	$0x8, %ecx
140a8e155: ba 20 00 00 00              	movl	$0x20, %edx
140a8e15a: e8 a1 ae e4 01              	callq	0x1428d9000 <.text+0x28d8000>
140a8e15f: e9 a3 00 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e164: b9 08 00 00 00              	movl	$0x8, %ecx
140a8e169: ba 20 00 00 00              	movl	$0x20, %edx
140a8e16e: e8 8d ae e4 01              	callq	0x1428d9000 <.text+0x28d8000>
140a8e173: e9 8f 00 00 00              	jmp	0x140a8e207 <.text+0xa8d207>
140a8e178: b9 08 00 00 00              	movl	$0x8, %ecx
140a8e17d: ba 20 00 00 00              	movl	$0x20, %edx
140a8e182: e8 79 ae e4 01              	callq	0x1428d9000 <.text+0x28d8000>
140a8e187: eb 7e                       	jmp	0x140a8e207 <.text+0xa8d207>
140a8e189: 4c 8d 05 a0 a9 04 02        	leaq	0x204a9a0(%rip), %r8    # 0x142ad8b30
140a8e190: b9 04 00 00 00              	movl	$0x4, %ecx
140a8e195: 48 8b 95 d8 0c 00 00        	movq	0xcd8(%rbp), %rdx
140a8e19c: e8 42 ae e4 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1a1: eb 64                       	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1a3: 31 c9                       	xorl	%ecx, %ecx
140a8e1a5: 4c 8d 05 cc a9 1a 02        	leaq	0x21aa9cc(%rip), %r8    # 0x142c38b78
140a8e1ac: 4c 89 ca                    	movq	%r9, %rdx
140a8e1af: e8 2f ae e4 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1b4: eb 51                       	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1b6: 31 c9                       	xorl	%ecx, %ecx
140a8e1b8: 4c 8d 05 b9 a9 1a 02        	leaq	0x21aa9b9(%rip), %r8    # 0x142c38b78
140a8e1bf: 4c 89 ca                    	movq	%r9, %rdx
140a8e1c2: e8 1c ae e4 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1c7: eb 3e                       	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1c9: 31 c9                       	xorl	%ecx, %ecx
140a8e1cb: 4c 8d 05 a6 a9 1a 02        	leaq	0x21aa9a6(%rip), %r8    # 0x142c38b78
140a8e1d2: 48 89 f2                    	movq	%rsi, %rdx
140a8e1d5: e8 09 ae e4 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1da: eb 2b                       	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1dc: 31 c9                       	xorl	%ecx, %ecx
140a8e1de: 4c 8d 05 93 a9 1a 02        	leaq	0x21aa993(%rip), %r8    # 0x142c38b78
140a8e1e5: 4c 89 ca                    	movq	%r9, %rdx
140a8e1e8: e8 f6 ad e4 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1ed: eb 18                       	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1ef: 31 c9                       	xorl	%ecx, %ecx
140a8e1f1: c6 85 56 0d 00 00 01        	movb	$0x1, 0xd56(%rbp)
140a8e1f8: 4c 8d 05 79 a9 1a 02        	leaq	0x21aa979(%rip), %r8    # 0x142c38b78
140a8e1ff: 4c 89 ca                    	movq	%r9, %rdx
140a8e202: e8 dc ad e4 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e207: 0f 0b                       	ud2
140a8e209: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a8e210: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8e215: 55                          	pushq	%rbp
140a8e216: 41 57                       	pushq	%r15
140a8e218: 41 56                       	pushq	%r14
140a8e21a: 41 55                       	pushq	%r13
140a8e21c: 41 54                       	pushq	%r12
140a8e21e: 56                          	pushq	%rsi
140a8e21f: 57                          	pushq	%rdi
140a8e220: 53                          	pushq	%rbx
140a8e221: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8e228: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8e22f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8e235: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8e23b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8e241: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8e247: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8e24d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8e256: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8e25f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8e268: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140a8e270: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
140a8e279: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8e280: 48 ff 00                    	incq	(%rax)
140a8e283: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8e28b: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8e293: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8e29c: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8e2a5: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8e2ae: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8e2b4: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8e2ba: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8e2c0: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8e2c6: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8e2cc: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8e2d3: 5b                          	popq	%rbx
140a8e2d4: 5f                          	popq	%rdi
140a8e2d5: 5e                          	popq	%rsi
140a8e2d6: 41 5c                       	popq	%r12
140a8e2d8: 41 5d                       	popq	%r13
140a8e2da: 41 5e                       	popq	%r14
140a8e2dc: 41 5f                       	popq	%r15
140a8e2de: 5d                          	popq	%rbp
140a8e2df: c3                          	retq
140a8e2e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8e2e5: 55                          	pushq	%rbp
140a8e2e6: 41 57                       	pushq	%r15
140a8e2e8: 41 56                       	pushq	%r14
140a8e2ea: 41 55                       	pushq	%r13
140a8e2ec: 41 54                       	pushq	%r12
140a8e2ee: 56                          	pushq	%rsi
140a8e2ef: 57                          	pushq	%rdi
140a8e2f0: 53                          	pushq	%rbx
140a8e2f1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8e2f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8e2ff: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8e305: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8e30b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8e311: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8e317: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8e31d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8e326: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8e32f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8e338: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140a8e340: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
140a8e349: 48 83 bd 00 0c 00 00 00     	cmpq	$0x0, 0xc00(%rbp)
140a8e351: 74 2f                       	je	0x140a8e382 <.text+0xa8d382>
140a8e353: 48 8d 85 08 0c 00 00        	leaq	0xc08(%rbp), %rax
140a8e35a: 48 89 85 b0 0c 00 00        	movq	%rax, 0xcb0(%rbp)
140a8e361: 48 8d 85 b0 0c 00 00        	leaq	0xcb0(%rbp), %rax
140a8e368: 48 89 85 f0 0b 00 00        	movq	%rax, 0xbf0(%rbp)
140a8e36f: 48 8d 0d 52 a9 04 02        	leaq	0x204a952(%rip), %rcx   # 0x142ad8cc8
140a8e376: 48 8d 95 f0 0b 00 00        	leaq	0xbf0(%rbp), %rdx
140a8e37d: e8 4e 0f 8a 00              	callq	0x14132f2d0 <.text+0x132e2d0>
140a8e382: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8e38a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8e392: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8e39b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8e3a4: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8e3ad: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8e3b3: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8e3b9: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8e3bf: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8e3c5: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8e3cb: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8e3d2: 5b                          	popq	%rbx
140a8e3d3: 5f                          	popq	%rdi
140a8e3d4: 5e                          	popq	%rsi
140a8e3d5: 41 5c                       	popq	%r12
140a8e3d7: 41 5d                       	popq	%r13
140a8e3d9: 41 5e                       	popq	%r14
140a8e3db: 41 5f                       	popq	%r15
140a8e3dd: 5d                          	popq	%rbp
140a8e3de: c3                          	retq
140a8e3df: 90                          	nop
140a8e3e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8e3e5: 55                          	pushq	%rbp
140a8e3e6: 41 57                       	pushq	%r15
140a8e3e8: 41 56                       	pushq	%r14
140a8e3ea: 41 55                       	pushq	%r13
140a8e3ec: 41 54                       	pushq	%r12
140a8e3ee: 56                          	pushq	%rsi
140a8e3ef: 57                          	pushq	%rdi
140a8e3f0: 53                          	pushq	%rbx
140a8e3f1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8e3f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8e3ff: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8e405: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8e40b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8e411: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8e417: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8e41d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8e426: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8e42f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8e438: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140a8e440: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
140a8e449: 48 83 bd 30 0d 00 00 00     	cmpq	$0x0, 0xd30(%rbp)
140a8e451: 48 8b 95 20 0d 00 00        	movq	0xd20(%rbp), %rdx
140a8e458: 74 18                       	je	0x140a8e472 <.text+0xa8d472>
140a8e45a: 48 85 d2                    	testq	%rdx, %rdx
140a8e45d: 74 13                       	je	0x140a8e472 <.text+0xa8d472>
140a8e45f: 48 8b 8d 00 0d 00 00        	movq	0xd00(%rbp), %rcx
140a8e466: 4c 8b 85 10 0d 00 00        	movq	0xd10(%rbp), %r8
140a8e46d: e8 ae 57 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8e472: c6 85 55 0d 00 00 00        	movb	$0x0, 0xd55(%rbp)
140a8e479: 66 0f 6f b4 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm6
140a8e482: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8e48a: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8e493: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8e49c: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8e4a5: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8e4ab: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8e4b1: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8e4b7: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8e4bd: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8e4c3: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8e4ca: 5b                          	popq	%rbx
140a8e4cb: 5f                          	popq	%rdi
140a8e4cc: 5e                          	popq	%rsi
140a8e4cd: 41 5c                       	popq	%r12
140a8e4cf: 41 5d                       	popq	%r13
140a8e4d1: 41 5e                       	popq	%r14
140a8e4d3: 41 5f                       	popq	%r15
140a8e4d5: 5d                          	popq	%rbp
140a8e4d6: c3                          	retq
140a8e4d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a8e4e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8e4e5: 55                          	pushq	%rbp
140a8e4e6: 41 57                       	pushq	%r15
140a8e4e8: 41 56                       	pushq	%r14
140a8e4ea: 41 55                       	pushq	%r13
140a8e4ec: 41 54                       	pushq	%r12
140a8e4ee: 56                          	pushq	%rsi
140a8e4ef: 57                          	pushq	%rdi
140a8e4f0: 53                          	pushq	%rbx
140a8e4f1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8e4f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8e4ff: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8e505: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8e50b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8e511: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8e517: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8e51d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8e526: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8e52f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8e538: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140a8e540: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
140a8e549: 80 bd 55 0d 00 00 00        	cmpb	$0x0, 0xd55(%rbp)
140a8e550: 74 0c                       	je	0x140a8e55e <.text+0xa8d55e>
140a8e552: 48 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %rcx
140a8e559: e8 f2 b9 6b ff              	callq	0x140149f50 <.text+0x148f50>
140a8e55e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8e566: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8e56e: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8e577: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8e580: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8e589: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8e58f: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8e595: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8e59b: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8e5a1: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8e5a7: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8e5ae: 5b                          	popq	%rbx
140a8e5af: 5f                          	popq	%rdi
140a8e5b0: 5e                          	popq	%rsi
140a8e5b1: 41 5c                       	popq	%r12
140a8e5b3: 41 5d                       	popq	%r13
140a8e5b5: 41 5e                       	popq	%r14
140a8e5b7: 41 5f                       	popq	%r15
140a8e5b9: 5d                          	popq	%rbp
140a8e5ba: c3                          	retq
140a8e5bb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140a8e5c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8e5c5: 55                          	pushq	%rbp
140a8e5c6: 41 57                       	pushq	%r15
140a8e5c8: 41 56                       	pushq	%r14
140a8e5ca: 41 55                       	pushq	%r13
140a8e5cc: 41 54                       	pushq	%r12
140a8e5ce: 56                          	pushq	%rsi
140a8e5cf: 57                          	pushq	%rdi
140a8e5d0: 53                          	pushq	%rbx
140a8e5d1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8e5d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8e5df: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8e5e5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8e5eb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8e5f1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8e5f7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8e5fd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8e606: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8e60f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8e618: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8e621: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8e629: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
140a8e630: 48 ff 00                    	incq	(%rax)
140a8e633: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8e63b: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8e643: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8e64c: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8e655: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8e65e: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8e664: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8e66a: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8e670: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8e676: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8e67c: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8e683: 5b                          	popq	%rbx
140a8e684: 5f                          	popq	%rdi
140a8e685: 5e                          	popq	%rsi
140a8e686: 41 5c                       	popq	%r12
140a8e688: 41 5d                       	popq	%r13
140a8e68a: 41 5e                       	popq	%r14
140a8e68c: 41 5f                       	popq	%r15
140a8e68e: 5d                          	popq	%rbp
140a8e68f: c3                          	retq
140a8e690: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8e695: 55                          	pushq	%rbp
140a8e696: 41 57                       	pushq	%r15
140a8e698: 41 56                       	pushq	%r14
140a8e69a: 41 55                       	pushq	%r13
140a8e69c: 41 54                       	pushq	%r12
140a8e69e: 56                          	pushq	%rsi
140a8e69f: 57                          	pushq	%rdi
140a8e6a0: 53                          	pushq	%rbx
140a8e6a1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8e6a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8e6af: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8e6b5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8e6bb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8e6c1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8e6c7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8e6cd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8e6d6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8e6df: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8e6e8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8e6f1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8e6f9: 48 8b 95 18 09 00 00        	movq	0x918(%rbp), %rdx
140a8e700: 48 85 d2                    	testq	%rdx, %rdx
140a8e703: 0f 85 96 00 00 00           	jne	0x140a8e79f <.text+0xa8d79f>
140a8e709: 48 8b 95 30 09 00 00        	movq	0x930(%rbp), %rdx
140a8e710: 48 85 d2                    	testq	%rdx, %rdx
140a8e713: 0f 85 ac 00 00 00           	jne	0x140a8e7c5 <.text+0xa8d7c5>
140a8e719: 48 8b 95 d0 09 00 00        	movq	0x9d0(%rbp), %rdx
140a8e720: 48 85 d2                    	testq	%rdx, %rdx
140a8e723: 74 16                       	je	0x140a8e73b <.text+0xa8d73b>
140a8e725: 48 8b 8d d8 09 00 00        	movq	0x9d8(%rbp), %rcx
140a8e72c: 48 c1 e2 03                 	shlq	$0x3, %rdx
140a8e730: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8e736: e8 e5 54 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8e73b: c6 85 57 0d 00 00 00        	movb	$0x0, 0xd57(%rbp)
140a8e742: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8e74a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8e752: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8e75b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8e764: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8e76d: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8e773: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8e779: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8e77f: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8e785: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8e78b: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8e792: 5b                          	popq	%rbx
140a8e793: 5f                          	popq	%rdi
140a8e794: 5e                          	popq	%rsi
140a8e795: 41 5c                       	popq	%r12
140a8e797: 41 5d                       	popq	%r13
140a8e799: 41 5e                       	popq	%r14
140a8e79b: 41 5f                       	popq	%r15
140a8e79d: 5d                          	popq	%rbp
140a8e79e: c3                          	retq
140a8e79f: 48 8b 8d 20 09 00 00        	movq	0x920(%rbp), %rcx
140a8e7a6: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8e7aa: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8e7b0: e8 6b 54 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8e7b5: 48 8b 95 30 09 00 00        	movq	0x930(%rbp), %rdx
140a8e7bc: 48 85 d2                    	testq	%rdx, %rdx
140a8e7bf: 0f 84 54 ff ff ff           	je	0x140a8e719 <.text+0xa8d719>
140a8e7c5: 48 8b 8d 38 09 00 00        	movq	0x938(%rbp), %rcx
140a8e7cc: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a8e7d0: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8e7d6: e8 45 54 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8e7db: 48 8b 95 d0 09 00 00        	movq	0x9d0(%rbp), %rdx
140a8e7e2: 48 85 d2                    	testq	%rdx, %rdx
140a8e7e5: 0f 85 3a ff ff ff           	jne	0x140a8e725 <.text+0xa8d725>
140a8e7eb: e9 4b ff ff ff              	jmp	0x140a8e73b <.text+0xa8d73b>
140a8e7f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8e7f5: 55                          	pushq	%rbp
140a8e7f6: 41 57                       	pushq	%r15
140a8e7f8: 41 56                       	pushq	%r14
140a8e7fa: 41 55                       	pushq	%r13
140a8e7fc: 41 54                       	pushq	%r12
140a8e7fe: 56                          	pushq	%rsi
140a8e7ff: 57                          	pushq	%rdi
140a8e800: 53                          	pushq	%rbx
140a8e801: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8e808: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8e80f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8e815: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8e81b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8e821: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8e827: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8e82d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8e836: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8e83f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8e848: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8e851: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8e859: 48 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %rcx
140a8e860: e8 9b 6d e0 ff              	callq	0x140895600 <.text+0x894600>
140a8e865: c6 85 57 0d 00 00 00        	movb	$0x0, 0xd57(%rbp)
140a8e86c: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8e874: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8e87c: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8e885: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8e88e: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8e897: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8e89d: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8e8a3: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8e8a9: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8e8af: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8e8b5: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8e8bc: 5b                          	popq	%rbx
140a8e8bd: 5f                          	popq	%rdi
140a8e8be: 5e                          	popq	%rsi
140a8e8bf: 41 5c                       	popq	%r12
140a8e8c1: 41 5d                       	popq	%r13
140a8e8c3: 41 5e                       	popq	%r14
140a8e8c5: 41 5f                       	popq	%r15
140a8e8c7: 5d                          	popq	%rbp
140a8e8c8: c3                          	retq
140a8e8c9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a8e8d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8e8d5: 55                          	pushq	%rbp
140a8e8d6: 41 57                       	pushq	%r15
140a8e8d8: 41 56                       	pushq	%r14
140a8e8da: 41 55                       	pushq	%r13
140a8e8dc: 41 54                       	pushq	%r12
140a8e8de: 56                          	pushq	%rsi
140a8e8df: 57                          	pushq	%rdi
140a8e8e0: 53                          	pushq	%rbx
140a8e8e1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8e8e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8e8ef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8e8f5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8e8fb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8e901: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8e907: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8e90d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8e916: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8e91f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8e928: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8e931: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8e939: 48 83 bd 38 0d 00 00 00     	cmpq	$0x0, 0xd38(%rbp)
140a8e941: 74 1d                       	je	0x140a8e960 <.text+0xa8d960>
140a8e943: 48 8b 95 38 0d 00 00        	movq	0xd38(%rbp), %rdx
140a8e94a: 48 c1 e2 03                 	shlq	$0x3, %rdx
140a8e94e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8e954: 48 8b 8d 18 0d 00 00        	movq	0xd18(%rbp), %rcx
140a8e95b: e8 c0 52 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8e960: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8e968: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8e971: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8e97a: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8e983: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8e98c: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8e992: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8e998: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8e99e: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8e9a4: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8e9aa: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8e9b1: 5b                          	popq	%rbx
140a8e9b2: 5f                          	popq	%rdi
140a8e9b3: 5e                          	popq	%rsi
140a8e9b4: 41 5c                       	popq	%r12
140a8e9b6: 41 5d                       	popq	%r13
140a8e9b8: 41 5e                       	popq	%r14
140a8e9ba: 41 5f                       	popq	%r15
140a8e9bc: 5d                          	popq	%rbp
140a8e9bd: c3                          	retq
140a8e9be: 66 90                       	nop
140a8e9c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8e9c5: 55                          	pushq	%rbp
140a8e9c6: 41 57                       	pushq	%r15
140a8e9c8: 41 56                       	pushq	%r14
140a8e9ca: 41 55                       	pushq	%r13
140a8e9cc: 41 54                       	pushq	%r12
140a8e9ce: 56                          	pushq	%rsi
140a8e9cf: 57                          	pushq	%rdi
140a8e9d0: 53                          	pushq	%rbx
140a8e9d1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8e9d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8e9df: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8e9e5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8e9eb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8e9f1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8e9f7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8e9fd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8ea06: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8ea0f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8ea18: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8ea21: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8ea29: 48 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %rcx
140a8ea30: e8 bb 3a b4 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140a8ea35: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8ea3d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8ea46: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8ea4f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8ea58: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8ea61: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8ea67: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8ea6d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8ea73: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8ea79: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8ea7f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8ea86: 5b                          	popq	%rbx
140a8ea87: 5f                          	popq	%rdi
140a8ea88: 5e                          	popq	%rsi
140a8ea89: 41 5c                       	popq	%r12
140a8ea8b: 41 5d                       	popq	%r13
140a8ea8d: 41 5e                       	popq	%r14
140a8ea8f: 41 5f                       	popq	%r15
140a8ea91: 5d                          	popq	%rbp
140a8ea92: c3                          	retq
140a8ea93: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8eaa0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8eaa5: 55                          	pushq	%rbp
140a8eaa6: 41 57                       	pushq	%r15
140a8eaa8: 41 56                       	pushq	%r14
140a8eaaa: 41 55                       	pushq	%r13
140a8eaac: 41 54                       	pushq	%r12
140a8eaae: 56                          	pushq	%rsi
140a8eaaf: 57                          	pushq	%rdi
140a8eab0: 53                          	pushq	%rbx
140a8eab1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8eab8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8eabf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8eac5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8eacb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8ead1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8ead7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8eadd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8eae6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8eaef: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8eaf8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8eb01: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8eb09: 48 8d 8d 60 02 00 00        	leaq	0x260(%rbp), %rcx
140a8eb10: e8 0b cf 6b ff              	callq	0x14014ba20 <.text+0x14aa20>
140a8eb15: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8eb1d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8eb26: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8eb2f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8eb38: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8eb41: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8eb47: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8eb4d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8eb53: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8eb59: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8eb5f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8eb66: 5b                          	popq	%rbx
140a8eb67: 5f                          	popq	%rdi
140a8eb68: 5e                          	popq	%rsi
140a8eb69: 41 5c                       	popq	%r12
140a8eb6b: 41 5d                       	popq	%r13
140a8eb6d: 41 5e                       	popq	%r14
140a8eb6f: 41 5f                       	popq	%r15
140a8eb71: 5d                          	popq	%rbp
140a8eb72: c3                          	retq
140a8eb73: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8eb80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8eb85: 55                          	pushq	%rbp
140a8eb86: 41 57                       	pushq	%r15
140a8eb88: 41 56                       	pushq	%r14
140a8eb8a: 41 55                       	pushq	%r13
140a8eb8c: 41 54                       	pushq	%r12
140a8eb8e: 56                          	pushq	%rsi
140a8eb8f: 57                          	pushq	%rdi
140a8eb90: 53                          	pushq	%rbx
140a8eb91: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8eb98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8eb9f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8eba5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8ebab: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8ebb1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8ebb7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8ebbd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8ebc6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8ebcf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8ebd8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8ebe1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8ebe9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8ebef: 48 8b 8d 38 0d 00 00        	movq	0xd38(%rbp), %rcx
140a8ebf6: 48 8b 95 d8 0c 00 00        	movq	0xcd8(%rbp), %rdx
140a8ebfd: e8 1e 50 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8ec02: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8ec0a: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8ec13: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8ec1c: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8ec25: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8ec2e: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8ec34: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8ec3a: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8ec40: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8ec46: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8ec4c: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8ec53: 5b                          	popq	%rbx
140a8ec54: 5f                          	popq	%rdi
140a8ec55: 5e                          	popq	%rsi
140a8ec56: 41 5c                       	popq	%r12
140a8ec58: 41 5d                       	popq	%r13
140a8ec5a: 41 5e                       	popq	%r14
140a8ec5c: 41 5f                       	popq	%r15
140a8ec5e: 5d                          	popq	%rbp
140a8ec5f: c3                          	retq
140a8ec60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8ec65: 55                          	pushq	%rbp
140a8ec66: 41 57                       	pushq	%r15
140a8ec68: 41 56                       	pushq	%r14
140a8ec6a: 41 55                       	pushq	%r13
140a8ec6c: 41 54                       	pushq	%r12
140a8ec6e: 56                          	pushq	%rsi
140a8ec6f: 57                          	pushq	%rdi
140a8ec70: 53                          	pushq	%rbx
140a8ec71: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8ec78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8ec7f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8ec85: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8ec8b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8ec91: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8ec97: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8ec9d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8eca6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8ecaf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8ecb8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8ecc1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8ecc9: 48 83 bd 18 0d 00 00 00     	cmpq	$0x0, 0xd18(%rbp)
140a8ecd1: 74 19                       	je	0x140a8ecec <.text+0xa8dcec>
140a8ecd3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8ecd9: 48 8b 8d 38 0d 00 00        	movq	0xd38(%rbp), %rcx
140a8ece0: 48 8b 95 d8 0c 00 00        	movq	0xcd8(%rbp), %rdx
140a8ece7: e8 34 4f b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8ecec: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8ecf4: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8ecfd: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8ed06: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8ed0f: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8ed18: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8ed1e: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8ed24: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8ed2a: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8ed30: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8ed36: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8ed3d: 5b                          	popq	%rbx
140a8ed3e: 5f                          	popq	%rdi
140a8ed3f: 5e                          	popq	%rsi
140a8ed40: 41 5c                       	popq	%r12
140a8ed42: 41 5d                       	popq	%r13
140a8ed44: 41 5e                       	popq	%r14
140a8ed46: 41 5f                       	popq	%r15
140a8ed48: 5d                          	popq	%rbp
140a8ed49: c3                          	retq
140a8ed4a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140a8ed50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8ed55: 55                          	pushq	%rbp
140a8ed56: 41 57                       	pushq	%r15
140a8ed58: 41 56                       	pushq	%r14
140a8ed5a: 41 55                       	pushq	%r13
140a8ed5c: 41 54                       	pushq	%r12
140a8ed5e: 56                          	pushq	%rsi
140a8ed5f: 57                          	pushq	%rdi
140a8ed60: 53                          	pushq	%rbx
140a8ed61: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8ed68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8ed6f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8ed75: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8ed7b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8ed81: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8ed87: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8ed8d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8ed96: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8ed9f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8eda8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8edb1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8edb9: 48 8d 8d 48 0b 00 00        	leaq	0xb48(%rbp), %rcx
140a8edc0: e8 cb 05 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8edc5: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8edcd: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8edd6: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8eddf: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8ede8: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8edf1: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8edf7: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8edfd: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8ee03: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8ee09: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8ee0f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8ee16: 5b                          	popq	%rbx
140a8ee17: 5f                          	popq	%rdi
140a8ee18: 5e                          	popq	%rsi
140a8ee19: 41 5c                       	popq	%r12
140a8ee1b: 41 5d                       	popq	%r13
140a8ee1d: 41 5e                       	popq	%r14
140a8ee1f: 41 5f                       	popq	%r15
140a8ee21: 5d                          	popq	%rbp
140a8ee22: c3                          	retq
140a8ee23: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8ee30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8ee35: 55                          	pushq	%rbp
140a8ee36: 41 57                       	pushq	%r15
140a8ee38: 41 56                       	pushq	%r14
140a8ee3a: 41 55                       	pushq	%r13
140a8ee3c: 41 54                       	pushq	%r12
140a8ee3e: 56                          	pushq	%rsi
140a8ee3f: 57                          	pushq	%rdi
140a8ee40: 53                          	pushq	%rbx
140a8ee41: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8ee48: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8ee4f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8ee55: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8ee5b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8ee61: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8ee67: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8ee6d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8ee76: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8ee7f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8ee88: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8ee91: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8ee99: 48 8d 8d 38 0b 00 00        	leaq	0xb38(%rbp), %rcx
140a8eea0: e8 eb 04 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8eea5: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8eead: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8eeb6: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8eebf: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8eec8: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8eed1: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8eed7: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8eedd: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8eee3: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8eee9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8eeef: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8eef6: 5b                          	popq	%rbx
140a8eef7: 5f                          	popq	%rdi
140a8eef8: 5e                          	popq	%rsi
140a8eef9: 41 5c                       	popq	%r12
140a8eefb: 41 5d                       	popq	%r13
140a8eefd: 41 5e                       	popq	%r14
140a8eeff: 41 5f                       	popq	%r15
140a8ef01: 5d                          	popq	%rbp
140a8ef02: c3                          	retq
140a8ef03: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8ef10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8ef15: 55                          	pushq	%rbp
140a8ef16: 41 57                       	pushq	%r15
140a8ef18: 41 56                       	pushq	%r14
140a8ef1a: 41 55                       	pushq	%r13
140a8ef1c: 41 54                       	pushq	%r12
140a8ef1e: 56                          	pushq	%rsi
140a8ef1f: 57                          	pushq	%rdi
140a8ef20: 53                          	pushq	%rbx
140a8ef21: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8ef28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8ef2f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8ef35: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8ef3b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8ef41: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8ef47: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8ef4d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8ef56: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8ef5f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8ef68: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8ef71: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8ef79: 48 8d 8d 28 0b 00 00        	leaq	0xb28(%rbp), %rcx
140a8ef80: e8 0b 04 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8ef85: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8ef8d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8ef96: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8ef9f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8efa8: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8efb1: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8efb7: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8efbd: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8efc3: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8efc9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8efcf: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8efd6: 5b                          	popq	%rbx
140a8efd7: 5f                          	popq	%rdi
140a8efd8: 5e                          	popq	%rsi
140a8efd9: 41 5c                       	popq	%r12
140a8efdb: 41 5d                       	popq	%r13
140a8efdd: 41 5e                       	popq	%r14
140a8efdf: 41 5f                       	popq	%r15
140a8efe1: 5d                          	popq	%rbp
140a8efe2: c3                          	retq
140a8efe3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8eff0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8eff5: 55                          	pushq	%rbp
140a8eff6: 41 57                       	pushq	%r15
140a8eff8: 41 56                       	pushq	%r14
140a8effa: 41 55                       	pushq	%r13
140a8effc: 41 54                       	pushq	%r12
140a8effe: 56                          	pushq	%rsi
140a8efff: 57                          	pushq	%rdi
140a8f000: 53                          	pushq	%rbx
140a8f001: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f008: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f00f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f015: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f01b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f021: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f027: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f02d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f036: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f03f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f048: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8f051: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f059: 48 8d 8d 18 0b 00 00        	leaq	0xb18(%rbp), %rcx
140a8f060: e8 2b 03 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8f065: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f06d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8f076: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f07f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f088: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f091: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f097: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f09d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f0a3: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f0a9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f0af: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f0b6: 5b                          	popq	%rbx
140a8f0b7: 5f                          	popq	%rdi
140a8f0b8: 5e                          	popq	%rsi
140a8f0b9: 41 5c                       	popq	%r12
140a8f0bb: 41 5d                       	popq	%r13
140a8f0bd: 41 5e                       	popq	%r14
140a8f0bf: 41 5f                       	popq	%r15
140a8f0c1: 5d                          	popq	%rbp
140a8f0c2: c3                          	retq
140a8f0c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8f0d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8f0d5: 55                          	pushq	%rbp
140a8f0d6: 41 57                       	pushq	%r15
140a8f0d8: 41 56                       	pushq	%r14
140a8f0da: 41 55                       	pushq	%r13
140a8f0dc: 41 54                       	pushq	%r12
140a8f0de: 56                          	pushq	%rsi
140a8f0df: 57                          	pushq	%rdi
140a8f0e0: 53                          	pushq	%rbx
140a8f0e1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f0e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f0ef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f0f5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f0fb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f101: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f107: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f10d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f116: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f11f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f128: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8f131: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f139: 48 8d 8d 08 0b 00 00        	leaq	0xb08(%rbp), %rcx
140a8f140: e8 4b 02 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8f145: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f14d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8f156: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f15f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f168: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f171: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f177: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f17d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f183: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f189: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f18f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f196: 5b                          	popq	%rbx
140a8f197: 5f                          	popq	%rdi
140a8f198: 5e                          	popq	%rsi
140a8f199: 41 5c                       	popq	%r12
140a8f19b: 41 5d                       	popq	%r13
140a8f19d: 41 5e                       	popq	%r14
140a8f19f: 41 5f                       	popq	%r15
140a8f1a1: 5d                          	popq	%rbp
140a8f1a2: c3                          	retq
140a8f1a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8f1b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8f1b5: 55                          	pushq	%rbp
140a8f1b6: 41 57                       	pushq	%r15
140a8f1b8: 41 56                       	pushq	%r14
140a8f1ba: 41 55                       	pushq	%r13
140a8f1bc: 41 54                       	pushq	%r12
140a8f1be: 56                          	pushq	%rsi
140a8f1bf: 57                          	pushq	%rdi
140a8f1c0: 53                          	pushq	%rbx
140a8f1c1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f1c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f1cf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f1d5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f1db: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f1e1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f1e7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f1ed: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f1f6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f1ff: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f208: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8f211: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f219: 48 8d 8d f8 0a 00 00        	leaq	0xaf8(%rbp), %rcx
140a8f220: e8 6b 01 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8f225: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f22d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8f236: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f23f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f248: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f251: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f257: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f25d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f263: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f269: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f26f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f276: 5b                          	popq	%rbx
140a8f277: 5f                          	popq	%rdi
140a8f278: 5e                          	popq	%rsi
140a8f279: 41 5c                       	popq	%r12
140a8f27b: 41 5d                       	popq	%r13
140a8f27d: 41 5e                       	popq	%r14
140a8f27f: 41 5f                       	popq	%r15
140a8f281: 5d                          	popq	%rbp
140a8f282: c3                          	retq
140a8f283: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8f290: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8f295: 55                          	pushq	%rbp
140a8f296: 41 57                       	pushq	%r15
140a8f298: 41 56                       	pushq	%r14
140a8f29a: 41 55                       	pushq	%r13
140a8f29c: 41 54                       	pushq	%r12
140a8f29e: 56                          	pushq	%rsi
140a8f29f: 57                          	pushq	%rdi
140a8f2a0: 53                          	pushq	%rbx
140a8f2a1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f2a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f2af: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f2b5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f2bb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f2c1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f2c7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f2cd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f2d6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f2df: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f2e8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8f2f1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f2f9: 48 8d 8d e8 0a 00 00        	leaq	0xae8(%rbp), %rcx
140a8f300: e8 8b 00 df ff              	callq	0x14087f390 <.text+0x87e390>
140a8f305: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f30d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8f316: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f31f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f328: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f331: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f337: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f33d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f343: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f349: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f34f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f356: 5b                          	popq	%rbx
140a8f357: 5f                          	popq	%rdi
140a8f358: 5e                          	popq	%rsi
140a8f359: 41 5c                       	popq	%r12
140a8f35b: 41 5d                       	popq	%r13
140a8f35d: 41 5e                       	popq	%r14
140a8f35f: 41 5f                       	popq	%r15
140a8f361: 5d                          	popq	%rbp
140a8f362: c3                          	retq
140a8f363: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8f370: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8f375: 55                          	pushq	%rbp
140a8f376: 41 57                       	pushq	%r15
140a8f378: 41 56                       	pushq	%r14
140a8f37a: 41 55                       	pushq	%r13
140a8f37c: 41 54                       	pushq	%r12
140a8f37e: 56                          	pushq	%rsi
140a8f37f: 57                          	pushq	%rdi
140a8f380: 53                          	pushq	%rbx
140a8f381: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f388: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f38f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f395: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f39b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f3a1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f3a7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f3ad: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f3b6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f3bf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f3c8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8f3d1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f3d9: 48 83 bd a0 0c 00 00 00     	cmpq	$0x0, 0xca0(%rbp)
140a8f3e1: 74 1d                       	je	0x140a8f400 <.text+0xa8e400>
140a8f3e3: 48 8b 95 a0 0c 00 00        	movq	0xca0(%rbp), %rdx
140a8f3ea: 48 c1 e2 03                 	shlq	$0x3, %rdx
140a8f3ee: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8f3f4: 48 8b 8d 20 0c 00 00        	movq	0xc20(%rbp), %rcx
140a8f3fb: e8 20 48 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8f400: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f408: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8f411: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f41a: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f423: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f42c: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f432: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f438: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f43e: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f444: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f44a: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f451: 5b                          	popq	%rbx
140a8f452: 5f                          	popq	%rdi
140a8f453: 5e                          	popq	%rsi
140a8f454: 41 5c                       	popq	%r12
140a8f456: 41 5d                       	popq	%r13
140a8f458: 41 5e                       	popq	%r14
140a8f45a: 41 5f                       	popq	%r15
140a8f45c: 5d                          	popq	%rbp
140a8f45d: c3                          	retq
140a8f45e: 66 90                       	nop
140a8f460: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8f465: 55                          	pushq	%rbp
140a8f466: 41 57                       	pushq	%r15
140a8f468: 41 56                       	pushq	%r14
140a8f46a: 41 55                       	pushq	%r13
140a8f46c: 41 54                       	pushq	%r12
140a8f46e: 56                          	pushq	%rsi
140a8f46f: 57                          	pushq	%rdi
140a8f470: 53                          	pushq	%rbx
140a8f471: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f478: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f47f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f485: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f48b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f491: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f497: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f49d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f4a6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f4af: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f4b8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8f4c1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f4c9: 48 8b 95 30 0c 00 00        	movq	0xc30(%rbp), %rdx
140a8f4d0: 48 85 d2                    	testq	%rdx, %rdx
140a8f4d3: 0f 94 c0                    	sete	%al
140a8f4d6: 8b 8d a8 0c 00 00           	movl	0xca8(%rbp), %ecx
140a8f4dc: 08 c1                       	orb	%al, %cl
140a8f4de: 75 16                       	jne	0x140a8f4f6 <.text+0xa8e4f6>
140a8f4e0: 48 c1 e2 03                 	shlq	$0x3, %rdx
140a8f4e4: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a8f4ea: 48 8b 8d 38 0c 00 00        	movq	0xc38(%rbp), %rcx
140a8f4f1: e8 2a 47 b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a8f4f6: 8b 85 28 0d 00 00           	movl	0xd28(%rbp), %eax
140a8f4fc: 88 85 57 0d 00 00           	movb	%al, 0xd57(%rbp)
140a8f502: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f50a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8f512: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f51b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f524: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f52d: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f533: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f539: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f53f: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f545: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f54b: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f552: 5b                          	popq	%rbx
140a8f553: 5f                          	popq	%rdi
140a8f554: 5e                          	popq	%rsi
140a8f555: 41 5c                       	popq	%r12
140a8f557: 41 5d                       	popq	%r13
140a8f559: 41 5e                       	popq	%r14
140a8f55b: 41 5f                       	popq	%r15
140a8f55d: 5d                          	popq	%rbp
140a8f55e: c3                          	retq
140a8f55f: 90                          	nop
140a8f560: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8f565: 55                          	pushq	%rbp
140a8f566: 41 57                       	pushq	%r15
140a8f568: 41 56                       	pushq	%r14
140a8f56a: 41 55                       	pushq	%r13
140a8f56c: 41 54                       	pushq	%r12
140a8f56e: 56                          	pushq	%rsi
140a8f56f: 57                          	pushq	%rdi
140a8f570: 53                          	pushq	%rbx
140a8f571: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f578: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f57f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f585: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f58b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f591: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f597: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f59d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f5a6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f5af: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f5b8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8f5c1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f5c9: 80 bd 56 0d 00 00 00        	cmpb	$0x0, 0xd56(%rbp)
140a8f5d0: 74 75                       	je	0x140a8f647 <.text+0xa8e647>
140a8f5d2: 48 8d 8d 00 05 00 00        	leaq	0x500(%rbp), %rcx
140a8f5d9: e8 22 60 e0 ff              	callq	0x140895600 <.text+0x894600>
140a8f5de: 8b 85 28 0d 00 00           	movl	0xd28(%rbp), %eax
140a8f5e4: 88 85 57 0d 00 00           	movb	%al, 0xd57(%rbp)
140a8f5ea: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f5f2: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8f5fa: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f603: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f60c: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f615: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f61b: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f621: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f627: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f62d: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f633: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f63a: 5b                          	popq	%rbx
140a8f63b: 5f                          	popq	%rdi
140a8f63c: 5e                          	popq	%rsi
140a8f63d: 41 5c                       	popq	%r12
140a8f63f: 41 5d                       	popq	%r13
140a8f641: 41 5e                       	popq	%r14
140a8f643: 41 5f                       	popq	%r15
140a8f645: 5d                          	popq	%rbp
140a8f646: c3                          	retq
140a8f647: 8b 85 28 0d 00 00           	movl	0xd28(%rbp), %eax
140a8f64d: 88 85 57 0d 00 00           	movb	%al, 0xd57(%rbp)
140a8f653: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f65b: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8f663: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f66c: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f675: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f67e: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f684: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f68a: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f690: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f696: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f69c: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f6a3: 5b                          	popq	%rbx
140a8f6a4: 5f                          	popq	%rdi
140a8f6a5: 5e                          	popq	%rsi
140a8f6a6: 41 5c                       	popq	%r12
140a8f6a8: 41 5d                       	popq	%r13
140a8f6aa: 41 5e                       	popq	%r14
140a8f6ac: 41 5f                       	popq	%r15
140a8f6ae: 5d                          	popq	%rbp
140a8f6af: c3                          	retq
140a8f6b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8f6b5: 55                          	pushq	%rbp
140a8f6b6: 41 57                       	pushq	%r15
140a8f6b8: 41 56                       	pushq	%r14
140a8f6ba: 41 55                       	pushq	%r13
140a8f6bc: 41 54                       	pushq	%r12
140a8f6be: 56                          	pushq	%rsi
140a8f6bf: 57                          	pushq	%rdi
140a8f6c0: 53                          	pushq	%rbx
140a8f6c1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f6c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f6cf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f6d5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f6db: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f6e1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f6e7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f6ed: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f6f6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f6ff: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f708: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140a8f710: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f718: 31 c0                       	xorl	%eax, %eax
140a8f71a: 48 3b 45 c0                 	cmpq	-0x40(%rbp), %rax
140a8f71e: 70 73                       	jo	0x140a8f793 <.text+0xa8e793>
140a8f720: f6 85 57 0d 00 00 01        	testb	$0x1, 0xd57(%rbp)
140a8f727: 0f 84 c3 00 00 00           	je	0x140a8f7f0 <.text+0xa8e7f0>
140a8f72d: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140a8f731: e8 ca 5e e0 ff              	callq	0x140895600 <.text+0x894600>
140a8f736: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f73e: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8f746: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f74f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f758: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f761: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f767: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f76d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f773: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f779: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f77f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f786: 5b                          	popq	%rbx
140a8f787: 5f                          	popq	%rdi
140a8f788: 5e                          	popq	%rsi
140a8f789: 41 5c                       	popq	%r12
140a8f78b: 41 5d                       	popq	%r13
140a8f78d: 41 5e                       	popq	%r14
140a8f78f: 41 5f                       	popq	%r15
140a8f791: 5d                          	popq	%rbp
140a8f792: c3                          	retq
140a8f793: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f79b: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8f7a3: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f7ac: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f7b5: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f7be: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f7c4: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f7ca: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f7d0: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f7d6: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f7dc: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f7e3: 5b                          	popq	%rbx
140a8f7e4: 5f                          	popq	%rdi
140a8f7e5: 5e                          	popq	%rsi
140a8f7e6: 41 5c                       	popq	%r12
140a8f7e8: 41 5d                       	popq	%r13
140a8f7ea: 41 5e                       	popq	%r14
140a8f7ec: 41 5f                       	popq	%r15
140a8f7ee: 5d                          	popq	%rbp
140a8f7ef: c3                          	retq
140a8f7f0: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f7f8: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8f800: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f809: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f812: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f81b: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f821: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f827: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f82d: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f833: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f839: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f840: 5b                          	popq	%rbx
140a8f841: 5f                          	popq	%rdi
140a8f842: 5e                          	popq	%rsi
140a8f843: 41 5c                       	popq	%r12
140a8f845: 41 5d                       	popq	%r13
140a8f847: 41 5e                       	popq	%r14
140a8f849: 41 5f                       	popq	%r15
140a8f84b: 5d                          	popq	%rbp
140a8f84c: c3                          	retq
140a8f84d: 0f 1f 00                    	nopl	(%rax)
140a8f850: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8f855: 55                          	pushq	%rbp
140a8f856: 41 57                       	pushq	%r15
140a8f858: 41 56                       	pushq	%r14
140a8f85a: 41 55                       	pushq	%r13
140a8f85c: 41 54                       	pushq	%r12
140a8f85e: 56                          	pushq	%rsi
140a8f85f: 57                          	pushq	%rdi
140a8f860: 53                          	pushq	%rbx
140a8f861: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f868: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f86f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f875: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f87b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f881: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f887: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f88d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f896: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f89f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f8a8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8f8b1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f8b9: 48 8d 8d 68 09 00 00        	leaq	0x968(%rbp), %rcx
140a8f8c0: e8 cb fa de ff              	callq	0x14087f390 <.text+0x87e390>
140a8f8c5: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f8cd: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8f8d6: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f8df: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f8e8: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f8f1: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f8f7: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f8fd: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f903: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f909: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f90f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f916: 5b                          	popq	%rbx
140a8f917: 5f                          	popq	%rdi
140a8f918: 5e                          	popq	%rsi
140a8f919: 41 5c                       	popq	%r12
140a8f91b: 41 5d                       	popq	%r13
140a8f91d: 41 5e                       	popq	%r14
140a8f91f: 41 5f                       	popq	%r15
140a8f921: 5d                          	popq	%rbp
140a8f922: c3                          	retq
140a8f923: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8f930: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8f935: 55                          	pushq	%rbp
140a8f936: 41 57                       	pushq	%r15
140a8f938: 41 56                       	pushq	%r14
140a8f93a: 41 55                       	pushq	%r13
140a8f93c: 41 54                       	pushq	%r12
140a8f93e: 56                          	pushq	%rsi
140a8f93f: 57                          	pushq	%rdi
140a8f940: 53                          	pushq	%rbx
140a8f941: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8f948: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8f94f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8f955: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8f95b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8f961: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8f967: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8f96d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8f976: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8f97f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8f988: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8f991: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8f999: 48 8d 8d 78 09 00 00        	leaq	0x978(%rbp), %rcx
140a8f9a0: e8 eb f9 de ff              	callq	0x14087f390 <.text+0x87e390>
140a8f9a5: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8f9ad: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8f9b6: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8f9bf: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8f9c8: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8f9d1: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8f9d7: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8f9dd: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8f9e3: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8f9e9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8f9ef: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8f9f6: 5b                          	popq	%rbx
140a8f9f7: 5f                          	popq	%rdi
140a8f9f8: 5e                          	popq	%rsi
140a8f9f9: 41 5c                       	popq	%r12
140a8f9fb: 41 5d                       	popq	%r13
140a8f9fd: 41 5e                       	popq	%r14
140a8f9ff: 41 5f                       	popq	%r15
140a8fa01: 5d                          	popq	%rbp
140a8fa02: c3                          	retq
140a8fa03: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8fa10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8fa15: 55                          	pushq	%rbp
140a8fa16: 41 57                       	pushq	%r15
140a8fa18: 41 56                       	pushq	%r14
140a8fa1a: 41 55                       	pushq	%r13
140a8fa1c: 41 54                       	pushq	%r12
140a8fa1e: 56                          	pushq	%rsi
140a8fa1f: 57                          	pushq	%rdi
140a8fa20: 53                          	pushq	%rbx
140a8fa21: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8fa28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8fa2f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8fa35: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8fa3b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8fa41: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8fa47: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8fa4d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8fa56: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8fa5f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8fa68: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8fa71: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8fa79: 48 8d 8d 88 09 00 00        	leaq	0x988(%rbp), %rcx
140a8fa80: e8 0b f9 de ff              	callq	0x14087f390 <.text+0x87e390>
140a8fa85: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8fa8d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8fa96: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8fa9f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8faa8: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8fab1: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8fab7: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8fabd: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8fac3: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8fac9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8facf: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8fad6: 5b                          	popq	%rbx
140a8fad7: 5f                          	popq	%rdi
140a8fad8: 5e                          	popq	%rsi
140a8fad9: 41 5c                       	popq	%r12
140a8fadb: 41 5d                       	popq	%r13
140a8fadd: 41 5e                       	popq	%r14
140a8fadf: 41 5f                       	popq	%r15
140a8fae1: 5d                          	popq	%rbp
140a8fae2: c3                          	retq
140a8fae3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8faf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8faf5: 55                          	pushq	%rbp
140a8faf6: 41 57                       	pushq	%r15
140a8faf8: 41 56                       	pushq	%r14
140a8fafa: 41 55                       	pushq	%r13
140a8fafc: 41 54                       	pushq	%r12
140a8fafe: 56                          	pushq	%rsi
140a8faff: 57                          	pushq	%rdi
140a8fb00: 53                          	pushq	%rbx
140a8fb01: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8fb08: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8fb0f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8fb15: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8fb1b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8fb21: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8fb27: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8fb2d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8fb36: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8fb3f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8fb48: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8fb51: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8fb59: 48 8d 8d 98 09 00 00        	leaq	0x998(%rbp), %rcx
140a8fb60: e8 2b f8 de ff              	callq	0x14087f390 <.text+0x87e390>
140a8fb65: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8fb6d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8fb76: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8fb7f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8fb88: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8fb91: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8fb97: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8fb9d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8fba3: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8fba9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8fbaf: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8fbb6: 5b                          	popq	%rbx
140a8fbb7: 5f                          	popq	%rdi
140a8fbb8: 5e                          	popq	%rsi
140a8fbb9: 41 5c                       	popq	%r12
140a8fbbb: 41 5d                       	popq	%r13
140a8fbbd: 41 5e                       	popq	%r14
140a8fbbf: 41 5f                       	popq	%r15
140a8fbc1: 5d                          	popq	%rbp
140a8fbc2: c3                          	retq
140a8fbc3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8fbd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8fbd5: 55                          	pushq	%rbp
140a8fbd6: 41 57                       	pushq	%r15
140a8fbd8: 41 56                       	pushq	%r14
140a8fbda: 41 55                       	pushq	%r13
140a8fbdc: 41 54                       	pushq	%r12
140a8fbde: 56                          	pushq	%rsi
140a8fbdf: 57                          	pushq	%rdi
140a8fbe0: 53                          	pushq	%rbx
140a8fbe1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8fbe8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8fbef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8fbf5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8fbfb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8fc01: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8fc07: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8fc0d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8fc16: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8fc1f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8fc28: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8fc31: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8fc39: 48 8d 8d a8 09 00 00        	leaq	0x9a8(%rbp), %rcx
140a8fc40: e8 4b f7 de ff              	callq	0x14087f390 <.text+0x87e390>
140a8fc45: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8fc4d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8fc56: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8fc5f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8fc68: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8fc71: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8fc77: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8fc7d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8fc83: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8fc89: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8fc8f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8fc96: 5b                          	popq	%rbx
140a8fc97: 5f                          	popq	%rdi
140a8fc98: 5e                          	popq	%rsi
140a8fc99: 41 5c                       	popq	%r12
140a8fc9b: 41 5d                       	popq	%r13
140a8fc9d: 41 5e                       	popq	%r14
140a8fc9f: 41 5f                       	popq	%r15
140a8fca1: 5d                          	popq	%rbp
140a8fca2: c3                          	retq
140a8fca3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a8fcb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a8fcb5: 55                          	pushq	%rbp
140a8fcb6: 41 57                       	pushq	%r15
140a8fcb8: 41 56                       	pushq	%r14
140a8fcba: 41 55                       	pushq	%r13
140a8fcbc: 41 54                       	pushq	%r12
140a8fcbe: 56                          	pushq	%rsi
140a8fcbf: 57                          	pushq	%rdi
140a8fcc0: 53                          	pushq	%rbx
140a8fcc1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140a8fcc8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a8fccf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140a8fcd5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140a8fcdb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140a8fce1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140a8fce7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140a8fced: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140a8fcf6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140a8fcff: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140a8fd08: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140a8fd11: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140a8fd19: 48 8d 8d b8 09 00 00        	leaq	0x9b8(%rbp), %rcx
140a8fd20: e8 6b f6 de ff              	callq	0x14087f390 <.text+0x87e390>
140a8fd25: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140a8fd2d: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140a8fd36: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140a8fd3f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140a8fd48: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140a8fd51: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140a8fd57: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140a8fd5d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140a8fd63: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140a8fd69: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140a8fd6f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140a8fd76: 5b                          	popq	%rbx
140a8fd77: 5f                          	popq	%rdi
140a8fd78: 5e                          	popq	%rsi
140a8fd79: 41 5c                       	popq	%r12
140a8fd7b: 41 5d                       	popq	%r13
140a8fd7d: 41 5e                       	popq	%r14
140a8fd7f: 41 5f                       	popq	%r15
140a8fd81: 5d                          	popq	%rbp
140a8fd82: c3                          	retq
