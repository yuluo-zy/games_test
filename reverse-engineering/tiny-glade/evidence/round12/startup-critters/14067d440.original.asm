
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14067d440: 56                          	pushq	%rsi
14067d441: 57                          	pushq	%rdi
14067d442: 53                          	pushq	%rbx
14067d443: 48 81 ec 20 01 00 00        	subq	$0x120, %rsp            # imm = 0x120
14067d44a: 0f 29 b4 24 10 01 00 00     	movaps	%xmm6, 0x110(%rsp)
14067d452: 48 89 d6                    	movq	%rdx, %rsi
14067d455: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
14067d45a: 48 89 d9                    	movq	%rbx, %rcx
14067d45d: e8 ce ff 8f 00              	callq	0x140f7d430 <.text+0xf7c430>
14067d462: 48 8d bc 24 a0 00 00 00     	leaq	0xa0(%rsp), %rdi
14067d46a: 48 89 f9                    	movq	%rdi, %rcx
14067d46d: 48 89 da                    	movq	%rbx, %rdx
14067d470: e8 6b 10 90 00              	callq	0x140f7e4e0 <.text+0xf7d4e0>
14067d475: 4c 8d 0d 64 98 39 02        	leaq	0x2399864(%rip), %r9    # 0x142a16ce0
14067d47c: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
14067d481: 48 89 d9                    	movq	%rbx, %rcx
14067d484: 48 89 fa                    	movq	%rdi, %rdx
14067d487: 41 b0 24                    	movb	$0x24, %r8b
14067d48a: e8 41 2d 90 00              	callq	0x140f801d0 <.text+0xf7f1d0>
14067d48f: 48 89 f1                    	movq	%rsi, %rcx
14067d492: 48 89 da                    	movq	%rbx, %rdx
14067d495: e8 86 31 90 00              	callq	0x140f80620 <.text+0xf7f620>
14067d49a: 4c 8d 05 57 98 39 02        	leaq	0x2399857(%rip), %r8    # 0x142a16cf8
14067d4a1: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
14067d4a6: 48 89 d9                    	movq	%rbx, %rcx
14067d4a9: b2 2c                       	movb	$0x2c, %dl
14067d4ab: e8 30 05 90 00              	callq	0x140f7d9e0 <.text+0xf7c9e0>
14067d4b0: 48 89 f1                    	movq	%rsi, %rcx
14067d4b3: 48 89 da                    	movq	%rbx, %rdx
14067d4b6: e8 65 31 90 00              	callq	0x140f80620 <.text+0xf7f620>
14067d4bb: 48 8d 4e 38                 	leaq	0x38(%rsi), %rcx
14067d4bf: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
14067d4c9: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
14067d4d3: e8 18 02 a6 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14067d4d8: 48 85 c0                    	testq	%rax, %rax
14067d4db: 0f 84 9e 02 00 00           	je	0x14067d77f <.text+0x67c77f>
14067d4e1: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14067d4e8: 0f 86 91 02 00 00           	jbe	0x14067d77f <.text+0x67c77f>
14067d4ee: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14067d4f5: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
14067d4f9: 48 85 c9                    	testq	%rcx, %rcx
14067d4fc: 0f 84 7d 02 00 00           	je	0x14067d77f <.text+0x67c77f>
14067d502: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
14067d509: 48 f7 d1                    	notq	%rcx
14067d50c: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14067d510: 48 c1 e1 05                 	shlq	$0x5, %rcx
14067d514: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
14067d51a: 0f 84 5f 02 00 00           	je	0x14067d77f <.text+0x67c77f>
14067d520: 48 01 c8                    	addq	%rcx, %rax
14067d523: 48 8b 48 10                 	movq	0x10(%rax), %rcx
14067d527: 8b 96 b0 05 00 00           	movl	0x5b0(%rsi), %edx
14067d52d: 89 50 5c                    	movl	%edx, 0x5c(%rax)
14067d530: b2 01                       	movb	$0x1, %dl
14067d532: e8 69 4a 90 00              	callq	0x140f81fa0 <.text+0xf80fa0>
14067d537: ba df c0 00 00              	movl	$0xc0df, %edx           # imm = 0xC0DF
14067d53c: 48 89 f1                    	movq	%rsi, %rcx
14067d53f: 45 31 c0                    	xorl	%r8d, %r8d
14067d542: e8 e9 6d 90 00              	callq	0x140f84330 <.text+0xf83330>
14067d547: 0f 57 f6                    	xorps	%xmm6, %xmm6
14067d54a: 0f 11 74 24 30              	movups	%xmm6, 0x30(%rsp)
14067d54f: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
14067d558: 48 c7 44 24 28 04 00 00 00  	movq	$0x4, 0x28(%rsp)
14067d561: 48 c7 44 24 40 08 00 00 00  	movq	$0x8, 0x40(%rsp)
14067d56a: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
14067d573: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
14067d57d: 48 89 4c 24 50              	movq	%rcx, 0x50(%rsp)
14067d582: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
14067d587: 48 89 c1                    	movq	%rax, %rcx
14067d58a: e8 e1 7e 90 00              	callq	0x140f85470 <.text+0xf84470>
14067d58f: 48 89 c6                    	movq	%rax, %rsi
14067d592: e8 f9 85 90 00              	callq	0x140f85b90 <.text+0xf84b90>
14067d597: 48 89 f1                    	movq	%rsi, %rcx
14067d59a: 48 89 c2                    	movq	%rax, %rdx
14067d59d: e8 4e 62 90 00              	callq	0x140f837f0 <.text+0xf827f0>
14067d5a2: 48 c7 44 24 38 08 00 00 00  	movq	$0x8, 0x38(%rsp)
14067d5ab: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
14067d5b4: 48 b9 99 06 61 6e 06 00 00 00       	movabsq	$0x66e610699, %rcx # imm = 0x66E610699
14067d5be: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14067d5c3: 0f 11 74 24 28              	movups	%xmm6, 0x28(%rsp)
14067d5c8: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
14067d5cd: 48 89 c1                    	movq	%rax, %rcx
14067d5d0: e8 2b 56 90 00              	callq	0x140f82c00 <.text+0xf81c00>
14067d5d5: 48 89 c6                    	movq	%rax, %rsi
14067d5d8: 48 8d 7c 24 20              	leaq	0x20(%rsp), %rdi
14067d5dd: 48 89 f9                    	movq	%rdi, %rcx
14067d5e0: e8 ab 18 91 00              	callq	0x140f8ee90 <.text+0xf8de90>
14067d5e5: 48 89 f1                    	movq	%rsi, %rcx
14067d5e8: 48 89 fa                    	movq	%rdi, %rdx
14067d5eb: e8 00 5c 90 00              	callq	0x140f831f0 <.text+0xf821f0>
14067d5f0: 48 89 c6                    	movq	%rax, %rsi
14067d5f3: 0f b6 05 0f 76 fb 02        	movzbl	0x2fb760f(%rip), %eax   # 0x143634c09
14067d5fa: b9 30 00 00 00              	movl	$0x30, %ecx
14067d5ff: ba 08 00 00 00              	movl	$0x8, %edx
14067d604: e8 07 66 f9 ff              	callq	0x140613c10 <.text+0x612c10>
14067d609: 48 85 c0                    	testq	%rax, %rax
14067d60c: 0f 84 e4 01 00 00           	je	0x14067d7f6 <.text+0x67c7f6>
14067d612: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14067d619: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14067d621: c6 40 10 00                 	movb	$0x0, 0x10(%rax)
14067d625: 48 c7 40 18 00 00 00 00     	movq	$0x0, 0x18(%rax)
14067d62d: 48 c7 40 20 08 00 00 00     	movq	$0x8, 0x20(%rax)
14067d635: 48 c7 40 28 00 00 00 00     	movq	$0x0, 0x28(%rax)
14067d63d: 48 89 f1                    	movq	%rsi, %rcx
14067d640: 48 89 c2                    	movq	%rax, %rdx
14067d643: 45 31 c0                    	xorl	%r8d, %r8d
14067d646: e8 e5 66 90 00              	callq	0x140f83d30 <.text+0xf82d30>
14067d64b: 0f 11 74 24 30              	movups	%xmm6, 0x30(%rsp)
14067d650: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
14067d659: 48 c7 44 24 28 04 00 00 00  	movq	$0x4, 0x28(%rsp)
14067d662: 48 c7 44 24 40 08 00 00 00  	movq	$0x8, 0x40(%rsp)
14067d66b: 0f 11 74 24 48              	movups	%xmm6, 0x48(%rsp)
14067d670: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
14067d675: 48 89 c1                    	movq	%rax, %rcx
14067d678: e8 03 78 90 00              	callq	0x140f84e80 <.text+0xf83e80>
14067d67d: 48 89 c6                    	movq	%rax, %rsi
14067d680: 48 8d 7c 24 20              	leaq	0x20(%rsp), %rdi
14067d685: 48 89 f9                    	movq	%rdi, %rcx
14067d688: e8 63 33 91 00              	callq	0x140f909f0 <.text+0xf8f9f0>
14067d68d: 48 89 f1                    	movq	%rsi, %rcx
14067d690: 48 89 fa                    	movq	%rdi, %rdx
14067d693: e8 e8 71 90 00              	callq	0x140f84880 <.text+0xf83880>
14067d698: 48 89 c6                    	movq	%rax, %rsi
14067d69b: 48 89 c7                    	movq	%rax, %rdi
14067d69e: 48 83 c7 38                 	addq	$0x38, %rdi
14067d6a2: 48 ba e0 df 19 48 83 08 0a ac       	movabsq	$-0x53f5f77cb7e62020, %rdx # imm = 0xAC0A08834819DFE0
14067d6ac: 49 b8 47 95 a6 87 fa 2e c7 62       	movabsq	$0x62c72efa87a69547, %r8 # imm = 0x62C72EFA87A69547
14067d6b6: 48 89 f9                    	movq	%rdi, %rcx
14067d6b9: e8 32 00 a6 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14067d6be: 48 85 c0                    	testq	%rax, %rax
14067d6c1: 74 33                       	je	0x14067d6f6 <.text+0x67c6f6>
14067d6c3: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14067d6ca: 76 2a                       	jbe	0x14067d6f6 <.text+0x67c6f6>
14067d6cc: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14067d6d3: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14067d6d7: 48 85 c0                    	testq	%rax, %rax
14067d6da: 74 1a                       	je	0x14067d6f6 <.text+0x67c6f6>
14067d6dc: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14067d6e3: 48 f7 d0                    	notq	%rax
14067d6e6: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14067d6ea: 48 c1 e0 05                 	shlq	$0x5, %rax
14067d6ee: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14067d6f4: 75 08                       	jne	0x14067d6fe <.text+0x67c6fe>
14067d6f6: 48 89 f1                    	movq	%rsi, %rcx
14067d6f9: e8 72 59 91 00              	callq	0x140f93070 <.text+0xf92070>
14067d6fe: 48 ba bf 4a af 81 35 a9 64 45       	movabsq	$0x4564a93581af4abf, %rdx # imm = 0x4564A93581AF4ABF
14067d708: 49 b8 dc 70 85 3e 2e 3a f8 6f       	movabsq	$0x6ff83a2e3e8570dc, %r8 # imm = 0x6FF83A2E3E8570DC
14067d712: 48 89 f9                    	movq	%rdi, %rcx
14067d715: e8 d6 ff a5 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14067d71a: 48 85 c0                    	testq	%rax, %rax
14067d71d: 74 46                       	je	0x14067d765 <.text+0x67c765>
14067d71f: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14067d726: 76 3d                       	jbe	0x14067d765 <.text+0x67c765>
14067d728: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14067d72f: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14067d733: 48 85 c0                    	testq	%rax, %rax
14067d736: 74 2d                       	je	0x14067d765 <.text+0x67c765>
14067d738: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14067d73f: 48 f7 d0                    	notq	%rax
14067d742: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14067d746: 48 c1 e0 05                 	shlq	$0x5, %rax
14067d74a: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14067d750: 74 13                       	je	0x14067d765 <.text+0x67c765>
14067d752: 0f 28 b4 24 10 01 00 00     	movaps	0x110(%rsp), %xmm6
14067d75a: 48 81 c4 20 01 00 00        	addq	$0x120, %rsp            # imm = 0x120
14067d761: 5b                          	popq	%rbx
14067d762: 5f                          	popq	%rdi
14067d763: 5e                          	popq	%rsi
14067d764: c3                          	retq
14067d765: 48 89 f1                    	movq	%rsi, %rcx
14067d768: 0f 28 b4 24 10 01 00 00     	movaps	0x110(%rsp), %xmm6
14067d770: 48 81 c4 20 01 00 00        	addq	$0x120, %rsp            # imm = 0x120
14067d777: 5b                          	popq	%rbx
14067d778: 5f                          	popq	%rdi
14067d779: 5e                          	popq	%rsi
14067d77a: e9 81 6b 91 00              	jmp	0x140f94300 <.text+0xf93300>
14067d77f: 48 8d 05 5a f3 52 02        	leaq	0x252f35a(%rip), %rax   # 0x142bacae0
14067d786: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
14067d78e: 48 c7 84 24 a8 00 00 00 27 00 00 00 	movq	$0x27, 0xa8(%rsp)
14067d79a: 48 89 bc 24 90 00 00 00     	movq	%rdi, 0x90(%rsp)
14067d7a2: 48 8d 05 e7 c6 a8 ff        	leaq	-0x573919(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14067d7a9: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
14067d7b1: 48 8d 05 58 f4 52 02        	leaq	0x252f458(%rip), %rax   # 0x142bacc10
14067d7b8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14067d7bd: 48 c7 44 24 28 02 00 00 00  	movq	$0x2, 0x28(%rsp)
14067d7c6: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
14067d7cf: 48 8d 84 24 90 00 00 00     	leaq	0x90(%rsp), %rax
14067d7d7: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14067d7dc: 48 c7 44 24 38 01 00 00 00  	movq	$0x1, 0x38(%rsp)
14067d7e5: 48 8d 15 dc f2 52 02        	leaq	0x252f2dc(%rip), %rdx   # 0x142bacac8
14067d7ec: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
14067d7f1: e8 9a bb 25 02              	callq	0x1428d9390 <.text+0x28d8390>
14067d7f6: b9 08 00 00 00              	movl	$0x8, %ecx
14067d7fb: ba 30 00 00 00              	movl	$0x30, %edx
14067d800: e8 fb b7 25 02              	callq	0x1428d9000 <.text+0x28d8000>
14067d805: cc                          	int3
