
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14031af10: 55                          	pushq	%rbp
14031af11: 41 57                       	pushq	%r15
14031af13: 41 56                       	pushq	%r14
14031af15: 41 55                       	pushq	%r13
14031af17: 41 54                       	pushq	%r12
14031af19: 56                          	pushq	%rsi
14031af1a: 57                          	pushq	%rdi
14031af1b: 53                          	pushq	%rbx
14031af1c: 48 81 ec c8 01 00 00        	subq	$0x1c8, %rsp            # imm = 0x1C8
14031af23: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14031af2b: 48 c7 85 40 01 00 00 fe ff ff ff    	movq	$-0x2, 0x140(%rbp)
14031af36: 4d 89 ce                    	movq	%r9, %r14
14031af39: 4c 89 45 08                 	movq	%r8, 0x8(%rbp)
14031af3d: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
14031af44: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
14031af4b: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
14031af52: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
14031af59: 48 8b 51 28                 	movq	0x28(%rcx), %rdx
14031af5d: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
14031af61: 48 8b 59 30                 	movq	0x30(%rcx), %rbx
14031af65: 48 8b 71 50                 	movq	0x50(%rcx), %rsi
14031af69: 4c 8b 61 58                 	movq	0x58(%rcx), %r12
14031af6d: 48 c7 85 10 01 00 00 00 00 00 00    	movq	$0x0, 0x110(%rbp)
14031af78: 48 c7 85 18 01 00 00 08 00 00 00    	movq	$0x8, 0x118(%rbp)
14031af83: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
14031af8e: 48 8b 38                    	movq	(%rax), %rdi
14031af91: 48 85 ff                    	testq	%rdi, %rdi
14031af94: 0f 84 ae 00 00 00           	je	0x14031b048 <.text+0x31a048>
14031af9a: 4c 8b 2f                    	movq	(%rdi), %r13
14031af9d: 49 8d 4d 10                 	leaq	0x10(%r13), %rcx
14031afa1: ba 08 00 00 00              	movl	$0x8, %edx
14031afa6: 31 c0                       	xorl	%eax, %eax
14031afa8: f0                          	lock
14031afa9: 49 0f b1 55 10              	cmpxchgq	%rdx, 0x10(%r13)
14031afae: 48 89 8d 30 01 00 00        	movq	%rcx, 0x130(%rbp)
14031afb5: 0f 85 91 15 00 00           	jne	0x14031c54c <.text+0x31b54c>
14031afbb: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031afc2: e8 a9 6c 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031afc7: 4c 8b bd 30 01 00 00        	movq	0x130(%rbp), %r15
14031afce: 49 83 cf 01                 	orq	$0x1, %r15
14031afd2: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031afd9: 4c 89 f9                    	movq	%r15, %rcx
14031afdc: e8 8f 6c 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031afe1: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031afe8: 49 8b 45 28                 	movq	0x28(%r13), %rax
14031afec: 48 85 c0                    	testq	%rax, %rax
14031afef: 74 23                       	je	0x14031b014 <.text+0x31a014>
14031aff1: 4d 8b 55 20                 	movq	0x20(%r13), %r10
14031aff5: 4c 8d 04 c5 00 00 00 00     	leaq	(,%rax,8), %r8
14031affd: 31 d2                       	xorl	%edx, %edx
14031afff: 90                          	nop
14031b000: 49 83 3c d2 00              	cmpq	$0x0, (%r10,%rdx,8)
14031b005: 0f 84 5a 01 00 00           	je	0x14031b165 <.text+0x31a165>
14031b00b: 48 ff c2                    	incq	%rdx
14031b00e: 49 83 c0 f8                 	addq	$-0x8, %r8
14031b012: 75 ec                       	jne	0x14031b000 <.text+0x31a000>
14031b014: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b01b: e8 f0 6d 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031b020: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b027: 4c 89 f9                    	movq	%r15, %rcx
14031b02a: e8 e1 6d 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031b02f: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031b036: 31 d2                       	xorl	%edx, %edx
14031b038: b8 08 00 00 00              	movl	$0x8, %eax
14031b03d: f0                          	lock
14031b03e: 48 0f b1 11                 	cmpxchgq	%rdx, (%rcx)
14031b042: 0f 85 52 15 00 00           	jne	0x14031c59a <.text+0x31b59a>
14031b048: 48 8b 06                    	movq	(%rsi), %rax
14031b04b: 45 31 ed                    	xorl	%r13d, %r13d
14031b04e: 49 89 c2                    	movq	%rax, %r10
14031b051: 4d 2b 54 24 18              	subq	0x18(%r12), %r10
14031b056: 4d 0f 42 d5                 	cmovbq	%r13, %r10
14031b05a: 49 2b 44 24 38              	subq	0x38(%r12), %rax
14031b05f: 49 0f 42 c5                 	cmovbq	%r13, %rax
14031b063: 4d 89 d0                    	movq	%r10, %r8
14031b066: 49 c1 e0 04                 	shlq	$0x4, %r8
14031b06a: 4d 03 44 24 08              	addq	0x8(%r12), %r8
14031b06f: 4d 8b 5c 24 10              	movq	0x10(%r12), %r11
14031b074: 48 89 c1                    	movq	%rax, %rcx
14031b077: 48 c1 e1 04                 	shlq	$0x4, %rcx
14031b07b: 49 03 4c 24 28              	addq	0x28(%r12), %rcx
14031b080: 4c 89 da                    	movq	%r11, %rdx
14031b083: 4c 29 d2                    	subq	%r10, %rdx
14031b086: 49 0f 42 d5                 	cmovbq	%r13, %rdx
14031b08a: 4d 8b 7c 24 30              	movq	0x30(%r12), %r15
14031b08f: 48 89 bd 30 01 00 00        	movq	%rdi, 0x130(%rbp)
14031b096: 48 89 df                    	movq	%rbx, %rdi
14031b099: 4c 89 f3                    	movq	%r14, %rbx
14031b09c: 41 be 08 00 00 00           	movl	$0x8, %r14d
14031b0a2: 4d 0f 42 c6                 	cmovbq	%r14, %r8
14031b0a6: 4d 0f 42 d3                 	cmovbq	%r11, %r10
14031b0aa: 4d 89 f9                    	movq	%r15, %r9
14031b0ad: 49 29 c1                    	subq	%rax, %r9
14031b0b0: 4d 0f 42 cd                 	cmovbq	%r13, %r9
14031b0b4: 49 0f 42 ce                 	cmovbq	%r14, %rcx
14031b0b8: 4d 8b 74 24 40              	movq	0x40(%r12), %r14
14031b0bd: 49 0f 42 c7                 	cmovbq	%r15, %rax
14031b0c1: 4c 01 f0                    	addq	%r14, %rax
14031b0c4: 4d 29 ce                    	subq	%r9, %r14
14031b0c7: 49 29 d6                    	subq	%rdx, %r14
14031b0ca: 4c 89 36                    	movq	%r14, (%rsi)
14031b0cd: 49 89 de                    	movq	%rbx, %r14
14031b0d0: 48 89 fb                    	movq	%rdi, %rbx
14031b0d3: 48 8b bd 30 01 00 00        	movq	0x130(%rbp), %rdi
14031b0da: 48 c1 e2 04                 	shlq	$0x4, %rdx
14031b0de: 4c 01 c2                    	addq	%r8, %rdx
14031b0e1: 49 c1 e1 04                 	shlq	$0x4, %r9
14031b0e5: 49 01 c9                    	addq	%rcx, %r9
14031b0e8: 4c 01 d0                    	addq	%r10, %rax
14031b0eb: 4c 29 f8                    	subq	%r15, %rax
14031b0ee: 4c 29 d8                    	subq	%r11, %rax
14031b0f1: 48 ff c0                    	incq	%rax
14031b0f4: eb 1e                       	jmp	0x14031b114 <.text+0x31a114>
14031b0f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14031b100: 4d 8d 50 10                 	leaq	0x10(%r8), %r10
14031b104: 48 89 06                    	movq	%rax, (%rsi)
14031b107: 48 ff c0                    	incq	%rax
14031b10a: 41 80 78 08 04              	cmpb	$0x4, 0x8(%r8)
14031b10f: 4d 89 d0                    	movq	%r10, %r8
14031b112: 74 20                       	je	0x14031b134 <.text+0x31a134>
14031b114: 4d 85 c0                    	testq	%r8, %r8
14031b117: 74 05                       	je	0x14031b11e <.text+0x31a11e>
14031b119: 49 39 d0                    	cmpq	%rdx, %r8
14031b11c: 75 e2                       	jne	0x14031b100 <.text+0x31a100>
14031b11e: 48 85 c9                    	testq	%rcx, %rcx
14031b121: 74 19                       	je	0x14031b13c <.text+0x31a13c>
14031b123: 4c 39 c9                    	cmpq	%r9, %rcx
14031b126: 74 14                       	je	0x14031b13c <.text+0x31a13c>
14031b128: 49 89 c8                    	movq	%rcx, %r8
14031b12b: 48 83 c1 10                 	addq	$0x10, %rcx
14031b12f: 45 31 d2                    	xorl	%r10d, %r10d
14031b132: eb d0                       	jmp	0x14031b104 <.text+0x31a104>
14031b134: 41 b7 01                    	movb	$0x1, %r15b
14031b137: e9 93 00 00 00              	jmp	0x14031b1cf <.text+0x31a1cf>
14031b13c: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14031b143: 48 8b 00                    	movq	(%rax), %rax
14031b146: 8b 88 e8 00 00 00           	movl	0xe8(%rax), %ecx
14031b14c: f3 0f 10 88 ec 00 00 00     	movss	0xec(%rax), %xmm1
14031b154: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b15b: e8 30 49 62 00              	callq	0x14093fa90 <.text+0x93ea90>
14031b160: 41 89 c7                    	movl	%eax, %r15d
14031b163: eb 6a                       	jmp	0x14031b1cf <.text+0x31a1cf>
14031b165: 4d 8b 04 d2                 	movq	(%r10,%rdx,8), %r8
14031b169: 4d 8b 4c c2 f8              	movq	-0x8(%r10,%rax,8), %r9
14031b16e: 48 ff c8                    	decq	%rax
14031b171: 4d 89 0c d2                 	movq	%r9, (%r10,%rdx,8)
14031b175: 49 89 45 28                 	movq	%rax, 0x28(%r13)
14031b179: 4c 89 45 68                 	movq	%r8, 0x68(%rbp)
14031b17d: 4d 85 c0                    	testq	%r8, %r8
14031b180: 74 0f                       	je	0x14031b191 <.text+0x31a191>
14031b182: f0                          	lock
14031b183: 49 ff 08                    	decq	(%r8)
14031b186: 75 09                       	jne	0x14031b191 <.text+0x31a191>
14031b188: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031b18c: e8 0f 0f 84 00              	callq	0x140b5c0a0 <.text+0xb5b0a0>
14031b191: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b198: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031b19f: e8 6c 6c 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031b1a4: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b1ab: 4c 89 f9                    	movq	%r15, %rcx
14031b1ae: e8 5d 6c 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031b1b3: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031b1ba: 31 d2                       	xorl	%edx, %edx
14031b1bc: b8 08 00 00 00              	movl	$0x8, %eax
14031b1c1: f0                          	lock
14031b1c2: 48 0f b1 11                 	cmpxchgq	%rdx, (%rcx)
14031b1c6: 41 b7 01                    	movb	$0x1, %r15b
14031b1c9: 0f 85 de 13 00 00           	jne	0x14031c5ad <.text+0x31b5ad>
14031b1cf: 48 85 ff                    	testq	%rdi, %rdi
14031b1d2: 0f 84 8e 00 00 00           	je	0x14031b266 <.text+0x31a266>
14031b1d8: 4c 8b 27                    	movq	(%rdi), %r12
14031b1db: 49 8d 74 24 10              	leaq	0x10(%r12), %rsi
14031b1e0: b9 08 00 00 00              	movl	$0x8, %ecx
14031b1e5: 31 c0                       	xorl	%eax, %eax
14031b1e7: f0                          	lock
14031b1e8: 49 0f b1 4c 24 10           	cmpxchgq	%rcx, 0x10(%r12)
14031b1ee: 0f 85 76 13 00 00           	jne	0x14031c56a <.text+0x31b56a>
14031b1f4: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b1fb: 48 89 f1                    	movq	%rsi, %rcx
14031b1fe: e8 6d 6a 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031b203: 49 89 f5                    	movq	%rsi, %r13
14031b206: 49 83 cd 01                 	orq	$0x1, %r13
14031b20a: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b211: 4c 89 e9                    	movq	%r13, %rcx
14031b214: e8 57 6a 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031b219: 48 89 b5 30 01 00 00        	movq	%rsi, 0x130(%rbp)
14031b220: 49 83 c4 18                 	addq	$0x18, %r12
14031b224: 4c 89 e1                    	movq	%r12, %rcx
14031b227: 4c 89 f2                    	movq	%r14, %rdx
14031b22a: e8 c1 a2 f9 ff              	callq	0x1402b54f0 <.text+0x2b44f0>
14031b22f: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b236: 48 8b b5 30 01 00 00        	movq	0x130(%rbp), %rsi
14031b23d: 48 89 f1                    	movq	%rsi, %rcx
14031b240: e8 cb 6b 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031b245: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b24c: 4c 89 e9                    	movq	%r13, %rcx
14031b24f: e8 bc 6b 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031b254: 31 c9                       	xorl	%ecx, %ecx
14031b256: b8 08 00 00 00              	movl	$0x8, %eax
14031b25b: f0                          	lock
14031b25c: 48 0f b1 0e                 	cmpxchgq	%rcx, (%rsi)
14031b260: 0f 85 1e 13 00 00           	jne	0x14031c584 <.text+0x31b584>
14031b266: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
14031b26d: 4c 8b a5 d0 01 00 00        	movq	0x1d0(%rbp), %r12
14031b274: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
14031b27b: 48 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %rdx
14031b282: 4c 8b 2a                    	movq	(%rdx), %r13
14031b285: 80 38 00                    	cmpb	$0x0, (%rax)
14031b288: 74 28                       	je	0x14031b2b2 <.text+0x31a2b2>
14031b28a: 0f b6 50 01                 	movzbl	0x1(%rax), %edx
14031b28e: 80 fa 10                    	cmpb	$0x10, %dl
14031b291: 73 47                       	jae	0x14031b2da <.text+0x31a2da>
14031b293: 41 8b 4c 24 1c              	movl	0x1c(%r12), %ecx
14031b298: 4d 8b 44 24 10              	movq	0x10(%r12), %r8
14031b29d: 41 89 08                    	movl	%ecx, (%r8)
14031b2a0: 49 8b 0c 24                 	movq	(%r12), %rcx
14031b2a4: c6 41 20 01                 	movb	$0x1, 0x20(%rcx)
14031b2a8: fe c2                       	incb	%dl
14031b2aa: 88 50 01                    	movb	%dl, 0x1(%rax)
14031b2ad: e9 d0 02 00 00              	jmp	0x14031b582 <.text+0x31a582>
14031b2b2: 41 83 7d 00 13              	cmpl	$0x13, (%r13)
14031b2b7: 0f 94 c2                    	sete	%dl
14031b2ba: 41 84 d7                    	testb	%dl, %r15b
14031b2bd: 74 4b                       	je	0x14031b30a <.text+0x31a30a>
14031b2bf: 8b 51 1c                    	movl	0x1c(%rcx), %edx
14031b2c2: 4c 8b 41 10                 	movq	0x10(%rcx), %r8
14031b2c6: 41 89 10                    	movl	%edx, (%r8)
14031b2c9: 48 8b 09                    	movq	(%rcx), %rcx
14031b2cc: c6 41 31 01                 	movb	$0x1, 0x31(%rcx)
14031b2d0: 66 c7 00 01 01              	movw	$0x101, (%rax)          # imm = 0x101
14031b2d5: e9 a8 02 00 00              	jmp	0x14031b582 <.text+0x31a582>
14031b2da: 75 06                       	jne	0x14031b2e2 <.text+0x31a2e2>
14031b2dc: c6 40 01 11                 	movb	$0x11, 0x1(%rax)
14031b2e0: eb 31                       	jmp	0x14031b313 <.text+0x31a313>
14031b2e2: 8b 51 1c                    	movl	0x1c(%rcx), %edx
14031b2e5: 4c 8b 41 10                 	movq	0x10(%rcx), %r8
14031b2e9: 41 89 10                    	movl	%edx, (%r8)
14031b2ec: 48 8b 09                    	movq	(%rcx), %rcx
14031b2ef: c6 41 31 00                 	movb	$0x0, 0x31(%rcx)
14031b2f3: 41 8b 4c 24 1c              	movl	0x1c(%r12), %ecx
14031b2f8: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
14031b2fd: 89 0a                       	movl	%ecx, (%rdx)
14031b2ff: 49 8b 0c 24                 	movq	(%r12), %rcx
14031b303: c6 41 20 00                 	movb	$0x0, 0x20(%rcx)
14031b307: c6 00 00                    	movb	$0x0, (%rax)
14031b30a: 45 84 ff                    	testb	%r15b, %r15b
14031b30d: 0f 84 6f 02 00 00           	je	0x14031b582 <.text+0x31a582>
14031b313: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
14031b31a: 48 8b 10                    	movq	(%rax), %rdx
14031b31d: 48 8d 05 77 14 6a 02        	leaq	0x26a1477(%rip), %rax   # 0x1429bc79b
14031b324: 48 8d 0d 6d 14 6a 02        	leaq	0x26a146d(%rip), %rcx   # 0x1429bc798
14031b32b: 48 89 55 10                 	movq	%rdx, 0x10(%rbp)
14031b32f: 80 ba 20 01 00 00 00        	cmpb	$0x0, 0x120(%rdx)
14031b336: 48 0f 45 c8                 	cmovneq	%rax, %rcx
14031b33a: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
14031b33e: 48 c7 45 e8 03 00 00 00     	movq	$0x3, -0x18(%rbp)
14031b346: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b34d: e8 de 27 88 00              	callq	0x140b9db30 <.text+0xb9cb30>
14031b352: 49 89 d0                    	movq	%rdx, %r8
14031b355: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b35c: 48 c7 44 24 20 0b 00 00 00  	movq	$0xb, 0x20(%rsp)
14031b365: 4c 8d 0d 32 14 6a 02        	leaq	0x26a1432(%rip), %r9    # 0x1429bc79e
14031b36c: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
14031b370: 48 89 c2                    	movq	%rax, %rdx
14031b373: e8 98 74 a4 00              	callq	0x140d62810 <.text+0xd61810>
14031b378: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
14031b37c: e8 4f bb a5 00              	callq	0x140d76ed0 <.text+0xd75ed0>
14031b381: 49 89 c6                    	movq	%rax, %r14
14031b384: 49 89 d7                    	movq	%rdx, %r15
14031b387: 48 8d 75 f0                 	leaq	-0x10(%rbp), %rsi
14031b38b: 48 89 f1                    	movq	%rsi, %rcx
14031b38e: e8 fd b2 97 00              	callq	0x140c96690 <.text+0xc95690>
14031b393: 48 89 75 48                 	movq	%rsi, 0x48(%rbp)
14031b397: 48 8d 05 32 07 d2 ff        	leaq	-0x2df8ce(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
14031b39e: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14031b3a2: 48 8d 45 e0                 	leaq	-0x20(%rbp), %rax
14031b3a6: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14031b3aa: 48 8d 05 df ea de ff        	leaq	-0x211521(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14031b3b1: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14031b3b5: 48 8d 05 f4 13 6a 02        	leaq	0x26a13f4(%rip), %rax   # 0x1429bc7b0
14031b3bc: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14031b3c0: 48 c7 45 70 02 00 00 00     	movq	$0x2, 0x70(%rbp)
14031b3c8: 48 8d 45 48                 	leaq	0x48(%rbp), %rax
14031b3cc: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
14031b3d0: 48 c7 85 80 00 00 00 02 00 00 00    	movq	$0x2, 0x80(%rbp)
14031b3db: 48 c7 85 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbp)
14031b3e6: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
14031b3ea: 48 8d 55 68                 	leaq	0x68(%rbp), %rdx
14031b3ee: e8 9d eb a5 00              	callq	0x140d79f90 <.text+0xd78f90>
14031b3f3: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
14031b3f7: 48 85 d2                    	testq	%rdx, %rdx
14031b3fa: 74 0f                       	je	0x14031b40b <.text+0x31a40b>
14031b3fc: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
14031b400: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031b406: e8 15 88 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031b40b: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
14031b40f: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14031b416: 4c 8b 4d d0                 	movq	-0x30(%rbp), %r9
14031b41a: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14031b41e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14031b423: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
14031b427: 4c 89 f2                    	movq	%r14, %rdx
14031b42a: 4d 89 f8                    	movq	%r15, %r8
14031b42d: 4c 89 4d 40                 	movq	%r9, 0x40(%rbp)
14031b431: e8 da 73 a4 00              	callq	0x140d62810 <.text+0xd61810>
14031b436: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
14031b43d: 48 85 d2                    	testq	%rdx, %rdx
14031b440: 74 0f                       	je	0x14031b451 <.text+0x31a451>
14031b442: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031b448: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
14031b44c: e8 cf 87 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031b451: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
14031b455: 48 85 d2                    	testq	%rdx, %rdx
14031b458: 74 0f                       	je	0x14031b469 <.text+0x31a469>
14031b45a: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
14031b45e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031b464: e8 b7 87 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031b469: 48 85 ff                    	testq	%rdi, %rdi
14031b46c: 0f 84 05 02 00 00           	je	0x14031b677 <.text+0x31a677>
14031b472: 48 8b 37                    	movq	(%rdi), %rsi
14031b475: 48 8d 7e 10                 	leaq	0x10(%rsi), %rdi
14031b479: b9 08 00 00 00              	movl	$0x8, %ecx
14031b47e: 31 c0                       	xorl	%eax, %eax
14031b480: f0                          	lock
14031b481: 48 0f b1 4e 10              	cmpxchgq	%rcx, 0x10(%rsi)
14031b486: 0f 85 34 11 00 00           	jne	0x14031c5c0 <.text+0x31b5c0>
14031b48c: c6 85 3e 01 00 00 01        	movb	$0x1, 0x13e(%rbp)
14031b493: 48 89 f9                    	movq	%rdi, %rcx
14031b496: e8 d5 67 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031b49b: 49 89 fe                    	movq	%rdi, %r14
14031b49e: 49 83 ce 01                 	orq	$0x1, %r14
14031b4a2: c6 85 3e 01 00 00 01        	movb	$0x1, 0x13e(%rbp)
14031b4a9: 4c 89 f1                    	movq	%r14, %rcx
14031b4ac: e8 bf 67 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031b4b1: 0f b6 05 51 97 31 03        	movzbl	0x3319751(%rip), %eax   # 0x143634c09
14031b4b8: b9 40 00 00 00              	movl	$0x40, %ecx
14031b4bd: ba 08 00 00 00              	movl	$0x8, %edx
14031b4c2: e8 49 87 2f 00              	callq	0x140613c10 <.text+0x612c10>
14031b4c7: 48 85 c0                    	testq	%rax, %rax
14031b4ca: 48 89 7d 40                 	movq	%rdi, 0x40(%rbp)
14031b4ce: 0f 84 6c 11 00 00           	je	0x14031c640 <.text+0x31b640>
14031b4d4: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14031b4db: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14031b4e3: c6 40 10 00                 	movb	$0x0, 0x10(%rax)
14031b4e7: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
14031b4f1: 48 89 48 18                 	movq	%rcx, 0x18(%rax)
14031b4f5: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14031b4f9: f0                          	lock
14031b4fa: 48 ff 00                    	incq	(%rax)
14031b4fd: 0f 8e 7a 11 00 00           	jle	0x14031c67d <.text+0x31b67d>
14031b503: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14031b50a: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14031b50e: 48 8b 7e 28                 	movq	0x28(%rsi), %rdi
14031b512: 48 3b 7e 18                 	cmpq	0x18(%rsi), %rdi
14031b516: 75 10                       	jne	0x14031b528 <.text+0x31a528>
14031b518: 48 8d 4e 18                 	leaq	0x18(%rsi), %rcx
14031b51c: 48 8d 15 ad 12 6a 02        	leaq	0x26a12ad(%rip), %rdx   # 0x1429bc7d0
14031b523: e8 48 73 24 00              	callq	0x140562870 <.text+0x561870>
14031b528: 48 8b 46 20                 	movq	0x20(%rsi), %rax
14031b52c: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031b533: 48 89 0c f8                 	movq	%rcx, (%rax,%rdi,8)
14031b537: 48 ff c7                    	incq	%rdi
14031b53a: 48 89 7e 28                 	movq	%rdi, 0x28(%rsi)
14031b53e: 4c 8b 7d 58                 	movq	0x58(%rbp), %r15
14031b542: 4d 85 ff                    	testq	%r15, %r15
14031b545: 0f 88 06 11 00 00           	js	0x14031c651 <.text+0x31b651>
14031b54b: 4c 89 ef                    	movq	%r13, %rdi
14031b54e: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
14031b552: 0f 84 26 01 00 00           	je	0x14031b67e <.text+0x31a67e>
14031b558: 0f b6 05 aa 96 31 03        	movzbl	0x33196aa(%rip), %eax   # 0x143634c09
14031b55f: be 01 00 00 00              	movl	$0x1, %esi
14031b564: ba 01 00 00 00              	movl	$0x1, %edx
14031b569: 4c 89 f9                    	movq	%r15, %rcx
14031b56c: e8 9f 86 2f 00              	callq	0x140613c10 <.text+0x612c10>
14031b571: 48 85 c0                    	testq	%rax, %rax
14031b574: 0f 84 d9 10 00 00           	je	0x14031c653 <.text+0x31b653>
14031b57a: 49 89 c5                    	movq	%rax, %r13
14031b57d: e9 02 01 00 00              	jmp	0x14031b684 <.text+0x31a684>
14031b582: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14031b589: 48 8b 38                    	movq	(%rax), %rdi
14031b58c: 8b 8f f0 00 00 00           	movl	0xf0(%rdi), %ecx
14031b592: f3 0f 10 8f f4 00 00 00     	movss	0xf4(%rdi), %xmm1
14031b59a: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b5a1: e8 ea 44 62 00              	callq	0x14093fa90 <.text+0x93ea90>
14031b5a6: 84 c0                       	testb	%al, %al
14031b5a8: 0f 84 6a 02 00 00           	je	0x14031b818 <.text+0x31a818>
14031b5ae: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b5b5: 48 8d 15 44 12 6a 02        	leaq	0x26a1244(%rip), %rdx   # 0x1429bc800
14031b5bc: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
14031b5c0: 41 b8 15 00 00 00           	movl	$0x15, %r8d
14031b5c6: e8 35 65 a4 00              	callq	0x140d61b00 <.text+0xd60b00>
14031b5cb: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
14031b5cf: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
14031b5d3: 0f 11 8d 90 00 00 00        	movups	%xmm1, 0x90(%rbp)
14031b5da: 0f 11 85 80 00 00 00        	movups	%xmm0, 0x80(%rbp)
14031b5e1: c6 85 b4 00 00 00 00        	movb	$0x0, 0xb4(%rbp)
14031b5e8: c7 85 a8 00 00 00 00 00 00 00       	movl	$0x0, 0xa8(%rbp)
14031b5f2: c6 85 a0 00 00 00 64        	movb	$0x64, 0xa0(%rbp)
14031b5f9: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14031b601: 48 8b b5 20 01 00 00        	movq	0x120(%rbp), %rsi
14031b608: 48 3b b5 10 01 00 00        	cmpq	0x110(%rbp), %rsi
14031b60f: 75 13                       	jne	0x14031b624 <.text+0x31a624>
14031b611: 48 8d 15 00 12 6a 02        	leaq	0x26a1200(%rip), %rdx   # 0x1429bc818
14031b618: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14031b61f: e8 1c 7f 24 00              	callq	0x140563540 <.text+0x562540>
14031b624: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
14031b62b: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
14031b62f: 48 c1 e1 04                 	shlq	$0x4, %rcx
14031b633: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031b637: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031b63b: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031b642: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031b649: 0f 10 a5 a8 00 00 00        	movups	0xa8(%rbp), %xmm4
14031b650: 0f 11 64 08 40              	movups	%xmm4, 0x40(%rax,%rcx)
14031b655: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
14031b65a: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
14031b65f: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
14031b664: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
14031b668: 48 ff c6                    	incq	%rsi
14031b66b: 48 89 b5 20 01 00 00        	movq	%rsi, 0x120(%rbp)
14031b672: e9 a1 01 00 00              	jmp	0x14031b818 <.text+0x31a818>
14031b677: 31 f6                       	xorl	%esi, %esi
14031b679: e9 bc 00 00 00              	jmp	0x14031b73a <.text+0x31a73a>
14031b67e: 41 bd 01 00 00 00           	movl	$0x1, %r13d
14031b684: 4c 89 e9                    	movq	%r13, %rcx
14031b687: 4c 89 e2                    	movq	%r12, %rdx
14031b68a: 4d 89 f8                    	movq	%r15, %r8
14031b68d: e8 6e 85 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031b692: 0f b6 45 60                 	movzbl	0x60(%rbp), %eax
14031b696: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031b69d: 48 89 8d 88 00 00 00        	movq	%rcx, 0x88(%rbp)
14031b6a4: 4c 89 7d 68                 	movq	%r15, 0x68(%rbp)
14031b6a8: 4c 89 6d 70                 	movq	%r13, 0x70(%rbp)
14031b6ac: 4c 89 7d 78                 	movq	%r15, 0x78(%rbp)
14031b6b0: 88 85 80 00 00 00           	movb	%al, 0x80(%rbp)
14031b6b6: 0f b6 05 4c 95 31 03        	movzbl	0x331954c(%rip), %eax   # 0x143634c09
14031b6bd: b9 28 00 00 00              	movl	$0x28, %ecx
14031b6c2: ba 08 00 00 00              	movl	$0x8, %edx
14031b6c7: e8 44 85 2f 00              	callq	0x140613c10 <.text+0x612c10>
14031b6cc: 48 85 c0                    	testq	%rax, %rax
14031b6cf: 0f 84 99 0f 00 00           	je	0x14031c66e <.text+0x31b66e>
14031b6d5: 49 89 c7                    	movq	%rax, %r15
14031b6d8: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
14031b6df: 49 89 47 20                 	movq	%rax, 0x20(%r15)
14031b6e3: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031b6e7: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031b6eb: 41 0f 11 4f 10              	movups	%xmm1, 0x10(%r15)
14031b6f0: 41 0f 11 07                 	movups	%xmm0, (%r15)
14031b6f4: c6 85 3e 01 00 00 01        	movb	$0x1, 0x13e(%rbp)
14031b6fb: 48 8b 75 40                 	movq	0x40(%rbp), %rsi
14031b6ff: 48 89 f1                    	movq	%rsi, %rcx
14031b702: e8 09 67 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031b707: 4c 8b a5 d0 01 00 00        	movq	0x1d0(%rbp), %r12
14031b70e: 49 89 fd                    	movq	%rdi, %r13
14031b711: 48 89 f7                    	movq	%rsi, %rdi
14031b714: c6 85 3e 01 00 00 01        	movb	$0x1, 0x13e(%rbp)
14031b71b: 4c 89 f1                    	movq	%r14, %rcx
14031b71e: e8 ed 66 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031b723: 31 c9                       	xorl	%ecx, %ecx
14031b725: b8 08 00 00 00              	movl	$0x8, %eax
14031b72a: f0                          	lock
14031b72b: 48 0f b1 0f                 	cmpxchgq	%rcx, (%rdi)
14031b72f: be 01 00 00 00              	movl	$0x1, %esi
14031b734: 0f 85 a0 0e 00 00           	jne	0x14031c5da <.text+0x31b5da>
14031b73a: 41 83 7d 00 13              	cmpl	$0x13, (%r13)
14031b73f: 0f 94 c0                    	sete	%al
14031b742: b1 04                       	movb	$0x4, %cl
14031b744: 28 c1                       	subb	%al, %cl
14031b746: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
14031b74a: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
14031b74e: 0f 11 8d 90 00 00 00        	movups	%xmm1, 0x90(%rbp)
14031b755: 0f 11 85 80 00 00 00        	movups	%xmm0, 0x80(%rbp)
14031b75c: 48 8b 45 10                 	movq	0x10(%rbp), %rax
14031b760: 80 b8 20 01 00 00 00        	cmpb	$0x0, 0x120(%rax)
14031b767: b8 64 00 00 00              	movl	$0x64, %eax
14031b76c: ba 55 00 00 00              	movl	$0x55, %edx
14031b771: 0f 45 d0                    	cmovnel	%eax, %edx
14031b774: 88 8d b4 00 00 00           	movb	%cl, 0xb4(%rbp)
14031b77a: c7 85 a8 00 00 00 00 00 00 00       	movl	$0x0, 0xa8(%rbp)
14031b784: 88 95 a0 00 00 00           	movb	%dl, 0xa0(%rbp)
14031b78a: 48 89 75 68                 	movq	%rsi, 0x68(%rbp)
14031b78e: 4c 89 7d 70                 	movq	%r15, 0x70(%rbp)
14031b792: 48 8d 05 e7 5a 69 02        	leaq	0x2695ae7(%rip), %rax   # 0x1429b1280
14031b799: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
14031b79d: 48 8b b5 20 01 00 00        	movq	0x120(%rbp), %rsi
14031b7a4: 48 3b b5 10 01 00 00        	cmpq	0x110(%rbp), %rsi
14031b7ab: 75 13                       	jne	0x14031b7c0 <.text+0x31a7c0>
14031b7ad: 48 8d 15 34 10 6a 02        	leaq	0x26a1034(%rip), %rdx   # 0x1429bc7e8
14031b7b4: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14031b7bb: e8 80 7d 24 00              	callq	0x140563540 <.text+0x562540>
14031b7c0: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
14031b7c7: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
14031b7cb: 48 c1 e1 04                 	shlq	$0x4, %rcx
14031b7cf: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031b7d3: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031b7d7: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031b7de: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031b7e5: 0f 10 a5 a8 00 00 00        	movups	0xa8(%rbp), %xmm4
14031b7ec: 0f 11 64 08 40              	movups	%xmm4, 0x40(%rax,%rcx)
14031b7f1: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
14031b7f6: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
14031b7fb: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
14031b800: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
14031b804: 48 ff c6                    	incq	%rsi
14031b807: 48 89 b5 20 01 00 00        	movq	%rsi, 0x120(%rbp)
14031b80e: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14031b815: 48 8b 38                    	movq	(%rax), %rdi
14031b818: 4c 8b 55 38                 	movq	0x38(%rbp), %r10
14031b81c: 49 8b 0a                    	movq	(%r10), %rcx
14031b81f: 31 c0                       	xorl	%eax, %eax
14031b821: 48 89 ca                    	movq	%rcx, %rdx
14031b824: 48 2b 53 18                 	subq	0x18(%rbx), %rdx
14031b828: 48 0f 42 d0                 	cmovbq	%rax, %rdx
14031b82c: 48 2b 4b 38                 	subq	0x38(%rbx), %rcx
14031b830: 48 0f 42 c8                 	cmovbq	%rax, %rcx
14031b834: 4c 8b 43 30                 	movq	0x30(%rbx), %r8
14031b838: 49 29 c8                    	subq	%rcx, %r8
14031b83b: 4c 0f 42 c0                 	cmovbq	%rax, %r8
14031b83f: 0f 97 c1                    	seta	%cl
14031b842: 4c 8b 4b 10                 	movq	0x10(%rbx), %r9
14031b846: 49 29 d1                    	subq	%rdx, %r9
14031b849: 4c 0f 42 c8                 	cmovbq	%rax, %r9
14031b84d: 48 8b 53 40                 	movq	0x40(%rbx), %rdx
14031b851: 40 0f 97 c6                 	seta	%sil
14031b855: 4d 01 c1                    	addq	%r8, %r9
14031b858: 40 08 ce                    	orb	%cl, %sil
14031b85b: 4c 0f 45 c8                 	cmovneq	%rax, %r9
14031b85f: 4c 29 ca                    	subq	%r9, %rdx
14031b862: 49 89 12                    	movq	%rdx, (%r10)
14031b865: 8b 8f f8 00 00 00           	movl	0xf8(%rdi), %ecx
14031b86b: f3 0f 10 8f fc 00 00 00     	movss	0xfc(%rdi), %xmm1
14031b873: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b87a: e8 11 42 62 00              	callq	0x14093fa90 <.text+0x93ea90>
14031b87f: 48 8b bd 00 02 00 00        	movq	0x200(%rbp), %rdi
14031b886: 84 c0                       	testb	%al, %al
14031b888: 74 1d                       	je	0x14031b8a7 <.text+0x31a8a7>
14031b88a: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14031b891: 48 8b 00                    	movq	(%rax), %rax
14031b894: 83 38 08                    	cmpl	$0x8, (%rax)
14031b897: 41 0f 95 c6                 	setne	%r14b
14031b89b: 48 8b 1f                    	movq	(%rdi), %rbx
14031b89e: 83 3b 08                    	cmpl	$0x8, (%rbx)
14031b8a1: 74 0f                       	je	0x14031b8b2 <.text+0x31a8b2>
14031b8a3: 31 c0                       	xorl	%eax, %eax
14031b8a5: eb 21                       	jmp	0x14031b8c8 <.text+0x31a8c8>
14031b8a7: 45 31 f6                    	xorl	%r14d, %r14d
14031b8aa: 48 8b 1f                    	movq	(%rdi), %rbx
14031b8ad: 83 3b 08                    	cmpl	$0x8, (%rbx)
14031b8b0: 75 f1                       	jne	0x14031b8a3 <.text+0x31a8a3>
14031b8b2: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14031b8b9: 48 8b 08                    	movq	(%rax), %rcx
14031b8bc: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031b8c3: e8 18 43 62 00              	callq	0x14093fbe0 <.text+0x93ebe0>
14031b8c8: 41 08 c6                    	orb	%al, %r14b
14031b8cb: 41 08 f6                    	orb	%sil, %r14b
14031b8ce: 41 80 fe 01                 	cmpb	$0x1, %r14b
14031b8d2: 0f 85 72 03 00 00           	jne	0x14031bc4a <.text+0x31ac4a>
14031b8d8: 0f b6 05 2a 93 31 03        	movzbl	0x331932a(%rip), %eax   # 0x143634c09
14031b8df: b9 30 00 00 00              	movl	$0x30, %ecx
14031b8e4: ba 08 00 00 00              	movl	$0x8, %edx
14031b8e9: e8 22 83 2f 00              	callq	0x140613c10 <.text+0x612c10>
14031b8ee: 48 85 c0                    	testq	%rax, %rax
14031b8f1: 0f 84 f9 0c 00 00           	je	0x14031c5f0 <.text+0x31b5f0>
14031b8f7: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14031b8fe: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14031b906: c6 40 10 00                 	movb	$0x0, 0x10(%rax)
14031b90a: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
14031b914: 48 89 48 18                 	movq	%rcx, 0x18(%rax)
14031b918: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14031b91c: f0                          	lock
14031b91d: 48 ff 00                    	incq	(%rax)
14031b920: 0f 8e 57 0d 00 00           	jle	0x14031c67d <.text+0x31b67d>
14031b926: c6 85 b4 00 00 00 06        	movb	$0x6, 0xb4(%rbp)
14031b92d: c7 85 a8 00 00 00 00 00 00 00       	movl	$0x0, 0xa8(%rbp)
14031b937: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
14031b93e: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14031b945: c6 85 90 00 00 00 3c        	movb	$0x3c, 0x90(%rbp)
14031b94c: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14031b954: 0f 10 45 b8                 	movups	-0x48(%rbp), %xmm0
14031b958: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
14031b95c: 4c 8b b5 20 01 00 00        	movq	0x120(%rbp), %r14
14031b963: 4c 3b b5 10 01 00 00        	cmpq	0x110(%rbp), %r14
14031b96a: 75 13                       	jne	0x14031b97f <.text+0x31a97f>
14031b96c: 48 8d 15 bd 0e 6a 02        	leaq	0x26a0ebd(%rip), %rdx   # 0x1429bc830
14031b973: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14031b97a: e8 c1 7b 24 00              	callq	0x140563540 <.text+0x562540>
14031b97f: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
14031b986: 4b 8d 0c b6                 	leaq	(%r14,%r14,4), %rcx
14031b98a: 48 c1 e1 04                 	shlq	$0x4, %rcx
14031b98e: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031b992: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031b996: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031b99d: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031b9a4: 0f 10 a5 a8 00 00 00        	movups	0xa8(%rbp), %xmm4
14031b9ab: 0f 11 64 08 40              	movups	%xmm4, 0x40(%rax,%rcx)
14031b9b0: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
14031b9b5: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
14031b9ba: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
14031b9bf: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
14031b9c3: 49 ff c6                    	incq	%r14
14031b9c6: 4c 89 b5 20 01 00 00        	movq	%r14, 0x120(%rbp)
14031b9cd: 48 c7 45 68 01 00 00 00     	movq	$0x1, 0x68(%rbp)
14031b9d5: 48 c7 45 70 01 00 00 00     	movq	$0x1, 0x70(%rbp)
14031b9dd: c6 45 78 00                 	movb	$0x0, 0x78(%rbp)
14031b9e1: c6 85 b6 00 00 00 07        	movb	$0x7, 0xb6(%rbp)
14031b9e8: 0f b6 05 1a 92 31 03        	movzbl	0x331921a(%rip), %eax   # 0x143634c09
14031b9ef: b9 50 00 00 00              	movl	$0x50, %ecx
14031b9f4: ba 08 00 00 00              	movl	$0x8, %edx
14031b9f9: e8 12 82 2f 00              	callq	0x140613c10 <.text+0x612c10>
14031b9fe: 48 85 c0                    	testq	%rax, %rax
14031ba01: 0f 84 01 0c 00 00           	je	0x14031c608 <.text+0x31b608>
14031ba07: 0f 10 85 a8 00 00 00        	movups	0xa8(%rbp), %xmm0
14031ba0e: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
14031ba12: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031ba16: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031ba1a: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031ba21: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031ba28: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
14031ba2c: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14031ba30: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
14031ba34: 0f 11 00                    	movups	%xmm0, (%rax)
14031ba37: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14031ba3b: 49 8b 14 24                 	movq	(%r12), %rdx
14031ba3f: 8b 0a                       	movl	(%rdx), %ecx
14031ba41: 8b 52 04                    	movl	0x4(%rdx), %edx
14031ba44: f0                          	lock
14031ba45: 48 ff 00                    	incq	(%rax)
14031ba48: 0f 8e 2f 0c 00 00           	jle	0x14031c67d <.text+0x31b67d>
14031ba4e: 89 d2                       	movl	%edx, %edx
14031ba50: 0f 57 c0                    	xorps	%xmm0, %xmm0
14031ba53: f3 48 0f 2a c2              	cvtsi2ss	%rdx, %xmm0
14031ba58: f3 0f 10 0d 24 9f 60 02     	movss	0x2609f24(%rip), %xmm1  # 0x142925984
14031ba60: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
14031ba64: f3 4c 0f 2c c8              	cvttss2si	%xmm0, %r9
14031ba69: 31 d2                       	xorl	%edx, %edx
14031ba6b: 0f 57 d2                    	xorps	%xmm2, %xmm2
14031ba6e: 0f 2e c2                    	ucomiss	%xmm2, %xmm0
14031ba71: 44 0f 42 ca                 	cmovbl	%edx, %r9d
14031ba75: f3 0f 10 1d 53 4d 68 02     	movss	0x2684d53(%rip), %xmm3  # 0x1429a07d0
14031ba7d: 0f 2e c3                    	ucomiss	%xmm3, %xmm0
14031ba80: 41 b8 ff ff ff ff           	movl	$0xffffffff, %r8d       # imm = 0xFFFFFFFF
14031ba86: 45 0f 47 c8                 	cmoval	%r8d, %r9d
14031ba8a: 41 83 f9 01                 	cmpl	$0x1, %r9d
14031ba8e: 41 83 d1 00                 	adcl	$0x0, %r9d
14031ba92: 89 c9                       	movl	%ecx, %ecx
14031ba94: 0f 57 c0                    	xorps	%xmm0, %xmm0
14031ba97: f3 48 0f 2a c1              	cvtsi2ss	%rcx, %xmm0
14031ba9c: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
14031baa0: f3 48 0f 2c c8              	cvttss2si	%xmm0, %rcx
14031baa5: 0f 2e c2                    	ucomiss	%xmm2, %xmm0
14031baa8: 0f 42 ca                    	cmovbl	%edx, %ecx
14031baab: 0f 2e c3                    	ucomiss	%xmm3, %xmm0
14031baae: 41 0f 47 c8                 	cmoval	%r8d, %ecx
14031bab2: 83 f9 01                    	cmpl	$0x1, %ecx
14031bab5: 83 d1 00                    	adcl	$0x0, %ecx
14031bab8: 49 c1 e1 20                 	shlq	$0x20, %r9
14031babc: 4c 09 c9                    	orq	%r9, %rcx
14031babf: c6 85 b4 00 00 00 06        	movb	$0x6, 0xb4(%rbp)
14031bac6: c7 85 a8 00 00 00 01 00 00 00       	movl	$0x1, 0xa8(%rbp)
14031bad0: 48 89 8d ac 00 00 00        	movq	%rcx, 0xac(%rbp)
14031bad7: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
14031bae1: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
14031bae8: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
14031baef: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14031baf6: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14031bafe: 0f 10 45 b8                 	movups	-0x48(%rbp), %xmm0
14031bb02: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
14031bb06: 4c 8b b5 20 01 00 00        	movq	0x120(%rbp), %r14
14031bb0d: 4c 3b b5 10 01 00 00        	cmpq	0x110(%rbp), %r14
14031bb14: 75 13                       	jne	0x14031bb29 <.text+0x31ab29>
14031bb16: 48 8d 15 2b 0d 6a 02        	leaq	0x26a0d2b(%rip), %rdx   # 0x1429bc848
14031bb1d: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14031bb24: e8 17 7a 24 00              	callq	0x140563540 <.text+0x562540>
14031bb29: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
14031bb30: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
14031bb37: 4b 8d 14 b6                 	leaq	(%r14,%r14,4), %rdx
14031bb3b: 48 c1 e2 04                 	shlq	$0x4, %rdx
14031bb3f: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031bb43: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031bb47: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031bb4e: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031bb55: 0f 10 a5 a8 00 00 00        	movups	0xa8(%rbp), %xmm4
14031bb5c: 0f 11 64 11 40              	movups	%xmm4, 0x40(%rcx,%rdx)
14031bb61: 0f 11 5c 11 30              	movups	%xmm3, 0x30(%rcx,%rdx)
14031bb66: 0f 11 54 11 20              	movups	%xmm2, 0x20(%rcx,%rdx)
14031bb6b: 0f 11 4c 11 10              	movups	%xmm1, 0x10(%rcx,%rdx)
14031bb70: 0f 11 04 11                 	movups	%xmm0, (%rcx,%rdx)
14031bb74: 49 ff c6                    	incq	%r14
14031bb77: 4c 89 b5 20 01 00 00        	movq	%r14, 0x120(%rbp)
14031bb7e: 44 8b 70 1c                 	movl	0x1c(%rax), %r14d
14031bb82: 4c 8b 78 10                 	movq	0x10(%rax), %r15
14031bb86: 45 89 37                    	movl	%r14d, (%r15)
14031bb89: 48 8b 00                    	movq	(%rax), %rax
14031bb8c: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14031bb93: 0f b6 40 3e                 	movzbl	0x3e(%rax), %eax
14031bb97: 04 f9                       	addb	$-0x7, %al
14031bb99: 0f b6 c8                    	movzbl	%al, %ecx
14031bb9c: 80 f9 02                    	cmpb	$0x2, %cl
14031bb9f: b8 02 00 00 00              	movl	$0x2, %eax
14031bba4: 0f 42 c1                    	cmovbl	%ecx, %eax
14031bba7: 84 c0                       	testb	%al, %al
14031bba9: 74 35                       	je	0x14031bbe0 <.text+0x31abe0>
14031bbab: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031bbb2: 48 83 c1 08                 	addq	$0x8, %rcx
14031bbb6: 0f b6 c0                    	movzbl	%al, %eax
14031bbb9: 83 f8 01                    	cmpl	$0x1, %eax
14031bbbc: 75 10                       	jne	0x14031bbce <.text+0x31abce>
14031bbbe: 48 8b 01                    	movq	(%rcx), %rax
14031bbc1: f0                          	lock
14031bbc2: 48 ff 08                    	decq	(%rax)
14031bbc5: 75 19                       	jne	0x14031bbe0 <.text+0x31abe0>
14031bbc7: e8 f4 0c 84 00              	callq	0x140b5c8c0 <.text+0xb5b8c0>
14031bbcc: eb 12                       	jmp	0x14031bbe0 <.text+0x31abe0>
14031bbce: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
14031bbd2: e8 a9 ff d1 ff              	callq	0x14003bb80 <.text+0x3ab80>
14031bbd7: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14031bbdb: e8 d0 68 8c 00              	callq	0x140be24b0 <.text+0xbe14b0>
14031bbe0: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14031bbe7: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031bbee: 48 89 41 08                 	movq	%rax, 0x8(%rcx)
14031bbf2: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031bbf6: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031bbfa: 0f 10 95 86 00 00 00        	movups	0x86(%rbp), %xmm2
14031bc01: 0f 11 41 10                 	movups	%xmm0, 0x10(%rcx)
14031bc05: 0f 11 49 20                 	movups	%xmm1, 0x20(%rcx)
14031bc09: 0f 11 51 2e                 	movups	%xmm2, 0x2e(%rcx)
14031bc0d: c6 41 3e 08                 	movb	$0x8, 0x3e(%rcx)
14031bc11: 48 8b 45 18                 	movq	0x18(%rbp), %rax
14031bc15: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
14031bc1c: 45 89 37                    	movl	%r14d, (%r15)
14031bc1f: 48 8b 01                    	movq	(%rcx), %rax
14031bc22: 48 85 c0                    	testq	%rax, %rax
14031bc25: 74 12                       	je	0x14031bc39 <.text+0x31ac39>
14031bc27: f0                          	lock
14031bc28: 48 ff 08                    	decq	(%rax)
14031bc2b: 75 0c                       	jne	0x14031bc39 <.text+0x31ac39>
14031bc2d: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031bc34: e8 57 0d 84 00              	callq	0x140b5c990 <.text+0xb5b990>
14031bc39: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
14031bc40: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
14031bc47: 48 89 08                    	movq	%rcx, (%rax)
14031bc4a: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14031bc51: 48 8b 00                    	movq	(%rax), %rax
14031bc54: 83 38 0a                    	cmpl	$0xa, (%rax)
14031bc57: 0f 85 ec 01 00 00           	jne	0x14031be49 <.text+0x31ae49>
14031bc5d: f6 40 08 01                 	testb	$0x1, 0x8(%rax)
14031bc61: 0f 85 e2 01 00 00           	jne	0x14031be49 <.text+0x31ae49>
14031bc67: 8b 47 1c                    	movl	0x1c(%rdi), %eax
14031bc6a: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
14031bc6e: 89 01                       	movl	%eax, (%rcx)
14031bc70: c6 45 70 01                 	movb	$0x1, 0x70(%rbp)
14031bc74: 48 c7 45 68 0a 00 00 00     	movq	$0xa, 0x68(%rbp)
14031bc7c: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031bc83: 48 8d 55 68                 	leaq	0x68(%rbp), %rdx
14031bc87: 48 89 d9                    	movq	%rbx, %rcx
14031bc8a: e8 41 3e 62 00              	callq	0x14093fad0 <.text+0x93ead0>
14031bc8f: 49 8b 04 24                 	movq	(%r12), %rax
14031bc93: 8b 40 04                    	movl	0x4(%rax), %eax
14031bc96: 0f 57 c0                    	xorps	%xmm0, %xmm0
14031bc99: f3 48 0f 2a c0              	cvtsi2ss	%rax, %xmm0
14031bc9e: f3 0f 59 05 ca 9c 60 02     	mulss	0x2609cca(%rip), %xmm0  # 0x142925970
14031bca6: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
14031bcab: 31 c9                       	xorl	%ecx, %ecx
14031bcad: 0f 57 c9                    	xorps	%xmm1, %xmm1
14031bcb0: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14031bcb3: 0f 43 c8                    	cmovael	%eax, %ecx
14031bcb6: 0f 2e 05 13 4b 68 02        	ucomiss	0x2684b13(%rip), %xmm0  # 0x1429a07d0
14031bcbd: bf ff ff ff ff              	movl	$0xffffffff, %edi       # imm = 0xFFFFFFFF
14031bcc2: 0f 46 f9                    	cmovbel	%ecx, %edi
14031bcc5: 83 ff 01                    	cmpl	$0x1, %edi
14031bcc8: 83 d7 00                    	adcl	$0x0, %edi
14031bccb: 48 c7 45 68 01 00 00 00     	movq	$0x1, 0x68(%rbp)
14031bcd3: 48 c7 45 70 01 00 00 00     	movq	$0x1, 0x70(%rbp)
14031bcdb: c6 45 78 00                 	movb	$0x0, 0x78(%rbp)
14031bcdf: c6 85 b6 00 00 00 07        	movb	$0x7, 0xb6(%rbp)
14031bce6: 0f b6 05 1c 8f 31 03        	movzbl	0x3318f1c(%rip), %eax   # 0x143634c09
14031bced: b9 50 00 00 00              	movl	$0x50, %ecx
14031bcf2: ba 08 00 00 00              	movl	$0x8, %edx
14031bcf7: e8 14 7f 2f 00              	callq	0x140613c10 <.text+0x612c10>
14031bcfc: 48 85 c0                    	testq	%rax, %rax
14031bcff: 0f 84 14 09 00 00           	je	0x14031c619 <.text+0x31b619>
14031bd05: 0f 10 85 a8 00 00 00        	movups	0xa8(%rbp), %xmm0
14031bd0c: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
14031bd10: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031bd14: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031bd18: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031bd1f: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031bd26: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
14031bd2a: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14031bd2e: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
14031bd32: 0f 11 00                    	movups	%xmm0, (%rax)
14031bd35: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14031bd39: f0                          	lock
14031bd3a: 48 ff 00                    	incq	(%rax)
14031bd3d: 0f 8e 3a 09 00 00           	jle	0x14031c67d <.text+0x31b67d>
14031bd43: 89 fa                       	movl	%edi, %edx
14031bd45: 48 89 d1                    	movq	%rdx, %rcx
14031bd48: 48 c1 e1 20                 	shlq	$0x20, %rcx
14031bd4c: 48 09 d1                    	orq	%rdx, %rcx
14031bd4f: c6 85 b4 00 00 00 01        	movb	$0x1, 0xb4(%rbp)
14031bd56: c7 85 a8 00 00 00 01 00 00 00       	movl	$0x1, 0xa8(%rbp)
14031bd60: 48 89 8d ac 00 00 00        	movq	%rcx, 0xac(%rbp)
14031bd67: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
14031bd71: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
14031bd78: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14031bd7f: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14031bd86: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14031bd8e: 48 8b bd 20 01 00 00        	movq	0x120(%rbp), %rdi
14031bd95: 48 3b bd 10 01 00 00        	cmpq	0x110(%rbp), %rdi
14031bd9c: 75 13                       	jne	0x14031bdb1 <.text+0x31adb1>
14031bd9e: 48 8d 15 bb 0a 6a 02        	leaq	0x26a0abb(%rip), %rdx   # 0x1429bc860
14031bda5: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14031bdac: e8 8f 77 24 00              	callq	0x140563540 <.text+0x562540>
14031bdb1: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
14031bdb8: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
14031bdbf: 48 8d 14 bf                 	leaq	(%rdi,%rdi,4), %rdx
14031bdc3: 48 c1 e2 04                 	shlq	$0x4, %rdx
14031bdc7: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031bdcb: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031bdcf: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031bdd6: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031bddd: 0f 10 a5 a8 00 00 00        	movups	0xa8(%rbp), %xmm4
14031bde4: 0f 11 64 11 40              	movups	%xmm4, 0x40(%rcx,%rdx)
14031bde9: 0f 11 5c 11 30              	movups	%xmm3, 0x30(%rcx,%rdx)
14031bdee: 0f 11 54 11 20              	movups	%xmm2, 0x20(%rcx,%rdx)
14031bdf3: 0f 11 4c 11 10              	movups	%xmm1, 0x10(%rcx,%rdx)
14031bdf8: 0f 11 04 11                 	movups	%xmm0, (%rcx,%rdx)
14031bdfc: 48 ff c7                    	incq	%rdi
14031bdff: 48 89 bd 20 01 00 00        	movq	%rdi, 0x120(%rbp)
14031be06: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14031be09: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14031be0d: 89 0a                       	movl	%ecx, (%rdx)
14031be0f: 48 8b 08                    	movq	(%rax), %rcx
14031be12: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14031be16: 48 83 c1 08                 	addq	$0x8, %rcx
14031be1a: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
14031be21: 48 85 c0                    	testq	%rax, %rax
14031be24: 74 12                       	je	0x14031be38 <.text+0x31ae38>
14031be26: f0                          	lock
14031be27: 48 ff 08                    	decq	(%rax)
14031be2a: 75 0c                       	jne	0x14031be38 <.text+0x31ae38>
14031be2c: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
14031be33: e8 88 0a 84 00              	callq	0x140b5c8c0 <.text+0xb5b8c0>
14031be38: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
14031be3f: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
14031be46: 48 89 01                    	movq	%rax, (%rcx)
14031be49: 40 84 f6                    	testb	%sil, %sil
14031be4c: 0f 84 b6 00 00 00           	je	0x14031bf08 <.text+0x31af08>
14031be52: 48 8b 45 08                 	movq	0x8(%rbp), %rax
14031be56: 48 8b 00                    	movq	(%rax), %rax
14031be59: 48 8b 00                    	movq	(%rax), %rax
14031be5c: f0                          	lock
14031be5d: 48 ff 00                    	incq	(%rax)
14031be60: 0f 8e 17 08 00 00           	jle	0x14031c67d <.text+0x31b67d>
14031be66: c6 85 b4 00 00 00 02        	movb	$0x2, 0xb4(%rbp)
14031be6d: c7 85 a8 00 00 00 00 00 00 00       	movl	$0x0, 0xa8(%rbp)
14031be77: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
14031be81: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
14031be88: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14031be8f: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14031be97: 48 8b b5 20 01 00 00        	movq	0x120(%rbp), %rsi
14031be9e: 48 3b b5 10 01 00 00        	cmpq	0x110(%rbp), %rsi
14031bea5: 75 13                       	jne	0x14031beba <.text+0x31aeba>
14031bea7: 48 8d 15 ca 09 6a 02        	leaq	0x26a09ca(%rip), %rdx   # 0x1429bc878
14031beae: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14031beb5: e8 86 76 24 00              	callq	0x140563540 <.text+0x562540>
14031beba: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
14031bec1: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
14031bec5: 48 c1 e1 04                 	shlq	$0x4, %rcx
14031bec9: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031becd: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031bed1: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031bed8: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031bedf: 0f 10 a5 a8 00 00 00        	movups	0xa8(%rbp), %xmm4
14031bee6: 0f 11 64 08 40              	movups	%xmm4, 0x40(%rax,%rcx)
14031beeb: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
14031bef0: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
14031bef5: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
14031befa: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
14031befe: 48 ff c6                    	incq	%rsi
14031bf01: 48 89 b5 20 01 00 00        	movq	%rsi, 0x120(%rbp)
14031bf08: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
14031bf0f: 48 8b 01                    	movq	(%rcx), %rax
14031bf12: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
14031bf16: 4c 8b 00                    	movq	(%rax), %r8
14031bf19: 31 d2                       	xorl	%edx, %edx
14031bf1b: 4d 89 c1                    	movq	%r8, %r9
14031bf1e: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
14031bf22: 4c 0f 42 ca                 	cmovbq	%rdx, %r9
14031bf26: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
14031bf2a: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
14031bf2e: 4f 8d 14 89                 	leaq	(%r9,%r9,4), %r10
14031bf32: 49 c1 e2 03                 	shlq	$0x3, %r10
14031bf36: 4c 03 51 08                 	addq	0x8(%rcx), %r10
14031bf3a: 4f 8d 1c 80                 	leaq	(%r8,%r8,4), %r11
14031bf3e: 49 c1 e3 03                 	shlq	$0x3, %r11
14031bf42: 4c 03 59 28                 	addq	0x28(%rcx), %r11
14031bf46: 48 8b 79 10                 	movq	0x10(%rcx), %rdi
14031bf4a: 4c 29 cf                    	subq	%r9, %rdi
14031bf4d: 48 0f 42 fa                 	cmovbq	%rdx, %rdi
14031bf51: 4c 8b 49 30                 	movq	0x30(%rcx), %r9
14031bf55: be 08 00 00 00              	movl	$0x8, %esi
14031bf5a: 4c 0f 42 d6                 	cmovbq	%rsi, %r10
14031bf5e: 48 8d 1c bf                 	leaq	(%rdi,%rdi,4), %rbx
14031bf62: 41 0f 96 c6                 	setbe	%r14b
14031bf66: 4d 8d 54 da d8              	leaq	-0x28(%r10,%rbx,8), %r10
14031bf6b: 4c 0f 46 d2                 	cmovbeq	%rdx, %r10
14031bf6f: 4d 29 c1                    	subq	%r8, %r9
14031bf72: 4c 0f 42 ca                 	cmovbq	%rdx, %r9
14031bf76: 4c 0f 42 de                 	cmovbq	%rsi, %r11
14031bf7a: 4f 8d 04 89                 	leaq	(%r9,%r9,4), %r8
14031bf7e: 0f 96 c3                    	setbe	%bl
14031bf81: 4b 8d 74 c3 d8              	leaq	-0x28(%r11,%r8,8), %rsi
14031bf86: 49 0f 46 f2                 	cmovbeq	%r10, %rsi
14031bf8a: 49 01 f9                    	addq	%rdi, %r9
14031bf8d: 41 84 de                    	testb	%bl, %r14b
14031bf90: 4c 0f 44 ca                 	cmoveq	%rdx, %r9
14031bf94: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
14031bf98: 4c 29 c9                    	subq	%r9, %rcx
14031bf9b: 48 89 08                    	movq	%rcx, (%rax)
14031bf9e: 41 84 de                    	testb	%bl, %r14b
14031bfa1: 0f 85 3f 02 00 00           	jne	0x14031c1e6 <.text+0x31b1e6>
14031bfa7: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031bfae: 48 89 f1                    	movq	%rsi, %rcx
14031bfb1: e8 1a af a5 00              	callq	0x140d76ed0 <.text+0xd75ed0>
14031bfb6: 49 89 d0                    	movq	%rdx, %r8
14031bfb9: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031bfc0: 48 c7 44 24 20 0e 00 00 00  	movq	$0xe, 0x20(%rsp)
14031bfc9: 4c 8d 0d c0 08 6a 02        	leaq	0x26a08c0(%rip), %r9    # 0x1429bc890
14031bfd0: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
14031bfd4: 48 89 c2                    	movq	%rax, %rdx
14031bfd7: e8 34 68 a4 00              	callq	0x140d62810 <.text+0xd61810>
14031bfdc: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
14031bfe0: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
14031bfe4: 0f 11 8d 90 00 00 00        	movups	%xmm1, 0x90(%rbp)
14031bfeb: 0f 11 85 80 00 00 00        	movups	%xmm0, 0x80(%rbp)
14031bff2: c6 85 b4 00 00 00 02        	movb	$0x2, 0xb4(%rbp)
14031bff9: c7 85 a8 00 00 00 01 00 00 00       	movl	$0x1, 0xa8(%rbp)
14031c003: 48 b8 00 02 00 00 00 02 00 00       	movabsq	$0x20000000200, %rax # imm = 0x20000000200
14031c00d: 48 89 85 ac 00 00 00        	movq	%rax, 0xac(%rbp)
14031c014: c6 85 a0 00 00 00 55        	movb	$0x55, 0xa0(%rbp)
14031c01b: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14031c023: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
14031c027: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
14031c02b: 48 8b bd 20 01 00 00        	movq	0x120(%rbp), %rdi
14031c032: 48 3b bd 10 01 00 00        	cmpq	0x110(%rbp), %rdi
14031c039: 75 13                       	jne	0x14031c04e <.text+0x31b04e>
14031c03b: 48 8d 15 5e 08 6a 02        	leaq	0x26a085e(%rip), %rdx   # 0x1429bc8a0
14031c042: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14031c049: e8 f2 74 24 00              	callq	0x140563540 <.text+0x562540>
14031c04e: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
14031c055: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
14031c059: 48 c1 e1 04                 	shlq	$0x4, %rcx
14031c05d: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031c061: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031c065: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031c06c: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031c073: 0f 10 a5 a8 00 00 00        	movups	0xa8(%rbp), %xmm4
14031c07a: 0f 11 64 08 40              	movups	%xmm4, 0x40(%rax,%rcx)
14031c07f: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
14031c084: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
14031c089: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
14031c08e: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
14031c092: 48 ff c7                    	incq	%rdi
14031c095: 48 89 bd 20 01 00 00        	movq	%rdi, 0x120(%rbp)
14031c09c: 49 8b 04 24                 	movq	(%r12), %rax
14031c0a0: 8b 08                       	movl	(%rax), %ecx
14031c0a2: 0f 57 c0                    	xorps	%xmm0, %xmm0
14031c0a5: f3 48 0f 2a c1              	cvtsi2ss	%rcx, %xmm0
14031c0aa: f3 0f 59 05 7e 03 6a 02     	mulss	0x26a037e(%rip), %xmm0  # 0x1429bc430
14031c0b2: 8b 40 04                    	movl	0x4(%rax), %eax
14031c0b5: 83 f8 01                    	cmpl	$0x1, %eax
14031c0b8: 83 d0 00                    	adcl	$0x0, %eax
14031c0bb: 0f 57 c9                    	xorps	%xmm1, %xmm1
14031c0be: f3 48 0f 2a c8              	cvtsi2ss	%rax, %xmm1
14031c0c3: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14031c0c7: f3 0f 5f 05 35 98 60 02     	maxss	0x2609835(%rip), %xmm0  # 0x142925904
14031c0cf: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
14031c0d4: 31 c9                       	xorl	%ecx, %ecx
14031c0d6: 0f 57 c9                    	xorps	%xmm1, %xmm1
14031c0d9: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14031c0dc: 0f 43 c8                    	cmovael	%eax, %ecx
14031c0df: 0f 2e 05 ea 46 68 02        	ucomiss	0x26846ea(%rip), %xmm0  # 0x1429a07d0
14031c0e6: bf ff ff ff ff              	movl	$0xffffffff, %edi       # imm = 0xFFFFFFFF
14031c0eb: 0f 46 f9                    	cmovbel	%ecx, %edi
14031c0ee: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c0f5: 48 89 f1                    	movq	%rsi, %rcx
14031c0f8: e8 d3 ad a5 00              	callq	0x140d76ed0 <.text+0xd75ed0>
14031c0fd: 49 89 d0                    	movq	%rdx, %r8
14031c100: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c107: 48 c7 44 24 20 16 00 00 00  	movq	$0x16, 0x20(%rsp)
14031c110: 4c 8d 0d a1 07 6a 02        	leaq	0x26a07a1(%rip), %r9    # 0x1429bc8b8
14031c117: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
14031c11b: 48 89 c2                    	movq	%rax, %rdx
14031c11e: e8 ed 66 a4 00              	callq	0x140d62810 <.text+0xd61810>
14031c123: 48 b8 00 00 00 00 40 00 00 00       	movabsq	$0x4000000000, %rax # imm = 0x4000000000
14031c12d: 48 09 f8                    	orq	%rdi, %rax
14031c130: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
14031c134: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
14031c138: 0f 11 8d 90 00 00 00        	movups	%xmm1, 0x90(%rbp)
14031c13f: 0f 11 85 80 00 00 00        	movups	%xmm0, 0x80(%rbp)
14031c146: c6 85 b4 00 00 00 02        	movb	$0x2, 0xb4(%rbp)
14031c14d: c7 85 a8 00 00 00 01 00 00 00       	movl	$0x1, 0xa8(%rbp)
14031c157: 48 89 85 ac 00 00 00        	movq	%rax, 0xac(%rbp)
14031c15e: c6 85 a0 00 00 00 5f        	movb	$0x5f, 0xa0(%rbp)
14031c165: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14031c16d: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
14031c171: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
14031c175: 48 8b b5 20 01 00 00        	movq	0x120(%rbp), %rsi
14031c17c: 48 3b b5 10 01 00 00        	cmpq	0x110(%rbp), %rsi
14031c183: 75 13                       	jne	0x14031c198 <.text+0x31b198>
14031c185: 48 8d 15 44 07 6a 02        	leaq	0x26a0744(%rip), %rdx   # 0x1429bc8d0
14031c18c: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14031c193: e8 a8 73 24 00              	callq	0x140563540 <.text+0x562540>
14031c198: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
14031c19f: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
14031c1a3: 48 c1 e1 04                 	shlq	$0x4, %rcx
14031c1a7: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031c1ab: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031c1af: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
14031c1b6: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
14031c1bd: 0f 10 a5 a8 00 00 00        	movups	0xa8(%rbp), %xmm4
14031c1c4: 0f 11 64 08 40              	movups	%xmm4, 0x40(%rax,%rcx)
14031c1c9: 0f 11 5c 08 30              	movups	%xmm3, 0x30(%rax,%rcx)
14031c1ce: 0f 11 54 08 20              	movups	%xmm2, 0x20(%rax,%rcx)
14031c1d3: 0f 11 4c 08 10              	movups	%xmm1, 0x10(%rax,%rcx)
14031c1d8: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
14031c1dc: 48 ff c6                    	incq	%rsi
14031c1df: 48 89 b5 20 01 00 00        	movq	%rsi, 0x120(%rbp)
14031c1e6: b1 01                       	movb	$0x1, %cl
14031c1e8: 31 c0                       	xorl	%eax, %eax
14031c1ea: f0                          	lock
14031c1eb: 0f b0 0d 5e ee 30 03        	cmpxchgb	%cl, 0x330ee5e(%rip)    # 0x14362b050
14031c1f2: 0f 85 ee 02 00 00           	jne	0x14031c4e6 <.text+0x31b4e6>
14031c1f8: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c1ff: 48 8d 0d 4a ee 30 03        	leaq	0x330ee4a(%rip), %rcx   # 0x14362b050
14031c206: e8 65 5a 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031c20b: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
14031c212: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
14031c219: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
14031c220: 4b 8d 14 80                 	leaq	(%r8,%r8,4), %rdx
14031c224: 48 c1 e2 04                 	shlq	$0x4, %rdx
14031c228: 48 01 ca                    	addq	%rcx, %rdx
14031c22b: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
14031c22f: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
14031c233: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
14031c237: 48 89 95 80 00 00 00        	movq	%rdx, 0x80(%rbp)
14031c23e: 48 8b 15 23 ee 30 03        	movq	0x330ee23(%rip), %rdx   # 0x14362b068
14031c245: 48 8b 05 0c ee 30 03        	movq	0x330ee0c(%rip), %rax   # 0x14362b058
14031c24c: 48 29 d0                    	subq	%rdx, %rax
14031c24f: 4c 39 c0                    	cmpq	%r8, %rax
14031c252: 0f 82 ac 02 00 00           	jb	0x14031c504 <.text+0x31b504>
14031c258: 48 8b 05 01 ee 30 03        	movq	0x330ee01(%rip), %rax   # 0x14362b060
14031c25f: 48 8d 0d 02 ee 30 03        	leaq	0x330ee02(%rip), %rcx   # 0x14362b068
14031c266: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
14031c26a: 48 89 55 50                 	movq	%rdx, 0x50(%rbp)
14031c26e: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14031c272: c6 85 2e 01 00 00 00        	movb	$0x0, 0x12e(%rbp)
14031c279: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031c27d: 48 8d 55 48                 	leaq	0x48(%rbp), %rdx
14031c281: e8 ea 18 17 00              	callq	0x14048db70 <.text+0x48cb70>
14031c286: 4c 8b b5 e8 01 00 00        	movq	0x1e8(%rbp), %r14
14031c28d: 49 8b 36                    	movq	(%r14), %rsi
14031c290: 48 89 f1                    	movq	%rsi, %rcx
14031c293: e8 28 2e 60 00              	callq	0x14091f0c0 <.text+0x91e0c0>
14031c298: 48 85 c0                    	testq	%rax, %rax
14031c29b: 0f 84 a2 01 00 00           	je	0x14031c443 <.text+0x31b443>
14031c2a1: 48 b9 09 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff7, %rcx # imm = 0x8000000000000009
14031c2ab: 48 39 08                    	cmpq	%rcx, (%rax)
14031c2ae: 0f 85 8f 01 00 00           	jne	0x14031c443 <.text+0x31b443>
14031c2b4: 4c 8b 60 18                 	movq	0x18(%rax), %r12
14031c2b8: 4d 85 e4                    	testq	%r12, %r12
14031c2bb: 0f 88 69 03 00 00           	js	0x14031c62a <.text+0x31b62a>
14031c2c1: 49 89 c7                    	movq	%rax, %r15
14031c2c4: 48 8b 78 10                 	movq	0x10(%rax), %rdi
14031c2c8: 74 23                       	je	0x14031c2ed <.text+0x31b2ed>
14031c2ca: 0f b6 05 38 89 31 03        	movzbl	0x3318938(%rip), %eax   # 0x143634c09
14031c2d1: bb 01 00 00 00              	movl	$0x1, %ebx
14031c2d6: ba 01 00 00 00              	movl	$0x1, %edx
14031c2db: 4c 89 e1                    	movq	%r12, %rcx
14031c2de: e8 2d 79 2f 00              	callq	0x140613c10 <.text+0x612c10>
14031c2e3: 48 85 c0                    	testq	%rax, %rax
14031c2e6: 75 0a                       	jne	0x14031c2f2 <.text+0x31b2f2>
14031c2e8: e9 3f 03 00 00              	jmp	0x14031c62c <.text+0x31b62c>
14031c2ed: b8 01 00 00 00              	movl	$0x1, %eax
14031c2f2: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14031c2f9: 48 89 c1                    	movq	%rax, %rcx
14031c2fc: 48 89 fa                    	movq	%rdi, %rdx
14031c2ff: 4c 89 a5 08 01 00 00        	movq	%r12, 0x108(%rbp)
14031c306: 4d 89 e0                    	movq	%r12, %r8
14031c309: e8 f2 78 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031c30e: 41 8a 5f 20                 	movb	0x20(%r15), %bl
14031c312: 41 8b 46 1c                 	movl	0x1c(%r14), %eax
14031c316: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
14031c31a: 89 01                       	movl	%eax, (%rcx)
14031c31c: c6 85 39 01 00 00 01        	movb	$0x1, 0x139(%rbp)
14031c323: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031c327: 48 89 f2                    	movq	%rsi, %rdx
14031c32a: e8 d1 2d 60 00              	callq	0x14091f100 <.text+0x91e100>
14031c32f: 48 8b 55 68                 	movq	0x68(%rbp), %rdx
14031c333: 48 b8 14 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffec, %rax # imm = 0x8000000000000014
14031c33d: 48 39 c2                    	cmpq	%rax, %rdx
14031c340: 74 4b                       	je	0x14031c38d <.text+0x31b38d>
14031c342: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14031c34c: 48 31 d0                    	xorq	%rdx, %rax
14031c34f: 48 83 f8 14                 	cmpq	$0x14, %rax
14031c353: b9 0d 00 00 00              	movl	$0xd, %ecx
14031c358: 48 0f 42 c8                 	cmovbq	%rax, %rcx
14031c35c: 48 83 f9 0f                 	cmpq	$0xf, %rcx
14031c360: 77 2b                       	ja	0x14031c38d <.text+0x31b38d>
14031c362: 48 8d 45 78                 	leaq	0x78(%rbp), %rax
14031c366: 4c 8d 05 ab 08 6a 02        	leaq	0x26a08ab(%rip), %r8    # 0x1429bcc18
14031c36d: 49 63 0c 88                 	movslq	(%r8,%rcx,4), %rcx
14031c371: 4c 01 c1                    	addq	%r8, %rcx
14031c374: ff e1                       	jmpq	*%rcx
14031c376: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
14031c37a: 48 85 d2                    	testq	%rdx, %rdx
14031c37d: 74 0e                       	je	0x14031c38d <.text+0x31b38d>
14031c37f: 48 8b 08                    	movq	(%rax), %rcx
14031c382: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031c388: e8 93 78 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031c38d: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14031c394: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
14031c39b: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031c3a2: 48 89 8d 88 00 00 00        	movq	%rcx, 0x88(%rbp)
14031c3a9: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14031c3b0: 88 9d 98 00 00 00           	movb	%bl, 0x98(%rbp)
14031c3b6: 8b 45 48                    	movl	0x48(%rbp), %eax
14031c3b9: 8b 4d 4b                    	movl	0x4b(%rbp), %ecx
14031c3bc: 89 85 99 00 00 00           	movl	%eax, 0x99(%rbp)
14031c3c2: 89 8d 9c 00 00 00           	movl	%ecx, 0x9c(%rbp)
14031c3c8: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14031c3d0: c6 85 a0 00 00 00 55        	movb	$0x55, 0xa0(%rbp)
14031c3d7: c7 85 a8 00 00 00 00 00 00 00       	movl	$0x0, 0xa8(%rbp)
14031c3e1: c6 85 b4 00 00 00 04        	movb	$0x4, 0xb4(%rbp)
14031c3e8: c7 85 b8 00 00 00 01 00 00 00       	movl	$0x1, 0xb8(%rbp)
14031c3f2: c6 85 fe 00 00 00 06        	movb	$0x6, 0xfe(%rbp)
14031c3f9: 48 8b 35 68 ec 30 03        	movq	0x330ec68(%rip), %rsi   # 0x14362b068
14031c400: 48 3b 35 51 ec 30 03        	cmpq	0x330ec51(%rip), %rsi   # 0x14362b058
14031c407: 75 13                       	jne	0x14031c41c <.text+0x31b41c>
14031c409: 48 8d 0d 48 ec 30 03        	leaq	0x330ec48(%rip), %rcx   # 0x14362b058
14031c410: 48 8d 15 d1 04 6a 02        	leaq	0x26a04d1(%rip), %rdx   # 0x1429bc8e8
14031c417: e8 d4 6b 24 00              	callq	0x140562ff0 <.text+0x561ff0>
14031c41c: 48 69 ce 98 00 00 00        	imulq	$0x98, %rsi, %rcx
14031c423: 48 03 0d 36 ec 30 03        	addq	0x330ec36(%rip), %rcx   # 0x14362b060
14031c42a: 48 8d 55 68                 	leaq	0x68(%rbp), %rdx
14031c42e: 41 b8 98 00 00 00           	movl	$0x98, %r8d
14031c434: e8 d7 77 60 02              	callq	0x142923c10 <.text+0x2922c10>
14031c439: 48 ff c6                    	incq	%rsi
14031c43c: 48 89 35 25 ec 30 03        	movq	%rsi, 0x330ec25(%rip)   # 0x14362b068
14031c443: c6 85 3f 01 00 00 00        	movb	$0x0, 0x13f(%rbp)
14031c44a: 48 8d 0d ff eb 30 03        	leaq	0x330ebff(%rip), %rcx   # 0x14362b050
14031c451: e8 ba 59 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031c456: 31 c9                       	xorl	%ecx, %ecx
14031c458: b0 01                       	movb	$0x1, %al
14031c45a: f0                          	lock
14031c45b: 0f b0 0d ee eb 30 03        	cmpxchgb	%cl, 0x330ebee(%rip)    # 0x14362b050
14031c462: 0f 85 ca 00 00 00           	jne	0x14031c532 <.text+0x31b532>
14031c468: 48 81 c4 c8 01 00 00        	addq	$0x1c8, %rsp            # imm = 0x1C8
14031c46f: 5b                          	popq	%rbx
14031c470: 5f                          	popq	%rdi
14031c471: 5e                          	popq	%rsi
14031c472: 41 5c                       	popq	%r12
14031c474: 41 5d                       	popq	%r13
14031c476: 41 5e                       	popq	%r14
14031c478: 41 5f                       	popq	%r15
14031c47a: 5d                          	popq	%rbp
14031c47b: c3                          	retq
14031c47c: 48 85 d2                    	testq	%rdx, %rdx
14031c47f: 74 0f                       	je	0x14031c490 <.text+0x31b490>
14031c481: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14031c485: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031c48b: e8 90 77 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031c490: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
14031c497: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
14031c49f: 48 85 c0                    	testq	%rax, %rax
14031c4a2: 0f 84 e5 fe ff ff           	je	0x14031c38d <.text+0x31b38d>
14031c4a8: 48 8d 85 88 00 00 00        	leaq	0x88(%rbp), %rax
14031c4af: e9 cb fe ff ff              	jmp	0x14031c37f <.text+0x31b37f>
14031c4b4: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
14031c4bb: 48 85 c0                    	testq	%rax, %rax
14031c4be: 0f 84 c9 fe ff ff           	je	0x14031c38d <.text+0x31b38d>
14031c4c4: f0                          	lock
14031c4c5: 48 ff 08                    	decq	(%rax)
14031c4c8: 0f 85 bf fe ff ff           	jne	0x14031c38d <.text+0x31b38d>
14031c4ce: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14031c4d5: c6 85 39 01 00 00 01        	movb	$0x1, 0x139(%rbp)
14031c4dc: e8 9f 98 fc ff              	callq	0x1402e5d80 <.text+0x2e4d80>
14031c4e1: e9 a7 fe ff ff              	jmp	0x14031c38d <.text+0x31b38d>
14031c4e6: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c4ed: 48 8d 0d 5c eb 30 03        	leaq	0x330eb5c(%rip), %rcx   # 0x14362b050
14031c4f4: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14031c4fa: e8 11 0d 5b 02              	callq	0x1428cd210 <.text+0x28cc210>
14031c4ff: e9 f4 fc ff ff              	jmp	0x14031c1f8 <.text+0x31b1f8>
14031c504: c6 85 2e 01 00 00 01        	movb	$0x1, 0x12e(%rbp)
14031c50b: 48 c7 44 24 20 98 00 00 00  	movq	$0x98, 0x20(%rsp)
14031c514: 48 8d 0d 3d eb 30 03        	leaq	0x330eb3d(%rip), %rcx   # 0x14362b058
14031c51b: 41 b9 08 00 00 00           	movl	$0x8, %r9d
14031c521: e8 ba 64 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031c526: 48 8b 15 3b eb 30 03        	movq	0x330eb3b(%rip), %rdx   # 0x14362b068
14031c52d: e9 26 fd ff ff              	jmp	0x14031c258 <.text+0x31b258>
14031c532: c6 85 3f 01 00 00 00        	movb	$0x0, 0x13f(%rbp)
14031c539: 48 8d 0d 10 eb 30 03        	leaq	0x330eb10(%rip), %rcx   # 0x14362b050
14031c540: 31 d2                       	xorl	%edx, %edx
14031c542: e8 49 16 5b 02              	callq	0x1428cdb90 <.text+0x28ccb90>
14031c547: e9 1c ff ff ff              	jmp	0x14031c468 <.text+0x31b468>
14031c54c: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c553: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14031c559: e8 92 f0 5a 02              	callq	0x1428cb5f0 <.text+0x28ca5f0>
14031c55e: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031c565: e9 51 ea ff ff              	jmp	0x14031afbb <.text+0x319fbb>
14031c56a: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c571: 48 89 f1                    	movq	%rsi, %rcx
14031c574: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14031c57a: e8 71 f0 5a 02              	callq	0x1428cb5f0 <.text+0x28ca5f0>
14031c57f: e9 70 ec ff ff              	jmp	0x14031b1f4 <.text+0x31a1f4>
14031c584: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c58b: 48 89 f1                    	movq	%rsi, %rcx
14031c58e: 31 d2                       	xorl	%edx, %edx
14031c590: e8 cb f9 5a 02              	callq	0x1428cbf60 <.text+0x28caf60>
14031c595: e9 cc ec ff ff              	jmp	0x14031b266 <.text+0x31a266>
14031c59a: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c5a1: 31 d2                       	xorl	%edx, %edx
14031c5a3: e8 b8 f9 5a 02              	callq	0x1428cbf60 <.text+0x28caf60>
14031c5a8: e9 9b ea ff ff              	jmp	0x14031b048 <.text+0x31a048>
14031c5ad: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c5b4: 31 d2                       	xorl	%edx, %edx
14031c5b6: e8 a5 f9 5a 02              	callq	0x1428cbf60 <.text+0x28caf60>
14031c5bb: e9 0f ec ff ff              	jmp	0x14031b1cf <.text+0x31a1cf>
14031c5c0: c6 85 3e 01 00 00 01        	movb	$0x1, 0x13e(%rbp)
14031c5c7: 48 89 f9                    	movq	%rdi, %rcx
14031c5ca: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14031c5d0: e8 1b f0 5a 02              	callq	0x1428cb5f0 <.text+0x28ca5f0>
14031c5d5: e9 b2 ee ff ff              	jmp	0x14031b48c <.text+0x31a48c>
14031c5da: c6 85 3e 01 00 00 01        	movb	$0x1, 0x13e(%rbp)
14031c5e1: 48 89 f9                    	movq	%rdi, %rcx
14031c5e4: 31 d2                       	xorl	%edx, %edx
14031c5e6: e8 75 f9 5a 02              	callq	0x1428cbf60 <.text+0x28caf60>
14031c5eb: e9 4a f1 ff ff              	jmp	0x14031b73a <.text+0x31a73a>
14031c5f0: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c5f7: b9 08 00 00 00              	movl	$0x8, %ecx
14031c5fc: ba 30 00 00 00              	movl	$0x30, %edx
14031c601: e8 fa c9 5b 02              	callq	0x1428d9000 <.text+0x28d8000>
14031c606: eb 75                       	jmp	0x14031c67d <.text+0x31b67d>
14031c608: b9 08 00 00 00              	movl	$0x8, %ecx
14031c60d: ba 50 00 00 00              	movl	$0x50, %edx
14031c612: e8 e9 c9 5b 02              	callq	0x1428d9000 <.text+0x28d8000>
14031c617: eb 64                       	jmp	0x14031c67d <.text+0x31b67d>
14031c619: b9 08 00 00 00              	movl	$0x8, %ecx
14031c61e: ba 50 00 00 00              	movl	$0x50, %edx
14031c623: e8 d8 c9 5b 02              	callq	0x1428d9000 <.text+0x28d8000>
14031c628: eb 53                       	jmp	0x14031c67d <.text+0x31b67d>
14031c62a: 31 db                       	xorl	%ebx, %ebx
14031c62c: 4c 8d 05 05 1e 69 02        	leaq	0x2691e05(%rip), %r8    # 0x1429ae438
14031c633: 48 89 d9                    	movq	%rbx, %rcx
14031c636: 4c 89 e2                    	movq	%r12, %rdx
14031c639: e8 a5 c9 5b 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14031c63e: eb 3d                       	jmp	0x14031c67d <.text+0x31b67d>
14031c640: b9 08 00 00 00              	movl	$0x8, %ecx
14031c645: ba 40 00 00 00              	movl	$0x40, %edx
14031c64a: e8 b1 c9 5b 02              	callq	0x1428d9000 <.text+0x28d8000>
14031c64f: eb 2c                       	jmp	0x14031c67d <.text+0x31b67d>
14031c651: 31 f6                       	xorl	%esi, %esi
14031c653: c6 85 3c 01 00 00 01        	movb	$0x1, 0x13c(%rbp)
14031c65a: 4c 8d 05 d7 1d 69 02        	leaq	0x2691dd7(%rip), %r8    # 0x1429ae438
14031c661: 48 89 f1                    	movq	%rsi, %rcx
14031c664: 4c 89 fa                    	movq	%r15, %rdx
14031c667: e8 77 c9 5b 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14031c66c: eb 0f                       	jmp	0x14031c67d <.text+0x31b67d>
14031c66e: b9 08 00 00 00              	movl	$0x8, %ecx
14031c673: ba 28 00 00 00              	movl	$0x28, %edx
14031c678: e8 83 c9 5b 02              	callq	0x1428d9000 <.text+0x28d8000>
14031c67d: 0f 0b                       	ud2
14031c67f: 90                          	nop
14031c680: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c685: 55                          	pushq	%rbp
14031c686: 41 57                       	pushq	%r15
14031c688: 41 56                       	pushq	%r14
14031c68a: 41 55                       	pushq	%r13
14031c68c: 41 54                       	pushq	%r12
14031c68e: 56                          	pushq	%rsi
14031c68f: 57                          	pushq	%rdi
14031c690: 53                          	pushq	%rbx
14031c691: 48 83 ec 28                 	subq	$0x28, %rsp
14031c695: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c69c: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
14031c6a0: 48 85 d2                    	testq	%rdx, %rdx
14031c6a3: 74 0f                       	je	0x14031c6b4 <.text+0x31b6b4>
14031c6a5: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
14031c6a9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031c6af: e8 6c 75 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031c6b4: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c6bb: 48 83 c4 28                 	addq	$0x28, %rsp
14031c6bf: 5b                          	popq	%rbx
14031c6c0: 5f                          	popq	%rdi
14031c6c1: 5e                          	popq	%rsi
14031c6c2: 41 5c                       	popq	%r12
14031c6c4: 41 5d                       	popq	%r13
14031c6c6: 41 5e                       	popq	%r14
14031c6c8: 41 5f                       	popq	%r15
14031c6ca: 5d                          	popq	%rbp
14031c6cb: c3                          	retq
14031c6cc: 0f 1f 40 00                 	nopl	(%rax)
14031c6d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c6d5: 55                          	pushq	%rbp
14031c6d6: 41 57                       	pushq	%r15
14031c6d8: 41 56                       	pushq	%r14
14031c6da: 41 55                       	pushq	%r13
14031c6dc: 41 54                       	pushq	%r12
14031c6de: 56                          	pushq	%rsi
14031c6df: 57                          	pushq	%rdi
14031c6e0: 53                          	pushq	%rbx
14031c6e1: 48 83 ec 28                 	subq	$0x28, %rsp
14031c6e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c6ec: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
14031c6f0: 48 85 d2                    	testq	%rdx, %rdx
14031c6f3: 74 0f                       	je	0x14031c704 <.text+0x31b704>
14031c6f5: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
14031c6f9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031c6ff: e8 1c 75 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031c704: 90                          	nop
14031c705: 48 83 c4 28                 	addq	$0x28, %rsp
14031c709: 5b                          	popq	%rbx
14031c70a: 5f                          	popq	%rdi
14031c70b: 5e                          	popq	%rsi
14031c70c: 41 5c                       	popq	%r12
14031c70e: 41 5d                       	popq	%r13
14031c710: 41 5e                       	popq	%r14
14031c712: 41 5f                       	popq	%r15
14031c714: 5d                          	popq	%rbp
14031c715: c3                          	retq
14031c716: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14031c720: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c725: 55                          	pushq	%rbp
14031c726: 41 57                       	pushq	%r15
14031c728: 41 56                       	pushq	%r14
14031c72a: 41 55                       	pushq	%r13
14031c72c: 41 54                       	pushq	%r12
14031c72e: 56                          	pushq	%rsi
14031c72f: 57                          	pushq	%rdi
14031c730: 53                          	pushq	%rbx
14031c731: 48 83 ec 28                 	subq	$0x28, %rsp
14031c735: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c73c: 48 83 bd 30 01 00 00 00     	cmpq	$0x0, 0x130(%rbp)
14031c744: 74 16                       	je	0x14031c75c <.text+0x31b75c>
14031c746: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031c74c: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
14031c750: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
14031c757: e8 c4 74 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031c75c: 90                          	nop
14031c75d: 48 83 c4 28                 	addq	$0x28, %rsp
14031c761: 5b                          	popq	%rbx
14031c762: 5f                          	popq	%rdi
14031c763: 5e                          	popq	%rsi
14031c764: 41 5c                       	popq	%r12
14031c766: 41 5d                       	popq	%r13
14031c768: 41 5e                       	popq	%r14
14031c76a: 41 5f                       	popq	%r15
14031c76c: 5d                          	popq	%rbp
14031c76d: c3                          	retq
14031c76e: 66 90                       	nop
14031c770: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c775: 55                          	pushq	%rbp
14031c776: 41 57                       	pushq	%r15
14031c778: 41 56                       	pushq	%r14
14031c77a: 41 55                       	pushq	%r13
14031c77c: 41 54                       	pushq	%r12
14031c77e: 56                          	pushq	%rsi
14031c77f: 57                          	pushq	%rdi
14031c780: 53                          	pushq	%rbx
14031c781: 48 83 ec 28                 	subq	$0x28, %rsp
14031c785: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c78c: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
14031c790: e8 3b 4b d2 ff              	callq	0x1400412d0 <.text+0x402d0>
14031c795: c6 85 3e 01 00 00 01        	movb	$0x1, 0x13e(%rbp)
14031c79c: 48 83 c4 28                 	addq	$0x28, %rsp
14031c7a0: 5b                          	popq	%rbx
14031c7a1: 5f                          	popq	%rdi
14031c7a2: 5e                          	popq	%rsi
14031c7a3: 41 5c                       	popq	%r12
14031c7a5: 41 5d                       	popq	%r13
14031c7a7: 41 5e                       	popq	%r14
14031c7a9: 41 5f                       	popq	%r15
14031c7ab: 5d                          	popq	%rbp
14031c7ac: c3                          	retq
14031c7ad: 0f 1f 00                    	nopl	(%rax)
14031c7b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c7b5: 55                          	pushq	%rbp
14031c7b6: 41 57                       	pushq	%r15
14031c7b8: 41 56                       	pushq	%r14
14031c7ba: 41 55                       	pushq	%r13
14031c7bc: 41 54                       	pushq	%r12
14031c7be: 56                          	pushq	%rsi
14031c7bf: 57                          	pushq	%rdi
14031c7c0: 53                          	pushq	%rbx
14031c7c1: 48 83 ec 28                 	subq	$0x28, %rsp
14031c7c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c7cc: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
14031c7d3: f0                          	lock
14031c7d4: 48 ff 08                    	decq	(%rax)
14031c7d7: 75 10                       	jne	0x14031c7e9 <.text+0x31b7e9>
14031c7d9: c6 85 3c 01 00 00 01        	movb	$0x1, 0x13c(%rbp)
14031c7e0: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031c7e4: e8 b7 f8 83 00              	callq	0x140b5c0a0 <.text+0xb5b0a0>
14031c7e9: c6 85 3c 01 00 00 01        	movb	$0x1, 0x13c(%rbp)
14031c7f0: 48 83 c4 28                 	addq	$0x28, %rsp
14031c7f4: 5b                          	popq	%rbx
14031c7f5: 5f                          	popq	%rdi
14031c7f6: 5e                          	popq	%rsi
14031c7f7: 41 5c                       	popq	%r12
14031c7f9: 41 5d                       	popq	%r13
14031c7fb: 41 5e                       	popq	%r14
14031c7fd: 41 5f                       	popq	%r15
14031c7ff: 5d                          	popq	%rbp
14031c800: c3                          	retq
14031c801: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14031c810: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c815: 55                          	pushq	%rbp
14031c816: 41 57                       	pushq	%r15
14031c818: 41 56                       	pushq	%r14
14031c81a: 41 55                       	pushq	%r13
14031c81c: 41 54                       	pushq	%r12
14031c81e: 56                          	pushq	%rsi
14031c81f: 57                          	pushq	%rdi
14031c820: 53                          	pushq	%rbx
14031c821: 48 83 ec 28                 	subq	$0x28, %rsp
14031c825: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c82c: c6 85 3c 01 00 00 00        	movb	$0x0, 0x13c(%rbp)
14031c833: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031c837: e8 54 1b fb ff              	callq	0x1402ce390 <.text+0x2cd390>
14031c83c: c6 85 3c 01 00 00 00        	movb	$0x0, 0x13c(%rbp)
14031c843: 48 83 c4 28                 	addq	$0x28, %rsp
14031c847: 5b                          	popq	%rbx
14031c848: 5f                          	popq	%rdi
14031c849: 5e                          	popq	%rsi
14031c84a: 41 5c                       	popq	%r12
14031c84c: 41 5d                       	popq	%r13
14031c84e: 41 5e                       	popq	%r14
14031c850: 41 5f                       	popq	%r15
14031c852: 5d                          	popq	%rbp
14031c853: c3                          	retq
14031c854: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031c860: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c865: 55                          	pushq	%rbp
14031c866: 41 57                       	pushq	%r15
14031c868: 41 56                       	pushq	%r14
14031c86a: 41 55                       	pushq	%r13
14031c86c: 41 54                       	pushq	%r12
14031c86e: 56                          	pushq	%rsi
14031c86f: 57                          	pushq	%rdi
14031c870: 53                          	pushq	%rbx
14031c871: 48 83 ec 28                 	subq	$0x28, %rsp
14031c875: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c87c: 80 bd 3c 01 00 00 00        	cmpb	$0x0, 0x13c(%rbp)
14031c883: 74 25                       	je	0x14031c8aa <.text+0x31b8aa>
14031c885: 48 8b 45 18                 	movq	0x18(%rbp), %rax
14031c889: f0                          	lock
14031c88a: 48 ff 08                    	decq	(%rax)
14031c88d: 75 09                       	jne	0x14031c898 <.text+0x31b898>
14031c88f: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
14031c893: e8 08 f8 83 00              	callq	0x140b5c0a0 <.text+0xb5b0a0>
14031c898: 90                          	nop
14031c899: 48 83 c4 28                 	addq	$0x28, %rsp
14031c89d: 5b                          	popq	%rbx
14031c89e: 5f                          	popq	%rdi
14031c89f: 5e                          	popq	%rsi
14031c8a0: 41 5c                       	popq	%r12
14031c8a2: 41 5d                       	popq	%r13
14031c8a4: 41 5e                       	popq	%r14
14031c8a6: 41 5f                       	popq	%r15
14031c8a8: 5d                          	popq	%rbp
14031c8a9: c3                          	retq
14031c8aa: 48 83 c4 28                 	addq	$0x28, %rsp
14031c8ae: 5b                          	popq	%rbx
14031c8af: 5f                          	popq	%rdi
14031c8b0: 5e                          	popq	%rsi
14031c8b1: 41 5c                       	popq	%r12
14031c8b3: 41 5d                       	popq	%r13
14031c8b5: 41 5e                       	popq	%r14
14031c8b7: 41 5f                       	popq	%r15
14031c8b9: 5d                          	popq	%rbp
14031c8ba: c3                          	retq
14031c8bb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14031c8c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c8c5: 55                          	pushq	%rbp
14031c8c6: 41 57                       	pushq	%r15
14031c8c8: 41 56                       	pushq	%r14
14031c8ca: 41 55                       	pushq	%r13
14031c8cc: 41 54                       	pushq	%r12
14031c8ce: 56                          	pushq	%rsi
14031c8cf: 57                          	pushq	%rdi
14031c8d0: 53                          	pushq	%rbx
14031c8d1: 48 83 ec 28                 	subq	$0x28, %rsp
14031c8d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c8dc: c6 85 3e 01 00 00 00        	movb	$0x0, 0x13e(%rbp)
14031c8e3: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031c8e7: e8 f4 af 2d 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14031c8ec: c6 85 3e 01 00 00 00        	movb	$0x0, 0x13e(%rbp)
14031c8f3: 48 83 c4 28                 	addq	$0x28, %rsp
14031c8f7: 5b                          	popq	%rbx
14031c8f8: 5f                          	popq	%rdi
14031c8f9: 5e                          	popq	%rsi
14031c8fa: 41 5c                       	popq	%r12
14031c8fc: 41 5d                       	popq	%r13
14031c8fe: 41 5e                       	popq	%r14
14031c900: 41 5f                       	popq	%r15
14031c902: 5d                          	popq	%rbp
14031c903: c3                          	retq
14031c904: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031c910: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c915: 55                          	pushq	%rbp
14031c916: 41 57                       	pushq	%r15
14031c918: 41 56                       	pushq	%r14
14031c91a: 41 55                       	pushq	%r13
14031c91c: 41 54                       	pushq	%r12
14031c91e: 56                          	pushq	%rsi
14031c91f: 57                          	pushq	%rdi
14031c920: 53                          	pushq	%rbx
14031c921: 48 83 ec 28                 	subq	$0x28, %rsp
14031c925: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c92c: 80 bd 3e 01 00 00 00        	cmpb	$0x0, 0x13e(%rbp)
14031c933: 74 30                       	je	0x14031c965 <.text+0x31b965>
14031c935: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
14031c939: 48 85 d2                    	testq	%rdx, %rdx
14031c93c: 74 0f                       	je	0x14031c94d <.text+0x31b94d>
14031c93e: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
14031c942: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031c948: e8 d3 72 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031c94d: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c954: 48 83 c4 28                 	addq	$0x28, %rsp
14031c958: 5b                          	popq	%rbx
14031c959: 5f                          	popq	%rdi
14031c95a: 5e                          	popq	%rsi
14031c95b: 41 5c                       	popq	%r12
14031c95d: 41 5d                       	popq	%r13
14031c95f: 41 5e                       	popq	%r14
14031c961: 41 5f                       	popq	%r15
14031c963: 5d                          	popq	%rbp
14031c964: c3                          	retq
14031c965: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c96c: 48 83 c4 28                 	addq	$0x28, %rsp
14031c970: 5b                          	popq	%rbx
14031c971: 5f                          	popq	%rdi
14031c972: 5e                          	popq	%rsi
14031c973: 41 5c                       	popq	%r12
14031c975: 41 5d                       	popq	%r13
14031c977: 41 5e                       	popq	%r14
14031c979: 41 5f                       	popq	%r15
14031c97b: 5d                          	popq	%rbp
14031c97c: c3                          	retq
14031c97d: 0f 1f 00                    	nopl	(%rax)
14031c980: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c985: 55                          	pushq	%rbp
14031c986: 41 57                       	pushq	%r15
14031c988: 41 56                       	pushq	%r14
14031c98a: 41 55                       	pushq	%r13
14031c98c: 41 54                       	pushq	%r12
14031c98e: 56                          	pushq	%rsi
14031c98f: 57                          	pushq	%rdi
14031c990: 53                          	pushq	%rbx
14031c991: 48 83 ec 28                 	subq	$0x28, %rsp
14031c995: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c99c: c6 85 3d 01 00 00 01        	movb	$0x1, 0x13d(%rbp)
14031c9a3: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031c9a7: e8 54 3e 2c 00              	callq	0x1405e0800 <.text+0x5df800>
14031c9ac: c6 85 3d 01 00 00 01        	movb	$0x1, 0x13d(%rbp)
14031c9b3: 48 83 c4 28                 	addq	$0x28, %rsp
14031c9b7: 5b                          	popq	%rbx
14031c9b8: 5f                          	popq	%rdi
14031c9b9: 5e                          	popq	%rsi
14031c9ba: 41 5c                       	popq	%r12
14031c9bc: 41 5d                       	popq	%r13
14031c9be: 41 5e                       	popq	%r14
14031c9c0: 41 5f                       	popq	%r15
14031c9c2: 5d                          	popq	%rbp
14031c9c3: c3                          	retq
14031c9c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031c9d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031c9d5: 55                          	pushq	%rbp
14031c9d6: 41 57                       	pushq	%r15
14031c9d8: 41 56                       	pushq	%r14
14031c9da: 41 55                       	pushq	%r13
14031c9dc: 41 54                       	pushq	%r12
14031c9de: 56                          	pushq	%rsi
14031c9df: 57                          	pushq	%rdi
14031c9e0: 53                          	pushq	%rbx
14031c9e1: 48 83 ec 28                 	subq	$0x28, %rsp
14031c9e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031c9ec: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031c9f3: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031c9f7: e8 04 3e 2c 00              	callq	0x1405e0800 <.text+0x5df800>
14031c9fc: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031ca03: 48 83 c4 28                 	addq	$0x28, %rsp
14031ca07: 5b                          	popq	%rbx
14031ca08: 5f                          	popq	%rdi
14031ca09: 5e                          	popq	%rsi
14031ca0a: 41 5c                       	popq	%r12
14031ca0c: 41 5d                       	popq	%r13
14031ca0e: 41 5e                       	popq	%r14
14031ca10: 41 5f                       	popq	%r15
14031ca12: 5d                          	popq	%rbp
14031ca13: c3                          	retq
14031ca14: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031ca20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031ca25: 55                          	pushq	%rbp
14031ca26: 41 57                       	pushq	%r15
14031ca28: 41 56                       	pushq	%r14
14031ca2a: 41 55                       	pushq	%r13
14031ca2c: 41 54                       	pushq	%r12
14031ca2e: 56                          	pushq	%rsi
14031ca2f: 57                          	pushq	%rdi
14031ca30: 53                          	pushq	%rbx
14031ca31: 48 83 ec 28                 	subq	$0x28, %rsp
14031ca35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031ca3c: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031ca43: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031ca47: e8 94 ae 2d 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14031ca4c: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031ca53: 48 83 c4 28                 	addq	$0x28, %rsp
14031ca57: 5b                          	popq	%rbx
14031ca58: 5f                          	popq	%rdi
14031ca59: 5e                          	popq	%rsi
14031ca5a: 41 5c                       	popq	%r12
14031ca5c: 41 5d                       	popq	%r13
14031ca5e: 41 5e                       	popq	%r14
14031ca60: 41 5f                       	popq	%r15
14031ca62: 5d                          	popq	%rbp
14031ca63: c3                          	retq
14031ca64: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031ca70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031ca75: 55                          	pushq	%rbp
14031ca76: 41 57                       	pushq	%r15
14031ca78: 41 56                       	pushq	%r14
14031ca7a: 41 55                       	pushq	%r13
14031ca7c: 41 54                       	pushq	%r12
14031ca7e: 56                          	pushq	%rsi
14031ca7f: 57                          	pushq	%rdi
14031ca80: 53                          	pushq	%rbx
14031ca81: 48 83 ec 28                 	subq	$0x28, %rsp
14031ca85: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031ca8c: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031ca93: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031ca97: e8 44 ae 2d 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14031ca9c: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031caa3: 48 83 c4 28                 	addq	$0x28, %rsp
14031caa7: 5b                          	popq	%rbx
14031caa8: 5f                          	popq	%rdi
14031caa9: 5e                          	popq	%rsi
14031caaa: 41 5c                       	popq	%r12
14031caac: 41 5d                       	popq	%r13
14031caae: 41 5e                       	popq	%r14
14031cab0: 41 5f                       	popq	%r15
14031cab2: 5d                          	popq	%rbp
14031cab3: c3                          	retq
14031cab4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031cac0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cac5: 55                          	pushq	%rbp
14031cac6: 41 57                       	pushq	%r15
14031cac8: 41 56                       	pushq	%r14
14031caca: 41 55                       	pushq	%r13
14031cacc: 41 54                       	pushq	%r12
14031cace: 56                          	pushq	%rsi
14031cacf: 57                          	pushq	%rdi
14031cad0: 53                          	pushq	%rbx
14031cad1: 48 83 ec 28                 	subq	$0x28, %rsp
14031cad5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cadc: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031cae3: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031cae7: e8 f4 ad 2d 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14031caec: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031caf3: 48 83 c4 28                 	addq	$0x28, %rsp
14031caf7: 5b                          	popq	%rbx
14031caf8: 5f                          	popq	%rdi
14031caf9: 5e                          	popq	%rsi
14031cafa: 41 5c                       	popq	%r12
14031cafc: 41 5d                       	popq	%r13
14031cafe: 41 5e                       	popq	%r14
14031cb00: 41 5f                       	popq	%r15
14031cb02: 5d                          	popq	%rbp
14031cb03: c3                          	retq
14031cb04: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031cb10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cb15: 55                          	pushq	%rbp
14031cb16: 41 57                       	pushq	%r15
14031cb18: 41 56                       	pushq	%r14
14031cb1a: 41 55                       	pushq	%r13
14031cb1c: 41 54                       	pushq	%r12
14031cb1e: 56                          	pushq	%rsi
14031cb1f: 57                          	pushq	%rdi
14031cb20: 53                          	pushq	%rbx
14031cb21: 48 83 ec 28                 	subq	$0x28, %rsp
14031cb25: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cb2c: 48 8d 0d 1d e5 30 03        	leaq	0x330e51d(%rip), %rcx   # 0x14362b050
14031cb33: e8 f8 50 e4 ff              	callq	0x140161c30 <.text+0x160c30>
14031cb38: c6 85 3f 01 00 00 00        	movb	$0x0, 0x13f(%rbp)
14031cb3f: 48 83 c4 28                 	addq	$0x28, %rsp
14031cb43: 5b                          	popq	%rbx
14031cb44: 5f                          	popq	%rdi
14031cb45: 5e                          	popq	%rsi
14031cb46: 41 5c                       	popq	%r12
14031cb48: 41 5d                       	popq	%r13
14031cb4a: 41 5e                       	popq	%r14
14031cb4c: 41 5f                       	popq	%r15
14031cb4e: 5d                          	popq	%rbp
14031cb4f: c3                          	retq
14031cb50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cb55: 55                          	pushq	%rbp
14031cb56: 41 57                       	pushq	%r15
14031cb58: 41 56                       	pushq	%r14
14031cb5a: 41 55                       	pushq	%r13
14031cb5c: 41 54                       	pushq	%r12
14031cb5e: 56                          	pushq	%rsi
14031cb5f: 57                          	pushq	%rdi
14031cb60: 53                          	pushq	%rbx
14031cb61: 48 83 ec 28                 	subq	$0x28, %rsp
14031cb65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cb6c: 80 bd 2e 01 00 00 00        	cmpb	$0x0, 0x12e(%rbp)
14031cb73: 74 09                       	je	0x14031cb7e <.text+0x31bb7e>
14031cb75: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031cb79: e8 b2 34 2b 00              	callq	0x1405d0030 <.text+0x5cf030>
14031cb7e: 90                          	nop
14031cb7f: 48 83 c4 28                 	addq	$0x28, %rsp
14031cb83: 5b                          	popq	%rbx
14031cb84: 5f                          	popq	%rdi
14031cb85: 5e                          	popq	%rsi
14031cb86: 41 5c                       	popq	%r12
14031cb88: 41 5d                       	popq	%r13
14031cb8a: 41 5e                       	popq	%r14
14031cb8c: 41 5f                       	popq	%r15
14031cb8e: 5d                          	popq	%rbp
14031cb8f: c3                          	retq
14031cb90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cb95: 55                          	pushq	%rbp
14031cb96: 41 57                       	pushq	%r15
14031cb98: 41 56                       	pushq	%r14
14031cb9a: 41 55                       	pushq	%r13
14031cb9c: 41 54                       	pushq	%r12
14031cb9e: 56                          	pushq	%rsi
14031cb9f: 57                          	pushq	%rdi
14031cba0: 53                          	pushq	%rbx
14031cba1: 48 83 ec 28                 	subq	$0x28, %rsp
14031cba5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cbac: 80 bd 39 01 00 00 00        	cmpb	$0x0, 0x139(%rbp)
14031cbb3: 74 35                       	je	0x14031cbea <.text+0x31bbea>
14031cbb5: 48 83 bd 08 01 00 00 00     	cmpq	$0x0, 0x108(%rbp)
14031cbbd: 74 19                       	je	0x14031cbd8 <.text+0x31bbd8>
14031cbbf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031cbc5: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031cbcc: 48 8b 95 08 01 00 00        	movq	0x108(%rbp), %rdx
14031cbd3: e8 48 70 2f 00              	callq	0x140613c20 <.text+0x612c20>
14031cbd8: 90                          	nop
14031cbd9: 48 83 c4 28                 	addq	$0x28, %rsp
14031cbdd: 5b                          	popq	%rbx
14031cbde: 5f                          	popq	%rdi
14031cbdf: 5e                          	popq	%rsi
14031cbe0: 41 5c                       	popq	%r12
14031cbe2: 41 5d                       	popq	%r13
14031cbe4: 41 5e                       	popq	%r14
14031cbe6: 41 5f                       	popq	%r15
14031cbe8: 5d                          	popq	%rbp
14031cbe9: c3                          	retq
14031cbea: 48 83 c4 28                 	addq	$0x28, %rsp
14031cbee: 5b                          	popq	%rbx
14031cbef: 5f                          	popq	%rdi
14031cbf0: 5e                          	popq	%rsi
14031cbf1: 41 5c                       	popq	%r12
14031cbf3: 41 5d                       	popq	%r13
14031cbf5: 41 5e                       	popq	%r14
14031cbf7: 41 5f                       	popq	%r15
14031cbf9: 5d                          	popq	%rbp
14031cbfa: c3                          	retq
14031cbfb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14031cc00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cc05: 55                          	pushq	%rbp
14031cc06: 41 57                       	pushq	%r15
14031cc08: 41 56                       	pushq	%r14
14031cc0a: 41 55                       	pushq	%r13
14031cc0c: 41 54                       	pushq	%r12
14031cc0e: 56                          	pushq	%rsi
14031cc0f: 57                          	pushq	%rdi
14031cc10: 53                          	pushq	%rbx
14031cc11: 48 83 ec 28                 	subq	$0x28, %rsp
14031cc15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cc1c: 80 bd 2f 01 00 00 00        	cmpb	$0x0, 0x12f(%rbp)
14031cc23: 74 2b                       	je	0x14031cc50 <.text+0x31bc50>
14031cc25: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14031cc29: f0                          	lock
14031cc2a: 48 ff 08                    	decq	(%rax)
14031cc2d: 75 09                       	jne	0x14031cc38 <.text+0x31bc38>
14031cc2f: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
14031cc33: e8 88 fc 83 00              	callq	0x140b5c8c0 <.text+0xb5b8c0>
14031cc38: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031cc3f: 48 83 c4 28                 	addq	$0x28, %rsp
14031cc43: 5b                          	popq	%rbx
14031cc44: 5f                          	popq	%rdi
14031cc45: 5e                          	popq	%rsi
14031cc46: 41 5c                       	popq	%r12
14031cc48: 41 5d                       	popq	%r13
14031cc4a: 41 5e                       	popq	%r14
14031cc4c: 41 5f                       	popq	%r15
14031cc4e: 5d                          	popq	%rbp
14031cc4f: c3                          	retq
14031cc50: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031cc57: 48 83 c4 28                 	addq	$0x28, %rsp
14031cc5b: 5b                          	popq	%rbx
14031cc5c: 5f                          	popq	%rdi
14031cc5d: 5e                          	popq	%rsi
14031cc5e: 41 5c                       	popq	%r12
14031cc60: 41 5d                       	popq	%r13
14031cc62: 41 5e                       	popq	%r14
14031cc64: 41 5f                       	popq	%r15
14031cc66: 5d                          	popq	%rbp
14031cc67: c3                          	retq
14031cc68: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14031cc70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cc75: 55                          	pushq	%rbp
14031cc76: 41 57                       	pushq	%r15
14031cc78: 41 56                       	pushq	%r14
14031cc7a: 41 55                       	pushq	%r13
14031cc7c: 41 54                       	pushq	%r12
14031cc7e: 56                          	pushq	%rsi
14031cc7f: 57                          	pushq	%rdi
14031cc80: 53                          	pushq	%rbx
14031cc81: 48 83 ec 28                 	subq	$0x28, %rsp
14031cc85: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cc8c: 0f b6 9d 3a 01 00 00        	movzbl	0x13a(%rbp), %ebx
14031cc93: 80 bd 3b 01 00 00 00        	cmpb	$0x0, 0x13b(%rbp)
14031cc9a: 74 2d                       	je	0x14031ccc9 <.text+0x31bcc9>
14031cc9c: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14031cca0: f0                          	lock
14031cca1: 48 ff 08                    	decq	(%rax)
14031cca4: 75 09                       	jne	0x14031ccaf <.text+0x31bcaf>
14031cca6: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
14031ccaa: e8 11 fc 83 00              	callq	0x140b5c8c0 <.text+0xb5b8c0>
14031ccaf: 80 e3 01                    	andb	$0x1, %bl
14031ccb2: 88 9d 3d 01 00 00           	movb	%bl, 0x13d(%rbp)
14031ccb8: 48 83 c4 28                 	addq	$0x28, %rsp
14031ccbc: 5b                          	popq	%rbx
14031ccbd: 5f                          	popq	%rdi
14031ccbe: 5e                          	popq	%rsi
14031ccbf: 41 5c                       	popq	%r12
14031ccc1: 41 5d                       	popq	%r13
14031ccc3: 41 5e                       	popq	%r14
14031ccc5: 41 5f                       	popq	%r15
14031ccc7: 5d                          	popq	%rbp
14031ccc8: c3                          	retq
14031ccc9: 80 e3 01                    	andb	$0x1, %bl
14031cccc: 88 9d 3d 01 00 00           	movb	%bl, 0x13d(%rbp)
14031ccd2: 48 83 c4 28                 	addq	$0x28, %rsp
14031ccd6: 5b                          	popq	%rbx
14031ccd7: 5f                          	popq	%rdi
14031ccd8: 5e                          	popq	%rsi
14031ccd9: 41 5c                       	popq	%r12
14031ccdb: 41 5d                       	popq	%r13
14031ccdd: 41 5e                       	popq	%r14
14031ccdf: 41 5f                       	popq	%r15
14031cce1: 5d                          	popq	%rbp
14031cce2: c3                          	retq
14031cce3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14031ccf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031ccf5: 55                          	pushq	%rbp
14031ccf6: 41 57                       	pushq	%r15
14031ccf8: 41 56                       	pushq	%r14
14031ccfa: 41 55                       	pushq	%r13
14031ccfc: 41 54                       	pushq	%r12
14031ccfe: 56                          	pushq	%rsi
14031ccff: 57                          	pushq	%rdi
14031cd00: 53                          	pushq	%rbx
14031cd01: 48 83 ec 28                 	subq	$0x28, %rsp
14031cd05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cd0c: 80 bd 3d 01 00 00 00        	cmpb	$0x0, 0x13d(%rbp)
14031cd13: 74 2b                       	je	0x14031cd40 <.text+0x31bd40>
14031cd15: 48 8b 45 18                 	movq	0x18(%rbp), %rax
14031cd19: f0                          	lock
14031cd1a: 48 ff 08                    	decq	(%rax)
14031cd1d: 75 09                       	jne	0x14031cd28 <.text+0x31bd28>
14031cd1f: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
14031cd23: e8 68 fc 83 00              	callq	0x140b5c990 <.text+0xb5b990>
14031cd28: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031cd2f: 48 83 c4 28                 	addq	$0x28, %rsp
14031cd33: 5b                          	popq	%rbx
14031cd34: 5f                          	popq	%rdi
14031cd35: 5e                          	popq	%rsi
14031cd36: 41 5c                       	popq	%r12
14031cd38: 41 5d                       	popq	%r13
14031cd3a: 41 5e                       	popq	%r14
14031cd3c: 41 5f                       	popq	%r15
14031cd3e: 5d                          	popq	%rbp
14031cd3f: c3                          	retq
14031cd40: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031cd47: 48 83 c4 28                 	addq	$0x28, %rsp
14031cd4b: 5b                          	popq	%rbx
14031cd4c: 5f                          	popq	%rdi
14031cd4d: 5e                          	popq	%rsi
14031cd4e: 41 5c                       	popq	%r12
14031cd50: 41 5d                       	popq	%r13
14031cd52: 41 5e                       	popq	%r14
14031cd54: 41 5f                       	popq	%r15
14031cd56: 5d                          	popq	%rbp
14031cd57: c3                          	retq
14031cd58: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14031cd60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cd65: 55                          	pushq	%rbp
14031cd66: 41 57                       	pushq	%r15
14031cd68: 41 56                       	pushq	%r14
14031cd6a: 41 55                       	pushq	%r13
14031cd6c: 41 54                       	pushq	%r12
14031cd6e: 56                          	pushq	%rsi
14031cd6f: 57                          	pushq	%rdi
14031cd70: 53                          	pushq	%rbx
14031cd71: 48 83 ec 28                 	subq	$0x28, %rsp
14031cd75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cd7c: 80 bd 3f 01 00 00 00        	cmpb	$0x0, 0x13f(%rbp)
14031cd83: 75 11                       	jne	0x14031cd96 <.text+0x31bd96>
14031cd85: 48 83 c4 28                 	addq	$0x28, %rsp
14031cd89: 5b                          	popq	%rbx
14031cd8a: 5f                          	popq	%rdi
14031cd8b: 5e                          	popq	%rsi
14031cd8c: 41 5c                       	popq	%r12
14031cd8e: 41 5d                       	popq	%r13
14031cd90: 41 5e                       	popq	%r14
14031cd92: 41 5f                       	popq	%r15
14031cd94: 5d                          	popq	%rbp
14031cd95: c3                          	retq
14031cd96: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14031cd9d: e8 be 98 2e 00              	callq	0x140606660 <.text+0x605660>
14031cda2: eb e1                       	jmp	0x14031cd85 <.text+0x31bd85>
14031cda4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031cdb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cdb5: 55                          	pushq	%rbp
14031cdb6: 41 57                       	pushq	%r15
14031cdb8: 41 56                       	pushq	%r14
14031cdba: 41 55                       	pushq	%r13
14031cdbc: 41 54                       	pushq	%r12
14031cdbe: 56                          	pushq	%rsi
14031cdbf: 57                          	pushq	%rdi
14031cdc0: 53                          	pushq	%rbx
14031cdc1: 48 83 ec 28                 	subq	$0x28, %rsp
14031cdc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cdcc: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031cdd3: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031cdda: e8 f1 44 d2 ff              	callq	0x1400412d0 <.text+0x402d0>
14031cddf: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031cde6: 48 83 c4 28                 	addq	$0x28, %rsp
14031cdea: 5b                          	popq	%rbx
14031cdeb: 5f                          	popq	%rdi
14031cdec: 5e                          	popq	%rsi
14031cded: 41 5c                       	popq	%r12
14031cdef: 41 5d                       	popq	%r13
14031cdf1: 41 5e                       	popq	%r14
14031cdf3: 41 5f                       	popq	%r15
14031cdf5: 5d                          	popq	%rbp
14031cdf6: c3                          	retq
14031cdf7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031ce00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031ce05: 55                          	pushq	%rbp
14031ce06: 41 57                       	pushq	%r15
14031ce08: 41 56                       	pushq	%r14
14031ce0a: 41 55                       	pushq	%r13
14031ce0c: 41 54                       	pushq	%r12
14031ce0e: 56                          	pushq	%rsi
14031ce0f: 57                          	pushq	%rdi
14031ce10: 53                          	pushq	%rbx
14031ce11: 48 83 ec 28                 	subq	$0x28, %rsp
14031ce15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031ce1c: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031ce23: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031ce2a: e8 a1 44 d2 ff              	callq	0x1400412d0 <.text+0x402d0>
14031ce2f: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031ce36: 48 83 c4 28                 	addq	$0x28, %rsp
14031ce3a: 5b                          	popq	%rbx
14031ce3b: 5f                          	popq	%rdi
14031ce3c: 5e                          	popq	%rsi
14031ce3d: 41 5c                       	popq	%r12
14031ce3f: 41 5d                       	popq	%r13
14031ce41: 41 5e                       	popq	%r14
14031ce43: 41 5f                       	popq	%r15
14031ce45: 5d                          	popq	%rbp
14031ce46: c3                          	retq
14031ce47: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031ce50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031ce55: 55                          	pushq	%rbp
14031ce56: 41 57                       	pushq	%r15
14031ce58: 41 56                       	pushq	%r14
14031ce5a: 41 55                       	pushq	%r13
14031ce5c: 41 54                       	pushq	%r12
14031ce5e: 56                          	pushq	%rsi
14031ce5f: 57                          	pushq	%rdi
14031ce60: 53                          	pushq	%rbx
14031ce61: 48 83 ec 28                 	subq	$0x28, %rsp
14031ce65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031ce6c: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031ce73: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031ce77: e8 64 aa 2d 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14031ce7c: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
14031ce83: 48 83 c4 28                 	addq	$0x28, %rsp
14031ce87: 5b                          	popq	%rbx
14031ce88: 5f                          	popq	%rdi
14031ce89: 5e                          	popq	%rsi
14031ce8a: 41 5c                       	popq	%r12
14031ce8c: 41 5d                       	popq	%r13
14031ce8e: 41 5e                       	popq	%r14
14031ce90: 41 5f                       	popq	%r15
14031ce92: 5d                          	popq	%rbp
14031ce93: c3                          	retq
14031ce94: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031cea0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cea5: 55                          	pushq	%rbp
14031cea6: 41 57                       	pushq	%r15
14031cea8: 41 56                       	pushq	%r14
14031ceaa: 41 55                       	pushq	%r13
14031ceac: 41 54                       	pushq	%r12
14031ceae: 56                          	pushq	%rsi
14031ceaf: 57                          	pushq	%rdi
14031ceb0: 53                          	pushq	%rbx
14031ceb1: 48 83 ec 28                 	subq	$0x28, %rsp
14031ceb5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cebc: c6 85 3d 01 00 00 01        	movb	$0x1, 0x13d(%rbp)
14031cec3: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031cec7: e8 14 aa 2d 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14031cecc: c6 85 3d 01 00 00 01        	movb	$0x1, 0x13d(%rbp)
14031ced3: 48 83 c4 28                 	addq	$0x28, %rsp
14031ced7: 5b                          	popq	%rbx
14031ced8: 5f                          	popq	%rdi
14031ced9: 5e                          	popq	%rsi
14031ceda: 41 5c                       	popq	%r12
14031cedc: 41 5d                       	popq	%r13
14031cede: 41 5e                       	popq	%r14
14031cee0: 41 5f                       	popq	%r15
14031cee2: 5d                          	popq	%rbp
14031cee3: c3                          	retq
14031cee4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031cef0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cef5: 55                          	pushq	%rbp
14031cef6: 41 57                       	pushq	%r15
14031cef8: 41 56                       	pushq	%r14
14031cefa: 41 55                       	pushq	%r13
14031cefc: 41 54                       	pushq	%r12
14031cefe: 56                          	pushq	%rsi
14031ceff: 57                          	pushq	%rdi
14031cf00: 53                          	pushq	%rbx
14031cf01: 48 83 ec 28                 	subq	$0x28, %rsp
14031cf05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cf0c: c6 85 3b 01 00 00 01        	movb	$0x1, 0x13b(%rbp)
14031cf13: c6 85 3a 01 00 00 01        	movb	$0x1, 0x13a(%rbp)
14031cf1a: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031cf1e: e8 bd a9 2d 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14031cf23: c6 85 3b 01 00 00 01        	movb	$0x1, 0x13b(%rbp)
14031cf2a: c6 85 3a 01 00 00 01        	movb	$0x1, 0x13a(%rbp)
14031cf31: 48 83 c4 28                 	addq	$0x28, %rsp
14031cf35: 5b                          	popq	%rbx
14031cf36: 5f                          	popq	%rdi
14031cf37: 5e                          	popq	%rsi
14031cf38: 41 5c                       	popq	%r12
14031cf3a: 41 5d                       	popq	%r13
14031cf3c: 41 5e                       	popq	%r14
14031cf3e: 41 5f                       	popq	%r15
14031cf40: 5d                          	popq	%rbp
14031cf41: c3                          	retq
14031cf42: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14031cf50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cf55: 55                          	pushq	%rbp
14031cf56: 41 57                       	pushq	%r15
14031cf58: 41 56                       	pushq	%r14
14031cf5a: 41 55                       	pushq	%r13
14031cf5c: 41 54                       	pushq	%r12
14031cf5e: 56                          	pushq	%rsi
14031cf5f: 57                          	pushq	%rdi
14031cf60: 53                          	pushq	%rbx
14031cf61: 48 83 ec 28                 	subq	$0x28, %rsp
14031cf65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cf6c: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14031cf70: e8 3b 55 8c 00              	callq	0x140be24b0 <.text+0xbe14b0>
14031cf75: 90                          	nop
14031cf76: 48 83 c4 28                 	addq	$0x28, %rsp
14031cf7a: 5b                          	popq	%rbx
14031cf7b: 5f                          	popq	%rdi
14031cf7c: 5e                          	popq	%rsi
14031cf7d: 41 5c                       	popq	%r12
14031cf7f: 41 5d                       	popq	%r13
14031cf81: 41 5e                       	popq	%r14
14031cf83: 41 5f                       	popq	%r15
14031cf85: 5d                          	popq	%rbp
14031cf86: c3                          	retq
14031cf87: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031cf90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031cf95: 55                          	pushq	%rbp
14031cf96: 41 57                       	pushq	%r15
14031cf98: 41 56                       	pushq	%r14
14031cf9a: 41 55                       	pushq	%r13
14031cf9c: 41 54                       	pushq	%r12
14031cf9e: 56                          	pushq	%rsi
14031cf9f: 57                          	pushq	%rdi
14031cfa0: 53                          	pushq	%rbx
14031cfa1: 48 83 ec 28                 	subq	$0x28, %rsp
14031cfa5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031cfac: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14031cfb3: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14031cfba: 48 89 41 08                 	movq	%rax, 0x8(%rcx)
14031cfbe: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14031cfc2: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
14031cfc6: 0f 10 95 86 00 00 00        	movups	0x86(%rbp), %xmm2
14031cfcd: 0f 11 41 10                 	movups	%xmm0, 0x10(%rcx)
14031cfd1: 0f 11 49 20                 	movups	%xmm1, 0x20(%rcx)
14031cfd5: 0f 11 51 2e                 	movups	%xmm2, 0x2e(%rcx)
14031cfd9: c6 41 3e 08                 	movb	$0x8, 0x3e(%rcx)
14031cfdd: c6 85 3b 01 00 00 00        	movb	$0x0, 0x13b(%rbp)
14031cfe4: c6 85 3a 01 00 00 01        	movb	$0x1, 0x13a(%rbp)
14031cfeb: 48 83 c4 28                 	addq	$0x28, %rsp
14031cfef: 5b                          	popq	%rbx
14031cff0: 5f                          	popq	%rdi
14031cff1: 5e                          	popq	%rsi
14031cff2: 41 5c                       	popq	%r12
14031cff4: 41 5d                       	popq	%r13
14031cff6: 41 5e                       	popq	%r14
14031cff8: 41 5f                       	popq	%r15
14031cffa: 5d                          	popq	%rbp
14031cffb: c3                          	retq
14031cffc: 0f 1f 40 00                 	nopl	(%rax)
14031d000: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031d005: 55                          	pushq	%rbp
14031d006: 41 57                       	pushq	%r15
14031d008: 41 56                       	pushq	%r14
14031d00a: 41 55                       	pushq	%r13
14031d00c: 41 54                       	pushq	%r12
14031d00e: 56                          	pushq	%rsi
14031d00f: 57                          	pushq	%rdi
14031d010: 53                          	pushq	%rbx
14031d011: 48 83 ec 28                 	subq	$0x28, %rsp
14031d015: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031d01c: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
14031d023: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
14031d02a: 48 89 08                    	movq	%rcx, (%rax)
14031d02d: c6 85 3b 01 00 00 00        	movb	$0x0, 0x13b(%rbp)
14031d034: c6 85 3a 01 00 00 00        	movb	$0x0, 0x13a(%rbp)
14031d03b: 48 83 c4 28                 	addq	$0x28, %rsp
14031d03f: 5b                          	popq	%rbx
14031d040: 5f                          	popq	%rdi
14031d041: 5e                          	popq	%rsi
14031d042: 41 5c                       	popq	%r12
14031d044: 41 5d                       	popq	%r13
14031d046: 41 5e                       	popq	%r14
14031d048: 41 5f                       	popq	%r15
14031d04a: 5d                          	popq	%rbp
14031d04b: c3                          	retq
14031d04c: 0f 1f 40 00                 	nopl	(%rax)
14031d050: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031d055: 55                          	pushq	%rbp
14031d056: 41 57                       	pushq	%r15
14031d058: 41 56                       	pushq	%r14
14031d05a: 41 55                       	pushq	%r13
14031d05c: 41 54                       	pushq	%r12
14031d05e: 56                          	pushq	%rsi
14031d05f: 57                          	pushq	%rdi
14031d060: 53                          	pushq	%rbx
14031d061: 48 83 ec 28                 	subq	$0x28, %rsp
14031d065: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031d06c: c6 85 2f 01 00 00 01        	movb	$0x1, 0x12f(%rbp)
14031d073: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031d077: e8 64 a8 2d 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14031d07c: c6 85 2f 01 00 00 01        	movb	$0x1, 0x12f(%rbp)
14031d083: 48 83 c4 28                 	addq	$0x28, %rsp
14031d087: 5b                          	popq	%rbx
14031d088: 5f                          	popq	%rdi
14031d089: 5e                          	popq	%rsi
14031d08a: 41 5c                       	popq	%r12
14031d08c: 41 5d                       	popq	%r13
14031d08e: 41 5e                       	popq	%r14
14031d090: 41 5f                       	popq	%r15
14031d092: 5d                          	popq	%rbp
14031d093: c3                          	retq
14031d094: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031d0a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031d0a5: 55                          	pushq	%rbp
14031d0a6: 41 57                       	pushq	%r15
14031d0a8: 41 56                       	pushq	%r14
14031d0aa: 41 55                       	pushq	%r13
14031d0ac: 41 54                       	pushq	%r12
14031d0ae: 56                          	pushq	%rsi
14031d0af: 57                          	pushq	%rdi
14031d0b0: 53                          	pushq	%rbx
14031d0b1: 48 83 ec 28                 	subq	$0x28, %rsp
14031d0b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031d0bc: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
14031d0c3: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
14031d0ca: 48 89 01                    	movq	%rax, (%rcx)
14031d0cd: c6 85 2f 01 00 00 00        	movb	$0x0, 0x12f(%rbp)
14031d0d4: 48 83 c4 28                 	addq	$0x28, %rsp
14031d0d8: 5b                          	popq	%rbx
14031d0d9: 5f                          	popq	%rdi
14031d0da: 5e                          	popq	%rsi
14031d0db: 41 5c                       	popq	%r12
14031d0dd: 41 5d                       	popq	%r13
14031d0df: 41 5e                       	popq	%r14
14031d0e1: 41 5f                       	popq	%r15
14031d0e3: 5d                          	popq	%rbp
14031d0e4: c3                          	retq
14031d0e5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14031d0f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031d0f5: 55                          	pushq	%rbp
14031d0f6: 41 57                       	pushq	%r15
14031d0f8: 41 56                       	pushq	%r14
14031d0fa: 41 55                       	pushq	%r13
14031d0fc: 41 54                       	pushq	%r12
14031d0fe: 56                          	pushq	%rsi
14031d0ff: 57                          	pushq	%rdi
14031d100: 53                          	pushq	%rbx
14031d101: 48 83 ec 28                 	subq	$0x28, %rsp
14031d105: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031d10c: c6 85 39 01 00 00 00        	movb	$0x0, 0x139(%rbp)
14031d113: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14031d117: e8 04 ee 2d 00              	callq	0x1405fbf20 <.text+0x5faf20>
14031d11c: c6 85 39 01 00 00 00        	movb	$0x0, 0x139(%rbp)
14031d123: 48 83 c4 28                 	addq	$0x28, %rsp
14031d127: 5b                          	popq	%rbx
14031d128: 5f                          	popq	%rdi
14031d129: 5e                          	popq	%rsi
14031d12a: 41 5c                       	popq	%r12
14031d12c: 41 5d                       	popq	%r13
14031d12e: 41 5e                       	popq	%r14
14031d130: 41 5f                       	popq	%r15
14031d132: 5d                          	popq	%rbp
14031d133: c3                          	retq
