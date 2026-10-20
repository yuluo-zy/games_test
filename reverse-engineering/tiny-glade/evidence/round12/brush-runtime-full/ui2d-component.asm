
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14084da10: 55                          	pushq	%rbp
14084da11: 41 57                       	pushq	%r15
14084da13: 41 56                       	pushq	%r14
14084da15: 41 55                       	pushq	%r13
14084da17: 41 54                       	pushq	%r12
14084da19: 56                          	pushq	%rsi
14084da1a: 57                          	pushq	%rdi
14084da1b: 53                          	pushq	%rbx
14084da1c: 48 81 ec e8 02 00 00        	subq	$0x2e8, %rsp            # imm = 0x2E8
14084da23: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14084da2b: 48 c7 85 60 02 00 00 fe ff ff ff    	movq	$-0x2, 0x260(%rbp)
14084da36: 49 89 d6                    	movq	%rdx, %r14
14084da39: 48 8b b5 d0 02 00 00        	movq	0x2d0(%rbp), %rsi
14084da40: 4c 8b 3a                    	movq	(%rdx), %r15
14084da43: 48 8b 5e 08                 	movq	0x8(%rsi), %rbx
14084da47: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
14084da4b: 4c 89 f9                    	movq	%r15, %rcx
14084da4e: 48 89 da                    	movq	%rbx, %rdx
14084da51: 49 89 f8                    	movq	%rdi, %r8
14084da54: e8 17 33 4f 00              	callq	0x140d40d70 <.text+0xd3fd70>
14084da59: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
14084da64: 48 c7 85 38 02 00 00 08 00 00 00    	movq	$0x8, 0x238(%rbp)
14084da6f: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
14084da7a: 49 83 7f 30 00              	cmpq	$0x0, 0x30(%r15)
14084da7f: 48 b8 8f 88 b9 a8 b4 da 0c c5       	movabsq	$-0x3af3254b57467771, %rax # imm = 0xC50CDAB4A8B9888F
14084da89: 48 b9 59 45 79 4f e1 67 60 0a       	movabsq	$0xa6067e14f794559, %rcx # imm = 0xA6067E14F794559
14084da93: 0f 84 9a 00 00 00           	je	0x14084db33 <.text+0x84cb33>
14084da99: 49 8b 57 18                 	movq	0x18(%r15), %rdx
14084da9d: 4d 8b 47 20                 	movq	0x20(%r15), %r8
14084daa1: 4d 89 c1                    	movq	%r8, %r9
14084daa4: 49 21 c1                    	andq	%rax, %r9
14084daa7: 45 31 d2                    	xorl	%r10d, %r10d
14084daaa: 66 0f 6f 05 8e 2a 10 02     	movdqa	0x2102a8e(%rip), %xmm0  # 0x142950540
14084dab2: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14084dab6: f3 42 0f 6f 14 0a           	movdqu	(%rdx,%r9), %xmm2
14084dabc: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14084dac0: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14084dac4: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
14084dac9: 45 85 db                    	testl	%r11d, %r11d
14084dacc: 75 2f                       	jne	0x14084dafd <.text+0x84cafd>
14084dace: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14084dad2: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14084dad7: 45 85 db                    	testl	%r11d, %r11d
14084dada: 75 57                       	jne	0x14084db33 <.text+0x84cb33>
14084dadc: 4d 01 d1                    	addq	%r10, %r9
14084dadf: 49 83 c1 10                 	addq	$0x10, %r9
14084dae3: 49 83 c2 10                 	addq	$0x10, %r10
14084dae7: 4d 21 c1                    	andq	%r8, %r9
14084daea: eb ca                       	jmp	0x14084dab6 <.text+0x84cab6>
14084daec: 0f 1f 40 00                 	nopl	(%rax)
14084daf0: 45 8d 63 ff                 	leal	-0x1(%r11), %r12d
14084daf4: 66 45 21 dc                 	andw	%r11w, %r12w
14084daf8: 45 89 e3                    	movl	%r12d, %r11d
14084dafb: 74 d1                       	je	0x14084dace <.text+0x84cace>
14084dafd: f3 45 0f bc e3              	tzcntl	%r11d, %r12d
14084db02: 4d 01 cc                    	addq	%r9, %r12
14084db05: 4d 21 c4                    	andq	%r8, %r12
14084db08: 49 f7 dc                    	negq	%r12
14084db0b: 4f 8d 24 64                 	leaq	(%r12,%r12,2), %r12
14084db0f: 4a 39 4c e2 e8              	cmpq	%rcx, -0x18(%rdx,%r12,8)
14084db14: 75 da                       	jne	0x14084daf0 <.text+0x84caf0>
14084db16: 4a 39 44 e2 f0              	cmpq	%rax, -0x10(%rdx,%r12,8)
14084db1b: 75 d3                       	jne	0x14084daf0 <.text+0x84caf0>
14084db1d: 4a 8d 04 e2                 	leaq	(%rdx,%r12,8), %rax
14084db21: 4c 8b 60 f8                 	movq	-0x8(%rax), %r12
14084db25: 48 3b 3e                    	cmpq	(%rsi), %rdi
14084db28: 0f 84 52 03 00 00           	je	0x14084de80 <.text+0x84ce80>
14084db2e: e9 f6 03 00 00              	jmp	0x14084df29 <.text+0x84cf29>
14084db33: 48 89 9d 48 02 00 00        	movq	%rbx, 0x248(%rbp)
14084db3a: 41 0f b6 57 60              	movzbl	0x60(%r15), %edx
14084db3f: 49 8b 97 80 00 00 00        	movq	0x80(%r15), %rdx
14084db46: 4d 8b 8f 88 00 00 00        	movq	0x88(%r15), %r9
14084db4d: 4d 89 ca                    	movq	%r9, %r10
14084db50: 49 21 c2                    	andq	%rax, %r10
14084db53: 45 31 db                    	xorl	%r11d, %r11d
14084db56: 66 0f 6f 05 e2 29 10 02     	movdqa	0x21029e2(%rip), %xmm0  # 0x142950540
14084db5e: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14084db62: f3 42 0f 6f 14 12           	movdqu	(%rdx,%r10), %xmm2
14084db68: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14084db6c: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14084db70: 66 44 0f d7 e3              	pmovmskb	%xmm3, %r12d
14084db75: 45 85 e4                    	testl	%r12d, %r12d
14084db78: 75 34                       	jne	0x14084dbae <.text+0x84cbae>
14084db7a: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14084db7e: 66 44 0f d7 c2              	pmovmskb	%xmm2, %r8d
14084db83: 45 85 c0                    	testl	%r8d, %r8d
14084db86: 0f 85 f2 00 00 00           	jne	0x14084dc7e <.text+0x84cc7e>
14084db8c: 4d 01 da                    	addq	%r11, %r10
14084db8f: 49 83 c2 10                 	addq	$0x10, %r10
14084db93: 49 83 c3 10                 	addq	$0x10, %r11
14084db97: 4d 21 ca                    	andq	%r9, %r10
14084db9a: eb c6                       	jmp	0x14084db62 <.text+0x84cb62>
14084db9c: 0f 1f 40 00                 	nopl	(%rax)
14084dba0: 45 8d 44 24 ff              	leal	-0x1(%r12), %r8d
14084dba5: 66 45 21 e0                 	andw	%r12w, %r8w
14084dba9: 45 89 c4                    	movl	%r8d, %r12d
14084dbac: 74 cc                       	je	0x14084db7a <.text+0x84cb7a>
14084dbae: f3 45 0f bc ec              	tzcntl	%r12d, %r13d
14084dbb3: 4d 01 d5                    	addq	%r10, %r13
14084dbb6: 4d 21 cd                    	andq	%r9, %r13
14084dbb9: 4d 89 e8                    	movq	%r13, %r8
14084dbbc: 49 f7 d8                    	negq	%r8
14084dbbf: 4a 8d 1c ed 00 00 00 00     	leaq	(,%r13,8), %rbx
14084dbc7: 49 c1 e0 07                 	shlq	$0x7, %r8
14084dbcb: 49 29 d8                    	subq	%rbx, %r8
14084dbce: 4a 39 8c 02 78 ff ff ff     	cmpq	%rcx, -0x88(%rdx,%r8)
14084dbd6: 75 c8                       	jne	0x14084dba0 <.text+0x84cba0>
14084dbd8: 4a 39 44 02 80              	cmpq	%rax, -0x80(%rdx,%r8)
14084dbdd: 75 c1                       	jne	0x14084dba0 <.text+0x84cba0>
14084dbdf: 4d 89 ea                    	movq	%r13, %r10
14084dbe2: 49 c1 e2 07                 	shlq	$0x7, %r10
14084dbe6: 4f 8d 1c ea                 	leaq	(%r10,%r13,8), %r11
14084dbea: 49 c1 fb 03                 	sarq	$0x3, %r11
14084dbee: 49 ba f1 f0 f0 f0 f0 f0 f0 f0       	movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
14084dbf8: 4d 0f af d3                 	imulq	%r11, %r10
14084dbfc: 4d 8d 5a f0                 	leaq	-0x10(%r10), %r11
14084dc00: 4d 21 cb                    	andq	%r9, %r11
14084dc03: f3 42 0f 6f 04 1a           	movdqu	(%rdx,%r11), %xmm0
14084dc09: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14084dc0d: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
14084dc11: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
14084dc16: f3 42 0f 6f 04 12           	movdqu	(%rdx,%r10), %xmm0
14084dc1c: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
14084dc20: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
14084dc25: 45 85 e4                    	testl	%r12d, %r12d
14084dc28: 74 0b                       	je	0x14084dc35 <.text+0x84cc35>
14084dc2a: 66 45 0f bd e4              	bsrw	%r12w, %r12w
14084dc2f: 41 83 f4 0f                 	xorl	$0xf, %r12d
14084dc33: eb 05                       	jmp	0x14084dc3a <.text+0x84cc3a>
14084dc35: 66 41 bc 10 00              	movw	$0x10, %r12w
14084dc3a: 49 01 d0                    	addq	%rdx, %r8
14084dc3d: 41 81 c9 00 00 01 00        	orl	$0x10000, %r9d          # imm = 0x10000
14084dc44: f3 45 0f bc c9              	tzcntl	%r9d, %r9d
14084dc49: 45 01 cc                    	addl	%r9d, %r12d
14084dc4c: 41 b1 80                    	movb	$-0x80, %r9b
14084dc4f: 66 41 83 fc 0f              	cmpw	$0xf, %r12w
14084dc54: 77 0a                       	ja	0x14084dc60 <.text+0x84cc60>
14084dc56: 49 ff 87 90 00 00 00        	incq	0x90(%r15)
14084dc5d: 41 b1 ff                    	movb	$-0x1, %r9b
14084dc60: 46 88 0c 12                 	movb	%r9b, (%rdx,%r10)
14084dc64: 46 88 4c 1a 10              	movb	%r9b, 0x10(%rdx,%r11)
14084dc69: 49 ff 8f 98 00 00 00        	decq	0x98(%r15)
14084dc70: 49 8b 50 88                 	movq	-0x78(%r8), %rdx
14084dc74: 48 83 fa 02                 	cmpq	$0x2, %rdx
14084dc78: 0f 85 26 02 00 00           	jne	0x14084dea4 <.text+0x84cea4>
14084dc7e: 49 8b 56 08                 	movq	0x8(%r14), %rdx
14084dc82: 4c 8b 22                    	movq	(%rdx), %r12
14084dc85: 4d 8d 6c 24 01              	leaq	0x1(%r12), %r13
14084dc8a: 4c 89 2a                    	movq	%r13, (%rdx)
14084dc8d: 48 ba 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
14084dc97: 48 89 95 08 02 00 00        	movq	%rdx, 0x208(%rbp)
14084dc9e: 48 8d 15 2f a3 3e 02        	leaq	0x23ea32f(%rip), %rdx   # 0x142c37fd4
14084dca5: 48 89 95 10 02 00 00        	movq	%rdx, 0x210(%rbp)
14084dcac: 48 c7 85 18 02 00 00 29 00 00 00    	movq	$0x29, 0x218(%rbp)
14084dcb7: 66 c7 85 28 02 00 00 00 01  	movw	$0x100, 0x228(%rbp)     # imm = 0x100
14084dcc0: 48 c7 85 d0 01 00 00 01 00 00 00    	movq	$0x1, 0x1d0(%rbp)
14084dccb: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
14084dcd2: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
14084dcd9: 48 c7 85 f8 01 00 00 01 00 00 00    	movq	$0x1, 0x1f8(%rbp)
14084dce4: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
14084dcef: 48 c7 85 20 02 00 00 00 00 00 00    	movq	$0x0, 0x220(%rbp)
14084dcfa: c6 85 2a 02 00 00 01        	movb	$0x1, 0x22a(%rbp)
14084dd01: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
14084dd0c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14084dd13: 4c 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %r8
14084dd1a: 4c 89 e2                    	movq	%r12, %rdx
14084dd1d: e8 4e 1b 4f 00              	callq	0x140d3f870 <.text+0xd3e870>
14084dd22: 4d 39 6f 10                 	cmpq	%r13, 0x10(%r15)
14084dd26: 73 19                       	jae	0x14084dd41 <.text+0x84cd41>
14084dd28: c6 85 5f 02 00 00 01        	movb	$0x1, 0x25f(%rbp)
14084dd2f: 4c 8d 05 3a 67 24 02        	leaq	0x224673a(%rip), %r8    # 0x142a94470
14084dd36: 4c 89 f9                    	movq	%r15, %rcx
14084dd39: 4c 89 ea                    	movq	%r13, %rdx
14084dd3c: e8 ff 65 aa 00              	callq	0x1412f4340 <.text+0x12f3340>
14084dd41: 49 8b 5f 08                 	movq	0x8(%r15), %rbx
14084dd45: 4d 69 fc d0 00 00 00        	imulq	$0xd0, %r12, %r15
14084dd4c: 4a 8d 04 3b                 	leaq	(%rbx,%r15), %rax
14084dd50: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
14084dd57: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14084dd5b: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
14084dd62: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
14084dd68: e8 93 5e 0d 02              	callq	0x142923c00 <.text+0x2922c00>
14084dd6d: 42 83 3c 3b 02              	cmpl	$0x2, (%rbx,%r15)
14084dd72: 74 0c                       	je	0x14084dd80 <.text+0x84cd80>
14084dd74: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
14084dd7b: e8 d0 2d da ff              	callq	0x1405f0b50 <.text+0x5efb50>
14084dd80: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14084dd84: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
14084dd8a: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
14084dd91: e8 6a 5e 0d 02              	callq	0x142923c00 <.text+0x2922c00>
14084dd96: 49 8b 0e                    	movq	(%r14), %rcx
14084dd99: 48 83 c1 18                 	addq	$0x18, %rcx
14084dd9d: 48 ba 59 45 79 4f e1 67 60 0a       	movabsq	$0xa6067e14f794559, %rdx # imm = 0xA6067E14F794559
14084dda7: 49 b8 8f 88 b9 a8 b4 da 0c c5       	movabsq	$-0x3af3254b57467771, %r8 # imm = 0xC50CDAB4A8B9888F
14084ddb1: 4d 89 e1                    	movq	%r12, %r9
14084ddb4: e8 67 4e 83 ff              	callq	0x140082c20 <.text+0x81c20>
14084ddb9: 0f 10 05 40 66 24 02        	movups	0x2246640(%rip), %xmm0  # 0x142a94400
14084ddc0: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
14084ddc7: f3 0f 6f 05 21 66 24 02     	movdqu	0x2246621(%rip), %xmm0  # 0x142a943f0
14084ddcf: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
14084ddd7: c6 85 5e 02 00 00 01        	movb	$0x1, 0x25e(%rbp)
14084ddde: 48 8d 85 30 02 00 00        	leaq	0x230(%rbp), %rax
14084dde5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14084ddea: 4c 89 f2                    	movq	%r14, %rdx
14084dded: e8 1e fc ff ff              	callq	0x14084da10 <.text+0x84ca10>
14084ddf2: 49 8b 06                    	movq	(%r14), %rax
14084ddf5: 48 8b 40 08                 	movq	0x8(%rax), %rax
14084ddf9: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
14084de00: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
14084de07: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
14084de0b: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
14084de0f: 4a 8d 4c 38 60              	leaq	0x60(%rax,%r15), %rcx
14084de14: 4a 8d 94 38 80 00 00 00     	leaq	0x80(%rax,%r15), %rdx
14084de1c: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14084de22: 41 b9 10 00 00 00           	movl	$0x10, %r9d
14084de28: 48 89 8d 50 02 00 00        	movq	%rcx, 0x250(%rbp)
14084de2f: e8 0c 35 80 ff              	callq	0x140051340 <.text+0x50340>
14084de34: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
14084de3b: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
14084de40: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
14084de45: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
14084de4c: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
14084de51: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14084de55: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
14084de5c: 48 85 d2                    	testq	%rdx, %rdx
14084de5f: 74 16                       	je	0x14084de77 <.text+0x84ce77>
14084de61: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
14084de68: 48 c1 e2 03                 	shlq	$0x3, %rdx
14084de6c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14084de72: e8 a9 5d dc ff              	callq	0x140613c20 <.text+0x612c20>
14084de77: 48 3b 3e                    	cmpq	(%rsi), %rdi
14084de7a: 0f 85 a9 00 00 00           	jne	0x14084df29 <.text+0x84cf29>
14084de80: 48 8d 15 11 68 24 02        	leaq	0x2246811(%rip), %rdx   # 0x142a94698
14084de87: 48 89 f1                    	movq	%rsi, %rcx
14084de8a: e8 61 3f 33 00              	callq	0x140b81df0 <.text+0xb80df0>
14084de8f: 48 8b 46 08                 	movq	0x8(%rsi), %rax
14084de93: 4c 89 24 f8                 	movq	%r12, (%rax,%rdi,8)
14084de97: 48 8d 47 01                 	leaq	0x1(%rdi), %rax
14084de9b: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14084de9f: e9 95 00 00 00              	jmp	0x14084df39 <.text+0x84cf39>
14084dea4: 41 0f 10 40 f0              	movups	-0x10(%r8), %xmm0
14084dea9: 0f 11 85 c0 01 00 00        	movups	%xmm0, 0x1c0(%rbp)
14084deb0: 41 0f 10 40 e0              	movups	-0x20(%r8), %xmm0
14084deb5: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
14084debc: 41 0f 10 40 d0              	movups	-0x30(%r8), %xmm0
14084dec1: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
14084dec8: f3 41 0f 6f 40 90           	movdqu	-0x70(%r8), %xmm0
14084dece: f3 41 0f 6f 48 a0           	movdqu	-0x60(%r8), %xmm1
14084ded4: f3 41 0f 6f 50 b0           	movdqu	-0x50(%r8), %xmm2
14084deda: f3 41 0f 6f 58 c0           	movdqu	-0x40(%r8), %xmm3
14084dee0: f3 0f 7f 9d 90 01 00 00     	movdqu	%xmm3, 0x190(%rbp)
14084dee8: f3 0f 7f 95 80 01 00 00     	movdqu	%xmm2, 0x180(%rbp)
14084def0: f3 0f 7f 8d 70 01 00 00     	movdqu	%xmm1, 0x170(%rbp)
14084def8: f3 0f 7f 85 60 01 00 00     	movdqu	%xmm0, 0x160(%rbp)
14084df00: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
14084df07: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
14084df0e: 4c 89 f2                    	movq	%r14, %rdx
14084df11: e8 2a 1b 4f 00              	callq	0x140d3fa40 <.text+0xd3ea40>
14084df16: 49 89 c4                    	movq	%rax, %r12
14084df19: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
14084df20: 48 3b 3e                    	cmpq	(%rsi), %rdi
14084df23: 0f 84 57 ff ff ff           	je	0x14084de80 <.text+0x84ce80>
14084df29: 4c 89 24 fb                 	movq	%r12, (%rbx,%rdi,8)
14084df2d: 48 89 f8                    	movq	%rdi, %rax
14084df30: 48 ff c0                    	incq	%rax
14084df33: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14084df37: 74 04                       	je	0x14084df3d <.text+0x84cf3d>
14084df39: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
14084df3d: 48 81 c4 e8 02 00 00        	addq	$0x2e8, %rsp            # imm = 0x2E8
14084df44: 5b                          	popq	%rbx
14084df45: 5f                          	popq	%rdi
14084df46: 5e                          	popq	%rsi
14084df47: 41 5c                       	popq	%r12
14084df49: 41 5d                       	popq	%r13
14084df4b: 41 5e                       	popq	%r14
14084df4d: 41 5f                       	popq	%r15
14084df4f: 5d                          	popq	%rbp
14084df50: c3                          	retq
14084df51: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14084df60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084df65: 55                          	pushq	%rbp
14084df66: 41 57                       	pushq	%r15
14084df68: 41 56                       	pushq	%r14
14084df6a: 41 55                       	pushq	%r13
14084df6c: 41 54                       	pushq	%r12
14084df6e: 56                          	pushq	%rsi
14084df6f: 57                          	pushq	%rdi
14084df70: 53                          	pushq	%rbx
14084df71: 48 83 ec 28                 	subq	$0x28, %rsp
14084df75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084df7c: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
14084df80: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
14084df86: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
14084df8d: e8 6e 5c 0d 02              	callq	0x142923c00 <.text+0x2922c00>
14084df92: c6 85 5f 02 00 00 00        	movb	$0x0, 0x25f(%rbp)
14084df99: 48 83 c4 28                 	addq	$0x28, %rsp
14084df9d: 5b                          	popq	%rbx
14084df9e: 5f                          	popq	%rdi
14084df9f: 5e                          	popq	%rsi
14084dfa0: 41 5c                       	popq	%r12
14084dfa2: 41 5d                       	popq	%r13
14084dfa4: 41 5e                       	popq	%r14
14084dfa6: 41 5f                       	popq	%r15
14084dfa8: 5d                          	popq	%rbp
14084dfa9: c3                          	retq
14084dfaa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14084dfb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084dfb5: 55                          	pushq	%rbp
14084dfb6: 41 57                       	pushq	%r15
14084dfb8: 41 56                       	pushq	%r14
14084dfba: 41 55                       	pushq	%r13
14084dfbc: 41 54                       	pushq	%r12
14084dfbe: 56                          	pushq	%rsi
14084dfbf: 57                          	pushq	%rdi
14084dfc0: 53                          	pushq	%rbx
14084dfc1: 48 83 ec 28                 	subq	$0x28, %rsp
14084dfc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084dfcc: 80 bd 5f 02 00 00 00        	cmpb	$0x0, 0x25f(%rbp)
14084dfd3: 74 0c                       	je	0x14084dfe1 <.text+0x84cfe1>
14084dfd5: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14084dfdc: e8 6f 2b da ff              	callq	0x1405f0b50 <.text+0x5efb50>
14084dfe1: 90                          	nop
14084dfe2: 48 83 c4 28                 	addq	$0x28, %rsp
14084dfe6: 5b                          	popq	%rbx
14084dfe7: 5f                          	popq	%rdi
14084dfe8: 5e                          	popq	%rsi
14084dfe9: 41 5c                       	popq	%r12
14084dfeb: 41 5d                       	popq	%r13
14084dfed: 41 5e                       	popq	%r14
14084dfef: 41 5f                       	popq	%r15
14084dff1: 5d                          	popq	%rbp
14084dff2: c3                          	retq
14084dff3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14084e000: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084e005: 55                          	pushq	%rbp
14084e006: 41 57                       	pushq	%r15
14084e008: 41 56                       	pushq	%r14
14084e00a: 41 55                       	pushq	%r13
14084e00c: 41 54                       	pushq	%r12
14084e00e: 56                          	pushq	%rsi
14084e00f: 57                          	pushq	%rdi
14084e010: 53                          	pushq	%rbx
14084e011: 48 83 ec 28                 	subq	$0x28, %rsp
14084e015: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084e01c: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
14084e021: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
14084e026: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
14084e02d: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
14084e032: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14084e036: c6 85 5e 02 00 00 00        	movb	$0x0, 0x25e(%rbp)
14084e03d: 48 83 c4 28                 	addq	$0x28, %rsp
14084e041: 5b                          	popq	%rbx
14084e042: 5f                          	popq	%rdi
14084e043: 5e                          	popq	%rsi
14084e044: 41 5c                       	popq	%r12
14084e046: 41 5d                       	popq	%r13
14084e048: 41 5e                       	popq	%r14
14084e04a: 41 5f                       	popq	%r15
14084e04c: 5d                          	popq	%rbp
14084e04d: c3                          	retq
14084e04e: 66 90                       	nop
14084e050: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084e055: 55                          	pushq	%rbp
14084e056: 41 57                       	pushq	%r15
14084e058: 41 56                       	pushq	%r14
14084e05a: 41 55                       	pushq	%r13
14084e05c: 41 54                       	pushq	%r12
14084e05e: 56                          	pushq	%rsi
14084e05f: 57                          	pushq	%rdi
14084e060: 53                          	pushq	%rbx
14084e061: 48 83 ec 28                 	subq	$0x28, %rsp
14084e065: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084e06c: 80 bd 5e 02 00 00 00        	cmpb	$0x0, 0x25e(%rbp)
14084e073: 74 1f                       	je	0x14084e094 <.text+0x84d094>
14084e075: 48 8d 95 a0 00 00 00        	leaq	0xa0(%rbp), %rdx
14084e07c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14084e083: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14084e089: 41 b9 10 00 00 00           	movl	$0x10, %r9d
14084e08f: e8 ac 32 80 ff              	callq	0x140051340 <.text+0x50340>
14084e094: 90                          	nop
14084e095: 48 83 c4 28                 	addq	$0x28, %rsp
14084e099: 5b                          	popq	%rbx
14084e09a: 5f                          	popq	%rdi
14084e09b: 5e                          	popq	%rsi
14084e09c: 41 5c                       	popq	%r12
14084e09e: 41 5d                       	popq	%r13
14084e0a0: 41 5e                       	popq	%r14
14084e0a2: 41 5f                       	popq	%r15
14084e0a4: 5d                          	popq	%rbp
14084e0a5: c3                          	retq
14084e0a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14084e0b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14084e0b5: 55                          	pushq	%rbp
14084e0b6: 41 57                       	pushq	%r15
14084e0b8: 41 56                       	pushq	%r14
14084e0ba: 41 55                       	pushq	%r13
14084e0bc: 41 54                       	pushq	%r12
14084e0be: 56                          	pushq	%rsi
14084e0bf: 57                          	pushq	%rdi
14084e0c0: 53                          	pushq	%rbx
14084e0c1: 48 83 ec 28                 	subq	$0x28, %rsp
14084e0c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14084e0cc: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
14084e0d3: 48 85 d2                    	testq	%rdx, %rdx
14084e0d6: 74 16                       	je	0x14084e0ee <.text+0x84d0ee>
14084e0d8: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
14084e0df: 48 c1 e2 03                 	shlq	$0x3, %rdx
14084e0e3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14084e0e9: e8 32 5b dc ff              	callq	0x140613c20 <.text+0x612c20>
14084e0ee: 90                          	nop
14084e0ef: 48 83 c4 28                 	addq	$0x28, %rsp
14084e0f3: 5b                          	popq	%rbx
14084e0f4: 5f                          	popq	%rdi
14084e0f5: 5e                          	popq	%rsi
14084e0f6: 41 5c                       	popq	%r12
14084e0f8: 41 5d                       	popq	%r13
14084e0fa: 41 5e                       	popq	%r14
14084e0fc: 41 5f                       	popq	%r15
14084e0fe: 5d                          	popq	%rbp
14084e0ff: c3                          	retq
