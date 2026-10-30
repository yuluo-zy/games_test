
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407c5650: 55                          	pushq	%rbp
1407c5651: 41 57                       	pushq	%r15
1407c5653: 41 56                       	pushq	%r14
1407c5655: 41 55                       	pushq	%r13
1407c5657: 41 54                       	pushq	%r12
1407c5659: 56                          	pushq	%rsi
1407c565a: 57                          	pushq	%rdi
1407c565b: 53                          	pushq	%rbx
1407c565c: 48 81 ec 68 03 00 00        	subq	$0x368, %rsp            # imm = 0x368
1407c5663: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407c566b: 44 0f 29 bd d0 02 00 00     	movaps	%xmm15, 0x2d0(%rbp)
1407c5673: 44 0f 29 b5 c0 02 00 00     	movaps	%xmm14, 0x2c0(%rbp)
1407c567b: 44 0f 29 ad b0 02 00 00     	movaps	%xmm13, 0x2b0(%rbp)
1407c5683: 44 0f 29 a5 a0 02 00 00     	movaps	%xmm12, 0x2a0(%rbp)
1407c568b: 44 0f 29 9d 90 02 00 00     	movaps	%xmm11, 0x290(%rbp)
1407c5693: 44 0f 29 95 80 02 00 00     	movaps	%xmm10, 0x280(%rbp)
1407c569b: 44 0f 29 8d 70 02 00 00     	movaps	%xmm9, 0x270(%rbp)
1407c56a3: 44 0f 29 85 60 02 00 00     	movaps	%xmm8, 0x260(%rbp)
1407c56ab: 0f 29 bd 50 02 00 00        	movaps	%xmm7, 0x250(%rbp)
1407c56b2: 66 0f 7f b5 40 02 00 00     	movdqa	%xmm6, 0x240(%rbp)
1407c56ba: 48 c7 85 38 02 00 00 fe ff ff ff    	movq	$-0x2, 0x238(%rbp)
1407c56c5: 4c 89 ce                    	movq	%r9, %rsi
1407c56c8: 4d 89 c5                    	movq	%r8, %r13
1407c56cb: 48 89 d3                    	movq	%rdx, %rbx
1407c56ce: 49 89 ce                    	movq	%rcx, %r14
1407c56d1: e8 2a 13 45 00              	callq	0x140c16a00 <.text+0xc15a00>
1407c56d6: 88 85 26 02 00 00           	movb	%al, 0x226(%rbp)
1407c56dc: 84 c0                       	testb	%al, %al
1407c56de: 0f 84 21 02 00 00           	je	0x1407c5905 <.text+0x7c4905>
1407c56e4: bf 4a 00 00 00              	movl	$0x4a, %edi
1407c56e9: b9 4a 00 00 00              	movl	$0x4a, %ecx
1407c56ee: e8 dd fa ff ff              	callq	0x1407c51d0 <.text+0x7c41d0>
1407c56f3: 4c 8d 25 7e 13 2c 02        	leaq	0x22c137e(%rip), %r12   # 0x142a86a78
1407c56fa: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c56fe: 0f 85 a0 00 00 00           	jne	0x1407c57a4 <.text+0x7c47a4>
1407c5704: 49 89 d1                    	movq	%rdx, %r9
1407c5707: 48 85 d2                    	testq	%rdx, %rdx
1407c570a: 74 36                       	je	0x1407c5742 <.text+0x7c4742>
1407c570c: 49 83 f9 4a                 	cmpq	$0x4a, %r9
1407c5710: 73 2e                       	jae	0x1407c5740 <.text+0x7c4740>
1407c5712: 48 8d 05 5f 13 2c 02        	leaq	0x22c135f(%rip), %rax   # 0x142a86a78
1407c5719: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1407c571e: 7f 22                       	jg	0x1407c5742 <.text+0x7c4742>
1407c5720: 48 8d 05 19 14 2c 02        	leaq	0x22c1419(%rip), %rax   # 0x142a86b40
1407c5727: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c572c: 48 8d 0d 45 13 2c 02        	leaq	0x22c1345(%rip), %rcx   # 0x142a86a78
1407c5733: ba 4a 00 00 00              	movl	$0x4a, %edx
1407c5738: 45 31 c0                    	xorl	%r8d, %r8d
1407c573b: e8 00 42 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c5740: 75 de                       	jne	0x1407c5720 <.text+0x7c4720>
1407c5742: 4c 89 c9                    	movq	%r9, %rcx
1407c5745: e8 86 fa ff ff              	callq	0x1407c51d0 <.text+0x7c41d0>
1407c574a: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c574e: 75 54                       	jne	0x1407c57a4 <.text+0x7c47a4>
1407c5750: 49 89 d0                    	movq	%rdx, %r8
1407c5753: 49 83 c0 02                 	addq	$0x2, %r8
1407c5757: 74 39                       	je	0x1407c5792 <.text+0x7c4792>
1407c5759: 49 83 f8 4a                 	cmpq	$0x4a, %r8
1407c575d: 73 31                       	jae	0x1407c5790 <.text+0x7c4790>
1407c575f: 48 8d 05 12 13 2c 02        	leaq	0x22c1312(%rip), %rax   # 0x142a86a78
1407c5766: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1407c576b: 7f 25                       	jg	0x1407c5792 <.text+0x7c4792>
1407c576d: 48 8d 05 e4 13 2c 02        	leaq	0x22c13e4(%rip), %rax   # 0x142a86b58
1407c5774: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c5779: 48 8d 0d f8 12 2c 02        	leaq	0x22c12f8(%rip), %rcx   # 0x142a86a78
1407c5780: ba 4a 00 00 00              	movl	$0x4a, %edx
1407c5785: 41 b9 4a 00 00 00           	movl	$0x4a, %r9d
1407c578b: e8 b0 41 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c5790: 75 db                       	jne	0x1407c576d <.text+0x7c476d>
1407c5792: bf 48 00 00 00              	movl	$0x48, %edi
1407c5797: 48 29 d7                    	subq	%rdx, %rdi
1407c579a: 4c 8d 25 d7 12 2c 02        	leaq	0x22c12d7(%rip), %r12   # 0x142a86a78
1407c57a1: 4d 01 c4                    	addq	%r8, %r12
1407c57a4: 4c 89 ad e0 01 00 00        	movq	%r13, 0x1e0(%rbp)
1407c57ab: 4c 8d 05 42 14 2c 02        	leaq	0x22c1442(%rip), %r8    # 0x142a86bf4
1407c57b2: 4c 8d 2d fd 13 2c 02        	leaq	0x22c13fd(%rip), %r13   # 0x142a86bb6
1407c57b9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407c57c0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407c57c5: 85 c0                       	testl	%eax, %eax
1407c57c7: 78 17                       	js	0x1407c57e0 <.text+0x7c47e0>
1407c57c9: 49 ff c8                    	decq	%r8
1407c57cc: 83 f8 5c                    	cmpl	$0x5c, %eax
1407c57cf: 75 63                       	jne	0x1407c5834 <.text+0x7c4834>
1407c57d1: eb 73                       	jmp	0x1407c5846 <.text+0x7c4846>
1407c57d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407c57e0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407c57e5: 80 f9 c0                    	cmpb	$-0x40, %cl
1407c57e8: 7d 23                       	jge	0x1407c580d <.text+0x7c480d>
1407c57ea: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407c57ef: 80 fa c0                    	cmpb	$-0x40, %dl
1407c57f2: 7d 22                       	jge	0x1407c5816 <.text+0x7c4816>
1407c57f4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407c57f9: 49 83 c0 fc                 	addq	$-0x4, %r8
1407c57fd: 41 83 e1 07                 	andl	$0x7, %r9d
1407c5801: 41 c1 e1 06                 	shll	$0x6, %r9d
1407c5805: 83 e2 3f                    	andl	$0x3f, %edx
1407c5808: 44 09 ca                    	orl	%r9d, %edx
1407c580b: eb 10                       	jmp	0x1407c581d <.text+0x7c481d>
1407c580d: 49 83 c0 fe                 	addq	$-0x2, %r8
1407c5811: 83 e1 1f                    	andl	$0x1f, %ecx
1407c5814: eb 0f                       	jmp	0x1407c5825 <.text+0x7c4825>
1407c5816: 49 83 c0 fd                 	addq	$-0x3, %r8
1407c581a: 83 e2 0f                    	andl	$0xf, %edx
1407c581d: c1 e2 06                    	shll	$0x6, %edx
1407c5820: 83 e1 3f                    	andl	$0x3f, %ecx
1407c5823: 09 d1                       	orl	%edx, %ecx
1407c5825: c1 e1 06                    	shll	$0x6, %ecx
1407c5828: 24 3f                       	andb	$0x3f, %al
1407c582a: 0f b6 c0                    	movzbl	%al, %eax
1407c582d: 09 c8                       	orl	%ecx, %eax
1407c582f: 83 f8 5c                    	cmpl	$0x5c, %eax
1407c5832: 74 12                       	je	0x1407c5846 <.text+0x7c4846>
1407c5834: 83 f8 2f                    	cmpl	$0x2f, %eax
1407c5837: 74 0d                       	je	0x1407c5846 <.text+0x7c4846>
1407c5839: 4d 39 e8                    	cmpq	%r13, %r8
1407c583c: 75 82                       	jne	0x1407c57c0 <.text+0x7c47c0>
1407c583e: 41 bf 3e 00 00 00           	movl	$0x3e, %r15d
1407c5844: eb 4d                       	jmp	0x1407c5893 <.text+0x7c4893>
1407c5846: 4c 8d 2d 69 13 2c 02        	leaq	0x22c1369(%rip), %r13   # 0x142a86bb6
1407c584d: 4d 29 e8                    	subq	%r13, %r8
1407c5850: 49 ff c0                    	incq	%r8
1407c5853: 74 32                       	je	0x1407c5887 <.text+0x7c4887>
1407c5855: 49 83 f8 3e                 	cmpq	$0x3e, %r8
1407c5859: 73 2a                       	jae	0x1407c5885 <.text+0x7c4885>
1407c585b: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1407c5860: 7f 25                       	jg	0x1407c5887 <.text+0x7c4887>
1407c5862: 48 8d 05 b7 12 2c 02        	leaq	0x22c12b7(%rip), %rax   # 0x142a86b20
1407c5869: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c586e: 48 8d 0d 41 13 2c 02        	leaq	0x22c1341(%rip), %rcx   # 0x142a86bb6
1407c5875: ba 3e 00 00 00              	movl	$0x3e, %edx
1407c587a: 41 b9 3e 00 00 00           	movl	$0x3e, %r9d
1407c5880: e8 bb 40 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c5885: 75 db                       	jne	0x1407c5862 <.text+0x7c4862>
1407c5887: 41 bf 3e 00 00 00           	movl	$0x3e, %r15d
1407c588d: 4d 29 c7                    	subq	%r8, %r15
1407c5890: 4d 01 c5                    	addq	%r8, %r13
1407c5893: e8 98 28 45 00              	callq	0x140c18130 <.text+0xc17130>
1407c5898: 48 89 c1                    	movq	%rax, %rcx
1407c589b: 48 8b 00                    	movq	(%rax), %rax
1407c589e: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c58a2: 0f 85 d9 15 00 00           	jne	0x1407c6e81 <.text+0x7c5e81>
1407c58a8: 48 83 c1 08                 	addq	$0x8, %rcx
1407c58ac: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407c58b0: 0f 85 f2 16 00 00           	jne	0x1407c6fa8 <.text+0x7c5fa8>
1407c58b6: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407c58bd: 48 89 8d 90 01 00 00        	movq	%rcx, 0x190(%rbp)
1407c58c4: 48 83 c1 08                 	addq	$0x8, %rcx
1407c58c8: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1407c58cd: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1407c58d6: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407c58df: 4c 89 e2                    	movq	%r12, %rdx
1407c58e2: 49 89 f8                    	movq	%rdi, %r8
1407c58e5: 4d 89 e9                    	movq	%r13, %r9
1407c58e8: e8 53 14 45 00              	callq	0x140c16d40 <.text+0xc15d40>
1407c58ed: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
1407c58f4: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1407c58fb: 48 ff 00                    	incq	(%rax)
1407c58fe: 4c 8b ad e0 01 00 00        	movq	0x1e0(%rbp), %r13
1407c5905: 4c 89 f1                    	movq	%r14, %rcx
1407c5908: e8 63 11 36 00              	callq	0x140b26a70 <.text+0xb25a70>
1407c590d: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
1407c5914: 48 89 95 38 01 00 00        	movq	%rdx, 0x138(%rbp)
1407c591b: 48 8b 13                    	movq	(%rbx), %rdx
1407c591e: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c5925: e8 d6 57 25 00              	callq	0x140a1b100 <.text+0xa1a100>
1407c592a: 48 8b 85 58 03 00 00        	movq	0x358(%rbp), %rax
1407c5931: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
1407c5938: 48 8b 9d a0 01 00 00        	movq	0x1a0(%rbp), %rbx
1407c593f: 4c 8b b5 a8 01 00 00        	movq	0x1a8(%rbp), %r14
1407c5946: 44 0f b7 8d b8 01 00 00     	movzwl	0x1b8(%rbp), %r9d
1407c594e: 4c 8b bd c0 01 00 00        	movq	0x1c0(%rbp), %r15
1407c5955: 48 8b 0e                    	movq	(%rsi), %rcx
1407c5958: 4c 8b 02                    	movq	(%rdx), %r8
1407c595b: 4c 89 85 90 01 00 00        	movq	%r8, 0x190(%rbp)
1407c5962: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
1407c5966: 48 89 95 e0 01 00 00        	movq	%rdx, 0x1e0(%rbp)
1407c596d: 49 8b 55 00                 	movq	(%r13), %rdx
1407c5971: 48 89 55 e8                 	movq	%rdx, -0x18(%rbp)
1407c5975: 8b 50 1c                    	movl	0x1c(%rax), %edx
1407c5978: 89 95 e4 00 00 00           	movl	%edx, 0xe4(%rbp)
1407c597e: 48 8b 10                    	movq	(%rax), %rdx
1407c5981: 48 89 55 f8                 	movq	%rdx, -0x8(%rbp)
1407c5985: 48 8b 40 10                 	movq	0x10(%rax), %rax
1407c5989: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1407c598d: 48 8d 41 20                 	leaq	0x20(%rcx), %rax
1407c5991: 48 89 45 00                 	movq	%rax, (%rbp)
1407c5995: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1407c5999: f3 44 0f 10 15 0e 0e 2c 02  	movss	0x22c0e0e(%rip), %xmm10 # 0x142a867b0
1407c59a2: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
1407c59a6: f3 44 0f 10 35 39 47 19 02  	movss	0x2194739(%rip), %xmm14 # 0x14295a0e8
1407c59af: f3 44 0f 10 3d b8 b3 16 02  	movss	0x216b3b8(%rip), %xmm15 # 0x142930d70
1407c59b8: 44 0f 28 05 30 18 25 02     	movaps	0x2251830(%rip), %xmm8  # 0x142a171f0
1407c59c0: 44 0f 28 0d f8 0d 2c 02     	movaps	0x22c0df8(%rip), %xmm9  # 0x142a867c0
1407c59c8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407c59d0: 4d 85 ff                    	testq	%r15, %r15
1407c59d3: 0f 84 c4 13 00 00           	je	0x1407c6d9d <.text+0x7c5d9d>
1407c59d9: 66 45 85 c9                 	testw	%r9w, %r9w
1407c59dd: 74 21                       	je	0x1407c5a00 <.text+0x7c4a00>
1407c59df: 48 85 db                    	testq	%rbx, %rbx
1407c59e2: 0f 84 b5 13 00 00           	je	0x1407c6d9d <.text+0x7c5d9d>
1407c59e8: 41 8d 79 ff                 	leal	-0x1(%r9), %edi
1407c59ec: 44 21 cf                    	andl	%r9d, %edi
1407c59ef: eb 3d                       	jmp	0x1407c5a2e <.text+0x7c4a2e>
1407c59f1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407c5a00: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
1407c5a05: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
1407c5a0a: 48 81 c3 80 f6 ff ff        	addq	$-0x980, %rbx           # imm = 0xF680
1407c5a11: 49 83 c6 10                 	addq	$0x10, %r14
1407c5a15: 41 81 f9 ff ff 00 00        	cmpl	$0xffff, %r9d           # imm = 0xFFFF
1407c5a1c: 74 e2                       	je	0x1407c5a00 <.text+0x7c4a00>
1407c5a1e: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1407c5a23: 44 29 c8                    	subl	%r9d, %eax
1407c5a26: 41 f7 d1                    	notl	%r9d
1407c5a29: 44 89 cf                    	movl	%r9d, %edi
1407c5a2c: 21 c7                       	andl	%eax, %edi
1407c5a2e: f3 41 0f bc c1              	tzcntl	%r9d, %eax
1407c5a33: 48 f7 d8                    	negq	%rax
1407c5a36: 48 69 f0 98 00 00 00        	imulq	$0x98, %rax, %rsi
1407c5a3d: 48 8b 94 33 68 ff ff ff     	movq	-0x98(%rbx,%rsi), %rdx
1407c5a45: 48 89 95 88 01 00 00        	movq	%rdx, 0x188(%rbp)
1407c5a4c: 49 89 cc                    	movq	%rcx, %r12
1407c5a4f: e8 5c c8 2c 00              	callq	0x140a922b0 <.text+0xa912b0>
1407c5a54: 49 ff cf                    	decq	%r15
1407c5a57: 48 89 c2                    	movq	%rax, %rdx
1407c5a5a: 48 c1 ea 20                 	shrq	$0x20, %rdx
1407c5a5e: 41 89 f9                    	movl	%edi, %r9d
1407c5a61: 4c 89 e1                    	movq	%r12, %rcx
1407c5a64: 0f 84 66 ff ff ff           	je	0x1407c59d0 <.text+0x7c49d0>
1407c5a6a: 89 c0                       	movl	%eax, %eax
1407c5a6c: 41 89 f9                    	movl	%edi, %r9d
1407c5a6f: 4c 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %r8
1407c5a76: 49 3b 40 10                 	cmpq	0x10(%r8), %rax
1407c5a7a: 0f 83 50 ff ff ff           	jae	0x1407c59d0 <.text+0x7c49d0>
1407c5a80: 4c 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %r8
1407c5a87: 4d 8b 40 08                 	movq	0x8(%r8), %r8
1407c5a8b: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1407c5a8f: 41 89 f9                    	movl	%edi, %r9d
1407c5a92: 41 39 14 80                 	cmpl	%edx, (%r8,%rax,4)
1407c5a96: 0f 85 34 ff ff ff           	jne	0x1407c59d0 <.text+0x7c49d0>
1407c5a9c: 49 8d 04 80                 	leaq	(%r8,%rax,4), %rax
1407c5aa0: 8b 50 04                    	movl	0x4(%rax), %edx
1407c5aa3: 41 89 f9                    	movl	%edi, %r9d
1407c5aa6: 41 b8 ff ff ff ff           	movl	$0xffffffff, %r8d       # imm = 0xFFFFFFFF
1407c5aac: 4c 39 c2                    	cmpq	%r8, %rdx
1407c5aaf: 0f 84 1b ff ff ff           	je	0x1407c59d0 <.text+0x7c49d0>
1407c5ab5: 41 89 f9                    	movl	%edi, %r9d
1407c5ab8: 4c 8b 85 90 01 00 00        	movq	0x190(%rbp), %r8
1407c5abf: 49 39 50 38                 	cmpq	%rdx, 0x38(%r8)
1407c5ac3: 0f 86 07 ff ff ff           	jbe	0x1407c59d0 <.text+0x7c49d0>
1407c5ac9: 4c 8b 85 90 01 00 00        	movq	0x190(%rbp), %r8
1407c5ad0: 4d 8b 48 28                 	movq	0x28(%r8), %r9
1407c5ad4: 41 89 d0                    	movl	%edx, %r8d
1407c5ad7: 41 c1 e8 06                 	shrl	$0x6, %r8d
1407c5adb: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
1407c5adf: 49 0f a3 d0                 	btq	%rdx, %r8
1407c5ae3: 41 89 f9                    	movl	%edi, %r9d
1407c5ae6: 0f 83 e4 fe ff ff           	jae	0x1407c59d0 <.text+0x7c49d0>
1407c5aec: 8b 48 0c                    	movl	0xc(%rax), %ecx
1407c5aef: 8b 40 10                    	movl	0x10(%rax), %eax
1407c5af2: 48 8b 95 e0 01 00 00        	movq	0x1e0(%rbp), %rdx
1407c5af9: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
1407c5b00: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1407c5b04: 4c 8b 85 90 01 00 00        	movq	0x190(%rbp), %r8
1407c5b0b: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
1407c5b12: 4c 8b 4c ca 18              	movq	0x18(%rdx,%rcx,8), %r9
1407c5b17: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
1407c5b1c: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
1407c5b20: 48 f7 d1                    	notq	%rcx
1407c5b23: 48 8d 14 49                 	leaq	(%rcx,%rcx,2), %rdx
1407c5b27: 48 c1 e2 04                 	shlq	$0x4, %rdx
1407c5b2b: 48 6b c8 58                 	imulq	$0x58, %rax, %rcx
1407c5b2f: 49 03 4c 11 10              	addq	0x10(%r9,%rdx), %rcx
1407c5b34: e8 47 d8 11 00              	callq	0x1408e3380 <.text+0x8e2380>
1407c5b39: 41 89 f9                    	movl	%edi, %r9d
1407c5b3c: 84 c0                       	testb	%al, %al
1407c5b3e: 4c 89 e1                    	movq	%r12, %rcx
1407c5b41: 0f 85 89 fe ff ff           	jne	0x1407c59d0 <.text+0x7c49d0>
1407c5b47: 48 8d 14 33                 	leaq	(%rbx,%rsi), %rdx
1407c5b4b: 48 81 c2 70 ff ff ff        	addq	$-0x90, %rdx
1407c5b52: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c5b59: e8 12 55 25 00              	callq	0x140a1b070 <.text+0xa1a070>
1407c5b5e: 48 8b b5 a0 01 00 00        	movq	0x1a0(%rbp), %rsi
1407c5b65: 4c 8b ad a8 01 00 00        	movq	0x1a8(%rbp), %r13
1407c5b6c: 0f b7 95 b8 01 00 00        	movzwl	0x1b8(%rbp), %edx
1407c5b73: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407c5b7a: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1407c5b81: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1407c5b88: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
1407c5b8f: 48 83 c0 50                 	addq	$0x50, %rax
1407c5b93: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
1407c5b97: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1407c5b9e: 48 b8 00 00 00 00 ff ff ff ff       	movabsq	$-0x100000000, %rax # imm = 0xFFFFFFFF00000000
1407c5ba8: 48 21 c1                    	andq	%rax, %rcx
1407c5bab: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
1407c5bb2: 89 d1                       	movl	%edx, %ecx
1407c5bb4: 48 83 bd b0 00 00 00 00     	cmpq	$0x0, 0xb0(%rbp)
1407c5bbc: 0f 84 d0 11 00 00           	je	0x1407c6d92 <.text+0x7c5d92>
1407c5bc2: 66 85 c9                    	testw	%cx, %cx
1407c5bc5: 74 19                       	je	0x1407c5be0 <.text+0x7c4be0>
1407c5bc7: 48 85 f6                    	testq	%rsi, %rsi
1407c5bca: 0f 84 c2 11 00 00           	je	0x1407c6d92 <.text+0x7c5d92>
1407c5bd0: 8d 41 ff                    	leal	-0x1(%rcx), %eax
1407c5bd3: 21 c8                       	andl	%ecx, %eax
1407c5bd5: 89 85 ec 01 00 00           	movl	%eax, 0x1ec(%rbp)
1407c5bdb: eb 30                       	jmp	0x1407c5c0d <.text+0x7c4c0d>
1407c5bdd: 0f 1f 00                    	nopl	(%rax)
1407c5be0: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
1407c5be6: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
1407c5bea: 48 83 c6 c0                 	addq	$-0x40, %rsi
1407c5bee: 49 83 c5 10                 	addq	$0x10, %r13
1407c5bf2: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
1407c5bf8: 74 e6                       	je	0x1407c5be0 <.text+0x7c4be0>
1407c5bfa: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1407c5bff: 29 c8                       	subl	%ecx, %eax
1407c5c01: f7 d1                       	notl	%ecx
1407c5c03: 89 ca                       	movl	%ecx, %edx
1407c5c05: 21 c2                       	andl	%eax, %edx
1407c5c07: 89 95 ec 01 00 00           	movl	%edx, 0x1ec(%rbp)
1407c5c0d: f3 0f bc c1                 	tzcntl	%ecx, %eax
1407c5c11: c1 e0 02                    	shll	$0x2, %eax
1407c5c14: 48 89 f1                    	movq	%rsi, %rcx
1407c5c17: 48 29 c1                    	subq	%rax, %rcx
1407c5c1a: 8b 41 fc                    	movl	-0x4(%rcx), %eax
1407c5c1d: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1407c5c24: 89 85 a0 01 00 00           	movl	%eax, 0x1a0(%rbp)
1407c5c2a: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
1407c5c31: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
1407c5c36: 0f 84 17 12 00 00           	je	0x1407c6e53 <.text+0x7c5e53>
1407c5c3c: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
1407c5c40: 48 8d 95 a0 01 00 00        	leaq	0x1a0(%rbp), %rdx
1407c5c47: e8 84 b2 de ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1407c5c4c: 48 ff 8d b0 00 00 00        	decq	0xb0(%rbp)
1407c5c53: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
1407c5c5a: 48 8b 51 30                 	movq	0x30(%rcx), %rdx
1407c5c5e: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
1407c5c62: 49 89 cb                    	movq	%rcx, %r11
1407c5c65: 49 21 c3                    	andq	%rax, %r11
1407c5c68: 48 c1 e8 39                 	shrq	$0x39, %rax
1407c5c6c: 66 0f 6e c0                 	movd	%eax, %xmm0
1407c5c70: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407c5c74: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407c5c79: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407c5c7e: 48 8d 42 b0                 	leaq	-0x50(%rdx), %rax
1407c5c82: 45 31 c0                    	xorl	%r8d, %r8d
1407c5c85: 48 89 95 28 02 00 00        	movq	%rdx, 0x228(%rbp)
1407c5c8c: f3 42 0f 6f 0c 1a           	movdqu	(%rdx,%r11), %xmm1
1407c5c92: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407c5c96: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407c5c9a: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407c5c9f: 45 85 c9                    	testl	%r9d, %r9d
1407c5ca2: 74 37                       	je	0x1407c5cdb <.text+0x7c4cdb>
1407c5ca4: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407c5ca9: 4d 01 da                    	addq	%r11, %r10
1407c5cac: 49 21 ca                    	andq	%rcx, %r10
1407c5caf: 49 f7 da                    	negq	%r10
1407c5cb2: 4f 8d 14 92                 	leaq	(%r10,%r10,4), %r10
1407c5cb6: 49 c1 e2 04                 	shlq	$0x4, %r10
1407c5cba: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1407c5cc1: 4c 89 95 18 02 00 00        	movq	%r10, 0x218(%rbp)
1407c5cc8: 42 3b 14 10                 	cmpl	(%rax,%r10), %edx
1407c5ccc: 74 42                       	je	0x1407c5d10 <.text+0x7c4d10>
1407c5cce: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407c5cd2: 66 45 21 ca                 	andw	%r9w, %r10w
1407c5cd6: 45 89 d1                    	movl	%r10d, %r9d
1407c5cd9: 75 c9                       	jne	0x1407c5ca4 <.text+0x7c4ca4>
1407c5cdb: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1407c5cdf: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407c5ce4: 45 85 c9                    	testl	%r9d, %r9d
1407c5ce7: 0f 85 66 11 00 00           	jne	0x1407c6e53 <.text+0x7c5e53>
1407c5ced: 4d 01 c3                    	addq	%r8, %r11
1407c5cf0: 49 83 c3 10                 	addq	$0x10, %r11
1407c5cf4: 49 83 c0 10                 	addq	$0x10, %r8
1407c5cf8: 49 21 cb                    	andq	%rcx, %r11
1407c5cfb: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
1407c5d02: eb 88                       	jmp	0x1407c5c8c <.text+0x7c4c8c>
1407c5d04: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1407c5d10: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
1407c5d17: e8 54 0c 36 00              	callq	0x140b26970 <.text+0xb25970>
1407c5d1c: 8b 8d ec 01 00 00           	movl	0x1ec(%rbp), %ecx
1407c5d22: 3c 08                       	cmpb	$0x8, %al
1407c5d24: 0f 85 8a fe ff ff           	jne	0x1407c5bb4 <.text+0x7c4bb4>
1407c5d2a: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
1407c5d2e: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
1407c5d32: 4c 8b 85 70 01 00 00        	movq	0x170(%rbp), %r8
1407c5d39: 4c 8d 0d b8 0e 2c 02        	leaq	0x22c0eb8(%rip), %r9    # 0x142a86bf8
1407c5d40: e8 db 4c 25 00              	callq	0x140a1aa20 <.text+0xa19a20>
1407c5d45: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407c5d4c: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
1407c5d53: 48 01 c8                    	addq	%rcx, %rax
1407c5d56: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1407c5d5d: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407c5d61: 88 85 00 02 00 00           	movb	%al, 0x200(%rbp)
1407c5d67: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
1407c5d6e: b1 08                       	movb	$0x8, %cl
1407c5d70: e8 db d4 06 00              	callq	0x140833250 <.text+0x832250>
1407c5d75: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
1407c5d7c: 48 89 8d 10 02 00 00        	movq	%rcx, 0x210(%rbp)
1407c5d83: 0f b6 8d 00 02 00 00        	movzbl	0x200(%rbp), %ecx
1407c5d8a: 89 c2                       	movl	%eax, %edx
1407c5d8c: e8 ff fe 35 00              	callq	0x140b25c90 <.text+0xb24c90>
1407c5d91: 88 85 50 01 00 00           	movb	%al, 0x150(%rbp)
1407c5d97: 88 95 51 01 00 00           	movb	%dl, 0x151(%rbp)
1407c5d9d: 48 8b 8d 10 02 00 00        	movq	0x210(%rbp), %rcx
1407c5da4: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407c5dab: e8 e0 71 a9 00              	callq	0x14125cf90 <.text+0x125bf90>
1407c5db0: 48 85 c0                    	testq	%rax, %rax
1407c5db3: 0f 84 43 12 00 00           	je	0x1407c6ffc <.text+0x7c5ffc>
1407c5db9: f3 0f 6f 40 20              	movdqu	0x20(%rax), %xmm0
1407c5dbe: f3 0f 6f 48 30              	movdqu	0x30(%rax), %xmm1
1407c5dc3: 66 0f 7f 4d 70              	movdqa	%xmm1, 0x70(%rbp)
1407c5dc8: 66 0f 7f 45 60              	movdqa	%xmm0, 0x60(%rbp)
1407c5dcd: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407c5dd4: 0f b6 50 fc                 	movzbl	-0x4(%rax), %edx
1407c5dd8: 31 c0                       	xorl	%eax, %eax
1407c5dda: 80 fa 02                    	cmpb	$0x2, %dl
1407c5ddd: 0f 94 c1                    	sete	%cl
1407c5de0: 41 0f 28 fa                 	movaps	%xmm10, %xmm7
1407c5de4: 80 fa 03                    	cmpb	$0x3, %dl
1407c5de7: 74 0e                       	je	0x1407c5df7 <.text+0x7c4df7>
1407c5de9: 88 c8                       	movb	%cl, %al
1407c5deb: 48 8d 0d b6 09 2c 02        	leaq	0x22c09b6(%rip), %rcx   # 0x142a867a8
1407c5df2: f3 0f 10 3c 81              	movss	(%rcx,%rax,4), %xmm7
1407c5df7: f3 0f 11 7d 70              	movss	%xmm7, 0x70(%rbp)
1407c5dfc: 8b 85 e4 00 00 00           	movl	0xe4(%rbp), %eax
1407c5e02: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1407c5e06: 89 01                       	movl	%eax, (%rcx)
1407c5e08: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1407c5e0f: e8 0c 67 0f 00              	callq	0x1408bc520 <.text+0x8bb520>
1407c5e14: 49 89 d1                    	movq	%rdx, %r9
1407c5e17: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
1407c5e1e: 0f b6 49 ff                 	movzbl	-0x1(%rcx), %ecx
1407c5e22: 88 4c 24 20                 	movb	%cl, 0x20(%rsp)
1407c5e26: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1407c5e2a: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1407c5e31: 49 89 c0                    	movq	%rax, %r8
1407c5e34: e8 77 fa 05 00              	callq	0x1408258b0 <.text+0x8248b0>
1407c5e39: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1407c5e40: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407c5e47: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
1407c5e4e: 48 01 c1                    	addq	%rax, %rcx
1407c5e51: 48 83 c1 b8                 	addq	$-0x48, %rcx
1407c5e55: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407c5e5c: 0f b6 50 ff                 	movzbl	-0x1(%rax), %edx
1407c5e60: 88 95 27 02 00 00           	movb	%dl, 0x227(%rbp)
1407c5e66: 8b 40 e0                    	movl	-0x20(%rax), %eax
1407c5e69: 89 85 10 02 00 00           	movl	%eax, 0x210(%rbp)
1407c5e6f: 48 8d 15 9a 0d 2c 02        	leaq	0x22c0d9a(%rip), %rdx   # 0x142a86c10
1407c5e76: e8 35 d5 06 00              	callq	0x1408333b0 <.text+0x8323b0>
1407c5e7b: 4c 89 a5 28 02 00 00        	movq	%r12, 0x228(%rbp)
1407c5e82: 49 83 7c 24 18 00           	cmpq	$0x0, 0x18(%r12)
1407c5e88: 0f 84 d6 0f 00 00           	je	0x1407c6e64 <.text+0x7c5e64>
1407c5e8e: f3 44 0f 10 18              	movss	(%rax), %xmm11
1407c5e93: 48 8b 4d 00                 	movq	(%rbp), %rcx
1407c5e97: 48 8d 95 88 01 00 00        	leaq	0x188(%rbp), %rdx
1407c5e9e: e8 cd ae de ff              	callq	0x1405b0d70 <.text+0x5afd70>
1407c5ea3: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1407c5eaa: 4c 8b 19                    	movq	(%rcx), %r11
1407c5ead: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1407c5eb1: 48 89 ca                    	movq	%rcx, %rdx
1407c5eb4: 48 21 c2                    	andq	%rax, %rdx
1407c5eb7: 48 c1 e8 39                 	shrq	$0x39, %rax
1407c5ebb: 66 0f 6e c0                 	movd	%eax, %xmm0
1407c5ebf: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407c5ec3: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407c5ec8: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407c5ecd: 49 8d 83 58 fd ff ff        	leaq	-0x2a8(%r11), %rax
1407c5ed4: 4c 8b 85 88 01 00 00        	movq	0x188(%rbp), %r8
1407c5edb: 45 31 c9                    	xorl	%r9d, %r9d
1407c5ede: f3 41 0f 6f 0c 13           	movdqu	(%r11,%rdx), %xmm1
1407c5ee4: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407c5ee8: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407c5eec: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1407c5ef1: 45 85 d2                    	testl	%r10d, %r10d
1407c5ef4: 74 3d                       	je	0x1407c5f33 <.text+0x7c4f33>
1407c5ef6: 4c 89 9d 18 02 00 00        	movq	%r11, 0x218(%rbp)
1407c5efd: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1407c5f02: 49 01 d3                    	addq	%rdx, %r11
1407c5f05: 49 21 cb                    	andq	%rcx, %r11
1407c5f08: 49 f7 db                    	negq	%r11
1407c5f0b: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
1407c5f12: 4c 89 9d 28 02 00 00        	movq	%r11, 0x228(%rbp)
1407c5f19: 4e 3b 04 18                 	cmpq	(%rax,%r11), %r8
1407c5f1d: 74 36                       	je	0x1407c5f55 <.text+0x7c4f55>
1407c5f1f: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1407c5f23: 66 45 21 d3                 	andw	%r10w, %r11w
1407c5f27: 45 89 da                    	movl	%r11d, %r10d
1407c5f2a: 4c 8b 9d 18 02 00 00        	movq	0x218(%rbp), %r11
1407c5f31: 75 ca                       	jne	0x1407c5efd <.text+0x7c4efd>
1407c5f33: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1407c5f37: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
1407c5f3c: 45 85 d2                    	testl	%r10d, %r10d
1407c5f3f: 0f 85 1f 0f 00 00           	jne	0x1407c6e64 <.text+0x7c5e64>
1407c5f45: 4c 01 ca                    	addq	%r9, %rdx
1407c5f48: 48 83 c2 10                 	addq	$0x10, %rdx
1407c5f4c: 49 83 c1 10                 	addq	$0x10, %r9
1407c5f50: 48 21 ca                    	andq	%rcx, %rdx
1407c5f53: eb 89                       	jmp	0x1407c5ede <.text+0x7c4ede>
1407c5f55: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407c5f5c: 8b 40 e4                    	movl	-0x1c(%rax), %eax
1407c5f5f: 89 85 f8 01 00 00           	movl	%eax, 0x1f8(%rbp)
1407c5f65: 48 8d 8d 88 01 00 00        	leaq	0x188(%rbp), %rcx
1407c5f6c: e8 5f 26 8c ff              	callq	0x1400885d0 <.text+0x875d0>
1407c5f71: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
1407c5f78: f3 44 0f 10 65 70           	movss	0x70(%rbp), %xmm12
1407c5f7e: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
1407c5f85: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
1407c5f8c: 48 01 c1                    	addq	%rax, %rcx
1407c5f8f: 48 83 c1 f9                 	addq	$-0x7, %rcx
1407c5f93: e8 d8 f6 0f 00              	callq	0x1408c5670 <.text+0x8c4670>
1407c5f98: 88 85 28 02 00 00           	movb	%al, 0x228(%rbp)
1407c5f9e: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407c5fa5: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407c5fa9: fe c8                       	decb	%al
1407c5fab: 3c 03                       	cmpb	$0x3, %al
1407c5fad: 72 0d                       	jb	0x1407c5fbc <.text+0x7c4fbc>
1407c5faf: 8b 85 ec 01 00 00           	movl	0x1ec(%rbp), %eax
1407c5fb5: 89 c1                       	movl	%eax, %ecx
1407c5fb7: e9 f8 fb ff ff              	jmp	0x1407c5bb4 <.text+0x7c4bb4>
1407c5fbc: 0f b6 c0                    	movzbl	%al, %eax
1407c5fbf: 48 8d 0d 72 0e 2c 02        	leaq	0x22c0e72(%rip), %rcx   # 0x142a86e38
1407c5fc6: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1407c5fca: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1407c5fd1: 81 bd 10 02 00 00 e7 03 00 00       	cmpl	$0x3e7, 0x210(%rbp) # imm = 0x3E7
1407c5fdb: 48 8d 0d c7 0c 2c 02        	leaq	0x22c0cc7(%rip), %rcx   # 0x142a86ca9
1407c5fe2: 48 8d 05 de 0c 2c 02        	leaq	0x22c0cde(%rip), %rax   # 0x142a86cc7
1407c5fe9: 48 0f 44 c8                 	cmoveq	%rax, %rcx
1407c5fed: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
1407c5ff1: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
1407c5ff8: 48 8d 82 b0 00 00 00        	leaq	0xb0(%rdx), %rax
1407c5fff: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1407c6006: 66 0f 6f 45 30              	movdqa	0x30(%rbp), %xmm0
1407c600b: 66 0f 6f 4d 40              	movdqa	0x40(%rbp), %xmm1
1407c6010: 66 0f 6f 55 50              	movdqa	0x50(%rbp), %xmm2
1407c6015: 66 0f 7f 85 00 01 00 00     	movdqa	%xmm0, 0x100(%rbp)
1407c601d: 66 0f 7f 8d 10 01 00 00     	movdqa	%xmm1, 0x110(%rbp)
1407c6025: 66 0f 7f 95 20 01 00 00     	movdqa	%xmm2, 0x120(%rbp)
1407c602d: 48 8d 85 a3 01 00 00        	leaq	0x1a3(%rbp), %rax
1407c6034: c7 40 08 00 00 00 00        	movl	$0x0, 0x8(%rax)
1407c603b: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1407c6042: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1407c6049: 48 8b 8d a7 01 00 00        	movq	0x1a7(%rbp), %rcx
1407c6050: 48 89 8d ef 00 00 00        	movq	%rcx, 0xef(%rbp)
1407c6057: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407c605e: 48 8b 82 c0 00 00 00        	movq	0xc0(%rdx), %rax
1407c6065: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1407c606c: 48 3b 82 b0 00 00 00        	cmpq	0xb0(%rdx), %rax
1407c6073: 75 13                       	jne	0x1407c6088 <.text+0x7c5088>
1407c6075: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
1407c607c: 48 8d 15 65 0c 2c 02        	leaq	0x22c0c65(%rip), %rdx   # 0x142a86ce8
1407c6083: e8 28 53 06 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407c6088: 4c 8b 8d 00 02 00 00        	movq	0x200(%rbp), %r9
1407c608f: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407c6096: 4c 8b 95 18 02 00 00        	movq	0x218(%rbp), %r10
1407c609d: 49 69 ca 60 01 00 00        	imulq	$0x160, %r10, %rcx      # imm = 0x160
1407c60a4: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407c60ac: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407c60b5: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407c60be: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407c60c6: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
1407c60ca: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407c60cf: 8b 55 28                    	movl	0x28(%rbp), %edx
1407c60d2: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407c60d6: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407c60de: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
1407c60e5: 0f 10 8d b0 01 00 00        	movups	0x1b0(%rbp), %xmm1
1407c60ec: 0f 10 95 c0 01 00 00        	movups	0x1c0(%rbp), %xmm2
1407c60f3: 0f 10 9d cc 01 00 00        	movups	0x1cc(%rbp), %xmm3
1407c60fa: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407c60ff: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407c6104: 0f 11 54 08 64              	movups	%xmm2, 0x64(%rax,%rcx)
1407c6109: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407c610e: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407c6119: 48 8b 95 84 00 00 00        	movq	0x84(%rbp), %rdx
1407c6120: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407c6128: 8b 95 8c 00 00 00           	movl	0x8c(%rbp), %edx
1407c612e: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407c6135: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407c6140: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
1407c6147: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407c614f: 8b 95 98 00 00 00           	movl	0x98(%rbp), %edx
1407c6155: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407c615c: 48 8d 15 d5 0c 2c 02        	leaq	0x22c0cd5(%rip), %rdx   # 0x142a86e38
1407c6163: 4c 8b 85 08 02 00 00        	movq	0x208(%rbp), %r8
1407c616a: 49 01 d0                    	addq	%rdx, %r8
1407c616d: 4c 89 84 08 a0 00 00 00     	movq	%r8, 0xa0(%rax,%rcx)
1407c6175: 48 c7 84 08 a8 00 00 00 23 00 00 00 	movq	$0x23, 0xa8(%rax,%rcx)
1407c6181: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
1407c6185: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407c618d: 48 c7 84 08 b8 00 00 00 1e 00 00 00 	movq	$0x1e, 0xb8(%rax,%rcx)
1407c6199: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
1407c61a0: 0f 28 8d 10 01 00 00        	movaps	0x110(%rbp), %xmm1
1407c61a7: 66 0f 6f 95 20 01 00 00     	movdqa	0x120(%rbp), %xmm2
1407c61af: 66 0f 7f 94 08 e0 00 00 00  	movdqa	%xmm2, 0xe0(%rax,%rcx)
1407c61b8: 0f 29 8c 08 d0 00 00 00     	movaps	%xmm1, 0xd0(%rax,%rcx)
1407c61c0: 0f 29 84 08 c0 00 00 00     	movaps	%xmm0, 0xc0(%rax,%rcx)
1407c61c8: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407c61d3: 8b 95 10 02 00 00           	movl	0x210(%rbp), %edx
1407c61d9: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407c61e0: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
1407c61e7: 48 89 94 08 f8 00 00 00     	movq	%rdx, 0xf8(%rax,%rcx)
1407c61ef: 8b 95 f8 01 00 00           	movl	0x1f8(%rbp), %edx
1407c61f5: 89 94 08 00 01 00 00        	movl	%edx, 0x100(%rax,%rcx)
1407c61fc: f3 41 0f 58 fb              	addss	%xmm11, %xmm7
1407c6201: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
1407c6208: 89 94 08 04 01 00 00        	movl	%edx, 0x104(%rax,%rcx)
1407c620f: f3 44 0f 11 9c 08 08 01 00 00       	movss	%xmm11, 0x108(%rax,%rcx)
1407c6219: f3 0f 11 bc 08 0c 01 00 00  	movss	%xmm7, 0x10c(%rax,%rcx)
1407c6222: f3 44 0f 11 a4 08 10 01 00 00       	movss	%xmm12, 0x110(%rax,%rcx)
1407c622c: 0f b6 95 28 02 00 00        	movzbl	0x228(%rbp), %edx
1407c6233: 88 94 08 14 01 00 00        	movb	%dl, 0x114(%rax,%rcx)
1407c623a: 0f b6 95 9e 00 00 00        	movzbl	0x9e(%rbp), %edx
1407c6241: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407c6248: 0f b7 95 9c 00 00 00        	movzwl	0x9c(%rbp), %edx
1407c624f: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407c6257: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407c6262: f3 0f 6f 85 50 01 00 00     	movdqu	0x150(%rbp), %xmm0
1407c626a: f3 0f 6f 8d 60 01 00 00     	movdqu	0x160(%rbp), %xmm1
1407c6272: f3 0f 7f 8c 08 2c 01 00 00  	movdqu	%xmm1, 0x12c(%rax,%rcx)
1407c627b: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407c6284: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407c628c: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
1407c6293: 4c 8b 85 ef 00 00 00        	movq	0xef(%rbp), %r8
1407c629a: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407c62a2: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407c62aa: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407c62b2: 0f b6 95 27 02 00 00        	movzbl	0x227(%rbp), %edx
1407c62b9: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407c62c0: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407c62ca: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
1407c62d1: 4c 8b 85 d7 00 00 00        	movq	0xd7(%rbp), %r8
1407c62d8: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407c62e0: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407c62e8: 49 ff c2                    	incq	%r10
1407c62eb: 4d 89 91 c0 00 00 00        	movq	%r10, 0xc0(%r9)
1407c62f2: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407c62f9: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407c62fd: 88 85 18 02 00 00           	movb	%al, 0x218(%rbp)
1407c6303: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
1407c630a: b1 08                       	movb	$0x8, %cl
1407c630c: e8 3f cf 06 00              	callq	0x140833250 <.text+0x832250>
1407c6311: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
1407c6318: 48 89 8d 28 02 00 00        	movq	%rcx, 0x228(%rbp)
1407c631f: 0f b6 8d 18 02 00 00        	movzbl	0x218(%rbp), %ecx
1407c6326: 89 c2                       	movl	%eax, %edx
1407c6328: e8 63 f9 35 00              	callq	0x140b25c90 <.text+0xb24c90>
1407c632d: 88 85 50 01 00 00           	movb	%al, 0x150(%rbp)
1407c6333: 88 95 51 01 00 00           	movb	%dl, 0x151(%rbp)
1407c6339: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407c6340: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
1407c6345: 0f 84 74 0b 00 00           	je	0x1407c6ebf <.text+0x7c5ebf>
1407c634b: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407c6352: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
1407c6356: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407c635d: e8 5e 1a a3 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407c6362: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
1407c6369: 48 8b 0a                    	movq	(%rdx), %rcx
1407c636c: 4c 8b 52 08                 	movq	0x8(%rdx), %r10
1407c6370: 4d 89 d0                    	movq	%r10, %r8
1407c6373: 49 21 c0                    	andq	%rax, %r8
1407c6376: 48 c1 e8 39                 	shrq	$0x39, %rax
1407c637a: 66 0f 6e c0                 	movd	%eax, %xmm0
1407c637e: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407c6382: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407c6387: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407c638c: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407c6390: 0f b6 95 50 01 00 00        	movzbl	0x150(%rbp), %edx
1407c6397: 44 0f b6 9d 51 01 00 00     	movzbl	0x151(%rbp), %r11d
1407c639f: 48 c7 85 28 02 00 00 00 00 00 00    	movq	$0x0, 0x228(%rbp)
1407c63aa: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407c63b0: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407c63b4: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407c63b8: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407c63bd: 45 85 c9                    	testl	%r9d, %r9d
1407c63c0: 0f 84 cc 00 00 00           	je	0x1407c6492 <.text+0x7c5492>
1407c63c6: 80 fa 1a                    	cmpb	$0x1a, %dl
1407c63c9: 4c 89 95 08 02 00 00        	movq	%r10, 0x208(%rbp)
1407c63d0: 88 95 18 02 00 00           	movb	%dl, 0x218(%rbp)
1407c63d6: 44 88 9d 10 02 00 00        	movb	%r11b, 0x210(%rbp)
1407c63dd: 75 48                       	jne	0x1407c6427 <.text+0x7c5427>
1407c63df: f3 45 0f bc d9              	tzcntl	%r9d, %r11d
1407c63e4: 4d 01 c3                    	addq	%r8, %r11
1407c63e7: 4d 21 d3                    	andq	%r10, %r11
1407c63ea: 49 f7 db                    	negq	%r11
1407c63ed: 49 89 c2                    	movq	%rax, %r10
1407c63f0: 4b 8d 04 db                 	leaq	(%r11,%r11,8), %rax
1407c63f4: 41 80 3c c2 1a              	cmpb	$0x1a, (%r10,%rax,8)
1407c63f9: 0f 84 ca 00 00 00           	je	0x1407c64c9 <.text+0x7c54c9>
1407c63ff: 41 8d 41 ff                 	leal	-0x1(%r9), %eax
1407c6403: 66 44 21 c8                 	andw	%r9w, %ax
1407c6407: 41 89 c1                    	movl	%eax, %r9d
1407c640a: 4c 89 d0                    	movq	%r10, %rax
1407c640d: 4c 8b 95 08 02 00 00        	movq	0x208(%rbp), %r10
1407c6414: 0f b6 95 18 02 00 00        	movzbl	0x218(%rbp), %edx
1407c641b: 44 0f b6 9d 10 02 00 00     	movzbl	0x210(%rbp), %r11d
1407c6423: 75 ba                       	jne	0x1407c63df <.text+0x7c53df>
1407c6425: eb 6b                       	jmp	0x1407c6492 <.text+0x7c5492>
1407c6427: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1407c642e: f3 45 0f bc d9              	tzcntl	%r9d, %r11d
1407c6433: 4d 01 c3                    	addq	%r8, %r11
1407c6436: 4d 21 d3                    	andq	%r10, %r11
1407c6439: 49 f7 db                    	negq	%r11
1407c643c: 4f 8d 14 db                 	leaq	(%r11,%r11,8), %r10
1407c6440: 42 0f b6 04 d0              	movzbl	(%rax,%r10,8), %eax
1407c6445: 3c 1a                       	cmpb	$0x1a, %al
1407c6447: 74 1f                       	je	0x1407c6468 <.text+0x7c5468>
1407c6449: 38 85 18 02 00 00           	cmpb	%al, 0x218(%rbp)
1407c644f: 75 17                       	jne	0x1407c6468 <.text+0x7c5468>
1407c6451: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1407c6458: 4a 8d 04 d0                 	leaq	(%rax,%r10,8), %rax
1407c645c: 0f b6 95 10 02 00 00        	movzbl	0x210(%rbp), %edx
1407c6463: 3a 50 01                    	cmpb	0x1(%rax), %dl
1407c6466: 74 61                       	je	0x1407c64c9 <.text+0x7c54c9>
1407c6468: 41 8d 41 ff                 	leal	-0x1(%r9), %eax
1407c646c: 66 44 21 c8                 	andw	%r9w, %ax
1407c6470: 41 89 c1                    	movl	%eax, %r9d
1407c6473: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1407c647a: 4c 8b 95 08 02 00 00        	movq	0x208(%rbp), %r10
1407c6481: 0f b6 95 18 02 00 00        	movzbl	0x218(%rbp), %edx
1407c6488: 44 0f b6 9d 10 02 00 00     	movzbl	0x210(%rbp), %r11d
1407c6490: 75 9c                       	jne	0x1407c642e <.text+0x7c542e>
1407c6492: 49 89 c1                    	movq	%rax, %r9
1407c6495: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1407c6499: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
1407c649d: 85 c0                       	testl	%eax, %eax
1407c649f: 0f 85 1a 0a 00 00           	jne	0x1407c6ebf <.text+0x7c5ebf>
1407c64a5: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407c64ac: 49 01 c0                    	addq	%rax, %r8
1407c64af: 49 83 c0 10                 	addq	$0x10, %r8
1407c64b3: 48 83 c0 10                 	addq	$0x10, %rax
1407c64b7: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
1407c64be: 4d 21 d0                    	andq	%r10, %r8
1407c64c1: 4c 89 c8                    	movq	%r9, %rax
1407c64c4: e9 e1 fe ff ff              	jmp	0x1407c63aa <.text+0x7c53aa>
1407c64c9: 4b 8d 04 db                 	leaq	(%r11,%r11,8), %rax
1407c64cd: 48 8b 44 c1 d8              	movq	-0x28(%rcx,%rax,8), %rax
1407c64d2: f0                          	lock
1407c64d3: 48 ff 00                    	incq	(%rax)
1407c64d6: 0f 8e c0 0b 00 00           	jle	0x1407c709c <.text+0x7c609c>
1407c64dc: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1407c64e3: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
1407c64ea: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407c64f1: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407c64f5: 88 85 18 02 00 00           	movb	%al, 0x218(%rbp)
1407c64fb: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
1407c6502: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
1407c6509: b1 08                       	movb	$0x8, %cl
1407c650b: e8 40 cd 06 00              	callq	0x140833250 <.text+0x832250>
1407c6510: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
1407c6517: 48 89 8d 28 02 00 00        	movq	%rcx, 0x228(%rbp)
1407c651e: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
1407c6525: 0f b6 8d 18 02 00 00        	movzbl	0x218(%rbp), %ecx
1407c652c: 89 c2                       	movl	%eax, %edx
1407c652e: e8 5d f7 35 00              	callq	0x140b25c90 <.text+0xb24c90>
1407c6533: 88 85 e8 00 00 00           	movb	%al, 0xe8(%rbp)
1407c6539: 88 95 e9 00 00 00           	movb	%dl, 0xe9(%rbp)
1407c653f: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407c6546: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
1407c654b: 0f 84 df 09 00 00           	je	0x1407c6f30 <.text+0x7c5f30>
1407c6551: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407c6558: 48 8d 48 50                 	leaq	0x50(%rax), %rcx
1407c655c: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
1407c6563: 48 8d 95 e8 00 00 00        	leaq	0xe8(%rbp), %rdx
1407c656a: e8 51 18 a3 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407c656f: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
1407c6576: 48 8b 4a 30                 	movq	0x30(%rdx), %rcx
1407c657a: 4c 8b 52 38                 	movq	0x38(%rdx), %r10
1407c657e: 4d 89 d0                    	movq	%r10, %r8
1407c6581: 49 21 c0                    	andq	%rax, %r8
1407c6584: 48 c1 e8 39                 	shrq	$0x39, %rax
1407c6588: 66 0f 6e c0                 	movd	%eax, %xmm0
1407c658c: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407c6590: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407c6595: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407c659a: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407c659e: 0f b6 95 e8 00 00 00        	movzbl	0xe8(%rbp), %edx
1407c65a5: 44 0f b6 9d e9 00 00 00     	movzbl	0xe9(%rbp), %r11d
1407c65ad: 48 c7 85 28 02 00 00 00 00 00 00    	movq	$0x0, 0x228(%rbp)
1407c65b8: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407c65be: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407c65c2: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407c65c6: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407c65cb: 45 85 c9                    	testl	%r9d, %r9d
1407c65ce: 0f 84 cc 00 00 00           	je	0x1407c66a0 <.text+0x7c56a0>
1407c65d4: 80 fa 1a                    	cmpb	$0x1a, %dl
1407c65d7: 4c 89 95 f8 01 00 00        	movq	%r10, 0x1f8(%rbp)
1407c65de: 88 95 18 02 00 00           	movb	%dl, 0x218(%rbp)
1407c65e4: 44 88 9d 08 02 00 00        	movb	%r11b, 0x208(%rbp)
1407c65eb: 75 48                       	jne	0x1407c6635 <.text+0x7c5635>
1407c65ed: f3 45 0f bc d9              	tzcntl	%r9d, %r11d
1407c65f2: 4d 01 c3                    	addq	%r8, %r11
1407c65f5: 4d 21 d3                    	andq	%r10, %r11
1407c65f8: 49 f7 db                    	negq	%r11
1407c65fb: 49 89 c2                    	movq	%rax, %r10
1407c65fe: 4b 8d 04 db                 	leaq	(%r11,%r11,8), %rax
1407c6602: 41 80 3c c2 1a              	cmpb	$0x1a, (%r10,%rax,8)
1407c6607: 0f 84 ca 00 00 00           	je	0x1407c66d7 <.text+0x7c56d7>
1407c660d: 41 8d 41 ff                 	leal	-0x1(%r9), %eax
1407c6611: 66 44 21 c8                 	andw	%r9w, %ax
1407c6615: 41 89 c1                    	movl	%eax, %r9d
1407c6618: 4c 89 d0                    	movq	%r10, %rax
1407c661b: 4c 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %r10
1407c6622: 0f b6 95 18 02 00 00        	movzbl	0x218(%rbp), %edx
1407c6629: 44 0f b6 9d 08 02 00 00     	movzbl	0x208(%rbp), %r11d
1407c6631: 75 ba                       	jne	0x1407c65ed <.text+0x7c55ed>
1407c6633: eb 6b                       	jmp	0x1407c66a0 <.text+0x7c56a0>
1407c6635: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
1407c663c: f3 45 0f bc d9              	tzcntl	%r9d, %r11d
1407c6641: 4d 01 c3                    	addq	%r8, %r11
1407c6644: 4d 21 d3                    	andq	%r10, %r11
1407c6647: 49 f7 db                    	negq	%r11
1407c664a: 4f 8d 14 db                 	leaq	(%r11,%r11,8), %r10
1407c664e: 42 0f b6 04 d0              	movzbl	(%rax,%r10,8), %eax
1407c6653: 3c 1a                       	cmpb	$0x1a, %al
1407c6655: 74 1f                       	je	0x1407c6676 <.text+0x7c5676>
1407c6657: 38 85 18 02 00 00           	cmpb	%al, 0x218(%rbp)
1407c665d: 75 17                       	jne	0x1407c6676 <.text+0x7c5676>
1407c665f: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
1407c6666: 4a 8d 04 d0                 	leaq	(%rax,%r10,8), %rax
1407c666a: 0f b6 95 08 02 00 00        	movzbl	0x208(%rbp), %edx
1407c6671: 3a 50 01                    	cmpb	0x1(%rax), %dl
1407c6674: 74 61                       	je	0x1407c66d7 <.text+0x7c56d7>
1407c6676: 41 8d 41 ff                 	leal	-0x1(%r9), %eax
1407c667a: 66 44 21 c8                 	andw	%r9w, %ax
1407c667e: 41 89 c1                    	movl	%eax, %r9d
1407c6681: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
1407c6688: 4c 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %r10
1407c668f: 0f b6 95 18 02 00 00        	movzbl	0x218(%rbp), %edx
1407c6696: 44 0f b6 9d 08 02 00 00     	movzbl	0x208(%rbp), %r11d
1407c669e: 75 9c                       	jne	0x1407c663c <.text+0x7c563c>
1407c66a0: 49 89 c1                    	movq	%rax, %r9
1407c66a3: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1407c66a7: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
1407c66ab: 85 c0                       	testl	%eax, %eax
1407c66ad: 0f 85 7d 08 00 00           	jne	0x1407c6f30 <.text+0x7c5f30>
1407c66b3: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
1407c66ba: 49 01 c0                    	addq	%rax, %r8
1407c66bd: 49 83 c0 10                 	addq	$0x10, %r8
1407c66c1: 48 83 c0 10                 	addq	$0x10, %rax
1407c66c5: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
1407c66cc: 4d 21 d0                    	andq	%r10, %r8
1407c66cf: 4c 89 c8                    	movq	%r9, %rax
1407c66d2: e9 e1 fe ff ff              	jmp	0x1407c65b8 <.text+0x7c55b8>
1407c66d7: 4b 8d 04 db                 	leaq	(%r11,%r11,8), %rax
1407c66db: 4c 8b 44 c1 d8              	movq	-0x28(%rcx,%rax,8), %r8
1407c66e0: f0                          	lock
1407c66e1: 49 ff 00                    	incq	(%r8)
1407c66e4: 0f 8e b2 09 00 00           	jle	0x1407c709c <.text+0x7c609c>
1407c66ea: f2 44 0f 10 65 40           	movsd	0x40(%rbp), %xmm12
1407c66f0: f3 44 0f 10 5d 48           	movss	0x48(%rbp), %xmm11
1407c66f6: 0f 28 7d 30                 	movaps	0x30(%rbp), %xmm7
1407c66fa: f2 44 0f 11 a5 a0 01 00 00  	movsd	%xmm12, 0x1a0(%rbp)
1407c6703: f3 44 0f 11 9d a8 01 00 00  	movss	%xmm11, 0x1a8(%rbp)
1407c670c: 0f 29 7d c0                 	movaps	%xmm7, -0x40(%rbp)
1407c6710: c6 85 37 02 00 00 00        	movb	$0x0, 0x237(%rbp)
1407c6717: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
1407c671b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c6720: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
1407c6727: 48 8b 95 10 02 00 00        	movq	0x210(%rbp), %rdx
1407c672e: 4c 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %r9
1407c6735: e8 76 eb 05 00              	callq	0x1408252b0 <.text+0x8242b0>
1407c673a: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
1407c6741: 48 8b 01                    	movq	(%rcx), %rax
1407c6744: 48 0b 41 08                 	orq	0x8(%rcx), %rax
1407c6748: 0f 85 66 08 00 00           	jne	0x1407c6fb4 <.text+0x7c5fb4>
1407c674e: 44 0f 29 65 10              	movaps	%xmm12, 0x10(%rbp)
1407c6753: f3 44 0f 11 9d 98 01 00 00  	movss	%xmm11, 0x198(%rbp)
1407c675c: 48 c7 41 08 00 00 00 00     	movq	$0x0, 0x8(%rcx)
1407c6764: 48 c7 01 01 00 00 00        	movq	$0x1, (%rcx)
1407c676b: 66 0f 6f 45 30              	movdqa	0x30(%rbp), %xmm0
1407c6770: 66 0f 6f 4d 40              	movdqa	0x40(%rbp), %xmm1
1407c6775: 66 0f 6f 55 50              	movdqa	0x50(%rbp), %xmm2
1407c677a: 66 0f 7f 41 10              	movdqa	%xmm0, 0x10(%rcx)
1407c677f: 66 0f 7f 49 20              	movdqa	%xmm1, 0x20(%rcx)
1407c6784: 66 0f 7f 51 30              	movdqa	%xmm2, 0x30(%rcx)
1407c6789: 48 8b 95 f0 01 00 00        	movq	0x1f0(%rbp), %rdx
1407c6790: 0f b6 42 fc                 	movzbl	-0x4(%rdx), %eax
1407c6794: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
1407c679b: ff c8                       	decl	%eax
1407c679d: 3c 03                       	cmpb	$0x3, %al
1407c679f: 0f 83 c5 08 00 00           	jae	0x1407c706a <.text+0x7c606a>
1407c67a5: 0f b6 c0                    	movzbl	%al, %eax
1407c67a8: 48 8d 0d 95 06 2c 02        	leaq	0x22c0695(%rip), %rcx   # 0x142a86e44
1407c67af: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1407c67b3: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1407c67ba: 48 8d 85 a3 01 00 00        	leaq	0x1a3(%rbp), %rax
1407c67c1: c7 40 08 00 00 00 00        	movl	$0x0, 0x8(%rax)
1407c67c8: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1407c67cf: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1407c67d6: 48 8b 8d a7 01 00 00        	movq	0x1a7(%rbp), %rcx
1407c67dd: 48 89 8d 57 01 00 00        	movq	%rcx, 0x157(%rbp)
1407c67e4: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
1407c67eb: 48 8d 4a e8                 	leaq	-0x18(%rdx), %rcx
1407c67ef: 48 8d 15 8a 05 2c 02        	leaq	0x22c058a(%rip), %rdx   # 0x142a86d80
1407c67f6: e8 b5 2b 07 00              	callq	0x1408393b0 <.text+0x8383b0>
1407c67fb: 8b 00                       	movl	(%rax), %eax
1407c67fd: 89 85 10 02 00 00           	movl	%eax, 0x210(%rbp)
1407c6803: ba 0c 00 00 00              	movl	$0xc, %edx
1407c6808: 48 8d 0d 61 aa 45 02        	leaq	0x245aa61(%rip), %rcx   # 0x142c21270
1407c680f: e8 6c 65 4d 00              	callq	0x140c9cd80 <.text+0xc9bd80>
1407c6814: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1407c681b: 66 0f 6f 45 30              	movdqa	0x30(%rbp), %xmm0
1407c6820: 66 0f 6f 4d 40              	movdqa	0x40(%rbp), %xmm1
1407c6825: 66 0f 6f 55 50              	movdqa	0x50(%rbp), %xmm2
1407c682a: 66 0f 7f 95 20 01 00 00     	movdqa	%xmm2, 0x120(%rbp)
1407c6832: 66 0f 7f 8d 10 01 00 00     	movdqa	%xmm1, 0x110(%rbp)
1407c683a: 66 0f 7f 85 00 01 00 00     	movdqa	%xmm0, 0x100(%rbp)
1407c6842: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
1407c6849: 48 8b 8d 57 01 00 00        	movq	0x157(%rbp), %rcx
1407c6850: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
1407c6857: 48 89 8d d7 00 00 00        	movq	%rcx, 0xd7(%rbp)
1407c685e: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407c6865: 48 8b 88 c0 00 00 00        	movq	0xc0(%rax), %rcx
1407c686c: 48 89 8d f8 01 00 00        	movq	%rcx, 0x1f8(%rbp)
1407c6873: 48 3b 88 b0 00 00 00        	cmpq	0xb0(%rax), %rcx
1407c687a: 75 13                       	jne	0x1407c688f <.text+0x7c588f>
1407c687c: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
1407c6883: 48 8d 15 0e 05 2c 02        	leaq	0x22c050e(%rip), %rdx   # 0x142a86d98
1407c688a: e8 21 4b 06 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407c688f: 48 8d 05 ae 05 2c 02        	leaq	0x22c05ae(%rip), %rax   # 0x142a86e44
1407c6896: 4c 8b 85 18 02 00 00        	movq	0x218(%rbp), %r8
1407c689d: 49 01 c0                    	addq	%rax, %r8
1407c68a0: 48 69 95 28 02 00 00 e8 03 00 00    	imulq	$0x3e8, 0x228(%rbp), %rdx # imm = 0x3E8
1407c68ab: 48 03 95 70 01 00 00        	addq	0x170(%rbp), %rdx
1407c68b2: 4c 8b 8d 00 02 00 00        	movq	0x200(%rbp), %r9
1407c68b9: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407c68c0: 4c 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %r10
1407c68c7: 49 69 ca 60 01 00 00        	imulq	$0x160, %r10, %rcx      # imm = 0x160
1407c68ce: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407c68d6: 48 c7 44 08 10 01 00 00 00  	movq	$0x1, 0x10(%rax,%rcx)
1407c68df: 48 89 54 08 18              	movq	%rdx, 0x18(%rax,%rcx)
1407c68e4: 48 c7 44 08 20 01 00 00 00  	movq	$0x1, 0x20(%rax,%rcx)
1407c68ed: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
1407c68f4: 48 89 54 08 28              	movq	%rdx, 0x28(%rax,%rcx)
1407c68f9: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407c6901: 48 8b 95 84 00 00 00        	movq	0x84(%rbp), %rdx
1407c6908: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407c690d: 8b 95 8c 00 00 00           	movl	0x8c(%rbp), %edx
1407c6913: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407c6917: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407c691f: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
1407c6926: 0f 10 8d b0 01 00 00        	movups	0x1b0(%rbp), %xmm1
1407c692d: 0f 10 95 c0 01 00 00        	movups	0x1c0(%rbp), %xmm2
1407c6934: 0f 10 9d cc 01 00 00        	movups	0x1cc(%rbp), %xmm3
1407c693b: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407c6940: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407c6945: 0f 11 54 08 64              	movups	%xmm2, 0x64(%rax,%rcx)
1407c694a: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407c694f: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407c695a: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
1407c6961: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407c6969: 8b 95 98 00 00 00           	movl	0x98(%rbp), %edx
1407c696f: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407c6976: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407c6981: 8b 95 a4 00 00 00           	movl	0xa4(%rbp), %edx
1407c6987: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407c698e: 48 8b 95 9c 00 00 00        	movq	0x9c(%rbp), %rdx
1407c6995: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407c699d: 4c 89 84 08 a0 00 00 00     	movq	%r8, 0xa0(%rax,%rcx)
1407c69a5: 48 c7 84 08 a8 00 00 00 29 00 00 00 	movq	$0x29, 0xa8(%rax,%rcx)
1407c69b1: 48 8d 15 94 a8 45 02        	leaq	0x245a894(%rip), %rdx   # 0x142c2124c
1407c69b8: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407c69c0: 48 c7 84 08 b8 00 00 00 24 00 00 00 	movq	$0x24, 0xb8(%rax,%rcx)
1407c69cc: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
1407c69d3: 0f 28 8d 10 01 00 00        	movaps	0x110(%rbp), %xmm1
1407c69da: 0f 28 95 20 01 00 00        	movaps	0x120(%rbp), %xmm2
1407c69e1: 0f 29 94 08 e0 00 00 00     	movaps	%xmm2, 0xe0(%rax,%rcx)
1407c69e9: 0f 29 8c 08 d0 00 00 00     	movaps	%xmm1, 0xd0(%rax,%rcx)
1407c69f1: 0f 29 84 08 c0 00 00 00     	movaps	%xmm0, 0xc0(%rax,%rcx)
1407c69f9: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407c6a04: 8b 95 10 02 00 00           	movl	0x210(%rbp), %edx
1407c6a0a: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407c6a11: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407c6a1c: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1407c6a23: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407c6a2b: 0f 10 85 e8 00 00 00        	movups	0xe8(%rbp), %xmm0
1407c6a32: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407c6a3a: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407c6a42: 0f b6 95 4f 01 00 00        	movzbl	0x14f(%rbp), %edx
1407c6a49: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407c6a50: 0f b7 95 4d 01 00 00        	movzwl	0x14d(%rbp), %edx
1407c6a57: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407c6a5f: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407c6a6a: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1407c6a71: 0f 10 8d 60 01 00 00        	movups	0x160(%rbp), %xmm1
1407c6a78: 0f 11 84 08 1c 01 00 00     	movups	%xmm0, 0x11c(%rax,%rcx)
1407c6a80: 0f 11 8c 08 2c 01 00 00     	movups	%xmm1, 0x12c(%rax,%rcx)
1407c6a88: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407c6a90: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
1407c6a97: 4c 8b 85 d7 00 00 00        	movq	0xd7(%rbp), %r8
1407c6a9e: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407c6aa6: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407c6aae: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407c6ab6: 0f b6 95 27 02 00 00        	movzbl	0x227(%rbp), %edx
1407c6abd: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407c6ac4: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407c6ace: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
1407c6ad2: 4c 8b 45 27                 	movq	0x27(%rbp), %r8
1407c6ad6: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407c6ade: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407c6ae6: 49 ff c2                    	incq	%r10
1407c6ae9: 4d 89 91 c0 00 00 00        	movq	%r10, 0xc0(%r9)
1407c6af0: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407c6af3: 0f 59 cf                    	mulps	%xmm7, %xmm1
1407c6af6: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
1407c6afa: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407c6afe: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407c6b01: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
1407c6b05: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407c6b09: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407c6b0d: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1407c6b11: 0f 5c ca                    	subps	%xmm2, %xmm1
1407c6b14: 0f 59 0d a5 65 20 02        	mulps	0x22065a5(%rip), %xmm1  # 0x1429cd0c0
1407c6b1b: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407c6b1e: 0f 59 05 9b 65 20 02        	mulps	0x220659b(%rip), %xmm0  # 0x1429cd0c0
1407c6b25: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1407c6b29: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407c6b2d: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407c6b30: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1407c6b34: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407c6b38: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407c6b3c: 0f 59 c7                    	mulps	%xmm7, %xmm0
1407c6b3f: 0f 58 c1                    	addps	%xmm1, %xmm0
1407c6b42: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407c6b45: 0f c6 cf d6                 	shufps	$0xd6, %xmm7, %xmm1     # xmm1 = xmm1[2,1],xmm7[1,3]
1407c6b49: 0f 59 0d 50 ee 15 02        	mulps	0x215ee50(%rip), %xmm1  # 0x1429259a0
1407c6b50: 0f 28 d7                    	movaps	%xmm7, %xmm2
1407c6b53: 0f c6 d7 c5                 	shufps	$0xc5, %xmm7, %xmm2     # xmm2 = xmm2[1,1],xmm7[0,3]
1407c6b57: 0f 59 15 52 7b 16 02        	mulps	0x2167b52(%rip), %xmm2  # 0x14292e6b0
1407c6b5e: 0f 5c d1                    	subps	%xmm1, %xmm2
1407c6b61: 0f 58 ff                    	addps	%xmm7, %xmm7
1407c6b64: 0f c6 ff ff                 	shufps	$0xff, %xmm7, %xmm7     # xmm7 = xmm7[3,3,3,3]
1407c6b68: 0f 59 fa                    	mulps	%xmm2, %xmm7
1407c6b6b: 0f 58 f8                    	addps	%xmm0, %xmm7
1407c6b6e: 0f c6 ff e8                 	shufps	$0xe8, %xmm7, %xmm7     # xmm7 = xmm7[0,2,2,3]
1407c6b72: 0f 13 bd 40 01 00 00        	movlps	%xmm7, 0x140(%rbp)
1407c6b79: f3 0f 10 55 78              	movss	0x78(%rbp), %xmm2
1407c6b7e: f3 0f 58 55 7c              	addss	0x7c(%rbp), %xmm2
1407c6b83: f3 0f 59 15 e5 ec 15 02     	mulss	0x215ece5(%rip), %xmm2  # 0x142925870
1407c6b8b: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1407c6b92: 48 8d 55 68                 	leaq	0x68(%rbp), %rdx
1407c6b96: e8 45 62 4d 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407c6b9b: f3 0f 10 bd 54 01 00 00     	movss	0x154(%rbp), %xmm7
1407c6ba3: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c6baa: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
1407c6bb1: e8 0a 62 4d 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407c6bb6: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1407c6bbd: 48 8d 55 60                 	leaq	0x60(%rbp), %rdx
1407c6bc1: e8 9a 1a 4d 00              	callq	0x140c98660 <.text+0xc97660>
1407c6bc6: f3 44 0f 10 a5 08 01 00 00  	movss	0x108(%rbp), %xmm12
1407c6bcf: f3 0f 7e 85 a0 01 00 00     	movq	0x1a0(%rbp), %xmm0
1407c6bd7: 66 0f 7f 85 70 01 00 00     	movdqa	%xmm0, 0x170(%rbp)
1407c6bdf: f3 44 0f 10 9d a8 01 00 00  	movss	0x1a8(%rbp), %xmm11
1407c6be8: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c6bef: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
1407c6bf6: e8 c5 61 4d 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407c6bfb: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407c6bfe: f3 41 0f 59 c5              	mulss	%xmm13, %xmm0
1407c6c03: f3 0f 10 8d 98 01 00 00     	movss	0x198(%rbp), %xmm1
1407c6c0b: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407c6c0f: 0f 14 c7                    	unpcklps	%xmm7, %xmm0            # xmm0 = xmm0[0],xmm7[0],xmm0[1],xmm7[1]
1407c6c12: 0f 58 45 10                 	addps	0x10(%rbp), %xmm0
1407c6c16: f3 45 0f 59 dc              	mulss	%xmm12, %xmm11
1407c6c1b: f3 45 0f 59 de              	mulss	%xmm14, %xmm11
1407c6c20: f3 44 0f 58 d9              	addss	%xmm1, %xmm11
1407c6c25: f3 41 0f 12 cc              	movsldup	%xmm12, %xmm1           # xmm1 = xmm12[0,0,2,2]
1407c6c2a: 0f 59 8d 70 01 00 00        	mulps	0x170(%rbp), %xmm1
1407c6c31: f3 0f 10 95 a8 01 00 00     	movss	0x1a8(%rbp), %xmm2
1407c6c39: f3 41 0f 59 d7              	mulss	%xmm15, %xmm2
1407c6c3e: f3 41 0f 58 d3              	addss	%xmm11, %xmm2
1407c6c43: 41 0f 59 c8                 	mulps	%xmm8, %xmm1
1407c6c47: 0f 58 c8                    	addps	%xmm0, %xmm1
1407c6c4a: f2 0f 10 85 a0 01 00 00     	movsd	0x1a0(%rbp), %xmm0
1407c6c52: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
1407c6c56: 0f 58 c1                    	addps	%xmm1, %xmm0
1407c6c59: 0f 13 45 d8                 	movlps	%xmm0, -0x28(%rbp)
1407c6c5d: f3 0f 11 55 e0              	movss	%xmm2, -0x20(%rbp)
1407c6c62: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407c6c69: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407c6c6d: fe c8                       	decb	%al
1407c6c6f: 3c 03                       	cmpb	$0x3, %al
1407c6c71: 0f 83 0d 04 00 00           	jae	0x1407c7084 <.text+0x7c6084>
1407c6c77: 0f b6 c0                    	movzbl	%al, %eax
1407c6c7a: 48 8d 0d cf 01 2c 02        	leaq	0x22c01cf(%rip), %rcx   # 0x142a86e50
1407c6c81: f3 0f 10 3c 81              	movss	(%rcx,%rax,4), %xmm7
1407c6c86: 48 8d 8d 88 01 00 00        	leaq	0x188(%rbp), %rcx
1407c6c8d: e8 3e 19 8c ff              	callq	0x1400885d0 <.text+0x875d0>
1407c6c92: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1407c6c99: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1407c6ca0: 48 8d 55 60                 	leaq	0x60(%rbp), %rdx
1407c6ca4: e8 b7 19 4d 00              	callq	0x140c98660 <.text+0xc97660>
1407c6ca9: 66 0f 6e 85 40 01 00 00     	movd	0x140(%rbp), %xmm0
1407c6cb1: 66 0f 6e 8d 44 01 00 00     	movd	0x144(%rbp), %xmm1
1407c6cb9: f3 0f 11 7c 24 38           	movss	%xmm7, 0x38(%rsp)
1407c6cbf: 66 0f 7e 4c 24 28           	movd	%xmm1, 0x28(%rsp)
1407c6cc5: 66 0f 7e 44 24 20           	movd	%xmm0, 0x20(%rsp)
1407c6ccb: c7 44 24 30 b8 1e 85 3f     	movl	$0x3f851eb8, 0x30(%rsp) # imm = 0x3F851EB8
1407c6cd3: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c6cda: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1407c6ce1: 4c 8d 45 d8                 	leaq	-0x28(%rbp), %r8
1407c6ce5: 4c 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %r9
1407c6cec: e8 8f f9 ab 00              	callq	0x141286680 <.text+0x1285680>
1407c6cf1: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407c6cf8: 48 8b 88 f0 00 00 00        	movq	0xf0(%rax), %rcx
1407c6cff: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
1407c6d06: 48 3b 88 e0 00 00 00        	cmpq	0xe0(%rax), %rcx
1407c6d0d: 75 1a                       	jne	0x1407c6d29 <.text+0x7c5d29>
1407c6d0f: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
1407c6d16: 48 81 c1 e0 00 00 00        	addq	$0xe0, %rcx
1407c6d1d: 48 8d 15 8c 00 2c 02        	leaq	0x22c008c(%rip), %rdx   # 0x142a86db0
1407c6d24: e8 c7 48 06 00              	callq	0x14082b5f0 <.text+0x82a5f0>
1407c6d29: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
1407c6d30: 48 8b 82 e8 00 00 00        	movq	0xe8(%rdx), %rax
1407c6d37: 4c 8b 85 70 01 00 00        	movq	0x170(%rbp), %r8
1407c6d3e: 4b 8d 0c 40                 	leaq	(%r8,%r8,2), %rcx
1407c6d42: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407c6d46: f3 0f 6f 85 a0 01 00 00     	movdqu	0x1a0(%rbp), %xmm0
1407c6d4e: f3 0f 6f 8d b0 01 00 00     	movdqu	0x1b0(%rbp), %xmm1
1407c6d56: f3 0f 6f 95 c0 01 00 00     	movdqu	0x1c0(%rbp), %xmm2
1407c6d5e: f3 0f 7f 54 08 20           	movdqu	%xmm2, 0x20(%rax,%rcx)
1407c6d64: f3 0f 7f 4c 08 10           	movdqu	%xmm1, 0x10(%rax,%rcx)
1407c6d6a: f3 0f 7f 04 08              	movdqu	%xmm0, (%rax,%rcx)
1407c6d6f: 49 ff c0                    	incq	%r8
1407c6d72: 4c 89 82 f0 00 00 00        	movq	%r8, 0xf0(%rdx)
1407c6d79: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
1407c6d80: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1407c6d87: 8b 95 ec 01 00 00           	movl	0x1ec(%rbp), %edx
1407c6d8d: e9 20 ee ff ff              	jmp	0x1407c5bb2 <.text+0x7c4bb2>
1407c6d92: 41 89 f9                    	movl	%edi, %r9d
1407c6d95: 4c 89 e1                    	movq	%r12, %rcx
1407c6d98: e9 33 ec ff ff              	jmp	0x1407c59d0 <.text+0x7c49d0>
1407c6d9d: 80 bd 26 02 00 00 00        	cmpb	$0x0, 0x226(%rbp)
1407c6da4: 74 4b                       	je	0x1407c6df1 <.text+0x7c5df1>
1407c6da6: e8 85 13 45 00              	callq	0x140c18130 <.text+0xc17130>
1407c6dab: 48 89 c1                    	movq	%rax, %rcx
1407c6dae: 48 8b 00                    	movq	(%rax), %rax
1407c6db1: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c6db5: 0f 85 e0 00 00 00           	jne	0x1407c6e9b <.text+0x7c5e9b>
1407c6dbb: 48 83 c1 08                 	addq	$0x8, %rcx
1407c6dbf: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407c6dc3: 0f 85 df 01 00 00           	jne	0x1407c6fa8 <.text+0x7c5fa8>
1407c6dc9: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407c6dd0: 48 89 8d e0 01 00 00        	movq	%rcx, 0x1e0(%rbp)
1407c6dd7: 48 83 c1 08                 	addq	$0x8, %rcx
1407c6ddb: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
1407c6de2: e8 09 03 45 00              	callq	0x140c170f0 <.text+0xc160f0>
1407c6de7: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
1407c6dee: 48 ff 00                    	incq	(%rax)
1407c6df1: 0f 28 b5 40 02 00 00        	movaps	0x240(%rbp), %xmm6
1407c6df8: 0f 28 bd 50 02 00 00        	movaps	0x250(%rbp), %xmm7
1407c6dff: 44 0f 28 85 60 02 00 00     	movaps	0x260(%rbp), %xmm8
1407c6e07: 44 0f 28 8d 70 02 00 00     	movaps	0x270(%rbp), %xmm9
1407c6e0f: 44 0f 28 95 80 02 00 00     	movaps	0x280(%rbp), %xmm10
1407c6e17: 44 0f 28 9d 90 02 00 00     	movaps	0x290(%rbp), %xmm11
1407c6e1f: 44 0f 28 a5 a0 02 00 00     	movaps	0x2a0(%rbp), %xmm12
1407c6e27: 44 0f 28 ad b0 02 00 00     	movaps	0x2b0(%rbp), %xmm13
1407c6e2f: 44 0f 28 b5 c0 02 00 00     	movaps	0x2c0(%rbp), %xmm14
1407c6e37: 44 0f 28 bd d0 02 00 00     	movaps	0x2d0(%rbp), %xmm15
1407c6e3f: 48 81 c4 68 03 00 00        	addq	$0x368, %rsp            # imm = 0x368
1407c6e46: 5b                          	popq	%rbx
1407c6e47: 5f                          	popq	%rdi
1407c6e48: 5e                          	popq	%rsi
1407c6e49: 41 5c                       	popq	%r12
1407c6e4b: 41 5d                       	popq	%r13
1407c6e4d: 41 5e                       	popq	%r14
1407c6e4f: 41 5f                       	popq	%r15
1407c6e51: 5d                          	popq	%rbp
1407c6e52: c3                          	retq
1407c6e53: 48 8d 0d 8e a4 45 02        	leaq	0x245a48e(%rip), %rcx   # 0x142c212e8
1407c6e5a: e8 b1 24 11 02              	callq	0x1428d9310 <.text+0x28d8310>
1407c6e5f: e9 38 02 00 00              	jmp	0x1407c709c <.text+0x7c609c>
1407c6e64: 48 8d 0d 6d fa 2b 02        	leaq	0x22bfa6d(%rip), %rcx   # 0x142a868d8
1407c6e6b: 4c 8d 05 b6 fd 2b 02        	leaq	0x22bfdb6(%rip), %r8    # 0x142a86c28
1407c6e72: ba 16 00 00 00              	movl	$0x16, %edx
1407c6e77: e8 b4 24 11 02              	callq	0x1428d9330 <.text+0x28d8330>
1407c6e7c: e9 1b 02 00 00              	jmp	0x1407c709c <.text+0x7c609c>
1407c6e81: 48 85 c0                    	testq	%rax, %rax
1407c6e84: 75 2d                       	jne	0x1407c6eb3 <.text+0x7c5eb3>
1407c6e86: 31 d2                       	xorl	%edx, %edx
1407c6e88: e8 f3 b6 12 02              	callq	0x1428f2580 <.text+0x28f1580>
1407c6e8d: 48 89 c1                    	movq	%rax, %rcx
1407c6e90: 48 85 c0                    	testq	%rax, %rax
1407c6e93: 0f 85 13 ea ff ff           	jne	0x1407c58ac <.text+0x7c48ac>
1407c6e99: eb 18                       	jmp	0x1407c6eb3 <.text+0x7c5eb3>
1407c6e9b: 48 85 c0                    	testq	%rax, %rax
1407c6e9e: 75 13                       	jne	0x1407c6eb3 <.text+0x7c5eb3>
1407c6ea0: 31 d2                       	xorl	%edx, %edx
1407c6ea2: e8 d9 b6 12 02              	callq	0x1428f2580 <.text+0x28f1580>
1407c6ea7: 48 89 c1                    	movq	%rax, %rcx
1407c6eaa: 48 85 c0                    	testq	%rax, %rax
1407c6ead: 0f 85 0c ff ff ff           	jne	0x1407c6dbf <.text+0x7c5dbf>
1407c6eb3: 48 8d 0d 0e a5 45 02        	leaq	0x245a50e(%rip), %rcx   # 0x142c213c8
1407c6eba: e8 11 12 11 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407c6ebf: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
1407c6ec6: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1407c6ecd: 48 8d 05 5c e2 ff ff        	leaq	-0x1da4(%rip), %rax     # 0x1407c5130 <.text+0x7c4130>
1407c6ed4: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1407c6edb: 48 8d 05 a6 fa 2b 02        	leaq	0x22bfaa6(%rip), %rax   # 0x142a86988
1407c6ee2: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1407c6ee9: 48 c7 85 a8 01 00 00 01 00 00 00    	movq	$0x1, 0x1a8(%rbp)
1407c6ef4: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
1407c6eff: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
1407c6f06: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
1407c6f0d: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
1407c6f18: 48 8d 15 79 fa 2b 02        	leaq	0x22bfa79(%rip), %rdx   # 0x142a86998
1407c6f1f: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c6f26: e8 65 24 11 02              	callq	0x1428d9390 <.text+0x28d8390>
1407c6f2b: e9 6c 01 00 00              	jmp	0x1407c709c <.text+0x7c609c>
1407c6f30: 48 8d 85 e8 00 00 00        	leaq	0xe8(%rbp), %rax
1407c6f37: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1407c6f3e: 48 8d 05 eb e1 ff ff        	leaq	-0x1e15(%rip), %rax     # 0x1407c5130 <.text+0x7c4130>
1407c6f45: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1407c6f4c: 48 8d 05 35 fa 2b 02        	leaq	0x22bfa35(%rip), %rax   # 0x142a86988
1407c6f53: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1407c6f5a: 48 c7 85 a8 01 00 00 01 00 00 00    	movq	$0x1, 0x1a8(%rbp)
1407c6f65: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
1407c6f70: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
1407c6f77: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
1407c6f7e: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
1407c6f89: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
1407c6f90: 48 8d 15 19 fa 2b 02        	leaq	0x22bfa19(%rip), %rdx   # 0x142a869b0
1407c6f97: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c6f9e: e8 ed 23 11 02              	callq	0x1428d9390 <.text+0x28d8390>
1407c6fa3: e9 f4 00 00 00              	jmp	0x1407c709c <.text+0x7c609c>
1407c6fa8: 48 8d 0d 89 a4 45 02        	leaq	0x245a489(%rip), %rcx   # 0x142c21438
1407c6faf: e8 9c 22 11 02              	callq	0x1428d9250 <.text+0x28d8250>
1407c6fb4: 48 8d 05 6d a7 2c 02        	leaq	0x22ca76d(%rip), %rax   # 0x142a91728
1407c6fbb: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1407c6fc2: 48 c7 85 a8 01 00 00 01 00 00 00    	movq	$0x1, 0x1a8(%rbp)
1407c6fcd: 48 c7 85 b0 01 00 00 08 00 00 00    	movq	$0x8, 0x1b0(%rbp)
1407c6fd8: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1407c6fdc: f3 0f 7f 85 b8 01 00 00     	movdqu	%xmm0, 0x1b8(%rbp)
1407c6fe4: 48 8d 15 4d a7 2c 02        	leaq	0x22ca74d(%rip), %rdx   # 0x142a91738
1407c6feb: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c6ff2: e8 99 23 11 02              	callq	0x1428d9390 <.text+0x28d8390>
1407c6ff7: e9 a0 00 00 00              	jmp	0x1407c709c <.text+0x7c609c>
1407c6ffc: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
1407c7003: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1407c700a: 48 8d 05 df 5e a9 00        	leaq	0xa95edf(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
1407c7011: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1407c7018: 48 8d 05 61 ac 45 02        	leaq	0x245ac61(%rip), %rax   # 0x142c21c80
1407c701f: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1407c7026: 48 c7 85 a8 01 00 00 01 00 00 00    	movq	$0x1, 0x1a8(%rbp)
1407c7031: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
1407c703c: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
1407c7043: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
1407c704a: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
1407c7055: 48 8d 15 4c ac 45 02        	leaq	0x245ac4c(%rip), %rdx   # 0x142c21ca8
1407c705c: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c7063: e8 28 23 11 02              	callq	0x1428d9390 <.text+0x28d8390>
1407c7068: eb 32                       	jmp	0x1407c709c <.text+0x7c609c>
1407c706a: 48 8d 0d 57 fd 2b 02        	leaq	0x22bfd57(%rip), %rcx   # 0x142a86dc8
1407c7071: 4c 8d 05 90 fd 2b 02        	leaq	0x22bfd90(%rip), %r8    # 0x142a86e08
1407c7078: ba 28 00 00 00              	movl	$0x28, %edx
1407c707d: e8 ae 23 11 02              	callq	0x1428d9430 <.text+0x28d8430>
1407c7082: eb 18                       	jmp	0x1407c709c <.text+0x7c609c>
1407c7084: 48 8d 0d 3d fd 2b 02        	leaq	0x22bfd3d(%rip), %rcx   # 0x142a86dc8
1407c708b: 4c 8d 05 5e fd 2b 02        	leaq	0x22bfd5e(%rip), %r8    # 0x142a86df0
1407c7092: ba 28 00 00 00              	movl	$0x28, %edx
1407c7097: e8 94 23 11 02              	callq	0x1428d9430 <.text+0x28d8430>
1407c709c: 0f 0b                       	ud2
1407c709e: 66 90                       	nop
1407c70a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c70a5: 55                          	pushq	%rbp
1407c70a6: 41 57                       	pushq	%r15
1407c70a8: 41 56                       	pushq	%r14
1407c70aa: 41 55                       	pushq	%r13
1407c70ac: 41 54                       	pushq	%r12
1407c70ae: 56                          	pushq	%rsi
1407c70af: 57                          	pushq	%rdi
1407c70b0: 53                          	pushq	%rbx
1407c70b1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1407c70b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c70bf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1407c70c5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1407c70cb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1407c70d1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1407c70d7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1407c70e0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1407c70e9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1407c70f2: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1407c70fb: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1407c7103: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
1407c710c: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1407c7113: 48 ff 00                    	incq	(%rax)
1407c7116: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1407c711e: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1407c7126: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1407c712f: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1407c7138: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1407c7141: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1407c714a: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1407c7150: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1407c7156: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1407c715c: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1407c7162: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1407c7169: 5b                          	popq	%rbx
1407c716a: 5f                          	popq	%rdi
1407c716b: 5e                          	popq	%rsi
1407c716c: 41 5c                       	popq	%r12
1407c716e: 41 5d                       	popq	%r13
1407c7170: 41 5e                       	popq	%r14
1407c7172: 41 5f                       	popq	%r15
1407c7174: 5d                          	popq	%rbp
1407c7175: c3                          	retq
1407c7176: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407c7180: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c7185: 55                          	pushq	%rbp
1407c7186: 41 57                       	pushq	%r15
1407c7188: 41 56                       	pushq	%r14
1407c718a: 41 55                       	pushq	%r13
1407c718c: 41 54                       	pushq	%r12
1407c718e: 56                          	pushq	%rsi
1407c718f: 57                          	pushq	%rdi
1407c7190: 53                          	pushq	%rbx
1407c7191: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1407c7198: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c719f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1407c71a5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1407c71ab: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1407c71b1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1407c71b7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1407c71c0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1407c71c9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1407c71d2: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1407c71db: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1407c71e3: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
1407c71ec: 80 bd 26 02 00 00 00        	cmpb	$0x0, 0x226(%rbp)
1407c71f3: 74 3d                       	je	0x1407c7232 <.text+0x7c6232>
1407c71f5: e8 36 0f 45 00              	callq	0x140c18130 <.text+0xc17130>
1407c71fa: 48 89 c6                    	movq	%rax, %rsi
1407c71fd: 48 8b 00                    	movq	(%rax), %rax
1407c7200: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c7204: 0f 85 88 00 00 00           	jne	0x1407c7292 <.text+0x7c6292>
1407c720a: 48 83 c6 08                 	addq	$0x8, %rsi
1407c720e: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
1407c7212: 0f 85 a1 00 00 00           	jne	0x1407c72b9 <.text+0x7c62b9>
1407c7218: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
1407c721f: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
1407c7223: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
1407c722a: e8 c1 fe 44 00              	callq	0x140c170f0 <.text+0xc160f0>
1407c722f: 48 ff 06                    	incq	(%rsi)
1407c7232: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1407c723a: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1407c7242: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1407c724b: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1407c7254: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1407c725d: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1407c7266: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1407c726c: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1407c7272: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1407c7278: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1407c727e: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1407c7285: 5b                          	popq	%rbx
1407c7286: 5f                          	popq	%rdi
1407c7287: 5e                          	popq	%rsi
1407c7288: 41 5c                       	popq	%r12
1407c728a: 41 5d                       	popq	%r13
1407c728c: 41 5e                       	popq	%r14
1407c728e: 41 5f                       	popq	%r15
1407c7290: 5d                          	popq	%rbp
1407c7291: c3                          	retq
1407c7292: 48 85 c0                    	testq	%rax, %rax
1407c7295: 75 16                       	jne	0x1407c72ad <.text+0x7c62ad>
1407c7297: 48 89 f1                    	movq	%rsi, %rcx
1407c729a: 31 d2                       	xorl	%edx, %edx
1407c729c: e8 df b2 12 02              	callq	0x1428f2580 <.text+0x28f1580>
1407c72a1: 48 89 c6                    	movq	%rax, %rsi
1407c72a4: 48 85 c0                    	testq	%rax, %rax
1407c72a7: 0f 85 61 ff ff ff           	jne	0x1407c720e <.text+0x7c620e>
1407c72ad: 48 8d 0d 14 a1 45 02        	leaq	0x245a114(%rip), %rcx   # 0x142c213c8
1407c72b4: e8 17 0e 11 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407c72b9: 48 8d 0d 78 a1 45 02        	leaq	0x245a178(%rip), %rcx   # 0x142c21438
1407c72c0: e8 8b 1f 11 02              	callq	0x1428d9250 <.text+0x28d8250>
1407c72c5: cc                          	int3
1407c72c6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407c72d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c72d5: 55                          	pushq	%rbp
1407c72d6: 41 57                       	pushq	%r15
1407c72d8: 41 56                       	pushq	%r14
1407c72da: 41 55                       	pushq	%r13
1407c72dc: 41 54                       	pushq	%r12
1407c72de: 56                          	pushq	%rsi
1407c72df: 57                          	pushq	%rdi
1407c72e0: 53                          	pushq	%rbx
1407c72e1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1407c72e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c72ef: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1407c72f5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1407c72fb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1407c7301: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1407c7307: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1407c7310: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1407c7319: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1407c7322: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1407c732b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1407c7333: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
1407c733c: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
1407c7343: 48 ff 00                    	incq	(%rax)
1407c7346: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1407c734e: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1407c7356: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1407c735f: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1407c7368: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1407c7371: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1407c737a: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1407c7380: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1407c7386: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1407c738c: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1407c7392: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1407c7399: 5b                          	popq	%rbx
1407c739a: 5f                          	popq	%rdi
1407c739b: 5e                          	popq	%rsi
1407c739c: 41 5c                       	popq	%r12
1407c739e: 41 5d                       	popq	%r13
1407c73a0: 41 5e                       	popq	%r14
1407c73a2: 41 5f                       	popq	%r15
1407c73a4: 5d                          	popq	%rbp
1407c73a5: c3                          	retq
1407c73a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407c73b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c73b5: 55                          	pushq	%rbp
1407c73b6: 41 57                       	pushq	%r15
1407c73b8: 41 56                       	pushq	%r14
1407c73ba: 41 55                       	pushq	%r13
1407c73bc: 41 54                       	pushq	%r12
1407c73be: 56                          	pushq	%rsi
1407c73bf: 57                          	pushq	%rdi
1407c73c0: 53                          	pushq	%rbx
1407c73c1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1407c73c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c73cf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1407c73d5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1407c73db: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1407c73e1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1407c73e7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1407c73f0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1407c73f9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1407c7402: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1407c740b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1407c7413: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
1407c741c: 80 bd 37 02 00 00 00        	cmpb	$0x0, 0x237(%rbp)
1407c7423: 74 7a                       	je	0x1407c749f <.text+0x7c649f>
1407c7425: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
1407c742c: f0                          	lock
1407c742d: 48 ff 08                    	decq	(%rax)
1407c7430: 75 0c                       	jne	0x1407c743e <.text+0x7c643e>
1407c7432: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1407c7439: e8 e2 51 39 00              	callq	0x140b5c620 <.text+0xb5b620>
1407c743e: 66 0f 6f b4 24 d0 00 00 00  	movdqa	0xd0(%rsp), %xmm6
1407c7447: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1407c744f: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1407c7458: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1407c7461: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1407c746a: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1407c7473: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1407c7479: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1407c747f: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1407c7485: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1407c748b: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1407c7492: 5b                          	popq	%rbx
1407c7493: 5f                          	popq	%rdi
1407c7494: 5e                          	popq	%rsi
1407c7495: 41 5c                       	popq	%r12
1407c7497: 41 5d                       	popq	%r13
1407c7499: 41 5e                       	popq	%r14
1407c749b: 41 5f                       	popq	%r15
1407c749d: 5d                          	popq	%rbp
1407c749e: c3                          	retq
1407c749f: 66 0f 6f b4 24 d0 00 00 00  	movdqa	0xd0(%rsp), %xmm6
1407c74a8: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1407c74b0: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1407c74b9: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1407c74c2: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1407c74cb: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1407c74d4: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1407c74da: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1407c74e0: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1407c74e6: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1407c74ec: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1407c74f3: 5b                          	popq	%rbx
1407c74f4: 5f                          	popq	%rdi
1407c74f5: 5e                          	popq	%rsi
1407c74f6: 41 5c                       	popq	%r12
1407c74f8: 41 5d                       	popq	%r13
1407c74fa: 41 5e                       	popq	%r14
1407c74fc: 41 5f                       	popq	%r15
1407c74fe: 5d                          	popq	%rbp
1407c74ff: c3                          	retq
