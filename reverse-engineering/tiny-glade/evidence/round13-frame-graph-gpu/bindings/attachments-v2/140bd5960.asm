
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bd5960: 48 89 c8                    	movq	%rcx, %rax
140bd5963: 44 89 42 34                 	movl	%r8d, 0x34(%rdx)
140bd5967: 0f 10 02                    	movups	(%rdx), %xmm0
140bd596a: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140bd596e: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
140bd5972: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
140bd5976: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
140bd597a: 0f 11 01                    	movups	%xmm0, (%rcx)
140bd597d: 48 8b 4a 2c                 	movq	0x2c(%rdx), %rcx
140bd5981: 48 89 48 2c                 	movq	%rcx, 0x2c(%rax)
140bd5985: 8b 4a 34                    	movl	0x34(%rdx), %ecx
140bd5988: 89 48 34                    	movl	%ecx, 0x34(%rax)
140bd598b: 8b 4a 38                    	movl	0x38(%rdx), %ecx
140bd598e: 89 48 38                    	movl	%ecx, 0x38(%rax)
140bd5991: c3                          	retq
