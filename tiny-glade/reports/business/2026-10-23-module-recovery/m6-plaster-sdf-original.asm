
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141227c90: 00 01                       	addb	%al, (%rcx)
141227c92: 0f 94 c1                    	sete	%cl
141227c95: 66 0f 70 d0 55              	pshufd	$0x55, %xmm0, %xmm2     # xmm2 = xmm0[1,1,1,1]
141227c9a: 66 41 0f 7e d3              	movd	%xmm2, %r11d
141227c9f: 66 0f 7e c6                 	movd	%xmm0, %esi
141227ca3: 66 41 0f 7e c8              	movd	%xmm1, %r8d
141227ca8: 66 0f 70 c1 55              	pshufd	$0x55, %xmm1, %xmm0     # xmm0 = xmm1[1,1,1,1]
141227cad: 66 0f 7e c3                 	movd	%xmm0, %ebx
141227cb1: 4a 8d 14 1e                 	leaq	(%rsi,%r11), %rdx
141227cb5: 48 c1 e2 05                 	shlq	$0x5, %rdx
141227cb9: 4c 8b 95 00 01 00 00        	movq	0x100(%rbp), %r10
141227cc0: 4c 01 d2                    	addq	%r10, %rdx
141227cc3: 4c 89 85 88 03 00 00        	movq	%r8, 0x388(%rbp)
141227cca: 49 01 d8                    	addq	%rbx, %r8
141227ccd: 49 c1 e0 05                 	shlq	$0x5, %r8
141227cd1: 4c 03 85 08 01 00 00        	addq	0x108(%rbp), %r8
141227cd8: 49 89 de                    	movq	%rbx, %r14
141227cdb: 4d 29 de                    	subq	%r11, %r14
141227cde: 49 83 fe 05                 	cmpq	$0x5, %r14
141227ce2: 41 0f 93 c4                 	setae	%r12b
141227ce6: 41 20 cc                    	andb	%cl, %r12b
141227ce9: 4c 8b 8d 48 03 00 00        	movq	0x348(%rbp), %r9
141227cf0: 4a 8d 0c 08                 	leaq	(%rax,%r9), %rcx
141227cf4: 48 83 c1 10                 	addq	$0x10, %rcx
141227cf8: 48 39 ca                    	cmpq	%rcx, %rdx
141227cfb: 4a 8d 44 08 08              	leaq	0x8(%rax,%r9), %rax
141227d00: 0f 28 75 10                 	movaps	0x10(%rbp), %xmm6
141227d04: 0f 28 85 a0 03 00 00        	movaps	0x3a0(%rbp), %xmm0
141227d0b: f3 0f 5c f0                 	subss	%xmm0, %xmm6
141227d0f: f3 0f 16 c0                 	movshdup	%xmm0, %xmm0            # xmm0 = xmm0[1,1,3,3]
141227d13: f3 44 0f 5c c0              	subss	%xmm0, %xmm8
141227d18: 0f 92 c1                    	setb	%cl
141227d1b: 4c 39 c0                    	cmpq	%r8, %rax
141227d1e: 0f 92 c0                    	setb	%al
141227d21: 20 c8                       	andb	%cl, %al
141227d23: 44 89 f2                    	movl	%r14d, %edx
141227d26: 83 e2 03                    	andl	$0x3, %edx
141227d29: b9 04 00 00 00              	movl	$0x4, %ecx
141227d2e: 48 0f 44 d1                 	cmoveq	%rcx, %rdx
141227d32: 49 29 d6                    	subq	%rdx, %r14
141227d35: 4d 01 de                    	addq	%r11, %r14
141227d38: 4c 89 b5 c8 02 00 00        	movq	%r14, 0x2c8(%rbp)
141227d3f: 66 0f fe 15 19 cc 8a 01     	paddd	0x18acc19(%rip), %xmm2  # 0x142ad4960
141227d47: 66 0f 7f 95 20 01 00 00     	movdqa	%xmm2, 0x120(%rbp)
141227d4f: 4c 01 da                    	addq	%r11, %rdx
141227d52: 48 29 da                    	subq	%rbx, %rdx
141227d55: 48 89 95 c8 00 00 00        	movq	%rdx, 0xc8(%rbp)
141227d5c: 4c 89 9d f0 01 00 00        	movq	%r11, 0x1f0(%rbp)
141227d63: 49 c1 e3 05                 	shlq	$0x5, %r11
141227d67: 41 f6 d4                    	notb	%r12b
141227d6a: 41 08 c4                    	orb	%al, %r12b
141227d6d: 44 88 a5 90 02 00 00        	movb	%r12b, 0x290(%rbp)
141227d74: 49 89 f5                    	movq	%rsi, %r13
141227d77: 49 c1 e5 05                 	shlq	$0x5, %r13
141227d7b: 4d 01 d5                    	addq	%r10, %r13
141227d7e: 4d 01 eb                    	addq	%r13, %r11
141227d81: 4c 89 9d 78 03 00 00        	movq	%r11, 0x378(%rbp)
141227d88: 0f 29 b5 30 01 00 00        	movaps	%xmm6, 0x130(%rbp)
141227d8f: 90                          	nop
141227d90: 0f 57 c9                    	xorps	%xmm1, %xmm1
141227d93: f3 0f 2a ce                 	cvtsi2ss	%esi, %xmm1
141227d97: f3 0f 10 85 80 03 00 00     	movss	0x380(%rbp), %xmm0
141227d9f: 0f 28 d6                    	movaps	%xmm6, %xmm2
141227da2: e8 49 c1 6f 01              	callq	0x142923ef0 <.text+0x2922ef0>
141227da7: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
141227dab: 45 0f 54 d1                 	andps	%xmm9, %xmm10
141227daf: 4c 8b b5 f0 01 00 00        	movq	0x1f0(%rbp), %r14
141227db6: f6 85 90 02 00 00 01        	testb	$0x1, 0x290(%rbp)
141227dbd: 0f 85 d0 01 00 00           	jne	0x141227f93 <.text+0x1226f93>
141227dc3: f3 41 0f 10 47 0c           	movss	0xc(%r15), %xmm0
141227dc9: 45 0f 28 fa                 	movaps	%xmm10, %xmm15
141227dcd: f3 45 0f 5c 7f 08           	subss	0x8(%r15), %xmm15
141227dd3: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141227dd7: 0f 29 85 a0 03 00 00        	movaps	%xmm0, 0x3a0(%rbp)
141227dde: 45 0f c6 ff 00              	shufps	$0x0, %xmm15, %xmm15    # xmm15 = xmm15[0,0,0,0]
141227de3: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
141227de7: 41 0f 5f c5                 	maxps	%xmm13, %xmm0
141227deb: 0f 59 c0                    	mulps	%xmm0, %xmm0
141227dee: 0f 29 85 d0 02 00 00        	movaps	%xmm0, 0x2d0(%rbp)
141227df5: 4c 8b a5 78 03 00 00        	movq	0x378(%rbp), %r12
141227dfc: 4c 8b b5 c8 00 00 00        	movq	0xc8(%rbp), %r14
141227e03: 66 0f 6f b5 20 01 00 00     	movdqa	0x120(%rbp), %xmm6
141227e0b: 66 44 0f 6f 0d 5c cb 8a 01  	movdqa	0x18acb5c(%rip), %xmm9  # 0x142ad4970
141227e14: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141227e20: 66 0f 6f c6                 	movdqa	%xmm6, %xmm0
141227e24: 66 0f db 05 44 2d 8f 01     	pand	0x18f2d44(%rip), %xmm0  # 0x142b1ab70
141227e2c: 66 0f eb 05 fc c9 95 01     	por	0x195c9fc(%rip), %xmm0  # 0x142b84830
141227e34: 66 44 0f 6f e6              	movdqa	%xmm6, %xmm12
141227e39: 66 41 0f 72 d4 10           	psrld	$0x10, %xmm12
141227e3f: 66 44 0f eb 25 c8 55 9f 01  	por	0x19f55c8(%rip), %xmm12 # 0x142c1d410
141227e48: 44 0f 5c 25 d0 55 9f 01     	subps	0x19f55d0(%rip), %xmm12 # 0x142c1d420
141227e50: 44 0f 58 e0                 	addps	%xmm0, %xmm12
141227e54: 0f 28 c7                    	movaps	%xmm7, %xmm0
141227e57: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
141227e5b: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141227e5f: e8 8c c0 6f 01              	callq	0x142923ef0 <.text+0x2922ef0>
141227e64: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
141227e68: f3 41 0f 16 cc              	movshdup	%xmm12, %xmm1           # xmm1 = xmm12[1,1,3,3]
141227e6d: 0f 28 c7                    	movaps	%xmm7, %xmm0
141227e70: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141227e74: e8 77 c0 6f 01              	callq	0x142923ef0 <.text+0x2922ef0>
141227e79: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
141227e7d: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
141227e81: 66 41 0f 15 cc              	unpckhpd	%xmm12, %xmm1           # xmm1 = xmm1[1],xmm12[1]
141227e86: 0f 28 c7                    	movaps	%xmm7, %xmm0
141227e89: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141227e8d: e8 5e c0 6f 01              	callq	0x142923ef0 <.text+0x2922ef0>
141227e92: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
141227e96: 45 0f c6 e4 ff              	shufps	$0xff, %xmm12, %xmm12   # xmm12 = xmm12[3,3,3,3]
141227e9b: 0f 28 c7                    	movaps	%xmm7, %xmm0
141227e9e: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
141227ea2: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141227ea6: e8 45 c0 6f 01              	callq	0x142923ef0 <.text+0x2922ef0>
141227eab: 44 0f 14 f0                 	unpcklps	%xmm0, %xmm14           # xmm14 = xmm14[0],xmm0[0],xmm14[1],xmm0[1]
141227eaf: 45 0f 14 dd                 	unpcklps	%xmm13, %xmm11          # xmm11 = xmm11[0],xmm13[0],xmm11[1],xmm13[1]
141227eb3: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
141227eb7: 45 0f 16 de                 	movlhps	%xmm14, %xmm11          # xmm11 = xmm11[0],xmm14[0]
141227ebb: 44 0f 54 1d bd 6a 70 01     	andps	0x1706abd(%rip), %xmm11 # 0x14292e980
141227ec3: 44 0f 5c 9d a0 03 00 00     	subps	0x3a0(%rbp), %xmm11
141227ecb: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
141227ecf: 41 0f 5f c5                 	maxps	%xmm13, %xmm0
141227ed3: 0f 59 c0                    	mulps	%xmm0, %xmm0
141227ed6: 0f 58 85 d0 02 00 00        	addps	0x2d0(%rbp), %xmm0
141227edd: 0f 51 c8                    	sqrtps	%xmm0, %xmm1
141227ee0: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
141227ee4: 41 0f c2 c7 03              	cmpunordps	%xmm15, %xmm0
141227ee9: 41 0f 28 d3                 	movaps	%xmm11, %xmm2
141227eed: 0f 54 d0                    	andps	%xmm0, %xmm2
141227ef0: 45 0f 5f df                 	maxps	%xmm15, %xmm11
141227ef4: 41 0f 55 c3                 	andnps	%xmm11, %xmm0
141227ef8: 0f 56 c2                    	orps	%xmm2, %xmm0
141227efb: 41 0f 5d c5                 	minps	%xmm13, %xmm0
141227eff: 0f 58 c1                    	addps	%xmm1, %xmm0
141227f02: 41 0f 10 0c 24              	movups	(%r12), %xmm1
141227f07: 41 0f 10 54 24 20           	movups	0x20(%r12), %xmm2
141227f0d: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
141227f10: 41 0f 10 54 24 60           	movups	0x60(%r12), %xmm2
141227f16: 41 0f 16 54 24 40           	movhps	0x40(%r12), %xmm2       # xmm2 = xmm2[0,1],mem[0,1]
141227f1c: 0f c6 ca 24                 	shufps	$0x24, %xmm2, %xmm1     # xmm1 = xmm1[0,1],xmm2[2,0]
141227f20: 0f 28 d0                    	movaps	%xmm0, %xmm2
141227f23: 0f 5d c1                    	minps	%xmm1, %xmm0
141227f26: 0f c2 c9 03                 	cmpunordps	%xmm1, %xmm1
141227f2a: 0f 54 d1                    	andps	%xmm1, %xmm2
141227f2d: 0f 55 c8                    	andnps	%xmm0, %xmm1
141227f30: 0f 56 ca                    	orps	%xmm2, %xmm1
141227f33: f3 41 0f 11 0c 24           	movss	%xmm1, (%r12)
141227f39: 66 0f 70 c1 f5              	pshufd	$0xf5, %xmm1, %xmm0     # xmm0 = xmm1[1,1,3,3]
141227f3e: 66 41 0f 7e 44 24 20        	movd	%xmm0, 0x20(%r12)
141227f45: 66 0f 70 c1 ee              	pshufd	$0xee, %xmm1, %xmm0     # xmm0 = xmm1[2,3,2,3]
141227f4a: 66 41 0f 7e 44 24 40        	movd	%xmm0, 0x40(%r12)
141227f51: 66 0f 70 c1 ff              	pshufd	$0xff, %xmm1, %xmm0     # xmm0 = xmm1[3,3,3,3]
141227f56: 66 41 0f 7e 44 24 60        	movd	%xmm0, 0x60(%r12)
141227f5d: 66 41 0f fe f1              	paddd	%xmm9, %xmm6
141227f62: 49 83 ec 80                 	subq	$-0x80, %r12
141227f66: 49 83 c6 04                 	addq	$0x4, %r14
141227f6a: 0f 85 b0 fe ff ff           	jne	0x141227e20 <.text+0x1226e20>
141227f70: 4c 8b b5 c8 02 00 00        	movq	0x2c8(%rbp), %r14
141227f77: 45 0f 57 ff                 	xorps	%xmm15, %xmm15
141227f7b: f3 44 0f 10 35 f4 50 7a 01  	movss	0x17a50f4(%rip), %xmm14 # 0x1429cd078
141227f84: 44 0f 28 0d f4 69 70 01     	movaps	0x17069f4(%rip), %xmm9  # 0x14292e980
141227f8c: 0f 28 b5 30 01 00 00        	movaps	0x130(%rbp), %xmm6
141227f93: 49 89 fc                    	movq	%rdi, %r12
141227f96: 4d 0f af e6                 	imulq	%r14, %r12
141227f9a: 4d 01 ec                    	addq	%r13, %r12
141227f9d: 0f 1f 00                    	nopl	(%rax)
141227fa0: 0f 57 c9                    	xorps	%xmm1, %xmm1
141227fa3: f3 41 0f 2a ce              	cvtsi2ss	%r14d, %xmm1
141227fa8: 0f 28 c7                    	movaps	%xmm7, %xmm0
141227fab: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141227faf: e8 3c bf 6f 01              	callq	0x142923ef0 <.text+0x2922ef0>
141227fb4: 41 0f 54 c1                 	andps	%xmm9, %xmm0
141227fb8: f2 41 0f 10 4f 08           	movsd	0x8(%r15), %xmm1
141227fbe: 41 0f 16 c2                 	movlhps	%xmm10, %xmm0           # xmm0 = xmm0[0],xmm10[0]
141227fc2: 41 0f c6 c2 e2              	shufps	$0xe2, %xmm10, %xmm0    # xmm0 = xmm0[2,0],xmm10[2,3]
141227fc7: 0f 5c c1                    	subps	%xmm1, %xmm0
141227fca: 0f 57 c9                    	xorps	%xmm1, %xmm1
141227fcd: 0f 5f c8                    	maxps	%xmm0, %xmm1
141227fd0: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
141227fd4: 0f 28 d8                    	movaps	%xmm0, %xmm3
141227fd7: f3 0f c2 d8 03              	cmpunordss	%xmm0, %xmm3
141227fdc: 0f 28 e3                    	movaps	%xmm3, %xmm4
141227fdf: 0f 54 e2                    	andps	%xmm2, %xmm4
141227fe2: f3 0f 5f d0                 	maxss	%xmm0, %xmm2
141227fe6: 0f c2 c0 07                 	cmpordps	%xmm0, %xmm0
141227fea: 0f 54 c1                    	andps	%xmm1, %xmm0
141227fed: 0f 59 c0                    	mulps	%xmm0, %xmm0
141227ff0: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
141227ff4: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141227ff8: 0f 57 c0                    	xorps	%xmm0, %xmm0
141227ffb: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
141227fff: 0f 55 da                    	andnps	%xmm2, %xmm3
141228002: 0f 56 dc                    	orps	%xmm4, %xmm3
141228005: f3 41 0f 5d df              	minss	%xmm15, %xmm3
14122800a: f3 0f 58 d8                 	addss	%xmm0, %xmm3
14122800e: 49 ff c6                    	incq	%r14
141228011: f3 41 0f 10 04 24           	movss	(%r12), %xmm0
141228017: 0f 28 c8                    	movaps	%xmm0, %xmm1
14122801a: f3 0f c2 c8 03              	cmpunordss	%xmm0, %xmm1
14122801f: 0f 28 d1                    	movaps	%xmm1, %xmm2
141228022: 0f 54 d3                    	andps	%xmm3, %xmm2
141228025: f3 0f 5d d8                 	minss	%xmm0, %xmm3
141228029: 0f 55 cb                    	andnps	%xmm3, %xmm1
14122802c: 0f 56 ca                    	orps	%xmm2, %xmm1
14122802f: f3 41 0f 11 0c 24           	movss	%xmm1, (%r12)
141228035: 49 01 fc                    	addq	%rdi, %r12
141228038: 4c 39 f3                    	cmpq	%r14, %rbx
14122803b: 0f 85 5f ff ff ff           	jne	0x141227fa0 <.text+0x1226fa0>
141228041: 48 ff c6                    	incq	%rsi
141228044: 48 83 85 78 03 00 00 20     	addq	$0x20, 0x378(%rbp)
14122804c: 49 83 c5 20                 	addq	$0x20, %r13
141228050: 48 3b b5 88 03 00 00        	cmpq	0x388(%rbp), %rsi
141228057: 0f 85 33 fd ff ff           	jne	0x141227d90 <.text+0x1226d90>
14122805d: e9 ae fa ff ff              	jmp	0x141227b10 <.text+0x1226b10>
