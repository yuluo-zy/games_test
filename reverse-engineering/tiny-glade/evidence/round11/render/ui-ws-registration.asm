
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14084eee0: 55                          	pushq	%rbp
14084eee1: 41 57                       	pushq	%r15
14084eee3: 41 56                       	pushq	%r14
14084eee5: 41 55                       	pushq	%r13
14084eee7: 41 54                       	pushq	%r12
14084eee9: 56                          	pushq	%rsi
14084eeea: 57                          	pushq	%rdi
14084eeeb: 53                          	pushq	%rbx
14084eeec: 48 81 ec e8 02 00 00        	subq	$0x2e8, %rsp            # imm = 0x2E8
14084eef3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14084eefb: 48 c7 85 60 02 00 00 fe ff ff ff    	movq	$-0x2, 0x260(%rbp)
14084ef06: 49 89 d6                    	movq	%rdx, %r14
14084ef09: 48 8b b5 d0 02 00 00        	movq	0x2d0(%rbp), %rsi
14084ef10: 4c 8b 3a                    	movq	(%rdx), %r15
14084ef13: 48 8b 5e 08                 	movq	0x8(%rsi), %rbx
14084ef17: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
14084ef1b: 4c 89 f9                    	movq	%r15, %rcx
14084ef1e: 48 89 da                    	movq	%rbx, %rdx
14084ef21: 49 89 f8                    	movq	%rdi, %r8
14084ef24: e8 47 1e 4f 00              	callq	0x140d40d70 <.text+0xd3fd70>
14084ef29: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
14084ef34: 48 c7 85 38 02 00 00 08 00 00 00    	movq	$0x8, 0x238(%rbp)
14084ef3f: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
14084ef4a: 49 83 7f 30 00              	cmpq	$0x0, 0x30(%r15)
14084ef4f: 48 b8 1e e0 88 03 02 d7 42 f5       	movabsq	$-0xabd28fdfc771fe2, %rax # imm = 0xF542D7020388E01E
14084ef59: 48 b9 f0 d5 87 28 96 e3 23 e5       	movabsq	$-0x1adc1c69d7782a10, %rcx # imm = 0xE523E3962887D5F0
14084ef63: 0f 84 9a 00 00 00           	je	0x14084f003 <.text+0x84e003>
14084ef69: 49 8b 57 18                 	movq	0x18(%r15), %rdx
14084ef6d: 4d 8b 47 20                 	movq	0x20(%r15), %r8
14084ef71: 4d 89 c1                    	movq	%r8, %r9
14084ef74: 49 21 c1                    	andq	%rax, %r9
14084ef77: 45 31 d2                    	xorl	%r10d, %r10d
14084ef7a: 66 0f 6f 05 9e 15 10 02     	movdqa	0x210159e(%rip), %xmm0  # 0x142950520
14084ef82: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14084ef86: f3 42 0f 6f 14 0a           	movdqu	(%rdx,%r9), %xmm2
14084ef8c: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14084ef90: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14084ef94: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
14084ef99: 45 85 db                    	testl	%r11d, %r11d
14084ef9c: 75 2f                       	jne	0x14084efcd <.text+0x84dfcd>
14084ef9e: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14084efa2: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14084efa7: 45 85 db                    	testl	%r11d, %r11d
14084efaa: 75 57                       	jne	0x14084f003 <.text+0x84e003>
14084efac: 4d 01 d1                    	addq	%r10, %r9
14084efaf: 49 83 c1 10                 	addq	$0x10, %r9
14084efb3: 49 83 c2 10                 	addq	$0x10, %r10
14084efb7: 4d 21 c1                    	andq	%r8, %r9
14084efba: eb ca                       	jmp	0x14084ef86 <.text+0x84df86>
14084efbc: 0f 1f 40 00                 	nopl	(%rax)
14084efc0: 45 8d 63 ff                 	leal	-0x1(%r11), %r12d
14084efc4: 66 45 21 dc                 	andw	%r11w, %r12w
14084efc8: 45 89 e3                    	movl	%r12d, %r11d
14084efcb: 74 d1                       	je	0x14084ef9e <.text+0x84df9e>
14084efcd: f3 45 0f bc e3              	tzcntl	%r11d, %r12d
14084efd2: 4d 01 cc                    	addq	%r9, %r12
14084efd5: 4d 21 c4                    	andq	%r8, %r12
14084efd8: 49 f7 dc                    	negq	%r12
14084efdb: 4f 8d 24 64                 	leaq	(%r12,%r12,2), %r12
14084efdf: 4a 39 4c e2 e8              	cmpq	%rcx, -0x18(%rdx,%r12,8)
14084efe4: 75 da                       	jne	0x14084efc0 <.text+0x84dfc0>
14084efe6: 4a 39 44 e2 f0              	cmpq	%rax, -0x10(%rdx,%r12,8)
14084efeb: 75 d3                       	jne	0x14084efc0 <.text+0x84dfc0>
14084efed: 4a 8d 04 e2                 	leaq	(%rdx,%r12,8), %rax
14084eff1: 4c 8b 60 f8                 	movq	-0x8(%rax), %r12
14084eff5: 48 3b 3e                    	cmpq	(%rsi), %rdi
14084eff8: 0f 84 52 03 00 00           	je	0x14084f350 <.text+0x84e350>
14084effe: e9 f6 03 00 00              	jmp	0x14084f3f9 <.text+0x84e3f9>
14084f003: 48 89 9d 48 02 00 00        	movq	%rbx, 0x248(%rbp)
14084f00a: 41 0f b6 57 60              	movzbl	0x60(%r15), %edx
14084f00f: 49 8b 97 80 00 00 00        	movq	0x80(%r15), %rdx
14084f016: 4d 8b 8f 88 00 00 00        	movq	0x88(%r15), %r9
14084f01d: 4d 89 ca                    	movq	%r9, %r10
14084f020: 49 21 c2                    	andq	%rax, %r10
14084f023: 45 31 db                    	xorl	%r11d, %r11d
14084f026: 66 0f 6f 05 f2 14 10 02     	movdqa	0x21014f2(%rip), %xmm0  # 0x142950520
14084f02e: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14084f032: f3 42 0f 6f 14 12           	movdqu	(%rdx,%r10), %xmm2
14084f038: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14084f03c: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14084f040: 66 44 0f d7 e3              	pmovmskb	%xmm3, %r12d
14084f045: 45 85 e4                    	testl	%r12d, %r12d
14084f048: 75 34                       	jne	0x14084f07e <.text+0x84e07e>
14084f04a: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14084f04e: 66 44 0f d7 c2              	pmovmskb	%xmm2, %r8d
14084f053: 45 85 c0                    	testl	%r8d, %r8d
14084f056: 0f 85 f2 00 00 00           	jne	0x14084f14e <.text+0x84e14e>
14084f05c: 4d 01 da                    	addq	%r11, %r10
14084f05f: 49 83 c2 10                 	addq	$0x10, %r10
14084f063: 49 83 c3 10                 	addq	$0x10, %r11
14084f067: 4d 21 ca                    	andq	%r9, %r10
14084f06a: eb c6                       	jmp	0x14084f032 <.text+0x84e032>
14084f06c: 0f 1f 40 00                 	nopl	(%rax)
14084f070: 45 8d 44 24 ff              	leal	-0x1(%r12), %r8d
14084f075: 66 45 21 e0                 	andw	%r12w, %r8w
14084f079: 45 89 c4                    	movl	%r8d, %r12d
14084f07c: 74 cc                       	je	0x14084f04a <.text+0x84e04a>
14084f07e: f3 45 0f bc ec              	tzcntl	%r12d, %r13d
14084f083: 4d 01 d5                    	addq	%r10, %r13
14084f086: 4d 21 cd                    	andq	%r9, %r13
14084f089: 4d 89 e8                    	movq	%r13, %r8
14084f08c: 49 f7 d8                    	negq	%r8
14084f08f: 4a 8d 1c ed 00 00 00 00     	leaq	(,%r13,8), %rbx
14084f097: 49 c1 e0 07                 	shlq	$0x7, %r8
14084f09b: 49 29 d8                    	subq	%rbx, %r8
14084f09e: 4a 39 8c 02 78 ff ff ff     	cmpq	%rcx, -0x88(%rdx,%r8)
14084f0a6: 75 c8                       	jne	0x14084f070 <.text+0x84e070>
14084f0a8: 4a 39 44 02 80              	cmpq	%rax, -0x80(%rdx,%r8)
14084f0ad: 75 c1                       	jne	0x14084f070 <.text+0x84e070>
14084f0af: 4d 89 ea                    	movq	%r13, %r10
14084f0b2: 49 c1 e2 07                 	shlq	$0x7, %r10
14084f0b6: 4f 8d 1c ea                 	leaq	(%r10,%r13,8), %r11
14084f0ba: 49 c1 fb 03                 	sarq	$0x3, %r11
14084f0be: 49 ba f1 f0 f0 f0 f0 f0 f0 f0       	movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
14084f0c8: 4d 0f af d3                 	imulq	%r11, %r10
14084f0cc: 4d 8d 5a f0                 	leaq	-0x10(%r10), %r11
14084f0d0: 4d 21 cb                    	andq	%r9, %r11
14084f0d3: f3 42 0f 6f 04 1a           	movdqu	(%rdx,%r11), %xmm0
14084f0d9: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14084f0dd: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
14084f0e1: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
14084f0e6: f3 42 0f 6f 04 12           	movdqu	(%rdx,%r10), %xmm0
14084f0ec: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
14084f0f0: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
14084f0f5: 45 85 e4                    	testl	%r12d, %r12d
14084f0f8: 74 0b                       	je	0x14084f105 <.text+0x84e105>
14084f0fa: 66 45 0f bd e4              	bsrw	%r12w, %r12w
14084f0ff: 41 83 f4 0f                 	xorl	$0xf, %r12d
14084f103: eb 05                       	jmp	0x14084f10a <.text+0x84e10a>
14084f105: 66 41 bc 10 00              	movw	$0x10, %r12w
14084f10a: 49 01 d0                    	addq	%rdx, %r8
14084f10d: 41 81 c9 00 00 01 00        	orl	$0x10000, %r9d          # imm = 0x10000
14084f114: f3 45 0f bc c9              	tzcntl	%r9d, %r9d
14084f119: 45 01 cc                    	addl	%r9d, %r12d
14084f11c: 41 b1 80                    	movb	$-0x80, %r9b
14084f11f: 66 41 83 fc 0f              	cmpw	$0xf, %r12w
14084f124: 77 0a                       	ja	0x14084f130 <.text+0x84e130>
14084f126: 49 ff 87 90 00 00 00        	incq	0x90(%r15)
14084f12d: 41 b1 ff                    	movb	$-0x1, %r9b
14084f130: 46 88 0c 12                 	movb	%r9b, (%rdx,%r10)
14084f134: 46 88 4c 1a 10              	movb	%r9b, 0x10(%rdx,%r11)
14084f139: 49 ff 8f 98 00 00 00        	decq	0x98(%r15)
14084f140: 49 8b 50 88                 	movq	-0x78(%r8), %rdx
14084f144: 48 83 fa 02                 	cmpq	$0x2, %rdx
14084f148: 0f 85 26 02 00 00           	jne	0x14084f374 <.text+0x84e374>
14084f14e: 49 8b 56 08                 	movq	0x8(%r14), %rdx
14084f152: 4c 8b 22                    	movq	(%rdx), %r12
14084f155: 4d 8d 6c 24 01              	leaq	0x1(%r12), %r13
14084f15a: 4c 89 2a                    	movq	%r13, (%rdx)
14084f15d: 48 ba 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
14084f167: 48 89 95 08 02 00 00        	movq	%rdx, 0x208(%rbp)
14084f16e: 48 8d 15 5b 81 3e 02        	leaq	0x23e815b(%rip), %rdx   # 0x142c372d0
14084f175: 48 89 95 10 02 00 00        	movq	%rdx, 0x210(%rbp)
14084f17c: 48 c7 85 18 02 00 00 26 00 00 00    	movq	$0x26, 0x218(%rbp)
14084f187: 66 c7 85 28 02 00 00 00 01  	movw	$0x100, 0x228(%rbp)     # imm = 0x100
14084f190: 48 c7 85 d0 01 00 00 01 00 00 00    	movq	$0x1, 0x1d0(%rbp)
14084f19b: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
14084f1a2: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
14084f1a9: 48 c7 85 f8 01 00 00 04 00 00 00    	movq	$0x4, 0x1f8(%rbp)
14084f1b4: 48 c7 85 00 02 00 00 14 00 00 00    	movq	$0x14, 0x200(%rbp)
14084f1bf: 48 c7 85 20 02 00 00 00 00 00 00    	movq	$0x0, 0x220(%rbp)
14084f1ca: c6 85 2a 02 00 00 01        	movb	$0x1, 0x22a(%rbp)
14084f1d1: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
14084f1dc: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14084f1e3: 4c 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %r8
14084f1ea: 4c 89 e2                    	movq	%r12, %rdx
14084f1ed: e8 7e 06 4f 00              	callq	0x140d3f870 <.text+0xd3e870>
14084f1f2: 4d 39 6f 10                 	cmpq	%r13, 0x10(%r15)
14084f1f6: 73 19                       	jae	0x14084f211 <.text+0x84e211>
14084f1f8: c6 85 5f 02 00 00 01        	movb	$0x1, 0x25f(%rbp)
14084f1ff: 4c 8d 05 6a 52 24 02        	leaq	0x224526a(%rip), %r8    # 0x142a94470
14084f206: 4c 89 f9                    	movq	%r15, %rcx
14084f209: 4c 89 ea                    	movq	%r13, %rdx
14084f20c: e8 2f 51 aa 00              	callq	0x1412f4340 <.text+0x12f3340>
14084f211: 49 8b 5f 08                 	movq	0x8(%r15), %rbx
14084f215: 4d 69 fc d0 00 00 00        	imulq	$0xd0, %r12, %r15
14084f21c: 4a 8d 04 3b                 	leaq	(%rbx,%r15), %rax
14084f220: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
14084f227: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14084f22b: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
14084f232: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
14084f238: e8 c3 49 0d 02              	callq	0x142923c00 <.text+0x2922c00>
14084f23d: 42 83 3c 3b 02              	cmpl	$0x2, (%rbx,%r15)
14084f242: 74 0c                       	je	0x14084f250 <.text+0x84e250>
14084f244: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
14084f24b: e8 00 19 da ff              	callq	0x1405f0b50 <.text+0x5efb50>
14084f250: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14084f254: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
14084f25a: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
14084f261: e8 9a 49 0d 02              	callq	0x142923c00 <.text+0x2922c00>
14084f266: 49 8b 0e                    	movq	(%r14), %rcx
14084f269: 48 83 c1 18                 	addq	$0x18, %rcx
14084f26d: 48 ba f0 d5 87 28 96 e3 23 e5       	movabsq	$-0x1adc1c69d7782a10, %rdx # imm = 0xE523E3962887D5F0
14084f277: 49 b8 1e e0 88 03 02 d7 42 f5       	movabsq	$-0xabd28fdfc771fe2, %r8 # imm = 0xF542D7020388E01E
14084f281: 4d 89 e1                    	movq	%r12, %r9
14084f284: e8 97 39 83 ff              	callq	0x140082c20 <.text+0x81c20>
14084f289: 0f 10 05 70 51 24 02        	movups	0x2245170(%rip), %xmm0  # 0x142a94400
14084f290: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
14084f297: f3 0f 6f 05 51 51 24 02     	movdqu	0x2245151(%rip), %xmm0  # 0x142a943f0
14084f29f: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
14084f2a7: c6 85 5e 02 00 00 01        	movb	$0x1, 0x25e(%rbp)
14084f2ae: 48 8d 85 30 02 00 00        	leaq	0x230(%rbp), %rax
14084f2b5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14084f2ba: 4c 89 f2                    	movq	%r14, %rdx
14084f2bd: e8 1e fc ff ff              	callq	0x14084eee0 <.text+0x84dee0>
14084f2c2: 49 8b 06                    	movq	(%r14), %rax
14084f2c5: 48 8b 40 08                 	movq	0x8(%rax), %rax
14084f2c9: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
14084f2d0: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
14084f2d7: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
14084f2db: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
14084f2df: 4a 8d 4c 38 60              	leaq	0x60(%rax,%r15), %rcx
14084f2e4: 4a 8d 94 38 80 00 00 00     	leaq	0x80(%rax,%r15), %rdx
14084f2ec: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14084f2f2: 41 b9 10 00 00 00           	movl	$0x10, %r9d
14084f2f8: 48 89 8d 50 02 00 00        	movq	%rcx, 0x250(%rbp)
14084f2ff: e8 3c 20 80 ff              	callq	0x140051340 <.text+0x50340>
14084f304: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
14084f30b: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
14084f310: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
14084f315: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
14084f31c: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
14084f321: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14084f325: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
14084f32c: 48 85 d2                    	testq	%rdx, %rdx
14084f32f: 74 16                       	je	0x14084f347 <.text+0x84e347>
14084f331: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
14084f338: 48 c1 e2 03                 	shlq	$0x3, %rdx
14084f33c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14084f342: e8 d9 48 dc ff              	callq	0x140613c20 <.text+0x612c20>
14084f347: 48 3b 3e                    	cmpq	(%rsi), %rdi
14084f34a: 0f 85 a9 00 00 00           	jne	0x14084f3f9 <.text+0x84e3f9>
14084f350: 48 8d 15 89 53 24 02        	leaq	0x2245389(%rip), %rdx   # 0x142a946e0
14084f357: 48 89 f1                    	movq	%rsi, %rcx
14084f35a: e8 91 2a 33 00              	callq	0x140b81df0 <.text+0xb80df0>
14084f35f: 48 8b 46 08                 	movq	0x8(%rsi), %rax
14084f363: 4c 89 24 f8                 	movq	%r12, (%rax,%rdi,8)
14084f367: 48 8d 47 01                 	leaq	0x1(%rdi), %rax
14084f36b: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14084f36f: e9 95 00 00 00              	jmp	0x14084f409 <.text+0x84e409>
14084f374: 41 0f 10 40 f0              	movups	-0x10(%r8), %xmm0
14084f379: 0f 11 85 c0 01 00 00        	movups	%xmm0, 0x1c0(%rbp)
14084f380: 41 0f 10 40 e0              	movups	-0x20(%r8), %xmm0
14084f385: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
14084f38c: 41 0f 10 40 d0              	movups	-0x30(%r8), %xmm0
14084f391: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
14084f398: f3 41 0f 6f 40 90           	movdqu	-0x70(%r8), %xmm0
14084f39e: f3 41 0f 6f 48 a0           	movdqu	-0x60(%r8), %xmm1
14084f3a4: f3 41 0f 6f 50 b0           	movdqu	-0x50(%r8), %xmm2
14084f3aa: f3 41 0f 6f 58 c0           	movdqu	-0x40(%r8), %xmm3
14084f3b0: f3 0f 7f 9d 90 01 00 00     	movdqu	%xmm3, 0x190(%rbp)
14084f3b8: f3 0f 7f 95 80 01 00 00     	movdqu	%xmm2, 0x180(%rbp)
14084f3c0: f3 0f 7f 8d 70 01 00 00     	movdqu	%xmm1, 0x170(%rbp)
14084f3c8: f3 0f 7f 85 60 01 00 00     	movdqu	%xmm0, 0x160(%rbp)
14084f3d0: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
14084f3d7: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
14084f3de: 4c 89 f2                    	movq	%r14, %rdx
14084f3e1: e8 5a 06 4f 00              	callq	0x140d3fa40 <.text+0xd3ea40>
14084f3e6: 49 89 c4                    	movq	%rax, %r12
14084f3e9: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
14084f3f0: 48 3b 3e                    	cmpq	(%rsi), %rdi
14084f3f3: 0f 84 57 ff ff ff           	je	0x14084f350 <.text+0x84e350>
14084f3f9: 4c 89 24 fb                 	movq	%r12, (%rbx,%rdi,8)
14084f3fd: 48 89 f8                    	movq	%rdi, %rax
14084f400: 48 ff c0                    	incq	%rax
14084f403: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14084f407: 74 04                       	je	0x14084f40d <.text+0x84e40d>
14084f409: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
14084f40d: 48 81 c4 e8 02 00 00        	addq	$0x2e8, %rsp            # imm = 0x2E8
14084f414: 5b                          	popq	%rbx
14084f415: 5f                          	popq	%rdi
14084f416: 5e                          	popq	%rsi
14084f417: 41 5c                       	popq	%r12
14084f419: 41 5d                       	popq	%r13
14084f41b: 41 5e                       	popq	%r14
14084f41d: 41 5f                       	popq	%r15
14084f41f: 5d                          	popq	%rbp
14084f420: c3                          	retq
14084f421: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14084f430: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084f435: 55                          	pushq	%rbp
14084f436: 41 57                       	pushq	%r15
14084f438: 41 56                       	pushq	%r14
14084f43a: 41 55                       	pushq	%r13
14084f43c: 41 54                       	pushq	%r12
14084f43e: 56                          	pushq	%rsi
14084f43f: 57                          	pushq	%rdi
14084f440: 53                          	pushq	%rbx
14084f441: 48 83 ec 28                 	subq	$0x28, %rsp
14084f445: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084f44c: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14084f450: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
14084f456: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
14084f45d: e8 9e 47 0d 02              	callq	0x142923c00 <.text+0x2922c00>
14084f462: c6 85 5f 02 00 00 00        	movb	$0x0, 0x25f(%rbp)
14084f469: 48 83 c4 28                 	addq	$0x28, %rsp
14084f46d: 5b                          	popq	%rbx
14084f46e: 5f                          	popq	%rdi
14084f46f: 5e                          	popq	%rsi
14084f470: 41 5c                       	popq	%r12
14084f472: 41 5d                       	popq	%r13
14084f474: 41 5e                       	popq	%r14
14084f476: 41 5f                       	popq	%r15
14084f478: 5d                          	popq	%rbp
14084f479: c3                          	retq
14084f47a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14084f480: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084f485: 55                          	pushq	%rbp
14084f486: 41 57                       	pushq	%r15
14084f488: 41 56                       	pushq	%r14
14084f48a: 41 55                       	pushq	%r13
14084f48c: 41 54                       	pushq	%r12
14084f48e: 56                          	pushq	%rsi
14084f48f: 57                          	pushq	%rdi
14084f490: 53                          	pushq	%rbx
14084f491: 48 83 ec 28                 	subq	$0x28, %rsp
14084f495: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084f49c: 80 bd 5f 02 00 00 00        	cmpb	$0x0, 0x25f(%rbp)
14084f4a3: 74 0c                       	je	0x14084f4b1 <.text+0x84e4b1>
14084f4a5: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14084f4ac: e8 9f 16 da ff              	callq	0x1405f0b50 <.text+0x5efb50>
14084f4b1: 90                          	nop
14084f4b2: 48 83 c4 28                 	addq	$0x28, %rsp
14084f4b6: 5b                          	popq	%rbx
14084f4b7: 5f                          	popq	%rdi
14084f4b8: 5e                          	popq	%rsi
14084f4b9: 41 5c                       	popq	%r12
14084f4bb: 41 5d                       	popq	%r13
14084f4bd: 41 5e                       	popq	%r14
14084f4bf: 41 5f                       	popq	%r15
14084f4c1: 5d                          	popq	%rbp
14084f4c2: c3                          	retq
14084f4c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14084f4d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084f4d5: 55                          	pushq	%rbp
14084f4d6: 41 57                       	pushq	%r15
14084f4d8: 41 56                       	pushq	%r14
14084f4da: 41 55                       	pushq	%r13
14084f4dc: 41 54                       	pushq	%r12
14084f4de: 56                          	pushq	%rsi
14084f4df: 57                          	pushq	%rdi
14084f4e0: 53                          	pushq	%rbx
14084f4e1: 48 83 ec 28                 	subq	$0x28, %rsp
14084f4e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084f4ec: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
14084f4f1: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
14084f4f6: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
14084f4fd: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
14084f502: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14084f506: c6 85 5e 02 00 00 00        	movb	$0x0, 0x25e(%rbp)
14084f50d: 48 83 c4 28                 	addq	$0x28, %rsp
14084f511: 5b                          	popq	%rbx
14084f512: 5f                          	popq	%rdi
14084f513: 5e                          	popq	%rsi
14084f514: 41 5c                       	popq	%r12
14084f516: 41 5d                       	popq	%r13
14084f518: 41 5e                       	popq	%r14
14084f51a: 41 5f                       	popq	%r15
14084f51c: 5d                          	popq	%rbp
14084f51d: c3                          	retq
14084f51e: 66 90                       	nop
14084f520: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084f525: 55                          	pushq	%rbp
14084f526: 41 57                       	pushq	%r15
14084f528: 41 56                       	pushq	%r14
14084f52a: 41 55                       	pushq	%r13
14084f52c: 41 54                       	pushq	%r12
14084f52e: 56                          	pushq	%rsi
14084f52f: 57                          	pushq	%rdi
14084f530: 53                          	pushq	%rbx
14084f531: 48 83 ec 28                 	subq	$0x28, %rsp
14084f535: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084f53c: 80 bd 5e 02 00 00 00        	cmpb	$0x0, 0x25e(%rbp)
14084f543: 74 1f                       	je	0x14084f564 <.text+0x84e564>
14084f545: 48 8d 95 a0 00 00 00        	leaq	0xa0(%rbp), %rdx
14084f54c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14084f553: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14084f559: 41 b9 10 00 00 00           	movl	$0x10, %r9d
14084f55f: e8 dc 1d 80 ff              	callq	0x140051340 <.text+0x50340>
14084f564: 90                          	nop
14084f565: 48 83 c4 28                 	addq	$0x28, %rsp
14084f569: 5b                          	popq	%rbx
14084f56a: 5f                          	popq	%rdi
14084f56b: 5e                          	popq	%rsi
14084f56c: 41 5c                       	popq	%r12
14084f56e: 41 5d                       	popq	%r13
14084f570: 41 5e                       	popq	%r14
14084f572: 41 5f                       	popq	%r15
14084f574: 5d                          	popq	%rbp
14084f575: c3                          	retq
14084f576: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14084f580: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084f585: 55                          	pushq	%rbp
14084f586: 41 57                       	pushq	%r15
14084f588: 41 56                       	pushq	%r14
14084f58a: 41 55                       	pushq	%r13
14084f58c: 41 54                       	pushq	%r12
14084f58e: 56                          	pushq	%rsi
14084f58f: 57                          	pushq	%rdi
14084f590: 53                          	pushq	%rbx
14084f591: 48 83 ec 28                 	subq	$0x28, %rsp
14084f595: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084f59c: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
14084f5a3: 48 85 d2                    	testq	%rdx, %rdx
14084f5a6: 74 16                       	je	0x14084f5be <.text+0x84e5be>
14084f5a8: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
14084f5af: 48 c1 e2 03                 	shlq	$0x3, %rdx
14084f5b3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14084f5b9: e8 62 46 dc ff              	callq	0x140613c20 <.text+0x612c20>
14084f5be: 90                          	nop
14084f5bf: 48 83 c4 28                 	addq	$0x28, %rsp
14084f5c3: 5b                          	popq	%rbx
14084f5c4: 5f                          	popq	%rdi
14084f5c5: 5e                          	popq	%rsi
14084f5c6: 41 5c                       	popq	%r12
14084f5c8: 41 5d                       	popq	%r13
14084f5ca: 41 5e                       	popq	%r14
14084f5cc: 41 5f                       	popq	%r15
14084f5ce: 5d                          	popq	%rbp
14084f5cf: c3                          	retq
