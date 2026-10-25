
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408b2b30: 55                          	pushq	%rbp
1408b2b31: 41 57                       	pushq	%r15
1408b2b33: 41 56                       	pushq	%r14
1408b2b35: 41 55                       	pushq	%r13
1408b2b37: 41 54                       	pushq	%r12
1408b2b39: 56                          	pushq	%rsi
1408b2b3a: 57                          	pushq	%rdi
1408b2b3b: 53                          	pushq	%rbx
1408b2b3c: 48 81 ec 68 01 00 00        	subq	$0x168, %rsp            # imm = 0x168
1408b2b43: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408b2b4b: 44 0f 29 9d d0 00 00 00     	movaps	%xmm11, 0xd0(%rbp)
1408b2b53: 44 0f 29 95 c0 00 00 00     	movaps	%xmm10, 0xc0(%rbp)
1408b2b5b: 44 0f 29 8d b0 00 00 00     	movaps	%xmm9, 0xb0(%rbp)
1408b2b63: 44 0f 29 85 a0 00 00 00     	movaps	%xmm8, 0xa0(%rbp)
1408b2b6b: 0f 29 bd 90 00 00 00        	movaps	%xmm7, 0x90(%rbp)
1408b2b72: 0f 29 b5 80 00 00 00        	movaps	%xmm6, 0x80(%rbp)
1408b2b79: 48 c7 45 78 fe ff ff ff     	movq	$-0x2, 0x78(%rbp)
1408b2b81: 4d 89 cf                    	movq	%r9, %r15
1408b2b84: 4c 89 c3                    	movq	%r8, %rbx
1408b2b87: 48 89 d7                    	movq	%rdx, %rdi
1408b2b8a: 49 89 ce                    	movq	%rcx, %r14
1408b2b8d: e8 6e 3e 36 00              	callq	0x140c16a00 <.text+0xc15a00>
1408b2b92: 41 89 c1                    	movl	%eax, %r9d
1408b2b95: 84 c0                       	testb	%al, %al
1408b2b97: 74 61                       	je	0x1408b2bfa <.text+0x8b1bfa>
1408b2b99: 44 88 4d 60                 	movb	%r9b, 0x60(%rbp)
1408b2b9d: 41 bc 3e 00 00 00           	movl	$0x3e, %r12d
1408b2ba3: b9 3e 00 00 00              	movl	$0x3e, %ecx
1408b2ba8: e8 83 f7 ff ff              	callq	0x1408b2330 <.text+0x8b1330>
1408b2bad: 48 8d 35 fc c2 1e 02        	leaq	0x21ec2fc(%rip), %rsi   # 0x142a9eeb0
1408b2bb4: 48 83 f8 01                 	cmpq	$0x1, %rax
1408b2bb8: 0f 85 a8 00 00 00           	jne	0x1408b2c66 <.text+0x8b1c66>
1408b2bbe: 49 89 d1                    	movq	%rdx, %r9
1408b2bc1: 48 85 d2                    	testq	%rdx, %rdx
1408b2bc4: 74 3d                       	je	0x1408b2c03 <.text+0x8b1c03>
1408b2bc6: 49 83 f9 3e                 	cmpq	$0x3e, %r9
1408b2bca: 73 35                       	jae	0x1408b2c01 <.text+0x8b1c01>
1408b2bcc: 48 8d 05 dd c2 1e 02        	leaq	0x21ec2dd(%rip), %rax   # 0x142a9eeb0
1408b2bd3: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1408b2bd8: 7f 29                       	jg	0x1408b2c03 <.text+0x8b1c03>
1408b2bda: 48 8d 05 97 c3 1e 02        	leaq	0x21ec397(%rip), %rax   # 0x142a9ef78
1408b2be1: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408b2be6: 48 8d 0d c3 c2 1e 02        	leaq	0x21ec2c3(%rip), %rcx   # 0x142a9eeb0
1408b2bed: ba 3e 00 00 00              	movl	$0x3e, %edx
1408b2bf2: 45 31 c0                    	xorl	%r8d, %r8d
1408b2bf5: e8 46 6d 02 02              	callq	0x1428d9940 <.text+0x28d8940>
1408b2bfa: 31 c0                       	xorl	%eax, %eax
1408b2bfc: e9 d6 01 00 00              	jmp	0x1408b2dd7 <.text+0x8b1dd7>
1408b2c01: 75 d7                       	jne	0x1408b2bda <.text+0x8b1bda>
1408b2c03: 4c 89 c9                    	movq	%r9, %rcx
1408b2c06: e8 25 f7 ff ff              	callq	0x1408b2330 <.text+0x8b1330>
1408b2c0b: 48 83 f8 01                 	cmpq	$0x1, %rax
1408b2c0f: 75 55                       	jne	0x1408b2c66 <.text+0x8b1c66>
1408b2c11: 49 89 d0                    	movq	%rdx, %r8
1408b2c14: 49 83 c0 02                 	addq	$0x2, %r8
1408b2c18: 74 39                       	je	0x1408b2c53 <.text+0x8b1c53>
1408b2c1a: 49 83 f8 3e                 	cmpq	$0x3e, %r8
1408b2c1e: 73 31                       	jae	0x1408b2c51 <.text+0x8b1c51>
1408b2c20: 48 8d 05 89 c2 1e 02        	leaq	0x21ec289(%rip), %rax   # 0x142a9eeb0
1408b2c27: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1408b2c2c: 7f 25                       	jg	0x1408b2c53 <.text+0x8b1c53>
1408b2c2e: 48 8d 05 5b c3 1e 02        	leaq	0x21ec35b(%rip), %rax   # 0x142a9ef90
1408b2c35: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408b2c3a: 48 8d 0d 6f c2 1e 02        	leaq	0x21ec26f(%rip), %rcx   # 0x142a9eeb0
1408b2c41: ba 3e 00 00 00              	movl	$0x3e, %edx
1408b2c46: 41 b9 3e 00 00 00           	movl	$0x3e, %r9d
1408b2c4c: e8 ef 6c 02 02              	callq	0x1428d9940 <.text+0x28d8940>
1408b2c51: 75 db                       	jne	0x1408b2c2e <.text+0x8b1c2e>
1408b2c53: 41 bc 3c 00 00 00           	movl	$0x3c, %r12d
1408b2c59: 49 29 d4                    	subq	%rdx, %r12
1408b2c5c: 48 8d 35 4d c2 1e 02        	leaq	0x21ec24d(%rip), %rsi   # 0x142a9eeb0
1408b2c63: 4c 01 c6                    	addq	%r8, %rsi
1408b2c66: 4c 8d 05 d3 c4 1e 02        	leaq	0x21ec4d3(%rip), %r8    # 0x142a9f140
1408b2c6d: 4c 8d 2d 94 c4 1e 02        	leaq	0x21ec494(%rip), %r13   # 0x142a9f108
1408b2c74: 48 89 7d 40                 	movq	%rdi, 0x40(%rbp)
1408b2c78: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1408b2c80: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1408b2c85: 85 c0                       	testl	%eax, %eax
1408b2c87: 78 17                       	js	0x1408b2ca0 <.text+0x8b1ca0>
1408b2c89: 49 ff c8                    	decq	%r8
1408b2c8c: 83 f8 5c                    	cmpl	$0x5c, %eax
1408b2c8f: 75 63                       	jne	0x1408b2cf4 <.text+0x8b1cf4>
1408b2c91: eb 78                       	jmp	0x1408b2d0b <.text+0x8b1d0b>
1408b2c93: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1408b2ca0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1408b2ca5: 80 f9 c0                    	cmpb	$-0x40, %cl
1408b2ca8: 7d 23                       	jge	0x1408b2ccd <.text+0x8b1ccd>
1408b2caa: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1408b2caf: 80 fa c0                    	cmpb	$-0x40, %dl
1408b2cb2: 7d 22                       	jge	0x1408b2cd6 <.text+0x8b1cd6>
1408b2cb4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1408b2cb9: 49 83 c0 fc                 	addq	$-0x4, %r8
1408b2cbd: 41 83 e1 07                 	andl	$0x7, %r9d
1408b2cc1: 41 c1 e1 06                 	shll	$0x6, %r9d
1408b2cc5: 83 e2 3f                    	andl	$0x3f, %edx
1408b2cc8: 44 09 ca                    	orl	%r9d, %edx
1408b2ccb: eb 10                       	jmp	0x1408b2cdd <.text+0x8b1cdd>
1408b2ccd: 49 83 c0 fe                 	addq	$-0x2, %r8
1408b2cd1: 83 e1 1f                    	andl	$0x1f, %ecx
1408b2cd4: eb 0f                       	jmp	0x1408b2ce5 <.text+0x8b1ce5>
1408b2cd6: 49 83 c0 fd                 	addq	$-0x3, %r8
1408b2cda: 83 e2 0f                    	andl	$0xf, %edx
1408b2cdd: c1 e2 06                    	shll	$0x6, %edx
1408b2ce0: 83 e1 3f                    	andl	$0x3f, %ecx
1408b2ce3: 09 d1                       	orl	%edx, %ecx
1408b2ce5: c1 e1 06                    	shll	$0x6, %ecx
1408b2ce8: 24 3f                       	andb	$0x3f, %al
1408b2cea: 0f b6 c0                    	movzbl	%al, %eax
1408b2ced: 09 c8                       	orl	%ecx, %eax
1408b2cef: 83 f8 5c                    	cmpl	$0x5c, %eax
1408b2cf2: 74 17                       	je	0x1408b2d0b <.text+0x8b1d0b>
1408b2cf4: 83 f8 2f                    	cmpl	$0x2f, %eax
1408b2cf7: 74 12                       	je	0x1408b2d0b <.text+0x8b1d0b>
1408b2cf9: 4d 39 e8                    	cmpq	%r13, %r8
1408b2cfc: 75 82                       	jne	0x1408b2c80 <.text+0x8b1c80>
1408b2cfe: 4c 89 ff                    	movq	%r15, %rdi
1408b2d01: 49 89 df                    	movq	%rbx, %r15
1408b2d04: bb 38 00 00 00              	movl	$0x38, %ebx
1408b2d09: eb 52                       	jmp	0x1408b2d5d <.text+0x8b1d5d>
1408b2d0b: 4c 8d 2d f6 c3 1e 02        	leaq	0x21ec3f6(%rip), %r13   # 0x142a9f108
1408b2d12: 4d 29 e8                    	subq	%r13, %r8
1408b2d15: 49 ff c0                    	incq	%r8
1408b2d18: 74 32                       	je	0x1408b2d4c <.text+0x8b1d4c>
1408b2d1a: 49 83 f8 38                 	cmpq	$0x38, %r8
1408b2d1e: 73 2a                       	jae	0x1408b2d4a <.text+0x8b1d4a>
1408b2d20: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1408b2d25: 7f 25                       	jg	0x1408b2d4c <.text+0x8b1d4c>
1408b2d27: 48 8d 05 2a c2 1e 02        	leaq	0x21ec22a(%rip), %rax   # 0x142a9ef58
1408b2d2e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408b2d33: 48 8d 0d ce c3 1e 02        	leaq	0x21ec3ce(%rip), %rcx   # 0x142a9f108
1408b2d3a: ba 38 00 00 00              	movl	$0x38, %edx
1408b2d3f: 41 b9 38 00 00 00           	movl	$0x38, %r9d
1408b2d45: e8 f6 6b 02 02              	callq	0x1428d9940 <.text+0x28d8940>
1408b2d4a: 75 db                       	jne	0x1408b2d27 <.text+0x8b1d27>
1408b2d4c: 4c 89 ff                    	movq	%r15, %rdi
1408b2d4f: 49 89 df                    	movq	%rbx, %r15
1408b2d52: bb 38 00 00 00              	movl	$0x38, %ebx
1408b2d57: 4c 29 c3                    	subq	%r8, %rbx
1408b2d5a: 4d 01 c5                    	addq	%r8, %r13
1408b2d5d: e8 ce 53 36 00              	callq	0x140c18130 <.text+0xc17130>
1408b2d62: 48 89 c1                    	movq	%rax, %rcx
1408b2d65: 48 8b 00                    	movq	(%rax), %rax
1408b2d68: 48 83 f8 01                 	cmpq	$0x1, %rax
1408b2d6c: 0f 85 80 05 00 00           	jne	0x1408b32f2 <.text+0x8b22f2>
1408b2d72: 48 83 c1 08                 	addq	$0x8, %rcx
1408b2d76: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1408b2d7a: 0f 85 b0 05 00 00           	jne	0x1408b3330 <.text+0x8b2330>
1408b2d80: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1408b2d87: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
1408b2d8b: 48 83 c1 08                 	addq	$0x8, %rcx
1408b2d8f: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1408b2d94: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1408b2d9d: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1408b2da6: 48 89 f2                    	movq	%rsi, %rdx
1408b2da9: 4d 89 e0                    	movq	%r12, %r8
1408b2dac: 4d 89 e9                    	movq	%r13, %r9
1408b2daf: e8 8c 3f 36 00              	callq	0x140c16d40 <.text+0xc15d40>
1408b2db4: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
1408b2db8: 48 ff 01                    	incq	(%rcx)
1408b2dbb: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1408b2dbf: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
1408b2dc3: b8 01 00 00 00              	movl	$0x1, %eax
1408b2dc8: 44 0f b6 4d 60              	movzbl	0x60(%rbp), %r9d
1408b2dcd: 4c 89 fb                    	movq	%r15, %rbx
1408b2dd0: 49 89 ff                    	movq	%rdi, %r15
1408b2dd3: 48 8b 7d 40                 	movq	0x40(%rbp), %rdi
1408b2dd7: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
1408b2dde: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1408b2de2: 48 8b 01                    	movq	(%rcx), %rax
1408b2de5: 80 b8 f8 00 00 00 00        	cmpb	$0x0, 0xf8(%rax)
1408b2dec: 74 19                       	je	0x1408b2e07 <.text+0x8b1e07>
1408b2dee: 48 8b 07                    	movq	(%rdi), %rax
1408b2df1: 8b 88 10 01 00 00           	movl	0x110(%rax), %ecx
1408b2df7: 48 8d 15 b2 c5 1e 02        	leaq	0x21ec5b2(%rip), %rdx   # 0x142a9f3b0
1408b2dfe: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1408b2e02: 48 01 d1                    	addq	%rdx, %rcx
1408b2e05: ff e1                       	jmpq	*%rcx
1408b2e07: 45 84 c9                    	testb	%r9b, %r9b
1408b2e0a: 74 42                       	je	0x1408b2e4e <.text+0x8b1e4e>
1408b2e0c: e8 1f 53 36 00              	callq	0x140c18130 <.text+0xc17130>
1408b2e11: 48 89 c1                    	movq	%rax, %rcx
1408b2e14: 48 8b 00                    	movq	(%rax), %rax
1408b2e17: 48 83 f8 01                 	cmpq	$0x1, %rax
1408b2e1b: 0f 85 eb 04 00 00           	jne	0x1408b330c <.text+0x8b230c>
1408b2e21: 48 83 c1 08                 	addq	$0x8, %rcx
1408b2e25: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1408b2e29: 0f 85 01 05 00 00           	jne	0x1408b3330 <.text+0x8b2330>
1408b2e2f: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1408b2e36: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
1408b2e3a: 48 83 c1 08                 	addq	$0x8, %rcx
1408b2e3e: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
1408b2e42: e8 a9 42 36 00              	callq	0x140c170f0 <.text+0xc160f0>
1408b2e47: 48 8b 45 60                 	movq	0x60(%rbp), %rax
1408b2e4b: 48 ff 00                    	incq	(%rax)
1408b2e4e: 0f 28 b5 80 00 00 00        	movaps	0x80(%rbp), %xmm6
1408b2e55: 0f 28 bd 90 00 00 00        	movaps	0x90(%rbp), %xmm7
1408b2e5c: 44 0f 28 85 a0 00 00 00     	movaps	0xa0(%rbp), %xmm8
1408b2e64: 44 0f 28 8d b0 00 00 00     	movaps	0xb0(%rbp), %xmm9
1408b2e6c: 44 0f 28 95 c0 00 00 00     	movaps	0xc0(%rbp), %xmm10
1408b2e74: 44 0f 28 9d d0 00 00 00     	movaps	0xd0(%rbp), %xmm11
1408b2e7c: 48 81 c4 68 01 00 00        	addq	$0x168, %rsp            # imm = 0x168
1408b2e83: 5b                          	popq	%rbx
1408b2e84: 5f                          	popq	%rdi
1408b2e85: 5e                          	popq	%rsi
1408b2e86: 41 5c                       	popq	%r12
1408b2e88: 41 5d                       	popq	%r13
1408b2e8a: 41 5e                       	popq	%r14
1408b2e8c: 41 5f                       	popq	%r15
1408b2e8e: 5d                          	popq	%rbp
1408b2e8f: c3                          	retq
1408b2e90: 0f 57 c0                    	xorps	%xmm0, %xmm0
1408b2e93: 0f 2e 80 14 01 00 00        	ucomiss	0x114(%rax), %xmm0
1408b2e9a: 0f 82 67 ff ff ff           	jb	0x1408b2e07 <.text+0x8b1e07>
1408b2ea0: 49 8b 0e                    	movq	(%r14), %rcx
1408b2ea3: 4c 8b a9 f8 00 00 00        	movq	0xf8(%rcx), %r13
1408b2eaa: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
1408b2eb1: 4d 8b 5f 08                 	movq	0x8(%r15), %r11
1408b2eb5: 4d 85 db                    	testq	%r11, %r11
1408b2eb8: 75 04                       	jne	0x1408b2ebe <.text+0x8b1ebe>
1408b2eba: 4d 8b 5f 10                 	movq	0x10(%r15), %r11
1408b2ebe: 4d 8b 7e 08                 	movq	0x8(%r14), %r15
1408b2ec2: 4c 8d 24 85 00 00 00 00     	leaq	(,%rax,4), %r12
1408b2eca: 4d 01 ec                    	addq	%r13, %r12
1408b2ecd: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
1408b2ed1: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
1408b2ed8: 88 45 40                    	movb	%al, 0x40(%rbp)
1408b2edb: 4c 8b 33                    	movq	(%rbx), %r14
1408b2ede: b8 08 00 00 00              	movl	$0x8, %eax
1408b2ee3: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1408b2ee7: 0f 57 f6                    	xorps	%xmm6, %xmm6
1408b2eea: 0f 28 3d ef 29 07 02        	movaps	0x20729ef(%rip), %xmm7  # 0x1429258e0
1408b2ef1: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
1408b2ef5: 31 db                       	xorl	%ebx, %ebx
1408b2ef7: 31 f6                       	xorl	%esi, %esi
1408b2ef9: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
1408b2f01: c7 45 74 00 00 00 00        	movl	$0x0, 0x74(%rbp)
1408b2f08: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1408b2f10: 4c 89 75 20                 	movq	%r14, 0x20(%rbp)
1408b2f14: 44 88 4d 60                 	movb	%r9b, 0x60(%rbp)
1408b2f18: 80 7d 40 00                 	cmpb	$0x0, 0x40(%rbp)
1408b2f1c: 0f 85 3e 01 00 00           	jne	0x1408b3060 <.text+0x8b2060>
1408b2f22: 48 39 de                    	cmpq	%rbx, %rsi
1408b2f25: 0f 85 0e 01 00 00           	jne	0x1408b3039 <.text+0x8b2039>
1408b2f2b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1408b2f30: 4d 39 e5                    	cmpq	%r12, %r13
1408b2f33: 0f 84 ce fe ff ff           	je	0x1408b2e07 <.text+0x8b1e07>
1408b2f39: 41 8b 4d 00                 	movl	(%r13), %ecx
1408b2f3d: 49 83 c5 04                 	addq	$0x4, %r13
1408b2f41: 49 8b 87 00 01 00 00        	movq	0x100(%r15), %rax
1408b2f48: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1408b2f4c: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408b2f50: 48 8b 5c 08 58              	movq	0x58(%rax,%rcx), %rbx
1408b2f55: 48 85 db                    	testq	%rbx, %rbx
1408b2f58: 74 d6                       	je	0x1408b2f30 <.text+0x8b1f30>
1408b2f5a: 48 01 c8                    	addq	%rcx, %rax
1408b2f5d: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
1408b2f63: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1408b2f67: 48 c1 e1 03                 	shlq	$0x3, %rcx
1408b2f6b: 49 03 8f a8 01 00 00        	addq	0x1a8(%r15), %rcx
1408b2f72: 4c 8b 55 38                 	movq	0x38(%rbp), %r10
1408b2f76: 49 8b 92 10 01 00 00        	movq	0x110(%r10), %rdx
1408b2f7d: 48 39 90 88 00 00 00        	cmpq	%rdx, 0x88(%rax)
1408b2f84: 76 4f                       	jbe	0x1408b2fd5 <.text+0x8b1fd5>
1408b2f86: 4c 8b 80 80 00 00 00        	movq	0x80(%rax), %r8
1408b2f8d: 49 83 3c d0 00              	cmpq	$0x0, (%r8,%rdx,8)
1408b2f92: 74 41                       	je	0x1408b2fd5 <.text+0x8b1fd5>
1408b2f94: 41 b0 01                    	movb	$0x1, %r8b
1408b2f97: 44 89 45 74                 	movl	%r8d, 0x74(%rbp)
1408b2f9b: 48 39 51 40                 	cmpq	%rdx, 0x40(%rcx)
1408b2f9f: 76 4a                       	jbe	0x1408b2feb <.text+0x8b1feb>
1408b2fa1: 4c 8b 41 38                 	movq	0x38(%rcx), %r8
1408b2fa5: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
1408b2fa9: 48 85 d2                    	testq	%rdx, %rdx
1408b2fac: 74 3d                       	je	0x1408b2feb <.text+0x8b1feb>
1408b2fae: 4c 8b 41 18                 	movq	0x18(%rcx), %r8
1408b2fb2: 48 f7 d2                    	notq	%rdx
1408b2fb5: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1408b2fb9: 48 c1 e2 04                 	shlq	$0x4, %rdx
1408b2fbd: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
1408b2fc2: 48 89 55 58                 	movq	%rdx, 0x58(%rbp)
1408b2fc6: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
1408b2fcd: 48 39 51 40                 	cmpq	%rdx, 0x40(%rcx)
1408b2fd1: 76 54                       	jbe	0x1408b3027 <.text+0x8b2027>
1408b2fd3: eb 2b                       	jmp	0x1408b3000 <.text+0x8b2000>
1408b2fd5: c7 45 74 00 00 00 00        	movl	$0x0, 0x74(%rbp)
1408b2fdc: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
1408b2fe3: 48 39 51 40                 	cmpq	%rdx, 0x40(%rcx)
1408b2fe7: 76 3e                       	jbe	0x1408b3027 <.text+0x8b2027>
1408b2fe9: eb 15                       	jmp	0x1408b3000 <.text+0x8b2000>
1408b2feb: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
1408b2ff3: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
1408b2ffa: 48 39 51 40                 	cmpq	%rdx, 0x40(%rcx)
1408b2ffe: 76 27                       	jbe	0x1408b3027 <.text+0x8b2027>
1408b3000: 4c 8b 41 38                 	movq	0x38(%rcx), %r8
1408b3004: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
1408b3008: 48 85 d2                    	testq	%rdx, %rdx
1408b300b: 74 1a                       	je	0x1408b3027 <.text+0x8b2027>
1408b300d: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
1408b3011: 48 f7 d2                    	notq	%rdx
1408b3014: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1408b3018: 48 c1 e2 04                 	shlq	$0x4, %rdx
1408b301c: 48 8b 4c 11 10              	movq	0x10(%rcx,%rdx), %rcx
1408b3021: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
1408b3025: eb 08                       	jmp	0x1408b302f <.text+0x8b202f>
1408b3027: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1408b302f: 48 8b 40 50                 	movq	0x50(%rax), %rax
1408b3033: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
1408b3037: 31 f6                       	xorl	%esi, %esi
1408b3039: 48 89 f0                    	movq	%rsi, %rax
1408b303c: 48 c1 e0 04                 	shlq	$0x4, %rax
1408b3040: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1408b3044: 48 8d 14 01                 	leaq	(%rcx,%rax), %rdx
1408b3048: 8b 44 01 08                 	movl	0x8(%rcx,%rax), %eax
1408b304c: e9 eb 00 00 00              	jmp	0x1408b313c <.text+0x8b213c>
1408b3051: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1408b3060: 48 39 de                    	cmpq	%rbx, %rsi
1408b3063: 0f 85 c9 00 00 00           	jne	0x1408b3132 <.text+0x8b2132>
1408b3069: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1408b3070: 4d 39 e5                    	cmpq	%r12, %r13
1408b3073: 0f 84 8e fd ff ff           	je	0x1408b2e07 <.text+0x8b1e07>
1408b3079: 41 8b 4d 00                 	movl	(%r13), %ecx
1408b307d: 49 83 c5 04                 	addq	$0x4, %r13
1408b3081: 49 8b 87 a8 01 00 00        	movq	0x1a8(%r15), %rax
1408b3088: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1408b308c: 48 8b 5c c8 10              	movq	0x10(%rax,%rcx,8), %rbx
1408b3091: 48 85 db                    	testq	%rbx, %rbx
1408b3094: 74 da                       	je	0x1408b3070 <.text+0x8b2070>
1408b3096: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1408b309a: 4c 8b 55 38                 	movq	0x38(%rbp), %r10
1408b309e: 49 8b 92 10 01 00 00        	movq	0x110(%r10), %rdx
1408b30a5: 48 8b 48 40                 	movq	0x40(%rax), %rcx
1408b30a9: 48 39 d1                    	cmpq	%rdx, %rcx
1408b30ac: 76 38                       	jbe	0x1408b30e6 <.text+0x8b20e6>
1408b30ae: 4c 8b 40 38                 	movq	0x38(%rax), %r8
1408b30b2: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
1408b30b6: 48 85 d2                    	testq	%rdx, %rdx
1408b30b9: 74 2b                       	je	0x1408b30e6 <.text+0x8b20e6>
1408b30bb: 4c 8b 40 18                 	movq	0x18(%rax), %r8
1408b30bf: 48 f7 d2                    	notq	%rdx
1408b30c2: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1408b30c6: 48 c1 e2 04                 	shlq	$0x4, %rdx
1408b30ca: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
1408b30cf: 48 89 55 58                 	movq	%rdx, 0x58(%rbp)
1408b30d3: b2 01                       	movb	$0x1, %dl
1408b30d5: 89 55 74                    	movl	%edx, 0x74(%rbp)
1408b30d8: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
1408b30df: 48 39 d1                    	cmpq	%rdx, %rcx
1408b30e2: 77 15                       	ja	0x1408b30f9 <.text+0x8b20f9>
1408b30e4: eb 3a                       	jmp	0x1408b3120 <.text+0x8b2120>
1408b30e6: c7 45 74 00 00 00 00        	movl	$0x0, 0x74(%rbp)
1408b30ed: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
1408b30f4: 48 39 d1                    	cmpq	%rdx, %rcx
1408b30f7: 76 27                       	jbe	0x1408b3120 <.text+0x8b2120>
1408b30f9: 48 8b 48 38                 	movq	0x38(%rax), %rcx
1408b30fd: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
1408b3101: 48 85 c9                    	testq	%rcx, %rcx
1408b3104: 74 1a                       	je	0x1408b3120 <.text+0x8b2120>
1408b3106: 48 8b 50 18                 	movq	0x18(%rax), %rdx
1408b310a: 48 f7 d1                    	notq	%rcx
1408b310d: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408b3111: 48 c1 e1 04                 	shlq	$0x4, %rcx
1408b3115: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
1408b311a: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
1408b311e: eb 08                       	jmp	0x1408b3128 <.text+0x8b2128>
1408b3120: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1408b3128: 48 8b 40 08                 	movq	0x8(%rax), %rax
1408b312c: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1408b3130: 31 f6                       	xorl	%esi, %esi
1408b3132: 48 8b 45 28                 	movq	0x28(%rbp), %rax
1408b3136: 48 8d 14 f0                 	leaq	(%rax,%rsi,8), %rdx
1408b313a: 89 f0                       	movl	%esi, %eax
1408b313c: 48 8d 0c 40                 	leaq	(%rax,%rax,2), %rcx
1408b3140: 48 8b 45 68                 	movq	0x68(%rbp), %rax
1408b3144: 48 8d 04 88                 	leaq	(%rax,%rcx,4), %rax
1408b3148: 48 c1 e1 04                 	shlq	$0x4, %rcx
1408b314c: 48 03 4d 58                 	addq	0x58(%rbp), %rcx
1408b3150: f6 45 74 01                 	testb	$0x1, 0x74(%rbp)
1408b3154: 41 b8 00 00 00 00           	movl	$0x0, %r8d
1408b315a: 49 0f 44 c8                 	cmoveq	%r8, %rcx
1408b315e: 48 8b 3a                    	movq	(%rdx), %rdi
1408b3161: 48 89 fa                    	movq	%rdi, %rdx
1408b3164: 48 c1 ea 20                 	shrq	$0x20, %rdx
1408b3168: 0f 84 99 fc ff ff           	je	0x1408b2e07 <.text+0x8b1e07>
1408b316e: 48 ff c6                    	incq	%rsi
1408b3171: 48 85 c9                    	testq	%rcx, %rcx
1408b3174: 74 3d                       	je	0x1408b31b3 <.text+0x8b21b3>
1408b3176: 49 8b 03                    	movq	(%r11), %rax
1408b3179: 4d 8b 73 10                 	movq	0x10(%r11), %r14
1408b317d: 4c 29 f0                    	subq	%r14, %rax
1408b3180: 48 83 f8 17                 	cmpq	$0x17, %rax
1408b3184: 0f 86 0e 01 00 00           	jbe	0x1408b3298 <.text+0x8b2298>
1408b318a: 49 8b 43 08                 	movq	0x8(%r11), %rax
1408b318e: 48 8d 0d db 9c a2 00        	leaq	0xa29cdb(%rip), %rcx    # 0x1412dce70 <.text+0x12dbe70>
1408b3195: 4a 89 0c 30                 	movq	%rcx, (%rax,%r14)
1408b3199: 48 8d 0d 30 aa a2 00        	leaq	0xa2aa30(%rip), %rcx    # 0x1412ddbd0 <.text+0x12dcbd0>
1408b31a0: 4a 89 4c 30 08              	movq	%rcx, 0x8(%rax,%r14)
1408b31a5: 4a 89 7c 30 10              	movq	%rdi, 0x10(%rax,%r14)
1408b31aa: 49 83 c6 18                 	addq	$0x18, %r14
1408b31ae: e9 c9 00 00 00              	jmp	0x1408b327c <.text+0x8b227c>
1408b31b3: 41 f6 06 01                 	testb	$0x1, (%r14)
1408b31b7: 74 14                       	je	0x1408b31cd <.text+0x8b21cd>
1408b31b9: 4c 89 f1                    	movq	%r14, %rcx
1408b31bc: 4d 89 de                    	movq	%r11, %r14
1408b31bf: f2 44 0f 10 41 04           	movsd	0x4(%rcx), %xmm8
1408b31c5: f3 44 0f 10 49 0c           	movss	0xc(%rcx), %xmm9
1408b31cb: eb 0b                       	jmp	0x1408b31d8 <.text+0x8b21d8>
1408b31cd: 4d 89 de                    	movq	%r11, %r14
1408b31d0: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
1408b31d4: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
1408b31d8: f3 44 0f 10 58 04           	movss	0x4(%rax), %xmm11
1408b31de: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1408b31e2: e8 99 9f 3e 00              	callq	0x140c9d180 <.text+0xc9c180>
1408b31e7: 4d 89 f3                    	movq	%r14, %r11
1408b31ea: 45 0f 28 d3                 	movaps	%xmm11, %xmm10
1408b31ee: f3 44 0f 59 d6              	mulss	%xmm6, %xmm10
1408b31f3: 45 0f 14 d3                 	unpcklps	%xmm11, %xmm10          # xmm10 = xmm10[0],xmm11[0],xmm10[1],xmm11[1]
1408b31f7: 44 0f 59 d7                 	mulps	%xmm7, %xmm10
1408b31fb: 44 0f 28 5d c0              	movaps	-0x40(%rbp), %xmm11
1408b3200: 48 8d 4d dc                 	leaq	-0x24(%rbp), %rcx
1408b3204: 48 8b 01                    	movq	(%rcx), %rax
1408b3207: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1408b320b: 8b 41 08                    	movl	0x8(%rcx), %eax
1408b320e: 89 45 18                    	movl	%eax, 0x18(%rbp)
1408b3211: 49 8b 06                    	movq	(%r14), %rax
1408b3214: 4d 8b 76 10                 	movq	0x10(%r14), %r14
1408b3218: 4c 29 f0                    	subq	%r14, %rax
1408b321b: 48 83 f8 67                 	cmpq	$0x67, %rax
1408b321f: 0f 86 a0 00 00 00           	jbe	0x1408b32c5 <.text+0x8b22c5>
1408b3225: f3 45 0f 58 ca              	addss	%xmm10, %xmm9
1408b322a: 45 0f 58 c2                 	addps	%xmm10, %xmm8
1408b322e: 49 8b 43 08                 	movq	0x8(%r11), %rax
1408b3232: 48 8d 0d 87 86 a2 00        	leaq	0xa28687(%rip), %rcx    # 0x1412db8c0 <.text+0x12da8c0>
1408b3239: 4a 89 0c 30                 	movq	%rcx, (%rax,%r14)
1408b323d: 46 0f 11 5c 30 08           	movups	%xmm11, 0x8(%rax,%r14)
1408b3243: 46 0f 13 44 30 18           	movlps	%xmm8, 0x18(%rax,%r14)
1408b3249: f3 46 0f 11 4c 30 20        	movss	%xmm9, 0x20(%rax,%r14)
1408b3250: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
1408b3254: 4a 89 4c 30 24              	movq	%rcx, 0x24(%rax,%r14)
1408b3259: 8b 4d 18                    	movl	0x18(%rbp), %ecx
1408b325c: 42 89 4c 30 2c              	movl	%ecx, 0x2c(%rax,%r14)
1408b3261: 42 c6 44 30 38 00           	movb	$0x0, 0x38(%rax,%r14)
1408b3267: 4a 89 7c 30 48              	movq	%rdi, 0x48(%rax,%r14)
1408b326c: 48 8d 0d bd 1a cb 00        	leaq	0xcb1abd(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1408b3273: 4a 89 4c 30 58              	movq	%rcx, 0x58(%rax,%r14)
1408b3278: 49 83 c6 68                 	addq	$0x68, %r14
1408b327c: 4d 89 73 10                 	movq	%r14, 0x10(%r11)
1408b3280: 44 0f b6 4d 60              	movzbl	0x60(%rbp), %r9d
1408b3285: 4c 8b 75 20                 	movq	0x20(%rbp), %r14
1408b3289: 80 7d 40 00                 	cmpb	$0x0, 0x40(%rbp)
1408b328d: 0f 84 8f fc ff ff           	je	0x1408b2f22 <.text+0x8b1f22>
1408b3293: e9 c8 fd ff ff              	jmp	0x1408b3060 <.text+0x8b2060>
1408b3298: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1408b32a1: 41 b8 18 00 00 00           	movl	$0x18, %r8d
1408b32a7: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1408b32ad: 4c 89 d9                    	movq	%r11, %rcx
1408b32b0: 4c 89 f2                    	movq	%r14, %rdx
1408b32b3: 4c 89 5d 48                 	movq	%r11, 0x48(%rbp)
1408b32b7: e8 a4 67 ff 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1408b32bc: 4c 8b 5d 48                 	movq	0x48(%rbp), %r11
1408b32c0: e9 c5 fe ff ff              	jmp	0x1408b318a <.text+0x8b218a>
1408b32c5: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1408b32ce: 41 b8 68 00 00 00           	movl	$0x68, %r8d
1408b32d4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1408b32da: 4c 89 d9                    	movq	%r11, %rcx
1408b32dd: 4c 89 f2                    	movq	%r14, %rdx
1408b32e0: 4c 89 5d 48                 	movq	%r11, 0x48(%rbp)
1408b32e4: e8 77 67 ff 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1408b32e9: 4c 8b 5d 48                 	movq	0x48(%rbp), %r11
1408b32ed: e9 33 ff ff ff              	jmp	0x1408b3225 <.text+0x8b2225>
1408b32f2: 48 85 c0                    	testq	%rax, %rax
1408b32f5: 75 2d                       	jne	0x1408b3324 <.text+0x8b2324>
1408b32f7: 31 d2                       	xorl	%edx, %edx
1408b32f9: e8 d2 54 04 02              	callq	0x1428f87d0 <.text+0x28f77d0>
1408b32fe: 48 89 c1                    	movq	%rax, %rcx
1408b3301: 48 85 c0                    	testq	%rax, %rax
1408b3304: 0f 85 6c fa ff ff           	jne	0x1408b2d76 <.text+0x8b1d76>
1408b330a: eb 18                       	jmp	0x1408b3324 <.text+0x8b2324>
1408b330c: 48 85 c0                    	testq	%rax, %rax
1408b330f: 75 13                       	jne	0x1408b3324 <.text+0x8b2324>
1408b3311: 31 d2                       	xorl	%edx, %edx
1408b3313: e8 b8 54 04 02              	callq	0x1428f87d0 <.text+0x28f77d0>
1408b3318: 48 89 c1                    	movq	%rax, %rcx
1408b331b: 48 85 c0                    	testq	%rax, %rax
1408b331e: 0f 85 01 fb ff ff           	jne	0x1408b2e25 <.text+0x8b1e25>
1408b3324: 48 8d 0d 75 77 38 02        	leaq	0x2387775(%rip), %rcx   # 0x142c3aaa0
1408b332b: e8 a0 4d 02 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1408b3330: 48 8d 0d 39 78 38 02        	leaq	0x2387839(%rip), %rcx   # 0x142c3ab70
1408b3337: e8 14 5f 02 02              	callq	0x1428d9250 <.text+0x28d8250>
1408b333c: cc                          	int3
1408b333d: 0f 1f 00                    	nopl	(%rax)
1408b3340: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408b3345: 55                          	pushq	%rbp
1408b3346: 41 57                       	pushq	%r15
1408b3348: 41 56                       	pushq	%r14
1408b334a: 41 55                       	pushq	%r13
1408b334c: 41 54                       	pushq	%r12
1408b334e: 56                          	pushq	%rsi
1408b334f: 57                          	pushq	%rdi
1408b3350: 53                          	pushq	%rbx
1408b3351: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1408b3358: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408b335f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
1408b3365: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
1408b336b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
1408b3371: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
1408b3377: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
1408b337c: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
1408b3384: 80 7d 60 00                 	cmpb	$0x0, 0x60(%rbp)
1408b3388: 74 1f                       	je	0x1408b33a9 <.text+0x8b23a9>
1408b338a: 48 8d 45 f8                 	leaq	-0x8(%rbp), %rax
1408b338e: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
1408b3392: 48 8d 45 08                 	leaq	0x8(%rbp), %rax
1408b3396: 48 89 45 00                 	movq	%rax, (%rbp)
1408b339a: 48 8d 0d 57 bb 1e 02        	leaq	0x21ebb57(%rip), %rcx   # 0x142a9eef8
1408b33a1: 48 89 ea                    	movq	%rbp, %rdx
1408b33a4: e8 27 bf a7 00              	callq	0x14132f2d0 <.text+0x132e2d0>
1408b33a9: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
1408b33b1: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
1408b33b6: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
1408b33bc: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
1408b33c2: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
1408b33c8: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
1408b33ce: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1408b33d5: 5b                          	popq	%rbx
1408b33d6: 5f                          	popq	%rdi
1408b33d7: 5e                          	popq	%rsi
1408b33d8: 41 5c                       	popq	%r12
1408b33da: 41 5d                       	popq	%r13
1408b33dc: 41 5e                       	popq	%r14
1408b33de: 41 5f                       	popq	%r15
1408b33e0: 5d                          	popq	%rbp
1408b33e1: c3                          	retq
1408b33e2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1408b33f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408b33f5: 55                          	pushq	%rbp
1408b33f6: 41 57                       	pushq	%r15
1408b33f8: 41 56                       	pushq	%r14
1408b33fa: 41 55                       	pushq	%r13
1408b33fc: 41 54                       	pushq	%r12
1408b33fe: 56                          	pushq	%rsi
1408b33ff: 57                          	pushq	%rdi
1408b3400: 53                          	pushq	%rbx
1408b3401: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1408b3408: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408b340f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
1408b3415: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
1408b341b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
1408b3421: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
1408b3427: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
1408b342c: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
1408b3434: 48 8b 45 60                 	movq	0x60(%rbp), %rax
1408b3438: 48 ff 00                    	incq	(%rax)
1408b343b: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
1408b3443: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
1408b3448: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
1408b344e: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
1408b3454: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
1408b345a: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
1408b3460: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1408b3467: 5b                          	popq	%rbx
1408b3468: 5f                          	popq	%rdi
1408b3469: 5e                          	popq	%rsi
1408b346a: 41 5c                       	popq	%r12
1408b346c: 41 5d                       	popq	%r13
1408b346e: 41 5e                       	popq	%r14
1408b3470: 41 5f                       	popq	%r15
1408b3472: 5d                          	popq	%rbp
1408b3473: c3                          	retq
1408b3474: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1408b3480: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408b3485: 55                          	pushq	%rbp
1408b3486: 41 57                       	pushq	%r15
1408b3488: 41 56                       	pushq	%r14
1408b348a: 41 55                       	pushq	%r13
1408b348c: 41 54                       	pushq	%r12
1408b348e: 56                          	pushq	%rsi
1408b348f: 57                          	pushq	%rdi
1408b3490: 53                          	pushq	%rbx
1408b3491: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1408b3498: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408b349f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
1408b34a5: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
1408b34ab: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
1408b34b1: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
1408b34b7: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
1408b34bc: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
1408b34c4: 48 8b 45 50                 	movq	0x50(%rbp), %rax
1408b34c8: 48 ff 00                    	incq	(%rax)
1408b34cb: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
1408b34d3: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
1408b34d8: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
1408b34de: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
1408b34e4: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
1408b34ea: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
1408b34f0: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1408b34f7: 5b                          	popq	%rbx
1408b34f8: 5f                          	popq	%rdi
1408b34f9: 5e                          	popq	%rsi
1408b34fa: 41 5c                       	popq	%r12
1408b34fc: 41 5d                       	popq	%r13
1408b34fe: 41 5e                       	popq	%r14
1408b3500: 41 5f                       	popq	%r15
1408b3502: 5d                          	popq	%rbp
1408b3503: c3                          	retq
