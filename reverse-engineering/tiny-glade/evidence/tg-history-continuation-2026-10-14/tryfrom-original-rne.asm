
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a84260: 48 89 c8                    	movq	%rcx, %rax
140a84263: 8b 4a 08                    	movl	0x8(%rdx), %ecx
140a84266: 48 8b 12                    	movq	(%rdx), %rdx
140a84269: 4c 8b 42 10                 	movq	0x10(%rdx), %r8
140a8426d: ba 01 00 00 00              	movl	$0x1, %edx
140a84272: 49 83 f8 02                 	cmpq	$0x2, %r8
140a84276: 72 09                       	jb	0x140a84281 <.text+0xa83281>
140a84278: 4c 89 40 08                 	movq	%r8, 0x8(%rax)
140a8427c: 89 48 10                    	movl	%ecx, 0x10(%rax)
140a8427f: 31 d2                       	xorl	%edx, %edx
140a84281: 48 89 10                    	movq	%rdx, (%rax)
140a84284: c3                          	retq
