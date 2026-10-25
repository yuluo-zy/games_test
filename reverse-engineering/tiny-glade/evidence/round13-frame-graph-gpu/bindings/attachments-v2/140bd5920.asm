
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bd5920: 48 89 c8                    	movq	%rcx, %rax
140bd5923: 44 89 42 20                 	movl	%r8d, 0x20(%rdx)
140bd5927: 44 89 4a 24                 	movl	%r9d, 0x24(%rdx)
140bd592b: 0f 10 02                    	movups	(%rdx), %xmm0
140bd592e: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140bd5932: 0f 10 52 2c                 	movups	0x2c(%rdx), %xmm2
140bd5936: 0f 11 51 2c                 	movups	%xmm2, 0x2c(%rcx)
140bd593a: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
140bd593e: 0f 11 01                    	movups	%xmm0, (%rcx)
140bd5941: 8b 4a 20                    	movl	0x20(%rdx), %ecx
140bd5944: 89 48 20                    	movl	%ecx, 0x20(%rax)
140bd5947: 8b 4a 24                    	movl	0x24(%rdx), %ecx
140bd594a: 89 48 24                    	movl	%ecx, 0x24(%rax)
140bd594d: 48 8b 4a 28                 	movq	0x28(%rdx), %rcx
140bd5951: 48 89 48 28                 	movq	%rcx, 0x28(%rax)
140bd5955: c3                          	retq
