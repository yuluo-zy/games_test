
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416f6e90: 55                          	pushq	%rbp
1416f6e91: 41 57                       	pushq	%r15
1416f6e93: 41 56                       	pushq	%r14
1416f6e95: 41 55                       	pushq	%r13
1416f6e97: 41 54                       	pushq	%r12
1416f6e99: 56                          	pushq	%rsi
1416f6e9a: 57                          	pushq	%rdi
1416f6e9b: 53                          	pushq	%rbx
1416f6e9c: 48 81 ec 18 03 00 00        	subq	$0x318, %rsp            # imm = 0x318
1416f6ea3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1416f6eab: 0f 29 b5 80 02 00 00        	movaps	%xmm6, 0x280(%rbp)
1416f6eb2: 48 c7 85 78 02 00 00 fe ff ff ff    	movq	$-0x2, 0x278(%rbp)
1416f6ebd: 4d 89 cd                    	movq	%r9, %r13
1416f6ec0: 4c 89 c0                    	movq	%r8, %rax
1416f6ec3: 48 89 d3                    	movq	%rdx, %rbx
1416f6ec6: 48 89 ce                    	movq	%rcx, %rsi
1416f6ec9: 4c 8b 02                    	movq	(%rdx), %r8
1416f6ecc: 48 8b 7a 08                 	movq	0x8(%rdx), %rdi
1416f6ed0: 48 8b 8f 48 06 00 00        	movq	0x648(%rdi), %rcx
1416f6ed7: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
1416f6ede: 48 89 c2                    	movq	%rax, %rdx
1416f6ee1: 4c 89 85 50 01 00 00        	movq	%r8, 0x150(%rbp)
1416f6ee8: e8 83 80 1b 00              	callq	0x1418aef70 <.text+0x18adf70>
1416f6eed: 48 85 c0                    	testq	%rax, %rax
1416f6ef0: 0f 84 63 08 00 00           	je	0x1416f7759 <.text+0x16f6759>
1416f6ef6: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1416f6efd: 4c 8b 87 48 06 00 00        	movq	0x648(%rdi), %r8
1416f6f04: 4c 8b 63 10                 	movq	0x10(%rbx), %r12
1416f6f08: 44 8b 11                    	movl	(%rcx), %r10d
1416f6f0b: 41 83 fa 02                 	cmpl	$0x2, %r10d
1416f6f0f: 48 89 b5 70 02 00 00        	movq	%rsi, 0x270(%rbp)
1416f6f16: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1416f6f1d: 44 89 95 58 02 00 00        	movl	%r10d, 0x258(%rbp)
1416f6f24: 75 08                       	jne	0x1416f6f2e <.text+0x16f5f2e>
1416f6f26: 45 31 c9                    	xorl	%r9d, %r9d
1416f6f29: e9 83 00 00 00              	jmp	0x1416f6fb1 <.text+0x16f5fb1>
1416f6f2e: 83 79 18 01                 	cmpl	$0x1, 0x18(%rcx)
1416f6f32: 75 0a                       	jne	0x1416f6f3e <.text+0x16f5f3e>
1416f6f34: f6 41 1c 06                 	testb	$0x6, 0x1c(%rcx)
1416f6f38: 0f 84 e9 06 00 00           	je	0x1416f7627 <.text+0x16f6627>
1416f6f3e: 8b 49 44                    	movl	0x44(%rcx), %ecx
1416f6f41: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
1416f6f46: 48 39 ca                    	cmpq	%rcx, %rdx
1416f6f49: 0f 86 3d 08 00 00           	jbe	0x1416f778c <.text+0x16f678c>
1416f6f4f: 4d 8b 4c 24 08              	movq	0x8(%r12), %r9
1416f6f54: 49 8b 54 24 40              	movq	0x40(%r12), %rdx
1416f6f59: 41 0f b7 0c 89              	movzwl	(%r9,%rcx,4), %ecx
1416f6f5e: 48 39 ca                    	cmpq	%rcx, %rdx
1416f6f61: 0f 86 31 08 00 00           	jbe	0x1416f7798 <.text+0x16f6798>
1416f6f67: 49 8b 54 24 38              	movq	0x38(%r12), %rdx
1416f6f6c: c1 e1 06                    	shll	$0x6, %ecx
1416f6f6f: 4c 8b 0c 0a                 	movq	(%rdx,%rcx), %r9
1416f6f73: 49 83 f9 02                 	cmpq	$0x2, %r9
1416f6f77: 0f 83 e8 07 00 00           	jae	0x1416f7765 <.text+0x16f6765>
1416f6f7d: 48 01 ca                    	addq	%rcx, %rdx
1416f6f80: 0f b7 4a 1c                 	movzwl	0x1c(%rdx), %ecx
1416f6f84: 66 41 3b 88 50 0a 00 00     	cmpw	0xa50(%r8), %cx
1416f6f8c: 0f 85 41 06 00 00           	jne	0x1416f75d3 <.text+0x16f65d3>
1416f6f92: 80 7a 1e 00                 	cmpb	$0x0, 0x1e(%rdx)
1416f6f96: 0f 85 61 06 00 00           	jne	0x1416f75fd <.text+0x16f65fd>
1416f6f9c: 4c 8b 4a 10                 	movq	0x10(%rdx), %r9
1416f6fa0: 41 8b 49 0c                 	movl	0xc(%r9), %ecx
1416f6fa4: 3b 4a 18                    	cmpl	0x18(%rdx), %ecx
1416f6fa7: 0f 85 f7 07 00 00           	jne	0x1416f77a4 <.text+0x16f67a4>
1416f6fad: 49 83 c1 10                 	addq	$0x10, %r9
1416f6fb1: 48 89 bd 60 02 00 00        	movq	%rdi, 0x260(%rbp)
1416f6fb8: 48 8b b5 00 03 00 00        	movq	0x300(%rbp), %rsi
1416f6fbf: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
1416f6fc3: 48 c1 e1 05                 	shlq	$0x5, %rcx
1416f6fc7: 4c 01 e9                    	addq	%r13, %rcx
1416f6fca: c7 85 80 01 00 00 00 00 00 00       	movl	$0x0, 0x180(%rbp)
1416f6fd4: 48 c7 85 e0 01 00 00 01 00 00 00    	movq	$0x1, 0x1e0(%rbp)
1416f6fdf: 4c 89 8d e8 01 00 00        	movq	%r9, 0x1e8(%rbp)
1416f6fe6: 4c 89 ad f0 01 00 00        	movq	%r13, 0x1f0(%rbp)
1416f6fed: 48 89 8d f8 01 00 00        	movq	%rcx, 0x1f8(%rbp)
1416f6ff4: 4c 89 85 00 02 00 00        	movq	%r8, 0x200(%rbp)
1416f6ffb: 4c 89 a5 08 02 00 00        	movq	%r12, 0x208(%rbp)
1416f7002: 48 8d 85 80 01 00 00        	leaq	0x180(%rbp), %rax
1416f7009: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
1416f7010: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1416f7017: 48 8d 5d 08                 	leaq	0x8(%rbp), %rbx
1416f701b: 48 8d 95 e0 01 00 00        	leaq	0x1e0(%rbp), %rdx
1416f7022: 48 89 d9                    	movq	%rbx, %rcx
1416f7025: e8 b6 c6 1c 00              	callq	0x1418c36e0 <.text+0x18c26e0>
1416f702a: f6 85 80 01 00 00 01        	testb	$0x1, 0x180(%rbp)
1416f7031: 0f 84 3d 07 00 00           	je	0x1416f7774 <.text+0x16f6774>
1416f7037: 48 8b 85 84 01 00 00        	movq	0x184(%rbp), %rax
1416f703e: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
1416f7042: 0f 10 4d 18                 	movups	0x18(%rbp), %xmm1
1416f7046: 0f 10 55 28                 	movups	0x28(%rbp), %xmm2
1416f704a: 0f 10 5d 38                 	movups	0x38(%rbp), %xmm3
1416f704e: 0f 29 85 a0 00 00 00        	movaps	%xmm0, 0xa0(%rbp)
1416f7055: 0f 29 8d b0 00 00 00        	movaps	%xmm1, 0xb0(%rbp)
1416f705c: 0f 29 95 c0 00 00 00        	movaps	%xmm2, 0xc0(%rbp)
1416f7063: 0f 29 9d d0 00 00 00        	movaps	%xmm3, 0xd0(%rbp)
1416f706a: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
1416f706e: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
1416f7075: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
1416f7079: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1416f7080: 0f 10 45 64                 	movups	0x64(%rbp), %xmm0
1416f7084: 0f 11 85 fc 00 00 00        	movups	%xmm0, 0xfc(%rbp)
1416f708b: 8b 4d 74                    	movl	0x74(%rbp), %ecx
1416f708e: 89 8d 5c 02 00 00           	movl	%ecx, 0x25c(%rbp)
1416f7094: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
1416f709b: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1416f70a2: c7 85 98 00 00 00 00 00 00 00       	movl	$0x0, 0x98(%rbp)
1416f70ac: c7 45 00 00 00 00 00        	movl	$0x0, (%rbp)
1416f70b3: 49 8d 45 40                 	leaq	0x40(%r13), %rax
1416f70b7: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
1416f70be: 48 c1 e6 05                 	shlq	$0x5, %rsi
1416f70c2: 48 8d 3c 76                 	leaq	(%rsi,%rsi,2), %rdi
1416f70c6: 45 31 f6                    	xorl	%r14d, %r14d
1416f70c9: 48 8d 85 e0 01 00 00        	leaq	0x1e0(%rbp), %rax
1416f70d0: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1416f70d7: 0f 57 f6                    	xorps	%xmm6, %xmm6
1416f70da: 31 f6                       	xorl	%esi, %esi
1416f70dc: 4c 39 f7                    	cmpq	%r14, %rdi
1416f70df: 0f 84 13 01 00 00           	je	0x1416f71f8 <.text+0x16f61f8>
1416f70e5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1416f70f0: 45 8b 7d 50                 	movl	0x50(%r13), %r15d
1416f70f4: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
1416f70f9: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1416f7100: 4c 89 e2                    	movq	%r12, %rdx
1416f7103: 4c 8b 85 60 02 00 00        	movq	0x260(%rbp), %r8
1416f710a: 45 89 f9                    	movl	%r15d, %r9d
1416f710d: e8 4e 09 4f ff              	callq	0x140be7a60 <.text+0xbe6a60>
1416f7112: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
1416f7119: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
1416f7120: 48 83 f9 0b                 	cmpq	$0xb, %rcx
1416f7124: 0f 85 f0 01 00 00           	jne	0x1416f731a <.text+0x16f631a>
1416f712a: 83 fe 09                    	cmpl	$0x9, %esi
1416f712d: 0f 83 bb 05 00 00           	jae	0x1416f76ee <.text+0x16f66ee>
1416f7133: 48 89 44 f5 b8              	movq	%rax, -0x48(%rbp,%rsi,8)
1416f7138: 8d 46 01                    	leal	0x1(%rsi), %eax
1416f713b: 89 45 00                    	movl	%eax, (%rbp)
1416f713e: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
1416f7143: 4c 39 fa                    	cmpq	%r15, %rdx
1416f7146: 0f 86 78 05 00 00           	jbe	0x1416f76c4 <.text+0x16f66c4>
1416f714c: 49 8b 44 24 08              	movq	0x8(%r12), %rax
1416f7151: 49 8b 54 24 40              	movq	0x40(%r12), %rdx
1416f7156: 42 0f b7 0c b8              	movzwl	(%rax,%r15,4), %ecx
1416f715b: 48 39 ca                    	cmpq	%rcx, %rdx
1416f715e: 0f 86 6f 05 00 00           	jbe	0x1416f76d3 <.text+0x16f66d3>
1416f7164: 49 8b 54 24 28              	movq	0x28(%r12), %rdx
1416f7169: 4c 39 fa                    	cmpq	%r15, %rdx
1416f716c: 0f 86 6d 05 00 00           	jbe	0x1416f76df <.text+0x16f66df>
1416f7172: 49 8b 44 24 38              	movq	0x38(%r12), %rax
1416f7177: c1 e1 06                    	shll	$0x6, %ecx
1416f717a: 83 3c 08 01                 	cmpl	$0x1, (%rax,%rcx)
1416f717e: 77 1b                       	ja	0x1416f719b <.text+0x16f619b>
1416f7180: 48 01 c8                    	addq	%rcx, %rax
1416f7183: 48 8b 40 2c                 	movq	0x2c(%rax), %rax
1416f7187: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1416f718e: 48 3b 85 68 02 00 00        	cmpq	0x268(%rbp), %rax
1416f7195: 0f 85 05 04 00 00           	jne	0x1416f75a0 <.text+0x16f65a0>
1416f719b: 41 f6 45 3c 01              	testb	$0x1, 0x3c(%r13)
1416f71a0: 74 1e                       	je	0x1416f71c0 <.text+0x16f61c0>
1416f71a2: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
1416f71a9: 42 0f 10 04 30              	movups	(%rax,%r14), %xmm0
1416f71ae: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
1416f71b5: eb 10                       	jmp	0x1416f71c7 <.text+0x16f61c7>
1416f71b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416f71c0: 0f 29 b5 80 01 00 00        	movaps	%xmm6, 0x180(%rbp)
1416f71c7: 83 fe 09                    	cmpl	$0x9, %esi
1416f71ca: 0f 83 50 05 00 00           	jae	0x1416f7720 <.text+0x16f6720>
1416f71d0: 49 83 c5 60                 	addq	$0x60, %r13
1416f71d4: 0f 28 85 80 01 00 00        	movaps	0x180(%rbp), %xmm0
1416f71db: 0f 11 03                    	movups	%xmm0, (%rbx)
1416f71de: 48 ff c6                    	incq	%rsi
1416f71e1: 89 b5 98 00 00 00           	movl	%esi, 0x98(%rbp)
1416f71e7: 48 83 c3 10                 	addq	$0x10, %rbx
1416f71eb: 49 83 c6 60                 	addq	$0x60, %r14
1416f71ef: 4c 39 f7                    	cmpq	%r14, %rdi
1416f71f2: 0f 85 f8 fe ff ff           	jne	0x1416f70f0 <.text+0x16f60f0>
1416f71f8: 83 bd 58 02 00 00 02        	cmpl	$0x2, 0x258(%rbp)
1416f71ff: 75 0e                       	jne	0x1416f720f <.text+0x16f620f>
1416f7201: 89 f0                       	movl	%esi, %eax
1416f7203: 48 8b bd 60 02 00 00        	movq	0x260(%rbp), %rdi
1416f720a: e9 6e 01 00 00              	jmp	0x1416f737d <.text+0x16f637d>
1416f720f: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
1416f7216: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
1416f721a: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1416f7221: 0f 10 00                    	movups	(%rax), %xmm0
1416f7224: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1416f7228: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
1416f722c: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
1416f7230: 0f 29 9d b0 01 00 00        	movaps	%xmm3, 0x1b0(%rbp)
1416f7237: 0f 29 95 a0 01 00 00        	movaps	%xmm2, 0x1a0(%rbp)
1416f723e: 0f 29 8d 90 01 00 00        	movaps	%xmm1, 0x190(%rbp)
1416f7245: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
1416f724c: 44 8b bd c4 01 00 00        	movl	0x1c4(%rbp), %r15d
1416f7253: 48 8d 85 80 01 00 00        	leaq	0x180(%rbp), %rax
1416f725a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416f725f: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1416f7266: 48 89 8d d0 01 00 00        	movq	%rcx, 0x1d0(%rbp)
1416f726d: 4c 89 e2                    	movq	%r12, %rdx
1416f7270: 48 8b bd 60 02 00 00        	movq	0x260(%rbp), %rdi
1416f7277: 49 89 f8                    	movq	%rdi, %r8
1416f727a: 45 89 f9                    	movl	%r15d, %r9d
1416f727d: e8 de 07 4f ff              	callq	0x140be7a60 <.text+0xbe6a60>
1416f7282: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
1416f7289: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
1416f7290: 48 83 f9 0b                 	cmpq	$0xb, %rcx
1416f7294: 0f 85 80 00 00 00           	jne	0x1416f731a <.text+0x16f631a>
1416f729a: 83 fe 09                    	cmpl	$0x9, %esi
1416f729d: 0f 83 25 05 00 00           	jae	0x1416f77c8 <.text+0x16f67c8>
1416f72a3: 48 89 44 f5 b8              	movq	%rax, -0x48(%rbp,%rsi,8)
1416f72a8: 48 8d 46 01                 	leaq	0x1(%rsi), %rax
1416f72ac: 89 45 00                    	movl	%eax, (%rbp)
1416f72af: 49 8b 54 24 10              	movq	0x10(%r12), %rdx
1416f72b4: 4c 39 fa                    	cmpq	%r15, %rdx
1416f72b7: 0f 86 07 04 00 00           	jbe	0x1416f76c4 <.text+0x16f66c4>
1416f72bd: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
1416f72c2: 49 8b 54 24 40              	movq	0x40(%r12), %rdx
1416f72c7: 42 0f b7 0c b9              	movzwl	(%rcx,%r15,4), %ecx
1416f72cc: 48 39 ca                    	cmpq	%rcx, %rdx
1416f72cf: 0f 86 fe 03 00 00           	jbe	0x1416f76d3 <.text+0x16f66d3>
1416f72d5: 49 8b 54 24 28              	movq	0x28(%r12), %rdx
1416f72da: 4c 39 fa                    	cmpq	%r15, %rdx
1416f72dd: 0f 86 fc 03 00 00           	jbe	0x1416f76df <.text+0x16f66df>
1416f72e3: 49 8b 54 24 38              	movq	0x38(%r12), %rdx
1416f72e8: c1 e1 06                    	shll	$0x6, %ecx
1416f72eb: 83 3c 0a 01                 	cmpl	$0x1, (%rdx,%rcx)
1416f72ef: 77 1b                       	ja	0x1416f730c <.text+0x16f630c>
1416f72f1: 48 01 ca                    	addq	%rcx, %rdx
1416f72f4: 48 8b 4a 2c                 	movq	0x2c(%rdx), %rcx
1416f72f8: 48 89 8d 60 01 00 00        	movq	%rcx, 0x160(%rbp)
1416f72ff: 48 3b 8d 68 02 00 00        	cmpq	0x268(%rbp), %rcx
1416f7306: 0f 85 93 03 00 00           	jne	0x1416f769f <.text+0x16f669f>
1416f730c: 80 bd bc 01 00 00 00        	cmpb	$0x0, 0x1bc(%rbp)
1416f7313: 75 44                       	jne	0x1416f7359 <.text+0x16f6359>
1416f7315: 0f 57 c0                    	xorps	%xmm0, %xmm0
1416f7318: eb 47                       	jmp	0x1416f7361 <.text+0x16f6361>
1416f731a: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1416f7321: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1416f7328: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1416f732f: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1416f7336: 48 8b b5 70 02 00 00        	movq	0x270(%rbp), %rsi
1416f733d: 0f 11 5e 40                 	movups	%xmm3, 0x40(%rsi)
1416f7341: 0f 11 56 30                 	movups	%xmm2, 0x30(%rsi)
1416f7345: 0f 11 4e 20                 	movups	%xmm1, 0x20(%rsi)
1416f7349: 0f 11 46 10                 	movups	%xmm0, 0x10(%rsi)
1416f734d: 48 89 0e                    	movq	%rcx, (%rsi)
1416f7350: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1416f7354: e9 29 02 00 00              	jmp	0x1416f7582 <.text+0x16f6582>
1416f7359: f3 0f 10 85 c0 01 00 00     	movss	0x1c0(%rbp), %xmm0
1416f7361: 83 fe 09                    	cmpl	$0x9, %esi
1416f7364: 0f 83 71 04 00 00           	jae	0x1416f77db <.text+0x16f67db>
1416f736a: f3 0f 11 03                 	movss	%xmm0, (%rbx)
1416f736e: c7 43 04 00 00 00 00        	movl	$0x0, 0x4(%rbx)
1416f7375: ff c6                       	incl	%esi
1416f7377: 89 b5 98 00 00 00           	movl	%esi, 0x98(%rbp)
1416f737d: 0f 10 85 fc 00 00 00        	movups	0xfc(%rbp), %xmm0
1416f7384: 0f 11 85 3c 02 00 00        	movups	%xmm0, 0x23c(%rbp)
1416f738b: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
1416f7392: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
1416f7399: 0f 28 85 e0 00 00 00        	movaps	0xe0(%rbp), %xmm0
1416f73a0: 0f 29 85 20 02 00 00        	movaps	%xmm0, 0x220(%rbp)
1416f73a7: 0f 28 85 a0 00 00 00        	movaps	0xa0(%rbp), %xmm0
1416f73ae: 0f 28 8d b0 00 00 00        	movaps	0xb0(%rbp), %xmm1
1416f73b5: 0f 28 95 c0 00 00 00        	movaps	0xc0(%rbp), %xmm2
1416f73bc: 0f 28 9d d0 00 00 00        	movaps	0xd0(%rbp), %xmm3
1416f73c3: 0f 29 9d 10 02 00 00        	movaps	%xmm3, 0x210(%rbp)
1416f73ca: 0f 29 95 00 02 00 00        	movaps	%xmm2, 0x200(%rbp)
1416f73d1: 0f 29 8d f0 01 00 00        	movaps	%xmm1, 0x1f0(%rbp)
1416f73d8: 0f 29 85 e0 01 00 00        	movaps	%xmm0, 0x1e0(%rbp)
1416f73df: 8b 8d 5c 02 00 00           	movl	0x25c(%rbp), %ecx
1416f73e5: 89 8d 4c 02 00 00           	movl	%ecx, 0x24c(%rbp)
1416f73eb: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
1416f73f2: 48 89 8d 50 02 00 00        	movq	%rcx, 0x250(%rbp)
1416f73f9: 89 c0                       	movl	%eax, %eax
1416f73fb: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1416f7400: 48 8d 45 b8                 	leaq	-0x48(%rbp), %rax
1416f7404: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416f7409: 48 8d 9d 80 01 00 00        	leaq	0x180(%rbp), %rbx
1416f7410: 4c 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %r9
1416f7417: 48 89 d9                    	movq	%rbx, %rcx
1416f741a: 48 89 fa                    	movq	%rdi, %rdx
1416f741d: 4c 8b 85 40 01 00 00        	movq	0x140(%rbp), %r8
1416f7424: e8 c7 4a 50 ff              	callq	0x140bfbef0 <.text+0xbfaef0>
1416f7429: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1416f7430: 0f 10 85 88 01 00 00        	movups	0x188(%rbp), %xmm0
1416f7437: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
1416f743e: 0f 10 85 98 01 00 00        	movups	0x198(%rbp), %xmm0
1416f7445: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
1416f744c: 48 83 f8 0b                 	cmpq	$0xb, %rax
1416f7450: 0f 85 eb 00 00 00           	jne	0x1416f7541 <.text+0x16f6541>
1416f7456: 0f 28 85 60 01 00 00        	movaps	0x160(%rbp), %xmm0
1416f745d: 0f 28 8d 70 01 00 00        	movaps	0x170(%rbp), %xmm1
1416f7464: 0f 29 8d 20 01 00 00        	movaps	%xmm1, 0x120(%rbp)
1416f746b: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1416f7472: f2 0f 10 85 68 02 00 00     	movsd	0x268(%rbp), %xmm0
1416f747a: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
1416f7481: 48 8b 08                    	movq	(%rax), %rcx
1416f7484: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1416f748b: 8b 42 0c                    	movl	0xc(%rdx), %eax
1416f748e: 3b 85 28 01 00 00           	cmpl	0x128(%rbp), %eax
1416f7494: 4c 8b 95 50 01 00 00        	movq	0x150(%rbp), %r10
1416f749b: 75 24                       	jne	0x1416f74c1 <.text+0x16f64c1>
1416f749d: 41 83 3a 01                 	cmpl	$0x1, (%r10)
1416f74a1: 0f 85 98 01 00 00           	jne	0x1416f763f <.text+0x16f663f>
1416f74a7: 4d 8b 42 08                 	movq	0x8(%r10), %r8
1416f74ab: 48 8b 02                    	movq	(%rdx), %rax
1416f74ae: 66 90                       	nop
1416f74b0: 4c 39 c0                    	cmpq	%r8, %rax
1416f74b3: 4d 89 c1                    	movq	%r8, %r9
1416f74b6: 4c 0f 47 c8                 	cmovaq	%rax, %r9
1416f74ba: f0                          	lock
1416f74bb: 4c 0f b1 0a                 	cmpxchgq	%r9, (%rdx)
1416f74bf: 75 ef                       	jne	0x1416f74b0 <.text+0x16f64b0>
1416f74c1: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
1416f74c8: 48 8b 00                    	movq	(%rax), %rax
1416f74cb: c7 85 e0 01 00 00 2b 00 00 00       	movl	$0x2b, 0x1e0(%rbp)
1416f74d5: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
1416f74e0: 48 89 8d f0 01 00 00        	movq	%rcx, 0x1f0(%rbp)
1416f74e7: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1416f74ee: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
1416f74f9: 0f 13 85 08 02 00 00        	movlps	%xmm0, 0x208(%rbp)
1416f7500: 89 b5 10 02 00 00           	movl	%esi, 0x210(%rbp)
1416f7506: 48 8d 45 08                 	leaq	0x8(%rbp), %rax
1416f750a: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1416f7511: 49 8b 4a 48                 	movq	0x48(%r10), %rcx
1416f7515: 48 8d 95 e0 01 00 00        	leaq	0x1e0(%rbp), %rdx
1416f751c: 45 31 c0                    	xorl	%r8d, %r8d
1416f751f: ff 97 18 04 00 00           	callq	*0x418(%rdi)
1416f7525: 48 8b b5 70 02 00 00        	movq	0x270(%rbp), %rsi
1416f752c: 48 c7 06 0b 00 00 00        	movq	$0xb, (%rsi)
1416f7533: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
1416f753a: e8 71 af 4e ff              	callq	0x140be24b0 <.text+0xbe14b0>
1416f753f: eb 41                       	jmp	0x1416f7582 <.text+0x16f6582>
1416f7541: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1416f7548: 48 8b b5 70 02 00 00        	movq	0x270(%rbp), %rsi
1416f754f: 48 89 4e 48                 	movq	%rcx, 0x48(%rsi)
1416f7553: 0f 10 85 a8 01 00 00        	movups	0x1a8(%rbp), %xmm0
1416f755a: 0f 10 8d b8 01 00 00        	movups	0x1b8(%rbp), %xmm1
1416f7561: 0f 11 4e 38                 	movups	%xmm1, 0x38(%rsi)
1416f7565: 0f 11 46 28                 	movups	%xmm0, 0x28(%rsi)
1416f7569: 0f 28 85 60 01 00 00        	movaps	0x160(%rbp), %xmm0
1416f7570: 0f 28 8d 70 01 00 00        	movaps	0x170(%rbp), %xmm1
1416f7577: 0f 11 4e 18                 	movups	%xmm1, 0x18(%rsi)
1416f757b: 0f 11 46 08                 	movups	%xmm0, 0x8(%rsi)
1416f757f: 48 89 06                    	movq	%rax, (%rsi)
1416f7582: 48 89 f0                    	movq	%rsi, %rax
1416f7585: 0f 28 b5 80 02 00 00        	movaps	0x280(%rbp), %xmm6
1416f758c: 48 81 c4 18 03 00 00        	addq	$0x318, %rsp            # imm = 0x318
1416f7593: 5b                          	popq	%rbx
1416f7594: 5f                          	popq	%rdi
1416f7595: 5e                          	popq	%rsi
1416f7596: 41 5c                       	popq	%r12
1416f7598: 41 5d                       	popq	%r13
1416f759a: 41 5e                       	popq	%r14
1416f759c: 41 5f                       	popq	%r15
1416f759e: 5d                          	popq	%rbp
1416f759f: c3                          	retq
1416f75a0: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1416f75ab: 48 8d 05 ae a0 59 01        	leaq	0x159a0ae(%rip), %rax   # 0x142c91660
1416f75b2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416f75b7: 4c 8d 85 68 02 00 00        	leaq	0x268(%rbp), %r8
1416f75be: 31 c9                       	xorl	%ecx, %ecx
1416f75c0: 48 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %rdx
1416f75c7: 4c 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %r9
1416f75ce: e8 f4 32 1d 01              	callq	0x1428ca8c7 <.text+0x28c98c7>
1416f75d3: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
1416f75db: 48 83 c2 1c                 	addq	$0x1c, %rdx
1416f75df: 49 81 c0 50 0a 00 00        	addq	$0xa50, %r8             # imm = 0xA50
1416f75e6: 48 8d 05 b3 ac 60 01        	leaq	0x160acb3(%rip), %rax   # 0x142d022a0
1416f75ed: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416f75f2: 4c 8d 4d 08                 	leaq	0x8(%rbp), %r9
1416f75f6: 31 c9                       	xorl	%ecx, %ecx
1416f75f8: e8 19 32 1d 01              	callq	0x1428ca816 <.text+0x28c9816>
1416f75fd: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
1416f7605: 48 83 c2 1e                 	addq	$0x1e, %rdx
1416f7609: 48 8d 05 b0 ac 60 01        	leaq	0x160acb0(%rip), %rax   # 0x142d022c0
1416f7610: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416f7615: 4c 8d 05 9c ac 60 01        	leaq	0x160ac9c(%rip), %r8    # 0x142d022b8
1416f761c: 4c 8d 4d 08                 	leaq	0x8(%rbp), %r9
1416f7620: 31 c9                       	xorl	%ecx, %ecx
1416f7622: e8 db 32 1d 01              	callq	0x1428ca902 <.text+0x28c9902>
1416f7627: 48 8d 0d 62 a0 59 01        	leaq	0x159a062(%rip), %rcx   # 0x142c91690
1416f762e: 4c 8d 05 cb a0 59 01        	leaq	0x159a0cb(%rip), %r8    # 0x142c91700
1416f7635: ba 6d 00 00 00              	movl	$0x6d, %edx
1416f763a: e8 f1 1d 1e 01              	callq	0x1428d9430 <.text+0x28d8430>
1416f763f: 4c 89 95 80 01 00 00        	movq	%r10, 0x180(%rbp)
1416f7646: 48 8d 05 d3 5f 4f ff        	leaq	-0xb0a02d(%rip), %rax   # 0x140bed620 <.text+0xbec620>
1416f764d: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1416f7654: 48 8d 05 55 d5 41 01        	leaq	0x141d555(%rip), %rax   # 0x142b14bb0
1416f765b: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1416f7662: 48 c7 85 e8 01 00 00 01 00 00 00    	movq	$0x1, 0x1e8(%rbp)
1416f766d: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
1416f7678: 48 89 9d f0 01 00 00        	movq	%rbx, 0x1f0(%rbp)
1416f767f: 48 c7 85 f8 01 00 00 01 00 00 00    	movq	$0x1, 0x1f8(%rbp)
1416f768a: 48 8d 15 5f d5 41 01        	leaq	0x141d55f(%rip), %rdx   # 0x142b14bf0
1416f7691: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
1416f7698: e8 f3 1c 1e 01              	callq	0x1428d9390 <.text+0x28d8390>
1416f769d: 0f 0b                       	ud2
1416f769f: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1416f76aa: 48 8d 85 60 01 00 00        	leaq	0x160(%rbp), %rax
1416f76b1: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
1416f76b8: 48 8d 05 59 9f 59 01        	leaq	0x1599f59(%rip), %rax   # 0x142c91618
1416f76bf: e9 ee fe ff ff              	jmp	0x1416f75b2 <.text+0x16f65b2>
1416f76c4: 4c 8d 05 95 c5 41 01        	leaq	0x141c595(%rip), %r8    # 0x142b13c60
1416f76cb: 4c 89 f9                    	movq	%r15, %rcx
1416f76ce: e8 45 1e 1e 01              	callq	0x1428d9518 <.text+0x28d8518>
1416f76d3: 4c 8d 05 9e c5 41 01        	leaq	0x141c59e(%rip), %r8    # 0x142b13c78
1416f76da: e8 39 1e 1e 01              	callq	0x1428d9518 <.text+0x28d8518>
1416f76df: 4c 8d 05 aa c5 41 01        	leaq	0x141c5aa(%rip), %r8    # 0x142b13c90
1416f76e6: 4c 89 f9                    	movq	%r15, %rcx
1416f76e9: e8 2a 1e 1e 01              	callq	0x1428d9518 <.text+0x28d8518>
1416f76ee: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1416f76f5: 48 8d 05 4c 9f 59 01        	leaq	0x1599f4c(%rip), %rax   # 0x142c91648
1416f76fc: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416f7701: 48 8d 0d 48 f1 60 01        	leaq	0x160f148(%rip), %rcx   # 0x142d06850
1416f7708: 4c 8d 0d 71 f1 60 01        	leaq	0x160f171(%rip), %r9    # 0x142d06880
1416f770f: 4c 8d 85 e0 01 00 00        	leaq	0x1e0(%rbp), %r8
1416f7716: ba 2b 00 00 00              	movl	$0x2b, %edx
1416f771b: e8 40 20 1e 01              	callq	0x1428d9760 <.text+0x28d8760>
1416f7720: 0f 28 85 80 01 00 00        	movaps	0x180(%rbp), %xmm0
1416f7727: 0f 29 85 e0 01 00 00        	movaps	%xmm0, 0x1e0(%rbp)
1416f772e: 48 8d 05 43 9f 59 01        	leaq	0x1599f43(%rip), %rax   # 0x142c91678
1416f7735: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416f773a: 48 8d 0d 0f f1 60 01        	leaq	0x160f10f(%rip), %rcx   # 0x142d06850
1416f7741: 4c 8d 0d e8 f0 60 01        	leaq	0x160f0e8(%rip), %r9    # 0x142d06830
1416f7748: 4c 8d 85 e0 01 00 00        	leaq	0x1e0(%rbp), %r8
1416f774f: ba 2b 00 00 00              	movl	$0x2b, %edx
1416f7754: e8 07 20 1e 01              	callq	0x1428d9760 <.text+0x28d8760>
1416f7759: 48 8d 0d 88 9e 59 01        	leaq	0x1599e88(%rip), %rcx   # 0x142c915e8
1416f7760: e8 ab 1b 1e 01              	callq	0x1428d9310 <.text+0x28d8310>
1416f7765: 49 83 c1 fe                 	addq	$-0x2, %r9
1416f7769: 49 83 f9 03                 	cmpq	$0x3, %r9
1416f776d: 73 41                       	jae	0x1416f77b0 <.text+0x16f67b0>
1416f776f: e8 cc 32 1d 01              	callq	0x1428caa40 <.text+0x28c9a40>
1416f7774: 48 8d 0d cd 1c 8e 01        	leaq	0x18e1ccd(%rip), %rcx   # 0x142fd9448
1416f777b: 4c 8d 05 1e 1d 8e 01        	leaq	0x18e1d1e(%rip), %r8    # 0x142fd94a0
1416f7782: ba 2d 00 00 00              	movl	$0x2d, %edx
1416f7787: e8 a4 1b 1e 01              	callq	0x1428d9330 <.text+0x28d8330>
1416f778c: 4c 8d 05 15 c5 41 01        	leaq	0x141c515(%rip), %r8    # 0x142b13ca8
1416f7793: e8 80 1d 1e 01              	callq	0x1428d9518 <.text+0x28d8518>
1416f7798: 4c 8d 05 21 c5 41 01        	leaq	0x141c521(%rip), %r8    # 0x142b13cc0
1416f779f: e8 74 1d 1e 01              	callq	0x1428d9518 <.text+0x28d8518>
1416f77a4: 48 8d 0d 6d 9f 59 01        	leaq	0x1599f6d(%rip), %rcx   # 0x142c91718
1416f77ab: e8 60 1b 1e 01              	callq	0x1428d9310 <.text+0x28d8310>
1416f77b0: 48 8d 0d 11 8a 41 01        	leaq	0x1418a11(%rip), %rcx   # 0x142b101c8
1416f77b7: 4c 8d 05 1a c5 41 01        	leaq	0x141c51a(%rip), %r8    # 0x142b13cd8
1416f77be: ba 28 00 00 00              	movl	$0x28, %edx
1416f77c3: e8 68 1c 1e 01              	callq	0x1428d9430 <.text+0x28d8430>
1416f77c8: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1416f77cf: 48 8d 05 2a 9e 59 01        	leaq	0x1599e2a(%rip), %rax   # 0x142c91600
1416f77d6: e9 21 ff ff ff              	jmp	0x1416f76fc <.text+0x16f66fc>
1416f77db: f3 0f 11 85 e0 01 00 00     	movss	%xmm0, 0x1e0(%rbp)
1416f77e3: c7 85 e4 01 00 00 00 00 00 00       	movl	$0x0, 0x1e4(%rbp)
1416f77ed: 48 8d 05 3c 9e 59 01        	leaq	0x1599e3c(%rip), %rax   # 0x142c91630
1416f77f4: e9 3c ff ff ff              	jmp	0x1416f7735 <.text+0x16f6735>
1416f77f9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1416f7800: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416f7805: 55                          	pushq	%rbp
1416f7806: 41 57                       	pushq	%r15
1416f7808: 41 56                       	pushq	%r14
1416f780a: 41 55                       	pushq	%r13
1416f780c: 41 54                       	pushq	%r12
1416f780e: 56                          	pushq	%rsi
1416f780f: 57                          	pushq	%rdi
1416f7810: 53                          	pushq	%rbx
1416f7811: 48 83 ec 48                 	subq	$0x48, %rsp
1416f7815: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416f781c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1416f7821: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
1416f7828: e8 83 ac 4e ff              	callq	0x140be24b0 <.text+0xbe14b0>
1416f782d: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1416f7832: 48 83 c4 48                 	addq	$0x48, %rsp
1416f7836: 5b                          	popq	%rbx
1416f7837: 5f                          	popq	%rdi
1416f7838: 5e                          	popq	%rsi
1416f7839: 41 5c                       	popq	%r12
1416f783b: 41 5d                       	popq	%r13
1416f783d: 41 5e                       	popq	%r14
1416f783f: 41 5f                       	popq	%r15
1416f7841: 5d                          	popq	%rbp
1416f7842: c3                          	retq
