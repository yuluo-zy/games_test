
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141c53530: 55                          	pushq	%rbp
141c53531: 41 57                       	pushq	%r15
141c53533: 41 56                       	pushq	%r14
141c53535: 41 55                       	pushq	%r13
141c53537: 41 54                       	pushq	%r12
141c53539: 56                          	pushq	%rsi
141c5353a: 57                          	pushq	%rdi
141c5353b: 53                          	pushq	%rbx
141c5353c: 48 81 ec e8 02 00 00        	subq	$0x2e8, %rsp            # imm = 0x2E8
141c53543: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141c5354b: 48 c7 85 60 02 00 00 fe ff ff ff    	movq	$-0x2, 0x260(%rbp)
141c53556: 49 89 d6                    	movq	%rdx, %r14
141c53559: 48 8b b5 d0 02 00 00        	movq	0x2d0(%rbp), %rsi
141c53560: 4c 8b 3a                    	movq	(%rdx), %r15
141c53563: 48 8b 5e 08                 	movq	0x8(%rsi), %rbx
141c53567: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
141c5356b: 4c 89 f9                    	movq	%r15, %rcx
141c5356e: 48 89 da                    	movq	%rbx, %rdx
141c53571: 49 89 f8                    	movq	%rdi, %r8
141c53574: e8 f7 d7 0e ff              	callq	0x140d40d70 <.text+0xd3fd70>
141c53579: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
141c53584: 48 c7 85 38 02 00 00 08 00 00 00    	movq	$0x8, 0x238(%rbp)
141c5358f: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
141c5359a: 49 83 7f 30 00              	cmpq	$0x0, 0x30(%r15)
141c5359f: 48 b8 26 79 69 2d 6e d0 43 f7       	movabsq	$-0x8bc2f91d29686da, %rax # imm = 0xF743D06E2D697926
141c535a9: 48 b9 00 a4 bd f6 7f b5 d6 df       	movabsq	$-0x20294a8009425c00, %rcx # imm = 0xDFD6B57FF6BDA400
141c535b3: 0f 84 9a 00 00 00           	je	0x141c53653 <.text+0x1c52653>
141c535b9: 49 8b 57 18                 	movq	0x18(%r15), %rdx
141c535bd: 4d 8b 47 20                 	movq	0x20(%r15), %r8
141c535c1: 4d 89 c1                    	movq	%r8, %r9
141c535c4: 49 21 c1                    	andq	%rax, %r9
141c535c7: 45 31 d2                    	xorl	%r10d, %r10d
141c535ca: 66 0f 6f 05 5e ce cf 00     	movdqa	0xcfce5e(%rip), %xmm0   # 0x142950430
141c535d2: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
141c535d6: f3 42 0f 6f 14 0a           	movdqu	(%rdx,%r9), %xmm2
141c535dc: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
141c535e0: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
141c535e4: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
141c535e9: 45 85 db                    	testl	%r11d, %r11d
141c535ec: 75 2f                       	jne	0x141c5361d <.text+0x1c5261d>
141c535ee: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
141c535f2: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
141c535f7: 45 85 db                    	testl	%r11d, %r11d
141c535fa: 75 57                       	jne	0x141c53653 <.text+0x1c52653>
141c535fc: 4d 01 d1                    	addq	%r10, %r9
141c535ff: 49 83 c1 10                 	addq	$0x10, %r9
141c53603: 49 83 c2 10                 	addq	$0x10, %r10
141c53607: 4d 21 c1                    	andq	%r8, %r9
141c5360a: eb ca                       	jmp	0x141c535d6 <.text+0x1c525d6>
141c5360c: 0f 1f 40 00                 	nopl	(%rax)
141c53610: 45 8d 63 ff                 	leal	-0x1(%r11), %r12d
141c53614: 66 45 21 dc                 	andw	%r11w, %r12w
141c53618: 45 89 e3                    	movl	%r12d, %r11d
141c5361b: 74 d1                       	je	0x141c535ee <.text+0x1c525ee>
141c5361d: f3 45 0f bc e3              	tzcntl	%r11d, %r12d
141c53622: 4d 01 cc                    	addq	%r9, %r12
141c53625: 4d 21 c4                    	andq	%r8, %r12
141c53628: 49 f7 dc                    	negq	%r12
141c5362b: 4f 8d 24 64                 	leaq	(%r12,%r12,2), %r12
141c5362f: 4a 39 4c e2 e8              	cmpq	%rcx, -0x18(%rdx,%r12,8)
141c53634: 75 da                       	jne	0x141c53610 <.text+0x1c52610>
141c53636: 4a 39 44 e2 f0              	cmpq	%rax, -0x10(%rdx,%r12,8)
141c5363b: 75 d3                       	jne	0x141c53610 <.text+0x1c52610>
141c5363d: 4a 8d 04 e2                 	leaq	(%rdx,%r12,8), %rax
141c53641: 4c 8b 60 f8                 	movq	-0x8(%rax), %r12
141c53645: 48 3b 3e                    	cmpq	(%rsi), %rdi
141c53648: 0f 84 52 03 00 00           	je	0x141c539a0 <.text+0x1c529a0>
141c5364e: e9 f6 03 00 00              	jmp	0x141c53a49 <.text+0x1c52a49>
141c53653: 48 89 9d 48 02 00 00        	movq	%rbx, 0x248(%rbp)
141c5365a: 41 0f b6 57 60              	movzbl	0x60(%r15), %edx
141c5365f: 49 8b 97 80 00 00 00        	movq	0x80(%r15), %rdx
141c53666: 4d 8b 8f 88 00 00 00        	movq	0x88(%r15), %r9
141c5366d: 4d 89 ca                    	movq	%r9, %r10
141c53670: 49 21 c2                    	andq	%rax, %r10
141c53673: 45 31 db                    	xorl	%r11d, %r11d
141c53676: 66 0f 6f 05 b2 cd cf 00     	movdqa	0xcfcdb2(%rip), %xmm0   # 0x142950430
141c5367e: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
141c53682: f3 42 0f 6f 14 12           	movdqu	(%rdx,%r10), %xmm2
141c53688: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
141c5368c: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
141c53690: 66 44 0f d7 e3              	pmovmskb	%xmm3, %r12d
141c53695: 45 85 e4                    	testl	%r12d, %r12d
141c53698: 75 34                       	jne	0x141c536ce <.text+0x1c526ce>
141c5369a: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
141c5369e: 66 44 0f d7 c2              	pmovmskb	%xmm2, %r8d
141c536a3: 45 85 c0                    	testl	%r8d, %r8d
141c536a6: 0f 85 f2 00 00 00           	jne	0x141c5379e <.text+0x1c5279e>
141c536ac: 4d 01 da                    	addq	%r11, %r10
141c536af: 49 83 c2 10                 	addq	$0x10, %r10
141c536b3: 49 83 c3 10                 	addq	$0x10, %r11
141c536b7: 4d 21 ca                    	andq	%r9, %r10
141c536ba: eb c6                       	jmp	0x141c53682 <.text+0x1c52682>
141c536bc: 0f 1f 40 00                 	nopl	(%rax)
141c536c0: 45 8d 44 24 ff              	leal	-0x1(%r12), %r8d
141c536c5: 66 45 21 e0                 	andw	%r12w, %r8w
141c536c9: 45 89 c4                    	movl	%r8d, %r12d
141c536cc: 74 cc                       	je	0x141c5369a <.text+0x1c5269a>
141c536ce: f3 45 0f bc ec              	tzcntl	%r12d, %r13d
141c536d3: 4d 01 d5                    	addq	%r10, %r13
141c536d6: 4d 21 cd                    	andq	%r9, %r13
141c536d9: 4d 89 e8                    	movq	%r13, %r8
141c536dc: 49 f7 d8                    	negq	%r8
141c536df: 4a 8d 1c ed 00 00 00 00     	leaq	(,%r13,8), %rbx
141c536e7: 49 c1 e0 07                 	shlq	$0x7, %r8
141c536eb: 49 29 d8                    	subq	%rbx, %r8
141c536ee: 4a 39 8c 02 78 ff ff ff     	cmpq	%rcx, -0x88(%rdx,%r8)
141c536f6: 75 c8                       	jne	0x141c536c0 <.text+0x1c526c0>
141c536f8: 4a 39 44 02 80              	cmpq	%rax, -0x80(%rdx,%r8)
141c536fd: 75 c1                       	jne	0x141c536c0 <.text+0x1c526c0>
141c536ff: 4d 89 ea                    	movq	%r13, %r10
141c53702: 49 c1 e2 07                 	shlq	$0x7, %r10
141c53706: 4f 8d 1c ea                 	leaq	(%r10,%r13,8), %r11
141c5370a: 49 c1 fb 03                 	sarq	$0x3, %r11
141c5370e: 49 ba f1 f0 f0 f0 f0 f0 f0 f0       	movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
141c53718: 4d 0f af d3                 	imulq	%r11, %r10
141c5371c: 4d 8d 5a f0                 	leaq	-0x10(%r10), %r11
141c53720: 4d 21 cb                    	andq	%r9, %r11
141c53723: f3 42 0f 6f 04 1a           	movdqu	(%rdx,%r11), %xmm0
141c53729: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
141c5372d: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
141c53731: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
141c53736: f3 42 0f 6f 04 12           	movdqu	(%rdx,%r10), %xmm0
141c5373c: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
141c53740: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
141c53745: 45 85 e4                    	testl	%r12d, %r12d
141c53748: 74 0b                       	je	0x141c53755 <.text+0x1c52755>
141c5374a: 66 45 0f bd e4              	bsrw	%r12w, %r12w
141c5374f: 41 83 f4 0f                 	xorl	$0xf, %r12d
141c53753: eb 05                       	jmp	0x141c5375a <.text+0x1c5275a>
141c53755: 66 41 bc 10 00              	movw	$0x10, %r12w
141c5375a: 49 01 d0                    	addq	%rdx, %r8
141c5375d: 41 81 c9 00 00 01 00        	orl	$0x10000, %r9d          # imm = 0x10000
141c53764: f3 45 0f bc c9              	tzcntl	%r9d, %r9d
141c53769: 45 01 cc                    	addl	%r9d, %r12d
141c5376c: 41 b1 80                    	movb	$-0x80, %r9b
141c5376f: 66 41 83 fc 0f              	cmpw	$0xf, %r12w
141c53774: 77 0a                       	ja	0x141c53780 <.text+0x1c52780>
141c53776: 49 ff 87 90 00 00 00        	incq	0x90(%r15)
141c5377d: 41 b1 ff                    	movb	$-0x1, %r9b
141c53780: 46 88 0c 12                 	movb	%r9b, (%rdx,%r10)
141c53784: 46 88 4c 1a 10              	movb	%r9b, 0x10(%rdx,%r11)
141c53789: 49 ff 8f 98 00 00 00        	decq	0x98(%r15)
141c53790: 49 8b 50 88                 	movq	-0x78(%r8), %rdx
141c53794: 48 83 fa 02                 	cmpq	$0x2, %rdx
141c53798: 0f 85 26 02 00 00           	jne	0x141c539c4 <.text+0x1c529c4>
141c5379e: 49 8b 56 08                 	movq	0x8(%r14), %rdx
141c537a2: 4c 8b 22                    	movq	(%rdx), %r12
141c537a5: 4d 8d 6c 24 01              	leaq	0x1(%r12), %r13
141c537aa: 4c 89 2a                    	movq	%r13, (%rdx)
141c537ad: 48 ba 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
141c537b7: 48 89 95 08 02 00 00        	movq	%rdx, 0x208(%rbp)
141c537be: 48 8d 15 ab 29 2c 01        	leaq	0x12c29ab(%rip), %rdx   # 0x142f16170
141c537c5: 48 89 95 10 02 00 00        	movq	%rdx, 0x210(%rbp)
141c537cc: 48 c7 85 18 02 00 00 35 00 00 00    	movq	$0x35, 0x218(%rbp)
141c537d7: 66 c7 85 28 02 00 00 00 01  	movw	$0x100, 0x228(%rbp)     # imm = 0x100
141c537e0: 48 c7 85 d0 01 00 00 01 00 00 00    	movq	$0x1, 0x1d0(%rbp)
141c537eb: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
141c537f2: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
141c537f9: 48 c7 85 f8 01 00 00 04 00 00 00    	movq	$0x4, 0x1f8(%rbp)
141c53804: 48 c7 85 00 02 00 00 0c 00 00 00    	movq	$0xc, 0x200(%rbp)
141c5380f: 48 c7 85 20 02 00 00 00 00 00 00    	movq	$0x0, 0x220(%rbp)
141c5381a: c6 85 2a 02 00 00 01        	movb	$0x1, 0x22a(%rbp)
141c53821: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
141c5382c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
141c53833: 4c 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %r8
141c5383a: 4c 89 e2                    	movq	%r12, %rdx
141c5383d: e8 2e c0 0e ff              	callq	0x140d3f870 <.text+0xd3e870>
141c53842: 4d 39 6f 10                 	cmpq	%r13, 0x10(%r15)
141c53846: 73 19                       	jae	0x141c53861 <.text+0x1c52861>
141c53848: c6 85 5f 02 00 00 01        	movb	$0x1, 0x25f(%rbp)
141c5384f: 4c 8d 05 1a 0c 2c 01        	leaq	0x12c0c1a(%rip), %r8    # 0x142f14470
141c53856: 4c 89 f9                    	movq	%r15, %rcx
141c53859: 4c 89 ea                    	movq	%r13, %rdx
141c5385c: e8 0f 4b 00 00              	callq	0x141c58370 <.text+0x1c57370>
141c53861: 49 8b 5f 08                 	movq	0x8(%r15), %rbx
141c53865: 4d 69 fc d0 00 00 00        	imulq	$0xd0, %r12, %r15
141c5386c: 4a 8d 04 3b                 	leaq	(%rbx,%r15), %rax
141c53870: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
141c53877: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141c5387b: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
141c53882: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
141c53888: e8 73 03 cd 00              	callq	0x142923c00 <.text+0x2922c00>
141c5388d: 42 83 3c 3b 02              	cmpl	$0x2, (%rbx,%r15)
141c53892: 74 0c                       	je	0x141c538a0 <.text+0x1c528a0>
141c53894: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
141c5389b: e8 b0 d2 99 fe              	callq	0x1405f0b50 <.text+0x5efb50>
141c538a0: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
141c538a4: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
141c538aa: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
141c538b1: e8 4a 03 cd 00              	callq	0x142923c00 <.text+0x2922c00>
141c538b6: 49 8b 0e                    	movq	(%r14), %rcx
141c538b9: 48 83 c1 18                 	addq	$0x18, %rcx
141c538bd: 48 ba 00 a4 bd f6 7f b5 d6 df       	movabsq	$-0x20294a8009425c00, %rdx # imm = 0xDFD6B57FF6BDA400
141c538c7: 49 b8 26 79 69 2d 6e d0 43 f7       	movabsq	$-0x8bc2f91d29686da, %r8 # imm = 0xF743D06E2D697926
141c538d1: 4d 89 e1                    	movq	%r12, %r9
141c538d4: e8 47 f3 42 fe              	callq	0x140082c20 <.text+0x81c20>
141c538d9: 0f 10 05 20 0b 2c 01        	movups	0x12c0b20(%rip), %xmm0  # 0x142f14400
141c538e0: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
141c538e7: f3 0f 6f 05 01 0b 2c 01     	movdqu	0x12c0b01(%rip), %xmm0  # 0x142f143f0
141c538ef: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
141c538f7: c6 85 5e 02 00 00 01        	movb	$0x1, 0x25e(%rbp)
141c538fe: 48 8d 85 30 02 00 00        	leaq	0x230(%rbp), %rax
141c53905: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141c5390a: 4c 89 f2                    	movq	%r14, %rdx
141c5390d: e8 1e fc ff ff              	callq	0x141c53530 <.text+0x1c52530>
141c53912: 49 8b 06                    	movq	(%r14), %rax
141c53915: 48 8b 40 08                 	movq	0x8(%rax), %rax
141c53919: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
141c53920: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
141c53927: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
141c5392b: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
141c5392f: 4a 8d 4c 38 60              	leaq	0x60(%rax,%r15), %rcx
141c53934: 4a 8d 94 38 80 00 00 00     	leaq	0x80(%rax,%r15), %rdx
141c5393c: 41 b8 20 00 00 00           	movl	$0x20, %r8d
141c53942: 41 b9 10 00 00 00           	movl	$0x10, %r9d
141c53948: 48 89 8d 50 02 00 00        	movq	%rcx, 0x250(%rbp)
141c5394f: e8 ec d9 3f fe              	callq	0x140051340 <.text+0x50340>
141c53954: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
141c5395b: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
141c53960: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
141c53965: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
141c5396c: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
141c53971: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
141c53975: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
141c5397c: 48 85 d2                    	testq	%rdx, %rdx
141c5397f: 74 16                       	je	0x141c53997 <.text+0x1c52997>
141c53981: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
141c53988: 48 c1 e2 03                 	shlq	$0x3, %rdx
141c5398c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141c53992: e8 89 02 9c fe              	callq	0x140613c20 <.text+0x612c20>
141c53997: 48 3b 3e                    	cmpq	(%rsi), %rdi
141c5399a: 0f 85 a9 00 00 00           	jne	0x141c53a49 <.text+0x1c52a49>
141c539a0: 48 8d 15 11 0e 2c 01        	leaq	0x12c0e11(%rip), %rdx   # 0x142f147b8
141c539a7: 48 89 f1                    	movq	%rsi, %rcx
141c539aa: e8 41 e4 f2 fe              	callq	0x140b81df0 <.text+0xb80df0>
141c539af: 48 8b 46 08                 	movq	0x8(%rsi), %rax
141c539b3: 4c 89 24 f8                 	movq	%r12, (%rax,%rdi,8)
141c539b7: 48 8d 47 01                 	leaq	0x1(%rdi), %rax
141c539bb: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
141c539bf: e9 95 00 00 00              	jmp	0x141c53a59 <.text+0x1c52a59>
141c539c4: 41 0f 10 40 f0              	movups	-0x10(%r8), %xmm0
141c539c9: 0f 11 85 c0 01 00 00        	movups	%xmm0, 0x1c0(%rbp)
141c539d0: 41 0f 10 40 e0              	movups	-0x20(%r8), %xmm0
141c539d5: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
141c539dc: 41 0f 10 40 d0              	movups	-0x30(%r8), %xmm0
141c539e1: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
141c539e8: f3 41 0f 6f 40 90           	movdqu	-0x70(%r8), %xmm0
141c539ee: f3 41 0f 6f 48 a0           	movdqu	-0x60(%r8), %xmm1
141c539f4: f3 41 0f 6f 50 b0           	movdqu	-0x50(%r8), %xmm2
141c539fa: f3 41 0f 6f 58 c0           	movdqu	-0x40(%r8), %xmm3
141c53a00: f3 0f 7f 9d 90 01 00 00     	movdqu	%xmm3, 0x190(%rbp)
141c53a08: f3 0f 7f 95 80 01 00 00     	movdqu	%xmm2, 0x180(%rbp)
141c53a10: f3 0f 7f 8d 70 01 00 00     	movdqu	%xmm1, 0x170(%rbp)
141c53a18: f3 0f 7f 85 60 01 00 00     	movdqu	%xmm0, 0x160(%rbp)
141c53a20: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
141c53a27: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
141c53a2e: 4c 89 f2                    	movq	%r14, %rdx
141c53a31: e8 0a c0 0e ff              	callq	0x140d3fa40 <.text+0xd3ea40>
141c53a36: 49 89 c4                    	movq	%rax, %r12
141c53a39: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
141c53a40: 48 3b 3e                    	cmpq	(%rsi), %rdi
141c53a43: 0f 84 57 ff ff ff           	je	0x141c539a0 <.text+0x1c529a0>
141c53a49: 4c 89 24 fb                 	movq	%r12, (%rbx,%rdi,8)
141c53a4d: 48 89 f8                    	movq	%rdi, %rax
141c53a50: 48 ff c0                    	incq	%rax
141c53a53: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
141c53a57: 74 04                       	je	0x141c53a5d <.text+0x1c52a5d>
141c53a59: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
141c53a5d: 48 81 c4 e8 02 00 00        	addq	$0x2e8, %rsp            # imm = 0x2E8
141c53a64: 5b                          	popq	%rbx
141c53a65: 5f                          	popq	%rdi
141c53a66: 5e                          	popq	%rsi
141c53a67: 41 5c                       	popq	%r12
