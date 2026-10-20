
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14084e100: 55                          	pushq	%rbp
14084e101: 41 57                       	pushq	%r15
14084e103: 41 56                       	pushq	%r14
14084e105: 41 55                       	pushq	%r13
14084e107: 41 54                       	pushq	%r12
14084e109: 56                          	pushq	%rsi
14084e10a: 57                          	pushq	%rdi
14084e10b: 53                          	pushq	%rbx
14084e10c: 48 81 ec e8 02 00 00        	subq	$0x2e8, %rsp            # imm = 0x2E8
14084e113: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14084e11b: 48 c7 85 60 02 00 00 fe ff ff ff    	movq	$-0x2, 0x260(%rbp)
14084e126: 49 89 d6                    	movq	%rdx, %r14
14084e129: 48 8b b5 d0 02 00 00        	movq	0x2d0(%rbp), %rsi
14084e130: 4c 8b 3a                    	movq	(%rdx), %r15
14084e133: 48 8b 5e 08                 	movq	0x8(%rsi), %rbx
14084e137: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
14084e13b: 4c 89 f9                    	movq	%r15, %rcx
14084e13e: 48 89 da                    	movq	%rbx, %rdx
14084e141: 49 89 f8                    	movq	%rdi, %r8
14084e144: e8 27 2c 4f 00              	callq	0x140d40d70 <.text+0xd3fd70>
14084e149: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
14084e154: 48 c7 85 38 02 00 00 08 00 00 00    	movq	$0x8, 0x238(%rbp)
14084e15f: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
14084e16a: 49 83 7f 30 00              	cmpq	$0x0, 0x30(%r15)
14084e16f: 48 b8 3f 87 6c 65 76 f6 e6 dd       	movabsq	$-0x221909899a9378c1, %rax # imm = 0xDDE6F676656C873F
14084e179: 48 b9 cd f1 10 71 77 b1 72 60       	movabsq	$0x6072b1777110f1cd, %rcx # imm = 0x6072B1777110F1CD
14084e183: 0f 84 9a 00 00 00           	je	0x14084e223 <.text+0x84d223>
14084e189: 49 8b 57 18                 	movq	0x18(%r15), %rdx
14084e18d: 4d 8b 47 20                 	movq	0x20(%r15), %r8
14084e191: 4d 89 c1                    	movq	%r8, %r9
14084e194: 49 21 c1                    	andq	%rax, %r9
14084e197: 45 31 d2                    	xorl	%r10d, %r10d
14084e19a: 66 0f 6f 05 1e 34 11 02     	movdqa	0x211341e(%rip), %xmm0  # 0x1429615c0
14084e1a2: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14084e1a6: f3 42 0f 6f 14 0a           	movdqu	(%rdx,%r9), %xmm2
14084e1ac: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14084e1b0: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14084e1b4: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
14084e1b9: 45 85 db                    	testl	%r11d, %r11d
14084e1bc: 75 2f                       	jne	0x14084e1ed <.text+0x84d1ed>
14084e1be: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14084e1c2: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14084e1c7: 45 85 db                    	testl	%r11d, %r11d
14084e1ca: 75 57                       	jne	0x14084e223 <.text+0x84d223>
14084e1cc: 4d 01 d1                    	addq	%r10, %r9
14084e1cf: 49 83 c1 10                 	addq	$0x10, %r9
14084e1d3: 49 83 c2 10                 	addq	$0x10, %r10
14084e1d7: 4d 21 c1                    	andq	%r8, %r9
14084e1da: eb ca                       	jmp	0x14084e1a6 <.text+0x84d1a6>
14084e1dc: 0f 1f 40 00                 	nopl	(%rax)
14084e1e0: 45 8d 63 ff                 	leal	-0x1(%r11), %r12d
14084e1e4: 66 45 21 dc                 	andw	%r11w, %r12w
14084e1e8: 45 89 e3                    	movl	%r12d, %r11d
14084e1eb: 74 d1                       	je	0x14084e1be <.text+0x84d1be>
14084e1ed: f3 45 0f bc e3              	tzcntl	%r11d, %r12d
14084e1f2: 4d 01 cc                    	addq	%r9, %r12
14084e1f5: 4d 21 c4                    	andq	%r8, %r12
14084e1f8: 49 f7 dc                    	negq	%r12
14084e1fb: 4f 8d 24 64                 	leaq	(%r12,%r12,2), %r12
14084e1ff: 4a 39 4c e2 e8              	cmpq	%rcx, -0x18(%rdx,%r12,8)
14084e204: 75 da                       	jne	0x14084e1e0 <.text+0x84d1e0>
14084e206: 4a 39 44 e2 f0              	cmpq	%rax, -0x10(%rdx,%r12,8)
14084e20b: 75 d3                       	jne	0x14084e1e0 <.text+0x84d1e0>
14084e20d: 4a 8d 04 e2                 	leaq	(%rdx,%r12,8), %rax
14084e211: 4c 8b 60 f8                 	movq	-0x8(%rax), %r12
14084e215: 48 3b 3e                    	cmpq	(%rsi), %rdi
14084e218: 0f 84 52 03 00 00           	je	0x14084e570 <.text+0x84d570>
14084e21e: e9 f6 03 00 00              	jmp	0x14084e619 <.text+0x84d619>
14084e223: 48 89 9d 48 02 00 00        	movq	%rbx, 0x248(%rbp)
14084e22a: 41 0f b6 57 60              	movzbl	0x60(%r15), %edx
14084e22f: 49 8b 97 80 00 00 00        	movq	0x80(%r15), %rdx
14084e236: 4d 8b 8f 88 00 00 00        	movq	0x88(%r15), %r9
14084e23d: 4d 89 ca                    	movq	%r9, %r10
14084e240: 49 21 c2                    	andq	%rax, %r10
14084e243: 45 31 db                    	xorl	%r11d, %r11d
14084e246: 66 0f 6f 05 72 33 11 02     	movdqa	0x2113372(%rip), %xmm0  # 0x1429615c0
14084e24e: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14084e252: f3 42 0f 6f 14 12           	movdqu	(%rdx,%r10), %xmm2
14084e258: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14084e25c: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14084e260: 66 44 0f d7 e3              	pmovmskb	%xmm3, %r12d
14084e265: 45 85 e4                    	testl	%r12d, %r12d
14084e268: 75 34                       	jne	0x14084e29e <.text+0x84d29e>
14084e26a: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14084e26e: 66 44 0f d7 c2              	pmovmskb	%xmm2, %r8d
14084e273: 45 85 c0                    	testl	%r8d, %r8d
14084e276: 0f 85 f2 00 00 00           	jne	0x14084e36e <.text+0x84d36e>
14084e27c: 4d 01 da                    	addq	%r11, %r10
14084e27f: 49 83 c2 10                 	addq	$0x10, %r10
14084e283: 49 83 c3 10                 	addq	$0x10, %r11
14084e287: 4d 21 ca                    	andq	%r9, %r10
14084e28a: eb c6                       	jmp	0x14084e252 <.text+0x84d252>
14084e28c: 0f 1f 40 00                 	nopl	(%rax)
14084e290: 45 8d 44 24 ff              	leal	-0x1(%r12), %r8d
14084e295: 66 45 21 e0                 	andw	%r12w, %r8w
14084e299: 45 89 c4                    	movl	%r8d, %r12d
14084e29c: 74 cc                       	je	0x14084e26a <.text+0x84d26a>
14084e29e: f3 45 0f bc ec              	tzcntl	%r12d, %r13d
14084e2a3: 4d 01 d5                    	addq	%r10, %r13
14084e2a6: 4d 21 cd                    	andq	%r9, %r13
14084e2a9: 4d 89 e8                    	movq	%r13, %r8
14084e2ac: 49 f7 d8                    	negq	%r8
14084e2af: 4a 8d 1c ed 00 00 00 00     	leaq	(,%r13,8), %rbx
14084e2b7: 49 c1 e0 07                 	shlq	$0x7, %r8
14084e2bb: 49 29 d8                    	subq	%rbx, %r8
14084e2be: 4a 39 8c 02 78 ff ff ff     	cmpq	%rcx, -0x88(%rdx,%r8)
14084e2c6: 75 c8                       	jne	0x14084e290 <.text+0x84d290>
14084e2c8: 4a 39 44 02 80              	cmpq	%rax, -0x80(%rdx,%r8)
14084e2cd: 75 c1                       	jne	0x14084e290 <.text+0x84d290>
14084e2cf: 4d 89 ea                    	movq	%r13, %r10
14084e2d2: 49 c1 e2 07                 	shlq	$0x7, %r10
14084e2d6: 4f 8d 1c ea                 	leaq	(%r10,%r13,8), %r11
14084e2da: 49 c1 fb 03                 	sarq	$0x3, %r11
14084e2de: 49 ba f1 f0 f0 f0 f0 f0 f0 f0       	movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
14084e2e8: 4d 0f af d3                 	imulq	%r11, %r10
14084e2ec: 4d 8d 5a f0                 	leaq	-0x10(%r10), %r11
14084e2f0: 4d 21 cb                    	andq	%r9, %r11
14084e2f3: f3 42 0f 6f 04 1a           	movdqu	(%rdx,%r11), %xmm0
14084e2f9: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14084e2fd: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
14084e301: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
14084e306: f3 42 0f 6f 04 12           	movdqu	(%rdx,%r10), %xmm0
14084e30c: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
14084e310: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
14084e315: 45 85 e4                    	testl	%r12d, %r12d
14084e318: 74 0b                       	je	0x14084e325 <.text+0x84d325>
14084e31a: 66 45 0f bd e4              	bsrw	%r12w, %r12w
14084e31f: 41 83 f4 0f                 	xorl	$0xf, %r12d
14084e323: eb 05                       	jmp	0x14084e32a <.text+0x84d32a>
14084e325: 66 41 bc 10 00              	movw	$0x10, %r12w
14084e32a: 49 01 d0                    	addq	%rdx, %r8
14084e32d: 41 81 c9 00 00 01 00        	orl	$0x10000, %r9d          # imm = 0x10000
14084e334: f3 45 0f bc c9              	tzcntl	%r9d, %r9d
14084e339: 45 01 cc                    	addl	%r9d, %r12d
14084e33c: 41 b1 80                    	movb	$-0x80, %r9b
14084e33f: 66 41 83 fc 0f              	cmpw	$0xf, %r12w
14084e344: 77 0a                       	ja	0x14084e350 <.text+0x84d350>
14084e346: 49 ff 87 90 00 00 00        	incq	0x90(%r15)
14084e34d: 41 b1 ff                    	movb	$-0x1, %r9b
14084e350: 46 88 0c 12                 	movb	%r9b, (%rdx,%r10)
14084e354: 46 88 4c 1a 10              	movb	%r9b, 0x10(%rdx,%r11)
14084e359: 49 ff 8f 98 00 00 00        	decq	0x98(%r15)
14084e360: 49 8b 50 88                 	movq	-0x78(%r8), %rdx
14084e364: 48 83 fa 02                 	cmpq	$0x2, %rdx
14084e368: 0f 85 26 02 00 00           	jne	0x14084e594 <.text+0x84d594>
14084e36e: 49 8b 56 08                 	movq	0x8(%r14), %rdx
14084e372: 4c 8b 22                    	movq	(%rdx), %r12
14084e375: 4d 8d 6c 24 01              	leaq	0x1(%r12), %r13
14084e37a: 4c 89 2a                    	movq	%r13, (%rdx)
14084e37d: 48 ba 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
14084e387: 48 89 95 08 02 00 00        	movq	%rdx, 0x208(%rbp)
14084e38e: 48 8d 15 56 83 3e 02        	leaq	0x23e8356(%rip), %rdx   # 0x142c366eb
14084e395: 48 89 95 10 02 00 00        	movq	%rdx, 0x210(%rbp)
14084e39c: 48 c7 85 18 02 00 00 24 00 00 00    	movq	$0x24, 0x218(%rbp)
14084e3a7: 66 c7 85 28 02 00 00 00 01  	movw	$0x100, 0x228(%rbp)     # imm = 0x100
14084e3b0: 48 c7 85 d0 01 00 00 01 00 00 00    	movq	$0x1, 0x1d0(%rbp)
14084e3bb: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
14084e3c2: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
14084e3c9: 48 c7 85 f8 01 00 00 01 00 00 00    	movq	$0x1, 0x1f8(%rbp)
14084e3d4: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
14084e3df: 48 c7 85 20 02 00 00 00 00 00 00    	movq	$0x0, 0x220(%rbp)
14084e3ea: c6 85 2a 02 00 00 01        	movb	$0x1, 0x22a(%rbp)
14084e3f1: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
14084e3fc: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14084e403: 4c 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %r8
14084e40a: 4c 89 e2                    	movq	%r12, %rdx
14084e40d: e8 5e 14 4f 00              	callq	0x140d3f870 <.text+0xd3e870>
14084e412: 4d 39 6f 10                 	cmpq	%r13, 0x10(%r15)
14084e416: 73 19                       	jae	0x14084e431 <.text+0x84d431>
14084e418: c6 85 5f 02 00 00 01        	movb	$0x1, 0x25f(%rbp)
14084e41f: 4c 8d 05 4a 60 24 02        	leaq	0x224604a(%rip), %r8    # 0x142a94470
14084e426: 4c 89 f9                    	movq	%r15, %rcx
14084e429: 4c 89 ea                    	movq	%r13, %rdx
14084e42c: e8 0f 5f aa 00              	callq	0x1412f4340 <.text+0x12f3340>
14084e431: 49 8b 5f 08                 	movq	0x8(%r15), %rbx
14084e435: 4d 69 fc d0 00 00 00        	imulq	$0xd0, %r12, %r15
14084e43c: 4a 8d 04 3b                 	leaq	(%rbx,%r15), %rax
14084e440: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
14084e447: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14084e44b: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
14084e452: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
14084e458: e8 a3 57 0d 02              	callq	0x142923c00 <.text+0x2922c00>
14084e45d: 42 83 3c 3b 02              	cmpl	$0x2, (%rbx,%r15)
14084e462: 74 0c                       	je	0x14084e470 <.text+0x84d470>
14084e464: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
14084e46b: e8 e0 26 da ff              	callq	0x1405f0b50 <.text+0x5efb50>
14084e470: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14084e474: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
14084e47a: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
14084e481: e8 7a 57 0d 02              	callq	0x142923c00 <.text+0x2922c00>
14084e486: 49 8b 0e                    	movq	(%r14), %rcx
14084e489: 48 83 c1 18                 	addq	$0x18, %rcx
14084e48d: 48 ba cd f1 10 71 77 b1 72 60       	movabsq	$0x6072b1777110f1cd, %rdx # imm = 0x6072B1777110F1CD
14084e497: 49 b8 3f 87 6c 65 76 f6 e6 dd       	movabsq	$-0x221909899a9378c1, %r8 # imm = 0xDDE6F676656C873F
14084e4a1: 4d 89 e1                    	movq	%r12, %r9
14084e4a4: e8 77 47 83 ff              	callq	0x140082c20 <.text+0x81c20>
14084e4a9: 0f 10 05 50 5f 24 02        	movups	0x2245f50(%rip), %xmm0  # 0x142a94400
14084e4b0: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
14084e4b7: f3 0f 6f 05 31 5f 24 02     	movdqu	0x2245f31(%rip), %xmm0  # 0x142a943f0
14084e4bf: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
14084e4c7: c6 85 5e 02 00 00 01        	movb	$0x1, 0x25e(%rbp)
14084e4ce: 48 8d 85 30 02 00 00        	leaq	0x230(%rbp), %rax
14084e4d5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14084e4da: 4c 89 f2                    	movq	%r14, %rdx
14084e4dd: e8 1e fc ff ff              	callq	0x14084e100 <.text+0x84d100>
14084e4e2: 49 8b 06                    	movq	(%r14), %rax
14084e4e5: 48 8b 40 08                 	movq	0x8(%rax), %rax
14084e4e9: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
14084e4f0: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
14084e4f7: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
14084e4fb: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
14084e4ff: 4a 8d 4c 38 60              	leaq	0x60(%rax,%r15), %rcx
14084e504: 4a 8d 94 38 80 00 00 00     	leaq	0x80(%rax,%r15), %rdx
14084e50c: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14084e512: 41 b9 10 00 00 00           	movl	$0x10, %r9d
14084e518: 48 89 8d 50 02 00 00        	movq	%rcx, 0x250(%rbp)
14084e51f: e8 1c 2e 80 ff              	callq	0x140051340 <.text+0x50340>
14084e524: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
14084e52b: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
14084e530: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
14084e535: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
14084e53c: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
14084e541: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14084e545: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
14084e54c: 48 85 d2                    	testq	%rdx, %rdx
14084e54f: 74 16                       	je	0x14084e567 <.text+0x84d567>
14084e551: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
14084e558: 48 c1 e2 03                 	shlq	$0x3, %rdx
14084e55c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14084e562: e8 b9 56 dc ff              	callq	0x140613c20 <.text+0x612c20>
14084e567: 48 3b 3e                    	cmpq	(%rsi), %rdi
14084e56a: 0f 85 a9 00 00 00           	jne	0x14084e619 <.text+0x84d619>
14084e570: 48 8d 15 39 61 24 02        	leaq	0x2246139(%rip), %rdx   # 0x142a946b0
14084e577: 48 89 f1                    	movq	%rsi, %rcx
14084e57a: e8 71 38 33 00              	callq	0x140b81df0 <.text+0xb80df0>
14084e57f: 48 8b 46 08                 	movq	0x8(%rsi), %rax
14084e583: 4c 89 24 f8                 	movq	%r12, (%rax,%rdi,8)
14084e587: 48 8d 47 01                 	leaq	0x1(%rdi), %rax
14084e58b: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14084e58f: e9 95 00 00 00              	jmp	0x14084e629 <.text+0x84d629>
14084e594: 41 0f 10 40 f0              	movups	-0x10(%r8), %xmm0
14084e599: 0f 11 85 c0 01 00 00        	movups	%xmm0, 0x1c0(%rbp)
14084e5a0: 41 0f 10 40 e0              	movups	-0x20(%r8), %xmm0
14084e5a5: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
14084e5ac: 41 0f 10 40 d0              	movups	-0x30(%r8), %xmm0
14084e5b1: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
14084e5b8: f3 41 0f 6f 40 90           	movdqu	-0x70(%r8), %xmm0
14084e5be: f3 41 0f 6f 48 a0           	movdqu	-0x60(%r8), %xmm1
14084e5c4: f3 41 0f 6f 50 b0           	movdqu	-0x50(%r8), %xmm2
14084e5ca: f3 41 0f 6f 58 c0           	movdqu	-0x40(%r8), %xmm3
14084e5d0: f3 0f 7f 9d 90 01 00 00     	movdqu	%xmm3, 0x190(%rbp)
14084e5d8: f3 0f 7f 95 80 01 00 00     	movdqu	%xmm2, 0x180(%rbp)
14084e5e0: f3 0f 7f 8d 70 01 00 00     	movdqu	%xmm1, 0x170(%rbp)
14084e5e8: f3 0f 7f 85 60 01 00 00     	movdqu	%xmm0, 0x160(%rbp)
14084e5f0: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
14084e5f7: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
14084e5fe: 4c 89 f2                    	movq	%r14, %rdx
14084e601: e8 3a 14 4f 00              	callq	0x140d3fa40 <.text+0xd3ea40>
14084e606: 49 89 c4                    	movq	%rax, %r12
14084e609: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
14084e610: 48 3b 3e                    	cmpq	(%rsi), %rdi
14084e613: 0f 84 57 ff ff ff           	je	0x14084e570 <.text+0x84d570>
14084e619: 4c 89 24 fb                 	movq	%r12, (%rbx,%rdi,8)
14084e61d: 48 89 f8                    	movq	%rdi, %rax
14084e620: 48 ff c0                    	incq	%rax
14084e623: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14084e627: 74 04                       	je	0x14084e62d <.text+0x84d62d>
14084e629: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
14084e62d: 48 81 c4 e8 02 00 00        	addq	$0x2e8, %rsp            # imm = 0x2E8
14084e634: 5b                          	popq	%rbx
14084e635: 5f                          	popq	%rdi
14084e636: 5e                          	popq	%rsi
14084e637: 41 5c                       	popq	%r12
14084e639: 41 5d                       	popq	%r13
14084e63b: 41 5e                       	popq	%r14
14084e63d: 41 5f                       	popq	%r15
14084e63f: 5d                          	popq	%rbp
14084e640: c3                          	retq
14084e641: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14084e650: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084e655: 55                          	pushq	%rbp
14084e656: 41 57                       	pushq	%r15
14084e658: 41 56                       	pushq	%r14
14084e65a: 41 55                       	pushq	%r13
14084e65c: 41 54                       	pushq	%r12
14084e65e: 56                          	pushq	%rsi
14084e65f: 57                          	pushq	%rdi
14084e660: 53                          	pushq	%rbx
14084e661: 48 83 ec 28                 	subq	$0x28, %rsp
14084e665: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084e66c: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14084e670: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
14084e676: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
14084e67d: e8 7e 55 0d 02              	callq	0x142923c00 <.text+0x2922c00>
14084e682: c6 85 5f 02 00 00 00        	movb	$0x0, 0x25f(%rbp)
14084e689: 48 83 c4 28                 	addq	$0x28, %rsp
14084e68d: 5b                          	popq	%rbx
14084e68e: 5f                          	popq	%rdi
14084e68f: 5e                          	popq	%rsi
14084e690: 41 5c                       	popq	%r12
14084e692: 41 5d                       	popq	%r13
14084e694: 41 5e                       	popq	%r14
14084e696: 41 5f                       	popq	%r15
14084e698: 5d                          	popq	%rbp
14084e699: c3                          	retq
14084e69a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14084e6a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084e6a5: 55                          	pushq	%rbp
14084e6a6: 41 57                       	pushq	%r15
14084e6a8: 41 56                       	pushq	%r14
14084e6aa: 41 55                       	pushq	%r13
14084e6ac: 41 54                       	pushq	%r12
14084e6ae: 56                          	pushq	%rsi
14084e6af: 57                          	pushq	%rdi
14084e6b0: 53                          	pushq	%rbx
14084e6b1: 48 83 ec 28                 	subq	$0x28, %rsp
14084e6b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084e6bc: 80 bd 5f 02 00 00 00        	cmpb	$0x0, 0x25f(%rbp)
14084e6c3: 74 0c                       	je	0x14084e6d1 <.text+0x84d6d1>
14084e6c5: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14084e6cc: e8 7f 24 da ff              	callq	0x1405f0b50 <.text+0x5efb50>
14084e6d1: 90                          	nop
14084e6d2: 48 83 c4 28                 	addq	$0x28, %rsp
14084e6d6: 5b                          	popq	%rbx
14084e6d7: 5f                          	popq	%rdi
14084e6d8: 5e                          	popq	%rsi
14084e6d9: 41 5c                       	popq	%r12
14084e6db: 41 5d                       	popq	%r13
14084e6dd: 41 5e                       	popq	%r14
14084e6df: 41 5f                       	popq	%r15
14084e6e1: 5d                          	popq	%rbp
14084e6e2: c3                          	retq
14084e6e3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14084e6f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084e6f5: 55                          	pushq	%rbp
14084e6f6: 41 57                       	pushq	%r15
14084e6f8: 41 56                       	pushq	%r14
14084e6fa: 41 55                       	pushq	%r13
14084e6fc: 41 54                       	pushq	%r12
14084e6fe: 56                          	pushq	%rsi
14084e6ff: 57                          	pushq	%rdi
14084e700: 53                          	pushq	%rbx
14084e701: 48 83 ec 28                 	subq	$0x28, %rsp
14084e705: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084e70c: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
14084e711: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
14084e716: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
14084e71d: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
14084e722: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14084e726: c6 85 5e 02 00 00 00        	movb	$0x0, 0x25e(%rbp)
14084e72d: 48 83 c4 28                 	addq	$0x28, %rsp
14084e731: 5b                          	popq	%rbx
14084e732: 5f                          	popq	%rdi
14084e733: 5e                          	popq	%rsi
14084e734: 41 5c                       	popq	%r12
14084e736: 41 5d                       	popq	%r13
14084e738: 41 5e                       	popq	%r14
14084e73a: 41 5f                       	popq	%r15
14084e73c: 5d                          	popq	%rbp
14084e73d: c3                          	retq
14084e73e: 66 90                       	nop
14084e740: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084e745: 55                          	pushq	%rbp
14084e746: 41 57                       	pushq	%r15
14084e748: 41 56                       	pushq	%r14
14084e74a: 41 55                       	pushq	%r13
14084e74c: 41 54                       	pushq	%r12
14084e74e: 56                          	pushq	%rsi
14084e74f: 57                          	pushq	%rdi
14084e750: 53                          	pushq	%rbx
14084e751: 48 83 ec 28                 	subq	$0x28, %rsp
14084e755: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084e75c: 80 bd 5e 02 00 00 00        	cmpb	$0x0, 0x25e(%rbp)
14084e763: 74 1f                       	je	0x14084e784 <.text+0x84d784>
14084e765: 48 8d 95 a0 00 00 00        	leaq	0xa0(%rbp), %rdx
14084e76c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14084e773: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14084e779: 41 b9 10 00 00 00           	movl	$0x10, %r9d
14084e77f: e8 bc 2b 80 ff              	callq	0x140051340 <.text+0x50340>
14084e784: 90                          	nop
14084e785: 48 83 c4 28                 	addq	$0x28, %rsp
14084e789: 5b                          	popq	%rbx
14084e78a: 5f                          	popq	%rdi
14084e78b: 5e                          	popq	%rsi
14084e78c: 41 5c                       	popq	%r12
14084e78e: 41 5d                       	popq	%r13
14084e790: 41 5e                       	popq	%r14
14084e792: 41 5f                       	popq	%r15
14084e794: 5d                          	popq	%rbp
14084e795: c3                          	retq
14084e796: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14084e7a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084e7a5: 55                          	pushq	%rbp
14084e7a6: 41 57                       	pushq	%r15
14084e7a8: 41 56                       	pushq	%r14
14084e7aa: 41 55                       	pushq	%r13
14084e7ac: 41 54                       	pushq	%r12
14084e7ae: 56                          	pushq	%rsi
14084e7af: 57                          	pushq	%rdi
14084e7b0: 53                          	pushq	%rbx
14084e7b1: 48 83 ec 28                 	subq	$0x28, %rsp
14084e7b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084e7bc: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
14084e7c3: 48 85 d2                    	testq	%rdx, %rdx
14084e7c6: 74 16                       	je	0x14084e7de <.text+0x84d7de>
14084e7c8: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
14084e7cf: 48 c1 e2 03                 	shlq	$0x3, %rdx
14084e7d3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14084e7d9: e8 42 54 dc ff              	callq	0x140613c20 <.text+0x612c20>
14084e7de: 90                          	nop
14084e7df: 48 83 c4 28                 	addq	$0x28, %rsp
14084e7e3: 5b                          	popq	%rbx
14084e7e4: 5f                          	popq	%rdi
14084e7e5: 5e                          	popq	%rsi
14084e7e6: 41 5c                       	popq	%r12
14084e7e8: 41 5d                       	popq	%r13
14084e7ea: 41 5e                       	popq	%r14
14084e7ec: 41 5f                       	popq	%r15
14084e7ee: 5d                          	popq	%rbp
14084e7ef: c3                          	retq
