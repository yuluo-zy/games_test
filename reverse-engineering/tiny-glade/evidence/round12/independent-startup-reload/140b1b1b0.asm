
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b1b1b0: 55                          	pushq	%rbp
140b1b1b1: 41 56                       	pushq	%r14
140b1b1b3: 56                          	pushq	%rsi
140b1b1b4: 57                          	pushq	%rdi
140b1b1b5: 53                          	pushq	%rbx
140b1b1b6: 48 81 ec 10 01 00 00        	subq	$0x110, %rsp            # imm = 0x110
140b1b1bd: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b1b1c5: 48 c7 85 88 00 00 00 fe ff ff ff    	movq	$-0x2, 0x88(%rbp)
140b1b1d0: 4c 89 cf                    	movq	%r9, %rdi
140b1b1d3: 48 89 d6                    	movq	%rdx, %rsi
140b1b1d6: 49 8b 00                    	movq	(%r8), %rax
140b1b1d9: 49 8b 50 08                 	movq	0x8(%r8), %rdx
140b1b1dd: 4c 8b 00                    	movq	(%rax), %r8
140b1b1e0: 45 31 c9                    	xorl	%r9d, %r9d
140b1b1e3: 4d 89 c2                    	movq	%r8, %r10
140b1b1e6: 4c 2b 52 18                 	subq	0x18(%rdx), %r10
140b1b1ea: 4d 0f 42 d1                 	cmovbq	%r9, %r10
140b1b1ee: 4c 2b 42 38                 	subq	0x38(%rdx), %r8
140b1b1f2: 4d 0f 42 c1                 	cmovbq	%r9, %r8
140b1b1f6: 4c 8b 5a 30                 	movq	0x30(%rdx), %r11
140b1b1fa: 4d 29 c3                    	subq	%r8, %r11
140b1b1fd: 4d 0f 42 d9                 	cmovbq	%r9, %r11
140b1b201: 41 0f 96 c0                 	setbe	%r8b
140b1b205: 48 8b 5a 10                 	movq	0x10(%rdx), %rbx
140b1b209: 4c 29 d3                    	subq	%r10, %rbx
140b1b20c: 49 0f 42 d9                 	cmovbq	%r9, %rbx
140b1b210: 48 8b 52 40                 	movq	0x40(%rdx), %rdx
140b1b214: 41 0f 96 c1                 	setbe	%r9b
140b1b218: 4c 01 db                    	addq	%r11, %rbx
140b1b21b: 49 89 d2                    	movq	%rdx, %r10
140b1b21e: 49 29 da                    	subq	%rbx, %r10
140b1b221: 4c 89 10                    	movq	%r10, (%rax)
140b1b224: 45 84 c1                    	testb	%r8b, %r9b
140b1b227: 0f 85 49 01 00 00           	jne	0x140b1b376 <.text+0xb1a376>
140b1b22d: 48 89 10                    	movq	%rdx, (%rax)
140b1b230: 48 8b 01                    	movq	(%rcx), %rax
140b1b233: 48 8b 10                    	movq	(%rax), %rdx
140b1b236: 48 85 d2                    	testq	%rdx, %rdx
140b1b239: 74 5a                       	je	0x140b1b295 <.text+0xb1a295>
140b1b23b: 48 83 c2 10                 	addq	$0x10, %rdx
140b1b23f: 4c 8d 05 ca e2 fc 01        	leaq	0x1fce2ca(%rip), %r8    # 0x142ae9510
140b1b246: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b1b24a: 41 b9 11 00 00 00           	movl	$0x11, %r9d
140b1b250: e8 eb 91 f3 ff              	callq	0x140a54440 <.text+0xa53440>
140b1b255: 48 83 7d 10 02              	cmpq	$0x2, 0x10(%rbp)
140b1b25a: 0f 85 82 00 00 00           	jne	0x140b1b2e2 <.text+0xb1a2e2>
140b1b260: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140b1b264: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140b1b268: 48 8d 05 b9 e2 fc 01        	leaq	0x1fce2b9(%rip), %rax   # 0x142ae9528
140b1b26f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1b274: 48 8d 0d 0d de fc 01        	leaq	0x1fcde0d(%rip), %rcx   # 0x142ae9088
140b1b27b: 4c 8d 0d e6 dd fc 01        	leaq	0x1fcdde6(%rip), %r9    # 0x142ae9068
140b1b282: 4c 8d 45 d8                 	leaq	-0x28(%rbp), %r8
140b1b286: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1b28b: e8 d0 e4 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1b290: e9 af 03 00 00              	jmp	0x140b1b644 <.text+0xb1a644>
140b1b295: 0f b6 05 6d 99 b1 02        	movzbl	0x2b1996d(%rip), %eax   # 0x143634c09
140b1b29c: b9 11 00 00 00              	movl	$0x11, %ecx
140b1b2a1: ba 01 00 00 00              	movl	$0x1, %edx
140b1b2a6: e8 65 89 af ff              	callq	0x140613c10 <.text+0x612c10>
140b1b2ab: 48 85 c0                    	testq	%rax, %rax
140b1b2ae: 0f 84 e8 02 00 00           	je	0x140b1b59c <.text+0xb1a59c>
140b1b2b4: 0f 10 05 55 e2 fc 01        	movups	0x1fce255(%rip), %xmm0  # 0x142ae9510
140b1b2bb: 0f 11 00                    	movups	%xmm0, (%rax)
140b1b2be: c6 40 10 67                 	movb	$0x67, 0x10(%rax)
140b1b2c2: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
140b1b2ca: 48 c7 45 20 11 00 00 00     	movq	$0x11, 0x20(%rbp)
140b1b2d2: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140b1b2d6: 48 c7 45 30 11 00 00 00     	movq	$0x11, 0x30(%rbp)
140b1b2de: c6 45 38 02                 	movb	$0x2, 0x38(%rbp)
140b1b2e2: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140b1b2e6: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
140b1b2ea: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
140b1b2ee: 4c 8b 45 28                 	movq	0x28(%rbp), %r8
140b1b2f2: 4c 8b 4d 30                 	movq	0x30(%rbp), %r9
140b1b2f6: 4c 89 4d 70                 	movq	%r9, 0x70(%rbp)
140b1b2fa: 44 0f b6 4d 38              	movzbl	0x38(%rbp), %r9d
140b1b2ff: 44 88 4d 78                 	movb	%r9b, 0x78(%rbp)
140b1b303: 44 8b 4d 39                 	movl	0x39(%rbp), %r9d
140b1b307: 44 89 4d 79                 	movl	%r9d, 0x79(%rbp)
140b1b30b: 44 0f b7 4d 3d              	movzwl	0x3d(%rbp), %r9d
140b1b310: 66 44 89 4d 7d              	movw	%r9w, 0x7d(%rbp)
140b1b315: 44 0f b6 4d 3f              	movzbl	0x3f(%rbp), %r9d
140b1b31a: 44 88 4d 7f                 	movb	%r9b, 0x7f(%rbp)
140b1b31e: 48 89 55 60                 	movq	%rdx, 0x60(%rbp)
140b1b322: 4c 89 45 68                 	movq	%r8, 0x68(%rbp)
140b1b326: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140b1b32a: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
140b1b32e: 83 7d 50 01                 	cmpl	$0x1, 0x50(%rbp)
140b1b332: 75 21                       	jne	0x140b1b355 <.text+0xb1a355>
140b1b334: 48 8d 55 58                 	leaq	0x58(%rbp), %rdx
140b1b338: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140b1b33c: e8 8f bf 91 00              	callq	0x1414372d0 <.text+0x14362d0>
140b1b341: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140b1b34b: 48 39 45 d8                 	cmpq	%rax, -0x28(%rbp)
140b1b34f: 0f 85 fa 01 00 00           	jne	0x140b1b54f <.text+0xb1a54f>
140b1b355: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b359: e8 22 ef d7 ff              	callq	0x14089a280 <.text+0x899280>
140b1b35e: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b362: 48 85 d2                    	testq	%rdx, %rdx
140b1b365: 74 0f                       	je	0x140b1b376 <.text+0xb1a376>
140b1b367: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b36b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b371: e8 aa 88 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b376: 48 8b 07                    	movq	(%rdi), %rax
140b1b379: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
140b1b37d: 48 8b 10                    	movq	(%rax), %rdx
140b1b380: 45 31 c0                    	xorl	%r8d, %r8d
140b1b383: 49 89 d1                    	movq	%rdx, %r9
140b1b386: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
140b1b38a: 4d 0f 42 c8                 	cmovbq	%r8, %r9
140b1b38e: 48 2b 51 38                 	subq	0x38(%rcx), %rdx
140b1b392: 49 0f 42 d0                 	cmovbq	%r8, %rdx
140b1b396: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
140b1b39a: 49 29 d2                    	subq	%rdx, %r10
140b1b39d: 4d 0f 42 d0                 	cmovbq	%r8, %r10
140b1b3a1: 0f 97 c2                    	seta	%dl
140b1b3a4: 4c 8b 59 10                 	movq	0x10(%rcx), %r11
140b1b3a8: 4d 29 cb                    	subq	%r9, %r11
140b1b3ab: 4d 0f 42 d8                 	cmovbq	%r8, %r11
140b1b3af: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
140b1b3b3: 41 0f 97 c0                 	seta	%r8b
140b1b3b7: 4d 01 d3                    	addq	%r10, %r11
140b1b3ba: 49 89 c9                    	movq	%rcx, %r9
140b1b3bd: 4d 29 d9                    	subq	%r11, %r9
140b1b3c0: 4c 89 08                    	movq	%r9, (%rax)
140b1b3c3: 41 08 d0                    	orb	%dl, %r8b
140b1b3c6: 75 0f                       	jne	0x140b1b3d7 <.text+0xb1a3d7>
140b1b3c8: 8b 7e 1c                    	movl	0x1c(%rsi), %edi
140b1b3cb: 48 8b 1e                    	movq	(%rsi), %rbx
140b1b3ce: 4c 8b 76 10                 	movq	0x10(%rsi), %r14
140b1b3d2: e9 9d 00 00 00              	jmp	0x140b1b474 <.text+0xb1a474>
140b1b3d7: 48 89 08                    	movq	%rcx, (%rax)
140b1b3da: 8b 7e 1c                    	movl	0x1c(%rsi), %edi
140b1b3dd: 4c 8b 76 10                 	movq	0x10(%rsi), %r14
140b1b3e1: 41 89 3e                    	movl	%edi, (%r14)
140b1b3e4: 48 8b 1e                    	movq	(%rsi), %rbx
140b1b3e7: 48 8b 03                    	movq	(%rbx), %rax
140b1b3ea: 48 8b 4b 28                 	movq	0x28(%rbx), %rcx
140b1b3ee: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
140b1b3f2: 0f 10 43 18                 	movups	0x18(%rbx), %xmm0
140b1b3f6: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140b1b3fa: 0f 10 43 08                 	movups	0x8(%rbx), %xmm0
140b1b3fe: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140b1b402: 48 c7 03 02 00 00 00        	movq	$0x2, (%rbx)
140b1b409: 48 83 f8 02                 	cmpq	$0x2, %rax
140b1b40d: 74 65                       	je	0x140b1b474 <.text+0xb1a474>
140b1b40f: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140b1b413: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
140b1b417: 0f 28 4d c0                 	movaps	-0x40(%rbp), %xmm1
140b1b41b: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140b1b41f: 0f 11 4d 68                 	movups	%xmm1, 0x68(%rbp)
140b1b423: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
140b1b427: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
140b1b42b: a8 01                       	testb	$0x1, %al
140b1b42d: 74 24                       	je	0x140b1b453 <.text+0xb1a453>
140b1b42f: 48 8d 55 58                 	leaq	0x58(%rbp), %rdx
140b1b433: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140b1b437: 45 31 c0                    	xorl	%r8d, %r8d
140b1b43a: e8 01 c0 91 00              	callq	0x141437440 <.text+0x1436440>
140b1b43f: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140b1b449: 48 39 45 d8                 	cmpq	%rax, -0x28(%rbp)
140b1b44d: 0f 85 5f 01 00 00           	jne	0x140b1b5b2 <.text+0xb1a5b2>
140b1b453: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b457: e8 24 ee d7 ff              	callq	0x14089a280 <.text+0x899280>
140b1b45c: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b460: 48 85 d2                    	testq	%rdx, %rdx
140b1b463: 74 0f                       	je	0x140b1b474 <.text+0xb1a474>
140b1b465: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b469: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b46f: e8 ac 87 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b474: 41 89 3e                    	movl	%edi, (%r14)
140b1b477: f6 43 30 01                 	testb	$0x1, 0x30(%rbx)
140b1b47b: 0f 84 c0 00 00 00           	je	0x140b1b541 <.text+0xb1a541>
140b1b481: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140b1b488: 48 8b 00                    	movq	(%rax), %rax
140b1b48b: f3 0f 10 40 28              	movss	0x28(%rax), %xmm0
140b1b490: f3 0f 58 43 34              	addss	0x34(%rbx), %xmm0
140b1b495: f3 0f 11 43 34              	movss	%xmm0, 0x34(%rbx)
140b1b49a: 0f 2e 05 63 a4 e0 01        	ucomiss	0x1e0a463(%rip), %xmm0  # 0x142925904
140b1b4a1: 0f 86 9a 00 00 00           	jbe	0x140b1b541 <.text+0xb1a541>
140b1b4a7: 41 89 3e                    	movl	%edi, (%r14)
140b1b4aa: 48 8b 03                    	movq	(%rbx), %rax
140b1b4ad: 48 8b 4b 28                 	movq	0x28(%rbx), %rcx
140b1b4b1: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
140b1b4b5: 0f 10 43 18                 	movups	0x18(%rbx), %xmm0
140b1b4b9: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140b1b4bd: 0f 10 43 08                 	movups	0x8(%rbx), %xmm0
140b1b4c1: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140b1b4c5: 48 c7 03 02 00 00 00        	movq	$0x2, (%rbx)
140b1b4cc: 48 83 f8 02                 	cmpq	$0x2, %rax
140b1b4d0: 74 65                       	je	0x140b1b537 <.text+0xb1a537>
140b1b4d2: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140b1b4d6: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
140b1b4da: 0f 28 4d c0                 	movaps	-0x40(%rbp), %xmm1
140b1b4de: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140b1b4e2: 0f 11 4d 68                 	movups	%xmm1, 0x68(%rbp)
140b1b4e6: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
140b1b4ea: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
140b1b4ee: a8 01                       	testb	$0x1, %al
140b1b4f0: 74 24                       	je	0x140b1b516 <.text+0xb1a516>
140b1b4f2: 48 8d 55 58                 	leaq	0x58(%rbp), %rdx
140b1b4f6: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140b1b4fa: 45 31 c0                    	xorl	%r8d, %r8d
140b1b4fd: e8 3e bf 91 00              	callq	0x141437440 <.text+0x1436440>
140b1b502: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140b1b50c: 48 39 45 d8                 	cmpq	%rax, -0x28(%rbp)
140b1b510: 0f 85 e6 00 00 00           	jne	0x140b1b5fc <.text+0xb1a5fc>
140b1b516: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b51a: e8 61 ed d7 ff              	callq	0x14089a280 <.text+0x899280>
140b1b51f: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b523: 48 85 d2                    	testq	%rdx, %rdx
140b1b526: 74 0f                       	je	0x140b1b537 <.text+0xb1a537>
140b1b528: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b52c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b532: e8 e9 86 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b537: 41 89 3e                    	movl	%edi, (%r14)
140b1b53a: c7 43 30 00 00 00 00        	movl	$0x0, 0x30(%rbx)
140b1b541: 48 81 c4 10 01 00 00        	addq	$0x110, %rsp            # imm = 0x110
140b1b548: 5b                          	popq	%rbx
140b1b549: 5f                          	popq	%rdi
140b1b54a: 5e                          	popq	%rsi
140b1b54b: 41 5e                       	popq	%r14
140b1b54d: 5d                          	popq	%rbp
140b1b54e: c3                          	retq
140b1b54f: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140b1b553: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140b1b557: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
140b1b55b: 0f 10 4d e8                 	movups	-0x18(%rbp), %xmm1
140b1b55f: 0f 10 55 f8                 	movups	-0x8(%rbp), %xmm2
140b1b563: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b1b567: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b1b56b: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b1b56f: 48 8d 05 4a 76 fb 01        	leaq	0x1fb764a(%rip), %rax   # 0x142ad2bc0
140b1b576: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1b57b: 48 8d 0d 7e 50 fb 01        	leaq	0x1fb507e(%rip), %rcx   # 0x142ad0600
140b1b582: 4c 8d 0d 37 50 fb 01        	leaq	0x1fb5037(%rip), %r9    # 0x142ad05c0
140b1b589: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
140b1b58d: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1b592: e8 c9 e1 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1b597: e9 a8 00 00 00              	jmp	0x140b1b644 <.text+0xb1a644>
140b1b59c: 4c 8d 05 0d 52 fb 01        	leaq	0x1fb520d(%rip), %r8    # 0x142ad07b0
140b1b5a3: b9 01 00 00 00              	movl	$0x1, %ecx
140b1b5a8: ba 11 00 00 00              	movl	$0x11, %edx
140b1b5ad: e8 31 da db 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140b1b5b2: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140b1b5b6: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140b1b5ba: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
140b1b5be: 0f 10 4d e8                 	movups	-0x18(%rbp), %xmm1
140b1b5c2: 0f 10 55 f8                 	movups	-0x8(%rbp), %xmm2
140b1b5c6: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b1b5ca: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b1b5ce: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b1b5d2: 48 8d 05 2f 76 fb 01        	leaq	0x1fb762f(%rip), %rax   # 0x142ad2c08
140b1b5d9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1b5de: 48 8d 0d 1b 50 fb 01        	leaq	0x1fb501b(%rip), %rcx   # 0x142ad0600
140b1b5e5: 4c 8d 0d d4 4f fb 01        	leaq	0x1fb4fd4(%rip), %r9    # 0x142ad05c0
140b1b5ec: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
140b1b5f0: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1b5f5: e8 66 e1 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1b5fa: eb 48                       	jmp	0x140b1b644 <.text+0xb1a644>
140b1b5fc: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140b1b600: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140b1b604: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
140b1b608: 0f 10 4d e8                 	movups	-0x18(%rbp), %xmm1
140b1b60c: 0f 10 55 f8                 	movups	-0x8(%rbp), %xmm2
140b1b610: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b1b614: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b1b618: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b1b61c: 48 8d 05 e5 75 fb 01        	leaq	0x1fb75e5(%rip), %rax   # 0x142ad2c08
140b1b623: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b1b628: 48 8d 0d d1 4f fb 01        	leaq	0x1fb4fd1(%rip), %rcx   # 0x142ad0600
140b1b62f: 4c 8d 0d 8a 4f fb 01        	leaq	0x1fb4f8a(%rip), %r9    # 0x142ad05c0
140b1b636: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
140b1b63a: ba 2b 00 00 00              	movl	$0x2b, %edx
140b1b63f: e8 1c e1 db 01              	callq	0x1428d9760 <.text+0x28d8760>
140b1b644: 0f 0b                       	ud2
140b1b646: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140b1b650: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b655: 55                          	pushq	%rbp
140b1b656: 41 56                       	pushq	%r14
140b1b658: 56                          	pushq	%rsi
140b1b659: 57                          	pushq	%rdi
140b1b65a: 53                          	pushq	%rbx
140b1b65b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b65f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b666: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b1b66a: e8 41 70 f3 ff              	callq	0x140a526b0 <.text+0xa516b0>
140b1b66f: 90                          	nop
140b1b670: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b674: 5b                          	popq	%rbx
140b1b675: 5f                          	popq	%rdi
140b1b676: 5e                          	popq	%rsi
140b1b677: 41 5e                       	popq	%r14
140b1b679: 5d                          	popq	%rbp
140b1b67a: c3                          	retq
140b1b67b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b680: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b685: 55                          	pushq	%rbp
140b1b686: 41 56                       	pushq	%r14
140b1b688: 56                          	pushq	%rsi
140b1b689: 57                          	pushq	%rdi
140b1b68a: 53                          	pushq	%rbx
140b1b68b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b68f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b696: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b69a: e8 e1 ef 62 ff              	callq	0x14014a680 <.text+0x149680>
140b1b69f: 90                          	nop
140b1b6a0: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b6a4: 5b                          	popq	%rbx
140b1b6a5: 5f                          	popq	%rdi
140b1b6a6: 5e                          	popq	%rsi
140b1b6a7: 41 5e                       	popq	%r14
140b1b6a9: 5d                          	popq	%rbp
140b1b6aa: c3                          	retq
140b1b6ab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b6b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b6b5: 55                          	pushq	%rbp
140b1b6b6: 41 56                       	pushq	%r14
140b1b6b8: 56                          	pushq	%rsi
140b1b6b9: 57                          	pushq	%rdi
140b1b6ba: 53                          	pushq	%rbx
140b1b6bb: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b6bf: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b6c6: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b6ca: 48 85 d2                    	testq	%rdx, %rdx
140b1b6cd: 74 0f                       	je	0x140b1b6de <.text+0xb1a6de>
140b1b6cf: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b6d3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b6d9: e8 42 85 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b6de: 90                          	nop
140b1b6df: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b6e3: 5b                          	popq	%rbx
140b1b6e4: 5f                          	popq	%rdi
140b1b6e5: 5e                          	popq	%rsi
140b1b6e6: 41 5e                       	popq	%r14
140b1b6e8: 5d                          	popq	%rbp
140b1b6e9: c3                          	retq
140b1b6ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b1b6f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b6f5: 55                          	pushq	%rbp
140b1b6f6: 41 56                       	pushq	%r14
140b1b6f8: 56                          	pushq	%rsi
140b1b6f9: 57                          	pushq	%rdi
140b1b6fa: 53                          	pushq	%rbx
140b1b6fb: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b6ff: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b706: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b70a: e8 71 ef 62 ff              	callq	0x14014a680 <.text+0x149680>
140b1b70f: 90                          	nop
140b1b710: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b714: 5b                          	popq	%rbx
140b1b715: 5f                          	popq	%rdi
140b1b716: 5e                          	popq	%rsi
140b1b717: 41 5e                       	popq	%r14
140b1b719: 5d                          	popq	%rbp
140b1b71a: c3                          	retq
140b1b71b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b720: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b725: 55                          	pushq	%rbp
140b1b726: 41 56                       	pushq	%r14
140b1b728: 56                          	pushq	%rsi
140b1b729: 57                          	pushq	%rdi
140b1b72a: 53                          	pushq	%rbx
140b1b72b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b72f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b736: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b73a: 48 85 d2                    	testq	%rdx, %rdx
140b1b73d: 74 0f                       	je	0x140b1b74e <.text+0xb1a74e>
140b1b73f: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b743: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b749: e8 d2 84 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b74e: 90                          	nop
140b1b74f: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b753: 5b                          	popq	%rbx
140b1b754: 5f                          	popq	%rdi
140b1b755: 5e                          	popq	%rsi
140b1b756: 41 5e                       	popq	%r14
140b1b758: 5d                          	popq	%rbp
140b1b759: c3                          	retq
140b1b75a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b1b760: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b765: 55                          	pushq	%rbp
140b1b766: 41 56                       	pushq	%r14
140b1b768: 56                          	pushq	%rsi
140b1b769: 57                          	pushq	%rdi
140b1b76a: 53                          	pushq	%rbx
140b1b76b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b76f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b776: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b1b77a: e8 01 ef 62 ff              	callq	0x14014a680 <.text+0x149680>
140b1b77f: 90                          	nop
140b1b780: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b784: 5b                          	popq	%rbx
140b1b785: 5f                          	popq	%rdi
140b1b786: 5e                          	popq	%rsi
140b1b787: 41 5e                       	popq	%r14
140b1b789: 5d                          	popq	%rbp
140b1b78a: c3                          	retq
140b1b78b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b790: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b795: 55                          	pushq	%rbp
140b1b796: 41 56                       	pushq	%r14
140b1b798: 56                          	pushq	%rsi
140b1b799: 57                          	pushq	%rdi
140b1b79a: 53                          	pushq	%rbx
140b1b79b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b79f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b7a6: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140b1b7aa: 48 85 d2                    	testq	%rdx, %rdx
140b1b7ad: 74 0f                       	je	0x140b1b7be <.text+0xb1a7be>
140b1b7af: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140b1b7b3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140b1b7b9: e8 62 84 af ff              	callq	0x140613c20 <.text+0x612c20>
140b1b7be: 90                          	nop
140b1b7bf: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b7c3: 5b                          	popq	%rbx
140b1b7c4: 5f                          	popq	%rdi
140b1b7c5: 5e                          	popq	%rsi
140b1b7c6: 41 5e                       	popq	%r14
140b1b7c8: 5d                          	popq	%rbp
140b1b7c9: c3                          	retq
140b1b7ca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140b1b7d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b7d5: 55                          	pushq	%rbp
140b1b7d6: 41 56                       	pushq	%r14
140b1b7d8: 56                          	pushq	%rsi
140b1b7d9: 57                          	pushq	%rdi
140b1b7da: 53                          	pushq	%rbx
140b1b7db: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b7df: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b7e6: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140b1b7ea: e8 41 14 23 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140b1b7ef: 90                          	nop
140b1b7f0: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b7f4: 5b                          	popq	%rbx
140b1b7f5: 5f                          	popq	%rdi
140b1b7f6: 5e                          	popq	%rsi
140b1b7f7: 41 5e                       	popq	%r14
140b1b7f9: 5d                          	popq	%rbp
140b1b7fa: c3                          	retq
140b1b7fb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b800: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b805: 55                          	pushq	%rbp
140b1b806: 41 56                       	pushq	%r14
140b1b808: 56                          	pushq	%rsi
140b1b809: 57                          	pushq	%rdi
140b1b80a: 53                          	pushq	%rbx
140b1b80b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b80f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b816: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b1b81a: e8 91 6e f3 ff              	callq	0x140a526b0 <.text+0xa516b0>
140b1b81f: 90                          	nop
140b1b820: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b824: 5b                          	popq	%rbx
140b1b825: 5f                          	popq	%rdi
140b1b826: 5e                          	popq	%rsi
140b1b827: 41 5e                       	popq	%r14
140b1b829: 5d                          	popq	%rbp
140b1b82a: c3                          	retq
140b1b82b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b1b830: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b1b835: 55                          	pushq	%rbp
140b1b836: 41 56                       	pushq	%r14
140b1b838: 56                          	pushq	%rsi
140b1b839: 57                          	pushq	%rdi
140b1b83a: 53                          	pushq	%rbx
140b1b83b: 48 83 ec 30                 	subq	$0x30, %rsp
140b1b83f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b1b846: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b1b84a: e8 61 6e f3 ff              	callq	0x140a526b0 <.text+0xa516b0>
140b1b84f: 90                          	nop
140b1b850: 48 83 c4 30                 	addq	$0x30, %rsp
140b1b854: 5b                          	popq	%rbx
140b1b855: 5f                          	popq	%rdi
140b1b856: 5e                          	popq	%rsi
140b1b857: 41 5e                       	popq	%r14
140b1b859: 5d                          	popq	%rbp
140b1b85a: c3                          	retq
