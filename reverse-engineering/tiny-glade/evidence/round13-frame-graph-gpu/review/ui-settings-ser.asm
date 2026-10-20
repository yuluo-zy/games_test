
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a46ad0: 56                          	pushq	%rsi
140a46ad1: 57                          	pushq	%rdi
140a46ad2: 53                          	pushq	%rbx
140a46ad3: 48 83 ec 40                 	subq	$0x40, %rsp
140a46ad7: 48 89 ce                    	movq	%rcx, %rsi
140a46ada: 48 8b 3a                    	movq	(%rdx), %rdi
140a46add: 48 ff 42 18                 	incq	0x18(%rdx)
140a46ae1: c6 42 20 00                 	movb	$0x0, 0x20(%rdx)
140a46ae5: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140a46ae9: 48 39 07                    	cmpq	%rax, (%rdi)
140a46aec: 0f 84 25 01 00 00           	je	0x140a46c17 <.text+0xa45c17>
140a46af2: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
140a46af6: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140a46afa: 48 ff c0                    	incq	%rax
140a46afd: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
140a46b01: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
140a46b06: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140a46b0b: 48 8d 15 54 81 08 02        	leaq	0x2088154(%rip), %rdx   # 0x142acec66
140a46b12: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140a46b17: 41 b8 0e 00 00 00           	movl	$0xe, %r8d
140a46b1d: 49 89 f1                    	movq	%rsi, %r9
140a46b20: e8 4b c8 b3 00              	callq	0x141583370 <.text+0x1582370>
140a46b25: 48 85 c0                    	testq	%rax, %rax
140a46b28: 0f 85 cb 00 00 00           	jne	0x140a46bf9 <.text+0xa45bf9>
140a46b2e: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140a46b32: 48 8d 15 3b 81 08 02        	leaq	0x208813b(%rip), %rdx   # 0x142acec74
140a46b39: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140a46b3e: 41 b8 11 00 00 00           	movl	$0x11, %r8d
140a46b44: e8 27 5c b4 00              	callq	0x14158c770 <.text+0x158b770>
140a46b49: 48 85 c0                    	testq	%rax, %rax
140a46b4c: 0f 85 a7 00 00 00           	jne	0x140a46bf9 <.text+0xa45bf9>
140a46b52: 4c 8d 4e 09                 	leaq	0x9(%rsi), %r9
140a46b56: 48 8d 15 28 81 08 02        	leaq	0x2088128(%rip), %rdx   # 0x142acec85
140a46b5d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140a46b62: 41 b8 11 00 00 00           	movl	$0x11, %r8d
140a46b68: e8 c3 1b b4 00              	callq	0x141588730 <.text+0x1587730>
140a46b6d: 48 85 c0                    	testq	%rax, %rax
140a46b70: 0f 85 83 00 00 00           	jne	0x140a46bf9 <.text+0xa45bf9>
140a46b76: 4c 8d 4e 0a                 	leaq	0xa(%rsi), %r9
140a46b7a: 48 8d 15 15 81 08 02        	leaq	0x2088115(%rip), %rdx   # 0x142acec96
140a46b81: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140a46b86: 41 b8 13 00 00 00           	movl	$0x13, %r8d
140a46b8c: e8 9f 1b b4 00              	callq	0x141588730 <.text+0x1587730>
140a46b91: 48 85 c0                    	testq	%rax, %rax
140a46b94: 75 63                       	jne	0x140a46bf9 <.text+0xa45bf9>
140a46b96: 4c 8d 4e 0b                 	leaq	0xb(%rsi), %r9
140a46b9a: 48 8d 15 08 81 08 02        	leaq	0x2088108(%rip), %rdx   # 0x142aceca9
140a46ba1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140a46ba6: 41 b8 09 00 00 00           	movl	$0x9, %r8d
140a46bac: e8 7f 1b b4 00              	callq	0x141588730 <.text+0x1587730>
140a46bb1: 48 85 c0                    	testq	%rax, %rax
140a46bb4: 75 43                       	jne	0x140a46bf9 <.text+0xa45bf9>
140a46bb6: 4c 8d 4e 0c                 	leaq	0xc(%rsi), %r9
140a46bba: 48 8d 15 f1 80 08 02        	leaq	0x20880f1(%rip), %rdx   # 0x142acecb2
140a46bc1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140a46bc6: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
140a46bcc: e8 5f 1b b4 00              	callq	0x141588730 <.text+0x1587730>
140a46bd1: 48 85 c0                    	testq	%rax, %rax
140a46bd4: 75 23                       	jne	0x140a46bf9 <.text+0xa45bf9>
140a46bd6: 48 83 c6 04                 	addq	$0x4, %rsi
140a46bda: 48 8d 15 de 80 08 02        	leaq	0x20880de(%rip), %rdx   # 0x142acecbf
140a46be1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140a46be6: 41 b8 12 00 00 00           	movl	$0x12, %r8d
140a46bec: 49 89 f1                    	movq	%rsi, %r9
140a46bef: e8 7c c7 b3 00              	callq	0x141583370 <.text+0x1582370>
140a46bf4: 48 85 c0                    	testq	%rax, %rax
140a46bf7: 74 08                       	je	0x140a46c01 <.text+0xa45c01>
140a46bf9: 48 83 c4 40                 	addq	$0x40, %rsp
140a46bfd: 5b                          	popq	%rbx
140a46bfe: 5f                          	popq	%rdi
140a46bff: 5e                          	popq	%rsi
140a46c00: c3                          	retq
140a46c01: 48 8b 4c 24 30              	movq	0x30(%rsp), %rcx
140a46c06: 0f b6 54 24 38              	movzbl	0x38(%rsp), %edx
140a46c0b: 48 83 c4 40                 	addq	$0x40, %rsp
140a46c0f: 5b                          	popq	%rbx
140a46c10: 5f                          	popq	%rdi
140a46c11: 5e                          	popq	%rsi
140a46c12: e9 99 df ff ff              	jmp	0x140a44bb0 <.text+0xa43bb0>
140a46c17: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a46c20: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a46c26: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a46c2c: 48 89 f9                    	movq	%rdi, %rcx
140a46c2f: 48 89 d3                    	movq	%rdx, %rbx
140a46c32: 48 89 c2                    	movq	%rax, %rdx
140a46c35: e8 26 2e e6 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a46c3a: 48 89 da                    	movq	%rbx, %rdx
140a46c3d: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140a46c41: e9 ac fe ff ff              	jmp	0x140a46af2 <.text+0xa45af2>
140a46c46: cc                          	int3
140a46c47: cc                          	int3
140a46c48: cc                          	int3
140a46c49: cc                          	int3
140a46c4a: cc                          	int3
140a46c4b: cc                          	int3
140a46c4c: cc                          	int3
140a46c4d: cc                          	int3
140a46c4e: cc                          	int3
140a46c4f: cc                          	int3
140a46c50: 56                          	pushq	%rsi
140a46c51: 48 83 ec 20                 	subq	$0x20, %rsp
140a46c55: 48 89 c8                    	movq	%rcx, %rax
140a46c58: 49 8d 48 f7                 	leaq	-0x9(%r8), %rcx
140a46c5c: 48 83 f9 0a                 	cmpq	$0xa, %rcx
140a46c60: 0f 87 18 01 00 00           	ja	0x140a46d7e <.text+0xa45d7e>
140a46c66: 4c 8d 0d 1f 8a 08 02        	leaq	0x2088a1f(%rip), %r9    # 0x142acf68c
140a46c6d: 49 63 0c 89                 	movslq	(%r9,%rcx,4), %rcx
140a46c71: 4c 01 c9                    	addq	%r9, %rcx
140a46c74: ff e1                       	jmpq	*%rcx
140a46c76: 48 b9 67 61 6d 65 5f 6c 6f 67       	movabsq	$0x676f6c5f656d6167, %rcx # imm = 0x676F6C5F656D6167
140a46c80: 48 33 0a                    	xorq	(%rdx), %rcx
140a46c83: 0f b6 52 08                 	movzbl	0x8(%rdx), %edx
140a46c87: 48 83 f2 6f                 	xorq	$0x6f, %rdx
140a46c8b: 48 09 ca                    	orq	%rcx, %rdx
140a46c8e: 0f 85 ea 00 00 00           	jne	0x140a46d7e <.text+0xa45d7e>
140a46c94: c6 40 01 04                 	movb	$0x4, 0x1(%rax)
140a46c98: e9 e8 00 00 00              	jmp	0x140a46d85 <.text+0xa45d85>
140a46c9d: 48 b9 69 63 6f 6e 5f 73 69 7a       	movabsq	$0x7a69735f6e6f6369, %rcx # imm = 0x7A69735F6E6F6369
140a46ca7: 48 33 0a                    	xorq	(%rdx), %rcx
140a46caa: 49 b8 69 7a 65 5f 6d 75 6c 74       	movabsq	$0x746c756d5f657a69, %r8 # imm = 0x746C756D5F657A69
140a46cb4: 4c 33 42 06                 	xorq	0x6(%rdx), %r8
140a46cb8: 49 09 c8                    	orq	%rcx, %r8
140a46cbb: 0f 85 bd 00 00 00           	jne	0x140a46d7e <.text+0xa45d7e>
140a46cc1: c6 40 01 00                 	movb	$0x0, 0x1(%rax)
140a46cc5: e9 bb 00 00 00              	jmp	0x140a46d85 <.text+0xa45d85>
140a46cca: f3 0f 6f 02                 	movdqu	(%rdx), %xmm0
140a46cce: f3 0f 6f 4a 03              	movdqu	0x3(%rdx), %xmm1
140a46cd3: 66 0f 74 0d 55 6c ef 01     	pcmpeqb	0x1ef6c55(%rip), %xmm1  # 0x14293d930
140a46cdb: 66 0f 74 05 5d 6c ef 01     	pcmpeqb	0x1ef6c5d(%rip), %xmm0  # 0x14293d940
140a46ce3: 66 0f db c1                 	pand	%xmm1, %xmm0
140a46ce7: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
140a46ceb: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
140a46cf1: 0f 85 87 00 00 00           	jne	0x140a46d7e <.text+0xa45d7e>
140a46cf7: c6 40 01 03                 	movb	$0x3, 0x1(%rax)
140a46cfb: e9 85 00 00 00              	jmp	0x140a46d85 <.text+0xa45d85>
140a46d00: 48 b9 63 75 73 74 6f 6d 5f 63       	movabsq	$0x635f6d6f74737563, %rcx # imm = 0x635F6D6F74737563
140a46d0a: 48 33 0a                    	xorq	(%rdx), %rcx
140a46d0d: 49 b8 6d 5f 63 75 72 73 6f 72       	movabsq	$0x726f737275635f6d, %r8 # imm = 0x726F737275635F6D
140a46d17: 4c 33 42 05                 	xorq	0x5(%rdx), %r8
140a46d1b: 49 09 c8                    	orq	%rcx, %r8
140a46d1e: 75 5e                       	jne	0x140a46d7e <.text+0xa45d7e>
140a46d20: c6 40 01 05                 	movb	$0x5, 0x1(%rax)
140a46d24: eb 5f                       	jmp	0x140a46d85 <.text+0xa45d85>
140a46d26: f3 0f 6f 02                 	movdqu	(%rdx), %xmm0
140a46d2a: 0f b6 4a 10                 	movzbl	0x10(%rdx), %ecx
140a46d2e: 66 0f 6e c9                 	movd	%ecx, %xmm1
140a46d32: 66 0f 74 05 16 6c ef 01     	pcmpeqb	0x1ef6c16(%rip), %xmm0  # 0x14293d950
140a46d3a: 66 0f 74 0d 1e 6c ef 01     	pcmpeqb	0x1ef6c1e(%rip), %xmm1  # 0x14293d960
140a46d42: 66 0f db c8                 	pand	%xmm0, %xmm1
140a46d46: 66 0f d7 c9                 	pmovmskb	%xmm1, %ecx
140a46d4a: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
140a46d50: 74 63                       	je	0x140a46db5 <.text+0xa45db5>
140a46d52: f3 0f 6f 02                 	movdqu	(%rdx), %xmm0
140a46d56: 0f b6 4a 10                 	movzbl	0x10(%rdx), %ecx
140a46d5a: 66 0f 6e c9                 	movd	%ecx, %xmm1
140a46d5e: 66 0f 74 05 0a 6c ef 01     	pcmpeqb	0x1ef6c0a(%rip), %xmm0  # 0x14293d970
140a46d66: 66 0f 74 0d 92 53 ee 01     	pcmpeqb	0x1ee5392(%rip), %xmm1  # 0x14292c100
140a46d6e: 66 0f db c8                 	pand	%xmm0, %xmm1
140a46d72: 66 0f d7 c9                 	pmovmskb	%xmm1, %ecx
140a46d76: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
140a46d7c: 74 3d                       	je	0x140a46dbb <.text+0xa45dbb>
140a46d7e: 48 8d 50 01                 	leaq	0x1(%rax), %rdx
140a46d82: c6 02 07                    	movb	$0x7, (%rdx)
140a46d85: c6 00 00                    	movb	$0x0, (%rax)
140a46d88: 48 83 c4 20                 	addq	$0x20, %rsp
140a46d8c: 5e                          	popq	%rsi
140a46d8d: c3                          	retq
140a46d8e: 4c 8d 0d 2a 7f 08 02        	leaq	0x2087f2a(%rip), %r9    # 0x142acecbf
140a46d95: 48 89 d1                    	movq	%rdx, %rcx
140a46d98: 4c 89 ca                    	movq	%r9, %rdx
140a46d9b: 48 89 c6                    	movq	%rax, %rsi
140a46d9e: e8 4d ce ed 01              	callq	0x142923bf0 <.text+0x2922bf0>
140a46da3: 89 c1                       	movl	%eax, %ecx
140a46da5: 48 89 f0                    	movq	%rsi, %rax
140a46da8: 48 8d 56 01                 	leaq	0x1(%rsi), %rdx
140a46dac: 85 c9                       	testl	%ecx, %ecx
140a46dae: 75 d2                       	jne	0x140a46d82 <.text+0xa45d82>
140a46db0: c6 02 06                    	movb	$0x6, (%rdx)
140a46db3: eb d0                       	jmp	0x140a46d85 <.text+0xa45d85>
140a46db5: c6 40 01 01                 	movb	$0x1, 0x1(%rax)
140a46db9: eb ca                       	jmp	0x140a46d85 <.text+0xa45d85>
140a46dbb: c6 40 01 02                 	movb	$0x2, 0x1(%rax)
140a46dbf: eb c4                       	jmp	0x140a46d85 <.text+0xa45d85>
140a46dc1: cc                          	int3
140a46dc2: cc                          	int3
140a46dc3: cc                          	int3
140a46dc4: cc                          	int3
140a46dc5: cc                          	int3
140a46dc6: cc                          	int3
140a46dc7: cc                          	int3
140a46dc8: cc                          	int3
140a46dc9: cc                          	int3
140a46dca: cc                          	int3
140a46dcb: cc                          	int3
140a46dcc: cc                          	int3
140a46dcd: cc                          	int3
140a46dce: cc                          	int3
140a46dcf: cc                          	int3
140a46dd0: 41 56                       	pushq	%r14
140a46dd2: 56                          	pushq	%rsi
140a46dd3: 57                          	pushq	%rdi
140a46dd4: 53                          	pushq	%rbx
140a46dd5: 48 83 ec 38                 	subq	$0x38, %rsp
140a46dd9: 48 89 ce                    	movq	%rcx, %rsi
140a46ddc: 48 8b 4a 20                 	movq	0x20(%rdx), %rcx
140a46de0: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140a46de4: 48 39 c8                    	cmpq	%rcx, %rax
140a46de7: 73 3d                       	jae	0x140a46e26 <.text+0xa45e26>
140a46de9: 4c 8b 42 18                 	movq	0x18(%rdx), %r8
140a46ded: 48 ff c0                    	incq	%rax
140a46df0: 48 f7 d9                    	negq	%rcx
140a46df3: 48 bf 00 26 00 00 01 00 00 00       	movabsq	$0x100002600, %rdi # imm = 0x100002600
140a46dfd: 0f 1f 00                    	nopl	(%rax)
140a46e00: 45 0f b6 4c 00 ff           	movzbl	-0x1(%r8,%rax), %r9d
140a46e06: 49 83 f9 22                 	cmpq	$0x22, %r9
140a46e0a: 77 6f                       	ja	0x140a46e7b <.text+0xa45e7b>
140a46e0c: 4c 0f a3 cf                 	btq	%r9, %rdi
140a46e10: 73 32                       	jae	0x140a46e44 <.text+0xa45e44>
140a46e12: 48 89 42 28                 	movq	%rax, 0x28(%rdx)
140a46e16: 4c 8d 0c 01                 	leaq	(%rcx,%rax), %r9
140a46e1a: 49 ff c1                    	incq	%r9
140a46e1d: 48 ff c0                    	incq	%rax
140a46e20: 49 83 f9 01                 	cmpq	$0x1, %r9
140a46e24: 75 da                       	jne	0x140a46e00 <.text+0xa45e00>
140a46e26: 48 c7 44 24 20 05 00 00 00  	movq	$0x5, 0x20(%rsp)
140a46e2f: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
140a46e34: 48 89 d1                    	movq	%rdx, %rcx
140a46e37: 48 89 c2                    	movq	%rax, %rdx
140a46e3a: e8 e1 9a e5 01              	callq	0x1428a0920 <.text+0x289f920>
140a46e3f: e9 f9 00 00 00              	jmp	0x140a46f3d <.text+0xa45f3d>
140a46e44: 49 83 f9 22                 	cmpq	$0x22, %r9
140a46e48: 75 31                       	jne	0x140a46e7b <.text+0xa45e7b>
140a46e4a: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140a46e4f: e8 dc 5d 9c 00              	callq	0x14140cc30 <.text+0x140bc30>
140a46e54: 80 7c 24 20 01              	cmpb	$0x1, 0x20(%rsp)
140a46e59: 74 4b                       	je	0x140a46ea6 <.text+0xa45ea6>
140a46e5b: 0f b6 44 24 21              	movzbl	0x21(%rsp), %eax
140a46e60: 48 8d 0d 51 88 08 02        	leaq	0x2088851(%rip), %rcx   # 0x142acf6b8
140a46e67: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140a46e6b: 48 01 c8                    	addq	%rcx, %rax
140a46e6e: ff e0                       	jmpq	*%rax
140a46e70: c6 46 01 00                 	movb	$0x0, 0x1(%rsi)
140a46e74: 31 c0                       	xorl	%eax, %eax
140a46e76: e9 c8 00 00 00              	jmp	0x140a46f43 <.text+0xa45f43>
140a46e7b: 41 83 f9 7b                 	cmpl	$0x7b, %r9d
140a46e7f: 0f 85 58 01 00 00           	jne	0x140a46fdd <.text+0xa45fdd>
140a46e85: fe 4a 30                    	decb	0x30(%rdx)
140a46e88: 0f 84 5d 01 00 00           	je	0x140a46feb <.text+0xa45feb>
140a46e8e: 48 89 42 28                 	movq	%rax, 0x28(%rdx)
140a46e92: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140a46e97: 49 89 d6                    	movq	%rdx, %r14
140a46e9a: e8 91 5d 9c 00              	callq	0x14140cc30 <.text+0x140bc30>
140a46e9f: 80 7c 24 20 00              	cmpb	$0x0, 0x20(%rsp)
140a46ea4: 74 0a                       	je	0x140a46eb0 <.text+0xa45eb0>
140a46ea6: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
140a46eab: e9 8d 00 00 00              	jmp	0x140a46f3d <.text+0xa45f3d>
140a46eb0: 4c 89 f1                    	movq	%r14, %rcx
140a46eb3: 49 8b 56 20                 	movq	0x20(%r14), %rdx
140a46eb7: 49 8b 46 28                 	movq	0x28(%r14), %rax
140a46ebb: 48 39 d0                    	cmpq	%rdx, %rax
140a46ebe: 73 3d                       	jae	0x140a46efd <.text+0xa45efd>
140a46ec0: 0f b6 5c 24 21              	movzbl	0x21(%rsp), %ebx
140a46ec5: 44 0f b6 c3                 	movzbl	%bl, %r8d
140a46ec9: 4c 8b 49 18                 	movq	0x18(%rcx), %r9
140a46ecd: 48 ff c0                    	incq	%rax
140a46ed0: 48 f7 da                    	negq	%rdx
140a46ed3: 45 0f b6 54 01 ff           	movzbl	-0x1(%r9,%rax), %r10d
140a46ed9: 49 83 fa 3a                 	cmpq	$0x3a, %r10
140a46edd: 0f 87 16 01 00 00           	ja	0x140a46ff9 <.text+0xa45ff9>
140a46ee3: 4c 0f a3 d7                 	btq	%r10, %rdi
140a46ee7: 73 29                       	jae	0x140a46f12 <.text+0xa45f12>
140a46ee9: 48 89 41 28                 	movq	%rax, 0x28(%rcx)
140a46eed: 4c 8d 14 02                 	leaq	(%rdx,%rax), %r10
140a46ef1: 49 ff c2                    	incq	%r10
140a46ef4: 48 ff c0                    	incq	%rax
140a46ef7: 49 83 fa 01                 	cmpq	$0x1, %r10
140a46efb: 75 d6                       	jne	0x140a46ed3 <.text+0xa45ed3>
140a46efd: 48 c7 44 24 20 03 00 00 00  	movq	$0x3, 0x20(%rsp)
140a46f06: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
140a46f0b: e8 10 9a e5 01              	callq	0x1428a0920 <.text+0x289f920>
140a46f10: eb 2b                       	jmp	0x140a46f3d <.text+0xa45f3d>
140a46f12: 49 83 fa 3a                 	cmpq	$0x3a, %r10
140a46f16: 0f 85 dd 00 00 00           	jne	0x140a46ff9 <.text+0xa45ff9>
140a46f1c: 48 89 41 28                 	movq	%rax, 0x28(%rcx)
140a46f20: 44 89 c0                    	movl	%r8d, %eax
140a46f23: 48 8d 15 9e 87 08 02        	leaq	0x208879e(%rip), %rdx   # 0x142acf6c8
140a46f2a: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
140a46f2e: 48 01 d0                    	addq	%rdx, %rax
140a46f31: ff e0                       	jmpq	*%rax
140a46f33: e8 68 6a 9c 00              	callq	0x14140d9a0 <.text+0x140c9a0>
140a46f38: 48 85 c0                    	testq	%rax, %rax
140a46f3b: 74 15                       	je	0x140a46f52 <.text+0xa45f52>
140a46f3d: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
140a46f41: b0 01                       	movb	$0x1, %al
140a46f43: 88 06                       	movb	%al, (%rsi)
140a46f45: 48 89 f0                    	movq	%rsi, %rax
140a46f48: 48 83 c4 38                 	addq	$0x38, %rsp
140a46f4c: 5b                          	popq	%rbx
140a46f4d: 5f                          	popq	%rdi
140a46f4e: 5e                          	popq	%rsi
140a46f4f: 41 5e                       	popq	%r14
140a46f51: c3                          	retq
140a46f52: 4c 89 f1                    	movq	%r14, %rcx
140a46f55: 41 fe 46 30                 	incb	0x30(%r14)
140a46f59: 49 8b 56 20                 	movq	0x20(%r14), %rdx
140a46f5d: 49 8b 46 28                 	movq	0x28(%r14), %rax
140a46f61: 48 39 d0                    	cmpq	%rdx, %rax
140a46f64: 73 30                       	jae	0x140a46f96 <.text+0xa45f96>
140a46f66: 4c 8b 41 18                 	movq	0x18(%rcx), %r8
140a46f6a: 48 ff c0                    	incq	%rax
140a46f6d: 48 f7 da                    	negq	%rdx
140a46f70: 45 0f b6 4c 00 ff           	movzbl	-0x1(%r8,%rax), %r9d
140a46f76: 49 83 f9 20                 	cmpq	$0x20, %r9
140a46f7a: 77 2f                       	ja	0x140a46fab <.text+0xa45fab>
140a46f7c: 4c 0f a3 cf                 	btq	%r9, %rdi
140a46f80: 73 29                       	jae	0x140a46fab <.text+0xa45fab>
140a46f82: 48 89 41 28                 	movq	%rax, 0x28(%rcx)
140a46f86: 4c 8d 0c 02                 	leaq	(%rdx,%rax), %r9
140a46f8a: 49 ff c1                    	incq	%r9
140a46f8d: 48 ff c0                    	incq	%rax
140a46f90: 49 83 f9 01                 	cmpq	$0x1, %r9
140a46f94: 75 da                       	jne	0x140a46f70 <.text+0xa45f70>
140a46f96: 48 c7 44 24 20 03 00 00 00  	movq	$0x3, 0x20(%rsp)
140a46f9f: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
140a46fa4: e8 f7 9f e5 01              	callq	0x1428a0fa0 <.text+0x289ffa0>
140a46fa9: eb 92                       	jmp	0x140a46f3d <.text+0xa45f3d>
140a46fab: 41 83 f9 7d                 	cmpl	$0x7d, %r9d
140a46faf: 75 56                       	jne	0x140a47007 <.text+0xa46007>
140a46fb1: 48 89 41 28                 	movq	%rax, 0x28(%rcx)
140a46fb5: 88 5e 01                    	movb	%bl, 0x1(%rsi)
140a46fb8: 31 c0                       	xorl	%eax, %eax
140a46fba: eb 87                       	jmp	0x140a46f43 <.text+0xa45f43>
140a46fbc: c6 46 01 02                 	movb	$0x2, 0x1(%rsi)
140a46fc0: 31 c0                       	xorl	%eax, %eax
140a46fc2: e9 7c ff ff ff              	jmp	0x140a46f43 <.text+0xa45f43>
140a46fc7: c6 46 01 03                 	movb	$0x3, 0x1(%rsi)
140a46fcb: 31 c0                       	xorl	%eax, %eax
140a46fcd: e9 71 ff ff ff              	jmp	0x140a46f43 <.text+0xa45f43>
140a46fd2: c6 46 01 01                 	movb	$0x1, 0x1(%rsi)
140a46fd6: 31 c0                       	xorl	%eax, %eax
140a46fd8: e9 66 ff ff ff              	jmp	0x140a46f43 <.text+0xa45f43>
140a46fdd: 48 c7 44 24 20 0a 00 00 00  	movq	$0xa, 0x20(%rsp)
140a46fe6: e9 44 fe ff ff              	jmp	0x140a46e2f <.text+0xa45e2f>
140a46feb: 48 c7 44 24 20 15 00 00 00  	movq	$0x15, 0x20(%rsp)
140a46ff4: e9 36 fe ff ff              	jmp	0x140a46e2f <.text+0xa45e2f>
140a46ff9: 48 c7 44 24 20 06 00 00 00  	movq	$0x6, 0x20(%rsp)
140a47002: e9 ff fe ff ff              	jmp	0x140a46f06 <.text+0xa45f06>
140a47007: 48 c7 44 24 20 0a 00 00 00  	movq	$0xa, 0x20(%rsp)
140a47010: eb 8d                       	jmp	0x140a46f9f <.text+0xa45f9f>
140a47012: cc                          	int3
140a47013: cc                          	int3
140a47014: cc                          	int3
140a47015: cc                          	int3
140a47016: cc                          	int3
140a47017: cc                          	int3
140a47018: cc                          	int3
140a47019: cc                          	int3
140a4701a: cc                          	int3
140a4701b: cc                          	int3
140a4701c: cc                          	int3
140a4701d: cc                          	int3
140a4701e: cc                          	int3
140a4701f: cc                          	int3
140a47020: 56                          	pushq	%rsi
140a47021: 48 83 ec 20                 	subq	$0x20, %rsp
140a47025: 48 89 c8                    	movq	%rcx, %rax
140a47028: 49 8d 48 fd                 	leaq	-0x3(%r8), %rcx
140a4702c: 48 83 f9 03                 	cmpq	$0x3, %rcx
140a47030: 77 7d                       	ja	0x140a470af <.text+0xa460af>
140a47032: 4c 8d 0d 9f 86 08 02        	leaq	0x208869f(%rip), %r9    # 0x142acf6d8
140a47039: 49 63 0c 89                 	movslq	(%r9,%rcx,4), %rcx
140a4703d: 4c 01 c9                    	addq	%r9, %rcx
140a47040: ff e1                       	jmpq	*%rcx
140a47042: 81 3a 4c 65 66 74           	cmpl	$0x7466654c, (%rdx)     # imm = 0x7466654C
140a47048: 75 65                       	jne	0x140a470af <.text+0xa460af>
140a4704a: c6 40 01 01                 	movb	$0x1, 0x1(%rax)
140a4704e: eb 55                       	jmp	0x140a470a5 <.text+0xa460a5>
140a47050: b9 42 6f 74 74              	movl	$0x74746f42, %ecx       # imm = 0x74746F42
140a47055: 33 0a                       	xorl	(%rdx), %ecx
140a47057: 44 0f b7 4a 04              	movzwl	0x4(%rdx), %r9d
140a4705c: 41 81 f1 6f 6d 00 00        	xorl	$0x6d6f, %r9d           # imm = 0x6D6F
140a47063: 41 09 c9                    	orl	%ecx, %r9d
140a47066: 75 47                       	jne	0x140a470af <.text+0xa460af>
140a47068: c6 40 01 00                 	movb	$0x0, 0x1(%rax)
140a4706c: eb 37                       	jmp	0x140a470a5 <.text+0xa460a5>
140a4706e: b9 52 69 67 68              	movl	$0x68676952, %ecx       # imm = 0x68676952
140a47073: 33 0a                       	xorl	(%rdx), %ecx
140a47075: 44 0f b6 4a 04              	movzbl	0x4(%rdx), %r9d
140a4707a: 41 83 f1 74                 	xorl	$0x74, %r9d
140a4707e: 41 09 c9                    	orl	%ecx, %r9d
140a47081: 75 2c                       	jne	0x140a470af <.text+0xa460af>
140a47083: c6 40 01 02                 	movb	$0x2, 0x1(%rax)
140a47087: eb 1c                       	jmp	0x140a470a5 <.text+0xa460a5>
140a47089: 0f b7 0a                    	movzwl	(%rdx), %ecx
140a4708c: 81 f1 54 6f 00 00           	xorl	$0x6f54, %ecx           # imm = 0x6F54
140a47092: 44 0f b6 4a 02              	movzbl	0x2(%rdx), %r9d
140a47097: 41 83 f1 70                 	xorl	$0x70, %r9d
140a4709b: 66 41 09 c9                 	orw	%cx, %r9w
140a4709f: 75 0e                       	jne	0x140a470af <.text+0xa460af>
140a470a1: c6 40 01 03                 	movb	$0x3, 0x1(%rax)
140a470a5: 31 c9                       	xorl	%ecx, %ecx
140a470a7: 88 08                       	movb	%cl, (%rax)
140a470a9: 48 83 c4 20                 	addq	$0x20, %rsp
140a470ad: 5e                          	popq	%rsi
140a470ae: c3                          	retq
140a470af: 4c 8d 15 32 7c 08 02        	leaq	0x2087c32(%rip), %r10   # 0x142acece8
140a470b6: 41 b9 04 00 00 00           	movl	$0x4, %r9d
140a470bc: 48 89 d1                    	movq	%rdx, %rcx
140a470bf: 4c 89 c2                    	movq	%r8, %rdx
140a470c2: 4d 89 d0                    	movq	%r10, %r8
140a470c5: 48 89 c6                    	movq	%rax, %rsi
140a470c8: e8 53 2d eb 01              	callq	0x1428f9e20 <.text+0x28f8e20>
140a470cd: 48 89 c1                    	movq	%rax, %rcx
140a470d0: 48 89 f0                    	movq	%rsi, %rax
140a470d3: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
140a470d7: b1 01                       	movb	$0x1, %cl
140a470d9: eb cc                       	jmp	0x140a470a7 <.text+0xa460a7>
140a470db: cc                          	int3
140a470dc: cc                          	int3
140a470dd: cc                          	int3
140a470de: cc                          	int3
140a470df: cc                          	int3
140a470e0: 48 83 ec 48                 	subq	$0x48, %rsp
140a470e4: 48 89 d0                    	movq	%rdx, %rax
140a470e7: 0f b6 11                    	movzbl	(%rcx), %edx
140a470ea: 80 c2 c8                    	addb	$-0x38, %dl
140a470ed: 80 fa 08                    	cmpb	$0x8, %dl
140a470f0: 0f b6 d2                    	movzbl	%dl, %edx
140a470f3: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140a470f9: 44 0f 42 c2                 	cmovbl	%edx, %r8d
140a470fd: 41 0f b6 d0                 	movzbl	%r8b, %edx
140a47101: 4c 8d 05 e0 85 08 02        	leaq	0x20885e0(%rip), %r8    # 0x142acf6e8
140a47108: 49 63 14 90                 	movslq	(%r8,%rdx,4), %rdx
140a4710c: 4c 01 c2                    	addq	%r8, %rdx
140a4710f: ff e2                       	jmpq	*%rdx
140a47111: 48 8d 15 10 7c 08 02        	leaq	0x2087c10(%rip), %rdx   # 0x142aced28
140a47118: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
140a4711e: 48 89 c1                    	movq	%rax, %rcx
140a47121: 48 83 c4 48                 	addq	$0x48, %rsp
140a47125: e9 66 fa 33 00              	jmp	0x140d86b90 <.text+0xd85b90>
140a4712a: 48 8d 15 20 7c 08 02        	leaq	0x2087c20(%rip), %rdx   # 0x142aced51
140a47131: eb 20                       	jmp	0x140a47153 <.text+0xa46153>
140a47133: 48 8d 15 07 7c 08 02        	leaq	0x2087c07(%rip), %rdx   # 0x142aced41
140a4713a: 41 b8 09 00 00 00           	movl	$0x9, %r8d
140a47140: 48 89 c1                    	movq	%rax, %rcx
140a47143: 48 83 c4 48                 	addq	$0x48, %rsp
140a47147: e9 44 fa 33 00              	jmp	0x140d86b90 <.text+0xd85b90>
140a4714c: 48 8d 15 f7 7b 08 02        	leaq	0x2087bf7(%rip), %rdx   # 0x142aced4a
