
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140865050: 55                          	pushq	%rbp
140865051: 41 57                       	pushq	%r15
140865053: 41 56                       	pushq	%r14
140865055: 41 55                       	pushq	%r13
140865057: 41 54                       	pushq	%r12
140865059: 56                          	pushq	%rsi
14086505a: 57                          	pushq	%rdi
14086505b: 53                          	pushq	%rbx
14086505c: 48 81 ec d8 01 00 00        	subq	$0x1d8, %rsp            # imm = 0x1D8
140865063: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14086506b: 44 0f 29 ad 40 01 00 00     	movaps	%xmm13, 0x140(%rbp)
140865073: 44 0f 29 a5 30 01 00 00     	movaps	%xmm12, 0x130(%rbp)
14086507b: 44 0f 29 9d 20 01 00 00     	movaps	%xmm11, 0x120(%rbp)
140865083: 44 0f 29 95 10 01 00 00     	movaps	%xmm10, 0x110(%rbp)
14086508b: 44 0f 29 8d 00 01 00 00     	movaps	%xmm9, 0x100(%rbp)
140865093: 44 0f 29 85 f0 00 00 00     	movaps	%xmm8, 0xf0(%rbp)
14086509b: 0f 29 bd e0 00 00 00        	movaps	%xmm7, 0xe0(%rbp)
1408650a2: 0f 29 b5 d0 00 00 00        	movaps	%xmm6, 0xd0(%rbp)
1408650a9: 48 c7 85 c8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xc8(%rbp)
1408650b4: 4c 89 cb                    	movq	%r9, %rbx
1408650b7: 4c 89 85 c0 00 00 00        	movq	%r8, 0xc0(%rbp)
1408650be: 49 89 d4                    	movq	%rdx, %r12
1408650c1: 49 89 ce                    	movq	%rcx, %r14
1408650c4: e8 37 19 3b 00              	callq	0x140c16a00 <.text+0xc15a00>
1408650c9: 84 c0                       	testb	%al, %al
1408650cb: 88 85 a0 00 00 00           	movb	%al, 0xa0(%rbp)
1408650d1: 74 5d                       	je	0x140865130 <.text+0x864130>
1408650d3: 41 bd 33 00 00 00           	movl	$0x33, %r13d
1408650d9: b9 33 00 00 00              	movl	$0x33, %ecx
1408650de: e8 5d cd ff ff              	callq	0x140861e40 <.text+0x860e40>
1408650e3: 48 8d 35 41 0e 23 02        	leaq	0x2230e41(%rip), %rsi   # 0x142a95f2b
1408650ea: 48 83 f8 01                 	cmpq	$0x1, %rax
1408650ee: 0f 85 a8 00 00 00           	jne	0x14086519c <.text+0x86419c>
1408650f4: 49 89 d1                    	movq	%rdx, %r9
1408650f7: 48 85 d2                    	testq	%rdx, %rdx
1408650fa: 74 3d                       	je	0x140865139 <.text+0x864139>
1408650fc: 49 83 f9 33                 	cmpq	$0x33, %r9
140865100: 73 35                       	jae	0x140865137 <.text+0x864137>
140865102: 48 8d 05 22 0e 23 02        	leaq	0x2230e22(%rip), %rax   # 0x142a95f2b
140865109: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14086510e: 7f 29                       	jg	0x140865139 <.text+0x864139>
140865110: 48 8d 05 d1 0e 23 02        	leaq	0x2230ed1(%rip), %rax   # 0x142a95fe8
140865117: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14086511c: 48 8d 0d 08 0e 23 02        	leaq	0x2230e08(%rip), %rcx   # 0x142a95f2b
140865123: ba 33 00 00 00              	movl	$0x33, %edx
140865128: 45 31 c0                    	xorl	%r8d, %r8d
14086512b: e8 10 48 07 02              	callq	0x1428d9940 <.text+0x28d8940>
140865130: 31 c0                       	xorl	%eax, %eax
140865132: e9 d9 01 00 00              	jmp	0x140865310 <.text+0x864310>
140865137: 75 d7                       	jne	0x140865110 <.text+0x864110>
140865139: 4c 89 c9                    	movq	%r9, %rcx
14086513c: e8 ff cc ff ff              	callq	0x140861e40 <.text+0x860e40>
140865141: 48 83 f8 01                 	cmpq	$0x1, %rax
140865145: 75 55                       	jne	0x14086519c <.text+0x86419c>
140865147: 49 89 d0                    	movq	%rdx, %r8
14086514a: 49 83 c0 02                 	addq	$0x2, %r8
14086514e: 74 39                       	je	0x140865189 <.text+0x864189>
140865150: 49 83 f8 33                 	cmpq	$0x33, %r8
140865154: 73 31                       	jae	0x140865187 <.text+0x864187>
140865156: 48 8d 05 ce 0d 23 02        	leaq	0x2230dce(%rip), %rax   # 0x142a95f2b
14086515d: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140865162: 7f 25                       	jg	0x140865189 <.text+0x864189>
140865164: 48 8d 05 95 0e 23 02        	leaq	0x2230e95(%rip), %rax   # 0x142a96000
14086516b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140865170: 48 8d 0d b4 0d 23 02        	leaq	0x2230db4(%rip), %rcx   # 0x142a95f2b
140865177: ba 33 00 00 00              	movl	$0x33, %edx
14086517c: 41 b9 33 00 00 00           	movl	$0x33, %r9d
140865182: e8 b9 47 07 02              	callq	0x1428d9940 <.text+0x28d8940>
140865187: 75 db                       	jne	0x140865164 <.text+0x864164>
140865189: 41 bd 31 00 00 00           	movl	$0x31, %r13d
14086518f: 49 29 d5                    	subq	%rdx, %r13
140865192: 48 8d 35 92 0d 23 02        	leaq	0x2230d92(%rip), %rsi   # 0x142a95f2b
140865199: 4c 01 c6                    	addq	%r8, %rsi
14086519c: 4c 8d 05 75 14 23 02        	leaq	0x2231475(%rip), %r8    # 0x142a96618
1408651a3: 4c 8d 3d 3e 14 23 02        	leaq	0x223143e(%rip), %r15   # 0x142a965e8
1408651aa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1408651b0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1408651b5: 85 c0                       	testl	%eax, %eax
1408651b7: 78 17                       	js	0x1408651d0 <.text+0x8641d0>
1408651b9: 49 ff c8                    	decq	%r8
1408651bc: 83 f8 5c                    	cmpl	$0x5c, %eax
1408651bf: 75 63                       	jne	0x140865224 <.text+0x864224>
1408651c1: eb 7b                       	jmp	0x14086523e <.text+0x86423e>
1408651c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1408651d0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1408651d5: 80 f9 c0                    	cmpb	$-0x40, %cl
1408651d8: 7d 23                       	jge	0x1408651fd <.text+0x8641fd>
1408651da: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1408651df: 80 fa c0                    	cmpb	$-0x40, %dl
1408651e2: 7d 22                       	jge	0x140865206 <.text+0x864206>
1408651e4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1408651e9: 49 83 c0 fc                 	addq	$-0x4, %r8
1408651ed: 41 83 e1 07                 	andl	$0x7, %r9d
1408651f1: 41 c1 e1 06                 	shll	$0x6, %r9d
1408651f5: 83 e2 3f                    	andl	$0x3f, %edx
1408651f8: 44 09 ca                    	orl	%r9d, %edx
1408651fb: eb 10                       	jmp	0x14086520d <.text+0x86420d>
1408651fd: 49 83 c0 fe                 	addq	$-0x2, %r8
140865201: 83 e1 1f                    	andl	$0x1f, %ecx
140865204: eb 0f                       	jmp	0x140865215 <.text+0x864215>
140865206: 49 83 c0 fd                 	addq	$-0x3, %r8
14086520a: 83 e2 0f                    	andl	$0xf, %edx
14086520d: c1 e2 06                    	shll	$0x6, %edx
140865210: 83 e1 3f                    	andl	$0x3f, %ecx
140865213: 09 d1                       	orl	%edx, %ecx
140865215: c1 e1 06                    	shll	$0x6, %ecx
140865218: 24 3f                       	andb	$0x3f, %al
14086521a: 0f b6 c0                    	movzbl	%al, %eax
14086521d: 09 c8                       	orl	%ecx, %eax
14086521f: 83 f8 5c                    	cmpl	$0x5c, %eax
140865222: 74 1a                       	je	0x14086523e <.text+0x86423e>
140865224: 83 f8 2f                    	cmpl	$0x2f, %eax
140865227: 74 15                       	je	0x14086523e <.text+0x86423e>
140865229: 4d 39 f8                    	cmpq	%r15, %r8
14086522c: 75 82                       	jne	0x1408651b0 <.text+0x8641b0>
14086522e: 4c 89 f7                    	movq	%r14, %rdi
140865231: 4d 89 e6                    	movq	%r12, %r14
140865234: 49 89 dc                    	movq	%rbx, %r12
140865237: bb 30 00 00 00              	movl	$0x30, %ebx
14086523c: eb 55                       	jmp	0x140865293 <.text+0x864293>
14086523e: 4c 8d 3d a3 13 23 02        	leaq	0x22313a3(%rip), %r15   # 0x142a965e8
140865245: 4d 29 f8                    	subq	%r15, %r8
140865248: 49 ff c0                    	incq	%r8
14086524b: 74 32                       	je	0x14086527f <.text+0x86427f>
14086524d: 49 83 f8 30                 	cmpq	$0x30, %r8
140865251: 73 2a                       	jae	0x14086527d <.text+0x86427d>
140865253: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
140865258: 7f 25                       	jg	0x14086527f <.text+0x86427f>
14086525a: 48 8d 05 67 0d 23 02        	leaq	0x2230d67(%rip), %rax   # 0x142a95fc8
140865261: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140865266: 48 8d 0d 7b 13 23 02        	leaq	0x223137b(%rip), %rcx   # 0x142a965e8
14086526d: ba 30 00 00 00              	movl	$0x30, %edx
140865272: 41 b9 30 00 00 00           	movl	$0x30, %r9d
140865278: e8 c3 46 07 02              	callq	0x1428d9940 <.text+0x28d8940>
14086527d: 75 db                       	jne	0x14086525a <.text+0x86425a>
14086527f: 4c 89 f7                    	movq	%r14, %rdi
140865282: 4d 89 e6                    	movq	%r12, %r14
140865285: 49 89 dc                    	movq	%rbx, %r12
140865288: bb 30 00 00 00              	movl	$0x30, %ebx
14086528d: 4c 29 c3                    	subq	%r8, %rbx
140865290: 4d 01 c7                    	addq	%r8, %r15
140865293: e8 98 2e 3b 00              	callq	0x140c18130 <.text+0xc17130>
140865298: 48 89 c1                    	movq	%rax, %rcx
14086529b: 48 8b 00                    	movq	(%rax), %rax
14086529e: 48 83 f8 01                 	cmpq	$0x1, %rax
1408652a2: 0f 85 42 0f 00 00           	jne	0x1408661ea <.text+0x8651ea>
1408652a8: 48 83 c1 08                 	addq	$0x8, %rcx
1408652ac: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1408652b0: 0f 85 98 0f 00 00           	jne	0x14086624e <.text+0x86524e>
1408652b6: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1408652bd: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
1408652c4: 48 83 c1 08                 	addq	$0x8, %rcx
1408652c8: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1408652cd: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1408652d6: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1408652df: 48 89 f2                    	movq	%rsi, %rdx
1408652e2: 4d 89 e8                    	movq	%r13, %r8
1408652e5: 4d 89 f9                    	movq	%r15, %r9
1408652e8: e8 53 1a 3b 00              	callq	0x140c16d40 <.text+0xc15d40>
1408652ed: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
1408652f4: 48 ff 01                    	incq	(%rcx)
1408652f7: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
1408652fe: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140865302: b8 01 00 00 00              	movl	$0x1, %eax
140865307: 4c 89 e3                    	movq	%r12, %rbx
14086530a: 4d 89 f4                    	movq	%r14, %r12
14086530d: 49 89 fe                    	movq	%rdi, %r14
140865310: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
140865317: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14086531b: 48 8b 01                    	movq	(%rcx), %rax
14086531e: 80 b8 fa 00 00 00 00        	cmpb	$0x0, 0xfa(%rax)
140865325: 0f 84 f4 01 00 00           	je	0x14086551f <.text+0x86451f>
14086532b: 49 8b 06                    	movq	(%r14), %rax
14086532e: 4c 8b a8 f8 00 00 00        	movq	0xf8(%rax), %r13
140865335: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
14086533c: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
140865343: 48 8b 73 08                 	movq	0x8(%rbx), %rsi
140865347: 48 85 f6                    	testq	%rsi, %rsi
14086534a: 75 04                       	jne	0x140865350 <.text+0x864350>
14086534c: 48 8b 73 10                 	movq	0x10(%rbx), %rsi
140865350: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
140865354: 48 89 8d a8 00 00 00        	movq	%rcx, 0xa8(%rbp)
14086535b: 4c 8d 24 85 00 00 00 00     	leaq	(,%rax,4), %r12
140865363: 4d 01 ec                    	addq	%r13, %r12
140865366: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
14086536d: 0f b6 80 08 01 00 00        	movzbl	0x108(%rax), %eax
140865374: 88 85 c0 00 00 00           	movb	%al, 0xc0(%rbp)
14086537a: b8 08 00 00 00              	movl	$0x8, %eax
14086537f: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
140865386: 45 31 ff                    	xorl	%r15d, %r15d
140865389: 45 31 f6                    	xorl	%r14d, %r14d
14086538c: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140865390: 80 bd c0 00 00 00 00        	cmpb	$0x0, 0xc0(%rbp)
140865397: 0f 85 a3 00 00 00           	jne	0x140865440 <.text+0x864440>
14086539d: 4d 39 f7                    	cmpq	%r14, %r15
1408653a0: 0f 85 82 00 00 00           	jne	0x140865428 <.text+0x864428>
1408653a6: 4d 39 e5                    	cmpq	%r12, %r13
1408653a9: 0f 84 a9 02 00 00           	je	0x140865658 <.text+0x864658>
1408653af: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
1408653b6: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
1408653bd: 0f 1f 00                    	nopl	(%rax)
1408653c0: 41 8b 4d 00                 	movl	(%r13), %ecx
1408653c4: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1408653c8: 48 c1 e1 05                 	shlq	$0x5, %rcx
1408653cc: 4c 8b 74 08 58              	movq	0x58(%rax,%rcx), %r14
1408653d1: 4d 85 f6                    	testq	%r14, %r14
1408653d4: 75 1a                       	jne	0x1408653f0 <.text+0x8643f0>
1408653d6: 49 83 c5 04                 	addq	$0x4, %r13
1408653da: 4d 39 e5                    	cmpq	%r12, %r13
1408653dd: 75 e1                       	jne	0x1408653c0 <.text+0x8643c0>
1408653df: e9 74 02 00 00              	jmp	0x140865658 <.text+0x864658>
1408653e4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1408653f0: 48 01 c8                    	addq	%rcx, %rax
1408653f3: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
1408653fa: 48 8b 89 18 01 00 00        	movq	0x118(%rcx), %rcx
140865401: 48 39 88 88 00 00 00        	cmpq	%rcx, 0x88(%rax)
140865408: 76 0c                       	jbe	0x140865416 <.text+0x864416>
14086540a: 48 8b 90 80 00 00 00        	movq	0x80(%rax), %rdx
140865411: 48 83 3c ca 00              	cmpq	$0x0, (%rdx,%rcx,8)
140865416: 49 83 c5 04                 	addq	$0x4, %r13
14086541a: 48 8b 40 50                 	movq	0x50(%rax), %rax
14086541e: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
140865425: 45 31 ff                    	xorl	%r15d, %r15d
140865428: 4c 89 f8                    	movq	%r15, %rax
14086542b: 48 c1 e0 04                 	shlq	$0x4, %rax
14086542f: 48 03 85 b8 00 00 00        	addq	0xb8(%rbp), %rax
140865436: eb 75                       	jmp	0x1408654ad <.text+0x8644ad>
140865438: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140865440: 4d 39 f7                    	cmpq	%r14, %r15
140865443: 75 60                       	jne	0x1408654a5 <.text+0x8644a5>
140865445: 4d 39 e5                    	cmpq	%r12, %r13
140865448: 0f 84 0a 02 00 00           	je	0x140865658 <.text+0x864658>
14086544e: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
140865455: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
14086545c: 0f 1f 40 00                 	nopl	(%rax)
140865460: 41 8b 4d 00                 	movl	(%r13), %ecx
140865464: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140865468: 4c 8b 74 c8 10              	movq	0x10(%rax,%rcx,8), %r14
14086546d: 4d 85 f6                    	testq	%r14, %r14
140865470: 75 0e                       	jne	0x140865480 <.text+0x864480>
140865472: 49 83 c5 04                 	addq	$0x4, %r13
140865476: 4d 39 e5                    	cmpq	%r12, %r13
140865479: 75 e5                       	jne	0x140865460 <.text+0x864460>
14086547b: e9 d8 01 00 00              	jmp	0x140865658 <.text+0x864658>
140865480: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
140865484: 48 8b 48 40                 	movq	0x40(%rax), %rcx
140865488: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
14086548f: 48 3b 8a 18 01 00 00        	cmpq	0x118(%rdx), %rcx
140865496: 49 83 c5 04                 	addq	$0x4, %r13
14086549a: 48 8b 40 08                 	movq	0x8(%rax), %rax
14086549e: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1408654a2: 45 31 ff                    	xorl	%r15d, %r15d
1408654a5: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1408654a9: 4a 8d 04 f8                 	leaq	(%rax,%r15,8), %rax
1408654ad: 48 8b 18                    	movq	(%rax), %rbx
1408654b0: 48 8b 06                    	movq	(%rsi), %rax
1408654b3: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
1408654b7: 48 29 f8                    	subq	%rdi, %rax
1408654ba: 48 83 f8 17                 	cmpq	$0x17, %rax
1408654be: 76 3d                       	jbe	0x1408654fd <.text+0x8644fd>
1408654c0: 49 ff c7                    	incq	%r15
1408654c3: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1408654c7: 48 8d 0d a2 79 a7 00        	leaq	0xa779a2(%rip), %rcx    # 0x1412dce70 <.text+0x12dbe70>
1408654ce: 48 89 0c 38                 	movq	%rcx, (%rax,%rdi)
1408654d2: 48 8d 0d f7 86 a7 00        	leaq	0xa786f7(%rip), %rcx    # 0x1412ddbd0 <.text+0x12dcbd0>
1408654d9: 48 89 4c 38 08              	movq	%rcx, 0x8(%rax,%rdi)
1408654de: 48 89 5c 38 10              	movq	%rbx, 0x10(%rax,%rdi)
1408654e3: 48 83 c7 18                 	addq	$0x18, %rdi
1408654e7: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
1408654eb: 80 bd c0 00 00 00 00        	cmpb	$0x0, 0xc0(%rbp)
1408654f2: 0f 84 a5 fe ff ff           	je	0x14086539d <.text+0x86439d>
1408654f8: e9 43 ff ff ff              	jmp	0x140865440 <.text+0x864440>
1408654fd: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140865506: 41 b8 18 00 00 00           	movl	$0x18, %r8d
14086550c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140865512: 48 89 f1                    	movq	%rsi, %rcx
140865515: 48 89 fa                    	movq	%rdi, %rdx
140865518: e8 43 45 04 02              	callq	0x1428a9a60 <.text+0x28a8a60>
14086551d: eb a1                       	jmp	0x1408654c0 <.text+0x8644c0>
14086551f: 48 89 9d b0 00 00 00        	movq	%rbx, 0xb0(%rbp)
140865526: 4c 89 b5 a0 00 00 00        	movq	%r14, 0xa0(%rbp)
14086552d: 49 8b 04 24                 	movq	(%r12), %rax
140865531: 4d 8b 5c 24 08              	movq	0x8(%r12), %r11
140865536: 48 8b 08                    	movq	(%rax), %rcx
140865539: 31 f6                       	xorl	%esi, %esi
14086553b: 48 89 ca                    	movq	%rcx, %rdx
14086553e: 49 2b 53 18                 	subq	0x18(%r11), %rdx
140865542: 48 0f 42 d6                 	cmovbq	%rsi, %rdx
140865546: 49 2b 4b 38                 	subq	0x38(%r11), %rcx
14086554a: 48 0f 42 ce                 	cmovbq	%rsi, %rcx
14086554e: 4d 8b 53 08                 	movq	0x8(%r11), %r10
140865552: 4d 8b 4b 30                 	movq	0x30(%r11), %r9
140865556: 4d 89 c8                    	movq	%r9, %r8
140865559: 49 29 c8                    	subq	%rcx, %r8
14086555c: 4c 0f 42 c6                 	cmovbq	%rsi, %r8
140865560: 4d 8b 7b 10                 	movq	0x10(%r11), %r15
140865564: 41 0f 97 c4                 	seta	%r12b
140865568: 4c 89 fb                    	movq	%r15, %rbx
14086556b: 48 29 d3                    	subq	%rdx, %rbx
14086556e: 48 0f 42 de                 	cmovbq	%rsi, %rbx
140865572: 49 8b 73 28                 	movq	0x28(%r11), %rsi
140865576: 49 8b 7b 40                 	movq	0x40(%r11), %rdi
14086557a: 41 0f 97 c5                 	seta	%r13b
14086557e: 49 89 fb                    	movq	%rdi, %r11
140865581: 4d 29 c3                    	subq	%r8, %r11
140865584: 49 29 db                    	subq	%rbx, %r11
140865587: 4c 89 18                    	movq	%r11, (%rax)
14086558a: 45 08 e5                    	orb	%r12b, %r13b
14086558d: 0f 85 18 01 00 00           	jne	0x1408656ab <.text+0x8646ab>
140865593: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
14086559a: 4c 8b 11                    	movq	(%rcx), %r10
14086559d: 48 8b 41 08                 	movq	0x8(%rcx), %rax
1408655a1: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1408655a8: 8b 41 14                    	movl	0x14(%rcx), %eax
1408655ab: 89 45 70                    	movl	%eax, 0x70(%rbp)
1408655ae: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1408655b5: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1408655bc: 49 8b ba f8 00 00 00        	movq	0xf8(%r10), %rdi
1408655c3: 49 8b 92 00 01 00 00        	movq	0x100(%r10), %rdx
1408655ca: 4c 8d 34 97                 	leaq	(%rdi,%rdx,4), %r14
1408655ce: 45 0f b6 9a 08 01 00 00     	movzbl	0x108(%r10), %r11d
1408655d6: 48 8b 09                    	movq	(%rcx), %rcx
1408655d9: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
1408655e0: 48 8b 00                    	movq	(%rax), %rax
1408655e3: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
1408655ea: b8 08 00 00 00              	movl	$0x8, %eax
1408655ef: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1408655f3: 31 db                       	xorl	%ebx, %ebx
1408655f5: f3 44 0f 10 0d 72 02 0c 02  	movss	0x20c0272(%rip), %xmm9  # 0x142925870
1408655fe: f3 44 0f 10 15 89 85 18 02  	movss	0x2188589(%rip), %xmm10 # 0x1429edb90
140865607: f3 44 0f 10 1d c0 07 23 02  	movss	0x22307c0(%rip), %xmm11 # 0x142a95dd0
140865610: f3 44 0f 10 25 eb 02 0c 02  	movss	0x20c02eb(%rip), %xmm12 # 0x142925904
140865619: f3 0f 10 35 b7 02 0c 02     	movss	0x20c02b7(%rip), %xmm6  # 0x1429258d8
140865621: f3 0f 10 3d c7 90 0c 02     	movss	0x20c90c7(%rip), %xmm7  # 0x14292e6f0
140865629: f3 44 0f 10 2d 46 7a 16 02  	movss	0x2167a46(%rip), %xmm13 # 0x1429cd078
140865632: 45 31 ff                    	xorl	%r15d, %r15d
140865635: 45 31 c9                    	xorl	%r9d, %r9d
140865638: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
140865643: 31 c9                       	xorl	%ecx, %ecx
140865645: 44 88 9d a8 00 00 00        	movb	%r11b, 0xa8(%rbp)
14086564c: 4c 89 95 a0 00 00 00        	movq	%r10, 0xa0(%rbp)
140865653: e9 88 07 00 00              	jmp	0x140865de0 <.text+0x864de0>
140865658: 80 bd a0 00 00 00 00        	cmpb	$0x0, 0xa0(%rbp)
14086565f: 0f 84 33 0b 00 00           	je	0x140866198 <.text+0x865198>
140865665: e8 c6 2a 3b 00              	callq	0x140c18130 <.text+0xc17130>
14086566a: 48 89 c1                    	movq	%rax, %rcx
14086566d: 48 8b 00                    	movq	(%rax), %rax
140865670: 48 83 f8 01                 	cmpq	$0x1, %rax
140865674: 0f 85 b0 0b 00 00           	jne	0x14086622a <.text+0x86522a>
14086567a: 48 83 c1 08                 	addq	$0x8, %rcx
14086567e: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140865682: 0f 85 c6 0b 00 00           	jne	0x14086624e <.text+0x86524e>
140865688: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14086568f: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
140865696: 48 83 c1 08                 	addq	$0x8, %rcx
14086569a: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
1408656a1: e8 4a 1a 3b 00              	callq	0x140c170f0 <.text+0xc160f0>
1408656a6: e9 e3 0a 00 00              	jmp	0x14086618e <.text+0x86518e>
1408656ab: 49 89 cb                    	movq	%rcx, %r11
1408656ae: 49 c1 e3 05                 	shlq	$0x5, %r11
1408656b2: 4c 01 de                    	addq	%r11, %rsi
1408656b5: 49 c1 e0 05                 	shlq	$0x5, %r8
1408656b9: 49 89 d3                    	movq	%rdx, %r11
1408656bc: 49 c1 e3 05                 	shlq	$0x5, %r11
1408656c0: 4d 01 da                    	addq	%r11, %r10
1408656c3: 48 c1 e3 05                 	shlq	$0x5, %rbx
1408656c7: 45 31 db                    	xorl	%r11d, %r11d
1408656ca: 49 39 d7                    	cmpq	%rdx, %r15
1408656cd: 41 be 08 00 00 00           	movl	$0x8, %r14d
1408656d3: 4d 0f 42 d6                 	cmovbq	%r14, %r10
1408656d7: 49 8d 54 1a e0              	leaq	-0x20(%r10,%rbx), %rdx
1408656dc: 41 ba 00 00 00 00           	movl	$0x0, %r10d
1408656e2: 4c 89 95 b8 00 00 00        	movq	%r10, 0xb8(%rbp)
1408656e9: 49 0f 46 d3                 	cmovbeq	%r11, %rdx
1408656ed: 49 39 c9                    	cmpq	%rcx, %r9
1408656f0: b9 08 00 00 00              	movl	$0x8, %ecx
1408656f5: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
1408656f9: 49 0f 42 f6                 	cmovbq	%r14, %rsi
1408656fd: 4e 8d 5c 06 e0              	leaq	-0x20(%rsi,%r8), %r11
140865702: 4c 0f 46 da                 	cmovbeq	%rdx, %r11
140865706: 48 89 38                    	movq	%rdi, (%rax)
140865709: 4c 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %r9
140865710: 49 8b 09                    	movq	(%r9), %rcx
140865713: 4c 8b a1 f8 00 00 00        	movq	0xf8(%rcx), %r12
14086571a: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140865721: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
140865728: 4c 8b 7a 08                 	movq	0x8(%rdx), %r15
14086572c: 4c 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %r8
140865733: 4d 8b 00                    	movq	(%r8), %r8
140865736: 4d 85 ff                    	testq	%r15, %r15
140865739: 75 04                       	jne	0x14086573f <.text+0x86473f>
14086573b: 4c 8b 7a 10                 	movq	0x10(%rdx), %r15
14086573f: 49 8b 51 08                 	movq	0x8(%r9), %rdx
140865743: 48 89 95 c0 00 00 00        	movq	%rdx, 0xc0(%rbp)
14086574a: 41 8b 51 14                 	movl	0x14(%r9), %edx
14086574e: 89 55 70                    	movl	%edx, 0x70(%rbp)
140865751: 49 8d 3c 84                 	leaq	(%r12,%rax,4), %rdi
140865755: 48 89 8d a0 00 00 00        	movq	%rcx, 0xa0(%rbp)
14086575c: 0f b6 99 08 01 00 00        	movzbl	0x108(%rcx), %ebx
140865763: 49 8d 43 0c                 	leaq	0xc(%r11), %rax
140865767: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14086576b: 4c 89 45 d8                 	movq	%r8, -0x28(%rbp)
14086576f: 49 8d 40 04                 	leaq	0x4(%r8), %rax
140865773: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140865777: b8 08 00 00 00              	movl	$0x8, %eax
14086577c: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
140865780: 45 31 f6                    	xorl	%r14d, %r14d
140865783: 45 31 ed                    	xorl	%r13d, %r13d
140865786: 48 c7 85 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbp)
140865791: c7 85 a8 00 00 00 00 00 00 00       	movl	$0x0, 0xa8(%rbp)
14086579b: 88 9d 80 00 00 00           	movb	%bl, 0x80(%rbp)
1408657a1: 84 db                       	testb	%bl, %bl
1408657a3: 74 4b                       	je	0x1408657f0 <.text+0x8647f0>
1408657a5: 4d 39 f5                    	cmpq	%r14, %r13
1408657a8: 0f 85 3e 02 00 00           	jne	0x1408659ec <.text+0x8649ec>
1408657ae: 49 39 fc                    	cmpq	%rdi, %r12
1408657b1: 0f 84 67 0a 00 00           	je	0x14086621e <.text+0x86521e>
1408657b7: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
1408657be: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
1408657c5: 4c 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %r10
1408657cc: 0f 1f 40 00                 	nopl	(%rax)
1408657d0: 41 8b 0c 24                 	movl	(%r12), %ecx
1408657d4: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1408657d8: 4c 8b 74 c8 10              	movq	0x10(%rax,%rcx,8), %r14
1408657dd: 4d 85 f6                    	testq	%r14, %r14
1408657e0: 75 62                       	jne	0x140865844 <.text+0x864844>
1408657e2: 49 83 c4 04                 	addq	$0x4, %r12
1408657e6: 49 39 fc                    	cmpq	%rdi, %r12
1408657e9: 75 e5                       	jne	0x1408657d0 <.text+0x8647d0>
1408657eb: e9 be fd ff ff              	jmp	0x1408655ae <.text+0x8645ae>
1408657f0: 4d 39 f5                    	cmpq	%r14, %r13
1408657f3: 0f 85 bb 02 00 00           	jne	0x140865ab4 <.text+0x864ab4>
1408657f9: 49 39 fc                    	cmpq	%rdi, %r12
1408657fc: 0f 84 1c 0a 00 00           	je	0x14086621e <.text+0x86521e>
140865802: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140865809: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
140865810: 4c 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %r10
140865817: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140865820: 41 8b 0c 24                 	movl	(%r12), %ecx
140865824: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140865828: 48 c1 e1 05                 	shlq	$0x5, %rcx
14086582c: 4c 8b 74 08 58              	movq	0x58(%rax,%rcx), %r14
140865831: 4d 85 f6                    	testq	%r14, %r14
140865834: 75 67                       	jne	0x14086589d <.text+0x86489d>
140865836: 49 83 c4 04                 	addq	$0x4, %r12
14086583a: 49 39 fc                    	cmpq	%rdi, %r12
14086583d: 75 e1                       	jne	0x140865820 <.text+0x864820>
14086583f: e9 6a fd ff ff              	jmp	0x1408655ae <.text+0x8645ae>
140865844: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
140865848: 49 8b 92 10 01 00 00        	movq	0x110(%r10), %rdx
14086584f: 48 8b 48 40                 	movq	0x40(%rax), %rcx
140865853: 48 39 d1                    	cmpq	%rdx, %rcx
140865856: 0f 86 b3 00 00 00           	jbe	0x14086590f <.text+0x86490f>
14086585c: 4c 8b 40 38                 	movq	0x38(%rax), %r8
140865860: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
140865864: 48 85 d2                    	testq	%rdx, %rdx
140865867: 0f 84 a2 00 00 00           	je	0x14086590f <.text+0x86490f>
14086586d: 4c 8b 40 18                 	movq	0x18(%rax), %r8
140865871: 48 f7 d2                    	notq	%rdx
140865874: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140865878: 48 c1 e2 04                 	shlq	$0x4, %rdx
14086587c: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
140865881: 48 89 95 b8 00 00 00        	movq	%rdx, 0xb8(%rbp)
140865888: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
14086588f: 48 39 d1                    	cmpq	%rdx, %rcx
140865892: 0f 87 92 00 00 00           	ja	0x14086592a <.text+0x86492a>
140865898: e9 36 01 00 00              	jmp	0x1408659d3 <.text+0x8649d3>
14086589d: 48 01 c8                    	addq	%rcx, %rax
1408658a0: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
1408658a6: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
1408658ad: 4c 8b 82 a8 01 00 00        	movq	0x1a8(%rdx), %r8
1408658b4: 4c 8d 0c c9                 	leaq	(%rcx,%rcx,8), %r9
1408658b8: 4b 8d 0c c8                 	leaq	(%r8,%r9,8), %rcx
1408658bc: 49 8b 92 10 01 00 00        	movq	0x110(%r10), %rdx
1408658c3: 4b 39 54 c8 40              	cmpq	%rdx, 0x40(%r8,%r9,8)
1408658c8: 0f 86 9b 00 00 00           	jbe	0x140865969 <.text+0x864969>
1408658ce: 4c 8b 41 38                 	movq	0x38(%rcx), %r8
1408658d2: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
1408658d6: 48 85 d2                    	testq	%rdx, %rdx
1408658d9: 0f 84 8a 00 00 00           	je	0x140865969 <.text+0x864969>
1408658df: 4c 8b 41 18                 	movq	0x18(%rcx), %r8
1408658e3: 48 f7 d2                    	notq	%rdx
1408658e6: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1408658ea: 48 c1 e2 04                 	shlq	$0x4, %rdx
1408658ee: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
1408658f3: 48 89 95 b8 00 00 00        	movq	%rdx, 0xb8(%rbp)
1408658fa: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
140865901: 48 39 90 88 00 00 00        	cmpq	%rdx, 0x88(%rax)
140865908: 77 7e                       	ja	0x140865988 <.text+0x864988>
14086590a: e9 8c 01 00 00              	jmp	0x140865a9b <.text+0x864a9b>
14086590f: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
14086591a: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
140865921: 48 39 d1                    	cmpq	%rdx, %rcx
140865924: 0f 86 a9 00 00 00           	jbe	0x1408659d3 <.text+0x8649d3>
14086592a: 48 8b 48 38                 	movq	0x38(%rax), %rcx
14086592e: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
140865932: 48 85 c9                    	testq	%rcx, %rcx
140865935: 0f 84 98 00 00 00           	je	0x1408659d3 <.text+0x8649d3>
14086593b: 48 8b 50 18                 	movq	0x18(%rax), %rdx
14086593f: 48 f7 d1                    	notq	%rcx
140865942: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140865946: 48 c1 e1 04                 	shlq	$0x4, %rcx
14086594a: 4c 8b 44 0a 10              	movq	0x10(%rdx,%rcx), %r8
14086594f: 4c 89 85 88 00 00 00        	movq	%r8, 0x88(%rbp)
140865956: 48 8b 4c 0a 28              	movq	0x28(%rdx,%rcx), %rcx
14086595b: 48 89 4d 00                 	movq	%rcx, (%rbp)
14086595f: b1 01                       	movb	$0x1, %cl
140865961: 89 8d a8 00 00 00           	movl	%ecx, 0xa8(%rbp)
140865967: eb 74                       	jmp	0x1408659dd <.text+0x8649dd>
140865969: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
140865974: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
14086597b: 48 39 90 88 00 00 00        	cmpq	%rdx, 0x88(%rax)
140865982: 0f 86 13 01 00 00           	jbe	0x140865a9b <.text+0x864a9b>
140865988: 4c 8b 80 80 00 00 00        	movq	0x80(%rax), %r8
14086598f: 49 83 3c d0 00              	cmpq	$0x0, (%r8,%rdx,8)
140865994: 0f 84 01 01 00 00           	je	0x140865a9b <.text+0x864a9b>
14086599a: 4c 8b 41 18                 	movq	0x18(%rcx), %r8
14086599e: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
1408659a2: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
1408659a6: 48 f7 d1                    	notq	%rcx
1408659a9: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408659ad: 48 c1 e1 04                 	shlq	$0x4, %rcx
1408659b1: 49 8b 54 08 10              	movq	0x10(%r8,%rcx), %rdx
1408659b6: 48 89 95 88 00 00 00        	movq	%rdx, 0x88(%rbp)
1408659bd: 49 8b 4c 08 28              	movq	0x28(%r8,%rcx), %rcx
1408659c2: 48 89 4d 00                 	movq	%rcx, (%rbp)
1408659c6: b1 01                       	movb	$0x1, %cl
1408659c8: 89 8d a8 00 00 00           	movl	%ecx, 0xa8(%rbp)
1408659ce: e9 d2 00 00 00              	jmp	0x140865aa5 <.text+0x864aa5>
1408659d3: c7 85 a8 00 00 00 00 00 00 00       	movl	$0x0, 0xa8(%rbp)
1408659dd: 49 83 c4 04                 	addq	$0x4, %r12
1408659e1: 48 8b 40 08                 	movq	0x8(%rax), %rax
1408659e5: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1408659e9: 45 31 ed                    	xorl	%r13d, %r13d
1408659ec: 48 8b 45 78                 	movq	0x78(%rbp), %rax
1408659f0: 4a 8b 04 e8                 	movq	(%rax,%r13,8), %rax
1408659f4: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1408659fb: 44 89 ea                    	movl	%r13d, %edx
1408659fe: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140865a05: 48 01 d0                    	addq	%rdx, %rax
140865a08: f6 85 a8 00 00 00 01        	testb	$0x1, 0xa8(%rbp)
140865a0f: 0f 84 80 00 00 00           	je	0x140865a95 <.text+0x864a95>
140865a15: 48 8d 0c 52                 	leaq	(%rdx,%rdx,2), %rcx
140865a19: 48 c1 e1 04                 	shlq	$0x4, %rcx
140865a1d: 48 03 8d 88 00 00 00        	addq	0x88(%rbp), %rcx
140865a24: 4c 8b 45 00                 	movq	(%rbp), %r8
140865a28: 49 8d 14 90                 	leaq	(%r8,%rdx,4), %rdx
140865a2c: 48 89 55 c8                 	movq	%rdx, -0x38(%rbp)
140865a30: 49 ff c5                    	incq	%r13
140865a33: 0f b6 00                    	movzbl	(%rax), %eax
140865a36: 41 3a 43 18                 	cmpb	0x18(%r11), %al
140865a3a: 0f 84 e4 00 00 00           	je	0x140865b24 <.text+0x864b24>
140865a40: 4c 89 de                    	movq	%r11, %rsi
140865a43: 49 8b 07                    	movq	(%r15), %rax
140865a46: 49 8b 5f 10                 	movq	0x10(%r15), %rbx
140865a4a: 48 29 d8                    	subq	%rbx, %rax
140865a4d: 48 83 f8 17                 	cmpq	$0x17, %rax
140865a51: 0f 86 cd 02 00 00           	jbe	0x140865d24 <.text+0x864d24>
140865a57: 49 8b 47 08                 	movq	0x8(%r15), %rax
140865a5b: 48 8d 0d 0e 74 a7 00        	leaq	0xa7740e(%rip), %rcx    # 0x1412dce70 <.text+0x12dbe70>
140865a62: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
140865a66: 48 8d 0d 63 81 a7 00        	leaq	0xa78163(%rip), %rcx    # 0x1412ddbd0 <.text+0x12dcbd0>
140865a6d: 48 89 4c 18 08              	movq	%rcx, 0x8(%rax,%rbx)
140865a72: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
140865a79: 48 89 4c 18 10              	movq	%rcx, 0x10(%rax,%rbx)
140865a7e: 48 83 c3 18                 	addq	$0x18, %rbx
140865a82: 49 89 5f 10                 	movq	%rbx, 0x10(%r15)
140865a86: 49 89 f3                    	movq	%rsi, %r11
140865a89: 0f b6 9d 80 00 00 00        	movzbl	0x80(%rbp), %ebx
140865a90: e9 0c fd ff ff              	jmp	0x1408657a1 <.text+0x8647a1>
140865a95: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
140865a99: eb 77                       	jmp	0x140865b12 <.text+0x864b12>
140865a9b: c7 85 a8 00 00 00 00 00 00 00       	movl	$0x0, 0xa8(%rbp)
140865aa5: 49 83 c4 04                 	addq	$0x4, %r12
140865aa9: 48 8b 40 50                 	movq	0x50(%rax), %rax
140865aad: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140865ab1: 45 31 ed                    	xorl	%r13d, %r13d
140865ab4: 4c 89 e8                    	movq	%r13, %rax
140865ab7: 48 c1 e0 04                 	shlq	$0x4, %rax
140865abb: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140865abf: 48 8b 14 01                 	movq	(%rcx,%rax), %rdx
140865ac3: 48 89 95 b0 00 00 00        	movq	%rdx, 0xb0(%rbp)
140865aca: 8b 54 01 08                 	movl	0x8(%rcx,%rax), %edx
140865ace: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140865ad5: 48 01 d0                    	addq	%rdx, %rax
140865ad8: f6 85 a8 00 00 00 01        	testb	$0x1, 0xa8(%rbp)
140865adf: 74 2d                       	je	0x140865b0e <.text+0x864b0e>
140865ae1: 48 8d 0c 52                 	leaq	(%rdx,%rdx,2), %rcx
140865ae5: 48 c1 e1 04                 	shlq	$0x4, %rcx
140865ae9: 48 03 8d 88 00 00 00        	addq	0x88(%rbp), %rcx
140865af0: 4c 8b 45 00                 	movq	(%rbp), %r8
140865af4: 49 8d 14 90                 	leaq	(%r8,%rdx,4), %rdx
140865af8: 48 89 55 d0                 	movq	%rdx, -0x30(%rbp)
140865afc: 49 ff c5                    	incq	%r13
140865aff: 0f b6 00                    	movzbl	(%rax), %eax
140865b02: 41 3a 43 18                 	cmpb	0x18(%r11), %al
140865b06: 0f 85 34 ff ff ff           	jne	0x140865a40 <.text+0x864a40>
140865b0c: eb 16                       	jmp	0x140865b24 <.text+0x864b24>
140865b0e: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
140865b12: 31 c9                       	xorl	%ecx, %ecx
140865b14: 49 ff c5                    	incq	%r13
140865b17: 0f b6 00                    	movzbl	(%rax), %eax
140865b1a: 41 3a 43 18                 	cmpb	0x18(%r11), %al
140865b1e: 0f 85 1c ff ff ff           	jne	0x140865a40 <.text+0x864a40>
140865b24: 41 f6 43 08 01              	testb	$0x1, 0x8(%r11)
140865b29: 74 2f                       	je	0x140865b5a <.text+0x864b5a>
140865b2b: 48 85 c9                    	testq	%rcx, %rcx
140865b2e: 74 63                       	je	0x140865b93 <.text+0x864b93>
140865b30: 4c 8b 45 f8                 	movq	-0x8(%rbp), %r8
140865b34: 41 8b 40 08                 	movl	0x8(%r8), %eax
140865b38: 89 45 18                    	movl	%eax, 0x18(%rbp)
140865b3b: 49 8b 00                    	movq	(%r8), %rax
140865b3e: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140865b42: 8b 45 70                    	movl	0x70(%rbp), %eax
140865b45: 89 02                       	movl	%eax, (%rdx)
140865b47: 8b 45 18                    	movl	0x18(%rbp), %eax
140865b4a: 89 41 18                    	movl	%eax, 0x18(%rcx)
140865b4d: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140865b51: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
140865b55: e9 47 fc ff ff              	jmp	0x1408657a1 <.text+0x8647a1>
140865b5a: 48 85 c9                    	testq	%rcx, %rcx
140865b5d: 0f 85 3e fc ff ff           	jne	0x1408657a1 <.text+0x8647a1>
140865b63: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
140865b67: f6 00 01                    	testb	$0x1, (%rax)
140865b6a: 4c 89 5d 08                 	movq	%r11, 0x8(%rbp)
140865b6e: 4c 89 fe                    	movq	%r15, %rsi
140865b71: 0f 84 e6 00 00 00           	je	0x140865c5d <.text+0x864c5d>
140865b77: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
140865b7b: 8b 41 08                    	movl	0x8(%rcx), %eax
140865b7e: 89 85 98 00 00 00           	movl	%eax, 0x98(%rbp)
140865b84: 48 8b 01                    	movq	(%rcx), %rax
140865b87: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
140865b8e: e9 df 00 00 00              	jmp	0x140865c72 <.text+0x864c72>
140865b93: 4c 89 5d 08                 	movq	%r11, 0x8(%rbp)
140865b97: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140865b9b: 8b 41 08                    	movl	0x8(%rcx), %eax
140865b9e: 89 85 98 00 00 00           	movl	%eax, 0x98(%rbp)
140865ba4: 48 8b 01                    	movq	(%rcx), %rax
140865ba7: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
140865bae: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140865bb2: e8 c9 75 43 00              	callq	0x140c9d180 <.text+0xc9c180>
140865bb7: 8b 85 98 00 00 00           	movl	0x98(%rbp), %eax
140865bbd: 89 45 58                    	movl	%eax, 0x58(%rbp)
140865bc0: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
140865bc7: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140865bcb: 0f 28 75 10                 	movaps	0x10(%rbp), %xmm6
140865bcf: 48 8d 4d 2c                 	leaq	0x2c(%rbp), %rcx
140865bd3: 48 8b 01                    	movq	(%rcx), %rax
140865bd6: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140865bda: 8b 41 08                    	movl	0x8(%rcx), %eax
140865bdd: 89 45 48                    	movl	%eax, 0x48(%rbp)
140865be0: 49 8b 07                    	movq	(%r15), %rax
140865be3: 49 8b 5f 10                 	movq	0x10(%r15), %rbx
140865be7: 48 29 d8                    	subq	%rbx, %rax
140865bea: 48 83 f8 67                 	cmpq	$0x67, %rax
140865bee: 0f 86 55 01 00 00           	jbe	0x140865d49 <.text+0x864d49>
140865bf4: 49 8b 47 08                 	movq	0x8(%r15), %rax
140865bf8: 48 8d 0d c1 5c a7 00        	leaq	0xa75cc1(%rip), %rcx    # 0x1412db8c0 <.text+0x12da8c0>
140865bff: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
140865c03: 0f 11 74 18 08              	movups	%xmm6, 0x8(%rax,%rbx)
140865c08: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140865c0c: 48 89 4c 18 18              	movq	%rcx, 0x18(%rax,%rbx)
140865c11: 8b 4d 58                    	movl	0x58(%rbp), %ecx
140865c14: 89 4c 18 20                 	movl	%ecx, 0x20(%rax,%rbx)
140865c18: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140865c1c: 48 89 4c 18 24              	movq	%rcx, 0x24(%rax,%rbx)
140865c21: 8b 4d 48                    	movl	0x48(%rbp), %ecx
140865c24: 89 4c 18 2c                 	movl	%ecx, 0x2c(%rax,%rbx)
140865c28: c6 44 18 38 00              	movb	$0x0, 0x38(%rax,%rbx)
140865c2d: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
140865c34: 48 89 4c 18 48              	movq	%rcx, 0x48(%rax,%rbx)
140865c39: 48 8d 0d f0 f0 cf 00        	leaq	0xcff0f0(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140865c40: 48 89 4c 18 58              	movq	%rcx, 0x58(%rax,%rbx)
140865c45: 48 83 c3 68                 	addq	$0x68, %rbx
140865c49: 49 89 5f 10                 	movq	%rbx, 0x10(%r15)
140865c4d: 4c 8b 5d 08                 	movq	0x8(%rbp), %r11
140865c51: 0f b6 9d 80 00 00 00        	movzbl	0x80(%rbp), %ebx
140865c58: e9 44 fb ff ff              	jmp	0x1408657a1 <.text+0x8647a1>
140865c5d: c7 85 98 00 00 00 00 00 00 00       	movl	$0x0, 0x98(%rbp)
140865c67: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
140865c72: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140865c76: e8 05 75 43 00              	callq	0x140c9d180 <.text+0xc9c180>
140865c7b: 8b 85 98 00 00 00           	movl	0x98(%rbp), %eax
140865c81: 89 45 58                    	movl	%eax, 0x58(%rbp)
140865c84: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
140865c8b: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140865c8f: 0f 28 75 10                 	movaps	0x10(%rbp), %xmm6
140865c93: 48 8d 4d 2c                 	leaq	0x2c(%rbp), %rcx
140865c97: 48 8b 01                    	movq	(%rcx), %rax
140865c9a: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140865c9e: 8b 41 08                    	movl	0x8(%rcx), %eax
140865ca1: 89 45 48                    	movl	%eax, 0x48(%rbp)
140865ca4: 48 8b 06                    	movq	(%rsi), %rax
140865ca7: 48 8b 5e 10                 	movq	0x10(%rsi), %rbx
140865cab: 48 29 d8                    	subq	%rbx, %rax
140865cae: 48 83 f8 67                 	cmpq	$0x67, %rax
140865cb2: 0f 86 b6 00 00 00           	jbe	0x140865d6e <.text+0x864d6e>
140865cb8: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140865cbc: 48 8d 0d fd 5b a7 00        	leaq	0xa75bfd(%rip), %rcx    # 0x1412db8c0 <.text+0x12da8c0>
140865cc3: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
140865cc7: 0f 11 74 18 08              	movups	%xmm6, 0x8(%rax,%rbx)
140865ccc: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140865cd0: 48 89 4c 18 18              	movq	%rcx, 0x18(%rax,%rbx)
140865cd5: 8b 4d 58                    	movl	0x58(%rbp), %ecx
140865cd8: 89 4c 18 20                 	movl	%ecx, 0x20(%rax,%rbx)
140865cdc: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140865ce0: 48 89 4c 18 24              	movq	%rcx, 0x24(%rax,%rbx)
140865ce5: 8b 4d 48                    	movl	0x48(%rbp), %ecx
140865ce8: 89 4c 18 2c                 	movl	%ecx, 0x2c(%rax,%rbx)
140865cec: c6 44 18 38 00              	movb	$0x0, 0x38(%rax,%rbx)
140865cf1: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
140865cf8: 48 89 4c 18 48              	movq	%rcx, 0x48(%rax,%rbx)
140865cfd: 48 8d 0d 2c f0 cf 00        	leaq	0xcff02c(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140865d04: 48 89 4c 18 58              	movq	%rcx, 0x58(%rax,%rbx)
140865d09: 48 83 c3 68                 	addq	$0x68, %rbx
140865d0d: 48 89 5e 10                 	movq	%rbx, 0x10(%rsi)
140865d11: 4c 8b 5d 08                 	movq	0x8(%rbp), %r11
140865d15: 49 89 f7                    	movq	%rsi, %r15
140865d18: 0f b6 9d 80 00 00 00        	movzbl	0x80(%rbp), %ebx
140865d1f: e9 7d fa ff ff              	jmp	0x1408657a1 <.text+0x8647a1>
140865d24: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140865d2d: 41 b8 18 00 00 00           	movl	$0x18, %r8d
140865d33: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140865d39: 4c 89 f9                    	movq	%r15, %rcx
140865d3c: 48 89 da                    	movq	%rbx, %rdx
140865d3f: e8 1c 3d 04 02              	callq	0x1428a9a60 <.text+0x28a8a60>
140865d44: e9 0e fd ff ff              	jmp	0x140865a57 <.text+0x864a57>
140865d49: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140865d52: 41 b8 68 00 00 00           	movl	$0x68, %r8d
140865d58: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140865d5e: 4c 89 f9                    	movq	%r15, %rcx
140865d61: 48 89 da                    	movq	%rbx, %rdx
140865d64: e8 f7 3c 04 02              	callq	0x1428a9a60 <.text+0x28a8a60>
140865d69: e9 86 fe ff ff              	jmp	0x140865bf4 <.text+0x864bf4>
140865d6e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140865d77: 41 b8 68 00 00 00           	movl	$0x68, %r8d
140865d7d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140865d83: 48 89 f1                    	movq	%rsi, %rcx
140865d86: 48 89 da                    	movq	%rbx, %rdx
140865d89: e8 d2 3c 04 02              	callq	0x1428a9a60 <.text+0x28a8a60>
140865d8e: e9 25 ff ff ff              	jmp	0x140865cb8 <.text+0x864cb8>
140865d93: f3 45 0f 59 c5              	mulss	%xmm13, %xmm8
140865d98: f3 45 0f 59 c1              	mulss	%xmm9, %xmm8
140865d9d: f3 44 0f 11 45 10           	movss	%xmm8, 0x10(%rbp)
140865da3: f3 44 0f 11 45 14           	movss	%xmm8, 0x14(%rbp)
140865da9: f3 44 0f 11 45 18           	movss	%xmm8, 0x18(%rbp)
140865daf: 8b 45 70                    	movl	0x70(%rbp), %eax
140865db2: 41 89 04 24                 	movl	%eax, (%r12)
140865db6: 8b 45 18                    	movl	0x18(%rbp), %eax
140865db9: 41 89 45 24                 	movl	%eax, 0x24(%r13)
140865dbd: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140865dc1: 49 89 45 1c                 	movq	%rax, 0x1c(%r13)
140865dc5: 4c 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %r10
140865dcc: 44 0f b6 9d a8 00 00 00     	movzbl	0xa8(%rbp), %r11d
140865dd4: 49 89 f1                    	movq	%rsi, %r9
140865dd7: 8b 8d b0 00 00 00           	movl	0xb0(%rbp), %ecx
140865ddd: 49 ff c7                    	incq	%r15
140865de0: 45 84 db                    	testb	%r11b, %r11b
140865de3: 74 4b                       	je	0x140865e30 <.text+0x864e30>
140865de5: 49 39 df                    	cmpq	%rbx, %r15
140865de8: 0f 85 e6 01 00 00           	jne	0x140865fd4 <.text+0x864fd4>
140865dee: 4c 39 f7                    	cmpq	%r14, %rdi
140865df1: 0f 84 52 03 00 00           	je	0x140866149 <.text+0x865149>
140865df7: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140865dfe: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
140865e05: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140865e10: 8b 0f                       	movl	(%rdi), %ecx
140865e12: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140865e16: 48 8b 5c c8 10              	movq	0x10(%rax,%rcx,8), %rbx
140865e1b: 48 85 db                    	testq	%rbx, %rbx
140865e1e: 75 52                       	jne	0x140865e72 <.text+0x864e72>
140865e20: 48 83 c7 04                 	addq	$0x4, %rdi
140865e24: 4c 39 f7                    	cmpq	%r14, %rdi
140865e27: 75 e7                       	jne	0x140865e10 <.text+0x864e10>
140865e29: e9 1b 03 00 00              	jmp	0x140866149 <.text+0x865149>
140865e2e: 66 90                       	nop
140865e30: 49 39 df                    	cmpq	%rbx, %r15
140865e33: 0f 85 c0 01 00 00           	jne	0x140865ff9 <.text+0x864ff9>
140865e39: 4c 39 f7                    	cmpq	%r14, %rdi
140865e3c: 0f 84 07 03 00 00           	je	0x140866149 <.text+0x865149>
140865e42: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140865e49: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
140865e50: 8b 0f                       	movl	(%rdi), %ecx
140865e52: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140865e56: 48 c1 e1 05                 	shlq	$0x5, %rcx
140865e5a: 48 8b 5c 08 58              	movq	0x58(%rax,%rcx), %rbx
140865e5f: 48 85 db                    	testq	%rbx, %rbx
140865e62: 75 64                       	jne	0x140865ec8 <.text+0x864ec8>
140865e64: 48 83 c7 04                 	addq	$0x4, %rdi
140865e68: 4c 39 f7                    	cmpq	%r14, %rdi
140865e6b: 75 e3                       	jne	0x140865e50 <.text+0x864e50>
140865e6d: e9 d7 02 00 00              	jmp	0x140866149 <.text+0x865149>
140865e72: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
140865e76: 49 8b 92 10 01 00 00        	movq	0x110(%r10), %rdx
140865e7d: 48 8b 48 40                 	movq	0x40(%rax), %rcx
140865e81: 48 39 d1                    	cmpq	%rdx, %rcx
140865e84: 0f 86 a5 00 00 00           	jbe	0x140865f2f <.text+0x864f2f>
140865e8a: 4c 8b 40 38                 	movq	0x38(%rax), %r8
140865e8e: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
140865e92: 48 85 d2                    	testq	%rdx, %rdx
140865e95: 0f 84 94 00 00 00           	je	0x140865f2f <.text+0x864f2f>
140865e9b: 4c 8b 40 18                 	movq	0x18(%rax), %r8
140865e9f: 48 f7 d2                    	notq	%rdx
140865ea2: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140865ea6: 48 c1 e2 04                 	shlq	$0x4, %rdx
140865eaa: 4d 8b 4c 10 10              	movq	0x10(%r8,%rdx), %r9
140865eaf: 48 83 c7 04                 	addq	$0x4, %rdi
140865eb3: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
140865eba: 48 39 d1                    	cmpq	%rdx, %rcx
140865ebd: 0f 87 83 00 00 00           	ja	0x140865f46 <.text+0x864f46>
140865ec3: e9 07 01 00 00              	jmp	0x140865fcf <.text+0x864fcf>
140865ec8: 48 01 c8                    	addq	%rcx, %rax
140865ecb: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
140865ed1: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
140865ed8: 4c 8b 82 a8 01 00 00        	movq	0x1a8(%rdx), %r8
140865edf: 4c 8d 0c c9                 	leaq	(%rcx,%rcx,8), %r9
140865ee3: 4b 8d 0c c8                 	leaq	(%r8,%r9,8), %rcx
140865ee7: 49 8b 92 10 01 00 00        	movq	0x110(%r10), %rdx
140865eee: 4b 39 54 c8 40              	cmpq	%rdx, 0x40(%r8,%r9,8)
140865ef3: 0f 86 85 00 00 00           	jbe	0x140865f7e <.text+0x864f7e>
140865ef9: 4c 8b 41 38                 	movq	0x38(%rcx), %r8
140865efd: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
140865f01: 48 85 d2                    	testq	%rdx, %rdx
140865f04: 74 78                       	je	0x140865f7e <.text+0x864f7e>
140865f06: 4c 8b 41 18                 	movq	0x18(%rcx), %r8
140865f0a: 48 f7 d2                    	notq	%rdx
140865f0d: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140865f11: 48 c1 e2 04                 	shlq	$0x4, %rdx
140865f15: 4d 8b 4c 10 10              	movq	0x10(%r8,%rdx), %r9
140865f1a: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
140865f21: 48 39 90 88 00 00 00        	cmpq	%rdx, 0x88(%rax)
140865f28: 77 67                       	ja	0x140865f91 <.text+0x864f91>
140865f2a: e9 b9 00 00 00              	jmp	0x140865fe8 <.text+0x864fe8>
140865f2f: 45 31 c9                    	xorl	%r9d, %r9d
140865f32: 48 83 c7 04                 	addq	$0x4, %rdi
140865f36: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
140865f3d: 48 39 d1                    	cmpq	%rdx, %rcx
140865f40: 0f 86 89 00 00 00           	jbe	0x140865fcf <.text+0x864fcf>
140865f46: 48 8b 48 38                 	movq	0x38(%rax), %rcx
140865f4a: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
140865f4e: 48 85 c9                    	testq	%rcx, %rcx
140865f51: 74 7c                       	je	0x140865fcf <.text+0x864fcf>
140865f53: 48 8b 40 18                 	movq	0x18(%rax), %rax
140865f57: 48 f7 d1                    	notq	%rcx
140865f5a: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140865f5e: 48 c1 e1 04                 	shlq	$0x4, %rcx
140865f62: 48 8b 54 08 10              	movq	0x10(%rax,%rcx), %rdx
140865f67: 48 89 95 b8 00 00 00        	movq	%rdx, 0xb8(%rbp)
140865f6e: 48 8b 44 08 28              	movq	0x28(%rax,%rcx), %rax
140865f73: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
140865f77: b1 01                       	movb	$0x1, %cl
140865f79: 45 31 ff                    	xorl	%r15d, %r15d
140865f7c: eb 56                       	jmp	0x140865fd4 <.text+0x864fd4>
140865f7e: 45 31 c9                    	xorl	%r9d, %r9d
140865f81: 49 8b 92 18 01 00 00        	movq	0x118(%r10), %rdx
140865f88: 48 39 90 88 00 00 00        	cmpq	%rdx, 0x88(%rax)
140865f8f: 76 57                       	jbe	0x140865fe8 <.text+0x864fe8>
140865f91: 4c 8b 80 80 00 00 00        	movq	0x80(%rax), %r8
140865f98: 49 83 3c d0 00              	cmpq	$0x0, (%r8,%rdx,8)
140865f9d: 74 49                       	je	0x140865fe8 <.text+0x864fe8>
140865f9f: 4c 8b 41 18                 	movq	0x18(%rcx), %r8
140865fa3: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
140865fa7: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
140865fab: 48 f7 d1                    	notq	%rcx
140865fae: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140865fb2: 48 c1 e1 04                 	shlq	$0x4, %rcx
140865fb6: 49 8b 54 08 10              	movq	0x10(%r8,%rcx), %rdx
140865fbb: 48 89 95 b8 00 00 00        	movq	%rdx, 0xb8(%rbp)
140865fc2: 49 8b 4c 08 28              	movq	0x28(%r8,%rcx), %rcx
140865fc7: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
140865fcb: b1 01                       	movb	$0x1, %cl
140865fcd: eb 1b                       	jmp	0x140865fea <.text+0x864fea>
140865fcf: 45 31 ff                    	xorl	%r15d, %r15d
140865fd2: 31 c9                       	xorl	%ecx, %ecx
140865fd4: f6 c1 01                    	testb	$0x1, %cl
140865fd7: 0f 84 00 fe ff ff           	je	0x140865ddd <.text+0x864ddd>
140865fdd: 89 8d b0 00 00 00           	movl	%ecx, 0xb0(%rbp)
140865fe3: 44 89 f9                    	movl	%r15d, %ecx
140865fe6: eb 2f                       	jmp	0x140866017 <.text+0x865017>
140865fe8: 31 c9                       	xorl	%ecx, %ecx
140865fea: 48 83 c7 04                 	addq	$0x4, %rdi
140865fee: 48 8b 40 50                 	movq	0x50(%rax), %rax
140865ff2: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140865ff6: 45 31 ff                    	xorl	%r15d, %r15d
140865ff9: f6 c1 01                    	testb	$0x1, %cl
140865ffc: 0f 84 db fd ff ff           	je	0x140865ddd <.text+0x864ddd>
140866002: 89 8d b0 00 00 00           	movl	%ecx, 0xb0(%rbp)
140866008: 4c 89 f8                    	movq	%r15, %rax
14086600b: 48 c1 e0 04                 	shlq	$0x4, %rax
14086600f: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140866013: 8b 4c 01 08                 	movl	0x8(%rcx,%rax), %ecx
140866017: 4c 89 ce                    	movq	%r9, %rsi
14086601a: 49 8d 04 09                 	leaq	(%r9,%rcx), %rax
14086601e: 4c 8d 2c 49                 	leaq	(%rcx,%rcx,2), %r13
140866022: 49 c1 e5 04                 	shlq	$0x4, %r13
140866026: 4c 03 ad b8 00 00 00        	addq	0xb8(%rbp), %r13
14086602d: 48 8b 55 78                 	movq	0x78(%rbp), %rdx
140866031: 4c 8d 24 8a                 	leaq	(%rdx,%rcx,4), %r12
140866035: 0f b6 00                    	movzbl	(%rax), %eax
140866038: 83 c0 fc                    	addl	$-0x4, %eax
14086603b: 83 f8 06                    	cmpl	$0x6, %eax
14086603e: 77 71                       	ja	0x1408660b1 <.text+0x8650b1>
140866040: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
140866047: f3 44 0f 10 01              	movss	(%rcx), %xmm8
14086604c: 48 8d 0d 8d 06 23 02        	leaq	0x223068d(%rip), %rcx   # 0x142a966e0
140866053: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140866057: 48 01 c8                    	addq	%rcx, %rax
14086605a: ff e0                       	jmpq	*%rax
14086605c: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
140866063: f3 0f 10 40 04              	movss	0x4(%rax), %xmm0
140866068: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
14086606d: eb 72                       	jmp	0x1408660e1 <.text+0x8650e1>
14086606f: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
140866073: f3 41 0f 5f c0              	maxss	%xmm8, %xmm0
140866078: f3 45 0f 59 c3              	mulss	%xmm11, %xmm8
14086607d: f3 45 0f 59 c1              	mulss	%xmm9, %xmm8
140866082: f3 44 0f 11 85 90 00 00 00  	movss	%xmm8, 0x90(%rbp)
14086608b: f3 44 0f 11 85 94 00 00 00  	movss	%xmm8, 0x94(%rbp)
140866094: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
140866098: f3 0f 5d d0                 	minss	%xmm0, %xmm2
14086609c: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1408660a0: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
1408660a7: e8 34 6d 43 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1408660ac: e9 fe fc ff ff              	jmp	0x140865daf <.text+0x864daf>
1408660b1: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
1408660bb: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1408660bf: c7 45 18 00 00 80 3f        	movl	$0x3f800000, 0x18(%rbp) # imm = 0x3F800000
1408660c6: e9 e4 fc ff ff              	jmp	0x140865daf <.text+0x864daf>
1408660cb: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1408660d2: f3 0f 10 40 08              	movss	0x8(%rax), %xmm0
1408660d7: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
1408660dc: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
1408660e1: f3 0f 11 45 10              	movss	%xmm0, 0x10(%rbp)
1408660e6: f3 0f 11 45 14              	movss	%xmm0, 0x14(%rbp)
1408660eb: f3 0f 11 45 18              	movss	%xmm0, 0x18(%rbp)
1408660f0: e9 ba fc ff ff              	jmp	0x140865daf <.text+0x864daf>
1408660f5: 0f 28 c6                    	movaps	%xmm6, %xmm0
1408660f8: 0f 28 cf                    	movaps	%xmm7, %xmm1
1408660fb: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1408660ff: e8 9c a3 42 00              	callq	0x140c904a0 <.text+0xc8f4a0>
140866104: f3 41 0f 59 c5              	mulss	%xmm13, %xmm0
140866109: f3 41 0f 58 c4              	addss	%xmm12, %xmm0
14086610e: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
140866113: f3 0f 11 85 90 00 00 00     	movss	%xmm0, 0x90(%rbp)
14086611b: f3 0f 11 85 94 00 00 00     	movss	%xmm0, 0x94(%rbp)
140866123: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
140866127: f3 41 0f 5f c0              	maxss	%xmm8, %xmm0
14086612c: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
140866130: f3 0f 5d d0                 	minss	%xmm0, %xmm2
140866134: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140866138: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
14086613f: e8 9c 6c 43 00              	callq	0x140c9cde0 <.text+0xc9bde0>
140866144: e9 66 fc ff ff              	jmp	0x140865daf <.text+0x864daf>
140866149: 48 83 7d e8 00              	cmpq	$0x0, -0x18(%rbp)
14086614e: 74 48                       	je	0x140866198 <.text+0x865198>
140866150: e8 db 1f 3b 00              	callq	0x140c18130 <.text+0xc17130>
140866155: 48 89 c1                    	movq	%rax, %rcx
140866158: 48 8b 00                    	movq	(%rax), %rax
14086615b: 48 83 f8 01                 	cmpq	$0x1, %rax
14086615f: 0f 85 9f 00 00 00           	jne	0x140866204 <.text+0x865204>
140866165: 48 83 c1 08                 	addq	$0x8, %rcx
140866169: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14086616d: 0f 85 db 00 00 00           	jne	0x14086624e <.text+0x86524e>
140866173: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14086617a: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
140866181: 48 83 c1 08                 	addq	$0x8, %rcx
140866185: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
140866189: e8 62 0f 3b 00              	callq	0x140c170f0 <.text+0xc160f0>
14086618e: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140866195: 48 ff 00                    	incq	(%rax)
140866198: 0f 28 b5 d0 00 00 00        	movaps	0xd0(%rbp), %xmm6
14086619f: 0f 28 bd e0 00 00 00        	movaps	0xe0(%rbp), %xmm7
1408661a6: 44 0f 28 85 f0 00 00 00     	movaps	0xf0(%rbp), %xmm8
1408661ae: 44 0f 28 8d 00 01 00 00     	movaps	0x100(%rbp), %xmm9
1408661b6: 44 0f 28 95 10 01 00 00     	movaps	0x110(%rbp), %xmm10
1408661be: 44 0f 28 9d 20 01 00 00     	movaps	0x120(%rbp), %xmm11
1408661c6: 44 0f 28 a5 30 01 00 00     	movaps	0x130(%rbp), %xmm12
1408661ce: 44 0f 28 ad 40 01 00 00     	movaps	0x140(%rbp), %xmm13
1408661d6: 48 81 c4 d8 01 00 00        	addq	$0x1d8, %rsp            # imm = 0x1D8
1408661dd: 5b                          	popq	%rbx
1408661de: 5f                          	popq	%rdi
1408661df: 5e                          	popq	%rsi
1408661e0: 41 5c                       	popq	%r12
1408661e2: 41 5d                       	popq	%r13
1408661e4: 41 5e                       	popq	%r14
1408661e6: 41 5f                       	popq	%r15
1408661e8: 5d                          	popq	%rbp
1408661e9: c3                          	retq
1408661ea: 48 85 c0                    	testq	%rax, %rax
1408661ed: 75 53                       	jne	0x140866242 <.text+0x865242>
1408661ef: 31 d2                       	xorl	%edx, %edx
1408661f1: e8 da 25 09 02              	callq	0x1428f87d0 <.text+0x28f77d0>
1408661f6: 48 89 c1                    	movq	%rax, %rcx
1408661f9: 48 85 c0                    	testq	%rax, %rax
1408661fc: 0f 85 aa f0 ff ff           	jne	0x1408652ac <.text+0x8642ac>
140866202: eb 3e                       	jmp	0x140866242 <.text+0x865242>
140866204: 48 85 c0                    	testq	%rax, %rax
140866207: 75 39                       	jne	0x140866242 <.text+0x865242>
140866209: 31 d2                       	xorl	%edx, %edx
14086620b: e8 c0 25 09 02              	callq	0x1428f87d0 <.text+0x28f77d0>
140866210: 48 89 c1                    	movq	%rax, %rcx
140866213: 48 85 c0                    	testq	%rax, %rax
140866216: 0f 85 4d ff ff ff           	jne	0x140866169 <.text+0x865169>
14086621c: eb 24                       	jmp	0x140866242 <.text+0x865242>
14086621e: 4c 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %r10
140866225: e9 84 f3 ff ff              	jmp	0x1408655ae <.text+0x8645ae>
14086622a: 48 85 c0                    	testq	%rax, %rax
14086622d: 75 13                       	jne	0x140866242 <.text+0x865242>
14086622f: 31 d2                       	xorl	%edx, %edx
140866231: e8 9a 25 09 02              	callq	0x1428f87d0 <.text+0x28f77d0>
140866236: 48 89 c1                    	movq	%rax, %rcx
140866239: 48 85 c0                    	testq	%rax, %rax
14086623c: 0f 85 3c f4 ff ff           	jne	0x14086567e <.text+0x86467e>
140866242: 48 8d 0d 57 48 3d 02        	leaq	0x23d4857(%rip), %rcx   # 0x142c3aaa0
140866249: e8 82 1e 07 02              	callq	0x1428d80d0 <.text+0x28d70d0>
14086624e: 48 8d 0d 1b 49 3d 02        	leaq	0x23d491b(%rip), %rcx   # 0x142c3ab70
140866255: e8 f6 2f 07 02              	callq	0x1428d9250 <.text+0x28d8250>
14086625a: cc                          	int3
14086625b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140866260: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140866265: 55                          	pushq	%rbp
140866266: 41 57                       	pushq	%r15
140866268: 41 56                       	pushq	%r14
14086626a: 41 55                       	pushq	%r13
14086626c: 41 54                       	pushq	%r12
14086626e: 56                          	pushq	%rsi
14086626f: 57                          	pushq	%rdi
140866270: 53                          	pushq	%rbx
140866271: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140866278: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086627f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140866285: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14086628b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140866291: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140866297: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
14086629d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
1408662a6: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
1408662ae: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
1408662b6: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
1408662bd: 48 ff 00                    	incq	(%rax)
1408662c0: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
1408662c8: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
1408662d0: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
1408662d9: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1408662df: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
1408662e5: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
1408662eb: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1408662f1: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1408662f7: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1408662fe: 5b                          	popq	%rbx
1408662ff: 5f                          	popq	%rdi
140866300: 5e                          	popq	%rsi
140866301: 41 5c                       	popq	%r12
140866303: 41 5d                       	popq	%r13
140866305: 41 5e                       	popq	%r14
140866307: 41 5f                       	popq	%r15
140866309: 5d                          	popq	%rbp
14086630a: c3                          	retq
14086630b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140866310: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140866315: 55                          	pushq	%rbp
140866316: 41 57                       	pushq	%r15
140866318: 41 56                       	pushq	%r14
14086631a: 41 55                       	pushq	%r13
14086631c: 41 54                       	pushq	%r12
14086631e: 56                          	pushq	%rsi
14086631f: 57                          	pushq	%rdi
140866320: 53                          	pushq	%rbx
140866321: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140866328: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086632f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140866335: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14086633b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140866341: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140866347: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
14086634d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140866356: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14086635e: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140866366: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
14086636d: 48 ff 00                    	incq	(%rax)
140866370: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140866378: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140866380: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140866389: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14086638f: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140866395: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14086639b: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
1408663a1: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
1408663a7: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1408663ae: 5b                          	popq	%rbx
1408663af: 5f                          	popq	%rdi
1408663b0: 5e                          	popq	%rsi
1408663b1: 41 5c                       	popq	%r12
1408663b3: 41 5d                       	popq	%r13
1408663b5: 41 5e                       	popq	%r14
1408663b7: 41 5f                       	popq	%r15
1408663b9: 5d                          	popq	%rbp
1408663ba: c3                          	retq
1408663bb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1408663c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408663c5: 55                          	pushq	%rbp
1408663c6: 41 57                       	pushq	%r15
1408663c8: 41 56                       	pushq	%r14
1408663ca: 41 55                       	pushq	%r13
1408663cc: 41 54                       	pushq	%r12
1408663ce: 56                          	pushq	%rsi
1408663cf: 57                          	pushq	%rdi
1408663d0: 53                          	pushq	%rbx
1408663d1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1408663d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408663df: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
1408663e5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
1408663eb: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
1408663f1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
1408663f7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
1408663fd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140866406: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14086640e: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140866416: 48 83 7d e8 00              	cmpq	$0x0, -0x18(%rbp)
14086641b: 74 20                       	je	0x14086643d <.text+0x86543d>
14086641d: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
140866421: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140866425: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
140866429: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
14086642d: 48 8d 0d 34 fb 22 02        	leaq	0x222fb34(%rip), %rcx   # 0x142a95f68
140866434: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
140866438: e8 93 8e ac 00              	callq	0x14132f2d0 <.text+0x132e2d0>
14086643d: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140866445: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14086644d: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140866456: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14086645c: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140866462: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140866468: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14086646e: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140866474: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14086647b: 5b                          	popq	%rbx
14086647c: 5f                          	popq	%rdi
14086647d: 5e                          	popq	%rsi
14086647e: 41 5c                       	popq	%r12
140866480: 41 5d                       	popq	%r13
140866482: 41 5e                       	popq	%r14
140866484: 41 5f                       	popq	%r15
140866486: 5d                          	popq	%rbp
140866487: c3                          	retq
140866488: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140866490: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140866495: 55                          	pushq	%rbp
140866496: 41 57                       	pushq	%r15
140866498: 41 56                       	pushq	%r14
14086649a: 41 55                       	pushq	%r13
14086649c: 41 54                       	pushq	%r12
14086649e: 56                          	pushq	%rsi
14086649f: 57                          	pushq	%rdi
1408664a0: 53                          	pushq	%rbx
1408664a1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1408664a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408664af: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
1408664b5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
1408664bb: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
1408664c1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
1408664c7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
1408664cd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
1408664d6: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
1408664de: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
1408664e6: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1408664ed: 48 ff 00                    	incq	(%rax)
1408664f0: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
1408664f8: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140866500: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140866509: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14086650f: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140866515: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14086651b: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140866521: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140866527: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14086652e: 5b                          	popq	%rbx
14086652f: 5f                          	popq	%rdi
140866530: 5e                          	popq	%rsi
140866531: 41 5c                       	popq	%r12
140866533: 41 5d                       	popq	%r13
140866535: 41 5e                       	popq	%r14
140866537: 41 5f                       	popq	%r15
140866539: 5d                          	popq	%rbp
14086653a: c3                          	retq
