
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a54a40: 55                          	pushq	%rbp
140a54a41: 41 57                       	pushq	%r15
140a54a43: 41 56                       	pushq	%r14
140a54a45: 41 55                       	pushq	%r13
140a54a47: 41 54                       	pushq	%r12
140a54a49: 56                          	pushq	%rsi
140a54a4a: 57                          	pushq	%rdi
140a54a4b: 53                          	pushq	%rbx
140a54a4c: 48 81 ec 98 06 00 00        	subq	$0x698, %rsp            # imm = 0x698
140a54a53: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a54a5b: 66 44 0f 7f 85 00 06 00 00  	movdqa	%xmm8, 0x600(%rbp)
140a54a64: 66 0f 7f bd f0 05 00 00     	movdqa	%xmm7, 0x5f0(%rbp)
140a54a6c: 0f 29 b5 e0 05 00 00        	movaps	%xmm6, 0x5e0(%rbp)
140a54a73: 48 c7 85 d8 05 00 00 fe ff ff ff    	movq	$-0x2, 0x5d8(%rbp)
140a54a7e: 41 89 d6                    	movl	%edx, %r14d
140a54a81: 49 89 cf                    	movq	%rcx, %r15
140a54a84: 48 8d 9d a0 02 00 00        	leaq	0x2a0(%rbp), %rbx
140a54a8b: 48 89 d9                    	movq	%rbx, %rcx
140a54a8e: e8 ad 34 9e 00              	callq	0x141437f40 <.text+0x1436f40>
140a54a93: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
140a54a9a: 48 8b b5 a8 02 00 00        	movq	0x2a8(%rbp), %rsi
140a54aa1: 49 bd 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %r13 # imm = 0x8000000000000004
140a54aab: 4c 39 e8                    	cmpq	%r13, %rax
140a54aae: 0f 85 1c 04 00 00           	jne	0x140a54ed0 <.text+0xa53ed0>
140a54ab4: 48 89 b5 68 05 00 00        	movq	%rsi, 0x568(%rbp)
140a54abb: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
140a54ac2: 48 8d 95 68 05 00 00        	leaq	0x568(%rbp), %rdx
140a54ac9: e8 62 37 9e 00              	callq	0x141438230 <.text+0x1437230>
140a54ace: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
140a54ad5: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
140a54adc: 4c 39 e8                    	cmpq	%r13, %rax
140a54adf: 0f 85 16 04 00 00           	jne	0x140a54efb <.text+0xa53efb>
140a54ae5: 48 89 8d 70 05 00 00        	movq	%rcx, 0x570(%rbp)
140a54aec: 48 89 8d 60 05 00 00        	movq	%rcx, 0x560(%rbp)
140a54af3: c7 44 24 28 00 00 00 00     	movl	$0x0, 0x28(%rsp)
140a54afb: c6 44 24 20 03              	movb	$0x3, 0x20(%rsp)
140a54b00: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
140a54b07: 48 8d 95 60 05 00 00        	leaq	0x560(%rbp), %rdx
140a54b0e: 45 31 c0                    	xorl	%r8d, %r8d
140a54b11: e8 ba 47 9e 00              	callq	0x1414392d0 <.text+0x14382d0>
140a54b16: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
140a54b1d: 4c 39 e8                    	cmpq	%r13, %rax
140a54b20: 0f 85 2c 06 00 00           	jne	0x140a55152 <.text+0xa54152>
140a54b26: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140a54b2a: 48 8d 95 60 05 00 00        	leaq	0x560(%rbp), %rdx
140a54b31: e8 0a 4b 9e 00              	callq	0x141439640 <.text+0x1438640>
140a54b36: 4c 39 6d c0                 	cmpq	%r13, -0x40(%rbp)
140a54b3a: 0f 85 c0 0d 00 00           	jne	0x140a55900 <.text+0xa54900>
140a54b40: 8b 45 c8                    	movl	-0x38(%rbp), %eax
140a54b43: 8b 4d cc                    	movl	-0x34(%rbp), %ecx
140a54b46: 89 85 c4 05 00 00           	movl	%eax, 0x5c4(%rbp)
140a54b4c: 89 8d c0 05 00 00           	movl	%ecx, 0x5c0(%rbp)
140a54b52: 48 8d 7d c0                 	leaq	-0x40(%rbp), %rdi
140a54b56: 48 8d 95 60 05 00 00        	leaq	0x560(%rbp), %rdx
140a54b5d: 48 89 f9                    	movq	%rdi, %rcx
140a54b60: e8 fb 48 9e 00              	callq	0x141439460 <.text+0x1438460>
140a54b65: 4c 39 6d c0                 	cmpq	%r13, -0x40(%rbp)
140a54b69: 0f 85 ef 0d 00 00           	jne	0x140a5595e <.text+0xa5495e>
140a54b6f: f3 0f 2a 75 c8              	cvtsi2ssl	-0x38(%rbp), %xmm6
140a54b74: 44 8b a5 c4 05 00 00        	movl	0x5c4(%rbp), %r12d
140a54b7b: 48 8b 05 26 78 bd 02        	movq	0x2bd7826(%rip), %rax   # 0x14362c3a8
140a54b82: 48 8b 00                    	movq	(%rax), %rax
140a54b85: 48 83 f8 05                 	cmpq	$0x5, %rax
140a54b89: 0f 83 38 07 00 00           	jae	0x140a552c7 <.text+0xa542c7>
140a54b8f: 48 8b 05 12 78 bd 02        	movq	0x2bd7812(%rip), %rax   # 0x14362c3a8
140a54b96: 48 8b 00                    	movq	(%rax), %rax
140a54b99: 48 83 f8 05                 	cmpq	$0x5, %rax
140a54b9d: 0f 83 0e 08 00 00           	jae	0x140a553b1 <.text+0xa543b1>
140a54ba3: 48 8b 05 fe 77 bd 02        	movq	0x2bd77fe(%rip), %rax   # 0x14362c3a8
140a54baa: 48 8b 00                    	movq	(%rax), %rax
140a54bad: 48 83 f8 04                 	cmpq	$0x4, %rax
140a54bb1: 0f 86 46 01 00 00           	jbe	0x140a54cfd <.text+0xa53cfd>
140a54bb7: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a54bba: f3 49 0f 2a c4              	cvtsi2ss	%r12, %xmm0
140a54bbf: f3 0f 59 05 69 6a f0 01     	mulss	0x1f06a69(%rip), %xmm0  # 0x14295b630
140a54bc7: f3 0f 2a 8d c0 05 00 00     	cvtsi2ssl	0x5c0(%rbp), %xmm1
140a54bcf: f3 0f 5e c6                 	divss	%xmm6, %xmm0
140a54bd3: f3 0f 58 0d a5 a5 f8 01     	addss	0x1f8a5a5(%rip), %xmm1  # 0x1429df180
140a54bdb: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
140a54bdf: f3 0f 11 8d 90 05 00 00     	movss	%xmm1, 0x590(%rbp)
140a54be7: 48 8d 85 90 05 00 00        	leaq	0x590(%rbp), %rax
140a54bee: 48 89 85 30 05 00 00        	movq	%rax, 0x530(%rbp)
140a54bf5: 48 8d 05 14 7d 33 00        	leaq	0x337d14(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
140a54bfc: 48 89 85 38 05 00 00        	movq	%rax, 0x538(%rbp)
140a54c03: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140a54c0b: 48 c7 45 c8 02 00 00 00     	movq	$0x2, -0x38(%rbp)
140a54c13: 48 c7 45 d0 02 00 00 00     	movq	$0x2, -0x30(%rbp)
140a54c1b: 48 c7 45 e0 00 00 00 00     	movq	$0x0, -0x20(%rbp)
140a54c23: 48 c7 45 e8 20 00 00 00     	movq	$0x20, -0x18(%rbp)
140a54c2b: c6 45 f0 03                 	movb	$0x3, -0x10(%rbp)
140a54c2f: 48 8d 0d 22 db 07 02        	leaq	0x207db22(%rip), %rcx   # 0x142ad2758
140a54c36: e8 45 91 c2 ff              	callq	0x14067dd80 <.text+0x67cd80>
140a54c3b: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140a54c3f: 8b 40 10                    	movl	0x10(%rax), %eax
140a54c42: 48 c7 85 d0 02 00 00 05 00 00 00    	movq	$0x5, 0x2d0(%rbp)
140a54c4d: 48 8d 0d fc d8 07 02        	leaq	0x207d8fc(%rip), %rcx   # 0x142ad2550
140a54c54: 48 89 8d d8 02 00 00        	movq	%rcx, 0x2d8(%rbp)
140a54c5b: 48 c7 85 e0 02 00 00 28 00 00 00    	movq	$0x28, 0x2e0(%rbp)
140a54c66: 48 8d 15 cb da 07 02        	leaq	0x207dacb(%rip), %rdx   # 0x142ad2738
140a54c6d: 48 89 95 f0 02 00 00        	movq	%rdx, 0x2f0(%rbp)
140a54c74: 48 c7 85 f8 02 00 00 02 00 00 00    	movq	$0x2, 0x2f8(%rbp)
140a54c7f: 48 8d 95 30 05 00 00        	leaq	0x530(%rbp), %rdx
140a54c86: 48 89 95 00 03 00 00        	movq	%rdx, 0x300(%rbp)
140a54c8d: 48 c7 85 08 03 00 00 01 00 00 00    	movq	$0x1, 0x308(%rbp)
140a54c98: 48 89 bd 10 03 00 00        	movq	%rdi, 0x310(%rbp)
140a54c9f: 48 c7 85 18 03 00 00 01 00 00 00    	movq	$0x1, 0x318(%rbp)
140a54caa: 48 c7 85 a0 02 00 00 00 00 00 00    	movq	$0x0, 0x2a0(%rbp)
140a54cb5: 48 89 8d a8 02 00 00        	movq	%rcx, 0x2a8(%rbp)
140a54cbc: 48 c7 85 b0 02 00 00 28 00 00 00    	movq	$0x28, 0x2b0(%rbp)
140a54cc7: 48 c7 85 b8 02 00 00 00 00 00 00    	movq	$0x0, 0x2b8(%rbp)
140a54cd2: f3 0f 7f 85 c0 02 00 00     	movdqu	%xmm0, 0x2c0(%rbp)
140a54cda: c7 85 e8 02 00 00 01 00 00 00       	movl	$0x1, 0x2e8(%rbp)
140a54ce4: 89 85 ec 02 00 00           	movl	%eax, 0x2ec(%rbp)
140a54cea: 48 8d 8d 20 05 00 00        	leaq	0x520(%rbp), %rcx
140a54cf1: 48 8d 95 a0 02 00 00        	leaq	0x2a0(%rbp), %rdx
140a54cf8: e8 33 56 2f 00              	callq	0x140d4a330 <.text+0xd49330>
140a54cfd: 41 80 f6 01                 	xorb	$0x1, %r14b
140a54d01: 45 0f b6 ce                 	movzbl	%r14b, %r9d
140a54d05: 41 c1 e1 02                 	shll	$0x2, %r9d
140a54d09: 48 c7 44 24 28 00 00 00 00  	movq	$0x0, 0x28(%rsp)
140a54d12: c7 44 24 20 04 00 00 00     	movl	$0x4, 0x20(%rsp)
140a54d1a: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
140a54d21: 48 8d 95 68 05 00 00        	leaq	0x568(%rbp), %rdx
140a54d28: 41 b8 00 04 00 00           	movl	$0x400, %r8d            # imm = 0x400
140a54d2e: e8 ad 7c b2 00              	callq	0x14157c9e0 <.text+0x157b9e0>
140a54d33: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
140a54d3a: 4c 39 e8                    	cmpq	%r13, %rax
140a54d3d: 0f 85 0f 04 00 00           	jne	0x140a55152 <.text+0xa54152>
140a54d43: 48 89 b5 78 05 00 00        	movq	%rsi, 0x578(%rbp)
140a54d4a: 48 8d 15 1f da 07 02        	leaq	0x207da1f(%rip), %rdx   # 0x142ad2770
140a54d51: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
140a54d58: 41 b8 16 00 00 00           	movl	$0x16, %r8d
140a54d5e: e8 1d 42 31 00              	callq	0x140d68f80 <.text+0xd67f80>
140a54d63: 48 8b bd a0 02 00 00        	movq	0x2a0(%rbp), %rdi
140a54d6a: 48 8b b5 a8 02 00 00        	movq	0x2a8(%rbp), %rsi
140a54d71: 48 83 ff 02                 	cmpq	$0x2, %rdi
140a54d75: 0f 84 3e 0c 00 00           	je	0x140a559b9 <.text+0xa549b9>
140a54d7b: 48 8d 95 b0 02 00 00        	leaq	0x2b0(%rbp), %rdx
140a54d82: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140a54d86: 41 b8 60 02 00 00           	movl	$0x260, %r8d            # imm = 0x260
140a54d8c: e8 6f ee ec 01              	callq	0x142923c00 <.text+0x2922c00>
140a54d91: 48 89 7d c0                 	movq	%rdi, -0x40(%rbp)
140a54d95: 48 89 75 c8                 	movq	%rsi, -0x38(%rbp)
140a54d99: c6 85 d7 05 00 00 01        	movb	$0x1, 0x5d7(%rbp)
140a54da0: 31 c9                       	xorl	%ecx, %ecx
140a54da2: e8 79 4d 68 ff              	callq	0x1400d9b20 <.text+0xd8b20>
140a54da7: 48 85 c0                    	testq	%rax, %rax
140a54daa: 48 8b b5 78 05 00 00        	movq	0x578(%rbp), %rsi
140a54db1: 0f 84 36 0c 00 00           	je	0x140a559ed <.text+0xa549ed>
140a54db7: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140a54dbb: 48 ff 00                    	incq	(%rax)
140a54dbe: 0f 10 0d 7b aa 20 02        	movups	0x220aa7b(%rip), %xmm1  # 0x142c5f840
140a54dc5: 0f 29 8d 40 05 00 00        	movaps	%xmm1, 0x540(%rbp)
140a54dcc: 0f 10 0d 5d aa 20 02        	movups	0x220aa5d(%rip), %xmm1  # 0x142c5f830
140a54dd3: 0f 29 8d 30 05 00 00        	movaps	%xmm1, 0x530(%rbp)
140a54dda: 66 0f 7f 85 50 05 00 00     	movdqa	%xmm0, 0x550(%rbp)
140a54de2: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
140a54de6: 41 b8 70 02 00 00           	movl	$0x270, %r8d            # imm = 0x270
140a54dec: 48 89 d9                    	movq	%rbx, %rcx
140a54def: e8 0c ee ec 01              	callq	0x142923c00 <.text+0x2922c00>
140a54df4: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
140a54dfb: 48 89 da                    	movq	%rbx, %rdx
140a54dfe: e8 cd d6 ac 00              	callq	0x1415224d0 <.text+0x15214d0>
140a54e03: 66 0f 6f 85 30 05 00 00     	movdqa	0x530(%rbp), %xmm0
140a54e0b: 0f 28 8d 40 05 00 00        	movaps	0x540(%rbp), %xmm1
140a54e12: 0f 28 95 50 05 00 00        	movaps	0x550(%rbp), %xmm2
140a54e19: 66 0f 7f 85 90 05 00 00     	movdqa	%xmm0, 0x590(%rbp)
140a54e21: 0f 29 8d a0 05 00 00        	movaps	%xmm1, 0x5a0(%rbp)
140a54e28: 0f 29 95 b0 05 00 00        	movaps	%xmm2, 0x5b0(%rbp)
140a54e2f: e8 ac e7 30 00              	callq	0x140d635e0 <.text+0xd625e0>
140a54e34: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
140a54e3b: 89 95 68 02 00 00           	movl	%edx, 0x268(%rbp)
140a54e41: 48 8d 05 70 d9 07 02        	leaq	0x207d970(%rip), %rax   # 0x142ad27b8
140a54e48: 48 89 85 a0 02 00 00        	movq	%rax, 0x2a0(%rbp)
140a54e4f: 48 c7 85 a8 02 00 00 13 00 00 00    	movq	$0x13, 0x2a8(%rbp)
140a54e5a: 48 8d 05 6a d9 07 02        	leaq	0x207d96a(%rip), %rax   # 0x142ad27cb
140a54e61: 48 89 85 b0 02 00 00        	movq	%rax, 0x2b0(%rbp)
140a54e68: 48 c7 85 b8 02 00 00 0b 00 00 00    	movq	$0xb, 0x2b8(%rbp)
140a54e73: 48 8d 05 5c d9 07 02        	leaq	0x207d95c(%rip), %rax   # 0x142ad27d6
140a54e7a: 48 89 85 c0 02 00 00        	movq	%rax, 0x2c0(%rbp)
140a54e81: 48 c7 85 c8 02 00 00 12 00 00 00    	movq	$0x12, 0x2c8(%rbp)
140a54e8c: 48 8d 05 55 d9 07 02        	leaq	0x207d955(%rip), %rax   # 0x142ad27e8
140a54e93: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
140a54e9a: 48 c7 85 d8 02 00 00 07 00 00 00    	movq	$0x7, 0x2d8(%rbp)
140a54ea5: 48 8d 05 43 d9 07 02        	leaq	0x207d943(%rip), %rax   # 0x142ad27ef
140a54eac: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
140a54eb3: 48 c7 85 e8 02 00 00 0e 00 00 00    	movq	$0xe, 0x2e8(%rbp)
140a54ebe: 31 c0                       	xorl	%eax, %eax
140a54ec0: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140a54ec4: 4c 89 bd 18 05 00 00        	movq	%r15, 0x518(%rbp)
140a54ecb: e9 78 03 00 00              	jmp	0x140a55248 <.text+0xa54248>
