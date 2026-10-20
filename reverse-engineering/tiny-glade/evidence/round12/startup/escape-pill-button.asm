
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1406811c0: 56                          	pushq	%rsi
1406811c1: 57                          	pushq	%rdi
1406811c2: 48 83 ec 68                 	subq	$0x68, %rsp
1406811c6: 48 89 ce                    	movq	%rcx, %rsi
1406811c9: 4c 89 4c 24 28              	movq	%r9, 0x28(%rsp)
1406811ce: 4c 89 44 24 20              	movq	%r8, 0x20(%rsp)
1406811d3: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1406811d8: e8 53 ef 91 00              	callq	0x140fa0130 <.text+0xf9f130>
1406811dd: 48 8b 7c 24 30              	movq	0x30(%rsp), %rdi
1406811e2: 48 83 bf b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rdi)
1406811ea: 0f 84 c4 02 00 00           	je	0x1406814b4 <.text+0x6804b4>
1406811f0: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1406811f5: 48 8d 8f c0 01 00 00        	leaq	0x1c0(%rdi), %rcx
1406811fc: e8 6f fb f2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140681201: 48 8b 8f a0 01 00 00        	movq	0x1a0(%rdi), %rcx
140681208: 48 8b 97 a8 01 00 00        	movq	0x1a8(%rdi), %rdx
14068120f: 49 89 d0                    	movq	%rdx, %r8
140681212: 49 21 c0                    	andq	%rax, %r8
140681215: 48 c1 e8 39                 	shrq	$0x39, %rax
140681219: 66 0f 6e c0                 	movd	%eax, %xmm0
14068121d: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140681221: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140681226: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068122b: 48 8d 81 40 ff ff ff        	leaq	-0xc0(%rcx), %rax
140681232: 45 31 c9                    	xorl	%r9d, %r9d
140681235: 4c 8b 54 24 38              	movq	0x38(%rsp), %r10
14068123a: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14068123e: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140681244: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140681248: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14068124c: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
140681251: 45 85 db                    	testl	%r11d, %r11d
140681254: 74 2a                       	je	0x140681280 <.text+0x680280>
140681256: f3 41 0f bc fb              	tzcntl	%r11d, %edi
14068125b: 4c 01 c7                    	addq	%r8, %rdi
14068125e: 48 21 d7                    	andq	%rdx, %rdi
140681261: 48 f7 df                    	negq	%rdi
140681264: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
140681268: 48 c1 e7 06                 	shlq	$0x6, %rdi
14068126c: 4c 3b 14 38                 	cmpq	(%rax,%rdi), %r10
140681270: 74 30                       	je	0x1406812a2 <.text+0x6802a2>
140681272: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
140681276: 66 44 21 df                 	andw	%r11w, %di
14068127a: 41 89 fb                    	movl	%edi, %r11d
14068127d: 75 d7                       	jne	0x140681256 <.text+0x680256>
14068127f: 90                          	nop
140681280: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140681284: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
140681289: 45 85 db                    	testl	%r11d, %r11d
14068128c: 0f 85 22 02 00 00           	jne	0x1406814b4 <.text+0x6804b4>
140681292: 4d 01 c8                    	addq	%r9, %r8
140681295: 49 83 c0 10                 	addq	$0x10, %r8
140681299: 49 83 c1 10                 	addq	$0x10, %r9
14068129d: 49 21 d0                    	andq	%rdx, %r8
1406812a0: eb 9c                       	jmp	0x14068123e <.text+0x68023e>
1406812a2: 66 c7 44 39 fc 00 0d        	movw	$0xd00, -0x4(%rcx,%rdi) # imm = 0xD00
1406812a9: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
1406812ae: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
1406812b3: f3 0f 6f 44 24 30           	movdqu	0x30(%rsp), %xmm0
1406812b9: 66 0f 7f 44 24 50           	movdqa	%xmm0, 0x50(%rsp)
1406812bf: 48 8b 7c 24 50              	movq	0x50(%rsp), %rdi
1406812c4: 48 83 bf b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rdi)
1406812cc: 0f 84 0e 02 00 00           	je	0x1406814e0 <.text+0x6804e0>
1406812d2: 48 8d 54 24 58              	leaq	0x58(%rsp), %rdx
1406812d7: 48 8d 8f c0 01 00 00        	leaq	0x1c0(%rdi), %rcx
1406812de: e8 8d fa f2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1406812e3: 48 8b 8f a0 01 00 00        	movq	0x1a0(%rdi), %rcx
1406812ea: 48 8b 97 a8 01 00 00        	movq	0x1a8(%rdi), %rdx
1406812f1: 49 89 d0                    	movq	%rdx, %r8
1406812f4: 49 21 c0                    	andq	%rax, %r8
1406812f7: 48 c1 e8 39                 	shrq	$0x39, %rax
1406812fb: 66 0f 6e c0                 	movd	%eax, %xmm0
1406812ff: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140681303: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140681308: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068130d: 48 8d 81 40 ff ff ff        	leaq	-0xc0(%rcx), %rax
140681314: 45 31 c9                    	xorl	%r9d, %r9d
140681317: 4c 8b 54 24 58              	movq	0x58(%rsp), %r10
14068131c: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140681320: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140681326: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14068132a: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14068132e: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
140681333: 45 85 db                    	testl	%r11d, %r11d
140681336: 74 38                       	je	0x140681370 <.text+0x680370>
140681338: f3 41 0f bc fb              	tzcntl	%r11d, %edi
14068133d: 4c 01 c7                    	addq	%r8, %rdi
140681340: 48 21 d7                    	andq	%rdx, %rdi
140681343: 48 f7 df                    	negq	%rdi
140681346: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
14068134a: 48 c1 e7 06                 	shlq	$0x6, %rdi
14068134e: 4c 3b 14 38                 	cmpq	(%rax,%rdi), %r10
140681352: 74 3e                       	je	0x140681392 <.text+0x680392>
140681354: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
140681358: 66 44 21 df                 	andw	%r11w, %di
14068135c: 41 89 fb                    	movl	%edi, %r11d
14068135f: 75 d7                       	jne	0x140681338 <.text+0x680338>
140681361: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140681370: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140681374: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
140681379: 45 85 db                    	testl	%r11d, %r11d
14068137c: 0f 85 5e 01 00 00           	jne	0x1406814e0 <.text+0x6804e0>
140681382: 4d 01 c8                    	addq	%r9, %r8
140681385: 49 83 c0 10                 	addq	$0x10, %r8
140681389: 49 83 c1 10                 	addq	$0x10, %r9
14068138d: 49 21 d0                    	andq	%rdx, %r8
140681390: eb 8e                       	jmp	0x140681320 <.text+0x680320>
140681392: 66 0f ef c0                 	pxor	%xmm0, %xmm0
140681396: f3 0f 7f 84 39 7c ff ff ff  	movdqu	%xmm0, -0x84(%rcx,%rdi)
14068139f: f3 0f 7f 84 39 70 ff ff ff  	movdqu	%xmm0, -0x90(%rcx,%rdi)
1406813a8: f3 0f 7f 84 39 60 ff ff ff  	movdqu	%xmm0, -0xa0(%rcx,%rdi)
1406813b1: 48 c7 44 39 8c 00 00 48 42  	movq	$0x42480000, -0x74(%rcx,%rdi) # imm = 0x42480000
1406813ba: c7 44 39 94 00 00 48 42     	movl	$0x42480000, -0x6c(%rcx,%rdi) # imm = 0x42480000
1406813c2: 48 c7 44 39 98 00 00 00 00  	movq	$0x0, -0x68(%rcx,%rdi)
1406813cb: 48 c7 44 39 9d 00 00 00 00  	movq	$0x0, -0x63(%rcx,%rdi)
1406813d4: c6 44 39 a5 02              	movb	$0x2, -0x5b(%rcx,%rdi)
1406813d9: 66 c7 44 39 a6 01 00        	movw	$0x1, -0x5a(%rcx,%rdi)
1406813e0: 48 8b 44 24 60              	movq	0x60(%rsp), %rax
1406813e5: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
1406813ea: 0f 28 44 24 50              	movaps	0x50(%rsp), %xmm0
1406813ef: 0f 29 44 24 30              	movaps	%xmm0, 0x30(%rsp)
1406813f4: 48 8b 7c 24 30              	movq	0x30(%rsp), %rdi
1406813f9: 48 83 bf b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rdi)
140681401: 0f 84 ad 00 00 00           	je	0x1406814b4 <.text+0x6804b4>
140681407: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
14068140c: 48 8d 8f c0 01 00 00        	leaq	0x1c0(%rdi), %rcx
140681413: e8 58 f9 f2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140681418: 48 8b 8f a0 01 00 00        	movq	0x1a0(%rdi), %rcx
14068141f: 48 8b 97 a8 01 00 00        	movq	0x1a8(%rdi), %rdx
140681426: 49 89 d0                    	movq	%rdx, %r8
140681429: 49 21 c0                    	andq	%rax, %r8
14068142c: 48 c1 e8 39                 	shrq	$0x39, %rax
140681430: 66 0f 6e c0                 	movd	%eax, %xmm0
140681434: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140681438: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14068143d: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140681442: 48 8d 81 40 ff ff ff        	leaq	-0xc0(%rcx), %rax
140681449: 45 31 c9                    	xorl	%r9d, %r9d
14068144c: 4c 8b 54 24 38              	movq	0x38(%rsp), %r10
140681451: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140681455: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14068145b: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14068145f: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140681463: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
140681468: 45 85 db                    	testl	%r11d, %r11d
14068146b: 74 29                       	je	0x140681496 <.text+0x680496>
14068146d: f3 41 0f bc fb              	tzcntl	%r11d, %edi
140681472: 4c 01 c7                    	addq	%r8, %rdi
140681475: 48 21 d7                    	andq	%rdx, %rdi
140681478: 48 f7 df                    	negq	%rdi
14068147b: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
14068147f: 48 c1 e7 06                 	shlq	$0x6, %rdi
140681483: 4c 3b 14 38                 	cmpq	(%rax,%rdi), %r10
140681487: 74 37                       	je	0x1406814c0 <.text+0x6804c0>
140681489: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
14068148d: 66 44 21 df                 	andw	%r11w, %di
140681491: 41 89 fb                    	movl	%edi, %r11d
140681494: 75 d7                       	jne	0x14068146d <.text+0x68046d>
140681496: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14068149a: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14068149f: 45 85 db                    	testl	%r11d, %r11d
1406814a2: 75 10                       	jne	0x1406814b4 <.text+0x6804b4>
1406814a4: 4d 01 c8                    	addq	%r9, %r8
1406814a7: 49 83 c0 10                 	addq	$0x10, %r8
1406814ab: 49 83 c1 10                 	addq	$0x10, %r9
1406814af: 49 21 d0                    	andq	%rdx, %r8
1406814b2: eb a1                       	jmp	0x140681455 <.text+0x680455>
1406814b4: 48 8d 0d 35 6b 39 02        	leaq	0x2396b35(%rip), %rcx   # 0x142a17ff0
1406814bb: e8 50 7e 25 02              	callq	0x1428d9310 <.text+0x28d8310>
1406814c0: 80 64 39 ff df              	andb	$-0x21, -0x1(%rcx,%rdi)
1406814c5: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
1406814ca: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1406814ce: 0f 28 44 24 30              	movaps	0x30(%rsp), %xmm0
1406814d3: 0f 11 06                    	movups	%xmm0, (%rsi)
1406814d6: 48 89 f0                    	movq	%rsi, %rax
1406814d9: 48 83 c4 68                 	addq	$0x68, %rsp
1406814dd: 5f                          	popq	%rdi
1406814de: 5e                          	popq	%rsi
1406814df: c3                          	retq
1406814e0: 48 8d 0d 79 6a 39 02        	leaq	0x2396a79(%rip), %rcx   # 0x142a17f60
1406814e7: e8 24 7e 25 02              	callq	0x1428d9310 <.text+0x28d8310>
1406814ec: cc                          	int3
1406814ed: cc                          	int3
1406814ee: cc                          	int3
1406814ef: cc                          	int3
1406814f0: 56                          	pushq	%rsi
1406814f1: 57                          	pushq	%rdi
1406814f2: 48 83 ec 38                 	subq	$0x38, %rsp
1406814f6: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1406814fb: 0f 28 f2                    	movaps	%xmm2, %xmm6
1406814fe: 48 89 d6                    	movq	%rdx, %rsi
140681501: 48 89 cf                    	movq	%rcx, %rdi
140681504: 48 89 d1                    	movq	%rdx, %rcx
140681507: b2 01                       	movb	$0x1, %dl
140681509: e8 72 c9 91 00              	callq	0x140f9de80 <.text+0xf9ce80>
14068150e: f3 0f 11 70 40              	movss	%xmm6, 0x40(%rax)
140681513: 0f 10 06                    	movups	(%rsi), %xmm0
140681516: 0f 11 07                    	movups	%xmm0, (%rdi)
140681519: 48 8b 46 10                 	movq	0x10(%rsi), %rax
14068151d: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
140681521: 48 89 f8                    	movq	%rdi, %rax
140681524: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
140681529: 48 83 c4 38                 	addq	$0x38, %rsp
14068152d: 5f                          	popq	%rdi
14068152e: 5e                          	popq	%rsi
14068152f: c3                          	retq
140681530: 56                          	pushq	%rsi
140681531: 57                          	pushq	%rdi
