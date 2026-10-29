
../B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1422d1af0: f3 0f 58 f0                 	addss	%xmm0, %xmm6
1422d1af4: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1422d1afb: 0f b6 41 30                 	movzbl	0x30(%rcx), %eax
1422d1aff: 04 fd                       	addb	$-0x3, %al
1422d1b01: 3c 04                       	cmpb	$0x4, %al
1422d1b03: 0f 93 c3                    	setae	%bl
1422d1b06: 3c 01                       	cmpb	$0x1, %al
1422d1b08: 41 0f 94 c6                 	sete	%r14b
1422d1b0c: 48 8b 41 38                 	movq	0x38(%rcx), %rax
1422d1b10: 44 8a 68 44                 	movb	0x44(%rax), %r13b
1422d1b14: f3 44 0f 10 71 60           	movss	0x60(%rcx), %xmm14
1422d1b1a: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1422d1b21: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
1422d1b28: e8 73 2b a4 fe              	callq	0x140d146a0 <.text+0xd136a0>
1422d1b2d: 4c 8b a5 98 00 00 00        	movq	0x98(%rbp), %r12
1422d1b34: 4d 85 e4                    	testq	%r12, %r12
1422d1b37: 75 07                       	jne	0x1422d1b40 <.text+0x22d0b40>
1422d1b39: 4c 8b a5 a0 00 00 00        	movq	0xa0(%rbp), %r12
1422d1b40: 48 8b bd b0 00 00 00        	movq	0xb0(%rbp), %rdi
1422d1b47: 49 8b 04 24                 	movq	(%r12), %rax
1422d1b4b: 4d 8b 7c 24 10              	movq	0x10(%r12), %r15
1422d1b50: 4c 29 f8                    	subq	%r15, %rax
1422d1b53: 48 83 f8 37                 	cmpq	$0x37, %rax
1422d1b57: 0f 86 60 08 00 00           	jbe	0x1422d23bd <.text+0x22d13bd>
1422d1b5d: 44 08 f3                    	orb	%r14b, %bl
1422d1b60: 49 8b 44 24 08              	movq	0x8(%r12), %rax
1422d1b65: 48 8d 0d d4 98 ff ff        	leaq	-0x672c(%rip), %rcx     # 0x1422cb440 <.text+0x22ca440>
1422d1b6c: 4a 89 0c 38                 	movq	%rcx, (%rax,%r15)
1422d1b70: 48 8d 0d a9 01 34 00        	leaq	0x3401a9(%rip), %rcx    # 0x142611d20 <.text+0x2610d20>
1422d1b77: 4a 89 4c 38 08              	movq	%rcx, 0x8(%rax,%r15)
1422d1b7c: 4a 89 7c 38 10              	movq	%rdi, 0x10(%rax,%r15)
1422d1b81: 4a 89 74 38 18              	movq	%rsi, 0x18(%rax,%r15)
1422d1b86: f3 42 0f 11 74 38 20        	movss	%xmm6, 0x20(%rax,%r15)
1422d1b8d: 42 c7 44 38 24 00 00 00 00  	movl	$0x0, 0x24(%rax,%r15)
1422d1b96: f3 46 0f 11 74 38 28        	movss	%xmm14, 0x28(%rax,%r15)
1422d1b9d: 46 88 6c 38 2c              	movb	%r13b, 0x2c(%rax,%r15)
1422d1ba2: 42 88 5c 38 2d              	movb	%bl, 0x2d(%rax,%r15)
1422d1ba7: 42 c6 44 38 30 00           	movb	$0x0, 0x30(%rax,%r15)
1422d1bad: 49 83 c7 38                 	addq	$0x38, %r15
1422d1bb1: 4d 89 7c 24 10              	movq	%r15, 0x10(%r12)
1422d1bb6: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1422d1bbd: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
1422d1bc4: f3 0f 6f 85 90 00 00 00     	movdqu	0x90(%rbp), %xmm0
1422d1bcc: 0f 10 8d a0 00 00 00        	movups	0xa0(%rbp), %xmm1
1422d1bd3: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
1422d1bda: 66 0f 7f 85 30 01 00 00     	movdqa	%xmm0, 0x130(%rbp)
1422d1be2: 48 8b b5 c8 01 00 00        	movq	0x1c8(%rbp), %rsi
1422d1be9: 0f b6 46 30                 	movzbl	0x30(%rsi), %eax
1422d1bed: 3c 03                       	cmpb	$0x3, %al
1422d1bef: 0f 92 c1                    	setb	%cl
1422d1bf2: 24 06                       	andb	$0x6, %al
1422d1bf4: 3c 04                       	cmpb	$0x4, %al
1422d1bf6: 0f 94 c0                    	sete	%al
1422d1bf9: 08 c8                       	orb	%cl, %al
1422d1bfb: 0f 85 42 01 00 00           	jne	0x1422d1d43 <.text+0x22d0d43>
1422d1c01: 48 8b 4e 38                 	movq	0x38(%rsi), %rcx
1422d1c05: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
1422d1c09: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
1422d1c0d: 0f 29 8d 80 01 00 00        	movaps	%xmm1, 0x180(%rbp)
1422d1c14: 66 0f 7f 85 70 01 00 00     	movdqa	%xmm0, 0x170(%rbp)
1422d1c1c: 48 8b 41 20                 	movq	0x20(%rcx), %rax
1422d1c20: 48 8b 49 28                 	movq	0x28(%rcx), %rcx
1422d1c24: 48 8b 16                    	movq	(%rsi), %rdx
1422d1c27: 4c 8d 42 fd                 	leaq	-0x3(%rdx), %r8
1422d1c2b: 4c 8d 4a fe                 	leaq	-0x2(%rdx), %r9
1422d1c2f: 49 83 f8 03                 	cmpq	$0x3, %r8
1422d1c33: 41 b8 00 00 00 00           	movl	$0x0, %r8d
1422d1c39: 4d 0f 43 c8                 	cmovaeq	%r8, %r9
1422d1c3d: 4c 8d 15 74 10 d7 00        	leaq	0xd71074(%rip), %r10    # 0x143042cb8
1422d1c44: 4f 63 04 8a                 	movslq	(%r10,%r9,4), %r8
1422d1c48: 4d 01 d0                    	addq	%r10, %r8
1422d1c4b: 4c 8b 9d c0 01 00 00        	movq	0x1c0(%rbp), %r11
1422d1c52: 41 ff e0                    	jmpq	*%r8
1422d1c55: 44 8b 46 10                 	movl	0x10(%rsi), %r8d
1422d1c59: 4c 8b 5e 08                 	movq	0x8(%rsi), %r11
1422d1c5d: 49 b9 00 00 00 00 ff ff ff ff       	movabsq	$-0x100000000, %r9 # imm = 0xFFFFFFFF00000000
1422d1c67: 4c 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %r10
1422d1c6e: 4d 21 ca                    	andq	%r9, %r10
1422d1c71: 4d 09 c2                    	orq	%r8, %r10
1422d1c74: eb 4a                       	jmp	0x1422d1cc0 <.text+0x22d0cc0>
1422d1c76: 4c 8b 5e 08                 	movq	0x8(%rsi), %r11
1422d1c7a: 4c 8b 56 10                 	movq	0x10(%rsi), %r10
1422d1c7e: f0                          	lock
1422d1c7f: 49 ff 03                    	incq	(%r11)
1422d1c82: 0f 8e de 09 00 00           	jle	0x1422d2666 <.text+0x22d1666>
1422d1c88: ba 04 00 00 00              	movl	$0x4, %edx
1422d1c8d: eb 31                       	jmp	0x1422d1cc0 <.text+0x22d0cc0>
1422d1c8f: ba 05 00 00 00              	movl	$0x5, %edx
1422d1c94: 4c 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %r10
1422d1c9b: eb 23                       	jmp	0x1422d1cc0 <.text+0x22d0cc0>
1422d1c9d: 4c 8b 5e 08                 	movq	0x8(%rsi), %r11
1422d1ca1: 8b 56 10                    	movl	0x10(%rsi), %edx
1422d1ca4: 49 b8 00 00 00 00 ff ff ff ff       	movabsq	$-0x100000000, %r8 # imm = 0xFFFFFFFF00000000
1422d1cae: 4c 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %r10
1422d1cb5: 4d 21 c2                    	andq	%r8, %r10
1422d1cb8: 49 09 d2                    	orq	%rdx, %r10
1422d1cbb: ba 03 00 00 00              	movl	$0x3, %edx
1422d1cc0: 4c 8b 46 28                 	movq	0x28(%rsi), %r8
1422d1cc4: 4c 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %r9
1422d1ccb: 4d 89 41 e8                 	movq	%r8, -0x18(%r9)
1422d1ccf: 0f 10 46 18                 	movups	0x18(%rsi), %xmm0
1422d1cd3: 41 0f 11 41 d8              	movups	%xmm0, -0x28(%r9)
1422d1cd8: 66 0f 6f 85 70 01 00 00     	movdqa	0x170(%rbp), %xmm0
1422d1ce0: 0f 28 8d 80 01 00 00        	movaps	0x180(%rbp), %xmm1
1422d1ce7: 41 0f 11 49 10              	movups	%xmm1, 0x10(%r9)
1422d1cec: f3 41 0f 7f 01              	movdqu	%xmm0, (%r9)
1422d1cf1: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1422d1cf8: 48 89 8d c8 00 00 00        	movq	%rcx, 0xc8(%rbp)
1422d1cff: 48 89 95 90 00 00 00        	movq	%rdx, 0x90(%rbp)
1422d1d06: 4c 89 9d c0 01 00 00        	movq	%r11, 0x1c0(%rbp)
1422d1d0d: 4c 89 9d 98 00 00 00        	movq	%r11, 0x98(%rbp)
1422d1d14: 4c 89 95 b8 01 00 00        	movq	%r10, 0x1b8(%rbp)
1422d1d1b: 4c 89 95 a0 00 00 00        	movq	%r10, 0xa0(%rbp)
1422d1d22: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
1422d1d29: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
1422d1d30: 4c 8d 05 c1 0e d7 00        	leaq	0xd70ec1(%rip), %r8     # 0x143042bf8
1422d1d37: e8 e4 9c ff ff              	callq	0x1422cba20 <.text+0x22caa20>
1422d1d3c: 48 8b b5 c8 01 00 00        	movq	0x1c8(%rbp), %rsi
1422d1d43: 4c 8b 66 38                 	movq	0x38(%rsi), %r12
1422d1d47: 41 80 7c 24 45 09           	cmpb	$0x9, 0x45(%r12)
1422d1d4d: 0f 85 ed 00 00 00           	jne	0x1422d1e40 <.text+0x22d0e40>
1422d1d53: 0f b6 46 30                 	movzbl	0x30(%rsi), %eax
1422d1d57: 8d 50 fd                    	leal	-0x3(%rax), %edx
1422d1d5a: 80 fa 04                    	cmpb	$0x4, %dl
1422d1d5d: 0f 93 c1                    	setae	%cl
1422d1d60: 80 fa 01                    	cmpb	$0x1, %dl
1422d1d63: 0f 94 c2                    	sete	%dl
1422d1d66: 3c 02                       	cmpb	$0x2, %al
1422d1d68: 0f 84 f2 02 00 00           	je	0x1422d2060 <.text+0x22d1060>
1422d1d6e: 08 d1                       	orb	%dl, %cl
1422d1d70: 0f 84 ea 02 00 00           	je	0x1422d2060 <.text+0x22d1060>
1422d1d76: 49 8b 7c 24 38              	movq	0x38(%r12), %rdi
1422d1d7b: 8b 46 50                    	movl	0x50(%rsi), %eax
1422d1d7e: 89 85 98 00 00 00           	movl	%eax, 0x98(%rbp)
1422d1d84: 48 8b 46 48                 	movq	0x48(%rsi), %rax
1422d1d88: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
1422d1d8f: e8 ec 9d d6 fd              	callq	0x14003bb80 <.text+0x3ab80>
1422d1d94: 8b 85 98 00 00 00           	movl	0x98(%rbp), %eax
1422d1d9a: 89 85 78 01 00 00           	movl	%eax, 0x178(%rbp)
1422d1da0: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1422d1da7: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1422d1dae: 4c 8b a5 38 01 00 00        	movq	0x138(%rbp), %r12
1422d1db5: 4d 85 e4                    	testq	%r12, %r12
1422d1db8: 75 07                       	jne	0x1422d1dc1 <.text+0x22d0dc1>
1422d1dba: 4c 8b a5 40 01 00 00        	movq	0x140(%rbp), %r12
1422d1dc1: 48 8b 9d 50 01 00 00        	movq	0x150(%rbp), %rbx
1422d1dc8: 49 8b 04 24                 	movq	(%r12), %rax
1422d1dcc: 4d 8b 7c 24 10              	movq	0x10(%r12), %r15
1422d1dd1: 4c 29 f8                    	subq	%r15, %rax
1422d1dd4: 48 3d 87 00 00 00           	cmpq	$0x87, %rax
1422d1dda: 0f 86 71 06 00 00           	jbe	0x1422d2451 <.text+0x22d1451>
1422d1de0: 49 8b 44 24 08              	movq	0x8(%r12), %rax
1422d1de5: 48 8d 0d 64 93 ff ff        	leaq	-0x6c9c(%rip), %rcx     # 0x1422cb150 <.text+0x22ca150>
1422d1dec: 4a 89 0c 38                 	movq	%rcx, (%rax,%r15)
1422d1df0: 46 0f 11 6c 38 08           	movups	%xmm13, 0x8(%rax,%r15)
1422d1df6: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1422d1dfd: 4a 89 4c 38 18              	movq	%rcx, 0x18(%rax,%r15)
