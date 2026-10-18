
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140be6850: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
140be6857: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
140be685c: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
140be6861: 8b 01                       	movl	(%rcx), %eax
140be6863: 83 f8 02                    	cmpl	$0x2, %eax
140be6866: 0f 84 1c 01 00 00           	je	0x140be6988 <.text+0xbe5988>
140be686c: 83 f8 03                    	cmpl	$0x3, %eax
140be686f: 0f 84 8e 00 00 00           	je	0x140be6903 <.text+0xbe5903>
140be6875: 83 f8 04                    	cmpl	$0x4, %eax
140be6878: 0f 85 2a 01 00 00           	jne	0x140be69a8 <.text+0xbe59a8>
140be687e: 48 83 c2 fc                 	addq	$-0x4, %rdx
140be6882: 48 c1 c2 3e                 	rolq	$0x3e, %rdx
140be6886: b9 64 00 00 00              	movl	$0x64, %ecx
140be688b: 48 83 fa 04                 	cmpq	$0x4, %rdx
140be688f: 0f 82 06 01 00 00           	jb	0x140be699b <.text+0xbe599b>
140be6895: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140be689a: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140be689f: 48 8d 05 0a c4 cd 00        	leaq	0xcdc40a(%rip), %rax    # 0x1418c2cb0 <.text+0x18c1cb0>
140be68a6: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140be68ab: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
140be68b0: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140be68b5: 48 8d 05 04 6e 1a 00        	leaq	0x1a6e04(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
140be68bc: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140be68c1: 48 8d 05 a0 d0 f2 01        	leaq	0x1f2d0a0(%rip), %rax   # 0x142b13968
140be68c8: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140be68cd: 48 c7 44 24 60 03 00 00 00  	movq	$0x3, 0x60(%rsp)
140be68d6: 48 c7 44 24 78 00 00 00 00  	movq	$0x0, 0x78(%rsp)
140be68df: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
