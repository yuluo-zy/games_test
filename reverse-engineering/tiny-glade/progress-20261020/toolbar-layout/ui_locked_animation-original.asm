
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14233c8b0: 41 57                       	pushq	%r15
14233c8b2: 41 56                       	pushq	%r14
14233c8b4: 41 55                       	pushq	%r13
14233c8b6: 41 54                       	pushq	%r12
14233c8b8: 56                          	pushq	%rsi
14233c8b9: 57                          	pushq	%rdi
14233c8ba: 55                          	pushq	%rbp
14233c8bb: 53                          	pushq	%rbx
14233c8bc: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
14233c8c3: 44 0f 29 94 24 c0 00 00 00  	movaps	%xmm10, 0xc0(%rsp)
14233c8cc: 44 0f 29 8c 24 b0 00 00 00  	movaps	%xmm9, 0xb0(%rsp)
14233c8d5: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
14233c8de: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14233c8e6: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
14233c8ee: 48 8b 19                    	movq	(%rcx), %rbx
14233c8f1: 4c 8b 51 08                 	movq	0x8(%rcx), %r10
14233c8f5: 48 8b 33                    	movq	(%rbx), %rsi
14233c8f8: 45 31 db                    	xorl	%r11d, %r11d
14233c8fb: 48 89 f0                    	movq	%rsi, %rax
14233c8fe: 49 2b 42 18                 	subq	0x18(%r10), %rax
14233c902: 49 0f 42 c3                 	cmovbq	%r11, %rax
14233c906: 49 2b 72 38                 	subq	0x38(%r10), %rsi
14233c90a: 49 0f 42 f3                 	cmovbq	%r11, %rsi
14233c90e: 48 89 c5                    	movq	%rax, %rbp
14233c911: 48 c1 e5 04                 	shlq	$0x4, %rbp
14233c915: 49 03 6a 08                 	addq	0x8(%r10), %rbp
14233c919: 4d 8b 72 10                 	movq	0x10(%r10), %r14
14233c91d: 49 8b 4a 30                 	movq	0x30(%r10), %rcx
14233c921: 49 29 c6                    	subq	%rax, %r14
14233c924: 4d 0f 42 f3                 	cmovbq	%r11, %r14
14233c928: bf 08 00 00 00              	movl	$0x8, %edi
14233c92d: 48 0f 42 ef                 	cmovbq	%rdi, %rbp
14233c931: 48 89 f0                    	movq	%rsi, %rax
14233c934: 48 c1 e0 04                 	shlq	$0x4, %rax
14233c938: 49 03 42 28                 	addq	0x28(%r10), %rax
14233c93c: 48 29 f1                    	subq	%rsi, %rcx
14233c93f: 49 0f 42 cb                 	cmovbq	%r11, %rcx
14233c943: 48 0f 42 c7                 	cmovbq	%rdi, %rax
14233c947: 48 89 cf                    	movq	%rcx, %rdi
14233c94a: 4d 8b 52 40                 	movq	0x40(%r10), %r10
14233c94e: 49 29 ca                    	subq	%rcx, %r10
14233c951: 4d 29 f2                    	subq	%r14, %r10
14233c954: 4c 89 13                    	movq	%r10, (%rbx)
14233c957: 49 c1 e6 04                 	shlq	$0x4, %r14
14233c95b: 48 c1 e7 04                 	shlq	$0x4, %rdi
14233c95f: 49 8b 49 08                 	movq	0x8(%r9), %rcx
14233c963: 48 85 c9                    	testq	%rcx, %rcx
14233c966: 75 04                       	jne	0x14233c96c <.text+0x233b96c>
14233c968: 49 8b 49 10                 	movq	0x10(%r9), %rcx
14233c96c: 4c 8b 9c 24 40 01 00 00     	movq	0x140(%rsp), %r11
14233c974: 49 01 ee                    	addq	%rbp, %r14
14233c977: 48 01 c7                    	addq	%rax, %rdi
14233c97a: 4c 8b 0a                    	movq	(%rdx), %r9
14233c97d: 4c 89 4c 24 28              	movq	%r9, 0x28(%rsp)
14233c982: 4c 8b 6a 08                 	movq	0x8(%rdx), %r13
14233c986: f2 0f 10 35 52 0d d1 00     	movsd	0xd10d52(%rip), %xmm6   # 0x14304d6e0
14233c98e: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
14233c993: 48 89 7c 24 40              	movq	%rdi, 0x40(%rsp)
14233c998: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14233c9a0: 48 85 ed                    	testq	%rbp, %rbp
14233c9a3: 74 1b                       	je	0x14233c9c0 <.text+0x233b9c0>
14233c9a5: 4c 39 f5                    	cmpq	%r14, %rbp
14233c9a8: 74 16                       	je	0x14233c9c0 <.text+0x233b9c0>
14233c9aa: 4c 8d 7d 10                 	leaq	0x10(%rbp), %r15
14233c9ae: 48 89 c6                    	movq	%rax, %rsi
14233c9b1: eb 2c                       	jmp	0x14233c9df <.text+0x233b9df>
14233c9b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233c9c0: 48 85 c0                    	testq	%rax, %rax
14233c9c3: 0f 84 28 01 00 00           	je	0x14233caf1 <.text+0x233baf1>
14233c9c9: 48 39 f8                    	cmpq	%rdi, %rax
14233c9cc: 0f 84 1f 01 00 00           	je	0x14233caf1 <.text+0x233baf1>
14233c9d2: 48 8d 70 10                 	leaq	0x10(%rax), %rsi
14233c9d6: 45 31 ff                    	xorl	%r15d, %r15d
14233c9d9: 48 89 c5                    	movq	%rax, %rbp
14233c9dc: 48 89 f0                    	movq	%rsi, %rax
14233c9df: 48 ff 03                    	incq	(%rbx)
14233c9e2: 4c 8b 65 08                 	movq	0x8(%rbp), %r12
14233c9e6: 44 89 e2                    	movl	%r12d, %edx
14233c9e9: 4c 89 fd                    	movq	%r15, %rbp
14233c9ec: 49 3b 55 10                 	cmpq	0x10(%r13), %rdx
14233c9f0: 73 ae                       	jae	0x14233c9a0 <.text+0x233b9a0>
14233c9f2: 4d 8b 4d 08                 	movq	0x8(%r13), %r9
14233c9f6: 4d 89 e2                    	movq	%r12, %r10
14233c9f9: 49 c1 ea 20                 	shrq	$0x20, %r10
14233c9fd: 48 8d 14 92                 	leaq	(%rdx,%rdx,4), %rdx
14233ca01: 48 89 f0                    	movq	%rsi, %rax
14233ca04: 4c 89 fd                    	movq	%r15, %rbp
14233ca07: 45 39 14 91                 	cmpl	%r10d, (%r9,%rdx,4)
14233ca0b: 75 93                       	jne	0x14233c9a0 <.text+0x233b9a0>
14233ca0d: 49 8d 04 91                 	leaq	(%r9,%rdx,4), %rax
14233ca11: 8b 50 04                    	movl	0x4(%rax), %edx
14233ca14: 48 89 f0                    	movq	%rsi, %rax
14233ca17: 4c 89 fd                    	movq	%r15, %rbp
14233ca1a: 41 b9 ff ff ff ff           	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
14233ca20: 4c 39 ca                    	cmpq	%r9, %rdx
14233ca23: 0f 84 77 ff ff ff           	je	0x14233c9a0 <.text+0x233b9a0>
14233ca29: 48 89 f0                    	movq	%rsi, %rax
14233ca2c: 4c 89 fd                    	movq	%r15, %rbp
14233ca2f: 4c 8b 4c 24 28              	movq	0x28(%rsp), %r9
14233ca34: 49 39 51 38                 	cmpq	%rdx, 0x38(%r9)
14233ca38: 0f 86 62 ff ff ff           	jbe	0x14233c9a0 <.text+0x233b9a0>
14233ca3e: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
14233ca43: 48 8b 40 28                 	movq	0x28(%rax), %rax
14233ca47: 41 89 d1                    	movl	%edx, %r9d
14233ca4a: 41 c1 e9 06                 	shrl	$0x6, %r9d
14233ca4e: 4a 8b 04 c8                 	movq	(%rax,%r9,8), %rax
14233ca52: 48 0f a3 d0                 	btq	%rdx, %rax
14233ca56: 48 89 f0                    	movq	%rsi, %rax
14233ca59: 4c 89 fd                    	movq	%r15, %rbp
14233ca5c: 0f 83 3e ff ff ff           	jae	0x14233c9a0 <.text+0x233b9a0>
14233ca62: 48 8b 01                    	movq	(%rcx), %rax
14233ca65: 48 8b 79 10                 	movq	0x10(%rcx), %rdi
14233ca69: 48 29 f8                    	subq	%rdi, %rax
14233ca6c: 48 83 f8 27                 	cmpq	$0x27, %rax
14233ca70: 76 4b                       	jbe	0x14233cabd <.text+0x233babd>
14233ca72: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14233ca76: 48 8d 15 a3 2d ca fe        	leaq	-0x135d25d(%rip), %rdx  # 0x140fdf820 <.text+0xfde820>
14233ca7d: 48 89 14 38                 	movq	%rdx, (%rax,%rdi)
14233ca81: 48 8d 15 d8 2d b1 ff        	leaq	-0x4ed228(%rip), %rdx   # 0x141e4f860 <.text+0x1e4e860>
14233ca88: 48 89 54 38 08              	movq	%rdx, 0x8(%rax,%rdi)
14233ca8d: 4c 89 64 38 10              	movq	%r12, 0x10(%rax,%rdi)
14233ca92: f2 0f 11 74 38 18           	movsd	%xmm6, 0x18(%rax,%rdi)
14233ca98: c7 44 38 20 00 00 00 00     	movl	$0x0, 0x20(%rax,%rdi)
14233caa0: c6 44 38 24 00              	movb	$0x0, 0x24(%rax,%rdi)
14233caa5: 48 83 c7 28                 	addq	$0x28, %rdi
14233caa9: 48 89 79 10                 	movq	%rdi, 0x10(%rcx)
14233caad: 48 89 f0                    	movq	%rsi, %rax
14233cab0: 4c 89 fd                    	movq	%r15, %rbp
14233cab3: 48 8b 7c 24 40              	movq	0x40(%rsp), %rdi
14233cab8: e9 e3 fe ff ff              	jmp	0x14233c9a0 <.text+0x233b9a0>
14233cabd: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14233cac6: 4c 89 44 24 30              	movq	%r8, 0x30(%rsp)
14233cacb: 41 b8 28 00 00 00           	movl	$0x28, %r8d
14233cad1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14233cad7: 48 89 fa                    	movq	%rdi, %rdx
14233cada: 4c 89 dd                    	movq	%r11, %rbp
14233cadd: e8 ee 8d 56 00              	callq	0x1428a58d0 <.text+0x28a48d0>
14233cae2: 49 89 eb                    	movq	%rbp, %r11
14233cae5: 48 8b 4c 24 38              	movq	0x38(%rsp), %rcx
14233caea: 4c 8b 44 24 30              	movq	0x30(%rsp), %r8
14233caef: eb 81                       	jmp	0x14233ca72 <.text+0x233ba72>
14233caf1: 4d 8b 08                    	movq	(%r8), %r9
14233caf4: 49 8b 50 08                 	movq	0x8(%r8), %rdx
14233caf8: 49 8b 81 10 01 00 00        	movq	0x110(%r9), %rax
14233caff: 48 39 82 98 01 00 00        	cmpq	%rax, 0x198(%rdx)
14233cb06: 76 39                       	jbe	0x14233cb41 <.text+0x233bb41>
14233cb08: 48 8b 8a 90 01 00 00        	movq	0x190(%rdx), %rcx
14233cb0f: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14233cb13: 48 85 c0                    	testq	%rax, %rax
14233cb16: 74 29                       	je	0x14233cb41 <.text+0x233bb41>
14233cb18: 48 f7 d0                    	notq	%rax
14233cb1b: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
14233cb1f: 48 c1 e0 04                 	shlq	$0x4, %rax
14233cb23: 48 03 82 60 01 00 00        	addq	0x160(%rdx), %rax
14233cb2a: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14233cb2f: 41 0f b6 81 08 01 00 00     	movzbl	0x108(%r9), %eax
14233cb37: 88 44 24 40                 	movb	%al, 0x40(%rsp)
14233cb3b: 84 c0                       	testb	%al, %al
14233cb3d: 74 5a                       	je	0x14233cb99 <.text+0x233bb99>
14233cb3f: eb 19                       	jmp	0x14233cb5a <.text+0x233bb5a>
14233cb41: 48 c7 44 24 28 00 00 00 00  	movq	$0x0, 0x28(%rsp)
14233cb4a: 41 0f b6 81 08 01 00 00     	movzbl	0x108(%r9), %eax
14233cb52: 88 44 24 40                 	movb	%al, 0x40(%rsp)
14233cb56: 84 c0                       	testb	%al, %al
14233cb58: 74 3f                       	je	0x14233cb99 <.text+0x233bb99>
14233cb5a: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14233cb62: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14233cb6a: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14233cb73: 44 0f 28 8c 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm9
14233cb7c: 44 0f 28 94 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm10
14233cb85: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
14233cb8c: 5b                          	popq	%rbx
14233cb8d: 5d                          	popq	%rbp
14233cb8e: 5f                          	popq	%rdi
14233cb8f: 5e                          	popq	%rsi
14233cb90: 41 5c                       	popq	%r12
14233cb92: 41 5d                       	popq	%r13
14233cb94: 41 5e                       	popq	%r14
14233cb96: 41 5f                       	popq	%r15
14233cb98: c3                          	retq
14233cb99: 41 8b 40 14                 	movl	0x14(%r8), %eax
14233cb9d: 89 44 24 4c                 	movl	%eax, 0x4c(%rsp)
14233cba1: 4d 8b a9 f8 00 00 00        	movq	0xf8(%r9), %r13
14233cba8: 49 8b 81 00 01 00 00        	movq	0x100(%r9), %rax
14233cbaf: 4c 8d 24 85 00 00 00 00     	leaq	(,%rax,4), %r12
14233cbb7: 4d 01 ec                    	addq	%r13, %r12
14233cbba: 49 8b 03                    	movq	(%r11), %rax
14233cbbd: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
14233cbc2: b8 08 00 00 00              	movl	$0x8, %eax
14233cbc7: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14233cbcc: 31 db                       	xorl	%ebx, %ebx
14233cbce: f3 0f 10 3d 2e 8d 5e 00     	movss	0x5e8d2e(%rip), %xmm7   # 0x142925904
14233cbd6: f3 44 0f 10 05 a9 8d 5e 00  	movss	0x5e8da9(%rip), %xmm8   # 0x142925988
14233cbdf: f3 44 0f 10 0d fc d4 61 00  	movss	0x61d4fc(%rip), %xmm9   # 0x14295a0e4
14233cbe8: f3 44 0f 10 15 b3 0f 6b 00  	movss	0x6b0fb3(%rip), %xmm10  # 0x1429edba4
14233cbf1: 31 ed                       	xorl	%ebp, %ebp
14233cbf3: 48 c7 44 24 50 00 00 00 00  	movq	$0x0, 0x50(%rsp)
14233cbfc: 4c 89 4c 24 70              	movq	%r9, 0x70(%rsp)
14233cc01: 48 89 54 24 68              	movq	%rdx, 0x68(%rsp)
14233cc06: 8b 74 24 4c                 	movl	0x4c(%rsp), %esi
14233cc0a: 48 39 eb                    	cmpq	%rbp, %rbx
14233cc0d: 0f 85 a4 00 00 00           	jne	0x14233ccb7 <.text+0x233bcb7>
14233cc13: 4d 39 e5                    	cmpq	%r12, %r13
14233cc16: 0f 84 3e ff ff ff           	je	0x14233cb5a <.text+0x233bb5a>
14233cc1c: 48 8b 82 00 01 00 00        	movq	0x100(%rdx), %rax
14233cc23: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233cc30: 41 8b 4d 00                 	movl	(%r13), %ecx
14233cc34: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233cc38: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233cc3c: 48 8b 6c 08 58              	movq	0x58(%rax,%rcx), %rbp
14233cc41: 48 85 ed                    	testq	%rbp, %rbp
14233cc44: 75 1a                       	jne	0x14233cc60 <.text+0x233bc60>
14233cc46: 49 83 c5 04                 	addq	$0x4, %r13
14233cc4a: 4d 39 e5                    	cmpq	%r12, %r13
14233cc4d: 75 e1                       	jne	0x14233cc30 <.text+0x233bc30>
14233cc4f: e9 06 ff ff ff              	jmp	0x14233cb5a <.text+0x233bb5a>
14233cc54: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14233cc60: 48 01 c8                    	addq	%rcx, %rax
14233cc63: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
14233cc69: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
14233cc70: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14233cc74: 4d 8b 81 18 01 00 00        	movq	0x118(%r9), %r8
14233cc7b: 4c 8b 4c ca 18              	movq	0x18(%rdx,%rcx,8), %r9
14233cc80: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
14233cc85: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
14233cc89: 48 f7 d1                    	notq	%rcx
14233cc8c: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14233cc90: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233cc94: 49 8b 54 09 10              	movq	0x10(%r9,%rcx), %rdx
14233cc99: 48 89 54 24 50              	movq	%rdx, 0x50(%rsp)
14233cc9e: 49 8b 4c 09 28              	movq	0x28(%r9,%rcx), %rcx
14233cca3: 48 89 4c 24 58              	movq	%rcx, 0x58(%rsp)
14233cca8: 48 8b 40 50                 	movq	0x50(%rax), %rax
14233ccac: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14233ccb1: 49 83 c5 04                 	addq	$0x4, %r13
14233ccb5: 31 db                       	xorl	%ebx, %ebx
14233ccb7: 48 89 d8                    	movq	%rbx, %rax
14233ccba: 48 c1 e0 04                 	shlq	$0x4, %rax
14233ccbe: 48 8b 4c 24 30              	movq	0x30(%rsp), %rcx
14233ccc3: 48 8b 14 01                 	movq	(%rcx,%rax), %rdx
14233ccc7: 8b 7c 01 08                 	movl	0x8(%rcx,%rax), %edi
14233cccb: 48 89 54 24 78              	movq	%rdx, 0x78(%rsp)
14233ccd0: 89 d0                       	movl	%edx, %eax
14233ccd2: 48 8b 54 24 28              	movq	0x28(%rsp), %rdx
14233ccd7: 48 8b 8a 80 00 00 00        	movq	0x80(%rdx), %rcx
14233ccde: 8b 44 c1 04                 	movl	0x4(%rcx,%rax,8), %eax
14233cce2: 4c 8b 72 08                 	movq	0x8(%rdx), %r14
14233cce6: 4c 0f af f0                 	imulq	%rax, %r14
14233ccea: 4c 8b 7a 10                 	movq	0x10(%rdx), %r15
14233ccee: 48 8b 4c 24 60              	movq	0x60(%rsp), %rcx
14233ccf3: f3 0f 10 41 58              	movss	0x58(%rcx), %xmm0
14233ccf8: f3 43 0f 5e 44 37 04        	divss	0x4(%r15,%r14), %xmm0
14233ccff: 48 8b 4a 50                 	movq	0x50(%rdx), %rcx
14233cd03: 89 34 81                    	movl	%esi, (%rcx,%rax,4)
14233cd06: f3 43 0f 58 44 37 08        	addss	0x8(%r15,%r14), %xmm0
14233cd0d: 0f 57 c9                    	xorps	%xmm1, %xmm1
14233cd10: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14233cd14: 0f 28 f7                    	movaps	%xmm7, %xmm6
14233cd17: f3 0f 5d f1                 	minss	%xmm1, %xmm6
14233cd1b: f3 43 0f 11 74 37 08        	movss	%xmm6, 0x8(%r15,%r14)
14233cd22: f3 41 0f 59 f0              	mulss	%xmm8, %xmm6
14233cd27: f3 41 0f 58 f1              	addss	%xmm9, %xmm6
14233cd2c: f3 41 0f 59 f2              	mulss	%xmm10, %xmm6
14233cd31: 0f 28 c6                    	movaps	%xmm6, %xmm0
14233cd34: e8 97 72 5e 00              	callq	0x142923fd0 <.text+0x2922fd0>
14233cd39: f3 0f 5e c6                 	divss	%xmm6, %xmm0
14233cd3d: f3 43 0f 59 04 37           	mulss	(%r15,%r14), %xmm0
14233cd43: f3 0f 58 c7                 	addss	%xmm7, %xmm0
14233cd47: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
14233cd4c: 89 34 b8                    	movl	%esi, (%rax,%rdi,4)
14233cd4f: 48 8d 04 bf                 	leaq	(%rdi,%rdi,4), %rax
14233cd53: 48 8b 4c 24 50              	movq	0x50(%rsp), %rcx
14233cd58: f3 0f 59 44 81 10           	mulss	0x10(%rcx,%rax,4), %xmm0
14233cd5e: f3 0f 11 44 81 10           	movss	%xmm0, 0x10(%rcx,%rax,4)
14233cd64: f3 43 0f 10 44 37 08        	movss	0x8(%r15,%r14), %xmm0
14233cd6b: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
14233cd6e: 48 8b 4c 24 38              	movq	0x38(%rsp), %rcx
14233cd73: 75 3f                       	jne	0x14233cdb4 <.text+0x233bdb4>
14233cd75: 7a 3d                       	jp	0x14233cdb4 <.text+0x233bdb4>
14233cd77: 48 8b 01                    	movq	(%rcx), %rax
14233cd7a: 48 8b 79 10                 	movq	0x10(%rcx), %rdi
14233cd7e: 48 29 f8                    	subq	%rdi, %rax
14233cd81: 48 83 f8 17                 	cmpq	$0x17, %rax
14233cd85: 76 4a                       	jbe	0x14233cdd1 <.text+0x233bdd1>
14233cd87: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14233cd8b: 48 8d 15 fe 1f ca fe        	leaq	-0x135e002(%rip), %rdx  # 0x140fded90 <.text+0xfddd90>
14233cd92: 48 89 14 38                 	movq	%rdx, (%rax,%rdi)
14233cd96: 48 8d 15 73 2d ca fe        	leaq	-0x135d28d(%rip), %rdx  # 0x140fdfb10 <.text+0xfdeb10>
14233cd9d: 48 89 54 38 08              	movq	%rdx, 0x8(%rax,%rdi)
14233cda2: 48 8b 54 24 78              	movq	0x78(%rsp), %rdx
14233cda7: 48 89 54 38 10              	movq	%rdx, 0x10(%rax,%rdi)
14233cdac: 48 83 c7 18                 	addq	$0x18, %rdi
14233cdb0: 48 89 79 10                 	movq	%rdi, 0x10(%rcx)
14233cdb4: 48 ff c3                    	incq	%rbx
14233cdb7: 80 7c 24 40 00              	cmpb	$0x0, 0x40(%rsp)
14233cdbc: 4c 8b 4c 24 70              	movq	0x70(%rsp), %r9
14233cdc1: 48 8b 54 24 68              	movq	0x68(%rsp), %rdx
14233cdc6: 0f 84 3e fe ff ff           	je	0x14233cc0a <.text+0x233bc0a>
14233cdcc: e9 89 fd ff ff              	jmp	0x14233cb5a <.text+0x233bb5a>
14233cdd1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14233cdda: 41 b8 18 00 00 00           	movl	$0x18, %r8d
14233cde0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14233cde6: 48 89 fa                    	movq	%rdi, %rdx
14233cde9: e8 e2 8a 56 00              	callq	0x1428a58d0 <.text+0x28a48d0>
14233cdee: 48 8b 4c 24 38              	movq	0x38(%rsp), %rcx
14233cdf3: eb 92                       	jmp	0x14233cd87 <.text+0x233bd87>
14233cdf5: cc                          	int3
14233cdf6: cc                          	int3
14233cdf7: cc                          	int3
14233cdf8: cc                          	int3
14233cdf9: cc                          	int3
14233cdfa: cc                          	int3
14233cdfb: cc                          	int3
14233cdfc: cc                          	int3
14233cdfd: cc                          	int3
14233cdfe: cc                          	int3
14233cdff: cc                          	int3
