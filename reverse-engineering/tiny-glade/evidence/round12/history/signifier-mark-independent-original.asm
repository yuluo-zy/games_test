
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140929160: 55                          	pushq	%rbp
140929161: 41 57                       	pushq	%r15
140929163: 41 56                       	pushq	%r14
140929165: 41 55                       	pushq	%r13
140929167: 41 54                       	pushq	%r12
140929169: 56                          	pushq	%rsi
14092916a: 57                          	pushq	%rdi
14092916b: 53                          	pushq	%rbx
14092916c: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
140929173: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14092917b: 66 0f 7f b5 80 00 00 00     	movdqa	%xmm6, 0x80(%rbp)
140929183: 48 c7 45 78 fe ff ff ff     	movq	$-0x2, 0x78(%rbp)
14092918b: 4c 89 c3                    	movq	%r8, %rbx
14092918e: 49 89 d6                    	movq	%rdx, %r14
140929191: 48 89 cf                    	movq	%rcx, %rdi
140929194: e8 67 d8 2e 00              	callq	0x140c16a00 <.text+0xc15a00>
140929199: 84 c0                       	testb	%al, %al
14092919b: 88 45 77                    	movb	%al, 0x77(%rbp)
14092919e: 74 5f                       	je	0x1409291ff <.text+0x9281ff>
1409291a0: 4c 8d 3d 61 54 18 02        	leaq	0x2185461(%rip), %r15   # 0x142aae608
1409291a7: be 5a 00 00 00              	movl	$0x5a, %esi
1409291ac: ba 5a 00 00 00              	movl	$0x5a, %edx
1409291b1: 4c 89 f9                    	movq	%r15, %rcx
1409291b4: e8 67 a7 ff ff              	callq	0x140923920 <.text+0x922920>
1409291b9: 48 83 f8 01                 	cmpq	$0x1, %rax
1409291bd: 0f 85 b1 00 00 00           	jne	0x140929274 <.text+0x928274>
1409291c3: 49 89 d1                    	movq	%rdx, %r9
1409291c6: 48 85 d2                    	testq	%rdx, %rdx
1409291c9: 74 3d                       	je	0x140929208 <.text+0x928208>
1409291cb: 49 83 f9 5a                 	cmpq	$0x5a, %r9
1409291cf: 73 35                       	jae	0x140929206 <.text+0x928206>
1409291d1: 48 8d 05 30 54 18 02        	leaq	0x2185430(%rip), %rax   # 0x142aae608
1409291d8: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1409291dd: 7f 29                       	jg	0x140929208 <.text+0x928208>
1409291df: 48 8d 05 52 55 18 02        	leaq	0x2185552(%rip), %rax   # 0x142aae738
1409291e6: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409291eb: 48 8d 0d 16 54 18 02        	leaq	0x2185416(%rip), %rcx   # 0x142aae608
1409291f2: ba 5a 00 00 00              	movl	$0x5a, %edx
1409291f7: 45 31 c0                    	xorl	%r8d, %r8d
1409291fa: e8 41 07 fb 01              	callq	0x1428d9940 <.text+0x28d8940>
1409291ff: 31 c0                       	xorl	%eax, %eax
140929201: e9 d4 01 00 00              	jmp	0x1409293da <.text+0x9283da>
140929206: 75 d7                       	jne	0x1409291df <.text+0x9281df>
140929208: 4c 8d 3d f9 53 18 02        	leaq	0x21853f9(%rip), %r15   # 0x142aae608
14092920f: 4c 89 f9                    	movq	%r15, %rcx
140929212: 4c 89 ca                    	movq	%r9, %rdx
140929215: e8 06 a7 ff ff              	callq	0x140923920 <.text+0x922920>
14092921a: 48 83 f8 01                 	cmpq	$0x1, %rax
14092921e: 75 54                       	jne	0x140929274 <.text+0x928274>
140929220: 49 89 d0                    	movq	%rdx, %r8
140929223: 49 83 c0 02                 	addq	$0x2, %r8
140929227: 74 39                       	je	0x140929262 <.text+0x928262>
140929229: 49 83 f8 5a                 	cmpq	$0x5a, %r8
14092922d: 73 31                       	jae	0x140929260 <.text+0x928260>
14092922f: 48 8d 05 d2 53 18 02        	leaq	0x21853d2(%rip), %rax   # 0x142aae608
140929236: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14092923b: 7f 25                       	jg	0x140929262 <.text+0x928262>
14092923d: 48 8d 05 0c 55 18 02        	leaq	0x218550c(%rip), %rax   # 0x142aae750
140929244: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140929249: 48 8d 0d b8 53 18 02        	leaq	0x21853b8(%rip), %rcx   # 0x142aae608
140929250: ba 5a 00 00 00              	movl	$0x5a, %edx
140929255: 41 b9 5a 00 00 00           	movl	$0x5a, %r9d
14092925b: e8 e0 06 fb 01              	callq	0x1428d9940 <.text+0x28d8940>
140929260: 75 db                       	jne	0x14092923d <.text+0x92823d>
140929262: be 58 00 00 00              	movl	$0x58, %esi
140929267: 48 29 d6                    	subq	%rdx, %rsi
14092926a: 4c 8d 3d 97 53 18 02        	leaq	0x2185397(%rip), %r15   # 0x142aae608
140929271: 4d 01 c7                    	addq	%r8, %r15
140929274: 48 8d 05 0b 63 18 02        	leaq	0x218630b(%rip), %rax   # 0x142aaf586
14092927b: 4c 8d 25 ba 62 18 02        	leaq	0x21862ba(%rip), %r12   # 0x142aaf53c
140929282: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140929290: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140929294: 85 c9                       	testl	%ecx, %ecx
140929296: 78 18                       	js	0x1409292b0 <.text+0x9282b0>
140929298: 48 ff c8                    	decq	%rax
14092929b: 83 f9 5c                    	cmpl	$0x5c, %ecx
14092929e: 75 69                       	jne	0x140929309 <.text+0x928309>
1409292a0: eb 7d                       	jmp	0x14092931f <.text+0x92831f>
1409292a2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409292b0: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
1409292b4: 80 fa c0                    	cmpb	$-0x40, %dl
1409292b7: 7d 25                       	jge	0x1409292de <.text+0x9282de>
1409292b9: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
1409292be: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
1409292c2: 7d 23                       	jge	0x1409292e7 <.text+0x9282e7>
1409292c4: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
1409292c9: 48 83 c0 fc                 	addq	$-0x4, %rax
1409292cd: 41 83 e1 07                 	andl	$0x7, %r9d
1409292d1: 41 c1 e1 06                 	shll	$0x6, %r9d
1409292d5: 41 83 e0 3f                 	andl	$0x3f, %r8d
1409292d9: 45 09 c8                    	orl	%r9d, %r8d
1409292dc: eb 11                       	jmp	0x1409292ef <.text+0x9282ef>
1409292de: 48 83 c0 fe                 	addq	$-0x2, %rax
1409292e2: 83 e2 1f                    	andl	$0x1f, %edx
1409292e5: eb 12                       	jmp	0x1409292f9 <.text+0x9282f9>
1409292e7: 48 83 c0 fd                 	addq	$-0x3, %rax
1409292eb: 41 83 e0 0f                 	andl	$0xf, %r8d
1409292ef: 41 c1 e0 06                 	shll	$0x6, %r8d
1409292f3: 83 e2 3f                    	andl	$0x3f, %edx
1409292f6: 44 09 c2                    	orl	%r8d, %edx
1409292f9: c1 e2 06                    	shll	$0x6, %edx
1409292fc: 80 e1 3f                    	andb	$0x3f, %cl
1409292ff: 0f b6 c9                    	movzbl	%cl, %ecx
140929302: 09 d1                       	orl	%edx, %ecx
140929304: 83 f9 5c                    	cmpl	$0x5c, %ecx
140929307: 74 16                       	je	0x14092931f <.text+0x92831f>
140929309: 83 f9 2f                    	cmpl	$0x2f, %ecx
14092930c: 74 11                       	je	0x14092931f <.text+0x92831f>
14092930e: 4c 39 e0                    	cmpq	%r12, %rax
140929311: 0f 85 79 ff ff ff           	jne	0x140929290 <.text+0x928290>
140929317: 41 bd 4a 00 00 00           	movl	$0x4a, %r13d
14092931d: eb 50                       	jmp	0x14092936f <.text+0x92836f>
14092931f: 4c 8d 25 16 62 18 02        	leaq	0x2186216(%rip), %r12   # 0x142aaf53c
140929326: 4c 29 e0                    	subq	%r12, %rax
140929329: 49 89 c0                    	movq	%rax, %r8
14092932c: 49 ff c0                    	incq	%r8
14092932f: 74 32                       	je	0x140929363 <.text+0x928363>
140929331: 49 83 f8 4a                 	cmpq	$0x4a, %r8
140929335: 73 2a                       	jae	0x140929361 <.text+0x928361>
140929337: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
14092933c: 7f 25                       	jg	0x140929363 <.text+0x928363>
14092933e: 48 8d 05 d3 53 18 02        	leaq	0x21853d3(%rip), %rax   # 0x142aae718
140929345: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14092934a: 48 8d 0d eb 61 18 02        	leaq	0x21861eb(%rip), %rcx   # 0x142aaf53c
140929351: ba 4a 00 00 00              	movl	$0x4a, %edx
140929356: 41 b9 4a 00 00 00           	movl	$0x4a, %r9d
14092935c: e8 df 05 fb 01              	callq	0x1428d9940 <.text+0x28d8940>
140929361: 75 db                       	jne	0x14092933e <.text+0x92833e>
140929363: 41 bd 49 00 00 00           	movl	$0x49, %r13d
140929369: 49 29 c5                    	subq	%rax, %r13
14092936c: 4d 01 c4                    	addq	%r8, %r12
14092936f: e8 bc ed 2e 00              	callq	0x140c18130 <.text+0xc17130>
140929374: 48 89 c1                    	movq	%rax, %rcx
140929377: 48 8b 00                    	movq	(%rax), %rax
14092937a: 48 83 f8 01                 	cmpq	$0x1, %rax
14092937e: 0f 85 87 05 00 00           	jne	0x14092990b <.text+0x92890b>
140929384: 48 83 c1 08                 	addq	$0x8, %rcx
140929388: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14092938c: 0f 85 b3 05 00 00           	jne	0x140929945 <.text+0x928945>
140929392: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140929399: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
14092939d: 48 83 c1 08                 	addq	$0x8, %rcx
1409293a1: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
1409293a6: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1409293af: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1409293b8: 4c 89 fa                    	movq	%r15, %rdx
1409293bb: 49 89 f0                    	movq	%rsi, %r8
1409293be: 4d 89 e1                    	movq	%r12, %r9
1409293c1: e8 7a d9 2e 00              	callq	0x140c16d40 <.text+0xc15d40>
1409293c6: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1409293ca: 48 ff 01                    	incq	(%rcx)
1409293cd: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
1409293d1: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1409293d5: b8 01 00 00 00              	movl	$0x1, %eax
1409293da: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1409293de: 31 c9                       	xorl	%ecx, %ecx
1409293e0: e8 3b 07 7b ff              	callq	0x1400d9b20 <.text+0xd8b20>
1409293e5: 48 85 c0                    	testq	%rax, %rax
1409293e8: 0f 84 63 05 00 00           	je	0x140929951 <.text+0x928951>
1409293ee: 0f 10 00                    	movups	(%rax), %xmm0
1409293f1: 48 ff 00                    	incq	(%rax)
1409293f4: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
1409293f8: 0f 10 05 11 2b 34 02        	movups	0x2342b11(%rip), %xmm0  # 0x142c6bf10
1409293ff: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140929403: f3 0f 6f 05 f5 2a 34 02     	movdqu	0x2342af5(%rip), %xmm0  # 0x142c6bf00
14092940b: 66 0f 7f 45 e0              	movdqa	%xmm0, -0x20(%rbp)
140929410: 48 8b 37                    	movq	(%rdi), %rsi
140929413: 80 7e 18 00                 	cmpb	$0x0, 0x18(%rsi)
140929417: 74 59                       	je	0x140929472 <.text+0x928472>
140929419: 48 8b 46 20                 	movq	0x20(%rsi), %rax
14092941d: 4c 8b 60 18                 	movq	0x18(%rax), %r12
140929421: 48 8b 40 20                 	movq	0x20(%rax), %rax
140929425: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
140929429: 4d 8d 2c c4                 	leaq	(%r12,%rax,8), %r13
14092942d: 48 8d 7d b8                 	leaq	-0x48(%rbp), %rdi
140929431: 4c 8d 7d e0                 	leaq	-0x20(%rbp), %r15
140929435: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140929440: 4d 39 ec                    	cmpq	%r13, %r12
140929443: 74 2d                       	je	0x140929472 <.text+0x928472>
140929445: 48 8b 46 38                 	movq	0x38(%rsi), %rax
140929449: f3 41 0f 6f 44 24 08        	movdqu	0x8(%r12), %xmm0
140929450: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140929454: f3 0f 7f 45 c0              	movdqu	%xmm0, -0x40(%rbp)
140929459: 48 89 f9                    	movq	%rdi, %rcx
14092945c: e8 af 12 b3 00              	callq	0x14145a710 <.text+0x1459710>
140929461: 49 83 c4 28                 	addq	$0x28, %r12
140929465: 4c 89 f9                    	movq	%r15, %rcx
140929468: 48 89 c2                    	movq	%rax, %rdx
14092946b: e8 80 0e 9a ff              	callq	0x1402ca2f0 <.text+0x2c92f0>
140929470: eb ce                       	jmp	0x140929440 <.text+0x928440>
140929472: 49 8b 0e                    	movq	(%r14), %rcx
140929475: 4c 8b a1 f8 00 00 00        	movq	0xf8(%rcx), %r12
14092947c: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140929483: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
140929487: 48 85 d2                    	testq	%rdx, %rdx
14092948a: 75 04                       	jne	0x140929490 <.text+0x928490>
14092948c: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140929490: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
140929494: 4d 8b 6e 08                 	movq	0x8(%r14), %r13
140929498: 49 8d 34 84                 	leaq	(%r12,%rax,4), %rsi
14092949c: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
1409294a0: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
1409294a7: 88 45 60                    	movb	%al, 0x60(%rbp)
1409294aa: b8 08 00 00 00              	movl	$0x8, %eax
1409294af: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1409294b3: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1409294b7: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1409294bb: 31 db                       	xorl	%ebx, %ebx
1409294bd: 45 31 ff                    	xorl	%r15d, %r15d
1409294c0: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1409294c8: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
1409294d0: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
1409294d7: e9 84 00 00 00              	jmp	0x140929560 <.text+0x928560>
1409294dc: 0f 1f 40 00                 	nopl	(%rax)
1409294e0: f3 42 0f 6f 0c 02           	movdqu	(%rdx,%r8), %xmm1
1409294e6: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1409294ea: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1409294ee: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1409294f3: 45 85 c9                    	testl	%r9d, %r9d
1409294f6: 74 38                       	je	0x140929530 <.text+0x928530>
1409294f8: 4d 8b 16                    	movq	(%r14), %r10
1409294fb: 48 89 c8                    	movq	%rcx, %rax
1409294fe: f3 45 0f bc d9              	tzcntl	%r9d, %r11d
140929503: 4d 01 c3                    	addq	%r8, %r11
140929506: 4c 23 5d 38                 	andq	0x38(%rbp), %r11
14092950a: 49 c1 e3 03                 	shlq	$0x3, %r11
14092950e: 48 89 d1                    	movq	%rdx, %rcx
140929511: 4c 29 d9                    	subq	%r11, %rcx
140929514: 4c 3b 51 f8                 	cmpq	-0x8(%rcx), %r10
140929518: 74 46                       	je	0x140929560 <.text+0x928560>
14092951a: 41 8d 49 ff                 	leal	-0x1(%r9), %ecx
14092951e: 66 44 21 c9                 	andw	%r9w, %cx
140929522: 41 89 c9                    	movl	%ecx, %r9d
140929525: 48 89 c1                    	movq	%rax, %rcx
140929528: 75 d1                       	jne	0x1409294fb <.text+0x9284fb>
14092952a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140929530: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140929534: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
140929538: 85 c0                       	testl	%eax, %eax
14092953a: 0f 85 c0 02 00 00           	jne	0x140929800 <.text+0x928800>
140929540: 49 01 c8                    	addq	%rcx, %r8
140929543: 49 83 c0 10                 	addq	$0x10, %r8
140929547: 48 83 c1 10                 	addq	$0x10, %rcx
14092954b: 4c 23 45 38                 	andq	0x38(%rbp), %r8
14092954f: eb 8f                       	jmp	0x1409294e0 <.text+0x9284e0>
140929551: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140929560: 80 7d 60 00                 	cmpb	$0x0, 0x60(%rbp)
140929564: 0f 84 96 00 00 00           	je	0x140929600 <.text+0x928600>
14092956a: 49 39 df                    	cmpq	%rbx, %r15
14092956d: 0f 85 dd 01 00 00           	jne	0x140929750 <.text+0x928750>
140929573: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140929580: 49 39 f4                    	cmpq	%rsi, %r12
140929583: 0f 84 f7 02 00 00           	je	0x140929880 <.text+0x928880>
140929589: 41 8b 0c 24                 	movl	(%r12), %ecx
14092958d: 49 83 c4 04                 	addq	$0x4, %r12
140929591: 49 8b 85 a8 01 00 00        	movq	0x1a8(%r13), %rax
140929598: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14092959c: 48 8b 5c c8 10              	movq	0x10(%rax,%rcx,8), %rbx
1409295a1: 48 85 db                    	testq	%rbx, %rbx
1409295a4: 74 da                       	je	0x140929580 <.text+0x928580>
1409295a6: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1409295aa: 4c 8b 4d 50                 	movq	0x50(%rbp), %r9
1409295ae: 49 8b 91 10 01 00 00        	movq	0x110(%r9), %rdx
1409295b5: 48 8b 48 40                 	movq	0x40(%rax), %rcx
1409295b9: 48 39 d1                    	cmpq	%rdx, %rcx
1409295bc: 0f 86 df 00 00 00           	jbe	0x1409296a1 <.text+0x9286a1>
1409295c2: 4c 8b 40 38                 	movq	0x38(%rax), %r8
1409295c6: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
1409295ca: 48 85 d2                    	testq	%rdx, %rdx
1409295cd: 0f 84 ce 00 00 00           	je	0x1409296a1 <.text+0x9286a1>
1409295d3: 4c 8b 40 18                 	movq	0x18(%rax), %r8
1409295d7: 48 f7 d2                    	notq	%rdx
1409295da: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1409295de: 48 c1 e2 04                 	shlq	$0x4, %rdx
1409295e2: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
1409295e7: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
1409295eb: 49 8b 91 18 01 00 00        	movq	0x118(%r9), %rdx
1409295f2: 48 39 d1                    	cmpq	%rdx, %rcx
1409295f5: 0f 87 be 00 00 00           	ja	0x1409296b9 <.text+0x9286b9>
1409295fb: e9 3e 01 00 00              	jmp	0x14092973e <.text+0x92873e>
140929600: 49 39 df                    	cmpq	%rbx, %r15
140929603: 0f 85 70 01 00 00           	jne	0x140929779 <.text+0x928779>
140929609: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140929610: 49 39 f4                    	cmpq	%rsi, %r12
140929613: 0f 84 67 02 00 00           	je	0x140929880 <.text+0x928880>
140929619: 41 8b 0c 24                 	movl	(%r12), %ecx
14092961d: 49 83 c4 04                 	addq	$0x4, %r12
140929621: 49 8b 85 00 01 00 00        	movq	0x100(%r13), %rax
140929628: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14092962c: 48 c1 e1 05                 	shlq	$0x5, %rcx
140929630: 48 8b 5c 08 58              	movq	0x58(%rax,%rcx), %rbx
140929635: 48 85 db                    	testq	%rbx, %rbx
140929638: 74 d6                       	je	0x140929610 <.text+0x928610>
14092963a: 48 01 c8                    	addq	%rcx, %rax
14092963d: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
140929643: 49 8b 95 a8 01 00 00        	movq	0x1a8(%r13), %rdx
14092964a: 4c 8d 0c c9                 	leaq	(%rcx,%rcx,8), %r9
14092964e: 4a 8d 0c ca                 	leaq	(%rdx,%r9,8), %rcx
140929652: 4c 8b 55 50                 	movq	0x50(%rbp), %r10
140929656: 4d 8b 82 10 01 00 00        	movq	0x110(%r10), %r8
14092965d: 4a 8b 54 ca 40              	movq	0x40(%rdx,%r9,8), %rdx
140929662: 4c 39 c2                    	cmpq	%r8, %rdx
140929665: 76 7e                       	jbe	0x1409296e5 <.text+0x9286e5>
140929667: 4c 8b 49 38                 	movq	0x38(%rcx), %r9
14092966b: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
14092966f: 4d 85 c0                    	testq	%r8, %r8
140929672: 74 71                       	je	0x1409296e5 <.text+0x9286e5>
140929674: 4c 8b 49 18                 	movq	0x18(%rcx), %r9
140929678: 49 f7 d0                    	notq	%r8
14092967b: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14092967f: 49 c1 e0 04                 	shlq	$0x4, %r8
140929683: 4f 8b 44 01 10              	movq	0x10(%r9,%r8), %r8
140929688: 4c 89 45 68                 	movq	%r8, 0x68(%rbp)
14092968c: 4d 8b 82 18 01 00 00        	movq	0x118(%r10), %r8
140929693: 4c 39 80 88 00 00 00        	cmpq	%r8, 0x88(%rax)
14092969a: 77 61                       	ja	0x1409296fd <.text+0x9286fd>
14092969c: e9 bc 00 00 00              	jmp	0x14092975d <.text+0x92875d>
1409296a1: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1409296a9: 49 8b 91 18 01 00 00        	movq	0x118(%r9), %rdx
1409296b0: 48 39 d1                    	cmpq	%rdx, %rcx
1409296b3: 0f 86 85 00 00 00           	jbe	0x14092973e <.text+0x92873e>
1409296b9: 48 8b 48 38                 	movq	0x38(%rax), %rcx
1409296bd: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
1409296c1: 48 85 c9                    	testq	%rcx, %rcx
1409296c4: 74 78                       	je	0x14092973e <.text+0x92873e>
1409296c6: 48 8b 50 18                 	movq	0x18(%rax), %rdx
1409296ca: 48 f7 d1                    	notq	%rcx
1409296cd: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409296d1: 48 c1 e1 04                 	shlq	$0x4, %rcx
1409296d5: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
1409296da: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
1409296de: b1 01                       	movb	$0x1, %cl
1409296e0: 89 4d 70                    	movl	%ecx, 0x70(%rbp)
1409296e3: eb 60                       	jmp	0x140929745 <.text+0x928745>
1409296e5: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
1409296ed: 4d 8b 82 18 01 00 00        	movq	0x118(%r10), %r8
1409296f4: 4c 39 80 88 00 00 00        	cmpq	%r8, 0x88(%rax)
1409296fb: 76 60                       	jbe	0x14092975d <.text+0x92875d>
1409296fd: 4c 8b 88 80 00 00 00        	movq	0x80(%rax), %r9
140929704: 4b 83 3c c1 00              	cmpq	$0x0, (%r9,%r8,8)
140929709: 74 52                       	je	0x14092975d <.text+0x92875d>
14092970b: 41 b1 01                    	movb	$0x1, %r9b
14092970e: 44 89 4d 70                 	movl	%r9d, 0x70(%rbp)
140929712: 4c 39 c2                    	cmpq	%r8, %rdx
140929715: 76 4f                       	jbe	0x140929766 <.text+0x928766>
140929717: 48 8b 51 38                 	movq	0x38(%rcx), %rdx
14092971b: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
14092971f: 48 85 d2                    	testq	%rdx, %rdx
140929722: 74 42                       	je	0x140929766 <.text+0x928766>
140929724: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
140929728: 48 f7 d2                    	notq	%rdx
14092972b: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14092972f: 48 c1 e2 04                 	shlq	$0x4, %rdx
140929733: 48 8b 4c 11 10              	movq	0x10(%rcx,%rdx), %rcx
140929738: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
14092973c: eb 30                       	jmp	0x14092976e <.text+0x92876e>
14092973e: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
140929745: 48 8b 40 08                 	movq	0x8(%rax), %rax
140929749: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14092974d: 45 31 ff                    	xorl	%r15d, %r15d
140929750: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140929754: 4a 8d 04 f8                 	leaq	(%rax,%r15,8), %rax
140929758: 45 89 fe                    	movl	%r15d, %r14d
14092975b: eb 30                       	jmp	0x14092978d <.text+0x92878d>
14092975d: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
140929764: eb 08                       	jmp	0x14092976e <.text+0x92876e>
140929766: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
14092976e: 48 8b 40 50                 	movq	0x50(%rax), %rax
140929772: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140929776: 45 31 ff                    	xorl	%r15d, %r15d
140929779: 4c 89 f9                    	movq	%r15, %rcx
14092977c: 48 c1 e1 04                 	shlq	$0x4, %rcx
140929780: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
140929784: 48 8d 04 0a                 	leaq	(%rdx,%rcx), %rax
140929788: 44 8b 74 0a 08              	movl	0x8(%rdx,%rcx), %r14d
14092978d: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140929791: 4a 8d 3c f1                 	leaq	(%rcx,%r14,8), %rdi
140929795: 49 c1 e6 04                 	shlq	$0x4, %r14
140929799: 4c 03 75 68                 	addq	0x68(%rbp), %r14
14092979d: f6 45 70 01                 	testb	$0x1, 0x70(%rbp)
1409297a1: b9 00 00 00 00              	movl	$0x0, %ecx
1409297a6: 48 0f 44 f9                 	cmoveq	%rcx, %rdi
1409297aa: 48 8b 00                    	movq	(%rax), %rax
1409297ad: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1409297b1: 49 ff c7                    	incq	%r15
1409297b4: 48 83 7d f8 00              	cmpq	$0x0, -0x8(%rbp)
1409297b9: 74 45                       	je	0x140929800 <.text+0x928800>
1409297bb: 48 8d 4d 00                 	leaq	(%rbp), %rcx
1409297bf: 4c 89 f2                    	movq	%r14, %rdx
1409297c2: e8 a9 75 c8 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1409297c7: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
1409297cb: 4c 8b 45 e8                 	movq	-0x18(%rbp), %r8
1409297cf: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
1409297d3: 49 21 c0                    	andq	%rax, %r8
1409297d6: 48 c1 e8 39                 	shrq	$0x39, %rax
1409297da: 66 0f 6e c0                 	movd	%eax, %xmm0
1409297de: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1409297e2: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1409297e7: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1409297ec: 31 c9                       	xorl	%ecx, %ecx
1409297ee: e9 ed fc ff ff              	jmp	0x1409294e0 <.text+0x9284e0>
1409297f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140929800: 48 85 ff                    	testq	%rdi, %rdi
140929803: 74 0e                       	je	0x140929813 <.text+0x928813>
140929805: f3 0f 10 47 04              	movss	0x4(%rdi), %xmm0
14092980a: 0f 2e 07                    	ucomiss	(%rdi), %xmm0
14092980d: 0f 87 4d fd ff ff           	ja	0x140929560 <.text+0x928560>
140929813: 48 8b 7d 28                 	movq	0x28(%rbp), %rdi
140929817: 48 8b 07                    	movq	(%rdi), %rax
14092981a: 4c 8b 77 10                 	movq	0x10(%rdi), %r14
14092981e: 4c 29 f0                    	subq	%r14, %rax
140929821: 48 83 f8 1f                 	cmpq	$0x1f, %rax
140929825: 76 37                       	jbe	0x14092985e <.text+0x92885e>
140929827: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14092982b: 48 8d 0d fe 34 9b 00        	leaq	0x9b34fe(%rip), %rcx    # 0x1412dcd30 <.text+0x12dbd30>
140929832: 4a 89 0c 30                 	movq	%rcx, (%rax,%r14)
140929836: 48 8d 0d f3 b4 c3 00        	leaq	0xc3b4f3(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
14092983d: 4a 89 4c 30 08              	movq	%rcx, 0x8(%rax,%r14)
140929842: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140929846: 4a 89 4c 30 10              	movq	%rcx, 0x10(%rax,%r14)
14092984b: 42 c6 44 30 18 00           	movb	$0x0, 0x18(%rax,%r14)
140929851: 49 83 c6 20                 	addq	$0x20, %r14
140929855: 4c 89 77 10                 	movq	%r14, 0x10(%rdi)
140929859: e9 02 fd ff ff              	jmp	0x140929560 <.text+0x928560>
14092985e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140929867: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14092986d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140929873: 48 89 f9                    	movq	%rdi, %rcx
140929876: 4c 89 f2                    	movq	%r14, %rdx
140929879: e8 e2 01 f8 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14092987e: eb a7                       	jmp	0x140929827 <.text+0x928827>
140929880: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140929884: 48 85 d2                    	testq	%rdx, %rdx
140929887: 74 27                       	je	0x1409298b0 <.text+0x9288b0>
140929889: 48 8d 04 d5 17 00 00 00     	leaq	0x17(,%rdx,8), %rax
140929891: 48 83 e0 f0                 	andq	$-0x10, %rax
140929895: 48 01 c2                    	addq	%rax, %rdx
140929898: 48 83 c2 11                 	addq	$0x11, %rdx
14092989c: 74 12                       	je	0x1409298b0 <.text+0x9288b0>
14092989e: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
1409298a2: 48 29 c1                    	subq	%rax, %rcx
1409298a5: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1409298ab: e8 70 a3 ce ff              	callq	0x140613c20 <.text+0x612c20>
1409298b0: 80 7d 77 00                 	cmpb	$0x0, 0x77(%rbp)
1409298b4: 74 3a                       	je	0x1409298f0 <.text+0x9288f0>
1409298b6: e8 75 e8 2e 00              	callq	0x140c18130 <.text+0xc17130>
1409298bb: 48 89 c1                    	movq	%rax, %rcx
1409298be: 48 8b 00                    	movq	(%rax), %rax
1409298c1: 48 83 f8 01                 	cmpq	$0x1, %rax
1409298c5: 75 5e                       	jne	0x140929925 <.text+0x928925>
1409298c7: 48 83 c1 08                 	addq	$0x8, %rcx
1409298cb: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409298cf: 75 74                       	jne	0x140929945 <.text+0x928945>
1409298d1: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409298d8: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
1409298dc: 48 83 c1 08                 	addq	$0x8, %rcx
1409298e0: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
1409298e4: e8 07 d8 2e 00              	callq	0x140c170f0 <.text+0xc160f0>
1409298e9: 48 8b 45 60                 	movq	0x60(%rbp), %rax
1409298ed: 48 ff 00                    	incq	(%rax)
1409298f0: 0f 28 b5 80 00 00 00        	movaps	0x80(%rbp), %xmm6
1409298f7: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
1409298fe: 5b                          	popq	%rbx
1409298ff: 5f                          	popq	%rdi
140929900: 5e                          	popq	%rsi
140929901: 41 5c                       	popq	%r12
140929903: 41 5d                       	popq	%r13
140929905: 41 5e                       	popq	%r14
140929907: 41 5f                       	popq	%r15
140929909: 5d                          	popq	%rbp
14092990a: c3                          	retq
14092990b: 48 85 c0                    	testq	%rax, %rax
14092990e: 75 29                       	jne	0x140929939 <.text+0x928939>
140929910: 31 d2                       	xorl	%edx, %edx
140929912: e8 b9 ee fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140929917: 48 89 c1                    	movq	%rax, %rcx
14092991a: 48 85 c0                    	testq	%rax, %rax
14092991d: 0f 85 65 fa ff ff           	jne	0x140929388 <.text+0x928388>
140929923: eb 14                       	jmp	0x140929939 <.text+0x928939>
140929925: 48 85 c0                    	testq	%rax, %rax
140929928: 75 0f                       	jne	0x140929939 <.text+0x928939>
14092992a: 31 d2                       	xorl	%edx, %edx
14092992c: e8 9f ee fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140929931: 48 89 c1                    	movq	%rax, %rcx
140929934: 48 85 c0                    	testq	%rax, %rax
140929937: 75 92                       	jne	0x1409298cb <.text+0x9288cb>
140929939: 48 8d 0d 60 11 31 02        	leaq	0x2311160(%rip), %rcx   # 0x142c3aaa0
140929940: e8 8b e7 fa 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140929945: 48 8d 0d 24 12 31 02        	leaq	0x2311224(%rip), %rcx   # 0x142c3ab70
14092994c: e8 ff f8 fa 01              	callq	0x1428d9250 <.text+0x28d8250>
140929951: 48 8d 0d 48 11 31 02        	leaq	0x2311148(%rip), %rcx   # 0x142c3aaa0
140929958: e8 73 e7 fa 01              	callq	0x1428d80d0 <.text+0x28d70d0>
14092995d: 0f 0b                       	ud2
14092995f: 90                          	nop
140929960: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140929965: 55                          	pushq	%rbp
140929966: 41 57                       	pushq	%r15
140929968: 41 56                       	pushq	%r14
14092996a: 41 55                       	pushq	%r13
14092996c: 41 54                       	pushq	%r12
14092996e: 56                          	pushq	%rsi
14092996f: 57                          	pushq	%rdi
140929970: 53                          	pushq	%rbx
140929971: 48 83 ec 48                 	subq	$0x48, %rsp
140929975: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14092997c: 66 0f 7f 74 24 30           	movdqa	%xmm6, 0x30(%rsp)
140929982: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140929986: 48 ff 00                    	incq	(%rax)
140929989: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14092998e: 48 83 c4 48                 	addq	$0x48, %rsp
140929992: 5b                          	popq	%rbx
140929993: 5f                          	popq	%rdi
140929994: 5e                          	popq	%rsi
140929995: 41 5c                       	popq	%r12
140929997: 41 5d                       	popq	%r13
140929999: 41 5e                       	popq	%r14
14092999b: 41 5f                       	popq	%r15
14092999d: 5d                          	popq	%rbp
14092999e: c3                          	retq
14092999f: 90                          	nop
1409299a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409299a5: 55                          	pushq	%rbp
1409299a6: 41 57                       	pushq	%r15
1409299a8: 41 56                       	pushq	%r14
1409299aa: 41 55                       	pushq	%r13
1409299ac: 41 54                       	pushq	%r12
1409299ae: 56                          	pushq	%rsi
1409299af: 57                          	pushq	%rdi
1409299b0: 53                          	pushq	%rbx
1409299b1: 48 83 ec 48                 	subq	$0x48, %rsp
1409299b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409299bc: 66 0f 7f 74 24 30           	movdqa	%xmm6, 0x30(%rsp)
1409299c2: 48 8b 45 60                 	movq	0x60(%rbp), %rax
1409299c6: 48 ff 00                    	incq	(%rax)
1409299c9: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1409299ce: 48 83 c4 48                 	addq	$0x48, %rsp
1409299d2: 5b                          	popq	%rbx
1409299d3: 5f                          	popq	%rdi
1409299d4: 5e                          	popq	%rsi
1409299d5: 41 5c                       	popq	%r12
1409299d7: 41 5d                       	popq	%r13
1409299d9: 41 5e                       	popq	%r14
1409299db: 41 5f                       	popq	%r15
1409299dd: 5d                          	popq	%rbp
1409299de: c3                          	retq
1409299df: 90                          	nop
1409299e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409299e5: 55                          	pushq	%rbp
1409299e6: 41 57                       	pushq	%r15
1409299e8: 41 56                       	pushq	%r14
1409299ea: 41 55                       	pushq	%r13
1409299ec: 41 54                       	pushq	%r12
1409299ee: 56                          	pushq	%rsi
1409299ef: 57                          	pushq	%rdi
1409299f0: 53                          	pushq	%rbx
1409299f1: 48 83 ec 48                 	subq	$0x48, %rsp
1409299f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409299fc: 66 0f 7f 74 24 30           	movdqa	%xmm6, 0x30(%rsp)
140929a02: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140929a06: e8 45 05 82 ff              	callq	0x140149f50 <.text+0x148f50>
140929a0b: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
140929a10: 48 83 c4 48                 	addq	$0x48, %rsp
140929a14: 5b                          	popq	%rbx
140929a15: 5f                          	popq	%rdi
140929a16: 5e                          	popq	%rsi
140929a17: 41 5c                       	popq	%r12
140929a19: 41 5d                       	popq	%r13
140929a1b: 41 5e                       	popq	%r14
140929a1d: 41 5f                       	popq	%r15
140929a1f: 5d                          	popq	%rbp
140929a20: c3                          	retq
140929a21: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140929a30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140929a35: 55                          	pushq	%rbp
140929a36: 41 57                       	pushq	%r15
140929a38: 41 56                       	pushq	%r14
140929a3a: 41 55                       	pushq	%r13
140929a3c: 41 54                       	pushq	%r12
140929a3e: 56                          	pushq	%rsi
140929a3f: 57                          	pushq	%rdi
140929a40: 53                          	pushq	%rbx
140929a41: 48 83 ec 48                 	subq	$0x48, %rsp
140929a45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140929a4c: 66 0f 7f 74 24 30           	movdqa	%xmm6, 0x30(%rsp)
140929a52: 80 7d 77 00                 	cmpb	$0x0, 0x77(%rbp)
140929a56: 74 20                       	je	0x140929a78 <.text+0x928a78>
140929a58: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
140929a5c: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140929a60: 48 8d 45 18                 	leaq	0x18(%rbp), %rax
140929a64: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140929a68: 48 8d 0d 49 4c 18 02        	leaq	0x2184c49(%rip), %rcx   # 0x142aae6b8
140929a6f: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
140929a73: e8 58 58 a0 00              	callq	0x14132f2d0 <.text+0x132e2d0>
140929a78: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
140929a7d: 48 83 c4 48                 	addq	$0x48, %rsp
140929a81: 5b                          	popq	%rbx
140929a82: 5f                          	popq	%rdi
140929a83: 5e                          	popq	%rsi
140929a84: 41 5c                       	popq	%r12
140929a86: 41 5d                       	popq	%r13
140929a88: 41 5e                       	popq	%r14
140929a8a: 41 5f                       	popq	%r15
140929a8c: 5d                          	popq	%rbp
140929a8d: c3                          	retq
