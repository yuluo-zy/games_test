
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14081b5e0: 41 57                       	pushq	%r15
14081b5e2: 41 56                       	pushq	%r14
14081b5e4: 41 55                       	pushq	%r13
14081b5e6: 41 54                       	pushq	%r12
14081b5e8: 56                          	pushq	%rsi
14081b5e9: 57                          	pushq	%rdi
14081b5ea: 55                          	pushq	%rbp
14081b5eb: 53                          	pushq	%rbx
14081b5ec: 48 83 ec 38                 	subq	$0x38, %rsp
14081b5f0: 4c 89 4c 24 28              	movq	%r9, 0x28(%rsp)
14081b5f5: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
14081b5fa: 49 8b 00                    	movq	(%r8), %rax
14081b5fd: 49 8b 78 08                 	movq	0x8(%r8), %rdi
14081b601: 48 8b 10                    	movq	(%rax), %rdx
14081b604: 31 f6                       	xorl	%esi, %esi
14081b606: 49 89 d0                    	movq	%rdx, %r8
14081b609: 4c 2b 47 18                 	subq	0x18(%rdi), %r8
14081b60d: 4c 0f 42 c6                 	cmovbq	%rsi, %r8
14081b611: 48 2b 57 38                 	subq	0x38(%rdi), %rdx
14081b615: 48 0f 42 d6                 	cmovbq	%rsi, %rdx
14081b619: 4c 8b 5f 10                 	movq	0x10(%rdi), %r11
14081b61d: 4c 8b 57 30                 	movq	0x30(%rdi), %r10
14081b621: 4d 89 d1                    	movq	%r10, %r9
14081b624: 49 29 d1                    	subq	%rdx, %r9
14081b627: 4c 0f 42 ce                 	cmovbq	%rsi, %r9
14081b62b: 0f 96 c3                    	setbe	%bl
14081b62e: 4d 89 df                    	movq	%r11, %r15
14081b631: 4d 29 c7                    	subq	%r8, %r15
14081b634: 4c 0f 42 fe                 	cmovbq	%rsi, %r15
14081b638: 48 89 ce                    	movq	%rcx, %rsi
14081b63b: 4c 8b 77 40                 	movq	0x40(%rdi), %r14
14081b63f: 40 0f 96 c5                 	setbe	%bpl
14081b643: 4c 89 f1                    	movq	%r14, %rcx
14081b646: 4c 29 c9                    	subq	%r9, %rcx
14081b649: 4c 29 f9                    	subq	%r15, %rcx
14081b64c: 4c 8b 6f 08                 	movq	0x8(%rdi), %r13
14081b650: 4c 8b 67 28                 	movq	0x28(%rdi), %r12
14081b654: 48 89 08                    	movq	%rcx, (%rax)
14081b657: 48 8b bc 24 a8 00 00 00     	movq	0xa8(%rsp), %rdi
14081b65f: 40 20 dd                    	andb	%bl, %bpl
14081b662: 40 88 6c 24 27              	movb	%bpl, 0x27(%rsp)
14081b667: 75 56                       	jne	0x14081b6bf <.text+0x81a6bf>
14081b669: 48 89 d1                    	movq	%rdx, %rcx
14081b66c: 48 c1 e1 04                 	shlq	$0x4, %rcx
14081b670: 49 01 cc                    	addq	%rcx, %r12
14081b673: 49 c1 e1 04                 	shlq	$0x4, %r9
14081b677: 4c 89 c1                    	movq	%r8, %rcx
14081b67a: 48 c1 e1 04                 	shlq	$0x4, %rcx
14081b67e: 49 01 cd                    	addq	%rcx, %r13
14081b681: 49 c1 e7 04                 	shlq	$0x4, %r15
14081b685: 31 c9                       	xorl	%ecx, %ecx
14081b687: 4d 39 c3                    	cmpq	%r8, %r11
14081b68a: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14081b690: 4d 0f 42 e8                 	cmovbq	%r8, %r13
14081b694: 4f 8d 5c 3d f0              	leaq	-0x10(%r13,%r15), %r11
14081b699: 49 0f 47 cb                 	cmovaq	%r11, %rcx
14081b69d: 49 39 d2                    	cmpq	%rdx, %r10
14081b6a0: 4d 0f 42 e0                 	cmovbq	%r8, %r12
14081b6a4: 4b 8d 54 0c f0              	leaq	-0x10(%r12,%r9), %rdx
14081b6a9: 48 0f 46 d1                 	cmovbeq	%rcx, %rdx
14081b6ad: 4c 89 30                    	movq	%r14, (%rax)
14081b6b0: 4c 8b 42 08                 	movq	0x8(%rdx), %r8
14081b6b4: 48 89 f9                    	movq	%rdi, %rcx
14081b6b7: 48 89 f2                    	movq	%rsi, %rdx
14081b6ba: e8 61 02 00 00              	callq	0x14081b920 <.text+0x81a920>
14081b6bf: 48 8b 4c 24 28              	movq	0x28(%rsp), %rcx
14081b6c4: 48 8b 01                    	movq	(%rcx), %rax
14081b6c7: 4c 8b 59 08                 	movq	0x8(%rcx), %r11
14081b6cb: 48 8b 08                    	movq	(%rax), %rcx
14081b6ce: 31 db                       	xorl	%ebx, %ebx
14081b6d0: 48 89 ca                    	movq	%rcx, %rdx
14081b6d3: 49 2b 53 18                 	subq	0x18(%r11), %rdx
14081b6d7: 48 0f 42 d3                 	cmovbq	%rbx, %rdx
14081b6db: 49 2b 4b 38                 	subq	0x38(%r11), %rcx
14081b6df: 48 0f 42 cb                 	cmovbq	%rbx, %rcx
14081b6e3: 4d 8b 53 08                 	movq	0x8(%r11), %r10
14081b6e7: 4d 8b 43 30                 	movq	0x30(%r11), %r8
14081b6eb: 4c 89 44 24 28              	movq	%r8, 0x28(%rsp)
14081b6f0: 49 29 c8                    	subq	%rcx, %r8
14081b6f3: 4c 0f 42 c3                 	cmovbq	%rbx, %r8
14081b6f7: 4d 8b 6b 10                 	movq	0x10(%r11), %r13
14081b6fb: 41 0f 96 c7                 	setbe	%r15b
14081b6ff: 4d 89 ec                    	movq	%r13, %r12
14081b702: 49 29 d4                    	subq	%rdx, %r12
14081b705: 4c 0f 42 e3                 	cmovbq	%rbx, %r12
14081b709: 4d 8b 73 28                 	movq	0x28(%r11), %r14
14081b70d: 4d 8b 4b 40                 	movq	0x40(%r11), %r9
14081b711: 0f 96 c3                    	setbe	%bl
14081b714: 4d 89 cb                    	movq	%r9, %r11
14081b717: 4d 29 c3                    	subq	%r8, %r11
14081b71a: 4d 29 e3                    	subq	%r12, %r11
14081b71d: 4c 89 18                    	movq	%r11, (%rax)
14081b720: 44 84 fb                    	testb	%r15b, %bl
14081b723: 0f 84 cf 00 00 00           	je	0x14081b7f8 <.text+0x81a7f8>
14081b729: 80 7c 24 27 00              	cmpb	$0x0, 0x27(%rsp)
14081b72e: 0f 84 1c 01 00 00           	je	0x14081b850 <.text+0x81a850>
14081b734: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
14081b739: 4c 8b 30                    	movq	(%rax), %r14
14081b73c: 48 8b 48 08                 	movq	0x8(%rax), %rcx
14081b740: 4d 8b 06                    	movq	(%r14), %r8
14081b743: 31 d2                       	xorl	%edx, %edx
14081b745: 4d 89 c1                    	movq	%r8, %r9
14081b748: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
14081b74c: 4c 0f 42 ca                 	cmovbq	%rdx, %r9
14081b750: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
14081b754: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
14081b758: 4b 8d 04 49                 	leaq	(%r9,%r9,2), %rax
14081b75c: 48 c1 e0 04                 	shlq	$0x4, %rax
14081b760: 48 03 41 08                 	addq	0x8(%rcx), %rax
14081b764: 4c 8b 51 10                 	movq	0x10(%rcx), %r10
14081b768: 4d 29 ca                    	subq	%r9, %r10
14081b76b: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
14081b76f: 41 b9 08 00 00 00           	movl	$0x8, %r9d
14081b775: 49 0f 42 c1                 	cmovbq	%r9, %rax
14081b779: 4b 8d 1c 40                 	leaq	(%r8,%r8,2), %rbx
14081b77d: 48 c1 e3 04                 	shlq	$0x4, %rbx
14081b781: 48 03 59 28                 	addq	0x28(%rcx), %rbx
14081b785: 4c 8b 59 30                 	movq	0x30(%rcx), %r11
14081b789: 4d 29 c3                    	subq	%r8, %r11
14081b78c: 4c 0f 42 da                 	cmovbq	%rdx, %r11
14081b790: 49 0f 42 d9                 	cmovbq	%r9, %rbx
14081b794: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
14081b798: 4c 29 d9                    	subq	%r11, %rcx
14081b79b: 4c 29 d1                    	subq	%r10, %rcx
14081b79e: 49 89 0e                    	movq	%rcx, (%r14)
14081b7a1: 4f 8d 24 52                 	leaq	(%r10,%r10,2), %r12
14081b7a5: 49 c1 e4 04                 	shlq	$0x4, %r12
14081b7a9: 49 01 c4                    	addq	%rax, %r12
14081b7ac: 4f 8d 2c 5b                 	leaq	(%r11,%r11,2), %r13
14081b7b0: 49 c1 e5 04                 	shlq	$0x4, %r13
14081b7b4: 49 01 dd                    	addq	%rbx, %r13
14081b7b7: eb 20                       	jmp	0x14081b7d9 <.text+0x81a7d9>
14081b7b9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14081b7c0: 48 8d 68 30                 	leaq	0x30(%rax), %rbp
14081b7c4: 49 ff 06                    	incq	(%r14)
14081b7c7: 4c 8b 40 18                 	movq	0x18(%rax), %r8
14081b7cb: 48 89 f9                    	movq	%rdi, %rcx
14081b7ce: 48 89 f2                    	movq	%rsi, %rdx
14081b7d1: e8 4a 01 00 00              	callq	0x14081b920 <.text+0x81a920>
14081b7d6: 48 89 e8                    	movq	%rbp, %rax
14081b7d9: 48 85 c0                    	testq	%rax, %rax
14081b7dc: 74 05                       	je	0x14081b7e3 <.text+0x81a7e3>
14081b7de: 4c 39 e0                    	cmpq	%r12, %rax
14081b7e1: 75 dd                       	jne	0x14081b7c0 <.text+0x81a7c0>
14081b7e3: 48 85 db                    	testq	%rbx, %rbx
14081b7e6: 74 68                       	je	0x14081b850 <.text+0x81a850>
14081b7e8: 4c 39 eb                    	cmpq	%r13, %rbx
14081b7eb: 74 63                       	je	0x14081b850 <.text+0x81a850>
14081b7ed: 48 89 d8                    	movq	%rbx, %rax
14081b7f0: 48 83 c3 30                 	addq	$0x30, %rbx
14081b7f4: 31 ed                       	xorl	%ebp, %ebp
14081b7f6: eb cc                       	jmp	0x14081b7c4 <.text+0x81a7c4>
14081b7f8: 49 89 cb                    	movq	%rcx, %r11
14081b7fb: 49 c1 e3 04                 	shlq	$0x4, %r11
14081b7ff: 4d 01 de                    	addq	%r11, %r14
14081b802: 49 c1 e0 04                 	shlq	$0x4, %r8
14081b806: 49 89 d3                    	movq	%rdx, %r11
14081b809: 49 c1 e3 04                 	shlq	$0x4, %r11
14081b80d: 4d 01 da                    	addq	%r11, %r10
14081b810: 49 c1 e4 04                 	shlq	$0x4, %r12
14081b814: 45 31 db                    	xorl	%r11d, %r11d
14081b817: 49 39 d5                    	cmpq	%rdx, %r13
14081b81a: ba 08 00 00 00              	movl	$0x8, %edx
14081b81f: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
14081b823: 4f 8d 54 22 f0              	leaq	-0x10(%r10,%r12), %r10
14081b828: 4d 0f 47 da                 	cmovaq	%r10, %r11
14081b82c: 48 39 4c 24 28              	cmpq	%rcx, 0x28(%rsp)
14081b831: 4c 0f 42 f2                 	cmovbq	%rdx, %r14
14081b835: 4b 8d 4c 06 f0              	leaq	-0x10(%r14,%r8), %rcx
14081b83a: 49 0f 46 cb                 	cmovbeq	%r11, %rcx
14081b83e: 4c 89 08                    	movq	%r9, (%rax)
14081b841: 4c 8b 41 08                 	movq	0x8(%rcx), %r8
14081b845: 48 89 f9                    	movq	%rdi, %rcx
14081b848: 48 89 f2                    	movq	%rsi, %rdx
14081b84b: e8 d0 00 00 00              	callq	0x14081b920 <.text+0x81a920>
14081b850: 48 8b 84 24 a0 00 00 00     	movq	0xa0(%rsp), %rax
14081b858: 48 8b 18                    	movq	(%rax), %rbx
14081b85b: 48 8b 48 08                 	movq	0x8(%rax), %rcx
14081b85f: 48 8b 13                    	movq	(%rbx), %rdx
14081b862: 45 31 c0                    	xorl	%r8d, %r8d
14081b865: 49 89 d1                    	movq	%rdx, %r9
14081b868: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
14081b86c: 4d 0f 42 c8                 	cmovbq	%r8, %r9
14081b870: 48 2b 51 38                 	subq	0x38(%rcx), %rdx
14081b874: 49 0f 42 d0                 	cmovbq	%r8, %rdx
14081b878: 4c 89 c8                    	movq	%r9, %rax
14081b87b: 48 c1 e0 04                 	shlq	$0x4, %rax
14081b87f: 48 03 41 08                 	addq	0x8(%rcx), %rax
14081b883: 4c 8b 71 10                 	movq	0x10(%rcx), %r14
14081b887: 4c 8b 79 30                 	movq	0x30(%rcx), %r15
14081b88b: 4d 29 ce                    	subq	%r9, %r14
14081b88e: 4d 0f 42 f0                 	cmovbq	%r8, %r14
14081b892: 41 b9 08 00 00 00           	movl	$0x8, %r9d
14081b898: 49 0f 42 c1                 	cmovbq	%r9, %rax
14081b89c: 49 89 d4                    	movq	%rdx, %r12
14081b89f: 49 c1 e4 04                 	shlq	$0x4, %r12
14081b8a3: 4c 03 61 28                 	addq	0x28(%rcx), %r12
14081b8a7: 49 29 d7                    	subq	%rdx, %r15
14081b8aa: 4d 0f 42 f8                 	cmovbq	%r8, %r15
14081b8ae: 4d 0f 42 e1                 	cmovbq	%r9, %r12
14081b8b2: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
14081b8b6: 4c 29 f9                    	subq	%r15, %rcx
14081b8b9: 4c 29 f1                    	subq	%r14, %rcx
14081b8bc: 48 89 0b                    	movq	%rcx, (%rbx)
14081b8bf: 49 c1 e6 04                 	shlq	$0x4, %r14
14081b8c3: 49 01 c6                    	addq	%rax, %r14
14081b8c6: 49 c1 e7 04                 	shlq	$0x4, %r15
14081b8ca: 4d 01 e7                    	addq	%r12, %r15
14081b8cd: eb 1a                       	jmp	0x14081b8e9 <.text+0x81a8e9>
14081b8cf: 90                          	nop
14081b8d0: 4c 8d 68 10                 	leaq	0x10(%rax), %r13
14081b8d4: 48 ff 03                    	incq	(%rbx)
14081b8d7: 4c 8b 40 08                 	movq	0x8(%rax), %r8
14081b8db: 48 89 f9                    	movq	%rdi, %rcx
14081b8de: 48 89 f2                    	movq	%rsi, %rdx
14081b8e1: e8 3a 00 00 00              	callq	0x14081b920 <.text+0x81a920>
14081b8e6: 4c 89 e8                    	movq	%r13, %rax
14081b8e9: 48 85 c0                    	testq	%rax, %rax
14081b8ec: 74 05                       	je	0x14081b8f3 <.text+0x81a8f3>
14081b8ee: 4c 39 f0                    	cmpq	%r14, %rax
14081b8f1: 75 dd                       	jne	0x14081b8d0 <.text+0x81a8d0>
14081b8f3: 4d 85 e4                    	testq	%r12, %r12
14081b8f6: 74 11                       	je	0x14081b909 <.text+0x81a909>
14081b8f8: 4d 39 fc                    	cmpq	%r15, %r12
14081b8fb: 74 0c                       	je	0x14081b909 <.text+0x81a909>
14081b8fd: 4c 89 e0                    	movq	%r12, %rax
14081b900: 49 83 c4 10                 	addq	$0x10, %r12
14081b904: 45 31 ed                    	xorl	%r13d, %r13d
14081b907: eb cb                       	jmp	0x14081b8d4 <.text+0x81a8d4>
14081b909: 48 83 c4 38                 	addq	$0x38, %rsp
14081b90d: 5b                          	popq	%rbx
14081b90e: 5d                          	popq	%rbp
14081b90f: 5f                          	popq	%rdi
14081b910: 5e                          	popq	%rsi
14081b911: 41 5c                       	popq	%r12
14081b913: 41 5d                       	popq	%r13
14081b915: 41 5e                       	popq	%r14
14081b917: 41 5f                       	popq	%r15
14081b919: c3                          	retq
14081b91a: cc                          	int3
14081b91b: cc                          	int3
14081b91c: cc                          	int3
14081b91d: cc                          	int3
14081b91e: cc                          	int3
14081b91f: cc                          	int3
14081b920: 41 57                       	pushq	%r15
14081b922: 41 56                       	pushq	%r14
14081b924: 41 55                       	pushq	%r13
14081b926: 41 54                       	pushq	%r12
14081b928: 56                          	pushq	%rsi
14081b929: 57                          	pushq	%rdi
14081b92a: 55                          	pushq	%rbp
14081b92b: 53                          	pushq	%rbx
14081b92c: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
14081b933: 4c 89 c6                    	movq	%r8, %rsi
14081b936: 48 89 d7                    	movq	%rdx, %rdi
14081b939: 8b 41 1c                    	movl	0x1c(%rcx), %eax
14081b93c: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
14081b940: 89 02                       	movl	%eax, (%rdx)
14081b942: 48 8b 09                    	movq	(%rcx), %rcx
14081b945: 4c 89 c2                    	movq	%r8, %rdx
14081b948: e8 d3 2a 20 00              	callq	0x140a1e420 <.text+0xa1d420>
14081b94d: 48 85 c0                    	testq	%rax, %rax
14081b950: 0f 84 24 01 00 00           	je	0x14081ba7a <.text+0x81aa7a>
14081b956: 48 8b 48 30                 	movq	0x30(%rax), %rcx
14081b95a: 48 8b 50 38                 	movq	0x38(%rax), %rdx
14081b95e: 48 01 ca                    	addq	%rcx, %rdx
14081b961: 48 ff c2                    	incq	%rdx
14081b964: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
14081b968: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
14081b96d: 41 f7 d0                    	notl	%r8d
14081b970: 66 44 89 44 24 38           	movw	%r8w, 0x38(%rsp)
14081b976: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14081b97b: 48 83 c1 10                 	addq	$0x10, %rcx
14081b97f: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
14081b984: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
14081b989: 48 8b 48 48                 	movq	0x48(%rax), %rcx
14081b98d: 48 89 4c 24 40              	movq	%rcx, 0x40(%rsp)
14081b992: 48 83 c0 60                 	addq	$0x60, %rax
14081b996: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
14081b99b: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
14081b9a0: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
14081b9a5: e8 56 f8 1f 00              	callq	0x140a1b200 <.text+0xa1a200>
14081b9aa: 48 83 7c 24 58 00           	cmpq	$0x0, 0x58(%rsp)
14081b9b0: 0f 84 c4 00 00 00           	je	0x14081ba7a <.text+0x81aa7a>
14081b9b6: 4c 8d 74 24 58              	leaq	0x58(%rsp), %r14
14081b9bb: 48 8d 5c 24 70              	leaq	0x70(%rsp), %rbx
14081b9c0: 45 31 ff                    	xorl	%r15d, %r15d
14081b9c3: 4c 8d 64 24 50              	leaq	0x50(%rsp), %r12
14081b9c8: 4c 8d 6c 24 20              	leaq	0x20(%rsp), %r13
14081b9cd: eb 22                       	jmp	0x14081b9f1 <.text+0x81a9f1>
14081b9cf: 90                          	nop
14081b9d0: 48 83 f8 01                 	cmpq	$0x1, %rax
14081b9d4: 0f 85 b4 00 00 00           	jne	0x14081ba8e <.text+0x81aa8e>
14081b9da: 4c 89 e1                    	movq	%r12, %rcx
14081b9dd: 4c 89 ea                    	movq	%r13, %rdx
14081b9e0: e8 1b f8 1f 00              	callq	0x140a1b200 <.text+0xa1a200>
14081b9e5: 48 83 7c 24 58 00           	cmpq	$0x0, 0x58(%rsp)
14081b9eb: 0f 84 89 00 00 00           	je	0x14081ba7a <.text+0x81aa7a>
14081b9f1: 49 8b 46 10                 	movq	0x10(%r14), %rax
14081b9f5: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
14081b9fd: 41 0f 10 06                 	movups	(%r14), %xmm0
14081ba01: 0f 29 44 24 70              	movaps	%xmm0, 0x70(%rsp)
14081ba06: 48 89 d9                    	movq	%rbx, %rcx
14081ba09: e8 c2 cb 86 ff              	callq	0x1400885d0 <.text+0x875d0>
14081ba0e: 0f b6 40 22                 	movzbl	0x22(%rax), %eax
14081ba12: 8d 48 fe                    	leal	-0x2(%rax), %ecx
14081ba15: 48 ff c8                    	decq	%rax
14081ba18: 80 f9 03                    	cmpb	$0x3, %cl
14081ba1b: 49 0f 43 c7                 	cmovaeq	%r15, %rax
14081ba1f: 48 85 c0                    	testq	%rax, %rax
14081ba22: 75 ac                       	jne	0x14081b9d0 <.text+0x81a9d0>
14081ba24: 48 89 d9                    	movq	%rbx, %rcx
14081ba27: e8 b4 f7 1f 00              	callq	0x140a1b1e0 <.text+0xa1a1e0>
14081ba2c: 48 89 c1                    	movq	%rax, %rcx
14081ba2f: 48 8d 15 72 49 27 02        	leaq	0x2274972(%rip), %rdx   # 0x142a903a8
14081ba36: e8 85 78 01 00              	callq	0x1408332c0 <.text+0x8322c0>
14081ba3b: 48 89 c5                    	movq	%rax, %rbp
14081ba3e: 48 8b 0f                    	movq	(%rdi), %rcx
14081ba41: 48 89 f2                    	movq	%rsi, %rdx
14081ba44: 4c 8d 05 75 49 27 02        	leaq	0x2274975(%rip), %r8    # 0x142a903c0
14081ba4b: e8 a0 64 27 00              	callq	0x140a91ef0 <.text+0xa90ef0>
14081ba50: 48 85 c0                    	testq	%rax, %rax
14081ba53: 74 51                       	je	0x14081baa6 <.text+0x81aaa6>
14081ba55: f3 0f 10 4d 00              	movss	(%rbp), %xmm1
14081ba5a: f3 0f 10 55 04              	movss	0x4(%rbp), %xmm2
14081ba5f: 48 89 c1                    	movq	%rax, %rcx
14081ba62: e8 29 ee 30 00              	callq	0x140b2a890 <.text+0xb29890>
14081ba67: f3 0f 10 4d 04              	movss	0x4(%rbp), %xmm1
14081ba6c: f3 0f 5c c8                 	subss	%xmm0, %xmm1
14081ba70: f3 0f 11 4d 10              	movss	%xmm1, 0x10(%rbp)
14081ba75: e9 60 ff ff ff              	jmp	0x14081b9da <.text+0x81a9da>
14081ba7a: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
14081ba81: 5b                          	popq	%rbx
14081ba82: 5d                          	popq	%rbp
14081ba83: 5f                          	popq	%rdi
14081ba84: 5e                          	popq	%rsi
14081ba85: 41 5c                       	popq	%r12
14081ba87: 41 5d                       	popq	%r13
14081ba89: 41 5e                       	popq	%r14
14081ba8b: 41 5f                       	popq	%r15
14081ba8d: c3                          	retq
14081ba8e: 48 8d 0d 5b 49 27 02        	leaq	0x227495b(%rip), %rcx   # 0x142a903f0
14081ba95: 4c 8d 05 7c 49 27 02        	leaq	0x227497c(%rip), %r8    # 0x142a90418
14081ba9c: ba 28 00 00 00              	movl	$0x28, %edx
14081baa1: e8 8a d9 0b 02              	callq	0x1428d9430 <.text+0x28d8430>
14081baa6: 48 8d 0d 2b 49 27 02        	leaq	0x227492b(%rip), %rcx   # 0x142a903d8
14081baad: e8 5e d8 0b 02              	callq	0x1428d9310 <.text+0x28d8310>
14081bab2: cc                          	int3
14081bab3: cc                          	int3
14081bab4: cc                          	int3
14081bab5: cc                          	int3
14081bab6: cc                          	int3
14081bab7: cc                          	int3
14081bab8: cc                          	int3
14081bab9: cc                          	int3
