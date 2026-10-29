
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14083d3f0: 41 57                       	pushq	%r15
14083d3f2: 41 56                       	pushq	%r14
14083d3f4: 41 55                       	pushq	%r13
14083d3f6: 41 54                       	pushq	%r12
14083d3f8: 56                          	pushq	%rsi
14083d3f9: 57                          	pushq	%rdi
14083d3fa: 55                          	pushq	%rbp
14083d3fb: 53                          	pushq	%rbx
14083d3fc: 48 81 ec d8 01 00 00        	subq	$0x1d8, %rsp            # imm = 0x1D8
14083d403: 44 0f 29 a4 24 c0 01 00 00  	movaps	%xmm12, 0x1c0(%rsp)
14083d40c: 44 0f 29 9c 24 b0 01 00 00  	movaps	%xmm11, 0x1b0(%rsp)
14083d415: 44 0f 29 94 24 a0 01 00 00  	movaps	%xmm10, 0x1a0(%rsp)
14083d41e: 44 0f 29 8c 24 90 01 00 00  	movaps	%xmm9, 0x190(%rsp)
14083d427: 44 0f 29 84 24 80 01 00 00  	movaps	%xmm8, 0x180(%rsp)
14083d430: 0f 29 bc 24 70 01 00 00     	movaps	%xmm7, 0x170(%rsp)
14083d438: 0f 29 b4 24 60 01 00 00     	movaps	%xmm6, 0x160(%rsp)
14083d440: 4d 89 ce                    	movq	%r9, %r14
14083d443: 4c 89 c5                    	movq	%r8, %rbp
14083d446: 48 89 d7                    	movq	%rdx, %rdi
14083d449: 48 89 ce                    	movq	%rcx, %rsi
14083d44c: 4d 8d b8 88 00 00 00        	leaq	0x88(%r8), %r15
14083d453: 48 8d 59 08                 	leaq	0x8(%rcx), %rbx
14083d457: 48 8b 51 20                 	movq	0x20(%rcx), %rdx
14083d45b: 48 89 d9                    	movq	%rbx, %rcx
14083d45e: e8 9d c1 45 00              	callq	0x140c99600 <.text+0xc98600>
14083d463: 0f 28 d1                    	movaps	%xmm1, %xmm2
14083d466: 4c 89 f9                    	movq	%r15, %rcx
14083d469: 0f 28 c8                    	movaps	%xmm0, %xmm1
14083d46c: e8 8f c9 c2 00              	callq	0x141469e00 <.text+0x1468e00>
14083d471: 41 89 c7                    	movl	%eax, %r15d
14083d474: 0f 28 f0                    	movaps	%xmm0, %xmm6
14083d477: 48 8b 56 20                 	movq	0x20(%rsi), %rdx
14083d47b: 48 89 d9                    	movq	%rbx, %rcx
14083d47e: e8 7d c1 45 00              	callq	0x140c99600 <.text+0xc98600>
14083d483: f3 0f 11 44 24 60           	movss	%xmm0, 0x60(%rsp)
14083d489: f3 0f 11 4c 24 64           	movss	%xmm1, 0x64(%rsp)
14083d48f: 45 89 ff                    	movl	%r15d, %r15d
14083d492: 48 8b 95 98 00 00 00        	movq	0x98(%rbp), %rdx
14083d499: 4c 39 fa                    	cmpq	%r15, %rdx
14083d49c: 0f 86 00 05 00 00           	jbe	0x14083d9a2 <.text+0x83c9a2>
14083d4a2: 4d 8d 6f 01                 	leaq	0x1(%r15), %r13
14083d4a6: 49 39 d5                    	cmpq	%rdx, %r13
14083d4a9: 0f 83 02 05 00 00           	jae	0x14083d9b1 <.text+0x83c9b1>
14083d4af: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
14083d4b3: 0f 28 f9                    	movaps	%xmm1, %xmm7
14083d4b6: 4c 8b a4 24 40 02 00 00     	movq	0x240(%rsp), %r12
14083d4be: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
14083d4c5: 4b 8d 0c 7f                 	leaq	(%r15,%r15,2), %rcx
14083d4c9: 4b 8d 54 6d 00              	leaq	(%r13,%r13,2), %rdx
14083d4ce: f3 44 0f 10 15 2d 84 0e 02  	movss	0x20e842d(%rip), %xmm10 # 0x142925904
14083d4d7: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
14083d4db: f3 0f 5c c6                 	subss	%xmm6, %xmm0
14083d4df: f3 0f 59 44 88 04           	mulss	0x4(%rax,%rcx,4), %xmm0
14083d4e5: f3 44 0f 10 5c 90 04        	movss	0x4(%rax,%rdx,4), %xmm11
14083d4ec: f3 44 0f 59 de              	mulss	%xmm6, %xmm11
14083d4f1: f3 44 0f 58 d8              	addss	%xmm0, %xmm11
14083d4f6: f3 45 0f 58 da              	addss	%xmm10, %xmm11
14083d4fb: f3 44 0f 11 5c 24 34        	movss	%xmm11, 0x34(%rsp)
14083d502: f3 44 0f 11 5e 28           	movss	%xmm11, 0x28(%rsi)
14083d508: 80 7e 2c 00                 	cmpb	$0x0, 0x2c(%rsi)
14083d50c: 74 0b                       	je	0x14083d519 <.text+0x83c519>
14083d50e: f3 44 0f 10 0d 39 84 0e 02  	movss	0x20e8439(%rip), %xmm9  # 0x142925950
14083d517: eb 3c                       	jmp	0x14083d555 <.text+0x83c555>
14083d519: 4d 39 e7                    	cmpq	%r12, %r15
14083d51c: 0f 83 9e 04 00 00           	jae	0x14083d9c0 <.text+0x83c9c0>
14083d522: 4d 39 e5                    	cmpq	%r12, %r13
14083d525: 0f 83 a7 04 00 00           	jae	0x14083d9d2 <.text+0x83c9d2>
14083d52b: f3 43 0f 10 04 be           	movss	(%r14,%r15,4), %xmm0
14083d531: f3 47 0f 10 4c be 04        	movss	0x4(%r14,%r15,4), %xmm9
14083d538: f3 44 0f 5c c8              	subss	%xmm0, %xmm9
14083d53d: f3 44 0f 59 ce              	mulss	%xmm6, %xmm9
14083d542: f3 44 0f 58 c8              	addss	%xmm0, %xmm9
14083d547: f3 45 0f 5c cb              	subss	%xmm11, %xmm9
14083d54c: f3 44 0f 59 0d 1b 83 0e 02  	mulss	0x20e831b(%rip), %xmm9  # 0x142925870
14083d555: 48 8b 56 20                 	movq	0x20(%rsi), %rdx
14083d559: 48 89 d9                    	movq	%rbx, %rcx
14083d55c: e8 df cb 45 00              	callq	0x140c9a140 <.text+0xc99140>
14083d561: 80 7e 2c 00                 	cmpb	$0x0, 0x2c(%rsi)
14083d565: 0f 84 c0 00 00 00           	je	0x14083d62b <.text+0x83c62b>
14083d56b: 4c 8b 46 20                 	movq	0x20(%rsi), %r8
14083d56f: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
14083d574: 48 89 da                    	movq	%rbx, %rdx
14083d577: e8 94 cc 45 00              	callq	0x140c9a210 <.text+0xc99210>
14083d57c: f2 0f 10 44 24 68           	movsd	0x68(%rsp), %xmm0
14083d582: f2 0f 10 4c 24 70           	movsd	0x70(%rsp), %xmm1
14083d588: 0f 28 15 61 83 0e 02        	movaps	0x20e8361(%rip), %xmm2  # 0x1429258f0
14083d58f: f2 0f 11 44 24 38           	movsd	%xmm0, 0x38(%rsp)
14083d595: f3 0f 16 d8                 	movshdup	%xmm0, %xmm3            # xmm3 = xmm0[1,1,3,3]
14083d599: 0f 57 c2                    	xorps	%xmm2, %xmm0
14083d59c: f3 0f 11 5c 24 40           	movss	%xmm3, 0x40(%rsp)
14083d5a2: f3 0f 11 44 24 44           	movss	%xmm0, 0x44(%rsp)
14083d5a8: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14083d5ac: 0f 57 c2                    	xorps	%xmm2, %xmm0
14083d5af: f2 0f 11 4c 24 48           	movsd	%xmm1, 0x48(%rsp)
14083d5b5: f3 0f 11 44 24 50           	movss	%xmm0, 0x50(%rsp)
14083d5bb: f3 0f 11 4c 24 54           	movss	%xmm1, 0x54(%rsp)
14083d5c1: 48 8d 44 24 58              	leaq	0x58(%rsp), %rax
14083d5c6: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
14083d5cb: 48 89 8c 24 90 00 00 00     	movq	%rcx, 0x90(%rsp)
14083d5d3: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
14083d5db: 48 8d 44 24 60              	leaq	0x60(%rsp), %rax
14083d5e0: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
14083d5e8: 48 8d 44 24 34              	leaq	0x34(%rsp), %rax
14083d5ed: 48 89 84 24 a8 00 00 00     	movq	%rax, 0xa8(%rsp)
14083d5f5: 48 c7 84 24 b0 00 00 00 00 00 00 00 	movq	$0x0, 0xb0(%rsp)
14083d601: 48 c7 84 24 d0 00 00 00 00 00 00 00 	movq	$0x0, 0xd0(%rsp)
14083d60d: 4c 8d 05 44 5d 25 02        	leaq	0x2255d44(%rip), %r8    # 0x142a93358
14083d614: 48 8d 4c 24 78              	leaq	0x78(%rsp), %rcx
14083d619: 48 8d 94 24 90 00 00 00     	leaq	0x90(%rsp), %rdx
14083d621: e8 8a 71 ac 00              	callq	0x1413047b0 <.text+0x13037b0>
14083d626: e9 4e 02 00 00              	jmp	0x14083d879 <.text+0x83c879>
14083d62b: 4d 39 e7                    	cmpq	%r12, %r15
14083d62e: 0f 83 b0 03 00 00           	jae	0x14083d9e4 <.text+0x83c9e4>
14083d634: 4d 39 e5                    	cmpq	%r12, %r13
14083d637: 0f 83 b9 03 00 00           	jae	0x14083d9f6 <.text+0x83c9f6>
14083d63d: f3 0f 10 15 93 82 0e 02     	movss	0x20e8293(%rip), %xmm2  # 0x1429258d8
14083d645: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
14083d649: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
14083d64d: f3 43 0f 10 14 be           	movss	(%r14,%r15,4), %xmm2
14083d653: f3 43 0f 10 5c be 04        	movss	0x4(%r14,%r15,4), %xmm3
14083d65a: f3 0f 5c da                 	subss	%xmm2, %xmm3
14083d65e: f3 0f 59 f3                 	mulss	%xmm3, %xmm6
14083d662: f3 0f 58 f2                 	addss	%xmm2, %xmm6
14083d666: f3 41 0f 5c f3              	subss	%xmm11, %xmm6
14083d66b: f3 44 0f 5c c0              	subss	%xmm0, %xmm8
14083d670: f3 45 0f 58 cb              	addss	%xmm11, %xmm9
14083d675: f3 0f 5c f9                 	subss	%xmm1, %xmm7
14083d679: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
14083d67e: 48 89 da                    	movq	%rbx, %rdx
14083d681: e8 ea be 45 00              	callq	0x140c99570 <.text+0xc98570>
14083d686: f3 0f 10 64 24 38           	movss	0x38(%rsp), %xmm4
14083d68c: f3 0f 10 5c 24 3c           	movss	0x3c(%rsp), %xmm3
14083d692: f2 0f 10 4c 24 40           	movsd	0x40(%rsp), %xmm1
14083d698: f3 44 0f 10 5c 24 48        	movss	0x48(%rsp), %xmm11
14083d69f: f2 0f 10 54 24 4c           	movsd	0x4c(%rsp), %xmm2
14083d6a5: f3 0f 10 44 24 54           	movss	0x54(%rsp), %xmm0
14083d6ab: f3 0f 10 6c 24 58           	movss	0x58(%rsp), %xmm5
14083d6b1: 45 0f 57 e4                 	xorps	%xmm12, %xmm12
14083d6b5: 44 0f 2e e5                 	ucomiss	%xmm5, %xmm12
14083d6b9: 73 57                       	jae	0x14083d712 <.text+0x83c712>
14083d6bb: f3 41 0f 58 e3              	addss	%xmm11, %xmm4
14083d6c0: f3 0f 58 2d 3c 82 0e 02     	addss	0x20e823c(%rip), %xmm5  # 0x142925904
14083d6c8: 44 0f 2e e4                 	ucomiss	%xmm4, %xmm12
14083d6cc: 0f 83 9d 00 00 00           	jae	0x14083d76f <.text+0x83c76f>
14083d6d2: 0f 16 e3                    	movlhps	%xmm3, %xmm4            # xmm4 = xmm4[0],xmm3[0]
14083d6d5: 0f c6 da 30                 	shufps	$0x30, %xmm2, %xmm3     # xmm3 = xmm3[0,0],xmm2[3,0]
14083d6d9: 44 0f 28 d2                 	movaps	%xmm2, %xmm10
14083d6dd: 44 0f c6 d3 84              	shufps	$0x84, %xmm3, %xmm10    # xmm10 = xmm10[0,1],xmm3[0,2]
14083d6e2: 0f c6 d4 24                 	shufps	$0x24, %xmm4, %xmm2     # xmm2 = xmm2[0,1],xmm4[2,0]
14083d6e6: 0f 16 c1                    	movlhps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0]
14083d6e9: 0f 28 d8                    	movaps	%xmm0, %xmm3
14083d6ec: 0f c6 d9 d8                 	shufps	$0xd8, %xmm1, %xmm3     # xmm3 = xmm3[0,2],xmm1[1,3]
14083d6f0: 44 0f 5c d3                 	subps	%xmm3, %xmm10
14083d6f4: 0f c6 e9 d4                 	shufps	$0xd4, %xmm1, %xmm5     # xmm5 = xmm5[0,1],xmm1[1,3]
14083d6f8: 0f c6 c5 28                 	shufps	$0x28, %xmm5, %xmm0     # xmm0 = xmm0[0,2],xmm5[2,0]
14083d6fc: 0f 58 c2                    	addps	%xmm2, %xmm0
14083d6ff: 0f 28 c8                    	movaps	%xmm0, %xmm1
14083d702: 41 0f c6 ca 23              	shufps	$0x23, %xmm10, %xmm1    # xmm1 = xmm1[3,0],xmm10[2,0]
14083d707: 44 0f c6 d1 24              	shufps	$0x24, %xmm1, %xmm10    # xmm10 = xmm10[0,1],xmm1[2,0]
14083d70c: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
14083d710: eb 45                       	jmp	0x14083d757 <.text+0x83c757>
14083d712: f3 44 0f 5c dc              	subss	%xmm4, %xmm11
14083d717: f3 44 0f 5c d5              	subss	%xmm5, %xmm10
14083d71c: 45 0f 2e e3                 	ucomiss	%xmm11, %xmm12
14083d720: 0f 83 95 00 00 00           	jae	0x14083d7bb <.text+0x83c7bb>
14083d726: 41 0f 14 db                 	unpcklps	%xmm11, %xmm3           # xmm3 = xmm3[0],xmm11[0],xmm3[1],xmm11[1]
14083d72a: 0f 16 da                    	movlhps	%xmm2, %xmm3            # xmm3 = xmm3[0],xmm2[0]
14083d72d: 0f 28 d1                    	movaps	%xmm1, %xmm2
14083d730: 0f c6 d1 25                 	shufps	$0x25, %xmm1, %xmm2     # xmm2 = xmm2[1,1],xmm1[2,0]
14083d734: 44 0f c6 d1 d4              	shufps	$0xd4, %xmm1, %xmm10    # xmm10 = xmm10[0,1],xmm1[1,3]
14083d739: 0f 16 c1                    	movlhps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0]
14083d73c: 44 0f c6 d0 82              	shufps	$0x82, %xmm0, %xmm10    # xmm10 = xmm10[2,0],xmm0[0,2]
14083d741: 44 0f 58 d3                 	addps	%xmm3, %xmm10
14083d745: 0f 5c da                    	subps	%xmm2, %xmm3
14083d748: 41 0f c6 da 23              	shufps	$0x23, %xmm10, %xmm3    # xmm3 = xmm3[3,0],xmm10[2,0]
14083d74d: f3 41 0f 16 c2              	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
14083d752: 44 0f c6 d3 24              	shufps	$0x24, %xmm3, %xmm10    # xmm10 = xmm10[0,1],xmm3[2,0]
14083d757: 0f 57 c9                    	xorps	%xmm1, %xmm1
14083d75a: f3 0f 51 c8                 	sqrtss	%xmm0, %xmm1
14083d75e: f3 0f 10 05 0a 81 0e 02     	movss	0x20e810a(%rip), %xmm0  # 0x142925870
14083d766: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14083d76a: e9 9b 00 00 00              	jmp	0x14083d80a <.text+0x83c80a>
14083d76f: 44 0f 28 d2                 	movaps	%xmm2, %xmm10
14083d773: 44 0f c6 d2 e1              	shufps	$0xe1, %xmm2, %xmm10    # xmm10 = xmm10[1,0],xmm2[2,3]
14083d778: 0f 16 dd                    	movlhps	%xmm5, %xmm3            # xmm3 = xmm3[0],xmm5[0]
14083d77b: 0f c6 d3 21                 	shufps	$0x21, %xmm3, %xmm2     # xmm2 = xmm2[1,0],xmm3[2,0]
14083d77f: 0f 16 c1                    	movlhps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0]
14083d782: 0f 28 d8                    	movaps	%xmm0, %xmm3
14083d785: 0f c6 d9 42                 	shufps	$0x42, %xmm1, %xmm3     # xmm3 = xmm3[2,0],xmm1[0,1]
14083d789: 41 0f 58 da                 	addps	%xmm10, %xmm3
14083d78d: 0f c6 e1 d4                 	shufps	$0xd4, %xmm1, %xmm4     # xmm4 = xmm4[0,1],xmm1[1,3]
14083d791: 0f c6 c4 82                 	shufps	$0x82, %xmm4, %xmm0     # xmm0 = xmm0[2,0],xmm4[0,2]
14083d795: 0f 5c d0                    	subps	%xmm0, %xmm2
14083d798: 0f c6 da e4                 	shufps	$0xe4, %xmm2, %xmm3     # xmm3 = xmm3[0,1],xmm2[2,3]
14083d79c: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
14083d79f: 0f 57 c9                    	xorps	%xmm1, %xmm1
14083d7a2: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
14083d7a6: f3 0f 10 05 c2 80 0e 02     	movss	0x20e80c2(%rip), %xmm0  # 0x142925870
14083d7ae: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14083d7b2: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14083d7b6: 0f 59 c3                    	mulps	%xmm3, %xmm0
14083d7b9: eb 57                       	jmp	0x14083d812 <.text+0x83c812>
14083d7bb: 44 0f 14 d3                 	unpcklps	%xmm3, %xmm10           # xmm10 = xmm10[0],xmm3[0],xmm10[1],xmm3[1]
14083d7bf: 41 0f 28 da                 	movaps	%xmm10, %xmm3
14083d7c3: 0f 16 d9                    	movlhps	%xmm1, %xmm3            # xmm3 = xmm3[0],xmm1[0]
14083d7c6: 0f 28 e2                    	movaps	%xmm2, %xmm4
14083d7c9: 0f 16 e1                    	movlhps	%xmm1, %xmm4            # xmm4 = xmm4[0],xmm1[0]
14083d7cc: 44 0f c6 d4 24              	shufps	$0x24, %xmm4, %xmm10    # xmm10 = xmm10[0,1],xmm4[2,0]
14083d7d1: 0f 28 e1                    	movaps	%xmm1, %xmm4
14083d7d4: f3 41 0f 10 e3              	movss	%xmm11, %xmm4           # xmm4 = xmm11[0],xmm4[1,2,3]
14083d7d9: 0f c6 ca 55                 	shufps	$0x55, %xmm2, %xmm1     # xmm1 = xmm1[1,1],xmm2[1,1]
14083d7dd: 0f 58 cb                    	addps	%xmm3, %xmm1
14083d7e0: 0f c6 c2 d4                 	shufps	$0xd4, %xmm2, %xmm0     # xmm0 = xmm0[0,1],xmm2[1,3]
14083d7e4: 0f c6 e0 24                 	shufps	$0x24, %xmm0, %xmm4     # xmm4 = xmm4[0,1],xmm0[2,0]
14083d7e8: 44 0f 5c d4                 	subps	%xmm4, %xmm10
14083d7ec: 0f 57 d2                    	xorps	%xmm2, %xmm2
14083d7ef: f3 41 0f 51 d2              	sqrtss	%xmm10, %xmm2
14083d7f4: 44 0f c6 d1 9c              	shufps	$0x9c, %xmm1, %xmm10    # xmm10 = xmm10[0,3],xmm1[1,2]
14083d7f9: 45 0f c6 d2 78              	shufps	$0x78, %xmm10, %xmm10   # xmm10 = xmm10[0,2,3,1]
14083d7fe: f3 0f 10 05 6a 80 0e 02     	movss	0x20e806a(%rip), %xmm0  # 0x142925870
14083d806: f3 0f 5e c2                 	divss	%xmm2, %xmm0
14083d80a: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14083d80e: 41 0f 59 c2                 	mulps	%xmm10, %xmm0
14083d812: f3 44 0f 11 84 24 a0 00 00 00       	movss	%xmm8, 0xa0(%rsp)
14083d81c: f3 44 0f 11 8c 24 a4 00 00 00       	movss	%xmm9, 0xa4(%rsp)
14083d826: f3 0f 11 bc 24 a8 00 00 00  	movss	%xmm7, 0xa8(%rsp)
14083d82f: 0f 29 84 24 90 00 00 00     	movaps	%xmm0, 0x90(%rsp)
14083d837: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
14083d841: 48 89 84 24 ac 00 00 00     	movq	%rax, 0xac(%rsp)
14083d849: c7 84 24 b4 00 00 00 00 00 80 3f    	movl	$0x3f800000, 0xb4(%rsp) # imm = 0x3F800000
14083d854: 48 8d 84 24 90 00 00 00     	leaq	0x90(%rsp), %rax
14083d85c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14083d861: 48 8d 4c 24 78              	leaq	0x78(%rsp), %rcx
14083d866: f3 0f 10 0d 62 80 0e 02     	movss	0x20e8062(%rip), %xmm1  # 0x1429258d0
14083d86e: 0f 28 d6                    	movaps	%xmm6, %xmm2
14083d871: 0f 28 d9                    	movaps	%xmm1, %xmm3
14083d874: e8 67 c8 29 00              	callq	0x140ada0e0 <.text+0xad90e0>
14083d879: 0f b6 84 24 48 02 00 00     	movzbl	0x248(%rsp), %eax
14083d881: 48 8b 8c 24 88 00 00 00     	movq	0x88(%rsp), %rcx
14083d889: 48 89 8c 24 58 01 00 00     	movq	%rcx, 0x158(%rsp)
14083d891: 0f 10 44 24 78              	movups	0x78(%rsp), %xmm0
14083d896: 0f 11 84 24 48 01 00 00     	movups	%xmm0, 0x148(%rsp)
14083d89e: 34 01                       	xorb	$0x1, %al
14083d8a0: 0f b6 c0                    	movzbl	%al, %eax
14083d8a3: 8d 0c 00                    	leal	(%rax,%rax), %ecx
14083d8a6: 8d 14 40                    	leal	(%rax,%rax,2), %edx
14083d8a9: 44 0f b6 46 2c              	movzbl	0x2c(%rsi), %r8d
14083d8ae: 41 80 f0 01                 	xorb	$0x1, %r8b
14083d8b2: 48 c7 84 24 90 00 00 00 00 00 00 00 	movq	$0x0, 0x90(%rsp)
14083d8be: 48 c7 84 24 98 00 00 00 08 00 00 00 	movq	$0x8, 0x98(%rsp)
14083d8ca: 48 c7 84 24 a0 00 00 00 00 00 00 00 	movq	$0x0, 0xa0(%rsp)
14083d8d6: 89 84 24 a8 00 00 00        	movl	%eax, 0xa8(%rsp)
14083d8dd: 48 c7 84 24 b0 00 00 00 00 00 00 00 	movq	$0x0, 0xb0(%rsp)
14083d8e9: 89 8c 24 c0 00 00 00        	movl	%ecx, 0xc0(%rsp)
14083d8f0: 89 94 24 d8 00 00 00        	movl	%edx, 0xd8(%rsp)
14083d8f7: 89 94 24 f0 00 00 00        	movl	%edx, 0xf0(%rsp)
14083d8fe: c6 84 24 08 01 00 00 00     	movb	$0x0, 0x108(%rsp)
14083d906: c6 84 24 18 01 00 00 02     	movb	$0x2, 0x118(%rsp)
14083d90e: 0f 10 84 24 41 01 00 00     	movups	0x141(%rsp), %xmm0
14083d916: 0f 11 84 24 19 01 00 00     	movups	%xmm0, 0x119(%rsp)
14083d91e: 0f 10 84 24 50 01 00 00     	movups	0x150(%rsp), %xmm0
14083d926: 0f 11 84 24 28 01 00 00     	movups	%xmm0, 0x128(%rsp)
14083d92e: 44 88 84 24 38 01 00 00     	movb	%r8b, 0x138(%rsp)
14083d936: c6 84 24 39 01 00 00 01     	movb	$0x1, 0x139(%rsp)
14083d93e: 4c 8d 84 24 90 00 00 00     	leaq	0x90(%rsp), %r8
14083d946: 48 89 f9                    	movq	%rdi, %rcx
14083d949: 48 89 f2                    	movq	%rsi, %rdx
14083d94c: e8 5f 56 0e 00              	callq	0x140922fb0 <.text+0x921fb0>
14083d951: 0f 28 b4 24 60 01 00 00     	movaps	0x160(%rsp), %xmm6
14083d959: 0f 28 bc 24 70 01 00 00     	movaps	0x170(%rsp), %xmm7
14083d961: 44 0f 28 84 24 80 01 00 00  	movaps	0x180(%rsp), %xmm8
14083d96a: 44 0f 28 8c 24 90 01 00 00  	movaps	0x190(%rsp), %xmm9
14083d973: 44 0f 28 94 24 a0 01 00 00  	movaps	0x1a0(%rsp), %xmm10
14083d97c: 44 0f 28 9c 24 b0 01 00 00  	movaps	0x1b0(%rsp), %xmm11
14083d985: 44 0f 28 a4 24 c0 01 00 00  	movaps	0x1c0(%rsp), %xmm12
14083d98e: 48 81 c4 d8 01 00 00        	addq	$0x1d8, %rsp            # imm = 0x1D8
14083d995: 5b                          	popq	%rbx
14083d996: 5d                          	popq	%rbp
14083d997: 5f                          	popq	%rdi
14083d998: 5e                          	popq	%rsi
14083d999: 41 5c                       	popq	%r12
14083d99b: 41 5d                       	popq	%r13
14083d99d: 41 5e                       	popq	%r14
14083d99f: 41 5f                       	popq	%r15
14083d9a1: c3                          	retq
14083d9a2: 4c 8d 05 af 5c 42 02        	leaq	0x2425caf(%rip), %r8    # 0x142c63658
14083d9a9: 4c 89 f9                    	movq	%r15, %rcx
14083d9ac: e8 67 bb 09 02              	callq	0x1428d9518 <.text+0x28d8518>
14083d9b1: 4c 8d 05 b8 5c 42 02        	leaq	0x2425cb8(%rip), %r8    # 0x142c63670
14083d9b8: 4c 89 e9                    	movq	%r13, %rcx
14083d9bb: e8 58 bb 09 02              	callq	0x1428d9518 <.text+0x28d8518>
14083d9c0: 4c 8d 05 59 5f 25 02        	leaq	0x2255f59(%rip), %r8    # 0x142a93920
14083d9c7: 4c 89 f9                    	movq	%r15, %rcx
14083d9ca: 4c 89 e2                    	movq	%r12, %rdx
14083d9cd: e8 46 bb 09 02              	callq	0x1428d9518 <.text+0x28d8518>
14083d9d2: 4c 8d 05 47 5f 25 02        	leaq	0x2255f47(%rip), %r8    # 0x142a93920
14083d9d9: 4c 89 e9                    	movq	%r13, %rcx
14083d9dc: 4c 89 e2                    	movq	%r12, %rdx
14083d9df: e8 34 bb 09 02              	callq	0x1428d9518 <.text+0x28d8518>
14083d9e4: 4c 8d 05 4d 5f 25 02        	leaq	0x2255f4d(%rip), %r8    # 0x142a93938
14083d9eb: 4c 89 f9                    	movq	%r15, %rcx
14083d9ee: 4c 89 e2                    	movq	%r12, %rdx
14083d9f1: e8 22 bb 09 02              	callq	0x1428d9518 <.text+0x28d8518>
14083d9f6: 4c 8d 05 3b 5f 25 02        	leaq	0x2255f3b(%rip), %r8    # 0x142a93938
14083d9fd: 4c 89 e9                    	movq	%r13, %rcx
14083da00: 4c 89 e2                    	movq	%r12, %rdx
14083da03: e8 10 bb 09 02              	callq	0x1428d9518 <.text+0x28d8518>
14083da08: cc                          	int3
