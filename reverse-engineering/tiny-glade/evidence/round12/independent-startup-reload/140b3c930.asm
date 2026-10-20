
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b3c930: 41 57                       	pushq	%r15
140b3c932: 41 56                       	pushq	%r14
140b3c934: 41 55                       	pushq	%r13
140b3c936: 41 54                       	pushq	%r12
140b3c938: 56                          	pushq	%rsi
140b3c939: 57                          	pushq	%rdi
140b3c93a: 53                          	pushq	%rbx
140b3c93b: 48 81 ec 80 00 00 00        	subq	$0x80, %rsp
140b3c942: bb 01 00 00 00              	movl	$0x1, %ebx
140b3c947: f0                          	lock
140b3c948: 0f c1 9a b0 05 00 00        	xaddl	%ebx, 0x5b0(%rdx)
140b3c94f: f6 01 01                    	testb	$0x1, (%rcx)
140b3c952: 0f 84 ad 01 00 00           	je	0x140b3cb05 <.text+0xb3bb05>
140b3c958: 48 89 ce                    	movq	%rcx, %rsi
140b3c95b: 48 8b 41 08                 	movq	0x8(%rcx), %rax
140b3c95f: 48 39 82 18 02 00 00        	cmpq	%rax, 0x218(%rdx)
140b3c966: 0f 86 f6 00 00 00           	jbe	0x140b3ca62 <.text+0xb3ba62>
140b3c96c: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
140b3c973: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
140b3c977: 48 85 c9                    	testq	%rcx, %rcx
140b3c97a: 0f 84 e2 00 00 00           	je	0x140b3ca62 <.text+0xb3ba62>
140b3c980: 48 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %rax
140b3c987: 48 f7 d1                    	notq	%rcx
140b3c98a: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b3c98e: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b3c992: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140b3c998: 0f 84 c4 00 00 00           	je	0x140b3ca62 <.text+0xb3ba62>
140b3c99e: 48 01 c8                    	addq	%rcx, %rax
140b3c9a1: 4c 8b 70 10                 	movq	0x10(%rax), %r14
140b3c9a5: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140b3c9ab: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140b3c9af: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
140b3c9b6: 0f 86 c4 00 00 00           	jbe	0x140b3ca80 <.text+0xb3ba80>
140b3c9bc: 4c 8b 82 10 02 00 00        	movq	0x210(%rdx), %r8
140b3c9c3: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
140b3c9c7: 48 85 c9                    	testq	%rcx, %rcx
140b3c9ca: 0f 84 b0 00 00 00           	je	0x140b3ca80 <.text+0xb3ba80>
140b3c9d0: 4c 8b ba e0 01 00 00        	movq	0x1e0(%rdx), %r15
140b3c9d7: 48 f7 d1                    	notq	%rcx
140b3c9da: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b3c9de: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b3c9e2: 49 83 7c 0f 20 00           	cmpq	$0x0, 0x20(%r15,%rcx)
140b3c9e8: 0f 84 92 00 00 00           	je	0x140b3ca80 <.text+0xb3ba80>
140b3c9ee: 49 01 cf                    	addq	%rcx, %r15
140b3c9f1: 49 8b 7f 10                 	movq	0x10(%r15), %rdi
140b3c9f5: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140b3c9fb: 89 58 5c                    	movl	%ebx, 0x5c(%rax)
140b3c9fe: 4d 8b 66 30                 	movq	0x30(%r14), %r12
140b3ca02: 4d 8b 6e 40                 	movq	0x40(%r14), %r13
140b3ca06: 4d 3b 66 20                 	cmpq	0x20(%r14), %r12
140b3ca0a: 75 10                       	jne	0x140b3ca1c <.text+0xb3ba1c>
140b3ca0c: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
140b3ca10: 48 8d 15 31 d7 0f 02        	leaq	0x20fd731(%rip), %rdx   # 0x142c3a148
140b3ca17: e8 14 5c a2 ff              	callq	0x140562630 <.text+0x561630>
140b3ca1c: 49 8b 46 28                 	movq	0x28(%r14), %rax
140b3ca20: 4c 89 e1                    	movq	%r12, %rcx
140b3ca23: 48 c1 e1 04                 	shlq	$0x4, %rcx
140b3ca27: 4c 89 2c 08                 	movq	%r13, (%rax,%rcx)
140b3ca2b: c6 44 08 08 00              	movb	$0x0, 0x8(%rax,%rcx)
140b3ca30: 49 ff c4                    	incq	%r12
140b3ca33: 4d 89 66 30                 	movq	%r12, 0x30(%r14)
140b3ca37: 49 ff 46 40                 	incq	0x40(%r14)
140b3ca3b: 41 89 5f 5c                 	movl	%ebx, 0x5c(%r15)
140b3ca3f: 48 89 f9                    	movq	%rdi, %rcx
140b3ca42: e8 f9 6d f0 ff              	callq	0x140a43840 <.text+0xa42840>
140b3ca47: 89 9e 50 01 00 00           	movl	%ebx, 0x150(%rsi)
140b3ca4d: 31 c0                       	xorl	%eax, %eax
140b3ca4f: 48 81 c4 80 00 00 00        	addq	$0x80, %rsp
140b3ca56: 5b                          	popq	%rbx
140b3ca57: 5f                          	popq	%rdi
140b3ca58: 5e                          	popq	%rsi
140b3ca59: 41 5c                       	popq	%r12
140b3ca5b: 41 5d                       	popq	%r13
140b3ca5d: 41 5e                       	popq	%r14
140b3ca5f: 41 5f                       	popq	%r15
140b3ca61: c3                          	retq
140b3ca62: 48 81 c6 b8 00 00 00        	addq	$0xb8, %rsi
140b3ca69: 48 8d 05 5c 1c fb 01        	leaq	0x1fb1c5c(%rip), %rax   # 0x142aee6cc
140b3ca70: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b3ca75: 48 c7 44 24 28 55 00 00 00  	movq	$0x55, 0x28(%rsp)
140b3ca7e: eb 1c                       	jmp	0x140b3ca9c <.text+0xb3ba9c>
140b3ca80: 48 81 c6 b8 00 00 00        	addq	$0xb8, %rsi
140b3ca87: 48 8d 05 e7 1c fb 01        	leaq	0x1fb1ce7(%rip), %rax   # 0x142aee775
140b3ca8e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b3ca93: 48 c7 44 24 28 35 00 00 00  	movq	$0x35, 0x28(%rsp)
140b3ca9c: 48 89 74 24 30              	movq	%rsi, 0x30(%rsp)
140b3caa1: 48 8d 05 28 f0 4f ff        	leaq	-0xb00fd8(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140b3caa8: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140b3caad: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
140b3cab2: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140b3cab7: 48 8d 05 d2 d3 5c ff        	leaq	-0xa32c2e(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b3cabe: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140b3cac3: 48 8d 05 9e 1a fb 01        	leaq	0x1fb1a9e(%rip), %rax   # 0x142aee568
140b3caca: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140b3cacf: 48 c7 44 24 58 02 00 00 00  	movq	$0x2, 0x58(%rsp)
140b3cad8: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140b3cae1: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
140b3cae6: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140b3caeb: 48 c7 44 24 68 02 00 00 00  	movq	$0x2, 0x68(%rsp)
140b3caf4: 48 8d 15 2d 1c fb 01        	leaq	0x1fb1c2d(%rip), %rdx   # 0x142aee728
140b3cafb: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140b3cb00: e8 8b c8 d9 01              	callq	0x1428d9390 <.text+0x28d8390>
140b3cb05: 48 8d 0d 7c 1d fb 01        	leaq	0x1fb1d7c(%rip), %rcx   # 0x142aee888
140b3cb0c: 4c 8d 05 d5 1d fb 01        	leaq	0x1fb1dd5(%rip), %r8    # 0x142aee8e8
140b3cb13: ba 59 00 00 00              	movl	$0x59, %edx
140b3cb18: e8 13 c8 d9 01              	callq	0x1428d9330 <.text+0x28d8330>
140b3cb1d: cc                          	int3
