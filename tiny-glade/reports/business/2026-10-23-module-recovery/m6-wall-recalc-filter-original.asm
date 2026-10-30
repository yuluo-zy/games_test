
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140757cf0: 48 8b bd c0 03 00 00        	movq	0x3c0(%rbp), %rdi
140757cf7: 48 3b bd b0 03 00 00        	cmpq	0x3b0(%rbp), %rdi
140757cfe: 0f 85 5c ff ff ff           	jne	0x140757c60 <.text+0x756c60>
140757d04: c6 85 33 04 00 00 01        	movb	$0x1, 0x433(%rbp)
140757d0b: 48 c7 44 24 20 08 00 00 00  	movq	$0x8, 0x20(%rsp)
140757d14: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140757d1a: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140757d20: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
140757d27: 48 89 fa                    	movq	%rdi, %rdx
140757d2a: e8 b1 00 15 02              	callq	0x1428a7de0 <.text+0x28a6de0>
140757d2f: e9 2c ff ff ff              	jmp	0x140757c60 <.text+0x756c60>
140757d34: 48 8b 95 18 04 00 00        	movq	0x418(%rbp), %rdx
