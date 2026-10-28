
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140968af0: 56                          	pushq	%rsi
140968af1: 48 83 ec 20                 	subq	$0x20, %rsp
140968af5: 48 89 d6                    	movq	%rdx, %rsi
140968af8: 0f b6 01                    	movzbl	(%rcx), %eax
140968afb: 48 8d 0d 76 cc 14 02        	leaq	0x214cc76(%rip), %rcx   # 0x142ab5778
140968b02: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140968b06: 48 01 c8                    	addq	%rcx, %rax
140968b09: ff e0                       	jmpq	*%rax
140968b0b: 48 8d 0d 9e cb 14 02        	leaq	0x214cb9e(%rip), %rcx   # 0x142ab56b0
140968b12: ba 13 00 00 00              	movl	$0x13, %edx
140968b17: eb 36                       	jmp	0x140968b4f <.text+0x967b4f>
140968b19: 48 8d 0d d6 cb 14 02        	leaq	0x214cbd6(%rip), %rcx   # 0x142ab56f6
140968b20: ba 0e 00 00 00              	movl	$0xe, %edx
140968b25: eb 28                       	jmp	0x140968b4f <.text+0x967b4f>
140968b27: 48 8d 0d a5 cb 14 02        	leaq	0x214cba5(%rip), %rcx   # 0x142ab56d3
140968b2e: ba 0f 00 00 00              	movl	$0xf, %edx
140968b33: eb 1a                       	jmp	0x140968b4f <.text+0x967b4f>
140968b35: 48 8d 0d a6 cb 14 02        	leaq	0x214cba6(%rip), %rcx   # 0x142ab56e2
140968b3c: ba 14 00 00 00              	movl	$0x14, %edx
140968b41: eb 0c                       	jmp	0x140968b4f <.text+0x967b4f>
140968b43: 48 8d 0d 79 cb 14 02        	leaq	0x214cb79(%rip), %rcx   # 0x142ab56c3
140968b4a: ba 10 00 00 00              	movl	$0x10, %edx
140968b4f: e8 2c 49 23 00              	callq	0x140b9d480 <.text+0xb9c480>
140968b54: 49 89 d0                    	movq	%rdx, %r8
140968b57: 48 89 f1                    	movq	%rsi, %rcx
140968b5a: 48 89 c2                    	movq	%rax, %rdx
140968b5d: 48 83 c4 20                 	addq	$0x20, %rsp
140968b61: 5e                          	popq	%rsi
140968b62: e9 29 e0 41 00              	jmp	0x140d86b90 <.text+0xd85b90>
140968b67: cc                          	int3
140968b68: cc                          	int3
140968b69: cc                          	int3
140968b6a: cc                          	int3
140968b6b: cc                          	int3
140968b6c: cc                          	int3
140968b6d: cc                          	int3
140968b6e: cc                          	int3
140968b6f: cc                          	int3
