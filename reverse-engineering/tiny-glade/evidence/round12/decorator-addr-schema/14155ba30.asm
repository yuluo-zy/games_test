
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14155ba30: 55                          	pushq	%rbp
14155ba31: 41 57                       	pushq	%r15
14155ba33: 41 56                       	pushq	%r14
14155ba35: 41 55                       	pushq	%r13
14155ba37: 41 54                       	pushq	%r12
14155ba39: 56                          	pushq	%rsi
14155ba3a: 57                          	pushq	%rdi
14155ba3b: 53                          	pushq	%rbx
14155ba3c: 48 81 ec 48 04 00 00        	subq	$0x448, %rsp            # imm = 0x448
14155ba43: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14155ba4b: 44 0f 29 b5 b0 03 00 00     	movaps	%xmm14, 0x3b0(%rbp)
14155ba53: 44 0f 29 ad a0 03 00 00     	movaps	%xmm13, 0x3a0(%rbp)
14155ba5b: 44 0f 29 a5 90 03 00 00     	movaps	%xmm12, 0x390(%rbp)
14155ba63: 44 0f 29 9d 80 03 00 00     	movaps	%xmm11, 0x380(%rbp)
14155ba6b: 44 0f 29 95 70 03 00 00     	movaps	%xmm10, 0x370(%rbp)
14155ba73: 66 44 0f 7f 8d 60 03 00 00  	movdqa	%xmm9, 0x360(%rbp)
14155ba7c: 44 0f 29 85 50 03 00 00     	movaps	%xmm8, 0x350(%rbp)
14155ba84: 0f 29 bd 40 03 00 00        	movaps	%xmm7, 0x340(%rbp)
14155ba8b: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
14155ba92: 48 c7 85 28 03 00 00 fe ff ff ff    	movq	$-0x2, 0x328(%rbp)
14155ba9d: 4c 89 8d e0 00 00 00        	movq	%r9, 0xe0(%rbp)
14155baa4: 48 89 8d 28 01 00 00        	movq	%rcx, 0x128(%rbp)
14155baab: 80 b9 b0 01 00 00 00        	cmpb	$0x0, 0x1b0(%rcx)
14155bab2: 0f 85 e6 16 00 00           	jne	0x14155d19e <.text+0x155c19e>
14155bab8: 44 89 c3                    	movl	%r8d, %ebx
14155babb: 48 89 d6                    	movq	%rdx, %rsi
14155babe: e8 7d 1c 7f ff              	callq	0x140d4d740 <.text+0xd4c740>
14155bac3: 48 89 c7                    	movq	%rax, %rdi
14155bac6: f6 00 01                    	testb	$0x1, (%rax)
14155bac9: 74 11                       	je	0x14155badc <.text+0x155aadc>
14155bacb: 48 89 f9                    	movq	%rdi, %rcx
14155bace: 48 83 c1 08                 	addq	$0x8, %rcx
14155bad2: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14155bad6: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
14155bada: eb 1b                       	jmp	0x14155baf7 <.text+0x155aaf7>
14155badc: e8 1f 93 80 ff              	callq	0x140d64e00 <.text+0xd63e00>
14155bae1: 48 89 f9                    	movq	%rdi, %rcx
14155bae4: 48 83 c1 08                 	addq	$0x8, %rcx
14155bae8: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
14155baef: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
14155baf3: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
14155baf7: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14155bafb: 4c 89 01                    	movq	%r8, (%rcx)
14155bafe: 0f 10 05 bb 22 71 01        	movups	0x17122bb(%rip), %xmm0  # 0x142c6ddc0
14155bb05: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
14155bb0c: 0f 10 05 9d 22 71 01        	movups	0x171229d(%rip), %xmm0  # 0x142c6ddb0
14155bb13: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
14155bb1a: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14155bb21: 48 89 95 b8 01 00 00        	movq	%rdx, 0x1b8(%rbp)
14155bb28: 4c 8b 2e                    	movq	(%rsi), %r13
14155bb2b: 48 8b 56 18                 	movq	0x18(%rsi), %rdx
14155bb2f: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
14155bb35: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155bb39: f7 d0                       	notl	%eax
14155bb3b: 4d 8d 7d 10                 	leaq	0x10(%r13), %r15
14155bb3f: 31 c9                       	xorl	%ecx, %ecx
14155bb41: 88 9d b0 02 00 00           	movb	%bl, 0x2b0(%rbp)
14155bb47: 84 db                       	testb	%bl, %bl
14155bb49: 41 b8 30 00 00 00           	movl	$0x30, %r8d
14155bb4f: 4c 0f 44 c1                 	cmoveq	%rcx, %r8
14155bb53: 4c 89 85 a8 02 00 00        	movq	%r8, 0x2a8(%rbp)
14155bb5a: 66 0f 6f 35 1e 15 47 01     	movdqa	0x147151e(%rip), %xmm6  # 0x1429cd080
14155bb62: 66 0f 6f 3d 26 15 47 01     	movdqa	0x1471526(%rip), %xmm7  # 0x1429cd090
14155bb6a: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
14155bb6e: 48 89 95 c0 02 00 00        	movq	%rdx, 0x2c0(%rbp)
14155bb75: 48 89 95 d8 02 00 00        	movq	%rdx, 0x2d8(%rbp)
14155bb7c: 4c 89 bd 30 01 00 00        	movq	%r15, 0x130(%rbp)
14155bb83: 4c 89 ad b8 02 00 00        	movq	%r13, 0x2b8(%rbp)
14155bb8a: eb 11                       	jmp	0x14155bb9d <.text+0x155ab9d>
14155bb8c: 0f 1f 40 00                 	nopl	(%rax)
14155bb90: 8b 85 08 03 00 00           	movl	0x308(%rbp), %eax
14155bb96: 4c 8b ad d0 02 00 00        	movq	0x2d0(%rbp), %r13
14155bb9d: 48 83 bd d8 02 00 00 00     	cmpq	$0x0, 0x2d8(%rbp)
14155bba5: 0f 84 03 07 00 00           	je	0x14155c2ae <.text+0x155b2ae>
14155bbab: 66 85 c0                    	testw	%ax, %ax
14155bbae: 74 10                       	je	0x14155bbc0 <.text+0x155abc0>
14155bbb0: 8d 48 ff                    	leal	-0x1(%rax), %ecx
14155bbb3: 21 c1                       	andl	%eax, %ecx
14155bbb5: 89 8d 08 03 00 00           	movl	%ecx, 0x308(%rbp)
14155bbbb: eb 31                       	jmp	0x14155bbee <.text+0x155abee>
14155bbbd: 0f 1f 00                    	nopl	(%rax)
14155bbc0: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
14155bbc5: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155bbc9: 49 81 c5 00 f9 ff ff        	addq	$-0x700, %r13           # imm = 0xF900
14155bbd0: 49 83 c7 10                 	addq	$0x10, %r15
14155bbd4: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
14155bbd9: 74 e5                       	je	0x14155bbc0 <.text+0x155abc0>
14155bbdb: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14155bbe0: 29 c1                       	subl	%eax, %ecx
14155bbe2: f7 d0                       	notl	%eax
14155bbe4: 89 c2                       	movl	%eax, %edx
14155bbe6: 21 ca                       	andl	%ecx, %edx
14155bbe8: 89 95 08 03 00 00           	movl	%edx, 0x308(%rbp)
14155bbee: f3 0f bc c0                 	tzcntl	%eax, %eax
14155bbf2: 48 f7 d8                    	negq	%rax
14155bbf5: 48 6b f0 70                 	imulq	$0x70, %rax, %rsi
14155bbf9: 49 8b 54 35 90              	movq	-0x70(%r13,%rsi), %rdx
14155bbfe: 4c 89 ad d0 02 00 00        	movq	%r13, 0x2d0(%rbp)
14155bc05: 4d 8b 44 35 98              	movq	-0x68(%r13,%rsi), %r8
14155bc0a: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155bc11: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
14155bc18: 48 89 95 c8 02 00 00        	movq	%rdx, 0x2c8(%rbp)
14155bc1f: 4c 89 85 00 03 00 00        	movq	%r8, 0x300(%rbp)
14155bc26: e8 b5 11 4c ff              	callq	0x140a1cde0 <.text+0xa1bde0>
14155bc2b: 48 89 85 18 03 00 00        	movq	%rax, 0x318(%rbp)
14155bc32: 48 89 95 a0 02 00 00        	movq	%rdx, 0x2a0(%rbp)
14155bc39: 48 ff 8d d8 02 00 00        	decq	0x2d8(%rbp)
14155bc40: 8b 85 08 03 00 00           	movl	0x308(%rbp), %eax
14155bc46: 48 8b 95 18 03 00 00        	movq	0x318(%rbp), %rdx
14155bc4d: 48 85 d2                    	testq	%rdx, %rdx
14155bc50: 4c 8b ad d0 02 00 00        	movq	0x2d0(%rbp), %r13
14155bc57: 0f 84 40 ff ff ff           	je	0x14155bb9d <.text+0x155ab9d>
14155bc5d: 4c 01 ee                    	addq	%r13, %rsi
14155bc60: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
14155bc67: 4c 8b 74 06 a0              	movq	-0x60(%rsi,%rax), %r14
14155bc6c: 4c 8b 6c 06 b8              	movq	-0x48(%rsi,%rax), %r13
14155bc71: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
14155bc76: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155bc7a: f7 d0                       	notl	%eax
14155bc7c: 49 8d 7e 10                 	leaq	0x10(%r14), %rdi
14155bc80: 48 8d 4a 60                 	leaq	0x60(%rdx), %rcx
14155bc84: 48 89 8d f8 02 00 00        	movq	%rcx, 0x2f8(%rbp)
14155bc8b: 48 89 d1                    	movq	%rdx, %rcx
14155bc8e: 48 83 c1 30                 	addq	$0x30, %rcx
14155bc92: 48 89 8d 90 02 00 00        	movq	%rcx, 0x290(%rbp)
14155bc99: eb 0e                       	jmp	0x14155bca9 <.text+0x155aca9>
14155bc9b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14155bca0: 89 f0                       	movl	%esi, %eax
14155bca2: 48 8b 95 18 03 00 00        	movq	0x318(%rbp), %rdx
14155bca9: 4d 85 ed                    	testq	%r13, %r13
14155bcac: 0f 84 de fe ff ff           	je	0x14155bb90 <.text+0x155ab90>
14155bcb2: 66 85 c0                    	testw	%ax, %ax
14155bcb5: 74 09                       	je	0x14155bcc0 <.text+0x155acc0>
14155bcb7: 8d 70 ff                    	leal	-0x1(%rax), %esi
14155bcba: 21 c6                       	andl	%eax, %esi
14155bcbc: eb 29                       	jmp	0x14155bce7 <.text+0x155ace7>
14155bcbe: 66 90                       	nop
14155bcc0: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
14155bcc4: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155bcc8: 49 81 c6 80 fa ff ff        	addq	$-0x580, %r14           # imm = 0xFA80
14155bccf: 48 83 c7 10                 	addq	$0x10, %rdi
14155bcd3: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
14155bcd8: 74 e6                       	je	0x14155bcc0 <.text+0x155acc0>
14155bcda: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14155bcdf: 29 c1                       	subl	%eax, %ecx
14155bce1: f7 d0                       	notl	%eax
14155bce3: 89 c6                       	movl	%eax, %esi
14155bce5: 21 ce                       	andl	%ecx, %esi
14155bce7: f3 0f bc c0                 	tzcntl	%eax, %eax
14155bceb: 48 f7 d8                    	negq	%rax
14155bcee: 48 6b c0 58                 	imulq	$0x58, %rax, %rax
14155bcf2: 45 8b 64 06 a8              	movl	-0x58(%r14,%rax), %r12d
14155bcf7: 44 89 a5 e0 02 00 00        	movl	%r12d, 0x2e0(%rbp)
14155bcfe: f3 0f 6f 82 80 00 00 00     	movdqu	0x80(%rdx), %xmm0
14155bd06: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14155bd0b: 66 0f ef ce                 	pxor	%xmm6, %xmm1
14155bd0f: 66 0f 7f 8d c0 01 00 00     	movdqa	%xmm1, 0x1c0(%rbp)
14155bd17: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14155bd1c: 66 0f ef cf                 	pxor	%xmm7, %xmm1
14155bd20: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
14155bd28: 66 0f 7f 85 e0 01 00 00     	movdqa	%xmm0, 0x1e0(%rbp)
14155bd30: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
14155bd37: 44 0f 29 40 20              	movaps	%xmm8, 0x20(%rax)
14155bd3c: 48 c7 40 30 00 00 00 00     	movq	$0x0, 0x30(%rax)
14155bd44: 44 89 65 20                 	movl	%r12d, 0x20(%rbp)
14155bd48: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14155bd4e: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14155bd55: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
14155bd59: e8 02 a4 ba fe              	callq	0x140106160 <.text+0x105160>
14155bd5e: 4c 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %r9
14155bd65: 49 c1 e1 38                 	shlq	$0x38, %r9
14155bd69: 4c 0b 8d f8 01 00 00        	orq	0x1f8(%rbp), %r9
14155bd70: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
14155bd77: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
14155bd7e: 4c 31 c8                    	xorq	%r9, %rax
14155bd81: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
14155bd88: 48 01 ca                    	addq	%rcx, %rdx
14155bd8b: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14155bd92: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155bd96: 49 01 c0                    	addq	%rax, %r8
14155bd99: 48 31 d1                    	xorq	%rdx, %rcx
14155bd9c: 48 c1 c0 10                 	rolq	$0x10, %rax
14155bda0: 4c 31 c0                    	xorq	%r8, %rax
14155bda3: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155bda7: 49 01 c8                    	addq	%rcx, %r8
14155bdaa: 48 01 c2                    	addq	%rax, %rdx
14155bdad: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155bdb1: 4c 31 c1                    	xorq	%r8, %rcx
14155bdb4: 48 c1 c0 15                 	rolq	$0x15, %rax
14155bdb8: 49 c1 c0 20                 	rolq	$0x20, %r8
14155bdbc: 48 31 d0                    	xorq	%rdx, %rax
14155bdbf: 4c 31 ca                    	xorq	%r9, %rdx
14155bdc2: 49 81 f0 ff 00 00 00        	xorq	$0xff, %r8
14155bdc9: 48 01 ca                    	addq	%rcx, %rdx
14155bdcc: 49 01 c0                    	addq	%rax, %r8
14155bdcf: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155bdd3: 48 31 d1                    	xorq	%rdx, %rcx
14155bdd6: 48 c1 c0 10                 	rolq	$0x10, %rax
14155bdda: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155bdde: 4c 31 c0                    	xorq	%r8, %rax
14155bde1: 49 01 c8                    	addq	%rcx, %r8
14155bde4: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155bde8: 48 01 c2                    	addq	%rax, %rdx
14155bdeb: 4c 31 c1                    	xorq	%r8, %rcx
14155bdee: 48 c1 c0 15                 	rolq	$0x15, %rax
14155bdf2: 48 31 d0                    	xorq	%rdx, %rax
14155bdf5: 49 c1 c0 20                 	rolq	$0x20, %r8
14155bdf9: 48 01 ca                    	addq	%rcx, %rdx
14155bdfc: 49 01 c0                    	addq	%rax, %r8
14155bdff: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155be03: 48 31 d1                    	xorq	%rdx, %rcx
14155be06: 48 c1 c0 10                 	rolq	$0x10, %rax
14155be0a: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155be0e: 4c 31 c0                    	xorq	%r8, %rax
14155be11: 49 01 c8                    	addq	%rcx, %r8
14155be14: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155be18: 48 01 c2                    	addq	%rax, %rdx
14155be1b: 4c 31 c1                    	xorq	%r8, %rcx
14155be1e: 48 c1 c0 15                 	rolq	$0x15, %rax
14155be22: 48 31 d0                    	xorq	%rdx, %rax
14155be25: 49 c1 c0 20                 	rolq	$0x20, %r8
14155be29: 48 01 ca                    	addq	%rcx, %rdx
14155be2c: 49 01 c0                    	addq	%rax, %r8
14155be2f: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155be33: 48 31 d1                    	xorq	%rdx, %rcx
14155be36: 48 c1 c0 10                 	rolq	$0x10, %rax
14155be3a: 4c 31 c0                    	xorq	%r8, %rax
14155be3d: 49 01 c8                    	addq	%rcx, %r8
14155be40: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155be44: 48 c1 c0 15                 	rolq	$0x15, %rax
14155be48: 4c 89 c2                    	movq	%r8, %rdx
14155be4b: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155be4f: 48 31 ca                    	xorq	%rcx, %rdx
14155be52: 48 31 c2                    	xorq	%rax, %rdx
14155be55: 4c 31 c2                    	xorq	%r8, %rdx
14155be58: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155be5f: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
14155be66: 4c 8d 85 e0 02 00 00        	leaq	0x2e0(%rbp), %r8
14155be6d: e8 5e b8 61 ff              	callq	0x140b776d0 <.text+0xb766d0>
14155be72: 8b 95 e0 02 00 00           	movl	0x2e0(%rbp), %edx
14155be78: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155be7f: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
14155be86: e8 75 d3 d6 fe              	callq	0x1402c9200 <.text+0x2c8200>
14155be8b: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14155be92: f3 0f 6f 40 50              	movdqu	0x50(%rax), %xmm0
14155be97: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14155be9c: 66 0f ef ce                 	pxor	%xmm6, %xmm1
14155bea0: 66 0f 7f 4d 20              	movdqa	%xmm1, 0x20(%rbp)
14155bea5: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14155beaa: 66 0f ef cf                 	pxor	%xmm7, %xmm1
14155beae: 66 0f 7f 4d 30              	movdqa	%xmm1, 0x30(%rbp)
14155beb3: 66 0f 7f 45 40              	movdqa	%xmm0, 0x40(%rbp)
14155beb8: 48 8d 45 50                 	leaq	0x50(%rbp), %rax
14155bebc: 44 0f 29 00                 	movaps	%xmm8, (%rax)
14155bec0: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
14155bec8: 8b 85 e0 02 00 00           	movl	0x2e0(%rbp), %eax
14155bece: 89 85 f8 00 00 00           	movl	%eax, 0xf8(%rbp)
14155bed4: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14155beda: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14155bede: 48 8d 95 f8 00 00 00        	leaq	0xf8(%rbp), %rdx
14155bee5: e8 76 a2 ba fe              	callq	0x140106160 <.text+0x105160>
14155beea: 4c 8b 4d 50                 	movq	0x50(%rbp), %r9
14155beee: 49 c1 e1 38                 	shlq	$0x38, %r9
14155bef2: 4c 0b 4d 58                 	orq	0x58(%rbp), %r9
14155bef6: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
14155befa: 48 8b 45 38                 	movq	0x38(%rbp), %rax
14155befe: 4c 31 c8                    	xorq	%r9, %rax
14155bf01: 4c 8b 45 20                 	movq	0x20(%rbp), %r8
14155bf05: 49 01 c8                    	addq	%rcx, %r8
14155bf08: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
14155bf0c: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155bf10: 48 01 c2                    	addq	%rax, %rdx
14155bf13: 4c 31 c1                    	xorq	%r8, %rcx
14155bf16: 48 c1 c0 10                 	rolq	$0x10, %rax
14155bf1a: 48 31 d0                    	xorq	%rdx, %rax
14155bf1d: 49 c1 c0 20                 	rolq	$0x20, %r8
14155bf21: 48 01 ca                    	addq	%rcx, %rdx
14155bf24: 49 01 c0                    	addq	%rax, %r8
14155bf27: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155bf2b: 48 31 d1                    	xorq	%rdx, %rcx
14155bf2e: 48 c1 c0 15                 	rolq	$0x15, %rax
14155bf32: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155bf36: 4c 31 c0                    	xorq	%r8, %rax
14155bf39: 4d 31 c8                    	xorq	%r9, %r8
14155bf3c: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
14155bf43: 49 01 c8                    	addq	%rcx, %r8
14155bf46: 48 01 c2                    	addq	%rax, %rdx
14155bf49: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155bf4d: 4c 31 c1                    	xorq	%r8, %rcx
14155bf50: 48 c1 c0 10                 	rolq	$0x10, %rax
14155bf54: 49 c1 c0 20                 	rolq	$0x20, %r8
14155bf58: 48 31 d0                    	xorq	%rdx, %rax
14155bf5b: 48 01 ca                    	addq	%rcx, %rdx
14155bf5e: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155bf62: 49 01 c0                    	addq	%rax, %r8
14155bf65: 48 31 d1                    	xorq	%rdx, %rcx
14155bf68: 48 c1 c0 15                 	rolq	$0x15, %rax
14155bf6c: 4c 31 c0                    	xorq	%r8, %rax
14155bf6f: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155bf73: 49 01 c8                    	addq	%rcx, %r8
14155bf76: 48 01 c2                    	addq	%rax, %rdx
14155bf79: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155bf7d: 4c 31 c1                    	xorq	%r8, %rcx
14155bf80: 48 c1 c0 10                 	rolq	$0x10, %rax
14155bf84: 49 c1 c0 20                 	rolq	$0x20, %r8
14155bf88: 48 31 d0                    	xorq	%rdx, %rax
14155bf8b: 48 01 ca                    	addq	%rcx, %rdx
14155bf8e: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155bf92: 49 01 c0                    	addq	%rax, %r8
14155bf95: 48 31 d1                    	xorq	%rdx, %rcx
14155bf98: 48 c1 c0 15                 	rolq	$0x15, %rax
14155bf9c: 4c 31 c0                    	xorq	%r8, %rax
14155bf9f: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155bfa3: 49 01 c8                    	addq	%rcx, %r8
14155bfa6: 48 01 c2                    	addq	%rax, %rdx
14155bfa9: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155bfad: 4c 31 c1                    	xorq	%r8, %rcx
14155bfb0: 48 c1 c0 10                 	rolq	$0x10, %rax
14155bfb4: 48 31 d0                    	xorq	%rdx, %rax
14155bfb7: 48 01 ca                    	addq	%rcx, %rdx
14155bfba: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155bfbe: 48 c1 c0 15                 	rolq	$0x15, %rax
14155bfc2: 49 89 d0                    	movq	%rdx, %r8
14155bfc5: 49 c1 c0 20                 	rolq	$0x20, %r8
14155bfc9: 49 31 c8                    	xorq	%rcx, %r8
14155bfcc: 49 31 c0                    	xorq	%rax, %r8
14155bfcf: 49 31 d0                    	xorq	%rdx, %r8
14155bfd2: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155bfd9: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14155bfe0: 48 8b 95 90 02 00 00        	movq	0x290(%rbp), %rdx
14155bfe7: 4c 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %r9
14155bfee: e8 2d bb 61 ff              	callq	0x140b77b20 <.text+0xb76b20>
14155bff3: 49 ff cd                    	decq	%r13
14155bff6: 0f b6 9d 0f 02 00 00        	movzbl	0x20f(%rbp), %ebx
14155bffd: 80 fb 02                    	cmpb	$0x2, %bl
14155c000: 0f 84 9a fc ff ff           	je	0x14155bca0 <.text+0x155aca0>
14155c006: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
14155c00d: 48 8b 41 37                 	movq	0x37(%rcx), %rax
14155c011: 48 89 85 7f 01 00 00        	movq	%rax, 0x17f(%rbp)
14155c018: 0f 10 41 f8                 	movups	-0x8(%rcx), %xmm0
14155c01c: 0f 10 49 08                 	movups	0x8(%rcx), %xmm1
14155c020: 0f 10 51 18                 	movups	0x18(%rcx), %xmm2
14155c024: 0f 10 59 28                 	movups	0x28(%rcx), %xmm3
14155c028: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
14155c02f: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
14155c036: 0f 29 8d 50 01 00 00        	movaps	%xmm1, 0x150(%rbp)
14155c03d: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
14155c044: 8b 85 e0 02 00 00           	movl	0x2e0(%rbp), %eax
14155c04a: 89 85 f8 00 00 00           	movl	%eax, 0xf8(%rbp)
14155c050: 4c 89 ad 98 02 00 00        	movq	%r13, 0x298(%rbp)
14155c057: 4c 8b ad a0 02 00 00        	movq	0x2a0(%rbp), %r13
14155c05e: f3 41 0f 6f 45 20           	movdqu	0x20(%r13), %xmm0
14155c064: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14155c069: 66 0f ef ce                 	pxor	%xmm6, %xmm1
14155c06d: 66 0f 7f 8d c0 01 00 00     	movdqa	%xmm1, 0x1c0(%rbp)
14155c075: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14155c07a: 66 0f ef cf                 	pxor	%xmm7, %xmm1
14155c07e: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
14155c086: 66 0f 7f 85 e0 01 00 00     	movdqa	%xmm0, 0x1e0(%rbp)
14155c08e: 44 0f 29 41 20              	movaps	%xmm8, 0x20(%rcx)
14155c093: 48 c7 41 30 00 00 00 00     	movq	$0x0, 0x30(%rcx)
14155c09b: 89 45 20                    	movl	%eax, 0x20(%rbp)
14155c09e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14155c0a4: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14155c0ab: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
14155c0af: e8 ac a0 ba fe              	callq	0x140106160 <.text+0x105160>
14155c0b4: 4c 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %r9
14155c0bb: 49 c1 e1 38                 	shlq	$0x38, %r9
14155c0bf: 4c 0b 8d f8 01 00 00        	orq	0x1f8(%rbp), %r9
14155c0c6: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
14155c0cd: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
14155c0d4: 4c 31 c8                    	xorq	%r9, %rax
14155c0d7: 4c 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %r8
14155c0de: 49 01 c8                    	addq	%rcx, %r8
14155c0e1: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
14155c0e8: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155c0ec: 48 01 c2                    	addq	%rax, %rdx
14155c0ef: 4c 31 c1                    	xorq	%r8, %rcx
14155c0f2: 48 c1 c0 10                 	rolq	$0x10, %rax
14155c0f6: 48 31 d0                    	xorq	%rdx, %rax
14155c0f9: 49 c1 c0 20                 	rolq	$0x20, %r8
14155c0fd: 48 01 ca                    	addq	%rcx, %rdx
14155c100: 49 01 c0                    	addq	%rax, %r8
14155c103: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155c107: 48 31 d1                    	xorq	%rdx, %rcx
14155c10a: 48 c1 c0 15                 	rolq	$0x15, %rax
14155c10e: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155c112: 4c 31 c0                    	xorq	%r8, %rax
14155c115: 4d 31 c8                    	xorq	%r9, %r8
14155c118: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
14155c11f: 49 01 c8                    	addq	%rcx, %r8
14155c122: 48 01 c2                    	addq	%rax, %rdx
14155c125: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155c129: 4c 31 c1                    	xorq	%r8, %rcx
14155c12c: 48 c1 c0 10                 	rolq	$0x10, %rax
14155c130: 49 c1 c0 20                 	rolq	$0x20, %r8
14155c134: 48 31 d0                    	xorq	%rdx, %rax
14155c137: 48 01 ca                    	addq	%rcx, %rdx
14155c13a: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155c13e: 49 01 c0                    	addq	%rax, %r8
14155c141: 48 31 d1                    	xorq	%rdx, %rcx
14155c144: 48 c1 c0 15                 	rolq	$0x15, %rax
14155c148: 4c 31 c0                    	xorq	%r8, %rax
14155c14b: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155c14f: 49 01 c8                    	addq	%rcx, %r8
14155c152: 48 01 c2                    	addq	%rax, %rdx
14155c155: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155c159: 4c 31 c1                    	xorq	%r8, %rcx
14155c15c: 48 c1 c0 10                 	rolq	$0x10, %rax
14155c160: 49 c1 c0 20                 	rolq	$0x20, %r8
14155c164: 48 31 d0                    	xorq	%rdx, %rax
14155c167: 48 01 ca                    	addq	%rcx, %rdx
14155c16a: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155c16e: 49 01 c0                    	addq	%rax, %r8
14155c171: 48 31 d1                    	xorq	%rdx, %rcx
14155c174: 48 c1 c0 15                 	rolq	$0x15, %rax
14155c178: 4c 31 c0                    	xorq	%r8, %rax
14155c17b: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155c17f: 49 01 c8                    	addq	%rcx, %r8
14155c182: 48 01 c2                    	addq	%rax, %rdx
14155c185: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155c189: 4c 31 c1                    	xorq	%r8, %rcx
14155c18c: 48 c1 c0 10                 	rolq	$0x10, %rax
14155c190: 48 31 d0                    	xorq	%rdx, %rax
14155c193: 48 01 ca                    	addq	%rcx, %rdx
14155c196: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155c19a: 48 c1 c0 15                 	rolq	$0x15, %rax
14155c19e: 49 89 d0                    	movq	%rdx, %r8
14155c1a1: 49 c1 c0 20                 	rolq	$0x20, %r8
14155c1a5: 49 31 c8                    	xorq	%rcx, %r8
14155c1a8: 49 31 c0                    	xorq	%rax, %r8
14155c1ab: 49 31 d0                    	xorq	%rdx, %r8
14155c1ae: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155c1b5: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14155c1bc: 4c 89 ea                    	movq	%r13, %rdx
14155c1bf: 4c 8b ad 98 02 00 00        	movq	0x298(%rbp), %r13
14155c1c6: 4c 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %r9
14155c1cd: e8 6e bf 61 ff              	callq	0x140b78140 <.text+0xb77140>
14155c1d2: 83 bd c8 01 00 00 03        	cmpl	$0x3, 0x1c8(%rbp)
14155c1d9: 0f 84 dd 0c 00 00           	je	0x14155cebc <.text+0x155bebc>
14155c1df: 0f 28 85 40 01 00 00        	movaps	0x140(%rbp), %xmm0
14155c1e6: 0f 28 8d 50 01 00 00        	movaps	0x150(%rbp), %xmm1
14155c1ed: 0f 28 95 60 01 00 00        	movaps	0x160(%rbp), %xmm2
14155c1f4: 0f 28 9d 70 01 00 00        	movaps	0x170(%rbp), %xmm3
14155c1fb: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
14155c1ff: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
14155c203: 0f 29 55 f0                 	movaps	%xmm2, -0x10(%rbp)
14155c207: 0f 29 5d 00                 	movaps	%xmm3, (%rbp)
14155c20b: 48 8b 85 7f 01 00 00        	movq	0x17f(%rbp), %rax
14155c212: 48 89 45 0f                 	movq	%rax, 0xf(%rbp)
14155c216: 88 5d 17                    	movb	%bl, 0x17(%rbp)
14155c219: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155c220: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14155c224: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
14155c228: e8 03 37 5e ff              	callq	0x140b3f930 <.text+0xb3e930>
14155c22d: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
14155c231: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
14155c238: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
14155c23c: f3 0f 6f 45 20              	movdqu	0x20(%rbp), %xmm0
14155c241: f3 0f 6f 4d 30              	movdqu	0x30(%rbp), %xmm1
14155c246: f3 0f 6f 55 40              	movdqu	0x40(%rbp), %xmm2
14155c24b: 0f 10 5d 50                 	movups	0x50(%rbp), %xmm3
14155c24f: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
14155c253: f3 0f 7f 50 20              	movdqu	%xmm2, 0x20(%rax)
14155c258: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
14155c25d: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14155c261: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
14155c268: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14155c26f: 48 8b 85 00 03 00 00        	movq	0x300(%rbp), %rax
14155c276: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
14155c27d: 48 c7 85 20 02 00 00 03 00 00 00    	movq	$0x3, 0x220(%rbp)
14155c288: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155c28f: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14155c293: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
14155c29a: 45 89 e0                    	movl	%r12d, %r8d
14155c29d: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
14155c2a4: e8 e7 00 fd ff              	callq	0x14152c390 <.text+0x152b390>
14155c2a9: e9 f2 f9 ff ff              	jmp	0x14155bca0 <.text+0x155aca0>
14155c2ae: 4c 8b 9d b8 02 00 00        	movq	0x2b8(%rbp), %r11
14155c2b5: 66 41 0f 6f 03              	movdqa	(%r11), %xmm0
14155c2ba: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155c2be: f7 d0                       	notl	%eax
14155c2c0: 31 c9                       	xorl	%ecx, %ecx
14155c2c2: 80 bd b0 02 00 00 00        	cmpb	$0x0, 0x2b0(%rbp)
14155c2c9: 41 b9 30 00 00 00           	movl	$0x30, %r9d
14155c2cf: 4c 0f 45 c9                 	cmovneq	%rcx, %r9
14155c2d3: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
14155c2d7: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
14155c2dc: 4c 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %r8
14155c2e3: 4c 89 8d 38 01 00 00        	movq	%r9, 0x138(%rbp)
14155c2ea: 4d 85 c0                    	testq	%r8, %r8
14155c2ed: 0f 84 ed 0b 00 00           	je	0x14155cee0 <.text+0x155bee0>
14155c2f3: 66 85 c0                    	testw	%ax, %ax
14155c2f6: 74 18                       	je	0x14155c310 <.text+0x155b310>
14155c2f8: 44 8d 50 ff                 	leal	-0x1(%rax), %r10d
14155c2fc: 41 21 c2                    	andl	%eax, %r10d
14155c2ff: eb 4f                       	jmp	0x14155c350 <.text+0x155b350>
14155c301: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14155c310: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14155c317: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14155c320: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
14155c324: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155c328: 49 81 c3 00 f9 ff ff        	addq	$-0x700, %r11           # imm = 0xF900
14155c32f: 48 83 c1 10                 	addq	$0x10, %rcx
14155c333: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
14155c338: 74 e6                       	je	0x14155c320 <.text+0x155b320>
14155c33a: 48 89 8d 30 01 00 00        	movq	%rcx, 0x130(%rbp)
14155c341: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14155c346: 29 c1                       	subl	%eax, %ecx
14155c348: f7 d0                       	notl	%eax
14155c34a: 41 89 c2                    	movl	%eax, %r10d
14155c34d: 41 21 ca                    	andl	%ecx, %r10d
14155c350: 49 ff c8                    	decq	%r8
14155c353: f3 0f bc c0                 	tzcntl	%eax, %eax
14155c357: 48 f7 d8                    	negq	%rax
14155c35a: 48 6b c8 70                 	imulq	$0x70, %rax, %rcx
14155c35e: 49 8d 34 0b                 	leaq	(%r11,%rcx), %rsi
14155c362: 49 83 7c 31 b8 00           	cmpq	$0x0, -0x48(%r9,%rsi)
14155c368: 44 89 d0                    	movl	%r10d, %eax
14155c36b: 0f 84 79 ff ff ff           	je	0x14155c2ea <.text+0x155b2ea>
14155c371: 44 89 95 8c 02 00 00        	movl	%r10d, 0x28c(%rbp)
14155c378: 4c 89 85 c0 02 00 00        	movq	%r8, 0x2c0(%rbp)
14155c37f: 49 8b 54 0b 90              	movq	-0x70(%r11,%rcx), %rdx
14155c384: 4c 89 9d b8 02 00 00        	movq	%r11, 0x2b8(%rbp)
14155c38b: 4d 8b 44 0b 98              	movq	-0x68(%r11,%rcx), %r8
14155c390: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155c397: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
14155c39e: 48 89 95 08 03 00 00        	movq	%rdx, 0x308(%rbp)
14155c3a5: 4c 89 85 18 03 00 00        	movq	%r8, 0x318(%rbp)
14155c3ac: e8 7f 0b 4c ff              	callq	0x140a1cf30 <.text+0xa1bf30>
14155c3b1: 48 89 95 d0 02 00 00        	movq	%rdx, 0x2d0(%rbp)
14155c3b8: 48 89 c1                    	movq	%rax, %rcx
14155c3bb: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
14155c3c2: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
14155c3c9: 4c 8b 6c 06 a0              	movq	-0x60(%rsi,%rax), %r13
14155c3ce: 48 8b 54 06 b8              	movq	-0x48(%rsi,%rax), %rdx
14155c3d3: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
14155c3d9: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155c3dd: f7 d0                       	notl	%eax
14155c3df: 49 8d 5d 10                 	leaq	0x10(%r13), %rbx
14155c3e3: 4c 8d 41 60                 	leaq	0x60(%rcx), %r8
14155c3e7: 4c 89 85 a8 02 00 00        	movq	%r8, 0x2a8(%rbp)
14155c3ee: 48 83 c1 30                 	addq	$0x30, %rcx
14155c3f2: 48 89 8d b0 02 00 00        	movq	%rcx, 0x2b0(%rbp)
14155c3f9: eb 78                       	jmp	0x14155c473 <.text+0x155b473>
14155c3fb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14155c400: 49 21 c8                    	andq	%rcx, %r8
14155c403: f3 42 0f 6f 0c 00           	movdqu	(%rax,%r8), %xmm1
14155c409: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14155c40d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14155c411: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14155c416: 45 85 d2                    	testl	%r10d, %r10d
14155c419: 74 25                       	je	0x14155c440 <.text+0x155b440>
14155c41b: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14155c420: 4d 01 c3                    	addq	%r8, %r11
14155c423: 49 21 cb                    	andq	%rcx, %r11
14155c426: 49 f7 db                    	negq	%r11
14155c429: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
14155c42d: 46 39 3c da                 	cmpl	%r15d, (%rdx,%r11,8)
14155c431: 74 2d                       	je	0x14155c460 <.text+0x155b460>
14155c433: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14155c437: 66 45 21 d3                 	andw	%r10w, %r11w
14155c43b: 45 89 da                    	movl	%r11d, %r10d
14155c43e: 75 db                       	jne	0x14155c41b <.text+0x155b41b>
14155c440: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
14155c445: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
14155c44a: 45 85 d2                    	testl	%r10d, %r10d
14155c44d: 0f 85 d5 09 00 00           	jne	0x14155ce28 <.text+0x155be28>
14155c453: 4d 01 c8                    	addq	%r9, %r8
14155c456: 49 83 c0 10                 	addq	$0x10, %r8
14155c45a: 49 83 c1 10                 	addq	$0x10, %r9
14155c45e: eb a0                       	jmp	0x14155c400 <.text+0x155b400>
14155c460: 46 0f 11 54 d8 f0           	movups	%xmm10, -0x10(%rax,%r11,8)
14155c466: 8b 85 00 03 00 00           	movl	0x300(%rbp), %eax
14155c46c: 48 8b 95 c8 02 00 00        	movq	0x2c8(%rbp), %rdx
14155c473: 48 83 ea 01                 	subq	$0x1, %rdx
14155c477: 0f 82 8b 09 00 00           	jb	0x14155ce08 <.text+0x155be08>
14155c47d: 66 85 c0                    	testw	%ax, %ax
14155c480: 48 89 95 c8 02 00 00        	movq	%rdx, 0x2c8(%rbp)
14155c487: 74 17                       	je	0x14155c4a0 <.text+0x155b4a0>
14155c489: 8d 48 ff                    	leal	-0x1(%rax), %ecx
14155c48c: 21 c1                       	andl	%eax, %ecx
14155c48e: 89 8d 00 03 00 00           	movl	%ecx, 0x300(%rbp)
14155c494: eb 37                       	jmp	0x14155c4cd <.text+0x155b4cd>
14155c496: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14155c4a0: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
14155c4a4: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155c4a8: 49 81 c5 80 fa ff ff        	addq	$-0x580, %r13           # imm = 0xFA80
14155c4af: 48 83 c3 10                 	addq	$0x10, %rbx
14155c4b3: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
14155c4b8: 74 e6                       	je	0x14155c4a0 <.text+0x155b4a0>
14155c4ba: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14155c4bf: 29 c1                       	subl	%eax, %ecx
14155c4c1: f7 d0                       	notl	%eax
14155c4c3: 89 c2                       	movl	%eax, %edx
14155c4c5: 21 ca                       	andl	%ecx, %edx
14155c4c7: 89 95 00 03 00 00           	movl	%edx, 0x300(%rbp)
14155c4cd: f3 0f bc c0                 	tzcntl	%eax, %eax
14155c4d1: 48 f7 d8                    	negq	%rax
14155c4d4: 48 6b c0 58                 	imulq	$0x58, %rax, %rax
14155c4d8: 4e 8d 34 28                 	leaq	(%rax,%r13), %r14
14155c4dc: 45 8b 7c 05 a8              	movl	-0x58(%r13,%rax), %r15d
14155c4e1: 4c 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %r8
14155c4e8: 41 8b 08                    	movl	(%r8), %ecx
14155c4eb: 4c 89 fa                    	movq	%r15, %rdx
14155c4ee: 48 ff c2                    	incq	%rdx
14155c4f1: 48 39 ca                    	cmpq	%rcx, %rdx
14155c4f4: 48 0f 46 d1                 	cmovbeq	%rcx, %rdx
14155c4f8: 49 8d 7c 05 b0              	leaq	-0x50(%r13,%rax), %rdi
14155c4fd: 41 89 10                    	movl	%edx, (%r8)
14155c500: f2 45 0f 10 54 05 d8        	movsd	-0x28(%r13,%rax), %xmm10
14155c507: f3 45 0f 10 5c 05 e0        	movss	-0x20(%r13,%rax), %xmm11
14155c50e: 41 0f b6 4c 05 fc           	movzbl	-0x4(%r13,%rax), %ecx
14155c514: 88 8d f8 02 00 00           	movb	%cl, 0x2f8(%rbp)
14155c51a: 41 8b 4c 05 e4              	movl	-0x1c(%r13,%rax), %ecx
14155c51f: 89 8d a0 02 00 00           	movl	%ecx, 0x2a0(%rbp)
14155c525: 41 0f b6 4c 05 fd           	movzbl	-0x3(%r13,%rax), %ecx
14155c52b: 88 8d 90 02 00 00           	movb	%cl, 0x290(%rbp)
14155c531: f2 45 0f 10 6c 05 e8        	movsd	-0x18(%r13,%rax), %xmm13
14155c538: f3 45 0f 10 64 05 f8        	movss	-0x8(%r13,%rax), %xmm12
14155c53f: 41 0f b6 4c 05 fe           	movzbl	-0x2(%r13,%rax), %ecx
14155c545: 88 8d 98 02 00 00           	movb	%cl, 0x298(%rbp)
14155c54b: f2 45 0f 10 74 05 f0        	movsd	-0x10(%r13,%rax), %xmm14
14155c552: 49 8b 4c 05 d0              	movq	-0x30(%r13,%rax), %rcx
14155c557: 41 0f 10 44 05 b0           	movups	-0x50(%r13,%rax), %xmm0
14155c55d: 41 0f 10 4c 05 c0           	movups	-0x40(%r13,%rax), %xmm1
14155c563: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
14155c567: 0f 29 4d 30                 	movaps	%xmm1, 0x30(%rbp)
14155c56b: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
14155c56f: 66 0f 6f 85 b0 01 00 00     	movdqa	0x1b0(%rbp), %xmm0
14155c577: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14155c57c: 66 0f ef ce                 	pxor	%xmm6, %xmm1
14155c580: 66 0f 7f 8d c0 01 00 00     	movdqa	%xmm1, 0x1c0(%rbp)
14155c588: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14155c58d: 66 0f ef cf                 	pxor	%xmm7, %xmm1
14155c591: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
14155c599: 66 0f 7f 85 e0 01 00 00     	movdqa	%xmm0, 0x1e0(%rbp)
14155c5a1: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
14155c5a8: 44 0f 29 40 20              	movaps	%xmm8, 0x20(%rax)
14155c5ad: 48 c7 40 30 00 00 00 00     	movq	$0x0, 0x30(%rax)
14155c5b5: 44 89 bd 40 01 00 00        	movl	%r15d, 0x140(%rbp)
14155c5bc: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14155c5c2: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14155c5c9: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
14155c5d0: e8 8b 9b ba fe              	callq	0x140106160 <.text+0x105160>
14155c5d5: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
14155c5dc: 4c 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %r9
14155c5e3: 49 c1 e1 38                 	shlq	$0x38, %r9
14155c5e7: 4c 0b 8d f8 01 00 00        	orq	0x1f8(%rbp), %r9
14155c5ee: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14155c5f5: 4c 31 c9                    	xorq	%r9, %rcx
14155c5f8: 4c 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %r8
14155c5ff: 49 01 c0                    	addq	%rax, %r8
14155c602: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
14155c609: 48 01 ca                    	addq	%rcx, %rdx
14155c60c: 48 c1 c0 0d                 	rolq	$0xd, %rax
14155c610: 4c 31 c0                    	xorq	%r8, %rax
14155c613: 48 c1 c1 10                 	rolq	$0x10, %rcx
14155c617: 49 c1 c0 20                 	rolq	$0x20, %r8
14155c61b: 48 31 d1                    	xorq	%rdx, %rcx
14155c61e: 48 01 c2                    	addq	%rax, %rdx
14155c621: 48 c1 c0 11                 	rolq	$0x11, %rax
14155c625: 49 01 c8                    	addq	%rcx, %r8
14155c628: 48 31 d0                    	xorq	%rdx, %rax
14155c62b: 48 c1 c1 15                 	rolq	$0x15, %rcx
14155c62f: 4c 31 c1                    	xorq	%r8, %rcx
14155c632: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155c636: 4d 31 c8                    	xorq	%r9, %r8
14155c639: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
14155c640: 49 01 c0                    	addq	%rax, %r8
14155c643: 48 c1 c0 0d                 	rolq	$0xd, %rax
14155c647: 48 01 ca                    	addq	%rcx, %rdx
14155c64a: 4c 31 c0                    	xorq	%r8, %rax
14155c64d: 48 c1 c1 10                 	rolq	$0x10, %rcx
14155c651: 48 31 d1                    	xorq	%rdx, %rcx
14155c654: 49 c1 c0 20                 	rolq	$0x20, %r8
14155c658: 48 01 c2                    	addq	%rax, %rdx
14155c65b: 49 01 c8                    	addq	%rcx, %r8
14155c65e: 48 c1 c0 11                 	rolq	$0x11, %rax
14155c662: 48 31 d0                    	xorq	%rdx, %rax
14155c665: 48 c1 c1 15                 	rolq	$0x15, %rcx
14155c669: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155c66d: 4c 31 c1                    	xorq	%r8, %rcx
14155c670: 49 01 c0                    	addq	%rax, %r8
14155c673: 48 c1 c0 0d                 	rolq	$0xd, %rax
14155c677: 48 01 ca                    	addq	%rcx, %rdx
14155c67a: 4c 31 c0                    	xorq	%r8, %rax
14155c67d: 48 c1 c1 10                 	rolq	$0x10, %rcx
14155c681: 48 31 d1                    	xorq	%rdx, %rcx
14155c684: 49 c1 c0 20                 	rolq	$0x20, %r8
14155c688: 48 01 c2                    	addq	%rax, %rdx
14155c68b: 49 01 c8                    	addq	%rcx, %r8
14155c68e: 48 c1 c0 11                 	rolq	$0x11, %rax
14155c692: 48 31 d0                    	xorq	%rdx, %rax
14155c695: 48 c1 c1 15                 	rolq	$0x15, %rcx
14155c699: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155c69d: 4c 31 c1                    	xorq	%r8, %rcx
14155c6a0: 49 01 c0                    	addq	%rax, %r8
14155c6a3: 48 c1 c0 0d                 	rolq	$0xd, %rax
14155c6a7: 48 01 ca                    	addq	%rcx, %rdx
14155c6aa: 4c 31 c0                    	xorq	%r8, %rax
14155c6ad: 48 c1 c1 10                 	rolq	$0x10, %rcx
14155c6b1: 48 31 d1                    	xorq	%rdx, %rcx
14155c6b4: 48 01 c2                    	addq	%rax, %rdx
14155c6b7: 48 c1 c0 11                 	rolq	$0x11, %rax
14155c6bb: 48 c1 c1 15                 	rolq	$0x15, %rcx
14155c6bf: 49 89 d4                    	movq	%rdx, %r12
14155c6c2: 49 c1 c4 20                 	rolq	$0x20, %r12
14155c6c6: 49 31 c4                    	xorq	%rax, %r12
14155c6c9: 49 31 cc                    	xorq	%rcx, %r12
14155c6cc: 49 31 d4                    	xorq	%rdx, %r12
14155c6cf: 4c 89 e6                    	movq	%r12, %rsi
14155c6d2: 48 c1 ee 39                 	shrq	$0x39, %rsi
14155c6d6: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
14155c6dd: 48 8b 8d 98 01 00 00        	movq	0x198(%rbp), %rcx
14155c6e4: 66 0f 6e c6                 	movd	%esi, %xmm0
14155c6e8: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14155c6ec: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14155c6f1: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14155c6f6: 48 8d 90 38 ff ff ff        	leaq	-0xc8(%rax), %rdx
14155c6fd: 45 31 c0                    	xorl	%r8d, %r8d
14155c700: 4d 89 e1                    	movq	%r12, %r9
14155c703: 49 21 c9                    	andq	%rcx, %r9
14155c706: f3 42 0f 6f 0c 08           	movdqu	(%rax,%r9), %xmm1
14155c70c: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14155c710: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14155c714: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14155c719: 45 85 d2                    	testl	%r10d, %r10d
14155c71c: 74 32                       	je	0x14155c750 <.text+0x155b750>
14155c71e: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14155c723: 4d 01 cb                    	addq	%r9, %r11
14155c726: 49 21 cb                    	andq	%rcx, %r11
14155c729: 49 f7 db                    	negq	%r11
14155c72c: 4d 69 db c8 00 00 00        	imulq	$0xc8, %r11, %r11
14155c733: 46 39 3c 1a                 	cmpl	%r15d, (%rdx,%r11)
14155c737: 74 37                       	je	0x14155c770 <.text+0x155b770>
14155c739: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14155c73d: 66 45 21 d3                 	andw	%r10w, %r11w
14155c741: 45 89 da                    	movl	%r11d, %r10d
14155c744: 75 d8                       	jne	0x14155c71e <.text+0x155b71e>
14155c746: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14155c750: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
14155c755: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
14155c75a: 45 85 d2                    	testl	%r10d, %r10d
14155c75d: 75 31                       	jne	0x14155c790 <.text+0x155b790>
14155c75f: 4d 01 c1                    	addq	%r8, %r9
14155c762: 49 83 c1 10                 	addq	$0x10, %r9
14155c766: 49 83 c0 10                 	addq	$0x10, %r8
14155c76a: eb 97                       	jmp	0x14155c703 <.text+0x155b703>
14155c76c: 0f 1f 40 00                 	nopl	(%rax)
14155c770: 4c 01 d8                    	addq	%r11, %rax
14155c773: 48 89 c1                    	movq	%rax, %rcx
14155c776: 4c 8b a5 08 03 00 00        	movq	0x308(%rbp), %r12
14155c77d: 44 0f b6 95 f8 02 00 00     	movzbl	0x2f8(%rbp), %r10d
14155c785: e9 a1 00 00 00              	jmp	0x14155c82b <.text+0x155b82b>
14155c78a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14155c790: 48 83 bd a0 01 00 00 00     	cmpq	$0x0, 0x1a0(%rbp)
14155c798: 0f 84 1f 06 00 00           	je	0x14155cdbd <.text+0x155bdbd>
14155c79e: 49 21 cc                    	andq	%rcx, %r12
14155c7a1: f3 42 0f 6f 04 20           	movdqu	(%rax,%r12), %xmm0
14155c7a7: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
14155c7ab: 85 d2                       	testl	%edx, %edx
14155c7ad: 0f 84 de 05 00 00           	je	0x14155cd91 <.text+0x155bd91>
14155c7b3: 44 0f b6 95 f8 02 00 00     	movzbl	0x2f8(%rbp), %r10d
14155c7bb: f3 0f bc d2                 	tzcntl	%edx, %edx
14155c7bf: 4c 01 e2                    	addq	%r12, %rdx
14155c7c2: 48 21 ca                    	andq	%rcx, %rdx
14155c7c5: 44 0f b6 04 10              	movzbl	(%rax,%rdx), %r8d
14155c7ca: 45 84 c0                    	testb	%r8b, %r8b
14155c7cd: 0f 89 1f 06 00 00           	jns	0x14155cdf2 <.text+0x155bdf2>
14155c7d3: 4c 8d 4a f0                 	leaq	-0x10(%rdx), %r9
14155c7d7: 49 21 c9                    	andq	%rcx, %r9
14155c7da: 40 88 34 10                 	movb	%sil, (%rax,%rdx)
14155c7de: 42 88 74 08 10              	movb	%sil, 0x10(%rax,%r9)
14155c7e3: 48 f7 da                    	negq	%rdx
14155c7e6: 41 80 e0 01                 	andb	$0x1, %r8b
14155c7ea: 41 0f b6 c8                 	movzbl	%r8b, %ecx
14155c7ee: 48 29 8d a0 01 00 00        	subq	%rcx, 0x1a0(%rbp)
14155c7f5: 48 69 d2 c8 00 00 00        	imulq	$0xc8, %rdx, %rdx
14155c7fc: 48 8d 0c 10                 	leaq	(%rax,%rdx), %rcx
14155c800: 44 89 bc 10 38 ff ff ff     	movl	%r15d, -0xc8(%rax,%rdx)
14155c808: 48 c7 84 10 40 ff ff ff 03 00 00 00 	movq	$0x3, -0xc0(%rax,%rdx)
14155c814: 48 c7 44 10 a0 03 00 00 00  	movq	$0x3, -0x60(%rax,%rdx)
14155c81d: 48 ff 85 a8 01 00 00        	incq	0x1a8(%rbp)
14155c824: 4c 8b a5 08 03 00 00        	movq	0x308(%rbp), %r12
14155c82b: 4c 89 61 a0                 	movq	%r12, -0x60(%rcx)
14155c82f: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14155c836: 48 89 41 a8                 	movq	%rax, -0x58(%rcx)
14155c83a: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
14155c83e: 0f 28 4d 30                 	movaps	0x30(%rbp), %xmm1
14155c842: 0f 11 41 b0                 	movups	%xmm0, -0x50(%rcx)
14155c846: 0f 11 49 c0                 	movups	%xmm1, -0x40(%rcx)
14155c84a: 48 8b 45 40                 	movq	0x40(%rbp), %rax
14155c84e: 48 89 41 d0                 	movq	%rax, -0x30(%rcx)
14155c852: 44 0f 13 51 d8              	movlps	%xmm10, -0x28(%rcx)
14155c857: f3 44 0f 11 59 e0           	movss	%xmm11, -0x20(%rcx)
14155c85d: 8b 85 a0 02 00 00           	movl	0x2a0(%rbp), %eax
14155c863: 89 41 e4                    	movl	%eax, -0x1c(%rcx)
14155c866: 45 0f 16 ee                 	movlhps	%xmm14, %xmm13          # xmm13 = xmm13[0],xmm14[0]
14155c86a: 44 0f 11 69 e8              	movups	%xmm13, -0x18(%rcx)
14155c86f: f3 44 0f 11 61 f8           	movss	%xmm12, -0x8(%rcx)
14155c875: 44 88 51 fc                 	movb	%r10b, -0x4(%rcx)
14155c879: 0f b6 85 90 02 00 00        	movzbl	0x290(%rbp), %eax
14155c880: 88 41 fd                    	movb	%al, -0x3(%rcx)
14155c883: 0f b6 85 98 02 00 00        	movzbl	0x298(%rbp), %eax
14155c88a: 88 41 fe                    	movb	%al, -0x2(%rcx)
14155c88d: 41 0f b6 46 fa              	movzbl	-0x6(%r14), %eax
14155c892: 8d 48 c8                    	leal	-0x38(%rax), %ecx
14155c895: 80 f9 08                    	cmpb	$0x8, %cl
14155c898: 0f b6 c9                    	movzbl	%cl, %ecx
14155c89b: ba 06 00 00 00              	movl	$0x6, %edx
14155c8a0: 0f 43 ca                    	cmovael	%edx, %ecx
14155c8a3: 31 d2                       	xorl	%edx, %edx
14155c8a5: 0f b6 c9                    	movzbl	%cl, %ecx
14155c8a8: 4c 8d 05 95 1f 71 01        	leaq	0x1711f95(%rip), %r8    # 0x142c6e844
14155c8af: 49 63 0c 88                 	movslq	(%r8,%rcx,4), %rcx
14155c8b3: 4c 01 c1                    	addq	%r8, %rcx
14155c8b6: ff e1                       	jmpq	*%rcx
14155c8b8: 45 8b 4e e4                 	movl	-0x1c(%r14), %r9d
14155c8bc: 44 89 ca                    	movl	%r9d, %edx
14155c8bf: 81 e2 00 ff ff ff           	andl	$0xffffff00, %edx       # imm = 0xFFFFFF00
14155c8c5: b1 06                       	movb	$0x6, %cl
14155c8c7: eb 67                       	jmp	0x14155c930 <.text+0x155b930>
14155c8c9: 45 8b 4e e8                 	movl	-0x18(%r14), %r9d
14155c8cd: 44 89 ca                    	movl	%r9d, %edx
14155c8d0: 81 e2 00 ff ff ff           	andl	$0xffffff00, %edx       # imm = 0xFFFFFF00
14155c8d6: b1 07                       	movb	$0x7, %cl
14155c8d8: eb 56                       	jmp	0x14155c930 <.text+0x155b930>
14155c8da: 45 8b 4e e0                 	movl	-0x20(%r14), %r9d
14155c8de: 45 0f b6 5e fd              	movzbl	-0x3(%r14), %r11d
14155c8e3: 44 89 ca                    	movl	%r9d, %edx
14155c8e6: 81 e2 00 ff ff ff           	andl	$0xffffff00, %edx       # imm = 0xFFFFFF00
14155c8ec: b1 04                       	movb	$0x4, %cl
14155c8ee: eb 40                       	jmp	0x14155c930 <.text+0x155b930>
14155c8f0: 45 8b 4e e0                 	movl	-0x20(%r14), %r9d
14155c8f4: 45 8b 5e ec                 	movl	-0x14(%r14), %r11d
14155c8f8: 45 0f b6 46 fb              	movzbl	-0x5(%r14), %r8d
14155c8fd: 41 0f b6 4e fe              	movzbl	-0x2(%r14), %ecx
14155c902: f2 41 0f 10 46 f0           	movsd	-0x10(%r14), %xmm0
14155c908: 44 89 ca                    	movl	%r9d, %edx
14155c90b: 81 e2 00 ff ff ff           	andl	$0xffffff00, %edx       # imm = 0xFFFFFF00
14155c911: 45 89 da                    	movl	%r11d, %r10d
14155c914: 41 c1 ea 08                 	shrl	$0x8, %r10d
14155c918: eb 16                       	jmp	0x14155c930 <.text+0x155b930>
14155c91a: 45 0f b6 4e fc              	movzbl	-0x4(%r14), %r9d
14155c91f: b1 05                       	movb	$0x5, %cl
14155c921: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14155c930: 45 0f b6 c9                 	movzbl	%r9b, %r9d
14155c934: 41 09 d1                    	orl	%edx, %r9d
14155c937: 41 0f b7 56 f8              	movzwl	-0x8(%r14), %edx
14155c93c: 49 8b 76 d8                 	movq	-0x28(%r14), %rsi
14155c940: 0f 10 0f                    	movups	(%rdi), %xmm1
14155c943: f3 0f 6f 57 10              	movdqu	0x10(%rdi), %xmm2
14155c948: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
14155c94f: 66 0f 7f 95 50 01 00 00     	movdqa	%xmm2, 0x150(%rbp)
14155c957: 48 8b 7f 20                 	movq	0x20(%rdi), %rdi
14155c95b: 48 89 bd 60 01 00 00        	movq	%rdi, 0x160(%rbp)
14155c962: 88 85 86 01 00 00           	movb	%al, 0x186(%rbp)
14155c968: c6 85 87 01 00 00 00        	movb	$0x0, 0x187(%rbp)
14155c96f: 66 89 95 84 01 00 00        	movw	%dx, 0x184(%rbp)
14155c976: 48 89 b5 68 01 00 00        	movq	%rsi, 0x168(%rbp)
14155c97d: 44 89 8d 70 01 00 00        	movl	%r9d, 0x170(%rbp)
14155c984: 44 88 9d 74 01 00 00        	movb	%r11b, 0x174(%rbp)
14155c98b: 66 44 89 95 75 01 00 00     	movw	%r10w, 0x175(%rbp)
14155c993: 41 c1 ea 10                 	shrl	$0x10, %r10d
14155c997: 44 88 95 77 01 00 00        	movb	%r10b, 0x177(%rbp)
14155c99e: 0f 13 85 78 01 00 00        	movlps	%xmm0, 0x178(%rbp)
14155c9a5: 44 88 85 80 01 00 00        	movb	%r8b, 0x180(%rbp)
14155c9ac: 88 8d 81 01 00 00           	movb	%cl, 0x181(%rbp)
14155c9b2: 48 8b 85 d0 02 00 00        	movq	0x2d0(%rbp), %rax
14155c9b9: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
14155c9c0: 4c 89 a5 e0 02 00 00        	movq	%r12, 0x2e0(%rbp)
14155c9c7: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14155c9ce: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
14155c9d5: 44 89 bd 14 03 00 00        	movl	%r15d, 0x314(%rbp)
14155c9dc: 48 8b b5 d8 02 00 00        	movq	0x2d8(%rbp), %rsi
14155c9e3: f3 0f 6f 46 20              	movdqu	0x20(%rsi), %xmm0
14155c9e8: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14155c9ed: 66 0f ef ce                 	pxor	%xmm6, %xmm1
14155c9f1: 66 0f 7f 8d c0 01 00 00     	movdqa	%xmm1, 0x1c0(%rbp)
14155c9f9: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14155c9fe: 66 0f ef cf                 	pxor	%xmm7, %xmm1
14155ca02: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
14155ca0a: 66 0f 7f 85 e0 01 00 00     	movdqa	%xmm0, 0x1e0(%rbp)
14155ca12: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
14155ca19: 44 0f 29 40 20              	movaps	%xmm8, 0x20(%rax)
14155ca1e: 48 c7 40 30 00 00 00 00     	movq	$0x0, 0x30(%rax)
14155ca26: 44 89 7d 20                 	movl	%r15d, 0x20(%rbp)
14155ca2a: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14155ca30: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14155ca37: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
14155ca3b: e8 20 97 ba fe              	callq	0x140106160 <.text+0x105160>
14155ca40: 4c 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %r9
14155ca47: 49 c1 e1 38                 	shlq	$0x38, %r9
14155ca4b: 4c 0b 8d f8 01 00 00        	orq	0x1f8(%rbp), %r9
14155ca52: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
14155ca59: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
14155ca60: 4c 31 c8                    	xorq	%r9, %rax
14155ca63: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
14155ca6a: 48 01 ca                    	addq	%rcx, %rdx
14155ca6d: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
14155ca74: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155ca78: 49 01 c0                    	addq	%rax, %r8
14155ca7b: 48 31 d1                    	xorq	%rdx, %rcx
14155ca7e: 48 c1 c0 10                 	rolq	$0x10, %rax
14155ca82: 4c 31 c0                    	xorq	%r8, %rax
14155ca85: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155ca89: 49 01 c8                    	addq	%rcx, %r8
14155ca8c: 48 01 c2                    	addq	%rax, %rdx
14155ca8f: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155ca93: 4c 31 c1                    	xorq	%r8, %rcx
14155ca96: 48 c1 c0 15                 	rolq	$0x15, %rax
14155ca9a: 49 c1 c0 20                 	rolq	$0x20, %r8
14155ca9e: 48 31 d0                    	xorq	%rdx, %rax
14155caa1: 4c 31 ca                    	xorq	%r9, %rdx
14155caa4: 49 81 f0 ff 00 00 00        	xorq	$0xff, %r8
14155caab: 48 01 ca                    	addq	%rcx, %rdx
14155caae: 49 01 c0                    	addq	%rax, %r8
14155cab1: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155cab5: 48 31 d1                    	xorq	%rdx, %rcx
14155cab8: 48 c1 c0 10                 	rolq	$0x10, %rax
14155cabc: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155cac0: 4c 31 c0                    	xorq	%r8, %rax
14155cac3: 49 01 c8                    	addq	%rcx, %r8
14155cac6: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155caca: 48 01 c2                    	addq	%rax, %rdx
14155cacd: 4c 31 c1                    	xorq	%r8, %rcx
14155cad0: 48 c1 c0 15                 	rolq	$0x15, %rax
14155cad4: 48 31 d0                    	xorq	%rdx, %rax
14155cad7: 49 c1 c0 20                 	rolq	$0x20, %r8
14155cadb: 48 01 ca                    	addq	%rcx, %rdx
14155cade: 49 01 c0                    	addq	%rax, %r8
14155cae1: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155cae5: 48 31 d1                    	xorq	%rdx, %rcx
14155cae8: 48 c1 c0 10                 	rolq	$0x10, %rax
14155caec: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155caf0: 4c 31 c0                    	xorq	%r8, %rax
14155caf3: 49 01 c8                    	addq	%rcx, %r8
14155caf6: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155cafa: 48 01 c2                    	addq	%rax, %rdx
14155cafd: 4c 31 c1                    	xorq	%r8, %rcx
14155cb00: 48 c1 c0 15                 	rolq	$0x15, %rax
14155cb04: 48 31 d0                    	xorq	%rdx, %rax
14155cb07: 49 c1 c0 20                 	rolq	$0x20, %r8
14155cb0b: 48 01 ca                    	addq	%rcx, %rdx
14155cb0e: 49 01 c0                    	addq	%rax, %r8
14155cb11: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155cb15: 48 31 d1                    	xorq	%rdx, %rcx
14155cb18: 48 c1 c0 10                 	rolq	$0x10, %rax
14155cb1c: 4c 31 c0                    	xorq	%r8, %rax
14155cb1f: 49 01 c8                    	addq	%rcx, %r8
14155cb22: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155cb26: 48 c1 c0 15                 	rolq	$0x15, %rax
14155cb2a: 4c 89 c2                    	movq	%r8, %rdx
14155cb2d: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155cb31: 48 31 ca                    	xorq	%rcx, %rdx
14155cb34: 48 31 c2                    	xorq	%rax, %rdx
14155cb37: 4c 31 c2                    	xorq	%r8, %rdx
14155cb3a: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cb41: 48 89 f1                    	movq	%rsi, %rcx
14155cb44: 4c 8d 85 14 03 00 00        	leaq	0x314(%rbp), %r8
14155cb4b: e8 80 ab 61 ff              	callq	0x140b776d0 <.text+0xb766d0>
14155cb50: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cb57: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
14155cb5e: 44 89 fa                    	movl	%r15d, %edx
14155cb61: e8 9a c6 d6 fe              	callq	0x1402c9200 <.text+0x2c8200>
14155cb66: 44 8b 85 14 03 00 00        	movl	0x314(%rbp), %r8d
14155cb6d: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cb74: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14155cb78: 48 8b 95 b0 02 00 00        	movq	0x2b0(%rbp), %rdx
14155cb7f: 4c 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %r9
14155cb86: e8 15 f4 c9 ff              	callq	0x1411fbfa0 <.text+0x11fafa0>
14155cb8b: 80 7d 67 02                 	cmpb	$0x2, 0x67(%rbp)
14155cb8f: 44 8b bd 14 03 00 00        	movl	0x314(%rbp), %r15d
14155cb96: 75 48                       	jne	0x14155cbe0 <.text+0x155bbe0>
14155cb98: 44 89 bd 88 02 00 00        	movl	%r15d, 0x288(%rbp)
14155cb9f: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
14155cba6: 4c 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %r8
14155cbad: 4c 8b 8d e8 02 00 00        	movq	0x2e8(%rbp), %r9
14155cbb4: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cbbb: 44 89 fa                    	movl	%r15d, %edx
14155cbbe: e8 9d f0 fc ff              	callq	0x14152bc60 <.text+0x152ac60>
14155cbc3: 48 83 f8 03                 	cmpq	$0x3, %rax
14155cbc7: 0f 84 99 f8 ff ff           	je	0x14155c466 <.text+0x155b466>
14155cbcd: e9 e4 05 00 00              	jmp	0x14155d1b6 <.text+0x155c1b6>
14155cbd2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14155cbe0: 44 89 bd e8 00 00 00        	movl	%r15d, 0xe8(%rbp)
14155cbe7: 48 8b bd f0 02 00 00        	movq	0x2f0(%rbp), %rdi
14155cbee: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
14155cbf3: 0f 84 2f 02 00 00           	je	0x14155ce28 <.text+0x155be28>
14155cbf9: 44 0f 28 95 e0 02 00 00     	movaps	0x2e0(%rbp), %xmm10
14155cc01: f3 0f 6f 47 20              	movdqu	0x20(%rdi), %xmm0
14155cc06: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14155cc0b: 66 0f ef ce                 	pxor	%xmm6, %xmm1
14155cc0f: 66 0f 7f 8d c0 01 00 00     	movdqa	%xmm1, 0x1c0(%rbp)
14155cc17: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14155cc1c: 66 0f ef cf                 	pxor	%xmm7, %xmm1
14155cc20: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
14155cc28: 66 0f 7f 85 e0 01 00 00     	movdqa	%xmm0, 0x1e0(%rbp)
14155cc30: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
14155cc37: 44 0f 29 40 20              	movaps	%xmm8, 0x20(%rax)
14155cc3c: 48 c7 40 30 00 00 00 00     	movq	$0x0, 0x30(%rax)
14155cc44: 44 89 bd f8 00 00 00        	movl	%r15d, 0xf8(%rbp)
14155cc4b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14155cc51: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14155cc58: 48 8d 95 f8 00 00 00        	leaq	0xf8(%rbp), %rdx
14155cc5f: e8 fc 94 ba fe              	callq	0x140106160 <.text+0x105160>
14155cc64: 4c 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %r9
14155cc6b: 49 c1 e1 38                 	shlq	$0x38, %r9
14155cc6f: 4c 0b 8d f8 01 00 00        	orq	0x1f8(%rbp), %r9
14155cc76: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
14155cc7d: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
14155cc84: 4c 31 c8                    	xorq	%r9, %rax
14155cc87: 4c 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %r8
14155cc8e: 49 01 c8                    	addq	%rcx, %r8
14155cc91: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
14155cc98: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155cc9c: 48 01 c2                    	addq	%rax, %rdx
14155cc9f: 4c 31 c1                    	xorq	%r8, %rcx
14155cca2: 48 c1 c0 10                 	rolq	$0x10, %rax
14155cca6: 48 31 d0                    	xorq	%rdx, %rax
14155cca9: 49 c1 c0 20                 	rolq	$0x20, %r8
14155ccad: 48 01 ca                    	addq	%rcx, %rdx
14155ccb0: 49 01 c0                    	addq	%rax, %r8
14155ccb3: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155ccb7: 48 31 d1                    	xorq	%rdx, %rcx
14155ccba: 48 c1 c0 15                 	rolq	$0x15, %rax
14155ccbe: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155ccc2: 4c 31 c0                    	xorq	%r8, %rax
14155ccc5: 4d 31 c8                    	xorq	%r9, %r8
14155ccc8: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
14155cccf: 49 01 c8                    	addq	%rcx, %r8
14155ccd2: 48 01 c2                    	addq	%rax, %rdx
14155ccd5: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155ccd9: 4c 31 c1                    	xorq	%r8, %rcx
14155ccdc: 48 c1 c0 10                 	rolq	$0x10, %rax
14155cce0: 49 c1 c0 20                 	rolq	$0x20, %r8
14155cce4: 48 31 d0                    	xorq	%rdx, %rax
14155cce7: 48 01 ca                    	addq	%rcx, %rdx
14155ccea: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155ccee: 49 01 c0                    	addq	%rax, %r8
14155ccf1: 48 31 d1                    	xorq	%rdx, %rcx
14155ccf4: 48 c1 c0 15                 	rolq	$0x15, %rax
14155ccf8: 4c 31 c0                    	xorq	%r8, %rax
14155ccfb: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155ccff: 49 01 c8                    	addq	%rcx, %r8
14155cd02: 48 01 c2                    	addq	%rax, %rdx
14155cd05: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155cd09: 4c 31 c1                    	xorq	%r8, %rcx
14155cd0c: 48 c1 c0 10                 	rolq	$0x10, %rax
14155cd10: 49 c1 c0 20                 	rolq	$0x20, %r8
14155cd14: 48 31 d0                    	xorq	%rdx, %rax
14155cd17: 48 01 ca                    	addq	%rcx, %rdx
14155cd1a: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155cd1e: 49 01 c0                    	addq	%rax, %r8
14155cd21: 48 31 d1                    	xorq	%rdx, %rcx
14155cd24: 48 c1 c0 15                 	rolq	$0x15, %rax
14155cd28: 4c 31 c0                    	xorq	%r8, %rax
14155cd2b: 48 c1 c2 20                 	rolq	$0x20, %rdx
14155cd2f: 49 01 c8                    	addq	%rcx, %r8
14155cd32: 48 01 c2                    	addq	%rax, %rdx
14155cd35: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14155cd39: 4c 31 c1                    	xorq	%r8, %rcx
14155cd3c: 48 c1 c0 10                 	rolq	$0x10, %rax
14155cd40: 48 31 d0                    	xorq	%rdx, %rax
14155cd43: 48 01 ca                    	addq	%rcx, %rdx
14155cd46: 48 c1 c1 11                 	rolq	$0x11, %rcx
14155cd4a: 48 c1 c0 15                 	rolq	$0x15, %rax
14155cd4e: 49 89 d0                    	movq	%rdx, %r8
14155cd51: 49 c1 c0 20                 	rolq	$0x20, %r8
14155cd55: 49 31 c8                    	xorq	%rcx, %r8
14155cd58: 49 31 c0                    	xorq	%rax, %r8
14155cd5b: 49 31 d0                    	xorq	%rdx, %r8
14155cd5e: 4c 89 c2                    	movq	%r8, %rdx
14155cd61: 48 c1 ea 39                 	shrq	$0x39, %rdx
14155cd65: 48 8b 07                    	movq	(%rdi), %rax
14155cd68: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
14155cd6c: 66 0f 6e c2                 	movd	%edx, %xmm0
14155cd70: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14155cd74: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14155cd79: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14155cd7e: 48 8d 50 e8                 	leaq	-0x18(%rax), %rdx
14155cd82: 45 31 c9                    	xorl	%r9d, %r9d
14155cd85: e9 76 f6 ff ff              	jmp	0x14155c400 <.text+0x155b400>
14155cd8a: b1 03                       	movb	$0x3, %cl
14155cd8c: e9 9f fb ff ff              	jmp	0x14155c930 <.text+0x155b930>
14155cd91: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14155cd97: 44 0f b6 95 f8 02 00 00     	movzbl	0x2f8(%rbp), %r10d
14155cd9f: 4d 01 c4                    	addq	%r8, %r12
14155cda2: 49 21 cc                    	andq	%rcx, %r12
14155cda5: f3 42 0f 6f 04 20           	movdqu	(%rax,%r12), %xmm0
14155cdab: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
14155cdaf: 49 83 c0 10                 	addq	$0x10, %r8
14155cdb3: 85 d2                       	testl	%edx, %edx
14155cdb5: 0f 85 00 fa ff ff           	jne	0x14155c7bb <.text+0x155b7bb>
14155cdbb: eb e2                       	jmp	0x14155cd9f <.text+0x155bd9f>
14155cdbd: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cdc4: ba 01 00 00 00              	movl	$0x1, %edx
14155cdc9: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
14155cdd0: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
14155cdd7: 41 b1 01                    	movb	$0x1, %r9b
14155cdda: e8 a1 ab 35 01              	callq	0x1428b7980 <.text+0x28b6980>
14155cddf: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
14155cde6: 48 8b 8d 98 01 00 00        	movq	0x198(%rbp), %rcx
14155cded: e9 ac f9 ff ff              	jmp	0x14155c79e <.text+0x155b79e>
14155cdf2: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
14155cdf6: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
14155cdfa: f3 0f bc d2                 	tzcntl	%edx, %edx
14155cdfe: 44 0f b6 04 10              	movzbl	(%rax,%rdx), %r8d
14155ce03: e9 cb f9 ff ff              	jmp	0x14155c7d3 <.text+0x155b7d3>
14155ce08: 8b 85 8c 02 00 00           	movl	0x28c(%rbp), %eax
14155ce0e: 4c 8b 9d b8 02 00 00        	movq	0x2b8(%rbp), %r11
14155ce15: 4c 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %r8
14155ce1c: 4c 8b 8d 38 01 00 00        	movq	0x138(%rbp), %r9
14155ce23: e9 c2 f4 ff ff              	jmp	0x14155c2ea <.text+0x155b2ea>
14155ce28: 48 8d 85 e8 00 00 00        	leaq	0xe8(%rbp), %rax
14155ce2f: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
14155ce36: 48 8d 05 f3 74 4c ff        	leaq	-0xb38b0d(%rip), %rax   # 0x140a24330 <.text+0xa23330>
14155ce3d: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
14155ce44: 48 8d 85 e0 02 00 00        	leaq	0x2e0(%rbp), %rax
14155ce4b: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
14155ce52: 48 8d 05 17 75 4c ff        	leaq	-0xb38ae9(%rip), %rax   # 0x140a24370 <.text+0xa23370>
14155ce59: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
14155ce60: 48 8d 05 c9 c1 56 01        	leaq	0x156c1c9(%rip), %rax   # 0x142ac9030
14155ce67: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14155ce6e: 48 c7 85 c8 01 00 00 02 00 00 00    	movq	$0x2, 0x1c8(%rbp)
14155ce79: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
14155ce84: 48 8d 85 f8 00 00 00        	leaq	0xf8(%rbp), %rax
14155ce8b: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
14155ce92: 48 c7 85 d8 01 00 00 02 00 00 00    	movq	$0x2, 0x1d8(%rbp)
14155ce9d: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cea4: 48 8d 15 a5 c1 56 01        	leaq	0x156c1a5(%rip), %rdx   # 0x142ac9050
14155ceab: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14155ceb2: e8 d9 c4 37 01              	callq	0x1428d9390 <.text+0x28d8390>
14155ceb7: e9 ac 03 00 00              	jmp	0x14155d268 <.text+0x155c268>
14155cebc: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cec3: 48 8d 0d 9e c1 56 01        	leaq	0x156c19e(%rip), %rcx   # 0x142ac9068
14155ceca: 4c 8d 05 87 13 71 01        	leaq	0x1711387(%rip), %r8    # 0x142c6e258
14155ced1: ba 30 00 00 00              	movl	$0x30, %edx
14155ced6: e8 55 c5 37 01              	callq	0x1428d9430 <.text+0x28d8430>
14155cedb: e9 88 03 00 00              	jmp	0x14155d268 <.text+0x155c268>
14155cee0: 48 8b bd 90 01 00 00        	movq	0x190(%rbp), %rdi
14155cee7: 48 8b 9d a8 01 00 00        	movq	0x1a8(%rbp), %rbx
14155ceee: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
14155cef2: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155cef6: f7 d0                       	notl	%eax
14155cef8: 4c 8d 77 10                 	leaq	0x10(%rdi), %r14
14155cefc: 48 8b 8d 38 04 00 00        	movq	0x438(%rbp), %rcx
14155cf03: 48 8d 71 30                 	leaq	0x30(%rcx), %rsi
14155cf07: 4c 8d a5 c0 01 00 00        	leaq	0x1c0(%rbp), %r12
14155cf0e: eb 03                       	jmp	0x14155cf13 <.text+0x155bf13>
14155cf10: 44 89 f8                    	movl	%r15d, %eax
14155cf13: 48 85 db                    	testq	%rbx, %rbx
14155cf16: 0f 84 de 01 00 00           	je	0x14155d0fa <.text+0x155c0fa>
14155cf1c: 66 85 c0                    	testw	%ax, %ax
14155cf1f: 74 0f                       	je	0x14155cf30 <.text+0x155bf30>
14155cf21: 44 8d 78 ff                 	leal	-0x1(%rax), %r15d
14155cf25: 41 21 c7                    	andl	%eax, %r15d
14155cf28: eb 30                       	jmp	0x14155cf5a <.text+0x155bf5a>
14155cf2a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14155cf30: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
14155cf35: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14155cf39: 48 81 c7 80 f3 ff ff        	addq	$-0xc80, %rdi           # imm = 0xF380
14155cf40: 49 83 c6 10                 	addq	$0x10, %r14
14155cf44: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
14155cf49: 74 e5                       	je	0x14155cf30 <.text+0x155bf30>
14155cf4b: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14155cf50: 29 c1                       	subl	%eax, %ecx
14155cf52: f7 d0                       	notl	%eax
14155cf54: 41 89 c7                    	movl	%eax, %r15d
14155cf57: 41 21 cf                    	andl	%ecx, %r15d
14155cf5a: 48 ff cb                    	decq	%rbx
14155cf5d: f3 0f bc c0                 	tzcntl	%eax, %eax
14155cf61: 48 f7 d8                    	negq	%rax
14155cf64: 4c 69 c0 c8 00 00 00        	imulq	$0xc8, %rax, %r8
14155cf6b: 4a 8b 94 07 40 ff ff ff     	movq	-0xc0(%rdi,%r8), %rdx
14155cf73: 44 89 f8                    	movl	%r15d, %eax
14155cf76: 48 83 fa 03                 	cmpq	$0x3, %rdx
14155cf7a: 74 97                       	je	0x14155cf13 <.text+0x155bf13>
14155cf7c: 4a 8d 0c 07                 	leaq	(%rdi,%r8), %rcx
14155cf80: 46 8b ac 07 38 ff ff ff     	movl	-0xc8(%rdi,%r8), %r13d
14155cf88: 48 8b 41 a0                 	movq	-0x60(%rcx), %rax
14155cf8c: 48 83 f8 03                 	cmpq	$0x3, %rax
14155cf90: 0f 85 aa 00 00 00           	jne	0x14155d040 <.text+0x155c040>
14155cf96: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cf9d: 4c 89 e1                    	movq	%r12, %rcx
14155cfa0: 48 8b 95 30 04 00 00        	movq	0x430(%rbp), %rdx
14155cfa7: 4c 8b 85 38 04 00 00        	movq	0x438(%rbp), %r8
14155cfae: 45 89 e9                    	movl	%r13d, %r9d
14155cfb1: e8 fa 39 5e ff              	callq	0x140b409b0 <.text+0xb3f9b0>
14155cfb6: 80 bd e0 01 00 00 11        	cmpb	$0x11, 0x1e0(%rbp)
14155cfbd: 74 20                       	je	0x14155cfdf <.text+0x155bfdf>
14155cfbf: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
14155cfc6: f0                          	lock
14155cfc7: 48 ff 08                    	decq	(%rax)
14155cfca: 75 13                       	jne	0x14155cfdf <.text+0x155bfdf>
14155cfcc: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cfd3: 48 8d 8d f8 01 00 00        	leaq	0x1f8(%rbp), %rcx
14155cfda: e8 41 f6 5f ff              	callq	0x140b5c620 <.text+0xb5b620>
14155cfdf: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155cfe6: 4c 89 e1                    	movq	%r12, %rcx
14155cfe9: 48 8b 95 30 04 00 00        	movq	0x430(%rbp), %rdx
14155cff0: 49 89 f0                    	movq	%rsi, %r8
14155cff3: 45 89 e9                    	movl	%r13d, %r9d
14155cff6: e8 b5 39 5e ff              	callq	0x140b409b0 <.text+0xb3f9b0>
14155cffb: 80 bd e0 01 00 00 11        	cmpb	$0x11, 0x1e0(%rbp)
14155d002: 0f 84 08 ff ff ff           	je	0x14155cf10 <.text+0x155bf10>
14155d008: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
14155d00f: f0                          	lock
14155d010: 48 ff 08                    	decq	(%rax)
14155d013: 0f 85 f7 fe ff ff           	jne	0x14155cf10 <.text+0x155bf10>
14155d019: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155d020: 48 8d 8d f8 01 00 00        	leaq	0x1f8(%rbp), %rcx
14155d027: e8 f4 f5 5f ff              	callq	0x140b5c620 <.text+0xb5b620>
14155d02c: e9 df fe ff ff              	jmp	0x14155cf10 <.text+0x155bf10>
14155d031: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14155d040: 39 c2                       	cmpl	%eax, %edx
14155d042: 75 20                       	jne	0x14155d064 <.text+0x155c064>
14155d044: 44 89 f8                    	movl	%r15d, %eax
14155d047: 48 85 d2                    	testq	%rdx, %rdx
14155d04a: 0f 85 c3 fe ff ff           	jne	0x14155cf13 <.text+0x155bf13>
14155d050: 48 8b 51 a8                 	movq	-0x58(%rcx), %rdx
14155d054: 44 89 f8                    	movl	%r15d, %eax
14155d057: 48 39 91 48 ff ff ff        	cmpq	%rdx, -0xb8(%rcx)
14155d05e: 0f 84 af fe ff ff           	je	0x14155cf13 <.text+0x155bf13>
14155d064: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155d06b: 4c 89 e1                    	movq	%r12, %rcx
14155d06e: 48 8b 95 30 04 00 00        	movq	0x430(%rbp), %rdx
14155d075: 4c 8b 85 38 04 00 00        	movq	0x438(%rbp), %r8
14155d07c: 45 89 e9                    	movl	%r13d, %r9d
14155d07f: e8 2c 39 5e ff              	callq	0x140b409b0 <.text+0xb3f9b0>
14155d084: 80 bd e0 01 00 00 11        	cmpb	$0x11, 0x1e0(%rbp)
14155d08b: 74 20                       	je	0x14155d0ad <.text+0x155c0ad>
14155d08d: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
14155d094: f0                          	lock
14155d095: 48 ff 08                    	decq	(%rax)
14155d098: 75 13                       	jne	0x14155d0ad <.text+0x155c0ad>
14155d09a: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155d0a1: 48 8d 8d f8 01 00 00        	leaq	0x1f8(%rbp), %rcx
14155d0a8: e8 73 f5 5f ff              	callq	0x140b5c620 <.text+0xb5b620>
14155d0ad: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155d0b4: 4c 89 e1                    	movq	%r12, %rcx
14155d0b7: 48 8b 95 30 04 00 00        	movq	0x430(%rbp), %rdx
14155d0be: 49 89 f0                    	movq	%rsi, %r8
14155d0c1: 45 89 e9                    	movl	%r13d, %r9d
14155d0c4: e8 e7 38 5e ff              	callq	0x140b409b0 <.text+0xb3f9b0>
14155d0c9: 80 bd e0 01 00 00 11        	cmpb	$0x11, 0x1e0(%rbp)
14155d0d0: 74 20                       	je	0x14155d0f2 <.text+0x155c0f2>
14155d0d2: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
14155d0d9: f0                          	lock
14155d0da: 48 ff 08                    	decq	(%rax)
14155d0dd: 75 13                       	jne	0x14155d0f2 <.text+0x155c0f2>
14155d0df: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155d0e6: 48 8d 8d f8 01 00 00        	leaq	0x1f8(%rbp), %rcx
14155d0ed: e8 2e f5 5f ff              	callq	0x140b5c620 <.text+0xb5b620>
14155d0f2: 44 89 f8                    	movl	%r15d, %eax
14155d0f5: e9 19 fe ff ff              	jmp	0x14155cf13 <.text+0x155bf13>
14155d0fa: 66 0f 6f 85 90 01 00 00     	movdqa	0x190(%rbp), %xmm0
14155d102: 66 0f 6f 8d a0 01 00 00     	movdqa	0x1a0(%rbp), %xmm1
14155d10a: 66 0f 6f 95 b0 01 00 00     	movdqa	0x1b0(%rbp), %xmm2
14155d112: 66 0f 7f 95 e0 01 00 00     	movdqa	%xmm2, 0x1e0(%rbp)
14155d11a: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
14155d122: 66 0f 7f 85 c0 01 00 00     	movdqa	%xmm0, 0x1c0(%rbp)
14155d12a: c6 85 27 03 00 00 00        	movb	$0x0, 0x327(%rbp)
14155d131: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14155d138: 48 8b 95 40 04 00 00        	movq	0x440(%rbp), %rdx
14155d13f: e8 9c 09 26 00              	callq	0x1417bdae0 <.text+0x17bcae0>
14155d144: 0f 28 b5 30 03 00 00        	movaps	0x330(%rbp), %xmm6
14155d14b: 0f 28 bd 40 03 00 00        	movaps	0x340(%rbp), %xmm7
14155d152: 44 0f 28 85 50 03 00 00     	movaps	0x350(%rbp), %xmm8
14155d15a: 44 0f 28 8d 60 03 00 00     	movaps	0x360(%rbp), %xmm9
14155d162: 44 0f 28 95 70 03 00 00     	movaps	0x370(%rbp), %xmm10
14155d16a: 44 0f 28 9d 80 03 00 00     	movaps	0x380(%rbp), %xmm11
14155d172: 44 0f 28 a5 90 03 00 00     	movaps	0x390(%rbp), %xmm12
14155d17a: 44 0f 28 ad a0 03 00 00     	movaps	0x3a0(%rbp), %xmm13
14155d182: 44 0f 28 b5 b0 03 00 00     	movaps	0x3b0(%rbp), %xmm14
14155d18a: 48 81 c4 48 04 00 00        	addq	$0x448, %rsp            # imm = 0x448
14155d191: 5b                          	popq	%rbx
14155d192: 5f                          	popq	%rdi
14155d193: 5e                          	popq	%rsi
14155d194: 41 5c                       	popq	%r12
14155d196: 41 5d                       	popq	%r13
14155d198: 41 5e                       	popq	%r14
14155d19a: 41 5f                       	popq	%r15
14155d19c: 5d                          	popq	%rbp
14155d19d: c3                          	retq
14155d19e: 48 8d 0d 13 11 71 01        	leaq	0x1711113(%rip), %rcx   # 0x142c6e2b8
14155d1a5: 4c 8d 05 34 11 71 01        	leaq	0x1711134(%rip), %r8    # 0x142c6e2e0
14155d1ac: ba 21 00 00 00              	movl	$0x21, %edx
14155d1b1: e8 7a c2 37 01              	callq	0x1428d9430 <.text+0x28d8430>
14155d1b6: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
14155d1bd: 48 89 95 f0 00 00 00        	movq	%rdx, 0xf0(%rbp)
14155d1c4: 48 8d 85 88 02 00 00        	leaq	0x288(%rbp), %rax
14155d1cb: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14155d1d2: 48 8d 05 57 71 4c ff        	leaq	-0xb38ea9(%rip), %rax   # 0x140a24330 <.text+0xa23330>
14155d1d9: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
14155d1e0: 48 8d 85 e8 00 00 00        	leaq	0xe8(%rbp), %rax
14155d1e7: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
14155d1ee: 48 8d 05 7b 71 4c ff        	leaq	-0xb38e85(%rip), %rax   # 0x140a24370 <.text+0xa23370>
14155d1f5: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
14155d1fc: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
14155d203: 48 89 8d e0 01 00 00        	movq	%rcx, 0x1e0(%rbp)
14155d20a: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
14155d211: 48 8d 05 d0 bd 56 01        	leaq	0x156bdd0(%rip), %rax   # 0x142ac8fe8
14155d218: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
14155d21f: 48 c7 85 00 01 00 00 03 00 00 00    	movq	$0x3, 0x100(%rbp)
14155d22a: 48 c7 85 18 01 00 00 00 00 00 00    	movq	$0x0, 0x118(%rbp)
14155d235: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
14155d23c: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
14155d243: 48 c7 85 10 01 00 00 03 00 00 00    	movq	$0x3, 0x110(%rbp)
14155d24e: c6 85 27 03 00 00 01        	movb	$0x1, 0x327(%rbp)
14155d255: 48 8d 15 cc 0f 71 01        	leaq	0x1710fcc(%rip), %rdx   # 0x142c6e228
14155d25c: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
14155d263: e8 28 c1 37 01              	callq	0x1428d9390 <.text+0x28d8390>
14155d268: 0f 0b                       	ud2
14155d26a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14155d270: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14155d275: 55                          	pushq	%rbp
14155d276: 41 57                       	pushq	%r15
14155d278: 41 56                       	pushq	%r14
14155d27a: 41 55                       	pushq	%r13
14155d27c: 41 54                       	pushq	%r12
14155d27e: 56                          	pushq	%rsi
14155d27f: 57                          	pushq	%rdi
14155d280: 53                          	pushq	%rbx
14155d281: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14155d288: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14155d28f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
14155d295: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
14155d29b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14155d2a1: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14155d2a7: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14155d2ad: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14155d2b4: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14155d2bd: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
14155d2c6: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14155d2cf: 80 bd 27 03 00 00 00        	cmpb	$0x0, 0x327(%rbp)
14155d2d6: 75 51                       	jne	0x14155d329 <.text+0x155c329>
14155d2d8: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
14155d2e0: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14155d2e8: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14155d2f1: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14155d2f7: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14155d2fd: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14155d303: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14155d309: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
14155d30f: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
14155d315: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14155d31c: 5b                          	popq	%rbx
14155d31d: 5f                          	popq	%rdi
14155d31e: 5e                          	popq	%rsi
14155d31f: 41 5c                       	popq	%r12
14155d321: 41 5d                       	popq	%r13
14155d323: 41 5e                       	popq	%r14
14155d325: 41 5f                       	popq	%r15
14155d327: 5d                          	popq	%rbp
14155d328: c3                          	retq
14155d329: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
14155d330: e8 6b 56 32 ff              	callq	0x1408829a0 <.text+0x8819a0>
14155d335: eb a1                       	jmp	0x14155d2d8 <.text+0x155c2d8>
