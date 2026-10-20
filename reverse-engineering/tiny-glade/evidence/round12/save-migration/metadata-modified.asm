
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140d59e60: 55                          	pushq	%rbp
140d59e61: 48 89 e5                    	movq	%rsp, %rbp
140d59e64: 48 89 c8                    	movq	%rcx, %rax
140d59e67: 48 8b 4a 30                 	movq	0x30(%rdx), %rcx
140d59e6b: 48 89 48 04                 	movq	%rcx, 0x4(%rax)
140d59e6f: c7 00 00 00 00 00           	movl	$0x0, (%rax)
140d59e75: 5d                          	popq	%rbp
140d59e76: c3                          	retq
