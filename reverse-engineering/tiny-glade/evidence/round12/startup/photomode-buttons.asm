
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14234c230: 55                          	pushq	%rbp
14234c231: 41 57                       	pushq	%r15
14234c233: 41 56                       	pushq	%r14
14234c235: 41 55                       	pushq	%r13
14234c237: 41 54                       	pushq	%r12
14234c239: 56                          	pushq	%rsi
14234c23a: 57                          	pushq	%rdi
14234c23b: 53                          	pushq	%rbx
14234c23c: 48 81 ec a8 02 00 00        	subq	$0x2a8, %rsp            # imm = 0x2A8
14234c243: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14234c24b: 44 0f 29 85 10 02 00 00     	movaps	%xmm8, 0x210(%rbp)
14234c253: 0f 29 bd 00 02 00 00        	movaps	%xmm7, 0x200(%rbp)
14234c25a: 0f 29 b5 f0 01 00 00        	movaps	%xmm6, 0x1f0(%rbp)
14234c261: 48 c7 85 e8 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1e8(%rbp)
14234c26c: 48 8b b5 c0 02 00 00        	movq	0x2c0(%rbp), %rsi
14234c273: 48 8b 46 20                 	movq	0x20(%rsi), %rax
14234c277: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
14234c27e: 4c 8b 56 30                 	movq	0x30(%rsi), %r10
14234c282: 44 8b 5e 3c                 	movl	0x3c(%rsi), %r11d
14234c286: f3 0f 6f 46 40              	movdqu	0x40(%rsi), %xmm0
14234c28b: f3 0f 6f 4e 50              	movdqu	0x50(%rsi), %xmm1
14234c290: 66 0f 7f 4d e0              	movdqa	%xmm1, -0x20(%rbp)
14234c295: 66 0f 7f 45 d0              	movdqa	%xmm0, -0x30(%rbp)
14234c29a: 48 8b 01                    	movq	(%rcx), %rax
14234c29d: 83 38 13                    	cmpl	$0x13, (%rax)
14234c2a0: 0f 85 06 13 00 00           	jne	0x14234d5ac <.text+0x234c5ac>
14234c2a6: 48 8b 85 d8 02 00 00        	movq	0x2d8(%rbp), %rax
14234c2ad: 48 8b 9d d0 02 00 00        	movq	0x2d0(%rbp), %rbx
14234c2b4: 48 8b 00                    	movq	(%rax), %rax
14234c2b7: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
14234c2be: 80 38 00                    	cmpb	$0x0, (%rax)
14234c2c1: 4c 89 95 58 01 00 00        	movq	%r10, 0x158(%rbp)
14234c2c8: 44 89 9d 9c 01 00 00        	movl	%r11d, 0x19c(%rbp)
14234c2cf: 0f 84 3b 01 00 00           	je	0x14234c410 <.text+0x234b410>
14234c2d5: 4d 89 c5                    	movq	%r8, %r13
14234c2d8: 48 89 95 b0 01 00 00        	movq	%rdx, 0x1b0(%rbp)
14234c2df: 4d 89 cc                    	movq	%r9, %r12
14234c2e2: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
14234c2e9: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14234c2ec: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14234c2f0: 89 0a                       	movl	%ecx, (%rdx)
14234c2f2: 48 8b 38                    	movq	(%rax), %rdi
14234c2f5: 48 8d 05 f5 2f d0 00        	leaq	0xd02ff5(%rip), %rax    # 0x14304f2f1
14234c2fc: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
14234c303: 48 c7 85 b8 00 00 00 18 00 00 00    	movq	$0x18, 0xb8(%rbp)
14234c30e: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234c315: e8 86 1a 27 fe              	callq	0x1405bdda0 <.text+0x5bcda0>
14234c31a: 48 be 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rsi # imm = 0xB1695320004FB443
14234c324: 48 31 c6                    	xorq	%rax, %rsi
14234c327: 48 89 f9                    	movq	%rdi, %rcx
14234c32a: 48 89 f2                    	movq	%rsi, %rdx
14234c32d: e8 8e 7b 33 fe              	callq	0x140683ec0 <.text+0x682ec0>
14234c332: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
14234c336: c6 80 b7 00 00 00 01        	movb	$0x1, 0xb7(%rax)
14234c33d: f6 c2 01                    	testb	$0x1, %dl
14234c340: 75 33                       	jne	0x14234c375 <.text+0x234b375>
14234c342: 4c 8b 77 10                 	movq	0x10(%rdi), %r14
14234c346: 4c 3b 37                    	cmpq	(%rdi), %r14
14234c349: 75 0f                       	jne	0x14234c35a <.text+0x234b35a>
14234c34b: 48 8d 15 76 2c 86 00        	leaq	0x862c76(%rip), %rdx    # 0x142baefc8
14234c352: 48 89 f9                    	movq	%rdi, %rcx
14234c355: e8 76 56 2e fe              	callq	0x1406319d0 <.text+0x6309d0>
14234c35a: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14234c35e: 49 6b ce 38                 	imulq	$0x38, %r14, %rcx
14234c362: c7 04 08 01 00 00 00        	movl	$0x1, (%rax,%rcx)
14234c369: 48 89 74 08 30              	movq	%rsi, 0x30(%rax,%rcx)
14234c36e: 49 ff c6                    	incq	%r14
14234c371: 4c 89 77 10                 	movq	%r14, 0x10(%rdi)
14234c375: 48 89 7d f0                 	movq	%rdi, -0x10(%rbp)
14234c379: 48 89 75 f8                 	movq	%rsi, -0x8(%rbp)
14234c37d: 48 c7 45 00 00 00 00 00     	movq	$0x0, (%rbp)
14234c385: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
14234c390: 49 be 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %r14 # imm = 0x41223169FF28813B
14234c39a: 4c 89 b5 b8 00 00 00        	movq	%r14, 0xb8(%rbp)
14234c3a1: 49 bf d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %r15 # imm = 0xA79B7268A2A968D9
14234c3ab: 4c 89 bd c0 00 00 00        	movq	%r15, 0xc0(%rbp)
14234c3b2: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234c3b9: e8 62 1b 27 fe              	callq	0x1405bdf20 <.text+0x5bcf20>
14234c3be: 48 89 b5 b0 00 00 00        	movq	%rsi, 0xb0(%rbp)
14234c3c5: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
14234c3cc: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234c3d3: e8 38 15 27 fe              	callq	0x1405bd910 <.text+0x5bc910>
14234c3d8: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14234c3dc: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
14234c3e0: 49 89 c0                    	movq	%rax, %r8
14234c3e3: e8 c8 9d 33 fe              	callq	0x1406861b0 <.text+0x6851b0>
14234c3e8: 48 8b 03                    	movq	(%rbx), %rax
14234c3eb: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
14234c3f2: 0f b6 00                    	movzbl	(%rax), %eax
14234c3f5: 85 c0                       	testl	%eax, %eax
14234c3f7: 74 26                       	je	0x14234c41f <.text+0x234b41f>
14234c3f9: 83 f8 01                    	cmpl	$0x1, %eax
14234c3fc: 0f 85 ec 11 00 00           	jne	0x14234d5ee <.text+0x234c5ee>
14234c402: 48 8d 35 18 2f d0 00        	leaq	0xd02f18(%rip), %rsi    # 0x14304f321
14234c409: bf 12 00 00 00              	movl	$0x12, %edi
14234c40e: eb 1b                       	jmp	0x14234c42b <.text+0x234b42b>
14234c410: 48 8b 03                    	movq	(%rbx), %rax
14234c413: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
14234c41a: e9 ce 01 00 00              	jmp	0x14234c5ed <.text+0x234b5ed>
14234c41f: 48 8d 35 e3 2e d0 00        	leaq	0xd02ee3(%rip), %rsi    # 0x14304f309
14234c426: bf 18 00 00 00              	movl	$0x18, %edi
14234c42b: 48 8b 5d 28                 	movq	0x28(%rbp), %rbx
14234c42f: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
14234c43a: 4c 89 b5 b8 00 00 00        	movq	%r14, 0xb8(%rbp)
14234c441: 4c 89 bd c0 00 00 00        	movq	%r15, 0xc0(%rbp)
14234c448: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234c44f: e8 cc 1a 27 fe              	callq	0x1405bdf20 <.text+0x5bcf20>
14234c454: 48 89 9d b0 00 00 00        	movq	%rbx, 0xb0(%rbp)
14234c45b: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
14234c462: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234c469: e8 a2 14 27 fe              	callq	0x1405bd910 <.text+0x5bc910>
14234c46e: 48 8d 9d b0 00 00 00        	leaq	0xb0(%rbp), %rbx
14234c475: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
14234c479: 48 89 d9                    	movq	%rbx, %rcx
14234c47c: 49 89 c0                    	movq	%rax, %r8
14234c47f: e8 2c 9d 33 fe              	callq	0x1406861b0 <.text+0x6851b0>
14234c484: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
14234c48d: 4c 8d 75 a8                 	leaq	-0x58(%rbp), %r14
14234c491: 4c 89 f1                    	movq	%r14, %rcx
14234c494: 48 89 da                    	movq	%rbx, %rdx
14234c497: 49 89 f0                    	movq	%rsi, %r8
14234c49a: 49 89 f9                    	movq	%rdi, %r9
14234c49d: e8 ce 24 c5 fe              	callq	0x140f9e970 <.text+0xf9d970>
14234c4a2: 4c 8d 05 8a 2e d0 00        	leaq	0xd02e8a(%rip), %r8     # 0x14304f333
14234c4a9: 48 8d 75 08                 	leaq	0x8(%rbp), %rsi
14234c4ad: 41 b9 06 00 00 00           	movl	$0x6, %r9d
14234c4b3: 48 89 f1                    	movq	%rsi, %rcx
14234c4b6: 4c 89 f2                    	movq	%r14, %rdx
14234c4b9: e8 b2 bb c6 fe              	callq	0x140fb8070 <.text+0xfb7070>
14234c4be: 48 8d 0d 74 2e d0 00        	leaq	0xd02e74(%rip), %rcx    # 0x14304f339
14234c4c5: ba 02 00 00 00              	movl	$0x2, %edx
14234c4ca: e8 b1 0f 85 fe              	callq	0x140b9d480 <.text+0xb9c480>
14234c4cf: 49 89 d1                    	movq	%rdx, %r9
14234c4d2: 48 8d bd b0 00 00 00        	leaq	0xb0(%rbp), %rdi
14234c4d9: 48 89 f9                    	movq	%rdi, %rcx
14234c4dc: 48 89 f2                    	movq	%rsi, %rdx
14234c4df: 49 89 c0                    	movq	%rax, %r8
14234c4e2: e8 d9 fa 33 fe              	callq	0x14068bfc0 <.text+0x68afc0>
14234c4e7: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
14234c4ee: 48 89 fa                    	movq	%rdi, %rdx
14234c4f1: 41 b0 13                    	movb	$0x13, %r8b
14234c4f4: e8 d7 21 33 fe              	callq	0x14067e6d0 <.text+0x67d6d0>
14234c4f9: 48 8b b5 18 01 00 00        	movq	0x118(%rbp), %rsi
14234c500: 48 83 be b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rsi)
14234c508: 0f 84 c8 10 00 00           	je	0x14234d5d6 <.text+0x234c5d6>
14234c50e: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
14234c515: 48 8d 8e c0 01 00 00        	leaq	0x1c0(%rsi), %rcx
14234c51c: e8 4f 48 26 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14234c521: 48 8b 8e a0 01 00 00        	movq	0x1a0(%rsi), %rcx
14234c528: 4c 8b 96 a8 01 00 00        	movq	0x1a8(%rsi), %r10
14234c52f: 4d 89 d3                    	movq	%r10, %r11
14234c532: 49 21 c3                    	andq	%rax, %r11
14234c535: 48 c1 e8 39                 	shrq	$0x39, %rax
14234c539: 66 0f 6e c0                 	movd	%eax, %xmm0
14234c53d: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14234c541: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14234c546: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14234c54b: 48 8d 81 40 ff ff ff        	leaq	-0xc0(%rcx), %rax
14234c552: 31 f6                       	xorl	%esi, %esi
14234c554: 48 8b bd 20 01 00 00        	movq	0x120(%rbp), %rdi
14234c55b: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14234c55f: f3 42 0f 6f 14 19           	movdqu	(%rcx,%r11), %xmm2
14234c565: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14234c569: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14234c56d: 66 0f d7 db                 	pmovmskb	%xmm3, %ebx
14234c571: 85 db                       	testl	%ebx, %ebx
14234c573: 48 8b 95 b0 01 00 00        	movq	0x1b0(%rbp), %rdx
14234c57a: 74 2f                       	je	0x14234c5ab <.text+0x234b5ab>
14234c57c: 4d 89 e1                    	movq	%r12, %r9
14234c57f: 4d 89 e8                    	movq	%r13, %r8
14234c582: f3 44 0f bc f3              	tzcntl	%ebx, %r14d
14234c587: 4d 01 de                    	addq	%r11, %r14
14234c58a: 4d 21 d6                    	andq	%r10, %r14
14234c58d: 49 f7 de                    	negq	%r14
14234c590: 4f 8d 34 76                 	leaq	(%r14,%r14,2), %r14
14234c594: 49 c1 e6 06                 	shlq	$0x6, %r14
14234c598: 4a 3b 3c 30                 	cmpq	(%rax,%r14), %rdi
14234c59c: 74 2d                       	je	0x14234c5cb <.text+0x234b5cb>
14234c59e: 44 8d 73 ff                 	leal	-0x1(%rbx), %r14d
14234c5a2: 66 41 21 de                 	andw	%bx, %r14w
14234c5a6: 44 89 f3                    	movl	%r14d, %ebx
14234c5a9: 75 d7                       	jne	0x14234c582 <.text+0x234b582>
14234c5ab: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14234c5af: 66 0f d7 d2                 	pmovmskb	%xmm2, %edx
14234c5b3: 85 d2                       	testl	%edx, %edx
14234c5b5: 0f 85 1b 10 00 00           	jne	0x14234d5d6 <.text+0x234c5d6>
14234c5bb: 49 01 f3                    	addq	%rsi, %r11
14234c5be: 49 83 c3 10                 	addq	$0x10, %r11
14234c5c2: 48 83 c6 10                 	addq	$0x10, %rsi
14234c5c6: 4d 21 d3                    	andq	%r10, %r11
14234c5c9: eb 94                       	jmp	0x14234c55f <.text+0x234b55f>
14234c5cb: 42 f6 44 31 fe 30           	testb	$0x30, -0x2(%rcx,%r14)
14234c5d1: 74 1a                       	je	0x14234c5ed <.text+0x234b5ed>
14234c5d3: 48 8b 8d d8 02 00 00        	movq	0x2d8(%rbp), %rcx
14234c5da: 8b 41 1c                    	movl	0x1c(%rcx), %eax
14234c5dd: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
14234c5e1: 89 01                       	movl	%eax, (%rcx)
14234c5e3: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14234c5ea: c6 00 00                    	movb	$0x0, (%rax)
14234c5ed: 48 8b 8d b8 02 00 00        	movq	0x2b8(%rbp), %rcx
14234c5f4: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
14234c5fb: 4c 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %r10
14234c602: 49 8b 38                    	movq	(%r8), %rdi
14234c605: 49 8b 58 08                 	movq	0x8(%r8), %rbx
14234c609: 4c 8b 07                    	movq	(%rdi), %r8
14234c60c: 31 f6                       	xorl	%esi, %esi
14234c60e: 4d 89 c6                    	movq	%r8, %r14
14234c611: 4c 2b 73 18                 	subq	0x18(%rbx), %r14
14234c615: 4c 0f 42 f6                 	cmovbq	%rsi, %r14
14234c619: 4c 2b 43 38                 	subq	0x38(%rbx), %r8
14234c61d: 4c 0f 42 c6                 	cmovbq	%rsi, %r8
14234c621: 4d 89 f7                    	movq	%r14, %r15
14234c624: 49 c1 e7 04                 	shlq	$0x4, %r15
14234c628: 4c 03 7b 08                 	addq	0x8(%rbx), %r15
14234c62c: 4c 8b 6b 10                 	movq	0x10(%rbx), %r13
14234c630: 4c 8b 5b 30                 	movq	0x30(%rbx), %r11
14234c634: 4d 29 f5                    	subq	%r14, %r13
14234c637: 4c 0f 42 ee                 	cmovbq	%rsi, %r13
14234c63b: 41 be 08 00 00 00           	movl	$0x8, %r14d
14234c641: 4d 0f 42 fe                 	cmovbq	%r14, %r15
14234c645: 4d 89 c4                    	movq	%r8, %r12
14234c648: 49 c1 e4 04                 	shlq	$0x4, %r12
14234c64c: 4c 03 63 28                 	addq	0x28(%rbx), %r12
14234c650: 4d 29 c3                    	subq	%r8, %r11
14234c653: 4c 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %r8
14234c65a: 4c 0f 42 de                 	cmovbq	%rsi, %r11
14234c65e: 4d 0f 42 e6                 	cmovbq	%r14, %r12
14234c662: 4c 8b b5 98 02 00 00        	movq	0x298(%rbp), %r14
14234c669: 48 8b 5b 40                 	movq	0x40(%rbx), %rbx
14234c66d: 4c 29 db                    	subq	%r11, %rbx
14234c670: 4c 29 eb                    	subq	%r13, %rbx
14234c673: 48 8b b5 90 02 00 00        	movq	0x290(%rbp), %rsi
14234c67a: 48 89 1f                    	movq	%rbx, (%rdi)
14234c67d: 49 c1 e5 04                 	shlq	$0x4, %r13
14234c681: 4c 89 ad b8 01 00 00        	movq	%r13, 0x1b8(%rbp)
14234c688: 4c 8b ad c8 02 00 00        	movq	0x2c8(%rbp), %r13
14234c68f: 49 c1 e3 04                 	shlq	$0x4, %r11
14234c693: 4c 89 9d b0 01 00 00        	movq	%r11, 0x1b0(%rbp)
14234c69a: 4c 8b 1e                    	movq	(%rsi), %r11
14234c69d: 4c 89 9d 90 00 00 00        	movq	%r11, 0x90(%rbp)
14234c6a4: 49 8b 5d 08                 	movq	0x8(%r13), %rbx
14234c6a8: 4c 8b 9d c0 02 00 00        	movq	0x2c0(%rbp), %r11
14234c6af: 4d 8b 1b                    	movq	(%r11), %r11
14234c6b2: 4c 89 9d a8 01 00 00        	movq	%r11, 0x1a8(%rbp)
14234c6b9: 48 89 9d d8 01 00 00        	movq	%rbx, 0x1d8(%rbp)
14234c6c0: 48 85 db                    	testq	%rbx, %rbx
14234c6c3: 75 0b                       	jne	0x14234c6d0 <.text+0x234b6d0>
14234c6c5: 4d 8b 5d 10                 	movq	0x10(%r13), %r11
14234c6c9: 4c 89 9d d8 01 00 00        	movq	%r11, 0x1d8(%rbp)
14234c6d0: 4c 01 bd b8 01 00 00        	addq	%r15, 0x1b8(%rbp)
14234c6d7: 4c 01 a5 b0 01 00 00        	addq	%r12, 0x1b0(%rbp)
14234c6de: 4c 8b 2a                    	movq	(%rdx), %r13
14234c6e1: 48 8b 5a 08                 	movq	0x8(%rdx), %rbx
14234c6e5: 8b 52 14                    	movl	0x14(%rdx), %edx
14234c6e8: 89 95 c4 01 00 00           	movl	%edx, 0x1c4(%rbp)
14234c6ee: 41 8b 52 1c                 	movl	0x1c(%r10), %edx
14234c6f2: 89 95 3c 01 00 00           	movl	%edx, 0x13c(%rbp)
14234c6f8: 49 8b 12                    	movq	(%r10), %rdx
14234c6fb: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
14234c6ff: 49 8b 52 10                 	movq	0x10(%r10), %rdx
14234c703: 48 89 55 40                 	movq	%rdx, 0x40(%rbp)
14234c707: 41 8b 51 1c                 	movl	0x1c(%r9), %edx
14234c70b: 89 95 40 01 00 00           	movl	%edx, 0x140(%rbp)
14234c711: 49 8b 11                    	movq	(%r9), %rdx
14234c714: 48 89 55 48                 	movq	%rdx, 0x48(%rbp)
14234c718: 49 8b 51 10                 	movq	0x10(%r9), %rdx
14234c71c: 48 89 55 50                 	movq	%rdx, 0x50(%rbp)
14234c720: 41 8b 56 1c                 	movl	0x1c(%r14), %edx
14234c724: 89 95 44 01 00 00           	movl	%edx, 0x144(%rbp)
14234c72a: 49 8b 16                    	movq	(%r14), %rdx
14234c72d: 48 89 95 60 01 00 00        	movq	%rdx, 0x160(%rbp)
14234c734: 49 8b 56 10                 	movq	0x10(%r14), %rdx
14234c738: 48 89 55 58                 	movq	%rdx, 0x58(%rbp)
14234c73c: 8b 56 1c                    	movl	0x1c(%rsi), %edx
14234c73f: 89 95 48 01 00 00           	movl	%edx, 0x148(%rbp)
14234c745: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
14234c749: 48 89 55 60                 	movq	%rdx, 0x60(%rbp)
14234c74d: 8b 51 1c                    	movl	0x1c(%rcx), %edx
14234c750: 89 95 88 01 00 00           	movl	%edx, 0x188(%rbp)
14234c756: 48 8b 11                    	movq	(%rcx), %rdx
14234c759: 48 89 95 90 01 00 00        	movq	%rdx, 0x190(%rbp)
14234c760: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
14234c764: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
14234c76b: 4c 8b 18                    	movq	(%rax), %r11
14234c76e: 48 8b 48 10                 	movq	0x10(%rax), %rcx
14234c772: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
14234c776: 8b 40 1c                    	movl	0x1c(%rax), %eax
14234c779: 89 85 4c 01 00 00           	movl	%eax, 0x14c(%rbp)
14234c77f: 41 8b 40 1c                 	movl	0x1c(%r8), %eax
14234c783: 89 85 50 01 00 00           	movl	%eax, 0x150(%rbp)
14234c789: 49 8b 00                    	movq	(%r8), %rax
14234c78c: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14234c790: 49 8b 40 10                 	movq	0x10(%r8), %rax
14234c794: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
14234c798: 48 8b 8d d8 02 00 00        	movq	0x2d8(%rbp), %rcx
14234c79f: 8b 41 1c                    	movl	0x1c(%rcx), %eax
14234c7a2: 89 85 8c 01 00 00           	movl	%eax, 0x18c(%rbp)
14234c7a8: 48 8b 41 10                 	movq	0x10(%rcx), %rax
14234c7ac: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
14234c7b3: 48 8b 8d c0 02 00 00        	movq	0x2c0(%rbp), %rcx
14234c7ba: 48 8b 41 10                 	movq	0x10(%rcx), %rax
14234c7be: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
14234c7c5: 8b 41 1c                    	movl	0x1c(%rcx), %eax
14234c7c8: 89 85 54 01 00 00           	movl	%eax, 0x154(%rbp)
14234c7ce: f2 0f 10 35 da 18 d0 00     	movsd	0xd018da(%rip), %xmm6   # 0x14304e0b0
14234c7d6: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
14234c7dd: 48 83 c0 09                 	addq	$0x9, %rax
14234c7e1: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
14234c7e8: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
14234c7ef: 48 83 c0 20                 	addq	$0x20, %rax
14234c7f3: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14234c7fa: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
14234c801: 48 83 c0 20                 	addq	$0x20, %rax
14234c805: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14234c80c: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234c812: 4c 89 9d d0 01 00 00        	movq	%r11, 0x1d0(%rbp)
14234c819: 48 8b b5 b8 01 00 00        	movq	0x1b8(%rbp), %rsi
14234c820: 4c 89 fa                    	movq	%r15, %rdx
14234c823: 4c 89 e1                    	movq	%r12, %rcx
14234c826: eb 37                       	jmp	0x14234c85f <.text+0x234b85f>
14234c828: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14234c830: 45 31 c0                    	xorl	%r8d, %r8d
14234c833: 49 8b 95 20 01 00 00        	movq	0x120(%r13), %rdx
14234c83a: 48 39 91 88 00 00 00        	cmpq	%rdx, 0x88(%rcx)
14234c841: 0f 86 59 01 00 00           	jbe	0x14234c9a0 <.text+0x234b9a0>
14234c847: 48 8b 89 80 00 00 00        	movq	0x80(%rcx), %rcx
14234c84e: 48 83 3c d1 00              	cmpq	$0x0, (%rcx,%rdx,8)
14234c853: 4c 89 fa                    	movq	%r15, %rdx
14234c856: 4c 89 e1                    	movq	%r12, %rcx
14234c859: 0f 84 41 01 00 00           	je	0x14234c9a0 <.text+0x234b9a0>
14234c85f: 48 85 d2                    	testq	%rdx, %rdx
14234c862: 74 1c                       	je	0x14234c880 <.text+0x234b880>
14234c864: 48 39 f2                    	cmpq	%rsi, %rdx
14234c867: 74 17                       	je	0x14234c880 <.text+0x234b880>
14234c869: 4c 8d 7a 10                 	leaq	0x10(%rdx), %r15
14234c86d: 49 89 cc                    	movq	%rcx, %r12
14234c870: 49 89 d6                    	movq	%rdx, %r14
14234c873: eb 2b                       	jmp	0x14234c8a0 <.text+0x234b8a0>
14234c875: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14234c880: 48 85 c9                    	testq	%rcx, %rcx
14234c883: 0f 84 18 08 00 00           	je	0x14234d0a1 <.text+0x234c0a1>
14234c889: 48 3b 8d b0 01 00 00        	cmpq	0x1b0(%rbp), %rcx
14234c890: 0f 84 0b 08 00 00           	je	0x14234d0a1 <.text+0x234c0a1>
14234c896: 4c 8d 61 10                 	leaq	0x10(%rcx), %r12
14234c89a: 45 31 ff                    	xorl	%r15d, %r15d
14234c89d: 49 89 ce                    	movq	%rcx, %r14
14234c8a0: 48 ff 07                    	incq	(%rdi)
14234c8a3: 49 8b 46 08                 	movq	0x8(%r14), %rax
14234c8a7: 89 c1                       	movl	%eax, %ecx
14234c8a9: 48 3b 4b 10                 	cmpq	0x10(%rbx), %rcx
14234c8ad: 0f 83 6d ff ff ff           	jae	0x14234c820 <.text+0x234b820>
14234c8b3: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
14234c8b7: 48 c1 e8 20                 	shrq	$0x20, %rax
14234c8bb: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14234c8bf: 4c 8d 04 8a                 	leaq	(%rdx,%rcx,4), %r8
14234c8c3: 39 04 8a                    	cmpl	%eax, (%rdx,%rcx,4)
14234c8c6: 0f 85 54 ff ff ff           	jne	0x14234c820 <.text+0x234b820>
14234c8cc: 41 8b 48 04                 	movl	0x4(%r8), %ecx
14234c8d0: 4c 39 d1                    	cmpq	%r10, %rcx
14234c8d3: 0f 84 47 ff ff ff           	je	0x14234c820 <.text+0x234b820>
14234c8d9: 41 8b 50 0c                 	movl	0xc(%r8), %edx
14234c8dd: 41 8b 40 10                 	movl	0x10(%r8), %eax
14234c8e1: 49 39 4d 38                 	cmpq	%rcx, 0x38(%r13)
14234c8e5: 0f 86 35 ff ff ff           	jbe	0x14234c820 <.text+0x234b820>
14234c8eb: 4d 8b 45 28                 	movq	0x28(%r13), %r8
14234c8ef: 41 89 c9                    	movl	%ecx, %r9d
14234c8f2: 41 c1 e9 06                 	shrl	$0x6, %r9d
14234c8f6: 4f 8b 04 c8                 	movq	(%r8,%r9,8), %r8
14234c8fa: 49 0f a3 c8                 	btq	%rcx, %r8
14234c8fe: 0f 83 1c ff ff ff           	jae	0x14234c820 <.text+0x234b820>
14234c904: 48 39 8b 08 01 00 00        	cmpq	%rcx, 0x108(%rbx)
14234c90b: 76 13                       	jbe	0x14234c920 <.text+0x234b920>
14234c90d: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14234c911: 48 c1 e1 05                 	shlq	$0x5, %rcx
14234c915: 48 03 8b 00 01 00 00        	addq	0x100(%rbx), %rcx
14234c91c: eb 04                       	jmp	0x14234c922 <.text+0x234b922>
14234c91e: 66 90                       	nop
14234c920: 31 c9                       	xorl	%ecx, %ecx
14234c922: 4c 8b 83 a8 01 00 00        	movq	0x1a8(%rbx), %r8
14234c929: 48 8d 14 d2                 	leaq	(%rdx,%rdx,8), %rdx
14234c92d: 4d 8b 8d 10 01 00 00        	movq	0x110(%r13), %r9
14234c934: 4d 39 4c d0 40              	cmpq	%r9, 0x40(%r8,%rdx,8)
14234c939: 0f 86 f1 fe ff ff           	jbe	0x14234c830 <.text+0x234b830>
14234c93f: 4d 8b 54 d0 38              	movq	0x38(%r8,%rdx,8), %r10
14234c944: 4f 8b 0c ca                 	movq	(%r10,%r9,8), %r9
14234c948: 4d 85 c9                    	testq	%r9, %r9
14234c94b: 74 34                       	je	0x14234c981 <.text+0x234b981>
14234c94d: 49 8d 14 d0                 	leaq	(%r8,%rdx,8), %rdx
14234c951: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14234c955: 49 f7 d1                    	notq	%r9
14234c958: 4f 8d 04 49                 	leaq	(%r9,%r9,2), %r8
14234c95c: 49 c1 e0 04                 	shlq	$0x4, %r8
14234c960: 4e 8b 44 02 10              	movq	0x10(%rdx,%r8), %r8
14234c965: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234c96b: 49 8b 95 20 01 00 00        	movq	0x120(%r13), %rdx
14234c972: 48 39 91 88 00 00 00        	cmpq	%rdx, 0x88(%rcx)
14234c979: 0f 87 c8 fe ff ff           	ja	0x14234c847 <.text+0x234b847>
14234c97f: eb 1f                       	jmp	0x14234c9a0 <.text+0x234b9a0>
14234c981: 45 31 c0                    	xorl	%r8d, %r8d
14234c984: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234c98a: 49 8b 95 20 01 00 00        	movq	0x120(%r13), %rdx
14234c991: 48 39 91 88 00 00 00        	cmpq	%rdx, 0x88(%rcx)
14234c998: 0f 87 a9 fe ff ff           	ja	0x14234c847 <.text+0x234b847>
14234c99e: 66 90                       	nop
14234c9a0: 41 0f b6 34 00              	movzbl	(%r8,%rax), %esi
14234c9a5: 8d 46 fc                    	leal	-0x4(%rsi), %eax
14234c9a8: 3c 0e                       	cmpb	$0xe, %al
14234c9aa: 0f b6 c0                    	movzbl	%al, %eax
14234c9ad: b9 02 00 00 00              	movl	$0x2, %ecx
14234c9b2: 0f 43 c1                    	cmovael	%ecx, %eax
14234c9b5: 0f b6 c0                    	movzbl	%al, %eax
14234c9b8: 48 8d 0d e9 29 d0 00        	leaq	0xd029e9(%rip), %rcx    # 0x14304f3a8
14234c9bf: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
14234c9c3: 48 01 c8                    	addq	%rcx, %rax
14234c9c6: ff e0                       	jmpq	*%rax
14234c9c8: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14234c9cf: 0f b6 00                    	movzbl	(%rax), %eax
14234c9d2: 85 c0                       	testl	%eax, %eax
14234c9d4: 74 09                       	je	0x14234c9df <.text+0x234b9df>
14234c9d6: 83 f8 01                    	cmpl	$0x1, %eax
14234c9d9: 0f 85 d3 04 00 00           	jne	0x14234ceb2 <.text+0x234beb2>
14234c9df: 8b 85 8c 01 00 00           	movl	0x18c(%rbp), %eax
14234c9e5: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14234c9ec: 89 01                       	movl	%eax, (%rcx)
14234c9ee: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14234c9f5: c6 00 01                    	movb	$0x1, (%rax)
14234c9f8: e9 53 04 00 00              	jmp	0x14234ce50 <.text+0x234be50>
14234c9fd: 8b 85 48 01 00 00           	movl	0x148(%rbp), %eax
14234ca03: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
14234ca07: 89 01                       	movl	%eax, (%rcx)
14234ca09: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
14234ca10: 80 70 0d 01                 	xorb	$0x1, 0xd(%rax)
14234ca14: e9 37 04 00 00              	jmp	0x14234ce50 <.text+0x234be50>
14234ca19: 48 8b 05 78 f6 2d 01        	movq	0x12df678(%rip), %rax   # 0x14362c098
14234ca20: 80 38 00                    	cmpb	$0x0, (%rax)
14234ca23: 0f 84 4c 03 00 00           	je	0x14234cd75 <.text+0x234bd75>
14234ca29: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
14234ca30: e8 7b cc 80 fe              	callq	0x140b596b0 <.text+0xb586b0>
14234ca35: e9 47 03 00 00              	jmp	0x14234cd81 <.text+0x234bd81>
14234ca3a: 8b 85 44 01 00 00           	movl	0x144(%rbp), %eax
14234ca40: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
14234ca44: 89 01                       	movl	%eax, (%rcx)
14234ca46: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
14234ca4d: e8 ee 47 5c fe              	callq	0x140911240 <.text+0x910240>
14234ca52: 8b 10                       	movl	(%rax), %edx
14234ca54: 83 c2 fe                    	addl	$-0x2, %edx
14234ca57: 83 fa 02                    	cmpl	$0x2, %edx
14234ca5a: b9 02 00 00 00              	movl	$0x2, %ecx
14234ca5f: 0f 43 d1                    	cmovael	%ecx, %edx
14234ca62: 85 d2                       	testl	%edx, %edx
14234ca64: 0f 84 c4 03 00 00           	je	0x14234ce2e <.text+0x234be2e>
14234ca6a: 83 fa 01                    	cmpl	$0x1, %edx
14234ca6d: 0f 85 b0 03 00 00           	jne	0x14234ce23 <.text+0x234be23>
14234ca73: f2 0f 10 78 04              	movsd	0x4(%rax), %xmm7
14234ca78: 31 c9                       	xorl	%ecx, %ecx
14234ca7a: e9 af 03 00 00              	jmp	0x14234ce2e <.text+0x234be2e>
14234ca7f: 8b 85 88 01 00 00           	movl	0x188(%rbp), %eax
14234ca85: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
14234ca8c: 89 01                       	movl	%eax, (%rcx)
14234ca8e: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234ca95: 48 8b 95 90 01 00 00        	movq	0x190(%rbp), %rdx
14234ca9c: e8 bf 1f 56 fe              	callq	0x1408aea60 <.text+0x8ada60>
14234caa1: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234caa7: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
14234caae: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
14234cab5: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
14234cabc: eb 09                       	jmp	0x14234cac7 <.text+0x234bac7>
14234cabe: 66 90                       	nop
14234cac0: 48 83 c0 40                 	addq	$0x40, %rax
14234cac4: 48 ff c2                    	incq	%rdx
14234cac7: 48 39 c8                    	cmpq	%rcx, %rax
14234caca: 74 4d                       	je	0x14234cb19 <.text+0x234bb19>
14234cacc: 48 85 d2                    	testq	%rdx, %rdx
14234cacf: 41 0f 95 c0                 	setne	%r8b
14234cad3: 48 85 c0                    	testq	%rax, %rax
14234cad6: 41 0f 95 c1                 	setne	%r9b
14234cada: 45 84 c8                    	testb	%r9b, %r8b
14234cadd: 74 e1                       	je	0x14234cac0 <.text+0x234bac0>
14234cadf: 80 70 3c 01                 	xorb	$0x1, 0x3c(%rax)
14234cae3: eb db                       	jmp	0x14234cac0 <.text+0x234bac0>
14234cae5: 8b 85 50 01 00 00           	movl	0x150(%rbp), %eax
14234caeb: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14234caef: 89 01                       	movl	%eax, (%rcx)
14234caf1: c6 85 b8 00 00 00 00        	movb	$0x0, 0xb8(%rbp)
14234caf8: 48 c7 85 b0 00 00 00 0a 00 00 00    	movq	$0xa, 0xb0(%rbp)
14234cb03: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14234cb07: 48 8d 95 b0 00 00 00        	leaq	0xb0(%rbp), %rdx
14234cb0e: e8 bd 2f 5f fe              	callq	0x14093fad0 <.text+0x93ead0>
14234cb13: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234cb19: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234cb20: e9 2b 03 00 00              	jmp	0x14234ce50 <.text+0x234be50>
14234cb25: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14234cb2c: 8b 8d 9c 01 00 00           	movl	0x19c(%rbp), %ecx
14234cb32: 89 08                       	movl	%ecx, (%rax)
14234cb34: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
14234cb3b: 4c 8b 40 30                 	movq	0x30(%rax), %r8
14234cb3f: 4c 8b 48 40                 	movq	0x40(%rax), %r9
14234cb43: 4c 3b 40 20                 	cmpq	0x20(%rax), %r8
14234cb47: 75 3c                       	jne	0x14234cb85 <.text+0x234bb85>
14234cb49: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14234cb50: 48 8d 15 19 87 86 00        	leaq	0x868719(%rip), %rdx    # 0x142bb5270
14234cb57: 4c 89 85 a0 01 00 00        	movq	%r8, 0x1a0(%rbp)
14234cb5e: 4c 89 8d 78 01 00 00        	movq	%r9, 0x178(%rbp)
14234cb65: e8 c6 5a 21 fe              	callq	0x140562630 <.text+0x561630>
14234cb6a: 4c 8b 8d 78 01 00 00        	movq	0x178(%rbp), %r9
14234cb71: 4c 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %r8
14234cb78: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234cb7e: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234cb85: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
14234cb8c: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14234cb90: 4c 89 c1                    	movq	%r8, %rcx
14234cb93: 48 c1 e1 04                 	shlq	$0x4, %rcx
14234cb97: 4c 89 0c 08                 	movq	%r9, (%rax,%rcx)
14234cb9b: 40 88 74 08 08              	movb	%sil, 0x8(%rax,%rcx)
14234cba0: 49 ff c0                    	incq	%r8
14234cba3: 4c 89 42 30                 	movq	%r8, 0x30(%rdx)
14234cba7: e9 28 01 00 00              	jmp	0x14234ccd4 <.text+0x234bcd4>
14234cbac: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234cbb3: e8 78 9f 73 fe              	callq	0x140a86b30 <.text+0xa85b30>
14234cbb8: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
14234cbbf: 8b 8d 54 01 00 00           	movl	0x154(%rbp), %ecx
14234cbc5: 89 08                       	movl	%ecx, (%rax)
14234cbc7: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
14234cbce: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14234cbd2: 4c 8b 40 40                 	movq	0x40(%rax), %r8
14234cbd6: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
14234cbda: 75 21                       	jne	0x14234cbfd <.text+0x234bbfd>
14234cbdc: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14234cbe3: 48 8d 15 86 86 86 00        	leaq	0x868686(%rip), %rdx    # 0x142bb5270
14234cbea: 4c 89 85 a0 01 00 00        	movq	%r8, 0x1a0(%rbp)
14234cbf1: e8 ba 61 21 fe              	callq	0x140562db0 <.text+0x561db0>
14234cbf6: 4c 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %r8
14234cbfd: 48 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %rdx
14234cc04: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14234cc08: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
14234cc0c: f3 0f 6f 85 b0 00 00 00     	movdqu	0xb0(%rbp), %xmm0
14234cc14: f3 0f 7f 04 c8              	movdqu	%xmm0, (%rax,%rcx,8)
14234cc19: 4c 89 44 c8 10              	movq	%r8, 0x10(%rax,%rcx,8)
14234cc1e: 48 ff c6                    	incq	%rsi
14234cc21: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
14234cc25: 48 ff 42 40                 	incq	0x40(%rdx)
14234cc29: 8b 85 3c 01 00 00           	movl	0x13c(%rbp), %eax
14234cc2f: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
14234cc33: 89 01                       	movl	%eax, (%rcx)
14234cc35: 48 8b 45 38                 	movq	0x38(%rbp), %rax
14234cc39: c6 80 03 01 00 00 00        	movb	$0x0, 0x103(%rax)
14234cc40: 8b 85 40 01 00 00           	movl	0x140(%rbp), %eax
14234cc46: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
14234cc4a: 89 01                       	movl	%eax, (%rcx)
14234cc4c: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14234cc50: c6 40 01 00                 	movb	$0x0, 0x1(%rax)
14234cc54: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234cc5b: e9 b4 01 00 00              	jmp	0x14234ce14 <.text+0x234be14>
14234cc60: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14234cc67: 8b 8d 9c 01 00 00           	movl	0x19c(%rbp), %ecx
14234cc6d: 89 08                       	movl	%ecx, (%rax)
14234cc6f: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
14234cc76: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14234cc7a: 4c 8b 40 40                 	movq	0x40(%rax), %r8
14234cc7e: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
14234cc82: 75 2e                       	jne	0x14234ccb2 <.text+0x234bcb2>
14234cc84: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14234cc8b: 48 8d 15 de 85 86 00        	leaq	0x8685de(%rip), %rdx    # 0x142bb5270
14234cc92: 4c 89 85 a0 01 00 00        	movq	%r8, 0x1a0(%rbp)
14234cc99: e8 92 59 21 fe              	callq	0x140562630 <.text+0x561630>
14234cc9e: 4c 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %r8
14234cca5: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234ccab: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234ccb2: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
14234ccb9: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14234ccbd: 48 89 f1                    	movq	%rsi, %rcx
14234ccc0: 48 c1 e1 04                 	shlq	$0x4, %rcx
14234ccc4: 4c 89 04 08                 	movq	%r8, (%rax,%rcx)
14234ccc8: c6 44 08 08 04              	movb	$0x4, 0x8(%rax,%rcx)
14234cccd: 48 ff c6                    	incq	%rsi
14234ccd0: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
14234ccd4: 48 ff 42 40                 	incq	0x40(%rdx)
14234ccd8: e9 73 01 00 00              	jmp	0x14234ce50 <.text+0x234be50>
14234ccdd: 8b 85 88 01 00 00           	movl	0x188(%rbp), %eax
14234cce3: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
14234ccea: 89 01                       	movl	%eax, (%rcx)
14234ccec: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234ccf3: 48 8b 95 90 01 00 00        	movq	0x190(%rbp), %rdx
14234ccfa: e8 61 1d 56 fe              	callq	0x1408aea60 <.text+0x8ada60>
14234ccff: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234cd05: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
14234cd0c: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
14234cd13: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
14234cd1a: eb 0b                       	jmp	0x14234cd27 <.text+0x234bd27>
14234cd1c: 0f 1f 40 00                 	nopl	(%rax)
14234cd20: 48 83 c0 40                 	addq	$0x40, %rax
14234cd24: 48 ff c2                    	incq	%rdx
14234cd27: 48 39 c8                    	cmpq	%rcx, %rax
14234cd2a: 0f 84 e9 fd ff ff           	je	0x14234cb19 <.text+0x234bb19>
14234cd30: 48 85 d2                    	testq	%rdx, %rdx
14234cd33: 41 0f 95 c0                 	setne	%r8b
14234cd37: 48 85 c0                    	testq	%rax, %rax
14234cd3a: 41 0f 95 c1                 	setne	%r9b
14234cd3e: 45 84 c8                    	testb	%r9b, %r8b
14234cd41: 74 dd                       	je	0x14234cd20 <.text+0x234bd20>
14234cd43: 80 70 3d 01                 	xorb	$0x1, 0x3d(%rax)
14234cd47: eb d7                       	jmp	0x14234cd20 <.text+0x234bd20>
14234cd49: 4c 89 d9                    	movq	%r11, %rcx
14234cd4c: 4c 89 de                    	movq	%r11, %rsi
14234cd4f: e8 0c b5 55 fe              	callq	0x1408a8260 <.text+0x8a7260>
14234cd54: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
14234cd58: 8b 95 4c 01 00 00           	movl	0x14c(%rbp), %edx
14234cd5e: 89 11                       	movl	%edx, (%rcx)
14234cd60: 48 89 f1                    	movq	%rsi, %rcx
14234cd63: 84 c0                       	testb	%al, %al
14234cd65: 0f 84 9c 00 00 00           	je	0x14234ce07 <.text+0x234be07>
14234cd6b: e8 d0 b3 55 fe              	callq	0x1408a8140 <.text+0x8a7140>
14234cd70: e9 9c 00 00 00              	jmp	0x14234ce11 <.text+0x234be11>
14234cd75: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
14234cd7c: e8 9f c9 80 fe              	callq	0x140b59720 <.text+0xb58720>
14234cd81: 41 89 c0                    	movl	%eax, %r8d
14234cd84: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14234cd8b: 8b 8d 9c 01 00 00           	movl	0x19c(%rbp), %ecx
14234cd91: 89 08                       	movl	%ecx, (%rax)
14234cd93: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
14234cd9a: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14234cd9e: 4c 8b 48 40                 	movq	0x40(%rax), %r9
14234cda2: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
14234cda6: 75 30                       	jne	0x14234cdd8 <.text+0x234bdd8>
14234cda8: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14234cdaf: 48 8d 15 ba 84 86 00        	leaq	0x8684ba(%rip), %rdx    # 0x142bb5270
14234cdb6: 44 88 85 a0 01 00 00        	movb	%r8b, 0x1a0(%rbp)
14234cdbd: 4c 89 8d 78 01 00 00        	movq	%r9, 0x178(%rbp)
14234cdc4: e8 67 58 21 fe              	callq	0x140562630 <.text+0x561630>
14234cdc9: 4c 8b 8d 78 01 00 00        	movq	0x178(%rbp), %r9
14234cdd0: 44 0f b6 85 a0 01 00 00     	movzbl	0x1a0(%rbp), %r8d
14234cdd8: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
14234cddf: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14234cde3: 48 89 f1                    	movq	%rsi, %rcx
14234cde6: 48 c1 e1 04                 	shlq	$0x4, %rcx
14234cdea: 4c 89 0c 08                 	movq	%r9, (%rax,%rcx)
14234cdee: 44 88 44 08 08              	movb	%r8b, 0x8(%rax,%rcx)
14234cdf3: 48 ff c6                    	incq	%rsi
14234cdf6: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
14234cdfa: 48 ff 42 40                 	incq	0x40(%rdx)
14234cdfe: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234ce05: eb 0d                       	jmp	0x14234ce14 <.text+0x234be14>
14234ce07: b2 01                       	movb	$0x1, %dl
14234ce09: 45 31 c0                    	xorl	%r8d, %r8d
14234ce0c: e8 4f b2 55 fe              	callq	0x1408a8060 <.text+0x8a7060>
14234ce11: 49 89 f3                    	movq	%rsi, %r11
14234ce14: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14234ce1b: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234ce21: eb 34                       	jmp	0x14234ce57 <.text+0x234be57>
14234ce23: f2 44 0f 10 40 10           	movsd	0x10(%rax), %xmm8
14234ce29: b9 03 00 00 00              	movl	$0x3, %ecx
14234ce2e: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234ce35: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234ce3b: 89 08                       	movl	%ecx, (%rax)
14234ce3d: 44 0f 13 40 04              	movlps	%xmm8, 0x4(%rax)
14234ce42: 0f 13 78 10                 	movlps	%xmm7, 0x10(%rax)
14234ce46: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14234ce50: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14234ce57: 49 8b 76 08                 	movq	0x8(%r14), %rsi
14234ce5b: 48 8b 01                    	movq	(%rcx), %rax
14234ce5e: 4c 8b 71 10                 	movq	0x10(%rcx), %r14
14234ce62: 4c 29 f0                    	subq	%r14, %rax
14234ce65: 48 83 f8 27                 	cmpq	$0x27, %rax
14234ce69: 0f 86 f5 01 00 00           	jbe	0x14234d064 <.text+0x234c064>
14234ce6f: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14234ce73: 48 8d 15 86 23 c9 fe        	leaq	-0x136dc7a(%rip), %rdx  # 0x140fdf200 <.text+0xfde200>
14234ce7a: 4a 89 14 30                 	movq	%rdx, (%rax,%r14)
14234ce7e: 48 8d 15 db 29 b0 ff        	leaq	-0x4fd625(%rip), %rdx   # 0x141e4f860 <.text+0x1e4e860>
14234ce85: 4a 89 54 30 08              	movq	%rdx, 0x8(%rax,%r14)
14234ce8a: 4a 89 74 30 10              	movq	%rsi, 0x10(%rax,%r14)
14234ce8f: f2 42 0f 11 74 30 18        	movsd	%xmm6, 0x18(%rax,%r14)
14234ce96: 42 c7 44 30 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r14)
14234ce9f: 42 c6 44 30 24 00           	movb	$0x0, 0x24(%rax,%r14)
14234cea5: 49 83 c6 28                 	addq	$0x28, %r14
14234cea9: 4c 89 71 10                 	movq	%r14, 0x10(%rcx)
14234cead: e9 67 f9 ff ff              	jmp	0x14234c819 <.text+0x234b819>
14234ceb2: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
14234ceb6: 48 8d 15 83 24 d0 00        	leaq	0xd02483(%rip), %rdx    # 0x14304f340
14234cebd: e8 6e 2c c8 fe              	callq	0x140fcfb30 <.text+0xfceb30>
14234cec2: e8 69 0c 85 fe              	callq	0x140b9db30 <.text+0xb9cb30>
14234cec7: 49 89 d0                    	movq	%rdx, %r8
14234ceca: 48 c7 44 24 20 0b 00 00 00  	movq	$0xb, 0x20(%rsp)
14234ced3: 48 8d b5 18 01 00 00        	leaq	0x118(%rbp), %rsi
14234ceda: 48 89 f1                    	movq	%rsi, %rcx
14234cedd: 48 89 c2                    	movq	%rax, %rdx
14234cee0: 4c 8d 0d 71 24 d0 00        	leaq	0xd02471(%rip), %r9     # 0x14304f358
14234cee7: e8 24 59 a1 fe              	callq	0x140d62810 <.text+0xd61810>
14234ceec: c6 85 e7 01 00 00 01        	movb	$0x1, 0x1e7(%rbp)
14234cef3: 48 89 f1                    	movq	%rsi, %rcx
14234cef6: e8 d5 9f a2 fe              	callq	0x140d76ed0 <.text+0xd75ed0>
14234cefb: c6 85 e7 01 00 00 01        	movb	$0x1, 0x1e7(%rbp)
14234cf02: 48 89 c1                    	movq	%rax, %rcx
14234cf05: e8 06 5f a1 fe              	callq	0x140d62e10 <.text+0xd61e10>
14234cf0a: 84 c0                       	testb	%al, %al
14234cf0c: 74 55                       	je	0x14234cf63 <.text+0x234bf63>
14234cf0e: f3 0f 6f 85 18 01 00 00     	movdqu	0x118(%rbp), %xmm0
14234cf16: f3 0f 6f 8d 28 01 00 00     	movdqu	0x128(%rbp), %xmm1
14234cf1e: 66 0f 7f 8d c0 00 00 00     	movdqa	%xmm1, 0xc0(%rbp)
14234cf26: 66 0f 7f 85 b0 00 00 00     	movdqa	%xmm0, 0xb0(%rbp)
14234cf2e: c6 85 e7 01 00 00 00        	movb	$0x0, 0x1e7(%rbp)
14234cf35: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234cf3c: e8 ff 9a 2d 00              	callq	0x142626a40 <.text+0x2625a40>
14234cf41: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14234cf45: 48 85 c0                    	testq	%rax, %rax
14234cf48: 0f 84 0a 01 00 00           	je	0x14234d058 <.text+0x234c058>
14234cf4e: c6 85 e7 01 00 00 00        	movb	$0x0, 0x1e7(%rbp)
14234cf55: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
14234cf59: e8 a2 aa 29 fe              	callq	0x1405e7a00 <.text+0x5e6a00>
14234cf5e: e9 f5 00 00 00              	jmp	0x14234d058 <.text+0x234c058>
14234cf63: c6 85 e7 01 00 00 01        	movb	$0x1, 0x1e7(%rbp)
14234cf6a: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234cf71: 48 8d 95 18 01 00 00        	leaq	0x118(%rbp), %rdx
14234cf78: 4c 8d 05 21 23 d0 00        	leaq	0xd02321(%rip), %r8     # 0x14304f2a0
14234cf7f: e8 bc 0f c8 fe              	callq	0x140fcdf40 <.text+0xfccf40>
14234cf84: 0f b6 85 30 01 00 00        	movzbl	0x130(%rbp), %eax
14234cf8b: 88 85 c8 00 00 00           	movb	%al, 0xc8(%rbp)
14234cf91: c6 85 e7 01 00 00 01        	movb	$0x1, 0x1e7(%rbp)
14234cf98: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234cf9f: e8 1c a2 2d 00              	callq	0x1426271c0 <.text+0x26261c0>
14234cfa4: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14234cfa8: 48 85 c0                    	testq	%rax, %rax
14234cfab: 74 5f                       	je	0x14234d00c <.text+0x234c00c>
14234cfad: c6 85 e7 01 00 00 01        	movb	$0x1, 0x1e7(%rbp)
14234cfb4: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14234cfb8: e8 43 aa 29 fe              	callq	0x1405e7a00 <.text+0x5e6a00>
14234cfbd: 48 8b 95 18 01 00 00        	movq	0x118(%rbp), %rdx
14234cfc4: 48 85 d2                    	testq	%rdx, %rdx
14234cfc7: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234cfce: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14234cfd5: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234cfdb: 0f 84 76 fe ff ff           	je	0x14234ce57 <.text+0x234be57>
14234cfe1: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
14234cfe8: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14234cfee: e8 2d 6c 2c fe              	callq	0x140613c20 <.text+0x612c20>
14234cff3: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234cff9: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14234d000: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d007: e9 4b fe ff ff              	jmp	0x14234ce57 <.text+0x234be57>
14234d00c: f3 0f 6f 85 18 01 00 00     	movdqu	0x118(%rbp), %xmm0
14234d014: f3 0f 6f 8d 28 01 00 00     	movdqu	0x128(%rbp), %xmm1
14234d01c: 66 0f 7f 8d c0 00 00 00     	movdqa	%xmm1, 0xc0(%rbp)
14234d024: 66 0f 7f 85 b0 00 00 00     	movdqa	%xmm0, 0xb0(%rbp)
14234d02c: c6 85 e7 01 00 00 00        	movb	$0x0, 0x1e7(%rbp)
14234d033: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14234d03a: e8 01 9a 2d 00              	callq	0x142626a40 <.text+0x2625a40>
14234d03f: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14234d043: 48 85 c0                    	testq	%rax, %rax
14234d046: 74 10                       	je	0x14234d058 <.text+0x234c058>
14234d048: c6 85 e7 01 00 00 00        	movb	$0x0, 0x1e7(%rbp)
14234d04f: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
14234d053: e8 a8 a9 29 fe              	callq	0x1405e7a00 <.text+0x5e6a00>
14234d058: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d05f: e9 b0 fd ff ff              	jmp	0x14234ce14 <.text+0x234be14>
14234d064: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14234d06d: 41 b8 28 00 00 00           	movl	$0x28, %r8d
14234d073: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14234d079: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14234d080: 4c 89 f2                    	movq	%r14, %rdx
14234d083: e8 48 88 55 00              	callq	0x1428a58d0 <.text+0x28a48d0>
14234d088: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14234d08e: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14234d095: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d09c: e9 ce fd ff ff              	jmp	0x14234ce6f <.text+0x234be6f>
14234d0a1: 49 8b b5 f8 00 00 00        	movq	0xf8(%r13), %rsi
14234d0a8: 49 8b 85 00 01 00 00        	movq	0x100(%r13), %rax
14234d0af: 48 8d 3c 86                 	leaq	(%rsi,%rax,4), %rdi
14234d0b3: 41 80 bd 08 01 00 00 00     	cmpb	$0x0, 0x108(%r13)
14234d0bb: 0f 84 4c 02 00 00           	je	0x14234d30d <.text+0x234c30d>
14234d0c1: 45 31 c0                    	xorl	%r8d, %r8d
14234d0c4: 41 b9 02 00 00 00           	movl	$0x2, %r9d
14234d0ca: 45 31 d2                    	xorl	%r10d, %r10d
14234d0cd: 45 31 e4                    	xorl	%r12d, %r12d
14234d0d0: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
14234d0db: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
14234d0e2: 4c 39 e0                    	cmpq	%r12, %rax
14234d0e5: 0f 85 a5 00 00 00           	jne	0x14234d190 <.text+0x234c190>
14234d0eb: 48 39 fe                    	cmpq	%rdi, %rsi
14234d0ee: 0f 84 b8 04 00 00           	je	0x14234d5ac <.text+0x234c5ac>
14234d0f4: 48 8b 83 a8 01 00 00        	movq	0x1a8(%rbx), %rax
14234d0fb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14234d100: 8b 0e                       	movl	(%rsi), %ecx
14234d102: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14234d106: 4c 8b 64 c8 10              	movq	0x10(%rax,%rcx,8), %r12
14234d10b: 4d 85 e4                    	testq	%r12, %r12
14234d10e: 75 0e                       	jne	0x14234d11e <.text+0x234c11e>
14234d110: 48 83 c6 04                 	addq	$0x4, %rsi
14234d114: 48 39 fe                    	cmpq	%rdi, %rsi
14234d117: 75 e7                       	jne	0x14234d100 <.text+0x234c100>
14234d119: e9 8e 04 00 00              	jmp	0x14234d5ac <.text+0x234c5ac>
14234d11e: 48 8d 14 c8                 	leaq	(%rax,%rcx,8), %rdx
14234d122: 4d 8b 85 10 01 00 00        	movq	0x110(%r13), %r8
14234d129: 48 8b 4a 38                 	movq	0x38(%rdx), %rcx
14234d12d: 48 8b 42 40                 	movq	0x40(%rdx), %rax
14234d131: 4c 39 c0                    	cmpq	%r8, %rax
14234d134: 76 1f                       	jbe	0x14234d155 <.text+0x234c155>
14234d136: 4e 8b 04 c1                 	movq	(%rcx,%r8,8), %r8
14234d13a: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14234d13e: 4d 85 c0                    	testq	%r8, %r8
14234d141: 74 16                       	je	0x14234d159 <.text+0x234c159>
14234d143: 49 f7 d0                    	notq	%r8
14234d146: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14234d14a: 49 c1 e0 04                 	shlq	$0x4, %r8
14234d14e: 4e 8b 54 02 10              	movq	0x10(%rdx,%r8), %r10
14234d153: eb 07                       	jmp	0x14234d15c <.text+0x234c15c>
14234d155: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14234d159: 45 31 d2                    	xorl	%r10d, %r10d
14234d15c: 48 83 c6 04                 	addq	$0x4, %rsi
14234d160: 4d 8b 85 18 01 00 00        	movq	0x118(%r13), %r8
14234d167: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
14234d16b: 48 f7 d1                    	notq	%rcx
14234d16e: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14234d172: 48 c1 e1 04                 	shlq	$0x4, %rcx
14234d176: 4c 8b 44 0a 10              	movq	0x10(%rdx,%rcx), %r8
14234d17b: 48 8b 4c 0a 28              	movq	0x28(%rdx,%rcx), %rcx
14234d180: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
14234d187: 49 3b 85 20 01 00 00        	cmpq	0x120(%r13), %rax
14234d18e: 31 c0                       	xorl	%eax, %eax
14234d190: 41 89 c7                    	movl	%eax, %r15d
14234d193: 48 ff c0                    	incq	%rax
14234d196: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14234d19d: 43 0f b6 04 3a              	movzbl	(%r10,%r15), %eax
14234d1a2: 8d 48 fc                    	leal	-0x4(%rax), %ecx
14234d1a5: 80 f9 0e                    	cmpb	$0xe, %cl
14234d1a8: 0f b6 c9                    	movzbl	%cl, %ecx
14234d1ab: 41 0f 43 c9                 	cmovael	%r9d, %ecx
14234d1af: 0f b6 c9                    	movzbl	%cl, %ecx
14234d1b2: 83 c1 fe                    	addl	$-0x2, %ecx
14234d1b5: 83 f9 0a                    	cmpl	$0xa, %ecx
14234d1b8: 0f 87 1d ff ff ff           	ja	0x14234d0db <.text+0x234c0db>
14234d1be: 4f 8d 34 7f                 	leaq	(%r15,%r15,2), %r14
14234d1c2: 4d 01 c6                    	addq	%r8, %r14
14234d1c5: 48 8d 15 40 22 d0 00        	leaq	0xd02240(%rip), %rdx    # 0x14304f40c
14234d1cc: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
14234d1d0: 48 01 d1                    	addq	%rdx, %rcx
14234d1d3: ff e1                       	jmpq	*%rcx
14234d1d5: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
14234d1dc: 3a 01                       	cmpb	(%rcx), %al
14234d1de: 8b 85 c4 01 00 00           	movl	0x1c4(%rbp), %eax
14234d1e4: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14234d1eb: 42 89 04 b9                 	movl	%eax, (%rcx,%r15,4)
14234d1ef: 41 0f 94 46 02              	sete	0x2(%r14)
14234d1f4: e9 e2 fe ff ff              	jmp	0x14234d0db <.text+0x234c0db>
14234d1f9: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
14234d200: 4c 89 95 b8 01 00 00        	movq	%r10, 0x1b8(%rbp)
14234d207: 4c 89 85 c8 01 00 00        	movq	%r8, 0x1c8(%rbp)
14234d20e: e8 2d 40 5c fe              	callq	0x140911240 <.text+0x910240>
14234d213: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14234d21a: 4c 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %r10
14234d221: 41 b9 02 00 00 00           	movl	$0x2, %r9d
14234d227: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d22e: 83 38 02                    	cmpl	$0x2, (%rax)
14234d231: 8b 85 c4 01 00 00           	movl	0x1c4(%rbp), %eax
14234d237: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14234d23e: 42 89 04 b9                 	movl	%eax, (%rcx,%r15,4)
14234d242: 41 0f 92 46 02              	setb	0x2(%r14)
14234d247: e9 8f fe ff ff              	jmp	0x14234d0db <.text+0x234c0db>
14234d24c: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
14234d253: 4c 89 95 b8 01 00 00        	movq	%r10, 0x1b8(%rbp)
14234d25a: 4c 89 85 c8 01 00 00        	movq	%r8, 0x1c8(%rbp)
14234d261: e8 ea 17 56 fe              	callq	0x1408aea50 <.text+0x8ada50>
14234d266: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14234d26d: 4c 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %r10
14234d274: 41 b9 02 00 00 00           	movl	$0x2, %r9d
14234d27a: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d281: 0f b6 40 3c                 	movzbl	0x3c(%rax), %eax
14234d285: eb 6c                       	jmp	0x14234d2f3 <.text+0x234c2f3>
14234d287: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
14234d28e: 4c 89 95 b8 01 00 00        	movq	%r10, 0x1b8(%rbp)
14234d295: 4c 89 85 c8 01 00 00        	movq	%r8, 0x1c8(%rbp)
14234d29c: e8 af 17 56 fe              	callq	0x1408aea50 <.text+0x8ada50>
14234d2a1: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14234d2a8: 4c 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %r10
14234d2af: 41 b9 02 00 00 00           	movl	$0x2, %r9d
14234d2b5: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d2bc: 0f b6 40 3d                 	movzbl	0x3d(%rax), %eax
14234d2c0: eb 31                       	jmp	0x14234d2f3 <.text+0x234c2f3>
14234d2c2: 4c 89 d9                    	movq	%r11, %rcx
14234d2c5: 4c 89 95 b8 01 00 00        	movq	%r10, 0x1b8(%rbp)
14234d2cc: 4c 89 85 c8 01 00 00        	movq	%r8, 0x1c8(%rbp)
14234d2d3: e8 88 af 55 fe              	callq	0x1408a8260 <.text+0x8a7260>
14234d2d8: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14234d2df: 4c 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %r10
14234d2e6: 41 b9 02 00 00 00           	movl	$0x2, %r9d
14234d2ec: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d2f3: 8b 8d c4 01 00 00           	movl	0x1c4(%rbp), %ecx
14234d2f9: 48 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %rdx
14234d300: 42 89 0c ba                 	movl	%ecx, (%rdx,%r15,4)
14234d304: 41 88 46 02                 	movb	%al, 0x2(%r14)
14234d308: e9 ce fd ff ff              	jmp	0x14234d0db <.text+0x234c0db>
14234d30d: 41 ba 08 00 00 00           	movl	$0x8, %r10d
14234d313: 45 31 c0                    	xorl	%r8d, %r8d
14234d316: 45 31 c9                    	xorl	%r9d, %r9d
14234d319: 45 31 e4                    	xorl	%r12d, %r12d
14234d31c: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
14234d327: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
14234d32e: 4c 39 e1                    	cmpq	%r12, %rcx
14234d331: 0f 85 cb 00 00 00           	jne	0x14234d402 <.text+0x234c402>
14234d337: 48 39 fe                    	cmpq	%rdi, %rsi
14234d33a: 0f 84 6c 02 00 00           	je	0x14234d5ac <.text+0x234c5ac>
14234d340: 48 8b 83 00 01 00 00        	movq	0x100(%rbx), %rax
14234d347: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14234d350: 8b 0e                       	movl	(%rsi), %ecx
14234d352: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14234d356: 48 c1 e1 05                 	shlq	$0x5, %rcx
14234d35a: 4c 8b 64 08 58              	movq	0x58(%rax,%rcx), %r12
14234d35f: 4d 85 e4                    	testq	%r12, %r12
14234d362: 75 0e                       	jne	0x14234d372 <.text+0x234c372>
14234d364: 48 83 c6 04                 	addq	$0x4, %rsi
14234d368: 48 39 fe                    	cmpq	%rdi, %rsi
14234d36b: 75 e3                       	jne	0x14234d350 <.text+0x234c350>
14234d36d: e9 3a 02 00 00              	jmp	0x14234d5ac <.text+0x234c5ac>
14234d372: 48 01 c8                    	addq	%rcx, %rax
14234d375: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
14234d37b: 4c 8b 8b a8 01 00 00        	movq	0x1a8(%rbx), %r9
14234d382: 4c 8d 14 c9                 	leaq	(%rcx,%rcx,8), %r10
14234d386: 4b 8d 14 d1                 	leaq	(%r9,%r10,8), %rdx
14234d38a: 4d 8b 85 10 01 00 00        	movq	0x110(%r13), %r8
14234d391: 4b 8b 4c d1 38              	movq	0x38(%r9,%r10,8), %rcx
14234d396: 4f 39 44 d1 40              	cmpq	%r8, 0x40(%r9,%r10,8)
14234d39b: 76 1f                       	jbe	0x14234d3bc <.text+0x234c3bc>
14234d39d: 4e 8b 04 c1                 	movq	(%rcx,%r8,8), %r8
14234d3a1: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14234d3a5: 4d 85 c0                    	testq	%r8, %r8
14234d3a8: 74 16                       	je	0x14234d3c0 <.text+0x234c3c0>
14234d3aa: 49 f7 d0                    	notq	%r8
14234d3ad: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14234d3b1: 49 c1 e0 04                 	shlq	$0x4, %r8
14234d3b5: 4e 8b 4c 02 10              	movq	0x10(%rdx,%r8), %r9
14234d3ba: eb 07                       	jmp	0x14234d3c3 <.text+0x234c3c3>
14234d3bc: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14234d3c0: 45 31 c9                    	xorl	%r9d, %r9d
14234d3c3: 4c 8b 80 88 00 00 00        	movq	0x88(%rax), %r8
14234d3ca: 4d 3b 85 20 01 00 00        	cmpq	0x120(%r13), %r8
14234d3d1: 48 83 c6 04                 	addq	$0x4, %rsi
14234d3d5: 4d 8b 85 18 01 00 00        	movq	0x118(%r13), %r8
14234d3dc: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
14234d3e0: 48 f7 d1                    	notq	%rcx
14234d3e3: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14234d3e7: 48 c1 e1 04                 	shlq	$0x4, %rcx
14234d3eb: 4c 8b 44 0a 10              	movq	0x10(%rdx,%rcx), %r8
14234d3f0: 48 8b 4c 0a 28              	movq	0x28(%rdx,%rcx), %rcx
14234d3f5: 48 89 8d a8 01 00 00        	movq	%rcx, 0x1a8(%rbp)
14234d3fc: 4c 8b 50 50                 	movq	0x50(%rax), %r10
14234d400: 31 c9                       	xorl	%ecx, %ecx
14234d402: 48 89 c8                    	movq	%rcx, %rax
14234d405: 48 c1 e0 04                 	shlq	$0x4, %rax
14234d409: 45 8b 74 02 08              	movl	0x8(%r10,%rax), %r14d
14234d40e: 48 ff c1                    	incq	%rcx
14234d411: 48 89 8d b0 01 00 00        	movq	%rcx, 0x1b0(%rbp)
14234d418: 43 0f b6 04 31              	movzbl	(%r9,%r14), %eax
14234d41d: 8d 48 fc                    	leal	-0x4(%rax), %ecx
14234d420: 80 f9 0e                    	cmpb	$0xe, %cl
14234d423: 0f b6 c9                    	movzbl	%cl, %ecx
14234d426: ba 02 00 00 00              	movl	$0x2, %edx
14234d42b: 0f 43 ca                    	cmovael	%edx, %ecx
14234d42e: 0f b6 c9                    	movzbl	%cl, %ecx
14234d431: 83 c1 fe                    	addl	$-0x2, %ecx
14234d434: 83 f9 0a                    	cmpl	$0xa, %ecx
14234d437: 0f 87 ea fe ff ff           	ja	0x14234d327 <.text+0x234c327>
14234d43d: 4f 8d 3c 76                 	leaq	(%r14,%r14,2), %r15
14234d441: 4d 01 c7                    	addq	%r8, %r15
14234d444: 48 8d 15 95 1f d0 00        	leaq	0xd01f95(%rip), %rdx    # 0x14304f3e0
14234d44b: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
14234d44f: 48 01 d1                    	addq	%rdx, %rcx
14234d452: ff e1                       	jmpq	*%rcx
14234d454: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
14234d45b: 3a 01                       	cmpb	(%rcx), %al
14234d45d: 8b 85 c4 01 00 00           	movl	0x1c4(%rbp), %eax
14234d463: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
14234d46a: 42 89 04 b1                 	movl	%eax, (%rcx,%r14,4)
14234d46e: 41 0f 94 47 02              	sete	0x2(%r15)
14234d473: e9 af fe ff ff              	jmp	0x14234d327 <.text+0x234c327>
14234d478: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
14234d47f: 4c 89 8d b8 01 00 00        	movq	%r9, 0x1b8(%rbp)
14234d486: 4c 89 85 c8 01 00 00        	movq	%r8, 0x1c8(%rbp)
14234d48d: 4c 89 95 d8 01 00 00        	movq	%r10, 0x1d8(%rbp)
14234d494: e8 a7 3d 5c fe              	callq	0x140911240 <.text+0x910240>
14234d499: 4c 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %r10
14234d4a0: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14234d4a7: 4c 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %r9
14234d4ae: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d4b5: 83 38 02                    	cmpl	$0x2, (%rax)
14234d4b8: 8b 85 c4 01 00 00           	movl	0x1c4(%rbp), %eax
14234d4be: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
14234d4c5: 42 89 04 b1                 	movl	%eax, (%rcx,%r14,4)
14234d4c9: 41 0f 92 47 02              	setb	0x2(%r15)
14234d4ce: e9 54 fe ff ff              	jmp	0x14234d327 <.text+0x234c327>
14234d4d3: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
14234d4da: 4c 89 8d b8 01 00 00        	movq	%r9, 0x1b8(%rbp)
14234d4e1: 4c 89 85 c8 01 00 00        	movq	%r8, 0x1c8(%rbp)
14234d4e8: 4c 89 95 d8 01 00 00        	movq	%r10, 0x1d8(%rbp)
14234d4ef: e8 5c 15 56 fe              	callq	0x1408aea50 <.text+0x8ada50>
14234d4f4: 4c 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %r10
14234d4fb: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14234d502: 4c 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %r9
14234d509: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d510: 0f b6 40 3c                 	movzbl	0x3c(%rax), %eax
14234d514: eb 7c                       	jmp	0x14234d592 <.text+0x234c592>
14234d516: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
14234d51d: 4c 89 8d b8 01 00 00        	movq	%r9, 0x1b8(%rbp)
14234d524: 4c 89 85 c8 01 00 00        	movq	%r8, 0x1c8(%rbp)
14234d52b: 4c 89 95 d8 01 00 00        	movq	%r10, 0x1d8(%rbp)
14234d532: e8 19 15 56 fe              	callq	0x1408aea50 <.text+0x8ada50>
14234d537: 4c 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %r10
14234d53e: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14234d545: 4c 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %r9
14234d54c: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d553: 0f b6 40 3d                 	movzbl	0x3d(%rax), %eax
14234d557: eb 39                       	jmp	0x14234d592 <.text+0x234c592>
14234d559: 4c 89 d9                    	movq	%r11, %rcx
14234d55c: 4c 89 8d b8 01 00 00        	movq	%r9, 0x1b8(%rbp)
14234d563: 4c 89 85 c8 01 00 00        	movq	%r8, 0x1c8(%rbp)
14234d56a: 4c 89 95 d8 01 00 00        	movq	%r10, 0x1d8(%rbp)
14234d571: e8 ea ac 55 fe              	callq	0x1408a8260 <.text+0x8a7260>
14234d576: 4c 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %r10
14234d57d: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14234d584: 4c 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %r9
14234d58b: 4c 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %r11
14234d592: 8b 8d c4 01 00 00           	movl	0x1c4(%rbp), %ecx
14234d598: 48 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %rdx
14234d59f: 42 89 0c b2                 	movl	%ecx, (%rdx,%r14,4)
14234d5a3: 41 88 47 02                 	movb	%al, 0x2(%r15)
14234d5a7: e9 7b fd ff ff              	jmp	0x14234d327 <.text+0x234c327>
14234d5ac: 0f 28 b5 f0 01 00 00        	movaps	0x1f0(%rbp), %xmm6
14234d5b3: 0f 28 bd 00 02 00 00        	movaps	0x200(%rbp), %xmm7
14234d5ba: 44 0f 28 85 10 02 00 00     	movaps	0x210(%rbp), %xmm8
14234d5c2: 48 81 c4 a8 02 00 00        	addq	$0x2a8, %rsp            # imm = 0x2A8
14234d5c9: 5b                          	popq	%rbx
14234d5ca: 5f                          	popq	%rdi
14234d5cb: 5e                          	popq	%rsi
14234d5cc: 41 5c                       	popq	%r12
14234d5ce: 41 5d                       	popq	%r13
14234d5d0: 41 5e                       	popq	%r14
14234d5d2: 41 5f                       	popq	%r15
14234d5d4: 5d                          	popq	%rbp
14234d5d5: c3                          	retq
14234d5d6: 48 8d 0d 2b 9c 6c 00        	leaq	0x6c9c2b(%rip), %rcx    # 0x142a17208
14234d5dd: 4c 8d 05 f4 a9 6c 00        	leaq	0x6ca9f4(%rip), %r8     # 0x142a17fd8
14234d5e4: ba 16 00 00 00              	movl	$0x16, %edx
14234d5e9: e8 42 bd 58 00              	callq	0x1428d9330 <.text+0x28d8330>
14234d5ee: 48 8d 0d 6e 1d d0 00        	leaq	0xd01d6e(%rip), %rcx    # 0x14304f363
14234d5f5: 4c 8d 05 94 1d d0 00        	leaq	0xd01d94(%rip), %r8     # 0x14304f390
14234d5fc: ba 28 00 00 00              	movl	$0x28, %edx
14234d601: e8 2a be 58 00              	callq	0x1428d9430 <.text+0x28d8430>
14234d606: cc                          	int3
14234d607: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14234d610: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14234d615: 55                          	pushq	%rbp
14234d616: 41 57                       	pushq	%r15
14234d618: 41 56                       	pushq	%r14
14234d61a: 41 55                       	pushq	%r13
14234d61c: 41 54                       	pushq	%r12
14234d61e: 56                          	pushq	%rsi
14234d61f: 57                          	pushq	%rdi
14234d620: 53                          	pushq	%rbx
14234d621: 48 83 ec 58                 	subq	$0x58, %rsp
14234d625: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234d62c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
14234d632: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
14234d637: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
14234d63c: 80 bd e7 01 00 00 00        	cmpb	$0x0, 0x1e7(%rbp)
14234d643: 74 1e                       	je	0x14234d663 <.text+0x234c663>
14234d645: 48 8b 95 18 01 00 00        	movq	0x118(%rbp), %rdx
14234d64c: 48 85 d2                    	testq	%rdx, %rdx
14234d64f: 74 12                       	je	0x14234d663 <.text+0x234c663>
14234d651: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
14234d658: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14234d65e: e8 bd 65 2c fe              	callq	0x140613c20 <.text+0x612c20>
14234d663: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
14234d668: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
14234d66d: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
14234d673: 48 83 c4 58                 	addq	$0x58, %rsp
14234d677: 5b                          	popq	%rbx
14234d678: 5f                          	popq	%rdi
14234d679: 5e                          	popq	%rsi
14234d67a: 41 5c                       	popq	%r12
14234d67c: 41 5d                       	popq	%r13
14234d67e: 41 5e                       	popq	%r14
14234d680: 41 5f                       	popq	%r15
14234d682: 5d                          	popq	%rbp
14234d683: c3                          	retq
