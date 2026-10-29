
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408fe7b0: 41 56                       	pushq	%r14
1408fe7b2: 56                          	pushq	%rsi
1408fe7b3: 57                          	pushq	%rdi
1408fe7b4: 53                          	pushq	%rbx
1408fe7b5: 48 83 ec 28                 	subq	$0x28, %rsp
1408fe7b9: 44 8b 42 10                 	movl	0x10(%rdx), %r8d
1408fe7bd: 44 89 44 24 24              	movl	%r8d, 0x24(%rsp)
1408fe7c2: 48 8b 71 38                 	movq	0x38(%rcx), %rsi
1408fe7c6: 48 85 f6                    	testq	%rsi, %rsi
1408fe7c9: 74 7b                       	je	0x1408fe846 <.text+0x8fd846>
1408fe7cb: 48 83 fe 01                 	cmpq	$0x1, %rsi
1408fe7cf: 75 0f                       	jne	0x1408fe7e0 <.text+0x8fd7e0>
1408fe7d1: 48 8b 41 30                 	movq	0x30(%rcx), %rax
1408fe7d5: 44 39 40 20                 	cmpl	%r8d, 0x20(%rax)
1408fe7d9: 75 6b                       	jne	0x1408fe846 <.text+0x8fd846>
1408fe7db: 45 31 c9                    	xorl	%r9d, %r9d
1408fe7de: eb 4a                       	jmp	0x1408fe82a <.text+0x8fd82a>
1408fe7e0: 48 8d 79 28                 	leaq	0x28(%rcx), %rdi
1408fe7e4: 48 8b 41 60                 	movq	0x60(%rcx), %rax
1408fe7e8: 4c 8b 49 68                 	movq	0x68(%rcx), %r9
1408fe7ec: 49 89 ce                    	movq	%rcx, %r14
1408fe7ef: 48 89 c1                    	movq	%rax, %rcx
1408fe7f2: 48 89 d3                    	movq	%rdx, %rbx
1408fe7f5: 4c 89 ca                    	movq	%r9, %rdx
1408fe7f8: e8 73 83 80 ff              	callq	0x140106b70 <.text+0x105b70>
1408fe7fd: 4c 8d 44 24 24              	leaq	0x24(%rsp), %r8
1408fe802: 48 89 f9                    	movq	%rdi, %rcx
1408fe805: 48 89 c2                    	movq	%rax, %rdx
1408fe808: e8 53 0b 25 00              	callq	0x140b4f360 <.text+0xb4e360>
1408fe80d: 4c 89 f1                    	movq	%r14, %rcx
1408fe810: 49 89 d1                    	movq	%rdx, %r9
1408fe813: 48 89 da                    	movq	%rbx, %rdx
1408fe816: 48 83 f8 01                 	cmpq	$0x1, %rax
1408fe81a: 75 2a                       	jne	0x1408fe846 <.text+0x8fd846>
1408fe81c: 49 39 f1                    	cmpq	%rsi, %r9
1408fe81f: 73 48                       	jae	0x1408fe869 <.text+0x8fd869>
1408fe821: 48 8b 41 30                 	movq	0x30(%rcx), %rax
1408fe825: 48 85 c0                    	testq	%rax, %rax
1408fe828: 74 1c                       	je	0x1408fe846 <.text+0x8fd846>
1408fe82a: 4f 8d 04 89                 	leaq	(%r9,%r9,4), %r8
1408fe82e: 4a 8b 44 c0 10              	movq	0x10(%rax,%r8,8), %rax
1408fe833: 48 85 c0                    	testq	%rax, %rax
1408fe836: 74 0e                       	je	0x1408fe846 <.text+0x8fd846>
1408fe838: 48 8d 04 c5 f8 ff ff ff     	leaq	-0x8(,%rax,8), %rax
1408fe840: 48 83 f8 08                 	cmpq	$0x8, %rax
1408fe844: 73 19                       	jae	0x1408fe85f <.text+0x8fd85f>
1408fe846: 48 8b 42 10                 	movq	0x10(%rdx), %rax
1408fe84a: 48 89 41 20                 	movq	%rax, 0x20(%rcx)
1408fe84e: 0f 10 02                    	movups	(%rdx), %xmm0
1408fe851: 0f 11 41 10                 	movups	%xmm0, 0x10(%rcx)
1408fe855: 48 83 c4 28                 	addq	$0x28, %rsp
1408fe859: 5b                          	popq	%rbx
1408fe85a: 5f                          	popq	%rdi
1408fe85b: 5e                          	popq	%rsi
1408fe85c: 41 5e                       	popq	%r14
1408fe85e: c3                          	retq
1408fe85f: 48 c7 41 10 03 00 00 00     	movq	$0x3, 0x10(%rcx)
1408fe867: eb ec                       	jmp	0x1408fe855 <.text+0x8fd855>
1408fe869: 4c 8d 05 40 2a 38 02        	leaq	0x2382a40(%rip), %r8    # 0x142c812b0
1408fe870: 4c 89 c9                    	movq	%r9, %rcx
1408fe873: 48 89 f2                    	movq	%rsi, %rdx
1408fe876: e8 9d ac fd 01              	callq	0x1428d9518 <.text+0x28d8518>
1408fe87b: cc                          	int3
1408fe87c: cc                          	int3
1408fe87d: cc                          	int3
1408fe87e: cc                          	int3
1408fe87f: cc                          	int3
1408fe880: 48 c7 41 10 03 00 00 00     	movq	$0x3, 0x10(%rcx)
1408fe888: c3                          	retq
1408fe889: cc                          	int3
1408fe88a: cc                          	int3
1408fe88b: cc                          	int3
1408fe88c: cc                          	int3
1408fe88d: cc                          	int3
1408fe88e: cc                          	int3
1408fe88f: cc                          	int3
1408fe890: 48 89 c8                    	movq	%rcx, %rax
1408fe893: 48 8b 4a 20                 	movq	0x20(%rdx), %rcx
1408fe897: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
1408fe89b: 0f 10 42 10                 	movups	0x10(%rdx), %xmm0
1408fe89f: 0f 11 00                    	movups	%xmm0, (%rax)
1408fe8a2: c3                          	retq
