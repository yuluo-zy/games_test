
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416ee210: 55                          	pushq	%rbp
1416ee211: 56                          	pushq	%rsi
1416ee212: 57                          	pushq	%rdi
1416ee213: 53                          	pushq	%rbx
1416ee214: 48 81 ec 48 02 00 00        	subq	$0x248, %rsp            # imm = 0x248
1416ee21b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1416ee223: 48 c7 85 c0 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1c0(%rbp)
1416ee22e: 48 89 d6                    	movq	%rdx, %rsi
1416ee231: 48 8b 19                    	movq	(%rcx), %rbx
1416ee234: 48 8d 15 25 23 5a 01        	leaq	0x15a2325(%rip), %rdx   # 0x142c90560
1416ee23b: 48 8d 7d a8                 	leaq	-0x58(%rbp), %rdi
1416ee23f: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1416ee245: 48 89 f9                    	movq	%rdi, %rcx
1416ee248: e8 23 e1 2e ff              	callq	0x1409dc370 <.text+0x9db370>
1416ee24d: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416ee254: 48 89 fa                    	movq	%rdi, %rdx
1416ee257: 49 89 f0                    	movq	%rsi, %r8
1416ee25a: e8 c1 8a 55 ff              	callq	0x140c46d20 <.text+0xc45d20>
1416ee25f: 80 bd b4 01 00 00 00        	cmpb	$0x0, 0x1b4(%rbp)
1416ee266: 74 42                       	je	0x1416ee2aa <.text+0x16ed2aa>
1416ee268: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1416ee26f: f0                          	lock
1416ee270: 48 ff 08                    	decq	(%rax)
1416ee273: 75 0c                       	jne	0x1416ee281 <.text+0x16ed281>
1416ee275: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ee27c: e8 ef 2e 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee281: 48 8b 0b                    	movq	(%rbx), %rcx
1416ee284: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
1416ee288: 48 ba ab aa aa aa aa aa aa aa       	movabsq	$-0x5555555555555555, %rdx # imm = 0xAAAAAAAAAAAAAAAB
1416ee292: 48 f7 e2                    	mulq	%rdx
1416ee295: 48 d1 ea                    	shrq	%rdx
1416ee298: 48 8d 04 52                 	leaq	(%rdx,%rdx,2), %rax
1416ee29c: 48 f7 d8                    	negq	%rax
1416ee29f: 48 01 c8                    	addq	%rcx, %rax
1416ee2a2: 48 ff c0                    	incq	%rax
1416ee2a5: 48 89 03                    	movq	%rax, (%rbx)
1416ee2a8: eb 3a                       	jmp	0x1416ee2e4 <.text+0x16ed2e4>
1416ee2aa: c6 45 a8 00                 	movb	$0x0, -0x58(%rbp)
1416ee2ae: 80 bd b3 01 00 00 00        	cmpb	$0x0, 0x1b3(%rbp)
1416ee2b5: 74 14                       	je	0x1416ee2cb <.text+0x16ed2cb>
1416ee2b7: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ee2be: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
1416ee2c2: e8 a9 45 95 fe              	callq	0x140042870 <.text+0x41870>
1416ee2c7: 84 c0                       	testb	%al, %al
1416ee2c9: 75 9d                       	jne	0x1416ee268 <.text+0x16ed268>
1416ee2cb: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1416ee2d2: f0                          	lock
1416ee2d3: 48 ff 08                    	decq	(%rax)
1416ee2d6: 75 0c                       	jne	0x1416ee2e4 <.text+0x16ed2e4>
1416ee2d8: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ee2df: e8 8c 2e 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee2e4: 48 89 9d 50 01 00 00        	movq	%rbx, 0x150(%rbp)
1416ee2eb: 48 8d 05 ce f3 69 ff        	leaq	-0x960c32(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
1416ee2f2: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1416ee2f9: 48 8d 05 78 22 5a 01        	leaq	0x15a2278(%rip), %rax   # 0x142c90578
1416ee300: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1416ee307: 48 c7 85 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbp)
1416ee312: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
1416ee319: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1416ee320: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
1416ee32b: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1416ee336: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
1416ee33a: 48 8d 95 60 01 00 00        	leaq	0x160(%rbp), %rdx
1416ee341: e8 4a bc 68 ff              	callq	0x140d79f90 <.text+0xd78f90>
1416ee346: c6 45 08 00                 	movb	$0x0, 0x8(%rbp)
1416ee34a: c7 45 0d 00 00 00 00        	movl	$0x0, 0xd(%rbp)
1416ee351: c7 45 10 00 00 00 00        	movl	$0x0, 0x10(%rbp)
1416ee358: 48 c7 45 a8 03 00 00 00     	movq	$0x3, -0x58(%rbp)
1416ee360: 48 c7 45 c0 06 00 00 00     	movq	$0x6, -0x40(%rbp)
1416ee368: c7 45 f0 00 00 00 00        	movl	$0x0, -0x10(%rbp)
1416ee36f: c7 45 f8 00 00 00 00        	movl	$0x0, -0x8(%rbp)
1416ee376: 48 c7 45 00 00 00 00 00     	movq	$0x0, (%rbp)
1416ee37e: 66 c7 45 18 00 02           	movw	$0x200, 0x18(%rbp)      # imm = 0x200
1416ee384: c6 45 1a 02                 	movb	$0x2, 0x1a(%rbp)
1416ee388: c6 45 1d 02                 	movb	$0x2, 0x1d(%rbp)
1416ee38c: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1416ee393: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
1416ee397: 49 89 f0                    	movq	%rsi, %r8
1416ee39a: e8 a1 a9 53 ff              	callq	0x140c28d40 <.text+0xc27d40>
1416ee39f: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1416ee3a6: f0                          	lock
1416ee3a7: 48 ff 08                    	decq	(%rax)
1416ee3aa: 75 0c                       	jne	0x1416ee3b8 <.text+0x16ed3b8>
1416ee3ac: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ee3b3: e8 b8 2d 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee3b8: 90                          	nop
1416ee3b9: 48 81 c4 48 02 00 00        	addq	$0x248, %rsp            # imm = 0x248
1416ee3c0: 5b                          	popq	%rbx
1416ee3c1: 5f                          	popq	%rdi
1416ee3c2: 5e                          	popq	%rsi
1416ee3c3: 5d                          	popq	%rbp
1416ee3c4: c3                          	retq
1416ee3c5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1416ee3d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416ee3d5: 55                          	pushq	%rbp
1416ee3d6: 56                          	pushq	%rsi
1416ee3d7: 57                          	pushq	%rdi
1416ee3d8: 53                          	pushq	%rbx
1416ee3d9: 48 83 ec 28                 	subq	$0x28, %rsp
1416ee3dd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416ee3e4: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1416ee3eb: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1416ee3f2: f0                          	lock
1416ee3f3: 48 ff 08                    	decq	(%rax)
1416ee3f6: 75 05                       	jne	0x1416ee3fd <.text+0x16ed3fd>
1416ee3f8: e8 73 2d 55 ff              	callq	0x140c41170 <.text+0xc40170>
1416ee3fd: 90                          	nop
1416ee3fe: 48 83 c4 28                 	addq	$0x28, %rsp
1416ee402: 5b                          	popq	%rbx
1416ee403: 5f                          	popq	%rdi
1416ee404: 5e                          	popq	%rsi
1416ee405: 5d                          	popq	%rbp
1416ee406: c3                          	retq
