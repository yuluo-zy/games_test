
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408d9d30: 55                          	pushq	%rbp
1408d9d31: 41 57                       	pushq	%r15
1408d9d33: 41 56                       	pushq	%r14
1408d9d35: 56                          	pushq	%rsi
1408d9d36: 57                          	pushq	%rdi
1408d9d37: 53                          	pushq	%rbx
1408d9d38: 48 81 ec 48 03 00 00        	subq	$0x348, %rsp            # imm = 0x348
1408d9d3f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408d9d47: 48 83 e4 80                 	andq	$-0x80, %rsp
1408d9d4b: 48 89 e3                    	movq	%rsp, %rbx
1408d9d4e: 48 89 ab 38 03 00 00        	movq	%rbp, 0x338(%rbx)
1408d9d55: 48 c7 85 c0 02 00 00 fe ff ff ff    	movq	$-0x2, 0x2c0(%rbp)
1408d9d60: 48 89 ce                    	movq	%rcx, %rsi
1408d9d63: 48 85 d2                    	testq	%rdx, %rdx
1408d9d66: 0f 84 50 01 00 00           	je	0x1408d9ebc <.text+0x8d8ebc>
1408d9d6c: 48 0f bd ca                 	bsrq	%rdx, %rcx
1408d9d70: f7 d1                       	notl	%ecx
1408d9d72: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
1408d9d79: 48 d3 e8                    	shrq	%cl, %rax
1408d9d7c: 48 8d 4a 01                 	leaq	0x1(%rdx), %rcx
1408d9d80: 48 ff c0                    	incq	%rax
1408d9d83: 48 83 f9 02                 	cmpq	$0x2, %rcx
1408d9d87: 41 be 01 00 00 00           	movl	$0x1, %r14d
1408d9d8d: 4c 0f 43 f0                 	cmovaeq	%rax, %r14
1408d9d91: 4f 8d 3c 36                 	leaq	(%r14,%r14), %r15
1408d9d95: 31 c9                       	xorl	%ecx, %ecx
1408d9d97: 48 89 d7                    	movq	%rdx, %rdi
1408d9d9a: e8 31 ca db 00              	callq	0x1416967d0 <.text+0x16957d0>
1408d9d9f: 48 c7 83 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbx)
1408d9daa: 48 c7 83 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbx)
1408d9db5: 48 89 83 18 02 00 00        	movq	%rax, 0x218(%rbx)
1408d9dbc: 48 89 93 20 02 00 00        	movq	%rdx, 0x220(%rbx)
1408d9dc3: 48 89 bb 00 02 00 00        	movq	%rdi, 0x200(%rbx)
1408d9dca: 4c 89 bb 08 02 00 00        	movq	%r15, 0x208(%rbx)
1408d9dd1: 4c 89 b3 10 02 00 00        	movq	%r14, 0x210(%rbx)
1408d9dd8: 66 c7 83 80 01 00 00 00 00  	movw	$0x0, 0x180(%rbx)
1408d9de1: 48 c7 83 88 01 00 00 00 00 00 00    	movq	$0x0, 0x188(%rbx)
1408d9dec: 48 c7 83 90 01 00 00 08 00 00 00    	movq	$0x8, 0x190(%rbx)
1408d9df7: 0f 57 c0                    	xorps	%xmm0, %xmm0
1408d9dfa: 0f 11 83 98 01 00 00        	movups	%xmm0, 0x198(%rbx)
1408d9e01: 48 c7 83 a8 01 00 00 08 00 00 00    	movq	$0x8, 0x1a8(%rbx)
1408d9e0c: 48 c7 83 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbx)
1408d9e17: c6 83 b8 01 00 00 01        	movb	$0x1, 0x1b8(%rbx)
1408d9e1e: 66 c7 83 c0 01 00 00 00 00  	movw	$0x0, 0x1c0(%rbx)
1408d9e27: 48 c7 83 c8 01 00 00 00 00 00 00    	movq	$0x0, 0x1c8(%rbx)
1408d9e32: 48 c7 83 d0 01 00 00 08 00 00 00    	movq	$0x8, 0x1d0(%rbx)
1408d9e3d: 0f 11 83 d8 01 00 00        	movups	%xmm0, 0x1d8(%rbx)
1408d9e44: 48 c7 83 e8 01 00 00 08 00 00 00    	movq	$0x8, 0x1e8(%rbx)
1408d9e4f: 48 c7 83 f0 01 00 00 00 00 00 00    	movq	$0x0, 0x1f0(%rbx)
1408d9e5a: c6 83 f8 01 00 00 01        	movb	$0x1, 0x1f8(%rbx)
1408d9e61: 48 c7 83 80 02 00 00 01 00 00 00    	movq	$0x1, 0x280(%rbx)
1408d9e6c: 48 c7 83 88 02 00 00 01 00 00 00    	movq	$0x1, 0x288(%rbx)
1408d9e77: c6 83 90 02 00 00 00        	movb	$0x0, 0x290(%rbx)
1408d9e7e: 0f b6 05 84 ad d5 02        	movzbl	0x2d5ad84(%rip), %eax   # 0x143634c09
1408d9e85: b9 80 02 00 00              	movl	$0x280, %ecx            # imm = 0x280
1408d9e8a: ba 80 00 00 00              	movl	$0x80, %edx
1408d9e8f: e8 7c 9d d3 ff              	callq	0x140613c10 <.text+0x612c10>
1408d9e94: 48 85 c0                    	testq	%rax, %rax
1408d9e97: 0f 84 00 01 00 00           	je	0x1408d9f9d <.text+0x8d8f9d>
1408d9e9d: 48 89 c7                    	movq	%rax, %rdi
1408d9ea0: 48 8d 93 80 00 00 00        	leaq	0x80(%rbx), %rdx
1408d9ea7: 41 b8 80 02 00 00           	movl	$0x280, %r8d            # imm = 0x280
1408d9ead: 48 89 c1                    	movq	%rax, %rcx
1408d9eb0: e8 4b 9d 04 02              	callq	0x142923c00 <.text+0x2922c00>
1408d9eb5: 31 c0                       	xorl	%eax, %eax
1408d9eb7: e9 bf 00 00 00              	jmp	0x1408d9f7b <.text+0x8d8f7b>
1408d9ebc: 48 c7 83 f0 00 00 00 01 00 00 00    	movq	$0x1, 0xf0(%rbx)
1408d9ec7: 48 c7 83 f8 00 00 00 01 00 00 00    	movq	$0x1, 0xf8(%rbx)
1408d9ed2: c6 83 00 01 00 00 00        	movb	$0x0, 0x100(%rbx)
1408d9ed9: 66 c7 83 80 00 00 00 00 00  	movw	$0x0, 0x80(%rbx)
1408d9ee2: 48 c7 83 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbx)
1408d9eed: 48 c7 83 90 00 00 00 08 00 00 00    	movq	$0x8, 0x90(%rbx)
1408d9ef8: 0f 57 c0                    	xorps	%xmm0, %xmm0
1408d9efb: 0f 11 83 98 00 00 00        	movups	%xmm0, 0x98(%rbx)
1408d9f02: 48 c7 83 a8 00 00 00 08 00 00 00    	movq	$0x8, 0xa8(%rbx)
1408d9f0d: 0f 11 83 b0 00 00 00        	movups	%xmm0, 0xb0(%rbx)
1408d9f14: 48 c7 83 c0 00 00 00 08 00 00 00    	movq	$0x8, 0xc0(%rbx)
1408d9f1f: 0f 11 83 c8 00 00 00        	movups	%xmm0, 0xc8(%rbx)
1408d9f26: 48 c7 83 d8 00 00 00 08 00 00 00    	movq	$0x8, 0xd8(%rbx)
1408d9f31: 48 c7 83 e0 00 00 00 00 00 00 00    	movq	$0x0, 0xe0(%rbx)
1408d9f3c: c6 83 e8 00 00 00 00        	movb	$0x0, 0xe8(%rbx)
1408d9f43: 0f b6 05 bf ac d5 02        	movzbl	0x2d5acbf(%rip), %eax   # 0x143634c09
1408d9f4a: b9 88 00 00 00              	movl	$0x88, %ecx
1408d9f4f: ba 08 00 00 00              	movl	$0x8, %edx
1408d9f54: e8 b7 9c d3 ff              	callq	0x140613c10 <.text+0x612c10>
1408d9f59: 48 85 c0                    	testq	%rax, %rax
1408d9f5c: 74 50                       	je	0x1408d9fae <.text+0x8d8fae>
1408d9f5e: 48 89 c7                    	movq	%rax, %rdi
1408d9f61: 48 8d 93 80 00 00 00        	leaq	0x80(%rbx), %rdx
1408d9f68: 41 b8 88 00 00 00           	movl	$0x88, %r8d
1408d9f6e: 48 89 c1                    	movq	%rax, %rcx
1408d9f71: e8 8a 9c 04 02              	callq	0x142923c00 <.text+0x2922c00>
1408d9f76: b8 02 00 00 00              	movl	$0x2, %eax
1408d9f7b: 48 89 06                    	movq	%rax, (%rsi)
1408d9f7e: 48 89 7e 08                 	movq	%rdi, 0x8(%rsi)
1408d9f82: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1408d9f86: 48 89 7e 18                 	movq	%rdi, 0x18(%rsi)
1408d9f8a: 48 89 f0                    	movq	%rsi, %rax
1408d9f8d: 48 8d a5 c8 02 00 00        	leaq	0x2c8(%rbp), %rsp
1408d9f94: 5b                          	popq	%rbx
1408d9f95: 5f                          	popq	%rdi
1408d9f96: 5e                          	popq	%rsi
1408d9f97: 41 5e                       	popq	%r14
1408d9f99: 41 5f                       	popq	%r15
1408d9f9b: 5d                          	popq	%rbp
1408d9f9c: c3                          	retq
1408d9f9d: b9 80 00 00 00              	movl	$0x80, %ecx
1408d9fa2: ba 80 02 00 00              	movl	$0x280, %edx            # imm = 0x280
1408d9fa7: e8 54 f0 ff 01              	callq	0x1428d9000 <.text+0x28d8000>
1408d9fac: eb 0f                       	jmp	0x1408d9fbd <.text+0x8d8fbd>
1408d9fae: b9 08 00 00 00              	movl	$0x8, %ecx
1408d9fb3: ba 88 00 00 00              	movl	$0x88, %edx
1408d9fb8: e8 43 f0 ff 01              	callq	0x1428d9000 <.text+0x28d8000>
1408d9fbd: 0f 0b                       	ud2
1408d9fbf: 90                          	nop
1408d9fc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408d9fc5: 55                          	pushq	%rbp
1408d9fc6: 41 57                       	pushq	%r15
1408d9fc8: 41 56                       	pushq	%r14
1408d9fca: 56                          	pushq	%rsi
1408d9fcb: 57                          	pushq	%rdi
1408d9fcc: 53                          	pushq	%rbx
1408d9fcd: 48 83 ec 28                 	subq	$0x28, %rsp
1408d9fd1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408d9fd8: 48 83 e2 80                 	andq	$-0x80, %rdx
1408d9fdc: 48 89 d3                    	movq	%rdx, %rbx
1408d9fdf: 48 8d 8b 80 00 00 00        	leaq	0x80(%rbx), %rcx
1408d9fe6: e8 65 8b cf ff              	callq	0x1405d2b50 <.text+0x5d1b50>
1408d9feb: 90                          	nop
1408d9fec: 48 83 c4 28                 	addq	$0x28, %rsp
