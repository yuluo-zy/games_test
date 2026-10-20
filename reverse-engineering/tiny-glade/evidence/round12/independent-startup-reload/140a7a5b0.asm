
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a7a5b0: 55                          	pushq	%rbp
140a7a5b1: 41 57                       	pushq	%r15
140a7a5b3: 41 56                       	pushq	%r14
140a7a5b5: 41 55                       	pushq	%r13
140a7a5b7: 41 54                       	pushq	%r12
140a7a5b9: 56                          	pushq	%rsi
140a7a5ba: 57                          	pushq	%rdi
140a7a5bb: 53                          	pushq	%rbx
140a7a5bc: 48 81 ec f8 00 00 00        	subq	$0xf8, %rsp
140a7a5c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a7a5cb: 48 c7 45 70 fe ff ff ff     	movq	$-0x2, 0x70(%rbp)
140a7a5d3: 4c 89 ce                    	movq	%r9, %rsi
140a7a5d6: 4c 89 c7                    	movq	%r8, %rdi
140a7a5d9: 48 89 d3                    	movq	%rdx, %rbx
140a7a5dc: 4c 8b bd f8 00 00 00        	movq	0xf8(%rbp), %r15
140a7a5e3: 4c 8b a5 e8 00 00 00        	movq	0xe8(%rbp), %r12
140a7a5ea: 4c 8b ad e0 00 00 00        	movq	0xe0(%rbp), %r13
140a7a5f1: 8b 41 1c                    	movl	0x1c(%rcx), %eax
140a7a5f4: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
140a7a5f8: 89 02                       	movl	%eax, (%rdx)
140a7a5fa: 4c 8b 31                    	movq	(%rcx), %r14
140a7a5fd: 4c 89 f1                    	movq	%r14, %rcx
140a7a600: e8 2b f7 0f 00              	callq	0x140b79d30 <.text+0xb78d30>
140a7a605: 49 83 c6 30                 	addq	$0x30, %r14
140a7a609: 4c 89 f1                    	movq	%r14, %rcx
140a7a60c: e8 bf f4 0f 00              	callq	0x140b79ad0 <.text+0xb78ad0>
140a7a611: 8b 43 1c                    	movl	0x1c(%rbx), %eax
140a7a614: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
140a7a618: 89 01                       	movl	%eax, (%rcx)
140a7a61a: 48 8b 03                    	movq	(%rbx), %rax
140a7a61d: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140a7a621: 48 c7 45 b8 01 00 00 00     	movq	$0x1, -0x48(%rbp)
140a7a629: 48 8d 5d 20                 	leaq	0x20(%rbp), %rbx
140a7a62d: 4c 8d 75 b0                 	leaq	-0x50(%rbp), %r14
140a7a631: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140a7a640: 48 89 d9                    	movq	%rbx, %rcx
140a7a643: 4c 89 f2                    	movq	%r14, %rdx
140a7a646: e8 e5 ec ad 00              	callq	0x141559330 <.text+0x1558330>
140a7a64b: 83 7d 20 00                 	cmpl	$0x0, 0x20(%rbp)
140a7a64f: 75 ef                       	jne	0x140a7a640 <.text+0xa79640>
140a7a651: 8b 47 1c                    	movl	0x1c(%rdi), %eax
140a7a654: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
140a7a658: 89 01                       	movl	%eax, (%rcx)
140a7a65a: 48 8b 3f                    	movq	(%rdi), %rdi
140a7a65d: 48 89 f9                    	movq	%rdi, %rcx
140a7a660: e8 5b ee 0f 00              	callq	0x140b794c0 <.text+0xb784c0>
140a7a665: c6 47 38 01                 	movb	$0x1, 0x38(%rdi)
140a7a669: 8b 46 1c                    	movl	0x1c(%rsi), %eax
140a7a66c: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140a7a670: 89 01                       	movl	%eax, (%rcx)
140a7a672: 48 8b 36                    	movq	(%rsi), %rsi
140a7a675: 31 ff                       	xorl	%edi, %edi
140a7a677: 48 bb 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rbx # imm = 0x8000000000000002
140a7a681: eb 48                       	jmp	0x140a7a6cb <.text+0xa796cb>
140a7a683: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a7a690: 48 c7 44 3e 38 00 00 00 00  	movq	$0x0, 0x38(%rsi,%rdi)
140a7a699: 49 8d 46 01                 	leaq	0x1(%r14), %rax
140a7a69d: 48 89 c1                    	movq	%rax, %rcx
140a7a6a0: 48 c1 e9 03                 	shrq	$0x3, %rcx
140a7a6a4: 48 83 e0 f8                 	andq	$-0x8, %rax
140a7a6a8: 48 29 c8                    	subq	%rcx, %rax
140a7a6ab: 49 83 fe 08                 	cmpq	$0x8, %r14
140a7a6af: 49 0f 42 c6                 	cmovbq	%r14, %rax
140a7a6b3: 48 89 44 3e 30              	movq	%rax, 0x30(%rsi,%rdi)
140a7a6b8: 48 c7 84 3e a8 00 00 00 00 00 00 00 	movq	$0x0, 0xa8(%rsi,%rdi)
140a7a6c4: 48 81 c7 c0 00 00 00        	addq	$0xc0, %rdi
140a7a6cb: 48 81 ff 40 14 00 00        	cmpq	$0x1440, %rdi           # imm = 0x1440
140a7a6d2: 74 34                       	je	0x140a7a708 <.text+0xa79708>
140a7a6d4: 48 39 5c 3e 08              	cmpq	%rbx, 0x8(%rsi,%rdi)
140a7a6d9: 7c e9                       	jl	0x140a7a6c4 <.text+0xa796c4>
140a7a6db: 48 c7 44 3e 18 00 00 00 00  	movq	$0x0, 0x18(%rsi,%rdi)
140a7a6e4: 48 83 7c 3e 38 00           	cmpq	$0x0, 0x38(%rsi,%rdi)
140a7a6ea: 74 cc                       	je	0x140a7a6b8 <.text+0xa796b8>
140a7a6ec: 4c 8b 74 3e 28              	movq	0x28(%rsi,%rdi), %r14
140a7a6f1: 4d 85 f6                    	testq	%r14, %r14
140a7a6f4: 74 9a                       	je	0x140a7a690 <.text+0xa79690>
140a7a6f6: 48 8b 4c 3e 20              	movq	0x20(%rsi,%rdi), %rcx
140a7a6fb: 4d 8d 46 11                 	leaq	0x11(%r14), %r8
140a7a6ff: b2 ff                       	movb	$-0x1, %dl
140a7a701: e8 1a 95 ea 01              	callq	0x142923c20 <.text+0x2922c20>
140a7a706: eb 88                       	jmp	0x140a7a690 <.text+0xa79690>
140a7a708: 41 8b 44 24 1c              	movl	0x1c(%r12), %eax
140a7a70d: 49 8b 4c 24 10              	movq	0x10(%r12), %rcx
140a7a712: 89 01                       	movl	%eax, (%rcx)
140a7a714: 49 8b 34 24                 	movq	(%r12), %rsi
140a7a718: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
140a7a71c: 48 8b 46 10                 	movq	0x10(%rsi), %rax
140a7a720: 48 c7 46 10 00 00 00 00     	movq	$0x0, 0x10(%rsi)
140a7a728: 48 85 c0                    	testq	%rax, %rax
140a7a72b: 74 47                       	je	0x140a7a774 <.text+0xa79774>
140a7a72d: ba 01 00 00 00              	movl	$0x1, %edx
140a7a732: 48 29 c2                    	subq	%rax, %rdx
140a7a735: eb 21                       	jmp	0x140a7a758 <.text+0xa79758>
140a7a737: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a7a740: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140a7a744: 48 83 c0 50                 	addq	$0x50, %rax
140a7a748: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140a7a74c: 48 ff c2                    	incq	%rdx
140a7a74f: 48 83 fa 01                 	cmpq	$0x1, %rdx
140a7a753: 48 89 c1                    	movq	%rax, %rcx
140a7a756: 74 1c                       	je	0x140a7a774 <.text+0xa79774>
140a7a758: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
140a7a75c: 48 89 55 e8                 	movq	%rdx, -0x18(%rbp)
140a7a760: 48 8b 01                    	movq	(%rcx), %rax
140a7a763: f0                          	lock
140a7a764: 48 ff 08                    	decq	(%rax)
140a7a767: 75 d7                       	jne	0x140a7a740 <.text+0xa79740>
140a7a769: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140a7a76d: e8 be 43 bd ff              	callq	0x14064eb30 <.text+0x64db30>
140a7a772: eb cc                       	jmp	0x140a7a740 <.text+0xa79740>
140a7a774: c6 46 18 00                 	movb	$0x0, 0x18(%rsi)
140a7a778: 49 8b 45 00                 	movq	(%r13), %rax
140a7a77c: 48 85 c0                    	testq	%rax, %rax
140a7a77f: 74 49                       	je	0x140a7a7ca <.text+0xa797ca>
140a7a781: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
140a7a785: 41 8b 55 1c                 	movl	0x1c(%r13), %edx
140a7a789: 89 11                       	movl	%edx, (%rcx)
140a7a78b: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140a7a78f: 48 85 c9                    	testq	%rcx, %rcx
140a7a792: 74 36                       	je	0x140a7a7ca <.text+0xa797ca>
140a7a794: 48 8b 40 08                 	movq	0x8(%rax), %rax
140a7a798: 48 c1 e1 03                 	shlq	$0x3, %rcx
140a7a79c: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a7a7a0: 31 d2                       	xorl	%edx, %edx
140a7a7a2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a7a7b0: 48 c7 44 10 08 00 00 00 00  	movq	$0x0, 0x8(%rax,%rdx)
140a7a7b9: c7 44 10 10 00 00 00 00     	movl	$0x0, 0x10(%rax,%rdx)
140a7a7c1: 48 83 c2 18                 	addq	$0x18, %rdx
140a7a7c5: 48 39 d1                    	cmpq	%rdx, %rcx
140a7a7c8: 75 e6                       	jne	0x140a7a7b0 <.text+0xa797b0>
140a7a7ca: 49 8b 1f                    	movq	(%r15), %rbx
140a7a7cd: 83 3b 02                    	cmpl	$0x2, (%rbx)
140a7a7d0: 0f 85 d4 01 00 00           	jne	0x140a7a9aa <.text+0xa799aa>
140a7a7d6: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
140a7a7dd: 48 8b 00                    	movq	(%rax), %rax
140a7a7e0: 48 8b 10                    	movq	(%rax), %rdx
140a7a7e3: 48 85 d2                    	testq	%rdx, %rdx
140a7a7e6: 74 5a                       	je	0x140a7a842 <.text+0xa79842>
140a7a7e8: 48 83 c2 10                 	addq	$0x10, %rdx
140a7a7ec: 4c 8d 05 fd c6 05 02        	leaq	0x205c6fd(%rip), %r8    # 0x142ad6ef0
140a7a7f3: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140a7a7f7: 41 b9 11 00 00 00           	movl	$0x11, %r9d
140a7a7fd: e8 3e 9c fd ff              	callq	0x140a54440 <.text+0xa53440>
140a7a802: 48 83 7d 20 02              	cmpq	$0x2, 0x20(%rbp)
140a7a807: 0f 85 82 00 00 00           	jne	0x140a7a88f <.text+0xa7988f>
140a7a80d: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140a7a811: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140a7a815: 48 8d 05 ec c6 05 02        	leaq	0x205c6ec(%rip), %rax   # 0x142ad6f08
140a7a81c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a7a821: 48 8d 0d 78 c1 05 02        	leaq	0x205c178(%rip), %rcx   # 0x142ad69a0
140a7a828: 4c 8d 0d 51 c1 05 02        	leaq	0x205c151(%rip), %r9    # 0x142ad6980
140a7a82f: 4c 8d 45 b0                 	leaq	-0x50(%rbp), %r8
140a7a833: ba 2b 00 00 00              	movl	$0x2b, %edx
140a7a838: e8 23 ef e5 01              	callq	0x1428d9760 <.text+0x28d8760>
140a7a83d: e9 c4 01 00 00              	jmp	0x140a7aa06 <.text+0xa79a06>
140a7a842: 0f b6 05 c0 a3 bb 02        	movzbl	0x2bba3c0(%rip), %eax   # 0x143634c09
140a7a849: b9 11 00 00 00              	movl	$0x11, %ecx
140a7a84e: ba 01 00 00 00              	movl	$0x1, %edx
140a7a853: e8 b8 93 b9 ff              	callq	0x140613c10 <.text+0x612c10>
140a7a858: 48 85 c0                    	testq	%rax, %rax
140a7a85b: 0f 84 a7 01 00 00           	je	0x140a7aa08 <.text+0xa79a08>
140a7a861: 0f 10 05 88 c6 05 02        	movups	0x205c688(%rip), %xmm0  # 0x142ad6ef0
140a7a868: 0f 11 00                    	movups	%xmm0, (%rax)
140a7a86b: c6 40 10 67                 	movb	$0x67, 0x10(%rax)
140a7a86f: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
140a7a877: 48 c7 45 30 11 00 00 00     	movq	$0x11, 0x30(%rbp)
140a7a87f: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140a7a883: 48 c7 45 40 11 00 00 00     	movq	$0x11, 0x40(%rbp)
140a7a88b: c6 45 48 02                 	movb	$0x2, 0x48(%rbp)
140a7a88f: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140a7a893: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140a7a897: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
140a7a89b: 4c 8b 45 38                 	movq	0x38(%rbp), %r8
140a7a89f: 4c 8b 4d 40                 	movq	0x40(%rbp), %r9
140a7a8a3: 4c 89 4d 10                 	movq	%r9, 0x10(%rbp)
140a7a8a7: 44 0f b6 4d 48              	movzbl	0x48(%rbp), %r9d
140a7a8ac: 44 88 4d 18                 	movb	%r9b, 0x18(%rbp)
140a7a8b0: 44 8b 4d 49                 	movl	0x49(%rbp), %r9d
140a7a8b4: 44 89 4d 19                 	movl	%r9d, 0x19(%rbp)
140a7a8b8: 44 0f b7 4d 4d              	movzwl	0x4d(%rbp), %r9d
140a7a8bd: 66 44 89 4d 1d              	movw	%r9w, 0x1d(%rbp)
140a7a8c2: 44 0f b6 4d 4f              	movzbl	0x4f(%rbp), %r9d
140a7a8c7: 44 88 4d 1f                 	movb	%r9b, 0x1f(%rbp)
140a7a8cb: 48 89 55 00                 	movq	%rdx, (%rbp)
140a7a8cf: 4c 89 45 08                 	movq	%r8, 0x8(%rbp)
140a7a8d3: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140a7a8d7: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
140a7a8db: 83 7d f0 01                 	cmpl	$0x1, -0x10(%rbp)
140a7a8df: 75 25                       	jne	0x140a7a906 <.text+0xa79906>
140a7a8e1: 48 8d 55 f8                 	leaq	-0x8(%rbp), %rdx
140a7a8e5: c6 45 6f 01                 	movb	$0x1, 0x6f(%rbp)
140a7a8e9: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140a7a8ed: e8 de c9 9b 00              	callq	0x1414372d0 <.text+0x14362d0>
140a7a8f2: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140a7a8fc: 48 39 45 b0                 	cmpq	%rax, -0x50(%rbp)
140a7a900: 0f 85 b8 00 00 00           	jne	0x140a7a9be <.text+0xa799be>
140a7a906: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140a7a90a: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140a7a90e: 48 8b 55 00                 	movq	(%rbp), %rdx
140a7a912: 4c 8b 45 08                 	movq	0x8(%rbp), %r8
140a7a916: 4c 8b 4d 10                 	movq	0x10(%rbp), %r9
140a7a91a: 4c 89 4d 40                 	movq	%r9, 0x40(%rbp)
140a7a91e: 44 8b 4d 18                 	movl	0x18(%rbp), %r9d
140a7a922: 44 89 4d 48                 	movl	%r9d, 0x48(%rbp)
140a7a926: 44 0f b7 4d 1c              	movzwl	0x1c(%rbp), %r9d
140a7a92b: 66 44 89 4d 4c              	movw	%r9w, 0x4c(%rbp)
140a7a930: 44 0f b6 4d 1e              	movzbl	0x1e(%rbp), %r9d
140a7a935: 44 88 4d 4e                 	movb	%r9b, 0x4e(%rbp)
140a7a939: 44 0f b6 4d 1f              	movzbl	0x1f(%rbp), %r9d
140a7a93e: 44 88 4d 4f                 	movb	%r9b, 0x4f(%rbp)
140a7a942: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
140a7a946: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140a7a94a: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140a7a94e: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
140a7a952: 41 8b 77 1c                 	movl	0x1c(%r15), %esi
140a7a956: 49 8b 7f 10                 	movq	0x10(%r15), %rdi
140a7a95a: 89 37                       	movl	%esi, (%rdi)
140a7a95c: 83 3b 02                    	cmpl	$0x2, (%rbx)
140a7a95f: 74 28                       	je	0x140a7a989 <.text+0xa79989>
140a7a961: 48 89 d9                    	movq	%rbx, %rcx
140a7a964: 48 89 5d 60                 	movq	%rbx, 0x60(%rbp)
140a7a968: e8 13 f9 e1 ff              	callq	0x14089a280 <.text+0x899280>
140a7a96d: 48 8b 5d 60                 	movq	0x60(%rbp), %rbx
140a7a971: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140a7a975: 48 85 d2                    	testq	%rdx, %rdx
140a7a978: 74 0f                       	je	0x140a7a989 <.text+0xa79989>
140a7a97a: 48 8b 4b 18                 	movq	0x18(%rbx), %rcx
140a7a97e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a7a984: e8 97 92 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7a989: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
140a7a98d: 0f 28 4d 30                 	movaps	0x30(%rbp), %xmm1
140a7a991: 0f 28 55 40                 	movaps	0x40(%rbp), %xmm2
140a7a995: 0f 11 53 20                 	movups	%xmm2, 0x20(%rbx)
140a7a999: 0f 11 4b 10                 	movups	%xmm1, 0x10(%rbx)
140a7a99d: 0f 11 03                    	movups	%xmm0, (%rbx)
140a7a9a0: 89 37                       	movl	%esi, (%rdi)
140a7a9a2: 48 c7 43 30 01 00 00 00     	movq	$0x1, 0x30(%rbx)
140a7a9aa: 48 81 c4 f8 00 00 00        	addq	$0xf8, %rsp
140a7a9b1: 5b                          	popq	%rbx
140a7a9b2: 5f                          	popq	%rdi
140a7a9b3: 5e                          	popq	%rsi
140a7a9b4: 41 5c                       	popq	%r12
140a7a9b6: 41 5d                       	popq	%r13
140a7a9b8: 41 5e                       	popq	%r14
140a7a9ba: 41 5f                       	popq	%r15
140a7a9bc: 5d                          	popq	%rbp
140a7a9bd: c3                          	retq
140a7a9be: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
140a7a9c2: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140a7a9c6: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
140a7a9ca: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
140a7a9ce: 0f 10 55 d0                 	movups	-0x30(%rbp), %xmm2
140a7a9d2: 0f 29 55 40                 	movaps	%xmm2, 0x40(%rbp)
140a7a9d6: 0f 29 4d 30                 	movaps	%xmm1, 0x30(%rbp)
140a7a9da: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
140a7a9de: 48 8d 05 db 81 05 02        	leaq	0x20581db(%rip), %rax   # 0x142ad2bc0
140a7a9e5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a7a9ea: 48 8d 0d 0f 5c 05 02        	leaq	0x2055c0f(%rip), %rcx   # 0x142ad0600
140a7a9f1: 4c 8d 0d c8 5b 05 02        	leaq	0x2055bc8(%rip), %r9    # 0x142ad05c0
140a7a9f8: 4c 8d 45 20                 	leaq	0x20(%rbp), %r8
140a7a9fc: ba 2b 00 00 00              	movl	$0x2b, %edx
140a7aa01: e8 5a ed e5 01              	callq	0x1428d9760 <.text+0x28d8760>
140a7aa06: 0f 0b                       	ud2
140a7aa08: 4c 8d 05 a1 5d 05 02        	leaq	0x2055da1(%rip), %r8    # 0x142ad07b0
140a7aa0f: b9 01 00 00 00              	movl	$0x1, %ecx
140a7aa14: ba 11 00 00 00              	movl	$0x11, %edx
140a7aa19: e8 c5 e5 e5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a7aa1e: cc                          	int3
140a7aa1f: 90                          	nop
140a7aa20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7aa25: 55                          	pushq	%rbp
140a7aa26: 41 57                       	pushq	%r15
140a7aa28: 41 56                       	pushq	%r14
140a7aa2a: 41 55                       	pushq	%r13
140a7aa2c: 41 54                       	pushq	%r12
140a7aa2e: 56                          	pushq	%rsi
140a7aa2f: 57                          	pushq	%rdi
140a7aa30: 53                          	pushq	%rbx
140a7aa31: 48 83 ec 28                 	subq	$0x28, %rsp
140a7aa35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7aa3c: 48 8b 7d e8                 	movq	-0x18(%rbp), %rdi
140a7aa40: 48 85 ff                    	testq	%rdi, %rdi
140a7aa43: 48 8b 75 60                 	movq	0x60(%rbp), %rsi
140a7aa47: 74 25                       	je	0x140a7aa6e <.text+0xa79a6e>
140a7aa49: 48 83 c6 50                 	addq	$0x50, %rsi
140a7aa4d: 48 f7 df                    	negq	%rdi
140a7aa50: eb 09                       	jmp	0x140a7aa5b <.text+0xa79a5b>
140a7aa52: 48 83 c6 50                 	addq	$0x50, %rsi
140a7aa56: 48 ff cf                    	decq	%rdi
140a7aa59: 74 13                       	je	0x140a7aa6e <.text+0xa79a6e>
140a7aa5b: 48 8b 06                    	movq	(%rsi), %rax
140a7aa5e: f0                          	lock
140a7aa5f: 48 ff 08                    	decq	(%rax)
140a7aa62: 75 ee                       	jne	0x140a7aa52 <.text+0xa79a52>
140a7aa64: 48 89 f1                    	movq	%rsi, %rcx
140a7aa67: e8 c4 40 bd ff              	callq	0x14064eb30 <.text+0x64db30>
140a7aa6c: eb e4                       	jmp	0x140a7aa52 <.text+0xa79a52>
140a7aa6e: 48 83 c4 28                 	addq	$0x28, %rsp
140a7aa72: 5b                          	popq	%rbx
140a7aa73: 5f                          	popq	%rdi
140a7aa74: 5e                          	popq	%rsi
140a7aa75: 41 5c                       	popq	%r12
140a7aa77: 41 5d                       	popq	%r13
140a7aa79: 41 5e                       	popq	%r14
140a7aa7b: 41 5f                       	popq	%r15
140a7aa7d: 5d                          	popq	%rbp
140a7aa7e: c3                          	retq
140a7aa7f: 90                          	nop
140a7aa80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7aa85: 55                          	pushq	%rbp
140a7aa86: 41 57                       	pushq	%r15
140a7aa88: 41 56                       	pushq	%r14
140a7aa8a: 41 55                       	pushq	%r13
140a7aa8c: 41 54                       	pushq	%r12
140a7aa8e: 56                          	pushq	%rsi
140a7aa8f: 57                          	pushq	%rdi
140a7aa90: 53                          	pushq	%rbx
140a7aa91: 48 83 ec 28                 	subq	$0x28, %rsp
140a7aa95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7aa9c: c6 45 6f 01                 	movb	$0x1, 0x6f(%rbp)
140a7aaa0: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140a7aaa4: e8 07 7c fd ff              	callq	0x140a526b0 <.text+0xa516b0>
140a7aaa9: c6 45 6f 01                 	movb	$0x1, 0x6f(%rbp)
140a7aaad: 48 83 c4 28                 	addq	$0x28, %rsp
140a7aab1: 5b                          	popq	%rbx
140a7aab2: 5f                          	popq	%rdi
140a7aab3: 5e                          	popq	%rsi
140a7aab4: 41 5c                       	popq	%r12
140a7aab6: 41 5d                       	popq	%r13
140a7aab8: 41 5e                       	popq	%r14
140a7aaba: 41 5f                       	popq	%r15
140a7aabc: 5d                          	popq	%rbp
140a7aabd: c3                          	retq
140a7aabe: 66 90                       	nop
140a7aac0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7aac5: 55                          	pushq	%rbp
140a7aac6: 41 57                       	pushq	%r15
140a7aac8: 41 56                       	pushq	%r14
140a7aaca: 41 55                       	pushq	%r13
140a7aacc: 41 54                       	pushq	%r12
140a7aace: 56                          	pushq	%rsi
140a7aacf: 57                          	pushq	%rdi
140a7aad0: 53                          	pushq	%rbx
140a7aad1: 48 83 ec 28                 	subq	$0x28, %rsp
140a7aad5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7aadc: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140a7aae0: 48 89 c6                    	movq	%rax, %rsi
140a7aae3: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140a7aae7: 48 85 d2                    	testq	%rdx, %rdx
140a7aaea: 74 0f                       	je	0x140a7aafb <.text+0xa79afb>
140a7aaec: 48 8b 4e 18                 	movq	0x18(%rsi), %rcx
140a7aaf0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a7aaf6: e8 25 91 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7aafb: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
140a7aaff: 0f 28 4d 30                 	movaps	0x30(%rbp), %xmm1
140a7ab03: 0f 28 55 40                 	movaps	0x40(%rbp), %xmm2
140a7ab07: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140a7ab0b: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140a7ab0f: 0f 11 06                    	movups	%xmm0, (%rsi)
140a7ab12: c6 45 6f 00                 	movb	$0x0, 0x6f(%rbp)
140a7ab16: 48 83 c4 28                 	addq	$0x28, %rsp
140a7ab1a: 5b                          	popq	%rbx
140a7ab1b: 5f                          	popq	%rdi
140a7ab1c: 5e                          	popq	%rsi
140a7ab1d: 41 5c                       	popq	%r12
140a7ab1f: 41 5d                       	popq	%r13
140a7ab21: 41 5e                       	popq	%r14
140a7ab23: 41 5f                       	popq	%r15
140a7ab25: 5d                          	popq	%rbp
140a7ab26: c3                          	retq
140a7ab27: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a7ab30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7ab35: 55                          	pushq	%rbp
140a7ab36: 41 57                       	pushq	%r15
140a7ab38: 41 56                       	pushq	%r14
140a7ab3a: 41 55                       	pushq	%r13
140a7ab3c: 41 54                       	pushq	%r12
140a7ab3e: 56                          	pushq	%rsi
140a7ab3f: 57                          	pushq	%rdi
140a7ab40: 53                          	pushq	%rbx
140a7ab41: 48 83 ec 28                 	subq	$0x28, %rsp
140a7ab45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7ab4c: 80 7d 6f 00                 	cmpb	$0x0, 0x6f(%rbp)
140a7ab50: 75 11                       	jne	0x140a7ab63 <.text+0xa79b63>
140a7ab52: 48 83 c4 28                 	addq	$0x28, %rsp
140a7ab56: 5b                          	popq	%rbx
140a7ab57: 5f                          	popq	%rdi
140a7ab58: 5e                          	popq	%rsi
140a7ab59: 41 5c                       	popq	%r12
140a7ab5b: 41 5d                       	popq	%r13
140a7ab5d: 41 5e                       	popq	%r14
140a7ab5f: 41 5f                       	popq	%r15
140a7ab61: 5d                          	popq	%rbp
140a7ab62: c3                          	retq
140a7ab63: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140a7ab67: e8 14 fb 6c ff              	callq	0x14014a680 <.text+0x149680>
140a7ab6c: eb e4                       	jmp	0x140a7ab52 <.text+0xa79b52>
140a7ab6e: 66 90                       	nop
140a7ab70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7ab75: 55                          	pushq	%rbp
140a7ab76: 41 57                       	pushq	%r15
140a7ab78: 41 56                       	pushq	%r14
140a7ab7a: 41 55                       	pushq	%r13
140a7ab7c: 41 54                       	pushq	%r12
140a7ab7e: 56                          	pushq	%rsi
140a7ab7f: 57                          	pushq	%rdi
140a7ab80: 53                          	pushq	%rbx
140a7ab81: 48 83 ec 28                 	subq	$0x28, %rsp
140a7ab85: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7ab8c: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140a7ab90: e8 9b 20 2d 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a7ab95: 90                          	nop
140a7ab96: 48 83 c4 28                 	addq	$0x28, %rsp
140a7ab9a: 5b                          	popq	%rbx
140a7ab9b: 5f                          	popq	%rdi
140a7ab9c: 5e                          	popq	%rsi
140a7ab9d: 41 5c                       	popq	%r12
140a7ab9f: 41 5d                       	popq	%r13
140a7aba1: 41 5e                       	popq	%r14
140a7aba3: 41 5f                       	popq	%r15
140a7aba5: 5d                          	popq	%rbp
140a7aba6: c3                          	retq
