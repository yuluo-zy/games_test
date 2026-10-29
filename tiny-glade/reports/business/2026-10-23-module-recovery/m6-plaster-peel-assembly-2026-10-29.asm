
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14075bf00: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075bf04: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075bf0a: 48 89 f9                    	movq	%rdi, %rcx
14075bf0d: e8 0e 7d eb ff              	callq	0x140613c20 <.text+0x612c20>
14075bf12: e9 e9 fc ff ff              	jmp	0x14075bc00 <.text+0x75ac00>
14075bf17: 48 8b 8d 10 02 00 00        	movq	0x210(%rbp), %rcx
14075bf1e: 48 8b 01                    	movq	(%rcx), %rax
14075bf21: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
14075bf25: 4c 8b 10                    	movq	(%rax), %r10
14075bf28: 45 31 c9                    	xorl	%r9d, %r9d
14075bf2b: 4d 89 d3                    	movq	%r10, %r11
14075bf2e: 4c 2b 5a 18                 	subq	0x18(%rdx), %r11
14075bf32: 4d 0f 42 d9                 	cmovbq	%r9, %r11
14075bf36: 4c 2b 52 38                 	subq	0x38(%rdx), %r10
14075bf3a: 4d 0f 42 d1                 	cmovbq	%r9, %r10
14075bf3e: 4c 89 d9                    	movq	%r11, %rcx
14075bf41: 48 c1 e1 04                 	shlq	$0x4, %rcx
14075bf45: 48 03 4a 08                 	addq	0x8(%rdx), %rcx
14075bf49: 4c 8b 42 10                 	movq	0x10(%rdx), %r8
14075bf4d: 4d 29 d8                    	subq	%r11, %r8
14075bf50: 4d 0f 42 c1                 	cmovbq	%r9, %r8
14075bf54: 41 bb 08 00 00 00           	movl	$0x8, %r11d
14075bf5a: 49 0f 42 cb                 	cmovbq	%r11, %rcx
14075bf5e: 4c 89 d6                    	movq	%r10, %rsi
14075bf61: 48 c1 e6 04                 	shlq	$0x4, %rsi
14075bf65: 48 03 72 28                 	addq	0x28(%rdx), %rsi
14075bf69: 48 8b 7a 30                 	movq	0x30(%rdx), %rdi
14075bf6d: 4c 29 d7                    	subq	%r10, %rdi
14075bf70: 49 0f 42 f9                 	cmovbq	%r9, %rdi
14075bf74: 49 0f 42 f3                 	cmovbq	%r11, %rsi
14075bf78: 4e 8d 0c 07                 	leaq	(%rdi,%r8), %r9
14075bf7c: 48 8b 52 40                 	movq	0x40(%rdx), %rdx
14075bf80: 4c 29 ca                    	subq	%r9, %rdx
14075bf83: 48 89 10                    	movq	%rdx, (%rax)
14075bf86: 49 c1 e0 04                 	shlq	$0x4, %r8
14075bf8a: 49 01 c8                    	addq	%rcx, %r8
14075bf8d: 48 c1 e7 04                 	shlq	$0x4, %rdi
14075bf91: 48 01 f7                    	addq	%rsi, %rdi
14075bf94: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14075bf9b: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
14075bfa2: 4c 89 85 a0 00 00 00        	movq	%r8, 0xa0(%rbp)
14075bfa9: 48 89 b5 a8 00 00 00        	movq	%rsi, 0xa8(%rbp)
14075bfb0: 48 89 bd b0 00 00 00        	movq	%rdi, 0xb0(%rbp)
14075bfb7: 4c 89 8d b8 00 00 00        	movq	%r9, 0xb8(%rbp)
14075bfbe: 4c 8d 05 ab 0e 32 02        	leaq	0x2320eab(%rip), %r8    # 0x142a7ce70
14075bfc5: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
14075bfc9: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
14075bfd0: e8 fb 76 aa 00              	callq	0x1412036d0 <.text+0x12026d0>
14075bfd5: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14075bfd9: 48 8b 00                    	movq	(%rax), %rax
14075bfdc: 4c 8b 30                    	movq	(%rax), %r14
14075bfdf: 48 8b 40 18                 	movq	0x18(%rax), %rax
14075bfe3: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
14075bfea: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
14075bfef: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14075bff3: f7 d0                       	notl	%eax
14075bff5: 4d 8d 66 10                 	leaq	0x10(%r14), %r12
14075bff9: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
14075c000: 48 8b 0a                    	movq	(%rdx), %rcx
14075c003: 4c 8b 42 10                 	movq	0x10(%rdx), %r8
14075c007: 4c 89 85 f0 00 00 00        	movq	%r8, 0xf0(%rbp)
14075c00e: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
14075c015: 48 8d 49 20                 	leaq	0x20(%rcx), %rcx
14075c019: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
14075c01d: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
14075c024: 48 8b 09                    	movq	(%rcx), %rcx
14075c027: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
14075c02b: 8b 4a 1c                    	movl	0x1c(%rdx), %ecx
14075c02e: 89 8d f8 00 00 00           	movl	%ecx, 0xf8(%rbp)
14075c034: 48 8b 8d 20 02 00 00        	movq	0x220(%rbp), %rcx
14075c03b: 8b 51 1c                    	movl	0x1c(%rcx), %edx
14075c03e: 89 55 04                    	movl	%edx, 0x4(%rbp)
14075c041: 48 8b 11                    	movq	(%rcx), %rdx
14075c044: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
14075c048: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
14075c04c: 48 89 55 f8                 	movq	%rdx, -0x8(%rbp)
14075c050: 48 8d 4a 20                 	leaq	0x20(%rdx), %rcx
14075c054: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
14075c058: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
14075c05d: 66 0f 6e 35 e3 0c 32 02     	movd	0x2320ce3(%rip), %xmm6  # 0x142a7cd48
14075c065: 44 0f 28 15 13 29 1d 02     	movaps	0x21d2913(%rip), %xmm10 # 0x14292e980
14075c06d: f3 44 0f 10 1d fe 45 20 02  	movss	0x22045fe(%rip), %xmm11 # 0x142960674
14075c076: f3 44 0f 10 25 f1 97 1c 02  	movss	0x21c97f1(%rip), %xmm12 # 0x142925870
14075c07f: e9 81 02 00 00              	jmp	0x14075c305 <.text+0x75b305>
14075c084: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14075c090: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
14075c096: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14075c09a: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14075c09e: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14075c0a3: 45 85 d2                    	testl	%r10d, %r10d
14075c0a6: 74 58                       	je	0x14075c100 <.text+0x75b100>
14075c0a8: 4d 8b 1f                    	movq	(%r15), %r11
14075c0ab: 89 b5 88 00 00 00           	movl	%esi, 0x88(%rbp)
14075c0b1: 4c 89 fe                    	movq	%r15, %rsi
14075c0b4: 49 89 df                    	movq	%rbx, %r15
14075c0b7: f3 41 0f bc fa              	tzcntl	%r10d, %edi
14075c0bc: 4c 01 c7                    	addq	%r8, %rdi
14075c0bf: 48 21 d7                    	andq	%rdx, %rdi
14075c0c2: 48 89 fb                    	movq	%rdi, %rbx
14075c0c5: 48 c1 e3 04                 	shlq	$0x4, %rbx
14075c0c9: 4c 89 c8                    	movq	%r9, %rax
14075c0cc: 48 29 d8                    	subq	%rbx, %rax
14075c0cf: 4c 3b 18                    	cmpq	(%rax), %r11
14075c0d2: 74 5c                       	je	0x14075c130 <.text+0x75b130>
14075c0d4: 41 8d 42 ff                 	leal	-0x1(%r10), %eax
14075c0d8: 66 44 21 d0                 	andw	%r10w, %ax
14075c0dc: 41 89 c2                    	movl	%eax, %r10d
14075c0df: 48 8b bd 00 01 00 00        	movq	0x100(%rbp), %rdi
14075c0e6: 4c 89 fb                    	movq	%r15, %rbx
14075c0e9: 49 89 f7                    	movq	%rsi, %r15
14075c0ec: 8b b5 88 00 00 00           	movl	0x88(%rbp), %esi
14075c0f2: 75 bd                       	jne	0x14075c0b1 <.text+0x75b0b1>
14075c0f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14075c100: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
14075c105: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
14075c109: 85 c0                       	testl	%eax, %eax
14075c10b: 0f 85 df 02 00 00           	jne	0x14075c3f0 <.text+0x75b3f0>
14075c111: 48 8b 45 68                 	movq	0x68(%rbp), %rax
14075c115: 49 01 c0                    	addq	%rax, %r8
14075c118: 49 83 c0 10                 	addq	$0x10, %r8
14075c11c: 48 83 c0 10                 	addq	$0x10, %rax
14075c120: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075c124: 49 21 d0                    	andq	%rdx, %r8
14075c127: e9 64 ff ff ff              	jmp	0x14075c090 <.text+0x75b090>
14075c12c: 0f 1f 40 00                 	nopl	(%rax)
14075c130: 48 f7 df                    	negq	%rdi
14075c133: 48 c1 e7 04                 	shlq	$0x4, %rdi
14075c137: f3 0f 10 44 39 f8           	movss	-0x8(%rcx,%rdi), %xmm0
14075c13d: 66 41 0f 6e 4d f0           	movd	-0x10(%r13), %xmm1
14075c143: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14075c147: f3 0f 10 50 28              	movss	0x28(%rax), %xmm2
14075c14c: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c153: 66 0f 6f de                 	movdqa	%xmm6, %xmm3
14075c157: e8 84 43 53 00              	callq	0x140c904e0 <.text+0xc8f4e0>
14075c15c: 4c 89 ff                    	movq	%r15, %rdi
14075c15f: 49 89 f7                    	movq	%rsi, %r15
14075c162: 8b b5 88 00 00 00           	movl	0x88(%rbp), %esi
14075c168: 0f 28 f8                    	movaps	%xmm0, %xmm7
14075c16b: f3 45 0f 10 45 f0           	movss	-0x10(%r13), %xmm8
14075c171: f3 41 0f 5c c0              	subss	%xmm8, %xmm0
14075c176: 41 0f 54 c2                 	andps	%xmm10, %xmm0
14075c17a: 44 0f 2e d8                 	ucomiss	%xmm0, %xmm11
14075c17e: 0f 87 18 01 00 00           	ja	0x14075c29c <.text+0x75b29c>
14075c184: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c18b: 48 89 f9                    	movq	%rdi, %rcx
14075c18e: e8 6d e4 3c 00              	callq	0x140b2a600 <.text+0xb29600>
14075c193: 48 85 c0                    	testq	%rax, %rax
14075c196: 74 11                       	je	0x14075c1a9 <.text+0x75b1a9>
14075c198: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c19f: 48 89 c1                    	movq	%rax, %rcx
14075c1a2: e8 f9 30 aa 00              	callq	0x1411ff2a0 <.text+0x11fe2a0>
14075c1a7: eb 07                       	jmp	0x14075c1b0 <.text+0x75b1b0>
14075c1a9: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075c1ac: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14075c1b0: f3 0f 11 45 38              	movss	%xmm0, 0x38(%rbp)
14075c1b5: 66 0f 7e 4d 3c              	movd	%xmm1, 0x3c(%rbp)
14075c1ba: f3 45 0f 10 45 e0           	movss	-0x20(%r13), %xmm8
14075c1c0: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c1c7: 48 89 f9                    	movq	%rdi, %rcx
14075c1ca: e8 51 e4 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075c1cf: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
14075c1d4: f3 45 0f 59 c4              	mulss	%xmm12, %xmm8
14075c1d9: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c1e0: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
14075c1e7: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
14075c1eb: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14075c1ef: e8 ec 0b 54 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14075c1f4: 49 8b 17                    	movq	(%r15), %rdx
14075c1f7: 8b 85 98 00 00 00           	movl	0x98(%rbp), %eax
14075c1fd: 89 45 18                    	movl	%eax, 0x18(%rbp)
14075c200: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
14075c207: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14075c20b: 8b 45 04                    	movl	0x4(%rbp), %eax
14075c20e: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
14075c212: 89 01                       	movl	%eax, (%rcx)
14075c214: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14075c218: 48 8b 58 30                 	movq	0x30(%rax), %rbx
14075c21c: 48 8b 78 40                 	movq	0x40(%rax), %rdi
14075c220: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
14075c224: 75 1f                       	jne	0x14075c245 <.text+0x75b245>
14075c226: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c22d: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
14075c231: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
14075c235: 48 8d 15 8c 02 4c 02        	leaq	0x24c028c(%rip), %rdx   # 0x142c1c4c8
14075c23c: e8 6f 68 e0 ff              	callq	0x140562ab0 <.text+0x561ab0>
14075c241: 48 8b 55 68                 	movq	0x68(%rbp), %rdx
14075c245: 4c 8b 4d f8                 	movq	-0x8(%rbp), %r9
14075c249: 49 8b 41 28                 	movq	0x28(%r9), %rax
14075c24d: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
14075c251: 48 89 3c c8                 	movq	%rdi, (%rax,%rcx,8)
14075c255: 48 89 54 c8 08              	movq	%rdx, 0x8(%rax,%rcx,8)
14075c25a: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
14075c25e: 48 89 54 c8 10              	movq	%rdx, 0x10(%rax,%rcx,8)
14075c263: 8b 55 18                    	movl	0x18(%rbp), %edx
14075c266: 89 54 c8 18                 	movl	%edx, 0x18(%rax,%rcx,8)
14075c26a: c7 44 c8 1c 00 00 80 3f     	movl	$0x3f800000, 0x1c(%rax,%rcx,8) # imm = 0x3F800000
14075c272: c6 44 c8 20 09              	movb	$0x9, 0x20(%rax,%rcx,8)
14075c277: 8b 95 90 00 00 00           	movl	0x90(%rbp), %edx
14075c27d: 44 8b 85 93 00 00 00        	movl	0x93(%rbp), %r8d
14075c284: 44 89 44 c8 24              	movl	%r8d, 0x24(%rax,%rcx,8)
14075c289: 89 54 c8 21                 	movl	%edx, 0x21(%rax,%rcx,8)
14075c28d: 48 ff c3                    	incq	%rbx
14075c290: 49 89 59 30                 	movq	%rbx, 0x30(%r9)
14075c294: 49 ff 41 40                 	incq	0x40(%r9)
14075c298: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
14075c29c: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14075c2a3: 8b 8d f8 00 00 00           	movl	0xf8(%rbp), %ecx
14075c2a9: 89 08                       	movl	%ecx, (%rax)
14075c2ab: 49 8b 17                    	movq	(%r15), %rdx
14075c2ae: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c2b5: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
14075c2bc: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14075c2c0: e8 0b 08 aa 00              	callq	0x1411fcad0 <.text+0x11fbad0>
14075c2c5: 45 0f 2e 45 f0              	ucomiss	-0x10(%r13), %xmm8
14075c2ca: 89 f0                       	movl	%esi, %eax
14075c2cc: 75 02                       	jne	0x14075c2d0 <.text+0x75b2d0>
14075c2ce: 7b 35                       	jnp	0x14075c305 <.text+0x75b305>
14075c2d0: 49 8b 1f                    	movq	(%r15), %rbx
14075c2d3: 48 8b 7d 60                 	movq	0x60(%rbp), %rdi
14075c2d7: 48 3b 7d 50                 	cmpq	0x50(%rbp), %rdi
14075c2db: 75 17                       	jne	0x14075c2f4 <.text+0x75b2f4>
14075c2dd: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c2e4: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
14075c2e8: 48 8d 15 31 0e 32 02        	leaq	0x2320e31(%rip), %rdx   # 0x142a7d120
14075c2ef: e8 7c 65 e0 ff              	callq	0x140562870 <.text+0x561870>
14075c2f4: 48 8b 45 58                 	movq	0x58(%rbp), %rax
14075c2f8: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
14075c2fc: 48 ff c7                    	incq	%rdi
14075c2ff: 48 89 7d 60                 	movq	%rdi, 0x60(%rbp)
