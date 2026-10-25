
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1421b67a0: 55                          	pushq	%rbp
1421b67a1: 41 57                       	pushq	%r15
1421b67a3: 41 56                       	pushq	%r14
1421b67a5: 41 55                       	pushq	%r13
1421b67a7: 41 54                       	pushq	%r12
1421b67a9: 56                          	pushq	%rsi
1421b67aa: 57                          	pushq	%rdi
1421b67ab: 53                          	pushq	%rbx
1421b67ac: 48 81 ec 28 03 00 00        	subq	$0x328, %rsp            # imm = 0x328
1421b67b3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1421b67bb: 44 0f 29 bd 90 02 00 00     	movaps	%xmm15, 0x290(%rbp)
1421b67c3: 44 0f 29 b5 80 02 00 00     	movaps	%xmm14, 0x280(%rbp)
1421b67cb: 44 0f 29 ad 70 02 00 00     	movaps	%xmm13, 0x270(%rbp)
1421b67d3: 44 0f 29 a5 60 02 00 00     	movaps	%xmm12, 0x260(%rbp)
1421b67db: 44 0f 29 9d 50 02 00 00     	movaps	%xmm11, 0x250(%rbp)
1421b67e3: 44 0f 29 95 40 02 00 00     	movaps	%xmm10, 0x240(%rbp)
1421b67eb: 44 0f 29 8d 30 02 00 00     	movaps	%xmm9, 0x230(%rbp)
1421b67f3: 44 0f 29 85 20 02 00 00     	movaps	%xmm8, 0x220(%rbp)
1421b67fb: 0f 29 bd 10 02 00 00        	movaps	%xmm7, 0x210(%rbp)
1421b6802: 0f 29 b5 00 02 00 00        	movaps	%xmm6, 0x200(%rbp)
1421b6809: 48 c7 85 f8 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1f8(%rbp)
1421b6814: 4c 89 ce                    	movq	%r9, %rsi
1421b6817: 49 89 d6                    	movq	%rdx, %r14
1421b681a: 48 89 cf                    	movq	%rcx, %rdi
1421b681d: 49 8b 08                    	movq	(%r8), %rcx
1421b6820: 48 8d 15 91 23 e4 00        	leaq	0xe42391(%rip), %rdx    # 0x142ff8bb8
1421b6827: e8 d4 f4 ff ff              	callq	0x1421b5d00 <.text+0x21b4d00>
1421b682c: 48 85 c0                    	testq	%rax, %rax
1421b682f: 0f 84 54 11 00 00           	je	0x1421b7989 <.text+0x21b6989>
1421b6835: 48 89 c3                    	movq	%rax, %rbx
1421b6838: 48 8d 05 b1 24 e4 00        	leaq	0xe424b1(%rip), %rax    # 0x142ff8cf0
1421b683f: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
1421b6843: 48 c7 45 b8 3d 00 00 00     	movq	$0x3d, -0x48(%rbp)
1421b684b: 48 83 7b 18 3d              	cmpq	$0x3d, 0x18(%rbx)
1421b6850: 0f 85 de 10 00 00           	jne	0x1421b7934 <.text+0x21b6934>
1421b6856: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1421b685a: 48 8d 0d 8f 24 e4 00        	leaq	0xe4248f(%rip), %rcx    # 0x142ff8cf0
1421b6861: 41 b8 3d 00 00 00           	movl	$0x3d, %r8d
1421b6867: e8 84 d3 76 00              	callq	0x142923bf0 <.text+0x2922bf0>
1421b686c: 85 c0                       	testl	%eax, %eax
1421b686e: 0f 85 c0 10 00 00           	jne	0x1421b7934 <.text+0x21b6934>
1421b6874: 48 8b 0b                    	movq	(%rbx), %rcx
1421b6877: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1421b687b: ff 50 18                    	callq	*0x18(%rax)
1421b687e: 48 85 c0                    	testq	%rax, %rax
1421b6881: 0f 84 02 11 00 00           	je	0x1421b7989 <.text+0x21b6989>
1421b6887: 48 89 c3                    	movq	%rax, %rbx
1421b688a: 49 8b 06                    	movq	(%r14), %rax
1421b688d: 48 83 38 00                 	cmpq	$0x0, (%rax)
1421b6891: 0f 84 fe 10 00 00           	je	0x1421b7995 <.text+0x21b6995>
1421b6897: 4c 8b 78 08                 	movq	0x8(%rax), %r15
1421b689b: 4d 85 ff                    	testq	%r15, %r15
1421b689e: 0f 84 fd 10 00 00           	je	0x1421b79a1 <.text+0x21b69a1>
1421b68a4: 49 8d 4f 18                 	leaq	0x18(%r15), %rcx
1421b68a8: b2 01                       	movb	$0x1, %dl
1421b68aa: 31 c0                       	xorl	%eax, %eax
1421b68ac: f0                          	lock
1421b68ad: 41 0f b0 57 18              	cmpxchgb	%dl, 0x18(%r15)
1421b68b2: 0f 85 ac 10 00 00           	jne	0x1421b7964 <.text+0x21b6964>
1421b68b8: e8 b3 b3 ac fe              	callq	0x140c81c70 <.text+0xc80c70>
1421b68bd: 49 8b 47 10                 	movq	0x10(%r15), %rax
1421b68c1: 48 85 c0                    	testq	%rax, %rax
1421b68c4: 0f 94 c1                    	sete	%cl
1421b68c7: 48 c1 e0 02                 	shlq	$0x2, %rax
1421b68cb: 49 03 47 08                 	addq	0x8(%r15), %rax
1421b68cf: 48 83 c0 fc                 	addq	$-0x4, %rax
1421b68d3: 0f 94 c2                    	sete	%dl
1421b68d6: 08 ca                       	orb	%cl, %dl
1421b68d8: 74 12                       	je	0x1421b68ec <.text+0x21b58ec>
1421b68da: 48 8b 43 28                 	movq	0x28(%rbx), %rax
1421b68de: 48 85 c0                    	testq	%rax, %rax
1421b68e1: 0f 84 c6 10 00 00           	je	0x1421b79ad <.text+0x21b69ad>
1421b68e7: 8d 48 ff                    	leal	-0x1(%rax), %ecx
1421b68ea: eb 06                       	jmp	0x1421b68f2 <.text+0x21b58f2>
1421b68ec: 8b 08                       	movl	(%rax), %ecx
1421b68ee: 48 8b 43 28                 	movq	0x28(%rbx), %rax
1421b68f2: 48 8b 53 20                 	movq	0x20(%rbx), %rdx
1421b68f6: 0f 10 43 30                 	movups	0x30(%rbx), %xmm0
1421b68fa: 0f 10 4b 40                 	movups	0x40(%rbx), %xmm1
1421b68fe: 0f 10 53 50                 	movups	0x50(%rbx), %xmm2
1421b6902: 89 4d 00                    	movl	%ecx, (%rbp)
1421b6905: 48 89 55 b0                 	movq	%rdx, -0x50(%rbp)
1421b6909: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
1421b690d: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
1421b6911: 4c 89 7d d0                 	movq	%r15, -0x30(%rbp)
1421b6915: 0f 11 4d d8                 	movups	%xmm1, -0x28(%rbp)
1421b6919: 48 89 5d e8                 	movq	%rbx, -0x18(%rbp)
1421b691d: 0f 11 55 f0                 	movups	%xmm2, -0x10(%rbp)
1421b6921: 4c 8b 17                    	movq	(%rdi), %r10
1421b6924: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1421b6928: 49 8b 0a                    	movq	(%r10), %rcx
1421b692b: 31 db                       	xorl	%ebx, %ebx
1421b692d: 48 89 ca                    	movq	%rcx, %rdx
1421b6930: 48 2b 50 18                 	subq	0x18(%rax), %rdx
1421b6934: 48 0f 42 d3                 	cmovbq	%rbx, %rdx
1421b6938: 48 2b 48 38                 	subq	0x38(%rax), %rcx
1421b693c: 48 0f 42 cb                 	cmovbq	%rbx, %rcx
1421b6940: 4c 8d 1c 92                 	leaq	(%rdx,%rdx,4), %r11
1421b6944: 49 c1 e3 03                 	shlq	$0x3, %r11
1421b6948: 4c 03 58 08                 	addq	0x8(%rax), %r11
1421b694c: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1421b6950: 4c 8b 48 30                 	movq	0x30(%rax), %r9
1421b6954: 49 29 d0                    	subq	%rdx, %r8
1421b6957: 4c 0f 42 c3                 	cmovbq	%rbx, %r8
1421b695b: ba 08 00 00 00              	movl	$0x8, %edx
1421b6960: 4c 0f 42 da                 	cmovbq	%rdx, %r11
1421b6964: 4c 8d 24 89                 	leaq	(%rcx,%rcx,4), %r12
1421b6968: 49 c1 e4 03                 	shlq	$0x3, %r12
1421b696c: 4c 03 60 28                 	addq	0x28(%rax), %r12
1421b6970: 49 29 c9                    	subq	%rcx, %r9
1421b6973: 4c 0f 42 cb                 	cmovbq	%rbx, %r9
1421b6977: 4c 0f 42 e2                 	cmovbq	%rdx, %r12
1421b697b: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
1421b6982: 48 8b 40 40                 	movq	0x40(%rax), %rax
1421b6986: 4c 29 c8                    	subq	%r9, %rax
1421b6989: 4c 29 c0                    	subq	%r8, %rax
1421b698c: 4c 89 55 30                 	movq	%r10, 0x30(%rbp)
1421b6990: 49 89 02                    	movq	%rax, (%r10)
1421b6993: 4b 8d 04 80                 	leaq	(%r8,%r8,4), %rax
1421b6997: 49 8d 04 c3                 	leaq	(%r11,%rax,8), %rax
1421b699b: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1421b699f: 4b 8d 04 89                 	leaq	(%r9,%r9,4), %rax
1421b69a3: 49 8d 04 c4                 	leaq	(%r12,%rax,8), %rax
1421b69a7: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1421b69ab: 48 8b 06                    	movq	(%rsi), %rax
1421b69ae: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1421b69b2: 4c 8d b5 30 01 00 00        	leaq	0x130(%rbp), %r14
1421b69b9: 8b 41 1c                    	movl	0x1c(%rcx), %eax
1421b69bc: 89 85 bc 00 00 00           	movl	%eax, 0xbc(%rbp)
1421b69c2: 48 8b 01                    	movq	(%rcx), %rax
1421b69c5: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1421b69c9: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1421b69cd: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1421b69d1: 48 8d bd 9c 00 00 00        	leaq	0x9c(%rbp), %rdi
1421b69d8: f3 44 0f 10 35 23 ef 76 00  	movss	0x76ef23(%rip), %xmm14  # 0x142925904
1421b69e1: 4c 8d bd 60 01 00 00        	leaq	0x160(%rbp), %r15
1421b69e8: 4c 8d ad c0 00 00 00        	leaq	0xc0(%rbp), %r13
1421b69ef: 4c 89 e0                    	movq	%r12, %rax
1421b69f2: 4c 89 d9                    	movq	%r11, %rcx
1421b69f5: 4c 8b 45 40                 	movq	0x40(%rbp), %r8
1421b69f9: 4c 8b 4d 48                 	movq	0x48(%rbp), %r9
1421b69fd: eb 1b                       	jmp	0x1421b6a1a <.text+0x21b5a1a>
1421b69ff: 90                          	nop
1421b6a00: 48 8d 51 28                 	leaq	0x28(%rcx), %rdx
1421b6a04: 49 89 c2                    	movq	%rax, %r10
1421b6a07: 49 89 cb                    	movq	%rcx, %r11
1421b6a0a: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1421b6a0e: 48 ff 01                    	incq	(%rcx)
1421b6a11: 41 f6 03 01                 	testb	$0x1, (%r11)
1421b6a15: 48 89 d1                    	movq	%rdx, %rcx
1421b6a18: 74 36                       	je	0x1421b6a50 <.text+0x21b5a50>
1421b6a1a: 48 85 c9                    	testq	%rcx, %rcx
1421b6a1d: 74 05                       	je	0x1421b6a24 <.text+0x21b5a24>
1421b6a1f: 4c 39 c1                    	cmpq	%r8, %rcx
1421b6a22: 75 dc                       	jne	0x1421b6a00 <.text+0x21b5a00>
1421b6a24: 48 85 c0                    	testq	%rax, %rax
1421b6a27: 0f 84 28 0c 00 00           	je	0x1421b7655 <.text+0x21b6655>
1421b6a2d: 4c 39 c8                    	cmpq	%r9, %rax
1421b6a30: 0f 84 1f 0c 00 00           	je	0x1421b7655 <.text+0x21b6655>
1421b6a36: 4c 8d 50 28                 	leaq	0x28(%rax), %r10
1421b6a3a: 31 d2                       	xorl	%edx, %edx
1421b6a3c: 49 89 c3                    	movq	%rax, %r11
1421b6a3f: 4c 89 d0                    	movq	%r10, %rax
1421b6a42: eb c6                       	jmp	0x1421b6a0a <.text+0x21b5a0a>
1421b6a44: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1421b6a50: 4c 89 55 78                 	movq	%r10, 0x78(%rbp)
1421b6a54: 48 89 55 70                 	movq	%rdx, 0x70(%rbp)
1421b6a58: 4c 89 5d 68                 	movq	%r11, 0x68(%rbp)
1421b6a5c: 49 8d 4b 04                 	leaq	0x4(%r11), %rcx
1421b6a60: 48 8b 45 50                 	movq	0x50(%rbp), %rax
1421b6a64: f3 0f 10 40 08              	movss	0x8(%rax), %xmm0
1421b6a69: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
1421b6a70: f3 0f 10 40 0c              	movss	0xc(%rax), %xmm0
1421b6a75: f3 0f 11 85 f4 01 00 00     	movss	%xmm0, 0x1f4(%rbp)
1421b6a7d: 48 89 8d 50 01 00 00        	movq	%rcx, 0x150(%rbp)
1421b6a84: e8 e7 0e ae fe              	callq	0x140c97970 <.text+0xc96970>
1421b6a89: f3 0f 5e 05 3f ee 76 00     	divss	0x76ee3f(%rip), %xmm0   # 0x1429258d0
1421b6a91: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
1421b6a96: 48 89 c1                    	movq	%rax, %rcx
1421b6a99: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
1421b6a9d: 0f 28 c8                    	movaps	%xmm0, %xmm1
1421b6aa0: f3 0f 5c 0d 34 36 7a 00     	subss	0x7a3634(%rip), %xmm1   # 0x14295a0dc
1421b6aa8: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
1421b6aad: 48 21 ca                    	andq	%rcx, %rdx
1421b6ab0: 48 09 c2                    	orq	%rax, %rdx
1421b6ab3: 0f 2e 05 d6 9b 7a 00        	ucomiss	0x7a9bd6(%rip), %xmm0   # 0x142960690
1421b6aba: 41 b8 00 00 00 00           	movl	$0x0, %r8d
1421b6ac0: 4c 0f 43 c2                 	cmovaeq	%rdx, %r8
1421b6ac4: 0f 2e 05 15 36 7a 00        	ucomiss	0x7a3615(%rip), %xmm0   # 0x14295a0e0
1421b6acb: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
1421b6ad2: 4c 0f 47 c0                 	cmovaq	%rax, %r8
1421b6ad6: 49 83 f8 07                 	cmpq	$0x7, %r8
1421b6ada: b8 06 00 00 00              	movl	$0x6, %eax
1421b6adf: 4c 0f 42 c0                 	cmovbq	%rax, %r8
1421b6ae3: 4c 89 f9                    	movq	%r15, %rcx
1421b6ae6: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
1421b6aed: 4c 89 85 58 01 00 00        	movq	%r8, 0x158(%rbp)
1421b6af4: e8 a7 10 ae fe              	callq	0x140c97ba0 <.text+0xc96ba0>
1421b6af9: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
1421b6b00: 48 85 c0                    	testq	%rax, %rax
1421b6b03: 78 0b                       	js	0x1421b6b10 <.text+0x21b5b10>
1421b6b05: f3 48 0f 2a f8              	cvtsi2ss	%rax, %xmm7
1421b6b0a: eb 19                       	jmp	0x1421b6b25 <.text+0x21b5b25>
1421b6b0c: 0f 1f 40 00                 	nopl	(%rax)
1421b6b10: 48 89 c1                    	movq	%rax, %rcx
1421b6b13: 48 d1 e9                    	shrq	%rcx
1421b6b16: 83 e0 01                    	andl	$0x1, %eax
1421b6b19: 48 09 c8                    	orq	%rcx, %rax
1421b6b1c: f3 48 0f 2a f8              	cvtsi2ss	%rax, %xmm7
1421b6b21: f3 0f 58 ff                 	addss	%xmm7, %xmm7
1421b6b25: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1421b6b2c: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1421b6b33: 4c 8b a5 70 01 00 00        	movq	0x170(%rbp), %r12
1421b6b3a: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1421b6b41: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
1421b6b48: f3 0f 58 3d 94 35 7a 00     	addss	0x7a3594(%rip), %xmm7   # 0x14295a0e4
1421b6b50: 31 f6                       	xorl	%esi, %esi
1421b6b52: 4c 3b a5 e8 01 00 00        	cmpq	0x1e8(%rbp), %r12
1421b6b59: 0f 83 8d 01 00 00           	jae	0x1421b6cec <.text+0x21b5cec>
1421b6b5f: 90                          	nop
1421b6b60: 4d 85 e4                    	testq	%r12, %r12
1421b6b63: 78 0b                       	js	0x1421b6b70 <.text+0x21b5b70>
1421b6b65: 0f 57 c9                    	xorps	%xmm1, %xmm1
1421b6b68: f3 49 0f 2a cc              	cvtsi2ss	%r12, %xmm1
1421b6b6d: eb 1c                       	jmp	0x1421b6b8b <.text+0x21b5b8b>
1421b6b6f: 90                          	nop
1421b6b70: 4c 89 e0                    	movq	%r12, %rax
1421b6b73: 48 d1 e8                    	shrq	%rax
1421b6b76: 44 89 e1                    	movl	%r12d, %ecx
1421b6b79: 83 e1 01                    	andl	$0x1, %ecx
1421b6b7c: 48 09 c1                    	orq	%rax, %rcx
1421b6b7f: 0f 57 c9                    	xorps	%xmm1, %xmm1
1421b6b82: f3 48 0f 2a c9              	cvtsi2ss	%rcx, %xmm1
1421b6b87: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1421b6b8b: f3 0f 5e cf                 	divss	%xmm7, %xmm1
1421b6b8f: f3 0f 11 8d c0 00 00 00     	movss	%xmm1, 0xc0(%rbp)
1421b6b97: 0f 2e 0d f2 9a 7a 00        	ucomiss	0x7a9af2(%rip), %xmm1   # 0x142960690
1421b6b9e: 0f 82 92 0c 00 00           	jb	0x1421b7836 <.text+0x21b6836>
1421b6ba4: 44 0f 2e f1                 	ucomiss	%xmm1, %xmm14
1421b6ba8: 0f 82 88 0c 00 00           	jb	0x1421b7836 <.text+0x21b6836>
1421b6bae: 0f 2e c9                    	ucomiss	%xmm1, %xmm1
1421b6bb1: 0f 8a e9 0c 00 00           	jp	0x1421b78a0 <.text+0x21b68a0>
1421b6bb7: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
1421b6bbe: e8 dd 09 ae fe              	callq	0x140c975a0 <.text+0xc965a0>
1421b6bc3: f3 0f 11 85 f0 00 00 00     	movss	%xmm0, 0xf0(%rbp)
1421b6bcb: f3 0f 11 8d f4 00 00 00     	movss	%xmm1, 0xf4(%rbp)
1421b6bd3: 4c 89 f9                    	movq	%r15, %rcx
1421b6bd6: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
1421b6bdd: 0f 28 95 80 00 00 00        	movaps	0x80(%rbp), %xmm2
1421b6be4: e8 f7 61 ae fe              	callq	0x140c9cde0 <.text+0xc9bde0>
1421b6be9: 8b 85 68 01 00 00           	movl	0x168(%rbp), %eax
1421b6bef: 89 85 c8 00 00 00           	movl	%eax, 0xc8(%rbp)
1421b6bf5: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1421b6bfc: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1421b6c03: c7 85 cc 00 00 00 cd cc 4c 3e       	movl	$0x3e4ccccd, 0xcc(%rbp) # imm = 0x3E4CCCCD
1421b6c0d: f3 0f 10 85 f4 01 00 00     	movss	0x1f4(%rbp), %xmm0
1421b6c15: f3 0f 11 85 d0 00 00 00     	movss	%xmm0, 0xd0(%rbp)
1421b6c1d: c7 85 d4 00 00 00 cd cc 4c 3e       	movl	$0x3e4ccccd, 0xd4(%rbp) # imm = 0x3E4CCCCD
1421b6c27: 0f 10 85 cc 00 00 00        	movups	0xcc(%rbp), %xmm0
1421b6c2e: 0f 28 8d d0 00 00 00        	movaps	0xd0(%rbp), %xmm1
1421b6c35: 0f 28 95 c0 00 00 00        	movaps	0xc0(%rbp), %xmm2
1421b6c3c: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1421b6c40: f3 0f 10 1d bc ec 76 00     	movss	0x76ecbc(%rip), %xmm3   # 0x142925904
1421b6c48: 0f 59 c3                    	mulps	%xmm3, %xmm0
1421b6c4b: 0f 28 d9                    	movaps	%xmm1, %xmm3
1421b6c4e: 0f c6 d9 00                 	shufps	$0x0, %xmm1, %xmm3      # xmm3 = xmm3[0,0],xmm1[0,0]
1421b6c52: 0f 59 1d 57 d9 77 00        	mulps	0x77d957(%rip), %xmm3   # 0x1429345b0
1421b6c59: 0f c6 c9 55                 	shufps	$0x55, %xmm1, %xmm1     # xmm1 = xmm1[1,1,1,1]
1421b6c5d: 0f 59 0d fc 44 85 00        	mulps	0x8544fc(%rip), %xmm1   # 0x142a0b160
1421b6c64: 0f c6 d2 a4                 	shufps	$0xa4, %xmm2, %xmm2     # xmm2 = xmm2[0,1,2,2]
1421b6c68: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
1421b6c6f: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
1421b6c76: 0f 29 8d 80 01 00 00        	movaps	%xmm1, 0x180(%rbp)
1421b6c7d: 0f 29 95 90 01 00 00        	movaps	%xmm2, 0x190(%rbp)
1421b6c84: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
1421b6c8b: 4c 89 fa                    	movq	%r15, %rdx
1421b6c8e: e8 cd 68 6e fe              	callq	0x14089d560 <.text+0x89c560>
1421b6c93: 0f 10 85 10 01 00 00        	movups	0x110(%rbp), %xmm0
1421b6c9a: 0f 10 8d 20 01 00 00        	movups	0x120(%rbp), %xmm1
1421b6ca1: 0f 10 95 30 01 00 00        	movups	0x130(%rbp), %xmm2
1421b6ca8: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
1421b6caf: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
1421b6cb6: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
1421b6cbd: 0f 28 05 dc e1 95 00        	movaps	0x95e1dc(%rip), %xmm0   # 0x142b14ea0
1421b6cc4: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
1421b6ccb: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1421b6ccf: 4c 89 fa                    	movq	%r15, %rdx
1421b6cd2: e8 29 f3 ff ff              	callq	0x1421b6000 <.text+0x21b5000>
1421b6cd7: 49 ff c4                    	incq	%r12
1421b6cda: 08 c3                       	orb	%al, %bl
1421b6cdc: 48 ff c6                    	incq	%rsi
1421b6cdf: 4c 3b a5 e8 01 00 00        	cmpq	0x1e8(%rbp), %r12
1421b6ce6: 0f 82 74 fe ff ff           	jb	0x1421b6b60 <.text+0x21b5b60>
1421b6cec: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
1421b6cf0: f3 0f 10 8d f4 01 00 00     	movss	0x1f4(%rbp), %xmm1
1421b6cf8: e8 83 e2 97 fe              	callq	0x140b34f80 <.text+0xb33f80>
1421b6cfd: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
1421b6d01: 48 8b 45 18                 	movq	0x18(%rbp), %rax
1421b6d05: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1421b6d09: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
1421b6d0d: c7 85 a8 00 00 00 00 00 00 00       	movl	$0x0, 0xa8(%rbp)
1421b6d17: f3 0f 10 85 f4 01 00 00     	movss	0x1f4(%rbp), %xmm0
1421b6d1f: f3 0f 11 85 ac 00 00 00     	movss	%xmm0, 0xac(%rbp)
1421b6d27: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
1421b6d2e: 48 89 c8                    	movq	%rcx, %rax
1421b6d31: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1421b6d3c: 4c 8d a5 10 01 00 00        	leaq	0x110(%rbp), %r12
1421b6d43: eb 12                       	jmp	0x1421b6d57 <.text+0x21b5d57>
1421b6d45: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1421b6d50: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
1421b6d57: 48 85 c0                    	testq	%rax, %rax
1421b6d5a: 74 14                       	je	0x1421b6d70 <.text+0x21b5d70>
1421b6d5c: 48 3b 45 38                 	cmpq	0x38(%rbp), %rax
1421b6d60: 74 0e                       	je	0x1421b6d70 <.text+0x21b5d70>
1421b6d62: 48 8d 48 04                 	leaq	0x4(%rax), %rcx
1421b6d66: 48 89 8d e8 01 00 00        	movq	%rcx, 0x1e8(%rbp)
1421b6d6d: eb 34                       	jmp	0x1421b6da3 <.text+0x21b5da3>
1421b6d6f: 90                          	nop
1421b6d70: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
1421b6d77: 48 83 f8 08                 	cmpq	$0x8, %rax
1421b6d7b: 0f 84 9f 04 00 00           	je	0x1421b7220 <.text+0x21b6220>
1421b6d81: 48 89 c1                    	movq	%rax, %rcx
1421b6d84: 48 01 e8                    	addq	%rbp, %rax
1421b6d87: 48 05 a8 00 00 00           	addq	$0xa8, %rax
1421b6d8d: 48 83 c1 04                 	addq	$0x4, %rcx
1421b6d91: 48 89 8d e0 01 00 00        	movq	%rcx, 0x1e0(%rbp)
1421b6d98: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
1421b6da3: f3 0f 10 30                 	movss	(%rax), %xmm6
1421b6da7: 4c 89 f9                    	movq	%r15, %rcx
1421b6daa: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
1421b6db1: 4c 8b 85 58 01 00 00        	movq	0x158(%rbp), %r8
1421b6db8: e8 e3 0d ae fe              	callq	0x140c97ba0 <.text+0xc96ba0>
1421b6dbd: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
1421b6dc4: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
1421b6dcb: 0f 29 8d 20 01 00 00        	movaps	%xmm1, 0x120(%rbp)
1421b6dd2: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1421b6dd9: c7 85 30 01 00 00 00 00 00 00       	movl	$0x0, 0x130(%rbp)
1421b6de3: f3 0f 58 b5 80 00 00 00     	addss	0x80(%rbp), %xmm6
1421b6deb: 44 0f 28 c6                 	movaps	%xmm6, %xmm8
1421b6def: f3 44 0f 59 05 98 98 7a 00  	mulss	0x7a9898(%rip), %xmm8   # 0x142960690
1421b6df8: 41 0f 28 f8                 	movaps	%xmm8, %xmm7
1421b6dfc: 0f 14 fe                    	unpcklps	%xmm6, %xmm7            # xmm7 = xmm7[0],xmm6[0],xmm7[1],xmm6[1]
1421b6dff: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
1421b6e06: 48 3b 85 28 01 00 00        	cmpq	0x128(%rbp), %rax
1421b6e0d: 0f 83 3d ff ff ff           	jae	0x1421b6d50 <.text+0x21b5d50>
1421b6e13: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1421b6e20: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
1421b6e24: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
1421b6e2b: 48 85 c0                    	testq	%rax, %rax
1421b6e2e: 78 30                       	js	0x1421b6e60 <.text+0x21b5e60>
1421b6e30: 0f 57 c9                    	xorps	%xmm1, %xmm1
1421b6e33: f3 48 0f 2a c8              	cvtsi2ss	%rax, %xmm1
1421b6e38: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
1421b6e3f: 48 85 c0                    	testq	%rax, %rax
1421b6e42: 79 40                       	jns	0x1421b6e84 <.text+0x21b5e84>
1421b6e44: 48 89 c1                    	movq	%rax, %rcx
1421b6e47: 48 d1 e9                    	shrq	%rcx
1421b6e4a: 83 e0 01                    	andl	$0x1, %eax
1421b6e4d: 48 09 c8                    	orq	%rcx, %rax
1421b6e50: 0f 57 c0                    	xorps	%xmm0, %xmm0
1421b6e53: f3 48 0f 2a c0              	cvtsi2ss	%rax, %xmm0
1421b6e58: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1421b6e5c: eb 2e                       	jmp	0x1421b6e8c <.text+0x21b5e8c>
1421b6e5e: 66 90                       	nop
1421b6e60: 48 89 c1                    	movq	%rax, %rcx
1421b6e63: 48 d1 e9                    	shrq	%rcx
1421b6e66: 83 e0 01                    	andl	$0x1, %eax
1421b6e69: 48 09 c8                    	orq	%rcx, %rax
1421b6e6c: 0f 57 c9                    	xorps	%xmm1, %xmm1
1421b6e6f: f3 48 0f 2a c8              	cvtsi2ss	%rax, %xmm1
1421b6e74: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1421b6e78: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
1421b6e7f: 48 85 c0                    	testq	%rax, %rax
1421b6e82: 78 c0                       	js	0x1421b6e44 <.text+0x21b5e44>
1421b6e84: 0f 57 c0                    	xorps	%xmm0, %xmm0
1421b6e87: f3 48 0f 2a c0              	cvtsi2ss	%rax, %xmm0
1421b6e8c: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
1421b6e93: f3 0f 58 05 49 32 7a 00     	addss	0x7a3249(%rip), %xmm0   # 0x14295a0e4
1421b6e9b: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1421b6e9f: f3 0f 11 8d b8 00 00 00     	movss	%xmm1, 0xb8(%rbp)
1421b6ea7: 0f 2e 0d e2 97 7a 00        	ucomiss	0x7a97e2(%rip), %xmm1   # 0x142960690
1421b6eae: 0f 82 f4 08 00 00           	jb	0x1421b77a8 <.text+0x21b67a8>
1421b6eb4: 44 0f 2e f1                 	ucomiss	%xmm1, %xmm14
1421b6eb8: 0f 82 ea 08 00 00           	jb	0x1421b77a8 <.text+0x21b67a8>
1421b6ebe: 0f 2e c9                    	ucomiss	%xmm1, %xmm1
1421b6ec1: 0f 8a 52 09 00 00           	jp	0x1421b7819 <.text+0x21b6819>
1421b6ec7: e8 d4 06 ae fe              	callq	0x140c975a0 <.text+0xc965a0>
1421b6ecc: f3 0f 11 85 60 01 00 00     	movss	%xmm0, 0x160(%rbp)
1421b6ed4: f3 0f 11 8d 64 01 00 00     	movss	%xmm1, 0x164(%rbp)
1421b6edc: 48 89 f9                    	movq	%rdi, %rcx
1421b6edf: 4c 89 fa                    	movq	%r15, %rdx
1421b6ee2: e8 d9 5e ae fe              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1421b6ee7: 83 bd 30 01 00 00 01        	cmpl	$0x1, 0x130(%rbp)
1421b6eee: 75 50                       	jne	0x1421b6f40 <.text+0x21b5f40>
1421b6ef0: 41 8b 46 18                 	movl	0x18(%r14), %eax
1421b6ef4: 41 89 46 0c                 	movl	%eax, 0xc(%r14)
1421b6ef8: 49 8b 46 10                 	movq	0x10(%r14), %rax
1421b6efc: 49 89 46 04                 	movq	%rax, 0x4(%r14)
1421b6f00: 8b 47 08                    	movl	0x8(%rdi), %eax
1421b6f03: 41 89 46 18                 	movl	%eax, 0x18(%r14)
1421b6f07: 48 8b 07                    	movq	(%rdi), %rax
1421b6f0a: 49 89 46 10                 	movq	%rax, 0x10(%r14)
1421b6f0e: f3 44 0f 10 a5 3c 01 00 00  	movss	0x13c(%rbp), %xmm12
1421b6f17: f2 44 0f 10 95 34 01 00 00  	movsd	0x134(%rbp), %xmm10
1421b6f20: f2 44 0f 10 8d 9c 00 00 00  	movsd	0x9c(%rbp), %xmm9
1421b6f29: f3 44 0f 10 bd a4 00 00 00  	movss	0xa4(%rbp), %xmm15
1421b6f32: e9 80 00 00 00              	jmp	0x1421b6fb7 <.text+0x21b5fb7>
1421b6f37: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421b6f40: 8b 47 08                    	movl	0x8(%rdi), %eax
1421b6f43: 48 8d 8d c4 00 00 00        	leaq	0xc4(%rbp), %rcx
1421b6f4a: 89 41 08                    	movl	%eax, 0x8(%rcx)
1421b6f4d: 48 8b 07                    	movq	(%rdi), %rax
1421b6f50: 48 89 01                    	movq	%rax, (%rcx)
1421b6f53: c7 85 c0 00 00 00 01 00 00 00       	movl	$0x1, 0xc0(%rbp)
1421b6f5d: 4c 89 a5 d0 00 00 00        	movq	%r12, 0xd0(%rbp)
1421b6f64: 4c 89 f9                    	movq	%r15, %rcx
1421b6f67: 4c 89 ea                    	movq	%r13, %rdx
1421b6f6a: e8 71 79 3d 00              	callq	0x14258e8e0 <.text+0x258d8e0>
1421b6f6f: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
1421b6f76: 0f 10 8d 6c 01 00 00        	movups	0x16c(%rbp), %xmm1
1421b6f7d: 41 0f 11 4e 0c              	movups	%xmm1, 0xc(%r14)
1421b6f82: 41 0f 11 06                 	movups	%xmm0, (%r14)
1421b6f86: 83 bd 30 01 00 00 01        	cmpl	$0x1, 0x130(%rbp)
1421b6f8d: 0f 85 bd fd ff ff           	jne	0x1421b6d50 <.text+0x21b5d50>
1421b6f93: f3 44 0f 10 a5 3c 01 00 00  	movss	0x13c(%rbp), %xmm12
1421b6f9c: f2 44 0f 10 95 34 01 00 00  	movsd	0x134(%rbp), %xmm10
1421b6fa5: f2 44 0f 10 8d 40 01 00 00  	movsd	0x140(%rbp), %xmm9
1421b6fae: f3 44 0f 10 bd 48 01 00 00  	movss	0x148(%rbp), %xmm15
1421b6fb7: 45 0f 28 ea                 	movaps	%xmm10, %xmm13
1421b6fbb: 45 0f 58 e9                 	addps	%xmm9, %xmm13
1421b6fbf: 41 0f 28 f4                 	movaps	%xmm12, %xmm6
1421b6fc3: f3 41 0f 58 f7              	addss	%xmm15, %xmm6
1421b6fc8: 44 0f 59 2d 10 e9 76 00     	mulps	0x76e910(%rip), %xmm13  # 0x1429258e0
1421b6fd0: f3 0f 59 35 98 e8 76 00     	mulss	0x76e898(%rip), %xmm6   # 0x142925870
1421b6fd8: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1421b6fdc: f3 41 0f 5c c2              	subss	%xmm10, %xmm0
1421b6fe1: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
1421b6fe5: f3 41 0f 5c cc              	subss	%xmm12, %xmm1
1421b6fea: 0f 28 d0                    	movaps	%xmm0, %xmm2
1421b6fed: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
1421b6ff1: 0f 28 d9                    	movaps	%xmm1, %xmm3
1421b6ff4: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
1421b6ff8: f3 0f 58 da                 	addss	%xmm2, %xmm3
1421b6ffc: 0f 57 d2                    	xorps	%xmm2, %xmm2
1421b6fff: f3 0f 51 d3                 	sqrtss	%xmm3, %xmm2
1421b7003: 41 0f 28 de                 	movaps	%xmm14, %xmm3
1421b7007: f3 0f 5e da                 	divss	%xmm2, %xmm3
1421b700b: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
1421b700f: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
1421b7013: f3 0f 11 85 98 00 00 00     	movss	%xmm0, 0x98(%rbp)
1421b701b: f3 0f 11 9d 9c 00 00 00     	movss	%xmm3, 0x9c(%rbp)
1421b7023: 0f 57 d2                    	xorps	%xmm2, %xmm2
1421b7026: 4c 89 e9                    	movq	%r13, %rcx
1421b7029: 48 8d 95 98 00 00 00        	leaq	0x98(%rbp), %rdx
1421b7030: e8 ab 5d ae fe              	callq	0x140c9cde0 <.text+0xc9bde0>
1421b7035: 4c 89 f9                    	movq	%r15, %rcx
1421b7038: 4c 89 ea                    	movq	%r13, %rdx
1421b703b: e8 00 9a ad fe              	callq	0x140c90a40 <.text+0xc8fa40>
1421b7040: 44 0f 58 ef                 	addps	%xmm7, %xmm13
1421b7044: f3 41 0f 58 f0              	addss	%xmm8, %xmm6
1421b7049: 44 0f 28 9d 60 01 00 00     	movaps	0x160(%rbp), %xmm11
1421b7051: f3 0f 10 9d 68 01 00 00     	movss	0x168(%rbp), %xmm3
1421b7059: 45 0f 5c d1                 	subps	%xmm9, %xmm10
1421b705d: f3 45 0f 5c e7              	subss	%xmm15, %xmm12
1421b7062: 45 0f 59 d2                 	mulps	%xmm10, %xmm10
1421b7066: f3 41 0f 16 c2              	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
1421b706b: f3 41 0f 58 c2              	addss	%xmm10, %xmm0
1421b7070: f3 45 0f 59 e4              	mulss	%xmm12, %xmm12
1421b7075: f3 44 0f 58 e0              	addss	%xmm0, %xmm12
1421b707a: 0f 57 c0                    	xorps	%xmm0, %xmm0
1421b707d: f3 41 0f 51 c4              	sqrtss	%xmm12, %xmm0
1421b7082: 44 0f 13 ad f0 00 00 00     	movlps	%xmm13, 0xf0(%rbp)
1421b708a: f3 0f 11 b5 f8 00 00 00     	movss	%xmm6, 0xf8(%rbp)
1421b7092: f2 0f 10 0d 56 56 8d 00     	movsd	0x8d5656(%rip), %xmm1   # 0x142a8c6f0
1421b709a: f2 0f 11 8d fc 00 00 00     	movsd	%xmm1, 0xfc(%rbp)
1421b70a2: f3 0f 11 85 04 01 00 00     	movss	%xmm0, 0x104(%rbp)
1421b70aa: 0f 10 95 fc 00 00 00        	movups	0xfc(%rbp), %xmm2
1421b70b1: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
1421b70b8: 0f 28 8d f0 00 00 00        	movaps	0xf0(%rbp), %xmm1
1421b70bf: 44 0f 28 cb                 	movaps	%xmm3, %xmm9
1421b70c3: f3 44 0f 58 cb              	addss	%xmm3, %xmm9
1421b70c8: f3 41 0f 59 d9              	mulss	%xmm9, %xmm3
1421b70cd: 41 0f 28 eb                 	movaps	%xmm11, %xmm5
1421b70d1: 41 0f 58 eb                 	addps	%xmm11, %xmm5
1421b70d5: f3 0f 16 e5                 	movshdup	%xmm5, %xmm4            # xmm4 = xmm5[1,1,3,3]
1421b70d9: f3 41 0f 59 e3              	mulss	%xmm11, %xmm4
1421b70de: 41 0f 28 f3                 	movaps	%xmm11, %xmm6
1421b70e2: f3 45 0f 12 d1              	movsldup	%xmm9, %xmm10           # xmm10 = xmm9[0,0,2,2]
1421b70e7: 45 0f 59 d3                 	mulps	%xmm11, %xmm10
1421b70eb: 45 0f c6 db ff              	shufps	$0xff, %xmm11, %xmm11   # xmm11 = xmm11[3,3,3,3]
1421b70f0: 0f 59 f5                    	mulps	%xmm5, %xmm6
1421b70f3: 41 0f 59 eb                 	mulps	%xmm11, %xmm5
1421b70f7: 45 0f 28 e3                 	movaps	%xmm11, %xmm12
1421b70fb: f3 45 0f 59 e1              	mulss	%xmm9, %xmm12
1421b7100: 0f c6 ed e1                 	shufps	$0xe1, %xmm5, %xmm5     # xmm5 = xmm5[1,0,2,3]
1421b7104: f3 44 0f 16 ee              	movshdup	%xmm6, %xmm13           # xmm13 = xmm6[1,1,3,3]
1421b7109: 45 0f 28 cd                 	movaps	%xmm13, %xmm9
1421b710d: f3 44 0f 58 cb              	addss	%xmm3, %xmm9
1421b7112: 45 0f 28 de                 	movaps	%xmm14, %xmm11
1421b7116: f3 45 0f 5c d9              	subss	%xmm9, %xmm11
1421b711b: 44 0f 28 cc                 	movaps	%xmm4, %xmm9
1421b711f: f3 45 0f 58 cc              	addss	%xmm12, %xmm9
1421b7124: 45 0f 14 d9                 	unpcklps	%xmm9, %xmm11           # xmm11 = xmm11[0],xmm9[0],xmm11[1],xmm9[1]
1421b7128: 45 0f 28 ca                 	movaps	%xmm10, %xmm9
1421b712c: 44 0f 5c cd                 	subps	%xmm5, %xmm9
1421b7130: 45 0f c6 d9 04              	shufps	$0x4, %xmm9, %xmm11     # xmm11 = xmm11[0,1],xmm9[0,0]
1421b7135: f3 41 0f 5c e4              	subss	%xmm12, %xmm4
1421b713a: f3 0f 58 de                 	addss	%xmm6, %xmm3
1421b713e: 45 0f 28 e6                 	movaps	%xmm14, %xmm12
1421b7142: f3 44 0f 5c e3              	subss	%xmm3, %xmm12
1421b7147: 41 0f 14 e4                 	unpcklps	%xmm12, %xmm4           # xmm4 = xmm4[0],xmm12[0],xmm4[1],xmm12[1]
1421b714b: 41 0f 58 ea                 	addps	%xmm10, %xmm5
1421b714f: 0f c6 e5 54                 	shufps	$0x54, %xmm5, %xmm4     # xmm4 = xmm4[0,1],xmm5[1,1]
1421b7153: 44 0f c6 cd 01              	shufps	$0x1, %xmm5, %xmm9      # xmm9 = xmm9[1,0],xmm5[0,0]
1421b7158: f3 41 0f 58 f5              	addss	%xmm13, %xmm6
1421b715d: 41 0f 28 de                 	movaps	%xmm14, %xmm3
1421b7161: f3 0f 5c de                 	subss	%xmm6, %xmm3
1421b7165: 44 0f c6 cb 02              	shufps	$0x2, %xmm3, %xmm9      # xmm9 = xmm9[2,0],xmm3[0,0]
1421b716a: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1421b716e: 41 0f 59 d3                 	mulps	%xmm11, %xmm2
1421b7172: 0f 28 d8                    	movaps	%xmm0, %xmm3
1421b7175: 0f c6 d8 00                 	shufps	$0x0, %xmm0, %xmm3      # xmm3 = xmm3[0,0],xmm0[0,0]
1421b7179: 0f 59 dc                    	mulps	%xmm4, %xmm3
1421b717c: 0f c6 c0 55                 	shufps	$0x55, %xmm0, %xmm0     # xmm0 = xmm0[1,1,1,1]
1421b7180: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
1421b7184: 0f c6 c9 a4                 	shufps	$0xa4, %xmm1, %xmm1     # xmm1 = xmm1[0,1,2,2]
1421b7188: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
1421b718f: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
1421b7196: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
1421b719d: 0f 29 8d 90 01 00 00        	movaps	%xmm1, 0x190(%rbp)
1421b71a4: 4c 89 e9                    	movq	%r13, %rcx
1421b71a7: 4c 89 fa                    	movq	%r15, %rdx
1421b71aa: e8 b1 63 6e fe              	callq	0x14089d560 <.text+0x89c560>
1421b71af: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1421b71b6: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
1421b71bd: 0f 10 95 e0 00 00 00        	movups	0xe0(%rbp), %xmm2
1421b71c4: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
1421b71cb: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
1421b71d2: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
1421b71d9: 0f 28 05 c0 dc 95 00        	movaps	0x95dcc0(%rip), %xmm0   # 0x142b14ea0
1421b71e0: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
1421b71e7: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1421b71eb: 4c 89 fa                    	movq	%r15, %rdx
1421b71ee: e8 0d ee ff ff              	callq	0x1421b6000 <.text+0x21b5000>
1421b71f3: 08 c3                       	orb	%al, %bl
1421b71f5: 48 ff c6                    	incq	%rsi
1421b71f8: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
1421b71ff: 48 3b 85 28 01 00 00        	cmpq	0x128(%rbp), %rax
1421b7206: 0f 82 14 fc ff ff           	jb	0x1421b6e20 <.text+0x21b5e20>
1421b720c: e9 3f fb ff ff              	jmp	0x1421b6d50 <.text+0x21b5d50>
1421b7211: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1421b7220: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
1421b7224: 48 85 d2                    	testq	%rdx, %rdx
1421b7227: 74 16                       	je	0x1421b723f <.text+0x21b623f>
1421b7229: 48 c1 e2 02                 	shlq	$0x2, %rdx
1421b722d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1421b7233: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
1421b723a: e8 e1 c9 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421b723f: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
1421b7243: 80 79 1c 00                 	cmpb	$0x0, 0x1c(%rcx)
1421b7247: 74 21                       	je	0x1421b726a <.text+0x21b626a>
1421b7249: 8b 85 bc 00 00 00           	movl	0xbc(%rbp), %eax
1421b724f: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1421b7253: 89 01                       	movl	%eax, (%rcx)
1421b7255: 48 8b 45 58                 	movq	0x58(%rbp), %rax
1421b7259: 48 89 70 08                 	movq	%rsi, 0x8(%rax)
1421b725d: 4c 8b 5d 70                 	movq	0x70(%rbp), %r11
1421b7261: 4c 8b 65 78                 	movq	0x78(%rbp), %r12
1421b7265: e9 85 f7 ff ff              	jmp	0x1421b69ef <.text+0x21b59ef>
1421b726a: e8 01 ca 72 fe              	callq	0x1408e3c70 <.text+0x8e2c70>
1421b726f: f3 0f 11 85 e8 01 00 00     	movss	%xmm0, 0x1e8(%rbp)
1421b7277: 4c 89 f9                    	movq	%r15, %rcx
1421b727a: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
1421b7281: 4c 8b 85 58 01 00 00        	movq	0x158(%rbp), %r8
1421b7288: e8 13 09 ae fe              	callq	0x140c97ba0 <.text+0xc96ba0>
1421b728d: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
1421b7294: 48 85 c0                    	testq	%rax, %rax
1421b7297: 78 07                       	js	0x1421b72a0 <.text+0x21b62a0>
1421b7299: f3 4c 0f 2a f8              	cvtsi2ss	%rax, %xmm15
1421b729e: eb 16                       	jmp	0x1421b72b6 <.text+0x21b62b6>
1421b72a0: 48 89 c1                    	movq	%rax, %rcx
1421b72a3: 48 d1 e9                    	shrq	%rcx
1421b72a6: 83 e0 01                    	andl	$0x1, %eax
1421b72a9: 48 09 c8                    	orq	%rcx, %rax
1421b72ac: f3 4c 0f 2a f8              	cvtsi2ss	%rax, %xmm15
1421b72b1: f3 45 0f 58 ff              	addss	%xmm15, %xmm15
1421b72b6: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
1421b72bd: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1421b72c4: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1421b72cb: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1421b72d2: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1421b72d9: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1421b72e0: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
1421b72e7: f3 0f 58 85 f4 01 00 00     	addss	0x1f4(%rbp), %xmm0
1421b72ef: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1421b72f3: f3 44 0f 59 05 94 93 7a 00  	mulss	0x7a9394(%rip), %xmm8   # 0x142960690
1421b72fc: f3 0f 10 8d e8 01 00 00     	movss	0x1e8(%rbp), %xmm1
1421b7304: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1421b7308: f3 0f 11 8d e8 01 00 00     	movss	%xmm1, 0x1e8(%rbp)
1421b7310: f3 44 0f 58 3d cb 2d 7a 00  	addss	0x7a2dcb(%rip), %xmm15  # 0x14295a0e4
1421b7319: 41 0f 28 f8                 	movaps	%xmm8, %xmm7
1421b731d: 0f 14 f8                    	unpcklps	%xmm0, %xmm7            # xmm7 = xmm7[0],xmm0[0],xmm7[1],xmm0[1]
1421b7320: 48 3b 8d e0 01 00 00        	cmpq	0x1e0(%rbp), %rcx
1421b7327: 0f 83 1c ff ff ff           	jae	0x1421b7249 <.text+0x21b6249>
1421b732d: 0f 1f 00                    	nopl	(%rax)
1421b7330: 48 85 c9                    	testq	%rcx, %rcx
1421b7333: 78 1b                       	js	0x1421b7350 <.text+0x21b6350>
1421b7335: 49 89 cc                    	movq	%rcx, %r12
1421b7338: 0f 57 c9                    	xorps	%xmm1, %xmm1
1421b733b: f3 48 0f 2a c9              	cvtsi2ss	%rcx, %xmm1
1421b7340: eb 29                       	jmp	0x1421b736b <.text+0x21b636b>
1421b7342: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1421b7350: 48 89 c8                    	movq	%rcx, %rax
1421b7353: 48 d1 e8                    	shrq	%rax
1421b7356: 49 89 cc                    	movq	%rcx, %r12
1421b7359: 83 e1 01                    	andl	$0x1, %ecx
1421b735c: 48 09 c1                    	orq	%rax, %rcx
1421b735f: 0f 57 c9                    	xorps	%xmm1, %xmm1
1421b7362: f3 48 0f 2a c9              	cvtsi2ss	%rcx, %xmm1
1421b7367: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1421b736b: f3 41 0f 5e cf              	divss	%xmm15, %xmm1
1421b7370: f3 0f 11 8d c0 00 00 00     	movss	%xmm1, 0xc0(%rbp)
1421b7378: 0f 2e 0d 11 93 7a 00        	ucomiss	0x7a9311(%rip), %xmm1   # 0x142960690
1421b737f: 0f 82 35 05 00 00           	jb	0x1421b78ba <.text+0x21b68ba>
1421b7385: 44 0f 2e f1                 	ucomiss	%xmm1, %xmm14
1421b7389: 0f 82 2b 05 00 00           	jb	0x1421b78ba <.text+0x21b68ba>
1421b738f: 0f 2e c9                    	ucomiss	%xmm1, %xmm1
1421b7392: 0f 8a 82 05 00 00           	jp	0x1421b791a <.text+0x21b691a>
1421b7398: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
1421b739f: e8 fc 01 ae fe              	callq	0x140c975a0 <.text+0xc965a0>
1421b73a4: f3 0f 11 85 60 01 00 00     	movss	%xmm0, 0x160(%rbp)
1421b73ac: f3 0f 11 8d 64 01 00 00     	movss	%xmm1, 0x164(%rbp)
1421b73b4: 48 8d 8d f4 00 00 00        	leaq	0xf4(%rbp), %rcx
1421b73bb: 4c 89 fa                    	movq	%r15, %rdx
1421b73be: e8 fd 59 ae fe              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1421b73c3: c7 85 f0 00 00 00 01 00 00 00       	movl	$0x1, 0xf0(%rbp)
1421b73cd: f2 44 0f 10 8d f4 00 00 00  	movsd	0xf4(%rbp), %xmm9
1421b73d6: f3 0f 10 b5 fc 00 00 00     	movss	0xfc(%rbp), %xmm6
1421b73de: f3 41 0f 58 f0              	addss	%xmm8, %xmm6
1421b73e3: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1421b73e7: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
1421b73ee: f3 0f 10 95 e8 01 00 00     	movss	0x1e8(%rbp), %xmm2
1421b73f6: e8 e5 59 ae fe              	callq	0x140c9cde0 <.text+0xc9bde0>
1421b73fb: 44 0f 58 cf                 	addps	%xmm7, %xmm9
1421b73ff: f2 44 0f 10 65 20           	movsd	0x20(%rbp), %xmm12
1421b7405: f3 44 0f 10 55 28           	movss	0x28(%rbp), %xmm10
1421b740b: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
1421b740f: f3 0f 5c c6                 	subss	%xmm6, %xmm0
1421b7413: 0f 28 c8                    	movaps	%xmm0, %xmm1
1421b7416: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1421b741a: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
1421b741e: 41 0f 5c d1                 	subps	%xmm9, %xmm2
1421b7422: 0f 28 da                    	movaps	%xmm2, %xmm3
1421b7425: 0f 59 da                    	mulps	%xmm2, %xmm3
1421b7428: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
1421b742c: f3 0f 58 e3                 	addss	%xmm3, %xmm4
1421b7430: f3 0f 58 e1                 	addss	%xmm1, %xmm4
1421b7434: 0f 57 c9                    	xorps	%xmm1, %xmm1
1421b7437: f3 0f 51 cc                 	sqrtss	%xmm4, %xmm1
1421b743b: 41 0f 28 de                 	movaps	%xmm14, %xmm3
1421b743f: f3 0f 5e d9                 	divss	%xmm1, %xmm3
1421b7443: f3 0f 12 cb                 	movsldup	%xmm3, %xmm1            # xmm1 = xmm3[0,0,2,2]
1421b7447: 0f 59 ca                    	mulps	%xmm2, %xmm1
1421b744a: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
1421b744e: 0f 13 8d 10 01 00 00        	movlps	%xmm1, 0x110(%rbp)
1421b7455: f3 0f 11 9d 18 01 00 00     	movss	%xmm3, 0x118(%rbp)
1421b745d: 4c 89 f9                    	movq	%r15, %rcx
1421b7460: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
1421b7467: e8 d4 95 ad fe              	callq	0x140c90a40 <.text+0xc8fa40>
1421b746c: 0f 28 c6                    	movaps	%xmm6, %xmm0
1421b746f: f3 41 0f 58 c2              	addss	%xmm10, %xmm0
1421b7474: f3 0f 59 05 f4 e3 76 00     	mulss	0x76e3f4(%rip), %xmm0   # 0x142925870
1421b747c: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1421b7480: 41 0f 58 cc                 	addps	%xmm12, %xmm1
1421b7484: 44 0f 28 ad 60 01 00 00     	movaps	0x160(%rbp), %xmm13
1421b748c: f3 0f 10 9d 68 01 00 00     	movss	0x168(%rbp), %xmm3
1421b7494: 45 0f 5c cc                 	subps	%xmm12, %xmm9
1421b7498: f3 41 0f 5c f2              	subss	%xmm10, %xmm6
1421b749d: 45 0f 59 c9                 	mulps	%xmm9, %xmm9
1421b74a1: f3 41 0f 16 d1              	movshdup	%xmm9, %xmm2            # xmm2 = xmm9[1,1,3,3]
1421b74a6: f3 41 0f 58 d1              	addss	%xmm9, %xmm2
1421b74ab: f3 0f 59 f6                 	mulss	%xmm6, %xmm6
1421b74af: f3 0f 58 f2                 	addss	%xmm2, %xmm6
1421b74b3: 0f 57 d2                    	xorps	%xmm2, %xmm2
1421b74b6: f3 0f 51 d6                 	sqrtss	%xmm6, %xmm2
1421b74ba: 0f 59 0d 1f e4 76 00        	mulps	0x76e41f(%rip), %xmm1   # 0x1429258e0
1421b74c1: 0f 13 8d c0 00 00 00        	movlps	%xmm1, 0xc0(%rbp)
1421b74c8: f3 0f 11 85 c8 00 00 00     	movss	%xmm0, 0xc8(%rbp)
1421b74d0: f2 0f 10 05 18 52 8d 00     	movsd	0x8d5218(%rip), %xmm0   # 0x142a8c6f0
1421b74d8: f2 0f 11 85 cc 00 00 00     	movsd	%xmm0, 0xcc(%rbp)
1421b74e0: f3 0f 11 95 d4 00 00 00     	movss	%xmm2, 0xd4(%rbp)
1421b74e8: 0f 10 95 cc 00 00 00        	movups	0xcc(%rbp), %xmm2
1421b74ef: 0f 28 85 d0 00 00 00        	movaps	0xd0(%rbp), %xmm0
1421b74f6: 0f 28 8d c0 00 00 00        	movaps	0xc0(%rbp), %xmm1
1421b74fd: 44 0f 28 cb                 	movaps	%xmm3, %xmm9
1421b7501: f3 44 0f 58 cb              	addss	%xmm3, %xmm9
1421b7506: f3 41 0f 59 d9              	mulss	%xmm9, %xmm3
1421b750b: 41 0f 28 ed                 	movaps	%xmm13, %xmm5
1421b750f: 41 0f 58 ed                 	addps	%xmm13, %xmm5
1421b7513: f3 0f 16 e5                 	movshdup	%xmm5, %xmm4            # xmm4 = xmm5[1,1,3,3]
1421b7517: f3 41 0f 59 e5              	mulss	%xmm13, %xmm4
1421b751c: 41 0f 28 f5                 	movaps	%xmm13, %xmm6
1421b7520: f3 45 0f 12 d1              	movsldup	%xmm9, %xmm10           # xmm10 = xmm9[0,0,2,2]
1421b7525: 45 0f 59 d5                 	mulps	%xmm13, %xmm10
1421b7529: 45 0f c6 ed ff              	shufps	$0xff, %xmm13, %xmm13   # xmm13 = xmm13[3,3,3,3]
1421b752e: 0f 59 f5                    	mulps	%xmm5, %xmm6
1421b7531: 41 0f 59 ed                 	mulps	%xmm13, %xmm5
1421b7535: f3 45 0f 59 e9              	mulss	%xmm9, %xmm13
1421b753a: 0f c6 ed e1                 	shufps	$0xe1, %xmm5, %xmm5     # xmm5 = xmm5[1,0,2,3]
1421b753e: f3 44 0f 16 e6              	movshdup	%xmm6, %xmm12           # xmm12 = xmm6[1,1,3,3]
1421b7543: 45 0f 28 cc                 	movaps	%xmm12, %xmm9
1421b7547: f3 44 0f 58 cb              	addss	%xmm3, %xmm9
1421b754c: 45 0f 28 de                 	movaps	%xmm14, %xmm11
1421b7550: f3 45 0f 5c d9              	subss	%xmm9, %xmm11
1421b7555: 44 0f 28 cc                 	movaps	%xmm4, %xmm9
1421b7559: f3 45 0f 58 cd              	addss	%xmm13, %xmm9
1421b755e: 45 0f 14 d9                 	unpcklps	%xmm9, %xmm11           # xmm11 = xmm11[0],xmm9[0],xmm11[1],xmm9[1]
1421b7562: 45 0f 28 ca                 	movaps	%xmm10, %xmm9
1421b7566: 44 0f 5c cd                 	subps	%xmm5, %xmm9
1421b756a: 45 0f c6 d9 04              	shufps	$0x4, %xmm9, %xmm11     # xmm11 = xmm11[0,1],xmm9[0,0]
1421b756f: f3 41 0f 5c e5              	subss	%xmm13, %xmm4
1421b7574: f3 0f 58 de                 	addss	%xmm6, %xmm3
1421b7578: 45 0f 28 ee                 	movaps	%xmm14, %xmm13
1421b757c: f3 44 0f 5c eb              	subss	%xmm3, %xmm13
1421b7581: 41 0f 14 e5                 	unpcklps	%xmm13, %xmm4           # xmm4 = xmm4[0],xmm13[0],xmm4[1],xmm13[1]
1421b7585: 41 0f 58 ea                 	addps	%xmm10, %xmm5
1421b7589: 0f c6 e5 54                 	shufps	$0x54, %xmm5, %xmm4     # xmm4 = xmm4[0,1],xmm5[1,1]
1421b758d: 44 0f c6 cd 01              	shufps	$0x1, %xmm5, %xmm9      # xmm9 = xmm9[1,0],xmm5[0,0]
1421b7592: f3 41 0f 58 f4              	addss	%xmm12, %xmm6
1421b7597: 41 0f 28 de                 	movaps	%xmm14, %xmm3
1421b759b: f3 0f 5c de                 	subss	%xmm6, %xmm3
1421b759f: 44 0f c6 cb 02              	shufps	$0x2, %xmm3, %xmm9      # xmm9 = xmm9[2,0],xmm3[0,0]
1421b75a4: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1421b75a8: 41 0f 59 d3                 	mulps	%xmm11, %xmm2
1421b75ac: 0f 28 d8                    	movaps	%xmm0, %xmm3
1421b75af: 0f c6 d8 00                 	shufps	$0x0, %xmm0, %xmm3      # xmm3 = xmm3[0,0],xmm0[0,0]
1421b75b3: 0f 59 dc                    	mulps	%xmm4, %xmm3
1421b75b6: 0f c6 c0 55                 	shufps	$0x55, %xmm0, %xmm0     # xmm0 = xmm0[1,1,1,1]
1421b75ba: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
1421b75be: 0f c6 c9 a4                 	shufps	$0xa4, %xmm1, %xmm1     # xmm1 = xmm1[0,1,2,2]
1421b75c2: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
1421b75c9: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
1421b75d0: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
1421b75d7: 0f 29 8d 90 01 00 00        	movaps	%xmm1, 0x190(%rbp)
1421b75de: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
1421b75e5: 4c 89 fa                    	movq	%r15, %rdx
1421b75e8: e8 73 5f 6e fe              	callq	0x14089d560 <.text+0x89c560>
1421b75ed: 0f 10 85 10 01 00 00        	movups	0x110(%rbp), %xmm0
1421b75f4: 0f 10 8d 20 01 00 00        	movups	0x120(%rbp), %xmm1
1421b75fb: 0f 10 95 30 01 00 00        	movups	0x130(%rbp), %xmm2
1421b7602: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
1421b7609: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
1421b7610: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
1421b7617: 0f 28 05 82 d8 95 00        	movaps	0x95d882(%rip), %xmm0   # 0x142b14ea0
1421b761e: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
1421b7625: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1421b7629: 4c 89 fa                    	movq	%r15, %rdx
1421b762c: e8 cf e9 ff ff              	callq	0x1421b6000 <.text+0x21b5000>
1421b7631: 49 ff c4                    	incq	%r12
1421b7634: 08 c3                       	orb	%al, %bl
1421b7636: 4c 89 e1                    	movq	%r12, %rcx
1421b7639: 48 ff c6                    	incq	%rsi
1421b763c: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
1421b7643: 48 3b 8d e0 01 00 00        	cmpq	0x1e0(%rbp), %rcx
1421b764a: 0f 82 e0 fc ff ff           	jb	0x1421b7330 <.text+0x21b6330>
1421b7650: e9 f4 fb ff ff              	jmp	0x1421b7249 <.text+0x21b6249>
1421b7655: f6 c3 01                    	testb	$0x1, %bl
1421b7658: 0f 84 c9 00 00 00           	je	0x1421b7727 <.text+0x21b6727>
1421b765e: 48 8b 05 43 4d 47 01        	movq	0x1474d43(%rip), %rax   # 0x14362c3a8
1421b7665: 48 8b 00                    	movq	(%rax), %rax
1421b7668: 48 85 c0                    	testq	%rax, %rax
1421b766b: 0f 84 b6 00 00 00           	je	0x1421b7727 <.text+0x21b6727>
1421b7671: 48 8d 0d d8 18 e4 00        	leaq	0xe418d8(%rip), %rcx    # 0x142ff8f50
1421b7678: e8 03 67 4c fe              	callq	0x14067dd80 <.text+0x67cd80>
1421b767d: 0f 10 00                    	movups	(%rax), %xmm0
1421b7680: 8b 40 10                    	movl	0x10(%rax), %eax
1421b7683: 48 c7 85 90 01 00 00 01 00 00 00    	movq	$0x1, 0x190(%rbp)
1421b768e: 48 8d 0d d3 18 e4 00        	leaq	0xe418d3(%rip), %rcx    # 0x142ff8f68
1421b7695: 48 89 8d 98 01 00 00        	movq	%rcx, 0x198(%rbp)
1421b769c: 48 c7 85 a0 01 00 00 1f 00 00 00    	movq	$0x1f, 0x1a0(%rbp)
1421b76a7: 48 8d 15 92 18 e4 00        	leaq	0xe41892(%rip), %rdx    # 0x142ff8f40
1421b76ae: 48 89 95 b0 01 00 00        	movq	%rdx, 0x1b0(%rbp)
1421b76b5: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
1421b76c0: 48 c7 85 c0 01 00 00 08 00 00 00    	movq	$0x8, 0x1c0(%rbp)
1421b76cb: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
1421b76d6: 0f 57 c9                    	xorps	%xmm1, %xmm1
1421b76d9: 0f 11 8d c8 01 00 00        	movups	%xmm1, 0x1c8(%rbp)
1421b76e0: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
1421b76e7: 48 c7 85 70 01 00 00 1f 00 00 00    	movq	$0x1f, 0x170(%rbp)
1421b76f2: 48 c7 85 78 01 00 00 00 00 00 00    	movq	$0x0, 0x178(%rbp)
1421b76fd: 0f 11 85 80 01 00 00        	movups	%xmm0, 0x180(%rbp)
1421b7704: c7 85 a8 01 00 00 01 00 00 00       	movl	$0x1, 0x1a8(%rbp)
1421b770e: 89 85 ac 01 00 00           	movl	%eax, 0x1ac(%rbp)
1421b7714: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
1421b771b: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1421b7722: e8 09 2c b9 fe              	callq	0x140d4a330 <.text+0xd49330>
1421b7727: 48 8b 7d d0                 	movq	-0x30(%rbp), %rdi
1421b772b: 48 8d 77 18                 	leaq	0x18(%rdi), %rsi
1421b772f: 48 89 f1                    	movq	%rsi, %rcx
1421b7732: e8 d9 a6 ac fe              	callq	0x140c81e10 <.text+0xc80e10>
1421b7737: 31 c9                       	xorl	%ecx, %ecx
1421b7739: b0 01                       	movb	$0x1, %al
1421b773b: f0                          	lock
1421b773c: 0f b0 4f 18                 	cmpxchgb	%cl, 0x18(%rdi)
1421b7740: 0f 85 34 02 00 00           	jne	0x1421b797a <.text+0x21b697a>
1421b7746: 0f 28 b5 00 02 00 00        	movaps	0x200(%rbp), %xmm6
1421b774d: 0f 28 bd 10 02 00 00        	movaps	0x210(%rbp), %xmm7
1421b7754: 44 0f 28 85 20 02 00 00     	movaps	0x220(%rbp), %xmm8
1421b775c: 44 0f 28 8d 30 02 00 00     	movaps	0x230(%rbp), %xmm9
1421b7764: 44 0f 28 95 40 02 00 00     	movaps	0x240(%rbp), %xmm10
1421b776c: 44 0f 28 9d 50 02 00 00     	movaps	0x250(%rbp), %xmm11
1421b7774: 44 0f 28 a5 60 02 00 00     	movaps	0x260(%rbp), %xmm12
1421b777c: 44 0f 28 ad 70 02 00 00     	movaps	0x270(%rbp), %xmm13
1421b7784: 44 0f 28 b5 80 02 00 00     	movaps	0x280(%rbp), %xmm14
1421b778c: 44 0f 28 bd 90 02 00 00     	movaps	0x290(%rbp), %xmm15
1421b7794: 48 81 c4 28 03 00 00        	addq	$0x328, %rsp            # imm = 0x328
1421b779b: 5b                          	popq	%rbx
1421b779c: 5f                          	popq	%rdi
1421b779d: 5e                          	popq	%rsi
1421b779e: 41 5c                       	popq	%r12
1421b77a0: 41 5d                       	popq	%r13
1421b77a2: 41 5e                       	popq	%r14
1421b77a4: 41 5f                       	popq	%r15
1421b77a6: 5d                          	popq	%rbp
1421b77a7: c3                          	retq
1421b77a8: 48 8d 85 b8 00 00 00        	leaq	0xb8(%rbp), %rax
1421b77af: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1421b77b6: 48 8d 05 53 51 bd fe        	leaq	-0x142aead(%rip), %rax  # 0x140d8c910 <.text+0xd8b910>
1421b77bd: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
1421b77c4: 48 8d 05 fd ad f4 00        	leaq	0xf4adfd(%rip), %rax    # 0x1431025c8
1421b77cb: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1421b77d2: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
1421b77dd: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1421b77e8: 48 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %rax
1421b77ef: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1421b77f6: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
1421b7801: 48 8d 15 40 ae f4 00        	leaq	0xf4ae40(%rip), %rdx    # 0x143102648
1421b7808: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1421b780f: e8 7c 1b 72 00              	callq	0x1428d9390 <.text+0x28d8390>
1421b7814: e9 19 01 00 00              	jmp	0x1421b7932 <.text+0x21b6932>
1421b7819: 48 8d 0d b8 ad f4 00        	leaq	0xf4adb8(%rip), %rcx    # 0x1431025d8
1421b7820: 4c 8d 05 21 ae f4 00        	leaq	0xf4ae21(%rip), %r8     # 0x143102648
1421b7827: ba 1d 00 00 00              	movl	$0x1d, %edx
1421b782c: e8 ff 1b 72 00              	callq	0x1428d9430 <.text+0x28d8430>
1421b7831: e9 fc 00 00 00              	jmp	0x1421b7932 <.text+0x21b6932>
1421b7836: 4c 89 ad 10 01 00 00        	movq	%r13, 0x110(%rbp)
1421b783d: 48 8d 05 cc 50 bd fe        	leaq	-0x142af34(%rip), %rax  # 0x140d8c910 <.text+0xd8b910>
1421b7844: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
1421b784b: 48 8d 05 76 ad f4 00        	leaq	0xf4ad76(%rip), %rax    # 0x1431025c8
1421b7852: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1421b7859: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
1421b7864: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1421b786f: 48 8d 85 10 01 00 00        	leaq	0x110(%rbp), %rax
1421b7876: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1421b787d: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
1421b7888: 48 8d 15 b9 ad f4 00        	leaq	0xf4adb9(%rip), %rdx    # 0x143102648
1421b788f: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1421b7896: e8 f5 1a 72 00              	callq	0x1428d9390 <.text+0x28d8390>
1421b789b: e9 92 00 00 00              	jmp	0x1421b7932 <.text+0x21b6932>
1421b78a0: 48 8d 0d 31 ad f4 00        	leaq	0xf4ad31(%rip), %rcx    # 0x1431025d8
1421b78a7: 4c 8d 05 9a ad f4 00        	leaq	0xf4ad9a(%rip), %r8     # 0x143102648
1421b78ae: ba 1d 00 00 00              	movl	$0x1d, %edx
1421b78b3: e8 78 1b 72 00              	callq	0x1428d9430 <.text+0x28d8430>
1421b78b8: eb 78                       	jmp	0x1421b7932 <.text+0x21b6932>
1421b78ba: 4c 89 ad 10 01 00 00        	movq	%r13, 0x110(%rbp)
1421b78c1: 48 8d 05 48 50 bd fe        	leaq	-0x142afb8(%rip), %rax  # 0x140d8c910 <.text+0xd8b910>
1421b78c8: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
1421b78cf: 48 8d 05 f2 ac f4 00        	leaq	0xf4acf2(%rip), %rax    # 0x1431025c8
1421b78d6: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1421b78dd: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
1421b78e8: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1421b78f3: 48 89 95 70 01 00 00        	movq	%rdx, 0x170(%rbp)
1421b78fa: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
1421b7905: 48 8d 15 3c ad f4 00        	leaq	0xf4ad3c(%rip), %rdx    # 0x143102648
1421b790c: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1421b7913: e8 78 1a 72 00              	callq	0x1428d9390 <.text+0x28d8390>
1421b7918: eb 18                       	jmp	0x1421b7932 <.text+0x21b6932>
1421b791a: 48 8d 0d b7 ac f4 00        	leaq	0xf4acb7(%rip), %rcx    # 0x1431025d8
1421b7921: 4c 8d 05 20 ad f4 00        	leaq	0xf4ad20(%rip), %r8     # 0x143102648
1421b7928: ba 1d 00 00 00              	movl	$0x1d, %edx
1421b792d: e8 fe 1a 72 00              	callq	0x1428d9430 <.text+0x28d8430>
1421b7932: 0f 0b                       	ud2
1421b7934: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
1421b793f: 48 83 c3 10                 	addq	$0x10, %rbx
1421b7943: 48 8d 05 de 12 e4 00        	leaq	0xe412de(%rip), %rax    # 0x142ff8c28
1421b794a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1421b794f: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1421b7953: 4c 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %r9
1421b795a: 31 c9                       	xorl	%ecx, %ecx
1421b795c: 49 89 d8                    	movq	%rbx, %r8
1421b795f: e8 dd 31 71 00              	callq	0x1428cab41 <.text+0x28c9b41>
1421b7964: 49 89 ce                    	movq	%rcx, %r14
1421b7967: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
1421b796d: e8 9e 58 71 00              	callq	0x1428cd210 <.text+0x28cc210>
1421b7972: 4c 89 f1                    	movq	%r14, %rcx
1421b7975: e9 3e ef ff ff              	jmp	0x1421b68b8 <.text+0x21b58b8>
1421b797a: 48 89 f1                    	movq	%rsi, %rcx
1421b797d: 31 d2                       	xorl	%edx, %edx
1421b797f: e8 0c 62 71 00              	callq	0x1428cdb90 <.text+0x28ccb90>
1421b7984: e9 bd fd ff ff              	jmp	0x1421b7746 <.text+0x21b6746>
1421b7989: 48 8d 0d 48 15 e4 00        	leaq	0xe41548(%rip), %rcx    # 0x142ff8ed8
1421b7990: e8 7b 19 72 00              	callq	0x1428d9310 <.text+0x28d8310>
1421b7995: 48 8d 0d 54 15 e4 00        	leaq	0xe41554(%rip), %rcx    # 0x142ff8ef0
1421b799c: e8 6f 19 72 00              	callq	0x1428d9310 <.text+0x28d8310>
1421b79a1: 48 8d 0d 20 14 e4 00        	leaq	0xe41420(%rip), %rcx    # 0x142ff8dc8
1421b79a8: e8 63 19 72 00              	callq	0x1428d9310 <.text+0x28d8310>
1421b79ad: 48 8d 0d 2c 14 e4 00        	leaq	0xe4142c(%rip), %rcx    # 0x142ff8de0
1421b79b4: e8 57 19 72 00              	callq	0x1428d9310 <.text+0x28d8310>
1421b79b9: cc                          	int3
1421b79ba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1421b79c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421b79c5: 55                          	pushq	%rbp
1421b79c6: 41 57                       	pushq	%r15
1421b79c8: 41 56                       	pushq	%r14
1421b79ca: 41 55                       	pushq	%r13
1421b79cc: 41 54                       	pushq	%r12
1421b79ce: 56                          	pushq	%rsi
1421b79cf: 57                          	pushq	%rdi
1421b79d0: 53                          	pushq	%rbx
1421b79d1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1421b79d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421b79df: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1421b79e5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1421b79eb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1421b79f1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1421b79f7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1421b79fd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1421b7a03: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1421b7a0c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1421b7a15: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1421b7a1d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1421b7a25: 48 8b 7d d0                 	movq	-0x30(%rbp), %rdi
1421b7a29: 48 8d 77 18                 	leaq	0x18(%rdi), %rsi
1421b7a2d: 48 89 f1                    	movq	%rsi, %rcx
1421b7a30: e8 db a3 ac fe              	callq	0x140c81e10 <.text+0xc80e10>
1421b7a35: 31 c9                       	xorl	%ecx, %ecx
1421b7a37: b0 01                       	movb	$0x1, %al
1421b7a39: f0                          	lock
1421b7a3a: 0f b0 4f 18                 	cmpxchgb	%cl, 0x18(%rdi)
1421b7a3e: 75 5a                       	jne	0x1421b7a9a <.text+0x21b6a9a>
1421b7a40: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1421b7a48: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1421b7a50: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1421b7a59: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1421b7a62: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1421b7a68: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1421b7a6e: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1421b7a74: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1421b7a7a: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1421b7a80: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
1421b7a86: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1421b7a8d: 5b                          	popq	%rbx
1421b7a8e: 5f                          	popq	%rdi
1421b7a8f: 5e                          	popq	%rsi
1421b7a90: 41 5c                       	popq	%r12
1421b7a92: 41 5d                       	popq	%r13
1421b7a94: 41 5e                       	popq	%r14
1421b7a96: 41 5f                       	popq	%r15
1421b7a98: 5d                          	popq	%rbp
1421b7a99: c3                          	retq
1421b7a9a: 48 89 f1                    	movq	%rsi, %rcx
1421b7a9d: 31 d2                       	xorl	%edx, %edx
1421b7a9f: e8 ec 60 71 00              	callq	0x1428cdb90 <.text+0x28ccb90>
1421b7aa4: eb 9a                       	jmp	0x1421b7a40 <.text+0x21b6a40>
1421b7aa6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1421b7ab0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421b7ab5: 55                          	pushq	%rbp
1421b7ab6: 41 57                       	pushq	%r15
1421b7ab8: 41 56                       	pushq	%r14
1421b7aba: 41 55                       	pushq	%r13
1421b7abc: 41 54                       	pushq	%r12
1421b7abe: 56                          	pushq	%rsi
1421b7abf: 57                          	pushq	%rdi
1421b7ac0: 53                          	pushq	%rbx
1421b7ac1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1421b7ac8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421b7acf: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1421b7ad5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1421b7adb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1421b7ae1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1421b7ae7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1421b7aed: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1421b7af3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1421b7afc: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1421b7b05: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1421b7b0d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1421b7b15: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
1421b7b19: 48 85 d2                    	testq	%rdx, %rdx
1421b7b1c: 74 16                       	je	0x1421b7b34 <.text+0x21b6b34>
1421b7b1e: 48 c1 e2 02                 	shlq	$0x2, %rdx
1421b7b22: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1421b7b28: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
1421b7b2f: e8 ec c0 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421b7b34: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1421b7b3c: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1421b7b44: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1421b7b4d: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1421b7b56: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1421b7b5c: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1421b7b62: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1421b7b68: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1421b7b6e: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1421b7b74: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
1421b7b7a: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1421b7b81: 5b                          	popq	%rbx
1421b7b82: 5f                          	popq	%rdi
1421b7b83: 5e                          	popq	%rsi
1421b7b84: 41 5c                       	popq	%r12
1421b7b86: 41 5d                       	popq	%r13
1421b7b88: 41 5e                       	popq	%r14
1421b7b8a: 41 5f                       	popq	%r15
1421b7b8c: 5d                          	popq	%rbp
1421b7b8d: c3                          	retq
