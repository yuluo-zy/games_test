
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aa8a60: 55                          	pushq	%rbp
140aa8a61: 41 57                       	pushq	%r15
140aa8a63: 41 56                       	pushq	%r14
140aa8a65: 41 55                       	pushq	%r13
140aa8a67: 41 54                       	pushq	%r12
140aa8a69: 56                          	pushq	%rsi
140aa8a6a: 57                          	pushq	%rdi
140aa8a6b: 53                          	pushq	%rbx
140aa8a6c: 48 81 ec 48 01 00 00        	subq	$0x148, %rsp            # imm = 0x148
140aa8a73: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140aa8a7b: 48 c7 85 c0 00 00 00 fe ff ff ff    	movq	$-0x2, 0xc0(%rbp)
140aa8a86: 44 89 cf                    	movl	%r9d, %edi
140aa8a89: 45 89 c4                    	movl	%r8d, %r12d
140aa8a8c: 49 89 d6                    	movq	%rdx, %r14
140aa8a8f: 48 89 8d a0 00 00 00        	movq	%rcx, 0xa0(%rbp)
140aa8a96: 4c 8b bd 48 01 00 00        	movq	0x148(%rbp), %r15
140aa8a9d: 44 89 45 48                 	movl	%r8d, 0x48(%rbp)
140aa8aa1: 44 89 4d 4c                 	movl	%r9d, 0x4c(%rbp)
140aa8aa5: 4c 8d 6a 20                 	leaq	0x20(%rdx), %r13
140aa8aa9: 48 8d 55 48                 	leaq	0x48(%rbp), %rdx
140aa8aad: 4c 89 e9                    	movq	%r13, %rcx
140aa8ab0: e8 2b 91 b0 ff              	callq	0x1405b1be0 <.text+0x5b0be0>
140aa8ab5: 48 89 c3                    	movq	%rax, %rbx
140aa8ab8: 48 c1 eb 39                 	shrq	$0x39, %rbx
140aa8abc: 4d 8b 0e                    	movq	(%r14), %r9
140aa8abf: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
140aa8ac3: 48 89 ca                    	movq	%rcx, %rdx
140aa8ac6: 48 21 c2                    	andq	%rax, %rdx
140aa8ac9: 66 0f 6e c3                 	movd	%ebx, %xmm0
140aa8acd: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140aa8ad1: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140aa8ad6: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140aa8adb: 45 31 c0                    	xorl	%r8d, %r8d
140aa8ade: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140aa8ae2: 4c 89 ce                    	movq	%r9, %rsi
140aa8ae5: f3 41 0f 6f 14 11           	movdqu	(%r9,%rdx), %xmm2
140aa8aeb: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140aa8aef: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140aa8af3: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
140aa8af8: 45 85 c9                    	testl	%r9d, %r9d
140aa8afb: 74 43                       	je	0x140aa8b40 <.text+0xaa7b40>
140aa8afd: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140aa8b02: 49 01 d2                    	addq	%rdx, %r10
140aa8b05: 49 21 ca                    	andq	%rcx, %r10
140aa8b08: 49 f7 da                    	negq	%r10
140aa8b0b: 4d 69 d2 10 01 00 00        	imulq	$0x110, %r10, %r10      # imm = 0x110
140aa8b12: 46 39 a4 16 f0 fe ff ff     	cmpl	%r12d, -0x110(%rsi,%r10)
140aa8b1a: 75 0d                       	jne	0x140aa8b29 <.text+0xaa7b29>
140aa8b1c: 49 89 f3                    	movq	%rsi, %r11
140aa8b1f: 42 39 bc 16 f4 fe ff ff     	cmpl	%edi, -0x10c(%rsi,%r10)
140aa8b27: 74 3f                       	je	0x140aa8b68 <.text+0xaa7b68>
140aa8b29: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140aa8b2d: 66 45 21 ca                 	andw	%r9w, %r10w
140aa8b31: 45 89 d1                    	movl	%r10d, %r9d
140aa8b34: 75 c7                       	jne	0x140aa8afd <.text+0xaa7afd>
140aa8b36: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140aa8b40: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140aa8b44: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140aa8b49: 45 85 c9                    	testl	%r9d, %r9d
140aa8b4c: 0f 85 8b 03 00 00           	jne	0x140aa8edd <.text+0xaa7edd>
140aa8b52: 4c 01 c2                    	addq	%r8, %rdx
140aa8b55: 48 83 c2 10                 	addq	$0x10, %rdx
140aa8b59: 49 83 c0 10                 	addq	$0x10, %r8
140aa8b5d: 48 21 ca                    	andq	%rcx, %rdx
140aa8b60: 49 89 f1                    	movq	%rsi, %r9
140aa8b63: e9 7a ff ff ff              	jmp	0x140aa8ae2 <.text+0xaa7ae2>
140aa8b68: 4d 01 d3                    	addq	%r10, %r11
140aa8b6b: 4d 89 dd                    	movq	%r11, %r13
140aa8b6e: 48 8b b5 30 01 00 00        	movq	0x130(%rbp), %rsi
140aa8b75: 49 83 7d d0 00              	cmpq	$0x0, -0x30(%r13)
140aa8b7a: 48 8b 9d 38 01 00 00        	movq	0x138(%rbp), %rbx
140aa8b81: 0f 84 fd 04 00 00           	je	0x140aa9084 <.text+0xaa8084>
140aa8b87: 4d 8b 75 d8                 	movq	-0x28(%r13), %r14
140aa8b8b: 4d 85 f6                    	testq	%r14, %r14
140aa8b8e: 0f 84 78 05 00 00           	je	0x140aa910c <.text+0xaa810c>
140aa8b94: 49 8d 4e 18                 	leaq	0x18(%r14), %rcx
140aa8b98: b2 01                       	movb	$0x1, %dl
140aa8b9a: 31 c0                       	xorl	%eax, %eax
140aa8b9c: f0                          	lock
140aa8b9d: 41 0f b0 56 18              	cmpxchgb	%dl, 0x18(%r14)
140aa8ba2: 0f 85 2c 06 00 00           	jne	0x140aa91d4 <.text+0xaa81d4>
140aa8ba8: e8 c3 90 1d 00              	callq	0x140c81c70 <.text+0xc80c70>
140aa8bad: 49 8b 46 10                 	movq	0x10(%r14), %rax
140aa8bb1: 48 85 c0                    	testq	%rax, %rax
140aa8bb4: 0f 94 c1                    	sete	%cl
140aa8bb7: 48 c1 e0 02                 	shlq	$0x2, %rax
140aa8bbb: 49 03 46 08                 	addq	0x8(%r14), %rax
140aa8bbf: 48 83 c0 fc                 	addq	$-0x4, %rax
140aa8bc3: 0f 94 c2                    	sete	%dl
140aa8bc6: 08 ca                       	orb	%cl, %dl
140aa8bc8: 74 12                       	je	0x140aa8bdc <.text+0xaa7bdc>
140aa8bca: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140aa8bce: 48 85 c0                    	testq	%rax, %rax
140aa8bd1: 0f 84 9e 06 00 00           	je	0x140aa9275 <.text+0xaa8275>
140aa8bd7: 8d 48 ff                    	leal	-0x1(%rax), %ecx
140aa8bda: eb 06                       	jmp	0x140aa8be2 <.text+0xaa7be2>
140aa8bdc: 8b 08                       	movl	(%rax), %ecx
140aa8bde: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140aa8be2: 48 8b 53 20                 	movq	0x20(%rbx), %rdx
140aa8be6: f3 0f 6f 43 30              	movdqu	0x30(%rbx), %xmm0
140aa8beb: f3 0f 6f 4b 40              	movdqu	0x40(%rbx), %xmm1
140aa8bf0: f3 0f 6f 53 50              	movdqu	0x50(%rbx), %xmm2
140aa8bf5: 89 4d 40                    	movl	%ecx, 0x40(%rbp)
140aa8bf8: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
140aa8bfc: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140aa8c00: f3 0f 7f 45 00              	movdqu	%xmm0, (%rbp)
140aa8c05: 4c 89 75 10                 	movq	%r14, 0x10(%rbp)
140aa8c09: f3 0f 7f 4d 18              	movdqu	%xmm1, 0x18(%rbp)
140aa8c0e: 48 89 5d 28                 	movq	%rbx, 0x28(%rbp)
140aa8c12: f3 0f 7f 55 30              	movdqu	%xmm2, 0x30(%rbp)
140aa8c17: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140aa8c1b: e8 40 bb 83 00              	callq	0x1412e4760 <.text+0x12e3760>
140aa8c20: 49 8d 8d 40 ff ff ff        	leaq	-0xc0(%r13), %rcx
140aa8c27: 48 89 da                    	movq	%rbx, %rdx
140aa8c2a: 4c 8b 85 40 01 00 00        	movq	0x140(%rbp), %r8
140aa8c31: e8 da f5 ff ff              	callq	0x140aa8210 <.text+0xaa7210>
140aa8c36: 49 83 7d e0 00              	cmpq	$0x0, -0x20(%r13)
140aa8c3b: 0f 84 d7 04 00 00           	je	0x140aa9118 <.text+0xaa8118>
140aa8c41: 49 8b 7d e8                 	movq	-0x18(%r13), %rdi
140aa8c45: 48 85 ff                    	testq	%rdi, %rdi
140aa8c48: 0f 84 4a 05 00 00           	je	0x140aa9198 <.text+0xaa8198>
140aa8c4e: 48 8d 4f 18                 	leaq	0x18(%rdi), %rcx
140aa8c52: b2 01                       	movb	$0x1, %dl
140aa8c54: 31 c0                       	xorl	%eax, %eax
140aa8c56: f0                          	lock
140aa8c57: 0f b0 57 18                 	cmpxchgb	%dl, 0x18(%rdi)
140aa8c5b: 0f 85 89 05 00 00           	jne	0x140aa91ea <.text+0xaa81ea>
140aa8c61: e8 0a 90 1d 00              	callq	0x140c81c70 <.text+0xc80c70>
140aa8c66: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140aa8c6a: 48 85 c0                    	testq	%rax, %rax
140aa8c6d: 0f 94 c1                    	sete	%cl
140aa8c70: 48 c1 e0 02                 	shlq	$0x2, %rax
140aa8c74: 48 03 47 08                 	addq	0x8(%rdi), %rax
140aa8c78: 48 83 c0 fc                 	addq	$-0x4, %rax
140aa8c7c: 0f 94 c2                    	sete	%dl
140aa8c7f: 08 ca                       	orb	%cl, %dl
140aa8c81: 74 12                       	je	0x140aa8c95 <.text+0xaa7c95>
140aa8c83: 49 8b 47 28                 	movq	0x28(%r15), %rax
140aa8c87: 48 85 c0                    	testq	%rax, %rax
140aa8c8a: 0f 84 f1 05 00 00           	je	0x140aa9281 <.text+0xaa8281>
140aa8c90: 8d 48 ff                    	leal	-0x1(%rax), %ecx
140aa8c93: eb 06                       	jmp	0x140aa8c9b <.text+0xaa7c9b>
140aa8c95: 8b 08                       	movl	(%rax), %ecx
140aa8c97: 49 8b 47 28                 	movq	0x28(%r15), %rax
140aa8c9b: 49 8b 57 20                 	movq	0x20(%r15), %rdx
140aa8c9f: f3 41 0f 6f 47 30           	movdqu	0x30(%r15), %xmm0
140aa8ca5: f3 41 0f 6f 4f 40           	movdqu	0x40(%r15), %xmm1
140aa8cab: f3 41 0f 6f 57 50           	movdqu	0x50(%r15), %xmm2
140aa8cb1: 89 8d 98 00 00 00           	movl	%ecx, 0x98(%rbp)
140aa8cb7: 48 89 55 48                 	movq	%rdx, 0x48(%rbp)
140aa8cbb: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140aa8cbf: f3 0f 7f 45 58              	movdqu	%xmm0, 0x58(%rbp)
140aa8cc4: 48 89 7d 68                 	movq	%rdi, 0x68(%rbp)
140aa8cc8: f3 0f 7f 4d 70              	movdqu	%xmm1, 0x70(%rbp)
140aa8ccd: 4c 89 bd 80 00 00 00        	movq	%r15, 0x80(%rbp)
140aa8cd4: f3 0f 7f 95 88 00 00 00     	movdqu	%xmm2, 0x88(%rbp)
140aa8cdc: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140aa8ce0: e8 7b ba 83 00              	callq	0x1412e4760 <.text+0x12e3760>
140aa8ce5: 49 8d 8d f8 fe ff ff        	leaq	-0x108(%r13), %rcx
140aa8cec: 49 8b 95 00 ff ff ff        	movq	-0x100(%r13), %rdx
140aa8cf3: 4d 8b a5 08 ff ff ff        	movq	-0xf8(%r13), %r12
140aa8cfa: 4c 89 ad a8 00 00 00        	movq	%r13, 0xa8(%rbp)
140aa8d01: 49 c7 85 08 ff ff ff 00 00 00 00    	movq	$0x0, -0xf8(%r13)
140aa8d0c: 4a 8d 04 e2                 	leaq	(%rdx,%r12,8), %rax
140aa8d10: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140aa8d14: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
140aa8d1b: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
140aa8d1f: 4c 89 65 e0                 	movq	%r12, -0x20(%rbp)
140aa8d23: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
140aa8d2b: 4d 85 e4                    	testq	%r12, %r12
140aa8d2e: 74 74                       	je	0x140aa8da4 <.text+0xaa7da4>
140aa8d30: 4c 8b 2e                    	movq	(%rsi), %r13
140aa8d33: 4c 8b 76 08                 	movq	0x8(%rsi), %r14
140aa8d37: 48 89 f0                    	movq	%rsi, %rax
140aa8d3a: 48 8b 76 18                 	movq	0x18(%rsi), %rsi
140aa8d3e: 4c 8b 78 10                 	movq	0x10(%rax), %r15
140aa8d42: 48 83 c2 08                 	addq	$0x8, %rdx
140aa8d46: 49 c1 e4 03                 	shlq	$0x3, %r12
140aa8d4a: 48 8d 3d 1f 46 03 02        	leaq	0x203461f(%rip), %rdi   # 0x142add370
140aa8d51: 48 8d 5d a0                 	leaq	-0x60(%rbp), %rbx
140aa8d55: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140aa8d60: 48 8b 42 f8                 	movq	-0x8(%rdx), %rax
140aa8d64: 48 89 c1                    	movq	%rax, %rcx
140aa8d67: 48 c1 e9 20                 	shrq	$0x20, %rcx
140aa8d6b: 74 37                       	je	0x140aa8da4 <.text+0xaa7da4>
140aa8d6d: 48 89 95 b8 00 00 00        	movq	%rdx, 0xb8(%rbp)
140aa8d74: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140aa8d78: 4c 89 6d a0                 	movq	%r13, -0x60(%rbp)
140aa8d7c: 4c 89 75 a8                 	movq	%r14, -0x58(%rbp)
140aa8d80: 4c 89 7d b0                 	movq	%r15, -0x50(%rbp)
140aa8d84: 48 89 75 b8                 	movq	%rsi, -0x48(%rbp)
140aa8d88: 48 89 d9                    	movq	%rbx, %rcx
140aa8d8b: 48 89 fa                    	movq	%rdi, %rdx
140aa8d8e: e8 dd b9 26 00              	callq	0x140d14770 <.text+0xd13770>
140aa8d93: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
140aa8d9a: 48 83 c2 08                 	addq	$0x8, %rdx
140aa8d9e: 49 83 c4 f8                 	addq	$-0x8, %r12
140aa8da2: 75 bc                       	jne	0x140aa8d60 <.text+0xaa7d60>
140aa8da4: 4c 8b b5 a8 00 00 00        	movq	0xa8(%rbp), %r14
140aa8dab: 49 8d 7e a0                 	leaq	-0x60(%r14), %rdi
140aa8daf: 49 c7 86 20 ff ff ff 00 00 00 00    	movq	$0x0, -0xe0(%r14)
140aa8dba: 49 c7 86 38 ff ff ff 00 00 00 00    	movq	$0x0, -0xc8(%r14)
140aa8dc5: 49 83 7e b8 00              	cmpq	$0x0, -0x48(%r14)
140aa8dca: 48 8b 9d a0 00 00 00        	movq	0xa0(%rbp), %rbx
140aa8dd1: 74 3d                       	je	0x140aa8e10 <.text+0xaa7e10>
140aa8dd3: 49 8b 76 a8                 	movq	-0x58(%r14), %rsi
140aa8dd7: 48 85 f6                    	testq	%rsi, %rsi
140aa8dda: 74 0e                       	je	0x140aa8dea <.text+0xaa7dea>
140aa8ddc: 48 8b 0f                    	movq	(%rdi), %rcx
140aa8ddf: 4c 8d 46 11                 	leaq	0x11(%rsi), %r8
140aa8de3: b2 ff                       	movb	$-0x1, %dl
140aa8de5: e8 36 ae e7 01              	callq	0x142923c20 <.text+0x2922c20>
140aa8dea: 49 c7 46 b8 00 00 00 00     	movq	$0x0, -0x48(%r14)
140aa8df2: 48 8d 46 01                 	leaq	0x1(%rsi), %rax
140aa8df6: 48 89 c1                    	movq	%rax, %rcx
140aa8df9: 48 c1 e9 03                 	shrq	$0x3, %rcx
140aa8dfd: 48 83 e0 f8                 	andq	$-0x8, %rax
140aa8e01: 48 29 c8                    	subq	%rcx, %rax
140aa8e04: 48 83 fe 08                 	cmpq	$0x8, %rsi
140aa8e08: 48 0f 42 c6                 	cmovbq	%rsi, %rax
140aa8e0c: 49 89 46 b0                 	movq	%rax, -0x50(%r14)
140aa8e10: 49 8d 86 28 ff ff ff        	leaq	-0xd8(%r14), %rax
140aa8e17: 49 8d 8e 10 ff ff ff        	leaq	-0xf0(%r14), %rcx
140aa8e1e: 49 c7 46 98 00 00 00 00     	movq	$0x0, -0x68(%r14)
140aa8e26: 66 41 c7 46 f8 00 00        	movw	$0x0, -0x8(%r14)
140aa8e2d: 49 c7 46 f0 00 00 00 00     	movq	$0x0, -0x10(%r14)
140aa8e35: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
140aa8e39: 48 89 53 50                 	movq	%rdx, 0x50(%rbx)
140aa8e3d: 0f 10 45 30                 	movups	0x30(%rbp), %xmm0
140aa8e41: 0f 11 43 40                 	movups	%xmm0, 0x40(%rbx)
140aa8e45: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140aa8e49: 0f 10 4d 00                 	movups	(%rbp), %xmm1
140aa8e4d: 0f 10 55 10                 	movups	0x10(%rbp), %xmm2
140aa8e51: 0f 10 5d 20                 	movups	0x20(%rbp), %xmm3
140aa8e55: 0f 11 5b 30                 	movups	%xmm3, 0x30(%rbx)
140aa8e59: 0f 11 53 20                 	movups	%xmm2, 0x20(%rbx)
140aa8e5d: 0f 11 4b 10                 	movups	%xmm1, 0x10(%rbx)
140aa8e61: 0f 11 03                    	movups	%xmm0, (%rbx)
140aa8e64: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
140aa8e68: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
140aa8e6c: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
140aa8e70: 0f 10 5d 78                 	movups	0x78(%rbp), %xmm3
140aa8e74: 0f 11 43 58                 	movups	%xmm0, 0x58(%rbx)
140aa8e78: 0f 11 4b 68                 	movups	%xmm1, 0x68(%rbx)
140aa8e7c: 0f 11 53 78                 	movups	%xmm2, 0x78(%rbx)
140aa8e80: 0f 11 9b 88 00 00 00        	movups	%xmm3, 0x88(%rbx)
140aa8e87: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
140aa8e8e: 0f 11 83 98 00 00 00        	movups	%xmm0, 0x98(%rbx)
140aa8e95: 48 8b 95 98 00 00 00        	movq	0x98(%rbp), %rdx
140aa8e9c: 48 89 93 a8 00 00 00        	movq	%rdx, 0xa8(%rbx)
140aa8ea3: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
140aa8eaa: 48 89 93 b0 00 00 00        	movq	%rdx, 0xb0(%rbx)
140aa8eb1: 48 89 bb b8 00 00 00        	movq	%rdi, 0xb8(%rbx)
140aa8eb8: 48 89 8b c0 00 00 00        	movq	%rcx, 0xc0(%rbx)
140aa8ebf: 48 89 83 c8 00 00 00        	movq	%rax, 0xc8(%rbx)
140aa8ec6: 48 89 d8                    	movq	%rbx, %rax
140aa8ec9: 48 81 c4 48 01 00 00        	addq	$0x148, %rsp            # imm = 0x148
140aa8ed0: 5b                          	popq	%rbx
140aa8ed1: 5f                          	popq	%rdi
140aa8ed2: 5e                          	popq	%rsi
140aa8ed3: 41 5c                       	popq	%r12
140aa8ed5: 41 5d                       	popq	%r13
140aa8ed7: 41 5e                       	popq	%r14
140aa8ed9: 41 5f                       	popq	%r15
140aa8edb: 5d                          	popq	%rbp
140aa8edc: c3                          	retq
140aa8edd: 4c 8b bd 30 01 00 00        	movq	0x130(%rbp), %r15
140aa8ee4: 48 89 c6                    	movq	%rax, %rsi
140aa8ee7: 49 83 7e 10 00              	cmpq	$0x0, 0x10(%r14)
140aa8eec: 0f 84 36 03 00 00           	je	0x140aa9228 <.text+0xaa8228>
140aa8ef2: 31 c9                       	xorl	%ecx, %ecx
140aa8ef4: e8 27 0c 63 ff              	callq	0x1400d9b20 <.text+0xd8b20>
140aa8ef9: 48 85 c0                    	testq	%rax, %rax
140aa8efc: 0f 84 50 03 00 00           	je	0x140aa9252 <.text+0xaa8252>
140aa8f02: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140aa8f06: 48 ff 00                    	incq	(%rax)
140aa8f09: 49 8b 06                    	movq	(%r14), %rax
140aa8f0c: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
140aa8f10: 48 89 f2                    	movq	%rsi, %rdx
140aa8f13: 48 21 ca                    	andq	%rcx, %rdx
140aa8f16: f3 0f 6f 0c 10              	movdqu	(%rax,%rdx), %xmm1
140aa8f1b: 66 44 0f d7 c1              	pmovmskb	%xmm1, %r8d
140aa8f20: 45 85 c0                    	testl	%r8d, %r8d
140aa8f23: 0f 84 d7 02 00 00           	je	0x140aa9200 <.text+0xaa8200>
140aa8f29: 4c 89 fe                    	movq	%r15, %rsi
140aa8f2c: f3 45 0f bc c0              	tzcntl	%r8d, %r8d
140aa8f31: 49 01 d0                    	addq	%rdx, %r8
140aa8f34: 49 21 c8                    	andq	%rcx, %r8
140aa8f37: 42 0f b6 14 00              	movzbl	(%rax,%r8), %edx
140aa8f3c: 84 d2                       	testb	%dl, %dl
140aa8f3e: 4c 8b bd 48 01 00 00        	movq	0x148(%rbp), %r15
140aa8f45: 0f 89 13 03 00 00           	jns	0x140aa925e <.text+0xaa825e>
140aa8f4b: 4d 8d 48 f0                 	leaq	-0x10(%r8), %r9
140aa8f4f: 49 21 c9                    	andq	%rcx, %r9
140aa8f52: 42 88 1c 00                 	movb	%bl, (%rax,%r8)
140aa8f56: 42 88 5c 08 10              	movb	%bl, 0x10(%rax,%r9)
140aa8f5b: 49 f7 d8                    	negq	%r8
140aa8f5e: 49 69 c8 10 01 00 00        	imulq	$0x110, %r8, %rcx       # imm = 0x110
140aa8f65: 80 e2 01                    	andb	$0x1, %dl
140aa8f68: 0f b6 d2                    	movzbl	%dl, %edx
140aa8f6b: 49 29 56 10                 	subq	%rdx, 0x10(%r14)
140aa8f6f: 4c 8d 2c 08                 	leaq	(%rax,%rcx), %r13
140aa8f73: 44 89 a4 08 f0 fe ff ff     	movl	%r12d, -0x110(%rax,%rcx)
140aa8f7b: 89 bc 08 f4 fe ff ff        	movl	%edi, -0x10c(%rax,%rcx)
140aa8f82: 48 c7 84 08 f8 fe ff ff 00 00 00 00 	movq	$0x0, -0x108(%rax,%rcx)
140aa8f8e: 48 c7 84 08 00 ff ff ff 08 00 00 00 	movq	$0x8, -0x100(%rax,%rcx)
140aa8f9a: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140aa8f9e: f3 0f 7f 8c 08 08 ff ff ff  	movdqu	%xmm1, -0xf8(%rax,%rcx)
140aa8fa7: 48 c7 84 08 18 ff ff ff 10 00 00 00 	movq	$0x10, -0xe8(%rax,%rcx)
140aa8fb3: f3 0f 7f 8c 08 20 ff ff ff  	movdqu	%xmm1, -0xe0(%rax,%rcx)
140aa8fbc: 48 c7 84 08 30 ff ff ff 08 00 00 00 	movq	$0x8, -0xd0(%rax,%rcx)
140aa8fc8: f3 0f 7f 8c 08 38 ff ff ff  	movdqu	%xmm1, -0xc8(%rax,%rcx)
140aa8fd1: 48 c7 84 08 48 ff ff ff 10 00 00 00 	movq	$0x10, -0xb8(%rax,%rcx)
140aa8fdd: 48 c7 84 08 50 ff ff ff 00 00 00 00 	movq	$0x0, -0xb0(%rax,%rcx)
140aa8fe9: c7 84 08 58 ff ff ff 00 00 00 00    	movl	$0x0, -0xa8(%rax,%rcx)
140aa8ff4: 48 c7 84 08 60 ff ff ff 00 00 00 00 	movq	$0x0, -0xa0(%rax,%rcx)
140aa9000: 48 c7 84 08 70 ff ff ff 00 00 00 00 	movq	$0x0, -0x90(%rax,%rcx)
140aa900c: c6 44 08 80 00              	movb	$0x0, -0x80(%rax,%rcx)
140aa9011: 48 c7 44 08 88 00 00 00 00  	movq	$0x0, -0x78(%rax,%rcx)
140aa901a: 48 c7 44 08 90 04 00 00 00  	movq	$0x4, -0x70(%rax,%rcx)
140aa9023: 48 c7 44 08 98 00 00 00 00  	movq	$0x0, -0x68(%rax,%rcx)
140aa902c: 0f 10 0d cd 2e 1c 02        	movups	0x21c2ecd(%rip), %xmm1  # 0x142c6bf00
140aa9033: 0f 11 4c 08 a0              	movups	%xmm1, -0x60(%rax,%rcx)
140aa9038: f3 0f 6f 0d d0 2e 1c 02     	movdqu	0x21c2ed0(%rip), %xmm1  # 0x142c6bf10
140aa9040: f3 0f 7f 4c 08 b0           	movdqu	%xmm1, -0x50(%rax,%rcx)
140aa9046: f3 0f 7f 44 08 c0           	movdqu	%xmm0, -0x40(%rax,%rcx)
140aa904c: 48 c7 44 08 d0 00 00 00 00  	movq	$0x0, -0x30(%rax,%rcx)
140aa9055: 48 c7 44 08 e0 00 00 00 00  	movq	$0x0, -0x20(%rax,%rcx)
140aa905e: 48 c7 44 08 f0 00 00 00 00  	movq	$0x0, -0x10(%rax,%rcx)
140aa9067: 66 c7 44 08 f8 00 00        	movw	$0x0, -0x8(%rax,%rcx)
140aa906e: 49 ff 46 18                 	incq	0x18(%r14)
140aa9072: 49 83 7d d0 00              	cmpq	$0x0, -0x30(%r13)
140aa9077: 48 8b 9d 38 01 00 00        	movq	0x138(%rbp), %rbx
140aa907e: 0f 85 03 fb ff ff           	jne	0x140aa8b87 <.text+0xaa7b87>
140aa9084: 0f b6 05 7e bb b8 02        	movzbl	0x2b8bb7e(%rip), %eax   # 0x143634c09
140aa908b: b9 20 00 00 00              	movl	$0x20, %ecx
140aa9090: ba 08 00 00 00              	movl	$0x8, %edx
140aa9095: e8 76 ab b6 ff              	callq	0x140613c10 <.text+0x612c10>
140aa909a: 48 85 c0                    	testq	%rax, %rax
140aa909d: 0f 84 ec 01 00 00           	je	0x140aa928f <.text+0xaa828f>
140aa90a3: 49 89 c6                    	movq	%rax, %r14
140aa90a6: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140aa90ad: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140aa90b5: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140aa90bd: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140aa90c1: 48 8b 4b 60                 	movq	0x60(%rbx), %rcx
140aa90c5: eb 0b                       	jmp	0x140aa90d2 <.text+0xaa80d2>
140aa90c7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140aa90d0: f3 90                       	pause
140aa90d2: 48 8b 41 08                 	movq	0x8(%rcx), %rax
140aa90d6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140aa90e0: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140aa90e4: 74 ea                       	je	0x140aa90d0 <.text+0xaa80d0>
140aa90e6: 48 85 c0                    	testq	%rax, %rax
140aa90e9: 0f 88 ba 00 00 00           	js	0x140aa91a9 <.text+0xaa81a9>
140aa90ef: 48 8d 50 01                 	leaq	0x1(%rax), %rdx
140aa90f3: f0                          	lock
140aa90f4: 48 0f b1 51 08              	cmpxchgq	%rdx, 0x8(%rcx)
140aa90f9: 75 e5                       	jne	0x140aa90e0 <.text+0xaa80e0>
140aa90fb: 49 89 4d d0                 	movq	%rcx, -0x30(%r13)
140aa90ff: 4d 89 75 d8                 	movq	%r14, -0x28(%r13)
140aa9103: 4d 85 f6                    	testq	%r14, %r14
140aa9106: 0f 85 88 fa ff ff           	jne	0x140aa8b94 <.text+0xaa7b94>
140aa910c: 48 8d 0d 65 5a 18 02        	leaq	0x2185a65(%rip), %rcx   # 0x142c2eb78
140aa9113: e8 f8 01 e3 01              	callq	0x1428d9310 <.text+0x28d8310>
140aa9118: 0f b6 05 ea ba b8 02        	movzbl	0x2b8baea(%rip), %eax   # 0x143634c09
140aa911f: b9 20 00 00 00              	movl	$0x20, %ecx
140aa9124: ba 08 00 00 00              	movl	$0x8, %edx
140aa9129: e8 e2 aa b6 ff              	callq	0x140613c10 <.text+0x612c10>
140aa912e: 48 85 c0                    	testq	%rax, %rax
140aa9131: 0f 84 67 01 00 00           	je	0x140aa929e <.text+0xaa829e>
140aa9137: 48 89 c7                    	movq	%rax, %rdi
140aa913a: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140aa9141: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140aa9149: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140aa9151: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140aa9155: 49 8b 4f 60                 	movq	0x60(%r15), %rcx
140aa9159: eb 07                       	jmp	0x140aa9162 <.text+0xaa8162>
140aa915b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140aa9160: f3 90                       	pause
140aa9162: 48 8b 41 08                 	movq	0x8(%rcx), %rax
140aa9166: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140aa9170: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140aa9174: 74 ea                       	je	0x140aa9160 <.text+0xaa8160>
140aa9176: 48 85 c0                    	testq	%rax, %rax
140aa9179: 78 41                       	js	0x140aa91bc <.text+0xaa81bc>
140aa917b: 48 8d 50 01                 	leaq	0x1(%rax), %rdx
140aa917f: f0                          	lock
140aa9180: 48 0f b1 51 08              	cmpxchgq	%rdx, 0x8(%rcx)
140aa9185: 75 e9                       	jne	0x140aa9170 <.text+0xaa8170>
140aa9187: 49 89 4d e0                 	movq	%rcx, -0x20(%r13)
140aa918b: 49 89 7d e8                 	movq	%rdi, -0x18(%r13)
140aa918f: 48 85 ff                    	testq	%rdi, %rdi
140aa9192: 0f 85 b6 fa ff ff           	jne	0x140aa8c4e <.text+0xaa7c4e>
140aa9198: 48 8d 0d d9 59 18 02        	leaq	0x21859d9(%rip), %rcx   # 0x142c2eb78
140aa919f: e8 6c 01 e3 01              	callq	0x1428d9310 <.text+0x28d8310>
140aa91a4: e9 04 01 00 00              	jmp	0x140aa92ad <.text+0xaa82ad>
140aa91a9: 48 8d 0d 90 81 04 02        	leaq	0x2048190(%rip), %rcx   # 0x142af1340
140aa91b0: 48 8d 15 e9 81 04 02        	leaq	0x20481e9(%rip), %rdx   # 0x142af13a0
140aa91b7: e8 44 08 e0 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140aa91bc: 48 8d 0d 7d 81 04 02        	leaq	0x204817d(%rip), %rcx   # 0x142af1340
140aa91c3: 48 8d 15 d6 81 04 02        	leaq	0x20481d6(%rip), %rdx   # 0x142af13a0
140aa91ca: e8 31 08 e0 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140aa91cf: e9 d9 00 00 00              	jmp	0x140aa92ad <.text+0xaa82ad>
140aa91d4: 48 89 cf                    	movq	%rcx, %rdi
140aa91d7: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140aa91dd: e8 2e 40 e2 01              	callq	0x1428cd210 <.text+0x28cc210>
140aa91e2: 48 89 f9                    	movq	%rdi, %rcx
140aa91e5: e9 be f9 ff ff              	jmp	0x140aa8ba8 <.text+0xaa7ba8>
140aa91ea: 48 89 cb                    	movq	%rcx, %rbx
140aa91ed: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140aa91f3: e8 18 40 e2 01              	callq	0x1428cd210 <.text+0x28cc210>
140aa91f8: 48 89 d9                    	movq	%rbx, %rcx
140aa91fb: e9 61 fa ff ff              	jmp	0x140aa8c61 <.text+0xaa7c61>
140aa9200: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140aa9206: 4c 89 fe                    	movq	%r15, %rsi
140aa9209: 4c 01 ca                    	addq	%r9, %rdx
140aa920c: 48 21 ca                    	andq	%rcx, %rdx
140aa920f: f3 0f 6f 0c 10              	movdqu	(%rax,%rdx), %xmm1
140aa9214: 66 44 0f d7 c1              	pmovmskb	%xmm1, %r8d
140aa9219: 49 83 c1 10                 	addq	$0x10, %r9
140aa921d: 45 85 c0                    	testl	%r8d, %r8d
140aa9220: 0f 85 06 fd ff ff           	jne	0x140aa8f2c <.text+0xaa7f2c>
140aa9226: eb e1                       	jmp	0x140aa9209 <.text+0xaa8209>
140aa9228: ba 01 00 00 00              	movl	$0x1, %edx
140aa922d: 4c 89 f1                    	movq	%r14, %rcx
140aa9230: 4d 89 e8                    	movq	%r13, %r8
140aa9233: 41 b1 01                    	movb	$0x1, %r9b
140aa9236: e8 75 d5 e1 01              	callq	0x1428c67b0 <.text+0x28c57b0>
140aa923b: 44 8b 65 48                 	movl	0x48(%rbp), %r12d
140aa923f: 8b 7d 4c                    	movl	0x4c(%rbp), %edi
140aa9242: 31 c9                       	xorl	%ecx, %ecx
140aa9244: e8 d7 08 63 ff              	callq	0x1400d9b20 <.text+0xd8b20>
140aa9249: 48 85 c0                    	testq	%rax, %rax
140aa924c: 0f 85 b0 fc ff ff           	jne	0x140aa8f02 <.text+0xaa7f02>
140aa9252: 48 8d 0d 47 18 19 02        	leaq	0x2191847(%rip), %rcx   # 0x142c3aaa0
140aa9259: e8 72 ee e2 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140aa925e: 66 0f 6f 08                 	movdqa	(%rax), %xmm1
140aa9262: 66 0f d7 d1                 	pmovmskb	%xmm1, %edx
140aa9266: f3 44 0f bc c2              	tzcntl	%edx, %r8d
140aa926b: 42 0f b6 14 00              	movzbl	(%rax,%r8), %edx
140aa9270: e9 d6 fc ff ff              	jmp	0x140aa8f4b <.text+0xaa7f4b>
140aa9275: 48 8d 0d 14 59 18 02        	leaq	0x2185914(%rip), %rcx   # 0x142c2eb90
140aa927c: e8 8f 00 e3 01              	callq	0x1428d9310 <.text+0x28d8310>
140aa9281: 48 8d 0d 08 59 18 02        	leaq	0x2185908(%rip), %rcx   # 0x142c2eb90
140aa9288: e8 83 00 e3 01              	callq	0x1428d9310 <.text+0x28d8310>
140aa928d: eb 1e                       	jmp	0x140aa92ad <.text+0xaa82ad>
140aa928f: b9 08 00 00 00              	movl	$0x8, %ecx
140aa9294: ba 20 00 00 00              	movl	$0x20, %edx
140aa9299: e8 62 fd e2 01              	callq	0x1428d9000 <.text+0x28d8000>
140aa929e: b9 08 00 00 00              	movl	$0x8, %ecx
140aa92a3: ba 20 00 00 00              	movl	$0x20, %edx
140aa92a8: e8 53 fd e2 01              	callq	0x1428d9000 <.text+0x28d8000>
140aa92ad: 0f 0b                       	ud2
140aa92af: 90                          	nop
140aa92b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140aa92b5: 55                          	pushq	%rbp
140aa92b6: 41 57                       	pushq	%r15
140aa92b8: 41 56                       	pushq	%r14
140aa92ba: 41 55                       	pushq	%r13
140aa92bc: 41 54                       	pushq	%r12
140aa92be: 56                          	pushq	%rsi
140aa92bf: 57                          	pushq	%rdi
140aa92c0: 53                          	pushq	%rbx
140aa92c1: 48 83 ec 28                 	subq	$0x28, %rsp
140aa92c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140aa92cc: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140aa92d0: e8 5b 59 b3 ff              	callq	0x1405dec30 <.text+0x5ddc30>
140aa92d5: 90                          	nop
140aa92d6: 48 83 c4 28                 	addq	$0x28, %rsp
140aa92da: 5b                          	popq	%rbx
140aa92db: 5f                          	popq	%rdi
140aa92dc: 5e                          	popq	%rsi
140aa92dd: 41 5c                       	popq	%r12
140aa92df: 41 5d                       	popq	%r13
140aa92e1: 41 5e                       	popq	%r14
140aa92e3: 41 5f                       	popq	%r15
140aa92e5: 5d                          	popq	%rbp
140aa92e6: c3                          	retq
140aa92e7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140aa92f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140aa92f5: 55                          	pushq	%rbp
140aa92f6: 41 57                       	pushq	%r15
140aa92f8: 41 56                       	pushq	%r14
140aa92fa: 41 55                       	pushq	%r13
140aa92fc: 41 54                       	pushq	%r12
140aa92fe: 56                          	pushq	%rsi
140aa92ff: 57                          	pushq	%rdi
140aa9300: 53                          	pushq	%rbx
140aa9301: 48 83 ec 28                 	subq	$0x28, %rsp
140aa9305: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140aa930c: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140aa9310: e8 1b 59 b3 ff              	callq	0x1405dec30 <.text+0x5ddc30>
140aa9315: 90                          	nop
140aa9316: 48 83 c4 28                 	addq	$0x28, %rsp
140aa931a: 5b                          	popq	%rbx
140aa931b: 5f                          	popq	%rdi
140aa931c: 5e                          	popq	%rsi
140aa931d: 41 5c                       	popq	%r12
140aa931f: 41 5d                       	popq	%r13
140aa9321: 41 5e                       	popq	%r14
140aa9323: 41 5f                       	popq	%r15
140aa9325: 5d                          	popq	%rbp
140aa9326: c3                          	retq
140aa9327: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140aa9330: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140aa9335: 55                          	pushq	%rbp
140aa9336: 41 57                       	pushq	%r15
140aa9338: 41 56                       	pushq	%r14
140aa933a: 41 55                       	pushq	%r13
140aa933c: 41 54                       	pushq	%r12
140aa933e: 56                          	pushq	%rsi
140aa933f: 57                          	pushq	%rdi
140aa9340: 53                          	pushq	%rbx
140aa9341: 48 83 ec 28                 	subq	$0x28, %rsp
140aa9345: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140aa934c: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140aa9353: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140aa9357: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
140aa935b: e8 a0 08 dd ff              	callq	0x140879c00 <.text+0x878c00>
140aa9360: 90                          	nop
140aa9361: 48 83 c4 28                 	addq	$0x28, %rsp
140aa9365: 5b                          	popq	%rbx
140aa9366: 5f                          	popq	%rdi
140aa9367: 5e                          	popq	%rsi
140aa9368: 41 5c                       	popq	%r12
140aa936a: 41 5d                       	popq	%r13
140aa936c: 41 5e                       	popq	%r14
140aa936e: 41 5f                       	popq	%r15
140aa9370: 5d                          	popq	%rbp
140aa9371: c3                          	retq
