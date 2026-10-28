
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140952e70: 55                          	pushq	%rbp
140952e71: 41 57                       	pushq	%r15
140952e73: 41 56                       	pushq	%r14
140952e75: 41 55                       	pushq	%r13
140952e77: 41 54                       	pushq	%r12
140952e79: 56                          	pushq	%rsi
140952e7a: 57                          	pushq	%rdi
140952e7b: 53                          	pushq	%rbx
140952e7c: 48 81 ec a8 02 00 00        	subq	$0x2a8, %rsp            # imm = 0x2A8
140952e83: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140952e8b: 44 0f 29 ad 10 02 00 00     	movaps	%xmm13, 0x210(%rbp)
140952e93: 44 0f 29 a5 00 02 00 00     	movaps	%xmm12, 0x200(%rbp)
140952e9b: 44 0f 29 9d f0 01 00 00     	movaps	%xmm11, 0x1f0(%rbp)
140952ea3: 44 0f 29 95 e0 01 00 00     	movaps	%xmm10, 0x1e0(%rbp)
140952eab: 44 0f 29 8d d0 01 00 00     	movaps	%xmm9, 0x1d0(%rbp)
140952eb3: 44 0f 29 85 c0 01 00 00     	movaps	%xmm8, 0x1c0(%rbp)
140952ebb: 0f 29 bd b0 01 00 00        	movaps	%xmm7, 0x1b0(%rbp)
140952ec2: 66 0f 7f b5 a0 01 00 00     	movdqa	%xmm6, 0x1a0(%rbp)
140952eca: 48 c7 85 98 01 00 00 fe ff ff ff    	movq	$-0x2, 0x198(%rbp)
140952ed5: 4c 89 cf                    	movq	%r9, %rdi
140952ed8: 4c 89 85 80 01 00 00        	movq	%r8, 0x180(%rbp)
140952edf: 48 89 d3                    	movq	%rdx, %rbx
140952ee2: 49 89 ce                    	movq	%rcx, %r14
140952ee5: e8 16 3b 2c 00              	callq	0x140c16a00 <.text+0xc15a00>
140952eea: 84 c0                       	testb	%al, %al
140952eec: 74 6c                       	je	0x140952f5a <.text+0x951f5a>
140952eee: 4c 8d 25 2c 0f 16 02        	leaq	0x2160f2c(%rip), %r12   # 0x142ab3e21
140952ef5: 41 bf 35 00 00 00           	movl	$0x35, %r15d
140952efb: ba 35 00 00 00              	movl	$0x35, %edx
140952f00: 4c 89 e1                    	movq	%r12, %rcx
140952f03: e8 18 c7 ff ff              	callq	0x14094f620 <.text+0x94e620>
140952f08: 48 83 f8 01                 	cmpq	$0x1, %rax
140952f0c: 0f 85 72 01 00 00           	jne	0x140953084 <.text+0x952084>
140952f12: 49 89 d1                    	movq	%rdx, %r9
140952f15: 48 85 d2                    	testq	%rdx, %rdx
140952f18: 0f 84 f9 00 00 00           	je	0x140953017 <.text+0x952017>
140952f1e: 49 83 f9 35                 	cmpq	$0x35, %r9
140952f22: 0f 83 e9 00 00 00           	jae	0x140953011 <.text+0x952011>
140952f28: 48 8d 05 f2 0e 16 02        	leaq	0x2160ef2(%rip), %rax   # 0x142ab3e21
140952f2f: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140952f34: 0f 8f dd 00 00 00           	jg	0x140953017 <.text+0x952017>
140952f3a: 48 8d 05 e7 0f 16 02        	leaq	0x2160fe7(%rip), %rax   # 0x142ab3f28
140952f41: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140952f46: 48 8d 0d d4 0e 16 02        	leaq	0x2160ed4(%rip), %rcx   # 0x142ab3e21
140952f4d: ba 35 00 00 00              	movl	$0x35, %edx
140952f52: 45 31 c0                    	xorl	%r8d, %r8d
140952f55: e8 e6 69 f8 01              	callq	0x1428d9940 <.text+0x28d8940>
140952f5a: 31 c0                       	xorl	%eax, %eax
140952f5c: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
140952f63: 49 8b 0e                    	movq	(%r14), %rcx
140952f66: 83 39 00                    	cmpl	$0x0, (%rcx)
140952f69: 0f 84 91 02 00 00           	je	0x140953200 <.text+0x952200>
140952f6f: 48 85 c0                    	testq	%rax, %rax
140952f72: 74 4b                       	je	0x140952fbf <.text+0x951fbf>
140952f74: e8 b7 51 2c 00              	callq	0x140c18130 <.text+0xc17130>
140952f79: 48 89 c1                    	movq	%rax, %rcx
140952f7c: 48 8b 00                    	movq	(%rax), %rax
140952f7f: 48 83 f8 01                 	cmpq	$0x1, %rax
140952f83: 0f 85 4b 0a 00 00           	jne	0x1409539d4 <.text+0x9529d4>
140952f89: 48 83 c1 08                 	addq	$0x8, %rcx
140952f8d: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140952f91: 0f 85 21 0f 00 00           	jne	0x140953eb8 <.text+0x952eb8>
140952f97: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140952f9e: 48 89 8d 80 01 00 00        	movq	%rcx, 0x180(%rbp)
140952fa5: 48 83 c1 08                 	addq	$0x8, %rcx
140952fa9: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
140952fb0: e8 3b 41 2c 00              	callq	0x140c170f0 <.text+0xc160f0>
140952fb5: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
140952fbc: 48 ff 00                    	incq	(%rax)
140952fbf: 0f 28 b5 a0 01 00 00        	movaps	0x1a0(%rbp), %xmm6
140952fc6: 0f 28 bd b0 01 00 00        	movaps	0x1b0(%rbp), %xmm7
140952fcd: 44 0f 28 85 c0 01 00 00     	movaps	0x1c0(%rbp), %xmm8
140952fd5: 44 0f 28 8d d0 01 00 00     	movaps	0x1d0(%rbp), %xmm9
140952fdd: 44 0f 28 95 e0 01 00 00     	movaps	0x1e0(%rbp), %xmm10
140952fe5: 44 0f 28 9d f0 01 00 00     	movaps	0x1f0(%rbp), %xmm11
140952fed: 44 0f 28 a5 00 02 00 00     	movaps	0x200(%rbp), %xmm12
140952ff5: 44 0f 28 ad 10 02 00 00     	movaps	0x210(%rbp), %xmm13
140952ffd: 48 81 c4 a8 02 00 00        	addq	$0x2a8, %rsp            # imm = 0x2A8
140953004: 5b                          	popq	%rbx
140953005: 5f                          	popq	%rdi
140953006: 5e                          	popq	%rsi
140953007: 41 5c                       	popq	%r12
140953009: 41 5d                       	popq	%r13
14095300b: 41 5e                       	popq	%r14
14095300d: 41 5f                       	popq	%r15
14095300f: 5d                          	popq	%rbp
140953010: c3                          	retq
140953011: 0f 85 23 ff ff ff           	jne	0x140952f3a <.text+0x951f3a>
140953017: 4c 8d 25 03 0e 16 02        	leaq	0x2160e03(%rip), %r12   # 0x142ab3e21
14095301e: 4c 89 e1                    	movq	%r12, %rcx
140953021: 4c 89 ca                    	movq	%r9, %rdx
140953024: e8 f7 c5 ff ff              	callq	0x14094f620 <.text+0x94e620>
140953029: 48 83 f8 01                 	cmpq	$0x1, %rax
14095302d: 75 55                       	jne	0x140953084 <.text+0x952084>
14095302f: 49 89 d0                    	movq	%rdx, %r8
140953032: 49 83 c0 02                 	addq	$0x2, %r8
140953036: 74 39                       	je	0x140953071 <.text+0x952071>
140953038: 49 83 f8 35                 	cmpq	$0x35, %r8
14095303c: 73 31                       	jae	0x14095306f <.text+0x95206f>
14095303e: 48 8d 05 dc 0d 16 02        	leaq	0x2160ddc(%rip), %rax   # 0x142ab3e21
140953045: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14095304a: 7f 25                       	jg	0x140953071 <.text+0x952071>
14095304c: 48 8d 05 ed 0e 16 02        	leaq	0x2160eed(%rip), %rax   # 0x142ab3f40
140953053: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140953058: 48 8d 0d c2 0d 16 02        	leaq	0x2160dc2(%rip), %rcx   # 0x142ab3e21
14095305f: ba 35 00 00 00              	movl	$0x35, %edx
140953064: 41 b9 35 00 00 00           	movl	$0x35, %r9d
14095306a: e8 d1 68 f8 01              	callq	0x1428d9940 <.text+0x28d8940>
14095306f: 75 db                       	jne	0x14095304c <.text+0x95204c>
140953071: 41 bf 33 00 00 00           	movl	$0x33, %r15d
140953077: 49 29 d7                    	subq	%rdx, %r15
14095307a: 4c 8d 25 a0 0d 16 02        	leaq	0x2160da0(%rip), %r12   # 0x142ab3e21
140953081: 4d 01 c4                    	addq	%r8, %r12
140953084: 48 8d 05 72 14 16 02        	leaq	0x2161472(%rip), %rax   # 0x142ab44fd
14095308b: 4c 8d 2d 3a 14 16 02        	leaq	0x216143a(%rip), %r13   # 0x142ab44cc
140953092: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409530a0: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
1409530a4: 85 c9                       	testl	%ecx, %ecx
1409530a6: 78 18                       	js	0x1409530c0 <.text+0x9520c0>
1409530a8: 48 ff c8                    	decq	%rax
1409530ab: 83 f9 5c                    	cmpl	$0x5c, %ecx
1409530ae: 75 69                       	jne	0x140953119 <.text+0x952119>
1409530b0: eb 7c                       	jmp	0x14095312e <.text+0x95212e>
1409530b2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409530c0: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
1409530c4: 80 fa c0                    	cmpb	$-0x40, %dl
1409530c7: 7d 25                       	jge	0x1409530ee <.text+0x9520ee>
1409530c9: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
1409530ce: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
1409530d2: 7d 23                       	jge	0x1409530f7 <.text+0x9520f7>
1409530d4: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
1409530d9: 48 83 c0 fc                 	addq	$-0x4, %rax
1409530dd: 41 83 e1 07                 	andl	$0x7, %r9d
1409530e1: 41 c1 e1 06                 	shll	$0x6, %r9d
1409530e5: 41 83 e0 3f                 	andl	$0x3f, %r8d
1409530e9: 45 09 c8                    	orl	%r9d, %r8d
1409530ec: eb 11                       	jmp	0x1409530ff <.text+0x9520ff>
1409530ee: 48 83 c0 fe                 	addq	$-0x2, %rax
1409530f2: 83 e2 1f                    	andl	$0x1f, %edx
1409530f5: eb 12                       	jmp	0x140953109 <.text+0x952109>
1409530f7: 48 83 c0 fd                 	addq	$-0x3, %rax
1409530fb: 41 83 e0 0f                 	andl	$0xf, %r8d
1409530ff: 41 c1 e0 06                 	shll	$0x6, %r8d
140953103: 83 e2 3f                    	andl	$0x3f, %edx
140953106: 44 09 c2                    	orl	%r8d, %edx
140953109: c1 e2 06                    	shll	$0x6, %edx
14095310c: 80 e1 3f                    	andb	$0x3f, %cl
14095310f: 0f b6 c9                    	movzbl	%cl, %ecx
140953112: 09 d1                       	orl	%edx, %ecx
140953114: 83 f9 5c                    	cmpl	$0x5c, %ecx
140953117: 74 15                       	je	0x14095312e <.text+0x95212e>
140953119: 83 f9 2f                    	cmpl	$0x2f, %ecx
14095311c: 74 10                       	je	0x14095312e <.text+0x95212e>
14095311e: 4c 39 e8                    	cmpq	%r13, %rax
140953121: 0f 85 79 ff ff ff           	jne	0x1409530a0 <.text+0x9520a0>
140953127: be 31 00 00 00              	movl	$0x31, %esi
14095312c: eb 4f                       	jmp	0x14095317d <.text+0x95217d>
14095312e: 4c 8d 2d 97 13 16 02        	leaq	0x2161397(%rip), %r13   # 0x142ab44cc
140953135: 4c 29 e8                    	subq	%r13, %rax
140953138: 49 89 c0                    	movq	%rax, %r8
14095313b: 49 ff c0                    	incq	%r8
14095313e: 74 32                       	je	0x140953172 <.text+0x952172>
140953140: 49 83 f8 31                 	cmpq	$0x31, %r8
140953144: 73 2a                       	jae	0x140953170 <.text+0x952170>
140953146: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
14095314b: 7f 25                       	jg	0x140953172 <.text+0x952172>
14095314d: 48 8d 05 b4 0d 16 02        	leaq	0x2160db4(%rip), %rax   # 0x142ab3f08
140953154: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140953159: 48 8d 0d 6c 13 16 02        	leaq	0x216136c(%rip), %rcx   # 0x142ab44cc
140953160: ba 31 00 00 00              	movl	$0x31, %edx
140953165: 41 b9 31 00 00 00           	movl	$0x31, %r9d
14095316b: e8 d0 67 f8 01              	callq	0x1428d9940 <.text+0x28d8940>
140953170: 75 db                       	jne	0x14095314d <.text+0x95214d>
140953172: be 30 00 00 00              	movl	$0x30, %esi
140953177: 48 29 c6                    	subq	%rax, %rsi
14095317a: 4d 01 c5                    	addq	%r8, %r13
14095317d: e8 ae 4f 2c 00              	callq	0x140c18130 <.text+0xc17130>
140953182: 48 89 c1                    	movq	%rax, %rcx
140953185: 48 8b 00                    	movq	(%rax), %rax
140953188: 48 83 f8 01                 	cmpq	$0x1, %rax
14095318c: 0f 85 51 07 00 00           	jne	0x1409538e3 <.text+0x9528e3>
140953192: 48 83 c1 08                 	addq	$0x8, %rcx
140953196: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14095319a: 0f 85 18 0d 00 00           	jne	0x140953eb8 <.text+0x952eb8>
1409531a0: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409531a7: 48 89 8d 50 01 00 00        	movq	%rcx, 0x150(%rbp)
1409531ae: 48 83 c1 08                 	addq	$0x8, %rcx
1409531b2: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1409531b7: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1409531c0: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1409531c9: 4c 89 e2                    	movq	%r12, %rdx
1409531cc: 4d 89 f8                    	movq	%r15, %r8
1409531cf: 4d 89 e9                    	movq	%r13, %r9
1409531d2: e8 69 3b 2c 00              	callq	0x140c16d40 <.text+0xc15d40>
1409531d7: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
1409531de: 48 ff 01                    	incq	(%rcx)
1409531e1: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1409531e8: b8 01 00 00 00              	movl	$0x1, %eax
1409531ed: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1409531f4: 49 8b 0e                    	movq	(%r14), %rcx
1409531f7: 83 39 00                    	cmpl	$0x0, (%rcx)
1409531fa: 0f 85 6f fd ff ff           	jne	0x140952f6f <.text+0x951f6f>
140953200: 48 8b 1b                    	movq	(%rbx), %rbx
140953203: 48 8b 33                    	movq	(%rbx), %rsi
140953206: 4c 8b 63 18                 	movq	0x18(%rbx), %r12
14095320a: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
14095320e: 66 44 0f d7 f8              	pmovmskb	%xmm0, %r15d
140953213: 41 f7 d7                    	notl	%r15d
140953216: 4c 8d 76 10                 	leaq	0x10(%rsi), %r14
14095321a: 48 8b 07                    	movq	(%rdi), %rax
14095321d: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
140953224: 48 8d 7b 20                 	leaq	0x20(%rbx), %rdi
140953228: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
14095322c: 0f 57 ff                    	xorps	%xmm7, %xmm7
14095322f: f3 44 0f 10 05 40 9e 07 02  	movss	0x2079e40(%rip), %xmm8  # 0x1429cd078
140953238: f3 44 0f 10 0d 2b bf 08 02  	movss	0x208bf2b(%rip), %xmm9  # 0x1429df16c
140953241: 48 c7 85 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rbp)
14095324c: 44 89 f8                    	movl	%r15d, %eax
14095324f: 49 83 ec 01                 	subq	$0x1, %r12
140953253: 0f 82 ab 06 00 00           	jb	0x140953904 <.text+0x952904>
140953259: 66 85 c0                    	testw	%ax, %ax
14095325c: 74 12                       	je	0x140953270 <.text+0x952270>
14095325e: 44 8d 78 ff                 	leal	-0x1(%rax), %r15d
140953262: 41 21 c7                    	andl	%eax, %r15d
140953265: eb 33                       	jmp	0x14095329a <.text+0x95229a>
140953267: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140953270: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
140953275: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140953279: 48 81 c6 80 d5 ff ff        	addq	$-0x2a80, %rsi          # imm = 0xD580
140953280: 49 83 c6 10                 	addq	$0x10, %r14
140953284: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140953289: 74 e5                       	je	0x140953270 <.text+0x952270>
14095328b: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140953290: 29 c1                       	subl	%eax, %ecx
140953292: f7 d0                       	notl	%eax
140953294: 41 89 c7                    	movl	%eax, %r15d
140953297: 41 21 cf                    	andl	%ecx, %r15d
14095329a: f3 0f bc c0                 	tzcntl	%eax, %eax
14095329e: 48 f7 d8                    	negq	%rax
1409532a1: 48 69 c0 a8 02 00 00        	imulq	$0x2a8, %rax, %rax      # imm = 0x2A8
1409532a8: 4c 8b ac 06 58 fd ff ff     	movq	-0x2a8(%rsi,%rax), %r13
1409532b0: 4c 89 ad 08 01 00 00        	movq	%r13, 0x108(%rbp)
1409532b7: 4c 89 6d 38                 	movq	%r13, 0x38(%rbp)
1409532bb: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
1409532c0: 0f 84 0c 06 00 00           	je	0x1409538d2 <.text+0x9528d2>
1409532c6: 48 89 f9                    	movq	%rdi, %rcx
1409532c9: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
1409532cd: e8 9e da c5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1409532d2: 48 8b 0b                    	movq	(%rbx), %rcx
1409532d5: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
1409532d9: 49 89 d0                    	movq	%rdx, %r8
1409532dc: 49 21 c0                    	andq	%rax, %r8
1409532df: 48 c1 e8 39                 	shrq	$0x39, %rax
1409532e3: 66 0f 6e c0                 	movd	%eax, %xmm0
1409532e7: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409532eb: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409532f0: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409532f5: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
1409532fc: 45 31 c9                    	xorl	%r9d, %r9d
1409532ff: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140953305: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140953309: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14095330d: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140953312: 45 85 d2                    	testl	%r10d, %r10d
140953315: 74 29                       	je	0x140953340 <.text+0x952340>
140953317: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14095331c: 4d 01 c3                    	addq	%r8, %r11
14095331f: 49 21 d3                    	andq	%rdx, %r11
140953322: 49 f7 db                    	negq	%r11
140953325: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
14095332c: 4e 39 2c 18                 	cmpq	%r13, (%rax,%r11)
140953330: 74 3e                       	je	0x140953370 <.text+0x952370>
140953332: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140953336: 66 45 21 d3                 	andw	%r10w, %r11w
14095333a: 45 89 da                    	movl	%r11d, %r10d
14095333d: 75 d8                       	jne	0x140953317 <.text+0x952317>
14095333f: 90                          	nop
140953340: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140953344: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140953349: 45 85 d2                    	testl	%r10d, %r10d
14095334c: 0f 85 80 05 00 00           	jne	0x1409538d2 <.text+0x9528d2>
140953352: 4d 01 c8                    	addq	%r9, %r8
140953355: 49 83 c0 10                 	addq	$0x10, %r8
140953359: 49 83 c1 10                 	addq	$0x10, %r9
14095335d: 49 21 d0                    	andq	%rdx, %r8
140953360: eb 9d                       	jmp	0x1409532ff <.text+0x9522ff>
140953362: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140953370: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
140953379: 0f 82 d0 0f 00 00           	jb	0x14095434f <.text+0x95334f>
14095337f: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
140953389: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
14095338c: 0f 86 bd 0f 00 00           	jbe	0x14095434f <.text+0x95334f>
140953392: 4c 01 d9                    	addq	%r11, %rcx
140953395: 83 79 b8 01                 	cmpl	$0x1, -0x48(%rcx)
140953399: 44 89 f8                    	movl	%r15d, %eax
14095339c: 0f 87 ad fe ff ff           	ja	0x14095324f <.text+0x95224f>
1409533a2: 4c 8b ad 08 01 00 00        	movq	0x108(%rbp), %r13
1409533a9: 4c 89 6d 38                 	movq	%r13, 0x38(%rbp)
1409533ad: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
1409533b2: 0f 84 d5 06 00 00           	je	0x140953a8d <.text+0x952a8d>
1409533b8: 48 89 f9                    	movq	%rdi, %rcx
1409533bb: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
1409533bf: e8 ac d9 c5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1409533c4: 4c 8b 0b                    	movq	(%rbx), %r9
1409533c7: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
1409533cb: 49 89 d0                    	movq	%rdx, %r8
1409533ce: 49 21 c0                    	andq	%rax, %r8
1409533d1: 48 c1 e8 39                 	shrq	$0x39, %rax
1409533d5: 66 0f 6e c0                 	movd	%eax, %xmm0
1409533d9: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409533dd: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409533e2: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409533e7: 49 8d 81 58 fd ff ff        	leaq	-0x2a8(%r9), %rax
1409533ee: 31 c9                       	xorl	%ecx, %ecx
1409533f0: f3 43 0f 6f 0c 01           	movdqu	(%r9,%r8), %xmm1
1409533f6: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1409533fa: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1409533fe: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140953403: 45 85 d2                    	testl	%r10d, %r10d
140953406: 74 28                       	je	0x140953430 <.text+0x952430>
140953408: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14095340d: 4d 01 c3                    	addq	%r8, %r11
140953410: 49 21 d3                    	andq	%rdx, %r11
140953413: 49 f7 db                    	negq	%r11
140953416: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
14095341d: 4e 39 2c 18                 	cmpq	%r13, (%rax,%r11)
140953421: 74 2f                       	je	0x140953452 <.text+0x952452>
140953423: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140953427: 66 45 21 d3                 	andw	%r10w, %r11w
14095342b: 45 89 da                    	movl	%r11d, %r10d
14095342e: 75 d8                       	jne	0x140953408 <.text+0x952408>
140953430: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140953434: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140953439: 45 85 d2                    	testl	%r10d, %r10d
14095343c: 0f 85 4b 06 00 00           	jne	0x140953a8d <.text+0x952a8d>
140953442: 49 01 c8                    	addq	%rcx, %r8
140953445: 49 83 c0 10                 	addq	$0x10, %r8
140953449: 48 83 c1 10                 	addq	$0x10, %rcx
14095344d: 49 21 d0                    	andq	%rdx, %r8
140953450: eb 9e                       	jmp	0x1409533f0 <.text+0x9523f0>
140953452: 4b 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%r9,%r11)
14095345b: 0f 82 0b 0f 00 00           	jb	0x14095436c <.text+0x95336c>
140953461: f3 43 0f 10 84 19 90 fd ff ff       	movss	-0x270(%r9,%r11), %xmm0
14095346b: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
14095346e: 0f 86 f8 0e 00 00           	jbe	0x14095436c <.text+0x95336c>
140953474: 4d 01 d9                    	addq	%r11, %r9
140953477: 45 0f 2e 41 e4              	ucomiss	-0x1c(%r9), %xmm8
14095347c: 0f 86 85 00 00 00           	jbe	0x140953507 <.text+0x952507>
140953482: 49 8b 91 a8 fd ff ff        	movq	-0x258(%r9), %rdx
140953489: 48 8d 04 95 00 00 00 00     	leaq	(,%rdx,4), %rax
140953491: 48 8d 0c 40                 	leaq	(%rax,%rax,2), %rcx
140953495: 48 b8 aa aa aa aa aa aa aa 0a       	movabsq	$0xaaaaaaaaaaaaaaa, %rax # imm = 0xAAAAAAAAAAAAAAA
14095349f: 48 39 c2                    	cmpq	%rax, %rdx
1409534a2: 0f 87 88 0f 00 00           	ja	0x140954430 <.text+0x953430>
1409534a8: 48 85 d2                    	testq	%rdx, %rdx
1409534ab: 0f 84 17 01 00 00           	je	0x1409535c8 <.text+0x9525c8>
1409534b1: 48 89 95 78 01 00 00        	movq	%rdx, 0x178(%rbp)
1409534b8: 4d 8b a9 a0 fd ff ff        	movq	-0x260(%r9), %r13
1409534bf: 0f b6 05 43 17 ce 02        	movzbl	0x2ce1743(%rip), %eax   # 0x143634c09
1409534c6: b8 04 00 00 00              	movl	$0x4, %eax
1409534cb: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1409534d2: ba 04 00 00 00              	movl	$0x4, %edx
1409534d7: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
1409534de: e8 2d 07 cc ff              	callq	0x140613c10 <.text+0x612c10>
1409534e3: 48 85 c0                    	testq	%rax, %rax
1409534e6: 0f 84 56 0f 00 00           	je	0x140954442 <.text+0x953442>
1409534ec: 49 89 c0                    	movq	%rax, %r8
1409534ef: 4c 8b 8d 78 01 00 00        	movq	0x178(%rbp), %r9
1409534f6: 49 83 f9 01                 	cmpq	$0x1, %r9
1409534fa: 0f 85 e7 00 00 00           	jne	0x1409535e7 <.text+0x9525e7>
140953500: 31 c0                       	xorl	%eax, %eax
140953502: e9 61 01 00 00              	jmp	0x140953668 <.text+0x952668>
140953507: 49 8b 81 a8 fd ff ff        	movq	-0x258(%r9), %rax
14095350e: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140953515: ba 0c 00 00 00              	movl	$0xc, %edx
14095351a: 48 f7 e2                    	mulq	%rdx
14095351d: 49 89 c2                    	movq	%rax, %r10
140953520: 0f 80 3b 0f 00 00           	jo	0x140954461 <.text+0x953461>
140953526: 41 bd 00 00 00 00           	movl	$0x0, %r13d
14095352c: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140953536: 49 39 c2                    	cmpq	%rax, %r10
140953539: 0f 87 25 0f 00 00           	ja	0x140954464 <.text+0x953464>
14095353f: 49 8b 91 a0 fd ff ff        	movq	-0x260(%r9), %rdx
140953546: 4d 85 d2                    	testq	%r10, %r10
140953549: 74 4f                       	je	0x14095359a <.text+0x95259a>
14095354b: 48 89 95 70 01 00 00        	movq	%rdx, 0x170(%rbp)
140953552: 0f b6 05 b0 16 ce 02        	movzbl	0x2ce16b0(%rip), %eax   # 0x143634c09
140953559: 41 bd 04 00 00 00           	movl	$0x4, %r13d
14095355f: ba 04 00 00 00              	movl	$0x4, %edx
140953564: 4c 89 d1                    	movq	%r10, %rcx
140953567: 4c 89 95 88 01 00 00        	movq	%r10, 0x188(%rbp)
14095356e: e8 9d 06 cc ff              	callq	0x140613c10 <.text+0x612c10>
140953573: 4c 8b 95 88 01 00 00        	movq	0x188(%rbp), %r10
14095357a: 48 85 c0                    	testq	%rax, %rax
14095357d: 0f 84 e1 0e 00 00           	je	0x140954464 <.text+0x953464>
140953583: 4c 8b ad 68 01 00 00        	movq	0x168(%rbp), %r13
14095358a: 4c 89 ad 78 01 00 00        	movq	%r13, 0x178(%rbp)
140953591: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
140953598: eb 17                       	jmp	0x1409535b1 <.text+0x9525b1>
14095359a: b8 04 00 00 00              	movl	$0x4, %eax
14095359f: 48 c7 85 78 01 00 00 00 00 00 00    	movq	$0x0, 0x178(%rbp)
1409535aa: 4c 8b ad 68 01 00 00        	movq	0x168(%rbp), %r13
1409535b1: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1409535b8: 48 89 c1                    	movq	%rax, %rcx
1409535bb: 4d 89 d0                    	movq	%r10, %r8
1409535be: e8 3d 06 fd 01              	callq	0x142923c00 <.text+0x2922c00>
1409535c3: e9 e6 00 00 00              	jmp	0x1409536ae <.text+0x9526ae>
1409535c8: b8 04 00 00 00              	movl	$0x4, %eax
1409535cd: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1409535d4: 48 c7 85 78 01 00 00 00 00 00 00    	movq	$0x0, 0x178(%rbp)
1409535df: 45 31 ed                    	xorl	%r13d, %r13d
1409535e2: e9 c7 00 00 00              	jmp	0x1409536ae <.text+0x9526ae>
1409535e7: 4c 89 c9                    	movq	%r9, %rcx
1409535ea: 48 b8 fe ff ff ff ff ff ff 0f       	movabsq	$0xffffffffffffffe, %rax # imm = 0xFFFFFFFFFFFFFFE
1409535f4: 48 21 c1                    	andq	%rax, %rcx
1409535f7: ba 14 00 00 00              	movl	$0x14, %edx
1409535fc: 31 c0                       	xorl	%eax, %eax
1409535fe: f3 41 0f 10 44 15 ec        	movss	-0x14(%r13,%rdx), %xmm0
140953605: f3 41 0f 10 4c 15 f4        	movss	-0xc(%r13,%rdx), %xmm1
14095360c: f3 41 0f 10 54 15 f0        	movss	-0x10(%r13,%rdx), %xmm2
140953613: f3 41 0f 58 d1              	addss	%xmm9, %xmm2
140953618: f3 41 0f 11 44 10 ec        	movss	%xmm0, -0x14(%r8,%rdx)
14095361f: f3 41 0f 11 54 10 f0        	movss	%xmm2, -0x10(%r8,%rdx)
140953626: f3 41 0f 11 4c 10 f4        	movss	%xmm1, -0xc(%r8,%rdx)
14095362d: f3 41 0f 10 44 15 f8        	movss	-0x8(%r13,%rdx), %xmm0
140953634: 66 41 0f 6e 4c 15 00        	movd	(%r13,%rdx), %xmm1
14095363b: f3 41 0f 10 54 15 fc        	movss	-0x4(%r13,%rdx), %xmm2
140953642: f3 41 0f 58 d1              	addss	%xmm9, %xmm2
140953647: f3 41 0f 11 44 10 f8        	movss	%xmm0, -0x8(%r8,%rdx)
14095364e: f3 41 0f 11 54 10 fc        	movss	%xmm2, -0x4(%r8,%rdx)
140953655: 66 41 0f 7e 0c 10           	movd	%xmm1, (%r8,%rdx)
14095365b: 48 83 c0 02                 	addq	$0x2, %rax
14095365f: 48 83 c2 18                 	addq	$0x18, %rdx
140953663: 48 39 c1                    	cmpq	%rax, %rcx
140953666: 75 96                       	jne	0x1409535fe <.text+0x9525fe>
140953668: 41 f6 c1 01                 	testb	$0x1, %r9b
14095366c: 4c 89 85 88 01 00 00        	movq	%r8, 0x188(%rbp)
140953673: 74 36                       	je	0x1409536ab <.text+0x9526ab>
140953675: 48 c1 e0 02                 	shlq	$0x2, %rax
140953679: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14095367d: f3 41 0f 10 44 05 00        	movss	(%r13,%rax), %xmm0
140953684: 66 41 0f 6e 4c 05 08        	movd	0x8(%r13,%rax), %xmm1
14095368b: f3 41 0f 10 54 05 04        	movss	0x4(%r13,%rax), %xmm2
140953692: f3 41 0f 58 d1              	addss	%xmm9, %xmm2
140953697: f3 41 0f 11 04 00           	movss	%xmm0, (%r8,%rax)
14095369d: f3 41 0f 11 54 00 04        	movss	%xmm2, 0x4(%r8,%rax)
1409536a4: 66 41 0f 7e 4c 00 08        	movd	%xmm1, 0x8(%r8,%rax)
1409536ab: 4d 89 cd                    	movq	%r9, %r13
1409536ae: 48 8d 85 08 01 00 00        	leaq	0x108(%rbp), %rax
1409536b5: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1409536bc: 48 8d 05 fd 17 1e 00        	leaq	0x1e17fd(%rip), %rax    # 0x140b34ec0 <.text+0xb33ec0>
1409536c3: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
1409536ca: 48 8d 05 ef 0f 16 02        	leaq	0x2160fef(%rip), %rax   # 0x142ab46c0
1409536d1: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
1409536d5: 48 c7 45 40 01 00 00 00     	movq	$0x1, 0x40(%rbp)
1409536dd: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
1409536e4: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1409536e8: 48 c7 45 50 01 00 00 00     	movq	$0x1, 0x50(%rbp)
1409536f0: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
1409536f8: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1409536fc: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
140953700: e8 8b 68 42 00              	callq	0x140d79f90 <.text+0xd78f90>
140953705: f3 0f 6f 45 b8              	movdqu	-0x48(%rbp), %xmm0
14095370a: 66 0f 7f 85 20 01 00 00     	movdqa	%xmm0, 0x120(%rbp)
140953712: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
140953716: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14095371d: 4c 89 ad 68 01 00 00        	movq	%r13, 0x168(%rbp)
140953724: 4c 89 e8                    	movq	%r13, %rax
140953727: b9 0c 00 00 00              	movl	$0xc, %ecx
14095372c: 48 f7 e1                    	mulq	%rcx
14095372f: 49 89 c1                    	movq	%rax, %r9
140953732: 0f 80 d7 0c 00 00           	jo	0x14095440f <.text+0x95340f>
140953738: 41 bd 00 00 00 00           	movl	$0x0, %r13d
14095373e: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140953748: 49 39 c1                    	cmpq	%rax, %r9
14095374b: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
140953752: 0f 87 ba 0c 00 00           	ja	0x140954412 <.text+0x953412>
140953758: 4d 85 c9                    	testq	%r9, %r9
14095375b: 74 48                       	je	0x1409537a5 <.text+0x9527a5>
14095375d: 0f b6 05 a5 14 ce 02        	movzbl	0x2ce14a5(%rip), %eax   # 0x143634c09
140953764: 41 bd 04 00 00 00           	movl	$0x4, %r13d
14095376a: ba 04 00 00 00              	movl	$0x4, %edx
14095376f: 4c 89 c9                    	movq	%r9, %rcx
140953772: 4c 89 8d 18 01 00 00        	movq	%r9, 0x118(%rbp)
140953779: e8 92 04 cc ff              	callq	0x140613c10 <.text+0x612c10>
14095377e: 4c 8b 8d 18 01 00 00        	movq	0x118(%rbp), %r9
140953785: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
14095378c: 48 85 c0                    	testq	%rax, %rax
14095378f: 0f 84 7d 0c 00 00           	je	0x140954412 <.text+0x953412>
140953795: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14095379c: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
1409537a3: eb 10                       	jmp	0x1409537b5 <.text+0x9527b5>
1409537a5: b8 04 00 00 00              	movl	$0x4, %eax
1409537aa: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
1409537b5: 48 89 c1                    	movq	%rax, %rcx
1409537b8: 4d 89 c8                    	movq	%r9, %r8
1409537bb: 49 89 c5                    	movq	%rax, %r13
1409537be: e8 3d 04 fd 01              	callq	0x142923c00 <.text+0x2922c00>
1409537c3: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
1409537cb: 48 c7 45 40 08 00 00 00     	movq	$0x8, 0x40(%rbp)
1409537d3: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
1409537db: c7 45 50 01 00 00 00        	movl	$0x1, 0x50(%rbp)
1409537e2: 48 c7 45 58 01 00 00 00     	movq	$0x1, 0x58(%rbp)
1409537ea: c7 45 68 02 00 00 00        	movl	$0x2, 0x68(%rbp)
1409537f1: c7 85 80 00 00 00 03 00 00 00       	movl	$0x3, 0x80(%rbp)
1409537fb: c7 85 98 00 00 00 03 00 00 00       	movl	$0x3, 0x98(%rbp)
140953805: c6 85 b0 00 00 00 00        	movb	$0x0, 0xb0(%rbp)
14095380c: c6 85 c0 00 00 00 01        	movb	$0x1, 0xc0(%rbp)
140953813: c7 85 c4 00 00 00 00 00 00 3f       	movl	$0x3f000000, 0xc4(%rbp) # imm = 0x3F000000
14095381d: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140953824: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14095382b: 4c 89 ad d0 00 00 00        	movq	%r13, 0xd0(%rbp)
140953832: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140953839: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140953840: 66 c7 85 e0 00 00 00 01 01  	movw	$0x101, 0xe0(%rbp)      # imm = 0x101
140953849: c6 85 96 01 00 00 00        	movb	$0x0, 0x196(%rbp)
140953850: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
140953857: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
14095385e: 4c 8d 45 38                 	leaq	0x38(%rbp), %r8
140953862: e8 89 f9 fc ff              	callq	0x1409231f0 <.text+0x9221f0>
140953867: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
14095386b: 74 36                       	je	0x1409538a3 <.text+0x9528a3>
14095386d: 48 39 50 38                 	cmpq	%rdx, 0x38(%rax)
140953871: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
140953878: 75 30                       	jne	0x1409538aa <.text+0x9528aa>
14095387a: 83 78 28 02                 	cmpl	$0x2, 0x28(%rax)
14095387e: 74 2a                       	je	0x1409538aa <.text+0x9528aa>
140953880: b8 01 00 00 00              	movl	$0x1, %eax
140953885: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
14095388c: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
140953893: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14095389a: 48 89 95 f8 00 00 00        	movq	%rdx, 0xf8(%rbp)
1409538a1: eb 07                       	jmp	0x1409538aa <.text+0x9528aa>
1409538a3: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1409538aa: 48 85 c9                    	testq	%rcx, %rcx
1409538ad: 0f 84 99 f9 ff ff           	je	0x14095324c <.text+0x95224c>
1409538b3: 48 c1 e1 02                 	shlq	$0x2, %rcx
1409538b7: 48 8d 14 49                 	leaq	(%rcx,%rcx,2), %rdx
1409538bb: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409538c1: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1409538c8: e8 53 03 cc ff              	callq	0x140613c20 <.text+0x612c20>
1409538cd: e9 7a f9 ff ff              	jmp	0x14095324c <.text+0x95224c>
1409538d2: 48 8d 0d 87 0d 16 02        	leaq	0x2160d87(%rip), %rcx   # 0x142ab4660
1409538d9: e8 32 5a f8 01              	callq	0x1428d9310 <.text+0x28d8310>
1409538de: e9 2a 0c 00 00              	jmp	0x14095450d <.text+0x95350d>
1409538e3: 48 85 c0                    	testq	%rax, %rax
1409538e6: 0f 85 00 01 00 00           	jne	0x1409539ec <.text+0x9529ec>
1409538ec: 31 d2                       	xorl	%edx, %edx
1409538ee: e8 dd 4e fa 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409538f3: 48 89 c1                    	movq	%rax, %rcx
1409538f6: 48 85 c0                    	testq	%rax, %rax
1409538f9: 0f 85 97 f8 ff ff           	jne	0x140953196 <.text+0x952196>
1409538ff: e9 e8 00 00 00              	jmp	0x1409539ec <.text+0x9529ec>
140953904: 48 83 bd 50 01 00 00 01     	cmpq	$0x1, 0x150(%rbp)
14095390c: 0f 85 28 0a 00 00           	jne	0x14095433a <.text+0x95333a>
140953912: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140953919: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140953920: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140953924: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
140953929: 0f 84 6f 01 00 00           	je	0x140953a9e <.text+0x952a9e>
14095392f: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
140953933: 48 89 f9                    	movq	%rdi, %rcx
140953936: e8 35 d4 c5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14095393b: 48 8b 33                    	movq	(%rbx), %rsi
14095393e: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140953942: 48 89 ca                    	movq	%rcx, %rdx
140953945: 48 21 c2                    	andq	%rax, %rdx
140953948: 48 c1 e8 39                 	shrq	$0x39, %rax
14095394c: 66 0f 6e c0                 	movd	%eax, %xmm0
140953950: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140953954: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140953959: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14095395e: 48 8d 86 58 fd ff ff        	leaq	-0x2a8(%rsi), %rax
140953965: 45 31 c0                    	xorl	%r8d, %r8d
140953968: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14095396c: 4c 8b 9d 60 01 00 00        	movq	0x160(%rbp), %r11
140953973: f3 0f 6f 14 16              	movdqu	(%rsi,%rdx), %xmm2
140953978: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14095397c: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140953980: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
140953985: 45 85 c9                    	testl	%r9d, %r9d
140953988: 74 28                       	je	0x1409539b2 <.text+0x9529b2>
14095398a: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
14095398f: 49 01 d2                    	addq	%rdx, %r10
140953992: 49 21 ca                    	andq	%rcx, %r10
140953995: 49 f7 da                    	negq	%r10
140953998: 4d 69 d2 a8 02 00 00        	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
14095399f: 4e 39 1c 10                 	cmpq	%r11, (%rax,%r10)
1409539a3: 74 53                       	je	0x1409539f8 <.text+0x9529f8>
1409539a5: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1409539a9: 66 45 21 ca                 	andw	%r9w, %r10w
1409539ad: 45 89 d1                    	movl	%r10d, %r9d
1409539b0: 75 d8                       	jne	0x14095398a <.text+0x95298a>
1409539b2: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1409539b6: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1409539bb: 45 85 c9                    	testl	%r9d, %r9d
1409539be: 0f 85 da 00 00 00           	jne	0x140953a9e <.text+0x952a9e>
1409539c4: 4c 01 c2                    	addq	%r8, %rdx
1409539c7: 48 83 c2 10                 	addq	$0x10, %rdx
1409539cb: 49 83 c0 10                 	addq	$0x10, %r8
1409539cf: 48 21 ca                    	andq	%rcx, %rdx
1409539d2: eb 9f                       	jmp	0x140953973 <.text+0x952973>
1409539d4: 48 85 c0                    	testq	%rax, %rax
1409539d7: 75 13                       	jne	0x1409539ec <.text+0x9529ec>
1409539d9: 31 d2                       	xorl	%edx, %edx
1409539db: e8 f0 4d fa 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409539e0: 48 89 c1                    	movq	%rax, %rcx
1409539e3: 48 85 c0                    	testq	%rax, %rax
1409539e6: 0f 85 a1 f5 ff ff           	jne	0x140952f8d <.text+0x951f8d>
1409539ec: 48 8d 0d ad 70 2e 02        	leaq	0x22e70ad(%rip), %rcx   # 0x142c3aaa0
1409539f3: e8 d8 46 f8 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409539f8: 4a 83 bc 16 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rsi,%r10)
140953a01: 0f 82 82 09 00 00           	jb	0x140954389 <.text+0x953389>
140953a07: f3 42 0f 10 84 16 90 fd ff ff       	movss	-0x270(%rsi,%r10), %xmm0
140953a11: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140953a15: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140953a18: 0f 86 6b 09 00 00           	jbe	0x140954389 <.text+0x953389>
140953a1e: 4c 01 d6                    	addq	%r10, %rsi
140953a21: 4c 8b be f8 fd ff ff        	movq	-0x208(%rsi), %r15
140953a28: 45 31 ed                    	xorl	%r13d, %r13d
140953a2b: b9 0c 00 00 00              	movl	$0xc, %ecx
140953a30: 4c 89 f8                    	movq	%r15, %rax
140953a33: 48 f7 e1                    	mulq	%rcx
140953a36: 49 89 c6                    	movq	%rax, %r14
140953a39: 0f 80 8b 09 00 00           	jo	0x1409543ca <.text+0x9533ca>
140953a3f: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140953a49: 49 39 c6                    	cmpq	%rax, %r14
140953a4c: 0f 87 78 09 00 00           	ja	0x1409543ca <.text+0x9533ca>
140953a52: 4c 8b a6 f0 fd ff ff        	movq	-0x210(%rsi), %r12
140953a59: 4d 85 f6                    	testq	%r14, %r14
140953a5c: 74 51                       	je	0x140953aaf <.text+0x952aaf>
140953a5e: 0f b6 05 a4 11 ce 02        	movzbl	0x2ce11a4(%rip), %eax   # 0x143634c09
140953a65: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140953a6b: ba 04 00 00 00              	movl	$0x4, %edx
140953a70: 4c 89 f1                    	movq	%r14, %rcx
140953a73: e8 98 01 cc ff              	callq	0x140613c10 <.text+0x612c10>
140953a78: 48 85 c0                    	testq	%rax, %rax
140953a7b: 0f 84 49 09 00 00           	je	0x1409543ca <.text+0x9533ca>
140953a81: 48 89 c1                    	movq	%rax, %rcx
140953a84: 4c 89 bd 88 01 00 00        	movq	%r15, 0x188(%rbp)
140953a8b: eb 32                       	jmp	0x140953abf <.text+0x952abf>
140953a8d: 48 8d 0d fc 0b 16 02        	leaq	0x2160bfc(%rip), %rcx   # 0x142ab4690
140953a94: e8 77 58 f8 01              	callq	0x1428d9310 <.text+0x28d8310>
140953a99: e9 6f 0a 00 00              	jmp	0x14095450d <.text+0x95350d>
140953a9e: 48 8d 0d 73 0a 16 02        	leaq	0x2160a73(%rip), %rcx   # 0x142ab4518
140953aa5: e8 66 58 f8 01              	callq	0x1428d9310 <.text+0x28d8310>
140953aaa: e9 5e 0a 00 00              	jmp	0x14095450d <.text+0x95350d>
140953aaf: b9 04 00 00 00              	movl	$0x4, %ecx
140953ab4: 48 c7 85 88 01 00 00 00 00 00 00    	movq	$0x0, 0x188(%rbp)
140953abf: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
140953ac6: 4c 89 e2                    	movq	%r12, %rdx
140953ac9: 4d 89 f0                    	movq	%r14, %r8
140953acc: e8 2f 01 fd 01              	callq	0x142923c00 <.text+0x2922c00>
140953ad1: 48 8b 86 10 fe ff ff        	movq	-0x1f0(%rsi), %rax
140953ad8: 4c 8d 24 85 00 00 00 00     	leaq	(,%rax,4), %r12
140953ae0: 45 31 ed                    	xorl	%r13d, %r13d
140953ae3: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
140953aea: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140953aee: 75 3a                       	jne	0x140953b2a <.text+0x952b2a>
140953af0: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140953afa: 49 39 c4                    	cmpq	%rax, %r12
140953afd: 77 2b                       	ja	0x140953b2a <.text+0x952b2a>
140953aff: 4c 8b b6 08 fe ff ff        	movq	-0x1f8(%rsi), %r14
140953b06: 4d 85 e4                    	testq	%r12, %r12
140953b09: 74 36                       	je	0x140953b41 <.text+0x952b41>
140953b0b: 0f b6 05 f7 10 ce 02        	movzbl	0x2ce10f7(%rip), %eax   # 0x143634c09
140953b12: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140953b18: ba 04 00 00 00              	movl	$0x4, %edx
140953b1d: 4c 89 e1                    	movq	%r12, %rcx
140953b20: e8 eb 00 cc ff              	callq	0x140613c10 <.text+0x612c10>
140953b25: 48 85 c0                    	testq	%rax, %rax
140953b28: 75 27                       	jne	0x140953b51 <.text+0x952b51>
140953b2a: 4c 8d 05 47 50 2e 02        	leaq	0x22e5047(%rip), %r8    # 0x142c38b78
140953b31: 4c 89 e9                    	movq	%r13, %rcx
140953b34: 4c 89 e2                    	movq	%r12, %rdx
140953b37: e8 a7 54 f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140953b3c: e9 cc 09 00 00              	jmp	0x14095450d <.text+0x95350d>
140953b41: b8 04 00 00 00              	movl	$0x4, %eax
140953b46: 48 c7 85 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rbp)
140953b51: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
140953b58: 48 89 c1                    	movq	%rax, %rcx
140953b5b: 4c 89 f2                    	movq	%r14, %rdx
140953b5e: 4d 89 e0                    	movq	%r12, %r8
140953b61: e8 9a 00 fd 01              	callq	0x142923c00 <.text+0x2922c00>
140953b66: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
140953b6d: 48 8b 00                    	movq	(%rax), %rax
140953b70: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140953b77: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
140953b7e: 4c 8b 28                    	movq	(%rax), %r13
140953b81: 49 83 c5 2c                 	addq	$0x2c, %r13
140953b85: 48 8d 85 10 01 00 00        	leaq	0x110(%rbp), %rax
140953b8c: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140953b93: 48 8d 05 26 13 1e 00        	leaq	0x1e1326(%rip), %rax    # 0x140b34ec0 <.text+0xb33ec0>
140953b9a: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140953ba1: 4c 89 ad 30 01 00 00        	movq	%r13, 0x130(%rbp)
140953ba8: 48 8d 05 b1 99 43 00        	leaq	0x4399b1(%rip), %rax    # 0x140d8d560 <.text+0xd8c560>
140953baf: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
140953bb6: 48 8d 05 8b 09 16 02        	leaq	0x216098b(%rip), %rax   # 0x142ab4548
140953bbd: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140953bc1: 48 c7 45 40 02 00 00 00     	movq	$0x2, 0x40(%rbp)
140953bc9: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
140953bd0: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140953bd4: 48 c7 45 50 02 00 00 00     	movq	$0x2, 0x50(%rbp)
140953bdc: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
140953be4: c6 85 97 01 00 00 00        	movb	$0x0, 0x197(%rbp)
140953beb: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
140953bef: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
140953bf3: e8 98 63 42 00              	callq	0x140d79f90 <.text+0xd78f90>
140953bf8: 4c 8b 65 08                 	movq	0x8(%rbp), %r12
140953bfc: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140953c00: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
140953c07: 4c 8b 75 18                 	movq	0x18(%rbp), %r14
140953c0b: 0f b6 05 f7 0f ce 02        	movzbl	0x2ce0ff7(%rip), %eax   # 0x143634c09
140953c12: b9 30 00 00 00              	movl	$0x30, %ecx
140953c17: ba 08 00 00 00              	movl	$0x8, %edx
140953c1c: e8 ef ff cb ff              	callq	0x140613c10 <.text+0x612c10>
140953c21: 48 85 c0                    	testq	%rax, %rax
140953c24: 4c 89 a5 60 01 00 00        	movq	%r12, 0x160(%rbp)
140953c2b: 0f 84 b0 07 00 00           	je	0x1409543e1 <.text+0x9533e1>
140953c31: 48 89 c1                    	movq	%rax, %rcx
140953c34: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140953c3b: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140953c43: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
140953c4a: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
140953c4e: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
140953c55: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140953c59: 4c 89 79 20                 	movq	%r15, 0x20(%rcx)
140953c5d: c7 41 28 00 00 40 40        	movl	$0x40400000, 0x28(%rcx) # imm = 0x40400000
140953c64: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
140953c68: 48 8d 35 f9 08 16 02        	leaq	0x21608f9(%rip), %rsi   # 0x142ab4568
140953c6f: 48 89 75 40                 	movq	%rsi, 0x40(%rbp)
140953c73: 4d 85 f6                    	testq	%r14, %r14
140953c76: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
140953c7d: 0f 88 72 07 00 00           	js	0x1409543f5 <.text+0x9533f5>
140953c83: 74 28                       	je	0x140953cad <.text+0x952cad>
140953c85: 0f b6 05 7d 0f ce 02        	movzbl	0x2ce0f7d(%rip), %eax   # 0x143634c09
140953c8c: 41 bc 01 00 00 00           	movl	$0x1, %r12d
140953c92: ba 01 00 00 00              	movl	$0x1, %edx
140953c97: 4c 89 f1                    	movq	%r14, %rcx
140953c9a: e8 71 ff cb ff              	callq	0x140613c10 <.text+0x612c10>
140953c9f: 48 85 c0                    	testq	%rax, %rax
140953ca2: 0f 84 50 07 00 00           	je	0x1409543f8 <.text+0x9533f8>
140953ca8: 49 89 c7                    	movq	%rax, %r15
140953cab: eb 06                       	jmp	0x140953cb3 <.text+0x952cb3>
140953cad: 41 bf 01 00 00 00           	movl	$0x1, %r15d
140953cb3: 4c 89 f9                    	movq	%r15, %rcx
140953cb6: 48 8b 95 80 01 00 00        	movq	0x180(%rbp), %rdx
140953cbd: 4d 89 f0                    	movq	%r14, %r8
140953cc0: e8 3b ff fc 01              	callq	0x142923c00 <.text+0x2922c00>
140953cc5: 4c 89 75 38                 	movq	%r14, 0x38(%rbp)
140953cc9: 4c 89 7d 40                 	movq	%r15, 0x40(%rbp)
140953ccd: 4c 89 75 48                 	movq	%r14, 0x48(%rbp)
140953cd1: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140953cd8: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140953cdc: 48 89 75 58                 	movq	%rsi, 0x58(%rbp)
140953ce0: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
140953ce4: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
140953ceb: e8 90 8b b1 00              	callq	0x14146c880 <.text+0x146b880>
140953cf0: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
140953cf7: 48 85 d2                    	testq	%rdx, %rdx
140953cfa: 48 8b b5 58 01 00 00        	movq	0x158(%rbp), %rsi
140953d01: 74 12                       	je	0x140953d15 <.text+0x952d15>
140953d03: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140953d09: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
140953d10: e8 0b ff cb ff              	callq	0x140613c20 <.text+0x612c20>
140953d15: 80 7e 18 00                 	cmpb	$0x0, 0x18(%rsi)
140953d19: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140953d20: 0f 84 f2 05 00 00           	je	0x140954318 <.text+0x953318>
140953d26: 48 39 46 38                 	cmpq	%rax, 0x38(%rsi)
140953d2a: 0f 85 e8 05 00 00           	jne	0x140954318 <.text+0x953318>
140953d30: 83 7e 28 02                 	cmpl	$0x2, 0x28(%rsi)
140953d34: 0f 84 de 05 00 00           	je	0x140954318 <.text+0x953318>
140953d3a: f3 0f 10 76 4c              	movss	0x4c(%rsi), %xmm6
140953d3f: f3 0f 10 7e 54              	movss	0x54(%rsi), %xmm7
140953d44: 4c 8b bd 10 01 00 00        	movq	0x110(%rbp), %r15
140953d4b: 4c 89 7d 38                 	movq	%r15, 0x38(%rbp)
140953d4f: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
140953d54: 0f 84 46 01 00 00           	je	0x140953ea0 <.text+0x952ea0>
140953d5a: c6 85 97 01 00 00 01        	movb	$0x1, 0x197(%rbp)
140953d61: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
140953d65: 48 89 f9                    	movq	%rdi, %rcx
140953d68: e8 03 d0 c5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140953d6d: 4c 8b 33                    	movq	(%rbx), %r14
140953d70: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140953d74: 48 89 ca                    	movq	%rcx, %rdx
140953d77: 48 21 c2                    	andq	%rax, %rdx
140953d7a: 48 c1 e8 39                 	shrq	$0x39, %rax
140953d7e: 66 0f 6e c0                 	movd	%eax, %xmm0
140953d82: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140953d86: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140953d8b: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140953d90: 49 8d 86 58 fd ff ff        	leaq	-0x2a8(%r14), %rax
140953d97: 45 31 c0                    	xorl	%r8d, %r8d
140953d9a: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140953d9e: f3 41 0f 6f 14 16           	movdqu	(%r14,%rdx), %xmm2
140953da4: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140953da8: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140953dac: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
140953db1: 45 85 c9                    	testl	%r9d, %r9d
140953db4: 74 28                       	je	0x140953dde <.text+0x952dde>
140953db6: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140953dbb: 49 01 d2                    	addq	%rdx, %r10
140953dbe: 49 21 ca                    	andq	%rcx, %r10
140953dc1: 49 f7 da                    	negq	%r10
140953dc4: 4d 69 d2 a8 02 00 00        	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
140953dcb: 4e 39 3c 10                 	cmpq	%r15, (%rax,%r10)
140953dcf: 74 2f                       	je	0x140953e00 <.text+0x952e00>
140953dd1: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140953dd5: 66 45 21 ca                 	andw	%r9w, %r10w
140953dd9: 45 89 d1                    	movl	%r10d, %r9d
140953ddc: 75 d8                       	jne	0x140953db6 <.text+0x952db6>
140953dde: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140953de2: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140953de7: 45 85 c9                    	testl	%r9d, %r9d
140953dea: 0f 85 b0 00 00 00           	jne	0x140953ea0 <.text+0x952ea0>
140953df0: 4c 01 c2                    	addq	%r8, %rdx
140953df3: 48 83 c2 10                 	addq	$0x10, %rdx
140953df7: 49 83 c0 10                 	addq	$0x10, %r8
140953dfb: 48 21 ca                    	andq	%rcx, %rdx
140953dfe: eb 9e                       	jmp	0x140953d9e <.text+0x952d9e>
140953e00: 4b 83 bc 16 70 fd ff ff 02  	cmpq	$0x2, -0x290(%r14,%r10)
140953e09: 0f 82 97 05 00 00           	jb	0x1409543a6 <.text+0x9533a6>
140953e0f: f3 43 0f 10 84 16 90 fd ff ff       	movss	-0x270(%r14,%r10), %xmm0
140953e19: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140953e1d: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140953e20: 0f 86 80 05 00 00           	jbe	0x1409543a6 <.text+0x9533a6>
140953e26: 4d 01 d6                    	addq	%r10, %r14
140953e29: 49 8b 86 f8 fd ff ff        	movq	-0x208(%r14), %rax
140953e30: 45 31 ff                    	xorl	%r15d, %r15d
140953e33: b9 0c 00 00 00              	movl	$0xc, %ecx
140953e38: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
140953e3f: 48 f7 e1                    	mulq	%rcx
140953e42: 48 89 c7                    	movq	%rax, %rdi
140953e45: 0f 80 30 06 00 00           	jo	0x14095447b <.text+0x95347b>
140953e4b: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140953e55: 48 39 c7                    	cmpq	%rax, %rdi
140953e58: 0f 87 1d 06 00 00           	ja	0x14095447b <.text+0x95347b>
140953e5e: 49 8b 9e f0 fd ff ff        	movq	-0x210(%r14), %rbx
140953e65: 48 85 ff                    	testq	%rdi, %rdi
140953e68: 74 5a                       	je	0x140953ec4 <.text+0x952ec4>
140953e6a: 0f b6 05 98 0d ce 02        	movzbl	0x2ce0d98(%rip), %eax   # 0x143634c09
140953e71: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140953e77: ba 04 00 00 00              	movl	$0x4, %edx
140953e7c: 48 89 f9                    	movq	%rdi, %rcx
140953e7f: e8 8c fd cb ff              	callq	0x140613c10 <.text+0x612c10>
140953e84: 48 85 c0                    	testq	%rax, %rax
140953e87: 0f 84 ee 05 00 00           	je	0x14095447b <.text+0x95347b>
140953e8d: 48 89 c1                    	movq	%rax, %rcx
140953e90: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
140953e97: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
140953e9e: eb 34                       	jmp	0x140953ed4 <.text+0x952ed4>
140953ea0: c6 85 97 01 00 00 01        	movb	$0x1, 0x197(%rbp)
140953ea7: 48 8d 0d 02 07 16 02        	leaq	0x2160702(%rip), %rcx   # 0x142ab45b0
140953eae: e8 5d 54 f8 01              	callq	0x1428d9310 <.text+0x28d8310>
140953eb3: e9 55 06 00 00              	jmp	0x14095450d <.text+0x95350d>
140953eb8: 48 8d 0d b1 6c 2e 02        	leaq	0x22e6cb1(%rip), %rcx   # 0x142c3ab70
140953ebf: e8 8c 53 f8 01              	callq	0x1428d9250 <.text+0x28d8250>
140953ec4: b9 04 00 00 00              	movl	$0x4, %ecx
140953ec9: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
140953ed4: 48 89 8d 60 01 00 00        	movq	%rcx, 0x160(%rbp)
140953edb: 48 89 da                    	movq	%rbx, %rdx
140953ede: 49 89 f8                    	movq	%rdi, %r8
140953ee1: e8 1a fd fc 01              	callq	0x142923c00 <.text+0x2922c00>
140953ee6: 49 8b b6 10 fe ff ff        	movq	-0x1f0(%r14), %rsi
140953eed: 4c 8d 3c b5 00 00 00 00     	leaq	(,%rsi,4), %r15
140953ef5: 31 ff                       	xorl	%edi, %edi
140953ef7: 48 89 f0                    	movq	%rsi, %rax
140953efa: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140953efe: 0f 85 92 05 00 00           	jne	0x140954496 <.text+0x953496>
140953f04: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140953f0e: 49 39 c7                    	cmpq	%rax, %r15
140953f11: 0f 87 7f 05 00 00           	ja	0x140954496 <.text+0x953496>
140953f17: 4d 8b a6 08 fe ff ff        	movq	-0x1f8(%r14), %r12
140953f1e: 4d 85 ff                    	testq	%r15, %r15
140953f21: 74 27                       	je	0x140953f4a <.text+0x952f4a>
140953f23: 0f b6 05 df 0c ce 02        	movzbl	0x2ce0cdf(%rip), %eax   # 0x143634c09
140953f2a: bf 04 00 00 00              	movl	$0x4, %edi
140953f2f: ba 04 00 00 00              	movl	$0x4, %edx
140953f34: 4c 89 f9                    	movq	%r15, %rcx
140953f37: e8 d4 fc cb ff              	callq	0x140613c10 <.text+0x612c10>
140953f3c: 48 85 c0                    	testq	%rax, %rax
140953f3f: 0f 84 51 05 00 00           	je	0x140954496 <.text+0x953496>
140953f45: 48 89 f3                    	movq	%rsi, %rbx
140953f48: eb 07                       	jmp	0x140953f51 <.text+0x952f51>
140953f4a: b8 04 00 00 00              	movl	$0x4, %eax
140953f4f: 31 db                       	xorl	%ebx, %ebx
140953f51: 48 89 c7                    	movq	%rax, %rdi
140953f54: 48 89 c1                    	movq	%rax, %rcx
140953f57: 4c 89 e2                    	movq	%r12, %rdx
140953f5a: 4d 89 f8                    	movq	%r15, %r8
140953f5d: e8 9e fc fc 01              	callq	0x142923c00 <.text+0x2922c00>
140953f62: f3 41 0f 10 86 18 fe ff ff  	movss	-0x1e8(%r14), %xmm0
140953f6b: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
140953f72: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140953f76: 4c 8b a5 60 01 00 00        	movq	0x160(%rbp), %r12
140953f7d: 4c 89 65 40                 	movq	%r12, 0x40(%rbp)
140953f81: 4c 8b b5 58 01 00 00        	movq	0x158(%rbp), %r14
140953f88: 4c 89 75 48                 	movq	%r14, 0x48(%rbp)
140953f8c: 48 89 9d 70 01 00 00        	movq	%rbx, 0x170(%rbp)
140953f93: 48 89 5d 50                 	movq	%rbx, 0x50(%rbp)
140953f97: 48 89 7d 30                 	movq	%rdi, 0x30(%rbp)
140953f9b: 48 89 7d 58                 	movq	%rdi, 0x58(%rbp)
140953f9f: 48 89 75 60                 	movq	%rsi, 0x60(%rbp)
140953fa3: f3 0f 11 45 68              	movss	%xmm0, 0x68(%rbp)
140953fa8: 4c 8d 0d 19 06 16 02        	leaq	0x2160619(%rip), %r9    # 0x142ab45c8
140953faf: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
140953fb3: 0f 28 ce                    	movaps	%xmm6, %xmm1
140953fb6: 0f 28 d7                    	movaps	%xmm7, %xmm2
140953fb9: e8 92 59 b1 00              	callq	0x141469950 <.text+0x1468950>
140953fbe: 0f 57 c9                    	xorps	%xmm1, %xmm1
140953fc1: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
140953fc5: f3 0f 10 3d 37 19 fd 01     	movss	0x1fd1937(%rip), %xmm7  # 0x142925904
140953fcd: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
140953fd1: f3 44 0f 5d c1              	minss	%xmm1, %xmm8
140953fd6: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
140953fda: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
140953fde: e8 fd 3c b1 00              	callq	0x141467ce0 <.text+0x1466ce0>
140953fe3: 89 c1                       	movl	%eax, %ecx
140953fe5: 49 39 ce                    	cmpq	%rcx, %r14
140953fe8: 0f 86 bc 04 00 00           	jbe	0x1409544aa <.text+0x9534aa>
140953fee: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
140953ff2: 4c 39 f0                    	cmpq	%r14, %rax
140953ff5: 0f 83 c0 04 00 00           	jae	0x1409544bb <.text+0x9534bb>
140953ffb: 0f 28 f0                    	movaps	%xmm0, %xmm6
140953ffe: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140954002: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140954006: 44 0f 28 df                 	movaps	%xmm7, %xmm11
14095400a: f3 44 0f 5c d8              	subss	%xmm0, %xmm11
14095400f: f3 45 0f 10 4c 8c 08        	movss	0x8(%r12,%rcx,4), %xmm9
140954016: f3 45 0f 59 cb              	mulss	%xmm11, %xmm9
14095401b: f2 45 0f 10 2c 8c           	movsd	(%r12,%rcx,4), %xmm13
140954021: f2 45 0f 10 24 84           	movsd	(%r12,%rax,4), %xmm12
140954027: f3 45 0f 10 54 84 08        	movss	0x8(%r12,%rax,4), %xmm10
14095402e: f3 44 0f 59 d0              	mulss	%xmm0, %xmm10
140954033: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
140954037: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14095403b: e8 a0 3c b1 00              	callq	0x141467ce0 <.text+0x1466ce0>
140954040: 89 c0                       	movl	%eax, %eax
140954042: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
140954046: 4c 39 f1                    	cmpq	%r14, %rcx
140954049: 0f 83 80 04 00 00           	jae	0x1409544cf <.text+0x9534cf>
14095404f: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140954053: f3 41 0f 10 4c 8c 08        	movss	0x8(%r12,%rcx,4), %xmm1
14095405a: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14095405e: f3 41 0f 5c 4c 84 08        	subss	0x8(%r12,%rax,4), %xmm1
140954065: f2 41 0f 10 04 8c           	movsd	(%r12,%rcx,4), %xmm0
14095406b: f2 41 0f 10 14 84           	movsd	(%r12,%rax,4), %xmm2
140954071: 0f 5c c2                    	subps	%xmm2, %xmm0
140954074: 0f 28 d0                    	movaps	%xmm0, %xmm2
140954077: 0f 59 d0                    	mulps	%xmm0, %xmm2
14095407a: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
14095407e: f3 0f 58 da                 	addss	%xmm2, %xmm3
140954082: 0f 28 d1                    	movaps	%xmm1, %xmm2
140954085: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
140954089: f3 0f 58 d3                 	addss	%xmm3, %xmm2
14095408d: f3 0f 51 d2                 	sqrtss	%xmm2, %xmm2
140954091: 0f 28 df                    	movaps	%xmm7, %xmm3
140954094: f3 0f 5e da                 	divss	%xmm2, %xmm3
140954098: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
14095409c: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
1409540a0: 0f 28 0d 49 18 fd 01        	movaps	0x1fd1849(%rip), %xmm1  # 0x1429258f0
1409540a7: 0f 57 cb                    	xorps	%xmm3, %xmm1
1409540aa: f3 0f 59 db                 	mulss	%xmm3, %xmm3
1409540ae: 0f 28 d0                    	movaps	%xmm0, %xmm2
1409540b1: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
1409540b5: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1409540b9: f3 0f 51 d2                 	sqrtss	%xmm2, %xmm2
1409540bd: f3 0f 5e fa                 	divss	%xmm2, %xmm7
1409540c1: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
1409540c5: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
1409540c9: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1409540d0: 0f 28 d7                    	movaps	%xmm7, %xmm2
1409540d3: e8 f8 ce 33 00              	callq	0x140c90fd0 <.text+0xc8ffd0>
1409540d8: 0f 28 bd 20 01 00 00        	movaps	0x120(%rbp), %xmm7
1409540df: 48 8d 85 10 01 00 00        	leaq	0x110(%rbp), %rax
1409540e6: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1409540ea: 48 8d 05 cf 0d 1e 00        	leaq	0x1e0dcf(%rip), %rax    # 0x140b34ec0 <.text+0xb33ec0>
1409540f1: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1409540f5: 4c 89 6d e0                 	movq	%r13, -0x20(%rbp)
1409540f9: 48 8d 05 60 94 43 00        	leaq	0x439460(%rip), %rax    # 0x140d8d560 <.text+0xd8c560>
140954100: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140954104: 48 8d 05 ed 04 16 02        	leaq	0x21604ed(%rip), %rax   # 0x142ab45f8
14095410b: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140954112: 48 c7 85 28 01 00 00 02 00 00 00    	movq	$0x2, 0x128(%rbp)
14095411d: 48 8d 45 d0                 	leaq	-0x30(%rbp), %rax
140954121: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
140954128: 48 c7 85 38 01 00 00 02 00 00 00    	movq	$0x2, 0x138(%rbp)
140954133: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
14095413e: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140954142: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
140954149: e8 42 5e 42 00              	callq	0x140d79f90 <.text+0xd78f90>
14095414e: 48 8b 7d f0                 	movq	-0x10(%rbp), %rdi
140954152: 48 8b 5d f8                 	movq	-0x8(%rbp), %rbx
140954156: 4c 8b 75 00                 	movq	(%rbp), %r14
14095415a: 0f b6 05 a8 0a ce 02        	movzbl	0x2ce0aa8(%rip), %eax   # 0x143634c09
140954161: b9 30 00 00 00              	movl	$0x30, %ecx
140954166: ba 10 00 00 00              	movl	$0x10, %edx
14095416b: e8 a0 fa cb ff              	callq	0x140613c10 <.text+0x612c10>
140954170: 48 85 c0                    	testq	%rax, %rax
140954173: 48 89 bd 58 01 00 00        	movq	%rdi, 0x158(%rbp)
14095417a: 48 89 9d 00 01 00 00        	movq	%rbx, 0x100(%rbp)
140954181: 0f 84 59 03 00 00           	je	0x1409544e0 <.text+0x9534e0>
140954187: 49 89 c5                    	movq	%rax, %r13
14095418a: f3 41 0f 12 c3              	movsldup	%xmm11, %xmm0           # xmm0 = xmm11[0,0,2,2]
14095418f: 41 0f 59 c5                 	mulps	%xmm13, %xmm0
140954193: f3 0f 12 ce                 	movsldup	%xmm6, %xmm1            # xmm1 = xmm6[0,0,2,2]
140954197: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
14095419b: 0f 58 c8                    	addps	%xmm0, %xmm1
14095419e: f3 45 0f 58 ca              	addss	%xmm10, %xmm9
1409541a3: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
1409541aa: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
1409541b2: 0f 29 78 10                 	movaps	%xmm7, 0x10(%rax)
1409541b6: 0f 13 48 20                 	movlps	%xmm1, 0x20(%rax)
1409541ba: f3 44 0f 11 48 28           	movss	%xmm9, 0x28(%rax)
1409541c0: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1409541c7: 48 8d 35 4a 04 16 02        	leaq	0x216044a(%rip), %rsi   # 0x142ab4618
1409541ce: 48 89 b5 28 01 00 00        	movq	%rsi, 0x128(%rbp)
1409541d5: 4d 85 f6                    	testq	%r14, %r14
1409541d8: 0f 88 13 03 00 00           	js	0x1409544f1 <.text+0x9534f1>
1409541de: 74 2f                       	je	0x14095420f <.text+0x95320f>
1409541e0: 0f b6 05 22 0a ce 02        	movzbl	0x2ce0a22(%rip), %eax   # 0x143634c09
1409541e7: 41 bc 01 00 00 00           	movl	$0x1, %r12d
1409541ed: ba 01 00 00 00              	movl	$0x1, %edx
1409541f2: 4c 89 f1                    	movq	%r14, %rcx
1409541f5: e8 16 fa cb ff              	callq	0x140613c10 <.text+0x612c10>
1409541fa: 48 85 c0                    	testq	%rax, %rax
1409541fd: 0f 84 f1 02 00 00           	je	0x1409544f4 <.text+0x9534f4>
140954203: 49 89 c7                    	movq	%rax, %r15
140954206: 4c 8b a5 60 01 00 00        	movq	0x160(%rbp), %r12
14095420d: eb 06                       	jmp	0x140954215 <.text+0x953215>
14095420f: 41 bf 01 00 00 00           	movl	$0x1, %r15d
140954215: 4c 89 f9                    	movq	%r15, %rcx
140954218: 48 89 da                    	movq	%rbx, %rdx
14095421b: 4d 89 f0                    	movq	%r14, %r8
14095421e: e8 dd f9 fc 01              	callq	0x142923c00 <.text+0x2922c00>
140954223: 4c 89 b5 20 01 00 00        	movq	%r14, 0x120(%rbp)
14095422a: 4c 89 bd 28 01 00 00        	movq	%r15, 0x128(%rbp)
140954231: 4c 89 b5 30 01 00 00        	movq	%r14, 0x130(%rbp)
140954238: 4c 89 ad 38 01 00 00        	movq	%r13, 0x138(%rbp)
14095423f: 48 89 b5 40 01 00 00        	movq	%rsi, 0x140(%rbp)
140954246: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
14095424d: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
140954254: e8 27 86 b1 00              	callq	0x14146c880 <.text+0x146b880>
140954259: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
140954260: 48 85 d2                    	testq	%rdx, %rdx
140954263: 74 12                       	je	0x140954277 <.text+0x953277>
140954265: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14095426b: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
140954272: e8 a9 f9 cb ff              	callq	0x140613c20 <.text+0x612c20>
140954277: 48 8b 9d 10 01 00 00        	movq	0x110(%rbp), %rbx
14095427e: 48 8b 95 98 02 00 00        	movq	0x298(%rbp), %rdx
140954285: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140954288: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14095428c: 89 01                       	movl	%eax, (%rcx)
14095428e: 48 8b 32                    	movq	(%rdx), %rsi
140954291: 4c 8b 76 30                 	movq	0x30(%rsi), %r14
140954295: 4c 8b 7e 40                 	movq	0x40(%rsi), %r15
140954299: 4c 3b 76 20                 	cmpq	0x20(%rsi), %r14
14095429d: 75 10                       	jne	0x1409542af <.text+0x9532af>
14095429f: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1409542a3: 48 8d 15 9e 5e 2e 02        	leaq	0x22e5e9e(%rip), %rdx   # 0x142c3a148
1409542aa: e8 01 eb c0 ff              	callq	0x140562db0 <.text+0x561db0>
1409542af: 48 8b 46 28                 	movq	0x28(%rsi), %rax
1409542b3: 4b 8d 0c 76                 	leaq	(%r14,%r14,2), %rcx
1409542b7: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1409542be: 48 89 14 c8                 	movq	%rdx, (%rax,%rcx,8)
1409542c2: 48 89 5c c8 08              	movq	%rbx, 0x8(%rax,%rcx,8)
1409542c7: 4c 89 7c c8 10              	movq	%r15, 0x10(%rax,%rcx,8)
1409542cc: 49 ff c6                    	incq	%r14
1409542cf: 4c 89 76 30                 	movq	%r14, 0x30(%rsi)
1409542d3: 48 ff 46 40                 	incq	0x40(%rsi)
1409542d7: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1409542de: 48 85 c0                    	testq	%rax, %rax
1409542e1: 74 16                       	je	0x1409542f9 <.text+0x9532f9>
1409542e3: 48 c1 e0 02                 	shlq	$0x2, %rax
1409542e7: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409542eb: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409542f1: 4c 89 e1                    	movq	%r12, %rcx
1409542f4: e8 27 f9 cb ff              	callq	0x140613c20 <.text+0x612c20>
1409542f9: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
140954300: 48 85 d2                    	testq	%rdx, %rdx
140954303: 74 13                       	je	0x140954318 <.text+0x953318>
140954305: 48 c1 e2 02                 	shlq	$0x2, %rdx
140954309: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14095430f: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140954313: e8 08 f9 cb ff              	callq	0x140613c20 <.text+0x612c20>
140954318: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
14095431f: 48 85 d2                    	testq	%rdx, %rdx
140954322: 74 16                       	je	0x14095433a <.text+0x95333a>
140954324: 48 c1 e2 02                 	shlq	$0x2, %rdx
140954328: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14095432e: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
140954335: e8 e6 f8 cb ff              	callq	0x140613c20 <.text+0x612c20>
14095433a: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
140954341: 48 85 c0                    	testq	%rax, %rax
140954344: 0f 85 2a ec ff ff           	jne	0x140952f74 <.text+0x951f74>
14095434a: e9 70 ec ff ff              	jmp	0x140952fbf <.text+0x951fbf>
14095434f: 48 8d 0d 3d 5e 18 02        	leaq	0x2185e3d(%rip), %rcx   # 0x142ada193
140954356: 4c 8d 05 eb 02 16 02        	leaq	0x21602eb(%rip), %r8    # 0x142ab4648
14095435d: ba 32 00 00 00              	movl	$0x32, %edx
140954362: e8 c9 50 f8 01              	callq	0x1428d9430 <.text+0x28d8430>
140954367: e9 a1 01 00 00              	jmp	0x14095450d <.text+0x95350d>
14095436c: 48 8d 0d 20 5e 18 02        	leaq	0x2185e20(%rip), %rcx   # 0x142ada193
140954373: 4c 8d 05 fe 02 16 02        	leaq	0x21602fe(%rip), %r8    # 0x142ab4678
14095437a: ba 32 00 00 00              	movl	$0x32, %edx
14095437f: e8 ac 50 f8 01              	callq	0x1428d9430 <.text+0x28d8430>
140954384: e9 84 01 00 00              	jmp	0x14095450d <.text+0x95350d>
140954389: 48 8d 0d 03 5e 18 02        	leaq	0x2185e03(%rip), %rcx   # 0x142ada193
140954390: 4c 8d 05 69 01 16 02        	leaq	0x2160169(%rip), %r8    # 0x142ab4500
140954397: ba 32 00 00 00              	movl	$0x32, %edx
14095439c: e8 8f 50 f8 01              	callq	0x1428d9430 <.text+0x28d8430>
1409543a1: e9 67 01 00 00              	jmp	0x14095450d <.text+0x95350d>
1409543a6: c6 85 97 01 00 00 01        	movb	$0x1, 0x197(%rbp)
1409543ad: 48 8d 0d df 5d 18 02        	leaq	0x2185ddf(%rip), %rcx   # 0x142ada193
1409543b4: 4c 8d 05 dd 01 16 02        	leaq	0x21601dd(%rip), %r8    # 0x142ab4598
1409543bb: ba 32 00 00 00              	movl	$0x32, %edx
1409543c0: e8 6b 50 f8 01              	callq	0x1428d9430 <.text+0x28d8430>
1409543c5: e9 43 01 00 00              	jmp	0x14095450d <.text+0x95350d>
1409543ca: 4c 8d 05 a7 47 2e 02        	leaq	0x22e47a7(%rip), %r8    # 0x142c38b78
1409543d1: 4c 89 e9                    	movq	%r13, %rcx
1409543d4: 4c 89 f2                    	movq	%r14, %rdx
1409543d7: e8 07 4c f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409543dc: e9 2c 01 00 00              	jmp	0x14095450d <.text+0x95350d>
1409543e1: b9 08 00 00 00              	movl	$0x8, %ecx
1409543e6: ba 30 00 00 00              	movl	$0x30, %edx
1409543eb: e8 10 4c f8 01              	callq	0x1428d9000 <.text+0x28d8000>
1409543f0: e9 18 01 00 00              	jmp	0x14095450d <.text+0x95350d>
1409543f5: 45 31 e4                    	xorl	%r12d, %r12d
1409543f8: 4c 8d 05 21 4b 14 02        	leaq	0x2144b21(%rip), %r8    # 0x142a98f20
1409543ff: 4c 89 e1                    	movq	%r12, %rcx
140954402: 4c 89 f2                    	movq	%r14, %rdx
140954405: e8 d9 4b f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14095440a: e9 fe 00 00 00              	jmp	0x14095450d <.text+0x95350d>
14095440f: 45 31 ed                    	xorl	%r13d, %r13d
140954412: c6 85 96 01 00 00 01        	movb	$0x1, 0x196(%rbp)
140954419: 4c 8d 05 58 47 2e 02        	leaq	0x22e4758(%rip), %r8    # 0x142c38b78
140954420: 4c 89 e9                    	movq	%r13, %rcx
140954423: 4c 89 ca                    	movq	%r9, %rdx
140954426: e8 b8 4b f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14095442b: e9 dd 00 00 00              	jmp	0x14095450d <.text+0x95350d>
140954430: 48 c7 85 68 01 00 00 00 00 00 00    	movq	$0x0, 0x168(%rbp)
14095443b: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
140954442: 4c 8d 05 cf f8 15 02        	leaq	0x215f8cf(%rip), %r8    # 0x142ab3d18
140954449: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
140954450: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
140954457: e8 87 4b f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14095445c: e9 ac 00 00 00              	jmp	0x14095450d <.text+0x95350d>
140954461: 45 31 ed                    	xorl	%r13d, %r13d
140954464: 4c 8d 05 0d 47 2e 02        	leaq	0x22e470d(%rip), %r8    # 0x142c38b78
14095446b: 4c 89 e9                    	movq	%r13, %rcx
14095446e: 4c 89 d2                    	movq	%r10, %rdx
140954471: e8 6d 4b f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140954476: e9 92 00 00 00              	jmp	0x14095450d <.text+0x95350d>
14095447b: c6 85 97 01 00 00 01        	movb	$0x1, 0x197(%rbp)
140954482: 4c 8d 05 ef 46 2e 02        	leaq	0x22e46ef(%rip), %r8    # 0x142c38b78
140954489: 4c 89 f9                    	movq	%r15, %rcx
14095448c: 48 89 fa                    	movq	%rdi, %rdx
14095448f: e8 4f 4b f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140954494: eb 77                       	jmp	0x14095450d <.text+0x95350d>
140954496: 4c 8d 05 db 46 2e 02        	leaq	0x22e46db(%rip), %r8    # 0x142c38b78
14095449d: 48 89 f9                    	movq	%rdi, %rcx
1409544a0: 4c 89 fa                    	movq	%r15, %rdx
1409544a3: e8 3b 4b f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409544a8: eb 63                       	jmp	0x14095450d <.text+0x95350d>
1409544aa: 4c 8d 05 a7 f1 30 02        	leaq	0x230f1a7(%rip), %r8    # 0x142c63658
1409544b1: 4c 89 f2                    	movq	%r14, %rdx
1409544b4: e8 5f 50 f8 01              	callq	0x1428d9518 <.text+0x28d8518>
1409544b9: eb 52                       	jmp	0x14095450d <.text+0x95350d>
1409544bb: 4c 8d 05 ae f1 30 02        	leaq	0x230f1ae(%rip), %r8    # 0x142c63670
1409544c2: 48 89 c1                    	movq	%rax, %rcx
1409544c5: 4c 89 f2                    	movq	%r14, %rdx
1409544c8: e8 4b 50 f8 01              	callq	0x1428d9518 <.text+0x28d8518>
1409544cd: eb 3e                       	jmp	0x14095450d <.text+0x95350d>
1409544cf: 4c 8d 05 b2 f1 30 02        	leaq	0x230f1b2(%rip), %r8    # 0x142c63688
1409544d6: 4c 89 f2                    	movq	%r14, %rdx
1409544d9: e8 3a 50 f8 01              	callq	0x1428d9518 <.text+0x28d8518>
1409544de: eb 2d                       	jmp	0x14095450d <.text+0x95350d>
1409544e0: b9 10 00 00 00              	movl	$0x10, %ecx
1409544e5: ba 30 00 00 00              	movl	$0x30, %edx
1409544ea: e8 11 4b f8 01              	callq	0x1428d9000 <.text+0x28d8000>
1409544ef: eb 1c                       	jmp	0x14095450d <.text+0x95350d>
1409544f1: 45 31 e4                    	xorl	%r12d, %r12d
1409544f4: 4c 89 ad 80 01 00 00        	movq	%r13, 0x180(%rbp)
1409544fb: 4c 8d 05 1e 4a 14 02        	leaq	0x2144a1e(%rip), %r8    # 0x142a98f20
140954502: 4c 89 e1                    	movq	%r12, %rcx
140954505: 4c 89 f2                    	movq	%r14, %rdx
140954508: e8 d6 4a f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14095450d: 0f 0b                       	ud2
14095450f: 90                          	nop
140954510: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954515: 55                          	pushq	%rbp
140954516: 41 57                       	pushq	%r15
140954518: 41 56                       	pushq	%r14
14095451a: 41 55                       	pushq	%r13
14095451c: 41 54                       	pushq	%r12
14095451e: 56                          	pushq	%rsi
14095451f: 57                          	pushq	%rdi
140954520: 53                          	pushq	%rbx
140954521: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954528: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095452f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954535: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14095453b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954541: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954547: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
14095454d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954556: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14095455e: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140954567: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
14095456e: 48 ff 00                    	incq	(%rax)
140954571: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140954579: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954581: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14095458a: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954590: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954596: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14095459c: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1409545a2: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1409545a8: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1409545af: 5b                          	popq	%rbx
1409545b0: 5f                          	popq	%rdi
1409545b1: 5e                          	popq	%rsi
1409545b2: 41 5c                       	popq	%r12
1409545b4: 41 5d                       	popq	%r13
1409545b6: 41 5e                       	popq	%r14
1409545b8: 41 5f                       	popq	%r15
1409545ba: 5d                          	popq	%rbp
1409545bb: c3                          	retq
1409545bc: 0f 1f 40 00                 	nopl	(%rax)
1409545c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409545c5: 55                          	pushq	%rbp
1409545c6: 41 57                       	pushq	%r15
1409545c8: 41 56                       	pushq	%r14
1409545ca: 41 55                       	pushq	%r13
1409545cc: 41 54                       	pushq	%r12
1409545ce: 56                          	pushq	%rsi
1409545cf: 57                          	pushq	%rdi
1409545d0: 53                          	pushq	%rbx
1409545d1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1409545d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409545df: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
1409545e5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
1409545eb: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
1409545f1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
1409545f7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
1409545fd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954606: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14095460e: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140954617: 48 83 bd e8 00 00 00 00     	cmpq	$0x0, 0xe8(%rbp)
14095461f: 74 23                       	je	0x140954644 <.text+0x953644>
140954621: 48 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %rax
140954628: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14095462c: 48 8d 45 28                 	leaq	0x28(%rbp), %rax
140954630: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140954634: 48 8d 0d 6d f8 15 02        	leaq	0x215f86d(%rip), %rcx   # 0x142ab3ea8
14095463b: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
14095463f: e8 8c ac 9d 00              	callq	0x14132f2d0 <.text+0x132e2d0>
140954644: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
14095464c: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954654: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14095465d: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954663: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954669: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14095466f: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140954675: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
14095467b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140954682: 5b                          	popq	%rbx
140954683: 5f                          	popq	%rdi
140954684: 5e                          	popq	%rsi
140954685: 41 5c                       	popq	%r12
140954687: 41 5d                       	popq	%r13
140954689: 41 5e                       	popq	%r14
14095468b: 41 5f                       	popq	%r15
14095468d: 5d                          	popq	%rbp
14095468e: c3                          	retq
14095468f: 90                          	nop
140954690: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954695: 55                          	pushq	%rbp
140954696: 41 57                       	pushq	%r15
140954698: 41 56                       	pushq	%r14
14095469a: 41 55                       	pushq	%r13
14095469c: 41 54                       	pushq	%r12
14095469e: 56                          	pushq	%rsi
14095469f: 57                          	pushq	%rdi
1409546a0: 53                          	pushq	%rbx
1409546a1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1409546a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409546af: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
1409546b5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
1409546bb: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
1409546c1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
1409546c7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
1409546cd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
1409546d6: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
1409546de: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
1409546e7: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1409546ee: 48 ff 00                    	incq	(%rax)
1409546f1: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
1409546f9: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954701: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14095470a: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954710: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954716: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14095471c: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140954722: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140954728: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14095472f: 5b                          	popq	%rbx
140954730: 5f                          	popq	%rdi
140954731: 5e                          	popq	%rsi
140954732: 41 5c                       	popq	%r12
140954734: 41 5d                       	popq	%r13
140954736: 41 5e                       	popq	%r14
140954738: 41 5f                       	popq	%r15
14095473a: 5d                          	popq	%rbp
14095473b: c3                          	retq
14095473c: 0f 1f 40 00                 	nopl	(%rax)
140954740: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954745: 55                          	pushq	%rbp
140954746: 41 57                       	pushq	%r15
140954748: 41 56                       	pushq	%r14
14095474a: 41 55                       	pushq	%r13
14095474c: 41 54                       	pushq	%r12
14095474e: 56                          	pushq	%rsi
14095474f: 57                          	pushq	%rdi
140954750: 53                          	pushq	%rbx
140954751: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954758: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095475f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954765: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14095476b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954771: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954777: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
14095477d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954786: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14095478e: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140954797: 48 83 bd 88 01 00 00 00     	cmpq	$0x0, 0x188(%rbp)
14095479f: 74 21                       	je	0x1409547c2 <.text+0x9537c2>
1409547a1: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
1409547a8: 48 c1 e0 02                 	shlq	$0x2, %rax
1409547ac: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409547b0: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409547b6: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1409547bd: e8 5e f4 cb ff              	callq	0x140613c20 <.text+0x612c20>
1409547c2: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
1409547ca: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
1409547d2: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1409547db: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1409547e1: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1409547e7: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1409547ed: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1409547f3: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1409547f9: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140954800: 5b                          	popq	%rbx
140954801: 5f                          	popq	%rdi
140954802: 5e                          	popq	%rsi
140954803: 41 5c                       	popq	%r12
140954805: 41 5d                       	popq	%r13
140954807: 41 5e                       	popq	%r14
140954809: 41 5f                       	popq	%r15
14095480b: 5d                          	popq	%rbp
14095480c: c3                          	retq
14095480d: 0f 1f 00                    	nopl	(%rax)
140954810: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954815: 55                          	pushq	%rbp
140954816: 41 57                       	pushq	%r15
140954818: 41 56                       	pushq	%r14
14095481a: 41 55                       	pushq	%r13
14095481c: 41 54                       	pushq	%r12
14095481e: 56                          	pushq	%rsi
14095481f: 57                          	pushq	%rdi
140954820: 53                          	pushq	%rbx
140954821: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954828: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095482f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954835: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14095483b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954841: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954847: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
14095484d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954856: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14095485e: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140954867: 48 83 bd 88 01 00 00 00     	cmpq	$0x0, 0x188(%rbp)
14095486f: 74 2a                       	je	0x14095489b <.text+0x95389b>
140954871: f6 85 97 01 00 00 01        	testb	$0x1, 0x197(%rbp)
140954878: 75 21                       	jne	0x14095489b <.text+0x95389b>
14095487a: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
140954881: 48 c1 e0 02                 	shlq	$0x2, %rax
140954885: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140954889: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14095488f: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
140954896: e8 85 f3 cb ff              	callq	0x140613c20 <.text+0x612c20>
14095489b: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
1409548a2: 48 85 d2                    	testq	%rdx, %rdx
1409548a5: 74 16                       	je	0x1409548bd <.text+0x9538bd>
1409548a7: 48 c1 e2 02                 	shlq	$0x2, %rdx
1409548ab: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409548b1: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
1409548b8: e8 63 f3 cb ff              	callq	0x140613c20 <.text+0x612c20>
1409548bd: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
1409548c5: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
1409548cd: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1409548d6: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1409548dc: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1409548e2: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1409548e8: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1409548ee: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1409548f4: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1409548fb: 5b                          	popq	%rbx
1409548fc: 5f                          	popq	%rdi
1409548fd: 5e                          	popq	%rsi
1409548fe: 41 5c                       	popq	%r12
140954900: 41 5d                       	popq	%r13
140954902: 41 5e                       	popq	%r14
140954904: 41 5f                       	popq	%r15
140954906: 5d                          	popq	%rbp
140954907: c3                          	retq
140954908: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140954910: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954915: 55                          	pushq	%rbp
140954916: 41 57                       	pushq	%r15
140954918: 41 56                       	pushq	%r14
14095491a: 41 55                       	pushq	%r13
14095491c: 41 54                       	pushq	%r12
14095491e: 56                          	pushq	%rsi
14095491f: 57                          	pushq	%rdi
140954920: 53                          	pushq	%rbx
140954921: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954928: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095492f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954935: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14095493b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954941: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954947: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
14095494d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954956: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14095495e: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140954967: 48 83 bd 60 01 00 00 00     	cmpq	$0x0, 0x160(%rbp)
14095496f: 74 19                       	je	0x14095498a <.text+0x95398a>
140954971: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140954977: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
14095497e: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
140954985: e8 96 f2 cb ff              	callq	0x140613c20 <.text+0x612c20>
14095498a: c6 85 97 01 00 00 01        	movb	$0x1, 0x197(%rbp)
140954991: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
14095499a: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
1409549a2: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1409549ab: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1409549b1: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1409549b7: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1409549bd: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1409549c3: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1409549c9: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1409549d0: 5b                          	popq	%rbx
1409549d1: 5f                          	popq	%rdi
1409549d2: 5e                          	popq	%rsi
1409549d3: 41 5c                       	popq	%r12
1409549d5: 41 5d                       	popq	%r13
1409549d7: 41 5e                       	popq	%r14
1409549d9: 41 5f                       	popq	%r15
1409549db: 5d                          	popq	%rbp
1409549dc: c3                          	retq
1409549dd: 0f 1f 00                    	nopl	(%rax)
1409549e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409549e5: 55                          	pushq	%rbp
1409549e6: 41 57                       	pushq	%r15
1409549e8: 41 56                       	pushq	%r14
1409549ea: 41 55                       	pushq	%r13
1409549ec: 41 54                       	pushq	%r12
1409549ee: 56                          	pushq	%rsi
1409549ef: 57                          	pushq	%rdi
1409549f0: 53                          	pushq	%rbx
1409549f1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1409549f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409549ff: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954a05: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140954a0b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954a11: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954a17: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140954a1d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954a26: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140954a2e: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140954a37: 48 83 bd 88 01 00 00 00     	cmpq	$0x0, 0x188(%rbp)
140954a3f: 74 25                       	je	0x140954a66 <.text+0x953a66>
140954a41: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
140954a48: 48 8d 04 85 00 00 00 00     	leaq	(,%rax,4), %rax
140954a50: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140954a54: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140954a5a: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
140954a61: e8 ba f1 cb ff              	callq	0x140613c20 <.text+0x612c20>
140954a66: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140954a6f: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954a77: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140954a80: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954a86: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954a8c: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140954a92: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140954a98: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140954a9e: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140954aa5: 5b                          	popq	%rbx
140954aa6: 5f                          	popq	%rdi
140954aa7: 5e                          	popq	%rsi
140954aa8: 41 5c                       	popq	%r12
140954aaa: 41 5d                       	popq	%r13
140954aac: 41 5e                       	popq	%r14
140954aae: 41 5f                       	popq	%r15
140954ab0: 5d                          	popq	%rbp
140954ab1: c3                          	retq
140954ab2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140954ac0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954ac5: 55                          	pushq	%rbp
140954ac6: 41 57                       	pushq	%r15
140954ac8: 41 56                       	pushq	%r14
140954aca: 41 55                       	pushq	%r13
140954acc: 41 54                       	pushq	%r12
140954ace: 56                          	pushq	%rsi
140954acf: 57                          	pushq	%rdi
140954ad0: 53                          	pushq	%rbx
140954ad1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954ad8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140954adf: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954ae5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140954aeb: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954af1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954af7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140954afd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954b06: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140954b0e: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140954b17: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140954b1e: f0                          	lock
140954b1f: 48 ff 08                    	decq	(%rax)
140954b22: 75 09                       	jne	0x140954b2d <.text+0x953b2d>
140954b24: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
140954b28: e8 a3 ae cf ff              	callq	0x14064f9d0 <.text+0x64e9d0>
140954b2d: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140954b36: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954b3e: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140954b47: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954b4d: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954b53: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140954b59: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140954b5f: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140954b65: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140954b6c: 5b                          	popq	%rbx
140954b6d: 5f                          	popq	%rdi
140954b6e: 5e                          	popq	%rsi
140954b6f: 41 5c                       	popq	%r12
140954b71: 41 5d                       	popq	%r13
140954b73: 41 5e                       	popq	%r14
140954b75: 41 5f                       	popq	%r15
140954b77: 5d                          	popq	%rbp
140954b78: c3                          	retq
140954b79: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140954b80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954b85: 55                          	pushq	%rbp
140954b86: 41 57                       	pushq	%r15
140954b88: 41 56                       	pushq	%r14
140954b8a: 41 55                       	pushq	%r13
140954b8c: 41 54                       	pushq	%r12
140954b8e: 56                          	pushq	%rsi
140954b8f: 57                          	pushq	%rdi
140954b90: 53                          	pushq	%rbx
140954b91: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954b98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140954b9f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954ba5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140954bab: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954bb1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954bb7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140954bbd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954bc6: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140954bce: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140954bd6: 48 83 bd 80 01 00 00 00     	cmpq	$0x0, 0x180(%rbp)
140954bde: 74 21                       	je	0x140954c01 <.text+0x953c01>
140954be0: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
140954be7: 48 c1 e0 02                 	shlq	$0x2, %rax
140954beb: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140954bef: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140954bf5: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
140954bfc: e8 1f f0 cb ff              	callq	0x140613c20 <.text+0x612c20>
140954c01: c6 85 97 01 00 00 01        	movb	$0x1, 0x197(%rbp)
140954c08: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140954c11: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954c19: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140954c22: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954c28: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954c2e: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140954c34: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140954c3a: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140954c40: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140954c47: 5b                          	popq	%rbx
140954c48: 5f                          	popq	%rdi
140954c49: 5e                          	popq	%rsi
140954c4a: 41 5c                       	popq	%r12
140954c4c: 41 5d                       	popq	%r13
140954c4e: 41 5e                       	popq	%r14
140954c50: 41 5f                       	popq	%r15
140954c52: 5d                          	popq	%rbp
140954c53: c3                          	retq
140954c54: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140954c60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954c65: 55                          	pushq	%rbp
140954c66: 41 57                       	pushq	%r15
140954c68: 41 56                       	pushq	%r14
140954c6a: 41 55                       	pushq	%r13
140954c6c: 41 54                       	pushq	%r12
140954c6e: 56                          	pushq	%rsi
140954c6f: 57                          	pushq	%rdi
140954c70: 53                          	pushq	%rbx
140954c71: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954c78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140954c7f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954c85: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140954c8b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954c91: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954c97: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140954c9d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954ca6: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140954cae: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140954cb6: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
140954cba: e8 61 6d 7f ff              	callq	0x14014ba20 <.text+0x14aa20>
140954cbf: c6 85 97 01 00 00 01        	movb	$0x1, 0x197(%rbp)
140954cc6: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140954ccf: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954cd7: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140954ce0: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954ce6: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954cec: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140954cf2: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140954cf8: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140954cfe: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140954d05: 5b                          	popq	%rbx
140954d06: 5f                          	popq	%rdi
140954d07: 5e                          	popq	%rsi
140954d08: 41 5c                       	popq	%r12
140954d0a: 41 5d                       	popq	%r13
140954d0c: 41 5e                       	popq	%r14
140954d0e: 41 5f                       	popq	%r15
140954d10: 5d                          	popq	%rbp
140954d11: c3                          	retq
140954d12: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140954d20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954d25: 55                          	pushq	%rbp
140954d26: 41 57                       	pushq	%r15
140954d28: 41 56                       	pushq	%r14
140954d2a: 41 55                       	pushq	%r13
140954d2c: 41 54                       	pushq	%r12
140954d2e: 56                          	pushq	%rsi
140954d2f: 57                          	pushq	%rdi
140954d30: 53                          	pushq	%rbx
140954d31: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954d38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140954d3f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954d45: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140954d4b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954d51: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954d57: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140954d5d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954d66: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140954d6e: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140954d76: 48 83 bd 58 01 00 00 00     	cmpq	$0x0, 0x158(%rbp)
140954d7e: 74 19                       	je	0x140954d99 <.text+0x953d99>
140954d80: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140954d86: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
140954d8d: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
140954d94: e8 87 ee cb ff              	callq	0x140613c20 <.text+0x612c20>
140954d99: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140954da1: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954da9: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140954db2: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954db8: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954dbe: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140954dc4: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140954dca: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140954dd0: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140954dd7: 5b                          	popq	%rbx
140954dd8: 5f                          	popq	%rdi
140954dd9: 5e                          	popq	%rsi
140954dda: 41 5c                       	popq	%r12
140954ddc: 41 5d                       	popq	%r13
140954dde: 41 5e                       	popq	%r14
140954de0: 41 5f                       	popq	%r15
140954de2: 5d                          	popq	%rbp
140954de3: c3                          	retq
140954de4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140954df0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954df5: 55                          	pushq	%rbp
140954df6: 41 57                       	pushq	%r15
140954df8: 41 56                       	pushq	%r14
140954dfa: 41 55                       	pushq	%r13
140954dfc: 41 54                       	pushq	%r12
140954dfe: 56                          	pushq	%rsi
140954dff: 57                          	pushq	%rdi
140954e00: 53                          	pushq	%rbx
140954e01: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954e08: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140954e0f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954e15: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140954e1b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954e21: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954e27: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140954e2d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954e36: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140954e3e: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140954e46: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
140954e4d: f0                          	lock
140954e4e: 48 ff 08                    	decq	(%rax)
140954e51: 75 0c                       	jne	0x140954e5f <.text+0x953e5f>
140954e53: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
140954e5a: e8 71 ab cf ff              	callq	0x14064f9d0 <.text+0x64e9d0>
140954e5f: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140954e67: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954e6f: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140954e78: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954e7e: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954e84: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140954e8a: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140954e90: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140954e96: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140954e9d: 5b                          	popq	%rbx
140954e9e: 5f                          	popq	%rdi
140954e9f: 5e                          	popq	%rsi
140954ea0: 41 5c                       	popq	%r12
140954ea2: 41 5d                       	popq	%r13
140954ea4: 41 5e                       	popq	%r14
140954ea6: 41 5f                       	popq	%r15
140954ea8: 5d                          	popq	%rbp
140954ea9: c3                          	retq
140954eaa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140954eb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954eb5: 55                          	pushq	%rbp
140954eb6: 41 57                       	pushq	%r15
140954eb8: 41 56                       	pushq	%r14
140954eba: 41 55                       	pushq	%r13
140954ebc: 41 54                       	pushq	%r12
140954ebe: 56                          	pushq	%rsi
140954ebf: 57                          	pushq	%rdi
140954ec0: 53                          	pushq	%rbx
140954ec1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954ec8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140954ecf: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954ed5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140954edb: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954ee1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954ee7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140954eed: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954ef6: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140954efe: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140954f07: 48 83 bd 78 01 00 00 00     	cmpq	$0x0, 0x178(%rbp)
140954f0f: 74 21                       	je	0x140954f32 <.text+0x953f32>
140954f11: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
140954f18: 48 c1 e0 02                 	shlq	$0x2, %rax
140954f1c: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140954f20: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140954f26: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
140954f2d: e8 ee ec cb ff              	callq	0x140613c20 <.text+0x612c20>
140954f32: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140954f3a: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140954f42: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140954f4b: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140954f51: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140954f57: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140954f5d: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140954f63: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140954f69: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140954f70: 5b                          	popq	%rbx
140954f71: 5f                          	popq	%rdi
140954f72: 5e                          	popq	%rsi
140954f73: 41 5c                       	popq	%r12
140954f75: 41 5d                       	popq	%r13
140954f77: 41 5e                       	popq	%r14
140954f79: 41 5f                       	popq	%r15
140954f7b: 5d                          	popq	%rbp
140954f7c: c3                          	retq
140954f7d: 0f 1f 00                    	nopl	(%rax)
140954f80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140954f85: 55                          	pushq	%rbp
140954f86: 41 57                       	pushq	%r15
140954f88: 41 56                       	pushq	%r14
140954f8a: 41 55                       	pushq	%r13
140954f8c: 41 54                       	pushq	%r12
140954f8e: 56                          	pushq	%rsi
140954f8f: 57                          	pushq	%rdi
140954f90: 53                          	pushq	%rbx
140954f91: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140954f98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140954f9f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140954fa5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140954fab: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140954fb1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140954fb7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140954fbd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140954fc6: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140954fce: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140954fd7: 80 bd 96 01 00 00 00        	cmpb	$0x0, 0x196(%rbp)
140954fde: 74 6a                       	je	0x14095504a <.text+0x95404a>
140954fe0: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
140954fe7: 48 85 d2                    	testq	%rdx, %rdx
140954fea: 74 12                       	je	0x140954ffe <.text+0x953ffe>
140954fec: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
140954ff3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140954ff9: e8 22 ec cb ff              	callq	0x140613c20 <.text+0x612c20>
140954ffe: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140955007: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14095500f: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140955018: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14095501e: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140955024: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14095502a: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140955030: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140955036: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14095503d: 5b                          	popq	%rbx
14095503e: 5f                          	popq	%rdi
14095503f: 5e                          	popq	%rsi
140955040: 41 5c                       	popq	%r12
140955042: 41 5d                       	popq	%r13
140955044: 41 5e                       	popq	%r14
140955046: 41 5f                       	popq	%r15
140955048: 5d                          	popq	%rbp
140955049: c3                          	retq
14095504a: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140955053: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14095505b: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140955064: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14095506a: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140955070: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140955076: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14095507c: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140955082: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140955089: 5b                          	popq	%rbx
14095508a: 5f                          	popq	%rdi
14095508b: 5e                          	popq	%rsi
14095508c: 41 5c                       	popq	%r12
14095508e: 41 5d                       	popq	%r13
140955090: 41 5e                       	popq	%r14
140955092: 41 5f                       	popq	%r15
140955094: 5d                          	popq	%rbp
140955095: c3                          	retq
