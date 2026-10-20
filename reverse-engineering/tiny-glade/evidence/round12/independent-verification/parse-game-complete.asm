
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141470d00: 55                          	pushq	%rbp
141470d01: 41 57                       	pushq	%r15
141470d03: 41 56                       	pushq	%r14
141470d05: 41 55                       	pushq	%r13
141470d07: 41 54                       	pushq	%r12
141470d09: 56                          	pushq	%rsi
141470d0a: 57                          	pushq	%rdi
141470d0b: 53                          	pushq	%rbx
141470d0c: 48 81 ec 68 04 00 00        	subq	$0x468, %rsp            # imm = 0x468
141470d13: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141470d1b: 44 0f 29 bd d0 03 00 00     	movaps	%xmm15, 0x3d0(%rbp)
141470d23: 66 44 0f 7f b5 c0 03 00 00  	movdqa	%xmm14, 0x3c0(%rbp)
141470d2c: 44 0f 29 ad b0 03 00 00     	movaps	%xmm13, 0x3b0(%rbp)
141470d34: 44 0f 29 a5 a0 03 00 00     	movaps	%xmm12, 0x3a0(%rbp)
141470d3c: 44 0f 29 9d 90 03 00 00     	movaps	%xmm11, 0x390(%rbp)
141470d44: 44 0f 29 95 80 03 00 00     	movaps	%xmm10, 0x380(%rbp)
141470d4c: 44 0f 29 8d 70 03 00 00     	movaps	%xmm9, 0x370(%rbp)
141470d54: 44 0f 29 85 60 03 00 00     	movaps	%xmm8, 0x360(%rbp)
141470d5c: 0f 29 bd 50 03 00 00        	movaps	%xmm7, 0x350(%rbp)
141470d63: 0f 29 b5 40 03 00 00        	movaps	%xmm6, 0x340(%rbp)
141470d6a: 48 c7 85 38 03 00 00 fe ff ff ff    	movq	$-0x2, 0x338(%rbp)
141470d75: 4d 89 ce                    	movq	%r9, %r14
141470d78: 4c 89 c3                    	movq	%r8, %rbx
141470d7b: 49 89 d4                    	movq	%rdx, %r12
141470d7e: 48 89 cf                    	movq	%rcx, %rdi
141470d81: 48 8d 4d 58                 	leaq	0x58(%rbp), %rcx
141470d85: e8 b6 83 8e ff              	callq	0x140d59140 <.text+0xd58140>
141470d8a: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
141470d8e: 48 8b 75 60                 	movq	0x60(%rbp), %rsi
141470d92: 48 89 c8                    	movq	%rcx, %rax
141470d95: 48 f7 d8                    	negq	%rax
141470d98: 0f 81 8a 00 00 00           	jno	0x141470e28 <.text+0x146fe28>
141470d9e: 48 89 b5 50 02 00 00        	movq	%rsi, 0x250(%rbp)
141470da5: c6 85 2e 03 00 00 01        	movb	$0x1, 0x32e(%rbp)
141470dac: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141470db3: e8 48 59 8e ff              	callq	0x140d56700 <.text+0xd55700>
141470db8: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
141470dbf: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
141470dc6: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
141470dcd: 0f 29 95 90 02 00 00        	movaps	%xmm2, 0x290(%rbp)
141470dd4: 0f 29 8d 80 02 00 00        	movaps	%xmm1, 0x280(%rbp)
141470ddb: 0f 29 85 70 02 00 00        	movaps	%xmm0, 0x270(%rbp)
141470de2: 48 8d 05 8a 36 7f 01        	leaq	0x17f368a(%rip), %rax   # 0x142c64473
141470de9: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141470df0: 48 c7 85 68 01 00 00 11 00 00 00    	movq	$0x11, 0x168(%rbp)
141470dfb: 48 89 b5 70 01 00 00        	movq	%rsi, 0x170(%rbp)
141470e02: c6 85 2e 03 00 00 00        	movb	$0x0, 0x32e(%rbp)
141470e09: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141470e10: 48 8d 95 70 02 00 00        	leaq	0x270(%rbp), %rdx
141470e17: e8 64 80 43 01              	callq	0x1428a8e80 <.text+0x28a7e80>
141470e1c: 48 89 07                    	movq	%rax, (%rdi)
141470e1f: c6 47 0f 02                 	movb	$0x2, 0xf(%rdi)
141470e23: e9 72 06 00 00              	jmp	0x14147149a <.text+0x147049a>
141470e28: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
141470e2f: 4c 8b 45 68                 	movq	0x68(%rbp), %r8
141470e33: 48 8d 4d 58                 	leaq	0x58(%rbp), %rcx
141470e37: 48 89 b5 f8 00 00 00        	movq	%rsi, 0xf8(%rbp)
141470e3e: 48 89 f2                    	movq	%rsi, %rdx
141470e41: e8 ea 4a f9 ff              	callq	0x141405930 <.text+0x1404930>
141470e46: 0f b6 45 58                 	movzbl	0x58(%rbp), %eax
141470e4a: 3c 06                       	cmpb	$0x6, %al
141470e4c: 0f 85 8e 00 00 00           	jne	0x141470ee0 <.text+0x146fee0>
141470e52: 48 8b 75 60                 	movq	0x60(%rbp), %rsi
141470e56: 48 89 b5 50 02 00 00        	movq	%rsi, 0x250(%rbp)
141470e5d: c6 85 2d 03 00 00 01        	movb	$0x1, 0x32d(%rbp)
141470e64: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141470e6b: e8 90 58 8e ff              	callq	0x140d56700 <.text+0xd55700>
141470e70: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
141470e77: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
141470e7e: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
141470e85: 0f 29 95 90 02 00 00        	movaps	%xmm2, 0x290(%rbp)
141470e8c: 0f 29 8d 80 02 00 00        	movaps	%xmm1, 0x280(%rbp)
141470e93: 0f 29 85 70 02 00 00        	movaps	%xmm0, 0x270(%rbp)
141470e9a: 48 8d 05 e3 35 7f 01        	leaq	0x17f35e3(%rip), %rax   # 0x142c64484
141470ea1: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141470ea8: 48 c7 85 68 01 00 00 0c 00 00 00    	movq	$0xc, 0x168(%rbp)
141470eb3: 48 89 b5 70 01 00 00        	movq	%rsi, 0x170(%rbp)
141470eba: c6 85 2d 03 00 00 00        	movb	$0x0, 0x32d(%rbp)
141470ec1: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141470ec8: 48 8d 95 70 02 00 00        	leaq	0x270(%rbp), %rdx
141470ecf: e8 2c 83 43 01              	callq	0x1428a9200 <.text+0x28a8200>
141470ed4: 48 89 07                    	movq	%rax, (%rdi)
141470ed7: c6 47 0f 02                 	movb	$0x2, 0xf(%rdi)
141470edb: e9 9c 05 00 00              	jmp	0x14147147c <.text+0x147047c>
141470ee0: 8b 4d 59                    	movl	0x59(%rbp), %ecx
141470ee3: 8b 55 5c                    	movl	0x5c(%rbp), %edx
141470ee6: 89 95 3c 01 00 00           	movl	%edx, 0x13c(%rbp)
141470eec: 89 8d 39 01 00 00           	movl	%ecx, 0x139(%rbp)
141470ef2: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
141470ef6: f3 0f 6f 45 68              	movdqu	0x68(%rbp), %xmm0
141470efb: f3 0f 7f 85 48 01 00 00     	movdqu	%xmm0, 0x148(%rbp)
141470f03: 88 85 38 01 00 00           	movb	%al, 0x138(%rbp)
141470f09: 48 89 8d 40 01 00 00        	movq	%rcx, 0x140(%rbp)
141470f10: 48 8d 0d 79 35 7f 01        	leaq	0x17f3579(%rip), %rcx   # 0x142c64490
141470f17: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
141470f1e: ba 07 00 00 00              	movl	$0x7, %edx
141470f23: e8 a8 35 00 00              	callq	0x1414744d0 <.text+0x14734d0>
141470f28: 48 85 c0                    	testq	%rax, %rax
141470f2b: 75 07                       	jne	0x141470f34 <.text+0x146ff34>
141470f2d: 48 8b 05 24 f8 1b 02        	movq	0x21bf824(%rip), %rax   # 0x143630758
141470f34: 0f b6 08                    	movzbl	(%rax), %ecx
141470f37: 48 8d 15 ea 3d 7f 01        	leaq	0x17f3dea(%rip), %rdx   # 0x142c64d28
141470f3e: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
141470f42: 48 01 d1                    	addq	%rdx, %rcx
141470f45: ff e1                       	jmpq	*%rcx
141470f47: c6 85 60 01 00 00 00        	movb	$0x0, 0x160(%rbp)
141470f4e: e9 ae 00 00 00              	jmp	0x141471001 <.text+0x1470001>
141470f53: 48 83 c0 08                 	addq	$0x8, %rax
141470f57: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
141470f5e: 4c 8d 05 eb 2f 7f 01        	leaq	0x17f2feb(%rip), %r8    # 0x142c63f50
141470f65: 48 89 c2                    	movq	%rax, %rdx
141470f68: e8 03 73 e8 ff              	callq	0x1412f8270 <.text+0x12f7270>
141470f6d: c6 85 60 01 00 00 04        	movb	$0x4, 0x160(%rbp)
141470f74: e9 88 00 00 00              	jmp	0x141471001 <.text+0x1470001>
141470f79: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
141470f7e: f3 0f 7f 85 68 01 00 00     	movdqu	%xmm0, 0x168(%rbp)
141470f86: c6 85 60 01 00 00 02        	movb	$0x2, 0x160(%rbp)
141470f8d: eb 72                       	jmp	0x141471001 <.text+0x1470001>
141470f8f: 48 83 c0 08                 	addq	$0x8, %rax
141470f93: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
141470f9a: 48 89 c2                    	movq	%rax, %rdx
141470f9d: e8 ee ad 90 ff              	callq	0x140d7bd90 <.text+0xd7ad90>
141470fa2: c6 85 60 01 00 00 03        	movb	$0x3, 0x160(%rbp)
141470fa9: eb 56                       	jmp	0x141471001 <.text+0x1470001>
141470fab: 0f b6 40 01                 	movzbl	0x1(%rax), %eax
141470faf: 88 85 61 01 00 00           	movb	%al, 0x161(%rbp)
141470fb5: c6 85 60 01 00 00 01        	movb	$0x1, 0x160(%rbp)
141470fbc: eb 43                       	jmp	0x141471001 <.text+0x1470001>
141470fbe: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
141470fc3: 74 1f                       	je	0x141470fe4 <.text+0x146ffe4>
141470fc5: 48 8b 50 08                 	movq	0x8(%rax), %rdx
141470fc9: 48 85 d2                    	testq	%rdx, %rdx
141470fcc: 0f 84 2e 23 00 00           	je	0x141473300 <.text+0x1472300>
141470fd2: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
141470fd9: 4c 8b 40 10                 	movq	0x10(%rax), %r8
141470fdd: e8 de b8 fc ff              	callq	0x14143c8c0 <.text+0x143b8c0>
141470fe2: eb 16                       	jmp	0x141470ffa <.text+0x146fffa>
141470fe4: 48 c7 85 68 01 00 00 00 00 00 00    	movq	$0x0, 0x168(%rbp)
141470fef: 48 c7 85 78 01 00 00 00 00 00 00    	movq	$0x0, 0x178(%rbp)
141470ffa: c6 85 60 01 00 00 05        	movb	$0x5, 0x160(%rbp)
141471001: 4c 8d bd 70 02 00 00        	leaq	0x270(%rbp), %r15
141471008: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
14147100f: 4c 89 f9                    	movq	%r15, %rcx
141471012: e8 d9 98 12 00              	callq	0x14159a8f0 <.text+0x15998f0>
141471017: f6 85 70 02 00 00 01        	testb	$0x1, 0x270(%rbp)
14147101e: 74 11                       	je	0x141471031 <.text+0x1470031>
141471020: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
141471027: e8 24 88 43 01              	callq	0x1428a9850 <.text+0x28a8850>
14147102c: e9 c7 03 00 00              	jmp	0x1414713f8 <.text+0x14703f8>
141471031: 44 8b ad 74 02 00 00        	movl	0x274(%rbp), %r13d
141471038: 44 89 ad 00 03 00 00        	movl	%r13d, 0x300(%rbp)
14147103f: 41 83 fd 03                 	cmpl	$0x3, %r13d
141471043: 0f 8d 85 00 00 00           	jge	0x1414710ce <.text+0x14700ce>
141471049: 48 8d 85 00 03 00 00        	leaq	0x300(%rbp), %rax
141471050: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
141471057: 48 8d 05 22 c5 91 ff        	leaq	-0x6e3ade(%rip), %rax   # 0x140d8d580 <.text+0xd8c580>
14147105e: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
141471065: 48 8d 0d 3c 39 7f 01        	leaq	0x17f393c(%rip), %rcx   # 0x142c649a8
14147106c: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
141471073: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
14147107a: 48 8d 05 67 39 7f 01        	leaq	0x17f3967(%rip), %rax   # 0x142c649e8
141471081: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141471088: 48 c7 85 68 01 00 00 02 00 00 00    	movq	$0x2, 0x168(%rbp)
141471093: 4c 89 bd 70 01 00 00        	movq	%r15, 0x170(%rbp)
14147109a: 48 c7 85 78 01 00 00 02 00 00 00    	movq	$0x2, 0x178(%rbp)
1414710a5: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1414710b0: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1414710b4: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1414710bb: e8 d0 8e 90 ff              	callq	0x140d79f90 <.text+0xd78f90>
1414710c0: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1414710c4: e8 17 73 43 01              	callq	0x1428a83e0 <.text+0x28a73e0>
1414710c9: e9 2a 03 00 00              	jmp	0x1414713f8 <.text+0x14703f8>
1414710ce: 41 83 fd 1b                 	cmpl	$0x1b, %r13d
1414710d2: 0f 86 85 00 00 00           	jbe	0x14147115d <.text+0x147015d>
1414710d8: 48 8d 85 00 03 00 00        	leaq	0x300(%rbp), %rax
1414710df: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1414710e6: 48 8d 05 93 c4 91 ff        	leaq	-0x6e3b6d(%rip), %rax   # 0x140d8d580 <.text+0xd8c580>
1414710ed: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
1414710f4: 48 8d 0d e5 34 7f 01        	leaq	0x17f34e5(%rip), %rcx   # 0x142c645e0
1414710fb: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
141471102: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
141471109: 48 8d 05 78 38 7f 01        	leaq	0x17f3878(%rip), %rax   # 0x142c64988
141471110: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141471117: 48 c7 85 68 01 00 00 02 00 00 00    	movq	$0x2, 0x168(%rbp)
141471122: 4c 89 bd 70 01 00 00        	movq	%r15, 0x170(%rbp)
141471129: 48 c7 85 78 01 00 00 02 00 00 00    	movq	$0x2, 0x178(%rbp)
141471134: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
14147113f: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141471143: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
14147114a: e8 41 8e 90 ff              	callq	0x140d79f90 <.text+0xd78f90>
14147114f: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141471153: e8 88 72 43 01              	callq	0x1428a83e0 <.text+0x28a73e0>
141471158: e9 9b 02 00 00              	jmp	0x1414713f8 <.text+0x14703f8>
14147115d: 48 8d 0d 33 33 7f 01        	leaq	0x17f3333(%rip), %rcx   # 0x142c64497
141471164: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
14147116b: ba 15 00 00 00              	movl	$0x15, %edx
141471170: e8 5b 33 00 00              	callq	0x1414744d0 <.text+0x14734d0>
141471175: 48 85 c0                    	testq	%rax, %rax
141471178: 75 07                       	jne	0x141471181 <.text+0x1470181>
14147117a: 48 8b 05 d7 f5 1b 02        	movq	0x21bf5d7(%rip), %rax   # 0x143630758
141471181: 0f b6 08                    	movzbl	(%rax), %ecx
141471184: 48 8d 15 b5 3b 7f 01        	leaq	0x17f3bb5(%rip), %rdx   # 0x142c64d40
14147118b: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
14147118f: 48 01 d1                    	addq	%rdx, %rcx
141471192: ff e1                       	jmpq	*%rcx
141471194: c6 85 60 01 00 00 00        	movb	$0x0, 0x160(%rbp)
14147119b: e9 82 00 00 00              	jmp	0x141471222 <.text+0x1470222>
1414711a0: 48 83 c0 08                 	addq	$0x8, %rax
1414711a4: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
1414711ab: 4c 8d 05 9e 2d 7f 01        	leaq	0x17f2d9e(%rip), %r8    # 0x142c63f50
1414711b2: 48 89 c2                    	movq	%rax, %rdx
1414711b5: e8 b6 70 e8 ff              	callq	0x1412f8270 <.text+0x12f7270>
1414711ba: c6 85 60 01 00 00 04        	movb	$0x4, 0x160(%rbp)
1414711c1: eb 5f                       	jmp	0x141471222 <.text+0x1470222>
1414711c3: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
1414711c8: f3 0f 7f 85 68 01 00 00     	movdqu	%xmm0, 0x168(%rbp)
1414711d0: c6 85 60 01 00 00 02        	movb	$0x2, 0x160(%rbp)
1414711d7: eb 49                       	jmp	0x141471222 <.text+0x1470222>
1414711d9: 48 83 c0 08                 	addq	$0x8, %rax
1414711dd: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
1414711e4: 48 89 c2                    	movq	%rax, %rdx
1414711e7: e8 a4 ab 90 ff              	callq	0x140d7bd90 <.text+0xd7ad90>
1414711ec: c6 85 60 01 00 00 03        	movb	$0x3, 0x160(%rbp)
1414711f3: eb 2d                       	jmp	0x141471222 <.text+0x1470222>
1414711f5: 0f b6 40 01                 	movzbl	0x1(%rax), %eax
1414711f9: 88 85 61 01 00 00           	movb	%al, 0x161(%rbp)
1414711ff: c6 85 60 01 00 00 01        	movb	$0x1, 0x160(%rbp)
141471206: eb 1a                       	jmp	0x141471222 <.text+0x1470222>
141471208: 48 83 c0 08                 	addq	$0x8, %rax
14147120c: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
141471213: 48 89 c2                    	movq	%rax, %rdx
141471216: e8 a5 bf fc ff              	callq	0x14143d1c0 <.text+0x143c1c0>
14147121b: c6 85 60 01 00 00 05        	movb	$0x5, 0x160(%rbp)
141471222: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
141471229: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
141471230: e8 bb 96 12 00              	callq	0x14159a8f0 <.text+0x15998f0>
141471235: f6 85 70 02 00 00 01        	testb	$0x1, 0x270(%rbp)
14147123c: 74 27                       	je	0x141471265 <.text+0x1470265>
14147123e: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
141471245: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
141471249: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14147124d: c7 45 58 01 00 00 00        	movl	$0x1, 0x58(%rbp)
141471254: c7 85 1c 01 00 00 19 00 00 00       	movl	$0x19, 0x11c(%rbp)
14147125e: e8 3d 76 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
141471263: eb 0c                       	jmp	0x141471271 <.text+0x1470271>
141471265: 8b 85 74 02 00 00           	movl	0x274(%rbp), %eax
14147126b: 89 85 1c 01 00 00           	movl	%eax, 0x11c(%rbp)
141471271: 48 8d 0d 34 32 7f 01        	leaq	0x17f3234(%rip), %rcx   # 0x142c644ac
141471278: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
14147127f: ba 06 00 00 00              	movl	$0x6, %edx
141471284: e8 47 32 00 00              	callq	0x1414744d0 <.text+0x14734d0>
141471289: 48 85 c0                    	testq	%rax, %rax
14147128c: 75 07                       	jne	0x141471295 <.text+0x1470295>
14147128e: 48 8b 05 c3 f4 1b 02        	movq	0x21bf4c3(%rip), %rax   # 0x143630758
141471295: 0f b6 08                    	movzbl	(%rax), %ecx
141471298: 48 8d 15 b9 3a 7f 01        	leaq	0x17f3ab9(%rip), %rdx   # 0x142c64d58
14147129f: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1414712a3: 48 01 d1                    	addq	%rdx, %rcx
1414712a6: ff e1                       	jmpq	*%rcx
1414712a8: c6 85 60 01 00 00 00        	movb	$0x0, 0x160(%rbp)
1414712af: e9 82 00 00 00              	jmp	0x141471336 <.text+0x1470336>
1414712b4: 48 83 c0 08                 	addq	$0x8, %rax
1414712b8: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
1414712bf: 4c 8d 05 8a 2c 7f 01        	leaq	0x17f2c8a(%rip), %r8    # 0x142c63f50
1414712c6: 48 89 c2                    	movq	%rax, %rdx
1414712c9: e8 a2 6f e8 ff              	callq	0x1412f8270 <.text+0x12f7270>
1414712ce: c6 85 60 01 00 00 04        	movb	$0x4, 0x160(%rbp)
1414712d5: eb 5f                       	jmp	0x141471336 <.text+0x1470336>
1414712d7: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
1414712dc: f3 0f 7f 85 68 01 00 00     	movdqu	%xmm0, 0x168(%rbp)
1414712e4: c6 85 60 01 00 00 02        	movb	$0x2, 0x160(%rbp)
1414712eb: eb 49                       	jmp	0x141471336 <.text+0x1470336>
1414712ed: 48 83 c0 08                 	addq	$0x8, %rax
1414712f1: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
1414712f8: 48 89 c2                    	movq	%rax, %rdx
1414712fb: e8 90 aa 90 ff              	callq	0x140d7bd90 <.text+0xd7ad90>
141471300: c6 85 60 01 00 00 03        	movb	$0x3, 0x160(%rbp)
141471307: eb 2d                       	jmp	0x141471336 <.text+0x1470336>
141471309: 0f b6 40 01                 	movzbl	0x1(%rax), %eax
14147130d: 88 85 61 01 00 00           	movb	%al, 0x161(%rbp)
141471313: c6 85 60 01 00 00 01        	movb	$0x1, 0x160(%rbp)
14147131a: eb 1a                       	jmp	0x141471336 <.text+0x1470336>
14147131c: 48 83 c0 08                 	addq	$0x8, %rax
141471320: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
141471327: 48 89 c2                    	movq	%rax, %rdx
14147132a: e8 91 be fc ff              	callq	0x14143d1c0 <.text+0x143c1c0>
14147132f: c6 85 60 01 00 00 05        	movb	$0x5, 0x160(%rbp)
141471336: 48 8d 05 fb 0e 82 01        	leaq	0x1820efb(%rip), %rax   # 0x142c92238
14147133d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141471342: 48 c7 44 24 28 09 00 00 00  	movq	$0x9, 0x28(%rsp)
14147134b: 4c 8d 05 74 0e 82 01        	leaq	0x1820e74(%rip), %r8    # 0x142c921c6
141471352: 48 8d 4d 58                 	leaq	0x58(%rbp), %rcx
141471356: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
14147135d: 41 b9 18 00 00 00           	movl	$0x18, %r9d
141471363: e8 c8 01 13 00              	callq	0x1415a1530 <.text+0x15a0530>
141471368: 83 7d 58 01                 	cmpl	$0x1, 0x58(%rbp)
14147136c: 0f 85 8d 01 00 00           	jne	0x1414714ff <.text+0x14704ff>
141471372: 48 8b 75 60                 	movq	0x60(%rbp), %rsi
141471376: 48 89 b5 50 02 00 00        	movq	%rsi, 0x250(%rbp)
14147137d: c6 85 2c 03 00 00 01        	movb	$0x1, 0x32c(%rbp)
141471384: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14147138b: e8 70 53 8e ff              	callq	0x140d56700 <.text+0xd55700>
141471390: f3 0f 6f 85 60 01 00 00     	movdqu	0x160(%rbp), %xmm0
141471398: f3 0f 6f 8d 70 01 00 00     	movdqu	0x170(%rbp), %xmm1
1414713a0: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
1414713a7: 0f 29 95 90 02 00 00        	movaps	%xmm2, 0x290(%rbp)
1414713ae: 66 0f 7f 8d 80 02 00 00     	movdqa	%xmm1, 0x280(%rbp)
1414713b6: 66 0f 7f 85 70 02 00 00     	movdqa	%xmm0, 0x270(%rbp)
1414713be: 48 8d 05 ed 30 7f 01        	leaq	0x17f30ed(%rip), %rax   # 0x142c644b2
1414713c5: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1414713cc: 48 c7 85 68 01 00 00 0e 00 00 00    	movq	$0xe, 0x168(%rbp)
1414713d7: 48 89 b5 70 01 00 00        	movq	%rsi, 0x170(%rbp)
1414713de: c6 85 2c 03 00 00 00        	movb	$0x0, 0x32c(%rbp)
1414713e5: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1414713ec: 48 8d 95 70 02 00 00        	leaq	0x270(%rbp), %rdx
1414713f3: e8 08 7e 43 01              	callq	0x1428a9200 <.text+0x28a8200>
1414713f8: 48 89 07                    	movq	%rax, (%rdi)
1414713fb: c6 47 0f 02                 	movb	$0x2, 0xf(%rdi)
1414713ff: 0f b6 85 38 01 00 00        	movzbl	0x138(%rbp), %eax
141471406: 83 f8 03                    	cmpl	$0x3, %eax
141471409: 72 71                       	jb	0x14147147c <.text+0x147047c>
14147140b: 74 43                       	je	0x141471450 <.text+0x1470450>
14147140d: 83 f8 04                    	cmpl	$0x4, %eax
141471410: 75 5e                       	jne	0x141471470 <.text+0x1470470>
141471412: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141471419: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
141471420: 48 89 8d 20 03 00 00        	movq	%rcx, 0x320(%rbp)
141471427: e8 44 79 41 ff              	callq	0x140888d70 <.text+0x887d70>
14147142c: 48 8b 95 40 01 00 00        	movq	0x140(%rbp), %rdx
141471433: 48 85 d2                    	testq	%rdx, %rdx
141471436: 74 44                       	je	0x14147147c <.text+0x147047c>
141471438: 48 c1 e2 05                 	shlq	$0x5, %rdx
14147143c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141471442: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
141471449: e8 d2 27 1a ff              	callq	0x140613c20 <.text+0x612c20>
14147144e: eb 2c                       	jmp	0x14147147c <.text+0x147047c>
141471450: 48 8b 95 40 01 00 00        	movq	0x140(%rbp), %rdx
141471457: 48 85 d2                    	testq	%rdx, %rdx
14147145a: 74 20                       	je	0x14147147c <.text+0x147047c>
14147145c: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
141471463: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141471469: e8 b2 27 1a ff              	callq	0x140613c20 <.text+0x612c20>
14147146e: eb 0c                       	jmp	0x14147147c <.text+0x147047c>
141471470: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
141471477: e8 c4 7a 42 ff              	callq	0x140898f40 <.text+0x897f40>
14147147c: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
141471483: 48 85 d2                    	testq	%rdx, %rdx
141471486: 74 12                       	je	0x14147149a <.text+0x147049a>
141471488: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14147148e: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
141471495: e8 86 27 1a ff              	callq	0x140613c20 <.text+0x612c20>
14147149a: 48 89 f8                    	movq	%rdi, %rax
14147149d: 0f 28 b5 40 03 00 00        	movaps	0x340(%rbp), %xmm6
1414714a4: 0f 28 bd 50 03 00 00        	movaps	0x350(%rbp), %xmm7
1414714ab: 44 0f 28 85 60 03 00 00     	movaps	0x360(%rbp), %xmm8
1414714b3: 44 0f 28 8d 70 03 00 00     	movaps	0x370(%rbp), %xmm9
1414714bb: 44 0f 28 95 80 03 00 00     	movaps	0x380(%rbp), %xmm10
1414714c3: 44 0f 28 9d 90 03 00 00     	movaps	0x390(%rbp), %xmm11
1414714cb: 44 0f 28 a5 a0 03 00 00     	movaps	0x3a0(%rbp), %xmm12
1414714d3: 44 0f 28 ad b0 03 00 00     	movaps	0x3b0(%rbp), %xmm13
1414714db: 44 0f 28 b5 c0 03 00 00     	movaps	0x3c0(%rbp), %xmm14
1414714e3: 44 0f 28 bd d0 03 00 00     	movaps	0x3d0(%rbp), %xmm15
1414714eb: 48 81 c4 68 04 00 00        	addq	$0x468, %rsp            # imm = 0x468
1414714f2: 5b                          	popq	%rbx
1414714f3: 5f                          	popq	%rdi
1414714f4: 5e                          	popq	%rsi
1414714f5: 41 5c                       	popq	%r12
1414714f7: 41 5d                       	popq	%r13
1414714f9: 41 5e                       	popq	%r14
1414714fb: 41 5f                       	popq	%r15
1414714fd: 5d                          	popq	%rbp
1414714fe: c3                          	retq
1414714ff: f3 44 0f 10 5d 5c           	movss	0x5c(%rbp), %xmm11
141471505: 48 8b 45 60                 	movq	0x60(%rbp), %rax
141471509: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14147150d: 48 8b 45 68                 	movq	0x68(%rbp), %rax
141471511: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
141471515: 44 0f 10 55 70              	movups	0x70(%rbp), %xmm10
14147151a: 48 8d 0d 9f 2f 7f 01        	leaq	0x17f2f9f(%rip), %rcx   # 0x142c644c0
141471521: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
141471528: ba 0b 00 00 00              	movl	$0xb, %edx
14147152d: e8 9e 2f 00 00              	callq	0x1414744d0 <.text+0x14734d0>
141471532: 48 85 c0                    	testq	%rax, %rax
141471535: 75 07                       	jne	0x14147153e <.text+0x147053e>
141471537: 48 8b 05 1a f2 1b 02        	movq	0x21bf21a(%rip), %rax   # 0x143630758
14147153e: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
141471545: 48 89 c2                    	movq	%rax, %rdx
141471548: e8 b3 b9 ff ff              	callq	0x14146cf00 <.text+0x146bf00>
14147154d: 48 8d 05 7c cd 67 01        	leaq	0x167cd7c(%rip), %rax   # 0x142aee2d0
141471554: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141471559: 48 c7 44 24 28 03 00 00 00  	movq	$0x3, 0x28(%rsp)
141471562: 4c 8d 05 2f cd 67 01        	leaq	0x167cd2f(%rip), %r8    # 0x142aee298
141471569: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141471570: 48 8d 95 70 02 00 00        	leaq	0x270(%rbp), %rdx
141471577: 41 b9 09 00 00 00           	movl	$0x9, %r9d
14147157d: e8 ae e1 12 00              	callq	0x14159f730 <.text+0x159e730>
141471582: 8b 85 60 01 00 00           	movl	0x160(%rbp), %eax
141471588: 85 c0                       	testl	%eax, %eax
14147158a: 0f 84 31 01 00 00           	je	0x1414716c1 <.text+0x14706c1>
141471590: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
141471597: e8 04 73 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
14147159c: 48 8d 0d 1d 2f 7f 01        	leaq	0x17f2f1d(%rip), %rcx   # 0x142c644c0
1414715a3: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
1414715aa: ba 0b 00 00 00              	movl	$0xb, %edx
1414715af: e8 1c 2f 00 00              	callq	0x1414744d0 <.text+0x14734d0>
1414715b4: 48 85 c0                    	testq	%rax, %rax
1414715b7: 75 07                       	jne	0x1414715c0 <.text+0x14705c0>
1414715b9: 48 8b 05 98 f1 1b 02        	movq	0x21bf198(%rip), %rax   # 0x143630758
1414715c0: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1414715c7: 48 89 c2                    	movq	%rax, %rdx
1414715ca: e8 31 b9 ff ff              	callq	0x14146cf00 <.text+0x146bf00>
1414715cf: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
1414715d6: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1414715dd: e8 8e 8d 12 00              	callq	0x14159a370 <.text+0x1599370>
1414715e2: 83 bd 70 02 00 00 00        	cmpl	$0x0, 0x270(%rbp)
1414715e9: 0f 84 15 01 00 00           	je	0x141471704 <.text+0x1470704>
1414715ef: 48 8d 8d 78 02 00 00        	leaq	0x278(%rbp), %rcx
1414715f6: e8 a5 72 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
1414715fb: 48 8b 05 a6 ad 1b 02        	movq	0x21bada6(%rip), %rax   # 0x14362c3a8
141471602: 48 8b 00                    	movq	(%rax), %rax
141471605: 44 0f 28 25 e3 26 7f 01     	movaps	0x17f26e3(%rip), %xmm12 # 0x142c63cf0
14147160d: 44 0f 28 05 eb 26 7f 01     	movaps	0x17f26eb(%rip), %xmm8  # 0x142c63d00
141471615: c7 85 f4 02 00 00 00 00 00 00       	movl	$0x0, 0x2f4(%rbp)
14147161f: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
141471623: 66 c7 85 16 03 00 00 cf 00  	movw	$0xcf, 0x316(%rbp)
14147162c: 48 83 f8 02                 	cmpq	$0x2, %rax
141471630: 0f 82 0d 01 00 00           	jb	0x141471743 <.text+0x1470743>
141471636: 48 8d 05 bb 2e 7f 01        	leaq	0x17f2ebb(%rip), %rax   # 0x142c644f8
14147163d: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141471644: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
14147164f: 48 c7 85 70 01 00 00 08 00 00 00    	movq	$0x8, 0x170(%rbp)
14147165a: 66 0f ef c0                 	pxor	%xmm0, %xmm0
14147165e: f3 0f 7f 85 78 01 00 00     	movdqu	%xmm0, 0x178(%rbp)
141471666: 48 8d 0d 9b 2e 7f 01        	leaq	0x17f2e9b(%rip), %rcx   # 0x142c64508
14147166d: e8 0e c7 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
141471672: 48 8d 0d cf 2a 7f 01        	leaq	0x17f2acf(%rip), %rcx   # 0x142c64148
141471679: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
141471680: 48 c7 85 78 02 00 00 26 00 00 00    	movq	$0x26, 0x278(%rbp)
14147168b: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
141471692: 48 c7 85 88 02 00 00 26 00 00 00    	movq	$0x26, 0x288(%rbp)
14147169d: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
1414716a4: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1414716ab: 4c 8d 85 70 02 00 00        	leaq	0x270(%rbp), %r8
1414716b2: ba 02 00 00 00              	movl	$0x2, %edx
1414716b7: e8 74 fd 03 ff              	callq	0x1404b1430 <.text+0x4b0430>
1414716bc: e9 82 00 00 00              	jmp	0x141471743 <.text+0x1470743>
1414716c1: 8b 85 64 01 00 00           	movl	0x164(%rbp), %eax
1414716c7: 89 85 f4 02 00 00           	movl	%eax, 0x2f4(%rbp)
1414716cd: 44 0f 10 85 68 01 00 00     	movups	0x168(%rbp), %xmm8
1414716d5: 44 0f 10 a5 78 01 00 00     	movups	0x178(%rbp), %xmm12
1414716dd: f3 44 0f 10 ad 88 01 00 00  	movss	0x188(%rbp), %xmm13
1414716e6: 0f b7 85 8c 01 00 00        	movzwl	0x18c(%rbp), %eax
1414716ed: 66 89 85 16 03 00 00        	movw	%ax, 0x316(%rbp)
1414716f4: 74 4d                       	je	0x141471743 <.text+0x1470743>
1414716f6: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
1414716fd: e8 9e 71 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
141471702: eb 3f                       	jmp	0x141471743 <.text+0x1470743>
141471704: f3 44 0f 10 85 74 02 00 00  	movss	0x274(%rbp), %xmm8
14147170d: 74 0c                       	je	0x14147171b <.text+0x147071b>
14147170f: 48 8d 8d 78 02 00 00        	leaq	0x278(%rbp), %rcx
141471716: e8 85 71 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
14147171b: 44 0f c6 05 bc 25 7f 01 e2  	shufps	$0xe2, 0x17f25bc(%rip), %xmm8 # xmm8 = xmm8[2,0],mem[2,3]
                                                                        # 0x142c63ce0
141471724: 44 0f 28 25 c4 25 7f 01     	movaps	0x17f25c4(%rip), %xmm12 # 0x142c63cf0
14147172c: c7 85 f4 02 00 00 00 00 00 00       	movl	$0x0, 0x2f4(%rbp)
141471736: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
14147173a: 66 c7 85 16 03 00 00 cf 00  	movw	$0xcf, 0x316(%rbp)
141471743: 48 8d 0d d6 2d 7f 01        	leaq	0x17f2dd6(%rip), %rcx   # 0x142c64520
14147174a: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
141471751: ba 05 00 00 00              	movl	$0x5, %edx
141471756: e8 75 2d 00 00              	callq	0x1414744d0 <.text+0x14734d0>
14147175b: 48 85 c0                    	testq	%rax, %rax
14147175e: 75 07                       	jne	0x141471767 <.text+0x1470767>
141471760: 48 8b 05 f1 ef 1b 02        	movq	0x21beff1(%rip), %rax   # 0x143630758
141471767: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14147176e: 48 89 c2                    	movq	%rax, %rdx
141471771: e8 8a b7 ff ff              	callq	0x14146cf00 <.text+0x146bf00>
141471776: 48 8d 05 3b 08 82 01        	leaq	0x182083b(%rip), %rax   # 0x142c91fb8
14147177d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141471782: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14147178b: 4c 8d 05 06 08 82 01        	leaq	0x1820806(%rip), %r8    # 0x142c91f98
141471792: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
141471799: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1414717a0: 41 b9 14 00 00 00           	movl	$0x14, %r9d
1414717a6: e8 95 09 13 00              	callq	0x1415a2140 <.text+0x15a1140>
1414717ab: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1414717b2: 48 8d 95 70 02 00 00        	leaq	0x270(%rbp), %rdx
1414717b9: e8 f2 b3 ff ff              	callq	0x14146cbb0 <.text+0x146bbb0>
1414717be: 31 c0                       	xorl	%eax, %eax
1414717c0: 48 3b 85 60 01 00 00        	cmpq	0x160(%rbp), %rax
1414717c7: 71 1c                       	jno	0x1414717e5 <.text+0x14707e5>
1414717c9: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1414717d3: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1414717d7: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
1414717de: e8 bd 70 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
1414717e3: eb 18                       	jmp	0x1414717fd <.text+0x14707fd>
1414717e5: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1414717ec: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1414717f0: f3 0f 6f 85 60 01 00 00     	movdqu	0x160(%rbp), %xmm0
1414717f8: 66 0f 7f 45 40              	movdqa	%xmm0, 0x40(%rbp)
1414717fd: 48 8d 0d 21 2d 7f 01        	leaq	0x17f2d21(%rip), %rcx   # 0x142c64525
141471804: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
14147180b: ba 06 00 00 00              	movl	$0x6, %edx
141471810: e8 bb 2c 00 00              	callq	0x1414744d0 <.text+0x14734d0>
141471815: 48 85 c0                    	testq	%rax, %rax
141471818: 75 07                       	jne	0x141471821 <.text+0x1470821>
14147181a: 48 8b 05 37 ef 1b 02        	movq	0x21bef37(%rip), %rax   # 0x143630758
141471821: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
141471828: 48 89 c2                    	movq	%rax, %rdx
14147182b: e8 d0 b6 ff ff              	callq	0x14146cf00 <.text+0x146bf00>
141471830: 48 8d 05 71 3f 62 01        	leaq	0x1623f71(%rip), %rax   # 0x142a957a8
141471837: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14147183c: 48 c7 44 24 28 0f 00 00 00  	movq	$0xf, 0x28(%rsp)
141471845: 4c 8d 05 ac 3e 62 01        	leaq	0x1623eac(%rip), %r8    # 0x142a956f8
14147184c: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141471853: 48 8d 95 70 02 00 00        	leaq	0x270(%rbp), %rdx
14147185a: 41 b9 17 00 00 00           	movl	$0x17, %r9d
141471860: e8 cb e8 12 00              	callq	0x1415a0130 <.text+0x159f130>
141471865: 8b 95 60 01 00 00           	movl	0x160(%rbp), %edx
14147186b: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
141471872: 83 fa 01                    	cmpl	$0x1, %edx
141471875: 0f 85 06 01 00 00           	jne	0x141471981 <.text+0x1470981>
14147187b: e8 20 70 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
141471880: 48 8b 05 21 ab 1b 02        	movq	0x21bab21(%rip), %rax   # 0x14362c3a8
141471887: 48 8b 00                    	movq	(%rax), %rax
14147188a: 48 83 f8 03                 	cmpq	$0x3, %rax
14147188e: 0f 82 86 00 00 00           	jb	0x14147191a <.text+0x147091a>
141471894: 48 8d 05 05 2d 7f 01        	leaq	0x17f2d05(%rip), %rax   # 0x142c645a0
14147189b: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1414718a2: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
1414718ad: 48 c7 85 70 01 00 00 08 00 00 00    	movq	$0x8, 0x170(%rbp)
1414718b8: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1414718bc: f3 0f 7f 85 78 01 00 00     	movdqu	%xmm0, 0x178(%rbp)
1414718c4: 48 8d 0d e5 2c 7f 01        	leaq	0x17f2ce5(%rip), %rcx   # 0x142c645b0
1414718cb: e8 b0 c4 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
1414718d0: 48 8d 0d 71 28 7f 01        	leaq	0x17f2871(%rip), %rcx   # 0x142c64148
1414718d7: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
1414718de: 48 c7 85 78 02 00 00 26 00 00 00    	movq	$0x26, 0x278(%rbp)
1414718e9: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
1414718f0: 48 c7 85 88 02 00 00 26 00 00 00    	movq	$0x26, 0x288(%rbp)
1414718fb: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
141471902: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141471909: 4c 8d 85 70 02 00 00        	leaq	0x270(%rbp), %r8
141471910: ba 03 00 00 00              	movl	$0x3, %edx
141471915: e8 16 fb 03 ff              	callq	0x1404b1430 <.text+0x4b0430>
14147191a: 41 8b 46 14                 	movl	0x14(%r14), %eax
14147191e: 89 85 20 01 00 00           	movl	%eax, 0x120(%rbp)
141471924: 66 41 0f 6e 46 18           	movd	0x18(%r14), %xmm0
14147192a: 66 41 0f 6e 4e 10           	movd	0x10(%r14), %xmm1
141471930: 66 0f 62 c8                 	punpckldq	%xmm0, %xmm1    # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
141471934: f3 44 0f 7e f1              	movq	%xmm1, %xmm14           # xmm14 = xmm1[0],zero
141471939: 0f 28 3d d0 23 7f 01        	movaps	0x17f23d0(%rip), %xmm7  # 0x142c63d10
141471940: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
141471944: c6 85 2f 03 00 00 01        	movb	$0x1, 0x32f(%rbp)
14147194b: f3 44 0f 10 3d b0 3f 4b 01  	movss	0x14b3fb0(%rip), %xmm15 # 0x142925904
141471954: c7 85 24 01 00 00 00 00 00 00       	movl	$0x0, 0x124(%rbp)
14147195e: c7 85 28 01 00 00 00 00 00 00       	movl	$0x0, 0x128(%rbp)
141471968: c7 85 2c 01 00 00 00 00 00 00       	movl	$0x0, 0x12c(%rbp)
141471972: c7 85 30 01 00 00 00 00 00 00       	movl	$0x0, 0x130(%rbp)
14147197c: e9 11 01 00 00              	jmp	0x141471a92 <.text+0x1470a92>
141471981: 0f 10 bd 64 01 00 00        	movups	0x164(%rbp), %xmm7
141471988: 8b 85 74 01 00 00           	movl	0x174(%rbp), %eax
14147198e: 89 85 24 01 00 00           	movl	%eax, 0x124(%rbp)
141471994: 0f b6 85 78 01 00 00        	movzbl	0x178(%rbp), %eax
14147199b: 89 85 28 01 00 00           	movl	%eax, 0x128(%rbp)
1414719a1: 8b 85 7c 01 00 00           	movl	0x17c(%rbp), %eax
1414719a7: 89 85 20 01 00 00           	movl	%eax, 0x120(%rbp)
1414719ad: f3 44 0f 6f b5 80 01 00 00  	movdqu	0x180(%rbp), %xmm14
1414719b6: 44 0f 10 8d 90 01 00 00     	movups	0x190(%rbp), %xmm9
1414719be: f3 44 0f 10 bd a0 01 00 00  	movss	0x1a0(%rbp), %xmm15
1414719c7: 0f b6 85 a4 01 00 00        	movzbl	0x1a4(%rbp), %eax
1414719ce: 89 85 2c 01 00 00           	movl	%eax, 0x12c(%rbp)
1414719d4: 0f b6 85 a5 01 00 00        	movzbl	0x1a5(%rbp), %eax
1414719db: 89 85 30 01 00 00           	movl	%eax, 0x130(%rbp)
1414719e1: 0f b6 85 a6 01 00 00        	movzbl	0x1a6(%rbp), %eax
1414719e8: 88 85 2f 03 00 00           	movb	%al, 0x32f(%rbp)
1414719ee: 48 8b 05 b3 a9 1b 02        	movq	0x21ba9b3(%rip), %rax   # 0x14362c3a8
1414719f5: 48 8b 00                    	movq	(%rax), %rax
1414719f8: 48 83 f8 03                 	cmpq	$0x3, %rax
1414719fc: 0f 82 87 00 00 00           	jb	0x141471a89 <.text+0x1470a89>
141471a02: 89 95 20 03 00 00           	movl	%edx, 0x320(%rbp)
141471a08: 48 8d 05 41 2b 7f 01        	leaq	0x17f2b41(%rip), %rax   # 0x142c64550
141471a0f: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
141471a16: 48 c7 85 78 02 00 00 01 00 00 00    	movq	$0x1, 0x278(%rbp)
141471a21: 48 c7 85 80 02 00 00 08 00 00 00    	movq	$0x8, 0x280(%rbp)
141471a2c: 66 0f ef c0                 	pxor	%xmm0, %xmm0
141471a30: f3 0f 7f 85 88 02 00 00     	movdqu	%xmm0, 0x288(%rbp)
141471a38: 48 8d 0d 21 2b 7f 01        	leaq	0x17f2b21(%rip), %rcx   # 0x142c64560
141471a3f: e8 3c c3 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
141471a44: 48 8d 0d fd 26 7f 01        	leaq	0x17f26fd(%rip), %rcx   # 0x142c64148
141471a4b: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
141471a4f: 48 c7 45 60 26 00 00 00     	movq	$0x26, 0x60(%rbp)
141471a57: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
141471a5b: 48 c7 45 70 26 00 00 00     	movq	$0x26, 0x70(%rbp)
141471a63: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
141471a67: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
141471a6e: 4c 8d 45 58                 	leaq	0x58(%rbp), %r8
141471a72: ba 03 00 00 00              	movl	$0x3, %edx
141471a77: e8 b4 f9 03 ff              	callq	0x1404b1430 <.text+0x4b0430>
141471a7c: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
141471a83: 8b 95 20 03 00 00           	movl	0x320(%rbp), %edx
141471a89: 85 d2                       	testl	%edx, %edx
141471a8b: 74 05                       	je	0x141471a92 <.text+0x1470a92>
141471a8d: e8 0e 6e 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
141471a92: 48 8d 0d 2f 2b 7f 01        	leaq	0x17f2b2f(%rip), %rcx   # 0x142c645c8
141471a99: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
141471aa0: ba 07 00 00 00              	movl	$0x7, %edx
141471aa5: e8 26 2a 00 00              	callq	0x1414744d0 <.text+0x14734d0>
141471aaa: 48 85 c0                    	testq	%rax, %rax
141471aad: 74 0b                       	je	0x141471aba <.text+0x1470aba>
141471aaf: 0f b6 08                    	movzbl	(%rax), %ecx
141471ab2: c6 00 00                    	movb	$0x0, (%rax)
141471ab5: 80 f9 06                    	cmpb	$0x6, %cl
141471ab8: 75 09                       	jne	0x141471ac3 <.text+0x1470ac3>
141471aba: c6 85 50 02 00 00 00        	movb	$0x0, 0x250(%rbp)
141471ac1: eb 20                       	jmp	0x141471ae3 <.text+0x1470ae3>
141471ac3: 88 8d 50 02 00 00           	movb	%cl, 0x250(%rbp)
141471ac9: f3 0f 6f 40 01              	movdqu	0x1(%rax), %xmm0
141471ace: f3 0f 6f 48 10              	movdqu	0x10(%rax), %xmm1
141471ad3: f3 0f 7f 85 51 02 00 00     	movdqu	%xmm0, 0x251(%rbp)
141471adb: f3 0f 7f 8d 60 02 00 00     	movdqu	%xmm1, 0x260(%rbp)
141471ae3: 48 8d 05 66 c5 67 01        	leaq	0x167c566(%rip), %rax   # 0x142aee050
141471aea: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141471aef: 48 c7 44 24 28 04 00 00 00  	movq	$0x4, 0x28(%rsp)
141471af8: 4c 8d 05 19 c5 67 01        	leaq	0x167c519(%rip), %r8    # 0x142aee018
141471aff: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141471b06: 4c 8d b5 50 02 00 00        	leaq	0x250(%rbp), %r14
141471b0d: 41 b9 07 00 00 00           	movl	$0x7, %r9d
141471b13: 4c 89 f2                    	movq	%r14, %rdx
141471b16: e8 15 f4 12 00              	callq	0x1415a0f30 <.text+0x159ff30>
141471b1b: 4c 8d 05 ad 2a 7f 01        	leaq	0x17f2aad(%rip), %r8    # 0x142c645cf
141471b22: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
141471b29: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
141471b30: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
141471b36: e8 95 41 13 00              	callq	0x1415a5cd0 <.text+0x15a4cd0>
141471b3b: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
141471b42: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
141471b49: 48 89 c1                    	movq	%rax, %rcx
141471b4c: 48 f7 d9                    	negq	%rcx
141471b4f: 71 39                       	jno	0x141471b8a <.text+0x1470b8a>
141471b51: 48 89 17                    	movq	%rdx, (%rdi)
141471b54: c6 47 0f 02                 	movb	$0x2, 0xf(%rdi)
141471b58: 48 8b 45 40                 	movq	0x40(%rbp), %rax
141471b5c: 48 89 c1                    	movq	%rax, %rcx
141471b5f: 48 f7 d9                    	negq	%rcx
141471b62: 0f 80 97 f8 ff ff           	jo	0x1414713ff <.text+0x14703ff>
141471b68: 0f 83 91 f8 ff ff           	jae	0x1414713ff <.text+0x14703ff>
141471b6e: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
141471b72: 48 c1 e0 03                 	shlq	$0x3, %rax
141471b76: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
141471b7a: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141471b80: e8 9b 20 1a ff              	callq	0x140613c20 <.text+0x612c20>
141471b85: e9 75 f8 ff ff              	jmp	0x1414713ff <.text+0x14703ff>
141471b8a: 48 89 5d 28                 	movq	%rbx, 0x28(%rbp)
141471b8e: 48 8b 8d e0 02 00 00        	movq	0x2e0(%rbp), %rcx
141471b95: 48 89 8d c8 00 00 00        	movq	%rcx, 0xc8(%rbp)
141471b9c: 0f 10 85 d0 02 00 00        	movups	0x2d0(%rbp), %xmm0
141471ba3: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141471baa: 0f 10 85 c0 02 00 00        	movups	0x2c0(%rbp), %xmm0
141471bb1: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141471bb8: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
141471bbf: 0f 10 8d 90 02 00 00        	movups	0x290(%rbp), %xmm1
141471bc6: 0f 10 95 a0 02 00 00        	movups	0x2a0(%rbp), %xmm2
141471bcd: 0f 10 9d b0 02 00 00        	movups	0x2b0(%rbp), %xmm3
141471bd4: 0f 11 9d 98 00 00 00        	movups	%xmm3, 0x98(%rbp)
141471bdb: 0f 11 95 88 00 00 00        	movups	%xmm2, 0x88(%rbp)
141471be2: 0f 11 4d 78                 	movups	%xmm1, 0x78(%rbp)
141471be6: 0f 11 45 68                 	movups	%xmm0, 0x68(%rbp)
141471bea: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141471bee: 48 89 55 60                 	movq	%rdx, 0x60(%rbp)
141471bf2: 8b 85 c0 00 00 00           	movl	0xc0(%rbp), %eax
141471bf8: 89 85 d4 00 00 00           	movl	%eax, 0xd4(%rbp)
141471bfe: c6 85 36 03 00 00 00        	movb	$0x0, 0x336(%rbp)
141471c05: c7 85 f0 02 00 00 00 00 00 00       	movl	$0x0, 0x2f0(%rbp)
141471c0f: b8 00 00 00 00              	movl	$0x0, %eax
141471c14: bb 00 00 00 00              	movl	$0x0, %ebx
141471c19: 41 83 fd 1b                 	cmpl	$0x1b, %r13d
141471c1d: 0f 85 19 01 00 00           	jne	0x141471d3c <.text+0x1470d3c>
141471c23: 89 85 04 03 00 00           	movl	%eax, 0x304(%rbp)
141471c29: 48 8b 05 78 a7 1b 02        	movq	0x21ba778(%rip), %rax   # 0x14362c3a8
141471c30: 48 8b 00                    	movq	(%rax), %rax
141471c33: 48 83 f8 03                 	cmpq	$0x3, %rax
141471c37: 0f 82 84 00 00 00           	jb	0x141471cc1 <.text+0x1470cc1>
141471c3d: 48 8d 05 f4 2a 7f 01        	leaq	0x17f2af4(%rip), %rax   # 0x142c64738
141471c44: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141471c4b: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
141471c56: 48 c7 85 70 01 00 00 08 00 00 00    	movq	$0x8, 0x170(%rbp)
141471c61: 0f 57 c0                    	xorps	%xmm0, %xmm0
141471c64: 0f 11 85 78 01 00 00        	movups	%xmm0, 0x178(%rbp)
141471c6b: 48 8d 0d d6 2a 7f 01        	leaq	0x17f2ad6(%rip), %rcx   # 0x142c64748
141471c72: e8 09 c1 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
141471c77: 48 8d 0d ca 24 7f 01        	leaq	0x17f24ca(%rip), %rcx   # 0x142c64148
141471c7e: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
141471c85: 48 c7 85 78 02 00 00 26 00 00 00    	movq	$0x26, 0x278(%rbp)
141471c90: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
141471c97: 48 c7 85 88 02 00 00 26 00 00 00    	movq	$0x26, 0x288(%rbp)
141471ca2: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
141471ca9: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141471cb0: 4c 8d 85 70 02 00 00        	leaq	0x270(%rbp), %r8
141471cb7: ba 03 00 00 00              	movl	$0x3, %edx
141471cbc: e8 6f f7 03 ff              	callq	0x1404b1430 <.text+0x4b0430>
141471cc1: 48 8d 0d 00 2b 7f 01        	leaq	0x17f2b00(%rip), %rcx   # 0x142c647c8
141471cc8: 4c 8d 85 38 01 00 00        	leaq	0x138(%rbp), %r8
141471ccf: ba 08 00 00 00              	movl	$0x8, %edx
141471cd4: e8 f7 27 00 00              	callq	0x1414744d0 <.text+0x14734d0>
141471cd9: 49 89 c7                    	movq	%rax, %r15
141471cdc: 48 8d 05 fd 2a 7f 01        	leaq	0x17f2afd(%rip), %rax   # 0x142c647e0
141471ce3: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141471cea: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
141471cf5: 48 c7 85 70 01 00 00 08 00 00 00    	movq	$0x8, 0x170(%rbp)
141471d00: 0f 57 c0                    	xorps	%xmm0, %xmm0
141471d03: 0f 11 85 78 01 00 00        	movups	%xmm0, 0x178(%rbp)
141471d0a: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141471d11: e8 0a 63 43 01              	callq	0x1428a8020 <.text+0x28a7020>
141471d16: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141471d1d: 4d 85 ff                    	testq	%r15, %r15
141471d20: 0f 84 b8 0d 00 00           	je	0x141472ade <.text+0x1471ade>
141471d26: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141471d2d: e8 fe ae 8d ff              	callq	0x140d4cc30 <.text+0xd4bc30>
141471d32: 31 c9                       	xorl	%ecx, %ecx
141471d34: 4c 89 f8                    	movq	%r15, %rax
141471d37: e9 a7 0d 00 00              	jmp	0x141472ae3 <.text+0x1471ae3>
141471d3c: 48 8b 05 65 a6 1b 02        	movq	0x21ba665(%rip), %rax   # 0x14362c3a8
141471d43: 48 8b 00                    	movq	(%rax), %rax
141471d46: 48 83 f8 03                 	cmpq	$0x3, %rax
141471d4a: 0f 82 c1 00 00 00           	jb	0x141471e11 <.text+0x1470e11>
141471d50: 48 8d 85 00 03 00 00        	leaq	0x300(%rbp), %rax
141471d57: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
141471d5e: 48 8d 05 1b b8 91 ff        	leaq	-0x6e47e5(%rip), %rax   # 0x140d8d580 <.text+0xd8c580>
141471d65: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
141471d6c: 48 8d 0d 6d 28 7f 01        	leaq	0x17f286d(%rip), %rcx   # 0x142c645e0
141471d73: 48 89 8d 60 02 00 00        	movq	%rcx, 0x260(%rbp)
141471d7a: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
141471d81: 48 8d 05 c0 28 7f 01        	leaq	0x17f28c0(%rip), %rax   # 0x142c64648
141471d88: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141471d8f: 48 c7 85 68 01 00 00 03 00 00 00    	movq	$0x3, 0x168(%rbp)
141471d9a: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
141471da5: 4c 89 b5 70 01 00 00        	movq	%r14, 0x170(%rbp)
141471dac: 48 c7 85 78 01 00 00 02 00 00 00    	movq	$0x2, 0x178(%rbp)
141471db7: 48 8d 0d ba 28 7f 01        	leaq	0x17f28ba(%rip), %rcx   # 0x142c64678
141471dbe: e8 bd bf 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
141471dc3: 48 8d 0d 7e 23 7f 01        	leaq	0x17f237e(%rip), %rcx   # 0x142c64148
141471dca: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
141471dd1: 48 c7 85 78 02 00 00 26 00 00 00    	movq	$0x26, 0x278(%rbp)
141471ddc: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
141471de3: 48 c7 85 88 02 00 00 26 00 00 00    	movq	$0x26, 0x288(%rbp)
141471dee: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
141471df5: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141471dfc: 4c 8d 85 70 02 00 00        	leaq	0x270(%rbp), %r8
141471e03: ba 03 00 00 00              	movl	$0x3, %edx
141471e08: e8 23 f6 03 ff              	callq	0x1404b1430 <.text+0x4b0430>
141471e0d: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
141471e11: f3 44 0f 11 bd 34 01 00 00  	movss	%xmm15, 0x134(%rbp)
141471e1a: 44 0f 29 4d f0              	movaps	%xmm9, -0x10(%rbp)
141471e1f: 0f 29 7d e0                 	movaps	%xmm7, -0x20(%rbp)
141471e23: 48 89 7d 38                 	movq	%rdi, 0x38(%rbp)
141471e27: 41 83 fd 10                 	cmpl	$0x10, %r13d
141471e2b: 0f 92 85 36 03 00 00        	setb	0x336(%rbp)
141471e32: c6 85 d8 00 00 00 01        	movb	$0x1, 0xd8(%rbp)
141471e39: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
141471e44: 48 8b 45 68                 	movq	0x68(%rbp), %rax
141471e48: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
141471e4c: 48 8d 04 c2                 	leaq	(%rdx,%rax,8), %rax
141471e50: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
141471e54: 49 c7 c2 ff ff ff ff        	movq	$-0x1, %r10
141471e5b: 49 be 0e 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff2, %r14 # imm = 0x800000000000000E
141471e65: bf 02 00 00 00              	movl	$0x2, %edi
141471e6a: f3 44 0f 10 3d 9d 05 61 01  	movss	0x161059d(%rip), %xmm15 # 0x142a82410
141471e73: f3 0f 10 35 89 3a 4b 01     	movss	0x14b3a89(%rip), %xmm6  # 0x142925904
141471e7b: f3 0f 10 3d f1 e7 4e 01     	movss	0x14ee7f1(%rip), %xmm7  # 0x142960674
141471e83: f3 44 0f 10 0d 84 ab 4d 01  	movss	0x14dab84(%rip), %xmm9  # 0x14294ca10
141471e8c: c7 85 04 03 00 00 00 00 00 00       	movl	$0x0, 0x304(%rbp)
141471e96: c7 85 f0 02 00 00 00 00 00 00       	movl	$0x0, 0x2f0(%rbp)
141471ea0: 44 89 ad 30 03 00 00        	movl	%r13d, 0x330(%rbp)
141471ea7: 4c 89 a5 10 01 00 00        	movq	%r12, 0x110(%rbp)
141471eae: 48 3b 55 20                 	cmpq	0x20(%rbp), %rdx
141471eb2: 0f 84 a6 10 00 00           	je	0x141472f5e <.text+0x1471f5e>
141471eb8: 48 89 95 f8 02 00 00        	movq	%rdx, 0x2f8(%rbp)
141471ebf: 48 8b 42 10                 	movq	0x10(%rdx), %rax
141471ec3: 48 85 c0                    	testq	%rax, %rax
141471ec6: 0f 84 65 04 00 00           	je	0x141472331 <.text+0x1471331>
141471ecc: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
141471ed3: 48 8b 59 08                 	movq	0x8(%rcx), %rbx
141471ed7: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141471edb: 4c 8d 3c c3                 	leaq	(%rbx,%rax,8), %r15
141471edf: 4c 89 7d 08                 	movq	%r15, 0x8(%rbp)
141471ee3: eb 12                       	jmp	0x141471ef7 <.text+0x1470ef7>
141471ee5: 44 89 44 08 10              	movl	%r8d, 0x10(%rax,%rcx)
141471eea: 48 83 c3 18                 	addq	$0x18, %rbx
141471eee: 4c 39 fb                    	cmpq	%r15, %rbx
141471ef1: 0f 84 3a 04 00 00           	je	0x141472331 <.text+0x1471331>
141471ef7: 48 8b 03                    	movq	(%rbx), %rax
141471efa: 48 83 f8 07                 	cmpq	$0x7, %rax
141471efe: 0f 85 43 02 00 00           	jne	0x141472147 <.text+0x1471147>
141471f04: 48 8b 73 10                 	movq	0x10(%rbx), %rsi
141471f08: 48 85 f6                    	testq	%rsi, %rsi
141471f0b: 0f 84 df 02 00 00           	je	0x1414721f0 <.text+0x14711f0>
141471f11: 48 89 5d 00                 	movq	%rbx, (%rbp)
141471f15: 4c 8b 7b 08                 	movq	0x8(%rbx), %r15
141471f19: 48 c1 e6 04                 	shlq	$0x4, %rsi
141471f1d: 4c 01 fe                    	addq	%r15, %rsi
141471f20: eb 1b                       	jmp	0x141471f3d <.text+0x1470f3d>
141471f22: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141471f30: 49 83 c7 10                 	addq	$0x10, %r15
141471f34: 49 39 f7                    	cmpq	%rsi, %r15
141471f37: 0f 84 f1 01 00 00           	je	0x14147212e <.text+0x147112e>
141471f3d: 4d 8b 67 08                 	movq	0x8(%r15), %r12
141471f41: 4d 85 e4                    	testq	%r12, %r12
141471f44: 74 ea                       	je	0x141471f30 <.text+0x1470f30>
141471f46: 49 8b 1f                    	movq	(%r15), %rbx
141471f49: 49 c1 e4 03                 	shlq	$0x3, %r12
141471f4d: 45 31 ed                    	xorl	%r13d, %r13d
141471f50: eb 1d                       	jmp	0x141471f6f <.text+0x1470f6f>
141471f52: 31 c0                       	xorl	%eax, %eax
141471f54: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141471f60: 08 85 36 03 00 00           	orb	%al, 0x336(%rbp)
141471f66: 49 83 c5 08                 	addq	$0x8, %r13
141471f6a: 4d 39 ec                    	cmpq	%r13, %r12
141471f6d: 74 c1                       	je	0x141471f30 <.text+0x1470f30>
141471f6f: 4a 8b 0c 2b                 	movq	(%rbx,%r13), %rcx
141471f73: b8 01 00 00 00              	movl	$0x1, %eax
141471f78: f0                          	lock
141471f79: 4c 0f b1 51 08              	cmpxchgq	%r10, 0x8(%rcx)
141471f7e: 0f 85 34 13 00 00           	jne	0x1414732b8 <.text+0x14722b8>
141471f84: 48 8b 01                    	movq	(%rcx), %rax
141471f87: 48 c7 41 08 01 00 00 00     	movq	$0x1, 0x8(%rcx)
141471f8f: 48 83 f8 01                 	cmpq	$0x1, %rax
141471f93: 0f 85 1f 13 00 00           	jne	0x1414732b8 <.text+0x14722b8>
141471f99: 4e 8b 04 2b                 	movq	(%rbx,%r13), %r8
141471f9d: 49 8b 48 10                 	movq	0x10(%r8), %rcx
141471fa1: 4c 39 f1                    	cmpq	%r14, %rcx
141471fa4: 0f 85 e6 00 00 00           	jne	0x141472090 <.text+0x1471090>
141471faa: 49 8d 48 10                 	leaq	0x10(%r8), %rcx
141471fae: 49 8b 40 18                 	movq	0x18(%r8), %rax
141471fb2: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
141471fb9: 45 8b 48 20                 	movl	0x20(%r8), %r9d
141471fbd: 49 8d 40 24                 	leaq	0x24(%r8), %rax
141471fc1: 48 89 85 18 03 00 00        	movq	%rax, 0x318(%rbp)
141471fc8: 41 80 78 3c 00              	cmpb	$0x0, 0x3c(%r8)
141471fcd: 48 8d 85 60 01 00 00        	leaq	0x160(%rbp), %rax
141471fd4: 48 8d 95 70 02 00 00        	leaq	0x270(%rbp), %rdx
141471fdb: 48 0f 45 c2                 	cmovneq	%rdx, %rax
141471fdf: 44 89 ca                    	movl	%r9d, %edx
141471fe2: 0f 45 d7                    	cmovnel	%edi, %edx
141471fe5: 89 95 58 01 00 00           	movl	%edx, 0x158(%rbp)
141471feb: 44 0f 44 cf                 	cmovel	%edi, %r9d
141471fef: 44 89 8d 20 03 00 00        	movl	%r9d, 0x320(%rbp)
141471ff6: 49 8b 50 34                 	movq	0x34(%r8), %rdx
141471ffa: 48 89 50 10                 	movq	%rdx, 0x10(%rax)
141471ffe: 41 0f 10 40 24              	movups	0x24(%r8), %xmm0
141472003: 0f 11 00                    	movups	%xmm0, (%rax)
141472006: 4c 89 85 08 03 00 00        	movq	%r8, 0x308(%rbp)
14147200d: e8 2e 09 61 ff              	callq	0x140a82940 <.text+0xa81940>
141472012: 48 b8 03 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
14147201c: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
141472023: 48 89 42 10                 	movq	%rax, 0x10(%rdx)
141472027: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14147202e: 48 89 42 18                 	movq	%rax, 0x18(%rdx)
141472032: 8b 85 58 01 00 00           	movl	0x158(%rbp), %eax
141472038: 89 42 20                    	movl	%eax, 0x20(%rdx)
14147203b: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
141472042: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
141472049: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
14147204d: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
141472054: 0f 11 01                    	movups	%xmm0, (%rcx)
141472057: 8b 85 20 03 00 00           	movl	0x320(%rbp), %eax
14147205d: 89 42 3c                    	movl	%eax, 0x3c(%rdx)
141472060: 0f 10 85 70 02 00 00        	movups	0x270(%rbp), %xmm0
141472067: 0f 11 42 40                 	movups	%xmm0, 0x40(%rdx)
14147206b: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
141472072: 48 89 42 50                 	movq	%rax, 0x50(%rdx)
141472076: b0 01                       	movb	$0x1, %al
141472078: 49 c7 c2 ff ff ff ff        	movq	$-0x1, %r10
14147207f: e9 dc fe ff ff              	jmp	0x141471f60 <.text+0x1470f60>
141472084: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141472090: 31 c0                       	xorl	%eax, %eax
141472092: 48 ba 10 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff0, %rdx # imm = 0x8000000000000010
14147209c: 48 39 d1                    	cmpq	%rdx, %rcx
14147209f: 0f 8c bb fe ff ff           	jl	0x141471f60 <.text+0x1470f60>
1414720a5: 41 83 b8 f8 00 00 00 02     	cmpl	$0x2, 0xf8(%r8)
1414720ad: 0f 83 ad fe ff ff           	jae	0x141471f60 <.text+0x1470f60>
1414720b3: f3 41 0f 10 80 fc 00 00 00  	movss	0xfc(%r8), %xmm0
1414720bc: 41 0f 2e 80 00 01 00 00     	ucomiss	0x100(%r8), %xmm0
1414720c4: 0f 85 88 fe ff ff           	jne	0x141471f52 <.text+0x1470f52>
1414720ca: 0f 8a 82 fe ff ff           	jp	0x141471f52 <.text+0x1470f52>
1414720d0: f3 41 0f 58 c7              	addss	%xmm15, %xmm0
1414720d5: 0f 2e c0                    	ucomiss	%xmm0, %xmm0
1414720d8: 0f 8a 14 0e 00 00           	jp	0x141472ef2 <.text+0x1471ef2>
1414720de: 66 0f 7e c0                 	movd	%xmm0, %eax
1414720e2: 25 ff ff ff 7f              	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
1414720e7: 3d 00 00 80 7f              	cmpl	$0x7f800000, %eax       # imm = 0x7F800000
1414720ec: 0f 84 24 0e 00 00           	je	0x141472f16 <.text+0x1471f16>
1414720f2: 0f 57 c9                    	xorps	%xmm1, %xmm1
1414720f5: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1414720f9: 0f 28 c6                    	movaps	%xmm6, %xmm0
1414720fc: f3 0f 5d c1                 	minss	%xmm1, %xmm0
141472100: f3 41 0f 11 80 fc 00 00 00  	movss	%xmm0, 0xfc(%r8)
141472109: f3 0f 58 c7                 	addss	%xmm7, %xmm0
14147210d: 41 0f 2e c1                 	ucomiss	%xmm9, %xmm0
141472111: 0f 83 23 0e 00 00           	jae	0x141472f3a <.text+0x1471f3a>
141472117: 0f 28 ce                    	movaps	%xmm6, %xmm1
14147211a: f3 0f 5d c8                 	minss	%xmm0, %xmm1
14147211e: f3 41 0f 11 88 00 01 00 00  	movss	%xmm1, 0x100(%r8)
141472127: b0 01                       	movb	$0x1, %al
141472129: e9 32 fe ff ff              	jmp	0x141471f60 <.text+0x1470f60>
14147212e: 48 8b 5d 00                 	movq	(%rbp), %rbx
141472132: 48 8b 03                    	movq	(%rbx), %rax
141472135: 44 8b ad 30 03 00 00        	movl	0x330(%rbp), %r13d
14147213c: 4c 8b a5 10 01 00 00        	movq	0x110(%rbp), %r12
141472143: 4c 8b 7d 08                 	movq	0x8(%rbp), %r15
141472147: 48 85 c0                    	testq	%rax, %rax
14147214a: 0f 85 a0 00 00 00           	jne	0x1414721f0 <.text+0x14711f0>
141472150: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141472154: b8 01 00 00 00              	movl	$0x1, %eax
141472159: f0                          	lock
14147215a: 4c 0f b1 51 08              	cmpxchgq	%r10, 0x8(%rcx)
14147215f: 0f 85 6b 11 00 00           	jne	0x1414732d0 <.text+0x14722d0>
141472165: 48 8b 01                    	movq	(%rcx), %rax
141472168: 48 c7 41 08 01 00 00 00     	movq	$0x1, 0x8(%rcx)
141472170: 48 83 f8 01                 	cmpq	$0x1, %rax
141472174: 0f 85 56 11 00 00           	jne	0x1414732d0 <.text+0x14722d0>
14147217a: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14147217e: 31 c9                       	xorl	%ecx, %ecx
141472180: 48 3b 48 10                 	cmpq	0x10(%rax), %rcx
141472184: 71 6a                       	jno	0x1414721f0 <.text+0x14711f0>
141472186: 48 83 78 28 02              	cmpq	$0x2, 0x28(%rax)
14147218b: 75 63                       	jne	0x1414721f0 <.text+0x14711f0>
14147218d: 48 8b 40 20                 	movq	0x20(%rax), %rax
141472191: f3 0f 10 50 08              	movss	0x8(%rax), %xmm2
141472196: f2 0f 10 48 0c              	movsd	0xc(%rax), %xmm1
14147219b: f3 0f 10 40 14              	movss	0x14(%rax), %xmm0
1414721a0: f3 0f 5c d0                 	subss	%xmm0, %xmm2
1414721a4: f2 0f 10 18                 	movsd	(%rax), %xmm3
1414721a8: 0f 5c d9                    	subps	%xmm1, %xmm3
1414721ab: 0f 59 db                    	mulps	%xmm3, %xmm3
1414721ae: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
1414721b2: f3 0f 58 e3                 	addss	%xmm3, %xmm4
1414721b6: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
1414721ba: f3 0f 58 d4                 	addss	%xmm4, %xmm2
1414721be: f3 0f 51 d2                 	sqrtss	%xmm2, %xmm2
1414721c2: 0f 2e 15 c7 e4 4e 01        	ucomiss	0x14ee4c7(%rip), %xmm2  # 0x142960690
1414721c9: 75 25                       	jne	0x1414721f0 <.text+0x14711f0>
1414721cb: 7a 23                       	jp	0x1414721f0 <.text+0x14711f0>
1414721cd: f3 0f 10 15 8b ae 55 01     	movss	0x155ae8b(%rip), %xmm2  # 0x1429cd060
1414721d5: 0f 58 ca                    	addps	%xmm2, %xmm1
1414721d8: 0f 13 48 0c                 	movlps	%xmm1, 0xc(%rax)
1414721dc: f3 0f 58 05 ac e4 4e 01     	addss	0x14ee4ac(%rip), %xmm0  # 0x142960690
1414721e4: f3 0f 11 40 14              	movss	%xmm0, 0x14(%rax)
1414721e9: c6 85 36 03 00 00 01        	movb	$0x1, 0x336(%rbp)
1414721f0: 41 83 fd 14                 	cmpl	$0x14, %r13d
1414721f4: 0f 83 f0 fc ff ff           	jae	0x141471eea <.text+0x1470eea>
1414721fa: 83 3b 09                    	cmpl	$0x9, (%rbx)
1414721fd: 0f 85 e7 fc ff ff           	jne	0x141471eea <.text+0x1470eea>
141472203: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141472207: b8 01 00 00 00              	movl	$0x1, %eax
14147220c: f0                          	lock
14147220d: 4c 0f b1 51 08              	cmpxchgq	%r10, 0x8(%rcx)
141472212: 0f 85 d0 10 00 00           	jne	0x1414732e8 <.text+0x14722e8>
141472218: 48 8b 01                    	movq	(%rcx), %rax
14147221b: 48 c7 41 08 01 00 00 00     	movq	$0x1, 0x8(%rcx)
141472223: 48 83 f8 01                 	cmpq	$0x1, %rax
141472227: 0f 85 bb 10 00 00           	jne	0x1414732e8 <.text+0x14722e8>
14147222d: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141472231: 0f b6 48 18                 	movzbl	0x18(%rax), %ecx
141472235: 83 f9 04                    	cmpl	$0x4, %ecx
141472238: 74 15                       	je	0x14147224f <.text+0x147124f>
14147223a: 83 f9 07                    	cmpl	$0x7, %ecx
14147223d: 0f 85 a7 fc ff ff           	jne	0x141471eea <.text+0x1470eea>
141472243: b9 14 00 00 00              	movl	$0x14, %ecx
141472248: ba 10 00 00 00              	movl	$0x10, %edx
14147224d: eb 0a                       	jmp	0x141472259 <.text+0x1471259>
14147224f: b9 1c 00 00 00              	movl	$0x1c, %ecx
141472254: ba 18 00 00 00              	movl	$0x18, %edx
141472259: 44 8b 4c 10 10              	movl	0x10(%rax,%rdx), %r9d
14147225e: 49 83 f9 08                 	cmpq	$0x8, %r9
141472262: 77 42                       	ja	0x1414722a6 <.text+0x14712a6>
141472264: 4c 8d 1d 05 2b 7f 01        	leaq	0x17f2b05(%rip), %r11   # 0x142c64d70
14147226b: 4f 63 04 8b                 	movslq	(%r11,%r9,4), %r8
14147226f: 4d 01 d8                    	addq	%r11, %r8
141472272: 41 ff e0                    	jmpq	*%r8
141472275: 45 89 c8                    	movl	%r9d, %r8d
141472278: 44 89 44 10 10              	movl	%r8d, 0x10(%rax,%rdx)
14147227d: 44 8b 44 08 10              	movl	0x10(%rax,%rcx), %r8d
141472282: 49 83 f8 08                 	cmpq	$0x8, %r8
141472286: 77 2f                       	ja	0x1414722b7 <.text+0x14712b7>
141472288: 4c 8d 0d 05 2b 7f 01        	leaq	0x17f2b05(%rip), %r9    # 0x142c64d94
14147228f: 4b 63 14 81                 	movslq	(%r9,%r8,4), %rdx
141472293: 4c 01 ca                    	addq	%r9, %rdx
141472296: ff e2                       	jmpq	*%rdx
141472298: ba 02 00 00 00              	movl	$0x2, %edx
14147229d: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
1414722a1: e9 44 fc ff ff              	jmp	0x141471eea <.text+0x1470eea>
1414722a6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414722ac: 41 81 f9 e7 03 00 00        	cmpl	$0x3e7, %r9d            # imm = 0x3E7
1414722b3: 74 c0                       	je	0x141472275 <.text+0x1471275>
1414722b5: eb c1                       	jmp	0x141472278 <.text+0x1471278>
1414722b7: ba 01 00 00 00              	movl	$0x1, %edx
1414722bc: 41 81 f8 e7 03 00 00        	cmpl	$0x3e7, %r8d            # imm = 0x3E7
1414722c3: 0f 84 1c fc ff ff           	je	0x141471ee5 <.text+0x1470ee5>
1414722c9: eb 5d                       	jmp	0x141472328 <.text+0x1471328>
1414722cb: 41 b8 02 00 00 00           	movl	$0x2, %r8d
1414722d1: eb a5                       	jmp	0x141472278 <.text+0x1471278>
1414722d3: 41 b8 03 00 00 00           	movl	$0x3, %r8d
1414722d9: eb 9d                       	jmp	0x141472278 <.text+0x1471278>
1414722db: ba 03 00 00 00              	movl	$0x3, %edx
1414722e0: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
1414722e4: e9 01 fc ff ff              	jmp	0x141471eea <.text+0x1470eea>
1414722e9: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1414722ef: eb 87                       	jmp	0x141472278 <.text+0x1471278>
1414722f1: ba 05 00 00 00              	movl	$0x5, %edx
1414722f6: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
1414722fa: e9 eb fb ff ff              	jmp	0x141471eea <.text+0x1470eea>
1414722ff: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141472305: e9 6e ff ff ff              	jmp	0x141472278 <.text+0x1471278>
14147230a: ba 08 00 00 00              	movl	$0x8, %edx
14147230f: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
141472313: e9 d2 fb ff ff              	jmp	0x141471eea <.text+0x1470eea>
141472318: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14147231e: e9 55 ff ff ff              	jmp	0x141472278 <.text+0x1471278>
141472323: ba 04 00 00 00              	movl	$0x4, %edx
141472328: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
14147232c: e9 b9 fb ff ff              	jmp	0x141471eea <.text+0x1470eea>
141472331: 41 83 fd 10                 	cmpl	$0x10, %r13d
141472335: 0f 83 71 01 00 00           	jae	0x1414724ac <.text+0x14714ac>
14147233b: 4c 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %r8
141472342: 49 8b 48 18                 	movq	0x18(%r8), %rcx
141472346: b8 01 00 00 00              	movl	$0x1, %eax
14147234b: 31 d2                       	xorl	%edx, %edx
14147234d: f0                          	lock
14147234e: 48 0f b1 11                 	cmpxchgq	%rdx, (%rcx)
141472352: 49 8d 70 18                 	leaq	0x18(%r8), %rsi
141472356: 4d 8b 78 18                 	movq	0x18(%r8), %r15
14147235a: 75 1a                       	jne	0x141472376 <.text+0x1471376>
14147235c: 49 8b 47 08                 	movq	0x8(%r15), %rax
141472360: 48 83 f8 01                 	cmpq	$0x1, %rax
141472364: 0f 85 50 04 00 00           	jne	0x1414727ba <.text+0x14717ba>
14147236a: 49 c7 07 01 00 00 00        	movq	$0x1, (%r15)
141472371: e9 23 01 00 00              	jmp	0x141472499 <.text+0x1471499>
141472376: 48 89 b5 18 03 00 00        	movq	%rsi, 0x318(%rbp)
14147237d: 49 8d 57 10                 	leaq	0x10(%r15), %rdx
141472381: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141472388: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
14147238f: 48 89 95 08 03 00 00        	movq	%rdx, 0x308(%rbp)
141472396: e8 d5 ad 6e ff              	callq	0x140b5d170 <.text+0xb5c170>
14147239b: 44 0f 29 85 00 01 00 00     	movaps	%xmm8, 0x100(%rbp)
1414723a3: 66 45 0f 6f c6              	movdqa	%xmm14, %xmm8
1414723a8: 45 0f 28 f5                 	movaps	%xmm13, %xmm14
1414723ac: 45 0f 28 ec                 	movaps	%xmm12, %xmm13
1414723b0: 45 0f 28 e3                 	movaps	%xmm11, %xmm12
1414723b4: 45 0f 28 da                 	movaps	%xmm10, %xmm11
1414723b8: 48 8b b5 70 02 00 00        	movq	0x270(%rbp), %rsi
1414723bf: 48 8b 9d 80 02 00 00        	movq	0x280(%rbp), %rbx
1414723c6: 4c 89 bd 20 03 00 00        	movq	%r15, 0x320(%rbp)
1414723cd: 45 0f 10 57 30              	movups	0x30(%r15), %xmm10
1414723d2: c6 85 35 03 00 00 01        	movb	$0x1, 0x335(%rbp)
1414723d9: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1414723e0: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
1414723e7: e8 54 07 70 ff              	callq	0x140b72b40 <.text+0xb71b40>
1414723ec: 48 8d 46 ff                 	leaq	-0x1(%rsi), %rax
1414723f0: 48 89 f1                    	movq	%rsi, %rcx
1414723f3: 48 31 c1                    	xorq	%rax, %rcx
1414723f6: 48 8d 56 0f                 	leaq	0xf(%rsi), %rdx
1414723fa: 48 f7 de                    	negq	%rsi
1414723fd: 48 21 d6                    	andq	%rdx, %rsi
141472400: 48 83 c6 f0                 	addq	$-0x10, %rsi
141472404: 48 39 c1                    	cmpq	%rax, %rcx
141472407: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
14147240e: 48 0f 46 f0                 	cmovbeq	%rax, %rsi
141472412: 44 0f 11 95 80 01 00 00     	movups	%xmm10, 0x180(%rbp)
14147241a: 0f 10 85 80 01 00 00        	movups	0x180(%rbp), %xmm0
141472421: 0f 11 44 33 30              	movups	%xmm0, 0x30(%rbx,%rsi)
141472426: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
14147242d: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
141472434: 0f 11 4c 33 20              	movups	%xmm1, 0x20(%rbx,%rsi)
141472439: 48 89 9d 08 03 00 00        	movq	%rbx, 0x308(%rbp)
141472440: 0f 11 44 33 10              	movups	%xmm0, 0x10(%rbx,%rsi)
141472445: f6 85 88 02 00 00 01        	testb	$0x1, 0x288(%rbp)
14147244c: 0f 84 e6 0e 00 00           	je	0x141473338 <.text+0x1472338>
141472452: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
141472459: f0                          	lock
14147245a: 48 ff 08                    	decq	(%rax)
14147245d: 45 0f 28 d3                 	movaps	%xmm11, %xmm10
141472461: 75 0c                       	jne	0x14147246f <.text+0x147146f>
141472463: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
14147246a: e8 f1 a6 6e ff              	callq	0x140b5cb60 <.text+0xb5bb60>
14147246f: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
141472476: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
14147247d: 4c 89 38                    	movq	%r15, (%rax)
141472480: 45 0f 28 dc                 	movaps	%xmm12, %xmm11
141472484: 45 0f 28 e5                 	movaps	%xmm13, %xmm12
141472488: 45 0f 28 ee                 	movaps	%xmm14, %xmm13
14147248c: 66 45 0f 6f f0              	movdqa	%xmm8, %xmm14
141472491: 44 0f 28 85 00 01 00 00     	movaps	0x100(%rbp), %xmm8
141472499: 49 83 c7 10                 	addq	$0x10, %r15
14147249d: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
1414724a4: 4c 89 f9                    	movq	%r15, %rcx
1414724a7: e8 24 77 70 ff              	callq	0x140b79bd0 <.text+0xb78bd0>
1414724ac: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
1414724b3: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1414724ba: 48 8d 95 36 03 00 00        	leaq	0x336(%rbp), %rdx
1414724c1: 4c 8d 85 00 03 00 00        	leaq	0x300(%rbp), %r8
1414724c8: e8 23 2c e8 ff              	callq	0x1412f50f0 <.text+0x12f40f0>
1414724cd: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1414724d4: 48 8d 50 28                 	leaq	0x28(%rax), %rdx
1414724d8: 41 83 fd 14                 	cmpl	$0x14, %r13d
1414724dc: 0f 83 95 03 00 00           	jae	0x141472877 <.text+0x1471877>
1414724e2: 48 8b 48 18                 	movq	0x18(%rax), %rcx
1414724e6: 48 83 79 28 00              	cmpq	$0x0, 0x28(%rcx)
1414724eb: 49 c7 c2 ff ff ff ff        	movq	$-0x1, %r10
1414724f2: 0f 84 b6 f9 ff ff           	je	0x141471eae <.text+0x1470eae>
1414724f8: b8 01 00 00 00              	movl	$0x1, %eax
1414724fd: f0                          	lock
1414724fe: 4c 0f b1 51 08              	cmpxchgq	%r10, 0x8(%rcx)
141472503: 0f 85 52 0e 00 00           	jne	0x14147335b <.text+0x147235b>
141472509: 48 8b 01                    	movq	(%rcx), %rax
14147250c: 48 c7 41 08 01 00 00 00     	movq	$0x1, 0x8(%rcx)
141472514: 48 83 f8 01                 	cmpq	$0x1, %rax
141472518: 0f 85 3d 0e 00 00           	jne	0x14147335b <.text+0x147235b>
14147251e: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
141472525: 48 8b 40 18                 	movq	0x18(%rax), %rax
141472529: 48 8b 48 28                 	movq	0x28(%rax), %rcx
14147252d: 48 85 c9                    	testq	%rcx, %rcx
141472530: 0f 84 54 03 00 00           	je	0x14147288a <.text+0x147188a>
141472536: 48 8b 58 10                 	movq	0x10(%rax), %rbx
14147253a: 48 8d 43 10                 	leaq	0x10(%rbx), %rax
14147253e: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
141472545: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
141472549: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14147254d: f7 d0                       	notl	%eax
14147254f: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
141472556: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
14147255d: eb 2c                       	jmp	0x14147258b <.text+0x147158b>
14147255f: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
141472566: 48 ff c9                    	decq	%rcx
141472569: 49 c7 c2 ff ff ff ff        	movq	$-0x1, %r10
141472570: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
141472577: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
14147257e: 48 8b 9d 08 03 00 00        	movq	0x308(%rbp), %rbx
141472585: 0f 84 f3 02 00 00           	je	0x14147287e <.text+0x147187e>
14147258b: 4c 8b 85 20 03 00 00        	movq	0x320(%rbp), %r8
141472592: 66 45 85 c0                 	testw	%r8w, %r8w
141472596: 48 89 8d 18 03 00 00        	movq	%rcx, 0x318(%rbp)
14147259d: 74 09                       	je	0x1414725a8 <.text+0x14715a8>
14147259f: 41 8d 40 ff                 	leal	-0x1(%r8), %eax
1414725a3: 44 21 c0                    	andl	%r8d, %eax
1414725a6: eb 3b                       	jmp	0x1414725e3 <.text+0x14715e3>
1414725a8: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
1414725af: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
1414725b3: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
1414725b8: 48 81 c3 00 f9 ff ff        	addq	$-0x700, %rbx           # imm = 0xF900
1414725bf: 48 83 c0 10                 	addq	$0x10, %rax
1414725c3: 41 81 f8 ff ff 00 00        	cmpl	$0xffff, %r8d           # imm = 0xFFFF
1414725ca: 74 e3                       	je	0x1414725af <.text+0x14715af>
1414725cc: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1414725d3: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1414725d8: 44 29 c1                    	subl	%r8d, %ecx
1414725db: 41 f7 d0                    	notl	%r8d
1414725de: 44 89 c0                    	movl	%r8d, %eax
1414725e1: 21 c8                       	andl	%ecx, %eax
1414725e3: 44 89 c1                    	movl	%r8d, %ecx
1414725e6: f3 0f bc c9                 	tzcntl	%ecx, %ecx
1414725ea: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
1414725f1: 48 f7 d9                    	negq	%rcx
1414725f4: 48 6b c9 70                 	imulq	$0x70, %rcx, %rcx
1414725f8: 4c 8b 44 0b a0              	movq	-0x60(%rbx,%rcx), %r8
1414725fd: 4c 8b 54 0b b8              	movq	-0x48(%rbx,%rcx), %r10
141472602: 66 41 0f 6f 00              	movdqa	(%r8), %xmm0
141472607: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
14147260b: f7 d2                       	notl	%edx
14147260d: 4d 8d 58 10                 	leaq	0x10(%r8), %r11
141472611: 48 8b 74 0b d0              	movq	-0x30(%rbx,%rcx), %rsi
141472616: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
14147261a: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
14147261f: 41 f7 d1                    	notl	%r9d
141472622: 48 8d 46 10                 	leaq	0x10(%rsi), %rax
141472626: 48 89 9d 08 03 00 00        	movq	%rbx, 0x308(%rbp)
14147262d: 48 8b 4c 0b e8              	movq	-0x18(%rbx,%rcx), %rcx
141472632: 4d 85 c0                    	testq	%r8, %r8
141472635: 75 47                       	jne	0x14147267e <.text+0x147167e>
141472637: e9 a0 00 00 00              	jmp	0x1414726dc <.text+0x14716dc>
14147263c: 45 8d 61 ff                 	leal	-0x1(%r9), %r12d
141472640: 45 21 cc                    	andl	%r9d, %r12d
141472643: 48 ff c9                    	decq	%rcx
141472646: 45 89 cf                    	movl	%r9d, %r15d
141472649: 48 89 f3                    	movq	%rsi, %rbx
14147264c: 45 31 c0                    	xorl	%r8d, %r8d
14147264f: 45 89 e1                    	movl	%r12d, %r9d
141472652: f3 45 0f bc ff              	tzcntl	%r15d, %r15d
141472657: 49 f7 df                    	negq	%r15
14147265a: 4d 6b ff 58                 	imulq	$0x58, %r15, %r15
14147265e: 46 0f b6 64 3b fa           	movzbl	-0x6(%rbx,%r15), %r12d
141472664: 41 80 c4 c8                 	addb	$-0x38, %r12b
141472668: 41 80 fc 02                 	cmpb	$0x2, %r12b
14147266c: 4c 8b a5 10 01 00 00        	movq	0x110(%rbp), %r12
141472673: 0f 86 bb 00 00 00           	jbe	0x141472734 <.text+0x1471734>
141472679: 4d 85 c0                    	testq	%r8, %r8
14147267c: 74 5e                       	je	0x1414726dc <.text+0x14716dc>
14147267e: 4d 85 d2                    	testq	%r10, %r10
141472681: 74 56                       	je	0x1414726d9 <.text+0x14716d9>
141472683: 66 85 d2                    	testw	%dx, %dx
141472686: 74 18                       	je	0x1414726a0 <.text+0x14716a0>
141472688: 44 8d 62 ff                 	leal	-0x1(%rdx), %r12d
14147268c: 41 21 d4                    	andl	%edx, %r12d
14147268f: 49 ff ca                    	decq	%r10
141472692: 41 89 d7                    	movl	%edx, %r15d
141472695: 4c 89 c3                    	movq	%r8, %rbx
141472698: 44 89 e2                    	movl	%r12d, %edx
14147269b: eb b5                       	jmp	0x141472652 <.text+0x1471652>
14147269d: 0f 1f 00                    	nopl	(%rax)
1414726a0: 66 41 0f 6f 03              	movdqa	(%r11), %xmm0
1414726a5: 66 44 0f d7 f8              	pmovmskb	%xmm0, %r15d
1414726aa: 49 81 c0 80 fa ff ff        	addq	$-0x580, %r8            # imm = 0xFA80
1414726b1: 49 83 c3 10                 	addq	$0x10, %r11
1414726b5: 41 81 ff ff ff 00 00        	cmpl	$0xffff, %r15d          # imm = 0xFFFF
1414726bc: 74 e2                       	je	0x1414726a0 <.text+0x14716a0>
1414726be: bb fe ff ff ff              	movl	$0xfffffffe, %ebx       # imm = 0xFFFFFFFE
1414726c3: 44 29 fb                    	subl	%r15d, %ebx
1414726c6: 41 f7 d7                    	notl	%r15d
1414726c9: 44 89 fa                    	movl	%r15d, %edx
1414726cc: 21 da                       	andl	%ebx, %edx
1414726ce: 49 ff ca                    	decq	%r10
1414726d1: 4c 89 c3                    	movq	%r8, %rbx
1414726d4: e9 79 ff ff ff              	jmp	0x141472652 <.text+0x1471652>
1414726d9: 45 31 d2                    	xorl	%r10d, %r10d
1414726dc: 48 85 f6                    	testq	%rsi, %rsi
1414726df: 0f 84 7a fe ff ff           	je	0x14147255f <.text+0x147155f>
1414726e5: 48 85 c9                    	testq	%rcx, %rcx
1414726e8: 0f 84 71 fe ff ff           	je	0x14147255f <.text+0x147155f>
1414726ee: 66 45 85 c9                 	testw	%r9w, %r9w
1414726f2: 0f 85 44 ff ff ff           	jne	0x14147263c <.text+0x147163c>
1414726f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141472700: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
141472704: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
141472709: 48 81 c6 80 fa ff ff        	addq	$-0x580, %rsi           # imm = 0xFA80
141472710: 48 83 c0 10                 	addq	$0x10, %rax
141472714: 41 81 f9 ff ff 00 00        	cmpl	$0xffff, %r9d           # imm = 0xFFFF
14147271b: 74 e3                       	je	0x141472700 <.text+0x1471700>
14147271d: 41 b8 fe ff ff ff           	movl	$0xfffffffe, %r8d       # imm = 0xFFFFFFFE
141472723: 45 29 c8                    	subl	%r9d, %r8d
141472726: 41 f7 d1                    	notl	%r9d
141472729: 45 89 cc                    	movl	%r9d, %r12d
14147272c: 45 21 c4                    	andl	%r8d, %r12d
14147272f: e9 0f ff ff ff              	jmp	0x141472643 <.text+0x1471643>
141472734: 4c 01 fb                    	addq	%r15, %rbx
141472737: 44 8b 63 d8                 	movl	-0x28(%rbx), %r12d
14147273b: 49 83 fc 08                 	cmpq	$0x8, %r12
14147273f: 77 17                       	ja	0x141472758 <.text+0x1471758>
141472741: 4c 8d 2d 70 26 7f 01        	leaq	0x17f2670(%rip), %r13   # 0x142c64db8
141472748: 4f 63 7c a5 00              	movslq	(%r13,%r12,4), %r15
14147274d: 4d 01 ef                    	addq	%r13, %r15
141472750: 41 ff e7                    	jmpq	*%r15
141472753: 45 89 e7                    	movl	%r12d, %r15d
141472756: eb 3a                       	jmp	0x141472792 <.text+0x1471792>
141472758: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14147275e: 41 81 fc e7 03 00 00        	cmpl	$0x3e7, %r12d           # imm = 0x3E7
141472765: 75 32                       	jne	0x141472799 <.text+0x1471799>
141472767: 45 89 e7                    	movl	%r12d, %r15d
14147276a: eb 2d                       	jmp	0x141472799 <.text+0x1471799>
14147276c: 41 bf 02 00 00 00           	movl	$0x2, %r15d
141472772: eb 1e                       	jmp	0x141472792 <.text+0x1471792>
141472774: 41 bf 03 00 00 00           	movl	$0x3, %r15d
14147277a: eb 16                       	jmp	0x141472792 <.text+0x1471792>
14147277c: 41 bf 05 00 00 00           	movl	$0x5, %r15d
141472782: eb 0e                       	jmp	0x141472792 <.text+0x1471792>
141472784: 41 bf 08 00 00 00           	movl	$0x8, %r15d
14147278a: eb 06                       	jmp	0x141472792 <.text+0x1471792>
14147278c: 41 bf 04 00 00 00           	movl	$0x4, %r15d
141472792: 44 8b ad 30 03 00 00        	movl	0x330(%rbp), %r13d
141472799: 44 89 7b d8                 	movl	%r15d, -0x28(%rbx)
14147279d: b3 01                       	movb	$0x1, %bl
14147279f: 89 9d f0 02 00 00           	movl	%ebx, 0x2f0(%rbp)
1414727a5: 4c 8b a5 10 01 00 00        	movq	0x110(%rbp), %r12
1414727ac: 4d 85 c0                    	testq	%r8, %r8
1414727af: 0f 85 c9 fe ff ff           	jne	0x14147267e <.text+0x147167e>
1414727b5: e9 22 ff ff ff              	jmp	0x1414726dc <.text+0x14716dc>
1414727ba: 49 8d 47 08                 	leaq	0x8(%r15), %rax
1414727be: 48 89 85 18 03 00 00        	movq	%rax, 0x318(%rbp)
1414727c5: 4c 89 bd 20 03 00 00        	movq	%r15, 0x320(%rbp)
1414727cc: 49 8d 5f 10                 	leaq	0x10(%r15), %rbx
1414727d0: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1414727d7: 48 89 da                    	movq	%rbx, %rdx
1414727da: e8 91 a9 6e ff              	callq	0x140b5d170 <.text+0xb5c170>
1414727df: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1414727e6: 4c 8b bd 70 01 00 00        	movq	0x170(%rbp), %r15
1414727ed: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
1414727f1: 48 89 c2                    	movq	%rax, %rdx
1414727f4: 48 31 ca                    	xorq	%rcx, %rdx
1414727f7: 4c 8d 40 0f                 	leaq	0xf(%rax), %r8
1414727fb: 48 f7 d8                    	negq	%rax
1414727fe: 4c 21 c0                    	andq	%r8, %rax
141472801: 48 83 c0 f0                 	addq	$-0x10, %rax
141472805: 48 39 ca                    	cmpq	%rcx, %rdx
141472808: 48 c7 c1 ff ff ff ff        	movq	$-0x1, %rcx
14147280f: 48 0f 46 c1                 	cmovbeq	%rcx, %rax
141472813: 0f 10 03                    	movups	(%rbx), %xmm0
141472816: 0f 10 4b 10                 	movups	0x10(%rbx), %xmm1
14147281a: 0f 10 53 20                 	movups	0x20(%rbx), %xmm2
14147281e: 41 0f 11 54 07 30           	movups	%xmm2, 0x30(%r15,%rax)
141472824: 41 0f 11 4c 07 20           	movups	%xmm1, 0x20(%r15,%rax)
14147282a: 41 0f 11 44 07 10           	movups	%xmm0, 0x10(%r15,%rax)
141472830: f6 85 78 01 00 00 01        	testb	$0x1, 0x178(%rbp)
141472837: 0f 84 10 0b 00 00           	je	0x14147334d <.text+0x147234d>
14147283d: 4c 89 3e                    	movq	%r15, (%rsi)
141472840: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
141472847: 48 83 f9 ff                 	cmpq	$-0x1, %rcx
14147284b: 0f 84 48 fc ff ff           	je	0x141472499 <.text+0x1471499>
141472851: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
141472858: f0                          	lock
141472859: 48 ff 08                    	decq	(%rax)
14147285c: 0f 85 37 fc ff ff           	jne	0x141472499 <.text+0x1471499>
141472862: ba 40 00 00 00              	movl	$0x40, %edx
141472867: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14147286d: e8 ae 13 1a ff              	callq	0x140613c20 <.text+0x612c20>
141472872: e9 22 fc ff ff              	jmp	0x141472499 <.text+0x1471499>
141472877: 49 c7 c2 ff ff ff ff        	movq	$-0x1, %r10
14147287e: 41 83 fd 15                 	cmpl	$0x15, %r13d
141472882: 0f 83 26 f6 ff ff           	jae	0x141471eae <.text+0x1470eae>
141472888: eb 07                       	jmp	0x141472891 <.text+0x1471891>
14147288a: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
141472891: 48 8b 48 18                 	movq	0x18(%rax), %rcx
141472895: 48 83 79 28 00              	cmpq	$0x0, 0x28(%rcx)
14147289a: 0f 84 0e f6 ff ff           	je	0x141471eae <.text+0x1470eae>
1414728a0: b8 01 00 00 00              	movl	$0x1, %eax
1414728a5: f0                          	lock
1414728a6: 4c 0f b1 51 08              	cmpxchgq	%r10, 0x8(%rcx)
1414728ab: 0f 85 bf 0a 00 00           	jne	0x141473370 <.text+0x1472370>
1414728b1: 48 8b 01                    	movq	(%rcx), %rax
1414728b4: 48 c7 41 08 01 00 00 00     	movq	$0x1, 0x8(%rcx)
1414728bc: 48 83 f8 01                 	cmpq	$0x1, %rax
1414728c0: 0f 85 aa 0a 00 00           	jne	0x141473370 <.text+0x1472370>
1414728c6: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1414728cd: 48 8b 40 18                 	movq	0x18(%rax), %rax
1414728d1: 48 8b 48 28                 	movq	0x28(%rax), %rcx
1414728d5: 48 85 c9                    	testq	%rcx, %rcx
1414728d8: 0f 84 d0 f5 ff ff           	je	0x141471eae <.text+0x1470eae>
1414728de: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
1414728e5: 48 8b 40 10                 	movq	0x10(%rax), %rax
1414728e9: 48 8d 50 10                 	leaq	0x10(%rax), %rdx
1414728ed: 48 89 95 08 03 00 00        	movq	%rdx, 0x308(%rbp)
1414728f4: 48 89 85 18 03 00 00        	movq	%rax, 0x318(%rbp)
1414728fb: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
1414728ff: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
141472903: f7 d0                       	notl	%eax
141472905: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
14147290c: 49 89 cd                    	movq	%rcx, %r13
14147290f: eb 09                       	jmp	0x14147291a <.text+0x147191a>
141472911: 49 ff cd                    	decq	%r13
141472914: 0f 84 a3 01 00 00           	je	0x141472abd <.text+0x1471abd>
14147291a: 48 8b 95 20 03 00 00        	movq	0x320(%rbp), %rdx
141472921: 66 85 d2                    	testw	%dx, %dx
141472924: 74 07                       	je	0x14147292d <.text+0x147192d>
141472926: 8d 42 ff                    	leal	-0x1(%rdx), %eax
141472929: 21 d0                       	andl	%edx, %eax
14147292b: eb 43                       	jmp	0x141472970 <.text+0x1471970>
14147292d: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
141472934: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
14147293b: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
14147293f: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
141472943: 48 05 00 f9 ff ff           	addq	$-0x700, %rax           # imm = 0xF900
141472949: 48 83 c1 10                 	addq	$0x10, %rcx
14147294d: 81 fa ff ff 00 00           	cmpl	$0xffff, %edx           # imm = 0xFFFF
141472953: 74 e6                       	je	0x14147293b <.text+0x147193b>
141472955: 48 89 8d 08 03 00 00        	movq	%rcx, 0x308(%rbp)
14147295c: 48 89 85 18 03 00 00        	movq	%rax, 0x318(%rbp)
141472963: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
141472968: 29 d1                       	subl	%edx, %ecx
14147296a: f7 d2                       	notl	%edx
14147296c: 89 d0                       	movl	%edx, %eax
14147296e: 21 c8                       	andl	%ecx, %eax
141472970: 89 d1                       	movl	%edx, %ecx
141472972: f3 0f bc c9                 	tzcntl	%ecx, %ecx
141472976: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
14147297d: 48 f7 d9                    	negq	%rcx
141472980: 48 6b c1 70                 	imulq	$0x70, %rcx, %rax
141472984: 48 8b 9d 18 03 00 00        	movq	0x318(%rbp), %rbx
14147298b: 4c 8b 44 03 a0              	movq	-0x60(%rbx,%rax), %r8
141472990: 48 8b 4c 03 b8              	movq	-0x48(%rbx,%rax), %rcx
141472995: 66 41 0f 6f 00              	movdqa	(%r8), %xmm0
14147299a: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
14147299e: f7 d2                       	notl	%edx
1414729a0: 4d 8d 50 10                 	leaq	0x10(%r8), %r10
1414729a4: 4c 8b 5c 03 d0              	movq	-0x30(%rbx,%rax), %r11
1414729a9: 66 41 0f 6f 03              	movdqa	(%r11), %xmm0
1414729ae: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
1414729b3: 41 f7 d1                    	notl	%r9d
1414729b6: 49 8d 73 10                 	leaq	0x10(%r11), %rsi
1414729ba: 48 8b 44 03 e8              	movq	-0x18(%rbx,%rax), %rax
1414729bf: 4d 85 c0                    	testq	%r8, %r8
1414729c2: 75 37                       	jne	0x1414729fb <.text+0x14719fb>
1414729c4: e9 86 00 00 00              	jmp	0x141472a4f <.text+0x1471a4f>
1414729c9: 45 8d 61 ff                 	leal	-0x1(%r9), %r12d
1414729cd: 45 21 cc                    	andl	%r9d, %r12d
1414729d0: 48 ff c8                    	decq	%rax
1414729d3: 44 89 cb                    	movl	%r9d, %ebx
1414729d6: 4d 89 df                    	movq	%r11, %r15
1414729d9: 45 31 c0                    	xorl	%r8d, %r8d
1414729dc: 45 89 e1                    	movl	%r12d, %r9d
1414729df: f3 0f bc db                 	tzcntl	%ebx, %ebx
1414729e3: 48 f7 db                    	negq	%rbx
1414729e6: 48 6b db 58                 	imulq	$0x58, %rbx, %rbx
1414729ea: 41 80 7c 1f fa 3a           	cmpb	$0x3a, -0x6(%r15,%rbx)
1414729f0: 0f 84 a9 00 00 00           	je	0x141472a9f <.text+0x1471a9f>
1414729f6: 4d 85 c0                    	testq	%r8, %r8
1414729f9: 74 54                       	je	0x141472a4f <.text+0x1471a4f>
1414729fb: 48 85 c9                    	testq	%rcx, %rcx
1414729fe: 74 4d                       	je	0x141472a4d <.text+0x1471a4d>
141472a00: 66 85 d2                    	testw	%dx, %dx
141472a03: 74 14                       	je	0x141472a19 <.text+0x1471a19>
141472a05: 44 8d 62 ff                 	leal	-0x1(%rdx), %r12d
141472a09: 41 21 d4                    	andl	%edx, %r12d
141472a0c: 48 ff c9                    	decq	%rcx
141472a0f: 89 d3                       	movl	%edx, %ebx
141472a11: 4d 89 c7                    	movq	%r8, %r15
141472a14: 44 89 e2                    	movl	%r12d, %edx
141472a17: eb c6                       	jmp	0x1414729df <.text+0x14719df>
141472a19: 66 41 0f 6f 02              	movdqa	(%r10), %xmm0
141472a1e: 66 0f d7 d8                 	pmovmskb	%xmm0, %ebx
141472a22: 49 81 c0 80 fa ff ff        	addq	$-0x580, %r8            # imm = 0xFA80
141472a29: 49 83 c2 10                 	addq	$0x10, %r10
141472a2d: 81 fb ff ff 00 00           	cmpl	$0xffff, %ebx           # imm = 0xFFFF
141472a33: 74 e4                       	je	0x141472a19 <.text+0x1471a19>
141472a35: 41 bf fe ff ff ff           	movl	$0xfffffffe, %r15d      # imm = 0xFFFFFFFE
141472a3b: 41 29 df                    	subl	%ebx, %r15d
141472a3e: f7 d3                       	notl	%ebx
141472a40: 89 da                       	movl	%ebx, %edx
141472a42: 44 21 fa                    	andl	%r15d, %edx
141472a45: 48 ff c9                    	decq	%rcx
141472a48: 4d 89 c7                    	movq	%r8, %r15
141472a4b: eb 92                       	jmp	0x1414729df <.text+0x14719df>
141472a4d: 31 c9                       	xorl	%ecx, %ecx
141472a4f: 4d 85 db                    	testq	%r11, %r11
141472a52: 0f 84 b9 fe ff ff           	je	0x141472911 <.text+0x1471911>
141472a58: 48 85 c0                    	testq	%rax, %rax
141472a5b: 0f 84 b0 fe ff ff           	je	0x141472911 <.text+0x1471911>
141472a61: 66 45 85 c9                 	testw	%r9w, %r9w
141472a65: 0f 85 5e ff ff ff           	jne	0x1414729c9 <.text+0x14719c9>
141472a6b: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
141472a6f: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
141472a74: 49 81 c3 80 fa ff ff        	addq	$-0x580, %r11           # imm = 0xFA80
141472a7b: 48 83 c6 10                 	addq	$0x10, %rsi
141472a7f: 41 81 f9 ff ff 00 00        	cmpl	$0xffff, %r9d           # imm = 0xFFFF
141472a86: 74 e3                       	je	0x141472a6b <.text+0x1471a6b>
141472a88: 41 b8 fe ff ff ff           	movl	$0xfffffffe, %r8d       # imm = 0xFFFFFFFE
141472a8e: 45 29 c8                    	subl	%r9d, %r8d
141472a91: 41 f7 d1                    	notl	%r9d
141472a94: 45 89 cc                    	movl	%r9d, %r12d
141472a97: 45 21 c4                    	andl	%r8d, %r12d
141472a9a: e9 31 ff ff ff              	jmp	0x1414729d0 <.text+0x14719d0>
141472a9f: 49 01 df                    	addq	%rbx, %r15
141472aa2: 41 c7 47 d8 e7 03 00 00     	movl	$0x3e7, -0x28(%r15)     # imm = 0x3E7
141472aaa: b3 01                       	movb	$0x1, %bl
141472aac: 89 9d 04 03 00 00           	movl	%ebx, 0x304(%rbp)
141472ab2: 4d 85 c0                    	testq	%r8, %r8
141472ab5: 0f 85 40 ff ff ff           	jne	0x1414729fb <.text+0x14719fb>
141472abb: eb 92                       	jmp	0x141472a4f <.text+0x1471a4f>
141472abd: 44 8b ad 30 03 00 00        	movl	0x330(%rbp), %r13d
141472ac4: 4c 8b a5 10 01 00 00        	movq	0x110(%rbp), %r12
141472acb: 49 c7 c2 ff ff ff ff        	movq	$-0x1, %r10
141472ad2: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
141472ad9: e9 d0 f3 ff ff              	jmp	0x141471eae <.text+0x1470eae>
141472ade: b9 01 00 00 00              	movl	$0x1, %ecx
141472ae3: 48 89 c2                    	movq	%rax, %rdx
141472ae6: e8 05 a2 ff ff              	callq	0x14146ccf0 <.text+0x146bcf0>
141472aeb: 49 89 d7                    	movq	%rdx, %r15
141472aee: 48 85 c0                    	testq	%rax, %rax
141472af1: 0f 84 98 01 00 00           	je	0x141472c8f <.text+0x1471c8f>
141472af7: 4c 89 bd d8 00 00 00        	movq	%r15, 0xd8(%rbp)
141472afe: 48 8b 35 a3 98 1b 02        	movq	0x21b98a3(%rip), %rsi   # 0x14362c3a8
141472b05: 48 8b 06                    	movq	(%rsi), %rax
141472b08: 48 83 f8 03                 	cmpq	$0x3, %rax
141472b0c: 0f 82 a8 00 00 00           	jb	0x141472bba <.text+0x1471bba>
141472b12: 48 8d 85 d8 00 00 00        	leaq	0xd8(%rbp), %rax
141472b19: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
141472b20: 48 8d 05 f9 a0 8d ff        	leaq	-0x725f07(%rip), %rax   # 0x140d4cc20 <.text+0xd4bc20>
141472b27: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
141472b2e: 48 8d 05 db 1c 7f 01        	leaq	0x17f1cdb(%rip), %rax   # 0x142c64810
141472b35: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141472b3c: 48 c7 85 68 01 00 00 02 00 00 00    	movq	$0x2, 0x168(%rbp)
141472b47: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
141472b52: 4c 89 b5 70 01 00 00        	movq	%r14, 0x170(%rbp)
141472b59: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
141472b64: 48 8d 0d c5 1c 7f 01        	leaq	0x17f1cc5(%rip), %rcx   # 0x142c64830
141472b6b: e8 10 b2 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
141472b70: 48 8d 0d d1 15 7f 01        	leaq	0x17f15d1(%rip), %rcx   # 0x142c64148
141472b77: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
141472b7e: 48 c7 85 78 02 00 00 26 00 00 00    	movq	$0x26, 0x278(%rbp)
141472b89: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
141472b90: 48 c7 85 88 02 00 00 26 00 00 00    	movq	$0x26, 0x288(%rbp)
141472b9b: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
141472ba2: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141472ba9: 4c 8d 85 70 02 00 00        	leaq	0x270(%rbp), %r8
141472bb0: ba 03 00 00 00              	movl	$0x3, %edx
141472bb5: e8 76 e8 03 ff              	callq	0x1404b1430 <.text+0x4b0430>
141472bba: 4c 89 e1                    	movq	%r12, %rcx
141472bbd: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
141472bc1: e8 7a 05 20 00              	callq	0x141673140 <.text+0x1672140>
141472bc6: 49 89 c7                    	movq	%rax, %r15
141472bc9: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
141472bcd: 48 8b 06                    	movq	(%rsi), %rax
141472bd0: 48 83 f8 03                 	cmpq	$0x3, %rax
141472bd4: 0f 82 a9 00 00 00           	jb	0x141472c83 <.text+0x1471c83>
141472bda: 48 8d 45 30                 	leaq	0x30(%rbp), %rax
141472bde: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
141472be5: 48 8d 05 d4 aa 91 ff        	leaq	-0x6e552c(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
141472bec: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
141472bf3: 48 8d 05 76 1c 7f 01        	leaq	0x17f1c76(%rip), %rax   # 0x142c64870
141472bfa: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141472c01: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
141472c0c: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
141472c17: 4c 89 b5 70 01 00 00        	movq	%r14, 0x170(%rbp)
141472c1e: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
141472c29: 48 8d 0d 50 1c 7f 01        	leaq	0x17f1c50(%rip), %rcx   # 0x142c64880
141472c30: e8 4b b1 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
141472c35: 48 8d 0d 0c 15 7f 01        	leaq	0x17f150c(%rip), %rcx   # 0x142c64148
141472c3c: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
141472c43: 48 c7 85 78 02 00 00 26 00 00 00    	movq	$0x26, 0x278(%rbp)
141472c4e: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
141472c55: 48 c7 85 88 02 00 00 26 00 00 00    	movq	$0x26, 0x288(%rbp)
141472c60: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
141472c67: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
141472c6e: 4c 8d 85 70 02 00 00        	leaq	0x270(%rbp), %r8
141472c75: ba 03 00 00 00              	movl	$0x3, %edx
141472c7a: e8 b1 e7 03 ff              	callq	0x1404b1430 <.text+0x4b0430>
141472c7f: 4c 8b 7d 30                 	movq	0x30(%rbp), %r15
141472c83: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
141472c8a: e8 a1 9f 8d ff              	callq	0x140d4cc30 <.text+0xd4bc30>
141472c8f: f2 0f 10 b5 c0 00 00 00     	movsd	0xc0(%rbp), %xmm6
141472c97: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
141472c9b: 48 8b 45 68                 	movq	0x68(%rbp), %rax
141472c9f: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
141472ca3: 4c 8d 04 c2                 	leaq	(%rdx,%rax,8), %r8
141472ca7: 4c 8d 0d 9a 11 7f 01        	leaq	0x17f119a(%rip), %r9    # 0x142c63e48
141472cae: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
141472cb5: e8 06 b2 e9 ff              	callq	0x14130dec0 <.text+0x130cec0>
141472cba: 48 8b 55 78                 	movq	0x78(%rbp), %rdx
141472cbe: 4c 8b 85 80 00 00 00        	movq	0x80(%rbp), %r8
141472cc5: 49 c1 e0 04                 	shlq	$0x4, %r8
141472cc9: 49 01 d0                    	addq	%rdx, %r8
141472ccc: 4c 8d 0d 75 11 7f 01        	leaq	0x17f1175(%rip), %r9    # 0x142c63e48
141472cd3: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
141472cda: e8 e1 29 e9 ff              	callq	0x1413056c0 <.text+0x13046c0>
141472cdf: 48 8b 75 40                 	movq	0x40(%rbp), %rsi
141472ce3: 48 89 f0                    	movq	%rsi, %rax
141472ce6: 48 f7 d8                    	negq	%rax
141472ce9: 71 16                       	jno	0x141472d01 <.text+0x1471d01>
141472ceb: 49 89 fe                    	movq	%rdi, %r14
141472cee: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141472cf8: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
141472cff: eb 1a                       	jmp	0x141472d1b <.text+0x1471d1b>
141472d01: 49 89 fe                    	movq	%rdi, %r14
141472d04: 4c 8d 05 a5 1e 7f 01        	leaq	0x17f1ea5(%rip), %r8    # 0x142c64bb0
141472d0b: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
141472d12: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
141472d16: e8 15 5c e8 ff              	callq	0x1412f8930 <.text+0x12f7930>
141472d1b: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
141472d1f: 48 89 d0                    	movq	%rdx, %rax
141472d22: 48 c1 e8 20                 	shrq	$0x20, %rax
141472d26: 4c 89 bd a8 01 00 00        	movq	%r15, 0x1a8(%rbp)
141472d2d: 8b 8d 1c 01 00 00           	movl	0x11c(%rbp), %ecx
141472d33: 89 8d 20 02 00 00           	movl	%ecx, 0x220(%rbp)
141472d39: f3 44 0f 11 9d 24 02 00 00  	movss	%xmm11, 0x224(%rbp)
141472d42: 89 95 28 02 00 00           	movl	%edx, 0x228(%rbp)
141472d48: 89 85 2c 02 00 00           	movl	%eax, 0x22c(%rbp)
141472d4e: 48 8b 45 10                 	movq	0x10(%rbp), %rax
141472d52: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
141472d59: 44 0f 11 95 38 02 00 00     	movups	%xmm10, 0x238(%rbp)
141472d61: 8b 85 f4 02 00 00           	movl	0x2f4(%rbp), %eax
141472d67: 89 85 f4 01 00 00           	movl	%eax, 0x1f4(%rbp)
141472d6d: 44 0f 11 85 f8 01 00 00     	movups	%xmm8, 0x1f8(%rbp)
141472d75: 44 0f 11 a5 08 02 00 00     	movups	%xmm12, 0x208(%rbp)
141472d7d: f3 44 0f 11 ad 18 02 00 00  	movss	%xmm13, 0x218(%rbp)
141472d86: 0f b7 85 16 03 00 00        	movzwl	0x316(%rbp), %eax
141472d8d: 66 89 85 1c 02 00 00        	movw	%ax, 0x21c(%rbp)
141472d94: 0f 13 b5 48 02 00 00        	movlps	%xmm6, 0x248(%rbp)
141472d9b: 0f 10 85 d8 00 00 00        	movups	0xd8(%rbp), %xmm0
141472da2: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
141472da9: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
141472db0: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
141472db7: 0f 10 85 50 02 00 00        	movups	0x250(%rbp), %xmm0
141472dbe: 0f 11 85 78 01 00 00        	movups	%xmm0, 0x178(%rbp)
141472dc5: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
141472dcc: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
141472dd3: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
141472dda: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
141472de1: 0f 10 85 70 02 00 00        	movups	0x270(%rbp), %xmm0
141472de8: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
141472def: 0f 29 bd b0 01 00 00        	movaps	%xmm7, 0x1b0(%rbp)
141472df6: 8b 85 24 01 00 00           	movl	0x124(%rbp), %eax
141472dfc: 89 85 c0 01 00 00           	movl	%eax, 0x1c0(%rbp)
141472e02: 8b 85 28 01 00 00           	movl	0x128(%rbp), %eax
141472e08: 88 85 c4 01 00 00           	movb	%al, 0x1c4(%rbp)
141472e0e: 8b 85 20 01 00 00           	movl	0x120(%rbp), %eax
141472e14: 89 85 c8 01 00 00           	movl	%eax, 0x1c8(%rbp)
141472e1a: f3 44 0f 7f b5 cc 01 00 00  	movdqu	%xmm14, 0x1cc(%rbp)
141472e23: 44 0f 11 8d dc 01 00 00     	movups	%xmm9, 0x1dc(%rbp)
141472e2b: f3 44 0f 11 bd ec 01 00 00  	movss	%xmm15, 0x1ec(%rbp)
141472e34: 8b 85 2c 01 00 00           	movl	0x12c(%rbp), %eax
141472e3a: 88 85 f0 01 00 00           	movb	%al, 0x1f0(%rbp)
141472e40: 8b 85 30 01 00 00           	movl	0x130(%rbp), %eax
141472e46: 88 85 f1 01 00 00           	movb	%al, 0x1f1(%rbp)
141472e4c: 0f b6 85 2f 03 00 00        	movzbl	0x32f(%rbp), %eax
141472e53: 88 85 f2 01 00 00           	movb	%al, 0x1f2(%rbp)
141472e59: e8 62 a0 ff ff              	callq	0x14146cec0 <.text+0x146bec0>
141472e5e: 48 89 c7                    	movq	%rax, %rdi
141472e61: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
141472e68: 41 b8 f0 00 00 00           	movl	$0xf0, %r8d
141472e6e: 48 89 c1                    	movq	%rax, %rcx
141472e71: e8 8a 0d 4b 01              	callq	0x142923c00 <.text+0x2922c00>
141472e76: 41 83 fd 1b                 	cmpl	$0x1b, %r13d
141472e7a: 0f 95 c0                    	setne	%al
141472e7d: 0a 85 36 03 00 00           	orb	0x336(%rbp), %al
141472e83: 44 8b 85 f0 02 00 00        	movl	0x2f0(%rbp), %r8d
141472e8a: 41 80 e0 01                 	andb	$0x1, %r8b
141472e8e: 8b 95 04 03 00 00           	movl	0x304(%rbp), %edx
141472e94: 80 e2 01                    	andb	$0x1, %dl
141472e97: 49 89 3e                    	movq	%rdi, (%r14)
141472e9a: 4c 89 f7                    	movq	%r14, %rdi
141472e9d: 8b 8d d4 00 00 00           	movl	0xd4(%rbp), %ecx
141472ea3: 41 89 4e 08                 	movl	%ecx, 0x8(%r14)
141472ea7: 41 88 46 0c                 	movb	%al, 0xc(%r14)
141472eab: 45 88 46 0d                 	movb	%r8b, 0xd(%r14)
141472eaf: 41 88 56 0e                 	movb	%dl, 0xe(%r14)
141472eb3: 41 88 5e 0f                 	movb	%bl, 0xf(%r14)
141472eb7: 48 8d 4d 58                 	leaq	0x58(%rbp), %rcx
141472ebb: e8 30 7e 41 ff              	callq	0x14088acf0 <.text+0x889cf0>
141472ec0: 48 89 f0                    	movq	%rsi, %rax
141472ec3: 48 f7 d8                    	negq	%rax
141472ec6: 70 19                       	jo	0x141472ee1 <.text+0x1471ee1>
141472ec8: 73 17                       	jae	0x141472ee1 <.text+0x1471ee1>
141472eca: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
141472ece: 48 c1 e6 03                 	shlq	$0x3, %rsi
141472ed2: 48 8d 14 b6                 	leaq	(%rsi,%rsi,4), %rdx
141472ed6: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141472edc: e8 3f 0d 1a ff              	callq	0x140613c20 <.text+0x612c20>
141472ee1: 48 8d 8d 38 01 00 00        	leaq	0x138(%rbp), %rcx
141472ee8: e8 a3 ab 3e ff              	callq	0x14085da90 <.text+0x85ca90>
141472eed: e9 8a e5 ff ff              	jmp	0x14147147c <.text+0x147047c>
141472ef2: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141472ef9: 48 8d 0d d0 24 63 01        	leaq	0x16324d0(%rip), %rcx   # 0x142aa53d0
141472f00: 4c 8d 05 b1 4b 66 01        	leaq	0x1664bb1(%rip), %r8    # 0x142ad7ab8
141472f07: ba 1d 00 00 00              	movl	$0x1d, %edx
141472f0c: e8 1f 65 46 01              	callq	0x1428d9430 <.text+0x28d8430>
141472f11: e9 6d 04 00 00              	jmp	0x141473383 <.text+0x1472383>
141472f16: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141472f1d: 48 8d 0d 74 25 63 01        	leaq	0x1632574(%rip), %rcx   # 0x142aa5498
141472f24: 4c 8d 05 8d 4b 66 01        	leaq	0x1664b8d(%rip), %r8    # 0x142ad7ab8
141472f2b: ba 22 00 00 00              	movl	$0x22, %edx
141472f30: e8 fb 64 46 01              	callq	0x1428d9430 <.text+0x28d8430>
141472f35: e9 49 04 00 00              	jmp	0x141473383 <.text+0x1472383>
141472f3a: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141472f41: 48 8d 0d 50 25 63 01        	leaq	0x1632550(%rip), %rcx   # 0x142aa5498
141472f48: 4c 8d 05 81 4b 66 01        	leaq	0x1664b81(%rip), %r8    # 0x142ad7ad0
141472f4f: ba 22 00 00 00              	movl	$0x22, %edx
141472f54: e8 d7 64 46 01              	callq	0x1428d9430 <.text+0x28d8430>
141472f59: e9 25 04 00 00              	jmp	0x141473383 <.text+0x1472383>
141472f5e: 41 83 fd 13                 	cmpl	$0x13, %r13d
141472f62: 0f 9c c3                    	setl	%bl
141472f65: 0f 8d 30 02 00 00           	jge	0x14147319b <.text+0x147219b>
141472f6b: 48 8b 05 36 94 1b 02        	movq	0x21b9436(%rip), %rax   # 0x14362c3a8
141472f72: 48 8b 00                    	movq	(%rax), %rax
141472f75: 48 83 f8 03                 	cmpq	$0x3, %rax
141472f79: 0f 82 bd 00 00 00           	jb	0x14147303c <.text+0x147203c>
141472f7f: 48 8d 05 0a 17 7f 01        	leaq	0x17f170a(%rip), %rax   # 0x142c64690
141472f86: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
141472f8d: 48 8d 05 ec a5 91 ff        	leaq	-0x6e5a14(%rip), %rax   # 0x140d8d580 <.text+0xd8c580>
141472f94: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
141472f9b: 48 8d 05 2e 17 7f 01        	leaq	0x17f172e(%rip), %rax   # 0x142c646d0
141472fa2: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141472fa9: 48 c7 85 68 01 00 00 02 00 00 00    	movq	$0x2, 0x168(%rbp)
141472fb4: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
141472fbf: 48 8d 85 50 02 00 00        	leaq	0x250(%rbp), %rax
141472fc6: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
141472fcd: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
141472fd8: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141472fdf: 48 8d 0d 0a 17 7f 01        	leaq	0x17f170a(%rip), %rcx   # 0x142c646f0
141472fe6: e8 95 ad 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
141472feb: 48 8d 0d 56 11 7f 01        	leaq	0x17f1156(%rip), %rcx   # 0x142c64148
141472ff2: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
141472ff9: 48 c7 85 78 02 00 00 26 00 00 00    	movq	$0x26, 0x278(%rbp)
141473004: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
14147300b: 48 c7 85 88 02 00 00 26 00 00 00    	movq	$0x26, 0x288(%rbp)
141473016: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
14147301d: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473024: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14147302b: 4c 8d 85 70 02 00 00        	leaq	0x270(%rbp), %r8
141473032: ba 03 00 00 00              	movl	$0x3, %edx
141473037: e8 f4 e3 03 ff              	callq	0x1404b1430 <.text+0x4b0430>
14147303c: 41 83 fd 11                 	cmpl	$0x11, %r13d
141473040: 0f 8d 55 01 00 00           	jge	0x14147319b <.text+0x147219b>
141473046: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
141473051: 48 8b 45 68                 	movq	0x68(%rbp), %rax
141473055: 48 85 c0                    	testq	%rax, %rax
141473058: 0f 84 3d 01 00 00           	je	0x14147319b <.text+0x147219b>
14147305e: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
141473062: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
141473066: 48 8d 14 c1                 	leaq	(%rcx,%rax,8), %rdx
14147306a: 45 31 c0                    	xorl	%r8d, %r8d
14147306d: 49 c7 c1 ff ff ff ff        	movq	$-0x1, %r9
141473074: f3 0f 10 05 2c 15 4c 01     	movss	0x14c152c(%rip), %xmm0  # 0x1429345a8
14147307c: f3 0f 10 0d ac c9 60 01     	movss	0x160c9ac(%rip), %xmm1  # 0x142a7fa30
141473084: 49 ba 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r10 # imm = 0x8000000000000001
14147308e: 0f 28 15 4b 60 7b 01        	movaps	0x17b604b(%rip), %xmm2  # 0x142c290e0
141473095: 0f 28 1d 24 79 64 01        	movaps	0x1647924(%rip), %xmm3  # 0x142aba9c0
14147309c: eb 0d                       	jmp	0x1414730ab <.text+0x14720ab>
14147309e: 48 83 c1 28                 	addq	$0x28, %rcx
1414730a2: 48 39 d1                    	cmpq	%rdx, %rcx
1414730a5: 0f 84 f0 00 00 00           	je	0x14147319b <.text+0x147219b>
1414730ab: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1414730af: 48 85 c0                    	testq	%rax, %rax
1414730b2: 74 ea                       	je	0x14147309e <.text+0x147209e>
1414730b4: 4c 8b 59 08                 	movq	0x8(%rcx), %r11
1414730b8: 48 c1 e0 03                 	shlq	$0x3, %rax
1414730bc: 48 8d 34 40                 	leaq	(%rax,%rax,2), %rsi
1414730c0: 31 ff                       	xorl	%edi, %edi
1414730c2: eb 10                       	jmp	0x1414730d4 <.text+0x14720d4>
1414730c4: 49 83 3c 3b 00              	cmpq	$0x0, (%r11,%rdi)
1414730c9: 74 6f                       	je	0x14147313a <.text+0x147213a>
1414730cb: 48 83 c7 18                 	addq	$0x18, %rdi
1414730cf: 48 39 fe                    	cmpq	%rdi, %rsi
1414730d2: 74 ca                       	je	0x14147309e <.text+0x147209e>
1414730d4: 49 83 3c 3b 00              	cmpq	$0x0, (%r11,%rdi)
1414730d9: 75 f0                       	jne	0x1414730cb <.text+0x14720cb>
1414730db: 4d 8b 74 3b 08              	movq	0x8(%r11,%rdi), %r14
1414730e0: b8 01 00 00 00              	movl	$0x1, %eax
1414730e5: f0                          	lock
1414730e6: 4d 0f b1 4e 08              	cmpxchgq	%r9, 0x8(%r14)
1414730eb: 0f 85 1d 02 00 00           	jne	0x14147330e <.text+0x147230e>
1414730f1: 49 8b 06                    	movq	(%r14), %rax
1414730f4: 49 c7 46 08 01 00 00 00     	movq	$0x1, 0x8(%r14)
1414730fc: 48 83 f8 01                 	cmpq	$0x1, %rax
141473100: 0f 85 08 02 00 00           	jne	0x14147330e <.text+0x147230e>
141473106: 49 8b 44 3b 08              	movq	0x8(%r11,%rdi), %rax
14147310b: 4c 3b 40 10                 	cmpq	0x10(%rax), %r8
14147310f: 71 b3                       	jno	0x1414730c4 <.text+0x14720c4>
141473111: f3 0f 10 60 34              	movss	0x34(%rax), %xmm4
141473116: f3 0f 51 e4                 	sqrtss	%xmm4, %xmm4
14147311a: f3 0f 59 e0                 	mulss	%xmm0, %xmm4
14147311e: 0f 57 ed                    	xorps	%xmm5, %xmm5
141473121: f3 0f 5f ec                 	maxss	%xmm4, %xmm5
141473125: 0f 28 e1                    	movaps	%xmm1, %xmm4
141473128: f3 0f 5d e5                 	minss	%xmm5, %xmm4
14147312c: f3 0f 11 60 34              	movss	%xmm4, 0x34(%rax)
141473131: c6 85 36 03 00 00 01        	movb	$0x1, 0x336(%rbp)
141473138: eb 8a                       	jmp	0x1414730c4 <.text+0x14720c4>
14147313a: 4d 8b 74 3b 08              	movq	0x8(%r11,%rdi), %r14
14147313f: b8 01 00 00 00              	movl	$0x1, %eax
141473144: f0                          	lock
141473145: 4d 0f b1 4e 08              	cmpxchgq	%r9, 0x8(%r14)
14147314a: 0f 85 d3 01 00 00           	jne	0x141473323 <.text+0x1472323>
141473150: 49 8b 06                    	movq	(%r14), %rax
141473153: 49 c7 46 08 01 00 00 00     	movq	$0x1, 0x8(%r14)
14147315b: 48 83 f8 01                 	cmpq	$0x1, %rax
14147315f: 0f 85 be 01 00 00           	jne	0x141473323 <.text+0x1472323>
141473165: 49 8b 44 3b 08              	movq	0x8(%r11,%rdi), %rax
14147316a: 4c 39 50 10                 	cmpq	%r10, 0x10(%rax)
14147316e: 0f 85 57 ff ff ff           	jne	0x1414730cb <.text+0x14720cb>
141473174: f2 0f 10 60 1c              	movsd	0x1c(%rax), %xmm4
141473179: 0f 51 e4                    	sqrtps	%xmm4, %xmm4
14147317c: 0f 59 e2                    	mulps	%xmm2, %xmm4
14147317f: 0f 57 ed                    	xorps	%xmm5, %xmm5
141473182: 0f 5f ec                    	maxps	%xmm4, %xmm5
141473185: 0f 28 e3                    	movaps	%xmm3, %xmm4
141473188: 0f 5d e5                    	minps	%xmm5, %xmm4
14147318b: 0f 13 60 1c                 	movlps	%xmm4, 0x1c(%rax)
14147318f: c6 85 36 03 00 00 01        	movb	$0x1, 0x336(%rbp)
141473196: e9 30 ff ff ff              	jmp	0x1414730cb <.text+0x14720cb>
14147319b: 80 bd 36 03 00 00 01        	cmpb	$0x1, 0x336(%rbp)
1414731a2: 75 09                       	jne	0x1414731ad <.text+0x14721ad>
1414731a4: 0f b6 95 d8 00 00 00        	movzbl	0xd8(%rbp), %edx
1414731ab: eb 09                       	jmp	0x1414731b6 <.text+0x14721b6>
1414731ad: c6 85 d8 00 00 00 00        	movb	$0x0, 0xd8(%rbp)
1414731b4: 31 d2                       	xorl	%edx, %edx
1414731b6: c6 85 37 03 00 00 00        	movb	$0x0, 0x337(%rbp)
1414731bd: 80 e2 01                    	andb	$0x1, %dl
1414731c0: 48 8d 4d 58                 	leaq	0x58(%rbp), %rcx
1414731c4: e8 b7 37 61 ff              	callq	0x140a86980 <.text+0xa85980>
1414731c9: f6 85 36 03 00 00 01        	testb	$0x1, 0x336(%rbp)
1414731d0: 0f 84 ba 00 00 00           	je	0x141473290 <.text+0x1472290>
1414731d6: 48 8b 05 cb 91 1b 02        	movq	0x21b91cb(%rip), %rax   # 0x14362c3a8
1414731dd: 48 8b 00                    	movq	(%rax), %rax
1414731e0: 48 83 f8 03                 	cmpq	$0x3, %rax
1414731e4: 48 8b 7d 38                 	movq	0x38(%rbp), %rdi
1414731e8: 0f 28 7d e0                 	movaps	-0x20(%rbp), %xmm7
1414731ec: 44 0f 28 4d f0              	movaps	-0x10(%rbp), %xmm9
1414731f1: f3 44 0f 10 bd 34 01 00 00  	movss	0x134(%rbp), %xmm15
1414731fa: 4c 8d b5 50 02 00 00        	leaq	0x250(%rbp), %r14
141473201: 0f 82 ba ea ff ff           	jb	0x141471cc1 <.text+0x1470cc1>
141473207: 48 8d 05 92 15 7f 01        	leaq	0x17f1592(%rip), %rax   # 0x142c647a0
14147320e: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
141473215: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
141473220: 48 c7 85 70 01 00 00 08 00 00 00    	movq	$0x8, 0x170(%rbp)
14147322b: 0f 57 c0                    	xorps	%xmm0, %xmm0
14147322e: 0f 11 85 78 01 00 00        	movups	%xmm0, 0x178(%rbp)
141473235: 48 8d 0d 74 15 7f 01        	leaq	0x17f1574(%rip), %rcx   # 0x142c647b0
14147323c: e8 3f ab 20 ff              	callq	0x14067dd80 <.text+0x67cd80>
141473241: 48 8d 0d 00 0f 7f 01        	leaq	0x17f0f00(%rip), %rcx   # 0x142c64148
141473248: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
14147324f: 48 c7 85 78 02 00 00 26 00 00 00    	movq	$0x26, 0x278(%rbp)
14147325a: 48 89 8d 80 02 00 00        	movq	%rcx, 0x280(%rbp)
141473261: 48 c7 85 88 02 00 00 26 00 00 00    	movq	$0x26, 0x288(%rbp)
14147326c: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
141473273: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14147327a: 4c 8d 85 70 02 00 00        	leaq	0x270(%rbp), %r8
141473281: ba 03 00 00 00              	movl	$0x3, %edx
141473286: e8 a5 e1 03 ff              	callq	0x1404b1430 <.text+0x4b0430>
14147328b: e9 31 ea ff ff              	jmp	0x141471cc1 <.text+0x1470cc1>
141473290: 48 8b 7d 38                 	movq	0x38(%rbp), %rdi
141473294: 0f 28 7d e0                 	movaps	-0x20(%rbp), %xmm7
141473298: 44 0f 28 4d f0              	movaps	-0x10(%rbp), %xmm9
14147329d: f3 44 0f 10 bd 34 01 00 00  	movss	0x134(%rbp), %xmm15
1414732a6: 8b 85 04 03 00 00           	movl	0x304(%rbp), %eax
1414732ac: 4c 8d b5 50 02 00 00        	leaq	0x250(%rbp), %r14
1414732b3: e9 6b e9 ff ff              	jmp	0x141471c23 <.text+0x1470c23>
1414732b8: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
1414732bf: 48 8d 0d 32 16 7f 01        	leaq	0x17f1632(%rip), %rcx   # 0x142c648f8
1414732c6: e8 45 60 46 01              	callq	0x1428d9310 <.text+0x28d8310>
1414732cb: e9 b3 00 00 00              	jmp	0x141473383 <.text+0x1472383>
1414732d0: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
1414732d7: 48 8d 0d 32 16 7f 01        	leaq	0x17f1632(%rip), %rcx   # 0x142c64910
1414732de: e8 2d 60 46 01              	callq	0x1428d9310 <.text+0x28d8310>
1414732e3: e9 9b 00 00 00              	jmp	0x141473383 <.text+0x1472383>
1414732e8: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
1414732ef: 48 8d 0d 32 16 7f 01        	leaq	0x17f1632(%rip), %rcx   # 0x142c64928
1414732f6: e8 15 60 46 01              	callq	0x1428d9310 <.text+0x28d8310>
1414732fb: e9 83 00 00 00              	jmp	0x141473383 <.text+0x1472383>
141473300: 48 8d 0d 71 bd 7e 01        	leaq	0x17ebd71(%rip), %rcx   # 0x142c5f078
141473307: e8 04 60 46 01              	callq	0x1428d9310 <.text+0x28d8310>
14147330c: eb 75                       	jmp	0x141473383 <.text+0x1472383>
14147330e: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473315: 48 8d 0d 7c 15 7f 01        	leaq	0x17f157c(%rip), %rcx   # 0x142c64898
14147331c: e8 ef 5f 46 01              	callq	0x1428d9310 <.text+0x28d8310>
141473321: eb 60                       	jmp	0x141473383 <.text+0x1472383>
141473323: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
14147332a: 48 8d 0d 7f 15 7f 01        	leaq	0x17f157f(%rip), %rcx   # 0x142c648b0
141473331: e8 da 5f 46 01              	callq	0x1428d9310 <.text+0x28d8310>
141473336: eb 4b                       	jmp	0x141473383 <.text+0x1472383>
141473338: c6 85 35 03 00 00 00        	movb	$0x0, 0x335(%rbp)
14147333f: 48 8d 0d b2 e0 67 01        	leaq	0x167e0b2(%rip), %rcx   # 0x142af13f8
141473346: e8 c5 5f 46 01              	callq	0x1428d9310 <.text+0x28d8310>
14147334b: eb 36                       	jmp	0x141473383 <.text+0x1472383>
14147334d: 48 8d 0d a4 e0 67 01        	leaq	0x167e0a4(%rip), %rcx   # 0x142af13f8
141473354: e8 b7 5f 46 01              	callq	0x1428d9310 <.text+0x28d8310>
141473359: eb 28                       	jmp	0x141473383 <.text+0x1472383>
14147335b: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473362: 48 8d 0d 5f 15 7f 01        	leaq	0x17f155f(%rip), %rcx   # 0x142c648c8
141473369: e8 a2 5f 46 01              	callq	0x1428d9310 <.text+0x28d8310>
14147336e: eb 13                       	jmp	0x141473383 <.text+0x1472383>
141473370: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473377: 48 8d 0d 62 15 7f 01        	leaq	0x17f1562(%rip), %rcx   # 0x142c648e0
14147337e: e8 8d 5f 46 01              	callq	0x1428d9310 <.text+0x28d8310>
141473383: 0f 0b                       	ud2
141473385: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141473390: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473395: 55                          	pushq	%rbp
141473396: 41 57                       	pushq	%r15
141473398: 41 56                       	pushq	%r14
14147339a: 41 55                       	pushq	%r13
14147339c: 41 54                       	pushq	%r12
14147339e: 56                          	pushq	%rsi
14147339f: 57                          	pushq	%rdi
1414733a0: 53                          	pushq	%rbx
1414733a1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1414733a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414733af: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1414733b5: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
1414733bc: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1414733c2: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1414733c8: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1414733ce: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1414733d7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1414733e0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1414733e9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1414733f1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1414733f9: 80 bd 2e 03 00 00 00        	cmpb	$0x0, 0x32e(%rbp)
141473400: 75 5d                       	jne	0x14147345f <.text+0x147245f>
141473402: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
14147340a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473412: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14147341b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473424: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
14147342d: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473433: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473439: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
14147343f: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141473445: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
14147344b: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473452: 5b                          	popq	%rbx
141473453: 5f                          	popq	%rdi
141473454: 5e                          	popq	%rsi
141473455: 41 5c                       	popq	%r12
141473457: 41 5d                       	popq	%r13
141473459: 41 5e                       	popq	%r14
14147345b: 41 5f                       	popq	%r15
14147345d: 5d                          	popq	%rbp
14147345e: c3                          	retq
14147345f: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
141473466: e8 95 45 17 ff              	callq	0x1405e7a00 <.text+0x5e6a00>
14147346b: eb 95                       	jmp	0x141473402 <.text+0x1472402>
14147346d: 0f 1f 00                    	nopl	(%rax)
141473470: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473475: 55                          	pushq	%rbp
141473476: 41 57                       	pushq	%r15
141473478: 41 56                       	pushq	%r14
14147347a: 41 55                       	pushq	%r13
14147347c: 41 54                       	pushq	%r12
14147347e: 56                          	pushq	%rsi
14147347f: 57                          	pushq	%rdi
141473480: 53                          	pushq	%rbx
141473481: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473488: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147348f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473495: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147349c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1414734a2: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1414734a8: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1414734ae: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1414734b7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1414734c0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1414734c9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1414734d1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1414734d9: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
1414734e0: 48 85 d2                    	testq	%rdx, %rdx
1414734e3: 74 12                       	je	0x1414734f7 <.text+0x14724f7>
1414734e5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414734eb: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1414734f2: e8 29 07 1a ff              	callq	0x140613c20 <.text+0x612c20>
1414734f7: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414734ff: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473507: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473510: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473519: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473522: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473528: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
14147352e: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473534: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
14147353a: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473540: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473547: 5b                          	popq	%rbx
141473548: 5f                          	popq	%rdi
141473549: 5e                          	popq	%rsi
14147354a: 41 5c                       	popq	%r12
14147354c: 41 5d                       	popq	%r13
14147354e: 41 5e                       	popq	%r14
141473550: 41 5f                       	popq	%r15
141473552: 5d                          	popq	%rbp
141473553: c3                          	retq
141473554: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141473560: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473565: 55                          	pushq	%rbp
141473566: 41 57                       	pushq	%r15
141473568: 41 56                       	pushq	%r14
14147356a: 41 55                       	pushq	%r13
14147356c: 41 54                       	pushq	%r12
14147356e: 56                          	pushq	%rsi
14147356f: 57                          	pushq	%rdi
141473570: 53                          	pushq	%rbx
141473571: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473578: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147357f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473585: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147358c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473592: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473598: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147359e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1414735a7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1414735b0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1414735b9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1414735c1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1414735c9: 80 bd 2d 03 00 00 00        	cmpb	$0x0, 0x32d(%rbp)
1414735d0: 74 0c                       	je	0x1414735de <.text+0x14725de>
1414735d2: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
1414735d9: e8 c2 52 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
1414735de: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414735e6: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414735ee: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1414735f7: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473600: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473609: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
14147360f: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473615: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
14147361b: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141473622: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473628: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
14147362f: 5b                          	popq	%rbx
141473630: 5f                          	popq	%rdi
141473631: 5e                          	popq	%rsi
141473632: 41 5c                       	popq	%r12
141473634: 41 5d                       	popq	%r13
141473636: 41 5e                       	popq	%r14
141473638: 41 5f                       	popq	%r15
14147363a: 5d                          	popq	%rbp
14147363b: c3                          	retq
14147363c: 0f 1f 40 00                 	nopl	(%rax)
141473640: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473645: 55                          	pushq	%rbp
141473646: 41 57                       	pushq	%r15
141473648: 41 56                       	pushq	%r14
14147364a: 41 55                       	pushq	%r13
14147364c: 41 54                       	pushq	%r12
14147364e: 56                          	pushq	%rsi
14147364f: 57                          	pushq	%rdi
141473650: 53                          	pushq	%rbx
141473651: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473658: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147365f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473665: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147366c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473672: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473678: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147367e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473687: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473690: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473699: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1414736a1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1414736a9: 48 8d 8d 38 01 00 00        	leaq	0x138(%rbp), %rcx
1414736b0: e8 db a3 3e ff              	callq	0x14085da90 <.text+0x85ca90>
1414736b5: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414736bd: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414736c5: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1414736ce: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1414736d7: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1414736e0: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1414736e6: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414736ec: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414736f2: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
1414736f9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414736ff: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473706: 5b                          	popq	%rbx
141473707: 5f                          	popq	%rdi
141473708: 5e                          	popq	%rsi
141473709: 41 5c                       	popq	%r12
14147370b: 41 5d                       	popq	%r13
14147370d: 41 5e                       	popq	%r14
14147370f: 41 5f                       	popq	%r15
141473711: 5d                          	popq	%rbp
141473712: c3                          	retq
141473713: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141473720: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473725: 55                          	pushq	%rbp
141473726: 41 57                       	pushq	%r15
141473728: 41 56                       	pushq	%r14
14147372a: 41 55                       	pushq	%r13
14147372c: 41 54                       	pushq	%r12
14147372e: 56                          	pushq	%rsi
14147372f: 57                          	pushq	%rdi
141473730: 53                          	pushq	%rbx
141473731: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473738: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147373f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473745: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147374c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473752: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473758: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147375e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473767: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473770: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473779: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473781: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473789: 80 bd 2c 03 00 00 00        	cmpb	$0x0, 0x32c(%rbp)
141473790: 74 0c                       	je	0x14147379e <.text+0x147279e>
141473792: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
141473799: e8 02 51 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
14147379e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414737a6: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414737ae: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1414737b7: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1414737c0: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1414737c9: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1414737cf: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414737d5: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414737db: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
1414737e2: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414737e8: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1414737ef: 5b                          	popq	%rbx
1414737f0: 5f                          	popq	%rdi
1414737f1: 5e                          	popq	%rsi
1414737f2: 41 5c                       	popq	%r12
1414737f4: 41 5d                       	popq	%r13
1414737f6: 41 5e                       	popq	%r14
1414737f8: 41 5f                       	popq	%r15
1414737fa: 5d                          	popq	%rbp
1414737fb: c3                          	retq
1414737fc: 0f 1f 40 00                 	nopl	(%rax)
141473800: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473805: 55                          	pushq	%rbp
141473806: 41 57                       	pushq	%r15
141473808: 41 56                       	pushq	%r14
14147380a: 41 55                       	pushq	%r13
14147380c: 41 54                       	pushq	%r12
14147380e: 56                          	pushq	%rsi
14147380f: 57                          	pushq	%rdi
141473810: 53                          	pushq	%rbx
141473811: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473818: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147381f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473825: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147382c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473832: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473838: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147383e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473847: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473850: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473859: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473861: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473869: 48 8b 45 40                 	movq	0x40(%rbp), %rax
14147386d: 48 8d 0c 45 00 00 00 00     	leaq	(,%rax,2), %rcx
141473875: 48 85 c9                    	testq	%rcx, %rcx
141473878: 74 17                       	je	0x141473891 <.text+0x1472891>
14147387a: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14147387e: 48 c1 e0 03                 	shlq	$0x3, %rax
141473882: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
141473886: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14147388c: e8 8f 03 1a ff              	callq	0x140613c20 <.text+0x612c20>
141473891: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473899: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414738a1: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1414738aa: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1414738b3: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1414738bc: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1414738c2: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414738c8: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414738ce: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
1414738d5: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414738db: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1414738e2: 5b                          	popq	%rbx
1414738e3: 5f                          	popq	%rdi
1414738e4: 5e                          	popq	%rsi
1414738e5: 41 5c                       	popq	%r12
1414738e7: 41 5d                       	popq	%r13
1414738e9: 41 5e                       	popq	%r14
1414738eb: 41 5f                       	popq	%r15
1414738ed: 5d                          	popq	%rbp
1414738ee: c3                          	retq
1414738ef: 90                          	nop
1414738f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414738f5: 55                          	pushq	%rbp
1414738f6: 41 57                       	pushq	%r15
1414738f8: 41 56                       	pushq	%r14
1414738fa: 41 55                       	pushq	%r13
1414738fc: 41 54                       	pushq	%r12
1414738fe: 56                          	pushq	%rsi
1414738ff: 57                          	pushq	%rdi
141473900: 53                          	pushq	%rbx
141473901: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473908: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147390f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473915: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147391c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473922: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473928: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147392e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473937: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473940: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473949: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473951: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473959: 83 bd 20 03 00 00 00        	cmpl	$0x0, 0x320(%rbp)
141473960: 74 0c                       	je	0x14147396e <.text+0x147296e>
141473962: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
141473969: e8 32 4f 17 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
14147396e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473976: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
14147397e: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473987: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473990: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473999: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
14147399f: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414739a5: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414739ab: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
1414739b2: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414739b8: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1414739bf: 5b                          	popq	%rbx
1414739c0: 5f                          	popq	%rdi
1414739c1: 5e                          	popq	%rsi
1414739c2: 41 5c                       	popq	%r12
1414739c4: 41 5d                       	popq	%r13
1414739c6: 41 5e                       	popq	%r14
1414739c8: 41 5f                       	popq	%r15
1414739ca: 5d                          	popq	%rbp
1414739cb: c3                          	retq
1414739cc: 0f 1f 40 00                 	nopl	(%rax)
1414739d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414739d5: 55                          	pushq	%rbp
1414739d6: 41 57                       	pushq	%r15
1414739d8: 41 56                       	pushq	%r14
1414739da: 41 55                       	pushq	%r13
1414739dc: 41 54                       	pushq	%r12
1414739de: 56                          	pushq	%rsi
1414739df: 57                          	pushq	%rdi
1414739e0: 53                          	pushq	%rbx
1414739e1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1414739e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414739ef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1414739f5: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
1414739fc: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473a02: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473a08: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473a0e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473a17: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473a20: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473a29: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473a31: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473a39: 48 8d 4d 58                 	leaq	0x58(%rbp), %rcx
141473a3d: e8 ae 72 41 ff              	callq	0x14088acf0 <.text+0x889cf0>
141473a42: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473a4a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473a52: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473a5b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473a64: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473a6d: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473a73: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473a79: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473a7f: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141473a86: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473a8c: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473a93: 5b                          	popq	%rbx
141473a94: 5f                          	popq	%rdi
141473a95: 5e                          	popq	%rsi
141473a96: 41 5c                       	popq	%r12
141473a98: 41 5d                       	popq	%r13
141473a9a: 41 5e                       	popq	%r14
141473a9c: 41 5f                       	popq	%r15
141473a9e: 5d                          	popq	%rbp
141473a9f: c3                          	retq
141473aa0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473aa5: 55                          	pushq	%rbp
141473aa6: 41 57                       	pushq	%r15
141473aa8: 41 56                       	pushq	%r14
141473aaa: 41 55                       	pushq	%r13
141473aac: 41 54                       	pushq	%r12
141473aae: 56                          	pushq	%rsi
141473aaf: 57                          	pushq	%rdi
141473ab0: 53                          	pushq	%rbx
141473ab1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473ab8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141473abf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473ac5: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
141473acc: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473ad2: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473ad8: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473ade: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473ae7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473af0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473af9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473b01: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473b09: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
141473b10: 48 85 d2                    	testq	%rdx, %rdx
141473b13: 74 16                       	je	0x141473b2b <.text+0x1472b2b>
141473b15: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
141473b1c: 48 c1 e2 04                 	shlq	$0x4, %rdx
141473b20: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141473b26: e8 f5 00 1a ff              	callq	0x140613c20 <.text+0x612c20>
141473b2b: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473b33: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473b3b: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473b44: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473b4d: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473b56: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473b5c: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473b62: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473b68: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141473b6f: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473b75: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473b7c: 5b                          	popq	%rbx
141473b7d: 5f                          	popq	%rdi
141473b7e: 5e                          	popq	%rsi
141473b7f: 41 5c                       	popq	%r12
141473b81: 41 5d                       	popq	%r13
141473b83: 41 5e                       	popq	%r14
141473b85: 41 5f                       	popq	%r15
141473b87: 5d                          	popq	%rbp
141473b88: c3                          	retq
141473b89: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141473b90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473b95: 55                          	pushq	%rbp
141473b96: 41 57                       	pushq	%r15
141473b98: 41 56                       	pushq	%r14
141473b9a: 41 55                       	pushq	%r13
141473b9c: 41 54                       	pushq	%r12
141473b9e: 56                          	pushq	%rsi
141473b9f: 57                          	pushq	%rdi
141473ba0: 53                          	pushq	%rbx
141473ba1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473ba8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141473baf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473bb5: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
141473bbc: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473bc2: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473bc8: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473bce: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473bd7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473be0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473be9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473bf1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473bf9: 48 83 bd 20 03 00 00 ff     	cmpq	$-0x1, 0x320(%rbp)
141473c01: 74 24                       	je	0x141473c27 <.text+0x1472c27>
141473c03: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
141473c0a: f0                          	lock
141473c0b: 48 ff 08                    	decq	(%rax)
141473c0e: 75 17                       	jne	0x141473c27 <.text+0x1472c27>
141473c10: ba 40 00 00 00              	movl	$0x40, %edx
141473c15: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141473c1b: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
141473c22: e8 f9 ff 19 ff              	callq	0x140613c20 <.text+0x612c20>
141473c27: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473c2e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473c36: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473c3e: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473c47: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473c50: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473c59: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473c5f: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473c65: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473c6b: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141473c71: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473c77: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473c7e: 5b                          	popq	%rbx
141473c7f: 5f                          	popq	%rdi
141473c80: 5e                          	popq	%rsi
141473c81: 41 5c                       	popq	%r12
141473c83: 41 5d                       	popq	%r13
141473c85: 41 5e                       	popq	%r14
141473c87: 41 5f                       	popq	%r15
141473c89: 5d                          	popq	%rbp
141473c8a: c3                          	retq
141473c8b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141473c90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473c95: 55                          	pushq	%rbp
141473c96: 41 57                       	pushq	%r15
141473c98: 41 56                       	pushq	%r14
141473c9a: 41 55                       	pushq	%r13
141473c9c: 41 54                       	pushq	%r12
141473c9e: 56                          	pushq	%rsi
141473c9f: 57                          	pushq	%rdi
141473ca0: 53                          	pushq	%rbx
141473ca1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473ca8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141473caf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473cb5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
141473cbb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473cc1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473cc7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473ccd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473cd6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473cdf: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
141473ce9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473cf1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473cf9: 80 bd 35 03 00 00 00        	cmpb	$0x0, 0x335(%rbp)
141473d00: 74 77                       	je	0x141473d79 <.text+0x1472d79>
141473d02: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473d09: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
141473d10: e8 1b 5e 40 ff              	callq	0x140879b30 <.text+0x878b30>
141473d15: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473d1c: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473d24: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473d2c: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473d35: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473d3e: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473d47: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473d4d: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473d53: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473d59: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141473d5f: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473d65: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473d6c: 5b                          	popq	%rbx
141473d6d: 5f                          	popq	%rdi
141473d6e: 5e                          	popq	%rsi
141473d6f: 41 5c                       	popq	%r12
141473d71: 41 5d                       	popq	%r13
141473d73: 41 5e                       	popq	%r14
141473d75: 41 5f                       	popq	%r15
141473d77: 5d                          	popq	%rbp
141473d78: c3                          	retq
141473d79: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473d80: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473d88: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473d90: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473d99: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473da2: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473dab: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473db1: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473db7: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473dbd: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141473dc3: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473dc9: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473dd0: 5b                          	popq	%rbx
141473dd1: 5f                          	popq	%rdi
141473dd2: 5e                          	popq	%rsi
141473dd3: 41 5c                       	popq	%r12
141473dd5: 41 5d                       	popq	%r13
141473dd7: 41 5e                       	popq	%r14
141473dd9: 41 5f                       	popq	%r15
141473ddb: 5d                          	popq	%rbp
141473ddc: c3                          	retq
141473ddd: 0f 1f 00                    	nopl	(%rax)
141473de0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473de5: 55                          	pushq	%rbp
141473de6: 41 57                       	pushq	%r15
141473de8: 41 56                       	pushq	%r14
141473dea: 41 55                       	pushq	%r13
141473dec: 41 54                       	pushq	%r12
141473dee: 56                          	pushq	%rsi
141473def: 57                          	pushq	%rdi
141473df0: 53                          	pushq	%rbx
141473df1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473df8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141473dff: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473e05: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
141473e0c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473e12: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473e18: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473e1e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473e27: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473e30: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473e39: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473e41: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473e49: 48 b8 03 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
141473e53: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
141473e5a: 48 89 42 10                 	movq	%rax, 0x10(%rdx)
141473e5e: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
141473e65: 48 89 42 18                 	movq	%rax, 0x18(%rdx)
141473e69: 8b 85 58 01 00 00           	movl	0x158(%rbp), %eax
141473e6f: 89 42 20                    	movl	%eax, 0x20(%rdx)
141473e72: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
141473e79: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
141473e80: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
141473e84: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
141473e8b: 0f 11 01                    	movups	%xmm0, (%rcx)
141473e8e: 8b 85 20 03 00 00           	movl	0x320(%rbp), %eax
141473e94: 89 42 3c                    	movl	%eax, 0x3c(%rdx)
141473e97: 0f 10 85 70 02 00 00        	movups	0x270(%rbp), %xmm0
141473e9e: 0f 11 42 40                 	movups	%xmm0, 0x40(%rdx)
141473ea2: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
141473ea9: 48 89 42 50                 	movq	%rax, 0x50(%rdx)
141473ead: c6 85 37 03 00 00 01        	movb	$0x1, 0x337(%rbp)
141473eb4: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473ebc: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473ec4: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473ecd: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473ed6: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473edf: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473ee5: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473eeb: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473ef1: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
141473ef7: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473efd: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473f04: 5b                          	popq	%rbx
141473f05: 5f                          	popq	%rdi
141473f06: 5e                          	popq	%rsi
141473f07: 41 5c                       	popq	%r12
141473f09: 41 5d                       	popq	%r13
141473f0b: 41 5e                       	popq	%r14
141473f0d: 41 5f                       	popq	%r15
141473f0f: 5d                          	popq	%rbp
141473f10: c3                          	retq
141473f11: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141473f20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141473f25: 55                          	pushq	%rbp
141473f26: 41 57                       	pushq	%r15
141473f28: 41 56                       	pushq	%r14
141473f2a: 41 55                       	pushq	%r13
141473f2c: 41 54                       	pushq	%r12
141473f2e: 56                          	pushq	%rsi
141473f2f: 57                          	pushq	%rdi
141473f30: 53                          	pushq	%rbx
141473f31: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141473f38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141473f3f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141473f45: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
141473f4c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141473f52: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141473f58: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
141473f5e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141473f67: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141473f70: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141473f79: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141473f81: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141473f89: 80 bd 37 03 00 00 00        	cmpb	$0x0, 0x337(%rbp)
141473f90: 74 6a                       	je	0x141473ffc <.text+0x1472ffc>
141473f92: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
141473f99: e8 12 b9 41 ff              	callq	0x14088f8b0 <.text+0x88e8b0>
141473f9e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141473fa6: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141473fae: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141473fb7: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141473fc0: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141473fc9: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141473fcf: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141473fd5: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141473fdb: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141473fe2: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141473fe8: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141473fef: 5b                          	popq	%rbx
141473ff0: 5f                          	popq	%rdi
141473ff1: 5e                          	popq	%rsi
141473ff2: 41 5c                       	popq	%r12
141473ff4: 41 5d                       	popq	%r13
141473ff6: 41 5e                       	popq	%r14
141473ff8: 41 5f                       	popq	%r15
141473ffa: 5d                          	popq	%rbp
141473ffb: c3                          	retq
141473ffc: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141474004: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
14147400c: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141474015: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
14147401e: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141474027: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
14147402d: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141474033: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141474039: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141474040: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141474046: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
14147404d: 5b                          	popq	%rbx
14147404e: 5f                          	popq	%rdi
14147404f: 5e                          	popq	%rsi
141474050: 41 5c                       	popq	%r12
141474052: 41 5d                       	popq	%r13
141474054: 41 5e                       	popq	%r14
141474056: 41 5f                       	popq	%r15
141474058: 5d                          	popq	%rbp
141474059: c3                          	retq
14147405a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141474060: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141474065: 55                          	pushq	%rbp
141474066: 41 57                       	pushq	%r15
141474068: 41 56                       	pushq	%r14
14147406a: 41 55                       	pushq	%r13
14147406c: 41 54                       	pushq	%r12
14147406e: 56                          	pushq	%rsi
14147406f: 57                          	pushq	%rdi
141474070: 53                          	pushq	%rbx
141474071: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474078: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147407f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474085: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147408c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474092: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474098: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147409e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1414740a7: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1414740b0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1414740b9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1414740c1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1414740c9: 48 8b 95 40 01 00 00        	movq	0x140(%rbp), %rdx
1414740d0: 48 85 d2                    	testq	%rdx, %rdx
1414740d3: 74 16                       	je	0x1414740eb <.text+0x14730eb>
1414740d5: 48 c1 e2 05                 	shlq	$0x5, %rdx
1414740d9: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1414740df: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
1414740e6: e8 35 fb 19 ff              	callq	0x140613c20 <.text+0x612c20>
1414740eb: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414740f3: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414740fb: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
141474104: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
14147410d: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141474116: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
14147411c: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
141474122: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141474128: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
14147412f: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
141474135: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
14147413c: 5b                          	popq	%rbx
14147413d: 5f                          	popq	%rdi
14147413e: 5e                          	popq	%rsi
14147413f: 41 5c                       	popq	%r12
141474141: 41 5d                       	popq	%r13
141474143: 41 5e                       	popq	%r14
141474145: 41 5f                       	popq	%r15
141474147: 5d                          	popq	%rbp
141474148: c3                          	retq
141474149: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141474150: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141474155: 55                          	pushq	%rbp
141474156: 41 57                       	pushq	%r15
141474158: 41 56                       	pushq	%r14
14147415a: 41 55                       	pushq	%r13
14147415c: 41 54                       	pushq	%r12
14147415e: 56                          	pushq	%rsi
14147415f: 57                          	pushq	%rdi
141474160: 53                          	pushq	%rbx
141474161: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474168: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147416f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474175: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147417c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474182: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474188: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147418e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141474197: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1414741a0: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1414741a9: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1414741b1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1414741b9: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
1414741c0: e8 9b 0b 42 ff              	callq	0x140894d60 <.text+0x893d60>
1414741c5: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414741cd: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414741d5: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1414741de: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1414741e7: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1414741f0: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1414741f6: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414741fc: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
141474202: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
141474209: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
14147420f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
141474216: 5b                          	popq	%rbx
141474217: 5f                          	popq	%rdi
141474218: 5e                          	popq	%rsi
141474219: 41 5c                       	popq	%r12
14147421b: 41 5d                       	popq	%r13
14147421d: 41 5e                       	popq	%r14
14147421f: 41 5f                       	popq	%r15
141474221: 5d                          	popq	%rbp
141474222: c3                          	retq
141474223: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141474230: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141474235: 55                          	pushq	%rbp
141474236: 41 57                       	pushq	%r15
141474238: 41 56                       	pushq	%r14
14147423a: 41 55                       	pushq	%r13
14147423c: 41 54                       	pushq	%r12
14147423e: 56                          	pushq	%rsi
14147423f: 57                          	pushq	%rdi
141474240: 53                          	pushq	%rbx
141474241: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474248: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147424f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474255: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147425c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474262: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474268: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147426e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141474277: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141474280: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141474289: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141474291: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141474299: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1414742a0: e8 9b a8 41 ff              	callq	0x14088eb40 <.text+0x88db40>
1414742a5: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1414742ad: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414742b5: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1414742be: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1414742c7: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1414742d0: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1414742d6: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414742dc: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414742e2: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
1414742e9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414742ef: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1414742f6: 5b                          	popq	%rbx
1414742f7: 5f                          	popq	%rdi
1414742f8: 5e                          	popq	%rsi
1414742f9: 41 5c                       	popq	%r12
1414742fb: 41 5d                       	popq	%r13
1414742fd: 41 5e                       	popq	%r14
1414742ff: 41 5f                       	popq	%r15
141474301: 5d                          	popq	%rbp
141474302: c3                          	retq
141474303: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141474310: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141474315: 55                          	pushq	%rbp
141474316: 41 57                       	pushq	%r15
141474318: 41 56                       	pushq	%r14
14147431a: 41 55                       	pushq	%r13
14147431c: 41 54                       	pushq	%r12
14147431e: 56                          	pushq	%rsi
14147431f: 57                          	pushq	%rdi
141474320: 53                          	pushq	%rbx
141474321: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474328: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147432f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474335: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
14147433b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474341: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474347: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147434d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141474356: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
14147435f: 66 44 0f 7f 84 24 a0 00 00 00       	movdqa	%xmm8, 0xa0(%rsp)
141474369: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141474371: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141474379: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
141474380: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141474387: 48 89 08                    	movq	%rcx, (%rax)
14147438a: c6 85 35 03 00 00 00        	movb	$0x0, 0x335(%rbp)
141474391: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
141474399: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1414743a1: 66 44 0f 6f 84 24 a0 00 00 00       	movdqa	0xa0(%rsp), %xmm8
1414743ab: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1414743b4: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1414743bd: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1414743c3: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1414743c9: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414743cf: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1414743d5: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414743db: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1414743e2: 5b                          	popq	%rbx
1414743e3: 5f                          	popq	%rdi
1414743e4: 5e                          	popq	%rsi
1414743e5: 41 5c                       	popq	%r12
1414743e7: 41 5d                       	popq	%r13
1414743e9: 41 5e                       	popq	%r14
1414743eb: 41 5f                       	popq	%r15
1414743ed: 5d                          	popq	%rbp
1414743ee: c3                          	retq
1414743ef: 90                          	nop
1414743f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414743f5: 55                          	pushq	%rbp
1414743f6: 41 57                       	pushq	%r15
1414743f8: 41 56                       	pushq	%r14
1414743fa: 41 55                       	pushq	%r13
1414743fc: 41 54                       	pushq	%r12
1414743fe: 56                          	pushq	%rsi
1414743ff: 57                          	pushq	%rdi
141474400: 53                          	pushq	%rbx
141474401: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
141474408: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147440f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
141474415: 66 44 0f 7f 74 24 40        	movdqa	%xmm14, 0x40(%rsp)
14147441c: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
141474422: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
141474428: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
14147442e: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
141474437: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
141474440: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
141474449: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
141474451: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
141474459: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
141474460: e8 cb 87 8d ff              	callq	0x140d4cc30 <.text+0xd4bc30>
141474465: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
14147446d: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
141474475: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14147447e: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
141474487: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
141474490: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
141474496: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
14147449c: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1414744a2: 66 44 0f 6f 74 24 40        	movdqa	0x40(%rsp), %xmm14
1414744a9: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1414744af: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1414744b6: 5b                          	popq	%rbx
1414744b7: 5f                          	popq	%rdi
1414744b8: 5e                          	popq	%rsi
1414744b9: 41 5c                       	popq	%r12
1414744bb: 41 5d                       	popq	%r13
1414744bd: 41 5e                       	popq	%r14
1414744bf: 41 5f                       	popq	%r15
1414744c1: 5d                          	popq	%rbp
1414744c2: c3                          	retq
