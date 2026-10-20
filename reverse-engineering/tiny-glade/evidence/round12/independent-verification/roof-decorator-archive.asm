
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1421a0470: 55                          	pushq	%rbp
1421a0471: 41 57                       	pushq	%r15
1421a0473: 41 56                       	pushq	%r14
1421a0475: 41 55                       	pushq	%r13
1421a0477: 41 54                       	pushq	%r12
1421a0479: 56                          	pushq	%rsi
1421a047a: 57                          	pushq	%rdi
1421a047b: 53                          	pushq	%rbx
1421a047c: 48 81 ec 18 03 00 00        	subq	$0x318, %rsp            # imm = 0x318
1421a0483: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1421a048b: 44 0f 29 8d 80 02 00 00     	movaps	%xmm9, 0x280(%rbp)
1421a0493: 44 0f 29 85 70 02 00 00     	movaps	%xmm8, 0x270(%rbp)
1421a049b: 0f 29 bd 60 02 00 00        	movaps	%xmm7, 0x260(%rbp)
1421a04a2: 66 0f 7f b5 50 02 00 00     	movdqa	%xmm6, 0x250(%rbp)
1421a04aa: 48 c7 85 48 02 00 00 fe ff ff ff    	movq	$-0x2, 0x248(%rbp)
1421a04b5: 4d 89 c6                    	movq	%r8, %r14
1421a04b8: 49 89 d7                    	movq	%rdx, %r15
1421a04bb: 49 89 cc                    	movq	%rcx, %r12
1421a04be: 48 8b b5 00 03 00 00        	movq	0x300(%rbp), %rsi
1421a04c5: 4c 89 c9                    	movq	%r9, %rcx
1421a04c8: e8 f3 bd 87 fe              	callq	0x140a1c2c0 <.text+0xa1b2c0>
1421a04cd: 48 85 c0                    	testq	%rax, %rax
1421a04d0: 0f 84 4c 01 00 00           	je	0x1421a0622 <.text+0x219f622>
1421a04d6: 49 89 c5                    	movq	%rax, %r13
1421a04d9: 48 8b 06                    	movq	(%rsi), %rax
1421a04dc: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1421a04e3: 48 8b 76 20                 	movq	0x20(%rsi), %rsi
1421a04e7: 4d 85 ff                    	testq	%r15, %r15
1421a04ea: 0f 84 79 01 00 00           	je	0x1421a0669 <.text+0x219f669>
1421a04f0: 49 83 ff 01                 	cmpq	$0x1, %r15
1421a04f4: 4c 89 a5 30 02 00 00        	movq	%r12, 0x230(%rbp)
1421a04fb: 0f 85 1f 02 00 00           	jne	0x1421a0720 <.text+0x219f720>
1421a0501: 49 83 bc 24 c0 00 00 00 00  	cmpq	$0x0, 0xc0(%r12)
1421a050a: 0f 85 c9 0e 00 00           	jne	0x1421a13d9 <.text+0x21a03d9>
1421a0510: 31 c9                       	xorl	%ecx, %ecx
1421a0512: e8 09 96 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a0517: 48 85 c0                    	testq	%rax, %rax
1421a051a: 0f 84 ae 17 00 00           	je	0x1421a1cce <.text+0x21a0cce>
1421a0520: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a0524: 48 ff 00                    	incq	(%rax)
1421a0527: 0f 10 0d d2 1d f6 00        	movups	0xf61dd2(%rip), %xmm1   # 0x143102300
1421a052e: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
1421a0532: f3 0f 6f 0d b6 1d f6 00     	movdqu	0xf61db6(%rip), %xmm1   # 0x1431022f0
1421a053a: 66 0f 7f 4d e0              	movdqa	%xmm1, -0x20(%rbp)
1421a053f: 66 0f 7f 45 00              	movdqa	%xmm0, (%rbp)
1421a0544: 31 c9                       	xorl	%ecx, %ecx
1421a0546: e8 d5 95 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a054b: 48 8b bd 30 02 00 00        	movq	0x230(%rbp), %rdi
1421a0552: 48 85 c0                    	testq	%rax, %rax
1421a0555: 0f 84 60 19 00 00           	je	0x1421a1ebb <.text+0x21a0ebb>
1421a055b: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a055f: 48 ff 00                    	incq	(%rax)
1421a0562: 66 0f 7f 85 b0 00 00 00     	movdqa	%xmm0, 0xb0(%rbp)
1421a056a: f3 0f 6f 35 4e 1d f6 00     	movdqu	0xf61d4e(%rip), %xmm6   # 0x1431022c0
1421a0572: 66 0f 7f b5 90 00 00 00     	movdqa	%xmm6, 0x90(%rbp)
1421a057a: 0f 10 3d 4f 1d f6 00        	movups	0xf61d4f(%rip), %xmm7   # 0x1431022d0
1421a0581: 0f 29 bd a0 00 00 00        	movaps	%xmm7, 0xa0(%rbp)
1421a0588: 31 c9                       	xorl	%ecx, %ecx
1421a058a: e8 91 95 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a058f: 48 85 c0                    	testq	%rax, %rax
1421a0592: 0f 84 45 19 00 00           	je	0x1421a1edd <.text+0x21a0edd>
1421a0598: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a059c: 48 ff 00                    	incq	(%rax)
1421a059f: 0f 28 4d e0                 	movaps	-0x20(%rbp), %xmm1
1421a05a3: 0f 28 55 f0                 	movaps	-0x10(%rbp), %xmm2
1421a05a7: 0f 28 5d 00                 	movaps	(%rbp), %xmm3
1421a05ab: 0f 29 9d e0 01 00 00        	movaps	%xmm3, 0x1e0(%rbp)
1421a05b2: 0f 29 95 d0 01 00 00        	movaps	%xmm2, 0x1d0(%rbp)
1421a05b9: 0f 29 8d c0 01 00 00        	movaps	%xmm1, 0x1c0(%rbp)
1421a05c0: 66 0f 6f 8d 90 00 00 00     	movdqa	0x90(%rbp), %xmm1
1421a05c8: 66 0f 6f 95 a0 00 00 00     	movdqa	0xa0(%rbp), %xmm2
1421a05d0: 66 0f 6f 9d b0 00 00 00     	movdqa	0xb0(%rbp), %xmm3
1421a05d8: 66 0f 7f 8d 90 01 00 00     	movdqa	%xmm1, 0x190(%rbp)
1421a05e0: 66 0f 7f 95 a0 01 00 00     	movdqa	%xmm2, 0x1a0(%rbp)
1421a05e8: 66 0f 7f 9d b0 01 00 00     	movdqa	%xmm3, 0x1b0(%rbp)
1421a05f0: 66 0f 7f b5 f0 01 00 00     	movdqa	%xmm6, 0x1f0(%rbp)
1421a05f8: 0f 29 bd 00 02 00 00        	movaps	%xmm7, 0x200(%rbp)
1421a05ff: 66 0f 7f 85 10 02 00 00     	movdqa	%xmm0, 0x210(%rbp)
1421a0607: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a060e: 4c 89 ea                    	movq	%r13, %rdx
1421a0611: e8 da da 3e 00              	callq	0x14258e0f0 <.text+0x258d0f0>
1421a0616: 4c 8d b7 c0 00 00 00        	leaq	0xc0(%rdi), %r14
1421a061d: e9 1a 02 00 00              	jmp	0x1421a083c <.text+0x219f83c>
1421a0622: 48 8b bd 10 03 00 00        	movq	0x310(%rbp), %rdi
1421a0629: 48 85 ff                    	testq	%rdi, %rdi
1421a062c: 0f 88 41 18 00 00           	js	0x1421a1e73 <.text+0x21a0e73>
1421a0632: 0f 84 43 02 00 00           	je	0x1421a087b <.text+0x219f87b>
1421a0638: 4d 89 e5                    	movq	%r12, %r13
1421a063b: 0f b6 05 c7 45 49 01        	movzbl	0x14945c7(%rip), %eax   # 0x143634c09
1421a0642: 41 bc 01 00 00 00           	movl	$0x1, %r12d
1421a0648: ba 01 00 00 00              	movl	$0x1, %edx
1421a064d: 48 89 f9                    	movq	%rdi, %rcx
1421a0650: e8 bb 35 47 fe              	callq	0x140613c10 <.text+0x612c10>
1421a0655: 48 85 c0                    	testq	%rax, %rax
1421a0658: 0f 84 18 18 00 00           	je	0x1421a1e76 <.text+0x21a0e76>
1421a065e: 48 89 c3                    	movq	%rax, %rbx
1421a0661: 4d 89 ec                    	movq	%r13, %r12
1421a0664: e9 17 02 00 00              	jmp	0x1421a0880 <.text+0x219f880>
1421a0669: 4c 89 b5 90 01 00 00        	movq	%r14, 0x190(%rbp)
1421a0670: 4d 8d 7c 24 20              	leaq	0x20(%r12), %r15
1421a0675: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1421a067c: 4c 89 f9                    	movq	%r15, %rcx
1421a067f: e8 ec 06 41 fe              	callq	0x1405b0d70 <.text+0x5afd70>
1421a0684: 48 89 c7                    	movq	%rax, %rdi
1421a0687: 48 c1 ef 39                 	shrq	$0x39, %rdi
1421a068b: 49 8b 0c 24                 	movq	(%r12), %rcx
1421a068f: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
1421a0694: 49 89 d0                    	movq	%rdx, %r8
1421a0697: 49 21 c0                    	andq	%rax, %r8
1421a069a: 66 0f 6e c7                 	movd	%edi, %xmm0
1421a069e: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1421a06a2: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1421a06a7: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1421a06ac: 4c 8d 89 68 ff ff ff        	leaq	-0x98(%rcx), %r9
1421a06b3: 45 31 d2                    	xorl	%r10d, %r10d
1421a06b6: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1421a06ba: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1421a06c0: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1421a06c4: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1421a06c8: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
1421a06cd: 45 85 db                    	testl	%r11d, %r11d
1421a06d0: 74 2c                       	je	0x1421a06fe <.text+0x219f6fe>
1421a06d2: f3 41 0f bc db              	tzcntl	%r11d, %ebx
1421a06d7: 4c 01 c3                    	addq	%r8, %rbx
1421a06da: 48 21 d3                    	andq	%rdx, %rbx
1421a06dd: 48 f7 db                    	negq	%rbx
1421a06e0: 48 69 db 98 00 00 00        	imulq	$0x98, %rbx, %rbx
1421a06e7: 4d 39 34 19                 	cmpq	%r14, (%r9,%rbx)
1421a06eb: 0f 84 e8 0c 00 00           	je	0x1421a13d9 <.text+0x21a03d9>
1421a06f1: 41 8d 5b ff                 	leal	-0x1(%r11), %ebx
1421a06f5: 66 44 21 db                 	andw	%r11w, %bx
1421a06f9: 41 89 db                    	movl	%ebx, %r11d
1421a06fc: 75 d4                       	jne	0x1421a06d2 <.text+0x219f6d2>
1421a06fe: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1421a0702: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1421a0707: 45 85 db                    	testl	%r11d, %r11d
1421a070a: 0f 85 f0 05 00 00           	jne	0x1421a0d00 <.text+0x219fd00>
1421a0710: 4d 01 d0                    	addq	%r10, %r8
1421a0713: 49 83 c0 10                 	addq	$0x10, %r8
1421a0717: 49 83 c2 10                 	addq	$0x10, %r10
1421a071b: 49 21 d0                    	andq	%rdx, %r8
1421a071e: eb 9a                       	jmp	0x1421a06ba <.text+0x219f6ba>
1421a0720: 49 83 bc 24 50 01 00 00 00  	cmpq	$0x0, 0x150(%r12)
1421a0729: 0f 85 aa 0c 00 00           	jne	0x1421a13d9 <.text+0x21a03d9>
1421a072f: 31 c9                       	xorl	%ecx, %ecx
1421a0731: e8 ea 93 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a0736: 48 85 c0                    	testq	%rax, %rax
1421a0739: 0f 84 8f 15 00 00           	je	0x1421a1cce <.text+0x21a0cce>
1421a073f: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a0743: 48 ff 00                    	incq	(%rax)
1421a0746: 0f 10 0d b3 1b f6 00        	movups	0xf61bb3(%rip), %xmm1   # 0x143102300
1421a074d: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
1421a0751: f3 0f 6f 0d 97 1b f6 00     	movdqu	0xf61b97(%rip), %xmm1   # 0x1431022f0
1421a0759: 66 0f 7f 4d e0              	movdqa	%xmm1, -0x20(%rbp)
1421a075e: 66 0f 7f 45 00              	movdqa	%xmm0, (%rbp)
1421a0763: 31 c9                       	xorl	%ecx, %ecx
1421a0765: e8 b6 93 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a076a: 48 8b bd 30 02 00 00        	movq	0x230(%rbp), %rdi
1421a0771: 48 85 c0                    	testq	%rax, %rax
1421a0774: 0f 84 52 17 00 00           	je	0x1421a1ecc <.text+0x21a0ecc>
1421a077a: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a077e: 48 ff 00                    	incq	(%rax)
1421a0781: 66 0f 7f 85 b0 00 00 00     	movdqa	%xmm0, 0xb0(%rbp)
1421a0789: f3 0f 6f 35 2f 1b f6 00     	movdqu	0xf61b2f(%rip), %xmm6   # 0x1431022c0
1421a0791: 66 0f 7f b5 90 00 00 00     	movdqa	%xmm6, 0x90(%rbp)
1421a0799: 0f 10 3d 30 1b f6 00        	movups	0xf61b30(%rip), %xmm7   # 0x1431022d0
1421a07a0: 0f 29 bd a0 00 00 00        	movaps	%xmm7, 0xa0(%rbp)
1421a07a7: 31 c9                       	xorl	%ecx, %ecx
1421a07a9: e8 72 93 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a07ae: 48 85 c0                    	testq	%rax, %rax
1421a07b1: 0f 84 37 17 00 00           	je	0x1421a1eee <.text+0x21a0eee>
1421a07b7: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a07bb: 48 ff 00                    	incq	(%rax)
1421a07be: 0f 28 4d e0                 	movaps	-0x20(%rbp), %xmm1
1421a07c2: 0f 28 55 f0                 	movaps	-0x10(%rbp), %xmm2
1421a07c6: 0f 28 5d 00                 	movaps	(%rbp), %xmm3
1421a07ca: 0f 29 9d e0 01 00 00        	movaps	%xmm3, 0x1e0(%rbp)
1421a07d1: 0f 29 95 d0 01 00 00        	movaps	%xmm2, 0x1d0(%rbp)
1421a07d8: 0f 29 8d c0 01 00 00        	movaps	%xmm1, 0x1c0(%rbp)
1421a07df: 66 0f 6f 8d 90 00 00 00     	movdqa	0x90(%rbp), %xmm1
1421a07e7: 66 0f 6f 95 a0 00 00 00     	movdqa	0xa0(%rbp), %xmm2
1421a07ef: 66 0f 6f 9d b0 00 00 00     	movdqa	0xb0(%rbp), %xmm3
1421a07f7: 66 0f 7f 8d 90 01 00 00     	movdqa	%xmm1, 0x190(%rbp)
1421a07ff: 66 0f 7f 95 a0 01 00 00     	movdqa	%xmm2, 0x1a0(%rbp)
1421a0807: 66 0f 7f 9d b0 01 00 00     	movdqa	%xmm3, 0x1b0(%rbp)
1421a080f: 66 0f 7f b5 f0 01 00 00     	movdqa	%xmm6, 0x1f0(%rbp)
1421a0817: 0f 29 bd 00 02 00 00        	movaps	%xmm7, 0x200(%rbp)
1421a081e: 66 0f 7f 85 10 02 00 00     	movdqa	%xmm0, 0x210(%rbp)
1421a0826: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a082d: 4c 89 ea                    	movq	%r13, %rdx
1421a0830: e8 bb d8 3e 00              	callq	0x14258e0f0 <.text+0x258d0f0>
1421a0835: 4c 8d b7 50 01 00 00        	leaq	0x150(%rdi), %r14
1421a083c: 4c 8d bd 90 00 00 00        	leaq	0x90(%rbp), %r15
1421a0843: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1421a084a: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1421a0850: 4c 89 f9                    	movq	%r15, %rcx
1421a0853: e8 a8 33 78 00              	callq	0x142923c00 <.text+0x2922c00>
1421a0858: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1421a085e: 4c 89 f1                    	movq	%r14, %rcx
1421a0861: 4c 89 fa                    	movq	%r15, %rdx
1421a0864: e8 97 33 78 00              	callq	0x142923c00 <.text+0x2922c00>
1421a0869: 49 8b 7d 48                 	movq	0x48(%r13), %rdi
1421a086d: 48 85 ff                    	testq	%rdi, %rdi
1421a0870: 0f 85 42 06 00 00           	jne	0x1421a0eb8 <.text+0x219feb8>
1421a0876: e9 5e 0b 00 00              	jmp	0x1421a13d9 <.text+0x21a03d9>
1421a087b: bb 01 00 00 00              	movl	$0x1, %ebx
1421a0880: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
1421a0887: 48 89 d9                    	movq	%rbx, %rcx
1421a088a: 49 89 f8                    	movq	%rdi, %r8
1421a088d: e8 6e 33 78 00              	callq	0x142923c00 <.text+0x2922c00>
1421a0892: 48 89 7d 70                 	movq	%rdi, 0x70(%rbp)
1421a0896: 48 89 5d 78                 	movq	%rbx, 0x78(%rbp)
1421a089a: 48 89 bd 80 00 00 00        	movq	%rdi, 0x80(%rbp)
1421a08a1: 31 c9                       	xorl	%ecx, %ecx
1421a08a3: e8 78 92 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a08a8: 48 85 c0                    	testq	%rax, %rax
1421a08ab: 0f 84 d7 15 00 00           	je	0x1421a1e88 <.text+0x21a0e88>
1421a08b1: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a08b5: 48 ff 00                    	incq	(%rax)
1421a08b8: 44 0f 10 0d 40 1a f6 00     	movups	0xf61a40(%rip), %xmm9   # 0x143102300
1421a08c0: 44 0f 29 8d a0 00 00 00     	movaps	%xmm9, 0xa0(%rbp)
1421a08c8: 44 0f 10 05 20 1a f6 00     	movups	0xf61a20(%rip), %xmm8   # 0x1431022f0
1421a08d0: 44 0f 29 85 90 00 00 00     	movaps	%xmm8, 0x90(%rbp)
1421a08d8: 66 0f 7f 85 b0 00 00 00     	movdqa	%xmm0, 0xb0(%rbp)
1421a08e0: 31 c9                       	xorl	%ecx, %ecx
1421a08e2: e8 39 92 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a08e7: 48 85 c0                    	testq	%rax, %rax
1421a08ea: 0f 84 a9 15 00 00           	je	0x1421a1e99 <.text+0x21a0e99>
1421a08f0: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a08f4: 48 ff 00                    	incq	(%rax)
1421a08f7: 66 0f 7f 85 b0 01 00 00     	movdqa	%xmm0, 0x1b0(%rbp)
1421a08ff: 0f 10 3d ba 19 f6 00        	movups	0xf619ba(%rip), %xmm7   # 0x1431022c0
1421a0906: 0f 29 bd 90 01 00 00        	movaps	%xmm7, 0x190(%rbp)
1421a090d: f3 0f 6f 35 bb 19 f6 00     	movdqu	0xf619bb(%rip), %xmm6   # 0x1431022d0
1421a0915: 66 0f 7f b5 a0 01 00 00     	movdqa	%xmm6, 0x1a0(%rbp)
1421a091d: 31 c9                       	xorl	%ecx, %ecx
1421a091f: e8 fc 91 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a0924: 48 85 c0                    	testq	%rax, %rax
1421a0927: 0f 84 7d 15 00 00           	je	0x1421a1eaa <.text+0x21a0eaa>
1421a092d: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a0931: 48 ff 00                    	incq	(%rax)
1421a0934: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
1421a093b: 0f 28 95 a0 00 00 00        	movaps	0xa0(%rbp), %xmm2
1421a0942: 0f 28 9d b0 00 00 00        	movaps	0xb0(%rbp), %xmm3
1421a0949: 0f 29 5d 30                 	movaps	%xmm3, 0x30(%rbp)
1421a094d: 0f 29 55 20                 	movaps	%xmm2, 0x20(%rbp)
1421a0951: 0f 29 4d 10                 	movaps	%xmm1, 0x10(%rbp)
1421a0955: 66 0f 6f 8d 90 01 00 00     	movdqa	0x190(%rbp), %xmm1
1421a095d: 66 0f 6f 95 a0 01 00 00     	movdqa	0x1a0(%rbp), %xmm2
1421a0965: 66 0f 6f 9d b0 01 00 00     	movdqa	0x1b0(%rbp), %xmm3
1421a096d: 66 0f 7f 4d e0              	movdqa	%xmm1, -0x20(%rbp)
1421a0972: 66 0f 7f 55 f0              	movdqa	%xmm2, -0x10(%rbp)
1421a0977: 66 0f 7f 5d 00              	movdqa	%xmm3, (%rbp)
1421a097c: 0f 29 7d 40                 	movaps	%xmm7, 0x40(%rbp)
1421a0980: 66 0f 7f 75 50              	movdqa	%xmm6, 0x50(%rbp)
1421a0985: 66 0f 7f 45 60              	movdqa	%xmm0, 0x60(%rbp)
1421a098a: 48 8d 45 70                 	leaq	0x70(%rbp), %rax
1421a098e: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
1421a0995: 48 8d 05 34 b1 e9 fd        	leaq	-0x2164ecc(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
1421a099c: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
1421a09a3: 48 8d 05 b6 5c e5 00        	leaq	0xe55cb6(%rip), %rax    # 0x142ff6660
1421a09aa: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1421a09b1: 48 c7 85 98 01 00 00 02 00 00 00    	movq	$0x2, 0x198(%rbp)
1421a09bc: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
1421a09c3: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1421a09ca: 48 c7 85 a8 01 00 00 01 00 00 00    	movq	$0x1, 0x1a8(%rbp)
1421a09d5: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
1421a09e0: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
1421a09e4: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1421a09eb: e8 a0 95 bd fe              	callq	0x140d79f90 <.text+0xd78f90>
1421a09f0: 48 8b 55 a8                 	movq	-0x58(%rbp), %rdx
1421a09f4: 48 8b 4d b0                 	movq	-0x50(%rbp), %rcx
1421a09f8: 48 8b 06                    	movq	(%rsi), %rax
1421a09fb: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1421a0a02: 48 8b 76 20                 	movq	0x20(%rsi), %rsi
1421a0a06: 49 83 ff 02                 	cmpq	$0x2, %r15
1421a0a0a: 0f 84 29 01 00 00           	je	0x1421a0b39 <.text+0x219fb39>
1421a0a10: 49 83 ff 01                 	cmpq	$0x1, %r15
1421a0a14: 0f 85 3c 01 00 00           	jne	0x1421a0b56 <.text+0x219fb56>
1421a0a1a: 49 83 bc 24 c0 00 00 00 00  	cmpq	$0x0, 0xc0(%r12)
1421a0a23: 0f 85 1f 01 00 00           	jne	0x1421a0b48 <.text+0x219fb48>
1421a0a29: 48 89 8d 28 02 00 00        	movq	%rcx, 0x228(%rbp)
1421a0a30: 48 89 95 38 02 00 00        	movq	%rdx, 0x238(%rbp)
1421a0a37: 31 c9                       	xorl	%ecx, %ecx
1421a0a39: e8 e2 90 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a0a3e: 48 85 c0                    	testq	%rax, %rax
1421a0a41: 0f 84 da 14 00 00           	je	0x1421a1f21 <.text+0x21a0f21>
1421a0a47: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a0a4b: 48 ff 00                    	incq	(%rax)
1421a0a4e: 44 0f 29 8d 30 01 00 00     	movaps	%xmm9, 0x130(%rbp)
1421a0a56: 44 0f 29 85 20 01 00 00     	movaps	%xmm8, 0x120(%rbp)
1421a0a5e: 66 0f 7f 85 40 01 00 00     	movdqa	%xmm0, 0x140(%rbp)
1421a0a66: 31 c9                       	xorl	%ecx, %ecx
1421a0a68: e8 b3 90 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a0a6d: 48 85 c0                    	testq	%rax, %rax
1421a0a70: 0f 84 c7 14 00 00           	je	0x1421a1f3d <.text+0x21a0f3d>
1421a0a76: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a0a7a: 48 ff 00                    	incq	(%rax)
1421a0a7d: 66 0f 7f 85 b0 00 00 00     	movdqa	%xmm0, 0xb0(%rbp)
1421a0a85: 0f 29 bd 90 00 00 00        	movaps	%xmm7, 0x90(%rbp)
1421a0a8c: 66 0f 7f b5 a0 00 00 00     	movdqa	%xmm6, 0xa0(%rbp)
1421a0a94: 31 c9                       	xorl	%ecx, %ecx
1421a0a96: e8 85 90 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a0a9b: 48 85 c0                    	testq	%rax, %rax
1421a0a9e: 0f 84 b5 14 00 00           	je	0x1421a1f59 <.text+0x21a0f59>
1421a0aa4: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a0aa8: 48 ff 00                    	incq	(%rax)
1421a0aab: 0f 28 8d 20 01 00 00        	movaps	0x120(%rbp), %xmm1
1421a0ab2: 0f 28 95 30 01 00 00        	movaps	0x130(%rbp), %xmm2
1421a0ab9: 0f 28 9d 40 01 00 00        	movaps	0x140(%rbp), %xmm3
1421a0ac0: 0f 29 9d e0 01 00 00        	movaps	%xmm3, 0x1e0(%rbp)
1421a0ac7: 0f 29 95 d0 01 00 00        	movaps	%xmm2, 0x1d0(%rbp)
1421a0ace: 0f 29 8d c0 01 00 00        	movaps	%xmm1, 0x1c0(%rbp)
1421a0ad5: 66 0f 6f 8d 90 00 00 00     	movdqa	0x90(%rbp), %xmm1
1421a0add: 66 0f 6f 95 a0 00 00 00     	movdqa	0xa0(%rbp), %xmm2
1421a0ae5: 66 0f 6f 9d b0 00 00 00     	movdqa	0xb0(%rbp), %xmm3
1421a0aed: 66 0f 7f 8d 90 01 00 00     	movdqa	%xmm1, 0x190(%rbp)
1421a0af5: 66 0f 7f 95 a0 01 00 00     	movdqa	%xmm2, 0x1a0(%rbp)
1421a0afd: 66 0f 7f 9d b0 01 00 00     	movdqa	%xmm3, 0x1b0(%rbp)
1421a0b05: 0f 29 bd f0 01 00 00        	movaps	%xmm7, 0x1f0(%rbp)
1421a0b0c: 66 0f 7f b5 00 02 00 00     	movdqa	%xmm6, 0x200(%rbp)
1421a0b14: 66 0f 7f 85 10 02 00 00     	movdqa	%xmm0, 0x210(%rbp)
1421a0b1c: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a0b23: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
1421a0b27: e8 c4 d5 3e 00              	callq	0x14258e0f0 <.text+0x258d0f0>
1421a0b2c: 49 8d 9c 24 c0 00 00 00     	leaq	0xc0(%r12), %rbx
1421a0b34: e9 dd 09 00 00              	jmp	0x1421a1516 <.text+0x21a0516>
1421a0b39: 49 83 bc 24 50 01 00 00 00  	cmpq	$0x0, 0x150(%r12)
1421a0b42: 0f 84 c3 08 00 00           	je	0x1421a140b <.text+0x21a040b>
1421a0b48: 48 85 d2                    	testq	%rdx, %rdx
1421a0b4b: 0f 85 f0 00 00 00           	jne	0x1421a0c41 <.text+0x219fc41>
1421a0b51: e9 f6 00 00 00              	jmp	0x1421a0c4c <.text+0x219fc4c>
1421a0b56: 48 89 8d 28 02 00 00        	movq	%rcx, 0x228(%rbp)
1421a0b5d: 48 89 95 38 02 00 00        	movq	%rdx, 0x238(%rbp)
1421a0b64: 4c 89 b5 90 01 00 00        	movq	%r14, 0x190(%rbp)
1421a0b6b: 4d 89 e5                    	movq	%r12, %r13
1421a0b6e: 4d 8d 7c 24 20              	leaq	0x20(%r12), %r15
1421a0b73: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1421a0b7a: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1421a0b81: 4c 89 f9                    	movq	%r15, %rcx
1421a0b84: e8 e7 01 41 fe              	callq	0x1405b0d70 <.text+0x5afd70>
1421a0b89: 48 89 c3                    	movq	%rax, %rbx
1421a0b8c: 48 89 c7                    	movq	%rax, %rdi
1421a0b8f: 48 c1 ef 39                 	shrq	$0x39, %rdi
1421a0b93: 4d 89 ec                    	movq	%r13, %r12
1421a0b96: 49 8b 45 00                 	movq	(%r13), %rax
1421a0b9a: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
1421a0b9e: 48 89 ca                    	movq	%rcx, %rdx
1421a0ba1: 48 21 da                    	andq	%rbx, %rdx
1421a0ba4: 66 0f 6e c7                 	movd	%edi, %xmm0
1421a0ba8: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1421a0bac: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1421a0bb1: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1421a0bb6: 4c 8d 80 68 ff ff ff        	leaq	-0x98(%rax), %r8
1421a0bbd: 45 31 c9                    	xorl	%r9d, %r9d
1421a0bc0: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1421a0bc4: f3 0f 6f 14 10              	movdqu	(%rax,%rdx), %xmm2
1421a0bc9: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1421a0bcd: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1421a0bd1: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1421a0bd6: 45 85 d2                    	testl	%r10d, %r10d
1421a0bd9: 74 28                       	je	0x1421a0c03 <.text+0x219fc03>
1421a0bdb: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1421a0be0: 49 01 d3                    	addq	%rdx, %r11
1421a0be3: 49 21 cb                    	andq	%rcx, %r11
1421a0be6: 49 f7 db                    	negq	%r11
1421a0be9: 4d 69 db 98 00 00 00        	imulq	$0x98, %r11, %r11
1421a0bf0: 4f 39 34 18                 	cmpq	%r14, (%r8,%r11)
1421a0bf4: 74 2f                       	je	0x1421a0c25 <.text+0x219fc25>
1421a0bf6: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1421a0bfa: 66 45 21 d3                 	andw	%r10w, %r11w
1421a0bfe: 45 89 da                    	movl	%r11d, %r10d
1421a0c01: 75 d8                       	jne	0x1421a0bdb <.text+0x219fbdb>
1421a0c03: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1421a0c07: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1421a0c0c: 45 85 d2                    	testl	%r10d, %r10d
1421a0c0f: 0f 85 55 09 00 00           	jne	0x1421a156a <.text+0x21a056a>
1421a0c15: 4c 01 ca                    	addq	%r9, %rdx
1421a0c18: 48 83 c2 10                 	addq	$0x10, %rdx
1421a0c1c: 49 83 c1 10                 	addq	$0x10, %r9
1421a0c20: 48 21 ca                    	andq	%rcx, %rdx
1421a0c23: eb 9f                       	jmp	0x1421a0bc4 <.text+0x219fbc4>
1421a0c25: 41 b5 01                    	movb	$0x1, %r13b
1421a0c28: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1421a0c2f: 48 85 d2                    	testq	%rdx, %rdx
1421a0c32: 0f 95 c0                    	setne	%al
1421a0c35: 41 84 c5                    	testb	%al, %r13b
1421a0c38: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1421a0c3f: 74 0b                       	je	0x1421a0c4c <.text+0x219fc4c>
1421a0c41: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421a0c47: e8 d4 2f 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a0c4c: 4c 8b 4d 18                 	movq	0x18(%rbp), %r9
1421a0c50: 4d 85 c9                    	testq	%r9, %r9
1421a0c53: 74 2a                       	je	0x1421a0c7f <.text+0x219fc7f>
1421a0c55: 49 8d 41 01                 	leaq	0x1(%r9), %rax
1421a0c59: b9 50 00 00 00              	movl	$0x50, %ecx
1421a0c5e: 48 f7 e1                    	mulq	%rcx
1421a0c61: 49 01 c1                    	addq	%rax, %r9
1421a0c64: 49 83 c1 11                 	addq	$0x11, %r9
1421a0c68: 74 15                       	je	0x1421a0c7f <.text+0x219fc7f>
1421a0c6a: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
1421a0c6e: 48 29 c1                    	subq	%rax, %rcx
1421a0c71: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1421a0c77: 4c 89 ca                    	movq	%r9, %rdx
1421a0c7a: e8 a1 2f 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a0c7f: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
1421a0c83: 48 85 d2                    	testq	%rdx, %rdx
1421a0c86: 74 27                       	je	0x1421a0caf <.text+0x219fcaf>
1421a0c88: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
1421a0c90: 48 83 e0 f0                 	andq	$-0x10, %rax
1421a0c94: 48 01 c2                    	addq	%rax, %rdx
1421a0c97: 48 83 c2 11                 	addq	$0x11, %rdx
1421a0c9b: 74 12                       	je	0x1421a0caf <.text+0x219fcaf>
1421a0c9d: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
1421a0ca1: 48 29 c1                    	subq	%rax, %rcx
1421a0ca4: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1421a0caa: e8 71 2f 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a0caf: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
1421a0cb3: 48 85 d2                    	testq	%rdx, %rdx
1421a0cb6: 74 27                       	je	0x1421a0cdf <.text+0x219fcdf>
1421a0cb8: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
1421a0cc0: 48 83 e0 f0                 	andq	$-0x10, %rax
1421a0cc4: 48 01 c2                    	addq	%rax, %rdx
1421a0cc7: 48 83 c2 11                 	addq	$0x11, %rdx
1421a0ccb: 74 12                       	je	0x1421a0cdf <.text+0x219fcdf>
1421a0ccd: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
1421a0cd1: 48 29 c1                    	subq	%rax, %rcx
1421a0cd4: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1421a0cda: e8 41 2f 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a0cdf: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
1421a0ce3: 48 85 d2                    	testq	%rdx, %rdx
1421a0ce6: 0f 84 ed 06 00 00           	je	0x1421a13d9 <.text+0x21a03d9>
1421a0cec: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
1421a0cf0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421a0cf6: e8 25 2f 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a0cfb: e9 d9 06 00 00              	jmp	0x1421a13d9 <.text+0x21a03d9>
1421a0d00: 48 89 c3                    	movq	%rax, %rbx
1421a0d03: 49 83 7c 24 10 00           	cmpq	$0x0, 0x10(%r12)
1421a0d09: 0f 84 95 0f 00 00           	je	0x1421a1ca4 <.text+0x21a0ca4>
1421a0d0f: 31 c9                       	xorl	%ecx, %ecx
1421a0d11: e8 0a 8e f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a0d16: 48 85 c0                    	testq	%rax, %rax
1421a0d19: 0f 84 af 0f 00 00           	je	0x1421a1cce <.text+0x21a0cce>
1421a0d1f: 0f 10 00                    	movups	(%rax), %xmm0
1421a0d22: 48 ff 00                    	incq	(%rax)
1421a0d25: 0f 10 0d d4 15 f6 00        	movups	0xf615d4(%rip), %xmm1   # 0x143102300
1421a0d2c: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
1421a0d30: 0f 10 0d b9 15 f6 00        	movups	0xf615b9(%rip), %xmm1   # 0x1431022f0
1421a0d37: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
1421a0d3b: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
1421a0d3f: 31 c9                       	xorl	%ecx, %ecx
1421a0d41: e8 da 8d f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a0d46: 48 85 c0                    	testq	%rax, %rax
1421a0d49: 0f 84 b0 11 00 00           	je	0x1421a1eff <.text+0x21a0eff>
1421a0d4f: 0f 10 00                    	movups	(%rax), %xmm0
1421a0d52: 48 ff 00                    	incq	(%rax)
1421a0d55: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
1421a0d5c: f3 0f 6f 35 5c 15 f6 00     	movdqu	0xf6155c(%rip), %xmm6   # 0x1431022c0
1421a0d64: 66 0f 7f b5 90 01 00 00     	movdqa	%xmm6, 0x190(%rbp)
1421a0d6c: 0f 10 3d 5d 15 f6 00        	movups	0xf6155d(%rip), %xmm7   # 0x1431022d0
1421a0d73: 0f 29 bd a0 01 00 00        	movaps	%xmm7, 0x1a0(%rbp)
1421a0d7a: 31 c9                       	xorl	%ecx, %ecx
1421a0d7c: e8 9f 8d f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a0d81: 48 85 c0                    	testq	%rax, %rax
1421a0d84: 0f 84 86 11 00 00           	je	0x1421a1f10 <.text+0x21a0f10>
1421a0d8a: 0f 10 00                    	movups	(%rax), %xmm0
1421a0d8d: 48 ff 00                    	incq	(%rax)
1421a0d90: 0f 28 4d e0                 	movaps	-0x20(%rbp), %xmm1
1421a0d94: 0f 28 55 f0                 	movaps	-0x10(%rbp), %xmm2
1421a0d98: 0f 28 5d 00                 	movaps	(%rbp), %xmm3
1421a0d9c: 0f 29 9d e0 00 00 00        	movaps	%xmm3, 0xe0(%rbp)
1421a0da3: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
1421a0daa: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
1421a0db1: 66 0f 6f 8d 90 01 00 00     	movdqa	0x190(%rbp), %xmm1
1421a0db9: 66 0f 6f 95 a0 01 00 00     	movdqa	0x1a0(%rbp), %xmm2
1421a0dc1: 66 0f 6f 9d b0 01 00 00     	movdqa	0x1b0(%rbp), %xmm3
1421a0dc9: 66 0f 7f 8d 90 00 00 00     	movdqa	%xmm1, 0x90(%rbp)
1421a0dd1: 66 0f 7f 95 a0 00 00 00     	movdqa	%xmm2, 0xa0(%rbp)
1421a0dd9: 66 0f 7f 9d b0 00 00 00     	movdqa	%xmm3, 0xb0(%rbp)
1421a0de1: 66 0f 7f b5 f0 00 00 00     	movdqa	%xmm6, 0xf0(%rbp)
1421a0de9: 0f 29 bd 00 01 00 00        	movaps	%xmm7, 0x100(%rbp)
1421a0df0: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1421a0df7: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a0dfe: 4c 89 ea                    	movq	%r13, %rdx
1421a0e01: e8 ea d2 3e 00              	callq	0x14258e0f0 <.text+0x258d0f0>
1421a0e06: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a0e0d: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
1421a0e14: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1421a0e1a: e8 e1 2d 78 00              	callq	0x142923c00 <.text+0x2922c00>
1421a0e1f: 49 8b 04 24                 	movq	(%r12), %rax
1421a0e23: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
1421a0e28: 48 21 cb                    	andq	%rcx, %rbx
1421a0e2b: f3 0f 6f 04 18              	movdqu	(%rax,%rbx), %xmm0
1421a0e30: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1421a0e34: 85 d2                       	testl	%edx, %edx
1421a0e36: 0f 84 45 0e 00 00           	je	0x1421a1c81 <.text+0x21a0c81>
1421a0e3c: f3 0f bc d2                 	tzcntl	%edx, %edx
1421a0e40: 48 01 da                    	addq	%rbx, %rdx
1421a0e43: 48 21 ca                    	andq	%rcx, %rdx
1421a0e46: 44 0f b6 04 10              	movzbl	(%rax,%rdx), %r8d
1421a0e4b: 45 84 c0                    	testb	%r8b, %r8b
1421a0e4e: 0f 89 86 0e 00 00           	jns	0x1421a1cda <.text+0x21a0cda>
1421a0e54: 4c 8d 4a f0                 	leaq	-0x10(%rdx), %r9
1421a0e58: 49 21 c9                    	andq	%rcx, %r9
1421a0e5b: 40 88 3c 10                 	movb	%dil, (%rax,%rdx)
1421a0e5f: 42 88 7c 08 10              	movb	%dil, 0x10(%rax,%r9)
1421a0e64: 41 80 e0 01                 	andb	$0x1, %r8b
1421a0e68: 41 0f b6 c8                 	movzbl	%r8b, %ecx
1421a0e6c: 49 29 4c 24 10              	subq	%rcx, 0x10(%r12)
1421a0e71: 48 f7 da                    	negq	%rdx
1421a0e74: 48 69 ca 98 00 00 00        	imulq	$0x98, %rdx, %rcx
1421a0e7b: 4c 89 b4 08 68 ff ff ff     	movq	%r14, -0x98(%rax,%rcx)
1421a0e83: 48 01 c1                    	addq	%rax, %rcx
1421a0e86: 48 81 c1 70 ff ff ff        	addq	$-0x90, %rcx
1421a0e8d: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1421a0e94: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1421a0e9a: e8 61 2d 78 00              	callq	0x142923c00 <.text+0x2922c00>
1421a0e9f: 4c 89 a5 30 02 00 00        	movq	%r12, 0x230(%rbp)
1421a0ea6: 49 ff 44 24 18              	incq	0x18(%r12)
1421a0eab: 49 8b 7d 48                 	movq	0x48(%r13), %rdi
1421a0eaf: 48 85 ff                    	testq	%rdi, %rdi
1421a0eb2: 0f 84 21 05 00 00           	je	0x1421a13d9 <.text+0x21a03d9>
1421a0eb8: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1421a0ebf: 48 83 c0 60                 	addq	$0x60, %rax
1421a0ec3: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1421a0eca: 4c 89 ad 88 00 00 00        	movq	%r13, 0x88(%rbp)
1421a0ed1: 49 8b 5d 30                 	movq	0x30(%r13), %rbx
1421a0ed5: 4c 8d 63 10                 	leaq	0x10(%rbx), %r12
1421a0ed9: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
1421a0edd: 66 44 0f d7 f0              	pmovmskb	%xmm0, %r14d
1421a0ee2: 41 f7 d6                    	notl	%r14d
1421a0ee5: 4c 8d 7e 20                 	leaq	0x20(%rsi), %r15
1421a0ee9: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1421a0eed: e9 9e 01 00 00              	jmp	0x1421a1090 <.text+0x21a0090>
1421a0ef2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1421a0f00: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1421a0f06: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1421a0f0a: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1421a0f0e: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1421a0f13: 45 85 db                    	testl	%r11d, %r11d
1421a0f16: 74 28                       	je	0x1421a0f40 <.text+0x219ff40>
1421a0f18: f3 45 0f bc eb              	tzcntl	%r11d, %r13d
1421a0f1d: 4d 01 c5                    	addq	%r8, %r13
1421a0f20: 49 21 d5                    	andq	%rdx, %r13
1421a0f23: 49 f7 dd                    	negq	%r13
1421a0f26: 4f 8d 6c 6d 00              	leaq	(%r13,%r13,2), %r13
1421a0f2b: 46 3b 0c e8                 	cmpl	(%rax,%r13,8), %r9d
1421a0f2f: 74 3f                       	je	0x1421a0f70 <.text+0x219ff70>
1421a0f31: 45 8d 6b ff                 	leal	-0x1(%r11), %r13d
1421a0f35: 66 45 21 dd                 	andw	%r11w, %r13w
1421a0f39: 45 89 eb                    	movl	%r13d, %r11d
1421a0f3c: 75 da                       	jne	0x1421a0f18 <.text+0x219ff18>
1421a0f3e: 66 90                       	nop
1421a0f40: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1421a0f44: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1421a0f49: 45 85 db                    	testl	%r11d, %r11d
1421a0f4c: 0f 85 ee 01 00 00           	jne	0x1421a1140 <.text+0x21a0140>
1421a0f52: 4d 01 d0                    	addq	%r10, %r8
1421a0f55: 49 83 c0 10                 	addq	$0x10, %r8
1421a0f59: 49 83 c2 10                 	addq	$0x10, %r10
1421a0f5d: 49 21 d0                    	andq	%rdx, %r8
1421a0f60: eb 9e                       	jmp	0x1421a0f00 <.text+0x219ff00>
1421a0f62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1421a0f70: 42 0f b6 44 e9 fc           	movzbl	-0x4(%rcx,%r13,8), %eax
1421a0f76: 3c 11                       	cmpb	$0x11, %al
1421a0f78: 0f 84 c2 01 00 00           	je	0x1421a1140 <.text+0x21a0140>
1421a0f7e: 4a 8d 0c e9                 	leaq	(%rcx,%r13,8), %rcx
1421a0f82: 48 8b 51 f4                 	movq	-0xc(%rcx), %rdx
1421a0f86: 48 89 55 70                 	movq	%rdx, 0x70(%rbp)
1421a0f8a: 88 45 78                    	movb	%al, 0x78(%rbp)
1421a0f8d: 0f b6 41 ff                 	movzbl	-0x1(%rcx), %eax
1421a0f91: 48 8d 55 79                 	leaq	0x79(%rbp), %rdx
1421a0f95: 88 42 02                    	movb	%al, 0x2(%rdx)
1421a0f98: 0f b7 41 fd                 	movzwl	-0x3(%rcx), %eax
1421a0f9c: 66 89 02                    	movw	%ax, (%rdx)
1421a0f9f: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a0fa6: 4c 8b ad 88 01 00 00        	movq	0x188(%rbp), %r13
1421a0fad: 4c 89 ea                    	movq	%r13, %rdx
1421a0fb0: 4c 8d 45 70                 	leaq	0x70(%rbp), %r8
1421a0fb4: e8 97 cb 72 fe              	callq	0x1408cdb50 <.text+0x8ccb50>
1421a0fb9: 8b 95 b0 01 00 00           	movl	0x1b0(%rbp), %edx
1421a0fbf: 85 d2                       	testl	%edx, %edx
1421a0fc1: 0f 84 88 0d 00 00           	je	0x1421a1d4f <.text+0x21a0d4f>
1421a0fc7: 0f 28 85 90 01 00 00        	movaps	0x190(%rbp), %xmm0
1421a0fce: 0f 28 8d a0 01 00 00        	movaps	0x1a0(%rbp), %xmm1
1421a0fd5: 0f 29 8d 30 01 00 00        	movaps	%xmm1, 0x130(%rbp)
1421a0fdc: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
1421a0fe3: 44 8b 85 b4 01 00 00        	movl	0x1b4(%rbp), %r8d
1421a0fea: 4c 89 e9                    	movq	%r13, %rcx
1421a0fed: e8 de cb 72 fe              	callq	0x1408cdbd0 <.text+0x8ccbd0>
1421a0ff2: 48 8b 48 18                 	movq	0x18(%rax), %rcx
1421a0ff6: f0                          	lock
1421a0ff7: 48 ff 01                    	incq	(%rcx)
1421a0ffa: 0f 8e a4 0f 00 00           	jle	0x1421a1fa4 <.text+0x21a0fa4>
1421a1000: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1421a1004: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
1421a100b: 49 89 50 10                 	movq	%rdx, 0x10(%r8)
1421a100f: 0f 10 00                    	movups	(%rax), %xmm0
1421a1012: 41 0f 11 00                 	movups	%xmm0, (%r8)
1421a1016: 66 0f 6f 85 20 01 00 00     	movdqa	0x120(%rbp), %xmm0
1421a101e: 66 0f 6f 8d 30 01 00 00     	movdqa	0x130(%rbp), %xmm1
1421a1026: 66 0f 7f 8d a0 01 00 00     	movdqa	%xmm1, 0x1a0(%rbp)
1421a102e: 66 0f 7f 85 90 01 00 00     	movdqa	%xmm0, 0x190(%rbp)
1421a1036: 48 89 8d c8 01 00 00        	movq	%rcx, 0x1c8(%rbp)
1421a103d: 44 8b 85 68 01 00 00        	movl	0x168(%rbp), %r8d
1421a1044: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a104b: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1421a1052: 4c 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %r9
1421a1059: e8 12 bc 05 ff              	callq	0x1411fcc70 <.text+0x11fbc70>
1421a105e: 80 bd b0 00 00 00 11        	cmpb	$0x11, 0xb0(%rbp)
1421a1065: 0f 84 d5 00 00 00           	je	0x1421a1140 <.text+0x21a0140>
1421a106b: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
1421a1072: f0                          	lock
1421a1073: 48 ff 08                    	decq	(%rax)
1421a1076: 0f 85 c4 00 00 00           	jne	0x1421a1140 <.text+0x21a0140>
1421a107c: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
1421a1083: e8 98 b5 9b fe              	callq	0x140b5c620 <.text+0xb5b620>
1421a1088: e9 b3 00 00 00              	jmp	0x1421a1140 <.text+0x21a0140>
1421a108d: 0f 1f 00                    	nopl	(%rax)
1421a1090: 66 45 85 f6                 	testw	%r14w, %r14w
1421a1094: 74 0a                       	je	0x1421a10a0 <.text+0x21a00a0>
1421a1096: 41 8d 46 ff                 	leal	-0x1(%r14), %eax
1421a109a: 44 21 f0                    	andl	%r14d, %eax
1421a109d: eb 30                       	jmp	0x1421a10cf <.text+0x21a00cf>
1421a109f: 90                          	nop
1421a10a0: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
1421a10a6: 66 44 0f d7 f0              	pmovmskb	%xmm0, %r14d
1421a10ab: 48 81 c3 00 fb ff ff        	addq	$-0x500, %rbx           # imm = 0xFB00
1421a10b2: 49 83 c4 10                 	addq	$0x10, %r12
1421a10b6: 41 81 fe ff ff 00 00        	cmpl	$0xffff, %r14d          # imm = 0xFFFF
1421a10bd: 74 e1                       	je	0x1421a10a0 <.text+0x21a00a0>
1421a10bf: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1421a10c4: 44 29 f1                    	subl	%r14d, %ecx
1421a10c7: 41 f7 d6                    	notl	%r14d
1421a10ca: 44 89 f0                    	movl	%r14d, %eax
1421a10cd: 21 c8                       	andl	%ecx, %eax
1421a10cf: 44 89 f1                    	movl	%r14d, %ecx
1421a10d2: f3 0f bc c9                 	tzcntl	%ecx, %ecx
1421a10d6: 41 89 c6                    	movl	%eax, %r14d
1421a10d9: 48 f7 d9                    	negq	%rcx
1421a10dc: 48 8d 04 89                 	leaq	(%rcx,%rcx,4), %rax
1421a10e0: 48 c1 e0 04                 	shlq	$0x4, %rax
1421a10e4: 8b 44 03 b0                 	movl	-0x50(%rbx,%rax), %eax
1421a10e8: 89 85 68 01 00 00           	movl	%eax, 0x168(%rbp)
1421a10ee: 48 83 7e 18 00              	cmpq	$0x0, 0x18(%rsi)
1421a10f3: 74 4b                       	je	0x1421a1140 <.text+0x21a0140>
1421a10f5: 4c 89 f9                    	movq	%r15, %rcx
1421a10f8: 48 8d 95 68 01 00 00        	leaq	0x168(%rbp), %rdx
1421a10ff: e8 cc fd 40 fe              	callq	0x1405b0ed0 <.text+0x5afed0>
1421a1104: 48 8b 0e                    	movq	(%rsi), %rcx
1421a1107: 48 8b 56 08                 	movq	0x8(%rsi), %rdx
1421a110b: 49 89 d0                    	movq	%rdx, %r8
1421a110e: 49 21 c0                    	andq	%rax, %r8
1421a1111: 48 c1 e8 39                 	shrq	$0x39, %rax
1421a1115: 66 0f 6e c0                 	movd	%eax, %xmm0
1421a1119: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1421a111d: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1421a1122: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1421a1127: 48 8d 41 e8                 	leaq	-0x18(%rcx), %rax
1421a112b: 44 8b 8d 68 01 00 00        	movl	0x168(%rbp), %r9d
1421a1132: 45 31 d2                    	xorl	%r10d, %r10d
1421a1135: e9 c6 fd ff ff              	jmp	0x1421a0f00 <.text+0x219ff00>
1421a113a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1421a1140: 48 ff cf                    	decq	%rdi
1421a1143: 0f 85 47 ff ff ff           	jne	0x1421a1090 <.text+0x21a0090>
1421a1149: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1421a1150: 48 8b 78 48                 	movq	0x48(%rax), %rdi
1421a1154: 48 85 ff                    	testq	%rdi, %rdi
1421a1157: 0f 84 7c 02 00 00           	je	0x1421a13d9 <.text+0x21a03d9>
1421a115d: 48 81 85 30 02 00 00 90 00 00 00    	addq	$0x90, 0x230(%rbp)
1421a1168: 4c 8b 68 30                 	movq	0x30(%rax), %r13
1421a116c: 4d 8d 75 10                 	leaq	0x10(%r13), %r14
1421a1170: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
1421a1176: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
1421a117b: 41 f7 d4                    	notl	%r12d
1421a117e: 48 8d 5e 50                 	leaq	0x50(%rsi), %rbx
1421a1182: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1421a1186: e9 95 01 00 00              	jmp	0x1421a1320 <.text+0x21a0320>
1421a118b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1421a1190: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1421a1196: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1421a119a: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1421a119e: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1421a11a3: 45 85 db                    	testl	%r11d, %r11d
1421a11a6: 74 28                       	je	0x1421a11d0 <.text+0x21a01d0>
1421a11a8: f3 45 0f bc fb              	tzcntl	%r11d, %r15d
1421a11ad: 4d 01 c7                    	addq	%r8, %r15
1421a11b0: 49 21 d7                    	andq	%rdx, %r15
1421a11b3: 49 f7 df                    	negq	%r15
1421a11b6: 4f 8d 3c 7f                 	leaq	(%r15,%r15,2), %r15
1421a11ba: 46 3b 0c f8                 	cmpl	(%rax,%r15,8), %r9d
1421a11be: 74 40                       	je	0x1421a1200 <.text+0x21a0200>
1421a11c0: 45 8d 7b ff                 	leal	-0x1(%r11), %r15d
1421a11c4: 66 45 21 df                 	andw	%r11w, %r15w
1421a11c8: 45 89 fb                    	movl	%r15d, %r11d
1421a11cb: 75 db                       	jne	0x1421a11a8 <.text+0x21a01a8>
1421a11cd: 0f 1f 00                    	nopl	(%rax)
1421a11d0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1421a11d4: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1421a11d9: 45 85 db                    	testl	%r11d, %r11d
1421a11dc: 0f 85 ee 01 00 00           	jne	0x1421a13d0 <.text+0x21a03d0>
1421a11e2: 4d 01 d0                    	addq	%r10, %r8
1421a11e5: 49 83 c0 10                 	addq	$0x10, %r8
1421a11e9: 49 83 c2 10                 	addq	$0x10, %r10
1421a11ed: 49 21 d0                    	andq	%rdx, %r8
1421a11f0: eb 9e                       	jmp	0x1421a1190 <.text+0x21a0190>
1421a11f2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1421a1200: 42 0f b6 44 f9 fc           	movzbl	-0x4(%rcx,%r15,8), %eax
1421a1206: 3c 11                       	cmpb	$0x11, %al
1421a1208: 0f 84 c2 01 00 00           	je	0x1421a13d0 <.text+0x21a03d0>
1421a120e: 4a 8d 0c f9                 	leaq	(%rcx,%r15,8), %rcx
1421a1212: 48 8b 51 f4                 	movq	-0xc(%rcx), %rdx
1421a1216: 48 89 55 c0                 	movq	%rdx, -0x40(%rbp)
1421a121a: 88 45 c8                    	movb	%al, -0x38(%rbp)
1421a121d: 0f b6 41 ff                 	movzbl	-0x1(%rcx), %eax
1421a1221: 48 8d 55 c9                 	leaq	-0x37(%rbp), %rdx
1421a1225: 88 42 02                    	movb	%al, 0x2(%rdx)
1421a1228: 0f b7 41 fd                 	movzwl	-0x3(%rcx), %eax
1421a122c: 66 89 02                    	movw	%ax, (%rdx)
1421a122f: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a1236: 4c 8b bd 88 01 00 00        	movq	0x188(%rbp), %r15
1421a123d: 4c 89 fa                    	movq	%r15, %rdx
1421a1240: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
1421a1244: e8 07 c9 72 fe              	callq	0x1408cdb50 <.text+0x8ccb50>
1421a1249: 8b 95 b0 01 00 00           	movl	0x1b0(%rbp), %edx
1421a124f: 85 d2                       	testl	%edx, %edx
1421a1251: 0f 84 d9 0b 00 00           	je	0x1421a1e30 <.text+0x21a0e30>
1421a1257: 0f 28 85 90 01 00 00        	movaps	0x190(%rbp), %xmm0
1421a125e: 0f 28 8d a0 01 00 00        	movaps	0x1a0(%rbp), %xmm1
1421a1265: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
1421a1269: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1421a126d: 44 8b 85 b4 01 00 00        	movl	0x1b4(%rbp), %r8d
1421a1274: 4c 89 f9                    	movq	%r15, %rcx
1421a1277: e8 54 c9 72 fe              	callq	0x1408cdbd0 <.text+0x8ccbd0>
1421a127c: 48 8b 48 18                 	movq	0x18(%rax), %rcx
1421a1280: f0                          	lock
1421a1281: 48 ff 01                    	incq	(%rcx)
1421a1284: 0f 8e 1a 0d 00 00           	jle	0x1421a1fa4 <.text+0x21a0fa4>
1421a128a: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1421a128e: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
1421a1295: 49 89 50 10                 	movq	%rdx, 0x10(%r8)
1421a1299: 0f 10 00                    	movups	(%rax), %xmm0
1421a129c: 41 0f 11 00                 	movups	%xmm0, (%r8)
1421a12a0: 66 0f 6f 45 e0              	movdqa	-0x20(%rbp), %xmm0
1421a12a5: 66 0f 6f 4d f0              	movdqa	-0x10(%rbp), %xmm1
1421a12aa: 66 0f 7f 8d a0 01 00 00     	movdqa	%xmm1, 0x1a0(%rbp)
1421a12b2: 66 0f 7f 85 90 01 00 00     	movdqa	%xmm0, 0x190(%rbp)
1421a12ba: 48 89 8d c8 01 00 00        	movq	%rcx, 0x1c8(%rbp)
1421a12c1: 44 8b 85 58 01 00 00        	movl	0x158(%rbp), %r8d
1421a12c8: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a12cf: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
1421a12d6: 4c 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %r9
1421a12dd: e8 8e b9 05 ff              	callq	0x1411fcc70 <.text+0x11fbc70>
1421a12e2: 80 bd b0 00 00 00 11        	cmpb	$0x11, 0xb0(%rbp)
1421a12e9: 0f 84 e1 00 00 00           	je	0x1421a13d0 <.text+0x21a03d0>
1421a12ef: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
1421a12f6: f0                          	lock
1421a12f7: 48 ff 08                    	decq	(%rax)
1421a12fa: 0f 85 d0 00 00 00           	jne	0x1421a13d0 <.text+0x21a03d0>
1421a1300: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
1421a1307: e8 14 b3 9b fe              	callq	0x140b5c620 <.text+0xb5b620>
1421a130c: e9 bf 00 00 00              	jmp	0x1421a13d0 <.text+0x21a03d0>
1421a1311: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1421a1320: 66 45 85 e4                 	testw	%r12w, %r12w
1421a1324: 74 0a                       	je	0x1421a1330 <.text+0x21a0330>
1421a1326: 41 8d 44 24 ff              	leal	-0x1(%r12), %eax
1421a132b: 44 21 e0                    	andl	%r12d, %eax
1421a132e: eb 2e                       	jmp	0x1421a135e <.text+0x21a035e>
1421a1330: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
1421a1335: 66 44 0f d7 e0              	pmovmskb	%xmm0, %r12d
1421a133a: 49 81 c5 00 fb ff ff        	addq	$-0x500, %r13           # imm = 0xFB00
1421a1341: 49 83 c6 10                 	addq	$0x10, %r14
1421a1345: 41 81 fc ff ff 00 00        	cmpl	$0xffff, %r12d          # imm = 0xFFFF
1421a134c: 74 e2                       	je	0x1421a1330 <.text+0x21a0330>
1421a134e: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1421a1353: 44 29 e1                    	subl	%r12d, %ecx
1421a1356: 41 f7 d4                    	notl	%r12d
1421a1359: 44 89 e0                    	movl	%r12d, %eax
1421a135c: 21 c8                       	andl	%ecx, %eax
1421a135e: 44 89 e1                    	movl	%r12d, %ecx
1421a1361: f3 0f bc c9                 	tzcntl	%ecx, %ecx
1421a1365: 41 89 c4                    	movl	%eax, %r12d
1421a1368: 48 f7 d9                    	negq	%rcx
1421a136b: 48 8d 04 89                 	leaq	(%rcx,%rcx,4), %rax
1421a136f: 48 c1 e0 04                 	shlq	$0x4, %rax
1421a1373: 41 8b 44 05 b0              	movl	-0x50(%r13,%rax), %eax
1421a1378: 89 85 58 01 00 00           	movl	%eax, 0x158(%rbp)
1421a137e: 48 83 7e 48 00              	cmpq	$0x0, 0x48(%rsi)
1421a1383: 74 4b                       	je	0x1421a13d0 <.text+0x21a03d0>
1421a1385: 48 89 d9                    	movq	%rbx, %rcx
1421a1388: 48 8d 95 58 01 00 00        	leaq	0x158(%rbp), %rdx
1421a138f: e8 3c fb 40 fe              	callq	0x1405b0ed0 <.text+0x5afed0>
1421a1394: 48 8b 4e 30                 	movq	0x30(%rsi), %rcx
1421a1398: 48 8b 56 38                 	movq	0x38(%rsi), %rdx
1421a139c: 49 89 d0                    	movq	%rdx, %r8
1421a139f: 49 21 c0                    	andq	%rax, %r8
1421a13a2: 48 c1 e8 39                 	shrq	$0x39, %rax
1421a13a6: 66 0f 6e c0                 	movd	%eax, %xmm0
1421a13aa: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1421a13ae: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1421a13b3: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1421a13b8: 48 8d 41 e8                 	leaq	-0x18(%rcx), %rax
1421a13bc: 44 8b 8d 58 01 00 00        	movl	0x158(%rbp), %r9d
1421a13c3: 45 31 d2                    	xorl	%r10d, %r10d
1421a13c6: e9 c5 fd ff ff              	jmp	0x1421a1190 <.text+0x21a0190>
1421a13cb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1421a13d0: 48 ff cf                    	decq	%rdi
1421a13d3: 0f 85 47 ff ff ff           	jne	0x1421a1320 <.text+0x21a0320>
1421a13d9: 0f 28 b5 50 02 00 00        	movaps	0x250(%rbp), %xmm6
1421a13e0: 0f 28 bd 60 02 00 00        	movaps	0x260(%rbp), %xmm7
1421a13e7: 44 0f 28 85 70 02 00 00     	movaps	0x270(%rbp), %xmm8
1421a13ef: 44 0f 28 8d 80 02 00 00     	movaps	0x280(%rbp), %xmm9
1421a13f7: 48 81 c4 18 03 00 00        	addq	$0x318, %rsp            # imm = 0x318
1421a13fe: 5b                          	popq	%rbx
1421a13ff: 5f                          	popq	%rdi
1421a1400: 5e                          	popq	%rsi
1421a1401: 41 5c                       	popq	%r12
1421a1403: 41 5d                       	popq	%r13
1421a1405: 41 5e                       	popq	%r14
1421a1407: 41 5f                       	popq	%r15
1421a1409: 5d                          	popq	%rbp
1421a140a: c3                          	retq
1421a140b: 48 89 8d 28 02 00 00        	movq	%rcx, 0x228(%rbp)
1421a1412: 48 89 95 38 02 00 00        	movq	%rdx, 0x238(%rbp)
1421a1419: 31 c9                       	xorl	%ecx, %ecx
1421a141b: e8 00 87 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a1420: 48 85 c0                    	testq	%rax, %rax
1421a1423: 0f 84 06 0b 00 00           	je	0x1421a1f2f <.text+0x21a0f2f>
1421a1429: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a142d: 48 ff 00                    	incq	(%rax)
1421a1430: 44 0f 29 8d 30 01 00 00     	movaps	%xmm9, 0x130(%rbp)
1421a1438: 44 0f 29 85 20 01 00 00     	movaps	%xmm8, 0x120(%rbp)
1421a1440: 66 0f 7f 85 40 01 00 00     	movdqa	%xmm0, 0x140(%rbp)
1421a1448: 31 c9                       	xorl	%ecx, %ecx
1421a144a: e8 d1 86 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a144f: 48 85 c0                    	testq	%rax, %rax
1421a1452: 0f 84 f3 0a 00 00           	je	0x1421a1f4b <.text+0x21a0f4b>
1421a1458: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a145c: 48 ff 00                    	incq	(%rax)
1421a145f: 66 0f 7f 85 b0 00 00 00     	movdqa	%xmm0, 0xb0(%rbp)
1421a1467: 0f 29 bd 90 00 00 00        	movaps	%xmm7, 0x90(%rbp)
1421a146e: 66 0f 7f b5 a0 00 00 00     	movdqa	%xmm6, 0xa0(%rbp)
1421a1476: 31 c9                       	xorl	%ecx, %ecx
1421a1478: e8 a3 86 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a147d: 48 85 c0                    	testq	%rax, %rax
1421a1480: 0f 84 e1 0a 00 00           	je	0x1421a1f67 <.text+0x21a0f67>
1421a1486: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1421a148a: 48 ff 00                    	incq	(%rax)
1421a148d: 0f 28 8d 20 01 00 00        	movaps	0x120(%rbp), %xmm1
1421a1494: 0f 28 95 30 01 00 00        	movaps	0x130(%rbp), %xmm2
1421a149b: 0f 28 9d 40 01 00 00        	movaps	0x140(%rbp), %xmm3
1421a14a2: 0f 29 9d e0 01 00 00        	movaps	%xmm3, 0x1e0(%rbp)
1421a14a9: 0f 29 95 d0 01 00 00        	movaps	%xmm2, 0x1d0(%rbp)
1421a14b0: 0f 29 8d c0 01 00 00        	movaps	%xmm1, 0x1c0(%rbp)
1421a14b7: 66 0f 6f 8d 90 00 00 00     	movdqa	0x90(%rbp), %xmm1
1421a14bf: 66 0f 6f 95 a0 00 00 00     	movdqa	0xa0(%rbp), %xmm2
1421a14c7: 66 0f 6f 9d b0 00 00 00     	movdqa	0xb0(%rbp), %xmm3
1421a14cf: 66 0f 7f 8d 90 01 00 00     	movdqa	%xmm1, 0x190(%rbp)
1421a14d7: 66 0f 7f 95 a0 01 00 00     	movdqa	%xmm2, 0x1a0(%rbp)
1421a14df: 66 0f 7f 9d b0 01 00 00     	movdqa	%xmm3, 0x1b0(%rbp)
1421a14e7: 0f 29 bd f0 01 00 00        	movaps	%xmm7, 0x1f0(%rbp)
1421a14ee: 66 0f 7f b5 00 02 00 00     	movdqa	%xmm6, 0x200(%rbp)
1421a14f6: 66 0f 7f 85 10 02 00 00     	movdqa	%xmm0, 0x210(%rbp)
1421a14fe: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a1505: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
1421a1509: e8 e2 cb 3e 00              	callq	0x14258e0f0 <.text+0x258d0f0>
1421a150e: 49 8d 9c 24 50 01 00 00     	leaq	0x150(%r12), %rbx
1421a1516: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a151d: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1421a1524: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1421a152a: e8 d1 26 78 00              	callq	0x142923c00 <.text+0x2922c00>
1421a152f: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1421a1536: 48 85 d2                    	testq	%rdx, %rdx
1421a1539: 74 12                       	je	0x1421a154d <.text+0x21a054d>
1421a153b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421a1541: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1421a1548: e8 d3 26 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a154d: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
1421a1554: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1421a155a: 48 89 d9                    	movq	%rbx, %rcx
1421a155d: e8 9e 26 78 00              	callq	0x142923c00 <.text+0x2922c00>
1421a1562: 45 31 ed                    	xorl	%r13d, %r13d
1421a1565: e9 a6 01 00 00              	jmp	0x1421a1710 <.text+0x21a0710>
1421a156a: 49 83 7c 24 10 00           	cmpq	$0x0, 0x10(%r12)
1421a1570: 0f 84 9d 07 00 00           	je	0x1421a1d13 <.text+0x21a0d13>
1421a1576: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1421a157d: 31 c9                       	xorl	%ecx, %ecx
1421a157f: e8 9c 85 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a1584: 48 85 c0                    	testq	%rax, %rax
1421a1587: 0f 84 e8 09 00 00           	je	0x1421a1f75 <.text+0x21a0f75>
1421a158d: 0f 10 00                    	movups	(%rax), %xmm0
1421a1590: 48 ff 00                    	incq	(%rax)
1421a1593: 44 0f 29 8d 30 01 00 00     	movaps	%xmm9, 0x130(%rbp)
1421a159b: 44 0f 29 85 20 01 00 00     	movaps	%xmm8, 0x120(%rbp)
1421a15a3: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1421a15aa: 31 c9                       	xorl	%ecx, %ecx
1421a15ac: e8 6f 85 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a15b1: 48 85 c0                    	testq	%rax, %rax
1421a15b4: 0f 84 d0 09 00 00           	je	0x1421a1f8a <.text+0x21a0f8a>
1421a15ba: 4d 89 ec                    	movq	%r13, %r12
1421a15bd: 0f 10 00                    	movups	(%rax), %xmm0
1421a15c0: 48 ff 00                    	incq	(%rax)
1421a15c3: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
1421a15ca: 0f 29 bd 90 01 00 00        	movaps	%xmm7, 0x190(%rbp)
1421a15d1: 66 0f 7f b5 a0 01 00 00     	movdqa	%xmm6, 0x1a0(%rbp)
1421a15d9: 31 c9                       	xorl	%ecx, %ecx
1421a15db: e8 40 85 f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a15e0: 48 85 c0                    	testq	%rax, %rax
1421a15e3: 0f 84 af 09 00 00           	je	0x1421a1f98 <.text+0x21a0f98>
1421a15e9: 0f 10 00                    	movups	(%rax), %xmm0
1421a15ec: 48 ff 00                    	incq	(%rax)
1421a15ef: 0f 28 8d 20 01 00 00        	movaps	0x120(%rbp), %xmm1
1421a15f6: 0f 28 95 30 01 00 00        	movaps	0x130(%rbp), %xmm2
1421a15fd: 0f 28 9d 40 01 00 00        	movaps	0x140(%rbp), %xmm3
1421a1604: 0f 29 9d e0 00 00 00        	movaps	%xmm3, 0xe0(%rbp)
1421a160b: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
1421a1612: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
1421a1619: 66 0f 6f 8d 90 01 00 00     	movdqa	0x190(%rbp), %xmm1
1421a1621: 66 0f 6f 95 a0 01 00 00     	movdqa	0x1a0(%rbp), %xmm2
1421a1629: 66 0f 6f 9d b0 01 00 00     	movdqa	0x1b0(%rbp), %xmm3
1421a1631: 66 0f 7f 8d 90 00 00 00     	movdqa	%xmm1, 0x90(%rbp)
1421a1639: 66 0f 7f 95 a0 00 00 00     	movdqa	%xmm2, 0xa0(%rbp)
1421a1641: 66 0f 7f 9d b0 00 00 00     	movdqa	%xmm3, 0xb0(%rbp)
1421a1649: 0f 29 bd f0 00 00 00        	movaps	%xmm7, 0xf0(%rbp)
1421a1650: 66 0f 7f b5 00 01 00 00     	movdqa	%xmm6, 0x100(%rbp)
1421a1658: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1421a165f: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a1666: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
1421a166a: e8 81 ca 3e 00              	callq	0x14258e0f0 <.text+0x258d0f0>
1421a166f: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a1676: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
1421a167d: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1421a1683: e8 78 25 78 00              	callq	0x142923c00 <.text+0x2922c00>
1421a1688: 49 8b 04 24                 	movq	(%r12), %rax
1421a168c: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
1421a1691: 48 21 cb                    	andq	%rcx, %rbx
1421a1694: f3 0f 6f 04 18              	movdqu	(%rax,%rbx), %xmm0
1421a1699: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1421a169d: 85 d2                       	testl	%edx, %edx
1421a169f: 0f 84 4b 06 00 00           	je	0x1421a1cf0 <.text+0x21a0cf0>
1421a16a5: f3 0f bc d2                 	tzcntl	%edx, %edx
1421a16a9: 48 01 da                    	addq	%rbx, %rdx
1421a16ac: 48 21 ca                    	andq	%rcx, %rdx
1421a16af: 44 0f b6 04 10              	movzbl	(%rax,%rdx), %r8d
1421a16b4: 45 84 c0                    	testb	%r8b, %r8b
1421a16b7: 0f 89 7c 06 00 00           	jns	0x1421a1d39 <.text+0x21a0d39>
1421a16bd: 4c 8d 4a f0                 	leaq	-0x10(%rdx), %r9
1421a16c1: 49 21 c9                    	andq	%rcx, %r9
1421a16c4: 40 88 3c 10                 	movb	%dil, (%rax,%rdx)
1421a16c8: 42 88 7c 08 10              	movb	%dil, 0x10(%rax,%r9)
1421a16cd: 41 80 e0 01                 	andb	$0x1, %r8b
1421a16d1: 41 0f b6 c8                 	movzbl	%r8b, %ecx
1421a16d5: 49 29 4c 24 10              	subq	%rcx, 0x10(%r12)
1421a16da: 48 f7 da                    	negq	%rdx
1421a16dd: 48 69 ca 98 00 00 00        	imulq	$0x98, %rdx, %rcx
1421a16e4: 4c 89 b4 08 68 ff ff ff     	movq	%r14, -0x98(%rax,%rcx)
1421a16ec: 48 01 c1                    	addq	%rax, %rcx
1421a16ef: 48 81 c1 70 ff ff ff        	addq	$-0x90, %rcx
1421a16f6: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1421a16fd: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1421a1703: e8 f8 24 78 00              	callq	0x142923c00 <.text+0x2922c00>
1421a1708: 49 ff 44 24 18              	incq	0x18(%r12)
1421a170d: 41 b5 01                    	movb	$0x1, %r13b
1421a1710: 4c 8b 7d 10                 	movq	0x10(%rbp), %r15
1421a1714: 48 8b 5d 28                 	movq	0x28(%rbp), %rbx
1421a1718: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
1421a171d: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1421a1721: f7 d0                       	notl	%eax
1421a1723: 49 8d 7f 10                 	leaq	0x10(%r15), %rdi
1421a1727: 4c 89 a5 30 02 00 00        	movq	%r12, 0x230(%rbp)
1421a172e: 49 8d 4c 24 60              	leaq	0x60(%r12), %rcx
1421a1733: 48 89 8d 88 00 00 00        	movq	%rcx, 0x88(%rbp)
1421a173a: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1421a173e: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
1421a1745: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1421a1749: e9 b5 01 00 00              	jmp	0x1421a1903 <.text+0x21a0903>
1421a174e: 66 90                       	nop
1421a1750: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1421a1756: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1421a175a: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1421a175e: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1421a1763: 45 85 db                    	testl	%r11d, %r11d
1421a1766: 74 28                       	je	0x1421a1790 <.text+0x21a0790>
1421a1768: f3 45 0f bc f3              	tzcntl	%r11d, %r14d
1421a176d: 4d 01 c6                    	addq	%r8, %r14
1421a1770: 49 21 d6                    	andq	%rdx, %r14
1421a1773: 49 f7 de                    	negq	%r14
1421a1776: 4f 8d 34 76                 	leaq	(%r14,%r14,2), %r14
1421a177a: 46 3b 0c f0                 	cmpl	(%rax,%r14,8), %r9d
1421a177e: 74 40                       	je	0x1421a17c0 <.text+0x21a07c0>
1421a1780: 45 8d 73 ff                 	leal	-0x1(%r11), %r14d
1421a1784: 66 45 21 de                 	andw	%r11w, %r14w
1421a1788: 45 89 f3                    	movl	%r14d, %r11d
1421a178b: 75 db                       	jne	0x1421a1768 <.text+0x21a0768>
1421a178d: 0f 1f 00                    	nopl	(%rax)
1421a1790: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1421a1794: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1421a1799: 45 85 db                    	testl	%r11d, %r11d
1421a179c: 0f 85 5e 01 00 00           	jne	0x1421a1900 <.text+0x21a0900>
1421a17a2: 4d 01 d0                    	addq	%r10, %r8
1421a17a5: 49 83 c0 10                 	addq	$0x10, %r8
1421a17a9: 49 83 c2 10                 	addq	$0x10, %r10
1421a17ad: 49 21 d0                    	andq	%rdx, %r8
1421a17b0: eb 9e                       	jmp	0x1421a1750 <.text+0x21a0750>
1421a17b2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1421a17c0: 42 0f b6 54 f1 fc           	movzbl	-0x4(%rcx,%r14,8), %edx
1421a17c6: 44 89 e0                    	movl	%r12d, %eax
1421a17c9: 80 fa 11                    	cmpb	$0x11, %dl
1421a17cc: 0f 84 31 01 00 00           	je	0x1421a1903 <.text+0x21a0903>
1421a17d2: 4a 8d 04 f1                 	leaq	(%rcx,%r14,8), %rax
1421a17d6: 48 8b 48 f4                 	movq	-0xc(%rax), %rcx
1421a17da: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
1421a17e1: 88 95 70 01 00 00           	movb	%dl, 0x170(%rbp)
1421a17e7: 0f b6 48 ff                 	movzbl	-0x1(%rax), %ecx
1421a17eb: 48 8d 95 71 01 00 00        	leaq	0x171(%rbp), %rdx
1421a17f2: 88 4a 02                    	movb	%cl, 0x2(%rdx)
1421a17f5: 0f b7 40 fd                 	movzwl	-0x3(%rax), %eax
1421a17f9: 66 89 02                    	movw	%ax, (%rdx)
1421a17fc: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a1803: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a180a: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
1421a1811: 4c 8d 85 68 01 00 00        	leaq	0x168(%rbp), %r8
1421a1818: e8 33 c3 72 fe              	callq	0x1408cdb50 <.text+0x8ccb50>
1421a181d: 8b 95 b0 01 00 00           	movl	0x1b0(%rbp), %edx
1421a1823: 85 d2                       	testl	%edx, %edx
1421a1825: 0f 84 67 05 00 00           	je	0x1421a1d92 <.text+0x21a0d92>
1421a182b: 0f 28 85 90 01 00 00        	movaps	0x190(%rbp), %xmm0
1421a1832: 0f 28 8d a0 01 00 00        	movaps	0x1a0(%rbp), %xmm1
1421a1839: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1421a183d: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1421a1841: 44 8b 85 b4 01 00 00        	movl	0x1b4(%rbp), %r8d
1421a1848: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a184f: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1421a1856: e8 75 c3 72 fe              	callq	0x1408cdbd0 <.text+0x8ccbd0>
1421a185b: 48 8b 48 18                 	movq	0x18(%rax), %rcx
1421a185f: f0                          	lock
1421a1860: 48 ff 01                    	incq	(%rcx)
1421a1863: 0f 8e 3b 07 00 00           	jle	0x1421a1fa4 <.text+0x21a0fa4>
1421a1869: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1421a186d: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
1421a1874: 49 89 50 10                 	movq	%rdx, 0x10(%r8)
1421a1878: 0f 10 00                    	movups	(%rax), %xmm0
1421a187b: 41 0f 11 00                 	movups	%xmm0, (%r8)
1421a187f: 66 0f 6f 45 c0              	movdqa	-0x40(%rbp), %xmm0
1421a1884: 66 0f 6f 4d d0              	movdqa	-0x30(%rbp), %xmm1
1421a1889: 66 0f 7f 8d a0 01 00 00     	movdqa	%xmm1, 0x1a0(%rbp)
1421a1891: 66 0f 7f 85 90 01 00 00     	movdqa	%xmm0, 0x190(%rbp)
1421a1899: 48 89 8d c8 01 00 00        	movq	%rcx, 0x1c8(%rbp)
1421a18a0: 44 8b 85 84 01 00 00        	movl	0x184(%rbp), %r8d
1421a18a7: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a18ae: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a18b5: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1421a18bc: 4c 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %r9
1421a18c3: e8 a8 b3 05 ff              	callq	0x1411fcc70 <.text+0x11fbc70>
1421a18c8: 80 bd b0 00 00 00 11        	cmpb	$0x11, 0xb0(%rbp)
1421a18cf: 74 20                       	je	0x1421a18f1 <.text+0x21a08f1>
1421a18d1: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
1421a18d8: f0                          	lock
1421a18d9: 48 ff 08                    	decq	(%rax)
1421a18dc: 75 13                       	jne	0x1421a18f1 <.text+0x21a08f1>
1421a18de: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a18e5: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
1421a18ec: e8 2f ad 9b fe              	callq	0x140b5c620 <.text+0xb5b620>
1421a18f1: 44 89 e0                    	movl	%r12d, %eax
1421a18f4: eb 0d                       	jmp	0x1421a1903 <.text+0x21a0903>
1421a18f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1421a1900: 44 89 e0                    	movl	%r12d, %eax
1421a1903: 48 85 db                    	testq	%rbx, %rbx
1421a1906: 0f 84 b4 00 00 00           	je	0x1421a19c0 <.text+0x21a09c0>
1421a190c: 66 85 c0                    	testw	%ax, %ax
1421a190f: 74 0f                       	je	0x1421a1920 <.text+0x21a0920>
1421a1911: 44 8d 60 ff                 	leal	-0x1(%rax), %r12d
1421a1915: 41 21 c4                    	andl	%eax, %r12d
1421a1918: eb 2f                       	jmp	0x1421a1949 <.text+0x21a0949>
1421a191a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1421a1920: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
1421a1924: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1421a1928: 49 81 c7 00 fb ff ff        	addq	$-0x500, %r15           # imm = 0xFB00
1421a192f: 48 83 c7 10                 	addq	$0x10, %rdi
1421a1933: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1421a1938: 74 e6                       	je	0x1421a1920 <.text+0x21a0920>
1421a193a: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1421a193f: 29 c1                       	subl	%eax, %ecx
1421a1941: f7 d0                       	notl	%eax
1421a1943: 41 89 c4                    	movl	%eax, %r12d
1421a1946: 41 21 cc                    	andl	%ecx, %r12d
1421a1949: 48 ff cb                    	decq	%rbx
1421a194c: f3 0f bc c0                 	tzcntl	%eax, %eax
1421a1950: 48 f7 d8                    	negq	%rax
1421a1953: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1421a1957: 48 c1 e0 04                 	shlq	$0x4, %rax
1421a195b: 41 8b 44 07 b0              	movl	-0x50(%r15,%rax), %eax
1421a1960: 89 85 84 01 00 00           	movl	%eax, 0x184(%rbp)
1421a1966: 48 83 7e 18 00              	cmpq	$0x0, 0x18(%rsi)
1421a196b: 44 89 e0                    	movl	%r12d, %eax
1421a196e: 74 93                       	je	0x1421a1903 <.text+0x21a0903>
1421a1970: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a1977: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1421a197e: 48 8d 95 84 01 00 00        	leaq	0x184(%rbp), %rdx
1421a1985: e8 46 f5 40 fe              	callq	0x1405b0ed0 <.text+0x5afed0>
1421a198a: 48 8b 0e                    	movq	(%rsi), %rcx
1421a198d: 48 8b 56 08                 	movq	0x8(%rsi), %rdx
1421a1991: 49 89 d0                    	movq	%rdx, %r8
1421a1994: 49 21 c0                    	andq	%rax, %r8
1421a1997: 48 c1 e8 39                 	shrq	$0x39, %rax
1421a199b: 66 0f 6e c0                 	movd	%eax, %xmm0
1421a199f: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1421a19a3: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1421a19a8: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1421a19ad: 48 8d 41 e8                 	leaq	-0x18(%rcx), %rax
1421a19b1: 44 8b 8d 84 01 00 00        	movl	0x184(%rbp), %r9d
1421a19b8: 45 31 d2                    	xorl	%r10d, %r10d
1421a19bb: e9 90 fd ff ff              	jmp	0x1421a1750 <.text+0x21a0750>
1421a19c0: 4c 8b 75 10                 	movq	0x10(%rbp), %r14
1421a19c4: 4c 8b 65 28                 	movq	0x28(%rbp), %r12
1421a19c8: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
1421a19cd: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1421a19d1: f7 d0                       	notl	%eax
1421a19d3: 49 8d 7e 10                 	leaq	0x10(%r14), %rdi
1421a19d7: 48 81 85 30 02 00 00 90 00 00 00    	addq	$0x90, 0x230(%rbp)
1421a19e2: 48 8d 4e 50                 	leaq	0x50(%rsi), %rcx
1421a19e6: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
1421a19ed: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1421a19f1: e9 cd 01 00 00              	jmp	0x1421a1bc3 <.text+0x21a0bc3>
1421a19f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1421a1a00: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1421a1a06: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1421a1a0a: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1421a1a0e: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1421a1a13: 45 85 db                    	testl	%r11d, %r11d
1421a1a16: 74 28                       	je	0x1421a1a40 <.text+0x21a0a40>
1421a1a18: f3 41 0f bc db              	tzcntl	%r11d, %ebx
1421a1a1d: 4c 01 c3                    	addq	%r8, %rbx
1421a1a20: 48 21 d3                    	andq	%rdx, %rbx
1421a1a23: 48 f7 db                    	negq	%rbx
1421a1a26: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
1421a1a2a: 44 3b 0c d8                 	cmpl	(%rax,%rbx,8), %r9d
1421a1a2e: 74 40                       	je	0x1421a1a70 <.text+0x21a0a70>
1421a1a30: 41 8d 5b ff                 	leal	-0x1(%r11), %ebx
1421a1a34: 66 44 21 db                 	andw	%r11w, %bx
1421a1a38: 41 89 db                    	movl	%ebx, %r11d
1421a1a3b: 75 db                       	jne	0x1421a1a18 <.text+0x21a0a18>
1421a1a3d: 0f 1f 00                    	nopl	(%rax)
1421a1a40: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1421a1a44: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1421a1a49: 45 85 db                    	testl	%r11d, %r11d
1421a1a4c: 0f 85 6e 01 00 00           	jne	0x1421a1bc0 <.text+0x21a0bc0>
1421a1a52: 4d 01 d0                    	addq	%r10, %r8
1421a1a55: 49 83 c0 10                 	addq	$0x10, %r8
1421a1a59: 49 83 c2 10                 	addq	$0x10, %r10
1421a1a5d: 49 21 d0                    	andq	%rdx, %r8
1421a1a60: eb 9e                       	jmp	0x1421a1a00 <.text+0x21a0a00>
1421a1a62: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1421a1a70: 0f b6 54 d9 fc              	movzbl	-0x4(%rcx,%rbx,8), %edx
1421a1a75: 44 89 f8                    	movl	%r15d, %eax
1421a1a78: 80 fa 11                    	cmpb	$0x11, %dl
1421a1a7b: 0f 84 42 01 00 00           	je	0x1421a1bc3 <.text+0x21a0bc3>
1421a1a81: 48 8d 04 d9                 	leaq	(%rcx,%rbx,8), %rax
1421a1a85: 48 8b 48 f4                 	movq	-0xc(%rax), %rcx
1421a1a89: 48 89 8d 58 01 00 00        	movq	%rcx, 0x158(%rbp)
1421a1a90: 88 95 60 01 00 00           	movb	%dl, 0x160(%rbp)
1421a1a96: 0f b6 48 ff                 	movzbl	-0x1(%rax), %ecx
1421a1a9a: 48 8d 95 61 01 00 00        	leaq	0x161(%rbp), %rdx
1421a1aa1: 88 4a 02                    	movb	%cl, 0x2(%rdx)
1421a1aa4: 0f b7 40 fd                 	movzwl	-0x3(%rax), %eax
1421a1aa8: 66 89 02                    	movw	%ax, (%rdx)
1421a1aab: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a1ab2: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a1ab9: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
1421a1ac0: 4c 8d 85 58 01 00 00        	leaq	0x158(%rbp), %r8
1421a1ac7: e8 84 c0 72 fe              	callq	0x1408cdb50 <.text+0x8ccb50>
1421a1acc: 8b 95 b0 01 00 00           	movl	0x1b0(%rbp), %edx
1421a1ad2: 85 d2                       	testl	%edx, %edx
1421a1ad4: 0f 84 07 03 00 00           	je	0x1421a1de1 <.text+0x21a0de1>
1421a1ada: 0f 28 85 90 01 00 00        	movaps	0x190(%rbp), %xmm0
1421a1ae1: 0f 28 8d a0 01 00 00        	movaps	0x1a0(%rbp), %xmm1
1421a1ae8: 0f 29 8d 30 01 00 00        	movaps	%xmm1, 0x130(%rbp)
1421a1aef: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
1421a1af6: 44 8b 85 b4 01 00 00        	movl	0x1b4(%rbp), %r8d
1421a1afd: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a1b04: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1421a1b0b: e8 c0 c0 72 fe              	callq	0x1408cdbd0 <.text+0x8ccbd0>
1421a1b10: 48 8b 48 18                 	movq	0x18(%rax), %rcx
1421a1b14: f0                          	lock
1421a1b15: 48 ff 01                    	incq	(%rcx)
1421a1b18: 0f 8e 86 04 00 00           	jle	0x1421a1fa4 <.text+0x21a0fa4>
1421a1b1e: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1421a1b22: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
1421a1b29: 49 89 50 10                 	movq	%rdx, 0x10(%r8)
1421a1b2d: 0f 10 00                    	movups	(%rax), %xmm0
1421a1b30: 41 0f 11 00                 	movups	%xmm0, (%r8)
1421a1b34: 66 0f 6f 85 20 01 00 00     	movdqa	0x120(%rbp), %xmm0
1421a1b3c: 66 0f 6f 8d 30 01 00 00     	movdqa	0x130(%rbp), %xmm1
1421a1b44: 66 0f 7f 8d a0 01 00 00     	movdqa	%xmm1, 0x1a0(%rbp)
1421a1b4c: 66 0f 7f 85 90 01 00 00     	movdqa	%xmm0, 0x190(%rbp)
1421a1b54: 48 89 8d c8 01 00 00        	movq	%rcx, 0x1c8(%rbp)
1421a1b5b: 44 8b 85 80 01 00 00        	movl	0x180(%rbp), %r8d
1421a1b62: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a1b69: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a1b70: 48 8b 95 30 02 00 00        	movq	0x230(%rbp), %rdx
1421a1b77: 4c 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %r9
1421a1b7e: e8 ed b0 05 ff              	callq	0x1411fcc70 <.text+0x11fbc70>
1421a1b83: 80 bd b0 00 00 00 11        	cmpb	$0x11, 0xb0(%rbp)
1421a1b8a: 74 20                       	je	0x1421a1bac <.text+0x21a0bac>
1421a1b8c: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
1421a1b93: f0                          	lock
1421a1b94: 48 ff 08                    	decq	(%rax)
1421a1b97: 75 13                       	jne	0x1421a1bac <.text+0x21a0bac>
1421a1b99: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a1ba0: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
1421a1ba7: e8 74 aa 9b fe              	callq	0x140b5c620 <.text+0xb5b620>
1421a1bac: 44 89 f8                    	movl	%r15d, %eax
1421a1baf: eb 12                       	jmp	0x1421a1bc3 <.text+0x21a0bc3>
1421a1bb1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1421a1bc0: 44 89 f8                    	movl	%r15d, %eax
1421a1bc3: 4d 85 e4                    	testq	%r12, %r12
1421a1bc6: 0f 84 5c f0 ff ff           	je	0x1421a0c28 <.text+0x219fc28>
1421a1bcc: 66 85 c0                    	testw	%ax, %ax
1421a1bcf: 74 0f                       	je	0x1421a1be0 <.text+0x21a0be0>
1421a1bd1: 44 8d 78 ff                 	leal	-0x1(%rax), %r15d
1421a1bd5: 41 21 c7                    	andl	%eax, %r15d
1421a1bd8: eb 2f                       	jmp	0x1421a1c09 <.text+0x21a0c09>
1421a1bda: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1421a1be0: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
1421a1be4: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1421a1be8: 49 81 c6 00 fb ff ff        	addq	$-0x500, %r14           # imm = 0xFB00
1421a1bef: 48 83 c7 10                 	addq	$0x10, %rdi
1421a1bf3: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1421a1bf8: 74 e6                       	je	0x1421a1be0 <.text+0x21a0be0>
1421a1bfa: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1421a1bff: 29 c1                       	subl	%eax, %ecx
1421a1c01: f7 d0                       	notl	%eax
1421a1c03: 41 89 c7                    	movl	%eax, %r15d
1421a1c06: 41 21 cf                    	andl	%ecx, %r15d
1421a1c09: 49 ff cc                    	decq	%r12
1421a1c0c: f3 0f bc c0                 	tzcntl	%eax, %eax
1421a1c10: 48 f7 d8                    	negq	%rax
1421a1c13: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1421a1c17: 48 c1 e0 04                 	shlq	$0x4, %rax
1421a1c1b: 41 8b 44 06 b0              	movl	-0x50(%r14,%rax), %eax
1421a1c20: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
1421a1c26: 48 83 7e 48 00              	cmpq	$0x0, 0x48(%rsi)
1421a1c2b: 44 89 f8                    	movl	%r15d, %eax
1421a1c2e: 74 93                       	je	0x1421a1bc3 <.text+0x21a0bc3>
1421a1c30: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a1c37: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1421a1c3e: 48 8d 95 80 01 00 00        	leaq	0x180(%rbp), %rdx
1421a1c45: e8 86 f2 40 fe              	callq	0x1405b0ed0 <.text+0x5afed0>
1421a1c4a: 48 8b 4e 30                 	movq	0x30(%rsi), %rcx
1421a1c4e: 48 8b 56 38                 	movq	0x38(%rsi), %rdx
1421a1c52: 49 89 d0                    	movq	%rdx, %r8
1421a1c55: 49 21 c0                    	andq	%rax, %r8
1421a1c58: 48 c1 e8 39                 	shrq	$0x39, %rax
1421a1c5c: 66 0f 6e c0                 	movd	%eax, %xmm0
1421a1c60: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1421a1c64: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1421a1c69: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1421a1c6e: 48 8d 41 e8                 	leaq	-0x18(%rcx), %rax
1421a1c72: 44 8b 8d 80 01 00 00        	movl	0x180(%rbp), %r9d
1421a1c79: 45 31 d2                    	xorl	%r10d, %r10d
1421a1c7c: e9 7f fd ff ff              	jmp	0x1421a1a00 <.text+0x21a0a00>
1421a1c81: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1421a1c87: 4c 01 c3                    	addq	%r8, %rbx
1421a1c8a: 48 21 cb                    	andq	%rcx, %rbx
1421a1c8d: f3 0f 6f 04 18              	movdqu	(%rax,%rbx), %xmm0
1421a1c92: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1421a1c96: 49 83 c0 10                 	addq	$0x10, %r8
1421a1c9a: 85 d2                       	testl	%edx, %edx
1421a1c9c: 0f 85 9a f1 ff ff           	jne	0x1421a0e3c <.text+0x219fe3c>
1421a1ca2: eb e3                       	jmp	0x1421a1c87 <.text+0x21a0c87>
1421a1ca4: ba 01 00 00 00              	movl	$0x1, %edx
1421a1ca9: 4c 89 e1                    	movq	%r12, %rcx
1421a1cac: 4d 89 f8                    	movq	%r15, %r8
1421a1caf: 41 b1 01                    	movb	$0x1, %r9b
1421a1cb2: e8 29 1a 71 00              	callq	0x1428b36e0 <.text+0x28b26e0>
1421a1cb7: 4c 8b b5 90 01 00 00        	movq	0x190(%rbp), %r14
1421a1cbe: 31 c9                       	xorl	%ecx, %ecx
1421a1cc0: e8 5b 7e f3 fd              	callq	0x1400d9b20 <.text+0xd8b20>
1421a1cc5: 48 85 c0                    	testq	%rax, %rax
1421a1cc8: 0f 85 51 f0 ff ff           	jne	0x1421a0d1f <.text+0x219fd1f>
1421a1cce: 48 8d 0d 13 48 e5 00        	leaq	0xe54813(%rip), %rcx    # 0x142ff64e8
1421a1cd5: e8 f6 63 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1cda: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
1421a1cde: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1421a1ce2: f3 0f bc d2                 	tzcntl	%edx, %edx
1421a1ce6: 44 0f b6 04 10              	movzbl	(%rax,%rdx), %r8d
1421a1ceb: e9 64 f1 ff ff              	jmp	0x1421a0e54 <.text+0x219fe54>
1421a1cf0: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1421a1cf6: 4c 01 c3                    	addq	%r8, %rbx
1421a1cf9: 48 21 cb                    	andq	%rcx, %rbx
1421a1cfc: f3 0f 6f 04 18              	movdqu	(%rax,%rbx), %xmm0
1421a1d01: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1421a1d05: 49 83 c0 10                 	addq	$0x10, %r8
1421a1d09: 85 d2                       	testl	%edx, %edx
1421a1d0b: 0f 85 94 f9 ff ff           	jne	0x1421a16a5 <.text+0x21a06a5>
1421a1d11: eb e3                       	jmp	0x1421a1cf6 <.text+0x21a0cf6>
1421a1d13: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1421a1d1a: ba 01 00 00 00              	movl	$0x1, %edx
1421a1d1f: 4c 89 e9                    	movq	%r13, %rcx
1421a1d22: 4d 89 f8                    	movq	%r15, %r8
1421a1d25: 41 b1 01                    	movb	$0x1, %r9b
1421a1d28: e8 b3 19 71 00              	callq	0x1428b36e0 <.text+0x28b26e0>
1421a1d2d: 4c 8b b5 90 01 00 00        	movq	0x190(%rbp), %r14
1421a1d34: e9 3d f8 ff ff              	jmp	0x1421a1576 <.text+0x21a0576>
1421a1d39: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
1421a1d3d: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1421a1d41: f3 0f bc d2                 	tzcntl	%edx, %edx
1421a1d45: 44 0f b6 04 10              	movzbl	(%rax,%rdx), %r8d
1421a1d4a: e9 6e f9 ff ff              	jmp	0x1421a16bd <.text+0x21a06bd>
1421a1d4f: 48 8d 05 52 48 e5 00        	leaq	0xe54852(%rip), %rax    # 0x142ff65a8
1421a1d56: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1421a1d5d: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1421a1d68: 48 c7 85 a0 01 00 00 08 00 00 00    	movq	$0x8, 0x1a0(%rbp)
1421a1d73: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1421a1d77: f3 0f 7f 85 a8 01 00 00     	movdqu	%xmm0, 0x1a8(%rbp)
1421a1d7f: 48 8d 15 ba 48 e5 00        	leaq	0xe548ba(%rip), %rdx    # 0x142ff6640
1421a1d86: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a1d8d: e8 fe 75 73 00              	callq	0x1428d9390 <.text+0x28d8390>
1421a1d92: 48 8d 05 0f 48 e5 00        	leaq	0xe5480f(%rip), %rax    # 0x142ff65a8
1421a1d99: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1421a1da0: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1421a1dab: 48 c7 85 a0 01 00 00 08 00 00 00    	movq	$0x8, 0x1a0(%rbp)
1421a1db6: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1421a1dba: f3 0f 7f 85 a8 01 00 00     	movdqu	%xmm0, 0x1a8(%rbp)
1421a1dc2: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a1dc9: 48 8d 15 70 48 e5 00        	leaq	0xe54870(%rip), %rdx    # 0x142ff6640
1421a1dd0: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a1dd7: e8 b4 75 73 00              	callq	0x1428d9390 <.text+0x28d8390>
1421a1ddc: e9 c3 01 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1de1: 48 8d 05 c0 47 e5 00        	leaq	0xe547c0(%rip), %rax    # 0x142ff65a8
1421a1de8: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1421a1def: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1421a1dfa: 48 c7 85 a0 01 00 00 08 00 00 00    	movq	$0x8, 0x1a0(%rbp)
1421a1e05: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1421a1e09: f3 0f 7f 85 a8 01 00 00     	movdqu	%xmm0, 0x1a8(%rbp)
1421a1e11: 44 88 ad 47 02 00 00        	movb	%r13b, 0x247(%rbp)
1421a1e18: 48 8d 15 09 48 e5 00        	leaq	0xe54809(%rip), %rdx    # 0x142ff6628
1421a1e1f: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a1e26: e8 65 75 73 00              	callq	0x1428d9390 <.text+0x28d8390>
1421a1e2b: e9 74 01 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1e30: 48 8d 05 71 47 e5 00        	leaq	0xe54771(%rip), %rax    # 0x142ff65a8
1421a1e37: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1421a1e3e: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1421a1e49: 48 c7 85 a0 01 00 00 08 00 00 00    	movq	$0x8, 0x1a0(%rbp)
1421a1e54: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1421a1e58: f3 0f 7f 85 a8 01 00 00     	movdqu	%xmm0, 0x1a8(%rbp)
1421a1e60: 48 8d 15 c1 47 e5 00        	leaq	0xe547c1(%rip), %rdx    # 0x142ff6628
1421a1e67: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a1e6e: e8 1d 75 73 00              	callq	0x1428d9390 <.text+0x28d8390>
1421a1e73: 45 31 e4                    	xorl	%r12d, %r12d
1421a1e76: 4c 8d 05 53 48 e5 00        	leaq	0xe54853(%rip), %r8     # 0x142ff66d0
1421a1e7d: 4c 89 e1                    	movq	%r12, %rcx
1421a1e80: 48 89 fa                    	movq	%rdi, %rdx
1421a1e83: e8 5b 71 73 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1421a1e88: 48 8d 0d 59 46 e5 00        	leaq	0xe54659(%rip), %rcx    # 0x142ff64e8
1421a1e8f: e8 3c 62 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1e94: e9 0b 01 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1e99: 48 8d 0d 48 46 e5 00        	leaq	0xe54648(%rip), %rcx    # 0x142ff64e8
1421a1ea0: e8 2b 62 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1ea5: e9 fa 00 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1eaa: 48 8d 0d 37 46 e5 00        	leaq	0xe54637(%rip), %rcx    # 0x142ff64e8
1421a1eb1: e8 1a 62 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1eb6: e9 e9 00 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1ebb: 48 8d 0d 26 46 e5 00        	leaq	0xe54626(%rip), %rcx    # 0x142ff64e8
1421a1ec2: e8 09 62 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1ec7: e9 d8 00 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1ecc: 48 8d 0d 15 46 e5 00        	leaq	0xe54615(%rip), %rcx    # 0x142ff64e8
1421a1ed3: e8 f8 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1ed8: e9 c7 00 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1edd: 48 8d 0d 04 46 e5 00        	leaq	0xe54604(%rip), %rcx    # 0x142ff64e8
1421a1ee4: e8 e7 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1ee9: e9 b6 00 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1eee: 48 8d 0d f3 45 e5 00        	leaq	0xe545f3(%rip), %rcx    # 0x142ff64e8
1421a1ef5: e8 d6 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1efa: e9 a5 00 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1eff: 48 8d 0d e2 45 e5 00        	leaq	0xe545e2(%rip), %rcx    # 0x142ff64e8
1421a1f06: e8 c5 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f0b: e9 94 00 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f10: 48 8d 0d d1 45 e5 00        	leaq	0xe545d1(%rip), %rcx    # 0x142ff64e8
1421a1f17: e8 b4 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f1c: e9 83 00 00 00              	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f21: 48 8d 0d c0 45 e5 00        	leaq	0xe545c0(%rip), %rcx    # 0x142ff64e8
1421a1f28: e8 a3 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f2d: eb 75                       	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f2f: 48 8d 0d b2 45 e5 00        	leaq	0xe545b2(%rip), %rcx    # 0x142ff64e8
1421a1f36: e8 95 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f3b: eb 67                       	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f3d: 48 8d 0d a4 45 e5 00        	leaq	0xe545a4(%rip), %rcx    # 0x142ff64e8
1421a1f44: e8 87 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f49: eb 59                       	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f4b: 48 8d 0d 96 45 e5 00        	leaq	0xe54596(%rip), %rcx    # 0x142ff64e8
1421a1f52: e8 79 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f57: eb 4b                       	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f59: 48 8d 0d 88 45 e5 00        	leaq	0xe54588(%rip), %rcx    # 0x142ff64e8
1421a1f60: e8 6b 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f65: eb 3d                       	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f67: 48 8d 0d 7a 45 e5 00        	leaq	0xe5457a(%rip), %rcx    # 0x142ff64e8
1421a1f6e: e8 5d 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f73: eb 2f                       	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f75: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1421a1f7c: 48 8d 0d 65 45 e5 00        	leaq	0xe54565(%rip), %rcx    # 0x142ff64e8
1421a1f83: e8 48 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f88: eb 1a                       	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f8a: 48 8d 0d 57 45 e5 00        	leaq	0xe54557(%rip), %rcx    # 0x142ff64e8
1421a1f91: e8 3a 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1f96: eb 0c                       	jmp	0x1421a1fa4 <.text+0x21a0fa4>
1421a1f98: 48 8d 0d 49 45 e5 00        	leaq	0xe54549(%rip), %rcx    # 0x142ff64e8
1421a1f9f: e8 2c 61 73 00              	callq	0x1428d80d0 <.text+0x28d70d0>
1421a1fa4: 0f 0b                       	ud2
1421a1fa6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1421a1fb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a1fb5: 55                          	pushq	%rbp
1421a1fb6: 41 57                       	pushq	%r15
1421a1fb8: 41 56                       	pushq	%r14
1421a1fba: 41 55                       	pushq	%r13
1421a1fbc: 41 54                       	pushq	%r12
1421a1fbe: 56                          	pushq	%rsi
1421a1fbf: 57                          	pushq	%rdi
1421a1fc0: 53                          	pushq	%rbx
1421a1fc1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a1fc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a1fcc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a1fd2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a1fd8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a1fdd: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a1fe3: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1421a1fe7: e8 a4 fb 6d fe              	callq	0x140881b90 <.text+0x880b90>
1421a1fec: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1421a1ff1: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a1ff6: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a1ffc: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2002: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2006: 5b                          	popq	%rbx
1421a2007: 5f                          	popq	%rdi
1421a2008: 5e                          	popq	%rsi
1421a2009: 41 5c                       	popq	%r12
1421a200b: 41 5d                       	popq	%r13
1421a200d: 41 5e                       	popq	%r14
1421a200f: 41 5f                       	popq	%r15
1421a2011: 5d                          	popq	%rbp
1421a2012: c3                          	retq
1421a2013: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1421a2020: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2025: 55                          	pushq	%rbp
1421a2026: 41 57                       	pushq	%r15
1421a2028: 41 56                       	pushq	%r14
1421a202a: 41 55                       	pushq	%r13
1421a202c: 41 54                       	pushq	%r12
1421a202e: 56                          	pushq	%rsi
1421a202f: 57                          	pushq	%rdi
1421a2030: 53                          	pushq	%rbx
1421a2031: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2035: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a203c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2042: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2048: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a204d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2053: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a205a: e8 e1 45 43 fe              	callq	0x1405d6640 <.text+0x5d5640>
1421a205f: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2065: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a206a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a2070: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2076: 48 83 c4 68                 	addq	$0x68, %rsp
1421a207a: 5b                          	popq	%rbx
1421a207b: 5f                          	popq	%rdi
1421a207c: 5e                          	popq	%rsi
1421a207d: 41 5c                       	popq	%r12
1421a207f: 41 5d                       	popq	%r13
1421a2081: 41 5e                       	popq	%r14
1421a2083: 41 5f                       	popq	%r15
1421a2085: 5d                          	popq	%rbp
1421a2086: c3                          	retq
1421a2087: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a2090: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2095: 55                          	pushq	%rbp
1421a2096: 41 57                       	pushq	%r15
1421a2098: 41 56                       	pushq	%r14
1421a209a: 41 55                       	pushq	%r13
1421a209c: 41 54                       	pushq	%r12
1421a209e: 56                          	pushq	%rsi
1421a209f: 57                          	pushq	%rdi
1421a20a0: 53                          	pushq	%rbx
1421a20a1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a20a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a20ac: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a20b2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a20b8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a20bd: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a20c3: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a20ca: e8 c1 29 46 fe              	callq	0x140604a90 <.text+0x603a90>
1421a20cf: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1421a20d4: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a20d9: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a20df: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a20e5: 48 83 c4 68                 	addq	$0x68, %rsp
1421a20e9: 5b                          	popq	%rbx
1421a20ea: 5f                          	popq	%rdi
1421a20eb: 5e                          	popq	%rsi
1421a20ec: 41 5c                       	popq	%r12
1421a20ee: 41 5d                       	popq	%r13
1421a20f0: 41 5e                       	popq	%r14
1421a20f2: 41 5f                       	popq	%r15
1421a20f4: 5d                          	popq	%rbp
1421a20f5: c3                          	retq
1421a20f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1421a2100: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2105: 55                          	pushq	%rbp
1421a2106: 41 57                       	pushq	%r15
1421a2108: 41 56                       	pushq	%r14
1421a210a: 41 55                       	pushq	%r13
1421a210c: 41 54                       	pushq	%r12
1421a210e: 56                          	pushq	%rsi
1421a210f: 57                          	pushq	%rdi
1421a2110: 53                          	pushq	%rbx
1421a2111: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2115: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a211c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2122: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2128: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a212d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2133: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1421a2137: e8 54 fa 6d fe              	callq	0x140881b90 <.text+0x880b90>
1421a213c: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1421a2141: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2146: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a214c: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2152: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2156: 5b                          	popq	%rbx
1421a2157: 5f                          	popq	%rdi
1421a2158: 5e                          	popq	%rsi
1421a2159: 41 5c                       	popq	%r12
1421a215b: 41 5d                       	popq	%r13
1421a215d: 41 5e                       	popq	%r14
1421a215f: 41 5f                       	popq	%r15
1421a2161: 5d                          	popq	%rbp
1421a2162: c3                          	retq
1421a2163: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1421a2170: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2175: 55                          	pushq	%rbp
1421a2176: 41 57                       	pushq	%r15
1421a2178: 41 56                       	pushq	%r14
1421a217a: 41 55                       	pushq	%r13
1421a217c: 41 54                       	pushq	%r12
1421a217e: 56                          	pushq	%rsi
1421a217f: 57                          	pushq	%rdi
1421a2180: 53                          	pushq	%rbx
1421a2181: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2185: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a218c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2192: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2198: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a219d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a21a3: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a21aa: e8 91 44 43 fe              	callq	0x1405d6640 <.text+0x5d5640>
1421a21af: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a21b5: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a21ba: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a21c0: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a21c6: 48 83 c4 68                 	addq	$0x68, %rsp
1421a21ca: 5b                          	popq	%rbx
1421a21cb: 5f                          	popq	%rdi
1421a21cc: 5e                          	popq	%rsi
1421a21cd: 41 5c                       	popq	%r12
1421a21cf: 41 5d                       	popq	%r13
1421a21d1: 41 5e                       	popq	%r14
1421a21d3: 41 5f                       	popq	%r15
1421a21d5: 5d                          	popq	%rbp
1421a21d6: c3                          	retq
1421a21d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a21e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a21e5: 55                          	pushq	%rbp
1421a21e6: 41 57                       	pushq	%r15
1421a21e8: 41 56                       	pushq	%r14
1421a21ea: 41 55                       	pushq	%r13
1421a21ec: 41 54                       	pushq	%r12
1421a21ee: 56                          	pushq	%rsi
1421a21ef: 57                          	pushq	%rdi
1421a21f0: 53                          	pushq	%rbx
1421a21f1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a21f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a21fc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2202: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2208: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a220d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2213: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a221a: e8 71 28 46 fe              	callq	0x140604a90 <.text+0x603a90>
1421a221f: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1421a2224: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2229: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a222f: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2235: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2239: 5b                          	popq	%rbx
1421a223a: 5f                          	popq	%rdi
1421a223b: 5e                          	popq	%rsi
1421a223c: 41 5c                       	popq	%r12
1421a223e: 41 5d                       	popq	%r13
1421a2240: 41 5e                       	popq	%r14
1421a2242: 41 5f                       	popq	%r15
1421a2244: 5d                          	popq	%rbp
1421a2245: c3                          	retq
1421a2246: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1421a2250: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2255: 55                          	pushq	%rbp
1421a2256: 41 57                       	pushq	%r15
1421a2258: 41 56                       	pushq	%r14
1421a225a: 41 55                       	pushq	%r13
1421a225c: 41 54                       	pushq	%r12
1421a225e: 56                          	pushq	%rsi
1421a225f: 57                          	pushq	%rdi
1421a2260: 53                          	pushq	%rbx
1421a2261: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2265: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a226c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2272: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2278: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a227d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2283: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1421a2287: e8 04 f9 6d fe              	callq	0x140881b90 <.text+0x880b90>
1421a228c: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1421a2291: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2296: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a229c: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a22a2: 48 83 c4 68                 	addq	$0x68, %rsp
1421a22a6: 5b                          	popq	%rbx
1421a22a7: 5f                          	popq	%rdi
1421a22a8: 5e                          	popq	%rsi
1421a22a9: 41 5c                       	popq	%r12
1421a22ab: 41 5d                       	popq	%r13
1421a22ad: 41 5e                       	popq	%r14
1421a22af: 41 5f                       	popq	%r15
1421a22b1: 5d                          	popq	%rbp
1421a22b2: c3                          	retq
1421a22b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1421a22c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a22c5: 55                          	pushq	%rbp
1421a22c6: 41 57                       	pushq	%r15
1421a22c8: 41 56                       	pushq	%r14
1421a22ca: 41 55                       	pushq	%r13
1421a22cc: 41 54                       	pushq	%r12
1421a22ce: 56                          	pushq	%rsi
1421a22cf: 57                          	pushq	%rdi
1421a22d0: 53                          	pushq	%rbx
1421a22d1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a22d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a22dc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a22e2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a22e8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a22ed: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a22f3: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a22fa: e8 41 43 43 fe              	callq	0x1405d6640 <.text+0x5d5640>
1421a22ff: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2305: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a230a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a2310: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2316: 48 83 c4 68                 	addq	$0x68, %rsp
1421a231a: 5b                          	popq	%rbx
1421a231b: 5f                          	popq	%rdi
1421a231c: 5e                          	popq	%rsi
1421a231d: 41 5c                       	popq	%r12
1421a231f: 41 5d                       	popq	%r13
1421a2321: 41 5e                       	popq	%r14
1421a2323: 41 5f                       	popq	%r15
1421a2325: 5d                          	popq	%rbp
1421a2326: c3                          	retq
1421a2327: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a2330: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2335: 55                          	pushq	%rbp
1421a2336: 41 57                       	pushq	%r15
1421a2338: 41 56                       	pushq	%r14
1421a233a: 41 55                       	pushq	%r13
1421a233c: 41 54                       	pushq	%r12
1421a233e: 56                          	pushq	%rsi
1421a233f: 57                          	pushq	%rdi
1421a2340: 53                          	pushq	%rbx
1421a2341: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2345: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a234c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2352: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2358: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a235d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2363: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a236a: e8 21 f8 6d fe              	callq	0x140881b90 <.text+0x880b90>
1421a236f: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2375: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a237a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a2380: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2386: 48 83 c4 68                 	addq	$0x68, %rsp
1421a238a: 5b                          	popq	%rbx
1421a238b: 5f                          	popq	%rdi
1421a238c: 5e                          	popq	%rsi
1421a238d: 41 5c                       	popq	%r12
1421a238f: 41 5d                       	popq	%r13
1421a2391: 41 5e                       	popq	%r14
1421a2393: 41 5f                       	popq	%r15
1421a2395: 5d                          	popq	%rbp
1421a2396: c3                          	retq
1421a2397: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a23a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a23a5: 55                          	pushq	%rbp
1421a23a6: 41 57                       	pushq	%r15
1421a23a8: 41 56                       	pushq	%r14
1421a23aa: 41 55                       	pushq	%r13
1421a23ac: 41 54                       	pushq	%r12
1421a23ae: 56                          	pushq	%rsi
1421a23af: 57                          	pushq	%rdi
1421a23b0: 53                          	pushq	%rbx
1421a23b1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a23b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a23bc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a23c2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a23c8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a23cd: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a23d3: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a23da: e8 61 42 43 fe              	callq	0x1405d6640 <.text+0x5d5640>
1421a23df: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a23e5: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a23ea: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a23f0: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a23f6: 48 83 c4 68                 	addq	$0x68, %rsp
1421a23fa: 5b                          	popq	%rbx
1421a23fb: 5f                          	popq	%rdi
1421a23fc: 5e                          	popq	%rsi
1421a23fd: 41 5c                       	popq	%r12
1421a23ff: 41 5d                       	popq	%r13
1421a2401: 41 5e                       	popq	%r14
1421a2403: 41 5f                       	popq	%r15
1421a2405: 5d                          	popq	%rbp
1421a2406: c3                          	retq
1421a2407: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a2410: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2415: 55                          	pushq	%rbp
1421a2416: 41 57                       	pushq	%r15
1421a2418: 41 56                       	pushq	%r14
1421a241a: 41 55                       	pushq	%r13
1421a241c: 41 54                       	pushq	%r12
1421a241e: 56                          	pushq	%rsi
1421a241f: 57                          	pushq	%rdi
1421a2420: 53                          	pushq	%rbx
1421a2421: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2425: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a242c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2432: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2438: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a243d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2443: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
1421a2447: 48 85 d2                    	testq	%rdx, %rdx
1421a244a: 74 0f                       	je	0x1421a245b <.text+0x21a145b>
1421a244c: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
1421a2450: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421a2456: e8 c5 17 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a245b: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1421a2460: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2465: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a246b: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2471: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2475: 5b                          	popq	%rbx
1421a2476: 5f                          	popq	%rdi
1421a2477: 5e                          	popq	%rsi
1421a2478: 41 5c                       	popq	%r12
1421a247a: 41 5d                       	popq	%r13
1421a247c: 41 5e                       	popq	%r14
1421a247e: 41 5f                       	popq	%r15
1421a2480: 5d                          	popq	%rbp
1421a2481: c3                          	retq
1421a2482: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1421a2490: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2495: 55                          	pushq	%rbp
1421a2496: 41 57                       	pushq	%r15
1421a2498: 41 56                       	pushq	%r14
1421a249a: 41 55                       	pushq	%r13
1421a249c: 41 54                       	pushq	%r12
1421a249e: 56                          	pushq	%rsi
1421a249f: 57                          	pushq	%rdi
1421a24a0: 53                          	pushq	%rbx
1421a24a1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a24a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a24ac: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a24b2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a24b8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a24bd: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a24c3: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1421a24c7: e8 c4 25 46 fe              	callq	0x140604a90 <.text+0x603a90>
1421a24cc: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a24d2: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a24d7: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a24dd: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a24e3: 48 83 c4 68                 	addq	$0x68, %rsp
1421a24e7: 5b                          	popq	%rbx
1421a24e8: 5f                          	popq	%rdi
1421a24e9: 5e                          	popq	%rsi
1421a24ea: 41 5c                       	popq	%r12
1421a24ec: 41 5d                       	popq	%r13
1421a24ee: 41 5e                       	popq	%r14
1421a24f0: 41 5f                       	popq	%r15
1421a24f2: 5d                          	popq	%rbp
1421a24f3: c3                          	retq
1421a24f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1421a2500: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2505: 55                          	pushq	%rbp
1421a2506: 41 57                       	pushq	%r15
1421a2508: 41 56                       	pushq	%r14
1421a250a: 41 55                       	pushq	%r13
1421a250c: 41 54                       	pushq	%r12
1421a250e: 56                          	pushq	%rsi
1421a250f: 57                          	pushq	%rdi
1421a2510: 53                          	pushq	%rbx
1421a2511: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2515: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a251c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2522: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2528: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a252d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2533: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1421a253a: e8 51 f6 6d fe              	callq	0x140881b90 <.text+0x880b90>
1421a253f: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2545: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a254a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a2550: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2556: 48 83 c4 68                 	addq	$0x68, %rsp
1421a255a: 5b                          	popq	%rbx
1421a255b: 5f                          	popq	%rdi
1421a255c: 5e                          	popq	%rsi
1421a255d: 41 5c                       	popq	%r12
1421a255f: 41 5d                       	popq	%r13
1421a2561: 41 5e                       	popq	%r14
1421a2563: 41 5f                       	popq	%r15
1421a2565: 5d                          	popq	%rbp
1421a2566: c3                          	retq
1421a2567: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a2570: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2575: 55                          	pushq	%rbp
1421a2576: 41 57                       	pushq	%r15
1421a2578: 41 56                       	pushq	%r14
1421a257a: 41 55                       	pushq	%r13
1421a257c: 41 54                       	pushq	%r12
1421a257e: 56                          	pushq	%rsi
1421a257f: 57                          	pushq	%rdi
1421a2580: 53                          	pushq	%rbx
1421a2581: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2585: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a258c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2592: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2598: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a259d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a25a3: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a25aa: e8 91 40 43 fe              	callq	0x1405d6640 <.text+0x5d5640>
1421a25af: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a25b5: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a25ba: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a25c0: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a25c6: 48 83 c4 68                 	addq	$0x68, %rsp
1421a25ca: 5b                          	popq	%rbx
1421a25cb: 5f                          	popq	%rdi
1421a25cc: 5e                          	popq	%rsi
1421a25cd: 41 5c                       	popq	%r12
1421a25cf: 41 5d                       	popq	%r13
1421a25d1: 41 5e                       	popq	%r14
1421a25d3: 41 5f                       	popq	%r15
1421a25d5: 5d                          	popq	%rbp
1421a25d6: c3                          	retq
1421a25d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a25e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a25e5: 55                          	pushq	%rbp
1421a25e6: 41 57                       	pushq	%r15
1421a25e8: 41 56                       	pushq	%r14
1421a25ea: 41 55                       	pushq	%r13
1421a25ec: 41 54                       	pushq	%r12
1421a25ee: 56                          	pushq	%rsi
1421a25ef: 57                          	pushq	%rdi
1421a25f0: 53                          	pushq	%rbx
1421a25f1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a25f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a25fc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2602: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2608: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a260d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2613: 48 83 bd 38 02 00 00 00     	cmpq	$0x0, 0x238(%rbp)
1421a261b: 74 19                       	je	0x1421a2636 <.text+0x21a1636>
1421a261d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421a2623: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1421a262a: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1421a2631: e8 ea 15 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a2636: c6 85 47 02 00 00 00        	movb	$0x0, 0x247(%rbp)
1421a263d: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2643: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2648: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a264e: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2654: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2658: 5b                          	popq	%rbx
1421a2659: 5f                          	popq	%rdi
1421a265a: 5e                          	popq	%rsi
1421a265b: 41 5c                       	popq	%r12
1421a265d: 41 5d                       	popq	%r13
1421a265f: 41 5e                       	popq	%r14
1421a2661: 41 5f                       	popq	%r15
1421a2663: 5d                          	popq	%rbp
1421a2664: c3                          	retq
1421a2665: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1421a2670: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2675: 55                          	pushq	%rbp
1421a2676: 41 57                       	pushq	%r15
1421a2678: 41 56                       	pushq	%r14
1421a267a: 41 55                       	pushq	%r13
1421a267c: 41 54                       	pushq	%r12
1421a267e: 56                          	pushq	%rsi
1421a267f: 57                          	pushq	%rdi
1421a2680: 53                          	pushq	%rbx
1421a2681: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2685: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a268c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2692: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2698: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a269d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a26a3: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a26aa: e8 e1 23 46 fe              	callq	0x140604a90 <.text+0x603a90>
1421a26af: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a26b5: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a26ba: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a26c0: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a26c6: 48 83 c4 68                 	addq	$0x68, %rsp
1421a26ca: 5b                          	popq	%rbx
1421a26cb: 5f                          	popq	%rdi
1421a26cc: 5e                          	popq	%rsi
1421a26cd: 41 5c                       	popq	%r12
1421a26cf: 41 5d                       	popq	%r13
1421a26d1: 41 5e                       	popq	%r14
1421a26d3: 41 5f                       	popq	%r15
1421a26d5: 5d                          	popq	%rbp
1421a26d6: c3                          	retq
1421a26d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a26e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a26e5: 55                          	pushq	%rbp
1421a26e6: 41 57                       	pushq	%r15
1421a26e8: 41 56                       	pushq	%r14
1421a26ea: 41 55                       	pushq	%r13
1421a26ec: 41 54                       	pushq	%r12
1421a26ee: 56                          	pushq	%rsi
1421a26ef: 57                          	pushq	%rdi
1421a26f0: 53                          	pushq	%rbx
1421a26f1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a26f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a26fc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2702: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2708: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a270d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2713: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1421a271a: e8 71 f4 6d fe              	callq	0x140881b90 <.text+0x880b90>
1421a271f: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2725: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a272a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a2730: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2736: 48 83 c4 68                 	addq	$0x68, %rsp
1421a273a: 5b                          	popq	%rbx
1421a273b: 5f                          	popq	%rdi
1421a273c: 5e                          	popq	%rsi
1421a273d: 41 5c                       	popq	%r12
1421a273f: 41 5d                       	popq	%r13
1421a2741: 41 5e                       	popq	%r14
1421a2743: 41 5f                       	popq	%r15
1421a2745: 5d                          	popq	%rbp
1421a2746: c3                          	retq
1421a2747: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a2750: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2755: 55                          	pushq	%rbp
1421a2756: 41 57                       	pushq	%r15
1421a2758: 41 56                       	pushq	%r14
1421a275a: 41 55                       	pushq	%r13
1421a275c: 41 54                       	pushq	%r12
1421a275e: 56                          	pushq	%rsi
1421a275f: 57                          	pushq	%rdi
1421a2760: 53                          	pushq	%rbx
1421a2761: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2765: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a276c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2772: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2778: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a277d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2783: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a278a: e8 b1 3e 43 fe              	callq	0x1405d6640 <.text+0x5d5640>
1421a278f: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2795: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a279a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a27a0: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a27a6: 48 83 c4 68                 	addq	$0x68, %rsp
1421a27aa: 5b                          	popq	%rbx
1421a27ab: 5f                          	popq	%rdi
1421a27ac: 5e                          	popq	%rsi
1421a27ad: 41 5c                       	popq	%r12
1421a27af: 41 5d                       	popq	%r13
1421a27b1: 41 5e                       	popq	%r14
1421a27b3: 41 5f                       	popq	%r15
1421a27b5: 5d                          	popq	%rbp
1421a27b6: c3                          	retq
1421a27b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a27c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a27c5: 55                          	pushq	%rbp
1421a27c6: 41 57                       	pushq	%r15
1421a27c8: 41 56                       	pushq	%r14
1421a27ca: 41 55                       	pushq	%r13
1421a27cc: 41 54                       	pushq	%r12
1421a27ce: 56                          	pushq	%rsi
1421a27cf: 57                          	pushq	%rdi
1421a27d0: 53                          	pushq	%rbx
1421a27d1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a27d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a27dc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a27e2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a27e8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a27ed: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a27f3: 48 83 bd 38 02 00 00 00     	cmpq	$0x0, 0x238(%rbp)
1421a27fb: 74 19                       	je	0x1421a2816 <.text+0x21a1816>
1421a27fd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421a2803: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1421a280a: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1421a2811: e8 0a 14 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a2816: c6 85 47 02 00 00 00        	movb	$0x0, 0x247(%rbp)
1421a281d: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2823: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2828: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a282e: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2834: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2838: 5b                          	popq	%rbx
1421a2839: 5f                          	popq	%rdi
1421a283a: 5e                          	popq	%rsi
1421a283b: 41 5c                       	popq	%r12
1421a283d: 41 5d                       	popq	%r13
1421a283f: 41 5e                       	popq	%r14
1421a2841: 41 5f                       	popq	%r15
1421a2843: 5d                          	popq	%rbp
1421a2844: c3                          	retq
1421a2845: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1421a2850: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2855: 55                          	pushq	%rbp
1421a2856: 41 57                       	pushq	%r15
1421a2858: 41 56                       	pushq	%r14
1421a285a: 41 55                       	pushq	%r13
1421a285c: 41 54                       	pushq	%r12
1421a285e: 56                          	pushq	%rsi
1421a285f: 57                          	pushq	%rdi
1421a2860: 53                          	pushq	%rbx
1421a2861: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2865: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a286c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2872: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2878: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a287d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2883: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a288a: e8 01 22 46 fe              	callq	0x140604a90 <.text+0x603a90>
1421a288f: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2895: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a289a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a28a0: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a28a6: 48 83 c4 68                 	addq	$0x68, %rsp
1421a28aa: 5b                          	popq	%rbx
1421a28ab: 5f                          	popq	%rdi
1421a28ac: 5e                          	popq	%rsi
1421a28ad: 41 5c                       	popq	%r12
1421a28af: 41 5d                       	popq	%r13
1421a28b1: 41 5e                       	popq	%r14
1421a28b3: 41 5f                       	popq	%r15
1421a28b5: 5d                          	popq	%rbp
1421a28b6: c3                          	retq
1421a28b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a28c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a28c5: 55                          	pushq	%rbp
1421a28c6: 41 57                       	pushq	%r15
1421a28c8: 41 56                       	pushq	%r14
1421a28ca: 41 55                       	pushq	%r13
1421a28cc: 41 54                       	pushq	%r12
1421a28ce: 56                          	pushq	%rsi
1421a28cf: 57                          	pushq	%rdi
1421a28d0: 53                          	pushq	%rbx
1421a28d1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a28d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a28dc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a28e2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a28e8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a28ed: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a28f3: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1421a28fa: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1421a2901: e8 8a f2 6d fe              	callq	0x140881b90 <.text+0x880b90>
1421a2906: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1421a290d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1421a2912: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2917: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a291d: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2923: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2927: 5b                          	popq	%rbx
1421a2928: 5f                          	popq	%rdi
1421a2929: 5e                          	popq	%rsi
1421a292a: 41 5c                       	popq	%r12
1421a292c: 41 5d                       	popq	%r13
1421a292e: 41 5e                       	popq	%r14
1421a2930: 41 5f                       	popq	%r15
1421a2932: 5d                          	popq	%rbp
1421a2933: c3                          	retq
1421a2934: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1421a2940: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2945: 55                          	pushq	%rbp
1421a2946: 41 57                       	pushq	%r15
1421a2948: 41 56                       	pushq	%r14
1421a294a: 41 55                       	pushq	%r13
1421a294c: 41 54                       	pushq	%r12
1421a294e: 56                          	pushq	%rsi
1421a294f: 57                          	pushq	%rdi
1421a2950: 53                          	pushq	%rbx
1421a2951: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2955: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a295c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2962: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2968: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a296d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2973: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1421a297a: e8 c1 3c 43 fe              	callq	0x1405d6640 <.text+0x5d5640>
1421a297f: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2985: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a298a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a2990: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2996: 48 83 c4 68                 	addq	$0x68, %rsp
1421a299a: 5b                          	popq	%rbx
1421a299b: 5f                          	popq	%rdi
1421a299c: 5e                          	popq	%rsi
1421a299d: 41 5c                       	popq	%r12
1421a299f: 41 5d                       	popq	%r13
1421a29a1: 41 5e                       	popq	%r14
1421a29a3: 41 5f                       	popq	%r15
1421a29a5: 5d                          	popq	%rbp
1421a29a6: c3                          	retq
1421a29a7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a29b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a29b5: 55                          	pushq	%rbp
1421a29b6: 41 57                       	pushq	%r15
1421a29b8: 41 56                       	pushq	%r14
1421a29ba: 41 55                       	pushq	%r13
1421a29bc: 41 54                       	pushq	%r12
1421a29be: 56                          	pushq	%rsi
1421a29bf: 57                          	pushq	%rdi
1421a29c0: 53                          	pushq	%rbx
1421a29c1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a29c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a29cc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a29d2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a29d8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a29dd: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a29e3: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a29ea: e8 a1 20 46 fe              	callq	0x140604a90 <.text+0x603a90>
1421a29ef: c6 85 47 02 00 00 01        	movb	$0x1, 0x247(%rbp)
1421a29f6: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1421a29fb: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2a00: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a2a06: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2a0c: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2a10: 5b                          	popq	%rbx
1421a2a11: 5f                          	popq	%rdi
1421a2a12: 5e                          	popq	%rsi
1421a2a13: 41 5c                       	popq	%r12
1421a2a15: 41 5d                       	popq	%r13
1421a2a17: 41 5e                       	popq	%r14
1421a2a19: 41 5f                       	popq	%r15
1421a2a1b: 5d                          	popq	%rbp
1421a2a1c: c3                          	retq
1421a2a1d: 0f 1f 00                    	nopl	(%rax)
1421a2a20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2a25: 55                          	pushq	%rbp
1421a2a26: 41 57                       	pushq	%r15
1421a2a28: 41 56                       	pushq	%r14
1421a2a2a: 41 55                       	pushq	%r13
1421a2a2c: 41 54                       	pushq	%r12
1421a2a2e: 56                          	pushq	%rsi
1421a2a2f: 57                          	pushq	%rdi
1421a2a30: 53                          	pushq	%rbx
1421a2a31: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2a35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a2a3c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2a42: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2a48: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a2a4d: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2a53: 48 83 bd 38 02 00 00 00     	cmpq	$0x0, 0x238(%rbp)
1421a2a5b: 0f 95 c0                    	setne	%al
1421a2a5e: 84 85 47 02 00 00           	testb	%al, 0x247(%rbp)
1421a2a64: 74 19                       	je	0x1421a2a7f <.text+0x21a1a7f>
1421a2a66: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421a2a6c: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1421a2a73: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1421a2a7a: e8 a1 11 47 fe              	callq	0x140613c20 <.text+0x612c20>
1421a2a7f: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1421a2a85: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2a8a: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a2a90: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2a96: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2a9a: 5b                          	popq	%rbx
1421a2a9b: 5f                          	popq	%rdi
1421a2a9c: 5e                          	popq	%rsi
1421a2a9d: 41 5c                       	popq	%r12
1421a2a9f: 41 5d                       	popq	%r13
1421a2aa1: 41 5e                       	popq	%r14
1421a2aa3: 41 5f                       	popq	%r15
1421a2aa5: 5d                          	popq	%rbp
1421a2aa6: c3                          	retq
1421a2aa7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1421a2ab0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1421a2ab5: 55                          	pushq	%rbp
1421a2ab6: 41 57                       	pushq	%r15
1421a2ab8: 41 56                       	pushq	%r14
1421a2aba: 41 55                       	pushq	%r13
1421a2abc: 41 54                       	pushq	%r12
1421a2abe: 56                          	pushq	%rsi
1421a2abf: 57                          	pushq	%rdi
1421a2ac0: 53                          	pushq	%rbx
1421a2ac1: 48 83 ec 68                 	subq	$0x68, %rsp
1421a2ac5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1421a2acc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1421a2ad2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1421a2ad8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1421a2add: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
1421a2ae3: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1421a2aea: e8 a1 1f 46 fe              	callq	0x140604a90 <.text+0x603a90>
1421a2aef: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1421a2af4: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1421a2af9: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1421a2aff: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1421a2b05: 48 83 c4 68                 	addq	$0x68, %rsp
1421a2b09: 5b                          	popq	%rbx
1421a2b0a: 5f                          	popq	%rdi
1421a2b0b: 5e                          	popq	%rsi
1421a2b0c: 41 5c                       	popq	%r12
1421a2b0e: 41 5d                       	popq	%r13
1421a2b10: 41 5e                       	popq	%r14
1421a2b12: 41 5f                       	popq	%r15
1421a2b14: 5d                          	popq	%rbp
1421a2b15: c3                          	retq
