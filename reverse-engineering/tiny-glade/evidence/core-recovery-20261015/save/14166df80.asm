
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14166df80: 55                          	pushq	%rbp
14166df81: 56                          	pushq	%rsi
14166df82: 57                          	pushq	%rdi
14166df83: 48 81 ec 70 03 00 00        	subq	$0x370, %rsp            # imm = 0x370
14166df8a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14166df92: 0f 29 b5 e0 02 00 00        	movaps	%xmm6, 0x2e0(%rbp)
14166df99: 48 c7 85 d8 02 00 00 fe ff ff ff    	movq	$-0x2, 0x2d8(%rbp)
14166dfa4: 4c 89 c2                    	movq	%r8, %rdx
14166dfa7: 48 89 ce                    	movq	%rcx, %rsi
14166dfaa: 48 8d 7d a0                 	leaq	-0x60(%rbp), %rdi
14166dfae: 41 b8 58 02 00 00           	movl	$0x258, %r8d            # imm = 0x258
14166dfb4: 48 89 f9                    	movq	%rdi, %rcx
14166dfb7: e8 44 5c 2b 01              	callq	0x142923c00 <.text+0x2922c00>
14166dfbc: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14166dfc3: 48 89 fa                    	movq	%rdi, %rdx
14166dfc6: e8 e5 c0 6e ff              	callq	0x140d5a0b0 <.text+0xd590b0>
14166dfcb: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
14166dfd2: 48 83 f8 02                 	cmpq	$0x2, %rax
14166dfd6: 0f 85 83 00 00 00           	jne	0x14166e05f <.text+0x166d05f>
14166dfdc: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
14166dfe3: 89 c1                       	movl	%eax, %ecx
14166dfe5: 83 e1 03                    	andl	$0x3, %ecx
14166dfe8: 83 f9 01                    	cmpl	$0x1, %ecx
14166dfeb: 0f 85 27 02 00 00           	jne	0x14166e218 <.text+0x166d218>
14166dff1: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
14166dff5: 48 89 8d c0 02 00 00        	movq	%rcx, 0x2c0(%rbp)
14166dffc: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
14166e000: 48 89 8d c8 02 00 00        	movq	%rcx, 0x2c8(%rbp)
14166e007: 48 8b 40 07                 	movq	0x7(%rax), %rax
14166e00b: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
14166e012: 48 8b 00                    	movq	(%rax), %rax
14166e015: 48 85 c0                    	testq	%rax, %rax
14166e018: 74 09                       	je	0x14166e023 <.text+0x166d023>
14166e01a: 48 8b 8d c8 02 00 00        	movq	0x2c8(%rbp), %rcx
14166e021: ff d0                       	callq	*%rax
14166e023: 48 8b 8d c8 02 00 00        	movq	0x2c8(%rbp), %rcx
14166e02a: 48 8b 85 d0 02 00 00        	movq	0x2d0(%rbp), %rax
14166e031: 48 8b 50 08                 	movq	0x8(%rax), %rdx
14166e035: 48 85 d2                    	testq	%rdx, %rdx
14166e038: 74 09                       	je	0x14166e043 <.text+0x166d043>
14166e03a: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14166e03e: e8 dd 5b fa fe              	callq	0x140613c20 <.text+0x612c20>
14166e043: ba 18 00 00 00              	movl	$0x18, %edx
14166e048: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14166e04e: 48 8b 8d c0 02 00 00        	movq	0x2c0(%rbp), %rcx
14166e055: e8 c6 5b fa fe              	callq	0x140613c20 <.text+0x612c20>
14166e05a: e9 b9 01 00 00              	jmp	0x14166e218 <.text+0x166d218>
14166e05f: 48 8d 8d 58 02 00 00        	leaq	0x258(%rbp), %rcx
14166e066: 0f 10 41 40                 	movups	0x40(%rcx), %xmm0
14166e06a: 0f 11 85 40 02 00 00        	movups	%xmm0, 0x240(%rbp)
14166e071: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
14166e075: f3 0f 6f 49 10              	movdqu	0x10(%rcx), %xmm1
14166e07a: 0f 10 51 20                 	movups	0x20(%rcx), %xmm2
14166e07e: 0f 10 59 30                 	movups	0x30(%rcx), %xmm3
14166e082: 0f 11 9d 30 02 00 00        	movups	%xmm3, 0x230(%rbp)
14166e089: 0f 11 95 20 02 00 00        	movups	%xmm2, 0x220(%rbp)
14166e090: f3 0f 7f 8d 10 02 00 00     	movdqu	%xmm1, 0x210(%rbp)
14166e098: f3 0f 7f 85 00 02 00 00     	movdqu	%xmm0, 0x200(%rbp)
14166e0a0: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
14166e0a7: 8b 95 44 02 00 00           	movl	0x244(%rbp), %edx
14166e0ad: f7 c2 00 04 00 00           	testl	$0x400, %edx            # imm = 0x400
14166e0b3: 0f 94 c0                    	sete	%al
14166e0b6: f6 85 4b 02 00 00 20        	testb	$0x20, 0x24b(%rbp)
14166e0bd: 0f 94 c1                    	sete	%cl
14166e0c0: f6 c2 10                    	testb	$0x10, %dl
14166e0c3: 0f 85 4f 01 00 00           	jne	0x14166e218 <.text+0x166d218>
14166e0c9: 08 c8                       	orb	%cl, %al
14166e0cb: 0f 84 47 01 00 00           	je	0x14166e218 <.text+0x166d218>
14166e0d1: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14166e0d8: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
14166e0dc: e8 2f be 6e ff              	callq	0x140d59f10 <.text+0xd58f10>
14166e0e1: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14166e0e8: e8 e3 8d 70 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
14166e0ed: 48 89 c1                    	movq	%rax, %rcx
14166e0f0: e8 ab 46 6f ff              	callq	0x140d627a0 <.text+0xd617a0>
14166e0f5: 48 85 c0                    	testq	%rax, %rax
14166e0f8: 74 3d                       	je	0x14166e137 <.text+0x166d137>
14166e0fa: 49 89 d0                    	movq	%rdx, %r8
14166e0fd: 48 8d 8d a8 02 00 00        	leaq	0x2a8(%rbp), %rcx
14166e104: 48 89 c2                    	movq	%rax, %rdx
14166e107: e8 64 ad 6e ff              	callq	0x140d58e70 <.text+0xd57e70>
14166e10c: 83 bd a8 02 00 00 01        	cmpl	$0x1, 0x2a8(%rbp)
14166e113: 74 22                       	je	0x14166e137 <.text+0x166d137>
14166e115: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
14166e11c: 48 8b 8d b8 02 00 00        	movq	0x2b8(%rbp), %rcx
14166e123: 48 83 f9 03                 	cmpq	$0x3, %rcx
14166e127: 74 50                       	je	0x14166e179 <.text+0x166d179>
14166e129: 48 83 f9 04                 	cmpq	$0x4, %rcx
14166e12d: 75 08                       	jne	0x14166e137 <.text+0x166d137>
14166e12f: 81 38 6a 73 6f 6e           	cmpl	$0x6e6f736a, (%rax)     # imm = 0x6E6F736A
14166e135: 74 57                       	je	0x14166e18e <.text+0x166d18e>
14166e137: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14166e141: 48 89 06                    	movq	%rax, (%rsi)
14166e144: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
14166e14b: 48 85 d2                    	testq	%rdx, %rdx
14166e14e: 0f 84 d1 00 00 00           	je	0x14166e225 <.text+0x166d225>
14166e154: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14166e15b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14166e161: e8 ba 5a fa fe              	callq	0x140613c20 <.text+0x612c20>
14166e166: 48 8b 45 a0                 	movq	-0x60(%rbp), %rax
14166e16a: f0                          	lock
14166e16b: 48 ff 08                    	decq	(%rax)
14166e16e: 0f 84 bb 00 00 00           	je	0x14166e22f <.text+0x166d22f>
14166e174: e9 bf 00 00 00              	jmp	0x14166e238 <.text+0x166d238>
14166e179: 0f b7 08                    	movzwl	(%rax), %ecx
14166e17c: 81 f1 62 61 00 00           	xorl	$0x6162, %ecx           # imm = 0x6162
14166e182: 0f b6 40 02                 	movzbl	0x2(%rax), %eax
14166e186: 83 f0 6b                    	xorl	$0x6b, %eax
14166e189: 66 09 c8                    	orw	%cx, %ax
14166e18c: 75 a9                       	jne	0x14166e137 <.text+0x166d137>
14166e18e: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
14166e195: 48 85 d2                    	testq	%rdx, %rdx
14166e198: 74 12                       	je	0x14166e1ac <.text+0x166d1ac>
14166e19a: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14166e1a1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14166e1a7: e8 74 5a fa fe              	callq	0x140613c20 <.text+0x612c20>
14166e1ac: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14166e1b3: 48 8d 95 f8 01 00 00        	leaq	0x1f8(%rbp), %rdx
14166e1ba: e8 a1 bc 6e ff              	callq	0x140d59e60 <.text+0xd58e60>
14166e1bf: 83 bd 50 02 00 00 01        	cmpl	$0x1, 0x250(%rbp)
14166e1c6: 0f 85 81 00 00 00           	jne	0x14166e24d <.text+0x166d24d>
14166e1cc: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
14166e1d3: 89 c1                       	movl	%eax, %ecx
14166e1d5: 83 e1 03                    	andl	$0x3, %ecx
14166e1d8: 83 f9 01                    	cmpl	$0x1, %ecx
14166e1db: 75 3b                       	jne	0x14166e218 <.text+0x166d218>
14166e1dd: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
14166e1e1: 48 89 8d c0 02 00 00        	movq	%rcx, 0x2c0(%rbp)
14166e1e8: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
14166e1ec: 48 89 8d c8 02 00 00        	movq	%rcx, 0x2c8(%rbp)
14166e1f3: 48 8b 40 07                 	movq	0x7(%rax), %rax
14166e1f7: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
14166e1fe: 48 8b 00                    	movq	(%rax), %rax
14166e201: 48 85 c0                    	testq	%rax, %rax
14166e204: 0f 84 19 fe ff ff           	je	0x14166e023 <.text+0x166d023>
14166e20a: 48 8b 8d c8 02 00 00        	movq	0x2c8(%rbp), %rcx
14166e211: ff d0                       	callq	*%rax
14166e213: e9 0b fe ff ff              	jmp	0x14166e023 <.text+0x166d023>
14166e218: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14166e222: 48 89 06                    	movq	%rax, (%rsi)
14166e225: 48 8b 45 a0                 	movq	-0x60(%rbp), %rax
14166e229: f0                          	lock
14166e22a: 48 ff 08                    	decq	(%rax)
14166e22d: 75 09                       	jne	0x14166e238 <.text+0x166d238>
14166e22f: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
14166e233: e8 88 59 6e ff              	callq	0x140d53bc0 <.text+0xd52bc0>
14166e238: 48 89 f0                    	movq	%rsi, %rax
14166e23b: 0f 28 b5 e0 02 00 00        	movaps	0x2e0(%rbp), %xmm6
14166e242: 48 81 c4 70 03 00 00        	addq	$0x370, %rsp            # imm = 0x370
14166e249: 5f                          	popq	%rdi
14166e24a: 5e                          	popq	%rsi
14166e24b: 5d                          	popq	%rbp
14166e24c: c3                          	retq
14166e24d: f2 0f 10 b5 54 02 00 00     	movsd	0x254(%rbp), %xmm6
14166e255: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14166e25c: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
14166e260: e8 ab bc 6e ff              	callq	0x140d59f10 <.text+0xd58f10>
14166e265: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14166e26c: e8 5f 8c 70 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
14166e271: 48 89 c1                    	movq	%rax, %rcx
14166e274: e8 87 3b 6f ff              	callq	0x140d61e00 <.text+0xd60e00>
14166e279: 40 b7 03                    	movb	$0x3, %dil
14166e27c: 48 85 c0                    	testq	%rax, %rax
14166e27f: 0f 84 03 01 00 00           	je	0x14166e388 <.text+0x166d388>
14166e285: 49 89 d0                    	movq	%rdx, %r8
14166e288: 48 8d 8d a8 02 00 00        	leaq	0x2a8(%rbp), %rcx
14166e28f: 48 89 c2                    	movq	%rax, %rdx
14166e292: e8 d9 ab 6e ff              	callq	0x140d58e70 <.text+0xd57e70>
14166e297: f6 85 a8 02 00 00 01        	testb	$0x1, 0x2a8(%rbp)
14166e29e: 0f 85 e4 00 00 00           	jne	0x14166e388 <.text+0x166d388>
14166e2a4: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
14166e2ab: 48 8b 95 b8 02 00 00        	movq	0x2b8(%rbp), %rdx
14166e2b2: 48 b9 68 69 73 74 6f 72 79 2d       	movabsq	$0x2d79726f74736968, %rcx # imm = 0x2D79726F74736968
14166e2bc: 48 83 fa 0f                 	cmpq	$0xf, %rdx
14166e2c0: 73 11                       	jae	0x14166e2d3 <.text+0x166d2d3>
14166e2c2: 48 83 fa 0d                 	cmpq	$0xd, %rdx
14166e2c6: 73 5a                       	jae	0x14166e322 <.text+0x166d322>
14166e2c8: 48 83 fa 0a                 	cmpq	$0xa, %rdx
14166e2cc: 73 6a                       	jae	0x14166e338 <.text+0x166d338>
14166e2ce: e9 b5 00 00 00              	jmp	0x14166e388 <.text+0x166d388>
14166e2d3: 4c 8b 00                    	movq	(%rax), %r8
14166e2d6: 49 31 c8                    	xorq	%rcx, %r8
14166e2d9: 49 b9 2d 6d 61 6e 75 61 6c 2d       	movabsq	$0x2d6c61756e616d2d, %r9 # imm = 0x2D6C61756E616D2D
14166e2e3: 4c 33 48 07                 	xorq	0x7(%rax), %r9
14166e2e7: 4d 09 c1                    	orq	%r8, %r9
14166e2ea: 0f 84 90 00 00 00           	je	0x14166e380 <.text+0x166d380>
14166e2f0: 48 83 fa 12                 	cmpq	$0x12, %rdx
14166e2f4: 72 2c                       	jb	0x14166e322 <.text+0x166d322>
14166e2f6: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
14166e2fa: 0f b7 50 10                 	movzwl	0x10(%rax), %edx
14166e2fe: 66 0f 6e ca                 	movd	%edx, %xmm1
14166e302: 66 0f 74 05 66 a6 5c 01     	pcmpeqb	0x15ca666(%rip), %xmm0  # 0x142c38970
14166e30a: 66 0f 74 0d 6e a6 5c 01     	pcmpeqb	0x15ca66e(%rip), %xmm1  # 0x142c38980
14166e312: 66 0f db c8                 	pand	%xmm0, %xmm1
14166e316: 66 0f d7 d1                 	pmovmskb	%xmm1, %edx
14166e31a: 81 fa ff ff 00 00           	cmpl	$0xffff, %edx           # imm = 0xFFFF
14166e320: 74 63                       	je	0x14166e385 <.text+0x166d385>
14166e322: 48 33 08                    	xorq	(%rax), %rcx
14166e325: 48 ba 72 79 2d 61 75 74 6f 2d       	movabsq	$0x2d6f7475612d7972, %rdx # imm = 0x2D6F7475612D7972
14166e32f: 48 33 50 05                 	xorq	0x5(%rax), %rdx
14166e333: 48 09 ca                    	orq	%rcx, %rdx
14166e336: 74 44                       	je	0x14166e37c <.text+0x166d37c>
14166e338: 48 8b 08                    	movq	(%rax), %rcx
14166e33b: 48 0f c9                    	bswapq	%rcx
14166e33e: 48 ba 65 74 70 75 72 72 6f 63       	movabsq	$0x636f727275707465, %rdx # imm = 0x636F727275707465
14166e348: 48 39 d1                    	cmpq	%rdx, %rcx
14166e34b: 75 1a                       	jne	0x14166e367 <.text+0x166d367>
14166e34d: 0f b7 40 08                 	movzwl	0x8(%rax), %eax
14166e351: 66 c1 c0 08                 	rolw	$0x8, %ax
14166e355: 0f b7 c8                    	movzwl	%ax, %ecx
14166e358: 31 c0                       	xorl	%eax, %eax
14166e35a: 81 f9 2d 64 00 00           	cmpl	$0x642d, %ecx           # imm = 0x642D
14166e360: 74 0f                       	je	0x14166e371 <.text+0x166d371>
14166e362: ba 2d 64 00 00              	movl	$0x642d, %edx           # imm = 0x642D
14166e367: 31 c0                       	xorl	%eax, %eax
14166e369: 48 39 ca                    	cmpq	%rcx, %rdx
14166e36c: 19 c0                       	sbbl	%eax, %eax
14166e36e: 83 c8 01                    	orl	$0x1, %eax
14166e371: 31 ff                       	xorl	%edi, %edi
14166e373: 83 f8 01                    	cmpl	$0x1, %eax
14166e376: 40 80 d7 03                 	adcb	$0x3, %dil
14166e37a: eb 0c                       	jmp	0x14166e388 <.text+0x166d388>
14166e37c: 31 ff                       	xorl	%edi, %edi
14166e37e: eb 08                       	jmp	0x14166e388 <.text+0x166d388>
14166e380: 40 b7 01                    	movb	$0x1, %dil
14166e383: eb 03                       	jmp	0x14166e388 <.text+0x166d388>
14166e385: 40 b7 02                    	movb	$0x2, %dil
14166e388: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
14166e38f: 48 85 d2                    	testq	%rdx, %rdx
14166e392: 74 12                       	je	0x14166e3a6 <.text+0x166d3a6>
14166e394: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14166e39b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14166e3a1: e8 7a 58 fa fe              	callq	0x140613c20 <.text+0x612c20>
14166e3a6: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14166e3ad: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
14166e3b1: e8 5a bb 6e ff              	callq	0x140d59f10 <.text+0xd58f10>
14166e3b6: f3 0f 6f 85 50 02 00 00     	movdqu	0x250(%rbp), %xmm0
14166e3be: f3 0f 6f 8d 60 02 00 00     	movdqu	0x260(%rbp), %xmm1
14166e3c6: f3 0f 7f 4e 10              	movdqu	%xmm1, 0x10(%rsi)
14166e3cb: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
14166e3cf: 0f 13 76 20                 	movlps	%xmm6, 0x20(%rsi)
14166e3d3: 40 88 7e 28                 	movb	%dil, 0x28(%rsi)
14166e3d7: 48 8b 45 a0                 	movq	-0x60(%rbp), %rax
14166e3db: f0                          	lock
14166e3dc: 48 ff 08                    	decq	(%rax)
14166e3df: 0f 84 4a fe ff ff           	je	0x14166e22f <.text+0x166d22f>
14166e3e5: e9 4e fe ff ff              	jmp	0x14166e238 <.text+0x166d238>
