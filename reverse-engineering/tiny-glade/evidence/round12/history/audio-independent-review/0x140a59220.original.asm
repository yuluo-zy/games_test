
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a59220: 55                          	pushq	%rbp
140a59221: 41 56                       	pushq	%r14
140a59223: 56                          	pushq	%rsi
140a59224: 57                          	pushq	%rdi
140a59225: 53                          	pushq	%rbx
140a59226: 48 81 ec a0 00 00 00        	subq	$0xa0, %rsp
140a5922d: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a59235: 48 c7 45 18 fe ff ff ff     	movq	$-0x2, 0x18(%rbp)
140a5923d: 4c 89 c7                    	movq	%r8, %rdi
140a59240: 48 89 ce                    	movq	%rcx, %rsi
140a59243: 49 8b 40 10                 	movq	0x10(%r8), %rax
140a59247: 48 ff c0                    	incq	%rax
140a5924a: 41 0f 10 00                 	movups	(%r8), %xmm0
140a5924e: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
140a59256: 48 89 55 a0                 	movq	%rdx, -0x60(%rbp)
140a5925a: 0f 11 45 a8                 	movups	%xmm0, -0x58(%rbp)
140a5925e: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
140a59266: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140a5926a: 48 8d 45 08                 	leaq	0x8(%rbp), %rax
140a5926e: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140a59272: c6 45 17 00                 	movb	$0x0, 0x17(%rbp)
140a59276: 4c 8d 05 33 e9 21 02        	leaq	0x221e933(%rip), %r8    # 0x142c77bb0
140a5927d: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140a59281: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140a59285: e8 26 1d 8a 00              	callq	0x1412fafb0 <.text+0x12f9fb0>
140a5928a: 48 8b 5d 08                 	movq	0x8(%rbp), %rbx
140a5928e: 48 85 db                    	testq	%rbx, %rbx
140a59291: 74 0f                       	je	0x140a592a2 <.text+0xa582a2>
140a59293: c6 45 17 01                 	movb	$0x1, 0x17(%rbp)
140a59297: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140a5929b: e8 a0 04 e4 ff              	callq	0x140899740 <.text+0x898740>
140a592a0: eb 10                       	jmp	0x140a592b2 <.text+0xa582b2>
140a592a2: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
140a592a6: 48 8b 5d e0                 	movq	-0x20(%rbp), %rbx
140a592aa: 48 89 c1                    	movq	%rax, %rcx
140a592ad: 48 f7 d9                    	negq	%rcx
140a592b0: 71 22                       	jno	0x140a592d4 <.text+0xa582d4>
140a592b2: 48 89 5e 08                 	movq	%rbx, 0x8(%rsi)
140a592b6: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a592c0: 48 89 06                    	movq	%rax, (%rsi)
140a592c3: 48 89 f0                    	movq	%rsi, %rax
140a592c6: 48 81 c4 a0 00 00 00        	addq	$0xa0, %rsp
140a592cd: 5b                          	popq	%rbx
140a592ce: 5f                          	popq	%rdi
140a592cf: 5e                          	popq	%rsi
140a592d0: 41 5e                       	popq	%r14
140a592d2: 5d                          	popq	%rbp
140a592d3: c3                          	retq
140a592d4: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140a592d8: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140a592dc: 48 89 5d f8                 	movq	%rbx, -0x8(%rbp)
140a592e0: 48 89 4d 00                 	movq	%rcx, (%rbp)
140a592e4: 48 85 c9                    	testq	%rcx, %rcx
140a592e7: 74 7c                       	je	0x140a59365 <.text+0xa58365>
140a592e9: 83 3b 01                    	cmpl	$0x1, (%rbx)
140a592ec: 75 41                       	jne	0x140a5932f <.text+0xa5832f>
140a592ee: 48 83 c3 08                 	addq	$0x8, %rbx
140a592f2: 4c 8d 05 27 9c 07 02        	leaq	0x2079c27(%rip), %r8    # 0x142ad2f20
140a592f9: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140a592fd: 41 b9 04 00 00 00           	movl	$0x4, %r9d
140a59303: 48 89 da                    	movq	%rbx, %rdx
140a59306: e8 55 e7 9d 00              	callq	0x141437a60 <.text+0x1436a60>
140a5930b: 48 8b 5d a0                 	movq	-0x60(%rbp), %rbx
140a5930f: 49 be 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %r14 # imm = 0x8000000000000004
140a59319: 4c 39 f3                    	cmpq	%r14, %rbx
140a5931c: 74 09                       	je	0x140a59327 <.text+0xa58327>
140a5931e: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140a59322: e8 89 93 ff ff              	callq	0x140a526b0 <.text+0xa516b0>
140a59327: 4c 39 f3                    	cmpq	%r14, %rbx
140a5932a: 0f 94 c0                    	sete	%al
140a5932d: eb 02                       	jmp	0x140a59331 <.text+0xa58331>
140a5932f: 31 c0                       	xorl	%eax, %eax
140a59331: 48 8b 4d 00                 	movq	(%rbp), %rcx
140a59335: 48 89 4e 10                 	movq	%rcx, 0x10(%rsi)
140a59339: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140a5933d: 0f 11 06                    	movups	%xmm0, (%rsi)
140a59340: 8b 4f 18                    	movl	0x18(%rdi), %ecx
140a59343: c7 46 20 00 ca 9a 3b        	movl	$0x3b9aca00, 0x20(%rsi) # imm = 0x3B9ACA00
140a5934a: 48 c7 46 28 00 00 00 00     	movq	$0x0, 0x28(%rsi)
140a59352: 89 4e 30                    	movl	%ecx, 0x30(%rsi)
140a59355: c6 46 34 00                 	movb	$0x0, 0x34(%rsi)
140a59359: 88 46 35                    	movb	%al, 0x35(%rsi)
140a5935c: c6 46 36 00                 	movb	$0x0, 0x36(%rsi)
140a59360: e9 5e ff ff ff              	jmp	0x140a592c3 <.text+0xa582c3>
140a59365: 4c 8d 05 9c 9b 07 02        	leaq	0x2079b9c(%rip), %r8    # 0x142ad2f08
140a5936c: 31 c9                       	xorl	%ecx, %ecx
140a5936e: 31 d2                       	xorl	%edx, %edx
140a59370: e8 a3 01 e8 01              	callq	0x1428d9518 <.text+0x28d8518>
140a59375: 0f 0b                       	ud2
140a59377: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a59380: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a59385: 55                          	pushq	%rbp
140a59386: 41 56                       	pushq	%r14
140a59388: 56                          	pushq	%rsi
140a59389: 57                          	pushq	%rdi
140a5938a: 53                          	pushq	%rbx
140a5938b: 48 83 ec 20                 	subq	$0x20, %rsp
140a5938f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a59396: 80 7d 17 00                 	cmpb	$0x0, 0x17(%rbp)
140a5939a: 75 10                       	jne	0x140a593ac <.text+0xa583ac>
140a5939c: 48 83 7d 08 00              	cmpq	$0x0, 0x8(%rbp)
140a593a1: 74 09                       	je	0x140a593ac <.text+0xa583ac>
140a593a3: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
140a593a7: e8 84 38 2f 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a593ac: 90                          	nop
140a593ad: 48 83 c4 20                 	addq	$0x20, %rsp
140a593b1: 5b                          	popq	%rbx
140a593b2: 5f                          	popq	%rdi
140a593b3: 5e                          	popq	%rsi
140a593b4: 41 5e                       	popq	%r14
140a593b6: 5d                          	popq	%rbp
140a593b7: c3                          	retq
140a593b8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140a593c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a593c5: 55                          	pushq	%rbp
140a593c6: 41 56                       	pushq	%r14
140a593c8: 56                          	pushq	%rsi
140a593c9: 57                          	pushq	%rdi
140a593ca: 53                          	pushq	%rbx
140a593cb: 48 83 ec 20                 	subq	$0x20, %rsp
140a593cf: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a593d6: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140a593da: e8 61 03 e4 ff              	callq	0x140899740 <.text+0x898740>
140a593df: 90                          	nop
140a593e0: 48 83 c4 20                 	addq	$0x20, %rsp
140a593e4: 5b                          	popq	%rbx
140a593e5: 5f                          	popq	%rdi
140a593e6: 5e                          	popq	%rsi
140a593e7: 41 5e                       	popq	%r14
140a593e9: 5d                          	popq	%rbp
140a593ea: c3                          	retq
