
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408ce4a0: 41 57                       	pushq	%r15
1408ce4a2: 41 56                       	pushq	%r14
1408ce4a4: 41 55                       	pushq	%r13
1408ce4a6: 41 54                       	pushq	%r12
1408ce4a8: 56                          	pushq	%rsi
1408ce4a9: 57                          	pushq	%rdi
1408ce4aa: 55                          	pushq	%rbp
1408ce4ab: 53                          	pushq	%rbx
1408ce4ac: 48 81 ec 28 01 00 00        	subq	$0x128, %rsp            # imm = 0x128
1408ce4b3: 44 0f 29 bc 24 10 01 00 00  	movaps	%xmm15, 0x110(%rsp)
1408ce4bc: 44 0f 29 b4 24 00 01 00 00  	movaps	%xmm14, 0x100(%rsp)
1408ce4c5: 44 0f 29 ac 24 f0 00 00 00  	movaps	%xmm13, 0xf0(%rsp)
1408ce4ce: 44 0f 29 a4 24 e0 00 00 00  	movaps	%xmm12, 0xe0(%rsp)
1408ce4d7: 44 0f 29 9c 24 d0 00 00 00  	movaps	%xmm11, 0xd0(%rsp)
1408ce4e0: 44 0f 29 94 24 c0 00 00 00  	movaps	%xmm10, 0xc0(%rsp)
1408ce4e9: 44 0f 29 8c 24 b0 00 00 00  	movaps	%xmm9, 0xb0(%rsp)
1408ce4f2: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1408ce4fb: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
1408ce503: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
1408ce50b: 48 8b 84 24 90 01 00 00     	movq	0x190(%rsp), %rax
1408ce513: f3 41 0f 10 70 08           	movss	0x8(%r8), %xmm6
1408ce519: f2 0f 10 02                 	movsd	(%rdx), %xmm0
1408ce51d: f2 0f 10 4a 0c              	movsd	0xc(%rdx), %xmm1
1408ce522: f3 0f 12 fe                 	movsldup	%xmm6, %xmm7            # xmm7 = xmm6[0,0,2,2]
1408ce526: 0f 59 f9                    	mulps	%xmm1, %xmm7
1408ce529: f3 44 0f 10 42 14           	movss	0x14(%rdx), %xmm8
1408ce52f: f3 44 0f 59 c6              	mulss	%xmm6, %xmm8
1408ce534: f3 44 0f 58 42 08           	addss	0x8(%rdx), %xmm8
1408ce53a: 0f 58 f8                    	addps	%xmm0, %xmm7
1408ce53d: 4c 8b 78 18                 	movq	0x18(%rax), %r15
1408ce541: 49 8b 57 28                 	movq	0x28(%r15), %rdx
1408ce545: 48 85 d2                    	testq	%rdx, %rdx
1408ce548: 0f 84 24 03 00 00           	je	0x1408ce872 <.text+0x8cd872>
1408ce54e: 41 83 38 02                 	cmpl	$0x2, (%r8)
1408ce552: 0f 85 56 03 00 00           	jne	0x1408ce8ae <.text+0x8cd8ae>
1408ce558: 41 8b 40 04                 	movl	0x4(%r8), %eax
1408ce55c: 45 31 d2                    	xorl	%r10d, %r10d
1408ce55f: 48 39 c2                    	cmpq	%rax, %rdx
1408ce562: 4c 0f 46 d2                 	cmovbeq	%rdx, %r10
1408ce566: 4c 29 d0                    	subq	%r10, %rax
1408ce569: 48 39 d0                    	cmpq	%rdx, %rax
1408ce56c: 0f 83 0d 04 00 00           	jae	0x1408ce97f <.text+0x8cd97f>
1408ce572: 49 8b 57 20                 	movq	0x20(%r15), %rdx
1408ce576: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1408ce57a: 8b 34 82                    	movl	(%rdx,%rax,4), %esi
1408ce57d: 44 8b 54 82 04              	movl	0x4(%rdx,%rax,4), %r10d
1408ce582: 44 8b 5c 82 08              	movl	0x8(%rdx,%rax,4), %r11d
1408ce587: f3 45 0f 10 51 0c           	movss	0xc(%r9), %xmm10
1408ce58d: f3 45 0f 10 21              	movss	(%r9), %xmm12
1408ce592: f2 45 0f 10 59 04           	movsd	0x4(%r9), %xmm11
1408ce598: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1408ce59c: f3 41 0f 5c 41 18           	subss	0x18(%r9), %xmm0
1408ce5a2: f2 41 0f 10 51 10           	movsd	0x10(%r9), %xmm2
1408ce5a8: 0f 28 cf                    	movaps	%xmm7, %xmm1
1408ce5ab: 0f 5c ca                    	subps	%xmm2, %xmm1
1408ce5ae: 45 0f 28 ec                 	movaps	%xmm12, %xmm13
1408ce5b2: 45 0f c6 eb d4              	shufps	$0xd4, %xmm11, %xmm13   # xmm13 = xmm13[0,1],xmm11[1,3]
1408ce5b7: 45 0f c6 eb f2              	shufps	$0xf2, %xmm11, %xmm13   # xmm13 = xmm13[2,0],xmm11[3,3]
1408ce5bc: f3 0f 16 d9                 	movshdup	%xmm1, %xmm3            # xmm3 = xmm1[1,1,3,3]
1408ce5c0: 0f 28 d0                    	movaps	%xmm0, %xmm2
1408ce5c3: 0f c6 d1 d4                 	shufps	$0xd4, %xmm1, %xmm2     # xmm2 = xmm2[0,1],xmm1[1,3]
1408ce5c7: 0f c6 d1 f2                 	shufps	$0xf2, %xmm1, %xmm2     # xmm2 = xmm2[2,0],xmm1[3,3]
1408ce5cb: 41 0f 59 d5                 	mulps	%xmm13, %xmm2
1408ce5cf: 0f 28 e1                    	movaps	%xmm1, %xmm4
1408ce5d2: 0f 16 e0                    	movlhps	%xmm0, %xmm4            # xmm4 = xmm4[0],xmm0[0]
1408ce5d5: 0f c6 e1 f2                 	shufps	$0xf2, %xmm1, %xmm4     # xmm4 = xmm4[2,0],xmm1[3,3]
1408ce5d9: 41 0f 59 e3                 	mulps	%xmm11, %xmm4
1408ce5dd: 0f 5c d4                    	subps	%xmm4, %xmm2
1408ce5e0: 41 0f 28 e3                 	movaps	%xmm11, %xmm4
1408ce5e4: f3 0f 59 e1                 	mulss	%xmm1, %xmm4
1408ce5e8: f3 41 0f 59 dc              	mulss	%xmm12, %xmm3
1408ce5ed: f3 0f 5c e3                 	subss	%xmm3, %xmm4
1408ce5f1: 0f 58 d2                    	addps	%xmm2, %xmm2
1408ce5f4: f3 0f 58 e4                 	addss	%xmm4, %xmm4
1408ce5f8: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
1408ce5fc: 0f 28 ec                    	movaps	%xmm4, %xmm5
1408ce5ff: 0f c6 ea d4                 	shufps	$0xd4, %xmm2, %xmm5     # xmm5 = xmm5[0,1],xmm2[1,3]
1408ce603: 0f c6 ea f2                 	shufps	$0xf2, %xmm2, %xmm5     # xmm5 = xmm5[2,0],xmm2[3,3]
1408ce607: 41 0f 59 ed                 	mulps	%xmm13, %xmm5
1408ce60b: 45 0f 28 f2                 	movaps	%xmm10, %xmm14
1408ce60f: f3 44 0f 59 f4              	mulss	%xmm4, %xmm14
1408ce614: 0f 14 e2                    	unpcklps	%xmm2, %xmm4            # xmm4 = xmm4[0],xmm2[0],xmm4[1],xmm2[1]
1408ce617: 41 0f 59 e3                 	mulps	%xmm11, %xmm4
1408ce61b: 0f 5c ec                    	subps	%xmm4, %xmm5
1408ce61e: 41 0f 28 e3                 	movaps	%xmm11, %xmm4
1408ce622: f3 0f 59 e2                 	mulss	%xmm2, %xmm4
1408ce626: f3 41 0f 59 dc              	mulss	%xmm12, %xmm3
1408ce62b: f3 0f 5c e3                 	subss	%xmm3, %xmm4
1408ce62f: f3 45 0f 12 ca              	movsldup	%xmm10, %xmm9           # xmm9 = xmm10[0,0,2,2]
1408ce634: 41 0f 59 d1                 	mulps	%xmm9, %xmm2
1408ce638: 0f 58 d5                    	addps	%xmm5, %xmm2
1408ce63b: f3 41 0f 58 e6              	addss	%xmm14, %xmm4
1408ce640: 0f 58 d1                    	addps	%xmm1, %xmm2
1408ce643: f3 0f 58 e0                 	addss	%xmm0, %xmm4
1408ce647: 0f 13 54 24 38              	movlps	%xmm2, 0x38(%rsp)
1408ce64c: f3 0f 11 64 24 40           	movss	%xmm4, 0x40(%rsp)
1408ce652: 49 8b 57 18                 	movq	0x18(%r15), %rdx
1408ce656: 48 39 f2                    	cmpq	%rsi, %rdx
1408ce659: 0f 86 2f 03 00 00           	jbe	0x1408ce98e <.text+0x8cd98e>
1408ce65f: 49 8b 47 10                 	movq	0x10(%r15), %rax
1408ce663: 48 89 74 24 30              	movq	%rsi, 0x30(%rsp)
1408ce668: 4c 8d 2c 76                 	leaq	(%rsi,%rsi,2), %r13
1408ce66c: 46 8b 4c a8 08              	movl	0x8(%rax,%r13,4), %r9d
1408ce671: 44 89 4c 24 50              	movl	%r9d, 0x50(%rsp)
1408ce676: 4e 8b 0c a8                 	movq	(%rax,%r13,4), %r9
1408ce67a: 4c 89 4c 24 48              	movq	%r9, 0x48(%rsp)
1408ce67f: 4c 39 d2                    	cmpq	%r10, %rdx
1408ce682: 0f 86 15 03 00 00           	jbe	0x1408ce99d <.text+0x8cd99d>
1408ce688: 49 89 ce                    	movq	%rcx, %r14
1408ce68b: 4f 8d 24 52                 	leaq	(%r10,%r10,2), %r12
1408ce68f: 42 8b 4c a0 08              	movl	0x8(%rax,%r12,4), %ecx
1408ce694: 89 4c 24 60                 	movl	%ecx, 0x60(%rsp)
1408ce698: 4a 8b 0c a0                 	movq	(%rax,%r12,4), %rcx
1408ce69c: 48 89 4c 24 58              	movq	%rcx, 0x58(%rsp)
1408ce6a1: 4c 39 da                    	cmpq	%r11, %rdx
1408ce6a4: 0f 86 02 03 00 00           	jbe	0x1408ce9ac <.text+0x8cd9ac>
1408ce6aa: 4c 89 c6                    	movq	%r8, %rsi
1408ce6ad: 4c 89 d7                    	movq	%r10, %rdi
1408ce6b0: 4c 89 db                    	movq	%r11, %rbx
1408ce6b3: 4b 8d 2c 5b                 	leaq	(%r11,%r11,2), %rbp
1408ce6b7: 8b 4c a8 08                 	movl	0x8(%rax,%rbp,4), %ecx
1408ce6bb: 89 4c 24 70                 	movl	%ecx, 0x70(%rsp)
1408ce6bf: 48 8b 04 a8                 	movq	(%rax,%rbp,4), %rax
1408ce6c3: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1408ce6c8: 48 8d 44 24 68              	leaq	0x68(%rsp), %rax
1408ce6cd: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408ce6d2: 48 8d 4c 24 74              	leaq	0x74(%rsp), %rcx
1408ce6d7: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1408ce6dc: 4c 8d 44 24 48              	leaq	0x48(%rsp), %r8
1408ce6e1: 4c 8d 4c 24 58              	leaq	0x58(%rsp), %r9
1408ce6e6: e8 35 48 df 00              	callq	0x1416c2f20 <.text+0x16c1f20>
1408ce6eb: f3 0f 10 54 24 74           	movss	0x74(%rsp), %xmm2
1408ce6f1: f2 0f 10 5c 24 78           	movsd	0x78(%rsp), %xmm3
1408ce6f7: 49 8b 47 40                 	movq	0x40(%r15), %rax
1408ce6fb: f3 0f 12 cb                 	movsldup	%xmm3, %xmm1            # xmm1 = xmm3[0,0,2,2]
1408ce6ff: f3 0f 16 c3                 	movshdup	%xmm3, %xmm0            # xmm0 = xmm3[1,1,3,3]
1408ce703: 48 85 c0                    	testq	%rax, %rax
1408ce706: 0f 84 ab 01 00 00           	je	0x1408ce8b7 <.text+0x8cd8b7>
1408ce70c: 49 8b 57 48                 	movq	0x48(%r15), %rdx
1408ce710: 4c 8b 5c 24 30              	movq	0x30(%rsp), %r11
1408ce715: 4c 39 da                    	cmpq	%r11, %rdx
1408ce718: 0f 86 9d 02 00 00           	jbe	0x1408ce9bb <.text+0x8cd9bb>
1408ce71e: 48 39 fa                    	cmpq	%rdi, %rdx
1408ce721: 0f 86 a3 02 00 00           	jbe	0x1408ce9ca <.text+0x8cd9ca>
1408ce727: 48 39 da                    	cmpq	%rbx, %rdx
1408ce72a: 0f 86 b8 02 00 00           	jbe	0x1408ce9e8 <.text+0x8cd9e8>
1408ce730: 4c 89 f1                    	movq	%r14, %rcx
1408ce733: f2 42 0f 10 24 a8           	movsd	(%rax,%r13,4), %xmm4
1408ce739: f3 44 0f 12 f2              	movsldup	%xmm2, %xmm14           # xmm14 = xmm2[0,0,2,2]
1408ce73e: 44 0f 59 f4                 	mulps	%xmm4, %xmm14
1408ce742: f3 42 0f 10 6c a8 08        	movss	0x8(%rax,%r13,4), %xmm5
1408ce749: f3 0f 59 ea                 	mulss	%xmm2, %xmm5
1408ce74d: f3 0f 10 64 a8 08           	movss	0x8(%rax,%rbp,4), %xmm4
1408ce753: f3 46 0f 10 7c a0 08        	movss	0x8(%rax,%r12,4), %xmm15
1408ce75a: 44 0f 14 fc                 	unpcklps	%xmm4, %xmm15           # xmm15 = xmm15[0],xmm4[0],xmm15[1],xmm4[1]
1408ce75e: 41 0f 59 df                 	mulps	%xmm15, %xmm3
1408ce762: f3 0f 58 eb                 	addss	%xmm3, %xmm5
1408ce766: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
1408ce76a: f3 0f 58 e5                 	addss	%xmm5, %xmm4
1408ce76e: f2 42 0f 10 1c a0           	movsd	(%rax,%r12,4), %xmm3
1408ce774: f2 0f 10 2c a8              	movsd	(%rax,%rbp,4), %xmm5
1408ce779: 0f 59 d9                    	mulps	%xmm1, %xmm3
1408ce77c: 41 0f 58 de                 	addps	%xmm14, %xmm3
1408ce780: 0f 59 e8                    	mulps	%xmm0, %xmm5
1408ce783: 0f 58 eb                    	addps	%xmm3, %xmm5
1408ce786: f3 44 0f 12 f5              	movsldup	%xmm5, %xmm14           # xmm14 = xmm5[0,0,2,2]
1408ce78b: f3 44 0f 10 f4              	movss	%xmm4, %xmm14           # xmm14 = xmm4[0],xmm14[1,2,3]
1408ce790: 45 0f 59 f3                 	mulps	%xmm11, %xmm14
1408ce794: 0f 28 dc                    	movaps	%xmm4, %xmm3
1408ce797: 0f c6 dd d4                 	shufps	$0xd4, %xmm5, %xmm3     # xmm3 = xmm3[0,1],xmm5[1,3]
1408ce79b: 0f c6 dd e2                 	shufps	$0xe2, %xmm5, %xmm3     # xmm3 = xmm3[2,0],xmm5[2,3]
1408ce79f: 41 0f 59 dd                 	mulps	%xmm13, %xmm3
1408ce7a3: 44 0f 5c f3                 	subps	%xmm3, %xmm14
1408ce7a7: f3 0f 16 dd                 	movshdup	%xmm5, %xmm3            # xmm3 = xmm5[1,1,3,3]
1408ce7ab: f3 41 0f 59 dc              	mulss	%xmm12, %xmm3
1408ce7b0: 45 0f 28 fb                 	movaps	%xmm11, %xmm15
1408ce7b4: f3 44 0f 59 fd              	mulss	%xmm5, %xmm15
1408ce7b9: f3 41 0f 5c df              	subss	%xmm15, %xmm3
1408ce7be: 45 0f 58 f6                 	addps	%xmm14, %xmm14
1408ce7c2: f3 0f 58 db                 	addss	%xmm3, %xmm3
1408ce7c6: f3 45 0f 12 fe              	movsldup	%xmm14, %xmm15          # xmm15 = xmm14[0,0,2,2]
1408ce7cb: f3 44 0f 10 fb              	movss	%xmm3, %xmm15           # xmm15 = xmm3[0],xmm15[1,2,3]
1408ce7d0: 45 0f 59 fb                 	mulps	%xmm11, %xmm15
1408ce7d4: f3 44 0f 59 d3              	mulss	%xmm3, %xmm10
1408ce7d9: 41 0f c6 de d4              	shufps	$0xd4, %xmm14, %xmm3    # xmm3 = xmm3[0,1],xmm14[1,3]
1408ce7de: 41 0f c6 de e2              	shufps	$0xe2, %xmm14, %xmm3    # xmm3 = xmm3[2,0],xmm14[2,3]
1408ce7e3: 41 0f 59 dd                 	mulps	%xmm13, %xmm3
1408ce7e7: 44 0f 5c fb                 	subps	%xmm3, %xmm15
1408ce7eb: f3 41 0f 16 de              	movshdup	%xmm14, %xmm3           # xmm3 = xmm14[1,1,3,3]
1408ce7f0: f3 41 0f 59 dc              	mulss	%xmm12, %xmm3
1408ce7f5: f3 45 0f 59 de              	mulss	%xmm14, %xmm11
1408ce7fa: f3 41 0f 5c db              	subss	%xmm11, %xmm3
1408ce7ff: 45 0f 59 ce                 	mulps	%xmm14, %xmm9
1408ce803: 45 0f 58 cf                 	addps	%xmm15, %xmm9
1408ce807: f3 41 0f 58 da              	addss	%xmm10, %xmm3
1408ce80c: 44 0f 58 cd                 	addps	%xmm5, %xmm9
1408ce810: f3 0f 58 dc                 	addss	%xmm4, %xmm3
1408ce814: 49 8b 47 50                 	movq	0x50(%r15), %rax
1408ce818: 48 85 c0                    	testq	%rax, %rax
1408ce81b: 4c 8b 84 24 90 01 00 00     	movq	0x190(%rsp), %r8
1408ce823: 0f 84 b6 00 00 00           	je	0x1408ce8df <.text+0x8cd8df>
1408ce829: 49 8b 57 58                 	movq	0x58(%r15), %rdx
1408ce82d: 4c 39 da                    	cmpq	%r11, %rdx
1408ce830: 0f 86 a3 01 00 00           	jbe	0x1408ce9d9 <.text+0x8cd9d9>
1408ce836: 48 39 fa                    	cmpq	%rdi, %rdx
1408ce839: 0f 86 b8 01 00 00           	jbe	0x1408ce9f7 <.text+0x8cd9f7>
1408ce83f: 48 39 da                    	cmpq	%rbx, %rdx
1408ce842: 0f 86 be 01 00 00           	jbe	0x1408cea06 <.text+0x8cda06>
1408ce848: f2 42 0f 10 24 d8           	movsd	(%rax,%r11,8), %xmm4
1408ce84e: f3 0f 12 d2                 	movsldup	%xmm2, %xmm2            # xmm2 = xmm2[0,0,2,2]
1408ce852: 0f 59 d4                    	mulps	%xmm4, %xmm2
1408ce855: f2 0f 10 24 f8              	movsd	(%rax,%rdi,8), %xmm4
1408ce85a: f2 0f 10 2c d8              	movsd	(%rax,%rbx,8), %xmm5
1408ce85f: 0f 59 cc                    	mulps	%xmm4, %xmm1
1408ce862: 0f 58 ca                    	addps	%xmm2, %xmm1
1408ce865: 0f 59 c5                    	mulps	%xmm5, %xmm0
1408ce868: 0f 58 c1                    	addps	%xmm1, %xmm0
1408ce86b: b8 01 00 00 00              	movl	$0x1, %eax
1408ce870: eb 6f                       	jmp	0x1408ce8e1 <.text+0x8cd8e1>
1408ce872: f3 41 0f 10 40 14           	movss	0x14(%r8), %xmm0
1408ce878: f2 41 0f 10 48 0c           	movsd	0xc(%r8), %xmm1
1408ce87e: f2 0f 11 49 24              	movsd	%xmm1, 0x24(%rcx)
1408ce883: f3 0f 11 41 2c              	movss	%xmm0, 0x2c(%rcx)
1408ce888: 0f 10 00                    	movups	(%rax), %xmm0
1408ce88b: 0f 11 01                    	movups	%xmm0, (%rcx)
1408ce88e: 48 8b 40 10                 	movq	0x10(%rax), %rax
1408ce892: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
1408ce896: 0f 13 79 18                 	movlps	%xmm7, 0x18(%rcx)
1408ce89a: f3 44 0f 11 41 20           	movss	%xmm8, 0x20(%rcx)
1408ce8a0: f3 0f 11 71 30              	movss	%xmm6, 0x30(%rcx)
1408ce8a5: c7 41 34 00 00 00 00        	movl	$0x0, 0x34(%rcx)
1408ce8ac: eb 62                       	jmp	0x1408ce910 <.text+0x8cd910>
1408ce8ae: c7 41 34 02 00 00 00        	movl	$0x2, 0x34(%rcx)
1408ce8b5: eb 59                       	jmp	0x1408ce910 <.text+0x8cd910>
1408ce8b7: f2 44 0f 10 4e 0c           	movsd	0xc(%rsi), %xmm9
1408ce8bd: f3 0f 10 5e 14              	movss	0x14(%rsi), %xmm3
1408ce8c2: 4c 89 f1                    	movq	%r14, %rcx
1408ce8c5: 4c 8b 5c 24 30              	movq	0x30(%rsp), %r11
1408ce8ca: 49 8b 47 50                 	movq	0x50(%r15), %rax
1408ce8ce: 48 85 c0                    	testq	%rax, %rax
1408ce8d1: 4c 8b 84 24 90 01 00 00     	movq	0x190(%rsp), %r8
1408ce8d9: 0f 85 4a ff ff ff           	jne	0x1408ce829 <.text+0x8cd829>
1408ce8df: 31 c0                       	xorl	%eax, %eax
1408ce8e1: 44 0f 13 49 24              	movlps	%xmm9, 0x24(%rcx)
1408ce8e6: f3 0f 11 59 2c              	movss	%xmm3, 0x2c(%rcx)
1408ce8eb: 41 0f 10 08                 	movups	(%r8), %xmm1
1408ce8ef: 0f 11 09                    	movups	%xmm1, (%rcx)
1408ce8f2: 49 8b 50 10                 	movq	0x10(%r8), %rdx
1408ce8f6: 48 89 51 10                 	movq	%rdx, 0x10(%rcx)
1408ce8fa: 0f 13 79 18                 	movlps	%xmm7, 0x18(%rcx)
1408ce8fe: f3 44 0f 11 41 20           	movss	%xmm8, 0x20(%rcx)
1408ce904: f3 0f 11 71 30              	movss	%xmm6, 0x30(%rcx)
1408ce909: 89 41 34                    	movl	%eax, 0x34(%rcx)
1408ce90c: 0f 13 41 38                 	movlps	%xmm0, 0x38(%rcx)
1408ce910: 48 89 c8                    	movq	%rcx, %rax
1408ce913: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
1408ce91b: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
1408ce923: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1408ce92c: 44 0f 28 8c 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm9
1408ce935: 44 0f 28 94 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm10
1408ce93e: 44 0f 28 9c 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm11
1408ce947: 44 0f 28 a4 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm12
1408ce950: 44 0f 28 ac 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm13
1408ce959: 44 0f 28 b4 24 00 01 00 00  	movaps	0x100(%rsp), %xmm14
1408ce962: 44 0f 28 bc 24 10 01 00 00  	movaps	0x110(%rsp), %xmm15
1408ce96b: 48 81 c4 28 01 00 00        	addq	$0x128, %rsp            # imm = 0x128
1408ce972: 5b                          	popq	%rbx
1408ce973: 5d                          	popq	%rbp
1408ce974: 5f                          	popq	%rdi
1408ce975: 5e                          	popq	%rsi
1408ce976: 41 5c                       	popq	%r12
1408ce978: 41 5d                       	popq	%r13
1408ce97a: 41 5e                       	popq	%r14
1408ce97c: 41 5f                       	popq	%r15
1408ce97e: c3                          	retq
1408ce97f: 4c 8d 05 42 54 1d 02        	leaq	0x21d5442(%rip), %r8    # 0x142aa3dc8
1408ce986: 48 89 c1                    	movq	%rax, %rcx
1408ce989: e8 8a ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408ce98e: 4c 8d 05 4b 54 1d 02        	leaq	0x21d544b(%rip), %r8    # 0x142aa3de0
1408ce995: 48 89 f1                    	movq	%rsi, %rcx
1408ce998: e8 7b ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408ce99d: 4c 8d 05 54 54 1d 02        	leaq	0x21d5454(%rip), %r8    # 0x142aa3df8
1408ce9a4: 4c 89 d1                    	movq	%r10, %rcx
1408ce9a7: e8 6c ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408ce9ac: 4c 8d 05 5d 54 1d 02        	leaq	0x21d545d(%rip), %r8    # 0x142aa3e10
1408ce9b3: 4c 89 d9                    	movq	%r11, %rcx
1408ce9b6: e8 5d ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408ce9bb: 4c 8d 05 66 54 1d 02        	leaq	0x21d5466(%rip), %r8    # 0x142aa3e28
1408ce9c2: 4c 89 d9                    	movq	%r11, %rcx
1408ce9c5: e8 4e ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408ce9ca: 4c 8d 05 6f 54 1d 02        	leaq	0x21d546f(%rip), %r8    # 0x142aa3e40
1408ce9d1: 48 89 f9                    	movq	%rdi, %rcx
1408ce9d4: e8 3f ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408ce9d9: 4c 8d 05 90 54 1d 02        	leaq	0x21d5490(%rip), %r8    # 0x142aa3e70
1408ce9e0: 4c 89 d9                    	movq	%r11, %rcx
1408ce9e3: e8 30 ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408ce9e8: 4c 8d 05 69 54 1d 02        	leaq	0x21d5469(%rip), %r8    # 0x142aa3e58
1408ce9ef: 48 89 d9                    	movq	%rbx, %rcx
1408ce9f2: e8 21 ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408ce9f7: 4c 8d 05 8a 54 1d 02        	leaq	0x21d548a(%rip), %r8    # 0x142aa3e88
1408ce9fe: 48 89 f9                    	movq	%rdi, %rcx
1408cea01: e8 12 ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408cea06: 4c 8d 05 93 54 1d 02        	leaq	0x21d5493(%rip), %r8    # 0x142aa3ea0
1408cea0d: 48 89 d9                    	movq	%rbx, %rcx
1408cea10: e8 03 ab 00 02              	callq	0x1428d9518 <.text+0x28d8518>
1408cea15: cc                          	int3
1408cea16: cc                          	int3
1408cea17: cc                          	int3
1408cea18: cc                          	int3
1408cea19: cc                          	int3
1408cea1a: cc                          	int3
1408cea1b: cc                          	int3
1408cea1c: cc                          	int3
1408cea1d: cc                          	int3
1408cea1e: cc                          	int3
1408cea1f: cc                          	int3
1408cea20: 0f b6 01                    	movzbl	(%rcx), %eax
1408cea23: 48 8d 0d eb 59 1d 02        	leaq	0x21d59eb(%rip), %rcx   # 0x142aa4415
1408cea2a: 0f b6 04 08                 	movzbl	(%rax,%rcx), %eax
1408cea2e: c3                          	retq
1408cea2f: cc                          	int3
1408cea30: 48 8b 41 18                 	movq	0x18(%rcx), %rax
1408cea34: 48 8b 80 80 00 00 00        	movq	0x80(%rax), %rax
1408cea3b: 4c 8b 42 18                 	movq	0x18(%rdx), %r8
1408cea3f: 49 3b 80 80 00 00 00        	cmpq	0x80(%r8), %rax
1408cea46: 0f 85 b8 00 00 00           	jne	0x1408ceb04 <.text+0x8cdb04>
1408cea4c: 44 0f b6 01                 	movzbl	(%rcx), %r8d
1408cea50: 44 3a 02                    	cmpb	(%rdx), %r8b
1408cea53: 0f 85 ab 00 00 00           	jne	0x1408ceb04 <.text+0x8cdb04>
1408cea59: b0 01                       	movb	$0x1, %al
1408cea5b: 41 83 f8 10                 	cmpl	$0x10, %r8d
1408cea5f: 0f 87 82 00 00 00           	ja	0x1408ceae7 <.text+0x8cdae7>
1408cea65: 45 89 c0                    	movl	%r8d, %r8d
1408cea68: 4c 8d 0d 85 5a 1d 02        	leaq	0x21d5a85(%rip), %r9    # 0x142aa44f4
1408cea6f: 4f 63 04 81                 	movslq	(%r9,%r8,4), %r8
1408cea73: 4d 01 c8                    	addq	%r9, %r8
1408cea76: 41 ff e0                    	jmpq	*%r8
1408cea79: 48 8b 41 08                 	movq	0x8(%rcx), %rax
1408cea7d: 48 3b 42 08                 	cmpq	0x8(%rdx), %rax
