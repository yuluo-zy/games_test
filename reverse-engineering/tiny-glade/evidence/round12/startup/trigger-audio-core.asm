
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409eb280: f0                          	lock
1409eb281: 4d 89 e9                    	movq	%r13, %r9
1409eb284: e8 b7 ba 22 00              	callq	0x140c16d40 <.text+0xc15d40>
1409eb289: 49 89 c6                    	movq	%rax, %r14
1409eb28c: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
1409eb293: 48 ff 00                    	incq	(%rax)
1409eb296: 4c 89 75 f8                 	movq	%r14, -0x8(%rbp)
1409eb29a: 48 c7 45 f0 01 00 00 00     	movq	$0x1, -0x10(%rbp)
1409eb2a2: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1409eb2a9: 48 8b 30                    	movq	(%rax), %rsi
1409eb2ac: f3 0f 10 46 58              	movss	0x58(%rsi), %xmm0
1409eb2b1: 0f 57 c9                    	xorps	%xmm1, %xmm1
1409eb2b4: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
1409eb2b7: 75 44                       	jne	0x1409eb2fd <.text+0x9ea2fd>
1409eb2b9: 7a 42                       	jp	0x1409eb2fd <.text+0x9ea2fd>
1409eb2bb: e8 70 ce 22 00              	callq	0x140c18130 <.text+0xc17130>
1409eb2c0: 48 89 c1                    	movq	%rax, %rcx
1409eb2c3: 48 8b 00                    	movq	(%rax), %rax
1409eb2c6: 48 83 f8 01                 	cmpq	$0x1, %rax
1409eb2ca: 0f 85 17 05 00 00           	jne	0x1409eb7e7 <.text+0x9ea7e7>
1409eb2d0: 48 83 c1 08                 	addq	$0x8, %rcx
1409eb2d4: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409eb2d8: 0f 85 51 05 00 00           	jne	0x1409eb82f <.text+0x9ea82f>
1409eb2de: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409eb2e5: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
1409eb2ec: 48 83 c1 08                 	addq	$0x8, %rcx
1409eb2f0: 4c 89 f2                    	movq	%r14, %rdx
1409eb2f3: e8 f8 bd 22 00              	callq	0x140c170f0 <.text+0xc160f0>
1409eb2f8: e9 6c 04 00 00              	jmp	0x1409eb769 <.text+0x9ea769>
1409eb2fd: 4c 89 75 38                 	movq	%r14, 0x38(%rbp)
1409eb301: 48 83 c6 58                 	addq	$0x58, %rsi
1409eb305: 48 8b 13                    	movq	(%rbx), %rdx
1409eb308: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1409eb30f: 48 89 95 a0 00 00 00        	movq	%rdx, 0xa0(%rbp)
1409eb316: e8 25 ba 2a 00              	callq	0x140c96d40 <.text+0xc95d40>
1409eb31b: 49 8b 1f                    	movq	(%r15), %rbx
1409eb31e: 48 8d 83 c4 00 00 00        	leaq	0xc4(%rbx), %rax
1409eb325: f2 44 0f 10 85 90 00 00 00  	movsd	0x90(%rbp), %xmm8
1409eb32e: f3 0f 10 bd 98 00 00 00     	movss	0x98(%rbp), %xmm7
1409eb336: f6 83 c0 00 00 00 01        	testb	$0x1, 0xc0(%rbx)
1409eb33d: 0f 28 f7                    	movaps	%xmm7, %xmm6
1409eb340: 45 0f 28 d0                 	movaps	%xmm8, %xmm10
1409eb344: 74 11                       	je	0x1409eb357 <.text+0x9ea357>
1409eb346: f3 0f 10 b3 cc 00 00 00     	movss	0xcc(%rbx), %xmm6
1409eb34e: f2 44 0f 10 93 c4 00 00 00  	movsd	0xc4(%rbx), %xmm10
1409eb357: 41 8b 57 1c                 	movl	0x1c(%r15), %edx
1409eb35b: 4d 8b 47 10                 	movq	0x10(%r15), %r8
1409eb35f: 41 89 10                    	movl	%edx, (%r8)
1409eb362: c7 83 c0 00 00 00 01 00 00 00       	movl	$0x1, 0xc0(%rbx)
1409eb36c: 8b 8d 98 00 00 00           	movl	0x98(%rbp), %ecx
1409eb372: 89 48 08                    	movl	%ecx, 0x8(%rax)
1409eb375: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
1409eb37c: 48 89 08                    	movq	%rcx, (%rax)
1409eb37f: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1409eb383: f3 0f 10 c7                 	movss	%xmm7, %xmm0            # xmm0 = xmm7[0],xmm0[1,2,3]
1409eb387: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1409eb38b: f3 0f 10 ce                 	movss	%xmm6, %xmm1            # xmm1 = xmm6[0],xmm1[1,2,3]
1409eb38f: 0f 5c c1                    	subps	%xmm1, %xmm0
1409eb392: 0f 59 c0                    	mulps	%xmm0, %xmm0
1409eb395: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1409eb399: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1409eb39d: f3 41 0f 5c d2              	subss	%xmm10, %xmm2
1409eb3a2: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
1409eb3a6: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1409eb3aa: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1409eb3ae: f3 44 0f 51 ca              	sqrtss	%xmm2, %xmm9
1409eb3b3: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1409eb3b7: f3 0f 5e 06                 	divss	(%rsi), %xmm0
1409eb3bb: f3 44 0f 10 1d 34 03 00 02  	movss	0x2000334(%rip), %xmm11 # 0x1429eb6f8
1409eb3c4: 0f 2e 05 39 a5 f3 01        	ucomiss	0x1f3a539(%rip), %xmm0  # 0x142925904
1409eb3cb: 0f 86 e9 00 00 00           	jbe	0x1409eb4ba <.text+0x9ea4ba>
1409eb3d1: 4c 89 85 88 00 00 00        	movq	%r8, 0x88(%rbp)
1409eb3d8: 89 95 ac 00 00 00           	movl	%edx, 0xac(%rbp)
1409eb3de: 48 8b 07                    	movq	(%rdi), %rax
1409eb3e1: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1409eb3e5: 48 8b 40 10                 	movq	0x10(%rax), %rax
1409eb3e9: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1409eb3ed: 4c 8d 24 52                 	leaq	(%rdx,%rdx,2), %r12
1409eb3f1: 49 01 cc                    	addq	%rcx, %r12
1409eb3f4: 49 01 c4                    	addq	%rax, %r12
1409eb3f7: 45 31 f6                    	xorl	%r14d, %r14d
1409eb3fa: 48 85 c0                    	testq	%rax, %rax
1409eb3fd: 41 0f 95 c6                 	setne	%r14b
1409eb401: 41 bf 1c 00 00 00           	movl	$0x1c, %r15d
1409eb407: 4c 0f 44 f8                 	cmoveq	%rax, %r15
1409eb40b: 49 01 cf                    	addq	%rcx, %r15
1409eb40e: 44 0f 28 25 ba 98 0d 02     	movaps	0x20d98ba(%rip), %xmm12 # 0x142ac4cd0
1409eb416: 44 0f 28 2d c2 98 0d 02     	movaps	0x20d98c2(%rip), %xmm13 # 0x142ac4ce0
1409eb41e: 4c 8d 6d d0                 	leaq	-0x30(%rbp), %r13
1409eb422: 48 8d 7d c0                 	leaq	-0x40(%rbp), %rdi
1409eb426: f3 44 0f 10 35 41 a4 f3 01  	movss	0x1f3a441(%rip), %xmm14 # 0x142925870
1409eb42f: 90                          	nop
1409eb430: 4d 39 e7                    	cmpq	%r12, %r15
1409eb433: 74 78                       	je	0x1409eb4ad <.text+0x9ea4ad>
1409eb435: 4c 89 f8                    	movq	%r15, %rax
1409eb438: 49 83 c7 1c                 	addq	$0x1c, %r15
1409eb43c: 41 ff c6                    	incl	%r14d
1409eb43f: 8b 48 18                    	movl	0x18(%rax), %ecx
1409eb442: f6 c1 01                    	testb	$0x1, %cl
1409eb445: 74 e9                       	je	0x1409eb430 <.text+0x9ea430>
1409eb447: 85 c9                       	testl	%ecx, %ecx
1409eb449: 74 62                       	je	0x1409eb4ad <.text+0x9ea4ad>
1409eb44b: 0f 10 00                    	movups	(%rax), %xmm0
1409eb44e: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
1409eb452: 48 8b 40 10                 	movq	0x10(%rax), %rax
1409eb456: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1409eb45a: 0f 28 45 40                 	movaps	0x40(%rbp), %xmm0
1409eb45e: 41 0f 58 c4                 	addps	%xmm12, %xmm0
1409eb462: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
1409eb466: f2 0f 10 45 50              	movsd	0x50(%rbp), %xmm0
1409eb46b: 41 0f 58 c5                 	addps	%xmm13, %xmm0
1409eb46f: 0f 13 45 50                 	movlps	%xmm0, 0x50(%rbp)
1409eb473: 44 0f 13 55 d0              	movlps	%xmm10, -0x30(%rbp)
1409eb478: f3 0f 11 75 d8              	movss	%xmm6, -0x28(%rbp)
1409eb47d: 8b 85 98 00 00 00           	movl	0x98(%rbp), %eax
1409eb483: 89 45 c8                    	movl	%eax, -0x38(%rbp)
1409eb486: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1409eb48d: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
1409eb491: 48 89 e9                    	movq	%rbp, %rcx
1409eb494: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
1409eb498: 4d 89 e8                    	movq	%r13, %r8
1409eb49b: 49 89 f9                    	movq	%rdi, %r9
1409eb49e: e8 cd 50 85 00              	callq	0x141240570 <.text+0x123f570>
1409eb4a3: 83 7d 00 00                 	cmpl	$0x0, (%rbp)
1409eb4a7: 74 87                       	je	0x1409eb430 <.text+0x9ea430>
1409eb4a9: 45 0f 28 de                 	movaps	%xmm14, %xmm11
1409eb4ad: 8b 95 ac 00 00 00           	movl	0xac(%rbp), %edx
1409eb4b3: 4c 8b 85 88 00 00 00        	movq	0x88(%rbp), %r8
1409eb4ba: 89 d7                       	movl	%edx, %edi
1409eb4bc: 4d 89 c7                    	movq	%r8, %r15
1409eb4bf: 41 89 10                    	movl	%edx, (%r8)
1409eb4c2: f3 44 0f 58 9b d0 00 00 00  	addss	0xd0(%rbx), %xmm11
1409eb4cb: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409eb4ce: 0f 57 c9                    	xorps	%xmm1, %xmm1
1409eb4d1: f3 41 0f 5f cb              	maxss	%xmm11, %xmm1
1409eb4d6: f3 0f 10 35 26 a4 f3 01     	movss	0x1f3a426(%rip), %xmm6  # 0x142925904
1409eb4de: 0f 28 d6                    	movaps	%xmm6, %xmm2
1409eb4e1: f3 0f 5d d1                 	minss	%xmm1, %xmm2
1409eb4e5: f3 0f 11 93 d0 00 00 00     	movss	%xmm2, 0xd0(%rbx)
1409eb4ed: 0f 2e d0                    	ucomiss	%xmm0, %xmm2
1409eb4f0: 76 0a                       	jbe	0x1409eb4fc <.text+0x9ea4fc>
1409eb4f2: 48 89 d9                    	movq	%rbx, %rcx
1409eb4f5: e8 06 be 06 00              	callq	0x140a57300 <.text+0xa56300>
1409eb4fa: eb 28                       	jmp	0x1409eb524 <.text+0x9ea524>
1409eb4fc: 83 3b 01                    	cmpl	$0x1, (%rbx)
1409eb4ff: 75 23                       	jne	0x1409eb524 <.text+0x9ea524>
1409eb501: 48 8d 53 08                 	leaq	0x8(%rbx), %rdx
1409eb505: 48 89 e9                    	movq	%rbp, %rcx
1409eb508: 45 31 c0                    	xorl	%r8d, %r8d
1409eb50b: e8 30 bf a4 00              	callq	0x141437440 <.text+0x1436440>
1409eb510: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409eb51a: 48 39 45 00                 	cmpq	%rax, (%rbp)
1409eb51e: 0f 85 64 03 00 00           	jne	0x1409eb888 <.text+0x9ea888>
1409eb524: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
1409eb52b: 0f b6 82 d0 04 00 00        	movzbl	0x4d0(%rdx), %eax
1409eb532: 48 8d 0d 37 a1 0d 02        	leaq	0x20da137(%rip), %rcx   # 0x142ac5670
1409eb539: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
1409eb53d: 80 bc 02 28 01 00 00 00     	cmpb	$0x0, 0x128(%rdx,%rax)
1409eb545: 74 08                       	je	0x1409eb54f <.text+0x9ea54f>
1409eb547: f3 0f 10 35 35 a4 f3 01     	movss	0x1f3a435(%rip), %xmm6  # 0x142925984
1409eb54f: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1409eb556: f3 45 0f 16 d0              	movshdup	%xmm8, %xmm10           # xmm10 = xmm8[1,1,3,3]
1409eb55b: f3 45 0f 59 c0              	mulss	%xmm8, %xmm8
1409eb560: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
1409eb564: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
1409eb569: f3 41 0f 58 c0              	addss	%xmm8, %xmm0
1409eb56e: f3 0f 59 ff                 	mulss	%xmm7, %xmm7
1409eb572: f3 0f 58 f8                 	addss	%xmm0, %xmm7
1409eb576: f3 0f 51 ff                 	sqrtss	%xmm7, %xmm7
1409eb57a: 4c 8b 30                    	movq	(%rax), %r14
1409eb57d: 49 8b 0e                    	movq	(%r14), %rcx
1409eb580: 48 85 c9                    	testq	%rcx, %rcx
1409eb583: 74 1d                       	je	0x1409eb5a2 <.text+0x9ea5a2>
1409eb585: 0f 28 df                    	movaps	%xmm7, %xmm3
1409eb588: f3 0f 59 de                 	mulss	%xmm6, %xmm3
1409eb58c: 48 83 c1 10                 	addq	$0x10, %rcx
1409eb590: 48 8d 15 29 9c 0d 02        	leaq	0x20d9c29(%rip), %rdx   # 0x142ac51c0
1409eb597: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
1409eb59d: e8 0e 88 06 00              	callq	0x140a53db0 <.text+0xa52db0>
1409eb5a2: f3 44 0f 5e 0e              	divss	(%rsi), %xmm9
1409eb5a7: f3 44 0f 59 0d 40 97 0d 02  	mulss	0x20d9740(%rip), %xmm9  # 0x142ac4cf0
1409eb5b0: f3 41 0f 59 f1              	mulss	%xmm9, %xmm6
1409eb5b5: 80 bb d8 00 00 00 00        	cmpb	$0x0, 0xd8(%rbx)
1409eb5bc: 74 08                       	je	0x1409eb5c6 <.text+0x9ea5c6>
1409eb5be: f3 0f 59 35 e2 8f f4 01     	mulss	0x1f48fe2(%rip), %xmm6  # 0x1429345a8
1409eb5c6: f3 0f 10 83 d4 00 00 00     	movss	0xd4(%rbx), %xmm0
1409eb5ce: 31 c0                       	xorl	%eax, %eax
1409eb5d0: 0f 2e f0                    	ucomiss	%xmm0, %xmm6
1409eb5d3: 0f 92 c0                    	setb	%al
1409eb5d6: f3 0f 5c f0                 	subss	%xmm0, %xmm6
1409eb5da: 48 8d 0d 17 97 0d 02        	leaq	0x20d9717(%rip), %rcx   # 0x142ac4cf8
1409eb5e1: f3 0f 59 34 81              	mulss	(%rcx,%rax,4), %xmm6
1409eb5e6: f3 0f 58 f0                 	addss	%xmm0, %xmm6
1409eb5ea: f3 0f 5d 35 fe 30 f4 01     	minss	0x1f430fe(%rip), %xmm6  # 0x14292e6f0
1409eb5f2: 41 89 3f                    	movl	%edi, (%r15)
1409eb5f5: f3 0f 11 b3 d4 00 00 00     	movss	%xmm6, 0xd4(%rbx)
1409eb5fd: 49 8b 0e                    	movq	(%r14), %rcx
1409eb600: 48 85 c9                    	testq	%rcx, %rcx
1409eb603: 74 19                       	je	0x1409eb61e <.text+0x9ea61e>
1409eb605: 48 83 c1 10                 	addq	$0x10, %rcx
1409eb609: 48 8d 15 bd 9b 0d 02        	leaq	0x20d9bbd(%rip), %rdx   # 0x142ac51cd
1409eb610: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
1409eb616: 0f 28 de                    	movaps	%xmm6, %xmm3
1409eb619: e8 92 87 06 00              	callq	0x140a53db0 <.text+0xa52db0>
1409eb61e: 48 8d 4b 30                 	leaq	0x30(%rbx), %rcx
1409eb622: e8 d9 bc 06 00              	callq	0x140a57300 <.text+0xa56300>
1409eb627: 48 8d 7b 60                 	leaq	0x60(%rbx), %rdi
1409eb62b: 48 89 f9                    	movq	%rdi, %rcx
1409eb62e: e8 cd bc 06 00              	callq	0x140a57300 <.text+0xa56300>
1409eb633: 80 bb d8 00 00 00 00        	cmpb	$0x0, 0xd8(%rbx)
1409eb63a: 74 1f                       	je	0x1409eb65b <.text+0x9ea65b>
1409eb63c: f3 0f 10 93 d4 00 00 00     	movss	0xd4(%rbx), %xmm2
1409eb644: f3 0f 59 15 24 a2 f3 01     	mulss	0x1f3a224(%rip), %xmm2  # 0x142925870
1409eb64c: f3 0f 58 15 b0 a2 f3 01     	addss	0x1f3a2b0(%rip), %xmm2  # 0x142925904
1409eb654: 83 3f 01                    	cmpl	$0x1, (%rdi)
1409eb657: 74 0f                       	je	0x1409eb668 <.text+0x9ea668>
1409eb659: eb 2d                       	jmp	0x1409eb688 <.text+0x9ea688>
1409eb65b: f3 0f 10 15 a1 a2 f3 01     	movss	0x1f3a2a1(%rip), %xmm2  # 0x142925904
1409eb663: 83 3f 01                    	cmpl	$0x1, (%rdi)
1409eb666: 75 20                       	jne	0x1409eb688 <.text+0x9ea688>
1409eb668: 48 8d 53 68                 	leaq	0x68(%rbx), %rdx
1409eb66c: 48 89 e9                    	movq	%rbp, %rcx
1409eb66f: e8 cc b4 a4 00              	callq	0x141436b40 <.text+0x1435b40>
1409eb674: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409eb67e: 48 39 45 00                 	cmpq	%rax, (%rbp)
1409eb682: 0f 85 b3 01 00 00           	jne	0x1409eb83b <.text+0x9ea83b>
1409eb688: 48 8d 8b 90 00 00 00        	leaq	0x90(%rbx), %rcx
1409eb68f: f3 0f 10 05 a5 62 0d 02     	movss	0x20d62a5(%rip), %xmm0  # 0x142ac193c
1409eb697: 41 0f 2e c2                 	ucomiss	%xmm10, %xmm0
1409eb69b: 76 36                       	jbe	0x1409eb6d3 <.text+0x9ea6d3>
1409eb69d: 0f 2e 3d 60 25 00 02        	ucomiss	0x2002560(%rip), %xmm7  # 0x1429edc04
1409eb6a4: 76 2d                       	jbe	0x1409eb6d3 <.text+0x9ea6d3>
1409eb6a6: e8 55 bc 06 00              	callq	0x140a57300 <.text+0xa56300>
1409eb6ab: 49 8b 0e                    	movq	(%r14), %rcx
1409eb6ae: 48 85 c9                    	testq	%rcx, %rcx
1409eb6b1: 74 6f                       	je	0x1409eb722 <.text+0x9ea722>
1409eb6b3: 48 83 c1 10                 	addq	$0x10, %rcx
1409eb6b7: 48 8d 15 1b 9b 0d 02        	leaq	0x20d9b1b(%rip), %rdx   # 0x142ac51d9
1409eb6be: f3 0f 10 1d 3e a2 f3 01     	movss	0x1f3a23e(%rip), %xmm3  # 0x142925904
1409eb6c6: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409eb6cc: e8 df 86 06 00              	callq	0x140a53db0 <.text+0xa52db0>
1409eb6d1: eb 4f                       	jmp	0x1409eb722 <.text+0x9ea722>
1409eb6d3: 83 39 01                    	cmpl	$0x1, (%rcx)
1409eb6d6: 75 29                       	jne	0x1409eb701 <.text+0x9ea701>
1409eb6d8: 48 81 c3 98 00 00 00        	addq	$0x98, %rbx
1409eb6df: 48 89 e9                    	movq	%rbp, %rcx
1409eb6e2: 48 89 da                    	movq	%rbx, %rdx
1409eb6e5: 45 31 c0                    	xorl	%r8d, %r8d
1409eb6e8: e8 53 bd a4 00              	callq	0x141437440 <.text+0x1436440>
1409eb6ed: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1409eb6f7: 48 39 45 00                 	cmpq	%rax, (%rbp)
1409eb6fb: 0f 85 d1 01 00 00           	jne	0x1409eb8d2 <.text+0x9ea8d2>
1409eb701: 49 8b 0e                    	movq	(%r14), %rcx
1409eb704: 48 85 c9                    	testq	%rcx, %rcx
1409eb707: 74 19                       	je	0x1409eb722 <.text+0x9ea722>
1409eb709: 48 83 c1 10                 	addq	$0x10, %rcx
1409eb70d: 48 8d 15 c5 9a 0d 02        	leaq	0x20d9ac5(%rip), %rdx   # 0x142ac51d9
1409eb714: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409eb71a: 0f 57 db                    	xorps	%xmm3, %xmm3
1409eb71d: e8 8e 86 06 00              	callq	0x140a53db0 <.text+0xa52db0>
1409eb722: 80 bd b0 00 00 00 00        	cmpb	$0x0, 0xb0(%rbp)
1409eb729: 74 48                       	je	0x1409eb773 <.text+0x9ea773>
1409eb72b: e8 00 ca 22 00              	callq	0x140c18130 <.text+0xc17130>
1409eb730: 48 89 c1                    	movq	%rax, %rcx
1409eb733: 48 8b 00                    	movq	(%rax), %rax
1409eb736: 48 83 f8 01                 	cmpq	$0x1, %rax
1409eb73a: 0f 85 c1 00 00 00           	jne	0x1409eb801 <.text+0x9ea801>
1409eb740: 48 83 c1 08                 	addq	$0x8, %rcx
1409eb744: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
1409eb748: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409eb74c: 0f 85 dd 00 00 00           	jne	0x1409eb82f <.text+0x9ea82f>
1409eb752: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409eb759: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
