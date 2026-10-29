
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407f3da0: 55                          	pushq	%rbp
1407f3da1: 41 57                       	pushq	%r15
1407f3da3: 41 56                       	pushq	%r14
1407f3da5: 41 55                       	pushq	%r13
1407f3da7: 41 54                       	pushq	%r12
1407f3da9: 56                          	pushq	%rsi
1407f3daa: 57                          	pushq	%rdi
1407f3dab: 53                          	pushq	%rbx
1407f3dac: 48 81 ec 08 03 00 00        	subq	$0x308, %rsp            # imm = 0x308
1407f3db3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407f3dbb: 44 0f 29 bd 70 02 00 00     	movaps	%xmm15, 0x270(%rbp)
1407f3dc3: 44 0f 29 b5 60 02 00 00     	movaps	%xmm14, 0x260(%rbp)
1407f3dcb: 44 0f 29 ad 50 02 00 00     	movaps	%xmm13, 0x250(%rbp)
1407f3dd3: 44 0f 29 a5 40 02 00 00     	movaps	%xmm12, 0x240(%rbp)
1407f3ddb: 44 0f 29 9d 30 02 00 00     	movaps	%xmm11, 0x230(%rbp)
1407f3de3: 44 0f 29 95 20 02 00 00     	movaps	%xmm10, 0x220(%rbp)
1407f3deb: 44 0f 29 8d 10 02 00 00     	movaps	%xmm9, 0x210(%rbp)
1407f3df3: 44 0f 29 85 00 02 00 00     	movaps	%xmm8, 0x200(%rbp)
1407f3dfb: 66 0f 7f bd f0 01 00 00     	movdqa	%xmm7, 0x1f0(%rbp)
1407f3e03: 0f 29 b5 e0 01 00 00        	movaps	%xmm6, 0x1e0(%rbp)
1407f3e0a: 48 c7 85 d8 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1d8(%rbp)
1407f3e15: 4d 89 ce                    	movq	%r9, %r14
1407f3e18: 4c 89 c6                    	movq	%r8, %rsi
1407f3e1b: 48 89 d7                    	movq	%rdx, %rdi
1407f3e1e: 49 89 cd                    	movq	%rcx, %r13
1407f3e21: e8 da 2b 42 00              	callq	0x140c16a00 <.text+0xc15a00>
1407f3e26: 88 85 d5 01 00 00           	movb	%al, 0x1d5(%rbp)
1407f3e2c: 84 c0                       	testb	%al, %al
1407f3e2e: 0f 84 1e 02 00 00           	je	0x1407f4052 <.text+0x7f3052>
1407f3e34: 41 bc 58 00 00 00           	movl	$0x58, %r12d
1407f3e3a: b9 58 00 00 00              	movl	$0x58, %ecx
1407f3e3f: e8 dc fa ff ff              	callq	0x1407f3920 <.text+0x7f2920>
1407f3e44: 48 8d 1d 35 7a 29 02        	leaq	0x2297a35(%rip), %rbx   # 0x142a8b880
1407f3e4b: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f3e4f: 0f 85 a1 00 00 00           	jne	0x1407f3ef6 <.text+0x7f2ef6>
1407f3e55: 49 89 d1                    	movq	%rdx, %r9
1407f3e58: 48 85 d2                    	testq	%rdx, %rdx
1407f3e5b: 74 36                       	je	0x1407f3e93 <.text+0x7f2e93>
1407f3e5d: 49 83 f9 58                 	cmpq	$0x58, %r9
1407f3e61: 73 2e                       	jae	0x1407f3e91 <.text+0x7f2e91>
1407f3e63: 48 8d 05 16 7a 29 02        	leaq	0x2297a16(%rip), %rax   # 0x142a8b880
1407f3e6a: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1407f3e6f: 7f 22                       	jg	0x1407f3e93 <.text+0x7f2e93>
1407f3e71: 48 8d 05 e0 7a 29 02        	leaq	0x2297ae0(%rip), %rax   # 0x142a8b958
1407f3e78: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f3e7d: 48 8d 0d fc 79 29 02        	leaq	0x22979fc(%rip), %rcx   # 0x142a8b880
1407f3e84: ba 58 00 00 00              	movl	$0x58, %edx
1407f3e89: 45 31 c0                    	xorl	%r8d, %r8d
1407f3e8c: e8 af 5a 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407f3e91: 75 de                       	jne	0x1407f3e71 <.text+0x7f2e71>
1407f3e93: 4c 89 c9                    	movq	%r9, %rcx
1407f3e96: e8 85 fa ff ff              	callq	0x1407f3920 <.text+0x7f2920>
1407f3e9b: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f3e9f: 75 55                       	jne	0x1407f3ef6 <.text+0x7f2ef6>
1407f3ea1: 49 89 d0                    	movq	%rdx, %r8
1407f3ea4: 49 83 c0 02                 	addq	$0x2, %r8
1407f3ea8: 74 39                       	je	0x1407f3ee3 <.text+0x7f2ee3>
1407f3eaa: 49 83 f8 58                 	cmpq	$0x58, %r8
1407f3eae: 73 31                       	jae	0x1407f3ee1 <.text+0x7f2ee1>
1407f3eb0: 48 8d 05 c9 79 29 02        	leaq	0x22979c9(%rip), %rax   # 0x142a8b880
1407f3eb7: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1407f3ebc: 7f 25                       	jg	0x1407f3ee3 <.text+0x7f2ee3>
1407f3ebe: 48 8d 05 ab 7a 29 02        	leaq	0x2297aab(%rip), %rax   # 0x142a8b970
1407f3ec5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f3eca: 48 8d 0d af 79 29 02        	leaq	0x22979af(%rip), %rcx   # 0x142a8b880
1407f3ed1: ba 58 00 00 00              	movl	$0x58, %edx
1407f3ed6: 41 b9 58 00 00 00           	movl	$0x58, %r9d
1407f3edc: e8 5f 5a 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407f3ee1: 75 db                       	jne	0x1407f3ebe <.text+0x7f2ebe>
1407f3ee3: 41 bc 56 00 00 00           	movl	$0x56, %r12d
1407f3ee9: 49 29 d4                    	subq	%rdx, %r12
1407f3eec: 48 8d 1d 8d 79 29 02        	leaq	0x229798d(%rip), %rbx   # 0x142a8b880
1407f3ef3: 4c 01 c3                    	addq	%r8, %rbx
1407f3ef6: 4c 89 ad 68 01 00 00        	movq	%r13, 0x168(%rbp)
1407f3efd: 4c 8d 05 0f 7b 29 02        	leaq	0x2297b0f(%rip), %r8    # 0x142a8ba13
1407f3f04: 4c 8d 2d c3 7a 29 02        	leaq	0x2297ac3(%rip), %r13   # 0x142a8b9ce
1407f3f0b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1407f3f10: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407f3f15: 85 c0                       	testl	%eax, %eax
1407f3f17: 78 17                       	js	0x1407f3f30 <.text+0x7f2f30>
1407f3f19: 49 ff c8                    	decq	%r8
1407f3f1c: 83 f8 5c                    	cmpl	$0x5c, %eax
1407f3f1f: 75 63                       	jne	0x1407f3f84 <.text+0x7f2f84>
1407f3f21: eb 73                       	jmp	0x1407f3f96 <.text+0x7f2f96>
1407f3f23: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407f3f30: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407f3f35: 80 f9 c0                    	cmpb	$-0x40, %cl
1407f3f38: 7d 23                       	jge	0x1407f3f5d <.text+0x7f2f5d>
1407f3f3a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407f3f3f: 80 fa c0                    	cmpb	$-0x40, %dl
1407f3f42: 7d 22                       	jge	0x1407f3f66 <.text+0x7f2f66>
1407f3f44: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407f3f49: 49 83 c0 fc                 	addq	$-0x4, %r8
1407f3f4d: 41 83 e1 07                 	andl	$0x7, %r9d
1407f3f51: 41 c1 e1 06                 	shll	$0x6, %r9d
1407f3f55: 83 e2 3f                    	andl	$0x3f, %edx
1407f3f58: 44 09 ca                    	orl	%r9d, %edx
1407f3f5b: eb 10                       	jmp	0x1407f3f6d <.text+0x7f2f6d>
1407f3f5d: 49 83 c0 fe                 	addq	$-0x2, %r8
1407f3f61: 83 e1 1f                    	andl	$0x1f, %ecx
1407f3f64: eb 0f                       	jmp	0x1407f3f75 <.text+0x7f2f75>
1407f3f66: 49 83 c0 fd                 	addq	$-0x3, %r8
1407f3f6a: 83 e2 0f                    	andl	$0xf, %edx
1407f3f6d: c1 e2 06                    	shll	$0x6, %edx
1407f3f70: 83 e1 3f                    	andl	$0x3f, %ecx
1407f3f73: 09 d1                       	orl	%edx, %ecx
1407f3f75: c1 e1 06                    	shll	$0x6, %ecx
1407f3f78: 24 3f                       	andb	$0x3f, %al
1407f3f7a: 0f b6 c0                    	movzbl	%al, %eax
1407f3f7d: 09 c8                       	orl	%ecx, %eax
1407f3f7f: 83 f8 5c                    	cmpl	$0x5c, %eax
1407f3f82: 74 12                       	je	0x1407f3f96 <.text+0x7f2f96>
1407f3f84: 83 f8 2f                    	cmpl	$0x2f, %eax
1407f3f87: 74 0d                       	je	0x1407f3f96 <.text+0x7f2f96>
1407f3f89: 4d 39 e8                    	cmpq	%r13, %r8
1407f3f8c: 75 82                       	jne	0x1407f3f10 <.text+0x7f2f10>
1407f3f8e: 41 bf 45 00 00 00           	movl	$0x45, %r15d
1407f3f94: eb 4d                       	jmp	0x1407f3fe3 <.text+0x7f2fe3>
1407f3f96: 4c 8d 2d 31 7a 29 02        	leaq	0x2297a31(%rip), %r13   # 0x142a8b9ce
1407f3f9d: 4d 29 e8                    	subq	%r13, %r8
1407f3fa0: 49 ff c0                    	incq	%r8
1407f3fa3: 74 32                       	je	0x1407f3fd7 <.text+0x7f2fd7>
1407f3fa5: 49 83 f8 45                 	cmpq	$0x45, %r8
1407f3fa9: 73 2a                       	jae	0x1407f3fd5 <.text+0x7f2fd5>
1407f3fab: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1407f3fb0: 7f 25                       	jg	0x1407f3fd7 <.text+0x7f2fd7>
1407f3fb2: 48 8d 05 7f 79 29 02        	leaq	0x229797f(%rip), %rax   # 0x142a8b938
1407f3fb9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f3fbe: 48 8d 0d 09 7a 29 02        	leaq	0x2297a09(%rip), %rcx   # 0x142a8b9ce
1407f3fc5: ba 45 00 00 00              	movl	$0x45, %edx
1407f3fca: 41 b9 45 00 00 00           	movl	$0x45, %r9d
1407f3fd0: e8 6b 59 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407f3fd5: 75 db                       	jne	0x1407f3fb2 <.text+0x7f2fb2>
1407f3fd7: 41 bf 45 00 00 00           	movl	$0x45, %r15d
1407f3fdd: 4d 29 c7                    	subq	%r8, %r15
1407f3fe0: 4d 01 c5                    	addq	%r8, %r13
1407f3fe3: e8 48 41 42 00              	callq	0x140c18130 <.text+0xc17130>
1407f3fe8: 48 89 c1                    	movq	%rax, %rcx
1407f3feb: 48 8b 00                    	movq	(%rax), %rax
1407f3fee: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f3ff2: 0f 85 c3 18 00 00           	jne	0x1407f58bb <.text+0x7f48bb>
1407f3ff8: 48 83 c1 08                 	addq	$0x8, %rcx
1407f3ffc: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407f4000: 0f 85 04 19 00 00           	jne	0x1407f590a <.text+0x7f490a>
1407f4006: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407f400d: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
1407f4014: 48 83 c1 08                 	addq	$0x8, %rcx
1407f4018: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1407f401d: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1407f4026: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407f402f: 48 89 da                    	movq	%rbx, %rdx
1407f4032: 4d 89 e0                    	movq	%r12, %r8
1407f4035: 4d 89 e9                    	movq	%r13, %r9
1407f4038: e8 03 2d 42 00              	callq	0x140c16d40 <.text+0xc15d40>
1407f403d: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1407f4041: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
1407f4048: 48 ff 00                    	incq	(%rax)
1407f404b: 4c 8b ad 68 01 00 00        	movq	0x168(%rbp), %r13
1407f4052: 4c 89 f1                    	movq	%r14, %rcx
1407f4055: e8 16 2a 33 00              	callq	0x140b26a70 <.text+0xb25a70>
1407f405a: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
1407f4061: 48 89 95 a0 00 00 00        	movq	%rdx, 0xa0(%rbp)
1407f4068: 48 8b 16                    	movq	(%rsi), %rdx
1407f406b: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f4072: e8 89 70 22 00              	callq	0x140a1b100 <.text+0xa1a100>
1407f4077: 48 8b 8d 00 03 00 00        	movq	0x300(%rbp), %rcx
1407f407e: 48 8b 95 f8 02 00 00        	movq	0x2f8(%rbp), %rdx
1407f4085: 4c 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %r8
1407f408c: 4c 8b a5 80 01 00 00        	movq	0x180(%rbp), %r12
1407f4093: 4c 8b bd 88 01 00 00        	movq	0x188(%rbp), %r15
1407f409a: 0f b7 85 98 01 00 00        	movzwl	0x198(%rbp), %eax
1407f40a1: 4c 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %r9
1407f40a8: 45 8b 50 1c                 	movl	0x1c(%r8), %r10d
1407f40ac: 44 89 55 68                 	movl	%r10d, 0x68(%rbp)
1407f40b0: 4d 8b 10                    	movq	(%r8), %r10
1407f40b3: 4c 89 55 f8                 	movq	%r10, -0x8(%rbp)
1407f40b7: 4d 8b 40 10                 	movq	0x10(%r8), %r8
1407f40bb: 4c 89 45 f0                 	movq	%r8, -0x10(%rbp)
1407f40bf: 4c 8b 07                    	movq	(%rdi), %r8
1407f40c2: 4c 89 45 00                 	movq	%r8, (%rbp)
1407f40c6: 49 83 c0 20                 	addq	$0x20, %r8
1407f40ca: 4c 89 45 08                 	movq	%r8, 0x8(%rbp)
1407f40ce: 4d 8b 45 00                 	movq	(%r13), %r8
1407f40d2: 49 83 c0 58                 	addq	$0x58, %r8
1407f40d6: 4c 89 45 18                 	movq	%r8, 0x18(%rbp)
1407f40da: 48 8b 12                    	movq	(%rdx), %rdx
1407f40dd: 48 89 55 58                 	movq	%rdx, 0x58(%rbp)
1407f40e1: 48 8b 09                    	movq	(%rcx), %rcx
1407f40e4: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
1407f40e8: 66 0f 76 ff                 	pcmpeqd	%xmm7, %xmm7
1407f40ec: 44 0f 28 0d ec 17 13 02     	movaps	0x21317ec(%rip), %xmm9  # 0x1429258e0
1407f40f4: f3 44 0f 10 15 07 18 13 02  	movss	0x2131807(%rip), %xmm10 # 0x142925904
1407f40fd: 4c 89 4d 78                 	movq	%r9, 0x78(%rbp)
1407f4101: 4d 85 c9                    	testq	%r9, %r9
1407f4104: 0f 84 76 15 00 00           	je	0x1407f5680 <.text+0x7f4680>
1407f410a: 66 85 c0                    	testw	%ax, %ax
1407f410d: 74 21                       	je	0x1407f4130 <.text+0x7f3130>
1407f410f: 4d 85 e4                    	testq	%r12, %r12
1407f4112: 0f 84 68 15 00 00           	je	0x1407f5680 <.text+0x7f4680>
1407f4118: 8d 48 ff                    	leal	-0x1(%rax), %ecx
1407f411b: 21 c1                       	andl	%eax, %ecx
1407f411d: 89 8d 4c 01 00 00           	movl	%ecx, 0x14c(%rbp)
1407f4123: eb 39                       	jmp	0x1407f415e <.text+0x7f315e>
1407f4125: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407f4130: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
1407f4135: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1407f4139: 49 81 c4 80 f6 ff ff        	addq	$-0x980, %r12           # imm = 0xF680
1407f4140: 49 83 c7 10                 	addq	$0x10, %r15
1407f4144: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1407f4149: 74 e5                       	je	0x1407f4130 <.text+0x7f3130>
1407f414b: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1407f4150: 29 c1                       	subl	%eax, %ecx
1407f4152: f7 d0                       	notl	%eax
1407f4154: 89 c2                       	movl	%eax, %edx
1407f4156: 21 ca                       	andl	%ecx, %edx
1407f4158: 89 95 4c 01 00 00           	movl	%edx, 0x14c(%rbp)
1407f415e: f3 0f bc c0                 	tzcntl	%eax, %eax
1407f4162: 48 f7 d8                    	negq	%rax
1407f4165: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
1407f416c: 49 8d 14 04                 	leaq	(%r12,%rax), %rdx
1407f4170: 48 81 c2 70 ff ff ff        	addq	$-0x90, %rdx
1407f4177: 49 8b 84 04 68 ff ff ff     	movq	-0x98(%r12,%rax), %rax
1407f417f: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1407f4183: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f418a: e8 e1 6e 22 00              	callq	0x140a1b070 <.text+0xa1a070>
1407f418f: 48 ff 4d 78                 	decq	0x78(%rbp)
1407f4193: 48 8b 9d 80 01 00 00        	movq	0x180(%rbp), %rbx
1407f419a: 4c 8b b5 88 01 00 00        	movq	0x188(%rbp), %r14
1407f41a1: 0f b7 8d 98 01 00 00        	movzwl	0x198(%rbp), %ecx
1407f41a8: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1407f41af: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1407f41b6: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
1407f41bd: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1407f41c4: 48 83 c0 50                 	addq	$0x50, %rax
1407f41c8: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1407f41cc: 48 83 bd 68 01 00 00 00     	cmpq	$0x0, 0x168(%rbp)
1407f41d4: 0f 84 8f 14 00 00           	je	0x1407f5669 <.text+0x7f4669>
1407f41da: 66 85 c9                    	testw	%cx, %cx
1407f41dd: 74 21                       	je	0x1407f4200 <.text+0x7f3200>
1407f41df: 8d 41 ff                    	leal	-0x1(%rcx), %eax
1407f41e2: 21 c8                       	andl	%ecx, %eax
1407f41e4: 89 85 c4 01 00 00           	movl	%eax, 0x1c4(%rbp)
1407f41ea: 48 85 db                    	testq	%rbx, %rbx
1407f41ed: 75 3d                       	jne	0x1407f422c <.text+0x7f322c>
1407f41ef: e9 75 14 00 00              	jmp	0x1407f5669 <.text+0x7f4669>
1407f41f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1407f4200: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
1407f4205: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
1407f4209: 48 83 c3 c0                 	addq	$-0x40, %rbx
1407f420d: 49 83 c6 10                 	addq	$0x10, %r14
1407f4211: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
1407f4217: 74 e7                       	je	0x1407f4200 <.text+0x7f3200>
1407f4219: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1407f421e: 29 c8                       	subl	%ecx, %eax
1407f4220: f7 d1                       	notl	%ecx
1407f4222: 89 ca                       	movl	%ecx, %edx
1407f4224: 21 c2                       	andl	%eax, %edx
1407f4226: 89 95 c4 01 00 00           	movl	%edx, 0x1c4(%rbp)
1407f422c: f3 0f bc c1                 	tzcntl	%ecx, %eax
1407f4230: c1 e0 02                    	shll	$0x2, %eax
1407f4233: 48 89 d9                    	movq	%rbx, %rcx
1407f4236: 48 29 c1                    	subq	%rax, %rcx
1407f4239: 8b 41 fc                    	movl	-0x4(%rcx), %eax
1407f423c: 89 85 38 01 00 00           	movl	%eax, 0x138(%rbp)
1407f4242: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
1407f4248: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
1407f424f: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
1407f4254: 0f 84 d9 14 00 00           	je	0x1407f5733 <.text+0x7f4733>
1407f425a: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1407f425e: 48 8d 95 80 01 00 00        	leaq	0x180(%rbp), %rdx
1407f4265: e8 66 cc db ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1407f426a: 48 ff 8d 68 01 00 00        	decq	0x168(%rbp)
1407f4271: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
1407f4278: 48 8b 79 30                 	movq	0x30(%rcx), %rdi
1407f427c: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
1407f4280: 49 89 cb                    	movq	%rcx, %r11
1407f4283: 49 21 c3                    	andq	%rax, %r11
1407f4286: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f428a: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f428e: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f4292: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f4297: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f429c: 48 8d 47 b0                 	leaq	-0x50(%rdi), %rax
1407f42a0: 45 31 c0                    	xorl	%r8d, %r8d
1407f42a3: f3 42 0f 6f 0c 1f           	movdqu	(%rdi,%r11), %xmm1
1407f42a9: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f42ad: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f42b1: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407f42b6: 45 85 c9                    	testl	%r9d, %r9d
1407f42b9: 74 35                       	je	0x1407f42f0 <.text+0x7f32f0>
1407f42bb: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407f42c0: 4d 01 da                    	addq	%r11, %r10
1407f42c3: 49 21 ca                    	andq	%rcx, %r10
1407f42c6: 49 f7 da                    	negq	%r10
1407f42c9: 4f 8d 2c 92                 	leaq	(%r10,%r10,4), %r13
1407f42cd: 49 c1 e5 04                 	shlq	$0x4, %r13
1407f42d1: 8b 95 38 01 00 00           	movl	0x138(%rbp), %edx
1407f42d7: 42 3b 14 28                 	cmpl	(%rax,%r13), %edx
1407f42db: 74 43                       	je	0x1407f4320 <.text+0x7f3320>
1407f42dd: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407f42e1: 66 45 21 ca                 	andw	%r9w, %r10w
1407f42e5: 45 89 d1                    	movl	%r10d, %r9d
1407f42e8: 75 d1                       	jne	0x1407f42bb <.text+0x7f32bb>
1407f42ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407f42f0: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
1407f42f4: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407f42f9: 45 85 c9                    	testl	%r9d, %r9d
1407f42fc: 0f 85 31 14 00 00           	jne	0x1407f5733 <.text+0x7f4733>
1407f4302: 4d 01 c3                    	addq	%r8, %r11
1407f4305: 49 83 c3 10                 	addq	$0x10, %r11
1407f4309: 49 83 c0 10                 	addq	$0x10, %r8
1407f430d: 49 21 cb                    	andq	%rcx, %r11
1407f4310: eb 91                       	jmp	0x1407f42a3 <.text+0x7f32a3>
1407f4312: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1407f4320: 48 8d 8d 98 00 00 00        	leaq	0x98(%rbp), %rcx
1407f4327: e8 44 26 33 00              	callq	0x140b26970 <.text+0xb25970>
1407f432c: 8b 8d c4 01 00 00           	movl	0x1c4(%rbp), %ecx
1407f4332: 3c 04                       	cmpb	$0x4, %al
1407f4334: 0f 85 92 fe ff ff           	jne	0x1407f41cc <.text+0x7f31cc>
1407f433a: 8b 45 68                    	movl	0x68(%rbp), %eax
1407f433d: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1407f4341: 89 01                       	movl	%eax, (%rcx)
1407f4343: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
1407f4347: 48 89 8d 40 01 00 00        	movq	%rcx, 0x140(%rbp)
1407f434e: e8 cd 81 0c 00              	callq	0x1408bc520 <.text+0x8bb520>
1407f4353: 49 89 d1                    	movq	%rdx, %r9
1407f4356: 4a 8d 0c 2f                 	leaq	(%rdi,%r13), %rcx
1407f435a: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
1407f4361: 0f b6 49 ff                 	movzbl	-0x1(%rcx), %ecx
1407f4365: 88 4c 24 20                 	movb	%cl, 0x20(%rsp)
1407f4369: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1407f436d: 8b 95 38 01 00 00           	movl	0x138(%rbp), %edx
1407f4373: 49 89 c0                    	movq	%rax, %r8
1407f4376: e8 35 15 03 00              	callq	0x1408258b0 <.text+0x8248b0>
1407f437b: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1407f4382: 48 8b 75 00                 	movq	(%rbp), %rsi
1407f4386: 48 83 7e 18 00              	cmpq	$0x0, 0x18(%rsi)
1407f438b: 0f 84 b3 13 00 00           	je	0x1407f5744 <.text+0x7f4744>
1407f4391: 4a 8d 04 2f                 	leaq	(%rdi,%r13), %rax
1407f4395: 48 83 c0 b8                 	addq	$-0x48, %rax
1407f4399: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
1407f43a0: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1407f43a7: 0f b6 78 ff                 	movzbl	-0x1(%rax), %edi
1407f43ab: 8b 40 e0                    	movl	-0x20(%rax), %eax
1407f43ae: 89 85 04 01 00 00           	movl	%eax, 0x104(%rbp)
1407f43b4: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
1407f43b8: 48 8d 55 70                 	leaq	0x70(%rbp), %rdx
1407f43bc: e8 af c9 db ff              	callq	0x1405b0d70 <.text+0x5afd70>
1407f43c1: 4c 8b 1e                    	movq	(%rsi), %r11
1407f43c4: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
1407f43c8: 48 89 ca                    	movq	%rcx, %rdx
1407f43cb: 48 21 c2                    	andq	%rax, %rdx
1407f43ce: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f43d2: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f43d6: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f43da: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f43df: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f43e4: 49 8d 83 58 fd ff ff        	leaq	-0x2a8(%r11), %rax
1407f43eb: 45 31 c0                    	xorl	%r8d, %r8d
1407f43ee: f3 41 0f 6f 0c 13           	movdqu	(%r11,%rdx), %xmm1
1407f43f4: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f43f8: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f43fc: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407f4401: 45 85 c9                    	testl	%r9d, %r9d
1407f4404: 74 2f                       	je	0x1407f4435 <.text+0x7f3435>
1407f4406: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407f440b: 49 01 d2                    	addq	%rdx, %r10
1407f440e: 49 21 ca                    	andq	%rcx, %r10
1407f4411: 49 f7 da                    	negq	%r10
1407f4414: 4d 69 ea a8 02 00 00        	imulq	$0x2a8, %r10, %r13      # imm = 0x2A8
1407f441b: 4c 8b 95 40 01 00 00        	movq	0x140(%rbp), %r10
1407f4422: 4e 3b 14 28                 	cmpq	(%rax,%r13), %r10
1407f4426: 74 2f                       	je	0x1407f4457 <.text+0x7f3457>
1407f4428: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407f442c: 66 45 21 ca                 	andw	%r9w, %r10w
1407f4430: 45 89 d1                    	movl	%r10d, %r9d
1407f4433: 75 d1                       	jne	0x1407f4406 <.text+0x7f3406>
1407f4435: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
1407f4439: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407f443e: 45 85 c9                    	testl	%r9d, %r9d
1407f4441: 0f 85 fd 12 00 00           	jne	0x1407f5744 <.text+0x7f4744>
1407f4447: 4c 01 c2                    	addq	%r8, %rdx
1407f444a: 48 83 c2 10                 	addq	$0x10, %rdx
1407f444e: 49 83 c0 10                 	addq	$0x10, %r8
1407f4452: 48 21 ca                    	andq	%rcx, %rdx
1407f4455: eb 97                       	jmp	0x1407f43ee <.text+0x7f33ee>
1407f4457: 4c 89 9d 40 01 00 00        	movq	%r11, 0x140(%rbp)
1407f445e: 40 88 bd d6 01 00 00        	movb	%dil, 0x1d6(%rbp)
1407f4465: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1407f446c: e8 9f f0 03 00              	callq	0x140833510 <.text+0x832510>
1407f4471: f2 44 0f 10 00              	movsd	(%rax), %xmm8
1407f4476: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1407f447d: 40 8a 78 fc                 	movb	-0x4(%rax), %dil
1407f4481: 48 8b 95 98 00 00 00        	movq	0x98(%rbp), %rdx
1407f4488: b1 04                       	movb	$0x4, %cl
1407f448a: e8 c1 ed 03 00              	callq	0x140833250 <.text+0x832250>
1407f448f: 48 8b b5 a0 00 00 00        	movq	0xa0(%rbp), %rsi
1407f4496: 89 f9                       	movl	%edi, %ecx
1407f4498: 89 c2                       	movl	%eax, %edx
1407f449a: e8 f1 17 33 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f449f: 88 85 50 01 00 00           	movb	%al, 0x150(%rbp)
1407f44a5: 88 95 51 01 00 00           	movb	%dl, 0x151(%rbp)
1407f44ab: 48 89 f1                    	movq	%rsi, %rcx
1407f44ae: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407f44b5: e8 d6 8a a6 00              	callq	0x14125cf90 <.text+0x125bf90>
1407f44ba: 48 85 c0                    	testq	%rax, %rax
1407f44bd: 0f 84 b8 14 00 00           	je	0x1407f597b <.text+0x7f497b>
1407f44c3: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407f44ca: 4a 8d 34 29                 	leaq	(%rcx,%r13), %rsi
1407f44ce: 48 81 c6 60 fd ff ff        	addq	$-0x2a0, %rsi           # imm = 0xFD60
1407f44d5: f3 0f 10 70 30              	movss	0x30(%rax), %xmm6
1407f44da: 44 0f 13 85 88 00 00 00     	movlps	%xmm8, 0x88(%rbp)
1407f44e2: 48 89 f1                    	movq	%rsi, %rcx
1407f44e5: e8 56 62 33 00              	callq	0x140b2a740 <.text+0xb29740>
1407f44ea: f3 0f 59 35 7e 13 13 02     	mulss	0x213137e(%rip), %xmm6  # 0x142925870
1407f44f2: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
1407f44f8: 66 0f 7e 85 84 01 00 00     	movd	%xmm0, 0x184(%rbp)
1407f4500: f3 0f 10 8d 88 00 00 00     	movss	0x88(%rbp), %xmm1
1407f4508: f3 0f 5c ce                 	subss	%xmm6, %xmm1
1407f450c: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f4513: e8 98 d9 49 00              	callq	0x140c91eb0 <.text+0xc90eb0>
1407f4518: 66 0f 7e 85 88 00 00 00     	movd	%xmm0, 0x88(%rbp)
1407f4520: 44 0f 13 85 10 01 00 00     	movlps	%xmm8, 0x110(%rbp)
1407f4528: 48 89 f1                    	movq	%rsi, %rcx
1407f452b: e8 10 62 33 00              	callq	0x140b2a740 <.text+0xb29740>
1407f4530: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
1407f4536: 66 0f 7e 85 84 01 00 00     	movd	%xmm0, 0x184(%rbp)
1407f453e: f3 0f 58 b5 10 01 00 00     	addss	0x110(%rbp), %xmm6
1407f4546: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f454d: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407f4550: e8 5b d9 49 00              	callq	0x140c91eb0 <.text+0xc90eb0>
1407f4555: 66 0f 7e 85 10 01 00 00     	movd	%xmm0, 0x110(%rbp)
1407f455d: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f4564: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
1407f456b: 49 89 f0                    	movq	%rsi, %r8
1407f456e: e8 1d db 49 00              	callq	0x140c92090 <.text+0xc91090>
1407f4573: f3 44 0f 10 a5 80 01 00 00  	movss	0x180(%rbp), %xmm12
1407f457c: f3 44 0f 10 9d 88 01 00 00  	movss	0x188(%rbp), %xmm11
1407f4585: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f458c: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
1407f4593: 49 89 f0                    	movq	%rsi, %r8
1407f4596: e8 f5 da 49 00              	callq	0x140c92090 <.text+0xc91090>
1407f459b: f3 44 0f 10 ad 80 01 00 00  	movss	0x180(%rbp), %xmm13
1407f45a4: f3 0f 10 b5 88 01 00 00     	movss	0x188(%rbp), %xmm6
1407f45ac: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1407f45b0: 41 0f 14 c3                 	unpcklps	%xmm11, %xmm0           # xmm0 = xmm0[0],xmm11[0],xmm0[1],xmm11[1]
1407f45b4: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
1407f45b8: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1407f45bc: 0f 14 ce                    	unpcklps	%xmm6, %xmm1            # xmm1 = xmm1[0],xmm6[0],xmm1[1],xmm6[1]
1407f45bf: 41 0f 59 c9                 	mulps	%xmm9, %xmm1
1407f45c3: 0f 58 c8                    	addps	%xmm0, %xmm1
1407f45c6: 0f 13 8d 20 01 00 00        	movlps	%xmm1, 0x120(%rbp)
1407f45cd: f3 41 0f 16 d0              	movshdup	%xmm8, %xmm2            # xmm2 = xmm8[1,1,3,3]
1407f45d2: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1407f45d6: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
1407f45dd: e8 fe 87 4a 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407f45e2: 41 0f 14 f5                 	unpcklps	%xmm13, %xmm6           # xmm6 = xmm6[0],xmm13[0],xmm6[1],xmm13[1]
1407f45e6: 45 0f 14 dc                 	unpcklps	%xmm12, %xmm11          # xmm11 = xmm11[0],xmm12[0],xmm11[1],xmm12[1]
1407f45ea: 41 0f 5c f3                 	subps	%xmm11, %xmm6
1407f45ee: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407f45f1: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f45f4: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407f45f8: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f45fc: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407f45ff: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1407f4603: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407f4607: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407f460b: 0f 28 c1                    	movaps	%xmm1, %xmm0
1407f460e: 0f 57 05 db 12 13 02        	xorps	0x21312db(%rip), %xmm0  # 0x1429258f0
1407f4615: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1407f4618: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f461b: 0f 13 85 50 01 00 00        	movlps	%xmm0, 0x150(%rbp)
1407f4622: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1407f4629: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407f4630: e8 8b 87 4a 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407f4635: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f463c: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
1407f4643: e8 88 cb 49 00              	callq	0x140c911d0 <.text+0xc901d0>
1407f4648: 0f 28 b5 80 01 00 00        	movaps	0x180(%rbp), %xmm6
1407f464f: f2 44 0f 10 6d 48           	movsd	0x48(%rbp), %xmm13
1407f4655: f3 44 0f 10 65 50           	movss	0x50(%rbp), %xmm12
1407f465b: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
1407f4665: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1407f466c: c7 85 d0 00 00 00 00 00 80 3f       	movl	$0x3f800000, 0xd0(%rbp) # imm = 0x3F800000
1407f4676: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1407f467d: 40 8a 70 fc                 	movb	-0x4(%rax), %sil
1407f4681: 48 8b 95 98 00 00 00        	movq	0x98(%rbp), %rdx
1407f4688: b1 04                       	movb	$0x4, %cl
1407f468a: e8 c1 eb 03 00              	callq	0x140833250 <.text+0x832250>
1407f468f: 48 8b bd a0 00 00 00        	movq	0xa0(%rbp), %rdi
1407f4696: 89 f1                       	movl	%esi, %ecx
1407f4698: 89 c2                       	movl	%eax, %edx
1407f469a: e8 f1 15 33 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f469f: 88 85 50 01 00 00           	movb	%al, 0x150(%rbp)
1407f46a5: 88 95 51 01 00 00           	movb	%dl, 0x151(%rbp)
1407f46ab: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
1407f46b0: 0f 84 ab 10 00 00           	je	0x1407f5761 <.text+0x7f4761>
1407f46b6: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
1407f46ba: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407f46c1: e8 fa 36 a0 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407f46c6: 4c 01 ad 40 01 00 00        	addq	%r13, 0x140(%rbp)
1407f46cd: 48 8b 0f                    	movq	(%rdi), %rcx
1407f46d0: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1407f46d4: 49 89 d0                    	movq	%rdx, %r8
1407f46d7: 49 21 c0                    	andq	%rax, %r8
1407f46da: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f46de: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f46e2: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f46e6: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f46eb: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f46f0: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407f46f4: 44 0f b6 8d 50 01 00 00     	movzbl	0x150(%rbp), %r9d
1407f46fc: 44 0f b6 95 51 01 00 00     	movzbl	0x151(%rbp), %r10d
1407f4704: 31 ff                       	xorl	%edi, %edi
1407f4706: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407f470c: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f4710: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f4714: 66 0f d7 f2                 	pmovmskb	%xmm2, %esi
1407f4718: 85 f6                       	testl	%esi, %esi
1407f471a: 74 79                       	je	0x1407f4795 <.text+0x7f3795>
1407f471c: 41 80 f9 1a                 	cmpb	$0x1a, %r9b
1407f4720: 75 2d                       	jne	0x1407f474f <.text+0x7f374f>
1407f4722: 49 89 fd                    	movq	%rdi, %r13
1407f4725: f3 0f bc fe                 	tzcntl	%esi, %edi
1407f4729: 4c 01 c7                    	addq	%r8, %rdi
1407f472c: 48 21 d7                    	andq	%rdx, %rdi
1407f472f: 48 f7 df                    	negq	%rdi
1407f4732: 4c 8d 1c ff                 	leaq	(%rdi,%rdi,8), %r11
1407f4736: 42 80 3c d8 1a              	cmpb	$0x1a, (%rax,%r11,8)
1407f473b: 74 7d                       	je	0x1407f47ba <.text+0x7f37ba>
1407f473d: 44 8d 5e ff                 	leal	-0x1(%rsi), %r11d
1407f4741: 66 41 21 f3                 	andw	%si, %r11w
1407f4745: 44 89 de                    	movl	%r11d, %esi
1407f4748: 4c 89 ef                    	movq	%r13, %rdi
1407f474b: 75 d5                       	jne	0x1407f4722 <.text+0x7f3722>
1407f474d: eb 46                       	jmp	0x1407f4795 <.text+0x7f3795>
1407f474f: 48 89 bd b0 00 00 00        	movq	%rdi, 0xb0(%rbp)
1407f4756: f3 0f bc fe                 	tzcntl	%esi, %edi
1407f475a: 4c 01 c7                    	addq	%r8, %rdi
1407f475d: 48 21 d7                    	andq	%rdx, %rdi
1407f4760: 48 f7 df                    	negq	%rdi
1407f4763: 4c 8d 2c ff                 	leaq	(%rdi,%rdi,8), %r13
1407f4767: 46 0f b6 1c e8              	movzbl	(%rax,%r13,8), %r11d
1407f476c: 41 80 fb 1a                 	cmpb	$0x1a, %r11b
1407f4770: 74 0f                       	je	0x1407f4781 <.text+0x7f3781>
1407f4772: 45 38 d9                    	cmpb	%r11b, %r9b
1407f4775: 75 0a                       	jne	0x1407f4781 <.text+0x7f3781>
1407f4777: 4e 8d 1c e8                 	leaq	(%rax,%r13,8), %r11
1407f477b: 45 3a 53 01                 	cmpb	0x1(%r11), %r10b
1407f477f: 74 39                       	je	0x1407f47ba <.text+0x7f37ba>
1407f4781: 44 8d 5e ff                 	leal	-0x1(%rsi), %r11d
1407f4785: 66 41 21 f3                 	andw	%si, %r11w
1407f4789: 44 89 de                    	movl	%r11d, %esi
1407f478c: 48 8b bd b0 00 00 00        	movq	0xb0(%rbp), %rdi
1407f4793: 75 c1                       	jne	0x1407f4756 <.text+0x7f3756>
1407f4795: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
1407f4799: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1407f479e: 45 85 db                    	testl	%r11d, %r11d
1407f47a1: 0f 85 ba 0f 00 00           	jne	0x1407f5761 <.text+0x7f4761>
1407f47a7: 49 01 f8                    	addq	%rdi, %r8
1407f47aa: 49 83 c0 10                 	addq	$0x10, %r8
1407f47ae: 48 83 c7 10                 	addq	$0x10, %rdi
1407f47b2: 49 21 d0                    	andq	%rdx, %r8
1407f47b5: e9 4c ff ff ff              	jmp	0x1407f4706 <.text+0x7f3706>
1407f47ba: 48 8d 04 ff                 	leaq	(%rdi,%rdi,8), %rax
1407f47be: 48 8b 44 c1 d8              	movq	-0x28(%rcx,%rax,8), %rax
1407f47c3: f0                          	lock
1407f47c4: 48 ff 00                    	incq	(%rax)
1407f47c7: 0f 8e 6d 12 00 00           	jle	0x1407f5a3a <.text+0x7f4a3a>
1407f47cd: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1407f47d4: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
1407f47db: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1407f47e2: 40 8a 70 fc                 	movb	-0x4(%rax), %sil
1407f47e6: 48 8b 95 98 00 00 00        	movq	0x98(%rbp), %rdx
1407f47ed: c6 85 d7 01 00 00 01        	movb	$0x1, 0x1d7(%rbp)
1407f47f4: b1 04                       	movb	$0x4, %cl
1407f47f6: e8 55 ea 03 00              	callq	0x140833250 <.text+0x832250>
1407f47fb: 48 8b bd a0 00 00 00        	movq	0xa0(%rbp), %rdi
1407f4802: c6 85 d7 01 00 00 01        	movb	$0x1, 0x1d7(%rbp)
1407f4809: 89 f1                       	movl	%esi, %ecx
1407f480b: 89 c2                       	movl	%eax, %edx
1407f480d: e8 7e 14 33 00              	callq	0x140b25c90 <.text+0xb24c90>
1407f4812: 88 85 20 01 00 00           	movb	%al, 0x120(%rbp)
1407f4818: 88 95 21 01 00 00           	movb	%dl, 0x121(%rbp)
1407f481e: 48 83 7f 48 00              	cmpq	$0x0, 0x48(%rdi)
1407f4823: 0f 84 a9 0f 00 00           	je	0x1407f57d2 <.text+0x7f47d2>
1407f4829: 48 8d 4f 50                 	leaq	0x50(%rdi), %rcx
1407f482d: c6 85 d7 01 00 00 01        	movb	$0x1, 0x1d7(%rbp)
1407f4834: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
1407f483b: e8 80 35 a0 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407f4840: 48 8b 4f 30                 	movq	0x30(%rdi), %rcx
1407f4844: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
1407f4848: 49 89 d0                    	movq	%rdx, %r8
1407f484b: 49 21 c0                    	andq	%rax, %r8
1407f484e: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f4852: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f4856: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f485a: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f485f: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f4864: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407f4868: 44 0f b6 8d 20 01 00 00     	movzbl	0x120(%rbp), %r9d
1407f4870: 44 0f b6 95 21 01 00 00     	movzbl	0x121(%rbp), %r10d
1407f4878: 31 ff                       	xorl	%edi, %edi
1407f487a: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407f4880: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f4884: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f4888: 66 0f d7 f2                 	pmovmskb	%xmm2, %esi
1407f488c: 85 f6                       	testl	%esi, %esi
1407f488e: 74 79                       	je	0x1407f4909 <.text+0x7f3909>
1407f4890: 41 80 f9 1a                 	cmpb	$0x1a, %r9b
1407f4894: 75 2d                       	jne	0x1407f48c3 <.text+0x7f38c3>
1407f4896: 49 89 fd                    	movq	%rdi, %r13
1407f4899: f3 0f bc fe                 	tzcntl	%esi, %edi
1407f489d: 4c 01 c7                    	addq	%r8, %rdi
1407f48a0: 48 21 d7                    	andq	%rdx, %rdi
1407f48a3: 48 f7 df                    	negq	%rdi
1407f48a6: 4c 8d 1c ff                 	leaq	(%rdi,%rdi,8), %r11
1407f48aa: 42 80 3c d8 1a              	cmpb	$0x1a, (%rax,%r11,8)
1407f48af: 74 7d                       	je	0x1407f492e <.text+0x7f392e>
1407f48b1: 44 8d 5e ff                 	leal	-0x1(%rsi), %r11d
1407f48b5: 66 41 21 f3                 	andw	%si, %r11w
1407f48b9: 44 89 de                    	movl	%r11d, %esi
1407f48bc: 4c 89 ef                    	movq	%r13, %rdi
1407f48bf: 75 d5                       	jne	0x1407f4896 <.text+0x7f3896>
1407f48c1: eb 46                       	jmp	0x1407f4909 <.text+0x7f3909>
1407f48c3: 48 89 bd a8 00 00 00        	movq	%rdi, 0xa8(%rbp)
1407f48ca: f3 0f bc fe                 	tzcntl	%esi, %edi
1407f48ce: 4c 01 c7                    	addq	%r8, %rdi
1407f48d1: 48 21 d7                    	andq	%rdx, %rdi
1407f48d4: 48 f7 df                    	negq	%rdi
1407f48d7: 4c 8d 2c ff                 	leaq	(%rdi,%rdi,8), %r13
1407f48db: 46 0f b6 1c e8              	movzbl	(%rax,%r13,8), %r11d
1407f48e0: 41 80 fb 1a                 	cmpb	$0x1a, %r11b
1407f48e4: 74 0f                       	je	0x1407f48f5 <.text+0x7f38f5>
1407f48e6: 45 38 d9                    	cmpb	%r11b, %r9b
1407f48e9: 75 0a                       	jne	0x1407f48f5 <.text+0x7f38f5>
1407f48eb: 4e 8d 1c e8                 	leaq	(%rax,%r13,8), %r11
1407f48ef: 45 3a 53 01                 	cmpb	0x1(%r11), %r10b
1407f48f3: 74 39                       	je	0x1407f492e <.text+0x7f392e>
1407f48f5: 44 8d 5e ff                 	leal	-0x1(%rsi), %r11d
1407f48f9: 66 41 21 f3                 	andw	%si, %r11w
1407f48fd: 44 89 de                    	movl	%r11d, %esi
1407f4900: 48 8b bd a8 00 00 00        	movq	0xa8(%rbp), %rdi
1407f4907: 75 c1                       	jne	0x1407f48ca <.text+0x7f38ca>
1407f4909: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
1407f490d: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1407f4912: 45 85 db                    	testl	%r11d, %r11d
1407f4915: 0f 85 b7 0e 00 00           	jne	0x1407f57d2 <.text+0x7f47d2>
1407f491b: 49 01 f8                    	addq	%rdi, %r8
1407f491e: 49 83 c0 10                 	addq	$0x10, %r8
1407f4922: 48 83 c7 10                 	addq	$0x10, %rdi
1407f4926: 49 21 d0                    	andq	%rdx, %r8
1407f4929: e9 4c ff ff ff              	jmp	0x1407f487a <.text+0x7f387a>
1407f492e: 48 8d 04 ff                 	leaq	(%rdi,%rdi,8), %rax
1407f4932: 4c 8b 44 c1 d8              	movq	-0x28(%rcx,%rax,8), %r8
1407f4937: f0                          	lock
1407f4938: 49 ff 00                    	incq	(%r8)
1407f493b: 0f 8e f9 10 00 00           	jle	0x1407f5a3a <.text+0x7f4a3a>
1407f4941: 0f 29 75 c0                 	movaps	%xmm6, -0x40(%rbp)
1407f4945: c6 85 d7 01 00 00 00        	movb	$0x0, 0x1d7(%rbp)
1407f494c: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
1407f4950: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f4955: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1407f495c: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1407f4963: 4c 8d 4d 48                 	leaq	0x48(%rbp), %r9
1407f4967: e8 44 09 03 00              	callq	0x1408252b0 <.text+0x8242b0>
1407f496c: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1407f4973: 48 8b 01                    	movq	(%rcx), %rax
1407f4976: 48 0b 41 08                 	orq	0x8(%rcx), %rax
1407f497a: 0f 85 96 0f 00 00           	jne	0x1407f5916 <.text+0x7f4916>
1407f4980: 48 c7 41 08 00 00 00 00     	movq	$0x0, 0x8(%rcx)
1407f4988: 48 c7 01 01 00 00 00        	movq	$0x1, (%rcx)
1407f498f: 0f 29 71 10                 	movaps	%xmm6, 0x10(%rcx)
1407f4993: 44 0f 13 69 20              	movlps	%xmm13, 0x20(%rcx)
1407f4998: f3 44 0f 11 61 28           	movss	%xmm12, 0x28(%rcx)
1407f499e: f3 0f 6f 85 c8 00 00 00     	movdqu	0xc8(%rbp), %xmm0
1407f49a6: f3 0f 7f 41 2c              	movdqu	%xmm0, 0x2c(%rcx)
1407f49ab: 8b 85 d8 00 00 00           	movl	0xd8(%rbp), %eax
1407f49b1: 89 41 3c                    	movl	%eax, 0x3c(%rcx)
1407f49b4: 48 8b b5 a0 00 00 00        	movq	0xa0(%rbp), %rsi
1407f49bb: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1407f49c2: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407f49c6: c6 85 50 01 00 00 04        	movb	$0x4, 0x150(%rbp)
1407f49cd: 88 85 51 01 00 00           	movb	%al, 0x151(%rbp)
1407f49d3: 48 83 7e 18 00              	cmpq	$0x0, 0x18(%rsi)
1407f49d8: 0f 84 6c 0e 00 00           	je	0x1407f584a <.text+0x7f484a>
1407f49de: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1407f49e2: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407f49e9: e8 d2 33 a0 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407f49ee: 48 8b 0e                    	movq	(%rsi), %rcx
1407f49f1: 48 8b 56 08                 	movq	0x8(%rsi), %rdx
1407f49f5: 49 89 d0                    	movq	%rdx, %r8
1407f49f8: 49 21 c0                    	andq	%rax, %r8
1407f49fb: 48 c1 e8 39                 	shrq	$0x39, %rax
1407f49ff: 66 0f 6e c0                 	movd	%eax, %xmm0
1407f4a03: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407f4a07: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407f4a0c: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f4a11: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407f4a15: 44 0f b6 8d 50 01 00 00     	movzbl	0x150(%rbp), %r9d
1407f4a1d: 44 0f b6 95 51 01 00 00     	movzbl	0x151(%rbp), %r10d
1407f4a25: 31 ff                       	xorl	%edi, %edi
1407f4a27: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407f4a2d: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407f4a31: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407f4a35: 66 0f d7 f2                 	pmovmskb	%xmm2, %esi
1407f4a39: 85 f6                       	testl	%esi, %esi
1407f4a3b: 74 79                       	je	0x1407f4ab6 <.text+0x7f3ab6>
1407f4a3d: 41 80 f9 1a                 	cmpb	$0x1a, %r9b
1407f4a41: 75 2d                       	jne	0x1407f4a70 <.text+0x7f3a70>
1407f4a43: 49 89 fd                    	movq	%rdi, %r13
1407f4a46: f3 0f bc fe                 	tzcntl	%esi, %edi
1407f4a4a: 4c 01 c7                    	addq	%r8, %rdi
1407f4a4d: 48 21 d7                    	andq	%rdx, %rdi
1407f4a50: 48 f7 df                    	negq	%rdi
1407f4a53: 4c 8d 1c ff                 	leaq	(%rdi,%rdi,8), %r11
1407f4a57: 42 80 3c d8 1a              	cmpb	$0x1a, (%rax,%r11,8)
1407f4a5c: 74 7d                       	je	0x1407f4adb <.text+0x7f3adb>
1407f4a5e: 44 8d 5e ff                 	leal	-0x1(%rsi), %r11d
1407f4a62: 66 41 21 f3                 	andw	%si, %r11w
1407f4a66: 44 89 de                    	movl	%r11d, %esi
1407f4a69: 4c 89 ef                    	movq	%r13, %rdi
1407f4a6c: 75 d5                       	jne	0x1407f4a43 <.text+0x7f3a43>
1407f4a6e: eb 46                       	jmp	0x1407f4ab6 <.text+0x7f3ab6>
1407f4a70: 48 89 bd b0 00 00 00        	movq	%rdi, 0xb0(%rbp)
1407f4a77: f3 0f bc fe                 	tzcntl	%esi, %edi
1407f4a7b: 4c 01 c7                    	addq	%r8, %rdi
1407f4a7e: 48 21 d7                    	andq	%rdx, %rdi
1407f4a81: 48 f7 df                    	negq	%rdi
1407f4a84: 4c 8d 2c ff                 	leaq	(%rdi,%rdi,8), %r13
1407f4a88: 46 0f b6 1c e8              	movzbl	(%rax,%r13,8), %r11d
1407f4a8d: 41 80 fb 1a                 	cmpb	$0x1a, %r11b
1407f4a91: 74 0f                       	je	0x1407f4aa2 <.text+0x7f3aa2>
1407f4a93: 45 38 d9                    	cmpb	%r11b, %r9b
1407f4a96: 75 0a                       	jne	0x1407f4aa2 <.text+0x7f3aa2>
1407f4a98: 4e 8d 1c e8                 	leaq	(%rax,%r13,8), %r11
1407f4a9c: 45 3a 53 01                 	cmpb	0x1(%r11), %r10b
1407f4aa0: 74 39                       	je	0x1407f4adb <.text+0x7f3adb>
1407f4aa2: 44 8d 5e ff                 	leal	-0x1(%rsi), %r11d
1407f4aa6: 66 41 21 f3                 	andw	%si, %r11w
1407f4aaa: 44 89 de                    	movl	%r11d, %esi
1407f4aad: 48 8b bd b0 00 00 00        	movq	0xb0(%rbp), %rdi
1407f4ab4: 75 c1                       	jne	0x1407f4a77 <.text+0x7f3a77>
1407f4ab6: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
1407f4aba: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1407f4abf: 45 85 db                    	testl	%r11d, %r11d
1407f4ac2: 0f 85 82 0d 00 00           	jne	0x1407f584a <.text+0x7f484a>
1407f4ac8: 49 01 f8                    	addq	%rdi, %r8
1407f4acb: 49 83 c0 10                 	addq	$0x10, %r8
1407f4acf: 48 83 c7 10                 	addq	$0x10, %rdi
1407f4ad3: 49 21 d0                    	andq	%rdx, %r8
1407f4ad6: e9 4c ff ff ff              	jmp	0x1407f4a27 <.text+0x7f3a27>
1407f4adb: 48 8d 04 ff                 	leaq	(%rdi,%rdi,8), %rax
1407f4adf: f3 0f 6f 44 c1 e0           	movdqu	-0x20(%rcx,%rax,8), %xmm0
1407f4ae5: 66 0f 7f 45 d0              	movdqa	%xmm0, -0x30(%rbp)
1407f4aea: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1407f4af1: 0f b6 52 fc                 	movzbl	-0x4(%rdx), %edx
1407f4af5: fe ca                       	decb	%dl
1407f4af7: 80 fa 03                    	cmpb	$0x3, %dl
1407f4afa: 0f 83 e9 0e 00 00           	jae	0x1407f59e9 <.text+0x7f49e9>
1407f4b00: f3 44 0f 10 7c c1 f0        	movss	-0x10(%rcx,%rax,8), %xmm15
1407f4b07: 44 8b 44 c1 f4              	movl	-0xc(%rcx,%rax,8), %r8d
1407f4b0c: 44 89 45 6c                 	movl	%r8d, 0x6c(%rbp)
1407f4b10: f2 44 0f 10 44 c1 f8        	movsd	-0x8(%rcx,%rax,8), %xmm8
1407f4b17: 0f b6 c2                    	movzbl	%dl, %eax
1407f4b1a: 48 8d 0d ff 70 29 02        	leaq	0x22970ff(%rip), %rcx   # 0x142a8bc20
1407f4b21: 48 63 34 81                 	movslq	(%rcx,%rax,4), %rsi
1407f4b25: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1407f4b2c: 48 8d 15 55 6f 29 02        	leaq	0x2296f55(%rip), %rdx   # 0x142a8ba88
1407f4b33: e8 28 e7 03 00              	callq	0x140833260 <.text+0x832260>
1407f4b38: f3 0f 7e 00                 	movq	(%rax), %xmm0
1407f4b3c: 66 0f 7f 85 b0 00 00 00     	movdqa	%xmm0, 0xb0(%rbp)
1407f4b44: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1407f4b4b: 48 8b b8 d8 00 00 00        	movq	0xd8(%rax), %rdi
1407f4b52: 48 3b b8 c8 00 00 00        	cmpq	0xc8(%rax), %rdi
1407f4b59: 75 1a                       	jne	0x1407f4b75 <.text+0x7f3b75>
1407f4b5b: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1407f4b62: 48 8d 88 c8 00 00 00        	leaq	0xc8(%rax), %rcx
1407f4b69: 48 8d 15 30 6f 29 02        	leaq	0x2296f30(%rip), %rdx   # 0x142a8baa0
1407f4b70: e8 8b ea d6 ff              	callq	0x140563600 <.text+0x562600>
1407f4b75: f3 41 0f 16 c0              	movshdup	%xmm8, %xmm0            # xmm0 = xmm8[1,1,3,3]
1407f4b7a: f3 41 0f 5c c0              	subss	%xmm8, %xmm0
1407f4b7f: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
1407f4b86: 48 8b 82 d0 00 00 00        	movq	0xd0(%rdx), %rax
1407f4b8d: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
1407f4b91: 66 0f 6f 8d b0 00 00 00     	movdqa	0xb0(%rbp), %xmm1
1407f4b99: 66 0f d6 0c 88              	movq	%xmm1, (%rax,%rcx,4)
1407f4b9e: f3 44 0f 11 7c 88 08        	movss	%xmm15, 0x8(%rax,%rcx,4)
1407f4ba5: f3 0f 11 44 88 0c           	movss	%xmm0, 0xc(%rax,%rcx,4)
1407f4bab: c6 44 88 10 00              	movb	$0x0, 0x10(%rax,%rcx,4)
1407f4bb0: 48 ff c7                    	incq	%rdi
1407f4bb3: 48 89 ba d8 00 00 00        	movq	%rdi, 0xd8(%rdx)
1407f4bba: 48 8d 82 b0 00 00 00        	leaq	0xb0(%rdx), %rax
1407f4bc1: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
1407f4bc8: f3 0f 6f 85 c8 00 00 00     	movdqu	0xc8(%rbp), %xmm0
1407f4bd0: 66 0f 7f 85 20 01 00 00     	movdqa	%xmm0, 0x120(%rbp)
1407f4bd8: 8b 85 d8 00 00 00           	movl	0xd8(%rbp), %eax
1407f4bde: 89 85 30 01 00 00           	movl	%eax, 0x130(%rbp)
1407f4be4: 48 8d 85 9c 01 00 00        	leaq	0x19c(%rbp), %rax
1407f4beb: c7 40 ef 00 00 00 00        	movl	$0x0, -0x11(%rax)
1407f4bf2: 48 c7 40 e7 00 00 00 00     	movq	$0x0, -0x19(%rax)
1407f4bfa: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1407f4c01: 48 8b 8d 87 01 00 00        	movq	0x187(%rbp), %rcx
1407f4c08: 48 89 8d 17 01 00 00        	movq	%rcx, 0x117(%rbp)
1407f4c0f: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1407f4c16: 48 8b ba c0 00 00 00        	movq	0xc0(%rdx), %rdi
1407f4c1d: 48 3b ba b0 00 00 00        	cmpq	0xb0(%rdx), %rdi
1407f4c24: 75 13                       	jne	0x1407f4c39 <.text+0x7f3c39>
1407f4c26: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
1407f4c2d: 48 8d 15 9c 6e 29 02        	leaq	0x2296e9c(%rip), %rdx   # 0x142a8bad0
1407f4c34: e8 77 67 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407f4c39: 48 8d 05 e0 6f 29 02        	leaq	0x2296fe0(%rip), %rax   # 0x142a8bc20
1407f4c40: 48 01 c6                    	addq	%rax, %rsi
1407f4c43: 4c 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %r9
1407f4c4a: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407f4c51: 48 69 cf 60 01 00 00        	imulq	$0x160, %rdi, %rcx      # imm = 0x160
1407f4c58: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407f4c60: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407f4c69: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407f4c72: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407f4c7a: 48 8b 55 24                 	movq	0x24(%rbp), %rdx
1407f4c7e: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407f4c83: 8b 55 2c                    	movl	0x2c(%rbp), %edx
1407f4c86: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f4c8a: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407f4c92: 0f 10 85 80 01 00 00        	movups	0x180(%rbp), %xmm0
1407f4c99: 0f 10 8d 90 01 00 00        	movups	0x190(%rbp), %xmm1
1407f4ca0: 0f 10 95 a0 01 00 00        	movups	0x1a0(%rbp), %xmm2
1407f4ca7: 0f 10 9d ac 01 00 00        	movups	0x1ac(%rbp), %xmm3
1407f4cae: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407f4cb3: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407f4cb8: 0f 11 54 08 64              	movups	%xmm2, 0x64(%rax,%rcx)
1407f4cbd: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407f4cc2: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407f4ccd: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
1407f4cd1: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407f4cd9: 8b 55 38                    	movl	0x38(%rbp), %edx
1407f4cdc: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407f4ce3: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407f4cee: 48 8b 55 3c                 	movq	0x3c(%rbp), %rdx
1407f4cf2: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407f4cfa: 8b 55 44                    	movl	0x44(%rbp), %edx
1407f4cfd: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407f4d04: 48 89 b4 08 a0 00 00 00     	movq	%rsi, 0xa0(%rax,%rcx)
1407f4d0c: 48 c7 84 08 a8 00 00 00 1d 00 00 00 	movq	$0x1d, 0xa8(%rax,%rcx)
1407f4d18: 48 8d 15 99 6d 29 02        	leaq	0x2296d99(%rip), %rdx   # 0x142a8bab8
1407f4d1f: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407f4d27: 48 c7 84 08 b8 00 00 00 11 00 00 00 	movq	$0x11, 0xb8(%rax,%rcx)
1407f4d33: 0f 29 b4 08 c0 00 00 00     	movaps	%xmm6, 0xc0(%rax,%rcx)
1407f4d3b: 44 0f 13 ac 08 d0 00 00 00  	movlps	%xmm13, 0xd0(%rax,%rcx)
1407f4d44: f3 44 0f 11 a4 08 d8 00 00 00       	movss	%xmm12, 0xd8(%rax,%rcx)
1407f4d4e: 8b 95 30 01 00 00           	movl	0x130(%rbp), %edx
1407f4d54: 89 94 08 ec 00 00 00        	movl	%edx, 0xec(%rax,%rcx)
1407f4d5b: 0f 28 85 20 01 00 00        	movaps	0x120(%rbp), %xmm0
1407f4d62: 0f 11 84 08 dc 00 00 00     	movups	%xmm0, 0xdc(%rax,%rcx)
1407f4d6a: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407f4d75: 8b 95 04 01 00 00           	movl	0x104(%rbp), %edx
1407f4d7b: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407f4d82: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407f4d8d: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1407f4d94: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407f4d9c: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
1407f4da3: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407f4dab: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407f4db3: 0f b6 95 7f 01 00 00        	movzbl	0x17f(%rbp), %edx
1407f4dba: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407f4dc1: 0f b7 95 7d 01 00 00        	movzwl	0x17d(%rbp), %edx
1407f4dc8: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407f4dd0: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407f4ddb: f3 0f 6f 85 e0 00 00 00     	movdqu	0xe0(%rbp), %xmm0
1407f4de3: 0f 10 8d f0 00 00 00        	movups	0xf0(%rbp), %xmm1
1407f4dea: 0f 11 8c 08 2c 01 00 00     	movups	%xmm1, 0x12c(%rax,%rcx)
1407f4df2: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407f4dfb: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407f4e03: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
1407f4e0a: 4c 8b 85 17 01 00 00        	movq	0x117(%rbp), %r8
1407f4e11: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407f4e19: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407f4e21: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407f4e29: 0f b6 95 d6 01 00 00        	movzbl	0x1d6(%rbp), %edx
1407f4e30: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407f4e37: 66 c7 84 08 4f 01 00 00 01 09       	movw	$0x901, 0x14f(%rax,%rcx) # imm = 0x901
1407f4e41: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1407f4e48: 4c 8b 85 8f 00 00 00        	movq	0x8f(%rbp), %r8
1407f4e4f: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407f4e57: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407f4e5f: 48 ff c7                    	incq	%rdi
1407f4e62: 49 89 b9 c0 00 00 00        	movq	%rdi, 0xc0(%r9)
1407f4e69: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1407f4e70: 80 78 fd 04                 	cmpb	$0x4, -0x3(%rax)
1407f4e74: 0f 82 0e 03 00 00           	jb	0x1407f5188 <.text+0x7f4188>
1407f4e7a: 48 8b 75 70                 	movq	0x70(%rbp), %rsi
1407f4e7e: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1407f4e85: 8b 48 e4                    	movl	-0x1c(%rax), %ecx
1407f4e88: e8 b3 13 4a 00              	callq	0x140c96240 <.text+0xc95240>
1407f4e8d: 48 89 c1                    	movq	%rax, %rcx
1407f4e90: 48 b8 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rax # imm = 0xA0761D6478BD642F
1407f4e9a: 48 01 c1                    	addq	%rax, %rcx
1407f4e9d: 48 89 c8                    	movq	%rcx, %rax
1407f4ea0: 48 ba db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rdx # imm = 0xE7037ED1A0B428DB
1407f4eaa: 48 31 d0                    	xorq	%rdx, %rax
1407f4ead: 48 f7 e1                    	mulq	%rcx
1407f4eb0: 48 0f ac d0 1e              	shrdq	$0x1e, %rdx, %rax
1407f4eb5: 48 c1 ea 1e                 	shrq	$0x1e, %rdx
1407f4eb9: 48 31 d0                    	xorq	%rdx, %rax
1407f4ebc: 89 c1                       	movl	%eax, %ecx
1407f4ebe: 80 e1 03                    	andb	$0x3, %cl
1407f4ec1: 80 f9 01                    	cmpb	$0x1, %cl
1407f4ec4: 0f 84 51 01 00 00           	je	0x1407f501b <.text+0x7f401b>
1407f4eca: 83 e0 03                    	andl	$0x3, %eax
1407f4ecd: 0f 85 b5 02 00 00           	jne	0x1407f5188 <.text+0x7f4188>
1407f4ed3: 44 0f 28 f6                 	movaps	%xmm6, %xmm14
1407f4ed7: 44 0f c6 f6 d2              	shufps	$0xd2, %xmm6, %xmm14    # xmm14 = xmm14[2,0],xmm6[1,3]
1407f4edc: 44 0f 59 35 5c 67 29 02     	mulps	0x229675c(%rip), %xmm14 # 0x142a8b640
1407f4ee4: 44 0f 28 de                 	movaps	%xmm6, %xmm11
1407f4ee8: 44 0f c6 de c9              	shufps	$0xc9, %xmm6, %xmm11    # xmm11 = xmm11[1,2],xmm6[0,3]
1407f4eed: 44 0f 59 1d 5b 67 29 02     	mulps	0x229675b(%rip), %xmm11 # 0x142a8b650
1407f4ef5: 8b 85 38 01 00 00           	movl	0x138(%rbp), %eax
1407f4efb: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
1407f4f01: 48 8d 05 d8 6c 29 02        	leaq	0x2296cd8(%rip), %rax   # 0x142a8bbe0
1407f4f08: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1407f4f0f: 48 c7 85 90 01 00 00 08 00 00 00    	movq	$0x8, 0x190(%rbp)
1407f4f1a: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f4f21: e8 0a 7d a9 00              	callq	0x14128cc30 <.text+0x128bc30>
1407f4f26: 49 89 c5                    	movq	%rax, %r13
1407f4f29: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1407f4f30: 48 8b b8 08 01 00 00        	movq	0x108(%rax), %rdi
1407f4f37: 48 3b b8 f8 00 00 00        	cmpq	0xf8(%rax), %rdi
1407f4f3e: 75 1a                       	jne	0x1407f4f5a <.text+0x7f3f5a>
1407f4f40: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1407f4f47: 48 81 c1 f8 00 00 00        	addq	$0xf8, %rcx
1407f4f4e: 48 8d 15 93 6c 29 02        	leaq	0x2296c93(%rip), %rdx   # 0x142a8bbe8
1407f4f55: e8 a6 7a f9 ff              	callq	0x14078ca00 <.text+0x78ba00>
1407f4f5a: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407f4f5d: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f4f60: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407f4f64: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f4f68: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407f4f6b: 66 0f 15 d0                 	unpckhpd	%xmm0, %xmm2            # xmm2 = xmm2[1],xmm0[1]
1407f4f6f: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407f4f73: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407f4f77: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
1407f4f7b: 0f 5c c2                    	subps	%xmm2, %xmm0
1407f4f7e: 0f 28 15 ab 66 29 02        	movaps	0x22966ab(%rip), %xmm2  # 0x142a8b630
1407f4f85: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407f4f88: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407f4f8b: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407f4f8e: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1407f4f92: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407f4f96: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407f4f99: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407f4f9d: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407f4fa1: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407f4fa5: 0f 59 ce                    	mulps	%xmm6, %xmm1
1407f4fa8: 0f 58 c8                    	addps	%xmm0, %xmm1
1407f4fab: 45 0f 5c de                 	subps	%xmm14, %xmm11
1407f4faf: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407f4fb2: 0f 58 c6                    	addps	%xmm6, %xmm0
1407f4fb5: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
1407f4fb9: 41 0f 59 c3                 	mulps	%xmm11, %xmm0
1407f4fbd: 0f 58 c1                    	addps	%xmm1, %xmm0
1407f4fc0: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1407f4fc4: 0f 58 c8                    	addps	%xmm0, %xmm1
1407f4fc7: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407f4fca: f3 41 0f 58 c4              	addss	%xmm12, %xmm0
1407f4fcf: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
1407f4fd6: 49 8b 80 00 01 00 00        	movq	0x100(%r8), %rax
1407f4fdd: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
1407f4fe1: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407f4fe5: c6 04 08 03                 	movb	$0x3, (%rax,%rcx)
1407f4fe9: 0f 29 74 08 10              	movaps	%xmm6, 0x10(%rax,%rcx)
1407f4fee: 0f 13 4c 08 20              	movlps	%xmm1, 0x20(%rax,%rcx)
1407f4ff3: f3 0f 11 44 08 28           	movss	%xmm0, 0x28(%rax,%rcx)
1407f4ff9: f3 0f 6f 85 c8 00 00 00     	movdqu	0xc8(%rbp), %xmm0
1407f5001: f3 0f 7f 44 08 2c           	movdqu	%xmm0, 0x2c(%rax,%rcx)
1407f5007: 8b 95 d8 00 00 00           	movl	0xd8(%rbp), %edx
1407f500d: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f5011: 4c 89 6c 08 40              	movq	%r13, 0x40(%rax,%rcx)
1407f5016: e9 5e 01 00 00              	jmp	0x1407f5179 <.text+0x7f4179>
1407f501b: 44 0f 28 de                 	movaps	%xmm6, %xmm11
1407f501f: 44 0f c6 de d2              	shufps	$0xd2, %xmm6, %xmm11    # xmm11 = xmm11[2,0],xmm6[1,3]
1407f5024: 44 0f 59 1d 34 22 29 02     	mulps	0x2292234(%rip), %xmm11 # 0x142a87260
1407f502c: 44 0f 28 f6                 	movaps	%xmm6, %xmm14
1407f5030: 44 0f c6 f6 c9              	shufps	$0xc9, %xmm6, %xmm14    # xmm14 = xmm14[1,2],xmm6[0,3]
1407f5035: 44 0f 59 35 e3 65 29 02     	mulps	0x22965e3(%rip), %xmm14 # 0x142a8b620
1407f503d: 8b 85 38 01 00 00           	movl	0x138(%rbp), %eax
1407f5043: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
1407f5049: 48 8d 05 b0 6b 29 02        	leaq	0x2296bb0(%rip), %rax   # 0x142a8bc00
1407f5050: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1407f5057: 48 c7 85 90 01 00 00 05 00 00 00    	movq	$0x5, 0x190(%rbp)
1407f5062: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f5069: e8 c2 7b a9 00              	callq	0x14128cc30 <.text+0x128bc30>
1407f506e: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
1407f5075: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
1407f5079: e8 42 96 13 00              	callq	0x14092e6c0 <.text+0x92d6c0>
1407f507e: 41 89 c5                    	movl	%eax, %r13d
1407f5081: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1407f5088: 48 8b b8 08 01 00 00        	movq	0x108(%rax), %rdi
1407f508f: 48 3b b8 f8 00 00 00        	cmpq	0xf8(%rax), %rdi
1407f5096: 75 1a                       	jne	0x1407f50b2 <.text+0x7f40b2>
1407f5098: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1407f509f: 48 81 c1 f8 00 00 00        	addq	$0xf8, %rcx
1407f50a6: 48 8d 15 5b 6b 29 02        	leaq	0x2296b5b(%rip), %rdx   # 0x142a8bc08
1407f50ad: e8 4e 79 f9 ff              	callq	0x14078ca00 <.text+0x78ba00>
1407f50b2: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407f50b5: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f50b8: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407f50bc: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f50c0: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407f50c3: 66 0f 15 d0                 	unpckhpd	%xmm0, %xmm2            # xmm2 = xmm2[1],xmm0[1]
1407f50c7: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407f50cb: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407f50cf: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
1407f50d3: 0f 5c c2                    	subps	%xmm2, %xmm0
1407f50d6: 0f 28 15 33 65 29 02        	movaps	0x2296533(%rip), %xmm2  # 0x142a8b610
1407f50dd: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407f50e0: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407f50e3: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407f50e6: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1407f50ea: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407f50ee: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407f50f1: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407f50f5: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407f50f9: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407f50fd: 0f 59 ce                    	mulps	%xmm6, %xmm1
1407f5100: 0f 58 c8                    	addps	%xmm0, %xmm1
1407f5103: 45 0f 5c f3                 	subps	%xmm11, %xmm14
1407f5107: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407f510a: 0f 58 c6                    	addps	%xmm6, %xmm0
1407f510d: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
1407f5111: 41 0f 59 c6                 	mulps	%xmm14, %xmm0
1407f5115: 0f 58 c1                    	addps	%xmm1, %xmm0
1407f5118: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1407f511c: 0f 58 c8                    	addps	%xmm0, %xmm1
1407f511f: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407f5122: f3 41 0f 58 c4              	addss	%xmm12, %xmm0
1407f5127: 41 80 cd 04                 	orb	$0x4, %r13b
1407f512b: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
1407f5132: 49 8b 80 00 01 00 00        	movq	0x100(%r8), %rax
1407f5139: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
1407f513d: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407f5141: 44 88 2c 08                 	movb	%r13b, (%rax,%rcx)
1407f5145: 0f 29 74 08 10              	movaps	%xmm6, 0x10(%rax,%rcx)
1407f514a: 0f 13 4c 08 20              	movlps	%xmm1, 0x20(%rax,%rcx)
1407f514f: f3 0f 11 44 08 28           	movss	%xmm0, 0x28(%rax,%rcx)
1407f5155: f3 0f 6f 85 c8 00 00 00     	movdqu	0xc8(%rbp), %xmm0
1407f515d: f3 0f 7f 44 08 2c           	movdqu	%xmm0, 0x2c(%rax,%rcx)
1407f5163: 8b 95 d8 00 00 00           	movl	0xd8(%rbp), %edx
1407f5169: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f516d: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
1407f5174: 48 89 54 08 40              	movq	%rdx, 0x40(%rax,%rcx)
1407f5179: 48 89 74 08 48              	movq	%rsi, 0x48(%rax,%rcx)
1407f517e: 48 ff c7                    	incq	%rdi
1407f5181: 49 89 b8 08 01 00 00        	movq	%rdi, 0x108(%r8)
1407f5188: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1407f518f: e8 7c e3 03 00              	callq	0x140833510 <.text+0x832510>
1407f5194: 80 78 21 01                 	cmpb	$0x1, 0x21(%rax)
1407f5198: 8b 85 c4 01 00 00           	movl	0x1c4(%rbp), %eax
1407f519e: 89 c1                       	movl	%eax, %ecx
1407f51a0: 0f 85 26 f0 ff ff           	jne	0x1407f41cc <.text+0x7f31cc>
1407f51a6: 0f 29 b5 80 01 00 00        	movaps	%xmm6, 0x180(%rbp)
1407f51ad: 44 0f 13 ad 90 01 00 00     	movlps	%xmm13, 0x190(%rbp)
1407f51b5: f3 44 0f 11 a5 98 01 00 00  	movss	%xmm12, 0x198(%rbp)
1407f51be: 8b 85 d8 00 00 00           	movl	0xd8(%rbp), %eax
1407f51c4: 48 8d 8d 9c 01 00 00        	leaq	0x19c(%rbp), %rcx
1407f51cb: 89 41 10                    	movl	%eax, 0x10(%rcx)
1407f51ce: 0f 10 85 c8 00 00 00        	movups	0xc8(%rbp), %xmm0
1407f51d5: 0f 11 01                    	movups	%xmm0, (%rcx)
1407f51d8: 66 0f 6f 45 d0              	movdqa	-0x30(%rbp), %xmm0
1407f51dd: 66 0f 7f 85 e0 00 00 00     	movdqa	%xmm0, 0xe0(%rbp)
1407f51e5: f3 44 0f 11 bd f0 00 00 00  	movss	%xmm15, 0xf0(%rbp)
1407f51ee: 8b 45 6c                    	movl	0x6c(%rbp), %eax
1407f51f1: 89 85 f4 00 00 00           	movl	%eax, 0xf4(%rbp)
1407f51f7: 44 0f 13 85 f8 00 00 00     	movlps	%xmm8, 0xf8(%rbp)
1407f51ff: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1407f5206: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407f520b: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
1407f520f: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
1407f5213: 4c 8d 85 80 01 00 00        	leaq	0x180(%rbp), %r8
1407f521a: 4c 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %r9
1407f5221: e8 1a e1 ff ff              	callq	0x1407f3340 <.text+0x7f2340>
1407f5226: 8b 8d c4 01 00 00           	movl	0x1c4(%rbp), %ecx
1407f522c: 84 c0                       	testb	%al, %al
1407f522e: 0f 85 98 ef ff ff           	jne	0x1407f41cc <.text+0x7f31cc>
1407f5234: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1407f523b: 48 8b 81 a8 fd ff ff        	movq	-0x258(%rcx), %rax
1407f5242: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f5246: 0f 84 89 06 00 00           	je	0x1407f58d5 <.text+0x7f48d5>
1407f524c: 48 85 c0                    	testq	%rax, %rax
1407f524f: 0f 84 09 07 00 00           	je	0x1407f595e <.text+0x7f495e>
1407f5255: 48 8b 89 a0 fd ff ff        	movq	-0x260(%rcx), %rcx
1407f525c: 48 89 8d 80 01 00 00        	movq	%rcx, 0x180(%rbp)
1407f5263: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1407f526a: 48 8d 05 cf 19 a1 00        	leaq	0xa119cf(%rip), %rax    # 0x141206c40 <.text+0x1205c40>
1407f5271: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407f5278: 0f 28 8d b0 00 00 00        	movaps	0xb0(%rbp), %xmm1
1407f527f: f3 0f 5f 0d 09 b4 16 02     	maxss	0x216b409(%rip), %xmm1  # 0x142960690
1407f5287: 48 8d 85 9c 01 00 00        	leaq	0x19c(%rbp), %rax
1407f528e: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1407f5292: f3 0f 7f 40 fc              	movdqu	%xmm0, -0x4(%rax)
1407f5297: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f529e: e8 3d 46 a9 00              	callq	0x1412898e0 <.text+0x12888e0>
1407f52a3: 8b 8d a0 01 00 00           	movl	0x1a0(%rbp), %ecx
1407f52a9: 4c 8b 85 40 01 00 00        	movq	0x140(%rbp), %r8
1407f52b0: 49 8b 90 a8 fd ff ff        	movq	-0x258(%r8), %rdx
1407f52b7: 48 39 ca                    	cmpq	%rcx, %rdx
1407f52ba: 0f 86 43 07 00 00           	jbe	0x1407f5a03 <.text+0x7f4a03>
1407f52c0: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
1407f52c4: 48 39 d0                    	cmpq	%rdx, %rax
1407f52c7: 0f 83 44 07 00 00           	jae	0x1407f5a11 <.text+0x7f4a11>
1407f52cd: f3 44 0f 10 85 a4 01 00 00  	movss	0x1a4(%rbp), %xmm8
1407f52d6: 49 8b 90 a0 fd ff ff        	movq	-0x260(%r8), %rdx
1407f52dd: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1407f52e1: f3 44 0f 10 74 8a 04        	movss	0x4(%rdx,%rcx,4), %xmm14
1407f52e8: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407f52ec: f3 44 0f 10 7c 82 04        	movss	0x4(%rdx,%rax,4), %xmm15
1407f52f3: 45 0f 28 da                 	movaps	%xmm10, %xmm11
1407f52f7: f3 45 0f 5c d8              	subss	%xmm8, %xmm11
1407f52fc: f3 0f 10 04 8a              	movss	(%rdx,%rcx,4), %xmm0
1407f5301: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
1407f5306: f3 0f 10 5c 8a 08           	movss	0x8(%rdx,%rcx,4), %xmm3
1407f530c: f3 41 0f 59 db              	mulss	%xmm11, %xmm3
1407f5311: f3 0f 10 0c 82              	movss	(%rdx,%rax,4), %xmm1
1407f5316: f3 41 0f 59 c8              	mulss	%xmm8, %xmm1
1407f531b: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f531f: f3 0f 10 54 82 08           	movss	0x8(%rdx,%rax,4), %xmm2
1407f5325: f3 41 0f 59 d0              	mulss	%xmm8, %xmm2
1407f532a: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1407f532e: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
1407f5332: 48 8b 42 08                 	movq	0x8(%rdx), %rax
1407f5336: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407f533a: 48 01 c9                    	addq	%rcx, %rcx
1407f533d: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407f5344: 48 89 8d 88 01 00 00        	movq	%rcx, 0x188(%rbp)
1407f534b: f2 0f 10 42 50              	movsd	0x50(%rdx), %xmm0
1407f5350: f2 0f 11 85 90 01 00 00     	movsd	%xmm0, 0x190(%rbp)
1407f5358: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f535f: e8 3c 28 a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f5364: f3 45 0f 59 de              	mulss	%xmm14, %xmm11
1407f5369: f3 45 0f 59 c7              	mulss	%xmm15, %xmm8
1407f536e: f3 45 0f 58 c3              	addss	%xmm11, %xmm8
1407f5373: f3 0f 58 05 a5 05 13 02     	addss	0x21305a5(%rip), %xmm0  # 0x142925920
1407f537b: 44 0f 2e c0                 	ucomiss	%xmm0, %xmm8
1407f537f: 8b 85 c4 01 00 00           	movl	0x1c4(%rbp), %eax
1407f5385: 89 c1                       	movl	%eax, %ecx
1407f5387: 0f 86 3f ee ff ff           	jbe	0x1407f41cc <.text+0x7f31cc>
1407f538d: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1407f5394: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407f5398: fe c8                       	decb	%al
1407f539a: 3c 03                       	cmpb	$0x3, %al
1407f539c: 0f 83 80 06 00 00           	jae	0x1407f5a22 <.text+0x7f4a22>
1407f53a2: 0f b6 c0                    	movzbl	%al, %eax
1407f53a5: 48 8d 0d 80 68 29 02        	leaq	0x2296880(%rip), %rcx   # 0x142a8bc2c
1407f53ac: 48 63 3c 81                 	movslq	(%rcx,%rax,4), %rdi
1407f53b0: 8b 85 d8 00 00 00           	movl	0xd8(%rbp), %eax
1407f53b6: 89 85 30 01 00 00           	movl	%eax, 0x130(%rbp)
1407f53bc: f3 0f 6f 85 c8 00 00 00     	movdqu	0xc8(%rbp), %xmm0
1407f53c4: 66 0f 7f 85 20 01 00 00     	movdqa	%xmm0, 0x120(%rbp)
1407f53cc: 48 8d 85 9c 01 00 00        	leaq	0x19c(%rbp), %rax
1407f53d3: c7 40 ef 00 00 00 00        	movl	$0x0, -0x11(%rax)
1407f53da: 48 c7 40 e7 00 00 00 00     	movq	$0x0, -0x19(%rax)
1407f53e2: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1407f53e9: 48 8b 8d 87 01 00 00        	movq	0x187(%rbp), %rcx
1407f53f0: 48 89 8d 17 01 00 00        	movq	%rcx, 0x117(%rbp)
1407f53f7: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1407f53fe: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1407f5405: 48 8b b0 c0 00 00 00        	movq	0xc0(%rax), %rsi
1407f540c: 48 3b b0 b0 00 00 00        	cmpq	0xb0(%rax), %rsi
1407f5413: 75 13                       	jne	0x1407f5428 <.text+0x7f4428>
1407f5415: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
1407f541c: 48 8d 15 4d 67 29 02        	leaq	0x229674d(%rip), %rdx   # 0x142a8bb70
1407f5423: e8 88 5f 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407f5428: 48 8d 05 fd 67 29 02        	leaq	0x22967fd(%rip), %rax   # 0x142a8bc2c
1407f542f: 48 01 c7                    	addq	%rax, %rdi
1407f5432: 4c 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %r9
1407f5439: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407f5440: 48 69 ce 60 01 00 00        	imulq	$0x160, %rsi, %rcx      # imm = 0x160
1407f5447: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407f544f: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407f5458: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407f5461: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407f5469: 48 8b 55 24                 	movq	0x24(%rbp), %rdx
1407f546d: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407f5472: 8b 55 2c                    	movl	0x2c(%rbp), %edx
1407f5475: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407f5479: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407f5481: 0f 10 85 80 01 00 00        	movups	0x180(%rbp), %xmm0
1407f5488: 0f 10 8d 90 01 00 00        	movups	0x190(%rbp), %xmm1
1407f548f: f3 0f 6f 95 a0 01 00 00     	movdqu	0x1a0(%rbp), %xmm2
1407f5497: 0f 10 9d ac 01 00 00        	movups	0x1ac(%rbp), %xmm3
1407f549e: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407f54a3: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407f54a8: f3 0f 7f 54 08 64           	movdqu	%xmm2, 0x64(%rax,%rcx)
1407f54ae: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407f54b3: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407f54be: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
1407f54c2: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407f54ca: 8b 55 38                    	movl	0x38(%rbp), %edx
1407f54cd: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407f54d4: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407f54df: 48 8b 55 3c                 	movq	0x3c(%rbp), %rdx
1407f54e3: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407f54eb: 8b 55 44                    	movl	0x44(%rbp), %edx
1407f54ee: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407f54f5: 48 89 bc 08 a0 00 00 00     	movq	%rdi, 0xa0(%rax,%rcx)
1407f54fd: 48 c7 84 08 a8 00 00 00 23 00 00 00 	movq	$0x23, 0xa8(%rax,%rcx)
1407f5509: 48 8d 15 41 66 29 02        	leaq	0x2296641(%rip), %rdx   # 0x142a8bb51
1407f5510: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407f5518: 48 c7 84 08 b8 00 00 00 1e 00 00 00 	movq	$0x1e, 0xb8(%rax,%rcx)
1407f5524: 0f 29 b4 08 c0 00 00 00     	movaps	%xmm6, 0xc0(%rax,%rcx)
1407f552c: 44 0f 13 ac 08 d0 00 00 00  	movlps	%xmm13, 0xd0(%rax,%rcx)
1407f5535: f3 44 0f 11 a4 08 d8 00 00 00       	movss	%xmm12, 0xd8(%rax,%rcx)
1407f553f: 8b 95 30 01 00 00           	movl	0x130(%rbp), %edx
1407f5545: 89 94 08 ec 00 00 00        	movl	%edx, 0xec(%rax,%rcx)
1407f554c: 0f 28 85 20 01 00 00        	movaps	0x120(%rbp), %xmm0
1407f5553: 0f 11 84 08 dc 00 00 00     	movups	%xmm0, 0xdc(%rax,%rcx)
1407f555b: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407f5566: 8b 95 04 01 00 00           	movl	0x104(%rbp), %edx
1407f556c: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407f5573: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407f557e: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1407f5585: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407f558d: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
1407f5594: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407f559c: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407f55a4: 0f b6 95 7f 01 00 00        	movzbl	0x17f(%rbp), %edx
1407f55ab: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407f55b2: 0f b7 95 7d 01 00 00        	movzwl	0x17d(%rbp), %edx
1407f55b9: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407f55c1: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407f55cc: f3 0f 6f 85 e0 00 00 00     	movdqu	0xe0(%rbp), %xmm0
1407f55d4: f3 0f 6f 8d f0 00 00 00     	movdqu	0xf0(%rbp), %xmm1
1407f55dc: f3 0f 7f 8c 08 2c 01 00 00  	movdqu	%xmm1, 0x12c(%rax,%rcx)
1407f55e5: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407f55ee: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407f55f6: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
1407f55fd: 4c 8b 85 17 01 00 00        	movq	0x117(%rbp), %r8
1407f5604: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407f560c: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407f5614: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407f561c: 0f b6 95 d6 01 00 00        	movzbl	0x1d6(%rbp), %edx
1407f5623: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407f562a: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407f5634: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1407f563b: 4c 8b 85 8f 00 00 00        	movq	0x8f(%rbp), %r8
1407f5642: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407f564a: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407f5652: 48 ff c6                    	incq	%rsi
1407f5655: 49 89 b1 c0 00 00 00        	movq	%rsi, 0xc0(%r9)
1407f565c: 8b 85 c4 01 00 00           	movl	0x1c4(%rbp), %eax
1407f5662: 89 c1                       	movl	%eax, %ecx
1407f5664: e9 63 eb ff ff              	jmp	0x1407f41cc <.text+0x7f31cc>
1407f5669: 8b 85 4c 01 00 00           	movl	0x14c(%rbp), %eax
1407f566f: 4c 8b 4d 78                 	movq	0x78(%rbp), %r9
1407f5673: 4c 89 4d 78                 	movq	%r9, 0x78(%rbp)
1407f5677: 4d 85 c9                    	testq	%r9, %r9
1407f567a: 0f 85 8a ea ff ff           	jne	0x1407f410a <.text+0x7f310a>
1407f5680: 80 bd d5 01 00 00 00        	cmpb	$0x0, 0x1d5(%rbp)
1407f5687: 74 48                       	je	0x1407f56d1 <.text+0x7f46d1>
1407f5689: e8 a2 2a 42 00              	callq	0x140c18130 <.text+0xc17130>
1407f568e: 48 89 c1                    	movq	%rax, %rcx
1407f5691: 48 8b 00                    	movq	(%rax), %rax
1407f5694: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f5698: 0f 85 48 02 00 00           	jne	0x1407f58e6 <.text+0x7f48e6>
1407f569e: 48 83 c1 08                 	addq	$0x8, %rcx
1407f56a2: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407f56a6: 0f 85 5e 02 00 00           	jne	0x1407f590a <.text+0x7f490a>
1407f56ac: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407f56b3: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
1407f56ba: 48 83 c1 08                 	addq	$0x8, %rcx
1407f56be: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
1407f56c2: e8 29 1a 42 00              	callq	0x140c170f0 <.text+0xc160f0>
1407f56c7: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
1407f56ce: 48 ff 00                    	incq	(%rax)
1407f56d1: 0f 28 b5 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm6
1407f56d8: 0f 28 bd f0 01 00 00        	movaps	0x1f0(%rbp), %xmm7
1407f56df: 44 0f 28 85 00 02 00 00     	movaps	0x200(%rbp), %xmm8
1407f56e7: 44 0f 28 8d 10 02 00 00     	movaps	0x210(%rbp), %xmm9
1407f56ef: 44 0f 28 95 20 02 00 00     	movaps	0x220(%rbp), %xmm10
1407f56f7: 44 0f 28 9d 30 02 00 00     	movaps	0x230(%rbp), %xmm11
1407f56ff: 44 0f 28 a5 40 02 00 00     	movaps	0x240(%rbp), %xmm12
1407f5707: 44 0f 28 ad 50 02 00 00     	movaps	0x250(%rbp), %xmm13
1407f570f: 44 0f 28 b5 60 02 00 00     	movaps	0x260(%rbp), %xmm14
1407f5717: 44 0f 28 bd 70 02 00 00     	movaps	0x270(%rbp), %xmm15
1407f571f: 48 81 c4 08 03 00 00        	addq	$0x308, %rsp            # imm = 0x308
1407f5726: 5b                          	popq	%rbx
1407f5727: 5f                          	popq	%rdi
1407f5728: 5e                          	popq	%rsi
1407f5729: 41 5c                       	popq	%r12
1407f572b: 41 5d                       	popq	%r13
1407f572d: 41 5e                       	popq	%r14
1407f572f: 41 5f                       	popq	%r15
1407f5731: 5d                          	popq	%rbp
1407f5732: c3                          	retq
1407f5733: 48 8d 0d ae bb 42 02        	leaq	0x242bbae(%rip), %rcx   # 0x142c212e8
1407f573a: e8 d1 3b 0e 02              	callq	0x1428d9310 <.text+0x28d8310>
1407f573f: e9 f6 02 00 00              	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f5744: 48 8d 0d 7d 5f 29 02        	leaq	0x2295f7d(%rip), %rcx   # 0x142a8b6c8
1407f574b: 4c 8d 05 c6 62 29 02        	leaq	0x22962c6(%rip), %r8    # 0x142a8ba18
1407f5752: ba 16 00 00 00              	movl	$0x16, %edx
1407f5757: e8 d4 3b 0e 02              	callq	0x1428d9330 <.text+0x28d8330>
1407f575c: e9 d9 02 00 00              	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f5761: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
1407f5768: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407f576f: 48 8d 05 0a e1 ff ff        	leaq	-0x1ef6(%rip), %rax     # 0x1407f3880 <.text+0x7f2880>
1407f5776: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407f577d: 48 8d 05 f4 5f 29 02        	leaq	0x2295ff4(%rip), %rax   # 0x142a8b778
1407f5784: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407f578b: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
1407f5796: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1407f57a1: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1407f57a8: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407f57af: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1407f57ba: 48 8d 15 c7 5f 29 02        	leaq	0x2295fc7(%rip), %rdx   # 0x142a8b788
1407f57c1: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f57c8: e8 c3 3b 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f57cd: e9 68 02 00 00              	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f57d2: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
1407f57d9: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407f57e0: 48 8d 05 99 e0 ff ff        	leaq	-0x1f67(%rip), %rax     # 0x1407f3880 <.text+0x7f2880>
1407f57e7: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407f57ee: 48 8d 05 83 5f 29 02        	leaq	0x2295f83(%rip), %rax   # 0x142a8b778
1407f57f5: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407f57fc: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
1407f5807: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1407f5812: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1407f5819: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407f5820: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1407f582b: c6 85 d7 01 00 00 01        	movb	$0x1, 0x1d7(%rbp)
1407f5832: 48 8d 15 67 5f 29 02        	leaq	0x2295f67(%rip), %rdx   # 0x142a8b7a0
1407f5839: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f5840: e8 4b 3b 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f5845: e9 f0 01 00 00              	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f584a: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
1407f5851: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407f5858: 48 8d 05 21 e0 ff ff        	leaq	-0x1fdf(%rip), %rax     # 0x1407f3880 <.text+0x7f2880>
1407f585f: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407f5866: 48 8d 05 0b 5f 29 02        	leaq	0x2295f0b(%rip), %rax   # 0x142a8b778
1407f586d: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407f5874: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
1407f587f: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1407f588a: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1407f5891: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407f5898: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1407f58a3: 48 8d 15 0e 5f 29 02        	leaq	0x2295f0e(%rip), %rdx   # 0x142a8b7b8
1407f58aa: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f58b1: e8 da 3a 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f58b6: e9 7f 01 00 00              	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f58bb: 48 85 c0                    	testq	%rax, %rax
1407f58be: 75 3e                       	jne	0x1407f58fe <.text+0x7f48fe>
1407f58c0: 31 d2                       	xorl	%edx, %edx
1407f58c2: e8 b9 cc 0f 02              	callq	0x1428f2580 <.text+0x28f1580>
1407f58c7: 48 89 c1                    	movq	%rax, %rcx
1407f58ca: 48 85 c0                    	testq	%rax, %rax
1407f58cd: 0f 85 29 e7 ff ff           	jne	0x1407f3ffc <.text+0x7f2ffc>
1407f58d3: eb 29                       	jmp	0x1407f58fe <.text+0x7f48fe>
1407f58d5: 48 8d 0d fc 33 43 02        	leaq	0x24333fc(%rip), %rcx   # 0x142c28cd8
1407f58dc: e8 2f 3a 0e 02              	callq	0x1428d9310 <.text+0x28d8310>
1407f58e1: e9 54 01 00 00              	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f58e6: 48 85 c0                    	testq	%rax, %rax
1407f58e9: 75 13                       	jne	0x1407f58fe <.text+0x7f48fe>
1407f58eb: 31 d2                       	xorl	%edx, %edx
1407f58ed: e8 8e cc 0f 02              	callq	0x1428f2580 <.text+0x28f1580>
1407f58f2: 48 89 c1                    	movq	%rax, %rcx
1407f58f5: 48 85 c0                    	testq	%rax, %rax
1407f58f8: 0f 85 a4 fd ff ff           	jne	0x1407f56a2 <.text+0x7f46a2>
1407f58fe: 48 8d 0d c3 ba 42 02        	leaq	0x242bac3(%rip), %rcx   # 0x142c213c8
1407f5905: e8 c6 27 0e 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407f590a: 48 8d 0d 27 bb 42 02        	leaq	0x242bb27(%rip), %rcx   # 0x142c21438
1407f5911: e8 3a 39 0e 02              	callq	0x1428d9250 <.text+0x28d8250>
1407f5916: 48 8d 05 0b be 29 02        	leaq	0x229be0b(%rip), %rax   # 0x142a91728
1407f591d: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407f5924: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
1407f592f: 48 c7 85 90 01 00 00 08 00 00 00    	movq	$0x8, 0x190(%rbp)
1407f593a: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1407f593e: f3 0f 7f 85 98 01 00 00     	movdqu	%xmm0, 0x198(%rbp)
1407f5946: 48 8d 15 eb bd 29 02        	leaq	0x229bdeb(%rip), %rdx   # 0x142a91738
1407f594d: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f5954: e8 37 3a 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f5959: e9 dc 00 00 00              	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f595e: 48 8d 0d 2b 33 43 02        	leaq	0x243332b(%rip), %rcx   # 0x142c28c90
1407f5965: 4c 8d 05 54 33 43 02        	leaq	0x2433354(%rip), %r8    # 0x142c28cc0
1407f596c: ba 2d 00 00 00              	movl	$0x2d, %edx
1407f5971: e8 ba 3a 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f5976: e9 bf 00 00 00              	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f597b: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
1407f5982: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407f5989: 48 8d 05 60 75 a6 00        	leaq	0xa67560(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
1407f5990: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407f5997: 48 8d 05 e2 c2 42 02        	leaq	0x242c2e2(%rip), %rax   # 0x142c21c80
1407f599e: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407f59a5: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
1407f59b0: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1407f59bb: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1407f59c2: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407f59c9: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1407f59d4: 48 8d 15 cd c2 42 02        	leaq	0x242c2cd(%rip), %rdx   # 0x142c21ca8
1407f59db: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407f59e2: e8 a9 39 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407f59e7: eb 51                       	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f59e9: 48 8d 0d 98 61 29 02        	leaq	0x2296198(%rip), %rcx   # 0x142a8bb88
1407f59f0: 4c 8d 05 d1 61 29 02        	leaq	0x22961d1(%rip), %r8    # 0x142a8bbc8
1407f59f7: ba 28 00 00 00              	movl	$0x28, %edx
1407f59fc: e8 2f 3a 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f5a01: eb 37                       	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f5a03: 4c 8d 05 de cb 42 02        	leaq	0x242cbde(%rip), %r8    # 0x142c225e8
1407f5a0a: e8 09 3b 0e 02              	callq	0x1428d9518 <.text+0x28d8518>
1407f5a0f: eb 29                       	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f5a11: 4c 8d 05 e8 cb 42 02        	leaq	0x242cbe8(%rip), %r8    # 0x142c22600
1407f5a18: 48 89 c1                    	movq	%rax, %rcx
1407f5a1b: e8 f8 3a 0e 02              	callq	0x1428d9518 <.text+0x28d8518>
1407f5a20: eb 18                       	jmp	0x1407f5a3a <.text+0x7f4a3a>
1407f5a22: 48 8d 0d 5f 61 29 02        	leaq	0x229615f(%rip), %rcx   # 0x142a8bb88
1407f5a29: 4c 8d 05 80 61 29 02        	leaq	0x2296180(%rip), %r8    # 0x142a8bbb0
1407f5a30: ba 28 00 00 00              	movl	$0x28, %edx
1407f5a35: e8 f6 39 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407f5a3a: 0f 0b                       	ud2
1407f5a3c: 0f 1f 40 00                 	nopl	(%rax)
1407f5a40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f5a45: 55                          	pushq	%rbp
1407f5a46: 41 57                       	pushq	%r15
1407f5a48: 41 56                       	pushq	%r14
1407f5a4a: 41 55                       	pushq	%r13
1407f5a4c: 41 54                       	pushq	%r12
1407f5a4e: 56                          	pushq	%rsi
1407f5a4f: 57                          	pushq	%rdi
1407f5a50: 53                          	pushq	%rbx
1407f5a51: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f5a58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f5a5f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f5a65: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f5a6b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f5a71: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f5a77: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407f5a7d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f5a86: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f5a8f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f5a98: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
1407f5aa1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f5aa9: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
1407f5ab0: 48 ff 00                    	incq	(%rax)
1407f5ab3: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f5abb: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f5ac3: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f5acc: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f5ad5: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f5ade: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407f5ae4: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f5aea: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f5af0: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f5af6: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f5afc: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f5b03: 5b                          	popq	%rbx
1407f5b04: 5f                          	popq	%rdi
1407f5b05: 5e                          	popq	%rsi
1407f5b06: 41 5c                       	popq	%r12
1407f5b08: 41 5d                       	popq	%r13
1407f5b0a: 41 5e                       	popq	%r14
1407f5b0c: 41 5f                       	popq	%r15
1407f5b0e: 5d                          	popq	%rbp
1407f5b0f: c3                          	retq
1407f5b10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f5b15: 55                          	pushq	%rbp
1407f5b16: 41 57                       	pushq	%r15
1407f5b18: 41 56                       	pushq	%r14
1407f5b1a: 41 55                       	pushq	%r13
1407f5b1c: 41 54                       	pushq	%r12
1407f5b1e: 56                          	pushq	%rsi
1407f5b1f: 57                          	pushq	%rdi
1407f5b20: 53                          	pushq	%rbx
1407f5b21: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f5b28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f5b2f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f5b35: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f5b3b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f5b41: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f5b47: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407f5b4d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f5b56: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f5b5f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f5b68: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
1407f5b71: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f5b79: 80 bd d5 01 00 00 00        	cmpb	$0x0, 0x1d5(%rbp)
1407f5b80: 74 3a                       	je	0x1407f5bbc <.text+0x7f4bbc>
1407f5b82: e8 a9 25 42 00              	callq	0x140c18130 <.text+0xc17130>
1407f5b87: 48 89 c6                    	movq	%rax, %rsi
1407f5b8a: 48 8b 00                    	movq	(%rax), %rax
1407f5b8d: 48 83 f8 01                 	cmpq	$0x1, %rax
1407f5b91: 0f 85 82 00 00 00           	jne	0x1407f5c19 <.text+0x7f4c19>
1407f5b97: 48 83 c6 08                 	addq	$0x8, %rsi
1407f5b9b: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
1407f5b9f: 0f 85 9b 00 00 00           	jne	0x1407f5c40 <.text+0x7f4c40>
1407f5ba5: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
1407f5bac: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
1407f5bb0: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
1407f5bb4: e8 37 15 42 00              	callq	0x140c170f0 <.text+0xc160f0>
1407f5bb9: 48 ff 06                    	incq	(%rsi)
1407f5bbc: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f5bc4: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f5bcc: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f5bd5: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f5bde: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f5be7: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407f5bed: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f5bf3: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f5bf9: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f5bff: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f5c05: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f5c0c: 5b                          	popq	%rbx
1407f5c0d: 5f                          	popq	%rdi
1407f5c0e: 5e                          	popq	%rsi
1407f5c0f: 41 5c                       	popq	%r12
1407f5c11: 41 5d                       	popq	%r13
1407f5c13: 41 5e                       	popq	%r14
1407f5c15: 41 5f                       	popq	%r15
1407f5c17: 5d                          	popq	%rbp
1407f5c18: c3                          	retq
1407f5c19: 48 85 c0                    	testq	%rax, %rax
1407f5c1c: 75 16                       	jne	0x1407f5c34 <.text+0x7f4c34>
1407f5c1e: 48 89 f1                    	movq	%rsi, %rcx
1407f5c21: 31 d2                       	xorl	%edx, %edx
1407f5c23: e8 58 c9 0f 02              	callq	0x1428f2580 <.text+0x28f1580>
1407f5c28: 48 89 c6                    	movq	%rax, %rsi
1407f5c2b: 48 85 c0                    	testq	%rax, %rax
1407f5c2e: 0f 85 67 ff ff ff           	jne	0x1407f5b9b <.text+0x7f4b9b>
1407f5c34: 48 8d 0d 8d b7 42 02        	leaq	0x242b78d(%rip), %rcx   # 0x142c213c8
1407f5c3b: e8 90 24 0e 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407f5c40: 48 8d 0d f1 b7 42 02        	leaq	0x242b7f1(%rip), %rcx   # 0x142c21438
1407f5c47: e8 04 36 0e 02              	callq	0x1428d9250 <.text+0x28d8250>
1407f5c4c: cc                          	int3
1407f5c4d: 0f 1f 00                    	nopl	(%rax)
1407f5c50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f5c55: 55                          	pushq	%rbp
1407f5c56: 41 57                       	pushq	%r15
1407f5c58: 41 56                       	pushq	%r14
1407f5c5a: 41 55                       	pushq	%r13
1407f5c5c: 41 54                       	pushq	%r12
1407f5c5e: 56                          	pushq	%rsi
1407f5c5f: 57                          	pushq	%rdi
1407f5c60: 53                          	pushq	%rbx
1407f5c61: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f5c68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f5c6f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f5c75: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f5c7b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f5c81: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f5c87: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407f5c8d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f5c96: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f5c9f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f5ca8: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
1407f5cb1: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f5cb9: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
1407f5cc0: 48 ff 00                    	incq	(%rax)
1407f5cc3: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f5ccb: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407f5cd3: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f5cdc: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f5ce5: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f5cee: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407f5cf4: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f5cfa: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f5d00: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f5d06: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f5d0c: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f5d13: 5b                          	popq	%rbx
1407f5d14: 5f                          	popq	%rdi
1407f5d15: 5e                          	popq	%rsi
1407f5d16: 41 5c                       	popq	%r12
1407f5d18: 41 5d                       	popq	%r13
1407f5d1a: 41 5e                       	popq	%r14
1407f5d1c: 41 5f                       	popq	%r15
1407f5d1e: 5d                          	popq	%rbp
1407f5d1f: c3                          	retq
1407f5d20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407f5d25: 55                          	pushq	%rbp
1407f5d26: 41 57                       	pushq	%r15
1407f5d28: 41 56                       	pushq	%r14
1407f5d2a: 41 55                       	pushq	%r13
1407f5d2c: 41 54                       	pushq	%r12
1407f5d2e: 56                          	pushq	%rsi
1407f5d2f: 57                          	pushq	%rdi
1407f5d30: 53                          	pushq	%rbx
1407f5d31: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407f5d38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407f5d3f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407f5d45: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407f5d4b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407f5d51: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407f5d57: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407f5d5d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407f5d66: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407f5d6f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f5d78: 66 0f 7f bc 24 b0 00 00 00  	movdqa	%xmm7, 0xb0(%rsp)
1407f5d81: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407f5d89: 80 bd d7 01 00 00 00        	cmpb	$0x0, 0x1d7(%rbp)
1407f5d90: 74 77                       	je	0x1407f5e09 <.text+0x7f4e09>
1407f5d92: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1407f5d99: f0                          	lock
1407f5d9a: 48 ff 08                    	decq	(%rax)
1407f5d9d: 75 0c                       	jne	0x1407f5dab <.text+0x7f4dab>
1407f5d9f: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1407f5da6: e8 75 68 36 00              	callq	0x140b5c620 <.text+0xb5b620>
1407f5dab: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f5db3: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
1407f5dbc: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f5dc5: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f5dce: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f5dd7: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407f5ddd: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f5de3: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f5de9: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f5def: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f5df5: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f5dfc: 5b                          	popq	%rbx
1407f5dfd: 5f                          	popq	%rdi
1407f5dfe: 5e                          	popq	%rsi
1407f5dff: 41 5c                       	popq	%r12
1407f5e01: 41 5d                       	popq	%r13
1407f5e03: 41 5e                       	popq	%r14
1407f5e05: 41 5f                       	popq	%r15
1407f5e07: 5d                          	popq	%rbp
1407f5e08: c3                          	retq
1407f5e09: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407f5e11: 66 0f 6f bc 24 b0 00 00 00  	movdqa	0xb0(%rsp), %xmm7
1407f5e1a: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f5e23: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407f5e2c: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407f5e35: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407f5e3b: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407f5e41: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407f5e47: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407f5e4d: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407f5e53: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407f5e5a: 5b                          	popq	%rbx
1407f5e5b: 5f                          	popq	%rdi
1407f5e5c: 5e                          	popq	%rsi
1407f5e5d: 41 5c                       	popq	%r12
1407f5e5f: 41 5d                       	popq	%r13
1407f5e61: 41 5e                       	popq	%r14
1407f5e63: 41 5f                       	popq	%r15
1407f5e65: 5d                          	popq	%rbp
1407f5e66: c3                          	retq
1407f5e67: cc                          	int3
1407f5e68: cc                          	int3
1407f5e69: cc                          	int3
1407f5e6a: cc                          	int3
1407f5e6b: cc                          	int3
1407f5e6c: cc                          	int3
1407f5e6d: cc                          	int3
1407f5e6e: cc                          	int3
1407f5e6f: cc                          	int3
