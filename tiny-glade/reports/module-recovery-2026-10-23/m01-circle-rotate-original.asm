
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409ac910: 55                          	pushq	%rbp
1409ac911: 41 57                       	pushq	%r15
1409ac913: 41 56                       	pushq	%r14
1409ac915: 41 55                       	pushq	%r13
1409ac917: 41 54                       	pushq	%r12
1409ac919: 56                          	pushq	%rsi
1409ac91a: 57                          	pushq	%rdi
1409ac91b: 53                          	pushq	%rbx
1409ac91c: 48 81 ec 38 03 00 00        	subq	$0x338, %rsp            # imm = 0x338
1409ac923: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409ac92b: 44 0f 29 bd a0 02 00 00     	movaps	%xmm15, 0x2a0(%rbp)
1409ac933: 44 0f 29 b5 90 02 00 00     	movaps	%xmm14, 0x290(%rbp)
1409ac93b: 44 0f 29 ad 80 02 00 00     	movaps	%xmm13, 0x280(%rbp)
1409ac943: 44 0f 29 a5 70 02 00 00     	movaps	%xmm12, 0x270(%rbp)
1409ac94b: 44 0f 29 9d 60 02 00 00     	movaps	%xmm11, 0x260(%rbp)
1409ac953: 44 0f 29 95 50 02 00 00     	movaps	%xmm10, 0x250(%rbp)
1409ac95b: 44 0f 29 8d 40 02 00 00     	movaps	%xmm9, 0x240(%rbp)
1409ac963: 44 0f 29 85 30 02 00 00     	movaps	%xmm8, 0x230(%rbp)
1409ac96b: 0f 29 bd 20 02 00 00        	movaps	%xmm7, 0x220(%rbp)
1409ac972: 0f 29 b5 10 02 00 00        	movaps	%xmm6, 0x210(%rbp)
1409ac979: 48 c7 85 08 02 00 00 fe ff ff ff    	movq	$-0x2, 0x208(%rbp)
1409ac984: 4c 8b 95 70 03 00 00        	movq	0x370(%rbp), %r10
1409ac98b: 48 8b 01                    	movq	(%rcx), %rax
1409ac98e: 8b 08                       	movl	(%rax), %ecx
1409ac990: 83 f9 05                    	cmpl	$0x5, %ecx
1409ac993: 0f 87 2f 01 00 00           	ja	0x1409acac8 <.text+0x9abac8>
1409ac999: 4d 89 cc                    	movq	%r9, %r12
1409ac99c: 4c 8b 9d 28 03 00 00        	movq	0x328(%rbp), %r11
1409ac9a3: 4c 8b 8d 20 03 00 00        	movq	0x320(%rbp), %r9
1409ac9aa: 49 8b 5c 24 40              	movq	0x40(%r12), %rbx
1409ac9af: 4d 8b 7c 24 50              	movq	0x50(%r12), %r15
1409ac9b4: 45 8b 6c 24 5c              	movl	0x5c(%r12), %r13d
1409ac9b9: 4d 8b 74 24 60              	movq	0x60(%r12), %r14
1409ac9be: 49 8b 31                    	movq	(%r9), %rsi
1409ac9c1: 48 89 75 f0                 	movq	%rsi, -0x10(%rbp)
1409ac9c5: 4d 8b 49 20                 	movq	0x20(%r9), %r9
1409ac9c9: 4c 89 4d 48                 	movq	%r9, 0x48(%rbp)
1409ac9cd: 49 8b 3b                    	movq	(%r11), %rdi
1409ac9d0: 49 8b 73 20                 	movq	0x20(%r11), %rsi
1409ac9d4: 4d 8b 4b 40                 	movq	0x40(%r11), %r9
1409ac9d8: 83 f9 01                    	cmpl	$0x1, %ecx
1409ac9db: 74 09                       	je	0x1409ac9e6 <.text+0x9ab9e6>
1409ac9dd: 83 f9 02                    	cmpl	$0x2, %ecx
1409ac9e0: 0f 85 e2 00 00 00           	jne	0x1409acac8 <.text+0x9abac8>
1409ac9e6: 44 89 ad a4 01 00 00        	movl	%r13d, 0x1a4(%rbp)
1409ac9ed: 4c 89 7d 60                 	movq	%r15, 0x60(%rbp)
1409ac9f1: 4c 89 75 58                 	movq	%r14, 0x58(%rbp)
1409ac9f5: 48 89 5d 50                 	movq	%rbx, 0x50(%rbp)
1409ac9f9: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
1409ac9fd: 4c 89 8d a8 01 00 00        	movq	%r9, 0x1a8(%rbp)
1409aca04: 4c 8b 68 08                 	movq	0x8(%rax), %r13
1409aca08: 4c 89 ad f8 01 00 00        	movq	%r13, 0x1f8(%rbp)
1409aca0f: 4d 8b 30                    	movq	(%r8), %r14
1409aca12: 4c 89 6d 70                 	movq	%r13, 0x70(%rbp)
1409aca16: 49 83 7e 18 00              	cmpq	$0x0, 0x18(%r14)
1409aca1b: 0f 84 07 02 00 00           	je	0x1409acc28 <.text+0x9abc28>
1409aca21: 49 8d 5e 20                 	leaq	0x20(%r14), %rbx
1409aca25: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
1409aca29: 48 89 d9                    	movq	%rbx, %rcx
1409aca2c: e8 3f 43 c0 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1409aca31: 4d 8b 3e                    	movq	(%r14), %r15
1409aca34: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
1409aca38: 48 89 ca                    	movq	%rcx, %rdx
1409aca3b: 48 21 c2                    	andq	%rax, %rdx
1409aca3e: 48 c1 e8 39                 	shrq	$0x39, %rax
1409aca42: 66 0f 6e c0                 	movd	%eax, %xmm0
1409aca46: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409aca4a: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409aca4f: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409aca54: 49 8d 87 58 fd ff ff        	leaq	-0x2a8(%r15), %rax
1409aca5b: 45 31 c0                    	xorl	%r8d, %r8d
1409aca5e: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1409aca62: f3 41 0f 6f 14 17           	movdqu	(%r15,%rdx), %xmm2
1409aca68: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1409aca6c: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1409aca70: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
1409aca75: 45 85 c9                    	testl	%r9d, %r9d
1409aca78: 74 2c                       	je	0x1409acaa6 <.text+0x9abaa6>
1409aca7a: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1409aca7f: 49 01 d2                    	addq	%rdx, %r10
1409aca82: 49 21 ca                    	andq	%rcx, %r10
1409aca85: 49 f7 da                    	negq	%r10
1409aca88: 4d 69 d2 a8 02 00 00        	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
1409aca8f: 4e 39 2c 10                 	cmpq	%r13, (%rax,%r10)
1409aca93: 0f 84 98 00 00 00           	je	0x1409acb31 <.text+0x9abb31>
1409aca99: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1409aca9d: 66 45 21 ca                 	andw	%r9w, %r10w
1409acaa1: 45 89 d1                    	movl	%r10d, %r9d
1409acaa4: 75 d4                       	jne	0x1409aca7a <.text+0x9aba7a>
1409acaa6: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1409acaaa: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1409acaaf: 45 85 c9                    	testl	%r9d, %r9d
1409acab2: 0f 85 70 01 00 00           	jne	0x1409acc28 <.text+0x9abc28>
1409acab8: 4c 01 c2                    	addq	%r8, %rdx
1409acabb: 48 83 c2 10                 	addq	$0x10, %rdx
1409acabf: 49 83 c0 10                 	addq	$0x10, %r8
1409acac3: 48 21 ca                    	andq	%rcx, %rdx
1409acac6: eb 9a                       	jmp	0x1409aca62 <.text+0x9aba62>
1409acac8: 41 c7 02 00 00 00 00        	movl	$0x0, (%r10)
1409acacf: 0f 28 b5 10 02 00 00        	movaps	0x210(%rbp), %xmm6
1409acad6: 0f 28 bd 20 02 00 00        	movaps	0x220(%rbp), %xmm7
1409acadd: 44 0f 28 85 30 02 00 00     	movaps	0x230(%rbp), %xmm8
1409acae5: 44 0f 28 8d 40 02 00 00     	movaps	0x240(%rbp), %xmm9
1409acaed: 44 0f 28 95 50 02 00 00     	movaps	0x250(%rbp), %xmm10
1409acaf5: 44 0f 28 9d 60 02 00 00     	movaps	0x260(%rbp), %xmm11
1409acafd: 44 0f 28 a5 70 02 00 00     	movaps	0x270(%rbp), %xmm12
1409acb05: 44 0f 28 ad 80 02 00 00     	movaps	0x280(%rbp), %xmm13
1409acb0d: 44 0f 28 b5 90 02 00 00     	movaps	0x290(%rbp), %xmm14
1409acb15: 44 0f 28 bd a0 02 00 00     	movaps	0x2a0(%rbp), %xmm15
1409acb1d: 48 81 c4 38 03 00 00        	addq	$0x338, %rsp            # imm = 0x338
1409acb24: 5b                          	popq	%rbx
1409acb25: 5f                          	popq	%rdi
1409acb26: 5e                          	popq	%rsi
1409acb27: 41 5c                       	popq	%r12
1409acb29: 41 5d                       	popq	%r13
1409acb2b: 41 5e                       	popq	%r14
1409acb2d: 41 5f                       	popq	%r15
1409acb2f: 5d                          	popq	%rbp
1409acb30: c3                          	retq
1409acb31: 4b 83 bc 17 70 fd ff ff 02  	cmpq	$0x2, -0x290(%r15,%r10)
1409acb3a: 0f 82 7b 12 00 00           	jb	0x1409addbb <.text+0x9acdbb>
1409acb40: f3 43 0f 10 84 17 90 fd ff ff       	movss	-0x270(%r15,%r10), %xmm0
1409acb4a: 66 0f ef c9                 	pxor	%xmm1, %xmm1
1409acb4e: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
1409acb51: 0f 86 64 12 00 00           	jbe	0x1409addbb <.text+0x9acdbb>
1409acb57: 4d 01 d7                    	addq	%r10, %r15
1409acb5a: 41 83 7f b8 00              	cmpl	$0x0, -0x48(%r15)
1409acb5f: 0f 85 6a ff ff ff           	jne	0x1409acacf <.text+0x9abacf>
1409acb65: f3 41 0f 6f 47 bc           	movdqu	-0x44(%r15), %xmm0
1409acb6b: 66 0f 7f 85 b0 01 00 00     	movdqa	%xmm0, 0x1b0(%rbp)
1409acb73: 4c 8b ad f8 01 00 00        	movq	0x1f8(%rbp), %r13
1409acb7a: 4c 89 6d 70                 	movq	%r13, 0x70(%rbp)
1409acb7e: 49 83 7e 18 00              	cmpq	$0x0, 0x18(%r14)
1409acb83: 0f 84 0e 06 00 00           	je	0x1409ad197 <.text+0x9ac197>
1409acb89: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
1409acb8d: 48 89 d9                    	movq	%rbx, %rcx
1409acb90: e8 db 41 c0 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1409acb95: 49 8b 0e                    	movq	(%r14), %rcx
1409acb98: 49 8b 56 08                 	movq	0x8(%r14), %rdx
1409acb9c: 49 89 d0                    	movq	%rdx, %r8
1409acb9f: 49 21 c0                    	andq	%rax, %r8
1409acba2: 48 c1 e8 39                 	shrq	$0x39, %rax
1409acba6: 66 0f 6e c0                 	movd	%eax, %xmm0
1409acbaa: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409acbae: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409acbb3: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409acbb8: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
1409acbbf: 45 31 c9                    	xorl	%r9d, %r9d
1409acbc2: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1409acbc6: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1409acbcc: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1409acbd0: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1409acbd4: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1409acbd9: 45 85 d2                    	testl	%r10d, %r10d
1409acbdc: 74 28                       	je	0x1409acc06 <.text+0x9abc06>
1409acbde: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1409acbe3: 4d 01 c3                    	addq	%r8, %r11
1409acbe6: 49 21 d3                    	andq	%rdx, %r11
1409acbe9: 49 f7 db                    	negq	%r11
1409acbec: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
1409acbf3: 4e 39 2c 18                 	cmpq	%r13, (%rax,%r11)
1409acbf7: 74 3b                       	je	0x1409acc34 <.text+0x9abc34>
1409acbf9: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1409acbfd: 66 45 21 d3                 	andw	%r10w, %r11w
1409acc01: 45 89 da                    	movl	%r11d, %r10d
1409acc04: 75 d8                       	jne	0x1409acbde <.text+0x9abbde>
1409acc06: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1409acc0a: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1409acc0f: 45 85 d2                    	testl	%r10d, %r10d
1409acc12: 0f 85 7f 05 00 00           	jne	0x1409ad197 <.text+0x9ac197>
1409acc18: 4d 01 c8                    	addq	%r9, %r8
1409acc1b: 49 83 c0 10                 	addq	$0x10, %r8
1409acc1f: 49 83 c1 10                 	addq	$0x10, %r9
1409acc23: 49 21 d0                    	andq	%rdx, %r8
1409acc26: eb 9e                       	jmp	0x1409acbc6 <.text+0x9abbc6>
1409acc28: 48 8d 0d 71 10 11 02        	leaq	0x2111071(%rip), %rcx   # 0x142abdca0
1409acc2f: e8 dc c6 f2 01              	callq	0x1428d9310 <.text+0x28d8310>
1409acc34: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
1409acc3d: 0f 82 90 11 00 00           	jb	0x1409addd3 <.text+0x9acdd3>
1409acc43: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
1409acc4d: 66 0f ef c9                 	pxor	%xmm1, %xmm1
1409acc51: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
1409acc54: 0f 86 79 11 00 00           	jbe	0x1409addd3 <.text+0x9acdd3>
1409acc5a: 4c 01 d9                    	addq	%r11, %rcx
1409acc5d: 48 8b 91 a8 fd ff ff        	movq	-0x258(%rcx), %rdx
1409acc64: 48 85 d2                    	testq	%rdx, %rdx
1409acc67: 4c 8b ad 68 03 00 00        	movq	0x368(%rbp), %r13
1409acc6e: 0f 84 9e 11 00 00           	je	0x1409ade12 <.text+0x9ace12>
1409acc74: 48 8b 81 a0 fd ff ff        	movq	-0x260(%rcx), %rax
1409acc7b: 48 8d 0c 52                 	leaq	(%rdx,%rdx,2), %rcx
1409acc7f: 48 8d 0c 88                 	leaq	(%rax,%rcx,4), %rcx
1409acc83: f3 44 0f 10 40 04           	movss	0x4(%rax), %xmm8
1409acc89: 8b 17                       	movl	(%rdi), %edx
1409acc8b: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409acc8e: f3 48 0f 2a c2              	cvtsi2ss	%rdx, %xmm0
1409acc93: f3 0f 11 85 f0 01 00 00     	movss	%xmm0, 0x1f0(%rbp)
1409acc9b: 0f 28 06                    	movaps	(%rsi), %xmm0
1409acc9e: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
1409acca5: 0f 28 56 10                 	movaps	0x10(%rsi), %xmm2
1409acca9: 0f 28 5e 20                 	movaps	0x20(%rsi), %xmm3
1409accad: 0f 28 66 30                 	movaps	0x30(%rsi), %xmm4
1409accb1: 0f 28 ae 80 00 00 00        	movaps	0x80(%rsi), %xmm5
1409accb8: 0f 28 b6 90 00 00 00        	movaps	0x90(%rsi), %xmm6
1409accbf: 44 0f 28 8e a0 00 00 00     	movaps	0xa0(%rsi), %xmm9
1409accc7: 44 0f 28 96 b0 00 00 00     	movaps	0xb0(%rsi), %xmm10
1409acccf: f3 0f 10 78 08              	movss	0x8(%rax), %xmm7
1409accd4: f3 0f 10 00                 	movss	(%rax), %xmm0
1409accd8: 0f 14 c7                    	unpcklps	%xmm7, %xmm0            # xmm0 = xmm0[0],xmm7[0],xmm0[1],xmm7[1]
1409accdb: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
1409acce2: f3 44 0f 10 25 7d fe 10 02  	movss	0x210fe7d(%rip), %xmm12 # 0x142abcb68
1409acceb: f3 44 0f 10 1d 7c 8b f7 01  	movss	0x1f78b7c(%rip), %xmm11 # 0x142925870
1409accf4: eb 38                       	jmp	0x1409acd2e <.text+0x9abd2e>
1409accf6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1409acd00: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
1409acd04: f3 41 0f c2 cc 01           	cmpltss	%xmm12, %xmm1
1409acd0a: 0f 54 c1                    	andps	%xmm1, %xmm0
1409acd0d: 41 0f 55 c8                 	andnps	%xmm8, %xmm1
1409acd11: 0f 56 c8                    	orps	%xmm0, %xmm1
1409acd14: 44 0f 28 c1                 	movaps	%xmm1, %xmm8
1409acd18: f3 45 0f 5d f4              	minss	%xmm12, %xmm14
1409acd1d: 48 83 c0 0c                 	addq	$0xc, %rax
1409acd21: 45 0f 28 e6                 	movaps	%xmm14, %xmm12
1409acd25: 48 39 c8                    	cmpq	%rcx, %rax
1409acd28: 0f 84 b0 00 00 00           	je	0x1409acdde <.text+0x9abdde>
1409acd2e: f3 44 0f 10 38              	movss	(%rax), %xmm15
1409acd33: f3 0f 10 40 04              	movss	0x4(%rax), %xmm0
1409acd38: f3 0f 10 78 08              	movss	0x8(%rax), %xmm7
1409acd3d: 45 0f 28 f7                 	movaps	%xmm15, %xmm14
1409acd41: 45 0f c6 f7 00              	shufps	$0x0, %xmm15, %xmm14    # xmm14 = xmm14[0,0],xmm15[0,0]
1409acd46: 44 0f 59 f5                 	mulps	%xmm5, %xmm14
1409acd4a: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
1409acd4e: 44 0f c6 e8 00              	shufps	$0x0, %xmm0, %xmm13     # xmm13 = xmm13[0,0],xmm0[0,0]
1409acd53: 44 0f 59 ee                 	mulps	%xmm6, %xmm13
1409acd57: 45 0f 58 ee                 	addps	%xmm14, %xmm13
1409acd5b: 44 0f 28 f7                 	movaps	%xmm7, %xmm14
1409acd5f: 44 0f c6 f7 00              	shufps	$0x0, %xmm7, %xmm14     # xmm14 = xmm14[0,0],xmm7[0,0]
1409acd64: 45 0f 59 f1                 	mulps	%xmm9, %xmm14
1409acd68: 45 0f 58 f5                 	addps	%xmm13, %xmm14
1409acd6c: 45 0f 58 f2                 	addps	%xmm10, %xmm14
1409acd70: 45 0f 28 ee                 	movaps	%xmm14, %xmm13
1409acd74: 45 0f c6 ee 00              	shufps	$0x0, %xmm14, %xmm13    # xmm13 = xmm13[0,0],xmm14[0,0]
1409acd79: 44 0f 59 ad 70 01 00 00     	mulps	0x170(%rbp), %xmm13
1409acd81: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
1409acd85: 41 0f c6 ce 55              	shufps	$0x55, %xmm14, %xmm1    # xmm1 = xmm1[1,1],xmm14[1,1]
1409acd8a: 0f 59 ca                    	mulps	%xmm2, %xmm1
1409acd8d: 41 0f 58 cd                 	addps	%xmm13, %xmm1
1409acd91: 45 0f c6 f6 aa              	shufps	$0xaa, %xmm14, %xmm14   # xmm14 = xmm14[2,2,2,2]
1409acd96: 44 0f 59 f3                 	mulps	%xmm3, %xmm14
1409acd9a: 44 0f 58 f1                 	addps	%xmm1, %xmm14
1409acd9e: 44 0f 58 f4                 	addps	%xmm4, %xmm14
1409acda2: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
1409acda6: 41 0f c6 ce ff              	shufps	$0xff, %xmm14, %xmm1    # xmm1 = xmm1[3,3],xmm14[3,3]
1409acdab: f3 44 0f 5e f1              	divss	%xmm1, %xmm14
1409acdb0: f3 45 0f 59 f3              	mulss	%xmm11, %xmm14
1409acdb5: f3 45 0f 58 f3              	addss	%xmm11, %xmm14
1409acdba: f3 44 0f 59 b5 f0 01 00 00  	mulss	0x1f0(%rbp), %xmm14
1409acdc3: 45 0f 2e e6                 	ucomiss	%xmm14, %xmm12
1409acdc7: 0f 86 33 ff ff ff           	jbe	0x1409acd00 <.text+0x9abd00>
1409acdcd: 44 0f 14 ff                 	unpcklps	%xmm7, %xmm15           # xmm15 = xmm15[0],xmm7[0],xmm15[1],xmm7[1]
1409acdd1: 44 0f 29 bd 80 01 00 00     	movaps	%xmm15, 0x180(%rbp)
1409acdd9: e9 22 ff ff ff              	jmp	0x1409acd00 <.text+0x9abd00>
1409acdde: f2 44 0f 10 b5 b0 01 00 00  	movsd	0x1b0(%rbp), %xmm14
1409acde7: 0f 28 b5 80 01 00 00        	movaps	0x180(%rbp), %xmm6
1409acdee: 0f 28 c6                    	movaps	%xmm6, %xmm0
1409acdf1: 41 0f 5c c6                 	subps	%xmm14, %xmm0
1409acdf5: 0f 28 c8                    	movaps	%xmm0, %xmm1
1409acdf8: 0f 59 c8                    	mulps	%xmm0, %xmm1
1409acdfb: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1409acdff: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1409ace03: 0f 57 c9                    	xorps	%xmm1, %xmm1
1409ace06: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
1409ace0a: f3 44 0f 10 2d f1 8a f7 01  	movss	0x1f78af1(%rip), %xmm13 # 0x142925904
1409ace13: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
1409ace17: f3 0f 5e d1                 	divss	%xmm1, %xmm2
1409ace1b: f3 0f 12 ca                 	movsldup	%xmm2, %xmm1            # xmm1 = xmm2[0,0,2,2]
1409ace1f: 0f 59 c8                    	mulps	%xmm0, %xmm1
1409ace22: 0f 13 8d 68 01 00 00        	movlps	%xmm1, 0x168(%rbp)
1409ace29: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409ace2d: 48 8d 9d 68 01 00 00        	leaq	0x168(%rbp), %rbx
1409ace34: 0f 57 d2                    	xorps	%xmm2, %xmm2
1409ace37: 48 89 da                    	movq	%rbx, %rdx
1409ace3a: e8 a1 ff 2e 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409ace3f: f3 44 0f 16 ce              	movshdup	%xmm6, %xmm9            # xmm9 = xmm6[1,1,3,3]
1409ace44: f3 44 0f 58 4d 78           	addss	0x78(%rbp), %xmm9
1409ace4a: f2 0f 10 45 70              	movsd	0x70(%rbp), %xmm0
1409ace4f: 44 0f 16 c6                 	movlhps	%xmm6, %xmm8            # xmm8 = xmm8[0],xmm6[0]
1409ace53: 44 0f c6 c6 e2              	shufps	$0xe2, %xmm6, %xmm8     # xmm8 = xmm8[2,0],xmm6[2,3]
1409ace58: 44 0f 58 c0                 	addps	%xmm0, %xmm8
1409ace5c: f3 0f 10 8d 68 01 00 00     	movss	0x168(%rbp), %xmm1
1409ace64: f3 0f 10 95 6c 01 00 00     	movss	0x16c(%rbp), %xmm2
1409ace6c: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409ace70: e8 5b 41 2e 00              	callq	0x140c90fd0 <.text+0xc8ffd0>
1409ace75: 0f 28 75 70                 	movaps	0x70(%rbp), %xmm6
1409ace79: 48 8d 75 70                 	leaq	0x70(%rbp), %rsi
1409ace7d: 0f 57 d2                    	xorps	%xmm2, %xmm2
1409ace80: 48 89 f1                    	movq	%rsi, %rcx
1409ace83: 48 89 da                    	movq	%rbx, %rdx
1409ace86: e8 55 ff 2e 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409ace8b: f2 0f 10 45 70              	movsd	0x70(%rbp), %xmm0
1409ace90: 0f 59 05 c9 2b 0d 02        	mulps	0x20d2bc9(%rip), %xmm0  # 0x142a7fa60
1409ace97: f3 0f 10 4d 78              	movss	0x78(%rbp), %xmm1
1409ace9c: f3 0f 59 0d 34 8a f7 01     	mulss	0x1f78a34(%rip), %xmm1  # 0x1429258d8
1409acea4: 41 0f 58 c0                 	addps	%xmm8, %xmm0
1409acea8: f3 41 0f 58 c9              	addss	%xmm9, %xmm1
1409acead: 0f 13 85 80 00 00 00        	movlps	%xmm0, 0x80(%rbp)
1409aceb4: f3 0f 11 8d 88 00 00 00     	movss	%xmm1, 0x88(%rbp)
1409acebc: 0f 29 75 70                 	movaps	%xmm6, 0x70(%rbp)
1409acec0: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
1409aceca: 48 89 85 8c 00 00 00        	movq	%rax, 0x8c(%rbp)
1409aced1: c7 85 94 00 00 00 00 00 80 3f       	movl	$0x3f800000, 0x94(%rbp) # imm = 0x3F800000
1409acedb: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1409acee0: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
1409acee4: f3 0f 10 0d 80 fc 10 02     	movss	0x210fc80(%rip), %xmm1  # 0x142abcb6c
1409aceec: f3 0f 10 15 7c 89 f7 01     	movss	0x1f7897c(%rip), %xmm2  # 0x142925870
1409acef4: 66 0f 6e 1d d4 89 f7 01     	movd	0x1f789d4(%rip), %xmm3  # 0x1429258d0
1409acefc: e8 df d1 12 00              	callq	0x140ada0e0 <.text+0xad90e0>
1409acf01: 48 8d 85 f8 01 00 00        	leaq	0x1f8(%rbp), %rax
1409acf08: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1409acf0f: 48 8d 05 aa 7f 18 00        	leaq	0x187faa(%rip), %rax    # 0x140b34ec0 <.text+0xb33ec0>
1409acf16: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1409acf1d: 48 8d 05 ec 0d 11 02        	leaq	0x2110dec(%rip), %rax   # 0x142abdd10
1409acf24: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1409acf28: 48 c7 45 78 01 00 00 00     	movq	$0x1, 0x78(%rbp)
1409acf30: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1409acf37: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
1409acf3e: 48 c7 85 88 00 00 00 01 00 00 00    	movq	$0x1, 0x88(%rbp)
1409acf49: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
1409acf54: c6 85 06 02 00 00 01        	movb	$0x1, 0x206(%rbp)
1409acf5b: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1409acf5f: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
1409acf63: e8 28 d0 3c 00              	callq	0x140d79f90 <.text+0xd78f90>
1409acf68: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
1409acf6c: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
1409acf70: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1409acf74: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1409acf78: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
1409acf7c: 0f 11 85 c7 01 00 00        	movups	%xmm0, 0x1c7(%rbp)
1409acf83: 48 8b 45 08                 	movq	0x8(%rbp), %rax
1409acf87: 48 89 85 d7 01 00 00        	movq	%rax, 0x1d7(%rbp)
1409acf8e: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1409acf95: 0f 11 85 f9 00 00 00        	movups	%xmm0, 0xf9(%rbp)
1409acf9c: 48 8b 85 cf 01 00 00        	movq	0x1cf(%rbp), %rax
1409acfa3: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1409acfaa: 48 8b 85 d7 01 00 00        	movq	0x1d7(%rbp), %rax
1409acfb1: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1409acfb8: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
1409acfc0: 48 c7 45 78 08 00 00 00     	movq	$0x8, 0x78(%rbp)
1409acfc8: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
1409acfd3: c7 85 88 00 00 00 01 00 00 00       	movl	$0x1, 0x88(%rbp)
1409acfdd: 48 c7 85 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rbp)
1409acfe8: c7 85 a0 00 00 00 02 00 00 00       	movl	$0x2, 0xa0(%rbp)
1409acff2: c7 85 b8 00 00 00 03 00 00 00       	movl	$0x3, 0xb8(%rbp)
1409acffc: c7 85 d0 00 00 00 03 00 00 00       	movl	$0x3, 0xd0(%rbp)
1409ad006: c6 85 e8 00 00 00 00        	movb	$0x0, 0xe8(%rbp)
1409ad00d: c6 85 f8 00 00 00 02        	movb	$0x2, 0xf8(%rbp)
1409ad014: 66 c7 85 18 01 00 00 00 01  	movw	$0x100, 0x118(%rbp)     # imm = 0x100
1409ad01d: c6 85 06 02 00 00 00        	movb	$0x0, 0x206(%rbp)
1409ad024: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
1409ad028: 4c 8d 45 70                 	leaq	0x70(%rbp), %r8
1409ad02c: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1409ad030: e8 bb 61 f7 ff              	callq	0x1409231f0 <.text+0x9221f0>
1409ad035: b1 02                       	movb	$0x2, %cl
1409ad037: 80 78 18 02                 	cmpb	$0x2, 0x18(%rax)
1409ad03b: 72 0a                       	jb	0x1409ad047 <.text+0x9ac047>
1409ad03d: 48 39 50 38                 	cmpq	%rdx, 0x38(%rax)
1409ad041: 75 04                       	jne	0x1409ad047 <.text+0x9ac047>
1409ad043: 0f b6 48 19                 	movzbl	0x19(%rax), %ecx
1409ad047: 48 89 95 70 01 00 00        	movq	%rdx, 0x170(%rbp)
1409ad04e: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1409ad055: 80 c1 fe                    	addb	$-0x2, %cl
1409ad058: 0f b6 c1                    	movzbl	%cl, %eax
1409ad05b: 3c 03                       	cmpb	$0x3, %al
1409ad05d: b9 03 00 00 00              	movl	$0x3, %ecx
1409ad062: 0f 42 c8                    	cmovbl	%eax, %ecx
1409ad065: 0f b6 c1                    	movzbl	%cl, %eax
1409ad068: 48 8d 0d 39 10 11 02        	leaq	0x2111039(%rip), %rcx   # 0x142abe0a8
1409ad06f: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1409ad073: 48 01 c8                    	addq	%rcx, %rax
1409ad076: f3 41 0f 16 f8              	movshdup	%xmm8, %xmm7            # xmm7 = xmm8[1,1,3,3]
1409ad07b: ff e0                       	jmpq	*%rax
1409ad07d: 48 8b 5d 60                 	movq	0x60(%rbp), %rbx
1409ad081: 44 8b b5 a4 01 00 00        	movl	0x1a4(%rbp), %r14d
1409ad088: 44 89 33                    	movl	%r14d, (%rbx)
1409ad08b: 4c 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %r8
1409ad092: 48 8d 05 87 0c 11 02        	leaq	0x2110c87(%rip), %rax   # 0x142abdd20
1409ad099: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1409ad09e: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
1409ad0a3: 48 c7 44 24 30 0d 00 00 00  	movq	$0xd, 0x30(%rsp)
1409ad0ac: 48 8b 75 50                 	movq	0x50(%rbp), %rsi
1409ad0b0: 48 89 f1                    	movq	%rsi, %rcx
1409ad0b3: 31 d2                       	xorl	%edx, %edx
1409ad0b5: 48 8b 7d 58                 	movq	0x58(%rbp), %rdi
1409ad0b9: 49 89 f9                    	movq	%rdi, %r9
1409ad0bc: e8 6f a1 a3 00              	callq	0x1413e7230 <.text+0x13e6230>
1409ad0c1: 44 89 33                    	movl	%r14d, (%rbx)
1409ad0c4: 48 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %rdx
1409ad0cb: 48 89 f1                    	movq	%rsi, %rcx
1409ad0ce: 49 89 f8                    	movq	%rdi, %r8
1409ad0d1: e8 2a eb 17 00              	callq	0x140b2bc00 <.text+0xb2ac00>
1409ad0d6: f3 41 0f 16 d6              	movshdup	%xmm14, %xmm2           # xmm2 = xmm14[1,1,3,3]
1409ad0db: 48 8d bd bc 01 00 00        	leaq	0x1bc(%rbp), %rdi
1409ad0e2: 45 0f 28 d0                 	movaps	%xmm8, %xmm10
1409ad0e6: f3 45 0f 5c d6              	subss	%xmm14, %xmm10
1409ad0eb: 45 0f 28 d9                 	movaps	%xmm9, %xmm11
1409ad0ef: f3 44 0f 5c da              	subss	%xmm2, %xmm11
1409ad0f4: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
1409ad0fb: f2 0f 10 00                 	movsd	(%rax), %xmm0
1409ad0ff: f2 0f 10 48 0c              	movsd	0xc(%rax), %xmm1
1409ad104: 41 0f 28 d8                 	movaps	%xmm8, %xmm3
1409ad108: 0f 5c d8                    	subps	%xmm0, %xmm3
1409ad10b: f3 44 0f 16 fb              	movshdup	%xmm3, %xmm15           # xmm15 = xmm3[1,1,3,3]
1409ad110: f3 0f 16 d9                 	movshdup	%xmm1, %xmm3            # xmm3 = xmm1[1,1,3,3]
1409ad114: f3 44 0f 5e fb              	divss	%xmm3, %xmm15
1409ad119: f3 41 0f 12 f7              	movsldup	%xmm15, %xmm6           # xmm6 = xmm15[0,0,2,2]
1409ad11e: f3 44 0f 59 78 14           	mulss	0x14(%rax), %xmm15
1409ad124: 0f 59 f1                    	mulps	%xmm1, %xmm6
1409ad127: 0f 58 f0                    	addps	%xmm0, %xmm6
1409ad12a: f3 44 0f 58 78 08           	addss	0x8(%rax), %xmm15
1409ad130: 0f 28 c6                    	movaps	%xmm6, %xmm0
1409ad133: f3 41 0f 5c c6              	subss	%xmm14, %xmm0
1409ad138: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
1409ad13c: f3 0f 5c ca                 	subss	%xmm2, %xmm1
1409ad140: e8 db 3c 2e 00              	callq	0x140c90e20 <.text+0xc8fe20>
1409ad145: 44 0f 28 e1                 	movaps	%xmm1, %xmm12
1409ad149: 48 89 f9                    	movq	%rdi, %rcx
1409ad14c: e8 bf 60 d4 00              	callq	0x1416f3210 <.text+0x16f2210>
1409ad151: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
1409ad155: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
1409ad159: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
1409ad15d: e8 be 3c 2e 00              	callq	0x140c90e20 <.text+0xc8fe20>
1409ad162: f3 41 0f 5c cc              	subss	%xmm12, %xmm1
1409ad167: f3 41 0f 11 4d 00           	movss	%xmm1, (%r13)
1409ad16d: f3 45 0f 11 65 04           	movss	%xmm12, 0x4(%r13)
1409ad173: f3 45 0f 11 6d 08           	movss	%xmm13, 0x8(%r13)
1409ad179: f3 44 0f 10 2d 82 87 f7 01  	movss	0x1f78782(%rip), %xmm13 # 0x142925904
1409ad182: 41 0f 13 75 0c              	movlps	%xmm6, 0xc(%r13)
1409ad187: f3 45 0f 11 7d 14           	movss	%xmm15, 0x14(%r13)
1409ad18d: 41 c6 45 18 00              	movb	$0x0, 0x18(%r13)
1409ad192: e9 b8 04 00 00              	jmp	0x1409ad64f <.text+0x9ac64f>
1409ad197: 48 8d 0d 32 0b 11 02        	leaq	0x2110b32(%rip), %rcx   # 0x142abdcd0
1409ad19e: e8 6d c1 f2 01              	callq	0x1428d9310 <.text+0x28d8310>
1409ad1a3: f3 45 0f 10 45 10           	movss	0x10(%r13), %xmm8
1409ad1a9: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
1409ad1b0: f2 0f 10 00                 	movsd	(%rax), %xmm0
1409ad1b4: f2 0f 10 48 0c              	movsd	0xc(%rax), %xmm1
1409ad1b9: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1409ad1bd: 45 0f 28 d8                 	movaps	%xmm8, %xmm11
1409ad1c1: f3 44 0f 5c da              	subss	%xmm2, %xmm11
1409ad1c6: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1409ad1ca: f3 44 0f 5e da              	divss	%xmm2, %xmm11
1409ad1cf: f3 45 0f 12 e3              	movsldup	%xmm11, %xmm12          # xmm12 = xmm11[0,0,2,2]
1409ad1d4: 44 0f 59 e1                 	mulps	%xmm1, %xmm12
1409ad1d8: f3 44 0f 59 58 14           	mulss	0x14(%rax), %xmm11
1409ad1de: 44 0f 58 e0                 	addps	%xmm0, %xmm12
1409ad1e2: 48 89 85 a8 01 00 00        	movq	%rax, 0x1a8(%rbp)
1409ad1e9: f3 44 0f 58 58 08           	addss	0x8(%rax), %xmm11
1409ad1ef: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1409ad1f3: f3 41 0f 5c c6              	subss	%xmm14, %xmm0
1409ad1f8: f3 45 0f 16 d6              	movshdup	%xmm14, %xmm10          # xmm10 = xmm14[1,1,3,3]
1409ad1fd: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
1409ad201: f3 41 0f 5c ca              	subss	%xmm10, %xmm1
1409ad206: e8 15 3c 2e 00              	callq	0x140c90e20 <.text+0xc8fe20>
1409ad20b: f3 41 0f 10 45 08           	movss	0x8(%r13), %xmm0
1409ad211: f3 41 0f 5c 45 04           	subss	0x4(%r13), %xmm0
1409ad217: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1409ad21b: e8 b0 5f d4 00              	callq	0x1416f31d0 <.text+0x16f21d0>
1409ad220: f3 0f 11 85 a0 01 00 00     	movss	%xmm0, 0x1a0(%rbp)
1409ad228: f3 45 0f 10 4d 00           	movss	(%r13), %xmm9
1409ad22e: 0f 2e 85 bc 01 00 00        	ucomiss	0x1bc(%rbp), %xmm0
1409ad235: 75 06                       	jne	0x1409ad23d <.text+0x9ac23d>
1409ad237: 0f 8b e1 04 00 00           	jnp	0x1409ad71e <.text+0x9ac71e>
1409ad23d: 48 8d 4d 78                 	leaq	0x78(%rbp), %rcx
1409ad241: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
1409ad248: 0f 28 d7                    	movaps	%xmm7, %xmm2
1409ad24b: e8 90 fb 2e 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409ad250: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
1409ad257: f3 0f 10 8d bc 01 00 00     	movss	0x1bc(%rbp), %xmm1
1409ad25f: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1409ad266: e8 f5 5f d4 00              	callq	0x1416f3260 <.text+0x16f2260>
1409ad26b: 0f 28 d0                    	movaps	%xmm0, %xmm2
1409ad26e: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
1409ad275: 48 8b 00                    	movq	(%rax), %rax
1409ad278: f3 0f 5e 50 28              	divss	0x28(%rax), %xmm2
1409ad27d: 0f 57 f6                    	xorps	%xmm6, %xmm6
1409ad280: f3 0f 10 0d e4 fd 01 02     	movss	0x201fde4(%rip), %xmm1  # 0x1429cd06c
1409ad288: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409ad28b: e8 10 32 2e 00              	callq	0x140c904a0 <.text+0xc8f4a0>
1409ad290: f3 0f 5f f0                 	maxss	%xmm0, %xmm6
1409ad294: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1409ad298: f3 0f 5d c6                 	minss	%xmm6, %xmm0
1409ad29c: c6 85 88 00 00 00 02        	movb	$0x2, 0x88(%rbp)
1409ad2a3: 48 89 75 70                 	movq	%rsi, 0x70(%rbp)
1409ad2a7: f3 0f 11 85 84 00 00 00     	movss	%xmm0, 0x84(%rbp)
1409ad2af: 48 8b 95 58 03 00 00        	movq	0x358(%rbp), %rdx
1409ad2b6: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409ad2b9: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409ad2bd: 89 01                       	movl	%eax, (%rcx)
1409ad2bf: 48 8b 32                    	movq	(%rdx), %rsi
1409ad2c2: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
1409ad2c6: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
1409ad2ca: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
1409ad2ce: 75 10                       	jne	0x1409ad2e0 <.text+0x9ac2e0>
1409ad2d0: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1409ad2d4: 48 8d 15 6d ce 28 02        	leaq	0x228ce6d(%rip), %rdx   # 0x142c3a148
1409ad2db: e8 d0 57 bb ff              	callq	0x140562ab0 <.text+0x561ab0>
1409ad2e0: 48 8b 46 28                 	movq	0x28(%rsi), %rax
1409ad2e4: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
1409ad2e8: 48 89 1c c8                 	movq	%rbx, (%rax,%rcx,8)
1409ad2ec: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
1409ad2f0: f3 0f 6f 8d 80 00 00 00     	movdqu	0x80(%rbp), %xmm1
1409ad2f8: f3 0f 7f 4c c8 18           	movdqu	%xmm1, 0x18(%rax,%rcx,8)
1409ad2fe: 0f 11 44 c8 08              	movups	%xmm0, 0x8(%rax,%rcx,8)
1409ad303: 48 ff c7                    	incq	%rdi
1409ad306: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
1409ad30a: 48 ff 46 40                 	incq	0x40(%rsi)
1409ad30e: 8b 85 b8 01 00 00           	movl	0x1b8(%rbp), %eax
1409ad314: 89 85 c8 01 00 00           	movl	%eax, 0x1c8(%rbp)
1409ad31a: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1409ad321: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1409ad328: f3 0f 10 85 a0 01 00 00     	movss	0x1a0(%rbp), %xmm0
1409ad330: f3 0f 11 85 cc 01 00 00     	movss	%xmm0, 0x1cc(%rbp)
1409ad338: 48 8b 45 68                 	movq	0x68(%rbp), %rax
1409ad33c: 44 8b 60 1c                 	movl	0x1c(%rax), %r12d
1409ad340: 4c 8b 68 10                 	movq	0x10(%rax), %r13
1409ad344: 45 89 65 00                 	movl	%r12d, (%r13)
1409ad348: 48 8b 30                    	movq	(%rax), %rsi
1409ad34b: 48 8b bd f8 01 00 00        	movq	0x1f8(%rbp), %rdi
1409ad352: 48 8d 5d 70                 	leaq	0x70(%rbp), %rbx
1409ad356: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1409ad35d: 66 0f 6e 15 0b 85 f7 01     	movd	0x1f7850b(%rip), %xmm2  # 0x142925870
1409ad365: 48 89 d9                    	movq	%rbx, %rcx
1409ad368: e8 83 a9 2e 00              	callq	0x140c97cf0 <.text+0xc96cf0>
1409ad36d: 48 89 f1                    	movq	%rsi, %rcx
1409ad370: 48 89 fa                    	movq	%rdi, %rdx
1409ad373: 49 89 d8                    	movq	%rbx, %r8
1409ad376: e8 15 cd 0f 00              	callq	0x140aaa090 <.text+0xaa9090>
1409ad37b: 45 89 65 00                 	movl	%r12d, (%r13)
1409ad37f: 48 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %rdx
1409ad386: f3 0f 6f 85 c0 01 00 00     	movdqu	0x1c0(%rbp), %xmm0
1409ad38e: f3 0f 7f 45 74              	movdqu	%xmm0, 0x74(%rbp)
1409ad393: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
1409ad39a: 4c 8d 45 70                 	leaq	0x70(%rbp), %r8
1409ad39e: 48 89 f1                    	movq	%rsi, %rcx
1409ad3a1: e8 6a c2 0f 00              	callq	0x140aa9610 <.text+0xaa8610>
1409ad3a6: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
1409ad3ad: 48 89 75 70                 	movq	%rsi, 0x70(%rbp)
1409ad3b1: 49 83 7e 48 00              	cmpq	$0x0, 0x48(%r14)
1409ad3b6: 0f 84 15 03 00 00           	je	0x1409ad6d1 <.text+0x9ac6d1>
1409ad3bc: 49 8d 4e 50                 	leaq	0x50(%r14), %rcx
1409ad3c0: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
1409ad3c4: e8 a7 39 c0 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1409ad3c9: 49 8b 4e 30                 	movq	0x30(%r14), %rcx
1409ad3cd: 49 8b 56 38                 	movq	0x38(%r14), %rdx
1409ad3d1: 49 89 d0                    	movq	%rdx, %r8
1409ad3d4: 49 21 c0                    	andq	%rax, %r8
1409ad3d7: 48 c1 e8 39                 	shrq	$0x39, %rax
1409ad3db: 66 0f 6e c0                 	movd	%eax, %xmm0
1409ad3df: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409ad3e3: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409ad3e8: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409ad3ed: 48 8d 41 f0                 	leaq	-0x10(%rcx), %rax
1409ad3f1: 45 31 c9                    	xorl	%r9d, %r9d
1409ad3f4: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1409ad3f8: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1409ad3fe: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1409ad402: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1409ad406: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1409ad40b: 45 85 d2                    	testl	%r10d, %r10d
1409ad40e: 4c 8b ad 68 03 00 00        	movq	0x368(%rbp), %r13
1409ad415: 74 2e                       	je	0x1409ad445 <.text+0x9ac445>
1409ad417: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1409ad41c: 4d 01 c3                    	addq	%r8, %r11
1409ad41f: 49 21 d3                    	andq	%rdx, %r11
1409ad422: 4c 89 df                    	movq	%r11, %rdi
1409ad425: 48 c1 e7 04                 	shlq	$0x4, %rdi
1409ad429: 48 89 c3                    	movq	%rax, %rbx
1409ad42c: 48 29 fb                    	subq	%rdi, %rbx
1409ad42f: 48 39 33                    	cmpq	%rsi, (%rbx)
1409ad432: 0f 84 a2 02 00 00           	je	0x1409ad6da <.text+0x9ac6da>
1409ad438: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1409ad43c: 66 45 21 d3                 	andw	%r10w, %r11w
1409ad440: 45 89 da                    	movl	%r11d, %r10d
1409ad443: 75 d2                       	jne	0x1409ad417 <.text+0x9ac417>
1409ad445: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1409ad449: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1409ad44e: 45 85 d2                    	testl	%r10d, %r10d
1409ad451: 0f 85 7a 02 00 00           	jne	0x1409ad6d1 <.text+0x9ac6d1>
1409ad457: 4d 01 c8                    	addq	%r9, %r8
1409ad45a: 49 83 c0 10                 	addq	$0x10, %r8
1409ad45e: 49 83 c1 10                 	addq	$0x10, %r9
1409ad462: 49 21 d0                    	andq	%rdx, %r8
1409ad465: eb 91                       	jmp	0x1409ad3f8 <.text+0x9ac3f8>
1409ad467: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
1409ad46e: 48 8b 95 38 03 00 00        	movq	0x338(%rbp), %rdx
1409ad475: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409ad478: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409ad47c: 89 01                       	movl	%eax, (%rcx)
1409ad47e: 48 8b 3a                    	movq	(%rdx), %rdi
1409ad481: 48 8b 5f 30                 	movq	0x30(%rdi), %rbx
1409ad485: 4c 8b 77 40                 	movq	0x40(%rdi), %r14
1409ad489: 48 3b 5f 20                 	cmpq	0x20(%rdi), %rbx
1409ad48d: 75 10                       	jne	0x1409ad49f <.text+0x9ac49f>
1409ad48f: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
1409ad493: 48 8d 15 ae cc 28 02        	leaq	0x228ccae(%rip), %rdx   # 0x142c3a148
1409ad49a: e8 91 51 bb ff              	callq	0x140562630 <.text+0x561630>
1409ad49f: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1409ad4a3: 48 89 d9                    	movq	%rbx, %rcx
1409ad4a6: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409ad4aa: 4c 89 34 08                 	movq	%r14, (%rax,%rcx)
1409ad4ae: 48 89 74 08 08              	movq	%rsi, 0x8(%rax,%rcx)
1409ad4b3: 48 ff c3                    	incq	%rbx
1409ad4b6: 48 89 5f 30                 	movq	%rbx, 0x30(%rdi)
1409ad4ba: 48 ff 47 40                 	incq	0x40(%rdi)
1409ad4be: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
1409ad4c6: 48 c7 45 18 08 00 00 00     	movq	$0x8, 0x18(%rbp)
1409ad4ce: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
1409ad4d6: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
1409ad4dd: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
1409ad4e4: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1409ad4eb: 48 8d 8d cc 01 00 00        	leaq	0x1cc(%rbp), %rcx
1409ad4f2: f3 41 0f 10 4d 08           	movss	0x8(%r13), %xmm1
1409ad4f8: c6 85 07 02 00 00 01        	movb	$0x1, 0x207(%rbp)
1409ad4ff: e8 7c 5c d4 00              	callq	0x1416f3180 <.text+0x16f2180>
1409ad504: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1409ad50b: 0f 11 85 84 00 00 00        	movups	%xmm0, 0x84(%rbp)
1409ad512: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
1409ad519: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
1409ad520: 48 89 75 78                 	movq	%rsi, 0x78(%rbp)
1409ad524: c7 85 80 00 00 00 00 00 00 00       	movl	$0x0, 0x80(%rbp)
1409ad52e: c7 85 9c 00 00 00 00 00 00 00       	movl	$0x0, 0x9c(%rbp)
1409ad538: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409ad542: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1409ad546: c6 85 07 02 00 00 01        	movb	$0x1, 0x207(%rbp)
1409ad54d: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1409ad551: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
1409ad555: e8 e6 55 0d 00              	callq	0x140a82b40 <.text+0xa81b40>
1409ad55a: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
1409ad561: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409ad564: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409ad568: 89 01                       	movl	%eax, (%rcx)
1409ad56a: 48 8b 3a                    	movq	(%rdx), %rdi
1409ad56d: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1409ad571: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1409ad578: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1409ad57c: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
1409ad583: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
1409ad58a: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
1409ad595: 48 c7 85 c8 01 00 00 08 00 00 00    	movq	$0x8, 0x1c8(%rbp)
1409ad5a0: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
1409ad5ab: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1409ad5af: 48 8d 04 c2                 	leaq	(%rdx,%rax,8), %rax
1409ad5b3: 48 89 55 70                 	movq	%rdx, 0x70(%rbp)
1409ad5b7: 48 89 55 78                 	movq	%rdx, 0x78(%rbp)
1409ad5bb: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
1409ad5c2: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
1409ad5c9: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409ad5cd: e8 7e 99 ce 00              	callq	0x141696f50 <.text+0x1695f50>
1409ad5d2: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1409ad5d6: 48 89 95 80 00 00 00        	movq	%rdx, 0x80(%rbp)
1409ad5dd: 48 c7 45 70 07 00 00 00     	movq	$0x7, 0x70(%rbp)
1409ad5e5: c6 85 07 02 00 00 00        	movb	$0x0, 0x207(%rbp)
1409ad5ec: 4c 8d 05 55 07 11 02        	leaq	0x2110755(%rip), %r8    # 0x142abdd48
1409ad5f3: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
1409ad5f7: 48 89 f9                    	movq	%rdi, %rcx
1409ad5fa: e8 c1 78 0d 00              	callq	0x140a84ec0 <.text+0xa83ec0>
1409ad5ff: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
1409ad606: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409ad609: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409ad60d: 89 01                       	movl	%eax, (%rcx)
1409ad60f: 48 8b 32                    	movq	(%rdx), %rsi
1409ad612: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
1409ad616: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
1409ad61a: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
1409ad61e: 75 17                       	jne	0x1409ad637 <.text+0x9ac637>
1409ad620: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1409ad624: c6 85 07 02 00 00 00        	movb	$0x0, 0x207(%rbp)
1409ad62b: 48 8d 15 16 cb 28 02        	leaq	0x228cb16(%rip), %rdx   # 0x142c3a148
1409ad632: e8 39 52 bb ff              	callq	0x140562870 <.text+0x561870>
1409ad637: 48 8b 46 28                 	movq	0x28(%rsi), %rax
1409ad63b: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
1409ad63f: 48 ff c7                    	incq	%rdi
1409ad642: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
1409ad646: 48 ff 46 40                 	incq	0x40(%rsi)
1409ad64a: 41 c6 45 18 01              	movb	$0x1, 0x18(%r13)
1409ad64f: f3 45 0f 5c c6              	subss	%xmm14, %xmm8
1409ad654: f3 41 0f 16 c6              	movshdup	%xmm14, %xmm0           # xmm0 = xmm14[1,1,3,3]
1409ad659: f3 44 0f 5c c8              	subss	%xmm0, %xmm9
1409ad65e: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1409ad662: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1409ad666: e8 b5 37 2e 00              	callq	0x140c90e20 <.text+0xc8fe20>
1409ad66b: 0f 28 c1                    	movaps	%xmm1, %xmm0
1409ad66e: e8 5d 5b d4 00              	callq	0x1416f31d0 <.text+0x16f21d0>
1409ad673: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1409ad677: 48 8b b5 70 03 00 00        	movq	0x370(%rbp), %rsi
1409ad67e: f3 44 0f 10 4e 04           	movss	0x4(%rsi), %xmm9
1409ad684: f6 06 01                    	testb	$0x1, (%rsi)
1409ad687: 0f 84 b9 02 00 00           	je	0x1409ad946 <.text+0x9ac946>
1409ad68d: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
1409ad694: 48 8b 00                    	movq	(%rax), %rax
1409ad697: f3 0f 10 40 28              	movss	0x28(%rax), %xmm0
1409ad69c: f3 0f 59 05 6c 96 06 02     	mulss	0x206966c(%rip), %xmm0  # 0x142a16d10
1409ad6a4: e8 17 68 f7 01              	callq	0x142923ec0 <.text+0x2922ec0>
1409ad6a9: f3 0f 10 15 53 82 f7 01     	movss	0x1f78253(%rip), %xmm2  # 0x142925904
1409ad6b1: f3 0f 5c d0                 	subss	%xmm0, %xmm2
1409ad6b5: f3 44 0f 11 4d 70           	movss	%xmm9, 0x70(%rbp)
1409ad6bb: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409ad6bf: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1409ad6c3: e8 68 5c d4 00              	callq	0x1416f3330 <.text+0x16f2330>
1409ad6c8: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1409ad6cc: e9 64 02 00 00              	jmp	0x1409ad935 <.text+0x9ac935>
1409ad6d1: 4c 8b ad 68 03 00 00        	movq	0x368(%rbp), %r13
1409ad6d8: eb 44                       	jmp	0x1409ad71e <.text+0x9ac71e>
1409ad6da: 49 f7 db                    	negq	%r11
1409ad6dd: 49 c1 e3 04                 	shlq	$0x4, %r11
1409ad6e1: 4a 8b 44 19 f8              	movq	-0x8(%rcx,%r11), %rax
1409ad6e6: 48 89 c1                    	movq	%rax, %rcx
1409ad6e9: 48 c1 e9 20                 	shrq	$0x20, %rcx
1409ad6ed: 74 2f                       	je	0x1409ad71e <.text+0x9ac71e>
1409ad6ef: 48 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %rdx
1409ad6f6: 48 89 55 78                 	movq	%rdx, 0x78(%rbp)
1409ad6fa: c6 85 80 00 00 00 02        	movb	$0x2, 0x80(%rbp)
1409ad701: 89 45 70                    	movl	%eax, 0x70(%rbp)
1409ad704: 89 4d 74                    	movl	%ecx, 0x74(%rbp)
1409ad707: 4c 8d 05 22 06 11 02        	leaq	0x2110622(%rip), %r8    # 0x142abdd30
1409ad70e: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
1409ad712: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1409ad719: e8 32 a0 97 00              	callq	0x141327750 <.text+0x1326750>
1409ad71e: 0f b6 05 e4 74 c8 02        	movzbl	0x2c874e4(%rip), %eax   # 0x143634c09
1409ad725: b9 18 00 00 00              	movl	$0x18, %ecx
1409ad72a: ba 04 00 00 00              	movl	$0x4, %edx
1409ad72f: e8 dc 64 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409ad734: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1409ad73b: 48 85 c0                    	testq	%rax, %rax
1409ad73e: 0f 84 de 06 00 00           	je	0x1409ade22 <.text+0x9ace22>
1409ad744: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409ad748: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
1409ad74f: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1409ad753: e8 88 f6 2e 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409ad758: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
1409ad75f: 44 0f 13 21                 	movlps	%xmm12, (%rcx)
1409ad763: f3 44 0f 11 59 08           	movss	%xmm11, 0x8(%rcx)
1409ad769: 8b 45 78                    	movl	0x78(%rbp), %eax
1409ad76c: 89 41 14                    	movl	%eax, 0x14(%rcx)
1409ad76f: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1409ad773: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
1409ad777: c7 85 a0 00 00 00 00 00 00 00       	movl	$0x0, 0xa0(%rbp)
1409ad781: c7 85 b0 00 00 00 00 00 80 3f       	movl	$0x3f800000, 0xb0(%rbp) # imm = 0x3F800000
1409ad78b: 48 c7 45 70 02 00 00 00     	movq	$0x2, 0x70(%rbp)
1409ad793: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
1409ad797: 48 c7 85 80 00 00 00 02 00 00 00    	movq	$0x2, 0x80(%rbp)
1409ad7a2: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1409ad7ac: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
1409ad7b3: 66 c7 85 b4 00 00 00 01 00  	movw	$0x1, 0xb4(%rbp)
1409ad7bc: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409ad7c0: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
1409ad7c4: e8 a7 77 15 00              	callq	0x140b04f70 <.text+0xb03f70>
1409ad7c9: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
1409ad7d0: 48 8b 95 38 03 00 00        	movq	0x338(%rbp), %rdx
1409ad7d7: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409ad7da: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409ad7de: 89 01                       	movl	%eax, (%rcx)
1409ad7e0: 48 8b 3a                    	movq	(%rdx), %rdi
1409ad7e3: 48 8b 5f 30                 	movq	0x30(%rdi), %rbx
1409ad7e7: 4c 8b 77 40                 	movq	0x40(%rdi), %r14
1409ad7eb: 48 3b 5f 20                 	cmpq	0x20(%rdi), %rbx
1409ad7ef: 75 10                       	jne	0x1409ad801 <.text+0x9ac801>
1409ad7f1: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
1409ad7f5: 48 8d 15 4c c9 28 02        	leaq	0x228c94c(%rip), %rdx   # 0x142c3a148
1409ad7fc: e8 2f 4e bb ff              	callq	0x140562630 <.text+0x561630>
1409ad801: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1409ad805: 48 89 d9                    	movq	%rbx, %rcx
1409ad808: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409ad80c: 4c 89 34 08                 	movq	%r14, (%rax,%rcx)
1409ad810: 48 89 74 08 08              	movq	%rsi, 0x8(%rax,%rcx)
1409ad815: 48 ff c3                    	incq	%rbx
1409ad818: 48 89 5f 30                 	movq	%rbx, 0x30(%rdi)
1409ad81c: 48 ff 47 40                 	incq	0x40(%rdi)
1409ad820: 41 f6 87 68 fe ff ff 01     	testb	$0x1, -0x198(%r15)
1409ad828: 0f 84 03 06 00 00           	je	0x1409ade31 <.text+0x9ace31>
1409ad82e: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
1409ad835: f3 41 0f 10 77 e0           	movss	-0x20(%r15), %xmm6
1409ad83b: f3 45 0f 10 9f 6c fe ff ff  	movss	-0x194(%r15), %xmm11
1409ad844: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
1409ad84b: 0f 11 45 72                 	movups	%xmm0, 0x72(%rbp)
1409ad84f: 48 8b 95 48 03 00 00        	movq	0x348(%rbp), %rdx
1409ad856: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409ad859: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409ad85d: 89 01                       	movl	%eax, (%rcx)
1409ad85f: 48 8b 3a                    	movq	(%rdx), %rdi
1409ad862: 48 8b 5f 30                 	movq	0x30(%rdi), %rbx
1409ad866: 4c 8b 77 40                 	movq	0x40(%rdi), %r14
1409ad86a: 48 3b 5f 20                 	cmpq	0x20(%rdi), %rbx
1409ad86e: 75 10                       	jne	0x1409ad880 <.text+0x9ac880>
1409ad870: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
1409ad874: 48 8d 15 cd c8 28 02        	leaq	0x228c8cd(%rip), %rdx   # 0x142c3a148
1409ad87b: e8 30 4f bb ff              	callq	0x1405627b0 <.text+0x5617b0>
1409ad880: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1409ad884: 48 89 d9                    	movq	%rbx, %rcx
1409ad887: 48 c1 e1 06                 	shlq	$0x6, %rcx
1409ad88b: 66 c7 04 08 00 01           	movw	$0x100, (%rax,%rcx)     # imm = 0x100
1409ad891: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
1409ad895: 0f 11 44 08 02              	movups	%xmm0, 0x2(%rax,%rcx)
1409ad89a: 0f b7 95 80 00 00 00        	movzwl	0x80(%rbp), %edx
1409ad8a1: 66 89 54 08 12              	movw	%dx, 0x12(%rax,%rcx)
1409ad8a6: f3 0f 11 74 08 14           	movss	%xmm6, 0x14(%rax,%rcx)
1409ad8ac: f3 44 0f 11 5c 08 18        	movss	%xmm11, 0x18(%rax,%rcx)
1409ad8b3: 48 c7 44 08 20 01 00 00 00  	movq	$0x1, 0x20(%rax,%rcx)
1409ad8bc: 48 89 74 08 28              	movq	%rsi, 0x28(%rax,%rcx)
1409ad8c1: 4c 89 74 08 38              	movq	%r14, 0x38(%rax,%rcx)
1409ad8c6: 48 ff c3                    	incq	%rbx
1409ad8c9: 48 89 5f 30                 	movq	%rbx, 0x30(%rdi)
1409ad8cd: 48 ff 47 40                 	incq	0x40(%rdi)
1409ad8d1: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
1409ad8d8: f3 44 0f 5c 40 04           	subss	0x4(%rax), %xmm8
1409ad8de: f3 44 0f 5e 40 10           	divss	0x10(%rax), %xmm8
1409ad8e4: f3 0f 10 40 0c              	movss	0xc(%rax), %xmm0
1409ad8e9: f3 41 0f 59 c0              	mulss	%xmm8, %xmm0
1409ad8ee: f3 44 0f 59 40 14           	mulss	0x14(%rax), %xmm8
1409ad8f4: f3 0f 58 00                 	addss	(%rax), %xmm0
1409ad8f8: f3 44 0f 58 40 08           	addss	0x8(%rax), %xmm8
1409ad8fe: f3 41 0f 5c c6              	subss	%xmm14, %xmm0
1409ad903: f3 45 0f 5c c2              	subss	%xmm10, %xmm8
1409ad908: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1409ad90c: e8 0f 35 2e 00              	callq	0x140c90e20 <.text+0xc8fe20>
1409ad911: f3 44 0f 58 c9              	addss	%xmm1, %xmm9
1409ad916: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1409ad91a: e8 b1 58 d4 00              	callq	0x1416f31d0 <.text+0x16f21d0>
1409ad91f: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1409ad923: 48 8b b5 70 03 00 00        	movq	0x370(%rbp), %rsi
1409ad92a: f3 44 0f 10 4e 04           	movss	0x4(%rsi), %xmm9
1409ad930: 83 3e 00                    	cmpl	$0x0, (%rsi)
1409ad933: 74 11                       	je	0x1409ad946 <.text+0x9ac946>
1409ad935: c7 06 01 00 00 00           	movl	$0x1, (%rsi)
1409ad93b: f3 44 0f 11 46 04           	movss	%xmm8, 0x4(%rsi)
1409ad941: 40 b6 01                    	movb	$0x1, %sil
1409ad944: eb 0e                       	jmp	0x1409ad954 <.text+0x9ac954>
1409ad946: c7 06 01 00 00 00           	movl	$0x1, (%rsi)
1409ad94c: f3 44 0f 11 46 04           	movss	%xmm8, 0x4(%rsi)
1409ad952: 31 f6                       	xorl	%esi, %esi
1409ad954: f3 44 0f 11 45 70           	movss	%xmm8, 0x70(%rbp)
1409ad95a: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409ad95e: e8 ad 58 d4 00              	callq	0x1416f3210 <.text+0x16f2210>
1409ad963: 0f 28 c8                    	movaps	%xmm0, %xmm1
1409ad966: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1409ad96a: e8 01 35 2e 00              	callq	0x140c90e70 <.text+0xc8fe70>
1409ad96f: f3 0f 10 95 b8 01 00 00     	movss	0x1b8(%rbp), %xmm2
1409ad977: f3 41 0f 58 d5              	addss	%xmm13, %xmm2
1409ad97c: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1409ad97f: f3 0f 12 ca                 	movsldup	%xmm2, %xmm1            # xmm1 = xmm2[0,0,2,2]
1409ad983: 0f 59 c8                    	mulps	%xmm0, %xmm1
1409ad986: 41 0f 58 ce                 	addps	%xmm14, %xmm1
1409ad98a: 0f 13 4d 70                 	movlps	%xmm1, 0x70(%rbp)
1409ad98e: 48 8d 8d 94 01 00 00        	leaq	0x194(%rbp), %rcx
1409ad995: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
1409ad999: 0f 28 d7                    	movaps	%xmm7, %xmm2
1409ad99c: e8 3f f4 2e 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409ad9a1: 40 84 f6                    	testb	%sil, %sil
1409ad9a4: 0f 84 2e 01 00 00           	je	0x1409adad8 <.text+0x9acad8>
1409ad9aa: 41 80 7d 18 01              	cmpb	$0x1, 0x18(%r13)
1409ad9af: 0f 85 23 01 00 00           	jne	0x1409adad8 <.text+0x9acad8>
1409ad9b5: f3 44 0f 11 8d c0 01 00 00  	movss	%xmm9, 0x1c0(%rbp)
1409ad9be: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1409ad9c5: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1409ad9c9: e8 92 58 d4 00              	callq	0x1416f3260 <.text+0x16f2260>
1409ad9ce: 0f 2e 05 9f 2c fb 01        	ucomiss	0x1fb2c9f(%rip), %xmm0  # 0x142960674
1409ad9d5: 0f 86 a5 00 00 00           	jbe	0x1409ada80 <.text+0x9aca80>
1409ad9db: 0f 28 d0                    	movaps	%xmm0, %xmm2
1409ad9de: f3 0f 10 05 8a 7f f7 01     	movss	0x1f77f8a(%rip), %xmm0  # 0x142925970
1409ad9e6: 0f 57 ff                    	xorps	%xmm7, %xmm7
1409ad9e9: 0f 57 c9                    	xorps	%xmm1, %xmm1
1409ad9ec: e8 af 2a 2e 00              	callq	0x140c904a0 <.text+0xc8f4a0>
1409ad9f1: f3 0f 5f f8                 	maxss	%xmm0, %xmm7
1409ad9f5: f3 0f 10 35 07 7f f7 01     	movss	0x1f77f07(%rip), %xmm6  # 0x142925904
1409ad9fd: f3 0f 5d f7                 	minss	%xmm7, %xmm6
1409ada01: 48 8b 85 94 01 00 00        	movq	0x194(%rbp), %rax
1409ada08: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1409ada0c: 8b 85 9c 01 00 00           	movl	0x19c(%rbp), %eax
1409ada12: 89 45 78                    	movl	%eax, 0x78(%rbp)
1409ada15: 48 8b 95 60 03 00 00        	movq	0x360(%rbp), %rdx
1409ada1c: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409ada1f: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409ada23: 89 01                       	movl	%eax, (%rcx)
1409ada25: 48 8b 32                    	movq	(%rdx), %rsi
1409ada28: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
1409ada2c: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
1409ada30: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
1409ada34: 75 10                       	jne	0x1409ada46 <.text+0x9aca46>
1409ada36: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1409ada3a: 48 8d 15 07 c7 28 02        	leaq	0x228c707(%rip), %rdx   # 0x142c3a148
1409ada41: e8 ea 51 bb ff              	callq	0x140562c30 <.text+0x561c30>
1409ada46: 48 8b 46 28                 	movq	0x28(%rsi), %rax
1409ada4a: 48 89 f9                    	movq	%rdi, %rcx
1409ada4d: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409ada51: c7 04 08 22 00 00 00        	movl	$0x22, (%rax,%rcx)
1409ada58: f3 0f 11 74 08 04           	movss	%xmm6, 0x4(%rax,%rcx)
1409ada5e: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
1409ada62: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
1409ada67: 8b 55 78                    	movl	0x78(%rbp), %edx
1409ada6a: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
1409ada6e: 48 89 5c 08 18              	movq	%rbx, 0x18(%rax,%rcx)
1409ada73: 48 ff c7                    	incq	%rdi
1409ada76: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
1409ada7a: 48 ff 46 40                 	incq	0x40(%rsi)
1409ada7e: eb 58                       	jmp	0x1409adad8 <.text+0x9acad8>
1409ada80: 48 8b 95 60 03 00 00        	movq	0x360(%rbp), %rdx
1409ada87: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1409ada8a: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1409ada8e: 89 01                       	movl	%eax, (%rcx)
1409ada90: 48 8b 32                    	movq	(%rdx), %rsi
1409ada93: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
1409ada97: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
1409ada9b: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
1409ada9f: 75 10                       	jne	0x1409adab1 <.text+0x9acab1>
1409adaa1: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1409adaa5: 48 8d 15 9c c6 28 02        	leaq	0x228c69c(%rip), %rdx   # 0x142c3a148
1409adaac: e8 7f 51 bb ff              	callq	0x140562c30 <.text+0x561c30>
1409adab1: 48 8b 46 28                 	movq	0x28(%rsi), %rax
1409adab5: 48 89 f9                    	movq	%rdi, %rcx
1409adab8: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409adabc: c7 04 08 23 00 00 00        	movl	$0x23, (%rax,%rcx)
1409adac3: 48 89 5c 08 18              	movq	%rbx, 0x18(%rax,%rcx)
1409adac8: 48 ff c7                    	incq	%rdi
1409adacb: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
1409adacf: 48 ff 46 40                 	incq	0x40(%rsi)
1409adad3: 41 c6 45 18 02              	movb	$0x2, 0x18(%r13)
1409adad8: f3 0f 10 8d 94 01 00 00     	movss	0x194(%rbp), %xmm1
1409adae0: f3 0f 10 85 9c 01 00 00     	movss	0x19c(%rbp), %xmm0
1409adae8: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1409adaeb: 41 0f 5c ce                 	subps	%xmm14, %xmm1
1409adaef: 0f 28 c1                    	movaps	%xmm1, %xmm0
1409adaf2: 0f 59 c1                    	mulps	%xmm1, %xmm0
1409adaf5: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1409adaf9: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1409adafd: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409adb00: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
1409adb04: 41 0f 28 dd                 	movaps	%xmm13, %xmm3
1409adb08: f3 0f 5e d8                 	divss	%xmm0, %xmm3
1409adb0c: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1409adb10: f3 0f 59 cb                 	mulss	%xmm3, %xmm1
1409adb14: f3 0f 59 d3                 	mulss	%xmm3, %xmm2
1409adb18: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409adb1c: e8 af 34 2e 00              	callq	0x140c90fd0 <.text+0xc8ffd0>
1409adb21: f3 0f 10 05 8f 91 06 02     	movss	0x206918f(%rip), %xmm0  # 0x142a16cb8
1409adb29: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1409adb30: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
1409adb34: 74 17                       	je	0x1409adb4d <.text+0x9acb4d>
1409adb36: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1409adb3d: 48 39 48 38                 	cmpq	%rcx, 0x38(%rax)
1409adb41: 75 0a                       	jne	0x1409adb4d <.text+0x9acb4d>
1409adb43: 83 78 28 02                 	cmpl	$0x2, 0x28(%rax)
1409adb47: 74 04                       	je	0x1409adb4d <.text+0x9acb4d>
1409adb49: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1409adb4d: 0f 28 75 70                 	movaps	0x70(%rbp), %xmm6
1409adb51: f3 0f 11 45 f8              	movss	%xmm0, -0x8(%rbp)
1409adb56: c6 45 70 00                 	movb	$0x0, 0x70(%rbp)
1409adb5a: 48 8d 05 ff 01 11 02        	leaq	0x21101ff(%rip), %rax   # 0x142abdd60
1409adb61: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1409adb65: 48 c7 85 80 00 00 00 0c 00 00 00    	movq	$0xc, 0x80(%rbp)
1409adb70: c6 85 88 00 00 00 01        	movb	$0x1, 0x88(%rbp)
1409adb77: 48 8d 05 ee 01 11 02        	leaq	0x21101ee(%rip), %rax   # 0x142abdd6c
1409adb7e: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
1409adb85: 48 c7 85 98 00 00 00 14 00 00 00    	movq	$0x14, 0x98(%rbp)
1409adb90: 48 c7 85 a8 00 00 00 02 00 00 00    	movq	$0x2, 0xa8(%rbp)
1409adb9b: 4c 8d bd c0 01 00 00        	leaq	0x1c0(%rbp), %r15
1409adba2: 48 8d 35 17 02 11 02        	leaq	0x2110217(%rip), %rsi   # 0x142abddc0
1409adba9: 31 c0                       	xorl	%eax, %eax
1409adbab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1409adbb0: 48 83 f8 02                 	cmpq	$0x2, %rax
1409adbb4: 0f 84 15 ef ff ff           	je	0x1409acacf <.text+0x9abacf>
1409adbba: 48 8d 78 01                 	leaq	0x1(%rax), %rdi
1409adbbe: 48 89 bd a0 00 00 00        	movq	%rdi, 0xa0(%rbp)
1409adbc5: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1409adbc9: 0f b6 5c c5 70              	movzbl	0x70(%rbp,%rax,8), %ebx
1409adbce: 80 fb 02                    	cmpb	$0x2, %bl
1409adbd1: 0f 84 f8 ee ff ff           	je	0x1409acacf <.text+0x9abacf>
1409adbd7: 4c 8b 6c c5 78              	movq	0x78(%rbp,%rax,8), %r13
1409adbdc: 4c 8b b4 c5 80 00 00 00     	movq	0x80(%rbp,%rax,8), %r14
1409adbe4: 48 8d 85 f8 01 00 00        	leaq	0x1f8(%rbp), %rax
1409adbeb: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1409adbef: 48 8d 05 1a 0b 00 00        	leaq	0xb1a(%rip), %rax       # 0x1409ae710 <.text+0x9ad710>
1409adbf6: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1409adbfa: 48 8d 85 94 01 00 00        	leaq	0x194(%rbp), %rax
1409adc01: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1409adc05: 48 8d 05 f4 dd 39 00        	leaq	0x39ddf4(%rip), %rax    # 0x140d4ba00 <.text+0xd4aa00>
1409adc0c: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1409adc10: 48 8d 45 f8                 	leaq	-0x8(%rbp), %rax
1409adc14: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
1409adc18: 48 8d 05 f1 ec 3d 00        	leaq	0x3decf1(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
1409adc1f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
1409adc23: 48 8d 05 66 01 11 02        	leaq	0x2110166(%rip), %rax   # 0x142abdd90
1409adc2a: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1409adc31: 48 c7 85 c8 01 00 00 03 00 00 00    	movq	$0x3, 0x1c8(%rbp)
1409adc3c: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
1409adc40: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1409adc47: 48 c7 85 d8 01 00 00 03 00 00 00    	movq	$0x3, 0x1d8(%rbp)
1409adc52: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1409adc5d: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
1409adc61: 4c 89 fa                    	movq	%r15, %rdx
1409adc64: e8 27 c3 3c 00              	callq	0x140d79f90 <.text+0xd78f90>
1409adc69: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
1409adc6d: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1409adc74: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
1409adc78: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1409adc7f: 4c 8b 65 e8                 	movq	-0x18(%rbp), %r12
1409adc83: f3 0f 10 7d f8              	movss	-0x8(%rbp), %xmm7
1409adc88: 48 8b 85 94 01 00 00        	movq	0x194(%rbp), %rax
1409adc8f: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1409adc96: 8b 85 9c 01 00 00           	movl	0x19c(%rbp), %eax
1409adc9c: 89 85 c8 01 00 00           	movl	%eax, 0x1c8(%rbp)
1409adca2: 0f b6 05 60 6f c8 02        	movzbl	0x2c86f60(%rip), %eax   # 0x143634c09
1409adca9: b9 50 00 00 00              	movl	$0x50, %ecx
1409adcae: ba 10 00 00 00              	movl	$0x10, %edx
1409adcb3: e8 58 5f c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409adcb8: 48 85 c0                    	testq	%rax, %rax
1409adcbb: 0f 84 2a 01 00 00           	je	0x1409addeb <.text+0x9acdeb>
1409adcc1: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
1409adcc8: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
1409adcd0: 4c 89 68 10                 	movq	%r13, 0x10(%rax)
1409adcd4: 4c 89 70 18                 	movq	%r14, 0x18(%rax)
1409adcd8: 0f 29 70 20                 	movaps	%xmm6, 0x20(%rax)
1409adcdc: f3 0f 11 78 30              	movss	%xmm7, 0x30(%rax)
1409adce1: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
1409adce8: 48 89 48 34                 	movq	%rcx, 0x34(%rax)
1409adcec: 8b 8d c8 01 00 00           	movl	0x1c8(%rbp), %ecx
1409adcf2: 89 48 3c                    	movl	%ecx, 0x3c(%rax)
1409adcf5: 88 58 40                    	movb	%bl, 0x40(%rax)
1409adcf8: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1409adcff: 48 89 b5 c8 01 00 00        	movq	%rsi, 0x1c8(%rbp)
1409add06: 4d 85 e4                    	testq	%r12, %r12
1409add09: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1409add10: 0f 88 e6 00 00 00           	js	0x1409addfc <.text+0x9acdfc>
1409add16: 74 28                       	je	0x1409add40 <.text+0x9acd40>
1409add18: 0f b6 05 ea 6e c8 02        	movzbl	0x2c86eea(%rip), %eax   # 0x143634c09
1409add1f: bb 01 00 00 00              	movl	$0x1, %ebx
1409add24: ba 01 00 00 00              	movl	$0x1, %edx
1409add29: 4c 89 e1                    	movq	%r12, %rcx
1409add2c: e8 df 5e c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409add31: 48 85 c0                    	testq	%rax, %rax
1409add34: 0f 84 c4 00 00 00           	je	0x1409addfe <.text+0x9acdfe>
1409add3a: 49 89 c5                    	movq	%rax, %r13
1409add3d: eb 07                       	jmp	0x1409add46 <.text+0x9acd46>
1409add3f: 90                          	nop
1409add40: 41 bd 01 00 00 00           	movl	$0x1, %r13d
1409add46: 4c 89 e9                    	movq	%r13, %rcx
1409add49: 48 8b 95 f0 01 00 00        	movq	0x1f0(%rbp), %rdx
1409add50: 4d 89 e0                    	movq	%r12, %r8
1409add53: e8 a8 5e f7 01              	callq	0x142923c00 <.text+0x2922c00>
1409add58: 4c 89 a5 c0 01 00 00        	movq	%r12, 0x1c0(%rbp)
1409add5f: 4c 89 ad c8 01 00 00        	movq	%r13, 0x1c8(%rbp)
1409add66: 4c 89 a5 d0 01 00 00        	movq	%r12, 0x1d0(%rbp)
1409add6d: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1409add74: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
1409add7b: 48 89 b5 e0 01 00 00        	movq	%rsi, 0x1e0(%rbp)
1409add82: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
1409add86: 4c 89 fa                    	movq	%r15, %rdx
1409add89: e8 f2 ea ab 00              	callq	0x14146c880 <.text+0x146b880>
1409add8e: 48 89 f8                    	movq	%rdi, %rax
1409add91: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1409add98: 48 85 d2                    	testq	%rdx, %rdx
1409add9b: 0f 84 0f fe ff ff           	je	0x1409adbb0 <.text+0x9acbb0>
1409adda1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409adda7: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
1409addae: e8 6d 5e c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409addb3: 48 89 f8                    	movq	%rdi, %rax
1409addb6: e9 f5 fd ff ff              	jmp	0x1409adbb0 <.text+0x9acbb0>
1409addbb: 48 8d 0d d1 c3 12 02        	leaq	0x212c3d1(%rip), %rcx   # 0x142ada193
1409addc2: 4c 8d 05 bf fe 10 02        	leaq	0x210febf(%rip), %r8    # 0x142abdc88
1409addc9: ba 32 00 00 00              	movl	$0x32, %edx
1409addce: e8 5d b6 f2 01              	callq	0x1428d9430 <.text+0x28d8430>
1409addd3: 48 8d 0d b9 c3 12 02        	leaq	0x212c3b9(%rip), %rcx   # 0x142ada193
1409addda: 4c 8d 05 d7 fe 10 02        	leaq	0x210fed7(%rip), %r8    # 0x142abdcb8
1409adde1: ba 32 00 00 00              	movl	$0x32, %edx
1409adde6: e8 45 b6 f2 01              	callq	0x1428d9430 <.text+0x28d8430>
1409addeb: b9 10 00 00 00              	movl	$0x10, %ecx
1409addf0: ba 50 00 00 00              	movl	$0x50, %edx
1409addf5: e8 06 b2 f2 01              	callq	0x1428d9000 <.text+0x28d8000>
1409addfa: eb 14                       	jmp	0x1409ade10 <.text+0x9ace10>
1409addfc: 31 db                       	xorl	%ebx, %ebx
1409addfe: 4c 8d 05 1b b1 0e 02        	leaq	0x20eb11b(%rip), %r8    # 0x142a98f20
1409ade05: 48 89 d9                    	movq	%rbx, %rcx
1409ade08: 4c 89 e2                    	movq	%r12, %rdx
1409ade0b: e8 d3 b1 f2 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409ade10: 0f 0b                       	ud2
1409ade12: 4c 8d 05 cf fe 10 02        	leaq	0x210fecf(%rip), %r8    # 0x142abdce8
1409ade19: 31 c9                       	xorl	%ecx, %ecx
1409ade1b: 31 d2                       	xorl	%edx, %edx
1409ade1d: e8 f6 b6 f2 01              	callq	0x1428d9518 <.text+0x28d8518>
1409ade22: b9 04 00 00 00              	movl	$0x4, %ecx
1409ade27: ba 18 00 00 00              	movl	$0x18, %edx
1409ade2c: e8 cf b1 f2 01              	callq	0x1428d9000 <.text+0x28d8000>
1409ade31: 48 8d 05 68 d0 13 02        	leaq	0x213d068(%rip), %rax   # 0x142aeaea0
1409ade38: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1409ade3c: 48 c7 45 78 01 00 00 00     	movq	$0x1, 0x78(%rbp)
1409ade44: 48 c7 85 80 00 00 00 08 00 00 00    	movq	$0x8, 0x80(%rbp)
1409ade4f: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409ade52: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
1409ade59: 48 8d 15 50 d0 13 02        	leaq	0x213d050(%rip), %rdx   # 0x142aeaeb0
1409ade60: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409ade64: e8 27 b5 f2 01              	callq	0x1428d9390 <.text+0x28d8390>
1409ade69: cc                          	int3
1409ade6a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1409ade70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ade75: 55                          	pushq	%rbp
1409ade76: 41 57                       	pushq	%r15
1409ade78: 41 56                       	pushq	%r14
1409ade7a: 41 55                       	pushq	%r13
1409ade7c: 41 54                       	pushq	%r12
1409ade7e: 56                          	pushq	%rsi
1409ade7f: 57                          	pushq	%rdi
1409ade80: 53                          	pushq	%rbx
1409ade81: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409ade88: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ade8f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409ade95: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409ade9b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409adea1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409adea7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409adead: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409adeb6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409adebf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409adec8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409aded0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409aded8: ba 18 00 00 00              	movl	$0x18, %edx
1409adedd: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409adee3: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
1409adeea: e8 31 5d c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409adeef: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409adef7: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409adeff: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409adf08: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409adf11: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409adf1a: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409adf20: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409adf26: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409adf2c: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409adf32: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409adf38: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409adf3f: 5b                          	popq	%rbx
1409adf40: 5f                          	popq	%rdi
1409adf41: 5e                          	popq	%rsi
1409adf42: 41 5c                       	popq	%r12
1409adf44: 41 5d                       	popq	%r13
1409adf46: 41 5e                       	popq	%r14
1409adf48: 41 5f                       	popq	%r15
1409adf4a: 5d                          	popq	%rbp
1409adf4b: c3                          	retq
1409adf4c: 0f 1f 40 00                 	nopl	(%rax)
1409adf50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409adf55: 55                          	pushq	%rbp
1409adf56: 41 57                       	pushq	%r15
1409adf58: 41 56                       	pushq	%r14
1409adf5a: 41 55                       	pushq	%r13
1409adf5c: 41 54                       	pushq	%r12
1409adf5e: 56                          	pushq	%rsi
1409adf5f: 57                          	pushq	%rdi
1409adf60: 53                          	pushq	%rbx
1409adf61: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409adf68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409adf6f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409adf75: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409adf7b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409adf81: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409adf87: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409adf8d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409adf96: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409adf9f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409adfa8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409adfb0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409adfb8: 48 83 bd 70 01 00 00 00     	cmpq	$0x0, 0x170(%rbp)
1409adfc0: 74 19                       	je	0x1409adfdb <.text+0x9acfdb>
1409adfc2: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409adfc8: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
1409adfcf: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1409adfd6: e8 45 5c c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409adfdb: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409adfe3: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409adfeb: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409adff4: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409adffd: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409ae006: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409ae00c: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409ae012: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409ae018: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409ae01e: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409ae024: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409ae02b: 5b                          	popq	%rbx
1409ae02c: 5f                          	popq	%rdi
1409ae02d: 5e                          	popq	%rsi
1409ae02e: 41 5c                       	popq	%r12
1409ae030: 41 5d                       	popq	%r13
1409ae032: 41 5e                       	popq	%r14
1409ae034: 41 5f                       	popq	%r15
1409ae036: 5d                          	popq	%rbp
1409ae037: c3                          	retq
1409ae038: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1409ae040: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ae045: 55                          	pushq	%rbp
1409ae046: 41 57                       	pushq	%r15
1409ae048: 41 56                       	pushq	%r14
1409ae04a: 41 55                       	pushq	%r13
1409ae04c: 41 54                       	pushq	%r12
1409ae04e: 56                          	pushq	%rsi
1409ae04f: 57                          	pushq	%rdi
1409ae050: 53                          	pushq	%rbx
1409ae051: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409ae058: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ae05f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409ae065: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409ae06b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409ae071: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409ae077: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409ae07d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409ae086: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409ae08f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409ae098: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409ae0a0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409ae0a8: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1409ae0af: f0                          	lock
1409ae0b0: 48 ff 08                    	decq	(%rax)
1409ae0b3: 75 0c                       	jne	0x1409ae0c1 <.text+0x9ad0c1>
1409ae0b5: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1409ae0bc: e8 0f 19 ca ff              	callq	0x14064f9d0 <.text+0x64e9d0>
1409ae0c1: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409ae0c9: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409ae0d1: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409ae0da: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409ae0e3: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409ae0ec: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409ae0f2: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409ae0f8: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409ae0fe: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409ae104: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409ae10a: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409ae111: 5b                          	popq	%rbx
1409ae112: 5f                          	popq	%rdi
1409ae113: 5e                          	popq	%rsi
1409ae114: 41 5c                       	popq	%r12
1409ae116: 41 5d                       	popq	%r13
1409ae118: 41 5e                       	popq	%r14
1409ae11a: 41 5f                       	popq	%r15
1409ae11c: 5d                          	popq	%rbp
1409ae11d: c3                          	retq
1409ae11e: 66 90                       	nop
1409ae120: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ae125: 55                          	pushq	%rbp
1409ae126: 41 57                       	pushq	%r15
1409ae128: 41 56                       	pushq	%r14
1409ae12a: 41 55                       	pushq	%r13
1409ae12c: 41 54                       	pushq	%r12
1409ae12e: 56                          	pushq	%rsi
1409ae12f: 57                          	pushq	%rdi
1409ae130: 53                          	pushq	%rbx
1409ae131: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409ae138: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ae13f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409ae145: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409ae14b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409ae151: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409ae157: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409ae15d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409ae166: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409ae16f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409ae178: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409ae180: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409ae188: 80 bd 07 02 00 00 00        	cmpb	$0x0, 0x207(%rbp)
1409ae18f: 75 5d                       	jne	0x1409ae1ee <.text+0x9ad1ee>
1409ae191: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409ae199: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409ae1a1: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409ae1aa: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409ae1b3: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409ae1bc: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409ae1c2: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409ae1c8: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409ae1ce: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409ae1d4: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409ae1da: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409ae1e1: 5b                          	popq	%rbx
1409ae1e2: 5f                          	popq	%rdi
1409ae1e3: 5e                          	popq	%rsi
1409ae1e4: 41 5c                       	popq	%r12
1409ae1e6: 41 5d                       	popq	%r13
1409ae1e8: 41 5e                       	popq	%r14
1409ae1ea: 41 5f                       	popq	%r15
1409ae1ec: 5d                          	popq	%rbp
1409ae1ed: c3                          	retq
1409ae1ee: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1409ae1f2: e8 79 a2 ee ff              	callq	0x140898470 <.text+0x897470>
1409ae1f7: eb 98                       	jmp	0x1409ae191 <.text+0x9ad191>
1409ae1f9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1409ae200: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ae205: 55                          	pushq	%rbp
1409ae206: 41 57                       	pushq	%r15
1409ae208: 41 56                       	pushq	%r14
1409ae20a: 41 55                       	pushq	%r13
1409ae20c: 41 54                       	pushq	%r12
1409ae20e: 56                          	pushq	%rsi
1409ae20f: 57                          	pushq	%rdi
1409ae210: 53                          	pushq	%rbx
1409ae211: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409ae218: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ae21f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409ae225: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409ae22b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409ae231: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409ae237: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409ae23d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409ae246: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409ae24f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409ae258: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409ae260: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409ae268: 80 bd 06 02 00 00 00        	cmpb	$0x0, 0x206(%rbp)
1409ae26f: 74 20                       	je	0x1409ae291 <.text+0x9ad291>
1409ae271: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1409ae275: 48 85 c0                    	testq	%rax, %rax
1409ae278: 74 17                       	je	0x1409ae291 <.text+0x9ad291>
1409ae27a: 48 8b 4d 00                 	movq	(%rbp), %rcx
1409ae27e: 48 c1 e0 02                 	shlq	$0x2, %rax
1409ae282: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1409ae286: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409ae28c: e8 8f 59 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409ae291: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409ae299: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409ae2a1: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409ae2aa: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409ae2b3: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409ae2bc: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409ae2c2: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409ae2c8: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409ae2ce: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409ae2d4: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409ae2da: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409ae2e1: 5b                          	popq	%rbx
1409ae2e2: 5f                          	popq	%rdi
1409ae2e3: 5e                          	popq	%rsi
1409ae2e4: 41 5c                       	popq	%r12
1409ae2e6: 41 5d                       	popq	%r13
1409ae2e8: 41 5e                       	popq	%r14
1409ae2ea: 41 5f                       	popq	%r15
1409ae2ec: 5d                          	popq	%rbp
1409ae2ed: c3                          	retq
1409ae2ee: 66 90                       	nop
1409ae2f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ae2f5: 55                          	pushq	%rbp
1409ae2f6: 41 57                       	pushq	%r15
1409ae2f8: 41 56                       	pushq	%r14
1409ae2fa: 41 55                       	pushq	%r13
1409ae2fc: 41 54                       	pushq	%r12
1409ae2fe: 56                          	pushq	%rsi
1409ae2ff: 57                          	pushq	%rdi
1409ae300: 53                          	pushq	%rbx
1409ae301: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409ae308: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ae30f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409ae315: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409ae31b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409ae321: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409ae327: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409ae32d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409ae336: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409ae33f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409ae348: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409ae350: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409ae358: c6 85 07 02 00 00 00        	movb	$0x0, 0x207(%rbp)
1409ae35f: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1409ae366: e8 05 a1 ee ff              	callq	0x140898470 <.text+0x897470>
1409ae36b: c6 85 07 02 00 00 00        	movb	$0x0, 0x207(%rbp)
1409ae372: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409ae37a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409ae382: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409ae38b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409ae394: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409ae39d: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409ae3a3: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409ae3a9: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409ae3af: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409ae3b5: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409ae3bb: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409ae3c2: 5b                          	popq	%rbx
1409ae3c3: 5f                          	popq	%rdi
1409ae3c4: 5e                          	popq	%rsi
1409ae3c5: 41 5c                       	popq	%r12
1409ae3c7: 41 5d                       	popq	%r13
1409ae3c9: 41 5e                       	popq	%r14
1409ae3cb: 41 5f                       	popq	%r15
1409ae3cd: 5d                          	popq	%rbp
1409ae3ce: c3                          	retq
