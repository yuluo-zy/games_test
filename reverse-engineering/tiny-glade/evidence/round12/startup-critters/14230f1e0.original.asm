
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14230f1e0: 55                          	pushq	%rbp
14230f1e1: 41 57                       	pushq	%r15
14230f1e3: 41 56                       	pushq	%r14
14230f1e5: 41 55                       	pushq	%r13
14230f1e7: 41 54                       	pushq	%r12
14230f1e9: 56                          	pushq	%rsi
14230f1ea: 57                          	pushq	%rdi
14230f1eb: 53                          	pushq	%rbx
14230f1ec: 48 83 ec 78                 	subq	$0x78, %rsp
14230f1f0: 48 8d 6c 24 70              	leaq	0x70(%rsp), %rbp
14230f1f5: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14230f1fd: 4c 89 c7                    	movq	%r8, %rdi
14230f200: 49 89 d7                    	movq	%rdx, %r15
14230f203: 48 89 cb                    	movq	%rcx, %rbx
14230f206: e8 f5 77 90 fe              	callq	0x140c16a00 <.text+0xc15a00>
14230f20b: 41 89 c5                    	movl	%eax, %r13d
14230f20e: 84 c0                       	testb	%al, %al
14230f210: 74 5d                       	je	0x14230f26f <.text+0x230e26f>
14230f212: 41 be 75 00 00 00           	movl	$0x75, %r14d
14230f218: b9 75 00 00 00              	movl	$0x75, %ecx
14230f21d: e8 ee 03 00 00              	callq	0x14230f610 <.text+0x230e610>
14230f222: 48 8d 35 47 b4 d3 00        	leaq	0xd3b447(%rip), %rsi    # 0x14304a670
14230f229: 48 83 f8 01                 	cmpq	$0x1, %rax
14230f22d: 0f 85 a8 00 00 00           	jne	0x14230f2db <.text+0x230e2db>
14230f233: 49 89 d1                    	movq	%rdx, %r9
14230f236: 48 85 d2                    	testq	%rdx, %rdx
14230f239: 74 3d                       	je	0x14230f278 <.text+0x230e278>
14230f23b: 49 83 f9 75                 	cmpq	$0x75, %r9
14230f23f: 73 35                       	jae	0x14230f276 <.text+0x230e276>
14230f241: 48 8d 05 28 b4 d3 00        	leaq	0xd3b428(%rip), %rax    # 0x14304a670
14230f248: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14230f24d: 7f 29                       	jg	0x14230f278 <.text+0x230e278>
14230f24f: 48 8d 05 12 b5 d3 00        	leaq	0xd3b512(%rip), %rax    # 0x14304a768
14230f256: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14230f25b: 48 8d 0d 0e b4 d3 00        	leaq	0xd3b40e(%rip), %rcx    # 0x14304a670
14230f262: ba 75 00 00 00              	movl	$0x75, %edx
14230f267: 45 31 c0                    	xorl	%r8d, %r8d
14230f26a: e8 d1 a6 5c 00              	callq	0x1428d9940 <.text+0x28d8940>
14230f26f: 31 c0                       	xorl	%eax, %eax
14230f271: e9 d6 01 00 00              	jmp	0x14230f44c <.text+0x230e44c>
14230f276: 75 d7                       	jne	0x14230f24f <.text+0x230e24f>
14230f278: 4c 89 c9                    	movq	%r9, %rcx
14230f27b: e8 90 03 00 00              	callq	0x14230f610 <.text+0x230e610>
14230f280: 48 83 f8 01                 	cmpq	$0x1, %rax
14230f284: 75 55                       	jne	0x14230f2db <.text+0x230e2db>
14230f286: 49 89 d0                    	movq	%rdx, %r8
14230f289: 49 83 c0 02                 	addq	$0x2, %r8
14230f28d: 74 39                       	je	0x14230f2c8 <.text+0x230e2c8>
14230f28f: 49 83 f8 75                 	cmpq	$0x75, %r8
14230f293: 73 31                       	jae	0x14230f2c6 <.text+0x230e2c6>
14230f295: 48 8d 05 d4 b3 d3 00        	leaq	0xd3b3d4(%rip), %rax    # 0x14304a670
14230f29c: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14230f2a1: 7f 25                       	jg	0x14230f2c8 <.text+0x230e2c8>
14230f2a3: 48 8d 05 d6 b4 d3 00        	leaq	0xd3b4d6(%rip), %rax    # 0x14304a780
14230f2aa: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14230f2af: 48 8d 0d ba b3 d3 00        	leaq	0xd3b3ba(%rip), %rcx    # 0x14304a670
14230f2b6: ba 75 00 00 00              	movl	$0x75, %edx
14230f2bb: 41 b9 75 00 00 00           	movl	$0x75, %r9d
14230f2c1: e8 7a a6 5c 00              	callq	0x1428d9940 <.text+0x28d8940>
14230f2c6: 75 db                       	jne	0x14230f2a3 <.text+0x230e2a3>
14230f2c8: 41 be 73 00 00 00           	movl	$0x73, %r14d
14230f2ce: 49 29 d6                    	subq	%rdx, %r14
14230f2d1: 48 8d 35 98 b3 d3 00        	leaq	0xd3b398(%rip), %rsi    # 0x14304a670
14230f2d8: 4c 01 c6                    	addq	%r8, %rsi
14230f2db: 4c 8d 05 c2 b2 d3 00        	leaq	0xd3b2c2(%rip), %r8     # 0x14304a5a4
14230f2e2: 4c 8d 25 95 b2 d3 00        	leaq	0xd3b295(%rip), %r12    # 0x14304a57e
14230f2e9: 48 89 5d f8                 	movq	%rbx, -0x8(%rbp)
14230f2ed: 0f 1f 00                    	nopl	(%rax)
14230f2f0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14230f2f5: 85 c0                       	testl	%eax, %eax
14230f2f7: 78 17                       	js	0x14230f310 <.text+0x230e310>
14230f2f9: 49 ff c8                    	decq	%r8
14230f2fc: 83 f8 5c                    	cmpl	$0x5c, %eax
14230f2ff: 75 63                       	jne	0x14230f364 <.text+0x230e364>
14230f301: eb 7c                       	jmp	0x14230f37f <.text+0x230e37f>
14230f303: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14230f310: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14230f315: 80 f9 c0                    	cmpb	$-0x40, %cl
14230f318: 7d 23                       	jge	0x14230f33d <.text+0x230e33d>
14230f31a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14230f31f: 80 fa c0                    	cmpb	$-0x40, %dl
14230f322: 7d 22                       	jge	0x14230f346 <.text+0x230e346>
14230f324: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14230f329: 49 83 c0 fc                 	addq	$-0x4, %r8
14230f32d: 41 83 e1 07                 	andl	$0x7, %r9d
14230f331: 41 c1 e1 06                 	shll	$0x6, %r9d
14230f335: 83 e2 3f                    	andl	$0x3f, %edx
14230f338: 44 09 ca                    	orl	%r9d, %edx
14230f33b: eb 10                       	jmp	0x14230f34d <.text+0x230e34d>
14230f33d: 49 83 c0 fe                 	addq	$-0x2, %r8
14230f341: 83 e1 1f                    	andl	$0x1f, %ecx
14230f344: eb 0f                       	jmp	0x14230f355 <.text+0x230e355>
14230f346: 49 83 c0 fd                 	addq	$-0x3, %r8
14230f34a: 83 e2 0f                    	andl	$0xf, %edx
14230f34d: c1 e2 06                    	shll	$0x6, %edx
14230f350: 83 e1 3f                    	andl	$0x3f, %ecx
14230f353: 09 d1                       	orl	%edx, %ecx
14230f355: c1 e1 06                    	shll	$0x6, %ecx
14230f358: 24 3f                       	andb	$0x3f, %al
14230f35a: 0f b6 c0                    	movzbl	%al, %eax
14230f35d: 09 c8                       	orl	%ecx, %eax
14230f35f: 83 f8 5c                    	cmpl	$0x5c, %eax
14230f362: 74 1b                       	je	0x14230f37f <.text+0x230e37f>
14230f364: 83 f8 2f                    	cmpl	$0x2f, %eax
14230f367: 74 16                       	je	0x14230f37f <.text+0x230e37f>
14230f369: 4d 39 e0                    	cmpq	%r12, %r8
14230f36c: 75 82                       	jne	0x14230f2f0 <.text+0x230e2f0>
14230f36e: 48 89 fb                    	movq	%rdi, %rbx
14230f371: 4c 89 ff                    	movq	%r15, %rdi
14230f374: 45 89 ef                    	movl	%r13d, %r15d
14230f377: 41 bd 26 00 00 00           	movl	$0x26, %r13d
14230f37d: eb 56                       	jmp	0x14230f3d5 <.text+0x230e3d5>
14230f37f: 4c 8d 25 f8 b1 d3 00        	leaq	0xd3b1f8(%rip), %r12    # 0x14304a57e
14230f386: 4d 29 e0                    	subq	%r12, %r8
14230f389: 49 ff c0                    	incq	%r8
14230f38c: 74 32                       	je	0x14230f3c0 <.text+0x230e3c0>
14230f38e: 49 83 f8 26                 	cmpq	$0x26, %r8
14230f392: 73 2a                       	jae	0x14230f3be <.text+0x230e3be>
14230f394: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
14230f399: 7f 25                       	jg	0x14230f3c0 <.text+0x230e3c0>
14230f39b: 48 8d 05 a6 b3 d3 00        	leaq	0xd3b3a6(%rip), %rax    # 0x14304a748
14230f3a2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14230f3a7: 48 8d 0d d0 b1 d3 00        	leaq	0xd3b1d0(%rip), %rcx    # 0x14304a57e
14230f3ae: ba 26 00 00 00              	movl	$0x26, %edx
14230f3b3: 41 b9 26 00 00 00           	movl	$0x26, %r9d
14230f3b9: e8 82 a5 5c 00              	callq	0x1428d9940 <.text+0x28d8940>
14230f3be: 75 db                       	jne	0x14230f39b <.text+0x230e39b>
14230f3c0: 48 89 fb                    	movq	%rdi, %rbx
14230f3c3: 4c 89 ff                    	movq	%r15, %rdi
14230f3c6: 45 89 ef                    	movl	%r13d, %r15d
14230f3c9: 41 bd 26 00 00 00           	movl	$0x26, %r13d
14230f3cf: 4d 29 c5                    	subq	%r8, %r13
14230f3d2: 4d 01 c4                    	addq	%r8, %r12
14230f3d5: e8 56 8d 90 fe              	callq	0x140c18130 <.text+0xc17130>
14230f3da: 48 89 c1                    	movq	%rax, %rcx
14230f3dd: 48 8b 00                    	movq	(%rax), %rax
14230f3e0: 48 83 f8 01                 	cmpq	$0x1, %rax
14230f3e4: 0f 85 d9 00 00 00           	jne	0x14230f4c3 <.text+0x230e4c3>
14230f3ea: 48 83 c1 08                 	addq	$0x8, %rcx
14230f3ee: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14230f3f2: 0f 85 20 01 00 00           	jne	0x14230f518 <.text+0x230e518>
14230f3f8: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14230f3ff: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
14230f403: 48 83 c1 08                 	addq	$0x8, %rcx
14230f407: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
14230f40c: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14230f415: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14230f41e: 48 89 f2                    	movq	%rsi, %rdx
14230f421: 4d 89 f0                    	movq	%r14, %r8
14230f424: 4d 89 e1                    	movq	%r12, %r9
14230f427: e8 14 79 90 fe              	callq	0x140c16d40 <.text+0xc15d40>
14230f42c: 49 89 c6                    	movq	%rax, %r14
14230f42f: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14230f433: 48 ff 00                    	incq	(%rax)
14230f436: 4c 89 75 d8                 	movq	%r14, -0x28(%rbp)
14230f43a: b8 01 00 00 00              	movl	$0x1, %eax
14230f43f: 45 89 fd                    	movl	%r15d, %r13d
14230f442: 49 89 ff                    	movq	%rdi, %r15
14230f445: 48 89 df                    	movq	%rbx, %rdi
14230f448: 48 8b 5d f8                 	movq	-0x8(%rbp), %rbx
14230f44c: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
14230f450: 4c 8b 43 08                 	movq	0x8(%rbx), %r8
14230f454: 4c 39 c7                    	cmpq	%r8, %rdi
14230f457: 4c 0f 42 c7                 	cmovbq	%rdi, %r8
14230f45b: 0f 87 9c 00 00 00           	ja	0x14230f4fd <.text+0x230e4fd>
14230f461: 48 8b 0b                    	movq	(%rbx), %rcx
14230f464: 49 c1 e0 04                 	shlq	$0x4, %r8
14230f468: 4c 89 fa                    	movq	%r15, %rdx
14230f46b: e8 90 47 61 00              	callq	0x142923c00 <.text+0x2922c00>
14230f470: 45 84 ed                    	testb	%r13b, %r13b
14230f473: 74 3d                       	je	0x14230f4b2 <.text+0x230e4b2>
14230f475: e8 b6 8c 90 fe              	callq	0x140c18130 <.text+0xc17130>
14230f47a: 48 89 c1                    	movq	%rax, %rcx
14230f47d: 48 8b 00                    	movq	(%rax), %rax
14230f480: 48 83 f8 01                 	cmpq	$0x1, %rax
14230f484: 75 57                       	jne	0x14230f4dd <.text+0x230e4dd>
14230f486: 48 83 c1 08                 	addq	$0x8, %rcx
14230f48a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14230f48e: 0f 85 84 00 00 00           	jne	0x14230f518 <.text+0x230e518>
14230f494: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14230f49b: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
14230f49f: 48 83 c1 08                 	addq	$0x8, %rcx
14230f4a3: 4c 89 f2                    	movq	%r14, %rdx
14230f4a6: e8 45 7c 90 fe              	callq	0x140c170f0 <.text+0xc160f0>
14230f4ab: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14230f4af: 48 ff 00                    	incq	(%rax)
14230f4b2: 48 83 c4 78                 	addq	$0x78, %rsp
14230f4b6: 5b                          	popq	%rbx
14230f4b7: 5f                          	popq	%rdi
14230f4b8: 5e                          	popq	%rsi
14230f4b9: 41 5c                       	popq	%r12
14230f4bb: 41 5d                       	popq	%r13
14230f4bd: 41 5e                       	popq	%r14
14230f4bf: 41 5f                       	popq	%r15
14230f4c1: 5d                          	popq	%rbp
14230f4c2: c3                          	retq
14230f4c3: 48 85 c0                    	testq	%rax, %rax
14230f4c6: 75 29                       	jne	0x14230f4f1 <.text+0x230e4f1>
14230f4c8: 31 d2                       	xorl	%edx, %edx
14230f4ca: e8 61 bb 60 00              	callq	0x14291b030 <.text+0x291a030>
14230f4cf: 48 89 c1                    	movq	%rax, %rcx
14230f4d2: 48 85 c0                    	testq	%rax, %rax
14230f4d5: 0f 85 13 ff ff ff           	jne	0x14230f3ee <.text+0x230e3ee>
14230f4db: eb 14                       	jmp	0x14230f4f1 <.text+0x230e4f1>
14230f4dd: 48 85 c0                    	testq	%rax, %rax
14230f4e0: 75 0f                       	jne	0x14230f4f1 <.text+0x230e4f1>
14230f4e2: 31 d2                       	xorl	%edx, %edx
14230f4e4: e8 47 bb 60 00              	callq	0x14291b030 <.text+0x291a030>
14230f4e9: 48 89 c1                    	movq	%rax, %rcx
14230f4ec: 48 85 c0                    	testq	%rax, %rax
14230f4ef: 75 99                       	jne	0x14230f48a <.text+0x230e48a>
14230f4f1: 48 8d 0d d8 ab d3 00        	leaq	0xd3abd8(%rip), %rcx    # 0x14304a0d0
14230f4f8: e8 d3 8b 5c 00              	callq	0x1428d80d0 <.text+0x28d70d0>
14230f4fd: 44 88 6d f8                 	movb	%r13b, -0x8(%rbp)
14230f501: 48 8d 05 a0 b0 d3 00        	leaq	0xd3b0a0(%rip), %rax    # 0x14304a5a8
14230f508: 4c 89 c1                    	movq	%r8, %rcx
14230f50b: 48 89 fa                    	movq	%rdi, %rdx
14230f50e: 49 89 c0                    	movq	%rax, %r8
14230f511: e8 ca a3 5c 00              	callq	0x1428d98e0 <.text+0x28d88e0>
14230f516: 0f 0b                       	ud2
14230f518: 48 8d 0d 21 ac d3 00        	leaq	0xd3ac21(%rip), %rcx    # 0x14304a140
14230f51f: e8 2c 9d 5c 00              	callq	0x1428d9250 <.text+0x28d8250>
14230f524: cc                          	int3
14230f525: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14230f530: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14230f535: 55                          	pushq	%rbp
14230f536: 41 57                       	pushq	%r15
14230f538: 41 56                       	pushq	%r14
14230f53a: 41 55                       	pushq	%r13
14230f53c: 41 54                       	pushq	%r12
14230f53e: 56                          	pushq	%rsi
14230f53f: 57                          	pushq	%rdi
14230f540: 53                          	pushq	%rbx
14230f541: 48 83 ec 38                 	subq	$0x38, %rsp
14230f545: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14230f549: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14230f54d: 48 ff 00                    	incq	(%rax)
14230f550: 48 83 c4 38                 	addq	$0x38, %rsp
14230f554: 5b                          	popq	%rbx
14230f555: 5f                          	popq	%rdi
14230f556: 5e                          	popq	%rsi
14230f557: 41 5c                       	popq	%r12
14230f559: 41 5d                       	popq	%r13
14230f55b: 41 5e                       	popq	%r14
14230f55d: 41 5f                       	popq	%r15
14230f55f: 5d                          	popq	%rbp
14230f560: c3                          	retq
14230f561: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14230f570: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14230f575: 55                          	pushq	%rbp
14230f576: 41 57                       	pushq	%r15
14230f578: 41 56                       	pushq	%r14
14230f57a: 41 55                       	pushq	%r13
14230f57c: 41 54                       	pushq	%r12
14230f57e: 56                          	pushq	%rsi
14230f57f: 57                          	pushq	%rdi
14230f580: 53                          	pushq	%rbx
14230f581: 48 83 ec 38                 	subq	$0x38, %rsp
14230f585: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14230f589: 80 7d f8 00                 	cmpb	$0x0, -0x8(%rbp)
14230f58d: 74 20                       	je	0x14230f5af <.text+0x230e5af>
14230f58f: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
14230f593: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14230f597: 48 8d 45 e8                 	leaq	-0x18(%rbp), %rax
14230f59b: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14230f59f: 48 8d 0d 42 b1 d3 00        	leaq	0xd3b142(%rip), %rcx    # 0x14304a6e8
14230f5a6: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
14230f5aa: e8 41 f0 ff ff              	callq	0x14230e5f0 <.text+0x230d5f0>
14230f5af: 90                          	nop
14230f5b0: 48 83 c4 38                 	addq	$0x38, %rsp
14230f5b4: 5b                          	popq	%rbx
14230f5b5: 5f                          	popq	%rdi
14230f5b6: 5e                          	popq	%rsi
14230f5b7: 41 5c                       	popq	%r12
14230f5b9: 41 5d                       	popq	%r13
14230f5bb: 41 5e                       	popq	%r14
14230f5bd: 41 5f                       	popq	%r15
14230f5bf: 5d                          	popq	%rbp
14230f5c0: c3                          	retq
14230f5c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14230f5d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14230f5d5: 55                          	pushq	%rbp
14230f5d6: 41 57                       	pushq	%r15
14230f5d8: 41 56                       	pushq	%r14
14230f5da: 41 55                       	pushq	%r13
14230f5dc: 41 54                       	pushq	%r12
14230f5de: 56                          	pushq	%rsi
14230f5df: 57                          	pushq	%rdi
14230f5e0: 53                          	pushq	%rbx
14230f5e1: 48 83 ec 38                 	subq	$0x38, %rsp
14230f5e5: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14230f5e9: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14230f5ed: 48 ff 00                    	incq	(%rax)
14230f5f0: 48 83 c4 38                 	addq	$0x38, %rsp
14230f5f4: 5b                          	popq	%rbx
14230f5f5: 5f                          	popq	%rdi
14230f5f6: 5e                          	popq	%rsi
14230f5f7: 41 5c                       	popq	%r12
14230f5f9: 41 5d                       	popq	%r13
14230f5fb: 41 5e                       	popq	%r14
14230f5fd: 41 5f                       	popq	%r15
14230f5ff: 5d                          	popq	%rbp
14230f600: c3                          	retq
