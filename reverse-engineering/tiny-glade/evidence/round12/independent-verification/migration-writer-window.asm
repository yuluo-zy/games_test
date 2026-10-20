
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14147263c: 45 8d 61 ff                 	leal	-0x1(%r9), %r12d
141472640: 45 21 cc                    	andl	%r9d, %r12d
141472643: 48 ff c9                    	decq	%rcx
141472646: 45 89 cf                    	movl	%r9d, %r15d
141472649: 48 89 f3                    	movq	%rsi, %rbx
14147264c: 45 31 c0                    	xorl	%r8d, %r8d
14147264f: 45 89 e1                    	movl	%r12d, %r9d
141472652: f3 45 0f bc ff              	tzcntl	%r15d, %r15d
141472657: 49 f7 df                    	negq	%r15
14147265a: 4d 6b ff 58                 	imulq	$0x58, %r15, %r15
14147265e: 46 0f b6 64 3b fa           	movzbl	-0x6(%rbx,%r15), %r12d
141472664: 41 80 c4 c8                 	addb	$-0x38, %r12b
141472668: 41 80 fc 02                 	cmpb	$0x2, %r12b
14147266c: 4c 8b a5 10 01 00 00        	movq	0x110(%rbp), %r12
141472673: 0f 86 bb 00 00 00           	jbe	0x141472734 <.text+0x1471734>
141472679: 4d 85 c0                    	testq	%r8, %r8
14147267c: 74 5e                       	je	0x1414726dc <.text+0x14716dc>
14147267e: 4d 85 d2                    	testq	%r10, %r10
141472681: 74 56                       	je	0x1414726d9 <.text+0x14716d9>
141472683: 66 85 d2                    	testw	%dx, %dx
141472686: 74 18                       	je	0x1414726a0 <.text+0x14716a0>
141472688: 44 8d 62 ff                 	leal	-0x1(%rdx), %r12d
14147268c: 41 21 d4                    	andl	%edx, %r12d
14147268f: 49 ff ca                    	decq	%r10
141472692: 41 89 d7                    	movl	%edx, %r15d
141472695: 4c 89 c3                    	movq	%r8, %rbx
141472698: 44 89 e2                    	movl	%r12d, %edx
14147269b: eb b5                       	jmp	0x141472652 <.text+0x1471652>
14147269d: 0f 1f 00                    	nopl	(%rax)
1414726a0: 66 41 0f 6f 03              	movdqa	(%r11), %xmm0
1414726a5: 66 44 0f d7 f8              	pmovmskb	%xmm0, %r15d
1414726aa: 49 81 c0 80 fa ff ff        	addq	$-0x580, %r8            # imm = 0xFA80
1414726b1: 49 83 c3 10                 	addq	$0x10, %r11
1414726b5: 41 81 ff ff ff 00 00        	cmpl	$0xffff, %r15d          # imm = 0xFFFF
1414726bc: 74 e2                       	je	0x1414726a0 <.text+0x14716a0>
1414726be: bb fe ff ff ff              	movl	$0xfffffffe, %ebx       # imm = 0xFFFFFFFE
1414726c3: 44 29 fb                    	subl	%r15d, %ebx
1414726c6: 41 f7 d7                    	notl	%r15d
1414726c9: 44 89 fa                    	movl	%r15d, %edx
1414726cc: 21 da                       	andl	%ebx, %edx
1414726ce: 49 ff ca                    	decq	%r10
1414726d1: 4c 89 c3                    	movq	%r8, %rbx
1414726d4: e9 79 ff ff ff              	jmp	0x141472652 <.text+0x1471652>
1414726d9: 45 31 d2                    	xorl	%r10d, %r10d
1414726dc: 48 85 f6                    	testq	%rsi, %rsi
1414726df: 0f 84 7a fe ff ff           	je	0x14147255f <.text+0x147155f>
1414726e5: 48 85 c9                    	testq	%rcx, %rcx
1414726e8: 0f 84 71 fe ff ff           	je	0x14147255f <.text+0x147155f>
1414726ee: 66 45 85 c9                 	testw	%r9w, %r9w
1414726f2: 0f 85 44 ff ff ff           	jne	0x14147263c <.text+0x147163c>
1414726f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141472700: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
141472704: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
141472709: 48 81 c6 80 fa ff ff        	addq	$-0x580, %rsi           # imm = 0xFA80
141472710: 48 83 c0 10                 	addq	$0x10, %rax
141472714: 41 81 f9 ff ff 00 00        	cmpl	$0xffff, %r9d           # imm = 0xFFFF
14147271b: 74 e3                       	je	0x141472700 <.text+0x1471700>
14147271d: 41 b8 fe ff ff ff           	movl	$0xfffffffe, %r8d       # imm = 0xFFFFFFFE
141472723: 45 29 c8                    	subl	%r9d, %r8d
141472726: 41 f7 d1                    	notl	%r9d
141472729: 45 89 cc                    	movl	%r9d, %r12d
14147272c: 45 21 c4                    	andl	%r8d, %r12d
14147272f: e9 0f ff ff ff              	jmp	0x141472643 <.text+0x1471643>
141472734: 4c 01 fb                    	addq	%r15, %rbx
141472737: 44 8b 63 d8                 	movl	-0x28(%rbx), %r12d
14147273b: 49 83 fc 08                 	cmpq	$0x8, %r12
14147273f: 77 17                       	ja	0x141472758 <.text+0x1471758>
141472741: 4c 8d 2d 70 26 7f 01        	leaq	0x17f2670(%rip), %r13   # 0x142c64db8
141472748: 4f 63 7c a5 00              	movslq	(%r13,%r12,4), %r15
14147274d: 4d 01 ef                    	addq	%r13, %r15
141472750: 41 ff e7                    	jmpq	*%r15
141472753: 45 89 e7                    	movl	%r12d, %r15d
141472756: eb 3a                       	jmp	0x141472792 <.text+0x1471792>
141472758: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14147275e: 41 81 fc e7 03 00 00        	cmpl	$0x3e7, %r12d           # imm = 0x3E7
141472765: 75 32                       	jne	0x141472799 <.text+0x1471799>
141472767: 45 89 e7                    	movl	%r12d, %r15d
14147276a: eb 2d                       	jmp	0x141472799 <.text+0x1471799>
14147276c: 41 bf 02 00 00 00           	movl	$0x2, %r15d
141472772: eb 1e                       	jmp	0x141472792 <.text+0x1471792>
141472774: 41 bf 03 00 00 00           	movl	$0x3, %r15d
14147277a: eb 16                       	jmp	0x141472792 <.text+0x1471792>
14147277c: 41 bf 05 00 00 00           	movl	$0x5, %r15d
141472782: eb 0e                       	jmp	0x141472792 <.text+0x1471792>
141472784: 41 bf 08 00 00 00           	movl	$0x8, %r15d
14147278a: eb 06                       	jmp	0x141472792 <.text+0x1471792>
14147278c: 41 bf 04 00 00 00           	movl	$0x4, %r15d
141472792: 44 8b ad 30 03 00 00        	movl	0x330(%rbp), %r13d
141472799: 44 89 7b d8                 	movl	%r15d, -0x28(%rbx)
14147279d: b3 01                       	movb	$0x1, %bl
14147279f: 89 9d f0 02 00 00           	movl	%ebx, 0x2f0(%rbp)
