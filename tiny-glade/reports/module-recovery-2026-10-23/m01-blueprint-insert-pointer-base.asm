
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408258b0: 55                          	pushq	%rbp
1408258b1: 41 57                       	pushq	%r15
1408258b3: 41 56                       	pushq	%r14
1408258b5: 41 55                       	pushq	%r13
1408258b7: 41 54                       	pushq	%r12
1408258b9: 56                          	pushq	%rsi
1408258ba: 57                          	pushq	%rdi
1408258bb: 53                          	pushq	%rbx
1408258bc: 48 81 ec 88 01 00 00        	subq	$0x188, %rsp            # imm = 0x188
1408258c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408258cb: 48 c7 85 00 01 00 00 fe ff ff ff    	movq	$-0x2, 0x100(%rbp)
1408258d6: 4c 89 ce                    	movq	%r9, %rsi
1408258d9: 4d 89 c6                    	movq	%r8, %r14
1408258dc: 48 89 cb                    	movq	%rcx, %rbx
1408258df: 89 95 f8 00 00 00           	movl	%edx, 0xf8(%rbp)
1408258e5: 89 55 a0                    	movl	%edx, -0x60(%rbp)
1408258e8: 4c 8d 79 20                 	leaq	0x20(%rcx), %r15
1408258ec: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
1408258f0: 4c 89 f9                    	movq	%r15, %rcx
1408258f3: e8 d8 b5 d8 ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1408258f8: 49 89 c5                    	movq	%rax, %r13
1408258fb: 49 c1 ed 39                 	shrq	$0x39, %r13
1408258ff: 48 8b 0b                    	movq	(%rbx), %rcx
140825902: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
140825906: 49 89 d0                    	movq	%rdx, %r8
140825909: 49 21 c0                    	andq	%rax, %r8
14082590c: 66 41 0f 6e c5              	movd	%r13d, %xmm0
140825911: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140825915: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14082591a: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14082591f: 48 8d 79 f0                 	leaq	-0x10(%rcx), %rdi
140825923: 45 31 d2                    	xorl	%r10d, %r10d
140825926: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14082592a: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140825930: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140825934: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140825938: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
14082593d: 45 85 db                    	testl	%r11d, %r11d
140825940: 74 4e                       	je	0x140825990 <.text+0x824990>
140825942: 4c 89 b5 f0 00 00 00        	movq	%r14, 0xf0(%rbp)
140825949: 49 89 f6                    	movq	%rsi, %r14
14082594c: f3 45 0f bc e3              	tzcntl	%r11d, %r12d
140825951: 4d 01 c4                    	addq	%r8, %r12
140825954: 49 21 d4                    	andq	%rdx, %r12
140825957: 4c 89 e6                    	movq	%r12, %rsi
14082595a: 48 c1 e6 04                 	shlq	$0x4, %rsi
14082595e: 49 89 f9                    	movq	%rdi, %r9
140825961: 49 29 f1                    	subq	%rsi, %r9
140825964: 8b b5 f8 00 00 00           	movl	0xf8(%rbp), %esi
14082596a: 41 39 31                    	cmpl	%esi, (%r9)
14082596d: 74 46                       	je	0x1408259b5 <.text+0x8249b5>
14082596f: 45 8d 4b ff                 	leal	-0x1(%r11), %r9d
140825973: 66 45 21 d9                 	andw	%r11w, %r9w
140825977: 45 89 cb                    	movl	%r9d, %r11d
14082597a: 4c 89 f6                    	movq	%r14, %rsi
14082597d: 4c 8b b5 f0 00 00 00        	movq	0xf0(%rbp), %r14
140825984: 75 c3                       	jne	0x140825949 <.text+0x824949>
140825986: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140825990: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140825994: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140825999: 45 85 c9                    	testl	%r9d, %r9d
14082599c: 0f 85 e6 02 00 00           	jne	0x140825c88 <.text+0x824c88>
1408259a2: 4d 01 d0                    	addq	%r10, %r8
1408259a5: 49 83 c0 10                 	addq	$0x10, %r8
1408259a9: 49 83 c2 10                 	addq	$0x10, %r10
1408259ad: 49 21 d0                    	andq	%rdx, %r8
1408259b0: e9 75 ff ff ff              	jmp	0x14082592a <.text+0x82492a>
1408259b5: 49 f7 dc                    	negq	%r12
1408259b8: 49 c1 e4 04                 	shlq	$0x4, %r12
1408259bc: 4e 8d 3c 21                 	leaq	(%rcx,%r12), %r15
1408259c0: 4a 8d 3c 21                 	leaq	(%rcx,%r12), %rdi
1408259c4: 48 83 c7 f8                 	addq	$-0x8, %rdi
1408259c8: 4a 8b 4c 21 f8              	movq	-0x8(%rcx,%r12), %rcx
1408259cd: 48 c7 c2 ff ff ff ff        	movq	$-0x1, %rdx
1408259d4: b8 01 00 00 00              	movl	$0x1, %eax
1408259d9: f0                          	lock
1408259da: 48 0f b1 51 08              	cmpxchgq	%rdx, 0x8(%rcx)
1408259df: 0f 85 a1 01 00 00           	jne	0x140825b86 <.text+0x824b86>
1408259e5: 48 8b 01                    	movq	(%rcx), %rax
1408259e8: 48 c7 41 08 01 00 00 00     	movq	$0x1, 0x8(%rcx)
1408259f0: 48 83 f8 01                 	cmpq	$0x1, %rax
1408259f4: 0f 85 8c 01 00 00           	jne	0x140825b86 <.text+0x824b86>
1408259fa: 48 8b 07                    	movq	(%rdi), %rax
1408259fd: 48 c7 40 50 01 00 00 00     	movq	$0x1, 0x50(%rax)
140825a05: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
140825a0c: 83 78 60 00                 	cmpl	$0x0, 0x60(%rax)
140825a10: 4c 89 f6                    	movq	%r14, %rsi
140825a13: 75 21                       	jne	0x140825a36 <.text+0x824a36>
140825a15: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140825a1c: 48 8b 40 70                 	movq	0x70(%rax), %rax
140825a20: f0                          	lock
140825a21: 48 ff 08                    	decq	(%rax)
140825a24: 75 10                       	jne	0x140825a36 <.text+0x824a36>
140825a26: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140825a2d: 48 8d 48 70                 	leaq	0x70(%rax), %rcx
140825a31: e8 ea 6b 33 00              	callq	0x140b5c620 <.text+0xb5b620>
140825a36: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140825a3d: c7 40 60 02 00 00 00        	movl	$0x2, 0x60(%rax)
140825a44: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140825a48: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140825a4c: 0f 10 55 bc                 	movups	-0x44(%rbp), %xmm2
140825a50: 0f 11 40 64                 	movups	%xmm0, 0x64(%rax)
140825a54: 0f 11 48 74                 	movups	%xmm1, 0x74(%rax)
140825a58: 0f 11 90 80 00 00 00        	movups	%xmm2, 0x80(%rax)
140825a5f: 83 b8 90 00 00 00 00        	cmpl	$0x0, 0x90(%rax)
140825a66: 4c 8b b5 f0 00 00 00        	movq	0xf0(%rbp), %r14
140825a6d: 75 27                       	jne	0x140825a96 <.text+0x824a96>
140825a6f: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140825a76: 48 8b 80 a0 00 00 00        	movq	0xa0(%rax), %rax
140825a7d: f0                          	lock
140825a7e: 48 ff 08                    	decq	(%rax)
140825a81: 75 13                       	jne	0x140825a96 <.text+0x824a96>
140825a83: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140825a8a: 48 8d 88 a0 00 00 00        	leaq	0xa0(%rax), %rcx
140825a91: e8 8a 6b 33 00              	callq	0x140b5c620 <.text+0xb5b620>
140825a96: 48 8b bd f8 00 00 00        	movq	0xf8(%rbp), %rdi
140825a9d: c7 87 90 00 00 00 02 00 00 00       	movl	$0x2, 0x90(%rdi)
140825aa7: f3 0f 6f 45 a0              	movdqu	-0x60(%rbp), %xmm0
140825aac: f3 0f 6f 4d b0              	movdqu	-0x50(%rbp), %xmm1
140825ab1: f3 0f 6f 55 bc              	movdqu	-0x44(%rbp), %xmm2
140825ab6: f3 0f 7f 87 94 00 00 00     	movdqu	%xmm0, 0x94(%rdi)
140825abe: f3 0f 7f 8f a4 00 00 00     	movdqu	%xmm1, 0xa4(%rdi)
140825ac6: f3 0f 7f 97 b0 00 00 00     	movdqu	%xmm2, 0xb0(%rdi)
140825ace: 66 0f ef c0                 	pxor	%xmm0, %xmm0
140825ad2: 66 0f 7f 47 10              	movdqa	%xmm0, 0x10(%rdi)
140825ad7: 48 8b 8f 20 01 00 00        	movq	0x120(%rdi), %rcx
140825ade: 48 85 c9                    	testq	%rcx, %rcx
140825ae1: 74 10                       	je	0x140825af3 <.text+0x824af3>
140825ae3: ba 50 00 00 00              	movl	$0x50, %edx
140825ae8: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140825aee: e8 2d e1 de ff              	callq	0x140613c20 <.text+0x612c20>
140825af3: 48 c7 87 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rdi)
140825afe: 48 8b 8f 28 01 00 00        	movq	0x128(%rdi), %rcx
140825b05: 48 85 c9                    	testq	%rcx, %rcx
140825b08: 74 10                       	je	0x140825b1a <.text+0x824b1a>
140825b0a: ba 28 00 00 00              	movl	$0x28, %edx
140825b0f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140825b15: e8 06 e1 de ff              	callq	0x140613c20 <.text+0x612c20>
140825b1a: 48 c7 87 28 01 00 00 00 00 00 00    	movq	$0x0, 0x128(%rdi)
140825b25: 48 8b 8f 30 01 00 00        	movq	0x130(%rdi), %rcx
140825b2c: 48 85 c9                    	testq	%rcx, %rcx
140825b2f: 74 10                       	je	0x140825b41 <.text+0x824b41>
140825b31: ba 20 00 00 00              	movl	$0x20, %edx
140825b36: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140825b3c: e8 df e0 de ff              	callq	0x140613c20 <.text+0x612c20>
140825b41: 48 c7 87 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rdi)
140825b4c: 48 c7 87 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rdi)
140825b57: 48 c7 87 e8 00 00 00 00 00 00 00    	movq	$0x0, 0xe8(%rdi)
140825b62: 48 c7 87 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rdi)
140825b6d: 48 c7 87 18 01 00 00 00 00 00 00    	movq	$0x0, 0x118(%rdi)
140825b78: 66 c7 87 38 01 00 00 00 00  	movw	$0x0, 0x138(%rdi)
140825b81: e9 3b 02 00 00              	jmp	0x140825dc1 <.text+0x824dc1>
140825b86: 66 0f ef c0                 	pxor	%xmm0, %xmm0
140825b8a: 66 0f 7f 45 70              	movdqa	%xmm0, 0x70(%rbp)
140825b8f: f3 0f 7f 85 88 00 00 00     	movdqu	%xmm0, 0x88(%rbp)
140825b97: f3 0f 7f 85 b8 00 00 00     	movdqu	%xmm0, 0xb8(%rbp)
140825b9f: f3 0f 7f 85 c8 00 00 00     	movdqu	%xmm0, 0xc8(%rbp)
140825ba7: 66 c7 85 d8 00 00 00 00 00  	movw	$0x0, 0xd8(%rbp)
140825bb0: 48 c7 45 a0 01 00 00 00     	movq	$0x1, -0x60(%rbp)
140825bb8: 48 c7 45 a8 01 00 00 00     	movq	$0x1, -0x58(%rbp)
140825bc0: 66 0f 7f 45 b0              	movdqa	%xmm0, -0x50(%rbp)
140825bc5: 48 c7 45 f0 01 00 00 00     	movq	$0x1, -0x10(%rbp)
140825bcd: c7 45 00 02 00 00 00        	movl	$0x2, (%rbp)
140825bd4: c7 45 30 02 00 00 00        	movl	$0x2, 0x30(%rbp)
140825bdb: 48 c7 45 60 00 00 00 00     	movq	$0x0, 0x60(%rbp)
140825be3: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
140825beb: 48 c7 85 80 00 00 00 04 00 00 00    	movq	$0x4, 0x80(%rbp)
140825bf6: 48 c7 85 98 00 00 00 04 00 00 00    	movq	$0x4, 0x98(%rbp)
140825c01: 66 0f 7f 85 a0 00 00 00     	movdqa	%xmm0, 0xa0(%rbp)
140825c09: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
140825c14: 0f b6 05 ee ef e0 02        	movzbl	0x2e0efee(%rip), %eax   # 0x143634c09
140825c1b: b9 40 01 00 00              	movl	$0x140, %ecx            # imm = 0x140
140825c20: ba 10 00 00 00              	movl	$0x10, %edx
140825c25: e8 e6 df de ff              	callq	0x140613c10 <.text+0x612c10>
140825c2a: 48 85 c0                    	testq	%rax, %rax
140825c2d: 0f 84 52 02 00 00           	je	0x140825e85 <.text+0x824e85>
140825c33: 4c 89 f6                    	movq	%r14, %rsi
140825c36: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140825c3a: 41 b8 40 01 00 00           	movl	$0x140, %r8d            # imm = 0x140
140825c40: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
140825c47: 48 89 c1                    	movq	%rax, %rcx
140825c4a: e8 b1 df 0f 02              	callq	0x142923c00 <.text+0x2922c00>
140825c4f: 48 8b 07                    	movq	(%rdi), %rax
140825c52: f0                          	lock
140825c53: 48 ff 08                    	decq	(%rax)
140825c56: 4c 8b b5 f0 00 00 00        	movq	0xf0(%rbp), %r14
140825c5d: 48 89 bd f8 00 00 00        	movq	%rdi, 0xf8(%rbp)
140825c64: 75 0c                       	jne	0x140825c72 <.text+0x824c72>
140825c66: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140825c6d: e8 2e 55 00 00              	callq	0x14082b1a0 <.text+0x82a1a0>
140825c72: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
140825c79: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140825c80: 48 89 01                    	movq	%rax, (%rcx)
140825c83: e9 39 01 00 00              	jmp	0x140825dc1 <.text+0x824dc1>
140825c88: 49 89 c4                    	movq	%rax, %r12
140825c8b: 48 83 7b 10 00              	cmpq	$0x0, 0x10(%rbx)
140825c90: 0f 84 b3 01 00 00           	je	0x140825e49 <.text+0x824e49>
140825c96: 0f b6 05 6c ef e0 02        	movzbl	0x2e0ef6c(%rip), %eax   # 0x143634c09
140825c9d: b9 40 01 00 00              	movl	$0x140, %ecx            # imm = 0x140
140825ca2: ba 10 00 00 00              	movl	$0x10, %edx
140825ca7: e8 64 df de ff              	callq	0x140613c10 <.text+0x612c10>
140825cac: 48 85 c0                    	testq	%rax, %rax
140825caf: 0f 84 e1 01 00 00           	je	0x140825e96 <.text+0x824e96>
140825cb5: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140825cbc: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140825cc4: 66 0f ef c0                 	pxor	%xmm0, %xmm0
140825cc8: 66 0f 7f 40 10              	movdqa	%xmm0, 0x10(%rax)
140825ccd: 48 c7 40 50 01 00 00 00     	movq	$0x1, 0x50(%rax)
140825cd5: c7 40 60 02 00 00 00        	movl	$0x2, 0x60(%rax)
140825cdc: c7 80 90 00 00 00 02 00 00 00       	movl	$0x2, 0x90(%rax)
140825ce6: 48 c7 80 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%rax)
140825cf1: 48 c7 80 c8 00 00 00 10 00 00 00    	movq	$0x10, 0xc8(%rax)
140825cfc: 66 0f 7f 80 d0 00 00 00     	movdqa	%xmm0, 0xd0(%rax)
140825d04: 48 c7 80 e0 00 00 00 04 00 00 00    	movq	$0x4, 0xe0(%rax)
140825d0f: f3 0f 7f 80 e8 00 00 00     	movdqu	%xmm0, 0xe8(%rax)
140825d17: 48 c7 80 f8 00 00 00 04 00 00 00    	movq	$0x4, 0xf8(%rax)
140825d22: 66 0f 7f 80 00 01 00 00     	movdqa	%xmm0, 0x100(%rax)
140825d2a: 48 c7 80 10 01 00 00 10 00 00 00    	movq	$0x10, 0x110(%rax)
140825d35: f3 0f 7f 80 18 01 00 00     	movdqu	%xmm0, 0x118(%rax)
140825d3d: f3 0f 7f 80 28 01 00 00     	movdqu	%xmm0, 0x128(%rax)
140825d45: 66 c7 80 38 01 00 00 00 00  	movw	$0x0, 0x138(%rax)
140825d4e: 48 8b 0b                    	movq	(%rbx), %rcx
140825d51: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
140825d55: 49 21 d4                    	andq	%rdx, %r12
140825d58: f3 42 0f 6f 04 21           	movdqu	(%rcx,%r12), %xmm0
140825d5e: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
140825d63: 45 85 c0                    	testl	%r8d, %r8d
140825d66: 0f 84 b7 00 00 00           	je	0x140825e23 <.text+0x824e23>
140825d6c: f3 45 0f bc c0              	tzcntl	%r8d, %r8d
140825d71: 4d 01 e0                    	addq	%r12, %r8
140825d74: 49 21 d0                    	andq	%rdx, %r8
140825d77: 46 0f b6 0c 01              	movzbl	(%rcx,%r8), %r9d
140825d7c: 45 84 c9                    	testb	%r9b, %r9b
140825d7f: 0f 89 dc 00 00 00           	jns	0x140825e61 <.text+0x824e61>
140825d85: 4d 8d 50 f0                 	leaq	-0x10(%r8), %r10
140825d89: 49 21 d2                    	andq	%rdx, %r10
140825d8c: 46 88 2c 01                 	movb	%r13b, (%rcx,%r8)
140825d90: 46 88 6c 11 10              	movb	%r13b, 0x10(%rcx,%r10)
140825d95: 49 c1 e0 04                 	shlq	$0x4, %r8
140825d99: 41 80 e1 01                 	andb	$0x1, %r9b
140825d9d: 41 0f b6 d1                 	movzbl	%r9b, %edx
140825da1: 48 29 53 10                 	subq	%rdx, 0x10(%rbx)
140825da5: 49 89 cf                    	movq	%rcx, %r15
140825da8: 4d 29 c7                    	subq	%r8, %r15
140825dab: 49 f7 d8                    	negq	%r8
140825dae: 8b 95 f8 00 00 00           	movl	0xf8(%rbp), %edx
140825db4: 42 89 54 01 f0              	movl	%edx, -0x10(%rcx,%r8)
140825db9: 49 89 47 f8                 	movq	%rax, -0x8(%r15)
140825dbd: 48 ff 43 18                 	incq	0x18(%rbx)
140825dc1: 49 8b 4f f8                 	movq	-0x8(%r15), %rcx
140825dc5: 48 c7 c2 ff ff ff ff        	movq	$-0x1, %rdx
140825dcc: b8 01 00 00 00              	movl	$0x1, %eax
140825dd1: f0                          	lock
140825dd2: 48 0f b1 51 08              	cmpxchgq	%rdx, 0x8(%rcx)
140825dd7: 0f 85 9c 00 00 00           	jne	0x140825e79 <.text+0x824e79>
140825ddd: 48 8b 01                    	movq	(%rcx), %rax
140825de0: 48 c7 41 08 01 00 00 00     	movq	$0x1, 0x8(%rcx)
140825de8: 48 83 f8 01                 	cmpq	$0x1, %rax
140825dec: 0f 85 87 00 00 00           	jne	0x140825e79 <.text+0x824e79>
140825df2: 49 8b 4f f8                 	movq	-0x8(%r15), %rcx
140825df6: 48 8d 41 10                 	leaq	0x10(%rcx), %rax
140825dfa: 4c 89 71 50                 	movq	%r14, 0x50(%rcx)
140825dfe: 48 89 71 58                 	movq	%rsi, 0x58(%rcx)
140825e02: 0f b6 95 70 01 00 00        	movzbl	0x170(%rbp), %edx
140825e09: 88 91 38 01 00 00           	movb	%dl, 0x138(%rcx)
140825e0f: 48 81 c4 88 01 00 00        	addq	$0x188, %rsp            # imm = 0x188
140825e16: 5b                          	popq	%rbx
140825e17: 5f                          	popq	%rdi
140825e18: 5e                          	popq	%rsi
140825e19: 41 5c                       	popq	%r12
140825e1b: 41 5d                       	popq	%r13
140825e1d: 41 5e                       	popq	%r14
140825e1f: 41 5f                       	popq	%r15
140825e21: 5d                          	popq	%rbp
140825e22: c3                          	retq
140825e23: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140825e29: 4d 01 cc                    	addq	%r9, %r12
140825e2c: 49 21 d4                    	andq	%rdx, %r12
140825e2f: f3 42 0f 6f 04 21           	movdqu	(%rcx,%r12), %xmm0
140825e35: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
140825e3a: 49 83 c1 10                 	addq	$0x10, %r9
140825e3e: 45 85 c0                    	testl	%r8d, %r8d
140825e41: 0f 85 25 ff ff ff           	jne	0x140825d6c <.text+0x824d6c>
140825e47: eb e0                       	jmp	0x140825e29 <.text+0x824e29>
140825e49: ba 01 00 00 00              	movl	$0x1, %edx
140825e4e: 48 89 d9                    	movq	%rbx, %rcx
140825e51: 4d 89 f8                    	movq	%r15, %r8
140825e54: 41 b1 01                    	movb	$0x1, %r9b
140825e57: e8 04 d9 0c 02              	callq	0x1428f3760 <.text+0x28f2760>
140825e5c: e9 35 fe ff ff              	jmp	0x140825c96 <.text+0x824c96>
140825e61: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
140825e65: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
140825e6a: f3 45 0f bc c0              	tzcntl	%r8d, %r8d
140825e6f: 46 0f b6 0c 01              	movzbl	(%rcx,%r8), %r9d
140825e74: e9 0c ff ff ff              	jmp	0x140825d85 <.text+0x824d85>
140825e79: 48 8d 0d d0 b8 26 02        	leaq	0x226b8d0(%rip), %rcx   # 0x142a91750
140825e80: e8 8b 34 0b 02              	callq	0x1428d9310 <.text+0x28d8310>
140825e85: b9 10 00 00 00              	movl	$0x10, %ecx
140825e8a: ba 40 01 00 00              	movl	$0x140, %edx            # imm = 0x140
140825e8f: e8 6c 31 0b 02              	callq	0x1428d9000 <.text+0x28d8000>
140825e94: 0f 0b                       	ud2
140825e96: b9 10 00 00 00              	movl	$0x10, %ecx
140825e9b: ba 40 01 00 00              	movl	$0x140, %edx            # imm = 0x140
140825ea0: e8 5b 31 0b 02              	callq	0x1428d9000 <.text+0x28d8000>
140825ea5: cc                          	int3
140825ea6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140825eb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140825eb5: 55                          	pushq	%rbp
140825eb6: 41 57                       	pushq	%r15
140825eb8: 41 56                       	pushq	%r14
140825eba: 41 55                       	pushq	%r13
140825ebc: 41 54                       	pushq	%r12
140825ebe: 56                          	pushq	%rsi
140825ebf: 57                          	pushq	%rdi
140825ec0: 53                          	pushq	%rbx
140825ec1: 48 83 ec 28                 	subq	$0x28, %rsp
140825ec5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140825ecc: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140825ed0: e8 db 3e a3 00              	callq	0x141259db0 <.text+0x1258db0>
140825ed5: 90                          	nop
140825ed6: 48 83 c4 28                 	addq	$0x28, %rsp
140825eda: 5b                          	popq	%rbx
140825edb: 5f                          	popq	%rdi
140825edc: 5e                          	popq	%rsi
140825edd: 41 5c                       	popq	%r12
140825edf: 41 5d                       	popq	%r13
140825ee1: 41 5e                       	popq	%r14
140825ee3: 41 5f                       	popq	%r15
140825ee5: 5d                          	popq	%rbp
140825ee6: c3                          	retq
140825ee7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140825ef0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140825ef5: 55                          	pushq	%rbp
140825ef6: 41 57                       	pushq	%r15
140825ef8: 41 56                       	pushq	%r14
140825efa: 41 55                       	pushq	%r13
140825efc: 41 54                       	pushq	%r12
140825efe: 56                          	pushq	%rsi
140825eff: 57                          	pushq	%rdi
140825f00: 53                          	pushq	%rbx
140825f01: 48 83 ec 28                 	subq	$0x28, %rsp
140825f05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140825f0c: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140825f13: c7 40 60 02 00 00 00        	movl	$0x2, 0x60(%rax)
140825f1a: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140825f1e: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140825f22: 0f 10 55 bc                 	movups	-0x44(%rbp), %xmm2
140825f26: 0f 11 40 64                 	movups	%xmm0, 0x64(%rax)
140825f2a: 0f 11 48 74                 	movups	%xmm1, 0x74(%rax)
140825f2e: 0f 11 90 80 00 00 00        	movups	%xmm2, 0x80(%rax)
140825f35: 48 83 c4 28                 	addq	$0x28, %rsp
140825f39: 5b                          	popq	%rbx
140825f3a: 5f                          	popq	%rdi
140825f3b: 5e                          	popq	%rsi
140825f3c: 41 5c                       	popq	%r12
140825f3e: 41 5d                       	popq	%r13
140825f40: 41 5e                       	popq	%r14
140825f42: 41 5f                       	popq	%r15
140825f44: 5d                          	popq	%rbp
140825f45: c3                          	retq
140825f46: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140825f50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140825f55: 55                          	pushq	%rbp
140825f56: 41 57                       	pushq	%r15
140825f58: 41 56                       	pushq	%r14
140825f5a: 41 55                       	pushq	%r13
140825f5c: 41 54                       	pushq	%r12
140825f5e: 56                          	pushq	%rsi
140825f5f: 57                          	pushq	%rdi
140825f60: 53                          	pushq	%rbx
140825f61: 48 83 ec 28                 	subq	$0x28, %rsp
140825f65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140825f6c: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140825f73: c7 80 90 00 00 00 02 00 00 00       	movl	$0x2, 0x90(%rax)
140825f7d: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140825f81: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140825f85: 0f 10 55 bc                 	movups	-0x44(%rbp), %xmm2
140825f89: 0f 11 80 94 00 00 00        	movups	%xmm0, 0x94(%rax)
140825f90: 0f 11 88 a4 00 00 00        	movups	%xmm1, 0xa4(%rax)
140825f97: 0f 11 90 b0 00 00 00        	movups	%xmm2, 0xb0(%rax)
140825f9e: 48 83 c4 28                 	addq	$0x28, %rsp
140825fa2: 5b                          	popq	%rbx
140825fa3: 5f                          	popq	%rdi
140825fa4: 5e                          	popq	%rsi
140825fa5: 41 5c                       	popq	%r12
140825fa7: 41 5d                       	popq	%r13
140825fa9: 41 5e                       	popq	%r14
140825fab: 41 5f                       	popq	%r15
140825fad: 5d                          	popq	%rbp
140825fae: c3                          	retq
140825faf: 90                          	nop
140825fb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140825fb5: 55                          	pushq	%rbp
140825fb6: 41 57                       	pushq	%r15
140825fb8: 41 56                       	pushq	%r14
140825fba: 41 55                       	pushq	%r13
140825fbc: 41 54                       	pushq	%r12
140825fbe: 56                          	pushq	%rsi
140825fbf: 57                          	pushq	%rdi
140825fc0: 53                          	pushq	%rbx
140825fc1: 48 83 ec 28                 	subq	$0x28, %rsp
140825fc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140825fcc: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
140825fd3: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140825fda: 48 89 01                    	movq	%rax, (%rcx)
140825fdd: 48 83 c4 28                 	addq	$0x28, %rsp
140825fe1: 5b                          	popq	%rbx
140825fe2: 5f                          	popq	%rdi
140825fe3: 5e                          	popq	%rsi
140825fe4: 41 5c                       	popq	%r12
140825fe6: 41 5d                       	popq	%r13
140825fe8: 41 5e                       	popq	%r14
140825fea: 41 5f                       	popq	%r15
140825fec: 5d                          	popq	%rbp
140825fed: c3                          	retq
