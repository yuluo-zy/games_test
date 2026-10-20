
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14233a170: 55                          	pushq	%rbp
14233a171: 41 57                       	pushq	%r15
14233a173: 41 56                       	pushq	%r14
14233a175: 41 55                       	pushq	%r13
14233a177: 41 54                       	pushq	%r12
14233a179: 56                          	pushq	%rsi
14233a17a: 57                          	pushq	%rdi
14233a17b: 53                          	pushq	%rbx
14233a17c: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
14233a183: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14233a18b: 48 c7 45 50 fe ff ff ff     	movq	$-0x2, 0x50(%rbp)
14233a193: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
14233a197: 49 89 d6                    	movq	%rdx, %r14
14233a19a: 48 89 cb                    	movq	%rcx, %rbx
14233a19d: e8 5e c8 8d fe              	callq	0x140c16a00 <.text+0xc15a00>
14233a1a2: 84 c0                       	testb	%al, %al
14233a1a4: 88 45 4f                    	movb	%al, 0x4f(%rbp)
14233a1a7: 74 5f                       	je	0x14233a208 <.text+0x2339208>
14233a1a9: 4c 8d 3d 8d 38 d1 00        	leaq	0xd1388d(%rip), %r15    # 0x14304da3d
14233a1b0: bf 33 00 00 00              	movl	$0x33, %edi
14233a1b5: ba 33 00 00 00              	movl	$0x33, %edx
14233a1ba: 4c 89 f9                    	movq	%r15, %rcx
14233a1bd: e8 6e df ff ff              	callq	0x142338130 <.text+0x2337130>
14233a1c2: 48 83 f8 01                 	cmpq	$0x1, %rax
14233a1c6: 0f 85 b1 00 00 00           	jne	0x14233a27d <.text+0x233927d>
14233a1cc: 49 89 d1                    	movq	%rdx, %r9
14233a1cf: 48 85 d2                    	testq	%rdx, %rdx
14233a1d2: 74 3d                       	je	0x14233a211 <.text+0x2339211>
14233a1d4: 49 83 f9 33                 	cmpq	$0x33, %r9
14233a1d8: 73 35                       	jae	0x14233a20f <.text+0x233920f>
14233a1da: 48 8d 05 5c 38 d1 00        	leaq	0xd1385c(%rip), %rax    # 0x14304da3d
14233a1e1: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14233a1e6: 7f 29                       	jg	0x14233a211 <.text+0x2339211>
14233a1e8: 48 8d 05 d9 39 d1 00        	leaq	0xd139d9(%rip), %rax    # 0x14304dbc8
14233a1ef: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14233a1f4: 48 8d 0d 42 38 d1 00        	leaq	0xd13842(%rip), %rcx    # 0x14304da3d
14233a1fb: ba 33 00 00 00              	movl	$0x33, %edx
14233a200: 45 31 c0                    	xorl	%r8d, %r8d
14233a203: e8 38 f7 59 00              	callq	0x1428d9940 <.text+0x28d8940>
14233a208: 31 c0                       	xorl	%eax, %eax
14233a20a: e9 bd 01 00 00              	jmp	0x14233a3cc <.text+0x23393cc>
14233a20f: 75 d7                       	jne	0x14233a1e8 <.text+0x23391e8>
14233a211: 4c 8d 3d 25 38 d1 00        	leaq	0xd13825(%rip), %r15    # 0x14304da3d
14233a218: 4c 89 f9                    	movq	%r15, %rcx
14233a21b: 4c 89 ca                    	movq	%r9, %rdx
14233a21e: e8 0d df ff ff              	callq	0x142338130 <.text+0x2337130>
14233a223: 48 83 f8 01                 	cmpq	$0x1, %rax
14233a227: 75 54                       	jne	0x14233a27d <.text+0x233927d>
14233a229: 49 89 d0                    	movq	%rdx, %r8
14233a22c: 49 83 c0 02                 	addq	$0x2, %r8
14233a230: 74 39                       	je	0x14233a26b <.text+0x233926b>
14233a232: 49 83 f8 33                 	cmpq	$0x33, %r8
14233a236: 73 31                       	jae	0x14233a269 <.text+0x2339269>
14233a238: 48 8d 05 fe 37 d1 00        	leaq	0xd137fe(%rip), %rax    # 0x14304da3d
14233a23f: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14233a244: 7f 25                       	jg	0x14233a26b <.text+0x233926b>
14233a246: 48 8d 05 93 39 d1 00        	leaq	0xd13993(%rip), %rax    # 0x14304dbe0
14233a24d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14233a252: 48 8d 0d e4 37 d1 00        	leaq	0xd137e4(%rip), %rcx    # 0x14304da3d
14233a259: ba 33 00 00 00              	movl	$0x33, %edx
14233a25e: 41 b9 33 00 00 00           	movl	$0x33, %r9d
14233a264: e8 d7 f6 59 00              	callq	0x1428d9940 <.text+0x28d8940>
14233a269: 75 db                       	jne	0x14233a246 <.text+0x2339246>
14233a26b: bf 31 00 00 00              	movl	$0x31, %edi
14233a270: 48 29 d7                    	subq	%rdx, %rdi
14233a273: 4c 8d 3d c3 37 d1 00        	leaq	0xd137c3(%rip), %r15    # 0x14304da3d
14233a27a: 4d 01 c7                    	addq	%r8, %r15
14233a27d: 4c 8d 05 80 3a d1 00        	leaq	0xd13a80(%rip), %r8     # 0x14304dd04
14233a284: 4c 8d 25 4d 3a d1 00        	leaq	0xd13a4d(%rip), %r12    # 0x14304dcd8
14233a28b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14233a290: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14233a295: 85 c0                       	testl	%eax, %eax
14233a297: 78 17                       	js	0x14233a2b0 <.text+0x23392b0>
14233a299: 49 ff c8                    	decq	%r8
14233a29c: 83 f8 5c                    	cmpl	$0x5c, %eax
14233a29f: 75 63                       	jne	0x14233a304 <.text+0x2339304>
14233a2a1: eb 72                       	jmp	0x14233a315 <.text+0x2339315>
14233a2a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233a2b0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14233a2b5: 80 f9 c0                    	cmpb	$-0x40, %cl
14233a2b8: 7d 23                       	jge	0x14233a2dd <.text+0x23392dd>
14233a2ba: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14233a2bf: 80 fa c0                    	cmpb	$-0x40, %dl
14233a2c2: 7d 22                       	jge	0x14233a2e6 <.text+0x23392e6>
14233a2c4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14233a2c9: 49 83 c0 fc                 	addq	$-0x4, %r8
14233a2cd: 41 83 e1 07                 	andl	$0x7, %r9d
14233a2d1: 41 c1 e1 06                 	shll	$0x6, %r9d
14233a2d5: 83 e2 3f                    	andl	$0x3f, %edx
14233a2d8: 44 09 ca                    	orl	%r9d, %edx
14233a2db: eb 10                       	jmp	0x14233a2ed <.text+0x23392ed>
14233a2dd: 49 83 c0 fe                 	addq	$-0x2, %r8
14233a2e1: 83 e1 1f                    	andl	$0x1f, %ecx
14233a2e4: eb 0f                       	jmp	0x14233a2f5 <.text+0x23392f5>
14233a2e6: 49 83 c0 fd                 	addq	$-0x3, %r8
14233a2ea: 83 e2 0f                    	andl	$0xf, %edx
14233a2ed: c1 e2 06                    	shll	$0x6, %edx
14233a2f0: 83 e1 3f                    	andl	$0x3f, %ecx
14233a2f3: 09 d1                       	orl	%edx, %ecx
14233a2f5: c1 e1 06                    	shll	$0x6, %ecx
14233a2f8: 24 3f                       	andb	$0x3f, %al
14233a2fa: 0f b6 c0                    	movzbl	%al, %eax
14233a2fd: 09 c8                       	orl	%ecx, %eax
14233a2ff: 83 f8 5c                    	cmpl	$0x5c, %eax
14233a302: 74 11                       	je	0x14233a315 <.text+0x2339315>
14233a304: 83 f8 2f                    	cmpl	$0x2f, %eax
14233a307: 74 0c                       	je	0x14233a315 <.text+0x2339315>
14233a309: 4d 39 e0                    	cmpq	%r12, %r8
14233a30c: 75 82                       	jne	0x14233a290 <.text+0x2339290>
14233a30e: be 2c 00 00 00              	movl	$0x2c, %esi
14233a313: eb 4c                       	jmp	0x14233a361 <.text+0x2339361>
14233a315: 4c 8d 25 bc 39 d1 00        	leaq	0xd139bc(%rip), %r12    # 0x14304dcd8
14233a31c: 4d 29 e0                    	subq	%r12, %r8
14233a31f: 49 ff c0                    	incq	%r8
14233a322: 74 32                       	je	0x14233a356 <.text+0x2339356>
14233a324: 49 83 f8 2c                 	cmpq	$0x2c, %r8
14233a328: 73 2a                       	jae	0x14233a354 <.text+0x2339354>
14233a32a: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
14233a32f: 7f 25                       	jg	0x14233a356 <.text+0x2339356>
14233a331: 48 8d 05 70 38 d1 00        	leaq	0xd13870(%rip), %rax    # 0x14304dba8
14233a338: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14233a33d: 48 8d 0d 94 39 d1 00        	leaq	0xd13994(%rip), %rcx    # 0x14304dcd8
14233a344: ba 2c 00 00 00              	movl	$0x2c, %edx
14233a349: 41 b9 2c 00 00 00           	movl	$0x2c, %r9d
14233a34f: e8 ec f5 59 00              	callq	0x1428d9940 <.text+0x28d8940>
14233a354: 75 db                       	jne	0x14233a331 <.text+0x2339331>
14233a356: be 2c 00 00 00              	movl	$0x2c, %esi
14233a35b: 4c 29 c6                    	subq	%r8, %rsi
14233a35e: 4d 01 c4                    	addq	%r8, %r12
14233a361: e8 ca dd 8d fe              	callq	0x140c18130 <.text+0xc17130>
14233a366: 48 89 c1                    	movq	%rax, %rcx
14233a369: 48 8b 00                    	movq	(%rax), %rax
14233a36c: 48 83 f8 01                 	cmpq	$0x1, %rax
14233a370: 0f 85 3e 03 00 00           	jne	0x14233a6b4 <.text+0x23396b4>
14233a376: 48 83 c1 08                 	addq	$0x8, %rcx
14233a37a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14233a37e: 0f 85 74 03 00 00           	jne	0x14233a6f8 <.text+0x23396f8>
14233a384: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14233a38b: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
14233a38f: 48 83 c1 08                 	addq	$0x8, %rcx
14233a393: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
14233a398: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14233a3a1: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14233a3aa: 4c 89 fa                    	movq	%r15, %rdx
14233a3ad: 49 89 f8                    	movq	%rdi, %r8
14233a3b0: 4d 89 e1                    	movq	%r12, %r9
14233a3b3: e8 88 c9 8d fe              	callq	0x140c16d40 <.text+0xc15d40>
14233a3b8: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
14233a3bc: 48 ff 01                    	incq	(%rcx)
14233a3bf: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14233a3c3: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
14233a3c7: b8 01 00 00 00              	movl	$0x1, %eax
14233a3cc: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
14233a3d0: 48 8b 0b                    	movq	(%rbx), %rcx
14233a3d3: 4c 8b 7b 08                 	movq	0x8(%rbx), %r15
14233a3d7: 48 8b b9 f8 00 00 00        	movq	0xf8(%rcx), %rdi
14233a3de: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
14233a3e5: 4c 8d 24 87                 	leaq	(%rdi,%rax,4), %r12
14233a3e9: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
14233a3ed: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
14233a3f4: 88 45 30                    	movb	%al, 0x30(%rbp)
14233a3f7: 48 8b 45 38                 	movq	0x38(%rbp), %rax
14233a3fb: 48 83 c0 20                 	addq	$0x20, %rax
14233a3ff: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
14233a403: 41 8b 46 1c                 	movl	0x1c(%r14), %eax
14233a407: 89 45 24                    	movl	%eax, 0x24(%rbp)
14233a40a: 49 8b 06                    	movq	(%r14), %rax
14233a40d: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
14233a411: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
14233a415: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14233a419: 48 83 c0 20                 	addq	$0x20, %rax
14233a41d: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14233a421: b8 08 00 00 00              	movl	$0x8, %eax
14233a426: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14233a42a: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
14233a432: 48 89 45 00                 	movq	%rax, (%rbp)
14233a436: 45 31 f6                    	xorl	%r14d, %r14d
14233a439: 31 db                       	xorl	%ebx, %ebx
14233a43b: 80 7d 30 00                 	cmpb	$0x0, 0x30(%rbp)
14233a43f: 74 7f                       	je	0x14233a4c0 <.text+0x23394c0>
14233a441: 4c 39 f3                    	cmpq	%r14, %rbx
14233a444: 0f 85 11 01 00 00           	jne	0x14233a55b <.text+0x233955b>
14233a44a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14233a450: 4c 39 e7                    	cmpq	%r12, %rdi
14233a453: 0f 84 07 02 00 00           	je	0x14233a660 <.text+0x2339660>
14233a459: 8b 0f                       	movl	(%rdi), %ecx
14233a45b: 48 83 c7 04                 	addq	$0x4, %rdi
14233a45f: 49 8b 87 a8 01 00 00        	movq	0x1a8(%r15), %rax
14233a466: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14233a46a: 4c 8b 74 c8 10              	movq	0x10(%rax,%rcx,8), %r14
14233a46f: 4d 85 f6                    	testq	%r14, %r14
14233a472: 74 dc                       	je	0x14233a450 <.text+0x2339450>
14233a474: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
14233a478: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
14233a47c: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
14233a483: 48 39 48 40                 	cmpq	%rcx, 0x40(%rax)
14233a487: 0f 86 bc 00 00 00           	jbe	0x14233a549 <.text+0x2339549>
14233a48d: 48 8b 50 38                 	movq	0x38(%rax), %rdx
14233a491: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
14233a495: 48 85 c9                    	testq	%rcx, %rcx
14233a498: 0f 84 ab 00 00 00           	je	0x14233a549 <.text+0x2339549>
14233a49e: 48 8b 50 18                 	movq	0x18(%rax), %rdx
14233a4a2: 48 f7 d1                    	notq	%rcx
14233a4a5: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14233a4a9: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233a4ad: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
14233a4b2: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
14233a4b6: e9 96 00 00 00              	jmp	0x14233a551 <.text+0x2339551>
14233a4bb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14233a4c0: 4c 39 f3                    	cmpq	%r14, %rbx
14233a4c3: 0f 85 b0 00 00 00           	jne	0x14233a579 <.text+0x2339579>
14233a4c9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14233a4d0: 4c 39 e7                    	cmpq	%r12, %rdi
14233a4d3: 0f 84 87 01 00 00           	je	0x14233a660 <.text+0x2339660>
14233a4d9: 8b 0f                       	movl	(%rdi), %ecx
14233a4db: 48 83 c7 04                 	addq	$0x4, %rdi
14233a4df: 49 8b 87 00 01 00 00        	movq	0x100(%r15), %rax
14233a4e6: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233a4ea: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233a4ee: 4c 8b 74 08 58              	movq	0x58(%rax,%rcx), %r14
14233a4f3: 4d 85 f6                    	testq	%r14, %r14
14233a4f6: 74 d8                       	je	0x14233a4d0 <.text+0x23394d0>
14233a4f8: 48 01 c8                    	addq	%rcx, %rax
14233a4fb: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
14233a501: 49 8b 97 a8 01 00 00        	movq	0x1a8(%r15), %rdx
14233a508: 4c 8d 04 c9                 	leaq	(%rcx,%rcx,8), %r8
14233a50c: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
14233a510: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
14233a517: 4a 39 4c c2 40              	cmpq	%rcx, 0x40(%rdx,%r8,8)
14233a51c: 76 49                       	jbe	0x14233a567 <.text+0x2339567>
14233a51e: 4a 8d 14 c2                 	leaq	(%rdx,%r8,8), %rdx
14233a522: 4c 8b 42 38                 	movq	0x38(%rdx), %r8
14233a526: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
14233a52a: 48 85 c9                    	testq	%rcx, %rcx
14233a52d: 74 38                       	je	0x14233a567 <.text+0x2339567>
14233a52f: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14233a533: 48 f7 d1                    	notq	%rcx
14233a536: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14233a53a: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233a53e: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
14233a543: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
14233a547: eb 26                       	jmp	0x14233a56f <.text+0x233956f>
14233a549: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
14233a551: 48 8b 40 08                 	movq	0x8(%rax), %rax
14233a555: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14233a559: 31 db                       	xorl	%ebx, %ebx
14233a55b: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14233a55f: 48 8d 04 d8                 	leaq	(%rax,%rbx,8), %rax
14233a563: 89 d9                       	movl	%ebx, %ecx
14233a565: eb 25                       	jmp	0x14233a58c <.text+0x233958c>
14233a567: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
14233a56f: 48 8b 40 50                 	movq	0x50(%rax), %rax
14233a573: 48 89 45 00                 	movq	%rax, (%rbp)
14233a577: 31 db                       	xorl	%ebx, %ebx
14233a579: 48 89 d9                    	movq	%rbx, %rcx
14233a57c: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233a580: 48 8b 55 00                 	movq	(%rbp), %rdx
14233a584: 48 8d 04 0a                 	leaq	(%rdx,%rcx), %rax
14233a588: 8b 4c 0a 08                 	movl	0x8(%rdx,%rcx), %ecx
14233a58c: 48 8d 34 49                 	leaq	(%rcx,%rcx,2), %rsi
14233a590: 48 03 75 40                 	addq	0x40(%rbp), %rsi
14233a594: 48 8b 10                    	movq	(%rax), %rdx
14233a597: 48 89 d0                    	movq	%rdx, %rax
14233a59a: 48 c1 e8 20                 	shrq	$0x20, %rax
14233a59e: 0f 84 bc 00 00 00           	je	0x14233a660 <.text+0x2339660>
14233a5a4: 48 ff c3                    	incq	%rbx
14233a5a7: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
14233a5ab: 80 7e 01 00                 	cmpb	$0x0, 0x1(%rsi)
14233a5af: 74 1f                       	je	0x14233a5d0 <.text+0x23395d0>
14233a5b1: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14233a5b5: e8 36 fd f8 fd              	callq	0x1402ca2f0 <.text+0x2c92f0>
14233a5ba: 84 c0                       	testb	%al, %al
14233a5bc: 0f 85 79 fe ff ff           	jne	0x14233a43b <.text+0x233943b>
14233a5c2: eb 33                       	jmp	0x14233a5f7 <.text+0x23395f7>
14233a5c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14233a5d0: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
14233a5d4: 48 8d 55 28                 	leaq	0x28(%rbp), %rdx
14233a5d8: e8 93 67 27 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14233a5dd: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14233a5e1: 48 89 c2                    	movq	%rax, %rdx
14233a5e4: 4c 8d 45 28                 	leaq	0x28(%rbp), %r8
14233a5e8: e8 c3 22 c9 fe              	callq	0x140fcc8b0 <.text+0xfcb8b0>
14233a5ed: 48 c1 e8 20                 	shrq	$0x20, %rax
14233a5f1: 0f 85 44 fe ff ff           	jne	0x14233a43b <.text+0x233943b>
14233a5f7: 80 7e 01 01                 	cmpb	$0x1, 0x1(%rsi)
14233a5fb: 0f 85 3a fe ff ff           	jne	0x14233a43b <.text+0x233943b>
14233a601: 48 8b 45 28                 	movq	0x28(%rbp), %rax
14233a605: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14233a609: 8b 45 24                    	movl	0x24(%rbp), %eax
14233a60c: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
14233a610: 89 01                       	movl	%eax, (%rcx)
14233a612: 48 8b 45 08                 	movq	0x8(%rbp), %rax
14233a616: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14233a61a: 4c 8b 68 40                 	movq	0x40(%rax), %r13
14233a61e: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
14233a622: 75 10                       	jne	0x14233a634 <.text+0x2339634>
14233a624: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
14233a628: 48 8d 15 41 ac 87 00        	leaq	0x87ac41(%rip), %rdx    # 0x142bb5270
14233a62f: e8 fc 7f 22 fe              	callq	0x140562630 <.text+0x561630>
14233a634: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
14233a638: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14233a63c: 48 89 f1                    	movq	%rsi, %rcx
14233a63f: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233a643: 4c 89 2c 08                 	movq	%r13, (%rax,%rcx)
14233a647: 4c 8b 45 f0                 	movq	-0x10(%rbp), %r8
14233a64b: 4c 89 44 08 08              	movq	%r8, 0x8(%rax,%rcx)
14233a650: 48 ff c6                    	incq	%rsi
14233a653: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
14233a657: 48 ff 42 40                 	incq	0x40(%rdx)
14233a65b: e9 db fd ff ff              	jmp	0x14233a43b <.text+0x233943b>
14233a660: 80 7d 4f 00                 	cmpb	$0x0, 0x4f(%rbp)
14233a664: 74 3a                       	je	0x14233a6a0 <.text+0x23396a0>
14233a666: e8 c5 da 8d fe              	callq	0x140c18130 <.text+0xc17130>
14233a66b: 48 89 c1                    	movq	%rax, %rcx
14233a66e: 48 8b 00                    	movq	(%rax), %rax
14233a671: 48 83 f8 01                 	cmpq	$0x1, %rax
14233a675: 75 57                       	jne	0x14233a6ce <.text+0x23396ce>
14233a677: 48 83 c1 08                 	addq	$0x8, %rcx
14233a67b: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
14233a67f: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14233a683: 75 73                       	jne	0x14233a6f8 <.text+0x23396f8>
14233a685: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14233a68c: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
14233a690: 48 83 c1 08                 	addq	$0x8, %rcx
14233a694: e8 57 ca 8d fe              	callq	0x140c170f0 <.text+0xc160f0>
14233a699: 48 8b 45 38                 	movq	0x38(%rbp), %rax
14233a69d: 48 ff 00                    	incq	(%rax)
14233a6a0: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
14233a6a7: 5b                          	popq	%rbx
14233a6a8: 5f                          	popq	%rdi
14233a6a9: 5e                          	popq	%rsi
14233a6aa: 41 5c                       	popq	%r12
14233a6ac: 41 5d                       	popq	%r13
14233a6ae: 41 5e                       	popq	%r14
14233a6b0: 41 5f                       	popq	%r15
14233a6b2: 5d                          	popq	%rbp
14233a6b3: c3                          	retq
14233a6b4: 48 85 c0                    	testq	%rax, %rax
14233a6b7: 75 33                       	jne	0x14233a6ec <.text+0x23396ec>
14233a6b9: 31 d2                       	xorl	%edx, %edx
14233a6bb: e8 90 bb 5a 00              	callq	0x1428e6250 <.text+0x28e5250>
14233a6c0: 48 89 c1                    	movq	%rax, %rcx
14233a6c3: 48 85 c0                    	testq	%rax, %rax
14233a6c6: 0f 85 ae fc ff ff           	jne	0x14233a37a <.text+0x233937a>
14233a6cc: eb 1e                       	jmp	0x14233a6ec <.text+0x23396ec>
14233a6ce: 48 85 c0                    	testq	%rax, %rax
14233a6d1: 48 8b 45 18                 	movq	0x18(%rbp), %rax
14233a6d5: 75 15                       	jne	0x14233a6ec <.text+0x23396ec>
14233a6d7: 48 89 c6                    	movq	%rax, %rsi
14233a6da: 31 d2                       	xorl	%edx, %edx
14233a6dc: e8 6f bb 5a 00              	callq	0x1428e6250 <.text+0x28e5250>
14233a6e1: 48 89 f2                    	movq	%rsi, %rdx
14233a6e4: 48 89 c1                    	movq	%rax, %rcx
14233a6e7: 48 85 c0                    	testq	%rax, %rax
14233a6ea: 75 93                       	jne	0x14233a67f <.text+0x233967f>
14233a6ec: 48 8d 0d e5 ab 87 00        	leaq	0x87abe5(%rip), %rcx    # 0x142bb52d8
14233a6f3: e8 d8 d9 59 00              	callq	0x1428d80d0 <.text+0x28d70d0>
14233a6f8: 48 8d 0d 49 ac 87 00        	leaq	0x87ac49(%rip), %rcx    # 0x142bb5348
14233a6ff: e8 4c eb 59 00              	callq	0x1428d9250 <.text+0x28d8250>
14233a704: cc                          	int3
14233a705: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14233a710: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233a715: 55                          	pushq	%rbp
14233a716: 41 57                       	pushq	%r15
14233a718: 41 56                       	pushq	%r14
14233a71a: 41 55                       	pushq	%r13
14233a71c: 41 54                       	pushq	%r12
14233a71e: 56                          	pushq	%rsi
14233a71f: 57                          	pushq	%rdi
14233a720: 53                          	pushq	%rbx
14233a721: 48 83 ec 38                 	subq	$0x38, %rsp
14233a725: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233a72c: 48 8b 45 38                 	movq	0x38(%rbp), %rax
14233a730: 48 ff 00                    	incq	(%rax)
14233a733: 48 83 c4 38                 	addq	$0x38, %rsp
14233a737: 5b                          	popq	%rbx
14233a738: 5f                          	popq	%rdi
14233a739: 5e                          	popq	%rsi
14233a73a: 41 5c                       	popq	%r12
14233a73c: 41 5d                       	popq	%r13
14233a73e: 41 5e                       	popq	%r14
14233a740: 41 5f                       	popq	%r15
14233a742: 5d                          	popq	%rbp
14233a743: c3                          	retq
14233a744: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14233a750: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233a755: 55                          	pushq	%rbp
14233a756: 41 57                       	pushq	%r15
14233a758: 41 56                       	pushq	%r14
14233a75a: 41 55                       	pushq	%r13
14233a75c: 41 54                       	pushq	%r12
14233a75e: 56                          	pushq	%rsi
14233a75f: 57                          	pushq	%rdi
14233a760: 53                          	pushq	%rbx
14233a761: 48 83 ec 38                 	subq	$0x38, %rsp
14233a765: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233a76c: 48 8b 45 30                 	movq	0x30(%rbp), %rax
14233a770: 48 ff 00                    	incq	(%rax)
14233a773: 48 83 c4 38                 	addq	$0x38, %rsp
14233a777: 5b                          	popq	%rbx
14233a778: 5f                          	popq	%rdi
14233a779: 5e                          	popq	%rsi
14233a77a: 41 5c                       	popq	%r12
14233a77c: 41 5d                       	popq	%r13
14233a77e: 41 5e                       	popq	%r14
14233a780: 41 5f                       	popq	%r15
14233a782: 5d                          	popq	%rbp
14233a783: c3                          	retq
14233a784: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14233a790: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233a795: 55                          	pushq	%rbp
14233a796: 41 57                       	pushq	%r15
14233a798: 41 56                       	pushq	%r14
14233a79a: 41 55                       	pushq	%r13
14233a79c: 41 54                       	pushq	%r12
14233a79e: 56                          	pushq	%rsi
14233a79f: 57                          	pushq	%rdi
14233a7a0: 53                          	pushq	%rbx
14233a7a1: 48 83 ec 38                 	subq	$0x38, %rsp
14233a7a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233a7ac: 80 7d 4f 00                 	cmpb	$0x0, 0x4f(%rbp)
14233a7b0: 74 20                       	je	0x14233a7d2 <.text+0x23397d2>
14233a7b2: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
14233a7b6: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
14233a7ba: 48 8d 45 d0                 	leaq	-0x30(%rbp), %rax
14233a7be: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
14233a7c2: 48 8d 0d 7f 33 d1 00        	leaq	0xd1337f(%rip), %rcx    # 0x14304db48
14233a7c9: 48 8d 55 c8                 	leaq	-0x38(%rbp), %rdx
14233a7cd: e8 4e 54 c9 fe              	callq	0x140fcfc20 <.text+0xfcec20>
14233a7d2: 90                          	nop
14233a7d3: 48 83 c4 38                 	addq	$0x38, %rsp
14233a7d7: 5b                          	popq	%rbx
14233a7d8: 5f                          	popq	%rdi
14233a7d9: 5e                          	popq	%rsi
14233a7da: 41 5c                       	popq	%r12
14233a7dc: 41 5d                       	popq	%r13
14233a7de: 41 5e                       	popq	%r14
14233a7e0: 41 5f                       	popq	%r15
14233a7e2: 5d                          	popq	%rbp
14233a7e3: c3                          	retq
