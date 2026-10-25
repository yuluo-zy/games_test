
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140817a40: 41 57                       	pushq	%r15
140817a42: 41 56                       	pushq	%r14
140817a44: 41 55                       	pushq	%r13
140817a46: 41 54                       	pushq	%r12
140817a48: 56                          	pushq	%rsi
140817a49: 57                          	pushq	%rdi
140817a4a: 55                          	pushq	%rbp
140817a4b: 53                          	pushq	%rbx
140817a4c: 48 81 ec 18 02 00 00        	subq	$0x218, %rsp            # imm = 0x218
140817a53: 66 44 0f 7f 8c 24 00 02 00 00       	movdqa	%xmm9, 0x200(%rsp)
140817a5d: 44 0f 29 84 24 f0 01 00 00  	movaps	%xmm8, 0x1f0(%rsp)
140817a66: 66 0f 7f bc 24 e0 01 00 00  	movdqa	%xmm7, 0x1e0(%rsp)
140817a6f: 66 0f 7f b4 24 d0 01 00 00  	movdqa	%xmm6, 0x1d0(%rsp)
140817a78: 4d 89 c4                    	movq	%r8, %r12
140817a7b: 48 89 d6                    	movq	%rdx, %rsi
140817a7e: 48 89 cf                    	movq	%rcx, %rdi
140817a81: 4c 8b bc 24 98 02 00 00     	movq	0x298(%rsp), %r15
140817a89: 41 8b 59 1c                 	movl	0x1c(%r9), %ebx
140817a8d: 4d 8b 71 10                 	movq	0x10(%r9), %r14
140817a91: 41 89 1e                    	movl	%ebx, (%r14)
140817a94: 49 8b 09                    	movq	(%r9), %rcx
140817a97: 48 89 8c 24 98 00 00 00     	movq	%rcx, 0x98(%rsp)
140817a9f: e8 0c 48 20 00              	callq	0x140a1c2b0 <.text+0xa1b2b0>
140817aa4: 49 8b 07                    	movq	(%r15), %rax
140817aa7: 49 8b 4f 08                 	movq	0x8(%r15), %rcx
140817aab: 48 8b 10                    	movq	(%rax), %rdx
140817aae: 45 31 c0                    	xorl	%r8d, %r8d
140817ab1: 49 89 d1                    	movq	%rdx, %r9
140817ab4: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
140817ab8: 4d 0f 42 c8                 	cmovbq	%r8, %r9
140817abc: 48 2b 51 38                 	subq	0x38(%rcx), %rdx
140817ac0: 49 0f 42 d0                 	cmovbq	%r8, %rdx
140817ac4: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
140817ac8: 49 29 d2                    	subq	%rdx, %r10
140817acb: 4d 0f 42 d0                 	cmovbq	%r8, %r10
140817acf: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
140817ad3: 41 0f 96 c3                 	setbe	%r11b
140817ad7: 4c 29 ca                    	subq	%r9, %rdx
140817ada: 49 0f 42 d0                 	cmovbq	%r8, %rdx
140817ade: 40 0f 96 c5                 	setbe	%bpl
140817ae2: 4c 01 d2                    	addq	%r10, %rdx
140817ae5: 44 20 dd                    	andb	%r11b, %bpl
140817ae8: 49 0f 44 d0                 	cmoveq	%r8, %rdx
140817aec: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
140817af0: 48 29 d1                    	subq	%rdx, %rcx
140817af3: 48 89 08                    	movq	%rcx, (%rax)
140817af6: 40 84 ed                    	testb	%bpl, %bpl
140817af9: 74 12                       	je	0x140817b0d <.text+0x816b0d>
140817afb: 48 8b 07                    	movq	(%rdi), %rax
140817afe: 83 38 09                    	cmpl	$0x9, (%rax)
140817b01: 75 0a                       	jne	0x140817b0d <.text+0x816b0d>
140817b03: 80 78 04 01                 	cmpb	$0x1, 0x4(%rax)
140817b07: 0f 84 1b 0e 00 00           	je	0x140818928 <.text+0x817928>
140817b0d: 40 88 6c 24 3a              	movb	%bpl, 0x3a(%rsp)
140817b12: 48 8b 84 24 88 02 00 00     	movq	0x288(%rsp), %rax
140817b1a: 48 8b 8c 24 80 02 00 00     	movq	0x280(%rsp), %rcx
140817b22: 41 89 1e                    	movl	%ebx, (%r14)
140817b25: 4c 89 a4 24 40 01 00 00     	movq	%r12, 0x140(%rsp)
140817b2d: 49 8b 3c 24                 	movq	(%r12), %rdi
140817b31: 48 8b 16                    	movq	(%rsi), %rdx
140817b34: 48 89 94 24 48 01 00 00     	movq	%rdx, 0x148(%rsp)
140817b3c: 8b 51 1c                    	movl	0x1c(%rcx), %edx
140817b3f: 4c 8b 41 10                 	movq	0x10(%rcx), %r8
140817b43: 41 89 10                    	movl	%edx, (%r8)
140817b46: 48 8b 09                    	movq	(%rcx), %rcx
140817b49: 48 89 8c 24 d0 00 00 00     	movq	%rcx, 0xd0(%rsp)
140817b51: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140817b54: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140817b58: 89 0a                       	movl	%ecx, (%rdx)
140817b5a: 48 8b 00                    	movq	(%rax), %rax
140817b5d: 48 89 84 24 e8 00 00 00     	movq	%rax, 0xe8(%rsp)
140817b65: 48 8b 8c 24 98 00 00 00     	movq	0x98(%rsp), %rcx
140817b6d: e8 0e 40 82 ff              	callq	0x14003bb80 <.text+0x3ab80>
140817b72: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140817b77: 48 89 bc 24 b0 00 00 00     	movq	%rdi, 0xb0(%rsp)
140817b7f: 48 89 fa                    	movq	%rdi, %rdx
140817b82: e8 f9 3f 31 00              	callq	0x140b2bb80 <.text+0xb2ab80>
140817b87: 48 8b 54 24 60              	movq	0x60(%rsp), %rdx
140817b8c: 4c 8b 6c 24 70              	movq	0x70(%rsp), %r13
140817b91: 48 8b 7c 24 50              	movq	0x50(%rsp), %rdi
140817b96: 48 8b 74 24 58              	movq	0x58(%rsp), %rsi
140817b9b: 48 8b 44 24 68              	movq	0x68(%rsp), %rax
140817ba0: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140817ba5: 4c 8b b4 24 90 00 00 00     	movq	0x90(%rsp), %r14
140817bad: 44 0f b7 bc 24 88 00 00 00  	movzwl	0x88(%rsp), %r15d
140817bb6: 48 8b 5c 24 78              	movq	0x78(%rsp), %rbx
140817bbb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140817bc0: 48 83 fa 05                 	cmpq	$0x5, %rdx
140817bc4: 0f 84 d0 00 00 00           	je	0x140817c9a <.text+0x816c9a>
140817bca: 4d 85 ed                    	testq	%r13, %r13
140817bcd: 0f 84 a6 00 00 00           	je	0x140817c79 <.text+0x816c79>
140817bd3: 4d 85 f6                    	testq	%r14, %r14
140817bd6: 0f 84 91 00 00 00           	je	0x140817c6d <.text+0x816c6d>
140817bdc: 66 45 85 ff                 	testw	%r15w, %r15w
140817be0: 74 0e                       	je	0x140817bf0 <.text+0x816bf0>
140817be2: 48 89 d5                    	movq	%rdx, %rbp
140817be5: 45 8d 67 ff                 	leal	-0x1(%r15), %r12d
140817be9: 45 21 fc                    	andl	%r15d, %r12d
140817bec: eb 3d                       	jmp	0x140817c2b <.text+0x816c2b>
140817bee: 66 90                       	nop
140817bf0: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
140817bf4: 66 44 0f d7 f8              	pmovmskb	%xmm0, %r15d
140817bf9: 49 81 c5 80 f6 ff ff        	addq	$-0x980, %r13           # imm = 0xF680
140817c00: 48 83 c3 10                 	addq	$0x10, %rbx
140817c04: 41 81 ff ff ff 00 00        	cmpl	$0xffff, %r15d          # imm = 0xFFFF
140817c0b: 74 e3                       	je	0x140817bf0 <.text+0x816bf0>
140817c0d: 48 89 d5                    	movq	%rdx, %rbp
140817c10: 48 89 5c 24 78              	movq	%rbx, 0x78(%rsp)
140817c15: 4c 89 6c 24 70              	movq	%r13, 0x70(%rsp)
140817c1a: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140817c1f: 44 29 f8                    	subl	%r15d, %eax
140817c22: 41 f7 d7                    	notl	%r15d
140817c25: 45 89 fc                    	movl	%r15d, %r12d
140817c28: 41 21 c4                    	andl	%eax, %r12d
140817c2b: 66 44 89 a4 24 88 00 00 00  	movw	%r12w, 0x88(%rsp)
140817c34: 49 ff ce                    	decq	%r14
140817c37: 4c 89 b4 24 90 00 00 00     	movq	%r14, 0x90(%rsp)
140817c3f: f3 41 0f bc c7              	tzcntl	%r15d, %eax
140817c44: 48 f7 d8                    	negq	%rax
140817c47: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
140817c4e: 49 8b 8c 05 68 ff ff ff     	movq	-0x98(%r13,%rax), %rcx
140817c56: e8 c5 48 0a 00              	callq	0x1408bc520 <.text+0x8bb520>
140817c5b: 48 83 f8 03                 	cmpq	$0x3, %rax
140817c5f: 0f 85 a2 00 00 00           	jne	0x140817d07 <.text+0x816d07>
140817c65: 45 89 e7                    	movl	%r12d, %r15d
140817c68: 48 89 ea                    	movq	%rbp, %rdx
140817c6b: eb 03                       	jmp	0x140817c70 <.text+0x816c70>
140817c6d: 45 31 f6                    	xorl	%r14d, %r14d
140817c70: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140817c79: 48 83 fa 04                 	cmpq	$0x4, %rdx
140817c7d: 74 0f                       	je	0x140817c8e <.text+0x816c8e>
140817c7f: 48 c7 44 24 60 03 00 00 00  	movq	$0x3, 0x60(%rsp)
140817c88: 48 83 fa 03                 	cmpq	$0x3, %rdx
140817c8c: 75 67                       	jne	0x140817cf5 <.text+0x816cf5>
140817c8e: 48 c7 44 24 60 05 00 00 00  	movq	$0x5, 0x60(%rsp)
140817c97: 45 31 ed                    	xorl	%r13d, %r13d
140817c9a: 48 83 ff 04                 	cmpq	$0x4, %rdi
140817c9e: 74 75                       	je	0x140817d15 <.text+0x816d15>
140817ca0: 48 c7 44 24 50 03 00 00 00  	movq	$0x3, 0x50(%rsp)
140817ca9: 48 83 ff 03                 	cmpq	$0x3, %rdi
140817cad: 74 66                       	je	0x140817d15 <.text+0x816d15>
140817caf: bd 05 00 00 00              	movl	$0x5, %ebp
140817cb4: 41 bc 03 00 00 00           	movl	$0x3, %r12d
140817cba: 49 89 f0                    	movq	%rsi, %r8
140817cbd: 48 89 fa                    	movq	%rdi, %rdx
140817cc0: 48 8b 8c 24 98 00 00 00     	movq	0x98(%rsp), %rcx
140817cc8: e8 13 51 20 00              	callq	0x140a1cde0 <.text+0xa1bde0>
140817ccd: 49 89 d0                    	movq	%rdx, %r8
140817cd0: 4c 89 e7                    	movq	%r12, %rdi
140817cd3: 48 89 ea                    	movq	%rbp, %rdx
140817cd6: 48 85 c0                    	testq	%rax, %rax
140817cd9: 0f 84 e1 fe ff ff           	je	0x140817bc0 <.text+0x816bc0>
140817cdf: 48 89 c1                    	movq	%rax, %rcx
140817ce2: 4c 89 c2                    	movq	%r8, %rdx
140817ce5: e8 16 fc a3 00              	callq	0x141257900 <.text+0x1256900>
140817cea: 4c 89 e7                    	movq	%r12, %rdi
140817ced: 48 89 ea                    	movq	%rbp, %rdx
140817cf0: e9 cb fe ff ff              	jmp	0x140817bc0 <.text+0x816bc0>
140817cf5: bd 03 00 00 00              	movl	$0x3, %ebp
140817cfa: 49 89 fc                    	movq	%rdi, %r12
140817cfd: 45 31 ed                    	xorl	%r13d, %r13d
140817d00: 4c 8b 44 24 48              	movq	0x48(%rsp), %r8
140817d05: eb b9                       	jmp	0x140817cc0 <.text+0x816cc0>
140817d07: 49 89 d0                    	movq	%rdx, %r8
140817d0a: 45 89 e7                    	movl	%r12d, %r15d
140817d0d: 49 89 fc                    	movq	%rdi, %r12
140817d10: 48 89 c2                    	movq	%rax, %rdx
140817d13: eb ab                       	jmp	0x140817cc0 <.text+0x816cc0>
140817d15: 48 8d 8c 24 88 01 00 00     	leaq	0x188(%rsp), %rcx
140817d1d: 48 8b 94 24 b0 00 00 00     	movq	0xb0(%rsp), %rdx
140817d25: e8 d6 3d 31 00              	callq	0x140b2bb00 <.text+0xb2ab00>
140817d2a: 0f b6 84 24 88 01 00 00     	movzbl	0x188(%rsp), %eax
140817d32: 88 44 24 3b                 	movb	%al, 0x3b(%rsp)
140817d36: 48 8b 84 24 90 01 00 00     	movq	0x190(%rsp), %rax
140817d3e: 48 89 84 24 c8 00 00 00     	movq	%rax, 0xc8(%rsp)
140817d46: 48 8b 84 24 98 01 00 00     	movq	0x198(%rsp), %rax
140817d4e: 48 89 84 24 a8 00 00 00     	movq	%rax, 0xa8(%rsp)
140817d56: 4c 8b bc 24 a0 01 00 00     	movq	0x1a0(%rsp), %r15
140817d5e: 48 8b 84 24 a8 01 00 00     	movq	0x1a8(%rsp), %rax
140817d66: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140817d6b: 48 8b 9c 24 b0 01 00 00     	movq	0x1b0(%rsp), %rbx
140817d73: 44 0f b7 ac 24 c0 01 00 00  	movzwl	0x1c0(%rsp), %r13d
140817d7c: 48 8b 84 24 c8 01 00 00     	movq	0x1c8(%rsp), %rax
140817d84: 48 89 84 24 e0 00 00 00     	movq	%rax, 0xe0(%rsp)
140817d8c: 48 8b 84 24 d0 00 00 00     	movq	0xd0(%rsp), %rax
140817d94: 48 83 c0 30                 	addq	$0x30, %rax
140817d98: 48 89 84 24 58 01 00 00     	movq	%rax, 0x158(%rsp)
140817da0: 66 0f 6f 35 d8 52 1b 02     	movdqa	0x21b52d8(%rip), %xmm6  # 0x1429cd080
140817da8: 66 0f 6f 3d e0 52 1b 02     	movdqa	0x21b52e0(%rip), %xmm7  # 0x1429cd090
140817db0: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
140817db4: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
140817db9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140817dc0: 48 8b b4 24 50 01 00 00     	movq	0x150(%rsp), %rsi
140817dc8: 44 89 e9                    	movl	%r13d, %ecx
140817dcb: 48 8b bc 24 48 01 00 00     	movq	0x148(%rsp), %rdi
140817dd3: 4c 8b a4 24 c8 00 00 00     	movq	0xc8(%rsp), %r12
140817ddb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140817de0: 48 83 bc 24 a8 00 00 00 02  	cmpq	$0x2, 0xa8(%rsp)
140817de9: 0f 84 07 01 00 00           	je	0x140817ef6 <.text+0x816ef6>
140817def: 48 8b 54 24 40              	movq	0x40(%rsp), %rdx
140817df4: 48 85 d2                    	testq	%rdx, %rdx
140817df7: 0f 84 ac 00 00 00           	je	0x140817ea9 <.text+0x816ea9>
140817dfd: 48 83 ac 24 e0 00 00 00 01  	subq	$0x1, 0xe0(%rsp)
140817e06: 0f 82 f4 00 00 00           	jb	0x140817f00 <.text+0x816f00>
140817e0c: 66 85 c9                    	testw	%cx, %cx
140817e0f: 74 0f                       	je	0x140817e20 <.text+0x816e20>
140817e11: 44 8d 69 ff                 	leal	-0x1(%rcx), %r13d
140817e15: 41 21 cd                    	andl	%ecx, %r13d
140817e18: eb 30                       	jmp	0x140817e4a <.text+0x816e4a>
140817e1a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140817e20: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
140817e24: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
140817e28: 48 81 c2 80 f6 ff ff        	addq	$-0x980, %rdx           # imm = 0xF680
140817e2f: 48 83 c3 10                 	addq	$0x10, %rbx
140817e33: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
140817e39: 74 e5                       	je	0x140817e20 <.text+0x816e20>
140817e3b: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140817e40: 29 c8                       	subl	%ecx, %eax
140817e42: f7 d1                       	notl	%ecx
140817e44: 41 89 cd                    	movl	%ecx, %r13d
140817e47: 41 21 c5                    	andl	%eax, %r13d
140817e4a: f3 0f bc c1                 	tzcntl	%ecx, %eax
140817e4e: 48 f7 d8                    	negq	%rax
140817e51: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
140817e58: 49 89 c6                    	movq	%rax, %r14
140817e5b: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
140817e60: 48 8d 0c 02                 	leaq	(%rdx,%rax), %rcx
140817e64: 48 81 c1 68 ff ff ff        	addq	$-0x98, %rcx
140817e6b: e8 30 44 20 00              	callq	0x140a1c2a0 <.text+0xa1b2a0>
140817e70: 48 89 d5                    	movq	%rdx, %rbp
140817e73: 48 85 c0                    	testq	%rax, %rax
140817e76: 75 28                       	jne	0x140817ea0 <.text+0x816ea0>
140817e78: 48 89 f9                    	movq	%rdi, %rcx
140817e7b: 48 89 ea                    	movq	%rbp, %rdx
140817e7e: e8 dd 26 04 00              	callq	0x14085a560 <.text+0x859560>
140817e83: 48 89 ee                    	movq	%rbp, %rsi
140817e86: 44 89 e9                    	movl	%r13d, %ecx
140817e89: 48 89 ac 24 d8 00 00 00     	movq	%rbp, 0xd8(%rsp)
140817e91: 48 85 c0                    	testq	%rax, %rax
140817e94: 0f 84 46 ff ff ff           	je	0x140817de0 <.text+0x816de0>
140817e9a: 31 c0                       	xorl	%eax, %eax
140817e9c: eb 42                       	jmp	0x140817ee0 <.text+0x816ee0>
140817e9e: 66 90                       	nop
140817ea0: 48 83 f8 03                 	cmpq	$0x3, %rax
140817ea4: 75 3a                       	jne	0x140817ee0 <.text+0x816ee0>
140817ea6: 44 89 e9                    	movl	%r13d, %ecx
140817ea9: f6 84 24 a8 00 00 00 01     	testb	$0x1, 0xa8(%rsp)
140817eb1: 74 63                       	je	0x140817f16 <.text+0x816f16>
140817eb3: b8 01 00 00 00              	movl	$0x1, %eax
140817eb8: 41 b8 00 00 00 00           	movl	$0x0, %r8d
140817ebe: ba 00 00 00 00              	movl	$0x0, %edx
140817ec3: 4d 85 ff                    	testq	%r15, %r15
140817ec6: 74 54                       	je	0x140817f1c <.text+0x816f1c>
140817ec8: 48 8b b4 24 d8 00 00 00     	movq	0xd8(%rsp), %rsi
140817ed0: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
140817ed9: 31 d2                       	xorl	%edx, %edx
140817edb: eb 69                       	jmp	0x140817f46 <.text+0x816f46>
140817edd: 0f 1f 00                    	nopl	(%rax)
140817ee0: 4c 03 74 24 40              	addq	0x40(%rsp), %r14
140817ee5: 49 83 c6 a0                 	addq	$-0x60, %r14
140817ee9: 48 89 ef                    	movq	%rbp, %rdi
140817eec: 48 89 ac 24 d8 00 00 00     	movq	%rbp, 0xd8(%rsp)
140817ef4: eb 68                       	jmp	0x140817f5e <.text+0x816f5e>
140817ef6: 4c 89 fa                    	movq	%r15, %rdx
140817ef9: 4c 8b 44 24 40              	movq	0x40(%rsp), %r8
140817efe: eb 1c                       	jmp	0x140817f1c <.text+0x816f1c>
140817f00: 48 c7 84 24 e0 00 00 00 00 00 00 00 	movq	$0x0, 0xe0(%rsp)
140817f0c: f6 84 24 a8 00 00 00 01     	testb	$0x1, 0xa8(%rsp)
140817f14: 75 9d                       	jne	0x140817eb3 <.text+0x816eb3>
140817f16: 45 31 c0                    	xorl	%r8d, %r8d
140817f19: 4c 89 fa                    	movq	%r15, %rdx
140817f1c: 4d 85 e4                    	testq	%r12, %r12
140817f1f: 0f 95 c0                    	setne	%al
140817f22: 22 44 24 3b                 	andb	0x3b(%rsp), %al
140817f26: 3c 01                       	cmpb	$0x1, %al
140817f28: 0f 85 ed 09 00 00           	jne	0x14081891b <.text+0x81791b>
140817f2e: 4c 89 44 24 40              	movq	%r8, 0x40(%rsp)
140817f33: b8 02 00 00 00              	movl	$0x2, %eax
140817f38: 48 89 84 24 a8 00 00 00     	movq	%rax, 0xa8(%rsp)
140817f40: 4d 89 e7                    	movq	%r12, %r15
140817f43: 45 31 e4                    	xorl	%r12d, %r12d
140817f46: 4c 89 a4 24 c8 00 00 00     	movq	%r12, 0xc8(%rsp)
140817f4e: 49 83 c7 30                 	addq	$0x30, %r15
140817f52: 4d 89 fe                    	movq	%r15, %r14
140817f55: 49 89 d7                    	movq	%rdx, %r15
140817f58: 41 89 cd                    	movl	%ecx, %r13d
140817f5b: 48 89 f7                    	movq	%rsi, %rdi
140817f5e: 48 8b 8c 24 98 00 00 00     	movq	0x98(%rsp), %rcx
140817f66: 48 89 c2                    	movq	%rax, %rdx
140817f69: 49 89 f8                    	movq	%rdi, %r8
140817f6c: 48 89 c6                    	movq	%rax, %rsi
140817f6f: e8 bc 4f 20 00              	callq	0x140a1cf30 <.text+0xa1bf30>
140817f74: 48 89 c5                    	movq	%rax, %rbp
140817f77: 48 8d 8c 24 70 01 00 00     	leaq	0x170(%rsp), %rcx
140817f7f: 49 89 f0                    	movq	%rsi, %r8
140817f82: 48 89 bc 24 50 01 00 00     	movq	%rdi, 0x150(%rsp)
140817f8a: 49 89 f9                    	movq	%rdi, %r9
140817f8d: e8 9e 34 20 00              	callq	0x140a1b430 <.text+0xa1a430>
140817f92: 4c 89 f0                    	movq	%r14, %rax
140817f95: 4d 8b 76 18                 	movq	0x18(%r14), %r14
140817f99: 4d 85 f6                    	testq	%r14, %r14
140817f9c: 0f 84 1e fe ff ff           	je	0x140817dc0 <.text+0x816dc0>
140817fa2: 4c 89 bc 24 a0 00 00 00     	movq	%r15, 0xa0(%rsp)
140817faa: 48 89 84 24 c0 00 00 00     	movq	%rax, 0xc0(%rsp)
140817fb2: 4c 8b 38                    	movq	(%rax), %r15
140817fb5: 49 8d 7f 10                 	leaq	0x10(%r15), %rdi
140817fb9: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
140817fbe: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140817fc2: f7 d0                       	notl	%eax
140817fc4: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
140817fc8: 48 89 8c 24 68 01 00 00     	movq	%rcx, 0x168(%rsp)
140817fd0: 48 89 e9                    	movq	%rbp, %rcx
140817fd3: 48 83 c1 30                 	addq	$0x30, %rcx
140817fd7: 48 89 8c 24 60 01 00 00     	movq	%rcx, 0x160(%rsp)
140817fdf: 44 89 6c 24 48              	movl	%r13d, 0x48(%rsp)
140817fe4: 49 89 ea                    	movq	%rbp, %r10
140817fe7: eb 26                       	jmp	0x14081800f <.text+0x81700f>
140817fe9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140817ff0: 4c 8d 05 69 93 40 02        	leaq	0x2409369(%rip), %r8    # 0x142c21360
140817ff7: e8 a4 35 20 00              	callq	0x140a1b5a0 <.text+0xa1a5a0>
140817ffc: 44 8b 6c 24 48              	movl	0x48(%rsp), %r13d
140818001: 89 e8                       	movl	%ebp, %eax
140818003: 49 ff ce                    	decq	%r14
140818006: 4d 89 e2                    	movq	%r12, %r10
140818009: 0f 84 71 02 00 00           	je	0x140818280 <.text+0x817280>
14081800f: 66 85 c0                    	testw	%ax, %ax
140818012: 74 0c                       	je	0x140818020 <.text+0x817020>
140818014: 8d 68 ff                    	leal	-0x1(%rax), %ebp
140818017: 21 c5                       	andl	%eax, %ebp
140818019: eb 2c                       	jmp	0x140818047 <.text+0x817047>
14081801b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140818020: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
140818024: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140818028: 49 81 c7 00 fb ff ff        	addq	$-0x500, %r15           # imm = 0xFB00
14081802f: 48 83 c7 10                 	addq	$0x10, %rdi
140818033: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140818038: 74 e6                       	je	0x140818020 <.text+0x817020>
14081803a: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14081803f: 29 c1                       	subl	%eax, %ecx
140818041: f7 d0                       	notl	%eax
140818043: 89 c5                       	movl	%eax, %ebp
140818045: 21 cd                       	andl	%ecx, %ebp
140818047: f3 0f bc c0                 	tzcntl	%eax, %eax
14081804b: 48 f7 d8                    	negq	%rax
14081804e: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
140818052: 48 c1 e0 04                 	shlq	$0x4, %rax
140818056: 41 8b 74 07 b0              	movl	-0x50(%r15,%rax), %esi
14081805b: 49 8b 4c 07 e0              	movq	-0x20(%r15,%rax), %rcx
140818060: 41 8b 54 07 fc              	movl	-0x4(%r15,%rax), %edx
140818065: 45 8b 44 07 f8              	movl	-0x8(%r15,%rax), %r8d
14081806a: 4c 8d 8c 24 20 01 00 00     	leaq	0x120(%rsp), %r9
140818072: 45 89 41 10                 	movl	%r8d, 0x10(%r9)
140818076: 41 0f 10 44 07 e8           	movups	-0x18(%r15,%rax), %xmm0
14081807c: 41 0f 11 01                 	movups	%xmm0, (%r9)
140818080: 41 0f 10 44 07 b8           	movups	-0x48(%r15,%rax), %xmm0
140818086: 41 0f 10 4c 07 c8           	movups	-0x38(%r15,%rax), %xmm1
14081808c: 0f 29 8c 24 00 01 00 00     	movaps	%xmm1, 0x100(%rsp)
140818094: 49 8b 44 07 d8              	movq	-0x28(%r15,%rax), %rax
140818099: 48 89 84 24 10 01 00 00     	movq	%rax, 0x110(%rsp)
1408180a1: 0f 29 84 24 f0 00 00 00     	movaps	%xmm0, 0xf0(%rsp)
1408180a9: 48 89 8c 24 18 01 00 00     	movq	%rcx, 0x118(%rsp)
1408180b1: 89 94 24 34 01 00 00        	movl	%edx, 0x134(%rsp)
1408180b8: 89 74 24 3c                 	movl	%esi, 0x3c(%rsp)
1408180bc: f3 41 0f 6f 42 20           	movdqu	0x20(%r10), %xmm0
1408180c2: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
1408180c7: 66 0f ef ce                 	pxor	%xmm6, %xmm1
1408180cb: 66 0f 7f 4c 24 50           	movdqa	%xmm1, 0x50(%rsp)
1408180d1: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
1408180d6: 66 0f ef cf                 	pxor	%xmm7, %xmm1
1408180da: 66 0f 7f 4c 24 60           	movdqa	%xmm1, 0x60(%rsp)
1408180e0: 66 0f 7f 44 24 70           	movdqa	%xmm0, 0x70(%rsp)
1408180e6: 48 8d 84 24 80 00 00 00     	leaq	0x80(%rsp), %rax
1408180ee: 44 0f 29 00                 	movaps	%xmm8, (%rax)
1408180f2: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
1408180fa: 89 b4 24 bc 00 00 00        	movl	%esi, 0xbc(%rsp)
140818101: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140818107: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
14081810c: 48 8d 94 24 bc 00 00 00     	leaq	0xbc(%rsp), %rdx
140818114: 4d 89 d5                    	movq	%r10, %r13
140818117: e8 44 e0 8e ff              	callq	0x140106160 <.text+0x105160>
14081811c: 4c 8b 8c 24 80 00 00 00     	movq	0x80(%rsp), %r9
140818124: 49 c1 e1 38                 	shlq	$0x38, %r9
140818128: 4c 0b 8c 24 88 00 00 00     	orq	0x88(%rsp), %r9
140818130: 48 8b 4c 24 60              	movq	0x60(%rsp), %rcx
140818135: 48 8b 44 24 68              	movq	0x68(%rsp), %rax
14081813a: 4c 31 c8                    	xorq	%r9, %rax
14081813d: 48 8b 54 24 50              	movq	0x50(%rsp), %rdx
140818142: 48 01 ca                    	addq	%rcx, %rdx
140818145: 4c 8b 44 24 58              	movq	0x58(%rsp), %r8
14081814a: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081814e: 49 01 c0                    	addq	%rax, %r8
140818151: 48 31 d1                    	xorq	%rdx, %rcx
140818154: 48 c1 c0 10                 	rolq	$0x10, %rax
140818158: 4c 31 c0                    	xorq	%r8, %rax
14081815b: 48 c1 c2 20                 	rolq	$0x20, %rdx
14081815f: 49 01 c8                    	addq	%rcx, %r8
140818162: 48 01 c2                    	addq	%rax, %rdx
140818165: 48 c1 c1 11                 	rolq	$0x11, %rcx
140818169: 4c 31 c1                    	xorq	%r8, %rcx
14081816c: 48 c1 c0 15                 	rolq	$0x15, %rax
140818170: 49 c1 c0 20                 	rolq	$0x20, %r8
140818174: 48 31 d0                    	xorq	%rdx, %rax
140818177: 4c 31 ca                    	xorq	%r9, %rdx
14081817a: 49 81 f0 ff 00 00 00        	xorq	$0xff, %r8
140818181: 48 01 ca                    	addq	%rcx, %rdx
140818184: 49 01 c0                    	addq	%rax, %r8
140818187: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081818b: 48 31 d1                    	xorq	%rdx, %rcx
14081818e: 48 c1 c0 10                 	rolq	$0x10, %rax
140818192: 48 c1 c2 20                 	rolq	$0x20, %rdx
140818196: 4c 31 c0                    	xorq	%r8, %rax
140818199: 49 01 c8                    	addq	%rcx, %r8
14081819c: 48 c1 c1 11                 	rolq	$0x11, %rcx
1408181a0: 48 01 c2                    	addq	%rax, %rdx
1408181a3: 4c 31 c1                    	xorq	%r8, %rcx
1408181a6: 48 c1 c0 15                 	rolq	$0x15, %rax
1408181aa: 48 31 d0                    	xorq	%rdx, %rax
1408181ad: 49 c1 c0 20                 	rolq	$0x20, %r8
1408181b1: 48 01 ca                    	addq	%rcx, %rdx
1408181b4: 49 01 c0                    	addq	%rax, %r8
1408181b7: 48 c1 c1 0d                 	rolq	$0xd, %rcx
1408181bb: 48 31 d1                    	xorq	%rdx, %rcx
1408181be: 48 c1 c0 10                 	rolq	$0x10, %rax
1408181c2: 48 c1 c2 20                 	rolq	$0x20, %rdx
1408181c6: 4c 31 c0                    	xorq	%r8, %rax
1408181c9: 49 01 c8                    	addq	%rcx, %r8
1408181cc: 48 c1 c1 11                 	rolq	$0x11, %rcx
1408181d0: 48 01 c2                    	addq	%rax, %rdx
1408181d3: 4c 31 c1                    	xorq	%r8, %rcx
1408181d6: 48 c1 c0 15                 	rolq	$0x15, %rax
1408181da: 48 31 d0                    	xorq	%rdx, %rax
1408181dd: 49 c1 c0 20                 	rolq	$0x20, %r8
1408181e1: 48 01 ca                    	addq	%rcx, %rdx
1408181e4: 49 01 c0                    	addq	%rax, %r8
1408181e7: 48 c1 c1 0d                 	rolq	$0xd, %rcx
1408181eb: 48 31 d1                    	xorq	%rdx, %rcx
1408181ee: 48 c1 c0 10                 	rolq	$0x10, %rax
1408181f2: 4c 31 c0                    	xorq	%r8, %rax
1408181f5: 49 01 c8                    	addq	%rcx, %r8
1408181f8: 48 c1 c1 11                 	rolq	$0x11, %rcx
1408181fc: 48 c1 c0 15                 	rolq	$0x15, %rax
140818200: 4c 89 c2                    	movq	%r8, %rdx
140818203: 48 c1 c2 20                 	rolq	$0x20, %rdx
140818207: 48 31 ca                    	xorq	%rcx, %rdx
14081820a: 48 31 c2                    	xorq	%rax, %rdx
14081820d: 4c 31 c2                    	xorq	%r8, %rdx
140818210: 4d 89 ec                    	movq	%r13, %r12
140818213: 4c 89 e9                    	movq	%r13, %rcx
140818216: 4c 8d 44 24 3c              	leaq	0x3c(%rsp), %r8
14081821b: e8 b0 f4 35 00              	callq	0x140b776d0 <.text+0xb766d0>
140818220: 48 8b 8c 24 68 01 00 00     	movq	0x168(%rsp), %rcx
140818228: 89 f2                       	movl	%esi, %edx
14081822a: e8 d1 0f ab ff              	callq	0x1402c9200 <.text+0x2c8200>
14081822f: 44 8b 44 24 3c              	movl	0x3c(%rsp), %r8d
140818234: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140818239: 48 8b 94 24 60 01 00 00     	movq	0x160(%rsp), %rdx
140818241: 4c 8d 8c 24 f0 00 00 00     	leaq	0xf0(%rsp), %r9
140818249: e8 52 3d 9e 00              	callq	0x1411fbfa0 <.text+0x11fafa0>
14081824e: 80 bc 24 97 00 00 00 02     	cmpb	$0x2, 0x97(%rsp)
140818256: 8b 54 24 3c                 	movl	0x3c(%rsp), %edx
14081825a: 48 8d 8c 24 70 01 00 00     	leaq	0x170(%rsp), %rcx
140818262: 0f 85 88 fd ff ff           	jne	0x140817ff0 <.text+0x816ff0>
140818268: 4c 8d 05 d9 90 40 02        	leaq	0x24090d9(%rip), %r8    # 0x142c21348
14081826f: e8 6c 32 20 00              	callq	0x140a1b4e0 <.text+0xa1a4e0>
140818274: e9 83 fd ff ff              	jmp	0x140817ffc <.text+0x816ffc>
140818279: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140818280: 48 8b 84 24 c0 00 00 00     	movq	0xc0(%rsp), %rax
140818288: 48 8b 70 18                 	movq	0x18(%rax), %rsi
14081828c: 48 85 f6                    	testq	%rsi, %rsi
14081828f: 4c 8b bc 24 a0 00 00 00     	movq	0xa0(%rsp), %r15
140818297: 0f 84 23 fb ff ff           	je	0x140817dc0 <.text+0x816dc0>
14081829d: 4c 8b 20                    	movq	(%rax), %r12
1408182a0: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
1408182a6: 49 8d 6c 24 10              	leaq	0x10(%r12), %rbp
1408182ab: 66 44 0f d7 f0              	pmovmskb	%xmm0, %r14d
1408182b0: 41 f7 d6                    	notl	%r14d
1408182b3: e9 18 01 00 00              	jmp	0x1408183d0 <.text+0x8173d0>
1408182b8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1408182c0: 49 21 c8                    	andq	%rcx, %r8
1408182c3: f3 42 0f 6f 0c 00           	movdqu	(%rax,%r8), %xmm1
1408182c9: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1408182cd: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1408182d1: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1408182d6: 45 85 d2                    	testl	%r10d, %r10d
1408182d9: 74 35                       	je	0x140818310 <.text+0x817310>
1408182db: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1408182e0: 4d 01 c3                    	addq	%r8, %r11
1408182e3: 49 21 cb                    	andq	%rcx, %r11
1408182e6: 49 f7 db                    	negq	%r11
1408182e9: 4f 8d 1c 9b                 	leaq	(%r11,%r11,4), %r11
1408182ed: 49 c1 e3 04                 	shlq	$0x4, %r11
1408182f1: 42 3b 3c 1a                 	cmpl	(%rdx,%r11), %edi
1408182f5: 74 49                       	je	0x140818340 <.text+0x817340>
1408182f7: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1408182fb: 66 45 21 d3                 	andw	%r10w, %r11w
1408182ff: 45 89 da                    	movl	%r11d, %r10d
140818302: 75 d7                       	jne	0x1408182db <.text+0x8172db>
140818304: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140818310: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
140818315: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
14081831a: 45 85 d2                    	testl	%r10d, %r10d
14081831d: 4c 8b bc 24 a0 00 00 00     	movq	0xa0(%rsp), %r15
140818325: 0f 85 95 02 00 00           	jne	0x1408185c0 <.text+0x8175c0>
14081832b: 4d 01 c8                    	addq	%r9, %r8
14081832e: 49 83 c0 10                 	addq	$0x10, %r8
140818332: 49 83 c1 10                 	addq	$0x10, %r9
140818336: eb 88                       	jmp	0x1408182c0 <.text+0x8172c0>
140818338: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140818340: 4a 8b 4c 18 f8              	movq	-0x8(%rax,%r11), %rcx
140818345: f0                          	lock
140818346: 48 ff 01                    	incq	(%rcx)
140818349: 4c 8b bc 24 a0 00 00 00     	movq	0xa0(%rsp), %r15
140818351: 0f 8e 66 06 00 00           	jle	0x1408189bd <.text+0x8179bd>
140818357: 4c 01 d8                    	addq	%r11, %rax
14081835a: 48 89 8c 24 08 01 00 00     	movq	%rcx, 0x108(%rsp)
140818362: 0f 10 40 e0                 	movups	-0x20(%rax), %xmm0
140818366: 0f 29 84 24 f0 00 00 00     	movaps	%xmm0, 0xf0(%rsp)
14081836e: 48 8b 48 f0                 	movq	-0x10(%rax), %rcx
140818372: 48 89 8c 24 00 01 00 00     	movq	%rcx, 0x100(%rsp)
14081837a: 66 0f 6f 40 c0              	movdqa	-0x40(%rax), %xmm0
14081837f: 66 0f 6f 48 d0              	movdqa	-0x30(%rax), %xmm1
140818384: 66 0f 7f 4c 24 60           	movdqa	%xmm1, 0x60(%rsp)
14081838a: 66 0f 7f 44 24 50           	movdqa	%xmm0, 0x50(%rsp)
140818390: 48 8d 44 24 50              	leaq	0x50(%rsp), %rax
140818395: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14081839a: 48 8d 84 24 f0 00 00 00     	leaq	0xf0(%rsp), %rax
1408183a2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408183a7: 48 8d 8c 24 70 01 00 00     	leaq	0x170(%rsp), %rcx
1408183af: 48 8b 94 24 e8 00 00 00     	movq	0xe8(%rsp), %rdx
1408183b7: 4c 8b 84 24 d0 00 00 00     	movq	0xd0(%rsp), %r8
1408183bf: 41 89 f9                    	movl	%edi, %r9d
1408183c2: e8 49 fd a7 00              	callq	0x141298110 <.text+0x1297110>
1408183c7: e9 f4 01 00 00              	jmp	0x1408185c0 <.text+0x8175c0>
1408183cc: 0f 1f 40 00                 	nopl	(%rax)
1408183d0: 66 45 85 f6                 	testw	%r14w, %r14w
1408183d4: 74 0a                       	je	0x1408183e0 <.text+0x8173e0>
1408183d6: 41 8d 46 ff                 	leal	-0x1(%r14), %eax
1408183da: 44 21 f0                    	andl	%r14d, %eax
1408183dd: eb 2f                       	jmp	0x14081840e <.text+0x81740e>
1408183df: 90                          	nop
1408183e0: 66 0f 6f 45 00              	movdqa	(%rbp), %xmm0
1408183e5: 66 44 0f d7 f0              	pmovmskb	%xmm0, %r14d
1408183ea: 49 81 c4 00 fb ff ff        	addq	$-0x500, %r12           # imm = 0xFB00
1408183f1: 48 83 c5 10                 	addq	$0x10, %rbp
1408183f5: 41 81 fe ff ff 00 00        	cmpl	$0xffff, %r14d          # imm = 0xFFFF
1408183fc: 74 e2                       	je	0x1408183e0 <.text+0x8173e0>
1408183fe: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140818403: 44 29 f1                    	subl	%r14d, %ecx
140818406: 41 f7 d6                    	notl	%r14d
140818409: 44 89 f0                    	movl	%r14d, %eax
14081840c: 21 c8                       	andl	%ecx, %eax
14081840e: 44 89 f1                    	movl	%r14d, %ecx
140818411: f3 0f bc c9                 	tzcntl	%ecx, %ecx
140818415: 41 89 c6                    	movl	%eax, %r14d
140818418: 48 f7 d9                    	negq	%rcx
14081841b: 48 8d 04 89                 	leaq	(%rcx,%rcx,4), %rax
14081841f: 48 c1 e0 04                 	shlq	$0x4, %rax
140818423: 41 8b 7c 04 b0              	movl	-0x50(%r12,%rax), %edi
140818428: 48 8b 8c 24 b0 00 00 00     	movq	0xb0(%rsp), %rcx
140818430: e8 2b d8 7e ff              	callq	0x140005c60 <.text+0x4c60>
140818435: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
14081843a: 0f 84 80 01 00 00           	je	0x1408185c0 <.text+0x8175c0>
140818440: f3 0f 6f 40 20              	movdqu	0x20(%rax), %xmm0
140818445: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14081844a: 66 0f ef ce                 	pxor	%xmm6, %xmm1
14081844e: 66 0f 7f 4c 24 50           	movdqa	%xmm1, 0x50(%rsp)
140818454: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
140818459: 66 0f ef cf                 	pxor	%xmm7, %xmm1
14081845d: 66 0f 7f 4c 24 60           	movdqa	%xmm1, 0x60(%rsp)
140818463: 66 0f 7f 44 24 70           	movdqa	%xmm0, 0x70(%rsp)
140818469: 48 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %rcx
140818471: 44 0f 29 01                 	movaps	%xmm8, (%rcx)
140818475: 48 c7 41 10 00 00 00 00     	movq	$0x0, 0x10(%rcx)
14081847d: 89 bc 24 f0 00 00 00        	movl	%edi, 0xf0(%rsp)
140818484: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14081848a: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
14081848f: 48 8d 94 24 f0 00 00 00     	leaq	0xf0(%rsp), %rdx
140818497: 49 89 c7                    	movq	%rax, %r15
14081849a: e8 c1 dc 8e ff              	callq	0x140106160 <.text+0x105160>
14081849f: 4c 8b 8c 24 80 00 00 00     	movq	0x80(%rsp), %r9
1408184a7: 49 c1 e1 38                 	shlq	$0x38, %r9
1408184ab: 4c 0b 8c 24 88 00 00 00     	orq	0x88(%rsp), %r9
1408184b3: 48 8b 4c 24 60              	movq	0x60(%rsp), %rcx
1408184b8: 48 8b 44 24 68              	movq	0x68(%rsp), %rax
1408184bd: 4c 31 c8                    	xorq	%r9, %rax
1408184c0: 4c 8b 44 24 50              	movq	0x50(%rsp), %r8
1408184c5: 49 01 c8                    	addq	%rcx, %r8
1408184c8: 48 8b 54 24 58              	movq	0x58(%rsp), %rdx
1408184cd: 48 c1 c1 0d                 	rolq	$0xd, %rcx
1408184d1: 48 01 c2                    	addq	%rax, %rdx
1408184d4: 4c 31 c1                    	xorq	%r8, %rcx
1408184d7: 48 c1 c0 10                 	rolq	$0x10, %rax
1408184db: 48 31 d0                    	xorq	%rdx, %rax
1408184de: 49 c1 c0 20                 	rolq	$0x20, %r8
1408184e2: 48 01 ca                    	addq	%rcx, %rdx
1408184e5: 49 01 c0                    	addq	%rax, %r8
1408184e8: 48 c1 c1 11                 	rolq	$0x11, %rcx
1408184ec: 48 31 d1                    	xorq	%rdx, %rcx
1408184ef: 48 c1 c0 15                 	rolq	$0x15, %rax
1408184f3: 48 c1 c2 20                 	rolq	$0x20, %rdx
1408184f7: 4c 31 c0                    	xorq	%r8, %rax
1408184fa: 4d 31 c8                    	xorq	%r9, %r8
1408184fd: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
140818504: 49 01 c8                    	addq	%rcx, %r8
140818507: 48 01 c2                    	addq	%rax, %rdx
14081850a: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081850e: 4c 31 c1                    	xorq	%r8, %rcx
140818511: 48 c1 c0 10                 	rolq	$0x10, %rax
140818515: 49 c1 c0 20                 	rolq	$0x20, %r8
140818519: 48 31 d0                    	xorq	%rdx, %rax
14081851c: 48 01 ca                    	addq	%rcx, %rdx
14081851f: 48 c1 c1 11                 	rolq	$0x11, %rcx
140818523: 49 01 c0                    	addq	%rax, %r8
140818526: 48 31 d1                    	xorq	%rdx, %rcx
140818529: 48 c1 c0 15                 	rolq	$0x15, %rax
14081852d: 4c 31 c0                    	xorq	%r8, %rax
140818530: 48 c1 c2 20                 	rolq	$0x20, %rdx
140818534: 49 01 c8                    	addq	%rcx, %r8
140818537: 48 01 c2                    	addq	%rax, %rdx
14081853a: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081853e: 4c 31 c1                    	xorq	%r8, %rcx
140818541: 48 c1 c0 10                 	rolq	$0x10, %rax
140818545: 49 c1 c0 20                 	rolq	$0x20, %r8
140818549: 48 31 d0                    	xorq	%rdx, %rax
14081854c: 48 01 ca                    	addq	%rcx, %rdx
14081854f: 48 c1 c1 11                 	rolq	$0x11, %rcx
140818553: 49 01 c0                    	addq	%rax, %r8
140818556: 48 31 d1                    	xorq	%rdx, %rcx
140818559: 48 c1 c0 15                 	rolq	$0x15, %rax
14081855d: 4c 31 c0                    	xorq	%r8, %rax
140818560: 48 c1 c2 20                 	rolq	$0x20, %rdx
140818564: 49 01 c8                    	addq	%rcx, %r8
140818567: 48 01 c2                    	addq	%rax, %rdx
14081856a: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081856e: 4c 31 c1                    	xorq	%r8, %rcx
140818571: 48 c1 c0 10                 	rolq	$0x10, %rax
140818575: 48 31 d0                    	xorq	%rdx, %rax
140818578: 48 01 ca                    	addq	%rcx, %rdx
14081857b: 48 c1 c1 11                 	rolq	$0x11, %rcx
14081857f: 48 c1 c0 15                 	rolq	$0x15, %rax
140818583: 49 89 d0                    	movq	%rdx, %r8
140818586: 49 c1 c0 20                 	rolq	$0x20, %r8
14081858a: 49 31 c8                    	xorq	%rcx, %r8
14081858d: 49 31 c0                    	xorq	%rax, %r8
140818590: 49 31 d0                    	xorq	%rdx, %r8
140818593: 4c 89 c2                    	movq	%r8, %rdx
140818596: 48 c1 ea 39                 	shrq	$0x39, %rdx
14081859a: 49 8b 07                    	movq	(%r15), %rax
14081859d: 49 8b 4f 08                 	movq	0x8(%r15), %rcx
1408185a1: 66 0f 6e c2                 	movd	%edx, %xmm0
1408185a5: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408185a9: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408185ae: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408185b3: 48 8d 50 b0                 	leaq	-0x50(%rax), %rdx
1408185b7: 45 31 c9                    	xorl	%r9d, %r9d
1408185ba: e9 01 fd ff ff              	jmp	0x1408182c0 <.text+0x8172c0>
1408185bf: 90                          	nop
1408185c0: 48 ff ce                    	decq	%rsi
1408185c3: 0f 85 07 fe ff ff           	jne	0x1408183d0 <.text+0x8173d0>
1408185c9: 48 8b 84 24 c0 00 00 00     	movq	0xc0(%rsp), %rax
1408185d1: 48 8b 70 18                 	movq	0x18(%rax), %rsi
1408185d5: 48 85 f6                    	testq	%rsi, %rsi
1408185d8: 44 8b 6c 24 48              	movl	0x48(%rsp), %r13d
1408185dd: 0f 84 dd f7 ff ff           	je	0x140817dc0 <.text+0x816dc0>
1408185e3: 4c 8b 38                    	movq	(%rax), %r15
1408185e6: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
1408185eb: 4d 8d 67 10                 	leaq	0x10(%r15), %r12
1408185ef: 66 44 0f d7 f0              	pmovmskb	%xmm0, %r14d
1408185f4: 41 f7 d6                    	notl	%r14d
1408185f7: e9 04 01 00 00              	jmp	0x140818700 <.text+0x817700>
1408185fc: 0f 1f 40 00                 	nopl	(%rax)
140818600: 49 21 c8                    	andq	%rcx, %r8
140818603: f3 42 0f 6f 0c 00           	movdqu	(%rax,%r8), %xmm1
140818609: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14081860d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140818611: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140818616: 45 85 d2                    	testl	%r10d, %r10d
140818619: 74 35                       	je	0x140818650 <.text+0x817650>
14081861b: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140818620: 4d 01 c3                    	addq	%r8, %r11
140818623: 49 21 cb                    	andq	%rcx, %r11
140818626: 49 f7 db                    	negq	%r11
140818629: 4f 8d 1c 9b                 	leaq	(%r11,%r11,4), %r11
14081862d: 49 c1 e3 04                 	shlq	$0x4, %r11
140818631: 42 3b 3c 1a                 	cmpl	(%rdx,%r11), %edi
140818635: 74 39                       	je	0x140818670 <.text+0x817670>
140818637: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14081863b: 66 45 21 d3                 	andw	%r10w, %r11w
14081863f: 45 89 da                    	movl	%r11d, %r10d
140818642: 75 d7                       	jne	0x14081861b <.text+0x81761b>
140818644: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140818650: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
140818655: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
14081865a: 45 85 d2                    	testl	%r10d, %r10d
14081865d: 0f 85 9d 02 00 00           	jne	0x140818900 <.text+0x817900>
140818663: 4d 01 c8                    	addq	%r9, %r8
140818666: 49 83 c0 10                 	addq	$0x10, %r8
14081866a: 49 83 c1 10                 	addq	$0x10, %r9
14081866e: eb 90                       	jmp	0x140818600 <.text+0x817600>
140818670: 4a 8b 4c 18 f8              	movq	-0x8(%rax,%r11), %rcx
140818675: f0                          	lock
140818676: 48 ff 01                    	incq	(%rcx)
140818679: 0f 8e 3e 03 00 00           	jle	0x1408189bd <.text+0x8179bd>
14081867f: 4c 01 d8                    	addq	%r11, %rax
140818682: 48 89 8c 24 08 01 00 00     	movq	%rcx, 0x108(%rsp)
14081868a: 0f 10 40 e0                 	movups	-0x20(%rax), %xmm0
14081868e: 0f 29 84 24 f0 00 00 00     	movaps	%xmm0, 0xf0(%rsp)
140818696: 48 8b 48 f0                 	movq	-0x10(%rax), %rcx
14081869a: 48 89 8c 24 00 01 00 00     	movq	%rcx, 0x100(%rsp)
1408186a2: 66 0f 6f 40 c0              	movdqa	-0x40(%rax), %xmm0
1408186a7: 66 0f 6f 48 d0              	movdqa	-0x30(%rax), %xmm1
1408186ac: 66 0f 7f 4c 24 60           	movdqa	%xmm1, 0x60(%rsp)
1408186b2: 66 0f 7f 44 24 50           	movdqa	%xmm0, 0x50(%rsp)
1408186b8: 48 8d 44 24 50              	leaq	0x50(%rsp), %rax
1408186bd: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1408186c2: 48 8d 84 24 f0 00 00 00     	leaq	0xf0(%rsp), %rax
1408186ca: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408186cf: 48 8d 8c 24 70 01 00 00     	leaq	0x170(%rsp), %rcx
1408186d7: 48 8b 94 24 e8 00 00 00     	movq	0xe8(%rsp), %rdx
1408186df: 4c 8b 84 24 58 01 00 00     	movq	0x158(%rsp), %r8
1408186e7: 41 89 f9                    	movl	%edi, %r9d
1408186ea: e8 21 fa a7 00              	callq	0x141298110 <.text+0x1297110>
1408186ef: e9 0c 02 00 00              	jmp	0x140818900 <.text+0x817900>
1408186f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140818700: 66 45 85 f6                 	testw	%r14w, %r14w
140818704: 74 0a                       	je	0x140818710 <.text+0x817710>
140818706: 41 8d 46 ff                 	leal	-0x1(%r14), %eax
14081870a: 44 21 f0                    	andl	%r14d, %eax
14081870d: eb 30                       	jmp	0x14081873f <.text+0x81773f>
14081870f: 90                          	nop
140818710: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
140818716: 66 44 0f d7 f0              	pmovmskb	%xmm0, %r14d
14081871b: 49 81 c7 00 fb ff ff        	addq	$-0x500, %r15           # imm = 0xFB00
140818722: 49 83 c4 10                 	addq	$0x10, %r12
140818726: 41 81 fe ff ff 00 00        	cmpl	$0xffff, %r14d          # imm = 0xFFFF
14081872d: 74 e1                       	je	0x140818710 <.text+0x817710>
14081872f: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140818734: 44 29 f1                    	subl	%r14d, %ecx
140818737: 41 f7 d6                    	notl	%r14d
14081873a: 44 89 f0                    	movl	%r14d, %eax
14081873d: 21 c8                       	andl	%ecx, %eax
14081873f: 44 89 f1                    	movl	%r14d, %ecx
140818742: f3 0f bc c9                 	tzcntl	%ecx, %ecx
140818746: 41 89 c6                    	movl	%eax, %r14d
140818749: 48 f7 d9                    	negq	%rcx
14081874c: 48 8d 04 89                 	leaq	(%rcx,%rcx,4), %rax
140818750: 48 c1 e0 04                 	shlq	$0x4, %rax
140818754: 41 8b 7c 07 b0              	movl	-0x50(%r15,%rax), %edi
140818759: 48 8b 8c 24 b0 00 00 00     	movq	0xb0(%rsp), %rcx
140818761: e8 5a 7c ba ff              	callq	0x1403c03c0 <.text+0x3bf3c0>
140818766: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
14081876b: 0f 84 8f 01 00 00           	je	0x140818900 <.text+0x817900>
140818771: f3 0f 6f 40 20              	movdqu	0x20(%rax), %xmm0
140818776: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14081877b: 66 0f ef ce                 	pxor	%xmm6, %xmm1
14081877f: 66 0f 7f 4c 24 50           	movdqa	%xmm1, 0x50(%rsp)
140818785: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14081878a: 66 0f ef cf                 	pxor	%xmm7, %xmm1
14081878e: 66 0f 7f 4c 24 60           	movdqa	%xmm1, 0x60(%rsp)
140818794: 66 0f 7f 44 24 70           	movdqa	%xmm0, 0x70(%rsp)
14081879a: 48 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %rcx
1408187a2: 44 0f 29 01                 	movaps	%xmm8, (%rcx)
1408187a6: 48 c7 41 10 00 00 00 00     	movq	$0x0, 0x10(%rcx)
1408187ae: 89 bc 24 f0 00 00 00        	movl	%edi, 0xf0(%rsp)
1408187b5: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1408187bb: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
1408187c0: 48 8d 94 24 f0 00 00 00     	leaq	0xf0(%rsp), %rdx
1408187c8: 48 89 c5                    	movq	%rax, %rbp
1408187cb: e8 90 d9 8e ff              	callq	0x140106160 <.text+0x105160>
1408187d0: 4c 8b 8c 24 80 00 00 00     	movq	0x80(%rsp), %r9
1408187d8: 49 c1 e1 38                 	shlq	$0x38, %r9
1408187dc: 4c 0b 8c 24 88 00 00 00     	orq	0x88(%rsp), %r9
1408187e4: 48 8b 4c 24 60              	movq	0x60(%rsp), %rcx
1408187e9: 48 8b 44 24 68              	movq	0x68(%rsp), %rax
1408187ee: 4c 31 c8                    	xorq	%r9, %rax
1408187f1: 4c 8b 44 24 50              	movq	0x50(%rsp), %r8
1408187f6: 49 01 c8                    	addq	%rcx, %r8
1408187f9: 48 8b 54 24 58              	movq	0x58(%rsp), %rdx
1408187fe: 48 c1 c1 0d                 	rolq	$0xd, %rcx
140818802: 48 01 c2                    	addq	%rax, %rdx
140818805: 4c 31 c1                    	xorq	%r8, %rcx
140818808: 48 c1 c0 10                 	rolq	$0x10, %rax
14081880c: 48 31 d0                    	xorq	%rdx, %rax
14081880f: 49 c1 c0 20                 	rolq	$0x20, %r8
140818813: 48 01 ca                    	addq	%rcx, %rdx
140818816: 49 01 c0                    	addq	%rax, %r8
140818819: 48 c1 c1 11                 	rolq	$0x11, %rcx
14081881d: 48 31 d1                    	xorq	%rdx, %rcx
140818820: 48 c1 c0 15                 	rolq	$0x15, %rax
140818824: 48 c1 c2 20                 	rolq	$0x20, %rdx
140818828: 4c 31 c0                    	xorq	%r8, %rax
14081882b: 4d 31 c8                    	xorq	%r9, %r8
14081882e: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
140818835: 49 01 c8                    	addq	%rcx, %r8
140818838: 48 01 c2                    	addq	%rax, %rdx
14081883b: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081883f: 4c 31 c1                    	xorq	%r8, %rcx
140818842: 48 c1 c0 10                 	rolq	$0x10, %rax
140818846: 49 c1 c0 20                 	rolq	$0x20, %r8
14081884a: 48 31 d0                    	xorq	%rdx, %rax
14081884d: 48 01 ca                    	addq	%rcx, %rdx
140818850: 48 c1 c1 11                 	rolq	$0x11, %rcx
140818854: 49 01 c0                    	addq	%rax, %r8
140818857: 48 31 d1                    	xorq	%rdx, %rcx
14081885a: 48 c1 c0 15                 	rolq	$0x15, %rax
14081885e: 4c 31 c0                    	xorq	%r8, %rax
140818861: 48 c1 c2 20                 	rolq	$0x20, %rdx
140818865: 49 01 c8                    	addq	%rcx, %r8
140818868: 48 01 c2                    	addq	%rax, %rdx
14081886b: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081886f: 4c 31 c1                    	xorq	%r8, %rcx
140818872: 48 c1 c0 10                 	rolq	$0x10, %rax
140818876: 49 c1 c0 20                 	rolq	$0x20, %r8
14081887a: 48 31 d0                    	xorq	%rdx, %rax
14081887d: 48 01 ca                    	addq	%rcx, %rdx
140818880: 48 c1 c1 11                 	rolq	$0x11, %rcx
140818884: 49 01 c0                    	addq	%rax, %r8
140818887: 48 31 d1                    	xorq	%rdx, %rcx
14081888a: 48 c1 c0 15                 	rolq	$0x15, %rax
14081888e: 4c 31 c0                    	xorq	%r8, %rax
140818891: 48 c1 c2 20                 	rolq	$0x20, %rdx
140818895: 49 01 c8                    	addq	%rcx, %r8
140818898: 48 01 c2                    	addq	%rax, %rdx
14081889b: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081889f: 4c 31 c1                    	xorq	%r8, %rcx
1408188a2: 48 c1 c0 10                 	rolq	$0x10, %rax
1408188a6: 48 31 d0                    	xorq	%rdx, %rax
1408188a9: 48 01 ca                    	addq	%rcx, %rdx
1408188ac: 48 c1 c1 11                 	rolq	$0x11, %rcx
1408188b0: 48 c1 c0 15                 	rolq	$0x15, %rax
1408188b4: 49 89 d0                    	movq	%rdx, %r8
1408188b7: 49 c1 c0 20                 	rolq	$0x20, %r8
1408188bb: 49 31 c8                    	xorq	%rcx, %r8
1408188be: 49 31 c0                    	xorq	%rax, %r8
1408188c1: 49 31 d0                    	xorq	%rdx, %r8
1408188c4: 4c 89 c2                    	movq	%r8, %rdx
1408188c7: 48 c1 ea 39                 	shrq	$0x39, %rdx
1408188cb: 48 8b 45 00                 	movq	(%rbp), %rax
1408188cf: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
1408188d3: 66 0f 6e c2                 	movd	%edx, %xmm0
1408188d7: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408188db: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408188e0: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408188e5: 48 8d 50 b0                 	leaq	-0x50(%rax), %rdx
1408188e9: 45 31 c9                    	xorl	%r9d, %r9d
1408188ec: e9 0f fd ff ff              	jmp	0x140818600 <.text+0x817600>
1408188f1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140818900: 48 ff ce                    	decq	%rsi
140818903: 0f 85 f7 fd ff ff           	jne	0x140818700 <.text+0x817700>
140818909: 4c 8b bc 24 a0 00 00 00     	movq	0xa0(%rsp), %r15
140818911: 44 8b 6c 24 48              	movl	0x48(%rsp), %r13d
140818916: e9 a5 f4 ff ff              	jmp	0x140817dc0 <.text+0x816dc0>
14081891b: 4c 8b a4 24 40 01 00 00     	movq	0x140(%rsp), %r12
140818923: 0f b6 6c 24 3a              	movzbl	0x3a(%rsp), %ebp
140818928: 48 8b 84 24 90 02 00 00     	movq	0x290(%rsp), %rax
140818930: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140818933: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140818937: 89 0a                       	movl	%ecx, (%rdx)
140818939: 48 8b 08                    	movq	(%rax), %rcx
14081893c: e8 cf 68 0e 00              	callq	0x1408ff210 <.text+0x8fe210>
140818941: 40 84 ed                    	testb	%bpl, %bpl
140818944: 75 41                       	jne	0x140818987 <.text+0x817987>
140818946: 41 8b 44 24 1c              	movl	0x1c(%r12), %eax
14081894b: 49 8b 4c 24 10              	movq	0x10(%r12), %rcx
140818950: 89 01                       	movl	%eax, (%rcx)
140818952: 49 8b 14 24                 	movq	(%r12), %rdx
140818956: 48 8d 74 24 50              	leaq	0x50(%rsp), %rsi
14081895b: 48 89 f1                    	movq	%rsi, %rcx
14081895e: 4c 8b 84 24 98 00 00 00     	movq	0x98(%rsp), %r8
140818966: 41 b1 01                    	movb	$0x1, %r9b
140818969: e8 c2 41 31 00              	callq	0x140b2cb30 <.text+0xb2bb30>
14081896e: 48 8d 54 24 70              	leaq	0x70(%rsp), %rdx
140818973: 41 b8 70 00 00 00           	movl	$0x70, %r8d
140818979: 41 b9 10 00 00 00           	movl	$0x10, %r9d
14081897f: 48 89 f1                    	movq	%rsi, %rcx
140818982: e8 59 ba 35 00              	callq	0x140b743e0 <.text+0xb733e0>
140818987: 0f 28 b4 24 d0 01 00 00     	movaps	0x1d0(%rsp), %xmm6
14081898f: 0f 28 bc 24 e0 01 00 00     	movaps	0x1e0(%rsp), %xmm7
140818997: 44 0f 28 84 24 f0 01 00 00  	movaps	0x1f0(%rsp), %xmm8
1408189a0: 44 0f 28 8c 24 00 02 00 00  	movaps	0x200(%rsp), %xmm9
1408189a9: 48 81 c4 18 02 00 00        	addq	$0x218, %rsp            # imm = 0x218
1408189b0: 5b                          	popq	%rbx
1408189b1: 5d                          	popq	%rbp
1408189b2: 5f                          	popq	%rdi
1408189b3: 5e                          	popq	%rsi
1408189b4: 41 5c                       	popq	%r12
1408189b6: 41 5d                       	popq	%r13
1408189b8: 41 5e                       	popq	%r14
1408189ba: 41 5f                       	popq	%r15
1408189bc: c3                          	retq
1408189bd: 0f 0b                       	ud2
