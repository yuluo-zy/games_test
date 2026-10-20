
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141215540: 55                          	pushq	%rbp
141215541: 41 57                       	pushq	%r15
141215543: 41 56                       	pushq	%r14
141215545: 41 55                       	pushq	%r13
141215547: 41 54                       	pushq	%r12
141215549: 56                          	pushq	%rsi
14121554a: 57                          	pushq	%rdi
14121554b: 53                          	pushq	%rbx
14121554c: 48 81 ec 48 04 00 00        	subq	$0x448, %rsp            # imm = 0x448
141215553: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14121555b: 44 0f 29 bd b0 03 00 00     	movaps	%xmm15, 0x3b0(%rbp)
141215563: 44 0f 29 b5 a0 03 00 00     	movaps	%xmm14, 0x3a0(%rbp)
14121556b: 44 0f 29 ad 90 03 00 00     	movaps	%xmm13, 0x390(%rbp)
141215573: 44 0f 29 a5 80 03 00 00     	movaps	%xmm12, 0x380(%rbp)
14121557b: 44 0f 29 9d 70 03 00 00     	movaps	%xmm11, 0x370(%rbp)
141215583: 44 0f 29 95 60 03 00 00     	movaps	%xmm10, 0x360(%rbp)
14121558b: 44 0f 29 8d 50 03 00 00     	movaps	%xmm9, 0x350(%rbp)
141215593: 44 0f 29 85 40 03 00 00     	movaps	%xmm8, 0x340(%rbp)
14121559b: 0f 29 bd 30 03 00 00        	movaps	%xmm7, 0x330(%rbp)
1412155a2: 0f 29 b5 20 03 00 00        	movaps	%xmm6, 0x320(%rbp)
1412155a9: 48 c7 85 18 03 00 00 fe ff ff ff    	movq	$-0x2, 0x318(%rbp)
1412155b4: 4c 89 8d 30 02 00 00        	movq	%r9, 0x230(%rbp)
1412155bb: 4c 89 85 c0 01 00 00        	movq	%r8, 0x1c0(%rbp)
1412155c2: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
1412155c9: 49 89 ce                    	movq	%rcx, %r14
1412155cc: 48 8b 91 90 00 00 00        	movq	0x90(%rcx), %rdx
1412155d3: 48 8b 81 98 00 00 00        	movq	0x98(%rcx), %rax
1412155da: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1412155de: 4c 8d 04 82                 	leaq	(%rdx,%rax,4), %r8
1412155e2: 4c 8d 0d a7 35 a0 01        	leaq	0x1a035a7(%rip), %r9    # 0x142c18b90
1412155e9: 48 8d b5 10 01 00 00        	leaq	0x110(%rbp), %rsi
1412155f0: 48 89 f1                    	movq	%rsi, %rcx
1412155f3: e8 e8 53 0a ff              	callq	0x1402ba9e0 <.text+0x2b99e0>
1412155f8: 48 8d 8d c0 02 00 00        	leaq	0x2c0(%rbp), %rcx
1412155ff: 31 ff                       	xorl	%edi, %edi
141215601: 48 89 f2                    	movq	%rsi, %rdx
141215604: 45 31 c0                    	xorl	%r8d, %r8d
141215607: e8 14 43 fe ff              	callq	0x1411f9920 <.text+0x11f8920>
14121560c: 48 3b bd c0 02 00 00        	cmpq	0x2c0(%rbp), %rdi
141215613: 0f 80 3f 1c 00 00           	jo	0x141217258 <.text+0x1216258>
141215619: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
141215620: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
141215627: 0f 10 85 c0 02 00 00        	movups	0x2c0(%rbp), %xmm0
14121562e: 0f 10 8d d0 02 00 00        	movups	0x2d0(%rbp), %xmm1
141215635: 0f 10 95 e0 02 00 00        	movups	0x2e0(%rbp), %xmm2
14121563c: 0f 29 95 70 01 00 00        	movaps	%xmm2, 0x170(%rbp)
141215643: 0f 29 8d 60 01 00 00        	movaps	%xmm1, 0x160(%rbp)
14121564a: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
141215651: 49 8b 86 f8 00 00 00        	movq	0xf8(%r14), %rax
141215658: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
14121565f: 49 8b 86 00 01 00 00        	movq	0x100(%r14), %rax
141215666: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
14121566d: 4c 89 f1                    	movq	%r14, %rcx
141215670: e8 cb 50 91 ff              	callq	0x140b2a740 <.text+0xb29740>
141215675: 89 85 80 02 00 00           	movl	%eax, 0x280(%rbp)
14121567b: f3 0f 11 85 84 02 00 00     	movss	%xmm0, 0x284(%rbp)
141215683: 49 8b 7e 08                 	movq	0x8(%r14), %rdi
141215687: 41 83 be 58 02 00 00 02     	cmpl	$0x2, 0x258(%r14)
14121568f: 49 8b 76 10                 	movq	0x10(%r14), %rsi
141215693: 48 c7 85 c0 02 00 00 00 00 00 00    	movq	$0x0, 0x2c0(%rbp)
14121569e: 48 c7 85 c8 02 00 00 08 00 00 00    	movq	$0x8, 0x2c8(%rbp)
1412156a9: 48 c7 85 d0 02 00 00 00 00 00 00    	movq	$0x0, 0x2d0(%rbp)
1412156b4: 74 6d                       	je	0x141215723 <.text+0x1214723>
1412156b6: 48 83 fe 01                 	cmpq	$0x1, %rsi
1412156ba: 0f 86 d1 1b 00 00           	jbe	0x141217291 <.text+0x1216291>
1412156c0: f2 0f 10 4c f7 f0           	movsd	-0x10(%rdi,%rsi,8), %xmm1
1412156c6: f2 0f 10 17                 	movsd	(%rdi), %xmm2
1412156ca: f2 0f 10 47 08              	movsd	0x8(%rdi), %xmm0
1412156cf: 0f 5c ca                    	subps	%xmm2, %xmm1
1412156d2: 0f 5c d0                    	subps	%xmm0, %xmm2
1412156d5: 0f 28 d9                    	movaps	%xmm1, %xmm3
1412156d8: 0f 59 da                    	mulps	%xmm2, %xmm3
1412156db: f3 0f 16 c3                 	movshdup	%xmm3, %xmm0            # xmm0 = xmm3[1,1,3,3]
1412156df: f3 0f 58 c3                 	addss	%xmm3, %xmm0
1412156e3: 0f 28 da                    	movaps	%xmm2, %xmm3
1412156e6: 0f c6 d9 01                 	shufps	$0x1, %xmm1, %xmm3      # xmm3 = xmm3[1,0],xmm1[0,0]
1412156ea: 0f c6 d9 e2                 	shufps	$0xe2, %xmm1, %xmm3     # xmm3 = xmm3[2,0],xmm1[2,3]
1412156ee: 0f 59 db                    	mulps	%xmm3, %xmm3
1412156f1: 0f c6 d1 10                 	shufps	$0x10, %xmm1, %xmm2     # xmm2 = xmm2[0,0],xmm1[1,0]
1412156f5: 0f c6 d1 e2                 	shufps	$0xe2, %xmm1, %xmm2     # xmm2 = xmm2[2,0],xmm1[2,3]
1412156f9: 0f 59 d2                    	mulps	%xmm2, %xmm2
1412156fc: 0f 58 d3                    	addps	%xmm3, %xmm2
1412156ff: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
141215703: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
141215707: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
14121570b: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14121570f: 66 0f 7e c0                 	movd	%xmm0, %eax
141215713: 25 ff ff ff 7f              	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
141215718: 3d ff ff 7f 7f              	cmpl	$0x7f7fffff, %eax       # imm = 0x7F7FFFFF
14121571d: 0f 8e 99 19 00 00           	jle	0x1412170bc <.text+0x12160bc>
141215723: 48 c7 85 08 03 00 00 00 00 00 00    	movq	$0x0, 0x308(%rbp)
14121572e: 4c 8d 3c f7                 	leaq	(%rdi,%rsi,8), %r15
141215732: 41 b0 01                    	movb	$0x1, %r8b
141215735: 45 31 e4                    	xorl	%r12d, %r12d
141215738: f3 0f 10 35 a0 6d a0 01     	movss	0x1a06da0(%rip), %xmm6  # 0x142c1c4e0
141215740: f3 0f 10 3d 9c 6d a0 01     	movss	0x1a06d9c(%rip), %xmm7  # 0x142c1c4e4
141215748: 48 8d 15 09 70 a0 01        	leaq	0x1a07009(%rip), %rdx   # 0x142c1c758
14121574f: 48 8d 9d c0 02 00 00        	leaq	0x2c0(%rbp), %rbx
141215756: 4c 39 ff                    	cmpq	%r15, %rdi
141215759: 0f 84 ba 01 00 00           	je	0x141215919 <.text+0x1214919>
14121575f: 48 8d 47 08                 	leaq	0x8(%rdi), %rax
141215763: 41 f6 c0 01                 	testb	$0x1, %r8b
141215767: 74 27                       	je	0x141215790 <.text+0x1214790>
141215769: 4c 39 f8                    	cmpq	%r15, %rax
14121576c: 0f 84 a7 01 00 00           	je	0x141215919 <.text+0x1214919>
141215772: 4c 8d 6f 10                 	leaq	0x10(%rdi), %r13
141215776: 4d 39 fd                    	cmpq	%r15, %r13
141215779: 0f 84 9a 01 00 00           	je	0x141215919 <.text+0x1214919>
14121577f: 4c 8d 4f 18                 	leaq	0x18(%rdi), %r9
141215783: 48 89 fe                    	movq	%rdi, %rsi
141215786: eb 11                       	jmp	0x141215799 <.text+0x1214799>
141215788: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141215790: 49 89 c1                    	movq	%rax, %r9
141215793: 4c 89 e8                    	movq	%r13, %rax
141215796: 49 89 fd                    	movq	%rdi, %r13
141215799: f2 0f 10 06                 	movsd	(%rsi), %xmm0
14121579d: f2 0f 10 08                 	movsd	(%rax), %xmm1
1412157a1: f2 41 0f 10 55 00           	movsd	(%r13), %xmm2
1412157a7: 0f 5c d1                    	subps	%xmm1, %xmm2
1412157aa: 0f 5c c8                    	subps	%xmm0, %xmm1
1412157ad: 0f 28 d9                    	movaps	%xmm1, %xmm3
1412157b0: 0f 59 da                    	mulps	%xmm2, %xmm3
1412157b3: f3 0f 16 c3                 	movshdup	%xmm3, %xmm0            # xmm0 = xmm3[1,1,3,3]
1412157b7: f3 0f 58 c3                 	addss	%xmm3, %xmm0
1412157bb: 0f 28 d9                    	movaps	%xmm1, %xmm3
1412157be: 0f 14 da                    	unpcklps	%xmm2, %xmm3            # xmm3 = xmm3[0],xmm2[0],xmm3[1],xmm2[1]
1412157c1: 0f 59 db                    	mulps	%xmm3, %xmm3
1412157c4: 0f c6 d1 11                 	shufps	$0x11, %xmm1, %xmm2     # xmm2 = xmm2[1,0],xmm1[1,0]
1412157c8: 0f c6 d1 e2                 	shufps	$0xe2, %xmm1, %xmm2     # xmm2 = xmm2[2,0],xmm1[2,3]
1412157cc: 0f 59 d2                    	mulps	%xmm2, %xmm2
1412157cf: 0f 58 d3                    	addps	%xmm3, %xmm2
1412157d2: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
1412157d6: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
1412157da: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
1412157de: f3 0f 5e c1                 	divss	%xmm1, %xmm0
1412157e2: 66 41 0f 7e c0              	movd	%xmm0, %r8d
1412157e7: 41 81 e0 ff ff ff 7f        	andl	$0x7fffffff, %r8d       # imm = 0x7FFFFFFF
1412157ee: 41 81 f8 ff ff 7f 7f        	cmpl	$0x7f7fffff, %r8d       # imm = 0x7F7FFFFF
1412157f5: 0f 8e b5 00 00 00           	jle	0x1412158b0 <.text+0x12148b0>
1412157fb: 49 83 c4 02                 	addq	$0x2, %r12
1412157ff: eb 1c                       	jmp	0x14121581d <.text+0x121481d>
141215801: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141215810: 4d 8d 48 08                 	leaq	0x8(%r8), %r9
141215814: 49 ff c4                    	incq	%r12
141215817: 4c 89 e8                    	movq	%r13, %rax
14121581a: 4d 89 c5                    	movq	%r8, %r13
14121581d: 4d 39 f9                    	cmpq	%r15, %r9
141215820: 0f 84 f3 00 00 00           	je	0x141215919 <.text+0x1214919>
141215826: 4d 89 c8                    	movq	%r9, %r8
141215829: f2 0f 10 00                 	movsd	(%rax), %xmm0
14121582d: f2 41 0f 10 4d 00           	movsd	(%r13), %xmm1
141215833: f2 41 0f 10 11              	movsd	(%r9), %xmm2
141215838: 0f 5c d1                    	subps	%xmm1, %xmm2
14121583b: 0f 5c c8                    	subps	%xmm0, %xmm1
14121583e: 0f 28 d9                    	movaps	%xmm1, %xmm3
141215841: 0f 59 da                    	mulps	%xmm2, %xmm3
141215844: f3 0f 16 c3                 	movshdup	%xmm3, %xmm0            # xmm0 = xmm3[1,1,3,3]
141215848: f3 0f 58 c3                 	addss	%xmm3, %xmm0
14121584c: 0f 28 d9                    	movaps	%xmm1, %xmm3
14121584f: 0f 14 da                    	unpcklps	%xmm2, %xmm3            # xmm3 = xmm3[0],xmm2[0],xmm3[1],xmm2[1]
141215852: 0f 59 db                    	mulps	%xmm3, %xmm3
141215855: 0f c6 d1 11                 	shufps	$0x11, %xmm1, %xmm2     # xmm2 = xmm2[1,0],xmm1[1,0]
141215859: 0f c6 d1 e2                 	shufps	$0xe2, %xmm1, %xmm2     # xmm2 = xmm2[2,0],xmm1[2,3]
14121585d: 0f 59 d2                    	mulps	%xmm2, %xmm2
141215860: 0f 58 d3                    	addps	%xmm3, %xmm2
141215863: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
141215867: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
14121586b: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
14121586f: f3 0f 5e c1                 	divss	%xmm1, %xmm0
141215873: 66 0f 7e c0                 	movd	%xmm0, %eax
141215877: 25 ff ff ff 7f              	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
14121587c: 3d ff ff 7f 7f              	cmpl	$0x7f7fffff, %eax       # imm = 0x7F7FFFFF
141215881: 7f 8d                       	jg	0x141215810 <.text+0x1214810>
141215883: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
141215886: 72 88                       	jb	0x141215810 <.text+0x1214810>
141215888: 0f 2e f8                    	ucomiss	%xmm0, %xmm7
14121588b: 76 83                       	jbe	0x141215810 <.text+0x1214810>
14121588d: 49 8d 78 08                 	leaq	0x8(%r8), %rdi
141215891: 4c 89 ee                    	movq	%r13, %rsi
141215894: 4d 89 c5                    	movq	%r8, %r13
141215897: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
14121589e: 48 3b 85 c0 02 00 00        	cmpq	0x2c0(%rbp), %rax
1412158a5: 74 34                       	je	0x1412158db <.text+0x12148db>
1412158a7: eb 41                       	jmp	0x1412158ea <.text+0x12148ea>
1412158a9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1412158b0: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
1412158b3: 0f 82 42 ff ff ff           	jb	0x1412157fb <.text+0x12147fb>
1412158b9: 0f 2e f8                    	ucomiss	%xmm0, %xmm7
1412158bc: 0f 86 39 ff ff ff           	jbe	0x1412157fb <.text+0x12147fb>
1412158c2: 49 ff c4                    	incq	%r12
1412158c5: 4c 89 cf                    	movq	%r9, %rdi
1412158c8: 48 89 c6                    	movq	%rax, %rsi
1412158cb: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
1412158d2: 48 3b 85 c0 02 00 00        	cmpq	0x2c0(%rbp), %rax
1412158d9: 75 0f                       	jne	0x1412158ea <.text+0x12148ea>
1412158db: 48 89 d9                    	movq	%rbx, %rcx
1412158de: e8 0d c5 96 ff              	callq	0x140b81df0 <.text+0xb80df0>
1412158e3: 48 8d 15 6e 6e a0 01        	leaq	0x1a06e6e(%rip), %rdx   # 0x142c1c758
1412158ea: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1412158f1: 4c 8b 85 08 03 00 00        	movq	0x308(%rbp), %r8
1412158f8: 4e 89 24 c0                 	movq	%r12, (%rax,%r8,8)
1412158fc: 49 ff c0                    	incq	%r8
1412158ff: 4c 89 85 08 03 00 00        	movq	%r8, 0x308(%rbp)
141215906: 4c 89 85 d0 02 00 00        	movq	%r8, 0x2d0(%rbp)
14121590d: 45 31 c0                    	xorl	%r8d, %r8d
141215910: 4c 39 ff                    	cmpq	%r15, %rdi
141215913: 0f 85 46 fe ff ff           	jne	0x14121575f <.text+0x121475f>
141215919: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
141215920: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
141215927: 48 8b 8d c8 02 00 00        	movq	0x2c8(%rbp), %rcx
14121592e: 48 c7 85 18 02 00 00 00 00 00 00    	movq	$0x0, 0x218(%rbp)
141215939: 48 c7 85 20 02 00 00 04 00 00 00    	movq	$0x4, 0x220(%rbp)
141215944: 48 c7 85 28 02 00 00 00 00 00 00    	movq	$0x0, 0x228(%rbp)
14121594f: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141215956: 48 8d 04 c1                 	leaq	(%rcx,%rax,8), %rax
14121595a: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
141215961: 48 8b 85 58 04 00 00        	movq	0x458(%rbp), %rax
141215968: 44 0f b6 40 14              	movzbl	0x14(%rax), %r8d
14121596d: 48 8d 40 15                 	leaq	0x15(%rax), %rax
141215971: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
141215978: f3 44 0f 10 15 83 ff 70 01  	movss	0x170ff83(%rip), %xmm10 # 0x142925904
141215981: f3 44 0f 10 25 52 47 74 01  	movss	0x1744752(%rip), %xmm12 # 0x14295a0dc
14121598a: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
14121598e: f3 44 0f 10 3d 89 77 7b 01  	movss	0x17b7789(%rip), %xmm15 # 0x1429cd120
141215997: f3 0f 10 35 d1 fe 70 01     	movss	0x170fed1(%rip), %xmm6  # 0x142925870
14121599f: 4c 8d a5 c0 02 00 00        	leaq	0x2c0(%rbp), %r12
1412159a6: 48 8d bd 10 01 00 00        	leaq	0x110(%rbp), %rdi
1412159ad: 48 89 8d f8 01 00 00        	movq	%rcx, 0x1f8(%rbp)
1412159b4: 44 88 85 16 03 00 00        	movb	%r8b, 0x316(%rbp)
1412159bb: eb 13                       	jmp	0x1412159d0 <.text+0x12149d0>
1412159bd: 0f 1f 00                    	nopl	(%rax)
1412159c0: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1412159c7: f3 44 0f 10 25 0c 47 74 01  	movss	0x174470c(%rip), %xmm12 # 0x14295a0dc
1412159d0: 48 89 8d 38 02 00 00        	movq	%rcx, 0x238(%rbp)
1412159d7: 48 3b 8d e8 01 00 00        	cmpq	0x1e8(%rbp), %rcx
1412159de: 0f 84 e1 15 00 00           	je	0x141216fc5 <.text+0x1215fc5>
1412159e4: 49 8b 56 10                 	movq	0x10(%r14), %rdx
1412159e8: 48 85 d2                    	testq	%rdx, %rdx
1412159eb: 0f 84 c5 17 00 00           	je	0x1412171b6 <.text+0x12161b6>
1412159f1: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
1412159f8: 4c 8b 38                    	movq	(%rax), %r15
1412159fb: 48 8d 42 fe                 	leaq	-0x2(%rdx), %rax
1412159ff: 4c 89 f9                    	movq	%r15, %rcx
141215a02: 48 83 e9 01                 	subq	$0x1, %rcx
141215a06: 48 0f 42 c8                 	cmovbq	%rax, %rcx
141215a0a: 49 39 d7                    	cmpq	%rdx, %r15
141215a0d: 0f 83 7b 17 00 00           	jae	0x14121718e <.text+0x121618e>
141215a13: 49 8d 47 01                 	leaq	0x1(%r15), %rax
141215a17: 48 39 d0                    	cmpq	%rdx, %rax
141215a1a: 41 b9 00 00 00 00           	movl	$0x0, %r9d
141215a20: 49 0f 44 c1                 	cmoveq	%r9, %rax
141215a24: 48 39 d1                    	cmpq	%rdx, %rcx
141215a27: 0f 83 35 17 00 00           	jae	0x141217162 <.text+0x1216162>
141215a2d: 4d 39 7e 28                 	cmpq	%r15, 0x28(%r14)
141215a31: 0f 86 1a 17 00 00           	jbe	0x141217151 <.text+0x1216151>
141215a37: 49 8b 56 08                 	movq	0x8(%r14), %rdx
141215a3b: f2 46 0f 10 2c fa           	movsd	(%rdx,%r15,8), %xmm13
141215a41: f2 44 0f 10 1c c2           	movsd	(%rdx,%rax,8), %xmm11
141215a47: f2 0f 10 04 ca              	movsd	(%rdx,%rcx,8), %xmm0
141215a4c: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
141215a53: 49 8b 46 20                 	movq	0x20(%r14), %rax
141215a57: f3 46 0f 10 0c b8           	movss	(%rax,%r15,4), %xmm9
141215a5d: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
141215a64: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141215a68: e8 a3 31 fe ff              	callq	0x1411f8c10 <.text+0x11f7c10>
141215a6d: 41 89 c5                    	movl	%eax, %r13d
141215a70: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
141215a77: 49 39 d5                    	cmpq	%rdx, %r13
141215a7a: 0f 83 22 17 00 00           	jae	0x1412171a2 <.text+0x12161a2>
141215a80: 49 8d 5d 01                 	leaq	0x1(%r13), %rbx
141215a84: 48 39 d3                    	cmpq	%rdx, %rbx
141215a87: 0f 83 e6 16 00 00           	jae	0x141217173 <.text+0x1216173>
141215a8d: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
141215a91: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
141215a98: f3 42 0f 10 04 a8           	movss	(%rax,%r13,4), %xmm0
141215a9e: f3 42 0f 10 4c a8 04        	movss	0x4(%rax,%r13,4), %xmm1
141215aa5: f3 0f 5c c8                 	subss	%xmm0, %xmm1
141215aa9: f3 41 0f 59 c8              	mulss	%xmm8, %xmm1
141215aae: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141215ab2: f3 45 0f 59 4e 30           	mulss	0x30(%r14), %xmm9
141215ab8: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
141215abc: e8 bf 60 e2 fe              	callq	0x14003bb80 <.text+0x3ab80>
141215ac1: f3 0f 11 85 4c 02 00 00     	movss	%xmm0, 0x24c(%rbp)
141215ac9: f3 0f 11 8d 08 03 00 00     	movss	%xmm1, 0x308(%rbp)
141215ad1: 48 8b 95 38 04 00 00        	movq	0x438(%rbp), %rdx
141215ad8: 80 bd 50 04 00 00 00        	cmpb	$0x0, 0x450(%rbp)
141215adf: 74 3f                       	je	0x141215b20 <.text+0x1214b20>
141215ae1: 49 39 d5                    	cmpq	%rdx, %r13
141215ae4: 0f 83 f1 16 00 00           	jae	0x1412171db <.text+0x12161db>
141215aea: 48 39 d3                    	cmpq	%rdx, %rbx
141215aed: 0f 83 d4 16 00 00           	jae	0x1412171c7 <.text+0x12161c7>
141215af3: 48 8b 85 30 04 00 00        	movq	0x430(%rbp), %rax
141215afa: f3 42 0f 10 04 a8           	movss	(%rax,%r13,4), %xmm0
141215b00: f3 42 0f 10 4c a8 04        	movss	0x4(%rax,%r13,4), %xmm1
141215b07: f3 0f 5c c8                 	subss	%xmm0, %xmm1
141215b0b: f3 44 0f 59 c1              	mulss	%xmm1, %xmm8
141215b10: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
141215b15: f3 44 0f 58 05 32 97 86 01  	addss	0x1869732(%rip), %xmm8  # 0x142a7f250
141215b1e: eb 3d                       	jmp	0x141215b5d <.text+0x1214b5d>
141215b20: 4d 85 ff                    	testq	%r15, %r15
141215b23: 0f 84 e2 13 00 00           	je	0x141216f0b <.text+0x1215f0b>
141215b29: 49 39 d5                    	cmpq	%rdx, %r13
141215b2c: 0f 83 bd 16 00 00           	jae	0x1412171ef <.text+0x12161ef>
141215b32: 48 39 d3                    	cmpq	%rdx, %rbx
141215b35: 0f 83 c8 16 00 00           	jae	0x141217203 <.text+0x1216203>
141215b3b: 48 8b 85 30 04 00 00        	movq	0x430(%rbp), %rax
141215b42: f3 42 0f 10 04 a8           	movss	(%rax,%r13,4), %xmm0
141215b48: f3 42 0f 10 4c a8 04        	movss	0x4(%rax,%r13,4), %xmm1
141215b4f: f3 0f 5c c8                 	subss	%xmm0, %xmm1
141215b53: f3 44 0f 59 c1              	mulss	%xmm1, %xmm8
141215b58: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
141215b5d: f3 44 0f 11 85 10 03 00 00  	movss	%xmm8, 0x310(%rbp)
141215b66: 48 8b 8d 48 04 00 00        	movq	0x448(%rbp), %rcx
141215b6d: e8 ee 1b a2 ff              	callq	0x140c37760 <.text+0xc36760>
141215b72: 48 85 d2                    	testq	%rdx, %rdx
141215b75: 0f 84 95 00 00 00           	je	0x141215c10 <.text+0x1214c10>
141215b7b: 48 c1 e2 02                 	shlq	$0x2, %rdx
141215b7f: 48 8d 0c 92                 	leaq	(%rdx,%rdx,4), %rcx
141215b83: 31 d2                       	xorl	%edx, %edx
141215b85: 45 31 db                    	xorl	%r11d, %r11d
141215b88: 48 c7 c6 ff ff ff ff        	movq	$-0x1, %rsi
141215b8f: f3 0f 10 95 4c 02 00 00     	movss	0x24c(%rbp), %xmm2
141215b97: eb 10                       	jmp	0x141215ba9 <.text+0x1214ba9>
141215b99: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141215ba0: 48 83 c2 14                 	addq	$0x14, %rdx
141215ba4: 48 39 d1                    	cmpq	%rdx, %rcx
141215ba7: 74 71                       	je	0x141215c1a <.text+0x1214c1a>
141215ba9: 0f 28 c2                    	movaps	%xmm2, %xmm0
141215bac: f3 0f 5c 04 10              	subss	(%rax,%rdx), %xmm0
141215bb1: 0f 54 05 c8 8d 71 01        	andps	0x1718dc8(%rip), %xmm0  # 0x14292e980
141215bb8: f3 0f 10 0d b4 aa 74 01     	movss	0x174aab4(%rip), %xmm1  # 0x142960674
141215bc0: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
141215bc3: 76 db                       	jbe	0x141215ba0 <.text+0x1214ba0>
141215bc5: f3 0f 10 44 10 04           	movss	0x4(%rax,%rdx), %xmm0
141215bcb: f3 4c 0f 2c c0              	cvttss2si	%xmm0, %r8
141215bd0: 4d 89 c1                    	movq	%r8, %r9
141215bd3: 49 c1 f9 3f                 	sarq	$0x3f, %r9
141215bd7: 0f 28 c8                    	movaps	%xmm0, %xmm1
141215bda: f3 41 0f 5c cc              	subss	%xmm12, %xmm1
141215bdf: f3 4c 0f 2c d1              	cvttss2si	%xmm1, %r10
141215be4: 4d 21 ca                    	andq	%r9, %r10
141215be7: 4d 09 c2                    	orq	%r8, %r10
141215bea: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
141215bee: 41 b8 00 00 00 00           	movl	$0x0, %r8d
141215bf4: 4d 0f 43 c2                 	cmovaeq	%r10, %r8
141215bf8: 0f 2e 05 e1 44 74 01        	ucomiss	0x17444e1(%rip), %xmm0  # 0x14295a0e0
141215bff: 4c 0f 47 c6                 	cmovaq	%rsi, %r8
141215c03: 4d 01 c3                    	addq	%r8, %r11
141215c06: eb 98                       	jmp	0x141215ba0 <.text+0x1214ba0>
141215c08: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141215c10: 45 31 db                    	xorl	%r11d, %r11d
141215c13: 48 c7 c6 ff ff ff ff        	movq	$-0x1, %rsi
141215c1a: 4c 89 9d d8 01 00 00        	movq	%r11, 0x1d8(%rbp)
141215c21: 4d 01 df                    	addq	%r11, %r15
141215c24: 4c 89 bd b0 01 00 00        	movq	%r15, 0x1b0(%rbp)
141215c2b: f3 44 0f 10 85 10 03 00 00  	movss	0x310(%rbp), %xmm8
141215c34: f3 44 0f 5c 85 08 03 00 00  	subss	0x308(%rbp), %xmm8
141215c3d: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
141215c41: f3 0f 5e 05 33 6c 86 01     	divss	0x1866c33(%rip), %xmm0  # 0x142a7c87c
141215c49: e8 12 e2 70 01              	callq	0x142923e60 <.text+0x2922e60>
141215c4e: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
141215c53: 48 89 c1                    	movq	%rax, %rcx
141215c56: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
141215c5a: 0f 28 c8                    	movaps	%xmm0, %xmm1
141215c5d: f3 41 0f 5c cc              	subss	%xmm12, %xmm1
141215c62: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
141215c67: 48 21 ca                    	andq	%rcx, %rdx
141215c6a: 48 09 c2                    	orq	%rax, %rdx
141215c6d: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
141215c71: 41 bf 00 00 00 00           	movl	$0x0, %r15d
141215c77: 4c 0f 43 fa                 	cmovaeq	%rdx, %r15
141215c7b: 0f 2e 05 5e 44 74 01        	ucomiss	0x174445e(%rip), %xmm0  # 0x14295a0e0
141215c82: 4c 0f 47 fe                 	cmovaq	%rsi, %r15
141215c86: 49 83 ff 03                 	cmpq	$0x3, %r15
141215c8a: b8 02 00 00 00              	movl	$0x2, %eax
141215c8f: 4c 0f 42 f8                 	cmovbq	%rax, %r15
141215c93: f3 0f 10 15 cd 39 87 01     	movss	0x18739cd(%rip), %xmm2  # 0x142a89668
141215c9b: f3 41 0f 5e d0              	divss	%xmm8, %xmm2
141215ca0: 48 8d 05 91 69 a0 01        	leaq	0x1a06991(%rip), %rax   # 0x142c1c638
141215ca7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141215cac: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
141215cb3: 4c 89 fa                    	movq	%r15, %rdx
141215cb6: 4c 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %r9
141215cbd: e8 8e a6 a7 ff              	callq	0x140c90350 <.text+0xc8f350>
141215cc2: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
141215cc9: 48 83 c0 08                 	addq	$0x8, %rax
141215ccd: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
141215cd4: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
141215cd8: 45 0f 5c dd                 	subps	%xmm13, %xmm11
141215cdc: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
141215ce0: 41 0f 59 c3                 	mulps	%xmm11, %xmm0
141215ce4: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
141215ce8: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141215cec: 0f 57 c0                    	xorps	%xmm0, %xmm0
141215cef: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
141215cf3: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
141215cf7: f3 0f 5e c8                 	divss	%xmm0, %xmm1
141215cfb: f3 41 0f 16 c3              	movshdup	%xmm11, %xmm0           # xmm0 = xmm11[1,1,3,3]
141215d00: f3 44 0f 59 d9              	mulss	%xmm1, %xmm11
141215d05: 44 0f 29 5d 20              	movaps	%xmm11, 0x20(%rbp)
141215d0a: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
141215d0e: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
141215d12: 0f 5c 95 90 01 00 00        	subps	0x190(%rbp), %xmm2
141215d19: 0f 28 c2                    	movaps	%xmm2, %xmm0
141215d1c: 0f 59 c2                    	mulps	%xmm2, %xmm0
141215d1f: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
141215d23: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141215d27: 0f 57 c0                    	xorps	%xmm0, %xmm0
141215d2a: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
141215d2e: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
141215d32: f3 0f 5e c8                 	divss	%xmm0, %xmm1
141215d36: f3 0f 16 c2                 	movshdup	%xmm2, %xmm0            # xmm0 = xmm2[1,1,3,3]
141215d3a: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
141215d3e: 0f 29 55 40                 	movaps	%xmm2, 0x40(%rbp)
141215d42: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
141215d46: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
141215d4a: 4c 8b ad b8 00 00 00        	movq	0xb8(%rbp), %r13
141215d51: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
141215d58: 48 8d 04 85 00 00 00 00     	leaq	(,%rax,4), %rax
141215d60: 4c 01 e8                    	addq	%r13, %rax
141215d63: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
141215d6a: 49 83 c7 fe                 	addq	$-0x2, %r15
141215d6e: 48 c7 85 88 02 00 00 00 00 00 00    	movq	$0x0, 0x288(%rbp)
141215d79: 4c 89 ad e0 01 00 00        	movq	%r13, 0x1e0(%rbp)
141215d80: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
141215d8b: 4c 89 bd 40 02 00 00        	movq	%r15, 0x240(%rbp)
141215d92: eb 42                       	jmp	0x141215dd6 <.text+0x1214dd6>
141215d94: 4c 8b bd 40 02 00 00        	movq	0x240(%rbp), %r15
141215d9b: 49 c1 e0 03                 	shlq	$0x3, %r8
141215d9f: 4b 8d 14 80                 	leaq	(%r8,%r8,4), %rdx
141215da3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141215da9: e8 72 de 3f ff              	callq	0x140613c20 <.text+0x612c20>
141215dae: b0 01                       	movb	$0x1, %al
141215db0: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
141215db7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141215dc0: f3 44 0f 10 15 3b fb 70 01  	movss	0x170fb3b(%rip), %xmm10 # 0x142925904
141215dc9: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
141215dcd: f3 44 0f 10 3d 4a 73 7b 01  	movss	0x17b734a(%rip), %xmm15 # 0x1429cd120
141215dd6: 48 8b b5 90 01 00 00        	movq	0x190(%rbp), %rsi
141215ddd: 4c 3b ad b8 01 00 00        	cmpq	0x1b8(%rbp), %r13
141215de4: 0f 84 f6 10 00 00           	je	0x141216ee0 <.text+0x1215ee0>
141215dea: 49 8d 45 04                 	leaq	0x4(%r13), %rax
141215dee: f3 45 0f 10 65 00           	movss	(%r13), %xmm12
141215df4: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
141215df8: f3 41 0f 5c c4              	subss	%xmm12, %xmm0
141215dfd: f3 0f 59 85 08 03 00 00     	mulss	0x308(%rbp), %xmm0
141215e05: f3 44 0f 59 a5 10 03 00 00  	mulss	0x310(%rbp), %xmm12
141215e0e: f3 44 0f 58 e0              	addss	%xmm0, %xmm12
141215e13: f6 85 88 02 00 00 01        	testb	$0x1, 0x288(%rbp)
141215e1a: 74 14                       	je	0x141215e30 <.text+0x1214e30>
141215e1c: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
141215e20: 49 89 c5                    	movq	%rax, %r13
141215e23: 44 0f 28 e7                 	movaps	%xmm7, %xmm12
141215e27: eb 3a                       	jmp	0x141215e63 <.text+0x1214e63>
141215e29: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141215e30: 48 3b 85 b8 01 00 00        	cmpq	0x1b8(%rbp), %rax
141215e37: 0f 84 a3 10 00 00           	je	0x141216ee0 <.text+0x1215ee0>
141215e3d: f3 41 0f 10 45 04           	movss	0x4(%r13), %xmm0
141215e43: 49 83 c5 08                 	addq	$0x8, %r13
141215e47: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
141215e4b: f3 0f 5c c8                 	subss	%xmm0, %xmm1
141215e4f: f3 0f 59 8d 08 03 00 00     	mulss	0x308(%rbp), %xmm1
141215e57: f3 0f 59 85 10 03 00 00     	mulss	0x310(%rbp), %xmm0
141215e5f: f3 0f 58 c1                 	addss	%xmm1, %xmm0
141215e63: 0f 28 f8                    	movaps	%xmm0, %xmm7
141215e66: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
141215e6d: e8 0e 95 a9 ff              	callq	0x140caf380 <.text+0xcae380>
141215e72: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141215e76: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
141215e7a: f3 41 0f 5c c5              	subss	%xmm13, %xmm0
141215e7f: f3 41 0f 59 c7              	mulss	%xmm15, %xmm0
141215e84: f3 45 0f 59 ef              	mulss	%xmm15, %xmm13
141215e89: f3 44 0f 5c e8              	subss	%xmm0, %xmm13
141215e8e: f3 44 0f 58 2d d9 73 86 01  	addss	0x18673d9(%rip), %xmm13 # 0x142a7d270
141215e97: 4c 39 fe                    	cmpq	%r15, %rsi
141215e9a: 75 0a                       	jne	0x141215ea6 <.text+0x1214ea6>
141215e9c: 41 83 be 58 02 00 00 02     	cmpl	$0x2, 0x258(%r14)
141215ea4: 75 0a                       	jne	0x141215eb0 <.text+0x1214eb0>
141215ea6: 31 db                       	xorl	%ebx, %ebx
141215ea8: eb 11                       	jmp	0x141215ebb <.text+0x1214ebb>
141215eaa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141215eb0: b3 01                       	movb	$0x1, %bl
141215eb2: f3 44 0f 10 2d 2d 66 a0 01  	movss	0x1a0662d(%rip), %xmm13 # 0x142c1c4e8
141215ebb: 48 8d 46 01                 	leaq	0x1(%rsi), %rax
141215ebf: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
141215ec6: 44 0f 28 cf                 	movaps	%xmm7, %xmm9
141215eca: f3 45 0f 5c cc              	subss	%xmm12, %xmm9
141215ecf: 45 0f 2e f1                 	ucomiss	%xmm9, %xmm14
141215ed3: b0 01                       	movb	$0x1, %al
141215ed5: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
141215edc: 0f 83 f4 fe ff ff           	jae	0x141215dd6 <.text+0x1214dd6>
141215ee2: 45 0f 2e f5                 	ucomiss	%xmm13, %xmm14
141215ee6: 0f 83 ea fe ff ff           	jae	0x141215dd6 <.text+0x1214dd6>
141215eec: 80 bd 16 03 00 00 02        	cmpb	$0x2, 0x316(%rbp)
141215ef3: 74 17                       	je	0x141215f0c <.text+0x1214f0c>
141215ef5: 4c 89 f1                    	movq	%r14, %rcx
141215ef8: e8 23 47 91 ff              	callq	0x140b2a620 <.text+0xb29620>
141215efd: 44 0f 2e e0                 	ucomiss	%xmm0, %xmm12
141215f01: 72 09                       	jb	0x141215f0c <.text+0x1214f0c>
141215f03: f3 44 0f 10 2d e0 7a 7c 01  	movss	0x17c7ae0(%rip), %xmm13 # 0x1429dd9ec
141215f0c: 31 c0                       	xorl	%eax, %eax
141215f0e: 40 f6 c6 01                 	testb	$0x1, %sil
141215f12: 0f 94 c0                    	sete	%al
141215f15: 48 8d 0d 54 8a 71 01        	leaq	0x1718a54(%rip), %rcx   # 0x14292e970
141215f1c: f3 44 0f 10 34 81           	movss	(%rcx,%rax,4), %xmm14
141215f22: 45 0f 28 dd                 	movaps	%xmm13, %xmm11
141215f26: f3 44 0f 59 de              	mulss	%xmm6, %xmm11
141215f2b: 45 0f 28 c3                 	movaps	%xmm11, %xmm8
141215f2f: f3 44 0f 58 05 dc 08 8c 01  	addss	0x18c08dc(%rip), %xmm8  # 0x142ad6814
141215f38: f3 44 0f 58 05 ab 65 a0 01  	addss	0x1a065ab(%rip), %xmm8  # 0x142c1c4ec
141215f41: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
141215f45: f3 41 0f 59 c8              	mulss	%xmm8, %xmm1
141215f4a: 8b 85 80 02 00 00           	movl	0x280(%rbp), %eax
141215f50: 89 85 c0 02 00 00           	movl	%eax, 0x2c0(%rbp)
141215f56: f3 0f 10 85 84 02 00 00     	movss	0x284(%rbp), %xmm0
141215f5e: f3 0f 11 85 c4 02 00 00     	movss	%xmm0, 0x2c4(%rbp)
141215f66: f3 0f 58 8d 4c 02 00 00     	addss	0x24c(%rbp), %xmm1
141215f6e: 4c 89 e1                    	movq	%r12, %rcx
141215f71: e8 3a bf a7 ff              	callq	0x140c91eb0 <.text+0xc90eb0>
141215f76: 44 0f 54 35 72 f9 70 01     	andps	0x170f972(%rip), %xmm14 # 0x1429258f0
141215f7e: f3 45 0f 59 f0              	mulss	%xmm8, %xmm14
141215f83: f3 44 0f 58 b5 08 03 00 00  	addss	0x308(%rbp), %xmm14
141215f8c: f3 44 0f 58 e7              	addss	%xmm7, %xmm12
141215f91: f3 44 0f 59 e6              	mulss	%xmm6, %xmm12
141215f96: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
141215f9a: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
141215f9e: e8 3d c6 a7 ff              	callq	0x140c925e0 <.text+0xc915e0>
141215fa3: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
141215fa7: 44 0f 28 f1                 	movaps	%xmm1, %xmm14
141215fab: f3 44 0f 11 4c 24 20        	movss	%xmm9, 0x20(%rsp)
141215fb2: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
141215fb9: 0f 28 c8                    	movaps	%xmm0, %xmm1
141215fbc: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
141215fc0: 41 0f 28 dd                 	movaps	%xmm13, %xmm3
141215fc4: e8 27 a0 02 00              	callq	0x14123fff0 <.text+0x123eff0>
141215fc9: 0f b6 05 39 ec 41 02        	movzbl	0x241ec39(%rip), %eax   # 0x143634c09
141215fd0: b9 28 00 00 00              	movl	$0x28, %ecx
141215fd5: ba 04 00 00 00              	movl	$0x4, %edx
141215fda: e8 31 dc 3f ff              	callq	0x140613c10 <.text+0x612c10>
141215fdf: 48 85 c0                    	testq	%rax, %rax
141215fe2: 0f 84 3a 11 00 00           	je	0x141217122 <.text+0x1216122>
141215fe8: 0f 57 c9                    	xorps	%xmm1, %xmm1
141215feb: 0f 29 8d 90 00 00 00        	movaps	%xmm1, 0x90(%rbp)
141215ff2: 0f 10 85 c8 00 00 00        	movups	0xc8(%rbp), %xmm0
141215ff9: 0f 29 85 c0 02 00 00        	movaps	%xmm0, 0x2c0(%rbp)
141216000: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
141216007: 45 0f 57 d2                 	xorps	%xmm10, %xmm10
14121600b: 0f 11 09                    	movups	%xmm1, (%rcx)
14121600e: 0f 28 85 c0 02 00 00        	movaps	0x2c0(%rbp), %xmm0
141216015: 0f 28 8d d0 02 00 00        	movaps	0x2d0(%rbp), %xmm1
14121601c: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
141216020: 0f 11 00                    	movups	%xmm0, (%rax)
141216023: c7 40 20 00 00 00 00        	movl	$0x0, 0x20(%rax)
14121602a: 66 c7 40 24 00 00           	movw	$0x0, 0x24(%rax)
141216030: 48 c7 85 70 02 00 00 00 00 00 00    	movq	$0x0, 0x270(%rbp)
14121603b: 48 c7 85 78 02 00 00 01 00 00 00    	movq	$0x1, 0x278(%rbp)
141216046: 48 c7 85 60 02 00 00 01 00 00 00    	movq	$0x1, 0x260(%rbp)
141216051: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
141216058: 84 db                       	testb	%bl, %bl
14121605a: 0f 84 f0 03 00 00           	je	0x141216450 <.text+0x1215450>
141216060: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216067: f3 44 0f 11 4c 24 20        	movss	%xmm9, 0x20(%rsp)
14121606e: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216075: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
141216079: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
14121607d: 41 0f 28 dd                 	movaps	%xmm13, %xmm3
141216081: e8 6a 9f 02 00              	callq	0x14123fff0 <.text+0x123eff0>
141216086: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121608d: 48 8b 8d 48 04 00 00        	movq	0x448(%rbp), %rcx
141216094: e8 c7 16 a2 ff              	callq	0x140c37760 <.text+0xc36760>
141216099: 49 89 c7                    	movq	%rax, %r15
14121609c: 48 8d 04 92                 	leaq	(%rdx,%rdx,4), %rax
1412160a0: 49 8d 1c 87                 	leaq	(%r15,%rax,4), %rbx
1412160a4: eb 57                       	jmp	0x1412160fd <.text+0x12150fd>
1412160a6: f3 0f 10 85 9c 02 00 00     	movss	0x29c(%rbp), %xmm0
1412160ae: f3 0f 10 8d c4 02 00 00     	movss	0x2c4(%rbp), %xmm1
1412160b6: 0f 28 d0                    	movaps	%xmm0, %xmm2
1412160b9: f3 0f c2 d0 03              	cmpunordss	%xmm0, %xmm2
1412160be: 0f 28 da                    	movaps	%xmm2, %xmm3
1412160c1: 0f 54 d9                    	andps	%xmm1, %xmm3
1412160c4: f3 0f 5d c8                 	minss	%xmm0, %xmm1
1412160c8: 0f 55 d1                    	andnps	%xmm1, %xmm2
1412160cb: 0f 56 d3                    	orps	%xmm3, %xmm2
1412160ce: f3 0f 11 95 9c 02 00 00     	movss	%xmm2, 0x29c(%rbp)
1412160d6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1412160e0: 49 83 c7 14                 	addq	$0x14, %r15
1412160e4: f3 0f 10 85 94 02 00 00     	movss	0x294(%rbp), %xmm0
1412160ec: f3 0f 10 8d 9c 02 00 00     	movss	0x29c(%rbp), %xmm1
1412160f4: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
1412160f7: 0f 83 3c 05 00 00           	jae	0x141216639 <.text+0x1215639>
1412160fd: 49 39 df                    	cmpq	%rbx, %r15
141216100: 0f 84 23 05 00 00           	je	0x141216629 <.text+0x1215629>
141216106: f3 41 0f 10 0f              	movss	(%r15), %xmm1
14121610b: f3 41 0f 10 57 04           	movss	0x4(%r15), %xmm2
141216111: f3 41 0f 10 5f 08           	movss	0x8(%r15), %xmm3
141216117: f3 41 0f 10 47 0c           	movss	0xc(%r15), %xmm0
14121611d: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216124: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
14121612a: 48 89 f9                    	movq	%rdi, %rcx
14121612d: e8 be 9e 02 00              	callq	0x14123fff0 <.text+0x123eff0>
141216132: f3 44 0f 10 8d 10 01 00 00  	movss	0x110(%rbp), %xmm9
14121613b: f3 44 0f 10 85 18 01 00 00  	movss	0x118(%rbp), %xmm8
141216144: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121614b: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216152: e8 69 9f 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
141216157: 41 0f 2e c1                 	ucomiss	%xmm9, %xmm0
14121615b: 0f 82 cf 00 00 00           	jb	0x141216230 <.text+0x1215230>
141216161: 44 0f 2e c0                 	ucomiss	%xmm0, %xmm8
141216165: 0f 86 c5 00 00 00           	jbe	0x141216230 <.text+0x1215230>
14121616b: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216172: 48 89 f9                    	movq	%rdi, %rcx
141216175: e8 46 9f 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
14121617a: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
14121617e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216185: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
14121618c: e8 2f 9f 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
141216191: 44 0f 2e e9                 	ucomiss	%xmm1, %xmm13
141216195: 76 39                       	jbe	0x1412161d0 <.text+0x12151d0>
141216197: f3 0f 10 85 9c 02 00 00     	movss	0x29c(%rbp), %xmm0
14121619f: f3 0f 10 8d 14 01 00 00     	movss	0x114(%rbp), %xmm1
1412161a7: 0f 28 d0                    	movaps	%xmm0, %xmm2
1412161aa: f3 0f c2 d0 03              	cmpunordss	%xmm0, %xmm2
1412161af: 0f 28 da                    	movaps	%xmm2, %xmm3
1412161b2: 0f 54 d9                    	andps	%xmm1, %xmm3
1412161b5: f3 0f 5d c8                 	minss	%xmm0, %xmm1
1412161b9: 0f 55 d1                    	andnps	%xmm1, %xmm2
1412161bc: 0f 56 d3                    	orps	%xmm3, %xmm2
1412161bf: f3 0f 11 95 9c 02 00 00     	movss	%xmm2, 0x29c(%rbp)
1412161c7: eb 67                       	jmp	0x141216230 <.text+0x1215230>
1412161c9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1412161d0: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412161d7: 48 89 f9                    	movq	%rdi, %rcx
1412161da: e8 e1 9e 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412161df: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
1412161e3: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412161ea: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
1412161f1: e8 ca 9e 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412161f6: 41 0f 2e cd                 	ucomiss	%xmm13, %xmm1
1412161fa: 76 34                       	jbe	0x141216230 <.text+0x1215230>
1412161fc: f3 0f 10 85 94 02 00 00     	movss	0x294(%rbp), %xmm0
141216204: f3 0f 10 8d 1c 01 00 00     	movss	0x11c(%rbp), %xmm1
14121620c: 0f 28 d0                    	movaps	%xmm0, %xmm2
14121620f: f3 0f c2 d0 03              	cmpunordss	%xmm0, %xmm2
141216214: 0f 28 da                    	movaps	%xmm2, %xmm3
141216217: 0f 54 d9                    	andps	%xmm1, %xmm3
14121621a: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14121621e: 0f 55 d1                    	andnps	%xmm1, %xmm2
141216221: 0f 56 d3                    	orps	%xmm3, %xmm2
141216224: f3 0f 11 95 94 02 00 00     	movss	%xmm2, 0x294(%rbp)
14121622c: 0f 1f 40 00                 	nopl	(%rax)
141216230: 41 83 be 58 02 00 00 02     	cmpl	$0x2, 0x258(%r14)
141216238: 0f 84 a2 fe ff ff           	je	0x1412160e0 <.text+0x12150e0>
14121623e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216245: 48 89 f9                    	movq	%rdi, %rcx
141216248: e8 73 9e 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
14121624d: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141216251: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216258: 48 89 f9                    	movq	%rdi, %rcx
14121625b: e8 80 a0 02 00              	callq	0x1412402e0 <.text+0x123f2e0>
141216260: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
141216264: f3 44 0f 58 e8              	addss	%xmm0, %xmm13
141216269: 45 0f 2e 6e 30              	ucomiss	0x30(%r14), %xmm13
14121626e: 0f 86 bc 00 00 00           	jbe	0x141216330 <.text+0x1215330>
141216274: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121627b: 48 89 f9                    	movq	%rdi, %rcx
14121627e: e8 3d 9e 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
141216283: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141216287: f3 45 0f 10 46 30           	movss	0x30(%r14), %xmm8
14121628d: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216294: 48 89 f9                    	movq	%rdi, %rcx
141216297: e8 24 9e 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
14121629c: 0f 28 d1                    	movaps	%xmm1, %xmm2
14121629f: f3 45 0f 5c e8              	subss	%xmm8, %xmm13
1412162a4: f3 41 0f 10 5f 08           	movss	0x8(%r15), %xmm3
1412162aa: f3 41 0f 10 47 0c           	movss	0xc(%r15), %xmm0
1412162b0: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412162b7: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1412162bd: 4c 89 e1                    	movq	%r12, %rcx
1412162c0: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1412162c4: e8 27 9d 02 00              	callq	0x14123fff0 <.text+0x123eff0>
1412162c9: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412162d0: 4c 89 e1                    	movq	%r12, %rcx
1412162d3: e8 e8 9d 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412162d8: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
1412162dc: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412162e3: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
1412162ea: e8 d1 9d 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412162ef: 44 0f 2e e9                 	ucomiss	%xmm1, %xmm13
1412162f3: 0f 87 ad fd ff ff           	ja	0x1412160a6 <.text+0x12150a6>
1412162f9: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216300: 4c 89 e1                    	movq	%r12, %rcx
141216303: e8 b8 9d 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
141216308: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
14121630c: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216313: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
14121631a: e8 a1 9d 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
14121631f: e9 e7 00 00 00              	jmp	0x14121640b <.text+0x121540b>
141216324: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141216330: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216337: 48 89 f9                    	movq	%rdi, %rcx
14121633a: e8 81 9d 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
14121633f: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141216343: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121634a: 48 89 f9                    	movq	%rdi, %rcx
14121634d: e8 8e 9f 02 00              	callq	0x1412402e0 <.text+0x123f2e0>
141216352: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
141216356: 41 0f 2e c5                 	ucomiss	%xmm13, %xmm0
14121635a: 0f 86 80 fd ff ff           	jbe	0x1412160e0 <.text+0x12150e0>
141216360: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216367: 48 89 f9                    	movq	%rdi, %rcx
14121636a: e8 51 9d 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
14121636f: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141216373: f3 45 0f 10 46 30           	movss	0x30(%r14), %xmm8
141216379: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216380: 48 89 f9                    	movq	%rdi, %rcx
141216383: e8 38 9d 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
141216388: 0f 28 d1                    	movaps	%xmm1, %xmm2
14121638b: f3 45 0f 58 e8              	addss	%xmm8, %xmm13
141216390: f3 41 0f 10 5f 08           	movss	0x8(%r15), %xmm3
141216396: f3 41 0f 10 47 0c           	movss	0xc(%r15), %xmm0
14121639c: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412163a3: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1412163a9: 4c 89 e1                    	movq	%r12, %rcx
1412163ac: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1412163b0: e8 3b 9c 02 00              	callq	0x14123fff0 <.text+0x123eff0>
1412163b5: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412163bc: 4c 89 e1                    	movq	%r12, %rcx
1412163bf: e8 fc 9c 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412163c4: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
1412163c8: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412163cf: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
1412163d6: e8 e5 9c 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412163db: 44 0f 2e e9                 	ucomiss	%xmm1, %xmm13
1412163df: 0f 87 c1 fc ff ff           	ja	0x1412160a6 <.text+0x12150a6>
1412163e5: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412163ec: 4c 89 e1                    	movq	%r12, %rcx
1412163ef: e8 cc 9c 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412163f4: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
1412163f8: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412163ff: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216406: e8 b5 9c 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
14121640b: 41 0f 2e cd                 	ucomiss	%xmm13, %xmm1
14121640f: 0f 86 cb fc ff ff           	jbe	0x1412160e0 <.text+0x12150e0>
141216415: f3 0f 10 85 94 02 00 00     	movss	0x294(%rbp), %xmm0
14121641d: f3 0f 10 8d cc 02 00 00     	movss	0x2cc(%rbp), %xmm1
141216425: 0f 28 d0                    	movaps	%xmm0, %xmm2
141216428: f3 0f c2 d0 03              	cmpunordss	%xmm0, %xmm2
14121642d: 0f 28 da                    	movaps	%xmm2, %xmm3
141216430: 0f 54 d9                    	andps	%xmm1, %xmm3
141216433: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
141216437: 0f 55 d1                    	andnps	%xmm1, %xmm2
14121643a: 0f 56 d3                    	orps	%xmm3, %xmm2
14121643d: f3 0f 11 95 94 02 00 00     	movss	%xmm2, 0x294(%rbp)
141216445: e9 96 fc ff ff              	jmp	0x1412160e0 <.text+0x12150e0>
14121644a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141216450: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216457: 48 8b 8d 48 04 00 00        	movq	0x448(%rbp), %rcx
14121645e: e8 fd 12 a2 ff              	callq	0x140c37760 <.text+0xc36760>
141216463: 49 89 c7                    	movq	%rax, %r15
141216466: 48 8d 04 92                 	leaq	(%rdx,%rdx,4), %rax
14121646a: 49 8d 1c 87                 	leaq	(%r15,%rax,4), %rbx
14121646e: eb 04                       	jmp	0x141216474 <.text+0x1215474>
141216470: 49 83 c7 14                 	addq	$0x14, %r15
141216474: 49 39 df                    	cmpq	%rbx, %r15
141216477: 0f 84 c7 02 00 00           	je	0x141216744 <.text+0x1215744>
14121647d: f3 45 0f 10 6f 08           	movss	0x8(%r15), %xmm13
141216483: f3 45 0f 10 4f 0c           	movss	0xc(%r15), %xmm9
141216489: f3 41 0f 10 0f              	movss	(%r15), %xmm1
14121648e: f3 41 0f 10 57 04           	movss	0x4(%r15), %xmm2
141216494: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121649b: f3 44 0f 11 4c 24 20        	movss	%xmm9, 0x20(%rsp)
1412164a2: 48 89 f9                    	movq	%rdi, %rcx
1412164a5: 41 0f 28 dd                 	movaps	%xmm13, %xmm3
1412164a9: e8 42 9b 02 00              	callq	0x14123fff0 <.text+0x123eff0>
1412164ae: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412164b5: 48 89 f9                    	movq	%rdi, %rcx
1412164b8: 48 8d 95 c8 00 00 00        	leaq	0xc8(%rbp), %rdx
1412164bf: e8 ac 9c 02 00              	callq	0x141240170 <.text+0x123f170>
1412164c4: 84 c0                       	testb	%al, %al
1412164c6: 74 16                       	je	0x1412164de <.text+0x12154de>
1412164c8: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412164cf: 48 89 f9                    	movq	%rdi, %rcx
1412164d2: 48 8d 95 60 02 00 00        	leaq	0x260(%rbp), %rdx
1412164d9: e8 e2 ac 02 00              	callq	0x1412411c0 <.text+0x12401c0>
1412164de: 41 83 be 58 02 00 00 02     	cmpl	$0x2, 0x258(%r14)
1412164e6: 74 88                       	je	0x141216470 <.text+0x1215470>
1412164e8: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412164ef: 48 89 f9                    	movq	%rdi, %rcx
1412164f2: e8 c9 9b 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412164f7: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
1412164fb: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216502: 48 89 f9                    	movq	%rdi, %rcx
141216505: e8 d6 9d 02 00              	callq	0x1412402e0 <.text+0x123f2e0>
14121650a: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
14121650e: f3 44 0f 58 f0              	addss	%xmm0, %xmm14
141216513: 45 0f 2e 76 30              	ucomiss	0x30(%r14), %xmm14
141216518: 76 76                       	jbe	0x141216590 <.text+0x1215590>
14121651a: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216521: 48 89 f9                    	movq	%rdi, %rcx
141216524: e8 97 9b 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
141216529: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
14121652d: f3 45 0f 10 46 30           	movss	0x30(%r14), %xmm8
141216533: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121653a: 48 89 f9                    	movq	%rdi, %rcx
14121653d: e8 7e 9b 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
141216542: 0f 28 d1                    	movaps	%xmm1, %xmm2
141216545: f3 45 0f 5c f0              	subss	%xmm8, %xmm14
14121654a: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216551: f3 44 0f 11 4c 24 20        	movss	%xmm9, 0x20(%rsp)
141216558: 4c 89 e1                    	movq	%r12, %rcx
14121655b: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
14121655f: 41 0f 28 dd                 	movaps	%xmm13, %xmm3
141216563: e8 88 9a 02 00              	callq	0x14123fff0 <.text+0x123eff0>
141216568: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121656f: 4c 89 e1                    	movq	%r12, %rcx
141216572: 48 8d 95 60 02 00 00        	leaq	0x260(%rbp), %rdx
141216579: e8 42 ac 02 00              	callq	0x1412411c0 <.text+0x12401c0>
14121657e: e9 ed fe ff ff              	jmp	0x141216470 <.text+0x1215470>
141216583: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141216590: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216597: 48 89 f9                    	movq	%rdi, %rcx
14121659a: e8 21 9b 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
14121659f: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
1412165a3: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412165aa: 48 89 f9                    	movq	%rdi, %rcx
1412165ad: e8 2e 9d 02 00              	callq	0x1412402e0 <.text+0x123f2e0>
1412165b2: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
1412165b6: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
1412165ba: 0f 86 b0 fe ff ff           	jbe	0x141216470 <.text+0x1215470>
1412165c0: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412165c7: 48 89 f9                    	movq	%rdi, %rcx
1412165ca: e8 f1 9a 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412165cf: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
1412165d3: f3 45 0f 10 46 30           	movss	0x30(%r14), %xmm8
1412165d9: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412165e0: 48 89 f9                    	movq	%rdi, %rcx
1412165e3: e8 d8 9a 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
1412165e8: 0f 28 d1                    	movaps	%xmm1, %xmm2
1412165eb: f3 45 0f 58 f0              	addss	%xmm8, %xmm14
1412165f0: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412165f7: f3 44 0f 11 4c 24 20        	movss	%xmm9, 0x20(%rsp)
1412165fe: 4c 89 e1                    	movq	%r12, %rcx
141216601: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
141216605: 41 0f 28 dd                 	movaps	%xmm13, %xmm3
141216609: e8 e2 99 02 00              	callq	0x14123fff0 <.text+0x123eff0>
14121660e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216615: 4c 89 e1                    	movq	%r12, %rcx
141216618: 48 8d 95 60 02 00 00        	leaq	0x260(%rbp), %rdx
14121661f: e8 9c ab 02 00              	callq	0x1412411c0 <.text+0x12401c0>
141216624: e9 47 fe ff ff              	jmp	0x141216470 <.text+0x1215470>
141216629: f3 0f 10 85 94 02 00 00     	movss	0x294(%rbp), %xmm0
141216631: f3 0f 10 8d 9c 02 00 00     	movss	0x29c(%rbp), %xmm1
141216639: f3 0f 5c c8                 	subss	%xmm0, %xmm1
14121663d: 0f 2e 0d 40 f3 70 01        	ucomiss	0x170f340(%rip), %xmm1  # 0x142925984
141216644: 0f 86 b3 00 00 00           	jbe	0x1412166fd <.text+0x12156fd>
14121664a: 0f b6 05 b8 e5 41 02        	movzbl	0x241e5b8(%rip), %eax   # 0x143634c09
141216651: b9 28 00 00 00              	movl	$0x28, %ecx
141216656: ba 04 00 00 00              	movl	$0x4, %edx
14121665b: e8 b0 d5 3f ff              	callq	0x140613c10 <.text+0x612c10>
141216660: 48 85 c0                    	testq	%rax, %rax
141216663: 0f 84 cd 0a 00 00           	je	0x141217136 <.text+0x1216136>
141216669: 48 89 c3                    	movq	%rax, %rbx
14121666c: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
141216673: 0f 29 85 c0 02 00 00        	movaps	%xmm0, 0x2c0(%rbp)
14121667a: 0f 28 85 90 00 00 00        	movaps	0x90(%rbp), %xmm0
141216681: 48 8d 85 d0 02 00 00        	leaq	0x2d0(%rbp), %rax
141216688: 0f 11 00                    	movups	%xmm0, (%rax)
14121668b: 0f 28 85 c0 02 00 00        	movaps	0x2c0(%rbp), %xmm0
141216692: 0f 28 8d d0 02 00 00        	movaps	0x2d0(%rbp), %xmm1
141216699: 0f 11 4b 10                 	movups	%xmm1, 0x10(%rbx)
14121669d: 0f 11 03                    	movups	%xmm0, (%rbx)
1412166a0: c7 43 20 00 00 00 00        	movl	$0x0, 0x20(%rbx)
1412166a7: 66 c7 43 24 00 00           	movw	$0x0, 0x24(%rbx)
1412166ad: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1412166b4: 48 85 c0                    	testq	%rax, %rax
1412166b7: 74 1a                       	je	0x1412166d3 <.text+0x12156d3>
1412166b9: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1412166c0: 48 c1 e0 03                 	shlq	$0x3, %rax
1412166c4: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1412166c8: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412166ce: e8 4d d5 3f ff              	callq	0x140613c20 <.text+0x612c20>
1412166d3: 48 c7 85 60 02 00 00 01 00 00 00    	movq	$0x1, 0x260(%rbp)
1412166de: 48 89 9d 68 02 00 00        	movq	%rbx, 0x268(%rbp)
1412166e5: 48 c7 85 70 02 00 00 00 00 00 00    	movq	$0x0, 0x270(%rbp)
1412166f0: 48 c7 85 78 02 00 00 01 00 00 00    	movq	$0x1, 0x278(%rbp)
1412166fb: eb 47                       	jmp	0x141216744 <.text+0x1215744>
1412166fd: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
141216704: 48 85 c0                    	testq	%rax, %rax
141216707: 74 1a                       	je	0x141216723 <.text+0x1215723>
141216709: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
141216710: 48 c1 e0 03                 	shlq	$0x3, %rax
141216714: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
141216718: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14121671e: e8 fd d4 3f ff              	callq	0x140613c20 <.text+0x612c20>
141216723: 48 c7 85 60 02 00 00 00 00 00 00    	movq	$0x0, 0x260(%rbp)
14121672e: 48 c7 85 68 02 00 00 04 00 00 00    	movq	$0x4, 0x268(%rbp)
141216739: 48 8d 85 70 02 00 00        	leaq	0x270(%rbp), %rax
141216740: 44 0f 11 10                 	movups	%xmm10, (%rax)
141216744: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
14121674b: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141216752: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
141216759: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
141216760: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
141216767: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
14121676e: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
141216775: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
14121677c: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
141216783: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14121678a: 48 8b 85 30 04 00 00        	movq	0x430(%rbp), %rax
141216791: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
141216798: 48 8b 85 38 04 00 00        	movq	0x438(%rbp), %rax
14121679f: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
1412167a6: 8b 85 80 02 00 00           	movl	0x280(%rbp), %eax
1412167ac: 89 85 c0 02 00 00           	movl	%eax, 0x2c0(%rbp)
1412167b2: f3 0f 10 85 84 02 00 00     	movss	0x284(%rbp), %xmm0
1412167ba: f3 0f 11 85 c4 02 00 00     	movss	%xmm0, 0x2c4(%rbp)
1412167c2: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1412167c6: f3 41 0f 5c cb              	subss	%xmm11, %xmm1
1412167cb: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1412167d2: 4c 89 e1                    	movq	%r12, %rcx
1412167d5: e8 d6 b6 a7 ff              	callq	0x140c91eb0 <.text+0xc90eb0>
1412167da: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
1412167de: f3 45 0f 10 46 30           	movss	0x30(%r14), %xmm8
1412167e4: 8b 85 80 02 00 00           	movl	0x280(%rbp), %eax
1412167ea: 89 85 c0 02 00 00           	movl	%eax, 0x2c0(%rbp)
1412167f0: f3 0f 10 85 84 02 00 00     	movss	0x284(%rbp), %xmm0
1412167f8: f3 0f 11 85 c4 02 00 00     	movss	%xmm0, 0x2c4(%rbp)
141216800: f3 45 0f 58 dc              	addss	%xmm12, %xmm11
141216805: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121680c: 4c 89 e1                    	movq	%r12, %rcx
14121680f: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
141216813: e8 98 b6 a7 ff              	callq	0x140c91eb0 <.text+0xc90eb0>
141216818: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
14121681c: f3 45 0f 5e e8              	divss	%xmm8, %xmm13
141216821: f3 45 0f 5e 5e 30           	divss	0x30(%r14), %xmm11
141216827: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121682e: 4c 89 e1                    	movq	%r12, %rcx
141216831: 48 89 fa                    	movq	%rdi, %rdx
141216834: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
141216838: e8 e3 11 00 00              	callq	0x141217a20 <.text+0x1216a20>
14121683d: f3 45 0f 5e e0              	divss	%xmm8, %xmm12
141216842: f3 44 0f 10 8d c0 02 00 00  	movss	0x2c0(%rbp), %xmm9
14121684b: f3 44 0f 10 85 c4 02 00 00  	movss	0x2c4(%rbp), %xmm8
141216854: f3 44 0f 10 bd c8 02 00 00  	movss	0x2c8(%rbp), %xmm15
14121685d: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216864: 4c 89 e1                    	movq	%r12, %rcx
141216867: 48 89 fa                    	movq	%rdi, %rdx
14121686a: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
14121686e: e8 ad 11 00 00              	callq	0x141217a20 <.text+0x1216a20>
141216873: f3 44 0f 10 b5 c0 02 00 00  	movss	0x2c0(%rbp), %xmm14
14121687c: f3 44 0f 10 95 c4 02 00 00  	movss	0x2c4(%rbp), %xmm10
141216885: f3 44 0f 10 ad c8 02 00 00  	movss	0x2c8(%rbp), %xmm13
14121688e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141216895: 4c 89 e1                    	movq	%r12, %rcx
141216898: 48 89 fa                    	movq	%rdi, %rdx
14121689b: 41 0f 28 d3                 	movaps	%xmm11, %xmm2
14121689f: e8 7c 11 00 00              	callq	0x141217a20 <.text+0x1216a20>
1412168a4: f3 44 0f 10 9d c0 02 00 00  	movss	0x2c0(%rbp), %xmm11
1412168ad: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1412168b1: f3 41 0f 58 c6              	addss	%xmm14, %xmm0
1412168b6: f3 41 0f 58 c3              	addss	%xmm11, %xmm0
1412168bb: 31 c0                       	xorl	%eax, %eax
1412168bd: 0f 57 c9                    	xorps	%xmm1, %xmm1
1412168c0: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1412168c3: 0f 92 c1                    	setb	%cl
1412168c6: 72 18                       	jb	0x1412168e0 <.text+0x12158e0>
1412168c8: f3 44 0f 10 1d 6f 7a 79 01  	movss	0x1797a6f(%rip), %xmm11 # 0x1429ae340
1412168d1: eb 16                       	jmp	0x1412168e9 <.text+0x12158e9>
1412168d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1412168e0: f3 44 0f 58 9d c4 02 00 00  	addss	0x2c4(%rbp), %xmm11
1412168e9: 4c 8b bd 40 02 00 00        	movq	0x240(%rbp), %r15
1412168f0: f3 44 0f 10 a5 c8 02 00 00  	movss	0x2c8(%rbp), %xmm12
1412168f9: 45 0f 14 ce                 	unpcklps	%xmm14, %xmm9           # xmm9 = xmm9[0],xmm14[0],xmm9[1],xmm14[1]
1412168fd: 45 0f 14 c2                 	unpcklps	%xmm10, %xmm8           # xmm8 = xmm8[0],xmm10[0],xmm8[1],xmm10[1]
141216901: 45 0f 58 c1                 	addps	%xmm9, %xmm8
141216905: 88 c8                       	movb	%cl, %al
141216907: 66 0f 6e c0                 	movd	%eax, %xmm0
14121690b: 66 44 0f 70 c8 50           	pshufd	$0x50, %xmm0, %xmm9     # xmm9 = xmm0[0,0,1,1]
141216911: 66 41 0f 72 f1 1f           	pslld	$0x1f, %xmm9
141216917: 66 41 0f 72 e1 1f           	psrad	$0x1f, %xmm9
14121691d: 45 0f 54 c1                 	andps	%xmm9, %xmm8
141216921: 44 0f 55 0d b7 5c 87 01     	andnps	0x1875cb7(%rip), %xmm9  # 0x142a8c5e0
141216929: 45 0f 56 c8                 	orps	%xmm8, %xmm9
14121692d: 44 0f 2e df                 	ucomiss	%xmm7, %xmm11
141216931: 76 40                       	jbe	0x141216973 <.text+0x1215973>
141216933: 44 0f 2e cf                 	ucomiss	%xmm7, %xmm9
141216937: 76 3a                       	jbe	0x141216973 <.text+0x1215973>
141216939: 66 41 0f 70 c1 f5           	pshufd	$0xf5, %xmm9, %xmm0     # xmm0 = xmm9[1,1,3,3]
14121693f: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
141216942: 76 2f                       	jbe	0x141216973 <.text+0x1215973>
141216944: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
14121694b: 48 85 c0                    	testq	%rax, %rax
14121694e: 0f 84 6c f4 ff ff           	je	0x141215dc0 <.text+0x1214dc0>
141216954: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
14121695b: 48 c1 e0 03                 	shlq	$0x3, %rax
14121695f: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
141216963: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141216969: e8 b2 d2 3f ff              	callq	0x140613c20 <.text+0x612c20>
14121696e: e9 4d f4 ff ff              	jmp	0x141215dc0 <.text+0x1214dc0>
141216973: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
141216977: f3 0f 11 85 50 02 00 00     	movss	%xmm0, 0x250(%rbp)
14121697f: 0f 28 45 30                 	movaps	0x30(%rbp), %xmm0
141216983: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
14121698a: 40 f6 c6 01                 	testb	$0x1, %sil
14121698e: 74 17                       	je	0x1412169a7 <.text+0x12159a7>
141216990: 0f 28 45 40                 	movaps	0x40(%rbp), %xmm0
141216994: f3 0f 11 85 50 02 00 00     	movss	%xmm0, 0x250(%rbp)
14121699c: 0f 28 45 50                 	movaps	0x50(%rbp), %xmm0
1412169a0: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
1412169a7: 48 ff 85 d8 01 00 00        	incq	0x1d8(%rbp)
1412169ae: 4c 8b 85 60 02 00 00        	movq	0x260(%rbp), %r8
1412169b5: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1412169bc: 4c 8b 8d 78 02 00 00        	movq	0x278(%rbp), %r9
1412169c3: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1412169c7: f3 0f 58 05 29 4d 7d 01     	addss	0x17d4d29(%rip), %xmm0  # 0x1429eb6f8
1412169cf: f3 0f 11 85 54 02 00 00     	movss	%xmm0, 0x254(%rbp)
1412169d7: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
1412169db: 41 0f 14 c5                 	unpcklps	%xmm13, %xmm0           # xmm0 = xmm0[0],xmm13[0],xmm0[1],xmm13[1]
1412169df: 0f 58 05 0a 5b a0 01        	addps	0x1a05b0a(%rip), %xmm0  # 0x142c1c4f0
1412169e6: 0f 29 85 a0 00 00 00        	movaps	%xmm0, 0xa0(%rbp)
1412169ed: 4d 85 c9                    	testq	%r9, %r9
1412169f0: 0f 84 d6 04 00 00           	je	0x141216ecc <.text+0x1215ecc>
1412169f6: 4c 8b bd 70 02 00 00        	movq	0x270(%rbp), %r15
1412169fd: 0f 28 85 a0 01 00 00        	movaps	0x1a0(%rbp), %xmm0
141216a04: 0f 57 05 e5 ee 70 01        	xorps	0x170eee5(%rip), %xmm0  # 0x1429258f0
141216a0b: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
141216a12: 49 ff c9                    	decq	%r9
141216a15: 48 89 8d c8 01 00 00        	movq	%rcx, 0x1c8(%rbp)
141216a1c: 4c 89 85 d0 01 00 00        	movq	%r8, 0x1d0(%rbp)
141216a23: eb 60                       	jmp	0x141216a85 <.text+0x1215a85>
141216a25: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141216a30: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
141216a37: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
141216a3b: 44 0f 13 04 88              	movlps	%xmm8, (%rax,%rcx,4)
141216a40: f3 44 0f 11 6c 88 08        	movss	%xmm13, 0x8(%rax,%rcx,4)
141216a47: f3 44 0f 11 74 88 0c        	movss	%xmm14, 0xc(%rax,%rcx,4)
141216a4e: c6 44 88 10 02              	movb	$0x2, 0x10(%rax,%rcx,4)
141216a53: 48 ff c6                    	incq	%rsi
141216a56: 48 89 b5 28 02 00 00        	movq	%rsi, 0x228(%rbp)
141216a5d: 48 f7 db                    	negq	%rbx
141216a60: 49 01 df                    	addq	%rbx, %r15
141216a63: 49 ff c7                    	incq	%r15
141216a66: 4c 8b 8d 88 02 00 00        	movq	0x288(%rbp), %r9
141216a6d: 49 83 c1 ff                 	addq	$-0x1, %r9
141216a71: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
141216a78: 4c 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %r8
141216a7f: 0f 83 0f f3 ff ff           	jae	0x141215d94 <.text+0x1214d94>
141216a85: 49 8d 47 01                 	leaq	0x1(%r15), %rax
141216a89: 4c 39 c0                    	cmpq	%r8, %rax
141216a8c: bb 00 00 00 00              	movl	$0x0, %ebx
141216a91: 49 0f 43 d8                 	cmovaeq	%r8, %rbx
141216a95: 4b 8d 14 bf                 	leaq	(%r15,%r15,4), %rdx
141216a99: 48 8b 44 d1 1d              	movq	0x1d(%rcx,%rdx,8), %rax
141216a9e: 48 89 45 7d                 	movq	%rax, 0x7d(%rbp)
141216aa2: 0f 10 04 d1                 	movups	(%rcx,%rdx,8), %xmm0
141216aa6: 0f 10 4c d1 10              	movups	0x10(%rcx,%rdx,8), %xmm1
141216aab: 0f 29 4d 70                 	movaps	%xmm1, 0x70(%rbp)
141216aaf: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
141216ab3: 0f b6 44 d1 25              	movzbl	0x25(%rcx,%rdx,8), %eax
141216ab8: 3c 02                       	cmpb	$0x2, %al
141216aba: 0f 84 d4 f2 ff ff           	je	0x141215d94 <.text+0x1214d94>
141216ac0: 4c 89 8d 88 02 00 00        	movq	%r9, 0x288(%rbp)
141216ac7: 48 8d 0c d1                 	leaq	(%rcx,%rdx,8), %rcx
141216acb: 0f b7 49 26                 	movzwl	0x26(%rcx), %ecx
141216acf: 48 8b 55 7d                 	movq	0x7d(%rbp), %rdx
141216ad3: 48 89 95 ad 02 00 00        	movq	%rdx, 0x2ad(%rbp)
141216ada: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
141216ade: 0f 28 4d 70                 	movaps	0x70(%rbp), %xmm1
141216ae2: 0f 29 8d a0 02 00 00        	movaps	%xmm1, 0x2a0(%rbp)
141216ae9: 0f 29 85 90 02 00 00        	movaps	%xmm0, 0x290(%rbp)
141216af0: 88 85 b5 02 00 00           	movb	%al, 0x2b5(%rbp)
141216af6: 66 89 8d b6 02 00 00        	movw	%cx, 0x2b6(%rbp)
141216afd: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216b04: e8 d7 97 02 00              	callq	0x1412402e0 <.text+0x123f2e0>
141216b09: 0f 2e 05 74 ee 70 01        	ucomiss	0x170ee74(%rip), %xmm0  # 0x142925984
141216b10: 0f 86 47 ff ff ff           	jbe	0x141216a5d <.text+0x1215a5d>
141216b16: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
141216b1d: e8 ce 97 02 00              	callq	0x1412402f0 <.text+0x123f2f0>
141216b22: 0f 2e 05 5b ee 70 01        	ucomiss	0x170ee5b(%rip), %xmm0  # 0x142925984
141216b29: 0f 86 2e ff ff ff           	jbe	0x141216a5d <.text+0x1215a5d>
141216b2f: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216b36: e8 85 95 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
141216b3b: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141216b3f: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216b46: e8 75 95 02 00              	callq	0x1412400c0 <.text+0x123f0c0>
141216b4b: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
141216b4f: e8 2c 50 e2 fe              	callq	0x14003bb80 <.text+0x3ab80>
141216b54: f3 0f 11 85 58 02 00 00     	movss	%xmm0, 0x258(%rbp)
141216b5c: f3 0f 11 8d 5c 02 00 00     	movss	%xmm1, 0x25c(%rbp)
141216b64: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
141216b6b: 48 8d 95 58 02 00 00        	leaq	0x258(%rbp), %rdx
141216b72: 4d 89 f0                    	movq	%r14, %r8
141216b75: e8 16 b5 a7 ff              	callq	0x140c92090 <.text+0xc91090>
141216b7a: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
141216b7e: 0f 28 8d a0 01 00 00        	movaps	0x1a0(%rbp), %xmm1
141216b85: f3 0f 10 95 50 02 00 00     	movss	0x250(%rbp), %xmm2
141216b8d: e8 3e a3 a7 ff              	callq	0x140c90ed0 <.text+0xc8fed0>
141216b92: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216b99: e8 42 97 02 00              	callq	0x1412402e0 <.text+0x123f2e0>
141216b9e: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141216ba2: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216ba9: e8 42 97 02 00              	callq	0x1412402f0 <.text+0x123f2f0>
141216bae: f3 0f 10 4d 00              	movss	(%rbp), %xmm1
141216bb3: f3 0f 59 0d 45 59 a0 01     	mulss	0x1a05945(%rip), %xmm1  # 0x142c1c500
141216bbb: 44 0f 14 e8                 	unpcklps	%xmm0, %xmm13           # xmm13 = xmm13[0],xmm0[0],xmm13[1],xmm0[1]
141216bbf: 45 0f c6 ed 40              	shufps	$0x40, %xmm13, %xmm13   # xmm13 = xmm13[0,0,0,1]
141216bc4: 44 0f 59 6d e0              	mulps	-0x20(%rbp), %xmm13
141216bc9: 44 0f 29 ad c0 02 00 00     	movaps	%xmm13, 0x2c0(%rbp)
141216bd1: 0f c6 05 37 59 a0 01 a0     	shufps	$0xa0, 0x1a05937(%rip), %xmm0 # xmm0 = xmm0[0,0],mem[2,2]
                                                                        # 0x142c1c510
141216bd9: 0f 59 45 f0                 	mulps	-0x10(%rbp), %xmm0
141216bdd: 0f 29 85 d0 02 00 00        	movaps	%xmm0, 0x2d0(%rbp)
141216be4: f3 0f 11 8d e0 02 00 00     	movss	%xmm1, 0x2e0(%rbp)
141216bec: 8b 85 08 02 00 00           	movl	0x208(%rbp), %eax
141216bf2: 89 85 e0 00 00 00           	movl	%eax, 0xe0(%rbp)
141216bf8: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
141216bff: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
141216c06: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141216c0a: 4c 89 e2                    	movq	%r12, %rdx
141216c0d: 4c 8d 85 d8 00 00 00        	leaq	0xd8(%rbp), %r8
141216c14: e8 b7 68 68 ff              	callq	0x14089d4d0 <.text+0x89c4d0>
141216c19: 0f b6 95 16 03 00 00        	movzbl	0x316(%rbp), %edx
141216c20: 80 fa 02                    	cmpb	$0x2, %dl
141216c23: 75 0e                       	jne	0x141216c33 <.text+0x1215c33>
141216c25: f3 44 0f 10 85 04 02 00 00  	movss	0x204(%rbp), %xmm8
141216c2e: e9 b5 00 00 00              	jmp	0x141216ce8 <.text+0x1215ce8>
141216c33: 48 8b 8d 58 04 00 00        	movq	0x458(%rbp), %rcx
141216c3a: 8b 41 10                    	movl	0x10(%rcx), %eax
141216c3d: 89 85 d0 02 00 00           	movl	%eax, 0x2d0(%rbp)
141216c43: 0f 10 01                    	movups	(%rcx), %xmm0
141216c46: 0f 29 85 c0 02 00 00        	movaps	%xmm0, 0x2c0(%rbp)
141216c4d: 88 95 d4 02 00 00           	movb	%dl, 0x2d4(%rbp)
141216c53: 48 8b 95 08 01 00 00        	movq	0x108(%rbp), %rdx
141216c5a: 0f b6 42 02                 	movzbl	0x2(%rdx), %eax
141216c5e: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
141216c65: 88 41 07                    	movb	%al, 0x7(%rcx)
141216c68: 0f b7 02                    	movzwl	(%rdx), %eax
141216c6b: 66 89 41 05                 	movw	%ax, 0x5(%rcx)
141216c6f: 48 8d 8d d4 02 00 00        	leaq	0x2d4(%rbp), %rcx
141216c76: e8 05 ea 6a ff              	callq	0x1408c5680 <.text+0x8c4680>
141216c7b: f3 44 0f 10 85 04 02 00 00  	movss	0x204(%rbp), %xmm8
141216c84: 84 c0                       	testb	%al, %al
141216c86: 74 60                       	je	0x141216ce8 <.text+0x1215ce8>
141216c88: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216c8f: e8 5c 96 02 00              	callq	0x1412402f0 <.text+0x123f2f0>
141216c94: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
141216c98: f3 41 0f 58 c0              	addss	%xmm8, %xmm0
141216c9d: b8 0c 00 00 00              	movl	$0xc, %eax
141216ca2: 41 0f 2e c7                 	ucomiss	%xmm15, %xmm0
141216ca6: 76 11                       	jbe	0x141216cb9 <.text+0x1215cb9>
141216ca8: f3 0f 10 85 54 02 00 00     	movss	0x254(%rbp), %xmm0
141216cb0: 0f 28 8d a0 00 00 00        	movaps	0xa0(%rbp), %xmm1
141216cb7: eb 68                       	jmp	0x141216d21 <.text+0x1215d21>
141216cb9: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216cc0: e8 2b 96 02 00              	callq	0x1412402f0 <.text+0x123f2f0>
141216cc5: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
141216cc9: f3 41 0f 58 c0              	addss	%xmm8, %xmm0
141216cce: 41 0f 2e c4                 	ucomiss	%xmm12, %xmm0
141216cd2: f3 0f 10 85 54 02 00 00     	movss	0x254(%rbp), %xmm0
141216cda: 0f 28 8d a0 00 00 00        	movaps	0xa0(%rbp), %xmm1
141216ce1: b8 0c 00 00 00              	movl	$0xc, %eax
141216ce6: 77 39                       	ja	0x141216d21 <.text+0x1215d21>
141216ce8: f3 44 0f 5c 85 10 03 00 00  	subss	0x310(%rbp), %xmm8
141216cf1: 44 0f 54 05 87 7c 71 01     	andps	0x1717c87(%rip), %xmm8  # 0x14292e980
141216cf9: b8 04 00 00 00              	movl	$0x4, %eax
141216cfe: f3 0f 10 05 8a 6e 7d 01     	movss	0x17d6e8a(%rip), %xmm0  # 0x1429edb90
141216d06: 41 0f 2e c0                 	ucomiss	%xmm8, %xmm0
141216d0a: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
141216d0e: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141216d12: 76 0d                       	jbe	0x141216d21 <.text+0x1215d21>
141216d14: b8 14 00 00 00              	movl	$0x14, %eax
141216d19: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
141216d1d: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
141216d21: 0f 29 4d 10                 	movaps	%xmm1, 0x10(%rbp)
141216d25: f3 0f 11 85 10 02 00 00     	movss	%xmm0, 0x210(%rbp)
141216d2d: 89 85 0c 02 00 00           	movl	%eax, 0x20c(%rbp)
141216d33: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
141216d3a: e8 51 1a 64 ff              	callq	0x140858790 <.text+0x857790>
141216d3f: 89 85 14 02 00 00           	movl	%eax, 0x214(%rbp)
141216d45: f3 44 0f 10 85 58 02 00 00  	movss	0x258(%rbp), %xmm8
141216d4e: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216d55: e8 86 95 02 00              	callq	0x1412402e0 <.text+0x123f2e0>
141216d5a: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141216d5e: f3 44 0f 59 2d 81 33 74 01  	mulss	0x1743381(%rip), %xmm13 # 0x14295a0e8
141216d67: f3 44 0f 10 95 58 02 00 00  	movss	0x258(%rbp), %xmm10
141216d70: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216d77: e8 64 95 02 00              	callq	0x1412402e0 <.text+0x123f2e0>
141216d7c: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
141216d80: f3 44 0f 59 f6              	mulss	%xmm6, %xmm14
141216d85: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
141216d89: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
141216d8d: 0f 10 55 d0                 	movups	-0x30(%rbp), %xmm2
141216d91: 0f 29 95 e0 02 00 00        	movaps	%xmm2, 0x2e0(%rbp)
141216d98: 0f 29 8d d0 02 00 00        	movaps	%xmm1, 0x2d0(%rbp)
141216d9f: 0f 29 85 c0 02 00 00        	movaps	%xmm0, 0x2c0(%rbp)
141216da6: 0f 28 85 90 00 00 00        	movaps	0x90(%rbp), %xmm0
141216dad: 48 8d 85 d0 02 00 00        	leaq	0x2d0(%rbp), %rax
141216db4: 0f 11 40 20                 	movups	%xmm0, 0x20(%rax)
141216db8: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
141216dbf: 48 8b 70 10                 	movq	0x10(%rax), %rsi
141216dc3: 48 3b 30                    	cmpq	(%rax), %rsi
141216dc6: 75 13                       	jne	0x141216ddb <.text+0x1215ddb>
141216dc8: 48 8b 8d 40 04 00 00        	movq	0x440(%rbp), %rcx
141216dcf: 48 8d 15 7a 58 a0 01        	leaq	0x1a0587a(%rip), %rdx   # 0x142c1c650
141216dd6: e8 e5 5c 57 ff              	callq	0x14078cac0 <.text+0x78bac0>
141216ddb: f3 45 0f 58 c5              	addss	%xmm13, %xmm8
141216de0: f3 45 0f 58 d6              	addss	%xmm14, %xmm10
141216de5: 48 8b 95 40 04 00 00        	movq	0x440(%rbp), %rdx
141216dec: 48 8b 42 08                 	movq	0x8(%rdx), %rax
141216df0: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
141216df4: 48 c1 e1 05                 	shlq	$0x5, %rcx
141216df8: 0f 28 85 c0 02 00 00        	movaps	0x2c0(%rbp), %xmm0
141216dff: 0f 28 8d d0 02 00 00        	movaps	0x2d0(%rbp), %xmm1
141216e06: 0f 28 95 e0 02 00 00        	movaps	0x2e0(%rbp), %xmm2
141216e0d: 0f 28 9d f0 02 00 00        	movaps	0x2f0(%rbp), %xmm3
141216e14: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
141216e19: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
141216e1e: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
141216e23: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
141216e27: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
141216e2b: 0f 13 44 08 40              	movlps	%xmm0, 0x40(%rax,%rcx)
141216e30: f3 0f 10 85 10 02 00 00     	movss	0x210(%rbp), %xmm0
141216e38: f3 0f 11 44 08 48           	movss	%xmm0, 0x48(%rax,%rcx)
141216e3e: 4c 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %r8
141216e45: 44 89 44 08 4c              	movl	%r8d, 0x4c(%rax,%rcx)
141216e4a: 44 8b 85 14 02 00 00        	movl	0x214(%rbp), %r8d
141216e51: 44 89 44 08 50              	movl	%r8d, 0x50(%rax,%rcx)
141216e56: f3 44 0f 11 44 08 54        	movss	%xmm8, 0x54(%rax,%rcx)
141216e5d: f3 44 0f 11 54 08 58        	movss	%xmm10, 0x58(%rax,%rcx)
141216e64: 44 8b 85 0c 02 00 00        	movl	0x20c(%rbp), %r8d
141216e6b: 44 89 44 08 5c              	movl	%r8d, 0x5c(%rax,%rcx)
141216e70: 48 ff c6                    	incq	%rsi
141216e73: 48 89 72 10                 	movq	%rsi, 0x10(%rdx)
141216e77: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216e7e: e8 5d 94 02 00              	callq	0x1412402e0 <.text+0x123f2e0>
141216e83: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141216e87: 48 8d 8d 90 02 00 00        	leaq	0x290(%rbp), %rcx
141216e8e: e8 5d 94 02 00              	callq	0x1412402f0 <.text+0x123f2f0>
141216e93: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
141216e97: f2 44 0f 10 85 58 02 00 00  	movsd	0x258(%rbp), %xmm8
141216ea0: 48 8b b5 28 02 00 00        	movq	0x228(%rbp), %rsi
141216ea7: 48 3b b5 18 02 00 00        	cmpq	0x218(%rbp), %rsi
141216eae: 0f 85 7c fb ff ff           	jne	0x141216a30 <.text+0x1215a30>
141216eb4: 48 8d 8d 18 02 00 00        	leaq	0x218(%rbp), %rcx
141216ebb: 48 8d 15 a6 57 a0 01        	leaq	0x1a057a6(%rip), %rdx   # 0x142c1c668
141216ec2: e8 39 c7 34 ff              	callq	0x140563600 <.text+0x562600>
141216ec7: e9 64 fb ff ff              	jmp	0x141216a30 <.text+0x1215a30>
141216ecc: 4d 85 c0                    	testq	%r8, %r8
141216ecf: 0f 85 c6 ee ff ff           	jne	0x141215d9b <.text+0x1214d9b>
141216ed5: e9 d4 ee ff ff              	jmp	0x141215dae <.text+0x1214dae>
141216eda: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141216ee0: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
141216ee7: 48 85 d2                    	testq	%rdx, %rdx
141216eea: 0f 84 d0 ea ff ff           	je	0x1412159c0 <.text+0x12149c0>
141216ef0: 48 c1 e2 02                 	shlq	$0x2, %rdx
141216ef4: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141216efa: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
141216f01: e8 1a cd 3f ff              	callq	0x140613c20 <.text+0x612c20>
141216f06: e9 b5 ea ff ff              	jmp	0x1412159c0 <.text+0x12149c0>
141216f0b: 49 39 d5                    	cmpq	%rdx, %r13
141216f0e: 0f 83 03 03 00 00           	jae	0x141217217 <.text+0x1216217>
141216f14: 48 39 d3                    	cmpq	%rdx, %rbx
141216f17: 0f 83 0b 03 00 00           	jae	0x141217228 <.text+0x1216228>
141216f1d: 48 8b 85 30 04 00 00        	movq	0x430(%rbp), %rax
141216f24: f3 42 0f 10 04 a8           	movss	(%rax,%r13,4), %xmm0
141216f2a: f3 0f 11 85 88 02 00 00     	movss	%xmm0, 0x288(%rbp)
141216f32: f3 46 0f 10 4c a8 04        	movss	0x4(%rax,%r13,4), %xmm9
141216f39: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
141216f40: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
141216f44: e8 c7 1c fe ff              	callq	0x1411f8c10 <.text+0x11f7c10>
141216f49: 89 c1                       	movl	%eax, %ecx
141216f4b: 48 8b 95 38 04 00 00        	movq	0x438(%rbp), %rdx
141216f52: 48 39 d1                    	cmpq	%rdx, %rcx
141216f55: 0f 83 de 02 00 00           	jae	0x141217239 <.text+0x1216239>
141216f5b: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
141216f5f: 48 39 d0                    	cmpq	%rdx, %rax
141216f62: 0f 83 df 02 00 00           	jae	0x141217247 <.text+0x1216247>
141216f68: f3 0f 10 8d 88 02 00 00     	movss	0x288(%rbp), %xmm1
141216f70: f3 44 0f 5c c9              	subss	%xmm1, %xmm9
141216f75: f3 45 0f 59 c1              	mulss	%xmm9, %xmm8
141216f7a: f3 44 0f 58 c1              	addss	%xmm1, %xmm8
141216f7f: 48 8b 85 30 04 00 00        	movq	0x430(%rbp), %rax
141216f86: f3 0f 10 0c 88              	movss	(%rax,%rcx,4), %xmm1
141216f8b: f3 0f 10 54 88 04           	movss	0x4(%rax,%rcx,4), %xmm2
141216f91: f3 0f 5c d1                 	subss	%xmm1, %xmm2
141216f95: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
141216f99: f3 0f 58 d1                 	addss	%xmm1, %xmm2
141216f9d: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
141216fa1: f3 41 0f c2 c0 03           	cmpunordss	%xmm8, %xmm0
141216fa7: 0f 28 c8                    	movaps	%xmm0, %xmm1
141216faa: 0f 54 ca                    	andps	%xmm2, %xmm1
141216fad: f3 41 0f 5f d0              	maxss	%xmm8, %xmm2
141216fb2: 0f 55 c2                    	andnps	%xmm2, %xmm0
141216fb5: 0f 56 c1                    	orps	%xmm1, %xmm0
141216fb8: f3 0f 11 85 10 03 00 00     	movss	%xmm0, 0x310(%rbp)
141216fc0: e9 a1 eb ff ff              	jmp	0x141215b66 <.text+0x1214b66>
141216fc5: 48 8b 95 f0 01 00 00        	movq	0x1f0(%rbp), %rdx
141216fcc: 48 85 d2                    	testq	%rdx, %rdx
141216fcf: 74 16                       	je	0x141216fe7 <.text+0x1215fe7>
141216fd1: 48 c1 e2 03                 	shlq	$0x3, %rdx
141216fd5: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141216fdb: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
141216fe2: e8 39 cc 3f ff              	callq	0x140613c20 <.text+0x612c20>
141216fe7: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
141216fee: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
141216ff5: 0f 10 85 18 02 00 00        	movups	0x218(%rbp), %xmm0
141216ffc: 0f 29 85 c0 02 00 00        	movaps	%xmm0, 0x2c0(%rbp)
141217003: 48 8d 95 c0 02 00 00        	leaq	0x2c0(%rbp), %rdx
14121700a: 48 8b 8d 48 04 00 00        	movq	0x448(%rbp), %rcx
141217011: e8 1a 8e 02 00              	callq	0x14123fe30 <.text+0x123ee30>
141217016: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
14121701d: 48 85 d2                    	testq	%rdx, %rdx
141217020: 74 16                       	je	0x141217038 <.text+0x1216038>
141217022: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
141217029: 48 c1 e2 03                 	shlq	$0x3, %rdx
14121702d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141217033: e8 e8 cb 3f ff              	callq	0x140613c20 <.text+0x612c20>
141217038: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
14121703f: 48 85 d2                    	testq	%rdx, %rdx
141217042: 74 16                       	je	0x14121705a <.text+0x121605a>
141217044: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14121704b: 48 c1 e2 02                 	shlq	$0x2, %rdx
14121704f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141217055: e8 c6 cb 3f ff              	callq	0x140613c20 <.text+0x612c20>
14121705a: 0f 28 b5 20 03 00 00        	movaps	0x320(%rbp), %xmm6
141217061: 0f 28 bd 30 03 00 00        	movaps	0x330(%rbp), %xmm7
141217068: 44 0f 28 85 40 03 00 00     	movaps	0x340(%rbp), %xmm8
141217070: 44 0f 28 8d 50 03 00 00     	movaps	0x350(%rbp), %xmm9
141217078: 44 0f 28 95 60 03 00 00     	movaps	0x360(%rbp), %xmm10
141217080: 44 0f 28 9d 70 03 00 00     	movaps	0x370(%rbp), %xmm11
141217088: 44 0f 28 a5 80 03 00 00     	movaps	0x380(%rbp), %xmm12
141217090: 44 0f 28 ad 90 03 00 00     	movaps	0x390(%rbp), %xmm13
141217098: 44 0f 28 b5 a0 03 00 00     	movaps	0x3a0(%rbp), %xmm14
1412170a0: 44 0f 28 bd b0 03 00 00     	movaps	0x3b0(%rbp), %xmm15
1412170a8: 48 81 c4 48 04 00 00        	addq	$0x448, %rsp            # imm = 0x448
1412170af: 5b                          	popq	%rbx
1412170b0: 5f                          	popq	%rdi
1412170b1: 5e                          	popq	%rsi
1412170b2: 41 5c                       	popq	%r12
1412170b4: 41 5d                       	popq	%r13
1412170b6: 41 5e                       	popq	%r14
1412170b8: 41 5f                       	popq	%r15
1412170ba: 5d                          	popq	%rbp
1412170bb: c3                          	retq
1412170bc: 48 c7 85 08 03 00 00 00 00 00 00    	movq	$0x0, 0x308(%rbp)
1412170c7: 0f 2e 05 12 54 a0 01        	ucomiss	0x1a05412(%rip), %xmm0  # 0x142c1c4e0
1412170ce: 0f 82 5a e6 ff ff           	jb	0x14121572e <.text+0x121472e>
1412170d4: f3 0f 10 0d 08 54 a0 01     	movss	0x1a05408(%rip), %xmm1  # 0x142c1c4e4
1412170dc: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1412170df: 0f 86 49 e6 ff ff           	jbe	0x14121572e <.text+0x121472e>
1412170e5: 48 8d 15 54 56 a0 01        	leaq	0x1a05654(%rip), %rdx   # 0x142c1c740
1412170ec: 48 8d 8d c0 02 00 00        	leaq	0x2c0(%rbp), %rcx
1412170f3: e8 f8 ac 96 ff              	callq	0x140b81df0 <.text+0xb80df0>
1412170f8: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1412170ff: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
141217106: 48 c7 85 d0 02 00 00 01 00 00 00    	movq	$0x1, 0x2d0(%rbp)
141217111: b8 01 00 00 00              	movl	$0x1, %eax
141217116: 48 89 85 08 03 00 00        	movq	%rax, 0x308(%rbp)
14121711d: e9 0c e6 ff ff              	jmp	0x14121572e <.text+0x121472e>
141217122: b9 04 00 00 00              	movl	$0x4, %ecx
141217127: ba 28 00 00 00              	movl	$0x28, %edx
14121712c: e8 cf 1e 6c 01              	callq	0x1428d9000 <.text+0x28d8000>
141217131: e9 6e 01 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
141217136: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14121713d: b9 04 00 00 00              	movl	$0x4, %ecx
141217142: ba 28 00 00 00              	movl	$0x28, %edx
141217147: e8 b4 1e 6c 01              	callq	0x1428d9000 <.text+0x28d8000>
14121714c: e9 53 01 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
141217151: 48 8d 0d 50 54 a0 01        	leaq	0x1a05450(%rip), %rcx   # 0x142c1c5a8
141217158: e8 b3 21 6c 01              	callq	0x1428d9310 <.text+0x28d8310>
14121715d: e9 42 01 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
141217162: 4c 8d 05 27 54 a0 01        	leaq	0x1a05427(%rip), %r8    # 0x142c1c590
141217169: e8 aa 23 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
14121716e: e9 31 01 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
141217173: 4c 8d 05 46 54 a0 01        	leaq	0x1a05446(%rip), %r8    # 0x142c1c5c0
14121717a: 48 89 d9                    	movq	%rbx, %rcx
14121717d: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
141217184: e8 8f 23 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
141217189: e9 16 01 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
14121718e: 4c 8d 05 e3 53 a0 01        	leaq	0x1a053e3(%rip), %r8    # 0x142c1c578
141217195: 4c 89 f9                    	movq	%r15, %rcx
141217198: e8 7b 23 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
14121719d: e9 02 01 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
1412171a2: 4c 8d 05 17 54 a0 01        	leaq	0x1a05417(%rip), %r8    # 0x142c1c5c0
1412171a9: 4c 89 e9                    	movq	%r13, %rcx
1412171ac: e8 67 23 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
1412171b1: e9 ee 00 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
1412171b6: 48 8d 0d a3 53 a0 01        	leaq	0x1a053a3(%rip), %rcx   # 0x142c1c560
1412171bd: e8 5e 28 6c 01              	callq	0x1428d9a20 <.text+0x28d8a20>
1412171c2: e9 dd 00 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
1412171c7: 4c 8d 05 52 54 a0 01        	leaq	0x1a05452(%rip), %r8    # 0x142c1c620
1412171ce: 48 89 d9                    	movq	%rbx, %rcx
1412171d1: e8 42 23 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
1412171d6: e9 c9 00 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
1412171db: 4c 8d 05 3e 54 a0 01        	leaq	0x1a0543e(%rip), %r8    # 0x142c1c620
1412171e2: 4c 89 e9                    	movq	%r13, %rcx
1412171e5: e8 2e 23 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
1412171ea: e9 b5 00 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
1412171ef: 4c 8d 05 e2 53 a0 01        	leaq	0x1a053e2(%rip), %r8    # 0x142c1c5d8
1412171f6: 4c 89 e9                    	movq	%r13, %rcx
1412171f9: e8 1a 23 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
1412171fe: e9 a1 00 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
141217203: 4c 8d 05 ce 53 a0 01        	leaq	0x1a053ce(%rip), %r8    # 0x142c1c5d8
14121720a: 48 89 d9                    	movq	%rbx, %rcx
14121720d: e8 06 23 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
141217212: e9 8d 00 00 00              	jmp	0x1412172a4 <.text+0x12162a4>
141217217: 4c 8d 05 d2 53 a0 01        	leaq	0x1a053d2(%rip), %r8    # 0x142c1c5f0
14121721e: 4c 89 e9                    	movq	%r13, %rcx
141217221: e8 f2 22 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
141217226: eb 7c                       	jmp	0x1412172a4 <.text+0x12162a4>
141217228: 4c 8d 05 c1 53 a0 01        	leaq	0x1a053c1(%rip), %r8    # 0x142c1c5f0
14121722f: 48 89 d9                    	movq	%rbx, %rcx
141217232: e8 e1 22 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
141217237: eb 6b                       	jmp	0x1412172a4 <.text+0x12162a4>
141217239: 4c 8d 05 c8 53 a0 01        	leaq	0x1a053c8(%rip), %r8    # 0x142c1c608
141217240: e8 d3 22 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
141217245: eb 5d                       	jmp	0x1412172a4 <.text+0x12162a4>
141217247: 4c 8d 05 ba 53 a0 01        	leaq	0x1a053ba(%rip), %r8    # 0x142c1c608
14121724e: 48 89 c1                    	movq	%rax, %rcx
141217251: e8 c2 22 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
141217256: eb 4c                       	jmp	0x1412172a4 <.text+0x12162a4>
141217258: 0f 10 85 c8 02 00 00        	movups	0x2c8(%rbp), %xmm0
14121725f: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
141217266: 48 8d 05 0b 21 a0 01        	leaq	0x1a0210b(%rip), %rax   # 0x142c19378
14121726d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141217272: 48 8d 0d 4f 19 a0 01        	leaq	0x1a0194f(%rip), %rcx   # 0x142c18bc8
141217279: 4c 8d 0d 28 19 a0 01        	leaq	0x1a01928(%rip), %r9    # 0x142c18ba8
141217280: 4c 8d 85 10 01 00 00        	leaq	0x110(%rbp), %r8
141217287: ba 2b 00 00 00              	movl	$0x2b, %edx
14121728c: e8 cf 24 6c 01              	callq	0x1428d9760 <.text+0x28d8760>
141217291: 48 8d 4e fe                 	leaq	-0x2(%rsi), %rcx
141217295: 4c 8d 05 8c 54 a0 01        	leaq	0x1a0548c(%rip), %r8    # 0x142c1c728
14121729c: 48 89 f2                    	movq	%rsi, %rdx
14121729f: e8 74 22 6c 01              	callq	0x1428d9518 <.text+0x28d8518>
1412172a4: 0f 0b                       	ud2
1412172a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1412172b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412172b5: 55                          	pushq	%rbp
1412172b6: 41 57                       	pushq	%r15
1412172b8: 41 56                       	pushq	%r14
1412172ba: 41 55                       	pushq	%r13
1412172bc: 41 54                       	pushq	%r12
1412172be: 56                          	pushq	%rsi
1412172bf: 57                          	pushq	%rdi
1412172c0: 53                          	pushq	%rbx
1412172c1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1412172c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412172cf: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1412172d5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1412172db: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1412172e1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1412172e7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1412172ed: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1412172f3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1412172fc: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141217305: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14121730d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141217315: 48 8b 95 c0 02 00 00        	movq	0x2c0(%rbp), %rdx
14121731c: 48 85 d2                    	testq	%rdx, %rdx
14121731f: 74 16                       	je	0x141217337 <.text+0x1216337>
141217321: 48 8b 8d c8 02 00 00        	movq	0x2c8(%rbp), %rcx
141217328: 48 c1 e2 03                 	shlq	$0x3, %rdx
14121732c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141217332: e8 e9 c8 3f ff              	callq	0x140613c20 <.text+0x612c20>
141217337: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14121733f: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141217347: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141217350: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141217359: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14121735f: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141217365: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14121736b: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141217371: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141217377: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14121737d: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141217384: 5b                          	popq	%rbx
141217385: 5f                          	popq	%rdi
141217386: 5e                          	popq	%rsi
141217387: 41 5c                       	popq	%r12
141217389: 41 5d                       	popq	%r13
14121738b: 41 5e                       	popq	%r14
14121738d: 41 5f                       	popq	%r15
14121738f: 5d                          	popq	%rbp
141217390: c3                          	retq
141217391: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1412173a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412173a5: 55                          	pushq	%rbp
1412173a6: 41 57                       	pushq	%r15
1412173a8: 41 56                       	pushq	%r14
1412173aa: 41 55                       	pushq	%r13
1412173ac: 41 54                       	pushq	%r12
1412173ae: 56                          	pushq	%rsi
1412173af: 57                          	pushq	%rdi
1412173b0: 53                          	pushq	%rbx
1412173b1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1412173b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412173bf: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1412173c5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1412173cb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1412173d1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1412173d7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1412173dd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1412173e3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1412173ec: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1412173f5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1412173fd: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141217405: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
14121740c: 48 c1 e0 03                 	shlq	$0x3, %rax
141217410: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
141217414: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14121741a: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
141217421: e8 fa c7 3f ff              	callq	0x140613c20 <.text+0x612c20>
141217426: c6 85 17 03 00 00 00        	movb	$0x0, 0x317(%rbp)
14121742d: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141217435: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14121743d: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141217446: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14121744f: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141217455: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14121745b: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141217461: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141217467: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14121746d: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
141217473: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14121747a: 5b                          	popq	%rbx
14121747b: 5f                          	popq	%rdi
14121747c: 5e                          	popq	%rsi
14121747d: 41 5c                       	popq	%r12
14121747f: 41 5d                       	popq	%r13
141217481: 41 5e                       	popq	%r14
141217483: 41 5f                       	popq	%r15
141217485: 5d                          	popq	%rbp
141217486: c3                          	retq
141217487: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141217490: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141217495: 55                          	pushq	%rbp
141217496: 41 57                       	pushq	%r15
141217498: 41 56                       	pushq	%r14
14121749a: 41 55                       	pushq	%r13
14121749c: 41 54                       	pushq	%r12
14121749e: 56                          	pushq	%rsi
14121749f: 57                          	pushq	%rdi
1412174a0: 53                          	pushq	%rbx
1412174a1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1412174a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412174af: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1412174b5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1412174bb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1412174c1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1412174c7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1412174cd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1412174d3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1412174dc: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1412174e5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1412174ed: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1412174f5: 80 bd 17 03 00 00 00        	cmpb	$0x0, 0x317(%rbp)
1412174fc: 0f 84 80 00 00 00           	je	0x141217582 <.text+0x1216582>
141217502: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
141217509: 48 85 c0                    	testq	%rax, %rax
14121750c: 74 1a                       	je	0x141217528 <.text+0x1216528>
14121750e: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
141217515: 48 c1 e0 03                 	shlq	$0x3, %rax
141217519: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
14121751d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141217523: e8 f8 c6 3f ff              	callq	0x140613c20 <.text+0x612c20>
141217528: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
141217530: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141217538: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141217541: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14121754a: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
141217550: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141217556: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14121755c: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141217562: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141217568: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14121756e: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141217575: 5b                          	popq	%rbx
141217576: 5f                          	popq	%rdi
141217577: 5e                          	popq	%rsi
141217578: 41 5c                       	popq	%r12
14121757a: 41 5d                       	popq	%r13
14121757c: 41 5e                       	popq	%r14
14121757e: 41 5f                       	popq	%r15
141217580: 5d                          	popq	%rbp
141217581: c3                          	retq
141217582: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14121758a: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141217592: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14121759b: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1412175a4: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1412175aa: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1412175b0: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1412175b6: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1412175bc: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1412175c2: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
1412175c8: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1412175cf: 5b                          	popq	%rbx
1412175d0: 5f                          	popq	%rdi
1412175d1: 5e                          	popq	%rsi
1412175d2: 41 5c                       	popq	%r12
1412175d4: 41 5d                       	popq	%r13
1412175d6: 41 5e                       	popq	%r14
1412175d8: 41 5f                       	popq	%r15
1412175da: 5d                          	popq	%rbp
1412175db: c3                          	retq
1412175dc: 0f 1f 40 00                 	nopl	(%rax)
1412175e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412175e5: 55                          	pushq	%rbp
1412175e6: 41 57                       	pushq	%r15
1412175e8: 41 56                       	pushq	%r14
1412175ea: 41 55                       	pushq	%r13
1412175ec: 41 54                       	pushq	%r12
1412175ee: 56                          	pushq	%rsi
1412175ef: 57                          	pushq	%rdi
1412175f0: 53                          	pushq	%rbx
1412175f1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1412175f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412175ff: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141217605: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14121760b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141217611: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141217617: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14121761d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141217623: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14121762c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141217635: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14121763d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141217645: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
14121764c: 48 85 d2                    	testq	%rdx, %rdx
14121764f: 74 16                       	je	0x141217667 <.text+0x1216667>
141217651: 48 c1 e2 02                 	shlq	$0x2, %rdx
141217655: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14121765b: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
141217662: e8 b9 c5 3f ff              	callq	0x140613c20 <.text+0x612c20>
141217667: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14121766f: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141217677: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141217680: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141217689: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14121768f: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141217695: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14121769b: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1412176a1: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1412176a7: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
1412176ad: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1412176b4: 5b                          	popq	%rbx
1412176b5: 5f                          	popq	%rdi
1412176b6: 5e                          	popq	%rsi
1412176b7: 41 5c                       	popq	%r12
1412176b9: 41 5d                       	popq	%r13
1412176bb: 41 5e                       	popq	%r14
1412176bd: 41 5f                       	popq	%r15
1412176bf: 5d                          	popq	%rbp
1412176c0: c3                          	retq
1412176c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1412176d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412176d5: 55                          	pushq	%rbp
1412176d6: 41 57                       	pushq	%r15
1412176d8: 41 56                       	pushq	%r14
1412176da: 41 55                       	pushq	%r13
1412176dc: 41 54                       	pushq	%r12
1412176de: 56                          	pushq	%rsi
1412176df: 57                          	pushq	%rdi
1412176e0: 53                          	pushq	%rbx
1412176e1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1412176e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412176ef: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1412176f5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1412176fb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141217701: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141217707: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14121770d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141217713: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14121771c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141217725: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14121772d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141217735: 48 8b 95 f0 01 00 00        	movq	0x1f0(%rbp), %rdx
14121773c: 48 85 d2                    	testq	%rdx, %rdx
14121773f: 74 16                       	je	0x141217757 <.text+0x1216757>
141217741: 48 c1 e2 03                 	shlq	$0x3, %rdx
141217745: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14121774b: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
141217752: e8 c9 c4 3f ff              	callq	0x140613c20 <.text+0x612c20>
141217757: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14121775f: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141217767: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
141217770: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141217779: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14121777f: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141217785: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14121778b: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
141217791: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141217797: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14121779d: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1412177a4: 5b                          	popq	%rbx
1412177a5: 5f                          	popq	%rdi
1412177a6: 5e                          	popq	%rsi
1412177a7: 41 5c                       	popq	%r12
1412177a9: 41 5d                       	popq	%r13
1412177ab: 41 5e                       	popq	%r14
1412177ad: 41 5f                       	popq	%r15
1412177af: 5d                          	popq	%rbp
1412177b0: c3                          	retq
1412177b1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1412177c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412177c5: 55                          	pushq	%rbp
1412177c6: 41 57                       	pushq	%r15
1412177c8: 41 56                       	pushq	%r14
1412177ca: 41 55                       	pushq	%r13
1412177cc: 41 54                       	pushq	%r12
1412177ce: 56                          	pushq	%rsi
1412177cf: 57                          	pushq	%rdi
1412177d0: 53                          	pushq	%rbx
1412177d1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1412177d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412177df: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1412177e5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1412177eb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1412177f1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1412177f7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1412177fd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141217803: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14121780c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141217815: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14121781d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141217825: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
14121782c: 48 3b 85 e8 01 00 00        	cmpq	0x1e8(%rbp), %rax
141217833: 75 5a                       	jne	0x14121788f <.text+0x121688f>
141217835: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14121783d: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
141217845: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14121784e: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
141217857: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14121785d: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
141217863: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
141217869: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14121786f: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
141217875: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14121787b: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141217882: 5b                          	popq	%rbx
141217883: 5f                          	popq	%rdi
141217884: 5e                          	popq	%rsi
141217885: 41 5c                       	popq	%r12
141217887: 41 5d                       	popq	%r13
141217889: 41 5e                       	popq	%r14
14121788b: 41 5f                       	popq	%r15
14121788d: 5d                          	popq	%rbp
14121788e: c3                          	retq
14121788f: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
141217896: 48 85 c0                    	testq	%rax, %rax
141217899: 74 1a                       	je	0x1412178b5 <.text+0x12168b5>
14121789b: 48 8b 8d 20 02 00 00        	movq	0x220(%rbp), %rcx
1412178a2: 48 c1 e0 02                 	shlq	$0x2, %rax
1412178a6: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1412178aa: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412178b0: e8 6b c3 3f ff              	callq	0x140613c20 <.text+0x612c20>
1412178b5: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1412178bd: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1412178c5: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1412178ce: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1412178d7: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1412178dd: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1412178e3: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1412178e9: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1412178ef: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1412178f5: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
1412178fb: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141217902: 5b                          	popq	%rbx
141217903: 5f                          	popq	%rdi
141217904: 5e                          	popq	%rsi
141217905: 41 5c                       	popq	%r12
141217907: 41 5d                       	popq	%r13
141217909: 41 5e                       	popq	%r14
14121790b: 41 5f                       	popq	%r15
14121790d: 5d                          	popq	%rbp
14121790e: c3                          	retq
14121790f: 90                          	nop
141217910: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141217915: 55                          	pushq	%rbp
141217916: 41 57                       	pushq	%r15
141217918: 41 56                       	pushq	%r14
14121791a: 41 55                       	pushq	%r13
14121791c: 41 54                       	pushq	%r12
14121791e: 56                          	pushq	%rsi
14121791f: 57                          	pushq	%rdi
141217920: 53                          	pushq	%rbx
141217921: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
141217928: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14121792f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
141217935: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14121793b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
141217941: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
141217947: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14121794d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
141217953: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14121795c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
141217965: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14121796d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
141217975: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
14121797c: 48 85 d2                    	testq	%rdx, %rdx
14121797f: 74 16                       	je	0x141217997 <.text+0x1216997>
141217981: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
141217988: 48 c1 e2 03                 	shlq	$0x3, %rdx
14121798c: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141217992: e8 89 c2 3f ff              	callq	0x140613c20 <.text+0x612c20>
141217997: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
14121799e: 48 85 d2                    	testq	%rdx, %rdx
1412179a1: 74 16                       	je	0x1412179b9 <.text+0x12169b9>
1412179a3: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1412179aa: 48 c1 e2 02                 	shlq	$0x2, %rdx
1412179ae: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412179b4: e8 67 c2 3f ff              	callq	0x140613c20 <.text+0x612c20>
1412179b9: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1412179c1: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1412179c9: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1412179d2: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1412179db: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1412179e1: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1412179e7: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1412179ed: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1412179f3: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1412179f9: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
1412179ff: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
141217a06: 5b                          	popq	%rbx
141217a07: 5f                          	popq	%rdi
141217a08: 5e                          	popq	%rsi
141217a09: 41 5c                       	popq	%r12
141217a0b: 41 5d                       	popq	%r13
141217a0d: 41 5e                       	popq	%r14
141217a0f: 41 5f                       	popq	%r15
141217a11: 5d                          	popq	%rbp
141217a12: c3                          	retq
