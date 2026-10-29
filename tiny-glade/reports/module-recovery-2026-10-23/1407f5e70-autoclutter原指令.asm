
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407f5e70: 41 57                       	pushq	%r15
1407f5e72: 41 56                       	pushq	%r14
1407f5e74: 41 54                       	pushq	%r12
1407f5e76: 56                          	pushq	%rsi
1407f5e77: 57                          	pushq	%rdi
1407f5e78: 53                          	pushq	%rbx
1407f5e79: 48 83 ec 78                 	subq	$0x78, %rsp
1407f5e7d: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
1407f5e83: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
1407f5e88: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
1407f5e8d: 80 79 45 04                 	cmpb	$0x4, 0x45(%rcx)
1407f5e91: 0f 82 d0 02 00 00           	jb	0x1407f6167 <.text+0x7f5167>
1407f5e97: 44 89 cf                    	movl	%r9d, %edi
1407f5e9a: 4c 89 c6                    	movq	%r8, %rsi
1407f5e9d: 48 89 d3                    	movq	%rdx, %rbx
1407f5ea0: 4c 8b b4 24 d0 00 00 00     	movq	0xd0(%rsp), %r14
1407f5ea8: 8b 49 2c                    	movl	0x2c(%rcx), %ecx
1407f5eab: e8 90 03 4a 00              	callq	0x140c96240 <.text+0xc95240>
1407f5eb0: 48 b9 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rcx # imm = 0xA0761D6478BD642F
1407f5eba: 48 01 c1                    	addq	%rax, %rcx
1407f5ebd: 48 b8 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rax # imm = 0xE7037ED1A0B428DB
1407f5ec7: 48 31 c8                    	xorq	%rcx, %rax
1407f5eca: 48 f7 e1                    	mulq	%rcx
1407f5ecd: 48 0f ac d0 1e              	shrdq	$0x1e, %rdx, %rax
1407f5ed2: 48 c1 ea 1e                 	shrq	$0x1e, %rdx
1407f5ed6: 48 31 d0                    	xorq	%rdx, %rax
1407f5ed9: a8 03                       	testb	$0x3, %al
1407f5edb: 0f 84 58 01 00 00           	je	0x1407f6039 <.text+0x7f5039>
1407f5ee1: 83 e0 03                    	andl	$0x3, %eax
1407f5ee4: 83 f8 01                    	cmpl	$0x1, %eax
1407f5ee7: 0f 85 7a 02 00 00           	jne	0x1407f6167 <.text+0x7f5167>
1407f5eed: 4c 8b a4 24 d8 00 00 00     	movq	0xd8(%rsp), %r12
1407f5ef5: 0f 28 33                    	movaps	(%rbx), %xmm6
1407f5ef8: f2 0f 10 7b 10              	movsd	0x10(%rbx), %xmm7
1407f5efd: 4c 8d 7b 1c                 	leaq	0x1c(%rbx), %r15
1407f5f01: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407f5f04: 0f 59 ce                    	mulps	%xmm6, %xmm1
1407f5f07: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
1407f5f0b: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407f5f0f: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407f5f12: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
1407f5f16: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407f5f1a: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407f5f1e: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1407f5f22: 0f 5c ca                    	subps	%xmm2, %xmm1
1407f5f25: 0f 28 05 e4 56 29 02        	movaps	0x22956e4(%rip), %xmm0  # 0x142a8b610
1407f5f2c: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407f5f2f: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f5f32: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1407f5f36: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407f5f3a: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407f5f3d: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1407f5f41: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407f5f45: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f5f49: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f5f4c: 0f 58 c1                    	addps	%xmm1, %xmm0
1407f5f4f: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407f5f52: 0f c6 ce d2                 	shufps	$0xd2, %xmm6, %xmm1     # xmm1 = xmm1[2,0],xmm6[1,3]
1407f5f56: 0f 59 0d 03 13 29 02        	mulps	0x2291303(%rip), %xmm1  # 0x142a87260
1407f5f5d: 0f 28 d6                    	movaps	%xmm6, %xmm2
1407f5f60: 0f c6 d6 c9                 	shufps	$0xc9, %xmm6, %xmm2     # xmm2 = xmm2[1,2],xmm6[0,3]
1407f5f64: 0f 59 15 b5 56 29 02        	mulps	0x22956b5(%rip), %xmm2  # 0x142a8b620
1407f5f6b: 0f 5c d1                    	subps	%xmm1, %xmm2
1407f5f6e: 44 0f 28 c6                 	movaps	%xmm6, %xmm8
1407f5f72: 44 0f 58 c6                 	addps	%xmm6, %xmm8
1407f5f76: 45 0f c6 c0 ff              	shufps	$0xff, %xmm8, %xmm8     # xmm8 = xmm8[3,3,3,3]
1407f5f7b: 44 0f 59 c2                 	mulps	%xmm2, %xmm8
1407f5f7f: 44 0f 58 c0                 	addps	%xmm0, %xmm8
1407f5f83: 41 0f 58 f8                 	addps	%xmm8, %xmm7
1407f5f87: 45 0f 12 c0                 	movhlps	%xmm8, %xmm8            # xmm8 = xmm8[1,1]
1407f5f8b: f3 44 0f 58 43 18           	addss	0x18(%rbx), %xmm8
1407f5f91: 89 7c 24 28                 	movl	%edi, 0x28(%rsp)
1407f5f95: 48 8d 05 64 5c 29 02        	leaq	0x2295c64(%rip), %rax   # 0x142a8bc00
1407f5f9c: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1407f5fa1: 48 c7 44 24 38 05 00 00 00  	movq	$0x5, 0x38(%rsp)
1407f5faa: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
1407f5faf: e8 7c 6c a9 00              	callq	0x14128cc30 <.text+0x128bc30>
1407f5fb4: 48 89 c7                    	movq	%rax, %rdi
1407f5fb7: 49 8b 0c 24                 	movq	(%r12), %rcx
1407f5fbb: 48 83 c1 58                 	addq	$0x58, %rcx
1407f5fbf: e8 fc 86 13 00              	callq	0x14092e6c0 <.text+0x92d6c0>
1407f5fc4: 89 c3                       	movl	%eax, %ebx
1407f5fc6: 80 cb 04                    	orb	$0x4, %bl
1407f5fc9: 4c 8b a6 08 01 00 00        	movq	0x108(%rsi), %r12
1407f5fd0: 4c 3b a6 f8 00 00 00        	cmpq	0xf8(%rsi), %r12
1407f5fd7: 75 13                       	jne	0x1407f5fec <.text+0x7f4fec>
1407f5fd9: 48 8d 8e f8 00 00 00        	leaq	0xf8(%rsi), %rcx
1407f5fe0: 48 8d 15 21 5c 29 02        	leaq	0x2295c21(%rip), %rdx   # 0x142a8bc08
1407f5fe7: e8 14 6a f9 ff              	callq	0x14078ca00 <.text+0x78ba00>
1407f5fec: 48 8b 86 00 01 00 00        	movq	0x100(%rsi), %rax
1407f5ff3: 4b 8d 0c a4                 	leaq	(%r12,%r12,4), %rcx
1407f5ff7: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407f5ffb: 88 1c 08                    	movb	%bl, (%rax,%rcx)
1407f5ffe: 0f 29 74 08 10              	movaps	%xmm6, 0x10(%rax,%rcx)
1407f6003: 0f 13 7c 08 20              	movlps	%xmm7, 0x20(%rax,%rcx)
1407f6008: f3 44 0f 11 44 08 28        	movss	%xmm8, 0x28(%rax,%rcx)
1407f600f: 41 0f 10 07                 	movups	(%r15), %xmm0
1407f6013: 0f 11 44 08 2c              	movups	%xmm0, 0x2c(%rax,%rcx)
1407f6018: 41 8b 57 10                 	movl	0x10(%r15), %edx
1407f601c: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f6020: 48 89 7c 08 40              	movq	%rdi, 0x40(%rax,%rcx)
1407f6025: 4c 89 74 08 48              	movq	%r14, 0x48(%rax,%rcx)
1407f602a: 49 ff c4                    	incq	%r12
1407f602d: 4c 89 a6 08 01 00 00        	movq	%r12, 0x108(%rsi)
1407f6034: e9 2e 01 00 00              	jmp	0x1407f6167 <.text+0x7f5167>
1407f6039: 0f 28 33                    	movaps	(%rbx), %xmm6
1407f603c: f2 0f 10 7b 10              	movsd	0x10(%rbx), %xmm7
1407f6041: 4c 8d 7b 1c                 	leaq	0x1c(%rbx), %r15
1407f6045: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407f6048: 0f 59 ce                    	mulps	%xmm6, %xmm1
1407f604b: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
1407f604f: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407f6053: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407f6056: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
1407f605a: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407f605e: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407f6062: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1407f6066: 0f 5c ca                    	subps	%xmm2, %xmm1
1407f6069: 0f 28 05 c0 55 29 02        	movaps	0x22955c0(%rip), %xmm0  # 0x142a8b630
1407f6070: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407f6073: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f6076: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1407f607a: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407f607e: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407f6081: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1407f6085: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407f6089: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f608d: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f6090: 0f 58 c1                    	addps	%xmm1, %xmm0
1407f6093: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407f6096: 0f c6 ce d2                 	shufps	$0xd2, %xmm6, %xmm1     # xmm1 = xmm1[2,0],xmm6[1,3]
1407f609a: 0f 59 0d 9f 55 29 02        	mulps	0x229559f(%rip), %xmm1  # 0x142a8b640
1407f60a1: 0f 28 d6                    	movaps	%xmm6, %xmm2
1407f60a4: 0f c6 d6 c9                 	shufps	$0xc9, %xmm6, %xmm2     # xmm2 = xmm2[1,2],xmm6[0,3]
1407f60a8: 0f 59 15 a1 55 29 02        	mulps	0x22955a1(%rip), %xmm2  # 0x142a8b650
1407f60af: 0f 5c d1                    	subps	%xmm1, %xmm2
1407f60b2: 44 0f 28 c6                 	movaps	%xmm6, %xmm8
1407f60b6: 44 0f 58 c6                 	addps	%xmm6, %xmm8
1407f60ba: 45 0f c6 c0 ff              	shufps	$0xff, %xmm8, %xmm8     # xmm8 = xmm8[3,3,3,3]
1407f60bf: 44 0f 59 c2                 	mulps	%xmm2, %xmm8
1407f60c3: 44 0f 58 c0                 	addps	%xmm0, %xmm8
1407f60c7: 41 0f 58 f8                 	addps	%xmm8, %xmm7
1407f60cb: 45 0f 12 c0                 	movhlps	%xmm8, %xmm8            # xmm8 = xmm8[1,1]
1407f60cf: f3 44 0f 58 43 18           	addss	0x18(%rbx), %xmm8
1407f60d5: 89 7c 24 28                 	movl	%edi, 0x28(%rsp)
1407f60d9: 48 8d 05 00 5b 29 02        	leaq	0x2295b00(%rip), %rax   # 0x142a8bbe0
1407f60e0: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1407f60e5: 48 c7 44 24 38 08 00 00 00  	movq	$0x8, 0x38(%rsp)
1407f60ee: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
1407f60f3: e8 38 6b a9 00              	callq	0x14128cc30 <.text+0x128bc30>
1407f60f8: 48 89 c7                    	movq	%rax, %rdi
1407f60fb: 48 8b 9e 08 01 00 00        	movq	0x108(%rsi), %rbx
1407f6102: 48 3b 9e f8 00 00 00        	cmpq	0xf8(%rsi), %rbx
1407f6109: 75 13                       	jne	0x1407f611e <.text+0x7f511e>
1407f610b: 48 8d 8e f8 00 00 00        	leaq	0xf8(%rsi), %rcx
1407f6112: 48 8d 15 cf 5a 29 02        	leaq	0x2295acf(%rip), %rdx   # 0x142a8bbe8
1407f6119: e8 e2 68 f9 ff              	callq	0x14078ca00 <.text+0x78ba00>
1407f611e: 48 8b 86 00 01 00 00        	movq	0x100(%rsi), %rax
1407f6125: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
1407f6129: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407f612d: c6 04 08 03                 	movb	$0x3, (%rax,%rcx)
1407f6131: 0f 29 74 08 10              	movaps	%xmm6, 0x10(%rax,%rcx)
1407f6136: 0f 13 7c 08 20              	movlps	%xmm7, 0x20(%rax,%rcx)
1407f613b: f3 44 0f 11 44 08 28        	movss	%xmm8, 0x28(%rax,%rcx)
1407f6142: 41 0f 10 07                 	movups	(%r15), %xmm0
1407f6146: 0f 11 44 08 2c              	movups	%xmm0, 0x2c(%rax,%rcx)
1407f614b: 41 8b 57 10                 	movl	0x10(%r15), %edx
1407f614f: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f6153: 48 89 7c 08 40              	movq	%rdi, 0x40(%rax,%rcx)
1407f6158: 4c 89 74 08 48              	movq	%r14, 0x48(%rax,%rcx)
1407f615d: 48 ff c3                    	incq	%rbx
1407f6160: 48 89 9e 08 01 00 00        	movq	%rbx, 0x108(%rsi)
1407f6167: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
1407f616c: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
1407f6171: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
1407f6177: 48 83 c4 78                 	addq	$0x78, %rsp
1407f617b: 5b                          	popq	%rbx
1407f617c: 5f                          	popq	%rdi
1407f617d: 5e                          	popq	%rsi
1407f617e: 41 5c                       	popq	%r12
1407f6180: 41 5e                       	popq	%r14
1407f6182: 41 5f                       	popq	%r15
1407f6184: c3                          	retq
1407f6185: cc                          	int3
1407f6186: cc                          	int3
1407f6187: cc                          	int3
1407f6188: cc                          	int3
1407f6189: cc                          	int3
1407f618a: cc                          	int3
1407f618b: cc                          	int3
1407f618c: cc                          	int3
1407f618d: cc                          	int3
1407f618e: cc                          	int3
1407f618f: cc                          	int3
1407f6190: 48 83 ec 68                 	subq	$0x68, %rsp
1407f6194: 48 89 d0                    	movq	%rdx, %rax
1407f6197: 80 39 1a                    	cmpb	$0x1a, (%rcx)
1407f619a: 75 19                       	jne	0x1407f61b5 <.text+0x7f51b5>
1407f619c: 48 8d 15 5f 5b 29 02        	leaq	0x2295b5f(%rip), %rdx   # 0x142a8bd02
1407f61a3: 41 b8 16 00 00 00           	movl	$0x16, %r8d
1407f61a9: 48 89 c1                    	movq	%rax, %rcx
1407f61ac: 48 83 c4 68                 	addq	$0x68, %rsp
1407f61b0: e9 db 09 59 00              	jmp	0x140d86b90 <.text+0xd85b90>
1407f61b5: 48 89 4c 24 60              	movq	%rcx, 0x60(%rsp)
1407f61ba: 48 ff c1                    	incq	%rcx
1407f61bd: 48 8d 15 0c 5b 29 02        	leaq	0x2295b0c(%rip), %rdx   # 0x142a8bcd0
1407f61c4: 48 89 54 24 50              	movq	%rdx, 0x50(%rsp)
1407f61c9: 48 8d 54 24 60              	leaq	0x60(%rsp), %rdx
1407f61ce: 48 89 54 24 48              	movq	%rdx, 0x48(%rsp)
1407f61d3: 48 8d 15 21 5b 29 02        	leaq	0x2295b21(%rip), %rdx   # 0x142a8bcfb
1407f61da: 48 89 54 24 38              	movq	%rdx, 0x38(%rsp)
1407f61df: 48 8d 15 ca 5a 29 02        	leaq	0x2295aca(%rip), %rdx   # 0x142a8bcb0
1407f61e6: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
1407f61eb: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
1407f61f0: 48 c7 44 24 40 07 00 00 00  	movq	$0x7, 0x40(%rsp)
1407f61f9: 48 c7 44 24 20 04 00 00 00  	movq	$0x4, 0x20(%rsp)
1407f6202: 48 8d 15 e7 5a 29 02        	leaq	0x2295ae7(%rip), %rdx   # 0x142a8bcf0
1407f6209: 4c 8d 0d e7 5a 29 02        	leaq	0x2295ae7(%rip), %r9    # 0x142a8bcf7
1407f6210: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1407f6216: 48 89 c1                    	movq	%rax, %rcx
1407f6219: e8 82 0a 59 00              	callq	0x140d86ca0 <.text+0xd85ca0>
1407f621e: 90                          	nop
1407f621f: 48 83 c4 68                 	addq	$0x68, %rsp
