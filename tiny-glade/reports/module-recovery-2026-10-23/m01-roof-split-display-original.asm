
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14127a790: 55                          	pushq	%rbp
14127a791: 41 57                       	pushq	%r15
14127a793: 41 56                       	pushq	%r14
14127a795: 41 55                       	pushq	%r13
14127a797: 41 54                       	pushq	%r12
14127a799: 56                          	pushq	%rsi
14127a79a: 57                          	pushq	%rdi
14127a79b: 53                          	pushq	%rbx
14127a79c: 48 81 ec 38 02 00 00        	subq	$0x238, %rsp            # imm = 0x238
14127a7a3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14127a7ab: 44 0f 29 bd a0 01 00 00     	movaps	%xmm15, 0x1a0(%rbp)
14127a7b3: 44 0f 29 b5 90 01 00 00     	movaps	%xmm14, 0x190(%rbp)
14127a7bb: 44 0f 29 ad 80 01 00 00     	movaps	%xmm13, 0x180(%rbp)
14127a7c3: 44 0f 29 a5 70 01 00 00     	movaps	%xmm12, 0x170(%rbp)
14127a7cb: 44 0f 29 9d 60 01 00 00     	movaps	%xmm11, 0x160(%rbp)
14127a7d3: 44 0f 29 95 50 01 00 00     	movaps	%xmm10, 0x150(%rbp)
14127a7db: 44 0f 29 8d 40 01 00 00     	movaps	%xmm9, 0x140(%rbp)
14127a7e3: 44 0f 29 85 30 01 00 00     	movaps	%xmm8, 0x130(%rbp)
14127a7eb: 0f 29 bd 20 01 00 00        	movaps	%xmm7, 0x120(%rbp)
14127a7f2: 0f 29 b5 10 01 00 00        	movaps	%xmm6, 0x110(%rbp)
14127a7f9: 48 c7 85 08 01 00 00 fe ff ff ff    	movq	$-0x2, 0x108(%rbp)
14127a804: 45 89 cf                    	movl	%r9d, %r15d
14127a807: 4c 89 c3                    	movq	%r8, %rbx
14127a80a: 48 89 d6                    	movq	%rdx, %rsi
14127a80d: 49 89 ce                    	movq	%rcx, %r14
14127a810: 48 8b bd 38 02 00 00        	movq	0x238(%rbp), %rdi
14127a817: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
14127a81e: e8 1d 1a 7a ff              	callq	0x140a1c240 <.text+0xa1b240>
14127a823: 48 89 f9                    	movq	%rdi, %rcx
14127a826: 48 89 c2                    	movq	%rax, %rdx
14127a829: e8 82 7a 81 ff              	callq	0x140a922b0 <.text+0xa912b0>
14127a82e: 48 89 c1                    	movq	%rax, %rcx
14127a831: 48 c1 e9 20                 	shrq	$0x20, %rcx
14127a835: 0f 84 cf 09 00 00           	je	0x14127b20a <.text+0x127a20a>
14127a83b: 4c 8b 8d 40 02 00 00        	movq	0x240(%rbp), %r9
14127a842: 4d 8b 51 08                 	movq	0x8(%r9), %r10
14127a846: 89 c2                       	movl	%eax, %edx
14127a848: 49 3b 52 10                 	cmpq	0x10(%r10), %rdx
14127a84c: 0f 83 b8 09 00 00           	jae	0x14127b20a <.text+0x127a20a>
14127a852: 49 8b 42 08                 	movq	0x8(%r10), %rax
14127a856: 4c 8d 04 92                 	leaq	(%rdx,%rdx,4), %r8
14127a85a: 42 39 0c 80                 	cmpl	%ecx, (%rax,%r8,4)
14127a85e: 0f 85 a6 09 00 00           	jne	0x14127b20a <.text+0x127a20a>
14127a864: 4a 8d 04 80                 	leaq	(%rax,%r8,4), %rax
14127a868: 8b 78 04                    	movl	0x4(%rax), %edi
14127a86b: 41 bb ff ff ff ff           	movl	$0xffffffff, %r11d      # imm = 0xFFFFFFFF
14127a871: 4c 39 df                    	cmpq	%r11, %rdi
14127a874: 0f 84 90 09 00 00           	je	0x14127b20a <.text+0x127a20a>
14127a87a: 4d 8b 19                    	movq	(%r9), %r11
14127a87d: 49 39 7b 38                 	cmpq	%rdi, 0x38(%r11)
14127a881: 0f 86 83 09 00 00           	jbe	0x14127b20a <.text+0x127a20a>
14127a887: 4d 8b 4b 28                 	movq	0x28(%r11), %r9
14127a88b: 41 89 fc                    	movl	%edi, %r12d
14127a88e: 41 c1 ec 06                 	shrl	$0x6, %r12d
14127a892: 4f 8b 0c e1                 	movq	(%r9,%r12,8), %r9
14127a896: 49 0f a3 f9                 	btq	%rdi, %r9
14127a89a: 0f 83 6a 09 00 00           	jae	0x14127b20a <.text+0x127a20a>
14127a8a0: 4c 8b 8d 48 02 00 00        	movq	0x248(%rbp), %r9
14127a8a7: 44 8b 60 0c                 	movl	0xc(%rax), %r12d
14127a8ab: 49 8b ba a8 01 00 00        	movq	0x1a8(%r10), %rdi
14127a8b2: 4f 8d 24 e4                 	leaq	(%r12,%r12,8), %r12
14127a8b6: 4d 8b 93 10 01 00 00        	movq	0x110(%r11), %r10
14127a8bd: 4e 39 54 e7 40              	cmpq	%r10, 0x40(%rdi,%r12,8)
14127a8c2: 76 2e                       	jbe	0x14127a8f2 <.text+0x12798f2>
14127a8c4: 4e 8d 1c e7                 	leaq	(%rdi,%r12,8), %r11
14127a8c8: 49 8b 7b 38                 	movq	0x38(%r11), %rdi
14127a8cc: 4e 8b 14 d7                 	movq	(%rdi,%r10,8), %r10
14127a8d0: 4d 85 d2                    	testq	%r10, %r10
14127a8d3: 74 1d                       	je	0x14127a8f2 <.text+0x12798f2>
14127a8d5: 4d 8b 5b 18                 	movq	0x18(%r11), %r11
14127a8d9: 49 f7 d2                    	notq	%r10
14127a8dc: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
14127a8e0: 49 c1 e2 04                 	shlq	$0x4, %r10
14127a8e4: 4f 8b 54 13 10              	movq	0x10(%r11,%r10), %r10
14127a8e9: 4c 89 95 e8 00 00 00        	movq	%r10, 0xe8(%rbp)
14127a8f0: eb 0b                       	jmp	0x14127a8fd <.text+0x12798fd>
14127a8f2: 48 c7 85 e8 00 00 00 00 00 00 00    	movq	$0x0, 0xe8(%rbp)
14127a8fd: 4d 8b 51 08                 	movq	0x8(%r9), %r10
14127a901: 49 3b 52 10                 	cmpq	0x10(%r10), %rdx
14127a905: 0f 83 ff 08 00 00           	jae	0x14127b20a <.text+0x127a20a>
14127a90b: 49 8b 52 08                 	movq	0x8(%r10), %rdx
14127a90f: 42 39 0c 82                 	cmpl	%ecx, (%rdx,%r8,4)
14127a913: 0f 85 f1 08 00 00           	jne	0x14127b20a <.text+0x127a20a>
14127a919: 4a 8d 14 82                 	leaq	(%rdx,%r8,4), %rdx
14127a91d: 8b 4a 04                    	movl	0x4(%rdx), %ecx
14127a920: 41 b8 ff ff ff ff           	movl	$0xffffffff, %r8d       # imm = 0xFFFFFFFF
14127a926: 4c 39 c1                    	cmpq	%r8, %rcx
14127a929: 0f 84 db 08 00 00           	je	0x14127b20a <.text+0x127a20a>
14127a92f: 4d 8b 01                    	movq	(%r9), %r8
14127a932: 49 39 48 38                 	cmpq	%rcx, 0x38(%r8)
14127a936: 0f 86 ce 08 00 00           	jbe	0x14127b20a <.text+0x127a20a>
14127a93c: 4d 8b 48 28                 	movq	0x28(%r8), %r9
14127a940: 41 89 cb                    	movl	%ecx, %r11d
14127a943: 41 c1 eb 06                 	shrl	$0x6, %r11d
14127a947: 4f 8b 0c d9                 	movq	(%r9,%r11,8), %r9
14127a94b: 49 0f a3 c9                 	btq	%rcx, %r9
14127a94f: 0f 83 b5 08 00 00           	jae	0x14127b20a <.text+0x127a20a>
14127a955: 8b 40 10                    	movl	0x10(%rax), %eax
14127a958: 48 6b c8 58                 	imulq	$0x58, %rax, %rcx
14127a95c: 8b 42 0c                    	movl	0xc(%rdx), %eax
14127a95f: 8b 52 10                    	movl	0x10(%rdx), %edx
14127a962: 4d 8b 8a a8 01 00 00        	movq	0x1a8(%r10), %r9
14127a969: 4c 8d 14 c0                 	leaq	(%rax,%rax,8), %r10
14127a96d: 49 8b 80 10 01 00 00        	movq	0x110(%r8), %rax
14127a974: 4b 39 44 d1 40              	cmpq	%rax, 0x40(%r9,%r10,8)
14127a979: 76 27                       	jbe	0x14127a9a2 <.text+0x12799a2>
14127a97b: 4f 8d 04 d1                 	leaq	(%r9,%r10,8), %r8
14127a97f: 4d 8b 48 38                 	movq	0x38(%r8), %r9
14127a983: 49 8b 04 c1                 	movq	(%r9,%rax,8), %rax
14127a987: 48 85 c0                    	testq	%rax, %rax
14127a98a: 74 16                       	je	0x14127a9a2 <.text+0x12799a2>
14127a98c: 4d 8b 40 18                 	movq	0x18(%r8), %r8
14127a990: 48 f7 d0                    	notq	%rax
14127a993: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14127a997: 48 c1 e0 04                 	shlq	$0x4, %rax
14127a99b: 49 8b 44 00 10              	movq	0x10(%r8,%rax), %rax
14127a9a0: eb 02                       	jmp	0x14127a9a4 <.text+0x12799a4>
14127a9a2: 31 c0                       	xorl	%eax, %eax
14127a9a4: 48 01 8d e8 00 00 00        	addq	%rcx, 0xe8(%rbp)
14127a9ab: 48 8d 0c d2                 	leaq	(%rdx,%rdx,8), %rcx
14127a9af: 44 89 bd f4 00 00 00        	movl	%r15d, 0xf4(%rbp)
14127a9b6: 44 0f 28 0b                 	movaps	(%rbx), %xmm9
14127a9ba: 45 0f 28 d1                 	movaps	%xmm9, %xmm10
14127a9be: 45 0f 59 d1                 	mulps	%xmm9, %xmm10
14127a9c2: f3 41 0f 16 c2              	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
14127a9c7: f3 41 0f 58 c2              	addss	%xmm10, %xmm0
14127a9cc: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
14127a9d0: 66 41 0f 15 ca              	unpckhpd	%xmm10, %xmm1           # xmm1 = xmm1[1],xmm10[1]
14127a9d5: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14127a9d9: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
14127a9dd: 45 0f c6 d2 ff              	shufps	$0xff, %xmm10, %xmm10   # xmm10 = xmm10[3,3,3,3]
14127a9e2: 44 0f 5c d1                 	subps	%xmm1, %xmm10
14127a9e6: 0f 28 0d d3 26 75 01        	movaps	0x17526d3(%rip), %xmm1  # 0x1429cd0c0
14127a9ed: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
14127a9f1: 0f 59 c1                    	mulps	%xmm1, %xmm0
14127a9f4: 41 0f 59 c9                 	mulps	%xmm9, %xmm1
14127a9f8: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14127a9fc: f3 0f 58 d1                 	addss	%xmm1, %xmm2
14127aa00: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
14127aa03: f3 0f 58 ca                 	addss	%xmm2, %xmm1
14127aa07: f3 0f 58 c9                 	addss	%xmm1, %xmm1
14127aa0b: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
14127aa0f: 41 0f 59 c9                 	mulps	%xmm9, %xmm1
14127aa13: 0f 58 c8                    	addps	%xmm0, %xmm1
14127aa16: f3 0f 10 15 e6 ae 6a 01     	movss	0x16aaee6(%rip), %xmm2  # 0x142925904
14127aa1e: 41 0f 28 d9                 	movaps	%xmm9, %xmm3
14127aa22: 0f 59 da                    	mulps	%xmm2, %xmm3
14127aa25: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
14127aa29: 41 0f c6 c1 c9              	shufps	$0xc9, %xmm9, %xmm0     # xmm0 = xmm0[1,2],xmm9[0,3]
14127aa2e: 0f 59 c2                    	mulps	%xmm2, %xmm0
14127aa31: 0f c6 db d2                 	shufps	$0xd2, %xmm3, %xmm3     # xmm3 = xmm3[2,0,1,3]
14127aa35: 0f 5c c3                    	subps	%xmm3, %xmm0
14127aa38: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
14127aa3c: 41 0f 58 d1                 	addps	%xmm9, %xmm2
14127aa40: 0f c6 d2 ff                 	shufps	$0xff, %xmm2, %xmm2     # xmm2 = xmm2[3,3,3,3]
14127aa44: 0f 29 55 b0                 	movaps	%xmm2, -0x50(%rbp)
14127aa48: 0f 59 c2                    	mulps	%xmm2, %xmm0
14127aa4b: 0f 58 c1                    	addps	%xmm1, %xmm0
14127aa4e: f3 41 0f 10 4e 18           	movss	0x18(%r14), %xmm1
14127aa54: f2 0f 10 73 10              	movsd	0x10(%rbx), %xmm6
14127aa59: f3 0f 10 7b 18              	movss	0x18(%rbx), %xmm7
14127aa5e: f2 41 0f 10 6e 1c           	movsd	0x1c(%r14), %xmm5
14127aa64: 44 0f 28 c5                 	movaps	%xmm5, %xmm8
14127aa68: f3 44 0f 10 c1              	movss	%xmm1, %xmm8            # xmm8 = xmm1[0],xmm8[1,2,3]
14127aa6d: 0f 28 d1                    	movaps	%xmm1, %xmm2
14127aa70: f3 0f 5c d6                 	subss	%xmm6, %xmm2
14127aa74: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
14127aa78: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
14127aa7c: f2 0f 10 5b 14              	movsd	0x14(%rbx), %xmm3
14127aa81: 0f 28 e5                    	movaps	%xmm5, %xmm4
14127aa84: 0f 5c e3                    	subps	%xmm3, %xmm4
14127aa87: 0f 28 d8                    	movaps	%xmm0, %xmm3
14127aa8a: 0f c6 d8 e9                 	shufps	$0xe9, %xmm0, %xmm3     # xmm3 = xmm3[1,2],xmm0[2,3]
14127aa8e: 0f 59 dc                    	mulps	%xmm4, %xmm3
14127aa91: f3 0f 58 d3                 	addss	%xmm3, %xmm2
14127aa95: f3 0f 16 db                 	movshdup	%xmm3, %xmm3            # xmm3 = xmm3[1,1,3,3]
14127aa99: f3 0f 58 da                 	addss	%xmm2, %xmm3
14127aa9d: f3 0f 59 cb                 	mulss	%xmm3, %xmm1
14127aaa1: 0f c6 c0 e8                 	shufps	$0xe8, %xmm0, %xmm0     # xmm0 = xmm0[0,2,2,3]
14127aaa5: f3 0f 12 d3                 	movsldup	%xmm3, %xmm2            # xmm2 = xmm3[0,0,2,2]
14127aaa9: 0f 59 d0                    	mulps	%xmm0, %xmm2
14127aaac: f2 0f 11 75 70              	movsd	%xmm6, 0x70(%rbp)
14127aab1: f3 0f 11 7d 78              	movss	%xmm7, 0x78(%rbp)
14127aab6: 44 0f b6 7e 44              	movzbl	0x44(%rsi), %r15d
14127aabb: 0f 57 db                    	xorps	%xmm3, %xmm3
14127aabe: f3 41 0f 2a df              	cvtsi2ss	%r15d, %xmm3
14127aac3: f3 0f 10 05 39 ae 6a 01     	movss	0x16aae39(%rip), %xmm0  # 0x142925904
14127aacb: f3 0f 5e c3                 	divss	%xmm3, %xmm0
14127aacf: 4c 89 fa                    	movq	%r15, %rdx
14127aad2: 48 ff ca                    	decq	%rdx
14127aad5: 78 07                       	js	0x14127aade <.text+0x1279ade>
14127aad7: f3 44 0f 2a fa              	cvtsi2ss	%edx, %xmm15
14127aadc: eb 16                       	jmp	0x14127aaf4 <.text+0x1279af4>
14127aade: 49 89 d0                    	movq	%rdx, %r8
14127aae1: 49 d1 e8                    	shrq	%r8
14127aae4: 83 e2 01                    	andl	$0x1, %edx
14127aae7: 4c 09 c2                    	orq	%r8, %rdx
14127aaea: f3 4c 0f 2a fa              	cvtsi2ss	%rdx, %xmm15
14127aaef: f3 45 0f 58 ff              	addss	%xmm15, %xmm15
14127aaf4: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
14127aaf8: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14127aafc: f3 0f 5c e9                 	subss	%xmm1, %xmm5
14127ab00: 0f 29 6d d0                 	movaps	%xmm5, -0x30(%rbp)
14127ab04: 44 0f 5c c2                 	subps	%xmm2, %xmm8
14127ab08: 44 0f 29 45 e0              	movaps	%xmm8, -0x20(%rbp)
14127ab0d: f3 44 0f 10 35 ee ad 6a 01  	movss	0x16aadee(%rip), %xmm14 # 0x142925904
14127ab16: f3 44 0f 5c f0              	subss	%xmm0, %xmm14
14127ab1b: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
14127ab22: 0f b6 48 08                 	movzbl	0x8(%rax), %ecx
14127ab26: 88 8d 07 01 00 00           	movb	%cl, 0x107(%rbp)
14127ab2c: f3 44 0f 10 68 4c           	movss	0x4c(%rax), %xmm13
14127ab32: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
14127ab36: 41 0f c6 c1 d2              	shufps	$0xd2, %xmm9, %xmm0     # xmm0 = xmm0[2,0],xmm9[1,3]
14127ab3b: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
14127ab3f: 41 8d 47 ff                 	leal	-0x1(%r15), %eax
14127ab43: 89 45 64                    	movl	%eax, 0x64(%rbp)
14127ab46: 0f b6 c0                    	movzbl	%al, %eax
14127ab49: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14127ab4d: f3 0f 10 05 17 1c 6b 01     	movss	0x16b1c17(%rip), %xmm0  # 0x14292c76c
14127ab55: f3 0f 11 85 bc 00 00 00     	movss	%xmm0, 0xbc(%rbp)
14127ab5d: 31 ff                       	xorl	%edi, %edi
14127ab5f: 4c 8d ad 88 00 00 00        	leaq	0x88(%rbp), %r13
14127ab66: 4c 8d b5 c0 00 00 00        	leaq	0xc0(%rbp), %r14
14127ab6d: 45 0f 57 db                 	xorps	%xmm11, %xmm11
14127ab71: 31 db                       	xorl	%ebx, %ebx
14127ab73: 48 89 75 50                 	movq	%rsi, 0x50(%rbp)
14127ab77: 4c 39 ff                    	cmpq	%r15, %rdi
14127ab7a: 0f 83 17 05 00 00           	jae	0x14127b097 <.text+0x127a097>
14127ab80: 48 89 7d 68                 	movq	%rdi, 0x68(%rbp)
14127ab84: 80 7d 64 03                 	cmpb	$0x3, 0x64(%rbp)
14127ab88: 0f 83 e3 06 00 00           	jae	0x14127b271 <.text+0x127a271>
14127ab8e: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14127ab92: 48 8d 0d 07 94 9a 01        	leaq	0x19a9407(%rip), %rcx   # 0x142c23fa0
14127ab99: f3 0f 10 04 81              	movss	(%rcx,%rax,4), %xmm0
14127ab9e: f3 41 0f 59 c6              	mulss	%xmm14, %xmm0
14127aba3: 45 0f 57 e4                 	xorps	%xmm12, %xmm12
14127aba7: f3 4c 0f 2a e7              	cvtsi2ss	%rdi, %xmm12
14127abac: f3 45 0f 5e e7              	divss	%xmm15, %xmm12
14127abb1: f3 44 0f 58 25 2e f5 6d 01  	addss	0x16df52e(%rip), %xmm12 # 0x14295a0e8
14127abba: f3 44 0f 59 e0              	mulss	%xmm0, %xmm12
14127abbf: f6 85 07 01 00 00 01        	testb	$0x1, 0x107(%rbp)
14127abc6: 74 6b                       	je	0x14127ac33 <.text+0x1279c33>
14127abc8: 0f 13 b5 c0 00 00 00        	movlps	%xmm6, 0xc0(%rbp)
14127abcf: f3 0f 11 bd c8 00 00 00     	movss	%xmm7, 0xc8(%rbp)
14127abd7: 4c 89 e9                    	movq	%r13, %rcx
14127abda: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
14127abe1: 4d 89 f0                    	movq	%r14, %r8
14127abe4: e8 67 9b 66 ff              	callq	0x1408e4750 <.text+0x8e3750>
14127abe9: 4c 89 e9                    	movq	%r13, %rcx
14127abec: e8 4f 4b f8 ff              	callq	0x1411ff740 <.text+0x11fe740>
14127abf1: 0f 10 00                    	movups	(%rax), %xmm0
14127abf4: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14127abfb: 48 8b 40 10                 	movq	0x10(%rax), %rax
14127abff: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
14127ac06: 4c 89 f1                    	movq	%r14, %rcx
14127ac09: 0f 28 ce                    	movaps	%xmm6, %xmm1
14127ac0c: 0f 28 d7                    	movaps	%xmm7, %xmm2
14127ac0f: e8 9c 00 a2 ff              	callq	0x140c9acb0 <.text+0xc99cb0>
14127ac14: 4c 89 f1                    	movq	%r14, %rcx
14127ac17: 48 89 c2                    	movq	%rax, %rdx
14127ac1a: e8 31 1e a2 ff              	callq	0x140c9ca50 <.text+0xc9ba50>
14127ac1f: 45 0f 28 c4                 	movaps	%xmm12, %xmm8
14127ac23: f3 44 0f 5e c0              	divss	%xmm0, %xmm8
14127ac28: f3 44 0f 59 05 1f ad 6a 01  	mulss	0x16aad1f(%rip), %xmm8  # 0x142925950
14127ac31: eb 32                       	jmp	0x14127ac65 <.text+0x1279c65>
14127ac33: 0f 13 b5 c0 00 00 00        	movlps	%xmm6, 0xc0(%rbp)
14127ac3a: f3 0f 11 bd c8 00 00 00     	movss	%xmm7, 0xc8(%rbp)
14127ac42: 4c 89 e9                    	movq	%r13, %rcx
14127ac45: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
14127ac4c: 4d 89 f0                    	movq	%r14, %r8
14127ac4f: e8 fc 9a 66 ff              	callq	0x1408e4750 <.text+0x8e3750>
14127ac54: 4c 89 e9                    	movq	%r13, %rcx
14127ac57: e8 e4 49 f8 ff              	callq	0x1411ff640 <.text+0x11fe640>
14127ac5c: 45 0f 28 c4                 	movaps	%xmm12, %xmm8
14127ac60: f3 44 0f 5e c0              	divss	%xmm0, %xmm8
14127ac65: 48 89 f1                    	movq	%rsi, %rcx
14127ac68: 48 8d 15 71 91 9a 01        	leaq	0x19a9171(%rip), %rdx   # 0x142c23de0
14127ac6f: e8 3c 87 5b ff              	callq	0x1408333b0 <.text+0x8323b0>
14127ac74: f3 0f 10 58 04              	movss	0x4(%rax), %xmm3
14127ac79: f3 44 0f 58 00              	addss	(%rax), %xmm8
14127ac7e: 44 0f 2e 05 7e ac 6a 01     	ucomiss	0x16aac7e(%rip), %xmm8  # 0x142925904
14127ac86: 76 0b                       	jbe	0x14127ac93 <.text+0x1279c93>
14127ac88: f3 44 0f 58 05 53 f4 6d 01  	addss	0x16df453(%rip), %xmm8  # 0x14295a0e4
14127ac91: eb 0f                       	jmp	0x14127aca2 <.text+0x1279ca2>
14127ac93: 45 0f 2e d8                 	ucomiss	%xmm8, %xmm11
14127ac97: 76 09                       	jbe	0x14127aca2 <.text+0x1279ca2>
14127ac99: f3 44 0f 58 05 62 ac 6a 01  	addss	0x16aac62(%rip), %xmm8  # 0x142925904
14127aca2: f3 44 0f 11 6c 24 20        	movss	%xmm13, 0x20(%rsp)
14127aca9: 4c 89 e9                    	movq	%r13, %rcx
14127acac: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
14127acb0: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14127acb4: e8 17 9a 66 ff              	callq	0x1408e46d0 <.text+0x8e36d0>
14127acb9: f6 85 88 00 00 00 01        	testb	$0x1, 0x88(%rbp)
14127acc0: 74 14                       	je	0x14127acd6 <.text+0x1279cd6>
14127acc2: f2 44 0f 10 a5 8c 00 00 00  	movsd	0x8c(%rbp), %xmm12
14127accb: f3 44 0f 10 85 94 00 00 00  	movss	0x94(%rbp), %xmm8
14127acd4: eb 77                       	jmp	0x14127ad4d <.text+0x1279d4d>
14127acd6: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
14127acda: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
14127acdf: 44 0f 14 e0                 	unpcklps	%xmm0, %xmm12           # xmm12 = xmm12[0],xmm0[0],xmm12[1],xmm0[1]
14127ace3: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
14127ace7: 0f 16 c8                    	movlhps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0]
14127acea: 44 0f c6 e0 04              	shufps	$0x4, %xmm0, %xmm12     # xmm12 = xmm12[0,1],xmm0[0,0]
14127acef: 45 0f 59 e2                 	mulps	%xmm10, %xmm12
14127acf3: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
14127acf7: 0f 59 c1                    	mulps	%xmm1, %xmm0
14127acfa: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
14127acfe: f3 0f 58 d0                 	addss	%xmm0, %xmm2
14127ad02: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
14127ad05: f3 0f 58 c2                 	addss	%xmm2, %xmm0
14127ad09: f3 0f 58 c0                 	addss	%xmm0, %xmm0
14127ad0d: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14127ad11: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
14127ad15: 41 0f 58 c4                 	addps	%xmm12, %xmm0
14127ad19: 44 0f 28 45 c0              	movaps	-0x40(%rbp), %xmm8
14127ad1e: 44 0f 59 c1                 	mulps	%xmm1, %xmm8
14127ad22: 0f c6 c9 d2                 	shufps	$0xd2, %xmm1, %xmm1     # xmm1 = xmm1[2,0,1,3]
14127ad26: 41 0f 59 c9                 	mulps	%xmm9, %xmm1
14127ad2a: 44 0f 5c c1                 	subps	%xmm1, %xmm8
14127ad2e: 45 0f c6 c0 d2              	shufps	$0xd2, %xmm8, %xmm8     # xmm8 = xmm8[2,0,1,3]
14127ad33: 44 0f 59 45 b0              	mulps	-0x50(%rbp), %xmm8
14127ad38: 44 0f 58 c0                 	addps	%xmm0, %xmm8
14127ad3c: 44 0f 28 e6                 	movaps	%xmm6, %xmm12
14127ad40: 45 0f 58 e0                 	addps	%xmm8, %xmm12
14127ad44: 45 0f 12 c0                 	movhlps	%xmm8, %xmm8            # xmm8 = xmm8[1,1]
14127ad48: f3 44 0f 58 c7              	addss	%xmm7, %xmm8
14127ad4d: 48 ff c7                    	incq	%rdi
14127ad50: 48 8b 45 68                 	movq	0x68(%rbp), %rax
14127ad54: 41 80 ff 03                 	cmpb	$0x3, %r15b
14127ad58: 0f 85 7c 01 00 00           	jne	0x14127aeda <.text+0x1279eda>
14127ad5e: 48 83 f8 01                 	cmpq	$0x1, %rax
14127ad62: 0f 85 72 01 00 00           	jne	0x14127aeda <.text+0x1279eda>
14127ad68: 48 89 bd 80 00 00 00        	movq	%rdi, 0x80(%rbp)
14127ad6f: 4d 89 fc                    	movq	%r15, %r12
14127ad72: 48 8d 45 68                 	leaq	0x68(%rbp), %rax
14127ad76: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14127ad7d: 48 8d 05 3c 29 b1 ff        	leaq	-0x4ed6c4(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
14127ad84: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14127ad8b: 48 8d 85 f4 00 00 00        	leaq	0xf4(%rbp), %rax
14127ad92: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
14127ad99: 48 8d 05 30 84 5b ff        	leaq	-0xa47bd0(%rip), %rax   # 0x1408331d0 <.text+0x8321d0>
14127ada0: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
14127ada7: 48 8d 05 92 8f 9a 01        	leaq	0x19a8f92(%rip), %rax   # 0x142c23d40
14127adae: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14127adb5: 48 c7 85 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rbp)
14127adc0: 4c 89 f7                    	movq	%r14, %rdi
14127adc3: 4c 89 b5 98 00 00 00        	movq	%r14, 0x98(%rbp)
14127adca: 48 c7 85 a0 00 00 00 02 00 00 00    	movq	$0x2, 0xa0(%rbp)
14127add5: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
14127ade0: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14127ade4: 4c 89 ea                    	movq	%r13, %rdx
14127ade7: e8 a4 f1 af ff              	callq	0x140d79f90 <.text+0xd78f90>
14127adec: 48 8b 45 10                 	movq	0x10(%rbp), %rax
14127adf0: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
14127adf7: 4c 8b 7d 18                 	movq	0x18(%rbp), %r15
14127adfb: 4c 8b 75 20                 	movq	0x20(%rbp), %r14
14127adff: 0f b6 05 03 9e 3b 02        	movzbl	0x23b9e03(%rip), %eax   # 0x143634c09
14127ae06: b9 20 00 00 00              	movl	$0x20, %ecx
14127ae0b: ba 08 00 00 00              	movl	$0x8, %edx
14127ae10: e8 fb 8d 39 ff              	callq	0x140613c10 <.text+0x612c10>
14127ae15: 48 85 c0                    	testq	%rax, %rax
14127ae18: 4c 89 7d 58                 	movq	%r15, 0x58(%rbp)
14127ae1c: 0f 84 67 04 00 00           	je	0x14127b289 <.text+0x127a289>
14127ae22: f3 45 0f 58 c3              	addss	%xmm11, %xmm8
14127ae27: 44 0f 58 25 b1 5e 80 01     	addps	0x1805eb1(%rip), %xmm12 # 0x142a80ce0
14127ae2f: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14127ae36: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14127ae3e: 44 0f 13 60 10              	movlps	%xmm12, 0x10(%rax)
14127ae43: f3 44 0f 11 40 18           	movss	%xmm8, 0x18(%rax)
14127ae49: 48 8d 0d a8 8f 9a 01        	leaq	0x19a8fa8(%rip), %rcx   # 0x142c23df8
14127ae50: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14127ae55: 4c 89 ee                    	movq	%r13, %rsi
14127ae58: 4c 89 e9                    	movq	%r13, %rcx
14127ae5b: 4c 89 fa                    	movq	%r15, %rdx
14127ae5e: 4d 89 f0                    	movq	%r14, %r8
14127ae61: 49 89 c1                    	movq	%rax, %r9
14127ae64: e8 47 2a 62 ff              	callq	0x14089d8b0 <.text+0x89c8b0>
14127ae69: 49 89 fe                    	movq	%rdi, %r14
14127ae6c: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
14127ae73: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
14127ae7a: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
14127ae81: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
14127ae88: 0f 29 8d d0 00 00 00        	movaps	%xmm1, 0xd0(%rbp)
14127ae8f: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14127ae96: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
14127ae9d: 48 89 fa                    	movq	%rdi, %rdx
14127aea0: e8 eb 20 8b ff              	callq	0x140b2cf90 <.text+0xb2bf90>
14127aea5: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
14127aeac: 48 85 d2                    	testq	%rdx, %rdx
14127aeaf: 4d 89 e7                    	movq	%r12, %r15
14127aeb2: 49 89 f5                    	movq	%rsi, %r13
14127aeb5: 48 8b 75 50                 	movq	0x50(%rbp), %rsi
14127aeb9: 48 8b bd 80 00 00 00        	movq	0x80(%rbp), %rdi
14127aec0: 0f 84 b1 fc ff ff           	je	0x14127ab77 <.text+0x1279b77>
14127aec6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14127aecc: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
14127aed0: e8 4b 8d 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127aed5: e9 9d fc ff ff              	jmp	0x14127ab77 <.text+0x1279b77>
14127aeda: f3 41 0f 16 c4              	movshdup	%xmm12, %xmm0           # xmm0 = xmm12[1,1,3,3]
14127aedf: f3 0f 5c 45 d0              	subss	-0x30(%rbp), %xmm0
14127aee4: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14127aee8: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14127aeec: 41 0f 16 cc                 	movlhps	%xmm12, %xmm1           # xmm1 = xmm1[0],xmm12[0]
14127aef0: 41 0f c6 cc e2              	shufps	$0xe2, %xmm12, %xmm1    # xmm1 = xmm1[2,0],xmm12[2,3]
14127aef5: 0f 5c 4d e0                 	subps	-0x20(%rbp), %xmm1
14127aef9: 0f 59 c9                    	mulps	%xmm1, %xmm1
14127aefc: f3 0f 58 c1                 	addss	%xmm1, %xmm0
14127af00: f3 0f 16 c9                 	movshdup	%xmm1, %xmm1            # xmm1 = xmm1[1,1,3,3]
14127af04: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14127af08: 0f 57 c0                    	xorps	%xmm0, %xmm0
14127af0b: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
14127af0f: f3 0f 10 8d bc 00 00 00     	movss	0xbc(%rbp), %xmm1
14127af17: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
14127af1a: f3 45 0f 58 c3              	addss	%xmm11, %xmm8
14127af1f: 76 20                       	jbe	0x14127af41 <.text+0x1279f41>
14127af21: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
14127af25: 0f 58 0d b4 5d 80 01        	addps	0x1805db4(%rip), %xmm1  # 0x142a80ce0
14127af2c: 0f 13 4d 70                 	movlps	%xmm1, 0x70(%rbp)
14127af30: f3 44 0f 11 45 78           	movss	%xmm8, 0x78(%rbp)
14127af36: f3 0f 11 85 bc 00 00 00     	movss	%xmm0, 0xbc(%rbp)
14127af3e: 48 89 c3                    	movq	%rax, %rbx
14127af41: 48 8d 45 68                 	leaq	0x68(%rbp), %rax
14127af45: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14127af4c: 48 8d 05 6d 27 b1 ff        	leaq	-0x4ed893(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
14127af53: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14127af5a: 48 8d 85 f4 00 00 00        	leaq	0xf4(%rbp), %rax
14127af61: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
14127af68: 48 8d 05 61 82 5b ff        	leaq	-0xa47d9f(%rip), %rax   # 0x1408331d0 <.text+0x8321d0>
14127af6f: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
14127af76: 48 8d 05 c3 8d 9a 01        	leaq	0x19a8dc3(%rip), %rax   # 0x142c23d40
14127af7d: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14127af84: 48 c7 85 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rbp)
14127af8f: 4c 89 b5 98 00 00 00        	movq	%r14, 0x98(%rbp)
14127af96: 48 c7 85 a0 00 00 00 02 00 00 00    	movq	$0x2, 0xa0(%rbp)
14127afa1: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
14127afac: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
14127afb0: 4c 89 ea                    	movq	%r13, %rdx
14127afb3: e8 d8 ef af ff              	callq	0x140d79f90 <.text+0xd78f90>
14127afb8: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14127afbc: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
14127afc3: 4c 8b 65 00                 	movq	(%rbp), %r12
14127afc7: 4c 8b 75 08                 	movq	0x8(%rbp), %r14
14127afcb: 0f b6 05 37 9c 3b 02        	movzbl	0x23b9c37(%rip), %eax   # 0x143634c09
14127afd2: b9 20 00 00 00              	movl	$0x20, %ecx
14127afd7: ba 08 00 00 00              	movl	$0x8, %edx
14127afdc: e8 2f 8c 39 ff              	callq	0x140613c10 <.text+0x612c10>
14127afe1: 48 85 c0                    	testq	%rax, %rax
14127afe4: 4c 89 a5 80 00 00 00        	movq	%r12, 0x80(%rbp)
14127afeb: 0f 84 ba 02 00 00           	je	0x14127b2ab <.text+0x127a2ab>
14127aff1: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14127aff8: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14127b000: 44 0f 58 25 d8 5c 80 01     	addps	0x1805cd8(%rip), %xmm12 # 0x142a80ce0
14127b008: 44 0f 13 60 10              	movlps	%xmm12, 0x10(%rax)
14127b00d: f3 44 0f 11 40 18           	movss	%xmm8, 0x18(%rax)
14127b013: 48 8d 0d 0e 8e 9a 01        	leaq	0x19a8e0e(%rip), %rcx   # 0x142c23e28
14127b01a: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14127b01f: 4c 89 e9                    	movq	%r13, %rcx
14127b022: 4c 89 e2                    	movq	%r12, %rdx
14127b025: 4d 89 f0                    	movq	%r14, %r8
14127b028: 49 89 c1                    	movq	%rax, %r9
14127b02b: e8 80 28 62 ff              	callq	0x14089d8b0 <.text+0x89c8b0>
14127b030: 4c 8d b5 c0 00 00 00        	leaq	0xc0(%rbp), %r14
14127b037: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
14127b03e: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
14127b045: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
14127b04c: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
14127b053: 0f 29 8d d0 00 00 00        	movaps	%xmm1, 0xd0(%rbp)
14127b05a: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14127b061: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
14127b068: 4c 89 f2                    	movq	%r14, %rdx
14127b06b: e8 20 1f 8b ff              	callq	0x140b2cf90 <.text+0xb2bf90>
14127b070: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
14127b077: 48 85 d2                    	testq	%rdx, %rdx
14127b07a: 0f 84 f7 fa ff ff           	je	0x14127ab77 <.text+0x1279b77>
14127b080: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14127b086: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
14127b08d: e8 8e 8b 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127b092: e9 e0 fa ff ff              	jmp	0x14127ab77 <.text+0x1279b77>
14127b097: 48 8d 45 70                 	leaq	0x70(%rbp), %rax
14127b09b: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14127b0a2: 48 8d 05 27 07 ad ff        	leaq	-0x52f8d9(%rip), %rax   # 0x140d4b7d0 <.text+0xd4a7d0>
14127b0a9: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14127b0b0: 48 8d 85 f4 00 00 00        	leaq	0xf4(%rbp), %rax
14127b0b7: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
14127b0be: 48 8d 05 0b 81 5b ff        	leaq	-0xa47ef5(%rip), %rax   # 0x1408331d0 <.text+0x8321d0>
14127b0c5: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
14127b0cc: 48 8d 05 6d 8c 9a 01        	leaq	0x19a8c6d(%rip), %rax   # 0x142c23d40
14127b0d3: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14127b0da: 48 c7 85 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rbp)
14127b0e5: 4c 89 b5 98 00 00 00        	movq	%r14, 0x98(%rbp)
14127b0ec: 48 c7 85 a0 00 00 00 02 00 00 00    	movq	$0x2, 0xa0(%rbp)
14127b0f7: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
14127b102: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
14127b106: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
14127b10d: e8 7e ee af ff              	callq	0x140d79f90 <.text+0xd78f90>
14127b112: 48 8b 7d 30                 	movq	0x30(%rbp), %rdi
14127b116: 48 8b 75 38                 	movq	0x38(%rbp), %rsi
14127b11a: 8b 45 78                    	movl	0x78(%rbp), %eax
14127b11d: 89 85 90 00 00 00           	movl	%eax, 0x90(%rbp)
14127b123: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14127b127: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14127b12e: 0f b6 05 d4 9a 3b 02        	movzbl	0x23b9ad4(%rip), %eax   # 0x143634c09
14127b135: b9 20 00 00 00              	movl	$0x20, %ecx
14127b13a: ba 08 00 00 00              	movl	$0x8, %edx
14127b13f: e8 cc 8a 39 ff              	callq	0x140613c10 <.text+0x612c10>
14127b144: 48 85 c0                    	testq	%rax, %rax
14127b147: 48 89 bd f8 00 00 00        	movq	%rdi, 0xf8(%rbp)
14127b14e: 0f 84 46 01 00 00           	je	0x14127b29a <.text+0x127a29a>
14127b154: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14127b15b: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14127b163: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14127b16a: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
14127b16e: 8b 8d 90 00 00 00           	movl	0x90(%rbp), %ecx
14127b174: 89 48 18                    	movl	%ecx, 0x18(%rax)
14127b177: 48 8d 0d e2 8b 9a 01        	leaq	0x19a8be2(%rip), %rcx   # 0x142c23d60
14127b17e: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14127b183: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
14127b18a: 48 89 fa                    	movq	%rdi, %rdx
14127b18d: 49 89 f0                    	movq	%rsi, %r8
14127b190: 49 89 c1                    	movq	%rax, %r9
14127b193: e8 18 27 62 ff              	callq	0x14089d8b0 <.text+0x89c8b0>
14127b198: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
14127b19f: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
14127b1a6: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
14127b1ad: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
14127b1b4: 0f 29 8d d0 00 00 00        	movaps	%xmm1, 0xd0(%rbp)
14127b1bb: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14127b1c2: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
14127b1c9: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
14127b1d0: e8 bb 1d 8b ff              	callq	0x140b2cf90 <.text+0xb2bf90>
14127b1d5: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
14127b1d9: 48 85 d2                    	testq	%rdx, %rdx
14127b1dc: 74 12                       	je	0x14127b1f0 <.text+0x127a1f0>
14127b1de: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14127b1e4: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
14127b1eb: e8 30 8a 39 ff              	callq	0x140613c20 <.text+0x612c20>
14127b1f0: 0f b6 85 20 02 00 00        	movzbl	0x220(%rbp), %eax
14127b1f7: 3c 08                       	cmpb	$0x8, %al
14127b1f9: 74 08                       	je	0x14127b203 <.text+0x127a203>
14127b1fb: 0f b6 c0                    	movzbl	%al, %eax
14127b1fe: 83 f8 02                    	cmpl	$0x2, %eax
14127b201: 75 07                       	jne	0x14127b20a <.text+0x127a20a>
14127b203: b8 01 00 00 00              	movl	$0x1, %eax
14127b208: eb 02                       	jmp	0x14127b20c <.text+0x127a20c>
14127b20a: 31 c0                       	xorl	%eax, %eax
14127b20c: 48 89 da                    	movq	%rbx, %rdx
14127b20f: 0f 28 b5 10 01 00 00        	movaps	0x110(%rbp), %xmm6
14127b216: 0f 28 bd 20 01 00 00        	movaps	0x120(%rbp), %xmm7
14127b21d: 44 0f 28 85 30 01 00 00     	movaps	0x130(%rbp), %xmm8
14127b225: 44 0f 28 8d 40 01 00 00     	movaps	0x140(%rbp), %xmm9
14127b22d: 44 0f 28 95 50 01 00 00     	movaps	0x150(%rbp), %xmm10
14127b235: 44 0f 28 9d 60 01 00 00     	movaps	0x160(%rbp), %xmm11
14127b23d: 44 0f 28 a5 70 01 00 00     	movaps	0x170(%rbp), %xmm12
14127b245: 44 0f 28 ad 80 01 00 00     	movaps	0x180(%rbp), %xmm13
14127b24d: 44 0f 28 b5 90 01 00 00     	movaps	0x190(%rbp), %xmm14
14127b255: 44 0f 28 bd a0 01 00 00     	movaps	0x1a0(%rbp), %xmm15
14127b25d: 48 81 c4 38 02 00 00        	addq	$0x238, %rsp            # imm = 0x238
14127b264: 5b                          	popq	%rbx
14127b265: 5f                          	popq	%rdi
14127b266: 5e                          	popq	%rsi
14127b267: 41 5c                       	popq	%r12
14127b269: 41 5d                       	popq	%r13
14127b26b: 41 5e                       	popq	%r14
14127b26d: 41 5f                       	popq	%r15
14127b26f: 5d                          	popq	%rbp
14127b270: c3                          	retq
14127b271: 48 8d 0d 00 be 80 01        	leaq	0x180be00(%rip), %rcx   # 0x142a87078
14127b278: 4c 8d 05 71 be 80 01        	leaq	0x180be71(%rip), %r8    # 0x142a870f0
14127b27f: ba 28 00 00 00              	movl	$0x28, %edx
14127b284: e8 a7 e1 65 01              	callq	0x1428d9430 <.text+0x28d8430>
14127b289: b9 08 00 00 00              	movl	$0x8, %ecx
14127b28e: ba 20 00 00 00              	movl	$0x20, %edx
14127b293: e8 68 dd 65 01              	callq	0x1428d9000 <.text+0x28d8000>
14127b298: eb 20                       	jmp	0x14127b2ba <.text+0x127a2ba>
14127b29a: b9 08 00 00 00              	movl	$0x8, %ecx
14127b29f: ba 20 00 00 00              	movl	$0x20, %edx
14127b2a4: e8 57 dd 65 01              	callq	0x1428d9000 <.text+0x28d8000>
14127b2a9: eb 0f                       	jmp	0x14127b2ba <.text+0x127a2ba>
14127b2ab: b9 08 00 00 00              	movl	$0x8, %ecx
14127b2b0: ba 20 00 00 00              	movl	$0x20, %edx
14127b2b5: e8 46 dd 65 01              	callq	0x1428d9000 <.text+0x28d8000>
14127b2ba: 0f 0b                       	ud2
14127b2bc: 0f 1f 40 00                 	nopl	(%rax)
