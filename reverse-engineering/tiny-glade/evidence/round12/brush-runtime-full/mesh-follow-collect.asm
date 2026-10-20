
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412fd550: 55                          	pushq	%rbp
1412fd551: 41 57                       	pushq	%r15
1412fd553: 41 56                       	pushq	%r14
1412fd555: 41 55                       	pushq	%r13
1412fd557: 41 54                       	pushq	%r12
1412fd559: 56                          	pushq	%rsi
1412fd55a: 57                          	pushq	%rdi
1412fd55b: 53                          	pushq	%rbx
1412fd55c: 48 83 ec 68                 	subq	$0x68, %rsp
1412fd560: 48 8d 6c 24 60              	leaq	0x60(%rsp), %rbp
1412fd565: 0f 29 7d f0                 	movaps	%xmm7, -0x10(%rbp)
1412fd569: 0f 29 75 e0                 	movaps	%xmm6, -0x20(%rbp)
1412fd56d: 48 c7 45 d8 fe ff ff ff     	movq	$-0x2, -0x28(%rbp)
1412fd575: 4c 89 45 d0                 	movq	%r8, -0x30(%rbp)
1412fd579: 48 89 d3                    	movq	%rdx, %rbx
1412fd57c: 48 89 ce                    	movq	%rcx, %rsi
1412fd57f: 4c 8b 2a                    	movq	(%rdx), %r13
1412fd582: 4c 8b 72 08                 	movq	0x8(%rdx), %r14
1412fd586: 4c 89 f7                    	movq	%r14, %rdi
1412fd589: 4c 29 ef                    	subq	%r13, %rdi
1412fd58c: 48 c1 ef 02                 	shrq	$0x2, %rdi
1412fd590: 45 31 e4                    	xorl	%r12d, %r12d
1412fd593: b9 0c 00 00 00              	movl	$0xc, %ecx
1412fd598: 48 89 f8                    	movq	%rdi, %rax
1412fd59b: 48 f7 e1                    	mulq	%rcx
1412fd59e: 49 89 c7                    	movq	%rax, %r15
1412fd5a1: 0f 80 1e 01 00 00           	jo	0x1412fd6c5 <.text+0x12fc6c5>
1412fd5a7: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
1412fd5b1: 49 39 c7                    	cmpq	%rax, %r15
1412fd5b4: 0f 87 0b 01 00 00           	ja	0x1412fd6c5 <.text+0x12fc6c5>
1412fd5ba: 4d 85 ff                    	testq	%r15, %r15
1412fd5bd: 0f 84 a1 00 00 00           	je	0x1412fd664 <.text+0x12fc664>
1412fd5c3: 0f b6 05 3f 76 33 02        	movzbl	0x233763f(%rip), %eax   # 0x143634c09
1412fd5ca: 41 bc 04 00 00 00           	movl	$0x4, %r12d
1412fd5d0: ba 04 00 00 00              	movl	$0x4, %edx
1412fd5d5: 4c 89 f9                    	movq	%r15, %rcx
1412fd5d8: e8 33 66 31 ff              	callq	0x140613c10 <.text+0x612c10>
1412fd5dd: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
1412fd5e1: 48 85 c0                    	testq	%rax, %rax
1412fd5e4: 0f 84 db 00 00 00           	je	0x1412fd6c5 <.text+0x12fc6c5>
1412fd5ea: 48 89 7d d0                 	movq	%rdi, -0x30(%rbp)
1412fd5ee: 48 89 75 c0                 	movq	%rsi, -0x40(%rbp)
1412fd5f2: 4d 39 f5                    	cmpq	%r14, %r13
1412fd5f5: 0f 84 87 00 00 00           	je	0x1412fd682 <.text+0x12fc682>
1412fd5fb: 4c 8b 7b 10                 	movq	0x10(%rbx), %r15
1412fd5ff: 4c 8b 73 18                 	movq	0x18(%rbx), %r14
1412fd603: 4c 8b 63 20                 	movq	0x20(%rbx), %r12
1412fd607: 31 db                       	xorl	%ebx, %ebx
1412fd609: 31 f6                       	xorl	%esi, %esi
1412fd60b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1412fd610: 41 8b 4c b5 00              	movl	(%r13,%rsi,4), %ecx
1412fd615: 49 39 ce                    	cmpq	%rcx, %r14
1412fd618: 0f 86 96 00 00 00           	jbe	0x1412fd6b4 <.text+0x12fc6b4>
1412fd61e: 48 8d 04 49                 	leaq	(%rcx,%rcx,2), %rax
1412fd622: f3 41 0f 10 3c 87           	movss	(%r15,%rax,4), %xmm7
1412fd628: f3 41 0f 10 74 87 08        	movss	0x8(%r15,%rax,4), %xmm6
1412fd62f: 49 8b 0c 24                 	movq	(%r12), %rcx
1412fd633: 0f 28 cf                    	movaps	%xmm7, %xmm1
1412fd636: 0f 28 d6                    	movaps	%xmm6, %xmm2
1412fd639: e8 02 c0 6e ff              	callq	0x1409e9640 <.text+0x9e8640>
1412fd63e: 48 8d 04 5b                 	leaq	(%rbx,%rbx,2), %rax
1412fd642: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
1412fd646: f3 0f 11 3c 81              	movss	%xmm7, (%rcx,%rax,4)
1412fd64b: f3 0f 11 44 81 04           	movss	%xmm0, 0x4(%rcx,%rax,4)
1412fd651: f3 0f 11 74 81 08           	movss	%xmm6, 0x8(%rcx,%rax,4)
1412fd657: 48 ff c3                    	incq	%rbx
1412fd65a: 48 ff c6                    	incq	%rsi
1412fd65d: 48 39 fe                    	cmpq	%rdi, %rsi
1412fd660: 75 ae                       	jne	0x1412fd610 <.text+0x12fc610>
1412fd662: eb 20                       	jmp	0x1412fd684 <.text+0x12fc684>
1412fd664: b8 04 00 00 00              	movl	$0x4, %eax
1412fd669: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
1412fd66d: 48 c7 45 d0 00 00 00 00     	movq	$0x0, -0x30(%rbp)
1412fd675: 48 89 75 c0                 	movq	%rsi, -0x40(%rbp)
1412fd679: 4d 39 f5                    	cmpq	%r14, %r13
1412fd67c: 0f 85 79 ff ff ff           	jne	0x1412fd5fb <.text+0x12fc5fb>
1412fd682: 31 db                       	xorl	%ebx, %ebx
1412fd684: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
1412fd688: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
1412fd68c: 48 89 08                    	movq	%rcx, (%rax)
1412fd68f: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
1412fd693: 48 89 48 08                 	movq	%rcx, 0x8(%rax)
1412fd697: 48 89 58 10                 	movq	%rbx, 0x10(%rax)
1412fd69b: 0f 28 75 e0                 	movaps	-0x20(%rbp), %xmm6
1412fd69f: 0f 28 7d f0                 	movaps	-0x10(%rbp), %xmm7
1412fd6a3: 48 83 c4 68                 	addq	$0x68, %rsp
1412fd6a7: 5b                          	popq	%rbx
1412fd6a8: 5f                          	popq	%rdi
1412fd6a9: 5e                          	popq	%rsi
1412fd6aa: 41 5c                       	popq	%r12
1412fd6ac: 41 5d                       	popq	%r13
1412fd6ae: 41 5e                       	popq	%r14
1412fd6b0: 41 5f                       	popq	%r15
1412fd6b2: 5d                          	popq	%rbp
1412fd6b3: c3                          	retq
1412fd6b4: 4c 8d 05 c5 12 97 01        	leaq	0x19712c5(%rip), %r8    # 0x142c6e980
1412fd6bb: 4c 89 f2                    	movq	%r14, %rdx
1412fd6be: e8 55 be 5d 01              	callq	0x1428d9518 <.text+0x28d8518>
1412fd6c3: 0f 0b                       	ud2
1412fd6c5: 4c 89 e1                    	movq	%r12, %rcx
1412fd6c8: 4c 89 fa                    	movq	%r15, %rdx
1412fd6cb: 4c 8b 45 d0                 	movq	-0x30(%rbp), %r8
1412fd6cf: e8 0f b9 5d 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1412fd6d4: cc                          	int3
1412fd6d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1412fd6e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412fd6e5: 55                          	pushq	%rbp
1412fd6e6: 41 57                       	pushq	%r15
1412fd6e8: 41 56                       	pushq	%r14
1412fd6ea: 41 55                       	pushq	%r13
1412fd6ec: 41 54                       	pushq	%r12
1412fd6ee: 56                          	pushq	%rsi
1412fd6ef: 57                          	pushq	%rdi
1412fd6f0: 53                          	pushq	%rbx
1412fd6f1: 48 83 ec 48                 	subq	$0x48, %rsp
1412fd6f5: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
1412fd6f9: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
1412fd6fe: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1412fd703: 48 83 7d d0 00              	cmpq	$0x0, -0x30(%rbp)
1412fd708: 74 1b                       	je	0x1412fd725 <.text+0x12fc725>
1412fd70a: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1412fd70e: 48 c1 e0 02                 	shlq	$0x2, %rax
1412fd712: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1412fd716: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412fd71c: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
1412fd720: e8 fb 64 31 ff              	callq	0x140613c20 <.text+0x612c20>
1412fd725: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1412fd72a: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
1412fd72f: 48 83 c4 48                 	addq	$0x48, %rsp
1412fd733: 5b                          	popq	%rbx
1412fd734: 5f                          	popq	%rdi
1412fd735: 5e                          	popq	%rsi
1412fd736: 41 5c                       	popq	%r12
1412fd738: 41 5d                       	popq	%r13
1412fd73a: 41 5e                       	popq	%r14
1412fd73c: 41 5f                       	popq	%r15
1412fd73e: 5d                          	popq	%rbp
1412fd73f: c3                          	retq
