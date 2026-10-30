
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408ffd30: 56                          	pushq	%rsi
1408ffd31: 57                          	pushq	%rdi
1408ffd32: 48 83 ec 28                 	subq	$0x28, %rsp
1408ffd36: 89 d6                       	movl	%edx, %esi
1408ffd38: 48 89 cf                    	movq	%rcx, %rdi
1408ffd3b: 48 c7 01 00 00 00 00        	movq	$0x0, (%rcx)
1408ffd42: c6 81 78 01 00 00 00        	movb	$0x0, 0x178(%rcx)
1408ffd49: 48 83 c1 28                 	addq	$0x28, %rcx
1408ffd4d: e8 6e b3 03 00              	callq	0x14093b0c0 <.text+0x93a0c0>
1408ffd52: 48 81 c7 48 01 00 00        	addq	$0x148, %rdi            # imm = 0x148
1408ffd59: 48 89 f9                    	movq	%rdi, %rcx
1408ffd5c: 89 f2                       	movl	%esi, %edx
1408ffd5e: 41 89 f0                    	movl	%esi, %r8d
1408ffd61: 48 83 c4 28                 	addq	$0x28, %rsp
1408ffd65: 5f                          	popq	%rdi
1408ffd66: 5e                          	popq	%rsi
1408ffd67: e9 d4 88 9c ff              	jmp	0x1402c8640 <.text+0x2c7640>
1408ffd6c: cc                          	int3
1408ffd6d: cc                          	int3
1408ffd6e: cc                          	int3
1408ffd6f: cc                          	int3
1408ffd70: 41 57                       	pushq	%r15
1408ffd72: 41 56                       	pushq	%r14
1408ffd74: 56                          	pushq	%rsi
1408ffd75: 57                          	pushq	%rdi
1408ffd76: 55                          	pushq	%rbp
1408ffd77: 53                          	pushq	%rbx
1408ffd78: 48 83 ec 28                 	subq	$0x28, %rsp
1408ffd7c: 44 39 c2                    	cmpl	%r8d, %edx
1408ffd7f: 0f 84 d7 00 00 00           	je	0x1408ffe5c <.text+0x8fee5c>
1408ffd85: 44 89 c6                    	movl	%r8d, %esi
1408ffd88: 89 d3                       	movl	%edx, %ebx
1408ffd8a: 48 89 cf                    	movq	%rcx, %rdi
1408ffd8d: 44 89 c5                    	movl	%r8d, %ebp
1408ffd90: 0f 42 ea                    	cmovbl	%edx, %ebp
1408ffd93: 45 89 c6                    	movl	%r8d, %r14d
1408ffd96: 44 0f 47 f2                 	cmoval	%edx, %r14d
1408ffd9a: 89 6c 24 20                 	movl	%ebp, 0x20(%rsp)
1408ffd9e: 44 89 74 24 24              	movl	%r14d, 0x24(%rsp)
1408ffda3: 48 8b 81 80 00 00 00        	movq	0x80(%rcx), %rax
1408ffdaa: 48 83 f8 01                 	cmpq	$0x1, %rax
1408ffdae: 74 3a                       	je	0x1408ffdea <.text+0x8fedea>
1408ffdb0: 48 85 c0                    	testq	%rax, %rax
1408ffdb3: 74 51                       	je	0x1408ffe06 <.text+0x8fee06>
1408ffdb5: 4c 8d 7f 70                 	leaq	0x70(%rdi), %r15
1408ffdb9: 48 8b 8f a8 00 00 00        	movq	0xa8(%rdi), %rcx
1408ffdc0: 48 8b 97 b0 00 00 00        	movq	0xb0(%rdi), %rdx
1408ffdc7: 41 89 e8                    	movl	%ebp, %r8d
1408ffdca: 45 89 f1                    	movl	%r14d, %r9d
1408ffdcd: e8 6e 6a 80 ff              	callq	0x140106840 <.text+0x105840>
1408ffdd2: 4c 8d 44 24 20              	leaq	0x20(%rsp), %r8
1408ffdd7: 4c 89 f9                    	movq	%r15, %rcx
1408ffdda: 48 89 c2                    	movq	%rax, %rdx
1408ffddd: e8 8e f4 24 00              	callq	0x140b4f270 <.text+0xb4e270>
1408ffde2: 48 83 f8 01                 	cmpq	$0x1, %rax
1408ffde6: 74 11                       	je	0x1408ffdf9 <.text+0x8fedf9>
1408ffde8: eb 1c                       	jmp	0x1408ffe06 <.text+0x8fee06>
1408ffdea: 48 8b 47 78                 	movq	0x78(%rdi), %rax
1408ffdee: 3b 68 08                    	cmpl	0x8(%rax), %ebp
1408ffdf1: 75 13                       	jne	0x1408ffe06 <.text+0x8fee06>
1408ffdf3: 44 3b 70 0c                 	cmpl	0xc(%rax), %r14d
1408ffdf7: 75 0d                       	jne	0x1408ffe06 <.text+0x8fee06>
1408ffdf9: 48 83 c4 28                 	addq	$0x28, %rsp
1408ffdfd: 5b                          	popq	%rbx
1408ffdfe: 5d                          	popq	%rbp
1408ffdff: 5f                          	popq	%rdi
1408ffe00: 5e                          	popq	%rsi
1408ffe01: 41 5e                       	popq	%r14
1408ffe03: 41 5f                       	popq	%r15
1408ffe05: c3                          	retq
1408ffe06: 48 8d 4f 28                 	leaq	0x28(%rdi), %rcx
1408ffe0a: 48 c7 07 00 00 00 00        	movq	$0x0, (%rdi)
1408ffe11: c6 87 78 01 00 00 00        	movb	$0x0, 0x178(%rdi)
1408ffe18: 89 da                       	movl	%ebx, %edx
1408ffe1a: 41 89 f0                    	movl	%esi, %r8d
1408ffe1d: e8 6e ae 03 00              	callq	0x14093ac90 <.text+0x939c90>
1408ffe22: 48 81 c7 48 01 00 00        	addq	$0x148, %rdi            # imm = 0x148
1408ffe29: 48 89 f9                    	movq	%rdi, %rcx
1408ffe2c: 89 ea                       	movl	%ebp, %edx
1408ffe2e: 45 89 f0                    	movl	%r14d, %r8d
1408ffe31: e8 0a 88 9c ff              	callq	0x1402c8640 <.text+0x2c7640>
1408ffe36: 48 89 f9                    	movq	%rdi, %rcx
1408ffe39: 89 da                       	movl	%ebx, %edx
1408ffe3b: 41 89 d8                    	movl	%ebx, %r8d
1408ffe3e: e8 fd 87 9c ff              	callq	0x1402c8640 <.text+0x2c7640>
1408ffe43: 48 89 f9                    	movq	%rdi, %rcx
1408ffe46: 89 f2                       	movl	%esi, %edx
1408ffe48: 41 89 f0                    	movl	%esi, %r8d
1408ffe4b: 48 83 c4 28                 	addq	$0x28, %rsp
1408ffe4f: 5b                          	popq	%rbx
1408ffe50: 5d                          	popq	%rbp
1408ffe51: 5f                          	popq	%rdi
1408ffe52: 5e                          	popq	%rsi
1408ffe53: 41 5e                       	popq	%r14
1408ffe55: 41 5f                       	popq	%r15
1408ffe57: e9 e4 87 9c ff              	jmp	0x1402c8640 <.text+0x2c7640>
1408ffe5c: 48 8d 0d cd 9e 1a 02        	leaq	0x21a9ecd(%rip), %rcx   # 0x142aa9d30
1408ffe63: 4c 8d 05 de 9e 1a 02        	leaq	0x21a9ede(%rip), %r8    # 0x142aa9d48
1408ffe6a: ba 18 00 00 00              	movl	$0x18, %edx
1408ffe6f: e8 bc 95 fd 01              	callq	0x1428d9430 <.text+0x28d8430>
