
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409a8350: 55                          	pushq	%rbp
1409a8351: 41 57                       	pushq	%r15
1409a8353: 41 56                       	pushq	%r14
1409a8355: 56                          	pushq	%rsi
1409a8356: 57                          	pushq	%rdi
1409a8357: 53                          	pushq	%rbx
1409a8358: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
1409a835f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409a8367: 0f 29 b5 80 00 00 00        	movaps	%xmm6, 0x80(%rbp)
1409a836e: 48 c7 45 78 fe ff ff ff     	movq	$-0x2, 0x78(%rbp)
1409a8376: 4c 89 cb                    	movq	%r9, %rbx
1409a8379: 4d 89 c6                    	movq	%r8, %r14
1409a837c: 48 89 d7                    	movq	%rdx, %rdi
1409a837f: 48 89 ce                    	movq	%rcx, %rsi
1409a8382: 0f b6 05 80 c8 c8 02        	movzbl	0x2c8c880(%rip), %eax   # 0x143634c09
1409a8389: b9 08 00 00 00              	movl	$0x8, %ecx
1409a838e: ba 08 00 00 00              	movl	$0x8, %edx
1409a8393: e8 78 b8 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a8398: 48 85 c0                    	testq	%rax, %rax
1409a839b: 0f 84 d4 01 00 00           	je	0x1409a8575 <.text+0x9a7575>
1409a83a1: 49 89 c7                    	movq	%rax, %r15
1409a83a4: f3 0f 10 77 10              	movss	0x10(%rdi), %xmm6
1409a83a9: 0f b6 05 59 c8 c8 02        	movzbl	0x2c8c859(%rip), %eax   # 0x143634c09
1409a83b0: b9 14 00 00 00              	movl	$0x14, %ecx
1409a83b5: ba 04 00 00 00              	movl	$0x4, %edx
1409a83ba: e8 51 b8 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a83bf: 48 85 c0                    	testq	%rax, %rax
1409a83c2: 4c 89 7d 70                 	movq	%r15, 0x70(%rbp)
1409a83c6: 0f 84 b8 01 00 00           	je	0x1409a8584 <.text+0x9a7584>
1409a83cc: 0f 10 07                    	movups	(%rdi), %xmm0
1409a83cf: 0f 11 00                    	movups	%xmm0, (%rax)
1409a83d2: f3 0f 11 70 10              	movss	%xmm6, 0x10(%rax)
1409a83d7: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1409a83db: 48 8d 05 9e 54 11 02        	leaq	0x211549e(%rip), %rax   # 0x142abd880
1409a83e2: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1409a83e6: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409a83f0: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1409a83f4: 0f b6 05 0e c8 c8 02        	movzbl	0x2c8c80e(%rip), %eax   # 0x143634c09
1409a83fb: b9 13 00 00 00              	movl	$0x13, %ecx
1409a8400: ba 01 00 00 00              	movl	$0x1, %edx
1409a8405: e8 06 b8 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a840a: 48 85 c0                    	testq	%rax, %rax
1409a840d: 0f 84 82 01 00 00           	je	0x1409a8595 <.text+0x9a7595>
1409a8413: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1409a841a: 8b 15 7b 10 0f 02           	movl	0x20f107b(%rip), %edx   # 0x142a9949b
1409a8420: 89 50 0f                    	movl	%edx, 0xf(%rax)
1409a8423: 0f 10 05 62 10 0f 02        	movups	0x20f1062(%rip), %xmm0  # 0x142a9948c
1409a842a: 0f 11 00                    	movups	%xmm0, (%rax)
1409a842d: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1409a8431: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1409a8435: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1409a8439: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1409a843d: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1409a8441: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1409a8445: 48 c7 45 2c 00 00 00 00     	movq	$0x0, 0x2c(%rbp)
1409a844d: 48 c7 45 31 00 00 00 00     	movq	$0x0, 0x31(%rbp)
1409a8455: 0f 28 05 64 5c f9 01        	movaps	0x1f95c64(%rip), %xmm0  # 0x14293e0c0
1409a845c: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1409a8460: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
1409a8466: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
1409a8470: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
1409a8474: c7 45 00 00 00 00 00        	movl	$0x0, (%rbp)
1409a847b: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
1409a8482: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
1409a848a: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1409a848e: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
1409a8496: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
1409a849d: 66 c7 45 39 00 00           	movw	$0x0, 0x39(%rbp)
1409a84a3: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1409a84a8: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409a84ac: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1409a84b0: 4d 89 f0                    	movq	%r14, %r8
1409a84b3: 49 89 d9                    	movq	%rbx, %r9
1409a84b6: e8 95 58 ef ff              	callq	0x14089dd50 <.text+0x89cd50>
1409a84bb: 48 8b 5d 48                 	movq	0x48(%rbp), %rbx
1409a84bf: 48 85 db                    	testq	%rbx, %rbx
1409a84c2: 75 04                       	jne	0x1409a84c8 <.text+0x9a74c8>
1409a84c4: 48 8b 5d 50                 	movq	0x50(%rbp), %rbx
1409a84c8: f3 0f 10 77 14              	movss	0x14(%rdi), %xmm6
1409a84cd: 4c 8b 75 60                 	movq	0x60(%rbp), %r14
1409a84d1: 48 8b 03                    	movq	(%rbx), %rax
1409a84d4: 48 8b 7b 10                 	movq	0x10(%rbx), %rdi
1409a84d8: 48 29 f8                    	subq	%rdi, %rax
1409a84db: 48 83 f8 27                 	cmpq	$0x27, %rax
1409a84df: 76 6f                       	jbe	0x1409a8550 <.text+0x9a7550>
1409a84e1: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1409a84e5: 48 8d 0d 04 44 93 00        	leaq	0x934404(%rip), %rcx    # 0x1412dc8f0 <.text+0x12db8f0>
1409a84ec: 48 89 0c 38                 	movq	%rcx, (%rax,%rdi)
1409a84f0: 4c 89 74 38 08              	movq	%r14, 0x8(%rax,%rdi)
1409a84f5: c7 44 38 10 01 00 00 00     	movl	$0x1, 0x10(%rax,%rdi)
1409a84fd: f3 0f 11 74 38 14           	movss	%xmm6, 0x14(%rax,%rdi)
1409a8503: c6 44 38 18 00              	movb	$0x0, 0x18(%rax,%rdi)
1409a8508: 48 8d 0d 21 c8 bb 00        	leaq	0xbbc821(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409a850f: 48 89 4c 38 20              	movq	%rcx, 0x20(%rax,%rdi)
1409a8514: 48 83 c7 28                 	addq	$0x28, %rdi
1409a8518: 48 89 7b 10                 	movq	%rdi, 0x10(%rbx)
1409a851c: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1409a8520: 4c 89 30                    	movq	%r14, (%rax)
1409a8523: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1409a852a: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1409a852e: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
1409a8536: 48 89 f0                    	movq	%rsi, %rax
1409a8539: 0f 28 b5 80 00 00 00        	movaps	0x80(%rbp), %xmm6
1409a8540: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
1409a8547: 5b                          	popq	%rbx
1409a8548: 5f                          	popq	%rdi
1409a8549: 5e                          	popq	%rsi
1409a854a: 41 5e                       	popq	%r14
1409a854c: 41 5f                       	popq	%r15
1409a854e: 5d                          	popq	%rbp
1409a854f: c3                          	retq
1409a8550: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409a8559: 41 b8 28 00 00 00           	movl	$0x28, %r8d
1409a855f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409a8565: 48 89 d9                    	movq	%rbx, %rcx
1409a8568: 48 89 fa                    	movq	%rdi, %rdx
1409a856b: e8 f0 14 f0 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1409a8570: e9 6c ff ff ff              	jmp	0x1409a84e1 <.text+0x9a74e1>
1409a8575: b9 08 00 00 00              	movl	$0x8, %ecx
1409a857a: ba 08 00 00 00              	movl	$0x8, %edx
1409a857f: e8 7c 0a f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a8584: b9 04 00 00 00              	movl	$0x4, %ecx
1409a8589: ba 14 00 00 00              	movl	$0x14, %edx
1409a858e: e8 6d 0a f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a8593: eb 16                       	jmp	0x1409a85ab <.text+0x9a75ab>
1409a8595: 4c 8d 05 84 09 0f 02        	leaq	0x20f0984(%rip), %r8    # 0x142a98f20
1409a859c: b9 01 00 00 00              	movl	$0x1, %ecx
1409a85a1: ba 13 00 00 00              	movl	$0x13, %edx
1409a85a6: e8 38 0a f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a85ab: 0f 0b                       	ud2
1409a85ad: 0f 1f 00                    	nopl	(%rax)
1409a85b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a85b5: 55                          	pushq	%rbp
1409a85b6: 41 57                       	pushq	%r15
1409a85b8: 41 56                       	pushq	%r14
1409a85ba: 56                          	pushq	%rsi
1409a85bb: 57                          	pushq	%rdi
1409a85bc: 53                          	pushq	%rbx
1409a85bd: 48 83 ec 38                 	subq	$0x38, %rsp
1409a85c1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a85c8: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1409a85cd: ba 08 00 00 00              	movl	$0x8, %edx
1409a85d2: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409a85d8: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
1409a85dc: e8 3f b6 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a85e1: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1409a85e6: 48 83 c4 38                 	addq	$0x38, %rsp
1409a85ea: 5b                          	popq	%rbx
1409a85eb: 5f                          	popq	%rdi
1409a85ec: 5e                          	popq	%rsi
1409a85ed: 41 5e                       	popq	%r14
1409a85ef: 41 5f                       	popq	%r15
1409a85f1: 5d                          	popq	%rbp
1409a85f2: c3                          	retq
1409a85f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1409a8600: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a8605: 55                          	pushq	%rbp
1409a8606: 41 57                       	pushq	%r15
1409a8608: 41 56                       	pushq	%r14
1409a860a: 56                          	pushq	%rsi
1409a860b: 57                          	pushq	%rdi
1409a860c: 53                          	pushq	%rbx
1409a860d: 48 83 ec 38                 	subq	$0x38, %rsp
1409a8611: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a8618: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1409a861d: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409a8621: e8 7a 27 ef ff              	callq	0x14089ada0 <.text+0x899da0>
1409a8626: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1409a862b: 48 83 c4 38                 	addq	$0x38, %rsp
1409a862f: 5b                          	popq	%rbx
1409a8630: 5f                          	popq	%rdi
1409a8631: 5e                          	popq	%rsi
1409a8632: 41 5e                       	popq	%r14
1409a8634: 41 5f                       	popq	%r15
1409a8636: 5d                          	popq	%rbp
1409a8637: c3                          	retq
1409a8638: cc                          	int3
1409a8639: cc                          	int3
1409a863a: cc                          	int3
1409a863b: cc                          	int3
1409a863c: cc                          	int3
1409a863d: cc                          	int3
1409a863e: cc                          	int3
1409a863f: cc                          	int3
1409a8640: 55                          	pushq	%rbp
1409a8641: 41 57                       	pushq	%r15
1409a8643: 41 56                       	pushq	%r14
1409a8645: 56                          	pushq	%rsi
1409a8646: 57                          	pushq	%rdi
1409a8647: 53                          	pushq	%rbx
1409a8648: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
1409a864f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409a8657: 48 c7 85 90 00 00 00 fe ff ff ff    	movq	$-0x2, 0x90(%rbp)
1409a8662: 4c 89 cf                    	movq	%r9, %rdi
1409a8665: 4c 89 c3                    	movq	%r8, %rbx
1409a8668: 49 89 d6                    	movq	%rdx, %r14
1409a866b: 48 89 ce                    	movq	%rcx, %rsi
1409a866e: 0f b6 05 94 c5 c8 02        	movzbl	0x2c8c594(%rip), %eax   # 0x143634c09
1409a8675: b9 08 00 00 00              	movl	$0x8, %ecx
1409a867a: ba 08 00 00 00              	movl	$0x8, %edx
1409a867f: e8 8c b5 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a8684: 48 85 c0                    	testq	%rax, %rax
1409a8687: 0f 84 76 01 00 00           	je	0x1409a8803 <.text+0x9a7803>
1409a868d: 49 89 c7                    	movq	%rax, %r15
1409a8690: 0f b6 05 72 c5 c8 02        	movzbl	0x2c8c572(%rip), %eax   # 0x143634c09
1409a8697: b9 1e 00 00 00              	movl	$0x1e, %ecx
1409a869c: ba 01 00 00 00              	movl	$0x1, %edx
1409a86a1: e8 6a b5 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a86a6: 48 85 c0                    	testq	%rax, %rax
1409a86a9: 4c 89 bd 88 00 00 00        	movq	%r15, 0x88(%rbp)
1409a86b0: 0f 84 5c 01 00 00           	je	0x1409a8812 <.text+0x9a7812>
1409a86b6: 0f 10 05 01 52 11 02        	movups	0x2115201(%rip), %xmm0  # 0x142abd8be
1409a86bd: 0f 11 40 0e                 	movups	%xmm0, 0xe(%rax)
1409a86c1: 0f 10 05 e8 51 11 02        	movups	0x21151e8(%rip), %xmm0  # 0x142abd8b0
1409a86c8: 0f 11 00                    	movups	%xmm0, (%rax)
1409a86cb: 48 c7 45 50 1e 00 00 00     	movq	$0x1e, 0x50(%rbp)
1409a86d3: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1409a86d7: 48 c7 45 60 1e 00 00 00     	movq	$0x1e, 0x60(%rbp)
1409a86df: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1409a86e9: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1409a86ed: 0f b6 05 15 c5 c8 02        	movzbl	0x2c8c515(%rip), %eax   # 0x143634c09
1409a86f4: b9 13 00 00 00              	movl	$0x13, %ecx
1409a86f9: ba 01 00 00 00              	movl	$0x1, %edx
1409a86fe: e8 0d b5 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a8703: 48 85 c0                    	testq	%rax, %rax
1409a8706: 0f 84 1e 01 00 00           	je	0x1409a882a <.text+0x9a782a>
1409a870c: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1409a8713: 8b 15 82 0d 0f 02           	movl	0x20f0d82(%rip), %edx   # 0x142a9949b
1409a8719: 89 50 0f                    	movl	%edx, 0xf(%rax)
1409a871c: 0f 10 05 69 0d 0f 02        	movups	0x20f0d69(%rip), %xmm0  # 0x142a9948c
1409a8723: 0f 11 00                    	movups	%xmm0, (%rax)
1409a8726: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
1409a872a: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
1409a872e: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
1409a8732: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1409a8736: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1409a873a: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1409a873e: 49 8b 56 10                 	movq	0x10(%r14), %rdx
1409a8742: 48 89 55 78                 	movq	%rdx, 0x78(%rbp)
1409a8746: 41 8b 56 18                 	movl	0x18(%r14), %edx
1409a874a: 89 95 80 00 00 00           	movl	%edx, 0x80(%rbp)
1409a8750: 41 0f 28 06                 	movaps	(%r14), %xmm0
1409a8754: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1409a8758: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
1409a875e: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
1409a8768: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
1409a876c: 48 ba 01 00 00 00 00 00 40 3f       	movabsq	$0x3f40000000000001, %rdx # imm = 0x3F40000000000001
1409a8776: 48 89 55 00                 	movq	%rdx, (%rbp)
1409a877a: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
1409a8781: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
1409a8789: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1409a878d: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
1409a8795: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
1409a879c: 8b 85 80 00 00 00           	movl	0x80(%rbp), %eax
1409a87a2: 89 45 34                    	movl	%eax, 0x34(%rbp)
1409a87a5: 48 8b 45 78                 	movq	0x78(%rbp), %rax
1409a87a9: 48 89 45 2c                 	movq	%rax, 0x2c(%rbp)
1409a87ad: 66 c7 45 38 00 00           	movw	$0x0, 0x38(%rbp)
1409a87b3: c6 45 3a 00                 	movb	$0x0, 0x3a(%rbp)
1409a87b7: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1409a87bc: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1409a87c0: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1409a87c4: 49 89 d8                    	movq	%rbx, %r8
1409a87c7: 49 89 f9                    	movq	%rdi, %r9
1409a87ca: e8 81 55 ef ff              	callq	0x14089dd50 <.text+0x89cd50>
1409a87cf: 48 8b 45 68                 	movq	0x68(%rbp), %rax
1409a87d3: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
1409a87da: 48 89 01                    	movq	%rax, (%rcx)
1409a87dd: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1409a87e4: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
1409a87e8: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
1409a87f0: 48 89 f0                    	movq	%rsi, %rax
1409a87f3: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
1409a87fa: 5b                          	popq	%rbx
1409a87fb: 5f                          	popq	%rdi
1409a87fc: 5e                          	popq	%rsi
1409a87fd: 41 5e                       	popq	%r14
1409a87ff: 41 5f                       	popq	%r15
1409a8801: 5d                          	popq	%rbp
1409a8802: c3                          	retq
1409a8803: b9 08 00 00 00              	movl	$0x8, %ecx
1409a8808: ba 08 00 00 00              	movl	$0x8, %edx
1409a880d: e8 ee 07 f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a8812: 4c 8d 05 17 44 11 02        	leaq	0x2114417(%rip), %r8    # 0x142abcc30
1409a8819: b9 01 00 00 00              	movl	$0x1, %ecx
1409a881e: ba 1e 00 00 00              	movl	$0x1e, %edx
1409a8823: e8 bb 07 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a8828: eb 16                       	jmp	0x1409a8840 <.text+0x9a7840>
1409a882a: 4c 8d 05 ef 06 0f 02        	leaq	0x20f06ef(%rip), %r8    # 0x142a98f20
1409a8831: b9 01 00 00 00              	movl	$0x1, %ecx
1409a8836: ba 13 00 00 00              	movl	$0x13, %edx
1409a883b: e8 a3 07 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a8840: 0f 0b                       	ud2
1409a8842: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409a8850: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a8855: 55                          	pushq	%rbp
1409a8856: 41 57                       	pushq	%r15
1409a8858: 41 56                       	pushq	%r14
1409a885a: 56                          	pushq	%rsi
1409a885b: 57                          	pushq	%rdi
1409a885c: 53                          	pushq	%rbx
1409a885d: 48 83 ec 28                 	subq	$0x28, %rsp
1409a8861: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a8868: ba 08 00 00 00              	movl	$0x8, %edx
1409a886d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409a8873: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
1409a887a: e8 a1 b3 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a887f: 90                          	nop
1409a8880: 48 83 c4 28                 	addq	$0x28, %rsp
1409a8884: 5b                          	popq	%rbx
1409a8885: 5f                          	popq	%rdi
1409a8886: 5e                          	popq	%rsi
1409a8887: 41 5e                       	popq	%r14
1409a8889: 41 5f                       	popq	%r15
1409a888b: 5d                          	popq	%rbp
1409a888c: c3                          	retq
1409a888d: 0f 1f 00                    	nopl	(%rax)
1409a8890: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a8895: 55                          	pushq	%rbp
1409a8896: 41 57                       	pushq	%r15
1409a8898: 41 56                       	pushq	%r14
1409a889a: 56                          	pushq	%rsi
1409a889b: 57                          	pushq	%rdi
1409a889c: 53                          	pushq	%rbx
1409a889d: 48 83 ec 28                 	subq	$0x28, %rsp
1409a88a1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a88a8: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1409a88ac: e8 ef 24 ef ff              	callq	0x14089ada0 <.text+0x899da0>
1409a88b1: 90                          	nop
1409a88b2: 48 83 c4 28                 	addq	$0x28, %rsp
1409a88b6: 5b                          	popq	%rbx
1409a88b7: 5f                          	popq	%rdi
1409a88b8: 5e                          	popq	%rsi
1409a88b9: 41 5e                       	popq	%r14
1409a88bb: 41 5f                       	popq	%r15
1409a88bd: 5d                          	popq	%rbp
1409a88be: c3                          	retq
1409a88bf: cc                          	int3
1409a88c0: 55                          	pushq	%rbp
1409a88c1: 41 57                       	pushq	%r15
1409a88c3: 41 56                       	pushq	%r14
1409a88c5: 56                          	pushq	%rsi
1409a88c6: 57                          	pushq	%rdi
1409a88c7: 53                          	pushq	%rbx
1409a88c8: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1409a88cf: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409a88d7: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
1409a88e2: 4c 89 cf                    	movq	%r9, %rdi
1409a88e5: 4c 89 c3                    	movq	%r8, %rbx
1409a88e8: 49 89 d6                    	movq	%rdx, %r14
1409a88eb: 48 89 ce                    	movq	%rcx, %rsi
1409a88ee: 0f b6 05 14 c3 c8 02        	movzbl	0x2c8c314(%rip), %eax   # 0x143634c09
1409a88f5: b9 08 00 00 00              	movl	$0x8, %ecx
1409a88fa: ba 08 00 00 00              	movl	$0x8, %edx
1409a88ff: e8 0c b3 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a8904: 48 85 c0                    	testq	%rax, %rax
1409a8907: 0f 84 76 01 00 00           	je	0x1409a8a83 <.text+0x9a7a83>
1409a890d: 49 89 c7                    	movq	%rax, %r15
1409a8910: 0f b6 05 f2 c2 c8 02        	movzbl	0x2c8c2f2(%rip), %eax   # 0x143634c09
1409a8917: b9 1a 00 00 00              	movl	$0x1a, %ecx
1409a891c: ba 01 00 00 00              	movl	$0x1, %edx
1409a8921: e8 ea b2 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a8926: 48 85 c0                    	testq	%rax, %rax
1409a8929: 4c 89 7d 78                 	movq	%r15, 0x78(%rbp)
1409a892d: 0f 84 5f 01 00 00           	je	0x1409a8a92 <.text+0x9a7a92>
1409a8933: 0f 10 05 9e 4f 11 02        	movups	0x2114f9e(%rip), %xmm0  # 0x142abd8d8
1409a893a: 0f 11 40 0a                 	movups	%xmm0, 0xa(%rax)
1409a893e: 0f 10 05 89 4f 11 02        	movups	0x2114f89(%rip), %xmm0  # 0x142abd8ce
1409a8945: 0f 11 00                    	movups	%xmm0, (%rax)
1409a8948: 48 c7 45 50 1a 00 00 00     	movq	$0x1a, 0x50(%rbp)
1409a8950: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1409a8954: 48 c7 45 60 1a 00 00 00     	movq	$0x1a, 0x60(%rbp)
1409a895c: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1409a8966: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1409a896a: 0f b6 05 98 c2 c8 02        	movzbl	0x2c8c298(%rip), %eax   # 0x143634c09
1409a8971: b9 13 00 00 00              	movl	$0x13, %ecx
1409a8976: ba 01 00 00 00              	movl	$0x1, %edx
1409a897b: e8 90 b2 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a8980: 48 85 c0                    	testq	%rax, %rax
1409a8983: 0f 84 21 01 00 00           	je	0x1409a8aaa <.text+0x9a7aaa>
1409a8989: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1409a8990: 8b 15 05 0b 0f 02           	movl	0x20f0b05(%rip), %edx   # 0x142a9949b
1409a8996: 89 50 0f                    	movl	%edx, 0xf(%rax)
1409a8999: 0f 10 05 ec 0a 0f 02        	movups	0x20f0aec(%rip), %xmm0  # 0x142a9948c
1409a89a0: 0f 11 00                    	movups	%xmm0, (%rax)
1409a89a3: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
1409a89a7: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
1409a89ab: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
1409a89af: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1409a89b3: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1409a89b7: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1409a89bb: 41 0f 28 06                 	movaps	(%r14), %xmm0
1409a89bf: f3 41 0f 10 4e 10           	movss	0x10(%r14), %xmm1
1409a89c5: f3 0f 59 0d 03 cf f7 01     	mulss	0x1f7cf03(%rip), %xmm1  # 0x1429258d0
1409a89cd: 41 8b 56 1c                 	movl	0x1c(%r14), %edx
1409a89d1: 89 55 50                    	movl	%edx, 0x50(%rbp)
1409a89d4: 49 8b 56 14                 	movq	0x14(%r14), %rdx
1409a89d8: 48 89 55 48                 	movq	%rdx, 0x48(%rbp)
1409a89dc: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1409a89e0: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
1409a89e6: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
1409a89f0: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
1409a89f4: c7 45 00 01 00 00 00        	movl	$0x1, (%rbp)
1409a89fb: f3 0f 11 4d 04              	movss	%xmm1, 0x4(%rbp)
1409a8a00: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
1409a8a07: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
1409a8a0f: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1409a8a13: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
1409a8a1b: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
1409a8a22: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1409a8a26: 48 89 45 2c                 	movq	%rax, 0x2c(%rbp)
1409a8a2a: 8b 45 50                    	movl	0x50(%rbp), %eax
1409a8a2d: 89 45 34                    	movl	%eax, 0x34(%rbp)
1409a8a30: 66 c7 45 38 00 00           	movw	$0x0, 0x38(%rbp)
1409a8a36: c6 45 3a 00                 	movb	$0x0, 0x3a(%rbp)
1409a8a3a: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1409a8a3f: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1409a8a43: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1409a8a47: 49 89 d8                    	movq	%rbx, %r8
1409a8a4a: 49 89 f9                    	movq	%rdi, %r9
1409a8a4d: e8 fe 52 ef ff              	callq	0x14089dd50 <.text+0x89cd50>
1409a8a52: 48 8b 45 68                 	movq	0x68(%rbp), %rax
1409a8a56: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
1409a8a5a: 48 89 01                    	movq	%rax, (%rcx)
1409a8a5d: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1409a8a64: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
1409a8a68: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
1409a8a70: 48 89 f0                    	movq	%rsi, %rax
1409a8a73: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1409a8a7a: 5b                          	popq	%rbx
1409a8a7b: 5f                          	popq	%rdi
1409a8a7c: 5e                          	popq	%rsi
1409a8a7d: 41 5e                       	popq	%r14
1409a8a7f: 41 5f                       	popq	%r15
1409a8a81: 5d                          	popq	%rbp
1409a8a82: c3                          	retq
1409a8a83: b9 08 00 00 00              	movl	$0x8, %ecx
1409a8a88: ba 08 00 00 00              	movl	$0x8, %edx
1409a8a8d: e8 6e 05 f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a8a92: 4c 8d 05 97 41 11 02        	leaq	0x2114197(%rip), %r8    # 0x142abcc30
1409a8a99: b9 01 00 00 00              	movl	$0x1, %ecx
1409a8a9e: ba 1a 00 00 00              	movl	$0x1a, %edx
1409a8aa3: e8 3b 05 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a8aa8: eb 16                       	jmp	0x1409a8ac0 <.text+0x9a7ac0>
1409a8aaa: 4c 8d 05 6f 04 0f 02        	leaq	0x20f046f(%rip), %r8    # 0x142a98f20
1409a8ab1: b9 01 00 00 00              	movl	$0x1, %ecx
1409a8ab6: ba 13 00 00 00              	movl	$0x13, %edx
1409a8abb: e8 23 05 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a8ac0: 0f 0b                       	ud2
1409a8ac2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409a8ad0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a8ad5: 55                          	pushq	%rbp
1409a8ad6: 41 57                       	pushq	%r15
1409a8ad8: 41 56                       	pushq	%r14
1409a8ada: 56                          	pushq	%rsi
1409a8adb: 57                          	pushq	%rdi
1409a8adc: 53                          	pushq	%rbx
1409a8add: 48 83 ec 28                 	subq	$0x28, %rsp
1409a8ae1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a8ae8: ba 08 00 00 00              	movl	$0x8, %edx
1409a8aed: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409a8af3: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
1409a8af7: e8 24 b1 c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a8afc: 90                          	nop
1409a8afd: 48 83 c4 28                 	addq	$0x28, %rsp
1409a8b01: 5b                          	popq	%rbx
1409a8b02: 5f                          	popq	%rdi
1409a8b03: 5e                          	popq	%rsi
1409a8b04: 41 5e                       	popq	%r14
1409a8b06: 41 5f                       	popq	%r15
1409a8b08: 5d                          	popq	%rbp
1409a8b09: c3                          	retq
1409a8b0a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1409a8b10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a8b15: 55                          	pushq	%rbp
1409a8b16: 41 57                       	pushq	%r15
1409a8b18: 41 56                       	pushq	%r14
1409a8b1a: 56                          	pushq	%rsi
1409a8b1b: 57                          	pushq	%rdi
1409a8b1c: 53                          	pushq	%rbx
1409a8b1d: 48 83 ec 28                 	subq	$0x28, %rsp
1409a8b21: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a8b28: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1409a8b2c: e8 6f 22 ef ff              	callq	0x14089ada0 <.text+0x899da0>
1409a8b31: 90                          	nop
1409a8b32: 48 83 c4 28                 	addq	$0x28, %rsp
1409a8b36: 5b                          	popq	%rbx
1409a8b37: 5f                          	popq	%rdi
1409a8b38: 5e                          	popq	%rsi
1409a8b39: 41 5e                       	popq	%r14
1409a8b3b: 41 5f                       	popq	%r15
1409a8b3d: 5d                          	popq	%rbp
1409a8b3e: c3                          	retq
1409a8b3f: cc                          	int3
1409a8b40: 55                          	pushq	%rbp
1409a8b41: 41 57                       	pushq	%r15
1409a8b43: 41 56                       	pushq	%r14
1409a8b45: 41 55                       	pushq	%r13
1409a8b47: 41 54                       	pushq	%r12
1409a8b49: 56                          	pushq	%rsi
1409a8b4a: 57                          	pushq	%rdi
1409a8b4b: 53                          	pushq	%rbx
1409a8b4c: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
1409a8b53: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409a8b5b: 48 c7 85 90 00 00 00 fe ff ff ff    	movq	$-0x2, 0x90(%rbp)
1409a8b66: 4c 89 cf                    	movq	%r9, %rdi
1409a8b69: 4c 89 c3                    	movq	%r8, %rbx
1409a8b6c: 49 89 d6                    	movq	%rdx, %r14
1409a8b6f: 48 89 ce                    	movq	%rcx, %rsi
1409a8b72: 0f b6 05 90 c0 c8 02        	movzbl	0x2c8c090(%rip), %eax   # 0x143634c09
1409a8b79: b9 08 00 00 00              	movl	$0x8, %ecx
1409a8b7e: ba 08 00 00 00              	movl	$0x8, %edx
1409a8b83: e8 88 b0 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a8b88: 48 85 c0                    	testq	%rax, %rax
1409a8b8b: 0f 84 9d 01 00 00           	je	0x1409a8d2e <.text+0x9a7d2e>
1409a8b91: 4d 8b 7e 08                 	movq	0x8(%r14), %r15
1409a8b95: 4d 85 ff                    	testq	%r15, %r15
1409a8b98: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
1409a8b9f: 0f 88 98 01 00 00           	js	0x1409a8d3d <.text+0x9a7d3d>
1409a8ba5: 4d 8b 26                    	movq	(%r14), %r12
1409a8ba8: 74 2e                       	je	0x1409a8bd8 <.text+0x9a7bd8>
1409a8baa: 0f b6 05 58 c0 c8 02        	movzbl	0x2c8c058(%rip), %eax   # 0x143634c09
1409a8bb1: b8 01 00 00 00              	movl	$0x1, %eax
1409a8bb6: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
1409a8bbd: ba 01 00 00 00              	movl	$0x1, %edx
1409a8bc2: 4c 89 f9                    	movq	%r15, %rcx
1409a8bc5: e8 46 b0 c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a8bca: 48 85 c0                    	testq	%rax, %rax
1409a8bcd: 0f 84 75 01 00 00           	je	0x1409a8d48 <.text+0x9a7d48>
1409a8bd3: 49 89 c5                    	movq	%rax, %r13
1409a8bd6: eb 06                       	jmp	0x1409a8bde <.text+0x9a7bde>
1409a8bd8: 41 bd 01 00 00 00           	movl	$0x1, %r13d
1409a8bde: 4c 89 e9                    	movq	%r13, %rcx
1409a8be1: 4c 89 e2                    	movq	%r12, %rdx
1409a8be4: 4d 89 f8                    	movq	%r15, %r8
1409a8be7: e8 14 b0 f7 01              	callq	0x142923c00 <.text+0x2922c00>
1409a8bec: 4c 89 7d 48                 	movq	%r15, 0x48(%rbp)
1409a8bf0: 4c 89 6d 50                 	movq	%r13, 0x50(%rbp)
1409a8bf4: 4c 89 7d 58                 	movq	%r15, 0x58(%rbp)
1409a8bf8: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1409a8c02: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1409a8c06: 0f b6 05 fc bf c8 02        	movzbl	0x2c8bffc(%rip), %eax   # 0x143634c09
1409a8c0d: b9 13 00 00 00              	movl	$0x13, %ecx
1409a8c12: ba 01 00 00 00              	movl	$0x1, %edx
1409a8c17: e8 f4 af c6 ff              	callq	0x140613c10 <.text+0x612c10>
1409a8c1c: 48 85 c0                    	testq	%rax, %rax
1409a8c1f: 0f 84 3b 01 00 00           	je	0x1409a8d60 <.text+0x9a7d60>
1409a8c25: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
1409a8c2c: 8b 15 69 08 0f 02           	movl	0x20f0869(%rip), %edx   # 0x142a9949b
1409a8c32: 89 50 0f                    	movl	%edx, 0xf(%rax)
1409a8c35: 0f 10 05 50 08 0f 02        	movups	0x20f0850(%rip), %xmm0  # 0x142a9948c
1409a8c3c: 0f 11 00                    	movups	%xmm0, (%rax)
1409a8c3f: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1409a8c43: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1409a8c47: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1409a8c4b: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1409a8c4f: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1409a8c53: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1409a8c57: f3 41 0f 10 46 20           	movss	0x20(%r14), %xmm0
1409a8c5d: f3 0f 59 05 6b cc f7 01     	mulss	0x1f7cc6b(%rip), %xmm0  # 0x1429258d0
1409a8c65: 41 8b 56 2c                 	movl	0x2c(%r14), %edx
1409a8c69: 89 55 78                    	movl	%edx, 0x78(%rbp)
1409a8c6c: 49 8b 56 24                 	movq	0x24(%r14), %rdx
1409a8c70: 48 89 55 70                 	movq	%rdx, 0x70(%rbp)
1409a8c74: 41 0f 28 4e 10              	movaps	0x10(%r14), %xmm1
1409a8c79: 41 0f b6 56 30              	movzbl	0x30(%r14), %edx
1409a8c7e: 80 f2 01                    	xorb	$0x1, %dl
1409a8c81: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
1409a8c85: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
1409a8c8b: 49 b8 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %r8 # imm = 0x13F800000
1409a8c95: 4c 89 45 f4                 	movq	%r8, -0xc(%rbp)
1409a8c99: c7 45 00 01 00 00 00        	movl	$0x1, (%rbp)
1409a8ca0: f3 0f 11 45 04              	movss	%xmm0, 0x4(%rbp)
1409a8ca5: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
1409a8cac: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
1409a8cb4: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1409a8cb8: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
1409a8cc0: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
1409a8cc7: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1409a8ccb: 48 89 45 2c                 	movq	%rax, 0x2c(%rbp)
1409a8ccf: 8b 45 78                    	movl	0x78(%rbp), %eax
1409a8cd2: 89 45 34                    	movl	%eax, 0x34(%rbp)
1409a8cd5: 66 c7 45 38 00 00           	movw	$0x0, 0x38(%rbp)
1409a8cdb: 88 55 3a                    	movb	%dl, 0x3a(%rbp)
1409a8cde: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1409a8ce3: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409a8ce7: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1409a8ceb: 49 89 d8                    	movq	%rbx, %r8
1409a8cee: 49 89 f9                    	movq	%rdi, %r9
1409a8cf1: e8 5a 50 ef ff              	callq	0x14089dd50 <.text+0x89cd50>
1409a8cf6: 48 8b 45 60                 	movq	0x60(%rbp), %rax
1409a8cfa: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1409a8d01: 48 89 01                    	movq	%rax, (%rcx)
1409a8d04: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1409a8d0b: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
1409a8d0f: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
1409a8d17: 48 89 f0                    	movq	%rsi, %rax
1409a8d1a: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
1409a8d21: 5b                          	popq	%rbx
1409a8d22: 5f                          	popq	%rdi
1409a8d23: 5e                          	popq	%rsi
1409a8d24: 41 5c                       	popq	%r12
1409a8d26: 41 5d                       	popq	%r13
1409a8d28: 41 5e                       	popq	%r14
1409a8d2a: 41 5f                       	popq	%r15
1409a8d2c: 5d                          	popq	%rbp
1409a8d2d: c3                          	retq
1409a8d2e: b9 08 00 00 00              	movl	$0x8, %ecx
1409a8d33: ba 08 00 00 00              	movl	$0x8, %edx
1409a8d38: e8 c3 02 f3 01              	callq	0x1428d9000 <.text+0x28d8000>
1409a8d3d: 48 c7 85 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbp)
1409a8d48: 4c 8d 05 e1 3e 11 02        	leaq	0x2113ee1(%rip), %r8    # 0x142abcc30
1409a8d4f: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
1409a8d56: 4c 89 fa                    	movq	%r15, %rdx
1409a8d59: e8 85 02 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a8d5e: eb 16                       	jmp	0x1409a8d76 <.text+0x9a7d76>
1409a8d60: 4c 8d 05 b9 01 0f 02        	leaq	0x20f01b9(%rip), %r8    # 0x142a98f20
1409a8d67: b9 01 00 00 00              	movl	$0x1, %ecx
1409a8d6c: ba 13 00 00 00              	movl	$0x13, %edx
1409a8d71: e8 6d 02 f3 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409a8d76: 0f 0b                       	ud2
1409a8d78: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1409a8d80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a8d85: 55                          	pushq	%rbp
1409a8d86: 41 57                       	pushq	%r15
1409a8d88: 41 56                       	pushq	%r14
1409a8d8a: 41 55                       	pushq	%r13
1409a8d8c: 41 54                       	pushq	%r12
1409a8d8e: 56                          	pushq	%rsi
1409a8d8f: 57                          	pushq	%rdi
1409a8d90: 53                          	pushq	%rbx
1409a8d91: 48 83 ec 28                 	subq	$0x28, %rsp
1409a8d95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a8d9c: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409a8da0: e8 fb 1f ef ff              	callq	0x14089ada0 <.text+0x899da0>
1409a8da5: 90                          	nop
1409a8da6: 48 83 c4 28                 	addq	$0x28, %rsp
1409a8daa: 5b                          	popq	%rbx
1409a8dab: 5f                          	popq	%rdi
1409a8dac: 5e                          	popq	%rsi
1409a8dad: 41 5c                       	popq	%r12
1409a8daf: 41 5d                       	popq	%r13
1409a8db1: 41 5e                       	popq	%r14
1409a8db3: 41 5f                       	popq	%r15
1409a8db5: 5d                          	popq	%rbp
1409a8db6: c3                          	retq
1409a8db7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409a8dc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409a8dc5: 55                          	pushq	%rbp
1409a8dc6: 41 57                       	pushq	%r15
1409a8dc8: 41 56                       	pushq	%r14
1409a8dca: 41 55                       	pushq	%r13
1409a8dcc: 41 54                       	pushq	%r12
1409a8dce: 56                          	pushq	%rsi
1409a8dcf: 57                          	pushq	%rdi
1409a8dd0: 53                          	pushq	%rbx
1409a8dd1: 48 83 ec 28                 	subq	$0x28, %rsp
1409a8dd5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409a8ddc: ba 08 00 00 00              	movl	$0x8, %edx
1409a8de1: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409a8de7: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1409a8dee: e8 2d ae c6 ff              	callq	0x140613c20 <.text+0x612c20>
1409a8df3: 90                          	nop
1409a8df4: 48 83 c4 28                 	addq	$0x28, %rsp
1409a8df8: 5b                          	popq	%rbx
1409a8df9: 5f                          	popq	%rdi
1409a8dfa: 5e                          	popq	%rsi
1409a8dfb: 41 5c                       	popq	%r12
1409a8dfd: 41 5d                       	popq	%r13
1409a8dff: 41 5e                       	popq	%r14
1409a8e01: 41 5f                       	popq	%r15
1409a8e03: 5d                          	popq	%rbp
1409a8e04: c3                          	retq
