
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408b0a4a: 48 89 b5 30 0f 00 00        	movq	%rsi, 0xf30(%rbp)
1408b0a51: 48 8b b5 70 0f 00 00        	movq	0xf70(%rbp), %rsi
1408b0a58: f3 0f 6f 46 08              	movdqu	0x8(%rsi), %xmm0
1408b0a5d: f3 0f 7f 85 70 0c 00 00     	movdqu	%xmm0, 0xc70(%rbp)
1408b0a65: 48 8d 85 70 0c 00 00        	leaq	0xc70(%rbp), %rax
1408b0a6c: 48 89 85 68 0d 00 00        	movq	%rax, 0xd68(%rbp)
1408b0a73: 48 8d 05 16 94 85 ff        	leaq	-0x7a6bea(%rip), %rax   # 0x140109e90 <.text+0x108e90>
1408b0a7a: 48 89 85 70 0d 00 00        	movq	%rax, 0xd70(%rbp)
1408b0a81: 48 8d 05 20 62 23 02        	leaq	0x2236220(%rip), %rax   # 0x142ae6ca8
1408b0a88: 48 89 85 90 07 00 00        	movq	%rax, 0x790(%rbp)
1408b0a8f: 48 c7 85 98 07 00 00 01 00 00 00    	movq	$0x1, 0x798(%rbp)
1408b0a9a: 4c 8d a5 68 0d 00 00        	leaq	0xd68(%rbp), %r12
1408b0aa1: 4c 89 a5 a0 07 00 00        	movq	%r12, 0x7a0(%rbp)
1408b0aa8: 48 c7 85 a8 07 00 00 01 00 00 00    	movq	$0x1, 0x7a8(%rbp)
1408b0ab3: 48 c7 85 b0 07 00 00 00 00 00 00    	movq	$0x0, 0x7b0(%rbp)
1408b0abe: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1408b0ac2: 48 8d 95 90 07 00 00        	leaq	0x790(%rbp), %rdx
1408b0ac9: e8 c2 94 4c 00              	callq	0x140d79f90 <.text+0xd78f90>
1408b0ace: 66 0f 6e 76 18              	movd	0x18(%rsi), %xmm6
1408b0ad3: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
1408b0ad7: 48 89 85 60 0f 00 00        	movq	%rax, 0xf60(%rbp)
1408b0ade: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1408b0ae2: 48 89 85 50 0f 00 00        	movq	%rax, 0xf50(%rbp)
1408b0ae9: 48 8b 75 d8                 	movq	-0x28(%rbp), %rsi
1408b0aed: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1408b0af1: 48 89 85 78 0f 00 00        	movq	%rax, 0xf78(%rbp)
1408b0af8: 48 8b 7f 10                 	movq	0x10(%rdi), %rdi
1408b0afc: 49 8d 4f 04                 	leaq	0x4(%r15), %rcx
1408b0b00: c6 85 87 0f 00 00 01        	movb	$0x1, 0xf87(%rbp)
1408b0b07: 48 89 8d e8 0e 00 00        	movq	%rcx, 0xee8(%rbp)
1408b0b0e: e8 7d 4c 32 00              	callq	0x140bd5790 <.text+0xbd4790>
1408b0b13: 66 0f 6f 05 15 c1 1e 02     	movdqa	0x21ec115(%rip), %xmm0  # 0x142a9cc30
1408b0b1b: f3 0f 7f 85 90 07 00 00     	movdqu	%xmm0, 0x790(%rbp)
1408b0b23: c7 85 a0 07 00 00 00 00 80 3f       	movl	$0x3f800000, 0x7a0(%rbp) # imm = 0x3F800000
1408b0b2d: 4c 89 bd a8 07 00 00        	movq	%r15, 0x7a8(%rbp)
1408b0b34: c7 85 68 0d 00 00 02 00 00 00       	movl	$0x2, 0xd68(%rbp)
1408b0b3e: c6 85 87 0f 00 00 01        	movb	$0x1, 0xf87(%rbp)
1408b0b45: 48 89 5c 24 40              	movq	%rbx, 0x40(%rsp)
1408b0b4a: 4c 89 74 24 38              	movq	%r14, 0x38(%rsp)
1408b0b4f: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1408b0b54: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
1408b0b59: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1408b0b62: 48 8d 15 8f c3 1e 02        	leaq	0x21ec38f(%rip), %rdx   # 0x142a9cef8
1408b0b69: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1408b0b6d: 4c 8d 8d 90 07 00 00        	leaq	0x790(%rbp), %r9
1408b0b74: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
1408b0b7a: e8 f1 6d fb ff              	callq	0x140867970 <.text+0x866970>
1408b0b7f: 48 8b 8d 70 0f 00 00        	movq	0xf70(%rbp), %rcx
1408b0b86: 0f b6 81 c0 00 00 00        	movzbl	0xc0(%rcx), %eax
1408b0b8d: c1 e0 06                    	shll	$0x6, %eax
1408b0b90: f3 0f 6f 44 01 08           	movdqu	0x8(%rcx,%rax), %xmm0
1408b0b96: f3 0f 7f 85 70 0c 00 00     	movdqu	%xmm0, 0xc70(%rbp)
1408b0b9e: 48 8d 85 70 0c 00 00        	leaq	0xc70(%rbp), %rax
1408b0ba5: 48 89 85 68 0d 00 00        	movq	%rax, 0xd68(%rbp)
1408b0bac: 48 8d 05 dd 92 85 ff        	leaq	-0x7a6d23(%rip), %rax   # 0x140109e90 <.text+0x108e90>
1408b0bb3: 48 89 85 70 0d 00 00        	movq	%rax, 0xd70(%rbp)
1408b0bba: 48 8d 05 e7 60 23 02        	leaq	0x22360e7(%rip), %rax   # 0x142ae6ca8
1408b0bc1: 48 89 85 90 07 00 00        	movq	%rax, 0x790(%rbp)
1408b0bc8: 48 c7 85 98 07 00 00 01 00 00 00    	movq	$0x1, 0x798(%rbp)
1408b0bd3: 4c 89 a5 a0 07 00 00        	movq	%r12, 0x7a0(%rbp)
1408b0bda: 48 c7 85 a8 07 00 00 01 00 00 00    	movq	$0x1, 0x7a8(%rbp)
1408b0be5: 48 c7 85 b0 07 00 00 00 00 00 00    	movq	$0x0, 0x7b0(%rbp)
1408b0bf0: c6 85 83 0f 00 00 01        	movb	$0x1, 0xf83(%rbp)
1408b0bf7: c6 85 82 0f 00 00 01        	movb	$0x1, 0xf82(%rbp)
1408b0bfe: 48 8d 8d 08 0f 00 00        	leaq	0xf08(%rbp), %rcx
1408b0c05: 48 8d 95 90 07 00 00        	leaq	0x790(%rbp), %rdx
1408b0c0c: e8 7f 93 4c 00              	callq	0x140d79f90 <.text+0xd78f90>
1408b0c11: 48 8b 8d 40 0f 00 00        	movq	0xf40(%rbp), %rcx
