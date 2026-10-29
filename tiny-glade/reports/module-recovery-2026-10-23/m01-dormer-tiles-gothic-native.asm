
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141290b30: 55                          	pushq	%rbp
141290b31: 41 57                       	pushq	%r15
141290b33: 41 56                       	pushq	%r14
141290b35: 41 55                       	pushq	%r13
141290b37: 41 54                       	pushq	%r12
141290b39: 56                          	pushq	%rsi
141290b3a: 57                          	pushq	%rdi
141290b3b: 53                          	pushq	%rbx
141290b3c: 48 81 ec 48 03 00 00        	subq	$0x348, %rsp            # imm = 0x348
141290b43: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141290b4b: 44 0f 29 bd b0 02 00 00     	movaps	%xmm15, 0x2b0(%rbp)
141290b53: 44 0f 29 b5 a0 02 00 00     	movaps	%xmm14, 0x2a0(%rbp)
141290b5b: 44 0f 29 ad 90 02 00 00     	movaps	%xmm13, 0x290(%rbp)
141290b63: 44 0f 29 a5 80 02 00 00     	movaps	%xmm12, 0x280(%rbp)
141290b6b: 44 0f 29 9d 70 02 00 00     	movaps	%xmm11, 0x270(%rbp)
141290b73: 44 0f 29 95 60 02 00 00     	movaps	%xmm10, 0x260(%rbp)
141290b7b: 44 0f 29 8d 50 02 00 00     	movaps	%xmm9, 0x250(%rbp)
141290b83: 44 0f 29 85 40 02 00 00     	movaps	%xmm8, 0x240(%rbp)
141290b8b: 0f 29 bd 30 02 00 00        	movaps	%xmm7, 0x230(%rbp)
141290b92: 0f 29 b5 20 02 00 00        	movaps	%xmm6, 0x220(%rbp)
141290b99: 48 c7 85 18 02 00 00 fe ff ff ff    	movq	$-0x2, 0x218(%rbp)
141290ba4: 48 83 79 10 00              	cmpq	$0x0, 0x10(%rcx)
141290ba9: 0f 84 f6 12 00 00           	je	0x141291ea5 <.text+0x1290ea5>
141290baf: 4c 89 cf                    	movq	%r9, %rdi
141290bb2: 48 89 d3                    	movq	%rdx, %rbx
141290bb5: 48 89 ce                    	movq	%rcx, %rsi
141290bb8: 48 8b 41 08                 	movq	0x8(%rcx), %rax
141290bbc: f2 0f 10 30                 	movsd	(%rax), %xmm6
141290bc0: f3 44 0f 16 c6              	movshdup	%xmm6, %xmm8            # xmm8 = xmm6[1,1,3,3]
141290bc5: f3 0f 10 78 08              	movss	0x8(%rax), %xmm7
141290bca: 48 8d 4d e4                 	leaq	-0x1c(%rbp), %rcx
141290bce: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141290bd2: e8 49 2c 65 ff              	callq	0x1408e3820 <.text+0x8e2820>
141290bd7: 8b 4d e4                    	movl	-0x1c(%rbp), %ecx
141290bda: 83 f9 02                    	cmpl	$0x2, %ecx
141290bdd: 75 62                       	jne	0x141290c41 <.text+0x128fc41>
141290bdf: 0f 28 b5 20 02 00 00        	movaps	0x220(%rbp), %xmm6
141290be6: 0f 28 bd 30 02 00 00        	movaps	0x230(%rbp), %xmm7
141290bed: 44 0f 28 85 40 02 00 00     	movaps	0x240(%rbp), %xmm8
141290bf5: 44 0f 28 8d 50 02 00 00     	movaps	0x250(%rbp), %xmm9
141290bfd: 44 0f 28 95 60 02 00 00     	movaps	0x260(%rbp), %xmm10
141290c05: 44 0f 28 9d 70 02 00 00     	movaps	0x270(%rbp), %xmm11
141290c0d: 44 0f 28 a5 80 02 00 00     	movaps	0x280(%rbp), %xmm12
141290c15: 44 0f 28 ad 90 02 00 00     	movaps	0x290(%rbp), %xmm13
141290c1d: 44 0f 28 b5 a0 02 00 00     	movaps	0x2a0(%rbp), %xmm14
141290c25: 44 0f 28 bd b0 02 00 00     	movaps	0x2b0(%rbp), %xmm15
141290c2d: 48 81 c4 48 03 00 00        	addq	$0x348, %rsp            # imm = 0x348
141290c34: 5b                          	popq	%rbx
141290c35: 5f                          	popq	%rdi
141290c36: 5e                          	popq	%rsi
141290c37: 41 5c                       	popq	%r12
141290c39: 41 5d                       	popq	%r13
141290c3b: 41 5e                       	popq	%r14
141290c3d: 41 5f                       	popq	%r15
141290c3f: 5d                          	popq	%rbp
141290c40: c3                          	retq
141290c41: 44 8b b5 30 03 00 00        	movl	0x330(%rbp), %r14d
141290c48: 48 8d 45 e8                 	leaq	-0x18(%rbp), %rax
141290c4c: 85 c9                       	testl	%ecx, %ecx
141290c4e: 48 89 b5 50 01 00 00        	movq	%rsi, 0x150(%rbp)
141290c55: 74 29                       	je	0x141290c80 <.text+0x128fc80>
141290c57: 48 8b 48 10                 	movq	0x10(%rax), %rcx
141290c5b: 48 89 8d d0 01 00 00        	movq	%rcx, 0x1d0(%rbp)
141290c62: 0f 10 00                    	movups	(%rax), %xmm0
141290c65: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
141290c6c: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
141290c73: 0f 28 ce                    	movaps	%xmm6, %xmm1
141290c76: 0f 28 d7                    	movaps	%xmm7, %xmm2
141290c79: e8 a2 a1 a0 ff              	callq	0x140c9ae20 <.text+0xc99e20>
141290c7e: eb 1c                       	jmp	0x141290c9c <.text+0x128fc9c>
141290c80: 0f 10 00                    	movups	(%rax), %xmm0
141290c83: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
141290c8a: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
141290c91: 0f 28 ce                    	movaps	%xmm6, %xmm1
141290c94: 0f 28 d7                    	movaps	%xmm7, %xmm2
141290c97: e8 a4 6e a0 ff              	callq	0x140c97b40 <.text+0xc96b40>
141290c9c: f3 0f 11 85 c0 01 00 00     	movss	%xmm0, 0x1c0(%rbp)
141290ca4: f3 0f 11 8d c4 01 00 00     	movss	%xmm1, 0x1c4(%rbp)
141290cac: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
141290cb3: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
141290cba: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141290cbe: e8 1d c1 a0 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
141290cc3: 4c 8d 0d 9e 87 99 01        	leaq	0x199879e(%rip), %r9    # 0x142c29468
141290cca: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141290cce: 48 89 fa                    	movq	%rdi, %rdx
141290cd1: 45 89 f0                    	movl	%r14d, %r8d
141290cd4: e8 47 9d 78 ff              	callq	0x140a1aa20 <.text+0xa19a20>
141290cd9: 0f 28 45 c0                 	movaps	-0x40(%rbp), %xmm0
141290cdd: f3 0f 10 4d c4              	movss	-0x3c(%rbp), %xmm1
141290ce2: f3 0f 59 0d be 38 6a 01     	mulss	0x16a38be(%rip), %xmm1  # 0x1429345a8
141290cea: f3 0f 10 55 c8              	movss	-0x38(%rbp), %xmm2
141290cef: 0f 16 d0                    	movlhps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0]
141290cf2: 0f c6 d0 e2                 	shufps	$0xe2, %xmm0, %xmm2     # xmm2 = xmm2[2,0],xmm0[2,3]
141290cf6: 0f 59 15 e3 83 99 01        	mulps	0x19983e3(%rip), %xmm2  # 0x142c290e0
141290cfd: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
141290d01: 0f 59 d2                    	mulps	%xmm2, %xmm2
141290d04: f3 0f 58 ca                 	addss	%xmm2, %xmm1
141290d08: f3 0f 16 c2                 	movshdup	%xmm2, %xmm0            # xmm0 = xmm2[1,1,3,3]
141290d0c: f3 0f 58 c1                 	addss	%xmm1, %xmm0
141290d10: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
141290d14: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
141290d19: 48 89 c1                    	movq	%rax, %rcx
141290d1c: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
141290d20: f3 44 0f 10 2d b3 93 6c 01  	movss	0x16c93b3(%rip), %xmm13 # 0x14295a0dc
141290d29: 0f 28 c8                    	movaps	%xmm0, %xmm1
141290d2c: f3 41 0f 5c cd              	subss	%xmm13, %xmm1
141290d31: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
141290d36: 48 21 ca                    	andq	%rcx, %rdx
141290d39: 48 09 c2                    	orq	%rax, %rdx
141290d3c: 31 f6                       	xorl	%esi, %esi
141290d3e: 45 0f 57 e4                 	xorps	%xmm12, %xmm12
141290d42: 41 0f 2e c4                 	ucomiss	%xmm12, %xmm0
141290d46: 48 0f 42 d6                 	cmovbq	%rsi, %rdx
141290d4a: f3 44 0f 10 35 8d 93 6c 01  	movss	0x16c938d(%rip), %xmm14 # 0x14295a0e0
141290d53: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
141290d57: 48 c7 c7 ff ff ff ff        	movq	$-0x1, %rdi
141290d5e: 48 0f 47 d7                 	cmovaq	%rdi, %rdx
141290d62: 48 89 95 10 02 00 00        	movq	%rdx, 0x210(%rbp)
141290d69: f3 44 0f 10 85 38 01 00 00  	movss	0x138(%rbp), %xmm8
141290d72: 44 0f 28 cf                 	movaps	%xmm7, %xmm9
141290d76: f3 45 0f 5c c8              	subss	%xmm8, %xmm9
141290d7b: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
141290d7f: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
141290d84: f3 44 0f 59 0d e3 ff 69 01  	mulss	0x169ffe3(%rip), %xmm9  # 0x142930d70
141290d8d: f2 44 0f 10 95 30 01 00 00  	movsd	0x130(%rbp), %xmm10
141290d96: 44 0f 28 de                 	movaps	%xmm6, %xmm11
141290d9a: 45 0f 5c da                 	subps	%xmm10, %xmm11
141290d9e: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
141290da2: 41 0f 59 cb                 	mulps	%xmm11, %xmm1
141290da6: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
141290daa: f3 0f 58 d1                 	addss	%xmm1, %xmm2
141290dae: f3 0f 58 d0                 	addss	%xmm0, %xmm2
141290db2: f3 44 0f 51 fa              	sqrtss	%xmm2, %xmm15
141290db7: 44 0f 59 1d 01 5a 7f 01     	mulps	0x17f5a01(%rip), %xmm11 # 0x142a867c0
141290dbf: 44 0f 58 de                 	addps	%xmm6, %xmm11
141290dc3: f3 44 0f 58 cf              	addss	%xmm7, %xmm9
141290dc8: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
141290dcc: f3 0f 5e 05 f0 85 99 01     	divss	0x19985f0(%rip), %xmm0  # 0x142c293c4
141290dd4: e8 c7 31 69 01              	callq	0x142923fa0 <.text+0x2922fa0>
141290dd9: f3 0f 5f 05 0f d9 69 01     	maxss	0x169d90f(%rip), %xmm0  # 0x14292e6f0
141290de1: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
141290de6: 48 89 c1                    	movq	%rax, %rcx
141290de9: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
141290ded: 0f 28 c8                    	movaps	%xmm0, %xmm1
141290df0: f3 41 0f 5c cd              	subss	%xmm13, %xmm1
141290df5: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
141290dfa: 48 21 ca                    	andq	%rcx, %rdx
141290dfd: 48 09 c2                    	orq	%rax, %rdx
141290e00: 41 0f 2e c4                 	ucomiss	%xmm12, %xmm0
141290e04: 48 0f 42 d6                 	cmovbq	%rsi, %rdx
141290e08: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
141290e0c: 48 0f 47 d7                 	cmovaq	%rdi, %rdx
141290e10: f3 0f 10 15 b0 85 99 01     	movss	0x19985b0(%rip), %xmm2  # 0x142c293c8
141290e18: f3 44 0f 11 bd 0c 02 00 00  	movss	%xmm15, 0x20c(%rbp)
141290e21: f3 41 0f 5e d7              	divss	%xmm15, %xmm2
141290e26: 48 8d 05 53 86 99 01        	leaq	0x1998653(%rip), %rax   # 0x142c29480
141290e2d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141290e32: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
141290e39: 4c 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %r9
141290e40: e8 0b f5 9f ff              	callq	0x140c90350 <.text+0xc8f350>
141290e45: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
141290e4c: 41 0f 28 e1                 	movaps	%xmm9, %xmm4
141290e50: f3 41 0f 5c e0              	subss	%xmm8, %xmm4
141290e55: 41 0f 28 db                 	movaps	%xmm11, %xmm3
141290e59: 41 0f 5c da                 	subps	%xmm10, %xmm3
141290e5d: 0f 28 c3                    	movaps	%xmm3, %xmm0
141290e60: 0f 59 c3                    	mulps	%xmm3, %xmm0
141290e63: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
141290e67: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141290e6b: 0f 28 c4                    	movaps	%xmm4, %xmm0
141290e6e: f3 0f 59 c4                 	mulss	%xmm4, %xmm0
141290e72: f3 0f 58 c1                 	addss	%xmm1, %xmm0
141290e76: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
141290e7a: f3 44 0f 10 35 81 4a 69 01  	movss	0x1694a81(%rip), %xmm14 # 0x142925904
141290e83: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
141290e87: f3 0f 5e c8                 	divss	%xmm0, %xmm1
141290e8b: f3 0f 16 c3                 	movshdup	%xmm3, %xmm0            # xmm0 = xmm3[1,1,3,3]
141290e8f: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
141290e93: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
141290e97: 0f 28 e8                    	movaps	%xmm0, %xmm5
141290e9a: f3 41 0f 59 ec              	mulss	%xmm12, %xmm5
141290e9f: 0f 28 fb                    	movaps	%xmm3, %xmm7
141290ea2: f3 41 0f 59 fc              	mulss	%xmm12, %xmm7
141290ea7: 0f 28 d3                    	movaps	%xmm3, %xmm2
141290eaa: f3 0f 5c d5                 	subss	%xmm5, %xmm2
141290eae: 44 0f 28 ea                 	movaps	%xmm2, %xmm13
141290eb2: f3 44 0f 59 ea              	mulss	%xmm2, %xmm13
141290eb7: f3 0f 59 cc                 	mulss	%xmm4, %xmm1
141290ebb: f3 0f 5c e9                 	subss	%xmm1, %xmm5
141290ebf: 0f 28 f1                    	movaps	%xmm1, %xmm6
141290ec2: f3 41 0f 59 f4              	mulss	%xmm12, %xmm6
141290ec7: f3 0f 5c f7                 	subss	%xmm7, %xmm6
141290ecb: 0f 28 e5                    	movaps	%xmm5, %xmm4
141290ece: f3 0f 59 e5                 	mulss	%xmm5, %xmm4
141290ed2: 0f 28 fe                    	movaps	%xmm6, %xmm7
141290ed5: f3 0f 59 fe                 	mulss	%xmm6, %xmm7
141290ed9: f3 0f 58 fc                 	addss	%xmm4, %xmm7
141290edd: f3 41 0f 58 fd              	addss	%xmm13, %xmm7
141290ee2: f3 0f 51 ff                 	sqrtss	%xmm7, %xmm7
141290ee6: 41 0f 28 e6                 	movaps	%xmm14, %xmm4
141290eea: f3 0f 5e e7                 	divss	%xmm7, %xmm4
141290eee: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
141290ef2: f3 0f 59 f4                 	mulss	%xmm4, %xmm6
141290ef6: f3 0f 5c c5                 	subss	%xmm5, %xmm0
141290efa: 31 c9                       	xorl	%ecx, %ecx
141290efc: 44 0f 2e e0                 	ucomiss	%xmm0, %xmm12
141290f00: 0f 92 c2                    	setb	%dl
141290f03: 41 0f 28 fe                 	movaps	%xmm14, %xmm7
141290f07: f3 0f 5c f8                 	subss	%xmm0, %xmm7
141290f0b: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
141290f0f: f3 44 0f 51 ef              	sqrtss	%xmm7, %xmm13
141290f14: f3 44 0f 10 3d 53 49 69 01  	movss	0x1694953(%rip), %xmm15 # 0x142925870
141290f1d: 41 0f 28 ef                 	movaps	%xmm15, %xmm5
141290f21: f3 41 0f 5e ed              	divss	%xmm13, %xmm5
141290f26: f3 0f 58 f3                 	addss	%xmm3, %xmm6
141290f2a: f3 41 0f 58 c6              	addss	%xmm14, %xmm0
141290f2f: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
141290f33: f3 44 0f 51 e8              	sqrtss	%xmm0, %xmm13
141290f38: 41 0f 28 df                 	movaps	%xmm15, %xmm3
141290f3c: f3 41 0f 5e dd              	divss	%xmm13, %xmm3
141290f41: 72 10                       	jb	0x141290f53 <.text+0x128ff53>
141290f43: f3 0f 59 fd                 	mulss	%xmm5, %xmm7
141290f47: 0f 28 c6                    	movaps	%xmm6, %xmm0
141290f4a: f3 0f 59 c5                 	mulss	%xmm5, %xmm0
141290f4e: 0f 28 f7                    	movaps	%xmm7, %xmm6
141290f51: eb 08                       	jmp	0x141290f5b <.text+0x128ff5b>
141290f53: f3 0f 59 f3                 	mulss	%xmm3, %xmm6
141290f57: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
141290f5b: 4c 8b 85 88 01 00 00        	movq	0x188(%rbp), %r8
141290f62: 4c 89 85 00 02 00 00        	movq	%r8, 0x200(%rbp)
141290f69: 44 8b 03                    	movl	(%rbx), %r8d
141290f6c: 44 89 85 bc 01 00 00        	movl	%r8d, 0x1bc(%rbp)
141290f73: 48 bf 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rdi # imm = 0xA0761D6478BD642F
141290f7d: 48 be db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rsi # imm = 0xE7037ED1A0B428DB
141290f87: 48 85 c0                    	testq	%rax, %rax
141290f8a: 0f 84 46 02 00 00           	je	0x1412911d6 <.text+0x12901d6>
141290f90: f3 0f 59 d4                 	mulss	%xmm4, %xmm2
141290f94: f3 44 0f 5c e2              	subss	%xmm2, %xmm12
141290f99: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
141290f9c: 0f 28 0d 0d 36 6a 01        	movaps	0x16a360d(%rip), %xmm1  # 0x1429345b0
141290fa3: 0f 58 ca                    	addps	%xmm2, %xmm1
141290fa6: 44 0f c6 e1 10              	shufps	$0x10, %xmm1, %xmm12    # xmm12 = xmm12[0,0],xmm1[1,0]
141290fab: 0f 58 15 0e 78 7f 01        	addps	0x17f780e(%rip), %xmm2  # 0x142a887c0
141290fb2: f3 0f 12 e5                 	movsldup	%xmm5, %xmm4            # xmm4 = xmm5[0,0,2,2]
141290fb6: 0f 59 e2                    	mulps	%xmm2, %xmm4
141290fb9: 44 0f c6 e1 e2              	shufps	$0xe2, %xmm1, %xmm12    # xmm12 = xmm12[2,0],xmm1[2,3]
141290fbe: f3 0f 12 cb                 	movsldup	%xmm3, %xmm1            # xmm1 = xmm3[0,0,2,2]
141290fc2: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
141290fc6: 4c 8b ad 00 02 00 00        	movq	0x200(%rbp), %r13
141290fcd: 4d 8d 64 85 00              	leaq	(%r13,%rax,4), %r12
141290fd2: 88 d1                       	movb	%dl, %cl
141290fd4: 66 0f 6e d1                 	movd	%ecx, %xmm2
141290fd8: 66 0f 70 d2 50              	pshufd	$0x50, %xmm2, %xmm2     # xmm2 = xmm2[0,0,1,1]
141290fdd: 66 0f 72 f2 1f              	pslld	$0x1f, %xmm2
141290fe2: 66 0f 72 e2 1f              	psrad	$0x1f, %xmm2
141290fe7: 0f 54 ca                    	andps	%xmm2, %xmm1
141290fea: 0f 55 d4                    	andnps	%xmm4, %xmm2
141290fed: 0f 56 d1                    	orps	%xmm1, %xmm2
141290ff0: 0f 14 f0                    	unpcklps	%xmm0, %xmm6            # xmm6 = xmm6[0],xmm0[0],xmm6[1],xmm0[1]
141290ff3: 0f 16 f2                    	movlhps	%xmm2, %xmm6            # xmm6 = xmm6[0],xmm2[0]
141290ff6: b1 01                       	movb	$0x1, %cl
141290ff8: f3 0f 10 3d 28 c1 73 01     	movss	0x173c128(%rip), %xmm7  # 0x1429cd128
141291000: 4c 8d b5 c0 01 00 00        	leaq	0x1c0(%rbp), %r14
141291007: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141291010: 49 8d 55 04                 	leaq	0x4(%r13), %rdx
141291014: f6 c1 01                    	testb	$0x1, %cl
141291017: 74 17                       	je	0x141291030 <.text+0x1290030>
141291019: 4c 39 e2                    	cmpq	%r12, %rdx
14129101c: 0f 84 b4 01 00 00           	je	0x1412911d6 <.text+0x12901d6>
141291022: 4c 89 e8                    	movq	%r13, %rax
141291025: 49 83 c5 08                 	addq	$0x8, %r13
141291029: 48 89 d3                    	movq	%rdx, %rbx
14129102c: eb 08                       	jmp	0x141291036 <.text+0x1290036>
14129102e: 66 90                       	nop
141291030: 4c 89 eb                    	movq	%r13, %rbx
141291033: 49 89 d5                    	movq	%rdx, %r13
141291036: f3 44 0f 10 30              	movss	(%rax), %xmm14
14129103b: f3 44 0f 10 2b              	movss	(%rbx), %xmm13
141291040: 45 0f 28 e6                 	movaps	%xmm14, %xmm12
141291044: f3 45 0f 58 e5              	addss	%xmm13, %xmm12
141291049: f3 44 0f 59 25 1e 48 69 01  	mulss	0x169481e(%rip), %xmm12 # 0x142925870
141291052: 0f 29 75 00                 	movaps	%xmm6, (%rbp)
141291056: 48 89 e9                    	movq	%rbp, %rcx
141291059: e8 52 61 d7 00              	callq	0x1420071b0 <.text+0x20061b0>
14129105e: 49 89 c7                    	movq	%rax, %r15
141291061: f3 45 0f 5c ee              	subss	%xmm14, %xmm13
141291066: f3 44 0f 59 ad 0c 02 00 00  	mulss	0x20c(%rbp), %xmm13
14129106f: f3 44 0f 59 ef              	mulss	%xmm7, %xmm13
141291074: c7 85 c0 01 00 00 cd cc cc 3d       	movl	$0x3dcccccd, 0x1c0(%rbp) # imm = 0x3DCCCCCD
14129107e: f3 44 0f 11 ad c4 01 00 00  	movss	%xmm13, 0x1c4(%rbp)
141291087: c7 85 c8 01 00 00 cd cc 4c 3e       	movl	$0x3e4ccccd, 0x1c8(%rbp) # imm = 0x3E4CCCCD
141291091: 4c 89 f1                    	movq	%r14, %rcx
141291094: e8 a7 f0 6a 00              	callq	0x141940140 <.text+0x193f140>
141291099: 48 89 c1                    	movq	%rax, %rcx
14129109c: 4c 8b 8d 10 02 00 00        	movq	0x210(%rbp), %r9
1412910a3: 4d 8d 04 39                 	leaq	(%r9,%rdi), %r8
1412910a7: 4c 89 85 10 02 00 00        	movq	%r8, 0x210(%rbp)
1412910ae: 4c 89 c0                    	movq	%r8, %rax
1412910b1: 48 31 f0                    	xorq	%rsi, %rax
1412910b4: 49 f7 e0                    	mulq	%r8
1412910b7: 48 31 c2                    	xorq	%rax, %rdx
1412910ba: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
1412910c0: 83 f8 5f                    	cmpl	$0x5f, %eax
1412910c3: f3 0f 10 05 39 48 69 01     	movss	0x1694839(%rip), %xmm0  # 0x142925904
1412910cb: 77 37                       	ja	0x141291104 <.text+0x1290104>
1412910cd: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1412910d7: 49 01 c1                    	addq	%rax, %r9
1412910da: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1412910e0: 49 01 f8                    	addq	%rdi, %r8
1412910e3: 4c 89 c8                    	movq	%r9, %rax
1412910e6: 48 31 f0                    	xorq	%rsi, %rax
1412910e9: 49 f7 e1                    	mulq	%r9
1412910ec: 48 31 c2                    	xorq	%rax, %rdx
1412910ef: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
1412910f5: 49 01 f9                    	addq	%rdi, %r9
1412910f8: 83 f8 60                    	cmpl	$0x60, %eax
1412910fb: 72 e3                       	jb	0x1412910e0 <.text+0x12900e0>
1412910fd: 4c 89 85 10 02 00 00        	movq	%r8, 0x210(%rbp)
141291104: f3 41 0f 5c c4              	subss	%xmm12, %xmm0
141291109: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
14129110d: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
141291111: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
141291115: 41 0f 59 c3                 	mulps	%xmm11, %xmm0
141291119: f3 41 0f 12 d4              	movsldup	%xmm12, %xmm2           # xmm2 = xmm12[0,0,2,2]
14129111e: 41 0f 59 d2                 	mulps	%xmm10, %xmm2
141291122: 0f 58 d0                    	addps	%xmm0, %xmm2
141291125: f3 45 0f 59 e0              	mulss	%xmm8, %xmm12
14129112a: f3 44 0f 58 e1              	addss	%xmm1, %xmm12
14129112f: 89 d0                       	movl	%edx, %eax
141291131: 48 69 c0 c8 00 00 00        	imulq	$0xc8, %rax, %rax
141291138: 48 c1 e8 20                 	shrq	$0x20, %rax
14129113c: 83 c0 9c                    	addl	$-0x64, %eax
14129113f: 0f 13 95 c0 01 00 00        	movlps	%xmm2, 0x1c0(%rbp)
141291146: f3 44 0f 11 a5 c8 01 00 00  	movss	%xmm12, 0x1c8(%rbp)
14129114f: c7 85 cc 01 00 00 00 00 00 00       	movl	$0x0, 0x1cc(%rbp)
141291159: 48 ba 00 00 00 3c 00 00 ff 7b       	movabsq	$0x7bff00003c000000, %rdx # imm = 0x7BFF00003C000000
141291163: 48 89 95 d0 01 00 00        	movq	%rdx, 0x1d0(%rbp)
14129116a: 89 8d d8 01 00 00           	movl	%ecx, 0x1d8(%rbp)
141291170: 48 c1 e9 20                 	shrq	$0x20, %rcx
141291174: 66 89 8d dc 01 00 00        	movw	%cx, 0x1dc(%rbp)
14129117b: 66 c7 85 de 01 00 00 00 00  	movw	$0x0, 0x1de(%rbp)
141291184: 4c 89 bd e0 01 00 00        	movq	%r15, 0x1e0(%rbp)
14129118b: 8b 8d bc 01 00 00           	movl	0x1bc(%rbp), %ecx
141291191: 89 8d e8 01 00 00           	movl	%ecx, 0x1e8(%rbp)
141291197: 48 b9 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rcx # imm = 0x3F80000000000000
1412911a1: 48 89 8d ec 01 00 00        	movq	%rcx, 0x1ec(%rbp)
1412911a8: 89 85 f4 01 00 00           	movl	%eax, 0x1f4(%rbp)
1412911ae: 48 c7 85 f8 01 00 00 00 00 00 00    	movq	$0x0, 0x1f8(%rbp)
1412911b9: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
1412911c0: 4c 89 f2                    	movq	%r14, %rdx
1412911c3: e8 18 bf ff ff              	callq	0x14128d0e0 <.text+0x128c0e0>
1412911c8: 31 c9                       	xorl	%ecx, %ecx
1412911ca: 48 89 d8                    	movq	%rbx, %rax
1412911cd: 4d 39 e5                    	cmpq	%r12, %r13
1412911d0: 0f 85 3a fe ff ff           	jne	0x141291010 <.text+0x1290010>
1412911d6: 48 8b 95 80 01 00 00        	movq	0x180(%rbp), %rdx
1412911dd: 48 85 d2                    	testq	%rdx, %rdx
1412911e0: 74 16                       	je	0x1412911f8 <.text+0x12901f8>
1412911e2: 48 c1 e2 02                 	shlq	$0x2, %rdx
1412911e6: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412911ec: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
1412911f3: e8 28 2a 38 ff              	callq	0x140613c20 <.text+0x612c20>
1412911f8: f3 0f 10 b5 0c 02 00 00     	movss	0x20c(%rbp), %xmm6
141291200: 0f 28 c6                    	movaps	%xmm6, %xmm0
141291203: f3 0f 5e 05 cd 46 69 01     	divss	0x16946cd(%rip), %xmm0  # 0x1429258d8
14129120b: e8 50 2c 69 01              	callq	0x142923e60 <.text+0x2922e60>
141291210: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
141291215: 48 89 c1                    	movq	%rax, %rcx
141291218: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
14129121c: 0f 28 c8                    	movaps	%xmm0, %xmm1
14129121f: f3 0f 5c 0d b5 8e 6c 01     	subss	0x16c8eb5(%rip), %xmm1  # 0x14295a0dc
141291227: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
14129122c: 48 21 ca                    	andq	%rcx, %rdx
14129122f: 48 09 c2                    	orq	%rax, %rdx
141291232: 45 31 e4                    	xorl	%r12d, %r12d
141291235: 45 0f 57 db                 	xorps	%xmm11, %xmm11
141291239: 41 0f 2e c3                 	ucomiss	%xmm11, %xmm0
14129123d: 49 0f 42 d4                 	cmovbq	%r12, %rdx
141291241: 0f 2e 05 98 8e 6c 01        	ucomiss	0x16c8e98(%rip), %xmm0  # 0x14295a0e0
141291248: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
14129124f: 48 0f 47 d0                 	cmovaq	%rax, %rdx
141291253: 48 83 fa 03                 	cmpq	$0x3, %rdx
141291257: b8 02 00 00 00              	movl	$0x2, %eax
14129125c: 48 0f 42 d0                 	cmovbq	%rax, %rdx
141291260: f3 0f 10 15 64 81 99 01     	movss	0x1998164(%rip), %xmm2  # 0x142c293cc
141291268: f3 0f 5e d6                 	divss	%xmm6, %xmm2
14129126c: 48 8d 05 25 82 99 01        	leaq	0x1998225(%rip), %rax   # 0x142c29498
141291273: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141291278: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
14129127f: 4c 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %r9
141291286: e8 c5 f0 9f ff              	callq	0x140c90350 <.text+0xc8f350>
14129128b: 0f 28 5d b0                 	movaps	-0x50(%rbp), %xmm3
14129128f: 0f 28 d3                    	movaps	%xmm3, %xmm2
141291292: 0f 59 d3                    	mulps	%xmm3, %xmm2
141291295: f3 0f 16 c2                 	movshdup	%xmm2, %xmm0            # xmm0 = xmm2[1,1,3,3]
141291299: f3 0f 58 c2                 	addss	%xmm2, %xmm0
14129129d: 0f 28 ca                    	movaps	%xmm2, %xmm1
1412912a0: 66 0f 15 ca                 	unpckhpd	%xmm2, %xmm1            # xmm1 = xmm1[1],xmm2[1]
1412912a4: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1412912a8: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1412912ac: 0f c6 d2 ff                 	shufps	$0xff, %xmm2, %xmm2     # xmm2 = xmm2[3,3,3,3]
1412912b0: 0f 5c d1                    	subps	%xmm1, %xmm2
1412912b3: 0f 28 c3                    	movaps	%xmm3, %xmm0
1412912b6: 0f c6 c3 d2                 	shufps	$0xd2, %xmm3, %xmm0     # xmm0 = xmm0[2,0],xmm3[1,3]
1412912ba: 0f 28 e3                    	movaps	%xmm3, %xmm4
1412912bd: 0f 58 e3                    	addps	%xmm3, %xmm4
1412912c0: 0f c6 e4 ff                 	shufps	$0xff, %xmm4, %xmm4     # xmm4 = xmm4[3,3,3,3]
1412912c4: 0f 28 2d f5 bd 73 01        	movaps	0x173bdf5(%rip), %xmm5  # 0x1429cd0c0
1412912cb: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
1412912d2: 0f 59 c5                    	mulps	%xmm5, %xmm0
1412912d5: 0f 28 0d c4 7d 71 01        	movaps	0x1717dc4(%rip), %xmm1  # 0x1429a90a0
1412912dc: 0f 59 cb                    	mulps	%xmm3, %xmm1
1412912df: 0f 58 c8                    	addps	%xmm0, %xmm1
1412912e2: 0f c6 c9 d2                 	shufps	$0xd2, %xmm1, %xmm1     # xmm1 = xmm1[2,0,1,3]
1412912e6: 0f 29 a5 f0 00 00 00        	movaps	%xmm4, 0xf0(%rbp)
1412912ed: 0f 59 cc                    	mulps	%xmm4, %xmm1
1412912f0: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
1412912f7: 0f 28 c2                    	movaps	%xmm2, %xmm0
1412912fa: 0f 59 c5                    	mulps	%xmm5, %xmm0
1412912fd: 0f 59 eb                    	mulps	%xmm3, %xmm5
141291300: f3 0f 16 d5                 	movshdup	%xmm5, %xmm2            # xmm2 = xmm5[1,1,3,3]
141291304: f3 0f 58 d5                 	addss	%xmm5, %xmm2
141291308: 0f 12 ed                    	movhlps	%xmm5, %xmm5            # xmm5 = xmm5[1,1]
14129130b: f3 0f 58 ea                 	addss	%xmm2, %xmm5
14129130f: f3 0f 58 ed                 	addss	%xmm5, %xmm5
141291313: 0f c6 ed 00                 	shufps	$0x0, %xmm5, %xmm5      # xmm5 = xmm5[0,0,0,0]
141291317: 0f 29 9d c0 00 00 00        	movaps	%xmm3, 0xc0(%rbp)
14129131e: 0f 59 eb                    	mulps	%xmm3, %xmm5
141291321: 0f 58 e8                    	addps	%xmm0, %xmm5
141291324: 0f 58 e9                    	addps	%xmm1, %xmm5
141291327: 0f 29 ad a0 00 00 00        	movaps	%xmm5, 0xa0(%rbp)
14129132e: 0f 12 ed                    	movhlps	%xmm5, %xmm5            # xmm5 = xmm5[1,1]
141291331: 0f 29 ad b0 00 00 00        	movaps	%xmm5, 0xb0(%rbp)
141291338: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
14129133f: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
141291346: 48 89 8d b0 01 00 00        	movq	%rcx, 0x1b0(%rbp)
14129134d: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
141291351: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
141291358: 45 31 ff                    	xorl	%r15d, %r15d
14129135b: f3 44 0f 10 35 a0 45 69 01  	movss	0x16945a0(%rip), %xmm14 # 0x142925904
141291364: 49 83 fc 70                 	cmpq	$0x70, %r12
141291368: 0f 84 ac 09 00 00           	je	0x141291d1a <.text+0x1290d1a>
14129136e: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
141291375: 4e 8d 2c 20                 	leaq	(%rax,%r12), %r13
141291379: f3 41 0f 10 75 30           	movss	0x30(%r13), %xmm6
14129137f: 0f 28 c6                    	movaps	%xmm6, %xmm0
141291382: f3 0f 5e 05 36 43 83 01     	divss	0x1834336(%rip), %xmm0  # 0x142ac56c0
14129138a: e8 d1 2a 69 01              	callq	0x142923e60 <.text+0x2922e60>
14129138f: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
141291394: 48 89 c1                    	movq	%rax, %rcx
141291397: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
14129139b: 0f 28 c8                    	movaps	%xmm0, %xmm1
14129139e: f3 0f 5c 0d 36 8d 6c 01     	subss	0x16c8d36(%rip), %xmm1  # 0x14295a0dc
1412913a6: f3 4c 0f 2c c1              	cvttss2si	%xmm1, %r8
1412913ab: 49 21 c8                    	andq	%rcx, %r8
1412913ae: 49 09 c0                    	orq	%rax, %r8
1412913b1: 41 0f 2e c3                 	ucomiss	%xmm11, %xmm0
1412913b5: ba 00 00 00 00              	movl	$0x0, %edx
1412913ba: 49 0f 43 d0                 	cmovaeq	%r8, %rdx
1412913be: 0f 2e 05 1b 8d 6c 01        	ucomiss	0x16c8d1b(%rip), %xmm0  # 0x14295a0e0
1412913c5: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
1412913cc: 48 0f 47 d0                 	cmovaq	%rax, %rdx
1412913d0: 48 83 fa 03                 	cmpq	$0x3, %rdx
1412913d4: b8 02 00 00 00              	movl	$0x2, %eax
1412913d9: 48 0f 42 d0                 	cmovbq	%rax, %rdx
1412913dd: f3 0f 10 15 eb 7f 99 01     	movss	0x1997feb(%rip), %xmm2  # 0x142c293d0
1412913e5: f3 0f 5e d6                 	divss	%xmm6, %xmm2
1412913e9: 48 8d 05 c0 80 99 01        	leaq	0x19980c0(%rip), %rax   # 0x142c294b0
1412913f0: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1412913f5: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1412913fc: 4c 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %r9
141291403: e8 48 ef 9f ff              	callq	0x140c90350 <.text+0xc8f350>
141291408: 49 83 c4 38                 	addq	$0x38, %r12
14129140c: 4c 89 a5 40 01 00 00        	movq	%r12, 0x140(%rbp)
141291413: 49 8d 47 01                 	leaq	0x1(%r15), %rax
141291417: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
14129141e: 31 c0                       	xorl	%eax, %eax
141291420: 4d 85 ff                    	testq	%r15, %r15
141291423: 0f 94 c0                    	sete	%al
141291426: 48 8d 0d 43 d5 69 01        	leaq	0x169d543(%rip), %rcx   # 0x14292e970
14129142d: f3 0f 10 04 81              	movss	(%rcx,%rax,4), %xmm0
141291432: 0f 28 c8                    	movaps	%xmm0, %xmm1
141291435: 0f 54 0d b4 44 69 01        	andps	0x16944b4(%rip), %xmm1  # 0x1429258f0
14129143c: 0f 28 d1                    	movaps	%xmm1, %xmm2
14129143f: 0f c6 d0 40                 	shufps	$0x40, %xmm0, %xmm2     # xmm2 = xmm2[0,0],xmm0[0,1]
141291443: 0f c6 c8 00                 	shufps	$0x0, %xmm0, %xmm1      # xmm1 = xmm1[0,0],xmm0[0,0]
141291447: 0f 59 8d d0 00 00 00        	mulps	0xd0(%rbp), %xmm1
14129144e: 0f 28 a5 c0 00 00 00        	movaps	0xc0(%rbp), %xmm4
141291455: 0f 28 c4                    	movaps	%xmm4, %xmm0
141291458: 0f 59 c2                    	mulps	%xmm2, %xmm0
14129145b: f3 0f 16 d8                 	movshdup	%xmm0, %xmm3            # xmm3 = xmm0[1,1,3,3]
14129145f: f3 0f 58 d8                 	addss	%xmm0, %xmm3
141291463: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
141291466: f3 0f 58 c3                 	addss	%xmm3, %xmm0
14129146a: f3 0f 58 c0                 	addss	%xmm0, %xmm0
14129146e: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141291472: 0f 59 c4                    	mulps	%xmm4, %xmm0
141291475: 0f 58 c1                    	addps	%xmm1, %xmm0
141291478: 0f 28 8d e0 00 00 00        	movaps	0xe0(%rbp), %xmm1
14129147f: 0f 59 ca                    	mulps	%xmm2, %xmm1
141291482: 0f c6 d2 d2                 	shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
141291486: 0f 59 d4                    	mulps	%xmm4, %xmm2
141291489: 0f 5c ca                    	subps	%xmm2, %xmm1
14129148c: 0f c6 c9 d2                 	shufps	$0xd2, %xmm1, %xmm1     # xmm1 = xmm1[2,0,1,3]
141291490: 0f 59 8d f0 00 00 00        	mulps	0xf0(%rbp), %xmm1
141291497: 0f 58 c8                    	addps	%xmm0, %xmm1
14129149a: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14129149e: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
1412914a2: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1412914a9: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1412914b0: 48 89 8d a8 01 00 00        	movq	%rcx, 0x1a8(%rbp)
1412914b7: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1412914bb: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1412914c2: 0f 28 c1                    	movaps	%xmm1, %xmm0
1412914c5: 0f c6 c1 e2                 	shufps	$0xe2, %xmm1, %xmm0     # xmm0 = xmm0[2,0],xmm1[2,3]
1412914c9: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
1412914d0: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
1412914d4: 0f c6 c9 e9                 	shufps	$0xe9, %xmm1, %xmm1     # xmm1 = xmm1[1,2,2,3]
1412914d8: 0f 29 8d 90 00 00 00        	movaps	%xmm1, 0x90(%rbp)
1412914df: b1 01                       	movb	$0x1, %cl
1412914e1: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1412914e8: eb 16                       	jmp	0x141291500 <.text+0x1290500>
1412914ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1412914f0: 31 c9                       	xorl	%ecx, %ecx
1412914f2: 48 8b 9d 70 01 00 00        	movq	0x170(%rbp), %rbx
1412914f9: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
141291500: 48 3b 85 78 01 00 00        	cmpq	0x178(%rbp), %rax
141291507: 0f 84 d3 07 00 00           	je	0x141291ce0 <.text+0x1290ce0>
14129150d: 48 8d 50 04                 	leaq	0x4(%rax), %rdx
141291511: f6 c1 01                    	testb	$0x1, %cl
141291514: 74 1a                       	je	0x141291530 <.text+0x1290530>
141291516: 48 3b 95 78 01 00 00        	cmpq	0x178(%rbp), %rdx
14129151d: 0f 84 bd 07 00 00           	je	0x141291ce0 <.text+0x1290ce0>
141291523: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
141291527: 48 89 8d a0 01 00 00        	movq	%rcx, 0x1a0(%rbp)
14129152e: eb 0d                       	jmp	0x14129153d <.text+0x129053d>
141291530: 48 89 95 a0 01 00 00        	movq	%rdx, 0x1a0(%rbp)
141291537: 48 89 c2                    	movq	%rax, %rdx
14129153a: 48 89 d8                    	movq	%rbx, %rax
14129153d: f3 0f 10 00                 	movss	(%rax), %xmm0
141291541: 48 89 95 70 01 00 00        	movq	%rdx, 0x170(%rbp)
141291548: f3 0f 10 12                 	movss	(%rdx), %xmm2
14129154c: 0f 28 c8                    	movaps	%xmm0, %xmm1
14129154f: f3 0f 58 ca                 	addss	%xmm2, %xmm1
141291553: f3 0f 59 0d 15 43 69 01     	mulss	0x1694315(%rip), %xmm1  # 0x142925870
14129155b: f3 0f 5c d0                 	subss	%xmm0, %xmm2
14129155f: f3 0f 10 85 0c 02 00 00     	movss	0x20c(%rbp), %xmm0
141291567: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
14129156b: f3 0f 11 95 9c 01 00 00     	movss	%xmm2, 0x19c(%rbp)
141291573: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
141291577: f3 0f 5c d1                 	subss	%xmm1, %xmm2
14129157b: f3 41 0f 59 d3              	mulss	%xmm11, %xmm2
141291580: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
141291584: f3 0f 5c d1                 	subss	%xmm1, %xmm2
141291588: f3 0f 12 c2                 	movsldup	%xmm2, %xmm0            # xmm0 = xmm2[0,0,2,2]
14129158c: 0f 59 85 a0 00 00 00        	mulps	0xa0(%rbp), %xmm0
141291593: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
141291597: f3 0f 59 95 b0 00 00 00     	mulss	0xb0(%rbp), %xmm2
14129159f: 0f 29 55 40                 	movaps	%xmm2, 0x40(%rbp)
1412915a3: b1 01                       	movb	$0x1, %cl
1412915a5: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
1412915ac: 48 3b 85 68 01 00 00        	cmpq	0x168(%rbp), %rax
1412915b3: 0f 84 37 ff ff ff           	je	0x1412914f0 <.text+0x12904f0>
1412915b9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1412915c0: 48 8d 58 04                 	leaq	0x4(%rax), %rbx
1412915c4: f6 c1 01                    	testb	$0x1, %cl
1412915c7: 74 17                       	je	0x1412915e0 <.text+0x12905e0>
1412915c9: 48 3b 9d 68 01 00 00        	cmpq	0x168(%rbp), %rbx
1412915d0: 0f 84 1a ff ff ff           	je	0x1412914f0 <.text+0x12904f0>
1412915d6: 4c 8d 60 08                 	leaq	0x8(%rax), %r12
1412915da: 49 89 c6                    	movq	%rax, %r14
1412915dd: eb 07                       	jmp	0x1412915e6 <.text+0x12905e6>
1412915df: 90                          	nop
1412915e0: 49 89 dc                    	movq	%rbx, %r12
1412915e3: 48 89 c3                    	movq	%rax, %rbx
1412915e6: f3 41 0f 10 0e              	movss	(%r14), %xmm1
1412915eb: f3 0f 10 3b                 	movss	(%rbx), %xmm7
1412915ef: f3 0f 11 8d 08 02 00 00     	movss	%xmm1, 0x208(%rbp)
1412915f7: 41 0f 2e cb                 	ucomiss	%xmm11, %xmm1
1412915fb: 0f 82 44 07 00 00           	jb	0x141291d45 <.text+0x1290d45>
141291601: 44 0f 2e f1                 	ucomiss	%xmm1, %xmm14
141291605: 0f 82 3a 07 00 00           	jb	0x141291d45 <.text+0x1290d45>
14129160b: 0f 2e c9                    	ucomiss	%xmm1, %xmm1
14129160e: 0f 8a a2 07 00 00           	jp	0x141291db6 <.text+0x1290db6>
141291614: 4c 89 e9                    	movq	%r13, %rcx
141291617: e8 f4 80 fd ff              	callq	0x141269710 <.text+0x1268710>
14129161c: 89 c1                       	movl	%eax, %ecx
14129161e: 4d 8b 75 10                 	movq	0x10(%r13), %r14
141291622: 49 39 ce                    	cmpq	%rcx, %r14
141291625: 0f 86 44 08 00 00           	jbe	0x141291e6f <.text+0x1290e6f>
14129162b: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
14129162f: 4c 39 f0                    	cmpq	%r14, %rax
141291632: 0f 83 23 08 00 00           	jae	0x141291e5b <.text+0x1290e5b>
141291638: 0f 28 f0                    	movaps	%xmm0, %xmm6
14129163b: 4d 8b 7d 08                 	movq	0x8(%r13), %r15
14129163f: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
141291643: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141291647: 45 0f 28 ce                 	movaps	%xmm14, %xmm9
14129164b: f3 44 0f 5c c8              	subss	%xmm0, %xmm9
141291650: f3 45 0f 10 6c 8f 08        	movss	0x8(%r15,%rcx,4), %xmm13
141291657: f3 45 0f 59 e9              	mulss	%xmm9, %xmm13
14129165c: f2 45 0f 10 24 8f           	movsd	(%r15,%rcx,4), %xmm12
141291662: f2 45 0f 10 14 87           	movsd	(%r15,%rax,4), %xmm10
141291668: f3 45 0f 10 44 87 08        	movss	0x8(%r15,%rax,4), %xmm8
14129166f: f3 44 0f 59 c0              	mulss	%xmm0, %xmm8
141291674: f3 0f 11 bd 08 02 00 00     	movss	%xmm7, 0x208(%rbp)
14129167c: 41 0f 2e fb                 	ucomiss	%xmm11, %xmm7
141291680: 0f 82 4d 07 00 00           	jb	0x141291dd3 <.text+0x1290dd3>
141291686: 44 0f 2e f7                 	ucomiss	%xmm7, %xmm14
14129168a: 0f 82 43 07 00 00           	jb	0x141291dd3 <.text+0x1290dd3>
141291690: 0f 2e ff                    	ucomiss	%xmm7, %xmm7
141291693: 0f 8a a8 07 00 00           	jp	0x141291e41 <.text+0x1290e41>
141291699: 4c 89 e9                    	movq	%r13, %rcx
14129169c: 0f 28 cf                    	movaps	%xmm7, %xmm1
14129169f: e8 6c 80 fd ff              	callq	0x141269710 <.text+0x1268710>
1412916a4: 89 c0                       	movl	%eax, %eax
1412916a6: 49 39 c6                    	cmpq	%rax, %r14
1412916a9: 0f 86 e2 07 00 00           	jbe	0x141291e91 <.text+0x1290e91>
1412916af: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
1412916b3: 4c 39 f1                    	cmpq	%r14, %rcx
1412916b6: 0f 83 c4 07 00 00           	jae	0x141291e80 <.text+0x1290e80>
1412916bc: f3 41 0f 12 c9              	movsldup	%xmm9, %xmm1            # xmm1 = xmm9[0,0,2,2]
1412916c1: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
1412916c5: f3 0f 12 ee                 	movsldup	%xmm6, %xmm5            # xmm5 = xmm6[0,0,2,2]
1412916c9: 41 0f 59 ea                 	mulps	%xmm10, %xmm5
1412916cd: 0f 58 e9                    	addps	%xmm1, %xmm5
1412916d0: f3 45 0f 58 e8              	addss	%xmm8, %xmm13
1412916d5: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1412916d9: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
1412916dd: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1412916e1: f3 0f 12 d1                 	movsldup	%xmm1, %xmm2            # xmm2 = xmm1[0,0,2,2]
1412916e5: f3 41 0f 59 4c 87 08        	mulss	0x8(%r15,%rax,4), %xmm1
1412916ec: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1412916f0: f3 0f 12 d8                 	movsldup	%xmm0, %xmm3            # xmm3 = xmm0[0,0,2,2]
1412916f4: f3 45 0f 10 64 8f 08        	movss	0x8(%r15,%rcx,4), %xmm12
1412916fb: f3 44 0f 59 e0              	mulss	%xmm0, %xmm12
141291700: f3 44 0f 58 e1              	addss	%xmm1, %xmm12
141291705: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
141291709: f3 44 0f 11 ad 00 02 00 00  	movss	%xmm13, 0x200(%rbp)
141291712: f3 41 0f 5c c5              	subss	%xmm13, %xmm0
141291717: 0f 28 c8                    	movaps	%xmm0, %xmm1
14129171a: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
14129171e: f2 41 0f 10 24 87           	movsd	(%r15,%rax,4), %xmm4
141291724: f2 45 0f 10 04 8f           	movsd	(%r15,%rcx,4), %xmm8
14129172a: 0f 59 e2                    	mulps	%xmm2, %xmm4
14129172d: 44 0f 59 c3                 	mulps	%xmm3, %xmm8
141291731: 44 0f 58 c4                 	addps	%xmm4, %xmm8
141291735: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
141291739: 0f 29 ad 00 01 00 00        	movaps	%xmm5, 0x100(%rbp)
141291740: 0f 5c d5                    	subps	%xmm5, %xmm2
141291743: 0f 28 da                    	movaps	%xmm2, %xmm3
141291746: 0f 59 da                    	mulps	%xmm2, %xmm3
141291749: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
14129174d: f3 0f 58 e3                 	addss	%xmm3, %xmm4
141291751: f3 0f 58 e1                 	addss	%xmm1, %xmm4
141291755: 0f 57 c9                    	xorps	%xmm1, %xmm1
141291758: f3 0f 51 cc                 	sqrtss	%xmm4, %xmm1
14129175c: 41 0f 28 fe                 	movaps	%xmm14, %xmm7
141291760: f3 0f 5e f9                 	divss	%xmm1, %xmm7
141291764: f3 0f 12 f7                 	movsldup	%xmm7, %xmm6            # xmm6 = xmm7[0,0,2,2]
141291768: 0f 59 f2                    	mulps	%xmm2, %xmm6
14129176b: f3 0f 16 ce                 	movshdup	%xmm6, %xmm1            # xmm1 = xmm6[1,1,3,3]
14129176f: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
141291773: 44 0f 28 ef                 	movaps	%xmm7, %xmm13
141291777: 44 0f c6 ee d4              	shufps	$0xd4, %xmm6, %xmm13    # xmm13 = xmm13[0,1],xmm6[1,3]
14129177c: 44 0f c6 ee f2              	shufps	$0xf2, %xmm6, %xmm13    # xmm13 = xmm13[2,0],xmm6[3,3]
141291781: 44 0f 59 ad 80 00 00 00     	mulps	0x80(%rbp), %xmm13
141291789: 0f 28 c6                    	movaps	%xmm6, %xmm0
14129178c: 0f 16 c7                    	movlhps	%xmm7, %xmm0            # xmm0 = xmm0[0],xmm7[0]
14129178f: 0f c6 c6 f2                 	shufps	$0xf2, %xmm6, %xmm0     # xmm0 = xmm0[2,0],xmm6[3,3]
141291793: 0f 59 85 90 00 00 00        	mulps	0x90(%rbp), %xmm0
14129179a: 44 0f 5c e8                 	subps	%xmm0, %xmm13
14129179e: 44 0f 28 7d 70              	movaps	0x70(%rbp), %xmm15
1412917a3: f3 44 0f 59 fe              	mulss	%xmm6, %xmm15
1412917a8: 0f 28 65 60                 	movaps	0x60(%rbp), %xmm4
1412917ac: f3 0f 59 cc                 	mulss	%xmm4, %xmm1
1412917b0: f3 44 0f 5c f9              	subss	%xmm1, %xmm15
1412917b5: 44 0f 2e df                 	ucomiss	%xmm7, %xmm11
1412917b9: f3 41 0f 16 c5              	movshdup	%xmm13, %xmm0           # xmm0 = xmm13[1,1,3,3]
1412917be: 0f 83 8c 00 00 00           	jae	0x141291850 <.text+0x1290850>
1412917c4: f3 0f 58 c4                 	addss	%xmm4, %xmm0
1412917c8: 0f 28 d7                    	movaps	%xmm7, %xmm2
1412917cb: f3 41 0f 58 d6              	addss	%xmm14, %xmm2
1412917d0: 44 0f 2e d8                 	ucomiss	%xmm0, %xmm11
1412917d4: 0f 83 d6 00 00 00           	jae	0x1412918b0 <.text+0x12908b0>
1412917da: 0f 28 cc                    	movaps	%xmm4, %xmm1
1412917dd: 0f 14 ce                    	unpcklps	%xmm6, %xmm1            # xmm1 = xmm1[0],xmm6[0],xmm1[1],xmm6[1]
1412917e0: f3 41 0f 10 cf              	movss	%xmm15, %xmm1           # xmm1 = xmm15[0],xmm1[1,2,3]
1412917e5: 0f c6 d4 d4                 	shufps	$0xd4, %xmm4, %xmm2     # xmm2 = xmm2[0,1],xmm4[1,3]
1412917e9: 0f 28 d9                    	movaps	%xmm1, %xmm3
1412917ec: 0f c6 da 24                 	shufps	$0x24, %xmm2, %xmm3     # xmm3 = xmm3[0,1],xmm2[2,0]
1412917f0: 0f 28 d4                    	movaps	%xmm4, %xmm2
1412917f3: 0f c6 d6 12                 	shufps	$0x12, %xmm6, %xmm2     # xmm2 = xmm2[2,0],xmm6[1,0]
1412917f7: 41 0f 28 e5                 	movaps	%xmm13, %xmm4
1412917fb: 0f c6 e6 f4                 	shufps	$0xf4, %xmm6, %xmm4     # xmm4 = xmm4[0,1],xmm6[3,3]
1412917ff: 0f 28 ea                    	movaps	%xmm2, %xmm5
141291802: 0f c6 ec 82                 	shufps	$0x82, %xmm4, %xmm5     # xmm5 = xmm5[2,0],xmm4[0,2]
141291806: 41 0f 16 c5                 	movlhps	%xmm13, %xmm0           # xmm0 = xmm0[0],xmm13[0]
14129180a: 0f c6 d0 22                 	shufps	$0x22, %xmm0, %xmm2     # xmm2 = xmm2[2,0],xmm0[2,0]
14129180e: 0f 58 d3                    	addps	%xmm3, %xmm2
141291811: 0f 5c cd                    	subps	%xmm5, %xmm1
141291814: 0f 28 c2                    	movaps	%xmm2, %xmm0
141291817: 0f c6 c1 23                 	shufps	$0x23, %xmm1, %xmm0     # xmm0 = xmm0[3,0],xmm1[2,0]
14129181b: 0f c6 c8 24                 	shufps	$0x24, %xmm0, %xmm1     # xmm1 = xmm1[0,1],xmm0[2,0]
14129181f: 0f c6 d2 ff                 	shufps	$0xff, %xmm2, %xmm2     # xmm2 = xmm2[3,3,3,3]
141291823: 0f 57 c0                    	xorps	%xmm0, %xmm0
141291826: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
14129182a: f3 44 0f 10 15 3d 40 69 01  	movss	0x169403d(%rip), %xmm10 # 0x142925870
141291833: f3 44 0f 5e d0              	divss	%xmm0, %xmm10
141291838: 45 0f c6 d2 00              	shufps	$0x0, %xmm10, %xmm10    # xmm10 = xmm10[0,0,0,0]
14129183d: 44 0f 59 d1                 	mulps	%xmm1, %xmm10
141291841: e9 15 01 00 00              	jmp	0x14129195b <.text+0x129095b>
141291846: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141291850: f3 0f 5c c4                 	subss	%xmm4, %xmm0
141291854: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
141291858: f3 0f 5c cf                 	subss	%xmm7, %xmm1
14129185c: 0f 28 d4                    	movaps	%xmm4, %xmm2
14129185f: 44 0f 2e d8                 	ucomiss	%xmm0, %xmm11
141291863: 0f 83 a7 00 00 00           	jae	0x141291910 <.text+0x1290910>
141291869: 0f c6 d6 15                 	shufps	$0x15, %xmm6, %xmm2     # xmm2 = xmm2[1,1],xmm6[1,0]
14129186d: 0f c6 cc d4                 	shufps	$0xd4, %xmm4, %xmm1     # xmm1 = xmm1[0,1],xmm4[1,3]
141291871: 0f c6 ce 12                 	shufps	$0x12, %xmm6, %xmm1     # xmm1 = xmm1[2,0],xmm6[1,0]
141291875: f3 0f 12 dc                 	movsldup	%xmm4, %xmm3            # xmm3 = xmm4[0,0,2,2]
141291879: 0f 5c d3                    	subps	%xmm3, %xmm2
14129187c: 41 0f 28 dd                 	movaps	%xmm13, %xmm3
141291880: 0f 14 d8                    	unpcklps	%xmm0, %xmm3            # xmm3 = xmm3[0],xmm0[0],xmm3[1],xmm0[1]
141291883: 0f 28 c4                    	movaps	%xmm4, %xmm0
141291886: f2 41 0f 10 c7              	movsd	%xmm15, %xmm0           # xmm0 = xmm15[0],xmm0[1]
14129188b: 0f c6 d8 84                 	shufps	$0x84, %xmm0, %xmm3     # xmm3 = xmm3[0,1],xmm0[0,2]
14129188f: 0f 58 d9                    	addps	%xmm1, %xmm3
141291892: 0f c6 d3 23                 	shufps	$0x23, %xmm3, %xmm2     # xmm2 = xmm2[3,0],xmm3[2,0]
141291896: f3 0f 16 c3                 	movshdup	%xmm3, %xmm0            # xmm0 = xmm3[1,1,3,3]
14129189a: 0f c6 da 24                 	shufps	$0x24, %xmm2, %xmm3     # xmm3 = xmm3[0,1],xmm2[2,0]
14129189e: f3 0f 51 c0                 	sqrtss	%xmm0, %xmm0
1412918a2: eb 44                       	jmp	0x1412918e8 <.text+0x12908e8>
1412918a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1412918b0: 0f c6 d4 d4                 	shufps	$0xd4, %xmm4, %xmm2     # xmm2 = xmm2[0,1],xmm4[1,3]
1412918b4: 0f 28 ce                    	movaps	%xmm6, %xmm1
1412918b7: 0f c6 ca 84                 	shufps	$0x84, %xmm2, %xmm1     # xmm1 = xmm1[0,1],xmm2[0,2]
1412918bb: 0f 28 d4                    	movaps	%xmm4, %xmm2
1412918be: f2 41 0f 10 d7              	movsd	%xmm15, %xmm2           # xmm2 = xmm15[0],xmm2[1]
1412918c3: 66 0f 28 da                 	movapd	%xmm2, %xmm3
1412918c7: 41 0f c6 dd 22              	shufps	$0x22, %xmm13, %xmm3    # xmm3 = xmm3[2,0],xmm13[2,0]
1412918cc: 41 0f 16 c5                 	movlhps	%xmm13, %xmm0           # xmm0 = xmm0[0],xmm13[0]
1412918d0: 0f c6 d0 82                 	shufps	$0x82, %xmm0, %xmm2     # xmm2 = xmm2[2,0],xmm0[0,2]
1412918d4: 0f 5c ca                    	subps	%xmm2, %xmm1
1412918d7: 0f 58 de                    	addps	%xmm6, %xmm3
1412918da: 0f c6 d9 e4                 	shufps	$0xe4, %xmm1, %xmm3     # xmm3 = xmm3[0,1],xmm1[2,3]
1412918de: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1412918e1: 0f 57 c0                    	xorps	%xmm0, %xmm0
1412918e4: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1412918e8: f3 44 0f 10 15 7f 3f 69 01  	movss	0x1693f7f(%rip), %xmm10 # 0x142925870
1412918f1: f3 44 0f 5e d0              	divss	%xmm0, %xmm10
1412918f6: 45 0f c6 d2 00              	shufps	$0x0, %xmm10, %xmm10    # xmm10 = xmm10[0,0,0,0]
1412918fb: 44 0f 59 d3                 	mulps	%xmm3, %xmm10
1412918ff: eb 5a                       	jmp	0x14129195b <.text+0x129095b>
141291901: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141291910: f3 0f 10 d1                 	movss	%xmm1, %xmm2            # xmm2 = xmm1[0],xmm2[1,2,3]
141291914: 0f 28 cc                    	movaps	%xmm4, %xmm1
141291917: f2 41 0f 10 cf              	movsd	%xmm15, %xmm1           # xmm1 = xmm15[0],xmm1[1]
14129191c: 0f c6 d1 24                 	shufps	$0x24, %xmm1, %xmm2     # xmm2 = xmm2[0,1],xmm1[2,0]
141291920: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
141291924: 0f c6 ce 40                 	shufps	$0x40, %xmm6, %xmm1     # xmm1 = xmm1[0,0],xmm6[0,1]
141291928: 0f 28 dc                    	movaps	%xmm4, %xmm3
14129192b: 0f 58 d9                    	addps	%xmm1, %xmm3
14129192e: f3 0f 10 c8                 	movss	%xmm0, %xmm1            # xmm1 = xmm0[0],xmm1[1,2,3]
141291932: 0f 5c d1                    	subps	%xmm1, %xmm2
141291935: 0f 57 c0                    	xorps	%xmm0, %xmm0
141291938: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
14129193c: 0f c6 d3 9c                 	shufps	$0x9c, %xmm3, %xmm2     # xmm2 = xmm2[0,3],xmm3[1,2]
141291940: 0f c6 d2 78                 	shufps	$0x78, %xmm2, %xmm2     # xmm2 = xmm2[0,2,3,1]
141291944: f3 44 0f 10 15 23 3f 69 01  	movss	0x1693f23(%rip), %xmm10 # 0x142925870
14129194d: f3 44 0f 5e d0              	divss	%xmm0, %xmm10
141291952: 45 0f c6 d2 00              	shufps	$0x0, %xmm10, %xmm10    # xmm10 = xmm10[0,0,0,0]
141291957: 44 0f 59 d2                 	mulps	%xmm2, %xmm10
14129195b: 0f 29 7d 30                 	movaps	%xmm7, 0x30(%rbp)
14129195f: 0f 29 75 20                 	movaps	%xmm6, 0x20(%rbp)
141291963: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
14129196a: e8 11 da a1 ff              	callq	0x140caf380 <.text+0xcae380>
14129196f: 0f 28 f8                    	movaps	%xmm0, %xmm7
141291972: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
141291979: e8 02 da a1 ff              	callq	0x140caf380 <.text+0xcae380>
14129197e: 0f 28 f0                    	movaps	%xmm0, %xmm6
141291981: 45 0f 28 da                 	movaps	%xmm10, %xmm11
141291985: 45 0f c6 da ff              	shufps	$0xff, %xmm10, %xmm11   # xmm11 = xmm11[3,3],xmm10[3,3]
14129198a: 0f 28 c7                    	movaps	%xmm7, %xmm0
14129198d: f3 0f 59 05 f3 3f 69 01     	mulss	0x1693ff3(%rip), %xmm0  # 0x142925988
141291995: 45 0f 28 c8                 	movaps	%xmm8, %xmm9
141291999: 45 0f 28 c6                 	movaps	%xmm14, %xmm8
14129199d: f3 44 0f 5c c7              	subss	%xmm7, %xmm8
1412919a2: f3 45 0f 58 c0              	addss	%xmm8, %xmm8
1412919a7: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
1412919ac: f3 44 0f 59 05 fb 9a 81 01  	mulss	0x1819afb(%rip), %xmm8  # 0x142aab4b0
1412919b5: f3 44 0f 59 05 b2 3e 69 01  	mulss	0x1693eb2(%rip), %xmm8  # 0x142925870
1412919be: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1412919c2: e8 09 26 69 01              	callq	0x142923fd0 <.text+0x2922fd0>
1412919c7: 0f 28 f8                    	movaps	%xmm0, %xmm7
1412919ca: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
1412919ce: f3 44 0f 59 35 b9 ec 6c 01  	mulss	0x16cecb9(%rip), %xmm14 # 0x142960690
1412919d7: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1412919db: 45 0f 28 c1                 	movaps	%xmm9, %xmm8
1412919df: e8 9c 24 69 01              	callq	0x142923e80 <.text+0x2922e80>
1412919e4: 41 0f 14 fe                 	unpcklps	%xmm14, %xmm7           # xmm7 = xmm7[0],xmm14[0],xmm7[1],xmm14[1]
1412919e8: 44 0f 14 f0                 	unpcklps	%xmm0, %xmm14           # xmm14 = xmm14[0],xmm0[0],xmm14[1],xmm0[1]
1412919ec: 0f 28 c7                    	movaps	%xmm7, %xmm0
1412919ef: 41 0f 16 c6                 	movlhps	%xmm14, %xmm0           # xmm0 = xmm0[0],xmm14[0]
1412919f3: 41 0f 59 c3                 	mulps	%xmm11, %xmm0
1412919f7: 45 0f 57 db                 	xorps	%xmm11, %xmm11
1412919fb: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1412919ff: 41 0f c6 ca 00              	shufps	$0x0, %xmm10, %xmm1     # xmm1 = xmm1[0,0],xmm10[0,0]
141291a04: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
141291a08: 0f c6 d7 11                 	shufps	$0x11, %xmm7, %xmm2     # xmm2 = xmm2[1,0],xmm7[1,0]
141291a0c: 0f 59 d1                    	mulps	%xmm1, %xmm2
141291a0f: 0f 28 1d 5a b7 73 01        	movaps	0x173b75a(%rip), %xmm3  # 0x1429cd170
141291a16: 0f 59 d3                    	mulps	%xmm3, %xmm2
141291a19: 0f 58 d0                    	addps	%xmm0, %xmm2
141291a1c: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
141291a20: 41 0f c6 c2 55              	shufps	$0x55, %xmm10, %xmm0    # xmm0 = xmm0[1,1],xmm10[1,1]
141291a25: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
141291a29: 0f 16 cf                    	movlhps	%xmm7, %xmm1            # xmm1 = xmm1[0],xmm7[0]
141291a2c: 0f 59 c8                    	mulps	%xmm0, %xmm1
141291a2f: 41 0f c6 fe 11              	shufps	$0x11, %xmm14, %xmm7    # xmm7 = xmm7[1,0],xmm14[1,0]
141291a34: f3 44 0f 10 35 c7 3e 69 01  	movss	0x1693ec7(%rip), %xmm14 # 0x142925904
141291a3d: 45 0f c6 d2 aa              	shufps	$0xaa, %xmm10, %xmm10   # xmm10 = xmm10[2,2,2,2]
141291a42: 41 0f 59 fa                 	mulps	%xmm10, %xmm7
141291a46: 0f 28 25 33 b7 73 01        	movaps	0x173b733(%rip), %xmm4  # 0x1429cd180
141291a4d: 0f 59 cc                    	mulps	%xmm4, %xmm1
141291a50: 0f 28 2d 39 b7 73 01        	movaps	0x173b739(%rip), %xmm5  # 0x1429cd190
141291a57: 0f 59 fd                    	mulps	%xmm5, %xmm7
141291a5a: 0f 58 f9                    	addps	%xmm1, %xmm7
141291a5d: 0f 58 fa                    	addps	%xmm2, %xmm7
141291a60: 0f 28 c7                    	movaps	%xmm7, %xmm0
141291a63: 0f c6 c7 ff                 	shufps	$0xff, %xmm7, %xmm0     # xmm0 = xmm0[3,3],xmm7[3,3]
141291a67: 0f 28 15 62 02 7f 01        	movaps	0x17f0262(%rip), %xmm2  # 0x142a81cd0
141291a6e: 0f 59 c2                    	mulps	%xmm2, %xmm0
141291a71: 0f 28 cf                    	movaps	%xmm7, %xmm1
141291a74: 0f c6 cf 00                 	shufps	$0x0, %xmm7, %xmm1      # xmm1 = xmm1[0,0],xmm7[0,0]
141291a78: 0f 59 ca                    	mulps	%xmm2, %xmm1
141291a7b: 0f 59 cb                    	mulps	%xmm3, %xmm1
141291a7e: 0f 58 c8                    	addps	%xmm0, %xmm1
141291a81: 0f 28 c7                    	movaps	%xmm7, %xmm0
141291a84: 0f c6 c7 55                 	shufps	$0x55, %xmm7, %xmm0     # xmm0 = xmm0[1,1],xmm7[1,1]
141291a88: 0f 28 15 51 02 7f 01        	movaps	0x17f0251(%rip), %xmm2  # 0x142a81ce0
141291a8f: 0f 59 c2                    	mulps	%xmm2, %xmm0
141291a92: 0f 59 c4                    	mulps	%xmm4, %xmm0
141291a95: 0f c6 ff aa                 	shufps	$0xaa, %xmm7, %xmm7     # xmm7 = xmm7[2,2,2,2]
141291a99: 0f 59 fa                    	mulps	%xmm2, %xmm7
141291a9c: 0f 59 fd                    	mulps	%xmm5, %xmm7
141291a9f: 0f 58 f8                    	addps	%xmm0, %xmm7
141291aa2: 0f 58 f9                    	addps	%xmm1, %xmm7
141291aa5: 0f 29 7d 10                 	movaps	%xmm7, 0x10(%rbp)
141291aa9: 41 0f 28 fe                 	movaps	%xmm14, %xmm7
141291aad: f3 0f 5c fe                 	subss	%xmm6, %xmm7
141291ab1: f3 0f 59 3d cb 3e 69 01     	mulss	0x1693ecb(%rip), %xmm7  # 0x142925984
141291ab9: f3 0f 59 35 a7 b5 73 01     	mulss	0x173b5a7(%rip), %xmm6  # 0x1429cd068
141291ac1: f3 44 0f 59 3d 0a 79 99 01  	mulss	0x199790a(%rip), %xmm15 # 0x142c293d4
141291aca: 44 0f 59 2d 0e 79 99 01     	mulps	0x199790e(%rip), %xmm13 # 0x142c293e0
141291ad2: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
141291ad6: e8 d5 56 d7 00              	callq	0x1420071b0 <.text+0x20061b0>
141291adb: 49 89 c6                    	movq	%rax, %r14
141291ade: f3 0f 58 f7                 	addss	%xmm7, %xmm6
141291ae2: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
141291ae9: 41 0f 5c c0                 	subps	%xmm8, %xmm0
141291aed: f3 0f 10 8d 00 02 00 00     	movss	0x200(%rbp), %xmm1
141291af5: f3 41 0f 5c cc              	subss	%xmm12, %xmm1
141291afa: 0f 59 c0                    	mulps	%xmm0, %xmm0
141291afd: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
141291b01: f3 0f 58 d0                 	addss	%xmm0, %xmm2
141291b05: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
141291b09: f3 0f 58 ca                 	addss	%xmm2, %xmm1
141291b0d: 0f 57 c0                    	xorps	%xmm0, %xmm0
141291b10: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
141291b14: f3 0f 58 c6                 	addss	%xmm6, %xmm0
141291b18: f3 0f 10 8d 9c 01 00 00     	movss	0x19c(%rbp), %xmm1
141291b20: f3 0f 11 8d c0 01 00 00     	movss	%xmm1, 0x1c0(%rbp)
141291b28: f3 0f 11 85 c4 01 00 00     	movss	%xmm0, 0x1c4(%rbp)
141291b30: c7 85 c8 01 00 00 0b d7 a3 3d       	movl	$0x3da3d70b, 0x1c8(%rbp) # imm = 0x3DA3D70B
141291b3a: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
141291b41: e8 fa e5 6a 00              	callq	0x141940140 <.text+0x193f140>
141291b46: 48 89 c1                    	movq	%rax, %rcx
141291b49: 4c 8b 8d 10 02 00 00        	movq	0x210(%rbp), %r9
141291b50: 4d 8d 04 39                 	leaq	(%r9,%rdi), %r8
141291b54: 4c 89 85 10 02 00 00        	movq	%r8, 0x210(%rbp)
141291b5b: 4c 89 c0                    	movq	%r8, %rax
141291b5e: 48 31 f0                    	xorq	%rsi, %rax
141291b61: 49 f7 e0                    	mulq	%r8
141291b64: 48 31 c2                    	xorq	%rax, %rdx
141291b67: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
141291b6d: 83 f8 5f                    	cmpl	$0x5f, %eax
141291b70: 0f 28 55 20                 	movaps	0x20(%rbp), %xmm2
141291b74: 0f 28 5d 30                 	movaps	0x30(%rbp), %xmm3
141291b78: 77 3a                       	ja	0x141291bb4 <.text+0x1290bb4>
141291b7a: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
141291b84: 49 01 c1                    	addq	%rax, %r9
141291b87: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141291b90: 49 01 f8                    	addq	%rdi, %r8
141291b93: 4c 89 c8                    	movq	%r9, %rax
141291b96: 48 31 f0                    	xorq	%rsi, %rax
141291b99: 49 f7 e1                    	mulq	%r9
141291b9c: 48 31 c2                    	xorq	%rax, %rdx
141291b9f: 69 c2 c8 00 00 00           	imull	$0xc8, %edx, %eax
141291ba5: 49 01 f9                    	addq	%rdi, %r9
141291ba8: 83 f8 60                    	cmpl	$0x60, %eax
141291bab: 72 e3                       	jb	0x141291b90 <.text+0x1290b90>
141291bad: 4c 89 85 10 02 00 00        	movq	%r8, 0x210(%rbp)
141291bb4: f3 0f 10 05 b4 3c 69 01     	movss	0x1693cb4(%rip), %xmm0  # 0x142925870
141291bbc: f3 0f 10 8d 00 02 00 00     	movss	0x200(%rbp), %xmm1
141291bc4: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
141291bc8: f3 44 0f 59 e0              	mulss	%xmm0, %xmm12
141291bcd: f3 44 0f 58 e1              	addss	%xmm1, %xmm12
141291bd2: f3 0f 59 de                 	mulss	%xmm6, %xmm3
141291bd6: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
141291bda: f3 41 0f 58 dc              	addss	%xmm12, %xmm3
141291bdf: f3 44 0f 59 f8              	mulss	%xmm0, %xmm15
141291be4: f3 44 0f 58 fb              	addss	%xmm3, %xmm15
141291be9: f3 44 0f 58 7d 40           	addss	0x40(%rbp), %xmm15
141291bef: 0f 28 0d ea 3c 69 01        	movaps	0x1693cea(%rip), %xmm1  # 0x1429258e0
141291bf6: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
141291bfd: 0f 59 c1                    	mulps	%xmm1, %xmm0
141291c00: 44 0f 59 c1                 	mulps	%xmm1, %xmm8
141291c04: 44 0f 58 c0                 	addps	%xmm0, %xmm8
141291c08: f3 0f 12 c6                 	movsldup	%xmm6, %xmm0            # xmm0 = xmm6[0,0,2,2]
141291c0c: 0f 59 d0                    	mulps	%xmm0, %xmm2
141291c0f: 0f 59 d1                    	mulps	%xmm1, %xmm2
141291c12: 41 0f 58 d0                 	addps	%xmm8, %xmm2
141291c16: 44 0f 59 e9                 	mulps	%xmm1, %xmm13
141291c1a: 44 0f 58 ea                 	addps	%xmm2, %xmm13
141291c1e: 44 0f 58 6d 50              	addps	0x50(%rbp), %xmm13
141291c23: 89 d0                       	movl	%edx, %eax
141291c25: 48 69 c0 c8 00 00 00        	imulq	$0xc8, %rax, %rax
141291c2c: 48 c1 e8 20                 	shrq	$0x20, %rax
141291c30: 83 c0 9c                    	addl	$-0x64, %eax
141291c33: 44 0f 13 ad c0 01 00 00     	movlps	%xmm13, 0x1c0(%rbp)
141291c3b: f3 44 0f 11 bd c8 01 00 00  	movss	%xmm15, 0x1c8(%rbp)
141291c44: c7 85 cc 01 00 00 00 00 00 00       	movl	$0x0, 0x1cc(%rbp)
141291c4e: 48 ba 00 00 00 3c 00 00 ff 7b       	movabsq	$0x7bff00003c000000, %rdx # imm = 0x7BFF00003C000000
141291c58: 48 89 95 d0 01 00 00        	movq	%rdx, 0x1d0(%rbp)
141291c5f: 89 8d d8 01 00 00           	movl	%ecx, 0x1d8(%rbp)
141291c65: 48 c1 e9 20                 	shrq	$0x20, %rcx
141291c69: 66 89 8d dc 01 00 00        	movw	%cx, 0x1dc(%rbp)
141291c70: 66 c7 85 de 01 00 00 00 00  	movw	$0x0, 0x1de(%rbp)
141291c79: 4c 89 b5 e0 01 00 00        	movq	%r14, 0x1e0(%rbp)
141291c80: 8b 8d bc 01 00 00           	movl	0x1bc(%rbp), %ecx
141291c86: 89 8d e8 01 00 00           	movl	%ecx, 0x1e8(%rbp)
141291c8c: 48 b9 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rcx # imm = 0x3F80000000000000
141291c96: 48 89 8d ec 01 00 00        	movq	%rcx, 0x1ec(%rbp)
141291c9d: 89 85 f4 01 00 00           	movl	%eax, 0x1f4(%rbp)
141291ca3: 48 c7 85 f8 01 00 00 00 00 00 00    	movq	$0x0, 0x1f8(%rbp)
141291cae: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
141291cb5: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
141291cbc: e8 1f b4 ff ff              	callq	0x14128d0e0 <.text+0x128c0e0>
141291cc1: 31 c9                       	xorl	%ecx, %ecx
141291cc3: 4c 89 e0                    	movq	%r12, %rax
141291cc6: 49 89 de                    	movq	%rbx, %r14
141291cc9: 48 3b 85 68 01 00 00        	cmpq	0x168(%rbp), %rax
141291cd0: 0f 85 ea f8 ff ff           	jne	0x1412915c0 <.text+0x12905c0>
141291cd6: e9 15 f8 ff ff              	jmp	0x1412914f0 <.text+0x12904f0>
141291cdb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141291ce0: 48 8b 95 80 01 00 00        	movq	0x180(%rbp), %rdx
141291ce7: 48 85 d2                    	testq	%rdx, %rdx
141291cea: 74 16                       	je	0x141291d02 <.text+0x1290d02>
141291cec: 48 c1 e2 02                 	shlq	$0x2, %rdx
141291cf0: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141291cf6: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
141291cfd: e8 1e 1f 38 ff              	callq	0x140613c20 <.text+0x612c20>
141291d02: 4c 8b bd 48 01 00 00        	movq	0x148(%rbp), %r15
141291d09: 4c 8b a5 40 01 00 00        	movq	0x140(%rbp), %r12
141291d10: 49 83 fc 70                 	cmpq	$0x70, %r12
141291d14: 0f 85 54 f6 ff ff           	jne	0x14129136e <.text+0x129036e>
141291d1a: 48 8b 95 18 01 00 00        	movq	0x118(%rbp), %rdx
141291d21: 48 85 d2                    	testq	%rdx, %rdx
141291d24: 0f 84 b5 ee ff ff           	je	0x141290bdf <.text+0x128fbdf>
141291d2a: 48 c1 e2 02                 	shlq	$0x2, %rdx
141291d2e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141291d34: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
141291d3b: e8 e0 1e 38 ff              	callq	0x140613c20 <.text+0x612c20>
141291d40: e9 9a ee ff ff              	jmp	0x141290bdf <.text+0x128fbdf>
141291d45: 48 8d 85 08 02 00 00        	leaq	0x208(%rbp), %rax
141291d4c: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
141291d53: 48 8d 05 b6 ab af ff        	leaq	-0x50544a(%rip), %rax   # 0x140d8c910 <.text+0xd8b910>
141291d5a: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141291d61: 48 8d 05 68 19 99 01        	leaq	0x1991968(%rip), %rax   # 0x142c236d0
141291d68: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
141291d6f: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
141291d7a: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
141291d85: 48 8d 85 58 01 00 00        	leaq	0x158(%rbp), %rax
141291d8c: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
141291d93: 48 c7 85 d8 01 00 00 01 00 00 00    	movq	$0x1, 0x1d8(%rbp)
141291d9e: 48 8d 15 23 77 99 01        	leaq	0x1997723(%rip), %rdx   # 0x142c294c8
141291da5: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
141291dac: e8 df 75 64 01              	callq	0x1428d9390 <.text+0x28d8390>
141291db1: e9 ed 00 00 00              	jmp	0x141291ea3 <.text+0x1290ea3>
141291db6: 48 8d 0d 23 19 99 01        	leaq	0x1991923(%rip), %rcx   # 0x142c236e0
141291dbd: 4c 8d 05 04 77 99 01        	leaq	0x1997704(%rip), %r8    # 0x142c294c8
141291dc4: ba 1d 00 00 00              	movl	$0x1d, %edx
141291dc9: e8 62 76 64 01              	callq	0x1428d9430 <.text+0x28d8430>
141291dce: e9 d0 00 00 00              	jmp	0x141291ea3 <.text+0x1290ea3>
141291dd3: 48 8d 85 08 02 00 00        	leaq	0x208(%rbp), %rax
141291dda: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
141291de1: 48 8d 05 28 ab af ff        	leaq	-0x5054d8(%rip), %rax   # 0x140d8c910 <.text+0xd8b910>
141291de8: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141291def: 48 8d 05 da 18 99 01        	leaq	0x19918da(%rip), %rax   # 0x142c236d0
141291df6: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
141291dfd: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
141291e08: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
141291e13: 48 8d 85 58 01 00 00        	leaq	0x158(%rbp), %rax
141291e1a: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
141291e21: 48 c7 85 d8 01 00 00 01 00 00 00    	movq	$0x1, 0x1d8(%rbp)
141291e2c: 48 8d 15 ad 76 99 01        	leaq	0x19976ad(%rip), %rdx   # 0x142c294e0
141291e33: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
141291e3a: e8 51 75 64 01              	callq	0x1428d9390 <.text+0x28d8390>
141291e3f: eb 62                       	jmp	0x141291ea3 <.text+0x1290ea3>
141291e41: 48 8d 0d 98 18 99 01        	leaq	0x1991898(%rip), %rcx   # 0x142c236e0
141291e48: 4c 8d 05 91 76 99 01        	leaq	0x1997691(%rip), %r8    # 0x142c294e0
141291e4f: ba 1d 00 00 00              	movl	$0x1d, %edx
141291e54: e8 d7 75 64 01              	callq	0x1428d9430 <.text+0x28d8430>
141291e59: eb 48                       	jmp	0x141291ea3 <.text+0x1290ea3>
141291e5b: 4c 8d 05 9e 07 99 01        	leaq	0x199079e(%rip), %r8    # 0x142c22600
141291e62: 48 89 c1                    	movq	%rax, %rcx
141291e65: 4c 89 f2                    	movq	%r14, %rdx
141291e68: e8 ab 76 64 01              	callq	0x1428d9518 <.text+0x28d8518>
141291e6d: eb 34                       	jmp	0x141291ea3 <.text+0x1290ea3>
141291e6f: 4c 8d 05 72 07 99 01        	leaq	0x1990772(%rip), %r8    # 0x142c225e8
141291e76: 4c 89 f2                    	movq	%r14, %rdx
141291e79: e8 9a 76 64 01              	callq	0x1428d9518 <.text+0x28d8518>
141291e7e: eb 23                       	jmp	0x141291ea3 <.text+0x1290ea3>
141291e80: 4c 8d 05 79 07 99 01        	leaq	0x1990779(%rip), %r8    # 0x142c22600
141291e87: 4c 89 f2                    	movq	%r14, %rdx
141291e8a: e8 89 76 64 01              	callq	0x1428d9518 <.text+0x28d8518>
141291e8f: eb 12                       	jmp	0x141291ea3 <.text+0x1290ea3>
141291e91: 4c 8d 05 50 07 99 01        	leaq	0x1990750(%rip), %r8    # 0x142c225e8
141291e98: 48 89 c1                    	movq	%rax, %rcx
141291e9b: 4c 89 f2                    	movq	%r14, %rdx
141291e9e: e8 75 76 64 01              	callq	0x1428d9518 <.text+0x28d8518>
141291ea3: 0f 0b                       	ud2
141291ea5: 48 8d 0d a4 75 99 01        	leaq	0x19975a4(%rip), %rcx   # 0x142c29450
141291eac: e8 5f 74 64 01              	callq	0x1428d9310 <.text+0x28d8310>
