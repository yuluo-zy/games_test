
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14127bab0: 55                          	pushq	%rbp
14127bab1: 41 57                       	pushq	%r15
14127bab3: 41 56                       	pushq	%r14
14127bab5: 41 55                       	pushq	%r13
14127bab7: 41 54                       	pushq	%r12
14127bab9: 56                          	pushq	%rsi
14127baba: 57                          	pushq	%rdi
14127babb: 53                          	pushq	%rbx
14127babc: 48 81 ec 18 03 00 00        	subq	$0x318, %rsp            # imm = 0x318
14127bac3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14127bacb: 44 0f 29 9d 80 02 00 00     	movaps	%xmm11, 0x280(%rbp)
14127bad3: 44 0f 29 95 70 02 00 00     	movaps	%xmm10, 0x270(%rbp)
14127badb: 44 0f 29 8d 60 02 00 00     	movaps	%xmm9, 0x260(%rbp)
14127bae3: 44 0f 29 85 50 02 00 00     	movaps	%xmm8, 0x250(%rbp)
14127baeb: 0f 29 bd 40 02 00 00        	movaps	%xmm7, 0x240(%rbp)
14127baf2: 0f 29 b5 30 02 00 00        	movaps	%xmm6, 0x230(%rbp)
14127baf9: 48 c7 85 28 02 00 00 fe ff ff ff    	movq	$-0x2, 0x228(%rbp)
14127bb04: 4d 89 c2                    	movq	%r8, %r10
14127bb07: 48 89 95 80 01 00 00        	movq	%rdx, 0x180(%rbp)
14127bb0e: 48 89 8d 38 01 00 00        	movq	%rcx, 0x138(%rbp)
14127bb15: 8b b5 00 03 00 00           	movl	0x300(%rbp), %esi
14127bb1b: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
14127bb22: 8b 48 18                    	movl	0x18(%rax), %ecx
14127bb25: 89 8d c8 00 00 00           	movl	%ecx, 0xc8(%rbp)
14127bb2b: 48 8b 40 10                 	movq	0x10(%rax), %rax
14127bb2f: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14127bb36: 49 8b 11                    	movq	(%r9), %rdx
14127bb39: 4c 89 8d d0 00 00 00        	movq	%r9, 0xd0(%rbp)
14127bb40: 4d 8b 41 08                 	movq	0x8(%r9), %r8
14127bb44: 4c 89 95 68 01 00 00        	movq	%r10, 0x168(%rbp)
14127bb4b: 4c 89 d1                    	movq	%r10, %rcx
14127bb4e: 48 89 95 70 01 00 00        	movq	%rdx, 0x170(%rbp)
14127bb55: 4c 89 85 78 01 00 00        	movq	%r8, 0x178(%rbp)
14127bb5c: 41 89 f1                    	movl	%esi, %r9d
14127bb5f: e8 9c 0c 7a ff              	callq	0x140a1c800 <.text+0xa1b800>
14127bb64: 48 85 c0                    	testq	%rax, %rax
14127bb67: 0f 84 15 0a 00 00           	je	0x14127c582 <.text+0x127b582>
14127bb6d: 48 89 c1                    	movq	%rax, %rcx
14127bb70: 4c 8b ad 10 03 00 00        	movq	0x310(%rbp), %r13
14127bb77: 0f b6 40 46                 	movzbl	0x46(%rax), %eax
14127bb7b: 88 85 25 02 00 00           	movb	%al, 0x225(%rbp)
14127bb81: 48 8b 41 20                 	movq	0x20(%rcx), %rax
14127bb85: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14127bb89: 0f 10 01                    	movups	(%rcx), %xmm0
14127bb8c: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
14127bb90: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
14127bb94: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
14127bb98: 0f b6 41 44                 	movzbl	0x44(%rcx), %eax
14127bb9c: 88 85 26 02 00 00           	movb	%al, 0x226(%rbp)
14127bba2: 8b 41 28                    	movl	0x28(%rcx), %eax
14127bba5: 89 85 14 02 00 00           	movl	%eax, 0x214(%rbp)
14127bbab: 8b 41 40                    	movl	0x40(%rcx), %eax
14127bbae: 89 85 f0 00 00 00           	movl	%eax, 0xf0(%rbp)
14127bbb4: 0f 10 41 30                 	movups	0x30(%rcx), %xmm0
14127bbb8: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
14127bbbf: 48 8d 15 aa 82 9a 01        	leaq	0x19a82aa(%rip), %rdx   # 0x142c23e70
14127bbc6: 48 89 8d 18 02 00 00        	movq	%rcx, 0x218(%rbp)
14127bbcd: e8 de 77 5b ff              	callq	0x1408333b0 <.text+0x8323b0>
14127bbd2: 48 8b 08                    	movq	(%rax), %rcx
14127bbd5: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
14127bbdc: 8b 40 08                    	movl	0x8(%rax), %eax
14127bbdf: 89 85 b8 00 00 00           	movl	%eax, 0xb8(%rbp)
14127bbe5: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
14127bbf0: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127bbf7: 4c 89 e9                    	movq	%r13, %rcx
14127bbfa: 89 f2                       	movl	%esi, %edx
14127bbfc: e8 6f ad 8a ff              	callq	0x140b26970 <.text+0xb25970>
14127bc01: 3c 08                       	cmpb	$0x8, %al
14127bc03: 74 49                       	je	0x14127bc4e <.text+0x127ac4e>
14127bc05: 0f b6 c0                    	movzbl	%al, %eax
14127bc08: 83 f8 02                    	cmpl	$0x2, %eax
14127bc0b: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
14127bc12: 0f 85 8e 09 00 00           	jne	0x14127c5a6 <.text+0x127b5a6>
14127bc18: 0f b6 40 44                 	movzbl	0x44(%rax), %eax
14127bc1c: fe c8                       	decb	%al
14127bc1e: 3c 03                       	cmpb	$0x3, %al
14127bc20: 4c 8b 8d 80 01 00 00        	movq	0x180(%rbp), %r9
14127bc27: 44 0f b6 95 26 02 00 00     	movzbl	0x226(%rbp), %r10d
14127bc2f: 0f 83 c1 09 00 00           	jae	0x14127c5f6 <.text+0x127b5f6>
14127bc35: 0f b6 c0                    	movzbl	%al, %eax
14127bc38: 48 8d 0d 61 83 9a 01        	leaq	0x19a8361(%rip), %rcx   # 0x142c23fa0
14127bc3f: f3 0f 10 3c 81              	movss	(%rcx,%rax,4), %xmm7
14127bc44: f3 0f 10 35 b8 9c 6a 01     	movss	0x16a9cb8(%rip), %xmm6  # 0x142925904
14127bc4c: eb 7c                       	jmp	0x14127bcca <.text+0x127acca>
14127bc4e: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
14127bc55: 8a 58 44                    	movb	0x44(%rax), %bl
14127bc58: 49 8b 55 00                 	movq	(%r13), %rdx
14127bc5c: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127bc63: b1 08                       	movb	$0x8, %cl
14127bc65: e8 e6 75 5b ff              	callq	0x140833250 <.text+0x832250>
14127bc6a: 4d 8b 6d 08                 	movq	0x8(%r13), %r13
14127bc6e: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127bc75: 89 d9                       	movl	%ebx, %ecx
14127bc77: 89 c2                       	movl	%eax, %edx
14127bc79: e8 12 a0 8a ff              	callq	0x140b25c90 <.text+0xb24c90>
14127bc7e: 88 85 00 01 00 00           	movb	%al, 0x100(%rbp)
14127bc84: 88 95 01 01 00 00           	movb	%dl, 0x101(%rbp)
14127bc8a: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127bc91: 48 8d 9d 00 01 00 00        	leaq	0x100(%rbp), %rbx
14127bc98: 4c 89 e9                    	movq	%r13, %rcx
14127bc9b: 48 89 da                    	movq	%rbx, %rdx
14127bc9e: e8 ed 12 fe ff              	callq	0x14125cf90 <.text+0x125bf90>
14127bca3: 48 85 c0                    	testq	%rax, %rax
14127bca6: 0f 84 6b 09 00 00           	je	0x14127c617 <.text+0x127b617>
14127bcac: f3 0f 10 78 30              	movss	0x30(%rax), %xmm7
14127bcb1: f3 0f 10 70 3c              	movss	0x3c(%rax), %xmm6
14127bcb6: f3 0f 5c 70 38              	subss	0x38(%rax), %xmm6
14127bcbb: 4c 8b 8d 80 01 00 00        	movq	0x180(%rbp), %r9
14127bcc2: 44 0f b6 95 26 02 00 00     	movzbl	0x226(%rbp), %r10d
14127bcca: 41 0f b6 ca                 	movzbl	%r10b, %ecx
14127bcce: 4c 89 c8                    	movq	%r9, %rax
14127bcd1: 48 f7 d0                    	notq	%rax
14127bcd4: 48 01 c8                    	addq	%rcx, %rax
14127bcd7: 4c 89 8d 88 01 00 00        	movq	%r9, 0x188(%rbp)
14127bcde: c7 85 90 01 00 00 00 00 80 bf       	movl	$0xbf800000, 0x190(%rbp) # imm = 0xBF800000
14127bce8: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
14127bcef: c7 85 a0 01 00 00 00 00 80 3f       	movl	$0x3f800000, 0x1a0(%rbp) # imm = 0x3F800000
14127bcf9: 0f 57 c0                    	xorps	%xmm0, %xmm0
14127bcfc: f3 0f 10 0d 00 9c 6a 01     	movss	0x16a9c00(%rip), %xmm1  # 0x142925904
14127bd04: 80 bd 25 02 00 00 3a        	cmpb	$0x3a, 0x225(%rbp)
14127bd0b: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
14127bd12: 74 06                       	je	0x14127bd1a <.text+0x127ad1a>
14127bd14: 44 0f 28 c1                 	movaps	%xmm1, %xmm8
14127bd18: eb 07                       	jmp	0x14127bd21 <.text+0x127ad21>
14127bd1a: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
14127bd1e: 0f 28 c1                    	movaps	%xmm1, %xmm0
14127bd21: 48 8d bd 60 01 00 00        	leaq	0x160(%rbp), %rdi
14127bd28: f3 0f 58 35 b4 e3 6d 01     	addss	0x16de3b4(%rip), %xmm6  # 0x14295a0e4
14127bd30: f3 0f 59 35 38 9b 6a 01     	mulss	0x16a9b38(%rip), %xmm6  # 0x142925870
14127bd38: f3 0f 59 f0                 	mulss	%xmm0, %xmm6
14127bd3c: 31 c0                       	xorl	%eax, %eax
14127bd3e: 4c 8d 35 5b 82 9a 01        	leaq	0x19a825b(%rip), %r14   # 0x142c23fa0
14127bd45: f3 44 0f 10 0d 22 9b 6a 01  	movss	0x16a9b22(%rip), %xmm9  # 0x142925870
14127bd4e: 4c 8d ad 40 01 00 00        	leaq	0x140(%rbp), %r13
14127bd55: 41 bf 02 00 00 00           	movl	$0x2, %r15d
14127bd5b: 4c 8d a5 48 01 00 00        	leaq	0x148(%rbp), %r12
14127bd62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14127bd70: 48 83 f8 02                 	cmpq	$0x2, %rax
14127bd74: 0f 84 ee 00 00 00           	je	0x14127be68 <.text+0x127ae68>
14127bd7a: 48 8d 58 01                 	leaq	0x1(%rax), %rbx
14127bd7e: 48 89 c1                    	movq	%rax, %rcx
14127bd81: 48 c1 e1 04                 	shlq	$0x4, %rcx
14127bd85: 48 8b b4 0d 88 01 00 00     	movq	0x188(%rbp,%rcx), %rsi
14127bd8d: 48 89 d8                    	movq	%rbx, %rax
14127bd90: 48 85 f6                    	testq	%rsi, %rsi
14127bd93: 74 db                       	je	0x14127bd70 <.text+0x127ad70>
14127bd95: 48 89 9d a8 01 00 00        	movq	%rbx, 0x1a8(%rbp)
14127bd9c: 8d 46 ff                    	leal	-0x1(%rsi), %eax
14127bd9f: 3c 03                       	cmpb	$0x3, %al
14127bda1: 0f 83 b7 07 00 00           	jae	0x14127c55e <.text+0x127b55e>
14127bda7: 48 8d 14 29                 	leaq	(%rcx,%rbp), %rdx
14127bdab: 48 81 c2 88 01 00 00        	addq	$0x188, %rdx            # imm = 0x188
14127bdb2: 0f b6 c0                    	movzbl	%al, %eax
14127bdb5: 44 0f 28 d7                 	movaps	%xmm7, %xmm10
14127bdb9: f3 45 0f 5c 14 86           	subss	(%r14,%rax,4), %xmm10
14127bdbf: 4c 8b 85 60 01 00 00        	movq	0x160(%rbp), %r8
14127bdc6: 49 83 f8 03                 	cmpq	$0x3, %r8
14127bdca: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
14127bdd1: 49 0f 42 c5                 	cmovbq	%r13, %rax
14127bdd5: f3 45 0f 59 d1              	mulss	%xmm9, %xmm10
14127bdda: 48 89 f9                    	movq	%rdi, %rcx
14127bddd: 4c 8d 9d 48 01 00 00        	leaq	0x148(%rbp), %r11
14127bde4: 49 0f 43 cb                 	cmovaeq	%r11, %rcx
14127bde8: f3 44 0f 10 5a 08           	movss	0x8(%rdx), %xmm11
14127bdee: 4d 0f 42 c7                 	cmovbq	%r15, %r8
14127bdf2: 48 89 fa                    	movq	%rdi, %rdx
14127bdf5: 49 0f 43 d4                 	cmovaeq	%r12, %rdx
14127bdf9: 48 8b 12                    	movq	(%rdx), %rdx
14127bdfc: 4c 39 c2                    	cmpq	%r8, %rdx
14127bdff: 74 32                       	je	0x14127be33 <.text+0x127ae33>
14127be01: f3 45 0f 59 d0              	mulss	%xmm8, %xmm10
14127be06: 44 0f 14 d6                 	unpcklps	%xmm6, %xmm10           # xmm10 = xmm10[0],xmm6[0],xmm10[1],xmm6[1]
14127be0a: f3 41 0f 12 c3              	movsldup	%xmm11, %xmm0           # xmm0 = xmm11[0,0,2,2]
14127be0f: 41 0f 59 c2                 	mulps	%xmm10, %xmm0
14127be13: 48 c1 e2 04                 	shlq	$0x4, %rdx
14127be17: c7 04 10 01 00 00 00        	movl	$0x1, (%rax,%rdx)
14127be1e: 0f 13 44 10 04              	movlps	%xmm0, 0x4(%rax,%rdx)
14127be23: 40 88 74 10 0c              	movb	%sil, 0xc(%rax,%rdx)
14127be28: 48 ff 01                    	incq	(%rcx)
14127be2b: 48 89 d8                    	movq	%rbx, %rax
14127be2e: e9 3d ff ff ff              	jmp	0x14127bd70 <.text+0x127ad70>
14127be33: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127be3a: 4c 89 e9                    	movq	%r13, %rcx
14127be3d: e8 ae c4 67 01              	callq	0x1428f82f0 <.text+0x28f72f0>
14127be42: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
14127be49: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
14127be50: 48 8d 8d 48 01 00 00        	leaq	0x148(%rbp), %rcx
14127be57: 4c 8b 8d 80 01 00 00        	movq	0x180(%rbp), %r9
14127be5e: 44 0f b6 95 26 02 00 00     	movzbl	0x226(%rbp), %r10d
14127be66: eb 99                       	jmp	0x14127be01 <.text+0x127ae01>
14127be68: 45 84 d2                    	testb	%r10b, %r10b
14127be6b: 0f 84 1d 07 00 00           	je	0x14127c58e <.text+0x127b58e>
14127be71: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
14127be78: 48 ff c9                    	decq	%rcx
14127be7b: 4d 85 c9                    	testq	%r9, %r9
14127be7e: 78 29                       	js	0x14127bea9 <.text+0x127aea9>
14127be80: 0f 57 f6                    	xorps	%xmm6, %xmm6
14127be83: f3 49 0f 2a f1              	cvtsi2ss	%r9, %xmm6
14127be88: 48 85 c9                    	testq	%rcx, %rcx
14127be8b: 79 3a                       	jns	0x14127bec7 <.text+0x127aec7>
14127be8d: 48 89 c8                    	movq	%rcx, %rax
14127be90: 48 d1 e8                    	shrq	%rax
14127be93: 83 e1 01                    	andl	$0x1, %ecx
14127be96: 48 09 c1                    	orq	%rax, %rcx
14127be99: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
14127be9d: f3 4c 0f 2a c9              	cvtsi2ss	%rcx, %xmm9
14127bea2: f3 45 0f 58 c9              	addss	%xmm9, %xmm9
14127bea7: eb 27                       	jmp	0x14127bed0 <.text+0x127aed0>
14127bea9: 4c 89 c8                    	movq	%r9, %rax
14127beac: 48 d1 e8                    	shrq	%rax
14127beaf: 41 83 e1 01                 	andl	$0x1, %r9d
14127beb3: 49 09 c1                    	orq	%rax, %r9
14127beb6: 0f 57 f6                    	xorps	%xmm6, %xmm6
14127beb9: f3 49 0f 2a f1              	cvtsi2ss	%r9, %xmm6
14127bebe: f3 0f 58 f6                 	addss	%xmm6, %xmm6
14127bec2: 48 85 c9                    	testq	%rcx, %rcx
14127bec5: 78 c6                       	js	0x14127be8d <.text+0x127ae8d>
14127bec7: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
14127becb: f3 44 0f 2a c9              	cvtsi2ss	%ecx, %xmm9
14127bed0: 4c 8b bd 40 01 00 00        	movq	0x140(%rbp), %r15
14127bed7: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
14127bede: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
14127bee5: 48 8b 9d 60 01 00 00        	movq	0x160(%rbp), %rbx
14127beec: 45 31 f6                    	xorl	%r14d, %r14d
14127beef: 48 83 fb 03                 	cmpq	$0x3, %rbx
14127bef3: 49 89 dd                    	movq	%rbx, %r13
14127bef6: 4c 0f 43 e8                 	cmovaeq	%rax, %r13
14127befa: b9 00 00 00 00              	movl	$0x0, %ecx
14127beff: 48 0f 43 cb                 	cmovaeq	%rbx, %rcx
14127bf03: 49 0f 43 c6                 	cmovaeq	%r14, %rax
14127bf07: 0f 11 85 10 01 00 00        	movups	%xmm0, 0x110(%rbp)
14127bf0e: 4c 89 bd 00 01 00 00        	movq	%r15, 0x100(%rbp)
14127bf15: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
14127bf1c: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
14127bf23: 48 c7 85 28 01 00 00 00 00 00 00    	movq	$0x0, 0x128(%rbp)
14127bf2e: 4c 89 ad 30 01 00 00        	movq	%r13, 0x130(%rbp)
14127bf35: 48 c7 85 f8 01 00 00 00 00 00 00    	movq	$0x0, 0x1f8(%rbp)
14127bf40: 48 c7 85 00 02 00 00 04 00 00 00    	movq	$0x4, 0x200(%rbp)
14127bf4b: 48 c7 85 08 02 00 00 00 00 00 00    	movq	$0x0, 0x208(%rbp)
14127bf56: 48 8b 85 48 03 00 00        	movq	0x348(%rbp), %rax
14127bf5d: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14127bf61: 48 8d 85 c0 00 00 00        	leaq	0xc0(%rbp), %rax
14127bf68: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14127bf6c: 48 8d 85 b0 00 00 00        	leaq	0xb0(%rbp), %rax
14127bf73: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14127bf77: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
14127bf7e: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14127bf82: 4d 85 ed                    	testq	%r13, %r13
14127bf85: 0f 84 6c 03 00 00           	je	0x14127c2f7 <.text+0x127b2f7>
14127bf8b: 48 83 fb 03                 	cmpq	$0x3, %rbx
14127bf8f: 48 c7 85 28 01 00 00 01 00 00 00    	movq	$0x1, 0x128(%rbp)
14127bf9a: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
14127bfa1: 49 0f 43 c7                 	cmovaeq	%r15, %rax
14127bfa5: 8b 38                       	movl	(%rax), %edi
14127bfa7: 83 ff 02                    	cmpl	$0x2, %edi
14127bfaa: 0f 84 36 01 00 00           	je	0x14127c0e6 <.text+0x127b0e6>
14127bfb0: 44 8b 60 04                 	movl	0x4(%rax), %r12d
14127bfb4: 40 8a 70 0c                 	movb	0xc(%rax), %sil
14127bfb8: 48 8d 15 e1 7e 9a 01        	leaq	0x19a7ee1(%rip), %rdx   # 0x142c23ea0
14127bfbf: 48 8d 8d f8 01 00 00        	leaq	0x1f8(%rbp), %rcx
14127bfc6: e8 a5 38 8e ff              	callq	0x140b5f870 <.text+0xb5e870>
14127bfcb: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
14127bfd2: 8b 8d 00 03 00 00           	movl	0x300(%rbp), %ecx
14127bfd8: 89 08                       	movl	%ecx, (%rax)
14127bfda: 48 c7 85 08 02 00 00 01 00 00 00    	movq	$0x1, 0x208(%rbp)
14127bfe5: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
14127bfec: 44 8a 70 46                 	movb	0x46(%rax), %r14b
14127bff0: 66 41 0f 6e dc              	movd	%r12d, %xmm3
14127bff5: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
14127bff9: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
14127bffd: 41 89 f8                    	movl	%edi, %r8d
14127c000: e8 eb 07 00 00              	callq	0x14127c7f0 <.text+0x127b7f0>
14127c005: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
14127c009: 0f 10 4d 00                 	movups	(%rbp), %xmm1
14127c00d: 0f 10 55 10                 	movups	0x10(%rbp), %xmm2
14127c011: f3 0f 6f 5d 20              	movdqu	0x20(%rbp), %xmm3
14127c016: f3 0f 7f 9d c8 01 00 00     	movdqu	%xmm3, 0x1c8(%rbp)
14127c01e: 0f 11 95 b8 01 00 00        	movups	%xmm2, 0x1b8(%rbp)
14127c025: 0f 11 8d a8 01 00 00        	movups	%xmm1, 0x1a8(%rbp)
14127c02c: 0f 11 85 98 01 00 00        	movups	%xmm0, 0x198(%rbp)
14127c033: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14127c03a: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14127c041: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
14127c048: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
14127c04f: c7 85 d8 01 00 00 00 00 00 00       	movl	$0x0, 0x1d8(%rbp)
14127c059: c7 85 e0 01 00 00 00 00 00 00       	movl	$0x0, 0x1e0(%rbp)
14127c063: c6 85 e8 01 00 00 01        	movb	$0x1, 0x1e8(%rbp)
14127c06a: 40 88 b5 e9 01 00 00        	movb	%sil, 0x1e9(%rbp)
14127c071: c6 85 ea 01 00 00 00        	movb	$0x0, 0x1ea(%rbp)
14127c078: 66 c7 85 ec 01 00 00 02 40  	movw	$0x4002, 0x1ec(%rbp)    # imm = 0x4002
14127c081: 8b 85 00 03 00 00           	movl	0x300(%rbp), %eax
14127c087: 89 85 f0 01 00 00           	movl	%eax, 0x1f0(%rbp)
14127c08d: 44 88 b5 f4 01 00 00        	movb	%r14b, 0x1f4(%rbp)
14127c094: c6 45 fc 02                 	movb	$0x2, -0x4(%rbp)
14127c098: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
14127c09f: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14127c0a4: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
14127c0ab: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14127c0b0: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
14127c0b7: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14127c0bc: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
14127c0c3: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14127c0c8: 48 8d 8d 88 01 00 00        	leaq	0x188(%rbp), %rcx
14127c0cf: 4c 8d 45 f0                 	leaq	-0x10(%rbp), %r8
14127c0d3: 0f b6 95 40 03 00 00        	movzbl	0x340(%rbp), %edx
14127c0da: 4c 8b 8d 68 01 00 00        	movq	0x168(%rbp), %r9
14127c0e1: e8 4a ad ff ff              	callq	0x141276e30 <.text+0x1275e30>
14127c0e6: 41 be 01 00 00 00           	movl	$0x1, %r14d
14127c0ec: 49 83 fd 01                 	cmpq	$0x1, %r13
14127c0f0: 0f 84 01 02 00 00           	je	0x14127c2f7 <.text+0x127b2f7>
14127c0f6: 48 83 fb 03                 	cmpq	$0x3, %rbx
14127c0fa: 48 c7 85 28 01 00 00 02 00 00 00    	movq	$0x2, 0x128(%rbp)
14127c105: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
14127c10c: 49 0f 43 c7                 	cmovaeq	%r15, %rax
14127c110: 44 8b 60 10                 	movl	0x10(%rax), %r12d
14127c114: 41 83 fc 02                 	cmpl	$0x2, %r12d
14127c118: 0f 84 a3 01 00 00           	je	0x14127c2c1 <.text+0x127b2c1>
14127c11e: 44 8b 70 14                 	movl	0x14(%rax), %r14d
14127c122: 0f b6 40 1c                 	movzbl	0x1c(%rax), %eax
14127c126: 88 85 18 02 00 00           	movb	%al, 0x218(%rbp)
14127c12c: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
14127c133: e8 a8 78 5b ff              	callq	0x1408339e0 <.text+0x8329e0>
14127c138: 89 c7                       	movl	%eax, %edi
14127c13a: 48 8b b5 08 02 00 00        	movq	0x208(%rbp), %rsi
14127c141: 48 3b b5 f8 01 00 00        	cmpq	0x1f8(%rbp), %rsi
14127c148: 75 13                       	jne	0x14127c15d <.text+0x127b15d>
14127c14a: 48 8d 15 67 7d 9a 01        	leaq	0x19a7d67(%rip), %rdx   # 0x142c23eb8
14127c151: 48 8d 8d f8 01 00 00        	leaq	0x1f8(%rbp), %rcx
14127c158: e8 13 37 8e ff              	callq	0x140b5f870 <.text+0xb5e870>
14127c15d: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
14127c164: 89 3c b0                    	movl	%edi, (%rax,%rsi,4)
14127c167: 48 ff c6                    	incq	%rsi
14127c16a: 48 89 b5 08 02 00 00        	movq	%rsi, 0x208(%rbp)
14127c171: 66 41 0f 6e de              	movd	%r14d, %xmm3
14127c176: 48 8d 8d 88 01 00 00        	leaq	0x188(%rbp), %rcx
14127c17d: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
14127c181: 45 89 e0                    	movl	%r12d, %r8d
14127c184: e8 67 06 00 00              	callq	0x14127c7f0 <.text+0x127b7f0>
14127c189: 83 bd 88 01 00 00 03        	cmpl	$0x3, 0x188(%rbp)
14127c190: 75 15                       	jne	0x14127c1a7 <.text+0x127b1a7>
14127c192: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
14127c196: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
14127c19d: 0f 28 45 c0                 	movaps	-0x40(%rbp), %xmm0
14127c1a1: 0f 28 4d d0                 	movaps	-0x30(%rbp), %xmm1
14127c1a5: eb 1c                       	jmp	0x14127c1c3 <.text+0x127b1c3>
14127c1a7: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
14127c1ae: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
14127c1b5: 0f 10 85 98 01 00 00        	movups	0x198(%rbp), %xmm0
14127c1bc: 0f 10 8d a8 01 00 00        	movups	0x1a8(%rbp), %xmm1
14127c1c3: 0f 29 8d 90 00 00 00        	movaps	%xmm1, 0x90(%rbp)
14127c1ca: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
14127c1d1: 0f b6 8d 18 02 00 00        	movzbl	0x218(%rbp), %ecx
14127c1d8: 0f 28 85 e0 00 00 00        	movaps	0xe0(%rbp), %xmm0
14127c1df: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
14127c1e6: 8b 85 f0 00 00 00           	movl	0xf0(%rbp), %eax
14127c1ec: 89 85 d8 01 00 00           	movl	%eax, 0x1d8(%rbp)
14127c1f2: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14127c1f9: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
14127c200: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
14127c207: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
14127c20e: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
14127c215: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
14127c21c: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
14127c223: 0f 11 8d b0 01 00 00        	movups	%xmm1, 0x1b0(%rbp)
14127c22a: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14127c231: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14127c238: 89 bd dc 01 00 00           	movl	%edi, 0x1dc(%rbp)
14127c23e: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
14127c244: 89 85 e0 01 00 00           	movl	%eax, 0x1e0(%rbp)
14127c24a: c6 85 e4 01 00 00 01        	movb	$0x1, 0x1e4(%rbp)
14127c251: 0f b6 85 25 02 00 00        	movzbl	0x225(%rbp), %eax
14127c258: 88 85 e5 01 00 00           	movb	%al, 0x1e5(%rbp)
14127c25e: 88 8d e6 01 00 00           	movb	%cl, 0x1e6(%rbp)
14127c264: 48 c7 85 88 01 00 00 03 00 00 00    	movq	$0x3, 0x188(%rbp)
14127c26f: c6 45 fc 02                 	movb	$0x2, -0x4(%rbp)
14127c273: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
14127c27a: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14127c27f: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
14127c286: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14127c28b: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
14127c292: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14127c297: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
14127c29e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14127c2a3: 48 8d 8d 88 01 00 00        	leaq	0x188(%rbp), %rcx
14127c2aa: 4c 8d 45 f0                 	leaq	-0x10(%rbp), %r8
14127c2ae: 0f b6 95 40 03 00 00        	movzbl	0x340(%rbp), %edx
14127c2b5: 4c 8b 8d 68 01 00 00        	movq	0x168(%rbp), %r9
14127c2bc: e8 6f ab ff ff              	callq	0x141276e30 <.text+0x1275e30>
14127c2c1: 41 be 02 00 00 00           	movl	$0x2, %r14d
14127c2c7: 49 83 fd 02                 	cmpq	$0x2, %r13
14127c2cb: 74 2a                       	je	0x14127c2f7 <.text+0x127b2f7>
14127c2cd: 48 83 fb 03                 	cmpq	$0x3, %rbx
14127c2d1: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
14127c2d8: 49 0f 43 c7                 	cmovaeq	%r15, %rax
14127c2dc: 48 c7 85 28 01 00 00 03 00 00 00    	movq	$0x3, 0x128(%rbp)
14127c2e7: 41 be 03 00 00 00           	movl	$0x3, %r14d
14127c2ed: 83 78 20 02                 	cmpl	$0x2, 0x20(%rax)
14127c2f1: 0f 85 4a 02 00 00           	jne	0x14127c541 <.text+0x127b541>
14127c2f7: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
14127c2fe: e8 dd 76 5b ff              	callq	0x1408339e0 <.text+0x8329e0>
14127c303: 89 c7                       	movl	%eax, %edi
14127c305: f3 0f 58 3d 43 37 80 01     	addss	0x1803743(%rip), %xmm7  # 0x142a7fa50
14127c30d: f3 0f 59 3d 5b 95 6a 01     	mulss	0x16a955b(%rip), %xmm7  # 0x142925870
14127c315: f3 0f 58 3d 4b 0d 75 01     	addss	0x1750d4b(%rip), %xmm7  # 0x1429cd068
14127c31d: f3 44 0f 59 c7              	mulss	%xmm7, %xmm8
14127c322: f3 41 0f 5e f1              	divss	%xmm9, %xmm6
14127c327: f3 0f 58 f6                 	addss	%xmm6, %xmm6
14127c32b: f3 0f 58 35 b1 dd 6d 01     	addss	0x16dddb1(%rip), %xmm6  # 0x14295a0e4
14127c333: f3 41 0f 59 f0              	mulss	%xmm8, %xmm6
14127c338: 48 8d 8d 88 01 00 00        	leaq	0x188(%rbp), %rcx
14127c33f: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
14127c343: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14127c349: 0f 28 de                    	movaps	%xmm6, %xmm3
14127c34c: e8 9f 04 00 00              	callq	0x14127c7f0 <.text+0x127b7f0>
14127c351: 83 bd 88 01 00 00 03        	cmpl	$0x3, 0x188(%rbp)
14127c358: 48 8d 85 98 01 00 00        	leaq	0x198(%rbp), %rax
14127c35f: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14127c363: 48 0f 45 c8                 	cmovneq	%rax, %rcx
14127c367: 0f 10 01                    	movups	(%rcx), %xmm0
14127c36a: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
14127c36e: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
14127c372: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
14127c376: 48 8b 41 20                 	movq	0x20(%rcx), %rax
14127c37a: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14127c37e: e8 ed f7 db fe              	callq	0x14003bb70 <.text+0x3ab70>
14127c383: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14127c387: 48 89 8d c0 01 00 00        	movq	%rcx, 0x1c0(%rbp)
14127c38e: 0f 28 45 50                 	movaps	0x50(%rbp), %xmm0
14127c392: 0f 28 4d 60                 	movaps	0x60(%rbp), %xmm1
14127c396: 0f 11 8d b0 01 00 00        	movups	%xmm1, 0x1b0(%rbp)
14127c39d: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
14127c3a4: 0f 28 85 e0 00 00 00        	movaps	0xe0(%rbp), %xmm0
14127c3ab: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
14127c3b2: 8b 8d f0 00 00 00           	movl	0xf0(%rbp), %ecx
14127c3b8: 89 8d d8 01 00 00           	movl	%ecx, 0x1d8(%rbp)
14127c3be: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14127c3c5: 48 89 8d 90 01 00 00        	movq	%rcx, 0x190(%rbp)
14127c3cc: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
14127c3d3: 48 89 8d 98 01 00 00        	movq	%rcx, 0x198(%rbp)
14127c3da: 89 bd dc 01 00 00           	movl	%edi, 0x1dc(%rbp)
14127c3e0: 8b 8d 14 02 00 00           	movl	0x214(%rbp), %ecx
14127c3e6: 89 8d e0 01 00 00           	movl	%ecx, 0x1e0(%rbp)
14127c3ec: c6 85 e4 01 00 00 01        	movb	$0x1, 0x1e4(%rbp)
14127c3f3: 0f b6 8d 25 02 00 00        	movzbl	0x225(%rbp), %ecx
14127c3fa: 88 8d e5 01 00 00           	movb	%cl, 0x1e5(%rbp)
14127c400: 88 85 e6 01 00 00           	movb	%al, 0x1e6(%rbp)
14127c406: 48 c7 85 88 01 00 00 03 00 00 00    	movq	$0x3, 0x188(%rbp)
14127c411: c6 45 fc 02                 	movb	$0x2, -0x4(%rbp)
14127c415: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
14127c41c: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14127c421: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
14127c428: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14127c42d: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
14127c434: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14127c439: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
14127c440: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14127c445: 48 8d 8d 88 01 00 00        	leaq	0x188(%rbp), %rcx
14127c44c: 4c 8d 45 f0                 	leaq	-0x10(%rbp), %r8
14127c450: 0f b6 95 40 03 00 00        	movzbl	0x340(%rbp), %edx
14127c457: 4c 8b 8d 68 01 00 00        	movq	0x168(%rbp), %r9
14127c45e: e8 cd a9 ff ff              	callq	0x141276e30 <.text+0x1275e30>
14127c463: 48 83 fb 03                 	cmpq	$0x3, %rbx
14127c467: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
14127c46e: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
14127c475: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
14127c479: 0f 10 85 f8 01 00 00        	movups	0x1f8(%rbp), %xmm0
14127c480: 0f 11 01                    	movups	%xmm0, (%rcx)
14127c483: 89 79 18                    	movl	%edi, 0x18(%rcx)
14127c486: 0f 28 45 50                 	movaps	0x50(%rbp), %xmm0
14127c48a: 0f 28 4d 60                 	movaps	0x60(%rbp), %xmm1
14127c48e: 0f 11 41 20                 	movups	%xmm0, 0x20(%rcx)
14127c492: 0f 11 49 30                 	movups	%xmm1, 0x30(%rcx)
14127c496: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14127c49a: 48 89 41 40                 	movq	%rax, 0x40(%rcx)
14127c49e: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14127c4a5: 49 0f 43 cf                 	cmovaeq	%r15, %rcx
14127c4a9: 44 89 f0                    	movl	%r14d, %eax
14127c4ac: c1 e0 04                    	shll	$0x4, %eax
14127c4af: 48 01 c8                    	addq	%rcx, %rax
14127c4b2: 49 ff c6                    	incq	%r14
14127c4b5: 49 f7 dd                    	negq	%r13
14127c4b8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14127c4c0: 4b 8d 0c 2e                 	leaq	(%r14,%r13), %rcx
14127c4c4: 48 83 f9 01                 	cmpq	$0x1, %rcx
14127c4c8: 74 13                       	je	0x14127c4dd <.text+0x127b4dd>
14127c4ca: 4c 89 b5 28 01 00 00        	movq	%r14, 0x128(%rbp)
14127c4d1: 49 ff c6                    	incq	%r14
14127c4d4: 83 38 02                    	cmpl	$0x2, (%rax)
14127c4d7: 48 8d 40 10                 	leaq	0x10(%rax), %rax
14127c4db: 75 e3                       	jne	0x14127c4c0 <.text+0x127b4c0>
14127c4dd: 48 83 fb 03                 	cmpq	$0x3, %rbx
14127c4e1: 72 15                       	jb	0x14127c4f8 <.text+0x127b4f8>
14127c4e3: 48 c1 e3 04                 	shlq	$0x4, %rbx
14127c4e7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14127c4ed: 4c 89 f9                    	movq	%r15, %rcx
14127c4f0: 48 89 da                    	movq	%rbx, %rdx
14127c4f3: e8 28 77 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127c4f8: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
14127c4ff: 0f 28 b5 30 02 00 00        	movaps	0x230(%rbp), %xmm6
14127c506: 0f 28 bd 40 02 00 00        	movaps	0x240(%rbp), %xmm7
14127c50d: 44 0f 28 85 50 02 00 00     	movaps	0x250(%rbp), %xmm8
14127c515: 44 0f 28 8d 60 02 00 00     	movaps	0x260(%rbp), %xmm9
14127c51d: 44 0f 28 95 70 02 00 00     	movaps	0x270(%rbp), %xmm10
14127c525: 44 0f 28 9d 80 02 00 00     	movaps	0x280(%rbp), %xmm11
14127c52d: 48 81 c4 18 03 00 00        	addq	$0x318, %rsp            # imm = 0x318
14127c534: 5b                          	popq	%rbx
14127c535: 5f                          	popq	%rdi
14127c536: 5e                          	popq	%rsi
14127c537: 41 5c                       	popq	%r12
14127c539: 41 5d                       	popq	%r13
14127c53b: 41 5e                       	popq	%r14
14127c53d: 41 5f                       	popq	%r15
14127c53f: 5d                          	popq	%rbp
14127c540: c3                          	retq
14127c541: 48 8d 0d 88 79 9a 01        	leaq	0x19a7988(%rip), %rcx   # 0x142c23ed0
14127c548: 4c 8d 05 b9 79 9a 01        	leaq	0x19a79b9(%rip), %r8    # 0x142c23f08
14127c54f: ba 32 00 00 00              	movl	$0x32, %edx
14127c554: e8 d7 ce 65 01              	callq	0x1428d9430 <.text+0x28d8430>
14127c559: e9 1c 01 00 00              	jmp	0x14127c67a <.text+0x127b67a>
14127c55e: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127c565: 48 8d 0d 0c ab 80 01        	leaq	0x180ab0c(%rip), %rcx   # 0x142a87078
14127c56c: 4c 8d 05 7d ab 80 01        	leaq	0x180ab7d(%rip), %r8    # 0x142a870f0
14127c573: ba 28 00 00 00              	movl	$0x28, %edx
14127c578: e8 b3 ce 65 01              	callq	0x1428d9430 <.text+0x28d8430>
14127c57d: e9 f8 00 00 00              	jmp	0x14127c67a <.text+0x127b67a>
14127c582: 48 8d 0d cf 78 9a 01        	leaq	0x19a78cf(%rip), %rcx   # 0x142c23e58
14127c589: e8 82 cd 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127c58e: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127c595: 48 8d 0d ec 78 9a 01        	leaq	0x19a78ec(%rip), %rcx   # 0x142c23e88
14127c59c: e8 6f cd 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127c5a1: e9 d4 00 00 00              	jmp	0x14127c67a <.text+0x127b67a>
14127c5a6: 48 8d 05 cb 79 9a 01        	leaq	0x19a79cb(%rip), %rax   # 0x142c23f78
14127c5ad: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14127c5b4: 48 c7 85 90 01 00 00 01 00 00 00    	movq	$0x1, 0x190(%rbp)
14127c5bf: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
14127c5c6: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
14127c5cd: 0f 57 c0                    	xorps	%xmm0, %xmm0
14127c5d0: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
14127c5d7: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127c5de: 48 8d 15 a3 79 9a 01        	leaq	0x19a79a3(%rip), %rdx   # 0x142c23f88
14127c5e5: 48 8d 8d 88 01 00 00        	leaq	0x188(%rbp), %rcx
14127c5ec: e8 9f cd 65 01              	callq	0x1428d9390 <.text+0x28d8390>
14127c5f1: e9 84 00 00 00              	jmp	0x14127c67a <.text+0x127b67a>
14127c5f6: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127c5fd: 48 8d 0d 74 aa 80 01        	leaq	0x180aa74(%rip), %rcx   # 0x142a87078
14127c604: 4c 8d 05 e5 aa 80 01        	leaq	0x180aae5(%rip), %r8    # 0x142a870f0
14127c60b: ba 28 00 00 00              	movl	$0x28, %edx
14127c610: e8 1b ce 65 01              	callq	0x1428d9430 <.text+0x28d8430>
14127c615: eb 63                       	jmp	0x14127c67a <.text+0x127b67a>
14127c617: 48 89 5d f0                 	movq	%rbx, -0x10(%rbp)
14127c61b: 48 8d 05 ce 08 fe ff        	leaq	-0x1f732(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
14127c622: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14127c626: 48 8d 05 53 56 9a 01        	leaq	0x19a5653(%rip), %rax   # 0x142c21c80
14127c62d: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14127c634: 48 c7 85 90 01 00 00 01 00 00 00    	movq	$0x1, 0x190(%rbp)
14127c63f: 48 c7 85 a8 01 00 00 00 00 00 00    	movq	$0x0, 0x1a8(%rbp)
14127c64a: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
14127c64e: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
14127c655: 48 c7 85 a0 01 00 00 01 00 00 00    	movq	$0x1, 0x1a0(%rbp)
14127c660: c6 85 27 02 00 00 01        	movb	$0x1, 0x227(%rbp)
14127c667: 48 8d 15 3a 56 9a 01        	leaq	0x19a563a(%rip), %rdx   # 0x142c21ca8
14127c66e: 48 8d 8d 88 01 00 00        	leaq	0x188(%rbp), %rcx
14127c675: e8 16 cd 65 01              	callq	0x1428d9390 <.text+0x28d8390>
14127c67a: 0f 0b                       	ud2
14127c67c: 0f 1f 40 00                 	nopl	(%rax)
14127c680: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14127c685: 55                          	pushq	%rbp
14127c686: 41 57                       	pushq	%r15
14127c688: 41 56                       	pushq	%r14
14127c68a: 41 55                       	pushq	%r13
14127c68c: 41 54                       	pushq	%r12
14127c68e: 56                          	pushq	%rsi
14127c68f: 57                          	pushq	%rdi
14127c690: 53                          	pushq	%rbx
14127c691: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
14127c698: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14127c69f: 44 0f 29 5c 24 40           	movaps	%xmm11, 0x40(%rsp)
14127c6a5: 44 0f 29 54 24 50           	movaps	%xmm10, 0x50(%rsp)
14127c6ab: 44 0f 29 4c 24 60           	movaps	%xmm9, 0x60(%rsp)
14127c6b1: 44 0f 29 44 24 70           	movaps	%xmm8, 0x70(%rsp)
14127c6b7: 0f 29 bc 24 80 00 00 00     	movaps	%xmm7, 0x80(%rsp)
14127c6bf: 0f 29 b4 24 90 00 00 00     	movaps	%xmm6, 0x90(%rsp)
14127c6c7: 48 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %rdx
14127c6ce: 48 85 d2                    	testq	%rdx, %rdx
14127c6d1: 74 16                       	je	0x14127c6e9 <.text+0x127b6e9>
14127c6d3: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
14127c6da: 48 c1 e2 02                 	shlq	$0x2, %rdx
14127c6de: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14127c6e4: e8 37 75 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127c6e9: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14127c6f0: e8 fb cb fd ff              	callq	0x1412592f0 <.text+0x12582f0>
14127c6f5: c6 85 27 02 00 00 00        	movb	$0x0, 0x227(%rbp)
14127c6fc: 0f 28 b4 24 90 00 00 00     	movaps	0x90(%rsp), %xmm6
14127c704: 0f 28 bc 24 80 00 00 00     	movaps	0x80(%rsp), %xmm7
14127c70c: 44 0f 28 44 24 70           	movaps	0x70(%rsp), %xmm8
14127c712: 44 0f 28 4c 24 60           	movaps	0x60(%rsp), %xmm9
14127c718: 44 0f 28 54 24 50           	movaps	0x50(%rsp), %xmm10
14127c71e: 44 0f 28 5c 24 40           	movaps	0x40(%rsp), %xmm11
14127c724: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
14127c72b: 5b                          	popq	%rbx
14127c72c: 5f                          	popq	%rdi
14127c72d: 5e                          	popq	%rsi
14127c72e: 41 5c                       	popq	%r12
14127c730: 41 5d                       	popq	%r13
14127c732: 41 5e                       	popq	%r14
14127c734: 41 5f                       	popq	%r15
14127c736: 5d                          	popq	%rbp
14127c737: c3                          	retq
14127c738: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14127c740: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14127c745: 55                          	pushq	%rbp
14127c746: 41 57                       	pushq	%r15
14127c748: 41 56                       	pushq	%r14
14127c74a: 41 55                       	pushq	%r13
14127c74c: 41 54                       	pushq	%r12
14127c74e: 56                          	pushq	%rsi
14127c74f: 57                          	pushq	%rdi
14127c750: 53                          	pushq	%rbx
14127c751: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
14127c758: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14127c75f: 44 0f 29 5c 24 40           	movaps	%xmm11, 0x40(%rsp)
14127c765: 44 0f 29 54 24 50           	movaps	%xmm10, 0x50(%rsp)
14127c76b: 44 0f 29 4c 24 60           	movaps	%xmm9, 0x60(%rsp)
14127c771: 44 0f 29 44 24 70           	movaps	%xmm8, 0x70(%rsp)
14127c777: 0f 29 bc 24 80 00 00 00     	movaps	%xmm7, 0x80(%rsp)
14127c77f: 0f 29 b4 24 90 00 00 00     	movaps	%xmm6, 0x90(%rsp)
14127c787: 80 bd 27 02 00 00 00        	cmpb	$0x0, 0x227(%rbp)
14127c78e: 74 23                       	je	0x14127c7b3 <.text+0x127b7b3>
14127c790: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
14127c797: 48 83 fa 03                 	cmpq	$0x3, %rdx
14127c79b: 72 16                       	jb	0x14127c7b3 <.text+0x127b7b3>
14127c79d: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
14127c7a4: 48 c1 e2 04                 	shlq	$0x4, %rdx
14127c7a8: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14127c7ae: e8 6d 74 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127c7b3: 0f 28 b4 24 90 00 00 00     	movaps	0x90(%rsp), %xmm6
14127c7bb: 0f 28 bc 24 80 00 00 00     	movaps	0x80(%rsp), %xmm7
14127c7c3: 44 0f 28 44 24 70           	movaps	0x70(%rsp), %xmm8
14127c7c9: 44 0f 28 4c 24 60           	movaps	0x60(%rsp), %xmm9
14127c7cf: 44 0f 28 54 24 50           	movaps	0x50(%rsp), %xmm10
14127c7d5: 44 0f 28 5c 24 40           	movaps	0x40(%rsp), %xmm11
14127c7db: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
14127c7e2: 5b                          	popq	%rbx
14127c7e3: 5f                          	popq	%rdi
14127c7e4: 5e                          	popq	%rsi
14127c7e5: 41 5c                       	popq	%r12
14127c7e7: 41 5d                       	popq	%r13
14127c7e9: 41 5e                       	popq	%r14
14127c7eb: 41 5f                       	popq	%r15
14127c7ed: 5d                          	popq	%rbp
14127c7ee: c3                          	retq
14127c7ef: cc                          	int3
14127c7f0: 41 56                       	pushq	%r14
14127c7f2: 56                          	pushq	%rsi
14127c7f3: 57                          	pushq	%rdi
14127c7f4: 53                          	pushq	%rbx
14127c7f5: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14127c7fc: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14127c805: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14127c80a: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
14127c80f: 48 89 ce                    	movq	%rcx, %rsi
14127c812: 41 f6 c0 01                 	testb	$0x1, %r8b
14127c816: 74 7f                       	je	0x14127c897 <.text+0x127b897>
14127c818: 0f 28 f3                    	movaps	%xmm3, %xmm6
14127c81b: 48 89 d7                    	movq	%rdx, %rdi
14127c81e: 48 8b 12                    	movq	(%rdx), %rdx
14127c821: f6 42 08 01                 	testb	$0x1, 0x8(%rdx)
14127c825: 74 7c                       	je	0x14127c8a3 <.text+0x127b8a3>
14127c827: 4c 8b 77 08                 	movq	0x8(%rdi), %r14
14127c82b: 41 8b 46 08                 	movl	0x8(%r14), %eax
14127c82f: 89 44 24 28                 	movl	%eax, 0x28(%rsp)
