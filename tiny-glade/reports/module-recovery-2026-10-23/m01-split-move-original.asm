
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141279410: 55                          	pushq	%rbp
141279411: 41 57                       	pushq	%r15
141279413: 41 56                       	pushq	%r14
141279415: 41 55                       	pushq	%r13
141279417: 41 54                       	pushq	%r12
141279419: 56                          	pushq	%rsi
14127941a: 57                          	pushq	%rdi
14127941b: 53                          	pushq	%rbx
14127941c: 48 81 ec 68 03 00 00        	subq	$0x368, %rsp            # imm = 0x368
141279423: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14127942b: 44 0f 29 b5 d0 02 00 00     	movaps	%xmm14, 0x2d0(%rbp)
141279433: 44 0f 29 ad c0 02 00 00     	movaps	%xmm13, 0x2c0(%rbp)
14127943b: 44 0f 29 a5 b0 02 00 00     	movaps	%xmm12, 0x2b0(%rbp)
141279443: 44 0f 29 9d a0 02 00 00     	movaps	%xmm11, 0x2a0(%rbp)
14127944b: 44 0f 29 95 90 02 00 00     	movaps	%xmm10, 0x290(%rbp)
141279453: 44 0f 29 8d 80 02 00 00     	movaps	%xmm9, 0x280(%rbp)
14127945b: 44 0f 29 85 70 02 00 00     	movaps	%xmm8, 0x270(%rbp)
141279463: 0f 29 bd 60 02 00 00        	movaps	%xmm7, 0x260(%rbp)
14127946a: 0f 29 b5 50 02 00 00        	movaps	%xmm6, 0x250(%rbp)
141279471: 48 c7 85 48 02 00 00 fe ff ff ff    	movq	$-0x2, 0x248(%rbp)
14127947c: 4c 89 cf                    	movq	%r9, %rdi
14127947f: 4c 89 c0                    	movq	%r8, %rax
141279482: 48 89 d3                    	movq	%rdx, %rbx
141279485: 48 89 ce                    	movq	%rcx, %rsi
141279488: 44 8b 8d 50 03 00 00        	movl	0x350(%rbp), %r9d
14127948f: 48 8b 17                    	movq	(%rdi), %rdx
141279492: 4c 8b 47 08                 	movq	0x8(%rdi), %r8
141279496: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14127949d: 48 89 c1                    	movq	%rax, %rcx
1412794a0: 48 89 95 80 01 00 00        	movq	%rdx, 0x180(%rbp)
1412794a7: 4c 89 85 88 01 00 00        	movq	%r8, 0x188(%rbp)
1412794ae: e8 4d 33 7a ff              	callq	0x140a1c800 <.text+0xa1b800>
1412794b3: 48 85 c0                    	testq	%rax, %rax
1412794b6: 0f 84 46 0c 00 00           	je	0x14127a102 <.text+0x1279102>
1412794bc: 49 89 c4                    	movq	%rax, %r12
1412794bf: 0f b6 40 46                 	movzbl	0x46(%rax), %eax
1412794c3: 88 85 46 02 00 00           	movb	%al, 0x246(%rbp)
1412794c9: 49 8b 44 24 20              	movq	0x20(%r12), %rax
1412794ce: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1412794d2: 41 0f 10 04 24              	movups	(%r12), %xmm0
1412794d7: 41 0f 10 4c 24 10           	movups	0x10(%r12), %xmm1
1412794dd: 0f 29 4d 30                 	movaps	%xmm1, 0x30(%rbp)
1412794e1: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
1412794e5: 41 0f b6 44 24 44           	movzbl	0x44(%r12), %eax
1412794eb: 88 85 40 02 00 00           	movb	%al, 0x240(%rbp)
1412794f1: 41 8b 44 24 28              	movl	0x28(%r12), %eax
1412794f6: 89 85 24 02 00 00           	movl	%eax, 0x224(%rbp)
1412794fc: 41 8b 44 24 40              	movl	0x40(%r12), %eax
141279501: 89 85 c0 00 00 00           	movl	%eax, 0xc0(%rbp)
141279507: 41 0f 10 44 24 30           	movups	0x30(%r12), %xmm0
14127950d: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
141279514: 4c 89 e1                    	movq	%r12, %rcx
141279517: e8 84 9e 5b ff              	callq	0x1408333a0 <.text+0x8323a0>
14127951c: 48 85 c0                    	testq	%rax, %rax
14127951f: 48 89 9d 28 02 00 00        	movq	%rbx, 0x228(%rbp)
141279526: 48 89 75 78                 	movq	%rsi, 0x78(%rbp)
14127952a: 48 89 bd 30 02 00 00        	movq	%rdi, 0x230(%rbp)
141279531: 74 33                       	je	0x141279566 <.text+0x1278566>
141279533: 48 8b 48 20                 	movq	0x20(%rax), %rcx
141279537: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
14127953b: 0f 10 00                    	movups	(%rax), %xmm0
14127953e: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
141279542: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
141279546: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
14127954a: 31 c0                       	xorl	%eax, %eax
14127954c: 80 7d 12 02                 	cmpb	$0x2, 0x12(%rbp)
141279550: 74 1a                       	je	0x14127956c <.text+0x127856c>
141279552: f2 0f 10 45 f0              	movsd	-0x10(%rbp), %xmm0
141279557: f2 0f 11 85 c8 00 00 00     	movsd	%xmm0, 0xc8(%rbp)
14127955f: b8 01 00 00 00              	movl	$0x1, %eax
141279564: eb 06                       	jmp	0x14127956c <.text+0x127856c>
141279566: c6 45 12 02                 	movb	$0x2, 0x12(%rbp)
14127956a: 31 c0                       	xorl	%eax, %eax
14127956c: 89 85 c4 00 00 00           	movl	%eax, 0xc4(%rbp)
141279572: 48 c7 85 f8 00 00 00 00 00 00 00    	movq	$0x0, 0xf8(%rbp)
14127957d: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
141279584: 4c 89 e1                    	movq	%r12, %rcx
141279587: e8 14 9e 5b ff              	callq	0x1408333a0 <.text+0x8323a0>
14127958c: 48 85 c0                    	testq	%rax, %rax
14127958f: 74 63                       	je	0x1412795f4 <.text+0x12785f4>
141279591: 0f b6 48 20                 	movzbl	0x20(%rax), %ecx
141279595: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
14127959c: e8 0f c1 8a ff              	callq	0x140b256b0 <.text+0xb246b0>
1412795a1: 84 c0                       	testb	%al, %al
1412795a3: 74 4f                       	je	0x1412795f4 <.text+0x12785f4>
1412795a5: 0f b6 85 40 02 00 00        	movzbl	0x240(%rbp), %eax
1412795ac: 84 c0                       	testb	%al, %al
1412795ae: 0f 84 d0 0c 00 00           	je	0x14127a284 <.text+0x1279284>
1412795b4: fe c8                       	decb	%al
1412795b6: c7 85 d8 00 00 00 00 00 00 00       	movl	$0x0, 0xd8(%rbp)
1412795c0: 88 85 e4 00 00 00           	movb	%al, 0xe4(%rbp)
1412795c6: 48 c7 85 f8 00 00 00 01 00 00 00    	movq	$0x1, 0xf8(%rbp)
1412795d1: c7 85 40 02 00 00 00 00 00 00       	movl	$0x0, 0x240(%rbp)
1412795db: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1412795e1: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
1412795e8: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
1412795ef: e9 44 04 00 00              	jmp	0x141279a38 <.text+0x1278a38>
1412795f4: 83 bd c4 00 00 00 00        	cmpl	$0x0, 0xc4(%rbp)
1412795fb: 0f 84 6e 03 00 00           	je	0x14127996f <.text+0x127896f>
141279601: 48 8b b5 68 03 00 00        	movq	0x368(%rbp), %rsi
141279608: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
14127960f: 48 89 f1                    	movq	%rsi, %rcx
141279612: 8b 95 50 03 00 00           	movl	0x350(%rbp), %edx
141279618: e8 53 d3 8a ff              	callq	0x140b26970 <.text+0xb25970>
14127961d: 88 85 3c 02 00 00           	movb	%al, 0x23c(%rbp)
141279623: 41 8a 7c 24 44              	movb	0x44(%r12), %dil
141279628: 48 8b 16                    	movq	(%rsi), %rdx
14127962b: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
141279632: 0f b6 8d 3c 02 00 00        	movzbl	0x23c(%rbp), %ecx
141279639: 48 89 95 d0 00 00 00        	movq	%rdx, 0xd0(%rbp)
141279640: e8 0b 9c 5b ff              	callq	0x140833250 <.text+0x832250>
141279645: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
141279649: 48 89 8d 90 01 00 00        	movq	%rcx, 0x190(%rbp)
141279650: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
141279657: 89 f9                       	movl	%edi, %ecx
141279659: 89 c2                       	movl	%eax, %edx
14127965b: e8 30 c6 8a ff              	callq	0x140b25c90 <.text+0xb24c90>
141279660: 88 85 00 01 00 00           	movb	%al, 0x100(%rbp)
141279666: 88 95 01 01 00 00           	movb	%dl, 0x101(%rbp)
14127966c: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
141279673: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
14127967a: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
141279681: e8 0a 39 fe ff              	callq	0x14125cf90 <.text+0x125bf90>
141279686: 48 85 c0                    	testq	%rax, %rax
141279689: 0f 84 7f 0a 00 00           	je	0x14127a10e <.text+0x127910e>
14127968f: 0f b6 95 40 02 00 00        	movzbl	0x240(%rbp), %edx
141279696: 4c 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %r9
14127969d: f3 0f 10 78 3c              	movss	0x3c(%rax), %xmm7
1412796a2: f3 0f 5c 78 38              	subss	0x38(%rax), %xmm7
1412796a7: f3 0f 10 70 30              	movss	0x30(%rax), %xmm6
1412796ac: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1412796b3: 48 89 c8                    	movq	%rcx, %rax
1412796b6: 48 f7 d0                    	notq	%rax
1412796b9: 48 89 95 80 00 00 00        	movq	%rdx, 0x80(%rbp)
1412796c0: 48 01 d0                    	addq	%rdx, %rax
1412796c3: 48 89 8d 38 01 00 00        	movq	%rcx, 0x138(%rbp)
1412796ca: c7 85 40 01 00 00 00 00 80 bf       	movl	$0xbf800000, 0x140(%rbp) # imm = 0xBF800000
1412796d4: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
1412796db: c7 85 50 01 00 00 00 00 80 3f       	movl	$0x3f800000, 0x150(%rbp) # imm = 0x3F800000
1412796e5: 31 ff                       	xorl	%edi, %edi
1412796e7: 31 c0                       	xorl	%eax, %eax
1412796e9: 80 bd 46 02 00 00 3a        	cmpb	$0x3a, 0x246(%rbp)
1412796f0: 40 0f 95 c7                 	setne	%dil
1412796f4: 4c 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %r10
1412796fb: 0f 94 c0                    	sete	%al
1412796fe: 66 0f 6e c0                 	movd	%eax, %xmm0
141279702: 66 44 0f 70 c0 50           	pshufd	$0x50, %xmm0, %xmm8     # xmm8 = xmm0[0,0,1,1]
141279708: 66 41 0f 72 f0 1f           	pslld	$0x1f, %xmm8
14127970e: 66 41 0f 72 e0 1f           	psrad	$0x1f, %xmm8
141279714: 66 0f 6e 05 e8 c1 6a 01     	movd	0x16ac1e8(%rip), %xmm0  # 0x142925904
14127971c: 66 41 0f 6f c8              	movdqa	%xmm8, %xmm1
141279721: 66 0f df c8                 	pandn	%xmm0, %xmm1
141279725: 66 44 0f db 05 82 ae 6b 01  	pand	0x16bae82(%rip), %xmm8  # 0x1429345b0
14127972e: 66 44 0f eb c1              	por	%xmm1, %xmm8
141279733: 66 0f 6e c7                 	movd	%edi, %xmm0
141279737: 66 0f 70 c0 44              	pshufd	$0x44, %xmm0, %xmm0     # xmm0 = xmm0[0,1,0,1]
14127973c: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
141279740: 44 0f 14 cf                 	unpcklps	%xmm7, %xmm9            # xmm9 = xmm9[0],xmm7[0],xmm9[1],xmm7[1]
141279744: 45 31 ed                    	xorl	%r13d, %r13d
141279747: 44 0f 28 15 91 c1 6a 01     	movaps	0x16ac191(%rip), %xmm10 # 0x1429258e0
14127974f: 66 44 0f 70 d8 e8           	pshufd	$0xe8, %xmm0, %xmm11    # xmm11 = xmm0[0,2,2,3]
141279755: 66 41 0f 72 f3 1f           	pslld	$0x1f, %xmm11
14127975b: 66 41 0f 72 e3 1f           	psrad	$0x1f, %xmm11
141279761: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
141279768: 4c 89 e8                    	movq	%r13, %rax
14127976b: 48 c1 e0 04                 	shlq	$0x4, %rax
14127976f: 48 01 e8                    	addq	%rbp, %rax
141279772: 48 05 38 01 00 00           	addq	$0x138, %rax            # imm = 0x138
141279778: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141279780: 49 83 fd 02                 	cmpq	$0x2, %r13
141279784: 0f 84 3e 01 00 00           	je	0x1412798c8 <.text+0x12788c8>
14127978a: 49 ff c5                    	incq	%r13
14127978d: 48 8b 30                    	movq	(%rax), %rsi
141279790: 48 83 c0 10                 	addq	$0x10, %rax
141279794: 48 85 f6                    	testq	%rsi, %rsi
141279797: 74 e7                       	je	0x141279780 <.text+0x1278780>
141279799: 49 89 ce                    	movq	%rcx, %r14
14127979c: 4d 89 d7                    	movq	%r10, %r15
14127979f: 4c 89 cb                    	movq	%r9, %rbx
1412797a2: 4c 89 ad 58 01 00 00        	movq	%r13, 0x158(%rbp)
1412797a9: f3 44 0f 10 68 f8           	movss	-0x8(%rax), %xmm13
1412797af: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1412797b6: 0f b6 8d 3c 02 00 00        	movzbl	0x23c(%rbp), %ecx
1412797bd: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
1412797c4: e8 87 9a 5b ff              	callq	0x140833250 <.text+0x832250>
1412797c9: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1412797d0: 89 f1                       	movl	%esi, %ecx
1412797d2: 89 c2                       	movl	%eax, %edx
1412797d4: e8 b7 c4 8a ff              	callq	0x140b25c90 <.text+0xb24c90>
1412797d9: 88 85 88 00 00 00           	movb	%al, 0x88(%rbp)
1412797df: 88 95 89 00 00 00           	movb	%dl, 0x89(%rbp)
1412797e5: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1412797ec: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
1412797f3: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
1412797fa: e8 91 37 fe ff              	callq	0x14125cf90 <.text+0x125bf90>
1412797ff: 48 85 c0                    	testq	%rax, %rax
141279802: 0f 84 07 0a 00 00           	je	0x14127a20f <.text+0x127920f>
141279808: 0f 10 40 30                 	movups	0x30(%rax), %xmm0
14127980c: f3 0f 10 48 3c              	movss	0x3c(%rax), %xmm1
141279811: f3 0f 5c 48 38              	subss	0x38(%rax), %xmm1
141279816: 0f 16 c8                    	movlhps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0]
141279819: 0f c6 c8 e2                 	shufps	$0xe2, %xmm0, %xmm1     # xmm1 = xmm1[2,0],xmm0[2,3]
14127981d: 45 0f 28 f1                 	movaps	%xmm9, %xmm14
141279821: 4c 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %r8
141279828: 49 83 f8 03                 	cmpq	$0x3, %r8
14127982c: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
141279833: 4c 89 f1                    	movq	%r14, %rcx
141279836: 49 0f 42 c6                 	cmovbq	%r14, %rax
14127983a: 44 0f 5c f1                 	subps	%xmm1, %xmm14
14127983e: 49 89 d9                    	movq	%rbx, %r9
141279841: 49 89 db                    	movq	%rbx, %r11
141279844: 4d 89 fa                    	movq	%r15, %r10
141279847: 4d 0f 43 df                 	cmovaeq	%r15, %r11
14127984b: 45 0f 59 f2                 	mulps	%xmm10, %xmm14
14127984f: ba 02 00 00 00              	movl	$0x2, %edx
141279854: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
141279858: 48 89 da                    	movq	%rbx, %rdx
14127985b: 49 0f 43 d7                 	cmovaeq	%r15, %rdx
14127985f: 48 8b 12                    	movq	(%rdx), %rdx
141279862: 4c 39 c2                    	cmpq	%r8, %rdx
141279865: 74 39                       	je	0x1412798a0 <.text+0x12788a0>
141279867: 66 41 0f 6f c3              	movdqa	%xmm11, %xmm0
14127986c: 66 41 0f df c4              	pandn	%xmm12, %xmm0
141279871: 45 0f 59 f0                 	mulps	%xmm8, %xmm14
141279875: f3 45 0f 12 e5              	movsldup	%xmm13, %xmm12          # xmm12 = xmm13[0,0,2,2]
14127987a: 45 0f 59 e6                 	mulps	%xmm14, %xmm12
14127987e: 45 0f 54 e3                 	andps	%xmm11, %xmm12
141279882: 44 0f 56 e0                 	orps	%xmm0, %xmm12
141279886: 48 c1 e2 04                 	shlq	$0x4, %rdx
14127988a: 89 3c 10                    	movl	%edi, (%rax,%rdx)
14127988d: 44 0f 13 64 10 04           	movlps	%xmm12, 0x4(%rax,%rdx)
141279893: 40 88 74 10 0c              	movb	%sil, 0xc(%rax,%rdx)
141279898: 49 ff 03                    	incq	(%r11)
14127989b: e9 c8 fe ff ff              	jmp	0x141279768 <.text+0x1278768>
1412798a0: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1412798a7: e8 44 ea 67 01              	callq	0x1428f82f0 <.text+0x28f72f0>
1412798ac: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
1412798b3: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
1412798ba: 4d 89 fa                    	movq	%r15, %r10
1412798bd: 4d 89 fb                    	movq	%r15, %r11
1412798c0: 49 89 d9                    	movq	%rbx, %r9
1412798c3: 4c 89 f1                    	movq	%r14, %rcx
1412798c6: eb 9f                       	jmp	0x141279867 <.text+0x1278867>
1412798c8: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1412798cf: e8 9c 22 dc fe              	callq	0x14003bb70 <.text+0x3ab70>
1412798d4: 40 88 c6                    	movb	%al, %sil
1412798d7: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1412798de: 0f b6 8d 3c 02 00 00        	movzbl	0x23c(%rbp), %ecx
1412798e5: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
1412798ec: e8 5f 99 5b ff              	callq	0x140833250 <.text+0x832250>
1412798f1: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1412798f8: 89 f1                       	movl	%esi, %ecx
1412798fa: 89 c2                       	movl	%eax, %edx
1412798fc: e8 8f c3 8a ff              	callq	0x140b25c90 <.text+0xb24c90>
141279901: 88 85 00 01 00 00           	movb	%al, 0x100(%rbp)
141279907: 88 95 01 01 00 00           	movb	%dl, 0x101(%rbp)
14127990d: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
141279914: 48 8d b5 00 01 00 00        	leaq	0x100(%rbp), %rsi
14127991b: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
141279922: 48 89 f2                    	movq	%rsi, %rdx
141279925: e8 66 36 fe ff              	callq	0x14125cf90 <.text+0x125bf90>
14127992a: 48 85 c0                    	testq	%rax, %rax
14127992d: 0f 84 53 08 00 00           	je	0x14127a186 <.text+0x1279186>
141279933: 80 bd 40 02 00 00 00        	cmpb	$0x0, 0x240(%rbp)
14127993a: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
141279941: 0f 84 b0 08 00 00           	je	0x14127a1f7 <.text+0x12791f7>
141279947: f3 0f 10 50 30              	movss	0x30(%rax), %xmm2
14127994c: f3 0f 10 48 3c              	movss	0x3c(%rax), %xmm1
141279951: f3 0f 5c 48 38              	subss	0x38(%rax), %xmm1
141279956: 4c 8b 85 80 00 00 00        	movq	0x80(%rbp), %r8
14127995d: 49 ff c8                    	decq	%r8
141279960: 48 85 c9                    	testq	%rcx, %rcx
141279963: 78 2a                       	js	0x14127998f <.text+0x127898f>
141279965: 0f 57 c0                    	xorps	%xmm0, %xmm0
141279968: f3 48 0f 2a c1              	cvtsi2ss	%rcx, %xmm0
14127996d: eb 38                       	jmp	0x1412799a7 <.text+0x12789a7>
14127996f: 45 31 ff                    	xorl	%r15d, %r15d
141279972: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
141279979: c7 85 40 02 00 00 00 00 00 00       	movl	$0x0, 0x240(%rbp)
141279983: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
14127998a: e9 a9 00 00 00              	jmp	0x141279a38 <.text+0x1278a38>
14127998f: 48 89 c8                    	movq	%rcx, %rax
141279992: 48 d1 e8                    	shrq	%rax
141279995: 83 e1 01                    	andl	$0x1, %ecx
141279998: 48 09 c1                    	orq	%rax, %rcx
14127999b: 0f 57 c0                    	xorps	%xmm0, %xmm0
14127999e: f3 48 0f 2a c1              	cvtsi2ss	%rcx, %xmm0
1412799a3: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1412799a7: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
1412799ae: f3 0f 5c f9                 	subss	%xmm1, %xmm7
1412799b2: f3 0f 10 1d b6 be 6a 01     	movss	0x16abeb6(%rip), %xmm3  # 0x142925870
1412799ba: f3 0f 59 fb                 	mulss	%xmm3, %xmm7
1412799be: f3 41 0f 16 c8              	movshdup	%xmm8, %xmm1            # xmm1 = xmm8[1,1,3,3]
1412799c3: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
1412799c7: f3 0f 5c f2                 	subss	%xmm2, %xmm6
1412799cb: f3 0f 59 f3                 	mulss	%xmm3, %xmm6
1412799cf: f3 41 0f 59 f0              	mulss	%xmm8, %xmm6
1412799d4: 4d 85 c0                    	testq	%r8, %r8
1412799d7: 78 0a                       	js	0x1412799e3 <.text+0x12789e3>
1412799d9: 0f 57 d2                    	xorps	%xmm2, %xmm2
1412799dc: f3 41 0f 2a d0              	cvtsi2ss	%r8d, %xmm2
1412799e1: eb 19                       	jmp	0x1412799fc <.text+0x12789fc>
1412799e3: 4c 89 c0                    	movq	%r8, %rax
1412799e6: 48 d1 e8                    	shrq	%rax
1412799e9: 41 83 e0 01                 	andl	$0x1, %r8d
1412799ed: 49 09 c0                    	orq	%rax, %r8
1412799f0: 0f 57 d2                    	xorps	%xmm2, %xmm2
1412799f3: f3 49 0f 2a d0              	cvtsi2ss	%r8, %xmm2
1412799f8: f3 0f 58 d2                 	addss	%xmm2, %xmm2
1412799fc: f3 0f 5e c2                 	divss	%xmm2, %xmm0
141279a00: f3 0f 58 c0                 	addss	%xmm0, %xmm0
141279a04: f3 0f 58 05 d8 06 6e 01     	addss	0x16e06d8(%rip), %xmm0  # 0x14295a0e4
141279a0c: f3 0f 59 f0                 	mulss	%xmm0, %xmm6
141279a10: f3 0f 11 b5 3c 02 00 00     	movss	%xmm6, 0x23c(%rbp)
141279a18: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
141279a1c: 66 0f 7e c0                 	movd	%xmm0, %eax
141279a20: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
141279a27: c7 85 40 02 00 00 01 00 00 00       	movl	$0x1, 0x240(%rbp)
141279a31: 4c 8b bd f8 00 00 00        	movq	0xf8(%rbp), %r15
141279a38: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
141279a3f: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
141279a46: 48 89 8d 88 00 00 00        	movq	%rcx, 0x88(%rbp)
141279a4d: 48 89 95 90 00 00 00        	movq	%rdx, 0x90(%rbp)
141279a54: 48 8d 8d c4 00 00 00        	leaq	0xc4(%rbp), %rcx
141279a5b: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
141279a62: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
141279a66: 48 89 8d a0 00 00 00        	movq	%rcx, 0xa0(%rbp)
141279a6d: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
141279a74: 4c 8b ad d8 00 00 00        	movq	0xd8(%rbp), %r13
141279a7b: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
141279a82: 0f 10 85 e8 00 00 00        	movups	0xe8(%rbp), %xmm0
141279a89: 0f 11 85 10 01 00 00        	movups	%xmm0, 0x110(%rbp)
141279a90: 45 31 f6                    	xorl	%r14d, %r14d
141279a93: 49 83 ff 03                 	cmpq	$0x3, %r15
141279a97: 4c 89 fb                    	movq	%r15, %rbx
141279a9a: 48 0f 43 d8                 	cmovaeq	%rax, %rbx
141279a9e: 49 0f 43 c6                 	cmovaeq	%r14, %rax
141279aa2: b9 00 00 00 00              	movl	$0x0, %ecx
141279aa7: 49 0f 43 cf                 	cmovaeq	%r15, %rcx
141279aab: 4c 89 ad 00 01 00 00        	movq	%r13, 0x100(%rbp)
141279ab2: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
141279ab9: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
141279ac0: 48 c7 85 28 01 00 00 00 00 00 00    	movq	$0x0, 0x128(%rbp)
141279acb: 48 89 9d 30 01 00 00        	movq	%rbx, 0x130(%rbp)
141279ad2: 48 c7 85 98 01 00 00 00 00 00 00    	movq	$0x0, 0x198(%rbp)
141279add: 48 c7 85 a0 01 00 00 04 00 00 00    	movq	$0x4, 0x1a0(%rbp)
141279ae8: 48 c7 85 a8 01 00 00 00 00 00 00    	movq	$0x0, 0x1a8(%rbp)
141279af3: 48 85 db                    	testq	%rbx, %rbx
141279af6: 0f 84 a7 03 00 00           	je	0x141279ea3 <.text+0x1278ea3>
141279afc: 49 83 ff 03                 	cmpq	$0x3, %r15
141279b00: 48 c7 85 28 01 00 00 01 00 00 00    	movq	$0x1, 0x128(%rbp)
141279b0b: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
141279b12: 49 0f 43 c5                 	cmovaeq	%r13, %rax
141279b16: 8b 30                       	movl	(%rax), %esi
141279b18: 83 fe 02                    	cmpl	$0x2, %esi
141279b1b: 0f 84 61 01 00 00           	je	0x141279c82 <.text+0x1278c82>
141279b21: 4c 8b 70 04                 	movq	0x4(%rax), %r14
141279b25: 0f b6 40 0c                 	movzbl	0xc(%rax), %eax
141279b29: 88 85 30 02 00 00           	movb	%al, 0x230(%rbp)
141279b2f: 48 8d 15 6a a0 9a 01        	leaq	0x19aa06a(%rip), %rdx   # 0x142c23ba0
141279b36: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
141279b3d: e8 2e 5d 8e ff              	callq	0x140b5f870 <.text+0xb5e870>
141279b42: 8b bd 50 03 00 00           	movl	0x350(%rbp), %edi
141279b48: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
141279b4f: 89 38                       	movl	%edi, (%rax)
141279b51: 48 c7 85 a8 01 00 00 01 00 00 00    	movq	$0x1, 0x1a8(%rbp)
141279b5c: 45 8a 64 24 46              	movb	0x46(%r12), %r12b
141279b61: 89 bd b0 01 00 00           	movl	%edi, 0x1b0(%rbp)
141279b67: 4c 8d 8d b4 01 00 00        	leaq	0x1b4(%rbp), %r9
141279b6e: 89 b5 b4 01 00 00           	movl	%esi, 0x1b4(%rbp)
141279b74: 4c 89 b5 b8 01 00 00        	movq	%r14, 0x1b8(%rbp)
141279b7b: 48 8d 8d 38 01 00 00        	leaq	0x138(%rbp), %rcx
141279b82: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
141279b89: 41 89 f8                    	movl	%edi, %r8d
141279b8c: e8 ef 08 00 00              	callq	0x14127a480 <.text+0x1279480>
141279b91: 0f 10 85 38 01 00 00        	movups	0x138(%rbp), %xmm0
141279b98: 0f 10 8d 48 01 00 00        	movups	0x148(%rbp), %xmm1
141279b9f: 0f 10 95 58 01 00 00        	movups	0x158(%rbp), %xmm2
141279ba6: 0f 10 9d 68 01 00 00        	movups	0x168(%rbp), %xmm3
141279bad: 0f 11 85 c0 01 00 00        	movups	%xmm0, 0x1c0(%rbp)
141279bb4: 0f 11 8d d0 01 00 00        	movups	%xmm1, 0x1d0(%rbp)
141279bbb: 0f 11 95 e0 01 00 00        	movups	%xmm2, 0x1e0(%rbp)
141279bc2: 0f 11 9d f0 01 00 00        	movups	%xmm3, 0x1f0(%rbp)
141279bc9: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
141279bd0: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
141279bd7: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
141279bde: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
141279be5: c7 85 00 02 00 00 00 00 00 00       	movl	$0x0, 0x200(%rbp)
141279bef: c7 85 08 02 00 00 00 00 00 00       	movl	$0x0, 0x208(%rbp)
141279bf9: c6 85 10 02 00 00 01        	movb	$0x1, 0x210(%rbp)
141279c00: 0f b6 85 30 02 00 00        	movzbl	0x230(%rbp), %eax
141279c07: 88 85 11 02 00 00           	movb	%al, 0x211(%rbp)
141279c0d: c6 85 12 02 00 00 00        	movb	$0x0, 0x212(%rbp)
141279c14: 66 c7 85 14 02 00 00 02 40  	movw	$0x4002, 0x214(%rbp)    # imm = 0x4002
141279c1d: 89 bd 18 02 00 00           	movl	%edi, 0x218(%rbp)
141279c23: 44 88 a5 1c 02 00 00        	movb	%r12b, 0x21c(%rbp)
141279c2a: c6 85 44 01 00 00 02        	movb	$0x2, 0x144(%rbp)
141279c31: 48 8b 85 88 03 00 00        	movq	0x388(%rbp), %rax
141279c38: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141279c3d: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
141279c44: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
141279c49: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
141279c50: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
141279c55: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
141279c5c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141279c61: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
141279c68: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
141279c6f: 0f b6 95 98 03 00 00        	movzbl	0x398(%rbp), %edx
141279c76: 4c 8b 8d 78 01 00 00        	movq	0x178(%rbp), %r9
141279c7d: e8 ae d1 ff ff              	callq	0x141276e30 <.text+0x1275e30>
141279c82: 41 be 01 00 00 00           	movl	$0x1, %r14d
141279c88: 48 83 fb 01                 	cmpq	$0x1, %rbx
141279c8c: 0f 84 11 02 00 00           	je	0x141279ea3 <.text+0x1278ea3>
141279c92: 49 83 ff 03                 	cmpq	$0x3, %r15
141279c96: 48 c7 85 28 01 00 00 02 00 00 00    	movq	$0x2, 0x128(%rbp)
141279ca1: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
141279ca8: 49 0f 43 c5                 	cmovaeq	%r13, %rax
141279cac: 44 8b 60 10                 	movl	0x10(%rax), %r12d
141279cb0: 41 83 fc 02                 	cmpl	$0x2, %r12d
141279cb4: 0f 84 b3 01 00 00           	je	0x141279e6d <.text+0x1278e6d>
141279cba: 48 8b 48 14                 	movq	0x14(%rax), %rcx
141279cbe: 48 89 8d 30 02 00 00        	movq	%rcx, 0x230(%rbp)
141279cc5: 40 8a 78 1c                 	movb	0x1c(%rax), %dil
141279cc9: 48 8b 8d 70 03 00 00        	movq	0x370(%rbp), %rcx
141279cd0: e8 0b 9d 5b ff              	callq	0x1408339e0 <.text+0x8329e0>
141279cd5: 41 89 c6                    	movl	%eax, %r14d
141279cd8: 48 8b b5 a8 01 00 00        	movq	0x1a8(%rbp), %rsi
141279cdf: 48 3b b5 98 01 00 00        	cmpq	0x198(%rbp), %rsi
141279ce6: 75 13                       	jne	0x141279cfb <.text+0x1278cfb>
141279ce8: 48 8d 15 c9 9e 9a 01        	leaq	0x19a9ec9(%rip), %rdx   # 0x142c23bb8
141279cef: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
141279cf6: e8 75 5b 8e ff              	callq	0x140b5f870 <.text+0xb5e870>
141279cfb: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
141279d02: 44 89 34 b0                 	movl	%r14d, (%rax,%rsi,4)
141279d06: 48 ff c6                    	incq	%rsi
141279d09: 48 89 b5 a8 01 00 00        	movq	%rsi, 0x1a8(%rbp)
141279d10: 44 89 b5 38 01 00 00        	movl	%r14d, 0x138(%rbp)
141279d17: 4c 8d 8d 3c 01 00 00        	leaq	0x13c(%rbp), %r9
141279d1e: 44 89 a5 3c 01 00 00        	movl	%r12d, 0x13c(%rbp)
141279d25: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
141279d2c: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
141279d33: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
141279d3a: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
141279d41: 45 89 f0                    	movl	%r14d, %r8d
141279d44: e8 37 07 00 00              	callq	0x14127a480 <.text+0x1279480>
141279d49: 83 bd b0 01 00 00 03        	cmpl	$0x3, 0x1b0(%rbp)
141279d50: 75 12                       	jne	0x141279d64 <.text+0x1278d64>
141279d52: 48 8b 45 40                 	movq	0x40(%rbp), %rax
141279d56: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
141279d5a: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
141279d5e: 0f 28 4d 30                 	movaps	0x30(%rbp), %xmm1
141279d62: eb 19                       	jmp	0x141279d7d <.text+0x1278d7d>
141279d64: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
141279d6b: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
141279d6f: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
141279d76: 0f 10 8d d0 01 00 00        	movups	0x1d0(%rbp), %xmm1
141279d7d: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
141279d81: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
141279d85: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
141279d8c: 0f 11 85 f0 01 00 00        	movups	%xmm0, 0x1f0(%rbp)
141279d93: 8b 85 c0 00 00 00           	movl	0xc0(%rbp), %eax
141279d99: 89 85 00 02 00 00           	movl	%eax, 0x200(%rbp)
141279d9f: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
141279da6: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
141279dad: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
141279db4: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
141279dbb: 0f 28 45 50                 	movaps	0x50(%rbp), %xmm0
141279dbf: 0f 28 4d 60                 	movaps	0x60(%rbp), %xmm1
141279dc3: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
141279dca: 0f 11 8d d8 01 00 00        	movups	%xmm1, 0x1d8(%rbp)
141279dd1: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141279dd5: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
141279ddc: 44 89 b5 04 02 00 00        	movl	%r14d, 0x204(%rbp)
141279de3: 8b 85 24 02 00 00           	movl	0x224(%rbp), %eax
141279de9: 89 85 08 02 00 00           	movl	%eax, 0x208(%rbp)
141279def: c6 85 0c 02 00 00 01        	movb	$0x1, 0x20c(%rbp)
141279df6: 0f b6 85 46 02 00 00        	movzbl	0x246(%rbp), %eax
141279dfd: 88 85 0d 02 00 00           	movb	%al, 0x20d(%rbp)
141279e03: 40 88 bd 0e 02 00 00        	movb	%dil, 0x20e(%rbp)
141279e0a: 48 c7 85 b0 01 00 00 03 00 00 00    	movq	$0x3, 0x1b0(%rbp)
141279e15: c6 85 44 01 00 00 02        	movb	$0x2, 0x144(%rbp)
141279e1c: 48 8b 85 88 03 00 00        	movq	0x388(%rbp), %rax
141279e23: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141279e28: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
141279e2f: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
141279e34: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
141279e3b: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
141279e40: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
141279e47: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141279e4c: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
141279e53: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
141279e5a: 0f b6 95 98 03 00 00        	movzbl	0x398(%rbp), %edx
141279e61: 4c 8b 8d 78 01 00 00        	movq	0x178(%rbp), %r9
141279e68: e8 c3 cf ff ff              	callq	0x141276e30 <.text+0x1275e30>
141279e6d: 41 be 02 00 00 00           	movl	$0x2, %r14d
141279e73: 48 83 fb 02                 	cmpq	$0x2, %rbx
141279e77: 74 2a                       	je	0x141279ea3 <.text+0x1278ea3>
141279e79: 49 83 ff 03                 	cmpq	$0x3, %r15
141279e7d: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
141279e84: 49 0f 43 c5                 	cmovaeq	%r13, %rax
141279e88: 48 c7 85 28 01 00 00 03 00 00 00    	movq	$0x3, 0x128(%rbp)
141279e93: 41 be 03 00 00 00           	movl	$0x3, %r14d
141279e99: 83 78 20 02                 	cmpl	$0x2, 0x20(%rax)
141279e9d: 0f 85 42 02 00 00           	jne	0x14127a0e5 <.text+0x12790e5>
141279ea3: 48 8b 8d 70 03 00 00        	movq	0x370(%rbp), %rcx
141279eaa: e8 31 9b 5b ff              	callq	0x1408339e0 <.text+0x8329e0>
141279eaf: 89 c6                       	movl	%eax, %esi
141279eb1: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
141279eb8: 48 c1 e1 20                 	shlq	$0x20, %rcx
141279ebc: 8b 85 3c 02 00 00           	movl	0x23c(%rbp), %eax
141279ec2: 48 09 c8                    	orq	%rcx, %rax
141279ec5: 89 b5 38 01 00 00           	movl	%esi, 0x138(%rbp)
141279ecb: 4c 8d 8d 3c 01 00 00        	leaq	0x13c(%rbp), %r9
141279ed2: 8b 8d 40 02 00 00           	movl	0x240(%rbp), %ecx
141279ed8: 89 8d 3c 01 00 00           	movl	%ecx, 0x13c(%rbp)
141279ede: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
141279ee5: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
141279eec: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
141279ef3: 41 89 f0                    	movl	%esi, %r8d
141279ef6: e8 85 05 00 00              	callq	0x14127a480 <.text+0x1279480>
141279efb: 83 bd b0 01 00 00 03        	cmpl	$0x3, 0x1b0(%rbp)
141279f02: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
141279f09: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
141279f0d: 48 0f 45 c8                 	cmovneq	%rax, %rcx
141279f11: 0f 10 01                    	movups	(%rcx), %xmm0
141279f14: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
141279f18: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
141279f1c: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
141279f20: 48 8b 41 20                 	movq	0x20(%rcx), %rax
141279f24: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
141279f28: e8 43 1c dc fe              	callq	0x14003bb70 <.text+0x3ab70>
141279f2d: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
141279f31: 48 89 8d e8 01 00 00        	movq	%rcx, 0x1e8(%rbp)
141279f38: 0f 28 45 c0                 	movaps	-0x40(%rbp), %xmm0
141279f3c: 0f 28 4d d0                 	movaps	-0x30(%rbp), %xmm1
141279f40: 0f 11 8d d8 01 00 00        	movups	%xmm1, 0x1d8(%rbp)
141279f47: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
141279f4e: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
141279f55: 0f 11 85 f0 01 00 00        	movups	%xmm0, 0x1f0(%rbp)
141279f5c: 8b 8d c0 00 00 00           	movl	0xc0(%rbp), %ecx
141279f62: 89 8d 00 02 00 00           	movl	%ecx, 0x200(%rbp)
141279f68: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
141279f6f: 48 89 8d b8 01 00 00        	movq	%rcx, 0x1b8(%rbp)
141279f76: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
141279f7d: 48 89 8d c0 01 00 00        	movq	%rcx, 0x1c0(%rbp)
141279f84: 89 b5 04 02 00 00           	movl	%esi, 0x204(%rbp)
141279f8a: 8b 8d 24 02 00 00           	movl	0x224(%rbp), %ecx
141279f90: 89 8d 08 02 00 00           	movl	%ecx, 0x208(%rbp)
141279f96: c6 85 0c 02 00 00 01        	movb	$0x1, 0x20c(%rbp)
141279f9d: 0f b6 8d 46 02 00 00        	movzbl	0x246(%rbp), %ecx
141279fa4: 88 8d 0d 02 00 00           	movb	%cl, 0x20d(%rbp)
141279faa: 88 85 0e 02 00 00           	movb	%al, 0x20e(%rbp)
141279fb0: 48 c7 85 b0 01 00 00 03 00 00 00    	movq	$0x3, 0x1b0(%rbp)
141279fbb: c6 85 44 01 00 00 02        	movb	$0x2, 0x144(%rbp)
141279fc2: 48 8b 85 88 03 00 00        	movq	0x388(%rbp), %rax
141279fc9: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141279fce: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
141279fd5: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
141279fda: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
141279fe1: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
141279fe6: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
141279fed: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141279ff2: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
141279ff9: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
14127a000: 0f b6 95 98 03 00 00        	movzbl	0x398(%rbp), %edx
14127a007: 4c 8b 8d 78 01 00 00        	movq	0x178(%rbp), %r9
14127a00e: e8 1d ce ff ff              	callq	0x141276e30 <.text+0x1275e30>
14127a013: 49 83 ff 03                 	cmpq	$0x3, %r15
14127a017: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
14127a01e: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14127a022: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
14127a026: 0f 10 85 98 01 00 00        	movups	0x198(%rbp), %xmm0
14127a02d: 0f 11 41 08                 	movups	%xmm0, 0x8(%rcx)
14127a031: 89 31                       	movl	%esi, (%rcx)
14127a033: 48 89 ce                    	movq	%rcx, %rsi
14127a036: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14127a03d: 49 0f 43 cd                 	cmovaeq	%r13, %rcx
14127a041: 44 89 f0                    	movl	%r14d, %eax
14127a044: c1 e0 04                    	shll	$0x4, %eax
14127a047: 48 01 c8                    	addq	%rcx, %rax
14127a04a: 49 ff c6                    	incq	%r14
14127a04d: 48 f7 db                    	negq	%rbx
14127a050: 4a 8d 0c 33                 	leaq	(%rbx,%r14), %rcx
14127a054: 48 83 f9 01                 	cmpq	$0x1, %rcx
14127a058: 74 13                       	je	0x14127a06d <.text+0x127906d>
14127a05a: 4c 89 b5 28 01 00 00        	movq	%r14, 0x128(%rbp)
14127a061: 49 ff c6                    	incq	%r14
14127a064: 83 38 02                    	cmpl	$0x2, (%rax)
14127a067: 48 8d 40 10                 	leaq	0x10(%rax), %rax
14127a06b: 75 e3                       	jne	0x14127a050 <.text+0x1279050>
14127a06d: 49 83 ff 03                 	cmpq	$0x3, %r15
14127a071: 72 15                       	jb	0x14127a088 <.text+0x1279088>
14127a073: 49 c1 e7 04                 	shlq	$0x4, %r15
14127a077: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14127a07d: 4c 89 e9                    	movq	%r13, %rcx
14127a080: 4c 89 fa                    	movq	%r15, %rdx
14127a083: e8 98 9b 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127a088: 48 89 f0                    	movq	%rsi, %rax
14127a08b: 0f 28 b5 50 02 00 00        	movaps	0x250(%rbp), %xmm6
14127a092: 0f 28 bd 60 02 00 00        	movaps	0x260(%rbp), %xmm7
14127a099: 44 0f 28 85 70 02 00 00     	movaps	0x270(%rbp), %xmm8
14127a0a1: 44 0f 28 8d 80 02 00 00     	movaps	0x280(%rbp), %xmm9
14127a0a9: 44 0f 28 95 90 02 00 00     	movaps	0x290(%rbp), %xmm10
14127a0b1: 44 0f 28 9d a0 02 00 00     	movaps	0x2a0(%rbp), %xmm11
14127a0b9: 44 0f 28 a5 b0 02 00 00     	movaps	0x2b0(%rbp), %xmm12
14127a0c1: 44 0f 28 ad c0 02 00 00     	movaps	0x2c0(%rbp), %xmm13
14127a0c9: 44 0f 28 b5 d0 02 00 00     	movaps	0x2d0(%rbp), %xmm14
14127a0d1: 48 81 c4 68 03 00 00        	addq	$0x368, %rsp            # imm = 0x368
14127a0d8: 5b                          	popq	%rbx
14127a0d9: 5f                          	popq	%rdi
14127a0da: 5e                          	popq	%rsi
14127a0db: 41 5c                       	popq	%r12
14127a0dd: 41 5d                       	popq	%r13
14127a0df: 41 5e                       	popq	%r14
14127a0e1: 41 5f                       	popq	%r15
14127a0e3: 5d                          	popq	%rbp
14127a0e4: c3                          	retq
14127a0e5: 48 8d 0d e4 9a 9a 01        	leaq	0x19a9ae4(%rip), %rcx   # 0x142c23bd0
14127a0ec: 4c 8d 05 15 9b 9a 01        	leaq	0x19a9b15(%rip), %r8    # 0x142c23c08
14127a0f3: ba 32 00 00 00              	movl	$0x32, %edx
14127a0f8: e8 33 f3 65 01              	callq	0x1428d9430 <.text+0x28d8430>
14127a0fd: e9 95 01 00 00              	jmp	0x14127a297 <.text+0x1279297>
14127a102: 48 8d 0d 4f 9a 9a 01        	leaq	0x19a9a4f(%rip), %rcx   # 0x142c23b58
14127a109: e8 02 f2 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127a10e: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
14127a115: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
14127a11c: 48 8d 05 cd 2d fe ff        	leaq	-0x1d233(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
14127a123: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
14127a12a: 48 8d 05 4f 7b 9a 01        	leaq	0x19a7b4f(%rip), %rax   # 0x142c21c80
14127a131: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14127a138: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14127a143: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
14127a14e: 48 8d 85 38 01 00 00        	leaq	0x138(%rbp), %rax
14127a155: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14127a15c: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
14127a167: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
14127a16e: 48 8d 15 1b 7b 9a 01        	leaq	0x19a7b1b(%rip), %rdx   # 0x142c21c90
14127a175: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14127a17c: e8 0f f2 65 01              	callq	0x1428d9390 <.text+0x28d8390>
14127a181: e9 11 01 00 00              	jmp	0x14127a297 <.text+0x1279297>
14127a186: 48 89 b5 38 01 00 00        	movq	%rsi, 0x138(%rbp)
14127a18d: 48 8d 05 5c 2d fe ff        	leaq	-0x1d2a4(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
14127a194: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
14127a19b: 48 8d 05 de 7a 9a 01        	leaq	0x19a7ade(%rip), %rax   # 0x142c21c80
14127a1a2: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14127a1a9: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14127a1b4: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
14127a1bf: 48 8d 85 38 01 00 00        	leaq	0x138(%rbp), %rax
14127a1c6: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14127a1cd: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
14127a1d8: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
14127a1df: 48 8d 15 aa 7a 9a 01        	leaq	0x19a7aaa(%rip), %rdx   # 0x142c21c90
14127a1e6: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14127a1ed: e8 9e f1 65 01              	callq	0x1428d9390 <.text+0x28d8390>
14127a1f2: e9 a0 00 00 00              	jmp	0x14127a297 <.text+0x1279297>
14127a1f7: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
14127a1fe: 48 8d 0d 6b 99 9a 01        	leaq	0x19a996b(%rip), %rcx   # 0x142c23b70
14127a205: e8 06 f1 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127a20a: e9 88 00 00 00              	jmp	0x14127a297 <.text+0x1279297>
14127a20f: 48 8d 85 88 00 00 00        	leaq	0x88(%rbp), %rax
14127a216: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
14127a21d: 48 8d 05 cc 2c fe ff        	leaq	-0x1d334(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
14127a224: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
14127a22b: 48 8d 05 4e 7a 9a 01        	leaq	0x19a7a4e(%rip), %rax   # 0x142c21c80
14127a232: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14127a239: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14127a244: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
14127a24f: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
14127a256: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14127a25d: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
14127a268: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
14127a26f: 48 8d 15 1a 7a 9a 01        	leaq	0x19a7a1a(%rip), %rdx   # 0x142c21c90
14127a276: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14127a27d: e8 0e f1 65 01              	callq	0x1428d9390 <.text+0x28d8390>
14127a282: eb 13                       	jmp	0x14127a297 <.text+0x1279297>
14127a284: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
14127a28b: 48 8d 0d f6 98 9a 01        	leaq	0x19a98f6(%rip), %rcx   # 0x142c23b88
14127a292: e8 79 f0 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127a297: 0f 0b                       	ud2
14127a299: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14127a2a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14127a2a5: 55                          	pushq	%rbp
14127a2a6: 41 57                       	pushq	%r15
14127a2a8: 41 56                       	pushq	%r14
14127a2aa: 41 55                       	pushq	%r13
14127a2ac: 41 54                       	pushq	%r12
14127a2ae: 56                          	pushq	%rsi
14127a2af: 57                          	pushq	%rdi
14127a2b0: 53                          	pushq	%rbx
14127a2b1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
14127a2b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14127a2bf: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
14127a2c5: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
14127a2cb: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
14127a2d1: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14127a2d7: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
14127a2e0: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
14127a2e9: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
14127a2f2: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
14127a2fa: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
14127a302: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
14127a309: 48 85 d2                    	testq	%rdx, %rdx
14127a30c: 74 16                       	je	0x14127a324 <.text+0x1279324>
14127a30e: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
14127a315: 48 c1 e2 02                 	shlq	$0x2, %rdx
14127a319: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14127a31f: e8 fc 98 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127a324: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14127a32b: e8 c0 ef fd ff              	callq	0x1412592f0 <.text+0x12582f0>
14127a330: c6 85 47 02 00 00 00        	movb	$0x0, 0x247(%rbp)
14127a337: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
14127a33f: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
14127a347: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14127a350: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
14127a359: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
14127a362: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
14127a368: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
14127a36e: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
14127a374: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
14127a37a: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
14127a381: 5b                          	popq	%rbx
14127a382: 5f                          	popq	%rdi
14127a383: 5e                          	popq	%rsi
14127a384: 41 5c                       	popq	%r12
14127a386: 41 5d                       	popq	%r13
14127a388: 41 5e                       	popq	%r14
14127a38a: 41 5f                       	popq	%r15
14127a38c: 5d                          	popq	%rbp
14127a38d: c3                          	retq
14127a38e: 66 90                       	nop
14127a390: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14127a395: 55                          	pushq	%rbp
14127a396: 41 57                       	pushq	%r15
14127a398: 41 56                       	pushq	%r14
14127a39a: 41 55                       	pushq	%r13
14127a39c: 41 54                       	pushq	%r12
14127a39e: 56                          	pushq	%rsi
14127a39f: 57                          	pushq	%rdi
14127a3a0: 53                          	pushq	%rbx
14127a3a1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
14127a3a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14127a3af: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
14127a3b5: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
14127a3bb: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
14127a3c1: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14127a3c7: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
14127a3d0: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
14127a3d9: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
14127a3e2: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
14127a3ea: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
14127a3f2: 80 bd 47 02 00 00 00        	cmpb	$0x0, 0x247(%rbp)
14127a3f9: 74 23                       	je	0x14127a41e <.text+0x127941e>
14127a3fb: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
14127a402: 48 83 fa 03                 	cmpq	$0x3, %rdx
14127a406: 72 16                       	jb	0x14127a41e <.text+0x127941e>
14127a408: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
14127a40f: 48 c1 e2 04                 	shlq	$0x4, %rdx
14127a413: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14127a419: e8 02 98 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127a41e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
14127a426: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
14127a42e: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14127a437: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
14127a440: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
14127a449: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
14127a44f: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
14127a455: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
14127a45b: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
14127a461: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
14127a468: 5b                          	popq	%rbx
14127a469: 5f                          	popq	%rdi
14127a46a: 5e                          	popq	%rsi
14127a46b: 41 5c                       	popq	%r12
14127a46d: 41 5d                       	popq	%r13
14127a46f: 41 5e                       	popq	%r14
14127a471: 41 5f                       	popq	%r15
14127a473: 5d                          	popq	%rbp
14127a474: c3                          	retq
14127a475: cc                          	int3
14127a476: cc                          	int3
14127a477: cc                          	int3
14127a478: cc                          	int3
14127a479: cc                          	int3
14127a47a: cc                          	int3
14127a47b: cc                          	int3
14127a47c: cc                          	int3
14127a47d: cc                          	int3
14127a47e: cc                          	int3
14127a47f: cc                          	int3
14127a480: 41 57                       	pushq	%r15
14127a482: 41 56                       	pushq	%r14
14127a484: 41 55                       	pushq	%r13
14127a486: 41 54                       	pushq	%r12
14127a488: 56                          	pushq	%rsi
14127a489: 57                          	pushq	%rdi
14127a48a: 55                          	pushq	%rbp
14127a48b: 53                          	pushq	%rbx
14127a48c: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14127a493: 44 0f 29 8c 24 b0 00 00 00  	movaps	%xmm9, 0xb0(%rsp)
14127a49c: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
14127a4a5: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14127a4ad: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
14127a4b5: 4d 89 cc                    	movq	%r9, %r12
14127a4b8: 44 89 c7                    	movl	%r8d, %edi
14127a4bb: 49 89 d6                    	movq	%rdx, %r14
14127a4be: 48 89 ce                    	movq	%rcx, %rsi
14127a4c1: 4c 8b 3a                    	movq	(%rdx), %r15
14127a4c4: 48 8b 5a 08                 	movq	0x8(%rdx), %rbx
14127a4c8: 48 89 d9                    	movq	%rbx, %rcx
14127a4cb: e8 c0 1d 7a ff              	callq	0x140a1c290 <.text+0xa1b290>
14127a4d0: a8 01                       	testb	$0x1, %al
14127a4d2: 0f 84 00 02 00 00           	je	0x14127a6d8 <.text+0x12796d8>
14127a4d8: 4c 8d 05 59 97 9a 01        	leaq	0x19a9759(%rip), %r8    # 0x142c23c38
14127a4df: 4c 89 f9                    	movq	%r15, %rcx
14127a4e2: e8 09 7a 81 ff              	callq	0x140a91ef0 <.text+0xa90ef0>
14127a4e7: 48 85 c0                    	testq	%rax, %rax
14127a4ea: 0f 84 f4 01 00 00           	je	0x14127a6e4 <.text+0x12796e4>
14127a4f0: 49 89 c7                    	movq	%rax, %r15
14127a4f3: 49 8b 46 10                 	movq	0x10(%r14), %rax
14127a4f7: f6 00 01                    	testb	$0x1, (%rax)
14127a4fa: 0f 84 f0 01 00 00           	je	0x14127a6f0 <.text+0x12796f0>
14127a500: 45 8b 2c 24                 	movl	(%r12), %r13d
14127a504: 41 f6 c5 01                 	testb	$0x1, %r13b
14127a508: 75 05                       	jne	0x14127a50f <.text+0x127950f>
14127a50a: 0f 57 ff                    	xorps	%xmm7, %xmm7
14127a50d: eb 07                       	jmp	0x14127a516 <.text+0x1279516>
14127a50f: f3 41 0f 10 7c 24 04        	movss	0x4(%r12), %xmm7
14127a516: f3 0f 10 70 08              	movss	0x8(%rax), %xmm6
14127a51b: f3 45 0f 10 44 24 08        	movss	0x8(%r12), %xmm8
14127a522: f3 0f 58 78 04              	addss	0x4(%rax), %xmm7
14127a527: 4c 89 f9                    	movq	%r15, %rcx
14127a52a: e8 11 02 8b ff              	callq	0x140b2a740 <.text+0xb29740>
14127a52f: 89 44 24 50                 	movl	%eax, 0x50(%rsp)
14127a533: f3 0f 11 44 24 54           	movss	%xmm0, 0x54(%rsp)
14127a539: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
14127a53e: 0f 28 cf                    	movaps	%xmm7, %xmm1
14127a541: e8 fa 78 a1 ff              	callq	0x140c91e40 <.text+0xc90e40>
14127a546: 83 f8 01                    	cmpl	$0x1, %eax
14127a549: 0f 85 ad 01 00 00           	jne	0x14127a6fc <.text+0x12796fc>
14127a54f: 0f 28 f8                    	movaps	%xmm0, %xmm7
14127a552: 48 89 5c 24 48              	movq	%rbx, 0x48(%rsp)
14127a557: 41 f6 c5 01                 	testb	$0x1, %r13b
14127a55b: 75 04                       	jne	0x14127a561 <.text+0x1279561>
14127a55d: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
14127a561: f3 41 0f 58 f0              	addss	%xmm8, %xmm6
14127a566: f3 0f 11 7c 24 28           	movss	%xmm7, 0x28(%rsp)
14127a56c: f3 0f 11 74 24 2c           	movss	%xmm6, 0x2c(%rsp)
14127a572: 4c 89 f9                    	movq	%r15, %rcx
14127a575: 0f 28 cf                    	movaps	%xmm7, %xmm1
14127a578: 0f 28 d6                    	movaps	%xmm6, %xmm2
14127a57b: e8 10 03 8b ff              	callq	0x140b2a890 <.text+0xb29890>
14127a580: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
14127a584: e8 e7 7d d8 fe              	callq	0x140002370 <.text+0x1370>
14127a589: 49 8b 5e 18                 	movq	0x18(%r14), %rbx
14127a58d: 80 7b 22 02                 	cmpb	$0x2, 0x22(%rbx)
14127a591: 0f 84 71 01 00 00           	je	0x14127a708 <.text+0x1279708>
14127a597: 88 44 24 27                 	movb	%al, 0x27(%rsp)
14127a59b: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14127a59f: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14127a5a4: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
14127a5a9: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
14127a5ae: 4d 89 f8                    	movq	%r15, %r8
14127a5b1: e8 da 7a a1 ff              	callq	0x140c92090 <.text+0xc91090>
14127a5b6: 0f b6 6b 22                 	movzbl	0x22(%rbx), %ebp
14127a5ba: 40 80 fd 02                 	cmpb	$0x2, %bpl
14127a5be: 0f 84 50 01 00 00           	je	0x14127a714 <.text+0x1279714>
14127a5c4: 0f b6 5b 21                 	movzbl	0x21(%rbx), %ebx
14127a5c8: 8b 44 24 58                 	movl	0x58(%rsp), %eax
14127a5cc: 89 44 24 38                 	movl	%eax, 0x38(%rsp)
14127a5d0: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
14127a5d5: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14127a5da: 4d 8b 76 20                 	movq	0x20(%r14), %r14
14127a5de: 48 8d 4c 24 60              	leaq	0x60(%rsp), %rcx
14127a5e3: 4c 8d 64 24 28              	leaq	0x28(%rsp), %r12
14127a5e8: 4c 89 e2                    	movq	%r12, %rdx
14127a5eb: 4d 89 f8                    	movq	%r15, %r8
14127a5ee: e8 9d 7a a1 ff              	callq	0x140c92090 <.text+0xc91090>
14127a5f3: 4c 89 e1                    	movq	%r12, %rcx
14127a5f6: 4c 89 fa                    	movq	%r15, %rdx
14127a5f9: e8 f2 7e a1 ff              	callq	0x140c924f0 <.text+0xc914f0>
14127a5fe: 0f 57 0d eb b2 6a 01        	xorps	0x16ab2eb(%rip), %xmm1  # 0x1429258f0
14127a605: 4c 8d 7c 24 50              	leaq	0x50(%rsp), %r15
14127a60a: 4c 89 f9                    	movq	%r15, %rcx
14127a60d: 0f 28 d0                    	movaps	%xmm0, %xmm2
14127a610: e8 bb 69 a1 ff              	callq	0x140c90fd0 <.text+0xc8ffd0>
14127a615: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
14127a61f: 48 89 44 24 6c              	movq	%rax, 0x6c(%rsp)
14127a624: c7 44 24 74 00 00 80 3f     	movl	$0x3f800000, 0x74(%rsp) # imm = 0x3F800000
14127a62c: 4c 89 f1                    	movq	%r14, %rcx
14127a62f: 89 fa                       	movl	%edi, %edx
14127a631: 4d 89 f8                    	movq	%r15, %r8
14127a634: e8 67 07 7a ff              	callq	0x140a1ada0 <.text+0xa19da0>
14127a639: 45 85 ed                    	testl	%r13d, %r13d
14127a63c: 74 5b                       	je	0x14127a699 <.text+0x1279699>
14127a63e: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
14127a642: f3 45 0f 5c c8              	subss	%xmm8, %xmm9
14127a647: 48 8b 44 24 48              	movq	0x48(%rsp), %rax
14127a64c: 4c 8b 30                    	movq	(%rax), %r14
14127a64f: 48 8b 78 08                 	movq	0x8(%rax), %rdi
14127a653: 8b 44 24 38                 	movl	0x38(%rsp), %eax
14127a657: 89 46 2c                    	movl	%eax, 0x2c(%rsi)
14127a65a: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
14127a65f: 48 89 46 24                 	movq	%rax, 0x24(%rsi)
14127a663: e8 08 15 dc fe              	callq	0x14003bb70 <.text+0x3ab70>
14127a668: 48 89 7e 08                 	movq	%rdi, 0x8(%rsi)
14127a66c: f3 0f 11 7e 10              	movss	%xmm7, 0x10(%rsi)
14127a671: f3 0f 11 76 14              	movss	%xmm6, 0x14(%rsi)
14127a676: 48 8b 4c 24 40              	movq	0x40(%rsp), %rcx
14127a67b: 48 89 4e 18                 	movq	%rcx, 0x18(%rsi)
14127a67f: f3 44 0f 11 4e 20           	movss	%xmm9, 0x20(%rsi)
14127a685: 0f b6 4c 24 27              	movzbl	0x27(%rsp), %ecx
14127a68a: 88 4e 30                    	movb	%cl, 0x30(%rsi)
14127a68d: 88 5e 31                    	movb	%bl, 0x31(%rsi)
14127a690: 40 88 6e 32                 	movb	%bpl, 0x32(%rsi)
14127a694: 88 46 38                    	movb	%al, 0x38(%rsi)
14127a697: eb 06                       	jmp	0x14127a69f <.text+0x127969f>
14127a699: 41 be 03 00 00 00           	movl	$0x3, %r14d
14127a69f: 4c 89 36                    	movq	%r14, (%rsi)
14127a6a2: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14127a6aa: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14127a6b2: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14127a6bb: 44 0f 28 8c 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm9
14127a6c4: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14127a6cb: 5b                          	popq	%rbx
14127a6cc: 5d                          	popq	%rbp
14127a6cd: 5f                          	popq	%rdi
14127a6ce: 5e                          	popq	%rsi
14127a6cf: 41 5c                       	popq	%r12
14127a6d1: 41 5d                       	popq	%r13
14127a6d3: 41 5e                       	popq	%r14
14127a6d5: 41 5f                       	popq	%r15
14127a6d7: c3                          	retq
14127a6d8: 48 8d 0d 41 95 9a 01        	leaq	0x19a9541(%rip), %rcx   # 0x142c23c20
14127a6df: e8 2c ec 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127a6e4: 48 8d 0d 65 95 9a 01        	leaq	0x19a9565(%rip), %rcx   # 0x142c23c50
14127a6eb: e8 20 ec 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127a6f0: 48 8d 0d 71 95 9a 01        	leaq	0x19a9571(%rip), %rcx   # 0x142c23c68
14127a6f7: e8 14 ec 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127a6fc: 48 8d 0d 7d 95 9a 01        	leaq	0x19a957d(%rip), %rcx   # 0x142c23c80
14127a703: e8 08 ec 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127a708: 48 8d 0d 89 95 9a 01        	leaq	0x19a9589(%rip), %rcx   # 0x142c23c98
14127a70f: e8 fc eb 65 01              	callq	0x1428d9310 <.text+0x28d8310>
14127a714: 48 8d 0d 95 95 9a 01        	leaq	0x19a9595(%rip), %rcx   # 0x142c23cb0
14127a71b: e8 f0 eb 65 01              	callq	0x1428d9310 <.text+0x28d8310>
