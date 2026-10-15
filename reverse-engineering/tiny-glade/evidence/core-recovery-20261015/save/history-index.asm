
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a84650: 56                          	pushq	%rsi
140a84651: 48 83 ec 30                 	subq	$0x30, %rsp
140a84655: 48 8b 41 28                 	movq	0x28(%rcx), %rax
140a84659: 48 85 c0                    	testq	%rax, %rax
140a8465c: 0f 84 d0 00 00 00           	je	0x140a84732 <.text+0xa83732>
140a84662: 8b 51 68                    	movl	0x68(%rcx), %edx
140a84665: 48 8b 49 20                 	movq	0x20(%rcx), %rcx
140a84669: 45 31 c0                    	xorl	%r8d, %r8d
140a8466c: 48 83 f8 01                 	cmpq	$0x1, %rax
140a84670: 74 30                       	je	0x140a846a2 <.text+0xa836a2>
140a84672: 49 89 c1                    	movq	%rax, %r9
140a84675: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a84680: 4d 89 ca                    	movq	%r9, %r10
140a84683: 49 d1 ea                    	shrq	%r10
140a84686: 4f 8d 1c 02                 	leaq	(%r10,%r8), %r11
140a8468a: 4c 89 de                    	movq	%r11, %rsi
140a8468d: 48 c1 e6 04                 	shlq	$0x4, %rsi
140a84691: 39 54 31 08                 	cmpl	%edx, 0x8(%rcx,%rsi)
140a84695: 4d 0f 4c c3                 	cmovlq	%r11, %r8
140a84699: 4d 29 d1                    	subq	%r10, %r9
140a8469c: 49 83 f9 01                 	cmpq	$0x1, %r9
140a846a0: 77 de                       	ja	0x140a84680 <.text+0xa83680>
140a846a2: 4d 89 c2                    	movq	%r8, %r10
140a846a5: 49 c1 e2 04                 	shlq	$0x4, %r10
140a846a9: 45 31 c9                    	xorl	%r9d, %r9d
140a846ac: 42 39 54 11 08              	cmpl	%edx, 0x8(%rcx,%r10)
140a846b1: 41 0f 9c c1                 	setl	%r9b
140a846b5: 4d 01 c1                    	addq	%r8, %r9
140a846b8: 49 39 c1                    	cmpq	%rax, %r9
140a846bb: 73 65                       	jae	0x140a84722 <.text+0xa83722>
140a846bd: 4c 89 c8                    	movq	%r9, %rax
140a846c0: 48 c1 e0 04                 	shlq	$0x4, %rax
140a846c4: 48 01 c1                    	addq	%rax, %rcx
140a846c7: 4d 85 c9                    	testq	%r9, %r9
140a846ca: 74 34                       	je	0x140a84700 <.text+0xa83700>
140a846cc: 8b 41 f8                    	movl	-0x8(%rcx), %eax
140a846cf: 44 8b 41 08                 	movl	0x8(%rcx), %r8d
140a846d3: 29 d0                       	subl	%edx, %eax
140a846d5: 41 89 c1                    	movl	%eax, %r9d
140a846d8: 41 f7 d9                    	negl	%r9d
140a846db: 44 0f 48 c8                 	cmovsl	%eax, %r9d
140a846df: 41 29 d0                    	subl	%edx, %r8d
140a846e2: 44 89 c0                    	movl	%r8d, %eax
140a846e5: f7 d8                       	negl	%eax
140a846e7: 41 0f 48 c0                 	cmovsl	%r8d, %eax
140a846eb: 45 31 c0                    	xorl	%r8d, %r8d
140a846ee: 44 39 c8                    	cmpl	%r9d, %eax
140a846f1: 41 0f 9e c0                 	setle	%r8b
140a846f5: 41 c1 e0 04                 	shll	$0x4, %r8d
140a846f9: 4c 01 c1                    	addq	%r8, %rcx
140a846fc: 48 83 c1 f0                 	addq	$-0x10, %rcx
140a84700: 48 85 c9                    	testq	%rcx, %rcx
140a84703: 74 2d                       	je	0x140a84732 <.text+0xa83732>
140a84705: 48 8b 01                    	movq	(%rcx), %rax
140a84708: 8b 71 08                    	movl	0x8(%rcx), %esi
140a8470b: f0                          	lock
140a8470c: 48 ff 00                    	incq	(%rax)
140a8470f: 7e 3f                       	jle	0x140a84750 <.text+0xa83750>
140a84711: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a84716: 89 74 24 28                 	movl	%esi, 0x28(%rsp)
140a8471a: 39 d6                       	cmpl	%edx, %esi
140a8471c: 75 18                       	jne	0x140a84736 <.text+0xa83736>
140a8471e: 89 d6                       	movl	%edx, %esi
140a84720: eb 26                       	jmp	0x140a84748 <.text+0xa83748>
140a84722: 48 c1 e0 04                 	shlq	$0x4, %rax
140a84726: 48 01 c1                    	addq	%rax, %rcx
140a84729: 48 83 c1 f0                 	addq	$-0x10, %rcx
140a8472d: 48 85 c9                    	testq	%rcx, %rcx
140a84730: 75 d3                       	jne	0x140a84705 <.text+0xa83705>
140a84732: 31 c0                       	xorl	%eax, %eax
140a84734: eb 12                       	jmp	0x140a84748 <.text+0xa83748>
140a84736: f0                          	lock
140a84737: 48 ff 08                    	decq	(%rax)
140a8473a: 75 0a                       	jne	0x140a84746 <.text+0xa83746>
140a8473c: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140a84741: e8 3a 16 86 ff              	callq	0x1402e5d80 <.text+0x2e4d80>
140a84746: 31 c0                       	xorl	%eax, %eax
140a84748: 89 f2                       	movl	%esi, %edx
140a8474a: 48 83 c4 30                 	addq	$0x30, %rsp
140a8474e: 5e                          	popq	%rsi
140a8474f: c3                          	retq
140a84750: 0f 0b                       	ud2
140a84752: cc                          	int3
140a84753: cc                          	int3
140a84754: cc                          	int3
140a84755: cc                          	int3
140a84756: cc                          	int3
140a84757: cc                          	int3
140a84758: cc                          	int3
140a84759: cc                          	int3
140a8475a: cc                          	int3
140a8475b: cc                          	int3
140a8475c: cc                          	int3
140a8475d: cc                          	int3
140a8475e: cc                          	int3
140a8475f: cc                          	int3
