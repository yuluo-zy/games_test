
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f86340: 55                          	pushq	%rbp
140f86341: 41 57                       	pushq	%r15
140f86343: 41 56                       	pushq	%r14
140f86345: 41 55                       	pushq	%r13
140f86347: 41 54                       	pushq	%r12
140f86349: 56                          	pushq	%rsi
140f8634a: 57                          	pushq	%rdi
140f8634b: 53                          	pushq	%rbx
140f8634c: 48 83 ec 48                 	subq	$0x48, %rsp
140f86350: 48 8d 6c 24 40              	leaq	0x40(%rsp), %rbp
140f86355: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
140f8635d: 4c 89 c3                    	movq	%r8, %rbx
140f86360: 48 89 d7                    	movq	%rdx, %rdi
140f86363: 48 89 ce                    	movq	%rcx, %rsi
140f86366: 49 8b 00                    	movq	(%r8), %rax
140f86369: 4d 8b 78 10                 	movq	0x10(%r8), %r15
140f8636d: 4c 39 f8                    	cmpq	%r15, %rax
140f86370: 76 5c                       	jbe	0x140f863ce <.text+0xf853ce>
140f86372: 4c 8b 6b 08                 	movq	0x8(%rbx), %r13
140f86376: 48 c1 e0 03                 	shlq	$0x3, %rax
140f8637a: 4c 8d 34 40                 	leaq	(%rax,%rax,2), %r14
140f8637e: 4d 85 ff                    	testq	%r15, %r15
140f86381: 74 51                       	je	0x140f863d4 <.text+0xf853d4>
140f86383: 4a 8d 04 fd 00 00 00 00     	leaq	(,%r15,8), %rax
140f8638b: 4c 8d 0c 40                 	leaq	(%rax,%rax,2), %r9
140f8638f: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f86395: 4c 89 e9                    	movq	%r13, %rcx
140f86398: 4c 89 f2                    	movq	%r14, %rdx
140f8639b: 4c 89 4d f0                 	movq	%r9, -0x10(%rbp)
140f8639f: e8 8c d8 68 ff              	callq	0x140613c30 <.text+0x612c30>
140f863a4: 49 89 c4                    	movq	%rax, %r12
140f863a7: 48 85 c0                    	testq	%rax, %rax
140f863aa: 75 3f                       	jne	0x140f863eb <.text+0xf853eb>
140f863ac: 4c 89 75 e8                 	movq	%r14, -0x18(%rbp)
140f863b0: 4c 89 6d f8                 	movq	%r13, -0x8(%rbp)
140f863b4: 4c 8d 05 55 6f c2 01        	leaq	0x1c26f55(%rip), %r8    # 0x142bad310
140f863bb: b9 08 00 00 00              	movl	$0x8, %ecx
140f863c0: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
140f863c4: e8 1a 2c 95 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140f863c9: e9 93 00 00 00              	jmp	0x140f86461 <.text+0xf85461>
140f863ce: 4c 8b 63 08                 	movq	0x8(%rbx), %r12
140f863d2: eb 1e                       	jmp	0x140f863f2 <.text+0xf853f2>
140f863d4: 41 bc 08 00 00 00           	movl	$0x8, %r12d
140f863da: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f863e0: 4c 89 e9                    	movq	%r13, %rcx
140f863e3: 4c 89 f2                    	movq	%r14, %rdx
140f863e6: e8 35 d8 68 ff              	callq	0x140613c20 <.text+0x612c20>
140f863eb: 4c 89 63 08                 	movq	%r12, 0x8(%rbx)
140f863ef: 4c 89 3b                    	movq	%r15, (%rbx)
140f863f2: 48 8b 0f                    	movq	(%rdi), %rcx
140f863f5: eb 0b                       	jmp	0x140f86402 <.text+0xf85402>
140f863f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140f86400: f3 90                       	pause
140f86402: 48 8b 41 08                 	movq	0x8(%rcx), %rax
140f86406: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140f86410: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140f86414: 74 ea                       	je	0x140f86400 <.text+0xf85400>
140f86416: 48 85 c0                    	testq	%rax, %rax
140f86419: 78 2b                       	js	0x140f86446 <.text+0xf85446>
140f8641b: 48 8d 50 01                 	leaq	0x1(%rax), %rdx
140f8641f: f0                          	lock
140f86420: 48 0f b1 51 08              	cmpxchgq	%rdx, 0x8(%rcx)
140f86425: 75 e9                       	jne	0x140f86410 <.text+0xf85410>
140f86427: 4c 89 26                    	movq	%r12, (%rsi)
140f8642a: 4c 89 7e 08                 	movq	%r15, 0x8(%rsi)
140f8642e: 48 89 4e 10                 	movq	%rcx, 0x10(%rsi)
140f86432: 48 89 f0                    	movq	%rsi, %rax
140f86435: 48 83 c4 48                 	addq	$0x48, %rsp
140f86439: 5b                          	popq	%rbx
140f8643a: 5f                          	popq	%rdi
140f8643b: 5e                          	popq	%rsi
140f8643c: 41 5c                       	popq	%r12
140f8643e: 41 5d                       	popq	%r13
140f86440: 41 5e                       	popq	%r14
140f86442: 41 5f                       	popq	%r15
140f86444: 5d                          	popq	%rbp
140f86445: c3                          	retq
140f86446: 4c 89 65 f8                 	movq	%r12, -0x8(%rbp)
140f8644a: 4c 89 7d f0                 	movq	%r15, -0x10(%rbp)
140f8644e: 48 8d 0d eb ae b6 01        	leaq	0x1b6aeeb(%rip), %rcx   # 0x142af1340
140f86455: 48 8d 15 44 af b6 01        	leaq	0x1b6af44(%rip), %rdx   # 0x142af13a0
140f8645c: e8 9f 35 92 01              	callq	0x1428a9a00 <.text+0x28a8a00>
140f86461: 0f 0b                       	ud2
140f86463: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140f86470: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f86475: 55                          	pushq	%rbp
140f86476: 41 57                       	pushq	%r15
140f86478: 41 56                       	pushq	%r14
140f8647a: 41 55                       	pushq	%r13
140f8647c: 41 54                       	pushq	%r12
140f8647e: 56                          	pushq	%rsi
140f8647f: 57                          	pushq	%rdi
140f86480: 53                          	pushq	%rbx
140f86481: 48 83 ec 28                 	subq	$0x28, %rsp
140f86485: 48 8d 6a 40                 	leaq	0x40(%rdx), %rbp
140f86489: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f8648f: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140f86493: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140f86497: e8 84 d7 68 ff              	callq	0x140613c20 <.text+0x612c20>
140f8649c: 90                          	nop
140f8649d: 48 83 c4 28                 	addq	$0x28, %rsp
140f864a1: 5b                          	popq	%rbx
140f864a2: 5f                          	popq	%rdi
140f864a3: 5e                          	popq	%rsi
140f864a4: 41 5c                       	popq	%r12
140f864a6: 41 5d                       	popq	%r13
140f864a8: 41 5e                       	popq	%r14
140f864aa: 41 5f                       	popq	%r15
140f864ac: 5d                          	popq	%rbp
140f864ad: c3                          	retq
140f864ae: 66 90                       	nop
140f864b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f864b5: 55                          	pushq	%rbp
140f864b6: 41 57                       	pushq	%r15
140f864b8: 41 56                       	pushq	%r14
140f864ba: 41 55                       	pushq	%r13
140f864bc: 41 54                       	pushq	%r12
140f864be: 56                          	pushq	%rsi
140f864bf: 57                          	pushq	%rdi
140f864c0: 53                          	pushq	%rbx
140f864c1: 48 83 ec 28                 	subq	$0x28, %rsp
140f864c5: 48 8d 6a 40                 	leaq	0x40(%rdx), %rbp
140f864c9: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140f864cd: 48 85 c0                    	testq	%rax, %rax
140f864d0: 74 17                       	je	0x140f864e9 <.text+0xf854e9>
140f864d2: 48 c1 e0 03                 	shlq	$0x3, %rax
140f864d6: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140f864da: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f864e0: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140f864e4: e8 37 d7 68 ff              	callq	0x140613c20 <.text+0x612c20>
140f864e9: 90                          	nop
140f864ea: 48 83 c4 28                 	addq	$0x28, %rsp
140f864ee: 5b                          	popq	%rbx
140f864ef: 5f                          	popq	%rdi
140f864f0: 5e                          	popq	%rsi
140f864f1: 41 5c                       	popq	%r12
140f864f3: 41 5d                       	popq	%r13
140f864f5: 41 5e                       	popq	%r14
140f864f7: 41 5f                       	popq	%r15
140f864f9: 5d                          	popq	%rbp
140f864fa: c3                          	retq
