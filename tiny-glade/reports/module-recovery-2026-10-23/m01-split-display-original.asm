
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412782b0: 55                          	pushq	%rbp
1412782b1: 41 57                       	pushq	%r15
1412782b3: 41 56                       	pushq	%r14
1412782b5: 41 55                       	pushq	%r13
1412782b7: 41 54                       	pushq	%r12
1412782b9: 56                          	pushq	%rsi
1412782ba: 57                          	pushq	%rdi
1412782bb: 53                          	pushq	%rbx
1412782bc: 48 81 ec f8 01 00 00        	subq	$0x1f8, %rsp            # imm = 0x1F8
1412782c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1412782cb: 44 0f 29 bd 60 01 00 00     	movaps	%xmm15, 0x160(%rbp)
1412782d3: 44 0f 29 b5 50 01 00 00     	movaps	%xmm14, 0x150(%rbp)
1412782db: 44 0f 29 ad 40 01 00 00     	movaps	%xmm13, 0x140(%rbp)
1412782e3: 44 0f 29 a5 30 01 00 00     	movaps	%xmm12, 0x130(%rbp)
1412782eb: 44 0f 29 9d 20 01 00 00     	movaps	%xmm11, 0x120(%rbp)
1412782f3: 44 0f 29 95 10 01 00 00     	movaps	%xmm10, 0x110(%rbp)
1412782fb: 44 0f 29 8d 00 01 00 00     	movaps	%xmm9, 0x100(%rbp)
141278303: 44 0f 29 85 f0 00 00 00     	movaps	%xmm8, 0xf0(%rbp)
14127830b: 0f 29 bd e0 00 00 00        	movaps	%xmm7, 0xe0(%rbp)
141278312: 0f 29 b5 d0 00 00 00        	movaps	%xmm6, 0xd0(%rbp)
141278319: 48 c7 85 c8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xc8(%rbp)
141278324: 0f 28 11                    	movaps	(%rcx), %xmm2
141278327: 0f 28 c2                    	movaps	%xmm2, %xmm0
14127832a: 0f 59 c2                    	mulps	%xmm2, %xmm0
14127832d: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
141278331: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141278335: 0f 28 d8                    	movaps	%xmm0, %xmm3
141278338: 66 0f 15 d8                 	unpckhpd	%xmm0, %xmm3            # xmm3 = xmm3[1],xmm0[1]
14127833c: f3 0f 58 d9                 	addss	%xmm1, %xmm3
141278340: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
141278344: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
141278348: 0f 5c c3                    	subps	%xmm3, %xmm0
14127834b: 0f 28 0d 6e 4d 75 01        	movaps	0x1754d6e(%rip), %xmm1  # 0x1429cd0c0
141278352: 0f 59 c1                    	mulps	%xmm1, %xmm0
141278355: 0f 59 ca                    	mulps	%xmm2, %xmm1
141278358: f3 0f 16 d9                 	movshdup	%xmm1, %xmm3            # xmm3 = xmm1[1,1,3,3]
14127835c: f3 0f 58 d9                 	addss	%xmm1, %xmm3
141278360: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
141278363: f3 0f 58 cb                 	addss	%xmm3, %xmm1
141278367: f3 0f 58 c9                 	addss	%xmm1, %xmm1
14127836b: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
14127836f: 0f 59 ca                    	mulps	%xmm2, %xmm1
141278372: 0f 58 c8                    	addps	%xmm0, %xmm1
141278375: f3 0f 10 05 87 d5 6a 01     	movss	0x16ad587(%rip), %xmm0  # 0x142925904
14127837d: 0f 28 da                    	movaps	%xmm2, %xmm3
141278380: 0f 59 d8                    	mulps	%xmm0, %xmm3
141278383: 0f 28 e2                    	movaps	%xmm2, %xmm4
141278386: 0f c6 e2 c9                 	shufps	$0xc9, %xmm2, %xmm4     # xmm4 = xmm4[1,2],xmm2[0,3]
14127838a: 0f 59 e0                    	mulps	%xmm0, %xmm4
14127838d: 0f c6 db d2                 	shufps	$0xd2, %xmm3, %xmm3     # xmm3 = xmm3[2,0,1,3]
141278391: 0f 5c e3                    	subps	%xmm3, %xmm4
141278394: 0f 58 d2                    	addps	%xmm2, %xmm2
141278397: 0f c6 d2 ff                 	shufps	$0xff, %xmm2, %xmm2     # xmm2 = xmm2[3,3,3,3]
14127839b: 0f 59 d4                    	mulps	%xmm4, %xmm2
14127839e: 0f 58 d1                    	addps	%xmm1, %xmm2
1412783a1: f3 44 0f 10 6a 18           	movss	0x18(%rdx), %xmm13
1412783a7: f3 0f 10 69 18              	movss	0x18(%rcx), %xmm5
1412783ac: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
1412783b0: 0f 28 da                    	movaps	%xmm2, %xmm3
1412783b3: 0f c6 da e2                 	shufps	$0xe2, %xmm2, %xmm3     # xmm3 = xmm3[2,0],xmm2[2,3]
1412783b7: f2 0f 10 71 10              	movsd	0x10(%rcx), %xmm6
1412783bc: 41 0f 28 e5                 	movaps	%xmm13, %xmm4
1412783c0: f3 0f 5c e6                 	subss	%xmm6, %xmm4
1412783c4: f3 0f 59 e2                 	mulss	%xmm2, %xmm4
1412783c8: f2 44 0f 10 72 1c           	movsd	0x1c(%rdx), %xmm14
1412783ce: f2 0f 11 75 40              	movsd	%xmm6, 0x40(%rbp)
1412783d3: f3 0f 11 6d 48              	movss	%xmm5, 0x48(%rbp)
1412783d8: 0f c6 ee d4                 	shufps	$0xd4, %xmm6, %xmm5     # xmm5 = xmm5[0,1],xmm6[1,3]
1412783dc: 0f c6 ee f2                 	shufps	$0xf2, %xmm6, %xmm5     # xmm5 = xmm5[2,0],xmm6[3,3]
1412783e0: 41 0f 28 f6                 	movaps	%xmm14, %xmm6
1412783e4: 0f 5c f5                    	subps	%xmm5, %xmm6
1412783e7: 0f c6 d2 e9                 	shufps	$0xe9, %xmm2, %xmm2     # xmm2 = xmm2[1,2,2,3]
1412783eb: 0f 59 d6                    	mulps	%xmm6, %xmm2
1412783ee: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1412783f2: f3 0f 16 d2                 	movshdup	%xmm2, %xmm2            # xmm2 = xmm2[1,1,3,3]
1412783f6: f3 0f 58 d4                 	addss	%xmm4, %xmm2
1412783fa: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
1412783fe: f3 0f 12 d2                 	movsldup	%xmm2, %xmm2            # xmm2 = xmm2[0,0,2,2]
141278402: 0f 59 d3                    	mulps	%xmm3, %xmm2
141278405: 41 0f 28 19                 	movaps	(%r9), %xmm3
141278409: f3 41 0f 10 71 08           	movss	0x8(%r9), %xmm6
14127840f: 0f 28 ee                    	movaps	%xmm6, %xmm5
141278412: f3 0f 58 ee                 	addss	%xmm6, %xmm5
141278416: f3 0f 59 f5                 	mulss	%xmm5, %xmm6
14127841a: 44 0f 28 d3                 	movaps	%xmm3, %xmm10
14127841e: 44 0f 58 d3                 	addps	%xmm3, %xmm10
141278422: f3 41 0f 16 e2              	movshdup	%xmm10, %xmm4           # xmm4 = xmm10[1,1,3,3]
141278427: f3 0f 59 e3                 	mulss	%xmm3, %xmm4
14127842b: 44 0f 28 cb                 	movaps	%xmm3, %xmm9
14127842f: f3 44 0f 12 c5              	movsldup	%xmm5, %xmm8            # xmm8 = xmm5[0,0,2,2]
141278434: 44 0f 59 c3                 	mulps	%xmm3, %xmm8
141278438: 0f c6 db ff                 	shufps	$0xff, %xmm3, %xmm3     # xmm3 = xmm3[3,3,3,3]
14127843c: 45 0f 59 ca                 	mulps	%xmm10, %xmm9
141278440: 44 0f 59 d3                 	mulps	%xmm3, %xmm10
141278444: 0f 28 fb                    	movaps	%xmm3, %xmm7
141278447: f3 0f 59 fd                 	mulss	%xmm5, %xmm7
14127844b: f3 45 0f 16 d9              	movshdup	%xmm9, %xmm11           # xmm11 = xmm9[1,1,3,3]
141278450: 45 0f 28 e3                 	movaps	%xmm11, %xmm12
141278454: f3 44 0f 58 e6              	addss	%xmm6, %xmm12
141278459: f3 0f 10 1d a3 d4 6a 01     	movss	0x16ad4a3(%rip), %xmm3  # 0x142925904
141278461: 0f 28 eb                    	movaps	%xmm3, %xmm5
141278464: f3 41 0f 5c ec              	subss	%xmm12, %xmm5
141278469: 44 0f 28 e4                 	movaps	%xmm4, %xmm12
14127846d: f3 44 0f 58 e7              	addss	%xmm7, %xmm12
141278472: 41 0f 14 ec                 	unpcklps	%xmm12, %xmm5           # xmm5 = xmm5[0],xmm12[0],xmm5[1],xmm12[1]
141278476: f3 0f 5c e7                 	subss	%xmm7, %xmm4
14127847a: f3 41 0f 58 f1              	addss	%xmm9, %xmm6
14127847f: 0f 28 fb                    	movaps	%xmm3, %xmm7
141278482: f3 0f 5c fe                 	subss	%xmm6, %xmm7
141278486: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
14127848d: 45 0f c6 ee d4              	shufps	$0xd4, %xmm14, %xmm13   # xmm13 = xmm13[0,1],xmm14[1,3]
141278492: 45 0f c6 ee f2              	shufps	$0xf2, %xmm14, %xmm13   # xmm13 = xmm13[2,0],xmm14[3,3]
141278497: 41 0f b6 78 44              	movzbl	0x44(%r8), %edi
14127849c: 45 0f c6 d2 e1              	shufps	$0xe1, %xmm10, %xmm10   # xmm10 = xmm10[1,0,2,3]
1412784a1: 0f 14 e7                    	unpcklps	%xmm7, %xmm4            # xmm4 = xmm4[0],xmm7[0],xmm4[1],xmm7[1]
1412784a4: 41 0f 28 f8                 	movaps	%xmm8, %xmm7
1412784a8: 41 0f 5c fa                 	subps	%xmm10, %xmm7
1412784ac: 45 0f 58 d0                 	addps	%xmm8, %xmm10
1412784b0: 45 0f 10 41 1c              	movups	0x1c(%r9), %xmm8
1412784b5: f3 45 0f 58 cb              	addss	%xmm11, %xmm9
1412784ba: 0f 28 f3                    	movaps	%xmm3, %xmm6
1412784bd: f3 41 0f 5c f1              	subss	%xmm9, %xmm6
1412784c2: 45 0f 28 49 20              	movaps	0x20(%r9), %xmm9
1412784c7: 0f 16 ef                    	movlhps	%xmm7, %xmm5            # xmm5 = xmm5[0],xmm7[0]
1412784ca: 41 0f c6 e2 d4              	shufps	$0xd4, %xmm10, %xmm4    # xmm4 = xmm4[0,1],xmm10[1,3]
1412784cf: 41 0f c6 fa 01              	shufps	$0x1, %xmm10, %xmm7     # xmm7 = xmm7[1,0],xmm10[0,0]
1412784d4: 41 0f c6 f2 30              	shufps	$0x30, %xmm10, %xmm6    # xmm6 = xmm6[0,0],xmm10[3,0]
1412784d9: 0f c6 fe 82                 	shufps	$0x82, %xmm6, %xmm7     # xmm7 = xmm7[2,0],xmm6[0,2]
1412784dd: 45 0f c6 c0 00              	shufps	$0x0, %xmm8, %xmm8      # xmm8 = xmm8[0,0,0,0]
1412784e2: 45 0f 57 d2                 	xorps	%xmm10, %xmm10
1412784e6: f3 44 0f 2a d7              	cvtsi2ss	%edi, %xmm10
1412784eb: 0f 28 f3                    	movaps	%xmm3, %xmm6
1412784ee: f3 41 0f 5e f2              	divss	%xmm10, %xmm6
1412784f3: 45 0f 28 d1                 	movaps	%xmm9, %xmm10
1412784f7: 45 0f c6 d1 00              	shufps	$0x0, %xmm9, %xmm10     # xmm10 = xmm10[0,0],xmm9[0,0]
1412784fc: 45 0f c6 c9 55              	shufps	$0x55, %xmm9, %xmm9     # xmm9 = xmm9[1,1,1,1]
141278501: 41 0f b6 48 46              	movzbl	0x46(%r8), %ecx
141278506: 88 8d c7 00 00 00           	movb	%cl, 0xc7(%rbp)
14127850c: 48 89 f9                    	movq	%rdi, %rcx
14127850f: 48 ff c9                    	decq	%rcx
141278512: 78 0b                       	js	0x14127851f <.text+0x127751f>
141278514: 45 0f 57 db                 	xorps	%xmm11, %xmm11
141278518: f3 44 0f 2a d9              	cvtsi2ss	%ecx, %xmm11
14127851d: eb 1a                       	jmp	0x141278539 <.text+0x1277539>
14127851f: 48 89 ca                    	movq	%rcx, %rdx
141278522: 48 d1 ea                    	shrq	%rdx
141278525: 83 e1 01                    	andl	$0x1, %ecx
141278528: 48 09 d1                    	orq	%rdx, %rcx
14127852b: 45 0f 57 db                 	xorps	%xmm11, %xmm11
14127852f: f3 4c 0f 2a d9              	cvtsi2ss	%rcx, %xmm11
141278534: f3 45 0f 58 db              	addss	%xmm11, %xmm11
141278539: f3 44 0f 5c f1              	subss	%xmm1, %xmm14
14127853e: 44 0f 29 75 c0              	movaps	%xmm14, -0x40(%rbp)
141278543: 44 0f 5c ea                 	subps	%xmm2, %xmm13
141278547: 44 0f 29 6d b0              	movaps	%xmm13, -0x50(%rbp)
14127854c: 45 0f 28 61 10              	movaps	0x10(%r9), %xmm12
141278551: 44 0f 59 c5                 	mulps	%xmm5, %xmm8
141278555: 44 0f 59 d4                 	mulps	%xmm4, %xmm10
141278559: 44 0f 59 cf                 	mulps	%xmm7, %xmm9
14127855d: 44 0f 28 eb                 	movaps	%xmm3, %xmm13
141278561: f3 44 0f 5c ee              	subss	%xmm6, %xmm13
141278566: 31 c9                       	xorl	%ecx, %ecx
141278568: 80 bd c7 00 00 00 3a        	cmpb	$0x3a, 0xc7(%rbp)
14127856f: 0f 94 c1                    	sete	%cl
141278572: 66 0f 6e c9                 	movd	%ecx, %xmm1
141278576: 66 0f 70 c9 50              	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
14127857b: 66 0f 72 f1 1f              	pslld	$0x1f, %xmm1
141278580: 66 0f 72 e1 1f              	psrad	$0x1f, %xmm1
141278585: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
141278589: 66 0f df d0                 	pandn	%xmm0, %xmm2
14127858d: 66 0f db 0d 1b c0 6b 01     	pand	0x16bc01b(%rip), %xmm1  # 0x1429345b0
141278595: 66 0f eb ca                 	por	%xmm2, %xmm1
141278599: 0f c2 d9 00                 	cmpeqps	%xmm1, %xmm3
14127859d: 66 0f 70 c1 f5              	pshufd	$0xf5, %xmm1, %xmm0     # xmm0 = xmm1[1,1,3,3]
1412785a2: 66 0f ef d2                 	pxor	%xmm2, %xmm2
1412785a6: 0f c2 d0 00                 	cmpeqps	%xmm0, %xmm2
1412785aa: 0f 54 d3                    	andps	%xmm3, %xmm2
1412785ad: f3 0f 11 55 1c              	movss	%xmm2, 0x1c(%rbp)
1412785b2: 48 8b 08                    	movq	(%rax), %rcx
1412785b5: 48 89 4d 00                 	movq	%rcx, (%rbp)
1412785b9: 4c 8b 60 08                 	movq	0x8(%rax), %r12
1412785bd: f3 44 0f 7e f1              	movq	%xmm1, %xmm14           # xmm14 = xmm1[0],zero
1412785c2: f3 0f 10 35 a2 41 6b 01     	movss	0x16b41a2(%rip), %xmm6  # 0x14292c76c
1412785ca: 45 31 f6                    	xorl	%r14d, %r14d
1412785cd: f3 44 0f 10 3d 12 1b 6e 01  	movss	0x16e1b12(%rip), %xmm15 # 0x14295a0e8
1412785d6: 48 8d 5d 50                 	leaq	0x50(%rbp), %rbx
1412785da: 48 8d b5 88 00 00 00        	leaq	0x88(%rbp), %rsi
1412785e1: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
1412785e9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1412785f0: 49 39 fe                    	cmpq	%rdi, %r14
1412785f3: 0f 83 e6 03 00 00           	jae	0x1412789df <.text+0x12779df>
1412785f9: 4c 89 75 20                 	movq	%r14, 0x20(%rbp)
1412785fd: f6 45 1c 01                 	testb	$0x1, 0x1c(%rbp)
141278601: 74 4d                       	je	0x141278650 <.text+0x1277650>
141278603: 8b 8d e8 01 00 00           	movl	0x1e8(%rbp), %ecx
141278609: 48 8b 55 00                 	movq	(%rbp), %rdx
14127860d: e8 8e b5 5b ff              	callq	0x140833ba0 <.text+0x832ba0>
141278612: 89 f9                       	movl	%edi, %ecx
141278614: 89 c2                       	movl	%eax, %edx
141278616: e8 75 d6 8a ff              	callq	0x140b25c90 <.text+0xb24c90>
14127861b: 88 45 28                    	movb	%al, 0x28(%rbp)
14127861e: 88 55 29                    	movb	%dl, 0x29(%rbp)
141278621: 4c 89 e1                    	movq	%r12, %rcx
141278624: 48 8d 55 28                 	leaq	0x28(%rbp), %rdx
141278628: e8 63 49 fe ff              	callq	0x14125cf90 <.text+0x125bf90>
14127862d: 48 85 c0                    	testq	%rax, %rax
141278630: 0f 84 68 05 00 00           	je	0x141278b9e <.text+0x1277b9e>
141278636: f3 0f 10 40 30              	movss	0x30(%rax), %xmm0
14127863b: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14127863f: 48 85 c0                    	testq	%rax, %rax
141278642: 78 5f                       	js	0x1412786a3 <.text+0x12776a3>
141278644: 0f 57 ff                    	xorps	%xmm7, %xmm7
141278647: f3 48 0f 2a f8              	cvtsi2ss	%rax, %xmm7
14127864c: eb 6f                       	jmp	0x1412786bd <.text+0x12776bd>
14127864e: 66 90                       	nop
141278650: 80 bd c7 00 00 00 3a        	cmpb	$0x3a, 0xc7(%rbp)
141278657: 0f 85 29 05 00 00           	jne	0x141278b86 <.text+0x1277b86>
14127865d: 8b 8d e8 01 00 00           	movl	0x1e8(%rbp), %ecx
141278663: 48 8b 55 00                 	movq	(%rbp), %rdx
141278667: e8 34 b5 5b ff              	callq	0x140833ba0 <.text+0x832ba0>
14127866c: 89 f9                       	movl	%edi, %ecx
14127866e: 89 c2                       	movl	%eax, %edx
141278670: e8 1b d6 8a ff              	callq	0x140b25c90 <.text+0xb24c90>
141278675: 88 45 28                    	movb	%al, 0x28(%rbp)
141278678: 88 55 29                    	movb	%dl, 0x29(%rbp)
14127867b: 4c 89 e1                    	movq	%r12, %rcx
14127867e: 48 8d 55 28                 	leaq	0x28(%rbp), %rdx
141278682: e8 09 49 fe ff              	callq	0x14125cf90 <.text+0x125bf90>
141278687: 48 85 c0                    	testq	%rax, %rax
14127868a: 0f 84 0e 05 00 00           	je	0x141278b9e <.text+0x1277b9e>
141278690: f3 0f 10 40 3c              	movss	0x3c(%rax), %xmm0
141278695: f3 0f 5c 40 38              	subss	0x38(%rax), %xmm0
14127869a: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14127869e: 48 85 c0                    	testq	%rax, %rax
1412786a1: 79 a1                       	jns	0x141278644 <.text+0x1277644>
1412786a3: 48 89 c1                    	movq	%rax, %rcx
1412786a6: 48 d1 e9                    	shrq	%rcx
1412786a9: 89 c2                       	movl	%eax, %edx
1412786ab: 83 e2 01                    	andl	$0x1, %edx
1412786ae: 48 09 ca                    	orq	%rcx, %rdx
1412786b1: 0f 57 ff                    	xorps	%xmm7, %xmm7
1412786b4: f3 48 0f 2a fa              	cvtsi2ss	%rdx, %xmm7
1412786b9: f3 0f 58 ff                 	addss	%xmm7, %xmm7
1412786bd: f3 41 0f 5e fb              	divss	%xmm11, %xmm7
1412786c2: 49 ff c6                    	incq	%r14
1412786c5: f3 41 0f 59 c5              	mulss	%xmm13, %xmm0
1412786ca: f3 41 0f 58 ff              	addss	%xmm15, %xmm7
1412786cf: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
1412786d3: 0f c6 ff 00                 	shufps	$0x0, %xmm7, %xmm7      # xmm7 = xmm7[0,0,0,0]
1412786d7: 41 0f 59 fe                 	mulps	%xmm14, %xmm7
1412786db: 0f 28 c7                    	movaps	%xmm7, %xmm0
1412786de: 0f c6 c7 00                 	shufps	$0x0, %xmm7, %xmm0      # xmm0 = xmm0[0,0],xmm7[0,0]
1412786e2: 41 0f 59 c0                 	mulps	%xmm8, %xmm0
1412786e6: 0f 28 cf                    	movaps	%xmm7, %xmm1
1412786e9: 0f c6 cf 55                 	shufps	$0x55, %xmm7, %xmm1     # xmm1 = xmm1[1,1],xmm7[1,1]
1412786ed: 41 0f 59 ca                 	mulps	%xmm10, %xmm1
1412786f1: 0f 58 c8                    	addps	%xmm0, %xmm1
1412786f4: 0f c6 ff aa                 	shufps	$0xaa, %xmm7, %xmm7     # xmm7 = xmm7[2,2,2,2]
1412786f8: 41 0f 59 f9                 	mulps	%xmm9, %xmm7
1412786fc: 0f 58 f9                    	addps	%xmm1, %xmm7
1412786ff: 41 0f 58 fc                 	addps	%xmm12, %xmm7
141278703: 40 80 ff 03                 	cmpb	$0x3, %dil
141278707: 0f 85 43 01 00 00           	jne	0x141278850 <.text+0x1277850>
14127870d: 48 83 f8 01                 	cmpq	$0x1, %rax
141278711: 0f 85 39 01 00 00           	jne	0x141278850 <.text+0x1277850>
141278717: 48 8d 45 20                 	leaq	0x20(%rbp), %rax
14127871b: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14127871f: 48 8d 05 9a 4f b1 ff        	leaq	-0x4eb066(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
141278726: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14127872a: 48 8d 85 e8 01 00 00        	leaq	0x1e8(%rbp), %rax
141278731: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141278735: 48 8d 05 94 aa 5b ff        	leaq	-0xa4556c(%rip), %rax   # 0x1408331d0 <.text+0x8321d0>
14127873c: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141278740: 48 8d 05 d1 b2 9a 01        	leaq	0x19ab2d1(%rip), %rax   # 0x142c23a18
141278747: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14127874e: 48 c7 85 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rbp)
141278759: 49 89 dd                    	movq	%rbx, %r13
14127875c: 48 89 9d 98 00 00 00        	movq	%rbx, 0x98(%rbp)
141278763: 48 c7 85 a0 00 00 00 02 00 00 00    	movq	$0x2, 0xa0(%rbp)
14127876e: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
141278779: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
14127877d: 48 89 f2                    	movq	%rsi, %rdx
141278780: e8 0b 18 b0 ff              	callq	0x140d79f90 <.text+0xd78f90>
141278785: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
141278789: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
141278790: 4c 8b 7d f0                 	movq	-0x10(%rbp), %r15
141278794: 48 8b 5d f8                 	movq	-0x8(%rbp), %rbx
141278798: 0f b6 05 6a c4 3b 02        	movzbl	0x23bc46a(%rip), %eax   # 0x143634c09
14127879f: b9 20 00 00 00              	movl	$0x20, %ecx
1412787a4: ba 08 00 00 00              	movl	$0x8, %edx
1412787a9: e8 62 b4 39 ff              	callq	0x140613c10 <.text+0x612c10>
1412787ae: 48 85 c0                    	testq	%rax, %rax
1412787b1: 4c 89 bd 80 00 00 00        	movq	%r15, 0x80(%rbp)
1412787b8: 0f 84 3c 04 00 00           	je	0x141278bfa <.text+0x1277bfa>
1412787be: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
1412787c5: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
1412787cd: 0f 13 78 10                 	movlps	%xmm7, 0x10(%rax)
1412787d1: 0f 12 ff                    	movhlps	%xmm7, %xmm7            # xmm7 = xmm7[1,1]
1412787d4: f3 0f 11 78 18              	movss	%xmm7, 0x18(%rax)
1412787d9: 48 8d 0d 18 b3 9a 01        	leaq	0x19ab318(%rip), %rcx   # 0x142c23af8
1412787e0: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1412787e5: 48 89 f1                    	movq	%rsi, %rcx
1412787e8: 4c 89 fa                    	movq	%r15, %rdx
1412787eb: 49 89 d8                    	movq	%rbx, %r8
1412787ee: 49 89 c1                    	movq	%rax, %r9
1412787f1: e8 ba 50 62 ff              	callq	0x14089d8b0 <.text+0x89c8b0>
1412787f6: 4c 89 eb                    	movq	%r13, %rbx
1412787f9: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
141278800: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
141278804: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
14127880b: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
141278812: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
141278816: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
14127881a: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
141278821: 4c 89 ea                    	movq	%r13, %rdx
141278824: e8 67 47 8b ff              	callq	0x140b2cf90 <.text+0xb2bf90>
141278829: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
141278830: 48 85 d2                    	testq	%rdx, %rdx
141278833: 0f 84 b7 fd ff ff           	je	0x1412785f0 <.text+0x12775f0>
141278839: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14127883f: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
141278846: e8 d5 b3 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127884b: e9 a0 fd ff ff              	jmp	0x1412785f0 <.text+0x12775f0>
141278850: f3 0f 16 c7                 	movshdup	%xmm7, %xmm0            # xmm0 = xmm7[1,1,3,3]
141278854: f3 0f 5c 45 c0              	subss	-0x40(%rbp), %xmm0
141278859: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14127885d: 0f 28 cf                    	movaps	%xmm7, %xmm1
141278860: 0f c6 cf e2                 	shufps	$0xe2, %xmm7, %xmm1     # xmm1 = xmm1[2,0],xmm7[2,3]
141278864: 0f 5c 4d b0                 	subps	-0x50(%rbp), %xmm1
141278868: 0f 59 c9                    	mulps	%xmm1, %xmm1
14127886b: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14127886f: f3 0f 58 d0                 	addss	%xmm0, %xmm2
141278873: f3 0f 58 d1                 	addss	%xmm1, %xmm2
141278877: 0f 57 c0                    	xorps	%xmm0, %xmm0
14127887a: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
14127887e: f3 0f 11 b5 b8 00 00 00     	movss	%xmm6, 0xb8(%rbp)
141278886: 0f 2e f0                    	ucomiss	%xmm0, %xmm6
141278889: 0f 28 f7                    	movaps	%xmm7, %xmm6
14127888c: 66 0f 15 f7                 	unpckhpd	%xmm7, %xmm6            # xmm6 = xmm6[1],xmm7[1]
141278890: 76 15                       	jbe	0x1412788a7 <.text+0x12778a7>
141278892: 0f 13 7d 40                 	movlps	%xmm7, 0x40(%rbp)
141278896: f3 0f 11 75 48              	movss	%xmm6, 0x48(%rbp)
14127889b: f3 0f 11 85 b8 00 00 00     	movss	%xmm0, 0xb8(%rbp)
1412788a3: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
1412788a7: 48 8d 45 20                 	leaq	0x20(%rbp), %rax
1412788ab: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1412788af: 48 8d 05 0a 4e b1 ff        	leaq	-0x4eb1f6(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
1412788b6: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1412788ba: 48 8d 85 e8 01 00 00        	leaq	0x1e8(%rbp), %rax
1412788c1: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1412788c5: 48 8d 05 04 a9 5b ff        	leaq	-0xa456fc(%rip), %rax   # 0x1408331d0 <.text+0x8321d0>
1412788cc: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1412788d0: 48 8d 05 41 b1 9a 01        	leaq	0x19ab141(%rip), %rax   # 0x142c23a18
1412788d7: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
1412788de: 48 c7 85 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rbp)
1412788e9: 49 89 dd                    	movq	%rbx, %r13
1412788ec: 48 89 9d 98 00 00 00        	movq	%rbx, 0x98(%rbp)
1412788f3: 48 c7 85 a0 00 00 00 02 00 00 00    	movq	$0x2, 0xa0(%rbp)
1412788fe: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
141278909: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
14127890d: 48 89 f2                    	movq	%rsi, %rdx
141278910: e8 7b 16 b0 ff              	callq	0x140d79f90 <.text+0xd78f90>
141278915: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
141278919: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
141278920: 4c 8b 7d d8                 	movq	-0x28(%rbp), %r15
141278924: 48 8b 5d e0                 	movq	-0x20(%rbp), %rbx
141278928: 0f b6 05 da c2 3b 02        	movzbl	0x23bc2da(%rip), %eax   # 0x143634c09
14127892f: b9 20 00 00 00              	movl	$0x20, %ecx
141278934: ba 08 00 00 00              	movl	$0x8, %edx
141278939: e8 d2 b2 39 ff              	callq	0x140613c10 <.text+0x612c10>
14127893e: 48 85 c0                    	testq	%rax, %rax
141278941: 4c 89 7d 10                 	movq	%r15, 0x10(%rbp)
141278945: 0f 84 d1 02 00 00           	je	0x141278c1c <.text+0x1277c1c>
14127894b: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
141278952: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14127895a: 0f 13 78 10                 	movlps	%xmm7, 0x10(%rax)
14127895e: f3 0f 11 70 18              	movss	%xmm6, 0x18(%rax)
141278963: 48 8d 0d be b1 9a 01        	leaq	0x19ab1be(%rip), %rcx   # 0x142c23b28
14127896a: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14127896f: 48 89 f1                    	movq	%rsi, %rcx
141278972: 4c 89 fa                    	movq	%r15, %rdx
141278975: 49 89 d8                    	movq	%rbx, %r8
141278978: 49 89 c1                    	movq	%rax, %r9
14127897b: e8 30 4f 62 ff              	callq	0x14089d8b0 <.text+0x89c8b0>
141278980: 4c 89 eb                    	movq	%r13, %rbx
141278983: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
14127898a: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14127898e: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
141278995: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
14127899c: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
1412789a0: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
1412789a4: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
1412789ab: 4c 89 ea                    	movq	%r13, %rdx
1412789ae: e8 dd 45 8b ff              	callq	0x140b2cf90 <.text+0xb2bf90>
1412789b3: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
1412789ba: 48 85 d2                    	testq	%rdx, %rdx
1412789bd: f3 0f 10 b5 b8 00 00 00     	movss	0xb8(%rbp), %xmm6
1412789c5: 0f 84 25 fc ff ff           	je	0x1412785f0 <.text+0x12775f0>
1412789cb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1412789d1: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
1412789d5: e8 46 b2 39 ff              	callq	0x140613c20 <.text+0x612c20>
1412789da: e9 11 fc ff ff              	jmp	0x1412785f0 <.text+0x12775f0>
1412789df: 48 8d 45 40                 	leaq	0x40(%rbp), %rax
1412789e3: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1412789e7: 48 8d 05 e2 2d ad ff        	leaq	-0x52d21e(%rip), %rax   # 0x140d4b7d0 <.text+0xd4a7d0>
1412789ee: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1412789f2: 48 8d 85 e8 01 00 00        	leaq	0x1e8(%rbp), %rax
1412789f9: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1412789fd: 48 8d 05 cc a7 5b ff        	leaq	-0xa45834(%rip), %rax   # 0x1408331d0 <.text+0x8321d0>
141278a04: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141278a08: 48 8d 05 09 b0 9a 01        	leaq	0x19ab009(%rip), %rax   # 0x142c23a18
141278a0f: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
141278a16: 48 c7 85 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rbp)
141278a21: 48 89 9d 98 00 00 00        	movq	%rbx, 0x98(%rbp)
141278a28: 48 c7 85 a0 00 00 00 02 00 00 00    	movq	$0x2, 0xa0(%rbp)
141278a33: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
141278a3e: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
141278a42: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
141278a49: e8 42 15 b0 ff              	callq	0x140d79f90 <.text+0xd78f90>
141278a4e: 48 8b 7d 30                 	movq	0x30(%rbp), %rdi
141278a52: 48 8b 75 38                 	movq	0x38(%rbp), %rsi
141278a56: 8b 45 48                    	movl	0x48(%rbp), %eax
141278a59: 89 85 90 00 00 00           	movl	%eax, 0x90(%rbp)
141278a5f: 48 8b 45 40                 	movq	0x40(%rbp), %rax
141278a63: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
141278a6a: 0f b6 05 98 c1 3b 02        	movzbl	0x23bc198(%rip), %eax   # 0x143634c09
141278a71: b9 20 00 00 00              	movl	$0x20, %ecx
141278a76: ba 08 00 00 00              	movl	$0x8, %edx
141278a7b: e8 90 b1 39 ff              	callq	0x140613c10 <.text+0x612c10>
141278a80: 48 85 c0                    	testq	%rax, %rax
141278a83: 48 89 bd b8 00 00 00        	movq	%rdi, 0xb8(%rbp)
141278a8a: 0f 84 7b 01 00 00           	je	0x141278c0b <.text+0x1277c0b>
141278a90: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
141278a97: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
141278a9f: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
141278aa6: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
141278aaa: 8b 8d 90 00 00 00           	movl	0x90(%rbp), %ecx
141278ab0: 89 48 18                    	movl	%ecx, 0x18(%rax)
141278ab3: 48 8d 0d 7e af 9a 01        	leaq	0x19aaf7e(%rip), %rcx   # 0x142c23a38
141278aba: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
141278abf: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141278ac6: 48 89 fa                    	movq	%rdi, %rdx
141278ac9: 49 89 f0                    	movq	%rsi, %r8
141278acc: 49 89 c1                    	movq	%rax, %r9
141278acf: e8 dc 4d 62 ff              	callq	0x14089d8b0 <.text+0x89c8b0>
141278ad4: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
141278adb: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
141278adf: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
141278ae6: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
141278aed: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
141278af1: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
141278af5: 48 8d 55 50                 	leaq	0x50(%rbp), %rdx
141278af9: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
141278b00: e8 8b 44 8b ff              	callq	0x140b2cf90 <.text+0xb2bf90>
141278b05: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
141278b09: 48 85 d2                    	testq	%rdx, %rdx
141278b0c: 74 12                       	je	0x141278b20 <.text+0x1277b20>
141278b0e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141278b14: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
141278b1b: e8 00 b1 39 ff              	callq	0x140613c20 <.text+0x612c20>
141278b20: 48 8b 45 08                 	movq	0x8(%rbp), %rax
141278b24: 0f 28 b5 d0 00 00 00        	movaps	0xd0(%rbp), %xmm6
141278b2b: 0f 28 bd e0 00 00 00        	movaps	0xe0(%rbp), %xmm7
141278b32: 44 0f 28 85 f0 00 00 00     	movaps	0xf0(%rbp), %xmm8
141278b3a: 44 0f 28 8d 00 01 00 00     	movaps	0x100(%rbp), %xmm9
141278b42: 44 0f 28 95 10 01 00 00     	movaps	0x110(%rbp), %xmm10
141278b4a: 44 0f 28 9d 20 01 00 00     	movaps	0x120(%rbp), %xmm11
141278b52: 44 0f 28 a5 30 01 00 00     	movaps	0x130(%rbp), %xmm12
141278b5a: 44 0f 28 ad 40 01 00 00     	movaps	0x140(%rbp), %xmm13
141278b62: 44 0f 28 b5 50 01 00 00     	movaps	0x150(%rbp), %xmm14
141278b6a: 44 0f 28 bd 60 01 00 00     	movaps	0x160(%rbp), %xmm15
141278b72: 48 81 c4 f8 01 00 00        	addq	$0x1f8, %rsp            # imm = 0x1F8
141278b79: 5b                          	popq	%rbx
141278b7a: 5f                          	popq	%rdi
141278b7b: 5e                          	popq	%rsi
141278b7c: 41 5c                       	popq	%r12
141278b7e: 41 5d                       	popq	%r13
141278b80: 41 5e                       	popq	%r14
141278b82: 41 5f                       	popq	%r15
141278b84: 5d                          	popq	%rbp
141278b85: c3                          	retq
141278b86: 48 8d 0d db ae 9a 01        	leaq	0x19aaedb(%rip), %rcx   # 0x142c23a68
141278b8d: 4c 8d 05 4c af 9a 01        	leaq	0x19aaf4c(%rip), %r8    # 0x142c23ae0
141278b94: ba 28 00 00 00              	movl	$0x28, %edx
141278b99: e8 92 08 66 01              	callq	0x1428d9430 <.text+0x28d8430>
141278b9e: 48 8d 45 28                 	leaq	0x28(%rbp), %rax
141278ba2: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
141278ba6: 48 8d 05 43 43 fe ff        	leaq	-0x1bcbd(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
141278bad: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141278bb1: 48 8d 05 c8 90 9a 01        	leaq	0x19a90c8(%rip), %rax   # 0x142c21c80
141278bb8: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
141278bbf: 48 c7 85 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rbp)
141278bca: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
141278bd5: 48 89 9d 98 00 00 00        	movq	%rbx, 0x98(%rbp)
141278bdc: 48 c7 85 a0 00 00 00 01 00 00 00    	movq	$0x1, 0xa0(%rbp)
141278be7: 48 8d 15 a2 90 9a 01        	leaq	0x19a90a2(%rip), %rdx   # 0x142c21c90
141278bee: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141278bf5: e8 96 07 66 01              	callq	0x1428d9390 <.text+0x28d8390>
141278bfa: b9 08 00 00 00              	movl	$0x8, %ecx
141278bff: ba 20 00 00 00              	movl	$0x20, %edx
141278c04: e8 f7 03 66 01              	callq	0x1428d9000 <.text+0x28d8000>
141278c09: eb 20                       	jmp	0x141278c2b <.text+0x1277c2b>
141278c0b: b9 08 00 00 00              	movl	$0x8, %ecx
141278c10: ba 20 00 00 00              	movl	$0x20, %edx
141278c15: e8 e6 03 66 01              	callq	0x1428d9000 <.text+0x28d8000>
141278c1a: eb 0f                       	jmp	0x141278c2b <.text+0x1277c2b>
141278c1c: b9 08 00 00 00              	movl	$0x8, %ecx
141278c21: ba 20 00 00 00              	movl	$0x20, %edx
141278c26: e8 d5 03 66 01              	callq	0x1428d9000 <.text+0x28d8000>
