
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bd59a0: 48 89 c8                    	movq	%rcx, %rax
140bd59a3: 44 89 42 28                 	movl	%r8d, 0x28(%rdx)
140bd59a7: 44 89 4a 2c                 	movl	%r9d, 0x2c(%rdx)
140bd59ab: 0f 10 02                    	movups	(%rdx), %xmm0
140bd59ae: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140bd59b2: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
140bd59b6: 0f 11 01                    	movups	%xmm0, (%rcx)
140bd59b9: 8b 4a 2c                    	movl	0x2c(%rdx), %ecx
140bd59bc: 89 48 2c                    	movl	%ecx, 0x2c(%rax)
140bd59bf: 48 8b 4a 30                 	movq	0x30(%rdx), %rcx
140bd59c3: 48 89 48 30                 	movq	%rcx, 0x30(%rax)
140bd59c7: 8b 4a 38                    	movl	0x38(%rdx), %ecx
140bd59ca: 89 48 38                    	movl	%ecx, 0x38(%rax)
140bd59cd: 48 8b 4a 20                 	movq	0x20(%rdx), %rcx
140bd59d1: 48 89 48 20                 	movq	%rcx, 0x20(%rax)
140bd59d5: 8b 4a 28                    	movl	0x28(%rdx), %ecx
140bd59d8: 89 48 28                    	movl	%ecx, 0x28(%rax)
140bd59db: 8b 4a 2c                    	movl	0x2c(%rdx), %ecx
140bd59de: 89 48 2c                    	movl	%ecx, 0x2c(%rax)
140bd59e1: c3                          	retq
