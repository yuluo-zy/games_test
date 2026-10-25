D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bd4b40: 48 83 ec 38                 	subq	$0x38, %rsp
140bd4b44: 48 89 c8                    	movq	%rcx, %rax
140bd4b47: 48 c1 e8 20                 	shrq	$0x20, %rax
140bd4b4b: 75 07                       	jne	0x140bd4b54 <.text+0xbd3b54>
140bd4b4d: 89 c8                       	movl	%ecx, %eax
140bd4b4f: 48 83 c4 38                 	addq	$0x38, %rsp
140bd4b53: c3                          	retq
140bd4b54: 48 8d 05 dd ab f3 01        	leaq	0x1f3abdd(%rip), %rax   # 0x142b0f738
140bd4b5b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140bd4b60: 48 8d 0d 41 aa f3 01        	leaq	0x1f3aa41(%rip), %rcx   # 0x142b0f5a8
140bd4b67: 4c 8d 0d 1a aa f3 01        	leaq	0x1f3aa1a(%rip), %r9    # 0x142b0f588
140bd4b6e: 4c 8d 44 24 37              	leaq	0x37(%rsp), %r8
140bd4b73: ba 2b 00 00 00              	movl	$0x2b, %edx
140bd4b78: e8 e3 4b d0 01              	callq	0x1428d9760 <.text+0x28d8760>
140bd4b7d: cc                          	int3
140bd4b7e: cc                          	int3
140bd4b7f: cc                          	int3
140bd4b80: 48 83 ec 38                 	subq	$0x38, %rsp
140bd4b84: 48 89 c8                    	movq	%rcx, %rax
140bd4b87: 48 c1 e8 20                 	shrq	$0x20, %rax
140bd4b8b: 75 07                       	jne	0x140bd4b94 <.text+0xbd3b94>
140bd4b8d: 89 c8                       	movl	%ecx, %eax
140bd4b8f: 48 83 c4 38                 	addq	$0x38, %rsp
140bd4b93: c3                          	retq
140bd4b94: 48 8d 05 b5 ab f3 01        	leaq	0x1f3abb5(%rip), %rax   # 0x142b0f750
140bd4b9b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140bd4ba0: 48 8d 0d 01 aa f3 01        	leaq	0x1f3aa01(%rip), %rcx   # 0x142b0f5a8
140bd4ba7: 4c 8d 0d da a9 f3 01        	leaq	0x1f3a9da(%rip), %r9    # 0x142b0f588
140bd4bae: 4c 8d 44 24 37              	leaq	0x37(%rsp), %r8
140bd4bb3: ba 2b 00 00 00              	movl	$0x2b, %edx
140bd4bb8: e8 a3 4b d0 01              	callq	0x1428d9760 <.text+0x28d8760>
140bd4bbd: cc                          	int3
140bd4bbe: cc                          	int3
140bd4bbf: cc                          	int3
