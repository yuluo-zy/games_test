
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14145f174: 8b 47 20                    	movl	0x20(%rdi), %eax
14145f177: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
14145f17d: 48 8b 47 18                 	movq	0x18(%rdi), %rax
14145f181: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14145f188: 41 b8 19 00 00 00           	movl	$0x19, %r8d
14145f18e: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14145f195: 48 8d 15 64 38 80 01        	leaq	0x1803864(%rip), %rdx   # 0x142c62a00
14145f19c: 4c 8d 8d 78 01 00 00        	leaq	0x178(%rbp), %r9
14145f1a3: e8 b8 41 64 ff              	callq	0x140aa3360 <.text+0xaa2360>
14145f1a8: e9 f3 ef ff ff              	jmp	0x14145e1a0 <.text+0x145d1a0>
