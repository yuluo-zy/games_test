
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140851f80: 55                          	pushq	%rbp
140851f81: 41 57                       	pushq	%r15
140851f83: 41 56                       	pushq	%r14
140851f85: 41 55                       	pushq	%r13
140851f87: 41 54                       	pushq	%r12
140851f89: 56                          	pushq	%rsi
140851f8a: 57                          	pushq	%rdi
140851f8b: 53                          	pushq	%rbx
140851f8c: 48 81 ec e8 02 00 00        	subq	$0x2e8, %rsp            # imm = 0x2E8
140851f93: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140851f9b: 48 c7 85 60 02 00 00 fe ff ff ff    	movq	$-0x2, 0x260(%rbp)
140851fa6: 49 89 d6                    	movq	%rdx, %r14
140851fa9: 48 8b b5 d0 02 00 00        	movq	0x2d0(%rbp), %rsi
140851fb0: 4c 8b 3a                    	movq	(%rdx), %r15
140851fb3: 48 8b 5e 08                 	movq	0x8(%rsi), %rbx
140851fb7: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
140851fbb: 4c 89 f9                    	movq	%r15, %rcx
140851fbe: 48 89 da                    	movq	%rbx, %rdx
140851fc1: 49 89 f8                    	movq	%rdi, %r8
140851fc4: e8 a7 ed 4e 00              	callq	0x140d40d70 <.text+0xd3fd70>
140851fc9: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
140851fd4: 48 c7 85 38 02 00 00 08 00 00 00    	movq	$0x8, 0x238(%rbp)
140851fdf: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
140851fea: 49 83 7f 30 00              	cmpq	$0x0, 0x30(%r15)
140851fef: 48 b8 c0 5b 83 19 96 c7 22 ba       	movabsq	$-0x45dd3869e67ca440, %rax # imm = 0xBA22C79619835BC0
140851ff9: 48 b9 8d ff 01 a7 39 a9 f1 ad       	movabsq	$-0x520e56c658fe0073, %rcx # imm = 0xADF1A939A701FF8D
140852003: 0f 84 9a 00 00 00           	je	0x1408520a3 <.text+0x8510a3>
140852009: 49 8b 57 18                 	movq	0x18(%r15), %rdx
14085200d: 4d 8b 47 20                 	movq	0x20(%r15), %r8
140852011: 4d 89 c1                    	movq	%r8, %r9
140852014: 49 21 c1                    	andq	%rax, %r9
140852017: 45 31 d2                    	xorl	%r10d, %r10d
14085201a: 66 0f 6f 05 5e e5 0f 02     	movdqa	0x20fe55e(%rip), %xmm0  # 0x142950580
140852022: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140852026: f3 42 0f 6f 14 0a           	movdqu	(%rdx,%r9), %xmm2
14085202c: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140852030: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140852034: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
140852039: 45 85 db                    	testl	%r11d, %r11d
14085203c: 75 2f                       	jne	0x14085206d <.text+0x85106d>
14085203e: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140852042: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
140852047: 45 85 db                    	testl	%r11d, %r11d
14085204a: 75 57                       	jne	0x1408520a3 <.text+0x8510a3>
14085204c: 4d 01 d1                    	addq	%r10, %r9
14085204f: 49 83 c1 10                 	addq	$0x10, %r9
140852053: 49 83 c2 10                 	addq	$0x10, %r10
140852057: 4d 21 c1                    	andq	%r8, %r9
14085205a: eb ca                       	jmp	0x140852026 <.text+0x851026>
14085205c: 0f 1f 40 00                 	nopl	(%rax)
140852060: 45 8d 63 ff                 	leal	-0x1(%r11), %r12d
140852064: 66 45 21 dc                 	andw	%r11w, %r12w
140852068: 45 89 e3                    	movl	%r12d, %r11d
14085206b: 74 d1                       	je	0x14085203e <.text+0x85103e>
14085206d: f3 45 0f bc e3              	tzcntl	%r11d, %r12d
140852072: 4d 01 cc                    	addq	%r9, %r12
140852075: 4d 21 c4                    	andq	%r8, %r12
140852078: 49 f7 dc                    	negq	%r12
14085207b: 4f 8d 24 64                 	leaq	(%r12,%r12,2), %r12
14085207f: 4a 39 4c e2 e8              	cmpq	%rcx, -0x18(%rdx,%r12,8)
140852084: 75 da                       	jne	0x140852060 <.text+0x851060>
140852086: 4a 39 44 e2 f0              	cmpq	%rax, -0x10(%rdx,%r12,8)
14085208b: 75 d3                       	jne	0x140852060 <.text+0x851060>
14085208d: 4a 8d 04 e2                 	leaq	(%rdx,%r12,8), %rax
140852091: 4c 8b 60 f8                 	movq	-0x8(%rax), %r12
140852095: 48 3b 3e                    	cmpq	(%rsi), %rdi
140852098: 0f 84 52 03 00 00           	je	0x1408523f0 <.text+0x8513f0>
14085209e: e9 f6 03 00 00              	jmp	0x140852499 <.text+0x851499>
1408520a3: 48 89 9d 48 02 00 00        	movq	%rbx, 0x248(%rbp)
1408520aa: 41 0f b6 57 60              	movzbl	0x60(%r15), %edx
1408520af: 49 8b 97 80 00 00 00        	movq	0x80(%r15), %rdx
1408520b6: 4d 8b 8f 88 00 00 00        	movq	0x88(%r15), %r9
1408520bd: 4d 89 ca                    	movq	%r9, %r10
1408520c0: 49 21 c2                    	andq	%rax, %r10
1408520c3: 45 31 db                    	xorl	%r11d, %r11d
1408520c6: 66 0f 6f 05 b2 e4 0f 02     	movdqa	0x20fe4b2(%rip), %xmm0  # 0x142950580
1408520ce: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1408520d2: f3 42 0f 6f 14 12           	movdqu	(%rdx,%r10), %xmm2
1408520d8: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1408520dc: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1408520e0: 66 44 0f d7 e3              	pmovmskb	%xmm3, %r12d
1408520e5: 45 85 e4                    	testl	%r12d, %r12d
1408520e8: 75 34                       	jne	0x14085211e <.text+0x85111e>
1408520ea: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1408520ee: 66 44 0f d7 c2              	pmovmskb	%xmm2, %r8d
1408520f3: 45 85 c0                    	testl	%r8d, %r8d
1408520f6: 0f 85 f2 00 00 00           	jne	0x1408521ee <.text+0x8511ee>
1408520fc: 4d 01 da                    	addq	%r11, %r10
1408520ff: 49 83 c2 10                 	addq	$0x10, %r10
140852103: 49 83 c3 10                 	addq	$0x10, %r11
140852107: 4d 21 ca                    	andq	%r9, %r10
14085210a: eb c6                       	jmp	0x1408520d2 <.text+0x8510d2>
14085210c: 0f 1f 40 00                 	nopl	(%rax)
140852110: 45 8d 44 24 ff              	leal	-0x1(%r12), %r8d
140852115: 66 45 21 e0                 	andw	%r12w, %r8w
140852119: 45 89 c4                    	movl	%r8d, %r12d
14085211c: 74 cc                       	je	0x1408520ea <.text+0x8510ea>
14085211e: f3 45 0f bc ec              	tzcntl	%r12d, %r13d
140852123: 4d 01 d5                    	addq	%r10, %r13
140852126: 4d 21 cd                    	andq	%r9, %r13
140852129: 4d 89 e8                    	movq	%r13, %r8
14085212c: 49 f7 d8                    	negq	%r8
14085212f: 4a 8d 1c ed 00 00 00 00     	leaq	(,%r13,8), %rbx
140852137: 49 c1 e0 07                 	shlq	$0x7, %r8
14085213b: 49 29 d8                    	subq	%rbx, %r8
14085213e: 4a 39 8c 02 78 ff ff ff     	cmpq	%rcx, -0x88(%rdx,%r8)
140852146: 75 c8                       	jne	0x140852110 <.text+0x851110>
140852148: 4a 39 44 02 80              	cmpq	%rax, -0x80(%rdx,%r8)
14085214d: 75 c1                       	jne	0x140852110 <.text+0x851110>
14085214f: 4d 89 ea                    	movq	%r13, %r10
140852152: 49 c1 e2 07                 	shlq	$0x7, %r10
140852156: 4f 8d 1c ea                 	leaq	(%r10,%r13,8), %r11
14085215a: 49 c1 fb 03                 	sarq	$0x3, %r11
14085215e: 49 ba f1 f0 f0 f0 f0 f0 f0 f0       	movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
140852168: 4d 0f af d3                 	imulq	%r11, %r10
14085216c: 4d 8d 5a f0                 	leaq	-0x10(%r10), %r11
140852170: 4d 21 cb                    	andq	%r9, %r11
140852173: f3 42 0f 6f 04 1a           	movdqu	(%rdx,%r11), %xmm0
140852179: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14085217d: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
140852181: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
140852186: f3 42 0f 6f 04 12           	movdqu	(%rdx,%r10), %xmm0
14085218c: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
140852190: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
140852195: 45 85 e4                    	testl	%r12d, %r12d
140852198: 74 0b                       	je	0x1408521a5 <.text+0x8511a5>
14085219a: 66 45 0f bd e4              	bsrw	%r12w, %r12w
14085219f: 41 83 f4 0f                 	xorl	$0xf, %r12d
1408521a3: eb 05                       	jmp	0x1408521aa <.text+0x8511aa>
1408521a5: 66 41 bc 10 00              	movw	$0x10, %r12w
1408521aa: 49 01 d0                    	addq	%rdx, %r8
1408521ad: 41 81 c9 00 00 01 00        	orl	$0x10000, %r9d          # imm = 0x10000
1408521b4: f3 45 0f bc c9              	tzcntl	%r9d, %r9d
1408521b9: 45 01 cc                    	addl	%r9d, %r12d
1408521bc: 41 b1 80                    	movb	$-0x80, %r9b
1408521bf: 66 41 83 fc 0f              	cmpw	$0xf, %r12w
1408521c4: 77 0a                       	ja	0x1408521d0 <.text+0x8511d0>
1408521c6: 49 ff 87 90 00 00 00        	incq	0x90(%r15)
1408521cd: 41 b1 ff                    	movb	$-0x1, %r9b
1408521d0: 46 88 0c 12                 	movb	%r9b, (%rdx,%r10)
1408521d4: 46 88 4c 1a 10              	movb	%r9b, 0x10(%rdx,%r11)
1408521d9: 49 ff 8f 98 00 00 00        	decq	0x98(%r15)
1408521e0: 49 8b 50 88                 	movq	-0x78(%r8), %rdx
1408521e4: 48 83 fa 02                 	cmpq	$0x2, %rdx
1408521e8: 0f 85 26 02 00 00           	jne	0x140852414 <.text+0x851414>
1408521ee: 49 8b 56 08                 	movq	0x8(%r14), %rdx
1408521f2: 4c 8b 22                    	movq	(%rdx), %r12
1408521f5: 4d 8d 6c 24 01              	leaq	0x1(%r12), %r13
1408521fa: 4c 89 2a                    	movq	%r13, (%rdx)
1408521fd: 48 ba 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
140852207: 48 89 95 08 02 00 00        	movq	%rdx, 0x208(%rbp)
14085220e: 48 8d 15 2e 48 3e 02        	leaq	0x23e482e(%rip), %rdx   # 0x142c36a43
140852215: 48 89 95 10 02 00 00        	movq	%rdx, 0x210(%rbp)
14085221c: 48 c7 85 18 02 00 00 2b 00 00 00    	movq	$0x2b, 0x218(%rbp)
140852227: 66 c7 85 28 02 00 00 00 01  	movw	$0x100, 0x228(%rbp)     # imm = 0x100
140852230: 48 c7 85 d0 01 00 00 01 00 00 00    	movq	$0x1, 0x1d0(%rbp)
14085223b: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
140852242: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
140852249: 48 c7 85 f8 01 00 00 01 00 00 00    	movq	$0x1, 0x1f8(%rbp)
140852254: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
14085225f: 48 c7 85 20 02 00 00 00 00 00 00    	movq	$0x0, 0x220(%rbp)
14085226a: c6 85 2a 02 00 00 01        	movb	$0x1, 0x22a(%rbp)
140852271: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
14085227c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140852283: 4c 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %r8
14085228a: 4c 89 e2                    	movq	%r12, %rdx
14085228d: e8 de d5 4e 00              	callq	0x140d3f870 <.text+0xd3e870>
140852292: 4d 39 6f 10                 	cmpq	%r13, 0x10(%r15)
140852296: 73 19                       	jae	0x1408522b1 <.text+0x8512b1>
140852298: c6 85 5f 02 00 00 01        	movb	$0x1, 0x25f(%rbp)
14085229f: 4c 8d 05 ca 21 24 02        	leaq	0x22421ca(%rip), %r8    # 0x142a94470
1408522a6: 4c 89 f9                    	movq	%r15, %rcx
1408522a9: 4c 89 ea                    	movq	%r13, %rdx
1408522ac: e8 8f 20 aa 00              	callq	0x1412f4340 <.text+0x12f3340>
1408522b1: 49 8b 5f 08                 	movq	0x8(%r15), %rbx
1408522b5: 4d 69 fc d0 00 00 00        	imulq	$0xd0, %r12, %r15
1408522bc: 4a 8d 04 3b                 	leaq	(%rbx,%r15), %rax
1408522c0: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1408522c7: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1408522cb: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
1408522d2: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
1408522d8: e8 23 19 0d 02              	callq	0x142923c00 <.text+0x2922c00>
1408522dd: 42 83 3c 3b 02              	cmpl	$0x2, (%rbx,%r15)
1408522e2: 74 0c                       	je	0x1408522f0 <.text+0x8512f0>
1408522e4: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
1408522eb: e8 60 e8 d9 ff              	callq	0x1405f0b50 <.text+0x5efb50>
1408522f0: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1408522f4: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
1408522fa: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
140852301: e8 fa 18 0d 02              	callq	0x142923c00 <.text+0x2922c00>
140852306: 49 8b 0e                    	movq	(%r14), %rcx
140852309: 48 83 c1 18                 	addq	$0x18, %rcx
14085230d: 48 ba 8d ff 01 a7 39 a9 f1 ad       	movabsq	$-0x520e56c658fe0073, %rdx # imm = 0xADF1A939A701FF8D
140852317: 49 b8 c0 5b 83 19 96 c7 22 ba       	movabsq	$-0x45dd3869e67ca440, %r8 # imm = 0xBA22C79619835BC0
140852321: 4d 89 e1                    	movq	%r12, %r9
140852324: e8 f7 08 83 ff              	callq	0x140082c20 <.text+0x81c20>
140852329: 0f 10 05 d0 20 24 02        	movups	0x22420d0(%rip), %xmm0  # 0x142a94400
140852330: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
140852337: f3 0f 6f 05 b1 20 24 02     	movdqu	0x22420b1(%rip), %xmm0  # 0x142a943f0
14085233f: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
140852347: c6 85 5e 02 00 00 01        	movb	$0x1, 0x25e(%rbp)
14085234e: 48 8d 85 30 02 00 00        	leaq	0x230(%rbp), %rax
140852355: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14085235a: 4c 89 f2                    	movq	%r14, %rdx
14085235d: e8 1e fc ff ff              	callq	0x140851f80 <.text+0x850f80>
140852362: 49 8b 06                    	movq	(%r14), %rax
140852365: 48 8b 40 08                 	movq	0x8(%rax), %rax
140852369: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
140852370: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
140852377: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
14085237b: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
14085237f: 4a 8d 4c 38 60              	leaq	0x60(%rax,%r15), %rcx
140852384: 4a 8d 94 38 80 00 00 00     	leaq	0x80(%rax,%r15), %rdx
14085238c: 41 b8 20 00 00 00           	movl	$0x20, %r8d
140852392: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140852398: 48 89 8d 50 02 00 00        	movq	%rcx, 0x250(%rbp)
14085239f: e8 9c ef 7f ff              	callq	0x140051340 <.text+0x50340>
1408523a4: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
1408523ab: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
1408523b0: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
1408523b5: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1408523bc: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
1408523c1: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1408523c5: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
1408523cc: 48 85 d2                    	testq	%rdx, %rdx
1408523cf: 74 16                       	je	0x1408523e7 <.text+0x8513e7>
1408523d1: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
1408523d8: 48 c1 e2 03                 	shlq	$0x3, %rdx
1408523dc: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1408523e2: e8 39 18 dc ff              	callq	0x140613c20 <.text+0x612c20>
1408523e7: 48 3b 3e                    	cmpq	(%rsi), %rdi
1408523ea: 0f 85 a9 00 00 00           	jne	0x140852499 <.text+0x851499>
1408523f0: 48 8d 15 91 23 24 02        	leaq	0x2242391(%rip), %rdx   # 0x142a94788
1408523f7: 48 89 f1                    	movq	%rsi, %rcx
1408523fa: e8 f1 f9 32 00              	callq	0x140b81df0 <.text+0xb80df0>
1408523ff: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140852403: 4c 89 24 f8                 	movq	%r12, (%rax,%rdi,8)
140852407: 48 8d 47 01                 	leaq	0x1(%rdi), %rax
14085240b: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14085240f: e9 95 00 00 00              	jmp	0x1408524a9 <.text+0x8514a9>
140852414: 41 0f 10 40 f0              	movups	-0x10(%r8), %xmm0
140852419: 0f 11 85 c0 01 00 00        	movups	%xmm0, 0x1c0(%rbp)
140852420: 41 0f 10 40 e0              	movups	-0x20(%r8), %xmm0
140852425: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
14085242c: 41 0f 10 40 d0              	movups	-0x30(%r8), %xmm0
140852431: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
140852438: f3 41 0f 6f 40 90           	movdqu	-0x70(%r8), %xmm0
14085243e: f3 41 0f 6f 48 a0           	movdqu	-0x60(%r8), %xmm1
140852444: f3 41 0f 6f 50 b0           	movdqu	-0x50(%r8), %xmm2
14085244a: f3 41 0f 6f 58 c0           	movdqu	-0x40(%r8), %xmm3
140852450: f3 0f 7f 9d 90 01 00 00     	movdqu	%xmm3, 0x190(%rbp)
140852458: f3 0f 7f 95 80 01 00 00     	movdqu	%xmm2, 0x180(%rbp)
140852460: f3 0f 7f 8d 70 01 00 00     	movdqu	%xmm1, 0x170(%rbp)
140852468: f3 0f 7f 85 60 01 00 00     	movdqu	%xmm0, 0x160(%rbp)
140852470: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
140852477: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
14085247e: 4c 89 f2                    	movq	%r14, %rdx
140852481: e8 ba d5 4e 00              	callq	0x140d3fa40 <.text+0xd3ea40>
140852486: 49 89 c4                    	movq	%rax, %r12
140852489: 48 8b 9d 48 02 00 00        	movq	0x248(%rbp), %rbx
140852490: 48 3b 3e                    	cmpq	(%rsi), %rdi
140852493: 0f 84 57 ff ff ff           	je	0x1408523f0 <.text+0x8513f0>
140852499: 4c 89 24 fb                 	movq	%r12, (%rbx,%rdi,8)
14085249d: 48 89 f8                    	movq	%rdi, %rax
1408524a0: 48 ff c0                    	incq	%rax
1408524a3: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1408524a7: 74 04                       	je	0x1408524ad <.text+0x8514ad>
1408524a9: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
1408524ad: 48 81 c4 e8 02 00 00        	addq	$0x2e8, %rsp            # imm = 0x2E8
1408524b4: 5b                          	popq	%rbx
1408524b5: 5f                          	popq	%rdi
1408524b6: 5e                          	popq	%rsi
1408524b7: 41 5c                       	popq	%r12
1408524b9: 41 5d                       	popq	%r13
1408524bb: 41 5e                       	popq	%r14
1408524bd: 41 5f                       	popq	%r15
1408524bf: 5d                          	popq	%rbp
1408524c0: c3                          	retq
1408524c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1408524d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408524d5: 55                          	pushq	%rbp
1408524d6: 41 57                       	pushq	%r15
1408524d8: 41 56                       	pushq	%r14
1408524da: 41 55                       	pushq	%r13
1408524dc: 41 54                       	pushq	%r12
1408524de: 56                          	pushq	%rsi
1408524df: 57                          	pushq	%rdi
1408524e0: 53                          	pushq	%rbx
1408524e1: 48 83 ec 28                 	subq	$0x28, %rsp
1408524e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408524ec: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1408524f0: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
1408524f6: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
1408524fd: e8 fe 16 0d 02              	callq	0x142923c00 <.text+0x2922c00>
140852502: c6 85 5f 02 00 00 00        	movb	$0x0, 0x25f(%rbp)
140852509: 48 83 c4 28                 	addq	$0x28, %rsp
14085250d: 5b                          	popq	%rbx
14085250e: 5f                          	popq	%rdi
14085250f: 5e                          	popq	%rsi
140852510: 41 5c                       	popq	%r12
140852512: 41 5d                       	popq	%r13
140852514: 41 5e                       	popq	%r14
140852516: 41 5f                       	popq	%r15
140852518: 5d                          	popq	%rbp
140852519: c3                          	retq
14085251a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140852520: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140852525: 55                          	pushq	%rbp
140852526: 41 57                       	pushq	%r15
140852528: 41 56                       	pushq	%r14
14085252a: 41 55                       	pushq	%r13
14085252c: 41 54                       	pushq	%r12
14085252e: 56                          	pushq	%rsi
14085252f: 57                          	pushq	%rdi
140852530: 53                          	pushq	%rbx
140852531: 48 83 ec 28                 	subq	$0x28, %rsp
140852535: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14085253c: 80 bd 5f 02 00 00 00        	cmpb	$0x0, 0x25f(%rbp)
140852543: 74 0c                       	je	0x140852551 <.text+0x851551>
140852545: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14085254c: e8 ff e5 d9 ff              	callq	0x1405f0b50 <.text+0x5efb50>
140852551: 90                          	nop
140852552: 48 83 c4 28                 	addq	$0x28, %rsp
140852556: 5b                          	popq	%rbx
140852557: 5f                          	popq	%rdi
140852558: 5e                          	popq	%rsi
140852559: 41 5c                       	popq	%r12
14085255b: 41 5d                       	popq	%r13
14085255d: 41 5e                       	popq	%r14
14085255f: 41 5f                       	popq	%r15
140852561: 5d                          	popq	%rbp
140852562: c3                          	retq
140852563: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140852570: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140852575: 55                          	pushq	%rbp
140852576: 41 57                       	pushq	%r15
140852578: 41 56                       	pushq	%r14
14085257a: 41 55                       	pushq	%r13
14085257c: 41 54                       	pushq	%r12
14085257e: 56                          	pushq	%rsi
14085257f: 57                          	pushq	%rdi
140852580: 53                          	pushq	%rbx
140852581: 48 83 ec 28                 	subq	$0x28, %rsp
140852585: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14085258c: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
140852591: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
140852596: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
14085259d: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
1408525a2: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1408525a6: c6 85 5e 02 00 00 00        	movb	$0x0, 0x25e(%rbp)
1408525ad: 48 83 c4 28                 	addq	$0x28, %rsp
1408525b1: 5b                          	popq	%rbx
1408525b2: 5f                          	popq	%rdi
1408525b3: 5e                          	popq	%rsi
1408525b4: 41 5c                       	popq	%r12
1408525b6: 41 5d                       	popq	%r13
1408525b8: 41 5e                       	popq	%r14
1408525ba: 41 5f                       	popq	%r15
1408525bc: 5d                          	popq	%rbp
1408525bd: c3                          	retq
1408525be: 66 90                       	nop
1408525c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408525c5: 55                          	pushq	%rbp
1408525c6: 41 57                       	pushq	%r15
1408525c8: 41 56                       	pushq	%r14
1408525ca: 41 55                       	pushq	%r13
1408525cc: 41 54                       	pushq	%r12
1408525ce: 56                          	pushq	%rsi
1408525cf: 57                          	pushq	%rdi
1408525d0: 53                          	pushq	%rbx
1408525d1: 48 83 ec 28                 	subq	$0x28, %rsp
1408525d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408525dc: 80 bd 5e 02 00 00 00        	cmpb	$0x0, 0x25e(%rbp)
1408525e3: 74 1f                       	je	0x140852604 <.text+0x851604>
1408525e5: 48 8d 95 a0 00 00 00        	leaq	0xa0(%rbp), %rdx
1408525ec: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1408525f3: 41 b8 20 00 00 00           	movl	$0x20, %r8d
1408525f9: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1408525ff: e8 3c ed 7f ff              	callq	0x140051340 <.text+0x50340>
140852604: 90                          	nop
140852605: 48 83 c4 28                 	addq	$0x28, %rsp
140852609: 5b                          	popq	%rbx
14085260a: 5f                          	popq	%rdi
14085260b: 5e                          	popq	%rsi
14085260c: 41 5c                       	popq	%r12
14085260e: 41 5d                       	popq	%r13
140852610: 41 5e                       	popq	%r14
140852612: 41 5f                       	popq	%r15
140852614: 5d                          	popq	%rbp
140852615: c3                          	retq
140852616: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140852620: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140852625: 55                          	pushq	%rbp
140852626: 41 57                       	pushq	%r15
140852628: 41 56                       	pushq	%r14
14085262a: 41 55                       	pushq	%r13
14085262c: 41 54                       	pushq	%r12
14085262e: 56                          	pushq	%rsi
14085262f: 57                          	pushq	%rdi
140852630: 53                          	pushq	%rbx
140852631: 48 83 ec 28                 	subq	$0x28, %rsp
140852635: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14085263c: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
140852643: 48 85 d2                    	testq	%rdx, %rdx
140852646: 74 16                       	je	0x14085265e <.text+0x85165e>
140852648: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
14085264f: 48 c1 e2 03                 	shlq	$0x3, %rdx
140852653: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140852659: e8 c2 15 dc ff              	callq	0x140613c20 <.text+0x612c20>
14085265e: 90                          	nop
14085265f: 48 83 c4 28                 	addq	$0x28, %rsp
140852663: 5b                          	popq	%rbx
140852664: 5f                          	popq	%rdi
140852665: 5e                          	popq	%rsi
140852666: 41 5c                       	popq	%r12
140852668: 41 5d                       	popq	%r13
14085266a: 41 5e                       	popq	%r14
14085266c: 41 5f                       	popq	%r15
14085266e: 5d                          	popq	%rbp
14085266f: c3                          	retq
