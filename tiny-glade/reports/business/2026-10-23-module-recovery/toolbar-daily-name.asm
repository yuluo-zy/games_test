
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14083c240: 49 83 f8 05                 	cmpq	$0x5, %r8
14083c244: 0f 85 5b 01 00 00           	jne	0x14083c3a5 <.text+0x83b3a5>
14083c24a: b8 65 6e 2d 55              	movl	$0x552d6e65, %eax       # imm = 0x552D6E65
14083c24f: 33 02                       	xorl	(%rdx), %eax
14083c251: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c256: 41 83 f0 53                 	xorl	$0x53, %r8d
14083c25a: 41 09 c0                    	orl	%eax, %r8d
14083c25d: 0f 84 42 01 00 00           	je	0x14083c3a5 <.text+0x83b3a5>
14083c263: b8 64 65 2d 44              	movl	$0x442d6564, %eax       # imm = 0x442D6564
14083c268: 33 02                       	xorl	(%rdx), %eax
14083c26a: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c26f: 41 83 f0 45                 	xorl	$0x45, %r8d
14083c273: 41 09 c0                    	orl	%eax, %r8d
14083c276: 0f 84 5f 01 00 00           	je	0x14083c3db <.text+0x83b3db>
14083c27c: b8 65 73 2d 45              	movl	$0x452d7365, %eax       # imm = 0x452D7365
14083c281: 33 02                       	xorl	(%rdx), %eax
14083c283: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c288: 41 83 f0 53                 	xorl	$0x53, %r8d
14083c28c: 41 09 c0                    	orl	%eax, %r8d
14083c28f: 0f 84 4d 01 00 00           	je	0x14083c3e2 <.text+0x83b3e2>
14083c295: b8 66 72 2d 46              	movl	$0x462d7266, %eax       # imm = 0x462D7266
14083c29a: 33 02                       	xorl	(%rdx), %eax
14083c29c: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c2a1: 41 83 f0 52                 	xorl	$0x52, %r8d
14083c2a5: 41 09 c0                    	orl	%eax, %r8d
14083c2a8: 0f 84 3b 01 00 00           	je	0x14083c3e9 <.text+0x83b3e9>
14083c2ae: b8 69 74 2d 49              	movl	$0x492d7469, %eax       # imm = 0x492D7469
14083c2b3: 33 02                       	xorl	(%rdx), %eax
14083c2b5: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c2ba: 41 83 f0 54                 	xorl	$0x54, %r8d
14083c2be: 41 09 c0                    	orl	%eax, %r8d
14083c2c1: 0f 84 29 01 00 00           	je	0x14083c3f0 <.text+0x83b3f0>
14083c2c7: b8 6a 61 2d 4a              	movl	$0x4a2d616a, %eax       # imm = 0x4A2D616A
14083c2cc: 33 02                       	xorl	(%rdx), %eax
14083c2ce: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c2d3: 41 83 f0 50                 	xorl	$0x50, %r8d
14083c2d7: 41 09 c0                    	orl	%eax, %r8d
14083c2da: 0f 84 17 01 00 00           	je	0x14083c3f7 <.text+0x83b3f7>
14083c2e0: b8 6b 6f 2d 4b              	movl	$0x4b2d6f6b, %eax       # imm = 0x4B2D6F6B
14083c2e5: 33 02                       	xorl	(%rdx), %eax
14083c2e7: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c2ec: 41 83 f0 52                 	xorl	$0x52, %r8d
14083c2f0: 41 09 c0                    	orl	%eax, %r8d
14083c2f3: 0f 84 05 01 00 00           	je	0x14083c3fe <.text+0x83b3fe>
14083c2f9: b8 70 6c 2d 50              	movl	$0x502d6c70, %eax       # imm = 0x502D6C70
14083c2fe: 33 02                       	xorl	(%rdx), %eax
14083c300: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c305: 41 83 f0 4c                 	xorl	$0x4c, %r8d
14083c309: 41 09 c0                    	orl	%eax, %r8d
14083c30c: 0f 84 f3 00 00 00           	je	0x14083c405 <.text+0x83b405>
14083c312: b8 70 74 2d 42              	movl	$0x422d7470, %eax       # imm = 0x422D7470
14083c317: 33 02                       	xorl	(%rdx), %eax
14083c319: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c31e: 41 83 f0 52                 	xorl	$0x52, %r8d
14083c322: 41 09 c0                    	orl	%eax, %r8d
14083c325: 0f 84 e1 00 00 00           	je	0x14083c40c <.text+0x83b40c>
14083c32b: b8 72 75 2d 52              	movl	$0x522d7572, %eax       # imm = 0x522D7572
14083c330: 33 02                       	xorl	(%rdx), %eax
14083c332: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c337: 41 83 f0 55                 	xorl	$0x55, %r8d
14083c33b: 41 09 c0                    	orl	%eax, %r8d
14083c33e: 0f 84 cf 00 00 00           	je	0x14083c413 <.text+0x83b413>
14083c344: b8 73 76 2d 53              	movl	$0x532d7673, %eax       # imm = 0x532D7673
14083c349: 33 02                       	xorl	(%rdx), %eax
14083c34b: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c350: 41 83 f0 45                 	xorl	$0x45, %r8d
14083c354: 41 09 c0                    	orl	%eax, %r8d
14083c357: 0f 84 bd 00 00 00           	je	0x14083c41a <.text+0x83b41a>
14083c35d: b8 74 72 2d 54              	movl	$0x542d7274, %eax       # imm = 0x542D7274
14083c362: 33 02                       	xorl	(%rdx), %eax
14083c364: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c369: 41 83 f0 52                 	xorl	$0x52, %r8d
14083c36d: 41 09 c0                    	orl	%eax, %r8d
14083c370: 0f 84 ab 00 00 00           	je	0x14083c421 <.text+0x83b421>
14083c376: b8 75 6b 2d 55              	movl	$0x552d6b75, %eax       # imm = 0x552D6B75
14083c37b: 33 02                       	xorl	(%rdx), %eax
14083c37d: 44 0f b6 42 04              	movzbl	0x4(%rdx), %r8d
14083c382: 41 83 f0 41                 	xorl	$0x41, %r8d
14083c386: 41 09 c0                    	orl	%eax, %r8d
14083c389: 0f 84 9c 00 00 00           	je	0x14083c42b <.text+0x83b42b>
14083c38f: b8 7a 68 2d 43              	movl	$0x432d687a, %eax       # imm = 0x432D687A
14083c394: 33 02                       	xorl	(%rdx), %eax
14083c396: 0f b6 52 04                 	movzbl	0x4(%rdx), %edx
14083c39a: 83 f2 4e                    	xorl	$0x4e, %edx
14083c39d: 09 c2                       	orl	%eax, %edx
14083c39f: 0f 84 90 00 00 00           	je	0x14083c435 <.text+0x83b435>
14083c3a5: 31 c0                       	xorl	%eax, %eax
14083c3a7: 48 3b 41 10                 	cmpq	0x10(%rcx), %rax
14083c3ab: 73 14                       	jae	0x14083c3c1 <.text+0x83b3c1>
14083c3ad: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
14083c3b1: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14083c3b5: 48 8b 4c c2 08              	movq	0x8(%rdx,%rax,8), %rcx
14083c3ba: 4c 8b 44 c2 10              	movq	0x10(%rdx,%rax,8), %r8
14083c3bf: eb 02                       	jmp	0x14083c3c3 <.text+0x83b3c3>
14083c3c1: 31 c9                       	xorl	%ecx, %ecx
14083c3c3: 48 85 c9                    	testq	%rcx, %rcx
14083c3c6: 48 8d 05 1c 71 25 02        	leaq	0x225711c(%rip), %rax   # 0x142a934e9
14083c3cd: 48 0f 45 c1                 	cmovneq	%rcx, %rax
14083c3d1: ba 22 00 00 00              	movl	$0x22, %edx
14083c3d6: 49 0f 45 d0                 	cmovneq	%r8, %rdx
14083c3da: c3                          	retq
14083c3db: b8 01 00 00 00              	movl	$0x1, %eax
14083c3e0: eb c5                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c3e2: b8 02 00 00 00              	movl	$0x2, %eax
14083c3e7: eb be                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c3e9: b8 03 00 00 00              	movl	$0x3, %eax
14083c3ee: eb b7                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c3f0: b8 04 00 00 00              	movl	$0x4, %eax
14083c3f5: eb b0                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c3f7: b8 05 00 00 00              	movl	$0x5, %eax
14083c3fc: eb a9                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c3fe: b8 06 00 00 00              	movl	$0x6, %eax
14083c403: eb a2                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c405: b8 07 00 00 00              	movl	$0x7, %eax
14083c40a: eb 9b                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c40c: b8 08 00 00 00              	movl	$0x8, %eax
14083c411: eb 94                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c413: b8 09 00 00 00              	movl	$0x9, %eax
14083c418: eb 8d                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c41a: b8 0a 00 00 00              	movl	$0xa, %eax
14083c41f: eb 86                       	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c421: b8 0b 00 00 00              	movl	$0xb, %eax
14083c426: e9 7c ff ff ff              	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c42b: b8 0c 00 00 00              	movl	$0xc, %eax
14083c430: e9 72 ff ff ff              	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c435: b8 0d 00 00 00              	movl	$0xd, %eax
14083c43a: e9 68 ff ff ff              	jmp	0x14083c3a7 <.text+0x83b3a7>
14083c43f: cc                          	int3
14083c440: 48 83 ec 38                 	subq	$0x38, %rsp
14083c444: 48 8d 91 e8 00 00 00        	leaq	0xe8(%rcx), %rdx
14083c44b: 44 8b 81 e0 00 00 00        	movl	0xe0(%rcx), %r8d
14083c452: 44 8b 89 e4 00 00 00        	movl	0xe4(%rcx), %r9d
14083c459: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
14083c45e: e8 6d 77 52 00              	callq	0x140d63bd0 <.text+0xd62bd0>
14083c463: 0f b6 44 24 20              	movzbl	0x20(%rsp), %eax
14083c468: 48 81 7c 24 28 80 97 06 00  	cmpq	$0x69780, 0x28(%rsp)    # imm = 0x69780
14083c471: 0f 93 c1                    	setae	%cl
14083c474: f6 d0                       	notb	%al
14083c476: 20 c8                       	andb	%cl, %al
14083c478: 48 83 c4 38                 	addq	$0x38, %rsp
14083c47c: c3                          	retq
14083c47d: cc                          	int3
14083c47e: cc                          	int3
14083c47f: cc                          	int3
14083c480: 48 89 c8                    	movq	%rcx, %rax
14083c483: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
14083c48d: 48 89 08                    	movq	%rcx, (%rax)
14083c490: 48 89 48 08                 	movq	%rcx, 0x8(%rax)
14083c494: c3                          	retq
14083c495: cc                          	int3
14083c496: cc                          	int3
14083c497: cc                          	int3
14083c498: cc                          	int3
14083c499: cc                          	int3
14083c49a: cc                          	int3
14083c49b: cc                          	int3
14083c49c: cc                          	int3
14083c49d: cc                          	int3
14083c49e: cc                          	int3
14083c49f: cc                          	int3
14083c4a0: 55                          	pushq	%rbp
14083c4a1: 41 57                       	pushq	%r15
14083c4a3: 41 56                       	pushq	%r14
14083c4a5: 56                          	pushq	%rsi
14083c4a6: 57                          	pushq	%rdi
14083c4a7: 53                          	pushq	%rbx
14083c4a8: 48 81 ec b8 03 00 00        	subq	$0x3b8, %rsp            # imm = 0x3B8
14083c4af: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
