
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aa3360: 55                          	pushq	%rbp
140aa3361: 41 57                       	pushq	%r15
140aa3363: 41 56                       	pushq	%r14
140aa3365: 56                          	pushq	%rsi
140aa3366: 57                          	pushq	%rdi
140aa3367: 53                          	pushq	%rbx
140aa3368: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140aa336f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140aa3377: 48 c7 45 10 fe ff ff ff     	movq	$-0x2, 0x10(%rbp)
140aa337f: 4c 89 ce                    	movq	%r9, %rsi
140aa3382: 4c 89 c7                    	movq	%r8, %rdi
140aa3385: 48 89 d3                    	movq	%rdx, %rbx
140aa3388: 48 8b 11                    	movq	(%rcx), %rdx
140aa338b: 48 85 d2                    	testq	%rdx, %rdx
140aa338e: 74 50                       	je	0x140aa33e0 <.text+0xaa23e0>
140aa3390: 48 83 c2 10                 	addq	$0x10, %rdx
140aa3394: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140aa3398: 49 89 d8                    	movq	%rbx, %r8
140aa339b: 49 89 f9                    	movq	%rdi, %r9
140aa339e: e8 9d 10 fb ff              	callq	0x140a54440 <.text+0xa53440>
140aa33a3: 48 83 7d b0 02              	cmpq	$0x2, -0x50(%rbp)
140aa33a8: 0f 85 91 00 00 00           	jne	0x140aa343f <.text+0xaa243f>
140aa33ae: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
140aa33b2: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140aa33b6: 48 8d 05 03 98 03 02        	leaq	0x2039803(%rip), %rax   # 0x142adcbc0
140aa33bd: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140aa33c2: 48 8d 0d df 93 03 02        	leaq	0x20393df(%rip), %rcx   # 0x142adc7a8
140aa33c9: 4c 8d 0d b8 93 03 02        	leaq	0x20393b8(%rip), %r9    # 0x142adc788
140aa33d0: 4c 8d 45 e0                 	leaq	-0x20(%rbp), %r8
140aa33d4: ba 2b 00 00 00              	movl	$0x2b, %edx
140aa33d9: e8 82 63 e3 01              	callq	0x1428d9760 <.text+0x28d8760>
140aa33de: 0f 0b                       	ud2
140aa33e0: 48 85 ff                    	testq	%rdi, %rdi
140aa33e3: 0f 88 e9 00 00 00           	js	0x140aa34d2 <.text+0xaa24d2>
140aa33e9: 74 28                       	je	0x140aa3413 <.text+0xaa2413>
140aa33eb: 0f b6 05 17 18 b9 02        	movzbl	0x2b91817(%rip), %eax   # 0x143634c09
140aa33f2: 41 bf 01 00 00 00           	movl	$0x1, %r15d
140aa33f8: ba 01 00 00 00              	movl	$0x1, %edx
140aa33fd: 48 89 f9                    	movq	%rdi, %rcx
140aa3400: e8 0b 08 b7 ff              	callq	0x140613c10 <.text+0x612c10>
140aa3405: 48 85 c0                    	testq	%rax, %rax
140aa3408: 0f 84 c7 00 00 00           	je	0x140aa34d5 <.text+0xaa24d5>
140aa340e: 49 89 c6                    	movq	%rax, %r14
140aa3411: eb 06                       	jmp	0x140aa3419 <.text+0xaa2419>
140aa3413: 41 be 01 00 00 00           	movl	$0x1, %r14d
140aa3419: 4c 89 f1                    	movq	%r14, %rcx
140aa341c: 48 89 da                    	movq	%rbx, %rdx
140aa341f: 49 89 f8                    	movq	%rdi, %r8
140aa3422: e8 d9 07 e8 01              	callq	0x142923c00 <.text+0x2922c00>
140aa3427: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
140aa342f: 48 89 7d c0                 	movq	%rdi, -0x40(%rbp)
140aa3433: 4c 89 75 c8                 	movq	%r14, -0x38(%rbp)
140aa3437: 48 89 7d d0                 	movq	%rdi, -0x30(%rbp)
140aa343b: c6 45 d8 02                 	movb	$0x2, -0x28(%rbp)
140aa343f: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
140aa3443: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
140aa3447: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
140aa344b: 4c 8b 45 c8                 	movq	-0x38(%rbp), %r8
140aa344f: 4c 8b 4d d0                 	movq	-0x30(%rbp), %r9
140aa3453: 4c 89 4d 00                 	movq	%r9, (%rbp)
140aa3457: 44 0f b6 4d d8              	movzbl	-0x28(%rbp), %r9d
140aa345c: 44 88 4d 08                 	movb	%r9b, 0x8(%rbp)
140aa3460: 44 8b 4d d9                 	movl	-0x27(%rbp), %r9d
140aa3464: 44 89 4d 09                 	movl	%r9d, 0x9(%rbp)
140aa3468: 44 0f b7 4d dd              	movzwl	-0x23(%rbp), %r9d
140aa346d: 66 44 89 4d 0d              	movw	%r9w, 0xd(%rbp)
140aa3472: 44 0f b6 4d df              	movzbl	-0x21(%rbp), %r9d
140aa3477: 44 88 4d 0f                 	movb	%r9b, 0xf(%rbp)
140aa347b: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
140aa347f: 4c 89 45 f8                 	movq	%r8, -0x8(%rbp)
140aa3483: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140aa3487: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140aa348b: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140aa348f: 48 89 f2                    	movq	%rsi, %rdx
140aa3492: e8 09 49 fb ff              	callq	0x140a57da0 <.text+0xa56da0>
140aa3497: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140aa349b: e8 60 3e fb ff              	callq	0x140a57300 <.text+0xa56300>
140aa34a0: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140aa34a4: e8 d7 6d df ff              	callq	0x14089a280 <.text+0x899280>
140aa34a9: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
140aa34ad: 48 85 d2                    	testq	%rdx, %rdx
140aa34b0: 74 0f                       	je	0x140aa34c1 <.text+0xaa24c1>
140aa34b2: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140aa34b6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140aa34bc: e8 5f 07 b7 ff              	callq	0x140613c20 <.text+0x612c20>
140aa34c1: 90                          	nop
140aa34c2: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140aa34c9: 5b                          	popq	%rbx
140aa34ca: 5f                          	popq	%rdi
140aa34cb: 5e                          	popq	%rsi
140aa34cc: 41 5e                       	popq	%r14
140aa34ce: 41 5f                       	popq	%r15
140aa34d0: 5d                          	popq	%rbp
140aa34d1: c3                          	retq
140aa34d2: 45 31 ff                    	xorl	%r15d, %r15d
140aa34d5: 4c 8d 05 d4 d2 02 02        	leaq	0x202d2d4(%rip), %r8    # 0x142ad07b0
140aa34dc: 4c 89 f9                    	movq	%r15, %rcx
140aa34df: 48 89 fa                    	movq	%rdi, %rdx
140aa34e2: e8 fc 5a e3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140aa34e7: cc                          	int3
