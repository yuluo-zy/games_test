
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14230edb0: 55                          	pushq	%rbp
14230edb1: 41 57                       	pushq	%r15
14230edb3: 41 56                       	pushq	%r14
14230edb5: 41 55                       	pushq	%r13
14230edb7: 41 54                       	pushq	%r12
14230edb9: 56                          	pushq	%rsi
14230edba: 57                          	pushq	%rdi
14230edbb: 53                          	pushq	%rbx
14230edbc: 48 83 ec 78                 	subq	$0x78, %rsp
14230edc0: 48 8d 6c 24 70              	leaq	0x70(%rsp), %rbp
14230edc5: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14230edcd: 4c 89 c7                    	movq	%r8, %rdi
14230edd0: 49 89 d7                    	movq	%rdx, %r15
14230edd3: 48 89 cb                    	movq	%rcx, %rbx
14230edd6: e8 25 7c 90 fe              	callq	0x140c16a00 <.text+0xc15a00>
14230eddb: 41 89 c5                    	movl	%eax, %r13d
14230edde: 84 c0                       	testb	%al, %al
14230ede0: 74 5d                       	je	0x14230ee3f <.text+0x230de3f>
14230ede2: 41 be 75 00 00 00           	movl	$0x75, %r14d
14230ede8: b9 75 00 00 00              	movl	$0x75, %ecx
14230eded: e8 1e 08 00 00              	callq	0x14230f610 <.text+0x230e610>
14230edf2: 48 8d 35 77 b8 d3 00        	leaq	0xd3b877(%rip), %rsi    # 0x14304a670
14230edf9: 48 83 f8 01                 	cmpq	$0x1, %rax
14230edfd: 0f 85 a8 00 00 00           	jne	0x14230eeab <.text+0x230deab>
14230ee03: 49 89 d1                    	movq	%rdx, %r9
14230ee06: 48 85 d2                    	testq	%rdx, %rdx
14230ee09: 74 3d                       	je	0x14230ee48 <.text+0x230de48>
14230ee0b: 49 83 f9 75                 	cmpq	$0x75, %r9
14230ee0f: 73 35                       	jae	0x14230ee46 <.text+0x230de46>
14230ee11: 48 8d 05 58 b8 d3 00        	leaq	0xd3b858(%rip), %rax    # 0x14304a670
14230ee18: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14230ee1d: 7f 29                       	jg	0x14230ee48 <.text+0x230de48>
14230ee1f: 48 8d 05 42 b9 d3 00        	leaq	0xd3b942(%rip), %rax    # 0x14304a768
14230ee26: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14230ee2b: 48 8d 0d 3e b8 d3 00        	leaq	0xd3b83e(%rip), %rcx    # 0x14304a670
14230ee32: ba 75 00 00 00              	movl	$0x75, %edx
14230ee37: 45 31 c0                    	xorl	%r8d, %r8d
14230ee3a: e8 01 ab 5c 00              	callq	0x1428d9940 <.text+0x28d8940>
14230ee3f: 31 c0                       	xorl	%eax, %eax
14230ee41: e9 d6 01 00 00              	jmp	0x14230f01c <.text+0x230e01c>
14230ee46: 75 d7                       	jne	0x14230ee1f <.text+0x230de1f>
14230ee48: 4c 89 c9                    	movq	%r9, %rcx
14230ee4b: e8 c0 07 00 00              	callq	0x14230f610 <.text+0x230e610>
14230ee50: 48 83 f8 01                 	cmpq	$0x1, %rax
14230ee54: 75 55                       	jne	0x14230eeab <.text+0x230deab>
14230ee56: 49 89 d0                    	movq	%rdx, %r8
14230ee59: 49 83 c0 02                 	addq	$0x2, %r8
14230ee5d: 74 39                       	je	0x14230ee98 <.text+0x230de98>
14230ee5f: 49 83 f8 75                 	cmpq	$0x75, %r8
14230ee63: 73 31                       	jae	0x14230ee96 <.text+0x230de96>
14230ee65: 48 8d 05 04 b8 d3 00        	leaq	0xd3b804(%rip), %rax    # 0x14304a670
14230ee6c: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14230ee71: 7f 25                       	jg	0x14230ee98 <.text+0x230de98>
14230ee73: 48 8d 05 06 b9 d3 00        	leaq	0xd3b906(%rip), %rax    # 0x14304a780
14230ee7a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14230ee7f: 48 8d 0d ea b7 d3 00        	leaq	0xd3b7ea(%rip), %rcx    # 0x14304a670
14230ee86: ba 75 00 00 00              	movl	$0x75, %edx
14230ee8b: 41 b9 75 00 00 00           	movl	$0x75, %r9d
14230ee91: e8 aa aa 5c 00              	callq	0x1428d9940 <.text+0x28d8940>
14230ee96: 75 db                       	jne	0x14230ee73 <.text+0x230de73>
14230ee98: 41 be 73 00 00 00           	movl	$0x73, %r14d
14230ee9e: 49 29 d6                    	subq	%rdx, %r14
14230eea1: 48 8d 35 c8 b7 d3 00        	leaq	0xd3b7c8(%rip), %rsi    # 0x14304a670
14230eea8: 4c 01 c6                    	addq	%r8, %rsi
14230eeab: 4c 8d 05 f2 b6 d3 00        	leaq	0xd3b6f2(%rip), %r8     # 0x14304a5a4
14230eeb2: 4c 8d 25 c5 b6 d3 00        	leaq	0xd3b6c5(%rip), %r12    # 0x14304a57e
14230eeb9: 48 89 5d f8                 	movq	%rbx, -0x8(%rbp)
14230eebd: 0f 1f 00                    	nopl	(%rax)
14230eec0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14230eec5: 85 c0                       	testl	%eax, %eax
14230eec7: 78 17                       	js	0x14230eee0 <.text+0x230dee0>
14230eec9: 49 ff c8                    	decq	%r8
14230eecc: 83 f8 5c                    	cmpl	$0x5c, %eax
14230eecf: 75 63                       	jne	0x14230ef34 <.text+0x230df34>
14230eed1: eb 7c                       	jmp	0x14230ef4f <.text+0x230df4f>
14230eed3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14230eee0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14230eee5: 80 f9 c0                    	cmpb	$-0x40, %cl
14230eee8: 7d 23                       	jge	0x14230ef0d <.text+0x230df0d>
14230eeea: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14230eeef: 80 fa c0                    	cmpb	$-0x40, %dl
14230eef2: 7d 22                       	jge	0x14230ef16 <.text+0x230df16>
14230eef4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14230eef9: 49 83 c0 fc                 	addq	$-0x4, %r8
14230eefd: 41 83 e1 07                 	andl	$0x7, %r9d
14230ef01: 41 c1 e1 06                 	shll	$0x6, %r9d
14230ef05: 83 e2 3f                    	andl	$0x3f, %edx
14230ef08: 44 09 ca                    	orl	%r9d, %edx
14230ef0b: eb 10                       	jmp	0x14230ef1d <.text+0x230df1d>
14230ef0d: 49 83 c0 fe                 	addq	$-0x2, %r8
14230ef11: 83 e1 1f                    	andl	$0x1f, %ecx
14230ef14: eb 0f                       	jmp	0x14230ef25 <.text+0x230df25>
14230ef16: 49 83 c0 fd                 	addq	$-0x3, %r8
14230ef1a: 83 e2 0f                    	andl	$0xf, %edx
14230ef1d: c1 e2 06                    	shll	$0x6, %edx
14230ef20: 83 e1 3f                    	andl	$0x3f, %ecx
14230ef23: 09 d1                       	orl	%edx, %ecx
14230ef25: c1 e1 06                    	shll	$0x6, %ecx
14230ef28: 24 3f                       	andb	$0x3f, %al
14230ef2a: 0f b6 c0                    	movzbl	%al, %eax
14230ef2d: 09 c8                       	orl	%ecx, %eax
14230ef2f: 83 f8 5c                    	cmpl	$0x5c, %eax
14230ef32: 74 1b                       	je	0x14230ef4f <.text+0x230df4f>
14230ef34: 83 f8 2f                    	cmpl	$0x2f, %eax
14230ef37: 74 16                       	je	0x14230ef4f <.text+0x230df4f>
14230ef39: 4d 39 e0                    	cmpq	%r12, %r8
14230ef3c: 75 82                       	jne	0x14230eec0 <.text+0x230dec0>
14230ef3e: 48 89 fb                    	movq	%rdi, %rbx
14230ef41: 4c 89 ff                    	movq	%r15, %rdi
14230ef44: 45 89 ef                    	movl	%r13d, %r15d
14230ef47: 41 bd 26 00 00 00           	movl	$0x26, %r13d
14230ef4d: eb 56                       	jmp	0x14230efa5 <.text+0x230dfa5>
14230ef4f: 4c 8d 25 28 b6 d3 00        	leaq	0xd3b628(%rip), %r12    # 0x14304a57e
14230ef56: 4d 29 e0                    	subq	%r12, %r8
14230ef59: 49 ff c0                    	incq	%r8
14230ef5c: 74 32                       	je	0x14230ef90 <.text+0x230df90>
14230ef5e: 49 83 f8 26                 	cmpq	$0x26, %r8
14230ef62: 73 2a                       	jae	0x14230ef8e <.text+0x230df8e>
14230ef64: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
14230ef69: 7f 25                       	jg	0x14230ef90 <.text+0x230df90>
14230ef6b: 48 8d 05 d6 b7 d3 00        	leaq	0xd3b7d6(%rip), %rax    # 0x14304a748
14230ef72: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14230ef77: 48 8d 0d 00 b6 d3 00        	leaq	0xd3b600(%rip), %rcx    # 0x14304a57e
14230ef7e: ba 26 00 00 00              	movl	$0x26, %edx
14230ef83: 41 b9 26 00 00 00           	movl	$0x26, %r9d
14230ef89: e8 b2 a9 5c 00              	callq	0x1428d9940 <.text+0x28d8940>
14230ef8e: 75 db                       	jne	0x14230ef6b <.text+0x230df6b>
14230ef90: 48 89 fb                    	movq	%rdi, %rbx
14230ef93: 4c 89 ff                    	movq	%r15, %rdi
14230ef96: 45 89 ef                    	movl	%r13d, %r15d
14230ef99: 41 bd 26 00 00 00           	movl	$0x26, %r13d
14230ef9f: 4d 29 c5                    	subq	%r8, %r13
14230efa2: 4d 01 c4                    	addq	%r8, %r12
14230efa5: e8 86 91 90 fe              	callq	0x140c18130 <.text+0xc17130>
14230efaa: 48 89 c1                    	movq	%rax, %rcx
14230efad: 48 8b 00                    	movq	(%rax), %rax
14230efb0: 48 83 f8 01                 	cmpq	$0x1, %rax
14230efb4: 0f 85 d9 00 00 00           	jne	0x14230f093 <.text+0x230e093>
14230efba: 48 83 c1 08                 	addq	$0x8, %rcx
14230efbe: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14230efc2: 0f 85 20 01 00 00           	jne	0x14230f0e8 <.text+0x230e0e8>
14230efc8: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14230efcf: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
14230efd3: 48 83 c1 08                 	addq	$0x8, %rcx
14230efd7: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
14230efdc: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14230efe5: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14230efee: 48 89 f2                    	movq	%rsi, %rdx
14230eff1: 4d 89 f0                    	movq	%r14, %r8
14230eff4: 4d 89 e1                    	movq	%r12, %r9
14230eff7: e8 44 7d 90 fe              	callq	0x140c16d40 <.text+0xc15d40>
14230effc: 49 89 c6                    	movq	%rax, %r14
14230efff: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14230f003: 48 ff 00                    	incq	(%rax)
14230f006: 4c 89 75 d8                 	movq	%r14, -0x28(%rbp)
14230f00a: b8 01 00 00 00              	movl	$0x1, %eax
14230f00f: 45 89 fd                    	movl	%r15d, %r13d
14230f012: 49 89 ff                    	movq	%rdi, %r15
14230f015: 48 89 df                    	movq	%rbx, %rdi
14230f018: 48 8b 5d f8                 	movq	-0x8(%rbp), %rbx
14230f01c: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
14230f020: 4c 8b 43 08                 	movq	0x8(%rbx), %r8
14230f024: 4c 39 c7                    	cmpq	%r8, %rdi
14230f027: 4c 0f 42 c7                 	cmovbq	%rdi, %r8
14230f02b: 0f 87 9c 00 00 00           	ja	0x14230f0cd <.text+0x230e0cd>
14230f031: 48 8b 0b                    	movq	(%rbx), %rcx
14230f034: 49 c1 e0 05                 	shlq	$0x5, %r8
14230f038: 4c 89 fa                    	movq	%r15, %rdx
14230f03b: e8 c0 4b 61 00              	callq	0x142923c00 <.text+0x2922c00>
14230f040: 45 84 ed                    	testb	%r13b, %r13b
14230f043: 74 3d                       	je	0x14230f082 <.text+0x230e082>
14230f045: e8 e6 90 90 fe              	callq	0x140c18130 <.text+0xc17130>
14230f04a: 48 89 c1                    	movq	%rax, %rcx
14230f04d: 48 8b 00                    	movq	(%rax), %rax
14230f050: 48 83 f8 01                 	cmpq	$0x1, %rax
14230f054: 75 57                       	jne	0x14230f0ad <.text+0x230e0ad>
14230f056: 48 83 c1 08                 	addq	$0x8, %rcx
14230f05a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14230f05e: 0f 85 84 00 00 00           	jne	0x14230f0e8 <.text+0x230e0e8>
14230f064: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14230f06b: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
14230f06f: 48 83 c1 08                 	addq	$0x8, %rcx
14230f073: 4c 89 f2                    	movq	%r14, %rdx
14230f076: e8 75 80 90 fe              	callq	0x140c170f0 <.text+0xc160f0>
14230f07b: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14230f07f: 48 ff 00                    	incq	(%rax)
14230f082: 48 83 c4 78                 	addq	$0x78, %rsp
14230f086: 5b                          	popq	%rbx
14230f087: 5f                          	popq	%rdi
14230f088: 5e                          	popq	%rsi
14230f089: 41 5c                       	popq	%r12
14230f08b: 41 5d                       	popq	%r13
14230f08d: 41 5e                       	popq	%r14
14230f08f: 41 5f                       	popq	%r15
14230f091: 5d                          	popq	%rbp
14230f092: c3                          	retq
14230f093: 48 85 c0                    	testq	%rax, %rax
14230f096: 75 29                       	jne	0x14230f0c1 <.text+0x230e0c1>
14230f098: 31 d2                       	xorl	%edx, %edx
14230f09a: e8 91 bf 60 00              	callq	0x14291b030 <.text+0x291a030>
14230f09f: 48 89 c1                    	movq	%rax, %rcx
14230f0a2: 48 85 c0                    	testq	%rax, %rax
14230f0a5: 0f 85 13 ff ff ff           	jne	0x14230efbe <.text+0x230dfbe>
14230f0ab: eb 14                       	jmp	0x14230f0c1 <.text+0x230e0c1>
14230f0ad: 48 85 c0                    	testq	%rax, %rax
14230f0b0: 75 0f                       	jne	0x14230f0c1 <.text+0x230e0c1>
14230f0b2: 31 d2                       	xorl	%edx, %edx
14230f0b4: e8 77 bf 60 00              	callq	0x14291b030 <.text+0x291a030>
14230f0b9: 48 89 c1                    	movq	%rax, %rcx
14230f0bc: 48 85 c0                    	testq	%rax, %rax
14230f0bf: 75 99                       	jne	0x14230f05a <.text+0x230e05a>
14230f0c1: 48 8d 0d 08 b0 d3 00        	leaq	0xd3b008(%rip), %rcx    # 0x14304a0d0
14230f0c8: e8 03 90 5c 00              	callq	0x1428d80d0 <.text+0x28d70d0>
14230f0cd: 44 88 6d f8                 	movb	%r13b, -0x8(%rbp)
14230f0d1: 48 8d 05 d0 b4 d3 00        	leaq	0xd3b4d0(%rip), %rax    # 0x14304a5a8
14230f0d8: 4c 89 c1                    	movq	%r8, %rcx
14230f0db: 48 89 fa                    	movq	%rdi, %rdx
14230f0de: 49 89 c0                    	movq	%rax, %r8
14230f0e1: e8 fa a7 5c 00              	callq	0x1428d98e0 <.text+0x28d88e0>
14230f0e6: 0f 0b                       	ud2
14230f0e8: 48 8d 0d 51 b0 d3 00        	leaq	0xd3b051(%rip), %rcx    # 0x14304a140
14230f0ef: e8 5c a1 5c 00              	callq	0x1428d9250 <.text+0x28d8250>
14230f0f4: cc                          	int3
14230f0f5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14230f100: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14230f105: 55                          	pushq	%rbp
14230f106: 41 57                       	pushq	%r15
14230f108: 41 56                       	pushq	%r14
14230f10a: 41 55                       	pushq	%r13
14230f10c: 41 54                       	pushq	%r12
14230f10e: 56                          	pushq	%rsi
14230f10f: 57                          	pushq	%rdi
14230f110: 53                          	pushq	%rbx
14230f111: 48 83 ec 38                 	subq	$0x38, %rsp
14230f115: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14230f119: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14230f11d: 48 ff 00                    	incq	(%rax)
14230f120: 48 83 c4 38                 	addq	$0x38, %rsp
14230f124: 5b                          	popq	%rbx
14230f125: 5f                          	popq	%rdi
14230f126: 5e                          	popq	%rsi
14230f127: 41 5c                       	popq	%r12
14230f129: 41 5d                       	popq	%r13
14230f12b: 41 5e                       	popq	%r14
14230f12d: 41 5f                       	popq	%r15
14230f12f: 5d                          	popq	%rbp
14230f130: c3                          	retq
14230f131: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14230f140: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14230f145: 55                          	pushq	%rbp
14230f146: 41 57                       	pushq	%r15
14230f148: 41 56                       	pushq	%r14
14230f14a: 41 55                       	pushq	%r13
14230f14c: 41 54                       	pushq	%r12
14230f14e: 56                          	pushq	%rsi
14230f14f: 57                          	pushq	%rdi
14230f150: 53                          	pushq	%rbx
14230f151: 48 83 ec 38                 	subq	$0x38, %rsp
14230f155: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14230f159: 80 7d f8 00                 	cmpb	$0x0, -0x8(%rbp)
14230f15d: 74 20                       	je	0x14230f17f <.text+0x230e17f>
14230f15f: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
14230f163: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14230f167: 48 8d 45 e8                 	leaq	-0x18(%rbp), %rax
14230f16b: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14230f16f: 48 8d 0d 72 b5 d3 00        	leaq	0xd3b572(%rip), %rcx    # 0x14304a6e8
14230f176: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
14230f17a: e8 71 f4 ff ff              	callq	0x14230e5f0 <.text+0x230d5f0>
14230f17f: 90                          	nop
14230f180: 48 83 c4 38                 	addq	$0x38, %rsp
14230f184: 5b                          	popq	%rbx
14230f185: 5f                          	popq	%rdi
14230f186: 5e                          	popq	%rsi
14230f187: 41 5c                       	popq	%r12
14230f189: 41 5d                       	popq	%r13
14230f18b: 41 5e                       	popq	%r14
14230f18d: 41 5f                       	popq	%r15
14230f18f: 5d                          	popq	%rbp
14230f190: c3                          	retq
14230f191: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14230f1a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14230f1a5: 55                          	pushq	%rbp
14230f1a6: 41 57                       	pushq	%r15
14230f1a8: 41 56                       	pushq	%r14
14230f1aa: 41 55                       	pushq	%r13
14230f1ac: 41 54                       	pushq	%r12
14230f1ae: 56                          	pushq	%rsi
14230f1af: 57                          	pushq	%rdi
14230f1b0: 53                          	pushq	%rbx
14230f1b1: 48 83 ec 38                 	subq	$0x38, %rsp
14230f1b5: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14230f1b9: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14230f1bd: 48 ff 00                    	incq	(%rax)
14230f1c0: 48 83 c4 38                 	addq	$0x38, %rsp
14230f1c4: 5b                          	popq	%rbx
14230f1c5: 5f                          	popq	%rdi
14230f1c6: 5e                          	popq	%rsi
14230f1c7: 41 5c                       	popq	%r12
14230f1c9: 41 5d                       	popq	%r13
14230f1cb: 41 5e                       	popq	%r14
14230f1cd: 41 5f                       	popq	%r15
14230f1cf: 5d                          	popq	%rbp
14230f1d0: c3                          	retq
