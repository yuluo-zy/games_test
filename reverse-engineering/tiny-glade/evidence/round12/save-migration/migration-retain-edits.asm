
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412f50f0: 55                          	pushq	%rbp
1412f50f1: 41 56                       	pushq	%r14
1412f50f3: 56                          	pushq	%rsi
1412f50f4: 57                          	pushq	%rdi
1412f50f5: 53                          	pushq	%rbx
1412f50f6: 48 83 ec 50                 	subq	$0x50, %rsp
1412f50fa: 48 8d 6c 24 50              	leaq	0x50(%rsp), %rbp
1412f50ff: 48 c7 45 f8 fe ff ff ff     	movq	$-0x2, -0x8(%rbp)
1412f5107: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1412f510b: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1412f510f: 48 85 c0                    	testq	%rax, %rax
1412f5112: 74 5a                       	je	0x1412f516e <.text+0x12f416e>
1412f5114: 4c 89 c7                    	movq	%r8, %rdi
1412f5117: 48 89 d6                    	movq	%rdx, %rsi
1412f511a: 48 c7 41 10 00 00 00 00     	movq	$0x0, 0x10(%rcx)
1412f5122: 4c 8b 41 08                 	movq	0x8(%rcx), %r8
1412f5126: 49 8b 10                    	movq	(%r8), %rdx
1412f5129: 48 83 fa 0b                 	cmpq	$0xb, %rdx
1412f512d: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
1412f5131: 75 46                       	jne	0x1412f5179 <.text+0x12f4179>
1412f5133: 31 db                       	xorl	%ebx, %ebx
1412f5135: 4c 89 c1                    	movq	%r8, %rcx
1412f5138: 4c 89 45 e0                 	movq	%r8, -0x20(%rbp)
1412f513c: c6 06 01                    	movb	$0x1, (%rsi)
1412f513f: 48 ff c3                    	incq	%rbx
1412f5142: 48 c7 45 d8 01 00 00 00     	movq	$0x1, -0x28(%rbp)
1412f514a: 48 89 5d d0                 	movq	%rbx, -0x30(%rbp)
1412f514e: e8 1d e0 ff ff              	callq	0x1412f3170 <.text+0x12f2170>
1412f5153: 41 be 01 00 00 00           	movl	$0x1, %r14d
1412f5159: 48 3b 5d f0                 	cmpq	-0x10(%rbp), %rbx
1412f515d: 75 7e                       	jne	0x1412f51dd <.text+0x12f41dd>
1412f515f: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1412f5163: 4c 29 f1                    	subq	%r14, %rcx
1412f5166: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
1412f516a: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
1412f516e: 48 83 c4 50                 	addq	$0x50, %rsp
1412f5172: 5b                          	popq	%rbx
1412f5173: 5f                          	popq	%rdi
1412f5174: 5e                          	popq	%rsi
1412f5175: 41 5e                       	popq	%r14
1412f5177: 5d                          	popq	%rbp
1412f5178: c3                          	retq
1412f5179: 83 3f 10                    	cmpl	$0x10, (%rdi)
1412f517c: 0f 8d f1 00 00 00           	jge	0x1412f5273 <.text+0x12f4273>
1412f5182: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1412f5186: 48 ff c8                    	decq	%rax
1412f5189: 45 31 f6                    	xorl	%r14d, %r14d
1412f518c: 4c 89 c1                    	movq	%r8, %rcx
1412f518f: 31 db                       	xorl	%ebx, %ebx
1412f5191: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1412f51a0: 48 83 fa 09                 	cmpq	$0x9, %rdx
1412f51a4: 74 0a                       	je	0x1412f51b0 <.text+0x12f41b0>
1412f51a6: 48 83 fa 0a                 	cmpq	$0xa, %rdx
1412f51aa: 75 0e                       	jne	0x1412f51ba <.text+0x12f41ba>
1412f51ac: eb 8a                       	jmp	0x1412f5138 <.text+0x12f4138>
1412f51ae: 66 90                       	nop
1412f51b0: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
1412f51b4: 80 7a 18 07                 	cmpb	$0x7, 0x18(%rdx)
1412f51b8: 74 16                       	je	0x1412f51d0 <.text+0x12f41d0>
1412f51ba: 48 39 d8                    	cmpq	%rbx, %rax
1412f51bd: 74 a0                       	je	0x1412f515f <.text+0x12f415f>
1412f51bf: 48 ff c3                    	incq	%rbx
1412f51c2: 48 8b 51 18                 	movq	0x18(%rcx), %rdx
1412f51c6: 48 83 c1 18                 	addq	$0x18, %rcx
1412f51ca: 48 83 fa 0b                 	cmpq	$0xb, %rdx
1412f51ce: 75 d0                       	jne	0x1412f51a0 <.text+0x12f41a0>
1412f51d0: 48 8d 04 5b                 	leaq	(%rbx,%rbx,2), %rax
1412f51d4: 49 8d 0c c0                 	leaq	(%r8,%rax,8), %rcx
1412f51d8: e9 5b ff ff ff              	jmp	0x1412f5138 <.text+0x12f4138>
1412f51dd: 8b 3f                       	movl	(%rdi), %edi
1412f51df: eb 2f                       	jmp	0x1412f5210 <.text+0x12f4210>
1412f51e1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1412f51f0: c6 06 01                    	movb	$0x1, (%rsi)
1412f51f3: 48 ff c3                    	incq	%rbx
1412f51f6: 49 ff c6                    	incq	%r14
1412f51f9: 4c 89 75 d8                 	movq	%r14, -0x28(%rbp)
1412f51fd: 48 89 5d d0                 	movq	%rbx, -0x30(%rbp)
1412f5201: e8 6a df ff ff              	callq	0x1412f3170 <.text+0x12f2170>
1412f5206: 48 3b 5d f0                 	cmpq	-0x10(%rbp), %rbx
1412f520a: 0f 84 4f ff ff ff           	je	0x1412f515f <.text+0x12f415f>
1412f5210: 48 8d 14 5b                 	leaq	(%rbx,%rbx,2), %rdx
1412f5214: 4c 8b 45 e0                 	movq	-0x20(%rbp), %r8
1412f5218: 49 8d 0c d0                 	leaq	(%r8,%rdx,8), %rcx
1412f521c: 49 8b 04 d0                 	movq	(%r8,%rdx,8), %rax
1412f5220: 48 83 f8 0b                 	cmpq	$0xb, %rax
1412f5224: 74 ca                       	je	0x1412f51f0 <.text+0x12f41f0>
1412f5226: 83 ff 0f                    	cmpl	$0xf, %edi
1412f5229: 7f 15                       	jg	0x1412f5240 <.text+0x12f4240>
1412f522b: 83 f8 09                    	cmpl	$0x9, %eax
1412f522e: 75 10                       	jne	0x1412f5240 <.text+0x12f4240>
1412f5230: 49 8b 44 d0 08              	movq	0x8(%r8,%rdx,8), %rax
1412f5235: 80 78 18 07                 	cmpb	$0x7, 0x18(%rax)
1412f5239: 74 b5                       	je	0x1412f51f0 <.text+0x12f41f0>
1412f523b: eb 0d                       	jmp	0x1412f524a <.text+0x12f424a>
1412f523d: 0f 1f 00                    	nopl	(%rax)
1412f5240: 83 ff 0f                    	cmpl	$0xf, %edi
1412f5243: 7f 05                       	jg	0x1412f524a <.text+0x12f424a>
1412f5245: 83 f8 0a                    	cmpl	$0xa, %eax
1412f5248: 74 a6                       	je	0x1412f51f0 <.text+0x12f41f0>
1412f524a: 48 89 d8                    	movq	%rbx, %rax
1412f524d: 4c 29 f0                    	subq	%r14, %rax
1412f5250: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1412f5254: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
1412f5258: 49 89 54 c0 10              	movq	%rdx, 0x10(%r8,%rax,8)
1412f525d: 0f 10 01                    	movups	(%rcx), %xmm0
1412f5260: 41 0f 11 04 c0              	movups	%xmm0, (%r8,%rax,8)
1412f5265: 48 ff c3                    	incq	%rbx
1412f5268: 48 3b 5d f0                 	cmpq	-0x10(%rbp), %rbx
1412f526c: 75 a2                       	jne	0x1412f5210 <.text+0x12f4210>
1412f526e: e9 ec fe ff ff              	jmp	0x1412f515f <.text+0x12f415f>
1412f5273: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1412f5277: 48 ff c8                    	decq	%rax
1412f527a: 45 31 f6                    	xorl	%r14d, %r14d
1412f527d: 4c 89 c2                    	movq	%r8, %rdx
1412f5280: 31 db                       	xorl	%ebx, %ebx
1412f5282: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1412f5290: 48 39 d8                    	cmpq	%rbx, %rax
1412f5293: 0f 84 c6 fe ff ff           	je	0x1412f515f <.text+0x12f415f>
1412f5299: 48 8d 4a 18                 	leaq	0x18(%rdx), %rcx
1412f529d: 48 ff c3                    	incq	%rbx
1412f52a0: 83 7a 18 0b                 	cmpl	$0xb, 0x18(%rdx)
1412f52a4: 48 89 ca                    	movq	%rcx, %rdx
1412f52a7: 75 e7                       	jne	0x1412f5290 <.text+0x12f4290>
1412f52a9: e9 8a fe ff ff              	jmp	0x1412f5138 <.text+0x12f4138>
1412f52ae: 66 90                       	nop
1412f52b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412f52b5: 55                          	pushq	%rbp
1412f52b6: 41 56                       	pushq	%r14
1412f52b8: 56                          	pushq	%rsi
1412f52b9: 57                          	pushq	%rdi
1412f52ba: 53                          	pushq	%rbx
1412f52bb: 48 83 ec 20                 	subq	$0x20, %rsp
1412f52bf: 48 8d 6a 50                 	leaq	0x50(%rdx), %rbp
1412f52c3: 48 8b 75 d8                 	movq	-0x28(%rbp), %rsi
1412f52c7: 48 85 f6                    	testq	%rsi, %rsi
1412f52ca: 74 2f                       	je	0x1412f52fb <.text+0x12f42fb>
1412f52cc: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1412f52d0: 48 8d 0c 40                 	leaq	(%rax,%rax,2), %rcx
1412f52d4: 4c 8b 4d e0                 	movq	-0x20(%rbp), %r9
1412f52d8: 49 8d 14 c9                 	leaq	(%r9,%rcx,8), %rdx
1412f52dc: 4c 8b 45 f0                 	movq	-0x10(%rbp), %r8
1412f52e0: 49 29 c0                    	subq	%rax, %r8
1412f52e3: 48 29 f0                    	subq	%rsi, %rax
1412f52e6: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1412f52ea: 49 8d 0c c1                 	leaq	(%r9,%rax,8), %rcx
1412f52ee: 49 c1 e0 03                 	shlq	$0x3, %r8
1412f52f2: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
1412f52f6: e8 15 e9 62 01              	callq	0x142923c10 <.text+0x2922c10>
1412f52fb: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1412f52ff: 48 29 f1                    	subq	%rsi, %rcx
1412f5302: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
1412f5306: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
1412f530a: 48 83 c4 20                 	addq	$0x20, %rsp
1412f530e: 5b                          	popq	%rbx
1412f530f: 5f                          	popq	%rdi
1412f5310: 5e                          	popq	%rsi
1412f5311: 41 5e                       	popq	%r14
1412f5313: 5d                          	popq	%rbp
1412f5314: c3                          	retq
