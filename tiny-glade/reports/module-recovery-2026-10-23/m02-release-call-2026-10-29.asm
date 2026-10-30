
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aa02c7: 44 8b 61 1c                 	movl	0x1c(%rcx), %r12d
140aa02cb: 4c 8b 69 10                 	movq	0x10(%rcx), %r13
140aa02cf: 45 89 65 00                 	movl	%r12d, (%r13)
140aa02d3: 48 8b 31                    	movq	(%rcx), %rsi
140aa02d6: 48 89 f1                    	movq	%rsi, %rcx
140aa02d9: e8 22 46 e8 ff              	callq	0x140924900 <.text+0x923900>
140aa02de: 48 85 c0                    	testq	%rax, %rax
140aa02e1: 0f 84 00 04 00 00           	je	0x140aa06e7 <.text+0xa9f6e7>
