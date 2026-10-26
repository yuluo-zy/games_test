
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403bad37: 4c 8b 2d ea 12 27 03        	movq	0x32712ea(%rip), %r13   # 0x14362c028
1403bad3e: 41 c6 45 00 00              	movb	$0x0, (%r13)
1403bad43: 48 8b 85 48 03 00 00        	movq	0x348(%rbp), %rax
1403bad4a: 83 38 13                    	cmpl	$0x13, (%rax)
1403bad4d: 0f 85 b6 00 00 00           	jne	0x1403bae09 <.text+0x3b9e09>
1403bad53: 48 8b 85 20 06 00 00        	movq	0x620(%rbp), %rax
1403bad5a: 48 8b 30                    	movq	(%rax), %rsi
1403bad5d: 48 89 f1                    	movq	%rsi, %rcx
1403bad60: e8 cb 64 55 00              	callq	0x140911230 <.text+0x910230>
1403bad65: 3c 03                       	cmpb	$0x3, %al
1403bad67: 0f 85 9c 00 00 00           	jne	0x1403bae09 <.text+0x3b9e09>
1403bad6d: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403bad74: 48 8b 00                    	movq	(%rax), %rax
1403bad77: 48 f7 d8                    	negq	%rax
1403bad7a: 0f 81 89 00 00 00           	jno	0x1403bae09 <.text+0x3b9e09>
1403bad80: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
1403bad87: 48 89 f2                    	movq	%rsi, %rdx
1403bad8a: e8 41 70 55 00              	callq	0x140911dd0 <.text+0x910dd0>
1403bad8f: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bad96: 48 89 f2                    	movq	%rsi, %rdx
1403bad99: e8 52 70 55 00              	callq	0x140911df0 <.text+0x910df0>
1403bad9e: 8b 85 38 05 00 00           	movl	0x538(%rbp), %eax
1403bada4: 89 85 b8 00 00 00           	movl	%eax, 0xb8(%rbp)
1403badaa: 48 8b 85 30 05 00 00        	movq	0x530(%rbp), %rax
1403badb1: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1403badb8: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
1403badbf: 48 89 85 bc 00 00 00        	movq	%rax, 0xbc(%rbp)
1403badc6: 8b 85 88 03 00 00           	movl	0x388(%rbp), %eax
1403badcc: 89 85 c4 00 00 00           	movl	%eax, 0xc4(%rbp)
1403badd2: c7 44 24 20 01 01 00 00     	movl	$0x101, 0x20(%rsp)      # imm = 0x101
1403badda: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bade1: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
1403bade5: 48 8d bd b0 00 00 00        	leaq	0xb0(%rbp), %rdi
1403badec: f3 0f 10 1d a0 98 57 02     	movss	0x25798a0(%rip), %xmm3  # 0x142934694
1403badf4: 49 89 f8                    	movq	%rdi, %r8
1403badf7: e8 e4 14 51 00              	callq	0x1408cc2e0 <.text+0x8cb2e0>
1403badfc: 83 bd b4 03 00 00 02        	cmpl	$0x2, 0x3b4(%rbp)
1403bae03: 0f 85 b7 00 00 00           	jne	0x1403baec0 <.text+0x3b9ec0>
1403bae09: 48 83 bd e8 05 00 00 00     	cmpq	$0x0, 0x5e8(%rbp)
1403bae11: 74 4b                       	je	0x1403bae5e <.text+0x3b9e5e>
1403bae13: e8 18 d3 85 00              	callq	0x140c18130 <.text+0xc17130>
1403bae18: 48 89 c1                    	movq	%rax, %rcx
1403bae1b: 48 8b 00                    	movq	(%rax), %rax
1403bae1e: 48 83 f8 01                 	cmpq	$0x1, %rax
1403bae22: 0f 85 f4 01 00 00           	jne	0x1403bb01c <.text+0x3ba01c>
1403bae28: 48 83 c1 08                 	addq	$0x8, %rcx
1403bae2c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1403bae30: 0f 85 88 11 00 00           	jne	0x1403bbfbe <.text+0x3bafbe>
1403bae36: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1403bae3d: 48 89 8d 88 07 00 00        	movq	%rcx, 0x788(%rbp)
1403bae44: 48 83 c1 08                 	addq	$0x8, %rcx
1403bae48: 48 8b 95 f0 05 00 00        	movq	0x5f0(%rbp), %rdx
1403bae4f: e8 9c c2 85 00              	callq	0x140c170f0 <.text+0xc160f0>
1403bae54: 48 8b 85 88 07 00 00        	movq	0x788(%rbp), %rax
1403bae5b: 48 ff 00                    	incq	(%rax)
1403bae5e: 0f 28 b5 a0 07 00 00        	movaps	0x7a0(%rbp), %xmm6
1403bae65: 0f 28 bd b0 07 00 00        	movaps	0x7b0(%rbp), %xmm7
1403bae6c: 44 0f 28 85 c0 07 00 00     	movaps	0x7c0(%rbp), %xmm8
1403bae74: 44 0f 28 8d d0 07 00 00     	movaps	0x7d0(%rbp), %xmm9
1403bae7c: 44 0f 28 95 e0 07 00 00     	movaps	0x7e0(%rbp), %xmm10
1403bae84: 44 0f 28 9d f0 07 00 00     	movaps	0x7f0(%rbp), %xmm11
1403bae8c: 44 0f 28 a5 00 08 00 00     	movaps	0x800(%rbp), %xmm12
1403bae94: 44 0f 28 ad 10 08 00 00     	movaps	0x810(%rbp), %xmm13
1403bae9c: 44 0f 28 b5 20 08 00 00     	movaps	0x820(%rbp), %xmm14
1403baea4: 44 0f 28 bd 30 08 00 00     	movaps	0x830(%rbp), %xmm15
1403baeac: 48 81 c4 c8 08 00 00        	addq	$0x8c8, %rsp            # imm = 0x8C8
1403baeb3: 5b                          	popq	%rbx
1403baeb4: 5f                          	popq	%rdi
1403baeb5: 5e                          	popq	%rsi
1403baeb6: 41 5c                       	popq	%r12
1403baeb8: 41 5d                       	popq	%r13
1403baeba: 41 5e                       	popq	%r14
1403baebc: 41 5f                       	popq	%r15
1403baebe: 5d                          	popq	%rbp
1403baebf: c3                          	retq
1403baec0: 80 bd 80 03 00 00 04        	cmpb	$0x4, 0x380(%rbp)
1403baec7: 0f 85 3c ff ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403baecd: 48 83 bd 88 03 00 00 02     	cmpq	$0x2, 0x388(%rbp)
1403baed5: 0f 85 2e ff ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403baedb: 44 8b b5 84 03 00 00        	movl	0x384(%rbp), %r14d
1403baee2: 44 89 b5 30 05 00 00        	movl	%r14d, 0x530(%rbp)
1403baee9: 48 8b 8d 60 03 00 00        	movq	0x360(%rbp), %rcx
1403baef0: 44 89 f2                    	movl	%r14d, %edx
1403baef3: e8 38 1a 66 00              	callq	0x140a1c930 <.text+0xa1b930>
1403baef8: 48 89 c3                    	movq	%rax, %rbx
1403baefb: 48 85 c0                    	testq	%rax, %rax
1403baefe: 0f 84 05 ff ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403baf04: 48 8d 15 dd 2b 61 02        	leaq	0x2612bdd(%rip), %rdx   # 0x1429cdae8
1403baf0b: 48 89 d9                    	movq	%rbx, %rcx
1403baf0e: e8 5d 85 47 00              	callq	0x140833470 <.text+0x832470>
1403baf13: 0f b6 5b 46                 	movzbl	0x46(%rbx), %ebx
1403baf17: 8d 4b c8                    	leal	-0x38(%rbx), %ecx
1403baf1a: 80 f9 08                    	cmpb	$0x8, %cl
1403baf1d: 0f 92 c2                    	setb	%dl
1403baf20: 80 f9 06                    	cmpb	$0x6, %cl
1403baf23: 0f 95 c1                    	setne	%cl
1403baf26: 84 ca                       	testb	%cl, %dl
1403baf28: 0f 85 db fe ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403baf2e: 0f b6 cb                    	movzbl	%bl, %ecx
1403baf31: 83 f9 36                    	cmpl	$0x36, %ecx
1403baf34: 0f 87 cf fe ff ff           	ja	0x1403bae09 <.text+0x3b9e09>
1403baf3a: 89 c9                       	movl	%ecx, %ecx
1403baf3c: 48 ba 00 00 80 01 00 01 40 00       	movabsq	$0x40010001800000, %rdx # imm = 0x40010001800000
1403baf46: 48 0f a3 ca                 	btq	%rcx, %rdx
1403baf4a: 0f 83 b9 fe ff ff           	jae	0x1403bae09 <.text+0x3b9e09>
1403baf50: 44 8b 38                    	movl	(%rax), %r15d
1403baf53: 8b 40 04                    	movl	0x4(%rax), %eax
1403baf56: 89 85 88 07 00 00           	movl	%eax, 0x788(%rbp)
1403baf5c: 48 8b 85 28 06 00 00        	movq	0x628(%rbp), %rax
1403baf63: 48 8d 48 18                 	leaq	0x18(%rax), %rcx
1403baf67: e8 14 2e 2c 00              	callq	0x14067dd80 <.text+0x67cd80>
1403baf6c: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
1403baf71: 0f 84 92 fe ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403baf77: 48 89 c1                    	movq	%rax, %rcx
1403baf7a: 48 83 c1 20                 	addq	$0x20, %rcx
1403baf7e: 48 8d 95 30 05 00 00        	leaq	0x530(%rbp), %rdx
1403baf85: 49 89 c4                    	movq	%rax, %r12
1403baf88: e8 43 5f 1f 00              	callq	0x1405b0ed0 <.text+0x5afed0>
1403baf8d: 49 8b 0c 24                 	movq	(%r12), %rcx
1403baf91: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
1403baf96: 49 89 d0                    	movq	%rdx, %r8
1403baf99: 49 21 c0                    	andq	%rax, %r8
1403baf9c: 48 c1 e8 39                 	shrq	$0x39, %rax
1403bafa0: 66 0f 6e c0                 	movd	%eax, %xmm0
1403bafa4: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1403bafa8: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1403bafad: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1403bafb2: 48 8d 41 d8                 	leaq	-0x28(%rcx), %rax
1403bafb6: 45 31 c9                    	xorl	%r9d, %r9d
1403bafb9: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1403bafbd: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1403bafc3: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1403bafc7: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1403bafcb: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1403bafd0: 45 85 d2                    	testl	%r10d, %r10d
1403bafd3: 74 25                       	je	0x1403baffa <.text+0x3b9ffa>
1403bafd5: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1403bafda: 4d 01 c3                    	addq	%r8, %r11
1403bafdd: 49 21 d3                    	andq	%rdx, %r11
1403bafe0: 49 f7 db                    	negq	%r11
1403bafe3: 4f 8d 1c 9b                 	leaq	(%r11,%r11,4), %r11
1403bafe7: 46 3b 34 d8                 	cmpl	(%rax,%r11,8), %r14d
1403bafeb: 74 6d                       	je	0x1403bb05a <.text+0x3ba05a>
1403bafed: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1403baff1: 66 45 21 d3                 	andw	%r10w, %r11w
1403baff5: 45 89 da                    	movl	%r11d, %r10d
1403baff8: 75 db                       	jne	0x1403bafd5 <.text+0x3b9fd5>
1403baffa: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1403baffe: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1403bb003: 45 85 d2                    	testl	%r10d, %r10d
1403bb006: 0f 85 fd fd ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403bb00c: 4d 01 c8                    	addq	%r9, %r8
1403bb00f: 49 83 c0 10                 	addq	$0x10, %r8
1403bb013: 49 83 c1 10                 	addq	$0x10, %r9
1403bb017: 49 21 d0                    	andq	%rdx, %r8
1403bb01a: eb a1                       	jmp	0x1403bafbd <.text+0x3b9fbd>
1403bb01c: 48 85 c0                    	testq	%rax, %rax
1403bb01f: 75 2d                       	jne	0x1403bb04e <.text+0x3ba04e>
1403bb021: 31 d2                       	xorl	%edx, %edx
1403bb023: e8 18 66 4e 02              	callq	0x1428a1640 <.text+0x28a0640>
1403bb028: 48 89 c1                    	movq	%rax, %rcx
1403bb02b: 48 85 c0                    	testq	%rax, %rax
1403bb02e: 0f 85 f8 fd ff ff           	jne	0x1403bae2c <.text+0x3b9e2c>
1403bb034: eb 18                       	jmp	0x1403bb04e <.text+0x3ba04e>
1403bb036: 48 85 c0                    	testq	%rax, %rax
1403bb039: 75 13                       	jne	0x1403bb04e <.text+0x3ba04e>
1403bb03b: 31 d2                       	xorl	%edx, %edx
1403bb03d: e8 fe 65 4e 02              	callq	0x1428a1640 <.text+0x28a0640>
1403bb042: 48 89 c1                    	movq	%rax, %rcx
1403bb045: 48 85 c0                    	testq	%rax, %rax
1403bb048: 0f 85 be d1 ff ff           	jne	0x1403b820c <.text+0x3b720c>
1403bb04e: 48 8d 0d bb 79 57 02        	leaq	0x25779bb(%rip), %rcx   # 0x142932a10
1403bb055: e8 76 d0 51 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1403bb05a: 4a 8d 0c d9                 	leaq	(%rcx,%r11,8), %rcx
1403bb05e: 48 83 c1 e0                 	addq	$-0x20, %rcx
1403bb062: e8 69 69 45 00              	callq	0x1408119d0 <.text+0x8109d0>
1403bb067: 48 85 c0                    	testq	%rax, %rax
1403bb06a: 0f 94 c1                    	sete	%cl
1403bb06d: 48 85 d2                    	testq	%rdx, %rdx
1403bb070: 0f 94 c2                    	sete	%dl
1403bb073: 08 ca                       	orb	%cl, %dl
1403bb075: 0f 85 8e fd ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403bb07b: 48 8b 00                    	movq	(%rax), %rax
1403bb07e: 48 89 c1                    	movq	%rax, %rcx
1403bb081: 48 c1 e9 20                 	shrq	$0x20, %rcx
1403bb085: 0f 84 7e fd ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403bb08b: 80 fb 38                    	cmpb	$0x38, %bl
1403bb08e: 0f 84 75 fd ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403bb094: 89 85 f8 02 00 00           	movl	%eax, 0x2f8(%rbp)
1403bb09a: 89 8d fc 02 00 00           	movl	%ecx, 0x2fc(%rbp)
1403bb0a0: 44 89 bd 00 03 00 00        	movl	%r15d, 0x300(%rbp)
1403bb0a7: 8b 85 88 07 00 00           	movl	0x788(%rbp), %eax
1403bb0ad: 89 85 04 03 00 00           	movl	%eax, 0x304(%rbp)
1403bb0b3: 44 89 b5 08 03 00 00        	movl	%r14d, 0x308(%rbp)
1403bb0ba: 88 9d 0c 03 00 00           	movb	%bl, 0x30c(%rbp)
1403bb0c0: 41 c6 45 00 01              	movb	$0x1, (%r13)
1403bb0c5: 48 8b 85 b0 08 00 00        	movq	0x8b0(%rbp), %rax
1403bb0cc: 48 8b 00                    	movq	(%rax), %rax
1403bb0cf: 8b 88 98 00 00 00           	movl	0x98(%rax), %ecx
1403bb0d5: 66 0f 6e 88 9c 00 00 00     	movd	0x9c(%rax), %xmm1
1403bb0dd: e8 ae 49 58 00              	callq	0x14093fa90 <.text+0x93ea90>
1403bb0e2: 84 c0                       	testb	%al, %al
1403bb0e4: 0f 84 1f fd ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403bb0ea: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1403bb0f1: 48 8b 8d c8 08 00 00        	movq	0x8c8(%rbp), %rcx
1403bb0f8: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1403bb0fc: 41 89 c0                    	movl	%eax, %r8d
1403bb0ff: 4c 3b 41 10                 	cmpq	0x10(%rcx), %r8
