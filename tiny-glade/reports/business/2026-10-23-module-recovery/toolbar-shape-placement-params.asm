
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14097f970: 55                          	pushq	%rbp
14097f971: 41 57                       	pushq	%r15
14097f973: 41 56                       	pushq	%r14
14097f975: 41 55                       	pushq	%r13
14097f977: 41 54                       	pushq	%r12
14097f979: 56                          	pushq	%rsi
14097f97a: 57                          	pushq	%rdi
14097f97b: 53                          	pushq	%rbx
14097f97c: 48 81 ec f8 01 00 00        	subq	$0x1f8, %rsp            # imm = 0x1F8
14097f983: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14097f98b: 44 0f 29 bd 60 01 00 00     	movaps	%xmm15, 0x160(%rbp)
14097f993: 44 0f 29 b5 50 01 00 00     	movaps	%xmm14, 0x150(%rbp)
14097f99b: 44 0f 29 ad 40 01 00 00     	movaps	%xmm13, 0x140(%rbp)
14097f9a3: 44 0f 29 a5 30 01 00 00     	movaps	%xmm12, 0x130(%rbp)
14097f9ab: 44 0f 29 9d 20 01 00 00     	movaps	%xmm11, 0x120(%rbp)
14097f9b3: 44 0f 29 95 10 01 00 00     	movaps	%xmm10, 0x110(%rbp)
14097f9bb: 44 0f 29 8d 00 01 00 00     	movaps	%xmm9, 0x100(%rbp)
14097f9c3: 44 0f 29 85 f0 00 00 00     	movaps	%xmm8, 0xf0(%rbp)
14097f9cb: 66 0f 7f bd e0 00 00 00     	movdqa	%xmm7, 0xe0(%rbp)
14097f9d3: 66 0f 7f b5 d0 00 00 00     	movdqa	%xmm6, 0xd0(%rbp)
14097f9db: 48 c7 85 c8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xc8(%rbp)
14097f9e6: 48 8b 31                    	movq	(%rcx), %rsi
14097f9e9: 83 3e 06                    	cmpl	$0x6, (%rsi)
14097f9ec: 0f 85 9f 0d 00 00           	jne	0x140980791 <.text+0x97f791>
14097f9f2: 0f b6 46 04                 	movzbl	0x4(%rsi), %eax
14097f9f6: fe c8                       	decb	%al
14097f9f8: 3c 02                       	cmpb	$0x2, %al
14097f9fa: 0f 83 91 0d 00 00           	jae	0x140980791 <.text+0x97f791>
14097fa00: 48 8b 8d 20 02 00 00        	movq	0x220(%rbp), %rcx
14097fa07: 48 8b 01                    	movq	(%rcx), %rax
14097fa0a: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
14097fa0e: 4c 8b 10                    	movq	(%rax), %r10
14097fa11: 45 31 db                    	xorl	%r11d, %r11d
14097fa14: 4c 89 d7                    	movq	%r10, %rdi
14097fa17: 48 2b 79 18                 	subq	0x18(%rcx), %rdi
14097fa1b: 49 0f 42 fb                 	cmovbq	%r11, %rdi
14097fa1f: 4c 2b 51 38                 	subq	0x38(%rcx), %r10
14097fa23: 4d 0f 42 d3                 	cmovbq	%r11, %r10
14097fa27: 48 8b 59 30                 	movq	0x30(%rcx), %rbx
14097fa2b: 4c 29 d3                    	subq	%r10, %rbx
14097fa2e: 49 0f 42 db                 	cmovbq	%r11, %rbx
14097fa32: 41 0f 96 c2                 	setbe	%r10b
14097fa36: 4c 8b 71 10                 	movq	0x10(%rcx), %r14
14097fa3a: 49 29 fe                    	subq	%rdi, %r14
14097fa3d: 4d 0f 42 f3                 	cmovbq	%r11, %r14
14097fa41: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
14097fa45: 41 0f 96 c3                 	setbe	%r11b
14097fa49: 49 01 de                    	addq	%rbx, %r14
14097fa4c: 48 89 cf                    	movq	%rcx, %rdi
14097fa4f: 4c 29 f7                    	subq	%r14, %rdi
14097fa52: 48 89 38                    	movq	%rdi, (%rax)
14097fa55: 45 84 d3                    	testb	%r10b, %r11b
14097fa58: 74 1f                       	je	0x14097fa79 <.text+0x97ea79>
14097fa5a: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
14097fa61: 48 8b 0a                    	movq	(%rdx), %rcx
14097fa64: f6 01 01                    	testb	$0x1, (%rcx)
14097fa67: 74 18                       	je	0x14097fa81 <.text+0x97ea81>
14097fa69: 8b 51 0c                    	movl	0xc(%rcx), %edx
14097fa6c: 89 55 30                    	movl	%edx, 0x30(%rbp)
14097fa6f: 48 8b 49 04                 	movq	0x4(%rcx), %rcx
14097fa73: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
14097fa77: eb 17                       	jmp	0x14097fa90 <.text+0x97ea90>
14097fa79: 48 89 08                    	movq	%rcx, (%rax)
14097fa7c: e9 10 0d 00 00              	jmp	0x140980791 <.text+0x97f791>
14097fa81: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
14097fa88: 48 c7 45 28 00 00 00 00     	movq	$0x0, 0x28(%rbp)
14097fa90: 4c 8b bd 00 02 00 00        	movq	0x200(%rbp), %r15
14097fa97: 4c 8b ad f0 01 00 00        	movq	0x1f0(%rbp), %r13
14097fa9e: 48 8b 9d e0 01 00 00        	movq	0x1e0(%rbp), %rbx
14097faa5: 48 8b 10                    	movq	(%rax), %rdx
14097faa8: 48 85 d2                    	testq	%rdx, %rdx
14097faab: 0f 84 a2 00 00 00           	je	0x14097fb53 <.text+0x97eb53>
14097fab1: 4c 89 ef                    	movq	%r13, %rdi
14097fab4: 4d 89 fd                    	movq	%r15, %r13
14097fab7: 4d 89 cc                    	movq	%r9, %r12
14097faba: 4d 89 c7                    	movq	%r8, %r15
14097fabd: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
14097fac4: 4c 8b 00                    	movq	(%rax), %r8
14097fac7: c7 44 24 30 fb 00 00 00     	movl	$0xfb, 0x30(%rsp)
14097facf: c7 44 24 28 ff ff 7f 7f     	movl	$0x7f7fffff, 0x28(%rsp) # imm = 0x7F7FFFFF
14097fad7: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
14097fadb: 45 31 c9                    	xorl	%r9d, %r9d
14097fade: e8 7d e1 f4 ff              	callq	0x1408cdc60 <.text+0x8ccc60>
14097fae3: 83 bd a4 00 00 00 02        	cmpl	$0x2, 0xa4(%rbp)
14097faea: 75 0e                       	jne	0x14097fafa <.text+0x97eafa>
14097faec: 4d 89 f8                    	movq	%r15, %r8
14097faef: 4d 89 e1                    	movq	%r12, %r9
14097faf2: 4d 89 ef                    	movq	%r13, %r15
14097faf5: 49 89 fd                    	movq	%rdi, %r13
14097faf8: eb 59                       	jmp	0x14097fb53 <.text+0x97eb53>
14097fafa: 44 8b b5 88 00 00 00        	movl	0x88(%rbp), %r14d
14097fb01: 66 41 0f 6e fe              	movd	%r14d, %xmm7
14097fb06: f3 44 0f 10 ad 8c 00 00 00  	movss	0x8c(%rbp), %xmm13
14097fb0f: 8b 9d 90 00 00 00           	movl	0x90(%rbp), %ebx
14097fb15: 66 0f 6e f3                 	movd	%ebx, %xmm6
14097fb19: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
14097fb20: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14097fb24: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14097fb28: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
14097fb2c: 0f b6 45 10                 	movzbl	0x10(%rbp), %eax
14097fb30: 48 8d 0d ed 79 13 02        	leaq	0x21379ed(%rip), %rcx   # 0x142ab7524
14097fb37: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
14097fb3b: 48 01 c8                    	addq	%rcx, %rax
14097fb3e: 4d 89 f8                    	movq	%r15, %r8
14097fb41: ff e0                       	jmpq	*%rax
14097fb43: 4d 89 e1                    	movq	%r12, %r9
14097fb46: 4d 89 ef                    	movq	%r13, %r15
14097fb49: 49 89 fd                    	movq	%rdi, %r13
14097fb4c: 48 8b 9d e0 01 00 00        	movq	0x1e0(%rbp), %rbx
14097fb53: 49 8b 09                    	movq	(%r9), %rcx
14097fb56: 4d 8b 20                    	movq	(%r8), %r12
14097fb59: 8b 45 30                    	movl	0x30(%rbp), %eax
14097fb5c: 89 45 40                    	movl	%eax, 0x40(%rbp)
14097fb5f: 48 8b 45 28                 	movq	0x28(%rbp), %rax
14097fb63: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14097fb67: 4c 8b 33                    	movq	(%rbx), %r14
14097fb6a: 48 8b 5b 08                 	movq	0x8(%rbx), %rbx
14097fb6e: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
14097fb76: 48 c7 45 18 08 00 00 00     	movq	$0x8, 0x18(%rbp)
14097fb7e: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
14097fb86: f3 0f 10 45 40              	movss	0x40(%rbp), %xmm0
14097fb8b: f3 0f 10 4d 38              	movss	0x38(%rbp), %xmm1
14097fb90: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
14097fb93: f2 0f 12 c1                 	movddup	%xmm1, %xmm0            # xmm0 = xmm1[0,0]
14097fb97: 0f 58 05 92 6a 13 02        	addps	0x2136a92(%rip), %xmm0  # 0x142ab6630
14097fb9e: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
14097fba2: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
14097fba6: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
14097fbaa: e8 61 5e 12 00              	callq	0x140aa5a10 <.text+0xaa4a10>
14097fbaf: 48 8b 45 18                 	movq	0x18(%rbp), %rax
14097fbb3: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
14097fbb7: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
14097fbbb: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14097fbbf: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
14097fbc3: 4c 89 a5 80 00 00 00        	movq	%r12, 0x80(%rbp)
14097fbca: 48 8d 7d 38                 	leaq	0x38(%rbp), %rdi
14097fbce: 48 89 bd 88 00 00 00        	movq	%rdi, 0x88(%rbp)
14097fbd5: 4c 8d 05 dc 6a 13 02        	leaq	0x2136adc(%rip), %r8    # 0x142ab66b8
14097fbdc: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
14097fbe0: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
14097fbe4: e8 27 ea 97 00              	callq	0x1412fe610 <.text+0x12fd610>
14097fbe9: 48 8b 75 58                 	movq	0x58(%rbp), %rsi
14097fbed: 48 85 f6                    	testq	%rsi, %rsi
14097fbf0: 0f 84 0a 01 00 00           	je	0x14097fd00 <.text+0x97ed00>
14097fbf6: 48 8b 45 50                 	movq	0x50(%rbp), %rax
14097fbfa: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14097fc01: 48 89 7d 08                 	movq	%rdi, 0x8(%rbp)
14097fc05: 48 8d 45 08                 	leaq	0x8(%rbp), %rax
14097fc09: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14097fc0d: 48 83 fe 01                 	cmpq	$0x1, %rsi
14097fc11: 0f 85 ca 0c 00 00           	jne	0x1409808e1 <.text+0x97f8e1>
14097fc17: 4c 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %r10
14097fc1e: 41 8b 42 1c                 	movl	0x1c(%r10), %eax
14097fc22: 89 45 78                    	movl	%eax, 0x78(%rbp)
14097fc25: 49 8b 42 14                 	movq	0x14(%r10), %rax
14097fc29: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14097fc2d: f3 41 0f 7e 72 08           	movq	0x8(%r10), %xmm6
14097fc33: 49 8b 32                    	movq	(%r10), %rsi
14097fc36: 66 41 0f 6e 7a 10           	movd	0x10(%r10), %xmm7
14097fc3c: 48 89 75 08                 	movq	%rsi, 0x8(%rbp)
14097fc40: 49 83 7c 24 48 00           	cmpq	$0x0, 0x48(%r12)
14097fc46: 0f 84 21 02 00 00           	je	0x14097fe6d <.text+0x97ee6d>
14097fc4c: 49 8d 4c 24 50              	leaq	0x50(%r12), %rcx
14097fc51: 48 8d 55 08                 	leaq	0x8(%rbp), %rdx
14097fc55: e8 16 11 c3 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14097fc5a: 49 8b 4c 24 30              	movq	0x30(%r12), %rcx
14097fc5f: 49 8b 54 24 38              	movq	0x38(%r12), %rdx
14097fc64: 49 89 d0                    	movq	%rdx, %r8
14097fc67: 49 21 c0                    	andq	%rax, %r8
14097fc6a: 48 c1 e8 39                 	shrq	$0x39, %rax
14097fc6e: 66 0f 6e c0                 	movd	%eax, %xmm0
14097fc72: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14097fc76: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14097fc7b: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14097fc80: 48 8d 41 f0                 	leaq	-0x10(%rcx), %rax
14097fc84: 45 31 c9                    	xorl	%r9d, %r9d
14097fc87: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14097fc8b: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14097fc91: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14097fc95: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14097fc99: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
14097fc9e: 45 85 d2                    	testl	%r10d, %r10d
14097fca1: 74 34                       	je	0x14097fcd7 <.text+0x97ecd7>
14097fca3: 4d 89 fc                    	movq	%r15, %r12
14097fca6: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14097fcab: 4d 01 c3                    	addq	%r8, %r11
14097fcae: 49 21 d3                    	andq	%rdx, %r11
14097fcb1: 4c 89 df                    	movq	%r11, %rdi
14097fcb4: 48 c1 e7 04                 	shlq	$0x4, %rdi
14097fcb8: 49 89 c7                    	movq	%rax, %r15
14097fcbb: 49 29 ff                    	subq	%rdi, %r15
14097fcbe: 49 39 37                    	cmpq	%rsi, (%r15)
14097fcc1: 0f 84 8a 00 00 00           	je	0x14097fd51 <.text+0x97ed51>
14097fcc7: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14097fccb: 66 45 21 d3                 	andw	%r10w, %r11w
14097fccf: 45 89 da                    	movl	%r11d, %r10d
14097fcd2: 4d 89 e7                    	movq	%r12, %r15
14097fcd5: 75 cc                       	jne	0x14097fca3 <.text+0x97eca3>
14097fcd7: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14097fcdb: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14097fce0: 45 85 d2                    	testl	%r10d, %r10d
14097fce3: 4c 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %r10
14097fcea: 0f 85 7d 01 00 00           	jne	0x14097fe6d <.text+0x97ee6d>
14097fcf0: 4d 01 c8                    	addq	%r9, %r8
14097fcf3: 49 83 c0 10                 	addq	$0x10, %r8
14097fcf7: 49 83 c1 10                 	addq	$0x10, %r9
14097fcfb: 49 21 d0                    	andq	%rdx, %r8
14097fcfe: eb 8b                       	jmp	0x14097fc8b <.text+0x97ec8b>
14097fd00: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
14097fd04: 48 85 d2                    	testq	%rdx, %rdx
14097fd07: 74 13                       	je	0x14097fd1c <.text+0x97ed1c>
14097fd09: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
14097fd0d: 48 c1 e2 05                 	shlq	$0x5, %rdx
14097fd11: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14097fd17: e8 04 3f c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097fd1c: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
14097fd20: 48 85 d2                    	testq	%rdx, %rdx
14097fd23: 74 13                       	je	0x14097fd38 <.text+0x97ed38>
14097fd25: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
14097fd29: 48 c1 e2 03                 	shlq	$0x3, %rdx
14097fd2d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14097fd33: e8 e8 3e c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097fd38: 49 8b 07                    	movq	(%r15), %rax
14097fd3b: 80 38 01                    	cmpb	$0x1, (%rax)
14097fd3e: 75 38                       	jne	0x14097fd78 <.text+0x97ed78>
14097fd40: 41 8b 5d 1c                 	movl	0x1c(%r13), %ebx
14097fd44: 49 8b 7d 00                 	movq	(%r13), %rdi
14097fd48: 4d 8b 75 10                 	movq	0x10(%r13), %r14
14097fd4c: e9 28 02 00 00              	jmp	0x14097ff79 <.text+0x97ef79>
14097fd51: 49 f7 db                    	negq	%r11
14097fd54: 49 c1 e3 04                 	shlq	$0x4, %r11
14097fd58: 4a 8b 4c 19 f8              	movq	-0x8(%rcx,%r11), %rcx
14097fd5d: 48 89 c8                    	movq	%rcx, %rax
14097fd60: 48 c1 e8 20                 	shrq	$0x20, %rax
14097fd64: 4d 89 e7                    	movq	%r12, %r15
14097fd67: 75 6b                       	jne	0x14097fdd4 <.text+0x97edd4>
14097fd69: 45 31 e4                    	xorl	%r12d, %r12d
14097fd6c: 4c 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %r10
14097fd73: e9 f8 00 00 00              	jmp	0x14097fe70 <.text+0x97ee70>
14097fd78: 8b 45 30                    	movl	0x30(%rbp), %eax
14097fd7b: 89 45 7b                    	movl	%eax, 0x7b(%rbp)
14097fd7e: 48 8b 45 28                 	movq	0x28(%rbp), %rax
14097fd82: 48 89 45 73                 	movq	%rax, 0x73(%rbp)
14097fd86: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
14097fd8a: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
14097fd8e: 89 01                       	movl	%eax, (%rcx)
14097fd90: 49 8b 45 00                 	movq	(%r13), %rax
14097fd94: c7 00 00 00 00 00           	movl	$0x0, (%rax)
14097fd9a: c7 40 08 00 00 00 00        	movl	$0x0, 0x8(%rax)
14097fda1: c6 40 2c 02                 	movb	$0x2, 0x2c(%rax)
14097fda5: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14097fda9: 48 8b 55 77                 	movq	0x77(%rbp), %rdx
14097fdad: 48 89 48 2d                 	movq	%rcx, 0x2d(%rax)
14097fdb1: 48 89 50 34                 	movq	%rdx, 0x34(%rax)
14097fdb5: 0f 57 c0                    	xorps	%xmm0, %xmm0
14097fdb8: 0f 11 40 3c                 	movups	%xmm0, 0x3c(%rax)
14097fdbc: c7 40 4c 00 00 00 00        	movl	$0x0, 0x4c(%rax)
14097fdc3: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
14097fdca: 48 8b 19                    	movq	(%rcx), %rbx
14097fdcd: 31 f6                       	xorl	%esi, %esi
14097fdcf: e9 b1 09 00 00              	jmp	0x140980785 <.text+0x97f785>
14097fdd4: 89 c9                       	movl	%ecx, %ecx
14097fdd6: 48 3b 4b 10                 	cmpq	0x10(%rbx), %rcx
14097fdda: 4c 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %r10
14097fde1: 0f 83 86 00 00 00           	jae	0x14097fe6d <.text+0x97ee6d>
14097fde7: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
14097fdeb: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14097fdef: 39 04 8a                    	cmpl	%eax, (%rdx,%rcx,4)
14097fdf2: 75 79                       	jne	0x14097fe6d <.text+0x97ee6d>
14097fdf4: 48 8d 04 8a                 	leaq	(%rdx,%rcx,4), %rax
14097fdf8: 8b 48 04                    	movl	0x4(%rax), %ecx
14097fdfb: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14097fe00: 48 39 d1                    	cmpq	%rdx, %rcx
14097fe03: 74 68                       	je	0x14097fe6d <.text+0x97ee6d>
14097fe05: 49 39 4e 38                 	cmpq	%rcx, 0x38(%r14)
14097fe09: 76 62                       	jbe	0x14097fe6d <.text+0x97ee6d>
14097fe0b: 49 8b 56 28                 	movq	0x28(%r14), %rdx
14097fe0f: 41 89 c8                    	movl	%ecx, %r8d
14097fe12: 41 c1 e8 06                 	shrl	$0x6, %r8d
14097fe16: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
14097fe1a: 48 0f a3 ca                 	btq	%rcx, %rdx
14097fe1e: 73 4d                       	jae	0x14097fe6d <.text+0x97ee6d>
14097fe20: 8b 48 0c                    	movl	0xc(%rax), %ecx
14097fe23: 8b 40 10                    	movl	0x10(%rax), %eax
14097fe26: 48 8b 93 a8 01 00 00        	movq	0x1a8(%rbx), %rdx
14097fe2d: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14097fe31: 4d 8b 86 10 01 00 00        	movq	0x110(%r14), %r8
14097fe38: 4c 8b 4c ca 18              	movq	0x18(%rdx,%rcx,8), %r9
14097fe3d: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
14097fe42: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
14097fe46: 48 f7 d1                    	notq	%rcx
14097fe49: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14097fe4d: 48 c1 e1 04                 	shlq	$0x4, %rcx
14097fe51: 49 8b 4c 09 10              	movq	0x10(%r9,%rcx), %rcx
14097fe56: 48 6b c0 58                 	imulq	$0x58, %rax, %rax
14097fe5a: f3 0f 10 05 22 5b fa 01     	movss	0x1fa5b22(%rip), %xmm0  # 0x142925984
14097fe62: 0f 2e 44 01 30              	ucomiss	0x30(%rcx,%rax), %xmm0
14097fe67: 41 0f 93 c4                 	setae	%r12b
14097fe6b: eb 03                       	jmp	0x14097fe70 <.text+0x97ee70>
14097fe6d: 45 31 e4                    	xorl	%r12d, %r12d
14097fe70: 49 8d 42 14                 	leaq	0x14(%r10), %rax
14097fe74: 8b 48 08                    	movl	0x8(%rax), %ecx
14097fe77: 89 4d f8                    	movl	%ecx, -0x8(%rbp)
14097fe7a: 48 8b 00                    	movq	(%rax), %rax
14097fe7d: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14097fe81: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14097fe85: 48 89 45 e4                 	movq	%rax, -0x1c(%rbp)
14097fe89: 8b 45 78                    	movl	0x78(%rbp), %eax
14097fe8c: 89 45 ec                    	movl	%eax, -0x14(%rbp)
14097fe8f: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
14097fe93: 48 85 d2                    	testq	%rdx, %rdx
14097fe96: 74 12                       	je	0x14097feaa <.text+0x97eeaa>
14097fe98: 48 c1 e2 05                 	shlq	$0x5, %rdx
14097fe9c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14097fea2: 4c 89 d1                    	movq	%r10, %rcx
14097fea5: e8 76 3d c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097feaa: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
14097feae: 48 85 d2                    	testq	%rdx, %rdx
14097feb1: 74 13                       	je	0x14097fec6 <.text+0x97eec6>
14097feb3: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
14097feb7: 48 c1 e2 03                 	shlq	$0x3, %rdx
14097febb: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14097fec1: e8 5a 3d c9 ff              	callq	0x140613c20 <.text+0x612c20>
14097fec6: 41 8b 5d 1c                 	movl	0x1c(%r13), %ebx
14097feca: 4d 8b 75 10                 	movq	0x10(%r13), %r14
14097fece: 41 89 1e                    	movl	%ebx, (%r14)
14097fed1: 49 8b 7d 00                 	movq	(%r13), %rdi
14097fed5: c7 07 00 00 00 00           	movl	$0x0, (%rdi)
14097fedb: c7 47 08 01 00 00 00        	movl	$0x1, 0x8(%rdi)
14097fee2: 66 0f d6 77 18              	movq	%xmm6, 0x18(%rdi)
14097fee7: 48 89 77 20                 	movq	%rsi, 0x20(%rdi)
14097feeb: 66 0f 7e 7f 28              	movd	%xmm7, 0x28(%rdi)
14097fef0: 44 88 67 2c                 	movb	%r12b, 0x2c(%rdi)
14097fef4: 48 8b 45 e1                 	movq	-0x1f(%rbp), %rax
14097fef8: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
14097fefc: 48 89 47 2d                 	movq	%rax, 0x2d(%rdi)
14097ff00: 48 89 4f 34                 	movq	%rcx, 0x34(%rdi)
14097ff04: 48 c7 47 3c 00 00 00 00     	movq	$0x0, 0x3c(%rdi)
14097ff0c: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14097ff10: 48 89 47 44                 	movq	%rax, 0x44(%rdi)
14097ff14: 8b 45 f8                    	movl	-0x8(%rbp), %eax
14097ff17: 89 47 4c                    	movl	%eax, 0x4c(%rdi)
14097ff1a: 40 b6 02                    	movb	$0x2, %sil
14097ff1d: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
14097ff24: 49 8b 0f                    	movq	(%r15), %rcx
14097ff27: 80 39 00                    	cmpb	$0x0, (%rcx)
14097ff2a: 0f 84 81 07 00 00           	je	0x1409806b1 <.text+0x97f6b1>
14097ff30: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14097ff33: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14097ff37: 89 0a                       	movl	%ecx, (%rdx)
14097ff39: 48 8b 30                    	movq	(%rax), %rsi
14097ff3c: 4c 8b 7e 30                 	movq	0x30(%rsi), %r15
14097ff40: 4c 8b 66 40                 	movq	0x40(%rsi), %r12
14097ff44: 4c 3b 7e 20                 	cmpq	0x20(%rsi), %r15
14097ff48: 75 10                       	jne	0x14097ff5a <.text+0x97ef5a>
14097ff4a: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14097ff4e: 48 8d 15 f3 a1 2b 02        	leaq	0x22ba1f3(%rip), %rdx   # 0x142c3a148
14097ff55: e8 d6 26 be ff              	callq	0x140562630 <.text+0x561630>
14097ff5a: 48 8b 46 28                 	movq	0x28(%rsi), %rax
14097ff5e: 4c 89 f9                    	movq	%r15, %rcx
14097ff61: 48 c1 e1 04                 	shlq	$0x4, %rcx
14097ff65: 4c 89 24 08                 	movq	%r12, (%rax,%rcx)
14097ff69: c6 44 08 08 01              	movb	$0x1, 0x8(%rax,%rcx)
14097ff6e: 49 ff c7                    	incq	%r15
14097ff71: 4c 89 7e 30                 	movq	%r15, 0x30(%rsi)
14097ff75: 48 ff 46 40                 	incq	0x40(%rsi)
14097ff79: 8b 45 30                    	movl	0x30(%rbp), %eax
14097ff7c: 89 45 7b                    	movl	%eax, 0x7b(%rbp)
14097ff7f: 48 8b 45 28                 	movq	0x28(%rbp), %rax
14097ff83: 48 89 45 73                 	movq	%rax, 0x73(%rbp)
14097ff87: 41 89 1e                    	movl	%ebx, (%r14)
14097ff8a: c7 07 00 00 00 00           	movl	$0x0, (%rdi)
14097ff90: c7 47 08 00 00 00 00        	movl	$0x0, 0x8(%rdi)
14097ff97: c6 47 2c 02                 	movb	$0x2, 0x2c(%rdi)
14097ff9b: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14097ff9f: 48 8b 4d 77                 	movq	0x77(%rbp), %rcx
14097ffa3: 48 89 47 2d                 	movq	%rax, 0x2d(%rdi)
14097ffa7: 48 89 4f 34                 	movq	%rcx, 0x34(%rdi)
14097ffab: 66 0f ef c0                 	pxor	%xmm0, %xmm0
14097ffaf: f3 0f 7f 47 3c              	movdqu	%xmm0, 0x3c(%rdi)
14097ffb4: c7 47 4c 00 00 00 00        	movl	$0x0, 0x4c(%rdi)
14097ffbb: e9 d1 07 00 00              	jmp	0x140980791 <.text+0x97f791>
14097ffc0: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
14097ffc4: 49 8b 08                    	movq	(%r8), %rcx
14097ffc7: 4c 8d 05 1a 70 13 02        	leaq	0x213701a(%rip), %r8    # 0x142ab6fe8
14097ffce: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
14097ffd5: 49 89 d4                    	movq	%rdx, %r12
14097ffd8: e8 13 1f 11 00              	callq	0x140a91ef0 <.text+0xa90ef0>
14097ffdd: 48 85 c0                    	testq	%rax, %rax
14097ffe0: 0f 84 bb 09 00 00           	je	0x1409809a1 <.text+0x97f9a1>
14097ffe6: 48 89 c3                    	movq	%rax, %rbx
14097ffe9: 4d 89 ef                    	movq	%r13, %r15
14097ffec: 4c 8d 70 38                 	leaq	0x38(%rax), %r14
14097fff0: f3 44 0f 5d 2d d3 18 05 02  	minss	0x20518d3(%rip), %xmm13 # 0x1429d18cc
14097fff9: 4c 8d 0d 18 70 13 02        	leaq	0x2137018(%rip), %r9    # 0x142ab7018
140980000: 4c 89 f1                    	movq	%r14, %rcx
140980003: 66 0f 6f cf                 	movdqa	%xmm7, %xmm1
140980007: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
14098000b: e8 40 99 ae 00              	callq	0x141469950 <.text+0x1468950>
140980010: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
140980014: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140980018: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14098001c: f3 44 0f 10 25 df 58 fa 01  	movss	0x1fa58df(%rip), %xmm12 # 0x142925904
140980025: 45 0f 28 c4                 	movaps	%xmm12, %xmm8
140980029: f3 44 0f 5d c1              	minss	%xmm1, %xmm8
14098002e: 4c 8d 0d fb 6f 13 02        	leaq	0x2136ffb(%rip), %r9    # 0x142ab7030
140980035: 48 89 d9                    	movq	%rbx, %rcx
140980038: 66 0f 6f cf                 	movdqa	%xmm7, %xmm1
14098003c: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
140980040: e8 db 99 ae 00              	callq	0x141469a20 <.text+0x1468a20>
140980045: f3 44 0f 5f c8              	maxss	%xmm0, %xmm9
14098004a: f3 45 0f 5d e1              	minss	%xmm9, %xmm12
14098004f: 8b 83 58 02 00 00           	movl	0x258(%rbx), %eax
140980055: 83 f8 02                    	cmpl	$0x2, %eax
140980058: 0f 83 ca 00 00 00           	jae	0x140980128 <.text+0x97f128>
14098005e: a8 01                       	testb	$0x1, %al
140980060: 0f 84 e0 00 00 00           	je	0x140980146 <.text+0x97f146>
140980066: f3 44 0f 10 8b 5c 02 00 00  	movss	0x25c(%rbx), %xmm9
14098006f: f3 44 0f 10 93 60 02 00 00  	movss	0x260(%rbx), %xmm10
140980078: e9 e1 00 00 00              	jmp	0x14098015e <.text+0x97f15e>
14098007d: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
140980081: 44 0f b6 7d 11              	movzbl	0x11(%rbp), %r15d
140980086: 49 8b 08                    	movq	(%r8), %rcx
140980089: 4c 8d 05 b0 6e 13 02        	leaq	0x2136eb0(%rip), %r8    # 0x142ab6f40
140980090: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
140980094: 49 89 d4                    	movq	%rdx, %r12
140980097: e8 54 1e 11 00              	callq	0x140a91ef0 <.text+0xa90ef0>
14098009c: 48 85 c0                    	testq	%rax, %rax
14098009f: 0f 84 08 09 00 00           	je	0x1409809ad <.text+0x97f9ad>
1409800a5: 44 88 bd bf 00 00 00        	movb	%r15b, 0xbf(%rbp)
1409800ac: 83 b8 58 02 00 00 02        	cmpl	$0x2, 0x258(%rax)
1409800b3: 0f 83 00 09 00 00           	jae	0x1409809b9 <.text+0x97f9b9>
1409800b9: 4d 89 ef                    	movq	%r13, %r15
1409800bc: 49 89 c5                    	movq	%rax, %r13
1409800bf: 48 8d 48 38                 	leaq	0x38(%rax), %rcx
1409800c3: 4c 8d 0d a6 6e 13 02        	leaq	0x2136ea6(%rip), %r9    # 0x142ab6f70
1409800ca: 66 0f 6f cf                 	movdqa	%xmm7, %xmm1
1409800ce: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
1409800d2: e8 79 98 ae 00              	callq	0x141469950 <.text+0x1468950>
1409800d7: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
1409800db: 45 0f 57 db                 	xorps	%xmm11, %xmm11
1409800df: f3 44 0f 10 05 1c 58 fa 01  	movss	0x1fa581c(%rip), %xmm8  # 0x142925904
1409800e8: 4c 8d 0d 99 6e 13 02        	leaq	0x2136e99(%rip), %r9    # 0x142ab6f88
1409800ef: 4c 89 e9                    	movq	%r13, %rcx
1409800f2: 66 0f 6f cf                 	movdqa	%xmm7, %xmm1
1409800f6: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
1409800fa: e8 21 99 ae 00              	callq	0x141469a20 <.text+0x1468a20>
1409800ff: 4c 89 e9                    	movq	%r13, %rcx
140980102: 48 81 c1 5c 02 00 00        	addq	$0x25c, %rcx            # imm = 0x25C
140980109: 4c 89 ad c0 00 00 00        	movq	%r13, 0xc0(%rbp)
140980110: 41 f6 85 58 02 00 00 01     	testb	$0x1, 0x258(%r13)
140980118: 0f 84 31 01 00 00           	je	0x14098024f <.text+0x97f24f>
14098011e: f2 44 0f 10 11              	movsd	(%rcx), %xmm10
140980123: e9 44 01 00 00              	jmp	0x14098026c <.text+0x97f26c>
140980128: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
14098012c: 4c 89 f2                    	movq	%r14, %rdx
14098012f: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
140980133: e8 a8 8d ae 00              	callq	0x141468ee0 <.text+0x1467ee0>
140980138: f3 44 0f 10 4d 70           	movss	0x70(%rbp), %xmm9
14098013e: f3 44 0f 10 55 78           	movss	0x78(%rbp), %xmm10
140980144: eb 18                       	jmp	0x14098015e <.text+0x97f15e>
140980146: 48 8d 8b 5c 02 00 00        	leaq	0x25c(%rbx), %rcx
14098014d: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
140980151: e8 8a 77 31 00              	callq	0x140c978e0 <.text+0xc968e0>
140980156: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
14098015a: 44 0f 28 d1                 	movaps	%xmm1, %xmm10
14098015e: f3 0f 10 05 6a 57 fa 01     	movss	0x1fa576a(%rip), %xmm0  # 0x1429258d0
140980166: 41 0f 2e c5                 	ucomiss	%xmm13, %xmm0
14098016a: 45 0f 28 dd                 	movaps	%xmm13, %xmm11
14098016e: 76 2b                       	jbe	0x14098019b <.text+0x97f19b>
140980170: 4c 8d 0d d1 6e 13 02        	leaq	0x2136ed1(%rip), %r9    # 0x142ab7048
140980177: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
14098017b: 4c 89 f2                    	movq	%r14, %rdx
14098017e: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
140980182: e8 49 70 ae 00              	callq	0x1414671d0 <.text+0x14661d0>
140980187: 66 0f 6e 7d 70              	movd	0x70(%rbp), %xmm7
14098018c: f3 44 0f 10 5d 74           	movss	0x74(%rbp), %xmm11
140980192: 66 0f 6e 75 78              	movd	0x78(%rbp), %xmm6
140980197: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
14098019b: 4c 8d 0d be 6e 13 02        	leaq	0x2136ebe(%rip), %r9    # 0x142ab7060
1409801a2: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409801a6: 4c 89 f2                    	movq	%r14, %rdx
1409801a9: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1409801ad: e8 1e 70 ae 00              	callq	0x1414671d0 <.text+0x14661d0>
1409801b2: f3 0f 10 45 70              	movss	0x70(%rbp), %xmm0
1409801b7: f3 0f 10 4d 78              	movss	0x78(%rbp), %xmm1
1409801bc: f3 0f 11 45 48              	movss	%xmm0, 0x48(%rbp)
1409801c1: f3 0f 11 4d 4c              	movss	%xmm1, 0x4c(%rbp)
1409801c6: 48 8d 4d d4                 	leaq	-0x2c(%rbp), %rcx
1409801ca: 48 8d 55 48                 	leaq	0x48(%rbp), %rdx
1409801ce: 41 0f 28 d3                 	movaps	%xmm11, %xmm2
1409801d2: e8 09 cc 31 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409801d7: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409801da: 44 0f 2e e8                 	ucomiss	%xmm0, %xmm13
1409801de: 0f 86 60 06 00 00           	jbe	0x140980844 <.text+0x97f844>
1409801e4: 8b 83 58 02 00 00           	movl	0x258(%rbx), %eax
1409801ea: 83 f8 02                    	cmpl	$0x2, %eax
1409801ed: 0f 83 51 06 00 00           	jae	0x140980844 <.text+0x97f844>
1409801f3: 83 3e 06                    	cmpl	$0x6, (%rsi)
1409801f6: 0f 85 48 06 00 00           	jne	0x140980844 <.text+0x97f844>
1409801fc: 80 7e 04 02                 	cmpb	$0x2, 0x4(%rsi)
140980200: 0f 85 3e 06 00 00           	jne	0x140980844 <.text+0x97f844>
140980206: 48 81 c3 5c 02 00 00        	addq	$0x25c, %rbx            # imm = 0x25C
14098020d: a8 01                       	testb	$0x1, %al
14098020f: 0f 84 de 05 00 00           	je	0x1409807f3 <.text+0x97f7f3>
140980215: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140980219: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140980220: 0f 10 03                    	movups	(%rbx), %xmm0
140980223: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140980227: f2 0f 10 85 80 00 00 00     	movsd	0x80(%rbp), %xmm0
14098022f: 0f 58 05 ea 63 13 02        	addps	0x21363ea(%rip), %xmm0  # 0x142ab6620
140980236: 0f 13 85 80 00 00 00        	movlps	%xmm0, 0x80(%rbp)
14098023d: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140980241: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
140980245: e8 b6 95 31 00              	callq	0x140c99800 <.text+0xc98800>
14098024a: e9 ca 05 00 00              	jmp	0x140980819 <.text+0x97f819>
14098024f: 66 0f ef d2                 	pxor	%xmm2, %xmm2
140980253: f3 0f 5f d0                 	maxss	%xmm0, %xmm2
140980257: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14098025b: f3 0f 5d ca                 	minss	%xmm2, %xmm1
14098025f: e8 7c 76 31 00              	callq	0x140c978e0 <.text+0xc968e0>
140980264: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
140980268: 44 0f 14 d1                 	unpcklps	%xmm1, %xmm10           # xmm10 = xmm10[0],xmm1[0],xmm10[1],xmm1[1]
14098026c: 8b 0e                       	movl	(%rsi), %ecx
14098026e: 83 f9 06                    	cmpl	$0x6, %ecx
140980271: 0f 94 c2                    	sete	%dl
140980274: 0f b6 46 04                 	movzbl	0x4(%rsi), %eax
140980278: 3c 02                       	cmpb	$0x2, %al
14098027a: 41 0f 94 c0                 	sete	%r8b
14098027e: 44 84 c2                    	testb	%r8b, %dl
140980281: 74 0b                       	je	0x14098028e <.text+0x97f28e>
140980283: f3 44 0f 10 25 48 b2 12 02  	movss	0x212b248(%rip), %xmm12 # 0x142aab4d4
14098028c: eb 1a                       	jmp	0x1409802a8 <.text+0x97f2a8>
14098028e: 83 f9 06                    	cmpl	$0x6, %ecx
140980291: 0f 85 83 06 00 00           	jne	0x14098091a <.text+0x97f91a>
140980297: 3c 01                       	cmpb	$0x1, %al
140980299: 0f 85 7b 06 00 00           	jne	0x14098091a <.text+0x97f91a>
14098029f: f3 44 0f 10 25 cc cd 04 02  	movss	0x204cdcc(%rip), %xmm12 # 0x1429cd074
1409802a8: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
1409802ac: 4c 89 e2                    	movq	%r12, %rdx
1409802af: e8 fc 1f 11 00              	callq	0x140a922b0 <.text+0xa912b0>
1409802b4: 48 89 c1                    	movq	%rax, %rcx
1409802b7: 48 c1 e9 20                 	shrq	$0x20, %rcx
1409802bb: 0f 84 04 07 00 00           	je	0x1409809c5 <.text+0x97f9c5>
1409802c1: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
1409802c8: 48 8b 11                    	movq	(%rcx), %rdx
1409802cb: 4c 8b 41 08                 	movq	0x8(%rcx), %r8
1409802cf: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1409802d3: 49 89 c1                    	movq	%rax, %r9
1409802d6: e8 c5 64 ff ff              	callq	0x1409767a0 <.text+0x9757a0>
1409802db: 83 7d 48 03                 	cmpl	$0x3, 0x48(%rbp)
1409802df: 0f 85 ec 06 00 00           	jne	0x1409809d1 <.text+0x97f9d1>
1409802e5: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
1409802e9: 44 89 75 70                 	movl	%r14d, 0x70(%rbp)
1409802ed: f3 44 0f 11 6d 74           	movss	%xmm13, 0x74(%rbp)
1409802f3: 89 5d 78                    	movl	%ebx, 0x78(%rbp)
1409802f6: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1409802fa: 4c 8d 45 70                 	leaq	0x70(%rbp), %r8
1409802fe: 66 0f 6e 1d 6a 55 fa 01     	movd	0x1fa556a(%rip), %xmm3  # 0x142925870
140980306: e8 a5 0a 00 00              	callq	0x140980db0 <.text+0x97fdb0>
14098030b: 8b 45 48                    	movl	0x48(%rbp), %eax
14098030e: 85 c0                       	testl	%eax, %eax
140980310: 0f 84 11 01 00 00           	je	0x140980427 <.text+0x97f427>
140980316: 48 8b 9d 28 02 00 00        	movq	0x228(%rbp), %rbx
14098031d: 83 f8 01                    	cmpl	$0x1, %eax
140980320: 0f 85 0c 01 00 00           	jne	0x140980432 <.text+0x97f432>
140980326: 49 89 fe                    	movq	%rdi, %r14
140980329: 8b 75 4c                    	movl	0x4c(%rbp), %esi
14098032c: f3 44 0f 10 75 50           	movss	0x50(%rbp), %xmm14
140980332: 8b 7d 54                    	movl	0x54(%rbp), %edi
140980335: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
14098033c: c7 85 80 00 00 00 9a 99 99 3e       	movl	$0x3e99999a, 0x80(%rbp) # imm = 0x3E99999A
140980346: 89 b5 84 00 00 00           	movl	%esi, 0x84(%rbp)
14098034c: f3 44 0f 11 b5 88 00 00 00  	movss	%xmm14, 0x88(%rbp)
140980355: 89 bd 8c 00 00 00           	movl	%edi, 0x8c(%rbp)
14098035b: c7 85 90 00 00 00 00 00 80 3f       	movl	$0x3f800000, 0x90(%rbp) # imm = 0x3F800000
140980365: c6 85 94 00 00 00 00        	movb	$0x0, 0x94(%rbp)
14098036c: 48 8b 1b                    	movq	(%rbx), %rbx
14098036f: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140980373: 48 89 da                    	movq	%rbx, %rdx
140980376: e8 a5 21 18 00              	callq	0x140b02520 <.text+0xb01520>
14098037b: e8 20 63 ff ff              	callq	0x1409766a0 <.text+0x9756a0>
140980380: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140980384: 89 75 38                    	movl	%esi, 0x38(%rbp)
140980387: 89 7d 3c                    	movl	%edi, 0x3c(%rbp)
14098038a: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140980391: f3 0f 10 90 80 02 00 00     	movss	0x280(%rax), %xmm2
140980399: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
14098039d: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
1409803a1: e8 3a ca 31 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1409803a6: 66 0f 6e fe                 	movd	%esi, %xmm7
1409803aa: 66 0f 6e f7                 	movd	%edi, %xmm6
1409803ae: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
1409803b2: 89 31                       	movl	%esi, (%rcx)
1409803b4: f3 44 0f 11 71 04           	movss	%xmm14, 0x4(%rcx)
1409803ba: 89 79 08                    	movl	%edi, 0x8(%rcx)
1409803bd: 8b 45 78                    	movl	0x78(%rbp), %eax
1409803c0: 89 41 14                    	movl	%eax, 0x14(%rcx)
1409803c3: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1409803c7: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
1409803cb: 48 c7 45 70 02 00 00 00     	movq	$0x2, 0x70(%rbp)
1409803d3: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
1409803d7: 48 c7 85 80 00 00 00 02 00 00 00    	movq	$0x2, 0x80(%rbp)
1409803e2: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1409803ec: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
1409803f3: c7 85 a0 00 00 00 00 00 00 00       	movl	$0x0, 0xa0(%rbp)
1409803fd: c7 85 b0 00 00 00 00 00 40 40       	movl	$0x40400000, 0xb0(%rbp) # imm = 0x40400000
140980407: 66 c7 85 b4 00 00 00 01 01  	movw	$0x101, 0xb4(%rbp)      # imm = 0x101
140980410: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140980414: 48 89 da                    	movq	%rbx, %rdx
140980417: e8 54 4b 18 00              	callq	0x140b04f70 <.text+0xb03f70>
14098041c: 40 b6 02                    	movb	$0x2, %sil
14098041f: 4c 89 f7                    	movq	%r14, %rdi
140980422: e9 db 01 00 00              	jmp	0x140980602 <.text+0x97f602>
140980427: 31 f6                       	xorl	%esi, %esi
140980429: 45 0f 28 f5                 	movaps	%xmm13, %xmm14
14098042d: e9 d0 01 00 00              	jmp	0x140980602 <.text+0x97f602>
140980432: f3 44 0f 11 65 04           	movss	%xmm12, 0x4(%rbp)
140980438: 44 0f 29 55 c0              	movaps	%xmm10, -0x40(%rbp)
14098043d: f3 44 0f 11 4d 00           	movss	%xmm9, (%rbp)
140980443: f3 0f 10 7d 4c              	movss	0x4c(%rbp), %xmm7
140980448: f3 44 0f 10 7d 58           	movss	0x58(%rbp), %xmm15
14098044e: f3 44 0f 10 75 64           	movss	0x64(%rbp), %xmm14
140980454: e8 47 62 ff ff              	callq	0x1409766a0 <.text+0x9756a0>
140980459: 48 c7 45 70 02 00 00 00     	movq	$0x2, 0x70(%rbp)
140980461: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
140980465: 48 c7 85 80 00 00 00 02 00 00 00    	movq	$0x2, 0x80(%rbp)
140980470: 48 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rsi # imm = 0x8000000000000000
14098047a: 48 89 b5 88 00 00 00        	movq	%rsi, 0x88(%rbp)
140980481: c7 85 a0 00 00 00 00 00 00 00       	movl	$0x0, 0xa0(%rbp)
14098048b: c7 85 b0 00 00 00 00 00 40 40       	movl	$0x40400000, 0xb0(%rbp) # imm = 0x40400000
140980495: 66 c7 85 b4 00 00 00 01 01  	movw	$0x101, 0xb4(%rbp)      # imm = 0x101
14098049e: 48 8b 1b                    	movq	(%rbx), %rbx
1409804a1: 45 0f 28 d0                 	movaps	%xmm8, %xmm10
1409804a5: f3 44 0f 5c d7              	subss	%xmm7, %xmm10
1409804aa: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
1409804ae: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
1409804b3: 41 0f 28 f6                 	movaps	%xmm14, %xmm6
1409804b7: f3 0f 59 f7                 	mulss	%xmm7, %xmm6
1409804bb: f3 0f 58 f0                 	addss	%xmm0, %xmm6
1409804bf: f2 44 0f 10 65 50           	movsd	0x50(%rbp), %xmm12
1409804c5: f2 44 0f 10 4d 5c           	movsd	0x5c(%rbp), %xmm9
1409804cb: 45 0f 16 f9                 	movlhps	%xmm9, %xmm15           # xmm15 = xmm15[0],xmm9[0]
1409804cf: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1409804d3: 41 0f c6 c7 84              	shufps	$0x84, %xmm15, %xmm0    # xmm0 = xmm0[0,1],xmm15[0,2]
1409804d8: 0f 58 05 11 9a 10 02        	addps	0x2109a11(%rip), %xmm0  # 0x142a89ef0
1409804df: 0f 11 00                    	movups	%xmm0, (%rax)
1409804e2: 45 0f c6 f1 d4              	shufps	$0xd4, %xmm9, %xmm14    # xmm14 = xmm14[0,1],xmm9[1,3]
1409804e7: 45 0f c6 f1 f2              	shufps	$0xf2, %xmm9, %xmm14    # xmm14 = xmm14[2,0],xmm9[3,3]
1409804ec: f3 0f 10 05 74 cb 04 02     	movss	0x204cb74(%rip), %xmm0  # 0x1429cd068
1409804f4: 41 0f 58 c6                 	addps	%xmm14, %xmm0
1409804f8: 0f 13 40 10                 	movlps	%xmm0, 0x10(%rax)
1409804fc: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140980500: 48 89 da                    	movq	%rbx, %rdx
140980503: e8 68 4a 18 00              	callq	0x140b04f70 <.text+0xb03f70>
140980508: f3 41 0f 12 c2              	movsldup	%xmm10, %xmm0           # xmm0 = xmm10[0,0,2,2]
14098050d: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
140980511: f3 0f 12 ff                 	movsldup	%xmm7, %xmm7            # xmm7 = xmm7[0,0,2,2]
140980515: 41 0f 59 f9                 	mulps	%xmm9, %xmm7
140980519: 0f 58 f8                    	addps	%xmm0, %xmm7
14098051c: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
140980523: c7 85 80 00 00 00 9a 99 99 3e       	movl	$0x3e99999a, 0x80(%rbp) # imm = 0x3E99999A
14098052d: 0f 13 bd 84 00 00 00        	movlps	%xmm7, 0x84(%rbp)
140980534: f3 0f 11 b5 8c 00 00 00     	movss	%xmm6, 0x8c(%rbp)
14098053c: c7 85 90 00 00 00 00 00 80 3f       	movl	$0x3f800000, 0x90(%rbp) # imm = 0x3F800000
140980546: c6 85 94 00 00 00 00        	movb	$0x0, 0x94(%rbp)
14098054d: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140980551: 48 89 da                    	movq	%rbx, %rdx
140980554: e8 c7 1f 18 00              	callq	0x140b02520 <.text+0xb01520>
140980559: e8 42 61 ff ff              	callq	0x1409766a0 <.text+0x9756a0>
14098055e: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140980562: f3 0f 11 7d 38              	movss	%xmm7, 0x38(%rbp)
140980567: f3 0f 11 75 3c              	movss	%xmm6, 0x3c(%rbp)
14098056c: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140980573: f3 0f 10 90 80 02 00 00     	movss	0x280(%rax), %xmm2
14098057b: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
14098057f: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
140980583: e8 58 c8 31 00              	callq	0x140c9cde0 <.text+0xc9bde0>
140980588: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
14098058c: 0f 13 39                    	movlps	%xmm7, (%rcx)
14098058f: f3 0f 11 71 08              	movss	%xmm6, 0x8(%rcx)
140980594: 8b 45 78                    	movl	0x78(%rbp), %eax
140980597: 89 41 14                    	movl	%eax, 0x14(%rcx)
14098059a: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14098059e: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
1409805a2: 48 c7 45 70 02 00 00 00     	movq	$0x2, 0x70(%rbp)
1409805aa: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
1409805ae: 48 c7 85 80 00 00 00 02 00 00 00    	movq	$0x2, 0x80(%rbp)
1409805b9: 48 89 b5 88 00 00 00        	movq	%rsi, 0x88(%rbp)
1409805c0: c7 85 a0 00 00 00 00 00 00 00       	movl	$0x0, 0xa0(%rbp)
1409805ca: c7 85 b0 00 00 00 00 00 40 40       	movl	$0x40400000, 0xb0(%rbp) # imm = 0x40400000
1409805d4: 66 c7 85 b4 00 00 00 01 01  	movw	$0x101, 0xb4(%rbp)      # imm = 0x101
1409805dd: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1409805e1: 48 89 da                    	movq	%rbx, %rdx
1409805e4: e8 87 49 18 00              	callq	0x140b04f70 <.text+0xb03f70>
1409805e9: f3 44 0f 16 f7              	movshdup	%xmm7, %xmm14           # xmm14 = xmm7[1,1,3,3]
1409805ee: 40 b6 02                    	movb	$0x2, %sil
1409805f1: f3 44 0f 10 4d 00           	movss	(%rbp), %xmm9
1409805f7: 44 0f 28 55 c0              	movaps	-0x40(%rbp), %xmm10
1409805fc: f3 44 0f 10 65 04           	movss	0x4(%rbp), %xmm12
140980602: f3 45 0f 5f d9              	maxss	%xmm9, %xmm11
140980607: f3 45 0f 58 ec              	addss	%xmm12, %xmm13
14098060c: f3 44 0f 58 2d 4f ca 04 02  	addss	0x204ca4f(%rip), %xmm13 # 0x1429cd064
140980615: f3 45 0f 5d c3              	minss	%xmm11, %xmm8
14098061a: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140980621: 66 0f 6e 80 80 02 00 00     	movd	0x280(%rax), %xmm0
140980629: 8b 5f 1c                    	movl	0x1c(%rdi), %ebx
14098062c: 4c 8b 77 10                 	movq	0x10(%rdi), %r14
140980630: 41 89 1e                    	movl	%ebx, (%r14)
140980633: 48 8b 3f                    	movq	(%rdi), %rdi
140980636: c7 07 01 00 00 00           	movl	$0x1, (%rdi)
14098063c: f3 44 0f 11 6f 04           	movss	%xmm13, 0x4(%rdi)
140980642: c7 47 08 02 00 00 00        	movl	$0x2, 0x8(%rdi)
140980649: f3 0f 11 7f 0c              	movss	%xmm7, 0xc(%rdi)
14098064e: f3 44 0f 11 77 10           	movss	%xmm14, 0x10(%rdi)
140980654: f3 0f 11 77 14              	movss	%xmm6, 0x14(%rdi)
140980659: 44 0f 13 57 18              	movlps	%xmm10, 0x18(%rdi)
14098065e: 4c 89 67 20                 	movq	%r12, 0x20(%rdi)
140980662: f3 44 0f 11 47 28           	movss	%xmm8, 0x28(%rdi)
140980668: 0f b6 85 bf 00 00 00        	movzbl	0xbf(%rbp), %eax
14098066f: 88 47 2c                    	movb	%al, 0x2c(%rdi)
140980672: f3 0f 11 7f 30              	movss	%xmm7, 0x30(%rdi)
140980677: f3 44 0f 11 77 34           	movss	%xmm14, 0x34(%rdi)
14098067d: f3 0f 11 77 38              	movss	%xmm6, 0x38(%rdi)
140980682: 66 0f 7e 47 3c              	movd	%xmm0, 0x3c(%rdi)
140980687: c7 47 40 00 00 00 00        	movl	$0x0, 0x40(%rdi)
14098068e: f3 0f 11 7f 44              	movss	%xmm7, 0x44(%rdi)
140980693: f3 44 0f 11 77 48           	movss	%xmm14, 0x48(%rdi)
140980699: f3 0f 11 77 4c              	movss	%xmm6, 0x4c(%rdi)
14098069e: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
1409806a5: 49 8b 0f                    	movq	(%r15), %rcx
1409806a8: 80 39 00                    	cmpb	$0x0, (%rcx)
1409806ab: 0f 85 7f f8 ff ff           	jne	0x14097ff30 <.text+0x97ef30>
1409806b1: 8b 48 1c                    	movl	0x1c(%rax), %ecx
1409806b4: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1409806b8: 89 0a                       	movl	%ecx, (%rdx)
1409806ba: 48 8b 18                    	movq	(%rax), %rbx
1409806bd: 4c 8b 73 30                 	movq	0x30(%rbx), %r14
1409806c1: 4c 8b 7b 40                 	movq	0x40(%rbx), %r15
1409806c5: 4c 3b 73 20                 	cmpq	0x20(%rbx), %r14
1409806c9: 75 10                       	jne	0x1409806db <.text+0x97f6db>
1409806cb: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
1409806cf: 48 8d 15 72 9a 2b 02        	leaq	0x22b9a72(%rip), %rdx   # 0x142c3a148
1409806d6: e8 55 1f be ff              	callq	0x140562630 <.text+0x561630>
1409806db: 48 8b 43 28                 	movq	0x28(%rbx), %rax
1409806df: 4c 89 f1                    	movq	%r14, %rcx
1409806e2: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409806e6: 4c 89 3c 08                 	movq	%r15, (%rax,%rcx)
1409806ea: c6 44 08 08 00              	movb	$0x0, 0x8(%rax,%rcx)
1409806ef: 49 ff c6                    	incq	%r14
1409806f2: 4c 89 73 30                 	movq	%r14, 0x30(%rbx)
1409806f6: 48 ff 43 40                 	incq	0x40(%rbx)
1409806fa: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
140980701: 48 8b 19                    	movq	(%rcx), %rbx
140980704: 8d 46 ff                    	leal	-0x1(%rsi), %eax
140980707: 3c 01                       	cmpb	$0x1, %al
140980709: 77 7a                       	ja	0x140980785 <.text+0x97f785>
14098070b: 80 3b 00                    	cmpb	$0x0, (%rbx)
14098070e: 75 75                       	jne	0x140980785 <.text+0x97f785>
140980710: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
140980717: 8b 4f 38                    	movl	0x38(%rdi), %ecx
14098071a: 89 4d 7f                    	movl	%ecx, 0x7f(%rbp)
14098071d: 48 8b 4f 30                 	movq	0x30(%rdi), %rcx
140980721: 48 89 4d 77                 	movq	%rcx, 0x77(%rbp)
140980725: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140980728: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14098072c: 89 0a                       	movl	%ecx, (%rdx)
14098072e: 48 8b 38                    	movq	(%rax), %rdi
140980731: 4c 8b 77 30                 	movq	0x30(%rdi), %r14
140980735: 4c 8b 7f 40                 	movq	0x40(%rdi), %r15
140980739: 4c 3b 77 20                 	cmpq	0x20(%rdi), %r14
14098073d: 75 10                       	jne	0x14098074f <.text+0x97f74f>
14098073f: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
140980743: 48 8d 15 fe 99 2b 02        	leaq	0x22b99fe(%rip), %rdx   # 0x142c3a148
14098074a: e8 e1 24 be ff              	callq	0x140562c30 <.text+0x561c30>
14098074f: 48 8b 47 28                 	movq	0x28(%rdi), %rax
140980753: 4c 89 f1                    	movq	%r14, %rcx
140980756: 48 c1 e1 05                 	shlq	$0x5, %rcx
14098075a: 4c 89 3c 08                 	movq	%r15, (%rax,%rcx)
14098075e: c6 44 08 08 03              	movb	$0x3, 0x8(%rax,%rcx)
140980763: 8b 55 7f                    	movl	0x7f(%rbp), %edx
140980766: 89 54 08 18                 	movl	%edx, 0x18(%rax,%rcx)
14098076a: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14098076e: 0f 11 44 08 09              	movups	%xmm0, 0x9(%rax,%rcx)
140980773: 49 ff c6                    	incq	%r14
140980776: 4c 89 77 30                 	movq	%r14, 0x30(%rdi)
14098077a: 48 ff 47 40                 	incq	0x40(%rdi)
14098077e: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
140980785: 8b 41 1c                    	movl	0x1c(%rcx), %eax
140980788: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
14098078c: 89 01                       	movl	%eax, (%rcx)
14098078e: 40 88 33                    	movb	%sil, (%rbx)
140980791: 0f 28 b5 d0 00 00 00        	movaps	0xd0(%rbp), %xmm6
140980798: 0f 28 bd e0 00 00 00        	movaps	0xe0(%rbp), %xmm7
14098079f: 44 0f 28 85 f0 00 00 00     	movaps	0xf0(%rbp), %xmm8
1409807a7: 44 0f 28 8d 00 01 00 00     	movaps	0x100(%rbp), %xmm9
1409807af: 44 0f 28 95 10 01 00 00     	movaps	0x110(%rbp), %xmm10
1409807b7: 44 0f 28 9d 20 01 00 00     	movaps	0x120(%rbp), %xmm11
1409807bf: 44 0f 28 a5 30 01 00 00     	movaps	0x130(%rbp), %xmm12
1409807c7: 44 0f 28 ad 40 01 00 00     	movaps	0x140(%rbp), %xmm13
1409807cf: 44 0f 28 b5 50 01 00 00     	movaps	0x150(%rbp), %xmm14
1409807d7: 44 0f 28 bd 60 01 00 00     	movaps	0x160(%rbp), %xmm15
1409807df: 48 81 c4 f8 01 00 00        	addq	$0x1f8, %rsp            # imm = 0x1F8
1409807e6: 5b                          	popq	%rbx
1409807e7: 5f                          	popq	%rdi
1409807e8: 5e                          	popq	%rsi
1409807e9: 41 5c                       	popq	%r12
1409807eb: 41 5d                       	popq	%r13
1409807ed: 41 5e                       	popq	%r14
1409807ef: 41 5f                       	popq	%r15
1409807f1: 5d                          	popq	%rbp
1409807f2: c3                          	retq
1409807f3: 0f 10 03                    	movups	(%rbx), %xmm0
1409807f6: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
1409807fa: f3 0f 10 45 78              	movss	0x78(%rbp), %xmm0
1409807ff: f3 0f 58 05 09 5e 13 02     	addss	0x2135e09(%rip), %xmm0  # 0x142ab6610
140980807: f3 0f 11 45 78              	movss	%xmm0, 0x78(%rbp)
14098080c: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140980810: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
140980814: e8 87 6d 31 00              	callq	0x140c975a0 <.text+0xc965a0>
140980819: f3 0f 11 45 48              	movss	%xmm0, 0x48(%rbp)
14098081e: f3 0f 11 4d 4c              	movss	%xmm1, 0x4c(%rbp)
140980823: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140980827: 48 8d 55 48                 	leaq	0x48(%rbp), %rdx
14098082b: 41 0f 28 d3                 	movaps	%xmm11, %xmm2
14098082f: e8 ac c5 31 00              	callq	0x140c9cde0 <.text+0xc9bde0>
140980834: 66 0f 6e 7d 70              	movd	0x70(%rbp), %xmm7
140980839: f3 44 0f 10 5d 74           	movss	0x74(%rbp), %xmm11
14098083f: 66 0f 6e 75 78              	movd	0x78(%rbp), %xmm6
140980844: 48 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %rsi
14098084b: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140980852: 4c 89 e2                    	movq	%r12, %rdx
140980855: e8 56 1a 11 00              	callq	0x140a922b0 <.text+0xa912b0>
14098085a: 48 8b 16                    	movq	(%rsi), %rdx
14098085d: 4c 8b 46 08                 	movq	0x8(%rsi), %r8
140980861: 48 89 c1                    	movq	%rax, %rcx
140980864: e8 87 5d ff ff              	callq	0x1409765f0 <.text+0x9755f0>
140980869: 48 85 c0                    	testq	%rax, %rax
14098086c: 74 11                       	je	0x14098087f <.text+0x97f87f>
14098086e: f3 0f 10 05 0e 51 fa 01     	movss	0x1fa510e(%rip), %xmm0  # 0x142925984
140980876: 0f 2e 40 30                 	ucomiss	0x30(%rax), %xmm0
14098087a: 0f 93 c0                    	setae	%al
14098087d: eb 02                       	jmp	0x140980881 <.text+0x97f881>
14098087f: 31 c0                       	xorl	%eax, %eax
140980881: 8b 5f 1c                    	movl	0x1c(%rdi), %ebx
140980884: 4c 8b 77 10                 	movq	0x10(%rdi), %r14
140980888: 41 89 1e                    	movl	%ebx, (%r14)
14098088b: 48 8b 3f                    	movq	(%rdi), %rdi
14098088e: c7 07 00 00 00 00           	movl	$0x0, (%rdi)
140980894: c7 47 08 01 00 00 00        	movl	$0x1, 0x8(%rdi)
14098089b: f3 44 0f 11 4f 18           	movss	%xmm9, 0x18(%rdi)
1409808a1: f3 44 0f 11 57 1c           	movss	%xmm10, 0x1c(%rdi)
1409808a7: 4c 89 67 20                 	movq	%r12, 0x20(%rdi)
1409808ab: f3 44 0f 11 47 28           	movss	%xmm8, 0x28(%rdi)
1409808b1: 88 47 2c                    	movb	%al, 0x2c(%rdi)
1409808b4: 66 0f 7e 7f 30              	movd	%xmm7, 0x30(%rdi)
1409808b9: f3 44 0f 11 5f 34           	movss	%xmm11, 0x34(%rdi)
1409808bf: 66 0f 7e 77 38              	movd	%xmm6, 0x38(%rdi)
1409808c4: f3 44 0f 11 6f 3c           	movss	%xmm13, 0x3c(%rdi)
1409808ca: c7 47 40 00 00 00 00        	movl	$0x0, 0x40(%rdi)
1409808d1: 48 8b 45 d4                 	movq	-0x2c(%rbp), %rax
1409808d5: 48 89 47 44                 	movq	%rax, 0x44(%rdi)
1409808d9: 8b 45 dc                    	movl	-0x24(%rbp), %eax
1409808dc: e9 36 f6 ff ff              	jmp	0x14097ff17 <.text+0x97ef17>
1409808e1: 48 83 fe 15                 	cmpq	$0x15, %rsi
1409808e5: 73 4b                       	jae	0x140980932 <.text+0x97f932>
1409808e7: 48 c1 e6 05                 	shlq	$0x5, %rsi
1409808eb: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
1409808f2: 48 01 c6                    	addq	%rax, %rsi
1409808f5: 48 8d 78 20                 	leaq	0x20(%rax), %rdi
1409808f9: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140980900: 48 89 fa                    	movq	%rdi, %rdx
140980903: 4c 8d 45 08                 	leaq	0x8(%rbp), %r8
140980907: e8 14 19 9b 00              	callq	0x141332220 <.text+0x1331220>
14098090c: 48 83 c7 20                 	addq	$0x20, %rdi
140980910: 48 39 f7                    	cmpq	%rsi, %rdi
140980913: 75 e4                       	jne	0x1409808f9 <.text+0x97f8f9>
140980915: e9 fd f2 ff ff              	jmp	0x14097fc17 <.text+0x97ec17>
14098091a: 48 8d 0d 20 5f 13 02        	leaq	0x2135f20(%rip), %rcx   # 0x142ab6841
140980921: 4c 8d 05 a8 66 13 02        	leaq	0x21366a8(%rip), %r8    # 0x142ab6fd0
140980928: ba 28 00 00 00              	movl	$0x28, %edx
14098092d: e8 fe 8a f5 01              	callq	0x1428d9430 <.text+0x28d8430>
140980932: 4c 8d 45 70                 	leaq	0x70(%rbp), %r8
140980936: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
14098093d: 48 89 f2                    	movq	%rsi, %rdx
140980940: e8 eb ad 1c 00              	callq	0x140b4b730 <.text+0xb4a730>
140980945: e9 cd f2 ff ff              	jmp	0x14097fc17 <.text+0x97ec17>
14098094a: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
14098094e: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140980952: 48 8d 05 47 0d 00 00        	leaq	0xd47(%rip), %rax       # 0x1409816a0 <.text+0x9806a0>
140980959: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14098095d: 48 8d 05 64 67 13 02        	leaq	0x2136764(%rip), %rax   # 0x142ab70c8
140980964: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140980968: 48 c7 45 78 02 00 00 00     	movq	$0x2, 0x78(%rbp)
140980970: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
14098097b: 48 8d 45 48                 	leaq	0x48(%rbp), %rax
14098097f: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140980986: 48 c7 85 88 00 00 00 01 00 00 00    	movq	$0x1, 0x88(%rbp)
140980991: 48 8d 15 50 67 13 02        	leaq	0x2136750(%rip), %rdx   # 0x142ab70e8
140980998: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
14098099c: e8 ef 89 f5 01              	callq	0x1428d9390 <.text+0x28d8390>
1409809a1: 48 8d 0d 58 66 13 02        	leaq	0x2136658(%rip), %rcx   # 0x142ab7000
1409809a8: e8 63 89 f5 01              	callq	0x1428d9310 <.text+0x28d8310>
1409809ad: 48 8d 0d a4 65 13 02        	leaq	0x21365a4(%rip), %rcx   # 0x142ab6f58
1409809b4: e8 57 89 f5 01              	callq	0x1428d9310 <.text+0x28d8310>
1409809b9: 48 8d 0d 30 a4 16 02        	leaq	0x216a430(%rip), %rcx   # 0x142aeadf0
1409809c0: e8 4b 89 f5 01              	callq	0x1428d9310 <.text+0x28d8310>
1409809c5: 48 8d 0d d4 65 13 02        	leaq	0x21365d4(%rip), %rcx   # 0x142ab6fa0
1409809cc: e8 3f 89 f5 01              	callq	0x1428d9310 <.text+0x28d8310>
1409809d1: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
1409809d5: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
1409809d9: 48 8d 05 d8 65 13 02        	leaq	0x21365d8(%rip), %rax   # 0x142ab6fb8
1409809e0: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409809e5: 48 8d 0d 04 5d 13 02        	leaq	0x2135d04(%rip), %rcx   # 0x142ab66f0
1409809ec: 4c 8d 0d dd 5c 13 02        	leaq	0x2135cdd(%rip), %r9    # 0x142ab66d0
1409809f3: 4c 8d 45 70                 	leaq	0x70(%rbp), %r8
1409809f7: ba 2b 00 00 00              	movl	$0x2b, %edx
1409809fc: e8 5f 8d f5 01              	callq	0x1428d9760 <.text+0x28d8760>
140980a01: cc                          	int3
140980a02: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140980a10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140980a15: 55                          	pushq	%rbp
140980a16: 41 57                       	pushq	%r15
140980a18: 41 56                       	pushq	%r14
140980a1a: 41 55                       	pushq	%r13
140980a1c: 41 54                       	pushq	%r12
140980a1e: 56                          	pushq	%rsi
140980a1f: 57                          	pushq	%rdi
140980a20: 53                          	pushq	%rbx
140980a21: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140980a28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140980a2f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140980a35: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140980a3b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140980a41: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140980a47: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140980a4d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140980a56: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140980a5f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140980a68: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140980a70: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140980a78: ba 18 00 00 00              	movl	$0x18, %edx
140980a7d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140980a83: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140980a87: e8 94 31 c9 ff              	callq	0x140613c20 <.text+0x612c20>
140980a8c: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140980a94: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140980a9c: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140980aa5: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140980aae: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140980ab7: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140980abd: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140980ac3: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140980ac9: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140980acf: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140980ad5: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140980adc: 5b                          	popq	%rbx
140980add: 5f                          	popq	%rdi
140980ade: 5e                          	popq	%rsi
140980adf: 41 5c                       	popq	%r12
140980ae1: 41 5d                       	popq	%r13
140980ae3: 41 5e                       	popq	%r14
140980ae5: 41 5f                       	popq	%r15
140980ae7: 5d                          	popq	%rbp
140980ae8: c3                          	retq
140980ae9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140980af0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140980af5: 55                          	pushq	%rbp
140980af6: 41 57                       	pushq	%r15
140980af8: 41 56                       	pushq	%r14
140980afa: 41 55                       	pushq	%r13
140980afc: 41 54                       	pushq	%r12
140980afe: 56                          	pushq	%rsi
140980aff: 57                          	pushq	%rdi
140980b00: 53                          	pushq	%rbx
140980b01: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140980b08: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140980b0f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140980b15: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140980b1b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140980b21: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140980b27: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140980b2d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140980b36: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140980b3f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140980b48: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140980b51: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
140980b5a: ba 18 00 00 00              	movl	$0x18, %edx
140980b5f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140980b65: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140980b69: e8 b2 30 c9 ff              	callq	0x140613c20 <.text+0x612c20>
140980b6e: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140980b76: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140980b7e: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140980b87: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140980b90: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140980b99: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140980b9f: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140980ba5: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140980bab: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140980bb1: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140980bb7: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140980bbe: 5b                          	popq	%rbx
140980bbf: 5f                          	popq	%rdi
140980bc0: 5e                          	popq	%rsi
140980bc1: 41 5c                       	popq	%r12
140980bc3: 41 5d                       	popq	%r13
140980bc5: 41 5e                       	popq	%r14
140980bc7: 41 5f                       	popq	%r15
140980bc9: 5d                          	popq	%rbp
140980bca: c3                          	retq
140980bcb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140980bd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140980bd5: 55                          	pushq	%rbp
140980bd6: 41 57                       	pushq	%r15
140980bd8: 41 56                       	pushq	%r14
140980bda: 41 55                       	pushq	%r13
140980bdc: 41 54                       	pushq	%r12
140980bde: 56                          	pushq	%rsi
140980bdf: 57                          	pushq	%rdi
140980be0: 53                          	pushq	%rbx
140980be1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140980be8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140980bef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140980bf5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140980bfb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140980c01: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140980c07: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140980c0d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140980c16: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140980c1f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140980c28: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140980c31: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
140980c3a: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
140980c3e: 48 85 d2                    	testq	%rdx, %rdx
140980c41: 74 16                       	je	0x140980c59 <.text+0x97fc59>
140980c43: 48 c1 e2 05                 	shlq	$0x5, %rdx
140980c47: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140980c4d: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140980c54: e8 c7 2f c9 ff              	callq	0x140613c20 <.text+0x612c20>
140980c59: 66 0f 6f b4 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm6
140980c62: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
140980c6b: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140980c74: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140980c7d: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140980c86: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140980c8c: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140980c92: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140980c98: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140980c9e: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140980ca4: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140980cab: 5b                          	popq	%rbx
140980cac: 5f                          	popq	%rdi
140980cad: 5e                          	popq	%rsi
140980cae: 41 5c                       	popq	%r12
140980cb0: 41 5d                       	popq	%r13
140980cb2: 41 5e                       	popq	%r14
140980cb4: 41 5f                       	popq	%r15
140980cb6: 5d                          	popq	%rbp
140980cb7: c3                          	retq
140980cb8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140980cc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140980cc5: 55                          	pushq	%rbp
140980cc6: 41 57                       	pushq	%r15
140980cc8: 41 56                       	pushq	%r14
140980cca: 41 55                       	pushq	%r13
140980ccc: 41 54                       	pushq	%r12
140980cce: 56                          	pushq	%rsi
140980ccf: 57                          	pushq	%rdi
140980cd0: 53                          	pushq	%rbx
140980cd1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
140980cd8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140980cdf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
140980ce5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
140980ceb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
140980cf1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
140980cf7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
140980cfd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
140980d06: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
140980d0f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
140980d18: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
140980d21: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
140980d2a: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
140980d2e: 48 85 d2                    	testq	%rdx, %rdx
140980d31: 74 13                       	je	0x140980d46 <.text+0x97fd46>
140980d33: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
140980d37: 48 c1 e2 03                 	shlq	$0x3, %rdx
140980d3b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140980d41: e8 da 2e c9 ff              	callq	0x140613c20 <.text+0x612c20>
140980d46: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
140980d4e: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140980d56: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
140980d5f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
140980d68: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
140980d71: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
140980d77: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
140980d7d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
140980d83: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
140980d89: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
140980d8f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
140980d96: 5b                          	popq	%rbx
140980d97: 5f                          	popq	%rdi
140980d98: 5e                          	popq	%rsi
140980d99: 41 5c                       	popq	%r12
140980d9b: 41 5d                       	popq	%r13
140980d9d: 41 5e                       	popq	%r14
140980d9f: 41 5f                       	popq	%r15
140980da1: 5d                          	popq	%rbp
140980da2: c3                          	retq
140980da3: cc                          	int3
140980da4: cc                          	int3
140980da5: cc                          	int3
140980da6: cc                          	int3
140980da7: cc                          	int3
140980da8: cc                          	int3
140980da9: cc                          	int3
140980daa: cc                          	int3
140980dab: cc                          	int3
140980dac: cc                          	int3
140980dad: cc                          	int3
140980dae: cc                          	int3
140980daf: cc                          	int3
