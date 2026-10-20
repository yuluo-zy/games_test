
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141a1a0a5: 48 8b 45 50                 	movq	0x50(%rbp), %rax
141a1a0a9: 44 8b 78 1c                 	movl	0x1c(%rax), %r15d
141a1a0ad: 4c 8b 60 10                 	movq	0x10(%rax), %r12
141a1a0b1: 45 89 3c 24                 	movl	%r15d, (%r12)
141a1a0b5: 4c 8b 30                    	movq	(%rax), %r14
141a1a0b8: 48 8b 0b                    	movq	(%rbx), %rcx
141a1a0bb: e8 60 24 ea fe              	callq	0x1408bc520 <.text+0x8bb520>
141a1a0c0: 49 89 d0                    	movq	%rdx, %r8
141a1a0c3: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
141a1a0ca: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
141a1a0ce: 48 8b 3a                    	movq	(%rdx), %rdi
141a1a0d1: 48 8d 15 e0 54 49 01        	leaq	0x14954e0(%rip), %rdx   # 0x142eaf5b8
141a1a0d8: 48 89 54 24 28              	movq	%rdx, 0x28(%rsp)
141a1a0dd: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
141a1a0e2: 48 c7 44 24 30 09 00 00 00  	movq	$0x9, 0x30(%rsp)
141a1a0eb: 4c 89 f1                    	movq	%r14, %rcx
141a1a0ee: 48 89 c2                    	movq	%rax, %rdx
141a1a0f1: 49 89 f9                    	movq	%rdi, %r9
141a1a0f4: e8 77 63 78 00              	callq	0x1421a0470 <.text+0x219f470>
141a1a0f9: 45 89 3c 24                 	movl	%r15d, (%r12)
141a1a0fd: 48 8b 13                    	movq	(%rbx), %rdx
141a1a100: 4c 89 f1                    	movq	%r14, %rcx
141a1a103: 49 89 f8                    	movq	%rdi, %r8
141a1a106: e8 f5 1a 11 ff              	callq	0x140b2bc00 <.text+0xb2ac00>
