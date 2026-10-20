
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a557a0: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
140a557a7: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140a557ab: 66 0f 6f 85 30 02 00 00     	movdqa	0x230(%rbp), %xmm0
140a557b3: 0f 28 8d 40 02 00 00        	movaps	0x240(%rbp), %xmm1
140a557ba: 0f 11 4d d8                 	movups	%xmm1, -0x28(%rbp)
140a557be: f3 0f 7f 45 c8              	movdqu	%xmm0, -0x38(%rbp)
140a557c3: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140a557c7: 48 89 75 f0                 	movq	%rsi, -0x10(%rbp)
140a557cb: c6 85 d6 05 00 00 01        	movb	$0x1, 0x5d6(%rbp)
140a557d2: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
140a557d9: ba 01 00 00 00              	movl	$0x1, %edx
140a557de: e8 4d 45 e8 ff              	callq	0x1408d9d30 <.text+0x8d8d30>
140a557e3: f3 0f 6f 85 a0 02 00 00     	movdqu	0x2a0(%rbp), %xmm0
140a557eb: 48 8b 9d b0 02 00 00        	movq	0x2b0(%rbp), %rbx
140a557f2: 4c 8b b5 b8 02 00 00        	movq	0x2b8(%rbp), %r14
