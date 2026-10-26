
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14155e460: 55                          	pushq	%rbp
14155e461: 41 57                       	pushq	%r15
14155e463: 41 56                       	pushq	%r14
14155e465: 41 55                       	pushq	%r13
14155e467: 41 54                       	pushq	%r12
14155e469: 56                          	pushq	%rsi
14155e46a: 57                          	pushq	%rdi
14155e46b: 53                          	pushq	%rbx
14155e46c: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
14155e473: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14155e47b: 48 c7 45 20 fe ff ff ff     	movq	$-0x2, 0x20(%rbp)
14155e483: 49 89 d6                    	movq	%rdx, %r14
14155e486: 48 89 ce                    	movq	%rcx, %rsi
14155e489: 44 88 45 1f                 	movb	%r8b, 0x1f(%rbp)
14155e48d: 48 89 55 a0                 	movq	%rdx, -0x60(%rbp)
14155e491: 48 8d 05 48 b8 ff ff        	leaq	-0x47b8(%rip), %rax     # 0x141559ce0 <.text+0x1558ce0>
14155e498: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
14155e49c: 48 8d 45 1f                 	leaq	0x1f(%rbp), %rax
14155e4a0: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
14155e4a4: 48 8d 05 65 82 bf fe        	leaq	-0x1407d9b(%rip), %rax  # 0x140156710 <.text+0x155710>
14155e4ab: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
14155e4af: 48 8d 05 6a fe 70 01        	leaq	0x170fe6a(%rip), %rax   # 0x142c6e320
14155e4b6: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14155e4ba: 48 c7 45 e8 02 00 00 00     	movq	$0x2, -0x18(%rbp)
14155e4c2: 48 8d 45 a0                 	leaq	-0x60(%rbp), %rax
14155e4c6: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14155e4ca: 48 c7 45 f8 02 00 00 00     	movq	$0x2, -0x8(%rbp)
14155e4d2: 48 c7 45 00 00 00 00 00     	movq	$0x0, (%rbp)
14155e4da: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14155e4de: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
14155e4e2: e8 a9 ba 81 ff              	callq	0x140d79f90 <.text+0xd78f90>
14155e4e7: 48 8b 7d c8                 	movq	-0x38(%rbp), %rdi
14155e4eb: 48 8b 5d d0                 	movq	-0x30(%rbp), %rbx
14155e4ef: 49 8b 46 10                 	movq	0x10(%r14), %rax
14155e4f3: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14155e4f7: 41 0f 10 06                 	movups	(%r14), %xmm0
14155e4fb: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
14155e4ff: 44 0f b6 75 1f              	movzbl	0x1f(%rbp), %r14d
14155e504: 0f b6 05 fe 66 0d 02        	movzbl	0x20d66fe(%rip), %eax   # 0x143634c09
14155e50b: b9 30 00 00 00              	movl	$0x30, %ecx
14155e510: ba 08 00 00 00              	movl	$0x8, %edx
14155e515: e8 f6 56 0b ff              	callq	0x140613c10 <.text+0x612c10>
14155e51a: 48 85 c0                    	testq	%rax, %rax
14155e51d: 0f 84 b7 00 00 00           	je	0x14155e5da <.text+0x155d5da>
14155e523: 49 89 c5                    	movq	%rax, %r13
14155e526: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14155e52d: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14155e535: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
14155e539: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
14155e53d: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14155e541: 49 89 45 20                 	movq	%rax, 0x20(%r13)
14155e545: 45 88 75 28                 	movb	%r14b, 0x28(%r13)
14155e549: 4c 89 6d e0                 	movq	%r13, -0x20(%rbp)
14155e54d: 4c 8d 25 ec fd 70 01        	leaq	0x170fdec(%rip), %r12   # 0x142c6e340
14155e554: 4c 89 65 e8                 	movq	%r12, -0x18(%rbp)
14155e558: 48 85 db                    	testq	%rbx, %rbx
14155e55b: 0f 88 8a 00 00 00           	js	0x14155e5eb <.text+0x155d5eb>
14155e561: 74 24                       	je	0x14155e587 <.text+0x155d587>
14155e563: 0f b6 05 9f 66 0d 02        	movzbl	0x20d669f(%rip), %eax   # 0x143634c09
14155e56a: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14155e570: ba 01 00 00 00              	movl	$0x1, %edx
14155e575: 48 89 d9                    	movq	%rbx, %rcx
14155e578: e8 93 56 0b ff              	callq	0x140613c10 <.text+0x612c10>
14155e57d: 48 85 c0                    	testq	%rax, %rax
14155e580: 74 6c                       	je	0x14155e5ee <.text+0x155d5ee>
14155e582: 49 89 c6                    	movq	%rax, %r14
14155e585: eb 06                       	jmp	0x14155e58d <.text+0x155d58d>
14155e587: 41 be 01 00 00 00           	movl	$0x1, %r14d
14155e58d: 4c 89 f1                    	movq	%r14, %rcx
14155e590: 48 89 fa                    	movq	%rdi, %rdx
14155e593: 49 89 d8                    	movq	%rbx, %r8
14155e596: e8 65 56 3c 01              	callq	0x142923c00 <.text+0x2922c00>
14155e59b: 48 89 1e                    	movq	%rbx, (%rsi)
14155e59e: 4c 89 76 08                 	movq	%r14, 0x8(%rsi)
14155e5a2: 48 89 5e 10                 	movq	%rbx, 0x10(%rsi)
14155e5a6: 4c 89 6e 18                 	movq	%r13, 0x18(%rsi)
14155e5aa: 4c 89 66 20                 	movq	%r12, 0x20(%rsi)
14155e5ae: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
14155e5b2: 48 85 d2                    	testq	%rdx, %rdx
14155e5b5: 74 0e                       	je	0x14155e5c5 <.text+0x155d5c5>
14155e5b7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14155e5bd: 48 89 f9                    	movq	%rdi, %rcx
14155e5c0: e8 5b 56 0b ff              	callq	0x140613c20 <.text+0x612c20>
14155e5c5: 90                          	nop
14155e5c6: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
14155e5cd: 5b                          	popq	%rbx
14155e5ce: 5f                          	popq	%rdi
14155e5cf: 5e                          	popq	%rsi
14155e5d0: 41 5c                       	popq	%r12
14155e5d2: 41 5d                       	popq	%r13
14155e5d4: 41 5e                       	popq	%r14
14155e5d6: 41 5f                       	popq	%r15
14155e5d8: 5d                          	popq	%rbp
14155e5d9: c3                          	retq
14155e5da: b9 08 00 00 00              	movl	$0x8, %ecx
14155e5df: ba 30 00 00 00              	movl	$0x30, %edx
14155e5e4: e8 17 aa 37 01              	callq	0x1428d9000 <.text+0x28d8000>
14155e5e9: eb 19                       	jmp	0x14155e604 <.text+0x155d604>
14155e5eb: 45 31 ff                    	xorl	%r15d, %r15d
14155e5ee: 4c 89 6d d8                 	movq	%r13, -0x28(%rbp)
14155e5f2: 4c 8d 05 27 a9 53 01        	leaq	0x153a927(%rip), %r8    # 0x142a98f20
14155e5f9: 4c 89 f9                    	movq	%r15, %rcx
14155e5fc: 48 89 da                    	movq	%rbx, %rdx
14155e5ff: e8 df a9 37 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14155e604: 0f 0b                       	ud2
14155e606: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14155e610: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14155e615: 55                          	pushq	%rbp
14155e616: 41 57                       	pushq	%r15
14155e618: 41 56                       	pushq	%r14
14155e61a: 41 55                       	pushq	%r13
14155e61c: 41 54                       	pushq	%r12
14155e61e: 56                          	pushq	%rsi
14155e61f: 57                          	pushq	%rdi
14155e620: 53                          	pushq	%rbx
14155e621: 48 83 ec 28                 	subq	$0x28, %rsp
14155e625: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14155e62c: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
14155e630: 48 85 d2                    	testq	%rdx, %rdx
14155e633: 74 0f                       	je	0x14155e644 <.text+0x155d644>
14155e635: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
14155e639: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14155e63f: e8 dc 55 0b ff              	callq	0x140613c20 <.text+0x612c20>
14155e644: 90                          	nop
14155e645: 48 83 c4 28                 	addq	$0x28, %rsp
14155e649: 5b                          	popq	%rbx
14155e64a: 5f                          	popq	%rdi
14155e64b: 5e                          	popq	%rsi
14155e64c: 41 5c                       	popq	%r12
14155e64e: 41 5d                       	popq	%r13
14155e650: 41 5e                       	popq	%r14
14155e652: 41 5f                       	popq	%r15
14155e654: 5d                          	popq	%rbp
14155e655: c3                          	retq
14155e656: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14155e660: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14155e665: 55                          	pushq	%rbp
14155e666: 41 57                       	pushq	%r15
14155e668: 41 56                       	pushq	%r14
14155e66a: 41 55                       	pushq	%r13
14155e66c: 41 54                       	pushq	%r12
14155e66e: 56                          	pushq	%rsi
14155e66f: 57                          	pushq	%rdi
14155e670: 53                          	pushq	%rbx
14155e671: 48 83 ec 28                 	subq	$0x28, %rsp
14155e675: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14155e67c: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14155e680: f0                          	lock
14155e681: 48 ff 08                    	decq	(%rax)
14155e684: 75 09                       	jne	0x14155e68f <.text+0x155d68f>
14155e686: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
14155e68a: e8 41 13 0f ff              	callq	0x14064f9d0 <.text+0x64e9d0>
14155e68f: 90                          	nop
14155e690: 48 83 c4 28                 	addq	$0x28, %rsp
14155e694: 5b                          	popq	%rbx
14155e695: 5f                          	popq	%rdi
14155e696: 5e                          	popq	%rsi
14155e697: 41 5c                       	popq	%r12
14155e699: 41 5d                       	popq	%r13
14155e69b: 41 5e                       	popq	%r14
14155e69d: 41 5f                       	popq	%r15
14155e69f: 5d                          	popq	%rbp
14155e6a0: c3                          	retq
