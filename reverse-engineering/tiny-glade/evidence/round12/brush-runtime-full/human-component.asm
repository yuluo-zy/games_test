
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408b3d50: 55                          	pushq	%rbp
1408b3d51: 41 57                       	pushq	%r15
1408b3d53: 41 56                       	pushq	%r14
1408b3d55: 41 55                       	pushq	%r13
1408b3d57: 41 54                       	pushq	%r12
1408b3d59: 56                          	pushq	%rsi
1408b3d5a: 57                          	pushq	%rdi
1408b3d5b: 53                          	pushq	%rbx
1408b3d5c: 48 81 ec 08 03 00 00        	subq	$0x308, %rsp            # imm = 0x308
1408b3d63: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408b3d6b: 66 0f 7f b5 70 02 00 00     	movdqa	%xmm6, 0x270(%rbp)
1408b3d73: 48 c7 85 68 02 00 00 fe ff ff ff    	movq	$-0x2, 0x268(%rbp)
1408b3d7e: 49 89 d6                    	movq	%rdx, %r14
1408b3d81: 4c 8b ad f0 02 00 00        	movq	0x2f0(%rbp), %r13
1408b3d88: 4c 8b 22                    	movq	(%rdx), %r12
1408b3d8b: 49 8b 55 08                 	movq	0x8(%r13), %rdx
1408b3d8f: 49 8b 7d 10                 	movq	0x10(%r13), %rdi
1408b3d93: 4c 89 e1                    	movq	%r12, %rcx
1408b3d96: 48 89 95 30 02 00 00        	movq	%rdx, 0x230(%rbp)
1408b3d9d: 49 89 f8                    	movq	%rdi, %r8
1408b3da0: e8 cb cf 48 00              	callq	0x140d40d70 <.text+0xd3fd70>
1408b3da5: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
1408b3db0: 48 c7 85 48 02 00 00 08 00 00 00    	movq	$0x8, 0x248(%rbp)
1408b3dbb: 48 c7 85 50 02 00 00 00 00 00 00    	movq	$0x0, 0x250(%rbp)
1408b3dc6: 49 83 7c 24 30 00           	cmpq	$0x0, 0x30(%r12)
1408b3dcc: 48 be 9f cd e6 28 2e 7a 92 ec       	movabsq	$-0x136d85d1d7193261, %rsi # imm = 0xEC927A2E28E6CD9F
1408b3dd6: 48 bb d1 67 b5 11 5c 3f db 4e       	movabsq	$0x4edb3f5c11b567d1, %rbx # imm = 0x4EDB3F5C11B567D1
1408b3de0: 0f 84 9e 00 00 00           	je	0x1408b3e84 <.text+0x8b2e84>
1408b3de6: 49 8b 44 24 18              	movq	0x18(%r12), %rax
1408b3deb: 49 8b 4c 24 20              	movq	0x20(%r12), %rcx
1408b3df0: 48 89 ca                    	movq	%rcx, %rdx
1408b3df3: 48 21 f2                    	andq	%rsi, %rdx
1408b3df6: 45 31 c0                    	xorl	%r8d, %r8d
1408b3df9: 66 0f 6f 05 ef b3 0a 02     	movdqa	0x20ab3ef(%rip), %xmm0  # 0x14295f1f0
1408b3e01: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1408b3e05: f3 0f 6f 14 10              	movdqu	(%rax,%rdx), %xmm2
1408b3e0a: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1408b3e0e: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1408b3e12: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
1408b3e17: 45 85 c9                    	testl	%r9d, %r9d
1408b3e1a: 75 31                       	jne	0x1408b3e4d <.text+0x8b2e4d>
1408b3e1c: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1408b3e20: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1408b3e25: 45 85 c9                    	testl	%r9d, %r9d
1408b3e28: 75 5a                       	jne	0x1408b3e84 <.text+0x8b2e84>
1408b3e2a: 4c 01 c2                    	addq	%r8, %rdx
1408b3e2d: 48 83 c2 10                 	addq	$0x10, %rdx
1408b3e31: 49 83 c0 10                 	addq	$0x10, %r8
1408b3e35: 48 21 ca                    	andq	%rcx, %rdx
1408b3e38: eb cb                       	jmp	0x1408b3e05 <.text+0x8b2e05>
1408b3e3a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1408b3e40: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1408b3e44: 66 45 21 ca                 	andw	%r9w, %r10w
1408b3e48: 45 89 d1                    	movl	%r10d, %r9d
1408b3e4b: 74 cf                       	je	0x1408b3e1c <.text+0x8b2e1c>
1408b3e4d: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1408b3e52: 49 01 d2                    	addq	%rdx, %r10
1408b3e55: 49 21 ca                    	andq	%rcx, %r10
1408b3e58: 49 f7 da                    	negq	%r10
1408b3e5b: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
1408b3e5f: 4a 39 5c d0 e8              	cmpq	%rbx, -0x18(%rax,%r10,8)
1408b3e64: 75 da                       	jne	0x1408b3e40 <.text+0x8b2e40>
1408b3e66: 4a 39 74 d0 f0              	cmpq	%rsi, -0x10(%rax,%r10,8)
1408b3e6b: 75 d3                       	jne	0x1408b3e40 <.text+0x8b2e40>
1408b3e6d: 4a 8d 04 d0                 	leaq	(%rax,%r10,8), %rax
1408b3e71: 4c 8b 78 f8                 	movq	-0x8(%rax), %r15
1408b3e75: 49 3b 7d 00                 	cmpq	(%r13), %rdi
1408b3e79: 0f 84 06 04 00 00           	je	0x1408b4285 <.text+0x8b3285>
1408b3e7f: e9 9d 04 00 00              	jmp	0x1408b4321 <.text+0x8b3321>
1408b3e84: 41 0f b6 44 24 60           	movzbl	0x60(%r12), %eax
1408b3e8a: 49 8b 84 24 80 00 00 00     	movq	0x80(%r12), %rax
1408b3e92: 49 8b 94 24 88 00 00 00     	movq	0x88(%r12), %rdx
1408b3e9a: 49 89 d0                    	movq	%rdx, %r8
1408b3e9d: 49 21 f0                    	andq	%rsi, %r8
1408b3ea0: 45 31 c9                    	xorl	%r9d, %r9d
1408b3ea3: 66 0f 6f 35 45 b3 0a 02     	movdqa	0x20ab345(%rip), %xmm6  # 0x14295f1f0
1408b3eab: 66 0f 76 c0                 	pcmpeqd	%xmm0, %xmm0
1408b3eaf: f3 42 0f 6f 0c 00           	movdqu	(%rax,%r8), %xmm1
1408b3eb5: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1408b3eb9: 66 0f 74 d6                 	pcmpeqb	%xmm6, %xmm2
1408b3ebd: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1408b3ec2: 45 85 d2                    	testl	%r10d, %r10d
1408b3ec5: 75 36                       	jne	0x1408b3efd <.text+0x8b2efd>
1408b3ec7: 66 0f 74 c8                 	pcmpeqb	%xmm0, %xmm1
1408b3ecb: 66 0f d7 c9                 	pmovmskb	%xmm1, %ecx
1408b3ecf: 85 c9                       	testl	%ecx, %ecx
1408b3ed1: 0f 85 f3 00 00 00           	jne	0x1408b3fca <.text+0x8b2fca>
1408b3ed7: 4d 01 c8                    	addq	%r9, %r8
1408b3eda: 49 83 c0 10                 	addq	$0x10, %r8
1408b3ede: 49 83 c1 10                 	addq	$0x10, %r9
1408b3ee2: 49 21 d0                    	andq	%rdx, %r8
1408b3ee5: eb c8                       	jmp	0x1408b3eaf <.text+0x8b2eaf>
1408b3ee7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1408b3ef0: 41 8d 4a ff                 	leal	-0x1(%r10), %ecx
1408b3ef4: 66 44 21 d1                 	andw	%r10w, %cx
1408b3ef8: 41 89 ca                    	movl	%ecx, %r10d
1408b3efb: 74 ca                       	je	0x1408b3ec7 <.text+0x8b2ec7>
1408b3efd: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1408b3f02: 4d 01 c3                    	addq	%r8, %r11
1408b3f05: 49 21 d3                    	andq	%rdx, %r11
1408b3f08: 4c 89 d9                    	movq	%r11, %rcx
1408b3f0b: 48 f7 d9                    	negq	%rcx
1408b3f0e: 4e 8d 3c dd 00 00 00 00     	leaq	(,%r11,8), %r15
1408b3f16: 48 c1 e1 07                 	shlq	$0x7, %rcx
1408b3f1a: 4c 29 f9                    	subq	%r15, %rcx
1408b3f1d: 48 39 9c 08 78 ff ff ff     	cmpq	%rbx, -0x88(%rax,%rcx)
1408b3f25: 75 c9                       	jne	0x1408b3ef0 <.text+0x8b2ef0>
1408b3f27: 48 39 74 08 80              	cmpq	%rsi, -0x80(%rax,%rcx)
1408b3f2c: 75 c2                       	jne	0x1408b3ef0 <.text+0x8b2ef0>
1408b3f2e: 4d 89 d8                    	movq	%r11, %r8
1408b3f31: 49 c1 e0 07                 	shlq	$0x7, %r8
1408b3f35: 4f 8d 0c d8                 	leaq	(%r8,%r11,8), %r9
1408b3f39: 49 c1 f9 03                 	sarq	$0x3, %r9
1408b3f3d: 49 b8 f1 f0 f0 f0 f0 f0 f0 f0       	movabsq	$-0xf0f0f0f0f0f0f0f, %r8 # imm = 0xF0F0F0F0F0F0F0F1
1408b3f47: 4d 0f af c1                 	imulq	%r9, %r8
1408b3f4b: 4d 8d 48 f0                 	leaq	-0x10(%r8), %r9
1408b3f4f: 49 21 d1                    	andq	%rdx, %r9
1408b3f52: f3 42 0f 6f 04 08           	movdqu	(%rax,%r9), %xmm0
1408b3f58: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1408b3f5c: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
1408b3f60: 66 44 0f d7 d0              	pmovmskb	%xmm0, %r10d
1408b3f65: f3 42 0f 6f 04 00           	movdqu	(%rax,%r8), %xmm0
1408b3f6b: 66 0f 74 c1                 	pcmpeqb	%xmm1, %xmm0
1408b3f6f: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1408b3f73: 45 85 d2                    	testl	%r10d, %r10d
1408b3f76: 74 0b                       	je	0x1408b3f83 <.text+0x8b2f83>
1408b3f78: 66 45 0f bd d2              	bsrw	%r10w, %r10w
1408b3f7d: 41 83 f2 0f                 	xorl	$0xf, %r10d
1408b3f81: eb 05                       	jmp	0x1408b3f88 <.text+0x8b2f88>
1408b3f83: 66 41 ba 10 00              	movw	$0x10, %r10w
1408b3f88: 48 01 c1                    	addq	%rax, %rcx
1408b3f8b: 81 ca 00 00 01 00           	orl	$0x10000, %edx          # imm = 0x10000
1408b3f91: f3 0f bc d2                 	tzcntl	%edx, %edx
1408b3f95: 41 01 d2                    	addl	%edx, %r10d
1408b3f98: b2 80                       	movb	$-0x80, %dl
1408b3f9a: 66 41 83 fa 0f              	cmpw	$0xf, %r10w
1408b3f9f: 77 0a                       	ja	0x1408b3fab <.text+0x8b2fab>
1408b3fa1: 49 ff 84 24 90 00 00 00     	incq	0x90(%r12)
1408b3fa9: b2 ff                       	movb	$-0x1, %dl
1408b3fab: 42 88 14 00                 	movb	%dl, (%rax,%r8)
1408b3faf: 42 88 54 08 10              	movb	%dl, 0x10(%rax,%r9)
1408b3fb4: 49 ff 8c 24 98 00 00 00     	decq	0x98(%r12)
1408b3fbc: 48 8b 41 88                 	movq	-0x78(%rcx), %rax
1408b3fc0: 48 83 f8 02                 	cmpq	$0x2, %rax
1408b3fc4: 0f 85 df 02 00 00           	jne	0x1408b42a9 <.text+0x8b32a9>
1408b3fca: 49 8b 46 08                 	movq	0x8(%r14), %rax
1408b3fce: 4c 8b 38                    	movq	(%rax), %r15
1408b3fd1: 4d 8d 6f 01                 	leaq	0x1(%r15), %r13
1408b3fd5: 4c 89 28                    	movq	%r13, (%rax)
1408b3fd8: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1408b3fe2: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1408b3fe9: 48 8d 05 0d 40 38 02        	leaq	0x238400d(%rip), %rax   # 0x142c37ffd
1408b3ff0: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1408b3ff7: 48 c7 85 18 02 00 00 3c 00 00 00    	movq	$0x3c, 0x218(%rbp)
1408b4002: 66 c7 85 28 02 00 00 00 01  	movw	$0x100, 0x228(%rbp)     # imm = 0x100
1408b400b: 48 c7 85 d0 01 00 00 01 00 00 00    	movq	$0x1, 0x1d0(%rbp)
1408b4016: 48 89 9d d8 01 00 00        	movq	%rbx, 0x1d8(%rbp)
1408b401d: 48 89 b5 e0 01 00 00        	movq	%rsi, 0x1e0(%rbp)
1408b4024: 48 c7 85 f8 01 00 00 04 00 00 00    	movq	$0x4, 0x1f8(%rbp)
1408b402f: 48 c7 85 00 02 00 00 0c 00 00 00    	movq	$0xc, 0x200(%rbp)
1408b403a: 48 c7 85 20 02 00 00 00 00 00 00    	movq	$0x0, 0x220(%rbp)
1408b4045: c6 85 2a 02 00 00 01        	movb	$0x1, 0x22a(%rbp)
1408b404c: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
1408b4057: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1408b405e: 4c 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %r8
1408b4065: 4c 89 fa                    	movq	%r15, %rdx
1408b4068: e8 03 b8 48 00              	callq	0x140d3f870 <.text+0xd3e870>
1408b406d: 4d 39 6c 24 10              	cmpq	%r13, 0x10(%r12)
1408b4072: 73 19                       	jae	0x1408b408d <.text+0x8b308d>
1408b4074: c6 85 67 02 00 00 01        	movb	$0x1, 0x267(%rbp)
1408b407b: 4c 8d 05 6e b0 1e 02        	leaq	0x21eb06e(%rip), %r8    # 0x142a9f0f0
1408b4082: 4c 89 e1                    	movq	%r12, %rcx
1408b4085: 4c 89 ea                    	movq	%r13, %rdx
1408b4088: e8 b3 02 a4 00              	callq	0x1412f4340 <.text+0x12f3340>
1408b408d: 4d 8b 64 24 08              	movq	0x8(%r12), %r12
1408b4092: 4d 69 ef d0 00 00 00        	imulq	$0xd0, %r15, %r13
1408b4099: 4b 8d 04 2c                 	leaq	(%r12,%r13), %rax
1408b409d: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1408b40a4: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1408b40a8: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
1408b40af: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
1408b40b5: e8 46 fb 06 02              	callq	0x142923c00 <.text+0x2922c00>
1408b40ba: 43 83 3c 2c 02              	cmpl	$0x2, (%r12,%r13)
1408b40bf: 74 0c                       	je	0x1408b40cd <.text+0x8b30cd>
1408b40c1: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
1408b40c8: e8 83 ca d3 ff              	callq	0x1405f0b50 <.text+0x5efb50>
1408b40cd: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1408b40d1: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
1408b40d7: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
1408b40de: e8 1d fb 06 02              	callq	0x142923c00 <.text+0x2922c00>
1408b40e3: 4d 8b 26                    	movq	(%r14), %r12
1408b40e6: 49 83 7c 24 28 00           	cmpq	$0x0, 0x28(%r12)
1408b40ec: 0f 84 ca 02 00 00           	je	0x1408b43bc <.text+0x8b33bc>
1408b40f2: 4c 89 ad 58 02 00 00        	movq	%r13, 0x258(%rbp)
1408b40f9: 49 8b 44 24 18              	movq	0x18(%r12), %rax
1408b40fe: 4c 89 a5 38 02 00 00        	movq	%r12, 0x238(%rbp)
1408b4105: 49 8b 4c 24 20              	movq	0x20(%r12), %rcx
1408b410a: 45 31 c0                    	xorl	%r8d, %r8d
1408b410d: 66 0f 76 c0                 	pcmpeqd	%xmm0, %xmm0
1408b4111: 49 89 f1                    	movq	%rsi, %r9
1408b4114: 45 31 d2                    	xorl	%r10d, %r10d
1408b4117: 49 21 c9                    	andq	%rcx, %r9
1408b411a: f3 42 0f 6f 0c 08           	movdqu	(%rax,%r9), %xmm1
1408b4120: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1408b4124: 66 0f 74 d6                 	pcmpeqb	%xmm6, %xmm2
1408b4128: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1408b412d: 45 85 db                    	testl	%r11d, %r11d
1408b4130: 75 5b                       	jne	0x1408b418d <.text+0x8b318d>
1408b4132: 49 83 fa 01                 	cmpq	$0x1, %r10
1408b4136: 41 ba 01 00 00 00           	movl	$0x1, %r10d
1408b413c: 74 18                       	je	0x1408b4156 <.text+0x8b3156>
1408b413e: 66 0f d7 d1                 	pmovmskb	%xmm1, %edx
1408b4142: 45 31 d2                    	xorl	%r10d, %r10d
1408b4145: 85 d2                       	testl	%edx, %edx
1408b4147: 41 0f 95 c2                 	setne	%r10b
1408b414b: f3 44 0f bc e2              	tzcntl	%edx, %r12d
1408b4150: 4d 01 cc                    	addq	%r9, %r12
1408b4153: 49 21 cc                    	andq	%rcx, %r12
1408b4156: 66 0f 74 c8                 	pcmpeqb	%xmm0, %xmm1
1408b415a: 66 0f d7 d1                 	pmovmskb	%xmm1, %edx
1408b415e: 85 d2                       	testl	%edx, %edx
1408b4160: 0f 85 f1 01 00 00           	jne	0x1408b4357 <.text+0x8b3357>
1408b4166: 4d 01 c1                    	addq	%r8, %r9
1408b4169: 49 83 c1 10                 	addq	$0x10, %r9
1408b416d: 49 83 c0 10                 	addq	$0x10, %r8
1408b4171: eb a4                       	jmp	0x1408b4117 <.text+0x8b3117>
1408b4173: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1408b4180: 41 8d 53 ff                 	leal	-0x1(%r11), %edx
1408b4184: 66 44 21 da                 	andw	%r11w, %dx
1408b4188: 41 89 d3                    	movl	%edx, %r11d
1408b418b: 74 a5                       	je	0x1408b4132 <.text+0x8b3132>
1408b418d: f3 41 0f bc d3              	tzcntl	%r11d, %edx
1408b4192: 4c 01 ca                    	addq	%r9, %rdx
1408b4195: 48 21 ca                    	andq	%rcx, %rdx
1408b4198: 48 f7 da                    	negq	%rdx
1408b419b: 4c 8d 2c 52                 	leaq	(%rdx,%rdx,2), %r13
1408b419f: 4a 39 5c e8 e8              	cmpq	%rbx, -0x18(%rax,%r13,8)
1408b41a4: 75 da                       	jne	0x1408b4180 <.text+0x8b3180>
1408b41a6: 4a 39 74 e8 f0              	cmpq	%rsi, -0x10(%rax,%r13,8)
1408b41ab: 75 d3                       	jne	0x1408b4180 <.text+0x8b3180>
1408b41ad: 48 8d 0c 52                 	leaq	(%rdx,%rdx,2), %rcx
1408b41b1: 4c 89 7c c8 f8              	movq	%r15, -0x8(%rax,%rcx,8)
1408b41b6: 0f 10 05 23 ae 1e 02        	movups	0x21eae23(%rip), %xmm0  # 0x142a9efe0
1408b41bd: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
1408b41c4: f3 0f 6f 05 04 ae 1e 02     	movdqu	0x21eae04(%rip), %xmm0  # 0x142a9efd0
1408b41cc: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
1408b41d4: c6 85 66 02 00 00 01        	movb	$0x1, 0x266(%rbp)
1408b41db: 48 8d 85 40 02 00 00        	leaq	0x240(%rbp), %rax
1408b41e2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408b41e7: 4c 89 f2                    	movq	%r14, %rdx
1408b41ea: e8 61 fb ff ff              	callq	0x1408b3d50 <.text+0x8b2d50>
1408b41ef: 49 8b 06                    	movq	(%r14), %rax
1408b41f2: 48 8b 40 08                 	movq	0x8(%rax), %rax
1408b41f6: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
1408b41fd: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
1408b4204: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
1408b4208: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1408b420c: 48 8b 95 58 02 00 00        	movq	0x258(%rbp), %rdx
1408b4213: 48 8d 4c 10 60              	leaq	0x60(%rax,%rdx), %rcx
1408b4218: 48 8d 94 10 80 00 00 00     	leaq	0x80(%rax,%rdx), %rdx
1408b4220: 41 b8 20 00 00 00           	movl	$0x20, %r8d
1408b4226: 41 b9 10 00 00 00           	movl	$0x10, %r9d
1408b422c: 48 89 8d 58 02 00 00        	movq	%rcx, 0x258(%rbp)
1408b4233: e8 08 d1 79 ff              	callq	0x140051340 <.text+0x50340>
1408b4238: 4c 8b ad f0 02 00 00        	movq	0x2f0(%rbp), %r13
1408b423f: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
1408b4244: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
1408b4249: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
1408b4250: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
1408b4255: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1408b4259: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
1408b4260: 48 85 d2                    	testq	%rdx, %rdx
1408b4263: 74 16                       	je	0x1408b427b <.text+0x8b327b>
1408b4265: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1408b426c: 48 c1 e2 03                 	shlq	$0x3, %rdx
1408b4270: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1408b4276: e8 a5 f9 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408b427b: 49 3b 7d 00                 	cmpq	(%r13), %rdi
1408b427f: 0f 85 9c 00 00 00           	jne	0x1408b4321 <.text+0x8b3321>
1408b4285: 48 8d 15 cc b0 1e 02        	leaq	0x21eb0cc(%rip), %rdx   # 0x142a9f358
1408b428c: 4c 89 e9                    	movq	%r13, %rcx
1408b428f: e8 5c db 2c 00              	callq	0x140b81df0 <.text+0xb80df0>
1408b4294: 49 8b 45 08                 	movq	0x8(%r13), %rax
1408b4298: 4c 89 3c f8                 	movq	%r15, (%rax,%rdi,8)
1408b429c: 48 8d 47 01                 	leaq	0x1(%rdi), %rax
1408b42a0: 49 89 45 10                 	movq	%rax, 0x10(%r13)
1408b42a4: e9 8f 00 00 00              	jmp	0x1408b4338 <.text+0x8b3338>
1408b42a9: 0f 10 41 f0                 	movups	-0x10(%rcx), %xmm0
1408b42ad: 0f 11 85 c0 01 00 00        	movups	%xmm0, 0x1c0(%rbp)
1408b42b4: 0f 10 41 e0                 	movups	-0x20(%rcx), %xmm0
1408b42b8: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
1408b42bf: 0f 10 41 d0                 	movups	-0x30(%rcx), %xmm0
1408b42c3: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
1408b42ca: f3 0f 6f 41 90              	movdqu	-0x70(%rcx), %xmm0
1408b42cf: f3 0f 6f 49 a0              	movdqu	-0x60(%rcx), %xmm1
1408b42d4: f3 0f 6f 51 b0              	movdqu	-0x50(%rcx), %xmm2
1408b42d9: f3 0f 6f 59 c0              	movdqu	-0x40(%rcx), %xmm3
1408b42de: f3 0f 7f 9d 90 01 00 00     	movdqu	%xmm3, 0x190(%rbp)
1408b42e6: f3 0f 7f 95 80 01 00 00     	movdqu	%xmm2, 0x180(%rbp)
1408b42ee: f3 0f 7f 8d 70 01 00 00     	movdqu	%xmm1, 0x170(%rbp)
1408b42f6: f3 0f 7f 85 60 01 00 00     	movdqu	%xmm0, 0x160(%rbp)
1408b42fe: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1408b4305: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
1408b430c: 4c 89 f2                    	movq	%r14, %rdx
1408b430f: e8 2c b7 48 00              	callq	0x140d3fa40 <.text+0xd3ea40>
1408b4314: 49 89 c7                    	movq	%rax, %r15
1408b4317: 49 3b 7d 00                 	cmpq	(%r13), %rdi
1408b431b: 0f 84 64 ff ff ff           	je	0x1408b4285 <.text+0x8b3285>
1408b4321: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1408b4328: 4c 89 3c f8                 	movq	%r15, (%rax,%rdi,8)
1408b432c: 48 89 f8                    	movq	%rdi, %rax
1408b432f: 48 ff c0                    	incq	%rax
1408b4332: 49 89 45 10                 	movq	%rax, 0x10(%r13)
1408b4336: 74 04                       	je	0x1408b433c <.text+0x8b333c>
1408b4338: 49 89 7d 10                 	movq	%rdi, 0x10(%r13)
1408b433c: 0f 28 b5 70 02 00 00        	movaps	0x270(%rbp), %xmm6
1408b4343: 48 81 c4 08 03 00 00        	addq	$0x308, %rsp            # imm = 0x308
1408b434a: 5b                          	popq	%rbx
1408b434b: 5f                          	popq	%rdi
1408b434c: 5e                          	popq	%rsi
1408b434d: 41 5c                       	popq	%r12
1408b434f: 41 5d                       	popq	%r13
1408b4351: 41 5e                       	popq	%r14
1408b4353: 41 5f                       	popq	%r15
1408b4355: 5d                          	popq	%rbp
1408b4356: c3                          	retq
1408b4357: 42 0f b6 14 20              	movzbl	(%rax,%r12), %edx
1408b435c: 84 d2                       	testb	%dl, %dl
1408b435e: 78 12                       	js	0x1408b4372 <.text+0x8b3372>
1408b4360: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
1408b4364: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1408b4368: f3 44 0f bc e2              	tzcntl	%edx, %r12d
1408b436d: 42 0f b6 14 20              	movzbl	(%rax,%r12), %edx
1408b4372: 80 e2 01                    	andb	$0x1, %dl
1408b4375: 0f b6 d2                    	movzbl	%dl, %edx
1408b4378: 4c 8b 85 38 02 00 00        	movq	0x238(%rbp), %r8
1408b437f: 49 29 50 28                 	subq	%rdx, 0x28(%r8)
1408b4383: 49 8d 54 24 f0              	leaq	-0x10(%r12), %rdx
1408b4388: 48 21 ca                    	andq	%rcx, %rdx
1408b438b: 42 c6 04 20 76              	movb	$0x76, (%rax,%r12)
1408b4390: c6 44 10 10 76              	movb	$0x76, 0x10(%rax,%rdx)
1408b4395: 49 ff 40 30                 	incq	0x30(%r8)
1408b4399: 49 f7 dc                    	negq	%r12
1408b439c: 4b 8d 0c 64                 	leaq	(%r12,%r12,2), %rcx
1408b43a0: 48 89 5c c8 e8              	movq	%rbx, -0x18(%rax,%rcx,8)
1408b43a5: 48 ba 9f cd e6 28 2e 7a 92 ec       	movabsq	$-0x136d85d1d7193261, %rdx # imm = 0xEC927A2E28E6CD9F
1408b43af: 48 89 54 c8 f0              	movq	%rdx, -0x10(%rax,%rcx,8)
1408b43b4: 4c 89 e2                    	movq	%r12, %rdx
1408b43b7: e9 f1 fd ff ff              	jmp	0x1408b41ad <.text+0x8b31ad>
1408b43bc: 49 8d 4c 24 18              	leaq	0x18(%r12), %rcx
1408b43c1: 4d 8d 44 24 38              	leaq	0x38(%r12), %r8
1408b43c6: ba 01 00 00 00              	movl	$0x1, %edx
1408b43cb: 41 b1 01                    	movb	$0x1, %r9b
1408b43ce: e8 ed db 01 02              	callq	0x1428d1fc0 <.text+0x28d0fc0>
1408b43d3: e9 1a fd ff ff              	jmp	0x1408b40f2 <.text+0x8b30f2>
1408b43d8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1408b43e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408b43e5: 55                          	pushq	%rbp
1408b43e6: 41 57                       	pushq	%r15
1408b43e8: 41 56                       	pushq	%r14
1408b43ea: 41 55                       	pushq	%r13
1408b43ec: 41 54                       	pushq	%r12
1408b43ee: 56                          	pushq	%rsi
1408b43ef: 57                          	pushq	%rdi
1408b43f0: 53                          	pushq	%rbx
1408b43f1: 48 83 ec 38                 	subq	$0x38, %rsp
1408b43f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408b43fc: 66 0f 7f 74 24 20           	movdqa	%xmm6, 0x20(%rsp)
1408b4402: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1408b4406: 41 b8 d0 00 00 00           	movl	$0xd0, %r8d
1408b440c: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
1408b4413: e8 e8 f7 06 02              	callq	0x142923c00 <.text+0x2922c00>
1408b4418: c6 85 67 02 00 00 00        	movb	$0x0, 0x267(%rbp)
1408b441f: 66 0f 6f 74 24 20           	movdqa	0x20(%rsp), %xmm6
1408b4425: 48 83 c4 38                 	addq	$0x38, %rsp
1408b4429: 5b                          	popq	%rbx
1408b442a: 5f                          	popq	%rdi
1408b442b: 5e                          	popq	%rsi
1408b442c: 41 5c                       	popq	%r12
1408b442e: 41 5d                       	popq	%r13
1408b4430: 41 5e                       	popq	%r14
1408b4432: 41 5f                       	popq	%r15
1408b4434: 5d                          	popq	%rbp
1408b4435: c3                          	retq
1408b4436: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1408b4440: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408b4445: 55                          	pushq	%rbp
1408b4446: 41 57                       	pushq	%r15
1408b4448: 41 56                       	pushq	%r14
1408b444a: 41 55                       	pushq	%r13
1408b444c: 41 54                       	pushq	%r12
1408b444e: 56                          	pushq	%rsi
1408b444f: 57                          	pushq	%rdi
1408b4450: 53                          	pushq	%rbx
1408b4451: 48 83 ec 38                 	subq	$0x38, %rsp
1408b4455: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408b445c: 66 0f 7f 74 24 20           	movdqa	%xmm6, 0x20(%rsp)
1408b4462: 80 bd 67 02 00 00 00        	cmpb	$0x0, 0x267(%rbp)
1408b4469: 74 0c                       	je	0x1408b4477 <.text+0x8b3477>
1408b446b: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1408b4472: e8 d9 c6 d3 ff              	callq	0x1405f0b50 <.text+0x5efb50>
1408b4477: 66 0f 6f 74 24 20           	movdqa	0x20(%rsp), %xmm6
1408b447d: 48 83 c4 38                 	addq	$0x38, %rsp
1408b4481: 5b                          	popq	%rbx
1408b4482: 5f                          	popq	%rdi
1408b4483: 5e                          	popq	%rsi
1408b4484: 41 5c                       	popq	%r12
1408b4486: 41 5d                       	popq	%r13
1408b4488: 41 5e                       	popq	%r14
1408b448a: 41 5f                       	popq	%r15
1408b448c: 5d                          	popq	%rbp
1408b448d: c3                          	retq
1408b448e: 66 90                       	nop
1408b4490: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408b4495: 55                          	pushq	%rbp
1408b4496: 41 57                       	pushq	%r15
1408b4498: 41 56                       	pushq	%r14
1408b449a: 41 55                       	pushq	%r13
1408b449c: 41 54                       	pushq	%r12
1408b449e: 56                          	pushq	%rsi
1408b449f: 57                          	pushq	%rdi
1408b44a0: 53                          	pushq	%rbx
1408b44a1: 48 83 ec 38                 	subq	$0x38, %rsp
1408b44a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408b44ac: 66 0f 7f 74 24 20           	movdqa	%xmm6, 0x20(%rsp)
1408b44b2: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
1408b44b7: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
1408b44bc: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
1408b44c3: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
1408b44c8: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
1408b44cc: c6 85 66 02 00 00 00        	movb	$0x0, 0x266(%rbp)
1408b44d3: 66 0f 6f 74 24 20           	movdqa	0x20(%rsp), %xmm6
1408b44d9: 48 83 c4 38                 	addq	$0x38, %rsp
1408b44dd: 5b                          	popq	%rbx
1408b44de: 5f                          	popq	%rdi
1408b44df: 5e                          	popq	%rsi
1408b44e0: 41 5c                       	popq	%r12
1408b44e2: 41 5d                       	popq	%r13
1408b44e4: 41 5e                       	popq	%r14
1408b44e6: 41 5f                       	popq	%r15
1408b44e8: 5d                          	popq	%rbp
1408b44e9: c3                          	retq
1408b44ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1408b44f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408b44f5: 55                          	pushq	%rbp
1408b44f6: 41 57                       	pushq	%r15
1408b44f8: 41 56                       	pushq	%r14
1408b44fa: 41 55                       	pushq	%r13
1408b44fc: 41 54                       	pushq	%r12
1408b44fe: 56                          	pushq	%rsi
1408b44ff: 57                          	pushq	%rdi
