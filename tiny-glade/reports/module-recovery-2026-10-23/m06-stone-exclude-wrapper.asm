
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408ccc20: 56                          	pushq	%rsi
1408ccc21: 48 83 ec 40                 	subq	$0x40, %rsp
1408ccc25: 48 89 ce                    	movq	%rcx, %rsi
1408ccc28: 8b 44 24 70                 	movl	0x70(%rsp), %eax
1408ccc2c: 89 44 24 30                 	movl	%eax, 0x30(%rsp)
1408ccc30: f3 0f 11 5c 24 28           	movss	%xmm3, 0x28(%rsp)
1408ccc36: 45 31 c9                    	xorl	%r9d, %r9d
1408ccc39: e8 42 15 00 00              	callq	0x1408ce180 <.text+0x8cd180>
1408ccc3e: 48 89 f0                    	movq	%rsi, %rax
1408ccc41: 48 83 c4 40                 	addq	$0x40, %rsp
1408ccc45: 5e                          	popq	%rsi
1408ccc46: c3                          	retq
1408ccc47: cc                          	int3
1408ccc48: cc                          	int3
1408ccc49: cc                          	int3
1408ccc4a: cc                          	int3
1408ccc4b: cc                          	int3
1408ccc4c: cc                          	int3
1408ccc4d: cc                          	int3
1408ccc4e: cc                          	int3
1408ccc4f: cc                          	int3
1408ccc50: 56                          	pushq	%rsi
1408ccc51: 48 83 ec 40                 	subq	$0x40, %rsp
1408ccc55: 48 89 ce                    	movq	%rcx, %rsi
1408ccc58: 4c 8b 4c 24 78              	movq	0x78(%rsp), %r9
1408ccc5d: 48 8b 84 24 80 00 00 00     	movq	0x80(%rsp), %rax
1408ccc65: 8b 4c 24 70                 	movl	0x70(%rsp), %ecx
1408ccc69: 89 4c 24 30                 	movl	%ecx, 0x30(%rsp)
1408ccc6d: f3 0f 11 5c 24 28           	movss	%xmm3, 0x28(%rsp)
1408ccc73: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408ccc78: 48 89 f1                    	movq	%rsi, %rcx
1408ccc7b: e8 00 15 00 00              	callq	0x1408ce180 <.text+0x8cd180>
1408ccc80: 48 89 f0                    	movq	%rsi, %rax
1408ccc83: 48 83 c4 40                 	addq	$0x40, %rsp
1408ccc87: 5e                          	popq	%rsi
1408ccc88: c3                          	retq
