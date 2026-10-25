
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140922c30: 56                          	pushq	%rsi
140922c31: 57                          	pushq	%rdi
140922c32: 53                          	pushq	%rbx
140922c33: 48 83 ec 30                 	subq	$0x30, %rsp
140922c37: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
140922c3c: 48 8b 12                    	movq	(%rdx), %rdx
140922c3f: 83 3a 00                    	cmpl	$0x0, (%rdx)
140922c42: 74 76                       	je	0x140922cba <.text+0x921cba>
140922c44: 4c 8b 00                    	movq	(%rax), %r8
140922c47: 45 31 d2                    	xorl	%r10d, %r10d
140922c4a: 49 8b 11                    	movq	(%r9), %rdx
140922c4d: 49 b9 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %r9 # imm = 0x8000000000000002
140922c57: 4c 39 4a 40                 	cmpq	%r9, 0x40(%rdx)
140922c5b: 0f 85 b6 00 00 00           	jne	0x140922d17 <.text+0x921d17>
140922c61: 48 8b 4a 48                 	movq	0x48(%rdx), %rcx
140922c65: 48 39 4a 38                 	cmpq	%rcx, 0x38(%rdx)
140922c69: 0f 86 11 03 00 00           	jbe	0x140922f80 <.text+0x921f80>
140922c6f: 48 03 4a 30                 	addq	0x30(%rdx), %rcx
140922c73: 4c 8b 4a 20                 	movq	0x20(%rdx), %r9
140922c77: 48 8b 52 28                 	movq	0x28(%rdx), %rdx
140922c7b: 45 31 db                    	xorl	%r11d, %r11d
140922c7e: 4c 39 c9                    	cmpq	%r9, %rcx
140922c81: 4d 0f 43 d9                 	cmovaeq	%r9, %r11
140922c85: 4c 29 d9                    	subq	%r11, %rcx
140922c88: 48 69 c9 d0 00 00 00        	imulq	$0xd0, %rcx, %rcx
140922c8f: f3 0f 10 84 0a c8 00 00 00  	movss	0xc8(%rdx,%rcx), %xmm0
140922c98: 44 8b 48 1c                 	movl	0x1c(%rax), %r9d
140922c9c: 4c 8b 58 10                 	movq	0x10(%rax), %r11
140922ca0: f3 41 0f 11 40 28           	movss	%xmm0, 0x28(%r8)
140922ca6: 45 89 0b                    	movl	%r9d, (%r11)
140922ca9: 0f 57 c9                    	xorps	%xmm1, %xmm1
140922cac: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
140922caf: 0f 86 d3 00 00 00           	jbe	0x140922d88 <.text+0x921d88>
140922cb5: e9 9f 02 00 00              	jmp	0x140922f59 <.text+0x921f59>
140922cba: 49 8b 10                    	movq	(%r8), %rdx
140922cbd: 4c 8b 02                    	movq	(%rdx), %r8
140922cc0: 49 83 f8 0a                 	cmpq	$0xa, %r8
140922cc4: 74 24                       	je	0x140922cea <.text+0x921cea>
140922cc6: 41 83 f8 07                 	cmpl	$0x7, %r8d
140922cca: 74 0f                       	je	0x140922cdb <.text+0x921cdb>
140922ccc: 41 83 f8 02                 	cmpl	$0x2, %r8d
140922cd0: 0f 85 6e ff ff ff           	jne	0x140922c44 <.text+0x921c44>
140922cd6: 4c 8b 00                    	movq	(%rax), %r8
140922cd9: eb 1c                       	jmp	0x140922cf7 <.text+0x921cf7>
140922cdb: 80 7a 08 01                 	cmpb	$0x1, 0x8(%rdx)
140922cdf: 4c 8b 00                    	movq	(%rax), %r8
140922ce2: 0f 85 5f ff ff ff           	jne	0x140922c47 <.text+0x921c47>
140922ce8: eb 0d                       	jmp	0x140922cf7 <.text+0x921cf7>
140922cea: 4c 8b 00                    	movq	(%rax), %r8
140922ced: 80 7a 08 00                 	cmpb	$0x0, 0x8(%rdx)
140922cf1: 0f 85 50 ff ff ff           	jne	0x140922c47 <.text+0x921c47>
140922cf7: 45 0f b6 50 31              	movzbl	0x31(%r8), %r10d
140922cfc: 41 80 f2 01                 	xorb	$0x1, %r10b
140922d00: 49 8b 11                    	movq	(%r9), %rdx
140922d03: 49 b9 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %r9 # imm = 0x8000000000000002
140922d0d: 4c 39 4a 40                 	cmpq	%r9, 0x40(%rdx)
140922d11: 0f 84 4a ff ff ff           	je	0x140922c61 <.text+0x921c61>
140922d17: 48 8b 09                    	movq	(%rcx), %rcx
140922d1a: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
140922d1e: 48 85 d2                    	testq	%rdx, %rdx
140922d21: 78 26                       	js	0x140922d49 <.text+0x921d49>
140922d23: f3 48 0f 2a ca              	cvtsi2ss	%rdx, %xmm1
140922d28: 41 8b 50 2c                 	movl	0x2c(%r8), %edx
140922d2c: 45 84 d2                    	testb	%r10b, %r10b
140922d2f: 74 36                       	je	0x140922d67 <.text+0x921d67>
140922d31: f3 0f 2a 41 18              	cvtsi2ssl	0x18(%rcx), %xmm0
140922d36: f3 0f 5e 05 02 49 08 02     	divss	0x2084902(%rip), %xmm0  # 0x1429a7640
140922d3e: f3 0f 58 c1                 	addss	%xmm1, %xmm0
140922d42: 83 fa 01                    	cmpl	$0x1, %edx
140922d45: 76 28                       	jbe	0x140922d6f <.text+0x921d6f>
140922d47: eb 2e                       	jmp	0x140922d77 <.text+0x921d77>
140922d49: 49 89 d1                    	movq	%rdx, %r9
140922d4c: 49 d1 e9                    	shrq	%r9
140922d4f: 83 e2 01                    	andl	$0x1, %edx
140922d52: 4c 09 ca                    	orq	%r9, %rdx
140922d55: f3 48 0f 2a ca              	cvtsi2ss	%rdx, %xmm1
140922d5a: f3 0f 58 c9                 	addss	%xmm1, %xmm1
140922d5e: 41 8b 50 2c                 	movl	0x2c(%r8), %edx
140922d62: 45 84 d2                    	testb	%r10b, %r10b
140922d65: 75 ca                       	jne	0x140922d31 <.text+0x921d31>
140922d67: 0f 57 c0                    	xorps	%xmm0, %xmm0
140922d6a: 83 fa 01                    	cmpl	$0x1, %edx
140922d6d: 77 08                       	ja	0x140922d77 <.text+0x921d77>
140922d6f: f3 0f 5d 05 31 a7 18 02     	minss	0x218a731(%rip), %xmm0  # 0x142aad4a8
140922d77: 44 8b 48 1c                 	movl	0x1c(%rax), %r9d
140922d7b: 4c 8b 58 10                 	movq	0x10(%rax), %r11
140922d7f: f3 41 0f 11 40 28           	movss	%xmm0, 0x28(%r8)
140922d85: 45 89 0b                    	movl	%r9d, (%r11)
140922d88: 66 0f 7e c7                 	movd	%xmm0, %edi
140922d8c: 89 fe                       	movl	%edi, %esi
140922d8e: c1 ee 17                    	shrl	$0x17, %esi
140922d91: 40 0f b6 ce                 	movzbl	%sil, %ecx
140922d95: 31 c0                       	xorl	%eax, %eax
140922d97: 66 83 f9 60                 	cmpw	$0x60, %cx
140922d9b: 72 4b                       	jb	0x140922de8 <.text+0x921de8>
140922d9d: 89 fa                       	movl	%edi, %edx
140922d9f: 81 e2 ff ff 7f 00           	andl	$0x7fffff, %edx         # imm = 0x7FFFFF
140922da5: 81 ca 00 00 80 00           	orl	$0x800000, %edx         # imm = 0x800000
140922dab: 66 83 f9 7f                 	cmpw	$0x7f, %cx
140922daf: 0f 83 20 01 00 00           	jae	0x140922ed5 <.text+0x921ed5>
140922db5: 89 d0                       	movl	%edx, %eax
140922db7: 8d 4e 2a                    	leal	0x2a(%rsi), %ecx
140922dba: 48 d3 e0                    	shlq	%cl, %rax
140922dbd: b9 00 ca 9a 3b              	movl	$0x3b9aca00, %ecx       # imm = 0x3B9ACA00
140922dc2: 48 f7 e1                    	mulq	%rcx
140922dc5: 48 89 c1                    	movq	%rax, %rcx
140922dc8: 48 f7 d9                    	negq	%rcx
140922dcb: 0f 91 c1                    	setno	%cl
140922dce: 48 c1 e8 3f                 	shrq	$0x3f, %rax
140922dd2: 89 d3                       	movl	%edx, %ebx
140922dd4: 08 cb                       	orb	%cl, %bl
140922dd6: 20 c3                       	andb	%al, %bl
140922dd8: 0f b6 c3                    	movzbl	%bl, %eax
140922ddb: 01 d0                       	addl	%edx, %eax
140922ddd: 3d 00 ca 9a 3b              	cmpl	$0x3b9aca00, %eax       # imm = 0x3B9ACA00
140922de2: 0f 84 71 01 00 00           	je	0x140922f59 <.text+0x921f59>
140922de8: 31 d2                       	xorl	%edx, %edx
140922dea: 49 03 10                    	addq	(%r8), %rdx
140922ded: 0f 82 4e 01 00 00           	jb	0x140922f41 <.text+0x921f41>
140922df3: 41 03 40 08                 	addl	0x8(%r8), %eax
140922df7: 3d 00 ca 9a 3b              	cmpl	$0x3b9aca00, %eax       # imm = 0x3B9ACA00
140922dfc: 72 12                       	jb	0x140922e10 <.text+0x921e10>
140922dfe: 48 83 fa ff                 	cmpq	$-0x1, %rdx
140922e02: 0f 84 39 01 00 00           	je	0x140922f41 <.text+0x921f41>
140922e08: 05 00 36 65 c4              	addl	$0xc4653600, %eax       # imm = 0xC4653600
140922e0d: 48 ff c2                    	incq	%rdx
140922e10: 49 89 10                    	movq	%rdx, (%r8)
140922e13: 41 89 40 08                 	movl	%eax, 0x8(%r8)
140922e17: 45 88 50 30                 	movb	%r10b, 0x30(%r8)
140922e1b: 45 89 0b                    	movl	%r9d, (%r11)
140922e1e: 41 ff 40 2c                 	incl	0x2c(%r8)
140922e22: 0f 57 c9                    	xorps	%xmm1, %xmm1
140922e25: f3 0f 5a c8                 	cvtss2sd	%xmm0, %xmm1
140922e29: 45 89 0b                    	movl	%r9d, (%r11)
140922e2c: f2 41 0f 58 48 10           	addsd	0x10(%r8), %xmm1
140922e32: f2 41 0f 11 48 10           	movsd	%xmm1, 0x10(%r8)
140922e38: 45 89 0b                    	movl	%r9d, (%r11)
140922e3b: f3 41 0f 58 40 20           	addss	0x20(%r8), %xmm0
140922e41: f3 0f 10 0d 83 ea 0a 02     	movss	0x20aea83(%rip), %xmm1  # 0x1429d18cc
140922e49: 0f 28 d8                    	movaps	%xmm0, %xmm3
140922e4c: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
140922e50: f3 48 0f 2c c3              	cvttss2si	%xmm3, %rax
140922e55: 48 89 c1                    	movq	%rax, %rcx
140922e58: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
140922e5c: 0f 28 d3                    	movaps	%xmm3, %xmm2
140922e5f: f3 0f 5c 15 75 72 03 02     	subss	0x2037275(%rip), %xmm2  # 0x14295a0dc
140922e67: f3 48 0f 2c d2              	cvttss2si	%xmm2, %rdx
140922e6c: 48 21 ca                    	andq	%rcx, %rdx
140922e6f: 48 09 c2                    	orq	%rax, %rdx
140922e72: 31 c9                       	xorl	%ecx, %ecx
140922e74: 0f 57 d2                    	xorps	%xmm2, %xmm2
140922e77: 0f 2e da                    	ucomiss	%xmm2, %xmm3
140922e7a: 48 0f 43 ca                 	cmovaeq	%rdx, %rcx
140922e7e: 0f 2e 1d 5b 72 03 02        	ucomiss	0x203725b(%rip), %xmm3  # 0x14295a0e0
140922e85: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
140922e8c: 48 0f 46 c1                 	cmovbeq	%rcx, %rax
140922e90: 49 89 40 18                 	movq	%rax, 0x18(%r8)
140922e94: 48 85 c0                    	testq	%rax, %rax
140922e97: 78 0a                       	js	0x140922ea3 <.text+0x921ea3>
140922e99: 0f 57 db                    	xorps	%xmm3, %xmm3
140922e9c: f3 48 0f 2a d8              	cvtsi2ss	%rax, %xmm3
140922ea1: eb 18                       	jmp	0x140922ebb <.text+0x921ebb>
140922ea3: 48 89 c1                    	movq	%rax, %rcx
140922ea6: 48 d1 e9                    	shrq	%rcx
140922ea9: 83 e0 01                    	andl	$0x1, %eax
140922eac: 48 09 c8                    	orq	%rcx, %rax
140922eaf: 0f 57 db                    	xorps	%xmm3, %xmm3
140922eb2: f3 48 0f 2a d8              	cvtsi2ss	%rax, %xmm3
140922eb7: f3 0f 58 db                 	addss	%xmm3, %xmm3
140922ebb: f3 0f 5e d9                 	divss	%xmm1, %xmm3
140922ebf: f3 0f 5c c3                 	subss	%xmm3, %xmm0
140922ec3: f3 0f 5f c2                 	maxss	%xmm2, %xmm0
140922ec7: f3 41 0f 11 40 20           	movss	%xmm0, 0x20(%r8)
140922ecd: 48 83 c4 30                 	addq	$0x30, %rsp
140922ed1: 5b                          	popq	%rbx
140922ed2: 5f                          	popq	%rdi
140922ed3: 5e                          	popq	%rsi
140922ed4: c3                          	retq
140922ed5: 0f b7 c9                    	movzwl	%cx, %ecx
140922ed8: 81 f9 96 00 00 00           	cmpl	$0x96, %ecx
140922ede: 73 4c                       	jae	0x140922f2c <.text+0x921f2c>
140922ee0: 8d 46 01                    	leal	0x1(%rsi), %eax
140922ee3: b1 16                       	movb	$0x16, %cl
140922ee5: 40 28 f1                    	subb	%sil, %cl
140922ee8: d3 ea                       	shrl	%cl, %edx
140922eea: 89 c1                       	movl	%eax, %ecx
140922eec: d3 e7                       	shll	%cl, %edi
140922eee: 81 e7 ff ff 7f 00           	andl	$0x7fffff, %edi         # imm = 0x7FFFFF
140922ef4: 48 69 c7 00 ca 9a 3b        	imulq	$0x3b9aca00, %rdi, %rax # imm = 0x3B9ACA00
140922efb: 48 89 c1                    	movq	%rax, %rcx
140922efe: 48 c1 e9 17                 	shrq	$0x17, %rcx
140922f02: 89 c6                       	movl	%eax, %esi
140922f04: 81 e6 00 fe 7f 00           	andl	$0x7ffe00, %esi         # imm = 0x7FFE00
140922f0a: 81 fe 00 00 40 00           	cmpl	$0x400000, %esi         # imm = 0x400000
140922f10: 0f 95 c3                    	setne	%bl
140922f13: c1 e8 16                    	shrl	$0x16, %eax
140922f16: 83 e0 01                    	andl	$0x1, %eax
140922f19: 89 ce                       	movl	%ecx, %esi
140922f1b: 40 08 de                    	orb	%bl, %sil
140922f1e: 40 20 c6                    	andb	%al, %sil
140922f21: 40 0f b6 c6                 	movzbl	%sil, %eax
140922f25: 01 c8                       	addl	%ecx, %eax
140922f27: e9 be fe ff ff              	jmp	0x140922dea <.text+0x921dea>
140922f2c: 81 f9 bf 00 00 00           	cmpl	$0xbf, %ecx
140922f32: 73 64                       	jae	0x140922f98 <.text+0x921f98>
140922f34: 89 d2                       	movl	%edx, %edx
140922f36: 8d 4e 2a                    	leal	0x2a(%rsi), %ecx
140922f39: 48 d3 e2                    	shlq	%cl, %rdx
140922f3c: e9 a9 fe ff ff              	jmp	0x140922dea <.text+0x921dea>
140922f41: 48 8d 0d e0 a9 18 02        	leaq	0x218a9e0(%rip), %rcx   # 0x142aad928
140922f48: 4c 8d 05 f9 a9 18 02        	leaq	0x218a9f9(%rip), %r8    # 0x142aad948
140922f4f: ba 1e 00 00 00              	movl	$0x1e, %edx
140922f54: e8 d7 63 fb 01              	callq	0x1428d9330 <.text+0x28d8330>
140922f59: b8 3b 00 00 00              	movl	$0x3b, %eax
140922f5e: 48 8d 0d 7b a6 18 02        	leaq	0x218a67b(%rip), %rcx   # 0x142aad5e0
140922f65: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140922f6a: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140922f6f: 48 8d 15 3a a7 18 02        	leaq	0x218a73a(%rip), %rdx   # 0x142aad6b0
140922f76: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140922f7b: e8 10 b1 fa 01              	callq	0x1428ce090 <.text+0x28cd090>
140922f80: 48 8d 0d 9d 02 19 02        	leaq	0x219029d(%rip), %rcx   # 0x142ab3224
140922f87: 4c 8d 05 a2 b3 18 02        	leaq	0x218b3a2(%rip), %r8    # 0x142aae330
140922f8e: ba 14 00 00 00              	movl	$0x14, %edx
140922f93: e8 98 63 fb 01              	callq	0x1428d9330 <.text+0x28d8330>
140922f98: b8 48 00 00 00              	movl	$0x48, %eax
140922f9d: 48 8d 0d 77 a6 18 02        	leaq	0x218a677(%rip), %rcx   # 0x142aad61b
140922fa4: eb bf                       	jmp	0x140922f65 <.text+0x921f65>
140922fa6: cc                          	int3
140922fa7: cc                          	int3
140922fa8: cc                          	int3
140922fa9: cc                          	int3
140922faa: cc                          	int3
140922fab: cc                          	int3
140922fac: cc                          	int3
140922fad: cc                          	int3
140922fae: cc                          	int3
140922faf: cc                          	int3
140922fb0: 55                          	pushq	%rbp
140922fb1: 41 57                       	pushq	%r15
