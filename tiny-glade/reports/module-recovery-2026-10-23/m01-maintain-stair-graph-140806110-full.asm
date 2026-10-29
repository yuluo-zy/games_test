
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140806110: 55                          	pushq	%rbp
140806111: 41 57                       	pushq	%r15
140806113: 41 56                       	pushq	%r14
140806115: 41 55                       	pushq	%r13
140806117: 41 54                       	pushq	%r12
140806119: 56                          	pushq	%rsi
14080611a: 57                          	pushq	%rdi
14080611b: 53                          	pushq	%rbx
14080611c: 48 81 ec e8 01 00 00        	subq	$0x1e8, %rsp            # imm = 0x1E8
140806123: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14080612b: 66 0f 7f b5 50 01 00 00     	movdqa	%xmm6, 0x150(%rbp)
140806133: 48 c7 85 48 01 00 00 fe ff ff ff    	movq	$-0x2, 0x148(%rbp)
14080613e: 4c 89 85 80 00 00 00        	movq	%r8, 0x80(%rbp)
140806145: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
140806149: 48 8b 05 48 5f e2 02        	movq	0x2e25f48(%rip), %rax   # 0x14362c098
140806150: 80 38 00                    	cmpb	$0x0, (%rax)
140806153: 0f 85 61 0b 00 00           	jne	0x140806cba <.text+0x805cba>
140806159: 4d 89 cc                    	movq	%r9, %r12
14080615c: 48 89 cb                    	movq	%rcx, %rbx
14080615f: e8 9c 08 41 00              	callq	0x140c16a00 <.text+0xc15a00>
140806164: 84 c0                       	testb	%al, %al
140806166: 74 60                       	je	0x1408061c8 <.text+0x8051c8>
140806168: 48 8d 35 cc 7a 28 02        	leaq	0x2287acc(%rip), %rsi   # 0x142a8dc3b
14080616f: 41 be 40 00 00 00           	movl	$0x40, %r14d
140806175: ba 40 00 00 00              	movl	$0x40, %edx
14080617a: 48 89 f1                    	movq	%rsi, %rcx
14080617d: e8 0e d2 ff ff              	callq	0x140803390 <.text+0x802390>
140806182: 48 83 f8 01                 	cmpq	$0x1, %rax
140806186: 0f 85 b2 00 00 00           	jne	0x14080623e <.text+0x80523e>
14080618c: 49 89 d1                    	movq	%rdx, %r9
14080618f: 48 85 d2                    	testq	%rdx, %rdx
140806192: 74 3d                       	je	0x1408061d1 <.text+0x8051d1>
140806194: 49 83 f9 40                 	cmpq	$0x40, %r9
140806198: 73 35                       	jae	0x1408061cf <.text+0x8051cf>
14080619a: 48 8d 05 9a 7a 28 02        	leaq	0x2287a9a(%rip), %rax   # 0x142a8dc3b
1408061a1: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1408061a6: 7f 29                       	jg	0x1408061d1 <.text+0x8051d1>
1408061a8: 48 8d 05 51 7b 28 02        	leaq	0x2287b51(%rip), %rax   # 0x142a8dd00
1408061af: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408061b4: 48 8d 0d 80 7a 28 02        	leaq	0x2287a80(%rip), %rcx   # 0x142a8dc3b
1408061bb: ba 40 00 00 00              	movl	$0x40, %edx
1408061c0: 45 31 c0                    	xorl	%r8d, %r8d
1408061c3: e8 78 37 0d 02              	callq	0x1428d9940 <.text+0x28d8940>
1408061c8: 31 c0                       	xorl	%eax, %eax
1408061ca: e9 c1 01 00 00              	jmp	0x140806390 <.text+0x805390>
1408061cf: 75 d7                       	jne	0x1408061a8 <.text+0x8051a8>
1408061d1: 48 8d 35 63 7a 28 02        	leaq	0x2287a63(%rip), %rsi   # 0x142a8dc3b
1408061d8: 48 89 f1                    	movq	%rsi, %rcx
1408061db: 4c 89 ca                    	movq	%r9, %rdx
1408061de: e8 ad d1 ff ff              	callq	0x140803390 <.text+0x802390>
1408061e3: 48 83 f8 01                 	cmpq	$0x1, %rax
1408061e7: 75 55                       	jne	0x14080623e <.text+0x80523e>
1408061e9: 49 89 d0                    	movq	%rdx, %r8
1408061ec: 49 83 c0 02                 	addq	$0x2, %r8
1408061f0: 74 39                       	je	0x14080622b <.text+0x80522b>
1408061f2: 49 83 f8 40                 	cmpq	$0x40, %r8
1408061f6: 73 31                       	jae	0x140806229 <.text+0x805229>
1408061f8: 48 8d 05 3c 7a 28 02        	leaq	0x2287a3c(%rip), %rax   # 0x142a8dc3b
1408061ff: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140806204: 7f 25                       	jg	0x14080622b <.text+0x80522b>
140806206: 48 8d 05 0b 7b 28 02        	leaq	0x2287b0b(%rip), %rax   # 0x142a8dd18
14080620d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140806212: 48 8d 0d 22 7a 28 02        	leaq	0x2287a22(%rip), %rcx   # 0x142a8dc3b
140806219: ba 40 00 00 00              	movl	$0x40, %edx
14080621e: 41 b9 40 00 00 00           	movl	$0x40, %r9d
140806224: e8 17 37 0d 02              	callq	0x1428d9940 <.text+0x28d8940>
140806229: 75 db                       	jne	0x140806206 <.text+0x805206>
14080622b: 41 be 3e 00 00 00           	movl	$0x3e, %r14d
140806231: 49 29 d6                    	subq	%rdx, %r14
140806234: 48 8d 35 00 7a 28 02        	leaq	0x2287a00(%rip), %rsi   # 0x142a8dc3b
14080623b: 4c 01 c6                    	addq	%r8, %rsi
14080623e: 4c 8d 05 3a 7b 28 02        	leaq	0x2287b3a(%rip), %r8    # 0x142a8dd7f
140806245: 48 8d 3d fc 7a 28 02        	leaq	0x2287afc(%rip), %rdi   # 0x142a8dd48
14080624c: 0f 1f 40 00                 	nopl	(%rax)
140806250: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
140806255: 85 c0                       	testl	%eax, %eax
140806257: 78 17                       	js	0x140806270 <.text+0x805270>
140806259: 49 ff c8                    	decq	%r8
14080625c: 83 f8 5c                    	cmpl	$0x5c, %eax
14080625f: 75 63                       	jne	0x1408062c4 <.text+0x8052c4>
140806261: eb 73                       	jmp	0x1408062d6 <.text+0x8052d6>
140806263: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140806270: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
140806275: 80 f9 c0                    	cmpb	$-0x40, %cl
140806278: 7d 23                       	jge	0x14080629d <.text+0x80529d>
14080627a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14080627f: 80 fa c0                    	cmpb	$-0x40, %dl
140806282: 7d 22                       	jge	0x1408062a6 <.text+0x8052a6>
140806284: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
140806289: 49 83 c0 fc                 	addq	$-0x4, %r8
14080628d: 41 83 e1 07                 	andl	$0x7, %r9d
140806291: 41 c1 e1 06                 	shll	$0x6, %r9d
140806295: 83 e2 3f                    	andl	$0x3f, %edx
140806298: 44 09 ca                    	orl	%r9d, %edx
14080629b: eb 10                       	jmp	0x1408062ad <.text+0x8052ad>
14080629d: 49 83 c0 fe                 	addq	$-0x2, %r8
1408062a1: 83 e1 1f                    	andl	$0x1f, %ecx
1408062a4: eb 0f                       	jmp	0x1408062b5 <.text+0x8052b5>
1408062a6: 49 83 c0 fd                 	addq	$-0x3, %r8
1408062aa: 83 e2 0f                    	andl	$0xf, %edx
1408062ad: c1 e2 06                    	shll	$0x6, %edx
1408062b0: 83 e1 3f                    	andl	$0x3f, %ecx
1408062b3: 09 d1                       	orl	%edx, %ecx
1408062b5: c1 e1 06                    	shll	$0x6, %ecx
1408062b8: 24 3f                       	andb	$0x3f, %al
1408062ba: 0f b6 c0                    	movzbl	%al, %eax
1408062bd: 09 c8                       	orl	%ecx, %eax
1408062bf: 83 f8 5c                    	cmpl	$0x5c, %eax
1408062c2: 74 12                       	je	0x1408062d6 <.text+0x8052d6>
1408062c4: 83 f8 2f                    	cmpl	$0x2f, %eax
1408062c7: 74 0d                       	je	0x1408062d6 <.text+0x8052d6>
1408062c9: 49 39 f8                    	cmpq	%rdi, %r8
1408062cc: 75 82                       	jne	0x140806250 <.text+0x805250>
1408062ce: 41 bf 37 00 00 00           	movl	$0x37, %r15d
1408062d4: eb 4d                       	jmp	0x140806323 <.text+0x805323>
1408062d6: 48 8d 3d 6b 7a 28 02        	leaq	0x2287a6b(%rip), %rdi   # 0x142a8dd48
1408062dd: 49 29 f8                    	subq	%rdi, %r8
1408062e0: 49 ff c0                    	incq	%r8
1408062e3: 74 32                       	je	0x140806317 <.text+0x805317>
1408062e5: 49 83 f8 37                 	cmpq	$0x37, %r8
1408062e9: 73 2a                       	jae	0x140806315 <.text+0x805315>
1408062eb: 41 80 3c 38 bf              	cmpb	$-0x41, (%r8,%rdi)
1408062f0: 7f 25                       	jg	0x140806317 <.text+0x805317>
1408062f2: 48 8d 05 e7 79 28 02        	leaq	0x22879e7(%rip), %rax   # 0x142a8dce0
1408062f9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408062fe: 48 8d 0d 43 7a 28 02        	leaq	0x2287a43(%rip), %rcx   # 0x142a8dd48
140806305: ba 37 00 00 00              	movl	$0x37, %edx
14080630a: 41 b9 37 00 00 00           	movl	$0x37, %r9d
140806310: e8 2b 36 0d 02              	callq	0x1428d9940 <.text+0x28d8940>
140806315: 75 db                       	jne	0x1408062f2 <.text+0x8052f2>
140806317: 41 bf 37 00 00 00           	movl	$0x37, %r15d
14080631d: 4d 29 c7                    	subq	%r8, %r15
140806320: 4c 01 c7                    	addq	%r8, %rdi
140806323: e8 08 1e 41 00              	callq	0x140c18130 <.text+0xc17130>
140806328: 48 89 c1                    	movq	%rax, %rcx
14080632b: 48 8b 00                    	movq	(%rax), %rax
14080632e: 48 83 f8 01                 	cmpq	$0x1, %rax
140806332: 0f 85 b3 09 00 00           	jne	0x140806ceb <.text+0x805ceb>
140806338: 48 83 c1 08                 	addq	$0x8, %rcx
14080633c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140806340: 0f 85 c9 09 00 00           	jne	0x140806d0f <.text+0x805d0f>
140806346: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14080634d: 48 89 8d 30 01 00 00        	movq	%rcx, 0x130(%rbp)
140806354: 48 83 c1 08                 	addq	$0x8, %rcx
140806358: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
14080635d: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140806366: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14080636f: 48 89 f2                    	movq	%rsi, %rdx
140806372: 4d 89 f0                    	movq	%r14, %r8
140806375: 49 89 f9                    	movq	%rdi, %r9
140806378: e8 c3 09 41 00              	callq	0x140c16d40 <.text+0xc15d40>
14080637d: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
140806384: 48 ff 01                    	incq	(%rcx)
140806387: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
14080638b: b8 01 00 00 00              	movl	$0x1, %eax
140806390: 4c 89 a5 88 00 00 00        	movq	%r12, 0x88(%rbp)
140806397: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14080639b: 48 8b 0b                    	movq	(%rbx), %rcx
14080639e: e8 cd 05 28 00              	callq	0x140a86970 <.text+0xa85970>
1408063a3: 88 85 20 01 00 00           	movb	%al, 0x120(%rbp)
1408063a9: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1408063b0: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
1408063bb: 48 c7 85 c0 00 00 00 04 00 00 00    	movq	$0x4, 0xc0(%rbp)
1408063c6: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
1408063d1: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
1408063dc: 48 c7 85 d8 00 00 00 04 00 00 00    	movq	$0x4, 0xd8(%rbp)
1408063e7: 48 c7 85 e0 00 00 00 00 00 00 00    	movq	$0x0, 0xe0(%rbp)
1408063f2: 4c 8b 20                    	movq	(%rax), %r12
1408063f5: 4c 89 e1                    	movq	%r12, %rcx
1408063f8: e8 b3 84 0f 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
1408063fd: 4c 8b 68 08                 	movq	0x8(%rax), %r13
140806401: 48 8b 40 10                 	movq	0x10(%rax), %rax
140806405: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
140806409: 4c 8d 34 c5 00 00 00 00     	leaq	(,%rax,8), %r14
140806411: 4d 01 ee                    	addq	%r13, %r14
140806414: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
14080641b: 48 8b 00                    	movq	(%rax), %rax
14080641e: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140806425: 48 8d 35 74 7a 28 02        	leaq	0x2287a74(%rip), %rsi   # 0x142a8dea0
14080642c: 48 8d bd d0 00 00 00        	leaq	0xd0(%rbp), %rdi
140806433: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140806437: eb 1c                       	jmp	0x140806455 <.text+0x805455>
140806439: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140806440: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
140806447: 44 89 3c 98                 	movl	%r15d, (%rax,%rbx,4)
14080644b: 48 ff c3                    	incq	%rbx
14080644e: 48 89 9d e0 00 00 00        	movq	%rbx, 0xe0(%rbp)
140806455: 4d 39 f5                    	cmpq	%r14, %r13
140806458: 0f 84 16 01 00 00           	je	0x140806574 <.text+0x805574>
14080645e: 45 8b 7d 20                 	movl	0x20(%r13), %r15d
140806462: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
140806469: 44 89 fa                    	movl	%r15d, %edx
14080646c: e8 9f 60 21 00              	callq	0x140a1c510 <.text+0xa1b510>
140806471: 49 83 c5 28                 	addq	$0x28, %r13
140806475: 48 83 f8 03                 	cmpq	$0x3, %rax
140806479: 0f 84 d1 00 00 00           	je	0x140806550 <.text+0x805550>
14080647f: 49 89 d0                    	movq	%rdx, %r8
140806482: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
140806489: 48 89 c2                    	movq	%rax, %rdx
14080648c: e8 2f 5e 21 00              	callq	0x140a1c2c0 <.text+0xa1b2c0>
140806491: 48 85 c0                    	testq	%rax, %rax
140806494: 0f 84 81 08 00 00           	je	0x140806d1b <.text+0x805d1b>
14080649a: 44 89 7d d0                 	movl	%r15d, -0x30(%rbp)
14080649e: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
1408064a3: 0f 84 a7 00 00 00           	je	0x140806550 <.text+0x805550>
1408064a9: 48 89 c1                    	movq	%rax, %rcx
1408064ac: 48 83 c1 50                 	addq	$0x50, %rcx
1408064b0: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1408064b4: 48 89 c3                    	movq	%rax, %rbx
1408064b7: e8 14 aa da ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1408064bc: 48 8b 4b 30                 	movq	0x30(%rbx), %rcx
1408064c0: 48 8b 53 38                 	movq	0x38(%rbx), %rdx
1408064c4: 49 89 d0                    	movq	%rdx, %r8
1408064c7: 49 21 c0                    	andq	%rax, %r8
1408064ca: 48 c1 e8 39                 	shrq	$0x39, %rax
1408064ce: 66 0f 6e c0                 	movd	%eax, %xmm0
1408064d2: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408064d6: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408064db: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408064e0: 48 8d 41 b0                 	leaq	-0x50(%rcx), %rax
1408064e4: 45 31 c9                    	xorl	%r9d, %r9d
1408064e7: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1408064ed: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1408064f1: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1408064f5: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1408064fa: 45 85 d2                    	testl	%r10d, %r10d
1408064fd: 74 31                       	je	0x140806530 <.text+0x805530>
1408064ff: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140806504: 4d 01 c3                    	addq	%r8, %r11
140806507: 49 21 d3                    	andq	%rdx, %r11
14080650a: 49 f7 db                    	negq	%r11
14080650d: 4f 8d 1c 9b                 	leaq	(%r11,%r11,4), %r11
140806511: 49 c1 e3 04                 	shlq	$0x4, %r11
140806515: 46 3b 3c 18                 	cmpl	(%rax,%r11), %r15d
140806519: 0f 84 36 ff ff ff           	je	0x140806455 <.text+0x805455>
14080651f: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140806523: 66 45 21 d3                 	andw	%r10w, %r11w
140806527: 45 89 da                    	movl	%r11d, %r10d
14080652a: 75 d3                       	jne	0x1408064ff <.text+0x8054ff>
14080652c: 0f 1f 40 00                 	nopl	(%rax)
140806530: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140806534: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140806539: 45 85 d2                    	testl	%r10d, %r10d
14080653c: 75 12                       	jne	0x140806550 <.text+0x805550>
14080653e: 4d 01 c8                    	addq	%r9, %r8
140806541: 49 83 c0 10                 	addq	$0x10, %r8
140806545: 49 83 c1 10                 	addq	$0x10, %r9
140806549: 49 21 d0                    	andq	%rdx, %r8
14080654c: eb 99                       	jmp	0x1408064e7 <.text+0x8054e7>
14080654e: 66 90                       	nop
140806550: 48 8b 9d e0 00 00 00        	movq	0xe0(%rbp), %rbx
140806557: 48 3b 9d d0 00 00 00        	cmpq	0xd0(%rbp), %rbx
14080655e: 0f 85 dc fe ff ff           	jne	0x140806440 <.text+0x805440>
140806564: 48 89 f9                    	movq	%rdi, %rcx
140806567: 48 89 f2                    	movq	%rsi, %rdx
14080656a: e8 01 93 35 00              	callq	0x140b5f870 <.text+0xb5e870>
14080656f: e9 cc fe ff ff              	jmp	0x140806440 <.text+0x805440>
140806574: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
14080657b: 48 8b 00                    	movq	(%rax), %rax
14080657e: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
140806585: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
14080658c: 8b 41 1c                    	movl	0x1c(%rcx), %eax
14080658f: 89 85 38 01 00 00           	movl	%eax, 0x138(%rbp)
140806595: 48 8b 41 10                 	movq	0x10(%rcx), %rax
140806599: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
1408065a0: 48 8d 75 d0                 	leaq	-0x30(%rbp), %rsi
1408065a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1408065b0: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
1408065b7: 48 85 c0                    	testq	%rax, %rax
1408065ba: 0f 84 85 01 00 00           	je	0x140806745 <.text+0x805745>
1408065c0: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
1408065c4: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
1408065cb: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
1408065d2: 44 8b 74 81 fc              	movl	-0x4(%rcx,%rax,4), %r14d
1408065d7: 80 bd 20 01 00 00 00        	cmpb	$0x0, 0x120(%rbp)
1408065de: 0f 84 0c 01 00 00           	je	0x1408066f0 <.text+0x8056f0>
1408065e4: 4c 89 e1                    	movq	%r12, %rcx
1408065e7: e8 c4 82 0f 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
1408065ec: 44 89 75 d0                 	movl	%r14d, -0x30(%rbp)
1408065f0: 48 89 c1                    	movq	%rax, %rcx
1408065f3: 48 89 f2                    	movq	%rsi, %rdx
1408065f6: e8 55 78 a5 00              	callq	0x14125de50 <.text+0x125ce50>
1408065fb: 48 85 c0                    	testq	%rax, %rax
1408065fe: 74 0e                       	je	0x14080660e <.text+0x80560e>
140806600: 4c 8b 78 08                 	movq	0x8(%rax), %r15
140806604: 48 8b 40 10                 	movq	0x10(%rax), %rax
140806608: 49 8d 1c c7                 	leaq	(%r15,%rax,8), %rbx
14080660c: eb 22                       	jmp	0x140806630 <.text+0x805630>
14080660e: bb 04 00 00 00              	movl	$0x4, %ebx
140806613: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140806619: eb 15                       	jmp	0x140806630 <.text+0x805630>
14080661b: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
140806622: 44 89 2c b8                 	movl	%r13d, (%rax,%rdi,4)
140806626: 48 ff c7                    	incq	%rdi
140806629: 48 89 bd e0 00 00 00        	movq	%rdi, 0xe0(%rbp)
140806630: 49 39 df                    	cmpq	%rbx, %r15
140806633: 0f 84 b7 00 00 00           	je	0x1408066f0 <.text+0x8056f0>
140806639: 45 8b 2f                    	movl	(%r15), %r13d
14080663c: 4c 89 e1                    	movq	%r12, %rcx
14080663f: e8 6c 82 0f 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
140806644: 44 89 6d d0                 	movl	%r13d, -0x30(%rbp)
140806648: 48 89 c1                    	movq	%rax, %rcx
14080664b: 48 89 f2                    	movq	%rsi, %rdx
14080664e: e8 fd 77 a5 00              	callq	0x14125de50 <.text+0x125ce50>
140806653: 49 83 c7 08                 	addq	$0x8, %r15
140806657: 48 85 c0                    	testq	%rax, %rax
14080665a: 74 d4                       	je	0x140806630 <.text+0x805630>
14080665c: 48 8b 40 10                 	movq	0x10(%rax), %rax
140806660: 48 85 c0                    	testq	%rax, %rax
140806663: 74 cb                       	je	0x140806630 <.text+0x805630>
140806665: 48 8d 04 c5 f8 ff ff ff     	leaq	-0x8(,%rax,8), %rax
14080666d: 48 83 f8 07                 	cmpq	$0x7, %rax
140806671: 77 bd                       	ja	0x140806630 <.text+0x805630>
140806673: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14080667a: 44 89 ea                    	movl	%r13d, %edx
14080667d: e8 0e 57 32 00              	callq	0x140b2bd90 <.text+0xb2ad90>
140806682: 84 c0                       	testb	%al, %al
140806684: 74 aa                       	je	0x140806630 <.text+0x805630>
140806686: 48 8b bd c8 00 00 00        	movq	0xc8(%rbp), %rdi
14080668d: 48 3b bd b8 00 00 00        	cmpq	0xb8(%rbp), %rdi
140806694: 75 13                       	jne	0x1408066a9 <.text+0x8056a9>
140806696: 48 8d 8d b8 00 00 00        	leaq	0xb8(%rbp), %rcx
14080669d: 48 8d 15 cc 77 28 02        	leaq	0x22877cc(%rip), %rdx   # 0x142a8de70
1408066a4: e8 c7 91 35 00              	callq	0x140b5f870 <.text+0xb5e870>
1408066a9: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
1408066b0: 44 89 2c b8                 	movl	%r13d, (%rax,%rdi,4)
1408066b4: 48 ff c7                    	incq	%rdi
1408066b7: 48 89 bd c8 00 00 00        	movq	%rdi, 0xc8(%rbp)
1408066be: 48 8b bd e0 00 00 00        	movq	0xe0(%rbp), %rdi
1408066c5: 48 3b bd d0 00 00 00        	cmpq	0xd0(%rbp), %rdi
1408066cc: 0f 85 49 ff ff ff           	jne	0x14080661b <.text+0x80561b>
1408066d2: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
1408066d9: 48 8d 15 a8 77 28 02        	leaq	0x22877a8(%rip), %rdx   # 0x142a8de88
1408066e0: e8 8b 91 35 00              	callq	0x140b5f870 <.text+0xb5e870>
1408066e5: e9 31 ff ff ff              	jmp	0x14080661b <.text+0x80561b>
1408066ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1408066f0: 8b 85 38 01 00 00           	movl	0x138(%rbp), %eax
1408066f6: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
1408066fd: 89 01                       	movl	%eax, (%rcx)
1408066ff: 4c 89 e1                    	movq	%r12, %rcx
140806702: 44 89 f2                    	movl	%r14d, %edx
140806705: e8 e6 97 0f 00              	callq	0x1408ffef0 <.text+0x8feef0>
14080670a: 48 89 f1                    	movq	%rsi, %rcx
14080670d: 4c 89 e2                    	movq	%r12, %rdx
140806710: e8 7b 81 0f 00              	callq	0x1408fe890 <.text+0x8fd890>
140806715: 83 7d d0 03                 	cmpl	$0x3, -0x30(%rbp)
140806719: 0f 84 91 fe ff ff           	je	0x1408065b0 <.text+0x8055b0>
14080671f: 44 39 75 e0                 	cmpl	%r14d, -0x20(%rbp)
140806723: 0f 85 87 fe ff ff           	jne	0x1408065b0 <.text+0x8055b0>
140806729: 8b 85 38 01 00 00           	movl	0x138(%rbp), %eax
14080672f: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
140806736: 89 01                       	movl	%eax, (%rcx)
140806738: 4c 89 e1                    	movq	%r12, %rcx
14080673b: e8 40 81 0f 00              	callq	0x1408fe880 <.text+0x8fd880>
140806740: e9 6b fe ff ff              	jmp	0x1408065b0 <.text+0x8055b0>
140806745: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
14080674c: 48 85 d2                    	testq	%rdx, %rdx
14080674f: 74 16                       	je	0x140806767 <.text+0x805767>
140806751: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140806758: 48 c1 e2 02                 	shlq	$0x2, %rdx
14080675c: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140806762: e8 b9 d4 e0 ff              	callq	0x140613c20 <.text+0x612c20>
140806767: c6 85 47 01 00 00 01        	movb	$0x1, 0x147(%rbp)
14080676e: 31 c9                       	xorl	%ecx, %ecx
140806770: e8 ab 33 8d ff              	callq	0x1400d9b20 <.text+0xd8b20>
140806775: 48 85 c0                    	testq	%rax, %rax
140806778: 0f 84 ab 05 00 00           	je	0x140806d29 <.text+0x805d29>
14080677e: 0f 10 00                    	movups	(%rax), %xmm0
140806781: 48 ff 00                    	incq	(%rax)
140806784: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
14080678b: 0f 10 05 5e ba 41 02        	movups	0x241ba5e(%rip), %xmm0  # 0x142c221f0
140806792: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
140806799: 0f 10 05 40 ba 41 02        	movups	0x241ba40(%rip), %xmm0  # 0x142c221e0
1408067a0: 0f 29 85 d0 00 00 00        	movaps	%xmm0, 0xd0(%rbp)
1408067a7: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
1408067ae: 48 8b 00                    	movq	(%rax), %rax
1408067b1: 48 8b 30                    	movq	(%rax), %rsi
1408067b4: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
1408067bb: 48 8b 78 18                 	movq	0x18(%rax), %rdi
1408067bf: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
1408067c3: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
1408067c7: f7 d1                       	notl	%ecx
1408067c9: 48 8d 5e 10                 	leaq	0x10(%rsi), %rbx
1408067cd: 4c 8d ad d0 00 00 00        	leaq	0xd0(%rbp), %r13
1408067d4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1408067e0: 48 85 ff                    	testq	%rdi, %rdi
1408067e3: 0f 84 9f 00 00 00           	je	0x140806888 <.text+0x805888>
1408067e9: 66 85 c9                    	testw	%cx, %cx
1408067ec: 74 12                       	je	0x140806800 <.text+0x805800>
1408067ee: 44 8d 71 ff                 	leal	-0x1(%rcx), %r14d
1408067f2: 41 21 ce                    	andl	%ecx, %r14d
1408067f5: eb 33                       	jmp	0x14080682a <.text+0x80582a>
1408067f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140806800: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
140806804: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
140806808: 48 81 c6 40 fe ff ff        	addq	$-0x1c0, %rsi           # imm = 0xFE40
14080680f: 48 83 c3 10                 	addq	$0x10, %rbx
140806813: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
140806819: 74 e5                       	je	0x140806800 <.text+0x805800>
14080681b: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140806820: 29 c8                       	subl	%ecx, %eax
140806822: f7 d1                       	notl	%ecx
140806824: 41 89 ce                    	movl	%ecx, %r14d
140806827: 41 21 c6                    	andl	%eax, %r14d
14080682a: f3 0f bc c1                 	tzcntl	%ecx, %eax
14080682e: 48 89 c1                    	movq	%rax, %rcx
140806831: 48 f7 d9                    	negq	%rcx
140806834: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140806838: 4c 8d 3c 49                 	leaq	(%rcx,%rcx,2), %r15
14080683c: 49 29 c7                    	subq	%rax, %r15
14080683f: 42 8b 54 3e e4              	movl	-0x1c(%rsi,%r15), %edx
140806844: 46 8b 44 3e e8              	movl	-0x18(%rsi,%r15), %r8d
140806849: c6 85 46 01 00 00 01        	movb	$0x1, 0x146(%rbp)
140806850: 4c 89 e1                    	movq	%r12, %rcx
140806853: e8 28 a6 0f 00              	callq	0x140900e80 <.text+0x8ffe80>
140806858: 48 ff cf                    	decq	%rdi
14080685b: 44 89 f1                    	movl	%r14d, %ecx
14080685e: 84 c0                       	testb	%al, %al
140806860: 0f 85 7a ff ff ff           	jne	0x1408067e0 <.text+0x8057e0>
140806866: 49 01 f7                    	addq	%rsi, %r15
140806869: 41 8b 57 e4                 	movl	-0x1c(%r15), %edx
14080686d: 45 8b 47 e8                 	movl	-0x18(%r15), %r8d
140806871: c6 85 46 01 00 00 01        	movb	$0x1, 0x146(%rbp)
140806878: 4c 89 e9                    	movq	%r13, %rcx
14080687b: e8 c0 1d ac ff              	callq	0x1402c8640 <.text+0x2c7640>
140806880: 44 89 f1                    	movl	%r14d, %ecx
140806883: e9 58 ff ff ff              	jmp	0x1408067e0 <.text+0x8057e0>
140806888: 48 8b 8d e8 01 00 00        	movq	0x1e8(%rbp), %rcx
14080688f: 8b 41 1c                    	movl	0x1c(%rcx), %eax
140806892: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
140806896: 89 01                       	movl	%eax, (%rcx)
140806898: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
14080689f: 8b 78 1c                    	movl	0x1c(%rax), %edi
1408068a2: 48 8b 58 10                 	movq	0x10(%rax), %rbx
1408068a6: 89 3b                       	movl	%edi, (%rbx)
1408068a8: 4c 8b 20                    	movq	(%rax), %r12
1408068ab: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
1408068b2: 48 8d 88 90 00 00 00        	leaq	0x90(%rax), %rcx
1408068b9: 4c 89 6d d0                 	movq	%r13, -0x30(%rbp)
1408068bd: c6 85 46 01 00 00 01        	movb	$0x1, 0x146(%rbp)
1408068c4: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1408068c8: 4d 89 e0                    	movq	%r12, %r8
1408068cb: e8 50 1c a6 00              	callq	0x141268520 <.text+0x1267520>
1408068d0: 4c 8b b5 d0 00 00 00        	movq	0xd0(%rbp), %r14
1408068d7: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
1408068de: 4c 8b ad e8 00 00 00        	movq	0xe8(%rbp), %r13
1408068e5: 48 85 c9                    	testq	%rcx, %rcx
1408068e8: 48 89 8d 90 00 00 00        	movq	%rcx, 0x90(%rbp)
1408068ef: 74 3d                       	je	0x14080692e <.text+0x80592e>
1408068f1: 48 8d 04 cd 17 00 00 00     	leaq	0x17(,%rcx,8), %rax
1408068f9: 48 83 e0 f0                 	andq	$-0x10, %rax
1408068fd: 48 01 c1                    	addq	%rax, %rcx
140806900: 48 83 c1 11                 	addq	$0x11, %rcx
140806904: 48 89 8d a8 00 00 00        	movq	%rcx, 0xa8(%rbp)
14080690b: 4c 89 f1                    	movq	%r14, %rcx
14080690e: 48 29 c1                    	subq	%rax, %rcx
140806911: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
140806918: b8 10 00 00 00              	movl	$0x10, %eax
14080691d: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140806924: 4d 85 ed                    	testq	%r13, %r13
140806927: 75 19                       	jne	0x140806942 <.text+0x805942>
140806929: e9 ba 01 00 00              	jmp	0x140806ae8 <.text+0x805ae8>
14080692e: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
140806939: 4d 85 ed                    	testq	%r13, %r13
14080693c: 0f 84 a6 01 00 00           	je	0x140806ae8 <.text+0x805ae8>
140806942: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
140806949: 48 8d 48 30                 	leaq	0x30(%rax), %rcx
14080694d: 48 89 8d 18 01 00 00        	movq	%rcx, 0x118(%rbp)
140806954: 48 83 c0 60                 	addq	$0x60, %rax
140806958: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
14080695f: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
140806964: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
14080696b: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14080696e: 89 8d a4 00 00 00           	movl	%ecx, 0xa4(%rbp)
140806974: 48 8b 08                    	movq	(%rax), %rcx
140806977: 48 8b 40 10                 	movq	0x10(%rax), %rax
14080697b: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14080697f: 48 8d 81 80 01 00 00        	leaq	0x180(%rcx), %rax
140806986: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14080698a: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14080698e: f7 d0                       	notl	%eax
140806990: 4d 8d 7e 10                 	leaq	0x10(%r14), %r15
140806994: 48 81 c1 a0 01 00 00        	addq	$0x1a0, %rcx            # imm = 0x1A0
14080699b: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
14080699f: 90                          	nop
1408069a0: 66 85 c0                    	testw	%ax, %ax
1408069a3: 74 0b                       	je	0x1408069b0 <.text+0x8059b0>
1408069a5: 8d 70 ff                    	leal	-0x1(%rax), %esi
1408069a8: 21 c6                       	andl	%eax, %esi
1408069aa: eb 29                       	jmp	0x1408069d5 <.text+0x8059d5>
1408069ac: 0f 1f 40 00                 	nopl	(%rax)
1408069b0: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
1408069b5: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1408069b9: 49 83 c6 80                 	addq	$-0x80, %r14
1408069bd: 49 83 c7 10                 	addq	$0x10, %r15
1408069c1: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1408069c6: 74 e8                       	je	0x1408069b0 <.text+0x8059b0>
1408069c8: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1408069cd: 29 c1                       	subl	%eax, %ecx
1408069cf: f7 d0                       	notl	%eax
1408069d1: 89 c6                       	movl	%eax, %esi
1408069d3: 21 ce                       	andl	%ecx, %esi
1408069d5: f3 0f bc c0                 	tzcntl	%eax, %eax
1408069d9: c1 e0 03                    	shll	$0x3, %eax
1408069dc: 4c 89 f1                    	movq	%r14, %rcx
1408069df: 48 29 c1                    	subq	%rax, %rcx
1408069e2: 48 8b 41 f8                 	movq	-0x8(%rcx), %rax
1408069e6: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
1408069ed: 89 3b                       	movl	%edi, (%rbx)
1408069ef: 44 8b 8d 28 01 00 00        	movl	0x128(%rbp), %r9d
1408069f6: 8b 85 2c 01 00 00           	movl	0x12c(%rbp), %eax
1408069fc: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
140806a00: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140806a04: 4c 89 e2                    	movq	%r12, %rdx
140806a07: 4c 8b 85 38 01 00 00        	movq	0x138(%rbp), %r8
140806a0e: e8 bd c0 a4 ff              	callq	0x140252ad0 <.text+0x251ad0>
140806a13: 80 7d f0 11                 	cmpb	$0x11, -0x10(%rbp)
140806a17: 74 13                       	je	0x140806a2c <.text+0x805a2c>
140806a19: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140806a1d: f0                          	lock
140806a1e: 48 ff 08                    	decq	(%rax)
140806a21: 75 09                       	jne	0x140806a2c <.text+0x805a2c>
140806a23: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
140806a27: e8 f4 5b 35 00              	callq	0x140b5c620 <.text+0xb5b620>
140806a2c: 89 3b                       	movl	%edi, (%rbx)
140806a2e: 44 8b 8d 28 01 00 00        	movl	0x128(%rbp), %r9d
140806a35: 8b 85 2c 01 00 00           	movl	0x12c(%rbp), %eax
140806a3b: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
140806a3f: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140806a43: 4c 89 e2                    	movq	%r12, %rdx
140806a46: 4c 8b 85 18 01 00 00        	movq	0x118(%rbp), %r8
140806a4d: e8 7e c0 a4 ff              	callq	0x140252ad0 <.text+0x251ad0>
140806a52: 80 7d f0 11                 	cmpb	$0x11, -0x10(%rbp)
140806a56: 74 13                       	je	0x140806a6b <.text+0x805a6b>
140806a58: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140806a5c: f0                          	lock
140806a5d: 48 ff 08                    	decq	(%rax)
140806a60: 75 09                       	jne	0x140806a6b <.text+0x805a6b>
140806a62: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
140806a66: e8 b5 5b 35 00              	callq	0x140b5c620 <.text+0xb5b620>
140806a6b: 89 3b                       	movl	%edi, (%rbx)
140806a6d: 44 8b 8d 28 01 00 00        	movl	0x128(%rbp), %r9d
140806a74: 8b 85 2c 01 00 00           	movl	0x12c(%rbp), %eax
140806a7a: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
140806a7e: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140806a82: 4c 89 e2                    	movq	%r12, %rdx
140806a85: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
140806a8c: e8 3f c0 a4 ff              	callq	0x140252ad0 <.text+0x251ad0>
140806a91: 80 7d f0 11                 	cmpb	$0x11, -0x10(%rbp)
140806a95: 74 13                       	je	0x140806aaa <.text+0x805aaa>
140806a97: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140806a9b: f0                          	lock
140806a9c: 48 ff 08                    	decq	(%rax)
140806a9f: 75 09                       	jne	0x140806aaa <.text+0x805aaa>
140806aa1: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
140806aa5: e8 76 5b 35 00              	callq	0x140b5c620 <.text+0xb5b620>
140806aaa: 8b 85 a4 00 00 00           	movl	0xa4(%rbp), %eax
140806ab0: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140806ab4: 89 01                       	movl	%eax, (%rcx)
140806ab6: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140806aba: 48 8d 95 28 01 00 00        	leaq	0x128(%rbp), %rdx
140806ac1: e8 1a b1 da ff              	callq	0x1405b1be0 <.text+0x5b0be0>
140806ac6: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140806aca: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140806ace: 49 89 c0                    	movq	%rax, %r8
140806ad1: 4c 8d 8d 28 01 00 00        	leaq	0x128(%rbp), %r9
140806ad8: e8 a3 11 37 00              	callq	0x140b77c80 <.text+0xb76c80>
140806add: 89 f0                       	movl	%esi, %eax
140806adf: 49 ff cd                    	decq	%r13
140806ae2: 0f 85 b8 fe ff ff           	jne	0x1408069a0 <.text+0x8059a0>
140806ae8: 48 83 bd 90 00 00 00 00     	cmpq	$0x0, 0x90(%rbp)
140806af0: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
140806af7: 74 18                       	je	0x140806b11 <.text+0x805b11>
140806af9: 48 85 d2                    	testq	%rdx, %rdx
140806afc: 74 13                       	je	0x140806b11 <.text+0x805b11>
140806afe: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140806b05: 4c 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %r8
140806b0c: e8 0f d1 e0 ff              	callq	0x140613c20 <.text+0x612c20>
140806b11: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
140806b18: 48 8b 41 08                 	movq	0x8(%rcx), %rax
140806b1c: 8b 49 18                    	movl	0x18(%rcx), %ecx
140806b1f: 4c 89 a5 d0 00 00 00        	movq	%r12, 0xd0(%rbp)
140806b26: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140806b2d: 48 89 9d e0 00 00 00        	movq	%rbx, 0xe0(%rbp)
140806b34: 89 8d e8 00 00 00           	movl	%ecx, 0xe8(%rbp)
140806b3a: 89 bd ec 00 00 00           	movl	%edi, 0xec(%rbp)
140806b40: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140806b44: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140806b48: f3 0f 6f 48 10              	movdqu	0x10(%rax), %xmm1
140806b4d: f3 0f 7f 85 f0 00 00 00     	movdqu	%xmm0, 0xf0(%rbp)
140806b55: f3 0f 7f 8d 00 01 00 00     	movdqu	%xmm1, 0x100(%rbp)
140806b5d: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140806b64: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140806b6b: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140806b72: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
140806b79: 4c 8b b5 c8 00 00 00        	movq	0xc8(%rbp), %r14
140806b80: 4d 85 f6                    	testq	%r14, %r14
140806b83: 0f 84 c8 00 00 00           	je	0x140806c51 <.text+0x805c51>
140806b89: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
140806b90: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140806b93: 89 8d 18 01 00 00           	movl	%ecx, 0x118(%rbp)
140806b99: 4c 8b 60 10                 	movq	0x10(%rax), %r12
140806b9d: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
140806ba4: 44 8b 68 1c                 	movl	0x1c(%rax), %r13d
140806ba8: 48 8b 70 10                 	movq	0x10(%rax), %rsi
140806bac: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
140806bb3: 48 8b 38                    	movq	(%rax), %rdi
140806bb6: 49 c1 e6 02                 	shlq	$0x2, %r14
140806bba: 45 31 ff                    	xorl	%r15d, %r15d
140806bbd: eb 0e                       	jmp	0x140806bcd <.text+0x805bcd>
140806bbf: 90                          	nop
140806bc0: 49 83 c7 04                 	addq	$0x4, %r15
140806bc4: 4d 39 fe                    	cmpq	%r15, %r14
140806bc7: 0f 84 84 00 00 00           	je	0x140806c51 <.text+0x805c51>
140806bcd: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
140806bd4: 42 8b 1c 38                 	movl	(%rax,%r15), %ebx
140806bd8: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
140806bdf: 89 da                       	movl	%ebx, %edx
140806be1: e8 2a 59 21 00              	callq	0x140a1c510 <.text+0xa1b510>
140806be6: 48 83 f8 03                 	cmpq	$0x3, %rax
140806bea: 74 d4                       	je	0x140806bc0 <.text+0x805bc0>
140806bec: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140806bf0: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
140806bf4: 89 5d e8                    	movl	%ebx, -0x18(%rbp)
140806bf7: 48 c7 45 d0 04 00 00 00     	movq	$0x4, -0x30(%rbp)
140806bff: c6 45 cc 02                 	movb	$0x2, -0x34(%rbp)
140806c03: 8b 85 18 01 00 00           	movl	0x118(%rbp), %eax
140806c09: 41 89 04 24                 	movl	%eax, (%r12)
140806c0d: 44 89 2e                    	movl	%r13d, (%rsi)
140806c10: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
140806c15: 48 8d 85 d0 00 00 00        	leaq	0xd0(%rbp), %rax
140806c1c: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140806c21: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
140806c28: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140806c2d: 48 c7 44 24 28 00 00 00 00  	movq	$0x0, 0x28(%rsp)
140806c36: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140806c3a: 31 d2                       	xorl	%edx, %edx
140806c3c: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
140806c40: 4c 8b 8d 10 01 00 00        	movq	0x110(%rbp), %r9
140806c47: e8 e4 01 a7 00              	callq	0x141276e30 <.text+0x1275e30>
140806c4c: e9 6f ff ff ff              	jmp	0x140806bc0 <.text+0x805bc0>
140806c51: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
140806c58: 48 85 d2                    	testq	%rdx, %rdx
140806c5b: 74 16                       	je	0x140806c73 <.text+0x805c73>
140806c5d: 48 c1 e2 02                 	shlq	$0x2, %rdx
140806c61: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140806c67: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
140806c6e: e8 ad cf e0 ff              	callq	0x140613c20 <.text+0x612c20>
140806c73: 48 83 7d 70 00              	cmpq	$0x0, 0x70(%rbp)
140806c78: 74 40                       	je	0x140806cba <.text+0x805cba>
140806c7a: e8 b1 14 41 00              	callq	0x140c18130 <.text+0xc17130>
140806c7f: 48 89 c1                    	movq	%rax, %rcx
140806c82: 48 8b 00                    	movq	(%rax), %rax
140806c85: 48 83 f8 01                 	cmpq	$0x1, %rax
140806c89: 75 4a                       	jne	0x140806cd5 <.text+0x805cd5>
140806c8b: 48 83 c1 08                 	addq	$0x8, %rcx
140806c8f: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140806c93: 75 7a                       	jne	0x140806d0f <.text+0x805d0f>
140806c95: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140806c9c: 48 89 8d 30 01 00 00        	movq	%rcx, 0x130(%rbp)
140806ca3: 48 83 c1 08                 	addq	$0x8, %rcx
140806ca7: 48 8b 55 78                 	movq	0x78(%rbp), %rdx
140806cab: e8 40 04 41 00              	callq	0x140c170f0 <.text+0xc160f0>
140806cb0: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
140806cb7: 48 ff 00                    	incq	(%rax)
140806cba: 0f 28 b5 50 01 00 00        	movaps	0x150(%rbp), %xmm6
140806cc1: 48 81 c4 e8 01 00 00        	addq	$0x1e8, %rsp            # imm = 0x1E8
140806cc8: 5b                          	popq	%rbx
140806cc9: 5f                          	popq	%rdi
140806cca: 5e                          	popq	%rsi
140806ccb: 41 5c                       	popq	%r12
140806ccd: 41 5d                       	popq	%r13
140806ccf: 41 5e                       	popq	%r14
140806cd1: 41 5f                       	popq	%r15
140806cd3: 5d                          	popq	%rbp
140806cd4: c3                          	retq
140806cd5: 48 85 c0                    	testq	%rax, %rax
140806cd8: 75 29                       	jne	0x140806d03 <.text+0x805d03>
140806cda: 31 d2                       	xorl	%edx, %edx
140806cdc: e8 9f b8 0e 02              	callq	0x1428f2580 <.text+0x28f1580>
140806ce1: 48 89 c1                    	movq	%rax, %rcx
140806ce4: 48 85 c0                    	testq	%rax, %rax
140806ce7: 75 a6                       	jne	0x140806c8f <.text+0x805c8f>
140806ce9: eb 18                       	jmp	0x140806d03 <.text+0x805d03>
140806ceb: 48 85 c0                    	testq	%rax, %rax
140806cee: 75 13                       	jne	0x140806d03 <.text+0x805d03>
140806cf0: 31 d2                       	xorl	%edx, %edx
140806cf2: e8 89 b8 0e 02              	callq	0x1428f2580 <.text+0x28f1580>
140806cf7: 48 89 c1                    	movq	%rax, %rcx
140806cfa: 48 85 c0                    	testq	%rax, %rax
140806cfd: 0f 85 39 f6 ff ff           	jne	0x14080633c <.text+0x80533c>
140806d03: 48 8d 0d be a6 41 02        	leaq	0x241a6be(%rip), %rcx   # 0x142c213c8
140806d0a: e8 c1 13 0d 02              	callq	0x1428d80d0 <.text+0x28d70d0>
140806d0f: 48 8d 0d 22 a7 41 02        	leaq	0x241a722(%rip), %rcx   # 0x142c21438
140806d16: e8 35 25 0d 02              	callq	0x1428d9250 <.text+0x28d8250>
140806d1b: 48 8d 0d 96 71 28 02        	leaq	0x2287196(%rip), %rcx   # 0x142a8deb8
140806d22: e8 e9 25 0d 02              	callq	0x1428d9310 <.text+0x28d8310>
140806d27: eb 13                       	jmp	0x140806d3c <.text+0x805d3c>
140806d29: c6 85 47 01 00 00 01        	movb	$0x1, 0x147(%rbp)
140806d30: 48 8d 0d 91 a6 41 02        	leaq	0x241a691(%rip), %rcx   # 0x142c213c8
140806d37: e8 94 13 0d 02              	callq	0x1428d80d0 <.text+0x28d70d0>
140806d3c: 0f 0b                       	ud2
140806d3e: 66 90                       	nop
140806d40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140806d45: 55                          	pushq	%rbp
140806d46: 41 57                       	pushq	%r15
140806d48: 41 56                       	pushq	%r14
140806d4a: 41 55                       	pushq	%r13
140806d4c: 41 54                       	pushq	%r12
140806d4e: 56                          	pushq	%rsi
140806d4f: 57                          	pushq	%rdi
140806d50: 53                          	pushq	%rbx
140806d51: 48 83 ec 58                 	subq	$0x58, %rsp
140806d55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140806d5c: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140806d62: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
140806d69: 48 ff 00                    	incq	(%rax)
140806d6c: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140806d71: 48 83 c4 58                 	addq	$0x58, %rsp
140806d75: 5b                          	popq	%rbx
140806d76: 5f                          	popq	%rdi
140806d77: 5e                          	popq	%rsi
140806d78: 41 5c                       	popq	%r12
140806d7a: 41 5d                       	popq	%r13
140806d7c: 41 5e                       	popq	%r14
140806d7e: 41 5f                       	popq	%r15
140806d80: 5d                          	popq	%rbp
140806d81: c3                          	retq
140806d82: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140806d90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140806d95: 55                          	pushq	%rbp
140806d96: 41 57                       	pushq	%r15
140806d98: 41 56                       	pushq	%r14
140806d9a: 41 55                       	pushq	%r13
140806d9c: 41 54                       	pushq	%r12
140806d9e: 56                          	pushq	%rsi
140806d9f: 57                          	pushq	%rdi
140806da0: 53                          	pushq	%rbx
140806da1: 48 83 ec 58                 	subq	$0x58, %rsp
140806da5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140806dac: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140806db2: 48 83 7d 70 00              	cmpq	$0x0, 0x70(%rbp)
140806db7: 74 20                       	je	0x140806dd9 <.text+0x805dd9>
140806db9: 48 8d 45 78                 	leaq	0x78(%rbp), %rax
140806dbd: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140806dc1: 48 8d 45 48                 	leaq	0x48(%rbp), %rax
140806dc5: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140806dc9: 48 8d 0d b0 6e 28 02        	leaq	0x2286eb0(%rip), %rcx   # 0x142a8dc80
140806dd0: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
140806dd4: e8 e7 0d a5 00              	callq	0x141257bc0 <.text+0x1256bc0>
140806dd9: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140806dde: 48 83 c4 58                 	addq	$0x58, %rsp
140806de2: 5b                          	popq	%rbx
140806de3: 5f                          	popq	%rdi
140806de4: 5e                          	popq	%rsi
140806de5: 41 5c                       	popq	%r12
140806de7: 41 5d                       	popq	%r13
140806de9: 41 5e                       	popq	%r14
140806deb: 41 5f                       	popq	%r15
140806ded: 5d                          	popq	%rbp
140806dee: c3                          	retq
140806def: 90                          	nop
140806df0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140806df5: 55                          	pushq	%rbp
140806df6: 41 57                       	pushq	%r15
140806df8: 41 56                       	pushq	%r14
140806dfa: 41 55                       	pushq	%r13
140806dfc: 41 54                       	pushq	%r12
140806dfe: 56                          	pushq	%rsi
140806dff: 57                          	pushq	%rdi
140806e00: 53                          	pushq	%rbx
140806e01: 48 83 ec 58                 	subq	$0x58, %rsp
140806e05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140806e0c: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140806e12: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
140806e19: 48 85 d2                    	testq	%rdx, %rdx
140806e1c: 74 16                       	je	0x140806e34 <.text+0x805e34>
140806e1e: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140806e25: 48 c1 e2 02                 	shlq	$0x2, %rdx
140806e29: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140806e2f: e8 ec cd e0 ff              	callq	0x140613c20 <.text+0x612c20>
140806e34: c6 85 47 01 00 00 01        	movb	$0x1, 0x147(%rbp)
140806e3b: 66 0f 6f 74 24 40           	movdqa	0x40(%rsp), %xmm6
140806e41: 48 83 c4 58                 	addq	$0x58, %rsp
140806e45: 5b                          	popq	%rbx
140806e46: 5f                          	popq	%rdi
140806e47: 5e                          	popq	%rsi
140806e48: 41 5c                       	popq	%r12
140806e4a: 41 5d                       	popq	%r13
140806e4c: 41 5e                       	popq	%r14
140806e4e: 41 5f                       	popq	%r15
140806e50: 5d                          	popq	%rbp
140806e51: c3                          	retq
140806e52: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140806e60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140806e65: 55                          	pushq	%rbp
140806e66: 41 57                       	pushq	%r15
140806e68: 41 56                       	pushq	%r14
140806e6a: 41 55                       	pushq	%r13
140806e6c: 41 54                       	pushq	%r12
140806e6e: 56                          	pushq	%rsi
140806e6f: 57                          	pushq	%rdi
140806e70: 53                          	pushq	%rbx
140806e71: 48 83 ec 58                 	subq	$0x58, %rsp
140806e75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140806e7c: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140806e82: 48 83 bd 90 00 00 00 00     	cmpq	$0x0, 0x90(%rbp)
140806e8a: 74 24                       	je	0x140806eb0 <.text+0x805eb0>
140806e8c: 48 83 bd a8 00 00 00 00     	cmpq	$0x0, 0xa8(%rbp)
140806e94: 74 1a                       	je	0x140806eb0 <.text+0x805eb0>
140806e96: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140806e9d: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
140806ea4: 4c 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %r8
140806eab: e8 70 cd e0 ff              	callq	0x140613c20 <.text+0x612c20>
140806eb0: c6 85 46 01 00 00 00        	movb	$0x0, 0x146(%rbp)
140806eb7: 66 0f 6f 74 24 40           	movdqa	0x40(%rsp), %xmm6
140806ebd: 48 83 c4 58                 	addq	$0x58, %rsp
140806ec1: 5b                          	popq	%rbx
140806ec2: 5f                          	popq	%rdi
140806ec3: 5e                          	popq	%rsi
140806ec4: 41 5c                       	popq	%r12
140806ec6: 41 5d                       	popq	%r13
140806ec8: 41 5e                       	popq	%r14
140806eca: 41 5f                       	popq	%r15
140806ecc: 5d                          	popq	%rbp
140806ecd: c3                          	retq
140806ece: 66 90                       	nop
140806ed0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140806ed5: 55                          	pushq	%rbp
140806ed6: 41 57                       	pushq	%r15
140806ed8: 41 56                       	pushq	%r14
140806eda: 41 55                       	pushq	%r13
140806edc: 41 54                       	pushq	%r12
140806ede: 56                          	pushq	%rsi
140806edf: 57                          	pushq	%rdi
140806ee0: 53                          	pushq	%rbx
140806ee1: 48 83 ec 58                 	subq	$0x58, %rsp
140806ee5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140806eec: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140806ef2: 48 83 bd 20 01 00 00 00     	cmpq	$0x0, 0x120(%rbp)
140806efa: 74 1d                       	je	0x140806f19 <.text+0x805f19>
140806efc: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
140806f03: 48 c1 e2 02                 	shlq	$0x2, %rdx
140806f07: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140806f0d: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
140806f14: e8 07 cd e0 ff              	callq	0x140613c20 <.text+0x612c20>
140806f19: c6 85 47 01 00 00 00        	movb	$0x0, 0x147(%rbp)
140806f20: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140806f25: 48 83 c4 58                 	addq	$0x58, %rsp
140806f29: 5b                          	popq	%rbx
140806f2a: 5f                          	popq	%rdi
140806f2b: 5e                          	popq	%rsi
140806f2c: 41 5c                       	popq	%r12
140806f2e: 41 5d                       	popq	%r13
140806f30: 41 5e                       	popq	%r14
140806f32: 41 5f                       	popq	%r15
140806f34: 5d                          	popq	%rbp
140806f35: c3                          	retq
140806f36: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140806f40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140806f45: 55                          	pushq	%rbp
140806f46: 41 57                       	pushq	%r15
140806f48: 41 56                       	pushq	%r14
140806f4a: 41 55                       	pushq	%r13
140806f4c: 41 54                       	pushq	%r12
140806f4e: 56                          	pushq	%rsi
140806f4f: 57                          	pushq	%rdi
140806f50: 53                          	pushq	%rbx
140806f51: 48 83 ec 58                 	subq	$0x58, %rsp
140806f55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140806f5c: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140806f62: 48 8b 85 30 01 00 00        	movq	0x130(%rbp), %rax
140806f69: 48 ff 00                    	incq	(%rax)
140806f6c: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140806f71: 48 83 c4 58                 	addq	$0x58, %rsp
140806f75: 5b                          	popq	%rbx
140806f76: 5f                          	popq	%rdi
140806f77: 5e                          	popq	%rsi
140806f78: 41 5c                       	popq	%r12
140806f7a: 41 5d                       	popq	%r13
140806f7c: 41 5e                       	popq	%r14
140806f7e: 41 5f                       	popq	%r15
140806f80: 5d                          	popq	%rbp
140806f81: c3                          	retq
140806f82: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140806f90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140806f95: 55                          	pushq	%rbp
140806f96: 41 57                       	pushq	%r15
140806f98: 41 56                       	pushq	%r14
140806f9a: 41 55                       	pushq	%r13
140806f9c: 41 54                       	pushq	%r12
140806f9e: 56                          	pushq	%rsi
140806f9f: 57                          	pushq	%rdi
140806fa0: 53                          	pushq	%rbx
140806fa1: 48 83 ec 58                 	subq	$0x58, %rsp
140806fa5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140806fac: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140806fb2: 80 bd 46 01 00 00 00        	cmpb	$0x0, 0x146(%rbp)
140806fb9: 74 29                       	je	0x140806fe4 <.text+0x805fe4>
140806fbb: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140806fc2: e8 89 2f 94 ff              	callq	0x140149f50 <.text+0x148f50>
140806fc7: c6 85 47 01 00 00 01        	movb	$0x1, 0x147(%rbp)
140806fce: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140806fd3: 48 83 c4 58                 	addq	$0x58, %rsp
140806fd7: 5b                          	popq	%rbx
140806fd8: 5f                          	popq	%rdi
140806fd9: 5e                          	popq	%rsi
140806fda: 41 5c                       	popq	%r12
140806fdc: 41 5d                       	popq	%r13
140806fde: 41 5e                       	popq	%r14
140806fe0: 41 5f                       	popq	%r15
140806fe2: 5d                          	popq	%rbp
140806fe3: c3                          	retq
140806fe4: c6 85 47 01 00 00 01        	movb	$0x1, 0x147(%rbp)
140806feb: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140806ff0: 48 83 c4 58                 	addq	$0x58, %rsp
140806ff4: 5b                          	popq	%rbx
140806ff5: 5f                          	popq	%rdi
140806ff6: 5e                          	popq	%rsi
140806ff7: 41 5c                       	popq	%r12
140806ff9: 41 5d                       	popq	%r13
140806ffb: 41 5e                       	popq	%r14
140806ffd: 41 5f                       	popq	%r15
140806fff: 5d                          	popq	%rbp
140807000: c3                          	retq
140807001: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140807010: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140807015: 55                          	pushq	%rbp
140807016: 41 57                       	pushq	%r15
140807018: 41 56                       	pushq	%r14
14080701a: 41 55                       	pushq	%r13
14080701c: 41 54                       	pushq	%r12
14080701e: 56                          	pushq	%rsi
14080701f: 57                          	pushq	%rdi
140807020: 53                          	pushq	%rbx
140807021: 48 83 ec 58                 	subq	$0x58, %rsp
140807025: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14080702c: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140807032: 80 bd 47 01 00 00 00        	cmpb	$0x0, 0x147(%rbp)
140807039: 74 39                       	je	0x140807074 <.text+0x806074>
14080703b: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
140807042: 48 85 d2                    	testq	%rdx, %rdx
140807045: 74 16                       	je	0x14080705d <.text+0x80605d>
140807047: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
14080704e: 48 c1 e2 02                 	shlq	$0x2, %rdx
140807052: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140807058: e8 c3 cb e0 ff              	callq	0x140613c20 <.text+0x612c20>
14080705d: 66 0f 6f 74 24 40           	movdqa	0x40(%rsp), %xmm6
140807063: 48 83 c4 58                 	addq	$0x58, %rsp
140807067: 5b                          	popq	%rbx
140807068: 5f                          	popq	%rdi
140807069: 5e                          	popq	%rsi
14080706a: 41 5c                       	popq	%r12
14080706c: 41 5d                       	popq	%r13
14080706e: 41 5e                       	popq	%r14
140807070: 41 5f                       	popq	%r15
140807072: 5d                          	popq	%rbp
140807073: c3                          	retq
140807074: 66 0f 6f 74 24 40           	movdqa	0x40(%rsp), %xmm6
14080707a: 48 83 c4 58                 	addq	$0x58, %rsp
14080707e: 5b                          	popq	%rbx
14080707f: 5f                          	popq	%rdi
140807080: 5e                          	popq	%rsi
140807081: 41 5c                       	popq	%r12
140807083: 41 5d                       	popq	%r13
140807085: 41 5e                       	popq	%r14
140807087: 41 5f                       	popq	%r15
140807089: 5d                          	popq	%rbp
14080708a: c3                          	retq
