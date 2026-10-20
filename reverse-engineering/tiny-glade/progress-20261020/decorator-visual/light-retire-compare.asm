
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141257fa0: 41 57                       	pushq	%r15
141257fa2: 41 56                       	pushq	%r14
141257fa4: 41 55                       	pushq	%r13
141257fa6: 41 54                       	pushq	%r12
141257fa8: 56                          	pushq	%rsi
141257fa9: 57                          	pushq	%rdi
141257faa: 55                          	pushq	%rbp
141257fab: 53                          	pushq	%rbx
141257fac: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
141257fb3: 4d 89 cd                    	movq	%r9, %r13
141257fb6: 4d 89 c7                    	movq	%r8, %r15
141257fb9: 49 89 d6                    	movq	%rdx, %r14
141257fbc: 49 89 c8                    	movq	%rcx, %r8
141257fbf: 48 8b 94 24 f0 00 00 00     	movq	0xf0(%rsp), %rdx
141257fc7: 4d 85 f6                    	testq	%r14, %r14
141257fca: 0f 94 c0                    	sete	%al
141257fcd: 4d 85 c9                    	testq	%r9, %r9
141257fd0: 0f 94 c1                    	sete	%cl
141257fd3: 08 c1                       	orb	%al, %cl
141257fd5: 0f 85 1d 02 00 00           	jne	0x1412581f8 <.text+0x12571f8>
141257fdb: 4c 8b 0a                    	movq	(%rdx), %r9
141257fde: 4c 8b 52 08                 	movq	0x8(%rdx), %r10
141257fe2: 48 8b 42 10                 	movq	0x10(%rdx), %rax
141257fe6: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
141257feb: 48 8b 42 18                 	movq	0x18(%rdx), %rax
141257fef: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141257ff4: 4c 8b 5a 20                 	movq	0x20(%rdx), %r11
141257ff8: 45 31 e4                    	xorl	%r12d, %r12d
141257ffb: ba ff 00 00 00              	movl	$0xff, %edx
141258000: 4c 89 c0                    	movq	%r8, %rax
141258003: 31 db                       	xorl	%ebx, %ebx
141258005: 4c 89 44 24 30              	movq	%r8, 0x30(%rsp)
14125800a: 4c 89 4c 24 28              	movq	%r9, 0x28(%rsp)
14125800f: 4c 89 54 24 50              	movq	%r10, 0x50(%rsp)
141258014: 4c 89 5c 24 48              	movq	%r11, 0x48(%rsp)
141258019: 4c 89 7c 24 60              	movq	%r15, 0x60(%rsp)
14125801e: 4c 89 6c 24 58              	movq	%r13, 0x58(%rsp)
141258023: eb 2e                       	jmp	0x141258053 <.text+0x1257053>
141258025: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141258030: 49 ff c4                    	incq	%r12
141258033: 4d 39 f4                    	cmpq	%r14, %r12
141258036: 0f 83 98 01 00 00           	jae	0x1412581d4 <.text+0x12571d4>
14125803c: 4b 8d 04 e4                 	leaq	(%r12,%r12,8), %rax
141258040: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141258044: 4c 01 c0                    	addq	%r8, %rax
141258047: 4c 01 e0                    	addq	%r12, %rax
14125804a: 4d 39 f4                    	cmpq	%r14, %r12
14125804d: 0f 83 61 03 00 00           	jae	0x1412583b4 <.text+0x12573b4>
141258053: 4c 39 eb                    	cmpq	%r13, %rbx
141258056: 0f 83 4c 03 00 00           	jae	0x1412583a8 <.text+0x12573a8>
14125805c: 48 8d 0c db                 	leaq	(%rbx,%rbx,8), %rcx
141258060: 48 8d 3c 49                 	leaq	(%rcx,%rcx,2), %rdi
141258064: 4c 01 ff                    	addq	%r15, %rdi
141258067: 48 01 df                    	addq	%rbx, %rdi
14125806a: 8b 08                       	movl	(%rax), %ecx
14125806c: 31 c0                       	xorl	%eax, %eax
14125806e: 3b 0f                       	cmpl	(%rdi), %ecx
141258070: 0f 95 c0                    	setne	%al
141258073: 0f 42 c2                    	cmovbl	%edx, %eax
141258076: 84 c0                       	testb	%al, %al
141258078: 0f 84 42 01 00 00           	je	0x1412581c0 <.text+0x12571c0>
14125807e: 0f b6 c0                    	movzbl	%al, %eax
141258081: 83 f8 01                    	cmpl	$0x1, %eax
141258084: 75 aa                       	jne	0x141258030 <.text+0x1257030>
141258086: 49 8b 29                    	movq	(%r9), %rbp
141258089: 83 7d 00 03                 	cmpl	$0x3, (%rbp)
14125808d: 0f 8f 96 00 00 00           	jg	0x141258129 <.text+0x1257129>
141258093: 49 8b 41 08                 	movq	0x8(%r9), %rax
141258097: 8b 4f 18                    	movl	0x18(%rdi), %ecx
14125809a: 48 8d 54 24 7c              	leaq	0x7c(%rsp), %rdx
14125809f: 89 4a 08                    	movl	%ecx, 0x8(%rdx)
1412580a2: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
1412580a6: 48 89 0a                    	movq	%rcx, (%rdx)
1412580a9: 8b 48 1c                    	movl	0x1c(%rax), %ecx
1412580ac: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1412580b0: 89 0a                       	movl	%ecx, (%rdx)
1412580b2: 4c 8b 38                    	movq	(%rax), %r15
1412580b5: 49 8b 77 30                 	movq	0x30(%r15), %rsi
1412580b9: 4d 8b 6f 40                 	movq	0x40(%r15), %r13
1412580bd: 49 3b 77 20                 	cmpq	0x20(%r15), %rsi
1412580c1: 75 24                       	jne	0x1412580e7 <.text+0x12570e7>
1412580c3: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1412580c7: 48 8d 15 b2 9e 9c 01        	leaq	0x19c9eb2(%rip), %rdx   # 0x142c21f80
1412580ce: e8 dd a9 30 ff              	callq	0x140562ab0 <.text+0x561ab0>
1412580d3: 4c 8b 5c 24 48              	movq	0x48(%rsp), %r11
1412580d8: 4c 8b 54 24 50              	movq	0x50(%rsp), %r10
1412580dd: 4c 8b 4c 24 28              	movq	0x28(%rsp), %r9
1412580e2: 4c 8b 44 24 30              	movq	0x30(%rsp), %r8
1412580e7: 49 8b 47 28                 	movq	0x28(%r15), %rax
1412580eb: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
1412580ef: 4c 89 2c c8                 	movq	%r13, (%rax,%rcx,8)
1412580f3: c6 44 c8 08 0e              	movb	$0xe, 0x8(%rax,%rcx,8)
1412580f8: 0f 10 44 24 6d              	movups	0x6d(%rsp), %xmm0
1412580fd: 0f 10 4c 24 78              	movups	0x78(%rsp), %xmm1
141258102: 0f 11 44 c8 09              	movups	%xmm0, 0x9(%rax,%rcx,8)
141258107: 0f 11 4c c8 14              	movups	%xmm1, 0x14(%rax,%rcx,8)
14125810c: c6 44 c8 24 00              	movb	$0x0, 0x24(%rax,%rcx,8)
141258111: 48 ff c6                    	incq	%rsi
141258114: 49 89 77 30                 	movq	%rsi, 0x30(%r15)
141258118: 49 ff 47 40                 	incq	0x40(%r15)
14125811c: ff 45 00                    	incl	(%rbp)
14125811f: 4c 8b 7c 24 60              	movq	0x60(%rsp), %r15
141258124: 4c 8b 6c 24 58              	movq	0x58(%rsp), %r13
141258129: 0f b7 7f 0c                 	movzwl	0xc(%rdi), %edi
14125812d: 49 8b 72 10                 	movq	0x10(%r10), %rsi
141258131: 49 3b 32                    	cmpq	(%r10), %rsi
141258134: 75 23                       	jne	0x141258159 <.text+0x1257159>
141258136: 4c 89 d1                    	movq	%r10, %rcx
141258139: 48 8d 15 48 93 9c 01        	leaq	0x19c9348(%rip), %rdx   # 0x142c21488
141258140: e8 ab 31 5d ff              	callq	0x14082b2f0 <.text+0x82a2f0>
141258145: 4c 8b 5c 24 48              	movq	0x48(%rsp), %r11
14125814a: 4c 8b 54 24 50              	movq	0x50(%rsp), %r10
14125814f: 4c 8b 4c 24 28              	movq	0x28(%rsp), %r9
141258154: 4c 8b 44 24 30              	movq	0x30(%rsp), %r8
141258159: 0f b7 cf                    	movzwl	%di, %ecx
14125815c: 49 8b 42 08                 	movq	0x8(%r10), %rax
141258160: 66 89 0c 70                 	movw	%cx, (%rax,%rsi,2)
141258164: 48 ff c6                    	incq	%rsi
141258167: 49 89 72 10                 	movq	%rsi, 0x10(%r10)
14125816b: 48 8b 54 24 40              	movq	0x40(%rsp), %rdx
141258170: 48 8b 02                    	movq	(%rdx), %rax
141258173: 48 39 c1                    	cmpq	%rax, %rcx
141258176: 48 0f 42 c1                 	cmovbq	%rcx, %rax
14125817a: 48 89 02                    	movq	%rax, (%rdx)
14125817d: 48 8b 54 24 38              	movq	0x38(%rsp), %rdx
141258182: 48 8b 02                    	movq	(%rdx), %rax
141258185: 48 39 c1                    	cmpq	%rax, %rcx
141258188: 48 0f 47 c1                 	cmovaq	%rcx, %rax
14125818c: 48 89 02                    	movq	%rax, (%rdx)
14125818f: 49 8b 53 10                 	movq	0x10(%r11), %rdx
141258193: 48 39 ca                    	cmpq	%rcx, %rdx
141258196: 0f 86 00 02 00 00           	jbe	0x14125839c <.text+0x125739c>
14125819c: 49 8b 43 08                 	movq	0x8(%r11), %rax
1412581a0: c1 e1 04                    	shll	$0x4, %ecx
1412581a3: 66 c7 44 08 0c 00 00        	movw	$0x0, 0xc(%rax,%rcx)
1412581aa: 48 ff c3                    	incq	%rbx
1412581ad: 4c 39 eb                    	cmpq	%r13, %rbx
1412581b0: ba ff 00 00 00              	movl	$0xff, %edx
1412581b5: 0f 82 81 fe ff ff           	jb	0x14125803c <.text+0x125703c>
1412581bb: eb 17                       	jmp	0x1412581d4 <.text+0x12571d4>
1412581bd: 0f 1f 00                    	nopl	(%rax)
1412581c0: 49 ff c4                    	incq	%r12
1412581c3: 48 ff c3                    	incq	%rbx
1412581c6: 4d 39 f4                    	cmpq	%r14, %r12
1412581c9: 73 09                       	jae	0x1412581d4 <.text+0x12571d4>
1412581cb: 4c 39 eb                    	cmpq	%r13, %rbx
1412581ce: 0f 82 68 fe ff ff           	jb	0x14125803c <.text+0x125703c>
1412581d4: 4d 39 f4                    	cmpq	%r14, %r12
1412581d7: 0f 87 9b 01 00 00           	ja	0x141258378 <.text+0x1257378>
1412581dd: 4c 39 eb                    	cmpq	%r13, %rbx
1412581e0: 0f 87 a4 01 00 00           	ja	0x14125838a <.text+0x125738a>
1412581e6: 48 8b 94 24 f0 00 00 00     	movq	0xf0(%rsp), %rdx
1412581ee: 4c 39 eb                    	cmpq	%r13, %rbx
1412581f1: 75 10                       	jne	0x141258203 <.text+0x1257203>
1412581f3: e9 6c 01 00 00              	jmp	0x141258364 <.text+0x1257364>
1412581f8: 31 db                       	xorl	%ebx, %ebx
1412581fa: 4c 39 eb                    	cmpq	%r13, %rbx
1412581fd: 0f 84 61 01 00 00           	je	0x141258364 <.text+0x1257364>
141258203: 4c 8b 22                    	movq	(%rdx), %r12
141258206: 4c 8b 72 08                 	movq	0x8(%rdx), %r14
14125820a: 48 8b 6a 10                 	movq	0x10(%rdx), %rbp
14125820e: 48 8b 42 18                 	movq	0x18(%rdx), %rax
141258212: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
141258217: 48 8b 4a 20                 	movq	0x20(%rdx), %rcx
14125821b: 49 83 c7 10                 	addq	$0x10, %r15
14125821f: 48 8d 04 db                 	leaq	(%rbx,%rbx,8), %rax
141258223: 48 8d 3c 40                 	leaq	(%rax,%rax,2), %rdi
141258227: 48 01 df                    	addq	%rbx, %rdi
14125822a: 4a 8d 04 ed 00 00 00 00     	leaq	(,%r13,8), %rax
141258232: 4c 01 e8                    	addq	%r13, %rax
141258235: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141258239: 4c 01 e8                    	addq	%r13, %rax
14125823c: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141258241: 49 89 cd                    	movq	%rcx, %r13
141258244: 48 89 6c 24 30              	movq	%rbp, 0x30(%rsp)
141258249: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
14125824e: 66 90                       	nop
141258250: 49 8b 1c 24                 	movq	(%r12), %rbx
141258254: 83 3b 04                    	cmpl	$0x4, (%rbx)
141258257: 0f 8d 90 00 00 00           	jge	0x1412582ed <.text+0x12572ed>
14125825d: 4c 89 e5                    	movq	%r12, %rbp
141258260: 49 8b 44 24 08              	movq	0x8(%r12), %rax
141258265: 41 8b 4c 3f 08              	movl	0x8(%r15,%rdi), %ecx
14125826a: 48 8d 54 24 7c              	leaq	0x7c(%rsp), %rdx
14125826f: 89 4a 08                    	movl	%ecx, 0x8(%rdx)
141258272: 4d 89 fd                    	movq	%r15, %r13
141258275: 49 8b 0c 3f                 	movq	(%r15,%rdi), %rcx
141258279: 48 89 0a                    	movq	%rcx, (%rdx)
14125827c: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14125827f: 48 8b 50 10                 	movq	0x10(%rax), %rdx
141258283: 89 0a                       	movl	%ecx, (%rdx)
141258285: 48 8b 30                    	movq	(%rax), %rsi
141258288: 4c 8b 7e 30                 	movq	0x30(%rsi), %r15
14125828c: 4c 8b 66 40                 	movq	0x40(%rsi), %r12
141258290: 4c 3b 7e 20                 	cmpq	0x20(%rsi), %r15
141258294: 75 10                       	jne	0x1412582a6 <.text+0x12572a6>
141258296: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14125829a: 48 8d 15 df 9c 9c 01        	leaq	0x19c9cdf(%rip), %rdx   # 0x142c21f80
1412582a1: e8 0a a8 30 ff              	callq	0x140562ab0 <.text+0x561ab0>
1412582a6: 48 8b 46 28                 	movq	0x28(%rsi), %rax
1412582aa: 4b 8d 0c bf                 	leaq	(%r15,%r15,4), %rcx
1412582ae: 4c 89 24 c8                 	movq	%r12, (%rax,%rcx,8)
1412582b2: c6 44 c8 08 0e              	movb	$0xe, 0x8(%rax,%rcx,8)
1412582b7: 0f 10 44 24 6d              	movups	0x6d(%rsp), %xmm0
1412582bc: 0f 10 4c 24 78              	movups	0x78(%rsp), %xmm1
1412582c1: 0f 11 44 c8 09              	movups	%xmm0, 0x9(%rax,%rcx,8)
1412582c6: 0f 11 4c c8 14              	movups	%xmm1, 0x14(%rax,%rcx,8)
1412582cb: c6 44 c8 24 00              	movb	$0x0, 0x24(%rax,%rcx,8)
1412582d0: 49 ff c7                    	incq	%r15
1412582d3: 4c 89 7e 30                 	movq	%r15, 0x30(%rsi)
1412582d7: 48 ff 46 40                 	incq	0x40(%rsi)
1412582db: ff 03                       	incl	(%rbx)
1412582dd: 4d 89 ef                    	movq	%r13, %r15
1412582e0: 49 89 ec                    	movq	%rbp, %r12
1412582e3: 48 8b 6c 24 30              	movq	0x30(%rsp), %rbp
1412582e8: 4c 8b 6c 24 28              	movq	0x28(%rsp), %r13
1412582ed: 41 0f b7 5c 3f fc           	movzwl	-0x4(%r15,%rdi), %ebx
1412582f3: 49 8b 76 10                 	movq	0x10(%r14), %rsi
1412582f7: 49 3b 36                    	cmpq	(%r14), %rsi
1412582fa: 75 0f                       	jne	0x14125830b <.text+0x125730b>
1412582fc: 4c 89 f1                    	movq	%r14, %rcx
1412582ff: 48 8d 15 82 91 9c 01        	leaq	0x19c9182(%rip), %rdx   # 0x142c21488
141258306: e8 e5 2f 5d ff              	callq	0x14082b2f0 <.text+0x82a2f0>
14125830b: 0f b7 cb                    	movzwl	%bx, %ecx
14125830e: 49 8b 46 08                 	movq	0x8(%r14), %rax
141258312: 66 89 0c 70                 	movw	%cx, (%rax,%rsi,2)
141258316: 48 ff c6                    	incq	%rsi
141258319: 49 89 76 10                 	movq	%rsi, 0x10(%r14)
14125831d: 48 8b 45 00                 	movq	(%rbp), %rax
141258321: 48 39 c1                    	cmpq	%rax, %rcx
141258324: 48 0f 42 c1                 	cmovbq	%rcx, %rax
141258328: 48 89 45 00                 	movq	%rax, (%rbp)
14125832c: 48 8b 54 24 40              	movq	0x40(%rsp), %rdx
141258331: 48 8b 02                    	movq	(%rdx), %rax
141258334: 48 39 c1                    	cmpq	%rax, %rcx
141258337: 48 0f 47 c1                 	cmovaq	%rcx, %rax
14125833b: 48 89 02                    	movq	%rax, (%rdx)
14125833e: 49 8b 55 10                 	movq	0x10(%r13), %rdx
141258342: 48 39 ca                    	cmpq	%rcx, %rdx
141258345: 76 55                       	jbe	0x14125839c <.text+0x125739c>
141258347: 49 8b 45 08                 	movq	0x8(%r13), %rax
14125834b: c1 e1 04                    	shll	$0x4, %ecx
14125834e: 66 c7 44 08 0c 00 00        	movw	$0x0, 0xc(%rax,%rcx)
141258355: 48 83 c7 1c                 	addq	$0x1c, %rdi
141258359: 48 39 7c 24 38              	cmpq	%rdi, 0x38(%rsp)
14125835e: 0f 85 ec fe ff ff           	jne	0x141258250 <.text+0x1257250>
141258364: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
14125836b: 5b                          	popq	%rbx
14125836c: 5d                          	popq	%rbp
14125836d: 5f                          	popq	%rdi
14125836e: 5e                          	popq	%rsi
14125836f: 41 5c                       	popq	%r12
141258371: 41 5d                       	popq	%r13
141258373: 41 5e                       	popq	%r14
141258375: 41 5f                       	popq	%r15
141258377: c3                          	retq
141258378: 4c 8d 05 b1 91 9c 01        	leaq	0x19c91b1(%rip), %r8    # 0x142c21530
14125837f: 4c 89 e1                    	movq	%r12, %rcx
141258382: 4c 89 f2                    	movq	%r14, %rdx
141258385: e8 a6 14 68 01              	callq	0x1428d9830 <.text+0x28d8830>
14125838a: 4c 8d 05 b7 91 9c 01        	leaq	0x19c91b7(%rip), %r8    # 0x142c21548
141258391: 48 89 d9                    	movq	%rbx, %rcx
141258394: 4c 89 ea                    	movq	%r13, %rdx
141258397: e8 94 14 68 01              	callq	0x1428d9830 <.text+0x28d8830>
14125839c: 4c 8d 05 fd 90 9c 01        	leaq	0x19c90fd(%rip), %r8    # 0x142c214a0
1412583a3: e8 70 11 68 01              	callq	0x1428d9518 <.text+0x28d8518>
1412583a8: 48 8d 0d 69 91 9c 01        	leaq	0x19c9169(%rip), %rcx   # 0x142c21518
1412583af: e8 5c 0f 68 01              	callq	0x1428d9310 <.text+0x28d8310>
1412583b4: 48 8d 0d 45 91 9c 01        	leaq	0x19c9145(%rip), %rcx   # 0x142c21500
1412583bb: e8 50 0f 68 01              	callq	0x1428d9310 <.text+0x28d8310>
1412583c0: cc                          	int3
1412583c1: cc                          	int3
1412583c2: cc                          	int3
1412583c3: cc                          	int3
1412583c4: cc                          	int3
1412583c5: cc                          	int3
1412583c6: cc                          	int3
1412583c7: cc                          	int3
1412583c8: cc                          	int3
1412583c9: cc                          	int3
1412583ca: cc                          	int3
1412583cb: cc                          	int3
1412583cc: cc                          	int3
1412583cd: cc                          	int3
1412583ce: cc                          	int3
1412583cf: cc                          	int3
