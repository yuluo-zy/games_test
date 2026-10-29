
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409a4fb0: 55                          	pushq	%rbp
1409a4fb1: 41 57                       	pushq	%r15
1409a4fb3: 41 56                       	pushq	%r14
1409a4fb5: 41 55                       	pushq	%r13
1409a4fb7: 41 54                       	pushq	%r12
1409a4fb9: 56                          	pushq	%rsi
1409a4fba: 57                          	pushq	%rdi
1409a4fbb: 53                          	pushq	%rbx
1409a4fbc: 48 81 ec 58 04 00 00        	subq	$0x458, %rsp            # imm = 0x458
1409a4fc3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409a4fcb: 44 0f 29 b5 c0 03 00 00     	movaps	%xmm14, 0x3c0(%rbp)
1409a4fd3: 44 0f 29 ad b0 03 00 00     	movaps	%xmm13, 0x3b0(%rbp)
1409a4fdb: 44 0f 29 a5 a0 03 00 00     	movaps	%xmm12, 0x3a0(%rbp)
1409a4fe3: 44 0f 29 9d 90 03 00 00     	movaps	%xmm11, 0x390(%rbp)
1409a4feb: 44 0f 29 95 80 03 00 00     	movaps	%xmm10, 0x380(%rbp)
1409a4ff3: 44 0f 29 8d 70 03 00 00     	movaps	%xmm9, 0x370(%rbp)
1409a4ffb: 44 0f 29 85 60 03 00 00     	movaps	%xmm8, 0x360(%rbp)
1409a5003: 0f 29 bd 50 03 00 00        	movaps	%xmm7, 0x350(%rbp)
1409a500a: 0f 29 b5 40 03 00 00        	movaps	%xmm6, 0x340(%rbp)
1409a5011: 48 c7 85 38 03 00 00 fe ff ff ff    	movq	$-0x2, 0x338(%rbp)
1409a501c: 48 8b 01                    	movq	(%rcx), %rax
1409a501f: 83 38 05                    	cmpl	$0x5, (%rax)
1409a5022: 0f 85 be 24 00 00           	jne	0x1409a74e6 <.text+0x9a64e6>
1409a5028: 4c 89 cb                    	movq	%r9, %rbx
1409a502b: 4c 89 c6                    	movq	%r8, %rsi
1409a502e: 48 8b 8d 80 04 00 00        	movq	0x480(%rbp), %rcx
1409a5035: 4c 8b 85 68 04 00 00        	movq	0x468(%rbp), %r8
1409a503c: 4d 8b 08                    	movq	(%r8), %r9
1409a503f: 4c 89 8d 88 00 00 00        	movq	%r9, 0x88(%rbp)
1409a5046: 4d 8b 58 20                 	movq	0x20(%r8), %r11
1409a504a: 4d 8b 48 30                 	movq	0x30(%r8), %r9
1409a504e: 4c 89 8d 98 00 00 00        	movq	%r9, 0x98(%rbp)
1409a5055: 45 8b 48 3c                 	movl	0x3c(%r8), %r9d
1409a5059: 44 89 8d 2c 01 00 00        	movl	%r9d, 0x12c(%rbp)
1409a5060: 4d 8b 50 40                 	movq	0x40(%r8), %r10
1409a5064: 4d 8b 48 50                 	movq	0x50(%r8), %r9
1409a5068: 4c 89 8d 90 00 00 00        	movq	%r9, 0x90(%rbp)
1409a506f: 45 8b 48 5c                 	movl	0x5c(%r8), %r9d
1409a5073: 44 89 8d 28 01 00 00        	movl	%r9d, 0x128(%rbp)
1409a507a: 4d 8b 48 60                 	movq	0x60(%r8), %r9
1409a507e: 4c 89 8d d8 00 00 00        	movq	%r9, 0xd8(%rbp)
1409a5085: 4d 8b 48 70                 	movq	0x70(%r8), %r9
1409a5089: 4c 89 8d 80 00 00 00        	movq	%r9, 0x80(%rbp)
1409a5090: 45 8b 40 7c                 	movl	0x7c(%r8), %r8d
1409a5094: 44 89 85 24 01 00 00        	movl	%r8d, 0x124(%rbp)
1409a509b: 8b 40 04                    	movl	0x4(%rax), %eax
1409a509e: 89 85 30 03 00 00           	movl	%eax, 0x330(%rbp)
1409a50a4: 48 8b 01                    	movq	(%rcx), %rax
1409a50a7: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1409a50ae: 48 8b 41 20                 	movq	0x20(%rcx), %rax
1409a50b2: 48 89 85 08 03 00 00        	movq	%rax, 0x308(%rbp)
1409a50b9: 4c 8b 69 40                 	movq	0x40(%rcx), %r13
1409a50bd: 48 8b 41 60                 	movq	0x60(%rcx), %rax
1409a50c1: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1409a50c5: 48 8b 81 80 00 00 00        	movq	0x80(%rcx), %rax
1409a50cc: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
1409a50d3: 48 8b 81 a0 00 00 00        	movq	0xa0(%rcx), %rax
1409a50da: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1409a50e1: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409a50e4: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409a50e8: 89 01                       	movl	%eax, (%rcx)
1409a50ea: 4c 8b 22                    	movq	(%rdx), %r12
1409a50ed: 44 8b bd 30 03 00 00        	movl	0x330(%rbp), %r15d
1409a50f4: 44 89 bd 40 02 00 00        	movl	%r15d, 0x240(%rbp)
1409a50fb: 49 83 7c 24 20 00           	cmpq	$0x0, 0x20(%r12)
1409a5101: 4c 8b b5 48 04 00 00        	movq	0x448(%rbp), %r14
1409a5108: 0f 84 b6 00 00 00           	je	0x1409a51c4 <.text+0x9a41c4>
1409a510e: 4c 89 9d 30 01 00 00        	movq	%r11, 0x130(%rbp)
1409a5115: 4c 89 95 38 01 00 00        	movq	%r10, 0x138(%rbp)
1409a511c: 48 8b bd 70 04 00 00        	movq	0x470(%rbp), %rdi
1409a5123: 49 8d 4c 24 28              	leaq	0x28(%r12), %rcx
1409a5128: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1409a512f: e8 9c bd c0 ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1409a5134: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
1409a5139: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
1409a513e: 49 89 d0                    	movq	%rdx, %r8
1409a5141: 49 21 c0                    	andq	%rax, %r8
1409a5144: 48 c1 e8 39                 	shrq	$0x39, %rax
1409a5148: 66 0f 6e c0                 	movd	%eax, %xmm0
1409a514c: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409a5150: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409a5155: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409a515a: 48 8d 41 d0                 	leaq	-0x30(%rcx), %rax
1409a515e: 45 31 c9                    	xorl	%r9d, %r9d
1409a5161: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1409a5165: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1409a516b: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1409a516f: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1409a5173: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1409a5178: 45 85 d2                    	testl	%r10d, %r10d
1409a517b: 74 29                       	je	0x1409a51a6 <.text+0x9a41a6>
1409a517d: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1409a5182: 4d 01 c3                    	addq	%r8, %r11
1409a5185: 49 21 d3                    	andq	%rdx, %r11
1409a5188: 49 f7 db                    	negq	%r11
1409a518b: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1409a518f: 49 c1 e3 04                 	shlq	$0x4, %r11
1409a5193: 46 39 3c 18                 	cmpl	%r15d, (%rax,%r11)
1409a5197: 74 33                       	je	0x1409a51cc <.text+0x9a41cc>
1409a5199: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1409a519d: 66 45 21 d3                 	andw	%r10w, %r11w
1409a51a1: 45 89 da                    	movl	%r11d, %r10d
1409a51a4: 75 d7                       	jne	0x1409a517d <.text+0x9a417d>
1409a51a6: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1409a51aa: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1409a51af: 45 85 d2                    	testl	%r10d, %r10d
1409a51b2: 75 10                       	jne	0x1409a51c4 <.text+0x9a41c4>
1409a51b4: 4d 01 c8                    	addq	%r9, %r8
1409a51b7: 49 83 c0 10                 	addq	$0x10, %r8
1409a51bb: 49 83 c1 10                 	addq	$0x10, %r9
1409a51bf: 49 21 d0                    	andq	%rdx, %r8
1409a51c2: eb a1                       	jmp	0x1409a5165 <.text+0x9a4165>
1409a51c4: 4d 8b 26                    	movq	(%r14), %r12
1409a51c7: e9 b0 21 00 00              	jmp	0x1409a737c <.text+0x9a637c>
1409a51cc: 4a 8b 54 19 e8              	movq	-0x18(%rcx,%r11), %rdx
1409a51d1: 48 85 d2                    	testq	%rdx, %rdx
1409a51d4: 0f 84 94 23 00 00           	je	0x1409a756e <.text+0x9a656e>
1409a51da: 4e 8d 3c 19                 	leaq	(%rcx,%r11), %r15
1409a51de: 4d 8b 06                    	movq	(%r14), %r8
1409a51e1: f3 42 0f 10 44 19 f8        	movss	-0x8(%rcx,%r11), %xmm0
1409a51e8: f3 41 0f 5c 40 04           	subss	0x4(%r8), %xmm0
1409a51ee: f3 41 0f 5e 40 10           	divss	0x10(%r8), %xmm0
1409a51f4: f3 41 0f 10 08              	movss	(%r8), %xmm1
1409a51f9: f3 41 0f 10 50 08           	movss	0x8(%r8), %xmm2
1409a51ff: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
1409a5202: f3 41 0f 10 50 0c           	movss	0xc(%r8), %xmm2
1409a5208: f3 41 0f 10 58 14           	movss	0x14(%r8), %xmm3
1409a520e: 0f 14 d3                    	unpcklps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0],xmm2[1],xmm3[1]
1409a5211: f3 0f 12 f0                 	movsldup	%xmm0, %xmm6            # xmm6 = xmm0[0,0,2,2]
1409a5215: 0f 59 f2                    	mulps	%xmm2, %xmm6
1409a5218: 0f 58 f1                    	addps	%xmm1, %xmm6
1409a521b: 4a 8b 44 19 e0              	movq	-0x20(%rcx,%r11), %rax
1409a5220: f3 0f 10 08                 	movss	(%rax), %xmm1
1409a5224: f3 0f 10 50 08              	movss	0x8(%rax), %xmm2
1409a5229: 48 8d 0c 52                 	leaq	(%rdx,%rdx,2), %rcx
1409a522d: 48 8d 0c 88                 	leaq	(%rax,%rcx,4), %rcx
1409a5231: 0f 28 c1                    	movaps	%xmm1, %xmm0
1409a5234: 0f 14 c2                    	unpcklps	%xmm2, %xmm0            # xmm0 = xmm0[0],xmm2[0],xmm0[1],xmm2[1]
1409a5237: 48 89 c2                    	movq	%rax, %rdx
1409a523a: 0f 28 d8                    	movaps	%xmm0, %xmm3
1409a523d: 0f 1f 00                    	nopl	(%rax)
1409a5240: f3 0f 10 22                 	movss	(%rdx), %xmm4
1409a5244: f3 0f 10 6a 08              	movss	0x8(%rdx), %xmm5
1409a5249: 0f 14 e5                    	unpcklps	%xmm5, %xmm4            # xmm4 = xmm4[0],xmm5[0],xmm4[1],xmm5[1]
1409a524c: 0f 28 ec                    	movaps	%xmm4, %xmm5
1409a524f: 0f 5d e8                    	minps	%xmm0, %xmm5
1409a5252: 0f c2 c0 03                 	cmpunordps	%xmm0, %xmm0
1409a5256: 0f 28 fc                    	movaps	%xmm4, %xmm7
1409a5259: 0f 54 f8                    	andps	%xmm0, %xmm7
1409a525c: 0f 55 c5                    	andnps	%xmm5, %xmm0
1409a525f: 0f 56 c7                    	orps	%xmm7, %xmm0
1409a5262: 0f 28 ec                    	movaps	%xmm4, %xmm5
1409a5265: 0f 5f e3                    	maxps	%xmm3, %xmm4
1409a5268: 0f c2 db 03                 	cmpunordps	%xmm3, %xmm3
1409a526c: 0f 54 eb                    	andps	%xmm3, %xmm5
1409a526f: 0f 55 dc                    	andnps	%xmm4, %xmm3
1409a5272: 0f 56 dd                    	orps	%xmm5, %xmm3
1409a5275: 48 83 c2 0c                 	addq	$0xc, %rdx
1409a5279: 48 39 ca                    	cmpq	%rcx, %rdx
1409a527c: 75 c2                       	jne	0x1409a5240 <.text+0x9a4240>
1409a527e: 4c 89 ad e8 00 00 00        	movq	%r13, 0xe8(%rbp)
1409a5285: 0f 58 c3                    	addps	%xmm3, %xmm0
1409a5288: 0f 59 05 51 06 f8 01        	mulps	0x1f80651(%rip), %xmm0  # 0x1429258e0
1409a528f: f3 0f 11 8d 78 01 00 00     	movss	%xmm1, 0x178(%rbp)
1409a5297: f3 0f 11 95 7c 01 00 00     	movss	%xmm2, 0x17c(%rbp)
1409a529f: f3 0f 10 0d 21 9f 03 02     	movss	0x2039f21(%rip), %xmm1  # 0x1429df1c8
1409a52a7: eb 10                       	jmp	0x1409a52b9 <.text+0x9a42b9>
1409a52a9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1409a52b0: 48 83 c0 0c                 	addq	$0xc, %rax
1409a52b4: 48 39 c8                    	cmpq	%rcx, %rax
1409a52b7: 74 3e                       	je	0x1409a52f7 <.text+0x9a42f7>
1409a52b9: f3 0f 10 10                 	movss	(%rax), %xmm2
1409a52bd: f3 0f 10 58 08              	movss	0x8(%rax), %xmm3
1409a52c2: 0f 28 e2                    	movaps	%xmm2, %xmm4
1409a52c5: 0f 14 e3                    	unpcklps	%xmm3, %xmm4            # xmm4 = xmm4[0],xmm3[0],xmm4[1],xmm3[1]
1409a52c8: 0f 5c e0                    	subps	%xmm0, %xmm4
1409a52cb: 0f 59 e4                    	mulps	%xmm4, %xmm4
1409a52ce: f3 0f 16 ec                 	movshdup	%xmm4, %xmm5            # xmm5 = xmm4[1,1,3,3]
1409a52d2: f3 0f 58 ec                 	addss	%xmm4, %xmm5
1409a52d6: 0f 57 e4                    	xorps	%xmm4, %xmm4
1409a52d9: f3 0f 51 e5                 	sqrtss	%xmm5, %xmm4
1409a52dd: 0f 2e cc                    	ucomiss	%xmm4, %xmm1
1409a52e0: 76 ce                       	jbe	0x1409a52b0 <.text+0x9a42b0>
1409a52e2: f3 0f 11 95 78 01 00 00     	movss	%xmm2, 0x178(%rbp)
1409a52ea: f3 0f 11 9d 7c 01 00 00     	movss	%xmm3, 0x17c(%rbp)
1409a52f2: 0f 28 cc                    	movaps	%xmm4, %xmm1
1409a52f5: eb b9                       	jmp	0x1409a52b0 <.text+0x9a42b0>
1409a52f7: 4c 89 85 e0 00 00 00        	movq	%r8, 0xe0(%rbp)
1409a52fe: f3 41 0f 10 57 f8           	movss	-0x8(%r15), %xmm2
1409a5304: 48 8d 8d 18 03 00 00        	leaq	0x318(%rbp), %rcx
1409a530b: 48 8d 95 78 01 00 00        	leaq	0x178(%rbp), %rdx
1409a5312: e8 c9 7a 2f 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409a5317: 0f 28 05 a2 8d f9 01        	movaps	0x1f98da2(%rip), %xmm0  # 0x14293e0c0
1409a531e: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409a5325: 48 8b 07                    	movq	(%rdi), %rax
1409a5328: 83 78 10 03                 	cmpl	$0x3, 0x10(%rax)
1409a532c: 41 0f 93 c6                 	setae	%r14b
1409a5330: 4c 89 bd f8 02 00 00        	movq	%r15, 0x2f8(%rbp)
1409a5337: f3 41 0f 10 5f f0           	movss	-0x10(%r15), %xmm3
1409a533d: f3 44 0f 10 0d be 05 f8 01  	movss	0x1f805be(%rip), %xmm9  # 0x142925904
1409a5346: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1409a534a: f3 0f 5c c3                 	subss	%xmm3, %xmm0
1409a534e: f3 0f 59 05 0e 7d 02 02     	mulss	0x2027d0e(%rip), %xmm0  # 0x1429cd064
1409a5356: f3 0f 59 1d ea 79 0d 02     	mulss	0x20d79ea(%rip), %xmm3  # 0x142a7cd48
1409a535e: f3 0f 58 d8                 	addss	%xmm0, %xmm3
1409a5362: f3 0f 11 9d 20 01 00 00     	movss	%xmm3, 0x120(%rbp)
1409a536a: f3 0f 10 8d 18 03 00 00     	movss	0x318(%rbp), %xmm1
1409a5372: f3 0f 10 95 20 03 00 00     	movss	0x320(%rbp), %xmm2
1409a537a: 48 8d bd c0 01 00 00        	leaq	0x1c0(%rbp), %rdi
1409a5381: 48 89 f9                    	movq	%rdi, %rcx
1409a5384: e8 b7 21 2f 00              	callq	0x140c97540 <.text+0xc96540>
1409a5389: f3 44 0f 10 85 1c 03 00 00  	movss	0x31c(%rbp), %xmm8
1409a5392: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a5399: f3 0f 10 3d cf 04 f8 01     	movss	0x1f804cf(%rip), %xmm7  # 0x142925870
1409a53a1: 48 89 fa                    	movq	%rdi, %rdx
1409a53a4: 0f 28 d7                    	movaps	%xmm7, %xmm2
1409a53a7: 41 0f 28 d8                 	movaps	%xmm8, %xmm3
1409a53ab: e8 60 2a 2f 00              	callq	0x140c97e10 <.text+0xc96e10>
1409a53b0: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1409a53b7: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1409a53be: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
1409a53c5: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
1409a53cc: c7 85 44 01 00 00 00 00 00 3f       	movl	$0x3f000000, 0x144(%rbp) # imm = 0x3F000000
1409a53d6: c6 85 40 01 00 00 01        	movb	$0x1, 0x140(%rbp)
1409a53dd: 48 8b 95 58 02 00 00        	movq	0x258(%rbp), %rdx
1409a53e4: 48 85 d2                    	testq	%rdx, %rdx
1409a53e7: 74 16                       	je	0x1409a53ff <.text+0x9a43ff>
1409a53e9: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
1409a53f0: 48 c1 e2 02                 	shlq	$0x2, %rdx
1409a53f4: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409a53fa: e8 21 e8 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a53ff: 48 8b 06                    	movq	(%rsi), %rax
1409a5402: 48 89 85 28 03 00 00        	movq	%rax, 0x328(%rbp)
1409a5409: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
1409a5410: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1409a5417: 48 8d 05 72 92 00 00        	leaq	0x9272(%rip), %rax      # 0x1409ae690 <.text+0x9ad690>
1409a541e: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1409a5425: 48 8d 05 5c 7e 11 02        	leaq	0x2117e5c(%rip), %rax   # 0x142abd288
1409a542c: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a5433: 48 c7 85 48 02 00 00 01 00 00 00    	movq	$0x1, 0x248(%rbp)
1409a543e: 48 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %rax
1409a5445: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1409a544c: 48 c7 85 58 02 00 00 01 00 00 00    	movq	$0x1, 0x258(%rbp)
1409a5457: 48 c7 85 60 02 00 00 00 00 00 00    	movq	$0x0, 0x260(%rbp)
1409a5462: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
1409a5469: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
1409a546d: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1409a5474: e8 17 4b 3d 00              	callq	0x140d79f90 <.text+0xd78f90>
1409a5479: 31 c9                       	xorl	%ecx, %ecx
1409a547b: 44 88 f1                    	movb	%r14b, %cl
1409a547e: 44 8d 04 09                 	leal	(%rcx,%rcx), %r8d
1409a5482: 8d 14 49                    	leal	(%rcx,%rcx,2), %edx
1409a5485: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
1409a5489: 0f 29 85 f0 01 00 00        	movaps	%xmm0, 0x1f0(%rbp)
1409a5490: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1409a5494: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1409a549b: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1409a54a2: 0f 10 8d 50 01 00 00        	movups	0x150(%rbp), %xmm1
1409a54a9: 0f 11 8d d8 02 00 00        	movups	%xmm1, 0x2d8(%rbp)
1409a54b0: 0f 11 85 c8 02 00 00        	movups	%xmm0, 0x2c8(%rbp)
1409a54b7: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
1409a54c2: 48 c7 85 48 02 00 00 08 00 00 00    	movq	$0x8, 0x248(%rbp)
1409a54cd: 48 c7 85 50 02 00 00 00 00 00 00    	movq	$0x0, 0x250(%rbp)
1409a54d8: 48 89 8d b0 01 00 00        	movq	%rcx, 0x1b0(%rbp)
1409a54df: 89 8d 58 02 00 00           	movl	%ecx, 0x258(%rbp)
1409a54e5: 48 c7 85 60 02 00 00 02 00 00 00    	movq	$0x2, 0x260(%rbp)
1409a54f0: 44 89 85 8c 01 00 00        	movl	%r8d, 0x18c(%rbp)
1409a54f7: 44 89 85 70 02 00 00        	movl	%r8d, 0x270(%rbp)
1409a54fe: 89 95 88 02 00 00           	movl	%edx, 0x288(%rbp)
1409a5504: 89 95 b8 01 00 00           	movl	%edx, 0x1b8(%rbp)
1409a550a: 89 95 a0 02 00 00           	movl	%edx, 0x2a0(%rbp)
1409a5510: c6 85 b8 02 00 00 00        	movb	$0x0, 0x2b8(%rbp)
1409a5517: 66 c7 85 e8 02 00 00 00 01  	movw	$0x100, 0x2e8(%rbp)     # imm = 0x100
1409a5520: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a5527: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1409a552e: 48 8d bd 40 02 00 00        	leaq	0x240(%rbp), %rdi
1409a5535: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
1409a553c: 49 89 f8                    	movq	%rdi, %r8
1409a553f: e8 ac dc f7 ff              	callq	0x1409231f0 <.text+0x9221f0>
1409a5544: 48 89 c6                    	movq	%rax, %rsi
1409a5547: 49 89 d5                    	movq	%rdx, %r13
1409a554a: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
1409a554e: 74 23                       	je	0x1409a5573 <.text+0x9a4573>
1409a5550: 4c 39 6e 38                 	cmpq	%r13, 0x38(%rsi)
1409a5554: 75 1d                       	jne	0x1409a5573 <.text+0x9a4573>
1409a5556: 83 7e 28 02                 	cmpl	$0x2, 0x28(%rsi)
1409a555a: 0f 95 c0                    	setne	%al
1409a555d: 89 85 14 03 00 00           	movl	%eax, 0x314(%rbp)
1409a5563: 0f 85 35 09 00 00           	jne	0x1409a5e9e <.text+0x9a4e9e>
1409a5569: f3 0f 10 05 7f 91 f8 01     	movss	0x1f8917f(%rip), %xmm0  # 0x14292e6f0
1409a5571: eb 12                       	jmp	0x1409a5585 <.text+0x9a4585>
1409a5573: f3 0f 10 05 75 91 f8 01     	movss	0x1f89175(%rip), %xmm0  # 0x14292e6f0
1409a557b: c7 85 14 03 00 00 00 00 00 00       	movl	$0x0, 0x314(%rbp)
1409a5585: f3 0f 11 85 b4 00 00 00     	movss	%xmm0, 0xb4(%rbp)
1409a558d: 48 8b 03                    	movq	(%rbx), %rax
1409a5590: 48 89 85 a8 01 00 00        	movq	%rax, 0x1a8(%rbp)
1409a5597: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
1409a559e: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a55a5: 48 8d 05 e4 90 00 00        	leaq	0x90e4(%rip), %rax      # 0x1409ae690 <.text+0x9ad690>
1409a55ac: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1409a55b3: 48 8d 85 b4 00 00 00        	leaq	0xb4(%rbp), %rax
1409a55ba: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1409a55c1: 48 8d 05 c8 72 3e 00        	leaq	0x3e72c8(%rip), %rax    # 0x140d8c890 <.text+0xd8b890>
1409a55c8: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1409a55cf: 48 8d 85 18 03 00 00        	leaq	0x318(%rbp), %rax
1409a55d6: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1409a55dd: 48 8d 05 ec 61 3a 00        	leaq	0x3a61ec(%rip), %rax    # 0x140d4b7d0 <.text+0xd4a7d0>
1409a55e4: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1409a55eb: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
1409a55f2: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1409a55f9: 48 8d 05 10 73 3e 00        	leaq	0x3e7310(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
1409a5600: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
1409a5607: 48 8d 05 9a 7c 11 02        	leaq	0x2117c9a(%rip), %rax   # 0x142abd2a8
1409a560e: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1409a5615: 48 c7 85 f8 01 00 00 04 00 00 00    	movq	$0x4, 0x1f8(%rbp)
1409a5620: 48 89 bd 00 02 00 00        	movq	%rdi, 0x200(%rbp)
1409a5627: 48 c7 85 08 02 00 00 04 00 00 00    	movq	$0x4, 0x208(%rbp)
1409a5632: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
1409a563d: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a5644: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1409a5648: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1409a564f: e8 3c 49 3d 00              	callq	0x140d79f90 <.text+0xd78f90>
1409a5654: 4c 8b 7d 48                 	movq	0x48(%rbp), %r15
1409a5658: 48 8b 45 50                 	movq	0x50(%rbp), %rax
1409a565c: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
1409a5663: 48 8b 7d 58                 	movq	0x58(%rbp), %rdi
1409a5667: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1409a566e: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
1409a5675: f3 44 0f 10 95 b4 00 00 00  	movss	0xb4(%rbp), %xmm10
1409a567e: 0f b6 05 84 f5 c8 02        	movzbl	0x2c8f584(%rip), %eax   # 0x143634c09
1409a5685: b9 28 00 00 00              	movl	$0x28, %ecx
1409a568a: ba 08 00 00 00              	movl	$0x8, %edx
1409a568f: e8 7c e5 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a5694: 48 85 c0                    	testq	%rax, %rax
1409a5697: 4c 89 bd 38 02 00 00        	movq	%r15, 0x238(%rbp)
1409a569e: 0f 84 da 1e 00 00           	je	0x1409a757e <.text+0x9a657e>
1409a56a4: 49 89 c6                    	movq	%rax, %r14
1409a56a7: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
1409a56ae: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
1409a56b6: 0f 28 85 40 02 00 00        	movaps	0x240(%rbp), %xmm0
1409a56bd: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1409a56c1: f3 44 0f 11 40 20           	movss	%xmm8, 0x20(%rax)
1409a56c7: f3 44 0f 11 50 24           	movss	%xmm10, 0x24(%rax)
1409a56cd: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a56d4: 4c 8d 25 0d 7c 11 02        	leaq	0x2117c0d(%rip), %r12   # 0x142abd2e8
1409a56db: 4c 89 a5 48 02 00 00        	movq	%r12, 0x248(%rbp)
1409a56e2: 48 85 ff                    	testq	%rdi, %rdi
1409a56e5: 0f 88 a7 1e 00 00           	js	0x1409a7592 <.text+0x9a6592>
1409a56eb: 74 36                       	je	0x1409a5723 <.text+0x9a4723>
1409a56ed: 4c 89 b5 e8 01 00 00        	movq	%r14, 0x1e8(%rbp)
1409a56f4: 0f b6 05 0e f5 c8 02        	movzbl	0x2c8f50e(%rip), %eax   # 0x143634c09
1409a56fb: 41 be 01 00 00 00           	movl	$0x1, %r14d
1409a5701: ba 01 00 00 00              	movl	$0x1, %edx
1409a5706: 48 89 f9                    	movq	%rdi, %rcx
1409a5709: e8 02 e5 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a570e: 48 85 c0                    	testq	%rax, %rax
1409a5711: 0f 84 85 1e 00 00           	je	0x1409a759c <.text+0x9a659c>
1409a5717: 48 89 c3                    	movq	%rax, %rbx
1409a571a: 4c 8b b5 e8 01 00 00        	movq	0x1e8(%rbp), %r14
1409a5721: eb 05                       	jmp	0x1409a5728 <.text+0x9a4728>
1409a5723: bb 01 00 00 00              	movl	$0x1, %ebx
1409a5728: 48 89 d9                    	movq	%rbx, %rcx
1409a572b: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
1409a5732: 49 89 f8                    	movq	%rdi, %r8
1409a5735: e8 c6 e4 f7 01              	callq	0x142923c00 <.text+0x2922c00>
1409a573a: 48 89 bd 40 02 00 00        	movq	%rdi, 0x240(%rbp)
1409a5741: 48 89 9d 48 02 00 00        	movq	%rbx, 0x248(%rbp)
1409a5748: 48 89 bd 50 02 00 00        	movq	%rdi, 0x250(%rbp)
1409a574f: 4c 89 b5 58 02 00 00        	movq	%r14, 0x258(%rbp)
1409a5756: 4c 89 a5 60 02 00 00        	movq	%r12, 0x260(%rbp)
1409a575d: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1409a5764: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
1409a576b: e8 10 71 ac 00              	callq	0x14146c880 <.text+0x146b880>
1409a5770: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1409a5777: 48 85 d2                    	testq	%rdx, %rdx
1409a577a: 74 12                       	je	0x1409a578e <.text+0x9a478e>
1409a577c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409a5782: 48 8b 8d 00 03 00 00        	movq	0x300(%rbp), %rcx
1409a5789: e8 92 e4 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a578e: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a5795: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1409a579c: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1409a57a3: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1409a57a7: e8 34 76 2f 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409a57ac: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1409a57b3: 4c 8d 60 30                 	leaq	0x30(%rax), %r12
1409a57b7: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a57be: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a57c5: 4c 89 e2                    	movq	%r12, %rdx
1409a57c8: e8 f3 75 2f 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1409a57cd: f3 44 0f 10 a5 44 02 00 00  	movss	0x244(%rbp), %xmm12
1409a57d6: f3 44 0f 59 25 85 78 02 02  	mulss	0x2027885(%rip), %xmm12 # 0x1429cd064
1409a57df: f3 44 0f 10 9d 48 02 00 00  	movss	0x248(%rbp), %xmm11
1409a57e8: f3 44 0f 58 a5 f4 01 00 00  	addss	0x1f4(%rbp), %xmm12
1409a57f1: f3 44 0f 10 95 40 02 00 00  	movss	0x240(%rbp), %xmm10
1409a57fa: f3 44 0f 10 ad f0 01 00 00  	movss	0x1f0(%rbp), %xmm13
1409a5803: f3 44 0f 10 b5 f8 01 00 00  	movss	0x1f8(%rbp), %xmm14
1409a580c: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a5813: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1409a581a: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1409a5821: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1409a5825: e8 b6 75 2f 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409a582a: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a5831: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a5838: 4c 89 e2                    	movq	%r12, %rdx
1409a583b: e8 80 75 2f 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1409a5840: 45 0f 14 d3                 	unpcklps	%xmm11, %xmm10          # xmm10 = xmm10[0],xmm11[0],xmm10[1],xmm11[1]
1409a5844: 44 0f 59 15 b4 72 11 02     	mulps	0x21172b4(%rip), %xmm10 # 0x142abcb00
1409a584c: 45 0f 14 ee                 	unpcklps	%xmm14, %xmm13          # xmm13 = xmm13[0],xmm14[0],xmm13[1],xmm14[1]
1409a5850: f3 0f 10 85 48 02 00 00     	movss	0x248(%rbp), %xmm0
1409a5858: f3 44 0f 10 9d 40 02 00 00  	movss	0x240(%rbp), %xmm11
1409a5861: 44 0f 14 d8                 	unpcklps	%xmm0, %xmm11           # xmm11 = xmm11[0],xmm0[0],xmm11[1],xmm0[1]
1409a5865: 44 0f 59 1d a3 72 11 02     	mulps	0x21172a3(%rip), %xmm11 # 0x142abcb10
1409a586d: f3 0f 10 85 f0 01 00 00     	movss	0x1f0(%rbp), %xmm0
1409a5875: f3 0f 10 8d f8 01 00 00     	movss	0x1f8(%rbp), %xmm1
1409a587d: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1409a5880: 0f b6 46 18                 	movzbl	0x18(%rsi), %eax
1409a5884: b1 02                       	movb	$0x2, %cl
1409a5886: 3c 02                       	cmpb	$0x2, %al
1409a5888: 72 0a                       	jb	0x1409a5894 <.text+0x9a4894>
1409a588a: 4c 39 6e 38                 	cmpq	%r13, 0x38(%rsi)
1409a588e: 75 04                       	jne	0x1409a5894 <.text+0x9a4894>
1409a5890: 0f b6 4e 19                 	movzbl	0x19(%rsi), %ecx
1409a5894: 45 0f 58 d5                 	addps	%xmm13, %xmm10
1409a5898: 44 0f 58 d8                 	addps	%xmm0, %xmm11
1409a589c: 80 c1 fe                    	addb	$-0x2, %cl
1409a589f: 0f b6 c9                    	movzbl	%cl, %ecx
1409a58a2: 80 f9 03                    	cmpb	$0x3, %cl
1409a58a5: ba 03 00 00 00              	movl	$0x3, %edx
1409a58aa: 0f 42 d1                    	cmovbl	%ecx, %edx
1409a58ad: 0f b6 ca                    	movzbl	%dl, %ecx
1409a58b0: 48 8d 15 e1 86 11 02        	leaq	0x21186e1(%rip), %rdx   # 0x142abdf98
1409a58b7: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1409a58bb: 48 01 d1                    	addq	%rdx, %rcx
1409a58be: ff e1                       	jmpq	*%rcx
1409a58c0: 84 c0                       	testb	%al, %al
1409a58c2: 0f 84 c6 02 00 00           	je	0x1409a5b8e <.text+0x9a4b8e>
1409a58c8: 4c 39 6e 38                 	cmpq	%r13, 0x38(%rsi)
1409a58cc: 0f 85 bc 02 00 00           	jne	0x1409a5b8e <.text+0x9a4b8e>
1409a58d2: 83 7e 28 02                 	cmpl	$0x2, 0x28(%rsi)
1409a58d6: 0f 84 b2 02 00 00           	je	0x1409a5b8e <.text+0x9a4b8e>
1409a58dc: f2 0f 10 85 c0 01 00 00     	movsd	0x1c0(%rbp), %xmm0
1409a58e4: f3 0f 10 4e 4c              	movss	0x4c(%rsi), %xmm1
1409a58e9: f3 0f 10 56 54              	movss	0x54(%rsi), %xmm2
1409a58ee: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
1409a58f1: 0f 5c c8                    	subps	%xmm0, %xmm1
1409a58f4: 0f 28 c1                    	movaps	%xmm1, %xmm0
1409a58f7: 0f 59 c1                    	mulps	%xmm1, %xmm0
1409a58fa: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1409a58fe: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1409a5902: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409a5905: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
1409a5909: f3 44 0f 5e c8              	divss	%xmm0, %xmm9
1409a590e: f3 41 0f 12 c1              	movsldup	%xmm9, %xmm0            # xmm0 = xmm9[0,0,2,2]
1409a5913: 0f 59 c1                    	mulps	%xmm1, %xmm0
1409a5916: 41 0f 13 04 24              	movlps	%xmm0, (%r12)
1409a591b: 80 7e 18 00                 	cmpb	$0x0, 0x18(%rsi)
1409a591f: 0f 85 e0 02 00 00           	jne	0x1409a5c05 <.text+0x9a4c05>
1409a5925: e9 e8 05 00 00              	jmp	0x1409a5f12 <.text+0x9a4f12>
1409a592a: f3 44 0f 10 ad 44 02 00 00  	movss	0x244(%rbp), %xmm13
1409a5933: f3 44 0f 59 2d 0c 74 0d 02  	mulss	0x20d740c(%rip), %xmm13 # 0x142a7cd48
1409a593c: f3 44 0f 58 ad f4 01 00 00  	addss	0x1f4(%rbp), %xmm13
1409a5945: 0f b6 05 bd f2 c8 02        	movzbl	0x2c8f2bd(%rip), %eax   # 0x143634c09
1409a594c: b9 18 00 00 00              	movl	$0x18, %ecx
1409a5951: ba 04 00 00 00              	movl	$0x4, %edx
1409a5956: e8 b5 e2 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a595b: 48 85 c0                    	testq	%rax, %rax
1409a595e: 0f 84 96 1c 00 00           	je	0x1409a75fa <.text+0x9a65fa>
1409a5964: f3 44 0f 11 10              	movss	%xmm10, (%rax)
1409a5969: f3 44 0f 11 60 04           	movss	%xmm12, 0x4(%rax)
1409a596f: f3 41 0f 16 c2              	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
1409a5974: f3 0f 11 40 08              	movss	%xmm0, 0x8(%rax)
1409a5979: f3 44 0f 11 58 0c           	movss	%xmm11, 0xc(%rax)
1409a597f: f3 44 0f 11 68 10           	movss	%xmm13, 0x10(%rax)
1409a5985: f3 41 0f 16 c3              	movshdup	%xmm11, %xmm0           # xmm0 = xmm11[1,1,3,3]
1409a598a: f3 0f 11 40 14              	movss	%xmm0, 0x14(%rax)
1409a598f: 48 c7 85 40 02 00 00 02 00 00 00    	movq	$0x2, 0x240(%rbp)
1409a599a: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1409a59a1: 48 c7 85 50 02 00 00 02 00 00 00    	movq	$0x2, 0x250(%rbp)
1409a59ac: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1409a59b6: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1409a59bd: c7 85 70 02 00 00 00 00 00 00       	movl	$0x0, 0x270(%rbp)
1409a59c7: c7 85 80 02 00 00 00 00 00 40       	movl	$0x40000000, 0x280(%rbp) # imm = 0x40000000
1409a59d1: 66 c7 85 84 02 00 00 01 00  	movw	$0x1, 0x284(%rbp)
1409a59da: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a59e1: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a59e8: 48 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %rdx
1409a59ef: e8 7c f5 15 00              	callq	0x140b04f70 <.text+0xb03f70>
1409a59f4: 45 0f 5c da                 	subps	%xmm10, %xmm11
1409a59f8: 0f 28 c6                    	movaps	%xmm6, %xmm0
1409a59fb: 41 0f 5c c2                 	subps	%xmm10, %xmm0
1409a59ff: 41 0f 59 c3                 	mulps	%xmm11, %xmm0
1409a5a03: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1409a5a07: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1409a5a0b: 45 0f 59 db                 	mulps	%xmm11, %xmm11
1409a5a0f: f3 41 0f 16 c3              	movshdup	%xmm11, %xmm0           # xmm0 = xmm11[1,1,3,3]
1409a5a14: f3 41 0f 58 c3              	addss	%xmm11, %xmm0
1409a5a19: f3 0f 5f 05 7f 60 0d 02     	maxss	0x20d607f(%rip), %xmm0  # 0x142a7baa0
1409a5a21: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1409a5a25: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1409a5a2c: f3 0f 5c 48 28              	subss	0x28(%rax), %xmm1
1409a5a31: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409a5a34: f3 0f 5f c1                 	maxss	%xmm1, %xmm0
1409a5a38: f3 44 0f 5d c8              	minss	%xmm0, %xmm9
1409a5a3d: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1409a5a44: 44 0f 2e 48 f0              	ucomiss	-0x10(%rax), %xmm9
1409a5a49: 75 06                       	jne	0x1409a5a51 <.text+0x9a4a51>
1409a5a4b: 0f 8b 3d 01 00 00           	jnp	0x1409a5b8e <.text+0x9a4b8e>
1409a5a51: 8b 85 20 03 00 00           	movl	0x320(%rbp), %eax
1409a5a57: 89 85 4c 02 00 00           	movl	%eax, 0x24c(%rbp)
1409a5a5d: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
1409a5a64: 48 89 85 44 02 00 00        	movq	%rax, 0x244(%rbp)
1409a5a6b: 44 8b b5 30 03 00 00        	movl	0x330(%rbp), %r14d
1409a5a72: 48 8b 95 78 04 00 00        	movq	0x478(%rbp), %rdx
1409a5a79: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409a5a7c: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409a5a80: 89 01                       	movl	%eax, (%rcx)
1409a5a82: 48 8b 3a                    	movq	(%rdx), %rdi
1409a5a85: 48 8b 5f 30                 	movq	0x30(%rdi), %rbx
1409a5a89: 4c 8b 7f 40                 	movq	0x40(%rdi), %r15
1409a5a8d: 48 3b 5f 20                 	cmpq	0x20(%rdi), %rbx
1409a5a91: 75 17                       	jne	0x1409a5aaa <.text+0x9a4aaa>
1409a5a93: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
1409a5a97: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a5a9e: 48 8d 15 a3 46 29 02        	leaq	0x22946a3(%rip), %rdx   # 0x142c3a148
1409a5aa5: e8 06 d0 bb ff              	callq	0x140562ab0 <.text+0x561ab0>
1409a5aaa: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1409a5aae: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
1409a5ab2: c7 04 c8 01 00 00 00        	movl	$0x1, (%rax,%rcx,8)
1409a5ab9: f3 44 0f 11 4c c8 04        	movss	%xmm9, 0x4(%rax,%rcx,8)
1409a5ac0: c7 44 c8 08 00 00 00 00     	movl	$0x0, 0x8(%rax,%rcx,8)
1409a5ac8: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
1409a5acf: 0f 11 44 c8 0c              	movups	%xmm0, 0xc(%rax,%rcx,8)
1409a5ad4: 44 89 74 c8 1c              	movl	%r14d, 0x1c(%rax,%rcx,8)
1409a5ad9: 4c 89 7c c8 20              	movq	%r15, 0x20(%rax,%rcx,8)
1409a5ade: 48 ff c3                    	incq	%rbx
1409a5ae1: 48 89 5f 30                 	movq	%rbx, 0x30(%rdi)
1409a5ae5: 48 ff 47 40                 	incq	0x40(%rdi)
1409a5ae9: 80 7e 18 00                 	cmpb	$0x0, 0x18(%rsi)
1409a5aed: 0f 85 12 01 00 00           	jne	0x1409a5c05 <.text+0x9a4c05>
1409a5af3: e9 1a 04 00 00              	jmp	0x1409a5f12 <.text+0x9a4f12>
1409a5af8: 48 8b 95 50 04 00 00        	movq	0x450(%rbp), %rdx
1409a5aff: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409a5b02: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409a5b06: 89 01                       	movl	%eax, (%rcx)
1409a5b08: 48 8b 02                    	movq	(%rdx), %rax
1409a5b0b: 48 8b 8d 40 04 00 00        	movq	0x440(%rbp), %rcx
1409a5b12: f3 0f 10 41 38              	movss	0x38(%rcx), %xmm0
1409a5b17: f3 0f 10 49 3c              	movss	0x3c(%rcx), %xmm1
1409a5b1c: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1409a5b23: f3 0f 10 51 f0              	movss	-0x10(%rcx), %xmm2
1409a5b28: f3 0f 10 59 f8              	movss	-0x8(%rcx), %xmm3
1409a5b2d: 8b 8d 30 03 00 00           	movl	0x330(%rbp), %ecx
1409a5b33: 48 8b 50 20                 	movq	0x20(%rax), %rdx
1409a5b37: 48 89 95 60 02 00 00        	movq	%rdx, 0x260(%rbp)
1409a5b3e: 0f 10 20                    	movups	(%rax), %xmm4
1409a5b41: 0f 10 68 10                 	movups	0x10(%rax), %xmm5
1409a5b45: 0f 29 ad 50 02 00 00        	movaps	%xmm5, 0x250(%rbp)
1409a5b4c: 0f 29 a5 40 02 00 00        	movaps	%xmm4, 0x240(%rbp)
1409a5b53: 48 ba 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rdx # imm = 0x8000000000000001
1409a5b5d: 48 89 10                    	movq	%rdx, (%rax)
1409a5b60: 89 48 08                    	movl	%ecx, 0x8(%rax)
1409a5b63: f3 0f 11 40 0c              	movss	%xmm0, 0xc(%rax)
1409a5b68: f3 0f 11 58 10              	movss	%xmm3, 0x10(%rax)
1409a5b6d: f3 0f 11 48 14              	movss	%xmm1, 0x14(%rax)
1409a5b72: f3 0f 11 50 18              	movss	%xmm2, 0x18(%rax)
1409a5b77: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
1409a5b81: 48 39 85 40 02 00 00        	cmpq	%rax, 0x240(%rbp)
1409a5b88: 0f 85 87 1a 00 00           	jne	0x1409a7615 <.text+0x9a6615>
1409a5b8e: 80 7e 18 00                 	cmpb	$0x0, 0x18(%rsi)
1409a5b92: 75 71                       	jne	0x1409a5c05 <.text+0x9a4c05>
1409a5b94: e9 79 03 00 00              	jmp	0x1409a5f12 <.text+0x9a4f12>
1409a5b99: 0f 28 c6                    	movaps	%xmm6, %xmm0
1409a5b9c: 41 0f 5c c2                 	subps	%xmm10, %xmm0
1409a5ba0: 45 0f 5c da                 	subps	%xmm10, %xmm11
1409a5ba4: f3 41 0f 16 cb              	movshdup	%xmm11, %xmm1           # xmm1 = xmm11[1,1,3,3]
1409a5ba9: 41 0f 59 c3                 	mulps	%xmm11, %xmm0
1409a5bad: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1409a5bb1: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1409a5bb5: f3 45 0f 59 db              	mulss	%xmm11, %xmm11
1409a5bba: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1409a5bbe: f3 41 0f 58 cb              	addss	%xmm11, %xmm1
1409a5bc3: f3 0f 5f 0d d5 5e 0d 02     	maxss	0x20d5ed5(%rip), %xmm1  # 0x142a7baa0
1409a5bcb: f3 0f 5e d1                 	divss	%xmm1, %xmm2
1409a5bcf: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1409a5bd6: f3 0f 5c 51 f0              	subss	-0x10(%rcx), %xmm2
1409a5bdb: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1409a5be2: f3 0f 11 50 28              	movss	%xmm2, 0x28(%rax)
1409a5be7: f3 0f 10 41 f0              	movss	-0x10(%rcx), %xmm0
1409a5bec: f3 0f 11 40 3c              	movss	%xmm0, 0x3c(%rax)
1409a5bf1: f3 0f 10 41 f8              	movss	-0x8(%rcx), %xmm0
1409a5bf6: f3 0f 11 40 38              	movss	%xmm0, 0x38(%rax)
1409a5bfb: 80 7e 18 00                 	cmpb	$0x0, 0x18(%rsi)
1409a5bff: 0f 84 0d 03 00 00           	je	0x1409a5f12 <.text+0x9a4f12>
1409a5c05: 4c 39 6e 38                 	cmpq	%r13, 0x38(%rsi)
1409a5c09: 0f 85 03 03 00 00           	jne	0x1409a5f12 <.text+0x9a4f12>
1409a5c0f: 83 7e 28 02                 	cmpl	$0x2, 0x28(%rsi)
1409a5c13: 0f 84 f9 02 00 00           	je	0x1409a5f12 <.text+0x9a4f12>
1409a5c19: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1409a5c20: f3 0f 10 48 30              	movss	0x30(%rax), %xmm1
1409a5c25: f3 0f 10 50 34              	movss	0x34(%rax), %xmm2
1409a5c2a: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a5c31: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a5c38: e8 93 b3 2e 00              	callq	0x140c90fd0 <.text+0xc8ffd0>
1409a5c3d: 44 0f 28 8d 40 02 00 00     	movaps	0x240(%rbp), %xmm9
1409a5c45: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1409a5c4c: f2 0f 10 8d c0 01 00 00     	movsd	0x1c0(%rbp), %xmm1
1409a5c54: f2 41 0f 10 14 24           	movsd	(%r12), %xmm2
1409a5c5a: 0f 15 c0                    	unpckhps	%xmm0, %xmm0            # xmm0 = xmm0[2,2,3,3]
1409a5c5d: 0f 59 c2                    	mulps	%xmm2, %xmm0
1409a5c60: 0f 58 c1                    	addps	%xmm1, %xmm0
1409a5c63: 0f 13 85 f0 01 00 00        	movlps	%xmm0, 0x1f0(%rbp)
1409a5c6a: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a5c71: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a5c78: 48 8d b5 f0 01 00 00        	leaq	0x1f0(%rbp), %rsi
1409a5c7f: 48 89 f2                    	movq	%rsi, %rdx
1409a5c82: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1409a5c86: e8 55 71 2f 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409a5c8b: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1409a5c8f: 41 0f 59 c9                 	mulps	%xmm9, %xmm1
1409a5c93: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
1409a5c97: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1409a5c9b: 0f 28 d1                    	movaps	%xmm1, %xmm2
1409a5c9e: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
1409a5ca2: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1409a5ca6: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1409a5caa: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1409a5cae: 0f 5c ca                    	subps	%xmm2, %xmm1
1409a5cb1: 0f 28 05 28 35 0e 02        	movaps	0x20e3528(%rip), %xmm0  # 0x142a891e0
1409a5cb8: 0f 59 c8                    	mulps	%xmm0, %xmm1
1409a5cbb: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
1409a5cbf: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1409a5cc3: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1409a5cc7: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1409a5cca: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1409a5cce: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1409a5cd2: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409a5cd6: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
1409a5cda: 0f 58 c1                    	addps	%xmm1, %xmm0
1409a5cdd: f3 0f 10 0d 83 73 02 02     	movss	0x2027383(%rip), %xmm1  # 0x1429cd068
1409a5ce5: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
1409a5ce9: 0f 59 d1                    	mulps	%xmm1, %xmm2
1409a5cec: 41 0f 28 d9                 	movaps	%xmm9, %xmm3
1409a5cf0: 41 0f c6 d9 c9              	shufps	$0xc9, %xmm9, %xmm3     # xmm3 = xmm3[1,2],xmm9[0,3]
1409a5cf5: 0f 59 d9                    	mulps	%xmm1, %xmm3
1409a5cf8: 0f c6 d2 d2                 	shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
1409a5cfc: 0f 5c da                    	subps	%xmm2, %xmm3
1409a5cff: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1409a5d03: 41 0f 58 c9                 	addps	%xmm9, %xmm1
1409a5d07: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1409a5d0b: 0f 59 cb                    	mulps	%xmm3, %xmm1
1409a5d0e: 0f 58 c8                    	addps	%xmm0, %xmm1
1409a5d11: f2 0f 10 85 40 02 00 00     	movsd	0x240(%rbp), %xmm0
1409a5d19: 0f 58 c1                    	addps	%xmm1, %xmm0
1409a5d1c: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1409a5d1f: f3 0f 58 8d 48 02 00 00     	addss	0x248(%rbp), %xmm1
1409a5d27: 0f 13 85 08 01 00 00        	movlps	%xmm0, 0x108(%rbp)
1409a5d2e: f3 0f 11 8d 10 01 00 00     	movss	%xmm1, 0x110(%rbp)
1409a5d36: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
1409a5d3d: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1409a5d44: 48 8d 05 45 89 00 00        	leaq	0x8945(%rip), %rax      # 0x1409ae690 <.text+0x9ad690>
1409a5d4b: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1409a5d52: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1409a5d59: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1409a5d60: 48 8d 05 79 e6 ff ff        	leaq	-0x1987(%rip), %rax     # 0x1409a43e0 <.text+0x9a33e0>
1409a5d67: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1409a5d6e: 48 8d 85 08 01 00 00        	leaq	0x108(%rbp), %rax
1409a5d75: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1409a5d7c: 48 8d 05 7d 5c 3a 00        	leaq	0x3a5c7d(%rip), %rax    # 0x140d4ba00 <.text+0xd4aa00>
1409a5d83: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1409a5d8a: 48 8d 05 17 77 11 02        	leaq	0x2117717(%rip), %rax   # 0x142abd4a8
1409a5d91: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a5d98: 48 c7 85 48 02 00 00 04 00 00 00    	movq	$0x4, 0x248(%rbp)
1409a5da3: 48 89 b5 50 02 00 00        	movq	%rsi, 0x250(%rbp)
1409a5daa: 48 c7 85 58 02 00 00 03 00 00 00    	movq	$0x3, 0x258(%rbp)
1409a5db5: 48 c7 85 60 02 00 00 00 00 00 00    	movq	$0x0, 0x260(%rbp)
1409a5dc0: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a5dc7: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
1409a5dcb: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1409a5dd2: e8 b9 41 3d 00              	callq	0x140d79f90 <.text+0xd78f90>
1409a5dd7: 48 8b 7d 30                 	movq	0x30(%rbp), %rdi
1409a5ddb: 48 8b 5d 38                 	movq	0x38(%rbp), %rbx
1409a5ddf: 48 8b 75 40                 	movq	0x40(%rbp), %rsi
1409a5de3: 8b 85 10 01 00 00           	movl	0x110(%rbp), %eax
1409a5de9: 89 85 48 02 00 00           	movl	%eax, 0x248(%rbp)
1409a5def: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
1409a5df6: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a5dfd: 0f b6 05 05 ee c8 02        	movzbl	0x2c8ee05(%rip), %eax   # 0x143634c09
1409a5e04: b9 30 00 00 00              	movl	$0x30, %ecx
1409a5e09: ba 10 00 00 00              	movl	$0x10, %edx
1409a5e0e: e8 fd dd c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a5e13: 48 85 c0                    	testq	%rax, %rax
1409a5e16: 48 89 bd 00 03 00 00        	movq	%rdi, 0x300(%rbp)
1409a5e1d: 48 89 9d 38 02 00 00        	movq	%rbx, 0x238(%rbp)
1409a5e24: 0f 84 83 18 00 00           	je	0x1409a76ad <.text+0x9a66ad>
1409a5e2a: 49 89 c7                    	movq	%rax, %r15
1409a5e2d: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
1409a5e34: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
1409a5e3c: 44 0f 29 48 10              	movaps	%xmm9, 0x10(%rax)
1409a5e41: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
1409a5e48: 49 89 47 20                 	movq	%rax, 0x20(%r15)
1409a5e4c: 8b 85 48 02 00 00           	movl	0x248(%rbp), %eax
1409a5e52: 41 89 47 28                 	movl	%eax, 0x28(%r15)
1409a5e56: 4c 89 bd 40 02 00 00        	movq	%r15, 0x240(%rbp)
1409a5e5d: 4c 8d 25 84 76 11 02        	leaq	0x2117684(%rip), %r12   # 0x142abd4e8
1409a5e64: 4c 89 a5 48 02 00 00        	movq	%r12, 0x248(%rbp)
1409a5e6b: 48 85 f6                    	testq	%rsi, %rsi
1409a5e6e: 0f 88 4a 18 00 00           	js	0x1409a76be <.text+0x9a66be>
1409a5e74: 74 35                       	je	0x1409a5eab <.text+0x9a4eab>
1409a5e76: 0f b6 05 8c ed c8 02        	movzbl	0x2c8ed8c(%rip), %eax   # 0x143634c09
1409a5e7d: 41 be 01 00 00 00           	movl	$0x1, %r14d
1409a5e83: ba 01 00 00 00              	movl	$0x1, %edx
1409a5e88: 48 89 f1                    	movq	%rsi, %rcx
1409a5e8b: e8 80 dd c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a5e90: 48 85 c0                    	testq	%rax, %rax
1409a5e93: 0f 84 28 18 00 00           	je	0x1409a76c1 <.text+0x9a66c1>
1409a5e99: 48 89 c7                    	movq	%rax, %rdi
1409a5e9c: eb 12                       	jmp	0x1409a5eb0 <.text+0x9a4eb0>
1409a5e9e: f3 0f 10 05 e2 fa f7 01     	movss	0x1f7fae2(%rip), %xmm0  # 0x142925988
1409a5ea6: e9 da f6 ff ff              	jmp	0x1409a5585 <.text+0x9a4585>
1409a5eab: bf 01 00 00 00              	movl	$0x1, %edi
1409a5eb0: 48 89 f9                    	movq	%rdi, %rcx
1409a5eb3: 48 89 da                    	movq	%rbx, %rdx
1409a5eb6: 49 89 f0                    	movq	%rsi, %r8
1409a5eb9: e8 42 dd f7 01              	callq	0x142923c00 <.text+0x2922c00>
1409a5ebe: 48 89 b5 40 02 00 00        	movq	%rsi, 0x240(%rbp)
1409a5ec5: 48 89 bd 48 02 00 00        	movq	%rdi, 0x248(%rbp)
1409a5ecc: 48 89 b5 50 02 00 00        	movq	%rsi, 0x250(%rbp)
1409a5ed3: 4c 89 bd 58 02 00 00        	movq	%r15, 0x258(%rbp)
1409a5eda: 4c 89 a5 60 02 00 00        	movq	%r12, 0x260(%rbp)
1409a5ee1: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1409a5ee8: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
1409a5eef: e8 8c 69 ac 00              	callq	0x14146c880 <.text+0x146b880>
1409a5ef4: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
1409a5efb: 48 85 d2                    	testq	%rdx, %rdx
1409a5efe: 74 12                       	je	0x1409a5f12 <.text+0x9a4f12>
1409a5f00: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409a5f06: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
1409a5f0d: e8 0e dd c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a5f12: f2 44 0f 10 8d 18 03 00 00  	movsd	0x318(%rbp), %xmm9
1409a5f1b: 44 0f 58 0d 8d e6 f8 01     	addps	0x1f8e68d(%rip), %xmm9  # 0x1429345b0
1409a5f23: f3 41 0f 16 c1              	movshdup	%xmm9, %xmm0            # xmm0 = xmm9[1,1,3,3]
1409a5f28: 45 0f 57 d2                 	xorps	%xmm10, %xmm10
1409a5f2c: f3 44 0f 58 95 20 03 00 00  	addss	0x320(%rbp), %xmm10
1409a5f35: f3 0f 58 c7                 	addss	%xmm7, %xmm0
1409a5f39: f3 44 0f 11 8d 50 02 00 00  	movss	%xmm9, 0x250(%rbp)
1409a5f42: f3 0f 11 85 54 02 00 00     	movss	%xmm0, 0x254(%rbp)
1409a5f4a: f3 44 0f 11 95 58 02 00 00  	movss	%xmm10, 0x258(%rbp)
1409a5f53: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
1409a5f5a: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
1409a5f61: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
1409a5f6b: 48 89 85 5c 02 00 00        	movq	%rax, 0x25c(%rbp)
1409a5f72: c7 85 64 02 00 00 00 00 80 3f       	movl	$0x3f800000, 0x264(%rbp) # imm = 0x3F800000
1409a5f7c: 48 8d 85 40 02 00 00        	leaq	0x240(%rbp), %rax
1409a5f83: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409a5f88: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1409a5f8f: f3 0f 10 0d f9 7b 04 02     	movss	0x2047bf9(%rip), %xmm1  # 0x1429edb90
1409a5f97: f3 44 0f 10 05 64 f9 f7 01  	movss	0x1f7f964(%rip), %xmm8  # 0x142925904
1409a5fa0: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1409a5fa4: 0f 28 df                    	movaps	%xmm7, %xmm3
1409a5fa7: e8 34 41 13 00              	callq	0x140ada0e0 <.text+0xad90e0>
1409a5fac: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
1409a5fb3: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1409a5fba: 48 8d 05 cf 86 00 00        	leaq	0x86cf(%rip), %rax      # 0x1409ae690 <.text+0x9ad690>
1409a5fc1: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1409a5fc8: 48 8d 05 61 75 11 02        	leaq	0x2117561(%rip), %rax   # 0x142abd530
1409a5fcf: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a5fd6: 48 c7 85 48 02 00 00 01 00 00 00    	movq	$0x1, 0x248(%rbp)
1409a5fe1: 48 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %rax
1409a5fe8: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1409a5fef: 48 c7 85 58 02 00 00 01 00 00 00    	movq	$0x1, 0x258(%rbp)
1409a5ffa: 48 c7 85 60 02 00 00 00 00 00 00    	movq	$0x0, 0x260(%rbp)
1409a6005: c6 85 36 03 00 00 01        	movb	$0x1, 0x336(%rbp)
1409a600c: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1409a6010: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1409a6017: e8 74 3f 3d 00              	callq	0x140d79f90 <.text+0xd78f90>
1409a601c: 0f 10 45 d0                 	movups	-0x30(%rbp), %xmm0
1409a6020: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1409a6027: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
1409a602b: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
1409a6032: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1409a6039: 0f 11 85 f7 01 00 00        	movups	%xmm0, 0x1f7(%rbp)
1409a6040: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1409a6047: 48 89 85 07 02 00 00        	movq	%rax, 0x207(%rbp)
1409a604e: 48 8b 85 a5 00 00 00        	movq	0xa5(%rbp), %rax
1409a6055: 48 8b 8d ac 00 00 00        	movq	0xac(%rbp), %rcx
1409a605c: 48 89 85 b9 02 00 00        	movq	%rax, 0x2b9(%rbp)
1409a6063: 48 89 8d c0 02 00 00        	movq	%rcx, 0x2c0(%rbp)
1409a606a: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1409a6071: 0f 11 85 c9 02 00 00        	movups	%xmm0, 0x2c9(%rbp)
1409a6078: 48 8b 85 ff 01 00 00        	movq	0x1ff(%rbp), %rax
1409a607f: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
1409a6086: 48 8b 85 07 02 00 00        	movq	0x207(%rbp), %rax
1409a608d: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
1409a6094: 0f 10 85 08 01 00 00        	movups	0x108(%rbp), %xmm0
1409a609b: 0f 11 85 74 02 00 00        	movups	%xmm0, 0x274(%rbp)
1409a60a2: 8b 85 18 01 00 00           	movl	0x118(%rbp), %eax
1409a60a8: 89 85 84 02 00 00           	movl	%eax, 0x284(%rbp)
1409a60ae: 8b 85 c4 00 00 00           	movl	0xc4(%rbp), %eax
1409a60b4: 89 85 9c 02 00 00           	movl	%eax, 0x29c(%rbp)
1409a60ba: 0f 10 85 b4 00 00 00        	movups	0xb4(%rbp), %xmm0
1409a60c1: 0f 11 85 8c 02 00 00        	movups	%xmm0, 0x28c(%rbp)
1409a60c8: 8b 45 74                    	movl	0x74(%rbp), %eax
1409a60cb: 89 85 b4 02 00 00           	movl	%eax, 0x2b4(%rbp)
1409a60d1: 0f 10 45 64                 	movups	0x64(%rbp), %xmm0
1409a60d5: 0f 11 85 a4 02 00 00        	movups	%xmm0, 0x2a4(%rbp)
1409a60dc: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
1409a60e7: 48 c7 85 48 02 00 00 08 00 00 00    	movq	$0x8, 0x248(%rbp)
1409a60f2: 48 c7 85 50 02 00 00 00 00 00 00    	movq	$0x0, 0x250(%rbp)
1409a60fd: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1409a6104: 89 85 58 02 00 00           	movl	%eax, 0x258(%rbp)
1409a610a: 48 c7 85 60 02 00 00 02 00 00 00    	movq	$0x2, 0x260(%rbp)
1409a6115: 8b 85 8c 01 00 00           	movl	0x18c(%rbp), %eax
1409a611b: 89 85 70 02 00 00           	movl	%eax, 0x270(%rbp)
1409a6121: 8b 85 b8 01 00 00           	movl	0x1b8(%rbp), %eax
1409a6127: 89 85 88 02 00 00           	movl	%eax, 0x288(%rbp)
1409a612d: 89 85 a0 02 00 00           	movl	%eax, 0x2a0(%rbp)
1409a6133: c6 85 b8 02 00 00 00        	movb	$0x0, 0x2b8(%rbp)
1409a613a: c6 85 c8 02 00 00 02        	movb	$0x2, 0x2c8(%rbp)
1409a6141: 66 c7 85 e8 02 00 00 00 01  	movw	$0x100, 0x2e8(%rbp)     # imm = 0x100
1409a614a: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a6151: 4c 8d b5 40 01 00 00        	leaq	0x140(%rbp), %r14
1409a6158: 48 8d 9d 40 02 00 00        	leaq	0x240(%rbp), %rbx
1409a615f: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
1409a6166: 4c 89 f2                    	movq	%r14, %rdx
1409a6169: 49 89 d8                    	movq	%rbx, %r8
1409a616c: e8 7f d0 f7 ff              	callq	0x1409231f0 <.text+0x9221f0>
1409a6171: 49 89 c4                    	movq	%rax, %r12
1409a6174: 48 89 d6                    	movq	%rdx, %rsi
1409a6177: 0f b6 40 18                 	movzbl	0x18(%rax), %eax
1409a617b: 84 c0                       	testb	%al, %al
1409a617d: 74 3d                       	je	0x1409a61bc <.text+0x9a51bc>
1409a617f: 49 39 74 24 38              	cmpq	%rsi, 0x38(%r12)
1409a6184: 75 36                       	jne	0x1409a61bc <.text+0x9a51bc>
1409a6186: 41 83 7c 24 28 02           	cmpl	$0x2, 0x28(%r12)
1409a618c: 0f 95 c1                    	setne	%cl
1409a618f: 8b 95 14 03 00 00           	movl	0x314(%rbp), %edx
1409a6195: 08 ca                       	orb	%cl, %dl
1409a6197: 89 95 14 03 00 00           	movl	%edx, 0x314(%rbp)
1409a619d: 41 0f b6 4c 24 19           	movzbl	0x19(%r12), %ecx
1409a61a3: 3c 01                       	cmpb	$0x1, %al
1409a61a5: 75 07                       	jne	0x1409a61ae <.text+0x9a51ae>
1409a61a7: f6 c1 01                    	testb	$0x1, %cl
1409a61aa: 75 07                       	jne	0x1409a61b3 <.text+0x9a51b3>
1409a61ac: eb 0e                       	jmp	0x1409a61bc <.text+0x9a51bc>
1409a61ae: 80 f9 02                    	cmpb	$0x2, %cl
1409a61b1: 74 09                       	je	0x1409a61bc <.text+0x9a51bc>
1409a61b3: f3 44 0f 10 05 6c 6f 02 02  	movss	0x2026f6c(%rip), %xmm8  # 0x1429cd128
1409a61bc: f3 44 0f 11 85 88 01 00 00  	movss	%xmm8, 0x188(%rbp)
1409a61c5: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
1409a61cc: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a61d3: 48 8d 05 b6 84 00 00        	leaq	0x84b6(%rip), %rax      # 0x1409ae690 <.text+0x9ad690>
1409a61da: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1409a61e1: 48 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %rax
1409a61e8: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1409a61ef: 48 8d 05 9a 4d 3a 00        	leaq	0x3a4d9a(%rip), %rax    # 0x140d4af90 <.text+0xd49f90>
1409a61f6: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1409a61fd: 48 8d 85 18 03 00 00        	leaq	0x318(%rbp), %rax
1409a6204: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1409a620b: 48 8d 05 ee 57 3a 00        	leaq	0x3a57ee(%rip), %rax    # 0x140d4ba00 <.text+0xd4aa00>
1409a6212: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1409a6219: 48 8d 85 88 01 00 00        	leaq	0x188(%rbp), %rax
1409a6220: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1409a6227: 48 8d 05 e2 66 3e 00        	leaq	0x3e66e2(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
1409a622e: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
1409a6235: 48 8d 05 14 73 11 02        	leaq	0x2117314(%rip), %rax   # 0x142abd550
1409a623c: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1409a6243: 48 c7 85 f8 01 00 00 04 00 00 00    	movq	$0x4, 0x1f8(%rbp)
1409a624e: 48 89 9d 00 02 00 00        	movq	%rbx, 0x200(%rbp)
1409a6255: 48 c7 85 08 02 00 00 04 00 00 00    	movq	$0x4, 0x208(%rbp)
1409a6260: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
1409a626b: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a6272: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
1409a6276: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1409a627d: e8 0e 3d 3d 00              	callq	0x140d79f90 <.text+0xd78f90>
1409a6282: 48 8b 45 18                 	movq	0x18(%rbp), %rax
1409a6286: 48 89 85 38 02 00 00        	movq	%rax, 0x238(%rbp)
1409a628d: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1409a6291: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
1409a6298: 48 8b 7d 28                 	movq	0x28(%rbp), %rdi
1409a629c: 0f 28 bd f0 00 00 00        	movaps	0xf0(%rbp), %xmm7
1409a62a3: f3 44 0f 10 85 88 01 00 00  	movss	0x188(%rbp), %xmm8
1409a62ac: 0f b6 05 56 e9 c8 02        	movzbl	0x2c8e956(%rip), %eax   # 0x143634c09
1409a62b3: b9 30 00 00 00              	movl	$0x30, %ecx
1409a62b8: ba 10 00 00 00              	movl	$0x10, %edx
1409a62bd: e8 4e d9 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a62c2: 48 85 c0                    	testq	%rax, %rax
1409a62c5: 0f 84 e8 12 00 00           	je	0x1409a75b3 <.text+0x9a65b3>
1409a62cb: 49 89 c7                    	movq	%rax, %r15
1409a62ce: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
1409a62d5: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
1409a62dd: 0f 29 78 10                 	movaps	%xmm7, 0x10(%rax)
1409a62e1: f3 44 0f 11 40 20           	movss	%xmm8, 0x20(%rax)
1409a62e7: 44 0f 13 48 24              	movlps	%xmm9, 0x24(%rax)
1409a62ec: f3 44 0f 11 50 2c           	movss	%xmm10, 0x2c(%rax)
1409a62f2: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a62f9: 4c 8d 2d 90 72 11 02        	leaq	0x2117290(%rip), %r13   # 0x142abd590
1409a6300: 4c 89 ad 48 02 00 00        	movq	%r13, 0x248(%rbp)
1409a6307: 48 85 ff                    	testq	%rdi, %rdi
1409a630a: 0f 88 b7 12 00 00           	js	0x1409a75c7 <.text+0x9a65c7>
1409a6310: 74 36                       	je	0x1409a6348 <.text+0x9a5348>
1409a6312: 4c 89 bd e8 01 00 00        	movq	%r15, 0x1e8(%rbp)
1409a6319: 0f b6 05 e9 e8 c8 02        	movzbl	0x2c8e8e9(%rip), %eax   # 0x143634c09
1409a6320: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1409a6326: ba 01 00 00 00              	movl	$0x1, %edx
1409a632b: 48 89 f9                    	movq	%rdi, %rcx
1409a632e: e8 dd d8 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a6333: 48 85 c0                    	testq	%rax, %rax
1409a6336: 0f 84 95 12 00 00           	je	0x1409a75d1 <.text+0x9a65d1>
1409a633c: 48 89 c3                    	movq	%rax, %rbx
1409a633f: 4c 8b bd e8 01 00 00        	movq	0x1e8(%rbp), %r15
1409a6346: eb 05                       	jmp	0x1409a634d <.text+0x9a534d>
1409a6348: bb 01 00 00 00              	movl	$0x1, %ebx
1409a634d: 48 89 d9                    	movq	%rbx, %rcx
1409a6350: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
1409a6357: 49 89 f8                    	movq	%rdi, %r8
1409a635a: e8 a1 d8 f7 01              	callq	0x142923c00 <.text+0x2922c00>
1409a635f: 48 89 bd 40 02 00 00        	movq	%rdi, 0x240(%rbp)
1409a6366: 48 89 9d 48 02 00 00        	movq	%rbx, 0x248(%rbp)
1409a636d: 48 89 bd 50 02 00 00        	movq	%rdi, 0x250(%rbp)
1409a6374: 4c 89 bd 58 02 00 00        	movq	%r15, 0x258(%rbp)
1409a637b: 4c 89 ad 60 02 00 00        	movq	%r13, 0x260(%rbp)
1409a6382: 48 8d 9d 40 02 00 00        	leaq	0x240(%rbp), %rbx
1409a6389: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
1409a6390: 48 89 da                    	movq	%rbx, %rdx
1409a6393: e8 e8 64 ac 00              	callq	0x14146c880 <.text+0x146b880>
1409a6398: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1409a639f: 48 85 d2                    	testq	%rdx, %rdx
1409a63a2: 49 bd 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %r13 # imm = 0x3F8000003F800000
1409a63ac: 74 12                       	je	0x1409a63c0 <.text+0x9a53c0>
1409a63ae: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409a63b4: 48 8b 8d 00 03 00 00        	movq	0x300(%rbp), %rcx
1409a63bb: e8 60 d8 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a63c0: b0 02                       	movb	$0x2, %al
1409a63c2: 41 80 7c 24 18 02           	cmpb	$0x2, 0x18(%r12)
1409a63c8: 72 0d                       	jb	0x1409a63d7 <.text+0x9a53d7>
1409a63ca: 49 39 74 24 38              	cmpq	%rsi, 0x38(%r12)
1409a63cf: 75 06                       	jne	0x1409a63d7 <.text+0x9a53d7>
1409a63d1: 41 0f b6 44 24 19           	movzbl	0x19(%r12), %eax
1409a63d7: 04 fe                       	addb	$-0x2, %al
1409a63d9: 0f b6 c0                    	movzbl	%al, %eax
1409a63dc: 3c 03                       	cmpb	$0x3, %al
1409a63de: b9 03 00 00 00              	movl	$0x3, %ecx
1409a63e3: 0f 42 c8                    	cmovbl	%eax, %ecx
1409a63e6: 0f b6 c1                    	movzbl	%cl, %eax
1409a63e9: 48 8d 0d b8 7b 11 02        	leaq	0x2117bb8(%rip), %rcx   # 0x142abdfa8
1409a63f0: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1409a63f4: 48 01 c8                    	addq	%rcx, %rax
1409a63f7: ff e0                       	jmpq	*%rax
1409a63f9: f3 0f 10 85 18 03 00 00     	movss	0x318(%rbp), %xmm0
1409a6401: f3 0f 10 8d 20 03 00 00     	movss	0x320(%rbp), %xmm1
1409a6409: f3 0f 11 85 80 01 00 00     	movss	%xmm0, 0x180(%rbp)
1409a6411: f3 0f 11 8d 84 01 00 00     	movss	%xmm1, 0x184(%rbp)
1409a6419: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a6420: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1409a6427: 48 8d 95 80 01 00 00        	leaq	0x180(%rbp), %rdx
1409a642e: 0f 57 d2                    	xorps	%xmm2, %xmm2
1409a6431: e8 aa 69 2f 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409a6436: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
1409a643d: 8b 01                       	movl	(%rcx), %eax
1409a643f: 8b 49 04                    	movl	0x4(%rcx), %ecx
1409a6442: 0f 57 d2                    	xorps	%xmm2, %xmm2
1409a6445: f3 48 0f 2a d0              	cvtsi2ss	%rax, %xmm2
1409a644a: f3 48 0f 2a d9              	cvtsi2ss	%rcx, %xmm3
1409a644f: 0f 28 fa                    	movaps	%xmm2, %xmm7
1409a6452: f3 0f 5e fb                 	divss	%xmm3, %xmm7
1409a6456: f3 0f 11 95 f0 01 00 00     	movss	%xmm2, 0x1f0(%rbp)
1409a645e: f3 0f 11 9d f4 01 00 00     	movss	%xmm3, 0x1f4(%rbp)
1409a6466: f3 0f 11 bd f8 01 00 00     	movss	%xmm7, 0x1f8(%rbp)
1409a646e: c7 85 fc 01 00 00 00 00 80 3f       	movl	$0x3f800000, 0x1fc(%rbp) # imm = 0x3F800000
1409a6478: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
1409a647f: f3 0f 10 00                 	movss	(%rax), %xmm0
1409a6483: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
1409a6488: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a648f: e8 3c a2 2e 00              	callq	0x140c906d0 <.text+0xc8f6d0>
1409a6494: 0f 28 d1                    	movaps	%xmm1, %xmm2
1409a6497: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
1409a649b: 8b 85 98 01 00 00           	movl	0x198(%rbp), %eax
1409a64a1: 89 85 48 02 00 00           	movl	%eax, 0x248(%rbp)
1409a64a7: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1409a64ae: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a64b5: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
1409a64bf: 48 89 85 4c 02 00 00        	movq	%rax, 0x24c(%rbp)
1409a64c6: c7 85 54 02 00 00 00 00 00 00       	movl	$0x0, 0x254(%rbp)
1409a64d0: f2 0f 10 05 48 66 11 02     	movsd	0x2116648(%rip), %xmm0  # 0x142abcb20
1409a64d8: f2 0f 11 85 40 01 00 00     	movsd	%xmm0, 0x140(%rbp)
1409a64e0: c6 85 48 01 00 00 00        	movb	$0x0, 0x148(%rbp)
1409a64e7: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a64ee: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1409a64f3: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1409a64f8: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
1409a64ff: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409a6504: 48 8d 8d dc 01 00 00        	leaq	0x1dc(%rbp), %rcx
1409a650b: 4c 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %r9
1409a6512: 0f 28 cf                    	movaps	%xmm7, %xmm1
1409a6515: e8 86 1f 12 00              	callq	0x140ac84a0 <.text+0xac74a0>
1409a651a: f3 0f 10 85 dc 01 00 00     	movss	0x1dc(%rbp), %xmm0
1409a6522: f3 0f 10 8d e0 01 00 00     	movss	0x1e0(%rbp), %xmm1
1409a652a: f3 0f 5c 85 18 03 00 00     	subss	0x318(%rbp), %xmm0
1409a6532: f3 0f 5c 8d 1c 03 00 00     	subss	0x31c(%rbp), %xmm1
1409a653a: f3 0f 10 95 e4 01 00 00     	movss	0x1e4(%rbp), %xmm2
1409a6542: f3 0f 5c 95 20 03 00 00     	subss	0x320(%rbp), %xmm2
1409a654a: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1409a654e: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1409a6552: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1409a6556: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
1409a655a: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1409a655e: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409a6561: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
1409a6565: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1409a656c: c7 40 28 00 00 00 00        	movl	$0x0, 0x28(%rax)
1409a6573: f3 0f 11 40 2c              	movss	%xmm0, 0x2c(%rax)
1409a6578: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1409a657f: f3 0f 10 41 f0              	movss	-0x10(%rcx), %xmm0
1409a6584: f3 0f 11 40 3c              	movss	%xmm0, 0x3c(%rax)
1409a6589: f3 0f 10 41 f8              	movss	-0x8(%rcx), %xmm0
1409a658e: f3 0f 11 40 38              	movss	%xmm0, 0x38(%rax)
1409a6593: 48 8b 95 58 04 00 00        	movq	0x458(%rbp), %rdx
1409a659a: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409a659d: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409a65a1: 89 01                       	movl	%eax, (%rcx)
1409a65a3: 48 8b 02                    	movq	(%rdx), %rax
1409a65a6: c6 80 e0 01 00 00 01        	movb	$0x1, 0x1e0(%rax)
1409a65ad: e9 fe 02 00 00              	jmp	0x1409a68b0 <.text+0x9a58b0>
1409a65b2: f3 0f 10 85 18 03 00 00     	movss	0x318(%rbp), %xmm0
1409a65ba: f3 0f 10 8d 20 03 00 00     	movss	0x320(%rbp), %xmm1
1409a65c2: f3 0f 11 85 80 01 00 00     	movss	%xmm0, 0x180(%rbp)
1409a65ca: f3 0f 11 8d 84 01 00 00     	movss	%xmm1, 0x184(%rbp)
1409a65d2: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a65d9: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1409a65e0: 48 8d 95 80 01 00 00        	leaq	0x180(%rbp), %rdx
1409a65e7: 0f 57 d2                    	xorps	%xmm2, %xmm2
1409a65ea: e8 f1 67 2f 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409a65ef: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
1409a65f6: 8b 01                       	movl	(%rcx), %eax
1409a65f8: 8b 49 04                    	movl	0x4(%rcx), %ecx
1409a65fb: 0f 57 d2                    	xorps	%xmm2, %xmm2
1409a65fe: f3 48 0f 2a d0              	cvtsi2ss	%rax, %xmm2
1409a6603: f3 48 0f 2a d9              	cvtsi2ss	%rcx, %xmm3
1409a6608: 0f 28 fa                    	movaps	%xmm2, %xmm7
1409a660b: f3 0f 5e fb                 	divss	%xmm3, %xmm7
1409a660f: f3 0f 11 95 f0 01 00 00     	movss	%xmm2, 0x1f0(%rbp)
1409a6617: f3 0f 11 9d f4 01 00 00     	movss	%xmm3, 0x1f4(%rbp)
1409a661f: f3 0f 11 bd f8 01 00 00     	movss	%xmm7, 0x1f8(%rbp)
1409a6627: c7 85 fc 01 00 00 00 00 80 3f       	movl	$0x3f800000, 0x1fc(%rbp) # imm = 0x3F800000
1409a6631: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
1409a6638: f3 0f 10 00                 	movss	(%rax), %xmm0
1409a663c: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
1409a6641: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a6648: e8 83 a0 2e 00              	callq	0x140c906d0 <.text+0xc8f6d0>
1409a664d: 0f 28 d1                    	movaps	%xmm1, %xmm2
1409a6650: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
1409a6654: 8b 85 98 01 00 00           	movl	0x198(%rbp), %eax
1409a665a: 89 85 48 02 00 00           	movl	%eax, 0x248(%rbp)
1409a6660: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1409a6667: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a666e: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
1409a6678: 48 89 85 4c 02 00 00        	movq	%rax, 0x24c(%rbp)
1409a667f: c7 85 54 02 00 00 00 00 00 00       	movl	$0x0, 0x254(%rbp)
1409a6689: f2 0f 10 05 8f 64 11 02     	movsd	0x211648f(%rip), %xmm0  # 0x142abcb20
1409a6691: f2 0f 11 85 40 01 00 00     	movsd	%xmm0, 0x140(%rbp)
1409a6699: c6 85 48 01 00 00 00        	movb	$0x0, 0x148(%rbp)
1409a66a0: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a66a7: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1409a66ac: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1409a66b1: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
1409a66b8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409a66bd: 48 8d 8d dc 01 00 00        	leaq	0x1dc(%rbp), %rcx
1409a66c4: 4c 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %r9
1409a66cb: 0f 28 cf                    	movaps	%xmm7, %xmm1
1409a66ce: e8 cd 1d 12 00              	callq	0x140ac84a0 <.text+0xac74a0>
1409a66d3: f3 0f 10 85 18 03 00 00     	movss	0x318(%rbp), %xmm0
1409a66db: f3 0f 10 8d 20 03 00 00     	movss	0x320(%rbp), %xmm1
1409a66e3: f3 0f 11 85 f0 01 00 00     	movss	%xmm0, 0x1f0(%rbp)
1409a66eb: f3 0f 11 8d f4 01 00 00     	movss	%xmm1, 0x1f4(%rbp)
1409a66f3: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a66fa: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a6701: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1409a6708: 0f 57 ff                    	xorps	%xmm7, %xmm7
1409a670b: 0f 57 d2                    	xorps	%xmm2, %xmm2
1409a670e: e8 cd 66 2f 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409a6713: f3 0f 10 85 e0 01 00 00     	movss	0x1e0(%rbp), %xmm0
1409a671b: f3 0f 5c 85 44 02 00 00     	subss	0x244(%rbp), %xmm0
1409a6723: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1409a6727: f3 0f 10 8d e4 01 00 00     	movss	0x1e4(%rbp), %xmm1
1409a672f: f3 0f 10 95 dc 01 00 00     	movss	0x1dc(%rbp), %xmm2
1409a6737: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
1409a673a: f3 0f 10 8d 40 02 00 00     	movss	0x240(%rbp), %xmm1
1409a6742: f3 0f 10 9d 48 02 00 00     	movss	0x248(%rbp), %xmm3
1409a674a: 0f 14 cb                    	unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
1409a674d: 0f 5c d1                    	subps	%xmm1, %xmm2
1409a6750: 0f 59 d2                    	mulps	%xmm2, %xmm2
1409a6753: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1409a6757: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
1409a675b: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1409a675f: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409a6762: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1409a6766: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1409a676d: f3 0f 5c 40 2c              	subss	0x2c(%rax), %xmm0
1409a6772: f3 0f 5f f8                 	maxss	%xmm0, %xmm7
1409a6776: f3 44 0f 10 05 b1 92 0d 02  	movss	0x20d92b1(%rip), %xmm8  # 0x142a7fa30
1409a677f: f3 44 0f 5d c7              	minss	%xmm7, %xmm8
1409a6784: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1409a678b: 44 0f 2e 40 f8              	ucomiss	-0x8(%rax), %xmm8
1409a6790: 75 06                       	jne	0x1409a6798 <.text+0x9a5798>
1409a6792: 0f 8b 18 01 00 00           	jnp	0x1409a68b0 <.text+0x9a58b0>
1409a6798: 8b bd 30 03 00 00           	movl	0x330(%rbp), %edi
1409a679e: 48 8b 95 78 04 00 00        	movq	0x478(%rbp), %rdx
1409a67a5: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409a67a8: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409a67ac: 89 01                       	movl	%eax, (%rcx)
1409a67ae: 48 8b 32                    	movq	(%rdx), %rsi
1409a67b1: 4c 8b 76 30                 	movq	0x30(%rsi), %r14
1409a67b5: 4c 8b 7e 40                 	movq	0x40(%rsi), %r15
1409a67b9: 4c 3b 76 20                 	cmpq	0x20(%rsi), %r14
1409a67bd: 75 17                       	jne	0x1409a67d6 <.text+0x9a57d6>
1409a67bf: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1409a67c3: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a67ca: 48 8d 15 77 39 29 02        	leaq	0x2293977(%rip), %rdx   # 0x142c3a148
1409a67d1: e8 da c2 bb ff              	callq	0x140562ab0 <.text+0x561ab0>
1409a67d6: 48 8b 46 28                 	movq	0x28(%rsi), %rax
1409a67da: 4b 8d 0c b6                 	leaq	(%r14,%r14,4), %rcx
1409a67de: c7 04 c8 00 00 00 00        	movl	$0x0, (%rax,%rcx,8)
1409a67e5: c7 44 c8 08 01 00 00 00     	movl	$0x1, 0x8(%rax,%rcx,8)
1409a67ed: f3 44 0f 11 44 c8 0c        	movss	%xmm8, 0xc(%rax,%rcx,8)
1409a67f4: 44 0f 13 4c c8 10           	movlps	%xmm9, 0x10(%rax,%rcx,8)
1409a67fa: f3 44 0f 11 54 c8 18        	movss	%xmm10, 0x18(%rax,%rcx,8)
1409a6801: 89 7c c8 1c                 	movl	%edi, 0x1c(%rax,%rcx,8)
1409a6805: 4c 89 7c c8 20              	movq	%r15, 0x20(%rax,%rcx,8)
1409a680a: 49 ff c6                    	incq	%r14
1409a680d: 4c 89 76 30                 	movq	%r14, 0x30(%rsi)
1409a6811: 48 ff 46 40                 	incq	0x40(%rsi)
1409a6815: e9 96 00 00 00              	jmp	0x1409a68b0 <.text+0x9a58b0>
1409a681a: 48 8b 95 50 04 00 00        	movq	0x450(%rbp), %rdx
1409a6821: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409a6824: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409a6828: 89 01                       	movl	%eax, (%rcx)
1409a682a: 48 8b 02                    	movq	(%rdx), %rax
1409a682d: 48 8b 8d 40 04 00 00        	movq	0x440(%rbp), %rcx
1409a6834: f3 0f 10 41 38              	movss	0x38(%rcx), %xmm0
1409a6839: f3 0f 10 49 3c              	movss	0x3c(%rcx), %xmm1
1409a683e: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1409a6845: f3 0f 10 51 f0              	movss	-0x10(%rcx), %xmm2
1409a684a: f3 0f 10 59 f8              	movss	-0x8(%rcx), %xmm3
1409a684f: 8b 8d 30 03 00 00           	movl	0x330(%rbp), %ecx
1409a6855: 48 8b 50 20                 	movq	0x20(%rax), %rdx
1409a6859: 48 89 95 60 02 00 00        	movq	%rdx, 0x260(%rbp)
1409a6860: 0f 10 20                    	movups	(%rax), %xmm4
1409a6863: 0f 10 68 10                 	movups	0x10(%rax), %xmm5
1409a6867: 0f 29 ad 50 02 00 00        	movaps	%xmm5, 0x250(%rbp)
1409a686e: 0f 29 a5 40 02 00 00        	movaps	%xmm4, 0x240(%rbp)
1409a6875: 48 ba 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rdx # imm = 0x8000000000000001
1409a687f: 48 89 10                    	movq	%rdx, (%rax)
1409a6882: 89 48 08                    	movl	%ecx, 0x8(%rax)
1409a6885: f3 0f 11 40 0c              	movss	%xmm0, 0xc(%rax)
1409a688a: f3 0f 11 58 10              	movss	%xmm3, 0x10(%rax)
1409a688f: f3 0f 11 48 14              	movss	%xmm1, 0x14(%rax)
1409a6894: f3 0f 11 50 18              	movss	%xmm2, 0x18(%rax)
1409a6899: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
1409a68a3: 48 39 85 40 02 00 00        	cmpq	%rax, 0x240(%rbp)
1409a68aa: 0f 85 95 0d 00 00           	jne	0x1409a7645 <.text+0x9a6645>
1409a68b0: 8b 85 20 03 00 00           	movl	0x320(%rbp), %eax
1409a68b6: 89 85 58 02 00 00           	movl	%eax, 0x258(%rbp)
1409a68bc: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
1409a68c3: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1409a68ca: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
1409a68d1: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
1409a68d8: 4c 89 ad 5c 02 00 00        	movq	%r13, 0x25c(%rbp)
1409a68df: c7 85 64 02 00 00 00 00 80 3f       	movl	$0x3f800000, 0x264(%rbp) # imm = 0x3F800000
1409a68e9: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1409a68ee: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1409a68f5: f3 0f 10 15 73 ef f7 01     	movss	0x1f7ef73(%rip), %xmm2  # 0x142925870
1409a68fd: f3 0f 10 0d cb ef f7 01     	movss	0x1f7efcb(%rip), %xmm1  # 0x1429258d0
1409a6905: 0f 28 d9                    	movaps	%xmm1, %xmm3
1409a6908: e8 d3 37 13 00              	callq	0x140ada0e0 <.text+0xad90e0>
1409a690d: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
1409a6914: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1409a691b: 48 8d 05 6e 7d 00 00        	leaq	0x7d6e(%rip), %rax      # 0x1409ae690 <.text+0x9ad690>
1409a6922: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1409a6929: 48 8d 05 b8 6c 11 02        	leaq	0x2116cb8(%rip), %rax   # 0x142abd5e8
1409a6930: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a6937: 48 c7 85 48 02 00 00 01 00 00 00    	movq	$0x1, 0x248(%rbp)
1409a6942: 48 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %rax
1409a6949: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1409a6950: 48 c7 85 58 02 00 00 01 00 00 00    	movq	$0x1, 0x258(%rbp)
1409a695b: 48 c7 85 60 02 00 00 00 00 00 00    	movq	$0x0, 0x260(%rbp)
1409a6966: c6 85 34 03 00 00 01        	movb	$0x1, 0x334(%rbp)
1409a696d: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1409a6971: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1409a6978: e8 13 36 3d 00              	callq	0x140d79f90 <.text+0xd78f90>
1409a697d: 0f 10 45 b8                 	movups	-0x48(%rbp), %xmm0
1409a6981: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1409a6988: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
1409a698c: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
1409a6993: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1409a699a: 0f 11 85 f7 01 00 00        	movups	%xmm0, 0x1f7(%rbp)
1409a69a1: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1409a69a8: 48 89 85 07 02 00 00        	movq	%rax, 0x207(%rbp)
1409a69af: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1409a69b6: 0f 11 85 c9 02 00 00        	movups	%xmm0, 0x2c9(%rbp)
1409a69bd: 48 8b 85 ff 01 00 00        	movq	0x1ff(%rbp), %rax
1409a69c4: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
1409a69cb: 48 8b 85 07 02 00 00        	movq	0x207(%rbp), %rax
1409a69d2: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
1409a69d9: 48 8b 85 a5 00 00 00        	movq	0xa5(%rbp), %rax
1409a69e0: 48 8b 8d ac 00 00 00        	movq	0xac(%rbp), %rcx
1409a69e7: 48 89 85 b9 02 00 00        	movq	%rax, 0x2b9(%rbp)
1409a69ee: 48 89 8d c0 02 00 00        	movq	%rcx, 0x2c0(%rbp)
1409a69f5: 0f 10 85 08 01 00 00        	movups	0x108(%rbp), %xmm0
1409a69fc: 0f 11 85 74 02 00 00        	movups	%xmm0, 0x274(%rbp)
1409a6a03: 8b 85 18 01 00 00           	movl	0x118(%rbp), %eax
1409a6a09: 89 85 84 02 00 00           	movl	%eax, 0x284(%rbp)
1409a6a0f: 8b 85 c4 00 00 00           	movl	0xc4(%rbp), %eax
1409a6a15: 89 85 9c 02 00 00           	movl	%eax, 0x29c(%rbp)
1409a6a1b: 0f 10 85 b4 00 00 00        	movups	0xb4(%rbp), %xmm0
1409a6a22: 0f 11 85 8c 02 00 00        	movups	%xmm0, 0x28c(%rbp)
1409a6a29: 8b 45 74                    	movl	0x74(%rbp), %eax
1409a6a2c: 89 85 b4 02 00 00           	movl	%eax, 0x2b4(%rbp)
1409a6a32: 0f 10 45 64                 	movups	0x64(%rbp), %xmm0
1409a6a36: 0f 11 85 a4 02 00 00        	movups	%xmm0, 0x2a4(%rbp)
1409a6a3d: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
1409a6a48: 48 c7 85 48 02 00 00 08 00 00 00    	movq	$0x8, 0x248(%rbp)
1409a6a53: 48 c7 85 50 02 00 00 00 00 00 00    	movq	$0x0, 0x250(%rbp)
1409a6a5e: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1409a6a65: 89 85 58 02 00 00           	movl	%eax, 0x258(%rbp)
1409a6a6b: 48 c7 85 60 02 00 00 02 00 00 00    	movq	$0x2, 0x260(%rbp)
1409a6a76: 8b 85 8c 01 00 00           	movl	0x18c(%rbp), %eax
1409a6a7c: 89 85 70 02 00 00           	movl	%eax, 0x270(%rbp)
1409a6a82: 8b 85 b8 01 00 00           	movl	0x1b8(%rbp), %eax
1409a6a88: 89 85 88 02 00 00           	movl	%eax, 0x288(%rbp)
1409a6a8e: 89 85 a0 02 00 00           	movl	%eax, 0x2a0(%rbp)
1409a6a94: c6 85 b8 02 00 00 00        	movb	$0x0, 0x2b8(%rbp)
1409a6a9b: c6 85 c8 02 00 00 02        	movb	$0x2, 0x2c8(%rbp)
1409a6aa2: 66 c7 85 e8 02 00 00 00 01  	movw	$0x100, 0x2e8(%rbp)     # imm = 0x100
1409a6aab: c6 85 34 03 00 00 00        	movb	$0x0, 0x334(%rbp)
1409a6ab2: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
1409a6ab9: 4c 8d a5 40 02 00 00        	leaq	0x240(%rbp), %r12
1409a6ac0: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
1409a6ac7: 4d 89 e0                    	movq	%r12, %r8
1409a6aca: e8 21 c7 f7 ff              	callq	0x1409231f0 <.text+0x9221f0>
1409a6acf: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
1409a6ad6: 48 89 95 b8 01 00 00        	movq	%rdx, 0x1b8(%rbp)
1409a6add: 4c 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %r8
1409a6ae4: 41 0f b6 40 18              	movzbl	0x18(%r8), %eax
1409a6ae9: 84 c0                       	testb	%al, %al
1409a6aeb: 74 39                       	je	0x1409a6b26 <.text+0x9a5b26>
1409a6aed: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
1409a6af4: 49 39 48 38                 	cmpq	%rcx, 0x38(%r8)
1409a6af8: 75 2c                       	jne	0x1409a6b26 <.text+0x9a5b26>
1409a6afa: 41 83 78 28 02              	cmpl	$0x2, 0x28(%r8)
1409a6aff: 0f 95 c1                    	setne	%cl
1409a6b02: 8b 95 14 03 00 00           	movl	0x314(%rbp), %edx
1409a6b08: 08 ca                       	orb	%cl, %dl
1409a6b0a: 89 95 14 03 00 00           	movl	%edx, 0x314(%rbp)
1409a6b10: 41 0f b6 48 19              	movzbl	0x19(%r8), %ecx
1409a6b15: 3c 01                       	cmpb	$0x1, %al
1409a6b17: 0f 85 a3 06 00 00           	jne	0x1409a71c0 <.text+0x9a61c0>
1409a6b1d: f6 c1 01                    	testb	$0x1, %cl
1409a6b20: 0f 85 a3 06 00 00           	jne	0x1409a71c9 <.text+0x9a61c9>
1409a6b26: f3 0f 10 05 d6 ed f7 01     	movss	0x1f7edd6(%rip), %xmm0  # 0x142925904
1409a6b2e: f3 0f 11 85 dc 01 00 00     	movss	%xmm0, 0x1dc(%rbp)
1409a6b36: c6 85 f0 01 00 00 00        	movb	$0x0, 0x1f0(%rbp)
1409a6b3d: 48 8d 05 b4 6a 11 02        	leaq	0x2116ab4(%rip), %rax   # 0x142abd5f8
1409a6b44: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1409a6b4b: 48 c7 85 00 02 00 00 0a 00 00 00    	movq	$0xa, 0x200(%rbp)
1409a6b56: c6 85 08 02 00 00 01        	movb	$0x1, 0x208(%rbp)
1409a6b5d: 48 8d 05 9e 6a 11 02        	leaq	0x2116a9e(%rip), %rax   # 0x142abd602
1409a6b64: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1409a6b6b: 48 c7 85 18 02 00 00 12 00 00 00    	movq	$0x12, 0x218(%rbp)
1409a6b76: 48 c7 85 28 02 00 00 02 00 00 00    	movq	$0x2, 0x228(%rbp)
1409a6b81: 48 8d 35 c8 6c 11 02        	leaq	0x2116cc8(%rip), %rsi   # 0x142abd850
1409a6b88: 31 c0                       	xorl	%eax, %eax
1409a6b8a: eb 07                       	jmp	0x1409a6b93 <.text+0x9a5b93>
1409a6b8c: 0f 1f 40 00                 	nopl	(%rax)
1409a6b90: 48 89 d8                    	movq	%rbx, %rax
1409a6b93: 48 83 f8 02                 	cmpq	$0x2, %rax
1409a6b97: 0f 84 7b 02 00 00           	je	0x1409a6e18 <.text+0x9a5e18>
1409a6b9d: 48 8d 58 01                 	leaq	0x1(%rax), %rbx
1409a6ba1: 48 89 9d 20 02 00 00        	movq	%rbx, 0x220(%rbp)
1409a6ba8: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1409a6bac: 44 0f b6 bc c5 f0 01 00 00  	movzbl	0x1f0(%rbp,%rax,8), %r15d
1409a6bb5: 41 80 ff 02                 	cmpb	$0x2, %r15b
1409a6bb9: 0f 84 59 02 00 00           	je	0x1409a6e18 <.text+0x9a5e18>
1409a6bbf: 48 8d 04 c5 f0 01 00 00     	leaq	0x1f0(,%rax,8), %rax
1409a6bc7: 48 01 e8                    	addq	%rbp, %rax
1409a6bca: 0f 10 40 08                 	movups	0x8(%rax), %xmm0
1409a6bce: 0f 11 85 90 01 00 00        	movups	%xmm0, 0x190(%rbp)
1409a6bd5: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
1409a6bdc: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a6be3: 48 8d 05 a6 7a 00 00        	leaq	0x7aa6(%rip), %rax      # 0x1409ae690 <.text+0x9ad690>
1409a6bea: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1409a6bf1: 48 8d 85 18 03 00 00        	leaq	0x318(%rbp), %rax
1409a6bf8: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1409a6bff: 48 8d 05 fa 4d 3a 00        	leaq	0x3a4dfa(%rip), %rax    # 0x140d4ba00 <.text+0xd4aa00>
1409a6c06: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1409a6c0d: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
1409a6c14: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1409a6c1b: 48 8d 05 6e 32 76 ff        	leaq	-0x89cd92(%rip), %rax   # 0x140109e90 <.text+0x108e90>
1409a6c22: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1409a6c29: 48 8d 85 dc 01 00 00        	leaq	0x1dc(%rbp), %rax
1409a6c30: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1409a6c37: 48 8d 05 d2 5c 3e 00        	leaq	0x3e5cd2(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
1409a6c3e: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
1409a6c45: 48 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %rax
1409a6c4c: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
1409a6c53: 48 8d 05 36 43 3a 00        	leaq	0x3a4336(%rip), %rax    # 0x140d4af90 <.text+0xd49f90>
1409a6c5a: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
1409a6c61: 48 8d 05 98 6b 11 02        	leaq	0x2116b98(%rip), %rax   # 0x142abd800
1409a6c68: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
1409a6c6f: 48 c7 85 48 01 00 00 05 00 00 00    	movq	$0x5, 0x148(%rbp)
1409a6c7a: 4c 89 a5 50 01 00 00        	movq	%r12, 0x150(%rbp)
1409a6c81: 48 c7 85 58 01 00 00 05 00 00 00    	movq	$0x5, 0x158(%rbp)
1409a6c8c: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
1409a6c97: c6 85 34 03 00 00 00        	movb	$0x0, 0x334(%rbp)
1409a6c9e: 48 89 e9                    	movq	%rbp, %rcx
1409a6ca1: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
1409a6ca8: e8 e3 32 3d 00              	callq	0x140d79f90 <.text+0xd78f90>
1409a6cad: 48 8b 45 00                 	movq	(%rbp), %rax
1409a6cb1: 48 89 85 08 03 00 00        	movq	%rax, 0x308(%rbp)
1409a6cb8: 48 8b 45 08                 	movq	0x8(%rbp), %rax
1409a6cbc: 48 89 85 28 03 00 00        	movq	%rax, 0x328(%rbp)
1409a6cc3: 48 8b 7d 10                 	movq	0x10(%rbp), %rdi
1409a6cc7: 4c 8b ad 90 01 00 00        	movq	0x190(%rbp), %r13
1409a6cce: 4c 8b b5 98 01 00 00        	movq	0x198(%rbp), %r14
1409a6cd5: f3 0f 10 bd dc 01 00 00     	movss	0x1dc(%rbp), %xmm7
1409a6cdd: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
1409a6ce4: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a6ceb: 8b 85 20 03 00 00           	movl	0x320(%rbp), %eax
1409a6cf1: 89 85 48 02 00 00           	movl	%eax, 0x248(%rbp)
1409a6cf7: 44 0f 28 85 f0 00 00 00     	movaps	0xf0(%rbp), %xmm8
1409a6cff: 0f b6 05 03 df c8 02        	movzbl	0x2c8df03(%rip), %eax   # 0x143634c09
1409a6d06: b9 50 00 00 00              	movl	$0x50, %ecx
1409a6d0b: ba 10 00 00 00              	movl	$0x10, %edx
1409a6d10: e8 fb ce c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a6d15: 48 85 c0                    	testq	%rax, %rax
1409a6d18: 0f 84 22 08 00 00           	je	0x1409a7540 <.text+0x9a6540>
1409a6d1e: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
1409a6d25: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
1409a6d2d: 4c 89 68 10                 	movq	%r13, 0x10(%rax)
1409a6d31: 4c 89 70 18                 	movq	%r14, 0x18(%rax)
1409a6d35: 44 0f 29 40 20              	movaps	%xmm8, 0x20(%rax)
1409a6d3a: f3 0f 11 78 30              	movss	%xmm7, 0x30(%rax)
1409a6d3f: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
1409a6d46: 48 89 48 34                 	movq	%rcx, 0x34(%rax)
1409a6d4a: 8b 8d 48 02 00 00           	movl	0x248(%rbp), %ecx
1409a6d50: 89 48 3c                    	movl	%ecx, 0x3c(%rax)
1409a6d53: 44 88 78 40                 	movb	%r15b, 0x40(%rax)
1409a6d57: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a6d5e: 48 89 b5 48 02 00 00        	movq	%rsi, 0x248(%rbp)
1409a6d65: 48 85 ff                    	testq	%rdi, %rdi
1409a6d68: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1409a6d6f: 0f 88 df 07 00 00           	js	0x1409a7554 <.text+0x9a6554>
1409a6d75: 74 29                       	je	0x1409a6da0 <.text+0x9a5da0>
1409a6d77: 0f b6 05 8b de c8 02        	movzbl	0x2c8de8b(%rip), %eax   # 0x143634c09
1409a6d7e: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1409a6d84: ba 01 00 00 00              	movl	$0x1, %edx
1409a6d89: 48 89 f9                    	movq	%rdi, %rcx
1409a6d8c: e8 7f ce c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a6d91: 48 85 c0                    	testq	%rax, %rax
1409a6d94: 0f 84 bd 07 00 00           	je	0x1409a7557 <.text+0x9a6557>
1409a6d9a: 49 89 c5                    	movq	%rax, %r13
1409a6d9d: eb 07                       	jmp	0x1409a6da6 <.text+0x9a5da6>
1409a6d9f: 90                          	nop
1409a6da0: 41 bd 01 00 00 00           	movl	$0x1, %r13d
1409a6da6: 4c 89 e9                    	movq	%r13, %rcx
1409a6da9: 48 8b 95 28 03 00 00        	movq	0x328(%rbp), %rdx
1409a6db0: 49 89 f8                    	movq	%rdi, %r8
1409a6db3: e8 48 ce f7 01              	callq	0x142923c00 <.text+0x2922c00>
1409a6db8: 48 89 bd 40 02 00 00        	movq	%rdi, 0x240(%rbp)
1409a6dbf: 4c 89 ad 48 02 00 00        	movq	%r13, 0x248(%rbp)
1409a6dc6: 48 89 bd 50 02 00 00        	movq	%rdi, 0x250(%rbp)
1409a6dcd: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1409a6dd4: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1409a6ddb: 48 89 b5 60 02 00 00        	movq	%rsi, 0x260(%rbp)
1409a6de2: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
1409a6de9: 4c 89 e2                    	movq	%r12, %rdx
1409a6dec: e8 8f 5a ac 00              	callq	0x14146c880 <.text+0x146b880>
1409a6df1: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
1409a6df8: 48 85 d2                    	testq	%rdx, %rdx
1409a6dfb: 0f 84 8f fd ff ff           	je	0x1409a6b90 <.text+0x9a5b90>
1409a6e01: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409a6e07: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
1409a6e0e: e8 0d ce c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a6e13: e9 78 fd ff ff              	jmp	0x1409a6b90 <.text+0x9a5b90>
1409a6e18: b0 02                       	movb	$0x2, %al
1409a6e1a: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
1409a6e21: 80 79 18 02                 	cmpb	$0x2, 0x18(%rcx)
1409a6e25: 73 09                       	jae	0x1409a6e30 <.text+0x9a5e30>
1409a6e27: 4c 8b a5 e0 00 00 00        	movq	0xe0(%rbp), %r12
1409a6e2e: eb 18                       	jmp	0x1409a6e48 <.text+0x9a5e48>
1409a6e30: 48 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %rdx
1409a6e37: 48 39 51 38                 	cmpq	%rdx, 0x38(%rcx)
1409a6e3b: 4c 8b a5 e0 00 00 00        	movq	0xe0(%rbp), %r12
1409a6e42: 75 04                       	jne	0x1409a6e48 <.text+0x9a5e48>
1409a6e44: 0f b6 41 19                 	movzbl	0x19(%rcx), %eax
1409a6e48: 04 fe                       	addb	$-0x2, %al
1409a6e4a: 0f b6 c0                    	movzbl	%al, %eax
1409a6e4d: 3c 03                       	cmpb	$0x3, %al
1409a6e4f: b9 03 00 00 00              	movl	$0x3, %ecx
1409a6e54: 0f 42 c8                    	cmovbl	%eax, %ecx
1409a6e57: 0f b6 c1                    	movzbl	%cl, %eax
1409a6e5a: 48 8d 0d 57 71 11 02        	leaq	0x2117157(%rip), %rcx   # 0x142abdfb8
1409a6e61: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1409a6e65: 48 01 c8                    	addq	%rcx, %rax
1409a6e68: ff e0                       	jmpq	*%rax
1409a6e6a: 48 8b 95 58 04 00 00        	movq	0x458(%rbp), %rdx
1409a6e71: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409a6e74: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409a6e78: 89 01                       	movl	%eax, (%rcx)
1409a6e7a: 48 8b 02                    	movq	(%rdx), %rax
1409a6e7d: c6 80 e0 01 00 00 01        	movb	$0x1, 0x1e0(%rax)
1409a6e84: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1409a6e8b: 48 8b 70 e8                 	movq	-0x18(%rax), %rsi
1409a6e8f: 45 31 ff                    	xorl	%r15d, %r15d
1409a6e92: b9 0c 00 00 00              	movl	$0xc, %ecx
1409a6e97: 48 89 f0                    	movq	%rsi, %rax
1409a6e9a: 48 f7 e1                    	mulq	%rcx
1409a6e9d: 48 89 c7                    	movq	%rax, %rdi
1409a6ea0: 0f 80 42 07 00 00           	jo	0x1409a75e8 <.text+0x9a65e8>
1409a6ea6: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
1409a6eb0: 48 39 c7                    	cmpq	%rax, %rdi
1409a6eb3: 0f 87 2f 07 00 00           	ja	0x1409a75e8 <.text+0x9a65e8>
1409a6eb9: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1409a6ec0: 4c 8b 70 e0                 	movq	-0x20(%rax), %r14
1409a6ec4: 48 85 ff                    	testq	%rdi, %rdi
1409a6ec7: 0f 84 f0 03 00 00           	je	0x1409a72bd <.text+0x9a62bd>
1409a6ecd: 0f b6 05 35 dd c8 02        	movzbl	0x2c8dd35(%rip), %eax   # 0x143634c09
1409a6ed4: 41 bf 04 00 00 00           	movl	$0x4, %r15d
1409a6eda: ba 04 00 00 00              	movl	$0x4, %edx
1409a6edf: 48 89 f9                    	movq	%rdi, %rcx
1409a6ee2: e8 29 cd c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a6ee7: 48 85 c0                    	testq	%rax, %rax
1409a6eea: 0f 84 f8 06 00 00           	je	0x1409a75e8 <.text+0x9a65e8>
1409a6ef0: 48 89 c3                    	movq	%rax, %rbx
1409a6ef3: 49 89 f7                    	movq	%rsi, %r15
1409a6ef6: e9 ca 03 00 00              	jmp	0x1409a72c5 <.text+0x9a62c5>
1409a6efb: 31 c0                       	xorl	%eax, %eax
1409a6efd: 48 8b 8d 40 04 00 00        	movq	0x440(%rbp), %rcx
1409a6f04: 48 3b 01                    	cmpq	(%rcx), %rax
1409a6f07: 0f 80 68 07 00 00           	jo	0x1409a7675 <.text+0x9a6675>
1409a6f0d: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1409a6f11: 48 85 c0                    	testq	%rax, %rax
1409a6f14: 0f 84 67 07 00 00           	je	0x1409a7681 <.text+0x9a6681>
1409a6f1a: 49 89 c8                    	movq	%rcx, %r8
1409a6f1d: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1409a6f21: 8b 51 08                    	movl	0x8(%rcx), %edx
1409a6f24: 89 95 c8 01 00 00           	movl	%edx, 0x1c8(%rbp)
1409a6f2a: 48 8b 11                    	movq	(%rcx), %rdx
1409a6f2d: 48 89 95 c0 01 00 00        	movq	%rdx, 0x1c0(%rbp)
1409a6f34: f2 41 0f 10 40 28           	movsd	0x28(%r8), %xmm0
1409a6f3a: 0f 28 ce                    	movaps	%xmm6, %xmm1
1409a6f3d: 0f 5c c8                    	subps	%xmm0, %xmm1
1409a6f40: 0f 13 8d 90 01 00 00        	movlps	%xmm1, 0x190(%rbp)
1409a6f47: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1409a6f4b: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1409a6f4f: 48 89 8d 40 02 00 00        	movq	%rcx, 0x240(%rbp)
1409a6f56: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1409a6f5d: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
1409a6f64: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1409a6f6b: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1409a6f72: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1409a6f79: 4c 8d 05 48 5c 11 02        	leaq	0x2115c48(%rip), %r8    # 0x142abcbc8
1409a6f80: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1409a6f87: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1409a6f8e: e8 3d 0a 96 00              	callq	0x1413079d0 <.text+0x13069d0>
1409a6f93: f3 0f 16 d6                 	movshdup	%xmm6, %xmm2            # xmm2 = xmm6[1,1,3,3]
1409a6f97: c6 85 35 03 00 00 01        	movb	$0x1, 0x335(%rbp)
1409a6f9e: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1409a6fa5: f3 0f 10 1d 43 77 f8 01     	movss	0x1f87743(%rip), %xmm3  # 0x14292e6f0
1409a6fad: 0f 28 ce                    	movaps	%xmm6, %xmm1
1409a6fb0: e8 8b 05 2f 00              	callq	0x140c97540 <.text+0xc96540>
1409a6fb5: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1409a6fbc: 0f 11 85 44 02 00 00        	movups	%xmm0, 0x244(%rbp)
1409a6fc3: c7 85 40 02 00 00 00 00 00 00       	movl	$0x0, 0x240(%rbp)
1409a6fcd: c6 85 35 03 00 00 01        	movb	$0x1, 0x335(%rbp)
1409a6fd4: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1409a6fdb: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
1409a6fe2: e8 89 b9 12 00              	callq	0x140ad2970 <.text+0xad1970>
1409a6fe7: 84 c0                       	testb	%al, %al
1409a6fe9: 0f 84 e7 01 00 00           	je	0x1409a71d6 <.text+0x9a61d6>
1409a6fef: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1409a6ff6: 48 89 85 28 03 00 00        	movq	%rax, 0x328(%rbp)
1409a6ffd: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1409a7004: 48 89 85 08 03 00 00        	movq	%rax, 0x308(%rbp)
1409a700b: 48 8b bd 00 02 00 00        	movq	0x200(%rbp), %rdi
1409a7012: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
1409a7019: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a7020: 8b 85 20 03 00 00           	movl	0x320(%rbp), %eax
1409a7026: 89 85 48 02 00 00           	movl	%eax, 0x248(%rbp)
1409a702c: 8b 9d 30 03 00 00           	movl	0x330(%rbp), %ebx
1409a7032: 48 8b 95 60 04 00 00        	movq	0x460(%rbp), %rdx
1409a7039: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409a703c: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409a7040: 89 01                       	movl	%eax, (%rcx)
1409a7042: 48 8b 32                    	movq	(%rdx), %rsi
1409a7045: 4c 8b 76 30                 	movq	0x30(%rsi), %r14
1409a7049: 4c 8b 7e 40                 	movq	0x40(%rsi), %r15
1409a704d: 4c 3b 76 20                 	cmpq	0x20(%rsi), %r14
1409a7051: 75 10                       	jne	0x1409a7063 <.text+0x9a6063>
1409a7053: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1409a7057: 48 8d 15 ea 30 29 02        	leaq	0x22930ea(%rip), %rdx   # 0x142c3a148
1409a705e: e8 8d b9 bb ff              	callq	0x1405629f0 <.text+0x5619f0>
1409a7063: 48 8b 46 28                 	movq	0x28(%rsi), %rax
1409a7067: 4b 8d 0c 76                 	leaq	(%r14,%r14,2), %rcx
1409a706b: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409a706f: 48 8b 95 28 03 00 00        	movq	0x328(%rbp), %rdx
1409a7076: 48 89 14 08                 	movq	%rdx, (%rax,%rcx)
1409a707a: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
1409a7081: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
1409a7086: 48 89 7c 08 10              	movq	%rdi, 0x10(%rax,%rcx)
1409a708b: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
1409a7092: 48 89 54 08 18              	movq	%rdx, 0x18(%rax,%rcx)
1409a7097: 8b 95 48 02 00 00           	movl	0x248(%rbp), %edx
1409a709d: 89 54 08 20                 	movl	%edx, 0x20(%rax,%rcx)
1409a70a1: 89 5c 08 24                 	movl	%ebx, 0x24(%rax,%rcx)
1409a70a5: 4c 89 7c 08 28              	movq	%r15, 0x28(%rax,%rcx)
1409a70aa: 49 ff c6                    	incq	%r14
1409a70ad: 4c 89 76 30                 	movq	%r14, 0x30(%rsi)
1409a70b1: 48 ff 46 40                 	incq	0x40(%rsi)
1409a70b5: e9 b5 02 00 00              	jmp	0x1409a736f <.text+0x9a636f>
1409a70ba: 48 8b 8d 40 04 00 00        	movq	0x440(%rbp), %rcx
1409a70c1: 4c 8b 11                    	movq	(%rcx), %r10
1409a70c4: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1409a70ce: 48 89 01                    	movq	%rax, (%rcx)
1409a70d1: 4c 89 d0                    	movq	%r10, %rax
1409a70d4: 48 f7 d8                    	negq	%rax
1409a70d7: 0f 80 92 02 00 00           	jo	0x1409a736f <.text+0x9a636f>
1409a70dd: 4c 8b 8d 40 04 00 00        	movq	0x440(%rbp), %r9
1409a70e4: 4d 8b 59 08                 	movq	0x8(%r9), %r11
1409a70e8: 49 8b 41 10                 	movq	0x10(%r9), %rax
1409a70ec: 48 8b 95 50 04 00 00        	movq	0x450(%rbp), %rdx
1409a70f3: 8b 4a 1c                    	movl	0x1c(%rdx), %ecx
1409a70f6: 48 8b 52 10                 	movq	0x10(%rdx), %rdx
1409a70fa: 89 0a                       	movl	%ecx, (%rdx)
1409a70fc: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1409a7103: 48 83 79 e8 00              	cmpq	$0x0, -0x18(%rcx)
1409a7108: 0f 84 ce 05 00 00           	je	0x1409a76dc <.text+0x9a66dc>
1409a710e: 41 f6 41 18 01              	testb	$0x1, 0x18(%r9)
1409a7113: 0f 84 e3 05 00 00           	je	0x1409a76fc <.text+0x9a66fc>
1409a7119: 48 8b 8d 50 04 00 00        	movq	0x450(%rbp), %rcx
1409a7120: 48 8b 09                    	movq	(%rcx), %rcx
1409a7123: 41 0f 10 41 1c              	movups	0x1c(%r9), %xmm0
1409a7128: 48 8b 95 f8 02 00 00        	movq	0x2f8(%rbp), %rdx
1409a712f: 48 8b 52 e0                 	movq	-0x20(%rdx), %rdx
1409a7133: 44 8b 85 30 03 00 00        	movl	0x330(%rbp), %r8d
1409a713a: 0f 10 09                    	movups	(%rcx), %xmm1
1409a713d: 0f 10 51 10                 	movups	0x10(%rcx), %xmm2
1409a7141: f3 0f 10 1a                 	movss	(%rdx), %xmm3
1409a7145: f3 0f 10 62 08              	movss	0x8(%rdx), %xmm4
1409a714a: 0f 14 dc                    	unpcklps	%xmm4, %xmm3            # xmm3 = xmm3[0],xmm4[0],xmm3[1],xmm4[1]
1409a714d: f3 41 0f 10 61 24           	movss	0x24(%r9), %xmm4
1409a7153: 0f 14 c4                    	unpcklps	%xmm4, %xmm0            # xmm0 = xmm0[0],xmm4[0],xmm0[1],xmm4[1]
1409a7156: 0f 5c d8                    	subps	%xmm0, %xmm3
1409a7159: 0f 29 8d 40 02 00 00        	movaps	%xmm1, 0x240(%rbp)
1409a7160: 48 8b 51 20                 	movq	0x20(%rcx), %rdx
1409a7164: 48 89 95 60 02 00 00        	movq	%rdx, 0x260(%rbp)
1409a716b: 0f 29 95 50 02 00 00        	movaps	%xmm2, 0x250(%rbp)
1409a7172: 4c 89 11                    	movq	%r10, (%rcx)
1409a7175: 4c 89 59 08                 	movq	%r11, 0x8(%rcx)
1409a7179: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
1409a717d: 0f 13 59 18                 	movlps	%xmm3, 0x18(%rcx)
1409a7181: 44 89 41 20                 	movl	%r8d, 0x20(%rcx)
1409a7185: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
1409a718f: 48 39 85 40 02 00 00        	cmpq	%rax, 0x240(%rbp)
1409a7196: 0f 84 d3 01 00 00           	je	0x1409a736f <.text+0x9a636f>
1409a719c: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a71a3: e8 38 c5 ff ff              	callq	0x1409a36e0 <.text+0x9a26e0>
1409a71a8: 48 8d 0d f9 64 11 02        	leaq	0x21164f9(%rip), %rcx   # 0x142abd6a8
1409a71af: 4c 8d 05 32 66 11 02        	leaq	0x2116632(%rip), %r8    # 0x142abd7e8
1409a71b6: ba 40 01 00 00              	movl	$0x140, %edx            # imm = 0x140
1409a71bb: e8 70 22 f3 01              	callq	0x1428d9430 <.text+0x28d8430>
1409a71c0: 80 f9 02                    	cmpb	$0x2, %cl
1409a71c3: 0f 84 5d f9 ff ff           	je	0x1409a6b26 <.text+0x9a5b26>
1409a71c9: f3 0f 10 05 57 5f 02 02     	movss	0x2025f57(%rip), %xmm0  # 0x1429cd128
1409a71d1: e9 58 f9 ff ff              	jmp	0x1409a6b2e <.text+0x9a5b2e>
1409a71d6: 48 8b 85 98 00 00 00        	movq	0x98(%rbp), %rax
1409a71dd: 8b 8d 2c 01 00 00           	movl	0x12c(%rbp), %ecx
1409a71e3: 89 08                       	movl	%ecx, (%rax)
1409a71e5: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
1409a71ec: 48 8b 70 30                 	movq	0x30(%rax), %rsi
1409a71f0: 48 8b 78 40                 	movq	0x40(%rax), %rdi
1409a71f4: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
1409a71f8: 75 1e                       	jne	0x1409a7218 <.text+0x9a6218>
1409a71fa: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
1409a7201: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
1409a7205: c6 85 35 03 00 00 01        	movb	$0x1, 0x335(%rbp)
1409a720c: 48 8d 15 35 2f 29 02        	leaq	0x2292f35(%rip), %rdx   # 0x142c3a148
1409a7213: e8 58 b6 bb ff              	callq	0x140562870 <.text+0x561870>
1409a7218: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
1409a721f: 48 8b 41 28                 	movq	0x28(%rcx), %rax
1409a7223: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
1409a7227: 48 ff c6                    	incq	%rsi
1409a722a: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
1409a722e: 48 ff 41 40                 	incq	0x40(%rcx)
1409a7232: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1409a7239: 8b 8d 28 01 00 00           	movl	0x128(%rbp), %ecx
1409a723f: 89 08                       	movl	%ecx, (%rax)
1409a7241: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
1409a7248: 48 8b 70 30                 	movq	0x30(%rax), %rsi
1409a724c: 48 8b 78 40                 	movq	0x40(%rax), %rdi
1409a7250: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
1409a7254: 75 1e                       	jne	0x1409a7274 <.text+0x9a6274>
1409a7256: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
1409a725d: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
1409a7261: c6 85 35 03 00 00 01        	movb	$0x1, 0x335(%rbp)
1409a7268: 48 8d 15 d9 2e 29 02        	leaq	0x2292ed9(%rip), %rdx   # 0x142c3a148
1409a726f: e8 fc b5 bb ff              	callq	0x140562870 <.text+0x561870>
1409a7274: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
1409a727b: 48 8b 41 28                 	movq	0x28(%rcx), %rax
1409a727f: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
1409a7283: 48 ff c6                    	incq	%rsi
1409a7286: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
1409a728a: 48 ff 41 40                 	incq	0x40(%rcx)
1409a728e: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1409a7295: 48 85 c0                    	testq	%rax, %rax
1409a7298: 0f 84 d1 00 00 00           	je	0x1409a736f <.text+0x9a636f>
1409a729e: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
1409a72a5: 48 c1 e0 02                 	shlq	$0x2, %rax
1409a72a9: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409a72ad: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409a72b3: e8 68 c9 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a72b8: e9 b2 00 00 00              	jmp	0x1409a736f <.text+0x9a636f>
1409a72bd: bb 04 00 00 00              	movl	$0x4, %ebx
1409a72c2: 45 31 ff                    	xorl	%r15d, %r15d
1409a72c5: 48 89 d9                    	movq	%rbx, %rcx
1409a72c8: 4c 89 f2                    	movq	%r14, %rdx
1409a72cb: 49 89 f8                    	movq	%rdi, %r8
1409a72ce: e8 2d c9 f7 01              	callq	0x142923c00 <.text+0x2922c00>
1409a72d3: 48 8b bd 40 04 00 00        	movq	0x440(%rbp), %rdi
1409a72da: 48 8b 07                    	movq	(%rdi), %rax
1409a72dd: 48 89 c1                    	movq	%rax, %rcx
1409a72e0: 48 f7 d9                    	negq	%rcx
1409a72e3: 70 19                       	jo	0x1409a72fe <.text+0x9a62fe>
1409a72e5: 73 17                       	jae	0x1409a72fe <.text+0x9a62fe>
1409a72e7: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
1409a72eb: 48 c1 e0 02                 	shlq	$0x2, %rax
1409a72ef: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409a72f3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409a72f9: e8 22 c9 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a72fe: 4c 89 3f                    	movq	%r15, (%rdi)
1409a7301: 48 89 5f 08                 	movq	%rbx, 0x8(%rdi)
1409a7305: 48 89 77 10                 	movq	%rsi, 0x10(%rdi)
1409a7309: 48 8b 95 f8 02 00 00        	movq	0x2f8(%rbp), %rdx
1409a7310: 48 83 7a e8 00              	cmpq	$0x0, -0x18(%rdx)
1409a7315: 0f 84 72 03 00 00           	je	0x1409a768d <.text+0x9a668d>
1409a731b: 48 8b 42 e0                 	movq	-0x20(%rdx), %rax
1409a731f: 8b 48 08                    	movl	0x8(%rax), %ecx
1409a7322: 89 8d 48 02 00 00           	movl	%ecx, 0x248(%rbp)
1409a7328: 48 8b 00                    	movq	(%rax), %rax
1409a732b: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1409a7332: c7 47 18 01 00 00 00        	movl	$0x1, 0x18(%rdi)
1409a7339: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
1409a7340: 48 89 47 1c                 	movq	%rax, 0x1c(%rdi)
1409a7344: 8b 85 48 02 00 00           	movl	0x248(%rbp), %eax
1409a734a: 89 47 24                    	movl	%eax, 0x24(%rdi)
1409a734d: 48 83 7a e8 00              	cmpq	$0x0, -0x18(%rdx)
1409a7352: 0f 84 45 03 00 00           	je	0x1409a769d <.text+0x9a669d>
1409a7358: 48 8b 42 e0                 	movq	-0x20(%rdx), %rax
1409a735c: f3 0f 10 00                 	movss	(%rax), %xmm0
1409a7360: f3 0f 10 48 08              	movss	0x8(%rax), %xmm1
1409a7365: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1409a7368: 0f 5c f0                    	subps	%xmm0, %xmm6
1409a736b: 0f 13 77 28                 	movlps	%xmm6, 0x28(%rdi)
1409a736f: 80 bd 14 03 00 00 00        	cmpb	$0x0, 0x314(%rbp)
1409a7376: 0f 85 6a 01 00 00           	jne	0x1409a74e6 <.text+0x9a64e6>
1409a737c: c7 44 24 30 43 3f 00 00     	movl	$0x3f43, 0x30(%rsp)     # imm = 0x3F43
1409a7384: c7 44 24 28 ff ff 7f 7f     	movl	$0x7f7fffff, 0x28(%rsp) # imm = 0x7F7FFFFF
1409a738c: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a7393: 48 8b 55 78                 	movq	0x78(%rbp), %rdx
1409a7397: 4d 89 e0                    	movq	%r12, %r8
1409a739a: 45 31 c9                    	xorl	%r9d, %r9d
1409a739d: e8 de 6d f2 ff              	callq	0x1408ce180 <.text+0x8cd180>
1409a73a2: 80 bd 40 02 00 00 11        	cmpb	$0x11, 0x240(%rbp)
1409a73a9: 75 1c                       	jne	0x1409a73c7 <.text+0x9a63c7>
1409a73ab: f3 0f 10 05 cd e5 f7 01     	movss	0x1f7e5cd(%rip), %xmm0  # 0x142925980
1409a73b3: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
1409a73ba: f6 00 01                    	testb	$0x1, (%rax)
1409a73bd: 75 63                       	jne	0x1409a7422 <.text+0x9a6422>
1409a73bf: 0f 57 d2                    	xorps	%xmm2, %xmm2
1409a73c2: 0f 57 c9                    	xorps	%xmm1, %xmm1
1409a73c5: eb 6d                       	jmp	0x1409a7434 <.text+0x9a6434>
1409a73c7: 0f 10 85 58 02 00 00        	movups	0x258(%rbp), %xmm0
1409a73ce: f3 0f 10 8d 5c 02 00 00     	movss	0x25c(%rbp), %xmm1
1409a73d6: f3 41 0f 5c 4c 24 04        	subss	0x4(%r12), %xmm1
1409a73dd: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1409a73e1: f3 0f 10 95 60 02 00 00     	movss	0x260(%rbp), %xmm2
1409a73e9: 0f 16 d0                    	movlhps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0]
1409a73ec: 0f c6 d0 e2                 	shufps	$0xe2, %xmm0, %xmm2     # xmm2 = xmm2[2,0],xmm0[2,3]
1409a73f0: f3 41 0f 10 04 24           	movss	(%r12), %xmm0
1409a73f6: f3 41 0f 10 5c 24 08        	movss	0x8(%r12), %xmm3
1409a73fd: 0f 14 c3                    	unpcklps	%xmm3, %xmm0            # xmm0 = xmm0[0],xmm3[0],xmm0[1],xmm3[1]
1409a7400: 0f 5c d0                    	subps	%xmm0, %xmm2
1409a7403: 0f 59 d2                    	mulps	%xmm2, %xmm2
1409a7406: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1409a740a: f3 0f 16 c2                 	movshdup	%xmm2, %xmm0            # xmm0 = xmm2[1,1,3,3]
1409a740e: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1409a7412: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
1409a7416: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
1409a741d: f6 00 01                    	testb	$0x1, (%rax)
1409a7420: 74 9d                       	je	0x1409a73bf <.text+0x9a63bf>
1409a7422: f3 0f 10 48 08              	movss	0x8(%rax), %xmm1
1409a7427: f3 0f 10 58 0c              	movss	0xc(%rax), %xmm3
1409a742c: f3 0f 10 50 04              	movss	0x4(%rax), %xmm2
1409a7431: 0f 14 d3                    	unpcklps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0],xmm2[1],xmm3[1]
1409a7434: f3 41 0f 5c 4c 24 04        	subss	0x4(%r12), %xmm1
1409a743b: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1409a743f: f3 41 0f 10 1c 24           	movss	(%r12), %xmm3
1409a7445: f3 41 0f 10 64 24 08        	movss	0x8(%r12), %xmm4
1409a744c: 0f 14 dc                    	unpcklps	%xmm4, %xmm3            # xmm3 = xmm3[0],xmm4[0],xmm3[1],xmm4[1]
1409a744f: 0f 5c d3                    	subps	%xmm3, %xmm2
1409a7452: 0f 59 d2                    	mulps	%xmm2, %xmm2
1409a7455: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1409a7459: f3 0f 16 d2                 	movshdup	%xmm2, %xmm2            # xmm2 = xmm2[1,1,3,3]
1409a745d: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1409a7461: 0f 57 c9                    	xorps	%xmm1, %xmm1
1409a7464: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
1409a7468: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
1409a746b: 76 79                       	jbe	0x1409a74e6 <.text+0x9a64e6>
1409a746d: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
1409a7474: 83 38 01                    	cmpl	$0x1, (%rax)
1409a7477: 75 6d                       	jne	0x1409a74e6 <.text+0x9a64e6>
1409a7479: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
1409a7480: 48 8b 70 08                 	movq	0x8(%rax), %rsi
1409a7484: 39 b5 30 03 00 00           	cmpl	%esi, 0x330(%rbp)
1409a748a: 74 5a                       	je	0x1409a74e6 <.text+0x9a64e6>
1409a748c: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1409a7493: 8b 8d 24 01 00 00           	movl	0x124(%rbp), %ecx
1409a7499: 89 08                       	movl	%ecx, (%rax)
1409a749b: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
1409a74a2: 48 8b 7a 30                 	movq	0x30(%rdx), %rdi
1409a74a6: 48 8b 5a 40                 	movq	0x40(%rdx), %rbx
1409a74aa: 48 3b 7a 20                 	cmpq	0x20(%rdx), %rdi
1409a74ae: 75 17                       	jne	0x1409a74c7 <.text+0x9a64c7>
1409a74b0: 48 8d 4a 20                 	leaq	0x20(%rdx), %rcx
1409a74b4: 48 8d 15 8d 2c 29 02        	leaq	0x2292c8d(%rip), %rdx   # 0x142c3a148
1409a74bb: e8 70 b1 bb ff              	callq	0x140562630 <.text+0x561630>
1409a74c0: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
1409a74c7: 48 8b 42 28                 	movq	0x28(%rdx), %rax
1409a74cb: 48 89 f9                    	movq	%rdi, %rcx
1409a74ce: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409a74d2: 48 89 1c 08                 	movq	%rbx, (%rax,%rcx)
1409a74d6: 48 89 74 08 08              	movq	%rsi, 0x8(%rax,%rcx)
1409a74db: 48 ff c7                    	incq	%rdi
1409a74de: 48 89 7a 30                 	movq	%rdi, 0x30(%rdx)
1409a74e2: 48 ff 42 40                 	incq	0x40(%rdx)
1409a74e6: 0f 28 b5 40 03 00 00        	movaps	0x340(%rbp), %xmm6
1409a74ed: 0f 28 bd 50 03 00 00        	movaps	0x350(%rbp), %xmm7
1409a74f4: 44 0f 28 85 60 03 00 00     	movaps	0x360(%rbp), %xmm8
1409a74fc: 44 0f 28 8d 70 03 00 00     	movaps	0x370(%rbp), %xmm9
1409a7504: 44 0f 28 95 80 03 00 00     	movaps	0x380(%rbp), %xmm10
1409a750c: 44 0f 28 9d 90 03 00 00     	movaps	0x390(%rbp), %xmm11
1409a7514: 44 0f 28 a5 a0 03 00 00     	movaps	0x3a0(%rbp), %xmm12
1409a751c: 44 0f 28 ad b0 03 00 00     	movaps	0x3b0(%rbp), %xmm13
1409a7524: 44 0f 28 b5 c0 03 00 00     	movaps	0x3c0(%rbp), %xmm14
1409a752c: 48 81 c4 58 04 00 00        	addq	$0x458, %rsp            # imm = 0x458
1409a7533: 5b                          	popq	%rbx
1409a7534: 5f                          	popq	%rdi
1409a7535: 5e                          	popq	%rsi
1409a7536: 41 5c                       	popq	%r12
1409a7538: 41 5d                       	popq	%r13
1409a753a: 41 5e                       	popq	%r14
1409a753c: 41 5f                       	popq	%r15
1409a753e: 5d                          	popq	%rbp
1409a753f: c3                          	retq
1409a7540: b9 10 00 00 00              	movl	$0x10, %ecx
1409a7545: ba 50 00 00 00              	movl	$0x50, %edx
1409a754a: e8 b1 1a f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a754f: e9 c2 01 00 00              	jmp	0x1409a7716 <.text+0x9a6716>
1409a7554: 45 31 ff                    	xorl	%r15d, %r15d
1409a7557: 4c 8d 05 c2 19 0f 02        	leaq	0x20f19c2(%rip), %r8    # 0x142a98f20
1409a755e: 4c 89 f9                    	movq	%r15, %rcx
1409a7561: 48 89 fa                    	movq	%rdi, %rdx
1409a7564: e8 7a 1a f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a7569: e9 a8 01 00 00              	jmp	0x1409a7716 <.text+0x9a6716>
1409a756e: 4c 8d 05 db 5c 11 02        	leaq	0x2115cdb(%rip), %r8    # 0x142abd250
1409a7575: 31 c9                       	xorl	%ecx, %ecx
1409a7577: 31 d2                       	xorl	%edx, %edx
1409a7579: e8 9a 1f f3 01              	callq	0x1428d9518 <.text+0x28d8518>
1409a757e: b9 08 00 00 00              	movl	$0x8, %ecx
1409a7583: ba 28 00 00 00              	movl	$0x28, %edx
1409a7588: e8 73 1a f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a758d: e9 84 01 00 00              	jmp	0x1409a7716 <.text+0x9a6716>
1409a7592: 4c 89 b5 e8 01 00 00        	movq	%r14, 0x1e8(%rbp)
1409a7599: 45 31 f6                    	xorl	%r14d, %r14d
1409a759c: 4c 8d 05 7d 19 0f 02        	leaq	0x20f197d(%rip), %r8    # 0x142a98f20
1409a75a3: 4c 89 f1                    	movq	%r14, %rcx
1409a75a6: 48 89 fa                    	movq	%rdi, %rdx
1409a75a9: e8 35 1a f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a75ae: e9 63 01 00 00              	jmp	0x1409a7716 <.text+0x9a6716>
1409a75b3: b9 10 00 00 00              	movl	$0x10, %ecx
1409a75b8: ba 30 00 00 00              	movl	$0x30, %edx
1409a75bd: e8 3e 1a f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a75c2: e9 4f 01 00 00              	jmp	0x1409a7716 <.text+0x9a6716>
1409a75c7: 4c 89 bd e8 01 00 00        	movq	%r15, 0x1e8(%rbp)
1409a75ce: 45 31 ff                    	xorl	%r15d, %r15d
1409a75d1: 4c 8d 05 48 19 0f 02        	leaq	0x20f1948(%rip), %r8    # 0x142a98f20
1409a75d8: 4c 89 f9                    	movq	%r15, %rcx
1409a75db: 48 89 fa                    	movq	%rdi, %rdx
1409a75de: e8 00 1a f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a75e3: e9 2e 01 00 00              	jmp	0x1409a7716 <.text+0x9a6716>
1409a75e8: 4c 8d 05 89 15 29 02        	leaq	0x2291589(%rip), %r8    # 0x142c38b78
1409a75ef: 4c 89 f9                    	movq	%r15, %rcx
1409a75f2: 48 89 fa                    	movq	%rdi, %rdx
1409a75f5: e8 e9 19 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a75fa: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a7601: b9 04 00 00 00              	movl	$0x4, %ecx
1409a7606: ba 18 00 00 00              	movl	$0x18, %edx
1409a760b: e8 f0 19 f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a7610: e9 01 01 00 00              	jmp	0x1409a7716 <.text+0x9a6716>
1409a7615: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a761c: e8 bf c0 ff ff              	callq	0x1409a36e0 <.text+0x9a26e0>
1409a7621: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a7628: 48 8d 0d e9 5c 11 02        	leaq	0x2115ce9(%rip), %rcx   # 0x142abd318
1409a762f: 4c 8d 05 52 5e 11 02        	leaq	0x2115e52(%rip), %r8    # 0x142abd488
1409a7636: ba 6d 01 00 00              	movl	$0x16d, %edx            # imm = 0x16D
1409a763b: e8 f0 1d f3 01              	callq	0x1428d9430 <.text+0x28d8430>
1409a7640: e9 d1 00 00 00              	jmp	0x1409a7716 <.text+0x9a6716>
1409a7645: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a764c: e8 8f c0 ff ff              	callq	0x1409a36e0 <.text+0x9a26e0>
1409a7651: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a7658: 48 8d 0d b9 5c 11 02        	leaq	0x2115cb9(%rip), %rcx   # 0x142abd318
1409a765f: 4c 8d 05 5a 5f 11 02        	leaq	0x2115f5a(%rip), %r8    # 0x142abd5c0
1409a7666: ba 6d 01 00 00              	movl	$0x16d, %edx            # imm = 0x16D
1409a766b: e8 c0 1d f3 01              	callq	0x1428d9430 <.text+0x28d8430>
1409a7670: e9 a1 00 00 00              	jmp	0x1409a7716 <.text+0x9a6716>
1409a7675: 48 8d 0d cc 5f 11 02        	leaq	0x2115fcc(%rip), %rcx   # 0x142abd648
1409a767c: e8 8f 1c f3 01              	callq	0x1428d9310 <.text+0x28d8310>
1409a7681: 48 8d 0d d8 5f 11 02        	leaq	0x2115fd8(%rip), %rcx   # 0x142abd660
1409a7688: e8 83 1c f3 01              	callq	0x1428d9310 <.text+0x28d8310>
1409a768d: 4c 8d 05 84 5f 11 02        	leaq	0x2115f84(%rip), %r8    # 0x142abd618
1409a7694: 31 c9                       	xorl	%ecx, %ecx
1409a7696: 31 d2                       	xorl	%edx, %edx
1409a7698: e8 7b 1e f3 01              	callq	0x1428d9518 <.text+0x28d8518>
1409a769d: 4c 8d 05 8c 5f 11 02        	leaq	0x2115f8c(%rip), %r8    # 0x142abd630
1409a76a4: 31 c9                       	xorl	%ecx, %ecx
1409a76a6: 31 d2                       	xorl	%edx, %edx
1409a76a8: e8 6b 1e f3 01              	callq	0x1428d9518 <.text+0x28d8518>
1409a76ad: b9 10 00 00 00              	movl	$0x10, %ecx
1409a76b2: ba 30 00 00 00              	movl	$0x30, %edx
1409a76b7: e8 44 19 f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a76bc: eb 58                       	jmp	0x1409a7716 <.text+0x9a6716>
1409a76be: 45 31 f6                    	xorl	%r14d, %r14d
1409a76c1: 4c 89 bd 28 03 00 00        	movq	%r15, 0x328(%rbp)
1409a76c8: 4c 8d 05 51 18 0f 02        	leaq	0x20f1851(%rip), %r8    # 0x142a98f20
1409a76cf: 4c 89 f1                    	movq	%r14, %rcx
1409a76d2: 48 89 f2                    	movq	%rsi, %rdx
1409a76d5: e8 09 19 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a76da: eb 3a                       	jmp	0x1409a7716 <.text+0x9a6716>
1409a76dc: 4c 89 9d 08 03 00 00        	movq	%r11, 0x308(%rbp)
1409a76e3: 4c 89 95 28 03 00 00        	movq	%r10, 0x328(%rbp)
1409a76ea: 4c 8d 05 87 5f 11 02        	leaq	0x2115f87(%rip), %r8    # 0x142abd678
1409a76f1: 31 c9                       	xorl	%ecx, %ecx
1409a76f3: 31 d2                       	xorl	%edx, %edx
1409a76f5: e8 1e 1e f3 01              	callq	0x1428d9518 <.text+0x28d8518>
1409a76fa: eb 1a                       	jmp	0x1409a7716 <.text+0x9a6716>
1409a76fc: 4c 89 9d 08 03 00 00        	movq	%r11, 0x308(%rbp)
1409a7703: 4c 89 95 28 03 00 00        	movq	%r10, 0x328(%rbp)
1409a770a: 48 8d 0d 7f 5f 11 02        	leaq	0x2115f7f(%rip), %rcx   # 0x142abd690
1409a7711: e8 fa 1b f3 01              	callq	0x1428d9310 <.text+0x28d8310>
1409a7716: 0f 0b                       	ud2
1409a7718: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1409a7720: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a7725: 55                          	pushq	%rbp
1409a7726: 41 57                       	pushq	%r15
1409a7728: 41 56                       	pushq	%r14
1409a772a: 41 55                       	pushq	%r13
1409a772c: 41 54                       	pushq	%r12
1409a772e: 56                          	pushq	%rsi
1409a772f: 57                          	pushq	%rdi
1409a7730: 53                          	pushq	%rbx
1409a7731: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7738: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a773f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a7745: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a774b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a7751: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a7757: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a775d: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7766: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a776f: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7777: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a777f: 48 83 bd 28 03 00 00 00     	cmpq	$0x0, 0x328(%rbp)
1409a7787: 74 21                       	je	0x1409a77aa <.text+0x9a67aa>
1409a7789: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
1409a7790: 48 c1 e0 02                 	shlq	$0x2, %rax
1409a7794: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409a7798: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409a779e: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1409a77a5: e8 76 c4 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a77aa: c6 85 35 03 00 00 00        	movb	$0x0, 0x335(%rbp)
1409a77b1: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a77b9: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a77c1: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a77ca: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a77d3: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a77d9: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a77df: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a77e5: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a77eb: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a77f1: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a77f8: 5b                          	popq	%rbx
1409a77f9: 5f                          	popq	%rdi
1409a77fa: 5e                          	popq	%rsi
1409a77fb: 41 5c                       	popq	%r12
1409a77fd: 41 5d                       	popq	%r13
1409a77ff: 41 5e                       	popq	%r14
1409a7801: 41 5f                       	popq	%r15
1409a7803: 5d                          	popq	%rbp
1409a7804: c3                          	retq
1409a7805: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1409a7810: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a7815: 55                          	pushq	%rbp
1409a7816: 41 57                       	pushq	%r15
1409a7818: 41 56                       	pushq	%r14
1409a781a: 41 55                       	pushq	%r13
1409a781c: 41 54                       	pushq	%r12
1409a781e: 56                          	pushq	%rsi
1409a781f: 57                          	pushq	%rdi
1409a7820: 53                          	pushq	%rbx
1409a7821: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7828: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a782f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a7835: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a783b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a7841: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a7847: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a784d: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7856: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a785f: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7867: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a786f: 48 83 bd 28 03 00 00 00     	cmpq	$0x0, 0x328(%rbp)
1409a7877: 74 21                       	je	0x1409a789a <.text+0x9a689a>
1409a7879: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
1409a7880: 48 c1 e0 02                 	shlq	$0x2, %rax
1409a7884: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409a7888: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409a788e: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1409a7895: e8 86 c3 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a789a: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a78a2: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a78aa: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a78b3: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a78bc: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a78c2: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a78c8: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a78ce: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a78d4: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a78da: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a78e1: 5b                          	popq	%rbx
1409a78e2: 5f                          	popq	%rdi
1409a78e3: 5e                          	popq	%rsi
1409a78e4: 41 5c                       	popq	%r12
1409a78e6: 41 5d                       	popq	%r13
1409a78e8: 41 5e                       	popq	%r14
1409a78ea: 41 5f                       	popq	%r15
1409a78ec: 5d                          	popq	%rbp
1409a78ed: c3                          	retq
1409a78ee: 66 90                       	nop
1409a78f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a78f5: 55                          	pushq	%rbp
1409a78f6: 41 57                       	pushq	%r15
1409a78f8: 41 56                       	pushq	%r14
1409a78fa: 41 55                       	pushq	%r13
1409a78fc: 41 54                       	pushq	%r12
1409a78fe: 56                          	pushq	%rsi
1409a78ff: 57                          	pushq	%rdi
1409a7900: 53                          	pushq	%rbx
1409a7901: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7908: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a790f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a7915: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a791b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a7921: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a7927: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a792d: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7936: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a793f: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7947: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a794f: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
1409a7956: f0                          	lock
1409a7957: 48 ff 08                    	decq	(%rax)
1409a795a: 75 0c                       	jne	0x1409a7968 <.text+0x9a6968>
1409a795c: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a7963: e8 68 80 ca ff              	callq	0x14064f9d0 <.text+0x64e9d0>
1409a7968: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a7970: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a7978: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a7981: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a798a: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a7990: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a7996: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a799c: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a79a2: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a79a8: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a79af: 5b                          	popq	%rbx
1409a79b0: 5f                          	popq	%rdi
1409a79b1: 5e                          	popq	%rsi
1409a79b2: 41 5c                       	popq	%r12
1409a79b4: 41 5d                       	popq	%r13
1409a79b6: 41 5e                       	popq	%r14
1409a79b8: 41 5f                       	popq	%r15
1409a79ba: 5d                          	popq	%rbp
1409a79bb: c3                          	retq
1409a79bc: 0f 1f 40 00                 	nopl	(%rax)
1409a79c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a79c5: 55                          	pushq	%rbp
1409a79c6: 41 57                       	pushq	%r15
1409a79c8: 41 56                       	pushq	%r14
1409a79ca: 41 55                       	pushq	%r13
1409a79cc: 41 54                       	pushq	%r12
1409a79ce: 56                          	pushq	%rsi
1409a79cf: 57                          	pushq	%rdi
1409a79d0: 53                          	pushq	%rbx
1409a79d1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a79d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a79df: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a79e5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a79eb: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a79f1: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a79f7: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a79fd: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7a06: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a7a0f: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7a17: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a7a1f: 48 83 bd 00 03 00 00 00     	cmpq	$0x0, 0x300(%rbp)
1409a7a27: 74 19                       	je	0x1409a7a42 <.text+0x9a6a42>
1409a7a29: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409a7a2f: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
1409a7a36: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
1409a7a3d: e8 de c1 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a7a42: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a7a49: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a7a51: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a7a59: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a7a62: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a7a6b: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a7a71: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a7a77: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a7a7d: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a7a83: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a7a89: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a7a90: 5b                          	popq	%rbx
1409a7a91: 5f                          	popq	%rdi
1409a7a92: 5e                          	popq	%rsi
1409a7a93: 41 5c                       	popq	%r12
1409a7a95: 41 5d                       	popq	%r13
1409a7a97: 41 5e                       	popq	%r14
1409a7a99: 41 5f                       	popq	%r15
1409a7a9b: 5d                          	popq	%rbp
1409a7a9c: c3                          	retq
1409a7a9d: 0f 1f 00                    	nopl	(%rax)
1409a7aa0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a7aa5: 55                          	pushq	%rbp
1409a7aa6: 41 57                       	pushq	%r15
1409a7aa8: 41 56                       	pushq	%r14
1409a7aaa: 41 55                       	pushq	%r13
1409a7aac: 41 54                       	pushq	%r12
1409a7aae: 56                          	pushq	%rsi
1409a7aaf: 57                          	pushq	%rdi
1409a7ab0: 53                          	pushq	%rbx
1409a7ab1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7ab8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a7abf: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a7ac5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a7acb: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a7ad1: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a7ad7: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a7add: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7ae6: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a7aef: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7af7: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a7aff: 80 bd 35 03 00 00 00        	cmpb	$0x0, 0x335(%rbp)
1409a7b06: 74 26                       	je	0x1409a7b2e <.text+0x9a6b2e>
1409a7b08: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1409a7b0f: 48 85 c0                    	testq	%rax, %rax
1409a7b12: 74 1a                       	je	0x1409a7b2e <.text+0x9a6b2e>
1409a7b14: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
1409a7b1b: 48 c1 e0 02                 	shlq	$0x2, %rax
1409a7b1f: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409a7b23: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409a7b29: e8 f2 c0 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a7b2e: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a7b36: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a7b3e: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a7b47: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a7b50: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a7b56: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a7b5c: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a7b62: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a7b68: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a7b6e: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a7b75: 5b                          	popq	%rbx
1409a7b76: 5f                          	popq	%rdi
1409a7b77: 5e                          	popq	%rsi
1409a7b78: 41 5c                       	popq	%r12
1409a7b7a: 41 5d                       	popq	%r13
1409a7b7c: 41 5e                       	popq	%r14
1409a7b7e: 41 5f                       	popq	%r15
1409a7b80: 5d                          	popq	%rbp
1409a7b81: c3                          	retq
1409a7b82: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409a7b90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a7b95: 55                          	pushq	%rbp
1409a7b96: 41 57                       	pushq	%r15
1409a7b98: 41 56                       	pushq	%r14
1409a7b9a: 41 55                       	pushq	%r13
1409a7b9c: 41 54                       	pushq	%r12
1409a7b9e: 56                          	pushq	%rsi
1409a7b9f: 57                          	pushq	%rdi
1409a7ba0: 53                          	pushq	%rbx
1409a7ba1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7ba8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a7baf: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a7bb5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a7bbb: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a7bc1: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a7bc7: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a7bcd: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7bd6: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a7bdf: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7be7: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a7bef: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
1409a7bf6: f0                          	lock
1409a7bf7: 48 ff 08                    	decq	(%rax)
1409a7bfa: 75 0c                       	jne	0x1409a7c08 <.text+0x9a6c08>
1409a7bfc: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a7c03: e8 c8 7d ca ff              	callq	0x14064f9d0 <.text+0x64e9d0>
1409a7c08: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a7c10: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a7c18: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a7c21: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a7c2a: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a7c30: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a7c36: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a7c3c: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a7c42: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a7c48: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a7c4f: 5b                          	popq	%rbx
1409a7c50: 5f                          	popq	%rdi
1409a7c51: 5e                          	popq	%rsi
1409a7c52: 41 5c                       	popq	%r12
1409a7c54: 41 5d                       	popq	%r13
1409a7c56: 41 5e                       	popq	%r14
1409a7c58: 41 5f                       	popq	%r15
1409a7c5a: 5d                          	popq	%rbp
1409a7c5b: c3                          	retq
1409a7c5c: 0f 1f 40 00                 	nopl	(%rax)
1409a7c60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a7c65: 55                          	pushq	%rbp
1409a7c66: 41 57                       	pushq	%r15
1409a7c68: 41 56                       	pushq	%r14
1409a7c6a: 41 55                       	pushq	%r13
1409a7c6c: 41 54                       	pushq	%r12
1409a7c6e: 56                          	pushq	%rsi
1409a7c6f: 57                          	pushq	%rdi
1409a7c70: 53                          	pushq	%rbx
1409a7c71: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7c78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a7c7f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a7c85: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a7c8b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a7c91: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a7c97: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a7c9d: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7ca6: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a7caf: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7cb7: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a7cbf: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
1409a7cc6: f0                          	lock
1409a7cc7: 48 ff 08                    	decq	(%rax)
1409a7cca: 75 0c                       	jne	0x1409a7cd8 <.text+0x9a6cd8>
1409a7ccc: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a7cd3: e8 f8 7c ca ff              	callq	0x14064f9d0 <.text+0x64e9d0>
1409a7cd8: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a7ce0: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a7ce8: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a7cf1: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a7cfa: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a7d00: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a7d06: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a7d0c: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a7d12: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a7d18: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a7d1f: 5b                          	popq	%rbx
1409a7d20: 5f                          	popq	%rdi
1409a7d21: 5e                          	popq	%rsi
1409a7d22: 41 5c                       	popq	%r12
1409a7d24: 41 5d                       	popq	%r13
1409a7d26: 41 5e                       	popq	%r14
1409a7d28: 41 5f                       	popq	%r15
1409a7d2a: 5d                          	popq	%rbp
1409a7d2b: c3                          	retq
1409a7d2c: 0f 1f 40 00                 	nopl	(%rax)
1409a7d30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a7d35: 55                          	pushq	%rbp
1409a7d36: 41 57                       	pushq	%r15
1409a7d38: 41 56                       	pushq	%r14
1409a7d3a: 41 55                       	pushq	%r13
1409a7d3c: 41 54                       	pushq	%r12
1409a7d3e: 56                          	pushq	%rsi
1409a7d3f: 57                          	pushq	%rdi
1409a7d40: 53                          	pushq	%rbx
1409a7d41: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7d48: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a7d4f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a7d55: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a7d5b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a7d61: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a7d67: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a7d6d: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7d76: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a7d7f: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7d87: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a7d8f: 48 83 bd 38 02 00 00 00     	cmpq	$0x0, 0x238(%rbp)
1409a7d97: 74 19                       	je	0x1409a7db2 <.text+0x9a6db2>
1409a7d99: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409a7d9f: 48 8b 8d 00 03 00 00        	movq	0x300(%rbp), %rcx
1409a7da6: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1409a7dad: e8 6e be c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a7db2: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
1409a7db9: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a7dc1: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a7dc9: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a7dd2: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a7ddb: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a7de1: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a7de7: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a7ded: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a7df3: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a7df9: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a7e00: 5b                          	popq	%rbx
1409a7e01: 5f                          	popq	%rdi
1409a7e02: 5e                          	popq	%rsi
1409a7e03: 41 5c                       	popq	%r12
1409a7e05: 41 5d                       	popq	%r13
1409a7e07: 41 5e                       	popq	%r14
1409a7e09: 41 5f                       	popq	%r15
1409a7e0b: 5d                          	popq	%rbp
1409a7e0c: c3                          	retq
1409a7e0d: 0f 1f 00                    	nopl	(%rax)
1409a7e10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a7e15: 55                          	pushq	%rbp
1409a7e16: 41 57                       	pushq	%r15
1409a7e18: 41 56                       	pushq	%r14
1409a7e1a: 41 55                       	pushq	%r13
1409a7e1c: 41 54                       	pushq	%r12
1409a7e1e: 56                          	pushq	%rsi
1409a7e1f: 57                          	pushq	%rdi
1409a7e20: 53                          	pushq	%rbx
1409a7e21: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7e28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a7e2f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a7e35: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a7e3b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a7e41: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a7e47: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a7e4d: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7e56: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a7e5f: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7e67: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a7e6f: 48 83 bd 38 02 00 00 00     	cmpq	$0x0, 0x238(%rbp)
1409a7e77: 74 19                       	je	0x1409a7e92 <.text+0x9a6e92>
1409a7e79: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409a7e7f: 48 8b 8d 00 03 00 00        	movq	0x300(%rbp), %rcx
1409a7e86: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1409a7e8d: e8 8e bd c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a7e92: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1409a7e99: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a7ea1: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a7ea9: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a7eb2: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a7ebb: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a7ec1: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a7ec7: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a7ecd: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a7ed3: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a7ed9: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a7ee0: 5b                          	popq	%rbx
1409a7ee1: 5f                          	popq	%rdi
1409a7ee2: 5e                          	popq	%rsi
1409a7ee3: 41 5c                       	popq	%r12
1409a7ee5: 41 5d                       	popq	%r13
1409a7ee7: 41 5e                       	popq	%r14
1409a7ee9: 41 5f                       	popq	%r15
1409a7eeb: 5d                          	popq	%rbp
1409a7eec: c3                          	retq
1409a7eed: 0f 1f 00                    	nopl	(%rax)
1409a7ef0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a7ef5: 55                          	pushq	%rbp
1409a7ef6: 41 57                       	pushq	%r15
1409a7ef8: 41 56                       	pushq	%r14
1409a7efa: 41 55                       	pushq	%r13
1409a7efc: 41 54                       	pushq	%r12
1409a7efe: 56                          	pushq	%rsi
1409a7eff: 57                          	pushq	%rdi
1409a7f00: 53                          	pushq	%rbx
1409a7f01: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7f08: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a7f0f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a7f15: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a7f1b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a7f21: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a7f27: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a7f2d: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a7f36: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a7f3f: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a7f47: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a7f4f: 80 bd 36 03 00 00 00        	cmpb	$0x0, 0x336(%rbp)
1409a7f56: 74 26                       	je	0x1409a7f7e <.text+0x9a6f7e>
1409a7f58: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1409a7f5f: 48 85 c0                    	testq	%rax, %rax
1409a7f62: 74 1a                       	je	0x1409a7f7e <.text+0x9a6f7e>
1409a7f64: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1409a7f6b: 48 c1 e0 02                 	shlq	$0x2, %rax
1409a7f6f: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1409a7f73: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409a7f79: e8 a2 bc c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a7f7e: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a7f86: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a7f8e: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a7f97: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a7fa0: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a7fa6: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a7fac: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a7fb2: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a7fb8: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a7fbe: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a7fc5: 5b                          	popq	%rbx
1409a7fc6: 5f                          	popq	%rdi
1409a7fc7: 5e                          	popq	%rsi
1409a7fc8: 41 5c                       	popq	%r12
1409a7fca: 41 5d                       	popq	%r13
1409a7fcc: 41 5e                       	popq	%r14
1409a7fce: 41 5f                       	popq	%r15
1409a7fd0: 5d                          	popq	%rbp
1409a7fd1: c3                          	retq
1409a7fd2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409a7fe0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a7fe5: 55                          	pushq	%rbp
1409a7fe6: 41 57                       	pushq	%r15
1409a7fe8: 41 56                       	pushq	%r14
1409a7fea: 41 55                       	pushq	%r13
1409a7fec: 41 54                       	pushq	%r12
1409a7fee: 56                          	pushq	%rsi
1409a7fef: 57                          	pushq	%rdi
1409a7ff0: 53                          	pushq	%rbx
1409a7ff1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a7ff8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a7fff: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a8005: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a800b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a8011: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a8017: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a801d: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a8026: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a802f: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a8037: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a803f: 80 bd 37 03 00 00 00        	cmpb	$0x0, 0x337(%rbp)
1409a8046: 75 54                       	jne	0x1409a809c <.text+0x9a709c>
1409a8048: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a8050: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a8058: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a8061: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a806a: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a8070: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a8076: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a807c: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a8082: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a8088: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a808f: 5b                          	popq	%rbx
1409a8090: 5f                          	popq	%rdi
1409a8091: 5e                          	popq	%rsi
1409a8092: 41 5c                       	popq	%r12
1409a8094: 41 5d                       	popq	%r13
1409a8096: 41 5e                       	popq	%r14
1409a8098: 41 5f                       	popq	%r15
1409a809a: 5d                          	popq	%rbp
1409a809b: c3                          	retq
1409a809c: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1409a80a3: e8 28 75 fa ff              	callq	0x14094f5d0 <.text+0x94e5d0>
1409a80a8: eb 9e                       	jmp	0x1409a8048 <.text+0x9a7048>
1409a80aa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1409a80b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a80b5: 55                          	pushq	%rbp
1409a80b6: 41 57                       	pushq	%r15
1409a80b8: 41 56                       	pushq	%r14
1409a80ba: 41 55                       	pushq	%r13
1409a80bc: 41 54                       	pushq	%r12
1409a80be: 56                          	pushq	%rsi
1409a80bf: 57                          	pushq	%rdi
1409a80c0: 53                          	pushq	%rbx
1409a80c1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a80c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a80cf: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a80d5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a80db: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a80e1: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a80e7: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a80ed: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a80f6: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a80ff: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a8107: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a810f: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1409a8116: f0                          	lock
1409a8117: 48 ff 08                    	decq	(%rax)
1409a811a: 75 0c                       	jne	0x1409a8128 <.text+0x9a7128>
1409a811c: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1409a8123: e8 a8 78 ca ff              	callq	0x14064f9d0 <.text+0x64e9d0>
1409a8128: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a8130: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a8138: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a8141: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a814a: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a8150: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a8156: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a815c: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a8162: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a8168: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a816f: 5b                          	popq	%rbx
1409a8170: 5f                          	popq	%rdi
1409a8171: 5e                          	popq	%rsi
1409a8172: 41 5c                       	popq	%r12
1409a8174: 41 5d                       	popq	%r13
1409a8176: 41 5e                       	popq	%r14
1409a8178: 41 5f                       	popq	%r15
1409a817a: 5d                          	popq	%rbp
1409a817b: c3                          	retq
1409a817c: 0f 1f 40 00                 	nopl	(%rax)
1409a8180: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a8185: 55                          	pushq	%rbp
1409a8186: 41 57                       	pushq	%r15
1409a8188: 41 56                       	pushq	%r14
1409a818a: 41 55                       	pushq	%r13
1409a818c: 41 54                       	pushq	%r12
1409a818e: 56                          	pushq	%rsi
1409a818f: 57                          	pushq	%rdi
1409a8190: 53                          	pushq	%rbx
1409a8191: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a8198: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a819f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a81a5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a81ab: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a81b1: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a81b7: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a81bd: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a81c6: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a81cf: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a81d7: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a81df: 48 83 bd 08 03 00 00 00     	cmpq	$0x0, 0x308(%rbp)
1409a81e7: 74 19                       	je	0x1409a8202 <.text+0x9a7202>
1409a81e9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409a81ef: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
1409a81f6: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
1409a81fd: e8 1e ba c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a8202: c6 85 34 03 00 00 00        	movb	$0x0, 0x334(%rbp)
1409a8209: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a8211: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a8219: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a8222: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a822b: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a8231: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a8237: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a823d: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a8243: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a8249: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a8250: 5b                          	popq	%rbx
1409a8251: 5f                          	popq	%rdi
1409a8252: 5e                          	popq	%rsi
1409a8253: 41 5c                       	popq	%r12
1409a8255: 41 5d                       	popq	%r13
1409a8257: 41 5e                       	popq	%r14
1409a8259: 41 5f                       	popq	%r15
1409a825b: 5d                          	popq	%rbp
1409a825c: c3                          	retq
1409a825d: 0f 1f 00                    	nopl	(%rax)
1409a8260: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a8265: 55                          	pushq	%rbp
1409a8266: 41 57                       	pushq	%r15
1409a8268: 41 56                       	pushq	%r14
1409a826a: 41 55                       	pushq	%r13
1409a826c: 41 54                       	pushq	%r12
1409a826e: 56                          	pushq	%rsi
1409a826f: 57                          	pushq	%rdi
1409a8270: 53                          	pushq	%rbx
1409a8271: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1409a8278: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a827f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1409a8285: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1409a828b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1409a8291: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1409a8297: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1409a829d: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1409a82a6: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1409a82af: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1409a82b7: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1409a82bf: 80 bd 34 03 00 00 00        	cmpb	$0x0, 0x334(%rbp)
1409a82c6: 74 26                       	je	0x1409a82ee <.text+0x9a72ee>
1409a82c8: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1409a82cf: 48 85 c0                    	testq	%rax, %rax
1409a82d2: 74 1a                       	je	0x1409a82ee <.text+0x9a72ee>
1409a82d4: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1409a82db: 48 c1 e0 02                 	shlq	$0x2, %rax
1409a82df: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1409a82e3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409a82e9: e8 32 b9 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a82ee: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1409a82f6: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1409a82fe: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1409a8307: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1409a8310: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1409a8316: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1409a831c: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1409a8322: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1409a8328: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1409a832e: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1409a8335: 5b                          	popq	%rbx
1409a8336: 5f                          	popq	%rdi
1409a8337: 5e                          	popq	%rsi
1409a8338: 41 5c                       	popq	%r12
1409a833a: 41 5d                       	popq	%r13
1409a833c: 41 5e                       	popq	%r14
1409a833e: 41 5f                       	popq	%r15
1409a8340: 5d                          	popq	%rbp
1409a8341: c3                          	retq
