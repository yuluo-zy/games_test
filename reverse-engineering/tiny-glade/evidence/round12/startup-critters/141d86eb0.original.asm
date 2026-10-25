
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141d86eb0: 41 57                       	pushq	%r15
141d86eb2: 41 56                       	pushq	%r14
141d86eb4: 41 55                       	pushq	%r13
141d86eb6: 41 54                       	pushq	%r12
141d86eb8: 56                          	pushq	%rsi
141d86eb9: 57                          	pushq	%rdi
141d86eba: 55                          	pushq	%rbp
141d86ebb: 53                          	pushq	%rbx
141d86ebc: 48 83 ec 38                 	subq	$0x38, %rsp
141d86ec0: 48 8b 11                    	movq	(%rcx), %rdx
141d86ec3: 48 8b 59 08                 	movq	0x8(%rcx), %rbx
141d86ec7: 4c 8b b2 f8 00 00 00        	movq	0xf8(%rdx), %r14
141d86ece: 48 8b 82 00 01 00 00        	movq	0x100(%rdx), %rax
141d86ed5: 4d 8d 3c 86                 	leaq	(%r14,%rax,4), %r15
141d86ed9: 48 89 54 24 20              	movq	%rdx, 0x20(%rsp)
141d86ede: 80 ba 08 01 00 00 00        	cmpb	$0x0, 0x108(%rdx)
141d86ee5: 74 11                       	je	0x141d86ef8 <.text+0x1d85ef8>
141d86ee7: 45 31 ed                    	xorl	%r13d, %r13d
141d86eea: 48 8d 74 24 28              	leaq	0x28(%rsp), %rsi
141d86eef: 31 ed                       	xorl	%ebp, %ebp
141d86ef1: 45 31 e4                    	xorl	%r12d, %r12d
141d86ef4: 31 ff                       	xorl	%edi, %edi
141d86ef6: eb 44                       	jmp	0x141d86f3c <.text+0x1d85f3c>
141d86ef8: bd 08 00 00 00              	movl	$0x8, %ebp
141d86efd: 45 31 ed                    	xorl	%r13d, %r13d
141d86f00: 31 f6                       	xorl	%esi, %esi
141d86f02: 45 31 e4                    	xorl	%r12d, %r12d
141d86f05: 31 ff                       	xorl	%edi, %edi
141d86f07: e9 3b 01 00 00              	jmp	0x141d87047 <.text+0x1d86047>
141d86f0c: 31 ed                       	xorl	%ebp, %ebp
141d86f0e: 31 ff                       	xorl	%edi, %edi
141d86f10: 89 f8                       	movl	%edi, %eax
141d86f12: 48 c1 e0 04                 	shlq	$0x4, %rax
141d86f16: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141d86f1a: 48 8d 0c 28                 	leaq	(%rax,%rbp), %rcx
141d86f1e: 48 ff c7                    	incq	%rdi
141d86f21: 41 8b 54 05 18              	movl	0x18(%r13,%rax), %edx
141d86f26: 89 54 24 30                 	movl	%edx, 0x30(%rsp)
141d86f2a: 49 8b 44 05 10              	movq	0x10(%r13,%rax), %rax
141d86f2f: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
141d86f34: 48 89 f2                    	movq	%rsi, %rdx
141d86f37: e8 64 ec 1f ff              	callq	0x140f85ba0 <.text+0xf84ba0>
141d86f3c: 4c 39 e7                    	cmpq	%r12, %rdi
141d86f3f: 75 cf                       	jne	0x141d86f10 <.text+0x1d85f10>
141d86f41: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141d86f50: 4d 39 fe                    	cmpq	%r15, %r14
141d86f53: 0f 84 c8 01 00 00           	je	0x141d87121 <.text+0x1d86121>
141d86f59: 41 8b 0e                    	movl	(%r14), %ecx
141d86f5c: 49 83 c6 04                 	addq	$0x4, %r14
141d86f60: 48 8b 83 a8 01 00 00        	movq	0x1a8(%rbx), %rax
141d86f67: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
141d86f6b: 4c 8b 64 c8 10              	movq	0x10(%rax,%rcx,8), %r12
141d86f70: 4d 85 e4                    	testq	%r12, %r12
141d86f73: 74 db                       	je	0x141d86f50 <.text+0x1d85f50>
141d86f75: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
141d86f79: 48 8b 4c 24 20              	movq	0x20(%rsp), %rcx
141d86f7e: 48 8b 91 10 01 00 00        	movq	0x110(%rcx), %rdx
141d86f85: 48 8b 48 40                 	movq	0x40(%rax), %rcx
141d86f89: 48 39 d1                    	cmpq	%rdx, %rcx
141d86f8c: 76 38                       	jbe	0x141d86fc6 <.text+0x1d85fc6>
141d86f8e: 4c 8b 40 38                 	movq	0x38(%rax), %r8
141d86f92: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
141d86f96: 48 85 d2                    	testq	%rdx, %rdx
141d86f99: 74 2b                       	je	0x141d86fc6 <.text+0x1d85fc6>
141d86f9b: 4c 8b 40 18                 	movq	0x18(%rax), %r8
141d86f9f: 48 f7 d2                    	notq	%rdx
141d86fa2: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
141d86fa6: 48 c1 e2 04                 	shlq	$0x4, %rdx
141d86faa: 4d 8b 6c 10 10              	movq	0x10(%r8,%rdx), %r13
141d86faf: 48 8b 54 24 20              	movq	0x20(%rsp), %rdx
141d86fb4: 48 8b 92 18 01 00 00        	movq	0x118(%rdx), %rdx
141d86fbb: 48 39 d1                    	cmpq	%rdx, %rcx
141d86fbe: 0f 86 48 ff ff ff           	jbe	0x141d86f0c <.text+0x1d85f0c>
141d86fc4: eb 18                       	jmp	0x141d86fde <.text+0x1d85fde>
141d86fc6: 45 31 ed                    	xorl	%r13d, %r13d
141d86fc9: 48 8b 54 24 20              	movq	0x20(%rsp), %rdx
141d86fce: 48 8b 92 18 01 00 00        	movq	0x118(%rdx), %rdx
141d86fd5: 48 39 d1                    	cmpq	%rdx, %rcx
141d86fd8: 0f 86 2e ff ff ff           	jbe	0x141d86f0c <.text+0x1d85f0c>
141d86fde: 48 8b 48 38                 	movq	0x38(%rax), %rcx
141d86fe2: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
141d86fe6: 48 85 c9                    	testq	%rcx, %rcx
141d86fe9: 0f 84 1d ff ff ff           	je	0x141d86f0c <.text+0x1d85f0c>
141d86fef: 48 8b 40 18                 	movq	0x18(%rax), %rax
141d86ff3: 48 f7 d1                    	notq	%rcx
141d86ff6: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
141d86ffa: 48 c1 e1 04                 	shlq	$0x4, %rcx
141d86ffe: 48 8b 6c 08 10              	movq	0x10(%rax,%rcx), %rbp
141d87003: e9 06 ff ff ff              	jmp	0x141d86f0e <.text+0x1d85f0e>
141d87008: 31 f6                       	xorl	%esi, %esi
141d8700a: 48 8b 68 50                 	movq	0x50(%rax), %rbp
141d8700e: 31 ff                       	xorl	%edi, %edi
141d87010: 48 89 f8                    	movq	%rdi, %rax
141d87013: 48 c1 e0 04                 	shlq	$0x4, %rax
141d87017: 8b 44 05 08                 	movl	0x8(%rbp,%rax), %eax
141d8701b: 48 c1 e0 04                 	shlq	$0x4, %rax
141d8701f: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141d87023: 48 8d 0c 06                 	leaq	(%rsi,%rax), %rcx
141d87027: 48 ff c7                    	incq	%rdi
141d8702a: 41 8b 54 05 18              	movl	0x18(%r13,%rax), %edx
141d8702f: 89 54 24 30                 	movl	%edx, 0x30(%rsp)
141d87033: 49 8b 44 05 10              	movq	0x10(%r13,%rax), %rax
141d87038: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
141d8703d: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
141d87042: e8 59 eb 1f ff              	callq	0x140f85ba0 <.text+0xf84ba0>
141d87047: 4c 39 e7                    	cmpq	%r12, %rdi
141d8704a: 75 c4                       	jne	0x141d87010 <.text+0x1d86010>
141d8704c: 0f 1f 40 00                 	nopl	(%rax)
141d87050: 4d 39 fe                    	cmpq	%r15, %r14
141d87053: 0f 84 c8 00 00 00           	je	0x141d87121 <.text+0x1d86121>
141d87059: 41 8b 0e                    	movl	(%r14), %ecx
141d8705c: 49 83 c6 04                 	addq	$0x4, %r14
141d87060: 48 8b 83 00 01 00 00        	movq	0x100(%rbx), %rax
141d87067: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
141d8706b: 48 c1 e1 05                 	shlq	$0x5, %rcx
141d8706f: 4c 8b 64 08 58              	movq	0x58(%rax,%rcx), %r12
141d87074: 4d 85 e4                    	testq	%r12, %r12
141d87077: 74 d7                       	je	0x141d87050 <.text+0x1d86050>
141d87079: 48 01 c8                    	addq	%rcx, %rax
141d8707c: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
141d87082: 48 8b 93 a8 01 00 00        	movq	0x1a8(%rbx), %rdx
141d87089: 4c 8d 0c c9                 	leaq	(%rcx,%rcx,8), %r9
141d8708d: 4a 8d 0c ca                 	leaq	(%rdx,%r9,8), %rcx
141d87091: 4c 8b 44 24 20              	movq	0x20(%rsp), %r8
141d87096: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
141d8709d: 4a 8b 54 ca 40              	movq	0x40(%rdx,%r9,8), %rdx
141d870a2: 4c 39 c2                    	cmpq	%r8, %rdx
141d870a5: 76 38                       	jbe	0x141d870df <.text+0x1d860df>
141d870a7: 4c 8b 49 38                 	movq	0x38(%rcx), %r9
141d870ab: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
141d870af: 4d 85 c0                    	testq	%r8, %r8
141d870b2: 74 2b                       	je	0x141d870df <.text+0x1d860df>
141d870b4: 4c 8b 49 18                 	movq	0x18(%rcx), %r9
141d870b8: 49 f7 d0                    	notq	%r8
141d870bb: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
141d870bf: 49 c1 e0 04                 	shlq	$0x4, %r8
141d870c3: 4f 8b 6c 01 10              	movq	0x10(%r9,%r8), %r13
141d870c8: 4c 8b 44 24 20              	movq	0x20(%rsp), %r8
141d870cd: 4d 8b 80 18 01 00 00        	movq	0x118(%r8), %r8
141d870d4: 4c 39 c2                    	cmpq	%r8, %rdx
141d870d7: 0f 86 2b ff ff ff           	jbe	0x141d87008 <.text+0x1d86008>
141d870dd: eb 18                       	jmp	0x141d870f7 <.text+0x1d860f7>
141d870df: 45 31 ed                    	xorl	%r13d, %r13d
141d870e2: 4c 8b 44 24 20              	movq	0x20(%rsp), %r8
141d870e7: 4d 8b 80 18 01 00 00        	movq	0x118(%r8), %r8
141d870ee: 4c 39 c2                    	cmpq	%r8, %rdx
141d870f1: 0f 86 11 ff ff ff           	jbe	0x141d87008 <.text+0x1d86008>
141d870f7: 48 8b 51 38                 	movq	0x38(%rcx), %rdx
141d870fb: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
141d870ff: 48 85 d2                    	testq	%rdx, %rdx
141d87102: 0f 84 00 ff ff ff           	je	0x141d87008 <.text+0x1d86008>
141d87108: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
141d8710c: 48 f7 d2                    	notq	%rdx
141d8710f: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
141d87113: 48 c1 e2 04                 	shlq	$0x4, %rdx
141d87117: 48 8b 74 11 10              	movq	0x10(%rcx,%rdx), %rsi
141d8711c: e9 e9 fe ff ff              	jmp	0x141d8700a <.text+0x1d8600a>
141d87121: 48 83 c4 38                 	addq	$0x38, %rsp
141d87125: 5b                          	popq	%rbx
141d87126: 5d                          	popq	%rbp
141d87127: 5f                          	popq	%rdi
141d87128: 5e                          	popq	%rsi
141d87129: 41 5c                       	popq	%r12
141d8712b: 41 5d                       	popq	%r13
141d8712d: 41 5e                       	popq	%r14
141d8712f: 41 5f                       	popq	%r15
141d87131: c3                          	retq
