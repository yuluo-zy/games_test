
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140867970: 55                          	pushq	%rbp
140867971: 41 57                       	pushq	%r15
140867973: 41 56                       	pushq	%r14
140867975: 41 55                       	pushq	%r13
140867977: 41 54                       	pushq	%r12
140867979: 56                          	pushq	%rsi
14086797a: 57                          	pushq	%rdi
14086797b: 53                          	pushq	%rbx
14086797c: 48 81 ec 48 04 00 00        	subq	$0x448, %rsp            # imm = 0x448
140867983: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14086798b: 44 0f 29 8d b0 03 00 00     	movaps	%xmm9, 0x3b0(%rbp)
140867993: 44 0f 29 85 a0 03 00 00     	movaps	%xmm8, 0x3a0(%rbp)
14086799b: 0f 29 bd 90 03 00 00        	movaps	%xmm7, 0x390(%rbp)
1408679a2: 0f 29 b5 80 03 00 00        	movaps	%xmm6, 0x380(%rbp)
1408679a9: 48 c7 85 78 03 00 00 fe ff ff ff    	movq	$-0x2, 0x378(%rbp)
1408679b4: 4c 89 cf                    	movq	%r9, %rdi
1408679b7: 4d 89 c6                    	movq	%r8, %r14
1408679ba: 49 89 d7                    	movq	%rdx, %r15
1408679bd: 48 89 ce                    	movq	%rcx, %rsi
1408679c0: 48 8b 9d 50 04 00 00        	movq	0x450(%rbp), %rbx
1408679c7: 4c 8b a5 30 04 00 00        	movq	0x430(%rbp), %r12
1408679ce: 48 8b 8b a0 00 00 00        	movq	0xa0(%rbx), %rcx
1408679d5: e8 66 d1 36 00              	callq	0x140bd4b40 <.text+0xbd3b40>
1408679da: 48 89 9d 88 02 00 00        	movq	%rbx, 0x288(%rbp)
1408679e1: 89 85 90 02 00 00           	movl	%eax, 0x290(%rbp)
1408679e7: 48 c7 85 48 02 00 00 00 00 00 00    	movq	$0x0, 0x248(%rbp)
1408679f2: 48 c7 85 50 02 00 00 04 00 00 00    	movq	$0x4, 0x250(%rbp)
1408679fd: 48 c7 85 58 02 00 00 00 00 00 00    	movq	$0x0, 0x258(%rbp)
140867a08: 4c 89 bd 60 02 00 00        	movq	%r15, 0x260(%rbp)
140867a0f: 4c 89 b5 68 02 00 00        	movq	%r14, 0x268(%rbp)
140867a16: 48 c7 85 70 02 00 00 00 00 00 00    	movq	$0x0, 0x270(%rbp)
140867a21: 89 85 80 02 00 00           	movl	%eax, 0x280(%rbp)
140867a27: 49 c1 e4 05                 	shlq	$0x5, %r12
140867a2b: 49 01 fc                    	addq	%rdi, %r12
140867a2e: 48 89 bd 08 03 00 00        	movq	%rdi, 0x308(%rbp)
140867a35: 4c 89 a5 10 03 00 00        	movq	%r12, 0x310(%rbp)
140867a3c: 48 8d 85 48 02 00 00        	leaq	0x248(%rbp), %rax
140867a43: 48 89 85 18 03 00 00        	movq	%rax, 0x318(%rbp)
140867a4a: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867a51: 48 8d 4d ac                 	leaq	-0x54(%rbp), %rcx
140867a55: 48 8d 95 08 03 00 00        	leaq	0x308(%rbp), %rdx
140867a5c: e8 ef 36 d1 00              	callq	0x14157b150 <.text+0x157a150>
140867a61: 48 8b 85 38 04 00 00        	movq	0x438(%rbp), %rax
140867a68: 44 8b 38                    	movl	(%rax), %r15d
140867a6b: 45 85 ff                    	testl	%r15d, %r15d
140867a6e: 0f 84 54 01 00 00           	je	0x140867bc8 <.text+0x866bc8>
140867a74: 41 83 ff 01                 	cmpl	$0x1, %r15d
140867a78: 0f 85 91 02 00 00           	jne	0x140867d0f <.text+0x866d0f>
140867a7e: 4c 8b 78 08                 	movq	0x8(%rax), %r15
140867a82: 8b 48 10                    	movl	0x10(%rax), %ecx
140867a85: 89 8d 6c 03 00 00           	movl	%ecx, 0x36c(%rbp)
140867a8b: f3 0f 10 70 14              	movss	0x14(%rax), %xmm6
140867a90: 44 8b 70 04                 	movl	0x4(%rax), %r14d
140867a94: 41 8b 3f                    	movl	(%r15), %edi
140867a97: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867a9e: 48 8d 8d 48 02 00 00        	leaq	0x248(%rbp), %rcx
140867aa5: 89 fa                       	movl	%edi, %edx
140867aa7: 41 b0 17                    	movb	$0x17, %r8b
140867aaa: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140867ab0: e8 db 4f 17 00              	callq	0x1409dca90 <.text+0x9dba90>
140867ab5: 41 8b 47 0c                 	movl	0xc(%r15), %eax
140867ab9: 89 85 70 03 00 00           	movl	%eax, 0x370(%rbp)
140867abf: 41 0f b6 47 21              	movzbl	0x21(%r15), %eax
140867ac4: 88 85 76 03 00 00           	movb	%al, 0x376(%rbp)
140867aca: c7 85 08 03 00 00 00 00 00 00       	movl	$0x0, 0x308(%rbp)
140867ad4: c7 85 10 03 00 00 00 00 00 00       	movl	$0x0, 0x310(%rbp)
140867ade: c7 85 18 03 00 00 00 00 00 00       	movl	$0x0, 0x318(%rbp)
140867ae8: c7 85 20 03 00 00 00 00 00 00       	movl	$0x0, 0x320(%rbp)
140867af2: c7 85 28 03 00 00 00 00 00 00       	movl	$0x0, 0x328(%rbp)
140867afc: c7 85 30 03 00 00 00 00 00 00       	movl	$0x0, 0x330(%rbp)
140867b06: 48 c7 85 38 03 00 00 00 00 00 00    	movq	$0x0, 0x338(%rbp)
140867b11: c6 85 40 03 00 00 00        	movb	$0x0, 0x340(%rbp)
140867b18: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867b1f: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140867b26: 48 8d 95 08 03 00 00        	leaq	0x308(%rbp), %rdx
140867b2d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140867b33: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140867b39: e8 e2 dd 36 00              	callq	0x140bd5920 <.text+0xbd4920>
140867b3e: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867b45: 48 8d 8d 08 03 00 00        	leaq	0x308(%rbp), %rcx
140867b4c: 48 8d 95 b0 02 00 00        	leaq	0x2b0(%rbp), %rdx
140867b53: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140867b59: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140867b5f: e8 3c de 36 00              	callq	0x140bd59a0 <.text+0xbd49a0>
140867b64: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867b6b: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140867b72: 48 8d 95 08 03 00 00        	leaq	0x308(%rbp), %rdx
140867b79: 45 89 f0                    	movl	%r14d, %r8d
140867b7c: e8 df dd 36 00              	callq	0x140bd5960 <.text+0xbd4960>
140867b81: 44 8b bd b0 02 00 00        	movl	0x2b0(%rbp), %r15d
140867b88: 0f 10 85 b4 02 00 00        	movups	0x2b4(%rbp), %xmm0
140867b8f: 0f 29 85 10 02 00 00        	movaps	%xmm0, 0x210(%rbp)
140867b96: 0f 10 85 c4 02 00 00        	movups	0x2c4(%rbp), %xmm0
140867b9d: 0f 29 85 20 02 00 00        	movaps	%xmm0, 0x220(%rbp)
140867ba4: 0f 10 85 d4 02 00 00        	movups	0x2d4(%rbp), %xmm0
140867bab: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
140867bb2: 48 8b 85 e4 02 00 00        	movq	0x2e4(%rbp), %rax
140867bb9: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
140867bc0: 45 31 e4                    	xorl	%r12d, %r12d
140867bc3: e9 47 01 00 00              	jmp	0x140867d0f <.text+0x866d0f>
140867bc8: 4c 8b 78 08                 	movq	0x8(%rax), %r15
140867bcc: 8b 48 10                    	movl	0x10(%rax), %ecx
140867bcf: 89 8d 6c 03 00 00           	movl	%ecx, 0x36c(%rbp)
140867bd5: f3 0f 10 70 14              	movss	0x14(%rax), %xmm6
140867bda: 44 8b 70 04                 	movl	0x4(%rax), %r14d
140867bde: 41 8b 3f                    	movl	(%r15), %edi
140867be1: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867be8: 48 8d 8d 48 02 00 00        	leaq	0x248(%rbp), %rcx
140867bef: 89 fa                       	movl	%edi, %edx
140867bf1: 41 b0 2f                    	movb	$0x2f, %r8b
140867bf4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140867bfa: e8 91 4e 17 00              	callq	0x1409dca90 <.text+0x9dba90>
140867bff: 41 8b 47 0c                 	movl	0xc(%r15), %eax
140867c03: 89 85 70 03 00 00           	movl	%eax, 0x370(%rbp)
140867c09: 41 0f b6 47 21              	movzbl	0x21(%r15), %eax
140867c0e: 88 85 76 03 00 00           	movb	%al, 0x376(%rbp)
140867c14: c7 85 08 03 00 00 00 00 00 00       	movl	$0x0, 0x308(%rbp)
140867c1e: c7 85 10 03 00 00 00 00 00 00       	movl	$0x0, 0x310(%rbp)
140867c28: c7 85 18 03 00 00 00 00 00 00       	movl	$0x0, 0x318(%rbp)
140867c32: c7 85 20 03 00 00 00 00 00 00       	movl	$0x0, 0x320(%rbp)
140867c3c: c7 85 28 03 00 00 00 00 00 00       	movl	$0x0, 0x328(%rbp)
140867c46: c7 85 30 03 00 00 00 00 00 00       	movl	$0x0, 0x330(%rbp)
140867c50: 48 c7 85 38 03 00 00 00 00 00 00    	movq	$0x0, 0x338(%rbp)
140867c5b: c6 85 40 03 00 00 00        	movb	$0x0, 0x340(%rbp)
140867c62: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867c69: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140867c70: 48 8d 95 08 03 00 00        	leaq	0x308(%rbp), %rdx
140867c77: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140867c7d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140867c83: e8 98 dc 36 00              	callq	0x140bd5920 <.text+0xbd4920>
140867c88: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867c8f: 48 8d 8d 08 03 00 00        	leaq	0x308(%rbp), %rcx
140867c96: 48 8d 95 b0 02 00 00        	leaq	0x2b0(%rbp), %rdx
140867c9d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140867ca3: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140867ca9: e8 f2 dc 36 00              	callq	0x140bd59a0 <.text+0xbd49a0>
140867cae: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867cb5: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140867cbc: 48 8d 95 08 03 00 00        	leaq	0x308(%rbp), %rdx
140867cc3: 45 89 f0                    	movl	%r14d, %r8d
140867cc6: e8 95 dc 36 00              	callq	0x140bd5960 <.text+0xbd4960>
140867ccb: 44 8b bd b0 02 00 00        	movl	0x2b0(%rbp), %r15d
140867cd2: 0f 10 85 b4 02 00 00        	movups	0x2b4(%rbp), %xmm0
140867cd9: 0f 29 85 10 02 00 00        	movaps	%xmm0, 0x210(%rbp)
140867ce0: 0f 10 85 c4 02 00 00        	movups	0x2c4(%rbp), %xmm0
140867ce7: 0f 29 85 20 02 00 00        	movaps	%xmm0, 0x220(%rbp)
140867cee: 0f 10 85 d4 02 00 00        	movups	0x2d4(%rbp), %xmm0
140867cf5: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
140867cfc: 48 8b 85 e4 02 00 00        	movq	0x2e4(%rbp), %rax
140867d03: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
140867d0a: 41 b4 01                    	movb	$0x1, %r12b
140867d0d: eb 00                       	jmp	0x140867d0f <.text+0x866d0f>
140867d0f: 4c 8b b5 48 04 00 00        	movq	0x448(%rbp), %r14
140867d16: c6 85 77 03 00 00 01        	movb	$0x1, 0x377(%rbp)
140867d1d: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
140867d24: 4c 89 f2                    	movq	%r14, %rdx
140867d27: e8 e4 15 0a 00              	callq	0x140909310 <.text+0x908310>
140867d2c: 0f 10 85 88 02 00 00        	movups	0x288(%rbp), %xmm0
140867d33: 0f 29 85 f0 02 00 00        	movaps	%xmm0, 0x2f0(%rbp)
140867d3a: 0f 10 85 48 02 00 00        	movups	0x248(%rbp), %xmm0
140867d41: 0f 10 8d 58 02 00 00        	movups	0x258(%rbp), %xmm1
140867d48: 0f 10 95 68 02 00 00        	movups	0x268(%rbp), %xmm2
140867d4f: 0f 10 9d 78 02 00 00        	movups	0x278(%rbp), %xmm3
140867d56: 0f 29 9d e0 02 00 00        	movaps	%xmm3, 0x2e0(%rbp)
140867d5d: 0f 29 95 d0 02 00 00        	movaps	%xmm2, 0x2d0(%rbp)
140867d64: 0f 29 8d c0 02 00 00        	movaps	%xmm1, 0x2c0(%rbp)
140867d6b: 0f 29 85 b0 02 00 00        	movaps	%xmm0, 0x2b0(%rbp)
140867d72: 44 8b ad 8c 01 00 00        	movl	0x18c(%rbp), %r13d
140867d79: f3 0f 6f bb 88 01 00 00     	movdqu	0x188(%rbx), %xmm7
140867d81: 48 8d 93 10 01 00 00        	leaq	0x110(%rbx), %rdx
140867d88: 48 8d 8d 08 03 00 00        	leaq	0x308(%rbp), %rcx
140867d8f: e8 7c 8c db 00              	callq	0x141620a10 <.text+0x161fa10>
140867d94: 48 81 c3 70 01 00 00        	addq	$0x170, %rbx            # imm = 0x170
140867d9b: 48 8d 8d c8 01 00 00        	leaq	0x1c8(%rbp), %rcx
140867da2: 48 89 da                    	movq	%rbx, %rdx
140867da5: e8 56 ed 36 00              	callq	0x140bd6b00 <.text+0xbd5b00>
140867daa: 48 c7 85 98 02 00 00 00 00 00 00    	movq	$0x0, 0x298(%rbp)
140867db5: 48 c7 85 a0 02 00 00 08 00 00 00    	movq	$0x8, 0x2a0(%rbp)
140867dc0: 48 c7 85 a8 02 00 00 00 00 00 00    	movq	$0x0, 0x2a8(%rbp)
140867dcb: e8 70 59 4e 00              	callq	0x140d4d740 <.text+0xd4c740>
140867dd0: 48 89 c3                    	movq	%rax, %rbx
140867dd3: f6 00 01                    	testb	$0x1, (%rax)
140867dd6: 74 11                       	je	0x140867de9 <.text+0x866de9>
140867dd8: 48 89 d9                    	movq	%rbx, %rcx
140867ddb: 48 83 c1 08                 	addq	$0x8, %rcx
140867ddf: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140867de3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140867de7: eb 1b                       	jmp	0x140867e04 <.text+0x866e04>
140867de9: e8 12 d0 4f 00              	callq	0x140d64e00 <.text+0xd63e00>
140867dee: 48 89 d9                    	movq	%rbx, %rcx
140867df1: 48 83 c1 08                 	addq	$0x8, %rcx
140867df5: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
140867dfc: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
140867e00: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140867e04: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140867e08: 4c 89 01                    	movq	%r8, (%rcx)
140867e0b: 44 0f 10 05 1d e9 22 02     	movups	0x222e91d(%rip), %xmm8  # 0x142a96730
140867e13: 44 0f 29 85 f0 01 00 00     	movaps	%xmm8, 0x1f0(%rbp)
140867e1b: 44 0f 10 0d fd e8 22 02     	movups	0x222e8fd(%rip), %xmm9  # 0x142a96720
140867e23: 44 0f 29 8d e0 01 00 00     	movaps	%xmm9, 0x1e0(%rbp)
140867e2b: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
140867e32: 48 89 95 08 02 00 00        	movq	%rdx, 0x208(%rbp)
140867e39: e8 02 59 4e 00              	callq	0x140d4d740 <.text+0xd4c740>
140867e3e: 48 89 c3                    	movq	%rax, %rbx
140867e41: f6 00 01                    	testb	$0x1, (%rax)
140867e44: 74 11                       	je	0x140867e57 <.text+0x866e57>
140867e46: 48 89 d9                    	movq	%rbx, %rcx
140867e49: 48 83 c1 08                 	addq	$0x8, %rcx
140867e4d: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140867e51: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140867e55: eb 1b                       	jmp	0x140867e72 <.text+0x866e72>
140867e57: e8 a4 cf 4f 00              	callq	0x140d64e00 <.text+0xd63e00>
140867e5c: 48 89 d9                    	movq	%rbx, %rcx
140867e5f: 48 83 c1 08                 	addq	$0x8, %rcx
140867e63: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
140867e6a: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
140867e6e: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140867e72: 48 8b 9d 40 04 00 00        	movq	0x440(%rbp), %rbx
140867e79: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140867e7d: 4c 89 01                    	movq	%r8, (%rcx)
140867e80: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
140867e87: 48 89 4e 10                 	movq	%rcx, 0x10(%rsi)
140867e8b: 0f 10 85 98 02 00 00        	movups	0x298(%rbp), %xmm0
140867e92: 0f 11 06                    	movups	%xmm0, (%rsi)
140867e95: 0f 28 85 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm0
140867e9c: 0f 28 8d f0 01 00 00        	movaps	0x1f0(%rbp), %xmm1
140867ea3: 0f 28 95 00 02 00 00        	movaps	0x200(%rbp), %xmm2
140867eaa: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
140867eae: 0f 11 4e 40                 	movups	%xmm1, 0x40(%rsi)
140867eb2: 0f 11 56 50                 	movups	%xmm2, 0x50(%rsi)
140867eb6: 48 c7 46 18 00 00 00 00     	movq	$0x0, 0x18(%rsi)
140867ebe: 48 c7 46 20 04 00 00 00     	movq	$0x4, 0x20(%rsi)
140867ec6: 48 c7 46 28 00 00 00 00     	movq	$0x0, 0x28(%rsi)
140867ece: 44 0f 11 46 70              	movups	%xmm8, 0x70(%rsi)
140867ed3: 44 0f 11 4e 60              	movups	%xmm9, 0x60(%rsi)
140867ed8: 48 89 86 80 00 00 00        	movq	%rax, 0x80(%rsi)
140867edf: 48 89 96 88 00 00 00        	movq	%rdx, 0x88(%rsi)
140867ee6: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
140867eed: 0f 28 8d c0 02 00 00        	movaps	0x2c0(%rbp), %xmm1
140867ef4: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
140867efb: 0f 28 9d e0 02 00 00        	movaps	0x2e0(%rbp), %xmm3
140867f02: 0f 11 86 90 00 00 00        	movups	%xmm0, 0x90(%rsi)
140867f09: 0f 11 8e a0 00 00 00        	movups	%xmm1, 0xa0(%rsi)
140867f10: 0f 11 96 b0 00 00 00        	movups	%xmm2, 0xb0(%rsi)
140867f17: 0f 11 9e c0 00 00 00        	movups	%xmm3, 0xc0(%rsi)
140867f1e: 0f 28 85 f0 02 00 00        	movaps	0x2f0(%rbp), %xmm0
140867f25: 0f 11 86 d0 00 00 00        	movups	%xmm0, 0xd0(%rsi)
140867f2c: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
140867f33: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
140867f3a: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
140867f41: 0f 11 86 a8 01 00 00        	movups	%xmm0, 0x1a8(%rsi)
140867f48: 0f 11 8e b8 01 00 00        	movups	%xmm1, 0x1b8(%rsi)
140867f4f: 0f 11 96 c8 01 00 00        	movups	%xmm2, 0x1c8(%rsi)
140867f56: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
140867f5d: 48 89 86 d8 01 00 00        	movq	%rax, 0x1d8(%rsi)
140867f64: 4c 89 b6 f0 01 00 00        	movq	%r14, 0x1f0(%rsi)
140867f6b: 66 0f 70 c7 4e              	pshufd	$0x4e, %xmm7, %xmm0     # xmm0 = xmm7[2,3,0,1]
140867f70: f3 0f 7f 86 e0 01 00 00     	movdqu	%xmm0, 0x1e0(%rsi)
140867f78: 0f 10 85 58 03 00 00        	movups	0x358(%rbp), %xmm0
140867f7f: 0f 11 86 80 01 00 00        	movups	%xmm0, 0x180(%rsi)
140867f86: 0f 10 85 48 03 00 00        	movups	0x348(%rbp), %xmm0
140867f8d: 0f 11 86 70 01 00 00        	movups	%xmm0, 0x170(%rsi)
140867f94: 0f 10 85 08 03 00 00        	movups	0x308(%rbp), %xmm0
140867f9b: 0f 10 8d 18 03 00 00        	movups	0x318(%rbp), %xmm1
140867fa2: 0f 10 95 28 03 00 00        	movups	0x328(%rbp), %xmm2
140867fa9: 0f 10 9d 38 03 00 00        	movups	0x338(%rbp), %xmm3
140867fb0: 0f 11 9e 60 01 00 00        	movups	%xmm3, 0x160(%rsi)
140867fb7: 0f 11 96 50 01 00 00        	movups	%xmm2, 0x150(%rsi)
140867fbe: 0f 11 8e 40 01 00 00        	movups	%xmm1, 0x140(%rsi)
140867fc5: 0f 11 86 30 01 00 00        	movups	%xmm0, 0x130(%rsi)
140867fcc: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
140867fd3: 48 89 86 a0 01 00 00        	movq	%rax, 0x1a0(%rsi)
140867fda: 0f 10 85 c8 01 00 00        	movups	0x1c8(%rbp), %xmm0
140867fe1: 0f 11 86 90 01 00 00        	movups	%xmm0, 0x190(%rsi)
140867fe8: 48 8d 8e 00 02 00 00        	leaq	0x200(%rsi), %rcx
140867fef: 48 8d 55 ac                 	leaq	-0x54(%rbp), %rdx
140867ff3: 41 b8 e0 01 00 00           	movl	$0x1e0, %r8d            # imm = 0x1E0
140867ff9: e8 02 bc 0b 02              	callq	0x142923c00 <.text+0x2922c00>
140867ffe: 44 89 ae e0 03 00 00        	movl	%r13d, 0x3e0(%rsi)
140868005: 44 89 be e0 00 00 00        	movl	%r15d, 0xe0(%rsi)
14086800c: 0f 28 85 10 02 00 00        	movaps	0x210(%rbp), %xmm0
140868013: 0f 28 8d 20 02 00 00        	movaps	0x220(%rbp), %xmm1
14086801a: 0f 28 95 30 02 00 00        	movaps	0x230(%rbp), %xmm2
140868021: 0f 11 86 e4 00 00 00        	movups	%xmm0, 0xe4(%rsi)
140868028: 0f 11 8e f4 00 00 00        	movups	%xmm1, 0xf4(%rsi)
14086802f: 0f 11 96 04 01 00 00        	movups	%xmm2, 0x104(%rsi)
140868036: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
14086803d: 48 89 86 14 01 00 00        	movq	%rax, 0x114(%rsi)
140868044: 8b 85 6c 03 00 00           	movl	0x36c(%rbp), %eax
14086804a: 89 86 1c 01 00 00           	movl	%eax, 0x11c(%rsi)
140868050: f3 0f 11 b6 20 01 00 00     	movss	%xmm6, 0x120(%rsi)
140868058: 89 be 24 01 00 00           	movl	%edi, 0x124(%rsi)
14086805e: 8b 85 70 03 00 00           	movl	0x370(%rbp), %eax
140868064: 89 86 28 01 00 00           	movl	%eax, 0x128(%rsi)
14086806a: 44 88 a6 2c 01 00 00        	movb	%r12b, 0x12c(%rsi)
140868071: 0f b6 85 76 03 00 00        	movzbl	0x376(%rbp), %eax
140868078: 88 86 2d 01 00 00           	movb	%al, 0x12d(%rsi)
14086807e: 48 89 9e f8 01 00 00        	movq	%rbx, 0x1f8(%rsi)
140868085: c6 86 e4 03 00 00 00        	movb	$0x0, 0x3e4(%rsi)
14086808c: 48 89 f0                    	movq	%rsi, %rax
14086808f: 0f 28 b5 80 03 00 00        	movaps	0x380(%rbp), %xmm6
140868096: 0f 28 bd 90 03 00 00        	movaps	0x390(%rbp), %xmm7
14086809d: 44 0f 28 85 a0 03 00 00     	movaps	0x3a0(%rbp), %xmm8
1408680a5: 44 0f 28 8d b0 03 00 00     	movaps	0x3b0(%rbp), %xmm9
1408680ad: 48 81 c4 48 04 00 00        	addq	$0x448, %rsp            # imm = 0x448
1408680b4: 5b                          	popq	%rbx
1408680b5: 5f                          	popq	%rdi
1408680b6: 5e                          	popq	%rsi
1408680b7: 41 5c                       	popq	%r12
1408680b9: 41 5d                       	popq	%r13
1408680bb: 41 5e                       	popq	%r14
1408680bd: 41 5f                       	popq	%r15
1408680bf: 5d                          	popq	%rbp
1408680c0: c3                          	retq
1408680c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1408680d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408680d5: 55                          	pushq	%rbp
1408680d6: 41 57                       	pushq	%r15
1408680d8: 41 56                       	pushq	%r14
1408680da: 41 55                       	pushq	%r13
1408680dc: 41 54                       	pushq	%r12
1408680de: 56                          	pushq	%rsi
1408680df: 57                          	pushq	%rdi
1408680e0: 53                          	pushq	%rbx
1408680e1: 48 83 ec 68                 	subq	$0x68, %rsp
1408680e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408680ec: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1408680f2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1408680f8: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
1408680fe: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140868103: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
14086810a: e8 81 57 01 00              	callq	0x14087d890 <.text+0x87c890>
14086810f: c6 85 77 03 00 00 00        	movb	$0x0, 0x377(%rbp)
140868116: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14086811b: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140868120: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140868126: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14086812c: 48 83 c4 68                 	addq	$0x68, %rsp
140868130: 5b                          	popq	%rbx
140868131: 5f                          	popq	%rdi
140868132: 5e                          	popq	%rsi
140868133: 41 5c                       	popq	%r12
140868135: 41 5d                       	popq	%r13
140868137: 41 5e                       	popq	%r14
140868139: 41 5f                       	popq	%r15
14086813b: 5d                          	popq	%rbp
14086813c: c3                          	retq
14086813d: 0f 1f 00                    	nopl	(%rax)
140868140: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140868145: 55                          	pushq	%rbp
140868146: 41 57                       	pushq	%r15
140868148: 41 56                       	pushq	%r14
14086814a: 41 55                       	pushq	%r13
14086814c: 41 54                       	pushq	%r12
14086814e: 56                          	pushq	%rsi
14086814f: 57                          	pushq	%rdi
140868150: 53                          	pushq	%rbx
140868151: 48 83 ec 68                 	subq	$0x68, %rsp
140868155: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086815c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
140868162: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
140868168: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
14086816e: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140868173: 48 8d 8d 08 03 00 00        	leaq	0x308(%rbp), %rcx
14086817a: e8 d1 99 8f ff              	callq	0x140161b50 <.text+0x160b50>
14086817f: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140868184: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
14086818a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140868190: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
140868196: 48 83 c4 68                 	addq	$0x68, %rsp
14086819a: 5b                          	popq	%rbx
14086819b: 5f                          	popq	%rdi
14086819c: 5e                          	popq	%rsi
14086819d: 41 5c                       	popq	%r12
14086819f: 41 5d                       	popq	%r13
1408681a1: 41 5e                       	popq	%r14
1408681a3: 41 5f                       	popq	%r15
1408681a5: 5d                          	popq	%rbp
1408681a6: c3                          	retq
1408681a7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1408681b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408681b5: 55                          	pushq	%rbp
1408681b6: 41 57                       	pushq	%r15
1408681b8: 41 56                       	pushq	%r14
1408681ba: 41 55                       	pushq	%r13
1408681bc: 41 54                       	pushq	%r12
1408681be: 56                          	pushq	%rsi
1408681bf: 57                          	pushq	%rdi
1408681c0: 53                          	pushq	%rbx
1408681c1: 48 83 ec 68                 	subq	$0x68, %rsp
1408681c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408681cc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1408681d2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1408681d8: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
1408681de: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1408681e3: 48 8d 8d 98 02 00 00        	leaq	0x298(%rbp), %rcx
1408681ea: e8 21 e5 00 00              	callq	0x140876710 <.text+0x875710>
1408681ef: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1408681f4: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1408681f9: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1408681ff: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
140868205: 48 83 c4 68                 	addq	$0x68, %rsp
140868209: 5b                          	popq	%rbx
14086820a: 5f                          	popq	%rdi
14086820b: 5e                          	popq	%rsi
14086820c: 41 5c                       	popq	%r12
14086820e: 41 5d                       	popq	%r13
140868210: 41 5e                       	popq	%r14
140868212: 41 5f                       	popq	%r15
140868214: 5d                          	popq	%rbp
140868215: c3                          	retq
140868216: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140868220: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140868225: 55                          	pushq	%rbp
140868226: 41 57                       	pushq	%r15
140868228: 41 56                       	pushq	%r14
14086822a: 41 55                       	pushq	%r13
14086822c: 41 54                       	pushq	%r12
14086822e: 56                          	pushq	%rsi
14086822f: 57                          	pushq	%rdi
140868230: 53                          	pushq	%rbx
140868231: 48 83 ec 68                 	subq	$0x68, %rsp
140868235: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086823c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
140868242: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
140868248: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
14086824e: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140868253: 48 8d 8d e0 01 00 00        	leaq	0x1e0(%rbp), %rcx
14086825a: e8 11 1a 8e ff              	callq	0x140149c70 <.text+0x148c70>
14086825f: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140868264: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
14086826a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140868270: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
140868276: 48 83 c4 68                 	addq	$0x68, %rsp
14086827a: 5b                          	popq	%rbx
14086827b: 5f                          	popq	%rdi
14086827c: 5e                          	popq	%rsi
14086827d: 41 5c                       	popq	%r12
14086827f: 41 5d                       	popq	%r13
140868281: 41 5e                       	popq	%r14
140868283: 41 5f                       	popq	%r15
140868285: 5d                          	popq	%rbp
140868286: c3                          	retq
140868287: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140868290: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140868295: 55                          	pushq	%rbp
140868296: 41 57                       	pushq	%r15
140868298: 41 56                       	pushq	%r14
14086829a: 41 55                       	pushq	%r13
14086829c: 41 54                       	pushq	%r12
14086829e: 56                          	pushq	%rsi
14086829f: 57                          	pushq	%rdi
1408682a0: 53                          	pushq	%rbx
1408682a1: 48 83 ec 68                 	subq	$0x68, %rsp
1408682a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408682ac: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1408682b2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1408682b8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1408682bd: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1408682c2: 48 8d 8d c8 01 00 00        	leaq	0x1c8(%rbp), %rcx
1408682c9: e8 e2 a1 37 00              	callq	0x140be24b0 <.text+0xbe14b0>
1408682ce: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1408682d3: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
1408682d9: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1408682df: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1408682e5: 48 83 c4 68                 	addq	$0x68, %rsp
1408682e9: 5b                          	popq	%rbx
1408682ea: 5f                          	popq	%rdi
1408682eb: 5e                          	popq	%rsi
1408682ec: 41 5c                       	popq	%r12
1408682ee: 41 5d                       	popq	%r13
1408682f0: 41 5e                       	popq	%r14
1408682f2: 41 5f                       	popq	%r15
1408682f4: 5d                          	popq	%rbp
1408682f5: c3                          	retq
1408682f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140868300: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140868305: 55                          	pushq	%rbp
140868306: 41 57                       	pushq	%r15
140868308: 41 56                       	pushq	%r14
14086830a: 41 55                       	pushq	%r13
14086830c: 41 54                       	pushq	%r12
14086830e: 56                          	pushq	%rsi
14086830f: 57                          	pushq	%rdi
140868310: 53                          	pushq	%rbx
140868311: 48 83 ec 68                 	subq	$0x68, %rsp
140868315: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086831c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
140868322: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
140868328: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14086832d: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140868332: 80 bd 77 03 00 00 00        	cmpb	$0x0, 0x377(%rbp)
140868339: 75 27                       	jne	0x140868362 <.text+0x867362>
14086833b: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140868340: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140868345: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14086834b: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
140868351: 48 83 c4 68                 	addq	$0x68, %rsp
140868355: 5b                          	popq	%rbx
140868356: 5f                          	popq	%rdi
140868357: 5e                          	popq	%rsi
140868358: 41 5c                       	popq	%r12
14086835a: 41 5d                       	popq	%r13
14086835c: 41 5e                       	popq	%r14
14086835e: 41 5f                       	popq	%r15
140868360: 5d                          	popq	%rbp
140868361: c3                          	retq
140868362: 48 8d 8d 48 02 00 00        	leaq	0x248(%rbp), %rcx
140868369: e8 22 55 01 00              	callq	0x14087d890 <.text+0x87c890>
14086836e: eb cb                       	jmp	0x14086833b <.text+0x86733b>
