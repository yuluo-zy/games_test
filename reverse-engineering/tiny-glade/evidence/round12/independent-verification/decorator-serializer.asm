
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1402de600: 41 56                       	pushq	%r14
1402de602: 56                          	pushq	%rsi
1402de603: 57                          	pushq	%rdi
1402de604: 53                          	pushq	%rbx
1402de605: 48 83 ec 38                 	subq	$0x38, %rsp
1402de609: 48 89 d6                    	movq	%rdx, %rsi
1402de60c: 0f b6 01                    	movzbl	(%rcx), %eax
1402de60f: 04 c8                       	addb	$-0x38, %al
1402de611: 3c 08                       	cmpb	$0x8, %al
1402de613: 0f b6 c0                    	movzbl	%al, %eax
1402de616: ba 06 00 00 00              	movl	$0x6, %edx
1402de61b: 0f 42 d0                    	cmovbl	%eax, %edx
1402de61e: 0f b6 c2                    	movzbl	%dl, %eax
1402de621: 48 8d 15 38 4e 6d 02        	leaq	0x26d4e38(%rip), %rdx   # 0x1429b3460
1402de628: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
1402de62c: 48 01 d0                    	addq	%rdx, %rax
1402de62f: ff e0                       	jmpq	*%rax
1402de631: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1402de635: 4c 8d 05 94 4a 6d 02        	leaq	0x26d4a94(%rip), %r8    # 0x1429b30d0
1402de63c: 41 b9 0d 00 00 00           	movl	$0xd, %r9d
1402de642: eb 70                       	jmp	0x1402de6b4 <.text+0x2dd6b4>
1402de644: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1402de648: 4c 8d 05 aa 4a 6d 02        	leaq	0x26d4aaa(%rip), %r8    # 0x1429b30f9
1402de64f: 41 b9 07 00 00 00           	movl	$0x7, %r9d
1402de655: eb 5d                       	jmp	0x1402de6b4 <.text+0x2dd6b4>
1402de657: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1402de65b: 4c 8d 05 87 4a 6d 02        	leaq	0x26d4a87(%rip), %r8    # 0x1429b30e9
1402de662: 41 b9 09 00 00 00           	movl	$0x9, %r9d
1402de668: eb 4a                       	jmp	0x1402de6b4 <.text+0x2dd6b4>
1402de66a: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1402de66e: 4c 8d 05 7d 4a 6d 02        	leaq	0x26d4a7d(%rip), %r8    # 0x1429b30f2
1402de675: 41 b9 07 00 00 00           	movl	$0x7, %r9d
1402de67b: eb 37                       	jmp	0x1402de6b4 <.text+0x2dd6b4>
1402de67d: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1402de681: 4c 8d 05 85 4a 6d 02        	leaq	0x26d4a85(%rip), %r8    # 0x1429b310d
1402de688: 41 b9 06 00 00 00           	movl	$0x6, %r9d
1402de68e: eb 24                       	jmp	0x1402de6b4 <.text+0x2dd6b4>
1402de690: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1402de694: 4c 8d 05 42 4a 6d 02        	leaq	0x26d4a42(%rip), %r8    # 0x1429b30dd
1402de69b: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
1402de6a1: eb 11                       	jmp	0x1402de6b4 <.text+0x2dd6b4>
1402de6a3: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1402de6a7: 4c 8d 05 52 4a 6d 02        	leaq	0x26d4a52(%rip), %r8    # 0x1429b3100
1402de6ae: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1402de6b4: 48 89 f1                    	movq	%rsi, %rcx
1402de6b7: e8 44 7a 1d 00              	callq	0x1404b6100 <.text+0x4b5100>
1402de6bc: 48 85 c0                    	testq	%rax, %rax
1402de6bf: 0f 85 e8 00 00 00           	jne	0x1402de7ad <.text+0x2dd7ad>
1402de6c5: 31 c0                       	xorl	%eax, %eax
1402de6c7: 48 83 c4 38                 	addq	$0x38, %rsp
1402de6cb: 5b                          	popq	%rbx
1402de6cc: 5f                          	popq	%rdi
1402de6cd: 5e                          	popq	%rsi
1402de6ce: 41 5e                       	popq	%r14
1402de6d0: c3                          	retq
1402de6d1: 48 89 cf                    	movq	%rcx, %rdi
1402de6d4: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1402de6d8: 48 8b 1e                    	movq	(%rsi), %rbx
1402de6db: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1402de6df: 48 39 03                    	cmpq	%rax, (%rbx)
1402de6e2: 0f 84 04 01 00 00           	je	0x1402de7ec <.text+0x2dd7ec>
1402de6e8: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1402de6ec: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1402de6f0: 48 ff c0                    	incq	%rax
1402de6f3: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1402de6f7: 4c 8d 05 06 4a 6d 02        	leaq	0x26d4a06(%rip), %r8    # 0x1429b3104
1402de6fe: 41 b9 07 00 00 00           	movl	$0x7, %r9d
1402de704: 48 89 f1                    	movq	%rsi, %rcx
1402de707: e8 f4 79 1d 00              	callq	0x1404b6100 <.text+0x4b5100>
1402de70c: 48 85 c0                    	testq	%rax, %rax
1402de70f: 0f 85 ca 00 00 00           	jne	0x1402de7df <.text+0x2dd7df>
1402de715: 48 8b 1e                    	movq	(%rsi), %rbx
1402de718: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1402de71c: 48 39 13                    	cmpq	%rdx, (%rbx)
1402de71f: 0f 84 f6 00 00 00           	je	0x1402de81b <.text+0x2dd81b>
1402de725: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1402de729: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1402de72d: 48 ff c2                    	incq	%rdx
1402de730: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1402de734: 48 8b 1e                    	movq	(%rsi), %rbx
1402de737: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1402de73b: 48 39 13                    	cmpq	%rdx, (%rbx)
1402de73e: 0f 84 fd 00 00 00           	je	0x1402de841 <.text+0x2dd841>
1402de744: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1402de748: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1402de74c: 48 ff c2                    	incq	%rdx
1402de74f: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1402de753: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
1402de758: c6 44 24 30 01              	movb	$0x1, 0x30(%rsp)
1402de75d: 48 8d 15 a7 49 6d 02        	leaq	0x26d49a7(%rip), %rdx   # 0x1429b310b
1402de764: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
1402de769: 41 b8 02 00 00 00           	movl	$0x2, %r8d
1402de76f: 49 89 f9                    	movq	%rdi, %r9
1402de772: e8 69 28 1e 00              	callq	0x1404c0fe0 <.text+0x4bffe0>
1402de777: 48 85 c0                    	testq	%rax, %rax
1402de77a: 0f 85 47 ff ff ff           	jne	0x1402de6c7 <.text+0x2dd6c7>
1402de780: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
1402de785: 48 8b 30                    	movq	(%rax), %rsi
1402de788: 80 7c 24 30 00              	cmpb	$0x0, 0x30(%rsp)
1402de78d: 74 2f                       	je	0x1402de7be <.text+0x2dd7be>
1402de78f: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1402de793: 48 39 16                    	cmpq	%rdx, (%rsi)
1402de796: 0f 84 f1 00 00 00           	je	0x1402de88d <.text+0x2dd88d>
1402de79c: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1402de7a0: c6 04 10 7d                 	movb	$0x7d, (%rax,%rdx)
1402de7a4: 48 ff c2                    	incq	%rdx
1402de7a7: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
1402de7ab: eb 15                       	jmp	0x1402de7c2 <.text+0x2dd7c2>
1402de7ad: 48 89 c1                    	movq	%rax, %rcx
1402de7b0: 48 83 c4 38                 	addq	$0x38, %rsp
1402de7b4: 5b                          	popq	%rbx
1402de7b5: 5f                          	popq	%rdi
1402de7b6: 5e                          	popq	%rsi
1402de7b7: 41 5e                       	popq	%r14
1402de7b9: e9 e2 b0 5e 02              	jmp	0x1428c98a0 <.text+0x28c88a0>
1402de7be: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1402de7c2: 48 39 16                    	cmpq	%rdx, (%rsi)
1402de7c5: 0f 84 9c 00 00 00           	je	0x1402de867 <.text+0x2dd867>
1402de7cb: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1402de7cf: c6 04 10 7d                 	movb	$0x7d, (%rax,%rdx)
1402de7d3: 48 ff c2                    	incq	%rdx
1402de7d6: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
1402de7da: e9 e6 fe ff ff              	jmp	0x1402de6c5 <.text+0x2dd6c5>
1402de7df: 48 89 c1                    	movq	%rax, %rcx
1402de7e2: e8 b9 b0 5e 02              	callq	0x1428c98a0 <.text+0x28c88a0>
1402de7e7: e9 db fe ff ff              	jmp	0x1402de6c7 <.text+0x2dd6c7>
1402de7ec: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1402de7f5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402de7fb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1402de801: 48 89 d9                    	movq	%rbx, %rcx
1402de804: 49 89 d6                    	movq	%rdx, %r14
1402de807: 48 89 c2                    	movq	%rax, %rdx
1402de80a: e8 d1 41 5c 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1402de80f: 4c 89 f2                    	movq	%r14, %rdx
1402de812: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1402de816: e9 cd fe ff ff              	jmp	0x1402de6e8 <.text+0x2dd6e8>
1402de81b: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1402de824: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402de82a: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1402de830: 48 89 d9                    	movq	%rbx, %rcx
1402de833: e8 a8 41 5c 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1402de838: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1402de83c: e9 e4 fe ff ff              	jmp	0x1402de725 <.text+0x2dd725>
1402de841: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1402de84a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402de850: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1402de856: 48 89 d9                    	movq	%rbx, %rcx
1402de859: e8 82 41 5c 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1402de85e: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1402de862: e9 dd fe ff ff              	jmp	0x1402de744 <.text+0x2dd744>
1402de867: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1402de870: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402de876: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1402de87c: 48 89 f1                    	movq	%rsi, %rcx
1402de87f: e8 5c 41 5c 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1402de884: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1402de888: e9 3e ff ff ff              	jmp	0x1402de7cb <.text+0x2dd7cb>
1402de88d: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1402de896: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402de89c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1402de8a2: 48 89 f1                    	movq	%rsi, %rcx
1402de8a5: e8 36 41 5c 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1402de8aa: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1402de8ae: e9 e9 fe ff ff              	jmp	0x1402de79c <.text+0x2dd79c>
