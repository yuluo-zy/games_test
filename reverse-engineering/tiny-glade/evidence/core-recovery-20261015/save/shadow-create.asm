
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409184d0: 55                          	pushq	%rbp
1409184d1: 41 57                       	pushq	%r15
1409184d3: 41 56                       	pushq	%r14
1409184d5: 41 55                       	pushq	%r13
1409184d7: 41 54                       	pushq	%r12
1409184d9: 56                          	pushq	%rsi
1409184da: 57                          	pushq	%rdi
1409184db: 53                          	pushq	%rbx
1409184dc: 48 81 ec 58 02 00 00        	subq	$0x258, %rsp            # imm = 0x258
1409184e3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409184eb: 48 c7 85 d0 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1d0(%rbp)
1409184f6: 4d 89 cc                    	movq	%r9, %r12
1409184f9: 4c 89 c3                    	movq	%r8, %rbx
1409184fc: 48 89 95 b0 01 00 00        	movq	%rdx, 0x1b0(%rbp)
140918503: 48 89 ce                    	movq	%rcx, %rsi
140918506: 4c 8b 8d 40 02 00 00        	movq	0x240(%rbp), %r9
14091850d: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
140918514: c6 85 c8 01 00 00 01        	movb	$0x1, 0x1c8(%rbp)
14091851b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140918520: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140918524: 4c 89 c2                    	movq	%r8, %rdx
140918527: 4d 89 e0                    	movq	%r12, %r8
14091852a: e8 e1 a2 44 00              	callq	0x140d62810 <.text+0xd61810>
14091852f: 48 c7 45 18 06 00 00 00     	movq	$0x6, 0x18(%rbp)
140918537: 4c 8d 0d da 44 19 02        	leaq	0x21944da(%rip), %r9    # 0x142aaca18
14091853e: 4c 89 4d f8                 	movq	%r9, -0x8(%rbp)
140918542: 48 c7 45 00 04 00 00 00     	movq	$0x4, (%rbp)
14091854a: 48 c7 45 08 01 00 00 00     	movq	$0x1, 0x8(%rbp)
140918552: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
14091855a: 66 c7 45 20 00 00           	movw	$0x0, 0x20(%rbp)
140918560: c7 45 f0 00 00 00 00        	movl	$0x0, -0x10(%rbp)
140918567: c6 85 cd 01 00 00 00        	movb	$0x0, 0x1cd(%rbp)
14091856e: c6 85 cc 01 00 00 01        	movb	$0x1, 0x1cc(%rbp)
140918575: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
140918579: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14091857e: 48 c7 44 24 38 06 00 00 00  	movq	$0x6, 0x38(%rsp)
140918587: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140918590: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140918599: 48 c7 44 24 20 04 00 00 00  	movq	$0x4, 0x20(%rsp)
1409185a2: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1409185a6: 48 89 da                    	movq	%rbx, %rdx
1409185a9: 4d 89 e0                    	movq	%r12, %r8
1409185ac: e8 bf d8 1a 00              	callq	0x140ac5e70 <.text+0xac4e70>
1409185b1: 80 7d e0 02                 	cmpb	$0x2, -0x20(%rbp)
1409185b5: 0f 85 97 01 00 00           	jne	0x140918752 <.text+0x917752>
1409185bb: 48 8b 7d c8                 	movq	-0x38(%rbp), %rdi
1409185bf: 48 89 7d 50                 	movq	%rdi, 0x50(%rbp)
1409185c3: 48 8b 05 de 3d d1 02        	movq	0x2d13dde(%rip), %rax   # 0x14362c3a8
1409185ca: 48 8b 00                    	movq	(%rax), %rax
1409185cd: 48 85 c0                    	testq	%rax, %rax
1409185d0: 0f 84 f9 00 00 00           	je	0x1409186cf <.text+0x9176cf>
1409185d6: 48 8d 45 50                 	leaq	0x50(%rbp), %rax
1409185da: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1409185e1: 48 8d 05 f8 26 44 00        	leaq	0x4426f8(%rip), %rax    # 0x140d5ace0 <.text+0xd59ce0>
1409185e8: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
1409185ef: c6 85 c9 01 00 00 01        	movb	$0x1, 0x1c9(%rbp)
1409185f6: 48 8d 0d 0b 4a 19 02        	leaq	0x2194a0b(%rip), %rcx   # 0x142aad008
1409185fd: e8 7e 57 d6 ff              	callq	0x14067dd80 <.text+0x67cd80>
140918602: 0f 10 00                    	movups	(%rax), %xmm0
140918605: 8b 40 10                    	movl	0x10(%rax), %eax
140918608: 48 c7 85 60 01 00 00 01 00 00 00    	movq	$0x1, 0x160(%rbp)
140918613: 48 8d 0d 1e 49 19 02        	leaq	0x219491e(%rip), %rcx   # 0x142aacf38
14091861a: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
140918621: 48 c7 85 70 01 00 00 28 00 00 00    	movq	$0x28, 0x170(%rbp)
14091862c: 48 8d 15 c5 49 19 02        	leaq	0x21949c5(%rip), %rdx   # 0x142aacff8
140918633: 48 89 95 80 01 00 00        	movq	%rdx, 0x180(%rbp)
14091863a: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
140918645: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
14091864c: 48 89 95 90 01 00 00        	movq	%rdx, 0x190(%rbp)
