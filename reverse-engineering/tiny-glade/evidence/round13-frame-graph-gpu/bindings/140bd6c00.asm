
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bd6c00: 48 89 c8                    	movq	%rcx, %rax
140bd6c03: 8b 0a                       	movl	(%rdx), %ecx
140bd6c05: 41 0f 10 00                 	movups	(%r8), %xmm0
140bd6c09: 41 0f 10 48 10              	movups	0x10(%r8), %xmm1
140bd6c0e: 41 0f 10 50 20              	movups	0x20(%r8), %xmm2
140bd6c13: 41 0f 10 58 2c              	movups	0x2c(%r8), %xmm3
140bd6c18: 0f 11 58 2c                 	movups	%xmm3, 0x2c(%rax)
140bd6c1c: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140bd6c20: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140bd6c24: 0f 11 00                    	movups	%xmm0, (%rax)
140bd6c27: 89 48 3c                    	movl	%ecx, 0x3c(%rax)
140bd6c2a: c6 40 40 00                 	movb	$0x0, 0x40(%rax)
140bd6c2e: c3                          	retq
