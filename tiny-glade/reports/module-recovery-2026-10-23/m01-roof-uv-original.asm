
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408e4040: 56                          	pushq	%rsi
1408e4041: 48 83 ec 50                 	subq	$0x50, %rsp
1408e4045: 48 89 ce                    	movq	%rcx, %rsi
1408e4048: 0f 28 84 24 80 00 00 00     	movaps	0x80(%rsp), %xmm0
1408e4050: 0f 11 44 24 20              	movups	%xmm0, 0x20(%rsp)
1408e4055: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1408e405a: e8 31 00 00 00              	callq	0x1408e4090 <.text+0x8e3090>
1408e405f: 80 7c 24 38 00              	cmpb	$0x0, 0x38(%rsp)
1408e4064: 74 0c                       	je	0x1408e4072 <.text+0x8e3072>
1408e4066: 0f b6 44 24 39              	movzbl	0x39(%rsp), %eax
1408e406b: 88 46 01                    	movb	%al, 0x1(%rsi)
1408e406e: b0 01                       	movb	$0x1, %al
1408e4070: eb 0d                       	jmp	0x1408e407f <.text+0x8e307f>
1408e4072: f2 0f 10 44 24 3c           	movsd	0x3c(%rsp), %xmm0
1408e4078: f2 0f 11 46 04              	movsd	%xmm0, 0x4(%rsi)
1408e407d: 31 c0                       	xorl	%eax, %eax
1408e407f: 88 06                       	movb	%al, (%rsi)
1408e4081: 48 89 f0                    	movq	%rsi, %rax
1408e4084: 48 83 c4 50                 	addq	$0x50, %rsp
1408e4088: 5e                          	popq	%rsi
1408e4089: c3                          	retq
1408e408a: cc                          	int3
1408e408b: cc                          	int3
1408e408c: cc                          	int3
1408e408d: cc                          	int3
1408e408e: cc                          	int3
1408e408f: cc                          	int3
1408e4090: 41 57                       	pushq	%r15
1408e4092: 41 56                       	pushq	%r14
1408e4094: 41 54                       	pushq	%r12
1408e4096: 56                          	pushq	%rsi
1408e4097: 57                          	pushq	%rdi
1408e4098: 55                          	pushq	%rbp
1408e4099: 53                          	pushq	%rbx
1408e409a: 48 81 ec 50 01 00 00        	subq	$0x150, %rsp            # imm = 0x150
1408e40a1: 48 89 ce                    	movq	%rcx, %rsi
1408e40a4: 4c 8b b4 24 b8 01 00 00     	movq	0x1b8(%rsp), %r14
1408e40ac: 66 0f 7e 54 24 30           	movd	%xmm2, 0x30(%rsp)
1408e40b2: 66 0f 7e 5c 24 34           	movd	%xmm3, 0x34(%rsp)
1408e40b8: 48 89 94 24 d0 00 00 00     	movq	%rdx, 0xd0(%rsp)
1408e40c0: 49 83 7e 48 00              	cmpq	$0x0, 0x48(%r14)
1408e40c5: 0f 84 b3 00 00 00           	je	0x1408e417e <.text+0x8e317e>
1408e40cb: 48 89 d7                    	movq	%rdx, %rdi
1408e40ce: 48 8b 9c 24 b0 01 00 00     	movq	0x1b0(%rsp), %rbx
1408e40d6: 49 8d 4e 50                 	leaq	0x50(%r14), %rcx
1408e40da: 48 8d 94 24 d0 00 00 00     	leaq	0xd0(%rsp), %rdx
1408e40e2: e8 89 cc cc ff              	callq	0x1405b0d70 <.text+0x5afd70>
1408e40e7: 49 8b 4e 30                 	movq	0x30(%r14), %rcx
1408e40eb: 49 8b 56 38                 	movq	0x38(%r14), %rdx
1408e40ef: 49 89 d0                    	movq	%rdx, %r8
1408e40f2: 49 21 c0                    	andq	%rax, %r8
1408e40f5: 48 c1 e8 39                 	shrq	$0x39, %rax
1408e40f9: 66 0f 6e c0                 	movd	%eax, %xmm0
1408e40fd: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408e4101: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408e4106: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408e410b: 48 8d 41 f0                 	leaq	-0x10(%rcx), %rax
1408e410f: 45 31 c9                    	xorl	%r9d, %r9d
1408e4112: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1408e4116: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1408e411c: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1408e4120: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1408e4124: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1408e4129: 45 85 d2                    	testl	%r10d, %r10d
1408e412c: 74 32                       	je	0x1408e4160 <.text+0x8e3160>
1408e412e: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1408e4133: 4d 01 c3                    	addq	%r8, %r11
1408e4136: 49 21 d3                    	andq	%rdx, %r11
1408e4139: 4d 89 de                    	movq	%r11, %r14
1408e413c: 49 c1 e6 04                 	shlq	$0x4, %r14
1408e4140: 49 89 c7                    	movq	%rax, %r15
1408e4143: 4d 29 f7                    	subq	%r14, %r15
1408e4146: 49 39 3f                    	cmpq	%rdi, (%r15)
1408e4149: 74 3a                       	je	0x1408e4185 <.text+0x8e3185>
1408e414b: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1408e414f: 66 45 21 d3                 	andw	%r10w, %r11w
1408e4153: 45 89 da                    	movl	%r11d, %r10d
1408e4156: 75 d6                       	jne	0x1408e412e <.text+0x8e312e>
1408e4158: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1408e4160: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1408e4164: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1408e4169: 45 85 d2                    	testl	%r10d, %r10d
1408e416c: 75 10                       	jne	0x1408e417e <.text+0x8e317e>
1408e416e: 4d 01 c8                    	addq	%r9, %r8
1408e4171: 49 83 c0 10                 	addq	$0x10, %r8
1408e4175: 49 83 c1 10                 	addq	$0x10, %r9
1408e4179: 49 21 d0                    	andq	%rdx, %r8
1408e417c: eb 98                       	jmp	0x1408e4116 <.text+0x8e3116>
1408e417e: 31 ff                       	xorl	%edi, %edi
1408e4180: e9 29 01 00 00              	jmp	0x1408e42ae <.text+0x8e32ae>
1408e4185: 49 f7 db                    	negq	%r11
1408e4188: 49 c1 e3 04                 	shlq	$0x4, %r11
1408e418c: 4a 8b 4c 19 f8              	movq	-0x8(%rcx,%r11), %rcx
1408e4191: 48 89 c8                    	movq	%rcx, %rax
1408e4194: 48 c1 e8 20                 	shrq	$0x20, %rax
1408e4198: 48 bf 00 00 00 00 01 00 00 00       	movabsq	$0x100000000, %rdi # imm = 0x100000000
1408e41a2: 48 39 f9                    	cmpq	%rdi, %rcx
1408e41a5: 48 0f 47 f9                 	cmovaq	%rcx, %rdi
1408e41a9: 48 85 c0                    	testq	%rax, %rax
1408e41ac: 0f 84 fc 00 00 00           	je	0x1408e42ae <.text+0x8e32ae>
1408e41b2: 89 ca                       	movl	%ecx, %edx
1408e41b4: c1 ea 08                    	shrl	$0x8, %edx
1408e41b7: 40 88 bc 24 90 00 00 00     	movb	%dil, 0x90(%rsp)
1408e41bf: c1 e9 18                    	shrl	$0x18, %ecx
1408e41c2: 88 8c 24 93 00 00 00        	movb	%cl, 0x93(%rsp)
1408e41c9: 66 89 94 24 91 00 00 00     	movw	%dx, 0x91(%rsp)
1408e41d1: 89 84 24 94 00 00 00        	movl	%eax, 0x94(%rsp)
1408e41d8: 40 b7 01                    	movb	$0x1, %dil
1408e41db: 48 83 bb 10 3e 00 00 00     	cmpq	$0x0, 0x3e10(%rbx)
1408e41e3: 0f 84 c5 00 00 00           	je	0x1408e42ae <.text+0x8e32ae>
1408e41e9: 48 8d 8b 18 3e 00 00        	leaq	0x3e18(%rbx), %rcx
1408e41f0: 48 8d 94 24 90 00 00 00     	leaq	0x90(%rsp), %rdx
1408e41f8: e8 73 cb cc ff              	callq	0x1405b0d70 <.text+0x5afd70>
1408e41fd: 48 8b 8b f8 3d 00 00        	movq	0x3df8(%rbx), %rcx
1408e4204: 48 8b 93 00 3e 00 00        	movq	0x3e00(%rbx), %rdx
1408e420b: 49 89 d0                    	movq	%rdx, %r8
1408e420e: 49 21 c0                    	andq	%rax, %r8
1408e4211: 48 c1 e8 39                 	shrq	$0x39, %rax
1408e4215: 66 0f 6e c0                 	movd	%eax, %xmm0
1408e4219: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408e421d: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408e4222: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408e4227: 48 8d 41 e0                 	leaq	-0x20(%rcx), %rax
1408e422b: 45 31 c9                    	xorl	%r9d, %r9d
1408e422e: 4c 8b 94 24 90 00 00 00     	movq	0x90(%rsp), %r10
1408e4236: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1408e423a: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1408e4240: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1408e4244: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1408e4248: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
1408e424d: 45 85 db                    	testl	%r11d, %r11d
1408e4250: 74 2b                       	je	0x1408e427d <.text+0x8e327d>
1408e4252: f3 45 0f bc f3              	tzcntl	%r11d, %r14d
1408e4257: 4d 01 c6                    	addq	%r8, %r14
1408e425a: 49 21 d6                    	andq	%rdx, %r14
1408e425d: 4d 89 f7                    	movq	%r14, %r15
1408e4260: 49 c1 e7 05                 	shlq	$0x5, %r15
1408e4264: 49 89 c4                    	movq	%rax, %r12
1408e4267: 4d 29 fc                    	subq	%r15, %r12
1408e426a: 4d 3b 14 24                 	cmpq	(%r12), %r10
1408e426e: 74 2b                       	je	0x1408e429b <.text+0x8e329b>
1408e4270: 41 8d 6b ff                 	leal	-0x1(%r11), %ebp
1408e4274: 66 44 21 dd                 	andw	%r11w, %bp
1408e4278: 41 89 eb                    	movl	%ebp, %r11d
1408e427b: 75 d5                       	jne	0x1408e4252 <.text+0x8e3252>
1408e427d: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1408e4281: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1408e4286: 45 85 db                    	testl	%r11d, %r11d
1408e4289: 75 23                       	jne	0x1408e42ae <.text+0x8e32ae>
1408e428b: 4d 01 c8                    	addq	%r9, %r8
1408e428e: 49 83 c0 10                 	addq	$0x10, %r8
1408e4292: 49 83 c1 10                 	addq	$0x10, %r9
1408e4296: 49 21 d0                    	andq	%rdx, %r8
1408e4299: eb 9f                       	jmp	0x1408e423a <.text+0x8e323a>
1408e429b: 49 f7 de                    	negq	%r14
1408e429e: 49 c1 e6 05                 	shlq	$0x5, %r14
1408e42a2: 42 0f b6 44 31 f8           	movzbl	-0x8(%rcx,%r14), %eax
1408e42a8: 48 83 f8 11                 	cmpq	$0x11, %rax
1408e42ac: 75 1c                       	jne	0x1408e42ca <.text+0x8e32ca>
1408e42ae: 40 88 7e 01                 	movb	%dil, 0x1(%rsi)
1408e42b2: c6 06 01                    	movb	$0x1, (%rsi)
1408e42b5: 48 89 f0                    	movq	%rsi, %rax
1408e42b8: 48 81 c4 50 01 00 00        	addq	$0x150, %rsp            # imm = 0x150
1408e42bf: 5b                          	popq	%rbx
1408e42c0: 5d                          	popq	%rbp
1408e42c1: 5f                          	popq	%rdi
1408e42c2: 5e                          	popq	%rsi
1408e42c3: 41 5c                       	popq	%r12
1408e42c5: 41 5e                       	popq	%r14
1408e42c7: 41 5f                       	popq	%r15
1408e42c9: c3                          	retq
1408e42ca: 4c 01 f1                    	addq	%r14, %rcx
1408e42cd: 8b 51 f0                    	movl	-0x10(%rcx), %edx
1408e42d0: 44 8b 41 f4                 	movl	-0xc(%rcx), %r8d
1408e42d4: 48 69 c0 90 03 00 00        	imulq	$0x390, %rax, %rax      # imm = 0x390
1408e42db: 48 8d 0c 03                 	leaq	(%rbx,%rax), %rcx
1408e42df: 48 83 c1 20                 	addq	$0x20, %rcx
1408e42e3: e8 58 f6 2a 00              	callq	0x140b93940 <.text+0xb92940>
1408e42e8: 48 85 c0                    	testq	%rax, %rax
1408e42eb: 0f 84 07 02 00 00           	je	0x1408e44f8 <.text+0x8e34f8>
1408e42f1: 48 8b 08                    	movq	(%rax), %rcx
1408e42f4: 41 89 c8                    	movl	%ecx, %r8d
1408e42f7: 4c 39 43 10                 	cmpq	%r8, 0x10(%rbx)
1408e42fb: 0f 86 df 01 00 00           	jbe	0x1408e44e0 <.text+0x8e34e0>
1408e4301: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
1408e4305: 48 85 d2                    	testq	%rdx, %rdx
1408e4308: 0f 84 d2 01 00 00           	je	0x1408e44e0 <.text+0x8e34e0>
1408e430e: 48 c1 e9 20                 	shrq	$0x20, %rcx
1408e4312: 83 c9 01                    	orl	$0x1, %ecx
1408e4315: 4f 8d 04 80                 	leaq	(%r8,%r8,4), %r8
1408e4319: 42 39 4c c2 20              	cmpl	%ecx, 0x20(%rdx,%r8,8)
1408e431e: 0f 85 bc 01 00 00           	jne	0x1408e44e0 <.text+0x8e34e0>
1408e4324: f2 0f 10 40 44              	movsd	0x44(%rax), %xmm0
1408e4329: f3 0f 10 48 4c              	movss	0x4c(%rax), %xmm1
1408e432e: 0f 10 50 34                 	movups	0x34(%rax), %xmm2
1408e4332: 4a 8d 3c c2                 	leaq	(%rdx,%r8,8), %rdi
1408e4336: 0f 13 44 24 38              	movlps	%xmm0, 0x38(%rsp)
1408e433b: f3 0f 11 4c 24 40           	movss	%xmm1, 0x40(%rsp)
1408e4341: 0f 29 54 24 50              	movaps	%xmm2, 0x50(%rsp)
1408e4346: 48 8d 8c 24 10 01 00 00     	leaq	0x110(%rsp), %rcx
1408e434e: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1408e4353: 4c 8d 44 24 50              	leaq	0x50(%rsp), %r8
1408e4358: e8 03 d8 3a 00              	callq	0x140c91b60 <.text+0xc90b60>
1408e435d: 48 8d 4c 24 78              	leaq	0x78(%rsp), %rcx
1408e4362: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
1408e4367: f3 0f 10 15 71 0d 1c 02     	movss	0x21c0d71(%rip), %xmm2  # 0x142aa50e0
1408e436f: e8 6c 8a 3b 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1408e4374: f2 0f 10 05 84 99 0f 02     	movsd	0x20f9984(%rip), %xmm0  # 0x1429ddd00
1408e437c: f2 0f 11 84 24 84 00 00 00  	movsd	%xmm0, 0x84(%rsp)
1408e4385: c7 84 24 8c 00 00 00 00 00 00 80    	movl	$0x80000000, 0x8c(%rsp) # imm = 0x80000000
1408e4390: 0f 28 8c 24 20 01 00 00     	movaps	0x120(%rsp), %xmm1
1408e4398: 0f 29 4c 24 50              	movaps	%xmm1, 0x50(%rsp)
1408e439d: f3 0f 10 8c 24 18 01 00 00  	movss	0x118(%rsp), %xmm1
1408e43a6: f3 0f 11 4c 24 68           	movss	%xmm1, 0x68(%rsp)
1408e43ac: 48 8b 84 24 10 01 00 00     	movq	0x110(%rsp), %rax
1408e43b4: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
1408e43b9: 48 8b 47 18                 	movq	0x18(%rdi), %rax
1408e43bd: 48 8b 48 30                 	movq	0x30(%rax), %rcx
1408e43c1: 48 8b 40 38                 	movq	0x38(%rax), %rax
1408e43c5: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1408e43c9: 48 ff ca                    	decq	%rdx
1408e43cc: 48 83 e2 f0                 	andq	$-0x10, %rdx
1408e43d0: 48 01 ca                    	addq	%rcx, %rdx
1408e43d3: 48 83 c2 10                 	addq	$0x10, %rdx
1408e43d7: f3 0f 10 8c 24 80 00 00 00  	movss	0x80(%rsp), %xmm1
1408e43e0: f2 0f 10 54 24 78           	movsd	0x78(%rsp), %xmm2
1408e43e6: f2 0f 11 54 24 38           	movsd	%xmm2, 0x38(%rsp)
1408e43ec: f3 0f 11 4c 24 40           	movss	%xmm1, 0x40(%rsp)
1408e43f2: f2 0f 11 44 24 44           	movsd	%xmm0, 0x44(%rsp)
1408e43f8: c7 44 24 4c 00 00 00 80     	movl	$0x80000000, 0x4c(%rsp) # imm = 0x80000000
1408e4400: c6 44 24 28 01              	movb	$0x1, 0x28(%rsp)
1408e4405: c7 44 24 20 ff ff 7f 7f     	movl	$0x7f7fffff, 0x20(%rsp) # imm = 0x7F7FFFFF
1408e440d: 48 8d 8c 24 38 01 00 00     	leaq	0x138(%rsp), %rcx
1408e4415: 4c 8d 44 24 50              	leaq	0x50(%rsp), %r8
1408e441a: 4c 8d 4c 24 38              	leaq	0x38(%rsp), %r9
1408e441f: ff 50 38                    	callq	*0x38(%rax)
1408e4422: 83 bc 24 38 01 00 00 04     	cmpl	$0x4, 0x138(%rsp)
1408e442a: 74 30                       	je	0x1408e445c <.text+0x8e345c>
1408e442c: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
1408e4431: 48 8d 8c 24 d0 00 00 00     	leaq	0xd0(%rsp), %rcx
1408e4439: 48 8d 54 24 78              	leaq	0x78(%rsp), %rdx
1408e443e: 4c 8d 84 24 38 01 00 00     	leaq	0x138(%rsp), %r8
1408e4446: 4c 8d 4c 24 50              	leaq	0x50(%rsp), %r9
1408e444b: e8 50 a0 fe ff              	callq	0x1408ce4a0 <.text+0x8cd4a0>
1408e4450: 8b 84 24 04 01 00 00        	movl	0x104(%rsp), %eax
1408e4457: 83 f8 02                    	cmpl	$0x2, %eax
1408e445a: 75 0a                       	jne	0x1408e4466 <.text+0x8e3466>
1408e445c: 66 c7 06 01 03              	movw	$0x301, (%rsi)          # imm = 0x301
1408e4461: e9 4f fe ff ff              	jmp	0x1408e42b5 <.text+0x8e32b5>
1408e4466: 8b 8c 24 00 01 00 00        	movl	0x100(%rsp), %ecx
1408e446d: 89 8c 24 bf 00 00 00        	movl	%ecx, 0xbf(%rsp)
1408e4474: 0f 10 84 24 d1 00 00 00     	movups	0xd1(%rsp), %xmm0
1408e447c: 0f 10 8c 24 e1 00 00 00     	movups	0xe1(%rsp), %xmm1
1408e4484: 0f 10 94 24 f1 00 00 00     	movups	0xf1(%rsp), %xmm2
1408e448c: 0f 29 94 24 b0 00 00 00     	movaps	%xmm2, 0xb0(%rsp)
1408e4494: 0f 29 8c 24 a0 00 00 00     	movaps	%xmm1, 0xa0(%rsp)
1408e449c: 0f 29 84 24 90 00 00 00     	movaps	%xmm0, 0x90(%rsp)
1408e44a4: a8 01                       	testb	$0x1, %al
1408e44a6: 74 2b                       	je	0x1408e44d3 <.text+0x8e34d3>
1408e44a8: 48 8b 84 24 08 01 00 00     	movq	0x108(%rsp), %rax
1408e44b0: 8b 8c 24 af 00 00 00        	movl	0xaf(%rsp), %ecx
1408e44b7: 89 4e 14                    	movl	%ecx, 0x14(%rsi)
1408e44ba: 48 8b 8c 24 a7 00 00 00     	movq	0xa7(%rsp), %rcx
1408e44c2: 48 89 4e 0c                 	movq	%rcx, 0xc(%rsi)
1408e44c6: 48 89 46 04                 	movq	%rax, 0x4(%rsi)
1408e44ca: 31 c0                       	xorl	%eax, %eax
1408e44cc: 88 06                       	movb	%al, (%rsi)
1408e44ce: e9 e2 fd ff ff              	jmp	0x1408e42b5 <.text+0x8e32b5>
1408e44d3: c6 46 01 04                 	movb	$0x4, 0x1(%rsi)
1408e44d7: b0 01                       	movb	$0x1, %al
1408e44d9: 88 06                       	movb	%al, (%rsi)
1408e44db: e9 d5 fd ff ff              	jmp	0x1408e42b5 <.text+0x8e32b5>
1408e44e0: 48 8d 0d f9 93 38 02        	leaq	0x23893f9(%rip), %rcx   # 0x142c6d8e0
1408e44e7: 4c 8d 05 c2 f8 1b 02        	leaq	0x21bf8c2(%rip), %r8    # 0x142aa3db0
1408e44ee: ba 18 00 00 00              	movl	$0x18, %edx
1408e44f3: e8 2d 42 ff 01              	callq	0x1428d8725 <.text+0x28d7725>
1408e44f8: 48 8d 0d 51 f8 1b 02        	leaq	0x21bf851(%rip), %rcx   # 0x142aa3d50
1408e44ff: 4c 8d 05 92 f8 1b 02        	leaq	0x21bf892(%rip), %r8    # 0x142aa3d98
1408e4506: ba 42 00 00 00              	movl	$0x42, %edx
1408e450b: e8 20 4e ff 01              	callq	0x1428d9330 <.text+0x28d8330>
