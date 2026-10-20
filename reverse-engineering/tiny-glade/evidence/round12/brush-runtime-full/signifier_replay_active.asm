
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b1b860: 41 57                       	pushq	%r15
140b1b862: 41 56                       	pushq	%r14
140b1b864: 41 55                       	pushq	%r13
140b1b866: 41 54                       	pushq	%r12
140b1b868: 56                          	pushq	%rsi
140b1b869: 57                          	pushq	%rdi
140b1b86a: 55                          	pushq	%rbp
140b1b86b: 53                          	pushq	%rbx
140b1b86c: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
140b1b873: 44 0f 29 bc 24 40 01 00 00  	movaps	%xmm15, 0x140(%rsp)
140b1b87c: 44 0f 29 b4 24 30 01 00 00  	movaps	%xmm14, 0x130(%rsp)
140b1b885: 44 0f 29 ac 24 20 01 00 00  	movaps	%xmm13, 0x120(%rsp)
140b1b88e: 44 0f 29 a4 24 10 01 00 00  	movaps	%xmm12, 0x110(%rsp)
140b1b897: 44 0f 29 9c 24 00 01 00 00  	movaps	%xmm11, 0x100(%rsp)
140b1b8a0: 44 0f 29 94 24 f0 00 00 00  	movaps	%xmm10, 0xf0(%rsp)
140b1b8a9: 44 0f 29 8c 24 e0 00 00 00  	movaps	%xmm9, 0xe0(%rsp)
140b1b8b2: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
140b1b8bb: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
140b1b8c3: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
140b1b8cb: 48 89 cf                    	movq	%rcx, %rdi
140b1b8ce: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140b1b8d1: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140b1b8d5: 89 01                       	movl	%eax, (%rcx)
140b1b8d7: 48 8b 02                    	movq	(%rdx), %rax
140b1b8da: 83 38 01                    	cmpl	$0x1, (%rax)
140b1b8dd: 0f 85 b8 06 00 00           	jne	0x140b1bf9b <.text+0xb1af9b>
140b1b8e3: 4c 89 ce                    	movq	%r9, %rsi
140b1b8e6: 4c 89 c3                    	movq	%r8, %rbx
140b1b8e9: 48 8b 8c 24 d0 01 00 00     	movq	0x1d0(%rsp), %rcx
140b1b8f1: 48 8b 94 24 c0 01 00 00     	movq	0x1c0(%rsp), %rdx
140b1b8f9: 48 8b 12                    	movq	(%rdx), %rdx
140b1b8fc: f3 0f 10 72 28              	movss	0x28(%rdx), %xmm6
140b1b901: f3 0f 58 70 04              	addss	0x4(%rax), %xmm6
140b1b906: f3 0f 11 70 04              	movss	%xmm6, 0x4(%rax)
140b1b90b: 48 8b 01                    	movq	(%rcx), %rax
140b1b90e: f3 0f 10 05 76 0f f6 01     	movss	0x1f60f76(%rip), %xmm0  # 0x142a7c88c
140b1b916: f3 0f 5e 40 28              	divss	0x28(%rax), %xmm0
140b1b91b: 4c 8b b4 24 c8 01 00 00     	movq	0x1c8(%rsp), %r14
140b1b923: f3 0f 5f 05 45 9f e0 01     	maxss	0x1e09f45(%rip), %xmm0  # 0x142925870
140b1b92b: f3 0f 5e f0                 	divss	%xmm0, %xmm6
140b1b92f: 0f 28 c6                    	movaps	%xmm6, %xmm0
140b1b932: e8 29 87 e0 01              	callq	0x142924060 <.text+0x2923060>
140b1b937: f3 0f 5c f0                 	subss	%xmm0, %xmm6
140b1b93b: 48 8b 0b                    	movq	(%rbx), %rcx
140b1b93e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140b1b942: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140b1b947: 8b 43 14                    	movl	0x14(%rbx), %eax
140b1b94a: 89 44 24 24                 	movl	%eax, 0x24(%rsp)
140b1b94e: 4c 8b a9 f8 00 00 00        	movq	0xf8(%rcx), %r13
140b1b955: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140b1b95c: 49 8d 5c 85 00              	leaq	(%r13,%rax,4), %rbx
140b1b961: 48 89 4c 24 48              	movq	%rcx, 0x48(%rsp)
140b1b966: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
140b1b96d: 88 44 24 23                 	movb	%al, 0x23(%rsp)
140b1b971: 48 8b 3f                    	movq	(%rdi), %rdi
140b1b974: 48 8b 06                    	movq	(%rsi), %rax
140b1b977: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
140b1b97f: 4d 8b 3e                    	movq	(%r14), %r15
140b1b982: 49 8d 47 04                 	leaq	0x4(%r15), %rax
140b1b986: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
140b1b98e: b8 08 00 00 00              	movl	$0x8, %eax
140b1b993: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
140b1b998: 31 ed                       	xorl	%ebp, %ebp
140b1b99a: f3 44 0f 10 0d 1d d6 fc 01  	movss	0x1fcd61d(%rip), %xmm9  # 0x142ae8fc0
140b1b9a3: f3 0f 10 3d 65 b3 ef 01     	movss	0x1efb365(%rip), %xmm7  # 0x142a16d10
140b1b9ab: f3 44 0f 10 1d bc 9e e0 01  	movss	0x1e09ebc(%rip), %xmm11 # 0x142925870
140b1b9b4: f3 44 0f 10 2d 47 9f e0 01  	movss	0x1e09f47(%rip), %xmm13 # 0x142925904
140b1b9bd: 44 0f 28 35 fb 16 eb 01     	movaps	0x1eb16fb(%rip), %xmm14 # 0x1429cd0c0
140b1b9c5: f2 44 0f 10 3d e2 8b e1 01  	movsd	0x1e18be2(%rip), %xmm15 # 0x1429345b0
140b1b9ce: f3 44 0f 10 25 89 12 f7 01  	movss	0x1f71289(%rip), %xmm12 # 0x142a8cc60
140b1b9d7: 44 0f 28 15 f1 d5 fc 01     	movaps	0x1fcd5f1(%rip), %xmm10 # 0x142ae8fd0
140b1b9df: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
140b1b9e8: 45 31 e4                    	xorl	%r12d, %r12d
140b1b9eb: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
140b1b9f4: 0f 28 c6                    	movaps	%xmm6, %xmm0
140b1b9f7: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
140b1b9fc: e8 cf 85 e0 01              	callq	0x142923fd0 <.text+0x2922fd0>
140b1ba01: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
140b1ba05: 0f 28 c6                    	movaps	%xmm6, %xmm0
140b1ba08: f3 0f 59 c7                 	mulss	%xmm7, %xmm0
140b1ba0c: e8 9f 84 e0 01              	callq	0x142923eb0 <.text+0x2922eb0>
140b1ba11: 0f 28 f8                    	movaps	%xmm0, %xmm7
140b1ba14: f3 41 0f 59 f8              	mulss	%xmm8, %xmm7
140b1ba19: f3 41 0f 59 fb              	mulss	%xmm11, %xmm7
140b1ba1e: f3 44 0f 12 c7              	movsldup	%xmm7, %xmm8            # xmm8 = xmm7[0,0,2,2]
140b1ba23: f3 44 0f 10 0d d4 21 ed 01  	movss	0x1ed21d4(%rip), %xmm9  # 0x1429edc00
140b1ba2c: 44 0f 28 1d bc d5 fc 01     	movaps	0x1fcd5bc(%rip), %xmm11 # 0x142ae8ff0
140b1ba34: 4c 8b 4c 24 78              	movq	0x78(%rsp), %r9
140b1ba39: 48 8b 54 24 40              	movq	0x40(%rsp), %rdx
140b1ba3e: 80 7c 24 23 00              	cmpb	$0x0, 0x23(%rsp)
140b1ba43: 74 4b                       	je	0x140b1ba90 <.text+0xb1aa90>
140b1ba45: 4c 39 e2                    	cmpq	%r12, %rdx
140b1ba48: 0f 85 3d 01 00 00           	jne	0x140b1bb8b <.text+0xb1ab8b>
140b1ba4e: 49 39 dd                    	cmpq	%rbx, %r13
140b1ba51: 0f 84 44 05 00 00           	je	0x140b1bf9b <.text+0xb1af9b>
140b1ba57: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
140b1ba5c: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
140b1ba63: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140b1ba70: 41 8b 4d 00                 	movl	(%r13), %ecx
140b1ba74: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140b1ba78: 4c 8b 64 c8 10              	movq	0x10(%rax,%rcx,8), %r12
140b1ba7d: 4d 85 e4                    	testq	%r12, %r12
140b1ba80: 75 52                       	jne	0x140b1bad4 <.text+0xb1aad4>
140b1ba82: 49 83 c5 04                 	addq	$0x4, %r13
140b1ba86: 49 39 dd                    	cmpq	%rbx, %r13
140b1ba89: 75 e5                       	jne	0x140b1ba70 <.text+0xb1aa70>
140b1ba8b: e9 0b 05 00 00              	jmp	0x140b1bf9b <.text+0xb1af9b>
140b1ba90: 4c 39 e2                    	cmpq	%r12, %rdx
140b1ba93: 0f 85 31 01 00 00           	jne	0x140b1bbca <.text+0xb1abca>
140b1ba99: 49 39 dd                    	cmpq	%rbx, %r13
140b1ba9c: 0f 84 f9 04 00 00           	je	0x140b1bf9b <.text+0xb1af9b>
140b1baa2: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
140b1baa7: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
140b1baae: 66 90                       	nop
140b1bab0: 41 8b 4d 00                 	movl	(%r13), %ecx
140b1bab4: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140b1bab8: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b1babc: 4c 8b 64 08 58              	movq	0x58(%rax,%rcx), %r12
140b1bac1: 4d 85 e4                    	testq	%r12, %r12
140b1bac4: 75 70                       	jne	0x140b1bb36 <.text+0xb1ab36>
140b1bac6: 49 83 c5 04                 	addq	$0x4, %r13
140b1baca: 49 39 dd                    	cmpq	%rbx, %r13
140b1bacd: 75 e1                       	jne	0x140b1bab0 <.text+0xb1aab0>
140b1bacf: e9 c7 04 00 00              	jmp	0x140b1bf9b <.text+0xb1af9b>
140b1bad4: 4c 8d 04 c8                 	leaq	(%rax,%rcx,8), %r8
140b1bad8: 48 8b 44 24 48              	movq	0x48(%rsp), %rax
140b1badd: 4c 8b 88 10 01 00 00        	movq	0x110(%rax), %r9
140b1bae4: 48 8b 88 18 01 00 00        	movq	0x118(%rax), %rcx
140b1baeb: 49 8b 40 18                 	movq	0x18(%r8), %rax
140b1baef: 49 8b 50 38                 	movq	0x38(%r8), %rdx
140b1baf3: 4e 8b 0c ca                 	movq	(%rdx,%r9,8), %r9
140b1baf7: 49 f7 d1                    	notq	%r9
140b1bafa: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
140b1bafe: 49 c1 e1 04                 	shlq	$0x4, %r9
140b1bb02: 4e 8b 54 08 10              	movq	0x10(%rax,%r9), %r10
140b1bb07: 4c 89 54 24 38              	movq	%r10, 0x38(%rsp)
140b1bb0c: 4e 8b 4c 08 28              	movq	0x28(%rax,%r9), %r9
140b1bb11: 49 83 c5 04                 	addq	$0x4, %r13
140b1bb15: 49 39 48 40                 	cmpq	%rcx, 0x40(%r8)
140b1bb19: 76 6c                       	jbe	0x140b1bb87 <.text+0xb1ab87>
140b1bb1b: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
140b1bb1f: 48 85 c9                    	testq	%rcx, %rcx
140b1bb22: 74 63                       	je	0x140b1bb87 <.text+0xb1ab87>
140b1bb24: 48 f7 d1                    	notq	%rcx
140b1bb27: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b1bb2b: 48 c1 e1 04                 	shlq	$0x4, %rcx
140b1bb2f: 48 8b 6c 08 10              	movq	0x10(%rax,%rcx), %rbp
140b1bb34: eb 53                       	jmp	0x140b1bb89 <.text+0xb1ab89>
140b1bb36: 48 01 c8                    	addq	%rcx, %rax
140b1bb39: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
140b1bb3f: 48 8b 54 24 50              	movq	0x50(%rsp), %rdx
140b1bb44: 4c 8b 8a a8 01 00 00        	movq	0x1a8(%rdx), %r9
140b1bb4b: 4c 8d 14 c9                 	leaq	(%rcx,%rcx,8), %r10
140b1bb4f: 48 8b 4c 24 48              	movq	0x48(%rsp), %rcx
140b1bb54: 4c 8b 81 18 01 00 00        	movq	0x118(%rcx), %r8
140b1bb5b: 4b 8b 4c d1 18              	movq	0x18(%r9,%r10,8), %rcx
140b1bb60: 4b 8b 54 d1 38              	movq	0x38(%r9,%r10,8), %rdx
140b1bb65: 4f 39 44 d1 40              	cmpq	%r8, 0x40(%r9,%r10,8)
140b1bb6a: 76 23                       	jbe	0x140b1bb8f <.text+0xb1ab8f>
140b1bb6c: 4e 8b 04 c2                 	movq	(%rdx,%r8,8), %r8
140b1bb70: 4d 85 c0                    	testq	%r8, %r8
140b1bb73: 74 1a                       	je	0x140b1bb8f <.text+0xb1ab8f>
140b1bb75: 49 f7 d0                    	notq	%r8
140b1bb78: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140b1bb7c: 49 c1 e0 04                 	shlq	$0x4, %r8
140b1bb80: 4a 8b 6c 01 10              	movq	0x10(%rcx,%r8), %rbp
140b1bb85: eb 0a                       	jmp	0x140b1bb91 <.text+0xb1ab91>
140b1bb87: 31 ed                       	xorl	%ebp, %ebp
140b1bb89: 31 d2                       	xorl	%edx, %edx
140b1bb8b: 89 d0                       	movl	%edx, %eax
140b1bb8d: eb 4b                       	jmp	0x140b1bbda <.text+0xb1abda>
140b1bb8f: 31 ed                       	xorl	%ebp, %ebp
140b1bb91: 4c 8b 44 24 48              	movq	0x48(%rsp), %r8
140b1bb96: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
140b1bb9d: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
140b1bba1: 48 f7 d2                    	notq	%rdx
140b1bba4: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140b1bba8: 48 c1 e2 04                 	shlq	$0x4, %rdx
140b1bbac: 4c 8b 44 11 10              	movq	0x10(%rcx,%rdx), %r8
140b1bbb1: 4c 89 44 24 38              	movq	%r8, 0x38(%rsp)
140b1bbb6: 4c 8b 4c 11 28              	movq	0x28(%rcx,%rdx), %r9
140b1bbbb: 49 83 c5 04                 	addq	$0x4, %r13
140b1bbbf: 48 8b 40 50                 	movq	0x50(%rax), %rax
140b1bbc3: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
140b1bbc8: 31 d2                       	xorl	%edx, %edx
140b1bbca: 48 89 d0                    	movq	%rdx, %rax
140b1bbcd: 48 c1 e0 04                 	shlq	$0x4, %rax
140b1bbd1: 48 8b 4c 24 70              	movq	0x70(%rsp), %rcx
140b1bbd6: 8b 44 01 08                 	movl	0x8(%rcx,%rax), %eax
140b1bbda: 4c 8d 34 40                 	leaq	(%rax,%rax,2), %r14
140b1bbde: 49 c1 e6 04                 	shlq	$0x4, %r14
140b1bbe2: 4c 03 74 24 38              	addq	0x38(%rsp), %r14
140b1bbe7: 4d 8d 14 81                 	leaq	(%r9,%rax,4), %r10
140b1bbeb: 48 8d 34 c5 00 00 00 00     	leaq	(,%rax,8), %rsi
140b1bbf3: 48 01 ee                    	addq	%rbp, %rsi
140b1bbf6: 48 ff c2                    	incq	%rdx
140b1bbf9: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
140b1bbfe: 0f 84 7c 00 00 00           	je	0x140b1bc80 <.text+0xb1ac80>
140b1bc04: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
140b1bc09: 4c 89 4c 24 78              	movq	%r9, 0x78(%rsp)
140b1bc0e: 48 89 6c 24 60              	movq	%rbp, 0x60(%rsp)
140b1bc13: 4c 89 fd                    	movq	%r15, %rbp
140b1bc16: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140b1bc1a: 48 8b 0f                    	movq	(%rdi), %rcx
140b1bc1d: 49 89 ff                    	movq	%rdi, %r15
140b1bc20: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
140b1bc24: 48 39 c8                    	cmpq	%rcx, %rax
140b1bc27: 41 b8 00 00 00 00           	movl	$0x0, %r8d
140b1bc2d: 4c 0f 43 c1                 	cmovaeq	%rcx, %r8
140b1bc31: 4c 29 c0                    	subq	%r8, %rax
140b1bc34: 48 c1 e0 06                 	shlq	$0x6, %rax
140b1bc38: 0f b6 7c 02 38              	movzbl	0x38(%rdx,%rax), %edi
140b1bc3d: 8b 44 24 24                 	movl	0x24(%rsp), %eax
140b1bc41: 4c 89 54 24 58              	movq	%r10, 0x58(%rsp)
140b1bc46: 41 89 02                    	movl	%eax, (%r10)
140b1bc49: 48 8d 8c 24 94 00 00 00     	leaq	0x94(%rsp), %rcx
140b1bc51: 48 8b 94 24 88 00 00 00     	movq	0x88(%rsp), %rdx
140b1bc59: e8 e2 b0 17 00              	callq	0x140c96d40 <.text+0xc95d40>
140b1bc5e: 40 84 ff                    	testb	%dil, %dil
140b1bc61: 74 4d                       	je	0x140b1bcb0 <.text+0xb1acb0>
140b1bc63: f2 0f 10 05 95 20 ec 01     	movsd	0x1ec2095(%rip), %xmm0  # 0x1429ddd00
140b1bc6b: f2 0f 11 44 24 28           	movsd	%xmm0, 0x28(%rsp)
140b1bc71: c7 44 24 30 00 00 00 80     	movl	$0x80000000, 0x30(%rsp) # imm = 0x80000000
140b1bc79: eb 4c                       	jmp	0x140b1bcc7 <.text+0xb1acc7>
140b1bc7b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1bc80: 41 f6 07 01                 	testb	$0x1, (%r15)
140b1bc84: 0f 84 65 01 00 00           	je	0x140b1bdef <.text+0xb1adef>
140b1bc8a: 48 8b 8c 24 80 00 00 00     	movq	0x80(%rsp), %rcx
140b1bc92: 8b 41 08                    	movl	0x8(%rcx), %eax
140b1bc95: 89 44 24 30                 	movl	%eax, 0x30(%rsp)
140b1bc99: 48 8b 01                    	movq	(%rcx), %rax
140b1bc9c: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140b1bca1: e9 5a 01 00 00              	jmp	0x140b1be00 <.text+0xb1ae00>
140b1bca6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140b1bcb0: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
140b1bcba: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140b1bcbf: c7 44 24 30 00 00 00 00     	movl	$0x0, 0x30(%rsp)
140b1bcc7: 4c 89 f1                    	movq	%r14, %rcx
140b1bcca: 48 8d 94 24 94 00 00 00     	leaq	0x94(%rsp), %rdx
140b1bcd2: 4c 8d 44 24 28              	leaq	0x28(%rsp), %r8
140b1bcd7: e8 24 11 18 00              	callq	0x140c9ce00 <.text+0xc9be00>
140b1bcdc: 48 83 3e 01                 	cmpq	$0x1, (%rsi)
140b1bce0: 4c 89 ff                    	movq	%r15, %rdi
140b1bce3: 0f 85 34 01 00 00           	jne	0x140b1be1d <.text+0xb1ae1d>
140b1bce9: 49 89 ef                    	movq	%rbp, %r15
140b1bcec: f6 45 00 01                 	testb	$0x1, (%rbp)
140b1bcf0: 4c 8b 4c 24 78              	movq	0x78(%rsp), %r9
140b1bcf5: 74 19                       	je	0x140b1bd10 <.text+0xb1ad10>
140b1bcf7: f2 41 0f 10 47 04           	movsd	0x4(%r15), %xmm0
140b1bcfd: f3 41 0f 10 57 0c           	movss	0xc(%r15), %xmm2
140b1bd03: eb 11                       	jmp	0x140b1bd16 <.text+0xb1ad16>
140b1bd05: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140b1bd10: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b1bd13: 0f 57 d2                    	xorps	%xmm2, %xmm2
140b1bd16: 48 8b 6c 24 60              	movq	0x60(%rsp), %rbp
140b1bd1b: 48 8b 54 24 40              	movq	0x40(%rsp), %rdx
140b1bd20: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
140b1bd25: 41 0f 28 0e                 	movaps	(%r14), %xmm1
140b1bd29: 0f 28 e1                    	movaps	%xmm1, %xmm4
140b1bd2c: 0f 59 e1                    	mulps	%xmm1, %xmm4
140b1bd2f: f3 0f 16 dc                 	movshdup	%xmm4, %xmm3            # xmm3 = xmm4[1,1,3,3]
140b1bd33: f3 0f 58 dc                 	addss	%xmm4, %xmm3
140b1bd37: 0f 28 ec                    	movaps	%xmm4, %xmm5
140b1bd3a: 66 0f 15 ec                 	unpckhpd	%xmm4, %xmm5            # xmm5 = xmm5[1],xmm4[1]
140b1bd3e: f3 0f 58 eb                 	addss	%xmm3, %xmm5
140b1bd42: 0f c6 ed 00                 	shufps	$0x0, %xmm5, %xmm5      # xmm5 = xmm5[0,0,0,0]
140b1bd46: 0f c6 e4 ff                 	shufps	$0xff, %xmm4, %xmm4     # xmm4 = xmm4[3,3,3,3]
140b1bd4a: 0f 5c e5                    	subps	%xmm5, %xmm4
140b1bd4d: 41 0f 59 e5                 	mulps	%xmm13, %xmm4
140b1bd51: 0f 28 d9                    	movaps	%xmm1, %xmm3
140b1bd54: 41 0f 59 dd                 	mulps	%xmm13, %xmm3
140b1bd58: f3 0f 16 eb                 	movshdup	%xmm3, %xmm5            # xmm5 = xmm3[1,1,3,3]
140b1bd5c: f3 0f 58 eb                 	addss	%xmm3, %xmm5
140b1bd60: 0f 12 db                    	movhlps	%xmm3, %xmm3            # xmm3 = xmm3[1,1]
140b1bd63: f3 0f 58 dd                 	addss	%xmm5, %xmm3
140b1bd67: f3 0f 58 db                 	addss	%xmm3, %xmm3
140b1bd6b: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
140b1bd6f: 0f 59 d9                    	mulps	%xmm1, %xmm3
140b1bd72: 0f 58 dc                    	addps	%xmm4, %xmm3
140b1bd75: 0f 28 e1                    	movaps	%xmm1, %xmm4
140b1bd78: 0f c6 e1 d2                 	shufps	$0xd2, %xmm1, %xmm4     # xmm4 = xmm4[2,0],xmm1[1,3]
140b1bd7c: 41 0f 59 e6                 	mulps	%xmm14, %xmm4
140b1bd80: 0f 28 e9                    	movaps	%xmm1, %xmm5
140b1bd83: 0f c6 e9 c9                 	shufps	$0xc9, %xmm1, %xmm5     # xmm5 = xmm5[1,2],xmm1[0,3]
140b1bd87: 41 0f 59 ef                 	mulps	%xmm15, %xmm5
140b1bd8b: 0f 5c ec                    	subps	%xmm4, %xmm5
140b1bd8e: 0f 58 c9                    	addps	%xmm1, %xmm1
140b1bd91: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
140b1bd95: 0f 59 cd                    	mulps	%xmm5, %xmm1
140b1bd98: 0f 58 cb                    	addps	%xmm3, %xmm1
140b1bd9b: 0f 28 d9                    	movaps	%xmm1, %xmm3
140b1bd9e: 66 0f 15 d9                 	unpckhpd	%xmm1, %xmm3            # xmm3 = xmm3[1],xmm1[1]
140b1bda2: 0f 28 e3                    	movaps	%xmm3, %xmm4
140b1bda5: f3 0f 59 e7                 	mulss	%xmm7, %xmm4
140b1bda9: f3 41 0f 59 e4              	mulss	%xmm12, %xmm4
140b1bdae: f3 0f 5c d4                 	subss	%xmm4, %xmm2
140b1bdb2: f3 41 0f 59 d9              	mulss	%xmm9, %xmm3
140b1bdb7: f3 41 0f 59 dc              	mulss	%xmm12, %xmm3
140b1bdbc: f3 0f 58 da                 	addss	%xmm2, %xmm3
140b1bdc0: 8b 44 24 24                 	movl	0x24(%rsp), %eax
140b1bdc4: 89 01                       	movl	%eax, (%rcx)
140b1bdc6: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
140b1bdca: 0f 59 d1                    	mulps	%xmm1, %xmm2
140b1bdcd: 41 0f 59 d2                 	mulps	%xmm10, %xmm2
140b1bdd1: 0f 5c c2                    	subps	%xmm2, %xmm0
140b1bdd4: 41 0f 59 cb                 	mulps	%xmm11, %xmm1
140b1bdd8: 41 0f 59 ca                 	mulps	%xmm10, %xmm1
140b1bddc: 0f 58 c8                    	addps	%xmm0, %xmm1
140b1bddf: 41 0f 13 4e 10              	movlps	%xmm1, 0x10(%r14)
140b1bde4: f3 41 0f 11 5e 18           	movss	%xmm3, 0x18(%r14)
140b1bdea: e9 4f fc ff ff              	jmp	0x140b1ba3e <.text+0xb1aa3e>
140b1bdef: c7 44 24 30 00 00 00 00     	movl	$0x0, 0x30(%rsp)
140b1bdf7: 48 c7 44 24 28 00 00 00 00  	movq	$0x0, 0x28(%rsp)
140b1be00: 8b 44 24 24                 	movl	0x24(%rsp), %eax
140b1be04: 41 89 02                    	movl	%eax, (%r10)
140b1be07: 8b 44 24 30                 	movl	0x30(%rsp), %eax
140b1be0b: 41 89 46 18                 	movl	%eax, 0x18(%r14)
140b1be0f: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
140b1be14: 49 89 46 10                 	movq	%rax, 0x10(%r14)
140b1be18: e9 21 fc ff ff              	jmp	0x140b1ba3e <.text+0xb1aa3e>
140b1be1d: f3 0f 58 35 c3 1e ec 01     	addss	0x1ec1ec3(%rip), %xmm6  # 0x1429ddce8
140b1be25: 0f 57 c9                    	xorps	%xmm1, %xmm1
140b1be28: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b1be2b: f3 0f 5f c6                 	maxss	%xmm6, %xmm0
140b1be2f: f3 0f 10 35 cd 9a e0 01     	movss	0x1e09acd(%rip), %xmm6  # 0x142925904
140b1be37: f3 0f 5d f0                 	minss	%xmm0, %xmm6
140b1be3b: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b1be3e: 49 89 ef                    	movq	%rbp, %r15
140b1be41: f6 45 00 01                 	testb	$0x1, (%rbp)
140b1be45: 74 0c                       	je	0x140b1be53 <.text+0xb1ae53>
140b1be47: f2 41 0f 10 47 04           	movsd	0x4(%r15), %xmm0
140b1be4d: f3 41 0f 10 4f 0c           	movss	0xc(%r15), %xmm1
140b1be53: f3 0f 11 4c 24 6c           	movss	%xmm1, 0x6c(%rsp)
140b1be59: 0f 29 84 24 a0 00 00 00     	movaps	%xmm0, 0xa0(%rsp)
140b1be61: 45 0f 28 06                 	movaps	(%r14), %xmm8
140b1be65: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
140b1be69: 41 0f 59 c8                 	mulps	%xmm8, %xmm1
140b1be6d: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
140b1be71: f3 0f 58 c1                 	addss	%xmm1, %xmm0
140b1be75: 0f 28 d1                    	movaps	%xmm1, %xmm2
140b1be78: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
140b1be7c: f3 0f 58 d0                 	addss	%xmm0, %xmm2
140b1be80: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
140b1be84: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
140b1be88: 0f 5c ca                    	subps	%xmm2, %xmm1
140b1be8b: 41 0f 59 cd                 	mulps	%xmm13, %xmm1
140b1be8f: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
140b1be93: 41 0f 59 c5                 	mulps	%xmm13, %xmm0
140b1be97: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
140b1be9b: f3 0f 58 d0                 	addss	%xmm0, %xmm2
140b1be9f: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
140b1bea2: f3 0f 58 c2                 	addss	%xmm2, %xmm0
140b1bea6: f3 0f 58 c0                 	addss	%xmm0, %xmm0
140b1beaa: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b1beae: 41 0f 59 c0                 	mulps	%xmm8, %xmm0
140b1beb2: 0f 58 c1                    	addps	%xmm1, %xmm0
140b1beb5: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
140b1beb9: 41 0f c6 c8 d2              	shufps	$0xd2, %xmm8, %xmm1     # xmm1 = xmm1[2,0],xmm8[1,3]
140b1bebe: 41 0f 59 ce                 	mulps	%xmm14, %xmm1
140b1bec2: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
140b1bec6: 41 0f c6 d0 c9              	shufps	$0xc9, %xmm8, %xmm2     # xmm2 = xmm2[1,2],xmm8[0,3]
140b1becb: 41 0f 59 d7                 	mulps	%xmm15, %xmm2
140b1becf: 0f 5c d1                    	subps	%xmm1, %xmm2
140b1bed2: 45 0f 58 c0                 	addps	%xmm8, %xmm8
140b1bed6: 45 0f c6 c0 ff              	shufps	$0xff, %xmm8, %xmm8     # xmm8 = xmm8[3,3,3,3]
140b1bedb: 44 0f 59 c2                 	mulps	%xmm2, %xmm8
140b1bedf: 44 0f 58 c0                 	addps	%xmm0, %xmm8
140b1bee3: 0f 28 c6                    	movaps	%xmm6, %xmm0
140b1bee6: f3 44 0f 10 0d d1 d0 fc 01  	movss	0x1fcd0d1(%rip), %xmm9  # 0x142ae8fc0
140b1beef: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
140b1bef4: e8 d7 80 e0 01              	callq	0x142923fd0 <.text+0x2922fd0>
140b1bef9: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
140b1befd: 0f 28 c6                    	movaps	%xmm6, %xmm0
140b1bf00: f3 0f 10 3d 08 ae ef 01     	movss	0x1efae08(%rip), %xmm7  # 0x142a16d10
140b1bf08: f3 0f 59 c7                 	mulss	%xmm7, %xmm0
140b1bf0c: e8 9f 7f e0 01              	callq	0x142923eb0 <.text+0x2922eb0>
140b1bf11: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
140b1bf16: f3 44 0f 10 1d 51 99 e0 01  	movss	0x1e09951(%rip), %xmm11 # 0x142925870
140b1bf1f: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
140b1bf24: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
140b1bf28: 66 41 0f 15 c8              	unpckhpd	%xmm8, %xmm1            # xmm1 = xmm1[1],xmm8[1]
140b1bf2d: 0f 28 d1                    	movaps	%xmm1, %xmm2
140b1bf30: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
140b1bf34: f3 41 0f 59 d4              	mulss	%xmm12, %xmm2
140b1bf39: f3 0f 10 5c 24 6c           	movss	0x6c(%rsp), %xmm3
140b1bf3f: f3 0f 5c da                 	subss	%xmm2, %xmm3
140b1bf43: f3 0f 59 0d d5 99 e0 01     	mulss	0x1e099d5(%rip), %xmm1  # 0x142925920
140b1bf4b: f3 41 0f 59 cc              	mulss	%xmm12, %xmm1
140b1bf50: f3 0f 58 cb                 	addss	%xmm3, %xmm1
140b1bf54: 8b 44 24 24                 	movl	0x24(%rsp), %eax
140b1bf58: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
140b1bf5d: 89 01                       	movl	%eax, (%rcx)
140b1bf5f: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
140b1bf63: 41 0f 59 c0                 	mulps	%xmm8, %xmm0
140b1bf67: 41 0f 59 c2                 	mulps	%xmm10, %xmm0
140b1bf6b: 0f 28 94 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm2
140b1bf73: 0f 5c d0                    	subps	%xmm0, %xmm2
140b1bf76: 44 0f 59 05 62 d0 fc 01     	mulps	0x1fcd062(%rip), %xmm8  # 0x142ae8fe0
140b1bf7e: 45 0f 59 c2                 	mulps	%xmm10, %xmm8
140b1bf82: 44 0f 58 c2                 	addps	%xmm2, %xmm8
140b1bf86: 45 0f 13 46 10              	movlps	%xmm8, 0x10(%r14)
140b1bf8b: f3 41 0f 11 4e 18           	movss	%xmm1, 0x18(%r14)
140b1bf91: 48 8b 6c 24 60              	movq	0x60(%rsp), %rbp
140b1bf96: e9 59 fa ff ff              	jmp	0x140b1b9f4 <.text+0xb1a9f4>
140b1bf9b: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
140b1bfa3: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
140b1bfab: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
140b1bfb4: 44 0f 28 8c 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm9
140b1bfbd: 44 0f 28 94 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm10
140b1bfc6: 44 0f 28 9c 24 00 01 00 00  	movaps	0x100(%rsp), %xmm11
140b1bfcf: 44 0f 28 a4 24 10 01 00 00  	movaps	0x110(%rsp), %xmm12
140b1bfd8: 44 0f 28 ac 24 20 01 00 00  	movaps	0x120(%rsp), %xmm13
140b1bfe1: 44 0f 28 b4 24 30 01 00 00  	movaps	0x130(%rsp), %xmm14
140b1bfea: 44 0f 28 bc 24 40 01 00 00  	movaps	0x140(%rsp), %xmm15
140b1bff3: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
140b1bffa: 5b                          	popq	%rbx
140b1bffb: 5d                          	popq	%rbp
140b1bffc: 5f                          	popq	%rdi
140b1bffd: 5e                          	popq	%rsi
140b1bffe: 41 5c                       	popq	%r12
140b1c000: 41 5d                       	popq	%r13
140b1c002: 41 5e                       	popq	%r14
140b1c004: 41 5f                       	popq	%r15
140b1c006: c3                          	retq
