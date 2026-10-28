
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408be360: 55                          	pushq	%rbp
1408be361: 41 57                       	pushq	%r15
1408be363: 41 56                       	pushq	%r14
1408be365: 41 55                       	pushq	%r13
1408be367: 41 54                       	pushq	%r12
1408be369: 56                          	pushq	%rsi
1408be36a: 57                          	pushq	%rdi
1408be36b: 53                          	pushq	%rbx
1408be36c: 48 81 ec c8 01 00 00        	subq	$0x1c8, %rsp            # imm = 0x1C8
1408be373: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408be37b: 48 c7 85 40 01 00 00 fe ff ff ff    	movq	$-0x2, 0x140(%rbp)
1408be386: 48 89 d7                    	movq	%rdx, %rdi
1408be389: 48 89 ce                    	movq	%rcx, %rsi
1408be38c: 45 8b 70 1c                 	movl	0x1c(%r8), %r14d
1408be390: 4d 8b 78 10                 	movq	0x10(%r8), %r15
1408be394: 45 89 37                    	movl	%r14d, (%r15)
1408be397: 49 8b 18                    	movq	(%r8), %rbx
1408be39a: 48 89 d9                    	movq	%rbx, %rcx
1408be39d: e8 3e 8f ff ff              	callq	0x1408b72e0 <.text+0x8b62e0>
1408be3a2: 45 89 37                    	movl	%r14d, (%r15)
1408be3a5: 48 89 9d 30 01 00 00        	movq	%rbx, 0x130(%rbp)
1408be3ac: c7 83 c8 01 00 00 00 00 00 00       	movl	$0x0, 0x1c8(%rbx)
1408be3b6: 4c 8b 27                    	movq	(%rdi), %r12
1408be3b9: 41 0f b6 44 24 58           	movzbl	0x58(%r12), %eax
1408be3bf: c1 e0 03                    	shll	$0x3, %eax
1408be3c2: 48 8d 0d 77 31 1e 02        	leaq	0x21e3177(%rip), %rcx   # 0x142aa1540
1408be3c9: 4c 8b 0c 08                 	movq	(%rax,%rcx), %r9
1408be3cd: 48 8d 0d 94 31 1e 02        	leaq	0x21e3194(%rip), %rcx   # 0x142aa1568
1408be3d4: 48 8b 04 08                 	movq	(%rax,%rcx), %rax
1408be3d8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408be3dd: 48 8d 15 36 11 23 02        	leaq	0x2231136(%rip), %rdx   # 0x142aef51a
1408be3e4: 48 8d 7d f8                 	leaq	-0x8(%rbp), %rdi
1408be3e8: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
1408be3ee: 48 89 f9                    	movq	%rdi, %rcx
1408be3f1: e8 1a 44 4a 00              	callq	0x140d62810 <.text+0xd61810>
1408be3f6: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1408be3fa: 48 89 fa                    	movq	%rdi, %rdx
1408be3fd: e8 5e 10 00 00              	callq	0x1408bf460 <.text+0x8be460>
1408be402: 0f b6 05 00 68 d7 02        	movzbl	0x2d76800(%rip), %eax   # 0x143634c09
1408be409: b9 22 00 00 00              	movl	$0x22, %ecx
1408be40e: ba 01 00 00 00              	movl	$0x1, %edx
1408be413: e8 f8 57 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408be418: 48 85 c0                    	testq	%rax, %rax
1408be41b: 0f 84 e0 07 00 00           	je	0x1408bec01 <.text+0x8bdc01>
1408be421: 48 89 c3                    	movq	%rax, %rbx
1408be424: 4c 89 65 58                 	movq	%r12, 0x58(%rbp)
1408be428: 0f 10 05 14 28 1e 02        	movups	0x21e2814(%rip), %xmm0  # 0x142aa0c43
1408be42f: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1408be433: 0f 10 05 f9 27 1e 02        	movups	0x21e27f9(%rip), %xmm0  # 0x142aa0c33
1408be43a: 0f 11 00                    	movups	%xmm0, (%rax)
1408be43d: 66 c7 40 20 61 66           	movw	$0x6661, 0x20(%rax)     # imm = 0x6661
1408be443: 0f b6 05 bf 67 d7 02        	movzbl	0x2d767bf(%rip), %eax   # 0x143634c09
1408be44a: b9 27 00 00 00              	movl	$0x27, %ecx
1408be44f: ba 01 00 00 00              	movl	$0x1, %edx
1408be454: e8 b7 57 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408be459: 48 85 c0                    	testq	%rax, %rax
1408be45c: 0f 84 c1 07 00 00           	je	0x1408bec23 <.text+0x8bdc23>
1408be462: 49 89 c5                    	movq	%rax, %r13
1408be465: 49 89 f4                    	movq	%rsi, %r12
1408be468: 0f 10 05 f6 27 1e 02        	movups	0x21e27f6(%rip), %xmm0  # 0x142aa0c65
1408be46f: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1408be473: 0f 10 05 db 27 1e 02        	movups	0x21e27db(%rip), %xmm0  # 0x142aa0c55
1408be47a: 0f 11 00                    	movups	%xmm0, (%rax)
1408be47d: 48 b8 65 6e 20 47 6c 61 64 65       	movabsq	$0x6564616c47206e65, %rax # imm = 0x6564616C47206E65
1408be487: 49 89 45 1f                 	movq	%rax, 0x1f(%r13)
1408be48b: 0f b6 05 77 67 d7 02        	movzbl	0x2d76777(%rip), %eax   # 0x143634c09
1408be492: b9 26 00 00 00              	movl	$0x26, %ecx
1408be497: ba 01 00 00 00              	movl	$0x1, %edx
1408be49c: e8 6f 57 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408be4a1: 48 85 c0                    	testq	%rax, %rax
1408be4a4: 0f 84 9b 07 00 00           	je	0x1408bec45 <.text+0x8bdc45>
1408be4aa: 0f 10 05 db 27 1e 02        	movups	0x21e27db(%rip), %xmm0  # 0x142aa0c8c
1408be4b1: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1408be4b5: 0f 10 05 c0 27 1e 02        	movups	0x21e27c0(%rip), %xmm0  # 0x142aa0c7c
1408be4bc: 0f 11 00                    	movups	%xmm0, (%rax)
1408be4bf: 48 b9 65 6e 20 52 75 69 6e 73       	movabsq	$0x736e697552206e65, %rcx # imm = 0x736E697552206E65
1408be4c9: 48 89 48 1e                 	movq	%rcx, 0x1e(%rax)
1408be4cd: 0f b6 0d 35 67 d7 02        	movzbl	0x2d76735(%rip), %ecx   # 0x143634c09
1408be4d4: 48 89 c6                    	movq	%rax, %rsi
1408be4d7: b9 27 00 00 00              	movl	$0x27, %ecx
1408be4dc: ba 01 00 00 00              	movl	$0x1, %edx
1408be4e1: e8 2a 57 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408be4e6: 48 85 c0                    	testq	%rax, %rax
1408be4e9: 0f 84 7f 07 00 00           	je	0x1408bec6e <.text+0x8bdc6e>
1408be4ef: 0f 10 05 bc 27 1e 02        	movups	0x21e27bc(%rip), %xmm0  # 0x142aa0cb2
1408be4f6: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1408be4fa: 0f 10 05 a1 27 1e 02        	movups	0x21e27a1(%rip), %xmm0  # 0x142aa0ca2
1408be501: 0f 11 00                    	movups	%xmm0, (%rax)
1408be504: 48 b9 64 20 46 6f 72 65 73 74       	movabsq	$0x747365726f462064, %rcx # imm = 0x747365726F462064
1408be50e: 48 89 48 1f                 	movq	%rcx, 0x1f(%rax)
1408be512: 48 c7 45 f8 22 00 00 00     	movq	$0x22, -0x8(%rbp)
1408be51a: 48 89 5d 00                 	movq	%rbx, (%rbp)
1408be51e: 48 c7 45 08 22 00 00 00     	movq	$0x22, 0x8(%rbp)
1408be526: 48 c7 45 10 27 00 00 00     	movq	$0x27, 0x10(%rbp)
1408be52e: 4c 89 6d 18                 	movq	%r13, 0x18(%rbp)
1408be532: 48 c7 45 20 27 00 00 00     	movq	$0x27, 0x20(%rbp)
1408be53a: 48 c7 45 28 26 00 00 00     	movq	$0x26, 0x28(%rbp)
1408be542: 48 89 75 30                 	movq	%rsi, 0x30(%rbp)
1408be546: 48 c7 45 38 26 00 00 00     	movq	$0x26, 0x38(%rbp)
1408be54e: 48 c7 45 40 27 00 00 00     	movq	$0x27, 0x40(%rbp)
1408be556: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1408be55a: 48 c7 45 50 27 00 00 00     	movq	$0x27, 0x50(%rbp)
1408be562: 44 89 b5 e8 00 00 00        	movl	%r14d, 0xe8(%rbp)
1408be569: 4c 89 bd a8 00 00 00        	movq	%r15, 0xa8(%rbp)
1408be570: 45 89 37                    	movl	%r14d, (%r15)
1408be573: 49 8b 1c 24                 	movq	(%r12), %rbx
1408be577: 4c 8b 65 58                 	movq	0x58(%rbp), %r12
1408be57b: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
1408be580: 49 8b 44 24 10              	movq	0x10(%r12), %rax
1408be585: 45 31 c0                    	xorl	%r8d, %r8d
1408be588: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1408be590: 4c 89 c2                    	movq	%r8, %rdx
1408be593: 4c 39 c0                    	cmpq	%r8, %rax
1408be596: 74 0a                       	je	0x1408be5a2 <.text+0x8bd5a2>
1408be598: 4c 8d 42 01                 	leaq	0x1(%rdx), %r8
1408be59c: 80 3c 11 01                 	cmpb	$0x1, (%rcx,%rdx)
1408be5a0: 77 ee                       	ja	0x1408be590 <.text+0x8bd590>
1408be5a2: 48 39 d0                    	cmpq	%rdx, %rax
1408be5a5: 4c 8b 6d c0                 	movq	-0x40(%rbp), %r13
1408be5a9: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1408be5af: 4d 0f 44 cd                 	cmoveq	%r13, %r9
1408be5b3: 48 8b 75 b8                 	movq	-0x48(%rbp), %rsi
1408be5b7: 48 0f 44 fe                 	cmoveq	%rsi, %rdi
1408be5bb: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
1408be5c2: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
1408be5c9: 48 89 da                    	movq	%rbx, %rdx
1408be5cc: 49 89 f8                    	movq	%rdi, %r8
1408be5cf: 48 89 cf                    	movq	%rcx, %rdi
1408be5d2: e8 39 85 ff ff              	callq	0x1408b6b10 <.text+0x8b5b10>
1408be5d7: 8b 85 e8 00 00 00           	movl	0xe8(%rbp), %eax
1408be5dd: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
1408be5e4: 89 01                       	movl	%eax, (%rcx)
1408be5e6: 4c 8b 7d d0                 	movq	-0x30(%rbp), %r15
1408be5ea: 4c 8b 75 d8                 	movq	-0x28(%rbp), %r14
1408be5ee: 83 bf 30 01 00 00 01        	cmpl	$0x1, 0x130(%rdi)
1408be5f5: 75 2e                       	jne	0x1408be625 <.text+0x8bd625>
1408be5f7: 48 8d 97 38 01 00 00        	leaq	0x138(%rdi), %rdx
1408be5fe: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
1408be605: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1408be609: 41 b0 01                    	movb	$0x1, %r8b
1408be60c: e8 2f 8e b7 00              	callq	0x141437440 <.text+0x1436440>
1408be611: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1408be61b: 48 39 45 68                 	cmpq	%rax, 0x68(%rbp)
1408be61f: 0f 85 79 06 00 00           	jne	0x1408bec9e <.text+0x8bdc9e>
1408be625: 48 8b 13                    	movq	(%rbx), %rdx
1408be628: 48 85 d2                    	testq	%rdx, %rdx
1408be62b: 74 63                       	je	0x1408be690 <.text+0x8bd690>
1408be62d: 48 83 c2 10                 	addq	$0x10, %rdx
1408be631: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
1408be638: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1408be63f: 4d 89 f8                    	movq	%r15, %r8
1408be642: 4d 89 f1                    	movq	%r14, %r9
1408be645: e8 f6 5d 19 00              	callq	0x140a54440 <.text+0xa53440>
1408be64a: 48 83 bd f0 00 00 00 02     	cmpq	$0x2, 0xf0(%rbp)
1408be652: 0f 85 b9 00 00 00           	jne	0x1408be711 <.text+0x8bd711>
1408be658: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
1408be65f: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1408be663: 48 8d 05 96 26 1e 02        	leaq	0x21e2696(%rip), %rax   # 0x142aa0d00
1408be66a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408be66f: 48 8d 0d 42 1d 1e 02        	leaq	0x21e1d42(%rip), %rcx   # 0x142aa03b8
1408be676: 4c 8d 0d 1b 1d 1e 02        	leaq	0x21e1d1b(%rip), %r9    # 0x142aa0398
1408be67d: 4c 8d 45 68                 	leaq	0x68(%rbp), %r8
1408be681: ba 2b 00 00 00              	movl	$0x2b, %edx
1408be686: e8 d5 b0 01 02              	callq	0x1428d9760 <.text+0x28d8760>
1408be68b: e9 76 07 00 00              	jmp	0x1408bee06 <.text+0x8bde06>
1408be690: 4d 85 f6                    	testq	%r14, %r14
1408be693: 0f 88 c9 06 00 00           	js	0x1408bed62 <.text+0x8bdd62>
1408be699: 4c 89 ff                    	movq	%r15, %rdi
1408be69c: 74 2e                       	je	0x1408be6cc <.text+0x8bd6cc>
1408be69e: 0f b6 05 64 65 d7 02        	movzbl	0x2d76564(%rip), %eax   # 0x143634c09
1408be6a5: b8 01 00 00 00              	movl	$0x1, %eax
1408be6aa: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
1408be6b1: ba 01 00 00 00              	movl	$0x1, %edx
1408be6b6: 4c 89 f1                    	movq	%r14, %rcx
1408be6b9: e8 52 55 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408be6be: 48 85 c0                    	testq	%rax, %rax
1408be6c1: 0f 84 a6 06 00 00           	je	0x1408bed6d <.text+0x8bdd6d>
1408be6c7: 49 89 c7                    	movq	%rax, %r15
1408be6ca: eb 06                       	jmp	0x1408be6d2 <.text+0x8bd6d2>
1408be6cc: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1408be6d2: 4c 89 f9                    	movq	%r15, %rcx
1408be6d5: 48 89 fa                    	movq	%rdi, %rdx
1408be6d8: 4d 89 f0                    	movq	%r14, %r8
1408be6db: e8 20 55 06 02              	callq	0x142923c00 <.text+0x2922c00>
1408be6e0: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
1408be6eb: 4c 89 b5 00 01 00 00        	movq	%r14, 0x100(%rbp)
1408be6f2: 4c 89 bd 08 01 00 00        	movq	%r15, 0x108(%rbp)
1408be6f9: 49 89 ff                    	movq	%rdi, %r15
1408be6fc: 4c 89 b5 10 01 00 00        	movq	%r14, 0x110(%rbp)
1408be703: c6 85 18 01 00 00 02        	movb	$0x2, 0x118(%rbp)
1408be70a: 48 8b bd 30 01 00 00        	movq	0x130(%rbp), %rdi
1408be711: 48 8d 8f 30 01 00 00        	leaq	0x130(%rdi), %rcx
1408be718: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
1408be71f: 4c 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %r10
1408be726: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1408be72d: 4c 8b 85 08 01 00 00        	movq	0x108(%rbp), %r8
1408be734: 4c 8b 8d 10 01 00 00        	movq	0x110(%rbp), %r9
1408be73b: 4c 89 8d d0 00 00 00        	movq	%r9, 0xd0(%rbp)
1408be742: 44 0f b6 8d 18 01 00 00     	movzbl	0x118(%rbp), %r9d
1408be74a: 44 88 8d d8 00 00 00        	movb	%r9b, 0xd8(%rbp)
1408be751: 44 8b 8d 19 01 00 00        	movl	0x119(%rbp), %r9d
1408be758: 44 89 8d d9 00 00 00        	movl	%r9d, 0xd9(%rbp)
1408be75f: 44 0f b7 8d 1d 01 00 00     	movzwl	0x11d(%rbp), %r9d
1408be767: 66 44 89 8d dd 00 00 00     	movw	%r9w, 0xdd(%rbp)
1408be76f: 44 0f b6 8d 1f 01 00 00     	movzbl	0x11f(%rbp), %r9d
1408be777: 44 88 8d df 00 00 00        	movb	%r9b, 0xdf(%rbp)
1408be77e: 48 89 95 c0 00 00 00        	movq	%rdx, 0xc0(%rbp)
1408be785: 4c 89 85 c8 00 00 00        	movq	%r8, 0xc8(%rbp)
1408be78c: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1408be793: 4c 89 95 b8 00 00 00        	movq	%r10, 0xb8(%rbp)
1408be79a: 48 89 8d 28 01 00 00        	movq	%rcx, 0x128(%rbp)
1408be7a1: e8 da ba fd ff              	callq	0x14089a280 <.text+0x899280>
1408be7a6: 48 8b bd 30 01 00 00        	movq	0x130(%rbp), %rdi
1408be7ad: 48 8b 97 40 01 00 00        	movq	0x140(%rdi), %rdx
1408be7b4: 48 85 d2                    	testq	%rdx, %rdx
1408be7b7: 74 12                       	je	0x1408be7cb <.text+0x8bd7cb>
1408be7b9: 48 8b 8f 48 01 00 00        	movq	0x148(%rdi), %rcx
1408be7c0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408be7c6: e8 55 54 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408be7cb: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1408be7d2: 0f 28 8d c0 00 00 00        	movaps	0xc0(%rbp), %xmm1
1408be7d9: 0f 28 95 d0 00 00 00        	movaps	0xd0(%rbp), %xmm2
1408be7e0: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
1408be7e7: 0f 11 00                    	movups	%xmm0, (%rax)
1408be7ea: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1408be7ee: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1408be7f2: 83 38 01                    	cmpl	$0x1, (%rax)
1408be7f5: 75 2b                       	jne	0x1408be822 <.text+0x8bd822>
1408be7f7: 48 8d 97 38 01 00 00        	leaq	0x138(%rdi), %rdx
1408be7fe: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
1408be805: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1408be809: e8 c2 8a b7 00              	callq	0x1414372d0 <.text+0x14362d0>
1408be80e: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1408be818: 48 39 45 68                 	cmpq	%rax, 0x68(%rbp)
1408be81c: 0f 85 de 04 00 00           	jne	0x1408bed00 <.text+0x8bdd00>
1408be822: 4c 8b 75 e0                 	movq	-0x20(%rbp), %r14
1408be826: 40 b7 01                    	movb	$0x1, %dil
1408be829: 4c 89 f0                    	movq	%r14, %rax
1408be82c: 48 f7 d8                    	negq	%rax
1408be82f: 0f 80 ee 01 00 00           	jo	0x1408bea23 <.text+0x8bda23>
1408be835: 4c 89 b5 a0 00 00 00        	movq	%r14, 0xa0(%rbp)
1408be83c: 4c 8b 45 e8                 	movq	-0x18(%rbp), %r8
1408be840: 4c 8b 75 f0                 	movq	-0x10(%rbp), %r14
1408be844: 48 8b 13                    	movq	(%rbx), %rdx
1408be847: 48 85 d2                    	testq	%rdx, %rdx
1408be84a: 4c 89 85 28 01 00 00        	movq	%r8, 0x128(%rbp)
1408be851: 74 59                       	je	0x1408be8ac <.text+0x8bd8ac>
1408be853: 48 83 c2 10                 	addq	$0x10, %rdx
1408be857: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1408be85e: 4d 89 f1                    	movq	%r14, %r9
1408be861: e8 da 5b 19 00              	callq	0x140a54440 <.text+0xa53440>
1408be866: 48 83 bd f0 00 00 00 02     	cmpq	$0x2, 0xf0(%rbp)
1408be86e: 0f 85 b3 00 00 00           	jne	0x1408be927 <.text+0x8bd927>
1408be874: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
1408be87b: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1408be87f: 48 8d 05 4a 24 1e 02        	leaq	0x21e244a(%rip), %rax   # 0x142aa0cd0
1408be886: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408be88b: 48 8d 0d 26 1b 1e 02        	leaq	0x21e1b26(%rip), %rcx   # 0x142aa03b8
1408be892: 4c 8d 0d ff 1a 1e 02        	leaq	0x21e1aff(%rip), %r9    # 0x142aa0398
1408be899: 4c 8d 45 68                 	leaq	0x68(%rbp), %r8
1408be89d: ba 2b 00 00 00              	movl	$0x2b, %edx
1408be8a2: e8 b9 ae 01 02              	callq	0x1428d9760 <.text+0x28d8760>
1408be8a7: e9 5a 05 00 00              	jmp	0x1408bee06 <.text+0x8bde06>
1408be8ac: 4d 85 f6                    	testq	%r14, %r14
1408be8af: 0f 88 36 05 00 00           	js	0x1408bedeb <.text+0x8bddeb>
1408be8b5: 4c 89 ff                    	movq	%r15, %rdi
1408be8b8: 74 2b                       	je	0x1408be8e5 <.text+0x8bd8e5>
1408be8ba: 0f b6 05 48 63 d7 02        	movzbl	0x2d76348(%rip), %eax   # 0x143634c09
1408be8c1: b8 01 00 00 00              	movl	$0x1, %eax
1408be8c6: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1408be8ca: ba 01 00 00 00              	movl	$0x1, %edx
1408be8cf: 4c 89 f1                    	movq	%r14, %rcx
1408be8d2: e8 39 53 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408be8d7: 48 85 c0                    	testq	%rax, %rax
1408be8da: 0f 84 13 05 00 00           	je	0x1408bedf3 <.text+0x8bddf3>
1408be8e0: 49 89 c7                    	movq	%rax, %r15
1408be8e3: eb 06                       	jmp	0x1408be8eb <.text+0x8bd8eb>
1408be8e5: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1408be8eb: 4c 89 f9                    	movq	%r15, %rcx
1408be8ee: 48 8b 95 28 01 00 00        	movq	0x128(%rbp), %rdx
1408be8f5: 4d 89 f0                    	movq	%r14, %r8
1408be8f8: e8 03 53 06 02              	callq	0x142923c00 <.text+0x2922c00>
1408be8fd: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
1408be908: 4c 89 b5 00 01 00 00        	movq	%r14, 0x100(%rbp)
1408be90f: 4c 89 bd 08 01 00 00        	movq	%r15, 0x108(%rbp)
1408be916: 4c 89 b5 10 01 00 00        	movq	%r14, 0x110(%rbp)
1408be91d: c6 85 18 01 00 00 02        	movb	$0x2, 0x118(%rbp)
1408be924: 49 89 ff                    	movq	%rdi, %r15
1408be927: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
1408be92e: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1408be935: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1408be93c: 4c 8b 85 08 01 00 00        	movq	0x108(%rbp), %r8
1408be943: 4c 8b 8d 10 01 00 00        	movq	0x110(%rbp), %r9
1408be94a: 4c 89 8d d0 00 00 00        	movq	%r9, 0xd0(%rbp)
1408be951: 44 0f b6 8d 18 01 00 00     	movzbl	0x118(%rbp), %r9d
1408be959: 44 88 8d d8 00 00 00        	movb	%r9b, 0xd8(%rbp)
1408be960: 44 8b 8d 19 01 00 00        	movl	0x119(%rbp), %r9d
1408be967: 44 89 8d d9 00 00 00        	movl	%r9d, 0xd9(%rbp)
1408be96e: 44 0f b7 8d 1d 01 00 00     	movzwl	0x11d(%rbp), %r9d
1408be976: 66 44 89 8d dd 00 00 00     	movw	%r9w, 0xdd(%rbp)
1408be97e: 44 0f b6 8d 1f 01 00 00     	movzbl	0x11f(%rbp), %r9d
1408be986: 44 88 8d df 00 00 00        	movb	%r9b, 0xdf(%rbp)
1408be98d: 48 89 95 c0 00 00 00        	movq	%rdx, 0xc0(%rbp)
1408be994: 4c 89 85 c8 00 00 00        	movq	%r8, 0xc8(%rbp)
1408be99b: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1408be9a2: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
1408be9a9: 83 bd b0 00 00 00 01        	cmpl	$0x1, 0xb0(%rbp)
1408be9b0: 75 24                       	jne	0x1408be9d6 <.text+0x8bd9d6>
1408be9b2: 48 8d 95 b8 00 00 00        	leaq	0xb8(%rbp), %rdx
1408be9b9: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1408be9bd: e8 0e 89 b7 00              	callq	0x1414372d0 <.text+0x14362d0>
1408be9c2: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1408be9cc: 48 39 45 68                 	cmpq	%rax, 0x68(%rbp)
1408be9d0: 0f 85 b6 03 00 00           	jne	0x1408bed8c <.text+0x8bdd8c>
1408be9d6: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1408be9dd: e8 9e b8 fd ff              	callq	0x14089a280 <.text+0x899280>
1408be9e2: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
1408be9e9: 48 85 d2                    	testq	%rdx, %rdx
1408be9ec: 74 12                       	je	0x1408bea00 <.text+0x8bda00>
1408be9ee: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
1408be9f5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408be9fb: e8 20 52 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bea00: 4c 8b b5 a0 00 00 00        	movq	0xa0(%rbp), %r14
1408bea07: 4d 85 f6                    	testq	%r14, %r14
1408bea0a: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
1408bea11: 74 0e                       	je	0x1408bea21 <.text+0x8bda21>
1408bea13: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bea19: 4c 89 f2                    	movq	%r14, %rdx
1408bea1c: e8 ff 51 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bea21: 31 ff                       	xorl	%edi, %edi
1408bea23: 41 80 7c 24 58 04           	cmpb	$0x4, 0x58(%r12)
1408bea29: 74 05                       	je	0x1408bea30 <.text+0x8bda30>
1408bea2b: 0f 57 c0                    	xorps	%xmm0, %xmm0
1408bea2e: eb 08                       	jmp	0x1408bea38 <.text+0x8bda38>
1408bea30: f3 0f 10 05 18 6f 06 02     	movss	0x2066f18(%rip), %xmm0  # 0x142925950
1408bea38: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
1408bea3f: f3 0f 11 80 c4 01 00 00     	movss	%xmm0, 0x1c4(%rax)
1408bea47: 8b 8d e8 00 00 00           	movl	0xe8(%rbp), %ecx
1408bea4d: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
1408bea54: 89 0a                       	movl	%ecx, (%rdx)
1408bea56: f3 0f 11 80 c0 01 00 00     	movss	%xmm0, 0x1c0(%rax)
1408bea5e: 48 8b 0b                    	movq	(%rbx), %rcx
1408bea61: 48 85 c9                    	testq	%rcx, %rcx
1408bea64: 74 2f                       	je	0x1408bea95 <.text+0x8bda95>
1408bea66: 41 0f b6 44 24 58           	movzbl	0x58(%r12), %eax
1408bea6c: 48 8d 15 b9 2a 1e 02        	leaq	0x21e2ab9(%rip), %rdx   # 0x142aa152c
1408bea73: f3 0f 10 1c 82              	movss	(%rdx,%rax,4), %xmm3
1408bea78: 48 83 c1 10                 	addq	$0x10, %rcx
1408bea7c: 40 88 bd 3f 01 00 00        	movb	%dil, 0x13f(%rbp)
1408bea83: 48 8d 15 5e 22 1e 02        	leaq	0x21e225e(%rip), %rdx   # 0x142aa0ce8
1408bea8a: 41 b8 09 00 00 00           	movl	$0x9, %r8d
1408bea90: e8 1b 53 19 00              	callq	0x140a53db0 <.text+0xa52db0>
1408bea95: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
1408bea9a: 49 8b 44 24 10              	movq	0x10(%r12), %rax
1408bea9f: 45 31 c0                    	xorl	%r8d, %r8d
1408beaa2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1408beab0: 4c 89 c2                    	movq	%r8, %rdx
1408beab3: 4c 39 c0                    	cmpq	%r8, %rax
1408beab6: 74 0a                       	je	0x1408beac2 <.text+0x8bdac2>
1408beab8: 4c 8d 42 01                 	leaq	0x1(%rdx), %r8
1408beabc: 80 3c 11 01                 	cmpb	$0x1, (%rcx,%rdx)
1408beac0: 77 ee                       	ja	0x1408beab0 <.text+0x8bdab0>
1408beac2: 48 8b 0b                    	movq	(%rbx), %rcx
1408beac5: 48 85 c9                    	testq	%rcx, %rcx
1408beac8: 74 2d                       	je	0x1408beaf7 <.text+0x8bdaf7>
1408beaca: 0f 57 db                    	xorps	%xmm3, %xmm3
1408beacd: 48 39 d0                    	cmpq	%rdx, %rax
1408bead0: 74 08                       	je	0x1408beada <.text+0x8bdada>
1408bead2: f3 0f 10 1d 2a 6e 06 02     	movss	0x2066e2a(%rip), %xmm3  # 0x142925904
1408beada: 48 83 c1 10                 	addq	$0x10, %rcx
1408beade: 40 88 bd 3f 01 00 00        	movb	%dil, 0x13f(%rbp)
1408beae5: 48 8d 15 05 22 1e 02        	leaq	0x21e2205(%rip), %rdx   # 0x142aa0cf1
1408beaec: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
1408beaf2: e8 b9 52 19 00              	callq	0x140a53db0 <.text+0xa52db0>
1408beaf7: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
1408beafb: 48 85 d2                    	testq	%rdx, %rdx
1408beafe: 74 0f                       	je	0x1408beb0f <.text+0x8bdb0f>
1408beb00: 48 8b 4d 00                 	movq	(%rbp), %rcx
1408beb04: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408beb0a: e8 11 51 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408beb0f: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
1408beb13: 48 85 d2                    	testq	%rdx, %rdx
1408beb16: 74 0f                       	je	0x1408beb27 <.text+0x8bdb27>
1408beb18: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
1408beb1c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408beb22: e8 f9 50 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408beb27: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
1408beb2b: 48 85 d2                    	testq	%rdx, %rdx
1408beb2e: 74 0f                       	je	0x1408beb3f <.text+0x8bdb3f>
1408beb30: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1408beb34: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408beb3a: e8 e1 50 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408beb3f: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
1408beb43: 48 85 d2                    	testq	%rdx, %rdx
1408beb46: 74 0f                       	je	0x1408beb57 <.text+0x8bdb57>
1408beb48: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
1408beb4c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408beb52: e8 c9 50 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408beb57: 4d 85 ed                    	testq	%r13, %r13
1408beb5a: 74 36                       	je	0x1408beb92 <.text+0x8bdb92>
1408beb5c: 48 8d 5e 08                 	leaq	0x8(%rsi), %rbx
1408beb60: eb 17                       	jmp	0x1408beb79 <.text+0x8bdb79>
1408beb62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1408beb70: 48 83 c3 18                 	addq	$0x18, %rbx
1408beb74: 49 ff cd                    	decq	%r13
1408beb77: 74 19                       	je	0x1408beb92 <.text+0x8bdb92>
1408beb79: 48 8b 53 f8                 	movq	-0x8(%rbx), %rdx
1408beb7d: 48 85 d2                    	testq	%rdx, %rdx
1408beb80: 74 ee                       	je	0x1408beb70 <.text+0x8bdb70>
1408beb82: 48 8b 0b                    	movq	(%rbx), %rcx
1408beb85: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408beb8b: e8 90 50 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408beb90: eb de                       	jmp	0x1408beb70 <.text+0x8bdb70>
1408beb92: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
1408beb96: 48 85 c0                    	testq	%rax, %rax
1408beb99: 74 16                       	je	0x1408bebb1 <.text+0x8bdbb1>
1408beb9b: 48 c1 e0 03                 	shlq	$0x3, %rax
1408beb9f: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1408beba3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1408beba9: 48 89 f1                    	movq	%rsi, %rcx
1408bebac: e8 6f 50 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bebb1: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
1408bebb5: 48 85 d2                    	testq	%rdx, %rdx
1408bebb8: 74 0e                       	je	0x1408bebc8 <.text+0x8bdbc8>
1408bebba: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bebc0: 4c 89 f9                    	movq	%r15, %rcx
1408bebc3: e8 58 50 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bebc8: 4c 89 f0                    	movq	%r14, %rax
1408bebcb: 48 f7 d8                    	negq	%rax
1408bebce: 70 1c                       	jo	0x1408bebec <.text+0x8bdbec>
1408bebd0: 40 84 ff                    	testb	%dil, %dil
1408bebd3: 74 17                       	je	0x1408bebec <.text+0x8bdbec>
1408bebd5: 4d 85 f6                    	testq	%r14, %r14
1408bebd8: 74 12                       	je	0x1408bebec <.text+0x8bdbec>
1408bebda: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1408bebde: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bebe4: 4c 89 f2                    	movq	%r14, %rdx
1408bebe7: e8 34 50 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bebec: 90                          	nop
1408bebed: 48 81 c4 c8 01 00 00        	addq	$0x1c8, %rsp            # imm = 0x1C8
1408bebf4: 5b                          	popq	%rbx
1408bebf5: 5f                          	popq	%rdi
1408bebf6: 5e                          	popq	%rsi
1408bebf7: 41 5c                       	popq	%r12
1408bebf9: 41 5d                       	popq	%r13
1408bebfb: 41 5e                       	popq	%r14
1408bebfd: 41 5f                       	popq	%r15
1408bebff: 5d                          	popq	%rbp
1408bec00: c3                          	retq
1408bec01: c6 85 3e 01 00 00 01        	movb	$0x1, 0x13e(%rbp)
1408bec08: 4c 8d 05 11 19 1e 02        	leaq	0x21e1911(%rip), %r8    # 0x142aa0520
1408bec0f: b9 01 00 00 00              	movl	$0x1, %ecx
1408bec14: ba 22 00 00 00              	movl	$0x22, %edx
1408bec19: e8 c5 a3 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408bec1e: e9 e3 01 00 00              	jmp	0x1408bee06 <.text+0x8bde06>
1408bec23: 48 89 9d 30 01 00 00        	movq	%rbx, 0x130(%rbp)
1408bec2a: 4c 8d 05 ef 18 1e 02        	leaq	0x21e18ef(%rip), %r8    # 0x142aa0520
1408bec31: b9 01 00 00 00              	movl	$0x1, %ecx
1408bec36: ba 27 00 00 00              	movl	$0x27, %edx
1408bec3b: e8 a3 a3 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408bec40: e9 c1 01 00 00              	jmp	0x1408bee06 <.text+0x8bde06>
1408bec45: 4c 89 ad e8 00 00 00        	movq	%r13, 0xe8(%rbp)
1408bec4c: 48 89 9d 30 01 00 00        	movq	%rbx, 0x130(%rbp)
1408bec53: 4c 8d 05 c6 18 1e 02        	leaq	0x21e18c6(%rip), %r8    # 0x142aa0520
1408bec5a: b9 01 00 00 00              	movl	$0x1, %ecx
1408bec5f: ba 26 00 00 00              	movl	$0x26, %edx
1408bec64: e8 7a a3 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408bec69: e9 98 01 00 00              	jmp	0x1408bee06 <.text+0x8bde06>
1408bec6e: 48 89 b5 a8 00 00 00        	movq	%rsi, 0xa8(%rbp)
1408bec75: 4c 89 ad e8 00 00 00        	movq	%r13, 0xe8(%rbp)
1408bec7c: 48 89 9d 30 01 00 00        	movq	%rbx, 0x130(%rbp)
1408bec83: 4c 8d 05 96 18 1e 02        	leaq	0x21e1896(%rip), %r8    # 0x142aa0520
1408bec8a: b9 01 00 00 00              	movl	$0x1, %ecx
1408bec8f: ba 27 00 00 00              	movl	$0x27, %edx
1408bec94: e8 4a a3 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408bec99: e9 68 01 00 00              	jmp	0x1408bee06 <.text+0x8bde06>
1408bec9e: 48 8b 85 98 00 00 00        	movq	0x98(%rbp), %rax
1408beca5: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1408becac: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
1408becb0: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
1408becb4: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
1408becbb: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1408becc2: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1408becc9: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1408becd0: 48 8d 05 49 3f 21 02        	leaq	0x2213f49(%rip), %rax   # 0x142ad2c20
1408becd7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408becdc: 48 8d 0d 1d 19 21 02        	leaq	0x221191d(%rip), %rcx   # 0x142ad0600
1408bece3: 4c 8d 0d d6 18 21 02        	leaq	0x22118d6(%rip), %r9    # 0x142ad05c0
1408becea: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1408becf1: ba 2b 00 00 00              	movl	$0x2b, %edx
1408becf6: e8 65 aa 01 02              	callq	0x1428d9760 <.text+0x28d8760>
1408becfb: e9 06 01 00 00              	jmp	0x1408bee06 <.text+0x8bde06>
1408bed00: 48 8b 85 98 00 00 00        	movq	0x98(%rbp), %rax
1408bed07: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1408bed0e: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
1408bed12: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
1408bed16: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
1408bed1d: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1408bed24: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1408bed2b: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1408bed32: 48 8d 05 87 3e 21 02        	leaq	0x2213e87(%rip), %rax   # 0x142ad2bc0
1408bed39: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408bed3e: 48 8d 0d bb 18 21 02        	leaq	0x22118bb(%rip), %rcx   # 0x142ad0600
1408bed45: 4c 8d 0d 74 18 21 02        	leaq	0x2211874(%rip), %r9    # 0x142ad05c0
1408bed4c: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1408bed53: ba 2b 00 00 00              	movl	$0x2b, %edx
1408bed58: e8 03 aa 01 02              	callq	0x1428d9760 <.text+0x28d8760>
1408bed5d: e9 a4 00 00 00              	jmp	0x1408bee06 <.text+0x8bde06>
1408bed62: 48 c7 85 28 01 00 00 00 00 00 00    	movq	$0x0, 0x128(%rbp)
1408bed6d: c6 85 3f 01 00 00 01        	movb	$0x1, 0x13f(%rbp)
1408bed74: 4c 8d 05 35 1a 21 02        	leaq	0x2211a35(%rip), %r8    # 0x142ad07b0
1408bed7b: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
1408bed82: 4c 89 f2                    	movq	%r14, %rdx
1408bed85: e8 59 a2 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408bed8a: eb 7a                       	jmp	0x1408bee06 <.text+0x8bde06>
1408bed8c: 48 8b 85 98 00 00 00        	movq	0x98(%rbp), %rax
1408bed93: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1408bed9a: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
1408bed9e: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
1408beda2: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
1408beda9: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1408bedb0: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1408bedb7: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1408bedbe: 48 8d 05 fb 3d 21 02        	leaq	0x2213dfb(%rip), %rax   # 0x142ad2bc0
1408bedc5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408bedca: 48 8d 0d 2f 18 21 02        	leaq	0x221182f(%rip), %rcx   # 0x142ad0600
1408bedd1: 4c 8d 0d e8 17 21 02        	leaq	0x22117e8(%rip), %r9    # 0x142ad05c0
1408bedd8: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1408beddf: ba 2b 00 00 00              	movl	$0x2b, %edx
1408bede4: e8 77 a9 01 02              	callq	0x1428d9760 <.text+0x28d8760>
1408bede9: eb 1b                       	jmp	0x1408bee06 <.text+0x8bde06>
1408bedeb: 48 c7 45 60 00 00 00 00     	movq	$0x0, 0x60(%rbp)
1408bedf3: 4c 8d 05 b6 19 21 02        	leaq	0x22119b6(%rip), %r8    # 0x142ad07b0
1408bedfa: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1408bedfe: 4c 89 f2                    	movq	%r14, %rdx
1408bee01: e8 dd a1 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408bee06: 0f 0b                       	ud2
1408bee08: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
