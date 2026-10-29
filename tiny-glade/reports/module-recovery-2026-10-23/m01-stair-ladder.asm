
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407db3e0: 41 57                       	pushq	%r15
1407db3e2: 41 56                       	pushq	%r14
1407db3e4: 41 55                       	pushq	%r13
1407db3e6: 41 54                       	pushq	%r12
1407db3e8: 56                          	pushq	%rsi
1407db3e9: 57                          	pushq	%rdi
1407db3ea: 55                          	pushq	%rbp
1407db3eb: 53                          	pushq	%rbx
1407db3ec: 48 81 ec f8 02 00 00        	subq	$0x2f8, %rsp            # imm = 0x2F8
1407db3f3: 44 0f 29 bc 24 e0 02 00 00  	movaps	%xmm15, 0x2e0(%rsp)
1407db3fc: 44 0f 29 b4 24 d0 02 00 00  	movaps	%xmm14, 0x2d0(%rsp)
1407db405: 44 0f 29 ac 24 c0 02 00 00  	movaps	%xmm13, 0x2c0(%rsp)
1407db40e: 44 0f 29 a4 24 b0 02 00 00  	movaps	%xmm12, 0x2b0(%rsp)
1407db417: 44 0f 29 9c 24 a0 02 00 00  	movaps	%xmm11, 0x2a0(%rsp)
1407db420: 44 0f 29 94 24 90 02 00 00  	movaps	%xmm10, 0x290(%rsp)
1407db429: 44 0f 29 8c 24 80 02 00 00  	movaps	%xmm9, 0x280(%rsp)
1407db432: 44 0f 29 84 24 70 02 00 00  	movaps	%xmm8, 0x270(%rsp)
1407db43b: 0f 29 bc 24 60 02 00 00     	movaps	%xmm7, 0x260(%rsp)
1407db443: 0f 29 b4 24 50 02 00 00     	movaps	%xmm6, 0x250(%rsp)
1407db44b: 48 8b 84 24 60 03 00 00     	movq	0x360(%rsp), %rax
1407db453: 48 85 c0                    	testq	%rax, %rax
1407db456: 0f 84 d6 0c 00 00           	je	0x1407dc132 <.text+0x7db132>
1407db45c: 49 8d 04 81                 	leaq	(%r9,%rax,4), %rax
1407db460: 48 83 c0 fc                 	addq	$-0x4, %rax
1407db464: 0f 84 d4 0c 00 00           	je	0x1407dc13e <.text+0x7db13e>
1407db46a: 4d 85 c0                    	testq	%r8, %r8
1407db46d: 0f 84 d7 0c 00 00           	je	0x1407dc14a <.text+0x7db14a>
1407db473: 48 89 ce                    	movq	%rcx, %rsi
1407db476: 4b 8d 0c 40                 	leaq	(%r8,%r8,2), %rcx
1407db47a: 48 8d 0c 8a                 	leaq	(%rdx,%rcx,4), %rcx
1407db47e: 48 83 f9 0c                 	cmpq	$0xc, %rcx
1407db482: 0f 84 ce 0c 00 00           	je	0x1407dc156 <.text+0x7db156>
1407db488: 48 8b ac 24 78 03 00 00     	movq	0x378(%rsp), %rbp
1407db490: 48 8b 9c 24 68 03 00 00     	movq	0x368(%rsp), %rbx
1407db498: f3 41 0f 10 31              	movss	(%r9), %xmm6
1407db49d: f3 0f 58 35 0f c3 2a 02     	addss	0x22ac30f(%rip), %xmm6  # 0x142a877b4
1407db4a5: f3 44 0f 10 18              	movss	(%rax), %xmm11
1407db4aa: f3 44 0f 58 1d 01 c3 2a 02  	addss	0x22ac301(%rip), %xmm11 # 0x142a877b4
1407db4b3: f2 0f 10 3a                 	movsd	(%rdx), %xmm7
1407db4b7: f3 44 0f 10 42 08           	movss	0x8(%rdx), %xmm8
1407db4bd: f2 44 0f 10 49 f4           	movsd	-0xc(%rcx), %xmm9
1407db4c3: f3 44 0f 10 51 fc           	movss	-0x4(%rcx), %xmm10
1407db4c9: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1407db4cd: 0f 5c c7                    	subps	%xmm7, %xmm0
1407db4d0: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407db4d4: f3 41 0f 5c c8              	subss	%xmm8, %xmm1
1407db4d9: 0f 28 15 10 a4 14 02        	movaps	0x214a410(%rip), %xmm2  # 0x1429258f0
1407db4e0: 0f 57 d1                    	xorps	%xmm1, %xmm2
1407db4e3: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407db4e7: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
1407db4ea: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1407db4ee: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407db4f2: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
1407db4f6: f3 0f 10 0d 06 a4 14 02     	movss	0x214a406(%rip), %xmm1  # 0x142925904
1407db4fe: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407db502: f3 0f 12 c1                 	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
1407db506: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407db509: 0f 13 44 24 50              	movlps	%xmm0, 0x50(%rsp)
1407db50e: 48 8d 8c 24 90 00 00 00     	leaq	0x90(%rsp), %rcx
1407db516: 48 8d 54 24 50              	leaq	0x50(%rsp), %rdx
1407db51b: e8 a0 18 4c 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407db520: 44 0f b6 3b                 	movzbl	(%rbx), %r15d
1407db524: 44 0f 29 94 24 a0 00 00 00  	movaps	%xmm10, 0xa0(%rsp)
1407db52d: 45 0f 16 d1                 	movlhps	%xmm9, %xmm10           # xmm10 = xmm10[0],xmm9[0]
1407db531: 45 0f c6 d1 e2              	shufps	$0xe2, %xmm9, %xmm10    # xmm10 = xmm10[2,0],xmm9[2,3]
1407db536: 44 0f 29 94 24 90 01 00 00  	movaps	%xmm10, 0x190(%rsp)
1407db53f: f3 41 0f 12 c3              	movsldup	%xmm11, %xmm0           # xmm0 = xmm11[0,0,2,2]
1407db544: 0f 29 84 24 80 01 00 00     	movaps	%xmm0, 0x180(%rsp)
1407db54c: 44 0f 29 84 24 c0 00 00 00  	movaps	%xmm8, 0xc0(%rsp)
1407db555: 44 0f 16 c7                 	movlhps	%xmm7, %xmm8            # xmm8 = xmm8[0],xmm7[0]
1407db559: 44 0f c6 c7 e2              	shufps	$0xe2, %xmm7, %xmm8     # xmm8 = xmm8[2,0],xmm7[2,3]
1407db55e: 44 0f 29 84 24 70 01 00 00  	movaps	%xmm8, 0x170(%rsp)
1407db567: f3 0f 12 c6                 	movsldup	%xmm6, %xmm0            # xmm0 = xmm6[0,0,2,2]
1407db56b: 0f 29 84 24 60 01 00 00     	movaps	%xmm0, 0x160(%rsp)
1407db573: 44 0f 29 8c 24 b0 00 00 00  	movaps	%xmm9, 0xb0(%rsp)
1407db57c: 44 0f c6 cf 11              	shufps	$0x11, %xmm7, %xmm9     # xmm9 = xmm9[1,0],xmm7[1,0]
1407db581: 0f 29 bc 24 d0 00 00 00     	movaps	%xmm7, 0xd0(%rsp)
1407db589: 44 0f c6 cf e2              	shufps	$0xe2, %xmm7, %xmm9     # xmm9 = xmm9[2,0],xmm7[2,3]
1407db58e: 44 0f 29 8c 24 50 01 00 00  	movaps	%xmm9, 0x150(%rsp)
1407db597: 0f 29 b4 24 30 01 00 00     	movaps	%xmm6, 0x130(%rsp)
1407db59f: 44 0f 29 9c 24 20 01 00 00  	movaps	%xmm11, 0x120(%rsp)
1407db5a8: 41 0f 14 f3                 	unpcklps	%xmm11, %xmm6           # xmm6 = xmm6[0],xmm11[0],xmm6[1],xmm11[1]
1407db5ac: 0f 29 b4 24 40 01 00 00     	movaps	%xmm6, 0x140(%rsp)
1407db5b4: 41 b4 01                    	movb	$0x1, %r12b
1407db5b7: 4c 8d 6c 24 40              	leaq	0x40(%rsp), %r13
1407db5bc: 48 bb 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rbx # imm = 0xA0761D6478BD642F
1407db5c6: 49 be db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %r14 # imm = 0xE7037ED1A0B428DB
1407db5d0: 44 89 bc 24 9c 00 00 00     	movl	%r15d, 0x9c(%rsp)
1407db5d8: e9 8a 00 00 00              	jmp	0x1407db667 <.text+0x7da667>
1407db5dd: 0f 1f 00                    	nopl	(%rax)
1407db5e0: 89 d0                       	movl	%edx, %eax
1407db5e2: 48 69 c0 c8 00 00 00        	imulq	$0xc8, %rax, %rax
1407db5e9: 48 c1 e8 20                 	shrq	$0x20, %rax
1407db5ed: 83 c0 9c                    	addl	$-0x64, %eax
1407db5f0: 0f 10 84 24 f0 01 00 00     	movups	0x1f0(%rsp), %xmm0
1407db5f8: 0f 10 8c 24 00 02 00 00     	movups	0x200(%rsp), %xmm1
1407db600: 0f 10 94 24 10 02 00 00     	movups	0x210(%rsp), %xmm2
1407db608: 0f 29 54 24 70              	movaps	%xmm2, 0x70(%rsp)
1407db60d: 0f 29 4c 24 60              	movaps	%xmm1, 0x60(%rsp)
1407db612: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
1407db617: c7 84 24 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rsp)
1407db622: 89 84 24 84 00 00 00        	movl	%eax, 0x84(%rsp)
1407db629: 8b 84 24 70 03 00 00        	movl	0x370(%rsp), %eax
1407db630: 89 84 24 88 00 00 00        	movl	%eax, 0x88(%rsp)
1407db637: c7 84 24 8c 00 00 00 18 00 00 00    	movl	$0x18, 0x8c(%rsp)
1407db642: 48 89 f1                    	movq	%rsi, %rcx
1407db645: 48 8d 54 24 50              	leaq	0x50(%rsp), %rdx
1407db64a: e8 91 1a ab 00              	callq	0x14128d0e0 <.text+0x128c0e0>
1407db64f: 44 8b bc 24 9c 00 00 00     	movl	0x9c(%rsp), %r15d
1407db657: 41 f6 c4 01                 	testb	$0x1, %r12b
1407db65b: 41 bc 00 00 00 00           	movl	$0x0, %r12d
1407db661: 0f 84 1f 06 00 00           	je	0x1407dbc86 <.text+0x7dac86>
1407db667: f3 0f 10 05 01 a2 14 02     	movss	0x214a201(%rip), %xmm0  # 0x142925870
1407db66f: 41 f6 c4 01                 	testb	$0x1, %r12b
1407db673: 75 08                       	jne	0x1407db67d <.text+0x7da67d>
1407db675: f3 0f 10 05 6b ea 17 02     	movss	0x217ea6b(%rip), %xmm0  # 0x14295a0e8
1407db67d: f3 44 0f 10 9c 24 90 00 00 00       	movss	0x90(%rsp), %xmm11
1407db687: f3 0f 10 8c 24 94 00 00 00  	movss	0x94(%rsp), %xmm1
1407db690: f3 0f 10 94 24 98 00 00 00  	movss	0x98(%rsp), %xmm2
1407db699: 44 0f 14 da                 	unpcklps	%xmm2, %xmm11           # xmm11 = xmm11[0],xmm2[0],xmm11[1],xmm2[1]
1407db69d: f3 0f 12 f1                 	movsldup	%xmm1, %xmm6            # xmm6 = xmm1[0,0,2,2]
1407db6a1: 0f 59 b4 24 40 01 00 00     	mulps	0x140(%rsp), %xmm6
1407db6a9: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1407db6ad: 0f 59 f0                    	mulps	%xmm0, %xmm6
1407db6b0: 0f 58 b4 24 50 01 00 00     	addps	0x150(%rsp), %xmm6
1407db6b8: f3 44 0f 16 ee              	movshdup	%xmm6, %xmm13           # xmm13 = xmm6[1,1,3,3]
1407db6bd: 44 0f 28 b4 24 60 01 00 00  	movaps	0x160(%rsp), %xmm14
1407db6c6: 45 0f 59 f3                 	mulps	%xmm11, %xmm14
1407db6ca: 44 0f 59 f0                 	mulps	%xmm0, %xmm14
1407db6ce: 44 0f 58 b4 24 70 01 00 00  	addps	0x170(%rsp), %xmm14
1407db6d7: 44 0f 59 9c 24 80 01 00 00  	mulps	0x180(%rsp), %xmm11
1407db6e0: 44 0f 59 d8                 	mulps	%xmm0, %xmm11
1407db6e4: 44 0f 58 9c 24 90 01 00 00  	addps	0x190(%rsp), %xmm11
1407db6ed: 41 0f 28 fb                 	movaps	%xmm11, %xmm7
1407db6f1: 41 0f 5c fe                 	subps	%xmm14, %xmm7
1407db6f5: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
1407db6f9: 44 0f 59 c7                 	mulps	%xmm7, %xmm8
1407db6fd: f3 45 0f 16 d0              	movshdup	%xmm8, %xmm10           # xmm10 = xmm8[1,1,3,3]
1407db702: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1407db706: f3 41 0f 58 c2              	addss	%xmm10, %xmm0
1407db70b: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
1407db70f: f3 44 0f 10 0d ec a1 14 02  	movss	0x214a1ec(%rip), %xmm9  # 0x142925904
1407db718: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1407db71c: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407db720: f3 41 0f 16 c6              	movshdup	%xmm14, %xmm0           # xmm0 = xmm14[1,1,3,3]
1407db725: 0f 29 84 24 b0 01 00 00     	movaps	%xmm0, 0x1b0(%rsp)
1407db72d: f3 0f 12 c1                 	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
1407db731: 0f 59 c7                    	mulps	%xmm7, %xmm0
1407db734: 0f 29 84 24 a0 01 00 00     	movaps	%xmm0, 0x1a0(%rsp)
1407db73c: 0f 13 44 24 50              	movlps	%xmm0, 0x50(%rsp)
1407db741: 48 8d 8c 24 fc 00 00 00     	leaq	0xfc(%rsp), %rcx
1407db749: 48 8d 7c 24 50              	leaq	0x50(%rsp), %rdi
1407db74e: 48 89 fa                    	movq	%rdi, %rdx
1407db751: e8 6a 16 4c 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407db756: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1407db75a: f3 0f 5c ce                 	subss	%xmm6, %xmm1
1407db75e: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407db761: f3 0f 10 f9                 	movss	%xmm1, %xmm7            # xmm7 = xmm1[0],xmm7[1,2,3]
1407db765: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407db769: f3 41 0f 58 c8              	addss	%xmm8, %xmm1
1407db76e: f3 41 0f 58 ca              	addss	%xmm10, %xmm1
1407db773: f3 0f 51 c9                 	sqrtss	%xmm1, %xmm1
1407db777: 41 0f 28 d9                 	movaps	%xmm9, %xmm3
1407db77b: f3 0f 5e d9                 	divss	%xmm1, %xmm3
1407db77f: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
1407db783: f3 44 0f 10 8c 24 fc 00 00 00       	movss	0xfc(%rsp), %xmm9
1407db78d: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407db790: f3 41 0f 5c cd              	subss	%xmm13, %xmm1
1407db795: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407db799: 45 0f 28 d6                 	movaps	%xmm14, %xmm10
1407db79d: 45 0f 5c d3                 	subps	%xmm11, %xmm10
1407db7a1: 45 0f 59 d2                 	mulps	%xmm10, %xmm10
1407db7a5: f3 41 0f 58 ca              	addss	%xmm10, %xmm1
1407db7aa: f3 45 0f 16 c2              	movshdup	%xmm10, %xmm8           # xmm8 = xmm10[1,1,3,3]
1407db7af: f3 41 0f 58 c8              	addss	%xmm8, %xmm1
1407db7b4: 0f 57 d2                    	xorps	%xmm2, %xmm2
1407db7b7: f3 0f 51 d1                 	sqrtss	%xmm1, %xmm2
1407db7bb: f3 0f 12 eb                 	movsldup	%xmm3, %xmm5            # xmm5 = xmm3[0,0,2,2]
1407db7bf: 0f 59 ef                    	mulps	%xmm7, %xmm5
1407db7c2: f2 44 0f 10 bc 24 00 01 00 00       	movsd	0x100(%rsp), %xmm15
1407db7cc: 41 0f 28 e1                 	movaps	%xmm9, %xmm4
1407db7d0: 41 0f c6 e7 d4              	shufps	$0xd4, %xmm15, %xmm4    # xmm4 = xmm4[0,1],xmm15[1,3]
1407db7d5: 41 0f c6 e7 f2              	shufps	$0xf2, %xmm15, %xmm4    # xmm4 = xmm4[2,0],xmm15[3,3]
1407db7da: 0f 59 e5                    	mulps	%xmm5, %xmm4
1407db7dd: 0f 28 d8                    	movaps	%xmm0, %xmm3
1407db7e0: 0f c6 dd d4                 	shufps	$0xd4, %xmm5, %xmm3     # xmm3 = xmm3[0,1],xmm5[1,3]
1407db7e4: 0f c6 dd f2                 	shufps	$0xf2, %xmm5, %xmm3     # xmm3 = xmm3[2,0],xmm5[3,3]
1407db7e8: 41 0f 28 ff                 	movaps	%xmm15, %xmm7
1407db7ec: 0f 59 fb                    	mulps	%xmm3, %xmm7
1407db7ef: 0f 5c e7                    	subps	%xmm7, %xmm4
1407db7f2: f3 44 0f 59 f8              	mulss	%xmm0, %xmm15
1407db7f7: f3 44 0f 59 cd              	mulss	%xmm5, %xmm9
1407db7fc: f3 45 0f 5c f9              	subss	%xmm9, %xmm15
1407db801: f3 0f 16 fc                 	movshdup	%xmm4, %xmm7            # xmm7 = xmm4[1,1,3,3]
1407db805: 45 0f 28 e7                 	movaps	%xmm15, %xmm12
1407db809: f3 44 0f 12 ca              	movsldup	%xmm2, %xmm9            # xmm9 = xmm2[0,0,2,2]
1407db80e: 44 0f 59 cd                 	mulps	%xmm5, %xmm9
1407db812: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
1407db816: 44 0f 14 fc                 	unpcklps	%xmm4, %xmm15           # xmm15 = xmm15[0],xmm4[0],xmm15[1],xmm4[1]
1407db81a: 44 0f 59 fd                 	mulps	%xmm5, %xmm15
1407db81e: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
1407db822: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
1407db826: f3 0f 5c ef                 	subss	%xmm7, %xmm5
1407db82a: 0f c6 cc d4                 	shufps	$0xd4, %xmm4, %xmm1     # xmm1 = xmm1[0,1],xmm4[1,3]
1407db82e: 0f c6 cc e2                 	shufps	$0xe2, %xmm4, %xmm1     # xmm1 = xmm1[2,0],xmm4[2,3]
1407db832: 0f 28 3d 87 bf 2a 02        	movaps	0x22abf87(%rip), %xmm7  # 0x142a877c0
1407db839: 0f 59 e7                    	mulps	%xmm7, %xmm4
1407db83c: 0f 13 a4 24 e0 00 00 00     	movlps	%xmm4, 0xe0(%rsp)
1407db844: f3 0f 10 25 84 bf 2a 02     	movss	0x22abf84(%rip), %xmm4  # 0x142a877d0
1407db84c: f3 44 0f 59 e4              	mulss	%xmm4, %xmm12
1407db851: f3 44 0f 11 a4 24 e8 00 00 00       	movss	%xmm12, 0xe8(%rsp)
1407db85b: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
1407db85f: f3 0f 11 44 24 30           	movss	%xmm0, 0x30(%rsp)
1407db865: 44 0f 13 4c 24 34           	movlps	%xmm9, 0x34(%rsp)
1407db86b: 0f 59 cb                    	mulps	%xmm3, %xmm1
1407db86e: 41 0f 5c cf                 	subps	%xmm15, %xmm1
1407db872: 0f 59 cf                    	mulps	%xmm7, %xmm1
1407db875: 0f 13 4c 24 40              	movlps	%xmm1, 0x40(%rsp)
1407db87a: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
1407db87e: f3 0f 11 6c 24 48           	movss	%xmm5, 0x48(%rsp)
1407db884: f3 44 0f 11 74 24 50        	movss	%xmm14, 0x50(%rsp)
1407db88b: f3 0f 11 74 24 54           	movss	%xmm6, 0x54(%rsp)
1407db891: 0f 28 84 24 b0 01 00 00     	movaps	0x1b0(%rsp), %xmm0
1407db899: f3 0f 11 44 24 58           	movss	%xmm0, 0x58(%rsp)
1407db89f: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
1407db8a4: 48 8d 8c 24 c0 01 00 00     	leaq	0x1c0(%rsp), %rcx
1407db8ac: 48 8d 94 24 e0 00 00 00     	leaq	0xe0(%rsp), %rdx
1407db8b4: 4c 8d 44 24 30              	leaq	0x30(%rsp), %r8
1407db8b9: 4d 89 e9                    	movq	%r13, %r9
1407db8bc: e8 3f 1c 0c 00              	callq	0x14089d500 <.text+0x89c500>
1407db8c1: 4c 8b 45 00                 	movq	(%rbp), %r8
1407db8c5: 49 8d 0c 18                 	leaq	(%r8,%rbx), %rcx
1407db8c9: 48 89 4d 00                 	movq	%rcx, (%rbp)
1407db8cd: 48 89 c8                    	movq	%rcx, %rax
1407db8d0: 4c 31 f0                    	xorq	%r14, %rax
1407db8d3: 48 f7 e1                    	mulq	%rcx
1407db8d6: 48 31 c2                    	xorq	%rax, %rdx
1407db8d9: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
1407db8df: 83 f8 5f                    	cmpl	$0x5f, %eax
1407db8e2: 77 3d                       	ja	0x1407db921 <.text+0x7da921>
1407db8e4: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1407db8ee: 49 01 c0                    	addq	%rax, %r8
1407db8f1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407db900: 48 01 d9                    	addq	%rbx, %rcx
1407db903: 4c 89 c0                    	movq	%r8, %rax
1407db906: 4c 31 f0                    	xorq	%r14, %rax
1407db909: 49 f7 e0                    	mulq	%r8
1407db90c: 48 31 c2                    	xorq	%rax, %rdx
1407db90f: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
1407db915: 49 01 d8                    	addq	%rbx, %r8
1407db918: 83 f8 60                    	cmpl	$0x60, %eax
1407db91b: 72 e3                       	jb	0x1407db900 <.text+0x7da900>
1407db91d: 48 89 4d 00                 	movq	%rcx, (%rbp)
1407db921: 89 d0                       	movl	%edx, %eax
1407db923: 48 69 c0 c8 00 00 00        	imulq	$0xc8, %rax, %rax
1407db92a: 48 c1 e8 20                 	shrq	$0x20, %rax
1407db92e: 83 c0 9c                    	addl	$-0x64, %eax
1407db931: 0f 10 84 24 c0 01 00 00     	movups	0x1c0(%rsp), %xmm0
1407db939: 0f 10 8c 24 d0 01 00 00     	movups	0x1d0(%rsp), %xmm1
1407db941: 0f 10 94 24 e0 01 00 00     	movups	0x1e0(%rsp), %xmm2
1407db949: 0f 29 54 24 70              	movaps	%xmm2, 0x70(%rsp)
1407db94e: 0f 29 4c 24 60              	movaps	%xmm1, 0x60(%rsp)
1407db953: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
1407db958: c7 84 24 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rsp)
1407db963: 89 84 24 84 00 00 00        	movl	%eax, 0x84(%rsp)
1407db96a: 8b 84 24 70 03 00 00        	movl	0x370(%rsp), %eax
1407db971: 89 84 24 88 00 00 00        	movl	%eax, 0x88(%rsp)
1407db978: c7 84 24 8c 00 00 00 18 00 00 00    	movl	$0x18, 0x8c(%rsp)
1407db983: 48 89 f1                    	movq	%rsi, %rcx
1407db986: 48 8d 54 24 50              	leaq	0x50(%rsp), %rdx
1407db98b: e8 50 17 ab 00              	callq	0x14128d0e0 <.text+0x128c0e0>
1407db990: 41 83 ff 04                 	cmpl	$0x4, %r15d
1407db994: 0f 87 bd fc ff ff           	ja	0x1407db657 <.text+0x7da657>
1407db99a: b8 16 00 00 00              	movl	$0x16, %eax
1407db99f: 44 0f a3 f8                 	btl	%r15d, %eax
1407db9a3: 0f 83 ae fc ff ff           	jae	0x1407db657 <.text+0x7da657>
1407db9a9: f3 45 0f 58 c2              	addss	%xmm10, %xmm8
1407db9ae: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407db9b1: f3 41 0f 51 c0              	sqrtss	%xmm8, %xmm0
1407db9b6: f3 0f 10 15 b2 9e 14 02     	movss	0x2149eb2(%rip), %xmm2  # 0x142925870
1407db9be: 0f 28 ca                    	movaps	%xmm2, %xmm1
1407db9c1: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407db9c5: 0f 28 c2                    	movaps	%xmm2, %xmm0
1407db9c8: f3 0f 5d c1                 	minss	%xmm1, %xmm0
1407db9cc: 31 c0                       	xorl	%eax, %eax
1407db9ce: 41 0f 2e f5                 	ucomiss	%xmm13, %xmm6
1407db9d2: 0f 97 c0                    	seta	%al
1407db9d5: f3 44 0f 10 15 26 9f 14 02  	movss	0x2149f26(%rip), %xmm10 # 0x142925904
1407db9de: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
1407db9e2: f3 0f 5c d0                 	subss	%xmm0, %xmm2
1407db9e6: 66 0f 6e c8                 	movd	%eax, %xmm1
1407db9ea: 66 0f 70 c9 50              	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
1407db9ef: 66 0f 72 f1 1f              	pslld	$0x1f, %xmm1
1407db9f4: 66 0f 72 e1 1f              	psrad	$0x1f, %xmm1
1407db9f9: 0f 28 de                    	movaps	%xmm6, %xmm3
1407db9fc: 0f c6 de e1                 	shufps	$0xe1, %xmm6, %xmm3     # xmm3 = xmm3[1,0],xmm6[2,3]
1407dba00: 66 0f 6f e1                 	movdqa	%xmm1, %xmm4
1407dba04: 66 0f df e3                 	pandn	%xmm3, %xmm4
1407dba08: 0f 28 de                    	movaps	%xmm6, %xmm3
1407dba0b: 0f 54 d9                    	andps	%xmm1, %xmm3
1407dba0e: 0f 56 dc                    	orps	%xmm4, %xmm3
1407dba11: f3 0f 12 e2                 	movsldup	%xmm2, %xmm4            # xmm4 = xmm2[0,0,2,2]
1407dba15: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
1407dba18: 0f 59 d3                    	mulps	%xmm3, %xmm2
1407dba1b: f3 44 0f 16 c2              	movshdup	%xmm2, %xmm8            # xmm8 = xmm2[1,1,3,3]
1407dba20: 44 0f 58 c2                 	addps	%xmm2, %xmm8
1407dba24: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407dba27: 41 0f 55 d3                 	andnps	%xmm11, %xmm2
1407dba2b: 0f 28 d9                    	movaps	%xmm1, %xmm3
1407dba2e: 41 0f 55 de                 	andnps	%xmm14, %xmm3
1407dba32: 44 0f 54 f1                 	andps	%xmm1, %xmm14
1407dba36: 44 0f 56 f2                 	orps	%xmm2, %xmm14
1407dba3a: 41 0f 54 cb                 	andps	%xmm11, %xmm1
1407dba3e: 0f 56 cb                    	orps	%xmm3, %xmm1
1407dba41: 44 0f 59 f4                 	mulps	%xmm4, %xmm14
1407dba45: f3 0f 12 f8                 	movsldup	%xmm0, %xmm7            # xmm7 = xmm0[0,0,2,2]
1407dba49: 0f 59 f9                    	mulps	%xmm1, %xmm7
1407dba4c: 41 0f 58 fe                 	addps	%xmm14, %xmm7
1407dba50: 0f 13 7c 24 50              	movlps	%xmm7, 0x50(%rsp)
1407dba55: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1407dba59: f3 0f 5d c6                 	minss	%xmm6, %xmm0
1407dba5d: f3 0f c2 f6 03              	cmpunordss	%xmm6, %xmm6
1407dba62: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407dba65: 41 0f 54 cd                 	andps	%xmm13, %xmm1
1407dba69: 0f 55 f0                    	andnps	%xmm0, %xmm6
1407dba6c: 0f 56 f1                    	orps	%xmm1, %xmm6
1407dba6f: 49 89 ef                    	movq	%rbp, %r15
1407dba72: 48 89 f5                    	movq	%rsi, %rbp
1407dba75: 4c 89 ee                    	movq	%r13, %rsi
1407dba78: 4c 8d ac 24 08 01 00 00     	leaq	0x108(%rsp), %r13
1407dba80: 4c 89 e9                    	movq	%r13, %rcx
1407dba83: 48 8d 7c 24 50              	leaq	0x50(%rsp), %rdi
1407dba88: 48 89 fa                    	movq	%rdi, %rdx
1407dba8b: 0f 28 d6                    	movaps	%xmm6, %xmm2
1407dba8e: e8 4d 13 4c 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407dba93: 0f 28 84 24 a0 01 00 00     	movaps	0x1a0(%rsp), %xmm0
1407dba9b: 0f 13 44 24 50              	movlps	%xmm0, 0x50(%rsp)
1407dbaa0: 48 8d 8c 24 14 01 00 00     	leaq	0x114(%rsp), %rcx
1407dbaa8: 48 89 fa                    	movq	%rdi, %rdx
1407dbaab: e8 10 13 4c 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407dbab0: f3 0f 10 b4 24 08 01 00 00  	movss	0x108(%rsp), %xmm6
1407dbab9: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407dbabc: f3 0f 5c c6                 	subss	%xmm6, %xmm0
1407dbac0: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407dbac3: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407dbac7: f3 44 0f 10 8c 24 14 01 00 00       	movss	0x114(%rsp), %xmm9
1407dbad1: f2 0f 10 ac 24 0c 01 00 00  	movsd	0x10c(%rsp), %xmm5
1407dbada: 0f 28 d7                    	movaps	%xmm7, %xmm2
1407dbadd: 41 0f c6 d0 01              	shufps	$0x1, %xmm8, %xmm2      # xmm2 = xmm2[1,0],xmm8[0,0]
1407dbae2: 41 0f c6 d0 e2              	shufps	$0xe2, %xmm8, %xmm2     # xmm2 = xmm2[2,0],xmm8[2,3]
1407dbae7: 0f 5c d5                    	subps	%xmm5, %xmm2
1407dbaea: 0f 28 da                    	movaps	%xmm2, %xmm3
1407dbaed: 0f 59 da                    	mulps	%xmm2, %xmm3
1407dbaf0: f3 0f 58 cb                 	addss	%xmm3, %xmm1
1407dbaf4: f3 0f 16 db                 	movshdup	%xmm3, %xmm3            # xmm3 = xmm3[1,1,3,3]
1407dbaf8: f3 0f 58 d9                 	addss	%xmm1, %xmm3
1407dbafc: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407dbaff: f3 0f 51 cb                 	sqrtss	%xmm3, %xmm1
1407dbb03: 41 0f 28 da                 	movaps	%xmm10, %xmm3
1407dbb07: f3 0f 5e d9                 	divss	%xmm1, %xmm3
1407dbb0b: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
1407dbb0f: f3 0f 12 e3                 	movsldup	%xmm3, %xmm4            # xmm4 = xmm3[0,0,2,2]
1407dbb13: 0f 59 e2                    	mulps	%xmm2, %xmm4
1407dbb16: f2 0f 10 8c 24 18 01 00 00  	movsd	0x118(%rsp), %xmm1
1407dbb1f: 41 0f 28 d9                 	movaps	%xmm9, %xmm3
1407dbb23: 0f c6 d9 d4                 	shufps	$0xd4, %xmm1, %xmm3     # xmm3 = xmm3[0,1],xmm1[1,3]
1407dbb27: 0f c6 d9 f2                 	shufps	$0xf2, %xmm1, %xmm3     # xmm3 = xmm3[2,0],xmm1[3,3]
1407dbb2b: 0f 59 dc                    	mulps	%xmm4, %xmm3
1407dbb2e: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407dbb31: 0f c6 d4 d4                 	shufps	$0xd4, %xmm4, %xmm2     # xmm2 = xmm2[0,1],xmm4[1,3]
1407dbb35: 0f c6 d4 f2                 	shufps	$0xf2, %xmm4, %xmm2     # xmm2 = xmm2[2,0],xmm4[3,3]
1407dbb39: 44 0f 28 d1                 	movaps	%xmm1, %xmm10
1407dbb3d: 44 0f 59 d2                 	mulps	%xmm2, %xmm10
1407dbb41: 41 0f 5c da                 	subps	%xmm10, %xmm3
1407dbb45: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407dbb49: f3 44 0f 59 cc              	mulss	%xmm4, %xmm9
1407dbb4e: f3 41 0f 5c c9              	subss	%xmm9, %xmm1
1407dbb53: f3 44 0f 16 cb              	movshdup	%xmm3, %xmm9            # xmm9 = xmm3[1,1,3,3]
1407dbb58: 44 0f 28 d5                 	movaps	%xmm5, %xmm10
1407dbb5c: f3 44 0f 10 d6              	movss	%xmm6, %xmm10           # xmm10 = xmm6[0],xmm10[1,2,3]
1407dbb61: 44 0f 5c d7                 	subps	%xmm7, %xmm10
1407dbb65: 45 0f 59 d2                 	mulps	%xmm10, %xmm10
1407dbb69: f3 41 0f 5c e8              	subss	%xmm8, %xmm5
1407dbb6e: f3 0f 59 ed                 	mulss	%xmm5, %xmm5
1407dbb72: f3 41 0f 58 ea              	addss	%xmm10, %xmm5
1407dbb77: f3 41 0f 16 f2              	movshdup	%xmm10, %xmm6           # xmm6 = xmm10[1,1,3,3]
1407dbb7c: f3 0f 58 f5                 	addss	%xmm5, %xmm6
1407dbb80: 0f 57 ed                    	xorps	%xmm5, %xmm5
1407dbb83: f3 0f 51 ee                 	sqrtss	%xmm6, %xmm5
1407dbb87: 0f 28 f1                    	movaps	%xmm1, %xmm6
1407dbb8a: f3 0f 12 fd                 	movsldup	%xmm5, %xmm7            # xmm7 = xmm5[0,0,2,2]
1407dbb8e: 0f 59 fc                    	mulps	%xmm4, %xmm7
1407dbb91: 44 0f 28 c1                 	movaps	%xmm1, %xmm8
1407dbb95: 0f 14 cb                    	unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
1407dbb98: 0f 59 cc                    	mulps	%xmm4, %xmm1
1407dbb9b: f3 0f 59 e3                 	mulss	%xmm3, %xmm4
1407dbb9f: f3 44 0f 59 c8              	mulss	%xmm0, %xmm9
1407dbba4: f3 41 0f 5c e1              	subss	%xmm9, %xmm4
1407dbba9: 44 0f c6 c3 d4              	shufps	$0xd4, %xmm3, %xmm8     # xmm8 = xmm8[0,1],xmm3[1,3]
1407dbbae: 44 0f c6 c3 e2              	shufps	$0xe2, %xmm3, %xmm8     # xmm8 = xmm8[2,0],xmm3[2,3]
1407dbbb3: 44 0f 28 0d 05 bc 2a 02     	movaps	0x22abc05(%rip), %xmm9  # 0x142a877c0
1407dbbbb: 41 0f 59 d9                 	mulps	%xmm9, %xmm3
1407dbbbf: f3 44 0f 10 15 08 bc 2a 02  	movss	0x22abc08(%rip), %xmm10 # 0x142a877d0
1407dbbc8: f3 41 0f 59 f2              	mulss	%xmm10, %xmm6
1407dbbcd: 0f 13 5c 24 30              	movlps	%xmm3, 0x30(%rsp)
1407dbbd2: f3 0f 11 74 24 38           	movss	%xmm6, 0x38(%rsp)
1407dbbd8: f3 0f 59 c5                 	mulss	%xmm5, %xmm0
1407dbbdc: f3 0f 11 44 24 40           	movss	%xmm0, 0x40(%rsp)
1407dbbe2: 0f 13 7c 24 44              	movlps	%xmm7, 0x44(%rsp)
1407dbbe7: f3 41 0f 59 e2              	mulss	%xmm10, %xmm4
1407dbbec: 44 0f 59 c2                 	mulps	%xmm2, %xmm8
1407dbbf0: 44 0f 5c c1                 	subps	%xmm1, %xmm8
1407dbbf4: 45 0f 59 c1                 	mulps	%xmm9, %xmm8
1407dbbf8: 44 0f 13 44 24 50           	movlps	%xmm8, 0x50(%rsp)
1407dbbfe: f3 0f 11 64 24 58           	movss	%xmm4, 0x58(%rsp)
1407dbc04: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
1407dbc09: 49 89 f5                    	movq	%rsi, %r13
1407dbc0c: 48 89 ee                    	movq	%rbp, %rsi
1407dbc0f: 4c 89 fd                    	movq	%r15, %rbp
1407dbc12: 48 8d 8c 24 f0 01 00 00     	leaq	0x1f0(%rsp), %rcx
1407dbc1a: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
1407dbc1f: 4d 89 e8                    	movq	%r13, %r8
1407dbc22: 49 89 f9                    	movq	%rdi, %r9
1407dbc25: e8 d6 18 0c 00              	callq	0x14089d500 <.text+0x89c500>
1407dbc2a: 4d 8b 07                    	movq	(%r15), %r8
1407dbc2d: 49 8d 0c 18                 	leaq	(%r8,%rbx), %rcx
1407dbc31: 49 89 0f                    	movq	%rcx, (%r15)
1407dbc34: 48 89 c8                    	movq	%rcx, %rax
1407dbc37: 4c 31 f0                    	xorq	%r14, %rax
1407dbc3a: 48 f7 e1                    	mulq	%rcx
1407dbc3d: 48 31 c2                    	xorq	%rax, %rdx
1407dbc40: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
1407dbc46: 83 f8 5f                    	cmpl	$0x5f, %eax
1407dbc49: 0f 87 91 f9 ff ff           	ja	0x1407db5e0 <.text+0x7da5e0>
1407dbc4f: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1407dbc59: 49 01 c0                    	addq	%rax, %r8
1407dbc5c: 0f 1f 40 00                 	nopl	(%rax)
1407dbc60: 48 01 d9                    	addq	%rbx, %rcx
1407dbc63: 4c 89 c0                    	movq	%r8, %rax
1407dbc66: 4c 31 f0                    	xorq	%r14, %rax
1407dbc69: 49 f7 e0                    	mulq	%r8
1407dbc6c: 48 31 c2                    	xorq	%rax, %rdx
1407dbc6f: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
1407dbc75: 49 01 d8                    	addq	%rbx, %r8
1407dbc78: 83 f8 60                    	cmpl	$0x60, %eax
1407dbc7b: 72 e3                       	jb	0x1407dbc60 <.text+0x7dac60>
1407dbc7d: 48 89 4d 00                 	movq	%rcx, (%rbp)
1407dbc81: e9 5a f9 ff ff              	jmp	0x1407db5e0 <.text+0x7da5e0>
1407dbc86: 0f 28 84 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm0
1407dbc8e: 0f 5c 84 24 b0 00 00 00     	subps	0xb0(%rsp), %xmm0
1407dbc96: 0f 28 8c 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm1
1407dbc9e: f3 0f 5c 8c 24 a0 00 00 00  	subss	0xa0(%rsp), %xmm1
1407dbca7: 0f 59 c0                    	mulps	%xmm0, %xmm0
1407dbcaa: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1407dbcae: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407dbcb2: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407dbcb6: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407dbcba: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407dbcbd: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1407dbcc1: f3 0f 5e 05 2b 41 2a 02     	divss	0x22a412b(%rip), %xmm0  # 0x142a7fdf4
1407dbcc9: e8 92 81 14 02              	callq	0x142923e60 <.text+0x2922e60>
1407dbcce: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
1407dbcd3: 48 89 c1                    	movq	%rax, %rcx
1407dbcd6: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
1407dbcda: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407dbcdd: f3 0f 5c 0d f7 e3 17 02     	subss	0x217e3f7(%rip), %xmm1  # 0x14295a0dc
1407dbce5: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
1407dbcea: 48 21 ca                    	andq	%rcx, %rdx
1407dbced: 48 09 c2                    	orq	%rax, %rdx
1407dbcf0: 31 ff                       	xorl	%edi, %edi
1407dbcf2: 0f 57 f6                    	xorps	%xmm6, %xmm6
1407dbcf5: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
1407dbcf8: 48 0f 42 d7                 	cmovbq	%rdi, %rdx
1407dbcfc: 0f 2e 05 dd e3 17 02        	ucomiss	0x217e3dd(%rip), %xmm0  # 0x14295a0e0
1407dbd03: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
1407dbd0a: 48 0f 46 c2                 	cmovbeq	%rdx, %rax
1407dbd0e: 48 83 f8 04                 	cmpq	$0x4, %rax
1407dbd12: 41 bc 03 00 00 00           	movl	$0x3, %r12d
1407dbd18: 4c 0f 43 e0                 	cmovaeq	%rax, %r12
1407dbd1c: 0f 28 ac 24 30 01 00 00     	movaps	0x130(%rsp), %xmm5
1407dbd24: 0f 28 84 24 20 01 00 00     	movaps	0x120(%rsp), %xmm0
1407dbd2c: 0f 16 e8                    	movlhps	%xmm0, %xmm5            # xmm5 = xmm5[0],xmm0[0]
1407dbd2f: 0f c6 e8 e2                 	shufps	$0xe2, %xmm0, %xmm5     # xmm5 = xmm5[2,0],xmm0[2,3]
1407dbd33: 0f 28 05 86 15 1f 02        	movaps	0x21f1586(%rip), %xmm0  # 0x1429cd2c0
1407dbd3a: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407dbd3d: 0f 5f cd                    	maxps	%xmm5, %xmm1
1407dbd40: 0f c2 ed 03                 	cmpunordps	%xmm5, %xmm5
1407dbd44: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407dbd47: 0f 54 d5                    	andps	%xmm5, %xmm2
1407dbd4a: 0f 55 e9                    	andnps	%xmm1, %xmm5
1407dbd4d: 0f 56 ea                    	orps	%xmm2, %xmm5
1407dbd50: 66 0f 70 d5 f5              	pshufd	$0xf5, %xmm5, %xmm2     # xmm2 = xmm5[1,1,3,3]
1407dbd55: f3 0f 10 bc 24 98 00 00 00  	movss	0x98(%rsp), %xmm7
1407dbd5e: 66 0f 6f e2                 	movdqa	%xmm2, %xmm4
1407dbd62: f3 0f 59 e7                 	mulss	%xmm7, %xmm4
1407dbd66: f3 44 0f 10 0d 01 9b 14 02  	movss	0x2149b01(%rip), %xmm9  # 0x142925870
1407dbd6f: f3 41 0f 59 e1              	mulss	%xmm9, %xmm4
1407dbd74: 0f 28 dd                    	movaps	%xmm5, %xmm3
1407dbd77: f3 0f 59 df                 	mulss	%xmm7, %xmm3
1407dbd7b: f2 44 0f 10 84 24 90 00 00 00       	movsd	0x90(%rsp), %xmm8
1407dbd85: f3 41 0f 16 c8              	movshdup	%xmm8, %xmm1            # xmm1 = xmm8[1,1,3,3]
1407dbd8a: 41 0f 59 d0                 	mulps	%xmm8, %xmm2
1407dbd8e: 44 0f 28 15 4a 9b 14 02     	movaps	0x2149b4a(%rip), %xmm10 # 0x1429258e0
1407dbd96: 41 0f 59 d2                 	mulps	%xmm10, %xmm2
1407dbd9a: 44 0f 28 e5                 	movaps	%xmm5, %xmm12
1407dbd9e: 66 0f 70 ed a0              	pshufd	$0xa0, %xmm5, %xmm5     # xmm5 = xmm5[0,0,2,2]
1407dbda3: 41 0f 59 e8                 	mulps	%xmm8, %xmm5
1407dbda7: 41 0f 59 ea                 	mulps	%xmm10, %xmm5
1407dbdab: f3 41 0f 59 d9              	mulss	%xmm9, %xmm3
1407dbdb0: 4c 89 e0                    	movq	%r12, %rax
1407dbdb3: 48 ff c8                    	decq	%rax
1407dbdb6: 78 0b                       	js	0x1407dbdc3 <.text+0x7dadc3>
1407dbdb8: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
1407dbdbc: f3 4c 0f 2a c8              	cvtsi2ss	%rax, %xmm9
1407dbdc1: eb 1a                       	jmp	0x1407dbddd <.text+0x7daddd>
1407dbdc3: 48 89 c1                    	movq	%rax, %rcx
1407dbdc6: 48 d1 e9                    	shrq	%rcx
1407dbdc9: 83 e0 01                    	andl	$0x1, %eax
1407dbdcc: 48 09 c8                    	orq	%rcx, %rax
1407dbdcf: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
1407dbdd3: f3 4c 0f 2a c8              	cvtsi2ss	%rax, %xmm9
1407dbdd8: f3 45 0f 58 c9              	addss	%xmm9, %xmm9
1407dbddd: 44 0f 28 94 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm10
1407dbde6: f3 44 0f 5c d4              	subss	%xmm4, %xmm10
1407dbdeb: 44 0f 29 94 24 c0 00 00 00  	movaps	%xmm10, 0xc0(%rsp)
1407dbdf4: 0f 28 a4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm4
1407dbdfc: 0f 5c e2                    	subps	%xmm2, %xmm4
1407dbdff: 0f 29 a4 24 d0 00 00 00     	movaps	%xmm4, 0xd0(%rsp)
1407dbe07: 0f 28 94 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm2
1407dbe0f: 0f 5c d5                    	subps	%xmm5, %xmm2
1407dbe12: 0f 29 94 24 b0 00 00 00     	movaps	%xmm2, 0xb0(%rsp)
1407dbe1a: 0f 28 94 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm2
1407dbe22: f3 0f 5c d3                 	subss	%xmm3, %xmm2
1407dbe26: 0f 29 94 24 a0 00 00 00     	movaps	%xmm2, 0xa0(%rsp)
1407dbe2e: 0f 28 d7                    	movaps	%xmm7, %xmm2
1407dbe31: f3 0f 59 d6                 	mulss	%xmm6, %xmm2
1407dbe35: 0f 57 db                    	xorps	%xmm3, %xmm3
1407dbe38: 41 0f 59 d8                 	mulps	%xmm8, %xmm3
1407dbe3c: f3 44 0f 16 d3              	movshdup	%xmm3, %xmm10           # xmm10 = xmm3[1,1,3,3]
1407dbe41: f3 44 0f 5c d7              	subss	%xmm7, %xmm10
1407dbe46: f3 45 0f 12 d8              	movsldup	%xmm8, %xmm11           # xmm11 = xmm8[0,0,2,2]
1407dbe4b: f3 44 0f 10 da              	movss	%xmm2, %xmm11           # xmm11 = xmm2[0],xmm11[1,2,3]
1407dbe50: 44 0f 5c db                 	subps	%xmm3, %xmm11
1407dbe54: f3 41 0f 16 d3              	movshdup	%xmm11, %xmm2           # xmm2 = xmm11[1,1,3,3]
1407dbe59: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
1407dbe5d: f3 44 0f 59 ea              	mulss	%xmm2, %xmm13
1407dbe62: 0f 28 df                    	movaps	%xmm7, %xmm3
1407dbe65: f3 41 0f 59 db              	mulss	%xmm11, %xmm3
1407dbe6a: f3 44 0f 5c eb              	subss	%xmm3, %xmm13
1407dbe6f: 44 0f 28 f7                 	movaps	%xmm7, %xmm14
1407dbe73: f3 45 0f 59 f2              	mulss	%xmm10, %xmm14
1407dbe78: f3 41 0f 59 d0              	mulss	%xmm8, %xmm2
1407dbe7d: f3 44 0f 5c f2              	subss	%xmm2, %xmm14
1407dbe82: 45 0f 28 f8                 	movaps	%xmm8, %xmm15
1407dbe86: f3 45 0f 59 fb              	mulss	%xmm11, %xmm15
1407dbe8b: f3 41 0f 59 ca              	mulss	%xmm10, %xmm1
1407dbe90: f3 44 0f 5c f9              	subss	%xmm1, %xmm15
1407dbe95: f3 0f 10 0d 83 12 1f 02     	movss	0x21f1283(%rip), %xmm1  # 0x1429cd120
1407dbe9d: f3 44 0f 59 e9              	mulss	%xmm1, %xmm13
1407dbea2: f3 44 0f 59 f1              	mulss	%xmm1, %xmm14
1407dbea7: f3 44 0f 59 f9              	mulss	%xmm1, %xmm15
1407dbeac: f3 44 0f 59 15 bb 9a 14 02  	mulss	0x2149abb(%rip), %xmm10 # 0x142925970
1407dbeb5: 44 0f 59 d8                 	mulps	%xmm0, %xmm11
1407dbeb9: 4c 8d 6c 24 40              	leaq	0x40(%rsp), %r13
1407dbebe: 4c 8d 7c 24 50              	leaq	0x50(%rsp), %r15
1407dbec3: e9 81 00 00 00              	jmp	0x1407dbf49 <.text+0x7daf49>
1407dbec8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407dbed0: 89 d0                       	movl	%edx, %eax
1407dbed2: 48 69 c0 c8 00 00 00        	imulq	$0xc8, %rax, %rax
1407dbed9: 48 c1 e8 20                 	shrq	$0x20, %rax
1407dbedd: 83 c0 9c                    	addl	$-0x64, %eax
1407dbee0: 0f 10 84 24 20 02 00 00     	movups	0x220(%rsp), %xmm0
1407dbee8: 0f 10 8c 24 30 02 00 00     	movups	0x230(%rsp), %xmm1
1407dbef0: 0f 10 94 24 40 02 00 00     	movups	0x240(%rsp), %xmm2
1407dbef8: 0f 29 54 24 70              	movaps	%xmm2, 0x70(%rsp)
1407dbefd: 0f 29 4c 24 60              	movaps	%xmm1, 0x60(%rsp)
1407dbf02: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
1407dbf07: c7 84 24 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rsp)
1407dbf12: 89 84 24 84 00 00 00        	movl	%eax, 0x84(%rsp)
1407dbf19: 8b 84 24 70 03 00 00        	movl	0x370(%rsp), %eax
1407dbf20: 89 84 24 88 00 00 00        	movl	%eax, 0x88(%rsp)
1407dbf27: c7 84 24 8c 00 00 00 19 00 00 00    	movl	$0x19, 0x8c(%rsp)
1407dbf32: 48 89 f1                    	movq	%rsi, %rcx
1407dbf35: 4c 89 fa                    	movq	%r15, %rdx
1407dbf38: e8 a3 11 ab 00              	callq	0x14128d0e0 <.text+0x128c0e0>
1407dbf3d: 48 ff c7                    	incq	%rdi
1407dbf40: 4c 39 e7                    	cmpq	%r12, %rdi
1407dbf43: 0f 84 7d 01 00 00           	je	0x1407dc0c6 <.text+0x7db0c6>
1407dbf49: 48 85 ff                    	testq	%rdi, %rdi
1407dbf4c: 78 22                       	js	0x1407dbf70 <.text+0x7daf70>
1407dbf4e: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407dbf51: f3 48 0f 2a c7              	cvtsi2ss	%rdi, %xmm0
1407dbf56: f3 41 0f 5e c1              	divss	%xmm9, %xmm0
1407dbf5b: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
1407dbf5e: 75 36                       	jne	0x1407dbf96 <.text+0x7daf96>
1407dbf60: 7a 34                       	jp	0x1407dbf96 <.text+0x7daf96>
1407dbf62: eb d9                       	jmp	0x1407dbf3d <.text+0x7daf3d>
1407dbf64: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1407dbf70: 48 89 f8                    	movq	%rdi, %rax
1407dbf73: 48 d1 e8                    	shrq	%rax
1407dbf76: 89 f9                       	movl	%edi, %ecx
1407dbf78: 83 e1 01                    	andl	$0x1, %ecx
1407dbf7b: 48 09 c1                    	orq	%rax, %rcx
1407dbf7e: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407dbf81: f3 48 0f 2a c1              	cvtsi2ss	%rcx, %xmm0
1407dbf86: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407dbf8a: f3 41 0f 5e c1              	divss	%xmm9, %xmm0
1407dbf8f: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
1407dbf92: 75 02                       	jne	0x1407dbf96 <.text+0x7daf96>
1407dbf94: 7b a7                       	jnp	0x1407dbf3d <.text+0x7daf3d>
1407dbf96: 0f 2e 05 67 99 14 02        	ucomiss	0x2149967(%rip), %xmm0  # 0x142925904
1407dbf9d: 75 02                       	jne	0x1407dbfa1 <.text+0x7dafa1>
1407dbf9f: 7b 9c                       	jnp	0x1407dbf3d <.text+0x7daf3d>
1407dbfa1: f3 0f 10 0d 5b 99 14 02     	movss	0x214995b(%rip), %xmm1  # 0x142925904
1407dbfa9: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1407dbfad: 0f 28 94 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm2
1407dbfb5: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
1407dbfb9: f3 0f 12 d8                 	movsldup	%xmm0, %xmm3            # xmm3 = xmm0[0,0,2,2]
1407dbfbd: 0f 59 9c 24 b0 00 00 00     	mulps	0xb0(%rsp), %xmm3
1407dbfc5: 0f 28 a4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm4
1407dbfcd: f3 0f 59 e0                 	mulss	%xmm0, %xmm4
1407dbfd1: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1407dbfd5: f3 0f 12 d1                 	movsldup	%xmm1, %xmm2            # xmm2 = xmm1[0,0,2,2]
1407dbfd9: 0f 59 94 24 d0 00 00 00     	mulps	0xd0(%rsp), %xmm2
1407dbfe1: 0f 58 d3                    	addps	%xmm3, %xmm2
1407dbfe4: 0f 13 94 24 f0 00 00 00     	movlps	%xmm2, 0xf0(%rsp)
1407dbfec: f3 0f 11 a4 24 f8 00 00 00  	movss	%xmm4, 0xf8(%rsp)
1407dbff5: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1407dbff8: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
1407dbffc: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407dc000: 0f 58 c8                    	addps	%xmm0, %xmm1
1407dc003: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407dc006: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
1407dc00a: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
1407dc00e: 41 0f 59 c8                 	mulps	%xmm8, %xmm1
1407dc012: 0f 13 4c 24 40              	movlps	%xmm1, 0x40(%rsp)
1407dc017: f3 0f 11 44 24 48           	movss	%xmm0, 0x48(%rsp)
1407dc01d: f3 44 0f 11 6c 24 30        	movss	%xmm13, 0x30(%rsp)
1407dc024: f3 44 0f 11 74 24 34        	movss	%xmm14, 0x34(%rsp)
1407dc02b: f3 44 0f 11 7c 24 38        	movss	%xmm15, 0x38(%rsp)
1407dc032: f3 44 0f 11 54 24 50        	movss	%xmm10, 0x50(%rsp)
1407dc039: 44 0f 13 5c 24 54           	movlps	%xmm11, 0x54(%rsp)
1407dc03f: 48 8d 84 24 f0 00 00 00     	leaq	0xf0(%rsp), %rax
1407dc047: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407dc04c: 48 8d 8c 24 20 02 00 00     	leaq	0x220(%rsp), %rcx
1407dc054: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
1407dc059: 4d 89 e8                    	movq	%r13, %r8
1407dc05c: 4d 89 f9                    	movq	%r15, %r9
1407dc05f: e8 9c 14 0c 00              	callq	0x14089d500 <.text+0x89c500>
1407dc064: 4c 8b 45 00                 	movq	(%rbp), %r8
1407dc068: 49 8d 0c 18                 	leaq	(%r8,%rbx), %rcx
1407dc06c: 48 89 4d 00                 	movq	%rcx, (%rbp)
1407dc070: 48 89 c8                    	movq	%rcx, %rax
1407dc073: 4c 31 f0                    	xorq	%r14, %rax
1407dc076: 48 f7 e1                    	mulq	%rcx
1407dc079: 48 31 c2                    	xorq	%rax, %rdx
1407dc07c: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
1407dc082: 83 f8 5f                    	cmpl	$0x5f, %eax
1407dc085: 0f 87 45 fe ff ff           	ja	0x1407dbed0 <.text+0x7daed0>
1407dc08b: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1407dc095: 49 01 c0                    	addq	%rax, %r8
1407dc098: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407dc0a0: 48 01 d9                    	addq	%rbx, %rcx
1407dc0a3: 4c 89 c0                    	movq	%r8, %rax
1407dc0a6: 4c 31 f0                    	xorq	%r14, %rax
1407dc0a9: 49 f7 e0                    	mulq	%r8
1407dc0ac: 48 31 c2                    	xorq	%rax, %rdx
1407dc0af: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
1407dc0b5: 49 01 d8                    	addq	%rbx, %r8
1407dc0b8: 83 f8 60                    	cmpl	$0x60, %eax
1407dc0bb: 72 e3                       	jb	0x1407dc0a0 <.text+0x7db0a0>
1407dc0bd: 48 89 4d 00                 	movq	%rcx, (%rbp)
1407dc0c1: e9 0a fe ff ff              	jmp	0x1407dbed0 <.text+0x7daed0>
1407dc0c6: 0f 28 b4 24 50 02 00 00     	movaps	0x250(%rsp), %xmm6
1407dc0ce: 0f 28 bc 24 60 02 00 00     	movaps	0x260(%rsp), %xmm7
1407dc0d6: 44 0f 28 84 24 70 02 00 00  	movaps	0x270(%rsp), %xmm8
1407dc0df: 44 0f 28 8c 24 80 02 00 00  	movaps	0x280(%rsp), %xmm9
1407dc0e8: 44 0f 28 94 24 90 02 00 00  	movaps	0x290(%rsp), %xmm10
1407dc0f1: 44 0f 28 9c 24 a0 02 00 00  	movaps	0x2a0(%rsp), %xmm11
1407dc0fa: 44 0f 28 a4 24 b0 02 00 00  	movaps	0x2b0(%rsp), %xmm12
1407dc103: 44 0f 28 ac 24 c0 02 00 00  	movaps	0x2c0(%rsp), %xmm13
1407dc10c: 44 0f 28 b4 24 d0 02 00 00  	movaps	0x2d0(%rsp), %xmm14
1407dc115: 44 0f 28 bc 24 e0 02 00 00  	movaps	0x2e0(%rsp), %xmm15
1407dc11e: 48 81 c4 f8 02 00 00        	addq	$0x2f8, %rsp            # imm = 0x2F8
1407dc125: 5b                          	popq	%rbx
1407dc126: 5d                          	popq	%rbp
1407dc127: 5f                          	popq	%rdi
1407dc128: 5e                          	popq	%rsi
1407dc129: 41 5c                       	popq	%r12
1407dc12b: 41 5d                       	popq	%r13
1407dc12d: 41 5e                       	popq	%r14
1407dc12f: 41 5f                       	popq	%r15
1407dc131: c3                          	retq
1407dc132: 48 8d 0d 87 c2 2a 02        	leaq	0x22ac287(%rip), %rcx   # 0x142a883c0
1407dc139: e8 d2 d1 0f 02              	callq	0x1428d9310 <.text+0x28d8310>
1407dc13e: 48 8d 0d 93 c2 2a 02        	leaq	0x22ac293(%rip), %rcx   # 0x142a883d8
1407dc145: e8 c6 d1 0f 02              	callq	0x1428d9310 <.text+0x28d8310>
1407dc14a: 48 8d 0d 9f c2 2a 02        	leaq	0x22ac29f(%rip), %rcx   # 0x142a883f0
