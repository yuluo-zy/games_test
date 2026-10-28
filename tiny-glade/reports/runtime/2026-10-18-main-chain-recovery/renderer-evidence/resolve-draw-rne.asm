
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409d00a0: 55                          	pushq	%rbp
1409d00a1: 41 57                       	pushq	%r15
1409d00a3: 41 56                       	pushq	%r14
1409d00a5: 41 55                       	pushq	%r13
1409d00a7: 41 54                       	pushq	%r12
1409d00a9: 56                          	pushq	%rsi
1409d00aa: 57                          	pushq	%rdi
1409d00ab: 53                          	pushq	%rbx
1409d00ac: 48 81 ec 68 0b 00 00        	subq	$0xb68, %rsp            # imm = 0xB68
1409d00b3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409d00bb: 0f 29 b5 d0 0a 00 00        	movaps	%xmm6, 0xad0(%rbp)
1409d00c2: 48 c7 85 c8 0a 00 00 fe ff ff ff    	movq	$-0x2, 0xac8(%rbp)
1409d00cd: 4c 89 ce                    	movq	%r9, %rsi
1409d00d0: 4c 89 c7                    	movq	%r8, %rdi
1409d00d3: 48 89 d3                    	movq	%rdx, %rbx
1409d00d6: 49 89 ce                    	movq	%rcx, %r14
1409d00d9: e8 22 69 24 00              	callq	0x140c16a00 <.text+0xc15a00>
1409d00de: 84 c0                       	testb	%al, %al
1409d00e0: 88 85 b8 0a 00 00           	movb	%al, 0xab8(%rbp)
1409d00e6: 74 60                       	je	0x1409d0148 <.text+0x9cf148>
1409d00e8: 4c 8d 25 dc 12 0f 02        	leaq	0x20f12dc(%rip), %r12   # 0x142ac13cb
1409d00ef: 41 bd 39 00 00 00           	movl	$0x39, %r13d
1409d00f5: ba 39 00 00 00              	movl	$0x39, %edx
1409d00fa: 4c 89 e1                    	movq	%r12, %rcx
1409d00fd: e8 fe d6 ff ff              	callq	0x1409cd800 <.text+0x9cc800>
1409d0102: 48 83 f8 01                 	cmpq	$0x1, %rax
1409d0106: 0f 85 b2 00 00 00           	jne	0x1409d01be <.text+0x9cf1be>
1409d010c: 49 89 d1                    	movq	%rdx, %r9
1409d010f: 48 85 d2                    	testq	%rdx, %rdx
1409d0112: 74 3d                       	je	0x1409d0151 <.text+0x9cf151>
1409d0114: 49 83 f9 39                 	cmpq	$0x39, %r9
1409d0118: 73 35                       	jae	0x1409d014f <.text+0x9cf14f>
1409d011a: 48 8d 05 aa 12 0f 02        	leaq	0x20f12aa(%rip), %rax   # 0x142ac13cb
1409d0121: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1409d0126: 7f 29                       	jg	0x1409d0151 <.text+0x9cf151>
1409d0128: 48 8d 05 f1 13 0f 02        	leaq	0x20f13f1(%rip), %rax   # 0x142ac1520
1409d012f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d0134: 48 8d 0d 90 12 0f 02        	leaq	0x20f1290(%rip), %rcx   # 0x142ac13cb
1409d013b: ba 39 00 00 00              	movl	$0x39, %edx
1409d0140: 45 31 c0                    	xorl	%r8d, %r8d
1409d0143: e8 f8 97 f0 01              	callq	0x1428d9940 <.text+0x28d8940>
1409d0148: 31 c9                       	xorl	%ecx, %ecx
1409d014a: e9 e7 01 00 00              	jmp	0x1409d0336 <.text+0x9cf336>
1409d014f: 75 d7                       	jne	0x1409d0128 <.text+0x9cf128>
1409d0151: 4c 8d 25 73 12 0f 02        	leaq	0x20f1273(%rip), %r12   # 0x142ac13cb
1409d0158: 4c 89 e1                    	movq	%r12, %rcx
1409d015b: 4c 89 ca                    	movq	%r9, %rdx
1409d015e: e8 9d d6 ff ff              	callq	0x1409cd800 <.text+0x9cc800>
1409d0163: 48 83 f8 01                 	cmpq	$0x1, %rax
1409d0167: 75 55                       	jne	0x1409d01be <.text+0x9cf1be>
1409d0169: 49 89 d0                    	movq	%rdx, %r8
1409d016c: 49 83 c0 02                 	addq	$0x2, %r8
1409d0170: 74 39                       	je	0x1409d01ab <.text+0x9cf1ab>
1409d0172: 49 83 f8 39                 	cmpq	$0x39, %r8
1409d0176: 73 31                       	jae	0x1409d01a9 <.text+0x9cf1a9>
1409d0178: 48 8d 05 4c 12 0f 02        	leaq	0x20f124c(%rip), %rax   # 0x142ac13cb
1409d017f: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1409d0184: 7f 25                       	jg	0x1409d01ab <.text+0x9cf1ab>
1409d0186: 48 8d 05 ab 13 0f 02        	leaq	0x20f13ab(%rip), %rax   # 0x142ac1538
1409d018d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d0192: 48 8d 0d 32 12 0f 02        	leaq	0x20f1232(%rip), %rcx   # 0x142ac13cb
1409d0199: ba 39 00 00 00              	movl	$0x39, %edx
1409d019e: 41 b9 39 00 00 00           	movl	$0x39, %r9d
1409d01a4: e8 97 97 f0 01              	callq	0x1428d9940 <.text+0x28d8940>
1409d01a9: 75 db                       	jne	0x1409d0186 <.text+0x9cf186>
1409d01ab: 41 bd 37 00 00 00           	movl	$0x37, %r13d
1409d01b1: 49 29 d5                    	subq	%rdx, %r13
1409d01b4: 4c 8d 25 10 12 0f 02        	leaq	0x20f1210(%rip), %r12   # 0x142ac13cb
1409d01bb: 4d 01 c4                    	addq	%r8, %r12
1409d01be: 4c 89 ad a8 0a 00 00        	movq	%r13, 0xaa8(%rbp)
1409d01c5: 48 8d 05 2e 14 0f 02        	leaq	0x20f142e(%rip), %rax   # 0x142ac15fa
1409d01cc: 4c 8d 2d f8 13 0f 02        	leaq	0x20f13f8(%rip), %r13   # 0x142ac15cb
1409d01d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1409d01e0: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
1409d01e4: 85 c9                       	testl	%ecx, %ecx
1409d01e6: 78 18                       	js	0x1409d0200 <.text+0x9cf200>
1409d01e8: 48 ff c8                    	decq	%rax
1409d01eb: 83 f9 5c                    	cmpl	$0x5c, %ecx
1409d01ee: 75 69                       	jne	0x1409d0259 <.text+0x9cf259>
1409d01f0: eb 7d                       	jmp	0x1409d026f <.text+0x9cf26f>
1409d01f2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409d0200: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
1409d0204: 80 fa c0                    	cmpb	$-0x40, %dl
1409d0207: 7d 25                       	jge	0x1409d022e <.text+0x9cf22e>
1409d0209: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
1409d020e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
1409d0212: 7d 23                       	jge	0x1409d0237 <.text+0x9cf237>
1409d0214: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
1409d0219: 48 83 c0 fc                 	addq	$-0x4, %rax
1409d021d: 41 83 e1 07                 	andl	$0x7, %r9d
1409d0221: 41 c1 e1 06                 	shll	$0x6, %r9d
1409d0225: 41 83 e0 3f                 	andl	$0x3f, %r8d
1409d0229: 45 09 c8                    	orl	%r9d, %r8d
1409d022c: eb 11                       	jmp	0x1409d023f <.text+0x9cf23f>
1409d022e: 48 83 c0 fe                 	addq	$-0x2, %rax
1409d0232: 83 e2 1f                    	andl	$0x1f, %edx
1409d0235: eb 12                       	jmp	0x1409d0249 <.text+0x9cf249>
1409d0237: 48 83 c0 fd                 	addq	$-0x3, %rax
1409d023b: 41 83 e0 0f                 	andl	$0xf, %r8d
1409d023f: 41 c1 e0 06                 	shll	$0x6, %r8d
1409d0243: 83 e2 3f                    	andl	$0x3f, %edx
1409d0246: 44 09 c2                    	orl	%r8d, %edx
1409d0249: c1 e2 06                    	shll	$0x6, %edx
1409d024c: 80 e1 3f                    	andb	$0x3f, %cl
1409d024f: 0f b6 c9                    	movzbl	%cl, %ecx
1409d0252: 09 d1                       	orl	%edx, %ecx
1409d0254: 83 f9 5c                    	cmpl	$0x5c, %ecx
1409d0257: 74 16                       	je	0x1409d026f <.text+0x9cf26f>
1409d0259: 83 f9 2f                    	cmpl	$0x2f, %ecx
1409d025c: 74 11                       	je	0x1409d026f <.text+0x9cf26f>
1409d025e: 4c 39 e8                    	cmpq	%r13, %rax
1409d0261: 0f 85 79 ff ff ff           	jne	0x1409d01e0 <.text+0x9cf1e0>
1409d0267: 41 bf 2f 00 00 00           	movl	$0x2f, %r15d
1409d026d: eb 50                       	jmp	0x1409d02bf <.text+0x9cf2bf>
1409d026f: 4c 8d 2d 55 13 0f 02        	leaq	0x20f1355(%rip), %r13   # 0x142ac15cb
1409d0276: 4c 29 e8                    	subq	%r13, %rax
1409d0279: 49 89 c0                    	movq	%rax, %r8
1409d027c: 49 ff c0                    	incq	%r8
1409d027f: 74 32                       	je	0x1409d02b3 <.text+0x9cf2b3>
1409d0281: 49 83 f8 2f                 	cmpq	$0x2f, %r8
1409d0285: 73 2a                       	jae	0x1409d02b1 <.text+0x9cf2b1>
1409d0287: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1409d028c: 7f 25                       	jg	0x1409d02b3 <.text+0x9cf2b3>
1409d028e: 48 8d 05 6b 12 0f 02        	leaq	0x20f126b(%rip), %rax   # 0x142ac1500
1409d0295: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d029a: 48 8d 0d 2a 13 0f 02        	leaq	0x20f132a(%rip), %rcx   # 0x142ac15cb
1409d02a1: ba 2f 00 00 00              	movl	$0x2f, %edx
1409d02a6: 41 b9 2f 00 00 00           	movl	$0x2f, %r9d
1409d02ac: e8 8f 96 f0 01              	callq	0x1428d9940 <.text+0x28d8940>
1409d02b1: 75 db                       	jne	0x1409d028e <.text+0x9cf28e>
1409d02b3: 41 bf 2e 00 00 00           	movl	$0x2e, %r15d
1409d02b9: 49 29 c7                    	subq	%rax, %r15
1409d02bc: 4d 01 c5                    	addq	%r8, %r13
1409d02bf: e8 6c 7e 24 00              	callq	0x140c18130 <.text+0xc17130>
1409d02c4: 48 89 c1                    	movq	%rax, %rcx
1409d02c7: 48 8b 00                    	movq	(%rax), %rax
1409d02ca: 48 83 f8 01                 	cmpq	$0x1, %rax
1409d02ce: 0f 85 68 03 00 00           	jne	0x1409d063c <.text+0x9cf63c>
1409d02d4: 48 83 c1 08                 	addq	$0x8, %rcx
1409d02d8: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409d02dc: 0f 85 94 03 00 00           	jne	0x1409d0676 <.text+0x9cf676>
1409d02e2: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409d02e9: 48 89 8d b0 0a 00 00        	movq	%rcx, 0xab0(%rbp)
1409d02f0: 48 83 c1 08                 	addq	$0x8, %rcx
1409d02f4: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1409d02f9: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1409d0302: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1409d030b: 4c 89 e2                    	movq	%r12, %rdx
1409d030e: 4c 8b 85 a8 0a 00 00        	movq	0xaa8(%rbp), %r8
1409d0315: 4d 89 e9                    	movq	%r13, %r9
1409d0318: e8 23 6a 24 00              	callq	0x140c16d40 <.text+0xc15d40>
1409d031d: 49 89 c7                    	movq	%rax, %r15
1409d0320: 48 8b 85 b0 0a 00 00        	movq	0xab0(%rbp), %rax
1409d0327: 48 ff 00                    	incq	(%rax)
1409d032a: 4c 89 bd 88 0a 00 00        	movq	%r15, 0xa88(%rbp)
1409d0331: b9 01 00 00 00              	movl	$0x1, %ecx
1409d0336: 48 8b 85 58 0b 00 00        	movq	0xb58(%rbp), %rax
1409d033d: 48 89 8d 80 0a 00 00        	movq	%rcx, 0xa80(%rbp)
1409d0344: 4c 8b 28                    	movq	(%rax), %r13
1409d0347: 4c 8b 60 18                 	movq	0x18(%rax), %r12
1409d034b: 4c 89 e1                    	movq	%r12, %rcx
1409d034e: 41 83 fd 01                 	cmpl	$0x1, %r13d
1409d0352: 74 04                       	je	0x1409d0358 <.text+0x9cf358>
1409d0354: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1409d0358: 0f 10 70 08                 	movups	0x8(%rax), %xmm6
1409d035c: 48 83 c1 04                 	addq	$0x4, %rcx
1409d0360: e8 2b 54 20 00              	callq	0x140bd5790 <.text+0xbd4790>
1409d0365: 66 49 0f 6e c5              	movq	%r13, %xmm0
1409d036a: 66 0f d6 85 a0 05 00 00     	movq	%xmm0, 0x5a0(%rbp)
1409d0372: 0f 11 b5 a8 05 00 00        	movups	%xmm6, 0x5a8(%rbp)
1409d0379: 4c 89 a5 b8 05 00 00        	movq	%r12, 0x5b8(%rbp)
1409d0380: c7 85 88 09 00 00 02 00 00 00       	movl	$0x2, 0x988(%rbp)
1409d038a: 4c 89 74 24 40              	movq	%r14, 0x40(%rsp)
1409d038f: 48 89 5c 24 38              	movq	%rbx, 0x38(%rsp)
1409d0394: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1409d0399: 4c 8d b5 88 09 00 00        	leaq	0x988(%rbp), %r14
1409d03a0: 4c 89 74 24 28              	movq	%r14, 0x28(%rsp)
1409d03a5: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409d03ae: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1409d03b2: 4c 8d 8d a0 05 00 00        	leaq	0x5a0(%rbp), %r9
1409d03b9: 48 89 fa                    	movq	%rdi, %rdx
1409d03bc: 49 89 f0                    	movq	%rsi, %r8
1409d03bf: e8 ac 75 e9 ff              	callq	0x140867970 <.text+0x866970>
1409d03c4: 48 8b 85 60 0b 00 00        	movq	0xb60(%rbp), %rax
1409d03cb: 4c 8b 20                    	movq	(%rax), %r12
1409d03ce: 48 8b 78 08                 	movq	0x8(%rax), %rdi
1409d03d2: 0f b6 70 10                 	movzbl	0x10(%rax), %esi
1409d03d6: c6 85 c7 0a 00 00 01        	movb	$0x1, 0xac7(%rbp)
1409d03dd: e8 6e 63 21 00              	callq	0x140be6750 <.text+0xbe5750>
1409d03e2: 89 c3                       	movl	%eax, %ebx
1409d03e4: c6 85 c7 0a 00 00 01        	movb	$0x1, 0xac7(%rbp)
1409d03eb: 31 c9                       	xorl	%ecx, %ecx
1409d03ed: e8 2e 97 70 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1409d03f2: 48 85 c0                    	testq	%rax, %rax
1409d03f5: 0f 84 87 02 00 00           	je	0x1409d0682 <.text+0x9cf682>
1409d03fb: 48 8b 8d 50 0b 00 00        	movq	0xb50(%rbp), %rcx
1409d0402: 89 9d 48 06 00 00           	movl	%ebx, 0x648(%rbp)
1409d0408: 66 c7 85 50 06 00 00 00 00  	movw	$0x0, 0x650(%rbp)
1409d0411: c7 85 4c 06 00 00 00 00 80 3f       	movl	$0x3f800000, 0x64c(%rbp) # imm = 0x3F800000
1409d041b: 48 c7 85 a0 05 00 00 00 00 00 00    	movq	$0x0, 0x5a0(%rbp)
1409d0426: 48 89 8d 40 06 00 00        	movq	%rcx, 0x640(%rbp)
1409d042d: 48 c7 85 b0 05 00 00 00 00 00 00    	movq	$0x0, 0x5b0(%rbp)
1409d0438: 48 c7 85 b8 05 00 00 04 00 00 00    	movq	$0x4, 0x5b8(%rbp)
1409d0443: 48 c7 85 c0 05 00 00 00 00 00 00    	movq	$0x0, 0x5c0(%rbp)
1409d044e: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1409d0452: 48 ff 00                    	incq	(%rax)
1409d0455: 0f 10 0d d4 62 0c 02        	movups	0x20c62d4(%rip), %xmm1  # 0x142a96730
1409d045c: 0f 11 8d 20 06 00 00        	movups	%xmm1, 0x620(%rbp)
1409d0463: 0f 10 0d b6 62 0c 02        	movups	0x20c62b6(%rip), %xmm1  # 0x142a96720
1409d046a: 0f 11 8d 10 06 00 00        	movups	%xmm1, 0x610(%rbp)
1409d0471: f3 0f 7f 85 30 06 00 00     	movdqu	%xmm0, 0x630(%rbp)
1409d0479: 48 b8 03 00 00 00 01 00 00 00       	movabsq	$0x100000003, %rax # imm = 0x100000003
1409d0483: 48 89 85 e0 05 00 00        	movq	%rax, 0x5e0(%rbp)
1409d048a: 48 c7 85 c8 05 00 00 00 00 00 00    	movq	$0x0, 0x5c8(%rbp)
1409d0495: 48 c7 85 d0 05 00 00 01 00 00 00    	movq	$0x1, 0x5d0(%rbp)
1409d04a0: 48 c7 85 d8 05 00 00 00 00 00 00    	movq	$0x0, 0x5d8(%rbp)
1409d04ab: c6 85 c7 0a 00 00 01        	movb	$0x1, 0xac7(%rbp)
1409d04b2: 48 8d 8d a8 04 00 00        	leaq	0x4a8(%rbp), %rcx
1409d04b9: 48 8d 95 a0 05 00 00        	leaq	0x5a0(%rbp), %rdx
1409d04c0: 4c 8d 45 c8                 	leaq	-0x38(%rbp), %r8
1409d04c4: 45 31 c9                    	xorl	%r9d, %r9d
1409d04c7: e8 14 57 f3 ff              	callq	0x140905be0 <.text+0x904be0>
1409d04cc: 48 8d 9d b0 03 00 00        	leaq	0x3b0(%rbp), %rbx
1409d04d3: 48 8d 95 a8 04 00 00        	leaq	0x4a8(%rbp), %rdx
1409d04da: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409d04e0: 48 89 d9                    	movq	%rbx, %rcx
1409d04e3: e8 18 37 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409d04e8: c6 85 c7 0a 00 00 01        	movb	$0x1, 0xac7(%rbp)
1409d04ef: 48 8d 8d a0 05 00 00        	leaq	0x5a0(%rbp), %rcx
1409d04f6: 49 b8 71 49 ea 8e 31 b9 1b 53       	movabsq	$0x531bb9318eea4971, %r8 # imm = 0x531BB9318EEA4971
1409d0500: 48 89 da                    	movq	%rbx, %rdx
1409d0503: 49 89 f9                    	movq	%rdi, %r9
1409d0506: e8 15 98 c5 00              	callq	0x141629d20 <.text+0x1628d20>
1409d050b: 4c 89 a5 90 0a 00 00        	movq	%r12, 0xa90(%rbp)
1409d0512: c7 85 88 09 00 00 02 00 00 00       	movl	$0x2, 0x988(%rbp)
1409d051c: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
1409d0521: 48 8d 8d 90 0a 00 00        	leaq	0xa90(%rbp), %rcx
1409d0528: 48 8d 95 a0 05 00 00        	leaq	0x5a0(%rbp), %rdx
1409d052f: 49 b8 dd b3 40 cc 97 56 ba 11       	movabsq	$0x11ba5697cc40b3dd, %r8 # imm = 0x11BA5697CC40B3DD
1409d0539: 41 b1 1d                    	movb	$0x1d, %r9b
1409d053c: e8 bf e2 f1 ff              	callq	0x1408ee800 <.text+0x8ed800>
1409d0541: 48 8d 8d 88 09 00 00        	leaq	0x988(%rbp), %rcx
1409d0548: 48 8d 95 a0 05 00 00        	leaq	0x5a0(%rbp), %rdx
1409d054f: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409d0555: e8 a6 36 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409d055a: 80 bd 66 0a 00 00 08        	cmpb	$0x8, 0xa66(%rbp)
1409d0561: 74 25                       	je	0x1409d0588 <.text+0x9cf588>
1409d0563: 48 8d 8d 40 0a 00 00        	leaq	0xa40(%rbp), %rcx
1409d056a: ba e8 00 00 00              	movl	$0xe8, %edx
1409d056f: 48 03 95 70 0a 00 00        	addq	0xa70(%rbp), %rdx
1409d0576: 49 b8 49 88 2d 38 bb c6 01 7b       	movabsq	$0x7b01c6bb382d8849, %r8 # imm = 0x7B01C6BB382D8849
1409d0580: 41 89 f1                    	movl	%esi, %r9d
1409d0583: e8 e8 af f9 ff              	callq	0x14096b570 <.text+0x96a570>
1409d0588: 48 8d b5 a0 05 00 00        	leaq	0x5a0(%rbp), %rsi
1409d058f: 48 8d 95 88 09 00 00        	leaq	0x988(%rbp), %rdx
1409d0596: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409d059c: 48 89 f1                    	movq	%rsi, %rcx
1409d059f: e8 5c 36 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409d05a4: c6 85 c7 0a 00 00 01        	movb	$0x1, 0xac7(%rbp)
1409d05ab: ba 03 00 00 00              	movl	$0x3, %edx
1409d05b0: 48 89 f1                    	movq	%rsi, %rcx
1409d05b3: e8 d8 64 f3 ff              	callq	0x140906a90 <.text+0x905a90>
1409d05b8: 48 8d 55 c8                 	leaq	-0x38(%rbp), %rdx
1409d05bc: 41 b8 e8 03 00 00           	movl	$0x3e8, %r8d            # imm = 0x3E8
1409d05c2: 48 89 f1                    	movq	%rsi, %rcx
1409d05c5: e8 36 36 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409d05ca: c6 85 c7 0a 00 00 00        	movb	$0x0, 0xac7(%rbp)
1409d05d1: 48 89 f1                    	movq	%rsi, %rcx
1409d05d4: e8 97 7d e9 ff              	callq	0x140868370 <.text+0x867370>
1409d05d9: 80 bd b8 0a 00 00 00        	cmpb	$0x0, 0xab8(%rbp)
1409d05e0: 74 3f                       	je	0x1409d0621 <.text+0x9cf621>
1409d05e2: e8 49 7b 24 00              	callq	0x140c18130 <.text+0xc17130>
1409d05e7: 48 89 c1                    	movq	%rax, %rcx
1409d05ea: 48 8b 00                    	movq	(%rax), %rax
1409d05ed: 48 83 f8 01                 	cmpq	$0x1, %rax
1409d05f1: 75 63                       	jne	0x1409d0656 <.text+0x9cf656>
1409d05f3: 48 83 c1 08                 	addq	$0x8, %rcx
1409d05f7: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409d05fb: 75 79                       	jne	0x1409d0676 <.text+0x9cf676>
1409d05fd: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409d0604: 48 89 8d b8 0a 00 00        	movq	%rcx, 0xab8(%rbp)
1409d060b: 48 83 c1 08                 	addq	$0x8, %rcx
1409d060f: 4c 89 fa                    	movq	%r15, %rdx
1409d0612: e8 d9 6a 24 00              	callq	0x140c170f0 <.text+0xc160f0>
1409d0617: 48 8b 85 b8 0a 00 00        	movq	0xab8(%rbp), %rax
1409d061e: 48 ff 00                    	incq	(%rax)
1409d0621: 0f 28 b5 d0 0a 00 00        	movaps	0xad0(%rbp), %xmm6
1409d0628: 48 81 c4 68 0b 00 00        	addq	$0xb68, %rsp            # imm = 0xB68
1409d062f: 5b                          	popq	%rbx
1409d0630: 5f                          	popq	%rdi
1409d0631: 5e                          	popq	%rsi
1409d0632: 41 5c                       	popq	%r12
1409d0634: 41 5d                       	popq	%r13
1409d0636: 41 5e                       	popq	%r14
1409d0638: 41 5f                       	popq	%r15
1409d063a: 5d                          	popq	%rbp
1409d063b: c3                          	retq
1409d063c: 48 85 c0                    	testq	%rax, %rax
1409d063f: 75 29                       	jne	0x1409d066a <.text+0x9cf66a>
1409d0641: 31 d2                       	xorl	%edx, %edx
1409d0643: e8 88 81 f2 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409d0648: 48 89 c1                    	movq	%rax, %rcx
1409d064b: 48 85 c0                    	testq	%rax, %rax
1409d064e: 0f 85 84 fc ff ff           	jne	0x1409d02d8 <.text+0x9cf2d8>
1409d0654: eb 14                       	jmp	0x1409d066a <.text+0x9cf66a>
1409d0656: 48 85 c0                    	testq	%rax, %rax
1409d0659: 75 0f                       	jne	0x1409d066a <.text+0x9cf66a>
1409d065b: 31 d2                       	xorl	%edx, %edx
1409d065d: e8 6e 81 f2 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409d0662: 48 89 c1                    	movq	%rax, %rcx
1409d0665: 48 85 c0                    	testq	%rax, %rax
1409d0668: 75 8d                       	jne	0x1409d05f7 <.text+0x9cf5f7>
1409d066a: 48 8d 0d 2f a4 26 02        	leaq	0x226a42f(%rip), %rcx   # 0x142c3aaa0
1409d0671: e8 5a 7a f0 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409d0676: 48 8d 0d f3 a4 26 02        	leaq	0x226a4f3(%rip), %rcx   # 0x142c3ab70
1409d067d: e8 ce 8b f0 01              	callq	0x1428d9250 <.text+0x28d8250>
1409d0682: c6 85 c7 0a 00 00 01        	movb	$0x1, 0xac7(%rbp)
