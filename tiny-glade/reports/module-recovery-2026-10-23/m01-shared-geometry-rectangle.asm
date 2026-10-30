
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c9c620: 48 83 ec 28                 	subq	$0x28, %rsp
140c9c624: f3 0f 10 4c 24 50           	movss	0x50(%rsp), %xmm1
140c9c62a: 48 89 d0                    	movq	%rdx, %rax
140c9c62d: 48 83 c8 02                 	orq	$0x2, %rax
140c9c631: 48 83 f8 03                 	cmpq	$0x3, %rax
140c9c635: 75 21                       	jne	0x140c9c658 <.text+0xc9b658>
140c9c637: 0f 2e cb                    	ucomiss	%xmm3, %xmm1
140c9c63a: 0f 82 ba 00 00 00           	jb	0x140c9c6fa <.text+0xc9b6fa>
140c9c640: f3 0f 10 41 14              	movss	0x14(%rcx), %xmm0
140c9c645: f3 0f 58 d0                 	addss	%xmm0, %xmm2
140c9c649: f3 0f 5f da                 	maxss	%xmm2, %xmm3
140c9c64d: f3 0f 5d cb                 	minss	%xmm3, %xmm1
140c9c651: f3 0f 11 49 14              	movss	%xmm1, 0x14(%rcx)
140c9c656: eb 29                       	jmp	0x140c9c681 <.text+0xc9b681>
140c9c658: 0f 2e cb                    	ucomiss	%xmm3, %xmm1
140c9c65b: 0f 82 99 00 00 00           	jb	0x140c9c6fa <.text+0xc9b6fa>
140c9c661: f3 0f 10 41 10              	movss	0x10(%rcx), %xmm0
140c9c666: f3 0f 58 d0                 	addss	%xmm0, %xmm2
140c9c66a: f3 0f 5f da                 	maxss	%xmm2, %xmm3
140c9c66e: f3 0f 5d cb                 	minss	%xmm3, %xmm1
140c9c672: f3 0f 11 49 10              	movss	%xmm1, 0x10(%rcx)
140c9c677: 48 83 fa 04                 	cmpq	$0x4, %rdx
140c9c67b: 0f 83 88 00 00 00           	jae	0x140c9c709 <.text+0xc9b709>
140c9c681: f3 0f 5c c8                 	subss	%xmm0, %xmm1
140c9c685: 48 83 fa 02                 	cmpq	$0x2, %rdx
140c9c689: 77 1e                       	ja	0x140c9c6a9 <.text+0xc9b6a9>
140c9c68b: 48 8d 05 f2 23 e9 01        	leaq	0x1e923f2(%rip), %rax   # 0x142b2ea84
140c9c692: 4c 8d 05 f7 23 e9 01        	leaq	0x1e923f7(%rip), %r8    # 0x142b2ea90
140c9c699: f3 0f 10 14 90              	movss	(%rax,%rdx,4), %xmm2
140c9c69e: f3 41 0f 10 04 90           	movss	(%r8,%rdx,4), %xmm0
140c9c6a4: 0f 14 c2                    	unpcklps	%xmm2, %xmm0            # xmm0 = xmm0[0],xmm2[0],xmm0[1],xmm2[1]
140c9c6a7: eb 08                       	jmp	0x140c9c6b1 <.text+0xc9b6b1>
140c9c6a9: f2 0f 10 05 4f 16 d4 01     	movsd	0x1d4164f(%rip), %xmm0  # 0x1429ddd00
140c9c6b1: 0f 14 c0                    	unpcklps	%xmm0, %xmm0            # xmm0 = xmm0[0,0,1,1]
140c9c6b4: f2 0f 10 11                 	movsd	(%rcx), %xmm2
140c9c6b8: f2 0f 10 59 08              	movsd	0x8(%rcx), %xmm3
140c9c6bd: f3 0f 16 e2                 	movshdup	%xmm2, %xmm4            # xmm4 = xmm2[1,1,3,3]
140c9c6c1: 0f 57 25 28 92 c8 01        	xorps	0x1c89228(%rip), %xmm4  # 0x1429258f0
140c9c6c8: 0f 28 ea                    	movaps	%xmm2, %xmm5
140c9c6cb: 0f 16 ec                    	movlhps	%xmm4, %xmm5            # xmm5 = xmm5[0],xmm4[0]
140c9c6ce: 0f c6 d5 24                 	shufps	$0x24, %xmm5, %xmm2     # xmm2 = xmm2[0,1],xmm5[2,0]
140c9c6d2: 0f 59 d0                    	mulps	%xmm0, %xmm2
140c9c6d5: 0f 28 c2                    	movaps	%xmm2, %xmm0
140c9c6d8: 66 0f 15 c2                 	unpckhpd	%xmm2, %xmm0            # xmm0 = xmm0[1],xmm2[1]
140c9c6dc: 0f 58 c2                    	addps	%xmm2, %xmm0
140c9c6df: f3 0f 59 0d 89 91 c8 01     	mulss	0x1c89189(%rip), %xmm1  # 0x142925870
140c9c6e7: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
140c9c6eb: 0f 59 c8                    	mulps	%xmm0, %xmm1
140c9c6ee: 0f 58 cb                    	addps	%xmm3, %xmm1
140c9c6f1: 0f 13 49 08                 	movlps	%xmm1, 0x8(%rcx)
140c9c6f5: 48 83 c4 28                 	addq	$0x28, %rsp
140c9c6f9: c3                          	retq
140c9c6fa: 4c 8d 05 df 1e e9 01        	leaq	0x1e91edf(%rip), %r8    # 0x142b2e5e0
140c9c701: 0f 28 c3                    	movaps	%xmm3, %xmm0
140c9c704: e8 77 ed 0e 00              	callq	0x140d8b480 <.text+0xd8a480>
140c9c709: 48 8d 0d a0 21 e9 01        	leaq	0x1e921a0(%rip), %rcx   # 0x142b2e8b0
140c9c710: 4c 8d 05 c9 21 e9 01        	leaq	0x1e921c9(%rip), %r8    # 0x142b2e8e0
140c9c717: ba 2b 00 00 00              	movl	$0x2b, %edx
140c9c71c: e8 0f cd c3 01              	callq	0x1428d9430 <.text+0x28d8430>
140c9c721: cc                          	int3
140c9c722: cc                          	int3
140c9c723: cc                          	int3
140c9c724: cc                          	int3
140c9c725: cc                          	int3
140c9c726: cc                          	int3
140c9c727: cc                          	int3
140c9c728: cc                          	int3
140c9c729: cc                          	int3
140c9c72a: cc                          	int3
140c9c72b: cc                          	int3
140c9c72c: cc                          	int3
140c9c72d: cc                          	int3
140c9c72e: cc                          	int3
140c9c72f: cc                          	int3
140c9c730: 56                          	pushq	%rsi
140c9c731: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
140c9c738: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
140c9c741: 44 0f 29 b4 24 a0 00 00 00  	movaps	%xmm14, 0xa0(%rsp)
140c9c74a: 44 0f 29 ac 24 90 00 00 00  	movaps	%xmm13, 0x90(%rsp)
140c9c753: 44 0f 29 a4 24 80 00 00 00  	movaps	%xmm12, 0x80(%rsp)
140c9c75c: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140c9c762: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140c9c768: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
140c9c76e: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
140c9c774: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140c9c779: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
140c9c77e: 0f 28 f3                    	movaps	%xmm3, %xmm6
140c9c781: 48 89 ce                    	movq	%rcx, %rsi
140c9c784: f2 44 0f 10 22              	movsd	(%rdx), %xmm12
140c9c789: f2 44 0f 10 4a 08           	movsd	0x8(%rdx), %xmm9
140c9c78f: f2 44 0f 10 6a 10           	movsd	0x10(%rdx), %xmm13
140c9c795: f3 41 0f 16 cc              	movshdup	%xmm12, %xmm1           # xmm1 = xmm12[1,1,3,3]
140c9c79a: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
140c9c79e: f3 41 0f 59 c4              	mulss	%xmm12, %xmm0
140c9c7a3: 0f 28 d9                    	movaps	%xmm1, %xmm3
140c9c7a6: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
140c9c7aa: f3 0f 58 d8                 	addss	%xmm0, %xmm3
140c9c7ae: f2 41 0f 10 20              	movsd	(%r8), %xmm4
140c9c7b3: f2 45 0f 10 58 08           	movsd	0x8(%r8), %xmm11
140c9c7b9: f2 45 0f 10 50 10           	movsd	0x10(%r8), %xmm10
140c9c7bf: 41 0f 28 ec                 	movaps	%xmm12, %xmm5
140c9c7c3: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c9c7c7: f3 0f 16 c4                 	movshdup	%xmm4, %xmm0            # xmm0 = xmm4[1,1,3,3]
140c9c7cb: 0f 28 d1                    	movaps	%xmm1, %xmm2
140c9c7ce: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
140c9c7d2: f3 0f 58 d5                 	addss	%xmm5, %xmm2
140c9c7d6: f3 0f 59 cc                 	mulss	%xmm4, %xmm1
140c9c7da: 0f 59 e4                    	mulps	%xmm4, %xmm4
140c9c7dd: f3 0f 16 ec                 	movshdup	%xmm4, %xmm5            # xmm5 = xmm4[1,1,3,3]
140c9c7e1: f3 0f 58 ec                 	addss	%xmm4, %xmm5
140c9c7e5: f3 0f 59 eb                 	mulss	%xmm3, %xmm5
140c9c7e9: 0f 57 db                    	xorps	%xmm3, %xmm3
140c9c7ec: f3 0f 51 dd                 	sqrtss	%xmm5, %xmm3
140c9c7f0: f3 0f 5e d3                 	divss	%xmm3, %xmm2
140c9c7f4: 0f 28 25 85 21 c9 01        	movaps	0x1c92185(%rip), %xmm4  # 0x14292e980
140c9c7fb: 0f 54 e2                    	andps	%xmm2, %xmm4
140c9c7fe: f3 0f 10 2d 3e 45 e4 01     	movss	0x1e4453e(%rip), %xmm5  # 0x142ae0d44
140c9c806: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c9c80a: f3 0f 58 2d 36 45 e4 01     	addss	0x1e44536(%rip), %xmm5  # 0x142ae0d48
140c9c812: f3 44 0f 10 3d e9 90 c8 01  	movss	0x1c890e9(%rip), %xmm15 # 0x142925904
140c9c81b: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c9c81f: f3 0f 58 2d 25 45 e4 01     	addss	0x1e44525(%rip), %xmm5  # 0x142ae0d4c
140c9c827: 41 0f 28 ff                 	movaps	%xmm15, %xmm7
140c9c82b: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c9c82f: f3 0f 58 2d 19 45 e4 01     	addss	0x1e44519(%rip), %xmm5  # 0x142ae0d50
140c9c837: f3 0f 5c fc                 	subss	%xmm4, %xmm7
140c9c83b: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c9c83f: f3 0f 58 2d 0d 45 e4 01     	addss	0x1e4450d(%rip), %xmm5  # 0x142ae0d54
140c9c847: 0f 57 db                    	xorps	%xmm3, %xmm3
140c9c84a: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c9c84e: f3 0f 58 2d 02 45 e4 01     	addss	0x1e44502(%rip), %xmm5  # 0x142ae0d58
140c9c856: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
140c9c85a: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c9c85e: f3 0f 58 2d f6 44 e4 01     	addss	0x1e444f6(%rip), %xmm5  # 0x142ae0d5c
140c9c866: f3 44 0f 5f c7              	maxss	%xmm7, %xmm8
140c9c86b: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c9c86f: f3 0f 58 2d e9 44 e4 01     	addss	0x1e444e9(%rip), %xmm5  # 0x142ae0d60
140c9c877: 0f 57 e4                    	xorps	%xmm4, %xmm4
140c9c87a: f3 41 0f 51 e0              	sqrtss	%xmm8, %xmm4
140c9c87f: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c9c883: f3 0f 10 25 19 13 d5 01     	movss	0x1d51319(%rip), %xmm4  # 0x1429edba4
140c9c88b: f3 0f 5c e5                 	subss	%xmm5, %xmm4
140c9c88f: 0f 57 ff                    	xorps	%xmm7, %xmm7
140c9c892: f3 0f c2 fa 06              	cmpnless	%xmm2, %xmm7
140c9c897: 0f 28 d7                    	movaps	%xmm7, %xmm2
140c9c89a: 0f 55 d5                    	andnps	%xmm5, %xmm2
140c9c89d: 0f 54 fc                    	andps	%xmm4, %xmm7
140c9c8a0: f3 41 0f 59 c4              	mulss	%xmm12, %xmm0
140c9c8a5: f3 0f 5c c1                 	subss	%xmm1, %xmm0
140c9c8a9: 44 0f 28 35 3f 90 c8 01     	movaps	0x1c8903f(%rip), %xmm14 # 0x1429258f0
140c9c8b1: 0f 28 c8                    	movaps	%xmm0, %xmm1
140c9c8b4: 41 0f 54 ce                 	andps	%xmm14, %xmm1
140c9c8b8: 0f 56 0d 71 90 c8 01        	orps	0x1c89071(%rip), %xmm1  # 0x142925930
140c9c8bf: 0f 56 fa                    	orps	%xmm2, %xmm7
140c9c8c2: f3 0f c2 c0 03              	cmpunordss	%xmm0, %xmm0
140c9c8c7: f3 44 0f 10 05 f4 0c df 01  	movss	0x1df0cf4(%rip), %xmm8  # 0x142a8d5c4
140c9c8d0: 44 0f 54 c0                 	andps	%xmm0, %xmm8
140c9c8d4: 0f 55 c1                    	andnps	%xmm1, %xmm0
140c9c8d7: 44 0f 56 c0                 	orps	%xmm0, %xmm8
140c9c8db: f3 44 0f 59 c7              	mulss	%xmm7, %xmm8
140c9c8e0: f3 44 0f 5c fe              	subss	%xmm6, %xmm15
140c9c8e5: f3 41 0f 59 df              	mulss	%xmm15, %xmm3
140c9c8ea: f3 44 0f 59 c6              	mulss	%xmm6, %xmm8
140c9c8ef: f3 44 0f 58 c3              	addss	%xmm3, %xmm8
140c9c8f4: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
140c9c8f8: e8 d3 76 c8 01              	callq	0x142923fd0 <.text+0x2922fd0>
140c9c8fd: 0f 28 f8                    	movaps	%xmm0, %xmm7
140c9c900: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
140c9c904: e8 77 75 c8 01              	callq	0x142923e80 <.text+0x2922e80>
140c9c909: f3 41 0f 12 cf              	movsldup	%xmm15, %xmm1           # xmm1 = xmm15[0,0,2,2]
140c9c90e: 44 0f 59 e9                 	mulps	%xmm1, %xmm13
140c9c912: f3 0f 12 d6                 	movsldup	%xmm6, %xmm2            # xmm2 = xmm6[0,0,2,2]
140c9c916: 44 0f 59 d2                 	mulps	%xmm2, %xmm10
140c9c91a: 45 0f 58 d5                 	addps	%xmm13, %xmm10
140c9c91e: 66 44 0f 7e d0              	movd	%xmm10, %eax
140c9c923: 25 ff ff ff 7f              	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
140c9c928: 3d 00 00 80 7f              	cmpl	$0x7f800000, %eax       # imm = 0x7F800000
140c9c92d: 0f 8d d2 00 00 00           	jge	0x140c9ca05 <.text+0xc9ba05>
140c9c933: f3 41 0f 16 da              	movshdup	%xmm10, %xmm3           # xmm3 = xmm10[1,1,3,3]
140c9c938: 66 0f 7e d8                 	movd	%xmm3, %eax
140c9c93c: 25 ff ff ff 7f              	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
140c9c941: 3d 00 00 80 7f              	cmpl	$0x7f800000, %eax       # imm = 0x7F800000
140c9c946: 0f 8d d1 00 00 00           	jge	0x140c9ca1d <.text+0xc9ba1d>
140c9c94c: 45 0f 14 e4                 	unpcklps	%xmm12, %xmm12          # xmm12 = xmm12[0,0,1,1]
140c9c950: 44 0f 57 f7                 	xorps	%xmm7, %xmm14
140c9c954: 44 0f 14 f0                 	unpcklps	%xmm0, %xmm14           # xmm14 = xmm14[0],xmm0[0],xmm14[1],xmm0[1]
140c9c958: 0f 14 c7                    	unpcklps	%xmm7, %xmm0            # xmm0 = xmm0[0],xmm7[0],xmm0[1],xmm7[1]
140c9c95b: 41 0f 16 c6                 	movlhps	%xmm14, %xmm0           # xmm0 = xmm0[0],xmm14[0]
140c9c95f: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
140c9c963: 0f 28 d8                    	movaps	%xmm0, %xmm3
140c9c966: 66 0f 15 d8                 	unpckhpd	%xmm0, %xmm3            # xmm3 = xmm3[1],xmm0[1]
140c9c96a: 0f 58 d8                    	addps	%xmm0, %xmm3
140c9c96d: 0f 28 c3                    	movaps	%xmm3, %xmm0
140c9c970: 0f 59 c3                    	mulps	%xmm3, %xmm0
140c9c973: f3 0f 16 e0                 	movshdup	%xmm0, %xmm4            # xmm4 = xmm0[1,1,3,3]
140c9c977: f3 0f 58 e0                 	addss	%xmm0, %xmm4
140c9c97b: f3 0f 58 25 61 d7 cb 01     	addss	0x1cbd761(%rip), %xmm4  # 0x14295a0e4
140c9c983: 0f 54 25 f6 1f c9 01        	andps	0x1c91ff6(%rip), %xmm4  # 0x14292e980
140c9c98a: f3 0f 10 05 a6 1b e9 01     	movss	0x1e91ba6(%rip), %xmm0  # 0x142b2e538
140c9c992: 0f 2e c4                    	ucomiss	%xmm4, %xmm0
140c9c995: 0f 82 9a 00 00 00           	jb	0x140c9ca35 <.text+0xc9ba35>
140c9c99b: 44 0f 59 da                 	mulps	%xmm2, %xmm11
140c9c99f: 41 0f 59 c9                 	mulps	%xmm9, %xmm1
140c9c9a3: 41 0f 58 cb                 	addps	%xmm11, %xmm1
140c9c9a7: 0f 16 d9                    	movlhps	%xmm1, %xmm3            # xmm3 = xmm3[0],xmm1[0]
140c9c9aa: 0f 11 1e                    	movups	%xmm3, (%rsi)
140c9c9ad: 66 44 0f 13 56 10           	movlpd	%xmm10, 0x10(%rsi)
140c9c9b3: 48 89 f0                    	movq	%rsi, %rax
140c9c9b6: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
140c9c9bb: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140c9c9c0: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
140c9c9c6: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
140c9c9cc: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140c9c9d2: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140c9c9d8: 44 0f 28 a4 24 80 00 00 00  	movaps	0x80(%rsp), %xmm12
140c9c9e1: 44 0f 28 ac 24 90 00 00 00  	movaps	0x90(%rsp), %xmm13
140c9c9ea: 44 0f 28 b4 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm14
140c9c9f3: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
140c9c9fc: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
140c9ca03: 5e                          	popq	%rsi
140c9ca04: c3                          	retq
140c9ca05: 48 8d 0d ec 1b e9 01        	leaq	0x1e91bec(%rip), %rcx   # 0x142b2e5f8
140c9ca0c: 4c 8d 05 1d 20 e9 01        	leaq	0x1e9201d(%rip), %r8    # 0x142b2ea30
140c9ca13: ba 23 00 00 00              	movl	$0x23, %edx
140c9ca18: e8 13 ca c3 01              	callq	0x1428d9430 <.text+0x28d8430>
140c9ca1d: 48 8d 0d f7 1b e9 01        	leaq	0x1e91bf7(%rip), %rcx   # 0x142b2e61b
140c9ca24: 4c 8d 05 05 20 e9 01        	leaq	0x1e92005(%rip), %r8    # 0x142b2ea30
140c9ca2b: ba 24 00 00 00              	movl	$0x24, %edx
140c9ca30: e8 fb c9 c3 01              	callq	0x1428d9430 <.text+0x28d8430>
140c9ca35: 48 8d 0d 03 1c e9 01        	leaq	0x1e91c03(%rip), %rcx   # 0x142b2e63f
140c9ca3c: 4c 8d 05 ed 1f e9 01        	leaq	0x1e91fed(%rip), %r8    # 0x142b2ea30
140c9ca43: ba 2d 00 00 00              	movl	$0x2d, %edx
140c9ca48: e8 e3 c9 c3 01              	callq	0x1428d9430 <.text+0x28d8430>
