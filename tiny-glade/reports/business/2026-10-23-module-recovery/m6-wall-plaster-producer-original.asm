
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140758110: 4c 89 bd d0 01 00 00        	movq	%r15, 0x1d0(%rbp)
140758117: 48 c7 85 b0 02 00 00 00 00 00 00    	movq	$0x0, 0x2b0(%rbp)
140758122: 48 c7 85 b8 02 00 00 04 00 00 00    	movq	$0x4, 0x2b8(%rbp)
14075812d: 48 c7 85 c0 02 00 00 00 00 00 00    	movq	$0x0, 0x2c0(%rbp)
140758138: 8b 85 9c 02 00 00           	movl	0x29c(%rbp), %eax
14075813e: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
140758145: 89 01                       	movl	%eax, (%rcx)
140758147: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
14075814e: 4c 8b 70 30                 	movq	0x30(%rax), %r14
140758152: 4c 8b 78 40                 	movq	0x40(%rax), %r15
140758156: 4c 3b 70 20                 	cmpq	0x20(%rax), %r14
14075815a: 75 1a                       	jne	0x140758176 <.text+0x757176>
14075815c: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
140758163: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
14075816a: 48 8d 15 57 43 4c 02        	leaq	0x24c4357(%rip), %rdx   # 0x142c1c4c8
140758171: e8 ba a4 e0 ff              	callq	0x140562630 <.text+0x561630>
140758176: 48 8b 95 c8 02 00 00        	movq	0x2c8(%rbp), %rdx
14075817d: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140758181: 4c 89 f1                    	movq	%r14, %rcx
140758184: 48 c1 e1 04                 	shlq	$0x4, %rcx
140758188: 4c 89 3c 08                 	movq	%r15, (%rax,%rcx)
14075818c: 48 89 74 08 08              	movq	%rsi, 0x8(%rax,%rcx)
140758191: 49 ff c6                    	incq	%r14
140758194: 4c 89 72 30                 	movq	%r14, 0x30(%rdx)
140758198: 48 ff 42 40                 	incq	0x40(%rdx)
14075819c: 48 8d 34 1f                 	leaq	(%rdi,%rbx), %rsi
1407581a0: 48 8d 04 1f                 	leaq	(%rdi,%rbx), %rax
1407581a4: 48 05 60 fd ff ff           	addq	$-0x2a0, %rax           # imm = 0xFD60
1407581aa: 48 89 85 20 04 00 00        	movq	%rax, 0x420(%rbp)
1407581b1: 80 7e f6 00                 	cmpb	$0x0, -0xa(%rsi)
1407581b5: 0f 85 b3 11 00 00           	jne	0x14075936e <.text+0x75836e>
1407581bb: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1407581c2: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
1407581c9: 4c 89 e1                    	movq	%r12, %rcx
1407581cc: e8 df a0 33 00              	callq	0x140a922b0 <.text+0xa912b0>
